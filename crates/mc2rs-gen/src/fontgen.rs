//! Font generation: walks the official font provider chain
//! (`font/default.json` -> `font/include/*.json` -> bitmap providers),
//! decodes the referenced glyph textures and computes per-glyph metrics
//! exactly like vanilla's BitmapProvider (trim scan for the rightmost
//! non-transparent column). Emits `gen_font.rs`.

use std::collections::BTreeMap;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::extract::{decode_rgba, png_size, read_entry};

/// One glyph as emitted into generated code.
#[derive(Debug, Clone)]
pub struct Glyph {
    pub ch: u32,
    /// index into FONT_TEXTURES
    pub tex: u16,
    /// left of the glyph cell in the texture (px)
    pub u: u16,
    /// top of the glyph cell in the texture (px)
    pub v: u16,
    /// trimmed glyph width in px (0 = empty)
    pub gw: u16,
    /// full cell height in px
    pub gh: u16,
    /// baseline ascent of the provider
    pub ascent: u16,
    /// extra advance (space provider values)
    pub space_advance: i16,
}

#[derive(Debug, Default)]
pub struct FontData {
    pub textures: Vec<String>,
    pub glyphs: BTreeMap<u32, Glyph>,
    pub skipped_providers: usize,
    pub decoded_textures: usize,
}

/// Parse `font/default.json` and every reachable bitmap/space provider.
pub fn generate<R: std::io::Read + std::io::Seek>(archive: &mut zip::ZipArchive<R>) -> Result<FontData> {
    let root: Value = serde_json::from_slice(&read_entry(archive, "assets/minecraft/font/default.json")?)
        .context("default.json")?;
    let mut data = FontData::default();
    let providers = root
        .get("providers")
        .and_then(Value::as_array)
        .context("no providers in default.json")?;
    for p in providers {
        walk_provider(archive, p, &mut data)?;
    }
    Ok(data)
}

fn load_json<R: std::io::Read + std::io::Seek>(archive: &mut zip::ZipArchive<R>, path: &str) -> Result<Value> {
    let bytes = read_entry(archive, path).with_context(|| format!("read {path}"))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parse {path}"))
}

/// A reference id like `minecraft:include/default` -> `assets/minecraft/font/include/default.json`.
fn reference_path(id: &str) -> String {
    let id = id.strip_prefix("minecraft:").unwrap_or(id);
    format!("assets/minecraft/font/{id}.json")
}

fn walk_provider<R: std::io::Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
    p: &Value,
    data: &mut FontData,
) -> Result<()> {
    match p.get("type").and_then(Value::as_str) {
        Some("reference") => {
            let id = p.get("id").and_then(Value::as_str).context("reference id")?;
            let sub = load_json(archive, &reference_path(id))?;
            if let Some(list) = sub.get("providers").and_then(Value::as_array) {
                for sp in list {
                    walk_provider(archive, sp, data)?;
                }
            }
            Ok(())
        }
        Some("bitmap") => bitmap_provider(archive, p, data),
        Some("space") => space_provider(p, data),
        // ttf / legacy_unicode providers are skipped for now (unifont is
        // empty in 26.2; every UI string is covered by the bitmap chain).
        _ => {
            data.skipped_providers += 1;
            Ok(())
        }
    }
}

fn texture_file(file: &str) -> String {
    // "minecraft:font/ascii.png" -> "assets/minecraft/textures/font/ascii.png"
    let f = file.strip_prefix("minecraft:").unwrap_or(file);
    format!("assets/minecraft/textures/{f}")
}

fn bitmap_provider<R: std::io::Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
    p: &Value,
    data: &mut FontData,
) -> Result<()> {
    let file = p.get("file").and_then(Value::as_str).context("bitmap file")?;
    let height = p.get("height").and_then(Value::as_u64).unwrap_or(8) as u16;
    let ascent = p.get("ascent").and_then(Value::as_u64).unwrap_or(height as u64 - 1) as u16;
    let rows = p
        .get("chars")
        .and_then(Value::as_array)
        .context("bitmap chars")?;
    let path = texture_file(file);

    let tex_idx = match data.textures.iter().position(|t| t == &path) {
        Some(i) => i as u16,
        None => {
            data.textures.push(path.clone());
            (data.textures.len() - 1) as u16
        }
    };

    // Decode the texture once to measure trimmed glyph widths (vanilla scans
    // for the rightmost non-transparent column inside each cell).
    let (tex_w, tex_h, rgba) = match decode_rgba(&read_entry(archive, &path)?) {
        Ok(v) => {
            data.decoded_textures += 1;
            v
        }
        Err(_) => (png_size(&read_entry(archive, &path)?)?.0, height as u32, Vec::new()),
    };

    let row_len = rows.iter().map(|r| r.as_str().map(str::chars).map_or(0, Iterator::count)).max().unwrap_or(1) as u32;
    let cell_w = (tex_w / row_len.max(1)) as u16;
    let cell_h = ((tex_h as usize) / rows.len().max(1)) as u16;
    let cell_h = cell_h.max(height) as u16;

    for (r, row) in rows.iter().enumerate() {
        let Some(line) = row.as_str() else { continue };
        for (c, ch) in line.chars().enumerate() {
            let ch_u32 = ch as u32;
            if ch_u32 == 0 {
                continue; // empty cell
            }
            let u = (c as u16) * cell_w;
            let v = (r as u16) * cell_h;
            let gw = if rgba.is_empty() {
                cell_w
            } else {
                trim_width(&rgba, tex_w as u16, tex_h as u16, u, v, cell_w, cell_h)
            };
            if gw == 0 && cell_w > 0 {
                // Vanilla drops fully-blank cells (e.g. the space character in
                // ascii.png) so the space provider's advance stays effective.
                continue;
            }
            data.glyphs.insert(
                ch_u32,
                Glyph {
                    ch: ch_u32,
                    tex: tex_idx,
                    u,
                    v,
                    gw,
                    gh: cell_h,
                    ascent,
                    space_advance: 0,
                },
            );
        }
    }
    Ok(())
}

/// Rightmost non-transparent column + 1 inside the cell (0 if fully empty) —
/// the same metric vanilla's BitmapProvider derives per glyph.
fn trim_width(
    rgba: &[u8],
    tex_w: u16,
    tex_h: u16,
    u: u16,
    v: u16,
    cell_w: u16,
    cell_h: u16,
) -> u16 {
    let mut best = 0u16;
    for x in 0..cell_w {
        for y in 0..cell_h {
            let px = (u + x) as u32;
            let py = (v + y) as u32;
            if px >= tex_w as u32 || py >= tex_h as u32 {
                continue;
            }
            let a = rgba[((py * tex_w as u32 + px) as usize) * 4 + 3];
            if a > 32 {
                best = best.max(x + 1);
                break;
            }
        }
    }
    best
}

fn space_provider(p: &Value, data: &mut FontData) -> Result<()> {
    if let Some(adv) = p.get("advances").and_then(Value::as_object) {
        for (k, v) in adv {
            let Some(ch) = k.chars().next() else { continue };
            let a = v.as_i64().unwrap_or(0) as i16;
            data.glyphs.insert(
                ch as u32,
                Glyph {
                    ch: ch as u32,
                    tex: u16::MAX,
                    u: 0,
                    v: 0,
                    gw: 0,
                    gh: 0,
                    ascent: 0,
                    space_advance: a,
                },
            );
        }
    }
    Ok(())
}

/// Emit `gen_font.rs` (byte-stable: glyphs sorted by codepoint).
pub fn emit_cache(data: &FontData) -> Result<Vec<u8>> {
    // THE MIGRATION'S FIFTH LANE (the font): the measured glyph table the
    // generator's own BINARY emit carries into the runtime cache
    // (assets-cache/gen/font.bin — the first generator-DERIVED cache
    // file: not a jar byte, the measured law the jar walk produces), the
    // client's loader (mc-client/src/font_table.rs) the only parser.
    // THE FORMAT: the MC2RSFONT1 magic, the texture list (the SHORT
    // paths the renderer's cache walk joins), the skipped-provider
    // count, then the glyph records sorted by codepoint (18 bytes each:
    // ch u32, tex u16, u u16, v u16, gw u16, gh u16, ascent u16,
    // space_advance i16 — LE).
    let mut out = Vec::with_capacity(10 + 4 + 4 + 64 + 4 + 4 + data.glyphs.len() * 18);
    out.extend_from_slice(b"MC2RSFONT1");
    out.extend_from_slice(&(data.textures.len() as u32).to_le_bytes());
    for t in &data.textures {
        let short = t.strip_prefix("assets/minecraft/textures/").unwrap_or(t);
        out.extend_from_slice(&(short.len() as u32).to_le_bytes());
        out.extend_from_slice(short.as_bytes());
    }
    out.extend_from_slice(&(data.skipped_providers as u32).to_le_bytes());
    out.extend_from_slice(&(data.glyphs.len() as u32).to_le_bytes());
    for g in data.glyphs.values() {
        // the BTreeMap order IS the sorted-by-ch law the lookup requires
        out.extend_from_slice(&g.ch.to_le_bytes());
        out.extend_from_slice(&g.tex.to_le_bytes());
        out.extend_from_slice(&g.u.to_le_bytes());
        out.extend_from_slice(&g.v.to_le_bytes());
        out.extend_from_slice(&g.gw.to_le_bytes());
        out.extend_from_slice(&g.gh.to_le_bytes());
        out.extend_from_slice(&g.ascent.to_le_bytes());
        out.extend_from_slice(&g.space_advance.to_le_bytes());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zip_of(entries: &[(&str, Vec<u8>)]) -> zip::ZipArchive<std::io::Cursor<Vec<u8>>> {
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut w = zip::ZipWriter::new(&mut buf);
            let opts = zip::write::SimpleFileOptions::default();
            for (name, data) in entries {
                w.start_file(*name, opts).unwrap();
                std::io::Write::write_all(&mut w, data).unwrap();
            }
            w.finish().unwrap();
        }
        zip::ZipArchive::new(buf).unwrap()
    }

    /// Build a 2-glyph font: 'A' solid 3px wide, 'B' empty, plus a space provider.
    fn tiny_png(w: u32, h: u32, paint: impl Fn(u32, u32) -> [u8; 4]) -> Vec<u8> {
        let mut rgba = Vec::with_capacity((w * h * 4) as usize);
        for y in 0..h {
            for x in 0..w {
                rgba.extend_from_slice(&paint(x, y));
            }
        }
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, w, h);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().unwrap();
            wr.write_image_data(&rgba).unwrap();
        }
        out
    }

    #[test]
    fn parses_chain_and_measures_widths() {
        // 16x8 texture, one row of 2 chars -> cell 8x8.
        let ascii_png = tiny_png(16, 8, |x, y| {
            if x < 3 && y < 7 { [255, 255, 255, 255] } else { [0, 0, 0, 0] }
        });
        let default_json = br#"{"providers":[{"type":"reference","id":"minecraft:include/space"},{"type":"reference","id":"minecraft:include/default"}]}"#.to_vec();
        let include_default = br#"{"providers":[
            {"type":"bitmap","file":"minecraft:font/ascii.png","ascent":7,"chars":["\u0041\u0042"]}]}"#.to_vec();
        let include_space = br#"{"providers":[{"type":"space","advances":{" ":4}}]}"#.to_vec();

        let mut zip = zip_of(&[
            ("assets/minecraft/font/default.json", default_json),
            ("assets/minecraft/font/include/default.json", include_default),
            ("assets/minecraft/font/include/space.json", include_space),
            ("assets/minecraft/textures/font/ascii.png", ascii_png),
        ]);
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.textures.len(), 1);
        assert_eq!(data.glyphs.len(), 2); // A + space (blank B dropped, vanilla rule)
        assert_eq!(data.skipped_providers, 0);

        let a = data.glyphs[&(u32::from('A'))].clone();
        assert_eq!(a.gw, 3, "trimmed to rightmost opaque column");
        assert_eq!(a.gh, 8, "cell height 8");
        assert_eq!(a.ascent, 7);
        assert_eq!(a.u, 0);
        assert!(
            !data.glyphs.contains_key(&(u32::from('B'))),
            "blank glyph cell must be dropped so space advance survives"
        );
        assert_eq!(data.glyphs[&(u32::from(' '))].space_advance, 4);

        // THE CACHE EMIT (the migration's fifth lane): the binary law
        // the client's loader parses — the magic, the texture list, the
        // skipped count, the sorted 18-byte records (the space 32 lands
        // BEFORE the 'A' 65 — the BTreeMap order the lookup requires).
        let emitted = emit_cache(&data).unwrap();
        assert_eq!(&emitted[..10], b"MC2RSFONT1", "the magic");
        // 10 magic + 4 tex-count + (4+14) "font/ascii.png" + 4 skipped
        // + 4 glyph-count + 2 records x 18 bytes
        assert_eq!(emitted.len(), 10 + 4 + 4 + 14 + 4 + 4 + 2 * 18);
        let rec_space = &emitted[10 + 4 + 4 + 14 + 4 + 4..][..18];
        assert_eq!(
            u32::from_le_bytes(rec_space[0..4].try_into().unwrap()),
            u32::from(' '),
            "the sorted order lands the space first"
        );
        assert_eq!(
            i16::from_le_bytes(rec_space[16..18].try_into().unwrap()),
            4,
            "the space provider's advance rides the record"
        );
        let rec_a = &emitted[10 + 4 + 4 + 14 + 4 + 4 + 18..][..18];
        assert_eq!(u32::from_le_bytes(rec_a[0..4].try_into().unwrap()), u32::from('A'));
        assert_eq!(u16::from_le_bytes(rec_a[10..12].try_into().unwrap()), 3, "the measured gw");
    }

    #[test]
    fn skips_unknown_provider_types() {
        let default_json = br#"{"providers":[{"type":"ttf","file":"x.ttf"}]}"#.to_vec();
        let mut zip = zip_of(&[("assets/minecraft/font/default.json", default_json)]);
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.skipped_providers, 1);
        assert!(data.glyphs.is_empty());
    }
}

//! The texture namespace — every concrete texture id the models table
//! references, decoded to RGBA8. The atlas-mapping round's data half: the
//! element quads (v0.3.68) keep a texture slot; texture_slots (v0.3.69)
//! resolves it to an id like `block/stone`; THIS table is the
//! id -> pixels link the consumer samples.
//!
//! The census ran first (scripts/atlas_domain_census.py +
//! atlas_domain_census2.py over the real 26.2 jar): the 3928 model files
//! yield 2113 distinct concrete texture ids — 1252 block/ + 796 item/ +
//! 64 trims/ + the literal `missingno` (item/air.json and block/air.json
//! reference `minecraft:missingno`) — of which 2048 exist as PNGs and 65
//! are MISSING (the trims/items armor-trim family + missingno; vanilla
//! renders unresolved materials as the checkerboard). 2001 tiles are 16x16,
//! 47 are 32x32 whole sprites (hanging signs, signs, shelves, dried_ghast
//! tentacles, the seven spears in hand). All 52 h>w domain textures carry a
//! sibling .png.mcmeta with an "animation" key — the first-frame crop is
//! therefore BOTH mcmeta-faithful and unconditional-crop-faithful on real
//! data; zero w>h exist; a tall texture without mcmeta stays whole (the
//! vanilla rule, never observed in 26.2). Pixels are RAW: NO tint is baked
//! (tintindex rides the quads; the consumer multiplies at draw time — the
//! WORLD atlas bakes because its render path has no per-face tint pass
//! yet, and its tinted tiles are deliberately NOT byte-identical to this
//! table for tinted textures; untinted textures like block/stone ARE).

use std::collections::BTreeSet;

use anyhow::{bail, Context, Result};
use serde_json::Value;
use zip::ZipArchive;

/// The jar directory the model files live at (the domain source).
pub const MODELS_DIR: &str = "assets/minecraft/models/";
/// The only two model directories the 26.2 jar ships (modelgen's census:
/// 2657 block / 1271 item).
pub const MODEL_PREFIXES: [&str; 2] = ["block/", "item/"];

/// The jar directory the textures live at.
pub const TEXTURES_DIR: &str = "assets/minecraft/textures/";

/// One decoded texture: RGBA8, first frame if animated.
#[derive(Debug, Clone)]
pub struct Tile {
    pub width: u16,
    pub height: u16,
    pub rgba: Vec<u8>,
}

/// The namespace: sorted ids, per-id dims, concatenated texels.
#[derive(Debug, Default)]
pub struct TextureData {
    /// Sorted unique ids in the bare form ("block/stone", "trims/items/…",
    /// "missingno") — the strip-#-lookup form texture_slots resolves to.
    pub ids: Vec<String>,
    /// Zero width/height == the MISSING material (no PNG in the jar).
    pub tiles: Vec<Tile>,
    pub texels: Vec<u8>,
    /// Census stats for the command's report line.
    pub n_model_files: usize,
    pub n_refs: usize,
    pub n_present: usize,
    pub n_missing: usize,
    pub n_first_frame: usize,
    pub n_mcmeta: usize,
}

/// The effective sprite dims under the vanilla animation rule: a texture is
/// animated ONLY when a sibling .png.mcmeta carries an "animation" key, and
/// its frames are w x w stacked vertically. A tall texture WITHOUT mcmeta is
/// a whole non-animated sprite (never observed in 26.2's domain — the census
/// pins all 52 h>w textures as mcmeta animations).
pub fn effective_dims(has_animation_mcmeta: bool, w: u32, h: u32) -> (u32, u32) {
    if has_animation_mcmeta && h > w {
        (w, w)
    } else {
        (w, h)
    }
}

/// Collect the concrete texture ids one model file references: every string
/// value of the textures map that doesn't start with '#', plus the 26.2
/// inline sprite dicts' "sprite" field (the {"force_translucent", "sprite"}
/// shape texture_slots pinned — every dict carries the field; its absence
/// refuses).
fn collect_texture_values(textures: &Value, out: &mut BTreeSet<String>) -> Result<usize> {
    let Some(map) = textures.as_object() else {
        return Ok(0);
    };
    let mut n = 0usize;
    for (_k, v) in map {
        match v {
            Value::String(s) => {
                if !s.starts_with('#') {
                    out.insert(normalize_id(s));
                }
                n += 1;
            }
            Value::Object(dict) => {
                let Some(sprite) = dict.get("sprite").and_then(|s| s.as_str()) else {
                    bail!("texture dict without a sprite field (census: every 26.2 dict has one)");
                };
                if !sprite.starts_with('#') {
                    out.insert(normalize_id(sprite));
                }
                n += 1;
            }
            other => bail!("unexpected textures-map value: {other}"),
        }
    }
    Ok(n)
}

/// Strip a leading "minecraft:" — the bare form the whole consumer chain
/// (texture_slots' strip-# lookup, the quads' slots) speaks.
fn normalize_id(s: &str) -> String {
    s.strip_prefix("minecraft:").unwrap_or(s).to_string()
}

/// Walk the jar's model files, resolve the concrete texture domain, decode
/// every existing member to RGBA8 (first frame if animated).
pub fn generate(archive: &mut ZipArchive<std::io::Cursor<&[u8]>>) -> Result<TextureData> {
    // The model file inventory (strict prefixes, .json only).
    let mut model_files: Vec<String> = Vec::new();
    for i in 0..archive.len() {
        let name = archive.by_index(i)?.name().to_string();
        if !name.starts_with(MODELS_DIR) || !name.ends_with(".json") {
            continue;
        }
        let rel = &name[MODELS_DIR.len()..];
        if !MODEL_PREFIXES.iter().any(|p| rel.starts_with(p)) {
            bail!("model file outside the census prefixes: {name}");
        }
        model_files.push(name);
    }
    model_files.sort();

    // The domain.
    let mut domain: BTreeSet<String> = BTreeSet::new();
    let mut n_refs = 0usize;
    for name in &model_files {
        let bytes = read_entry_opt(archive, name)?
            .with_context(|| format!("model {name}: vanished mid-walk"))?;
        let json: Value = serde_json::from_slice(&bytes)
            .with_context(|| format!("model {name}: json parse"))?;
        if let Some(textures) = json.get("textures") {
            n_refs += collect_texture_values(textures, &mut domain)?;
        }
    }

    // Decode every existing member; record the missing ones.
    let mut ids: Vec<String> = Vec::with_capacity(domain.len());
    let mut tiles: Vec<Tile> = Vec::with_capacity(domain.len());
    let mut texels: Vec<u8> = Vec::new();
    let mut n_present = 0usize;
    let mut n_missing = 0usize;
    let mut n_first_frame = 0usize;
    let mut n_mcmeta = 0usize;
    for id in &domain {
        let png_path = format!("{TEXTURES_DIR}{id}.png");
        let raw = match read_entry_opt(archive, &png_path)? {
            Some(raw) => raw,
            None => {
                ids.push(id.clone());
                tiles.push(Tile { width: 0, height: 0, rgba: Vec::new() });
                n_missing += 1;
                continue;
            }
        };
        let (w, h, mut rgba) = crate::atlas::decode_rgba(&raw)
            .with_context(|| format!("texture {id}: decode"))?;
        // The vanilla animation gate: a sibling mcmeta with "animation".
        let mcmeta_path = format!("{TEXTURES_DIR}{id}.png.mcmeta");
        let mut animated = false;
        if let Some(mc_bytes) = read_entry_opt(archive, &mcmeta_path)? {
            n_mcmeta += 1;
            let mc_json: Value = serde_json::from_slice(&mc_bytes)
                .with_context(|| format!("texture {id}: mcmeta parse"))?;
            animated = mc_json.get("animation").is_some();
        }
        let (fw, fh) = effective_dims(animated, w, h);
        if animated && h > w {
            let cropped = crate::atlas::first_frame(w, h, &rgba);
            rgba = cropped.2;
            n_first_frame += 1;
        }
        if fw > u32::from(u16::MAX) {
            bail!("texture {id}: width {fw} overflows u16");
        }
        texels.extend_from_slice(&rgba);
        ids.push(id.clone());
        tiles.push(Tile {
            width: fw as u16,
            height: fh as u16,
            rgba,
        });
        n_present += 1;
    }

    // THE SPECIAL RENDERERS' entity textures — the dump-backed law's own
    // additions over the models' domain (the specialgen census: the 63
    // special items' renderer textures; the banner base rides ONCE as the
    // tint source and 16 times as the pre-tinted DyeColor variants — the
    // generation-time form of the vanilla vertex-color multiply, since
    // the special lane's raster carries no vertex-color channel).
    let mut specials: Vec<(String, Tile)> = Vec::new();
    for (id, png_rel, tint) in crate::specialgen::special_texture_variants() {
        if domain.contains(&id) {
            bail!("special texture {id} collides with the models' domain");
        }
        let raw = read_entry_opt(archive, &format!("{TEXTURES_DIR}{png_rel}.png"))?
            .with_context(|| format!("special texture {id}: textures/{png_rel}.png absent from the jar"))?;
        let (w, h, mut rgba) = crate::atlas::decode_rgba(&raw)
            .with_context(|| format!("special texture {id}: decode"))?;
        if let Some(t) = tint {
            apply_dye_tint(&mut rgba, t);
        }
        if w > u32::from(u16::MAX) {
            bail!("texture {id}: width {w} overflows u16");
        }
        specials.push((id, Tile { width: w as u16, height: h as u16, rgba }));
    }

    // THE MERGE: one sorted id space (the generated TEXTURE_IDS doc +
    // texture_index's binary_search both demand it) — the specials
    // interleave into the models' domain, the texels rebuilt in the
    // SAME order (the emitted offsets are positional).
    let mut merged: Vec<(String, Tile)> = ids.into_iter().zip(tiles).collect();
    merged.extend(specials);
    merged.sort_by(|a, b| a.0.cmp(&b.0));
    let mut texels = Vec::with_capacity(texels.len() + (1 << 18));
    let mut ids = Vec::with_capacity(merged.len());
    let mut tiles = Vec::with_capacity(merged.len());
    for (id, t) in merged {
        texels.extend_from_slice(&t.rgba);
        ids.push(id);
        tiles.push(t);
    }

    Ok(TextureData {
        ids,
        tiles,
        texels,
        n_model_files: model_files.len(),
        n_refs,
        n_present,
        n_missing,
        n_first_frame,
        n_mcmeta,
    })
}

/// The vanilla vertex-color multiply, generation-time: the texel's
/// 0..1 channel times the DyeColor textureDiffuseColor's 0..1 channel
/// (the 0xRRGGBB unpack — the same /255 law the client's tint.rs
/// speaks), back to u8 with round-to-nearest (the f32 shader path's
/// own store). Alpha untouched.
fn apply_dye_tint(rgba: &mut [u8], tint: u32) {
    let channels = [
        ((tint >> 16) & 0xFF) as f32 / 255.0,
        ((tint >> 8) & 0xFF) as f32 / 255.0,
        (tint & 0xFF) as f32 / 255.0,
    ];
    for px in rgba.chunks_exact_mut(4) {
        for c in 0..3 {
            px[c] = ((px[c] as f32 / 255.0) * channels[c] * 255.0).round() as u8;
        }
    }
}

fn read_entry_opt(
    archive: &mut ZipArchive<std::io::Cursor<&[u8]>>,
    name: &str,
) -> Result<Option<Vec<u8>>> {
    let mut f = match archive.by_name(name) {
        Ok(f) => f,
        Err(zip::result::ZipError::FileNotFound) => return Ok(None),
        Err(e) => return Err(e).with_context(|| format!("jar entry {name}")),
    };
    let mut out = Vec::new();
    std::io::Read::read_to_end(&mut f, &mut out)?;
    Ok(Some(out))
}

/// Emit `mc-registry/src/generated/textures.rs`.
pub fn emit(data: &TextureData, version: &str, client_sha1: &str) -> Result<String> {
    if data.ids.len() != data.tiles.len() {
        bail!("texture data inconsistent: {} ids vs {} tiles", data.ids.len(), data.tiles.len());
    }
    // Byte offsets into TEXELS — missing textures carry an empty span.
    let mut offsets: Vec<u32> = Vec::with_capacity(data.ids.len() + 1);
    let mut acc: u32 = 0;
    for t in &data.tiles {
        offsets.push(acc);
        acc += t.rgba.len() as u32;
    }
    offsets.push(acc);

    let mut src = String::with_capacity(data.texels.len() * 43 / 10 + 131072);
    src.push_str(&format!(
        "// @generated by mc2rs-gen from the OFFICIAL client jar {version} (sha1 {client_sha1}).\n\
         // The texture namespace the models table references: every concrete id in any\n\
         // model's textures map (incl. the 26.2 inline sprite dicts' \"sprite\" fields)\n\
         // decoded to RAW RGBA8 — NO tint baked (tintindex rides the quads; the\n\
         // consumer multiplies at draw time) — PLUS the special renderers' entity\n\
         // textures (the 63 specials' law: shield/trident/chest/shulker/banner/head/\n\
         // conduit/pot/golem skins; the banner entries PRE-TINTED per DyeColor, the\n\
         // multiply the special lane's raster has no vertex-color channel for).\n\
         // Animated strips (sibling .png.mcmeta\n\
         // with \"animation\") are cropped to the first w x w frame. MISSING materials\n\
         // ({}, the trims/items armor-trim family + the literal \"missingno\" the air\n\
         // models reference) carry width=height=0 and an empty texel span — the\n\
         // consumer renders the vanilla checkerboard for them.\n\
         // DO NOT EDIT BY HAND — regenerate: `cargo run -p mc2rs-gen -- textures --version {version}`\n\
         // (Mojang assets are NOT committed; this file is regenerated from the jar.)\n\n\
         pub const TEXTURES_VERSION: &str = \"{version}\";\n\
         pub const N_TEXTURES: usize = {};\n\
         pub const N_MISSING: usize = {};\n\n\
         /// Sorted unique ids in the bare form texture_slots resolves to.\n\
         pub static TEXTURE_IDS: [&str; N_TEXTURES] = [\n",
        if data.n_missing > 0 { data.n_missing.to_string() } else { "none".to_string() },
        data.ids.len(),
        data.n_missing,
    ));
    for chunk in data.ids.chunks(6) {
        src.push_str("    ");
        for id in chunk {
            src.push_str(&format!("{id:?},"));
        }
        src.push('\n');
    }
    src.push_str("];\n\n");

    src.push_str("/// Pixel width per id; 0 == the MISSING material.\n\
                  pub static WIDTHS: [u16; N_TEXTURES] = [\n");
    for chunk in data.tiles.chunks(24) {
        src.push_str("    ");
        for t in chunk {
            src.push_str(&format!("{},", t.width));
        }
        src.push('\n');
    }
    src.push_str("];\n\n");

    src.push_str("/// Pixel height per id; 0 == the MISSING material.\n\
                  pub static HEIGHTS: [u16; N_TEXTURES] = [\n");
    for chunk in data.tiles.chunks(24) {
        src.push_str("    ");
        for t in chunk {
            src.push_str(&format!("{},", t.height));
        }
        src.push('\n');
    }
    src.push_str("];\n\n");

    src.push_str("/// Byte offsets into TEXELS (N+1 entries; missing ids span zero bytes).\n\
                  pub static OFFSETS: [u32; N_TEXTURES + 1] = [\n");
    for chunk in offsets.chunks(16) {
        src.push_str("    ");
        for o in chunk {
            src.push_str(&format!("{o},"));
        }
        src.push('\n');
    }
    src.push_str("];\n\n");

    src.push_str(&format!(
        "/// The concatenated RGBA8 texels, row-major, unpremultiplied alpha.\n\
         pub static TEXELS: [u8; {}] = [",
        data.texels.len()
    ));
    for (j, b) in data.texels.iter().enumerate() {
        if j % 24 == 0 {
            src.push('\n');
        }
        src.push_str(&format!("{b},"));
    }
    src.push_str("\n];\n\n");

    src.push_str(
        "/// Binary search the sorted namespace. Note: a hit is NOT existence —\n\
         /// the literal `missingno` and the armor-trim family are pinned missing.\n\
         pub fn texture_index(id: &str) -> Option<u16> {\n\
         \x20   TEXTURE_IDS.binary_search(&id).ok().map(|i| i as u16)\n\
         }\n\n\
         /// The id's RGBA8 texels (empty for a missing material).\n\
         pub fn texture_rgba(idx: u16) -> &'static [u8] {\n\
         \x20   let i = idx as usize;\n\
         \x20   &TEXELS[OFFSETS[i] as usize..OFFSETS[i + 1] as usize]\n\
         }\n\n\
         /// The id's pixel dims (0, 0) for a missing material.\n\
         pub fn texture_size(idx: u16) -> (u16, u16) {\n\
         \x20   (WIDTHS[idx as usize], HEIGHTS[idx as usize])\n\
         }\n\n\
         /// True when the id has no PNG in the jar (the vanilla checkerboard).\n\
         pub fn is_missing(idx: u16) -> bool {\n\
         \x20   WIDTHS[idx as usize] == 0\n\
         }\n\n\
         pub fn has_textures() -> bool { N_TEXTURES > 0 }\n",
    );
    Ok(src)
}

/// Committed fallback source (no jar available): an empty table that
/// compiles everywhere; consumers keep the TILE_NONE-style degraded mode.
pub fn fallback_source(version: &str) -> String {
    format!(
        "// @generated fallback — run `cargo run -p mc2rs-gen -- textures --version {version}`\n\
         // against the official jar to replace this with real texture data.\n\
         pub const TEXTURES_VERSION: &str = \"{version}\";\n\
         pub const N_TEXTURES: usize = 0;\n\
         pub const N_MISSING: usize = 0;\n\
         pub static TEXTURE_IDS: [&str; N_TEXTURES] = [];\n\
         pub static WIDTHS: [u16; N_TEXTURES] = [];\n\
         pub static HEIGHTS: [u16; N_TEXTURES] = [];\n\
         pub static OFFSETS: [u32; N_TEXTURES + 1] = [0];\n\
         pub static TEXELS: [u8; 0] = [];\n\
         pub fn texture_index(id: &str) -> Option<u16> {{\n\
         \x20   let _ = id;\n\
         \x20   None\n\
         }}\n\
         pub fn texture_rgba(idx: u16) -> &'static [u8] {{\n\
         \x20   let _ = idx;\n\
         \x20   &[]\n\
         }}\n\
         pub fn texture_size(idx: u16) -> (u16, u16) {{\n\
         \x20   let _ = idx;\n\
         \x20   (0, 0)\n\
         }}\n\
         pub fn is_missing(idx: u16) -> bool {{\n\
         \x20   let _ = idx;\n\
         \x20   true\n\
         }}\n\
         pub fn has_textures() -> bool {{ false }}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_source_is_valid_shape() {
        let src = fallback_source("26.2");
        assert!(src.contains("N_TEXTURES: usize = 0"));
        assert!(src.contains("OFFSETS: [u32; N_TEXTURES + 1] = [0]"));
        assert!(src.contains("26.2"));
        // The fallback must carry every accessor the real table does.
        for f in ["texture_index", "texture_rgba", "texture_size", "is_missing", "has_textures"] {
            assert!(src.contains(f), "missing accessor {f}");
        }
    }

    #[test]
    fn the_mcmeta_animation_rule_gates_the_first_frame_crop() {
        // The census's discipline: every branch pinned.
        // Animated strip: mcmeta + tall -> first w x w frame.
        assert_eq!(effective_dims(true, 16, 512), (16, 16));
        assert_eq!(effective_dims(true, 16, 2), (16, 2)); // h > w required
        // Square with mcmeta: whole (a 1-frame animation is the sprite itself).
        assert_eq!(effective_dims(true, 16, 16), (16, 16));
        // Tall WITHOUT mcmeta: the whole non-animated sprite (the vanilla
        // rule; zero instances in the 26.2 domain — the census pins that).
        assert_eq!(effective_dims(false, 16, 512), (16, 512));
        // Flat/wide without mcmeta: whole.
        assert_eq!(effective_dims(false, 32, 16), (32, 16));
    }

    #[test]
    fn normalize_strips_the_namespace_prefix() {
        assert_eq!(normalize_id("minecraft:block/stone"), "block/stone");
        assert_eq!(normalize_id("block/stone"), "block/stone");
        assert_eq!(normalize_id("minecraft:missingno"), "missingno");
    }

    #[test]
    fn every_sprite_dict_without_a_sprite_field_refuses() {
        let mut out = BTreeSet::new();
        let dict = serde_json::json!({ "top": { "force_translucent": true } });
        assert!(collect_texture_values(&dict, &mut out).is_err());
        let ok = serde_json::json!({
            "all": "block/stone",
            "particle": { "force_translucent": true, "sprite": "minecraft:block/dust" },
            "ref": "#all"
        });
        let n = collect_texture_values(&ok, &mut out).unwrap();
        assert_eq!(n, 3, "all three entries count");
        assert_eq!(out, BTreeSet::from(["block/stone".to_string(), "block/dust".to_string()]));
    }
}

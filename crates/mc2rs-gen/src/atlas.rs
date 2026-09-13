//! REAL texture atlas extraction from the official client jar.
//!
//! Resolution chain mirrors the game: blockstates/<block>.json -> models/block
//! (parent chain, merged texture maps) -> textures/block/<name>.png decoded to
//! RGBA8. Animated strips (water/lava) are cropped to the first frame. Biome
//! tints (grass/foliage/water, vanilla plains constants) are baked into the
//! emitted pixels. Output: `crates/mc-registry/src/generated/atlas.rs`.
//!
//! No Mojang files are committed — pixels are regenerated from the official
//! jar at build/CI time by `mc2rs-gen generate` / `mc2rs-gen atlas`.

use std::collections::BTreeMap;
use std::io::Read;

use anyhow::{bail, Context, Result};
use serde_json::Value;
use zip::ZipArchive;

/// Runtime block ids of the reference client (must mirror
/// `mc-client/src/world.rs`). air is never meshed but keeps the ids aligned.
/// Members after the original survival-classic eight render network terrain
/// that previously stayed air (stone variants, deepslate, ores, netherrack);
/// pale_oak_leaves rides the SAME leaves geometry chain (parent block/leaves)
/// with NO tint — vanilla's BlockColors registers no source for it (the
/// audited createDefault(): the model's tintindex falls back to BLANK_LAYER
/// and the PRE-COLORED texture renders as authored).
pub const RUNTIME_PALETTE: [&str; 25] = [
    "air", "stone", "dirt", "grass_block", "oak_log", "oak_leaves", "water", "sand",
    "granite", "diorite", "andesite", "deepslate", "cobbled_deepslate", "gravel",
    "bedrock", "coal_ore", "copper_ore", "iron_ore", "gold_ore", "redstone_ore",
    "lapis_ore", "diamond_ore", "emerald_ore", "netherrack", "pale_oak_leaves",
];

/// Renderer face order: 0:+X 1:-X 2:+Y 3:-Y 4:+Z 5:-Z.
pub const FACE_KEYS: [&str; 6] = ["east", "west", "up", "down", "south", "north"];

/// Vanilla plains-biome tint constants (0xRRGGBB), applied by tintindex.
pub const TINT_GRASS: u32 = 0x91BD59;
pub const TINT_FOLIAGE: u32 = 0x77AB2F;
pub const TINT_WATER: u32 = 0x3F76E4;

#[derive(Debug, Clone)]
struct ResolvedFace {
    texture: String,   // e.g. "block/grass_block_top"
    tinted: bool,      // model face carried a tintindex
}

fn tint_for_texture(stem: &str) -> u32 {
    // Grayscale master textures the game tints in-world (plains biome here).
    match stem {
        "grass_block_top" => TINT_GRASS,
        "oak_leaves" => TINT_FOLIAGE,
        "water_still" => TINT_WATER,
        _ => 0,
    }
}

/// Resolve the per-face textures of one block through the real model chain.
fn resolve_block(
    archive: &mut ZipArchive<std::io::Cursor<&[u8]>>,
    block: &str,
) -> Result<[ResolvedFace; 6]> {
    if block == "water" {
        // Fluid rendering is special-cased in the game itself (no culled
        // blockstate model); the still texture with water tint is the
        // canonical lookup used by the item/fluid renderer.
        return Ok(resolve_flat_stem("water_still", true));
    }

    let bs_bytes = read_jar(archive, &format!("assets/minecraft/blockstates/{block}.json"))?;
    let bs: Value = serde_json::from_slice(&bs_bytes)
        .with_context(|| format!("blockstate {block}: json parse"))?;
    let variants = bs
        .get("variants")
        .and_then(|v| v.as_object())
        .with_context(|| format!("blockstate {block}: no variants map"))?;
    let first = variants
        .values()
        .next()
        .with_context(|| format!("blockstate {block}: empty variants"))?;
    // A variant is either one entry or a list (rotations) — take the first.
    let entry = match first {
        Value::Array(list) => list.first().cloned(),
        v @ Value::Object(_) => Some(v.clone()),
        _ => None,
    }
    .with_context(|| format!("blockstate {block}: no model entry"))?;
    let model_ref = entry
        .get("model")
        .and_then(|m| m.as_str())
        .with_context(|| format!("blockstate {block}: entry has no model"))?
        .trim_start_matches("minecraft:")
        .to_string();

    let model = load_model_chain(archive, &model_ref)
        .with_context(|| format!("block {block}: model chain {model_ref}"))?;

    // Find the first element that defines all six faces (full cube).
    let elements = model
        .1
        .get("elements")
        .and_then(|e| e.as_array())
        .with_context(|| format!("model {model_ref}: no elements"))?;
    let mut faces: BTreeMap<&str, &Value> = BTreeMap::new();
    for el in elements {
        if let Some(f) = el.get("faces").and_then(|f| f.as_object()) {
            if FACE_KEYS.iter().all(|k| f.contains_key(*k)) {
                for k in FACE_KEYS {
                    faces.insert(k, &f[k]);
                }
                break;
            }
        }
    }
    if faces.len() != 6 {
        bail!("model {model_ref}: no full-cube element (6 faces)");
    }

    let (tex_map, _) = model;
    let mut out = Vec::with_capacity(6);
    for key in FACE_KEYS {
        let face = faces[key];
        let tref = face
            .get("texture")
            .and_then(|t| t.as_str())
            .with_context(|| format!("model {model_ref}: face {key} without texture"))?;
        let texture = tex_map
            .get(tref.trim_start_matches('#'))
            .with_context(|| format!("model {model_ref}: unresolved ref {tref}"))?
            .clone();
        let tinted = face.get("tintindex").is_some();
        out.push(ResolvedFace { texture, tinted });
    }
    Ok([
        out[0].clone(),
        out[1].clone(),
        out[2].clone(),
        out[3].clone(),
        out[4].clone(),
        out[5].clone(),
    ])
}

fn resolve_flat_stem(stem: &str, tinted: bool) -> [ResolvedFace; 6] {
    let f = ResolvedFace { texture: format!("block/{stem}"), tinted };
    [f.clone(), f.clone(), f.clone(), f.clone(), f.clone(), f]
}

/// Load a model JSON and follow `parent` links, merging texture maps
/// (child overrides parent). Returns (merged textures, child-most model JSON
/// that defines `elements` — geometry lives on ancestors like block/cube).
fn load_model_chain(
    archive: &mut ZipArchive<std::io::Cursor<&[u8]>>,
    model_ref: &str,
) -> Result<(BTreeMap<String, String>, Value)> {
    let mut models_seen: Vec<Value> = Vec::new();
    let mut current = model_ref.to_string();
    let mut textures = BTreeMap::new();
    for _ in 0..12 {
        let bytes = read_jar(archive, &format!("assets/minecraft/models/{current}.json"))?;
        let json: Value = serde_json::from_slice(&bytes)
            .with_context(|| format!("model {current}: json parse"))?;
        if let Some(t) = json.get("textures").and_then(|t| t.as_object()) {
            for (k, v) in t {
                if let Some(s) = v.as_str() {
                    textures.insert(k.clone(), s.to_string());
                }
            }
        }
        models_seen.push(json.clone());
        match json.get("parent").and_then(|p| p.as_str()) {
            Some(p) => current = p.trim_start_matches("minecraft:").to_string(),
            None => break,
        }
    }
    if models_seen.is_empty() {
        bail!("model chain empty for {model_ref}");
    }
    // Child-most model that carries the element geometry.
    let geometry = models_seen
        .iter()
        .find(|j| j.get("elements").is_some())
        .cloned()
        .unwrap_or(Value::Null);
    // Resolve internal "#ref" links now that the full map is merged.
    let resolved: BTreeMap<String, String> = textures
        .iter()
        .map(|(k, v)| {
            let mut val = v.clone();
            for _ in 0..4 {
                if let Some(stripped) = val.strip_prefix('#') {
                    match textures.get(stripped) {
                        Some(next) => val = next.clone(),
                        None => break,
                    }
                } else {
                    break;
                }
            }
            (k.clone(), val)
        })
        .collect();
    Ok((resolved, geometry))
}

fn read_jar(
    archive: &mut ZipArchive<std::io::Cursor<&[u8]>>,
    path: &str,
) -> Result<Vec<u8>> {
    let mut file = archive
        .by_name(path)
        .with_context(|| format!("jar entry missing: {path}"))?;
    let mut out = Vec::new();
    file.read_to_end(&mut out)?;
    Ok(out)
}

/// Decode any official PNG into RGBA8 (palette/grayscale expanded, 16-bit
/// stripped). Returns (width, height, rgba).
pub fn decode_rgba(bytes: &[u8]) -> Result<(u32, u32, Vec<u8>)> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().context("png read_info")?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).context("png next_frame")?;
    let (w, h) = (info.width, info.height);
    let samples = info.color_type.samples();
    let n = (w as usize) * (h as usize);
    let mut rgba = Vec::with_capacity(n * 4);
    for px in 0..n {
        let off = px * samples;
        match info.color_type {
            png::ColorType::Grayscale => {
                let g = buf[off];
                rgba.extend_from_slice(&[g, g, g, 255]);
            }
            png::ColorType::GrayscaleAlpha => {
                let (g, a) = (buf[off], buf[off + 1]);
                rgba.extend_from_slice(&[g, g, g, a]);
            }
            png::ColorType::Rgb => rgba.extend_from_slice(&[buf[off], buf[off + 1], buf[off + 2], 255]),
            png::ColorType::Rgba => rgba.extend_from_slice(&[buf[off], buf[off + 1], buf[off + 2], buf[off + 3]]),
            png::ColorType::Indexed => bail!("indexed survived EXPAND"),
        }
    }
    Ok((w, h, rgba))
}

/// Crop an animated strip (e.g. water_still 16x512) to its first frame.
pub fn first_frame(w: u32, h: u32, rgba: &[u8]) -> (u32, u32, Vec<u8>) {
    if h <= w {
        return (w, h, rgba.to_vec());
    }
    let side = w as usize;
    let mut out = Vec::with_capacity(side * side * 4);
    out.extend_from_slice(&rgba[..side * side * 4]);
    (w, w, out)
}

/// Bake an sRGB-ish multiply tint into RGBA pixels (0 = no tint).
fn bake_tint(rgba: &mut [u8], tint: u32) {
    if tint == 0 {
        return;
    }
    let (tr, tg, tb) = (((tint >> 16) & 0xFF) as u16, ((tint >> 8) & 0xFF) as u16, (tint & 0xFF) as u16);
    for px in rgba.chunks_exact_mut(4) {
        px[0] = ((px[0] as u16 * tr + 127) / 255) as u8;
        px[1] = ((px[1] as u16 * tg + 127) / 255) as u8;
        px[2] = ((px[2] as u16 * tb + 127) / 255) as u8;
    }
}

/// Build the atlas from the official jar bytes and emit generated Rust.
pub fn build(jar_bytes: &[u8], version: &str, client_sha1: &str) -> Result<String> {
    let cursor = std::io::Cursor::new(jar_bytes);
    let mut archive = zip::ZipArchive::new(cursor).context("open client jar")?;

    // face -> texture per runtime kind
    let mut faces: Vec<[ResolvedFace; 6]> = Vec::with_capacity(RUNTIME_PALETTE.len());
    for block in RUNTIME_PALETTE {
        if block == "air" {
            faces.push(resolve_flat_stem("__air__", false));
            continue;
        }
        faces.push(resolve_block(&mut archive, block)?);
    }

    // Collect unique textures (dedup: dirt is shared by grass bottom etc.)
    let mut tile_names: Vec<String> = Vec::new();
    let mut tile_bytes: Vec<Vec<u8>> = Vec::new();
    let mut face_tiles: Vec<[u16; 6]> = Vec::with_capacity(faces.len());
    let mut face_tints: Vec<[u32; 6]> = Vec::with_capacity(faces.len());
    for (ki, block_faces) in faces.iter().enumerate() {
        let mut ft = [0u16; 6];
        let mut tt = [0u32; 6];
        for (fi, face) in block_faces.iter().enumerate() {
            if RUNTIME_PALETTE[ki] == "air" {
                ft[fi] = u16::MAX;
                tt[fi] = 0;
                continue;
            }
            let stem = face
                .texture
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            let idx = match tile_names.iter().position(|n| *n == stem) {
                Some(i) => i,
                None => {
                    let raw = read_jar(
                        &mut archive,
                        &format!("assets/minecraft/textures/block/{stem}.png"),
                    )
                    .with_context(|| format!("texture block/{stem}.png"))?;
                    let (w, h, mut rgba) = decode_rgba(&raw)?;
                    let (w, h, rgba) = first_frame(w, h, &rgba);
                    if w != 16 || h != 16 {
                        bail!("texture {stem}: expected 16x16 (first frame), got {w}x{h}");
                    }
                    let mut bytes = rgba;
                    bake_tint(&mut bytes, tint_for_texture(&stem));
                    tile_names.push(stem.clone());
                    tile_bytes.push(bytes);
                    tile_names.len() - 1
                }
            };
            ft[fi] = idx as u16;
            tt[fi] = tint_for_texture(&stem);
        }
        face_tiles.push(ft);
        face_tints.push(tt);
    }

    // ---- emit Rust ----
    let mut src = String::with_capacity(tile_bytes.len() * 1100 + 4096);
    src.push_str(&format!(
        "// @generated by mc2rs-gen from the OFFICIAL client jar {version} (sha1 {client_sha1}).\n\
         // Resolution chain: blockstates -> models (parent+textures merged) -> textures/block.\n\
         // Animated strips cropped to first frame; plains-biome tints baked into pixels.\n\
         // DO NOT EDIT BY HAND — regenerate: `cargo run -p mc2rs-gen -- atlas --version {version}`\n\
         // (Mojang assets are NOT committed; this file is regenerated from the jar.)\n\n\
         pub const ATLAS_VERSION: &str = \"{version}\";\n\
         pub const TILE_SIZE: usize = 16;\n\
         pub const TILE_NONE: u16 = u16::MAX;\n\
         pub const N_KINDS: usize = {};\n\
         pub const KIND_NAMES: [&str; N_KINDS] = {:?};\n\n\
         pub const FACE_TILES: [[u16; 6]; N_KINDS] = [\n",
        RUNTIME_PALETTE.len(),
        RUNTIME_PALETTE
    ));
    for ft in &face_tiles {
        src.push_str(&format!("    [{}, {}, {}, {}, {}, {}],\n", ft[0], ft[1], ft[2], ft[3], ft[4], ft[5]));
    }
    src.push_str("];\n\npub const TINTS: [[u32; 6]; N_KINDS] = [\n");
    for tt in &face_tints {
        src.push_str(&format!("    [{:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}],\n", tt[0], tt[1], tt[2], tt[3], tt[4], tt[5]));
    }
    src.push_str("];\n\n");
    for (i, bytes) in tile_bytes.iter().enumerate() {
        src.push_str(&format!("static TILE_{}_RGBA: [u8; {}] = [", i, bytes.len()));
        for (j, b) in bytes.iter().enumerate() {
            if j % 24 == 0 {
                src.push('\n');
            }
            src.push_str(&format!("{b},"));
        }
        src.push_str("\n];\n");
    }
    src.push_str(&format!(
        "\n/// RGBA8 texel data, one entry per tile ({}x{}x4 bytes).\n\
         pub static TILES: [&[u8]; {}] = [",
        16, 16, tile_bytes.len()
    ));
    for i in 0..tile_bytes.len() {
        src.push_str(&format!("&TILE_{i}_RGBA,"));
    }
    src.push_str("];\n\n");
    src.push_str(
        "pub fn has_tiles() -> bool { !TILES.is_empty() }\n\n\
         pub fn tile_rgba(idx: u16) -> &'static [u8] {\n\
         \x20   TILES[idx as usize]\n\
         }\n\n\
         pub fn tile_name(idx: u16) -> &'static str {\n\
         \x20   TILE_NAMES[idx as usize]\n\
         }\n",
    );
    src.push_str(&format!(
        "\npub static TILE_NAMES: [&str; {}] = {:?};\n",
        tile_names.len(),
        tile_names
    ));
    Ok(src)
}

/// The biome colormap grids the vanilla grass/foliage/dry-foliage lookups
/// sample: `assets/minecraft/textures/colormap/{grass,foliage,dry_foliage}.png`
/// from the jar. Sampled at runtime by `mc-client/src/colormap.rs` with the
/// verbatim `ColorMapColorUtil.get` arithmetic (javap-audited, 26.2; the dry
/// foliage audit: `DryFoliageColor.get` delegates the SAME util with default
/// -10732494, and `Biome.getDryFoliageColorFromTexture` carries the same
/// caller-side Mth.clamp as its foliage sibling).
pub const COLORMAP_STEMS: [&str; 3] = ["grass", "foliage", "dry_foliage"];
/// The vanilla colormap side (a 256x256 grid, one 0xRRGGBB per cell).
pub const COLORMAP_SIDE: usize = 256;

/// Build the biome colormap grids from the official jar bytes and emit
/// generated Rust (`mc-registry/src/generated/colormap.rs`). The PNGs are
/// decoded row-major to opaque 0xRRGGBB (alpha verified opaque — the
/// vanilla loader reads them into an int[65536] via NativeImage).
pub fn build_colormap(jar_bytes: &[u8], version: &str, client_sha1: &str) -> Result<String> {
    let cursor = std::io::Cursor::new(jar_bytes);
    let mut archive = zip::ZipArchive::new(cursor).context("open client jar")?;

    let mut grids: Vec<(&str, Vec<u32>)> = Vec::with_capacity(COLORMAP_STEMS.len());
    for stem in COLORMAP_STEMS {
        let raw = read_jar(
            &mut archive,
            &format!("assets/minecraft/textures/colormap/{stem}.png"),
        )
        .with_context(|| format!("colormap {stem}.png"))?;
        let (w, h, rgba) = decode_rgba(&raw)?;
        let n = COLORMAP_SIDE * COLORMAP_SIDE;
        if w as usize != COLORMAP_SIDE || h as usize != COLORMAP_SIDE || rgba.len() != n * 4 {
            bail!(
                "colormap {stem}: expected {s}x{s} RGBA8, got {w}x{h} ({} bytes)",
                rgba.len(),
                s = COLORMAP_SIDE
            );
        }
        let mut grid = Vec::with_capacity(n);
        for px in rgba.chunks_exact(4) {
            if px[3] != 255 {
                bail!("colormap {stem}: non-opaque pixel (alpha {})", px[3]);
            }
            grid.push(((px[0] as u32) << 16) | ((px[1] as u32) << 8) | px[2] as u32);
        }
        grids.push((stem, grid));
    }

    let mut src = String::with_capacity(grids.len() * n_len(65536) + 2048);
    src.push_str(&format!(
        "// @generated by mc2rs-gen from the OFFICIAL client jar {version} (sha1 {client_sha1}).\n\
         // The biome colormap grids: assets/minecraft/textures/colormap/*.png decoded\n\
         // row-major to opaque 0xRRGGBB. Sampled verbatim (ColorMapColorUtil.get,\n\
         // javap-audited) by mc-client/src/colormap.rs. Do not consume without the\n\
         // caller-side Mth.clamp(0,1) — the util itself does NOT clamp.\n\
         // DO NOT EDIT BY HAND — regenerate: `cargo run -p mc2rs-gen -- atlas --version {version}`\n\
         // (Mojang assets are NOT committed; this file is regenerated from the jar.)\n\n\
         pub const COLORMAP_VERSION: &str = \"{version}\";\n\
         pub const COLORMAP_SIDE: usize = {};\n\
         /// The committed grids are the REAL jar data (a no-jar fallback build\n\
         /// emits `false` — consumers keep the baked-plains look instead).\n\
         pub const REAL_GRIDS: bool = true;\n\n",
        COLORMAP_SIDE
    ));
    for (stem, grid) in &grids {
        src.push_str(&format!(
            "/// The {stem}.png grid, row-major (index = (row << 8) | column).\n\
             pub static {}: [u32; {}] = [",
            stem.to_uppercase(),
            grid.len()
        ));
        for (j, v) in grid.iter().enumerate() {
            if j % 12 == 0 {
                src.push('\n');
            }
            src.push_str(&format!("{v:#x},"));
        }
        src.push_str("\n];\n\n");
    }
    src.push_str(
        "/// True when both grids carry the REAL jar data.\n\
         pub fn has_grids() -> bool { REAL_GRIDS }\n",
    );
    Ok(src)
}

/// Byte length estimate helper for the source buffer sizing above.
fn n_len(n: usize) -> usize {
    n * 10
}

/// Committed fallback source for colormap.rs (no jar available): compiles
/// everywhere, zeroes both grids and pins REAL_GRIDS=false so the client's
/// tint resolution keeps the baked-plains look (the same degraded mode the
/// TILE_NONE atlas fallback drives).
pub fn colormap_fallback_source(version: &str) -> String {
    let mut src = format!(
        "// @generated fallback — run `cargo run -p mc2rs-gen -- atlas --version {version}`\n\
         // against the official jar to replace this with real colormap data.\n\
         pub const COLORMAP_VERSION: &str = \"{version}\";\n\
         pub const COLORMAP_SIDE: usize = {};\n\
         pub const REAL_GRIDS: bool = false;\n\n",
        COLORMAP_SIDE
    );
    for stem in COLORMAP_STEMS {
        src.push_str(&format!(
            "pub static {}: [u32; {}] = [0; {}];\n\n",
            stem.to_uppercase(),
            COLORMAP_SIDE * COLORMAP_SIDE,
            COLORMAP_SIDE * COLORMAP_SIDE
        ));
    }
    src.push_str("pub fn has_grids() -> bool { REAL_GRIDS }\n");
    src
}

/// Committed fallback source: compiles everywhere, maps every face to
/// TILE_NONE so the renderer uses its palette path until the real atlas
/// is generated from the jar.
pub fn fallback_source(version: &str) -> String {
    format!(
        "// @generated fallback — run `cargo run -p mc2rs-gen -- atlas --version {version}`\n\
         // against the official jar to replace this with real texture data.\n\
         pub const ATLAS_VERSION: &str = \"{version}\";\n\
         pub const TILE_SIZE: usize = 16;\n\
         pub const TILE_NONE: u16 = u16::MAX;\n\
         pub const N_KINDS: usize = {};\n\
         pub const KIND_NAMES: [&str; N_KINDS] = {:?};\n\
         pub const FACE_TILES: [[u16; 6]; N_KINDS] = [[TILE_NONE; 6]; N_KINDS];\n\
         pub const TINTS: [[u32; 6]; N_KINDS] = [[0; 6]; N_KINDS];\n\
         pub static TILES: [&[u8]; 0] = [];\n\
         pub static TILE_NAMES: [&str; 0] = [];\n\
         pub fn has_tiles() -> bool {{ !TILES.is_empty() }}\n\
         pub fn tile_rgba(idx: u16) -> &'static [u8] {{ TILES[idx as usize] }}\n\
         pub fn tile_name(idx: u16) -> &'static str {{ TILE_NAMES[idx as usize] }}\n",
        RUNTIME_PALETTE.len(),
        RUNTIME_PALETTE
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_palette_matches_client_ids() {
        // world.rs: AIR=0 STONE=1 DIRT=2 GRASS=3 LOG=4 LEAVES=5 WATER=6
        // SAND=7, then the generated members 8..=23
        assert_eq!(RUNTIME_PALETTE[0], "air");
        assert_eq!(RUNTIME_PALETTE[1], "stone");
        assert_eq!(RUNTIME_PALETTE[3], "grass_block");
        assert_eq!(RUNTIME_PALETTE[5], "oak_leaves");
        assert_eq!(RUNTIME_PALETTE[6], "water");
        assert_eq!(RUNTIME_PALETTE[7], "sand");
        // the generated members — ids 8..=23, appended after the classics
        assert_eq!(RUNTIME_PALETTE[8], "granite");
        assert_eq!(RUNTIME_PALETTE[9], "diorite");
        assert_eq!(RUNTIME_PALETTE[10], "andesite");
        assert_eq!(RUNTIME_PALETTE[11], "deepslate");
        assert_eq!(RUNTIME_PALETTE[12], "cobbled_deepslate");
        assert_eq!(RUNTIME_PALETTE[13], "gravel");
        assert_eq!(RUNTIME_PALETTE[14], "bedrock");
        assert_eq!(RUNTIME_PALETTE[15], "coal_ore");
        assert_eq!(RUNTIME_PALETTE[16], "copper_ore");
        assert_eq!(RUNTIME_PALETTE[17], "iron_ore");
        assert_eq!(RUNTIME_PALETTE[18], "gold_ore");
        assert_eq!(RUNTIME_PALETTE[19], "redstone_ore");
        assert_eq!(RUNTIME_PALETTE[20], "lapis_ore");
        assert_eq!(RUNTIME_PALETTE[21], "diamond_ore");
        assert_eq!(RUNTIME_PALETTE[22], "emerald_ore");
        assert_eq!(RUNTIME_PALETTE[23], "netherrack");
        assert_eq!(RUNTIME_PALETTE[24], "pale_oak_leaves");
        assert_eq!(FACE_KEYS, ["east", "west", "up", "down", "south", "north"]);
    }

    #[test]
    fn tint_map_covers_grayscale_masters() {
        assert_eq!(tint_for_texture("grass_block_top"), TINT_GRASS);
        assert_eq!(tint_for_texture("oak_leaves"), TINT_FOLIAGE);
        assert_eq!(tint_for_texture("water_still"), TINT_WATER);
        assert_eq!(tint_for_texture("stone"), 0, "untinted texture");
        // the AUDITED verdict (12th audit): pale oak leaves carry NO tint —
        // vanilla's BlockColors.createDefault() registers no dryFoliage
        // source for them (leaf_litter is the sole consumer), the model's
        // tintindex falls back to BLANK_LAYER and the PRE-COLORED texture
        // (probe-verified: zero pure-gray pixels) renders as authored.
        assert_eq!(tint_for_texture("pale_oak_leaves"), 0, "pre-colored");
    }

    #[test]
    fn bake_tint_multiplies_channels() {
        let mut px = vec![255u8, 255, 255, 255, 100, 100, 100, 255];
        bake_tint(&mut px, 0x91BD59);
        assert_eq!(px[0], 0x91, "white * tint.r");
        assert_eq!(px[1], 0xBD);
        assert_eq!(px[2], 0x59);
        assert_eq!(px[3], 255, "alpha preserved");
        let expect = ((100u16 * 0x91 + 127) / 255) as u8;
        assert_eq!(px[4], expect);
        // zero tint is a no-op
        let mut none = vec![200u8, 100, 50, 255];
        bake_tint(&mut none, 0);
        assert_eq!(none, vec![200, 100, 50, 255]);
    }

    #[test]
    fn animated_strip_cropped_to_first_frame() {
        // 2x6 strip of two 2x2 frames
        let w = 2u32;
        let h = 6u32;
        let mut rgba = Vec::new();
        rgba.extend_from_slice(&[1, 0, 0, 255, 2, 0, 0, 255, 3, 0, 0, 255, 4, 0, 0, 255]); // frame 1
        let frame2 = [9u8, 0, 0, 255, 9, 0, 0, 255, 9, 0, 0, 255, 9, 0, 0, 255];
        rgba.extend_from_slice(&frame2); // frame 2
        rgba.extend_from_slice(&frame2); // frame 3
        let (w2, h2, out) = first_frame(w, h, &rgba);
        assert_eq!((w2, h2), (2, 2));
        assert_eq!(out, vec![1, 0, 0, 255, 2, 0, 0, 255, 3, 0, 0, 255, 4, 0, 0, 255]);
        // square input untouched
        let sq = vec![5u8; 16];
        let (w3, h3, out3) = first_frame(2, 2, &sq);
        assert_eq!((w3, h3), (2, 2));
        assert_eq!(out3, sq);
    }

    #[test]
    fn fallback_source_is_valid_shape() {
        let src = fallback_source("26.2");
        assert!(src.contains("pub static TILES: [&[u8]; 0]"));
        assert!(src.contains("TILE_NONE"));
        assert!(src.contains("26.2"));
    }

    #[test]
    fn water_is_special_cased_without_jar() {
        // resolve_flat_stem must give water_still on all six faces
        let f = resolve_flat_stem("water_still", true);
        assert!(f.iter().all(|face| face.texture == "block/water_still"));
        assert!(f.iter().all(|face| face.tinted));
    }
}

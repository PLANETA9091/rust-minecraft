//! Special item renderer geometry — the parsed `dump/special_models_dump.txt`
//! evidence (the reflection dump of the running client's OWN
//! LayerDefinitions) emitted as a generated table. Every cube face is the
//! client's RESOLVED polygon: four vertices, each with the model-space
//! position and the NORMALIZED texture UV — no decode law is ported, the
//! running client's own bake IS the data (the dumper's javap-grade
//! evidence arc, the ReflectContainerClick pattern).
//!
//! The dump's float text is the JVM's own Float.toString (uniquely
//! round-trippable); Rust's f32 parse is correctly rounded, so the table
//! carries bit-exact values. Spot pins ride the dump's hex floats.

use anyhow::{Context, Result};
use std::path::Path;

/// One cube face: four vertices of (x, y, z, u, v) — the client's own
/// resolved polygon (the vanilla Cube face order: down, up, west, north,
/// east, south; the dump's polygon array order).
pub struct SpecialFace {
    pub verts: [[f32; 5]; 4],
}

pub struct SpecialCube {
    /// The resolved bounds (minX..maxZ) — the grow already folded in.
    pub min: [f32; 3],
    pub max: [f32; 3],
    pub faces: Vec<SpecialFace>,
}

pub struct SpecialPart {
    pub name: String,
    /// The part's parent index within the layer's flat part list
    /// (usize::MAX = the root itself).
    pub parent: usize,
    pub pose: [f32; 3],
    pub rot: [f32; 3],
    pub scale: [f32; 3],
    pub visible: bool,
    pub cubes: Vec<SpecialCube>,
}

pub struct SpecialLayer {
    /// The layer's ModelLayerLocation id, e.g. "minecraft:shield#main".
    pub id: String,
    pub tex_w: u32,
    pub tex_h: u32,
    pub parts: Vec<SpecialPart>,
}

pub struct SpecialData {
    pub layers: Vec<SpecialLayer>,
    /// (dye name, textureDiffuseColor) — the banner tints' source.
    pub dyes: Vec<(String, i32)>,
}

fn parse_float(tok: &str) -> Result<f32> {
    // the dump prints "<hexfloat>/<decimal>"; the decimal round-trips
    let dec = tok.split('/').nth(1).with_context(|| format!("float {tok:?}"))?;
    dec.parse::<f32>().with_context(|| format!("parse {tok:?}"))
}

/// Parse the dump text the checked-in dumper wrote.
pub fn parse_dump(text: &str) -> Result<SpecialData> {
    let mut layers: Vec<SpecialLayer> = Vec::new();
    let mut dyes: Vec<(String, i32)> = Vec::new();

    let mut cur_layer: Option<SpecialLayer> = None;
    // the part stack: the indices of the OPEN parts (ancestors first)
    let mut stack: Vec<usize> = Vec::new();
    let mut cur_cube: Option<SpecialCube> = None;

    macro_rules! flush_cube {
        () => {
            if let (Some(c), Some(l)) = (cur_cube.take(), cur_layer.as_mut()) {
                if let Some(&pi) = stack.last() {
                    l.parts[pi].cubes.push(c);
                }
            }
        };
    }

    for (ln, raw) in text.lines().enumerate() {
        let ind = raw.len() - raw.trim_start().len();
        let line = raw.trim_start();
        let ctx = || format!("line {}", ln + 1);

        if line.starts_with("=== LAYER ") {
            if cur_layer.is_some() {
                flush_cube!();
                layers.push(cur_layer.take().with_context(ctx)?);
                stack.clear();
            }
            cur_layer = Some(SpecialLayer {
                id: line["=== LAYER ".len()..].trim().to_string(),
                tex_w: 0,
                tex_h: 0,
                parts: Vec::new(),
            });
            continue;
        }
        if let Some(rest) = line.strip_prefix("TEX ") {
            let Some(l) = cur_layer.as_mut() else { continue };
            let mut it = rest.split_whitespace();
            l.tex_w = it.next().with_context(ctx)?.parse()?;
            l.tex_h = it.next().with_context(ctx)?.parse()?;
            continue;
        }
        if let Some(rest) = line.strip_prefix("PART ") {
            flush_cube!();
            // part depth: the dump indents the root at 2 spaces, each
            // child level +2 — so level = ind/2 - 1
            let depth = ind / 2 - 1;
            stack.truncate(depth);
            let parent = stack.last().copied().unwrap_or(usize::MAX);
            let idx = cur_layer.as_ref().with_context(ctx)?.parts.len();
            let l = cur_layer.as_mut().with_context(ctx)?;
            l.parts.push(SpecialPart {
                name: rest.trim().to_string(),
                parent,
                pose: [0.0; 3],
                rot: [0.0; 3],
                scale: [1.0; 3],
                visible: true,
                cubes: Vec::new(),
            });
            stack.push(idx);
            continue;
        }
        if let Some(rest) = line.strip_prefix("POSE") {
            let l = cur_layer.as_mut().with_context(ctx)?;
            let &pi = stack.last().with_context(ctx)?;
            for tok in rest.split_whitespace() {
                let Some((k, v)) = tok.split_once('=') else { continue };
                if !matches!(k, "x" | "y" | "z" | "xRot" | "yRot" | "zRot"
                    | "xScale" | "yScale" | "zScale")
                {
                    continue; // visible=true and friends are not floats
                }
                let f = parse_float(v)?;
                match k {
                    "x" => l.parts[pi].pose[0] = f,
                    "y" => l.parts[pi].pose[1] = f,
                    "z" => l.parts[pi].pose[2] = f,
                    "xRot" => l.parts[pi].rot[0] = f,
                    "yRot" => l.parts[pi].rot[1] = f,
                    "zRot" => l.parts[pi].rot[2] = f,
                    "xScale" => l.parts[pi].scale[0] = f,
                    "yScale" => l.parts[pi].scale[1] = f,
                    "zScale" => l.parts[pi].scale[2] = f,
                    _ => {}
                }
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("CUBE ") {
            flush_cube!();
            let _l = cur_layer.as_ref().with_context(ctx)?;
            let &_pi = stack.last().with_context(ctx)?;
            let mut min = [0f32; 3];
            let mut max = [0f32; 3];
            for tok in rest.split_whitespace() {
                let Some((k, v)) = tok.split_once('=') else { continue };
                match k {
                    "minX" => min[0] = parse_float(v)?,
                    "minY" => min[1] = parse_float(v)?,
                    "minZ" => min[2] = parse_float(v)?,
                    "maxX" => max[0] = parse_float(v)?,
                    "maxY" => max[1] = parse_float(v)?,
                    "maxZ" => max[2] = parse_float(v)?,
                    _ => {}
                }
            }
            cur_cube = Some(SpecialCube { min, max, faces: Vec::new() });
            continue;
        }
        if let Some(rest) = line.strip_prefix("FACE verts=") {
            let c = cur_cube.as_mut().with_context(ctx)?;
            let mut verts: Vec<[f32; 5]> = Vec::new();
            for chunk in rest
                .trim_start_matches('[')
                .trim_end_matches(']')
                .split("][")
            {
                let body = chunk.strip_prefix("xyz=").with_context(ctx)?;
                let (xyz, uv) = body.split_once(" uv=").with_context(ctx)?;
                let mut f = [0f32; 5];
                for (i, tok) in xyz.split_whitespace().enumerate() {
                    f[i] = parse_float(tok)?;
                }
                for (i, tok) in uv.split_whitespace().enumerate() {
                    f[3 + i] = parse_float(tok)?;
                }
                verts.push(f);
            }
            anyhow::ensure!(verts.len() == 4, "{}: a face needs 4 verts", ctx());
            c.faces.push(SpecialFace {
                verts: [verts[0], verts[1], verts[2], verts[3]],
            });
            continue;
        }
        if line.contains("textureDiffuseColor=") {
            let toks: Vec<&str> = line.split_whitespace().collect();
            let name = toks[0].to_string();
            let mut color = None;
            for t in &toks[1..] {
                if let Some(v) = t.strip_prefix("textureDiffuseColor=") {
                    color = Some(v.parse::<i32>()?);
                }
            }
            if let Some(c) = color {
                dyes.push((name, c));
            }
            continue;
        }
    }
    if let Some(mut l) = cur_layer.take() {
        if let (Some(c), Some(&pi)) = (cur_cube.take(), stack.last()) {
            l.parts[pi].cubes.push(c);
        }
        layers.push(l);
    }
    anyhow::ensure!(!layers.is_empty(), "the dump carried no layers");
    anyhow::ensure!(!dyes.is_empty(), "the dump carried no dye colors");
    Ok(SpecialData { layers, dyes })
}

/// Emit `generated/special_models.rs`: the layers, parts, cubes and dye
/// colors as statics, the part tree as a flat parent-indexed list.
pub fn emit(data: &SpecialData) -> Result<String> {
    let mut out = String::with_capacity(512 << 10);
    out.push_str("//! GENERATED by mc2rs-gen from dump/special_models_dump.txt — the\n");
    out.push_str("//! reflection dump of the running client's own LayerDefinitions\n");
    out.push_str("//! (the special item renderers' geometry: shield, trident, chest,\n");
    out.push_str("//! shulker_box, banner, heads, conduit, decorated_pot, copper_golem\n");
    out.push_str("//! statue). Every face is the client's RESOLVED polygon (four verts,\n");
    out.push_str("//! model-space xyz + normalized uv) — no decode law is ported; the\n");
    out.push_str("//! dump's hex floats are the bit-exact oracle.\n\n");

    out.push_str("/// One cube face: four verts of (x, y, z, u, v).\n");
    out.push_str("pub struct SpecialFace {\n    pub verts: [[f32; 5]; 4],\n}\n\n");
    out.push_str("pub struct SpecialCube {\n    pub min: [f32; 3],\n    pub max: [f32; 3],\n    pub faces: &'static [SpecialFace],\n}\n\n");
    out.push_str("pub struct SpecialPart {\n    pub name: &'static str,\n    pub parent: i32,\n    pub pose: [f32; 3],\n    pub rot: [f32; 3],\n    pub scale: [f32; 3],\n    pub visible: bool,\n    pub cubes: &'static [SpecialCube],\n}\n\n");
    out.push_str("pub struct SpecialLayer {\n    pub tex_w: u32,\n    pub tex_h: u32,\n    pub parts: &'static [SpecialPart],\n}\n\n");

    for l in &data.layers {
        let slug = layer_slug(&l.id);
        out.push_str(&format!(
            "pub static {slug}: SpecialLayer = SpecialLayer {{\n    tex_w: {}, tex_h: {},\n    parts: &[\n",
            l.tex_w, l.tex_h
        ));
        for p in &l.parts {
            let parent: i32 = if p.parent == usize::MAX { -1 } else { p.parent as i32 };
            out.push_str(&format!(
                "        SpecialPart {{ name: {:?}, parent: {}, pose: [{:?}, {:?}, {:?}], rot: [{:?}, {:?}, {:?}], scale: [{:?}, {:?}, {:?}], visible: {}, cubes: &[\n",
                p.name, parent,
                p.pose[0], p.pose[1], p.pose[2],
                p.rot[0], p.rot[1], p.rot[2],
                p.scale[0], p.scale[1], p.scale[2],
                p.visible,
            ));
            for c in &p.cubes {
                out.push_str(&format!(
                    "            SpecialCube {{ min: [{:?}, {:?}, {:?}], max: [{:?}, {:?}, {:?}], faces: &[\n",
                    c.min[0], c.min[1], c.min[2], c.max[0], c.max[1], c.max[2],
                ));
                for f in &c.faces {
                    out.push_str("                SpecialFace { verts: [\n");
                    for v in &f.verts {
                        out.push_str(&format!(
                            "                    [{:?}, {:?}, {:?}, {:?}, {:?}],\n",
                            v[0], v[1], v[2], v[3], v[4]
                        ));
                    }
                    out.push_str("                ] },\n");
                }
                out.push_str("            ] },\n");
            }
            out.push_str("        ] },\n");
        }
        out.push_str("    ],\n};\n\n");
    }

    out.push_str("pub static SPECIAL_LAYERS: &[(&str, &SpecialLayer)] = &[\n");
    for l in &data.layers {
        let slug = layer_slug(&l.id);
        out.push_str(&format!("    ({:?}, &{slug}),\n", l.id));
    }
    out.push_str("];\n\n");

    out.push_str("/// (dye name, textureDiffuseColor) — the banner tint's source color.\n");
    out.push_str("pub static DYE_TEXTURE_DIFFUSE: [(&str, i32); ");
    out.push_str(&data.dyes.len().to_string());
    out.push_str("] = [\n");
    for (n, c) in &data.dyes {
        out.push_str(&format!("    (\"{n}\", {c}),\n"));
    }
    out.push_str("];\n");

    Ok(out)
}

/// The texture namespace additions the special renderers need — the
/// EXACT texture ids itemgen's renderer law binds (the same census, the
/// drift guarded by the registry's suite: every SpecialLayerRef texture
/// must resolve in TEXTURE_IDS). The banner FLAG layer binds the
/// PER-COLOR pre-tinted variant ("..._white" etc. — the DyeColor
/// textureDiffuseColor multiply, generation-time: the special lane's
/// raster carries no vertex-color channel); the ROD rides the raw
/// pattern-canvas sprite untinted. Sources: (namespace id, png path
/// under textures/, per-channel tint RGB).
pub fn special_texture_variants() -> Vec<(String, String, Option<u32>)> {
    // the raw, untinted ids the renderer laws bind
    const RAW: &[&str] = &[
        "entity/trident/trident",
        "entity/shield/shield_base_nopattern",
        "entity/chest/normal",
        "entity/chest/trapped",
        "entity/chest/ender",
        "entity/chest/copper",
        "entity/chest/copper_exposed",
        "entity/chest/copper_weathered",
        "entity/chest/copper_oxidized",
        "entity/shulker/shulker",
        "entity/shulker/shulker_white",
        "entity/shulker/shulker_orange",
        "entity/shulker/shulker_magenta",
        "entity/shulker/shulker_light_blue",
        "entity/shulker/shulker_yellow",
        "entity/shulker/shulker_lime",
        "entity/shulker/shulker_pink",
        "entity/shulker/shulker_gray",
        "entity/shulker/shulker_light_gray",
        "entity/shulker/shulker_cyan",
        "entity/shulker/shulker_purple",
        "entity/shulker/shulker_blue",
        "entity/shulker/shulker_brown",
        "entity/shulker/shulker_green",
        "entity/shulker/shulker_red",
        "entity/shulker/shulker_black",
        "entity/banner/base",
        "entity/conduit/base",
        "entity/conduit/cage",
        "entity/conduit/closed_eye",
        "entity/decorated_pot/decorated_pot_base",
        "entity/decorated_pot/decorated_pot_side",
        "entity/creeper/creeper",
        "entity/skeleton/skeleton",
        "entity/skeleton/wither_skeleton",
        "entity/zombie/zombie",
        "entity/piglin/piglin",
        "entity/enderdragon/dragon",
        "entity/player/slim/steve",
        "entity/copper_golem/copper_golem",
        "entity/copper_golem/copper_golem_exposed",
        "entity/copper_golem/copper_golem_oxidized",
        "entity/copper_golem/copper_golem_weathered",
    ];
    let mut out: Vec<(String, String, Option<u32>)> =
        RAW.iter().map(|id| (id.to_string(), id.to_string(), None)).collect();
    // the banner flag's 16 pre-tinted variants over the ONE base png
    for (name, rgb) in DYE_TEXTURE_DIFFUSE_RGB {
        out.push((
            format!("entity/banner/banner_base_{name}"),
            "entity/banner/banner_base".to_string(),
            Some(*rgb),
        ));
    }
    out
}

/// The 16 DyeColor textureDiffuseColors as 0xRRGGBB (the dump's i32s
/// masked — the top byte is the alpha the vertex color never carries).
pub const DYE_TEXTURE_DIFFUSE_RGB: &[(&str, u32)] = &[
    ("white", 0xf9fffe),
    ("orange", 0xf9801d),
    ("magenta", 0xc74ebd),
    ("light_blue", 0x3ab3da),
    ("yellow", 0xfed83d),
    ("lime", 0x80c71f),
    ("pink", 0xf38baa),
    ("gray", 0x474f52),
    ("light_gray", 0x9d9d97),
    ("cyan", 0x169c9c),
    ("purple", 0x8932b8),
    ("blue", 0x3c44aa),
    ("brown", 0x835432),
    ("green", 0x5e7c16),
    ("red", 0xb02e26),
    ("black", 0x1d1d21),
];

fn layer_slug(id: &str) -> String {
    let mut s = String::from("L");
    for ch in id.chars() {
        if ch.is_ascii_alphanumeric() {
            s.push(ch.to_ascii_uppercase());
        } else {
            s.push('_');
        }
    }
    s
}

/// Generate the table text from a checked-in dump file.
pub fn generate(dump_path: &Path) -> Result<String> {
    let text = std::fs::read_to_string(dump_path)
        .with_context(|| format!("read {}", dump_path.display()))?;
    let data = parse_dump(&text)?;
    emit(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_dump_shield_and_dyes() {
        // the checked-in dump: the shield's plate carries the resolved
        // bounds and six faces; the dyes carry white's textureDiffuseColor
        let data = parse_dump(
            std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/dump/special_models_dump.txt"
            ))
            .unwrap()
            .as_str(),
        )
        .unwrap();
        let shield = data
            .layers
            .iter()
            .find(|l| l.id == "minecraft:shield#main")
            .expect("shield layer");
        assert_eq!((shield.tex_w, shield.tex_h), (64, 64));
        assert_eq!(shield.parts[0].name, "?");
        let plate = shield
            .parts
            .iter()
            .find(|p| p.name == "plate")
            .expect("plate");
        assert_eq!(plate.cubes.len(), 1);
        let c = &plate.cubes[0];
        assert_eq!(c.min, [-6.0, -11.0, -2.0]);
        assert_eq!(c.max, [6.0, 11.0, -1.0]);
        assert_eq!(c.faces.len(), 6);
        // the top face's first vertex, hex-pinned against the dump line
        let top = &c.faces[0].verts[0];
        assert_eq!(top[0].to_bits(), 6.0f32.to_bits());
        assert_eq!(top[3], 13.0 / 64.0, "u = the dump's 0.203125");
        let dyes = data.dyes.iter().find(|d| d.0 == "white").unwrap();
        assert_eq!(dyes.1, -393218);
        assert_eq!(data.layers.len(), 23);
        assert_eq!(data.dyes.len(), 16);
    }

    #[test]
    fn the_emit_reaches_all_layers_and_parents_stay_ordered() {
        let data = parse_dump(
            std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/dump/special_models_dump.txt"
            ))
            .unwrap()
            .as_str(),
        )
        .unwrap();
        let text = emit(&data).unwrap();
        assert_eq!(text.matches("pub static L").count(), 23);
        // every part's parent index precedes the part itself (tree order)
        for l in &data.layers {
            for (i, p) in l.parts.iter().enumerate() {
                if p.parent != usize::MAX {
                    assert!(p.parent < i, "{} part {} after its parent", l.id, p.name);
                }
            }
        }
    }

    /// The texture-supply law's own census: 59 additions (43 raw + the
    /// banner flag's 16 pre-tinted variants over the ONE base png), the
    /// DyeColor law checked dye-for-dye against the dump's DYECOLOR
    /// section, and every tinted variant's source the raw base.
    #[test]
    fn the_texture_variants_census_and_the_dye_law_hold() {
        let data = parse_dump(
            std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/dump/special_models_dump.txt"
            ))
            .unwrap()
            .as_str(),
        )
        .unwrap();
        let variants = special_texture_variants();
        assert_eq!(variants.len(), 59);
        let banner_variants = variants
            .iter()
            .filter(|(id, _, _)| id.starts_with("entity/banner/banner_base_"))
            .count();
        assert_eq!(banner_variants, 16);
        assert!(variants
            .iter()
            .all(|(_, src, _)| !src.starts_with("textures/")));
        // every tinted variant's source IS the raw banner base, and its
        // tint matches the dye the id names
        for (id, src, tint) in &variants {
            match tint {
                Some(rgb) => {
                    let color = id
                        .strip_prefix("entity/banner/banner_base_")
                        .unwrap_or_else(|| panic!("tinted {id} not a banner variant"));
                    let law = DYE_TEXTURE_DIFFUSE_RGB
                        .iter()
                        .find(|(n, _)| **n == *color)
                        .unwrap_or_else(|| panic!("dye {color}"))
                        .1;
                    assert_eq!(*rgb, law, "variant {id}");
                    assert_eq!(src, "entity/banner/banner_base");
                }
                None => assert_eq!(id, src, "raw {id}"),
            }
        }
        // the dye law: the dump's textureDiffuseColor i32s, masked
        for (name, rgb) in DYE_TEXTURE_DIFFUSE_RGB {
            let (dump_name, dump_i32) = data
                .dyes
                .iter()
                .find(|(n, _)| n == name)
                .unwrap_or_else(|| panic!("dye {name}"));
            assert_eq!(dump_name, name);
            assert_eq!(*dump_i32 as u32 & 0xffffff, *rgb, "dye {name}");
        }
    }
}

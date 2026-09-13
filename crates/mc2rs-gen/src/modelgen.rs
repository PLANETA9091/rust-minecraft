//! Block/item model files — `assets/minecraft/models/{block,item}/` — the
//! held-item arc's data half round 2 (the biome-tags data-first precedent:
//! the table lands first, the consumer round comes later). Every model is
//! parsed with the extend-parser-first refusals: the parent chain is
//! validated to terminate (file or `builtin/*`, no cycles), texture stanzas
//! and display transforms are retained as value-faithful canonical compact
//! JSON (26.2's object-valued texture entries — redstone lines, glass
//! panes — ride through untouched), and the `elements` geometry is RETAINED
//! as an honest skip list, never silently dropped. The census ran before
//! any emitter code and is cross-checked by an independent python pass.

use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// The jar directory the model files live at.
pub const MODELS_DIR: &str = "assets/minecraft/models/";

/// The only two model directories the 26.2 jar ships (census: 2657 block /
/// 1271 item); a file anywhere else under models/ refuses (strict prefix).
pub const PREFIXES: [&str; 2] = ["block/", "item/"];

/// The accepted top-level model keys — the closed 26.2 census set. The two
/// outliers the full census caught: `groups` (Blockbench editor metadata the
/// engine ignores — retained value-faithfully, 4 custom_fence files) and
/// `texture_size` (the element-UV size override — typed, heavy_core).
const MODEL_KEYS: [&str; 8] = [
    "parent",
    "textures",
    "elements",
    "display",
    "gui_light",
    "ambientocclusion",
    "texture_size",
    "groups",
];

/// The display contexts. 26.2 adds `on_shelf` (the shelf block's display).
const DISPLAY_CONTEXTS: [&str; 9] = [
    "thirdperson_righthand",
    "thirdperson_lefthand",
    "firstperson_righthand",
    "firstperson_lefthand",
    "gui",
    "head",
    "ground",
    "fixed",
    "on_shelf",
];

/// The per-context transform keys (the closed census set: every transform
/// value is a numeric array; scalar scales do not occur in 26.2).
const TRANSFORM_KEYS: [&str; 3] = ["rotation", "translation", "scale"];

/// The keys a special (object-valued) texture entry may carry — 26.2 ships
/// exactly `{sprite, force_translucent}` on 163 entries (redstone, glass).
const TEXTURE_OBJECT_KEYS: [&str; 2] = ["sprite", "force_translucent"];

/// The gui_light enum.
const GUI_LIGHTS: [&str; 2] = ["front", "side"];

/// Parents that terminate the chain without a file (vanilla built-ins:
/// `builtin/generated` is the item-layer extruder).
const BUILTIN_PREFIX: &str = "builtin/";

/// One parsed model file.
#[derive(Debug)]
pub struct ModelRow {
    /// The parent path with a `minecraft:` namespace stripped (26.2 mixes
    /// bare and namespaced parents — both normalize to the bare path);
    /// `builtin/*` parents are kept verbatim and terminate the chain.
    pub parent: Option<String>,
    /// The texture stanza as canonical compact JSON (value-faithful).
    pub textures: Option<String>,
    /// The display stanza as canonical compact JSON (value-faithful).
    pub display: Option<String>,
    /// The `elements` array length (the geometry is retained in the skip
    /// list, never parsed here).
    pub elements: u16,
    /// The gui_light enum value.
    pub gui_light: Option<String>,
    /// The ambient-occlusion override.
    pub ambientocclusion: Option<bool>,
    /// The element-UV texture size override (the default is 16x16).
    pub texture_size: Option<(u16, u16)>,
    /// The Blockbench `groups` editor metadata, retained value-faithfully
    /// as canonical compact JSON (the engine ignores it; we never drop).
    pub groups: Option<String>,
}

/// The parsed table the generated file carries.
#[derive(Debug)]
pub struct ModelsData {
    /// All models, keyed by the bare path ("block/stone", "item/bow").
    pub models: BTreeMap<String, ModelRow>,
    /// The ids whose model carries `elements` — the honest skip record.
    pub element_models: Vec<String>,
    /// Census split (block/ vs item/), for the emit-time accounting.
    pub blocks: u32,
    pub items: u32,
}

/// Normalize a parent reference: strip the only allowed namespace.
fn normalize_parent(model: &str, raw: &str) -> Result<String> {
    let p = match raw.split_once(':') {
        Some(("minecraft", path)) => path,
        Some((ns, _)) => {
            anyhow::bail!("{model}: parent namespace \"{ns}:\" is not minecraft: (extend the parser first)")
        }
        None => raw,
    };
    anyhow::ensure!(!p.is_empty(), "{model}: parent path is empty");
    Ok(p.to_string())
}

/// Validate a texture value: a string path/var-ref, or the closed
/// object shape `{sprite, force_translucent}`.
fn validate_texture_value(model: &str, key: &str, v: &Value) -> Result<()> {
    match v {
        Value::String(_) => Ok(()),
        Value::Object(o) => {
            for k in o.keys() {
                anyhow::ensure!(
                    TEXTURE_OBJECT_KEYS.contains(&k.as_str()),
                    "{model}: textures.{key}: unexpected object key \"{k}\" (extend the parser first)"
                );
            }
            if let Some(s) = o.get("sprite") {
                anyhow::ensure!(
                    s.is_string(),
                    "{model}: textures.{key}.sprite must be a string"
                );
            }
            if let Some(b) = o.get("force_translucent") {
                anyhow::ensure!(
                    b.is_boolean(),
                    "{model}: textures.{key}.force_translucent must be a bool"
                );
            }
            Ok(())
        }
        _ => anyhow::bail!("{model}: textures.{key} must be a string or an object"),
    }
}

/// Validate the display stanza: the nine census contexts, the three
/// transform keys, numeric-array values.
fn validate_display(model: &str, d: &Value) -> Result<()> {
    let d = d
        .as_object()
        .with_context(|| format!("{model}: display must be an object"))?;
    for (ctx, tr) in d {
        anyhow::ensure!(
            DISPLAY_CONTEXTS.contains(&ctx.as_str()),
            "{model}: display context \"{ctx}\" is not a vanilla context (extend the parser first)"
        );
        let tr = tr
            .as_object()
            .with_context(|| format!("{model}: display.{ctx} must be an object"))?;
        for (k, v) in tr {
            anyhow::ensure!(
                TRANSFORM_KEYS.contains(&k.as_str()),
                "{model}: display.{ctx}.{k} is not a transform key (extend the parser first)"
            );
            let nums = v
                .as_array()
                .with_context(|| format!("{model}: display.{ctx}.{k} must be an array"))?;
            anyhow::ensure!(
                !nums.is_empty() && nums.len() <= 3 && nums.iter().all(Value::is_number),
                "{model}: display.{ctx}.{k} must be 1..=3 numbers"
            );
        }
    }
    Ok(())
}

/// Parse every model file out of the jar.
pub fn generate<R: std::io::Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
) -> Result<ModelsData> {
    let names: Vec<String> = archive
        .file_names()
        .filter(|n| n.starts_with(MODELS_DIR) && n.ends_with(".json"))
        .map(str::to_string)
        .collect();
    anyhow::ensure!(!names.is_empty(), "the jar ships no {MODELS_DIR} models");

    let mut models: BTreeMap<String, ModelRow> = BTreeMap::new();
    let mut blocks: u32 = 0;
    let mut items: u32 = 0;

    for name in &names {
        let rel = name[MODELS_DIR.len()..].trim_end_matches(".json");
        let prefix = rel.split('/').next().unwrap_or("");
        anyhow::ensure!(
            PREFIXES.contains(&format!("{prefix}/").as_str()),
            "{name}: model directory \"{prefix}/\" is not one of {PREFIXES:?} (extend the parser first)"
        );
        anyhow::ensure!(!rel.is_empty(), "empty model id from {name}");
        if prefix == "block" { blocks += 1 } else { items += 1 }

        let bytes = crate::extract::read_entry(archive, name)
            .with_context(|| format!("read {name}"))?;
        let doc: Value =
            serde_json::from_slice(&bytes).with_context(|| format!("parse {name}"))?;
        let obj = doc
            .as_object()
            .with_context(|| format!("{name}: the model must be an object"))?;
        for key in obj.keys() {
            anyhow::ensure!(
                MODEL_KEYS.contains(&key.as_str()),
                "{name}: unexpected model key \"{key}\" (extend the parser first)"
            );
        }

        let parent = match obj.get("parent") {
            Some(v) => Some(
                normalize_parent(rel, v.as_str().with_context(|| {
                    format!("{name}: parent must be a string")
                })?)
                .with_context(|| format!("read {name}"))?,
            ),
            None => None,
        };

        let textures = match obj.get("textures") {
            Some(v) => {
                let t = v
                    .as_object()
                    .with_context(|| format!("{name}: textures must be an object"))?;
                for (k, val) in t {
                    validate_texture_value(rel, k, val)
                        .with_context(|| format!("read {name}"))?;
                }
                Some(serde_json::to_string(v).with_context(|| format!("{name}: re-serialize textures"))?)
            }
            None => None,
        };

        let display = match obj.get("display") {
            Some(v) => {
                validate_display(rel, v).with_context(|| format!("read {name}"))?;
                Some(serde_json::to_string(v).with_context(|| format!("{name}: re-serialize display"))?)
            }
            None => None,
        };

        let elements = match obj.get("elements") {
            Some(v) => {
                let e = v
                    .as_array()
                    .with_context(|| format!("{name}: elements must be an array"))?;
                let n = e.len();
                anyhow::ensure!(
                    n <= u16::MAX as usize,
                    "{name}: {} elements exceeds the u16 count",
                    n
                );
                n as u16
            }
            None => 0,
        };

        let gui_light = match obj.get("gui_light") {
            Some(v) => {
                let s = v
                    .as_str()
                    .with_context(|| format!("{name}: gui_light must be a string"))?;
                anyhow::ensure!(
                    GUI_LIGHTS.contains(&s),
                    "{name}: gui_light \"{s}\" is not one of {GUI_LIGHTS:?}"
                );
                Some(s.to_string())
            }
            None => None,
        };

        let ambientocclusion = match obj.get("ambientocclusion") {
            Some(v) => Some(
                v.as_bool()
                    .with_context(|| format!("{name}: ambientocclusion must be a bool"))?,
            ),
            None => None,
        };

        let texture_size = match obj.get("texture_size") {
            Some(v) => {
                let a = v
                    .as_array()
                    .with_context(|| format!("{name}: texture_size must be an array"))?;
                anyhow::ensure!(
                    a.len() == 2,
                    "{name}: texture_size must be exactly [width, height]"
                );
                let dim = |x: &Value| -> Result<u16> {
                    let n = x
                        .as_u64()
                        .with_context(|| format!("{name}: texture_size must be u16s"))?;
                    anyhow::ensure!(n > 0 && n <= u16::MAX as u64, "{name}: texture_size dim {n} out of range");
                    Ok(n as u16)
                };
                Some((dim(&a[0])?, dim(&a[1])?))
            }
            None => None,
        };

        let groups = match obj.get("groups") {
            Some(v) => {
                anyhow::ensure!(
                    v.is_array(),
                    "{name}: groups must be an array (Blockbench metadata)"
                );
                Some(serde_json::to_string(v).with_context(|| format!("{name}: re-serialize groups"))?)
            }
            None => None,
        };

        models.insert(rel.to_string(), ModelRow {
            parent,
            textures,
            display,
            elements,
            gui_light,
            ambientocclusion,
            texture_size,
            groups,
        });
    }

    // Chain validation: every parent resolves to a file or a builtin, and
    // the graph is acyclic (the census says depth <= 4; walk with a set).
    for id in models.keys() {
        let mut cur = id.clone();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        loop {
            let row = &models[&cur];
            let p = match &row.parent {
                Some(p) => p.clone(),
                None => break,
            };
            if p.starts_with(BUILTIN_PREFIX) {
                break;
            }
            anyhow::ensure!(
                models.contains_key(&p),
                "{id}: dangling parent \"{p}\" (the model file is missing from the jar)"
            );
            anyhow::ensure!(seen.insert(cur.clone()), "{id}: parent cycle at \"{cur}\"");
            cur = p;
        }
    }

    let element_models: Vec<String> = models
        .iter()
        .filter(|(_, r)| r.elements > 0)
        .map(|(id, _)| id.clone())
        .collect();
    anyhow::ensure!(
        blocks + items == models.len() as u32,
        "accounting mismatch: {}/{} models classified",
        blocks + items,
        models.len()
    );
    Ok(ModelsData { models, element_models, blocks, items })
}

/// Emit `generated/models.rs`: the full table and the element skip list.
pub fn emit(data: &ModelsData) -> Result<String> {
    let mut out = String::with_capacity(1024 << 10);
    out.push_str("//! GENERATED by mc2rs-gen from assets/minecraft/models/ of the official jar.\n");
    out.push_str("//! The block/item model files: parent chains validated to terminate\n");
    out.push_str("//! (file or builtin/*, acyclic), textures + display stanzas retained as\n");
    out.push_str("//! value-faithful canonical compact JSON, gui_light/ambientocclusion\n");
    out.push_str("//! typed, and the elements geometry retained in the skip list. The\n");
    out.push_str("//! held-item consumer (display transforms -> hand pose) comes later.\n\n");

    out.push_str("/// (id, parent, textures JSON, display JSON, element count, gui_light,\n");
    out.push_str("/// ambientocclusion, texture_size, groups JSON) per model; parent/JSON\n");
    out.push_str("/// columns are namespace-stripped bare paths; builtin/* parents are\n");
    out.push_str("/// vanilla built-ins, not files; groups is Blockbench editor metadata.\n");
    out.push_str("pub static MODELS: [(&str, Option<&str>, Option<&str>, Option<&str>, u16, Option<&str>, Option<bool>, Option<(u16, u16)>, Option<&str>); ");
    out.push_str(&data.models.len().to_string());
    out.push_str("] = [\n");
    for (id, r) in &data.models {
        let opt = |s: &Option<String>| match s {
            Some(v) => format!("Some({v:?})"),
            None => "None".to_string(),
        };
        out.push_str(&format!(
            "    ({id:?}, {}, {}, {}, {}u16, {}, {}, {}, {}),\n",
            opt(&r.parent),
            opt(&r.textures),
            opt(&r.display),
            r.elements,
            opt(&r.gui_light),
            match r.ambientocclusion {
                Some(true) => "Some(true)".to_string(),
                Some(false) => "Some(false)".to_string(),
                None => "None".to_string(),
            },
            match r.texture_size {
                Some((w, h)) => format!("Some(({w}, {h}))"),
                None => "None".to_string(),
            },
            opt(&r.groups),
        ));
    }
    out.push_str("];\n\n");

    out.push_str("/// Models whose `elements` geometry this table retains but does not\n");
    out.push_str("/// parse — the honest skip record (the consumer round parses them).\n");
    out.push_str("pub static ELEMENT_MODELS: &[&str] = &[\n");
    for id in &data.element_models {
        out.push_str(&format!("    \"{id}\",\n"));
    }
    out.push_str("];\n");
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

    fn json_bytes(v: serde_json::Value) -> Vec<u8> {
        serde_json::to_vec(&v).unwrap()
    }

    #[test]
    fn parses_the_models_and_validates_the_parent_graph() {
        let zip = zip_of(&[
            // the root transform carrier (26.2's block/block shape)
            (
                "assets/minecraft/models/block/block.json",
                json_bytes(serde_json::json!({
                    "display": {
                        "gui": {"rotation": [30, 225, 0], "translation": [0, 0, 0],
                                 "scale": [0.625, 0.625, 0.625]},
                        "on_shelf": {"rotation": [0, 180, 0], "translation": [0, 0, 0],
                                      "scale": [1, 1, 1]}
                    },
                    "gui_light": "side"
                })),
            ),
            // a second display carrier (a child root, the fixture's cube)
            (
                "assets/minecraft/models/block/cube.json",
                json_bytes(serde_json::json!({
                    "display": {
                        "gui": {"rotation": [0, 0, 0], "scale": [1.0, 1.0, 1.0]}
                    }
                })),
            ),
            // bare parent + var-ref + plain texture
            (
                "assets/minecraft/models/block/cube_all.json",
                json_bytes(serde_json::json!({
                    "parent": "block/cube",
                    "textures": {"all": "block/stone", "particle": "#all"}
                })),
            ),
            // namespaced parent resolves to the same bare path
            (
                "assets/minecraft/models/block/stone.json",
                json_bytes(serde_json::json!({
                    "parent": "minecraft:block/cube_all",
                    "textures": {"all": "minecraft:block/stone"}
                })),
            ),
            // the 26.2 object-valued texture entry (the redstone shape)
            (
                "assets/minecraft/models/block/redstone_dust_dot.json",
                json_bytes(serde_json::json!({
                    "parent": "block/block",
                    "textures": {"overlay": {"sprite": "block/redstone_dust_overlay",
                                              "force_translucent": true}}
                })),
            ),
            // the texture_size outlier (the heavy_core shape)
            (
                "assets/minecraft/models/block/heavy_core.json",
                json_bytes(serde_json::json!({
                    "texture_size": [16, 16],
                    "display": {"gui": {"rotation": [0, 0, 0], "scale": [0.5, 0.5, 0.5]}},
                    "textures": {"all": "block/heavy_core"},
                    "elements": [{"from": [0, 0, 0], "to": [16, 16, 16]}]
                })),
            ),
            // the groups outlier (the Blockbench-metadata shape)
            (
                "assets/minecraft/models/block/custom_fence_side_east.json",
                json_bytes(serde_json::json!({
                    "textures": {"texture": "block/oak_planks"},
                    "elements": [{"from": [0, 0, 0], "to": [1, 1, 1]}],
                    "groups": [{"name": "east", "origin": [0, 0, 0], "color": 0,
                                 "children": [0, 1]}]
                })),
            ),
            // builtin parent terminates the chain
            (
                "assets/minecraft/models/item/generated.json",
                json_bytes(serde_json::json!({
                    "parent": "builtin/generated", "gui_light": "front",
                    "ambientocclusion": false
                })),
            ),
            // elements count + skip list
            (
                "assets/minecraft/models/item/stick.json",
                json_bytes(serde_json::json!({
                    "parent": "minecraft:item/generated",
                    "textures": {"layer0": "minecraft:item/stick"},
                    "elements": [{"from": [0, 0, 0], "to": [1, 1, 1]},
                                  {"from": [1, 1, 1], "to": [2, 2, 2]}]
                })),
            ),
        ]);
        let mut zip = zip;
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.models.len(), 9);
        assert_eq!((data.blocks, data.items), (7, 2));

        let stone = &data.models["block/stone"];
        assert_eq!(stone.parent.as_deref(), Some("block/cube_all"), "namespace stripped");
        assert_eq!(stone.textures.as_deref(), Some("{\"all\":\"minecraft:block/stone\"}"));
        assert!(stone.display.is_none());
        assert_eq!(stone.elements, 0);

        // the two fixture roots: block/block carries the gui_light + the
        // display stanza verbatim; cube is a display-only child root
        let root = &data.models["block/block"];
        assert_eq!(root.parent, None);
        assert_eq!(root.gui_light.as_deref(), Some("side"));
        let disp = root.display.as_deref().unwrap();
        assert!(disp.contains("\"gui\":{\"rotation\":[30,225,0]"), "{disp}");
        assert!(disp.contains("\"on_shelf\":{\"rotation\":[0,180,0]"), "{disp}");
        let cube = &data.models["block/cube"];
        assert_eq!(cube.parent, None, "the cube fixture is a root too");
        assert!(cube.gui_light.is_none());

        let all = &data.models["block/cube_all"];
        assert_eq!(all.parent.as_deref(), Some("block/cube"), "bare parent kept");
        assert!(all.textures.as_deref().unwrap().contains("\"particle\":\"#all\""));

        let dot = &data.models["block/redstone_dust_dot"];
        assert!(dot.textures.as_deref().unwrap()
            .contains("\"force_translucent\":true"), "object texture survives");

        let gen = &data.models["item/generated"];
        assert_eq!(gen.parent.as_deref(), Some("builtin/generated"));
        assert_eq!(gen.ambientocclusion, Some(false));

        let stick = &data.models["item/stick"];
        assert_eq!(stick.elements, 2);
        assert_eq!(data.element_models, vec!["block/custom_fence_side_east".to_string(), "block/heavy_core".to_string(), "item/stick".to_string()]);

        let heavy = &data.models["block/heavy_core"];
        assert_eq!(heavy.texture_size, Some((16, 16)), "the UV override is typed");
        assert_eq!(heavy.elements, 1);
        let fence = &data.models["block/custom_fence_side_east"];
        let groups = fence.groups.as_deref().unwrap();
        assert!(groups.contains("\"name\":\"east\""), "{groups}");
        assert!(fence.texture_size.is_none());

        let emitted = emit(&data).unwrap();
        assert!(emitted.contains("(\"block/stone\", Some(\"block/cube_all\"), \
            Some(\"{\\\"all\\\":\\\"minecraft:block/stone\\\"}\"), None, 0u16, None, None, None, None)"));
        assert!(emitted.contains("ELEMENT_MODELS: &[&str] = &["));
        assert!(emitted.contains("    \"item/stick\","));
    }

    #[test]
    fn refuses_a_stated_field_it_does_not_know() {
        let zip = zip_of(&[(
            "assets/minecraft/models/block/mystery.json",
            json_bytes(serde_json::json!({"parent": "block/block", "mystery": 1})),
        )]);
        let mut zip = zip;
        assert!(generate(&mut zip).is_err(), "unknown model key must refuse");
    }

    #[test]
    fn refuses_a_broken_parent_graph() {
        // dangling: the parent file is missing
        let zip = zip_of(&[(
            "assets/minecraft/models/block/dangling.json",
            json_bytes(serde_json::json!({"parent": "block/nope"})),
        )]);
        let mut z = zip;
        let err = format!("{:#}", generate(&mut z).unwrap_err());
        assert!(err.contains("dangling"), "{err}");

        // cycle: a -> b -> a
        let zip = zip_of(&[
            (
                "assets/minecraft/models/block/a.json",
                json_bytes(serde_json::json!({"parent": "minecraft:block/b"})),
            ),
            (
                "assets/minecraft/models/block/b.json",
                json_bytes(serde_json::json!({"parent": "block/a"})),
            ),
        ]);
        let mut z = zip;
        let err = format!("{:#}", generate(&mut z).unwrap_err());
        assert!(err.contains("cycle"), "{err}");

        // foreign namespace
        let zip = zip_of(&[(
            "assets/minecraft/models/block/foreign.json",
            json_bytes(serde_json::json!({"parent": "neoforge:block/x"})),
        )]);
        let mut z = zip;
        let err = format!("{:#}", generate(&mut z).unwrap_err());
        assert!(err.contains("neoforge"), "{err}");
    }

    #[test]
    fn refuses_an_unknown_display_context_or_texture_shape() {
        // unknown display context
        let zip = zip_of(&[(
            "assets/minecraft/models/block/bad_ctx.json",
            json_bytes(serde_json::json!({
                "display": {"shelf_left": {"rotation": [0, 0, 0]}}
            })),
        )]);
        let mut z = zip;
        let err = format!("{:#}", generate(&mut z).unwrap_err());
        assert!(err.contains("shelf_left"), "{err}");

        // unknown transform key
        let zip = zip_of(&[(
            "assets/minecraft/models/block/bad_tr.json",
            json_bytes(serde_json::json!({
                "display": {"gui": {"pivot": [1, 2, 3]}}
            })),
        )]);
        let mut z = zip;
        let err = format!("{:#}", generate(&mut z).unwrap_err());
        assert!(err.contains("pivot"), "{err}");

        // unknown object-texture key
        let zip = zip_of(&[(
            "assets/minecraft/models/block/bad_tex.json",
            json_bytes(serde_json::json!({
                "textures": {"overlay": {"sprite": "block/x", "atlas": "blocks"}}
            })),
        )]);
        let mut z = zip;
        let err = format!("{:#}", generate(&mut z).unwrap_err());
        assert!(err.contains("atlas"), "{err}");
    }

    #[test]
    fn refuses_a_path_outside_the_model_directories() {
        let zip = zip_of(&[(
            "assets/minecraft/models/entity/foo.json",
            json_bytes(serde_json::json!({})),
        )]);
        let mut zip = zip;
        let err = generate(&mut zip).unwrap_err().to_string();
        assert!(err.contains("entity"), "{err}");
    }
}

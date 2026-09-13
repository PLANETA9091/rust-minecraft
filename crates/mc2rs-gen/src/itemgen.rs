//! Item model definitions — the 1.21.4+ `assets/minecraft/items/` format —
//! the data half of the held-item arc (the biome-tags data-first precedent:
//! the table lands first, the consumer round comes later). A definition tree
//! rooted at a plain `minecraft:model` node resolves to its model path plus
//! optional tints (retained as canonical compact JSON, value-faithful) and
//! the optional `swap_animation_scale`; every other root shape (select /
//! special / composite / condition / range_dispatch) is RETAINED as an
//! honest skip list, never silently dropped.

use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::BTreeMap;

/// The jar directory the item model definitions live at.
pub const ITEMS_DIR: &str = "assets/minecraft/items/";

/// The model-node type a plain definition resolves through.
pub const MODEL_TYPE: &str = "minecraft:model";

/// One plain `minecraft:model` definition.
pub struct ItemModelDef {
    /// The item id (the file stem), e.g. "stone".
    pub id: String,
    /// The resolved model path, e.g. "minecraft:block/stone".
    pub model: String,
    /// The `tints` entries as canonical compact JSON (serde key order,
    /// values verbatim — empty when the model carries none).
    pub tints: Vec<String>,
    /// The optional swap-animation speed scale (the 26.2 spears ship 1.95).
    pub swap_animation_scale: Option<f32>,
}

/// The parsed table the generated file carries.
pub struct ItemModelsData {
    /// Plain definitions, sorted by id.
    pub defs: BTreeMap<String, ItemModelDef>,
    /// Items whose tree is not a plain model ref (selectors, specials,
    /// composites) — sorted, retained so nothing is silently dropped.
    pub complex: Vec<String>,
    /// The root-node type census (count descending), the honest-skip record.
    pub root_types: Vec<(String, u32)>,
}

/// One accepted root shape: `{"model": <node>, "swap_animation_scale"?: f64}`.
const ROOT_KEYS: [&str; 2] = ["model", "swap_animation_scale"];
/// One accepted `minecraft:model` node: `{type, model, tints?}`.
const MODEL_KEYS: [&str; 3] = ["type", "model", "tints"];

/// Parse every item model definition out of the jar.
pub fn generate<R: std::io::Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
) -> Result<ItemModelsData> {
    let names: Vec<String> = archive
        .file_names()
        .filter(|n| n.starts_with(ITEMS_DIR) && n.ends_with(".json"))
        .map(str::to_string)
        .collect();
    anyhow::ensure!(
        !names.is_empty(),
        "the jar ships no {ITEMS_DIR} definitions"
    );

    let mut defs = BTreeMap::new();
    let mut complex: Vec<String> = Vec::new();
    let mut types: BTreeMap<String, u32> = BTreeMap::new();

    for name in &names {
        let id = name[ITEMS_DIR.len()..].trim_end_matches(".json").to_string();
        anyhow::ensure!(!id.is_empty(), "empty item id from {name}");
        let bytes = crate::extract::read_entry(archive, name)
            .with_context(|| format!("read {name}"))?;
        let doc: Value = serde_json::from_slice(&bytes)
            .with_context(|| format!("parse {name}"))?;
        let obj = doc
            .as_object()
            .with_context(|| format!("{name}: the definition must be an object"))?;
        for key in obj.keys() {
            anyhow::ensure!(
                ROOT_KEYS.contains(&key.as_str()),
                "{name}: unexpected root key \"{key}\" (extend the parser first)"
            );
        }
        let node = obj
            .get("model")
            .with_context(|| format!("{name}: the \"model\" node is required"))?;
        let node = node
            .as_object()
            .with_context(|| format!("{name}: \"model\" must be an object"))?;
        let ty = node
            .get("type")
            .and_then(Value::as_str)
            .with_context(|| format!("{name}: \"model.type\" must be a string"))?;
        *types.entry(ty.to_string()).or_default() += 1;
        if ty != MODEL_TYPE {
            complex.push(id);
            continue;
        }
        for key in node.keys() {
            anyhow::ensure!(
                MODEL_KEYS.contains(&key.as_str()),
                "{name}: unexpected model-node key \"{key}\" (extend the parser first)"
            );
        }
        let model = node
            .get("model")
            .and_then(Value::as_str)
            .with_context(|| format!("{name}: \"model.model\" must be a string"))?;
        anyhow::ensure!(
            model.starts_with("minecraft:"),
            "{name}: model path \"{model}\" must be namespaced minecraft:"
        );
        let mut tints = Vec::new();
        if let Some(list) = node.get("tints") {
            let list = list
                .as_array()
                .with_context(|| format!("{name}: \"tints\" must be an array"))?;
            for tint in list {
                let canon = serde_json::to_string(tint)
                    .with_context(|| format!("{name}: re-serialize a tint"))?;
                tints.push(canon);
            }
        }
        let swap = match obj.get("swap_animation_scale") {
            Some(v) => Some(
                v.as_f64()
                    .with_context(|| format!("{name}: swap_animation_scale must be a number"))?
                    as f32,
            ),
            None => None,
        };
        defs.insert(
            id.clone(),
            ItemModelDef { id, model: model.to_string(), tints, swap_animation_scale: swap },
        );
    }

    complex.sort();
    let mut root_types: Vec<(String, u32)> = types.into_iter().collect();
    root_types.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let total = defs.len() + complex.len();
    anyhow::ensure!(
        total == names.len(),
        "accounting mismatch: {}/{} definitions classified",
        total,
        names.len()
    );
    Ok(ItemModelsData { defs, complex, root_types })
}

/// Emit `generated/item_models.rs`: the resolved table, the skip list and
/// the root-type census.
pub fn emit(data: &ItemModelsData) -> Result<String> {
    let mut out = String::with_capacity(256 << 10);
    out.push_str("//! GENERATED by mc2rs-gen from assets/minecraft/items/ of the official jar.\n");
    out.push_str("//! The item model definitions (the 1.21.4+ format): plain minecraft:model\n");
    out.push_str("//! roots resolved to (model path, tints as canonical compact JSON, optional\n");
    out.push_str("//! swap_animation_scale); the selector/special trees are retained in the\n");
    out.push_str("//! skip list, never silently dropped. The held-item consumer comes later\n");
    out.push_str("//! (the biome-tags data-first precedent).\n\n");

    out.push_str("/// (item id, model path, tints, swap_animation_scale) per plain definition.\n");
    out.push_str("pub static ITEM_MODEL_DEFS: [(&str, &str, &[&str], Option<f32>); ");
    out.push_str(&data.defs.len().to_string());
    out.push_str("] = [\n");
    for d in data.defs.values() {
        let tints = match d.tints.as_slice() {
            [] => "&[]".to_string(),
            [one] => format!("&[{one:?}]"),
            many => {
                let inner: Vec<String> = many.iter().map(|t| format!("{t:?}")).collect();
                format!("&[{}]", inner.join(", "))
            }
        };
        let swap = match d.swap_animation_scale {
            Some(v) => format!("Some({v:?})"),
            None => "None".to_string(),
        };
        out.push_str(&format!(
            "    (\"{}\", \"{}\", {}, {}),\n",
            d.id, d.model, tints, swap
        ));
    }
    out.push_str("];\n\n");

    out.push_str("/// Items whose definition tree is not a plain minecraft:model ref\n");
    out.push_str("/// (select / special / composite / condition / range_dispatch roots).\n");
    out.push_str("pub static COMPLEX_ITEM_MODELS: &[&str] = &[\n");
    for id in &data.complex {
        out.push_str(&format!("    \"{id}\",\n"));
    }
    out.push_str("];\n\n");

    out.push_str("/// The root-node type census (count descending) — the honest-skip record.\n");
    out.push_str("pub static ITEM_MODEL_ROOT_TYPES: [(&str, u32); ");
    out.push_str(&data.root_types.len().to_string());
    out.push_str("] = [\n");
    for (ty, n) in &data.root_types {
        out.push_str(&format!("    (\"{ty}\", {n}),\n"));
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
    fn parses_the_item_definitions_and_resolves_the_plain_refs() {
        let zip = zip_of(&[
            // a plain block-item ref
            (
                "assets/minecraft/items/stone.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:model", "model": "minecraft:block/stone"}
                })),
            ),
            // plain + a grass tint + the swap scale (the spear shape)
            (
                "assets/minecraft/items/grass_block.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:model", "model": "minecraft:block/grass_block",
                              "tints": [{"type": "minecraft:grass", "downfall": 1.0, "temperature": 0.5}]},
                    "swap_animation_scale": 1.95
                })),
            ),
            // a selector tree -> complex
            (
                "assets/minecraft/items/bow.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:select", "property": "minecraft:using_item",
                              "cases": [], "fallback": {"type": "minecraft:model", "model": "minecraft:item/bow"}}
                })),
            ),
            // a models/item/ json is NOT an item definition (strict prefix)
            (
                "assets/minecraft/models/item/imposter.json",
                b"{}".to_vec(),
            ),
        ]);
        let mut zip = zip;
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.defs.len(), 2, "the two plain defs resolve");
        assert_eq!(data.complex, vec!["bow".to_string()], "the selector root skips");
        let stone = &data.defs["stone"];
        assert_eq!(stone.model, "minecraft:block/stone");
        assert!(stone.tints.is_empty());
        assert!(stone.swap_animation_scale.is_none());
        let grass = &data.defs["grass_block"];
        assert_eq!(grass.model, "minecraft:block/grass_block");
        assert_eq!(grass.tints.len(), 1);
        // the tint survives as value-faithful canonical JSON
        let tint: Value = serde_json::from_str(&grass.tints[0]).unwrap();
        assert_eq!(tint["type"], "minecraft:grass");
        assert_eq!(tint["temperature"], 0.5);
        assert_eq!(tint["downfall"], 1.0);
        assert_eq!(grass.swap_animation_scale, Some(1.95));
        // the census counts every root node exactly once (count desc)
        assert_eq!(data.root_types, vec![
            ("minecraft:model".to_string(), 2),
            ("minecraft:select".to_string(), 1),
        ]);

        let emitted = emit(&data).unwrap();
        assert!(emitted.contains("(\"stone\", \"minecraft:block/stone\", &[], None)"));
        assert!(emitted.contains("Some(1.95)"));
        assert!(emitted.contains("COMPLEX_ITEM_MODELS: &[&str] = &["));
        assert!(emitted.contains("    \"bow\","));
    }

    #[test]
    fn refuses_a_stated_field_it_does_not_know() {
        // the extend-parser-first discipline: a stated key the parser has no
        // fields for is DROPPED data unless parsed — so it refuses.
        let zip = zip_of(&[(
            "assets/minecraft/items/mystery.json",
            json_bytes(serde_json::json!({
                "model": {"type": "minecraft:model", "model": "minecraft:block/stone"},
                "mystery_key": 1
            })),
        )]);
        let mut zip = zip;
        assert!(generate(&mut zip).is_err(), "unknown root key must refuse");
    }

    #[test]
    fn refuses_an_unnamespaced_model_path() {
        let zip = zip_of(&[(
            "assets/minecraft/items/bare.json",
            json_bytes(serde_json::json!({
                "model": {"type": "minecraft:model", "model": "block/stone"}
            })),
        )]);
        let mut zip = zip;
        assert!(generate(&mut zip).is_err(), "the model path must be namespaced");
    }
}

//! Blockstate definitions — `assets/minecraft/blockstates/` — the world-
//! render chain's data link (the models round's sibling table): every block
//! id maps to its variant or multipart definition, retained as value-
//! faithful canonical compact JSON after a strict extend-parser-first
//! validation. The census ran before any emitter code (two python passes):
//! 1198 files split EXACTLY variants (1096) or multipart (102) — never both;
//! 8661 applies over the closed 5-key set {model, x, y, uvlock, weight};
//! rotations only 0/90/180/270 (0 never written); uvlock only ever true;
//! weight on 2 applies (value 2); multipart parts carry {apply} (46
//! unconditional) or {apply, when} (1086); when is a plain AND map or the
//! {"OR": [...]} / {"AND": [...]} wrappers — never mixed, never nested in
//! 26.2; 4 vanilla refs ship WITHOUT a
//! namespace (item_frame / glow_item_frame) beside 8657 `minecraft:` ones;
//! all 2380 distinct refs resolve to model files (0 dangling — the first
//! census pass undercounted them 2363 via a python loop-variable leak; the
//! independent re-derivation caught its own census, the third such
//! self-catch in the project's history).

use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::BTreeSet;

/// The jar directory the blockstate definitions live at.
pub const BLOCKSTATES_DIR: &str = "assets/minecraft/blockstates/";

/// The only top-level shapes — a file carries EXACTLY one (census: 1096 +
/// 102 = 1198, zero files with both, zero with neither).
const TOP_KEYS: [&str; 2] = ["variants", "multipart"];

/// The closed apply key set — the census's 11 observed key combos all sit
/// inside these 5 keys.
const APPLY_KEYS: [&str; 5] = ["model", "x", "y", "uvlock", "weight"];

/// The closed multipart-part key set (census: {apply} 46 / {apply, when}
/// 1086 — nothing else ships).
const PART_KEYS: [&str; 2] = ["apply", "when"];

/// The blockstate rotation quantum (census: every x/y is a positive
/// multiple of 90 — 0 is never written, absent means 0).
const ROTATION_QUANTUM: i64 = 90;

/// One parsed blockstate file: the definition retained value-faithfully as
/// canonical compact JSON (serde key order), exactly like the models table's
/// texture/display stanzas.
#[derive(Debug)]
pub struct BlockstateRow {
    /// The `variants` object as canonical compact JSON; None for multipart.
    pub variants: Option<String>,
    /// The `multipart` array as canonical compact JSON; None for variants.
    pub multipart: Option<String>,
}

/// The parsed table the generated file carries.
#[derive(Debug, Default)]
pub struct BlockstatesData {
    /// All blockstates, keyed by the bare id ("stone", "leaf_litter").
    pub rows: Vec<(String, BlockstateRow)>,
    /// Census accounting (the report + the registry join).
    pub variant_files: u32,
    pub multipart_files: u32,
    /// The total number of applies (variant entries + multipart parts'
    /// applies, list-flattened) — the census's 8661.
    pub applies: u32,
    /// The distinct model refs (namespace-stripped) — the census's 2380.
    pub model_refs: u32,
}

/// Normalize a model reference: strip the only allowed namespace, refuse
/// foreign ones. Bare refs RIDE (4 vanilla quirks: item_frame x2,
/// glow_item_frame x2) — the join test resolves them bare.
fn normalize_model_ref(file: &str, ctx: &str, raw: &str) -> Result<String> {
    let m = match raw.split_once(':') {
        Some(("minecraft", path)) => path,
        Some((ns, _)) => {
            anyhow::bail!("{file}: {ctx}: model namespace \"{ns}:\" is not minecraft: (extend the parser first)")
        }
        None => raw,
    };
    anyhow::ensure!(!m.is_empty(), "{file}: {ctx}: model path is empty");
    Ok(m.to_string())
}

/// Validate one apply object (variant entry or multipart apply): the closed
/// key set, a string model ref, rotations a positive multiple of 90, uvlock
/// a bool, weight a positive u16.
fn validate_apply(file: &str, ctx: &str, v: &Value) -> Result<()> {
    let a = v
        .as_object()
        .with_context(|| format!("{file}: {ctx} must be an object"))?;
    for k in a.keys() {
        anyhow::ensure!(
            APPLY_KEYS.contains(&k.as_str()),
            "{file}: {ctx}: unexpected apply key \"{k}\" (extend the parser first)"
        );
    }
    let model = a
        .get("model")
        .with_context(|| format!("{file}: {ctx}.model missing"))?
        .as_str()
        .with_context(|| format!("{file}: {ctx}.model must be a string"))?;
    normalize_model_ref(file, ctx, model)?;
    for k in ["x", "y"] {
        if let Some(v) = a.get(k) {
            let n = v
                .as_i64()
                .with_context(|| format!("{file}: {ctx}.{k} must be an integer"))?;
            anyhow::ensure!(
                n >= 0 && n <= 270 && n % ROTATION_QUANTUM == 0,
                "{file}: {ctx}.{k} = {n} is not a blockstate rotation (0/90/180/270)"
            );
        }
    }
    if let Some(v) = a.get("uvlock") {
        anyhow::ensure!(
            v.is_boolean(),
            "{file}: {ctx}.uvlock must be a bool"
        );
    }
    if let Some(v) = a.get("weight") {
        let n = v
            .as_u64()
            .with_context(|| format!("{file}: {ctx}.weight must be a positive integer"))?;
        anyhow::ensure!(
            n > 0 && n <= u16::MAX as u64,
            "{file}: {ctx}.weight {n} out of range"
        );
    }
    Ok(())
}

/// Validate a when-condition: either a plain AND map (string values, "|"
/// or-lists ride inside the string) or the {"OR": [...]} / {"AND": [...]}
/// wrappers — never mixed with plain keys (the census: 0 files mix them),
/// never nested in 26.2 (but the validation recurses, so a future wrapper
/// inside a wrapper parses rather than refuses). The census: AND x288,
/// OR x5 (fire x4, redstone_wire x1).
fn validate_when(file: &str, ctx: &str, v: &Value) -> Result<()> {
    let w = v
        .as_object()
        .with_context(|| format!("{file}: {ctx} must be an object"))?;
    for (k, val) in w {
        match k.as_str() {
            "OR" | "AND" => {
                anyhow::ensure!(
                    w.len() == 1,
                    "{file}: {ctx}: the {k} wrapper is mixed with other keys (extend the parser first)"
                );
                let list = val
                    .as_array()
                    .with_context(|| format!("{file}: {ctx}.{k} must be an array"))?;
                anyhow::ensure!(!list.is_empty(), "{file}: {ctx}.{k} must not be empty");
                for (i, inner) in list.iter().enumerate() {
                    validate_when(file, &format!("{ctx}.{k}[{i}]"), inner)
                        .with_context(|| format!("read {name_placeholder}", name_placeholder = file))?;
                }
            }
            _ => {
                anyhow::ensure!(val.is_string(), "{file}: {ctx}.{k} must be a string");
            }
        }
    }
    Ok(())
}

/// Parse every blockstate file out of the jar.
pub fn generate<R: std::io::Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
) -> Result<BlockstatesData> {
    let names: Vec<String> = archive
        .file_names()
        .filter(|n| n.starts_with(BLOCKSTATES_DIR) && n.ends_with(".json"))
        .map(str::to_string)
        .collect();
    anyhow::ensure!(
        !names.is_empty(),
        "the jar ships no {BLOCKSTATES_DIR} definitions"
    );

    let mut rows: Vec<(String, BlockstateRow)> = Vec::with_capacity(names.len());
    let mut refs: BTreeSet<String> = BTreeSet::new();
    let mut variant_files = 0u32;
    let mut multipart_files = 0u32;
    let mut applies = 0u32;

    for name in &names {
        let rel = name[BLOCKSTATES_DIR.len()..].trim_end_matches(".json");
        anyhow::ensure!(!rel.is_empty(), "empty blockstate id from {name}");
        let bytes = crate::extract::read_entry(archive, name)
            .with_context(|| format!("read {name}"))?;
        let doc: Value =
            serde_json::from_slice(&bytes).with_context(|| format!("parse {name}"))?;
        let obj = doc
            .as_object()
            .with_context(|| format!("{name}: the blockstate must be an object"))?;
        for key in obj.keys() {
            anyhow::ensure!(
                TOP_KEYS.contains(&key.as_str()),
                "{name}: unexpected blockstate key \"{key}\" (extend the parser first)"
            );
        }
        anyhow::ensure!(
            obj.contains_key("variants") ^ obj.contains_key("multipart"),
            "{name}: exactly one of variants/multipart must be present (the census: never both, never neither)"
        );

        let row = if let Some(vs) = obj.get("variants") {
            variant_files += 1;
            let map = vs
                .as_object()
                .with_context(|| format!("{name}: variants must be an object"))?;
            anyhow::ensure!(
                !map.is_empty(),
                "{name}: variants must not be empty (the census: 0 empty files)"
            );
            for (state_key, apply) in map {
                // NOTE: the EMPTY state key is a real vanilla shape — the
                // property-less blocks ship `{"": {...}}` (acacia_leaves and
                // every one-variant file; the real-jar run refused it first,
                // the census's 549 one-variant files ARE these). Ride as
                // written.
                let list: Vec<&Value> = match apply {
                    Value::Array(arr) => {
                        anyhow::ensure!(
                            !arr.is_empty(),
                            "{name}: variants.{state_key} array must not be empty"
                        );
                        arr.iter().collect()
                    }
                    other => vec![other],
                };
                for a in &list {
                    validate_apply(rel, &format!("variants.{state_key}"), a)
                        .with_context(|| format!("read {name}"))?;
                    applies += 1;
                    refs.insert(normalize_model_ref(
                        rel,
                        "variants",
                        a.get("model").and_then(Value::as_str).unwrap_or_default(),
                    )?);
                }
            }
            BlockstateRow {
                variants: Some(
                    serde_json::to_string(vs)
                        .with_context(|| format!("{name}: re-serialize variants"))?,
                ),
                multipart: None,
            }
        } else {
            multipart_files += 1;
            let parts = obj
                .get("multipart")
                .and_then(Value::as_array)
                .with_context(|| format!("{name}: multipart must be an array"))?;
            anyhow::ensure!(
                !parts.is_empty(),
                "{name}: multipart must not be empty (the census: 0 empty files)"
            );
            for (i, part) in parts.iter().enumerate() {
                let p = part
                    .as_object()
                    .with_context(|| format!("{name}: multipart[{i}] must be an object"))?;
                for k in p.keys() {
                    anyhow::ensure!(
                        PART_KEYS.contains(&k.as_str()),
                        "{name}: multipart[{i}]: unexpected part key \"{k}\" (extend the parser first)"
                    );
                }
                let apply = p
                    .get("apply")
                    .with_context(|| format!("{name}: multipart[{i}].apply missing"))?;
                let list: Vec<&Value> = match apply {
                    Value::Array(arr) => {
                        anyhow::ensure!(
                            !arr.is_empty(),
                            "{name}: multipart[{i}].apply array must not be empty"
                        );
                        arr.iter().collect()
                    }
                    other => vec![other],
                };
                if let Some(w) = p.get("when") {
                    validate_when(rel, &format!("multipart[{i}].when"), w)
                        .with_context(|| format!("read {name}"))?;
                }
                for a in &list {
                    validate_apply(rel, &format!("multipart[{i}].apply"), a)
                        .with_context(|| format!("read {name}"))?;
                    applies += 1;
                    refs.insert(normalize_model_ref(
                        rel,
                        "multipart",
                        a.get("model").and_then(Value::as_str).unwrap_or_default(),
                    )?);
                }
            }
            BlockstateRow {
                variants: None,
                multipart: Some(
                    serde_json::to_string(obj.get("multipart").unwrap())
                        .with_context(|| format!("{name}: re-serialize multipart"))?,
                ),
            }
        };
        rows.push((rel.to_string(), row));
    }

    rows.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(BlockstatesData {
        rows,
        variant_files,
        multipart_files,
        applies,
        model_refs: refs.len() as u32,
    })
}

/// Emit `generated/blockstates.rs`: the full table, value-faithful JSON
/// columns. The consumer round (state-id -> model + rotation) comes later.
pub fn emit(data: &BlockstatesData) -> Result<String> {
    let mut out = String::with_capacity(1024 << 10);
    out.push_str("//! GENERATED by mc2rs-gen from assets/minecraft/blockstates/ of the\n");
    out.push_str("//! official jar. The world-render chain's data link: every block id maps\n");
    out.push_str("//! to its variant or multipart definition, retained as value-faithful\n");
    out.push_str("//! canonical compact JSON (serde key order) after a strict census-first\n");
    out.push_str("//! validation: exactly one of variants/multipart per file, the closed\n");
    out.push_str("//! 5-key apply set {model, x, y, uvlock, weight}, rotations only\n");
    out.push_str("//! 0/90/180/270, uvlock only true, the when AND-map or {\"OR\": [...]}.\n");
    out.push_str("//! 4 vanilla refs ship without a namespace (item_frame / glow_item_frame)\n");
    out.push_str("//! and ride as written; every referenced model resolves in MODELS.\n\n");

    out.push_str("/// (block id, variants JSON, multipart JSON) per blockstate; exactly one\n");
    out.push_str("/// JSON column is Some per row. State keys are the property-combo strings\n");
    out.push_str("/// (\"facing=east,half=bottom,..\"), array values are weighted/equal-\n");
    out.push_str("/// random applies in jar order, when conditions are the AND map or the\n");
    out.push_str("/// {\"OR\": [maps]} wrapper (values may carry \"a|b\" or-lists).\n");
    out.push_str("pub static BLOCKSTATES: [(&str, Option<&str>, Option<&str>); ");
    out.push_str(&data.rows.len().to_string());
    out.push_str("] = [\n");
    for (id, r) in &data.rows {
        let opt = |s: &Option<String>| match s {
            Some(v) => format!("Some({v:?})"),
            None => "None".to_string(),
        };
        out.push_str(&format!(
            "    ({id:?}, {}, {}),\n",
            opt(&r.variants),
            opt(&r.multipart),
        ));
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
    fn parses_variants_and_multipart_with_the_census_shapes() {
        let zip = zip_of(&[
            // a variants file carrying EVERY apply shape: the object form,
            // the weighted array form, uvlock, x/y rotations, and the
            // vanilla BARE model ref (the item_frame quirk)
            (
                "assets/minecraft/blockstates/fixture_block.json",
                json_bytes(serde_json::json!({
                    "variants": {
                        "facing=east,half=bottom": {"model": "block/fixture_a", "x": 90, "y": 180, "uvlock": true},
                        "facing=north,half=top": [
                            {"model": "minecraft:block/fixture_a", "weight": 2},
                            {"model": "minecraft:block/fixture_b"}
                        ]
                    }
                })),
            ),
            // a multipart file: the AND map, the AND wrapper (the
            // acacia_shelf shape), the when-absent part, the OR wrapper,
            // and the apply LIST
            (
                "assets/minecraft/blockstates/fixture_part.json",
                json_bytes(serde_json::json!({
                    "multipart": [
                        {"apply": {"model": "minecraft:block/fixture_a"}},
                        {"when": {"facing": "north", "segment_amount": "1"},
                         "apply": {"model": "minecraft:block/fixture_b", "y": 90}},
                        {"when": {"AND": [{"facing": "north"}, {"powered": "false"}]},
                         "apply": {"model": "minecraft:block/fixture_b"}},
                        {"when": {"OR": [{"east": "side|up", "north": "side|up"},
                                          {"east": "none", "north": "none"}]},
                         "apply": [{"model": "minecraft:block/fixture_a"},
                                    {"model": "minecraft:block/fixture_b"}]}
                    ]
                })),
            ),
        ]);
        let mut zip = zip;
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.rows.len(), 2);
        assert_eq!((data.variant_files, data.multipart_files), (1, 1));
        // 2 object/array applies + 1 + 1 + 1 + 2 (the apply list flattens)
        assert_eq!(data.applies, 8);
        // distinct refs: fixture_a (bare + minecraft: BOTH normalize bare) + fixture_b
        assert_eq!(data.model_refs, 2);

        let (id, row) = &data.rows[0];
        assert_eq!(id, "fixture_block");
        let v = row.variants.as_ref().unwrap();
        // canonical compact JSON, serde (alphabetical) key order
        assert!(v.contains("\"facing=east,half=bottom\":{\"model\":\"block/fixture_a\",\"uvlock\":true,\"x\":90,\"y\":180}"), "{v}");
        assert!(v.contains("\"weight\":2"), "{v}");
        assert!(row.multipart.is_none());

        let (_, row) = &data.rows[1];
        let m = row.multipart.as_ref().unwrap();
        assert!(m.starts_with('['), "the multipart rides as an array: {m}");
        // the when-absent part first (jar order — arrays keep order)
        assert!(m.contains("{\"apply\":{\"model\":\"minecraft:block/fixture_a\"}}"), "{m}");
        assert!(m.contains("\"when\":{\"facing\":\"north\",\"segment_amount\":\"1\"}"), "{m}");
        assert!(m.contains("\"AND\":[{\"facing\":\"north\"},{\"powered\":\"false\"}]"), "{m}");
        assert!(m.contains("\"OR\":[{\"east\":\"side|up\",\"north\":\"side|up\"}"), "{m}");
        assert!(row.variants.is_none());

        let emitted = emit(&data).unwrap();
        assert!(emitted.contains("pub static BLOCKSTATES: [(&str, Option<&str>, Option<&str>); 2]"));
    }

    #[test]
    fn refuses_unknown_keys_or_a_doubled_shape() {
        for (name, doc, want) in [
            (
                "unknown top key",
                serde_json::json!({"bag_of_variants": {}}),
                "unexpected blockstate key",
            ),
            (
                "both shapes",
                serde_json::json!({"variants": {"": {"model": "minecraft:block/x"}},
                                    "multipart": [{"apply": {"model": "minecraft:block/x"}}]}),
                "exactly one of variants/multipart",
            ),
            (
                "neither shape",
                serde_json::json!({}),
                "exactly one of variants/multipart",
            ),
            (
                "unknown apply key",
                serde_json::json!({"variants": {"": {"model": "minecraft:block/x", "tint": 1}}}),
                "unexpected apply key",
            ),
            (
                "unknown part key",
                serde_json::json!({"multipart": [{"apply": {"model": "minecraft:block/x"}, "when": {}, "weight": 1}]}),
                "unexpected part key",
            ),
            (
                "empty multipart",
                serde_json::json!({"multipart": []}),
                "must not be empty",
            ),
        ] {
            let zip = zip_of(&[(
                "assets/minecraft/blockstates/fixture.json",
                json_bytes(doc),
            )]);
            let mut zip = zip;
            let err = format!("{:#}", generate(&mut zip).unwrap_err());
            assert!(err.contains(want), "{name}: expected {want} in {err}");
        }
    }

    #[test]
    fn refuses_bad_refs_rotations_or_conditions() {
        for (name, doc, want) in [
            (
                "foreign namespace",
                serde_json::json!({"variants": {"": {"model": "neoforge:block/x"}}}),
                "is not minecraft:",
            ),
            (
                "bad rotation",
                serde_json::json!({"variants": {"": {"model": "minecraft:block/x", "y": 45}}}),
                "not a blockstate rotation",
            ),
            (
                "negative rotation",
                serde_json::json!({"variants": {"": {"model": "minecraft:block/x", "x": -90}}}),
                "not a blockstate rotation",
            ),
            (
                "zero weight",
                serde_json::json!({"variants": {"": {"model": "minecraft:block/x", "weight": 0}}}),
                "out of range",
            ),
            (
                "OR mixed with AND",
                serde_json::json!({"multipart": [{"when": {"OR": [{"east": "none"}], "north": "up"},
                                                   "apply": {"model": "minecraft:block/x"}}]}),
                "mixed with other keys",
            ),
            (
                "non-string when value",
                serde_json::json!({"multipart": [{"when": {"north": 1},
                                                   "apply": {"model": "minecraft:block/x"}}]}),
                "must be a string",
            ),
            (
                "non-string OR inner value",
                serde_json::json!({"multipart": [{"when": {"OR": [{"east": ["side"]}]},
                                                   "apply": {"model": "minecraft:block/x"}}]}),
                "must be a string",
            ),
        ] {
            let zip = zip_of(&[(
                "assets/minecraft/blockstates/fixture.json",
                json_bytes(doc),
            )]);
            let mut zip = zip;
            let err = format!("{:#}", generate(&mut zip).unwrap_err());
            assert!(err.contains(want), "{name}: expected {want} in {err}");
        }
    }
}

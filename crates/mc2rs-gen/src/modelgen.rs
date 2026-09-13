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

/// The closed element key set — the COMPLETE 26.2 census (1185 elements):
/// from/to/faces 1185 each, rotation 303, shade 334, __comment 63, name 94,
/// light_emission 4; nothing else ships. `name`/`__comment` are editor
/// metadata (both always strings), retained value-faithfully.
const ELEMENT_KEYS: [&str; 8] = [
    "from",
    "to",
    "faces",
    "rotation",
    "shade",
    "light_emission",
    "name",
    "__comment",
];

/// The closed face key set — the COMPLETE 26.2 census (4238 faces):
/// texture 4238, uv 3707, cullface 1041, rotation 246, tintindex 122.
const FACE_KEYS: [&str; 5] = ["uv", "texture", "cullface", "tintindex", "rotation"];

/// The vanilla face directions in `Direction` order — both the faces-object
/// keys (census: down 671, up 706, north 779, south 752, west 665, east 665)
/// and the cullface domain (1041 faces).
const FACE_DIRS: [&str; 6] = ["down", "up", "north", "south", "west", "east"];

/// The single-axis rotation key set (census: angle+axis+origin 301, rescale
/// 114 of those) and the 26.2 euler EXTENSION key set — two elements of
/// block/template_hanging_sign_rot_3 carry `{x,y,z,origin}`, so the format
/// now ships BOTH shapes. The angle domain is likewise relaxed in 26.2: the
/// old {-45,-22.5,0,22.5,45} restriction is gone (census: -90, -67.5 and
/// 0.0001 ship) — the angle is stored as written, f32 like vanilla.
const AXIS_ROTATION_KEYS: [&str; 4] = ["angle", "axis", "origin", "rescale"];
const EULER_ROTATION_KEYS: [&str; 4] = ["x", "y", "z", "origin"];

/// Vanilla clamps element bounds into [-16, 32]; the 26.2 jar uses
/// [-8, 24], so a refusal here means a NEW shape, not a vanilla-valid one.
const ELEMENT_BOUND: f64 = 32.0;

/// The face uv-rotation quantum (census: 0x1, 90x151, 180x39, 270x55).
const FACE_ROTATION_QUANTUM: u16 = 90;

/// The element-level light_emission domain (census: 15 x4, the two
/// *_cross_emissive templates).
const MAX_LIGHT_EMISSION: u64 = 15;

pub enum Axis {
    X,
    Y,
    Z,
}

pub enum ElementRotation {
    /// The classic single-axis form.
    Axis {
        angle: f32,
        axis: Axis,
        origin: [f32; 3],
        rescale: bool,
    },
    /// The 26.2 euler extension (template_hanging_sign_rot_3 x2).
    Euler { x: f32, y: f32, z: f32, origin: [f32; 3] },
}

pub struct ElementFace {
    /// None = the vanilla default UV (the bounds projection the consumer
    /// expands); 531 faces ship without one.
    pub uv: Option<[f32; 4]>,
    /// As written: "#var" refs (4232) AND bare names (6 — heavy_core's
    /// faces reference "all" without the '#'; the runtime resolution is
    /// the consumer round's bytecode homework).
    pub texture: String,
    pub cullface: Option<usize>,
    pub tintindex: Option<u8>,
    pub rotation: Option<u16>,
}

pub struct Element {
    pub from: [f32; 3],
    pub to: [f32; 3],
    /// Explicit or the vanilla default (true).
    pub shade: bool,
    pub light_emission: Option<u8>,
    pub rotation: Option<ElementRotation>,
    /// The six vanilla faces in Direction order; absent faces stay None
    /// (census: 1..6 faces per element, never zero).
    pub faces: [Option<ElementFace>; 6],
    pub name: Option<String>,
    pub comment: Option<String>,
}

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
    /// The parsed `elements` geometry, keyed by the same bare paths (only
    /// models that carry elements; the count joins the MODELS row column).
    pub elements: BTreeMap<String, Vec<Element>>,
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
    let mut elements_map: BTreeMap<String, Vec<Element>> = BTreeMap::new();
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

        let elements: Vec<Element> = match obj.get("elements") {
            Some(v) => {
                let e = v
                    .as_array()
                    .with_context(|| format!("{name}: elements must be an array"))?;
                anyhow::ensure!(
                    e.len() <= u16::MAX as usize,
                    "{name}: {} elements exceeds the u16 count",
                    e.len()
                );
                let mut els = Vec::with_capacity(e.len());
                for (i, ev) in e.iter().enumerate() {
                    els.push(parse_element(rel, i, ev).with_context(|| format!("read {name}"))?);
                }
                els
            }
            None => Vec::new(),
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

        let element_count = elements.len() as u16;
        if !elements.is_empty() {
            elements_map.insert(rel.to_string(), elements);
        }
        models.insert(rel.to_string(), ModelRow {
            parent,
            textures,
            display,
            elements: element_count,
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
    Ok(ModelsData {
        models,
        element_models,
        elements: elements_map,
        blocks,
        items,
    })
}

/// Parse one coordinate triple (from/to/rotation origin), refusing anything
/// outside the vanilla clamp domain.
fn parse_point3(model: &str, what: &str, v: &Value) -> Result<[f32; 3]> {
    let a = v
        .as_array()
        .with_context(|| format!("{model}: {what} must be an array"))?;
    anyhow::ensure!(a.len() == 3, "{model}: {what} must be exactly 3 numbers");
    let mut out = [0.0f32; 3];
    for (i, x) in a.iter().enumerate() {
        let n = x
            .as_f64()
            .with_context(|| format!("{model}: {what} must be numbers"))?;
        anyhow::ensure!(
            n >= -ELEMENT_BOUND && n <= ELEMENT_BOUND,
            "{model}: {what} coordinate {n} outside the vanilla clamp domain [-16, 32] (extend the parser first)"
        );
        out[i] = n as f32;
    }
    Ok(out)
}

/// Parse one number-valued JSON field as f32.
fn parse_f32(model: &str, what: &str, v: &Value) -> Result<f32> {
    let n = v
        .as_f64()
        .with_context(|| format!("{model}: {what} must be a number"))?;
    anyhow::ensure!(n.is_finite(), "{model}: {what} must be finite");
    Ok(n as f32)
}

/// Parse one element rotation: the single-axis form OR the 26.2 euler
/// extension — the two key sets are disjoint, mixing refuses.
fn parse_rotation(model: &str, idx: usize, v: &Value) -> Result<ElementRotation> {
    let obj = v
        .as_object()
        .with_context(|| format!("{model}: element[{idx}].rotation must be an object"))?;
    let axis_form = obj.contains_key("angle") || obj.contains_key("axis");
    let euler_form = obj.contains_key("x") || obj.contains_key("y") || obj.contains_key("z");
    anyhow::ensure!(
        axis_form ^ euler_form,
        "{model}: element[{idx}].rotation must be the single-axis {{angle,axis,origin,rescale}} or the euler {{x,y,z,origin}} form (extend the parser first)"
    );
    let (allowed, what) = if axis_form {
        (AXIS_ROTATION_KEYS, "single-axis")
    } else {
        (EULER_ROTATION_KEYS, "euler")
    };
    for k in obj.keys() {
        anyhow::ensure!(
            allowed.contains(&k.as_str()),
            "{model}: element[{idx}].rotation: unexpected {what}-form key \"{k}\" (extend the parser first)"
        );
    }
    if axis_form {
        let angle = parse_f32(model, "rotation.angle", obj.get("angle").with_context(|| {
            format!("{model}: element[{idx}].rotation.angle missing")
        })?)?;
        let axis = match obj.get("axis").and_then(Value::as_str) {
            Some("x") => Axis::X,
            Some("y") => Axis::Y,
            Some("z") => Axis::Z,
            _ => anyhow::bail!(
                "{model}: element[{idx}].rotation.axis must be one of \"x\"|\"y\"|\"z\""
            ),
        };
        let origin =
            parse_point3(model, "rotation.origin", obj.get("origin").with_context(|| {
                format!("{model}: element[{idx}].rotation.origin missing")
            })?)?;
        let rescale = match obj.get("rescale") {
            Some(v) => v
                .as_bool()
                .with_context(|| format!("{model}: element[{idx}].rotation.rescale must be a bool"))?,
            None => false,
        };
        Ok(ElementRotation::Axis { angle, axis, origin, rescale })
    } else {
        let coord = |k: &str| -> Result<f32> {
            parse_f32(
                model,
                &format!("rotation.{k}"),
                obj.get(k)
                    .with_context(|| format!("{model}: element[{idx}].rotation.{k} missing"))?,
            )
        };
        let origin =
            parse_point3(model, "rotation.origin", obj.get("origin").with_context(|| {
                format!("{model}: element[{idx}].rotation.origin missing")
            })?)?;
        Ok(ElementRotation::Euler {
            x: coord("x")?,
            y: coord("y")?,
            z: coord("z")?,
            origin,
        })
    }
}

/// Parse one element face.
fn parse_face(model: &str, idx: usize, dir: &str, v: &Value) -> Result<ElementFace> {
    let obj = v
        .as_object()
        .with_context(|| format!("{model}: element[{idx}].{dir} face must be an object"))?;
    for k in obj.keys() {
        anyhow::ensure!(
            FACE_KEYS.contains(&k.as_str()),
            "{model}: element[{idx}].{dir}: unexpected face key \"{k}\" (extend the parser first)"
        );
    }
    let uv = match obj.get("uv") {
        Some(v) => {
            let a = v
                .as_array()
                .with_context(|| format!("{model}: element[{idx}].{dir}.uv must be an array"))?;
            anyhow::ensure!(
                a.len() == 4,
                "{model}: element[{idx}].{dir}.uv must be exactly 4 numbers"
            );
            let mut out = [0.0f32; 4];
            for (i, x) in a.iter().enumerate() {
                out[i] = parse_f32(model, &format!("element[{idx}].{dir}.uv"), x)?;
            }
            Some(out)
        }
        None => None,
    };
    let texture = obj
        .get("texture")
        .with_context(|| format!("{model}: element[{idx}].{dir}.texture missing"))?
        .as_str()
        .with_context(|| format!("{model}: element[{idx}].{dir}.texture must be a string"))?
        .to_string();
    let cullface = match obj.get("cullface") {
        Some(v) => {
            let s = v
                .as_str()
                .with_context(|| format!("{model}: element[{idx}].{dir}.cullface must be a string"))?;
            Some(FACE_DIRS
                .iter()
                .position(|d| *d == s)
                .with_context(|| {
                    format!("{model}: element[{idx}].{dir}.cullface \"{s}\" is not a vanilla face (extend the parser first)")
                })?)
        }
        None => None,
    };
    let tintindex = match obj.get("tintindex") {
        Some(v) => {
            let n = v
                .as_u64()
                .with_context(|| format!("{model}: element[{idx}].{dir}.tintindex must be a non-negative integer"))?;
            anyhow::ensure!(
                n <= u8::MAX as u64,
                "{model}: element[{idx}].{dir}.tintindex {n} exceeds u8"
            );
            Some(n as u8)
        }
        None => None,
    };
    let rotation = match obj.get("rotation") {
        Some(v) => {
            let n = v
                .as_u64()
                .with_context(|| format!("{model}: element[{idx}].{dir}.rotation must be a non-negative integer"))?;
            anyhow::ensure!(
                n <= u16::MAX as u64 && (n as u16) % FACE_ROTATION_QUANTUM == 0,
                "{model}: element[{idx}].{dir}.rotation {n} is not a multiple of {FACE_ROTATION_QUANTUM}"
            );
            Some(n as u16)
        }
        None => None,
    };
    Ok(ElementFace { uv, texture, cullface, tintindex, rotation })
}

/// Parse one element (the census-closed key set; faces required — all 1185
/// 26.2 elements carry at least one).
fn parse_element(model: &str, idx: usize, v: &Value) -> Result<Element> {
    let obj = v
        .as_object()
        .with_context(|| format!("{model}: element[{idx}] must be an object"))?;
    for k in obj.keys() {
        anyhow::ensure!(
            ELEMENT_KEYS.contains(&k.as_str()),
            "{model}: element[{idx}]: unexpected key \"{k}\" (extend the parser first)"
        );
    }
    let from = parse_point3(model, "from", obj.get("from").with_context(|| {
        format!("{model}: element[{idx}].from missing")
    })?)?;
    let to = parse_point3(model, "to", obj.get("to").with_context(|| {
        format!("{model}: element[{idx}].to missing")
    })?)?;
    let shade = match obj.get("shade") {
        Some(v) => v
            .as_bool()
            .with_context(|| format!("{model}: element[{idx}].shade must be a bool"))?,
        None => true,
    };
    let light_emission = match obj.get("light_emission") {
        Some(v) => {
            let n = v
                .as_u64()
                .with_context(|| format!("{model}: element[{idx}].light_emission must be a non-negative integer"))?;
            anyhow::ensure!(
                n <= MAX_LIGHT_EMISSION,
                "{model}: element[{idx}].light_emission {n} exceeds {MAX_LIGHT_EMISSION}"
            );
            Some(n as u8)
        }
        None => None,
    };
    let rotation = match obj.get("rotation") {
        Some(v) => Some(parse_rotation(model, idx, v)?),
        None => None,
    };
    let str_field = |k: &str| -> Result<Option<String>> {
        match obj.get(k) {
            Some(v) => Ok(Some(v
                .as_str()
                .with_context(|| format!("{model}: element[{idx}].{k} must be a string"))?
                .to_string())),
            None => Ok(None),
        }
    };
    let name = str_field("name")?;
    let comment = str_field("__comment")?;
    let faces_v = obj.get("faces").with_context(|| {
        format!("{model}: element[{idx}].faces missing (every 26.2 element carries faces)")
    })?;
    let faces_obj = faces_v
        .as_object()
        .with_context(|| format!("{model}: element[{idx}].faces must be an object"))?;
    let mut faces: [Option<ElementFace>; 6] = [None, None, None, None, None, None];
    for (dir, fv) in faces_obj {
        let slot = FACE_DIRS
            .iter()
            .position(|d| d == dir)
            .with_context(|| {
                format!("{model}: element[{idx}]: face \"{dir}\" is not a vanilla direction (extend the parser first)")
            })?;
        faces[slot] = Some(parse_face(model, idx, dir, fv)?);
    }
    Ok(Element { from, to, shade, light_emission, rotation, faces, name, comment })
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

/// Shortest-roundtrip f32 emission (vanilla's own precision — the JSON
/// source is f64, vanilla narrows to f32, so the table narrows too).
fn f(v: f32) -> String {
    format!("{v:?}")
}

fn point(p: &[f32; 3]) -> String {
    format!("[{}, {}, {}]", f(p[0]), f(p[1]), f(p[2]))
}

/// Emit `generated/elements.rs`: the typed elements geometry of every
/// element-carrying model. The quad derivation (rotation math, shade
/// normals, default-UV expansion) is the consumer round — this is data.
pub fn emit_elements(data: &ModelsData) -> Result<String> {
    let total: usize = data.elements.values().map(|v| v.len()).sum();
    let mut out = String::with_capacity(512 << 10);
    out.push_str("//! GENERATED by mc2rs-gen from assets/minecraft/models/ of the official jar.\n");
    out.push_str("//! The `elements` geometry of the element-carrying models, typed: from/to\n");
    out.push_str("//! bounds (f32 like vanilla), the single-axis rotations AND the 26.2 euler\n");
    out.push_str("//! extension (block/template_hanging_sign_rot_3 x2 — the format now ships\n");
    out.push_str("//! both shapes, and the old {-45..45} angle restriction is gone: -90,\n");
    out.push_str("//! -67.5 and 0.0001 ship), per-face uv/texture/cullface/tintindex/uv-rotation\n");
    out.push_str("//! in the vanilla down/up/north/south/west/east order, shade + light_emission\n");
    out.push_str("//! typed, editor metadata (name/__comment) retained. Texture refs stay as\n");
    out.push_str("//! written (\"#var\" refs AND heavy_core's bare \"all\"); uv: None is the\n");
    out.push_str("//! vanilla bounds projection the consumer expands. The quad derivation is\n");
    out.push_str("//! the consumer round.\n\n");

    out.push_str("/// The vanilla face directions (the faces-array slot order AND the\n");
    out.push_str("/// cullface domain).\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    out.push_str("pub enum Cull { Down, Up, North, South, West, East }\n\n");
    out.push_str("/// The rotation axis of the single-axis form.\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    out.push_str("pub enum Axis { X, Y, Z }\n\n");
    out.push_str("/// One element rotation: the classic single-axis form, or the 26.2\n");
    out.push_str("/// euler extension.\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
    out.push_str("pub enum ElementRotation {\n");
    out.push_str("    Axis { angle: f32, axis: Axis, origin: [f32; 3], rescale: bool },\n");
    out.push_str("    Euler { x: f32, y: f32, z: f32, origin: [f32; 3] },\n");
    out.push_str("}\n\n");
    out.push_str("/// One element face. `texture` is the ref as written (\"#var\" or bare).\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
    out.push_str("pub struct ElementFace {\n");
    out.push_str("    pub uv: Option<[f32; 4]>,\n");
    out.push_str("    pub texture: &'static str,\n");
    out.push_str("    pub cullface: Option<Cull>,\n");
    out.push_str("    pub tintindex: Option<u8>,\n");
    out.push_str("    pub rotation: Option<u16>,\n");
    out.push_str("}\n\n");
    out.push_str("/// One element. `faces` is the six vanilla slots in Direction order.\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
    out.push_str("pub struct Element {\n");
    out.push_str("    pub from: [f32; 3],\n");
    out.push_str("    pub to: [f32; 3],\n");
    out.push_str("    pub shade: bool,\n");
    out.push_str("    pub light_emission: Option<u8>,\n");
    out.push_str("    pub rotation: Option<ElementRotation>,\n");
    out.push_str("    pub faces: [Option<ElementFace>; 6],\n");
    out.push_str("    pub name: Option<&'static str>,\n");
    out.push_str("    pub comment: Option<&'static str>,\n");
    out.push_str("}\n\n");

    out.push_str("/// (model id, the model's elements) per element-carrying model, id-sorted;\n");
    out.push_str("/// joins the MODELS table's element-count column one-to-one.\n");
    out.push_str("pub static ELEMENTS: &[(&str, &[Element])] = &[\n");
    for (id, els) in &data.elements {
        out.push_str("    (\"");
        out.push_str(id);
        out.push_str("\", &[");
        for el in els {
            let rot = match &el.rotation {
                Some(ElementRotation::Axis { angle, axis, origin, rescale }) => format!(
                    "Some(ElementRotation::Axis {{ angle: {}, axis: Axis::{:?}, origin: {}, rescale: {} }})",
                    f(*angle), axis, point(origin), rescale
                ),
                Some(ElementRotation::Euler { x, y, z, origin }) => format!(
                    "Some(ElementRotation::Euler {{ x: {}, y: {}, z: {}, origin: {} }})",
                    f(*x), f(*y), f(*z), point(origin)
                ),
                None => "None".to_string(),
            };
            let mut face_strs = Vec::with_capacity(6);
            for slot in &el.faces {
                face_strs.push(match slot {
                    Some(fc) => {
                        let uv = match fc.uv {
                            Some([a, b, c2, d]) => format!("Some([{}, {}, {}, {}])", f(*a), f(*b), f(*c2), f(*d)),
                            None => "None".to_string(),
                        };
                        let cull = match fc.cullface {
                            Some(i) => format!(
                                "Some(Cull::{})",
                                ["Down", "Up", "North", "South", "West", "East"][i]
                            ),
                            None => "None".to_string(),
                        };
                        let tint = match fc.tintindex {
                            Some(t) => format!("Some({t})"),
                            None => "None".to_string(),
                        };
                        let fr = match fc.rotation {
                            Some(r) => format!("Some({r})"),
                            None => "None".to_string(),
                        };
                        format!(
                            "Some(ElementFace {{ uv: {}, texture: {:?}, cullface: {}, tintindex: {}, rotation: {} }})",
                            uv, fc.texture, cull, tint, fr
                        )
                    }
                    None => "None".to_string(),
                });
            }
            let name = match &el.name {
                Some(v) => format!("Some({v:?})"),
                None => "None".to_string(),
            };
            let comment = match &el.comment {
                Some(v) => format!("Some({v:?})"),
                None => "None".to_string(),
            };
            let le = match el.light_emission {
                Some(n) => format!("Some({n})"),
                None => "None".to_string(),
            };
            out.push_str("Element { from: ");
            out.push_str(&point(&el.from));
            out.push_str(", to: ");
            out.push_str(&point(&el.to));
            out.push_str(&format!(", shade: {}, light_emission: {}, rotation: {}, faces: [{}, {}, {}, {}, {}, {}], name: {}, comment: {} }},",
                el.shade, le, rot,
                face_strs[0], face_strs[1], face_strs[2],
                face_strs[3], face_strs[4], face_strs[5],
                name, comment));
        }
        out.push_str("]),\n");
    }
    out.push_str("];\n");
    let _ = total;
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

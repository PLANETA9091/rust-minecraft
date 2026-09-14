//! Item model definitions — the 1.21.4+ `assets/minecraft/items/` format —
//! the data half of the held-item arc. A definition tree rooted at a plain
//! `minecraft:model` node resolves to its model path plus optional tints
//! (retained as canonical compact JSON, value-faithful) and the optional
//! `swap_animation_scale`. Every OTHER root shape is evaluated by the
//! RESTING-HAND resolver (the first-person hand of a fresh, unused stack):
//!
//! - `minecraft:condition` -> the property's resting state is FALSE
//!   (not using, not broken, not cast, no selected bundle item, the
//!   audited components empty) -> the `on_false` branch;
//! - `minecraft:select` -> the hand context matches no case (verified
//!   against the whole jar: no case targets a hand context) -> the
//!   `fallback` branch (absent -> refuse);
//! - `minecraft:range_dispatch` -> the resting value is 0 -> the entry
//!   with the greatest threshold <= 0 (the vanilla loop's own pick),
//!   else the fallback;
//! - `minecraft:composite` -> the children in order, each carrying its
//!   optional decoded transformation (the `com.mojang.math.Transformation
//!   .compose` bytecode order: translation x left_rotation x scale x
//!   right_rotation);
//! - `minecraft:special` -> the block-entity renderers (shield, trident,
//!   chest, shulker box, banner, head, conduit, decorated pot, golem
//!   statue) have NO model JSON (every base path in the jar is missing)
//!   — refused, the renderer named;
//! - anything else -> refused, the shape named.
//!
//! A node's own `transformation` is accepted and rides its whole subtree
//! (the vanilla `LayerRenderState.applyTransform` bytecode: the display
//! stanza FIRST, then `mulPose(localTransform)` — so the local transform
//! acts on the raw bake vertices, the display pose on top). Nested
//! transformations (a transformed node under a transformed node) never
//! occur in the jar and are refused loudly until they do.

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

/// One decoded node transformation — the four components verbatim from
/// the JSON, identity defaults where absent (the vanilla codec's own
/// defaults). The runtime composes T·L·S·R per the compose bytecode.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformComponents {
    pub translation: [f32; 3],
    pub left_rotation: [f32; 4],
    pub scale: [f32; 3],
    pub right_rotation: [f32; 4],
}

impl TransformComponents {
    fn identity() -> Self {
        Self {
            translation: [0.0; 3],
            left_rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0; 3],
            right_rotation: [0.0, 0.0, 0.0, 1.0],
        }
    }

    fn is_identity(&self) -> bool {
        self.translation == [0.0; 3]
            && self.left_rotation == [0.0, 0.0, 0.0, 1.0]
            && self.scale == [1.0; 3]
            && self.right_rotation == [0.0, 0.0, 0.0, 1.0]
    }
}

/// One resolved layer of a complex definition: the model path, the
/// tints (canonical compact JSON) and the optional decoded node
/// transformation (None = identity — the overwhelmingly common case).
/// A resolved layer is EITHER a plain model layer OR the special
/// renderer payload — never both (the emit enforces the invariant).
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedLayer {
    pub model: String,
    pub tints: Vec<String>,
    pub transform: Option<TransformComponents>,
    /// The block-entity renderer payload when the resting branch hits a
    /// `minecraft:special` node — the geometry lives in the
    /// LayerDefinitions dump (generated/special_models.rs), the texture
    /// per layer rides the atlas sprite vocabulary.
    pub special: Option<SpecialPayload>,
}

/// The special renderer payload: the renderer kind, the wrapper's base
/// model path (whose display stanza poses the geometry), the per-layer
/// (LayerDefinitions id, atlas sprite texture) pairs and the banner's
/// dye name (the DyeColor.textureDiffuseColor tint — resolved by the
/// consumer from the dump's DyeColor lane).
#[derive(Debug, Clone, PartialEq)]
pub struct SpecialPayload {
    pub renderer: String,
    pub base: String,
    pub layers: Vec<(String, String)>,
    pub tint: Option<String>,
}

/// One resolved complex definition: the resting-hand layers in order
/// plus the root-level `swap_animation_scale` (the spears' 1.95).
#[derive(Debug, Clone)]
pub struct ResolvedItem {
    pub layers: Vec<ResolvedLayer>,
    pub swap_animation_scale: Option<f32>,
}

/// One item's GUI-context icon resolution — the definition tree
/// evaluated for `ItemDisplayContext.GUI` (the GuiGraphicsExtractor
/// item()'s own updateForTopItem(GUI)): the display_context select
/// cases naming "gui" win (the spears' and the trident's 2D sprites),
/// the resting-false conditions branch on_false (a fresh stack in an
/// inventory), the range_dispatch keeps its threshold<=0 entry, the
/// composites concatenate, the special nodes mark the payload.
pub struct GuiIcon {
    /// The gui-context model paths (empty for the special payloads).
    pub models: Vec<String>,
    /// true = the ITEM_SPECIAL_MODELS payload renders (the geometry
    /// the dump carries); false = `models` render as cuboid bakes.
    pub special: bool,
}

/// One charge-keyed state of the USING branch: the charge value this
/// state engages at (the vanilla range_dispatch's own compare — the
/// LAST threshold <= value wins) and the resolved layers (a special
/// payload lands as its own SpecialItemDef static instead).
#[derive(Debug, Clone)]
pub struct UsingState {
    pub threshold: f32,
    pub layers: Vec<ResolvedLayer>,
}

/// One item's USING-context resolution — the definition tree evaluated
/// with the `minecraft:using_item` condition TRUE (the right-mouse
/// hold): the gate walks to the on_true branch, the selects fall
/// through (no case targets a hand context — the component/display
/// cases are the other states' own), and a range_dispatch EXPANDS into
/// the charge-keyed states (the fallback at 0.0, then every entry at
/// its threshold). The jar's five using trees: bow, crossbow,
/// goat_horn, shield, trident.
pub struct UsingItem {
    /// The range-select property the charge reads:
    /// "minecraft:use_duration" (the elapsed ticks, times `scale` —
    /// javap UseDuration.get: getUseDuration - remainingTicks), or
    /// "minecraft:crossbow/pull" (elapsed / getChargeDuration — javap:
    /// Mth.floor(1.25f * 20f) = 25 ticks). Empty = the pure using gate
    /// (no charge lane — the shield's and the trident's swaps).
    pub property: String,
    /// The dispatch's own scale (the bow's 0.05; 1.0 when the property
    /// law needs none).
    pub scale: f32,
    /// The states, thresholds ascending (the fallback first at 0.0).
    pub states: Vec<UsingState>,
}

/// The parsed table the generated file carries.
pub struct ItemModelsData {
    /// Plain definitions, sorted by id.
    pub defs: BTreeMap<String, ItemModelDef>,
    /// Complex trees RESOLVED to resting-hand layers, sorted by id.
    pub resolved: BTreeMap<String, ResolvedItem>,
    /// Items whose tree cannot resolve (the special-renderer shapes and
    /// any refused node) — sorted, retained WITH the refusal reason so
    /// nothing is silently dropped.
    pub complex: Vec<(String, String)>,
    /// The root-node type census (count descending), the honest-skip record.
    pub root_types: Vec<(String, u32)>,
    /// The GUI-context icon overrides for every NON-plain definition
    /// (the resolved trees + the special payloads), sorted by id. The
    /// plain defs need none: their gui icon IS their model path.
    pub gui_icons: Vec<(String, GuiIcon)>,
    /// The USING-context resolutions (the trees carrying a
    /// `minecraft:using_item` gate), sorted by id.
    pub using: BTreeMap<String, UsingItem>,
}

/// The condition properties whose RESTING state is false: a fresh stack
/// in a resting first-person hand is not being used, not broken, not
/// cast, carries no selected bundle item, and the audited components
/// (`has_component` — lodestone_tracker and friends) sit empty.
const RESTING_FALSE_CONDITIONS: [&str; 5] = [
    "minecraft:using_item",
    "minecraft:broken",
    "minecraft:fishing_rod/cast",
    "minecraft:bundle/has_selected_item",
    "minecraft:has_component",
];

/// The range-dispatch properties of the USE animation (the bow's pull,
/// the crossbow's) — they sit under `on_true` of `using_item` conditions,
/// unreachable at rest; as conditions themselves they resolve through
/// the same resting-false rule.
const USE_ANIMATION_CONDITIONS: [&str; 3] = [
    "minecraft:use_duration",
    "minecraft:use_cycle",
    "minecraft:crossbow/pull",
];

// the key lists name every key the jar's nodes carry. The property-
// declaration keys (block_state_property / pattern / period / source /
// target) say WHICH attribute a select or range_dispatch reads — the
// resting-hand resolution (fallback / threshold-0) is unchanged by them;
// `transformation` is the generic node transform every node may carry.
const CONDITION_KEYS: [&str; 6] = ["type", "property", "on_true", "on_false", "component", "transformation"];
const SELECT_KEYS: [&str; 7] = ["type", "property", "cases", "fallback", "block_state_property", "pattern", "transformation"];
const RANGE_KEYS: [&str; 9] = ["type", "property", "entries", "scale", "fallback", "period", "source", "target", "transformation"];
const COMPOSITE_KEYS: [&str; 3] = ["type", "models", "transformation"];
const NODE_KEYS: [&str; 4] = ["type", "model", "tints", "transformation"];
/// The `minecraft:special` wrapper node: the base model path (whose
/// display stanza poses the renderer's geometry), the renderer node and
/// the optional node transformation.
const SPECIAL_KEYS: [&str; 4] = ["type", "base", "model", "transformation"];
/// The renderer nodes' own key sets, by renderer kind (the jar's
/// declaration keys — the census of all ten kinds).
const BANNER_RENDERER_KEYS: [&str; 2] = ["type", "color"];
const SHULKER_RENDERER_KEYS: [&str; 2] = ["type", "texture"];
const CHEST_RENDERER_KEYS: [&str; 2] = ["type", "texture"];
const HEAD_RENDERER_KEYS: [&str; 2] = ["type", "kind"];
const PLAYER_HEAD_RENDERER_KEYS: [&str; 1] = ["type"];
const CONDUIT_RENDERER_KEYS: [&str; 1] = ["type"];
const POT_RENDERER_KEYS: [&str; 1] = ["type"];
const STATUE_RENDERER_KEYS: [&str; 3] = ["type", "pose", "texture"];
const SHIELD_RENDERER_KEYS: [&str; 1] = ["type"];
const TRIDENT_RENDERER_KEYS: [&str; 1] = ["type"];
const TRANSFORM_KEYS: [&str; 4] =
    ["translation", "left_rotation", "scale", "right_rotation"];

/// The recursion ceiling — the jar's trees top out at depth 4; anything
/// deeper is a malformed future shape and refuses loudly.
const RESOLVE_MAX_DEPTH: usize = 16;

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
    let mut resolved: BTreeMap<String, ResolvedItem> = BTreeMap::new();
    let mut gui_icons: BTreeMap<String, GuiIcon> = BTreeMap::new();
    let mut using: BTreeMap<String, UsingItem> = BTreeMap::new();
    let mut complex: Vec<(String, String)> = Vec::new();
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
            // THE GUI ICON: the same tree, evaluated for the gui
            // display context (the resolver's own branch laws, the
            // display_context select's gui cases included). An
            // unresolvable branch = the empty icon — the honest
            // nothing-drawn, never a guessed model.
            let gui = resolve_gui(&Value::Object(node.clone()), &id).unwrap_or(GuiIcon {
                models: Vec::new(),
                special: false,
            });
            gui_icons.insert(id.clone(), gui);
            // THE USING STATE: the same tree evaluated with the
            // using_item gate TRUE — only the trees that actually carry
            // the gate (the property census) resolve one; everything
            // else has no using lane. A refusal here is a RENDER GAP on
            // a live interaction (the vanilla client renders the branch
            // we cannot) — the loud bail, never a silent skip.
            if tree_has_using_gate(&Value::Object(node.clone())) {
                match resolve_using_root(&Value::Object(node.clone()), &id, 0, &None, false)
                    .map_err(|e| anyhow::anyhow!("{e}"))?
                {
                    Some(u) => {
                        using.insert(id.clone(), u);
                    }
                    None => anyhow::bail!(
                        "{name}: carries a using gate but no using lane resolved from the walk"
                    ),
                }
            }
            match resolve_node(&Value::Object(node.clone()), &id, 0, &None) {
                Ok(layers) => {
                    let swap = match obj.get("swap_animation_scale") {
                        Some(v) => Some(
                            v.as_f64().with_context(|| format!(
                                "{name}: swap_animation_scale must be a number"
                            ))? as f32,
                        ),
                        None => None,
                    };
                    resolved.insert(id, ResolvedItem { layers, swap_animation_scale: swap });
                }
                Err(why) => complex.push((id, why)),
            }
            // THE GUI ICON: the same tree, evaluated for the gui
            // display context (the resolver's own branch laws, the
            // display_context select's gui cases included). An
            // unresolvable branch = the empty icon — the honest
            // nothing-drawn, never a guessed model.
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
    let mut gui_icons: Vec<(String, GuiIcon)> = gui_icons.into_iter().collect();
    gui_icons.sort_by(|a, b| a.0.cmp(&b.0));
    let mut root_types: Vec<(String, u32)> = types.into_iter().collect();
    root_types.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let total = defs.len() + resolved.len() + complex.len();
    anyhow::ensure!(
        total == names.len(),
        "accounting mismatch: {}/{} definitions classified",
        total,
        names.len()
    );
    Ok(ItemModelsData { defs, resolved, complex, root_types, gui_icons, using })
}

/// The GUI-context resolution — the branch laws the gui display
/// context speaks. `Ok` always (the unresolvable branch = the empty
/// icon); the empty `models` + `special: false` = nothing drawn.
fn resolve_gui(node: &Value, id: &str) -> Result<GuiIcon, String> {
    let obj = node
        .as_object()
        .ok_or_else(|| format!("{id}: the node must be an object"))?;
    let ty = obj
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{id}: the node \"type\" must be a string"))?;
    match ty {
        "minecraft:model" => {
            let model = obj
                .get("model")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("{id}: \"model.model\" must be a string"))?;
            Ok(GuiIcon { models: vec![model.to_string()], special: false })
        }
        "minecraft:special" => Ok(GuiIcon { models: Vec::new(), special: true }),
        "minecraft:select" => {
            // the case matches the gui context iff the select's property
            // is display_context AND the case's "when" names "gui"
            let prop = obj.get("property").and_then(Value::as_str).unwrap_or("");
            if prop == "minecraft:display_context" {
                if let Some(cases) = obj.get("cases").and_then(Value::as_array) {
                    for case in cases {
                        let Some(c) = case.as_object() else { continue };
                        let matched = match c.get("when") {
                            Some(Value::String(s)) => s == "gui",
                            Some(Value::Array(a)) => {
                                a.iter().any(|v| v.as_str() == Some("gui"))
                            }
                            _ => false,
                        };
                        if matched {
                            if let Some(m) = c.get("model") {
                                return resolve_gui(m, id);
                            }
                        }
                    }
                }
            }
            match obj.get("fallback") {
                Some(f) => resolve_gui(f, id),
                None => Ok(GuiIcon { models: Vec::new(), special: false }),
            }
        }
        "minecraft:condition" => {
            // the resting-false conditions are false for a fresh stack
            // in an inventory — the on_false branch (the same five
            // properties the hand resolver speaks)
            match obj.get("on_false") {
                Some(f) => resolve_gui(f, id),
                None => Ok(GuiIcon { models: Vec::new(), special: false }),
            }
        }
        "minecraft:range_dispatch" => {
            // the resting value 0: the greatest threshold <= 0 entry
            // (the vanilla loop's LAST-wins), else the fallback
            let best = obj
                .get("entries")
                .and_then(Value::as_array)
                .ok_or_else(|| format!("{id}: \"entries\" must be an array"))?
                .iter()
                .filter_map(|e| {
                    let e = e.as_object()?;
                    let th = e.get("threshold")?.as_f64()?;
                    let model = e.get("model")?;
                    if th <= 0.0 { Some((th, model)) } else { None }
                })
                .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            match best {
                Some((_, model)) => resolve_gui(model, id),
                None => match obj.get("fallback") {
                    Some(f) => resolve_gui(f, id),
                    None => Ok(GuiIcon { models: Vec::new(), special: false }),
                },
            }
        }
        "minecraft:composite" => {
            let models_list = obj
                .get("models")
                .and_then(Value::as_array)
                .ok_or_else(|| format!("{id}: \"models\" must be an array"))?;
            let mut models = Vec::new();
            for m in models_list {
                let g = resolve_gui(m, id)?;
                if g.special {
                    return Ok(GuiIcon { models: Vec::new(), special: true });
                }
                models.extend(g.models);
            }
            Ok(GuiIcon { models, special: false })
        }
        other => Err(format!("{id}: gui resolution of {other}")),
    }
}

/// The USING-GATE census: whether the tree carries a
/// `minecraft:using_item` condition anywhere (the pre-filter — only
/// those trees attempt the using walk).
fn tree_has_using_gate(node: &Value) -> bool {
    const GATE: &str = "minecraft:using_item";
    if let Some(obj) = node.as_object() {
        if obj.get("property").and_then(Value::as_str) == Some(GATE) {
            return true;
        }
        return obj.values().any(tree_has_using_gate);
    }
    node.as_array().is_some_and(|list| list.iter().any(tree_has_using_gate))
}

/// The USING-context tree walk — the definition tree evaluated with the
/// `minecraft:using_item` condition TRUE (the right-mouse hold's own
/// branch). `gated` tracks whether the gate has been crossed; `outer`
/// carries the ancestor transformation chain (the shield's and the
/// trident's mirror transforms ride the CONDITION node — a sibling key
/// of property/on_true — so the gate itself decodes it).
///
/// `Ok(None)` = this path holds no using lane. `Err` = a shape the
/// using lane reaches but the resolver cannot honor — the loud refuse
/// (the caller bails the generator; the round extends first, never
/// renders a guessed payload).
fn resolve_using_root(
    node: &Value,
    id: &str,
    depth: usize,
    outer: &Option<TransformComponents>,
    gated: bool,
) -> Result<Option<UsingItem>, String> {
    const GATE: &str = "minecraft:using_item";
    if depth > RESOLVE_MAX_DEPTH {
        return Err(format!("{id}: the tree is deeper than {RESOLVE_MAX_DEPTH}"));
    }
    let obj = node
        .as_object()
        .ok_or_else(|| format!("{id}: the node must be an object"))?;
    let ty = obj
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{id}: the node \"type\" must be a string"))?;

    // the node's own transformation rides its whole subtree (the same
    // applyTransform law the resting walk speaks)
    let my_transform = match obj.get("transformation") {
        Some(t) => {
            if outer.is_some() {
                return Err(format!(
                    "{id}: a transformed node under a transformed node — compose first"
                ));
            }
            Some(decode_transform(t).map_err(|e| format!("{id}: {e}"))?)
        }
        None => None,
    };
    let active: Option<TransformComponents> =
        my_transform.clone().or_else(|| outer.clone());

    match ty {
        "minecraft:condition" => {
            check_keys(obj, &CONDITION_KEYS, id, ty)?;
            let prop = obj
                .get("property")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("{id}: condition \"property\" must be a string"))?;
            if prop == GATE {
                if gated {
                    return Err(format!("{id}: a second using gate on the using path"));
                }
                // THE GATE: the using walk lives on the on_true branch
                resolve_using_root(
                    obj.get("on_true")
                        .ok_or_else(|| format!("{id}: the using gate without on_true"))?,
                    id,
                    depth + 1,
                    &active,
                    true,
                )
            } else {
                Err(format!(
                    "{id}: condition {prop} on the using path — no using rule (extend first)"
                ))
            }
        }
        "minecraft:select" => {
            check_keys(obj, &SELECT_KEYS, id, ty)?;
            // no case targets a hand context (the jar-wide sweep — the
            // component/display cases are the other states' own): the
            // using walk falls through exactly like the resting walk
            match obj.get("fallback") {
                Some(f) => resolve_using_root(f, id, depth + 1, &active, gated),
                None => Err(format!("{id}: select without fallback on the using path")),
            }
        }
        "minecraft:range_dispatch" => {
            check_keys(obj, &RANGE_KEYS, id, ty)?;
            if !gated {
                return Err(format!(
                    "{id}: a dispatch before the using gate — no using rule"
                ));
            }
            // THE CHARGE LANE: expand into the charge-keyed states —
            // the fallback at 0.0 (the pull's bottom lane), then every
            // entry at its threshold (the vanilla loop's LAST-wins)
            let prop = obj
                .get("property")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("{id}: range_dispatch \"property\" must be a string"))?;
            let scale = obj.get("scale").and_then(Value::as_f64).unwrap_or(1.0) as f32;
            let mut states = Vec::new();
            match obj.get("fallback") {
                Some(f) => states.push(UsingState {
                    threshold: 0.0,
                    layers: resolve_node(f, id, depth + 1, &active)?,
                }),
                None => {
                    return Err(format!(
                        "{id}: the charge dispatch without a fallback — the bottom lane"
                    ))
                }
            }
            for e in obj
                .get("entries")
                .and_then(Value::as_array)
                .ok_or_else(|| format!("{id}: range_dispatch \"entries\" must be an array"))?
            {
                let e = e
                    .as_object()
                    .ok_or_else(|| format!("{id}: the dispatch entry must be an object"))?;
                let th = e
                    .get("threshold")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| format!("{id}: the dispatch entry \"threshold\" must be a number"))?
                    as f32;
                let model = e
                    .get("model")
                    .ok_or_else(|| format!("{id}: the dispatch entry without \"model\""))?;
                states.push(UsingState {
                    threshold: th,
                    layers: resolve_node(model, id, depth + 1, &active)?,
                });
            }
            Ok(Some(UsingItem { property: prop.to_string(), scale, states }))
        }
        "minecraft:model" | "minecraft:special" => {
            // the gate's plain payload: the single-state swap (the goat
            // horn's tooting model, the shield's blocking special) — the
            // resting resolver resolves the subtree (the same node laws)
            let layers = resolve_node(node, id, depth, outer)?;
            Ok(Some(UsingItem {
                property: String::new(),
                scale: 1.0,
                states: vec![UsingState { threshold: 0.0, layers }],
            }))
        }
        "minecraft:composite" => Err(format!(
            "{id}: a composite on the using path — extend the resolver first"
        )),
        other => Err(format!("{id}: unknown node type {other} on the using path")),
    }
}

/// The resting-hand tree resolver. `outer` carries a parent node's
/// transformation (a transformed node under a transformed node never
/// occurs in the jar and refuses loudly — compose when the data asks).
fn resolve_node(
    node: &Value,
    id: &str,
    depth: usize,
    outer: &Option<TransformComponents>,
) -> Result<Vec<ResolvedLayer>, String> {
    if depth > RESOLVE_MAX_DEPTH {
        return Err(format!("{id}: the tree is deeper than {RESOLVE_MAX_DEPTH}"));
    }
    let obj = node
        .as_object()
        .ok_or_else(|| format!("{id}: the node must be an object"))?;
    let ty = obj
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{id}: the node \"type\" must be a string"))?;

    // the node's own transformation rides its whole subtree (the vanilla
    // applyTransform bytecode: the local transform on the raw vertices,
    // the display stanza on top)
    let my_transform = match obj.get("transformation") {
        Some(t) => {
            if outer.is_some() {
                return Err(format!(
                    "{id}: a transformed node under a transformed node — compose first"
                ));
            }
            Some(decode_transform(t).map_err(|e| format!("{id}: {e}"))?)
        }
        None => None,
    };
    let active: Option<TransformComponents> =
        my_transform.clone().or_else(|| outer.clone());

    match ty {
        "minecraft:model" => {
            check_keys(obj, &NODE_KEYS, id, ty)?;
            let model = obj
                .get("model")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("{id}: \"model.model\" must be a string"))?;
            model.starts_with("minecraft:")
                .then_some(())
                .ok_or_else(|| format!("{id}: model path \"{model}\" must be namespaced minecraft:"))?;
            let mut tints = Vec::new();
            if let Some(list) = obj.get("tints") {
                let list = list
                    .as_array()
                    .ok_or_else(|| format!("{id}: \"tints\" must be an array"))?;
                for tint in list {
                    let canon = serde_json::to_string(tint)
                        .map_err(|e| format!("{id}: re-serialize a tint: {e}"))?;
                    tints.push(canon);
                }
            }
            let transform = my_transform.filter(|t| !t.is_identity());
            Ok(vec![ResolvedLayer { model: model.to_string(), tints, transform, special: None }])
        }
        "minecraft:condition" => {
            check_keys(obj, &CONDITION_KEYS, id, ty)?;
            let prop = obj
                .get("property")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("{id}: condition \"property\" must be a string"))?;
            if RESTING_FALSE_CONDITIONS.contains(&prop)
                || USE_ANIMATION_CONDITIONS.contains(&prop)
            {
                let on_false = obj
                    .get("on_false")
                    .ok_or_else(|| format!("{id}: resting-false condition without on_false"))?;
                resolve_node(on_false, id, depth + 1, &active)
            } else {
                Err(format!("{id}: condition property {prop} — no resting rule"))
            }
        }
        "minecraft:select" => {
            check_keys(obj, &SELECT_KEYS, id, ty)?;
            // the hand context matches no case (the jar-wide sweep: no
            // case targets a hand context) — vanilla's own fallthrough
            match obj.get("fallback") {
                Some(f) => resolve_node(f, id, depth + 1, &active),
                None => Err(format!(
                    "{id}: select without fallback — the hand context would render nothing"
                )),
            }
        }
        "minecraft:range_dispatch" => {
            check_keys(obj, &RANGE_KEYS, id, ty)?;
            // the resting value 0: the entry with the greatest threshold
            // <= 0 (the vanilla loop keeps the LAST such entry), else the
            // fallback
            let best = obj
                .get("entries")
                .and_then(Value::as_array)
                .ok_or_else(|| format!("{id}: range_dispatch \"entries\" must be an array"))?
                .iter()
                .filter_map(|e| {
                    let e = e.as_object()?;
                    let th = e.get("threshold")?.as_f64()?;
                    let model = e.get("model")?;
                    if th <= 0.0 { Some((th, model)) } else { None }
                })
                .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            match best {
                Some((_, model)) => resolve_node(model, id, depth + 1, &active),
                None => match obj.get("fallback") {
                    Some(f) => resolve_node(f, id, depth + 1, &active),
                    None => Err(format!(
                        "{id}: range_dispatch with no threshold<=0 entry and no fallback"
                    )),
                },
            }
        }
        "minecraft:composite" => {
            check_keys(obj, &COMPOSITE_KEYS, id, ty)?;
            let models = obj
                .get("models")
                .and_then(Value::as_array)
                .ok_or_else(|| format!("{id}: composite \"models\" must be an array"))?;
            let mut layers = Vec::new();
            for child in models {
                layers.extend(resolve_node(child, id, depth + 1, &active)?);
            }
            // a block-entity renderer inside a composite has no jar case
            // — refuse loudly rather than half-render
            if layers.iter().any(|l| l.special.is_some()) {
                return Err(format!("{id}: a special renderer inside a composite"));
            }
            Ok(layers)
        }
        "minecraft:special" => {
            // the block-entity renderers — RESOLVED this round: the
            // geometry is the LayerDefinitions dump's (generated/
            // special_models.rs), the textures ride the Sheets/SMIN map
            // constants the renderers hold (the javap + reflection
            // evidence in DumpSpecialModels.java's lane 3).
            check_keys(obj, &SPECIAL_KEYS, id, ty)?;
            if my_transform.is_some() && outer.is_some() {
                return Err(format!(
                    "{id}: a transformed special under a transformed node — compose first"
                ));
            }
            let base = obj
                .get("base")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("{id}: special \"base\" must be a string"))?
                .to_string();
            let renderer_node = obj
                .get("model")
                .and_then(Value::as_object)
                .ok_or_else(|| format!("{id}: special \"model\" must be an object"))?;
            let renderer = renderer_node
                .get("type")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("{id}: special renderer \"type\" must be a string"))?
                .strip_prefix("minecraft:")
                .unwrap_or(renderer_node
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or("?")
                    )
                .to_string();
            let param = |key: &str| -> Option<String> {
                renderer_node.get(key).and_then(Value::as_str).map(str::to_string)
            };
            let layers: Vec<(String, String)> = match renderer.as_str() {
                "shield" => {
                    check_keys(renderer_node, &SHIELD_RENDERER_KEYS, id, &renderer)?;
                    vec![("minecraft:shield#main".into(), "entity/shield/shield_base_nopattern".into())]
                }
                "trident" => {
                    check_keys(renderer_node, &TRIDENT_RENDERER_KEYS, id, &renderer)?;
                    vec![("minecraft:trident#main".into(), "entity/trident/trident".into())]
                }
                "chest" => {
                    check_keys(renderer_node, &CHEST_RENDERER_KEYS, id, &renderer)?;
                    let t = param("texture").unwrap_or_else(|| "normal".into());
                    let t = t.strip_prefix("minecraft:").unwrap_or(&t).to_string();
                    vec![("minecraft:chest#main".into(), format!("entity/chest/{t}"))]
                }
                "shulker_box" => {
                    check_keys(renderer_node, &SHULKER_RENDERER_KEYS, id, &renderer)?;
                    let t = param("texture").unwrap_or_else(|| "shulker".into());
                    let t = t.strip_prefix("minecraft:").unwrap_or(&t).to_string();
                    vec![("minecraft:shulker_box#main".into(), format!("entity/shulker/{t}"))]
                }
                "banner" => {
                    check_keys(renderer_node, &BANNER_RENDERER_KEYS, id, &renderer)?;
                    // the pole rides the pattern-canvas sprite (untinted);
                    // the flag rides the base cloth, tinted by the dye
                    vec![
                        ("minecraft:standing_banner#main".into(), "entity/banner/base".into()),
                        ("minecraft:standing_banner#flag".into(), "entity/banner/banner_base".into()),
                    ]
                }
                "head" => {
                    check_keys(renderer_node, &HEAD_RENDERER_KEYS, id, &renderer)?;
                    let kind = param("kind")
                        .ok_or_else(|| format!("{id}: head renderer without kind"))?;
                    let (layer, tex) = match kind.as_str() {
                        "creeper" => ("minecraft:creeper_head#main", "entity/creeper/creeper"),
                        "skeleton" => ("minecraft:skeleton_skull#main", "entity/skeleton/skeleton"),
                        "wither_skeleton" => {
                            ("minecraft:wither_skeleton_skull#main", "entity/skeleton/wither_skeleton")
                        }
                        "zombie" => ("minecraft:zombie_head#main", "entity/zombie/zombie"),
                        "piglin" => ("minecraft:piglin_head#main", "entity/piglin/piglin"),
                        "dragon" => ("minecraft:dragon_skull#main", "entity/enderdragon/dragon"),
                        other => return Err(format!("{id}: head kind {other} — no layer mapping")),
                    };
                    vec![(layer.into(), tex.into())]
                }
                "player_head" => {
                    check_keys(renderer_node, &PLAYER_HEAD_RENDERER_KEYS, id, &renderer)?;
                    // SkullBlockRenderer.SKIN_BY_TYPE's PLAYER entry
                    vec![("minecraft:player_head#main".into(), "entity/player/slim/steve".into())]
                }
                "conduit" => {
                    check_keys(renderer_node, &CONDUIT_RENDERER_KEYS, id, &renderer)?;
                    // the resting conduit: the shell, the cage ring and
                    // the closed eye (the wind columns are the active
                    // state's) — ConduitRenderer's own sprite constants
                    vec![
                        ("minecraft:conduit#shell".into(), "entity/conduit/base".into()),
                        ("minecraft:conduit#cage".into(), "entity/conduit/cage".into()),
                        ("minecraft:conduit#eye".into(), "entity/conduit/closed_eye".into()),
                    ]
                }
                "decorated_pot" => {
                    check_keys(renderer_node, &POT_RENDERER_KEYS, id, &renderer)?;
                    vec![
                        ("minecraft:decorated_pot_base#main".into(), "entity/decorated_pot/decorated_pot_base".into()),
                        ("minecraft:decorated_pot_sides#main".into(), "entity/decorated_pot/decorated_pot_side".into()),
                    ]
                }
                "copper_golem_statue" => {
                    check_keys(renderer_node, &STATUE_RENDERER_KEYS, id, &renderer)?;
                    let pose = param("pose")
                        .ok_or_else(|| format!("{id}: statue renderer without pose"))?;
                    let layer = match pose.as_str() {
                        "standing" => "minecraft:copper_golem#main",
                        "sitting" => "minecraft:copper_golem_sitting#main",
                        "running" => "minecraft:copper_golem_running#main",
                        "star" => "minecraft:copper_golem_star#main",
                        other => return Err(format!("{id}: statue pose {other} — no layer mapping")),
                    };
                    let t = param("texture")
                        .ok_or_else(|| format!("{id}: statue renderer without texture"))?;
                    let t = t
                        .strip_prefix("minecraft:textures/")
                        .and_then(|s| s.strip_suffix(".png"))
                        .ok_or_else(|| format!("{id}: statue texture {t} is not a jar png path"))?
                        .to_string();
                    vec![(layer.into(), t)]
                }
                other => return Err(format!("special:{other} — no renderer mapping")),
            };
            let tint = if renderer == "banner" {
                let color = renderer_node
                    .get("color")
                    .and_then(Value::as_str)
                    .ok_or_else(|| format!("{id}: banner renderer without color"))?;
                Some(color.to_string())
            } else {
                None
            };
            // the active chain: the wrapper's own transform OR the
            // inherited ancestor's (the shield's scale rides its
            // condition; the golem statue's rides its select)
            let transform = active.filter(|t| !t.is_identity());
            Ok(vec![ResolvedLayer {
                model: String::new(),
                tints: Vec::new(),
                transform,
                special: Some(SpecialPayload { renderer, base, layers, tint }),
            }])
        }
        "minecraft:bundle/selected_item" => {
            Err("bundle/selected_item — the GUI-only node".to_string())
        }
        other => Err(format!("{id}: unknown node type {other}")),
    }
}

fn check_keys(
    obj: &serde_json::Map<String, Value>,
    allowed: &[&str],
    id: &str,
    ty: &str,
) -> Result<(), String> {
    for key in obj.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(format!(
                "{id}: unexpected {ty} key \"{key}\" (extend the resolver first)"
            ));
        }
    }
    Ok(())
}

/// Decode the node transformation object — the four components verbatim
/// (f64 JSON -> f32), the vanilla codec's identity defaults where absent.
fn decode_transform(v: &Value) -> Result<TransformComponents, String> {
    let obj = v
        .as_object()
        .ok_or_else(|| "the transformation must be an object".to_string())?;
    for key in obj.keys() {
        if !TRANSFORM_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "unexpected transformation key \"{key}\" (extend the decoder first)"
            ));
        }
    }
    let vec3 = |key: &str, default: [f32; 3]| -> Result<[f32; 3], String> {
        match obj.get(key) {
            None => Ok(default),
            Some(v) => {
                let arr = v
                    .as_array()
                    .ok_or_else(|| format!("transformation {key} must be an array"))?;
                if arr.len() != 3 {
                    return Err(format!("transformation {key} must hold three numbers"));
                }
                let mut out = default;
                for (i, x) in arr.iter().enumerate() {
                    out[i] = x
                        .as_f64()
                        .ok_or_else(|| format!("transformation {key} must hold numbers"))?
                        as f32;
                }
                Ok(out)
            }
        }
    };
    let quat = |key: &str, default: [f32; 4]| -> Result<[f32; 4], String> {
        match obj.get(key) {
            None => Ok(default),
            Some(v) => {
                let arr = v
                    .as_array()
                    .ok_or_else(|| format!("transformation {key} must be an array"))?;
                if arr.len() != 4 {
                    return Err(format!("transformation {key} must hold four numbers"));
                }
                let mut out = default;
                for (i, x) in arr.iter().enumerate() {
                    out[i] = x
                        .as_f64()
                        .ok_or_else(|| format!("transformation {key} must hold numbers"))?
                        as f32;
                }
                Ok(out)
            }
        }
    };
    Ok(TransformComponents {
        translation: vec3("translation", [0.0; 3])?,
        left_rotation: quat("left_rotation", [0.0, 0.0, 0.0, 1.0])?,
        scale: vec3("scale", [1.0; 3])?,
        right_rotation: quat("right_rotation", [0.0, 0.0, 0.0, 1.0])?,
    })
}

/// Emit `generated/item_models.rs`: the plain table, the resolved
/// resting-hand layers, the reasoned skip list and the root-type census.
pub fn emit(data: &ItemModelsData) -> Result<String> {
    let mut out = String::with_capacity(256 << 10);
    out.push_str("//! GENERATED by mc2rs-gen from assets/minecraft/items/ of the official jar.\n");
    out.push_str("//! The item model definitions (the 1.21.4+ format): plain minecraft:model\n");
    out.push_str("//! roots resolved to (model path, tints as canonical compact JSON, optional\n");
    out.push_str("//! swap_animation_scale); the OTHER trees evaluated by the RESTING-HAND\n");
    out.push_str("//! resolver — condition on_false, select fallback (no case targets a hand\n");
    out.push_str("//! context in the whole jar), range_dispatch's threshold<=0 entry, the\n");
    out.push_str("//! composite children with their decoded transformations — land in\n");
    out.push_str("//! RESOLVED_ITEM_LAYERS; the special block-entity renderers (no model\n");
    out.push_str("//! JSON exists — the geometry is bytecode) stay in the reasoned skip\n");
    out.push_str("//! list, never silently dropped.\n\n");

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

    out.push_str("/// One resting-hand layer of a resolved complex definition. The\n");
    out.push_str("/// transformation is the decoded node transform — (translation,\n");
    out.push_str("/// left_rotation quaternion [x,y,z,w], scale, right_rotation) — the\n");
    out.push_str("/// com.mojang.math.Transformation compose order T*L*S*R; None = identity.\n");
    out.push_str("pub struct ResolvedItemLayer {\n");
    out.push_str("    pub model: &'static str,\n");
    out.push_str("    pub tints: &'static [&'static str],\n");
    out.push_str("    pub transform: Option<([f32; 3], [f32; 4], [f32; 3], [f32; 4])>,\n");
    out.push_str("}\n\n");
    out.push_str("/// (item id, swap_animation_scale, the resting-hand layers in order).\n");
    // the split: a resolved item is EITHER plain model layers (the
    // RESOLVED_ITEM_LAYERS table) OR one special-renderer payload (the
    // ITEM_SPECIAL_MODELS table) — never both
    let mut specials: Vec<(&String, &ResolvedItem)> = Vec::new();
    let mut layer_items: Vec<(&String, &ResolvedItem)> = Vec::new();
    for (id, item) in &data.resolved {
        let sp = item.layers.iter().filter(|l| l.special.is_some()).count();
        if sp == 0 {
            layer_items.push((id, item));
        } else if sp == 1 && item.layers.len() == 1 {
            specials.push((id, item));
        } else {
            anyhow::bail!("item {id}: a mixed or multi-special resolution");
        }
    }

    out.push_str("pub static RESOLVED_ITEM_LAYERS: [(&str, Option<f32>, &[ResolvedItemLayer]); ");
    out.push_str(&layer_items.len().to_string());
    out.push_str("] = [\n");
    for (id, item) in &layer_items {
        let swap = match item.swap_animation_scale {
            Some(v) => format!("Some({v:?})"),
            None => "None".to_string(),
        };
        let layers: Vec<String> = item
            .layers
            .iter()
            .map(|l| {
                let tints = match l.tints.as_slice() {
                    [] => "&[]".to_string(),
                    [one] => format!("&[{one:?}]"),
                    many => {
                        let inner: Vec<String> = many.iter().map(|t| format!("{t:?}")).collect();
                        format!("&[{}]", inner.join(", "))
                    }
                };
                let tf = match &l.transform {
                    None => "None".to_string(),
                    Some(t) => format!(
                        "Some(({:?}, {:?}, {:?}, {:?}))",
                        t.translation, t.left_rotation, t.scale, t.right_rotation
                    ),
                };
                format!(
                    "ResolvedItemLayer {{ model: {:?}, tints: {}, transform: {} }}",
                    l.model, tints, tf
                )
            })
            .collect();
        out.push_str(&format!(
            "    (\"{}\", {}, &[{}]),\n",
            id,
            swap,
            layers.join(", ")
        ));
    }
    out.push_str("];\n\n");

    out.push_str("/// One special-renderer layer: the LayerDefinitions geometry id and\n");
    out.push_str("/// the atlas sprite texture it samples; the banner flag's tint is\n");
    out.push_str("/// the dye name (DyeColor.textureDiffuseColor, the dump's lane 4).\n");
    out.push_str("pub struct SpecialLayerRef {\n    pub layer: &'static str,\n    pub texture: &'static str,\n    pub tint: Option<&'static str>,\n}\n\n");
    out.push_str("/// One special-renderer item: the wrapper's base model path (whose\n");
    out.push_str("/// display stanza poses the geometry) and the layers in render order.\n");
    out.push_str("pub struct SpecialItemDef {\n    pub base: &'static str,\n    pub layers: &'static [SpecialLayerRef],\n    pub transform: Option<([f32; 3], [f32; 4], [f32; 3], [f32; 4])>,\n}\n\n");
    out.push_str("/// (item id, the special definition) — the resting branch's block-entity\n");
    out.push_str("/// renderer, RESOLVED from the tree (the wrapper's base + the renderer\n");
    out.push_str("/// kind's LayerDefinitions ids and Sheet sprite constants).\n");
    // the per-item statics first, then the referencing table
    for (id, item) in &specials {
        let l = &item.layers[0];
        let sp = l.special.as_ref().expect("the special payload");
        let tf = match &l.transform {
            None => "None".to_string(),
            Some(t) => format!(
                "Some(({:?}, {:?}, {:?}, {:?}))",
                t.translation, t.left_rotation, t.scale, t.right_rotation
            ),
        };
        let slug = special_slug(id);
        out.push_str(&format!("static SPECIAL_{slug}: SpecialItemDef = SpecialItemDef {{\n"));
        out.push_str(&format!("    base: {:?},\n", sp.base));
        out.push_str("    layers: &[\n");
        for (layer, tex) in &sp.layers {
            let tint = match &sp.tint {
                Some(dye) if layer.ends_with("#flag") => format!("Some({dye:?})"),
                _ => "None".to_string(),
            };
            out.push_str(&format!(
                "        SpecialLayerRef {{ layer: {layer:?}, texture: {tex:?}, tint: {tint} }},\n"
            ));
        }
        out.push_str("    ],\n");
        out.push_str(&format!("    transform: {tf},\n}};\n"));
    }
    out.push_str("pub static ITEM_SPECIAL_MODELS: &[(&str, &SpecialItemDef); ");
    out.push_str(&specials.len().to_string());
    out.push_str("] = &[\n");
    for (id, _) in &specials {
        let slug = special_slug(id);
        out.push_str(&format!("    (\"{id}\", &SPECIAL_{slug}),\n"));
    }
    out.push_str("];\n\n");

    out.push_str("/// One item's GUI-context icon: the gui branch's model paths or\n");
    out.push_str("/// the special payload marker (the GuiGraphicsExtractor item()'s\n");
    out.push_str("/// updateForTopItem(GUI) resolution, the branch laws the gui speaks).\n");
    out.push_str("pub struct GuiIconDef {\n    pub models: &'static [&'static str],\n    pub special: bool,\n}\n\n");
    out.push_str("/// The GUI icon OVERRIDES for the non-plain definitions — the plain\n");
    out.push_str("/// defs need none (their gui icon IS their model path, the tint\n");
    out.push_str("/// jsons riding ITEM_MODEL_DEFS). Sorted by id.\n");
    out.push_str("pub static ITEM_GUI_OVERRIDES: &[(&str, GuiIconDef); ");
    out.push_str(&data.gui_icons.len().to_string());
    out.push_str("] = &[\n");
    for (id, icon) in &data.gui_icons {
        let models = match icon.models.as_slice() {
            [] => "&[]".to_string(),
            [one] => format!("&[{one:?}]"),
            many => {
                let inner: Vec<String> = many.iter().map(|m| format!("{m:?}")).collect();
                format!("&[{}]", inner.join(", "))
            }
        };
        out.push_str(&format!(
            "    (\"{id}\", GuiIconDef {{ models: {}, special: {} }}),\n",
            models, icon.special
        ));
    }
    out.push_str("];\n\n");

    out.push_str("/// One charge-keyed state of the using branch: the charge value\n");
    out.push_str("/// this state engages at and the payload — either the model-path\n");
    out.push_str("/// layers (empty for a special payload) or the special def (the\n");
    out.push_str("/// shield_blocking / trident_throwing lane).\n");
    out.push_str("pub struct UsingStateDef {\n    pub threshold: f32,\n    pub layers: &'static [ResolvedItemLayer],\n    pub special: Option<&'static SpecialItemDef>,\n}\n\n");
    out.push_str("/// One item's USING-context definition — the tree evaluated with\n");
    out.push_str("/// the minecraft:using_item gate TRUE. `property` names the\n");
    out.push_str("/// charge's range-select property (empty = the pure gate), `scale`\n");
    out.push_str("/// its dispatch scale, `states` the charge-keyed payloads\n");
    out.push_str("/// (thresholds ascending, the fallback first at 0.0 — the LAST\n");
    out.push_str("/// threshold <= charge wins, the vanilla loop's own law).\n");
    out.push_str("pub struct ItemUsingDef {\n    pub property: &'static str,\n    pub scale: f32,\n    pub states: &'static [UsingStateDef],\n}\n\n");
    out.push_str("/// The USING branches — the 5 items whose trees carry the\n");
    out.push_str("/// using_item gate (bow, crossbow, goat_horn, shield, trident).\n");
    out.push_str("/// Sorted by id.\n");
    // the per-item statics FIRST (the layer arrays and the special
    // defs), then the referencing table — the same shape the resting
    // specials' emit speaks
    for (id, u) in &data.using {
        let slug = special_slug(id);
        for (i, state) in u.states.iter().enumerate() {
            let special = state.layers.iter().find_map(|l| l.special.as_ref());
            if state.layers.iter().any(|l| l.special.is_some()) && state.layers.len() != 1 {
                anyhow::bail!("item {id}: a mixed using state (special + layers)");
            }
            match special {
                Some(sp) => {
                    let tf = match &state.layers[0].transform {
                        None => "None".to_string(),
                        Some(t) => format!(
                            "Some(({:?}, {:?}, {:?}, {:?}))",
                            t.translation, t.left_rotation, t.scale, t.right_rotation
                        ),
                    };
                    out.push_str(&format!("static USING_SPECIAL_{slug}: SpecialItemDef = SpecialItemDef {{\n"));
                    out.push_str(&format!("    base: {:?},\n", sp.base));
                    out.push_str("    layers: &[\n");
                    for (layer, tex) in &sp.layers {
                        let tint = match &sp.tint {
                            Some(dye) if layer.ends_with("#flag") => format!("Some({dye:?})"),
                            _ => "None".to_string(),
                        };
                        out.push_str(&format!(
                            "        SpecialLayerRef {{ layer: {layer:?}, texture: {tex:?}, tint: {tint} }},\n"
                        ));
                    }
                    out.push_str("    ],\n");
                    out.push_str(&format!("    transform: {tf},\n}};\n"));
                }
                None => {
                    let layers: Vec<String> = state
                        .layers
                        .iter()
                        .map(|l| {
                            let tints = match l.tints.as_slice() {
                                [] => "&[]".to_string(),
                                [one] => format!("&[{one:?}]"),
                                many => {
                                    let inner: Vec<String> =
                                        many.iter().map(|t| format!("{t:?}")).collect();
                                    format!("&[{}]", inner.join(", "))
                                }
                            };
                            let tf = match &l.transform {
                                None => "None".to_string(),
                                Some(t) => format!(
                                    "Some(({:?}, {:?}, {:?}, {:?}))",
                                    t.translation, t.left_rotation, t.scale, t.right_rotation
                                ),
                            };
                            format!(
                                "ResolvedItemLayer {{ model: {:?}, tints: {}, transform: {} }}",
                                l.model, tints, tf
                            )
                        })
                        .collect();
                    out.push_str(&format!(
                        "static USING_LAYERS_{slug}_{i}: &[ResolvedItemLayer] = &[{}];\n",
                        layers.join(", ")
                    ));
                }
            }
        }
    }
    out.push_str("pub static ITEM_USING_STATES: &[(&str, ItemUsingDef); ");
    out.push_str(&data.using.len().to_string());
    out.push_str("] = &[\n");
    for (id, u) in &data.using {
        let slug = special_slug(id);
        let states: Vec<String> = u
            .states
            .iter()
            .enumerate()
            .map(|(i, state)| {
                let special = state.layers.iter().find_map(|l| l.special.as_ref());
                let body = match special {
                    Some(_) => {
                        format!("layers: &[], special: Some(&USING_SPECIAL_{slug})")
                    }
                    None => format!("layers: USING_LAYERS_{slug}_{i}, special: None"),
                };
                format!(
                    "UsingStateDef {{ threshold: {:?}, {} }}",
                    state.threshold, body
                )
            })
            .collect();
        let prop = format!("{:?}", u.property);
        out.push_str(&format!(
            "    (\"{id}\", ItemUsingDef {{ property: {prop}, scale: {:?}, states: &[{}] }}),\n",
            u.scale,
            states.join(", ")
        ));
    }
    out.push_str("];\n\n");

    out.push_str("/// Items whose definition tree cannot resolve to resting-hand layers\n");
    out.push_str("/// or special renderers — (item id, the refusal reason): the\n");
    out.push_str("/// runtime-gated properties (local_time, compass, context_dimension,\n");
    out.push_str("/// use_cycle) stay honestly unresolved.\n");
    out.push_str("pub static COMPLEX_ITEM_MODELS: &[(&str, &str)] = &[
");
    for (id, why) in &data.complex {
        out.push_str(&format!("    (\"{id}\", {why:?}),\n"));
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

/// The Rust identifier slug for a special item's static (SPECIAL_<SLUG>).
fn special_slug(id: &str) -> String {
    let mut s = String::new();
    for ch in id.chars() {
        if ch.is_ascii_alphanumeric() {
            s.push(ch.to_ascii_uppercase());
        } else {
            s.push('_');
        }
    }
    s
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
            // a selector tree -> the fallback resolves (the hand context
            // matches no case — the vanilla fallthrough)
            (
                "assets/minecraft/items/bow.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:select", "property": "minecraft:using_item",
                              "cases": [], "fallback": {"type": "minecraft:model", "model": "minecraft:item/bow"}}
                })),
            ),
            // a special renderer -> refused with the renderer named
            (
                "assets/minecraft/items/shield.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:condition", "property": "minecraft:using_item",
                              "on_true": {"type": "minecraft:special", "base": "minecraft:item/shield_blocking",
                                          "model": {"type": "minecraft:shield"}},
                              "on_false": {"type": "minecraft:special", "base": "minecraft:item/shield",
                                           "model": {"type": "minecraft:shield"}}}
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
        // the bow's select resolves through the fallback; the shield's
        // special renderer RESOLVES to its payload (the dump-backed
        // lane: the base + the layer/texture law) — nothing refuses
        assert_eq!(data.resolved["bow"].layers.len(), 1);
        assert_eq!(data.resolved["bow"].layers[0].model, "minecraft:item/bow");
        assert!(data.complex.is_empty(), "every tree resolves now");
        let sp = data.resolved["shield"].layers[0]
            .special
            .as_ref()
            .expect("the shield's special payload");
        assert_eq!(sp.base, "minecraft:item/shield");
        assert_eq!(
            sp.layers,
            vec![(
                "minecraft:shield#main".to_string(),
                "entity/shield/shield_base_nopattern".to_string()
            )]
        );
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
        // the census counts every root node exactly once (count desc;
        // ties alphabetical — condition sorts before select)
        assert_eq!(data.root_types, vec![
            ("minecraft:model".to_string(), 2),
            ("minecraft:condition".to_string(), 1),
            ("minecraft:select".to_string(), 1),
        ]);

        let emitted = emit(&data).unwrap();
        assert!(emitted.contains("(\"stone\", \"minecraft:block/stone\", &[], None)"));
        assert!(emitted.contains("Some(1.95)"));
        assert!(emitted.contains("RESOLVED_ITEM_LAYERS: [(&str, Option<f32>, &[ResolvedItemLayer]); 1"));
        assert!(emitted.contains("(\"bow\", None, &[ResolvedItemLayer { model: \"minecraft:item/bow\", tints: &[], transform: None }])"));
        assert!(emitted.contains("COMPLEX_ITEM_MODELS: &[(&str, &str)] = &[\n];"));
        // the shield rides the SPECIAL payload table (the geometry join
        // happens in the registry against the dump's layers)
        assert!(emitted.contains("(\"shield\", &SPECIAL_SHIELD)"));
        assert!(emitted.contains("base: \"minecraft:item/shield\""));
        assert!(emitted.contains("\"entity/shield/shield_base_nopattern\""));
    }

    /// The GUI-context resolution: the display_context select's gui
    /// cases win (the spears' and the trident's 2D sprites), the
    /// resting-false conditions branch on_false, the special trees
    /// mark the payload — the overrides table carries every non-plain
    /// definition and the emit speaks it.
    #[test]
    fn the_gui_resolution_evaluates_the_gui_branch() {
        let zip = zip_of(&[
            // the display_context select: the gui case is the 2D sprite
            (
                "assets/minecraft/items/trident.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:select", "property": "minecraft:display_context",
                              "cases": [{"when": ["gui", "ground"], "model": {"type": "minecraft:model", "model": "minecraft:item/trident"}}],
                              "fallback": {"type": "minecraft:special", "base": "minecraft:item/trident_in_hand",
                                           "model": {"type": "minecraft:trident"}}}
                })),
            ),
            // the condition tree: the resting-false on_false branch
            (
                "assets/minecraft/items/bow.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:condition", "property": "minecraft:using_item",
                              "on_true": {"type": "minecraft:model", "model": "minecraft:item/bow_pulling_0"},
                              "on_false": {"type": "minecraft:model", "model": "minecraft:item/bow"}}
                })),
            ),
        ]);
        let mut zip = zip;
        let data = generate(&mut zip).unwrap();
        // the trident: gui = the flat sprite, hand = the special payload
        let trident = &data.gui_icons.iter().find(|(id, _)| id == "trident").unwrap().1;
        assert_eq!(trident.models, vec!["minecraft:item/trident"]);
        assert!(!trident.special);
        assert!(data.resolved["trident"].layers[0].special.is_some());
        // the bow: gui = the resting sprite
        let bow = &data.gui_icons.iter().find(|(id, _)| id == "bow").unwrap().1;
        assert_eq!(bow.models, vec!["minecraft:item/bow"]);
        // the emit speaks the overrides verbatim
        let emitted = emit(&data).unwrap();
        assert!(emitted.contains("(\"trident\", GuiIconDef { models: &[\"minecraft:item/trident\"], special: false })"));
        assert!(emitted.contains("ITEM_GUI_OVERRIDES: &[(&str, GuiIconDef); 2]"));
    }

    #[test]
    fn the_resting_hand_resolver_evaluates_every_branch_shape() {
        // the bed composite: two layers, the foot's decoded translation
        // (the only transform family in the resolved jar set)
        let zip = zip_of(&[
            (
                "assets/minecraft/items/black_bed.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:composite", "models": [
                        {"type": "minecraft:model", "model": "minecraft:block/black_bed_head"},
                        {"type": "minecraft:model", "model": "minecraft:block/black_bed_foot",
                         "transformation": {"left_rotation": [0.0,0.0,0.0,1.0], "right_rotation": [0.0,0.0,0.0,1.0],
                                            "scale": [1.0,1.0,1.0], "translation": [0.0,0.0,1.0]}}
                    ]}
                })),
            ),
            // the spear shape: select display_context with a gui-case list
            // and a fallback (the hand context falls through)
            (
                "assets/minecraft/items/iron_spear.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:select", "property": "minecraft:display_context",
                              "cases": [{"model": {"type": "minecraft:model", "model": "minecraft:item/iron_spear"},
                                         "when": ["gui", "ground", "fixed", "on_shelf"]}],
                              "fallback": {"type": "minecraft:model", "model": "minecraft:item/iron_spear_in_hand"}},
                    "swap_animation_scale": 1.95
                })),
            ),
            // the bow shape: condition using_item -> on_false; the on_true
            // carries a range_dispatch over use_duration (unreachable)
            (
                "assets/minecraft/items/bow.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:condition", "property": "minecraft:using_item",
                              "on_false": {"type": "minecraft:model", "model": "minecraft:item/bow"},
                              "on_true": {"type": "minecraft:range_dispatch", "property": "minecraft:use_duration",
                                          "scale": 0.05, "entries": [],
                                          "fallback": {"type": "minecraft:model", "model": "minecraft:item/bow_pulling_0"}}}
                })),
            ),
            // the clock shape: range_dispatch whose threshold-0 entry wins
            // at the resting value 0
            (
                "assets/minecraft/items/clock.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:range_dispatch", "property": "minecraft:time",
                              "entries": [
                                  {"model": {"type": "minecraft:model", "model": "minecraft:item/clock_00"}, "threshold": 0.0},
                                  {"model": {"type": "minecraft:model", "model": "minecraft:item/clock_01"}, "threshold": 0.5}],
                              "fallback": {"type": "minecraft:model", "model": "minecraft:item/clock_00"}}
                })),
            ),
        ]);
        let mut zip = zip;
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.complex, vec![], "every shape resolves here");
        let bed = &data.resolved["black_bed"];
        assert_eq!(bed.layers.len(), 2, "head + foot");
        assert_eq!(bed.layers[0].model, "minecraft:block/black_bed_head");
        assert!(bed.layers[0].transform.is_none(), "identity decodes to None");
        assert_eq!(bed.layers[1].model, "minecraft:block/black_bed_foot");
        assert_eq!(
            bed.layers[1].transform,
            Some(super::TransformComponents {
                translation: [0.0, 0.0, 1.0],
                left_rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0, 1.0, 1.0],
                right_rotation: [0.0, 0.0, 0.0, 1.0],
            })
        );
        let spear = &data.resolved["iron_spear"];
        assert_eq!(spear.layers[0].model, "minecraft:item/iron_spear_in_hand");
        assert_eq!(spear.swap_animation_scale, Some(1.95), "the root swap rides");
        assert_eq!(data.resolved["bow"].layers[0].model, "minecraft:item/bow");
        assert_eq!(data.resolved["clock"].layers[0].model, "minecraft:item/clock_00", "the threshold-0 frame");
    }

    #[test]
    fn the_resolver_refuses_every_shape_the_data_does_not_justify() {
        // unknown condition property
        let zip = zip_of(&[
            (
                "assets/minecraft/items/weird.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:condition", "property": "minecraft:view_cone",
                              "on_true": {"type": "minecraft:model", "model": "minecraft:item/a"},
                              "on_false": {"type": "minecraft:model", "model": "minecraft:item/b"}}
                })),
            ),
            (
                "assets/minecraft/items/nofallback.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:select", "property": "minecraft:display_context",
                              "cases": []}
                })),
            ),
            (
                "assets/minecraft/items/mystery_node.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:teleport", "target": "nowhere"}
                })),
            ),
            (
                "assets/minecraft/items/bad_transform.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:select", "property": "minecraft:display_context",
                              "cases": [],
                              "fallback": {"type": "minecraft:model", "model": "minecraft:item/x",
                                           "transformation": {"warp": [1.0, 2.0, 3.0]}}}
                })),
            ),
        ]);
        let mut zip = zip;
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.defs.len(), 0);
        let reasons: Vec<&str> = data.complex.iter().map(|(id, why)| {
            let _ = id;
            why.as_str()
        }).collect();
        assert!(reasons.iter().any(|r| r.contains("condition property minecraft:view_cone")), "{reasons:?}");
        assert!(reasons.iter().any(|r| r.contains("select without fallback")), "{reasons:?}");
        assert!(reasons.iter().any(|r| r.contains("unknown node type minecraft:teleport")), "{reasons:?}");
        assert!(reasons.iter().any(|r| r.contains("unexpected transformation key \"warp\"")), "{reasons:?}");
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

    /// THE USING BRANCH: the five using shapes the official jar carries
    /// (the trees dumped verbatim: bow.json, crossbow.json,
    /// goat_horn.json, shield.json, trident.json), resolved with the
    /// using_item gate TRUE. The thresholds, the property names, the
    /// scales, the bases and the mirror transforms are the JSON's own
    /// values — the charge laws ride the registry (the javap evidence).
    #[test]
    fn the_using_branch_resolves_the_five_jar_trees() {
        let zip = zip_of(&[
            // bow.json verbatim
            (
                "assets/minecraft/items/bow.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:condition",
                              "on_false": {"type": "minecraft:model", "model": "minecraft:item/bow"},
                              "on_true": {"type": "minecraft:range_dispatch",
                                          "entries": [
                                              {"model": {"type": "minecraft:model", "model": "minecraft:item/bow_pulling_1"}, "threshold": 0.65},
                                              {"model": {"type": "minecraft:model", "model": "minecraft:item/bow_pulling_2"}, "threshold": 0.9}],
                                          "fallback": {"type": "minecraft:model", "model": "minecraft:item/bow_pulling_0"},
                                          "property": "minecraft:use_duration", "scale": 0.05},
                              "property": "minecraft:using_item"}
                })),
            ),
            // crossbow.json verbatim (the charge_type select falls
            // through — no case targets the hand)
            (
                "assets/minecraft/items/crossbow.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:select",
                              "cases": [
                                  {"model": {"type": "minecraft:model", "model": "minecraft:item/crossbow_arrow"}, "when": "arrow"},
                                  {"model": {"type": "minecraft:model", "model": "minecraft:item/crossbow_firework"}, "when": "rocket"}],
                              "fallback": {"type": "minecraft:condition",
                                           "on_false": {"type": "minecraft:model", "model": "minecraft:item/crossbow"},
                                           "on_true": {"type": "minecraft:range_dispatch",
                                                       "entries": [
                                                           {"model": {"type": "minecraft:model", "model": "minecraft:item/crossbow_pulling_1"}, "threshold": 0.58},
                                                           {"model": {"type": "minecraft:model", "model": "minecraft:item/crossbow_pulling_2"}, "threshold": 1.0}],
                                                       "fallback": {"type": "minecraft:model", "model": "minecraft:item/crossbow_pulling_0"},
                                                       "property": "minecraft:crossbow/pull"},
                                           "property": "minecraft:using_item"},
                              "property": "minecraft:charge_type"}
                })),
            ),
            // goat_horn.json verbatim
            (
                "assets/minecraft/items/goat_horn.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:condition",
                              "on_false": {"type": "minecraft:model", "model": "minecraft:item/goat_horn"},
                              "on_true": {"type": "minecraft:model", "model": "minecraft:item/tooting_goat_horn"},
                              "property": "minecraft:using_item"}
                })),
            ),
            // shield.json verbatim — the mirror transform rides the
            // CONDITION node (a sibling key of property/on_true)
            (
                "assets/minecraft/items/shield.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:condition",
                              "on_false": {"type": "minecraft:special", "base": "minecraft:item/shield", "model": {"type": "minecraft:shield"}},
                              "on_true": {"type": "minecraft:special", "base": "minecraft:item/shield_blocking", "model": {"type": "minecraft:shield"}},
                              "property": "minecraft:using_item",
                              "transformation": {"left_rotation": [0.0, 0.0, 0.0, 1.0], "right_rotation": [0.0, 0.0, 0.0, 1.0],
                                                 "scale": [1.0, -1.0, -1.0], "translation": [0.0, 0.0, 0.0]}}
                })),
            ),
            // trident.json verbatim — the display_context select's
            // fallback is the using gate
            (
                "assets/minecraft/items/trident.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:select",
                              "cases": [{"model": {"type": "minecraft:model", "model": "minecraft:item/trident"},
                                         "when": ["gui", "ground", "fixed", "on_shelf"]}],
                              "fallback": {"type": "minecraft:condition",
                                           "on_false": {"type": "minecraft:special", "base": "minecraft:item/trident_in_hand", "model": {"type": "minecraft:trident"}},
                                           "on_true": {"type": "minecraft:special", "base": "minecraft:item/trident_throwing", "model": {"type": "minecraft:trident"}},
                                           "property": "minecraft:using_item",
                                           "transformation": {"left_rotation": [0.0, 0.0, 0.0, 1.0], "right_rotation": [0.0, 0.0, 0.0, 1.0],
                                                              "scale": [1.0, -1.0, -1.0], "translation": [0.0, 0.0, 0.0]}},
                              "property": "minecraft:display_context"}
                })),
            ),
            // a NO-gate tree: the using map carries none of it
            (
                "assets/minecraft/items/compass.json",
                json_bytes(serde_json::json!({
                    "model": {"type": "minecraft:range_dispatch", "property": "minecraft:compass_angle",
                              "entries": [], "fallback": {"type": "minecraft:model", "model": "minecraft:item/compass"}}
                })),
            ),
        ]);
        let mut zip = zip;
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.using.len(), 5, "the five gated trees");
        assert!(data.using.get("compass").is_none(), "no gate, no using lane");

        // THE BOW: the charge-keyed pull states, the JSON's own numbers
        let bow = &data.using["bow"];
        assert_eq!(bow.property, "minecraft:use_duration");
        assert_eq!(bow.scale, 0.05);
        let th: Vec<f32> = bow.states.iter().map(|s| s.threshold).collect();
        assert_eq!(th, [0.0, 0.65, 0.9], "the fallback first, then the entries");
        assert_eq!(bow.states[0].layers[0].model, "minecraft:item/bow_pulling_0");
        assert_eq!(bow.states[1].layers[0].model, "minecraft:item/bow_pulling_1");
        assert_eq!(bow.states[2].layers[0].model, "minecraft:item/bow_pulling_2");

        // THE CROSSBOW: the select fell through to the gate, the pull
        // dispatch carries its own property, scale 1.0
        let xbow = &data.using["crossbow"];
        assert_eq!(xbow.property, "minecraft:crossbow/pull");
        assert_eq!(xbow.scale, 1.0);
        assert_eq!(xbow.states[1].layers[0].model, "minecraft:item/crossbow_pulling_1");

        // THE GOAT HORN: the single-state swap
        let horn = &data.using["goat_horn"];
        assert_eq!(horn.property, "", "the pure gate — no charge lane");
        assert_eq!(horn.states[0].layers[0].model, "minecraft:item/tooting_goat_horn");

        // THE SHIELD: the blocking special, the mirror transform decoded
        let shield = &data.using["shield"];
        assert_eq!(shield.property, "");
        let sp = shield.states[0].layers[0].special.as_ref().expect("the blocking payload");
        assert_eq!(sp.base, "minecraft:item/shield_blocking");
        assert_eq!(
            sp.layers,
            vec![("minecraft:shield#main".to_string(), "entity/shield/shield_base_nopattern".to_string())]
        );
        let tf = shield.states[0].layers[0].transform.as_ref().expect("the mirror");
        assert_eq!(tf.scale, [1.0, -1.0, -1.0], "the JSON's own mirror");

        // THE TRIDENT: the throwing special through the select's fallback
        let trident = &data.using["trident"];
        let tp = trident.states[0].layers[0].special.as_ref().expect("the throwing payload");
        assert_eq!(tp.base, "minecraft:item/trident_throwing");
        assert_eq!(
            tp.layers,
            vec![("minecraft:trident#main".to_string(), "entity/trident/trident".to_string())]
        );
        let ttf = trident.states[0].layers[0].transform.as_ref().expect("the mirror");
        assert_eq!(ttf.scale, [1.0, -1.0, -1.0]);
    }

    /// The using walk's loud-refuse law: a shape the using lane reaches
    /// but the resolver cannot honor bails the generator (a render gap
    /// on a live interaction is worse than a skipped resting shape —
    /// the round extends first, never renders a guessed payload).
    #[test]
    fn the_using_walk_refuses_unhonored_shapes_loudly() {
        // a composite on the using path (no jar tree carries one)
        let zip = zip_of(&[(
            "assets/minecraft/items/future.json",
            json_bytes(serde_json::json!({
                "model": {"type": "minecraft:condition",
                          "on_false": {"type": "minecraft:model", "model": "minecraft:item/a"},
                          "on_true": {"type": "minecraft:composite",
                                      "models": [{"type": "minecraft:model", "model": "minecraft:item/b"}]},
                          "property": "minecraft:using_item"}
            })),
        )]);
        let mut zip = zip;
        let why = match generate(&mut zip) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("a composite on the using path must refuse"),
        };
        assert!(why.contains("a composite on the using path"), "{why}");

        // another condition inside the gate (the jar nests none)
        let zip = zip_of(&[(
            "assets/minecraft/items/nested.json",
            json_bytes(serde_json::json!({
                "model": {"type": "minecraft:condition",
                          "on_false": {"type": "minecraft:model", "model": "minecraft:item/a"},
                          "on_true": {"type": "minecraft:condition", "property": "minecraft:broken",
                                      "on_false": {"type": "minecraft:model", "model": "minecraft:item/b"},
                                      "on_true": {"type": "minecraft:model", "model": "minecraft:item/c"}},
                          "property": "minecraft:using_item"}
            })),
        )]);
        let mut zip = zip;
        let why = match generate(&mut zip) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("a nested condition on the using path must refuse"),
        };
        assert!(why.contains("condition minecraft:broken on the using path"), "{why}");
    }
}

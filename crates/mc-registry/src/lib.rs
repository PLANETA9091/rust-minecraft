//! mc-registry — version-generated block/item registry.
//!
//! The `generated` module is produced by `mc2rs-gen generate --version <id>`.
//! Regenerating for a new version swaps the data without touching the API.

pub mod generated;

/// The hand-written consumer layer over generated::blockstates: the
/// strict JSON-subset parse of both canonical columns plus the vanilla
/// selection semantics (first-match variants, all-matching multipart
/// parts, "a|b" pipe values). See the module docs.
pub mod blockstate_lookup;

/// The vanilla FaceBakery quad derivation over generated::elements —
/// corners, default uvs, uv rotation, axis/euler element rotations with
/// rescale, computed facing, winding recalculation. Pure data, no
/// render path. See the module docs for the javap evidence.
pub mod element_quads;

/// The vanilla TextureSlots resolution over generated::models — the
/// parent-chain merge (child overrides per key), the "#var" reference
/// fixpoint, the 26.2 inline sprite dicts, and the strip-# lookup that
/// makes bare refs behave identically. Pure data, no render path.
pub mod texture_slots;

/// THE TEXTURE TABLE'S OWN LOADER (the migration's sixth lane): the
/// decoded RGBA8 texels the compiled source no longer carries, the
/// runtime cache's `assets-cache/gen/textures.bin` the only source, the
/// honest empty table (the vanilla checkerboard) the no-cache boot keeps.
/// The pixel accessors' home — `texture_index` stays on the compiled
/// namespace, the pixels load.
pub mod textures_table;

/// The vanilla sprite-uv consumer — the quads' 0..16 uv space mapped to
/// texels over generated::textures (CuboidFace.getU /16, the
/// TextureAtlasSprite span, NEAREST + CLAMP_TO_EDGE) with the vanilla
/// missingno checkerboard for the missing materials. Pure data, no
/// render path. See the module docs for the javap evidence.
pub mod sprite_uv;

/// The vanilla display-transform link — the models table's `display`
/// stanzas resolved per context through the parent chain into the
/// view-space pose over the quads (ItemTransform/ItemTransforms +
/// JOML 1.10.8 op-for-op, bit-pinned against the joml oracle). Pure
/// data, no render path. See the module docs for the javap evidence.
pub mod display;

/// The vanilla item-model bake — the ItemModelGenerator sprite extrusion
/// (the generated item models' quads baked from the layer sprites'
/// opaque-pixel outline over generated::textures). Pure data, no render
/// path. See the module docs for the javap evidence.
pub mod item_bake;

/// The blockstate-driven palette (the data half) — a block state -> the
/// per-face average color of its REAL model's REAL textures, over the
/// BLOCKSTATES -> blockstate_lookup -> element_quads -> texture_slots ->
/// generated::textures chain. Pure data, no render path (the mesher
/// consumes it next round). See the module docs for the semantics.
pub mod palette;

/// The vanilla biome-tag RESOLUTION semantic — the flattened member set a
/// tag builds at datapack load (`TagLoader`'s built-tag verdict over
/// `data/minecraft/tags/worldgen/biome/*.json`, the raw form `BIOME_TAGS`
/// carries verbatim): a tag's entries are walked in FILE ORDER; a concrete
/// `minecraft:*` member is emitted, a `#minecraft:*` entry expands the
/// referenced tag DEPTH-FIRST at its position (the referenced tag's own
/// file order), and the built set deduplicates on FIRST occurrence (the
/// built tag is a set: a biome reachable through two paths resolves once,
/// at the first path that reaches it). Three load failures are refused to
/// `None` instead of guessed at — an unknown tag, an unknown nested
/// reference, and a REFERENCE CYCLE (a tag that transitively contains
/// itself cannot build; the resolver fails closed rather than looping).
/// The flattened members are additionally validated against the biome
/// table: a registry-bound tag loads only when every reference resolves
/// into the registry.
///
/// The FIRST consumer surface for the generated tag table (68 tags / 390
/// raw members; the census test pins the raw join never dangles) — the
/// flattened set is what the placement, spawn-allowance, and category
/// predicates actually check against, and what any future client-side
/// tag consumer should read instead of re-deriving the expansion.
pub fn flatten_biome_tag(name: &str) -> Option<Vec<&'static str>> {
    let out = flatten_tag_table(&generated::worldgen::BIOME_TAGS, name)?;
    for member in &out {
        generated::worldgen::biome(member)?; // missing reference: refused
    }
    Some(out)
}

/// The vanilla timeline-tag RESOLUTION semantic — the same built-tag
/// verdict as [`flatten_biome_tag`], over the jar's
/// `data/minecraft/tags/timeline/*.json` table (file-order entries,
/// `#`-nested references expanded depth-first in place,
/// first-occurrence dedup, the same three load failures refused to
/// `None`: unknown tag, unknown nested reference, cycle). The flattened
/// members are validated against the TIMELINES table — every concrete
/// member must resolve into the timeline registry. This is the
/// resolution the dimension types imply: a dimension's `timelines`
/// field names a TAG verbatim (`"#minecraft:in_overworld"`), and the
/// timelines that apply to it are the tag's flattened members.
pub fn flatten_timeline_tag(name: &str) -> Option<Vec<&'static str>> {
    let out = flatten_tag_table(&generated::worldgen::TIMELINE_TAGS, name)?;
    for member in &out {
        generated::worldgen::timeline(member)?; // missing reference: refused
    }
    Some(out)
}

/// The mob-effect registry id's NAME — the resolution lane the effect
/// state's debug surface (and the future HUD icon/tooltip rounds)
/// consumes. The wire carries the 0-based built-in registration id
/// (the `holderRegistry(Registries.MOB_EFFECT)` lane); the client
/// resolves the name LOCALLY through this jar's boot order — the
/// generated table's own header: NOT a synchronized registry, the
/// order is this boot's. Out-of-range ids — negative (the decode's
/// own refusal) or past the table (a server registry NEWER than this
/// client jar's boot; vanilla's `IdMapper.byId` THROWS there) —
/// resolve to `None`: the honest unknown, never a guessed name.
pub fn mob_effect_name(id: i32) -> Option<&'static str> {
    if id < 0 {
        return None;
    }
    generated::mob_effects::MOB_EFFECT_ORDER.get(id as usize).copied()
}

/// The ITEM registry id's NAME — the resolution lane the container
/// screens' slot icons consume. The order is `BuiltInRegistries.ITEM`'s
/// own — THE WIRE's built-in registration order (the checked-in
/// DumpItemOrder.java's dump; the holderRegistry(Registries.ITEM)
/// serializers ride it, so the container lanes' item ids resolve
/// through THIS table, not the assets table's alphabetical order —
/// that is the item model JSONs' TEXTURE vocabulary). Names ride the
/// plain form (the prefix stripped). Negative ids (the decode's own
/// refusal) and ids past the table (a server registry newer than this
/// client jar's boot; the decode's `byIdOrThrow` THROWS there) resolve
/// `None`: the honest unknown, never a guessed name.
pub fn item_name(id: i32) -> Option<&'static str> {
    if id < 0 {
        return None;
    }
    generated::item_order::ITEM_WIRE_ORDER.get(id as usize).copied()
}

/// The ITEM registry id's default RARITY LEVEL — the `Rarity.id` the
/// item's own `Item.Properties` construction carries (0 COMMON / 1
/// UNCOMMON / 2 RARE / 3 EPIC). The table only rows the NON-COMMON
/// items (the javap walk of `Items.class`'s static init — each
/// `Item$Properties.rarity(...)` call paired with its registration
/// statement); the MISSING rows are the `Item.Properties` field
/// init's own COMMON default, never a guessed level. Unresolvable
/// ids are `None` — the honest unknown, the same law [`item_name`]
/// keeps.
pub fn item_rarity(id: i32) -> Option<u8> {
    let name = item_name(id)?;
    let name = name.strip_prefix("minecraft:").unwrap_or(name);
    Some(
        generated::item_rarity::ITEM_RARITY
            .binary_search_by_key(&name, |&(n, _)| n)
            .map(|idx| generated::item_rarity::ITEM_RARITY[idx].1)
            .unwrap_or(0),
    )
}

/// The ITEM registry id's default rarity's CHAT COLOR — the
/// `getRarity().color()` walk the HUD's held-item name rides, the
/// `ChatFormatting` enum's own mapping (Rarity.class's static init:
/// COMMON → WHITE / UNCOMMON → YELLOW / RARE → AQUA / EPIC →
/// LIGHT_PURPLE) resolved through `TextColor.fromLegacyFormat`'s own
/// switch (TextColor.class's static init: white 16777215 = 0xFFFFFF,
/// yellow 16777045 = 0xFFFF55, aqua 5636095 = 0x55FFFF, light_purple
/// 16733695 = 0xFF55FF — the 26.2 jar's NAMED-color palette, the
/// legacy §-code trios). Unresolvable ids are `None`.
pub fn item_rarity_color(id: i32) -> Option<u32> {
    match item_rarity(id)? {
        1 => Some(0xFFFF55),
        2 => Some(0x55FFFF),
        3 => Some(0xFF55FF),
        _ => Some(0xFFFFFF),
    }
}

/// The ITEM registry id's flat GUI icon — the generated texture table's
/// `item/<name>` RGBA texels with the (w, h) the sprite rides (16x16
/// for the plain items the chest screen pins; the 3D block-model items
/// are the model round's own line). Unresolvable ids and the pinned
/// missing materials are `None` — the honest unknown draws nothing.
pub fn item_icon(id: i32) -> Option<(&'static [u8], u16, u16)> {
    let name = item_name(id)?;
    let idx = generated::textures::texture_index(&format!("item/{name}"))?;
    let (w, h) = textures_table::texture_size(idx);
    Some((textures_table::texture_rgba(idx), w, h))
}

/// The ITEM registry id's first-person model path — the generated
/// item-model definition table's PLAIN `minecraft:model` roots (the
/// 1.21.4+ items/ format), the namespace stripped to the bake's own
/// vocabulary (`"item/diamond"`, the shape `posed_item_quads` walks).
/// The complex-definition trees resolve through [`item_model_layers`]
/// (the resting-hand table); THIS accessor keeps the plain-only
/// single-path semantics — the offline `--held-item` golden lane's own
/// vocabulary — and the unresolvable ids are `None`: the honest
/// unknown, never a guessed name.
pub fn item_model_path(id: i32) -> Option<&'static str> {
    let name = item_name(id)?;
    generated::item_models::ITEM_MODEL_DEFS
        .iter()
        .find(|d| d.0 == name)
        .map(|d| d.1.strip_prefix("minecraft:").unwrap_or(d.1))
}

/// One resting-hand model layer: the (namespace-stripped) model path,
/// the tints as canonical compact JSON, and the decoded node
/// transformation — (translation, left_rotation quaternion [x,y,z,w],
/// scale, right_rotation) — composed T·L·S·R per the
/// `com.mojang.math.Transformation.compose` bytecode; `None` = identity.
pub struct ItemModelLayer {
    pub model: &'static str,
    pub tints: &'static [&'static str],
    pub transform: Option<([f32; 3], [f32; 4], [f32; 3], [f32; 4])>,
}

/// The ITEM registry id's RESTING-HAND model layers — the vanilla item
/// model definition trees evaluated for the first-person hand of a
/// fresh, unused stack. The plain `minecraft:model` roots ride as their
/// single layer; the complex trees resolve through the generated
/// resting-hand table (condition on_false, select fallback — no case in
/// the jar targets a hand context, range_dispatch's threshold<=0 entry,
/// the composite children with their transformations); the special
/// block-entity renderers (shield, trident, chest, shulker box, banner,
/// head, conduit, decorated pot, golem statue — no model JSON exists,
/// the geometry is bytecode) and the unknown ids are `None`: the honest
/// unknown, the caller renders the bare arm.
pub fn item_model_layers(id: i32) -> Option<Vec<ItemModelLayer>> {
    let name = item_name(id)?;
    if let Some(def) = generated::item_models::ITEM_MODEL_DEFS
        .iter()
        .find(|d| d.0 == name)
    {
        return Some(vec![ItemModelLayer {
            model: def.1.strip_prefix("minecraft:").unwrap_or(def.1),
            tints: def.2,
            transform: None,
        }]);
    }
    generated::item_models::RESOLVED_ITEM_LAYERS
        .iter()
        .find(|d| d.0 == name)
        .map(|(_, _, layers)| {
            layers
                .iter()
                .map(|l| ItemModelLayer {
                    model: l.model.strip_prefix("minecraft:").unwrap_or(l.model),
                    tints: l.tints,
                    transform: l.transform,
                })
                .collect()
        })
}

/// THE BROKEN CONDITION'S ON_TRUE ARM (the vanilla item jsons' own
/// walk — the 26.2 cache's `items/elytra.json` the only root
/// `minecraft:broken` condition the items/ tree speaks, the javap
/// evidence byte-level: `properties.conditional.Broken.get` is
/// `stack.nextDamageWillBreak()` verbatim — the aload_1/invokevirtual
/// pair the whole body keeps — and `ItemStack.nextDamageWillBreak` is
/// `isDamageableItem() && getDamageValue() >= getMaxDamage() - 1`:
/// the isDamageableItem gate the ifeq answers false, the damage the
/// max-minus-one threshold the if_icmplt compares, the crossing
/// (pc 17) the true verdict wears). The rows (item name, the
/// on_true branch's plain model path): the elytra's condition —
/// on_false `minecraft:item/elytra` (the RESOLVED_ITEM_LAYERS row the
/// resting accessor already speaks), on_true `minecraft:item/elytra_broken`
/// (a plain model node, no tints, no transform — the same shape the
/// plain-def rows keep). The sibling `Damaged` property rides
/// `isDamaged()` — no item json roots it, the table stays one row.
pub const BROKEN_CONDITION_ON_TRUE: &[(&str, &str)] =
    &[("elytra", "minecraft:item/elytra_broken")];

/// The ITEM registry id's resting-hand model layers with the
/// `minecraft:broken` condition's own verdict spoken: `broken = true`
/// swaps the condition items' layers to the on_true branch's model
/// (the torn elytra's icon/hand render the vanilla wear state wears —
/// one damage point BEFORE the break, the same threshold the glide's
/// `canGlideUsing` refusal pairs with), `false` or a non-condition
/// item rides [`item_model_layers`] untouched.
pub fn item_model_layers_broken(id: i32, broken: bool) -> Option<Vec<ItemModelLayer>> {
    if broken {
        let name = item_name(id)?;
        if let Some((_, model)) = BROKEN_CONDITION_ON_TRUE
            .iter()
            .find(|(n, _)| *n == name)
        {
            return Some(vec![ItemModelLayer {
                model: model.strip_prefix("minecraft:").unwrap_or(model),
                tints: &[],
                transform: None,
            }]);
        }
    }
    item_model_layers(id)
}

/// One resolved special layer: the geometry (the dump's polygons) and
/// the texture id the pixels ride. The tinted banner flag binds its
/// PER-COLOR pre-tinted variant ("entity/banner/banner_base_<dye>" —
/// the DyeColor textureDiffuseColor multiply, generation-time); every
/// other layer binds its raw entity texture.
pub struct SpecialItemLayer {
    pub layer: &'static generated::special_models::SpecialLayer,
    pub texture: &'static str,
}

/// One special item's full resting-hand payload: the base model (whose
/// display stanza poses the geometry), the item-level LOCAL transform
/// (the renderer's own pose, the tree resolution's decode; None =
/// identity), and the layers in render order (the string ids of the
/// generated law joined against the dump's geometry tables).
pub struct SpecialItemRender {
    pub base: &'static str,
    pub transform: Option<([f32; 3], [f32; 4], [f32; 3], [f32; 4])>,
    pub layers: Vec<SpecialItemLayer>,
}

pub(crate) fn special_render_from_def(
    def: &'static generated::item_models::SpecialItemDef,
) -> Option<SpecialItemRender> {
    let layers = def
        .layers
        .iter()
        .map(|l| {
            let geometry = generated::special_models::SPECIAL_LAYERS
                .iter()
                .find(|(id, _)| *id == l.layer)
                .map(|(_, g)| *g)?;
            let texture = match l.tint {
                Some(dye) => {
                    let want = format!("{}_{}", l.texture, dye);
                    let idx = generated::textures::texture_index(&want)?;
                    generated::textures::TEXTURE_IDS[idx as usize]
                }
                None => l.texture,
            };
            Some(SpecialItemLayer { layer: geometry, texture })
        })
        .collect::<Option<Vec<_>>>()?;
    Some(SpecialItemRender {
        base: def.base.strip_prefix("minecraft:").unwrap_or(def.base),
        transform: def.transform,
        layers,
    })
}

fn special_item_render_by_name(name: &str) -> Option<SpecialItemRender> {
    let (_, def) = generated::item_models::ITEM_SPECIAL_MODELS
        .iter()
        .find(|d| d.0 == name)?;
    special_render_from_def(def)
}

/// The ITEM registry id's SPECIAL-renderer payload — the 63 block-entity
/// / entity-model items (shield, trident, the chests, the shulker boxes,
/// the banners, the heads, the conduit, the decorated pot, the golem
/// statues) whose geometry the running client's OWN LayerDefinitions
/// dumped (the resting-hand polygons, bit-exact). The plain and complex
/// trees go through [`item_model_layers`]; the two lanes are disjoint
/// (the generator's census: 1390 plain + 84 resolved + 63 special = the
/// whole registry, 0 refuses). Unknown ids are `None`.
pub fn special_item_renderer(id: i32) -> Option<SpecialItemRender> {
    special_item_render_by_name(item_name(id)?)
}

/// The crossbow's charge duration at the enchantment-less base —
/// javap CrossbowItem.getChargeDuration: `Mth.floor(
/// modifyCrossbowChargingTime(.., 1.25f) * 20f)` = 25 ticks. The
/// pull property divides by it (the dispatch's own law) and the
/// release law loads the projectile when the power reaches 1.0 —
/// the same constant both lanes speak.
pub const CROSSBOW_CHARGE_TICKS: u32 = 25;

/// The brush's use duration — javap `BrushItem.getUseDuration` over the
/// 26.2 jar: the `sipush 200` the ireturn returns verbatim (200 ticks —
/// the using window's own length). The tick walk's completion the same
/// constant speaks (`--useItemRemaining <= 0` the completeUsingItem
/// fires), the cadence voice the `(duration - remaining + 1) % 10 == 5`
/// gate the same duration divides.
pub const BRUSH_USE_TICKS: u32 = 200;

/// The item's USING-branch definition — the 5 items whose definition
/// trees carry the `minecraft:using_item` gate (bow, crossbow,
/// goat_horn, shield, trident). `None` = the item has no using lane
/// (the right-mouse hold renders nothing different — the honest gate
/// the App's use arc speaks).
pub fn item_using_def(
    id: i32,
) -> Option<&'static generated::item_models::ItemUsingDef> {
    let name = item_name(id)?;
    generated::item_models::ITEM_USING_STATES
        .iter()
        .find(|d| d.0 == name)
        .map(|d| &d.1)
}

/// The charge the item's dispatch property reads at `elapsed` use
/// ticks — the vanilla property classes' own math (javap-pinned):
/// `minecraft:use_duration` = UseDuration.get (remaining=false) =
/// `getUseDuration - remainingTicks` = the elapsed ticks, times the
/// dispatch's scale; `minecraft:crossbow/pull` = CrossbowPull.get =
/// `useDuration / getChargeDuration` with getChargeDuration =
/// `Mth.floor(modifyCrossbowChargingTime(.., 1.25f) * 20f)` = 25 ticks
/// at the enchantment-less base. An unknown property reads 0 (the
/// bottom state renders — the generator only emits the two known ones).
pub fn using_charge(def: &generated::item_models::ItemUsingDef, elapsed_ticks: u32) -> f32 {
    match def.property {
        "minecraft:use_duration" => elapsed_ticks as f32 * def.scale,
        "minecraft:crossbow/pull" => elapsed_ticks as f32 / CROSSBOW_CHARGE_TICKS as f32,
        _ => 0.0,
    }
}

/// The vanilla range_dispatch pick: the LAST state whose threshold
/// <= charge wins (the fallback at 0.0 always matches — charge never
/// goes negative).
pub fn using_state_index(
    def: &generated::item_models::ItemUsingDef,
    charge: f32,
) -> usize {
    let mut picked = 0;
    for (i, s) in def.states.iter().enumerate() {
        if s.threshold <= charge {
            picked = i;
        }
    }
    picked
}

/// One using state's render payload: either the model-path layers
/// (the bow's and the crossbow's pulling sprites, the goat horn's
/// tooting model) or the special renderer payload (the shield's
/// blocking base + the mirror transform, the trident's throwing pose).
pub enum UsingStateRender {
    Layers(Vec<ItemModelLayer>),
    Special(SpecialItemRender),
}

/// The tests' honest probe — the model paths name the payload, the
/// special arm keeps its shape (the dump geometry carries no Debug).
impl std::fmt::Debug for UsingStateRender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UsingStateRender::Layers(layers) => {
                let models: Vec<&str> = layers.iter().map(|l| l.model).collect();
                write!(f, "Layers({models:?})")
            }
            UsingStateRender::Special(render) => write!(
                f,
                "Special(base: {}, layers: {})",
                render.base,
                render.layers.len()
            ),
        }
    }
}

/// The state's render payload at the picked index — the same
/// three-table join the resting special law speaks (the dump geometry,
/// the texture namespace), through the state's own def.
pub fn item_using_state(id: i32, index: usize) -> Option<UsingStateRender> {
    let def = item_using_def(id)?;
    let state = def.states.get(index)?;
    if let Some(sp) = state.special {
        return special_render_from_def(sp).map(UsingStateRender::Special);
    }
    let layers = state
        .layers
        .iter()
        .map(|l| ItemModelLayer {
            model: l.model.strip_prefix("minecraft:").unwrap_or(l.model),
            tints: l.tints,
            transform: l.transform,
        })
        .collect();
    Some(UsingStateRender::Layers(layers))
}

/// Which loaded projectile the crossbow's CHARGED_PROJECTILES
/// component carries — the CrossbowItem$ChargeType enum's own shape
/// (javap Charge.get: contains FIREWORK_ROCKET -> ROCKET, any other
/// projectile -> ARROW; the empty component never reaches a charge
/// lane — the fallback renders instead).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChargeKind {
    Arrow,
    Rocket,
}

/// The item's CHARGE-branch definition — the item whose tree carries
/// the `minecraft:charge_type` select (the census: exactly the
/// crossbow). `None` = the item renders one resting shape regardless
/// of its component state (the honest no-lane).
pub fn item_charge_def(
    id: i32,
) -> Option<&'static generated::item_models::ItemChargeDef> {
    let name = item_name(id)?;
    generated::item_models::ITEM_CHARGE_STATES
        .iter()
        .find(|d| d.0 == name)
        .map(|d| &d.1)
}

/// The charged state's render layers — the select case's model
/// resolution (the crossbow's loaded bolt rides the crossbow_arrow /
/// crossbow_firework models, the same three-table join the plain
/// lane speaks). `None` = no charge lane; the prefix strip follows
/// the registry's own vocabulary.
pub fn item_charge_layers(id: i32, kind: ChargeKind) -> Option<Vec<ItemModelLayer>> {
    let def = item_charge_def(id)?;
    let layers = match kind {
        ChargeKind::Arrow => def.arrow,
        ChargeKind::Rocket => def.rocket,
    };
    Some(
        layers
            .iter()
            .map(|l| ItemModelLayer {
                model: l.model.strip_prefix("minecraft:").unwrap_or(l.model),
                tints: l.tints,
                transform: l.transform,
            })
            .collect(),
    )
}

/// The registry's reverse lookup — the WIRE id whose ITEM_WIRE_ORDER
/// name is `name` (the offline `--held-item` model id's own route to
/// the wire domain the charge and using lanes speak). `None` = not a
/// registry item.
pub fn item_id_by_name(name: &str) -> Option<i32> {
    generated::item_order::ITEM_WIRE_ORDER
        .iter()
        .position(|n| *n == name)
        .map(|p| p as i32)
}

/// The resolver core, generic over the table so the dedup and cycle
/// rules are testable without a jar that contains a cycle (the real
/// 68-tag table cannot cycle: the census join proves every nested
/// reference resolves, and a cyclic datapack would refuse to load in
/// vanilla too). The expansion stack starts at the tag's own key so a
/// direct self-reference is refused by the same rule as a deeper one.
fn flatten_tag_table<'a>(
    table: &[(&'a str, &'a [&'a str])],
    name: &str,
) -> Option<Vec<&'a str>> {
    fn expand<'a>(
        table: &[(&'a str, &'a [&'a str])],
        members: &'a [&'a str],
        stack: &mut Vec<&'a str>,
        out: &mut Vec<&'a str>,
    ) -> Option<()> {
        for &entry in members {
            if let Some(id) = entry.strip_prefix('#') {
                // a nested reference expands depth-first in place; an id
                // already on the expansion stack is a cycle -> refused
                if stack.contains(&id) {
                    return None;
                }
                let nested = table
                    .binary_search_by_key(&id, |t| t.0)
                    .ok()
                    .map(|i| table[i].1)?;
                stack.push(id);
                expand(table, nested, stack, out)?;
                stack.pop();
            } else if !out.contains(&entry) {
                out.push(entry); // first occurrence wins (the built set)
            }
        }
        Some(())
    }
    let idx = table.binary_search_by_key(&name, |t| t.0).ok()?;
    let (key, members) = table[idx];
    let mut stack = vec![key];
    let mut out = Vec::new();
    expand(table, members, &mut stack, &mut out)?;
    Some(out)
}

/// One block definition: identifier + sorted list of state property names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block {
    pub name: &'static str,
    pub properties: &'static [&'static str],
}

/// Static, binary-searchable registry over generated data.
pub struct Registry {
    blocks: &'static [Block],
    items: &'static [&'static str],
}

impl Registry {
    pub const fn new(blocks: &'static [Block], items: &'static [&'static str]) -> Self {
        Self { blocks, items }
    }

    pub fn blocks(&self) -> &'static [Block] {
        self.blocks
    }

    pub fn items(&self) -> &'static [&'static str] {
        self.items
    }

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Find a block by identifier (exact match, binary search).
    pub fn find_block(&self, name: &str) -> Option<&'static Block> {
        self.blocks
            .binary_search_by(|b| b.name.cmp(name))
            .ok()
            .map(|i| &self.blocks[i])
    }

    /// Does an item with this identifier exist?
    pub fn has_item(&self, name: &str) -> bool {
        self.items.binary_search(&name).is_ok()
    }

    /// All blocks that expose a given state property, e.g. "facing".
    pub fn blocks_with_property(&self, property: &str) -> Vec<&'static Block> {
        self.blocks
            .iter()
            .filter(|b| b.properties.iter().any(|p| *p == property))
            .collect()
    }
}

/// The registry for the generated target version.
pub static REGISTRY: Registry = Registry::new(generated::blocks::BLOCKS, generated::items::ITEMS);

/// Version stamp written by the generator.
pub mod meta {
    include!("generated/build.rs");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use generated::models::MODELS;

    #[test]
    fn registry_is_sorted_and_searchable() {
        let blocks = generated::blocks::BLOCKS;
        assert!(!blocks.is_empty(), "generated registry must not be empty");
        for w in blocks.windows(2) {
            assert!(w[0].name < w[1].name, "registry not sorted: {} !< {}", w[0].name, w[1].name);
        }
    }

    #[test]
    fn common_blocks_exist() {
        assert!(REGISTRY.find_block("stone").is_some());
        assert!(REGISTRY.find_block("grass_block").is_some());
    }

    #[test]
    fn unknown_block_returns_none() {
        assert!(REGISTRY.find_block("not_a_real_block_xyz").is_none());
    }

    #[test]
    fn mob_effect_name_resolves_the_wire_order() {
        // the 0-based wire order's ends and one middle pin (the same
        // slots the client's night-vision const test pins — one truth)
        assert_eq!(mob_effect_name(0), Some("minecraft:speed"));
        assert_eq!(mob_effect_name(15), Some("minecraft:night_vision"));
        assert_eq!(mob_effect_name(28), Some("minecraft:conduit_power"));
        assert_eq!(mob_effect_name(39), Some("minecraft:breath_of_the_nautilus"));
        // the boundaries refuse: negative (the decode's own lane
        // refuses it too) and past-the-table (a newer server's
        // registry; vanilla's byId THROWS there — None is the honest
        // unknown, never a guessed name)
        assert_eq!(mob_effect_name(-1), None);
        assert_eq!(mob_effect_name(40), None);
        assert_eq!(mob_effect_name(i32::MAX), None);
        // every table slot resolves, and no two slots share a name —
        // the id->name lane is a FUNCTION (the resolution a debug
        // surface prints can never collide)
        let mut names: Vec<&str> = (0..40i32).filter_map(mob_effect_name).collect();
        assert_eq!(names.len(), 40);
        names.sort_unstable();
        let dup_count = names.windows(2).filter(|w| w[0] == w[1]).count();
        assert_eq!(dup_count, 0, "the name lane is injective");
    }

    #[test]
    fn item_name_resolves_the_registry_order() {
        // the WIRE order's ends and the chest screen's own pins (the ids
        // the deterministic loadout rides — one truth for both sides).
        // The pins are the DumpItemOrder dump's own values (the built-in
        // registration order — NOT the assets table's alphabetical one)
        assert_eq!(item_name(0), Some("air"));
        assert_eq!(item_name(921), Some("apple"));
        assert_eq!(item_name(926), Some("diamond"));
        assert_eq!(item_name(932), Some("iron_ingot"));
        assert_eq!(item_name(974), Some("stick"));
        // the boundaries refuse: negative and past-the-table — None is
        // the honest unknown (the decode's byIdOrThrow THROWS there)
        assert_eq!(item_name(-1), None);
        let n = generated::item_order::ITEM_WIRE_ORDER.len() as i32;
        assert_eq!(item_name(n), None);
        assert_eq!(item_name(i32::MAX), None);
        // every entry resolves, and no two ids share a name — the
        // id->name lane is a FUNCTION (the registry's own dedup)
        let mut names: Vec<&str> = (0..n).filter_map(item_name).collect();
        assert_eq!(names.len(), n as usize);
        names.sort_unstable();
        let dup_count = names.windows(2).filter(|w| w[0] == w[1]).count();
        assert_eq!(dup_count, 0, "the item name lane is injective");
    }

    #[test]
    fn item_model_path_resolves_the_plain_defs() {
        // the plain minecraft:model roots resolve to the bake's own
        // vocabulary (the namespace stripped): the diamond's wire id
        // 926 -> "item/diamond", the shape posed_item_quads walks
        assert_eq!(item_model_path(926), Some("item/diamond"));
        assert_eq!(item_model_path(921), Some("item/apple"));
        // air's own plain def (the registry's first entry)
        assert_eq!(item_model_path(0), Some("item/air"));
        // the complex-definition trees are the honest None on the PLAIN
        // accessor (the 84 resolved + 63 special live on their own lanes)
        let bow = generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == "bow")
            .expect("the bow rides the wire table") as i32;
        assert_eq!(item_model_path(bow), None, "the complex tree is the None");
        // the boundaries refuse like item_name's own
        assert_eq!(item_model_path(-1), None);
        let n = generated::item_order::ITEM_WIRE_ORDER.len() as i32;
        assert_eq!(item_model_path(n), None);
    }

    #[test]
    fn item_rarity_resolves_the_jar_laws() {
        // the wire ids the vocabulary names (the position IS the id)
        let id_of = |name: &str| {
            generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect(name) as i32
        };
        // the NO-rarity registrations the COMMON default keeps (the
        // Item.Properties field init the jar's own construction speaks)
        assert_eq!(item_rarity(id_of("diamond_sword")), Some(0));
        assert_eq!(item_rarity_color(id_of("diamond_sword")), Some(0xFFFFFF));
        assert_eq!(item_rarity(id_of("golden_apple")), Some(0),
            "the plain golden apple wears NO rarity call — COMMON, never guessed");
        // the direct static-init rarity calls the javap walk paired
        assert_eq!(item_rarity(id_of("enchanted_golden_apple")), Some(2));
        assert_eq!(item_rarity_color(id_of("enchanted_golden_apple")), Some(0x55FFFF));
        assert_eq!(item_rarity(id_of("elytra")), Some(3));
        assert_eq!(item_rarity_color(id_of("elytra")), Some(0xFF55FF));
        assert_eq!(item_rarity(id_of("chainmail_helmet")), Some(1));
        assert_eq!(item_rarity_color(id_of("chainmail_helmet")), Some(0xFFFF55));
        assert_eq!(item_rarity(id_of("trident")), Some(2));
        assert_eq!(item_rarity(id_of("command_block")), Some(3));
        // the LAMBDA customization lane (the BootstrapMethods mapping):
        // lambda$static$21 = EPIC rides LIGHT, lambda$static$30 =
        // UNCOMMON rides SNIFFER_EGG
        assert_eq!(item_rarity(id_of("light")), Some(3));
        assert_eq!(item_rarity(id_of("sniffer_egg")), Some(1));
        // the boundaries refuse like item_name's own
        assert_eq!(item_rarity(-1), None);
        assert_eq!(item_rarity_color(-1), None);
        let n = generated::item_order::ITEM_WIRE_ORDER.len() as i32;
        assert_eq!(item_rarity(n), None);
        // the table's own shape: sorted by name (the binary-search law)
        // and every row resolves through the wire vocabulary
        let rows = generated::item_rarity::ITEM_RARITY;
        assert!(rows.windows(2).all(|w| w[0].0 < w[1].0), "sorted for the search");
        for (name, _) in rows {
            assert!(
                generated::item_order::ITEM_WIRE_ORDER.iter().any(|&n| n == name),
                "the rarity row {name} rides the wire vocabulary",
            );
        }
    }

    #[test]
    fn item_icon_resolves_the_flat_gui_layer() {
        // the diamond's flat 16x16 icon: present, correctly sized, and
        // NOT blank (the cyan rides) — resolved through the WIRE id
        let (rgba, w, h) = item_icon(926).expect("diamond has a flat icon");
        assert_eq!((w, h), (16, 16));
        assert_eq!(rgba.len(), 16 * 16 * 4);
        assert!(rgba.iter().any(|&b| b != 0), "the icon is not blank");
        // the unresolvable refuses: the honest unknown draws nothing
        assert!(item_icon(-1).is_none());
        assert!(item_icon(i32::MAX).is_none());
    }

    #[test]
    fn built_in_dimension_types_carry_the_jar_layout() {
        use generated::worldgen::{dimension_type, Skybox, DIMENSION_TYPES};
        assert_eq!(DIMENSION_TYPES.len(), 4);
        let ow = dimension_type("minecraft:overworld").expect("overworld");
        assert_eq!((ow.min_y, ow.height, ow.logical_height), (-64, 384, 384));
        assert!(ow.has_skylight && !ow.has_fixed_time);
        assert_eq!(ow.timelines, Some("#minecraft:in_overworld"));
        assert_eq!(ow.default_clock, Some("minecraft:overworld"));
        // the skybox column: the overworld pair states NO key — the codec
        // default OVERWORLD (optionalFieldOf bytecode); the end states
        // "end", the nether states "none" (the raw JSONs read first-hand)
        assert_eq!(ow.skybox, Skybox::Overworld);
        let caves = dimension_type("minecraft:overworld_caves").expect("caves");
        assert_eq!(caves.skybox, Skybox::Overworld);
        let end = dimension_type("minecraft:the_end").expect("end");
        assert_eq!(end.skybox, Skybox::End);
        let nether = dimension_type("minecraft:the_nether").expect("nether");
        // the values the protocol tests pin for the synced registry: floor 0,
        // 256 tall — and the ceiling keeps logical_height BELOW height
        assert_eq!((nether.min_y, nether.height, nether.logical_height), (0, 256, 128));
        assert!(nether.has_fixed_time && !nether.has_skylight);
        assert_eq!((nether.ambient_light, nether.coordinate_scale), (0.1, 8.0));
        assert_eq!(nether.default_clock, None, "the nether names no default clock");
        assert_eq!(nether.skybox, Skybox::None);
    }

    #[test]
    fn the_day_timeline_keyframes_come_from_the_jar() {
        use generated::worldgen::{timeline, timeline_tag, TIMELINES, TIMELINE_TAGS};
        assert_eq!(TIMELINES.len(), 4);
        assert_eq!(TIMELINE_TAGS.len(), 4);
        let day = timeline("minecraft:day").expect("day timeline");
        assert_eq!(day.clock, "minecraft:overworld");
        assert_eq!(day.period_ticks, Some(24_000));
        assert_eq!(day.tracks, 18);
        assert_eq!(
            day.sky_light_factor,
            [(730, 1.0), (11_270, 1.0), (13_140, 0.24), (22_860, 0.24)]
        );
        // the other real timelines carry no render track — empty, not invented
        let moon = timeline("minecraft:moon").expect("moon timeline");
        assert_eq!((moon.period_ticks, moon.tracks), (Some(192_000), 2));
        assert!(moon.sky_light_factor.is_empty());
        // the tag the overworld dimension type names resolves through the table
        let tag = timeline_tag("minecraft:in_overworld").expect("in_overworld tag");
        assert!(tag.contains(&"minecraft:day"));
        assert!(tag.contains(&"#minecraft:universal"), "nested tag refs ride verbatim");
    }

    /// The timeline tags flatten through the SAME resolver the biome
    /// tags use (one generic core, two validated tables): in_overworld's
    /// FIRST member is the nested "#minecraft:universal", so universal's
    /// expansion lands AT THE HEAD, then the tag's own concrete members
    /// in file order; the end/nether tags are the single-ref form and
    /// build to universal's flatten exactly.
    #[test]
    fn the_timeline_tags_flatten_through_the_universal_ref() {
        assert_eq!(
            flatten_timeline_tag("minecraft:in_overworld").expect("in_overworld"),
            vec![
                "minecraft:villager_schedule",
                "minecraft:day",
                "minecraft:moon",
                "minecraft:early_game",
            ]
        );
        let universal = flatten_timeline_tag("minecraft:universal").expect("universal");
        assert_eq!(universal, vec!["minecraft:villager_schedule"]);
        assert_eq!(
            flatten_timeline_tag("minecraft:in_end").expect("in_end"),
            universal
        );
        assert_eq!(
            flatten_timeline_tag("minecraft:in_nether").expect("in_nether"),
            universal
        );
        // the same refusals ride the timeline wrapper: an unknown tag and
        // the raw "#"-carrying form (the table's keys never carry it)
        assert_eq!(flatten_timeline_tag("minecraft:nope"), None);
        assert_eq!(flatten_timeline_tag("#minecraft:universal"), None);
    }

    /// The end-to-end load property over the jar's real timeline table:
    /// all four tags build, none empty, and every flattened member
    /// resolves into the TIMELINES registry (the registry-bound load
    /// validation the wrapper performs).
    #[test]
    fn every_timeline_tag_flattens_to_registered_timelines() {
        use generated::worldgen::{timeline, TIMELINE_TAGS};
        for (tag, _) in TIMELINE_TAGS {
            let flat = flatten_timeline_tag(tag)
                .unwrap_or_else(|| panic!("{tag} must build"));
            assert!(!flat.is_empty(), "{tag} must not build empty");
            for m in &flat {
                assert!(timeline(m).is_some(), "{tag} built dangling member {m}");
            }
        }
    }

    #[test]
    fn the_player_skin_arm_geometry_is_the_vanilla_unwrap() {
        use generated::skin::{
            ARM_BOX_PX, ARM_FACES, ARM_TEX_OFFS, SKIN_SIDE, STEVE_WIDE_PNG_SHA1,
            STEVE_WIDE_RGBA_SHA1,
        };
        // the classic skin layout
        assert_eq!(SKIN_SIDE, 64);
        // vanilla PlayerModel (wide model): texOffs (40, 16), box 4x12x4
        assert_eq!(ARM_TEX_OFFS, (40, 16));
        assert_eq!(ARM_BOX_PX, [4, 12, 4]);
        // the six faces unfold exactly like vanilla ModelPart's cube unwrap:
        // top, bottom, then the side strip right -> front -> left -> back
        let (u, v) = ARM_TEX_OFFS;
        let [w, h, d] = ARM_BOX_PX;
        let expected = [
            (u + d, v, w, d),
            (u + d + w, v, w, d),
            (u, v + d, d, h),
            (u + d, v + d, w, h),
            (u + d + w, v + d, d, h),
            (u + d + w + d, v + d, w, h),
        ];
        for (f, e) in ARM_FACES.iter().zip(expected) {
            assert_eq!((f.u, f.v, f.w, f.h), e, "arm unwrap must be the vanilla one");
        }
        // every face stays inside the 64x64 texture
        for f in ARM_FACES.iter() {
            assert!(
                u32::from(f.u) + u32::from(f.w) <= SKIN_SIDE
                    && u32::from(f.v) + u32::from(f.h) <= SKIN_SIDE,
                "face {:?} escapes the skin texture",
                f
            );
        }
        // the source is the REAL 26.2 jar entry, pinned at BOTH layers: the
        // verbatim PNG bytes and the generator's decode of them (independently
        // cross-checked against a second decoder over all 64*64*4 bytes).
        assert_eq!(
            STEVE_WIDE_PNG_SHA1,
            "a030e0de66508e6b21e44ede14cbf71b68edf68c"
        );
        assert_eq!(
            STEVE_WIDE_RGBA_SHA1,
            "e2683b8b87d3a671d54c49377f8bcee89eeb34bf"
        );
    }

    #[test]
    fn the_wire_order_tables_match_the_built_in_tables() {
        use generated::registry_order::{BIOME_ORDER, DIMENSION_TYPE_ORDER, TIMELINE_ORDER};
        use generated::worldgen::{BIOMES, DIMENSION_TYPES, TIMELINES};
        // the evidence layer (the bootstrapped vanilla runtime dump) and the
        // data layer (the generated jar tables) must name the SAME members:
        // a claim built on an order table whose set drifts from the data
        // table would map wire ids onto wrong entries
        let same_set = |order: &[&str], table: Vec<&str>| {
            let mut a: Vec<&str> = order.to_vec();
            a.sort();
            let mut b = table;
            b.sort();
            a == b
        };
        assert!(same_set(&DIMENSION_TYPE_ORDER, DIMENSION_TYPES.iter().map(|d| d.name).collect()));
        assert!(same_set(&TIMELINE_ORDER, TIMELINES.iter().map(|t| t.name).collect()));
        assert!(same_set(&BIOME_ORDER, BIOMES.iter().map(|b| b.name).collect()));
        // the world_clock twin: the 26.2 data files are EMPTY objects, so
        // the names ARE the data — and they must name the same member set
        // the order table does (the skip-sync day-tick path maps a clock
        // name to its wire id through that order)
        assert!(same_set(
            &crate::generated::registry_order::WORLD_CLOCK_ORDER,
            generated::worldgen::WORLD_CLOCKS.to_vec()
        ));
        assert_eq!(
            generated::worldgen::WORLD_CLOCKS,
            ["minecraft:overworld", "minecraft:the_end"]
        );
        // the evidence pins: the real boot's first wire ids
        assert_eq!(DIMENSION_TYPE_ORDER[0], "minecraft:overworld");
        assert_eq!(TIMELINE_ORDER[0], "minecraft:day");
        // no duplicates inside an order table (dense, unique wire ids)
        for order in [&DIMENSION_TYPE_ORDER[..], &TIMELINE_ORDER[..], &BIOME_ORDER[..]] {
            let mut sorted = order.to_vec();
            sorted.sort();
            sorted.dedup();
            assert_eq!(sorted.len(), order.len());
        }
    }

    #[test]
    fn the_temperature_modifier_the_generated_rows_own_word() {
        use generated::worldgen::{biome, BIOMES};
        // the 26.2 jar speaks exactly TWO carriers of the optional
        // top-level `temperature_modifier` codec field (the frozen
        // oceans); the other 64 rows the field's absence — the codec's
        // NONE default the None word wears
        let frozen: Vec<&str> = BIOMES
            .iter()
            .filter(|b| b.temperature_modifier.is_some())
            .map(|b| b.name)
            .collect();
        assert_eq!(frozen, ["minecraft:deep_frozen_ocean", "minecraft:frozen_ocean"]);
        assert_eq!(
            biome("minecraft:frozen_ocean").unwrap().temperature_modifier,
            Some("frozen")
        );
        assert_eq!(
            biome("minecraft:deep_frozen_ocean").unwrap().temperature_modifier,
            Some("frozen")
        );
        assert_eq!(
            biome("minecraft:plains").unwrap().temperature_modifier,
            None,
            "the absence IS the codec's default — no invented word"
        );
        assert_eq!(
            biome("minecraft:snowy_plains").unwrap().temperature_modifier,
            None,
            "the cold biomes' cold IS the plain temperature — the modifier never rides"
        );
    }

    #[test]
    fn the_tint_extras_join_the_biome_table_with_the_jar_values() {
        use generated::worldgen::{biome, biome_tint_extra, BIOME_TINT_EXTRAS};
        // the four real 26.2 carriers, verbatim from the biome JSONs
        // (effects.grass_color_modifier + effects.dry_foliage_color)
        assert_eq!(BIOME_TINT_EXTRAS.len(), 4);
        let dark = biome_tint_extra("minecraft:dark_forest").expect("dark_forest extras");
        assert_eq!(dark.grass_color_modifier, Some("dark_forest"));
        assert_eq!(dark.dry_foliage_color, Some(0x7B5334));
        let swamp = biome_tint_extra("minecraft:swamp").expect("swamp extras");
        assert_eq!(swamp.grass_color_modifier, Some("swamp"));
        assert_eq!(swamp.dry_foliage_color, Some(0x7B5334));
        let mangrove = biome_tint_extra("minecraft:mangrove_swamp").expect("mangrove extras");
        assert_eq!(mangrove.grass_color_modifier, Some("swamp"));
        let pale = biome_tint_extra("minecraft:pale_garden").expect("pale garden extras");
        assert_eq!(pale.grass_color_modifier, None);
        assert_eq!(pale.dry_foliage_color, Some(0xA0A69C));
        assert_eq!(biome_tint_extra("minecraft:plains"), None, "sparse: carriers only");
        // the join must never dangle: every extras entry names a real biome
        for e in &BIOME_TINT_EXTRAS {
            assert!(
                biome(e.biome).is_some(),
                "extras entry {} has no biome-table twin",
                e.biome
            );
        }
    }

    #[test]
    fn the_biome_tags_join_the_biome_table_and_never_dangle() {
        use generated::worldgen::{biome, biome_tag, BIOME_TAGS};
        // the jar's data/minecraft/tags/worldgen/biome/ file set, verbatim:
        // 68 tags / 390 members, no "replace" key anywhere
        assert_eq!(BIOME_TAGS.len(), 68);
        let total: usize = BIOME_TAGS.iter().map(|t| t.1.len()).sum();
        assert_eq!(total, 390, "every member the jar states is emitted");
        // is_forest's members verbatim, in the jar's stored order
        let forest = biome_tag("minecraft:is_forest").expect("is_forest");
        assert_eq!(
            forest,
            &[
                "minecraft:forest",
                "minecraft:flower_forest",
                "minecraft:birch_forest",
                "minecraft:old_growth_birch_forest",
                "minecraft:dark_forest",
                "minecraft:pale_garden",
                "minecraft:grove",
            ]
        );
        // a nested reference rides verbatim and names a KNOWN tag (the
        // structure tags live in a SUBDIRECTORY — the stem carries the
        // slash: tags/worldgen/biome/has_structure/buried_treasure.json)
        let buried = biome_tag("minecraft:has_structure/buried_treasure")
            .expect("has_structure/buried_treasure");
        assert_eq!(buried[0], "#minecraft:is_beach");
        assert!(biome_tag("#minecraft:is_beach").is_none());
        // THE JOIN NEVER DANGLES: every concrete member names a biome the
        // table carries; every nested ref names a tag the table carries
        for (tag, members) in &BIOME_TAGS {
            for m in members.iter() {
                if let Some(id) = m.strip_prefix('#') {
                    assert!(
                        biome_tag(id).is_some(),
                        "tag {tag} references unknown tag {id}"
                    );
                } else {
                    assert!(
                        biome(m).is_some(),
                        "tag {tag} references unknown biome {m}"
                    );
                }
            }
        }
    }

    #[test]
    fn the_flat_tags_flatten_to_their_own_file_order() {
        // a tag with only concrete members builds to ITS OWN slice,
        // verbatim and in order — no expansion, no reorder
        let forest = flatten_biome_tag("minecraft:is_forest").expect("is_forest");
        assert_eq!(
            forest,
            vec![
                "minecraft:forest",
                "minecraft:flower_forest",
                "minecraft:birch_forest",
                "minecraft:old_growth_birch_forest",
                "minecraft:dark_forest",
                "minecraft:pale_garden",
                "minecraft:grove",
            ]
        );
        assert_eq!(
            flatten_biome_tag("minecraft:is_beach").expect("is_beach"),
            vec!["minecraft:beach", "minecraft:snowy_beach"]
        );
        // is_overworld: the largest flat tag — zero nested refs, every
        // member concrete, the file order preserved end to end (the
        // flattened set IS the raw slice for a flat tag, any size)
        let overworld = flatten_biome_tag("minecraft:is_overworld").expect("is_overworld");
        let raw = generated::worldgen::biome_tag("minecraft:is_overworld").expect("raw");
        assert_eq!(overworld, raw.to_vec());
        assert_eq!(overworld[0], "minecraft:mushroom_fields");
        // the dimension categories stay disjoint: no nether or end biome
        // leaks into the overworld set (a flat tag's flatten adds nothing)
        for nether_member in generated::worldgen::biome_tag("minecraft:is_nether").unwrap() {
            assert!(!overworld.contains(nether_member));
        }
        for end_member in generated::worldgen::biome_tag("minecraft:is_end").unwrap() {
            assert!(!overworld.contains(end_member));
        }
    }

    #[test]
    fn the_nested_reference_expands_depth_first_in_place() {
        // is_ocean's FIRST entry is the nested "#minecraft:is_deep_ocean":
        // the four deep oceans expand AT THE HEAD, then the tag's own five
        // concrete members follow — the built order is file order with
        // refs expanded at their position
        assert_eq!(
            flatten_biome_tag("minecraft:is_ocean").expect("is_ocean"),
            vec![
                "minecraft:deep_frozen_ocean",
                "minecraft:deep_cold_ocean",
                "minecraft:deep_ocean",
                "minecraft:deep_lukewarm_ocean",
                "minecraft:frozen_ocean",
                "minecraft:ocean",
                "minecraft:cold_ocean",
                "minecraft:lukewarm_ocean",
                "minecraft:warm_ocean",
            ]
        );
        // TWO-LEVEL recursion through one ref: a tag whose only member is
        // "#minecraft:is_ocean" builds to exactly is_ocean's flatten —
        // the grandparent sees the deep expansion transitively
        let ocean = flatten_biome_tag("minecraft:is_ocean").expect("is_ocean again");
        assert_eq!(
            flatten_biome_tag("minecraft:has_structure/ruined_portal_ocean").expect("rpo"),
            ocean
        );
        assert_eq!(
            flatten_biome_tag("minecraft:has_structure/shipwreck").expect("shipwreck"),
            ocean
        );
        // the single-concrete tag builds to its one member
        assert_eq!(
            flatten_biome_tag("minecraft:has_structure/nether_fossil").expect("nether_fossil"),
            vec!["minecraft:soul_sand_valley"]
        );
    }

    #[test]
    fn the_synthetics_pin_the_dedup_and_the_cycle_rules() {
        // the real 68-tag table has no cycle and no cross-path repeat, so
        // the two refusal/dedup rules pin on a synthetic table through the
        // generic core (the public wrapper additionally validates against
        // the real biome table, which a synthetic member cannot pass)
        let table: &[(&str, &[&str])] = &[
            // "x" is reachable three ways: directly, through b, and again
            // directly — it resolves ONCE, at the first path (the built
            // set's first-occurrence order)
            ("minecraft:a", &["minecraft:x", "#minecraft:b", "minecraft:x"]),
            ("minecraft:b", &["minecraft:x", "minecraft:y"]),
        ];
        assert_eq!(
            flatten_tag_table(table, "minecraft:a").expect("a"),
            vec!["minecraft:x", "minecraft:y"]
        );
        // a two-tag cycle and a direct self-reference both refuse to
        // build — vanilla refuses the datapack, the resolver fails closed
        let cyclic: &[(&str, &[&str])] = &[
            ("minecraft:c", &["#minecraft:d"]),
            ("minecraft:d", &["#minecraft:c"]),
        ];
        assert_eq!(flatten_tag_table(cyclic, "minecraft:c"), None);
        let selfref: &[(&str, &[&str])] = &[("minecraft:s", &["#minecraft:s"])];
        assert_eq!(flatten_tag_table(selfref, "minecraft:s"), None);
        // an unknown nested reference and an unknown tag refuse too
        let dangling: &[(&str, &[&str])] = &[("minecraft:u", &["#minecraft:missing"])];
        assert_eq!(flatten_tag_table(dangling, "minecraft:u"), None);
        assert_eq!(flatten_tag_table(table, "minecraft:nope"), None);
        // the same refusals ride the public wrapper on the REAL table
        assert_eq!(flatten_biome_tag("minecraft:nope"), None);
        assert_eq!(flatten_biome_tag("#minecraft:is_beach"), None);
    }

    #[test]
    fn every_real_tag_flattens_to_registered_biomes() {
        // the end-to-end load property over the jar's real table: all 68
        // tags build, none empty, and every flattened member resolves
        // into the biome registry (the registry-bound load validation the
        // wrapper performs — here asserted member-by-member so a failure
        // NAMES the dangling member instead of failing the whole sweep)
        use generated::worldgen::{biome, BIOME_TAGS};
        for (tag, _) in BIOME_TAGS {
            let flat = flatten_biome_tag(tag)
                .unwrap_or_else(|| panic!("{tag} must build"));
            assert!(!flat.is_empty(), "{tag} must not build empty");
            for m in &flat {
                assert!(biome(m).is_some(), "{tag} built dangling member {m}");
            }
        }
        // the two-level sum sanity: the flattened ocean family (9) rides
        // under every is_ocean-referencing structure tag verbatim
        assert_eq!(
            flatten_biome_tag("minecraft:required_ocean_monument_surrounding")
                .expect("monument"),
            [&
                flatten_biome_tag("minecraft:is_ocean").expect("ocean")[..],
                &flatten_biome_tag("minecraft:is_river").expect("river")[..],
            ]
            .concat()
        );
    }

    #[test]
    fn the_item_model_definitions_resolve_and_the_joins_hold() {
        use generated::item_models::{
            COMPLEX_ITEM_MODELS, ITEM_MODEL_DEFS, ITEM_MODEL_ROOT_TYPES, ITEM_SPECIAL_MODELS,
            RESOLVED_ITEM_LAYERS,
        };
        use generated::items::ITEMS;
        // the jar's assets/minecraft/items/ file set, verbatim: 1537
        // definitions split 1390 plain minecraft:model roots + 84 complex
        // trees RESOLVED to resting-hand layers + 63 special renderers
        // RESOLVED to the LayerDefinitions dump's payloads — the refuse
        // list is EMPTY (the v0.3.141 arc: every tree resolves or names
        // its renderer, none refuse)
        assert_eq!(ITEM_MODEL_DEFS.len(), 1390);
        assert_eq!(RESOLVED_ITEM_LAYERS.len(), 84);
        assert_eq!(ITEM_SPECIAL_MODELS.len(), 63);
        assert_eq!(COMPLEX_ITEM_MODELS.len(), 0);
        assert_eq!(
            ITEM_MODEL_DEFS.len() + RESOLVED_ITEM_LAYERS.len() + ITEM_SPECIAL_MODELS.len(),
            1537
        );
        // the lanes are disjoint: no id in two tables
        for (id, _) in ITEM_SPECIAL_MODELS {
            assert!(
                ITEM_MODEL_DEFS.iter().all(|d| d.0 != *id)
                    && RESOLVED_ITEM_LAYERS.iter().all(|d| d.0 != *id),
                "{id} rides two tables"
            );
        }
        let census: u32 = ITEM_MODEL_ROOT_TYPES.iter().map(|t| t.1).sum();
        assert_eq!(census, 1537, "the root-type census accounts for every item");
        assert_eq!(
            &ITEM_MODEL_ROOT_TYPES,
            &[
                ("minecraft:model", 1390),
                ("minecraft:select", 71),
                ("minecraft:special", 51),
                ("minecraft:composite", 16),
                ("minecraft:condition", 7),
                ("minecraft:range_dispatch", 2),
            ]
        );

        // verbatim spot rows — the jar's own stored strings
        let def = |id: &str| {
            ITEM_MODEL_DEFS
                .iter()
                .find(|d| d.0 == id)
                .unwrap_or_else(|| panic!("{id} must be a plain def"))
        };
        let stone = def("stone");
        assert_eq!(stone.1, "minecraft:block/stone");
        assert!(stone.2.is_empty());
        assert!(stone.3.is_none());
        assert_eq!(def("dirt").1, "minecraft:block/dirt");
        // the fence points at its INVENTORY model variant, verbatim
        assert_eq!(def("acacia_fence").1, "minecraft:block/acacia_fence_inventory");
        // grass_block: the one grass tint, canonical compact JSON, values verbatim
        let grass = def("grass_block");
        assert_eq!(grass.1, "minecraft:block/grass_block");
        assert_eq!(
            grass.2,
            &["{\"downfall\":1.0,\"temperature\":0.5,\"type\":\"minecraft:grass\"}"]
        );
        // filled_map rides TWO tints (constant + map_color), in jar order
        let map = def("filled_map");
        assert_eq!(map.1, "minecraft:item/filled_map");
        assert_eq!(
            map.2,
            &[
                "{\"type\":\"minecraft:constant\",\"value\":-1}",
                "{\"default\":4603950,\"type\":\"minecraft:map_color\"}",
            ]
        );
        // the swap-animation scale ships ONLY beside the spears' select
        // trees — zero plain def carries it (the column exists so the
        // parser drops nothing the jar states)
        assert!(ITEM_MODEL_DEFS.iter().all(|d| d.3.is_none()));
        for spear in [
            "wooden_spear",
            "stone_spear",
            "iron_spear",
            "golden_spear",
            "diamond_spear",
            "netherite_spear",
            "copper_spear",
        ] {
            assert!(ITEM_MODEL_DEFS.iter().all(|d| d.0 != spear));
            // the spears' select trees RESOLVE through the fallback (the
            // hand model + the 1.95 swap) — they left the skip list
            assert!(RESOLVED_ITEM_LAYERS.iter().any(|d| d.0 == spear), "{spear} resolved");
            assert_eq!(RESOLVED_ITEM_LAYERS.iter().find(|d| d.0 == spear).unwrap().1, Some(1.95));
        }
        // the shield RESOLVES to the special renderer — the using_item
        // condition's resting (on_false) branch (the blocking branch's
        // mirror-flip transform rode the same payload's transform)
        let shield = ITEM_SPECIAL_MODELS.iter().find(|(id, _)| *id == "shield").unwrap().1;
        assert_eq!(shield.base, "minecraft:item/shield");
        assert_eq!(shield.layers[0].layer, "minecraft:shield#main");
        // every special payload's layer ids are dump layer ids (the join
        // against generated::special_models never dangles)
        for (id, def) in ITEM_SPECIAL_MODELS {
            for l in def.layers {
                assert!(
                    generated::special_models::SPECIAL_LAYERS.iter().any(|(lid, _)| *lid == l.layer),
                    "{id}: layer {} absent from the dump", l.layer
                );
            }
        }

        // every model path is namespaced minecraft: under block/ or item/
        let (mut block, mut item) = (0usize, 0usize);
        for (id, path, _, _) in &ITEM_MODEL_DEFS {
            let rest = path
                .strip_prefix("minecraft:")
                .unwrap_or_else(|| panic!("{id}: unnamespaced model path {path}"));
            match rest.split('/').next().unwrap() {
                "block" => block += 1,
                "item" => item += 1,
                other => panic!("{id}: model path under unexpected prefix {other}/"),
            }
        }
        assert_eq!((block, item), (731, 659), "the target split the jar states");

        // THE JOINS NEVER DANGLE: every plain def, every resolved layer
        // and every refused item names an item the registry table carries
        for (id, _, _, _) in &ITEM_MODEL_DEFS {
            assert!(ITEMS.contains(id), "def {id} names no known item");
        }
        for (id, _, layers) in RESOLVED_ITEM_LAYERS {
            assert!(ITEMS.contains(&id), "resolved {id} names no known item");
            // the namespace-stripped paths must resolve in the MODELS table
            for layer in layers.iter() {
                let rest = layer.model.strip_prefix("minecraft:").unwrap_or(layer.model);
                assert!(
                    MODELS.iter().any(|m| m.0 == rest),
                    "{id}: layer model {} names no known model",
                    layer.model
                );
            }
        }
        for (id, _) in COMPLEX_ITEM_MODELS {
            assert!(ITEMS.contains(id), "complex {id} names no known item");
        }
    }

    #[test]
    fn the_layer_accessor_walks_the_resting_hand_tables() {
        // a plain def: the single layer, no transform (the diamond hand)
        let diamond_id = generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|n| *n == "diamond")
            .unwrap() as i32;
        let layers = item_model_layers(diamond_id).expect("diamond resolves");
        assert_eq!(layers.len(), 1);
        assert_eq!(layers[0].model, "item/diamond");
        assert!(layers[0].transform.is_none());

        // the bow: the condition's on_false branch through the resolved
        // table — the plain bow model (the resting frame, not pulling)
        let bow_id = generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|n| *n == "bow")
            .unwrap() as i32;
        let layers = item_model_layers(bow_id).expect("bow resolves now");
        assert_eq!(layers.len(), 1);
        assert_eq!(layers[0].model, "item/bow");

        // the bed: the composite's two layers, the foot's decoded z=1
        // translation (the compose bytecode's T·L·S·R, identity rotations)
        let bed_id = generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|n| *n == "black_bed")
            .unwrap() as i32;
        let layers = item_model_layers(bed_id).expect("the bed resolves");
        assert_eq!(layers.len(), 2);
        assert_eq!(layers[0].model, "block/black_bed_head");
        assert!(layers[0].transform.is_none());
        assert_eq!(layers[1].model, "block/black_bed_foot");
        let (t, l, s, r) = layers[1].transform.expect("the foot carries the translation");
        assert_eq!(t, [0.0, 0.0, 1.0]);
        assert_eq!(l, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(s, [1.0, 1.0, 1.0]);
        assert_eq!(r, [0.0, 0.0, 0.0, 1.0]);

        // the shield: the special renderer's honest None (the bare arm)
        let shield_id = generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|n| *n == "shield")
            .unwrap() as i32;
        assert!(item_model_layers(shield_id).is_none(), "no model JSON — no lie");

        // item_model_path keeps its plain-only semantics (the golden
        // offline lane's own vocabulary): the bow stays None there
        assert!(item_model_path(bow_id).is_none());
        assert_eq!(item_model_path(diamond_id), Some("item/diamond"));
    }

    /// THE BROKEN CONDITION'S OWN WALK (javap `Broken.get` =
    /// `stack.nextDamageWillBreak()` verbatim; `nextDamageWillBreak` =
    /// `isDamageableItem() && damage >= maxDamage - 1` — the if_icmplt
    /// threshold the pc 17 crossing speaks): the elytra — the ONLY
    /// root `minecraft:broken` condition the 26.2 items/ tree carries —
    /// swaps to the on_true branch's plain model the torn verdict
    /// wears, the resting arm the false answer keeps, the
    /// non-condition items both verdicts ride the same.
    #[test]
    fn the_broken_condition_the_torn_elytra_wears() {
        let wire = |name: &str| {
            generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|n| *n == name)
                .unwrap() as i32
        };
        // THE ONLY ROOT CONDITION (the one row the table keeps)
        assert_eq!(BROKEN_CONDITION_ON_TRUE.len(), 1);
        let elytra_id = wire("elytra");
        // THE RESTING ARM (the RESOLVED table's on_false branch — the
        // plain elytra model the fresh stack wears)
        let rest = item_model_layers(elytra_id).expect("the elytra resolves");
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].model, "item/elytra");
        // THE TORN ARM (the on_true branch's model the verdict swaps —
        // the plain node, no tints, no transform)
        let torn = item_model_layers_broken(elytra_id, true).expect("the torn elytra resolves");
        assert_eq!(torn.len(), 1);
        assert_eq!(torn[0].model, "item/elytra_broken");
        assert!(torn[0].tints.is_empty());
        assert!(torn[0].transform.is_none());
        // THE RESTING VERDICT THE FALSE ANSWER KEEPS (the plain
        // accessor's own walk)
        let untorn =
            item_model_layers_broken(elytra_id, false).expect("the resting elytra resolves");
        assert_eq!(untorn[0].model, "item/elytra");
        // THE NON-CONDITION ITEM THE VERDICT NEVER MOVES (the plain
        // def's own layers both verdicts speak — the field walk the
        // bare struct's own shape keeps)
        let diamond_id = wire("diamond");
        let resting = item_model_layers(diamond_id).expect("the diamond resolves");
        let broken = item_model_layers_broken(diamond_id, true).expect("the diamond resolves");
        assert_eq!(resting.len(), broken.len(), "the same layer count");
        for (a, b) in resting.iter().zip(broken.iter()) {
            assert_eq!(a.model, b.model, "the plain def both verdicts wear");
            assert_eq!(a.tints.len(), b.tints.len());
            assert_eq!(a.transform.is_some(), b.transform.is_some());
        }
    }

    /// THE USING BRANCH: the five gated items resolve through the
    /// generated table, the charge laws read the javap-pinned property
    /// math, and the payloads carry the JSON's own bases and mirrors.
    #[test]
    fn the_using_branches_resolve_and_the_charge_laws_hold() {
        let wire = |name: &str| {
            generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|n| *n == name)
                .unwrap() as i32
        };

        // the census: exactly the five gated items; everything else None
        assert_eq!(generated::item_models::ITEM_USING_STATES.len(), 5);
        for name in ["bow", "crossbow", "goat_horn", "shield", "trident"] {
            assert!(item_using_def(wire(name)).is_some(), "{name} has a using lane");
        }
        assert!(item_using_def(wire("diamond")).is_none());
        assert!(item_using_def(wire("stone")).is_none());

        // THE BOW: charge = elapsed * 0.05 (javap UseDuration.get, the
        // remaining=false branch — the elapsed ticks the dispatch scales)
        let bow = item_using_def(wire("bow")).unwrap();
        let at = |ticks: u32| using_state_index(bow, using_charge(bow, ticks));
        assert_eq!(at(0), 0, "the rest frame: bow_pulling_0");
        assert_eq!(at(12), 0, "12 * 0.05 = 0.60 < 0.65");
        assert_eq!(at(13), 1, "13 * 0.05 = 0.65 — pulling_1 engages");
        assert_eq!(at(17), 1);
        assert_eq!(at(18), 2, "18 * 0.05 = 0.90 — pulling_2 engages");
        // the payload at the picked index: the pulling sprites
        match item_using_state(wire("bow"), 2) {
            Some(UsingStateRender::Layers(layers)) => {
                assert_eq!(layers.len(), 1);
                assert_eq!(layers[0].model, "item/bow_pulling_2");
                assert!(layers[0].transform.is_none());
            }
            other => panic!("the bow's states are model-path layers, got {other:?}"),
        }

        // THE CROSSBOW: charge = elapsed / 25 (javap CrossbowPull.get x
        // getChargeDuration = Mth.floor(1.25f * 20f))
        let xbow = item_using_def(wire("crossbow")).unwrap();
        let at = |ticks: u32| using_state_index(xbow, using_charge(xbow, ticks));
        assert_eq!(at(14), 0, "14/25 = 0.56 < 0.58");
        assert_eq!(at(15), 1, "15/25 = 0.60 — pulling_1");
        assert_eq!(at(25), 2, "25/25 = 1.0 — pulling_2 (the full charge)");
        assert_eq!(using_charge(xbow, 0), 0.0);

        // THE GOAT HORN: the pure gate, one state, the tooting model
        let horn = item_using_def(wire("goat_horn")).unwrap();
        assert_eq!(horn.property, "");
        match item_using_state(wire("goat_horn"), 0) {
            Some(UsingStateRender::Layers(layers)) => {
                assert_eq!(layers[0].model, "item/tooting_goat_horn");
            }
            other => panic!("the horn's state is a model layer, got {other:?}"),
        }

        // THE SHIELD: the blocking special — the shield_blocking base
        // poses the geometry, the mirror transform rides the local decode
        match item_using_state(wire("shield"), 0) {
            Some(UsingStateRender::Special(render)) => {
                assert_eq!(render.base, "item/shield_blocking");
                let (t, l, s, r) = render.transform.expect("the blocking mirror");
                assert_eq!(t, [0.0, 0.0, 0.0]);
                assert_eq!(l, [0.0, 0.0, 0.0, 1.0]);
                assert_eq!(s, [1.0, -1.0, -1.0], "the JSON's own mirror");
                assert_eq!(r, [0.0, 0.0, 0.0, 1.0]);
                assert_eq!(render.layers.len(), 1);
                assert_eq!(render.layers[0].texture, "entity/shield/shield_base_nopattern");
            }
            other => panic!("the shield's state is a special payload, got {other:?}"),
        }

        // THE TRIDENT: the throwing special, the same mirror
        match item_using_state(wire("trident"), 0) {
            Some(UsingStateRender::Special(render)) => {
                assert_eq!(render.base, "item/trident_throwing");
                let (_, _, s, _) = render.transform.expect("the throwing mirror");
                assert_eq!(s, [1.0, -1.0, -1.0]);
            }
            other => panic!("the trident's state is a special payload, got {other:?}"),
        }
    }

    #[test]
    fn the_charge_lane_is_the_crossbows_component_select() {
        use generated::item_models::ITEM_CHARGE_STATES;
        // THE CENSUS: the jar sweep names exactly one charge_type tree
        // (crossbow.json — javap Charge.get reads the CHARGED_PROJECTILES
        // component, the CrossbowItem$ChargeType enum's own property)
        assert_eq!(ITEM_CHARGE_STATES.len(), 1);
        let wire = |name: &str| {
            generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|n| *n == name)
                .map(|p| p as i32)
                .unwrap()
        };
        assert!(item_charge_def(wire("crossbow")).is_some(), "crossbow has a charge lane");
        assert!(item_charge_def(wire("diamond")).is_none(), "no lane, no charge");
        assert!(item_charge_def(wire("stone")).is_none());
        assert!(item_charge_def(wire("bow")).is_none(), "the bow's pull is a using lane, not a charge");

        // THE SELECT CASES: the loaded bolt rides the JSON's own case
        // models (the arrow -> crossbow_arrow, the firework rocket ->
        // crossbow_firework), one layer each, no transforms
        let arrow = item_charge_layers(wire("crossbow"), ChargeKind::Arrow).unwrap();
        assert_eq!(arrow.len(), 1);
        assert_eq!(arrow[0].model, "item/crossbow_arrow");
        assert!(arrow[0].transform.is_none());
        let rocket = item_charge_layers(wire("crossbow"), ChargeKind::Rocket).unwrap();
        assert_eq!(rocket.len(), 1);
        assert_eq!(rocket[0].model, "item/crossbow_firework");

        // THE REVERSE LOOKUP: the wire domain's own round trip
        assert_eq!(item_id_by_name("crossbow"), Some(wire("crossbow")));
        assert_eq!(item_id_by_name("diamond"), Some(wire("diamond")));
        assert_eq!(item_id_by_name("not_an_item"), None);

        // THE CHARGE DURATION: the release law's own constant (javap
        // getChargeDuration = Mth.floor(1.25f * 20f) = 25)
        assert_eq!(CROSSBOW_CHARGE_TICKS, 25);
    }

    #[test]
    fn the_model_table_is_the_jar_census_and_the_joins_hold() {
        use generated::item_models::ITEM_MODEL_DEFS;
        use generated::models::{ELEMENT_MODELS, MODELS};
        // the jar's assets/minecraft/models/ file set, verbatim: 3928 files
        // split 2657 block/ + 1271 item/ — cross-checked against an
        // independent python census (the itemgen dual-census precedent)
        assert_eq!(MODELS.len(), 3928);
        let blocks = MODELS.iter().filter(|m| m.0.starts_with("block/")).count();
        let items = MODELS.iter().filter(|m| m.0.starts_with("item/")).count();
        assert_eq!((blocks, items), (2657, 1271));
        // the element geometry is retained in the honest skip list
        assert_eq!(ELEMENT_MODELS.len(), 344);
        assert_eq!(ELEMENT_MODELS.first(), Some(&"block/bamboo1_age0"));
        assert_eq!(ELEMENT_MODELS.last(), Some(&"item/spyglass_in_hand"));
        // the closed census counts: gui_light carriers and the
        // ambient-occlusion overrides
        assert_eq!(MODELS.iter().filter(|r| r.5.is_some()).count(), 12);
        assert_eq!(MODELS.iter().filter(|r| r.6.is_some()).count(), 157);

        let model = |id: &str| {
            MODELS
                .iter()
                .find(|m| m.0 == id)
                .unwrap_or_else(|| panic!("{id} must be a model"))
        };

        // verbatim spot rows — the jar's own stored strings (canonical
        // compact JSON, serde key order)
        let stone = model("block/stone");
        assert_eq!(stone.1, Some("block/cube_all"), "namespace stripped");
        assert_eq!(stone.2, Some("{\"all\":\"minecraft:block/stone\"}"));
        assert_eq!(stone.4, 0);
        let apple = model("item/apple");
        assert_eq!(apple.1, Some("item/generated"));
        assert_eq!(apple.2, Some("{\"layer0\":\"minecraft:item/apple\"}"));
        let stairs = model("block/oak_stairs");
        assert_eq!(
            stairs.2,
            Some("{\"bottom\":\"minecraft:block/oak_planks\",\"side\":\"minecraft:block/oak_planks\",\"top\":\"minecraft:block/oak_planks\"}")
        );

        // the root display transforms — the held-item consumer's constants
        let block_root = model("block/block");
        assert_eq!(block_root.1, None, "block/block is a chain root");
        assert_eq!(block_root.5, Some("side"));
        let d = block_root.3.unwrap();
        assert!(d.contains("\"gui\":{\"rotation\":[30,225,0],\"scale\":[0.625,0.625,0.625],\"translation\":[0,0,0]}"), "{d}");
        // 26.2's shelf context rides the root
        assert!(d.contains("\"on_shelf\":{\"rotation\":[0,180,0],\"scale\":[1,1,1],\"translation\":[0,0,0]}"), "{d}");
        assert!(!d.contains("thirdperson_lefthand"), "the block root states only the right-hand context");
        let generated = model("item/generated");
        assert_eq!(generated.1, Some("builtin/generated"));
        assert_eq!(generated.5, Some("front"));
        let d = generated.3.unwrap();
        assert!(d.contains("\"firstperson_righthand\":{\"rotation\":[0,-90,25],\"scale\":[0.68,0.68,0.68],\"translation\":[1.13,3.2,1.13]}"), "{d}");
        let handheld = model("item/handheld");
        let d = handheld.3.unwrap();
        assert!(d.contains("\"thirdperson_righthand\":{\"rotation\":[0,-90,55],\"scale\":[0.85,0.85,0.85],\"translation\":[0,4.0,0.5]}"), "{d}");

        // the two outliers the full census caught, verbatim
        let heavy = model("block/heavy_core");
        assert_eq!(heavy.1, None, "heavy_core is its own root");
        assert_eq!(heavy.7, Some((16, 16)), "the element-UV size override");
        let fence = model("block/custom_fence_side_east");
        assert_eq!(
            fence.8,
            Some("[{\"children\":[0,1],\"color\":0,\"name\":\"east\",\"origin\":[0,0,0]}]"),
            "the Blockbench groups metadata is retained, never dropped"
        );
        // the 26.2 object-valued texture entries (redstone/glass) ride through
        let dust = model("block/redstone_dust_dot");
        assert!(dust.2.unwrap().contains("\"force_translucent\":true"));

        // the deepest chain, verbatim: the crafter family down to the root
        let chain = [
            "block/crafter_crafting_triggered",
            "block/crafter_crafting",
            "block/crafter_triggered",
            "block/crafter",
        ];
        for (a, b) in chain.iter().zip(chain.iter().skip(1)) {
            assert_eq!(model(a).1, Some(*b));
        }
        assert_eq!(model("block/crafter").1, Some("block/block"));

        // the parent graph re-verified FROM the emitted table: every parent
        // resolves to a file or a builtin/, acyclic, max depth 4
        let by_id = |id: &str| MODELS.iter().find(|m| m.0 == id);
        let mut max_depth = 0u32;
        for row in MODELS.iter() {
            let id = row.0;
            let mut cur = row.0;
            let mut seen = std::collections::BTreeSet::new();
            let mut depth = 0u32;
            loop {
                let parent = by_id(cur).expect("every walked id exists").1;
                let p = match parent {
                    None => break,
                    Some(p) if p.starts_with("builtin/") => break,
                    Some(p) => p,
                };
                assert!(by_id(p).is_some(), "{id}: dangling parent {p}");
                assert!(seen.insert(cur), "{id}: parent cycle at {cur}");
                cur = p;
                depth += 1;
            }
            max_depth = max_depth.max(depth);
        }
        assert_eq!(max_depth, 4, "the census depth (the crafter chain)");

        // THE JOIN: every plain item def's model path resolves to a file
        for (_, path, _, _) in ITEM_MODEL_DEFS.iter() {
            let bare = path.strip_prefix("minecraft:").unwrap();
            assert!(by_id(bare).is_some(), "def model {bare} resolves to no model file");
        }
        // 24 defs point directly at element-carrying models (beacon,
        // cactus, crafter...) — the consumer round's bridge set
        let elemental: std::collections::BTreeSet<&str> = ELEMENT_MODELS.iter().copied().collect();
        let hits = ITEM_MODEL_DEFS
            .iter()
            .filter(|d| elemental.contains(d.1.strip_prefix("minecraft:").unwrap()))
            .count();
        assert_eq!(hits, 24, "defs pointing directly at element-carrying models");
    }

    #[test]
    fn the_elements_geometry_is_the_jar_census_and_the_joins_hold() {
        use generated::elements::{ELEMENTS, ElementRotation};
        use generated::models::{ELEMENT_MODELS, MODELS};

        // the census, verbatim: 344 element-carrying models, 1185 elements,
        // 4238 faces — the python census, cross-checked by the Rust run
        assert_eq!(ELEMENTS.len(), 344);
        let total: usize = ELEMENTS.iter().map(|(_, e)| e.len()).sum();
        assert_eq!(total, 1185);
        let faces: usize = ELEMENTS
            .iter()
            .flat_map(|(_, e)| e.iter())
            .map(|el| el.faces.iter().flatten().count())
            .sum();
        assert_eq!(faces, 4238);

        // THE JOIN: the ELEMENTS rows match ELEMENT_MODELS id-for-id and the
        // MODELS element-count column row-for-row; the mirror direction too
        let ids: Vec<&str> = ELEMENTS.iter().map(|(id, _)| *id).collect();
        let skip: Vec<&str> = ELEMENT_MODELS.iter().copied().collect();
        assert_eq!(ids, skip);
        for (id, els) in ELEMENTS.iter() {
            let row = MODELS
                .iter()
                .find(|m| m.0 == *id)
                .unwrap_or_else(|| panic!("{id} must be a model"));
            assert_eq!(row.4 as usize, els.len(), "{id}: element-count join");
        }
        for m in MODELS.iter().filter(|m| m.4 > 0) {
            assert!(
                ELEMENTS.iter().any(|(id, _)| *id == m.0),
                "{} carries elements in MODELS but has no ELEMENTS row",
                m.0
            );
        }

        // closed domains across the whole table: bounds inside the vanilla
        // clamp [-16, 32], uvs inside [0, 16], face rotation a multiple of
        // 90 (the census domain {0, 90, 180, 270})
        for (id, els) in ELEMENTS.iter() {
            for el in els.iter() {
                for c in el.from.iter().chain(el.to.iter()) {
                    assert!((-16.0..=32.0).contains(c), "{id}: bound {c}");
                }
                for f in el.faces.iter().flatten() {
                    if let Some(uv) = f.uv {
                        for v in uv {
                            assert!((0.0..=16.0).contains(&v), "{id}: uv {v}");
                        }
                    }
                    if let Some(r) = f.rotation {
                        assert_eq!(r % 90, 0, "{id}: face rotation {r}");
                    }
                }
            }
        }

        // block/cube — the six-face box, verbatim (uv: None = the vanilla
        // bounds projection; cullface mirrors each slot's direction)
        let cube = ELEMENTS
            .iter()
            .find(|(id, _)| *id == "block/cube")
            .unwrap()
            .1;
        assert_eq!(cube.len(), 1);
        let c0 = &cube[0];
        assert_eq!(c0.from, [0.0, 0.0, 0.0]);
        assert_eq!(c0.to, [16.0, 16.0, 16.0]);
        assert!(c0.shade && c0.rotation.is_none() && c0.light_emission.is_none());
        assert_eq!(c0.name, None);
        let refs = ["#down", "#up", "#north", "#south", "#west", "#east"];
        for (slot, face) in c0.faces.iter().enumerate() {
            let f = face
                .as_ref()
                .unwrap_or_else(|| panic!("cube face {slot} missing"));
            assert_eq!(f.uv, None, "the cube ships default uvs");
            assert_eq!(f.texture, refs[slot]);
            assert_eq!(f.cullface.map(|c| c as usize), Some(slot));
            assert_eq!(f.tintindex, None);
            assert_eq!(f.rotation, None);
        }

        // the 26.2 euler EXTENSION — template_hanging_sign_rot_3 ships two
        // {x,y,z,origin} rotations verbatim among its axis rotations, and
        // the mirrored uvs survive as written
        let hs = ELEMENTS
            .iter()
            .find(|(id, _)| *id == "block/template_hanging_sign_rot_3")
            .unwrap()
            .1;
        assert_eq!(hs.len(), 5);
        let rotations: Vec<String> = hs
            .iter()
            .map(|el| match el.rotation {
                Some(ElementRotation::Axis { angle, axis, origin, rescale }) => format!(
                    "A({angle},{:?},{origin:?},{rescale})",
                    axis
                ),
                Some(ElementRotation::Euler { x, y, z, origin }) => {
                    format!("E({x},{y},{z},{origin:?})")
                }
                None => "none".to_string(),
            })
            .collect();
        assert_eq!(
            rotations,
            vec![
                "A(-67.5,Y,[8.0, 0.0, 8.0],false)",
                "A(-22.5,Y,[8.0, 0.0, 8.0],false)",
                "E(180,-67.5,-180,[8.0, 0.0, 8.0])",
                "A(-22.5,Y,[8.0, 0.0, 8.0],false)",
                "E(180,-67.5,-180,[8.0, 0.0, 8.0])",
            ]
        );
        assert_eq!(hs[2].faces[2].unwrap().uv, Some([15.5, 4.0, 14.0, 6.0]));
        assert_eq!(hs[2].faces[3].unwrap().uv, Some([14.0, 4.0, 15.5, 6.0]));

        // heavy_core's BARE texture refs (no '#') — six faces, as written
        let hc = ELEMENTS
            .iter()
            .find(|(id, _)| *id == "block/heavy_core")
            .unwrap()
            .1;
        assert_eq!(hc.len(), 1);
        for f in hc[0].faces.iter().flatten() {
            assert_eq!(f.texture, "all", "heavy_core's bare ref");
        }

        // the angle domain is RELAXED in 26.2 — the old {-45,-22.5,0,22.5,45}
        // five-value set is gone; the census counts of every shipped angle
        let angle_count = |want: f32| {
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .filter_map(|el| el.rotation)
                .filter(|r| matches!(r, ElementRotation::Axis { angle, .. } if *angle == want))
                .count()
        };
        assert_eq!(angle_count(45.0), 131);
        assert_eq!(angle_count(-45.0), 70);
        assert_eq!(angle_count(-22.5), 49);
        assert_eq!(angle_count(22.5), 22);
        assert_eq!(angle_count(0.0), 19);
        assert_eq!(angle_count(-67.5), 7, "the relaxed domain");
        assert_eq!(angle_count(-90.0), 2, "the relaxed domain");
        assert_eq!(angle_count(0.0001), 1, "the relaxed domain");
        assert_eq!(
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .filter(|el| matches!(el.rotation, Some(ElementRotation::Euler { .. })))
                .count(),
            2,
            "the euler rotations ride alongside"
        );

        // the face-level census: rotation {0x1, 90x151, 180x39, 270x55},
        // tintindex {0x90, 1x32}, 531 default-uv faces, 1041 culled faces
        let frot = |want: u16| {
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .flat_map(|el| el.faces.iter().flatten())
                .filter_map(|f| f.rotation)
                .filter(|r| *r == want)
                .count()
        };
        assert_eq!((frot(0), frot(90), frot(180), frot(270)), (1, 151, 39, 55));
        let ftint = |want: u8| {
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .flat_map(|el| el.faces.iter().flatten())
                .filter_map(|f| f.tintindex)
                .filter(|t| *t == want)
                .count()
        };
        assert_eq!((ftint(0), ftint(1)), (90, 32));
        let all_faces = ELEMENTS
            .iter()
            .flat_map(|(_, els)| els.iter())
            .flat_map(|el| el.faces.iter().flatten());
        assert_eq!(all_faces.clone().filter(|f| f.uv.is_none()).count(), 531);
        assert_eq!(all_faces.filter(|f| f.cullface.is_some()).count(), 1041);

        // light_emission: exactly the four emissive-cross elements, 15
        let emissive: Vec<(&str, u8)> = ELEMENTS
            .iter()
            .flat_map(|(id, els)| {
                els.iter()
                    .filter_map(move |el| el.light_emission.map(|l| (*id, l)))
            })
            .collect();
        assert_eq!(emissive.len(), 4);
        assert!(
            emissive
                .iter()
                .all(|(id, l)| *l == 15 && id.ends_with("cross_emissive"))
        );

        // the shade census: 333 explicit-false elements (852 default-or-true)
        assert_eq!(
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .filter(|el| !el.shade)
                .count(),
            333
        );
        // editor metadata retained: 94 names, 63 comments
        assert_eq!(
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .filter(|el| el.name.is_some())
                .count(),
            94
        );
        assert_eq!(
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .filter(|el| el.comment.is_some())
                .count(),
            63
        );
    }

    #[test]
    fn the_blockstate_table_is_the_jar_census_and_the_joins_hold() {
        use generated::blockstates::BLOCKSTATES;
        use generated::blocks::BLOCKS;
        use generated::models::MODELS;

        // the census, verbatim: 1198 files split EXACTLY variants (1096) or
        // multipart (102) — never both, never neither
        assert_eq!(BLOCKSTATES.len(), 1198);
        let with_variants = BLOCKSTATES.iter().filter(|r| r.1.is_some()).count();
        let with_multipart = BLOCKSTATES.iter().filter(|r| r.2.is_some()).count();
        assert_eq!((with_variants, with_multipart), (1096, 102));
        assert!(BLOCKSTATES.iter().all(|r| r.1.is_some() ^ r.2.is_some()));

        // the total applies = the "model" key occurrences across all JSON
        // columns; the distinct namespace-stripped refs join the models table
        let mut total_applies = 0usize;
        let mut refs: Vec<&str> = Vec::new();
        for (_, variants, multipart) in BLOCKSTATES.iter() {
            let json = variants.or(*multipart).unwrap();
            let needle = "\"model\":\"";
            let mut rest = json;
            while let Some(i) = rest.find(needle) {
                rest = &rest[i + needle.len()..];
                let end = rest.find('"').unwrap();
                refs.push(&rest[..end]);
                rest = &rest[end..];
                total_applies += 1;
            }
        }
        assert_eq!(total_applies, 8661);
        refs.sort_unstable();
        refs.dedup();
        assert_eq!(refs.len(), 2380, "the distinct model refs");
        for r in &refs {
            let bare = r.strip_prefix("minecraft:").unwrap_or(r);
            assert!(
                MODELS.iter().any(|m| m.0 == bare),
                "blockstate model ref {r} resolves to no model file"
            );
        }
        // the 4 vanilla BARE refs (no namespace: item_frame x2 incl. the
        // _map variant, glow_item_frame x2 likewise) ride as written — the
        // needle's leading quote keeps the namespaced refs out
        let bare_total = BLOCKSTATES
            .iter()
            .filter_map(|r| r.1.as_ref().or(r.2.as_ref()))
            .map(|j| j.matches("\"model\":\"block/").count())
            .sum::<usize>();
        assert_eq!(bare_total, 4, "the vanilla bare refs");

        // spot rows — the serde-canonical JSON fragments, byte-verbatim
        let grass = BLOCKSTATES
            .iter()
            .find(|r| r.0 == "grass_block")
            .unwrap()
            .1
            .as_ref()
            .unwrap();
        assert!(grass.contains("\"snowy=false\":[{\"model\":\"minecraft:block/grass_block\"},"));
        assert!(grass.contains("{\"model\":\"minecraft:block/grass_block\",\"y\":270}]"));
        assert!(grass.contains("\"snowy=true\":{\"model\":\"minecraft:block/grass_block_snow\"}}"));
        let leaf = BLOCKSTATES
            .iter()
            .find(|r| r.0 == "leaf_litter")
            .unwrap()
            .2
            .as_ref()
            .unwrap();
        assert!(leaf.starts_with(
            "[{\"apply\":{\"model\":\"minecraft:block/leaf_litter_1\"},\"when\":{\"facing\":\"north\",\"segment_amount\":\"1\"}}"
        ));
        let wire = BLOCKSTATES
            .iter()
            .find(|r| r.0 == "redstone_wire")
            .unwrap()
            .2
            .as_ref()
            .unwrap();
        assert!(wire
            .contains("\"OR\":[{\"east\":\"none\",\"north\":\"none\",\"south\":\"none\",\"west\":\"none\"}"));
        let shelf = BLOCKSTATES
            .iter()
            .find(|r| r.0 == "acacia_shelf")
            .unwrap()
            .2
            .as_ref()
            .unwrap();
        assert!(shelf.contains("\"AND\":[{\"facing\":\"north\"},{\"powered\":\"false\"}]"));
        // the repeater carries the census max variant count (64 states)
        let rep = BLOCKSTATES
            .iter()
            .find(|r| r.0 == "repeater")
            .unwrap()
            .1
            .as_ref()
            .unwrap();
        assert_eq!(rep.matches("{\"model\":").count(), 64);

        // THE JOIN: every blockstate id is a block in the registry (both
        // tables derive from the same jar directory)
        for (id, _, _) in BLOCKSTATES.iter() {
            assert!(
                BLOCKS.iter().any(|b| b.name == *id),
                "blockstate {id} is not a block"
            );
        }
    }

    #[test]
    fn the_texture_namespace_is_sorted_and_binary_searchable() {
        use crate::generated::textures::{texture_index, TEXTURE_IDS, N_TEXTURES};
        use crate::textures_table::{is_missing, texture_rgba};

        // the census: 2172 distinct ids over the 3928 model files + the
        // special renderers' 59 entity textures (43 raw + the banner
        // flag's 16 pre-tinted DyeColor variants)
        assert_eq!(N_TEXTURES, 2172);
        assert_eq!(TEXTURE_IDS.len(), N_TEXTURES);
        assert!(
            TEXTURE_IDS.windows(2).all(|w| w[0] < w[1]),
            "the namespace is strictly ascending (binary-search prerequisite)"
        );

        // hits: block/, item/, and a 32x32 sign sprite
        for hit in ["block/stone", "item/iron_sword", "block/oak_sign"] {
            assert!(texture_index(hit).is_some(), "{hit} must hit");
        }
        // a hit is NOT existence: the literal `missingno` (item/air.json and
        // block/air.json reference `minecraft:missingno`) is a domain member
        // pinned missing — the empty span both boot arms speak
        let air = texture_index("missingno").expect("the air models' literal ref");
        assert!(is_missing(air));
        assert!(texture_rgba(air).is_empty());

        // misses: the empty id, a bare stem, an unknown block, a texture
        // that EXISTS in the jar but is NOT model-referenced (the skin rides
        // the entity path — skingen's domain, not the models'), and a case
        // mismatch
        for miss in ["", "stone", "block/nonexistent", "entity/player/wide/steve", "Block/stone"] {
            assert!(texture_index(miss).is_none(), "{miss:?} must miss");
        }
    }

    #[test]
    fn every_present_tile_spans_exactly_its_size() {
        use crate::generated::textures::{TEXTURE_IDS, N_MISSING, N_TEXTURES};
        use crate::textures_table::{is_missing, real, texture_rgba, texture_size};

        assert_eq!(N_MISSING, 65);
        // the no-cache boot the honest-empty arms walk (every id missing,
        // the census pins gate on the real table)
        if !real() {
            for i in 0..N_TEXTURES {
                assert!(is_missing(i as u16), "{} missing at the empty table", TEXTURE_IDS[i]);
                assert!(texture_rgba(i as u16).is_empty());
                assert_eq!(texture_size(i as u16), (0, 0));
            }
            return;
        }
        let mut present = 0usize;
        let mut n16 = 0usize;
        let mut n32 = 0usize;
        let mut n64 = 0usize;
        let mut n_skin = 0usize;
        let mut n_dragon = 0usize;
        let mut total = 0usize;
        for i in 0..N_TEXTURES {
            let (w, h) = texture_size(i as u16);
            let span = texture_rgba(i as u16).len();
            if w == 0 {
                // the missing invariant: all three encodings agree
                assert_eq!(h, 0, "{} must be fully empty", TEXTURE_IDS[i]);
                assert_eq!(span, 0);
                assert!(is_missing(i as u16));
                continue;
            }
            assert!(!is_missing(i as u16));
            assert_eq!(span, w as usize * h as usize * 4, "{}", TEXTURE_IDS[i]);
            total += span;
            match (w, h) {
                (16, 16) => n16 += 1,
                (32, 32) => n32 += 1,
                // the special renderers' entity textures: the skins'
                // wide canvases, the conduit's base strip, the dragon's
                // 256 sheet (the census join below pins their counts)
                (32, 16) => n_skin += 1,
                (64, 32) => n_skin += 1,
                (64, 64) => n64 += 1,
                (256, 256) => n_dragon += 1,
                other => panic!("unexpected dims {other:?} for {}", TEXTURE_IDS[i]),
            }
            present += 1;
        }
        // the census, verbatim: 2107 present (2003 square-16 + 49
        // square-32 + 49 64-wide skins/canvases + 5 skin strips + 1
        // dragon sheet) + 65 missing = 2172; the spans tile the
        // decoded texels exactly
        assert_eq!(present, 2107);
        assert_eq!((n16, n32), (2003, 49));
        assert_eq!(n64, 49);
        assert_eq!(n_skin, 5, "the 32x16/64x32 strips: conduit base x2 + creeper/skeleton-family skins");
        assert_eq!(n_dragon, 1);
        assert_eq!(present + N_MISSING, N_TEXTURES);
        assert_eq!(total, 3_345_408, "the census texel total");
    }

    #[test]
    fn block_stone_pixels_are_the_world_atlas_tile_byte_for_byte() {
        use crate::generated::atlas::{TILE_NAMES, TILES};
        use crate::generated::textures::texture_index;
        use crate::textures_table::{real, texture_rgba, texture_size};

        if !real() {
            return; // the no-cache boot has no pixels to pin
        }
        // the CROSS-TABLE pin: stone is untinted, so the world atlas's baked
        // tile and this table's raw tile must be byte-identical
        let stone = texture_index("block/stone").expect("stone in the namespace");
        let atlas_idx = TILE_NAMES
            .iter()
            .position(|n| *n == "stone")
            .expect("the world atlas carries stone");
        assert_eq!(texture_size(stone), (16, 16));
        assert_eq!(texture_rgba(stone), TILES[atlas_idx]);

        // the independent-decoder pins (PIL over the same jar): the corners
        // and the center of the REAL stone.png
        let px = texture_rgba(stone);
        assert_eq!(&px[0..4], &[143, 143, 143, 255], "top-left");
        assert_eq!(&px[15 * 4..16 * 4], &[127, 127, 127, 255], "top-right");
        assert_eq!(&px[15 * 16 * 4..15 * 16 * 4 + 4], &[127, 127, 127, 255], "bottom-left");
        assert_eq!(&px[16 * 16 * 4 - 4..], &[127, 127, 127, 255], "bottom-right");
        assert_eq!(&px[(8 * 16 + 8) * 4..(8 * 16 + 8) * 4 + 4], &[143, 143, 143, 255], "center");
    }

    #[test]
    fn the_raw_pixels_carry_no_baked_tint() {
        use crate::generated::atlas::{TILE_NAMES, TILES};
        use crate::generated::textures::texture_index;
        use crate::textures_table::{real, texture_rgba};

        if !real() {
            return; // the no-cache boot has no pixels to pin
        }
        // the TINT-BOUNDARY pair: stone (no tint) is byte-identical to the
        // atlas; grass_block_top (the 0x91BD59 plains bake in the world
        // atlas) must DIFFER — this table keeps the raw grayscale master the
        // consumer tints at draw time (the quads' tintindex)
        let grass = texture_index("block/grass_block_top").expect("grass top");
        let px = texture_rgba(grass);
        // the PIL pins: raw top-left and center
        assert_eq!(&px[0..4], &[148, 148, 148, 255]);
        assert_eq!(&px[(8 * 16 + 8) * 4..(8 * 16 + 8) * 4 + 4], &[179, 179, 179, 255]);
        let atlas_idx = TILE_NAMES
            .iter()
            .position(|n| *n == "grass_block_top")
            .expect("the world atlas bakes grass_block_top");
        assert_ne!(
            px, TILES[atlas_idx],
            "the raw master must NOT equal the tint-baked atlas tile"
        );
        // the baked value the atlas carries for the same pixel: the vanilla
        // multiply (148 * 0x91 + 127) / 255 — proving the DIFF is the tint
        let baked = ((148u16 * 0x91 + 127) / 255) as u8;
        assert_eq!(TILES[atlas_idx][0], baked);

        // semi-transparency rides intact: water_still's raw alpha is 180
        let water = texture_index("block/water_still").expect("water");
        assert_eq!(&texture_rgba(water)[0..4], &[194, 194, 194, 180]);
    }

    #[test]
    fn animated_strips_ride_their_first_frame_and_signs_ride_whole() {
        use crate::generated::textures::texture_index;
        use crate::textures_table::{real, texture_rgba, texture_size};

        if !real() {
            return; // the no-cache boot has no pixels to pin
        }
        // water_still.png is 16x512 with an animation mcmeta: the table's
        // sprite is the first 16x16 frame (the world atlas's crop rule,
        // now mcmeta-faithful and pinned on the namespace)
        let water = texture_index("block/water_still").unwrap();
        assert_eq!(texture_size(water), (16, 16));

        // oak_sign.png is a WHOLE 32x32 sprite (no mcmeta): never cropped
        let sign = texture_index("block/oak_sign").unwrap();
        assert_eq!(texture_size(sign), (32, 32));
        let px = texture_rgba(sign);
        assert_eq!(px.len(), 32 * 32 * 4);
        assert_eq!(&px[0..4], &[194, 157, 98, 255], "sign top-left (PIL)");
        assert_eq!(&px[32 * 32 * 4 - 4..], &[0, 0, 0, 0], "sign bottom-right (transparent)");

        // transparent pixels survive decode: the iron sword's top-left is
        // fully transparent, its center the steel gray (PIL)
        let sword = texture_index("item/iron_sword").unwrap();
        assert_eq!(texture_size(sword), (16, 16));
        let px = texture_rgba(sword);
        assert_eq!(&px[0..4], &[0, 0, 0, 0]);
        assert_eq!(&px[(8 * 16 + 8) * 4..(8 * 16 + 8) * 4 + 4], &[216, 216, 216, 255]);
    }

    #[test]
    fn the_missing_materials_are_exactly_the_census() {
        use crate::generated::textures::{texture_index, TEXTURE_IDS, N_MISSING};
        use crate::textures_table::{is_missing, real};

        if !real() {
            return; // the census of MISSINGNESS needs the real table
        }
        // 65 missing: the 64 trims/items armor-trim refs (the client jar
        // ships no PNG for them — vanilla renders the checkerboard) + the
        // literal `missingno`; NOTHING from block/ or item/ is missing
        let mut missing: Vec<&str> = Vec::new();
        for (i, id) in TEXTURE_IDS.iter().enumerate() {
            assert_eq!(
                is_missing(i as u16),
                *id == "missingno" || id.starts_with("trims/items/"),
                "{id} missingness disagrees with the census"
            );
            if is_missing(i as u16) {
                missing.push(id);
            }
        }
        assert_eq!(missing.len(), N_MISSING);
        assert_eq!(missing.len(), 65);
        assert_eq!(
            missing.iter().filter(|id| id.starts_with("trims/items/")).count(),
            64
        );
        assert!(missing.contains(&"missingno"));

        // the 57 unreferenced trims PNGs stay OUT of the namespace: the jar
        // ships trims/color_palettes/* (the RUNTIME material lookup — zero
        // overlap with the 64 missing trims/items/* refs), so only REFERENCED
        // ids land in the table — the domain is the models' domain
        assert!(texture_index("trims/color_palettes/iron").is_none());
        assert!(!TEXTURE_IDS.iter().any(|id| id.starts_with("trims/color_palettes/")));
    }

    #[test]
    fn the_special_payloads_join_the_dump_and_the_texture_namespace() {
        use crate::generated::item_models::ITEM_SPECIAL_MODELS;
        use crate::generated::textures::texture_index;
        use crate::textures_table::{real, texture_size};

        if !real() {
            return; // the join's pixel half needs the real table
        }
        // THE BIJECTION: 63 payloads, every one resolving through the
        // join — the dump geometry, the base model path, and (for the
        // tinted layers) the pre-tinted texture variant all in place
        assert_eq!(ITEM_SPECIAL_MODELS.len(), 63);
        for (id, _) in ITEM_SPECIAL_MODELS {
            let render = special_item_render_by_name(id)
                .unwrap_or_else(|| panic!("{id}: the special join dangles"));
            assert!(!render.layers.is_empty(), "{id}: no layers");
            for layer in &render.layers {
                let idx = texture_index(layer.texture)
                    .unwrap_or_else(|| panic!("{id}: texture {} absent", layer.texture));
                let (w, _h) = texture_size(idx);
                assert!(w > 0, "{id}: texture {} missing", layer.texture);
            }
        }

        // the shield: the nopattern base (the plain shield's own law),
        // the plate's dump bounds — and the plate's TOP face's first
        // vertex bit-exact at the raw model space (identity part pose)
        let shield = special_item_render_by_name("shield").unwrap();
        assert_eq!(shield.base, "item/shield");
        assert_eq!(shield.layers[0].texture, "entity/shield/shield_base_nopattern");
        let plate = shield.layers[0]
            .layer
            .parts
            .iter()
            .find(|p| p.name == "plate")
            .expect("the plate");
        assert_eq!(plate.cubes[0].min, [-6.0, -11.0, -2.0]);
        assert_eq!(plate.cubes[0].max, [6.0, 11.0, -1.0]);

        // the white banner: the ROD rides the untinted canvas, the FLAG
        // rides the pre-tinted white variant (the DyeColor multiply,
        // generation-time)
        let banner = special_item_render_by_name("white_banner").unwrap();
        assert_eq!(banner.layers.len(), 2);
        assert_eq!(banner.layers[0].texture, "entity/banner/base");
        assert_eq!(
            banner.layers[1].texture,
            "entity/banner/banner_base_white"
        );

        // the conduit: THREE resting layers (shell, cage, eye) — the
        // wind column is the powered state's, never the item's
        let conduit = special_item_render_by_name("conduit").unwrap();
        assert_eq!(conduit.layers.len(), 3);
        for tex in [
            "entity/conduit/base",
            "entity/conduit/cage",
            "entity/conduit/closed_eye",
        ] {
            assert!(conduit.layers.iter().any(|l| l.texture == tex), "{tex}");
        }

        // the golem statue: the STANDING fallback, the plain golem skin
        let statue = special_item_render_by_name("copper_golem_statue").unwrap();
        assert_eq!(statue.layers[0].texture, "entity/copper_golem/copper_golem");

        // the unknown is honest
        assert!(special_item_render_by_name("not_an_item").is_none());
    }

    #[test]
    fn the_special_ids_disjoint_the_layer_lanes_exactly() {
        // the special ids take the special lane and NOTHING else: no
        // plain def, no resolved layers — the client's resolution order
        // (special first) can never shadow the other lanes
        use generated::item_models::{
            ITEM_MODEL_DEFS, ITEM_SPECIAL_MODELS, RESOLVED_ITEM_LAYERS,
        };
        for (id, _) in ITEM_SPECIAL_MODELS {
            assert!(ITEM_MODEL_DEFS.iter().all(|d| d.0 != *id));
            assert!(RESOLVED_ITEM_LAYERS.iter().all(|d| d.0 != *id));
            // and the id resolves on the WIRE registry too (a live
            // server's held slot lands by number)
            let wire = generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == *id)
                .unwrap_or_else(|| panic!("{id}: no wire id"))
                as i32;
            assert!(special_item_renderer(wire).is_some(), "{id}({wire})");
        }
    }
}

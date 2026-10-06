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
/// leaf_litter is the FIRST non-cube member (the thirteenth audit): its
/// blockstate is a multipart and its models are flat zero-thickness ground
/// quads (template_leaf_litter_1..4), so the full-cube resolution chain
/// doesn't apply — the build loop special-cases it to the flat stem, and
/// the client's mesher meshes it as the dry-fold ground cover it is.
/// farmland is kind 27 (the twenty-eighth entry, the autopush-311 round): its model
/// chain (blockstates/farmland.json moisture=0 variant -> block/farmland
/// -> template_farmland) resolves through the SAME full-cube face walk —
/// the element's six faces name dirt sides + the farmland top, the mesher
/// meshes it as the cube the vocabulary speaks (the vanilla 15/16 height
/// the generic full-cube path folds — the documented simplification the
/// offline world's own shape keeps).
/// dirt_path is kind 28 (the twenty-ninth entry, the autopush-312 round):
/// the shovel's own flatten arm's product (javap ShovelItem.useOn — the
/// FLATTENABLES map's `Blocks.DIRT_PATH.defaultBlockState()` the
/// GRASS_BLOCK/DIRT rows speak). The model chain (blockstates/dirt_path.json
/// -> block/dirt_path -> template_dirt_path) resolves through the SAME
/// full-cube face walk — the element's six faces name the dirt bottom +
/// the path top/sides, the 15/16 height the generic path folds (the same
/// documented simplification).
/// wheat is kind 29 (the thirtieth entry, the autopush-313 round): the
/// seeds' own planting arm's product (javap BlockItem.place over the
/// real 26.2 jar — the SUPPORTS_CROPS tag's sole farmland row the
/// CropBlock.canSurvive speaks). The crop model is the SECOND non-cube
/// member: blockstates/wheat.json age=0 -> block/wheat_stage0 ->
/// block/crop, whose elements are FOUR zero-thickness vertical planes
/// (x=4 and x=12 spanning z 0..16, z=4 and z=12 spanning x 0..16, each
/// y -1..15 — the sprite sinks one pixel into the farmland below),
/// `shade: false` on every face and `ambientocclusion: false` on the
/// model — the full-cube six-face requirement cannot apply, so the
/// build loop special-cases it to the age=0 texture stem through the
/// REAL chain (the generator's own blockstate -> model -> texture-ref
/// walk, never a hardcoded name). The faces carry NO tintindex — the
/// wheat_stage0 master renders as authored (the pre-colored pixels the
/// probe verifies).
/// crafting_table is kind 37 (the thirty-eighth entry, the autopush-328
/// round): the placement arm's own product (javap BlockItem.place — the
/// crafting_table item the world's vocabulary wears). The model chain
/// (blockstates/crafting_table.json -> block/crafting_table) resolves
/// through the SAME full-cube face walk — the element's six faces name
/// crafting_table_front/side/top, the plain opaque cube the generic
/// path meshes (the vanilla's facing rotation the generic fold keeps —
/// the axis-aligned placement the offline world speaks).
/// tnt is kind 38 (the thirty-ninth entry, the autopush-410 round): the
/// ignite arm's own target (javap TntBlock.useItemOn — the
/// flint_and_steel/fire_charge held the prime walk speaks) and the
/// blast's own chain seed (the exploded TNT cell the wasExploded walk
/// answers). The model chain (blockstates/tnt.json -> block/tnt ->
/// block/cube_bottom_top) resolves the element's six faces
/// tnt_top/tnt_side/tnt_bottom, the plain opaque cube the generic path
/// meshes; the vanilla `instabreak()` strength the blast resistance
/// table speaks (0.0 — the ray marches through).
/// dandelion is kind 39 and poppy kind 40 (the fortieth/forty-first
/// entries, the autopush-411 round): the plains' own flower patch (javap
/// the jar's worldgen json — the flower_plain placed feature the
/// noise_threshold_provider wears, the default dandelion the
/// high-chance poppy third keeps) the worldgen decorate pass plants.
/// The model chain (blockstates/dandelion.json -> block/dandelion ->
/// block/cross; blockstates/poppy.json -> block/poppy -> block/cross)
/// resolves the `cross` texture ref — the two diagonal plane elements
/// the cross model's own 45° rotY + rescale walk bakes, `shade: false`
/// + `ambientocclusion: false` — so resolve_block's full-cube six-face
/// requirement cannot apply; the pre-colored masters render as authored
/// (the faces carry NO tintindex).
/// short_grass is kind 41 and bush kind 42 (the forty-second/forty-third
/// entries, the autopush-412 round): the plains' own GRASS TUFT (javap
/// the jar's worldgen json — the patch_grass_plain placed feature the
/// simple_state_provider's short_grass state wears, the patch_bush the
/// rarity-4/count-24 bush row keeps — the two decorations the step-9
/// list the flowers' own siblings wear) the decorate pass plants. The
/// model chain (blockstates/short_grass.json -> block/short_grass ->
/// block/tinted_cross; blockstates/bush.json -> block/bush -> the same
/// parent) is the flowers' own cross bake plus the TINT: the tinted_cross
/// parent's four faces ALL carry `tintindex: 0` — vanilla's
/// `BlockColors.createDefault()` registers `BlockTintSources.grass()`
/// (the `BiomeColors.getAverageGrassColor` source — the same source the
/// grass_block top consumes) for FERN/SHORT_GRASS/POTTED_FERN/BUSH
/// (the audited register walk) — so the grayscale masters ride the
/// TINT_GRASS bake exactly the grass_block_top row speaks, and the
/// generated TINTS table carries the plains constant per (kind, face)
/// (the no-op rule the client's fold keeps: the baked pixels and the
/// resolved biome color agree at plains, the fold fires only off-plains).
/// tall_grass is kind 43, tall_grass_upper kind 44, large_fern kind 45
/// and large_fern_upper kind 46 (the forty-fourth..forty-seventh
/// entries, the autopush-413 round): the plains' own DOUBLE PLANTS
/// (javap the jar's worldgen json — the patch_tall_grass_2 placed
/// feature the plains step-9 list wears, the patch_large_fern the
/// forest row keeps; the configured features' simple_state_providers
/// place the `half=lower` state, the SimpleBlockFeature's
/// DoublePlantBlock arm the placeAt walk speaks — the LOWER and the
/// UPPER cell the pair wears). The `_upper` palette names are the
/// client's own half-kind word (the blockstate's half=upper variant the
/// resolver reads — the NAME_OF registry row keeps the BASE name: the
/// dig/drop/sound lanes both halves speak the same block
/// row). The model chain (blockstates/tall_grass.json half=lower ->
/// block/tall_grass_bottom, half=upper -> block/tall_grass_top;
/// large_fern the same chain's own rows) is the tuft's own tinted_cross
/// bake per HALF: the two diagonal plane elements, `shade: false`,
/// `ambientocclusion: false`, the tintindex-0 faces — vanilla's
/// `BlockColors.createDefault()` registers `BlockTintSources.
/// doubleTallGrass()` for TALL_GRASS + LARGE_FERN (the audited 2-aastore
/// register row; the $2 source's colorInWorld the UPPER half's
/// `pos.below()` read keeps) — so the four grayscale masters ride the
/// TINT_GRASS bake exactly the short_grass row speaks.
/// azure_bluet is kind 47, oxeye_daisy kind 48, cornflower kind 49,
/// orange_tulip kind 50, red_tulip kind 51, pink_tulip kind 52 and
/// white_tulip kind 53 (the forty-eighth..fifty-fourth entries, the
/// autopush-414 round): the flower_plain NOISE THRESHOLD PROVIDER'S
/// OWN FULL PALETTE (javap the jar's worldgen json — the
/// configured feature's `high_states` the [poppy, azure_bluet,
/// oxeye_daisy, cornflower] list wears, the `low_states` the
/// [orange_tulip, red_tulip, pink_tulip, white_tulip] rows keep: the
/// provider's own list order the indices bank — the kinds ride the
/// provider's rows exactly the poppy first-row word the prior rounds
/// kept). The model chain (blockstates/<flower>.json -> block/<flower>
/// -> block/cross) is the dandelion's own cross bake PER KIND: the two
/// diagonal plane elements, `shade: false` faces under model-level
/// `ambientocclusion: false`, the pre-colored masters (the faces carry
/// NO tintindex — vanilla's BlockColors registers no source for the
/// flower family, the pixels render as authored). The low_states rows
/// the noise port's own round plants (the provider's `d0 < threshold`
/// arm the client's hash lane never carries — the -0.8 valley the
/// NormalNoise walk banks); the high_states rows the bonemeal fold's
/// Util.getRandom pick and the decorate pass's stand-in wear.
/// torch is kind 54 (the fifty-fifth entry, the dev-32 round): THE
/// TORCH'S OWN MODEL CHAIN — the blockstate is a single variant map
/// ("") -> block/torch -> block/template_torch, the ONE ELEMENT box
/// (from [7,0,7] to [9,10,9], `shade: false` faces under model-level
/// `ambientocclusion: false` — the tesselateWithoutAO shape the crop
/// family wears) whose SIX faces ALL reference `#torch` (javap the
/// model json: down uv [7,13,9,15] with cullface down, up uv
/// [7,6,9,8], the four sides uv [7,6,9,16]) — the all-six-faces row
/// resolve_block's full-cube finder keeps, every face the ONE
/// block/torch texture answers (the single tile the whole kind
/// samples, the per-quad UV windows the sub-sprite law carries).
/// white_concrete_powder is kind 55 (the fifty-sixth entry, the
/// autopush-426 round): THE CONCRETE POWDER'S OWN MODEL CHAIN —
/// the blockstate is a single variant map ("") -> block/white_concrete_
/// powder -> block/cube_all (the parent chain the sand/gravel pair
/// wears), the SIX faces ALL reference `#all` = block/white_concrete_
/// powder — the full-cube resolve_block walk the plain stems answer.
/// The FALLING family's third member (the ConcretePowderBlock ctor's
/// superclass the javap walk speaks) — the falling body the carried
/// kind's own block model renders, the water verdict the conversion
/// arms (the client's own FallingBlockEntity.tick port).
/// white_concrete is kind 56 (the fifty-seventh entry, the same round):
/// the CONVERSION'S OWN TARGET — the `concrete` field the ctor's
/// registry read banks, the getConcrete().defaultBlockState() the
/// onLand and the updateShape conversions both speak; the SAME
/// cube_all chain the powder's walk keeps (the color pair the
/// vanilla's own two rows carry).
/// fire is kind 57 and soul_fire kind 58 (the fifty-eighth/fifty-ninth
/// entries, the autopush-515 round): the flint arm's own product and
/// the ENTITY-INSIDE brand the body's fire walk reads (javap the 26.2
/// jar — BaseFireBlock.entityInside's CLEAR_FREEZE + FIRE_IGNITE +
/// the runAfter hurt(inFire(), fireDamage) the ctor floats speak:
/// FireBlock fconst_1 = 1.0, SoulFireBlock fconst_2 = 2.0; the
/// blocks clinit's noCollision/instabreak/replaceable rows the walk
/// banks, the light lambdas' bipush 15 / bipush 10 the emission
/// table answers). The blockstate is a MULTIPART (the floor pair
/// under a no-walls condition, four side groups + the up pair),
/// every model built from ROTATED elements (template_fire_floor's
/// 22.5° rotX/rotZ + rescale planes, template_fire_side's flat
/// z=0.01 walls — all y 0..22.4, `shade: false`, model-level
/// `ambientocclusion: false`) — resolve_block's full-cube six-face
/// requirement cannot apply. The texture stem resolves through the
/// REAL chain (the multipart's FIRST floor apply fire_floor0 /
/// soul_fire_floor0 -> the merged texture map's `fire` ref ->
/// block/fire_0 / block/soul_fire_0), tinted = false (the faces
/// carry NO tintindex — the animated strips the game's own crop
/// machinery plays). The mesher's fire lane a documented follow-up
/// keeps (the palette row the kind's tile the FACE_TILES table
/// banks — the rendered quads the follow-up speaks).
/// candle is kind 59 and candle_lit kind 60 (the sixtieth/sixty-first
/// entries, the autopush-522 round): the CANLIGHT CONSULT'S OWN KINDS
/// the two igniter arms wait for (javap the 26.2 jar —
/// FlintAndSteelItem.useOn pc 24-45 the CampfireBlock/CandleBlock/
/// CandleCakeBlock canLight disjunction the LIT branch serves, the
/// FireChargeItem.useOn pc 20-41 the same walk the setBlockAndUpdate
/// flag-3 arm keeps). CandleBlock's ctor the DEFAULT state registers
/// (CANDLES=1, LIT=false, WATERLOGGED=false — the placement walk the
/// unlit kind speaks), the lightLevel lambda `lambda$static$0` the
/// `lit ? 3 * candles : 0` fold banks (the 1-candle default the LIT
/// row's 3 the emission table answers), the canSurvive the
/// `Block.canSupportCenter(level, below, UP)` delegate the torch's
/// own law speaks (the client's [`can_support_torch`] fold the same
/// below-cell verdict keeps). The blockstate is a plain VARIANTS map
/// over candles=1..4 x lit — the UNLIT kind the FIRST row resolves
/// ("candles=1,lit=false" -> block/candle_one_candle ->
/// block/template_candle — the stick box [7,0,7]->[9,6,9] whose SIX
/// faces the resolve_block full-cube finder accepts, the `#all` ref
/// block/candle the tile answers), the LIT kind the "candles=1,
/// lit=true" row the arm reads (block/candle_one_candle_lit -> the
/// SAME template geometry, `#all` block/candle_lit). The offline
/// state fold the two kinds speak (the kind-only vocabulary the
/// LIT/CANDLES properties never carries — the default state's own
/// rows the palette banks, the lit swap the igniter arms' own
/// setValue walk wears).
/// campfire is kind 61, campfire_lit kind 62, soul_campfire kind 63
/// and soul_campfire_lit kind 64 (the sixty-second..sixty-fifth
/// entries, the dev-6 round): THE DOUSE LANE'S OWN KINDS the shovel's
/// arm and the canLight consult's campfire rows wait for (javap the
/// 26.2 jar — CampfireBlock's ctor the DEFAULT state registers
/// LIT=true, SIGNAL_FIRE=false, WATERLOGGED=false, FACING=NORTH — the
/// vanilla campfire the player places lands LIT, the shovel's douse
/// arm the setValue(LIT, false) walk the unlit kind speaks, the flint
/// canLight relight the `!LIT` verdict the doused kind answers).
/// CampfireBlock.canLight's pc walk: the CAMPFIRES tag consult +
/// the `!WATERLOGGED` and `!LIT` reads the offline kind-only palette
/// the UNLIT kind's own row answers (the lit kind the consult
/// refuses — the same verdict the candle pair wears). The
/// properties chain the blocks clinit's lambda$static$310/311 arms
/// bank: strength(2.0), SoundType.WOOD, lightLevel litBlockEmission(15)
/// the plain row / (10) the soul row keeps (the `bipush 15`/`bipush
/// 10` the litBlockEmission calls speak), noOcclusion; the ctor args
/// (spawnParticles, fireDamage) the plain (true, 1) and the soul
/// (false, 2) rows keep. THE SHAPE the static init's `Block.column(
/// 16.0, 0.0, 7.0)` walk banks (ldc2_w 16.0/dconst_0/ldc2_w 7.0 —
/// the full 0..1 footprint under the 0..7/16 band the client's
/// Column(7) collision row answers). No canSurvive override the
/// 26.2 class carries — the campfire floats the support's loss
/// never douses (the placement gate's absence the generic lane
/// keeps). The blockstate is a plain VARIANTS map over facing x lit
/// — the UNLIT kinds the "facing=south,lit=false" row resolve
/// (block/campfire_off — the five-log element geometry BOTH kinds
/// share, the `#log` ref block/campfire_log the tile answers; the
/// SOUL_CAMPFIRE kind the SAME campfire_off model the blockstate's
/// own rows point, the unlit soul the plain-log look the vanilla
/// itself renders), the LIT kinds the "facing=south,lit=true" rows
/// read (block/campfire / block/soul_campfire — BOTH parents
/// block/template_campfire: the five-log boxes the lit_log faces
/// swap, PLUS the two ±45° rotY rescale fire planes the `#fire` ref
/// block/campfire_fire / block/soul_campfire_fire the strips' own
/// frametime-2 animations play). The offline state fold the four
/// kinds speak (the kind-only vocabulary the LIT/SIGNAL_FIRE/
/// WATERLOGGED/FACING properties never carries — the douse and the
/// relight the kind swaps the two lanes wear).
/// candle_cake is kind 65 and candle_cake_lit kind 66 (the
/// sixty-sixth/sixty-seventh entries, the autopush-523 round): THE
/// CANLIGHT CONSULT'S THIRD ROW the two igniter arms' disjunction
/// names and the palette has never banked (javap the 26.2 jar —
/// CandleCakeBlock's ctor the DEFAULT state registers LIT=false
/// (the setValue(LIT, false) iconst_0 walk the any() builder
/// serves), the registration the blocks clinit's BlockIds.
/// CANDLE_CAKE row banks: `ofLegacyCopy(CAKE)` the CAKE strength/
/// sound rows the legacy copy wears, `lightLevel(litBlockEmission(
/// 3))` the lit row's 3 the emission table answers — and the
/// register(ResourceKey, Function, Properties) walk carries NO
/// BlockItemId: the block the ITEM FORM never had, the placeable
/// lookup the vanilla itself refuses (the vanilla player the
/// candle-on-cake use lane the cake kind's own round serves).
/// CandleCakeBlock.canLight's pc walk: the CANDLE_CAKES tag consult
/// + the `hasProperty(LIT) && !getValue(LIT)` predicate
/// (lambda$canLight$0) the offline kind-only palette the UNLIT
/// kind's own row answers (the lit kind the consult refuses — the
/// same verdict the candle/campfire rows wear). THE SHAPE the
/// static init's `Shapes.or(column(2.0, 8.0, 14.0), column(14.0,
/// 0.0, 8.0))` walk banks — the cake slab's 0..8/16 band the
/// collision table's Column(8) row answers (the body walks onto
/// the cake the 0.6-block auto-step serves), the 2-px stick's
/// 8..14 band the per-cell vocabulary never speaks (the honest
/// fold the documented gap keeps). The blockstate is a plain
/// VARIANTS map over lit — "lit=false" -> block/candle_cake,
/// "lit=true" -> block/candle_cake_lit, BOTH parents
/// block/template_cake_with_candle: the cake slab box [1,0,1]->
/// [15,8,15] (#bottom/#top/#side the cake masters name) + the
/// candle stick box [7,8,7]->[9,14,9] (#candle — block/candle the
/// unlit row, block/candle_lit the lit row the vanilla's own
/// texture overrides carry) + the two ±45° rotY wick planes the
/// rotation walk bakes. The offline state fold the two kinds
/// speak (the kind-only vocabulary the LIT property never carries
/// — the relight the igniter arms' own kind swap wears; the use
/// lane the cake's eat/douse walks a future round keeps).
pub const RUNTIME_PALETTE: [&str; 67] = [
    "air", "stone", "dirt", "grass_block", "oak_log", "oak_leaves", "water", "sand",
    "granite", "diorite", "andesite", "deepslate", "cobbled_deepslate", "gravel",
    "bedrock", "coal_ore", "copper_ore", "iron_ore", "gold_ore", "redstone_ore",
    "lapis_ore", "diamond_ore", "emerald_ore", "netherrack", "pale_oak_leaves",
    "leaf_litter", "lava", "farmland", "dirt_path", "wheat",
    "wheat_stage1", "wheat_stage2", "wheat_stage3", "wheat_stage4",
    "wheat_stage5", "wheat_stage6", "wheat_stage7", "crafting_table", "tnt",
    "dandelion", "poppy", "short_grass", "bush",
    "tall_grass", "tall_grass_upper", "large_fern", "large_fern_upper",
    "azure_bluet", "oxeye_daisy", "cornflower",
    "orange_tulip", "red_tulip", "pink_tulip", "white_tulip",
    "torch",
    "white_concrete_powder", "white_concrete",
    "fire", "soul_fire",
    "candle", "candle_lit",
    "campfire", "campfire_lit", "soul_campfire", "soul_campfire_lit",
    "candle_cake", "candle_cake_lit",
];

/// Renderer face order: 0:+X 1:-X 2:+Y 3:-Y 4:+Z 5:-Z.
pub const FACE_KEYS: [&str; 6] = ["east", "west", "up", "down", "south", "north"];

/// Vanilla plains-biome tint constants (0xRRGGBB), applied by tintindex.
pub const TINT_GRASS: u32 = 0x91BD59;
pub const TINT_FOLIAGE: u32 = 0x77AB2F;
pub const TINT_WATER: u32 = 0x3F76E4;
/// The dry-foliage plains constant: the REAL dry_foliage.png grid sampled at
/// the plains climate (0.8, 0.4) through the audited `DryFoliageColor.get`
/// arithmetic (probed in the twelfth-audit run) — the dry analog of
/// TINT_GRASS/TINT_FOLIAGE, which are the same kind of in-world plains
/// grid samples (the plains equivalence). Baked into the grayscale
/// leaf_litter master the same way the game tints it in-world.
pub const TINT_DRY: u32 = 0xA37546;

#[derive(Debug, Clone)]
struct ResolvedFace {
    texture: String,   // e.g. "block/grass_block_top"
    #[cfg_attr(not(test), expect(dead_code))]
    tinted: bool,      // model face carried a tintindex
}

fn tint_for_texture(stem: &str) -> u32 {
    // Grayscale master textures the game tints in-world (plains biome here).
    match stem {
        "grass_block_top" => TINT_GRASS,
        "oak_leaves" => TINT_FOLIAGE,
        "water_still" => TINT_WATER,
        "leaf_litter" => TINT_DRY,
        // THE GRASS TUFT'S OWN ROW (the autopush-412 round): the
        // tinted_cross family's grayscale masters the
        // BlockTintSources.grass() source tints in-world — the SAME
        // TINT_GRASS the grass_block_top row speaks (the audited
        // register walk: FERN/SHORT_GRASS/POTTED_FERN/BUSH all ride
        // the biome grass color source)
        "short_grass" | "bush" => TINT_GRASS,
        // THE DOUBLE PLANT'S OWN ROW (the autopush-413 round): the
        // half-models' grayscale masters the BlockTintSources.
        // doubleTallGrass() source tints in-world — the SAME
        // TINT_GRASS the tuft row speaks (the audited register walk:
        // the TALL_GRASS/LARGE_FERN pair the 2-aastore row keeps; the
        // $2 source's colorInWorld the UPPER half's pos.below() read
        // the client's own below-shift folds)
        "tall_grass_bottom" | "tall_grass_top" | "large_fern_bottom"
        | "large_fern_top" => TINT_GRASS,
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
    if block == "lava" {
        // The SECOND fluid special case (the lava lane's own lookup —
        // the same no-blockstate shape the water arm speaks): the still
        // texture with NO tint — lava_still.png is the pre-colored
        // 16x512 strip the game itself never biome-tints (the animated
        // crop machinery the water arm rides handles the strip).
        return Ok(resolve_flat_stem("lava_still", false));
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
        if block == "leaf_litter" {
            // The thirteenth audit (official 26.2 jar): the blockstate is a
            // MULTIPART (leaf_litter_1..4 x facing y-rotations; segment 3
            // cumulatively applies model_2 AND model_3), and every
            // template_leaf_litter_N is a zero-thickness horizontal element
            // at y=0.25 with ONLY up+down faces — resolve_block's full-cube
            // six-face requirement cannot apply. The full-footprint
            // template (segment 4, the 16x16 quad) is the canonical
            // texture lookup; BOTH faces carry tintindex 0 -> the
            // dryFoliage() source (the DRY_SLOT), a GRAYSCALE master the
            // game tints in-world (probe: every opaque pixel r==g==b,
            // binary alpha 0/255 — cutout, unlike pre-colored pale oak).
            faces.push(resolve_flat_stem("leaf_litter", true));
            continue;
        }
        if block == "fire" || block == "soul_fire" {
            // THE FIRE KINDS' OWN CHAIN (the autopush-515 round, the
            // official 26.2 jar): the blockstate is a MULTIPART — the
            // floor pair fires under the no-walls condition, the four
            // side groups ride the wall directions, the up pair the
            // block above — and every template model is built from
            // ROTATED elements (template_fire_floor's ±22.5° rotX/
            // rotZ + rescale planes, template_fire_side's flat
            // z=0.01 walls, all y 0..22.4), so resolve_block's
            // full-cube six-face requirement cannot apply. The
            // texture stem resolves through the REAL chain: the
            // multipart's FIRST floor apply (block/fire_floor0 /
            // block/soul_fire_floor0 — the floor pair the no-walls
            // condition applies, floor0 the canonical first) ->
            // load_model_chain's merged texture map -> the `fire`
            // ref (block/fire_0 / block/soul_fire_0 — the animated
            // strips the game's own crop machinery plays), tinted =
            // false (the faces carry NO tintindex).
            let stem = if block == "fire" { "fire" } else { "soul_fire" };
            let bs_path = format!("assets/minecraft/blockstates/{stem}.json");
            let bs_bytes = read_jar(&mut archive, &bs_path)?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {stem}: json parse"))?;
            let parts = bs
                .get("multipart")
                .and_then(|v| v.as_array())
                .with_context(|| format!("blockstate {stem}: no multipart"))?;
            // the FIRST floor apply (the no-walls condition's own
            // pair — fire_floor0/soul_fire_floor0 the canonical
            // first row wears)
            let mut model_ref: Option<String> = None;
            for part in parts {
                let applies = part.get("apply").and_then(|v| v.as_array());
                let Some(applies) = applies else { continue };
                if let Some(first) = applies
                    .first()
                    .and_then(|a| a.get("model"))
                    .and_then(|m| m.as_str())
                {
                    let name = first.trim_start_matches("minecraft:").to_string();
                    if name.ends_with("floor0") {
                        model_ref = Some(name);
                        break;
                    }
                }
            }
            let model_ref = model_ref
                .with_context(|| format!("blockstate {stem}: no floor0 apply"))?;
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {stem}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("fire")
                .with_context(|| format!("model {model_ref}: no fire texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block == "candle_lit" {
            // THE LIT KIND'S OWN ROW (the autopush-522 round, the
            // official 26.2 jar): the palette name the client's own
            // half-kind word carries — the blockstate has NO
            // candle_lit.json (the lit state the SAME candle.json's
            // "candles=1,lit=true" variant resolves — the
            // `_upper` half-kind precedent the registry-name strip
            // wears). The walk reads the EXPLICIT row (never the
            // first-variant default the unlit kind resolves), the
            // merged texture map's `all` ref the block/candle_lit
            // stem answers, tinted = false (the faces carry NO
            // tintindex — the pre-colored master renders as
            // authored).
            let bs_bytes = read_jar(
                &mut archive,
                "assets/minecraft/blockstates/candle.json",
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| "blockstate candle: json parse")?;
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get("candles=1,lit=true"))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| "blockstate candle: no candles=1,lit=true model")?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block candle_lit: model chain {model_ref}"))?;
            let tex = tex_map
                .get("all")
                .with_context(|| format!("model {model_ref}: no all texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block == "campfire" || block == "soul_campfire" {
            // THE UNLIT KINDS' OWN ROWS (the dev-6 round, the official
            // 26.2 jar): the blockstates read the "facing=south,
            // lit=false" row (the y-rotation-free row the palette's
            // kind-only vocabulary keeps — the facing property the
            // offline world never speaks, the south row the default
            // orientation banks), BOTH rows resolving
            // block/campfire_off (the SOUL_CAMPFIRE kind the blockstate's
            // own lit=false rows the SAME model point — the unlit soul
            // campfire the plain-log look the vanilla itself renders).
            // The model's own textures (particle/log = block/
            // campfire_log) the `#log` ref answers, tinted = false
            // (the faces carry NO tintindex — the pre-colored logs
            // render as authored).
            let stem = if block == "campfire" {
                "campfire"
            } else {
                "soul_campfire"
            };
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{stem}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {stem}: json parse"))?;
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get("facing=south,lit=false"))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate {stem}: no facing=south,lit=false model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {stem}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("log")
                .with_context(|| format!("model {model_ref}: no log texture ref"))?;
            let tex_stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&tex_stem, false));
            continue;
        }
        if block == "campfire_lit" || block == "soul_campfire_lit" {
            // THE LIT KINDS' OWN ROWS (the same round — the half-kind
            // name the registry-name strip wears, the blockstate has
            // NO campfire_lit.json — the lit state the SAME
            // campfire/soul_campfire.json's "facing=south,lit=true"
            // rows resolve): block/campfire / block/soul_campfire —
            // BOTH parents block/template_campfire. The merged texture
            // map's `#fire` ref the block/campfire_fire /
            // block/soul_campfire_fire stem answers (the palette row
            // the strip's own frame-0 crop banks — the frametime-2
            // animations the anim-strip walk plays, the draw-time
            // remap the base tile swaps). The five-log boxes' `#log`
            // and `#lit_log` refs the mesher's own lane reads (the
            // per-quad texture verdict the kind's tile mapping keeps),
            // tinted = false (no tintindex anywhere).
            let stem = if block == "campfire_lit" {
                "campfire"
            } else {
                "soul_campfire"
            };
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{stem}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {stem}: json parse"))?;
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get("facing=south,lit=true"))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate {stem}: no facing=south,lit=true model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {stem}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("fire")
                .with_context(|| format!("model {model_ref}: no fire texture ref"))?;
            let tex_stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&tex_stem, false));
            continue;
        }
        if block == "candle_cake" || block == "candle_cake_lit" {
            // THE CANDLE CAKE KINDS' OWN ROWS (the autopush-523 round,
            // the official 26.2 jar): the blockstate is a plain
            // VARIANTS map over lit — the UNLIT kind the "lit=false"
            // row resolves (block/candle_cake), the LIT kind the
            // "lit=true" row reads (block/candle_cake_lit), BOTH
            // parents block/template_cake_with_candle. The flat row
            // the `side` ref banks (block/cake_side — the template's
            // own particle the cake body's dominant texture speaks);
            // the multi-texture template's #bottom/#top/#candle refs
            // the MESHER's own per-quad verdict reads (the cake
            // masters the appended tiles bank, the candle tiles the
            // palette's candle rows keep). tinted = false (the faces
            // carry NO tintindex — the pre-colored masters render as
            // authored).
            let lit_row = if block == "candle_cake_lit" {
                "lit=true"
            } else {
                "lit=false"
            };
            let bs_bytes = read_jar(
                &mut archive,
                "assets/minecraft/blockstates/candle_cake.json",
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| "blockstate candle_cake: json parse")?;
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(lit_row))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate candle_cake: no {lit_row} model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block candle_cake: model chain {model_ref}"))?;
            let tex = tex_map
                .get("side")
                .with_context(|| format!("model {model_ref}: no side texture ref"))?;
            let tex_stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&tex_stem, false));
            continue;
        }
        if block == "wheat" || block.starts_with("wheat_stage") {
            // THE FIFTEENTH AUDIT + THE GROWTH ROUND (official 26.2 jar):
            // the blockstate is a plain variants map (age=0..7) — "wheat"
            // resolves the age=0 row and "wheat_stageN" the age=N row,
            // each -> block/wheat_stageN -> block/crop (the SAME four
            // zero-thickness vertical plane elements — x=4, x=12, z=4,
            // z=12, each y -1..15 with `shade: false` faces and
            // model-level `ambientocclusion: false`), so resolve_block's
            // full-cube six-face requirement cannot apply. The stage
            // texture stem resolves through the REAL chain (the
            // blockstate variant's model ref -> the merged texture map's
            // `crop` ref), tinted = false (the faces carry NO tintindex —
            // the pre-colored masters render as authored). The AGE
            // LADDER the client's kinds wear: WHEAT = age 0,
            // WHEAT_STAGE1..7 = ages 1..7 — the growth walk's own art.
            let age: u8 = if block == "wheat" {
                0
            } else {
                block
                    .trim_start_matches("wheat_stage")
                    .parse()
                    .with_context(|| format!("palette {block}: bad stage number"))?
            };
            let bs_bytes = read_jar(&mut archive, "assets/minecraft/blockstates/wheat.json")?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| "blockstate wheat: json parse")?;
            let age_key = format!("age={age}");
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(&age_key))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate wheat: no {age_key} model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block wheat: model chain {model_ref}"))?;
            let tex = tex_map
                .get("crop")
                .with_context(|| format!("model {model_ref}: no crop texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if matches!(
            block,
            "dandelion" | "poppy" | "azure_bluet" | "oxeye_daisy" | "cornflower"
                | "orange_tulip" | "red_tulip" | "pink_tulip" | "white_tulip"
        ) {
            // THE PLAINS' OWN FLOWER PATCH (the autopush-411 round, the
            // official 26.2 jar; the FULL PALETTE the autopush-414
            // round wears — the noise_threshold_provider's own
            // high_states/low_states rows the RUNTIME_PALETTE banks):
            // the blockstate is a single variant map ("") — the model
            // ref -> block/cross (the TWO diagonal plane elements the
            // 45° rotY + rescale fold bakes, `shade: false` faces under
            // model-level `ambientocclusion: false`), so resolve_block's
            // full-cube six-face requirement cannot apply. The texture
            // stem resolves through the REAL chain (the blockstate
            // variant's model ref -> the merged texture map's `cross`
            // ref — dandelion -> block/dandelion, poppy ->
            // block/poppy, each new flower its own block/<name> row),
            // tinted = false (the faces carry NO tintindex — the
            // pre-colored masters render as authored; the audited
            // BlockColors register walk the flower family never covers).
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{block}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {block}: json parse"))?;
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(""))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate {block}: no empty-variant model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("cross")
                .with_context(|| format!("model {model_ref}: no cross texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block == "short_grass" || block == "bush" {
            // THE PLAINS' OWN GRASS TUFT (the autopush-412 round, the
            // official 26.2 jar): the blockstate is a single variant
            // map ("") — the model ref -> block/short_grass (bush ->
            // block/bush) whose PARENT is block/tinted_cross: the
            // flowers' own block/cross bake (the two diagonal plane
            // elements, the 45° rotY + rescale fold, `shade: false`
            // faces under model-level `ambientocclusion: false`) PLUS
            // the tint — the tinted_cross faces carry `tintindex: 0`
            // (the audited model json), so the grayscale master rides
            // the TINT_GRASS bake (tint_for_texture's row) and the
            // resolved faces record `tinted = true` (the client's
            // biome fold the no-op rule keeps). The texture stem
            // resolves through the REAL chain (the blockstate
            // variant's model ref -> the merged texture map's `cross`
            // ref — short_grass -> block/short_grass, bush ->
            // block/bush).
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{block}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {block}: json parse"))?;
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(""))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate {block}: no empty-variant model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("cross")
                .with_context(|| format!("model {model_ref}: no cross texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            // the tinted_cross parent's own verdict — the faces carry
            // tintindex 0, the master the TINT_GRASS bake wears
            debug_assert!(tint_for_texture(&stem) == TINT_GRASS);
            faces.push(resolve_flat_stem(&stem, true));
            continue;
        }
        if block == "tall_grass" || block == "tall_grass_upper" || block == "large_fern"
            || block == "large_fern_upper"
        {
            // THE PLAINS' OWN DOUBLE PLANTS (the autopush-413 round, the
            // official 26.2 jar): the blockstate is a HALF-KEYED variant
            // map ("half=lower" -> block/tall_grass_bottom, "half=upper"
            // -> block/tall_grass_top; large_fern the same chain's own
            // rows) — the `_upper` palette name the half=upper variant
            // resolves (the base name the half=lower row keeps). Each
            // model's PARENT is block/tinted_cross: the tuft's own cross
            // bake (the two diagonal plane elements, the 45° rotY +
            // rescale fold, `shade: false` faces under model-level
            // `ambientocclusion: false`) PLUS the tint — the faces carry
            // `tintindex: 0`, the grayscale masters ride the TINT_GRASS
            // bake (tint_for_texture's row) and the resolved faces
            // record `tinted = true`. The texture stem resolves through
            // the REAL chain (the variant's model ref -> the merged
            // texture map's `cross` ref).
            let base = block
                .strip_suffix("_upper")
                .unwrap_or(&block);
            let half_key = if block.ends_with("_upper") {
                "half=upper"
            } else {
                "half=lower"
            };
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{base}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {base}: json parse"))?;
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(half_key))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate {base}: no {half_key} model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {base}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("cross")
                .with_context(|| format!("model {model_ref}: no cross texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            // the tinted_cross parent's own verdict — the faces carry
            // tintindex 0, the master the TINT_GRASS bake wears
            debug_assert!(tint_for_texture(&stem) == TINT_GRASS);
            faces.push(resolve_flat_stem(&stem, true));
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
                    let (w, h, rgba) = decode_rgba(&raw)?;
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

    // THE FARMLAND'S WET FACE — the moisture=7 variant's own model the
    // blockstates/farmland.json's second arm names (moisture=0..6 the
    // block/farmland dry arms keep, moisture=7 the block/farmland_moist
    // wet arm speaks). The REAL chain walk the generator's own law keeps
    // (blockstates -> model -> texture ref, never a hardcoded name): the
    // template_farmland element's `up` face the farmland_moist texture
    // ref resolves. APPENDED after the block tiles — the block palette's
    // tile indices stay byte-identical (the deterministic scenes keep
    // their exact pixels), the destroy stages renumber +1, the client's
    // mesher the farmland top-face verdict the named const reads.
    let wet_idx: u16 = {
        let bs_bytes = read_jar(
            &mut archive,
            "assets/minecraft/blockstates/farmland.json",
        )?;
        let bs: Value = serde_json::from_slice(&bs_bytes)
            .with_context(|| "blockstate farmland: json parse")?;
        let wet_model_ref = bs
            .get("variants")
            .and_then(|v| v.get("moisture=7"))
            .and_then(|m| m.get("model"))
            .and_then(|m| m.as_str())
            .with_context(|| "blockstate farmland: no moisture=7 arm")?
            .trim_start_matches("minecraft:")
            .to_string();
        let (tex_map, geometry) = load_model_chain(&mut archive, &wet_model_ref)
            .with_context(|| format!("block farmland: wet model chain {wet_model_ref}"))?;
        let elements = geometry
            .get("elements")
            .and_then(|e| e.as_array())
            .with_context(|| format!("model {wet_model_ref}: no elements"))?;
        let mut top_ref = None;
        for el in elements {
            if let Some(f) = el.get("faces").and_then(|f| f.as_object()) {
                if FACE_KEYS.iter().all(|k| f.contains_key(*k)) {
                    let tref = f
                        .get("up")
                        .and_then(|u| u.get("texture"))
                        .and_then(|t| t.as_str())
                        .with_context(|| {
                            format!("model {wet_model_ref}: up face without texture")
                        })?;
                    let tex = tex_map
                        .get(tref.trim_start_matches('#'))
                        .with_context(|| {
                            format!("model {wet_model_ref}: unresolved ref {tref}")
                        })?;
                    top_ref = Some(
                        tex
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/")
                            .to_string(),
                    );
                    break;
                }
            }
        }
        let stem =
            top_ref.with_context(|| format!("model {wet_model_ref}: no full-cube element"))?;
        match tile_names.iter().position(|n| *n == stem) {
            Some(i) => i as u16,
            None => {
                let raw = read_jar(
                    &mut archive,
                    &format!("assets/minecraft/textures/block/{stem}.png"),
                )
                .with_context(|| format!("texture block/{stem}.png"))?;
                let (w, h, rgba) = decode_rgba(&raw)?;
                let (w, h, rgba) = first_frame(w, h, &rgba);
                if w != 16 || h != 16 {
                    bail!("texture {stem}: expected 16x16 (first frame), got {w}x{h}");
                }
                let mut bytes = rgba;
                bake_tint(&mut bytes, tint_for_texture(&stem));
                tile_names.push(stem.clone());
                tile_bytes.push(bytes);
                (tile_names.len() - 1) as u16
            }
        }
    };

    // THE DIG LANE's destroy stages: the crack textures the vanilla
    // breaking overlay speaks (`assets/minecraft/textures/block/
    // destroy_stage_0..9.png`, the DestroyStage destructor's own 10
    // frames). APPENDED after the block tiles — the block palette's tile
    // indices stay byte-identical (the deterministic scenes keep their
    // exact pixels), the overlay lane indexes through DESTROY_TILES.
    let mut destroy_tiles: Vec<u16> = Vec::with_capacity(10);
    for stage in 0..10u16 {
        let stem = format!("destroy_stage_{stage}");
        let raw = read_jar(
            &mut archive,
            &format!("assets/minecraft/textures/block/{stem}.png"),
        )
        .with_context(|| format!("texture block/{stem}.png"))?;
        let (w, h, rgba) = decode_rgba(&raw)?;
        let (w, h, rgba) = first_frame(w, h, &rgba);
        if w != 16 || h != 16 {
            bail!("texture {stem}: expected 16x16 (first frame), got {w}x{h}");
        }
        let mut bytes = rgba;
        bake_tint(&mut bytes, 0);
        let idx = tile_names.len() as u16;
        tile_names.push(stem);
        tile_bytes.push(bytes);
        destroy_tiles.push(idx);
    }

    // THE ANIMATED STRIPS' OWN WALK (the dev-5 round's fire, the
    // autopush-521 round's fluids, the official 26.2 jar): the SIX
    // animated textures the client's world quads name — the fire
    // multipart's two frame families (block/fire_0, block/fire_1,
    // block/soul_fire_0, block/soul_fire_1) AND the fluids' still
    // masters (block/water_still, block/lava_still — the surface quads
    // the fluid lane's FACE_TILES rows carry). THE MCMETA'S OWN TRUTH,
    // PER STRIP: fire_0/soul_fire_0 the frames array [16..31, 0..15]
    // frametime default 1; fire_1/soul_fire_1 the EMPTY animation the
    // sequential rule frametime 1 serves; water_still frametime 2 the
    // sequential 0..31 serves (the 64-tick cycle); lava_still frametime
    // 2 the EXPLICIT ping-pong [0..19, 18..1] speaks (the 38-row
    // playlist, the 76-tick cycle). APPENDED after the destroy stages —
    // the block palette's tile indices stay byte-identical (the
    // deterministic scenes keep their exact pixels). Each strip banks
    // ALL frames (frame k = the strip's k-th 16x16 row block —
    // first_frame's own slice the general walk keeps); the TICK-0
    // VERDICT the goldens' own safety rides: the FLUIDS' playlists
    // OPEN at frame 0 (water sequential, lava's explicit array the
    // same head speaks) — the tick-0 remap the appended frame-00 tile
    // answers, the pixels the palette's own crop already banked, the
    // golden renders (the scenes' water) byte-identical the still
    // keeps. The FIRE strips' playlists the frame-16 head the vanilla
    // fresh atlas speaks (the mcmeta's own array — the dev-5 test
    // pins) — no golden scene rides a fire cell, the tick-0 still the
    // vanilla-faithful first row wears.
    let mut anim_strips: Vec<(String, Vec<u16>, Vec<u8>, u64)> = Vec::new();
    for stem in [
        "fire_0",
        "fire_1",
        "soul_fire_0",
        "soul_fire_1",
        "water_still",
        "lava_still",
        "campfire_fire",
        "soul_campfire_fire",
    ] {
        let raw = read_jar(
            &mut archive,
            &format!("assets/minecraft/textures/block/{stem}.png"),
        )
        .with_context(|| format!("texture block/{stem}.png"))?;
        let (w, h, rgba) = decode_rgba(&raw)?;
        if w != 16 || h == 0 || h % w != 0 {
            bail!("texture {stem}: expected the 16-wide strip, got {w}x{h}");
        }
        let frame_count = (h / w) as usize;
        let mc_bytes = read_jar(
            &mut archive,
            &format!("assets/minecraft/textures/block/{stem}.png.mcmeta"),
        )
        .with_context(|| format!("texture block/{stem}.png.mcmeta"))?;
        let mc: Value = serde_json::from_slice(&mc_bytes)
            .with_context(|| format!("texture {stem}: mcmeta parse"))?;
        let anim = mc
            .get("animation")
            .with_context(|| format!("texture {stem}: no animation key"))?;
        // THE SERIALIZER'S OWN DEFAULTS — the frametime the absent key's
        // 1 tick wears (AnimationMetadataSection.DEFAULT_FRAME_TIME),
        // the frames the absent array's sequential 0..count-1 walk
        // serves (the empty `{}` animation the full strip plays).
        let frametime = anim.get("frametime").and_then(|v| v.as_u64()).unwrap_or(1);
        if frametime == 0 {
            bail!("texture {stem}: frametime 0 refuses");
        }
        let playlist: Vec<u8> = match anim.get("frames").and_then(|v| v.as_array()) {
            Some(frames) => frames
                .iter()
                .map(|f| {
                    f.as_u64()
                        .map(|v| v as u8)
                        .with_context(|| format!("texture {stem}: bad frames row"))
                })
                .collect::<Result<Vec<_>>>()?,
            None => (0..frame_count as u8).collect(),
        };
        // THE PLAYLIST'S OWN LENGTH — the rows the DISPLAY steps speak,
        // repeats legal (lava_still's ping-pong [0..19, 18..1] the 38
        // steps over the strip's 20 frames the 26.2 jar banks): the
        // strip's frame COUNT the rows' BOUNDS alone gate, the length
        // never (the vanilla AnimationMetadataSection's own frames
        // list the length-free walk keeps).
        if playlist.iter().any(|&f| (f as usize) >= frame_count) {
            bail!("texture {stem}: playlist row beyond the strip's frames");
        }
        // THE TICK-0 VERDICT (the FLUIDS' own law) — the water/lava
        // playlists the frame-0 row open (the golden scenes' water the
        // still keeps byte-identical); the fire strips' frame-16 heads
        // the vanilla fresh atlas speaks, the mcmeta's own arrays — the
        // honest frames the vanilla plays, no golden scene rides fire.
        if (stem == "water_still" || stem == "lava_still") && playlist[0] != 0 {
            bail!("texture {stem}: the playlist's head row {head} refuses — the tick-0 still the frame 0 wears", head = playlist[0]);
        }
        // THE TINT BAKE THE PALETTE ROWS WEAR — the appended frames the
        // SAME per-texture bake carry (water_still the TINT_WATER master
        // the palette's tile bakes, the fire strips the untinted 0):
        // the tick-0 pixel identity the golden safety rides DEMANDS the
        // frame tiles the palette row's exact bytes speak, the animated
        // frames the blue water the in-world look paints.
        let strip_tint = tint_for_texture(stem);
        let row = 16 * 16 * 4;
        let mut tiles: Vec<u16> = Vec::with_capacity(frame_count);
        for k in 0..frame_count {
            let mut bytes = rgba[k * row..(k + 1) * row].to_vec();
            bake_tint(&mut bytes, strip_tint);
            let idx = tile_names.len() as u16;
            tile_names.push(format!("{stem}_frame{k:02}"));
            tile_bytes.push(bytes);
            tiles.push(idx);
        }
        anim_strips.push((stem.to_string(), tiles, playlist, frametime));
    }
    // THE LIT-LOG MASTERS' OWN STATIC BANK (the dev-6 round): the two
    // lit_log textures the template_campfire's lit faces name — 16x64
    // strips whose mcmeta carries `{"interpolate": true, "frametime":
    // 20}` (the 4-frame 80-tick cycle). THE INTERPOLATION GAP the
    // honest round keeps: the vanilla sprite animator CROSS-FADES the
    // frames the interpolate key speaks — a blend lane the atlas walk
    // never carried (the fire/fluid strips' frametime swaps the
    // step-only rule serves), so the masters bank the FRAME-0 still
    // (the ember row the cycle opens), the cross-fade a future lane's
    // own growth keeps. Named tiles the mesher's per-quad texture
    // verdict reads (the TILE_CAMPFIRE_LOG_LIT/TILE_SOUL_CAMPFIRE_
    // LOG_LIT consts the emit answers).
    for stem in ["campfire_log_lit", "soul_campfire_log_lit"] {
        if tile_names.iter().any(|n| n == stem) {
            continue; // the palette row's own bank — the dedup the walk keeps
        }
        let raw = read_jar(
            &mut archive,
            &format!("assets/minecraft/textures/block/{stem}.png"),
        )
        .with_context(|| format!("texture block/{stem}.png"))?;
        let (w, h, rgba) = decode_rgba(&raw)?;
        let (w, h, rgba) = first_frame(w, h, &rgba);
        if w != 16 || h != 16 {
            bail!("texture {stem}: expected 16x16 (first frame), got {w}x{h}");
        }
        let mut bytes = rgba;
        bake_tint(&mut bytes, tint_for_texture(stem));
        let idx = tile_names.len() as u16;
        tile_names.push(stem.to_string());
        tile_bytes.push(bytes);
        let _ = idx;
    }
    // THE CAKE MASTERS' OWN STATIC BANK (the autopush-523 round): the
    // three cake textures the template_cake_with_candle's faces name —
    // plain 16x16 stills the mcmeta never animates (the cake body the
    // candle rides). APPENDED after the palette rows — the block
    // palette's tile indices stay byte-identical (the deterministic
    // scenes keep their exact pixels). Named tiles the mesher's
    // per-quad texture verdict reads (the TILE_CAKE_BOTTOM/TILE_CAKE_
    // TOP/TILE_CAKE_SIDE consts the emit answers); the candle tiles
    // the lane reads the PALETTE's own candle rows keep (no append —
    // the same block/candle and block/candle_lit masters the kind
    // tiles already bank).
    for stem in ["cake_bottom", "cake_top", "cake_side"] {
        if tile_names.iter().any(|n| n == stem) {
            continue; // the dedup the walk keeps
        }
        let raw = read_jar(
            &mut archive,
            &format!("assets/minecraft/textures/block/{stem}.png"),
        )
        .with_context(|| format!("texture block/{stem}.png"))?;
        let (w, h, rgba) = decode_rgba(&raw)?;
        if w != 16 || h != 16 {
            bail!("texture {stem}: expected 16x16, got {w}x{h}");
        }
        let mut bytes = rgba;
        bake_tint(&mut bytes, tint_for_texture(stem));
        let idx = tile_names.len() as u16;
        tile_names.push(stem.to_string());
        tile_bytes.push(bytes);
        let _ = idx;
    }
    // THE BASE TILES the quads bake: the four palette-named strips the
    // BLOCK palette's own rows keep (the "fire_0" / "soul_fire_0" /
    // "water_still" / "lava_still" tiles the FACE_TILES rows bank — the
    // existing law byte-identical), the two *_1 fire families the
    // appended frame-00 tiles speak (the remap's own bases).
    let strip_base = |stem: &str| -> Result<u16> {
        match tile_names.iter().position(|n| n == stem) {
            Some(i) => Ok(i as u16),
            None => anim_strips
                .iter()
                .find(|(s, _, _, _)| s == stem)
                .map(|(_, t, _, _)| t[0])
                .with_context(|| format!("strip {stem}: neither palette row nor appended frames")),
        }
    };

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
         pub const REGISTRY_NAMES: [&str; N_KINDS] = {:?};\n\n\
         pub const FACE_TILES: [[u16; 6]; N_KINDS] = [\n",
        RUNTIME_PALETTE.len(),
        RUNTIME_PALETTE,
        registry_names()
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
    src.push_str(&format!(
        "\n/// THE FARMLAND'S WET FACE — the moisture=7 variant's own tile the\n\
         \x20    /// blockstates/farmland.json's second arm resolves (block/farmland_moist\n\
         \x20    /// -> the farmland_moist texture the template_farmland's up face names).\n\
         \x20    /// The mesher's farmland top-face verdict the named const reads.\n\
         \x20    pub const TILE_FARMLAND_MOIST: u16 = {};\n",
        wet_idx
    ));
    // THE MESHER LANES' OWN TILE CONSTS (the dev-6 round's campfire
    // logs, the autopush-523 round's cake masters): the named tiles
    // the mesher's per-quad texture verdicts read (the fire quads'
    // bases the strip tables' FRAME0 consts answer — the log and cake
    // masters ride no strip). The names the tile_names walk banks
    // (the palette arms' resolve walk speaks); the absent name the
    // honest error keeps (the lane's const the walk's own guarantee
    // demands).
    let tile_const = |tile_names: &Vec<String>, stem: &str| -> Result<u16> {
        tile_names
            .iter()
            .position(|n| n == stem)
            .map(|i| i as u16)
            .with_context(|| format!("tile {stem}: absent from the banked tiles"))
    };
    for (stem, prefix) in [
        ("campfire_log", "TILE_CAMPFIRE_LOG"),
        ("campfire_log_lit", "TILE_CAMPFIRE_LOG_LIT"),
        ("soul_campfire_log_lit", "TILE_SOUL_CAMPFIRE_LOG_LIT"),
    ] {
        let idx = tile_const(&tile_names, stem)?;
        src.push_str(&format!(
            "\n/// The campfire lane's own {} tile (the dev-6 round — the\n\
             \x20    /// five-log geometry's per-quad texture verdict reads it;\n\
             \x20    /// the {} ref the merged texture map answers).\n\
             \x20    pub const {}: u16 = {};\n",
            stem,
            if prefix.ends_with("LOG_LIT") {
                if stem.starts_with("soul") { "#lit_log (soul)" } else { "#lit_log" }
            } else {
                "#log"
            },
            prefix,
            idx
        ));
    }
    for (stem, prefix) in [
        ("cake_bottom", "TILE_CAKE_BOTTOM"),
        ("cake_top", "TILE_CAKE_TOP"),
        ("cake_side", "TILE_CAKE_SIDE"),
    ] {
        let idx = tile_const(&tile_names, stem)?;
        src.push_str(&format!(
            "\n/// The candle-cake lane's own {} tile (the autopush-523\n\
             \x20    /// round — the template_cake_with_candle geometry's\n\
             \x20    /// per-quad texture verdict reads it; the candle faces\n\
             \x20    /// the palette's own candle tiles answer).\n\
             \x20    pub const {}: u16 = {};\n",
            stem, prefix, idx
        ));
    }
    src.push_str(&format!(
        "\n/// The destroy_stage_0..9 tile indices (the breaking overlay's own\n\
         \x20    /// crack frames; the vanilla `DestroyStage.getDestructor` law maps\n\
         \x20    /// the progress to the index). Appended after the block tiles.\n\
         \x20    pub const DESTROY_TILES: [u16; 10] = {:?};\n",
        destroy_tiles
    ));
    // THE ANIMATED STRIPS' OWN TABLES (the dev-5 round's fire, the
    // autopush-521 round's fluids): per-strip constants the draw-time
    // remap walks — the frame-tile arrays, the mcmeta playlists, the
    // per-strip frametime (the single-constant law the fire-only lane
    // wore can't serve the fluids' frametime 2 — the per-strip truth
    // the mcmeta itself speaks), the base tile the quads bake. The
    // generated names the palette order carries: the const prefix the
    // strip's own stem maps (fire_0 -> FIRE_0 ... water_still -> WATER,
    // lava_still -> LAVA — the names the client's walks read).
    let prefixes: [(&str, &str); 8] = [
        ("fire_0", "FIRE_0"),
        ("fire_1", "FIRE_1"),
        ("soul_fire_0", "SOUL_FIRE_0"),
        ("soul_fire_1", "SOUL_FIRE_1"),
        ("water_still", "WATER"),
        ("lava_still", "LAVA"),
        ("campfire_fire", "CAMPFIRE_FIRE"),
        ("soul_campfire_fire", "SOUL_CAMPFIRE_FIRE"),
    ];
    src.push_str(
        "\n/// THE ANIMATED STRIPS' OWN TABLES (the dev-5 round's fire, the\n\
         \x20    /// autopush-521 round's fluids): per-strip constants the draw-time\n\
         \x20    /// remap walks. Each strip banks ALL its frames (frame k = the\n\
         \x20    /// strip's k-th 16x16 row block; the palette-named strips'\n\
         \x20    /// frame-0 PIXELS the block palette's own tiles already carry —\n\
         \x20    /// the appended rows the independent index space keep), the\n\
         \x20    /// playlists the mcmeta's own frames arrays (the empty/absent\n\
         \x20    /// animation the vanilla sequential rule serves), the frametime\n\
         \x20    /// the mcmeta's own key the per-strip truth speaks (the absent\n\
         \x20    /// key the serializer's default 1 tick wears). The base tiles\n\
         \x20    /// the quads bake: the palette-named strips the block palette's\n\
         \x20    /// own rows, the fire *_1 families the appended frame-00 tiles —\n\
         \x20    /// anim_tile the draw-time remap answers (the FLUIDS' playlists the\n\
         \x20    /// frame-0 row open — the tick-0 answer the pixels the palette's\n\
         \x20    /// crop carries, the golden renders byte-identical the still keeps;\n\
         \x20    /// the fire strips' frame-16 heads the vanilla fresh atlas speaks).\n",
    );
    for (stem, prefix) in &prefixes {
        let (_, tiles, playlist, frametime) = anim_strips
            .iter()
            .find(|(s, _, _, _)| s == stem)
            .with_context(|| format!("strip {stem}: absent from the anim walk"))?;
        let base = strip_base(stem)?;
        src.push_str(&format!(
            "    pub const {prefix}_FRAME0: u16 = {base};\n\
             \x20    pub const {prefix}_FRAMETIME: u64 = {frametime};\n\
             \x20    pub const {prefix}_FRAMES: [u16; {nf}] = {tiles:?};\n\
             \x20    pub const {prefix}_PLAYLIST: [u8; {np}] = {playlist:?};\n",
            nf = tiles.len(),
            np = playlist.len(),
        ));
    }
    src.push_str(
        "    /// THE ANIMATION'S DRAW-TIME REMAP — the base frame-0 tile the\n\
         \x20    /// mesher bakes (the strip's own palette row or appended frame-00)\n\
         \x20    /// the current animation frame's tile the tick answers: the\n\
         \x20    /// playlist position `(tick / frametime) % len` the vanilla\n\
         \x20    /// TextureAtlasSprite animator's per-tick walk speaks, the six\n\
         \x20    /// base tiles their own strips keep, every other tile the\n\
         \x20    /// identity wears (the destroy stages, the farmland's wet face,\n\
         \x20    /// the whole block palette).\n\
         \x20    pub fn anim_tile(base: u16, tick: u64) -> u16 {\n\
         \x20        match base {\n",
    );
    for (_, prefix) in &prefixes {
        src.push_str(&format!(
            "        {prefix}_FRAME0 => {{\n\
             \x20            let pos = ((tick / {prefix}_FRAMETIME) % {prefix}_PLAYLIST.len() as u64) as usize;\n\
             \x20            {prefix}_FRAMES[{prefix}_PLAYLIST[pos] as usize]\n\
             \x20        }}\n"
        ));
    }
    src.push_str(
        "        _ => base,\n\
         \x20    }\n\
         }\n",
    );
    Ok(src)
}

/// The biome colormap PNGs the vanilla grass/foliage/dry-foliage lookups
/// sample: `assets/minecraft/textures/colormap/{grass,foliage,dry_foliage}.png`
/// from the jar. THE MIGRATION'S FOURTH LANE: the generator copies them
/// into the runtime cache VERBATIM (assets-cache/jar/assets/minecraft/
/// textures/colormap/ — the write_atlas walk), the client's loader
/// (`mc-client/src/colormap.rs`) the only decoder — it samples the grids
/// with the verbatim `ColorMapColorUtil.get` arithmetic (javap-audited,
/// 26.2; the dry foliage audit: `DryFoliageColor.get` delegates the SAME
/// util with default -10732494, and `Biome.getDryFoliageColorFromTexture`
/// carries the same caller-side Mth.clamp as its foliage sibling).
pub const COLORMAP_STEMS: [&str; 3] = ["grass", "foliage", "dry_foliage"];

/// Committed fallback source: compiles everywhere, maps every face to
/// TILE_NONE so the renderer uses its palette path until the real atlas
/// is generated from the jar.
#[cfg_attr(not(test), expect(dead_code))]
pub fn fallback_source(version: &str) -> String {
    format!(
        "// @generated fallback — run `cargo run -p mc2rs-gen -- atlas --version {version}`\n\
         // against the official jar to replace this with real texture data.\n\
         pub const ATLAS_VERSION: &str = \"{version}\";\n\
         pub const TILE_SIZE: usize = 16;\n\
         pub const TILE_NONE: u16 = u16::MAX;\n\
         pub const N_KINDS: usize = {};\n\
         pub const KIND_NAMES: [&str; N_KINDS] = {:?};\n\
         pub const REGISTRY_NAMES: [&str; N_KINDS] = {:?};\n\
         pub const FACE_TILES: [[u16; 6]; N_KINDS] = [[TILE_NONE; 6]; N_KINDS];\n\
         pub const TINTS: [[u32; 6]; N_KINDS] = [[0; 6]; N_KINDS];\n\
         pub static TILES: [&[u8]; 0] = [];\n\
         pub static TILE_NAMES: [&str; 0] = [];\n\
         pub fn has_tiles() -> bool {{ !TILES.is_empty() }}\n\
         pub fn tile_rgba(idx: u16) -> &'static [u8] {{ TILES[idx as usize] }}\n\
         pub fn tile_name(idx: u16) -> &'static str {{ TILE_NAMES[idx as usize] }}\n",
        RUNTIME_PALETTE.len(),
        RUNTIME_PALETTE,
        registry_names()
    )
}

/// THE REGISTRY NAME'S OWN WORD — the dig/drop/sound/item lanes' row
/// both halves speak: the half-kind kinds the palette carries (the
/// `_upper` suffix the blockstate's half=upper variant resolves) map
/// to the BASE block name the jar's registry rows answer (the
/// tall_grass/large_fern chains the instabreak + replaceable rows
/// wear — the two loot pools the identical walk keeps, the grass
/// sound row both halves speak). Every other kind the base name IS
/// the palette name.
fn registry_names() -> [&'static str; RUNTIME_PALETTE.len()] {
    let mut names = [""; RUNTIME_PALETTE.len()];
    for (i, name) in RUNTIME_PALETTE.iter().enumerate() {
        names[i] = name
            .strip_suffix("_upper")
            .or_else(|| name.strip_suffix("_lit"))
            .unwrap_or(name);
    }
    names
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
        // the FIRST non-cube member (the 13th audit): the dry fold's sole
        // consumer, meshed as flat ground quads by the client
        assert_eq!(RUNTIME_PALETTE[25], "leaf_litter");
        // the SECOND fluid member (the lava lane): the still-texture
        // special case the water arm speaks, tint-free
        assert_eq!(RUNTIME_PALETTE[26], "lava");
        // the SECOND non-cube member (the 15th audit): the crop's four
        // plane elements the client's wheat path meshes
        assert_eq!(RUNTIME_PALETTE[29], "wheat");
        // the AGE LADDER (the growth round): the blockstate's own stage
        // rows the kinds wear — the same block/crop elements, the
        // wheat_stageN master each row's own texture speaks
        assert_eq!(RUNTIME_PALETTE[30], "wheat_stage1");
        assert_eq!(RUNTIME_PALETTE[31], "wheat_stage2");
        assert_eq!(RUNTIME_PALETTE[32], "wheat_stage3");
        assert_eq!(RUNTIME_PALETTE[33], "wheat_stage4");
        assert_eq!(RUNTIME_PALETTE[34], "wheat_stage5");
        assert_eq!(RUNTIME_PALETTE[35], "wheat_stage6");
        assert_eq!(RUNTIME_PALETTE[36], "wheat_stage7");
        // the crafting table's own kind (the autopush-328 round): the
        // plain opaque cube the full-cube face walk resolves (front/
        // side/top + the oak_planks down face the vanilla model speaks)
        assert_eq!(RUNTIME_PALETTE[37], "crafting_table");
        // the grass tuft's own kinds (the autopush-412 round): the
        // tinted-cross family the TINT_GRASS bake wears — the
        // patch_grass_plain short_grass row and the patch_bush bush row
        assert_eq!(RUNTIME_PALETTE[41], "short_grass");
        assert_eq!(RUNTIME_PALETTE[42], "bush");
        // the double plants (the autopush-413 round): the half-keyed
        // rows the placeAt pair wears
        assert_eq!(RUNTIME_PALETTE[43], "tall_grass");
        assert_eq!(RUNTIME_PALETTE[44], "tall_grass_upper");
        assert_eq!(RUNTIME_PALETTE[45], "large_fern");
        assert_eq!(RUNTIME_PALETTE[46], "large_fern_upper");
        // the noise_threshold_provider's own full palette (the
        // autopush-414 round): the high_states rows the [poppy,
        // azure_bluet, oxeye_daisy, cornflower] list order keeps (the
        // poppy first row the kind 40 already banks), the low_states
        // the [orange_tulip, red_tulip, pink_tulip, white_tulip] rows
        // — the provider's own json order the indices wear
        assert_eq!(RUNTIME_PALETTE[47], "azure_bluet");
        assert_eq!(RUNTIME_PALETTE[48], "oxeye_daisy");
        assert_eq!(RUNTIME_PALETTE[49], "cornflower");
        assert_eq!(RUNTIME_PALETTE[50], "orange_tulip");
        assert_eq!(RUNTIME_PALETTE[51], "red_tulip");
        assert_eq!(RUNTIME_PALETTE[52], "pink_tulip");
        assert_eq!(RUNTIME_PALETTE[53], "white_tulip");
        // the candle-cake pair (the autopush-523 round): the canLight
        // consult's third row the two igniter arms' disjunction names
        assert_eq!(RUNTIME_PALETTE[65], "candle_cake");
        assert_eq!(RUNTIME_PALETTE[66], "candle_cake_lit");
        assert_eq!(FACE_KEYS, ["east", "west", "up", "down", "south", "north"]);
    }

    #[test]
    fn tint_map_covers_grayscale_masters() {
        assert_eq!(tint_for_texture("grass_block_top"), TINT_GRASS);
        assert_eq!(tint_for_texture("oak_leaves"), TINT_FOLIAGE);
        assert_eq!(tint_for_texture("water_still"), TINT_WATER);
        // the dry analog (13th audit): the REAL dry_foliage grid at the
        // plains climate (0.8, 0.4) — probed 0xA37546 in the 12th-audit run
        assert_eq!(tint_for_texture("leaf_litter"), TINT_DRY);
        // the grass tuft's own row (the autopush-412 round): the
        // tinted_cross masters the BlockTintSources.grass() source
        // tints — the SAME TINT_GRASS the grass_block_top row speaks
        assert_eq!(tint_for_texture("short_grass"), TINT_GRASS);
        assert_eq!(tint_for_texture("bush"), TINT_GRASS);
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

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
///
/// cake is kind 67 and cake_slice1..6 kinds 68..73 (the
/// sixty-eighth..seventy-fourth entries, the autopush-524 round): THE
/// CAKE KIND ITSELF the eat lane the useWithoutItem walk serves and
/// the palette has never banked (javap the 26.2 jar — CakeBlock's
/// ctor the DEFAULT state registers BITES=0, the registration the
/// blocks clinit's BlockItemIds.CAKE row banks: `forceSolidOn()`
/// the pushReaction the pc 11239-11262 walk wears, `strength(0.5f)`
/// the ldc 0.5f row the dig lane answers, `sound(SoundType.WOOL)`
/// the WOOL row the sounds table speaks — and the register(
/// BlockItemIds.CAKE, ...) walk CARRIES the item id: the block the
/// ITEM FORM HAS, the placeable lookup the reverse walk answers,
/// the pick lane the getCloneItemStack's CAKE item grants). The
/// BITES property 0..6 the SEVEN palette kinds the kind-only
/// vocabulary wears (the wheat_stageN ladder the same law keeps —
/// each slice the bite's own stage speaks). The SHAPE the static
/// init's `Block.boxes(MAX_BITES + 1, lambda$static$0)` walk banks —
/// the lambda's `Block.box(1 + bites*2, 0, 1, 15, 8, 15)` the
/// x-start walk the per-slice model boxes exactly mirror (the
/// per-cell fold the Column(8) row the cake band answers, the
/// x-narrowing the documented gap keeps). The blockstate is a
/// plain VARIANTS map over bites — "bites=0" -> block/cake,
/// "bites=N" -> block/cake_sliceN, each a SINGLE box element
/// [1+2N, 0, 1]->[15, 8, 15] (#bottom/#top/#side the cake masters
/// name, the west face #inside the bitten cut speaks — block/
/// cake_inner the bank's own fourth tile answers). The eat lane
/// the CakeBlock.eat walk serves: `canEat(false)` the gate, the
/// `FoodData.eat(2, 0.1F)` the bite's own feed, the bites<6
/// setBlock(flag 3) the ladder's own step, the max-bite
/// removeBlock the AIR the cell returns — the candle-on-cake arm
/// the useItemOn walk the CANDLES tag + bites==0 consult answers
/// (the CAKE_ADD_CANDLE voice the swap plays).
pub const RUNTIME_PALETTE: [&str; 756] = [
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
    "cake", "cake_slice1", "cake_slice2", "cake_slice3",
    "cake_slice4", "cake_slice5", "cake_slice6",
    // THE SIGNAL FIRE'S OWN KIND (the autopush-530 round): hay_block
    // the campfire's below-cell the isSmokeSource verdict reads (javap
    // CampfireBlock pc 229-230 — the `state.is(Blocks.HAY_BLOCK)` the
    // SIGNAL_FIRE property's own law speaks), the palette row the
    // offline signal-fire lane the kind-only vocabulary now carries.
    "hay_block",
    // THE TOOL WAKE'S OWN PAIR (the dev-22 round): the cobweb the
    // SWORD/SHEARS pinned rows' first palette-native consumer — javap
    // Blocks clinit pc 4221-4268 the 26.2 jar: register(
    // BlockItemIds.COBWEB, ...) the ITEM FORM HAS (the placeable
    // lookup the reverse walk answers), mapColor(WOOL) the pc 4232
    // row, sound(SoundType.COBWEB) pc 4238-4241, forceSolidOn()
    // pc 4244, noCollision() pc 4247, requiresCorrectToolForDrops()
    // pc 4250 the dig row's own true the generated table speaks,
    // strength(4.0f) pc 4253-4256, pushReaction(DESTROY) pc 4259-
    // 4262 — the cross model the blockstate's single "" variant
    // resolves (block/cobweb parent block/cross), the cobweb arm the
    // resolve walk's cross family reads; the WHITE WOOL the
    // ColorCollection.registerBlocks walk's own member (javap Blocks
    // pc 4725-4746 — the 16 colors one lambda family) the full-cube
    // generic walk resolves (blockstates white_wool.json "" ->
    // block/white_wool parent block/cube_all), sound(WOOL) the wool
    // lambda (encourage 30 / burn 60 the bootStrap$0 pair the fire
    // tables bank) the wool sounds' own rows keep, no
    // requiresCorrectToolForDrops the dig row's false the bare-hand
    // harvest wears.
    "cobweb", "white_wool",
    // THE SPEED FACTOR WAKE'S OWN ROW (the autopush-598 round): the
    // soul sand the speedFactor(0.4f) clinit row's first palette-native
    // consumer (javap Blocks pc 10667-10670 the 26.2 jar — the
    // Entity.getBlockSpeedFactor walk the body's move tail reads, the
    // Block.column(16.0, 0.0, 14.0) SHAPE the collision's Column(14)
    // row keeps; the makeStuckInBlock family the older versions
    // carried retired — the SoulSandBlock the entityInside override
    // never carries), the full-cube generic walk resolves (blockstates
    // soul_sand.json "" -> block/soul_sand parent block/cube_all), the
    // strength(0.5f) pc 10662-10664 the dig row's own no-demand arm
    // speaks, NO requiresCorrectToolForDrops the bare-hand harvest
    // the dig row's false wears
    "soul_sand",
    // THE HONEY WAKE'S OWN ROW (the autopush-599 round): the honey
    // block the speedFactor(0.4f) + jumpFactor(0.5f) rows' first
    // palette-native consumer (javap Blocks pc 26860-26900 the 26.2
    // jar — the generated block_factors table the rows already bank,
    // the body's move tail + jump fold the wake finally feeds), the
    // NO-strength default the instabreak dig row keeps (the generated
    // 0.0 the bootstrapped registry speaks), noOcclusion() pc 26888
    // the non-occluding verdict keeps (the is_transparent row the
    // neighbors' faces the honey against draws), the TWO-ELEMENT model
    // the element walk resolves (blockstates "" -> block/honey_block
    // — the full outer cube the #down texture carries + the 1..15
    // inset cube the #up/#side/#down skins wear, the textures'
    // UNIFORM partial alpha the per-texel translucent arm composites:
    // top 191 / side 189 / bottom 129 the PNG alpha maps bank — the
    // double-layer stack the back-to-front submit order serves), the
    // SoundType.HONEY_BLOCK row the sounds' own lane banks
    "honey_block",
    // THE FURNACE'S OWN PAIR (the autopush-601 round): the smelter the
    // dev-24 GUI lane's own block finally renders — javap Blocks clinit
    // pcs 7156-7199 the 26.2 jar: register(BlockItemIds.FURNACE, ...)
    // mapColor(STONE) pc 7170, instrument(BASEDRUM) pc 7173,
    // requiresCorrectToolForDrops() pc 7179 the dig row's own true the
    // generated table speaks, strength(3.5f) pc 7182-7185, bipush 13
    // litBlockEmission(I) pc 7188-7190 the lightLevel row the lit
    // state's own 13 banks (the unlit the 0 — light.rs's row keeps).
    // The full-cube generic walk resolves (blockstates furnace.json's
    // facing=north rows — block/furnace / block/furnace_on, the model
    // chain block/orientable -> block/orientable_with_bottom ->
    // block/cube the north=#front authoring keeps), the facing
    // property the variant byte carries (world_view's furnace_variant
    // decode), the mesher's front-tile swap the facing serves
    "furnace", "furnace_lit",
    // THE BLAST FURNACE / SMOKER QUARTET (the dev-26 round): the
    // furnace family's own two siblings the same orientable lane
    // renders — javap Blocks clinit the 26.2 jar: SMOKER register
    // pcs 24209-24252 (mapColor(STONE) pc 24220-24223, instrument(
    // BASEDRUM) pc 24226-24229, requiresCorrectToolForDrops() pc
    // 24232, strength(3.5f) pc 24235-24238, bipush 13
    // litBlockEmission(I) pc 24241-24246), BLAST_FURNACE register
    // pcs 24255-24298 (the same walk, pc 24266-24292) — the furnace's
    // exact property pair both keep. The blockstates blast_furnace.json
    // / smoker.json facing=north rows resolve (block/blast_furnace,
    // block/blast_furnace_on / block/smoker, block/smoker_on — the
    // smoker the orientable_with_bottom chain the #bottom texture
    // speaks), the facing property the variant byte carries
    // (world_view's orientable_variant decode), the mesher's
    // front-tile swap the facing serves; the lit fronts the ANIMATED
    // strips the .mcmeta files bank (the furnace's own frame-0 law)
    "blast_furnace", "blast_furnace_lit",
    "smoker", "smoker_lit",
    // THE PORTAL'S OWN ROW (the autopush-605 round): the gate the
    // FACTORY register walk serves (javap Blocks clinit pc 11083-11126
    // the 26.2 jar — register(ResourceKey, lambda$static$218, props)
    // the NO-BlockItemId overload the item-less block wears), the
    // noCollision() pc 11091 + strength(-1.0f) pc 11097 + GLASS sound
    // pc 11103 + the lightLevel lambda$static$143 bipush 11 the
    // light.rs row keeps. The blockstates nether_portal.json's axis
    // variants resolve (axis=x -> block/nether_portal_ns, axis=z ->
    // block/nether_portal_ew — the thin-plane models the ONE element
    // the #portal texture two faces carry, the full-cube walk's six-
    // face demand the element rows never speak), the flat-stem arm
    // the palette row resolves (the strip's frame-0 crop the FACE_
    // TILES row banks, the mesher's axis lane the element walk the
    // real geometry serves), the block/nether_portal.png the 16x512
    // 32-frame sequential strip the anim walk banks (the mcmeta's
    // empty animation the frametime-1 cadence wears), the NETHER_
    // PORTAL strip consts the draw-time remap reads
    "nether_portal",
    // THE FRAME'S OWN ROW (the autopush-606 round): the obsidian the
    // portal frame-scan lane's own BUILDING BLOCK the player places —
    // javap Blocks clinit pc 6491-6524 the 26.2 jar: register(
    // BlockItemIds.OBSIDIAN, of().mapColor(COLOR_BLACK) pc 6497-6500 +
    // instrument(BASEDRUM) pc 6503-6506 + requiresCorrectToolForDrops()
    // pc 6509 + strength(50.0f, 1200.0f) pc 6512-6518) the ITEM FORM
    // HAS — the placeable lookup the reverse walk answers, the dig row
    // (50.0, true) the generated table banks, the sounds' STONE family
    // the generated rows keep, the NO sound()/lightLevel call the
    // emission-0 default the light grid keeps, the full-cube shape the
    // BlockBehaviour default the collision's Full arm answers. The
    // blockstates obsidian.json's "" variant resolves (block/obsidian
    // parent block/cube_all — the #all texture the six faces the
    // generic walk banks, the tintless row the TINTS' 0 keeps)
    "obsidian",
    // THE STALK'S OWN ROW (the autopush-608 round): the bamboo the
    // sword's f32::MAX instant lane the palette finally wakes — javap
    // Blocks clinit pc 23144-23212 the 26.2 jar: register(
    // BlockItemIds.BAMBOO, of().mapColor(PLANT) pc 23155-23158 +
    // forceSolidOn() pc 23161 + randomTicks() pc 23164 + instabreak()
    // pc 23167 OVERRIDDEN by strength(1.0f) pc 23170-23171 the dig
    // row's own 1.0 the generated table banks + sound(BAMBOO) pc
    // 23174-23177 + noOcclusion() pc 23180 + dynamicShape() pc 23183
    // + offsetType(XZ) pc 23186-23189 the plant offset the XZ row
    // wears + ignitedByLava() pc 23192 + pushReaction(DESTROY) pc
    // 23195-23198 + isRedstoneConductor(never) pc 23201-23206) the
    // ITEM FORM HAS — the placeable lookup the reverse walk answers.
    // The NO lightLevel call the emission-0 default the light grid
    // keeps; the collision SHAPE_COLLISION the BambooStalkBlock
    // static{} pc 37-47 banks (Block.column(3.0, 0.0, 16.0) — the
    // 3/16 stalk column the offset sweep the inset-6 band contains);
    // the skylight the propagatesSkylightDown override pc 0-1 the
    // iconst_1 TRUE keeps. The blockstates bamboo.json is a MULTIPART
    // whose age arms apply FOUR overlapping stalk models each (the
    // bamboo1..4_age0 fours the same [7,0,7]->[9,16,9] box wears,
    // the side UV strips alone differ — the vanilla LEQUAL depth the
    // LAST-declared apply's framebuffer verdict speaks, the mesher's
    // LESS raster the REVERSED emission the same visible strip
    // serves) and the leaves arms the crossed-plane pairs — the
    // multipart the mesher's own lane selects, the palette row the
    // FIRST age0 model's #all texture (block/bamboo_stalk) banks.
    // The NO tint anywhere: the textures pre-colored (the BlockColors
    // bamboo absence the jar's own registry speaks), the TINTS' 0
    // keeps.
    "bamboo",
    // THE SAPLING'S OWN ROW (the autopush-608 round): the placement
    // law's own FIRST product — the bamboo item on plain
    // SUPPORTS_BAMBOO ground the BambooStalkBlock.getStateForPlacement
    // pc 178-184 walk serves (the BAMBOO_SAPLING.defaultBlockState()
    // return the vanilla's own quirk the bytecode pins), the sapling
    // the only block the plain-ground placement places. javap Blocks
    // clinit pc 23084-23141: register(BlockIds.BAMBOO_SAPLING, ...)
    // the NO-BlockItemId overload the item-less block the vanilla
    // itself grants (the portal arm's own law), mapColor(WOOD) pc
    // 23095-23098 + forceSolidOn() pc 23101 + randomTicks() pc 23104
    // + instabreak() pc 23107 OVERRIDDEN by strength(1.0f) pc
    // 23113-23114 + noCollision() pc 23110 the Empty collision row
    // + sound(BAMBOO_SAPLING) pc 23117-23120 + offsetType(XZ) pc
    // 23123-23126 + ignitedByLava() pc 23129 + pushReaction(DESTROY)
    // pc 23132-23135. The blockstates bamboo_sapling.json's ""
    // variant resolves (block/bamboo_sapling parent block/tinted_cross
    // — the #cross texture block/bamboo_stage0 the generic walk banks,
    // the PRE-COLORED master the TINTS' 0 keeps — the tintindex the
    // tinted_cross parent carries the resolver the BlockColors
    // absence never serves, the pixels as authored).
    "bamboo_sapling",
    // THE END'S OWN BUILDING BLOCK (the autopush-612 round): the
    // island the END world build raises — javap Blocks clinit pc
    // 13680-13713 the 26.2 jar: register(BlockItemIds.END_STONE, ...)
    // the ITEM FORM HAS (the placeable lookup the reverse walk
    // answers), mapColor(SAND) pc 13686-13689 + instrument(BASEDRUM)
    // pc 13692-13695 + requiresCorrectToolForDrops() pc 13698 the dig
    // row's own true the generated table banks + strength(3.0f, 9.0f)
    // pc 13701-13707 the (3.0, true) row speaks. The full-cube
    // generic walk resolves (blockstates end_stone.json's "" variant
    // -> block/end_stone parent block/cube_all — the one #all texture
    // block/end_stone the six faces wear), the sounds' STONE family
    // the NO sound() call the clinit arm carries (the generated rows
    // bank block.stone.break / block.stone.step), the NO
    // sound()/lightLevel call the emission-0 default the light grid
    // keeps.
    "end_stone",
    // THE DOOR FAMILY'S OWN QUARTET (the autopush-655 round): the oak
    // door the vocabulary's next frontier — javap Blocks clinit the
    // 26.2 jar: the DoorBlock register walk the BlockSetType.OAK
    // sound(WOODEN) pair banks (the wooden open/close voices the
    // runtime sounds.json carries), strength(3.0f) the dig row's own
    // 3.0 speaks, NO requiresCorrectToolForDrops the bare-hand
    // harvest the dig row's false wears. javap DoorBlock the 26.2
    // jar: the ctor's registerDefaultState walk pc 28-87 pins
    // FACING=NORTH, OPEN=false, HINGE=LEFT, POWERED=false, HALF=LOWER
    // — the hinge-left default the first lane's own placement keeps.
    // The blockstates oak_door.json is a plain 32-variant map (4
    // facing x 2 half x 2 hinge x 2 open — the 26.2 door the
    // POWERED property NEVER carries: the 64-state wiki-era ladder
    // the 32-variant table replaces), the closed rows
    // oak_door_bottom_left/top_left (y: north 270, east 0, south 90,
    // west 180 — the panel edge the OPPOSITE facing speaks: closed
    // facing=east y=0 keeps the base model's west strip x 0..3),
    // the open rows the _open models (y: north 0, east 90, south
    // 180, west 270 — the LEFT-hinge swing the edge the facing's
    // LEFT speaks: open facing=north y=0 keeps the west strip). The
    // models one box element [0,0,0]->[3,16,16] each (the lower's
    // faces all #bottom = block/oak_door_bottom, the upper's all
    // #top = block/oak_door_top — door_bottom_left/door_top_left
    // the parent chain resolves, ambientocclusion false the
    // withoutAO fold keeps), the collision SHAPES clinit's
    // Block.boxZ(16.0, 13.0, 16.0) the 3/16 panel box(0, 0, 16,
    // 16, 16, 13) speaks — the panel edge the OPPOSITE facing the
    // closed state wears (SHAPES[NORTH] = z 13..16, the south
    // edge), the model's own y rotations the SAME edges the
    // consistency check answers. The kind-only palette the four
    // offline rows banks: the closed lower/upper the placement
    // lands, the open pair the flip walk's kind swaps wear; the
    // facing property the variant byte carries (the orientable
    // vocabulary's 1..=4 law), the mesher's element lane the real
    // geometry the facing/open theta tables rotate.
    "oak_door", "oak_door_upper", "oak_door_open", "oak_door_upper_open",
    // THE DOOR FAMILY'S WOOD GENERALIZATION (the dev-43 round): the
    // eleven wooden quartets the oak lane's own walk speaks — javap
    // BlockSetType the 26.2 jar (pinned sha1 2dc72797, javap 21.0.12):
    // the single-arg convenience ctor `<init>(String)` pc 0-38 the
    // OAK/SPRUCE/BIRCH/ACACIA/JUNGLE/DARK_OAK/PALE_OAK/MANGROVE set
    // types delegate — canOpenByHand iconst_1 the PASS speaks,
    // SoundType.WOOD the dig/place rows the wooden SoundType wears,
    // WOODEN_DOOR_CLOSE/OPEN the doorOpen/doorClose events the
    // block.wooden_door.open/close voices the runtime sounds.json
    // carries (the whole classic family the ONE wooden voice wears);
    // the CHERRY set type the full ctor pc 338-380 speaks —
    // SoundType.CHERRY_WOOD, CHERRY_WOOD_DOOR_CLOSE/OPEN the
    // block.cherry_wood_door.open/close voices; the CRIMSON/WARPED
    // pair the NETHER_WOOD set type's walk banks — NETHER_WOOD_DOOR_
    // CLOSE/OPEN the block.nether_wood_door.open/close voices (the
    // full-ctor canOpenByHand iconst_1 the PASS keeps); the BAMBOO
    // set type the same full-ctor shape — BAMBOO_WOOD_DOOR_CLOSE/
    // OPEN the block.bamboo_wood_door.open/close voices. The
    // COPPER family (the eight oxidization-ladder states) and the
    // IRON door the canOpenByHand law divides (the copper iconst_1
    // the PASS, the iron iconst_0 the refusal) — the families the
    // ladder's own round banks (the copper quartets the next lane
    // wears). Each wood the SAME quartet the oak lane banks: the
    // blockstate {wood}_door.json the plain 32-variant map (4 facing
    // x 2 half x 2 hinge x 2 open — the POWERED property never
    // carried), the hinge-LEFT facing=north rows the kind-only
    // palette resolves (the closed kinds the open=false rows, the
    // open kinds the open=true rows), the models ONE box element
    // [0,0,0]->[3,16,16] each (the lower's #bottom, the upper's
    // #top — the per-wood masters the mesher's verdict reads), the
    // 3/16 panel the SHAPES clinit's boxZ walk every wood speaks.
    // The quartet seats the CONTIGUOUS tail the kind ids bank (the
    // closed lower/upper, the open lower/upper — the +1 upper bit
    // the +2 open bit the helpers' arithmetic wears), the END_PORTAL
    // kind the beyond-table seat the quartet ladder's own growth
    // displaces (94 -> 138 the dev-43 walk keeps).
    "spruce_door", "spruce_door_upper", "spruce_door_open", "spruce_door_upper_open",
    "birch_door", "birch_door_upper", "birch_door_open", "birch_door_upper_open",
    "jungle_door", "jungle_door_upper", "jungle_door_open", "jungle_door_upper_open",
    "acacia_door", "acacia_door_upper", "acacia_door_open", "acacia_door_upper_open",
    "dark_oak_door", "dark_oak_door_upper", "dark_oak_door_open", "dark_oak_door_upper_open",
    "mangrove_door", "mangrove_door_upper", "mangrove_door_open", "mangrove_door_upper_open",
    "cherry_door", "cherry_door_upper", "cherry_door_open", "cherry_door_upper_open",
    "pale_oak_door", "pale_oak_door_upper", "pale_oak_door_open", "pale_oak_door_upper_open",
    "bamboo_door", "bamboo_door_upper", "bamboo_door_open", "bamboo_door_upper_open",
    "crimson_door", "crimson_door_upper", "crimson_door_open", "crimson_door_upper_open",
    "warped_door", "warped_door_upper", "warped_door_open", "warped_door_upper_open",
    // THE DOOR FAMILY'S COPPER LADDER + IRON (the autopush-656 round):
    // the eight oxidization states the COPPER set type shares — javap
    // Blocks clinit the 26.2 jar: the four weathering copper doors the
    // lambda$static$372 factory walks (WeatheringCopperDoorBlock
    // <init>(BlockSetType.COPPER, weatherState, props) pc 0-9 the
    // shared set type speaks), the four waxed doors the
    // lambda$static$371 factory carries (DoorBlock
    // <init>(BlockSetType.COPPER, props) pc 0-8 — the plain door walk
    // the weathering the wax the halt wears); the iron door the
    // BlockSetType.IRON registration banks (canOpenByHand iconst_0
    // the refusal the set type's own field keeps — the useWithoutItem
    // walk the PASS the hand press never consumes). The voices the
    // set types' own pairs speak (the COPPER set type's
    // COPPER_DOOR_CLOSE/OPEN the block.copper_door.open/close events,
    // the IRON pair the block.iron_door.open/close events — the hand
    // never hears the iron's own walk, the wire/redstone echo the
    // wired lane serves). The blockstates the REAL chain the waxed
    // ladder serves: the waxed_copper_door.json variants the UNWAXED
    // models point (waxed_copper_door the block/copper_door_bottom_
    // left model the facing=east row speaks, the waxed_exposed/
    // weathered/oxidized the same per-state share wears) — the
    // per-oxidization textures the waxed kinds inherit, the waxed
    // models NO jar file keeps (the blockstate's own row the one
    // resolution the regen and the mesher both walk). The dig rows
    // the generated table banks (iron_door (5.0, false) the hardness
    // 5.0 speaks, the copper ladder (3.0, false) the wooden rows the
    // same shape wears), the loot tables the SAME conditional pool
    // (survives_explosion + block_state_property half=lower — the
    // {base}_door item every lower rolls), the items the registry
    // every door grants.
    "copper_door", "copper_door_upper", "copper_door_open", "copper_door_upper_open",
    "exposed_copper_door", "exposed_copper_door_upper", "exposed_copper_door_open", "exposed_copper_door_upper_open",
    "weathered_copper_door", "weathered_copper_door_upper", "weathered_copper_door_open", "weathered_copper_door_upper_open",
    "oxidized_copper_door", "oxidized_copper_door_upper", "oxidized_copper_door_open", "oxidized_copper_door_upper_open",
    "waxed_copper_door", "waxed_copper_door_upper", "waxed_copper_door_open", "waxed_copper_door_upper_open",
    "waxed_exposed_copper_door", "waxed_exposed_copper_door_upper", "waxed_exposed_copper_door_open", "waxed_exposed_copper_door_upper_open",
    "waxed_weathered_copper_door", "waxed_weathered_copper_door_upper", "waxed_weathered_copper_door_open", "waxed_weathered_copper_door_upper_open",
    "waxed_oxidized_copper_door", "waxed_oxidized_copper_door_upper", "waxed_oxidized_copper_door_open", "waxed_oxidized_copper_door_upper_open",
    "iron_door", "iron_door_upper", "iron_door_open", "iron_door_upper_open",
    // THE DOOR FAMILY'S HINGE-RIGHT TAIL (the autopush-658 round): the
    // twenty-one RIGHT quartets the LEFT ladder's own order mirrors —
    // javap DoorBlock the 26.2 jar (pinned sha1 2dc72797, javap
    // 21.0.12): the HINGE property the blockstate's plain 32-variant
    // map carries (4 facing x 2 half x 2 hinge x 2 open — the LEFT
    // rows the first lane's default pin banks, the RIGHT rows the
    // getHinge census's own verdict speaks), the placement's
    // getHinge(BlockPlaceContext) walk pc 5-108 the four-cell scan
    // wears (the ccw side cell + the ccw-above cell the LEFT pair
    // reads, the cw side cell + the cw-above cell the RIGHT pair
    // reads — the isCollisionShapeFullBlock score the full blocks
    // ±1 each speak, the LOWER-door presence checks the double-door
    // seam's own census arms), the click-location walk pcs 281-389 the
    // score==0 fallback answers (the in-cell x/z the step arms read:
    // stepX<0 the dz<0.5 RIGHT, stepX>0 the dz>0.5, stepZ<0 the
    // dx>0.5, stepZ>0 the dx<0.5 — the half-cell boundary the LEFT
    // verdict keeps), the RIGHT quartets the census's true verdict
    // lands (the blockstate hinge=right facing=north rows the
    // {base}_door_bottom_right/_open models resolve — the y tables
    // the LEFT closed [3,0,1,2] the SAME rows speak, the RIGHT open
    // north 180 / east 270 / south 0 / west 90 the [2,3,0,1] steps
    // the mesher's theta tables fold). The RIGHT kinds the LEFT
    // ladder's CONTIGUOUS tail the kind ids wear (the LEFT quartets
    // the 90..173 seats never move — the ±84 hinge mate the octet
    // walk answers), the END_PORTAL kind the beyond-table seat the
    // ladder's growth displaces (174 -> 258 the 655/643/656
    // precedent wears).
    "oak_door_right", "oak_door_upper_right", "oak_door_open_right", "oak_door_upper_open_right",
    "spruce_door_right", "spruce_door_upper_right", "spruce_door_open_right", "spruce_door_upper_open_right",
    "birch_door_right", "birch_door_upper_right", "birch_door_open_right", "birch_door_upper_open_right",
    "jungle_door_right", "jungle_door_upper_right", "jungle_door_open_right", "jungle_door_upper_open_right",
    "acacia_door_right", "acacia_door_upper_right", "acacia_door_open_right", "acacia_door_upper_open_right",
    "dark_oak_door_right", "dark_oak_door_upper_right", "dark_oak_door_open_right", "dark_oak_door_upper_open_right",
    "mangrove_door_right", "mangrove_door_upper_right", "mangrove_door_open_right", "mangrove_door_upper_open_right",
    "cherry_door_right", "cherry_door_upper_right", "cherry_door_open_right", "cherry_door_upper_open_right",
    "pale_oak_door_right", "pale_oak_door_upper_right", "pale_oak_door_open_right", "pale_oak_door_upper_open_right",
    "bamboo_door_right", "bamboo_door_upper_right", "bamboo_door_open_right", "bamboo_door_upper_open_right",
    "crimson_door_right", "crimson_door_upper_right", "crimson_door_open_right", "crimson_door_upper_open_right",
    "warped_door_right", "warped_door_upper_right", "warped_door_open_right", "warped_door_upper_open_right",
    "copper_door_right", "copper_door_upper_right", "copper_door_open_right", "copper_door_upper_open_right",
    "exposed_copper_door_right", "exposed_copper_door_upper_right", "exposed_copper_door_open_right", "exposed_copper_door_upper_open_right",
    "weathered_copper_door_right", "weathered_copper_door_upper_right", "weathered_copper_door_open_right", "weathered_copper_door_upper_open_right",
    "oxidized_copper_door_right", "oxidized_copper_door_upper_right", "oxidized_copper_door_open_right", "oxidized_copper_door_upper_open_right",
    "waxed_copper_door_right", "waxed_copper_door_upper_right", "waxed_copper_door_open_right", "waxed_copper_door_upper_open_right",
    "waxed_exposed_copper_door_right", "waxed_exposed_copper_door_upper_right", "waxed_exposed_copper_door_open_right", "waxed_exposed_copper_door_upper_open_right",
    "waxed_weathered_copper_door_right", "waxed_weathered_copper_door_upper_right", "waxed_weathered_copper_door_open_right", "waxed_weathered_copper_door_upper_open_right",
    "waxed_oxidized_copper_door_right", "waxed_oxidized_copper_door_upper_right", "waxed_oxidized_copper_door_open_right", "waxed_oxidized_copper_door_upper_open_right",
    "iron_door_right", "iron_door_upper_right", "iron_door_open_right", "iron_door_upper_open_right",
    // THE LEVER'S OWN PAIR (the dev-45 round): the toggle's own block
    // finally RENDERS and SPEAKS — javap LeverBlock + the
    // FaceAttachedHorizontalDirectionalBlock base the 26.2 jar (the
    // useWithoutItem cycle pc 7-21 the POWERED flip speaks, the
    // playSound walk pc 23-42 the LEVER_CLICK 0.3 volume / the 0.6
    // powered-true 0.5 false pitch pair banks, the updateNeighbours
    // walk pc 9-65 the lever pos + the support pos broadcasts the
    // wire voice rides), the blockstate's own FLOOR rows the two
    // kinds resolve (powered=false the "block/lever_on" model the
    // jar's own reversed authoring banks, powered=true the
    // "block/lever" model — the verbatim rows the resolution speaks),
    // the facing the variant byte carries (the door's own raw-look
    // law the placement stamps, the mesher's y-rot steps the byte
    // serves), the iron door's wire voice the signal walk answers.
    "lever", "lever_on",
    // THE BUTTON'S OWN PAIR (the autopush-660 round): the redstone
    // pulse's own block the client finally speaks — javap ButtonBlock
    // the 26.2 jar (pinned sha1 2dc72797): the press walk pc 0-47 the
    // POWERED=true setBlock flag-3 speaks + the updateNeighbours walk
    // the wire voice rides + the scheduleTick(pos, ticksToStayPressed)
    // pc 28-37 the pulse's own clock banks (the STONE lambda
    // lambda$static$338 the BlockSetType.STONE + bipush 20 ctor args
    // wear — the 20-tick 1-second pulse the tick serves) + the
    // playSound walk pc 0-26 the LevelAccessor default fconst_1 pair
    // banks (the volume 1.0 / pitch 1.0 the lever's own 0.3/0.6/0.5
    // constants the button never wears), the getSound walk the
    // BlockSetType's buttonClickOn/buttonClickOff pair speaks (the
    // block.stone_button.click_on/click_off events the stone set type
    // carries), the useWithoutItem walk pc 0-19 the already-powered
    // CONSUME verdict banks (the re-press the pass never flips), the
    // tick walk pc 0-24 the checkPressed delegate speaks (the
    // unpress the POWERED=false setBlock flag-3 + the wire voice +
    // the click-off voice + the BLOCK_DEACTIVATE vibration wears),
    // the buttonProperties walk the noCollision + strength(0.5f) +
    // pushReaction(DESTROY) rows banks (the pass-through + the dig
    // row (0.5, false) the lever's own laws wear). The blockstates
    // the DIRECT authoring the lever's REVERSED rows never share
    // (powered=false the block/stone_button model, powered=true the
    // block/stone_button_pressed — the verbatim rows the resolution
    // speaks), the face/facing rows the SAME x/y rotation laws the
    // lever's lanes wear (the floor y-only, the wall x=90 + the y
    // steps, the ceiling x=180 + the (idx+2)%4 remap), the ONE
    // #texture ref block/stone the row's tile banks (all six slots
    // the ONE stone tile — the stone master the palette already
    // banks, no new tile the regen keeps).
    "stone_button", "stone_button_pressed",
    // THE WOODEN BUTTON'S OWN RUNG (the dev-47 round — the button
    // family's wooden ladder the arrows' own activation lane arms):
    // javap ButtonBlock + BlockSetType the 26.2 jar — the OAK set the
    // String ctor's iconst_1,1,1 triple speaks (canOpenByHand +
    // canOpenByWindCharge + canButtonBeActivatedByArrows the wooden
    // sets' own triple arms — the STONE row's iconst_0 the stone set's
    // refusal the arrow lane never presses), the SoundType.WOOD row
    // the dig row (1.0-ish, the wooden SoundType) banks, the
    // WOODEN_BUTTON_CLICK_ON/OFF events the getSound walk serves (the
    // block.wooden_button.click_on/click_off events the cache's
    // sounds.json rows carry), the ticksToStayPressed the OAK
    // factory's bipush 30 ctor arg banks (the 30-tick 1.5-second
    // wooden window the stone set's 20 the STONE lambda wears). The
    // entityInside walk pc 0-41 the isClientSide gate + the
    // canButtonBeActivatedByArrows gate + the POWERED gate the
    // checkPressed delegate serves — the arrows' own press the scan
    // law arms (the getShape bounds the plate box the arrow's
    // sized(0.5f, 0.5f) bbox the EntityTypes ARROW row speaks
    // intersects). The blockstates/oak_button.json the SAME DIRECT
    // authoring the stone rows wear (powered=false the
    // block/oak_button model, powered=true the block/oak_button_pressed
    // — the face/facing rotation laws the stone lanes share), the ONE
    // #texture ref block/oak_planks the row's tile banks (all six
    // slots the ONE oak-planks tile — the planks master the palette
    // already banks, no new tile the regen keeps).
    "oak_button", "oak_button_pressed",
    // THE BUTTON FAMILY'S FULL LADDER (the autopush-662 round — the
    // twelve remaining rungs the oak lane's own walk generalizes):
    // javap the 26.2 jar the clinit's own indy→bootstrap→lambda walk
    // banks — SPRUCE/BIRCH/JUNGLE/ACACIA/CHERRY/DARK_OAK/PALE_OAK/
    // MANGROVE/BAMBOO the lambda$static$217..226 rows, the CRIMSON/
    // WARPED pair the lambda$static$323/324 rows (ALL `bipush 30`
    // ctor args the ticksToStayPressed field banks — the 30-tick
    // 1.5-second wooden window the oak rung's own law the whole
    // ladder wears), the POLISHED_BLACKSTONE rung the lambda$static
    // $338 row the STONE set type + `bipush 20` pair speaks (the
    // stone button's own 20-tick window + the STONE set's
    // block.stone_button.click_on/off voices + the arrows' refusal
    // the STONE row's full-ctor iconst_0 the shared set type arms —
    // the indy walk the BLOCKS clinit's #5794 → bootstrap → the SAME
    // lambda$static$338 body the stone button's #2916 → bootstrap 195
    // → lambda$static$131 shares — javac the identical-lambda bodies
    // the two factories the one ctor-arg shape serves). The wooden
    // sets' own voices the BlockSetType rows bank: the String-ctor
    // woods (SPRUCE/BIRCH/JUNGLE/ACACIA/DARK_OAK/PALE_OAK/MANGROVE)
    // the WOODEN_BUTTON_CLICK_ON/OFF defaults speak, the CHERRY full
    // ctor the CHERRY_WOOD_BUTTON_CLICK_ON/OFF pair (clinit pc 368-371
    // the iconst_1,1,1 triple pc 343-345 the arrows' verdict wears),
    // the CRIMSON/WARPED pair the NETHER_WOOD_BUTTON_CLICK_ON/OFF
    // rows (pc 465-468 / 514-517, the iconst_1 triples pc 438-440 /
    // 487-489), the BAMBOO row the BAMBOO_WOOD_BUTTON_CLICK_ON/OFF
    // pair (pc 579-582, the iconst_1 triple pc 552-554) — the
    // block.cherry_wood_button/bamboo_wood_button/nether_wood_button
    // click_on/off events the cache's sounds.json rows carry, the
    // ogg files the restored sound lane already banks. The blockstates
    // the SAME DIRECT authoring every rung wears (powered=false the
    // block/{wood}_button model, powered=true the block/{wood}_
    // button_pressed — the face/facing rotation laws the oak lanes
    // share), the ONE #texture ref block/{wood}_planks the row's tile
    // banks (the polished rung the block/polished_blackstone master —
    // twelve NEW tiles the first-encounter walk gains, the special
    // walks' later tiles the +12 shift the regenerated sidecar
    // constants re-bank, the NAME→PIXELS mapping byte-identical).
    "spruce_button", "spruce_button_pressed",
    "birch_button", "birch_button_pressed",
    "jungle_button", "jungle_button_pressed",
    "acacia_button", "acacia_button_pressed",
    "dark_oak_button", "dark_oak_button_pressed",
    "mangrove_button", "mangrove_button_pressed",
    "cherry_button", "cherry_button_pressed",
    "pale_oak_button", "pale_oak_button_pressed",
    "bamboo_button", "bamboo_button_pressed",
    "crimson_button", "crimson_button_pressed",
    "warped_button", "warped_button_pressed",
    "polished_blackstone_button", "polished_blackstone_button_pressed",
    // THE PRESSURE PLATES' OWN FIRST LANE (the dev-48 round — the
    // sensor family's own rung the attach floor gains): javap the
    // 26.2 jar PressurePlateBlock + BasePressurePlateBlock — the
    // Base clinit the three shapes bank (SHAPE the column(14.0, 0.0,
    // 1.0) the 1-px resting plate speaks, SHAPE_PRESSED the
    // column(14.0, 0.0, 0.5) the thinner pressed band wears, the
    // TOUCH_AABB the column(14.0, 0.0, 4.0).toAabbs().getFirst() the
    // 14/16-wide × 4/16-tall sensor slab the cell's floor wears), the
    // getPressedTime the `bipush 20` the release beat banks (the
    // 20-tick window BOTH sets wear), the checkPressed walk pc 0-168
    // the flag-2 setBlock + the updateNeighbours (the pos + the below
    // cell the wire's two-cell read) + the clickOn/clickOff voices +
    // the BLOCK_ACTIVATE/DEACTIVATE vibrations + the scheduleTick(+20)
    // the hold loop re-arms while occupied speak, the getSignal
    // Strength pc 0-79 the sensitivity switch answers (the EVERYTHING
    // row the Entity.class count arms, the MOBS row the
    // LivingEntity.class count wears — the OAK String-ctor default the
    // EVERYTHING verdict speaks, the STONE full-ctor row the
    // `iconst_1,1,0` + MOBS pair the arrows' refusal the same row
    // banks). The blockstates the ONE file BOTH states serve
    // (powered=false the block/{base} model — the 1-px element
    // [1,0,1]-[15,1,15] the pressure_plate_up parent wears, the down
    // cullface carries; powered=true the block/{base}_down — the
    // 0.5-px [1,0,1]-[15,0.5,15] band the thinner plate speaks), the
    // ONE #texture ref block/{wood}_planks / block/stone the row's
    // tile banks (the palette the tiles already bank — the plate rungs
    // no new tile the regen keeps, the pre-plate rows byte-identical).
    "oak_pressure_plate", "oak_pressure_plate_down",
    "stone_pressure_plate", "stone_pressure_plate_down",
    // THE PLATE FAMILY'S FULL LADDER (the autopush-663 round): the
    // twelve remaining rungs the oak/stone lane's own walk generalizes —
    // javap the 26.2 jar Blocks clinit the twelve plate factories the
    // indy constants wear (SPRUCE's own window pc 9509-9559 the bsm[183]
    // → lambda$static$122 body speaks, BIRCH 9561/bsm[184]→λ123, JUNGLE
    // 9613/bsm[185]→λ124, ACACIA bsm[186]→λ125, CHERRY 9717/bsm[187]→
    // λ126, PALE_OAK 9821/bsm[189]→λ128, MANGROVE bsm[190]→λ127,
    // BAMBOO 9925/bsm[191]→λ130, CRIMSON 25848/bsm[491]→λ317, WARPED
    // 25897/bsm[492]→λ318, POLISHED_BLACKSTONE 27508/bsm[521]→λ337) —
    // every row the ONE two-arg ctor shape serves (`new PressurePlate
    // Block(BlockSetType.X, props)` — the set type the sensitivity AND
    // the voices carry): the wooden woods the BlockSetType rows the
    // String ctor's EVERYTHING default wear, the CHERRY/BAMBOO/CRIMSON/
    // WARPED full-ctor rows the iconst_1,1,1 triple + the EVERYTHING
    // sensitivity + the per-wood PRESSURE_PLATE_CLICK_ON/OFF pairs bank
    // (pc 334-380 cherry, 431-477 crimson, 480-526 warped, 545-591
    // bamboo), the STONE row the `iconst_1,1,0` + MOBS pair the arrows'
    // refusal the same row banks (pc 178-223) — the POLISHED_BLACKSTONE
    // row pc 226-271 the SAME `iconst_1,1,0` + MOBS + the
    // STONE_PRESSURE_PLATE_CLICK_ON/OFF pair the stone-family law keeps
    // (the polished rung the stone set's own voices + refusal the
    // separate POLISHED_BLACKSTONE set type answers — the plate's own
    // set the STONE row's fields byte-equal serves). The Properties
    // rows the twelve windows the SAME walk serves (mapColor planks,
    // forceSolidOn, instrument BASS, noCollision, strength(0.5f),
    // ignitedByLava, pushReaction DESTROY — the spruce/birch/jungle/
    // cherry/pale_oak windows pc-identical speak). The blockstates the
    // ONE {wood}_pressure_plate.json BOTH states the same variants map
    // serves (the strip-suffix walk the _down rows the own file never
    // wears), the ONE #texture ref block/{wood}_planks / block/stone /
    // block/polished_blackstone the row's tile banks (the palette the
    // tiles already bank — the planks masters the button ladder's own
    // walk gains, the polished_blackstone master the lever row's tail
    // seat wears, no new tile the regen keeps, the pre-ladder rows
    // byte-identical).
    "spruce_pressure_plate", "spruce_pressure_plate_down",
    "birch_pressure_plate", "birch_pressure_plate_down",
    "jungle_pressure_plate", "jungle_pressure_plate_down",
    "acacia_pressure_plate", "acacia_pressure_plate_down",
    "dark_oak_pressure_plate", "dark_oak_pressure_plate_down",
    "mangrove_pressure_plate", "mangrove_pressure_plate_down",
    "cherry_pressure_plate", "cherry_pressure_plate_down",
    "pale_oak_pressure_plate", "pale_oak_pressure_plate_down",
    "bamboo_pressure_plate", "bamboo_pressure_plate_down",
    "crimson_pressure_plate", "crimson_pressure_plate_down",
    "warped_pressure_plate", "warped_pressure_plate_down",
    "polished_blackstone_pressure_plate", "polished_blackstone_pressure_plate_down",
    // THE WEIGHTED PLATES' OWN FIRST LANE (the dev-49 round): the gold
    // and iron rungs the sensor family's OWN subclass serves — javap
    // the 26.2 jar (pinned sha1 2dc72797) `WeightedPressurePlateBlock`
    // the probe scripts/jar_work/weighted_plate_49.py banks: the ctor
    // `(int maxWeight, BlockSetType, Properties)` walk pc 0-38 the
    // maxWeight field banks, the Blocks clinit the two factory rows
    // speak — LIGHT pc 15701-15738 bsm[349] → lambda$static$236 the
    // `bipush 15` + `BlockSetType.GOLD` pair banks (MapColor.GOLD,
    // forceSolidOn + noCollision + strength(0.5f) + pushReaction
    // DESTROY the same window the plate ladder's rows wear), HEAVY
    // pc 15741-15778 bsm[350] → lambda$static$237 the `sipush 150` +
    // `BlockSetType.IRON` pair banks (MapColor.METAL). The
    // getSignalStrength walk pc 0-52 the COUNT law serves — the
    // getEntityCount(level, TOUCH_AABB.move(pos), Entity.class) the
    // EVERY entity counts (the items/arrows/mobs/player the weighted
    // walk's own class arg arms — the MOBS row the stone set wears
    // the weighted rows never speak), the `Math.min(count, maxWeight)`
    // pc 13-20 fold + the `Mth.ceil(min/maxWeight * 15.0f)` pc 25-50
    // the graded signal answers (light: 15 entities full — count per
    // entity; heavy: 150 entities full — ten per rung), the `getPress
    // edTime` the `bipush 10` pc 0-2 the HALF-window banks (the 10-tick
    // release the weighted rows wear — the 20-tick BOTH-sets law the
    // PressurePlateBlock rows keep). The POWER property the integer
    // 0..15 the blockstates speak — power=0 the up model, power=1..15
    // the down rows the SAME _down model the whole range answers (the
    // binary pair the OFF/ON kinds the ±1 mate wears — the power>0
    // gate the ON bit the checkPressed walk's own signal>0 law keeps).
    // The blockstates the `power={0..15}` rows the weighted files wear
    // (the powered={p} rows the plate ladder's own files never share),
    // the models the ONE #texture ref block/gold_block / block/iron_
    // block the row's tile banks (the jar's PNGs the first-encounter
    // walk gains — the two metal masters the tile table's tail seats
    // bank), the parents pressure_plate_up/down the ladder's own
    // elements wear. The sounds.json the metal rows carry (block.metal
    // _pressure_plate.click_on pitch 0.75 / click_off 0.65, volume 0.3
    // — the random/click event BOTH rungs share — the GOLD/IRON set
    // types' pressurePlateClickOn/Off fields the record rows bank).
    "light_weighted_pressure_plate", "light_weighted_pressure_plate_down",
    "heavy_weighted_pressure_plate", "heavy_weighted_pressure_plate_down",
    // THE REDSTONE DUST'S OWN PAIR (the autopush-665 round): the wire's
    // binary OFF/ON kinds the sensor family's carrier lane wears —
    // javap `net.minecraft.world.level.block.RedStoneWireBlock` the 26.2
    // jar (pinned sha1 2dc72797, the python walk scripts/jar_work/
    // redstone_wire_665*.py): the clinit's COLORS table the
    // `bipush 16; newarray; invokedynamic bsm[2]` lambda$static$0 fill
    // banks (the getColorForPower walk pc 0-6 the `COLORS[power]` row
    // speaks), the lambda's own math the pool floats answer —
    // `f = power / 15.0f` (pc 8-14 the i2f/ldc_w 15.0f/fdiv fold),
    // `red = f * 0.6f + (f > 0.0f ? 0.4f : 0.3f)` (pc 15-36 the
    // 0.6/0.4/0.3 pool rows the fcmpl/ifle branch arms), `green =
    // Mth.clamp(f * f * 0.7f - 0.5f, 0, 1)` (pc 37-53), `blue =
    // Mth.clamp(f * f * 0.6f - 0.7f, 0, 1)` (pc 54-71), the
    // `ARGB.colorFromFloat(1.0f, r, g, b)` pc 72-84 wrap — the OFF row
    // the power-0 answer (0.3, 0, 0) the (77, 0, 0) RGB triple wears
    // (the colorFromFloat byte fold `(int)(f * 255.0 + 0.5)`), the ON
    // row the power-15 answer (1.0, 0.2, 0.0) the (255, 51, 0) triple
    // banks. THE BINARY PAIR the graded ramp the power kinds a future
    // lane wears (the OFF row the power-0 color, the ON row the
    // power-15 color — the whole 1..15 range the ON kind the binary
    // client's own vocabulary answers). The canSurvive walk pc 0-26
    // the `canSurviveOn(level, pos.below(), state)` delegate speaks —
    // the below cell's `isFaceSturdy(UP)` read the straight-DOWN
    // support the plate family's own gate wears (the hopper row the
    // vocabulary never banks). The getSignal walk pc 0-81 the
    // shouldSignal gate + the DOWN-dir 0 + the UP-dir the raw POWER
    // + the horizontal dirs the opposite-side connection verdict
    // arms (the ON kind's 15 the nonzero verdict the door's
    // hasNeighborSignal read wears), the isSignalSource walk pc 0-5
    // the shouldSignal field the connection walks' TRUE keeps. The
    // blockstates/redstone_wire.json the MULTIPART 9 parts wear —
    // the dot center part[0] the all-none OR the perpendicular-pair
    // OR arms serve, the side arms parts[1..4] the
    // side0/side_alt0/side_alt1/side1 models name (the #line sprites
    // redstone_dust_line0/line1 the N-S/E-W masters bank — the
    // 16x16 colortype-4 grayscale PNGs the pixel probes speak, the
    // line the 1-px v-axis strip the dot the 29-px center blob
    // wears), the up parts[5..8] the climbing wall arms the binary
    // lane never banks (the same-level run the first lane walks —
    // the up-connection the graded lane's own walk serves). The y=0.25
    // plane the elements speak (from [0,0.25,0] to [16,0.25,16] the
    // zero-thickness up+down pair both faces wear — the FLAT_Y the
    // leaf_litter's own 0.25 the same quarter-pixel height keeps).
    // The `redstone_wire_on` kind's row the strip law the lever pair's
    // _on row keeps (the registry_names walk the _on strip the ONE
    // "redstone_wire" registry entry both states banks — the dig row
    // (0.0, false) the instabreak clinit speaks, the item id the
    // pick lane grants BOTH kinds speak, the drops table the
    // DROPS_REDSTONE_WIRE row the redstone item answers). The
    // Properties row the clinit's own walk banks (pc 6865-6891):
    // `Properties.of().noCollision().instabreak().pushReaction(
    // DESTROY)` — NO sound() call the default STONE SoundType the
    // block_sounds row keeps, the pass-through + the instant dig the
    // collision table's Empty row wears. The Items clinit row
    // (pc 7237-7262) the BlockItem pairing speaks —
    // `Items.REDSTONE = registerItem(BlockItemIds.REDSTONE_DUST,
    // createBlockItemWithCustomItemName(Blocks.REDSTONE_WIRE),
    // new Item.Properties().trimMaterial(...))` — the "redstone" item
    // the placement lane's own BlockItem walk serves.
    "redstone_wire", "redstone_wire_on",
    // THE REDSTONE TORCH'S OWN PAIR (the autopush-666 round — javap
    // RedstoneTorchBlock the 26.2 jar the python walk scripts/
    // jar_work/redstone_torch_666.py banks): the NOT-gate's own source
    // the lit/off pair the palette's tail banks — the ctor pc 0-32 the
    // `registerDefaultState(any().setValue(LIT, true))` walk the LIT
    // default speaks (the plain-name kind the lit row the blockstate's
    // own mapping keeps: lit=true the block/redstone_torch model
    // answers, lit=false the block/redstone_torch_off row the
    // template_torch_unlit parent the stick box alone keeps — the lit
    // parent template_redstone_torch the stick + the SIX glow planes
    // the 7-element chain banks, the full-cube finder the FIRST
    // element's six-face box the tile walk answers), the getSignal pc
    // 0-19 the `direction == UP ? 0 : ownSignal` arms the UP exclusion
    // banks, the ownSignal pc 0-22 the `LIT ? 15 : 0` fold the lit
    // kind's own 15 banks, the blocks clinit row pc 10070-10107 the
    // `noCollision().instabreak()` + `lightLevel(litBlockEmission(7))`
    // + `sound(WOOD)` + `pushReaction(DESTROY)` chain speaks, the
    // Items clinit row pc 7265-7276 the StandingAndWallBlockItem
    // pairing the standing torch the item's own primary block keeps
    // (the wall torch the separate block the follow-up round wears).
    // The `_off` row the jar's OWN model suffix wears — the
    // RedstoneTorchBlock's ONE "redstone_torch" registry entry both
    // states the same name banks (the registry_names walk the _off
    // strip the plate's _down law keeps).
    "redstone_torch", "redstone_torch_off",
    // THE REDSTONE WALL TORCH'S OWN PAIR (the autopush-667 round —
    // javap RedstoneWallTorchBlock the 26.2 jar the python walk
    // scripts/jar_work/redstone_wall_torch_667.py banks): the
    // wall-mounted NOT-gate the lit/off pair the palette's tail banks
    // — the ctor pc 16-41 the `registerDefaultState(any().setValue
    // (FACING, NORTH).setValue(LIT, true))` walk the NORTH-facing LIT
    // default speaks, the blockstates' redstone_wall_torch.json the
    // 8-variant facing x lit map the y-table banks (facing=east the
    // base model the NO-y row answers, north 270 / south 90 / west 180
    // the rotated rows speak), the getSignal pc 0-23 the `direction ==
    // FACING ? 0 : ownSignal` arms the FACING exclusion banks (the
    // standing pair's UP arm the wall's own forward arm keeps), the
    // hasNeighborSignal pc 0-27 the `pos.relative(FACING.getOpposite())`
    // read the attached cell's own power verdict serves, the canSurvive
    // pc 0-15 the WallTorchBlock delegate the attached block's
    // isFaceSturdy read speaks, the blocks clinit row pc 10113-10154
    // the `wallVariant(REDSTONE_TORCH, true)` + `noCollision()
    // .instabreak()` + `lightLevel(litBlockEmission(7))` + `sound(WOOD)`
    // + `pushReaction(DESTROY)` chain banks, the Items clinit row pc
    // 7265-7276 the StandingAndWallBlockItem pairing the wall torch the
    // item's own secondary block keeps (the attachmentDirection DOWN
    // the ctor's own arg banks — the wall state the placement's own
    // scan arms). The `_off` row the jar's OWN model suffix wears — the
    // RedstoneWallTorchBlock's ONE "redstone_wall_torch" registry entry
    // both states the same name banks (the registry_names walk the _off
    // strip the standing pair's law keeps).
    "redstone_wall_torch", "redstone_wall_torch_off",
    // THE OAK TRAPDOOR'S OWN PAIR (the autopush-668 round — scripts/
    // jar_work/trapdoor_668.txt banks): the thin-panel door family's
    // sibling the interactive lane finally wears — the OPEN bit the
    // ±1 kind pair the lever/button/plate/wire/torch ladders' own law
    // carries, the FACING + HALF bits the variant byte's spare seats
    // bank (the facing byte the wall torch's own vocabulary keeps:
    // north 1, east 2, south 3, west 4 — the 0 byte the no-opinion
    // seat the ctor's NORTH+BOTTOM default answers; the +8 bit the
    // HALF=top word the placement's own click-height law stamps).
    // javap TrapDoorBlock the 26.2 jar: the ctor pc 17-95 the
    // `registerDefaultState(any().setValue(FACING, NORTH).setValue
    // (OPEN, false).setValue(HALF, BOTTOM).setValue(POWERED, false)
    // .setValue(WATERLOGGED, false))` walk the NORTH-facing CLOSED
    // BOTTOM default speaks, the useWithoutItem pc 0-26 the
    // `canOpenByHand` gate the toggle walk arms (the WOODEN set type
    // the OAK row the PASS never keeps), the toggle pc 0-75 the
    // `state.cycle(OPEN)` the setBlock flag 2 the playSound
    // trapdoorOpen/trapdoorClose events the BLOCK_OPEN/BLOCK_CLOSE
    // gameEvents the hand lane wears, the getStateForPlacement
    // pc 0-204 the side-hit FACING=clickedFace + HALF=clickLocation.y
    // - pos.y > 0.5 walk the top/bottom-hit FACING=horizontalDirection
    // .getOpposite() + HALF=UP?BOTTOM:TOP arms the hasNeighborSignal
    // preset the OPEN+POWERED placement lands, the getShape pc 0-59
    // the SHAPES map the OPEN ? FACING : (HALF==TOP ? DOWN : UP) key
    // folds (the Shapes.rotateAll six-key map the base NORTH panel
    // [0,0,13,16,16,16] the X_90/X_270 rotations the UP/DOWN slabs
    // bank — the closed bottom [0,0,0,16,3,16], the closed top
    // [0,13,0,16,16,16], the open panel the FACING-opposite edge the
    // 3/16 thickness every rotation keeps), the neighborChanged
    // pc 0-129 the `signal != POWERED` gate the OPEN=signal sync +
    // the playSound arms (the POWERED tracking bit the wire consult's
    // own verdict serves), the blocks clinit row pc 11324-11370 the
    // `mapColor(WOOD).instrument(BASS).strength(3.0f).noOcclusion()
    // .isValidSpawn(...).ignitedByLava()` chain banks (the WOOD
    // SoundType the ctor's own `sound(type.soundType())` walk pc 1-6
    // speaks — the BlockSetType.OAK soundType the WOOD row keeps), the
    // Items clinit row pc 8193-8202 the `registerBlock
    // (BlockItemIds.OAK_TRAPDOOR, Blocks.OAK_TRAPDOOR)` the BlockItem
    // pairing speaks. The blockstates' oak_trapdoor.json the 16-row
    // facing x half x open map banks (facing=north the base model the
    // NO-y row answers, east 90 / south 180 / west 270 the rotated
    // rows speak — the open rows the y table the closed rows never
    // carry: the slab the facing the symmetric geometry never turns),
    // the template_trapdoor_bottom/top/open parents the ONE-element
    // chains bank (the closed bottom [0,0,0]->[16,3,16], the closed
    // top [0,13,0]->[16,16,16], the open [0,0,13]->[16,16,16] the
    // north strip the SIX faces the #texture ref every element wears),
    // the oak_trapdoor.png master the 16x16 colortype-3 tile banks.
    // The `_open` row the jar's registry never speaks — the
    // TrapDoorBlock's ONE "oak_trapdoor" registry entry both states
    // the same name banks (the registry_names walk the _open strip
    // the door family's own law keeps).
    "oak_trapdoor", "oak_trapdoor_open",
    // THE TRAPDOOR LADDER'S OWN WOOD RUNGS (the autopush-669 round —
    // scripts/jar_work/trapdoor_woods_669.txt banks): the eleven
    // remaining wooden trapdoors the oak lane's own walk generalizes
    // (the door family's wood order the rungs wear — the dev-43
    // round's spruce/birch/jungle/acacia/dark_oak/mangrove/cherry/
    // pale_oak/bamboo/crimson/warped sequence the contiguity law
    // keeps). Every wood the SAME two-kind pair the OPEN bit the ±1
    // mate carries, the FACING + HALF bits the variant byte's own
    // seats bank (the oak round's own vocabulary every rung shares).
    // The blockstates' {wood}_trapdoor.json the 16-row maps the per-
    // row arms read (the closed facing=east,half=bottom,open=false
    // rows the block/{wood}_trapdoor_bottom models answer, the open
    // facing=north,half=bottom,open=true rows the block/{wood}_
    // trapdoor_open models keep — the probe's 16-row census the
    // eleven woods the SAME key shape speaks), the template parents
    // the ONE-element chains bank (the ten orientable woods the
    // template_orientable_trapdoor_* parents wear, the dark_oak the
    // oak's own template_trapdoor_* chain keeps — the same geometry
    // both template families bank), the {wood}_trapdoor.png masters
    // the 16x16 tiles the bank gains (the acacia's colortype-6 RGBA
    // row the tile walk handles), the {wood} clinit rows the
    // Properties chains the oak's own row speaks (the mapColor/instrument/
    // strength(3.0f)/noOcclusion chains the per-wood map colors wear),
    // the Items clinit rows the registerBlock pairings the placement
    // lane grants. The set types the voice stems split (the official
    // sounds.json the four pairs banks): the plain woods the WOODEN
    // set type's block.wooden_trapdoor pair speaks, the cherry the
    // CHERRY_WOOD set type's block.cherry_wood_trapdoor pair wears,
    // the bamboo the BAMBOO_WOOD pair keeps, the crimson/warped the
    // NETHER_WOOD pair answers (the door family's own stem law the
    // trapdoors share).
    "spruce_trapdoor", "spruce_trapdoor_open",
    "birch_trapdoor", "birch_trapdoor_open",
    "jungle_trapdoor", "jungle_trapdoor_open",
    "acacia_trapdoor", "acacia_trapdoor_open",
    "dark_oak_trapdoor", "dark_oak_trapdoor_open",
    "mangrove_trapdoor", "mangrove_trapdoor_open",
    "cherry_trapdoor", "cherry_trapdoor_open",
    "pale_oak_trapdoor", "pale_oak_trapdoor_open",
    "bamboo_trapdoor", "bamboo_trapdoor_open",
    "crimson_trapdoor", "crimson_trapdoor_open",
    "warped_trapdoor", "warped_trapdoor_open",
    // THE METAL RUNGS' OWN ROWS (the autopush-670 round — the copper
    // ladder + the iron pair the door family's copper order the tail
    // wears, scripts/jar_work/iron_copper_trapdoors_670.txt the
    // 16-row maps bank): the blockstates' {stem}_trapdoor.json the
    // per-row arms read (the closed facing=east,half=bottom,open=false
    // rows the block/{stem}_trapdoor_bottom models answer, the open
    // facing=north,half=bottom,open=true rows the block/{stem}_
    // trapdoor_open models keep — the waxed rows the UNWAXED models
    // the same-model law banks, the waxed_copper_trapdoor.json the
    // block/copper_trapdoor_bottom rows answer, the SAME sprite the
    // unwaxed tiles wear), the {stem}_trapdoor.png masters the five
    // metal tiles the bank gains, the COPPER set type's
    // block.copper_trapdoor pair the ladder voices, the IRON pair the
    // block.iron_trapdoor events wear, the ONE "{stem}_trapdoor"
    // registry name both kinds each pair speaks.
    "copper_trapdoor", "copper_trapdoor_open",
    "exposed_copper_trapdoor", "exposed_copper_trapdoor_open",
    "weathered_copper_trapdoor", "weathered_copper_trapdoor_open",
    "oxidized_copper_trapdoor", "oxidized_copper_trapdoor_open",
    "waxed_copper_trapdoor", "waxed_copper_trapdoor_open",
    "waxed_exposed_copper_trapdoor", "waxed_exposed_copper_trapdoor_open",
    "waxed_weathered_copper_trapdoor", "waxed_weathered_copper_trapdoor_open",
    "waxed_oxidized_copper_trapdoor", "waxed_oxidized_copper_trapdoor_open",
    "iron_trapdoor", "iron_trapdoor_open",
    // THE LAMP'S OWN PAIR (the autopush-672 round): redstone_lamp the
    // consumer family's own block the wire's verdict finally answers
    // — the blockstates/redstone_lamp.json the lit variant's own pair
    // the SAME DIRECT authoring the torch rows wear (lit=false the
    // block/redstone_lamp cube_all model, lit=true the
    // block/redstone_lamp_on arm — the ONE #texture ref each the
    // row's tile banks, the lit row's own amber tile the regen
    // banks), the ONE "redstone_lamp" registry name both kinds speak
    // (the _on strip the generated rows' law keeps), the Items
    // clinit's redstone_lamp registration the placement lane grants
    // (the item row the generated table already banks), the
    // lightLevel 15 the lit registration the emission's own row
    // keeps.
    "redstone_lamp", "redstone_lamp_on",
    // THE WIRE'S OWN GRADED LADDER (the autopush-673 round): the 14
    // middle rungs the tail rows wear — the POWER 1..=14 states the
    // decay's own walk banks, the ONE "redstone_wire" registry id ALL
    // SIXTEEN rows speak (the _pNN strip the generated rows' law
    // keeps), the NO jar blockstate the rung rows carry — the wire
    // special-case arm the same multipart chain the OFF/ON rows read
    // serves, the per-power tiles the append pass's own bank colors.
    "redstone_wire_p01", "redstone_wire_p02", "redstone_wire_p03",
    "redstone_wire_p04", "redstone_wire_p05", "redstone_wire_p06",
    "redstone_wire_p07", "redstone_wire_p08", "redstone_wire_p09",
    "redstone_wire_p10", "redstone_wire_p11", "redstone_wire_p12",
    "redstone_wire_p13", "redstone_wire_p14",
    // THE PLANKS' OWN TWELVE SEATS (the autopush-674 round): the twelve
    // wood families' build material the world never carried — the
    // simplest full-cube family the palette's append-only law banks.
    // javap Blocks clinit the 26.2 jar (pinned sha1 2dc72797): each
    // plank the BlockBehaviour.Properties.of().mapColor(MaterialColor.
    // WOOD..).strength(2.0f, 3.0f).sound(SoundType.WOOD).ignitedByLava()
    // row the lambda$static$NNN factory answers (oak_planks the
    // mapColor planks row 148 the comment law wears — the per-wood
    // mapColor the SAME MaterialColor family the planks' own hue
    // keeps), the ONE blockstate {stem}_planks.json the generic
    // single-variant map carries ("" -> block/{stem}_planks the
    // cube_all parent the ONE #all texture ref speaks), the
    // {stem}_planks.png master the jar's own 16x16 colortype-3 tile
    // banks, the ONE "{stem}_planks" registry name its kind alone
    // speaks (no strip law — each plank its own Block + BlockItem
    // pairing the Items clinit's registerBlock rows answer), the
    // mineable/axe tag + the block_dig (2.0, false) rows + the
    // block_sounds wood family + the block_drops self-item pool the
    // generated tables already bank — the NAME_OF row the lanes wire.
    // The seats the CONTIGUOUS tail the kind ids wear (385..396 — the
    // door/button family order the same twelve woods keeps), the
    // END_PORTAL kind the beyond-table seat the family's growth
    // displaces (384 -> 396 the 658/670/672/673 precedent wears).
    "oak_planks", "spruce_planks", "birch_planks", "jungle_planks",
    "acacia_planks", "dark_oak_planks", "mangrove_planks", "cherry_planks",
    "pale_oak_planks", "bamboo_planks", "crimson_planks", "warped_planks",
    // THE STONE MASON'S OWN KIT (the autopush-675 round): the cobble
    // + the smooth slab-block + the stone bricks quartet the builder's
    // own stone shelf banks — javap Blocks clinit the 26.2 jar (pinned
    // sha1 2dc72797): the cobblestone the lambda$static$NNN factory's
    // strength(2.0f, 6.0f) + requiresCorrectToolForDrops row the
    // mineable/pickaxe tag answers, the smooth_stone the
    // strength(2.0f, 6.0f) + sound(SoundType.STONE) row keeps, the
    // bricks quartet the strength(1.5f, 6.0f) rows the SAME pickaxe
    // demand wears (the 1.5 hardness the pick's own speed the dig
    // walk answers — the bare hand the stone family's drop refuses).
    // The ONE blockstate {name}.json the generic single-variant map
    // every row carries ("" -> block/{name} the cube_all parent the
    // ONE #all texture ref speaks — the smooth_stone the jar's own
    // cube_all row banks, the side-top split the slab models alone
    // wear), the {name}.png master the jar's own 16x16 colortype-3
    // tile banks (the tile the furnace/jukebox/monster-spawner
    // families' own models already banked — the REUSED index the
    // append-only law the equivalence proof keeps). The ONE "{name}"
    // registry name its kind ALONE speaks (no strip law — each stone
    // its own Block + BlockItem pairing the Items clinit's
    // registerBlock rows answer), the generated dig/drops/sounds/
    // items tables the rows already bank — the NAME_OF row the lanes
    // wire. The seats the CONTIGUOUS tail the kind ids wear
    // (396..401 — the vanilla stone shelf's own order keeps), the
    // END_PORTAL kind the beyond-table seat the kit's growth
    // displaces (396 -> 402 the 658/670/672/673/674 precedent wears).
    "cobblestone", "smooth_stone", "stone_bricks", "mossy_stone_bricks",
    "cracked_stone_bricks", "chiseled_stone_bricks",
    // THE DESERT'S OWN SHELF (the autopush-676 round): the sandstone
    // family's eight seats the builder's desert banks — the regular
    // quartet the red quartet the door family's own order the tail
    // wears. javap Blocks clinit the 26.2 jar (pinned sha1 2dc72797):
    // the sandstone the lambda$static$NNN factory's Properties.of().
    // mapColor(MaterialColor.SAND).instrument(NoteBlockInstrument.
    // SNARE).requiresCorrectToolForDrops().strength(0.8f, 0.8f).sound(
    // SoundType.STONE) row the mineable/pickaxe tag answers (the
    // generated block_dig table the 0.8 + requires-tool rows the
    // jar's REAL bootstrapped registry banks — the chiseled/cut trio
    // the SAME 0.8 row wears, the smooth pair the 2.0 row keeps, the
    // bare hand the whole family's drop refuses), the block.stone.
    // break voice the generated block_sounds table every row banks,
    // the self-entry drop pool the block_drops table speaks. The
    // blockstates the jar's own three shapes carry — the sandstone/
    // red_sandstone.json the cube_bottom_top parent the THREE refs
    // speak (top={name}_top, bottom={name}_bottom, side={name} —
    // the TNT precedent the per-face lane wears), the cut/chiseled
    // pair the cube_column parent the end/side refs keep (end={name}_
    // top the chiseled pair's own cut rows answer), the smooth pair
    // the cube_all parent the ONE #all ref wears (all={name}_top —
    // the smooth master the jar's own top tile re-wears). The ten
    // {name}.png masters the jar's own 16x16 colortype-3 tiles bank
    // (sandstone/_top/_bottom + red_sandstone/_top/_bottom + the
    // cut/chiseled pair each — NEW tiles the append-only tail seats,
    // every pre-existing row the same pixels + tints + names keeps).
    // The ONE "{name}" registry name its kind ALONE speaks (no strip
    // law — each sandstone its own Block + BlockItem pairing the
    // Items clinit's registerBlock rows answer), the generated dig/
    // drops/sounds/items tables the rows already bank — the NAME_OF
    // row the lanes wire. The seats the CONTIGUOUS tail the kind ids
    // wear (402..409 — the family order the quartets keep), the
    // END_PORTAL kind the beyond-table seat the family's growth
    // displaces (402 -> 410 the 658/670/672/673/674/675 precedent
    // wears).
    "sandstone", "chiseled_sandstone", "cut_sandstone", "smooth_sandstone",
    "red_sandstone", "chiseled_red_sandstone", "cut_red_sandstone",
    "smooth_red_sandstone",
    // THE WOOL'S OWN SIXTEEN CLOTHS (the autopush-677 round): the wool
    // family's fifteen seats the builder's dyed banks — the orange..
    // black rungs the tail wears, the WHITE kind its clinit position
    // the 76 seat the cobweb's own neighbor keeps (the dev-22 round's
    // row the tail never moves — the append-only law the seat's own
    // history banks). javap Blocks clinit the 26.2 jar (pinned sha1
    // 2dc72797): the sixteen colors ONE helper walk banks — pc 4743
    // the `ColorCollection.registerBlocks(..., BlockItemIds.WOOL)`
    // row the piston_head register's own follower speaks (the 16
    // colors one lambda family the white entry's doc already banks),
    // the per-color rows the generated block_dig table the jar's
    // REAL bootstrapped registry answers (the 0.8 + no-tool rows
    // every color wears — the bare hand the whole family's drop
    // keeps, the vanilla hand-harvest law the sheep's own lane
    // speaks), the block.wool.break voice the generated block_sounds
    // table every seat banks, the self-entry drop pool the
    // block_drops table speaks. The blockstates the jar's own ONE
    // shape carry — the {color}_wool.json the cube_all parent the
    // ONE #all ref wears (all={name} — the white master the jar's
    // own tile re-wears), the FACE_TILES rows the one-tile lane
    // banks. The FIFTEEN {name}.png masters the jar's own 16x16
    // colortype-3 tiles bank — NEW tiles the append-only tail seats
    // (685 -> 700: orange, magenta, light_blue, yellow, lime, pink,
    // gray, light_gray, cyan, purple, blue, brown, green, red, black
    // — the clinit ladder's own order the tail wears), every
    // pre-existing row the same pixels + tints + names keeps, the
    // white tile the bank's own index re-wears. The ONE "{name}"
    // registry name its kind ALONE speaks (no strip law — each
    // color its own Block + BlockItem pairing the Items clinit's
    // registerBlock rows answer), the generated dig/drops/sounds/
    // items tables the rows already bank — the NAME_OF row the
    // lanes wire. The seats the CONTIGUOUS tail the kind ids wear
    // (410..424), the END_PORTAL kind the beyond-table seat the
    // family's growth displaces (410 -> 425 the
    // 658/670/672/673/674/675/676 precedent wears).
    "orange_wool", "magenta_wool", "light_blue_wool", "yellow_wool",
    "lime_wool", "pink_wool", "gray_wool", "light_gray_wool",
    "cyan_wool", "purple_wool", "blue_wool", "brown_wool",
    "green_wool", "red_wool", "black_wool",
    // THE POTTER'S OWN SEVENTEEN KILNS (the autopush-678 round): the
    // terracotta family's seventeen seats the builder's kiln banks —
    // the sixteen dyed rungs the clinit ladder's own order the tail
    // wears, the PLAIN kind its own register the dyed walk's follower
    // banks (javap Blocks clinit the 26.2 jar, pinned sha1 2dc72797:
    // pc 16200 the `ColorCollection.registerBlocks(...,
    // BlockItemIds.DYED_TERRACOTTA)` row the sixteen colors one
    // lambda family speaks, pc 16914 the plain
    // `register(BlockItemIds.TERRACOTTA, ...mapColor(COLOR_ORANGE).
    // instrument(BASEDRUM).requiresCorrectToolForDrops().strength(
    // 1.25, 4.2))` row the dyed walk's own follower wears — the
    // clinit order the dyed rungs FIRST the plain seat the tail
    // keeps, the DIG table's own ladder answers: the colored
    // 484..499 rows the plain 554 row the same 1.25 + requires-tool
    // law every seat banks — the pickaxe the whole family's drop
    // demands, the bare hand the drop refuses), the block.stone.
    // break voice the generated block_sounds table every seat banks
    // (the STONE SoundType the SoundType.TERRACOTTA alias wears —
    // the ONE block.stone row the plain + dyed seats share), the
    // self-entry drop pool the block_drops table speaks. The
    // blockstates the jar's own ONE shape carry — the {name}.json
    // the cube_all parent the ONE #all ref wears (all={name}), the
    // FACE_TILES rows the one-tile lane banks. The SEVENTEEN {name}.
    // png masters the jar's own 16x16 colortype-3 tiles bank — NEW
    // tiles the append-only tail seats (700 -> 717: the sixteen
    // {color}_terracotta rungs + the plain terracotta master — the
    // clinit ladder's own order the tail wears), every pre-existing
    // row the same pixels + tints + names keeps. The ONE "{name}"
    // registry name its kind ALONE speaks (no strip law — each
    // seat its own Block + BlockItem pairing the Items clinit's
    // registerBlock rows answer), the generated dig/drops/sounds/
    // items tables the rows already bank — the NAME_OF row the
    // lanes wire. The seats the CONTIGUOUS tail the kind ids wear
    // (425..441 — the dyed rungs 425..440 the plain kiln the 441
    // seat keeps), the END_PORTAL kind the beyond-table seat the
    // family's growth displaces (425 -> 442 the
    // 658/670/672/673/674/675/676/677 precedent wears).
    "white_terracotta", "orange_terracotta", "magenta_terracotta",
    "light_blue_terracotta", "yellow_terracotta", "lime_terracotta",
    "pink_terracotta", "gray_terracotta", "light_gray_terracotta",
    "cyan_terracotta", "purple_terracotta", "blue_terracotta",
    "brown_terracotta", "green_terracotta", "red_terracotta",
    "black_terracotta", "terracotta",
    // THE FENCE'S OWN GATE (the dev-53 round): the interactive
    // vocabulary's next member the oak rung opens — javap
    // FenceGateBlock the 26.2 jar (pinned sha1 2dc72797, the probe
    // scripts/jar_work/fence_gate_53.py banks): the ctor's
    // registerDefaultState walk pc 17-67 pins OPEN=false,
    // POWERED=false, IN_WALL=false (the FACING the
    // HorizontalDirectionalBlock default's NORTH pin wears), the
    // Properties chain the Blocks clinit row pc 12870-12912 banks
    // (mapColor(OAK_PLANKS.defaultMapColor()) + forceSolidOn() +
    // instrument(BASS) + strength(2.0, ...) + ignitedByLava() — NO
    // sound() row the Properties chain carries, the ctor's
    // `Properties.sound(type.soundType())` pc 2-6 walk the WOOD
    // SoundType the dig/place rows bank), the useWithoutItem walk
    // pc 0-189 the RMB toggle serves (the open bit the ±1 seat the
    // kind pair banks, the closed branch's re-face law pc 43-77
    // arms — the `FACING == dir.getOpposite()` verdict the
    // player's own look the FACING byte wears, the setBlock flag
    // 10 pc 34-98, the playSound pc 117-162 the fenceGateOpen/
    // fenceGateClose events the `nextFloat() * 0.1F + 0.9F` pitch
    // walk speaks [cp@244 = 0.1f, cp@245 = 0.9f], the gameEvent
    // pc 165-183 the BLOCK_OPEN/BLOCK_CLOSE posts), the
    // getStateForPlacement walk pc 0-170 the FACING=
    // getHorizontalDirection() + the OPEN/POWERED hasNeighborSignal
    // preset arms (the IN_WALL the isWall tag verdict the walls the
    // palette never banks — the pin the false seat keeps), the
    // getCollisionShape walk pc 0-50 the OPEN state's Shapes.empty()
    // the pass-through verdict speaks (the OPEN gate even the posts
    // the collision never banks) and the closed SHAPE_COLLISION
    // column(16.0, 4.0, 0.0, 24.0) the centered 4/16 strip banks
    // [the 0..24 y-band the anti-jump the fence family's 1.5-tall
    // law speaks — the per-cell sweep's vocabulary the over-tall
    // extension a later round serves]), the SHAPES map
    // rotateHorizontalAxis(Block.cube(16.0, 16.0, 4.0)) the outline
    // walk banks. The ONE "oak_fence_gate" registry name its kinds
    // BOTH speak (the BlockItemIds.OAK_FENCE_GATE the Items clinit
    // row pc 8354-8363 the registerBlock pairing answers), the
    // generated dig/drops/sounds/items tables the rows already
    // bank. The blockstates oak_fence_gate.json the 16-variant map
    // carries — the facing=south,in_wall=false,open={p} rows the
    // two kinds resolve (the y table north 180 / east 270 / south
    // 0 / west 90 the mesher's theta tables fold), the models the
    // template chain's EIGHT elements speak (the two posts + the
    // two inner verticals + the four bars — the derive walk the
    // real geometry resolves), the #texture ref block/oak_planks
    // the ONE master every face wears (NO new tile — the planks
    // master the palette already banks, the append-only law the
    // equivalence proof scripts/verify_atlas_full.py keeps). The
    // seats the CONTIGUOUS tail the kind ids wear (442/443), the
    // END_PORTAL kind the beyond-table seat the family's growth
    // displaces (442 -> 444 the
    // 658/670/672/673/674/675/676/677/678 precedent wears).
    "oak_fence_gate", "oak_fence_gate_open",
    // THE BUILDER'S OWN FIFTEEN CONCRETE RUNGS (the autopush-679
    // round): the dyed concrete family's fifteen seats the builder's
    // color banks — the orange..black rungs the clinit ladder's own
    // order the tail wears, the WHITE kind its early seat the
    // fifty-seventh entry keeps, the END_PORTAL the beyond-table 460
    // the every-family displacement law keeps. javap Blocks clinit
    // the 26.2 jar (pinned sha1 2dc72797, the probe scripts/
    // jar_work/concrete_679.py + .out banks): pc 20979-20987 the
    // ColorCollection.registerBlocks(..., TriFunction, BiFunction,
    // Function) walk the putstatic Blocks.CONCRETE row pc 20987 the
    // sixteen-color collection ONE lambda family speaks (the
    // CONCRETE_POWDER register pc 20990-21011 the white pair's own
    // walk the 55/56 seats bank), the Items clinit row pc 6275-6292
    // the registerBlockItems(BlockItemIds.CONCRETE, Blocks.CONCRETE,
    // ...) pairing speaks (the putstatic Items.CONCRETE pc 6292),
    // the dig rows the generated block_dig table the jar's REAL
    // bootstrapped registry answers (the 1.8 + requires-tool rows
    // every color wears — the pickaxe the whole family's drop
    // demands, the bare hand the drop refuses), the block.stone.
    // break voice the generated block_sounds table every seat banks,
    // the self-entry drop pool the block_drops table speaks. The
    // blockstates the jar's own ONE shape carry — the {name}.json
    // the single "" variant the cube_all model wears, the models the
    // ONE #all ref (all={name}), the FACE_TILES rows the one-tile
    // lane banks. The FIFTEEN {name}.png masters the jar's own 16x16
    // colortype-3 tiles bank — NEW tiles the append-only tail seats
    // (717 -> 732: the fifteen {color}_concrete rungs in clinit
    // order), every pre-existing row the same pixels + tints + names
    // keeps, the append-only law the equivalence proof scripts/
    // verify_atlas_full.py banks. The ONE "{name}" registry name its
    // kind ALONE speaks (no strip law — each seat its own Block +
    // BlockItem pairing the Items clinit's registerBlockItems rows
    // answer), the placeable lane the reverse walk the pairing
    // serves, the dig rows + the drop pools + the stone voice the
    // generated tables already bank, the fire trio the family's own
    // SILENCE keeps — no setFlammable row the concrete family wears.
    // The seats the CONTIGUOUS tail the kind ids wear (444..458),
    // the END_PORTAL kind the beyond-table seat the family's growth
    // displaces (444 -> 460 the
    // 658/670/672/673/674/675/676/677/678/53 precedent wears).
    "orange_concrete", "magenta_concrete", "light_blue_concrete",
    "yellow_concrete", "lime_concrete", "pink_concrete",
    "gray_concrete", "light_gray_concrete", "cyan_concrete",
    "purple_concrete", "blue_concrete", "brown_concrete",
    "green_concrete", "red_concrete", "black_concrete",
    // THE POWDER'S OWN FIFTEEN RUNGS (the autopush-680 round): the
    // falling builder's fifteen dyed seats the palette's tail banks
    // — the orange..black rungs the CONCRETE_POWDER collection's own
    // clinit order the tail wears (the WHITE kind its early seat the
    // fifty-sixth entry keeps), the END_PORTAL the beyond-table 475
    // the every-family displacement law keeps. javap Blocks clinit
    // the 26.2 jar (pinned sha1 2dc72797, the probe scripts/
    // jar_work/concrete_powder_680.py + .out banks): pc 20990-21011
    // the getstatic BlockItemIds.CONCRETE_POWDER + the
    // ColorCollection.registerBlocks(..., TriFunction, BiFunction,
    // Function) walk the putstatic Blocks.CONCRETE_POWDER row pc
    // 21011 the sixteen-color collection ONE lambda family speaks
    // (the CONCRETE collection's own register pc 20979-20987 the
    // 679 probe banks — the two collections the sibling walks the
    // same builder banks), the Items clinit row pc 6295-6309 the
    // registerBlockItems(BlockItemIds.CONCRETE_POWDER,
    // Blocks.CONCRETE_POWDER, ...) pairing speaks (the putstatic
    // Items.CONCRETE_POWDER pc 6309), the ConcretePowderBlock's own
    // ctor the FallingBlock.<init> superclass walk the putfield
    // .concrete field read banks (the solidify target the ctor's
    // registry argument carries). The dig rows the generated
    // block_dig table the jar's REAL bootstrapped registry answers
    // (the 0.5 + no-tool rows every color wears — the bare hand the
    // whole family's drop keeps, the shovel's own speed-up the
    // mineable/shovel tag carries), the block.sand.break voice the
    // generated block_sounds table every seat banks, the self-entry
    // drop pool the block_drops table speaks. The blockstates the
    // jar's own ONE shape carry — the {name}.json the single ""
    // variant the cube_all model wears, the models the ONE #all ref
    // (all={name}), the FACE_TILES rows the one-tile lane banks. The
    // FIFTEEN {name}.png masters the jar's own 16x16 colortype-2
    // RGB truecolor tiles bank — NEW tiles the append-only tail
    // seats (732 -> 747: the fifteen {color}_concrete_powder rungs
    // in clinit order), every pre-existing row the same pixels +
    // tints + names keeps, the append-only law the equivalence
    // proof scripts/verify_atlas_full.py banks. The ONE "{name}"
    // registry name its kind ALONE speaks (no strip law — each seat
    // its own Block + BlockItem pairing the Items clinit's
    // registerBlockItems rows answer), the placeable lane the
    // reverse walk the pairing serves, the FALLING family the
    // FallingBlock superclass the [`is_falling_block`] growth
    // banks, the powder->concrete conversion the ctor's own .concrete
    // field the ±15-offset pairing (the white pair's adjacency the
    // 55/56 seats keep) the world walk serves, the fire trio the
    // family's own SILENCE keeps — no setFlammable row the powder
    // family wears. The seats the CONTIGUOUS tail the kind ids wear
    // (459..473), the END_PORTAL kind the beyond-table seat the
    // family's growth displaces (459 -> 475 the
    // 658/670/672/673/674/675/676/677/678/53/679 precedent wears).
    "orange_concrete_powder", "magenta_concrete_powder",
    "light_blue_concrete_powder", "yellow_concrete_powder",
    "lime_concrete_powder", "pink_concrete_powder",
    "gray_concrete_powder", "light_gray_concrete_powder",
    "cyan_concrete_powder", "purple_concrete_powder",
    "blue_concrete_powder", "brown_concrete_powder",
    "green_concrete_powder", "red_concrete_powder",
    "black_concrete_powder",
    // THE GATE'S OWN LADDER (the autopush-681 round): the fence
    // gate's eleven wooden rungs the ladder's tail banks — the
    // spruce..warped pairs the Blocks/Items clinit ladder's own
    // order the tail wears (the OAK pair its early seat the 442/443
    // seats keep), each wood the closed/open ±1 pair the OPEN bit
    // banks, the END_PORTAL the beyond-table 497 the every-family
    // displacement law keeps (the +1 gap id the 679/680 rounds' own
    // seat law wears — the old 475 seat the spruce open kind's own
    // rung re-banks). javap Blocks clinit the 26.2 jar
    // (pinned sha1 2dc72797, the probe scripts/jar_work/
    // gate_ladder_681.py + .out banks): the TWELVE putstatic rows
    // the register(BlockItemId, Function, Properties) walks speak —
    // oak pc 12912, spruce 18697, birch 18742, jungle 18787, acacia
    // 18832, cherry 18877, dark_oak 18922, pale_oak 18967, mangrove
    // 19012, bamboo 19057, crimson 26170 (the nether pair the
    // NO-ignitedByLava arm wears — the Properties chain the fconst_2
    // strength row alone keeps), warped 26212 — the tail order the
    // Items clinit's registerBlock pairing rows the same ladder
    // banks (oak 8363, spruce 8375, birch 8387, jungle 8399, acacia
    // 8411, cherry 8423, dark_oak 8435, pale_oak 8447, mangrove
    // 8459, bamboo 8471, crimson 8483, warped 8495), the WoodType
    // clinit the per-wood `new WoodType(name, BlockSetType)` rows
    // the fence_gate.open/close voice pair every wood the SAME
    // createVariableRangeEvent names speaks. The dig rows the
    // generated block_dig table the jar's REAL bootstrapped
    // registry answers (the 2.0 + no-tool rows every wood wears —
    // the bare hand the whole ladder's drop keeps), the block_sounds
    // rows the plank-sound law the family carries (block.wood.break
    // the ten woods wear, cherry the block.cherry_wood.break stem,
    // bamboo the block.bamboo_wood, crimson/warped the
    // block.nether_wood pair — the planks' own law the gates wear),
    // the self-entry drop pool the block_drops table speaks. The
    // blockstates the jar's own 16-variant maps carry — the facing x
    // in_wall x open rows the facing=south,in_wall=false,open={p}
    // keys resolve, the models the template_fence_gate parents wear
    // (the ONE #texture ref block/{wood}_planks the planks-master
    // chain resolves), the BAMBOO pair the template_custom_fence
    // gate parents its OWN bamboo_fence_gate master speaks (the
    // particle ref the chain never banks). The ONE tile NEW the
    // append-only tail seats (747 -> 748: the bamboo_fence_gate
    // master the planks-masters the ten woods re-wear), every
    // pre-existing row the same pixels + tints + names keeps, the
    // append-only law the equivalence proof scripts/
    // verify_atlas_full.py banks. The ONE "{wood}_fence_gate"
    // registry name BOTH kinds the strip law speaks (the _open
    // suffix the registry never answers — the DoorBlock's own law
    // the gate family wears), the placeable lane the reverse walk
    // the pairing serves, the RMB flip / wire open / re-face /
    // collision lanes the dev-53 family fns every wood the same
    // laws serve, the fire trio the family's own SILENCE keeps —
    // no setFlammable row the gate class wears (the ignitedByLava
    // the flint's own law the place-lane banks). The seats the
    // CONTIGUOUS tail the kind ids wear (474..495), the END_PORTAL
    // kind the beyond-table seat the family's growth displaces
    // (475 -> 497 the
    // 658/670/672/673/674/675/676/677/678/53/679/680 precedent
    // wears).
    "spruce_fence_gate", "spruce_fence_gate_open",
    "birch_fence_gate", "birch_fence_gate_open",
    "jungle_fence_gate", "jungle_fence_gate_open",
    "acacia_fence_gate", "acacia_fence_gate_open",
    "cherry_fence_gate", "cherry_fence_gate_open",
    "dark_oak_fence_gate", "dark_oak_fence_gate_open",
    "pale_oak_fence_gate", "pale_oak_fence_gate_open",
    "mangrove_fence_gate", "mangrove_fence_gate_open",
    "bamboo_fence_gate", "bamboo_fence_gate_open",
    "crimson_fence_gate", "crimson_fence_gate_open",
    "warped_fence_gate", "warped_fence_gate_open",
    // THE NOTE BLOCK'S OWN SEAT (the dev-54 round — the musician's
    // single kind the palette's tail banks, the RMB pitch-ladder
    // instrument the interactive core serves): javap Blocks clinit
    // the 26.2 jar (pinned sha1 2dc72797, the probe
    // scripts/jar_work/note_block_682.py + 682b/682c/682d banks) —
    // the NOTE_BLOCK register row putstatic pc 4113 (the
    // Properties.of() + mapColor(MapColor.WOOD) + .instrument(BASS —
    // the note block's OWN Properties the below-law the wood body
    // wears: a note block seated UNDER another note block the BASS
    // stem speaks) + sound(WOOD) + strength(0.8f) + ignitedByLava()
    // chain the window pc 4072..4113 wears), the Items clinit
    // pairing row pc 7580 (getstatic BlockItemIds.NOTE_BLOCK +
    // registerBlock + putstatic Items.NOTE_BLOCK). The NOTE the 26.2 jar the
    // BLOCK STATE PROPERTY carries — the NoteBlockEntity class the
    // jar RETIRES (absent from the namelist), the note/instrument/
    // powered trio the INSTRUMENT/POWERED/NOTE properties the
    // block's own stateDefinition registers (the ctor's
    // registerDefaultState walk: INSTRUMENT=HARP, NOTE=0,
    // POWERED=false). The blockstate note_block.json the ONE-shape
    // map wears (single "" variant), the model note_block.json the
    // cube_all parent carries (the ONE #all ref), the note_block.png
    // master the jar's own 16x16 colortype-3 tile speaks — the ONE
    // tile NEW the append-only tail seats (748 -> 749), every
    // pre-existing row the same pixels + tints + names keeps, the
    // append-only law the equivalence proof scripts/
    // verify_atlas_full.py banks. The dig row the generated
    // block_dig table the real bootstrapped registry answers (0.8 +
    // no-tool — the bare hand the drop keeps), the block_sounds rows
    // the wood law the family wears (block.wood.break/step/place/
    // hit/fall — the WOOD SoundType the registration's own chain
    // banks), the self-entry drop pool the block_drops table speaks.
    "note_block",
    // THE FENCE FAMILY'S OWN THIRTEEN SEATS (the dev-55 round — the
    // enclosure vocabulary the gate's documented gap closes, the
    // twelve wooden fences + the nether brick fence the palette's
    // tail banks): javap `net.minecraft.world.level.block.FenceBlock`
    // the 26.2 jar (pinned sha1 2dc72797, the probes
    // scripts/jar_work/fence_683.py + fence_683b.py banks) — the
    // ONE FenceBlock class EVERY wood pair registers (the
    // CrossCollisionBlock super the ctor's (4.0F, 16.0F, 4.0F,
    // 16.0F, 24.0F) arg walk carries), the register rows the Blocks
    // clinit pc 10589 (oak) .. 26030 (warped) + 13328 (nether brick)
    // putstatic walk banks — the wood rows the Properties chain
    // mapColor(planks.defaultMapColor()) + instrument(BASS — the
    // note law the family joins) + strength(2.0, 3.0) +
    // ignitedByLava() + sound(WOOD | BAMBOO_WOOD for bamboo,
    // NETHER_WOOD for the crimson/warped pair) wears, the nether
    // brick row the BASEDRUM instrument + requiresCorrectToolForDrops
    // + strength(2.0, 6.0) + sound(NETHER_BRICKS) chain speaks (NO
    // ignitedByLava — the stone family the nether seam keeps), the
    // Items clinit the pairing rows pc 4062..(oak..warped) +
    // 4923 (nether brick) bank. THE CONNECTION LAW the
    // connectsTo(state, sideSolid, dir) walk pc 0-62 speaks — the
    // THREE arms: `!isExceptionForConnection(state) && sideSolid`
    // (the sturdy-face arm the neighbor's isFaceSturdy(opp) read
    // pc 72 feeds — the exception list Block.isExceptionForConnection
    // pc 0-75 banks: LeavesBlock + barrier/carved_pumpkin/jack_o_
    // lantern/melon/pumpkin + the SHULKER_BOXES tag — the client's
    // palette the leaves pair alone seats), `isSameFence(state)`
    // (pc 8 the FENCES-tag read && the WOODEN_FENCES parity pc 10-27
    // wears — the wooden seam the nether brick fence REFUSES: the
    // wood fences connect EVERY wood fence, the nether brick fence
    // the nether brick fence alone), the gate arm pc 13-34 the
    // `block instanceof FenceGateBlock &&
    // FenceGateBlock.connectsToDirection(state, dir)` verdict keeps
    // (the perpendicular law pc 0-28 the gate's own FACING axis the
    // dir.getClockWise().getAxis() equality banks — the fence line
    // THROUGH the gate the panel-axis sides connect, the gate's
    // facing axis the fence never touches). THE COLLISION LAW the
    // CrossCollisionBlock.makeShapes(nodeWidth, height, armWidth,
    // armMinY, armMaxY) walk pc 0-53 speaks: the post
    // Block.column(nodeWidth=4.0, 0.0, height=24.0) — the 4/16-wide
    // 0..24 band the ANTI-JUMP the 1.5-tall law serves (the CenterBand
    // doc's over-tall extension the honest cut retires), the arm
    // Block.boxZ(4.0, 0.0, 24.0, 0.0, 8.0) rotateHorizontal the four
    // when-rows bank — the 4/16-wide 0..24-tall members the cell
    // edge the post band reach. The isPathfindable pc 0-1 the false
    // verdict keeps (the mobs the fence never paths through), the
    // WATERLOGGED the fifth property the ctor's registerDefaultState
    // walk pc 78-85 pins (the variant-less palette the waterlogged
    // half never seats). The blockstates {wood}_fence.json the
    // MULTIPART carry — the UNCONDITIONAL post apply
    // block/{wood}_fence_post + the four side arms the
    // north/east/south/west when-rows (uvlock: true, the y table
    // north 0 / east 90 / south 180 / west 270 the quarter turns
    // speak), the models the template fence_post/fence_side parents
    // resolve (the ONE #texture ref the per-wood planks master
    // banks — the post element [6,0,6]->[10,16,10] the center post
    // walk wears, the side's TWO bar elements the [7,12,0]->
    // [9,15,9] top + [7,6,0]->[9,9,9] lower pair speak). The palette
    // rows the FACE_TILES row the ONE planks master every face
    // banks (the gate arm's law the family keeps), the atlas tile
    // count the ZERO new tiles keeps (the planks + nether_bricks
    // masters the palette already seats — 749 tiles the family
    // rides), the append-only law the equivalence proof scripts/
    // verify_atlas_full.py banks. The dig/sounds/drops rows the
    // name-keyed generated tables the registry answers (the wood
    // rows 2.0 + no-tool + block.wood.* voices, the nether brick row
    // 2.0 + requires-correct-tool + block.nether_bricks.* voices).
    "oak_fence", "spruce_fence", "birch_fence", "jungle_fence",
    "acacia_fence", "cherry_fence", "dark_oak_fence", "pale_oak_fence",
    "mangrove_fence", "bamboo_fence", "crimson_fence", "warped_fence",
    "nether_brick_fence",
    // THE PORTAL'S OWN SEAT (the dev-56 round — the beyond-table 510
    // the palette's own row the walls' contiguous run the tail
    // requires: the END_PORTAL kind the id the displacement law
    // keeps, the row the inert seat the starry-plane lane banks —
    // the mesher's own lane the portal renders, the FACE_TILES row
    // the lane never reads, the filler arm the atlas builder's own
    // walk banks)
    "end_portal",
    // THE WALL FAMILY'S OWN THIRTY-TWO SEATS (the dev-56 round — the
    // rampart vocabulary the WALLS tag the palette's tail banks, the
    // 32 rows the tags/block/walls.json file the probe
    // scripts/jar_work/wall_684.py + wall_684b.py banks): javap
    // `net.minecraft.world.level.block.WallBlock` the 26.2 jar
    // (pinned sha1 2dc72797) — the ONE WallBlock class EVERY row
    // registers, the ctor's registerDefaultState walk the UP=true +
    // NORTH/EAST/SOUTH/WEST=WallSide.NONE + WATERLOGGED=false pin
    // speaks, the makeShapes(FF) pair the two shape maps bank — the
    // VISUAL call makeShapes(16.0F, 14.0F) the POST
    // Block.column(8.0, 0.0, 16.0) box(4,0,4,12,16,12) + the LOW side
    // boxZ(6.0, 0.0, 14.0, 0.0, 11.0) box(5,0,0,11,14,11) + the TALL
    // side boxZ(6.0, 0.0, 16.0, 0.0, 11.0) box(5,0,0,11,16,11) the
    // models' own geometry echoes (template_wall_post [4,0,4]->
    // [12,16,12], template_wall_side [5,0,0]->[11,14,8],
    // template_wall_side_tall [5,0,0]->[11,16,8]), the COLLISION call
    // makeShapes(24.0F, 24.0F) the post column(8.0,0.0,24.0) box(4,0,
    // 4,12,24,12) + the side strips box(5,0,0,11,24,11) BOTH heights
    // the SAME 0..24 band wear — the ANTI-JUMP the 1.5-tall law the
    // fence family's own rung shares. The CONNECTION LAW the
    // connectsTo(state, sideSolid, dir) walk pc 0-68 speaks — the
    // FOUR arms: `state.is(BlockTags.WALLS)` (the wall-to-wall arm
    // EVERY wall kind connects — NO wood/stone parity the fence's
    // isSameFence law wears), `!isExceptionForConnection(state) &&
    // sideSolid` (the sturdy-face arm the fence's own law the same
    // exception list banks), `block instanceof IronBarsBlock` (the
    // bars arm the pane family the connection keeps), the
    // FenceGateBlock.connectsToDirection gate arm (the perpendicular
    // law the fence's own walk verbatim wears). The LOW/TALL verdict
    // the makeWallState(connected, faceShape, TEST_SHAPES_WALL[dir])
    // walk speaks — isCovered(aboveFaceShape, testShape) the
    // `!joinIsNotEmpty(test, face, ONLY_FIRST)` fold pc 0-16 banks
    // (the block ABOVE's collision DOWN-face the test strip covers —
    // the TEST_SHAPES_WALL boxZ(2.0, 16.0, 0.0, 9.0) box(7,0,0,9,16,9)
    // rotateHorizontal the four strips bank), the connected verdict
    // TALL the covered read wears, LOW the uncovered keeps. The UP
    // verdict the shouldRaisePost(state, aboveState, faceShape) walk
    // pc 0-267 banks — the FIVE arms: (1) the above wall's own UP=true
    // read (the column stack the raised chain serves), (2) ALL FOUR
    // sides NONE (the lone post raises), (3) the nNone!=sNone ||
    // eNone!=wNone asymmetric walk (the corner/T-junction the raised
    // center keeps — the straight line n==s && e==w the verdict
    // REFUSES), (4) (north==TALL && south==TALL) ||
    // (east==TALL && west==TALL) the refusal arms (the straight
    // wall line the flat top wears), (5) the above block the
    // WALL_POST_OVERRIDE tag answers (torch/soul_torch/redstone_torch/
    // copper_torch/tripwire/#signs/#banners/#pressure_plates/
    // cactus_flower the tag file banks) || isCovered(faceShape,
    // TEST_SHAPE_POST column(2.0, 0.0, 16.0)) — the covered read the
    // solid roof the raised post keeps. The register rows the Blocks
    // clinit pc 14233 (cobblestone) putstatic walk banks — the
    // Properties.ofLegacyCopy(BASE) + forceSolidOn() chain EVERY row
    // wears (the cobblestone/mossy pair the COBBLESTONE copy speaks,
    // the brick rows their own bases the same legacy law keeps — the
    // is_transparent forceSolidOn verdict the fence's own row the
    // family joins), the dig/sounds/drops rows the name-keyed
    // generated tables the registry answers (each wall row its BASE
    // block's material the legacy copy wears), the blockstates
    // {base}_wall.json the MULTIPART carry — the up=true conditional
    // post apply block/{base}_wall_post + the four side arms the
    // north/east/south/west LOW/TALL when-rows (uvlock: true, the y
    // table north 0 / east 90 / south 180 / west 270 the quarter
    // turns speak), the models the template wall_post/wall_side/
    // wall_side_tall parents resolve (the ONE #texture ref the
    // per-wall BASE master every face banks — the atlas tile count
    // the ZERO new tiles keeps (the cobblestone/brick/sandstone/...
    // masters the palette already seats — 749 tiles the family
    // rides), the append-only law the equivalence proof scripts/
    // verify_atlas_full.py banks).
    "cobblestone_wall", "mossy_cobblestone_wall", "brick_wall",
    "prismarine_wall", "red_sandstone_wall", "mossy_stone_brick_wall",
    "granite_wall", "stone_brick_wall", "nether_brick_wall",
    "andesite_wall", "red_nether_brick_wall", "sandstone_wall",
    "end_stone_brick_wall", "diorite_wall", "blackstone_wall",
    "polished_blackstone_brick_wall", "polished_blackstone_wall",
    "cobbled_deepslate_wall", "polished_deepslate_wall",
    "deepslate_tile_wall", "deepslate_brick_wall", "mud_brick_wall",
    "tuff_wall", "polished_tuff_wall", "tuff_brick_wall",
    "resin_brick_wall", "cinnabar_wall", "polished_cinnabar_wall",
    "cinnabar_brick_wall", "sulfur_wall", "polished_sulfur_wall",
    "sulfur_brick_wall",
    // THE GLASS FAMILY'S OWN SEVENTEEN PANES (the dev-57 round): the
    // window the palette's own transparency frontier — the plain
    // glass the ONE seat the register row's own chain banks + the
    // sixteen stained cloths the DyeColor ladder the collection walk
    // serves. javap Blocks clinit the 26.2 jar (pinned sha1 2dc72797,
    // the probes scripts/jar_work/glass_685.py + 685b/685c/685d banks):
    // the GLASS register row pc 3764..3820 the properties chain
    // speaks — instrument(HAT) pc 3764-3767, strength(0.3f) pc
    // 3770-3773, sound(SoundType.GLASS) pc 3776-3779, noOcclusion()
    // pc 3782, the four predicate folds pc 3785-3814 (isValidSpawn/
    // isRedstoneConductor/isSuffocating/isViewBlocking — the indy
    // lambdas the transparent family's false verdicts wear), the
    // register call pc 3817 the putstatic pc 3820 banks. The
    // GlassBlock class the 26.2 jar RETIRES (absent from the
    // namelist — the StainedGlassBlock/StainedGlassPaneBlock/
    // TintedGlassBlock trio the ONLY glass classes the walk carries):
    // the plain glass the register's Function factory the generic
    // HalfTransparent chain serves. javap HalfTransparentBlock.
    // skipRendering pc 0-17 the CULL LAW the mesher's same-kind arm
    // speaks — the neighborState.is(this) verdict true the shared
    // face the SAME block never draws (glass|glass the cull keeps,
    // glass|stained the DIFFERENT blocks the draw wears), the super
    // fold pc 10-14 the default law keeps. javap StainedGlassBlock
    // the ctor pc 0-10 the DyeColor field stores (the TransparentBlock
    // super the HalfTransparent chain extends — the class header the
    // walk banks), the getColor pc 0-4 the field read keeps; the
    // STAINED_GLASS ladder the ColorCollection.registerBlocks walk
    // pc 11300-11321 banks (the three indy lambdas the
    // TriFunction/BiFunction/Function trio carries — the 16 members
    // the DyeColor ordinals order, the WHITE seat the ladder's head
    // wears). The blockstates ALL SEVENTEEN the ONE "" variant the
    // block/{name} model resolves (the probe's glass/white/orange/
    // black rows bank), the models the parent minecraft:block/cube_all
    // the texture ref the OBJECT form carries — {"force_translucent":
    // true, "sprite": "minecraft:block/{name}"} the 26.2 model
    // format's own render-layer signal (modelgen.rs's
    // TEXTURE_OBJECT_KEYS the closed shape validates, the merge walk
    // the sprite field unwraps). The textures the alpha histograms
    // the render lanes bank (the probe's png walk speaks): the glass
    // 16x16 alpha {0: 191, 255: 65} the BINARY cutout (the frame+
    // streak texels opaque, the panes' interior the discard law
    // wears), the sixteen stained cloths 16x16 alpha {102: 191,
    // 155: 5, 163: 60} the UNIFORM map every color wears (the
    // 0.4 body band the translucent blend serves, the 0.61/0.64
    // border+streak rungs) — the masters PRE-COLORED per dye (white
    // 255/255/255, orange 216/127/51, red 153/51/51 the probe's
    // texel walks speak), NO tintindex the tintless verdict keeps.
    // The loot tables the SILK-TOUCH-ONLY law bank (the probe's
    // glass/white rows speak): the one pool the match_tool
    // silk_touch min-1 condition carries, the bare hand's survival
    // dig the EMPTY drop answers. The Items clinit the pairing rows
    // speak (pc 2317-2326: getstatic BlockItemIds.GLASS →
    // registerBlock(Blocks.GLASS) → putstatic Items.GLASS, the
    // TINTED_GLASS row pc 2329-2338 the follower keeps), the stained
    // ladder the collection's own pairing the item walk serves. The
    // tags the probe's rows bank: mineable/pickaxe the 417 values NO
    // glass row wears (the bare-hand dig the tool row's false keeps),
    // the impermeable tag the 17 glassy members the glass+stained
    // ladder the tinted/barrier companions bank.
    "glass", "white_stained_glass", "orange_stained_glass",
    "magenta_stained_glass", "light_blue_stained_glass",
    "yellow_stained_glass", "lime_stained_glass", "pink_stained_glass",
    "gray_stained_glass", "light_gray_stained_glass",
    "cyan_stained_glass", "purple_stained_glass", "blue_stained_glass",
    "brown_stained_glass", "green_stained_glass", "red_stained_glass",
    "black_stained_glass",
    // THE PANE FAMILY'S OWN EIGHTEEN WINDOWS (the dev-58 round): the
    // thin-strip rung the glass family's own lane wears — the glass
    // pane the ONE seat the register row's own chain banks + the
    // sixteen stained cloths the collection ladder serves + the iron
    // bars the metal seam keeps. javap Blocks clinit the 26.2 jar
    // (pinned sha1 2dc72797, the probes scripts/jar_work/pane_686.py
    // + 686b/686c banks): the IRON_BARS row pc 12293-12328 —
    // requiresCorrectToolForDrops() pc 12304, strength(5.0f, 6.0f)
    // pc 12307-12313, sound(SoundType.IRON) pc 12316-12319,
    // noOcclusion() pc 12322, register + putstatic pc 12325-12328
    // (the COPPER_BARS WeatheringCopperCollection row pc 12331-12357
    // the eight oxidization rungs the FOLLOW-UP round banks — the
    // WeatheringCopper walk the chain carries, the palette the
    // iron seam alone seats), the GLASS_PANE row pc 12430-12465 —
    // instrument(HAT) pc 12441-12444, strength(0.3f) pc 12447-12450,
    // sound(SoundType.GLASS) pc 12453-12456, noOcclusion() pc 12459,
    // register + putstatic pc 12462-12465. javap IronBarsBlock the
    // 26.2 jar: the ctor pc 0-13 the CrossCollisionBlock super
    // (2.0F, 16.0F, 2.0F, 16.0F, 16.0F) args walk speaks — the
    // 2/16-wide 16-tall band the collision makeShapes banks (the
    // post Block.column(2.0, 0.0, 16.0) the [7,0,7]->[9,16,9]
    // footprint wears, the arms the boxZ(2.0, ...) rotateHorizontal
    // the four members keep — the FULL 16-tall band the anti-jump
    // band's absence the standard-height law speaks: the vanilla
    // jump NEVER clears the pane, the 1.0-tall collision the
    // regular block's own answer keeps), the registerDefaultState
    // pc 13-92 the NORTH/EAST/SOUTH/WEST/WATERLOGGED false pins
    // speak. javap StainedGlassPaneBlock the ctor pc 0-10 the
    // IronBarsBlock super the DyeColor field stores (the class
    // header the walk banks — the STAINED_GLASS_PANE collection the
    // ColorCollection.registerBlocks walk banks), the skipRendering
    // override the class RETIRES (absent — the IronBarsBlock law
    // every pane inherits). javap IronBarsBlock.skipRendering pc
    // 0-124 the CULL LAW the mesher's same-kind arm speaks — the
    // neighborState.is(this) arm pc 0-5, the BARS-tag pair arm pc
    // 8-47 (the neighbor AND the state BOTH in the BARS tag — the
    // tag the iron_bars + the eight copper rungs the probe's rows
    // bank, the panes NEVER in — the pair arm the chain|bars law
    // the client's iron-only palette never trips), the vertical
    // axis arm pc 50-61 the TRUE verdict the stacked caps' shared
    // plane culls, the mutual-connection arm pc 62-116 the state's
    // own toward-dir property + the neighbor's opposite property
    // the connected seam faces cull, the super fold pc 117-124 the
    // default false keeps. javap IronBarsBlock.updateShape pc 0-105
    // the connection walk the attachsTo predicate feeds (the
    // WATERLOGGED schedule tick pc 0-33 the fluid pair banks, the
    // horizontal axis arm pc 34-87 the PROPERTY_BY_DIRECTION[dir]
    // the setValue walk serves, the super fold pc 88-105 keeps);
    // javap IronBarsBlock.attachsTo pc 0-36 the THREE arms speak —
    // (1) `!isExceptionForConnection(state) && isFaceSturdy` the
    // sturdy-face arm (the leaves pair the exception verdict the
    // client's palette ALONE seats), (2) `state.getBlock()
    // instanceof IronBarsBlock` the INSTANCE arm EVERY pane and
    // bars kind the class check connects (pane|pane ANY color pair,
    // pane|bars, bars|bars — the whole thin-strip family the ONE
    // class law banks), (3) `state.is(BlockTags.WALLS)` the WALLS
    // arm the 32 wall kinds the dev-56 family connects — the
    // ASYMMETRIC law the vanilla truth wears (the pane connects TO
    // the wall, the wall's own connectsTo the pane never grows).
    // The blockstates ALL EIGHTEEN the MULTIPART carry — the panes
    // the UNCONDITIONAL post apply + the four side arms the
    // north/east y0/y90 + the south/west side_ALT rows + the four
    // NOSIDE arms the unconnected caps speak (the probe's glass/
    // white rows bank), the iron_bars the post_ENDS unconditional
    // + the all-false post arm + the four cap/cap_alt arms the
    // single-connection patterns + the four side/side_alt arms
    // wear. The models the pane templates the ZERO-inner-face law
    // speaks (template_glass_pane_post the down/up cap pair only,
    // template_glass_pane_side the north END face the cullface row
    // carries + the west/east #pane long faces — NO south face the
    // connected seam the z-fight never banks, template_glass_pane_
    // noside the ONE-face cap the unconnected edge wears; the
    // template_bars_* the ZERO-WIDTH planes the crossed-sheet look
    // renders — the post pair x=8/z=8 planes, the side's plane +
    // the END face + the y=0.001/15.999 edge strips), the
    // ambientocclusion: false the pane templates ALL carry. The
    // textures the alpha histograms the render lanes bank (the
    // probe's png walk speaks): the glass_pane_top 16x16 alpha
    // {0: 224, 255: 32} the BINARY cutout (the 2px edge strip
    // opaque), the sixteen stained tops alpha {0: 224, 230: 32}
    // the 0.90 edge band the translucent blend serves, the
    // iron_bars 16x16 alpha {0: 140, 255: 116} the binary cutout
    // (the bars' gaps the discard law wears) — the #pane side refs
    // the GLASS FAMILY's own masters re-wear (the glass the binary
    // cutout, the stained cloths the 0.4 body band the per-texel
    // blend serves), the pane side quads the 26.2 models'
    // force_translucent: true refs the render-layer signal carry.
    "glass_pane", "white_stained_glass_pane", "orange_stained_glass_pane",
    "magenta_stained_glass_pane", "light_blue_stained_glass_pane",
    "yellow_stained_glass_pane", "lime_stained_glass_pane",
    "pink_stained_glass_pane", "gray_stained_glass_pane",
    "light_gray_stained_glass_pane", "cyan_stained_glass_pane",
    "purple_stained_glass_pane", "blue_stained_glass_pane",
    "brown_stained_glass_pane", "green_stained_glass_pane",
    "red_stained_glass_pane", "black_stained_glass_pane",
    "iron_bars",
    // THE COPPER BARS FAMILY'S OWN EIGHT OXIDIZATION RUNGS (the
    // dev-59 round): the WeatheringCopper walk the pane family's own
    // lane wears — the four bare rungs + the four waxed twins the
    // PREFIXES ladder banks. javap Blocks clinit the 26.2 jar (pinned
    // sha1 2dc72797, the probes scripts/jar_work/copper_687.py
    // banks): the COPPER_BARS row pc 12331-12357 the
    // WeatheringCopperCollection.registerBlocks walk the FOUR indy
    // lambdas carry (putstatic pc 12357 the ONE clinit row ALL EIGHT
    // kinds registers — the WAXED_COPPER_BARS putstatic rows the
    // clinit RETIRES), the WeatheringCopperCollection clinit pc 0-62
    // the PREFIXES ladder speaks (the UNAFFECTED/EXPOSED/WEATHERED/
    // OXIDIZED states the ByState pc 4-13 banks, the prefixes the
    // ByState pc 29-53 carries — "" the bare head, "exposed_",
    // "weathered_", "oxidized_", then the waxed twins "waxed_",
    // "waxed_exposed_", "waxed_weathered_", "waxed_oxidized_" — the
    // EIGHT rungs the ladder order the palette tail banks). javap
    // lambda$static$162 the FACTORY lambda speaks — `new
    // IronBarsBlock(properties)` the PLAIN class every rung wears
    // (the instanceof IronBarsBlock arm EVERY copper rung the class
    // check connects, the skipRendering/attachsTo overrides the
    // subclasses never add), javap lambda$static$163 the PROPERTIES
    // lambda — requiresCorrectToolForDrops() pc 3, strength(5.0f,
    // 6.0f) pc 6-12, sound(SoundType.COPPER) pc 15-18, noOcclusion()
    // pc 21. The BARS TAG the pair-cull arm the palette ARMS (the
    // probe's tags/block/bars.json walk the NINE members banks —
    // iron_bars + the eight copper rungs, the panes NEVER in). The
    // blockstates the EIGHT the multipart carry — the four bare rungs
    // the OWN 6-model sets wear ({name}_post_ends/_post/_cap/_cap_
    // alt/_side/_side_alt), the four waxed twins the BASE models
    // RE-WEAR (waxed_copper_bars the copper_bars_* models the
    // blockstate references — the texture-sharing law the vanilla
    // waxed look keeps). The models the template_bars_* parents
    // wear, the edge+bars refs the kind's own master the bare rungs
    // resolve (copper_bars/exposed/weathered/oxidized the FOUR
    // masters the on-demand collection the append-only tail banks:
    // 807 -> 811), the alpha histograms the iron seam's own law
    // answers — {0: 140, 255: 116} the BINARY cutout every rung
    // wears. The dig/sounds/drops rows the name-keyed generated
    // tables the registry answers (the eight rows (5.0, true) the
    // pickaxe demand keeps, the copper voices block.copper.* the
    // SoundType.COPPER row banks, the drop pools the SELF-entry 1.0
    // the iron law wears — the SILK-TOUCH-ONLY law the glass family
    // keeps the copper rungs NEVER wear), the placeable lane the
    // Items pairing rows the generated items table banks.
    "copper_bars", "exposed_copper_bars", "weathered_copper_bars",
    "oxidized_copper_bars", "waxed_copper_bars",
    "waxed_exposed_copper_bars", "waxed_weathered_copper_bars",
    "waxed_oxidized_copper_bars",
    // THE CHAIN FAMILY'S OWN NINE LINKS (the dev-60 round): the
    // hanging rung the pane/bars lane's own element walk wears — the
    // iron chain the clinit row banks + the eight copper rungs the
    // WeatheringCopper walk serves. javap the 26.2 jar (pinned sha1
    // 2dc72797, the probe scripts/jar_work/chain_688.py banks): the
    // IRON_CHAIN row pc 12360-12398 — forceSolidOn() pc 12371,
    // requiresCorrectToolForDrops() pc 12374, strength(5.0f, 6.0f)
    // pc 12377-12383, sound(SoundType.CHAIN) pc 12386-12389,
    // noOcclusion() pc 12392 (the COPPER_CHAIN collection row
    // pc 12401-12427 the eight rungs the SAME walk banks — putstatic
    // pc 12427 the ONE clinit row ALL EIGHT kinds registers), javap
    // lambda$static$165 the chain PROPERTIES lambda — forceSolidOn()
    // pc 3, requiresCorrectToolForDrops() pc 6, strength(5.0f, 6.0f)
    // pc 9-15, sound(SoundType.CHAIN) pc 18-21, noOcclusion() pc 24
    // (the copper rungs the IRON voice the chains KEEP — the bars
    // lambda's COPPER voice the chains never wear, the block.chain.*
    // events the generated table every rung banks). javap ChainBlock
    // the class header the RotatedPillarBlock super the AXIS
    // vocabulary speaks — the registerDefaultState pc 0-41 pins
    // AXIS=Y + WATERLOGGED=false, the <clinit> pc 17-32 the SHAPES
    // map banks (Block.cube(3.0, 3.0, 16.0) the [6,0,6]->[10,16,10]
    // band + Shapes.rotateAllAxis the X/Z arms), getShape pc 0-18
    // the SHAPES.get(AXIS) read, getStateForPlacement pc 20-22 the
    // CLICKED FACE's axis the placement stamps. The blockstates the
    // NINE the variants carry — the axis=y entry the IDENTITY model
    // wears (block/{base} the x=0 y=0 row), the axis=z the x=90
    // turn banks, the axis=x the x=90 + y=90 pair, the WAXED twins
    // the BASE models RE-WEAR (the texture-sharing law the vanilla
    // waxed look keeps). The models the template_chain parent wear —
    // the TWO zero-width planes the ±45° y-axis element rotations
    // speak (shade: false every face keeps), the textures the
    // #texture refs the kind's own master resolves (iron_chain/
    // copper_chain/exposed/weathered/oxidized the FIVE masters the
    // on-demand collection the append-only tail banks: 812 -> 817),
    // the alpha histograms the binary cutout every link wears —
    // {0: 200, 255: 56} the 16x16 walk speaks. The dig/sounds/drops
    // rows the name-keyed generated tables the registry answers (the
    // nine rows (5.0, true) the pickaxe demand keeps, the
    // block.chain.break/step/place/hit events the SoundType.CHAIN
    // row banks, the drop pools the SELF-entry 1.0 the iron law
    // wears), the placeable lane the Items pairing rows the
    // generated items table banks.
    "iron_chain", "copper_chain", "exposed_copper_chain",
    "weathered_copper_chain", "oxidized_copper_chain",
    "waxed_copper_chain", "waxed_exposed_copper_chain",
    "waxed_weathered_copper_chain", "waxed_oxidized_copper_chain",
    // THE LANTERN FAMILY'S OWN ROWS (the dev-61 round, the official
    // 26.2 jar): javap LanternBlock the class header the plain Block
    // super the HANGING + WATERLOGGED properties bank — the
    // registerDefaultState pc 0-41 pins HANGING=false +
    // WATERLOGGED=false, the <clinit> pc 23-65 the SHAPES map speaks
    // (Block.column(4.0, 7.0, 9.0) ∪ Block.column(6.0, 0.0, 7.0) the
    // standing body [5,0,5]->[11,7,11] + top [6,7,6]->[10,9,10] pair
    // banks, SHAPE_HANGING the move(0, 1/16, 0).optimize() the same
    // pair one pixel up wears), getShape pc 0-25 the HANGING read
    // answers, getStateForPlacement pc 12-123 the nearest-looking Y
    // walk the canSurvive fall-through speaks (the first Y direction
    // the HANGING bit arms — dir == UP — the canSurvive walk the
    // candidate verdict gates, the OTHER Y direction the loop's next
    // arm serves, the both-fail walk the aconst_null refusal keeps),
    // canSurvive pc 0-24 the connected direction's opposite the
    // canSupportCenter verdict reads. javap the Blocks clinit the
    // LANTERN row pc 24599-24651 — Properties.of().mapColor(METAL)
    // .forceSolidOn().strength(3.5f).sound(SoundType.LANTERN)
    // .lightLevel(indy).noOcclusion().pushReaction(DESTROY) the chain
    // speaks (putstatic pc 24651, the lightLevel indy the
    // lambda$static$305 `bipush 15` body banks), the SOUL_LANTERN row
    // pc 24654-24706 the SAME chain the soul row wears (putstatic pc
    // 24706, lambda$static$306 the `bipush 10` the dimmer flame
    // banks), the COPPER_LANTERN collection pc 24709-24735 the
    // WeatheringCopperCollection.registerBlocks walk ALL EIGHT rungs
    // registers (putstatic pc 24735, lambda$static$307 the LanternBlock
    // ctor lambda, lambda$static$308 the properties chain — the SAME
    // METAL/forceSolidOn/3.5/LANTERN-sound/noOcclusion/DESTROY rows,
    // lambda$static$309 the `bipush 15` lightLevel the bare and the
    // waxed rungs ALIKE wear). The blockstates the TEN the hanging
    // pair carry — hanging=false the block/{name} model banks,
    // hanging=true the block/{name}_hanging wears, the WAXED twins
    // the BASE models RE-WEAR (the texture-sharing law the vanilla
    // waxed look keeps). The models the template parents wear — the
    // STANDING body [5,0,5]->[11,7,11] + top [6,7,6]->[10,9,10] the
    // SHAPE mirrors, the two hook planes the ±45° y-axis rotations
    // (shade: false every hook face keeps), the HANGING pair one
    // pixel up rides. The textures the alpha histograms answer —
    // 16x48 strips the atlas's own first-frame read the top 16x16
    // sprite the tile banks (the model UVs the v<=15 band the frame
    // never leaves), {0: 450, 255: 318} the binary cutout every
    // lantern wears. The dig/sounds/drops rows the name-keyed
    // generated tables the registry answers (the ten rows (3.5,
    // false) the NO-tool demand keeps — forceSolidOn the
    // requiresCorrectToolForDrops call never wears, the
    // block.lantern.break/step/place/hit events the SoundType.LANTERN
    // row banks, the drop pools the SELF-entry the ten names bank),
    // the placeable lane the Items pairing rows the generated items
    // table banks.
    "lantern", "soul_lantern",
    "copper_lantern", "exposed_copper_lantern",
    "weathered_copper_lantern", "oxidized_copper_lantern",
    "waxed_copper_lantern", "waxed_exposed_copper_lantern",
    "waxed_weathered_copper_lantern", "waxed_oxidized_copper_lantern",
    // THE STAIRS FAMILY'S OWN ROWS (the dev-62 round, the official
    // 26.2 jar): javap StairBlock the class header the plain Block
    // super the FACING + HALF + SHAPE + WATERLOGGED properties bank —
    // the registerDefaultState pc 0-65 pins FACING=NORTH + HALF=BOTTOM
    // + SHAPE=STRAIGHT + WATERLOGGED=false, the <clinit> pc 35-164 the
    // SHAPE walk speaks (SHAPE_OUTER = column(16.0, 0.0, 8.0) ∪
    // box(0, 8, 0, 8, 16, 8) the slab + NW-corner pair banks,
    // SHAPE_STRAIGHT the ∪ rotate(BLOCK_ROT_Y_90) the north-half step
    // completes, SHAPE_INNER the ∪ rotate the east-half arm adds, the
    // SIX map pairs the rotateHorizontal / rotateHorizontal(INVERT_Y)
    // walk banks), getShape pc 0-228 the (shape, half) map pair the
    // direction key answers (STRAIGHT the facing itself, OUTER_LEFT
    // the facing arm, OUTER_RIGHT the getClockWise arm, INNER the
    // facing-key walk the INNER maps serve), getStairsShape pc 0-222
    // the neighbor walk answers (the front walk pos.relative(FACING)
    // the same-half + cross-axis + canTakeShape(gate) verdicts the
    // OUTER_LEFT (bf == facing.getCounterClockWise) / OUTER_RIGHT
    // arms bank, the back walk pos.relative(FACING.getOpposite()) the
    // INNER pair arms), canTakeShape pc 0-62 the no-stairs-OR-
    // different-facing-OR-different-half true verdict keeps,
    // getStateForPlacement pc 0-132 the FACING=
    // getHorizontalDirection() + the HALF clicked-face walk banks
    // (face == DOWN the TOP arm, face == UP the BOTTOM arm, the side
    // faces the `clickLocation.y - pos.y > 0.5` boundary answers) +
    // the SHAPE the getStairsShape pre-compute the same walk banks,
    // updateShape pc 34-62 the horizontal-neighbor walk the SHAPE
    // recompute speaks. javap the Blocks clinit the OAK_STAIRS row +
    // the stone walk + the CUT_COPPER_STAIRS row the
    // WeatheringCopperStairBlock collection the 64 rows register (the
    // four waxed rungs the waxed() picker the base states wear). The
    // blockstates the 64 the SAME 40-key rotation table carry (the
    // probe's identity walk: 60 kinds byte-identical, the four waxed
    // twins the BASE copper models re-wear — the x/y turns the same
    // table keeps) — the base model the facing=east orientation wears
    // (the step the x 8..16 half the template parent banks), the
    // template trio the element boxes speak (block/stairs the slab
    // [0,0,0]->[16,8,16] + the east-half step [8,8,0]->[16,16,16]
    // pair, block/inner_stairs the + [0,8,8]->[8,16,16] SW-quarter
    // arm, block/outer_stairs the [8,8,8]->[16,16,16] SE-quarter
    // step — the shade:true every element keeps). The textures the
    // PARENT masters re-wear (the stairs' own pngs the jar never
    // banks — the bottom/side/top refs the planks/stone/copper rows
    // resolve, the sandstone walk the top/side split keeps), the dig/
    // sounds/drops rows the name-keyed generated tables the registry
    // answers (the wooden rows (2.0, false) the no-tool demand keeps,
    // the stone rows the tool-required verdicts bank, the block.wood/
    // stone/copper/deepslate_tiles break events the SoundType rows
    // speak, the drop pools the SELF-entry the 64 names bank), the
    // placeable lane the Items pairing rows the generated items table
    // banks.
    "oak_stairs", "cobblestone_stairs", "brick_stairs",
    "stone_brick_stairs", "mud_brick_stairs", "resin_brick_stairs",
    "nether_brick_stairs", "sandstone_stairs", "spruce_stairs",
    "birch_stairs", "jungle_stairs", "quartz_stairs", "acacia_stairs",
    "cherry_stairs", "dark_oak_stairs", "pale_oak_stairs",
    "mangrove_stairs", "bamboo_stairs", "bamboo_mosaic_stairs",
    "prismarine_stairs", "prismarine_brick_stairs",
    "dark_prismarine_stairs", "red_sandstone_stairs", "purpur_stairs",
    "polished_granite_stairs", "smooth_red_sandstone_stairs",
    "mossy_stone_brick_stairs", "polished_diorite_stairs",
    "mossy_cobblestone_stairs", "end_stone_brick_stairs", "stone_stairs",
    "smooth_sandstone_stairs", "smooth_quartz_stairs", "granite_stairs",
    "andesite_stairs", "red_nether_brick_stairs",
    "polished_andesite_stairs", "diorite_stairs", "crimson_stairs",
    "warped_stairs", "blackstone_stairs",
    "polished_blackstone_brick_stairs", "polished_blackstone_stairs",
    "tuff_stairs", "polished_tuff_stairs", "tuff_brick_stairs",
    "sulfur_stairs", "polished_sulfur_stairs", "sulfur_brick_stairs",
    "cinnabar_stairs", "polished_cinnabar_stairs",
    "cinnabar_brick_stairs", "cut_copper_stairs",
    "exposed_cut_copper_stairs", "weathered_cut_copper_stairs",
    "oxidized_cut_copper_stairs", "waxed_cut_copper_stairs",
    "waxed_exposed_cut_copper_stairs", "waxed_weathered_cut_copper_stairs",
    "waxed_oxidized_cut_copper_stairs", "cobbled_deepslate_stairs",
    "polished_deepslate_stairs", "deepslate_tile_stairs",
    "deepslate_brick_stairs",
    // THE SLAB FAMILY'S OWN ROWS (the dev-63 round, the official
    // 26.2 jar): javap SlabBlock the class header the TYPE +
    // WATERLOGGED properties bank — the registerDefaultState pins
    // TYPE=BOTTOM, the <clinit> pc 23-48 the SHAPE pair speaks
    // (SHAPE_BOTTOM = column(16.0, 0.0, 8.0) the 0..8/16 band the
    // bottom half banks, SHAPE_TOP = column(16.0, 8.0, 16.0) the
    // upper half wears, the DOUBLE state Shapes.block() the full
    // cube answers), getStateForPlacement pc 0-156 the clicked-face
    // walk banks (face DOWN the TOP arm, face UP the BOTTOM fall,
    // the side faces the `clickLocation.y - pos.y > 0.5` boundary —
    // the FIRST branch the same-slab cell the DOUBLE completion
    // speaks + WATERLOGGED=false pin), canBeReplaced pc 0-150 the
    // doubling walk answers (the BOTTOM arm the face==UP OR the
    // upper-half side-click true verdict, the TOP arm the face==DOWN
    // OR the lower-half side-click — the same-kind item gate the
    // pair completion keeps), useShapeForLightOcclusion the
    // type!=DOUBLE verdict the half slabs the light the flood
    // passes. javap the Blocks clinit the 61 slab rows the
    // strength/sound chains speak (the wooden rows (2.0, 3.0) the
    // WOOD/BAMBOO_WOOD/CHERRY_WOOD/NETHER_WOOD voices the
    // ignitedByLava row keeps, the stone rows (2.0, 6.0) the
    // requiresCorrectToolForDrops pickaxe verdicts bank, the
    // prismarine trio (1.5, 6.0), the legacy rows the
    // ofLegacyCopy/registerSlab parent copies wear), the
    // CUT_COPPER_SLAB collection row pc 29302-29328 the
    // WeatheringCopperSlabBlock walk registers (lambda$static$370
    // the ofFullCopy(CUT_COPPER.weathering().pick(state)) the
    // copper slab rows the parent's own (3.0, 6.0) + COPPER voice
    // wears, the four waxed rungs the plain SlabBlock ctor the
    // lambda$static$369 banks), the blockstates the 68 the SAME
    // three-key type map carry (type=bottom/top/double — the waxed
    // twins the BASE copper models re-wear, the smooth_stone_slab
    // double the block/smooth_stone_slab_double special model
    // wears, the petrified twin the block/oak_planks parent
    // answers), the models block/slab + block/slab_top the element
    // pair speak (the bottom [0,0,0]->[16,8,16] the down/up
    // cullfaces the band banks, the top [0,8,0]->[16,16,16] the
    // mirrored pair wears — the sandstone walk the bottom/side/top
    // split rows keep, the smooth_stone_slab the slab_side special
    // master the ONLY own png banks), the dig/sounds/drops rows the
    // name-keyed generated tables the registry answers (the FULL
    // 1196-block runtime dump the slab rows already bank), the
    // placeable lane the Items pairing rows the generated items
    // table banks.
    "resin_brick_slab", "prismarine_slab", "prismarine_brick_slab",
    "dark_prismarine_slab", "oak_slab", "spruce_slab", "birch_slab",
    "jungle_slab", "acacia_slab", "cherry_slab", "dark_oak_slab",
    "pale_oak_slab", "mangrove_slab", "bamboo_slab", "bamboo_mosaic_slab",
    "stone_slab", "smooth_stone_slab", "sandstone_slab",
    "cut_sandstone_slab", "petrified_oak_slab", "cobblestone_slab",
    "brick_slab", "stone_brick_slab", "mud_brick_slab",
    "nether_brick_slab", "quartz_slab", "red_sandstone_slab",
    "cut_red_sandstone_slab", "purpur_slab", "polished_granite_slab",
    "smooth_red_sandstone_slab", "mossy_stone_brick_slab",
    "polished_diorite_slab", "mossy_cobblestone_slab",
    "end_stone_brick_slab", "smooth_sandstone_slab",
    "smooth_quartz_slab", "granite_slab", "andesite_slab",
    "red_nether_brick_slab", "polished_andesite_slab", "diorite_slab",
    "crimson_slab", "warped_slab", "blackstone_slab",
    "polished_blackstone_brick_slab", "polished_blackstone_slab",
    "tuff_slab", "polished_tuff_slab", "tuff_brick_slab",
    "sulfur_slab", "polished_sulfur_slab", "sulfur_brick_slab",
    "cinnabar_slab", "polished_cinnabar_slab", "cinnabar_brick_slab",
    "cut_copper_slab", "exposed_cut_copper_slab",
    "weathered_cut_copper_slab", "oxidized_cut_copper_slab",
    "waxed_cut_copper_slab", "waxed_exposed_cut_copper_slab",
    "waxed_weathered_cut_copper_slab", "waxed_oxidized_cut_copper_slab",
    "cobbled_deepslate_slab", "polished_deepslate_slab",
    "deepslate_tile_slab", "deepslate_brick_slab",
    // THE CHEST FAMILY'S OWN SEATS (the dev-64 round — the 11 kinds
    // the vanilla registration order the clinit rows speak: the
    // chest the head row banks, the trapped/ender pair the middle
    // rows wear, the copper ladder the collection walk registers
    // (the WeatheringCopperCollection.registerBlocks order — the
    // PREFIXES ladder the bare quartet + the waxed twins speak); the
    // BE-rendered family the blockstate's dummy model never carries
    // — the composite-tile arm the entity sheets answer).
    "chest", "trapped_chest", "ender_chest",
    "copper_chest", "exposed_copper_chest", "weathered_copper_chest",
    "oxidized_copper_chest", "waxed_copper_chest", "waxed_exposed_copper_chest",
    "waxed_weathered_copper_chest", "waxed_oxidized_copper_chest",
    // THE BARREL'S OWN SEAT (the dev-65 round — the ONE kind the
    // Blocks clinit's BARREL row banks: mapColor+instrument(BASS).
    // strength(2.5).sound(WOOD).ignitedByLava the chain speaks, the
    // FACING(6)+OPEN(2) blockstate the 12-key map carries — the
    // full-cube family the generic walk's first-variant read
    // resolves [the closed model block/barrel the cube_bottom_top
    // parent wears — the x/y turns the apply's own rotations bank,
    // the faces the ONE model every state shares]).
    "barrel",
    // THE MOB HEADS' OWN SEVEN SEATS (the dev-80 round — the floor
    // kinds the vanilla registration order the block_dig table's own
    // rows banks: skeleton_skull, wither_skeleton_skull, zombie_head,
    // player_head, creeper_head, dragon_head, piglin_head — the
    // wall pair the same clinit window registers the floor kinds'
    // tail keeps [the wall kinds the future round seats, the floor
    // vocabulary the note block's above consult alone reads]). The
    // NOTE BLOCK'S OWN ABOVE LAW the seats serve (javap
    // NoteBlock.setInstrument pc 182-190 the 682d probe banks — the
    // Properties rows: SKELETON_SKULL -> SKELETON,
    // WITHER_SKELETON_SKULL -> WITHER_SKELETON, ZOMBIE_HEAD ->
    // ZOMBIE, PLAYER_HEAD -> CUSTOM_HEAD, CREEPER_HEAD -> CREEPER,
    // DRAGON_HEAD -> DRAGON, PIGLIN_HEAD -> PIGLIN), the stems the
    // NoteBlockInstrument clinit carries (the MOB_HEAD type the
    // imitate family wears: "imitate.skeleton",
    // "imitate.wither_skeleton", "imitate.zombie", "imitate.creeper",
    // "imitate.ender_dragon", "imitate.piglin" — the sounds.json rows
    // the client's voice lane banks; the CUSTOM_HEAD the CUSTOM type
    // wears, the UI_BUTTON_CLICK holder the clinit row banks — the
    // head's own profile the getCustomSoundId walk reads, the null
    // the offline palette never carries). THE BE-RENDERED FAMILY'S
    // OWN PLACEHOLDER (the chest precedent's law the family shares):
    // the blockstate's "" variant -> the bare block/skull model the
    // PARTICLE row alone carries (block/soul_sand — the seven
    // blockstates the ONE model every head wears), the six-face walk
    // the chain demands would bail — the particle stem the resolve
    // walk's placeholder banks, the tile the EXISTING soul_sand row
    // re-wears (the ZERO-or-append tail keeps, 870 -> 870 the tile
    // count never moves). The face the BE lane renders (the
    // entity/skeleton/skeleton sheets the SkullBlockEntityRenderer
    // serves) the mini-cube mesher lane the future round keeps —
    // the palette seats the kind vocabulary + the above consult the
    // note block's law reads, the mesher the generic full-cube
    // verdict every unseated lane wears.
    "skeleton_skull", "wither_skeleton_skull", "zombie_head",
    "player_head", "creeper_head", "dragon_head", "piglin_head",
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

/// THE CHEST FAMILY'S OWN CENSUS (the dev-64 round): the 11 kinds the
/// palette tail banks — the placeholder arm the BE-rendered family the
/// blockstate's dummy model never resolves, the composite-tile arm the
/// entity sheets answers.
fn is_chest_family(block: &str) -> bool {
    matches!(
        block,
        "chest"
            | "trapped_chest"
            | "ender_chest"
            | "copper_chest"
            | "exposed_copper_chest"
            | "weathered_copper_chest"
            | "oxidized_copper_chest"
            | "waxed_copper_chest"
            | "waxed_exposed_copper_chest"
            | "waxed_weathered_copper_chest"
            | "waxed_oxidized_copper_chest"
    )
}

/// THE MOB HEADS' OWN CENSUS (the dev-80 round): the seven floor kinds
/// the palette tail banks — the placeholder arm the BE-rendered family
/// the blockstate's bare-particle model never resolves (the seven
/// blockstates the ONE block/skull model the particle row alone
/// carries — the six-face walk the chain demands would bail).
fn is_skull_family(block: &str) -> bool {
    matches!(
        block,
        "skeleton_skull"
            | "wither_skeleton_skull"
            | "zombie_head"
            | "player_head"
            | "creeper_head"
            | "dragon_head"
            | "piglin_head"
    )
}

/// THE CHEST FAMILY'S OWN SHEET (the tile prefix + the entity sheet the
/// renderer wears — the waxed kinds the BASE family's sheet the vanilla
/// law keeps, no waxed sheet the jar banks).
fn chest_family_sheet(block: &str) -> Option<(&'static str, &'static str)> {
    match block {
        "chest" => Some(("chest", "entity/chest/normal")),
        "trapped_chest" => Some(("trapped_chest", "entity/chest/trapped")),
        "ender_chest" => Some(("ender_chest", "entity/chest/ender")),
        "copper_chest" | "waxed_copper_chest" => {
            Some(("copper_chest", "entity/chest/copper"))
        }
        "exposed_copper_chest" | "waxed_exposed_copper_chest" => Some((
            "exposed_copper_chest",
            "entity/chest/copper_exposed",
        )),
        "weathered_copper_chest" | "waxed_weathered_copper_chest" => Some((
            "weathered_copper_chest",
            "entity/chest/copper_weathered",
        )),
        "oxidized_copper_chest" | "waxed_oxidized_copper_chest" => Some((
            "oxidized_copper_chest",
            "entity/chest/copper_oxidized",
        )),
        _ => None,
    }
}

/// THE SHEET'S OWN CROP (the region the texOffs law speaks — the RGBA8
/// bytes the [x0,y0,x1,y1) px window banks).
fn crop_sheet(rgba: &[u8], w: u32, x0: u32, y0: u32, x1: u32, y1: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(((x1 - x0) * (y1 - y0) * 4) as usize);
    for y in y0..y1 {
        for x in x0..x1 {
            let i = ((y * w + x) * 4) as usize;
            out.extend_from_slice(&rgba[i..i + 4]);
        }
    }
    out
}

/// THE CHEST TILES' OWN COMPOSITION (the four 16x16 RGBA8 tiles the
/// chest lane's faces sample — front/side/top/bottom). The side/front
/// tiles the 14-row band wear (the lid strip the rows 0..5 keep, the
/// body's exposed 9 rows the rows 5..14 bank, the 15th row the body
/// base trim the doubles' 15-tall faces sample), the top/bottom tiles
/// the centered 1..15 band keep, the lock the front tile's rows 2..5
/// carry (the latch the seam's own band the vanilla look speaks).
/// The sheet regions the ASCII probe banks: lid up (14,0)-(28,14),
/// lid front (14,14)-(28,19), lid side (28,14)-(42,19), body down
/// (28,19)-(42,33), body front (14,33)-(28,43), body side
/// (28,33)-(42,43), lock (1,1)-(3,4).
fn compose_chest_tiles(rgba: &[u8], w: u32, _h: u32) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let blank = [0u8; 16 * 16 * 4];
    let compose_face = |lid_y: u32, body_x: u32| -> Vec<u8> {
        // the vertical composition: the lid strip (rows 0..5) + the
        // body's exposed rows (5..14) + the base extension (row 14)
        let mut tile = blank.to_vec();
        let put = |tile: &mut Vec<u8>, tx: u32, ty: u32, sx: u32, sy: u32| {
            let i = ((ty * 16 + tx) * 4) as usize;
            let j = ((sy * w + sx) * 4) as usize;
            tile[i..i + 4].copy_from_slice(&rgba[j..j + 4]);
        };
        // the lid strip: face cols 1..15 <- sheet cols lid_x..lid_x+14
        let lid_x = 14u32;
        for fy in 0..5u32 {
            for fx in 0..14u32 {
                put(&mut tile, 1 + fx, fy, lid_x + fx, lid_y + fy);
            }
        }
        // the body's exposed 9 rows: face rows 5..14 <- sheet rows
        // body_y+1..body_y+10 (the top row the lid covers)
        let body_y = 33u32;
        for fy in 5..14u32 {
            for fx in 0..14u32 {
                put(&mut tile, 1 + fx, fy, body_x + fx, body_y + 1 + (fy - 5));
            }
        }
        // the base trim extension (the doubles' 15th row)
        for fx in 0..14u32 {
            put(&mut tile, 1 + fx, 14, body_x + fx, body_y + 9);
        }
        tile
    };
    // FRONT: the lid's front strip (14,14) + the body's front + the lock
    let mut front = compose_face(14, 14);
    // the lock's 2x3 latch: face (7..9, 2..5) <- sheet (1..3, 1..4)
    for ly in 0..3u32 {
        for lx in 0..2u32 {
            let i = (((2 + ly) * 16 + (7 + lx)) * 4) as usize;
            let j = (((1 + ly) * w + (1 + lx)) * 4) as usize;
            front[i..i + 4].copy_from_slice(&rgba[j..j + 4]);
        }
    }
    // SIDE: the lid's side strip (28,14) + the body's side (28,33)
    let side = compose_face(28, 28);
    // TOP: the lid's up face centered (14,0)-(28,14)
    let mut top = blank.to_vec();
    for ty in 0..14u32 {
        for tx in 0..14u32 {
            let i = (((1 + ty) * 16 + (1 + tx)) * 4) as usize;
            let j = (((ty) * w + (14 + tx)) * 4) as usize;
            top[i..i + 4].copy_from_slice(&rgba[j..j + 4]);
        }
    }
    // BOTTOM: the body's down face centered (28,19)-(42,33)
    let mut bottom = blank.to_vec();
    for ty in 0..14u32 {
        for tx in 0..14u32 {
            let i = (((1 + ty) * 16 + (1 + tx)) * 4) as usize;
            let j = (((19 + ty) * w + (28 + tx)) * 4) as usize;
            bottom[i..i + 4].copy_from_slice(&rgba[j..j + 4]);
        }
    }
    let _ = crop_sheet; // the crop helper the composition arms share
    (front, side, top, bottom)
}

fn tint_for_texture(stem: &str) -> u32 {
    // Grayscale master textures the game tints in-world (plains biome here).
    match stem {
        "grass_block_top" => TINT_GRASS,
        "oak_leaves" => TINT_FOLIAGE,
        "water_still" => TINT_WATER,
        // THE FLOW MASTER'S OWN ROW (the autopush-529 round): the
        // water_flow strip the fluid SIDE faces wear the SAME in-world
        // biome tint the still master wears — javap FluidRenderer the
        // ARGB tint the per-STATE color the render carries (the tint
        // rides the fluid, never the sprite), the grayscale flow
        // master the same TINT_WATER bake the water_still rows speak.
        "water_flow" => TINT_WATER,
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

    // THE ORIENTABLE FURNACE TRIO'S OWN ROWS (the autopush-601
    // furnace arm, the dev-26 round GENERALIZED to the blast furnace
    // and the smoker — the same blockstate shape the three
    // registrations speak: facing x lit -> the orientable model, the
    // y-rotation the variant byte carries, the palette's kind-only
    // vocabulary the facing=north rows resolve). The LIT kinds the
    // SAME <stem>.json's "facing=north,lit=true" rows resolve (block/
    // <stem>_on — the <stem>_front_on the #front ref answers; the
    // blockstate has NO <stem>_lit.json — the half-kind name the
    // campfire_lit precedent wears). The model chains block/<stem> ->
    // block/orientable -> block/orientable_with_bottom -> block/cube
    // the full-cube element walk resolves (north=#front, east/south/
    // west=#side, up=#top, down=#bottom or #top — the vanilla's own
    // authoring; the smoker's #bottom the smoker_bottom.png answers),
    // tinted = false (no tintindex anywhere). The dig rows the
    // registrations' requiresCorrectToolForDrops pc 7179 / 24278 /
    // 24232 + strength(3.5f) pc 7182-7185 / 24281-24284 / 24235-24238
    // the clinit walks speak; the lightLevel the bipush 13
    // litBlockEmission pc 7188-7190 / 24287-24292 / 24241-24246 pairs
    // bank (the lit state's own 13, the unlit the 0 — light.rs's rows
    // keep).
    let orientable_stem = ["furnace", "blast_furnace", "smoker"].into_iter().find(|stem| {
        *stem == block || block.strip_suffix("_lit") == Some(*stem)
    });
    if let Some(stem) = orientable_stem {
        let lit = block.ends_with("_lit");
        let bs_bytes =
            read_jar(archive, &format!("assets/minecraft/blockstates/{stem}.json"))?;
        let bs: Value = serde_json::from_slice(&bs_bytes)
            .with_context(|| format!("blockstate {stem}: json parse"))?;
        let model_ref = bs
            .get("variants")
            .and_then(|v| v.get(format!("facing=north,lit={lit}").as_str()))
            .and_then(|v| v.get("model"))
            .and_then(|m| m.as_str())
            .with_context(|| format!("blockstate {stem}: no facing=north,lit={lit} model"))?
            .trim_start_matches("minecraft:")
            .to_string();
        return full_cube_faces(archive, &model_ref)
            .with_context(|| format!("block {block}: model chain {model_ref}"));
    }

    // THE PORTAL'S OWN FLAT ROW (the autopush-605 round): the gate the
    // ONE-element models resolve (block/nether_portal_ns the axis=x
    // variant, block/nether_portal_ew the axis=z — the thin planes the
    // TWO faces the #portal ref carries, the full-cube walk's 6-face
    // demand the elements never speak), the FLAT STEM the palette row
    // banks (the strip's frame-0 crop the first_frame read serves —
    // the same law the water/lava fluid rows keep), the mesher's axis
    // lane the derive walk the REAL geometry serves (the element
    // tables both models bank, the per-quad #portal verdict the
    // NETHER_PORTAL strip consts answer)
    if block == "nether_portal" {
        return Ok(resolve_flat_stem("nether_portal", false));
    }

    // THE REDSTONE TORCH'S OWN ROWS (the autopush-666 round — javap
    // RedstoneTorchBlock the 26.2 jar: the blockstates'
    // redstone_torch.json the TWO rows the lit bit wears — the lit
    // kind the "lit=true" row's block/redstone_torch model answers,
    // the off kind the "lit=false" row's block/redstone_torch_off
    // model keeps; the per-row arm the candle pair's own walk the
    // same shape serves, the generic walk's FIRST-variant read the
    // off model both kinds would wrongly wear). The LIT model's
    // parent template_redstone_torch the SEVEN-element chain banks
    // (the stick box [7,0,7]->[9,10,9] the SIX-face row the full-cube
    // finder answers + the SIX tiny glow planes the finder's break
    // arm skips — the first six-face element the tile walk keeps),
    // the OFF model's template_torch_unlit parent the stick box
    // ALONE banks (the shade field the unlit element never carries —
    // the model-level ambientocclusion: false the lane's own law
    // wears).
    if block == "redstone_torch" || block == "redstone_torch_off" {
        let lit = block == "redstone_torch";
        let bs_bytes =
            read_jar(archive, "assets/minecraft/blockstates/redstone_torch.json")?;
        let bs: Value = serde_json::from_slice(&bs_bytes)
            .with_context(|| "blockstate redstone_torch: json parse")?;
        let model_ref = bs
            .get("variants")
            .and_then(|v| v.get(if lit { "lit=true" } else { "lit=false" }))
            .and_then(|v| v.get("model"))
            .and_then(|m| m.as_str())
            .with_context(|| "blockstate redstone_torch: no lit model")?
            .trim_start_matches("minecraft:")
            .to_string();
        return full_cube_faces(archive, &model_ref)
            .with_context(|| format!("block {block}: model chain {model_ref}"));
    }

    // THE LAMP'S OWN ROWS (the autopush-672 round — the
    // blockstates' redstone_lamp.json the TWO rows the lit bit wears
    // — the lit kind the "lit=true" row's block/redstone_lamp_on
    // model answers, the unlit kind the "lit=false" row's
    // block/redstone_lamp model keeps; the per-row arm the torch
    // pair's own walk the same shape serves, the generic walk's
    // FIRST-variant read the wrong model both kinds would wear — the
    // vanilla json the lit=true arm the SECOND variant keys, the
    // alphabetical first read the unlit model both rows would wear).
    // BOTH models the cube_all template the ONE #all texture each
    // banks (block/redstone_lamp the terracotta-brown row, the lit
    // arm the block/redstone_lamp_on amber row the regen tiles), the
    // full-cube finder the six-face row the plain walk answers.
    if block == "redstone_lamp" || block == "redstone_lamp_on" {
        let lit = block == "redstone_lamp_on";
        let bs_bytes =
            read_jar(archive, "assets/minecraft/blockstates/redstone_lamp.json")?;
        let bs: Value = serde_json::from_slice(&bs_bytes)
            .with_context(|| "blockstate redstone_lamp: json parse")?;
        let model_ref = bs
            .get("variants")
            .and_then(|v| v.get(if lit { "lit=true" } else { "lit=false" }))
            .and_then(|v| v.get("model"))
            .and_then(|m| m.as_str())
            .with_context(|| "blockstate redstone_lamp: no lit model")?
            .trim_start_matches("minecraft:")
            .to_string();
        return full_cube_faces(archive, &model_ref)
            .with_context(|| format!("block {block}: model chain {model_ref}"));
    }

    // THE REDSTONE WALL TORCH'S OWN ROWS (the autopush-667 round — the
    // blockstates' redstone_wall_torch.json the 8-variant map the
    // facing=east rows resolve (the base model the NO-y row wears —
    // the per-row arm the standing torch's own walk serves): lit=true
    // the block/redstone_wall_torch model answers, lit=false the
    // block/redstone_wall_torch_off row keeps; the lit model's parent
    // template_redstone_torch_wall the SEVEN-element chain banks (the
    // tilted stick box [-1,3.5,7]->[1,13.5,9] the -22.5° rotZ the
    // derive walk bakes + the SIX glow planes), the off model's
    // template_torch_wall_unlit parent the stick box ALONE keeps — the
    // full-cube finder the FIRST six-face element the tile walk
    // answers, the same sprites the standing pair's seats bank (the
    // #torch ref the redstone_torch / redstone_torch_off masters
    // answer — the atlas's tile set the wall pair never grows).
    if block == "redstone_wall_torch" || block == "redstone_wall_torch_off" {
        let lit = block == "redstone_wall_torch";
        let bs_bytes =
            read_jar(archive, "assets/minecraft/blockstates/redstone_wall_torch.json")?;
        let bs: Value = serde_json::from_slice(&bs_bytes)
            .with_context(|| "blockstate redstone_wall_torch: json parse")?;
        let model_ref = bs
            .get("variants")
            .and_then(|v| {
                v.get(if lit {
                    "facing=east,lit=true"
                } else {
                    "facing=east,lit=false"
                })
            })
            .and_then(|v| v.get("model"))
            .and_then(|m| m.as_str())
            .with_context(|| "blockstate redstone_wall_torch: no facing=east model")?
            .trim_start_matches("minecraft:")
            .to_string();
        return full_cube_faces(archive, &model_ref)
            .with_context(|| format!("block {block}: model chain {model_ref}"));
    }

    // THE OAK TRAPDOOR'S OWN ROWS (the autopush-668 round — javap
    // TrapDoorBlock the 26.2 jar, scripts/jar_work/trapdoor_668.txt
    // banks): the blockstates' oak_trapdoor.json the 16-row facing x
    // half x open map the per-row arm resolves (the generic walk's
    // FIRST-variant read the open kind would wrongly wear — the first
    // row facing=east,half=bottom,open=false the CLOSED model both
    // kinds would bank). The CLOSED kind the "facing=east,half=bottom,
    // open=false" row's block/oak_trapdoor_bottom model answers (the
    // parent template_trapdoor_bottom the ONE-element chain [0,0,0]->
    // [16,3,16] the six-face row the full-cube finder answers — the
    // #texture ref the block/oak_trapdoor sprite the tile walk keeps),
    // the OPEN kind the "facing=north,half=bottom,open=true" row's
    // block/oak_trapdoor_open model keeps (the parent
    // template_trapdoor_open the [0,0,13]->[16,16,16] north-strip
    // element the same #texture ref wears — the SAME sprite both
    // kinds' tiles bank, the atlas's tile set the pair grows by
    // exactly ONE). The HALF=top closed state the mesher's own model
    // choice serves (the block/oak_trapdoor_top parent the [0,13,0]
    // chain the variant byte's half bit picks — the KIND's tile the
    // same sprite the three models share). THE LADDER THE SAME WALK
    // WEARS (the autopush-669 round — the eleven wooden rungs the
    // family base scan serves, the {base}_trapdoor.json rows the
    // per-wood arms read, the probe scripts/jar_work/trapdoor_woods_
    // 669.txt banks).
    if let Some(base) = trapdoor_family_base(block) {
        let open = block.ends_with("_open");
        let key = if open {
            "facing=north,half=bottom,open=true"
        } else {
            "facing=east,half=bottom,open=false"
        };
        let bs_bytes = read_jar(
            archive,
            &format!("assets/minecraft/blockstates/{base}.json"),
        )?;
        let bs: Value = serde_json::from_slice(&bs_bytes)
            .with_context(|| format!("blockstate {base}: json parse"))?;
        let model_ref = bs
            .get("variants")
            .and_then(|v| v.get(key))
            .and_then(|v| v.get("model"))
            .and_then(|m| m.as_str())
            .with_context(|| format!("blockstate {base}: no {key} model"))?
            .trim_start_matches("minecraft:")
            .to_string();
        return full_cube_faces(archive, &model_ref)
            .with_context(|| format!("block {block}: model chain {model_ref}"));
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

    full_cube_faces(archive, &model_ref).with_context(|| format!("block {block}: model chain {model_ref}"))
}

/// The full-cube face walk ONE resolved model wears (the generic
/// resolve_block's own tail, extracted so the furnace rows' variant
/// selection shares the exact walk): the first element defining all six
/// faces answers, each face's texture ref the merged texture map
/// resolves, the tintindex the tinted verdict keeps.
fn full_cube_faces(
    archive: &mut ZipArchive<std::io::Cursor<&[u8]>>,
    model_ref: &str,
) -> Result<[ResolvedFace; 6]> {
    let model = load_model_chain(archive, model_ref)
        .with_context(|| format!("model chain {model_ref}"))?;

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

/// THE DOOR FAMILY'S OWN GATE (the dev-43 round): the palette rows
/// the twelve wooden quartets wear — the `{wood}_door` base the
/// row arm's blockstate read keys. The copper ladder (the eight
/// oxidization states) and the iron door the next lane's rows bank
/// (the canOpenByHand law the set-type walk already banked). The
/// list the row arm AND the TILE-const emit both read — the one
/// table the family's data lanes share.
const DOOR_FAMILY_BASES: [&str; 21] = [
    "oak_door", "spruce_door", "birch_door", "jungle_door", "acacia_door",
    "dark_oak_door", "mangrove_door", "cherry_door", "pale_oak_door",
    "bamboo_door", "crimson_door", "warped_door",
    // THE COPPER LADDER + IRON (the autopush-656 round): the eight
    // oxidization states the COPPER set type shares, the iron door
    // the canOpenByHand refusal keeps — the blockstate rows the real
    // chain resolves (the waxed bases the UNWAXED models the jar's
    // own rows serve).
    "copper_door", "exposed_copper_door", "weathered_copper_door",
    "oxidized_copper_door", "waxed_copper_door", "waxed_exposed_copper_door",
    "waxed_weathered_copper_door", "waxed_oxidized_copper_door", "iron_door",
];

/// THE DOOR FAMILY'S OWN TEXTURED BASES (the autopush-656 round): the
/// bases whose #bottom/#top masters exist as jar textures — the
/// TILE-const emit's walk keys (the WAXED bases the list never
/// carries: their blockstates the UNWAXED models the real chain
/// serves, the per-oxidization textures the waxed kinds inherit —
/// the resolved stem the mesher's pair table the unwaxed rows
/// answer).
const DOOR_TEXTURED_BASES: [&str; 17] = [
    "oak_door", "spruce_door", "birch_door", "jungle_door", "acacia_door",
    "dark_oak_door", "mangrove_door", "cherry_door", "pale_oak_door",
    "bamboo_door", "crimson_door", "warped_door",
    "copper_door", "exposed_copper_door", "weathered_copper_door",
    "oxidized_copper_door", "iron_door",
];

/// The palette row's `{wood}_door` base the family gate answers (the
/// `_upper`/`_open` suffixes the strip walk peels — the `_open` FIRST
/// the both-suffix word the upper-open kind wears, the `_right` tail
/// the peel's own first word the autopush-658 round serves), the
/// non-family rows the None the generic arms serve.
fn door_family_base(block: &str) -> Option<&'static str> {
    // THE HINGE-RIGHT SUFFIX THE PEEL FIRST ANSWERS (the autopush-658
    // round — the RIGHT rows the _right word the both-suffix stems
    // carry, the strip the upper/open flags' own reads serve)
    let stem = block.strip_suffix("_right").unwrap_or(block);
    let stem = stem.strip_suffix("_open").unwrap_or(stem);
    let stem = stem.strip_suffix("_upper").unwrap_or(stem);
    DOOR_FAMILY_BASES
        .iter()
        .copied()
        .find(|b| *b == stem)
}

/// THE DOOR ROW'S OWN MODEL WALK (the autopush-656 round, the
/// autopush-658 hinge word — the shared resolution the face arm and
/// the DOOR_MODEL_BY_ROW emit both read): the blockstate {base}.json's
/// facing=north row the per-hinge key resolves (the LEFT rows the
/// ctor's registerDefaultState pin the hinge-left default speaks, the
/// RIGHT rows the getHinge census's own verdict the `hinge=right` key
/// answers), the model ref the `minecraft:` strip keeps. The WAXED
/// LADDER the UNWAXED models the jar's own rows serve
/// (waxed_copper_door.json the block/copper_door_bottom_left model the
/// real chain answers — the per-oxidization textures the waxed kinds
/// inherit, the waxed model files NO jar keeps).
fn door_row_model(
    archive: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>,
    base: &str,
    upper: bool,
    open: bool,
    right: bool,
) -> Result<String> {
    // THE HINGE WORD THE KEY SPEAKS (the autopush-658 round — the
    // LEFT rows the first lane's default pin banks, the RIGHT rows
    // the getHinge census's own verdict the blockstate resolves)
    let key = format!(
        "facing=north,half={},hinge={},open={open}",
        if upper { "upper" } else { "lower" },
        if right { "right" } else { "left" }
    );
    let bs_bytes = read_jar(
        archive,
        &format!("assets/minecraft/blockstates/{base}.json"),
    )?;
    let bs: Value = serde_json::from_slice(&bs_bytes)
        .with_context(|| format!("blockstate {base}: json parse"))?;
    let model_ref = bs
        .get("variants")
        .and_then(|v| v.get(&key))
        .and_then(|v| v.get("model"))
        .and_then(|m| m.as_str())
        .with_context(|| format!("blockstate {base}: no {key} model"))?
        .trim_start_matches("minecraft:")
        .to_string();
    Ok(model_ref)
}

/// THE TRAPDOOR FAMILY'S OWN GATE (the autopush-669 round): the
/// palette rows the wooden rungs wear — the `{wood}_trapdoor`
/// base the row arm's blockstate read keys (the `_open` suffix the
/// strip walk peels — the both-suffix rows the open kinds answer).
/// The eleven rungs the oak round's own walk generalizes, the probe
/// scripts/jar_work/trapdoor_woods_669.txt the 16-row maps banks.
/// The metal rungs the autopush-670 round the tail wears (the
/// copper ladder + the iron pair the same walk serves — the waxed
/// bases the blockstates' own json the UNWAXED models answer, the
/// SAME sprite the unwaxed tiles keep — the probe
/// scripts/jar_work/iron_copper_trapdoors_670.txt banks).
const TRAPDOOR_FAMILY_BASES: [&str; 21] = [
    "oak_trapdoor", "spruce_trapdoor", "birch_trapdoor",
    "jungle_trapdoor", "acacia_trapdoor", "dark_oak_trapdoor",
    "mangrove_trapdoor", "cherry_trapdoor", "pale_oak_trapdoor",
    "bamboo_trapdoor", "crimson_trapdoor", "warped_trapdoor",
    // THE METAL RUNGS' OWN BASES (the copper ladder the oxidation
    // order wears, the waxed rows the OWN names the blockstates'
    // json keys — the json the unwaxed models the alias answers),
    // the iron pair the tail keeps
    "copper_trapdoor", "exposed_copper_trapdoor",
    "weathered_copper_trapdoor", "oxidized_copper_trapdoor",
    "waxed_copper_trapdoor", "waxed_exposed_copper_trapdoor",
    "waxed_weathered_copper_trapdoor", "waxed_oxidized_copper_trapdoor",
    "iron_trapdoor",
];

/// The palette row's `{wood}_trapdoor` base the family gate answers
/// (the `_open` suffix the strip walk peels — the non-family rows the
/// None the generic arms serve).
fn trapdoor_family_base(block: &str) -> Option<&'static str> {
    let stem = block.strip_suffix("_open").unwrap_or(block);
    TRAPDOOR_FAMILY_BASES
        .iter()
        .copied()
        .find(|b| *b == stem)
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
                } else if let Some(o) = v.as_object() {
                    // THE 26.2 OBJECT FORM (the glass family's own refs —
                    // the closed {sprite, force_translucent} shape the
                    // modelgen's TEXTURE_OBJECT_KEYS validator banks): the
                    // sprite field the resolved name the plain strings the
                    // same map keeps (the force_translucent flag the RENDER
                    // LAYER's own signal — the mesher's per-kind lanes the
                    // verdicts own, the tile walk the name alone reads).
                    if let Some(sp) = o.get("sprite").and_then(|s| s.as_str()) {
                        textures.insert(k.clone(), sp.to_string());
                    }
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

/// THE INTERPOLATE LANE'S OWN MIX (the autopush-527 round): the two
/// frame pixels the sub-frame alpha blends — the alpha law the vanilla
/// sprite animator speaks (javap SpriteContents$AnimationState.
/// drawToAtlas pc 39-76 over the 26.2 jar): `(float) subFrame / (float)
/// frameTime` the fdiv serves, the `ldc 1000.0f` fmul + the f2i
/// truncation the thousandths quantization banks — alpha_eff =
/// (int)(alpha * 1000.0f) / 1000 (for the lit-log's frametime 20 the
/// quantization the NO-OP law keeps — the f32 grid the k/20 values the
/// exact k*50 thousandths answer, the empirical walk the 3/20 -> 150
/// and the 19/20 -> 950 truncations speak), the next frame the
/// (frame + 1) % count modulo the wrap law speaks (pc 88-115 — the
/// LAST frame's span the FIRST frame blends toward), the channel mix
/// the ANIMATE_SPRITE_INTERPOLATE pipeline's float lerp serves, the
/// unorm8 round-to-nearest the u8 bank keeps. The four channels the
/// same law rides (the RGBA bytes the strip frames bank).
fn blend_frames(cur: &[u8], next: &[u8], sub: usize, time: u64) -> Vec<u8> {
    let a = (sub as f32 / time as f32 * 1000.0f32) as i32;
    let alpha = a as f64 / 1000.0;
    cur.iter()
        .zip(next.iter())
        .map(|(&c, &n)| (c as f64 + (n as f64 - c as f64) * alpha).round() as u8)
        .collect()
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
        if is_chest_family(block) {
            // THE CHEST FAMILY'S OWN PLACEHOLDER (the dev-64 round):
            // the BE-rendered family the blockstate's dummy model
            // never carries (chest.json's "" variant -> the bare
            // particle model — the six-face walk the chain demands
            // would bail), the composite-tile arm below the entity
            // sheets answers. The placeholder the composite arm's
            // face_tiles overwrite keeps (the oak planks the particle
            // row the dummy model carries).
            faces.push(resolve_flat_stem("oak_planks", false));
            continue;
        }
        if is_skull_family(block) {
            // THE MOB HEADS' OWN PLACEHOLDER (the dev-80 round): the
            // BE-rendered family the blockstate's bare model never
            // resolves (all seven blockstates' "" variant -> the ONE
            // block/skull model — the PARTICLE row alone the model
            // carries, block/soul_sand the jar banks — the six-face
            // walk the chain demands would bail). The placeholder the
            // particle row the dummy model carries (the chest arm's
            // own law the family shares — resolve_flat_stem the
            // EXISTING soul_sand tile re-wears, the ZERO-or-append
            // tail the tile count keeps). The face the BE lane
            // renders (the entity sheets the SkullBlockEntityRenderer
            // serves) the mini-cube mesher lane the future round
            // keeps — the placeholder the six slots the kind
            // vocabulary's own row banks.
            faces.push(resolve_flat_stem("soul_sand", false));
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
        if block == "bamboo" {
            // THE STALK'S OWN PALETTE ARM (the autopush-608 round, the
            // official 26.2 jar): the blockstate is a MULTIPART whose
            // age arms apply FOUR overlapping stalk models each (the
            // bamboo1..4_age0 / bamboo1..4_age1 fours) and the leaves
            // arms the crossed-plane pairs — resolve_block's full-cube
            // six-face requirement cannot apply, and the generic
            // multipart walk the fire arm serves picks a SINGLE
            // canonical model. The FIRST age0 apply (block/
            // bamboo1_age0 — the declaration order's own head) is the
            // canonical stalk model: its merged texture map's `all`
            // ref (block/bamboo_stalk — the 16x16 FULLY-OPAQUE master
            // the six faces bank) is the palette row. The leaves
            // masters (block/bamboo_small_leaves, block/
            // bamboo_large_leaves) the mesher's per-quad texture
            // verdict reads through the named consts the appended
            // tiles carry — the palette row the stalk tile alone
            // needs.
            let bs_bytes = read_jar(
                &mut archive,
                "assets/minecraft/blockstates/bamboo.json",
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| "blockstate bamboo: json parse")?;
            let parts = bs
                .get("multipart")
                .and_then(|v| v.as_array())
                .with_context(|| "blockstate bamboo: no multipart")?;
            let mut model_ref: Option<String> = None;
            for part in parts {
                let when = part.get("when");
                let age0 = when
                    .and_then(|w| w.get("age"))
                    .and_then(|v| v.as_str())
                    .map(|v| v == "0")
                    .unwrap_or(false);
                if !age0 {
                    continue;
                }
                let applies = part.get("apply").and_then(|v| v.as_array());
                let Some(applies) = applies else { continue };
                if let Some(first) = applies
                    .first()
                    .and_then(|a| a.get("model"))
                    .and_then(|m| m.as_str())
                {
                    model_ref =
                        Some(first.trim_start_matches("minecraft:").to_string());
                    break;
                }
            }
            let model_ref = model_ref
                .with_context(|| "blockstate bamboo: no age0 apply")?;
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block bamboo: model chain {model_ref}"))?;
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
        if block == "end_portal" {
            // THE PORTAL'S OWN FILLER ROW (the dev-56 round — the
            // beyond-table seat the palette's own row FINALLY banks:
            // the END_PORTAL kind the 510 id the walls' contiguous
            // run the seat the tail requires — the row the inert seat
            // keeps [the mesher's own starry-plane lane the portal
            // renders, the FACE_TILES row the lane never reads]). The
            // blockstate's ONE variant the block/end_portal model
            // resolves (the jar's own chain), the merged texture
            // map's `particle` ref (block/obsidian — the model's OWN
            // texture map the jar banks: the portal the particle
            // master carries, NO face refs the element-less model
            // wears), the tile the six slots bank, tinted = false.
            let bs_bytes = read_jar(
                &mut archive,
                "assets/minecraft/blockstates/end_portal.json",
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| "blockstate end_portal: json parse")?;
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(""))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .map(|m| m.trim_start_matches("minecraft:").to_string())
                .with_context(|| "blockstate end_portal: no model")?;
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block end_portal: model chain {model_ref}"))?;
            let tex = tex_map
                .get("particle")
                .with_context(|| format!("model {model_ref}: no particle texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block.ends_with("_fence") {
            // THE FENCE FAMILY'S OWN PALETTE ARM (the dev-55 round,
            // the official 26.2 jar): the blockstate is a MULTIPART —
            // the UNCONDITIONAL post apply (block/{wood}_fence_post —
            // the declaration order's own head) + the four side arms
            // the north/east/south/west when-rows — resolve_block's
            // full-cube six-face requirement cannot apply, and the
            // generic multipart walk the fire/bamboo arms serve picks
            // a SINGLE canonical model. The FIRST apply (the post —
            // the fence's every connection state wears it) is the
            // canonical model: its merged texture map's `texture` ref
            // (block/{wood}_planks for the twelve wooden rows,
            // block/nether_bricks the nether seam's master) is the
            // palette row — the ONE master EVERY element face wears
            // (the post's six + the side bars' faces the same ref
            // resolve), the FACE_TILES row the ONE tile the six slots
            // bank (the gate arm's law the family keeps), tinted =
            // false (the planks masters carry NO tintindex). The
            // atlas tile count the ZERO new tiles keeps — the planks
            // and nether_bricks masters the palette already seats.
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{block}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {block}: json parse"))?;
            let parts = bs
                .get("multipart")
                .and_then(|v| v.as_array())
                .with_context(|| format!("blockstate {block}: no multipart"))?;
            // the FIRST apply (the unconditional post — the fence's
            // every connection state wears it)
            let model_ref = parts
                .first()
                .and_then(|p| p.get("apply"))
                .and_then(|a| if a.is_array() { a.get(0) } else { Some(a) })
                .and_then(|a| a.get("model"))
                .and_then(|m| m.as_str())
                .map(|m| m.trim_start_matches("minecraft:").to_string())
                .with_context(|| format!("blockstate {block}: no post apply"))?;
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("texture")
                .with_context(|| format!("model {model_ref}: no texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block.ends_with("_wall") {
            // THE WALL FAMILY'S OWN PALETTE ARM (the dev-56 round,
            // the official 26.2 jar): the blockstate is a MULTIPART —
            // the up=true CONDITIONAL post apply (block/{base}_wall_post
            // — the declaration order's own head, the up bit the
            // vanilla default true wears) + the four side arms the
            // north/east/south/west LOW/TALL when-rows — resolve_block's
            // full-cube six-face requirement cannot apply. The FIRST
            // apply (the post — the wall's up default every lone row
            // wears) is the canonical model: its merged texture map's
            // `wall` ref (block/cobblestone the cobblestone row's
            // master, block/brick the brick row's — the ONE #texture
            // ref the template_wall_post parent chain resolves) is the
            // palette row — the ONE master EVERY element face wears
            // (the post's six + the side arms' faces the same ref
            // resolve), the FACE_TILES row the ONE tile the six slots
            // bank (the fence arm's law the family keeps), tinted =
            // false (the stone masters carry NO tintindex). The atlas
            // tile count the ZERO-or-append tail keeps — the
            // cobblestone/brick/sandstone/... masters the palette
            // already seats (the cinnabar/sulfur/resin/tuff rows the
            // NEW masters the append-only tail banks — the note_block
            // 748->749 precedent the same law wears).
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{block}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {block}: json parse"))?;
            let parts = bs
                .get("multipart")
                .and_then(|v| v.as_array())
                .with_context(|| format!("blockstate {block}: no multipart"))?;
            // the FIRST apply (the up=true post — the wall's default
            // state every lone row wears)
            let model_ref = parts
                .first()
                .and_then(|p| p.get("apply"))
                .and_then(|a| if a.is_array() { a.get(0) } else { Some(a) })
                .and_then(|a| a.get("model"))
                .and_then(|m| m.as_str())
                .map(|m| m.trim_start_matches("minecraft:").to_string())
                .with_context(|| format!("blockstate {block}: no post apply"))?;
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("wall")
                .with_context(|| format!("model {model_ref}: no wall texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block.ends_with("_chain") {
            // THE CHAIN FAMILY'S OWN PALETTE ARM (the dev-60 round, the
            // official 26.2 jar): the blockstate is a VARIANTS map —
            // the axis=y entry the IDENTITY model wears (block/{base}
            // the x=0 y=0 row keeps), the axis=z the x=90 turn banks,
            // the axis=x the x=90 + y=90 pair (the probe's blockstate
            // walk speaks), the waxed twins the BASE models re-wear —
            // resolve_block's full-cube six-face requirement cannot
            // apply (the two zero-width plane elements the full-cube
            // finder never banks). The axis=y entry's model (the
            // vertical chain every unstamped cell wears) is the
            // canonical: its merged texture map's `all` ref (block/
            // iron_chain the metal link, block/copper_chain the bare
            // rung, block/{prefix}copper_chain the dyed rungs wear —
            // the ONE ref the template's #texture parent chain
            // resolves) is the palette row — the ONE master the six
            // slots bank (the fence arm's law the family keeps),
            // tinted = false (the pre-colored masters carry NO
            // tintindex). The atlas tile count the ZERO-or-append tail
            // keeps — the FIVE masters (iron + 4 bare copper rungs)
            // the NEW tiles the append-only tail banks (812 -> 817),
            // the waxed twins the base rows re-wear.
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{block}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {block}: json parse"))?;
            let variants = bs
                .get("variants")
                .and_then(|v| v.as_object())
                .with_context(|| format!("blockstate {block}: no variants"))?;
            let y_entry = variants
                .get("axis=y")
                .with_context(|| format!("blockstate {block}: no axis=y variant"))?;
            let model_ref = (if y_entry.is_array() {
                y_entry.get(0)
            } else {
                Some(y_entry)
            })
            .and_then(|a| a.get("model"))
            .and_then(|m| m.as_str())
            .map(|m| m.trim_start_matches("minecraft:").to_string())
            .with_context(|| format!("blockstate {block}: no axis=y model"))?;
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("all")
                .or_else(|| tex_map.get("texture"))
                .with_context(|| format!("model {model_ref}: no all/texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block.ends_with("lantern") && block != "jack_o_lantern" {
            // THE LANTERN FAMILY'S OWN PALETTE ARM (the dev-61 round,
            // the official 26.2 jar): the blockstate is a VARIANTS map
            // — the hanging=false entry the STANDING model wears
            // (block/{name} the x=0 y=0 row keeps), the hanging=true
            // the block/{name}_hanging banks, the waxed twins the BASE
            // models re-wear — resolve_block's full-cube six-face
            // requirement cannot apply (the body + top + hook-plane
            // elements the full-cube finder never banks; the plain
            // jack_o_lantern the pumpkin family's full-cube row the
            // exclusion keeps). The hanging=false entry's model (the
            // standing lantern every unstamped cell wears) is the
            // canonical: its merged texture map's `lantern` ref
            // (block/lantern the iron seam, block/soul_lantern the
            // nether flame, block/{prefix}copper_lantern the bare
            // rungs wear — the ONE ref the template parent resolves)
            // is the palette row — the ONE master the six slots bank
            // (the fence arm's law the family keeps), tinted = false
            // (the pre-colored masters carry NO tintindex). The atlas
            // tile count the ZERO-or-append tail keeps — the SIX
            // masters (lantern + soul_lantern + 4 bare copper rungs)
            // the NEW tiles the append-only tail banks (817 -> 823),
            // the waxed twins the base rows re-wear. The 16x48 strip
            // the first-frame read the top 16x16 sprite the tile
            // banks — the model UVs the v<=15 band the frame never
            // leaves (the {0, 48} walk the alpha histogram the probe
            // banks), the {0: 450, 255: 318} the binary cutout the
            // OPAQUE pass rides.
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{block}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {block}: json parse"))?;
            let variants = bs
                .get("variants")
                .and_then(|v| v.as_object())
                .with_context(|| format!("blockstate {block}: no variants"))?;
            let stand_entry = variants
                .get("hanging=false")
                .with_context(|| format!("blockstate {block}: no hanging=false variant"))?;
            let model_ref = (if stand_entry.is_array() {
                stand_entry.get(0)
            } else {
                Some(stand_entry)
            })
            .and_then(|a| a.get("model"))
            .and_then(|m| m.as_str())
            .map(|m| m.trim_start_matches("minecraft:").to_string())
            .with_context(|| format!("blockstate {block}: no standing model"))?;
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("lantern")
                .with_context(|| format!("model {model_ref}: no lantern ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block.ends_with("_slab") {
            // THE SLAB FAMILY'S OWN PALETTE ARM (the dev-63 round,
            // the official 26.2 jar): the blockstate is a VARIANTS
            // map — the "type=bottom" entry the BASE model wears
            // (block/{name} the parent-texture refs keep, the waxed
            // twins the BASE copper models re-wear, the petrified
            // twin the block/oak_planks parent answers) —
            // resolve_block's full-cube six-face requirement cannot
            // apply (the slab band element the half-height geometry
            // never banks). The canonical entry's model is the
            // palette row: its merged texture map's `side` ref
            // (block/oak_planks the wooden rungs, block/stone the
            // stone walk, block/cut_copper the copper ladder, the
            // sandstone walk the side arm, block/smooth_stone_slab_
            // side the ONE own png the special master keeps) is the
            // palette row — the ONE master the six slots bank (the
            // stairs arm's law the family keeps; the `bottom`/`top`
            // split refs the mesher's per-quad verdict the exact
            // tiles reads), tinted = false (the pre-colored masters
            // carry NO tintindex). The atlas tile count the
            // ZERO-or-append tail keeps — the slab's own pngs the
            // jar banks ONE (smooth_stone_slab_side), the PARENT
            // masters the existing tiles the rows re-wear, the kinds
            // the palette tail banks (669 -> 737).
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{block}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {block}: json parse"))?;
            let variants = bs
                .get("variants")
                .and_then(|v| v.as_object())
                .with_context(|| format!("blockstate {block}: no variants"))?;
            let canon_entry = variants
                .get("type=bottom")
                .with_context(|| format!("blockstate {block}: no type=bottom variant"))?;
            let model_ref = (if canon_entry.is_array() {
                canon_entry.get(0)
            } else {
                Some(canon_entry)
            })
            .and_then(|a| a.get("model"))
            .and_then(|m| m.as_str())
            .map(|m| m.trim_start_matches("minecraft:").to_string())
            .with_context(|| format!("blockstate {block}: no canonical model"))?;
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("side")
                .or_else(|| tex_map.get("top"))
                .or_else(|| tex_map.get("bottom"))
                .with_context(|| format!("model {model_ref}: no side/top/bottom ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block.ends_with("_stairs") {
            // THE STAIRS FAMILY'S OWN PALETTE ARM (the dev-62 round,
            // the official 26.2 jar): the blockstate is a VARIANTS map
            // — the "facing=east,half=bottom,shape=straight" entry the
            // BASE model wears (block/{name} the x=0 y=0 row keeps —
            // the base model the EAST orientation the template parent
            // banks, the step the x 8..16 half speaks; the waxed twins
            // the BASE copper models re-wear) — resolve_block's
            // full-cube six-face requirement cannot apply (the slab +
            // step elements the full-cube finder never banks). The
            // canonical entry's model is the palette row: its merged
            // texture map's `side` ref (block/oak_planks the wooden
            // rungs, block/stone the stone walk, block/cut_copper the
            // copper ladder, block/sandstone the sandstone stair's
            // side — the ONE ref the template parent resolves; the
            // `top`/`bottom` refs the sandstone walk the split rows
            // wear, the mesher's per-quad verdict the exact tiles
            // reads) is the palette row — the ONE master the six slots
            // bank (the lantern arm's law the family keeps), tinted =
            // false (the pre-colored masters carry NO tintindex). The
            // atlas tile count the ZERO-or-append tail keeps — the
            // stairs' own pngs the jar never banks, the PARENT masters
            // the existing tiles the rows re-wear (823 -> 823 the tile
            // count never moves), the kinds the palette tail banks
            // (605 -> 669).
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{block}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {block}: json parse"))?;
            let variants = bs
                .get("variants")
                .and_then(|v| v.as_object())
                .with_context(|| format!("blockstate {block}: no variants"))?;
            let canon_entry = variants
                .get("facing=east,half=bottom,shape=straight")
                .with_context(|| {
                    format!("blockstate {block}: no canonical straight variant")
                })?;
            let model_ref = (if canon_entry.is_array() {
                canon_entry.get(0)
            } else {
                Some(canon_entry)
            })
            .and_then(|a| a.get("model"))
            .and_then(|m| m.as_str())
            .map(|m| m.trim_start_matches("minecraft:").to_string())
            .with_context(|| format!("blockstate {block}: no canonical model"))?;
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("side")
                .or_else(|| tex_map.get("top"))
                .or_else(|| tex_map.get("bottom"))
                .with_context(|| format!("model {model_ref}: no side/top/bottom ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block.ends_with("glass_pane") || block == "iron_bars" || block.ends_with("copper_bars") {
            // THE PANE FAMILY'S OWN PALETTE ARM (the dev-58 round, the
            // official 26.2 jar): the blockstate is a MULTIPART — the
            // UNCONDITIONAL post apply (block/{name}_post for the
            // panes, block/iron_bars_post_ends for the bars — the
            // declaration order's own head) + the side/noside or
            // cap/side arms the when-rows — resolve_block's full-cube
            // six-face requirement cannot apply (the pane post element
            // carries the down/up cap pair ONLY, the full-cube finder
            // the two-face element never banks). The FIRST apply (the
            // post — the pane's every connection state wears it) is
            // the canonical model: its merged texture map's `edge` ref
            // (block/glass_pane_top for the plain row, block/{color}_
            // stained_glass_pane_top the sixteen dyed rungs wear,
            // block/iron_bars the metal seam keeps — the ONE ref the
            // post AND the side/cap models the template parents bank)
            // is the palette row — the EDGE master the six slots bank
            // (the #pane side refs the GLASS FAMILY's own masters
            // re-wear — the sibling kinds the id-17 rows the mesher's
            // per-quad verdict reads — the edge tiles the on-demand
            // collection the pane arm's own stem banks: 790 -> 808 the
            // seventeen tops + the iron_bars master the append law
            // keeps), tinted = false (the pre-colored masters carry
            // NO tintindex). The append-only law verify_atlas_full.py
            // banks — every pre-existing row the same pixels + tints +
            // names renders.
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{block}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {block}: json parse"))?;
            let parts = bs
                .get("multipart")
                .and_then(|v| v.as_array())
                .with_context(|| format!("blockstate {block}: no multipart"))?;
            // the FIRST apply (the unconditional post/post_ends — the
            // pane's every connection state wears it)
            let model_ref = parts
                .first()
                .and_then(|p| p.get("apply"))
                .and_then(|a| if a.is_array() { a.get(0) } else { Some(a) })
                .and_then(|a| a.get("model"))
                .and_then(|m| m.as_str())
                .map(|m| m.trim_start_matches("minecraft:").to_string())
                .with_context(|| format!("blockstate {block}: no post apply"))?;
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("edge")
                .with_context(|| format!("model {model_ref}: no edge texture ref"))?;
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
        if block == "cake" || block.starts_with("cake_slice") {
            // THE CAKE FAMILY'S OWN ROWS (the autopush-524 round, the
            // official 26.2 jar): the blockstate is a plain VARIANTS
            // map over bites — the FULL cake the "bites=0" row
            // resolves (block/cake), the bitten stages the "bites=N"
            // rows read (block/cake_sliceN — the palette name's own
            // trailing digits the row key keeps, the wheat_stageN arm
            // the same walk generalizes), the per-slice blockstates
            // the vanilla itself never carries (no cake_slice1.json —
            // the resolve_block first-variant walk the slices'
            // explicit-row arm alone answers). Each model is a SINGLE
            // box element [1+2N, 0, 1]->[15, 8, 15] (the lambda$
            // static$0 shape the x-start walk mirrors): #bottom/
            // #top/#side the cake masters name, the west face #inside
            // the bitten cut speaks (block/cake_inner — the bank's
            // own fourth tile the appended walk serves). The flat row
            // the `side` ref banks (block/cake_side — the template's
            // own particle the cake body's dominant texture speaks;
            // the multi-texture box's other faces the MESHER's own
            // per-quad verdict reads, the candle-cake lane the same
            // law keeps). tinted = false (the faces carry NO
            // tintindex — the pre-colored masters render as
            // authored).
            let bites = if block == "cake" {
                0
            } else {
                block
                    .trim_start_matches("cake_slice")
                    .parse::<u8>()
                    .with_context(|| format!("block {block}: no bite stage"))?
            };
            let row_key = format!("bites={bites}");
            let bs_bytes = read_jar(&mut archive, "assets/minecraft/blockstates/cake.json")?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| "blockstate cake: json parse")?;
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(&row_key))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate cake: no {row_key} model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
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
        if let Some(wood_base) = door_family_base(block) {
            // THE DOOR FAMILY'S OWN ROWS (the autopush-655 round, the
            // dev-43 wood generalization, the autopush-658 hinge-right
            // tail — the official 26.2 jar): the blockstate {wood}_door
            // .json is a plain 32-variant map — the kind-only palette
            // the per-hinge rows resolve (javap DoorBlock's ctor
            // registerDefaultState walk pc 53-59 the HINGE=LEFT pin
            // speaks, the RIGHT rows the getHinge census's own verdict
            // the tail rows wear), the closed kinds the open=false
            // rows, the open kinds the open=true rows, the half
            // property the upper-word row keys. The _right suffix the
            // flags' own reads peel FIRST (the tail rows the stem walk
            // serves), the hinge word the key speaks. The EXPLICIT row
            // the walk reads (the candle_lit arm's own law — the
            // facing=north rows the orientable vocabulary's default
            // the variant byte's 0 answers), the model chain's
            // resolved ref the flat stem keeps: the lower's faces all
            // #bottom (block/{wood}_door_bottom — the RIGHT rows the
            // SAME masters the hinge-independent chain resolves), the
            // upper's all #top (block/{wood}_door_top), the per-wood
            // masters the mesher's per-quad verdict the named consts
            // read, tinted = false (the pre-colored masters render as
            // authored). The family walk the twelve woods share (the
            // DOOR_FAMILY_BASES list the row arms gate — the copper
            // ladder and the iron door the 656 lane's rows wear).
            let right = block.ends_with("_right");
            let stem = block.strip_suffix("_right").unwrap_or(block);
            let upper = stem.ends_with("_upper") || stem.ends_with("_upper_open");
            let open = stem.ends_with("_open");
            let model_ref = door_row_model(&mut archive, wood_base, upper, open, right)?;
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {wood_base}: model chain {model_ref}"))?;
            let tex = tex_map
                .get(if upper { "top" } else { "bottom" })
                .with_context(|| format!("model {model_ref}: no door texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
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
        if block == "cobweb" {
            // THE WEB'S OWN CROSS ROW (the dev-22 round, the official
            // 26.2 jar): the blockstate is a single variant map ("") —
            // the model ref -> block/cobweb whose PARENT is block/cross
            // (the flowers' own two diagonal plane elements the 45°
            // rotY + rescale fold bakes, `shade: false` faces under
            // model-level `ambientocclusion: false`), so resolve_block's
            // full-cube six-face requirement cannot apply. The texture
            // stem resolves through the REAL chain (the blockstate
            // variant's model ref -> the merged texture map's `cross`
            // ref — cobweb -> block/cobweb), tinted = false (the faces
            // carry NO tintindex — the pre-colored master renders as
            // authored; the audited BlockColors walk the web never
            // covers, the WEAVING effect the entity walk alone reads).
            let bs_bytes = read_jar(
                &mut archive,
                "assets/minecraft/blockstates/cobweb.json",
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| "blockstate cobweb: json parse")?;
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(""))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| "blockstate cobweb: no empty-variant model")?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block cobweb: model chain {model_ref}"))?;
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
        if block == "bamboo_sapling" {
            // THE SAPLING'S OWN CROSS ROW (the autopush-608 round, the
            // official 26.2 jar): the blockstate is a single variant
            // map ("") — the model ref -> block/bamboo_sapling whose
            // PARENT is block/tinted_cross (the flowers' own two
            // diagonal plane elements the 45° rotY + rescale fold
            // bakes, `shade: false` faces under model-level
            // `ambientocclusion: false`), so resolve_block's full-cube
            // six-face requirement cannot apply. The texture stem
            // resolves through the REAL chain (the blockstate
            // variant's model ref -> the merged texture map's `cross`
            // ref — bamboo_sapling -> block/bamboo_stage0), tinted =
            // FALSE despite the tinted_cross parent's tintindex-0
            // faces — the PRE-COLORED master renders as authored (the
            // audited BlockColors register walk covers
            // FERN/SHORT_GRASS/POTTED_FERN/BUSH + the TALL_GRASS/
            // LARGE_FERN pair, the bamboo family the absence keeps —
            // the tintindex the resolver never serves rides white).
            let bs_bytes = read_jar(
                &mut archive,
                "assets/minecraft/blockstates/bamboo_sapling.json",
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| "blockstate bamboo_sapling: json parse")?;
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(""))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| "blockstate bamboo_sapling: no empty-variant model")?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block bamboo_sapling: model chain {model_ref}"))?;
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
        if block == "lever" || block == "lever_on" {
            // THE LEVER FAMILY'S OWN ROWS (the dev-45 round): the
            // blockstate's own FLOOR rows the two kinds wear — the
            // "face=floor,facing=north,powered={p}" row the jar's own
            // REVERSED authoring resolves (powered=false the
            // "block/lever_on" model, powered=true the "block/lever"
            // model — the verbatim rows the resolution speaks), the
            // merged texture map's `#lever` ref block/lever the row's
            // tile banks (all six slots the ONE lever texture — the
            // mesher's lever lane the per-quad verdict reads, the
            // cobblestone base the sidecar tile answers), tinted =
            // false (the faces carry NO tintindex — the pre-colored
            // masters render as authored).
            let powered = block == "lever_on";
            let bs_bytes = read_jar(
                &mut archive,
                "assets/minecraft/blockstates/lever.json",
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| "blockstate lever: json parse")?;
            let row_key = format!("face=floor,facing=north,powered={powered}");
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(&row_key))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate lever: no {row_key} model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block lever: model chain {model_ref}"))?;
            let tex = tex_map
                .get("lever")
                .with_context(|| format!("model {model_ref}: no lever texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block.ends_with("_button") || block.ends_with("_button_pressed") {
            // THE BUTTON FAMILY'S OWN ROWS (the autopush-660 round +
            // the dev-47 oak rung + the autopush-662 full ladder): the
            // blockstate's own FLOOR rows the kinds wear — the
            // "face=floor,facing=north,powered={p}" row the jar's
            // DIRECT authoring resolves (EVERY rung the block's own
            // name the blockstate path answers — the stone pair the
            // block/stone_button models, the oak pair the
            // block/oak_button models, the twelve ladder rungs the
            // block/{wood}_button models — the verbatim rows the
            // resolution speaks, the lever's REVERSED authoring the
            // buttons never wear), the merged texture map's `#texture`
            // ref the row's tile banks (the stone the block/stone
            // master, the wooden rungs the block/{wood}_planks
            // masters, the polished rung the block/polished_blackstone
            // master — the stone master the palette already banks, the
            // twelve planks/blackstone masters the first-encounter walk
            // gains (the lever row's own tail seat the twelve tiles
            // bank — the SPECIAL walks' tiles the farmland/bamboo/
            // destroy/campfire/cake/lava rows the +12 shift the
            // sidecar constants re-bank, the NAME→PIXELS mapping the
            // byte-identical law keeps, the rendered scenes the
            // buttonless frames the same pixels wear); the mesher's
            // button lane the per-quad verdict reads), tinted = false
            // (the faces carry NO tintindex — the pre-colored masters
            // render as authored).
            let powered = block.ends_with("_pressed");
            // the ONE blockstate file BOTH states the same variants map
            // serves (the stone_button.json the _pressed row the own
            // file never wears — the row key the powered={p} arm reads)
            let bs_name = block.strip_suffix("_pressed").unwrap_or(block);
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{bs_name}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {bs_name}: json parse"))?;
            let row_key = format!("face=floor,facing=north,powered={powered}");
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(&row_key))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate {block}: no {row_key} model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("texture")
                .with_context(|| format!("model {model_ref}: no texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        if block == "redstone_wire"
            || block == "redstone_wire_on"
            || block.starts_with("redstone_wire_p")
        {
            // THE REDSTONE DUST'S OWN ROWS (the autopush-665 round): the
            // multipart's own parts the REAL chain the generator's own
            // law keeps (blockstates -> multipart apply -> model -> the
            // #line sprite ref, never a hardcoded name): the DOT stem
            // the part[0]'s apply model (redstone_dust_dot) its own
            // `textures.line` sprite ref banks, the N-S master the
            // part[1]'s apply (redstone_dust_side0) its `textures.line`
            // sprite ref names (block/redstone_dust_line0), the E-W
            // master the part[4]'s apply (redstone_dust_side1) the same
            // walk serves (block/redstone_dust_line1). The sprite ref
            // the STRING the older models wear OR the `{sprite: ...}`
            // object the 26.2 force_translucent rows speak — both the
            // read answers. The faces the THREE stems the six slots
            // bank (slots 0..2 the dot/N-S/E-W masters the mesher's
            // wire path reads by slot, slots 3..5 the dot master the
            // never-rendered faces keep), tinted = false (the models'
            // tintindex the power ramp the append pass bakes per state
            // — the OFF row the power-0 color the masters' own bytes
            // wear, the ON row the power-15 color the _on tiles bank).
            // NO palette-row tint the tint_for_texture walk answers —
            // the grayscale masters ride the bake the append pass's
            // own wire rows carry.
            let wire_sprite = |archive: &mut ZipArchive<std::io::Cursor<&[u8]>>,
                               model_ref: &str|
             -> Result<String> {
                let bytes = read_jar(
                    archive,
                    &format!("assets/minecraft/models/{model_ref}.json"),
                )?;
                let json: Value = serde_json::from_slice(&bytes)
                    .with_context(|| format!("model {model_ref}: json parse"))?;
                let tex = json
                    .get("textures")
                    .with_context(|| format!("model {model_ref}: no textures"))?;
                let line = tex
                    .get("line")
                    .with_context(|| format!("model {model_ref}: no #line ref"))?;
                let sprite = line
                    .as_str()
                    .map(|s| s.to_string())
                    .or_else(|| {
                        line.get("sprite")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                    })
                    .with_context(|| format!("model {model_ref}: #line sprite absent"))?;
                Ok(sprite
                    .trim_start_matches("minecraft:")
                    .trim_start_matches("block/")
                    .to_string())
            };
            let bs_bytes = read_jar(
                &mut archive,
                "assets/minecraft/blockstates/redstone_wire.json",
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| "blockstate redstone_wire: json parse")?;
            let parts = bs
                .get("multipart")
                .and_then(|v| v.as_array())
                .with_context(|| "blockstate redstone_wire: no multipart")?;
            let part_model = |part: &Value| -> Result<String> {
                let apply = part
                    .get("apply")
                    .with_context(|| "multipart part: no apply")?;
                let row = if let Some(first) = apply.as_array().and_then(|a| a.first()) {
                    first.clone()
                } else {
                    apply.clone()
                };
                row.get("model")
                    .and_then(|m| m.as_str())
                    .map(|s| s.trim_start_matches("minecraft:").to_string())
                    .with_context(|| "multipart part: no model ref")
            };
            let dot_stem = wire_sprite(&mut archive, &part_model(&parts[0])?)?;
            let ns_stem = wire_sprite(&mut archive, &part_model(&parts[1])?)?;
            let ew_stem = wire_sprite(&mut archive, &part_model(&parts[4])?)?;
            let face = |stem: &str| ResolvedFace {
                texture: format!("block/{stem}"),
                tinted: false,
            };
            faces.push([
                face(&dot_stem),
                face(&ns_stem),
                face(&ew_stem),
                face(&dot_stem),
                face(&dot_stem),
                face(&dot_stem),
            ]);
            continue;
        }
        if block.ends_with("_pressure_plate") || block.ends_with("_pressure_plate_down") {
            // THE PRESSURE PLATES' OWN ROWS (the dev-48 round): the
            // blockstate's own POWERED rows the kinds wear — the
            // "powered={p}" row the jar's DIRECT authoring resolves (the
            // ONE {base}_pressure_plate.json BOTH states the same
            // variants map serves — the strip-suffix walk the _down rows
            // the own file never wears), the merged texture map's
            // `#texture` ref the row's tile banks (the oak rung the
            // block/oak_planks planks master, the stone rung the
            // block/stone master — the palette the tiles already bank,
            // no new tile the regen keeps, the pre-plate rows
            // byte-identical), tinted = false (the faces carry NO
            // tintindex — the pre-colored masters render as authored).
            let powered = block.ends_with("_down");
            let bs_name = block.strip_suffix("_down").unwrap_or(block);
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{bs_name}.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {bs_name}: json parse"))?;
            // THE WEIGHTED PLATES' OWN ROW KEY (the dev-49 round — the
            // blockstates the jar itself banks the weighted files the
            // `power={0..15}` rows wear: power=0 the up model, power=1
            // the down rows' head the whole 1..15 range the same model
            // answers — the POWER integer property the IntegerProperty
            // clinit row speaks, the powered={p} rows the wooden/stone
            // ladder's own files wear)
            let row_key = if bs_name.ends_with("_weighted_pressure_plate") {
                format!("power={}", if powered { 1 } else { 0 })
            } else {
                format!("powered={powered}")
            };
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(&row_key))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate {bs_name}: no {row_key} model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("texture")
                .with_context(|| format!("model {model_ref}: no texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        let gate_open = block.ends_with("_fence_gate_open");
        if gate_open || block.ends_with("_fence_gate") {
            // THE FENCE GATE FAMILY'S OWN ROWS (the dev-53 round, the
            // autopush-681 round the ladder's ELEVEN wooden rungs the
            // same walk every wood serves): the blockstate's own
            // facing=south,in_wall=false,open={p} rows the two kinds
            // each wood wears — the registerDefaultState's
            // IN_WALL=false pin the row key's own seat keeps (the
            // isWall tag verdict the walls the palette never banks —
            // the wall lane's own models the row never resolves), the
            // south row the y=0 authoring the mesher's theta tables
            // fold (the closed/open rows the SAME y values the pair
            // serves), the merged texture map's `#texture` ref the
            // row's tile banks (the template_fence_gate chain the
            // #texture = block/{wood}_planks planks master resolves —
            // the BAMBOO pair the template_custom_fence_gate parents
            // its OWN bamboo_fence_gate master speaks — the palette
            // the tile banks or the planks masters the ten woods
            // re-wear, the one NEW tile the bamboo master seats, the
            // pre-gate rows byte-identical), tinted = false (the
            // faces carry NO tintindex — the pre-colored masters
            // render as authored).
            let wood = block
                .trim_end_matches("_open")
                .strip_suffix("_fence_gate")
                .with_context(|| format!("block {block}: gate stem decode"))?;
            let open = gate_open;
            let bs_bytes = read_jar(
                &mut archive,
                &format!("assets/minecraft/blockstates/{wood}_fence_gate.json"),
            )?;
            let bs: Value = serde_json::from_slice(&bs_bytes)
                .with_context(|| format!("blockstate {wood}_fence_gate: json parse"))?;
            let row_key = format!("facing=south,in_wall=false,open={open}");
            let model_ref = bs
                .get("variants")
                .and_then(|v| v.get(&row_key))
                .and_then(|v| v.get("model"))
                .and_then(|m| m.as_str())
                .with_context(|| format!("blockstate {wood}_fence_gate: no {row_key} model"))?
                .trim_start_matches("minecraft:")
                .to_string();
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: model chain {model_ref}"))?;
            let tex = tex_map
                .get("texture")
                .with_context(|| format!("model {model_ref}: no texture ref"))?;
            let stem = tex
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string();
            faces.push(resolve_flat_stem(&stem, false));
            continue;
        }
        faces.push(resolve_block(&mut archive, block)?);
    }

    // THE DOOR FAMILY'S OWN MODEL ROWS (the autopush-656 round, the
    // autopush-658 hinge-right tail): the per-kind model ref the
    // blockstate's own facing=north row speaks (the LEFT rows the
    // hinge=left key resolves, the RIGHT tail the hinge=right key —
    // the blockstate's own per-hinge rows the real chain serves) —
    // the SECOND pass the same door_row_model walk the face arm
    // shares (the jar's real chain the waxed ladder the UNWAXED
    // models serves), the generated table the mesher's bake reads
    // (the NAME_OF-derived word the waxed kinds the absent models
    // never answer — the row ref the honest resolution keeps).
    let mut door_models: Vec<String> = Vec::with_capacity(RUNTIME_PALETTE.len());
    for block in RUNTIME_PALETTE {
        if let Some(base) = door_family_base(block) {
            let right = block.ends_with("_right");
            let stem = block.strip_suffix("_right").unwrap_or(block);
            let upper = stem.ends_with("_upper") || stem.ends_with("_upper_open");
            let open = stem.ends_with("_open");
            door_models.push(door_row_model(&mut archive, base, upper, open, right)?);
        } else {
            door_models.push(String::new());
        }
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

    // THE CHEST FAMILY'S OWN COMPOSITE TILES (the dev-64 round): the
    // entity sheets the chest renderer wears (entity/chest/*.png the
    // 64x64 sheets the lid/body/lock regions bank) the SINGLE 16x16
    // tiles the chest lane's faces sample — the CROP law (the sheet's
    // own texOffs regions the ASCII probe
    // scripts/jar_work/container_692j.py banks: the lid up
    // (14,0)-(28,14), the lid front (14,14)-(28,19), the lid side
    // (28,14)-(42,19), the body down (28,19)-(42,33), the body front
    // (14,33)-(28,43), the body side (28,33)-(42,43), the lock
    // (1,1)-(3,4) — the latch the front tile's rows carry), the
    // 14x14 face compositions the pad law keeps (the side/front
    // tiles the rows 0..14 band wear — the top-aligned 14 rows +
    // the 15th row the body base the doubles' 15-tall faces sample,
    // the top/bottom tiles the centered 1..15 band keep), the waxed
    // kinds the BASE family's tiles the vanilla law wears (no waxed
    // sheet the jar keeps — the waxed copper chest the bare copper
    // look wears like the waxed lantern's base models). The face
    // slots the renderer order banks (0:+X 1:-X 2:+Y 3:-Y 4:+Z
    // 5:-Z): the sides the four vertical bands keep, the front the
    // NORTH slot wears (the latch the facing face's own tile — the
    // mesher's per-quad verdict the slot map rotates).
    for (ki, block) in RUNTIME_PALETTE.iter().enumerate() {
        let Some((prefix, sheet)) = chest_family_sheet(block) else {
            continue;
        };
        let raw = read_jar(
            &mut archive,
            &format!("assets/minecraft/textures/{sheet}.png"),
        )
        .with_context(|| format!("chest sheet {sheet}.png"))?;
        let (w, h, rgba) = decode_rgba(&raw)?;
        let (front, side, top, bottom) = compose_chest_tiles(&rgba, w, h);
        let stems = [
            (format!("{prefix}_front"), front),
            (format!("{prefix}_side"), side),
            (format!("{prefix}_top"), top),
            (format!("{prefix}_bottom"), bottom),
        ];
        let mut idx = [0u16; 4];
        for (i, (stem, bytes)) in stems.iter().enumerate() {
            idx[i] = match tile_names.iter().position(|n| n == stem) {
                Some(p) => p as u16,
                None => {
                    tile_names.push(stem.clone());
                    tile_bytes.push(bytes.clone());
                    (tile_names.len() - 1) as u16
                }
            };
        }
        // face order: [east, west, up, down, south, north]
        face_tiles[ki] = [idx[1], idx[1], idx[2], idx[3], idx[1], idx[0]];
    }

    // THE BARREL'S OWN OPEN LID TILE (the dev-65 round — the OPEN
    // property's own model the blockstates/barrel.json's open=true
    // arm carries: the block/barrel_open model the cube_bottom_top
    // parent wears, the `top` ref the barrel_top_open master the
    // closed walk never banks — the mesher's barrel lane the OPEN
    // bit's own tile the named const reads). The append law the
    // banked-row dedup keeps (the FACE_TILES row the CLOSED state
    // the generic walk resolves — the FIRST-variant read the
    // block/barrel model the six faces answers).
    {
        let bs_bytes = read_jar(
            &mut archive,
            "assets/minecraft/blockstates/barrel.json",
        )
        .with_context(|| "blockstate barrel: json read")?;
        let bs: Value = serde_json::from_slice(&bs_bytes)
            .with_context(|| "blockstate barrel: json parse")?;
        let model_ref = bs
            .get("variants")
            .and_then(|v| v.get("facing=up,open=true"))
            .and_then(|v| v.get("model"))
            .and_then(|m| m.as_str())
            .with_context(|| "blockstate barrel: no open model")?
            .trim_start_matches("minecraft:")
            .to_string();
        let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
            .with_context(|| format!("barrel open: model chain {model_ref}"))?;
        let tex = tex_map
            .get("top")
            .with_context(|| format!("model {model_ref}: no top ref"))?;
        let stem = tex
            .trim_start_matches("minecraft:")
            .trim_start_matches("block/")
            .to_string();
        if !tile_names.iter().any(|n| *n == stem) {
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
            tile_names.push(stem.clone());
            tile_bytes.push(rgba);
        }
    }

    // THE SLAB FAMILY'S OWN SPLIT REFS (the dev-63 round — the
    // bottom/top models' split texture refs the mesher's per-quad
    // verdict reads: the blackstone_slab walk the blackstone_top
    // top/bottom pair banks, the quartz_slab the quartz_block_top
    // top ref keeps, the sandstone/red_sandstone walks the
    // bottom/top arms the earlier rounds' full-cube rows already
    // bank, the smooth_stone_slab the slab_side special master the
    // palette row's own stem answers). The palette arm's side row
    // the ONE master banks, the OTHER refs the exact-tile law the
    // mesher's stem walk serves — the jar-driven walk the
    // blockstates -> models chain the real refs collects (never a
    // hardcoded name): any stem the tile census misses APPENDS here
    // (the palette order the walk speaks, the merged map's sorted
    // keys the per-kind order keeps — the deterministic scenes the
    // stable order banks), the existing indices stay byte-identical
    // (the golden safety the append law keeps), the pre-colored
    // masters ride no tint (the split refs NO tintindex carry).
    for block in RUNTIME_PALETTE {
        if !block.ends_with("_slab") {
            continue;
        }
        let bs_bytes = read_jar(
            &mut archive,
            &format!("assets/minecraft/blockstates/{block}.json"),
        )?;
        let bs: Value = serde_json::from_slice(&bs_bytes)
            .with_context(|| format!("blockstate {block}: json parse"))?;
        let Some(variants) = bs.get("variants").and_then(|v| v.as_object()) else {
            continue;
        };
        for key in ["type=top", "type=double"] {
            let Some(entry) = variants.get(key) else {
                continue;
            };
            let Some(model_ref) = (if entry.is_array() {
                entry.get(0)
            } else {
                Some(entry)
            })
            .and_then(|a| a.get("model"))
            .and_then(|m| m.as_str())
            .map(|m| m.trim_start_matches("minecraft:").to_string())
            else {
                continue;
            };
            let (tex_map, _) = load_model_chain(&mut archive, &model_ref)
                .with_context(|| format!("block {block}: split model chain {model_ref}"))?;
            let mut stems: Vec<String> = tex_map
                .values()
                .map(|tex| {
                    tex.trim_start_matches("minecraft:")
                        .trim_start_matches("block/")
                        .to_string()
                })
                .collect();
            stems.sort();
            stems.dedup(); // the bottom/top split keys the SAME master the pair wears — the one-tile bank the dedup keeps
            for stem in stems {
                if tile_names.iter().any(|n| *n == stem) {
                    continue; // the banked row the dedup the walk keeps
                }
                let raw = read_jar(
                    &mut archive,
                    &format!("assets/minecraft/textures/block/{stem}.png"),
                )
                .with_context(|| format!("texture block/{stem}.png (slab {block} {key})"))?;
                let (w, h, rgba) = decode_rgba(&raw)?;
                let (w, h, rgba) = first_frame(w, h, &rgba);
                if w != 16 || h != 16 {
                    bail!("texture {stem}: expected 16x16 (first frame), got {w}x{h}");
                }
                let mut bytes = rgba;
                bake_tint(&mut bytes, tint_for_texture(&stem));
                tile_names.push(stem.clone());
                tile_bytes.push(bytes);
            }
        }
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

    // THE BAMBOO LEAVES' OWN APPEND (the autopush-608 round, the
    // official 26.2 jar): the two crossed-plane masters the stalk
    // multipart's leaves arms reference (block/bamboo_small_leaves,
    // block/bamboo_large_leaves — the blockstates' own when-conditions
    // the mesher's lane selects, the per-quad texture verdict the
    // named consts read). NO palette row references them (the stalk
    // row the #all texture banks), so the two tiles APPEND here
    // after the block tiles — the existing indices stay byte-identical
    // (the golden safety the append-at-the-end law keeps), the destroy
    // stages the +2 shift the named DESTROY_TILES consts absorb. The
    // PRE-COLORED masters ride no tint (the BlockColors bamboo
    // absence the jar's own registry speaks — the pixels as authored).
    let mut bamboo_leaf_tiles: Vec<u16> = Vec::with_capacity(2);
    for stem in ["bamboo_small_leaves", "bamboo_large_leaves"] {
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
        tile_names.push(stem.to_string());
        tile_bytes.push(bytes);
        bamboo_leaf_tiles.push(idx);
    }

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
        // THE PORTAL'S OWN STRIP (the autopush-605 round): the gate's
        // purple noise the 16x512 32-frame strip animates (the mcmeta's
        // empty animation the sequential 0..31 frametime-1 playlist
        // wears), the NETHER_PORTAL strip consts the walk banks
        "nether_portal",
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
    // THE LIT-LOG MASTERS' OWN BLEND BANK (the dev-6 round banked the
    // static frame-0 still — the autopush-527 round the interpolate
    // gap CLOSES): the two lit_log textures the template_campfire's
    // lit faces name — 16x64 strips whose mcmeta carries
    // `{"interpolate": true, "frametime": 20}` (the 4-frame 80-tick
    // cycle). The vanilla sprite animator CROSS-FADES the frames the
    // interpolate key speaks (javap SpriteContents$AnimationState.
    // drawToAtlas — the alpha = subFrame/time law the thousandths
    // quantization quantizes, the (frame+1) % count modulo the wrap
    // law speaks), so the bank walks the FULL cycle: the base tile =
    // the frame-0 still the named const carries (the tick-0 blend the
    // SAME pixels answers — alpha 0 the mix's own identity keeps, the
    // golden safety the byte-identical head rides), then one blend
    // tile PER TICK of the 80-tick cycle (the per-tick resolution the
    // vanilla per-sub-tick alpha the tick fold quantizes — the frametime-1
    // cadence the strip registration wears). Named tiles the mesher's
    // per-quad texture verdict reads (the TILE_CAMPFIRE_LOG_LIT/
    // TILE_SOUL_CAMPFIRE_LOG_LIT consts the emit answers); the strip
    // the anim walk's own tables bank (the CAMPFIRE_LOG_LIT_* consts
    // the emission answers).
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
        if w != 16 || h == 0 || h % w != 0 {
            bail!("texture {stem}: expected the 16-wide strip, got {w}x{h}");
        }
        let frame_count = (h / w) as usize;
        // THE MCMETA'S OWN TRUTH (the fire strips' walk the same read
        // keeps): the frametime the key speaks, the interpolate key
        // the blend lane DEMANDS (the absent/false key the static
        // walk a future step strip keeps).
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
        let frametime_lit = anim
            .get("frametime")
            .and_then(|v| v.as_u64())
            .unwrap_or(1);
        if frametime_lit == 0 {
            bail!("texture {stem}: frametime 0 refuses");
        }
        if !anim
            .get("interpolate")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            bail!("texture {stem}: the interpolate key the blend lane demands");
        }
        // THE TINT BAKE THE FRAMES WEAR FIRST (the static bank's own
        // walk the per-frame bake keeps): the blend the TINTED frames
        // the atlas banks mix (the lit-log stems the neutral
        // fall-through answers — the bake the no-op keeps).
        let strip_tint = tint_for_texture(stem);
        let row = 16 * 16 * 4;
        let frames: Vec<Vec<u8>> = (0..frame_count)
            .map(|k| {
                let mut bytes = rgba[k * row..(k + 1) * row].to_vec();
                bake_tint(&mut bytes, strip_tint);
                bytes
            })
            .collect();
        // THE CYCLE'S OWN BANK: tick 0 the base tile reuses (the pure
        // frame 0 the alpha-0 blend answers), ticks 1..frames*frametime
        // the blend tiles bank (the pure frames 1.. the alpha-0 tails
        // the span boundaries speak).
        let cycle = frame_count as u64 * frametime_lit;
        let base_idx = tile_names.len() as u16;
        tile_names.push(stem.to_string());
        tile_bytes.push(frames[0].clone());
        let mut tiles: Vec<u16> = Vec::with_capacity(cycle as usize);
        tiles.push(base_idx);
        for t in 1..cycle {
            let cur = (t / frametime_lit) as usize % frame_count;
            let nxt = (cur + 1) % frame_count;
            let blended = blend_frames(
                &frames[cur],
                &frames[nxt],
                (t % frametime_lit) as usize,
                frametime_lit,
            );
            let idx = tile_names.len() as u16;
            tile_names.push(format!("{stem}_blend{t:02}"));
            tile_bytes.push(blended);
            tiles.push(idx);
        }
        // THE STRIP'S OWN REGISTRATION — the anim walk the frametime-1
        // cadence keeps (each tick ONE blend tile the cycle banks),
        // the identity playlist the FRAMES walk serves.
        anim_strips.push((
            stem.to_string(),
            tiles,
            (0..cycle).map(|i| i as u8).collect(),
            1,
        ));
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
    // tiles already bank). THE FOURTH CAKE TILE (the autopush-524
    // round): cake_inner the bitten cut's own texture the slice
    // models' #inside ref names — the west face the per-quad verdict
    // the TILE_CAKE_INNER const answers.
    // THE LEVER'S BASE PLATE'S OWN SIDECAR (the dev-45 round): the
    // cobblestone master the lever model's `#base` ref names — a plain
    // 16x16 still the mcmeta never animates (the lever base the same
    // law the cake bank wears). APPENDED after the palette rows — the
    // block palette's tile indices stay byte-identical (the
    // deterministic scenes keep their exact pixels). The named tile
    // the mesher's lever lane's per-quad texture verdict reads (the
    // TILE_COBBLESTONE const the emit answers).
    if !tile_names.iter().any(|n| n == "cobblestone") {
        let raw = read_jar(
            &mut archive,
            "assets/minecraft/textures/block/cobblestone.png",
        )
        .with_context(|| "texture block/cobblestone.png")?;
        let (w, h, rgba) = decode_rgba(&raw)?;
        if w != 16 || h != 16 {
            bail!("texture cobblestone: expected 16x16, got {w}x{h}");
        }
        let mut bytes = rgba;
        bake_tint(&mut bytes, tint_for_texture("cobblestone"));
        tile_names.push("cobblestone".to_string());
        tile_bytes.push(bytes);
    }
    for stem in ["cake_bottom", "cake_top", "cake_side", "cake_inner"] {
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
    // THE REDSTONE DUST'S OWN TINT BANK (the autopush-665 round): the
    // wire's two states the power ramp the getColorForPower table the
    // lambda$static$0 fill speaks — the OFF row the power-0 color
    // (0.3, 0, 0) the (77, 0, 0) RGB triple wears, the ON row the
    // power-15 color (1.0, 0.2, 0.0) the (255, 51, 0) triple banks
    // (the colorFromFloat byte fold `(int)(f * 255.0 + 0.5)`). The
    // grayscale masters the wire arm's resolve walk already banked
    // (the untinted dot/line0/line1 tiles the palette rows' own slots
    // carry) — the OFF tint bakes INTO the masters' own bytes (the
    // tile names the jar stems keep, the tile table's growth the
    // three ON tiles the tail seats bank — the pre-wire indices stay
    // byte-identical, the golden safety the append-at-the-end law
    // keeps). The ON tiles the {stem}_on names bank (the mesher's
    // wire path the ON kind's own slots read). APPENDED after the
    // cake masters — the block palette's tile indices stay
    // byte-identical, the destroy stages the +3 shift the named
    // consts the emit absorbs.
    if let Some(dot_off) = tile_names.iter().position(|n| n == "redstone_dust_dot") {
        let line0_off = tile_names
            .iter()
            .position(|n| n == "redstone_dust_line0")
            .with_context(|| "tile redstone_dust_line0: absent from the banked tiles")?;
        let line1_off = tile_names
            .iter()
            .position(|n| n == "redstone_dust_line1")
            .with_context(|| "tile redstone_dust_line1: absent from the banked tiles")?;
        // THE OFF TINT'S OWN BAKE (the masters' own bytes the power-0
        // color wears — the grayscale 217..254 pixels the dark red the
        // vanilla's unpowered dust paints)
        for idx in [dot_off, line0_off, line1_off] {
            bake_tint(&mut tile_bytes[idx], 0x4D0000);
        }
        // THE ON TILES' OWN BANK (the power-15 color the bright red
        // the powered dust speaks — the three masters the tinted
        // copies the {stem}_on names carry; the jar's OWN grayscale
        // the honest source the bake reads — the OFF-tinted bytes the
        // double-darken never wear)
        let on_tint = 0xFF3300u32;
        for stem in [
            "redstone_dust_dot_on",
            "redstone_dust_line0_on",
            "redstone_dust_line1_on",
        ] {
            let raw = read_jar(
                &mut archive,
                &format!(
                    "assets/minecraft/textures/block/{}.png",
                    stem.trim_end_matches("_on")
                ),
            )
            .with_context(|| format!("texture {stem}: master read"))?;
            let (w, h, rgba) = decode_rgba(&raw)?;
            if w != 16 || h != 16 {
                bail!("texture {stem}: expected 16x16 (first frame), got {w}x{h}");
            }
            let (_, _, rgba) = first_frame(w, h, &rgba);
            let mut bytes = rgba;
            bake_tint(&mut bytes, on_tint);
            tile_names.push(stem.to_string());
            tile_bytes.push(bytes);
        }
        // THE WIRE ROWS' OWN SLOT WALK (the palette tail's own seats
        // the index finder banks — the OFF row the dot/N-S/E-W masters
        // slots 0..2 wear, the ON row the _on tiles the same seats
        // keep; slots 3..5 the dot master the never-rendered faces
        // bank)
        let wire_off_row = RUNTIME_PALETTE
            .iter()
            .position(|b| *b == "redstone_wire")
            .with_context(|| "palette row redstone_wire: absent")?;
        let wire_on_row = RUNTIME_PALETTE
            .iter()
            .position(|b| *b == "redstone_wire_on")
            .with_context(|| "palette row redstone_wire_on: absent")?;
        let dot_on_idx = tile_names
            .iter()
            .position(|n| n == "redstone_dust_dot_on")
            .with_context(|| "tile redstone_dust_dot_on: absent")?;
        let line0_on_idx = tile_names
            .iter()
            .position(|n| n == "redstone_dust_line0_on")
            .with_context(|| "tile redstone_dust_line0_on: absent")?;
        let line1_on_idx = tile_names
            .iter()
            .position(|n| n == "redstone_dust_line1_on")
            .with_context(|| "tile redstone_dust_line1_on: absent")?;
        let (dot_idx, line0_idx, line1_idx) = (dot_off as u16, line0_off as u16, line1_off as u16);
        let (dot_on_idx, line0_on_idx, line1_on_idx) = (
            dot_on_idx as u16,
            line0_on_idx as u16,
            line1_on_idx as u16,
        );
        face_tiles[wire_off_row] = [
            dot_idx, line0_idx, line1_idx, dot_idx, dot_idx, dot_idx,
        ];
        face_tiles[wire_on_row] = [
            dot_on_idx, line0_on_idx, line1_on_idx, dot_on_idx, dot_on_idx, dot_on_idx,
        ];
        // THE TINT ROWS' OWN ZEROS (the pre-colored pixels the TINTS
        // table never re-multiples — the wire rows the same law the
        // pre-colored masters keep)
        face_tints[wire_off_row] = [0; 6];
        face_tints[wire_on_row] = [0; 6];
        // THE GRADED RUNGS' OWN BANK (the autopush-673 round): the 14
        // middle powers the per-power tinted copies the tail seats
        // bank — the getColorForPower ramp the lambda$static$0 fill
        // speaks (f = power/15; r = f*0.6 + (p > 0 ? 0.4 : 0.3);
        // g = clamp(f*f*0.7 - 0.5, 0, 1); b = clamp(f*f*0.6 - 0.7, 0,
        // 1); the colorFromFloat byte fold `(int)(f * 255.0 + 0.5)` —
        // the power-0 row the (77, 0, 0) OFF bake the masters' own
        // bytes wear, the power-15 row the (255, 51, 0) ON tint the
        // same fold banks). The jar's OWN grayscale masters the
        // honest source each bake reads (the OFF-tinted bytes the
        // double-darken never wear), the {stem}_pNN names the tail
        // tiles carry, the ladder rows' slots the same seats the
        // OFF/ON rows wear (slots 0..2 the dot/N-S/E-W masters, slots
        // 3..5 the dot master the never-rendered faces keep), the
        // tint rows the zeros the pre-colored law keeps.
        let rung_rows: Vec<usize> = RUNTIME_PALETTE
            .iter()
            .enumerate()
            .filter(|(_, n)| n.starts_with("redstone_wire_p"))
            .map(|(i, _)| i)
            .collect();
        if rung_rows.len() != 14 {
            bail!(
                "palette rows redstone_wire_pNN: expected 14 rungs, found {}",
                rung_rows.len()
            );
        }
        for (r, row) in rung_rows.iter().enumerate() {
            let power = (r + 1) as u8; // the row order the power order wears
            let f = power as f32 / 15.0;
            let rr = f * 0.6 + 0.4; // the p > 0 arm the middle rungs wear
            let gg = (f * f * 0.7 - 0.5).clamp(0.0, 1.0);
            let bb = (f * f * 0.6 - 0.7).clamp(0.0, 1.0);
            let color = (((rr * 255.0 + 0.5) as u32) << 16)
                | (((gg * 255.0 + 0.5) as u32) << 8)
                | ((bb * 255.0 + 0.5) as u32);
            let mut rung_tiles = [0u16; 3];
            for (slot, master_stem) in [
                "redstone_dust_dot",
                "redstone_dust_line0",
                "redstone_dust_line1",
            ]
            .iter()
            .enumerate()
            {
                let stem = format!("{}_p{:02}", master_stem, power);
                let raw = read_jar(
                    &mut archive,
                    &format!(
                        "assets/minecraft/textures/block/{}.png",
                        master_stem
                    ),
                )
                .with_context(|| format!("texture {stem}: master read"))?;
                let (w, h, rgba) = decode_rgba(&raw)?;
                if w != 16 || h != 16 {
                    bail!("texture {stem}: expected 16x16 (first frame), got {w}x{h}");
                }
                let (_, _, rgba) = first_frame(w, h, &rgba);
                let mut bytes = rgba;
                bake_tint(&mut bytes, color);
                tile_names.push(stem.clone());
                tile_bytes.push(bytes);
                rung_tiles[slot] = (tile_names.len() - 1) as u16;
            }
            face_tiles[*row] = [
                rung_tiles[0],
                rung_tiles[1],
                rung_tiles[2],
                rung_tiles[0],
                rung_tiles[0],
                rung_tiles[0],
            ];
            face_tints[*row] = [0; 6];
        }
    }
    // THE HONEY MASTERS' OWN STATIC BANK (the autopush-599 round): the
    // two skin textures the honey model's INSET element names — #up
    // (honey_block_top) and #side (honey_block_side) the two-element
    // walk's per-quad verdict reads; the #down master the palette's
    // own kind row already banks (the generic walk's first-element
    // resolution — tile 78 the six slots wear). The UNIFORM PARTIAL
    // ALPHA the textures keep (the PNG alpha maps: top 191 / side 189
    // / bottom 129 — every texel the same value speaks) the per-texel
    // translucent arm composites. APPENDED after the palette rows —
    // the block palette's tile indices stay byte-identical (the
    // deterministic scenes keep their exact pixels).
    for stem in ["honey_block_top", "honey_block_side"] {
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
    // THE FLOW STRIPS' OWN QUADRANT BANK (the autopush-529 round): the
    // two flow textures the fluid SIDE faces wear — the still-only
    // world the lane never drew. 32x32-PER-FRAME strips (water_flow
    // 32x1024 = 32 frames, lava_flow 32x512 = 16 frames; the official
    // 26.2 jar's own IHDRs), the mcmeta the frametime speaks (water {}
    // -> the default 1, lava {"frametime": 3}) and NO interpolate key
    // (the static sequential walk the fire strips' own law keeps).
    // THE QUADRANT LAW (javap FluidRenderer pc 1515-1567, the 26.2
    // jar): the horizontal sides ALWAYS flowingMaterial().sprite()
    // wear (the still pick the top/bottom faces' own verdict keeps),
    // the corner UVs the HALF-sprite window sample — u = getU(0.0)..
    // getU(0.5), v = getV((1-h)*0.5)..getV(0.5) (the corner heights h
    // the fluid's own walk speaks; the full-height cell the top-left
    // 16x16 quadrant answers) — so each frame banks EXACTLY its
    // top-left 16x16 quadrant: the tile the side quad's whole 0..1 UV
    // rect maps, the vanilla-sampled region the pixel-perfect match
    // speaks. The tick-0 verdict the fluids' own law keeps (the
    // sequential head row 0 — the fresh atlas's first frame), the
    // appended rows the existing indices 0..442 the END append keeps.
    for stem in ["water_flow", "lava_flow"] {
        let raw = read_jar(
            &mut archive,
            &format!("assets/minecraft/textures/block/{stem}.png"),
        )
        .with_context(|| format!("texture block/{stem}.png"))?;
        let (w, h, rgba) = decode_rgba(&raw)?;
        if w != 32 || h == 0 || h % w != 0 {
            bail!("texture {stem}: expected the 32-wide flow strip, got {w}x{h}");
        }
        let frame_count = (h / w) as usize;
        let mc_bytes = read_jar(
            &mut archive,
            &format!("assets/minecraft/textures/block/{stem}.png.mcmeta"),
        )
        .with_context(|| format!("texture {stem}: mcmeta read"))?;
        let mc: Value = serde_json::from_slice(&mc_bytes)
            .with_context(|| format!("texture {stem}: mcmeta parse"))?;
        let anim = mc
            .get("animation")
            .with_context(|| format!("texture {stem}: no animation key"))?;
        // THE SERIALIZER'S OWN DEFAULTS (the still strips' own walk the
        // same laws keep): the frametime the absent key's 1 tick wears,
        // the frames the absent array's sequential 0..count-1 walk
        // serves (the flow mcmetas' EMPTY {} animation the full strip
        // plays — no interpolate key the static walk keeps).
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
        if playlist.iter().any(|&f| (f as usize) >= frame_count) {
            bail!("texture {stem}: playlist row beyond the strip's frames");
        }
        if playlist[0] != 0 {
            bail!(
                "texture {stem}: the playlist's head row {head} refuses — the tick-0 still the frame 0 wears",
                head = playlist[0]
            );
        }
        // THE QUADRANT CROP: frame k's byte base k*32rows*stride (the
        // 32-px row the 128-byte stride wears), the quadrant the TOP
        // half's rows (0..16) x the LEFT half's columns (bytes 0..64)
        // — the 16x16 RGBA tile the side quad's UV rect the whole
        // window answers. The tint bake the water master's own row
        // carries (water_flow the TINT_WATER bake the water_still
        // rows speak; lava_flow the pre-colored master the untinted
        // default keeps).
        let tile_len = 16 * 16 * 4;
        let src_stride = 32 * 4;
        let mut tiles: Vec<u16> = Vec::with_capacity(frame_count);
        for k in 0..frame_count {
            let frame_base = k * 32 * src_stride;
            let mut bytes = vec![0u8; tile_len];
            for r in 0..16usize {
                let src = frame_base + r * src_stride;
                let dst = r * 16 * 4;
                bytes[dst..dst + 16 * 4].copy_from_slice(&rgba[src..src + 16 * 4]);
            }
            bake_tint(&mut bytes, tint_for_texture(stem));
            let idx = tile_names.len() as u16;
            tile_names.push(format!("{stem}_frame{k:02}"));
            tile_bytes.push(bytes);
            tiles.push(idx);
        }
        anim_strips.push((stem.to_string(), tiles, playlist, frametime));
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
    // THE DOOR FAMILY'S OWN MODEL ROWS (the autopush-656 round, the
    // autopush-658 hinge-right tail): the per-kind model ref the
    // blockstate's facing=north row resolved (the hinge word the per-
    // hinge row keys — the LEFT rows the first lane's pin banks, the
    // RIGHT tail the census's verdict rows speak) — the palette-row
    // order the kind ids speak (the non-door rows the empty string
    // the mesher's door kinds alone read), the WAXED ladder the
    // UNWAXED models the real chain serves (the mesher's bake the
    // jar's own resolution keeps — the NAME_OF-derived word the waxed
    // kinds the absent models never answer).
    src.push_str("\n/// The door family's per-kind model refs (the palette-row order the \
     \x20/// kind ids wear — the non-door rows the empty string keeps; the \
     \x20/// autopush-656 round the mesher's bake the real chain serves).\n\n");
    src.push_str("pub const DOOR_MODEL_BY_ROW: [&str; N_KINDS] = [\n");
    for m in &door_models {
        src.push_str(&format!("    {:?},\n", m));
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
        ("barrel_top_open", "TILE_BARREL_TOP_OPEN"),
        ("cobblestone", "TILE_COBBLESTONE"),
        ("cake_bottom", "TILE_CAKE_BOTTOM"),
        ("cake_top", "TILE_CAKE_TOP"),
        ("cake_side", "TILE_CAKE_SIDE"),
        ("cake_inner", "TILE_CAKE_INNER"),
        ("honey_block_bottom", "TILE_HONEY_BLOCK_BOTTOM"),
        ("honey_block_top", "TILE_HONEY_BLOCK_TOP"),
        ("honey_block_side", "TILE_HONEY_BLOCK_SIDE"),
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
    // THE DOOR MASTERS' OWN CONSTS (the autopush-655 round, the
    // dev-43 wood generalization): the per-wood door texture pairs
    // the palette's own quartet rows bank (the lower's #bottom / the
    // upper's #top refs the flat-stem arms resolve) — the mesher's
    // door lane's per-quad texture verdict the named consts read.
    // The seventeen textured bases the DOOR_TEXTURED_BASES list the
    // emit walks (the waxed bases the unwaxed pairs the resolved
    // chain shares — no waxed texture the jar keeps).
    for base in DOOR_TEXTURED_BASES {
        let prefix_base: String = base
            .split('_')
            .map(|w| w.to_uppercase())
            .collect::<Vec<_>>()
            .join("_");
        for (half, prefix_half) in [("bottom", "BOTTOM"), ("top", "TOP")] {
            let stem = format!("{base}_{half}");
            let prefix = format!("TILE_{prefix_base}_{prefix_half}");
            let idx = tile_const(&tile_names, &stem)?;
            src.push_str(&format!(
                "\n/// The door lane's own {} tile (the autopush-655\n\
                 \x20    /// round's family walk, the dev-43 wood\n\
                 \x20    /// generalization — the door element geometry's\n\
                 \x20    /// per-quad texture verdict reads it; the two\n\
                 \x20    /// masters the palette's own quartet rows bank).\n\
                 \x20    pub const {}: u16 = {};\n",
                stem, prefix, idx
            ));
        }
    }
    src.push_str(&format!(
        "\n/// The destroy_stage_0..9 tile indices (the breaking overlay's own\n\
         \x20    /// crack frames; the vanilla `DestroyStage.getDestructor` law maps\n\
         \x20    /// the progress to the index). Appended after the block tiles.\n\
         \x20    pub const DESTROY_TILES: [u16; 10] = {:?};\n",
        destroy_tiles
    ));
    // THE BAMBOO LEAVES' OWN CONSTS (the autopush-608 round): the two
    // appended masters the stalk lane's per-quad texture verdict reads
    // (the leaves arms' crossed-plane quads the tiles carry; the
    // stalk's own tile the FACE_TILES row the palette arm banks).
    let bamboo_leaves_consts: [(&str, &str, u16); 2] = [
        ("bamboo_small_leaves", "TILE_BAMBOO_SMALL_LEAVES", bamboo_leaf_tiles[0]),
        ("bamboo_large_leaves", "TILE_BAMBOO_LARGE_LEAVES", bamboo_leaf_tiles[1]),
    ];
    for (stem, prefix, idx) in bamboo_leaves_consts {
        src.push_str(&format!(
            "\n/// The bamboo lane's own {} tile (the autopush-608\n\
             \x20    /// round — the stalk multipart's leaves arms'\n\
             \x20    /// per-quad texture verdict reads it; the appended\n\
             \x20    /// master the palette walk never references).\n\
             \x20    pub const {}: u16 = {};\n",
            stem, prefix, idx
        ));
    }
    // THE ANIMATED STRIPS' OWN TABLES (the dev-5 round's fire, the
    // autopush-521 round's fluids): per-strip constants the draw-time
    // remap walks — the frame-tile arrays, the mcmeta playlists, the
    // per-strip frametime (the single-constant law the fire-only lane
    // wore can't serve the fluids' frametime 2 — the per-strip truth
    // the mcmeta itself speaks), the base tile the quads bake. The
    // generated names the palette order carries: the const prefix the
    // strip's own stem maps (fire_0 -> FIRE_0 ... water_still -> WATER,
    // lava_still -> LAVA — the names the client's walks read).
    let prefixes: [(&str, &str); 13] = [
        ("fire_0", "FIRE_0"),
        ("fire_1", "FIRE_1"),
        ("soul_fire_0", "SOUL_FIRE_0"),
        ("soul_fire_1", "SOUL_FIRE_1"),
        ("water_still", "WATER"),
        ("lava_still", "LAVA"),
        ("campfire_fire", "CAMPFIRE_FIRE"),
        ("soul_campfire_fire", "SOUL_CAMPFIRE_FIRE"),
        ("campfire_log_lit", "CAMPFIRE_LOG_LIT"),
        ("soul_campfire_log_lit", "SOUL_CAMPFIRE_LOG_LIT"),
        // THE FLOW STRIPS' OWN PREFIXES (the autopush-529 round): the
        // side-face lane's base tiles the consts carry (the mesher's
        // side-face verdict the WATER_FLOW_FRAME0/LAVA_FLOW_FRAME0
        // rows read).
        ("water_flow", "WATER_FLOW"),
        ("lava_flow", "LAVA_FLOW"),
        // THE PORTAL'S OWN PREFIX (the autopush-605 round): the gate's
        // strip the consts bank (the FRAME0 the palette row's own tile
        // the strip_base lookup answers, the 32-frame playlist the
        // anim_tile arm remaps)
        ("nether_portal", "NETHER_PORTAL"),
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
         \x20    /// TextureAtlasSprite animator's per-tick walk speaks, the strips'\n\
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
/// sound row both halves speak). The `_lit` suffix the same walk
/// answers (the candle/campfire/candle-cake pairs the lit swaps'
/// products never place), the cake ladder's `cake_sliceN` the BASE
/// "cake" row the shared-row law keeps (the autopush-524 round —
/// the dig row, the drops table, the sounds' WOOL row and the item
/// id the pick lane grants EVERY stage speak). Every other kind the
/// base name IS the palette name.
fn registry_names() -> [&'static str; RUNTIME_PALETTE.len()] {
    let mut names = [""; RUNTIME_PALETTE.len()];
    for (i, name) in RUNTIME_PALETTE.iter().enumerate() {
        // THE DOOR FAMILY'S OWN STRIP (the autopush-655 round, the
        // autopush-658 hinge word — the open bit the kind-only
        // vocabulary wears, the RIGHT tail the hinge the jar's
        // registry never speaks: the OPEN suffix the jar's registry
        // never answers, the _right word the same one-entry law the
        // base row keeps — the DoorBlock's ONE registry entry both
        // hinges and both states the same "oak_door" name keeps, the
        // dig row (3.0, false), the sounds' WOODEN row and the item
        // id the pick lane grants BOTH halves speak), the half-word
        // the _upper strip the same law answers — the _right strip
        // FIRST (the tail rows the hinge word the both-suffix kinds
        // wear), the _open strip NEXT (the upper-open kind the
        // two-suffix word wears, the both strips the base row banks)
        let base = name.strip_suffix("_right").unwrap_or(name);
        let base = base.strip_suffix("_open").unwrap_or(base);
        names[i] = base
            .strip_suffix("_upper")
            .or_else(|| base.strip_suffix("_lit"))
            .or_else(|| base.strip_suffix("_on"))
            // THE BUTTON'S OWN PRESSED STRIP (the autopush-660 round):
            // the pressed row the jar's registry never speaks — the
            // ButtonBlock's ONE "stone_button" entry both states the
            // same name keeps (the dig row (0.5, false), the item id
            // the pick lane grants BOTH kinds speak)
            .or_else(|| base.strip_suffix("_pressed"))
            // THE PLATE'S OWN DOWN STRIP (the dev-48 round): the down
            // row the jar's registry never speaks — the
            // PressurePlateBlock's ONE "oak_pressure_plate" entry both
            // states the same name keeps (the dig row (0.5, false),
            // the item id the pick lane grants BOTH kinds speak)
            .or_else(|| base.strip_suffix("_down"))
            // THE REDSTONE TORCH'S OWN OFF STRIP (the autopush-666
            // round): the off row the jar's OWN model suffix wears —
            // the blockstates' lit=false row the redstone_torch_off
            // model answers, the RedstoneTorchBlock's ONE
            // "redstone_torch" registry entry both states the same
            // name keeps (the dig row (0.0, false), the item id the
            // pick lane grants BOTH kinds speak)
            .or_else(|| base.strip_suffix("_off"))
            // THE CAKE LADDER'S OWN STRIP (the autopush-524 round): the
            // seven bite stages the BASE "cake" row the shared-row law
            // keeps — the dig row (0.5 hardness), the drops table (the
            // cake item), the sounds' WOOL row and the item id the pick
            // lane grants EVERY stage speak (the world.rs NAME_OF rows
            // the cross-check asserts — the cake_sliceN name the palette
            // alone wears)
            .or_else(|| {
                if name.starts_with("cake_slice") {
                    Some("cake")
                } else {
                    None
                }
            })
            // THE WIRE LADDER'S OWN P-STRIP (the autopush-673 round):
            // the 14 middle rungs the jar's registry never speaks —
            // the RedStoneWireBlock's ONE "redstone_wire" entry all
            // sixteen states the same name keeps (the dig row (0.0,
            // false), the drops table, the sounds' block row and the
            // item id the pick lane grants every rung speak)
            .or_else(|| {
                if name.starts_with("redstone_wire_p") {
                    Some("redstone_wire")
                } else {
                    None
                }
            })
            .unwrap_or(base);
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
        // the cake ladder (the autopush-524 round): the seven bite
        // stages the eat lane's kind-swap walk wears
        assert_eq!(RUNTIME_PALETTE[67], "cake");
        assert_eq!(RUNTIME_PALETTE[68], "cake_slice1");
        assert_eq!(RUNTIME_PALETTE[69], "cake_slice2");
        assert_eq!(RUNTIME_PALETTE[70], "cake_slice3");
        assert_eq!(RUNTIME_PALETTE[71], "cake_slice4");
        assert_eq!(RUNTIME_PALETTE[72], "cake_slice5");
        assert_eq!(RUNTIME_PALETTE[73], "cake_slice6");
        // the signal fire's own kind (the autopush-530 round): the hay
        // bale the campfire's below-cell the isSmokeSource verdict reads
        assert_eq!(RUNTIME_PALETTE[74], "hay_block");
        // the TOOL WAKE'S OWN PAIR (the dev-22 round): the cobweb the
        // sword/shears pinned rows' first palette-native consumer (the
        // cross model the cobweb arm resolves), the white wool the
        // ColorCollection member the full-cube generic walk resolves
        assert_eq!(RUNTIME_PALETTE[75], "cobweb");
        assert_eq!(RUNTIME_PALETTE[76], "white_wool");
        // the speed factor wake's own row (the autopush-598 round): the
        // soul sand the speedFactor(0.4f) clinit row's first palette-native
        // consumer (the full-cube generic walk resolves, the Column(14)
        // collision the Block.column(16.0, 0.0, 14.0) SHAPE keeps)
        assert_eq!(RUNTIME_PALETTE[77], "soul_sand");
        // the honey wake's own row (the autopush-599 round): the speed
        // factor 0.4 + jump factor 0.5 rows' first palette-native consumer
        // (the two-element model the element walk resolves)
        assert_eq!(RUNTIME_PALETTE[78], "honey_block");
        // the furnace's own pair (the autopush-601 round): the smelter
        // the facing=north rows resolve (block/furnace, the lit kind
        // block/furnace_on the facing=north,lit=true row answers)
        assert_eq!(RUNTIME_PALETTE[79], "furnace");
        assert_eq!(RUNTIME_PALETTE[80], "furnace_lit");
        // THE MOB HEADS' OWN SEVEN SEATS (the dev-80 round): the
        // palette tail the vanilla registration order the block_dig
        // rows banks — the floor kinds the note block's above consult
        // alone reads, the wall pair the future round seats
        assert_eq!(RUNTIME_PALETTE[748], "barrel");
        assert_eq!(RUNTIME_PALETTE[749], "skeleton_skull");
        assert_eq!(RUNTIME_PALETTE[750], "wither_skeleton_skull");
        assert_eq!(RUNTIME_PALETTE[751], "zombie_head");
        assert_eq!(RUNTIME_PALETTE[752], "player_head");
        assert_eq!(RUNTIME_PALETTE[753], "creeper_head");
        assert_eq!(RUNTIME_PALETTE[754], "dragon_head");
        assert_eq!(RUNTIME_PALETTE[755], "piglin_head");
        // the census's own verdict: the seven seats the skull family
        // the placeholder arm answers, the barrel the generic walk wears
        assert!(is_skull_family("skeleton_skull") && is_skull_family("piglin_head"));
        assert!(!is_skull_family("barrel") && !is_skull_family("skeleton_wall_skull"));
        // the shared-row law the registry-name walk speaks: every
        // stage the BASE "cake" row the dig/drop/sound/item lanes keep
        let names = registry_names();
        assert_eq!(names[67], "cake");
        assert_eq!(names[73], "cake");
        // THE HEADS' OWN NAME ROWS (the registry name the palette name
        // IS — no suffix strip the family's plain names wear)
        assert_eq!(names[749], "skeleton_skull");
        assert_eq!(names[752], "player_head");
        assert_eq!(names[755], "piglin_head");
        assert_eq!(FACE_KEYS, ["east", "west", "up", "down", "south", "north"]);
    }

    #[test]
    fn tint_map_covers_grayscale_masters() {
        assert_eq!(tint_for_texture("grass_block_top"), TINT_GRASS);
        assert_eq!(tint_for_texture("oak_leaves"), TINT_FOLIAGE);
        assert_eq!(tint_for_texture("water_still"), TINT_WATER);
        // the flow master's own row (the autopush-529 round): the
        // water_flow strip the side faces wear the SAME TINT_WATER bake
        // the still master speaks (the tint rides the fluid state, never
        // the sprite), the lava_flow the pre-colored master the
        // untinted default keeps (lava_still's own law)
        assert_eq!(tint_for_texture("water_flow"), TINT_WATER);
        assert_eq!(tint_for_texture("lava_flow"), 0, "pre-colored");
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
    fn blend_frames_walks_the_thousandths_law() {
        // THE ALPHA LAW'S OWN QUANTIZATION (javap SpriteContents$AnimationState.
        // drawToAtlas pc 39-76 over the 26.2 jar — the (float) subFrame /
        // (float) frameTime fdiv the ldc 1000.0f fmul + the f2i truncation
        // the thousandths quantization banks): for the lit-log's frametime
        // 20 the quantization the NO-OP law keeps — the f32 grid the k/20
        // values the exact k*50 thousandths answer (the empirical walk
        // the 3/20 -> 150 and the 19/20 -> 950 truncations speak — the
        // f32 rounding rounds UP here, the 0.1500000060 neighbor). The
        // channel mix the unorm8 round-to-nearest the u8 bank keeps.
        let cur = vec![100u8; 4];
        let next = vec![200u8; 4];
        // sub 0: the pure current frame (alpha 0 — the tick-0 head the
        // base tile reuses, the golden safety rides)
        assert_eq!(blend_frames(&cur, &next, 0, 20), cur, "alpha 0 the mix's identity");
        // sub 10: alpha 500/1000 — the exact half the round-to-nearest
        // .5 tie the half-up answer keeps (150)
        assert_eq!(blend_frames(&cur, &next, 10, 20), vec![150u8; 4]);
        // THE DELTA PROBE the laws' own razor keeps — cur 0 / next 170:
        // the 150/1000 alpha's 25.5 the half-up 26 answers, the
        // 149/1000 neighbor's 25.33 the 25 the truncation keeps — the
        // two alphas the probe alone parts
        let lo = vec![0u8; 4];
        let hi = vec![170u8; 4];
        assert_eq!(blend_frames(&lo, &hi, 3, 20), vec![26u8; 4], "the 150/1000 alpha the half-up 26 wears");
        assert_eq!(blend_frames(&lo, &hi, 19, 20), vec![162u8; 4], "the 950/1000 alpha the half-up 162 wears");
        // the plain delta the same law rides (100..200 the 115.0 exact)
        assert_eq!(blend_frames(&cur, &next, 3, 20), vec![115u8; 4]);
        assert_eq!(blend_frames(&cur, &next, 19, 20), vec![195u8; 4]);
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

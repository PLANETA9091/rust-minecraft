//! Deterministic voxel world for the reference renderer.
//! Block ids match the compact runtime palette (mirrored by mc2rs-gen's
//! atlas generator); `mc-registry` supplies the full version registry —
//! the original survival-classic subset generates the deterministic scene,
//! generated members extend it for network terrain.
//!
//! The network world also carries the optional BIOME layer (ids + the
//! per-biome tint quads) — the raw material the mesher's tint fold
//! consumes; the procedural scene carries none and keeps the baked look.

pub const CHUNK_X: usize = 16;
pub const CHUNK_Y: usize = 48;
pub const CHUNK_Z: usize = 16;
pub const WATER_LEVEL: i32 = 10;

/// THE NETHER'S OWN SEA LEVEL (the lava ocean's own surface the
/// 48-tall grid scales — the vanilla y=31 the 128-tall dimension
/// keeps, the proportional ~11 the fiction's band wears; the
/// [`World::generate_nether`] walk the cells the level below fills).
pub const NETHER_LAVA_LEVEL: i32 = 11;

/// THE END'S OWN ISLAND SURFACE (the autopush-612 round — the island
/// top the 48-tall grid scales: the real dimension's y≈64 surface the
/// 256-tall height keeps, the proportional ~12 the fiction's band
/// wears — the pillars' heads the +10 climb answers, the pedestal the
/// surface wears; the [`World::generate_end`] walk the cells raise).
pub const END_ISLAND_TOP: i32 = 12;

/// THE END'S OWN ISLAND DEPTH (the center's own thickness the cone
/// tapers from — the real island's ~40-block underside the 48-tall
/// grid proportionally keeps; the rim the zero the taper speaks).
pub const END_ISLAND_DEPTH: i32 = 8;

use crate::tint;
use mc_registry::generated::atlas;

pub const AIR: u16 = 0;
pub const STONE: u16 = 1;
pub const DIRT: u16 = 2;
pub const GRASS: u16 = 3;
pub const LOG: u16 = 4;
pub const LEAVES: u16 = 5;
pub const WATER: u16 = 6;
pub const SAND: u16 = 7;
// Generated members (mc2rs-gen atlas RUNTIME_PALETTE, same order): network
// terrain that previously stayed air now renders with its official texture.
pub const GRANITE: u16 = 8;
pub const DIORITE: u16 = 9;
pub const ANDESITE: u16 = 10;
pub const DEEPSLATE: u16 = 11;
pub const COBBLED_DEEPSLATE: u16 = 12;
pub const GRAVEL: u16 = 13;
pub const BEDROCK: u16 = 14;
pub const COAL_ORE: u16 = 15;
pub const COPPER_ORE: u16 = 16;
pub const IRON_ORE: u16 = 17;
pub const GOLD_ORE: u16 = 18;
pub const REDSTONE_ORE: u16 = 19;
pub const LAPIS_ORE: u16 = 20;
pub const DIAMOND_ORE: u16 = 21;
pub const EMERALD_ORE: u16 = 22;
pub const NETHERRACK: u16 = 23;
/// The twelfth-audit kind: pale_oak_leaves rides the SAME leaves
/// geometry chain (parent block/leaves) with NO tint — vanilla's
/// BlockColors registers no source for it, the PRE-COLORED texture
/// renders as authored (see `crate::tint::slot_for`'s verdict).
pub const PALE_LEAVES: u16 = 24;
/// The thirteenth-audit kind — the FIRST non-cube member: leaf_litter is
/// a flat zero-thickness ground cover (template_leaf_litter_1..4: quads
/// at y=0.25/16 with up+down faces, both tintindex 0, ambientocclusion
/// false). The dry fold's sole consumer: its faces carry the DRY_SLOT
/// source (`BlockTintSources.dryFoliage()`, the twelfth audit), and the
/// texture is a GRAYSCALE master the game tints in-world (probe: every
/// opaque pixel r==g==b, binary alpha 0/255). The mesher's flat path
/// meshes the full footprint; per-segment coverage needs state-property
/// extraction (documented follow-up).
pub const LEAF_LITTER: u16 = 25;
/// The SECOND fluid member (the lava lane): the body's fire laws read
/// it (the box-overlap scan the touches_lava law speaks), the mesher
/// meshes it as the non-occluding fluid (the water shape, the opaque
/// lava_still pixels), the collision never stands on it.
pub const LAVA: u16 = 26;
/// Kind 27 (the twenty-eighth entry, the autopush-311 round): the hoe's
/// own till arm's product (javap HoeItem.useOn — the TILLABLE map's
/// `Blocks.FARMLAND.defaultBlockState()` the GRASS_BLOCK/DIRT rows speak).
/// The model chain resolves through the atlas generator's full-cube face
/// walk (dirt sides + the farmland top), the mesher meshes it as the cube
/// the vocabulary speaks (the vanilla 15/16 height the generic path folds
/// — the documented simplification).
pub const FARMLAND: u16 = 27;
/// Kind 28 (the twenty-ninth entry, the autopush-312 round): the shovel's
/// own flatten arm's product (javap ShovelItem.useOn — the FLATTENABLES
/// map's `Blocks.DIRT_PATH.defaultBlockState()` the GRASS_BLOCK/DIRT rows
/// speak). The atlas generator's full-cube face walk resolves the model
/// chain (dirt bottom + the path top/sides); the mesher meshes it as the
/// cube the vocabulary speaks (the vanilla 15/16 height the generic path
/// folds — the same documented simplification the farmland kind keeps).
pub const DIRT_PATH: u16 = 28;
/// Kind 29 (the thirtieth entry, the autopush-313 round): the seeds' own
/// planting arm's product (javap BlockItem.place + CropBlock.canSurvive
/// over the real 26.2 jar — the SUPPORTS_CROPS tag's sole
/// `minecraft:farmland` row the VegetationBlock.canSurvive below-check
/// speaks, the `getRawBrightness(pos, 0) >= 8` the hasSufficientLight's
/// bipush/if_icmplt pair keeps). The SECOND non-cube member: block/crop's
/// four zero-thickness vertical planes (x=4/12 spanning z, z=4/12
/// spanning x, each y -1..15 — the sprite sinks one pixel into the
/// farmland), `shade: false` + `ambientocclusion: false` — the mesher's
/// wheat path meshes the four planes' eight quads. The `.noCollision()`
/// row the Blocks clinit speaks: the body/hull/mob collision walks pass
/// the crop, the CLIP targets it (the vanilla outline's full-cube fold —
/// the harvest lane the dig arm rides). The vanilla `Items.WHEAT` is no
/// BlockItem — the harvest item never plants, the placeable lookup
/// refuses the kind (the seeds' own product the plant arm alone wears).
pub const WHEAT: u16 = 29;
/// Kinds 30..36 (the thirty-first..thirty-seventh entries, the
/// autopush-314 round): the crop's own AGE LADDER — the blockstate
/// property the kinds model the farmland's moisture=0 precedent keeps
/// (the state variants the kind list wears). The vanilla ladder is
/// `age=0..7` on `minecraft:wheat` (the CropBlock.AGE property the
/// getMaxAge 7 law speaks); the repo wears each stage as its own kind:
/// WHEAT (29) is age=0, WHEAT_STAGE1..7 (30..36) are ages 1..7 — the
/// names the blockstates/wheat.json's `age=N` rows speak
/// (block/wheat_stage1..7, each model the block/crop parent's own
/// texture ref carries). The growth law (javap CropBlock.randomTick:
/// the nextInt roll the getStateForAge(age + 1) setBlock wears) climbs
/// the ladder kind by kind; every non-cube lane the WHEAT kind speaks
/// the whole family keeps (the four zero-thickness planes the mesher's
/// wheat path meshes per-kind tile, the `.noCollision()` row the
/// body/hull/mob/spawn walks pass, the CLIP targets the stalks the
/// outline's full-cube fold keeps, the placeable refusal the harvest
/// item's no-BlockItem law the whole family wears).
#[cfg_attr(not(test), expect(dead_code))]
pub const WHEAT_STAGE1: u16 = 30;
#[cfg_attr(not(test), expect(dead_code))]
pub const WHEAT_STAGE2: u16 = 31;
#[cfg_attr(not(test), expect(dead_code))]
pub const WHEAT_STAGE3: u16 = 32;
#[cfg_attr(not(test), expect(dead_code))]
pub const WHEAT_STAGE4: u16 = 33;
#[cfg_attr(not(test), expect(dead_code))]
pub const WHEAT_STAGE5: u16 = 34;
#[cfg_attr(not(test), expect(dead_code))]
pub const WHEAT_STAGE6: u16 = 35;
pub const WHEAT_STAGE7: u16 = 36;
/// THE CRAFTING TABLE'S OWN KIND (the autopush-328 round — the atlas
/// RUNTIME_PALETTE's 37 index): the placement arm's own product, the
/// plain opaque cube the full-cube face walk meshes (front/side/top +
/// the oak_planks down face the vanilla model speaks), and the USE
/// lane's own door — the right-press on its cell opens the 3x3 screen
/// (`on_right_press_block_lane`'s interactive arm). The vanilla facing
/// rotation the generic full-cube fold keeps axis-aligned.
pub const CRAFTING_TABLE: u16 = 37;
/// THE TNT BLOCK'S OWN KIND (the autopush-410 round — the atlas
/// RUNTIME_PALETTE's 38 index): the placement arm's own product, the
/// plain opaque cube the full-cube face walk meshes (tnt_top/side/
/// bottom the block/cube_bottom_top model chain resolves), the ignite
/// arm's own target (`on_right_press_block_lane`'s prime walk — the
/// flint_and_steel/fire_charge held the TntBlock.useItemOn law
/// speaks), and the BLAST'S OWN CHAIN SEED (the exploded TNT cell the
/// wasExploded walk answers — the short-fuse PrimedTnt the ray
/// machine's own queue wears). The vanilla `instabreak()` strength
/// the blast resistance table speaks (0.0 — the ray marches through),
/// the `.noCollision()`-less solid cube the collision walks stand on.
pub const TNT: u16 = 38;
/// THE PLAINS' OWN FLOWER KINDS (the autopush-411 round — the atlas
/// RUNTIME_PALETTE's 39/40 indices): the dandelion the flower_plain
/// feature's default state wears, the poppy the high-chance third's own
/// first row keeps (javap the jar's worldgen json — the
/// noise_threshold_provider the 0.33333334 high_chance speaks). The
/// cross model's two diagonal planes the mesher's flower path meshes
/// (the block/cross bake — shade false, no AO), the noCollision +
/// instabreak + grass sound rows the Blocks clinit's own chain keeps,
/// the SUPPORTS_VEGETATION canSurvive the placement arm answers (the
/// honest subset the palette's grass_block/dirt/farmland rows name).
pub const DANDELION: u16 = 39;
pub const POPPY: u16 = 40;
/// THE PLAINS' OWN GRASS TUFT KINDS (the autopush-412 round — the atlas
/// RUNTIME_PALETTE's 41/42 indices): the short_grass the
/// patch_grass_plain placed feature's simple_state_provider wears, the
/// bush the patch_bush's rarity-4/count-24 row keeps (javap the jar's
/// worldgen json — the two decorations the step-9 list the flowers'
/// own siblings wear). The tinted_cross model's two diagonal planes the
/// mesher's tuft path meshes (the block/cross bake PLUS the tint — the
/// tintindex-0 faces the BlockTintSources.grass() source tints), the
/// noCollision + instabreak + grass sound + REPLACEABLE rows the Blocks
/// clinit's own chain keeps (pc 4318/4371 — the `.replaceable()` call
/// the flower chains never make), the same VegetationBlock.canSurvive
/// the SUPPORTS_VEGETATION law the placement arm answers (BushBlock is
/// the short_grass/bush ctor class — the audited Blocks pool). The
/// dig lane rolls the vanilla loot: the short_grass row the 1/8
/// wheat_seeds chance wears, the bush the shears-only gate the hand
/// lane never opens (the generated tables' own verdict — no drops).
pub const SHORT_GRASS: u16 = 41;
pub const BUSH: u16 = 42;
/// THE PLAINS' OWN DOUBLE PLANT KINDS (the autopush-413 round — the
/// atlas RUNTIME_PALETTE's 43..46 indices): the tall_grass the
/// patch_tall_grass_2 placed feature's simple_state_provider wears, the
/// large_fern the patch_large_fern's rarity-5 row keeps (javap the jar's
/// worldgen json — the configured feature the `half=lower` state places,
/// the SimpleBlockFeature's DoublePlantBlock arm the placeAt walk
/// speaks: the LOWER cell the provider's state wears, the UPPER cell
/// the `pos.above()` setBlock adds — the pair the TWO CELLS wear). The
/// `_UPPER` kinds are the client's own HALF-KIND word: the blockstate's
/// `half=upper` variant the atlas resolves (block/tall_grass_top —
/// the upper half's own sprite), while the NAME_OF registry row keeps
/// the BASE name (the dig row's instabreak, the drops' 1/8 wheat_seeds
/// pool — the jar's two pools the IDENTICAL walk answers, the sounds'
/// grass row both halves speak). The canSurvive law (javap
/// DoublePlantBlock over the 26.2 jar): the UPPER half the below-cell
/// read keeps (the same block at half=lower — the is() + getValue
/// chain the bytecode speaks), the LOWER half the VegetationBlock
/// fallback wears (the SUPPORTS_VEGETATION substrate the flowers'
/// own law keeps). The dig lane's PAIR LAW: the broken half the
/// mate's removal wears (the preventDropFromBottomPart walk the
/// upper's dig speaks — setBlock(AIR, 35) + the levelEvent 2001 —
/// the updateShape walk the lower's dig answers silently), the loot
/// the BROKEN cell alone rolls (the setBlock(35) flag the mate's
/// drop walk refuses). The `.replaceable()` rows the clinit pc
/// 17244/17297 chains speak — the placement-into law the tuft
/// family keeps, the pair the orphan's own clear wears.
pub const TALL_GRASS: u16 = 43;
pub const TALL_GRASS_UPPER: u16 = 44;
pub const LARGE_FERN: u16 = 45;
pub const LARGE_FERN_UPPER: u16 = 46;
/// THE FLOWER PROVIDER'S OWN FULL PALETTE (the autopush-414 round — the
/// atlas RUNTIME_PALETTE's 47..53 indices): the flower_plain
/// noise_threshold_provider's own high_states/low_states rows the jar's
/// worldgen json banks — the high band the [poppy, azure_bluet,
/// oxeye_daisy, cornflower] list wears (the poppy first row the kind 40
/// already keeps, the provider's own list order the three successors
/// bank), the low band the [orange_tulip, red_tulip, pink_tulip,
/// white_tulip] rows answer (the provider's `d0 < threshold` arm the
/// NormalNoise walk serves — the -0.8 valley the noise port's own round
/// plants; the rows the vocabulary names, the band the hash lane never
/// fires). The model chain (blockstates/<flower>.json -> block/<flower>
/// -> block/cross) is the dandelion's own cross bake per kind — the two
/// diagonal planes the mesher's flower path meshes, the pre-colored
/// masters the no-tint row keeps (the audited BlockColors register walk
/// the flower family never covers). The noCollision + instabreak +
/// grass sound rows the Blocks clinit's own chains keep, the
/// SUPPORTS_VEGETATION canSurvive the placement arm answers (the same
/// grass_block/dirt/farmland subset the whole family wears), the
/// offsetType(XZ) row the dandelion/poppy chains speak every kind
/// carries.
pub const AZURE_BLUET: u16 = 47;
pub const OXEYE_DAISY: u16 = 48;
pub const CORNFLOWER: u16 = 49;
// THE LOW BAND'S OWN ROWS — the vocabulary the noise port's own round
// plants (the provider's `d0 < threshold` arm the client's hash lane
// never carries): the kind ids the atlas/lanes bank, the placement the
// walk's own NormalNoise walk answers
pub const ORANGE_TULIP: u16 = 50;
pub const RED_TULIP: u16 = 51;
pub const PINK_TULIP: u16 = 52;
pub const WHITE_TULIP: u16 = 53;

/// THE TORCH'S OWN KIND (the dev-32 round — javap the 26.2 jar's
/// Blocks clinit: the TorchBlock Properties chain the
/// `.noCollision().instabreak().sound(SoundType.WOOD).lightLevel(...)`
/// rows the generated tables bank): the ONE-kind family the
/// placement's canSurvive gate, the mesher's element lane and the
/// body's passable set read. The noCollision row the walk-through
/// verdict keeps; the lightLevel row the WIRE world's block-light
/// grid answers (the server's own chunk light data lights the wire
/// torch — the offline lane's own light engine the deferred round
/// banks, the honest documented gap the offline render wears).
pub const TORCH: u16 = 54;

/// THE CONCRETE POWDER'S OWN KIND (javap the 26.2 jar's Blocks clinit —
/// the `white_concrete_powder` row the ConcretePowderBlock ctor wears:
/// the `new ConcretePowderBlock(BlockBehaviour.Properties.of()...,
/// BuiltInRegistries.BLOCK.get(ResourceLocation.withDefaultNamespace(
/// "white_concrete")))` walk the codec's `concrete` field speaks).
/// The FALLING family's third member (the [`is_falling_block`] fn's
/// growth — the falling body the powder's own conversion law arms:
/// the water verdict the FallingBlockEntity.tick's instanceof walk
/// reads, the solidify the shouldSolidify/touchesLiquid pair wears).
/// The FULL cube the default registration keeps (the collision shape's
/// own table the Full arm answers).
pub const CONCRETE_POWDER: u16 = 55;

/// THE CONCRETE'S OWN KIND (the powder's own target — javap
/// ConcretePowderBlock: the `concrete` field the ctor's registry read
/// banks, the `getConcrete().defaultBlockState()` the onLand and the
/// updateShape conversions both speak; the color the powder's own
/// pair keeps — the white row the white powder wears). The block is
/// NOT a FallingBlock in vanilla (the ConcreteBlock extends Block —
/// the falling family's [`is_falling_block`] verdict refuses it: the
/// solidified block the support loss the gravity never wears).
pub const CONCRETE: u16 = 56;

/// Kind 57 (the fifty-eighth entry, the autopush-515 round): the
/// fire arm's own kind — the FLINT'S OWN PRODUCT and the entity-inside
/// brand the body's fire walk reads (javap the 26.2 jar's Blocks
/// clinit, the `BlockIds.FIRE` registration walk pcs 6617-6666: the
/// `replaceable().noCollision().instabreak()` triple the properties
/// chain speaks, the lightLevel lambda `lambda$static$67`'s
/// `bipush 15; ireturn` the emission table answers, the SoundType.WOOL
/// row the dig voice keeps, the PushReaction.DESTROY the piston's
/// verdict wears; the ctor's `super(properties, 1.0F)` the fconst_1
/// the FireBlock bytecode speaks — the fireDamage float the
/// entityInside's hurt(inFire(), fireDamage) tail consumes). The
/// blockstate is a MULTIPART of rotated elements (the floor pair's
/// ±22.5° planes, the side walls' flat z=0.01, all y 0..22.4) — the
/// mesher's fire lane a documented follow-up keeps (the palette row
/// + the fire_0 tile the FACE_TILES table banks; the rendered quads
/// the follow-up speaks, the honest hole the glass kind wears until
/// then). The ENTITY-INSIDE law (BaseFireBlock.entityInside — the
/// same walk both fire kinds inherit): CLEAR_FREEZE (no vocabulary
/// — the freeze lane the repo never carries), then FIRE_IGNITE =
/// `BaseFireBlock::fireIgnite` (fireImmune aside — the bare player
/// never is; the negative arm the no-freeze clock never speaks;
/// the ServerPlayer +1..2 arm the rng-less replay the splash's own
/// (r - r) precedent folds — the arm the body's walk never banks;
/// the `remainingFireTicks < igniteForTicks(floor(8.0F * 20.0F) =
/// 160)` lift the igniteForTicks verdict wears — never a lower),
/// then the runAfter hurt(inFire(), 1.0) the hurt pipeline's own
/// cooldown answers (the GENERIC_BURN voice the landed hurt speaks).
pub const FIRE: u16 = 57;

/// Kind 58 (the fifty-ninth entry, the same round): the SOUL FIRE'S
/// OWN KIND — the `BlockIds.SOUL_FIRE` registration walk pcs 6669-6718
/// the same clinit keeps (the SAME noCollision/instabreak/replaceable
/// rows, the light lambda `lambda$static$68`'s `bipush 10; ireturn`
/// the emission table answers), the SoulFireBlock ctor's
/// `super(properties, 2.0F)` the fconst_2 the bytecode speaks — the
/// fireDamage float the shared entityInside tail consumes: the hurt
/// the soul fire's cell doubles. The ignite law the BaseFireBlock
/// parent keeps VERBATIM (the same 160-tick lift the fire kind's
/// walk speaks — the law the two kinds share, the damage the ctor
/// floats split).
pub const SOUL_FIRE: u16 = 58;

/// Kind 59 (the sixtieth entry, the autopush-522 round): THE CANDLE'S
/// OWN KIND — the canLight consult's own unlit row the two igniter
/// arms wait for (javap the 26.2 jar's CandleBlock ctor: the
/// `setValue(CANDLES, 1).setValue(LIT, false).setValue(WATERLOGGED,
/// false)` walk the default state registers — the kind-only palette
/// the 1-candle unlit state's rows banks). The blockstate's FIRST
/// variant row ("candles=1,lit=false" -> block/candle_one_candle ->
/// block/template_candle — the stick box [7,0,7]->[9,6,9] plus the
/// two ±45° wick planes the generated ELEMENTS table carries) the
/// atlas walk resolves, the `#all` ref the block/candle tile
/// answers. The lightLevel lambda `lambda$static$0` the `lit ? 3 *
/// candles : 0` fold banks — the UNLIT row the dark 0 the emission
/// table keeps. The canSurvive the `Block.canSupportCenter(below,
/// UP)` delegate the torch's own law speaks (the placement gate the
/// [`can_support_torch`] fold serves). The LIT state the palette
/// CANNOT speak as a property — the kind-only vocabulary the
/// [`CANDLE_LIT`] half-kind word carries, the igniter arms' own
/// setValue walk the swap answers.
pub const CANDLE: u16 = 59;

/// Kind 60 (the sixty-first entry, the same round): THE LIT CANDLE'S
/// OWN KIND — the canLight branch's own product (javap
/// FlintAndSteelItem.useOn pc 77-97: the `setBlock(clickedPos,
/// state.setValue(BlockStateProperties.LIT, true), 11)` walk the
/// flag-11 broadcast wears; FireChargeItem.useOn pc 50-67 the
/// `setBlockAndUpdate` flag-3 sibling serves). The "candles=1,
/// lit=true" variant the SAME template geometry resolves (the
/// stick box the wick planes the lit walk keeps verbatim), the
/// `#all` ref the block/candle_lit tile answers (the flame pixels
/// the lit master carries). The lightLevel lambda's lit arm the
/// 1-candle row's 3 the emission table answers — the light grid
/// the set walk's incremental flood bakes (the fire kinds' own
/// rows the same walk speaks). The name the dig/drop/sound lanes
/// resolve the BASE "candle" row keeps (the double-plant's shared
/// row precedent — the instabreak 0.1 + the CANDLE sound row the
/// jar's registry answers, the pick lane the same item id grants).
pub const CANDLE_LIT: u16 = 60;

/// Kind 61 (the sixty-second entry, the dev-6 round): THE UNLIT
/// CAMPFIRE'S OWN KIND — the douse lane's own product and the
/// canLight consult's campfire row the two igniter arms read (javap
/// the 26.2 jar — CampfireBlock's ctor the DEFAULT state registers
/// `LIT=true, SIGNAL_FIRE=false, WATERLOGGED=false, FACING=NORTH`:
/// the vanilla campfire the player places lands LIT — the offline
/// placement lane the generic walk serves, the shovel's douse arm
/// the `setValue(LIT, false)` product this kind speaks, the flint
/// canLight relight the `!LIT` verdict the doused kind answers).
/// CampfireBlock.canLight's pc walk: the CAMPFIRES tag consult +
/// the `!WATERLOGGED` and `!LIT` reads the kind-only palette the
/// UNLIT row answers (the lit kind the consult refuses — the same
/// verdict the candle pair wears). The properties chain the blocks
/// clinit's lambda$static$310 arm banks: strength(2.0), SoundType.
/// WOOD, lightLevel litBlockEmission(15) — the UNLIT row the dark 0
/// the emission table keeps — noOcclusion, the ctor args (true, 1)
/// the spawnParticles/fireDamage pair the body's dose walk reads.
/// THE SHAPE the static init's `Block.column(16.0, 0.0, 7.0)` walk
/// banks (ldc2_w 16.0 / dconst_0 / ldc2_w 7.0) — the full 0..1
/// footprint under the 0..7/16 band the collision table's Column(7)
/// row answers (the body steps onto the 7-px box, the 0.6-block
/// auto-step the sweep serves). No canSurvive override the 26.2
/// class carries — the campfire floats the support's loss never
/// douses, the placement gate's absence the generic lane keeps.
/// The name the dig/drop/sound lanes resolve the BASE "campfire"
/// row keeps (the generated block_dig/block_sounds/block_drops
/// tables' own rows the registry answers, the item id the pick
/// lane grants).
pub const CAMPFIRE: u16 = 61;

/// Kind 62 (the sixty-third entry, the same round): THE LIT
/// CAMPFIRE'S OWN KIND — the canLight branch's own product and the
/// placement default's own word (javap the 26.2 jar: the ctor's
/// `registerDefaultState` the LIT=true row registers, the flint's
/// `setBlock(clickedPos, state.setValue(CampfireBlock.LIT, true),
/// 11)` walk the flag-11 broadcast wears — pc 77-97 — the
/// FireChargeItem sibling the setBlockAndUpdate flag-3 keeps).
/// The "facing=south,lit=true" variant the SAME template geometry
/// resolves (block/campfire — the five-log boxes the lit_log faces
/// swap, PLUS the two ±45° rotY rescale fire planes the `#fire`
/// ref block/campfire_fire the strip's own frametime-2 animation
/// plays). The lightLevel litBlockEmission(15) the lit row's 15
/// the emission table answers — the light grid the set walk's
/// incremental flood bakes. The entityInside walk the body's dose
/// lanes read (javap CampfireBlock.entityInside: the LIT verdict
/// + the LivingEntity gate the `hurt(damageSources.campfire(),
/// fireDamage)` call serves — the fireDamage int the ctor's (true,
/// 1) row keeps, the plain fire's 1.0 dose the shared walk speaks).
/// The name the dig/drop/sound lanes resolve the BASE "campfire"
/// row keeps (the shared-row law the candle pair wears — the lit
/// swap the lanes' own product never places).
pub const CAMPFIRE_LIT: u16 = 62;

/// Kind 63 (the sixty-fourth entry, the same round): THE UNLIT SOUL
/// CAMPFIRE'S OWN KIND — the douse lane's own soul product (javap
/// the 26.2 jar: the blocks clinit's lambda$static$311 arm the
/// `new CampfireBlock(false, 2, props)` ctor call banks — the
/// spawnParticles=false/fireDamage=2 pair the soul's own rows
/// keep, the properties chain the SAME strength/sound/lightLevel
/// shape the plain row speaks). The blockstate's lit=false rows
/// the SAME block/campfire_off model the plain kind points (the
/// unlit soul the plain-log look the vanilla itself renders, the
/// soul tint only the lit masters' pixels carry). The canLight
/// consult's soul row the two igniter arms read (the `!LIT`
/// verdict the doused kind answers), the douse arm the unlit
/// product the swap speaks. The name the dig/drop/sound lanes
/// resolve the BASE "soul_campfire" row keeps (the item id the
/// pick lane grants).
pub const SOUL_CAMPFIRE: u16 = 63;

/// Kind 64 (the sixty-fifth entry, the same round): THE LIT SOUL
/// CAMPFIRE'S OWN KIND — the canLight branch's own soul product
/// (javap the 26.2 jar: the "facing=south,lit=true" row
/// block/soul_campfire resolves — the template_campfire parent
/// the soul_campfire_fire/soul_campfire_log_lit texture overrides
/// carry, the two ±45° fire planes the soul strip's own
/// frametime-2 animation plays). The lightLevel litBlockEmission(10)
/// the lit row's 10 the emission table answers (the `bipush 10`
/// the soul registration's lightLevel call speaks). The
/// entityInside walk the body's dose lanes read — the ctor's
/// (false, 2) row the fireDamage int keeps, the soul's 2.0 dose
/// the plain's window the bypass pierces (the BaseFireBlock walk's
/// own float split the same beat wears). The name the dig/drop/
/// sound lanes resolve the BASE "soul_campfire" row keeps.
pub const SOUL_CAMPFIRE_LIT: u16 = 64;

/// Kind 65 (the sixty-sixth entry, the autopush-523 round): THE
/// UNLIT CANDLE CAKE'S OWN KIND — the canLight consult's THIRD row
/// the two igniter arms' disjunction names and the palette has
/// never banked (javap the 26.2 jar — CandleCakeBlock's ctor the
/// DEFAULT state registers `LIT=false` (the setValue(LIT, false)
/// iconst_0 walk the any() builder serves), the registration the
/// blocks clinit's BlockIds.CANDLE_CAKE row banks:
/// `ofLegacyCopy(CAKE)` the CAKE strength/sound rows the legacy
/// copy wears, `lightLevel(litBlockEmission(3))` the lit row's 3
/// the emission table answers — and the register(ResourceKey,
/// Function, Properties) walk carries NO BlockItemId: the block
/// the ITEM FORM never had, the placeable lookup the vanilla
/// itself refuses (the vanilla player the candle-on-cake use lane
/// the cake kind's own round serves). CandleCakeBlock.canLight's
/// pc walk: the CANDLE_CAKES tag consult + the `hasProperty(LIT)
/// && !getValue(LIT)` predicate (lambda$canLight$0) the offline
/// kind-only palette the UNLIT row answers (the lit kind the
/// consult refuses — the same verdict the candle/campfire rows
/// wear). THE SHAPE the static init's `Shapes.or(column(2.0, 8.0,
/// 14.0), column(14.0, 0.0, 8.0))` walk banks — the cake slab's
/// 0..8/16 band the collision table's Column(8) row answers (the
/// body walks onto the cake the 0.6-block auto-step serves), the
/// 2-px stick's 8..14 band the per-cell vocabulary never speaks
/// (the honest fold the documented gap keeps). The name the dig/
/// drop/sound lanes resolve the BASE "candle_cake" row keeps (the
/// generated block_dig/block_sounds/block_drops tables' own rows
/// the registry answers — the drops table the CANDLE item the
/// vanilla loot table speaks, the self-entry the candle wears the
/// cake never carries; the getCloneItemStack the CAKE item the
/// pick lane answers a future cake round keeps).
pub const CANDLE_CAKE: u16 = 65;

/// Kind 66 (the sixty-seventh entry, the same round): THE LIT
/// CANDLE CAKE'S OWN KIND — the canLight branch's own product
/// (javap the 26.2 jar: the "lit=true" row block/candle_cake_lit
/// resolves — the template_cake_with_candle parent the candle_lit
/// texture override carries, the same cake slab + stick + wick
/// geometry the unlit row banks). The lightLevel
/// litBlockEmission(3) the lit row's 3 the emission table answers
/// (the `iconst_3` the registration's lightLevel call speaks —
/// the dimmest of the lit family, the candle's own 3 the same
/// helper folds). The `!LIT` verdict the consult REFUSES the lit
/// kind — the fall-through the placement walk answers. The name
/// the dig/drop/sound lanes resolve the BASE "candle_cake" row
/// keeps (the shared-row law the candle pair wears — the relight
/// the lanes' own product never places; the useWithoutItem eat
/// walk the CakeBlock.eat lane a future cake round keeps).
pub const CANDLE_CAKE_LIT: u16 = 66;

/// Kind 67 (the sixty-eighth entry, the autopush-524 round): THE FULL
/// CAKE'S OWN KIND — the eat lane the useWithoutItem walk serves and
/// the palette has never banked (javap the 26.2 jar — CakeBlock's
/// ctor the DEFAULT state registers BITES=0, the registration the
/// blocks clinit's BlockItemIds.CAKE row banks: `forceSolidOn()` the
/// pc 11242 walk the pushReaction(DESTROY) rows the pc 11222/11256
/// wear, `strength(0.5f)` the ldc 0.5f the dig lane answers,
/// `sound(SoundType.WOOL)` the WOOL row the sounds table speaks —
/// and the register(BlockItemIds.CAKE, ...) walk CARRIES the item
/// id: the block the ITEM FORM HAS, the placeable lookup the reverse
/// walk answers, the pick lane the getCloneItemStack's CAKE item
/// grants). The BITES property 0..6 the SEVEN palette kinds the
/// kind-only vocabulary wears (the wheat_stageN ladder the same law
/// keeps — each kind the bite's own stage speaks). The SHAPE the
/// static init's `Block.boxes(MAX_BITES + 1, lambda$static$0)` walk
/// banks — the lambda's `Block.box(1 + bites*2, 0, 1, 15, 8, 15)`
/// the x-start walk the per-slice model boxes exactly mirror (the
/// per-cell fold the Column(8) row the cake band answers, the
/// x-narrowing the documented gap keeps). The name the dig/drop/
/// sound/item lanes resolve the BASE "cake" row the shared-row law
/// keeps (the generated block_dig/block_drops/block_sounds/item_
/// order tables' own rows the registry answers — the cake item the
/// loot table's self-entry drops, the pick and the placement the
/// same item id grants).
pub const CAKE: u16 = 67;

/// Kinds 68..73 (the sixty-ninth..seventy-fourth entries, the same
/// round): THE BITTEN STAGES' OWN KINDS — the BITES ladder the
/// kind-only vocabulary wears (javap the 26.2 jar — the blockstate's
/// "bites=N" rows block/cake_sliceN resolve, each a SINGLE box
/// element [1+2N, 0, 1]->[15, 8, 15]: the west face #inside the
/// bitten cut speaks, block/cake_inner the bank's own fourth tile
/// answers). The eat lane the ladder's own walk rides: the bites<6
/// branch `setValue(BITES, bites + 1)` the flag-3 setBlock the
/// stage's own kind swap answers (the CAKE..CAKE_SLICE6 kinds the
/// +1 walk speaks), the max-bite branch the removeBlock the AIR the
/// cell returns (javap Level.removeBlock — the pc 12 iconst_3
/// setBlock the UPDATE_NEIGHBORS broadcast keeps). The shape lambda
/// the SAME x-start walk every stage narrows — the per-cell fold the
/// Column(8) row every stage wears. The name the dig/drop/sound/
/// item lanes the BASE "cake" row the shared-row law keeps (the
/// registry_names strip the generated REGISTRY_NAMES rows speak —
/// the world.rs NAME_OF rows the cross-check asserts).
pub const CAKE_SLICE1: u16 = 68;
pub const CAKE_SLICE2: u16 = 69;
pub const CAKE_SLICE3: u16 = 70;
pub const CAKE_SLICE4: u16 = 71;
pub const CAKE_SLICE5: u16 = 72;
pub const CAKE_SLICE6: u16 = 73;
/// THE SIGNAL FIRE'S OWN KIND (the autopush-530 round): the hay bale
/// the campfire's below-cell the isSmokeSource verdict reads — javap
/// CampfireBlock pc 229-230 over the 26.2 jar: `state.is(Blocks.
/// HAY_BLOCK)` the SIGNAL_FIRE property's own law speaks (the
/// getStateForPlacement pc 43-63 the below-cell read banks, the
/// updateShape pc 34-58 the DOWN-neighbor change re-answers — the
/// offline kind-only palette the LIVE below-read the same verdict
/// answers, the property the cache the read-each-time law never
/// needs). The wire registry's own row the generated block_ids
/// (12893..12896) the real block's state range banks — the offline
/// kind the palette's own tail slot 74 the kind-only vocabulary
/// carries. The full opaque cube the cube_column model resolves (the
/// hay_block_top/side tiles the FACE_TILES row banks) — the dig/drop
/// lanes the generated hay_block rows the name lookup answers.
pub const HAY_BLOCK: u16 = 74;

/// THE TOOL WAKE'S OWN PAIR (the dev-22 round): the cobweb the
/// SWORD/SHEARS pinned rows' first palette-native consumer — javap
/// Blocks clinit pc 4221-4268 the 26.2 jar: register(BlockItemIds.
/// COBWEB, ...) the ITEM FORM HAS (the placeable lookup the reverse
/// walk answers), mapColor(WOOL) pc 4232, sound(SoundType.COBWEB)
/// pc 4238-4241, forceSolidOn() pc 4244, noCollision() pc 4247 the
/// collision's Empty row speaks, requiresCorrectToolForDrops() pc
/// 4250 the dig row's own true the bare-hand harvest the verdict
/// refuses, strength(4.0f) pc 4253-4256, pushReaction(DESTROY)
/// pc 4259-4262. The cross model the atlas's cobweb arm resolves —
/// the two diagonal planes the flowers' own path meshes (plant_offset
/// the NONE row the web never carries — no offset the plants' jitter
/// never speaks). The entityInside walk the SLOW lane keeps (javap
/// WebBlock.entityInSide — the Vec3(0.25, 0.05, 0.25) clamp the
/// living body's motion wears, the WEAVING effect the 0.5/0.25/0.5
/// wider clamp speaks) a future movement round serves. THE WHITE
/// WOOL the ColorCollection.registerBlocks walk's own member (javap
/// Blocks pc 4725-4746 — the 16 colors one lambda family): the
/// full-cube generic walk resolves (blockstates white_wool.json ""
/// -> block/white_wool parent block/cube_all), the strength(0.8f)
/// the wool lambda speaks, NO requiresCorrectToolForDrops the
/// bare-hand harvest the dig row's false wears, the fire lambda
/// bootStrap$0 the (30, 60) pair the odds tables bank (javap
/// FireBlock BootstrapMethods #1 — the WOOL ColorCollection.forEach
/// the lambda's bipush 30/bipush 60 pair keeps).
pub const COBWEB: u16 = 75;
pub const WHITE_WOOL: u16 = 76;

/// THE SPEED FACTOR WAKE'S OWN ROW (the autopush-598 round — javap
/// Blocks clinit pc 10639-10722 the 26.2 jar): register(BlockItemIds.
/// SOUL_SAND, ...) the ITEM FORM HAS (the placeable lookup the
/// reverse walk answers), mapColor(COLOR_BROWN) pc 10650,
/// instrument(COW_BELL) pc 10656, strength(0.5f) pc 10662-10664 the
/// dig row's own 0.5 the no-demand arm speaks (NO
/// requiresCorrectToolForDrops the bare-hand harvest the verdict
/// wears), **speedFactor(0.4f) pc 10667-10670** — THE SLOW THE
/// PROPERTY ITSELF CARRIES (the makeStuckInBlock family the older
/// versions wore RETIRED: javap SoulSandBlock the 26.2 jar — the
/// entityInside override the class never speaks, the SHAPE
/// `Block.column(16.0d, 0.0d, 14.0d)` pcs 11-21 the collision's
/// Column(14) row keeps — the 14/16 band the body sinks the 2px
/// wears), sound(SoundType.SOUL_SAND) pc 10673-10676, the
/// isValidSpawn/isRedstoneConductor/isViewBlocking/isSuffocating
/// predicate rows the same-lambda family keeps. The full-cube
/// generic walk resolves (blockstates soul_sand.json "" ->
/// block/soul_sand parent block/cube_all — the atlas tile the cube
/// walk banks).
pub const SOUL_SAND: u16 = 77;

/// THE HONEY WAKE'S OWN ROW (the autopush-599 round — javap Blocks
/// clinit pc 26860-26900 the 26.2 jar): register(BlockItemIds.
/// HONEY_BLOCK, ...) the ITEM FORM HAS (the placeable lookup the
/// reverse walk answers), mapColor(COLOR_ORANGE) pc 26871, **
/// speedFactor(0.4f) pc 26877-26880 + jumpFactor(0.5f) pc 26883-26885
/// ** — the generated block_factors rows the body's move tail + jump
/// fold the wake finally feeds, noOcclusion() pc 26888 the
/// non-occluding verdict keeps (the is_transparent row the mesher
/// banks — the neighbors' faces the honey against draw; the
/// suffocation conjunction the noOcclusion shape refuses — the
/// view-blocking walk the honey never admits), sound(SoundType.
/// HONEY_BLOCK) pc 26891-26894, NO strength call the instabreak dig
/// row keeps (the generated 0.0 the bootstrapped registry speaks —
/// the bare fist the instant break arms). The TWO-ELEMENT model the
/// element walk resolves (blockstates honey_block.json "" ->
/// block/honey_block: the full outer cube the #down texture carries
/// + the 1..15 inset cube the #up/#side/#down skins wear — the
/// textures' UNIFORM partial alpha the per-texel translucent arm
/// composites, the double-layer stack the back-to-front submit order
/// serves). The collision the full-cube default keeps (no
/// getCollisionShape override the class speaks — the body the full
/// top stands on).
pub const HONEY_BLOCK: u16 = 78;

/// Kind 79 (the eightieth entry, the autopush-601 round): THE FURNACE
/// itself — the dev-24 GUI lane's own block finally renders in the
/// world. javap Blocks clinit pcs 7156-7199 the 26.2 jar:
/// register(BlockItemIds.FURNACE, ...) mapColor(STONE) pc 7170,
/// instrument(BASEDRUM) pc 7173, requiresCorrectToolForDrops() pc 7179
/// (the dig row's own true the generated table speaks), strength(3.5f)
/// pc 7182-7185, `bipush 13; litBlockEmission(I)` pc 7188-7190 the
/// lightLevel row the lit state's own 13 banks (light.rs's FURNACE_LIT
/// row keeps; the unlit kind the dark 0 the else arm wears). The
/// full-cube generic walk resolves the atlas rows (blockstates
/// furnace.json's facing=north,lit=false -> block/furnace, the model
/// chain block/orientable -> block/orientable_with_bottom ->
/// block/cube the north=#front authoring keeps — the FACE_TILES row's
/// north entry the front tile banks), the FACING property the variant
/// byte carries (world_view's furnace_variant decode — 1..=4 the
/// north/east/south/west order the blockstate's y-rotation keeps),
/// the mesher's front-tile swap the facing serves (the front tile the
/// facing direction wears, the vacated north slot the side tile
/// keeps). The collision the full-cube default speaks (no
/// getCollisionShape override the FurnaceBlock class carries), the
/// occlusion the full-cube verdict keeps (the neighbors' faces the
/// furnace against cull).
pub const FURNACE: u16 = 79;

/// Kind 80 (the eighty-first entry, the autopush-601 round): THE LIT
/// FURNACE — the same furnace.json's "facing=north,lit=true" row
/// resolves (block/furnace_on, the furnace_front_on the #front ref
/// answers; the blockstate has NO furnace_lit.json — the half-kind
/// name the campfire_lit precedent wears), the `bipush 13`
/// litBlockEmission(I) pc 7188-7190 the lightLevel row the lit
/// state's own 13 banks (light.rs's row the flicker's own source
/// feeds), the shared "furnace" NAME_OF row the dig/drop/sound/item
/// lanes keep (the lit swap the smelt lane's own product never
/// places). The facing variant byte the same furnace_variant decode
/// carries, the front-tile swap the same mesher arm serves.
pub const FURNACE_LIT: u16 = 80;

/// Kind 81 (the eighty-second entry, the dev-26 round): THE BLAST
/// FURNACE — the ore-smelting family's own fast kind (javap Blocks
/// clinit pcs 24255-24298 the 26.2 jar: register(BlockItemIds.BLAST_
/// FURNACE, ...) mapColor(STONE) pc 24266-24269, instrument(BASEDRUM)
/// pc 24272-24275, requiresCorrectToolForDrops() pc 24278,
/// strength(3.5f) pc 24281-24284, bipush 13 litBlockEmission(I)
/// pc 24287-24292 — the furnace's exact property walk the same
/// constants keep). The blockstates blast_furnace.json's facing=north
/// rows resolve (block/blast_furnace — the orientable chain the
/// furnace's own #front authoring shares), the facing property the
/// variant byte carries (world_view's orientable_variant decode), the
/// front-tile swap the same mesher arm serves. The BLASTING recipes
/// the sim's own table answers (the codec default 100 the
/// BlastingRecipe clinit's `bipush 100` keeps), the fuel the HALF
/// burn duration the getBurnDuration override's `iconst_2; idiv` walk
/// speaks (javap BlastFurnaceBlockEntity pcs 0-8).
pub const BLAST_FURNACE: u16 = 81;

/// Kind 82: THE LIT BLAST FURNACE — the same blast_furnace.json's
/// "facing=north,lit=true" row resolves (block/blast_furnace_on, the
/// blast_furnace_front_on the #front ref answers — the ANIMATED strip
/// the .mcmeta banks), the `bipush 13` litBlockEmission(I) pc 24287-
/// 24292 the lightLevel row the lit state's own 13 banks, the shared
/// "blast_furnace" NAME_OF row the dig/drop/sound/item lanes keep.
/// The facing variant byte the same decode carries, the front-tile
/// swap the same mesher arm serves; the animate-tick arm the SMOKE-
/// ONLY pair the furnace's flame-less sibling wears (javap
/// BlastFurnaceBlock.animateTick — the ONE addParticle call pc 187-
/// 209, the FLAME pair the furnace speaks ABSENT).
pub const BLAST_FURNACE_LIT: u16 = 82;

/// Kind 83: THE SMOKER — the food-cooking family's own kind (javap
/// Blocks clinit pcs 24209-24252: register(BlockItemIds.SMOKER, ...)
/// mapColor(STONE) pc 24220-24223, instrument(BASEDRUM) pc 24226-
/// 24229, requiresCorrectToolForDrops() pc 24232, strength(3.5f)
/// pc 24235-24238, bipush 13 litBlockEmission(I) pc 24241-24246 —
/// the same property walk again). The blockstates smoker.json's
/// facing=north rows resolve (block/smoker — the orientable_with_
/// bottom chain the #bottom texture the smoker_bottom.png answers),
/// the facing property the variant byte carries, the front-tile swap
/// the same mesher arm serves. The SMOKING recipes the sim's own
/// table answers (the codec default 100 the SmokingRecipe clinit's
/// `bipush 100` keeps), the fuel the HALF burn duration the same
/// getBurnDuration override speaks (javap SmokerBlockEntity pcs 0-8).
pub const SMOKER: u16 = 83;

/// Kind 84: THE LIT SMOKER — the same smoker.json's
/// "facing=north,lit=true" row resolves (block/smoker_on, the
/// smoker_front_on the #front ref answers), the `bipush 13`
/// lightLevel row the lit state's own 13 banks, the shared "smoker"
/// NAME_OF row the dig/drop/sound/item lanes keep. The animate-tick
/// arm the SIMPLEST tail the trio wears (javap SmokerBlock.animateTick
/// — the sound gate draw the ONLY random the walk spends, the ONE
/// addParticle(SMOKE) at (x+0.5, y+1.1, z+0.5) pc 79-96 the
/// `ldc2_w 1.1; dadd` the +1.1 height speaks, NO facing reads, NO
/// lateral/height draws — the SMOKER_SMOKE event the pc 67 field
/// banks, "block.smoker.smoke" the registration name carries).
pub const SMOKER_LIT: u16 = 84;

/// Kind 85: THE NETHER PORTAL — the dimension gate's own kind (javap
/// Blocks clinit pc 11083-11126 over the 26.2 jar: register(
/// ResourceKey, lambda$static$218, Properties.of() — the FACTORY
/// register the BlockItemIds register overload never wears, the
/// NO-ITEM block the vanilla itself refuses to grant). The props
/// chain the walk speaks: noCollision() pc 11091 the pass-through
/// collision row keeps, randomTicks() pc 11094, strength(-1.0f)
/// pc 11097-11100 the unbreakable -1 the dig row's own instabreak
/// the generated table banks, sound(SoundType.GLASS) pc 11103-11106
/// the glass sounds' rows the NAME_OF row wires, the lightLevel
/// lambda the bootstrap #219 walk answers — lambda$static$143 the
/// `bipush 11; ireturn` the portal's own emission-11 row banks (the
/// purple glow the light grid paints), pushReaction(BLOCK) pc
/// 11117-11120. The blockstates nether_portal.json's axis variants
/// resolve (axis=x -> block/nether_portal_ns — the thin-in-Z plane
/// the X-EXTENDING gate the west/east neighbors' own verdict
/// derives; axis=z -> block/nether_portal_ew the thin-in-X plane),
/// the element tables both models bank (the #portal ref the
/// block/nether_portal animated strip answers — the 32-frame
/// sequential strip the mcmeta's empty animation wears), the mesher's
/// axis lane the neighbor reads the same walk the animate arm
/// speaks. The ANIMATE-TICK LANE the sampler's own arm serves (javap
/// NetherPortalBlock.animateTick over the 26.2 jar — the walk below).
pub const NETHER_PORTAL: u16 = 85;

/// THE FRAME'S OWN KIND — the obsidian the portal frame-scan lane's
/// own building block (the autopush-606 round): javap Blocks clinit
/// pc 6491-6524 the 26.2 jar — `register(BlockItemIds.OBSIDIAN,
/// of().mapColor(COLOR_BLACK) pc 6497-6500 + instrument(BASEDRUM) pc
/// 6503-6506 + requiresCorrectToolForDrops() pc 6509 +
/// strength(50.0f, 1200.0f) pc 6512-6518)` the ITEM FORM HAS (the
/// placeable lookup the reverse walk answers — the frame the player
/// BUILDS the placement lane serves), the dig row (50.0, true) the
/// generated table banks (the full-block registry's own dump the
/// runtime walk reads), the sounds' STONE family the generated rows
/// keep (the NO SoundType call the clinit arm carries — the default
/// the registration keeps), the full-cube collision the BlockBehaviour
/// default the collision's Full arm answers, the emission the
/// NO-lightLevel call the 0 keeps (the frame's own glow the PORTAL
/// kind's emission-11 row the interior alone paints). The FRAME tag
/// the scan's own test answers (javap PortalShape.lambda$static$0 —
/// the `state.is(Blocks.OBSIDIAN)` verdict the FRAME StatePredicate
/// lambda speaks, the invokedynamic #334 the static{} walk banks).
pub const OBSIDIAN: u16 = 86;

/// THE STALK'S OWN KIND (the autopush-608 round): the bamboo the sword's
/// f32::MAX instant lane the palette finally wakes — javap Blocks clinit
/// pc 23144-23212 the 26.2 jar — `register(BlockItemIds.BAMBOO,
/// of().mapColor(PLANT) pc 23155-23158 + forceSolidOn() pc 23161 +
/// randomTicks() pc 23164 + instabreak() pc 23167 OVERRIDDEN by
/// strength(1.0f) pc 23170-23171 + sound(BAMBOO) pc 23174-23177 +
/// noOcclusion() pc 23180 + dynamicShape() pc 23183 + offsetType(XZ) pc
/// 23186-23189 + ignitedByLava() pc 23192 + pushReaction(DESTROY) pc
/// 23195-23198 + isRedstoneConductor(never) pc 23201-23206)` the ITEM
/// FORM HAS — the placeable lookup the reverse walk answers, the dig
/// row (1.0, false) the generated table banks (the instabreak's own 0
/// the strength(1.0f) arm overrides — the LAST call the clinit's own
/// order speaks), the SWORD lane the generated block_sword row banks
/// (SWORD_INSTANTLY_MINES = bamboo, bamboo_sapling — the f32::MAX
/// overrideSpeed the autopush-595 round pinned, the instant mine the
/// e2e lane the held-sword press now serves), the sounds' BAMBOO family
/// the generated rows keep, the XZ offset the plant walk's own row
/// keeps (the stalk the offset the vanilla getShape's move(getOffset)
/// walk rides — the render and the collision the same seed-sliced
/// fold), the collision SHAPE_COLLISION the BambooStalkBlock static{}
/// pc 37-47 banks (javap `Block.column(3.0, 0.0, 16.0)` — the 3/16
/// stalk column the InsetColumn(16, 6) band the offset sweep
/// [6.25..9.75] contains — the half-16th the u8 inset rounds WIDE,
/// the honest cell-slab band the sweep never escapes), the emission
/// the NO-lightLevel call the 0 keeps, the skylight the
/// propagatesSkylightDown override pc 0-1 the iconst_1 TRUE keeps
/// (the stalk cell the shadow never casts), the outline the getShape
/// walk answers (leaves == LARGE ? SHAPE_LARGE : SHAPE_SMALL — javap
/// pc 0-22 the acmp branch the SHAPE_LARGE/SHAPE_SMALL statics keep,
/// the shape the getOffset move rides). The KIND-ONLY vocabulary the
/// AGE/LEAVES/STAGE properties never stores — the placement law the
/// default state (age 0, leaves NONE, stage 0) always serves: the
/// placed stalk the thin arm renders, the growth the offline client
/// never runs (the honest limitation the palette-wake test pins).
pub const BAMBOO: u16 = 87;

/// THE SAPLING'S OWN KIND (the autopush-608 round): the placement
/// law's own FIRST product — the bamboo item on plain SUPPORTS_BAMBOO
/// ground the BambooStalkBlock.getStateForPlacement pc 178-184 walk
/// serves (`getstatic Blocks.BAMBOO_SAPLING;
/// invokevirtual Block.defaultBlockState()` the return the vanilla's
/// own quirk pins — the sapling the ONLY block the plain-ground
/// placement places, the stalk the bamboo/sapling-below arms alone
/// wear). javap Blocks clinit pc 23084-23141 — `register(
/// BlockIds.BAMBOO_SAPLING, ...)` the NO-BlockItemId overload the
/// item-less block the vanilla itself grants (the FACTORY walk the
/// portal kind's own law wears), mapColor(WOOD) pc 23095-23098 +
/// forceSolidOn() pc 23101 + randomTicks() pc 23104 + instabreak() pc
/// 23107 OVERRIDDEN by strength(1.0f) pc 23113-23114 + noCollision()
/// pc 23110 the Empty collision row + sound(BAMBOO_SAPLING) pc
/// 23117-23120 + offsetType(XZ) pc 23123-23126 + ignitedByLava() pc
/// 23129 + pushReaction(DESTROY) pc 23132-23135. The dig row (1.0,
/// false) the generated table banks, the SWORD lane the same
/// SWORD_INSTANTLY_MINES row banks (the f32::MAX instant the sapling
/// cell wears too), the drops table the BAMBOO item entry keeps (the
/// sapling block the ITEM the loot serves — the item-less block the
/// dig the bamboo item drops), the cross model the tinted_cross
/// parent resolves (block/bamboo_sapling -> block/cross — the #cross
/// texture block/bamboo_stage0 the pre-colored master renders as
/// authored — the BlockColors bamboo absence the tintindex the
/// resolver never serves), the collision the noCollision() pc 23110
/// the Empty row keeps, the XZ offset the plant walk's own row keeps.
pub const BAMBOO_SAPLING: u16 = 88;

/// THE END'S OWN BUILDING BLOCK (the autopush-612 round): the island
/// stone the END world build raises — javap Blocks clinit pc 13680-
/// 13713 the 26.2 jar: `register(BlockItemIds.END_STONE, of()
/// .mapColor(MapColor.SAND).instrument(NoteBlockInstrument.BASEDRUM)
/// .requiresCorrectToolForDrops().strength(3.0f, 9.0f))` the ITEM
/// FORM HAS (the placeable lookup the reverse walk answers — the dug
/// blocks the drops table's self-entry grants). The dig row (3.0,
/// true) the generated table banks (the requiresCorrectToolForDrops()
/// pc 13698 the pickaxe-demand arm speaks — the bare hand's 3.0/100
/// crawl the correct-tool 30 divisor serves), the drops table the
/// DROPS_END_STONE self-entry keeps, the sounds' STONE family the NO
/// sound() call the clinit arm carries (the generated rows bank
/// block.stone.break / block.stone.step), the full-cube shape the
/// collision's Full default arm answers (no noCollision()/noOcclusion
/// call the clinit arm carries), the mesher's generic cube path the
/// blockstates end_stone.json's "" variant resolves (block/end_stone
/// parent block/cube_all — the one #all texture block/end_stone the
/// six faces wear, atlas tile 96 the regen banks).
pub const END_STONE: u16 = 89;

/// THE END PORTAL'S OWN ROW (the dev-29 round — the exit pedestal's
/// starry surface the special renderer lane serves; javap the blocks
/// clinit the 26.2 jar: the END_PORTAL registration the noCollision()
/// + strength(-1.0f, 3600000.0f) + lightLevel 15 the properties
/// chain wears — the generated tables the same rows bank [the dig
/// row's own -1.0 the bedrock law keeps], the blockstates
/// end_portal.json's NO-elements model the generic walk never
/// resolves, the EndPortalRenderer's own cube the starfield shader
/// draws — the mesher's sentinel tile the surface quads carry, the
/// render walk's own pass the shader law serves)
pub const END_PORTAL: u16 = 90;

/// THE SCAN'S OWN SHAPE — the PortalShape verdict the walks bank
/// (javap PortalShape's own fields the ctor pc 0-115 carries): the
/// bottom-left interior corner, the width/height the bands answer,
/// the axis the trial's own arm wears, the numPortalBlocks the
/// EMPTY-shape predicate (javap lambda$findEmptyPortalShape$0 pc
/// 0-19 — `isValid() && numPortalBlocks == 0` the verdict the lit
/// gate the re-scan never refills) reads.
pub struct PortalShapeFill {
    pub bottom_left: (i32, i32, i32),
    pub width: i32,
    pub height: i32,
    pub axis_x: bool,
    pub num_portal_blocks: i32,
}

impl PortalShapeFill {
    /// THE VALIDITY'S OWN CONJUNCTION (javap isValid pc 0-39 — the
    /// `width in [2,21] && height in [3,21]` pair the verdict keeps,
    /// MIN_WIDTH 2 / MAX_WIDTH 21 / MIN_HEIGHT 3 / MAX_HEIGHT 21 the
    /// ConstantValue attributes speak)
    pub fn is_valid(&self) -> bool {
        (2..=21).contains(&self.width) && (3..=21).contains(&self.height)
    }

    /// THE COMPLETENESS THE LIT GATE READS (javap PortalShape.
    /// isComplete pc 0-28 over the real 26.2 jar — `isValid() && num
    /// PortalBlocks == width * height` the conjunction the douse
    /// verdict consults: the intact interior the every-cell count the
    /// w*h equality keeps, the doused or never-filled cell the count
    /// the equality refuses — the updateShape's own third arm the
    /// walk's verdict reads)
    pub fn is_complete(&self) -> bool {
        self.is_valid() && self.num_portal_blocks == self.width * self.height
    }

    /// THE FILL'S OWN CELLS (javap createPortalBlocks pc 20-53 — the
    /// `betweenClosed(bottomLeft, bottomLeft + UP*(h-1) + rightDir*
    /// (w-1))` box the interior walks, the rightDir the axis arm
    /// picks [X -> WEST, Z -> SOUTH] the findAnyShape's own law
    /// keeps)
    pub fn cells(&self) -> Vec<(i32, i32, i32)> {
        let (rx, rz) = if self.axis_x { (-1, 0) } else { (0, 1) };
        let mut out = Vec::with_capacity((self.width * self.height) as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                out.push((
                    self.bottom_left.0 + rx * x,
                    self.bottom_left.1 + y,
                    self.bottom_left.2 + rz * x,
                ));
            }
        }
        out
    }
}

/// THE EMPTY PREDICATE'S OWN ROW (javap PortalShape.isEmpty pc 0-32
/// over the real 26.2 jar): `state.isAir() || state.is(BlockTags.
/// FIRE) || state.is(NETHER_PORTAL)` the three-way verdict the
/// interior cells answer — THE FIRE CELL ITSELF the scan's own start
/// the rule admits (the just-placed flame the igniter arms bank the
/// BlockTags.FIRE row the walk reads), the portal cells the interior
/// count the numPortalBlocks gate reads
pub fn portal_cell_empty(id: u16) -> bool {
    id == AIR || is_fire(id) || id == NETHER_PORTAL
}

/// THE FURNACE'S OWN FAMILY — the two kinds the palette row wakes
/// (the unlit kind the wire world's own furnaces land through, the
/// lit kind the burning state's own product the block updates wear):
/// the mesher's front-tile swap and the emission's lit-13 row the
/// same family predicate reads.
pub fn is_furnace(id: u16) -> bool {
    id == FURNACE || id == FURNACE_LIT
}

/// THE BLAST FURNACE'S OWN FAMILY (the same two-kind shape the
/// furnace's predicate keeps — the smelting family's fast lane the
/// offline sim's own table answers).
pub fn is_blast_furnace(id: u16) -> bool {
    id == BLAST_FURNACE || id == BLAST_FURNACE_LIT
}

/// THE SMOKER'S OWN FAMILY (the same two-kind shape — the food lane's
/// own cooker).
pub fn is_smoker(id: u16) -> bool {
    id == SMOKER || id == SMOKER_LIT
}

/// THE ORIENTABLE TRIO (the AbstractFurnaceBlock subclass family —
/// the facing property + the lit flip + the front-tile swap the
/// THREE registrations share): the mesher's front-tile arm and the
/// placement facing walk the shared predicate reads.
pub fn is_orientable_furnace(id: u16) -> bool {
    is_furnace(id) || is_blast_furnace(id) || is_smoker(id)
}

/// The orientable kind's own UNLIT/LIT pair — the LIT flip's world
/// walk the sim's verdict wears (the setBlock LIT swap the
/// AbstractFurnaceBlockEntity pc 369-403 speaks). The unlit kind the
/// pair's first half keeps.
pub fn orientable_pair(id: u16) -> Option<(u16, u16)> {
    if is_furnace(id) {
        Some((FURNACE, FURNACE_LIT))
    } else if is_blast_furnace(id) {
        Some((BLAST_FURNACE, BLAST_FURNACE_LIT))
    } else if is_smoker(id) {
        Some((SMOKER, SMOKER_LIT))
    } else {
        None
    }
}

/// Block -> texture-group mapping used by the real-colors LUT.
pub const NAME_OF: [&str; 90] = [
    "air", "stone", "dirt", "grass_block", "oak_log", "oak_leaves", "water", "sand",
    "granite", "diorite", "andesite", "deepslate", "cobbled_deepslate", "gravel",
    "bedrock", "coal_ore", "copper_ore", "iron_ore", "gold_ore", "redstone_ore",
    "lapis_ore", "diamond_ore", "emerald_ore", "netherrack", "pale_oak_leaves",
    "leaf_litter", "lava", "farmland", "dirt_path", "wheat",
    "wheat_stage1", "wheat_stage2", "wheat_stage3", "wheat_stage4",
    "wheat_stage5", "wheat_stage6", "wheat_stage7", "crafting_table", "tnt",
    "dandelion", "poppy", "short_grass", "bush",
    // THE DOUBLE PLANT'S SHARED ROWS: the upper kinds keep the BASE
    // name — the dig row (instabreak 0.0), the drops table (the two
    // pools the jar's identical walk answers), the sounds' grass row
    // and the item id the pick lane grants BOTH halves speak (the
    // blockstate's half property the atlas's own `_upper` palette
    // name resolves — the registry lane the base block's row wears)
    "tall_grass", "tall_grass",
    "large_fern", "large_fern",
    // THE PROVIDER'S OWN ROWS: the high_states successors and the
    // low_states tulips the provider's own list order wears — the dig
    // row (instabreak 0.0), the drops table (the self-entry the
    // survives gate answers), the sounds' grass row and the item id the
    // pick lane grants the same one-kind-one-row walk the dandelion
    // speaks
    "azure_bluet", "oxeye_daisy", "cornflower",
    "orange_tulip", "red_tulip", "pink_tulip", "white_tulip",
    "torch",
    // THE CONCRETE PAIR'S OWN ROWS (the powder the falling family's
    // third member wears, the concrete the conversion's own target
    // speaks — the white variant the vanilla's own ctor pair banks,
    // the names the item wire order's rows exactly keep: the
    // placement lane's placeable lookup the reverse walk serves)
    "white_concrete_powder", "white_concrete",
    // THE FIRE KINDS' OWN ROWS (the flint's own product and the
    // entity-inside brand the body's fire walk reads — the names the
    // generated atlas REGISTRY_NAMES rows exactly keep, the runtime
    // mapper's own verdict the networked world's fires land through)
    "fire", "soul_fire",
    // THE CANDLE PAIR'S OWN ROWS (the canLight consult's own kinds —
    // the lit kind the BASE name the shared-row law keeps: the dig
    // row (0.1 hardness), the drops table (the self-entry), the
    // sounds' CANDLE row and the item id the pick lane grants BOTH
    // states speak — the placeable lookup the reverse walk the
    // UNLIT kind's first row answers, the lit swap the igniter
    // arms' own product never places)
    "candle", "candle",
    // THE CAMPFIRE FAMILY'S OWN ROWS (the douse lane's own kinds —
    // the lit kinds the BASE names the shared-row law keeps: the dig
    // rows (2.0 hardness — the strength(2.0) the clinit arm speaks),
    // the drops tables, the sounds' WOOD rows and the item ids the
    // pick lane grants BOTH states speak — the placeable lookup the
    // reverse walk the UNLIT kinds' rows answer, the lit swaps the
    // canLight branches' own products never place)
    "campfire", "campfire",
    "soul_campfire", "soul_campfire",
    // THE CANDLE CAKE PAIR'S OWN ROWS (the canLight consult's third
    // row — the lit kind the BASE name the shared-row law keeps: the
    // dig row (the CAKE's own strength the ofLegacyCopy wears), the
    // drops table (the CANDLE item the vanilla loot table speaks —
    // the self-entry the cake never carries), the sounds' WOOL row
    // the cake's own registration banks; NO item id the vanilla
    // itself grants — the register walk carries NO BlockItemId, the
    // placeable lookup the vanilla itself refuses, the pick lane the
    // getCloneItemStack's CAKE answer a future cake round keeps)
    "candle_cake", "candle_cake",
    // THE CAKE LADDER'S OWN ROWS (the autopush-524 round — every
    // stage the BASE "cake" name the shared-row law keeps: the dig
    // row (0.5 hardness — the strength(0.5f) the clinit arm speaks),
    // the drops table (the cake item the loot table's self-entry
    // drops), the sounds' WOOL row and the item id the pick lane
    // grants EVERY stage speak — the placeable lookup the reverse
    // walk the FULL row's first position answers (the bites=0 cake
    // the item places), the bitten stages the eat lane's own kind
    // swaps never place)
    "cake", "cake", "cake", "cake", "cake", "cake", "cake",
    // THE SIGNAL FIRE'S OWN ROW (the autopush-530 round): the hay bale
    // the dig row (1.0 hardness — the strength the clinit arm speaks),
    // the drops table (the self-entry the generated hay_block rows
    // bank), the sounds' GRASS row and the item id the pick lane
    // grants — the placeable lookup the reverse walk the kind's own
    // row answers (the below-campfire placement the signal verdict
    // lights)
    "hay_block",
    // THE TOOL WAKE'S OWN PAIR (the dev-22 round): the cobweb the dig
    // row (4.0 hardness + requiresCorrectToolForDrops — the clinit's
    // strength(4.0f) pc 4253 + requiresCorrectToolForDrops() pc 4250
    // the generated rows speak), the drops table (the string entry the
    // sword/shears verdicts the gate serves), the sounds' COBWEB row
    // and the item id the placeable lookup answers; the white wool the
    // dig row (0.8 hardness, no tool demand — the bare-hand harvest
    // the requiresCorrectTool absence wears), the drops table (the
    // self-entry), the sounds' WOOL row and the item id the reverse
    // walk answers
    "cobweb", "white_wool",
    // THE SPEED FACTOR WAKE'S OWN ROW (the autopush-598 round): the
    // soul sand the dig row (0.5 hardness, no tool demand — the
    // strength(0.5f) pc 10662 the clinit arm speaks), the drops table
    // (the self-entry the generated soul_sand rows bank), the sounds'
    // SOUL_SAND row and the item id the reverse walk answers — the
    // speedFactor(0.4f) pc 10667 the body's move-tail walk reads (the
    // generated block_factors row the name-driven lane banks)
    "soul_sand",
    // THE HONEY WAKE'S OWN ROW (the autopush-599 round): the honey
    // block the dig row (the instabreak 0.0 the no-strength default
    // the bootstrapped registry speaks), the drops table (the
    // self-entry the generated honey_block rows bank), the sounds'
    // HONEY_BLOCK row and the item id the reverse walk answers — the
    // speedFactor(0.4f) + jumpFactor(0.5f) rows the body's move tail
    // + jump fold read (the generated block_factors rows the name-
    // driven lanes bank), the two-element model the mesher's element
    // walk resolves
    "honey_block",
    // THE FURNACE PAIR'S OWN ROWS (the autopush-601 round — the lit
    // kind the BASE name the shared-row law keeps: the dig row (3.5
    // hardness + requiresCorrectToolForDrops — the strength(3.5f) pc
    // 7182 + requiresCorrectToolForDrops() pc 7179 the clinit arm
    // speaks), the drops table (the self-entry the generated furnace
    // rows bank), the sounds' STONE row (no SoundType call the clinit
    // arm carries — the default the registration keeps) and the item
    // id the placeable lookup answers; BOTH kinds the same row the
    // lit swap the smelt lane's own product never places)
    "furnace", "furnace",
    // THE BLAST FURNACE / SMOKER QUARTET'S OWN ROWS (the dev-26
    // round — the lit kinds the BASE name the shared-row law the
    // furnace pair keeps: the dig row (3.5 hardness +
    // requiresCorrectToolForDrops — the strength(3.5f) pc 24281 /
    // 24235 + requiresCorrectToolForDrops() pc 24278 / 24232 the
    // clinit arms speak), the drops tables (the self-entries the
    // generated DROPS_BLAST_FURNACE / DROPS_SMOKER rows bank), the
    // sounds' STONE rows (no SoundType call the clinit arms carry)
    // and the item ids the placeable lookup answers; BOTH lit kinds
    // the same row the lit swap the smelt lanes' own products never
    // place)
    "blast_furnace", "blast_furnace",
    "smoker", "smoker",
    // THE PORTAL'S OWN ROW (the autopush-605 round): the gate the
    // dig row (the strength(-1.0f) pc 11097 the clinit arm speaks —
    // the unbreakable -1 the generated table's own row banks), the
    // drops table (the NOTHING the no-loot block the vanilla keeps),
    // the sounds' GLASS row (sound(SoundType.GLASS) pc 11103 the
    // registration speaks) and NO item id the vanilla itself grants
    // (the FACTORY register walk carries NO BlockItemId — the place-
    // able lookup the vanilla itself refuses, the frame-lighting
    // scan a future round keeps)
    "nether_portal",
    // THE FRAME'S OWN ROW (the autopush-606 round): the obsidian the
    // dig row (50.0 hardness + requiresCorrectToolForDrops — the
    // strength(50.0f, 1200.0f) pc 6512-6518 + requiresCorrectTool
    // ForDrops() pc 6509 the clinit arm speaks — the generated
    // table's own dump banks (50.0, true)), the drops table (the
    // self-entry the generated DROPS_OBSIDIAN row banks), the
    // sounds' STONE family (the NO SoundType call the clinit arm
    // carries — the default the registration keeps) and the item id
    // the placeable lookup answers (BlockItemIds.OBSIDIAN pc 6491
    // the ITEM FORM HAS — the frame the player BUILDS the placement
    // lane serves, the portal frame-scan the igniter arm's own tail
    // lights)
    "obsidian",
    // THE STALK'S OWN ROW (the autopush-608 round): the bamboo the
    // dig row (1.0 hardness, no tool demand — the instabreak() pc
    // 23167 the strength(1.0f) pc 23170-23171 arm overrides, the
    // generated table's own dump banks (1.0, false)), the SWORD lane
    // the f32::MAX instant row the block_sword table banks (the
    // SWORD_INSTANTLY_MINES tag the autopush-595 round pinned — the
    // held-sword press the instant break the e2e lane serves), the
    // drops table (the self-entry the generated DROPS_BAMBOO row
    // banks), the sounds' BAMBOO family (sound(SoundType.BAMBOO) pc
    // 23174 the registration speaks) and the item id the placeable
    // lookup answers (BlockItemIds.BAMBOO pc 23144 the ITEM FORM HAS
    // — the placement law the sapling/stalk three-way the getStateFor
    // Placement walk serves), the XZ offset the plant walk's row
    // keeps (offsetType(XZ) pc 23186), the InsetColumn(16, 6) the
    // SHAPE_COLLISION column(3.0) the sweep band contains
    "bamboo",
    // THE SAPLING'S OWN ROW (the autopush-608 round): the placement
    // law's own FIRST product (the plain-ground placement the
    // getStateForPlacement pc 178-184 walk serves — the
    // BAMBOO_SAPLING.defaultBlockState() return the vanilla's own
    // quirk pins), the dig row (1.0, false — the instabreak() pc
    // 23107 the strength(1.0f) pc 23113-23114 arm overrides), the
    // SWORD lane the same f32::MAX instant row banks, the drops
    // table (the BAMBOO item entry the loot serves — the item-less
    // block the dig the bamboo item drops), the sounds'
    // BAMBOO_SAPLING family (sound(SoundType.BAMBOO_SAPLING) pc
    // 23117) and NO item id the vanilla itself grants (the register
    // walk carries NO BlockItemId — the placeable lookup the vanilla
    // itself refuses), the XZ offset the plant walk's row keeps
    // (offsetType(XZ) pc 23123), the Empty collision the
    // noCollision() pc 23110 row keeps
    "bamboo_sapling",
    // THE END'S OWN ROW (the autopush-612 round): the island stone
    // the dig row (3.0 hardness + requiresCorrectToolForDrops — the
    // strength(3.0f, 9.0f) pc 13701-13707 + requiresCorrectTool
    // ForDrops() pc 13698 the clinit arm speaks — the generated
    // table's own dump banks (3.0, true)), the drops table (the
    // self-entry the generated DROPS_END_STONE row banks), the
    // sounds' STONE family (no SoundType call the clinit arm carries
    // — the default the registration keeps — the generated rows bank
    // block.stone.break / block.stone.step) and the item id the
    // placeable lookup answers (BlockItemIds.END_STONE pc 13680 the
    // ITEM FORM HAS — the dug island the pick lane grants)
    "end_stone",
];

/// The plains' own FLOWER FAMILY — the cross kinds the decorate pass
/// plants (the dandelion/poppy pair the flower_plain feature's default
/// and first high row wear; the provider's own successors the 47..53
/// indices bank — the high band's azure_bluet/oxeye_daisy/cornflower
/// rows and the low band's four tulips the noise port's own round
/// plants). The mesher's non-cube family the cross path meshes, the
/// collision/clip/solid lanes the noCollision law keeps.
pub fn is_flower(id: u16) -> bool {
    (DANDELION..=POPPY).contains(&id)
        || (AZURE_BLUET..=WHITE_TULIP).contains(&id)
}

/// THE WEB'S OWN FAMILY — the cross kind the sword/shears rows wake
/// (the dev-22 round): the mesher's non-occluding verdict and the
/// cross-path mesh predicate read the family, the collision's Empty
/// row the noCollision() clinit walk keeps, the plant_offset the NONE
/// row the web never carries (no jitter the plants' own render offset
/// speaks — the rigid cross the vanilla model renders).
pub fn is_web(id: u16) -> bool {
    id == COBWEB
}

/// THE BAMBOO PAIR'S OWN FAMILY (the autopush-608 round): the stalk
/// kind the multipart lane renders (the four-model age arms the
/// blockstate's own when-conditions speak) and the sapling kind the
/// cross lane renders (the tinted_cross parent the flowers' own bake
/// carries) — the placement law the sapling/stalk three-way the
/// [`bamboo_placement_kind`] walk serves, the offset the XZ rows the
/// offsetType(XZ) clinit calls keep, the sword's f32::MAX instant the
/// NAME_OF rows the dig lane wakes.
pub fn is_bamboo_stalk(id: u16) -> bool {
    id == BAMBOO
}

pub fn is_bamboo_sapling(id: u16) -> bool {
    id == BAMBOO_SAPLING
}

/// THE SUPPORTS_BAMBOO GATE'S OWN PALETTE SUBSET (javap
/// BambooStalkBlock.canSurvive pc 0-16 the 26.2 jar — the
/// `below.is(BlockTags.SUPPORTS_BAMBOO)` verdict the placement's own
/// gate refuses through; the tag the jar's own data walk banks:
/// #sand ∪ #substrate_overworld ∪ {bamboo, bamboo_sapling, gravel,
/// suspicious_gravel} — the subtags the extracted jsons speak:
/// #sand = sand, red_sand, suspicious_sand; #substrate_overworld =
/// #dirt ∪ #mud ∪ #moss_blocks ∪ #grass_blocks). The CLIENT'S OWN
/// PALETTE the tag's members the NAME_OF vocabulary speaks: sand,
/// gravel, dirt, grass_block — the red_sand/suspicious_gravel/
/// podzol/mycelium/moss/mud rows the palette never carries (the
/// honest subset the placeable kinds alone serve — the gate the
/// below cell's kind verdict reads).
pub fn supports_bamboo(id: u16) -> bool {
    matches!(
        id,
        SAND | GRAVEL | DIRT | GRASS | BAMBOO | BAMBOO_SAPLING
    )
}

/// THE PLACEMENT LAW'S OWN THREE-WAY (javap BambooStalkBlock.
/// getStateForPlacement pc 0-186 the 26.2 jar — the walk the bamboo
/// item's placement serves, the state the item's own kind NEVER is):
/// pc 0-20 the TARGET cell's fluid the non-empty refusal keeps (the
/// water the placement never arms); pc 21-43 the below cell's
/// SUPPORTS_BAMBOO verdict the gate reads (the false the pc 185
/// aconst_null refusal keeps); pc 46-73 the below BAMBOO_SAPLING
/// arm the STALK's default state (AGE=0) serves; pc 74-125 the
/// below BAMBOO arm the stalk's own state serves (the AGE the
/// below's own age inherits — `age > 0 ? 1 : 0` pc 94-105 the
/// branch walks — the kind-only vocabulary the age never stores,
/// the stalk kind the default the arm answers); pc 126-184 the
/// PLAIN-GROUND arm the BAMBOO_SAPLING.defaultBlockState() return
/// serves (the vanilla's own quirk — the sapling the only block the
/// plain-ground placement places). The stage/leaves the default
/// state's own arms keep (stage 0, leaves NONE — the growth the
/// offline client never runs). The None the caller's own refusal
/// keeps (the canPlace gate the PASS never brands).
pub fn bamboo_placement_kind(below: u16) -> Option<u16> {
    if !supports_bamboo(below) {
        return None;
    }
    Some(match below {
        BAMBOO_SAPLING | BAMBOO => BAMBOO,
        _ => BAMBOO_SAPLING,
    })
}

/// THE CANDLE PAIR'S OWN FAMILY — the canLight consult's own kinds the
/// two igniter arms read (the unlit kind the consult answers, the lit
/// kind the branch's own product the consult refuses — the vanilla
/// `!LIT` verdict the kind-only vocabulary the pair's split wears).
/// The mesher's element lane, the collision's Empty row and the
/// emission's lit-3 row the same family predicate reads.
pub fn is_candle(id: u16) -> bool {
    id == CANDLE || id == CANDLE_LIT
}

/// THE CAMPFIRE FAMILY'S OWN PREDICATE — the four kinds the douse
/// lane's consult and the mesher's element lane read (the unlit
/// pair the canLight verdict answers, the lit pair the branch's
/// own product speaks — the vanilla `!LIT` verdict the kind-only
/// vocabulary the four-way split wears).
pub fn is_campfire(id: u16) -> bool {
    id == CAMPFIRE || id == CAMPFIRE_LIT || id == SOUL_CAMPFIRE || id == SOUL_CAMPFIRE_LIT
}

/// THE CANDLE CAKE PAIR'S OWN FAMILY — the canLight consult's third
/// row the two igniter arms read (the unlit kind the consult
/// answers, the lit kind the branch's own product the consult
/// refuses — the vanilla `!LIT` verdict the kind-only vocabulary
/// the pair's split wears). The mesher's element lane, the
/// collision's Column(8) row and the emission's lit-3 row the same
/// family predicate reads.
pub fn is_candle_cake(id: u16) -> bool {
    id == CANDLE_CAKE || id == CANDLE_CAKE_LIT
}

/// THE CAKE LADDER'S OWN FAMILY — the seven bite stages the eat
/// lane's kind-swap walk and the mesher's element lane read (the
/// full kind the item places, the bitten stages the bites<6 branch's
/// own products wear — the vanilla `setValue(BITES, bites + 1)`
/// walk the kind-only vocabulary the +1 swaps wear).
pub fn is_cake(id: u16) -> bool {
    (CAKE..=CAKE_SLICE6).contains(&id)
}

/// THE EAT LANE'S OWN BITE MAP — javap `CakeBlock.eat(LevelAccessor,
/// BlockPos, BlockState, Player)` over the 26.2 jar: the `bites =
/// state.getValue(BITES)` read pc 29-42, the `bites < 6` branch pc
/// 55-59 the `setBlock(pos, state.setValue(BITES, bites + 1), 3)`
/// flag-3 step serves (the stage's own kind swap the +1 walk speaks
/// — the CAKE..CAKE_SLICE5 kinds the next stage answers), the
/// max-bite branch pc 91-106 the `removeBlock(pos, false)` walk the
/// AIR returns (javap Level.removeBlock — the pc 12 iconst_3
/// setBlock the UPDATE_NEIGHBORS broadcast keeps — the None the
/// caller's own clear wears). Every non-cake cell the PASS the None
/// keeps.
pub fn cake_bite_next(cell: u16) -> Option<u16> {
    match cell {
        CAKE => Some(CAKE_SLICE1),
        CAKE_SLICE1 => Some(CAKE_SLICE2),
        CAKE_SLICE2 => Some(CAKE_SLICE3),
        CAKE_SLICE3 => Some(CAKE_SLICE4),
        CAKE_SLICE4 => Some(CAKE_SLICE5),
        CAKE_SLICE5 => Some(CAKE_SLICE6),
        CAKE_SLICE6 => None,
        _ => None,
    }
}

/// THE DOSE'S OWN PRODUCT MAP — the lit campfire the `setValue(LIT,
/// false)` walk the shovel's douse arm serves (javap ShovelItem.useOn
/// pc 151-166 over the 26.2 jar: the `state.setValue(CampfireBlock.
/// LIT, false)` the douse's own product carries), the unlit kind the
/// swap answers (the plain and the soul pair the CAMPFIRES family
/// both speaks), the non-campfire cells the PASS the None keeps.
pub fn match_lit_campfire(cell: u16) -> Option<u16> {
    match cell {
        CAMPFIRE_LIT => Some(CAMPFIRE),
        SOUL_CAMPFIRE_LIT => Some(SOUL_CAMPFIRE),
        _ => None,
    }
}

/// THE CANLIGHT CONSULT'S OWN PRODUCT — the two igniter arms'
/// `CampfireBlock.canLight(state) || CandleBlock.canLight(state) ||
/// CandleCakeBlock.canLight(state)` disjunction the offline kind-only
/// vocabulary folds (javap the 26.2 jar: CandleBlock.canLight the
/// CANDLES tag + the `!LIT` verdict speaks; CampfireBlock.canLight
/// the CAMPFIRES tag + the `!WATERLOGGED` and `!LIT` reads — the
/// kind-only palette the WATERLOGGED property never carries, the
/// UNLIT kind the verdict answers; the CandleCakeBlock row the
/// CANDLE_CAKES tag + the `hasProperty(LIT) && !getValue(LIT)`
/// predicate the lambda$canLight$0 walk speaks — the autopush-523
/// round the cake pair the palette banks). The lit kind the branch's
/// own `setValue(LIT, true)` product carries — the LIT kinds and
/// every non-lightable cell the fall-through the placement walk
/// answers.
pub fn can_light_product(cell: u16) -> Option<u16> {
    match cell {
        CANDLE => Some(CANDLE_LIT),
        CAMPFIRE => Some(CAMPFIRE_LIT),
        SOUL_CAMPFIRE => Some(SOUL_CAMPFIRE_LIT),
        CANDLE_CAKE => Some(CANDLE_CAKE_LIT),
        _ => None,
    }
}

/// THE HIGH STATES' OWN FAMILY — the poppy first row and the three
/// successors the flower_plain provider's `high_states` list banks
/// (javap the jar's worldgen json: the [poppy, azure_bluet,
/// oxeye_daisy, cornflower] rows the Util.getRandom uniform pick
/// wears). The ids are NOT contiguous (the tuft/double-plant kinds
/// ride between) — the matches! set the provider's own list order
/// keeps.
#[cfg_attr(not(test), expect(dead_code))]
pub fn is_flower_high(id: u16) -> bool {
    matches!(id, POPPY | AZURE_BLUET | OXEYE_DAISY | CORNFLOWER)
}

/// THE LOW STATES' OWN FAMILY — the four tulips the provider's
/// `low_states` list banks (the [orange_tulip, red_tulip, pink_tulip,
/// white_tulip] rows the `d0 < threshold` arm wears — the NormalNoise
/// port's own round the walk plants, the -0.8 valley the client's
/// hash lane never carries).
#[cfg_attr(not(test), expect(dead_code))]
pub fn is_flower_low(id: u16) -> bool {
    matches!(
        id,
        ORANGE_TULIP | RED_TULIP | PINK_TULIP | WHITE_TULIP
    )
}

/// The plains' own GRASS TUFT FAMILY — the two REPLACEABLE tinted
/// cross kinds the decorate pass plants (javap the Blocks clinit's
/// `.replaceable()` rows pc 4318/4371: the short_grass/bush chains the
/// flower chains never speak). The mesher's non-cube family the same
/// cross path meshes (the tintindex-0 faces the biome fold answers),
/// the collision/clip/solid lanes the noCollision law keeps, the
/// placement-into law the BlockPlaceContext walk serves — the
/// replaceable clicked cell the placement target KEEPS (the audited
/// `getClickedPos` walk: `replaceClicked` the clicked cell rides,
/// the offset pos the non-replaceable keeps).
pub fn is_tuft(id: u16) -> bool {
    id == SHORT_GRASS || id == BUSH
}

/// THE REPLACEABLE PLANT LAW — the kinds the world's own placement
/// walk reads as give-way cells (javap BlockState.canBeReplaced over
/// the 26.2 jar: the `.replaceable()` Properties flag the
/// short_grass/bush rows wear — the vanilla set the flowers' own
/// NO-replaceable absence refuses). The fluids the caller's own
/// arms speak (the air and the water the matches! keeps) — this fn
/// is the PLANT family's verdict alone.
pub fn is_replaceable(id: u16) -> bool {
    is_tuft(id) || is_double_plant(id)
}

/// THE HORIZONTAL LOOK TABLE (javap 26.2 Direction clinit — the
/// BY_2D_DATA rows the data2d sort keeps: the ctor rows' own
/// literals speak SOUTH data2d 0, WEST 1, NORTH 2, EAST 3, the
/// `lambda$static$4` comparator the values() sort serves). The
/// values the [`crate::world_view::furnace_variant`] decode's own
/// byte vocabulary wears (north 1, east 2, south 3, west 4).
const FACING_BY_2D: [u8; 4] = [3, 4, 1, 2]; // south, west, north, east

/// THE LOOK-YAW'S OWN DECODE (javap 26.2 Direction.fromYRot pcs 0-17
/// — the `Mth.floor(yaw / 90.0 + 0.5) & 3` walk the from2DDataValue
/// read serves; Entity.getDirection pcs 0-8 the
/// `fromYRot((double) getYRot())` f2d widen wears). The &3 walk the
/// negative floor results the same two's-complement answer keeps —
/// from2DDataValue's own `Mth.abs(i % 4)` the dead shape there wears
/// (fromYRot's &3 the input 0..3 already bounds; the two folds every
/// int the same row answers). The return the FACING_BY_2D row's own
/// index speaks: 0 south, 1 west, 2 north, 3 east.
pub fn direction_index_from_y_rot(yaw_degrees: f64) -> usize {
    ((yaw_degrees / 90.0 + 0.5).floor() as i32 & 3) as usize
}

/// THE FURNACE'S OWN PLACEMENT FACING (javap 26.2 chain —
/// AbstractFurnaceBlock.getStateForPlacement pcs 0-20: the
/// `defaultBlockState().setValue(FACING, ctx.getHorizontalDirection()
/// .getOpposite())` walk the placement state's own facing speaks;
/// UseOnContext.getHorizontalDirection pcs 0-20 the
/// `player == null ? NORTH : player.getDirection()` answer wears;
/// Entity.getDirection pcs 0-8 the fromYRot yaw decode keeps;
/// getOpposite pcs 0-7 the oppositeIndex read — the horizontal four
/// the +2 &3 flip in the BY_2D table speaks (SOUTH(3).opposite =
/// NORTH(2), WEST(4).opposite = EAST(5), the data3d rows the ±1
/// pairs keep)). The client's own yaw the [`crate::world_view::
/// wire_angles`] law wears (vanilla degrees = (cam.yaw - PI) as
/// degrees — the f32 subtract the same walk keeps): the look the
/// fromYRot decode answers, the opposite flip the +2 index the
/// table serves. The byte the furnace_variant vocabulary keeps —
/// the placement lane's own stamp the [`World::set_variant`] layer
/// banks, the mesher's front-tile walk the byte consumes.
pub fn furnace_facing_byte(cam_yaw_radians: f32) -> u8 {
    let vanilla_yaw = (cam_yaw_radians - std::f32::consts::PI).to_degrees();
    let look = direction_index_from_y_rot(f64::from(vanilla_yaw));
    FACING_BY_2D[(look + 2) & 3]
}

/// The plains' own DOUBLE PLANT FAMILY — the two-cell kinds the
/// decorate pass plants and the placeAt walk pairs (javap the Blocks
/// clinit pc 17244/17297: the tall_grass/large_fern chains the
/// `.replaceable()` row the tuft family wears, the offsetType(XZ) the
/// XYZ tufts never keep). The mesher's non-cube family the same cross
/// path meshes (the tintindex-0 faces the doubleTallGrass source
/// answers), the collision/clip/solid lanes the noCollision law keeps.
pub fn is_double_plant(id: u16) -> bool {
    (TALL_GRASS..=LARGE_FERN_UPPER).contains(&id)
}

/// The LOWER HALF of the double-plant family — the cell the substrate
/// carries (the VegetationBlock.canSurvive fallback the audited
/// bytecode keeps), the kind the placeAt walk wears at the clicked
/// cell.
pub fn is_double_plant_lower(id: u16) -> bool {
    id == TALL_GRASS || id == LARGE_FERN
}

/// The UPPER HALF — the cell the below read pairs (javap canSurvive:
/// `below.is(this) && below.getValue(HALF) == LOWER` — the same-block
/// chain the bytecode speaks), the kind the setPlacedBy walk wears at
/// `pos.above()`.
pub fn is_double_plant_upper(id: u16) -> bool {
    id == TALL_GRASS_UPPER || id == LARGE_FERN_UPPER
}

/// THE TORCH'S OWN FAMILY — the ONE kind the placement's survival
/// gate, the mesher's element lane, the body's passable set and the
/// pick/dig/sound rows read (the family-fn shape the plant families
/// wear, the single member the vanilla's TorchBlock/WallTorchBlock
/// split keeps — the wall torch the separate block the palette never
/// banks, the floor torch the item's own placement lands).
pub fn is_torch(id: u16) -> bool {
    id == TORCH
}

/// THE FALLING FAMILY — the kinds the vanilla's FallingBlock registers
/// (the sand/gravel pair the falling-block entity's own factory wears;
/// the white CONCRETE POWDER the palette's own growth banks — the
/// ConcretePowderBlock extends FallingBlock the ctor's superclass
/// speaks; the red sand/anvil the palette never banks — the honest
/// absence the members keep. The SOLIDIFIED concrete the family
/// refuses: ConcreteBlock extends Block — the conversion's own target
/// the support loss the gravity never wears).
pub fn is_falling_block(id: u16) -> bool {
    id == SAND || id == GRAVEL || id == CONCRETE_POWDER
}

/// THE CAN-SOLIDIFY LAW (javap `ConcretePowderBlock.canSolidify` over
/// the 26.2 jar — the lone read the bytecode speaks: `return state.
/// getFluidState().is(FluidTags.WATER)`): the state's OWN fluid the
/// water tag answers. The client's kind-only palette the WATER kind
/// alone wears (no waterlogged states exist — the lava the water tag
/// never carries, the honest single-row verdict).
pub fn concrete_can_solidify(id: u16) -> bool {
    id == WATER
}

/// THE TOUCHES-LIQUID WALK (javap `ConcretePowderBlock.touchesLiquid`
/// over the 26.2 jar — the raw disasm's own control flow keeps): the
/// `boolean flag = false` + the `BlockPos.MutableBlockPos` scan the
/// `Direction.values()` loop wears. THE DOWN ARM'S OWN GATE the
/// loop-head read arms — `if (direction == DOWN && !canSolidify(
/// level.getBlockState(mutable))) continue;` the mutable's PRE-offset
/// cell (the pos's own state) the gate reads: the water own-cell the
/// below scan admits, the powder's own cell the gate refuses (the
/// vanilla's own famous asymmetry — the powder NEVER solidifies from
/// the water BELOW it). The shared arm per direction: the
/// `setWithOffset(pos, direction)` offset, the neighbor read, the
/// `canSolidify(neighbor) && !neighbor.isFaceSturdy(level, pos,
/// direction.getOpposite())` pair the flag arms — the FIRST armed
/// neighbor the early verdict wears (the `flag = true; break;` walk
/// the goto-return tail keeps). The water the never-sturdy verdict
/// keeps (the Empty collision shape the [`collision_shape`] table
/// speaks — the isFaceSturdy cache the liquid rows never fill); the
/// opposite-direction word the arm's own argument keeps (DOWN→UP,
/// UP→DOWN, NORTH→SOUTH, SOUTH→NORTH, WEST→EAST, EAST→WEST — the
/// Direction.values() order the enum's declaration wears).
pub fn concrete_touches_liquid(world: &World, x: i32, y: i32, z: i32) -> bool {
    let own = world.get(x, y, z);
    // THE DOWN ARM — the gate the own-cell read arms (the water own
    // cell the below scan admits), the scan the below cell's own
    // canSolidify + the never-sturdy pair wears (the sturdy read the
    // UP-face argument the vanilla's `isFaceSturdy(level, pos, UP)`
    // walk keeps)
    if concrete_can_solidify(own) {
        let below = world.get(x, y - 1, z);
        if concrete_can_solidify(below) && !matches!(collision_shape(below), CollisionShape::Full) {
            return true;
        }
    }
    for (dx, dy, dz) in [
        (0, 1, 0),   // UP — the sturdy read the DOWN face asks
        (0, 0, -1),  // NORTH — the sturdy read the SOUTH face asks
        (0, 0, 1),   // SOUTH — the sturdy read the NORTH face asks
        (-1, 0, 0),  // WEST — the sturdy read the EAST face asks
        (1, 0, 0),   // EAST — the sturdy read the WEST face asks
    ] {
        let neighbor = world.get(x + dx, y + dy, z + dz);
        if concrete_can_solidify(neighbor)
            && !matches!(collision_shape(neighbor), CollisionShape::Full)
        {
            return true;
        }
    }
    false
}

/// THE SHOULD-SOLIDIFY LAW (javap `ConcretePowderBlock.shouldSolidify`
/// over the 26.2 jar — the OR pair the bytecode's two invokes keep):
/// `return canSolidify(state) || touchesLiquid(level, pos)` — the
/// state arg the CALLER's pre-placement read carries (the
/// getStateForPlacement's clicked-state walk and the onLand's
/// captured `into` state the two callers speak — the arg the world's
/// own current kind the onLand moment already replaced). The
/// shouldSolidify's own verdict the getStateForPlacement arm (the
/// powder placed into/touching water the CONCRETE state speaks) and
/// the onLand conversion (the falling body's landing) both read.
pub fn concrete_should_solidify(world: &World, x: i32, y: i32, z: i32, state: u16) -> bool {
    concrete_can_solidify(state) || concrete_touches_liquid(world, x, y, z)
}

/// THE TORCH'S OWN SURVIVAL LAW — javap BaseTorchBlock.canSurvive
/// over the 26.2 jar: the `canSupportCenter(level, pos.below(),
/// Direction.UP)` delegate the bytecode's lone invokestatic speaks.
/// canSupportCenter's two arms (javap Block.canSupportCenter): the
/// below state's `is(BlockTags.UNSTABLE_BOTTOM_CENTER)` read answers
/// FALSE first — the tag the jar's own json values as
/// `["#minecraft:fence_gates"]`, the fence-gate family the palette
/// never banks, the arm the vocabulary's honest no-op keeps — then
/// the `isFaceSturdy(level, pos, UP, SupportType.CENTER)` verdict.
/// The CENTER-support law over the client's palette: the below cell
/// carries a FULL collision box — the body's passable set's
/// negation (the air, the fluids, the noCollision plants and the
/// torch itself the walk-through verdict refuses; the leaves the
/// full-cube collision the sturdy answer keeps, the vanilla's own
/// torch-on-leaves verdict the same law wears).
pub fn can_support_torch(below: u16) -> bool {
    below != AIR
        && below != WATER
        && below != LAVA
        && !is_wheat(below)
        && !is_flower(below)
        && !is_tuft(below)
        && !is_double_plant(below)
        && !is_torch(below)
}

/// THE COLLISION SHAPE'S OWN THREE (the vanilla `getCollisionShape`'s
/// own answers over the client's palette — javap the 26.2 jar: the
/// `BlockBehaviour.getCollisionShape` walk returns `hasCollision ?
/// state.getShape(...) : Shapes.empty()`, the `Properties.noCollision()`
/// flag the Blocks clinit rows speak — the AIR row's
/// `.replaceable().noCollision()` chain, the water/lava rows' `.liquid()`
/// pair, the cross-plant/tuft/double-plant/wheat/leaf-litter/torch rows'
/// same flag — and the two column overrides the FarmlandBlock and
/// DirtPathBlock SHAPE constants answer (`Block.column(16, 0, 15)`, the
/// 15/16 band the clinit walk pins); the rest of the palette the
/// default full-cube registration keeps).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionShape {
    /// The noCollision() family — the fluid, the cross plants, the
    /// crops, the torch, the ground cover. The vanilla
    /// `isUnobstructed`'s own shortcut answers on the empty shape
    /// BEFORE any entity walk — the placement at the player's own cell
    /// NEVER refuses them (the flower-at-your-feet law the cross
    /// plants' rows speak).
    Empty,
    /// The column family (the farmland, the dirt path): the full 0..1
    /// footprint under a 15/16-tall band — the `Block.column(16, 0, 15)`
    /// the two SHAPE constants speak.
    Column(u8),
    /// THE INSET COLUMN FAMILY (the honey block — the dev-23 round):
    /// the `Block.column(14.0, 0.0, 15.0)` the SHAPE constant speaks —
    /// the (height, inset) pair the (15, 1) honey row banks: the
    /// 15/16-tall band under a 14/16 footprint (the 1/16 the four
    /// sides shrink — javap Block.column(DDDD) the box(8-w/2, y0,
    /// 8-d/2, 8+w/2, y1, 8+d/2) walk keeps). THE INSET IS THE SLIDE'S
    /// OWN GEOMETRY: the body pressed against the VISUAL face stops at
    /// the 14/16 collision band — its box overlaps the honey CELL by
    /// exactly 1/16 — the entityInside walk the box scan speaks, the
    /// isSlidingDown margin (0.4375 + bbWidth/2) the pressed center
    /// distance (15/16 - 0.5 + 0.3 = 0.7375) the +1e-7 slack admits.
    InsetColumn(u8, u8),
    /// The full-cube family — the default `Shapes.block()` registration
    /// the collision the body's box intersects refuses.
    Full,
}

/// The block's own COLLISION shape over the palette (the rows the
/// enum's doc names — the placement's `isUnobstructed` walk consumes;
/// the RAY lane's own generic predicate the visual shapes keep, the
/// two lanes the vanilla's getShape/getCollisionShape split speaks).
pub fn collision_shape(id: u16) -> CollisionShape {
    if id == AIR
        || id == WATER
        || id == LAVA
        || is_torch(id)
        || is_flower(id)
        || is_tuft(id)
        || is_double_plant(id)
        || is_wheat(id)
        || id == LEAF_LITTER
        // THE FIRE KINDS' OWN ROW (the blocks clinit's noCollision()
        // call the fire registrations both speak — the replaceable
        // pass-through the vanilla's own shape keeps: the body walks
        // THROUGH the flame cell the entityInside walk then brands)
        || id == FIRE
        || id == SOUL_FIRE
        // THE WEB'S OWN ROW (the dev-22 round — the blocks clinit's
        // noCollision() pc 4247 the WebBlock registration speaks — the
        // body walks THROUGH the web the entityInside slow lane a
        // future round serves, the placement's obstruction the empty
        // shape never refuses)
        || is_web(id)
        // THE CANDLE PAIR'S OWN ROW (the autopush-522 round — the
        // blocks clinit's CandleBlock registration the SHAPE boxes
        // speak: the 1-candle box [7,0,7]->[9,6,9] a 2x6x2-px center
        // column the per-cell vocabulary's Empty/Column/Full arms
        // cannot speak — the Empty row the honest feel-fold keeps:
        // the vanilla body the 6-px box the auto-step walks over
        // (the 0.6-block step height the sweep instantly answers),
        // the pass-through the per-cell fold the same feel speaks;
        // the placement-obstruction refusal the coarse fold never
        // wears — the documented gap the small-shape family keeps)
        || is_candle(id)
        // THE PORTAL'S OWN ROW (the autopush-605 round — the blocks
        // clinit's noCollision() pc 11091 the NETHER_PORTAL
        // registration speaks — the body walks THROUGH the gate the
        // dimension-change walk a future round serves, the placement's
        // obstruction the empty shape never refuses)
        || id == NETHER_PORTAL
        // THE END PORTAL'S OWN ROW (the dev-29 round — the blocks
        // clinit's noCollision() the END_PORTAL registration speaks:
        // the body FALLS INTO the starry cell the touch lane arms,
        // the instant teleport the vanilla walk serves)
        || id == END_PORTAL
        // THE SAPLING'S OWN ROW (the autopush-608 round — the blocks
        // clinit's noCollision() pc 23110 the BAMBOO_SAPLING
        // registration speaks — the body walks THROUGH the cross the
        // flowers' own pass-through keeps, the placement's
        // obstruction the empty shape never refuses)
        || is_bamboo_sapling(id)
    {
        CollisionShape::Empty
    } else if is_bamboo_stalk(id) {
        // THE STALK'S OWN INSET ROW (the autopush-608 round — javap
        // BambooStalkBlock.getCollisionShape pc 0-11 the 26.2 jar: the
        // SHAPE_COLLISION static returns (the static{} pc 37-47 walk
        // banks `Block.column(3.0, 0.0, 16.0)` — the 3/16 stalk column
        // [6.5, 0, 6.5] -> [9.5, 16, 9.5] the half-16th box the
        // getOffset move rides), the InsetColumn(16, 6) band the
        // [6..10] x/z slabs speak — the half-16th the u8 inset rounds
        // WIDE, the sweep band the offset jitter's own reach
        // ([6.25..9.75] the ±0.25 lattice holds) the wider slab
        // never lets the body clip into; the height the full 16/16
        // band the column's own 0..16 walk keeps)
        CollisionShape::InsetColumn(16, 6)
    } else if id == FARMLAND || id == DIRT_PATH {
        CollisionShape::Column(15)
    } else if id == SOUL_SAND {
        // THE SOUL SAND'S OWN COLUMN (the autopush-598 round — javap
        // SoulSandBlock the 26.2 jar: `private static final VoxelShape
        // SHAPE` the clinit's `Block.column(16.0d, 0.0d, 14.0d)` pcs
        // 11-21 banks, getCollisionShape pcs 0-3 the field areturn
        // keeps): the full 0..1 footprint under the 14/16-tall band —
        // the 2px the body sinks below the cube top the feel speaks,
        // the getBlockSupportShape/getVisualShape the Shapes.block()
        // full answers keep (the support the FULL band the column's
        // own top never splits — the redstone conductor the predicate
        // row arms)
        CollisionShape::Column(14)
    } else if id == HONEY_BLOCK {
        // THE HONEY BLOCK'S OWN INSET COLUMN (the dev-23 round — javap
        // HoneyBlock the 26.2 jar: `private static final VoxelShape
        // SHAPE` the clinit's `Block.column(14.0d, 0.0d, 15.0d)` pcs
        // 11-21 banks, getCollisionShape pcs 0-3 the field areturn
        // keeps): the 15/16-tall band under a 14/16 footprint — the
        // 1/16 the four sides shrink and the top short the slide's own
        // geometry speaks (the pressed body's box the honey CELL the
        // 1/16 overlap wears — the full-cube row the 599 round kept
        // the noOcclusion() call alone answered, the shape constant
        // the jar's own clinit retires), the getBlockSupportShape the
        // BlockBehaviour default the soul sand's own precedent keeps
        // (the support the FULL band the column's own top never
        // splits — the redstone conductor the predicate row arms)
        CollisionShape::InsetColumn(15, 1)
    } else if is_candle_cake(id) || is_cake(id) {
        // THE CAKE FAMILY'S OWN COLUMN (the autopush-523/524 rounds —
        // javap CandleCakeBlock.getShape the SHAPE field returns, the
        // static init's `Shapes.or(column(2.0, 8.0, 14.0), column(14.0,
        // 0.0, 8.0))` walk banks: the cake slab's 0..8/16 band the
        // Column row's own vocabulary speaks — the body walks onto
        // the cake the 0.6-block auto-step serves, the vanilla's own
        // walk-over feel the per-cell fold keeps; the 14-px slab
        // footprint's 1-px inset and the 2-px stick's 8..14 band the
        // full-footprint fold never speaks — the documented gap the
        // small-shape family keeps). THE PLAIN CAKE the same band
        // wears (the autopush-524 round — javap CakeBlock.getShape
        // the SHAPES[bites] field returns, the static init's
        // `Block.boxes(MAX_BITES + 1, lambda$static$0)` walk banks:
        // the lambda's `Block.box(1 + bites*2, 0, 1, 15, 8, 15)` the
        // y-band 0..8 the Column row speaks, the x-start's per-bite
        // narrowing 1+2*bites the per-cell vocabulary never carries —
        // the same documented gap the per-cell fold keeps)
        CollisionShape::Column(8)
    } else if is_campfire(id) {
        // THE CAMPFIRE FAMILY'S OWN ROW (the dev-6 round — javap the
        // 26.2 jar: CampfireBlock.getShape the SHAPE field returns,
        // the static init's `Block.column(16.0, 0.0, 7.0)` walk the
        // ldc2_w 16.0/dconst_0/ldc2_w 7.0 triple banks — the full
        // 0..1 footprint under the 0..7/16 band the Column row's own
        // vocabulary speaks): the body steps onto the 7-px box (the
        // 0.6-block auto-step the sweep instantly answers), the
        // vanilla's own walk-over feel the per-cell fold keeps; the
        // lit kinds' entityInside the box overlap the dose lanes read
        // (the standing body the 7-px band tops, the walking-in body
        // the overlap brands)
        CollisionShape::Column(7)
    } else {
        CollisionShape::Full
    }
}

/// THE FLINT'S OWN BURNABLE SUBSET — javap `FireBlock` over the real
/// 26.2 jar (`/tmp/client.jar`): the static init's `setFlammable`
/// rows the ignite-odds map banks (`getIgniteOdds(state) > 0` the
/// `canBurn` verdict's own walk answers — the ONLY arm the fire's
/// survival and spread consult). The offline palette's honest rows
/// (the bytecode's own constants: OAK_LOG iconst_5/iconst_5 the 5/5
/// pair keeps, OAK_LEAVES bipush 30/60, PALE_OAK_LEAVES 30/60,
/// LEAF_LITTER 60/100, TNT 15/100, SHORT_GRASS 60/100, BUSH 60/100,
/// TALL_GRASS 60/100, LARGE_FERN 60/100, the flower family
/// DANDELION/POPPY/AZURE_BLUET/OXEYE_DAISY/CORNFLOWER/the four
/// TULIPS all 60/100 — the upper halves the SAME block the offline
/// HALF-KIND word splits, the odds the base rows speak). THE SILENCE
/// THE VERDICT KEEPS: WHEAT and CRAFTING_TABLE ride NO setFlammable
/// row (the static init's own walk the grep of the full dump answers
/// — the crop and the table the canBurn refusal wears), every stone/
/// dirt/ore/fluid kind the vocabulary speaks the same refusal keeps.
pub fn fire_can_burn(id: u16) -> bool {
    matches!(
        id,
        LOG
            | LEAVES
            | PALE_LEAVES
            | LEAF_LITTER
            | TNT
            // THE WOOL FAMILY'S OWN ROW (the dev-22 round — javap
            // FireBlock BootstrapMethods #1: the WOOL collection's
            // forEach the lambda$bootStrap$0 (30, 60) pair speaks; the
            // COBWEB the setFlammable walk's own silence keeps — no
            // Web row the grep of the full dump answers)
            | WHITE_WOOL
            | SHORT_GRASS
            | BUSH
            | TALL_GRASS
            | TALL_GRASS_UPPER
            | LARGE_FERN
            | LARGE_FERN_UPPER
    ) || is_flower(id)
}

/// THE FIRE'S OWN SURVIVAL LAW — javap `FireBlock.canSurvive(state,
/// level, pos)` (the 26.2 jar): `belowState.isFaceSturdy(level,
/// below, UP) || isValidFireLocation(level, pos)` the two arms the
/// verdict ORs. THE STURDY ARM the offline collision table answers:
/// the full-cube row the UP face the vanilla's own isFaceFull math
/// keeps (the FARMLAND/DIRT_PATH 15/16 columns the full-face
/// verdict REFUSES — the sub-one height the shape's own math speaks);
/// THE LOCATION ARM (javap `isValidFireLocation` — the
/// `Direction.values()` loop the bytecode's pc 0-52 speaks): ANY of
/// the six neighbor cells the `canBurn` odds > 0 answers — the below
/// cell the scan's own read repeats (the sturdy refusal the burnable
/// odds may still answer), the DOWN..EAST enum order the array's own
/// walk keeps.
pub fn fire_can_survive(below: u16, neighbors: [u16; 6]) -> bool {
    collision_shape(below) == CollisionShape::Full || neighbors.into_iter().any(fire_can_burn)
}

/// THE FIRE'S OWN ANIMATE-TICK VERDICT — javap `BaseFireBlock.animateTick`
/// (the 26.2 jar, the walk BOTH fire kinds inherit): the walk in order
/// speaks
/// (1) THE SOUND ARM (pc 0-69): the `nextInt(24) == 0` gate the draw
///     ALWAYS spends; when the gate passes the `playLocalSound(x+0.5,
///     y+0.5, z+0.5, FIRE_AMBIENT, BLOCKS, 1.0F + nextFloat(),
///     nextFloat() * 0.7F + 0.3F, false)` tail rides — the two float
///     draws the gate's own body keeps (the FAIL the pc 9 `ifne 72`
///     jump past speaks — no float the failed gate spends);
/// (2) THE BELOW ARM'S GATE (pc 72-106): `below = pos.below()`, the
///     `canBurn(belowState) || belowState.isFaceSturdy(level, below,
///     UP)` disjunction — the arm the TRUE answer takes, the pc 193
///     `goto 677` return its tail keeps (the side arms the skipped
///     below's else lane alone serves);
/// (3) THE BELOW ARM (pc 109-193): the THREE LARGE_SMOKE loop — per
///     iteration the THREE nextDouble draws speak `(x + d, y + d*0.5 +
///     0.5, z + d)` the fire cell's own interior the offsets fold;
/// (4) THE SIDE ARMS (pc 196-677, the below-skipped lane): each of the
///     five faces the `canBurn(sideState)` gate reads (west, east,
///     north, south, above — the bytecode's own order), the TWO
///     LARGE_SMOKE loop the passing gate keeps — per iteration the
///     three draws fold the face's own inset: west `(x + d*0.1, y + d,
///     z + d)`, east `(x + 1 - d*0.1, y + d, z + d)` (the pc 333
///     `dsub` the subtraction speaks), north `(x + d, y + d, z +
///     d*0.1)`, south `(x + d, y + d, z + 1 - d*0.1)` (the pc 555
///     dsub), above `(x + d, y + 1 - d*0.1, z + d)` (the pc 637 dsub);
/// the SOUL FIRE'S OWN VERDICT (javap `SoulFireBlock.canBurn` — the
/// `iconst_1; ireturn` the override speaks): the body the EVERY block
/// answer burns — the below arm the ALWAYS-true gate takes, the side
/// arms the never-skipped lane the walk never reaches. The canBurn
/// consults the plain flame wears read the offline [`fire_can_burn`]
/// table, the sturdy arm the offline collision table's own Full row
/// answers (the [`fire_can_survive`] walk's own vocabulary).
/// The `body` stream the block's own draws ride (the doAnimateTick's
/// thread-local the stand-in serves); the returned offsets RELATIVE to
/// the fire cell's origin (the addParticle args the caller's cell
/// folds), the smoke CTOR draws the caller's own level stream feeds
/// (the sampler rounds' established policy).
pub struct FireAnimateTick {
    /// the 1/24 gate's own answer — the voice the caller plays
    pub sound: bool,
    /// `1.0F + nextFloat()` the volume the passing gate folds (0.0 the
    /// failed gate never speaks)
    pub volume: f32,
    /// `nextFloat() * 0.7F + 0.3F` the pitch the passing gate folds
    pub pitch: f32,
    /// the LARGE_SMOKE birth offsets, `(dx, dy, dz)` relative to the
    /// cell origin — three the below arm speaks, two the passing side
    /// arm each keeps, the bytecode's own order
    pub smokes: Vec<(f64, f64, f64)>,
}

pub fn fire_animate_tick(
    soul: bool,
    below: u16,
    sides: [u16; 5], // west, east, north, south, above
    body: &mut crate::drops::Lcg,
) -> FireAnimateTick {
    // THE SOUND ARM (pc 0-69): the gate the draw always spends, the
    // two floats the passing body alone keeps
    let sound = body.next_int(24) == 0;
    let (volume, pitch) = if sound {
        (1.0 + body.next_f32(), body.next_f32() * 0.7 + 0.3)
    } else {
        (0.0, 0.0)
    };
    let mut smokes = Vec::new();
    // THE BELOW ARM'S GATE (pc 86-106): the soul fire's own verdict the
    // EVERY-block answer keeps, the plain flame's the canBurn || sturdy
    // disjunction reads
    let below_burns = soul || fire_can_burn(below);
    let below_sturdy = collision_shape(below) == CollisionShape::Full;
    if below_burns || below_sturdy {
        // THE BELOW ARM (pc 109-193): the three-draw fold per smoke,
        // the interior band the y + d*0.5 + 0.5 keeps; the pc 193 goto
        // return the side arms never reaches
        for _ in 0..3 {
            let dx = body.next_f64();
            let dy = body.next_f64();
            let dz = body.next_f64();
            smokes.push((dx, dy * 0.5 + 0.5, dz));
        }
    } else {
        // THE SIDE ARMS (pc 196-677): the five faces the bytecode's own
        // order keeps, each the canBurn gate reads, the TWO-smoke loop
        // the passing gate speaks — the face's own inset fold the
        // ±0.1d coefficient carries (the x+1/z+1/ y+1 the east/south/
        // above faces subtract from)
        const SIDE_FOLDS: [fn(f64, f64, f64) -> (f64, f64, f64); 5] = [
            // west (pc 220-267): the x + d*0.1 the dadd keeps
            |dx, dy, dz| (dx * 0.1, dy, dz),
            // east (pc 315-363): the (x+1) - d*0.1 the dsub keeps
            |dx, dy, dz| (1.0 - dx * 0.1, dy, dz),
            // north (pc 412-458): the z + d*0.1 the dadd keeps
            |dx, dy, dz| (dx, dy, dz * 0.1),
            // south (pc 507-555): the (z+1) - d*0.1 the dsub keeps
            |dx, dy, dz| (dx, dy, 1.0 - dz * 0.1),
            // above (pc 604-652): the (y+1) - d*0.1 the dsub keeps
            |dx, dy, dz| (dx, 1.0 - dy * 0.1, dz),
        ];
        for (i, &side) in sides.iter().enumerate() {
            if !fire_can_burn(side) {
                continue;
            }
            for _ in 0..2 {
                let dx = body.next_f64();
                let dy = body.next_f64();
                let dz = body.next_f64();
                smokes.push(SIDE_FOLDS[i](dx, dy, dz));
            }
        }
    }
    FireAnimateTick {
        sound,
        volume,
        pitch,
        smokes,
    }
}

/// THE PORTAL'S OWN ANIMATE-TICK VERDICT (javap `NetherPortalBlock.
/// animateTick` over the 26.2 jar — the block display tick's own lane
/// the doAnimateTick sampler feeds, the purple particles + the
/// 1-in-100 ambient voice the gate cells speak). THE SOUND ARM pc
/// 0-67: the `nextInt(100) == 0` gate the draw ALWAYS spends, the
/// playLocalSound(x+0.5, y+0.5, z+0.5, PORTAL_AMBIENT, BLOCKS, 0.5f,
/// nextFloat() * 0.4f + 0.8f, false) tail the ONE float the passing
/// body alone keeps (the volume the `ldc_w 0.5f` FIXED arg the no-
/// draw law keeps; the pitch the `nextFloat() * 0.4f + 0.8f` fold
/// pc 49-63 the [0.8, 1.2) band wears; the pc 9 ifne 68 jump the
/// failed gate past speaks — no float the failed gate spends),
/// 'block.portal.ambient' the SoundEvents ldc pc 5454-ish banks.
/// THE PARTICLE ARM pc 68-318: the FOUR iterations NO gate keeps,
/// per iteration the SEVEN-OR-EIGHT draw walk — the base trio the
/// three nextDouble draws speak (d6 = x + d, d8 = y + d, d10 = z +
/// d the raw cell folds), the velocity trio the THREE nextFloat
/// draws wear (d12/d14/d16 = (nextFloat - 0.5) * 0.5 each — the
/// dsub/dmul pair the ±0.25 band folds), the ±1 pick the
/// `nextInt(2) * 2 - 1` fold pc 176-188 keeps, then the NEIGHBOR
/// arm's ONE replacement draw: the west/east-portal verdict (the
/// `getBlockState(pos.west()).is(this) || getBlockState(pos.east())
/// .is(this)` short-circuit pc 190-217 — the X-EXTENDING gate the
/// plane's own thin axis answers) TRUE the z-arm takes (pc 220-253:
/// d10 = z + 0.5 + 0.25 * i18 the snapped z the thin-in-Z plane's
/// own mid-band keeps, d16 = nextFloat() * 2.0f * i18 the OUT-OF-
/// PLANE push the NO-centering fold speaks), FALSE the x-arm keeps
/// (pc 258-291: d6 = x + 0.5 + 0.25 * i18, d12 = nextFloat() *
/// 2.0f * i18 — the SAME folds the other thin axis serves). The
/// addParticle(PORTAL, d6, d8, d10, d12, d14, d16) tail the verdict's
/// own sextet carries (the ctor's plain stores the velocities
/// verbatim keep — PortalParticle's own spread the recompute never
/// folds). The `body` stream the block's own draws ride (the
/// doAnimateTick's thread-local the stand-in serves); the returned
/// offsets/speeds RELATIVE to the portal cell's origin (the
/// addParticle args the caller's cell folds), the ctor chains the
/// caller's own level stream feeds (the sampler rounds' established
/// policy).
pub struct PortalAnimateTick {
    /// the 1/100 gate's own answer — the voice the caller plays
    pub sound: bool,
    /// `nextFloat() * 0.4f + 0.8f` the pitch the passing gate folds
    /// (0.0 the failed gate never speaks; the volume the FIXED 0.5
    /// the caller's own constant keeps)
    pub pitch: f32,
    /// the PORTAL particle births, `(dx, dy, dz, vx, vy, vz)`
    /// relative to the cell origin — four the no-gate arm always
    /// keeps, the bytecode's own order
    pub purples: Vec<(f64, f64, f64, f64, f64, f64)>,
}

pub fn portal_animate_tick(
    west_portal: bool,
    east_portal: bool,
    body: &mut crate::drops::Lcg,
) -> PortalAnimateTick {
    // THE SOUND ARM (pc 0-67): the gate the draw always spends, the
    // one float the passing body alone keeps (the volume the FIXED
    // 0.5f arg the caller's own constant keeps)
    let sound = body.next_int(100) == 0;
    let pitch = if sound {
        body.next_f32() * 0.4 + 0.8
    } else {
        0.0
    };
    let mut purples = Vec::new();
    // THE PARTICLE ARM (pc 68-318): the four iterations the no-gate
    // walk keeps — per iteration the base trio + the velocity trio +
    // the ±1 pick + the arm's own replacement draw the bytecode's
    // order speaks
    for _ in 0..4 {
        let dx = body.next_f64();
        let dy = body.next_f64();
        let dz = body.next_f64();
        // the f2d FIRST the dsub/dmul pair keeps (the vanilla fold the
        // double-space widening wears — the f32-space subtract a
        // 25-bit mantissa the rounding error invents)
        let vx = (f64::from(body.next_f32()) - 0.5) * 0.5;
        let vy = (f64::from(body.next_f32()) - 0.5) * 0.5;
        let vz = (f64::from(body.next_f32()) - 0.5) * 0.5;
        // THE ±1 PICK (pc 176-188): `nextInt(2) * 2 - 1` the fold
        let pick = body.next_int(2) as i64 * 2 - 1;
        let pick = pick as f64;
        if west_portal || east_portal {
            // THE X-EXTENDING ARM (pc 220-253): the z snapped to the
            // plane's own mid-band, the vz the out-of-plane push
            // carries (the fconst_2 fmul + i2f fmul the f32-space
            // chain the f2d widening TAILS — the folded f32 pair the
            // exact half picks keep)
            let rz = 0.5 + 0.25 * pick;
            let vz = f64::from(body.next_f32() * 2.0f32 * pick as f32);
            purples.push((dx, dy, rz, vx, vy, vz));
        } else {
            // THE Z-EXTENDING ARM (pc 258-291): the x snapped, the vx
            // the out-of-plane push carries (the same f32-space chain
            // the f2d widening tails)
            let rx = 0.5 + 0.25 * pick;
            let vx = f64::from(body.next_f32() * 2.0f32 * pick as f32);
            purples.push((rx, dy, dz, vx, vy, vz));
        }
    }
    PortalAnimateTick { sound, pitch, purples }
}

/// THE FIRE KINDS' OWN FAMILY — the two kinds the placement's igniter
/// arms, the body's flame-branding walk, the light grid's emission rows
/// and the mesher's multipart lane read (the family-fn shape the plant
/// and torch families wear, the two members the Blocks clinit's own
/// fire registrations keep — the plain flame the flint's and the
/// charge's own product, the soul flame the soul-sand base's verdict).
pub fn is_fire(id: u16) -> bool {
    id == FIRE || id == SOUL_FIRE
}

/// THE FIRE'S OWN AGE CEILING — javap `FireBlock` the 26.2 jar: `public
/// static final int MAX_AGE` the field the AGE property's own ladder
/// caps (the 0..=15 the IntegerProperty.create("age", 0, 15) walk
/// banks), the age-15 cell the extinguish arm's own roll reads.
pub const FIRE_MAX_AGE: u8 = 15;

/// THE FIRE'S OWN SCHEDULED-TICK DELAY — javap `FireBlock.getFireTickDelay`
/// (the 26.2 jar, the static the tick's own opening draw and the
/// onPlace law's own first schedule share): `bipush 30` the base the
/// pc 0-2 pair keeps, `nextInt(10)` the window the pc 3-5 draw wears —
/// 30..=39 game ticks between a fire cell's own walks (~1.5-2s the
/// vanilla cadence speaks).
pub const FIRE_TICK_DELAY_BASE: u64 = 30;
pub const FIRE_TICK_DELAY_WINDOW: u64 = 10;

/// THE FIRE'S OWN SCHEDULED DELAY DRAW — javap `getFireTickDelay(RandomSource)`
/// verbatim: `30 + random.nextInt(10)`. The lane's own rng the caller
/// keeps (the crop lane's own stream the walk's precedent wears).
pub fn fire_tick_delay(rng: &mut crate::drops::Lcg) -> u64 {
    FIRE_TICK_DELAY_BASE + rng.next_int(FIRE_TICK_DELAY_WINDOW) as u64
}

/// THE IGNITE ODDS' OWN TABLE — javap `FireBlock.bootStrap()` over the
/// real 26.2 jar (the `setFlammable(block, ignite, burn)` pairs the
/// static init's own walk banks, the 175 rows the dump speaks): the
/// OAK_LOG 5/5 pair the iconst_5/iconst_5 keeps, the LEAVES family
/// 30/60, the LEAF_LITTER 60/100, the TNT 15/100, the flora family
/// SHORT_GRASS/BUSH/TALL_GRASS/LARGE_FERN/the flower rows all 60/100 —
/// the upper halves the SAME block the odds never split (the offline
/// HALF-KIND word the base row's own odds wear). THE SILENCE THE
/// TABLE KEEPS: WHEAT and CRAFTING_TABLE ride NO row (the canBurn
/// refusal the crop and the table wear), every stone/dirt/fluid kind
/// the same refusal keeps. THE SPREAD'S OWN READ (javap
/// `getIgniteOdds(LevelReader, BlockPos)` — the `isEmptyBlock` gate
/// the pc 0-11 pair serves FIRST, the `Direction.values()` max fold
/// pc 12-72 the SIX neighbors wear): the target cell the AIR alone
/// answers, the odds the direct neighbors' own rows speak — the
/// threshold `(odds + 40 + difficulty*7) / (age + 30)` the walk's
/// own arithmetic keeps.
pub fn fire_ignite_odds(id: u16) -> i32 {
    match id {
        LOG => 5,
        LEAVES | PALE_LEAVES => 30,
        TNT => 15,
        // THE WOOL FAMILY'S OWN ROW (the dev-22 round — javap FireBlock
        // BootstrapMethods #1: the WOOL ColorCollection.forEach the
        // lambda$bootStrap$0 body's bipush 30/bipush 60 pair keeps —
        // the carpet family the bootStrap$1 60/20 pair the separate
        // collection wears)
        WHITE_WOOL => 30,
        LEAF_LITTER | SHORT_GRASS | BUSH | TALL_GRASS | TALL_GRASS_UPPER | LARGE_FERN
        | LARGE_FERN_UPPER => 60,
        _ if is_flower(id) => 60,
        _ => 0,
    }
}

/// THE BURN ODDS' OWN TABLE — javap `FireBlock.bootStrap()` the same
/// pairs' second column (the `getBurnOdds(BlockState)` read the
/// checkBurnOut roll wears — `random.nextInt(chance) < burnOdds` the
/// pc 11-21 compare serves; the waterlogged read the map lookup's own
/// gate keeps — the offline kind-only world the fluid kinds the
/// zero rows speak): the LOG family 5/5 (the trunk the slow coal the
/// pair speaks), the LEAVES family 60/60, the LEAF_LITTER 60/100,
/// the TNT 15/100 (the 100 the prime chain's own chance wears), the
/// flora family all 60/100 — the same silence the non-flammable kinds
/// keep.
pub fn fire_burn_odds(id: u16) -> i32 {
    match id {
        LOG => 5,
        LEAVES | PALE_LEAVES => 60,
        // THE WOOL FAMILY'S OWN ROW (the same lambda's second bipush —
        // the burn column the checkBurnOut roll reads)
        WHITE_WOOL => 60,
        TNT | LEAF_LITTER | SHORT_GRASS | BUSH | TALL_GRASS | TALL_GRASS_UPPER | LARGE_FERN
        | LARGE_FERN_UPPER => 100,
        _ if is_flower(id) => 100,
        _ => 0,
    }
}

/// THE PAIR'S OWN MATE — the opposite half's kind (`Some` the family
/// alone answers): the TALL_GRASS pair (43 <-> 44) and the LARGE_FERN
/// pair (45 <-> 46) the two-cell law swaps. The dig lane's
/// preventDropFromBottomPart and the placement's orphan clear both
/// read the verdict.
pub fn double_plant_mate(id: u16) -> Option<u16> {
    match id {
        TALL_GRASS => Some(TALL_GRASS_UPPER),
        TALL_GRASS_UPPER => Some(TALL_GRASS),
        LARGE_FERN => Some(LARGE_FERN_UPPER),
        LARGE_FERN_UPPER => Some(LARGE_FERN),
        _ => None,
    }
}

/// THE PLANT'S OWN RENDER OFFSET — the per-block jitter the vanilla
/// world wears (javap the 26.2 jar: the `BlockState.getOffset(pos)`
/// walk the `BlockBehaviour$BlockStateBase.offsetFunction` field
/// serves, the `Properties.offsetType(OffsetType)` switch the two
/// lambda bodies speak — `lambda$offsetType$1` the XZ row, `lambda$
/// offsetType$0` the XYZ row, the NONE row the null fn keeps). The
/// kind rows the audited Properties chains bank: the flower family
/// the offsetType(XZ) row wears (the dandelion/poppy chains pc
/// 4836-4866 the autopush-411 audit keeps), the tuft family the
/// offsetType(XYZ) row wears (the short_grass/bush chains pc
/// 4318/4371 the autopush-412 audit keeps), the double-plant family
/// the offsetType(XZ) row wears (the tall_grass/large_fern chains pc
/// 17244/17297 the autopush-413 audit keeps — `offsetType(XZ) the
/// XYZ tufts never keep`). Every other kind the vocabulary speaks
/// (the cubes, the wheat ladder, the fluids) the NONE verdict keeps —
/// the crop's four-plane cross the cell's own center holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlantOffset {
    None,
    XZ,
    XYZ,
}

/// The kind's own offset row (the `getOffsetType()` law the enum
/// carries). The UPPER half its own cell answers — the seed the
/// Y-ZERO law hashes (the lambda's `getSeed(getX(), 0, getZ())` walk
/// the column speaks), so the pair's two cells the SAME offset wear
/// and the two-cell plant the rigid pair rides.
pub fn plant_offset_type(id: u16) -> PlantOffset {
    if is_flower(id) || is_double_plant(id) {
        PlantOffset::XZ
    } else if is_tuft(id) {
        PlantOffset::XYZ
    }
    // THE BAMBOO PAIR'S OWN ROW (the autopush-608 round — the two
    // offsetType(XZ) clinit calls the registrations both speak: the
    // stalk pc 23186-23189 the OffsetType.XZ row banks, the sapling
    // pc 23123-23126 the same row keeps — the stalk's getShape the
    // getOffset move the collision walk rides, the sapling the
    // flowers' own jitter the cross bake wears)
    else if is_bamboo_stalk(id) || is_bamboo_sapling(id) {
        PlantOffset::XZ
    } else {
        PlantOffset::None
    }
}

/// THE MTH.GETSEED LAW (javap `Mth.getSeed(III)J` — the 26.2 jar's
/// own column hash the offset lambdas and the decoration rolls ride):
/// `l = (long)(x * 3129871) ^ (z * 116129781L) ^ (long)y` — the X
/// branch the INT multiply then i2l wears (the wrapping i32 product
/// the bytecode's `imul; i2l` order keeps), the Z branch the LONG
/// multiply wears (`i2l; lmul`) — then `l = l * l * 42317861L + l *
/// 11L`, `return l >> 16` (the arithmetic shift the long keeps). The
/// constants the jar's own pool banks: 3129871 (Mth #179),
/// 116129781 (#180), 42317861 (#182), 11 (#184) — the classic values
/// the decompile chain confirms.
pub fn mth_get_seed(x: i32, y: i32, z: i32) -> i64 {
    let mut l: i64 = ((x.wrapping_mul(3129871)) as i64)
        ^ ((z as i64).wrapping_mul(116129781))
        ^ (y as i64);
    l = l
        .wrapping_mul(l)
        .wrapping_mul(42317861)
        .wrapping_add(l.wrapping_mul(11));
    l >> 16
}

/// The bit-slice fold (the lambdas' own light RNG — NO java.util.Random
/// LCG the modern client carries: the bytecade's `land; l2f; fdiv`
/// chain the 4-bit slice speaks): `k = (seed >> shift) & 15`, the
/// slice the `l2f; ldc 15.0; fdiv; f2d` walk wears — the k/15.0f32
/// division the FLOAT lane runs (the f64 promotion only after), the
/// 16-level lattice the vanilla look keeps.
fn offset_slice(seed: i64, shift: i32) -> f64 {
    (((seed >> shift) & 15) as f32 / 15.0f32) as f64
}

/// The horizontal fold (both lambda bodies' x/z rows): `clamp((v -
/// 0.5) * 0.5, -maxH, maxH)` — the `Mth.clamp(DDD)` call the
/// `getMaxHorizontalOffset()` float the bounds ride (the fneg the
/// min bound speaks), the default 0.25 the BlockBehaviour overrides
/// never carry (our plant family the defaults wear — the clamp the
/// ±0.25 lattice the slice already fits inside, the law kept for the
/// tighter rows the vocabulary may yet speak).
fn offset_h(seed: i64, shift: i32) -> f64 {
    let max_h = 0.25f32 as f64;
    let d = (offset_slice(seed, shift) - 0.5) * 0.5;
    if d < -max_h {
        -max_h
    } else if d > max_h {
        max_h
    } else {
        d
    }
}

/// THE PLANT OFFSET LAW — the Vec3 the `OffsetFunction.evaluate`
/// returns, the float triple the renderer's tesselate walk receives
/// (javap ModelBlockRenderer: the `tesselateFlat(BlockQuadOutput, FFF,
/// ...)` / `tesselateAmbientOcclusion(..., FFF, ...)` signatures the
/// offset components carry, the indy `getOffset` capture the per-block
/// lambda speaks). The XZ lambda: `(h(seed,0), 0.0, h(seed,8))` — the
/// Y the ZERO keeps. The XYZ lambda: the same x/z rows plus the
/// vertical `(v(seed,4) - 1.0) * maxV` — the dsub the [-1..0] band
/// speaks (the tuft its own cell never lifts above, the vanilla's own
/// grounded look keeps), the `getMaxVerticalOffset()` default 0.2f
/// the f2d promotion wears exactly. The seed the Y-ZERO law hashes —
/// the column the pair shares, the per-column determinism the world
/// never re-rolls.
pub fn plant_offset(id: u16, x: i32, _y: i32, z: i32) -> (f64, f64, f64) {
    match plant_offset_type(id) {
        PlantOffset::None => (0.0, 0.0, 0.0),
        PlantOffset::XZ => {
            let seed = mth_get_seed(x, 0, z);
            (offset_h(seed, 0), 0.0, offset_h(seed, 8))
        }
        PlantOffset::XYZ => {
            let seed = mth_get_seed(x, 0, z);
            let max_v = 0.2f32 as f64;
            let dy = (offset_slice(seed, 4) - 1.0) * max_v;
            (offset_h(seed, 0), dy, offset_h(seed, 8))
        }
    }
}

/// The crop's own AGE FAMILY — the wheat kinds the ladder wears
/// (WHEAT..=WHEAT_STAGE7, the ages 0..7 the blockstate property
/// speaks). Every non-cube lane the WHEAT kind taught the whole
/// family keeps: the mesher's transparent verdict, the four-plane
/// path's gate, the body/hull/mob collision passes, the spawn-box
/// scan, the clip's own generic predicate (the stalks the ray
/// targets — the outline's full-cube fold), the placeable refusal.
pub fn is_wheat(id: u16) -> bool {
    (WHEAT..=WHEAT_STAGE7).contains(&id)
}

/// The kind's own age — `Some(0..7)` the wheat family answers,
/// `None` everywhere else (the ladder the growth walk reads). The
/// lazy closure the subtraction orders (the eager `then_some` the
/// non-wheat id's own underflow would evaluate — the debug build's
/// overflow panic the lane refuses).
pub fn wheat_age(id: u16) -> Option<u8> {
    is_wheat(id).then(|| (id - WHEAT) as u8)
}

/// THE BONE MEAL'S OWN AGE JUMP — javap CropBlock.getBonemealAgeIncrease
/// over the real 26.2 jar: the `Mth.nextInt(level.getRandom(), 2, 5)`
/// ride the performBonemeal's growCrops walk speaks (iconst_2, iconst_5
/// the min/max pair the invokestatic wears), the unfold the Mth.nextInt
/// bytecode keeps — the `min >= max` guard ireturns, the live arm
/// `min + random.nextInt(max - min + 1)` the ONE draw speaks — so the
/// increase `2 + rng.next_int(4)` the uniform {2,3,4,5} the four
/// outcomes wear.
pub fn bonemeal_age_increase(rng: &mut crate::drops::Lcg) -> u8 {
    (2 + rng.next_int(4)) as u8
}

/// THE BONE MEAL'S OWN GROW WALK — javap BoneMealItem.growCrop +
/// CropBlock over the real 26.2 jar: the `instanceof BonemealableBlock`
/// gate the wheat family's own membership wears (the vocabulary's
/// other kinds the growCrop miss keeps — the water-plant arm the
/// caller's fall-through speaks, the vocabulary's no-water-plants the
/// PASS answers), the `isValidBonemealTarget` the `!isMaxAge(state)`
/// law keeps (the mature crop the false verdict the PASS speaks — NO
/// draw the stream spends, the stack never shrinks), the
/// `isBonemealSuccess` the constant true the zero-draw verdict wears
/// (iconst_1, ireturn), and the `performBonemeal`'s growCrops the
/// `Math.min(getMaxAge(), getAge(state) + getBonemealAgeIncrease())`
/// climb speaks — the `setBlock(pos, getStateForAge(new), 2)` the same
/// ladder the random tick wears (`WHEAT + age`, the ages 0..7). Returns
/// the new kind + the increase the ONE draw rode, `None` the PASS keeps.
pub fn bonemeal_grow(id: u16, rng: &mut crate::drops::Lcg) -> Option<(u16, u8)> {
    let age = wheat_age(id)?;
    if age >= 7 {
        return None; // the isValidBonemealTarget refusal — the mature PASS
    }
    let increase = bonemeal_age_increase(rng);
    let new_age = 7u8.min(age + increase);
    Some((WHEAT + new_age as u16, increase))
}

/// THE HORIZONTAL PLANE'S OWN ENUM ORDER (Direction$Plane.HORIZONTAL —
/// the faces array the shuffledCopy copies: NORTH, SOUTH, WEST, EAST
/// the enum rows keep, the plane's own declaration order the
/// stream().toList() walk wears).
pub const HORIZONTAL_ENUM: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

/// THE SUPPORTS_VEGETATION SUBSET (javap VegetationBlock.mayPlaceOn
/// over the real 26.2 jar: the `below.is(BlockTags.SUPPORTS_VEGETATION)`
/// read the canSurvive wears) — the honest rows the client's own
/// palette names: the grass_block/dirt/farmland trio the tag's
/// plains-relevant members answer, everything else the canSurvive
/// refusal keeps.
pub fn supports_vegetation(id: u16) -> bool {
    matches!(id, GRASS | DIRT | FARMLAND)
}

/// THE SPREADABLE NEIGHBOUR WALK (javap
/// BonemealableBlock.getSpreadableNeighbourPos over the real 26.2
/// jar): the direction list's own order the caller wears, the FIRST
/// `isEmptyBlock(nb) && state.canSurvive(level, nb)` neighbour the
/// Optional.of answers — the canSurvive the below-cell read keeps
/// (the SUPPORTS_VEGETATION subset the [`supports_vegetation`] law
/// banks), the exhausted walk the Optional.empty speaks. NO draw the
/// walk ever spends (the world reads alone the verdict rides).
pub fn spreadable_neighbour(
    world: &World,
    x: i32,
    y: i32,
    z: i32,
    dirs: &[(i32, i32); 4],
) -> Option<(i32, i32, i32)> {
    for (dx, dz) in dirs {
        let (nx, nz) = (x + dx, z + dz);
        if world.get(nx, y, nz) == AIR && supports_vegetation(world.get(nx, y - 1, nz)) {
            return Some((nx, y, nz));
        }
    }
    None
}

/// THE SHUFFLE'S OWN FISHER-YATES (javap Direction$Plane.shuffledCopy
/// → Util.shuffledCopy → Util.shuffle over the real 26.2 jar): the
/// faces array the copy wears, the descent `for i = size..2` the
/// `swap(i-1, nextInt(i))` body keeps — the size-4 list the THREE
/// draws spends (nextInt(4), nextInt(3), nextInt(2), that order),
/// the shuffled order the performBonemeal's placement walk wears.
pub fn shuffled_horizontal(rng: &mut crate::drops::Lcg) -> [(i32, i32); 4] {
    let mut dirs = HORIZONTAL_ENUM;
    for i in (2..=dirs.len()).rev() {
        let j = rng.next_int(i as u64) as usize;
        dirs.swap(i - 1, j);
    }
    dirs
}

/// THE GRASS BLOCK'S OWN BONE MEAL WALK — javap GrassBlock.
/// performBonemeal over the real 26.2 jar: the 128-iteration spread
/// the world's own tufts and blooms the right-press plants. THE WALK
/// (the raw disassembly's offsets bank): the origin the clicked
/// cell's `above()` speaks, the i-loop `0..128` the sipush wears, the
/// inner j-loop `0..i/16` the offset chain FOUR draws ride — x
/// `nextInt(3) - 1`, y `(nextInt(3) - 1) * nextInt(3) / 2` the
/// truncating division speaks (the `-1 / 2 = 0` row the Java
/// truncation keeps, the Rust `i32 /` the same toward-zero law), z
/// `nextInt(3) - 1`; the gates the loop-abort law keeps — the below
/// cell `is(this)` (the grass block alone wears) and the full-cube
/// refusal the isCollisionShapeFullBlock verdict speaks (the
/// [`crate::body::solid`] gate the repo's own collision vocabulary
/// answers) — the abort the WHOLE i-iteration abandons (the
/// bytecode's `continue label48` jump keeps, the walk the partial
/// steps never speaks). THE ROWS (the offsets' own order): the 1/10
/// SHORT_GRASS row reads FIRST (the offsets 156-213 — the 1/10 draw
/// the block-identity compare precedes: the walked cell the
/// SHORT_GRASS block alone wears, the bush the sibling instance the
/// compare refuses) — the fired row the walked tuft's own
/// BonemealableBlock dispatch speaks (the offsets 197/213 the
/// interface calls keep): the isValidBonemealTarget the
/// hasSpreadableNeighbourPos presence wears (the enum order's own
/// stream — NO draw the verdict spends), the performBonemeal the
/// findSpreadableNeighbourPos walk banks — the shuffledCopy the
/// Util.shuffle Fisher-Yates keeps (the size-4 faces the THREE draws
/// spend: nextInt(4), nextInt(3), nextInt(2) — the swap(i-1, j) the
/// i's descent wears), the FIRST valid neighbour the shuffled order
/// plants (the `isEmptyBlock` + the SUPPORTS_VEGETATION canSurvive
/// pair the grass/dirt/farmland subset the client's own palette
/// names gates), the `setBlockAndUpdate` the walked block's own
/// default the lambda body wears (the ONE placement the ifPresent
/// speaks); the AIR row (the
/// offsets 218-236) — the non-air cell the placement rows never see,
/// the outside-height cell the isInsideBuildHeight refusal keeps;
/// the 1/8 FLOWER row (the offset 238) — the tagged features the
/// plains list wears (the `#can_spawn_from_bone_meal` tag the eight
/// flower rows names, the biome's own list the flower_plains member
/// alone answers — the pick's `nextInt(1)` the one draw speaks), the
/// KIND the noise_threshold_provider's own word wears — the default
/// dandelion the else arm plants, the 0.33333334 high-chance third
/// the Util.getRandom pick answers (the `list.get(random.nextInt(
/// list.size()))` uniform index the provider's own high_states list
/// order wears — the poppy/azure_bluet/oxeye_daisy/cornflower rows the
/// autopush-414 round banks; the low-band tulip rows the `d0 <
/// threshold` arm the NormalNoise walk serves — the -0.8 valley the
/// noise port's own round plants, the hash lane the walk never
/// carries, the stream the walk's shape keeps); the 7/8 GRASS
/// row (the offset 315) — the GRASS_BONEMEAL placed feature the
/// VegetationFeatures.GRASS simple_block wears, the SHORT_GRASS
/// state the walk cell lands (the isEmpty filter the air gate
/// re-speaks, the canSurvive the below-is-grass walk gate keeps).
/// THE RNG: the caller's own crop stream the same source the age
/// ladder wears (the repo's mod-law stand-in convention the
/// [`crate::drops::Lcg::next_int`] speaks).
pub fn bonemeal_grass_walk(
    world: &mut World,
    x: i32,
    y: i32,
    z: i32,
    rng: &mut crate::drops::Lcg,
) {
    let (ox, oy, oz) = (x, y + 1, z);
    for i in 0..128i32 {
        let (mut px, mut py, mut pz) = (ox, oy, oz);
        let mut walked = true;
        for _ in 0..(i / 16) {
            let dx = rng.next_int(3) - 1;
            let dy = (rng.next_int(3) - 1) * rng.next_int(3) / 2;
            let dz = rng.next_int(3) - 1;
            px += dx;
            py += dy;
            pz += dz;
            // THE WALK GATES: the below the grass block wears, the
            // full cube the isCollisionShapeFullBlock refusal keeps —
            // the abort the whole i-iteration abandons
            if world.get(px, py - 1, pz) != GRASS || crate::body::solid(world, px, py, pz) {
                walked = false;
                break;
            }
        }
        if !walked {
            continue;
        }
        let at = world.get(px, py, pz);
        // THE 1/10 SHORT_GRASS ROW — the block-identity compare the
        // draw precedes, the fired row the walked tuft's own
        // BonemealableBlock dispatch wears: the isValidBonemealTarget
        // the enum-order presence speaks (NO draw the verdict
        // spends), the performBonemeal the shuffled order plants (the
        // Fisher-Yates the three draws spends, the FIRST valid
        // neighbour the shuffled order answers, the walked block's own
        // default the lambda wears)
        if at == SHORT_GRASS && rng.next_int(10) == 0 {
            if spreadable_neighbour(world, px, py, pz, &HORIZONTAL_ENUM).is_some() {
                let dirs = shuffled_horizontal(rng);
                if let Some((nx, ny, nz)) = spreadable_neighbour(world, px, py, pz, &dirs) {
                    world.set(nx, ny, nz, at);
                }
            }
        }
        // THE AIR ROW — the non-air cell the placement rows never see
        if at != AIR {
            continue;
        }
        // THE HEIGHT GATE — the isInsideBuildHeight band the vanilla
        // refuses (the y<0 arm the walk's own below-gate aborts first,
        // the top arm the outside set the honest no-op keeps)
        if py < 0 || py >= CHUNK_Y as i32 {
            continue;
        }
        // THE 1/8 FLOWER ROW vs THE 7/8 GRASS ROW (the tag pick the
        // sole member answers, the provider's own high pick the fold
        // wears)
        if rng.next_int(8) == 0 {
            let _feature = rng.next_int(1);
            if rng.next_f32() < 0.33333334 {
                // THE HIGH BAND'S OWN PICK (javap Util.getRandom over
                // the 26.2 jar: `list.get(random.nextInt(list.size()))`
                // — the uniform index the provider's own high_states
                // list order wears: poppy, azure_bluet, oxeye_daisy,
                // cornflower — the autopush-414 round the rows plant)
                world.set(
                    px,
                    py,
                    pz,
                    match rng.next_int(4) {
                        1 => AZURE_BLUET,
                        2 => OXEYE_DAISY,
                        3 => CORNFLOWER,
                        _ => POPPY,
                    },
                );
            } else {
                world.set(px, py, pz, DANDELION);
            }
        } else {
            world.set(px, py, pz, SHORT_GRASS);
        }
    }
}

/// THE GROWS_CROPS TAG (javap CropBlock.getGrowthSpeed: the 3x3
/// below-scan's `state.is(BlockTags.GROWS_CROPS)` the tag walk
/// speaks — NOT the older `is(Blocks.FARMLAND)` the symbolic fold
/// never printed). The tag json's values are EXACTLY
/// `["minecraft:farmland"]` — the sole member the offline
/// vocabulary's FARMLAND kind wears.
pub fn grows_crops(id: u16) -> bool {
    id == FARMLAND
}

/// The block interaction reach — the `Attributes.BLOCK_INTERACTION_RANGE`
/// attribute's own default (javap of the real 26.2 class: `4.5d` for
/// `attribute.name.block_interaction_range`, the entity range's 3.0
/// sibling). The block clip's own reach.
pub const BLOCK_INTERACTION_RANGE: f64 = 4.5;

/// The entity interaction reach — the `Attributes.ENTITY_INTERACTION_RANGE`
/// attribute's own default (the 26.2 clinit's own ctor args:
/// `('entity_interaction_range', ..., 3.0, 0.0, 64.0)` — the 4.5 block
/// sibling's pairing the raw dump speaks). The crosshair's entity arm
/// rides it: `LocalPlayer.filterHitResult(hit, eye, entityRange)` keeps
/// the picked entity only while its hit location stays `closerThan(eye,
/// 3.0)` — the strict compare, else the hit degrades to a MISS.
pub const ENTITY_INTERACTION_RANGE: f64 = 3.0;

/// THE FIRE WALK'S OWN FRUITS — the one-cell [`World::fire_tick` ]
/// verdict the caller's own lanes answer: `changed` the remesh+relight
/// batch speaks once (the walk's own sets the mesher's fire lane and
/// the light grid's emission rows re-speak), `new_fire` the placed
/// cells the walk's own schedule already queued (the onPlace law the
/// placement arms and the burn-out/spread arms share), `tnt_primed`
/// the burnt TNT cells the caller's own entity lane spawns (the
/// PrimedTnt the prime walk's own ctor law keeps).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FireTickOutcome {
    pub changed: bool,
    pub new_fire: Vec<(i32, i32, i32)>,
    pub tnt_primed: Vec<(i32, i32, i32)>,
}

/// One block-ray hit — the vanilla `BlockHitResult`'s own shape: the hit
/// voxel, the entry face's Direction ordinal, the exact hit location the
/// wire's cursor floats subtract from (double math, float cast), and the
/// eye-inside flag.
#[derive(Debug, Clone, PartialEq)]
pub struct ClipHit {
    /// The hit voxel's grid coordinates.
    pub pos: (i32, i32, i32),
    /// The entry face's Direction ordinal (DOWN=0, UP=1, NORTH=2,
    /// SOUTH=3, WEST=4, EAST=5 — the enum's own order, the wire's own
    /// numbers).
    pub face: u8,
    /// The exact hit point on the face plane (the wire cursor's own
    /// source).
    pub location: (f64, f64, f64),
    /// The ray started inside a solid cell (the vanilla isInside).
    pub inside: bool,
}

/// A plain data grid — derives follow the data (the join result carries a
/// World through the `Debug/Clone/PartialEq` LoginResult surface; the
/// stated sky-light factor is an f32, so `Eq` departs with it).
#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub chunks_per_side: usize,
    pub seed: u64,
    /// chunk-major: chunk = cx + cz * chunks_per_side
    pub chunks: Vec<Vec<u16>>,
    /// Optional per-cell light layer — the NETWORK world's sky/block
    /// light as the server's engine reported it (`materialize` fills it
    /// from the model's javap-proven light queries). `None` for the
    /// procedural scene: the mesher then behaves exactly as it always
    /// did (no light opinion -> the legacy look, bit for bit).
    pub light: Option<LightGrid>,
    /// The server clock's day tick — total ticks mod the day timeline's
    /// period, as the latest Set Time reported (`materialize` copies the
    /// model's clock state). `None` = the server has sent no time yet:
    /// the honest day, and every procedural/golden scene stays bit for
    /// bit. The mesher derives the vanilla lightmap's SkyFactor from
    /// this through [`crate::timeline::sky_light_factor`] (1.0 day /
    /// 0.24 night, the jar-extracted day.json keyframes).
    pub day_tick: Option<i64>,
    /// The network world's biome layer — `materialize` fills it from the
    /// model's chunk biomes plus the synced registry (the tint quads
    /// ride along). `None` for the procedural scene: the mesher keeps
    /// the baked look bit for bit.
    pub biomes: Option<BiomeLayer>,
    /// The network world's per-cell STATE-VARIANT byte — the property
    /// shape the palette kind alone cannot carry (`materialize` decodes
    /// it from the global state id through the generated per-state
    /// property table, the 14th audit). `0` = no opinion: the flat
    /// ground cover renders the full footprint (the 13th-audit look,
    /// every procedural scene). `1..=16` = leaf_litter's facing x
    /// segment_amount: `1 + facing_idx + (segment_amount - 1) * 4` with
    /// facing north/east/south/west = 0..=3 (the blockstate's y-rotation
    /// order). `None` for the procedural scene: bit for bit the previous
    /// mesh.
    pub variants: Option<Vec<Vec<u8>>>,
    /// The dimension's lightmap AmbientColor base (the shader's
    /// `max(AmbientColor, nightVisionColor)` starting color) as RAW
    /// 0..=255 channel bytes — the built-in
    /// `visual/ambient_light_color` attribute (`materialize` stamps it
    /// from the model's dimension). `None` = no opinion: the mesher
    /// reads the BYTECODE default (`EnvironmentAttributes.<clinit>`
    /// defaultValue -16777216 = 0xFF000000) — black, a no-op base — so
    /// every procedural/golden scene and every unknown dimension keeps
    /// the pre-ambient arithmetic bit for bit.
    pub ambient_light: Option<[u8; 3]>,
    /// The dimension's lightmap BlockLightTint (the shader's warm
    /// block-light cast) as RAW 0..=255 channel bytes — the stated
    /// `visual/block_light_tint` attribute (`materialize` stamps it
    /// from the model's dimension). `None` = no opinion: the mesher
    /// reads the BYTECODE default (`EnvironmentAttributes.<clinit>`
    /// defaultValue -10100 = 0xFFFFD88C = #FFD88C — the corrected
    /// blue 140; an earlier literal misread it 0xAC) — so every
    /// procedural/golden scene and every unknown dimension keeps the
    /// default-tint arithmetic (modulo the correction itself).
    pub block_light_tint: Option<[u8; 3]>,
    /// The dimension's STATED sky-light factor — the multiply-BASE the
    /// lightmap's sky term scales by (the attribute-system fold: the
    /// stated value OVERRIDES the bytecode default 1.0, the timeline's
    /// multiply track applies after). `None` = no opinion: the default
    /// 1.0 — an exact f32 identity under the track multiply, so every
    /// procedural/golden scene keeps the pre-attribute arithmetic bit
    /// for bit. The real jar's END and NETHER state **0.0** — their
    /// sky term is pinned to zero (vanilla's own verdict).
    pub sky_light_factor: Option<f32>,
    /// The dimension's STATED sky-light color — the multiply-BASE the
    /// lightmap's sky term tints by, folded UNDER the timeline's
    /// `visual/sky_light_color` track per channel in INTEGER space
    /// (the attribute-system fold: javap ARGB.multiply — the -1 white
    /// default short-circuits to the track, else per-channel
    /// `imul` / `idiv 255`). `None` = no opinion: the -1 default's
    /// shape — the track verbatim, the exact expression this mesher
    /// shipped before the stated attribute arrived, so every
    /// procedural/golden scene keeps its arithmetic bit for bit. The
    /// real jar's END (#ac60cd) and NETHER (#7a7aff) state their
    /// moonlight bases; a custom server's stated color rides the wire.
    pub sky_light_color: Option<[u8; 3]>,
    /// The dimension's STATED night-vision color — the shader's
    /// `max(AmbientColor, NightVisionColor * NightVisionFactor)` term's
    /// stated base, as RAW 0..=255 channel bytes (`materialize` stamps
    /// it from the model's dimension). `None` = no opinion: the mesher
    /// reads the BYTECODE default (`EnvironmentAttributes.<clinit>`
    /// defaultValue -6710887 = 0xFF999999 = #999999 gray) — and the
    /// night-vision FACTOR is the player's EFFECT state (NIGHT_VISION
    /// effect / conduit power + water vision, javap
    /// LightmapRenderStateExtractor), 0.0 at this client's state, so
    /// every procedural/golden scene keeps the pre-attribute arithmetic
    /// bit for bit (max(A, C·0.0) = A exactly). No vanilla dimension
    /// states it; a custom server's stated color rides the wire.
    pub night_vision_color: Option<[u8; 3]>,
    /// The night-vision FACTOR itself — the player's EFFECT state (the
    /// model's `night_vision_factor()`): `hasEffect(NIGHT_VISION)` ->
    /// 1.0 (the `GameRenderer.nightVisionScale` plateau; the fade-out
    /// flicker is the documented non-goal), else exactly 0.0 — the
    /// value every procedural/golden scene rides, so their lightmap
    /// arithmetic stays bit for bit (`max(A, C·0.0) = A` for any
    /// tint). The effect state's wire transport is the Update/Remove
    /// Mob Effect packets (the model's own-entity-filtered store).
    pub night_vision_factor: f32,
    /// THE WEATHER'S OWN LEVELS (javap `Level` — the rain/thunder state
    /// the Game Event packets keep: `setRainLevel`/`setThunderLevel` the
    /// `Mth.clamp(param, 0.0, 1.0)` pair answers, the same value BOTH
    /// the old and the current slots store). The snapshot rides the
    /// model's levels (`materialize`); `0.0` for the procedural scene —
    /// the dry world the goldens render, the sun/moon at full alpha.
    pub rain_level: f32,
    pub thunder_level: f32,
    /// THE DARKNESS EFFECT'S OWN LIGHTMAP SCALE (javap
    /// `LightmapRenderStateExtractor` pc 304-344 — the `darknessEffect
    /// Scale` the blend factor × the pulse cosine fold speaks, the
    /// shader's `color -= vec3(DarknessScale)` arm the lightmap build
    /// consumes). THE PLAYER'S OWN EFFECT STATE the world's render
    /// fold carries — the same shape the `night_vision_factor` field
    /// wears (the effect's own factor the snapshot rides); the App
    /// writes it per frame from the effect store's blend, the mesher's
    /// lightmap bake consumes. `0.0` = the no-effect floor (the fold's
    /// bit-for-bit identity — the subtraction the 0.0 no-op keeps).
    pub darkness_scale: f32,
    /// THE HOST'S OWN DANGER DIAL (the `ServerLevel.getDifficulty()`
    /// walk the F3's LocalDifficulty entry reads — the world's own
    /// `Difficulty` the integrated-server default keeps: NORMAL, the
    /// vanilla new-world screen's own default the host world banks).
    /// The procedural world wears the default; the networked world's
    /// synced-difficulty lane the F3's own `instanceof ServerLevel`
    /// gate never reads (the client level the vanilla multiplayer F3
    /// keeps silent — the row the offline/host world alone wears).
    pub difficulty: crate::difficulty::Difficulty,
    /// THE DIFFICULTY'S OWN LOCK (the `LevelData.isDifficultyLocked()`
    /// flag the `DifficultyButtons.isDifficultyLocked` walk reads —
    /// `locked || hardcore` the raw bytecode keeps, the hardcore arm
    /// the client's own worlds never wear): the lock button's own
    /// state the `setLocked` walk stores, the dial + lock pair the
    /// INACTIVE arm answers (the `!locked && permission` both actives
    /// keep). The OFFLINE world the integrated-server lane IS — the
    /// `ServerboundLockDifficultyPacket(true)` the onLockCallback walk
    /// sends the sim stores here (the confirm's Yes the flag arms);
    /// the NETWORKED world the wire's difficulty lane the net store
    /// never banks — the absent lane the honest `false` wears.
    pub difficulty_locked: bool,
    /// THE CHUNKS' OWN DWELL CLOCK (the `ChunkAccess.inhabitedTime`
    /// field the `incrementInhabitedTime` walk banks — per chunk, the
    /// ticks any player's been near). The 26.2 law the server walk
    /// keeps (`ServerChunkCache.tickSpawningChunk` → `collectSpawningChunks`
    /// → `anyPlayerCloseEnoughForSpawningInternal`):
    /// every tick, every loaded chunk whose 2D center-distance-squared
    /// to a non-spectator player is STRICTLY below 16384.0 (the 128-block
    /// radius the ldc2_w banks — `euclideanDistanceSquared` the
    /// (cx*16+8, cz*16+8) center walks) banks +1. `Some` for the
    /// locally-generated world — the client IS the host, the
    /// accumulator its own server-side lane (zeros at birth);
    /// `None` for the networked world — the server's own truth the
    /// wire never sends, the client's words never predict it (the
    /// no-opinion convention every lane field keeps).
    pub inhabited: Option<Vec<u64>>,
    /// THE FIRE'S OWN SCHEDULED-TICK QUEUE — javap `FireBlock.onPlace`
    /// (the 26.2 jar: `level.scheduleTick(pos, this, getFireTickDelay(
    /// level.getRandom()))` the pc 11-21 walk serves — the FIRST tick
    /// the placement's own set speaks) and `FireBlock.tick`'s own
    /// opening pc 0-10 pair (the re-arm EVERY walk's first draw keeps):
    /// the (due game tick, x, y, z) entries the offline world's own
    /// [`World::fire_tick` ] answers. THE SET'S OWN LAW (javap
    /// `LevelChunkTicks.schedule` — the `ticksPerPosition` Set's
    /// `add`-returns-false dedup the pc 1-10 pair serves): ONE live
    /// entry per cell — the re-arm the walk's opening draw claims, the
    /// ladder's flag-260 setBlock the onPlace schedule the slot taken
    /// DROPS (the exponential the queue never grows), the dead cell's
    /// stale entry the drain's own kind gate drops. The networked
    /// world the SERVER ticks (the same split the crop lane's own
    /// authority keeps) — the queue the offline world alone fills.
    pub fire_ticks: Vec<(u64, i32, i32, i32)>,
    /// THE CAMPFIRE'S OWN AMBIENT QUEUE (the autopush-530 round — the
    /// fire queue's own shape the BE tick's every-tick cadence wears):
    /// the (due game tick, x, y, z) entries the lit campfires the
    /// offline world tracks (javap CampfireBlockEntity.particleTick —
    /// the CLIENT block-entity ticker the loaded campfires every tick
    /// walks: the `nextFloat() < 0.11f` gate the 11% share answers,
    /// the `nextInt(2) + 2` loop the 2..3 puffs spends, the
    /// makeParticles(level, pos, SIGNAL_FIRE, false) call the signal
    /// flag the state carries). THE SET'S OWN LAW the fire queue's
    /// walk keeps: ONE live entry per cell — the set()'s own lit
    /// transition the entry pushes (due 0 — the next drain picks it
    /// up, the re-arm the walk's own tail speaks), the un-lit
    /// transition the entry drops. The re-arm EVERY tick claims — the
    /// vanilla BE the LIT gate the walk's own head reads, the queue
    /// never expires the cell stays lit. The networked world the
    /// SERVER's word rides (the same split the fire lane's own
    /// authority keeps) — the queue the offline world alone fills.
    pub campfire_ticks: Vec<(u64, i32, i32, i32)>,
    /// THE BAMBOO'S OWN FALL QUEUE (the dig-side cascade's scheduled
    /// ticks — the fire queue's own shape the walk's one-tick lag
    /// wears): the (due game tick, x, y, z) entries the UNSUPPORTED
    /// stalks the updateShape walk arms (javap BambooStalkBlock.
    /// updateShape pc 0-19 — the canSurvive verdict the failed branch
    /// `scheduleTick(pos, this, 1)` serves). THE SET'S OWN LAW the
    /// fire queue's walk keeps: ONE live entry per cell (the
    /// LevelChunkTicks' add-returns-false dedup — the re-schedule the
    /// tower's stacked broadcasts the dedup refuses), the pop's own
    /// fire the re-broadcast the next level arms. The DRAIN'S OWN
    /// VERDICT the tick law keeps (javap BambooStalkBlock.tick pc
    /// 0-15): the block-match gate the stale rows drop, the
    /// canSurvive re-check the repaired support the queue never pops
    /// (the dirt the placement's own repair the verdict answers), the
    /// pop the destroyBlock(pos, true) walk serves. The networked
    /// world the SERVER ticks (the same split the fire lane's own
    /// authority keeps) — the queue the offline world alone fills.
    pub bamboo_ticks: Vec<(u64, i32, i32, i32)>,
    /// THE FIRE'S OWN AGE LADDER — the per-cell AGE property the
    /// offline kind-only world the `FIRE`/`SOUL_FIRE` kind ids alone
    /// cannot carry (the SOUL_FIRE 58 the FIRE+age arithmetic the
    /// range collision refuses — the side-table the honest store
    /// wears): the 0..=15 ladder the walk's own roll advances
    /// (`min(15, age + nextInt(3)/2)`), the burn-out and spread arms'
    /// own `min(15, age + nextInt(5)/4)` the new cells' entries bank,
    /// the removal the entry the map drops. `None`-shaped absence =
    /// age 0 (the placement's own default the getStateForPlacement
    /// walk banks).
    pub fire_age: std::collections::HashMap<(i32, i32, i32), u8>,
    /// THE PORTAL'S OWN AXIS STORE (javap createPortalBlocks pc 24-28
    /// over the real 26.2 jar — the `NetherPortalBlock.defaultBlock
    /// State().setValue(AXIS, axis)` state the kind-only vocabulary
    /// never keeps): the fill's own trial the truth banks (true = the
    /// X-extending portal the X-trial's win wears), the updateShape's
    /// flag11 verdict the store reads. The `fire_age` precedent the
    /// side map wears; the store-less cell the west/east derive the
    /// mesher's own short-circuit answers.
    pub portal_axis: std::collections::HashMap<(i32, i32, i32), bool>,
    /// THE DIMENSION'S OWN SKY GATE (javap `DimensionType.hasSkylight`
    /// — the nether's false the light engine's sky-column pass never
    /// runs: the grid the block flood alone answers, the sky layer
    /// the zero keeps). The overworld's true the default build speaks;
    /// [`Self::new_nether`] the false the nether build banks.
    pub has_skylight: bool,
}

/// A biome cell with no opinion — the server never answered for the
/// voxel (outside the column's extent, an unsent column, an
/// unresolvable y). Never rendered as a tint:
/// [`World::face_tint`] keeps the baked look.
pub const BIOME_UNKNOWN: u32 = u32::MAX;

/// The network world's biome layer: the per-voxel resolved registry ids
/// (expanded from the wire's 4×4×4 cell grids through the model's proven
/// queries — the SAME index conventions as [`World::chunks`]) plus the
/// per-biome tint quad resolved from the synced registry table
/// (slot order: grass top override / foliage override / water color /
/// dry-foliage color — see [`crate::tint`]) and the per-biome CLIMATE
/// pair the weather renderer's precipitation pick consumes (`Biome
/// .getPrecipitationAt`'s own inputs: the `hasPrecipitation` gate and
/// the `temperature` the 0.15 snow threshold reads — the synced
/// `BiomeEntry` fields, or the built-in jar tables in the skip-sync
/// fallback; an empty vec = no climate opinion, no rain anywhere).
#[derive(Debug, Clone, PartialEq)]
pub struct BiomeLayer {
    /// per-chunk per-voxel registry biome ids; [`BIOME_UNKNOWN`] where unsent
    pub ids: Vec<Vec<u32>>,
    /// per registry id (the arrival order IS the network id): the tint
    /// quad [grass override, foliage override, water color, dry-foliage
    /// color]
    pub tints: Vec<[Option<u32>; 4]>,
    /// per registry id (the SAME arrival-order table): the climate pair
    /// the precipitation pick reads — [`BiomeClimate::NONE`] where the
    /// table never answered
    pub climates: Vec<BiomeClimate>,
}

/// The `Biome$TemperatureModifier` enum the synced entry's optional
/// string drives — `none` the identity (`TemperatureModifier$1` returns
/// the temperature unchanged), `frozen` the noise-gated 0.2F patchwork
/// (`TemperatureModifier$2`, the weather module's
/// `modify_temperature` walk). An unknown string the jar's own registry
/// codec would have refused at load (the StringRepresentable
/// fromEnum law) — the entry would never reach a vanilla client, so
/// the port's honest word maps anything but `frozen` to `None` (never
/// a guess, the no-opinion convention).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TemperatureModifier {
    /// the identity — the base temperature rides
    #[default]
    None,
    /// the frozen patchwork — the three-noise gate the 0.2F override
    /// wears
    Frozen,
}

/// The climate trio `Biome.getPrecipitationAt` consumes — the synced
/// `BiomeEntry`'s `hasPrecipitation` boolean, `temperature` float and
/// optional `temperature_modifier` string. The modifier's noise-gated
/// FROZEN patchwork (the `TemperatureModifier$2` walk: two
/// PerlinSimplexNoise samples gate a 0.2F override) rides through the
/// synced path; the built-in jar tables the generator emits predate the
/// modifier field — that fallback keeps [`TemperatureModifier::None`]
/// (the documented limitation, frozen biomes' base temperature 0.0 the
/// snow answer either way).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BiomeClimate {
    /// the synced `has_precipitation` — the gate the jar's own walk
    /// returns NONE through
    pub has_precipitation: bool,
    /// the synced `temperature` — the base the 0.15 threshold and the
    /// height adjustment read
    pub temperature: f32,
    /// the synced `temperature_modifier` — the frozen patchwork's
    /// gate key
    pub temperature_modifier: TemperatureModifier,
}

impl BiomeClimate {
    /// The no-opinion word: no precipitation, the temperature the gate
    /// never reaches (the `None` pick renders no rain).
    pub const NONE: BiomeClimate = BiomeClimate {
        has_precipitation: false,
        temperature: 0.0,
        temperature_modifier: TemperatureModifier::None,
    };
}

/// A light cell with no opinion — the server never answered for it
/// (outside the column's extent, an unsent column, an unmentioned
/// light layer). Never rendered as zero: the mesher skips unknown
/// cells in its per-corner average, and a corner with NO known cell
/// keeps the legacy unshaded look.
pub const LIGHT_UNKNOWN: u8 = 255;

/// The network world's light grid: one array per chunk (the same
/// chunk-major layout as [`World::chunks`]), each cell 0..=15 or
/// [`LIGHT_UNKNOWN`]. Sky and block are carried SEPARATELY — the day
/// combine (`max(sky, block)`) happens at mesh time, so a future
/// night renderer can re-combine with its own sky-darken factor
/// without re-materializing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrid {
    /// per-chunk sky-light cell levels; 255 = unknown
    pub sky: Vec<Vec<u8>>,
    /// per-chunk block-light cell levels; 255 = unknown
    pub block: Vec<Vec<u8>>,
}

impl LightGrid {
    fn chunk_index(&self, chunks_per_side: usize, cx: i32, cz: i32) -> Option<usize> {
        if cx < 0 || cz < 0 {
            return None;
        }
        let s = chunks_per_side as i32;
        if cx >= s || cz >= s {
            return None;
        }
        Some((cx + cz * s) as usize)
    }

    /// The cell's (sky, block) levels at world-grid coordinates —
    /// `None` per layer where unknown, both `None` outside the grid
    /// or above/below the world (the honest unknown, matching
    /// [`World::get`]'s conventions: div_euclid chunks, rem_euclid
    /// locals, y clamped to the grid's height).
    pub fn cell(&self, chunks_per_side: usize, wx: i32, wy: i32, wz: i32) -> (Option<u8>, Option<u8>) {
        if wy < 0 || wy >= CHUNK_Y as i32 {
            return (None, None);
        }
        let cx = wx.div_euclid(CHUNK_X as i32);
        let cz = wz.div_euclid(CHUNK_Z as i32);
        let Some(ci) = self.chunk_index(chunks_per_side, cx, cz) else {
            return (None, None);
        };
        let lx = wx.rem_euclid(CHUNK_X as i32) as usize;
        let lz = wz.rem_euclid(CHUNK_Z as i32) as usize;
        let idx = lx + lz * CHUNK_X + wy as usize * CHUNK_X * CHUNK_Z;
        let (s, b) = (self.sky[ci][idx], self.block[ci][idx]);
        let layer = |v: u8| (v != LIGHT_UNKNOWN).then_some(v);
        (layer(s), layer(b))
    }
}

/// Deterministic 64-bit mix (xorshift-based) — identical output on all platforms.
pub fn hash2(x: i64, z: i64, seed: u64) -> u32 {
    let mut h = seed
        ^ (x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (z as u64).rotate_left(32).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    h ^= h >> 33;
    h = h.wrapping_mul(0xFF51_AFD7_ED55_8CCD);
    h ^= h >> 33;
    h = h.wrapping_mul(0xC4CE_B9FE_1A85_EC53);
    h ^= h >> 33;
    (h >> 32) as u32
}

impl World {
    pub fn new(chunks_per_side: usize, seed: u64) -> Self {
        let mut w = Self::blank(chunks_per_side, seed);
        w.generate();
        w
    }

    /// THE NETHER'S OWN WORLD BUILD (the dimension_type's own
    /// has_skylight false the light walk's sky-column skip reads —
    /// [`crate::light::compute`] the gate consumes; the terrain the
    /// deterministic fiction raises — the bedrock floor the real
    /// dimension's y=0 keeps, the bedrock ceiling the roof the
    /// 48-tall grid scales, the netherrack rise the hash band speaks,
    /// the lava sea the ocean level the shoreline's soul sand wears,
    /// the pillars the spike roll raises).
    pub fn new_nether(chunks_per_side: usize, seed: u64) -> Self {
        let mut w = Self::blank(chunks_per_side, seed);
        w.has_skylight = false;
        w.generate_nether();
        w
    }

    /// THE END'S OWN WORLD BUILD (the autopush-612 round — the
    /// dimension_type's own the_end row the registry banks, the
    /// deterministic fiction the nether build's own shape wears): the
    /// has_skylight TRUE the 26.2 jar row keeps (javap the
    /// `data/minecraft/dimension_type/the_end.json` — the
    /// `"has_skylight": true` pair the sky-column pass RUNS, the
    /// stated `visual/sky_light_factor: 0.0` the lightmap's sky term
    /// the mesher's multiply pins to zero — the grid fills, the
    /// brightness the zero factor never feeds), the sky_light_factor
    /// Some(0.0) the stated row stamps, the sky_light_color #ac60cd
    /// the `visual/sky_light_color` row carries, the ambient_light
    /// #3f473f the `visual/ambient_light_color` row stamps (the
    /// lightmap's own base the [63, 71, 63] bytes speak). THE VOID
    /// the dimension's own law: NO bedrock floor — the island the
    /// end_stone disc the center raises, the rim the taper thins,
    /// everything below AIR (the body's fall the void damage walk a
    /// future round arms); the obsidian pillars the ring raises (the
    /// dragon fight's own perches the scaled ring keeps); the exit
    /// pedestal the bedrock base keeps (the fountain's own shape the
    /// center wears — the END_PORTAL surface the special renderer
    /// lane a future round serves, the blockstates end_portal.json's
    /// NO-elements model the generic walk cannot resolve).
    /// THE END'S OWN WORLD BUILD (the swap's END arm the destination
    /// builds — the [`crate::screens::App::swap_dimension_to`] walk
    /// the far slot's fresh build answers)
    pub fn new_end(chunks_per_side: usize, seed: u64) -> Self {
        let mut w = Self::blank(chunks_per_side, seed);
        w.sky_light_factor = Some(0.0); // the jar's stated 0.0 — the sky term pinned
        w.sky_light_color = Some([172, 96, 205]); // #ac60cd the stated moonlight base
        w.ambient_light = Some([63, 71, 63]); // #3f473f the stated ambient base
        w.generate_end();
        w
    }

    /// THE END'S OWN TERRAIN (the client's deterministic stand-in the
    /// PortalForcer-less jar keeps): the end_stone island the disc
    /// the center raises (the top END_ISLAND_TOP the 48-tall grid
    /// scales — the real dimension's y≈64 surface the proportion
    /// keeps), the depth the rim taper thins (the inverted cone the
    /// real island's own underside speaks), the obsidian pillars the
    /// ring raises (the real fight's radius-43 ring the scaled band
    /// keeps, the heights the hash roll varies), the exit pedestal
    /// the bedrock base the center wears (the fountain's own rows —
    /// the return walk's own site the future portal lane lights).
    /// No skylight law here — [`Self::new_end`] the factor 0.0
    /// stamps, the lightmap the mesher's multiply pins.
    fn generate_end(&mut self) {
        let n = self.size_blocks();
        let top = END_ISLAND_TOP;
        let cx = (n - 1) as f64 / 2.0;
        let cz = cx;
        let radius = n as f64 / 4.0;
        for wx in 0..n {
            for wz in 0..n {
                let dx = wx as f64 - cx;
                let dz = wz as f64 - cz;
                let dist = (dx * dx + dz * dz).sqrt();
                // THE RIM'S OWN TAPER (the island's own edge the hash
                // jitter roughens — the deterministic roll the
                // overworld tree roll's own hash lane keeps)
                let jitter =
                    (hash2(wx as i64, wz as i64, self.seed ^ 0x3E4D) % 3) as f64 - 1.0;
                if dist > radius + jitter {
                    continue; // the void the rim never fills
                }
                // THE DEPTH'S OWN CONE (the center's full depth the
                // rim's zero keeps — the underside the taper speaks)
                let depth =
                    ((radius - dist) / radius * END_ISLAND_DEPTH as f64).ceil() as i32;
                let bottom = (top - depth).max(1);
                for y in bottom..=top {
                    self.set(wx, y, wz, END_STONE);
                }
            }
        }
        // THE PILLAR RING (the real fight's 10 pillars the scaled
        // ring keeps — the radius the island's own band speaks, the
        // heights the hash roll varies, the 2x2 columns the
        // proportion wears; the island's own TOP the ground the
        // build's own geometry keeps — the height() fn the
        // overworld's hills the END never carries, the footprint the
        // island's END_STONE verdict the rim straddles refuse)
        let count = 10;
        let ring = radius * 0.72;
        for i in 0..count {
            let angle = std::f64::consts::TAU * i as f64 / count as f64;
            let px = (cx + ring * angle.cos()).round() as i32;
            let pz = (cz + ring * angle.sin()).round() as i32;
            let corners = [(px, pz), (px + 1, pz), (px, pz + 1), (px + 1, pz + 1)];
            if corners.iter().any(|&(x, z)| {
                x < 0 || z < 0 || x >= n || z >= n || self.get(x, top, z) != END_STONE
            }) {
                continue; // the void's rim the pillar never straddles
            }
            let height = 4 + (hash2(px as i64, pz as i64, self.seed ^ 0x911) % 5) as i32;
            for y in top..=(top + height) {
                self.set(px, y, pz, OBSIDIAN);
                self.set(px + 1, y, pz, OBSIDIAN);
                self.set(px, y, pz + 1, OBSIDIAN);
                self.set(px + 1, y, pz + 1, OBSIDIAN);
            }
        }
        // THE EXIT PEDESTAL (the fountain's own base rows the center
        // wears — the bedrock the real structure's own material keeps,
        // THE PORTAL'S OWN SURFACE (javap EndPortalBlock — the
        // diamond's interior the portal cells the vanilla exit
        // portal's own shape keeps: the bedrock ring the |dx|+|dz|<=2
        // band speaks, the starry field the inner 5 cells wear — the
        // touch lane the instant teleport arms)
        let (ix, iz) = (cx.round() as i32, cz.round() as i32);
        for dx in -2i32..=2 {
            for dz in -2i32..=2 {
                if dx.abs() + dz.abs() <= 2 {
                    let cell = if dx.abs() + dz.abs() <= 1 {
                        END_PORTAL
                    } else {
                        BEDROCK
                    };
                    self.set(ix + dx, top, iz + dz, cell);
                }
            }
        }
        self.set(ix, top + 1, iz, BEDROCK);
        self.set(ix, top + 2, iz, BEDROCK);
        self.set(ix, top + 3, iz, BEDROCK);
    }

    /// THE BLANK WORLD'S OWN CANVAS — the struct literal the `new`
    /// walk carried verbatim (the defaults the procedural scene
    /// speaks), the generate call the two dimensional builds answer
    /// each their own (the overworld the terrain walk, the nether
    /// the cavern build).
    fn blank(chunks_per_side: usize, seed: u64) -> Self {
        Self {
            chunks_per_side,
            seed,
            chunks: vec![vec![AIR; CHUNK_X * CHUNK_Y * CHUNK_Z]; chunks_per_side * chunks_per_side],
            light: None, // the procedural scene carries no light layer
            day_tick: None, // no clock yet: full day
            biomes: None, // the procedural scene carries no biome layer
            variants: None, // the procedural scene carries no state variants
            ambient_light: None, // the procedural scene: the black bytecode default
            block_light_tint: None, // the procedural scene: the bytecode default tint
            sky_light_factor: None, // the procedural scene: the bytecode default 1.0
            sky_light_color: None, // the procedural scene: the -1 default's white identity
            night_vision_color: None, // the procedural scene: the #999999 default, factor 0.0
            night_vision_factor: 0.0, // the procedural scene: no effect store, the golden floor
            rain_level: 0.0,  // the procedural scene: the dry world the goldens keep
            thunder_level: 0.0,
            darkness_scale: 0.0, // the procedural scene: no effect the store arms // the procedural scene: no storm the clouds speak
            difficulty: crate::difficulty::Difficulty::Normal, // the host's own new-world default
            difficulty_locked: false, // the lock the confirm's Yes alone arms
            inhabited: Some(vec![0u64; chunks_per_side * chunks_per_side]), // the host world's own dwell clock
            fire_ticks: Vec::new(), // the fire's own queue: no fire, no schedule
            campfire_ticks: Vec::new(), // the campfire's own queue: no lit campfire, no walk
            bamboo_ticks: Vec::new(), // the bamboo's own queue: no unsupported stalk, no fall
            fire_age: std::collections::HashMap::new(), // the ladder's store: empty
            portal_axis: std::collections::HashMap::new(), // the portal's own axis store: empty
            has_skylight: true, // the overworld default: the sky light the column pass speaks
        }
    }

    /// THE NETHER'S OWN TERRAIN (the client's deterministic stand-in
    /// the PortalForcer-less jar keeps): the bedrock floor y=0 and
    /// ceiling y=47 the real dimension's roof/floor the 48-tall grid
    /// scales, the netherrack rise the hash band 6..=13 speaks, the
    /// lava sea NETHER_LAVA_LEVEL the ocean's own level
    /// keeps, the soul sand the shoreline's patch roll wears, the
    /// pillars the spike roll raises (the classic cavern's own
    /// columns). No skylight law here — [`Self::new_nether`] the flag
    /// banks, the light walk the gate reads.
    fn generate_nether(&mut self) {
        const NETHER_SEED_SALT: u64 = 0x4E7B_0D; // the nether's own salt
        let n = self.size_blocks();
        let top = CHUNK_Y as i32 - 1;
        for wx in 0..n {
            for wz in 0..n {
                let ground =
                    6 + (hash2(wx as i64, wz as i64, self.seed ^ NETHER_SEED_SALT) % 8) as i32;
                let shore =
                    hash2(wx as i64, wz as i64, self.seed ^ (NETHER_SEED_SALT << 8)) % 7 == 0;
                let spike =
                    hash2(wx as i64, wz as i64, self.seed ^ (NETHER_SEED_SALT << 16)) % 23 == 0;
                for y in 0..=top {
                    let id = if y == 0 || y == top {
                        BEDROCK
                    } else if y <= ground {
                        if y >= ground - 1 && shore {
                            SOUL_SAND
                        } else {
                            NETHERRACK
                        }
                    } else if y <= NETHER_LAVA_LEVEL {
                        LAVA
                    } else if spike {
                        NETHERRACK
                    } else {
                        AIR
                    };
                    self.set(wx, y, wz, id);
                }
            }
        }
    }

    /// THE RETURN PORTAL'S OWN BUILD (the PortalForcer fiction — the
    /// server-side class the client jar never carries, the swap's
    /// deterministic stand-in the vanilla createPortal's own shape
    /// keeps: the 4x5 obsidian frame the interior 2x3 the portal
    /// room's platform floor wears; the X axis the RNG arm's
    /// determinism the client fiction pins — the interior the
    /// [`Self::check_portal_spawn`] walk's own scan lights).
    pub fn build_portal_frame(&mut self, bottom_left: (i32, i32, i32)) -> (i32, i32, i32) {
        let (bx, by, bz) = bottom_left;
        // THE SITE'S OWN CLEAR (the PortalForcer's clearing walk —
        // the frame plane the terrain's intrusions never keep)
        for i in 0..4 {
            for j in 0..5 {
                self.set(bx + i, by + j, bz, AIR);
            }
        }
        // THE PLATFORM (the portal room's own floor — the descent's
        // stop the fill's bottom-left walk wears)
        for i in 0..4 {
            self.set(bx + i, by - 1, bz, NETHERRACK);
        }
        // THE FRAME (the 4x5 obsidian the vanilla shape keeps)
        for i in 0..4 {
            self.set(bx + i, by, bz, OBSIDIAN);
            self.set(bx + i, by + 4, bz, OBSIDIAN);
        }
        for j in 1..4 {
            self.set(bx, by + j, bz, OBSIDIAN);
            self.set(bx + 3, by + j, bz, OBSIDIAN);
        }
        // THE INTERIOR'S OWN START CELL (the fill's start the finder
        // walk reads)
        (bx + 1, by + 1, bz)
    }

    /// THE OFFLINE WORLD'S OWN RELIGHT — the light engine's grid the
    /// world carries from the act that owns it ([`crate::light::compute`]
    /// the sky column + the block flood walks). `World::new` itself
    /// keeps the honest None (the raw procedural scene — the bare orbit
    /// lane's golden contract): the WORLD SCREEN's construction lights
    /// the world once here, the edits after that the set walk's own
    /// incremental answer keeps current ([`Self::ensure_light_current`]
    /// the remesh's guard wears), the integrated-server fiction the
    /// offline lane speaks.
    pub fn relight(&mut self) {
        self.light = Some(crate::light::compute(self));
    }

    /// THE REMESH'S OWN LIGHT GUARD — the grid the set walk's
    /// incremental speak already keeps current rides as-is (the edit
    /// lane's per-set answer [`crate::light::update_cell`] speaks); the
    /// never-lit world the full compute lights once (the honest None
    /// the raw procedural scene carries). The per-edit full recompute
    /// the remesh used to pay the walk retires — the dig tick's own
    /// hitch the incremental answer lifts.
    pub fn ensure_light_current(&mut self) {
        if self.light.is_none() {
            self.relight();
        }
    }

    pub fn size_blocks(&self) -> i32 {
        (self.chunks_per_side * CHUNK_X) as i32
    }

    fn chunk_index(&self, cx: i32, cz: i32) -> Option<usize> {
        if cx < 0 || cz < 0 {
            return None;
        }
        let s = self.chunks_per_side as i32;
        if cx >= s || cz >= s {
            return None;
        }
        Some((cx + cz * s) as usize)
    }

    pub fn set(&mut self, wx: i32, wy: i32, wz: i32, id: u16) {
        if wy < 0 || wy >= CHUNK_Y as i32 {
            return;
        }
        let cx = wx.div_euclid(CHUNK_X as i32);
        let cz = wz.div_euclid(CHUNK_Z as i32);
        if let Some(ci) = self.chunk_index(cx, cz) {
            let lx = wx.rem_euclid(CHUNK_X as i32) as usize;
            let lz = wz.rem_euclid(CHUNK_Z as i32) as usize;
            let cell = lx + lz * CHUNK_X + wy as usize * CHUNK_X * CHUNK_Z;
            let old = self.chunks[ci][cell];
            if old == id {
                return; // THE SAME STATE'S OWN SILENCE — the no-op edit the walk never pays
            }
            self.chunks[ci][cell] = id;
            // THE CAMPFIRE QUEUE'S OWN MAINTENANCE (the autopush-530
            // round — the lit transition the entry pushes, the un-lit
            // the entry drops: the ONE-live-entry-per-cell law the fire
            // queue's own dedup speaks). The due-0 entry the next
            // drain picks up (the walk's re-arm the every-tick cadence
            // the BE ticker wears); the set()'s own same-state silence
            // the early return above keeps — the maintenance the
            // TRANSITIONS alone spends.
            {
                let was_lit = old == CAMPFIRE_LIT || old == SOUL_CAMPFIRE_LIT;
                let is_lit = id == CAMPFIRE_LIT || id == SOUL_CAMPFIRE_LIT;
                if is_lit && !was_lit {
                    if !self
                        .campfire_ticks
                        .iter()
                        .any(|&(_, px, py, pz)| px == wx && py == wy && pz == wz)
                    {
                        self.campfire_ticks.push((0, wx, wy, wz));
                    }
                } else if was_lit && !is_lit {
                    self.campfire_ticks
                        .retain(|&(_, px, py, pz)| !(px == wx && py == wy && pz == wz));
                }
            }
            // THE EDIT'S OWN INCREMENTAL LIGHT (the vanilla LightEngine's
            // block-change answer — the 26.2 jar's checkBlock update the
            // one setBlock speaks): the grid the world carries the walk
            // updates in place, the two-queue removal/increase flood the
            // one cell's edit wears — the full recompute the per-edit
            // tick never pays again. The never-lit world keeps the
            // honest None (the remesh's guard the full compute speaks
            // once).
            if self.light.is_some() {
                let mut grid = self.light.take();
                if let Some(grid) = grid.as_mut() {
                    crate::light::update_cell(self, grid, wx, wy, wz);
                }
                self.light = grid;
            }
        }
    }

    /// THE DWELL CLOCK'S OWN TICK (the 26.2 server walk the accumulator
    /// mirrors — `ServerChunkCache.tickSpawningChunk` the
    /// `incrementInhabitedTime(gameTimeDelta)` call banks, the
    /// eligibility the `collectSpawningChunks` →
    /// `anyPlayerCloseEnoughForSpawningInternal` pair speaks): one
    /// game tick, every loaded chunk whose 2D chunk-center
    /// distance-squared to the player is STRICTLY below 16384.0 (the
    /// 128-block radius — `euclideanDistanceSquared` the
    /// `(cx*16+8, cz*16+8)` center the `sectionToBlockCoord(pos, 8)`
    /// walk keeps, the `dcmpg; ifge -> false` the strict `<` wears,
    /// the spectator the vanilla's own gate refuses — the client's
    /// body never spectates) banks +1. `None` = the no-opinion lane:
    /// the networked world the walk never touches.
    pub fn tick_inhabited(&mut self, px: f64, pz: f64) {
        let Some(inhabited) = &mut self.inhabited else {
            return;
        };
        let side = self.chunks_per_side as i32;
        for cz in 0..side {
            for cx in 0..side {
                let center_x = (cx * CHUNK_X as i32 + 8) as f64;
                let center_z = (cz * CHUNK_Z as i32 + 8) as f64;
                let dx = center_x - px;
                let dz = center_z - pz;
                if dx * dx + dz * dz < 16384.0 {
                    inhabited[(cx + cz * side) as usize] += 1;
                }
            }
        }
    }

    /// The data-driven biome tint for one block face — the synced
    /// registry's color for the voxel's biome, `None` when the baked
    /// look stands: no biome layer, an unsent cell, an id the server
    /// never named, an untinted face, or a color equal to the baked
    /// default (the atlas pixels already carry it exactly — see the
    /// `tint` module's no-op rule). The mesher folds a `Some` into the
    /// vertex-color carrier: the vanilla tint multiply.
    pub fn face_tint(&self, kind: u16, face: usize, wx: i32, wy: i32, wz: i32) -> Option<u32> {
        let layer = self.biomes.as_ref()?;
        let slot = tint::slot_for(kind, face)?;
        // THE DOUBLE PLANT'S OWN BELOW-SHIFT (javap
        // BlockTintSources$2.colorInWorld over the 26.2 jar — the
        // doubleTallGrass() source the audited walk keeps: the UPPER
        // half's `BiomeColors.getAverageGrassColor(level, pos.below())`
        // read the HALF compare speaks, the LOWER half's own-pos read
        // the else arm wears — the biome color the LOWER cell's
        // position samples for BOTH halves). The no-op rule the exact
        // lanes keep: the offline no-biome lane and the same-biome
        // verdict both ride the identity (the shifted query the same
        // column's answer keeps).
        let (qx, qy, qz) = if is_double_plant_upper(kind) {
            (wx, wy - 1, wz)
        } else {
            (wx, wy, wz)
        };
        let id = self.biome_id(layer, qx, qy, qz)?;
        let t = layer.tints.get(id as usize)?;
        let c = t[slot]?;
        (c != atlas::TINTS[kind as usize][face]).then_some(c)
    }

    /// The voxel's resolved registry biome id — `None` outside the grid
    /// (above/below the world, an unsent chunk) or where the server
    /// never answered ([`BIOME_UNKNOWN`]).
    fn biome_id(&self, layer: &BiomeLayer, wx: i32, wy: i32, wz: i32) -> Option<u32> {
        if wy < 0 || wy >= CHUNK_Y as i32 {
            return None;
        }
        let cx = wx.div_euclid(CHUNK_X as i32);
        let cz = wz.div_euclid(CHUNK_Z as i32);
        let ci = self.chunk_index(cx, cz)?;
        let lx = wx.rem_euclid(CHUNK_X as i32) as usize;
        let lz = wz.rem_euclid(CHUNK_Z as i32) as usize;
        let id = layer.ids[ci][lx + lz * CHUNK_X + wy as usize * CHUNK_X * CHUNK_Z];
        (id != BIOME_UNKNOWN).then_some(id)
    }

    pub fn get(&self, wx: i32, wy: i32, wz: i32) -> u16 {
        if wy < 0 {
            return STONE; // treat below-world as solid (no bottom faces)
        }
        if wy >= CHUNK_Y as i32 {
            return AIR;
        }
        let cx = wx.div_euclid(CHUNK_X as i32);
        let cz = wz.div_euclid(CHUNK_Z as i32);
        if let Some(ci) = self.chunk_index(cx, cz) {
            let lx = wx.rem_euclid(CHUNK_X as i32) as usize;
            let lz = wz.rem_euclid(CHUNK_Z as i32) as usize;
            self.chunks[ci][lx + lz * CHUNK_X + wy as usize * CHUNK_X * CHUNK_Z]
        } else {
            AIR
        }
    }

    /// The column the grid CARRIES — the client's own `hasChunk` word
    /// (the jar's `ClientLevel.getPrecipitationAt` gate: an unloaded
    /// column's precipitation pick is NONE before any biome reads).
    /// The weather renderer's extraction walks this gate.
    pub fn column_sent(&self, wx: i32, wz: i32) -> bool {
        let cx = wx.div_euclid(CHUNK_X as i32);
        let cz = wz.div_euclid(CHUNK_Z as i32);
        self.chunk_index(cx, cz).is_some()
    }

    /// The voxel's biome CLIMATE pair (the synced registry's
    /// `has_precipitation`/`temperature` through the layer's
    /// arrival-order table) — `None` outside the grid, an unsent
    /// column, a [`BIOME_UNKNOWN`] voxel, or an id the table never
    /// answered: the honest no-opinion, never a guessed climate.
    pub fn biome_climate_at(&self, wx: i32, wy: i32, wz: i32) -> Option<BiomeClimate> {
        let layer = self.biomes.as_ref()?;
        let id = self.biome_id(layer, wx, wy, wz)?;
        layer.climates.get(id as usize).copied()
    }

    /// The per-cell STATE-VARIANT byte (the network world's
    /// `variants` layer the materialize walk stamps — see
    /// [`crate::world_view::leaf_litter_variant`] and
    /// [`crate::world_view::bubble_column_variant`]): the state
    /// properties one decode the wire's id keeps. `0` the no-opinion
    /// answer everywhere else — the procedural scene (no layer), the
    /// outside-the-grid cell, the unsent chunk.
    pub fn variant(&self, wx: i32, wy: i32, wz: i32) -> u8 {
        let Some(vs) = self.variants.as_ref() else {
            return 0;
        };
        if wy < 0 || wy >= CHUNK_Y as i32 {
            return 0;
        }
        let cx = wx.div_euclid(CHUNK_X as i32);
        let cz = wz.div_euclid(CHUNK_Z as i32);
        let Some(ci) = self.chunk_index(cx, cz) else {
            return 0;
        };
        let lx = wx.rem_euclid(CHUNK_X as i32) as usize;
        let lz = wz.rem_euclid(CHUNK_Z as i32) as usize;
        vs[ci][lx + lz * CHUNK_X + wy as usize * CHUNK_X * CHUNK_Z]
    }

    /// THE OFFLINE VARIANT'S OWN STAMP (the networked world's layer the
    /// materialize walk builds wholesale — the offline world's own edits
    /// the lazy allocation serves: the furnace's placement facing the
    /// first byte the grid keeps, every other cell the 0 no-opinion
    /// answer the materialize walk's fresh layer wears — the same
    /// `vec![vec![0u8; CHUNK_X*CHUNK_Y*CHUNK_Z]; chunks_per_side²]`
    /// layout the [`crate::world_view::materialize`] walk banks). The
    /// bounds the [`Self::variant`] read's own walks keep — the
    /// outside-grid cell the honest no-write carries.
    pub fn set_variant(&mut self, wx: i32, wy: i32, wz: i32, v: u8) {
        if wy < 0 || wy >= CHUNK_Y as i32 {
            return;
        }
        let cx = wx.div_euclid(CHUNK_X as i32);
        let cz = wz.div_euclid(CHUNK_Z as i32);
        let Some(ci) = self.chunk_index(cx, cz) else {
            return;
        };
        let layer = self.variants.get_or_insert_with(|| {
            vec![
                vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z];
                self.chunks_per_side * self.chunks_per_side
            ]
        });
        let lx = wx.rem_euclid(CHUNK_X as i32) as usize;
        let lz = wz.rem_euclid(CHUNK_Z as i32) as usize;
        layer[ci][lx + lz * CHUNK_X + wy as usize * CHUNK_X * CHUNK_Z] = v;
    }

    /// The entry face's own normal offsets, indexed by the Direction
    /// ordinal the wire speaks (DOWN=0, UP=1, NORTH=2, SOUTH=3, WEST=4,
    /// EAST=5 — the enum's own order). The placement cell is the hit
    /// voxel offset by the hit face's normal (the vanilla
    /// `BlockHitResult.getLocation`-relative placement law).
    pub const DIRECTION_NORMAL: [(i32, i32, i32); 6] = [
        (0, -1, 0), // DOWN
        (0, 1, 0),  // UP
        (0, 0, -1), // NORTH
        (0, 0, 1),  // SOUTH
        (-1, 0, 0), // WEST
        (1, 0, 0),  // EAST
    ];

    /// The item name's placeable block id — the vanilla `BlockItem`'s
    /// own pairing (the item's registry name IS its block's for the
    /// simple cubes): the reverse of [`NAME_OF`], `air` refused (the
    /// empty id is the void, never a stack's payload) and `water`
    /// refused (no vanilla item carries the fluid as its block).
    /// THE ITEM REGISTRY'S OWN CONSULT (the autopush-608 round — the
    /// item-less blocks the vanilla itself grants: the
    /// BAMBOO_SAPLING the register(BlockIds.BAMBOO_SAPLING, ...)
    /// NO-BlockItemId overload places nothing (the plain-ground
    /// placement the bamboo item's own getStateForPlacement walk
    /// serves — the SAPLING BLOCK the product, never the item), the
    /// NETHER_PORTAL the FACTORY walk the same law keeps): the name
    /// the ITEM registry never carries the lookup refuses (the honest
    /// no-place, never a guessed voxel). `None` = the item places
    /// nothing this client's world vocabulary can render.
    pub fn placeable_block(item_name: &str) -> Option<u16> {
        // the item the registry never carries — the item-less block
        // the vanilla's own registration refuses (the BlockItem
        // pairing the item's own existence demands)
        mc_registry::item_id_by_name(item_name)?;
        let id = NAME_OF.iter().position(|n| *n == item_name)? as u16;
        if id == AIR || id == WATER {
            return None;
        }
        // THE WHEAT REFUSAL (javap Items: the harvest item is no
        // BlockItem — the right-press the use lane's PASS keeps): the
        // crop kind the seeds' own plant arm alone wears, the generic
        // placement lane never speaks it — the whole AGE LADDER the
        // same law keeps (no stage's kind the item vocabulary names,
        // the ladder the growth walk's own setBlock alone wears)
        if is_wheat(id) {
            return None;
        }
        Some(id)
    }

    /// THE PORTAL FRAME'S OWN SCAN — javap
    /// `net.minecraft.world.level.portal.PortalShape` over the real
    /// 26.2 jar (the full walk the `BaseFireBlock.onPlace` tail
    /// serves, the autopush-606 round): the fire the igniter arms
    /// place the scan's own start cell wears, the EMPTY predicate
    /// (javap PortalShape.isEmpty pc 0-32 — `state.isAir() ||
    /// state.is(BlockTags.FIRE) || state.is(NETHER_PORTAL)` the
    /// three-way verdict the walk keeps — THE FIRE CELL ITSELF the
    /// scan's own start the rule admits, the just-placed flame the
    /// interior's own member the fill overwrites) the interior test
    /// answers, the FRAME predicate (javap lambda$static$0 pc 0-7 —
    /// `state.is(Blocks.OBSIDIAN)` the verdict the static{}'s
    /// invokedynamic #334 banks) the frame cells test. THE WALK
    /// (javap findEmptyPortalShape pc 0-11 → findPortalShape pc 0-58
    /// → findAnyShape pc 0-115): the X-axis trial FIRST (rightDir
    /// WEST pc 7-10 the axis-X arm keeps), the predicate filter
    /// (javap lambda$findEmptyPortalShape$0 — `isValid() &&
    /// numPortalBlocks == 0` the EMPTY-shape verdict the interior's
    /// own portal blocks the gate refuses — the lit gate the re-scan
    /// never refills), the Z-axis trial the failed filter's own
    /// else-branch speaks (pc 26-58 the otherAxis the X arm's
    /// refusal keeps). THE DESCENT (javap calculateBottomLeft pc
    /// 0-81): the floor bound `Math.max(level.getMinY(), y - 21)`
    /// pc 0-16 the 21-step cap the walk wears (the offline world's
    /// own floor the y=0 the below-world STONE read never crosses —
    /// the getMinY fold the same bound the client's own vertical
    /// range banks), the while `y > i && isEmpty(below)` loop pc
    /// 17-46 the air column descends, the OPPOSITE-edge walk pc
    /// 55-62 (`getDistanceUntilEdgeAboveFrame(level, pos,
    /// rightDir.opposite())` the -1 the pc 62-64 subtraction keeps)
    /// the bottom-left cell the last interior column answers, the
    /// negative distance pc 66-72 the null the degenerate shape
    /// (width 0, height 0, bottomLeft the start cell the findAny
    /// Shape's pc 30-43 arm keeps) wears. THE EDGE WALK (javap
    /// getDistanceUntilEdgeAboveFrame pc 0-105): the `for i in
    /// 0..=21` loop pc 11-101 the INCLUSIVE bound the if_icmpgt 21
    /// keeps — the cursor `pos.move(dir, i)` the cell reads, the
    /// non-empty cell the edge candidate answers (FRAME test pc
    /// 47-64 the obsidian the distance returns, the non-obsidian
    /// solid the 0 the pc 104 fallback banks), the empty cell the
    /// below-FRAME check pc 65-97 keeps (the floor the non-obsidian
    /// cell the 0 the walk refuses — the stone floor the scan's own
    /// refusal speaks), the bound-exhaust the 0 keeps. THE WIDTH
    /// (javap calculateWidth pc 0-21): the same edge walk the
    /// rightDir arm answers, the `< 2 || > 21` band pc 7-20 the 0
    /// the out-of-band walk keeps (MIN_WIDTH 2 / MAX_WIDTH 21 the
    /// ConstantValue attributes speak). THE HEIGHT (javap calculate
    /// Height pc 0-53 → getDistanceUntilTop pc 0-175): the `while y
    /// < 21` row walk pc 3-170 — per row the LEFT frame cell
    /// `bottomLeft + UP*y + rightDir*(-1)` pc 10-51 the FRAME test
    /// the failed walk returns y keeps, the RIGHT frame cell
    /// `+ rightDir*width` pc 52-94 the same test keeps, the interior
    /// row pc 95-164 the EMPTY test the non-empty cell the y keeps +
    /// the portal count pc 145-160 the NETHER_PORTAL cell the
    /// MutableInt increments — the 21 the bound-exhaust keeps (pc
    /// 173-175); the `d < 3 || d > 21` band pc 22-49 the 0 keeps
    /// (MIN_HEIGHT 3 / MAX_HEIGHT 21 the ConstantValue pair speaks)
    /// and the TOP FRAME (javap hasTopFrame pc 0-62) the row ABOVE
    /// the measured height the `bottomLeft + UP*d + rightDir*x` walk
    /// the every-column FRAME test answers (the missing top the 0
    /// the pc 49-50 refusal keeps). THE VALIDITY (javap isValid pc
    /// 0-39): `width in [2,21] && height in [3,21]` the conjunction
    /// the shape verdict keeps. THE FILL (javap createPortalBlocks pc
    /// 0-68): `NETHER_PORTAL.defaultBlockState().setValue(AXIS, axis)`
    /// the state the kind-only vocabulary never stores (the mesher's
    /// axis lane the NEIGHBOR reads derive — the autopush-605 round's
    /// own law the west/east short-circuit keeps), the
    /// `betweenClosed(bottomLeft, bottomLeft + UP*(h-1) + rightDir*
    /// (w-1))` box the interior cells the setBlock(state, 18) walk
    /// fills (flags 18 = UPDATE_CLIENTS | UPDATE_NOOBSERVER — the
    /// neighbor-less broadcast the kind-only client never needs), the
    /// FIRE CELL the scan's own start the interior member the fill
    /// OVERWRITES (the vanilla setBlock the later call the same pos
    /// the portal state wears — the scheduled fire tick the stale
    /// entry the block-match gate drops). THE DIMENSION GATE (javap
    /// BaseFireBlock.inPortalDimension pc 0-25): `dimension() ==
    /// OVERWORLD || NETHER` the verdict the scan's own caller
    /// consults — the offline client the OVERWORLD's own dimension
    /// wears, the gate the TRUE the single-dimension walk keeps.
    pub fn find_empty_portal_shape(&self, start: (i32, i32, i32)) -> Option<PortalShapeFill> {
        // THE X-AXIS TRIAL FIRST (findPortalShape pc 0-25 — the axis
        // the caller's own X arg carries, the rightDir WEST the
        // findAnyShape's pc 1-10 arm keeps), the predicate the empty
        // verdict filters, THE Z-AXIS TRIAL the failed filter's own
        // else speaks (pc 26-58 — the otherAxis the X arm's refusal
        // keeps, the SOUTH rightDir the axis-Z arm answers)
        let x_trial = self.portal_find_any_shape(start, true);
        if x_trial.is_valid() && x_trial.num_portal_blocks == 0 {
            return Some(x_trial);
        }
        let z_trial = self.portal_find_any_shape(start, false);
        if z_trial.is_valid() && z_trial.num_portal_blocks == 0 {
            return Some(z_trial);
        }
        None
    }

    /// THE ONPLACE TAIL'S OWN FILL — javap `BaseFireBlock.onPlace` pc
    /// 0-50 over the 26.2 jar: the `inPortalDimension(level)` gate pc
    /// 13-17 the scan consults (the OVERWORLD/NETHER disjunction the
    /// offline client's own overworld the TRUE answers), the
    /// `findEmptyPortalShape(level, pos, Axis.X)` pc 20-28 the scan
    /// the fire's own cell starts, the `createPortalBlocks` pc 38-47
    /// the present shape the interior fills — the TRUE the fill's own
    /// verdict carries, the failed scan the FALSE the fire's own
    /// landing keeps (the vanilla's own pc 50 return the portal-less
    /// arm the canSurvive check the arm's own gate already answered).
    pub fn check_portal_spawn(&mut self, pos: (i32, i32, i32)) -> bool {
        if let Some(fill) = self.find_empty_portal_shape(pos) {
            for (x, y, z) in fill.cells() {
                self.set(x, y, z, NETHER_PORTAL);
                // THE AXIS PROPERTY THE SETVALUE WALK BANKS (javap
                // createPortalBlocks pc 24-28 — the `setValue(AXIS,
                // axis)` state the kind-only vocabulary never keeps,
                // the side map the fire_age precedent wears): the
                // winning trial's own axis the store carries, the
                // updateShape's flag11 verdict the douse walk reads
                self.portal_axis.insert((x, y, z), fill.axis_x);
            }
            true
        } else {
            false
        }
    }

    /// THE ONE-AXIS SHAPE WALK (javap findAnyShape pc 0-115 — the
    /// rightDir the axis arm picks [X -> WEST pc 7-10, else SOUTH pc
    /// 13-16], the bottom-left the descent answers, the degenerate
    /// shapes the 0x0 the pc 30-43 / 58-72 arms bank, the full walk
    /// the width + height the ctor pc 95-115 carries)
    fn portal_find_any_shape(&self, start: (i32, i32, i32), axis_x: bool) -> PortalShapeFill {
        // THE RIGHT DIR (pc 1-16): X -> WEST (-1,0), Z -> SOUTH (0,+1)
        let right = if axis_x { (-1, 0) } else { (0, 1) };
        // THE BOTTOM-LEFT (pc 17-48 — the null the degenerate shape
        // the pc 30-43 arm wears: width 0, height 0, the START cell
        // the bottomLeft the ctor keeps)
        let bl = match self.portal_calculate_bottom_left(right, start) {
            Some(bl) => bl,
            None => {
                return PortalShapeFill {
                    bottom_left: start,
                    width: 0,
                    height: 0,
                    axis_x,
                    num_portal_blocks: 0,
                }
            }
        };
        // THE WIDTH (pc 44-72 — the 0 the degenerate shape the pc
        // 58-72 arm wears: width 0, height 0, the bottomLeft the
        // ctor keeps)
        let width = self.portal_calculate_width(bl, right);
        if width == 0 {
            return PortalShapeFill {
                bottom_left: bl,
                width: 0,
                height: 0,
                axis_x,
                num_portal_blocks: 0,
            };
        }
        // THE HEIGHT (pc 73-115 — the MutableInt the numPortalBlocks
        // the calculateHeight's own count banks, the ctor the full
        // shape pc 95-115 carries)
        let mut num_portal_blocks = 0;
        let height = self.portal_calculate_height(bl, right, width, &mut num_portal_blocks);
        PortalShapeFill {
            bottom_left: bl,
            width,
            height,
            axis_x,
            num_portal_blocks,
        }
    }

    /// THE DESCENT'S OWN FLOOR (javap calculateBottomLeft pc 0-81):
    /// the floor bound `Math.max(getMinY(), y - 21)` pc 0-16 (the
    /// offline world's own floor the 0 the below-world STONE read
    /// never crosses), the `while y > i && isEmpty(below)` descent pc
    /// 17-46, the opposite-edge walk pc 55-62 the -1 pc 62-64 keeps,
    /// the negative pc 66-72 the None the caller's degenerate arm
    /// wears
    fn portal_calculate_bottom_left(
        &self,
        right: (i32, i32),
        start: (i32, i32, i32),
    ) -> Option<(i32, i32, i32)> {
        let floor_bound = std::cmp::max(0, start.1 - 21);
        let mut p = start;
        while p.1 > floor_bound && portal_cell_empty(self.get(p.0, p.1 - 1, p.2)) {
            p.1 -= 1;
        }
        // THE OPPOSITE WALK (pc 49-64 — the getOpposite() the pc 49-53
        // fold keeps, the distance the -1 the interior cell answers)
        let opp = (-right.0, -right.1);
        let d = self.portal_distance_until_edge_above_frame(p, opp) - 1;
        if d < 0 {
            return None; // pc 66-72 — the null the degenerate shape wears
        }
        // pc 73-81 — the relative(opposite, d) the last interior
        // column the bottom-left keeps
        Some((p.0 + opp.0 * d, p.1, p.2 + opp.1 * d))
    }

    /// THE EDGE WALK (javap getDistanceUntilEdgeAboveFrame pc 0-105):
    /// the `for i in 0..=21` INCLUSIVE loop pc 11-101, the non-empty
    /// cell the FRAME test pc 47-64 answers (the obsidian the
    /// distance returns, the non-obsidian solid the 0 keeps), the
    /// empty cell the below-FRAME check pc 65-97 keeps (the floor
    /// the non-obsidian cell the 0 refuses), the bound-exhaust the 0
    /// pc 104-105 keeps
    fn portal_distance_until_edge_above_frame(
        &self,
        from: (i32, i32, i32),
        dir: (i32, i32),
    ) -> i32 {
        for i in 0..=21 {
            let cx = from.0 + dir.0 * i;
            let cz = from.2 + dir.1 * i;
            let s = self.get(cx, from.1, cz);
            if !portal_cell_empty(s) {
                if s == OBSIDIAN {
                    return i; // pc 62-64 — the edge the distance answers
                }
                return 0; // pc 47-59 ifeq 104 — the solid non-frame the walk refuses
            }
            // pc 65-97 — the below cell the FRAME test the failed
            // floor the walk refuses
            if self.get(cx, from.1 - 1, cz) != OBSIDIAN {
                return 0;
            }
        }
        0 // pc 104-105 — the bound-exhaust the 0 keeps
    }

    /// THE WIDTH'S OWN BAND (javap calculateWidth pc 0-21): the edge
    /// walk the rightDir arm answers, the `< 2 || > 21` band pc 7-20
    /// the 0 keeps (MIN_WIDTH 2 / MAX_WIDTH 21 the ConstantValue
    /// pair speaks)
    fn portal_calculate_width(&self, bl: (i32, i32, i32), right: (i32, i32)) -> i32 {
        let d = self.portal_distance_until_edge_above_frame(bl, right);
        if d < 2 || d > 21 {
            0
        } else {
            d
        }
    }

    /// THE HEIGHT'S OWN WALK (javap calculateHeight pc 0-53 →
    /// getDistanceUntilTop pc 0-175): the row walk pc 3-170 — the
    /// LEFT frame `rightDir*(-1)` pc 10-51, the RIGHT frame
    /// `rightDir*width` pc 52-94, the interior row pc 95-164 the
    /// EMPTY test + the portal count pc 145-160 the MutableInt keeps,
    /// the bound-exhaust the 21 pc 173-175 keeps; the `d < 3 || d >
    /// 21` band pc 22-49 the 0 keeps and the TOP FRAME (javap
    /// hasTopFrame pc 0-62) the row ABOVE the measured height the
    /// every-column FRAME test answers
    fn portal_calculate_height(
        &self,
        bl: (i32, i32, i32),
        right: (i32, i32),
        width: i32,
        num_portal_blocks: &mut i32,
    ) -> i32 {
        // THE ROW WALK'S OWN PRODUCT (javap getDistanceUntilTop's own
        // EVERY return path the calculateHeight's verdicts the walk's
        // d answers — the early arms the y the row the walk keeps, the
        // bound-exhaust the 21 the cap keeps)
        let d = self.portal_get_distance_until_top(bl, right, width, num_portal_blocks);
        // THE BAND (javap calculateHeight pc 22-49 — the `d < 3 ||
        // d > 21` pair the 0 the out-of-band walk keeps: MIN_HEIGHT 3
        // / MAX_HEIGHT 21 the ConstantValue pair speaks)
        if d < 3 || d > 21 {
            return 0;
        }
        // THE TOP FRAME (javap calculateHeight pc 35-46 — the
        // hasTopFrame consult the missing top the 0 keeps)
        if !self.portal_has_top_frame(bl, right, width, d) {
            return 0;
        }
        d
    }

    /// THE ROW WALK'S OWN DISTANCE (javap getDistanceUntilTop pc
    /// 0-175): the `while y < 21` row walk — per row the LEFT frame
    /// cell pc 10-51 the FRAME test the failed walk returns y keeps,
    /// the RIGHT frame cell pc 52-94 the same test keeps, the
    /// interior row pc 95-164 the EMPTY test + the portal count pc
    /// 145-160 the MutableInt increments — the 21 the bound-exhaust
    /// pc 173-175 keeps
    fn portal_get_distance_until_top(
        &self,
        bl: (i32, i32, i32),
        right: (i32, i32),
        width: i32,
        num_portal_blocks: &mut i32,
    ) -> i32 {
        let mut y = 0;
        while y < 21 {
            // pc 10-51 — the LEFT frame cell the FRAME test answers
            if self.get(bl.0 - right.0, bl.1 + y, bl.2 - right.1) != OBSIDIAN {
                return y;
            }
            // pc 52-94 — the RIGHT frame cell the same test keeps
            if self.get(bl.0 + right.0 * width, bl.1 + y, bl.2 + right.1 * width) != OBSIDIAN {
                return y;
            }
            // pc 95-164 — the interior row the EMPTY test + the
            // portal count the MutableInt increments
            for x in 0..width {
                let s = self.get(bl.0 + right.0 * x, bl.1 + y, bl.2 + right.1 * x);
                if !portal_cell_empty(s) {
                    return y; // pc 142-144 — the non-empty the walk keeps
                }
                if s == NETHER_PORTAL {
                    *num_portal_blocks += 1; // pc 156-160 — the count the gate reads
                }
            }
            y += 1;
        }
        21 // pc 173-175 — the bound-exhaust the cap keeps
    }

    /// THE TOP FRAME'S OWN ROW (javap hasTopFrame pc 0-62 — the
    /// every-column FRAME test the row ABOVE the measured height
    /// answers: the `bottomLeft + UP*d + rightDir*x` cursor walk pc
    /// 10-45, the failed cell the false the pc 50-54 refusal keeps,
    /// the full row the true the pc 61-62 tail keeps)
    fn portal_has_top_frame(
        &self,
        bl: (i32, i32, i32),
        right: (i32, i32),
        width: i32,
        height: i32,
    ) -> bool {
        (0..width).all(|x| self.get(bl.0 + right.0 * x, bl.1 + height, bl.2 + right.1 * x) == OBSIDIAN)
    }

    /// THE FARMLAND'S OWN HYDRATION SCAN — javap
    /// FarmlandBlock.isNearWater over the real 26.2 jar: the
    /// `BlockPos.betweenClosed(pos.offset(-4, 0, -4), pos.offset(4, 1,
    /// 4))` box the bytecode's bipush/iconst pairs keep — the x/z ±4
    /// ring, the y the farmland's own level + the one above (the 9x2x9
    /// the 162 cells wear), each cell the `getFluidState(cell).
    /// is(FluidTags.WATER)` read answers (the vocabulary's WATER kind
    /// the fluid-state law wears, the lava the water tag never
    /// carries). The randomTick's own hydrate arm the same scan speaks
    /// (`!isNearWater && !isRainingAt(pos.above())` the dry ramp keeps
    /// — the MOISTURE 0..7 the state property the kind-only cells
    /// never carry, the derived verdict the instant answer wears).
    pub fn is_near_water(&self, x: i32, y: i32, z: i32) -> bool {
        for dy in 0..=1 {
            for dx in -4..=4 {
                for dz in -4..=4 {
                    if self.get(x + dx, y + dy, z + dz) == WATER {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// THE GROWTH SPEED (javap CropBlock.getGrowthSpeed(Block, BlockGetter,
    /// BlockPos)F over the real 26.2 jar — the audited formula the fields'
    /// rhythm keeps):
    ///
    /// 1. `f = 1.0`;
    /// 2. the 3x3 scan AT `pos.below()`'s layer (the `below.offset(i, 0, j)`
    ///    walk, i/j in -1..=1): each GROWS_CROPS hit speaks `f1 = 1.0`, the
    ///    moisture read `getValueOrElse(FarmlandBlock.MOISTURE, 0) > 0` the
    ///    `f1 = 3.0` wet arm names — the arm the HYDRATION SCAN answers
    ///    (FarmlandBlock.isNearWater the 162-cell box wears: the farmland
    ///    cell's water within the x/z ±4 ring, the y the farmland's own
    ///    level + the one above — the MOISTURE 0..7 the state property the
    ///    kind-only cells never carry, the derived verdict the growth law's
    ///    own `> 0` gate exactly answers: the near-water farmland the wet
    ///    arm wears, the dry the 1.0 keeps; the drying ramp the state law
    ///    the fold never carries — the instant verdict the honest fold
    ///    speaks, the wet tile the atlas's later round wears); every
    ///    NON-CENTER cell the `f1 /= 4.0` quartering speaks
    ///    (`i != 0 || j != 0`); `f += f1`;
    /// 3. the SAME-LAYER crop neighbors (the `pos.north()/south()/west()/
    ///    east()` reads — the crop's own layer the symbolic trace's loc2
    ///    annotations name, NOT the below plane): any cardinal wheat hit
    ///    the `(f + 1.0) / 2.0` halving speaks, then any diagonal hit
    ///    (`north().west()` etc.) the second halving — the two sequential
    ///    `(f+1)/2` steps the javap tail's fold chain
    ///    `((1.0 + 3.0/4.0) / 2.0) / 2.0` keeps.
    ///
    /// The dyadic anchors the tests pin: isolated dry farmland = 2.0 (the
    /// center 1.0 + the below hit 1.0), the isolated wet farmland = 4.0
    /// (the below hit the 3.0 wet arm wears), the 3x3 farmland ring = 4.0
    /// dry / 10.0 wet, one cardinal neighbor = 2.5, + diagonal = 1.75, no
    /// farmland below = 1.0.
    pub fn crop_growth_speed(&self, x: i32, y: i32, z: i32) -> f32 {
        let mut f = 1.0f32;
        for di in -1i32..=1 {
            for dj in -1i32..=1 {
                let below = self.get(x + di, y - 1, z + dj);
                // THE MOISTURE'S OWN WET ARM (javap getGrowthSpeed: the
                // getValueOrElse(MOISTURE, 0) > 0 the 3.0 speaks): the
                // farmland's hydration the isNearWater scan answers —
                // the derived verdict the kind-only cells the state
                // property never carries
                let mut f1 = if grows_crops(below) {
                    if self.is_near_water(x + di, y - 1, z + dj) {
                        3.0
                    } else {
                        1.0
                    }
                } else {
                    0.0
                };
                if di != 0 || dj != 0 {
                    f1 /= 4.0;
                }
                f += f1;
            }
        }
        // the same-layer neighbor reads (the crop's own layer): the
        // cardinal OR-chain the first (f+1)/2 speaks, the diagonal the
        // second — the vanilla's own two sequential halvings
        let is_crop =
            |dx: i32, dz: i32| is_wheat(self.get(x + dx, y, z + dz));
        if is_crop(0, -1) || is_crop(0, 1) || is_crop(-1, 0) || is_crop(1, 0) {
            f = (f + 1.0) / 2.0;
        }
        if is_crop(-1, -1) || is_crop(1, -1) || is_crop(1, 1) || is_crop(-1, 1) {
            f = (f + 1.0) / 2.0;
        }
        f
    }

    /// THE CROP'S OWN RANDOM TICK (javap CropBlock.randomTick over the
    /// real 26.2 jar — the growth roll one law speaks):
    ///
    /// 1. `getRawBrightness(pos, 0) >= 9` — the bipush 9 + if_icmplt the
    ///    raw walk keeps; the grid ABSENT the procedural lane keeps the
    ///    documented fold (the sky-exposed field the outdoor farms speak —
    ///    the plant arm's own `>= 8` fold the same lane rides, the wire
    ///    world's server the real light engine answers);
    /// 2. `getAge(state) < getMaxAge()` (the 7 the ladder's ceiling
    ///    speaks) — the max-age cell the branch REFUSES with ZERO draws
    ///    (the if_icmpge the roll never reaches);
    /// 3. `nextInt((int)(25.0f / getGrowthSpeed(...)) + 1) == 0` — the
    ///    f2i TRUNCATION the javap's fdiv/f2i/iconst_1 pair keeps
    ///    (25.0/2.0 = 12.5 -> 12 -> 13 the isolated field speaks);
    /// 4. `setBlock(pos, getStateForAge(age + 1), 2)` — the ladder's
    ///    own step (the flag-2 the client's mesh re-speak answers, the
    ///    kind+1 the age family wears).
    ///
    /// Returns whether the crop GREW (the caller's remesh batch rides).
    pub fn crop_random_tick(&mut self, rng: &mut crate::drops::Lcg, x: i32, y: i32, z: i32) -> bool {
        let Some(age) = wheat_age(self.get(x, y, z)) else {
            return false; // the isRandomlyTicking gate — the non-crop cell draws nothing
        };
        if age >= 7 {
            return false; // the getMaxAge branch — the ceiling draws nothing
        }
        // the light gate the procedural sky's fold passes (the doc above)
        let f = self.crop_growth_speed(x, y, z);
        let modulus = (25.0f32 / f) as i32 + 1;
        if rng.next_int(modulus as u64) == 0 {
            self.set(x, y, z, WHEAT + age as u16 + 1);
            true
        } else {
            false
        }
    }

    /// THE BAMBOO'S OWN HEIGHT (javap BambooStalkBlock.
    /// getHeightBelowUpToMax pc 0-37 the real 26.2 jar): the
    /// contiguous bamboo-below count the `i < 16` loop guard caps —
    /// each round the `pos.below(i + 1)` read the BAMBOO verdict the
    /// count walks (pc 8-27), the non-bamboo cell the loop ends.
    fn bamboo_height_below(&self, x: i32, y: i32, z: i32) -> u32 {
        let mut count = 0u32;
        while count < 16 && self.get(x, y - count as i32 - 1, z) == BAMBOO {
            count += 1;
        }
        count
    }

    /// THE BAMBOO'S OWN RANDOM TICK (javap BambooSaplingBlock.
    /// randomTick pc 0-42 + BambooStalkBlock.randomTick pc 0-81 over
    /// the real 26.2 jar — the pair the two classes' own walks
    /// speak):
    ///
    /// THE SAPLING ARM (pc 0-42): `nextInt(3) == 0` FIRST (the pc 0-8
    /// roll the empty/light checks PRECEDE — the draw the occupied
    /// cell still consumes), `isEmptyBlock(pos.above())` (pc 11-19 —
    /// the strict-air verdict the kind-only store speaks), the
    /// `getRawBrightness(pos.above(), 0) >= 9` gate (pc 22-33 — the
    /// wheat's own light fold the grid-absent lane keeps), then
    /// growBamboo pc 0-28: `setBlock(pos.above(), BAMBOO(LEAVES=
    /// SMALL), 3)` — AND the sapling's own updateShape pc 17-43 (the
    /// flag-3 update the setBlock fires): the bamboo-above neighbor
    /// the sapling ITSELF converts (`Blocks.BAMBOO.defaultBlockState()
    /// ` the pc 36-42 return speaks) — the pair the store's net state
    /// wears (the base stalk + the top the growth's own two-block
    /// product answers).
    ///
    /// THE STALK ARM (pc 0-81): the STAGE==0 gate (pc 0-16 — the
    /// kind-only vocabulary the stage never stores: the fold EVERY
    /// stalk stage 0 keeps, the bone-meal lane the client never
    /// serves), the same `nextInt(3)` roll (pc 17-25), the same
    /// empty-above + light-above pair (pc 28-50), the height law
    /// `getHeightBelowUpToMax(...) + 1 < 16` (pc 53-67 — the bipush
    /// 16 the max height speaks), then growBamboo: the new top the
    /// `pos.above()` cell wears (the LEAVES cascade / the AGE / the
    /// STAGE properties the kind-only vocabulary cannot store — the
    /// steady-state pattern the mesher's own neighbor derivation
    /// answers).
    ///
    /// Returns whether the plant GREW (the caller's remesh batch rides).
    pub fn bamboo_random_tick(
        &mut self,
        rng: &mut crate::drops::Lcg,
        x: i32,
        y: i32,
        z: i32,
    ) -> bool {
        let kind = self.get(x, y, z);
        if is_bamboo_sapling(kind) {
            // the pc 0-8 roll FIRST — the draw the occupied cell still consumes
            if rng.next_int(3) != 0 {
                return false;
            }
            if self.get(x, y + 1, z) != AIR {
                return false; // the isEmptyBlock verdict pc 11-19
            }
            // the light-above gate pc 22-33 — the fold passes (the doc above)
            // growBamboo pc 0-28 + the updateShape conversion pc 17-43
            self.set(x, y + 1, z, BAMBOO);
            self.set(x, y, z, BAMBOO);
            true
        } else if is_bamboo_stalk(kind) {
            // the STAGE gate pc 0-16 — the fold the kind-only store keeps
            if rng.next_int(3) != 0 {
                return false; // the pc 17-25 roll
            }
            if self.get(x, y + 1, z) != AIR {
                return false; // the isEmptyBlock verdict pc 28-36
            }
            // the light-above gate pc 39-50 — the fold passes
            let height = self.bamboo_height_below(x, y, z) + 1;
            if height >= 16 {
                return false; // the pc 63-67 law — the 16-tall plant refuses
            }
            // growBamboo: the new top (the leaves cascade the kind-only
            // vocabulary cannot store — the derived-render steady state
            // the mesher's walk answers)
            self.set(x, y + 1, z, BAMBOO);
            true
        } else {
            false
        }
    }

    /// THE RANDOM TICK'S OWN DISPATCH (the ServerLevel's ONE
    /// randomTick per picked cell — the block class the kind verdict
    /// answers): the wheat cell the [`World::crop_random_tick`] arm,
    /// the bamboo pair the [`World::bamboo_random_tick`] arm, the
    /// rest the isRandomlyTicking refusal keeps (the zero-draw law
    /// the walk's 27-draw bare budget never sways).
    pub fn random_tick_block(
        &mut self,
        rng: &mut crate::drops::Lcg,
        x: i32,
        y: i32,
        z: i32,
    ) -> bool {
        let kind = self.get(x, y, z);
        if is_wheat(kind) {
            self.crop_random_tick(rng, x, y, z)
        } else if is_bamboo_stalk(kind) || is_bamboo_sapling(kind) {
            self.bamboo_random_tick(rng, x, y, z)
        } else {
            false
        }
    }

    /// THE RANDOM TICK WALK (the ServerLevel's per-tick lane the vanilla
    /// game law speaks: `randomTickSpeed = 3` — THREE random cells per
    /// 16³ chunk section per game tick, each the block's
    /// isRandomlyTicking/randomTick the crop lane answers). The offline
    /// world's own authority (the networked world the server ticks —
    /// the same split the mob lane speaks): every section the walk
    /// sweeps (the chunk-major order the tickChunk walk keeps), three
    /// `nextInt(16)` draws per axis the pick wears, the picked cell
    /// the [`World::random_tick_block`] dispatch answers (the wheat
    /// arm the crop lane, the bamboo pair the growth walk, the rest
    /// the zero-draw refusal keeps). Returns whether ANY block grew
    /// (the caller's remesh the batch grew speaks once).
    pub fn random_tick_walk(&mut self, rng: &mut crate::drops::Lcg) -> bool {
        let mut grew = false;
        let sections_y = CHUNK_Y / 16;
        for cx in 0..self.chunks_per_side {
            for cz in 0..self.chunks_per_side {
                for sy in 0..sections_y {
                    for _ in 0..3 {
                        let lx = rng.next_int(16);
                        let ly = rng.next_int(16);
                        let lz = rng.next_int(16);
                        let wx = cx as i32 * CHUNK_X as i32 + lx;
                        let wy = sy as i32 * 16 + ly;
                        let wz = cz as i32 * CHUNK_Z as i32 + lz;
                        grew |= self.random_tick_block(rng, wx, wy, wz);
                    }
                }
            }
        }
        grew
    }

    /// THE FIRE'S OWN SCHEDULE (javap `LevelChunkTicks.schedule` the
    /// 26.2 jar — the `ticksPerPosition` Set's `add`-returns-false
    /// dedup the pc 1-10 pair keeps): the (due, x, y, z) entry the
    /// queue gains UNLESS the cell already rides one (the ONE live
    /// entry per cell the Set's own membership law speaks — the re-arm
    /// the walk's opening draw claims, the ladder's onPlace schedule
    /// the slot taken drops, the exponential the queue never grows).
    /// The `ScheduledTick.equals` the type+pos pair alone compares —
    /// the kind the fire's own family alone wears (the two kinds the
    /// one-cell-at-a-time law never mixes) — the pos the dedup key.
    pub fn schedule_fire_tick(&mut self, due: u64, x: i32, y: i32, z: i32) {
        if self
            .fire_ticks
            .iter()
            .any(|&(_, px, py, pz)| px == x && py == y && pz == z)
        {
            return; // the Set's own verdict — the duplicate the add refuses
        }
        self.fire_ticks.push((due, x, y, z));
    }

    /// THE CAMPFIRE QUEUE'S OWN DRAIN (the fire queue's own shape the
    /// BE tick's every-tick cadence wears): the due-at-or-before-`now`
    /// entries the queue gives UP, the time order the stable sort
    /// banks. The caller's own walk the 11% gate + the 2..3 puff loop
    /// answers, the re-arm the due+1 entry the walk's own tail speaks
    /// ([`World::rearm_campfire_tick`]).
    pub fn drain_due_campfire_ticks(&mut self, now: u64) -> Vec<(i32, i32, i32)> {
        let mut due_cells: Vec<(u64, i32, i32, i32)> = Vec::new();
        self.campfire_ticks.retain(|&(at, x, y, z)| {
            if at <= now {
                due_cells.push((at, x, y, z));
                false
            } else {
                true
            }
        });
        due_cells.sort_by_key(|&(at, _, _, _)| at);
        due_cells
            .into_iter()
            .map(|(_, x, y, z)| (x, y, z))
            .collect()
    }

    /// THE CAMPFIRE WALK'S OWN RE-ARM (the BE ticker's every-tick
    /// cadence the due+1 entry banks — the walk each tick the gate
    /// rolls, the queue the entry the same dedup law keeps).
    pub fn rearm_campfire_tick(&mut self, due: u64, x: i32, y: i32, z: i32) {
        if self
            .campfire_ticks
            .iter()
            .any(|&(_, px, py, pz)| px == x && py == y && pz == z)
        {
            return; // the Set's own verdict — the duplicate the add refuses
        }
        self.campfire_ticks.push((due, x, y, z));
    }

    /// THE SIGNAL FIRE'S OWN VERDICT (javap CampfireBlock.isSmokeSource
    /// pc 228-231 over the 26.2 jar — the `state.is(Blocks.HAY_BLOCK)`
    /// read the SIGNAL_FIRE property's own law carries): the campfire
    /// cell's BELOW read the hay bale alone answers true. The vanilla
    /// state-cache (the getStateForPlacement bank + the updateShape
    /// DOWN-neighbor re-answer) the property stores — the offline
    /// kind-only palette the LIVE read the same verdict answers (the
    /// property's own updates track the below cell exactly, the
    /// per-tick read the value the cache never drifts from).
    pub fn is_signal_fire(&self, x: i32, y: i32, z: i32) -> bool {
        self.get(x, y - 1, z) == HAY_BLOCK
    }

    /// THE FIRE QUEUE'S OWN DRAIN (the `ServerLevel.tick` scheduled
    /// lane's own order the due ticks keep — the time order the
    /// LevelTicks' own sort wears): the due-at-or-before-`now` entries
    /// the queue gives UP (the retain's own removal the falling lane's
    /// drain speaks), the time order the stable sort banks. The due
    /// list the caller's own walks answer.
    pub fn drain_due_fire_ticks(&mut self, now: u64) -> Vec<(i32, i32, i32)> {
        let mut due_cells: Vec<(u64, i32, i32, i32)> = Vec::new();
        self.fire_ticks.retain(|&(at, x, y, z)| {
            if at <= now {
                due_cells.push((at, x, y, z));
                false
            } else {
                true
            }
        });
        due_cells.sort_by_key(|&(at, _, _, _)| at);
        due_cells
            .into_iter()
            .map(|(_, x, y, z)| (x, y, z))
            .collect()
    }

    /// THE FIRE'S OWN SHAPE UPDATE — javap `FireBlock.updateShape(state,
    /// level, sched, pos, dir, neighborPos, neighborState, random)` over
    /// the real 26.2 jar: the pc 0-8 `canSurvive` verdict the failed
    /// branch the pc 32-38 `Blocks.AIR.defaultBlockState()` serves (the
    /// surviving branch the pc 11-31 `getStateWithAge` re-derivation the
    /// mesher's own live-props walk the offline world never re-speaks),
    /// the CHAIN javap `Level.removeBlock` pc 12 `iconst_3` the flag-3
    /// setBlock the UPDATE_NEIGHBORS broadcast keeps — the dig's own
    /// `destroyBlock` and the burn-out's own two arms (the FIRE arm's
    /// pc 85 `iconst_3` setBlock, the AIR arm's pc 95-96
    /// `removeBlock(pos, false)`) every one the chain fires, the fire
    /// cells the changed block held the chain douses the SAME beat (the
    /// scheduled tick's own survival arm the 30..=39 lag the vanilla
    /// never waits: the updateShape the flag-3 walk answers). The
    /// changed cell itself the walk never reads (the chain targets the
    /// SIX neighbors the broadcast serves); the DYING cell's own
    /// setBlock the chain re-fires (the fire stack the support's loss
    /// kills upward the cascade the worklist walks — the vanilla's own
    /// recursion the depth the world's own bounds keep). The verdict
    /// the [`fire_can_survive`] law reads (the below-sturdy arm OR the
    /// six-neighbor burnable scan the 26.2 bytecodes speak). The AGE
    /// entries the dead cells drop (the ladder's own store); the
    /// SCHEDULE entries the block-match gate later drops (the stale
    /// rows the drain's own verdict refuses). Returns whether any fire
    /// cell died (the caller's own remesh the flag folds).
    pub fn fire_shape_updates(&mut self, changed: (i32, i32, i32)) -> bool {
        let (cx, cy, cz) = changed;
        // the DOWN..EAST order the Direction.values() walk keeps (the
        // verdict the per-cell read the order never gates)
        let mut queue: Vec<(i32, i32, i32)> = vec![
            (cx, cy - 1, cz),
            (cx, cy + 1, cz),
            (cx, cy, cz - 1),
            (cx, cy, cz + 1),
            (cx - 1, cy, cz),
            (cx + 1, cy, cz),
        ];
        let mut changed_any = false;
        while let Some((x, y, z)) = queue.pop() {
            if !is_fire(self.get(x, y, z)) {
                continue;
            }
            let below = self.get(x, y - 1, z);
            let neighbors = [
                self.get(x, y - 1, z), // DOWN
                self.get(x, y + 1, z), // UP
                self.get(x, y, z - 1), // NORTH
                self.get(x, y, z + 1), // SOUTH
                self.get(x - 1, y, z), // WEST
                self.get(x + 1, y, z), // EAST
            ];
            if fire_can_survive(below, neighbors) {
                continue; // the surviving branch — the props re-derivation the mesher's own live walk wears
            }
            self.set(x, y, z, AIR);
            self.fire_age.remove(&(x, y, z));
            changed_any = true;
            // THE CASCADE: the dying cell's own broadcast the chain
            // re-reads (the fire stack the support's loss the walk
            // kills — the vanilla's own flag-3 recursion)
            queue.extend([
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
                (x - 1, y, z),
                (x + 1, y, z),
            ]);
        }
        changed_any
    }

    /// THE PORTAL CELL'S OWN AXIS (javap NetherPortalBlock.AXIS — the
    /// EnumProperty the updateShape's flag11 verdict reads): the fill's
    /// own store the truth carries (the winning trial the bank wears),
    /// the store-less cell the mesher's own west/east short-circuit
    /// the derive fallback keeps (the autopush-605 animate arm's own
    /// verdict — the x-extending gate the sampled cell's west/east
    /// answers, the else the Z the default wears)
    fn portal_cell_axis_x(&self, pos: (i32, i32, i32)) -> bool {
        if let Some(&x) = self.portal_axis.get(&pos) {
            return x;
        }
        self.get(pos.0 - 1, pos.1, pos.2) == NETHER_PORTAL
            || self.get(pos.0 + 1, pos.1, pos.2) == NETHER_PORTAL
    }

    /// THE PORTAL'S OWN UPDATE-SHAPE DOUSE (javap NetherPortalBlock.
    /// updateShape pc 0-93 over the real 26.2 jar — the walk the
    /// changed neighbor's own beat serves; the canSurvive/onRemove
    /// overrides the class dump REFUSES, the updateShape the douse
    /// law ALONE carries). THE VERDICT the pc map keeps in order:
    ///
    /// 1. pc 0-17 — `axis9 = direction.getAxis()` the changed
    ///    neighbor's own axis, `axis10 = state.getValue(AXIS)` the
    ///    portal's own axis the store carries;
    /// 2. pc 19-39 — `flag11 = axis10 != axis9 && axis9.isHorizontal()`
    ///    the conjunction (the Y-direction change the vertical refusal
    ///    the flag never passes — the top/bottom frame's own loss the
    ///    douse path takes);
    /// 3. pc 41-66 — the KEEP disjunction: `flag11 || neighborState.
    ///    is(this) || PortalShape.findAnyShape(level, pos, axis10).
    ///    isComplete()` (the in-front block the flag11 arm keeps, the
    ///    sibling portal cell the is() arm keeps, the intact frame
    ///    the every-cell count the isComplete arm keeps);
    /// 4. pc 87-93 — the douse: the failed verdict the
    ///    `Blocks.AIR.defaultBlockState()` return serves.
    ///
    /// THE CASCADE the vanilla's own flag-3 recursion keeps: the
    /// doused cell the changed block its own neighbors re-read (the
    /// count the w*h equality the every-douse walk refuses — the
    /// whole interior the chain the tail cell's own store-backed
    /// axis the verdict keeps). The queue the (cell, changed
    /// neighbor) PAIR carries — the updateShape's own arg list the
    /// direction the pair's delta answers. The DOWN..EAST order the
    /// fire walk's own queue wears (the verdict the per-cell read
    /// the order never gates).
    pub fn portal_shape_updates(&mut self, changed: (i32, i32, i32)) -> bool {
        let (cx, cy, cz) = changed;
        let mut queue: Vec<((i32, i32, i32), (i32, i32, i32))> = vec![
            ((cx, cy - 1, cz), (cx, cy, cz)),
            ((cx, cy + 1, cz), (cx, cy, cz)),
            ((cx, cy, cz - 1), (cx, cy, cz)),
            ((cx, cy, cz + 1), (cx, cy, cz)),
            ((cx - 1, cy, cz), (cx, cy, cz)),
            ((cx + 1, cy, cz), (cx, cy, cz)),
        ];
        let mut changed_any = false;
        while let Some(((x, y, z), (nx, ny, nz))) = queue.pop() {
            if self.get(x, y, z) != NETHER_PORTAL {
                continue;
            }
            // THE NEIGHBORSTATE ARG — the changed cell's NEW kind the
            // verdict reads (the AIR the dig's own set serves, the
            // placed kind the placement's own set carries)
            let neighbor_kind = self.get(nx, ny, nz);
            // THE FLAG11 (pc 19-39): the delta the changed pair's own
            // axis answers — the Y the vertical refusal keeps, the
            // horizontal the axis pair the equality gate reads
            let axis10_x = self.portal_cell_axis_x((x, y, z));
            let flag11 = if ny != y {
                false
            } else {
                axis10_x != (nx != x)
            };
            // THE KEEP DISJUNCTION (pc 41-66): the flag11 arm + the
            // neighborState.is(this) arm + the isComplete arm the
            // findAnyShape walk (the lit portal's own re-scan the
            // EMPTY predicate never filters) banks
            let keeps = flag11
                || neighbor_kind == NETHER_PORTAL
                || self.portal_find_any_shape((x, y, z), axis10_x).is_complete();
            if keeps {
                continue;
            }
            // THE DOUSE (pc 87-93): the AIR the failed verdict serves,
            // the axis entry the map drops (the fire_age removal the
            // same law keeps)
            self.set(x, y, z, AIR);
            self.portal_axis.remove(&(x, y, z));
            changed_any = true;
            // THE CASCADE: the doused cell the changed block its own
            // neighbors re-read (the vanilla's own flag-3 recursion —
            // the count the w*h equality the every-douse walk refuses)
            queue.extend([
                ((x, y - 1, z), (x, y, z)),
                ((x, y + 1, z), (x, y, z)),
                ((x, y, z - 1), (x, y, z)),
                ((x, y, z + 1), (x, y, z)),
                ((x - 1, y, z), (x, y, z)),
                ((x + 1, y, z), (x, y, z)),
            ]);
        }
        changed_any
    }

    /// THE BAMBOO'S OWN SCHEDULE (the fire schedule's own dedup law —
    /// javap `LevelChunkTicks.schedule` the add-returns-false pair
    /// keeps): the (due, x, y, z) entry the queue gains UNLESS the
    /// cell already rides one. The stalk the ONE live entry the law
    /// keeps — the tower's stacked broadcasts the dedup refuses (the
    /// dig's own schedule + the pop's re-broadcast the same cell the
    /// duplicate entry never banks), the exponential the queue never
    /// grows.
    pub fn schedule_bamboo_tick(&mut self, due: u64, x: i32, y: i32, z: i32) {
        if self
            .bamboo_ticks
            .iter()
            .any(|&(_, px, py, pz)| px == x && py == y && pz == z)
        {
            return; // the Set's own verdict — the duplicate the add refuses
        }
        self.bamboo_ticks.push((due, x, y, z));
    }

    /// THE BAMBOO QUEUE'S OWN DRAIN (the fire drain's own shape — the
    /// `ServerLevel.tick` scheduled lane the due entries the time
    /// order the stable sort banks): the due-at-or-before-`now`
    /// entries the queue gives UP, the caller's own tick law the pop
    /// verdicts answer ([`World::bamboo_tick`]).
    pub fn drain_due_bamboo_ticks(&mut self, now: u64) -> Vec<(i32, i32, i32)> {
        let mut due_cells: Vec<(u64, i32, i32, i32)> = Vec::new();
        self.bamboo_ticks.retain(|&(at, x, y, z)| {
            if at <= now {
                due_cells.push((at, x, y, z));
                false
            } else {
                true
            }
        });
        due_cells.sort_by_key(|&(at, _, _, _)| at);
        due_cells
            .into_iter()
            .map(|(_, x, y, z)| (x, y, z))
            .collect()
    }

    /// THE BAMBOO'S OWN UPDATE-SHAPE WALK — javap the pair's
    /// updateShape over the real 26.2 jar, the two bytecodes the
    /// flag-3 broadcast the dig's own beat serves:
    ///
    /// **Stalk** (`BambooStalkBlock.updateShape` pc 0-19): the
    /// canSurvive verdict the UNCONDITIONAL head keeps (the direction
    /// the pc 20 UP gate the age cycle alone gates — the below read
    /// every changed neighbor re-answers); the failed branch the
    /// `scheduleTick(pos, this, 1)` lag arms (NO state change — the
    /// broadcast the schedule alone speaks, the walk's own recursion
    /// the schedule never fires).
    ///
    /// **Sapling** (`BambooSaplingBlock.updateShape` pc 0-16): the
    /// same verdict the failed branch the `Blocks.AIR.
    /// defaultBlockState()` RETURN serves — the IMMEDIATE death the
    /// one-tick lag never waits (the vanilla's own asymmetry the
    /// pair's two arms wear). The pc 17-42 conversion arm the
    /// UP-neighbor pair reads (direction == UP — the changed cell the
    /// sapling's own ABOVE is — and the neighborState.is(BAMBOO) the
    /// placed bamboo alone arms): the sapling itself becomes
    /// BAMBOO (the growth walk's own conversion the same law speaks).
    ///
    /// THE VERDICT the [`World::bamboo_can_survive`] law reads — the
    /// `below.is(SUPPORTS_BAMBOO)` read BOTH bytecodes' canSurvive
    /// speak (javap BambooStalkBlock.canSurvive pc 0-16 =
    /// BambooSaplingBlock.canSurvive pc 0-16 — the SAME walk the two
    /// classes wear). The SIX neighbors the broadcast serves (the
    /// DOWN..EAST order the Direction.values() walk keeps — the
    /// verdict the per-cell read the order never gates); the CHANGED
    /// cell itself the walk never reads. The sapling's death the
    /// CHAIN re-reads (the dying cell's own broadcast the queue
    /// extends — the stalk the sapling held the schedule arms, the
    /// vanilla's own flag-3 recursion). The STALK'S schedule the
    /// changed_any flag never sets (no state change — the mesh the
    /// schedule never re-speaks); the sapling's death the flag
    /// answers (the caller's own remesh folds).
    pub fn bamboo_shape_updates(&mut self, changed: (i32, i32, i32), due: u64) -> bool {
        let (cx, cy, cz) = changed;
        // the DOWN..EAST order the Direction.values() walk keeps
        let mut queue: Vec<(i32, i32, i32)> = vec![
            (cx, cy - 1, cz),
            (cx, cy + 1, cz),
            (cx, cy, cz - 1),
            (cx, cy, cz + 1),
            (cx - 1, cy, cz),
            (cx + 1, cy, cz),
        ];
        let changed_kind = self.get(cx, cy, cz);
        let mut changed_any = false;
        while let Some((x, y, z)) = queue.pop() {
            let kind = self.get(x, y, z);
            if is_bamboo_stalk(kind) {
                // THE STALK'S OWN HEAD (pc 0-19): the failed verdict
                // the scheduleTick's one-tick lag arms — the pop the
                // drain's own tick law answers
                if !self.bamboo_can_survive(x, y, z) {
                    self.schedule_bamboo_tick(due, x, y, z);
                }
            } else if is_bamboo_sapling(kind) {
                // THE SAPLING'S OWN ARMS (pc 0-16 the death, pc 17-42
                // the conversion): the below-support loss the AIR the
                // same beat serves, the placed-bamboo-above pair the
                // stalk the conversion wears
                if !self.bamboo_can_survive(x, y, z) {
                    self.set(x, y, z, AIR);
                    changed_any = true;
                    // THE CHAIN: the dying cell's own broadcast the
                    // stalk above re-reads (the vanilla's own flag-3
                    // recursion — the queue the same walk keeps)
                    queue.extend([
                        (x, y - 1, z),
                        (x, y + 1, z),
                        (x, y, z - 1),
                        (x, y, z + 1),
                        (x - 1, y, z),
                        (x + 1, y, z),
                    ]);
                } else if y == cy - 1 && x == cx && z == cz && changed_kind == BAMBOO {
                    // THE CONVERSION ARM (pc 17-42): the UP-neighbor
                    // read the direction the pair's delta answers —
                    // the changed cell the sapling's own above is, the
                    // bamboo kind the is() read arms — the sapling
                    // itself becomes BAMBOO (the defaultBlockState
                    // return the walk's own product wears)
                    self.set(x, y, z, BAMBOO);
                    changed_any = true;
                    queue.extend([
                        (x, y - 1, z),
                        (x, y + 1, z),
                        (x, y, z - 1),
                        (x, y, z + 1),
                        (x - 1, y, z),
                        (x + 1, y, z),
                    ]);
                }
            }
        }
        changed_any
    }

    /// THE BAMBOO'S OWN canSurvive — javap `BambooStalkBlock.
    /// canSurvive` pc 0-16 over the 26.2 jar: the below read the
    /// `BlockTags.SUPPORTS_BAMBOO` tag the verdict alone carries (the
    /// sapling's own canSurvive the IDENTICAL walk speaks — the same
    /// law the pair wears). The tag the palette subset the
    /// [`supports_bamboo`] walk keeps (the generated table the jar's
    /// tags/block/supports_bamboo.json banks).
    pub fn bamboo_can_survive(&self, x: i32, y: i32, z: i32) -> bool {
        supports_bamboo(self.get(x, y - 1, z))
    }

    /// THE BAMBOO'S OWN SCHEDULED TICK — javap `BambooStalkBlock.tick`
    /// pc 0-15 over the 26.2 jar, the walk the drain's own verdict
    /// keeps:
    ///
    /// 1. the canSurvive re-check the pop's own gate keeps — the
    ///    support the placement's own repair the verdict answers (the
    ///    dirt placed beneath the scheduled stalk the queue never
    ///    pops);
    /// 2. the failed verdict the `destroyBlock(pos, true)` walk
    ///    serves — the pop the drop=true arg wears (the loot the
    ///    caller's own lane rolls), the AIR the cell wears, the
    ///    flag-3 broadcast the pop's own re-read arms (the next
    ///    level's scheduleTick the cascade the one-per-tick rhythm
    ///    keeps — the tower the bottom-up crackle the vanilla's own
    ///    cadence speaks).
    ///
    /// THE BLOCK-MATCH GATE the stale rows the drain's own verdict
    /// refuses (the fire tick's own head the same law keeps): the
    /// kind-only cell the stalk kinds alone walk — the dug cell's
    /// stale entry the re-dig the queue never pops. Returns whether
    /// the cell POPPED (the caller's own voice/particles/drops the
    /// flag arms).
    pub fn bamboo_tick(&mut self, now: u64, x: i32, y: i32, z: i32) -> bool {
        if !is_bamboo_stalk(self.get(x, y, z)) {
            return false; // the block-match gate — the stale row the drain drops
        }
        if self.bamboo_can_survive(x, y, z) {
            return false; // the surviving arm — the repaired support the queue never pops
        }
        self.set(x, y, z, AIR);
        // THE POP'S OWN FLAG-3 BROADCAST (the destroyBlock walk the
        // setBlock keeps): the next level's schedule the cascade the
        // one-per-tick rhythm arms
        let _ = self.bamboo_shape_updates((x, y, z), now + 1);
        true
    }

    /// THE FIRE'S OWN SCHEDULED TICK — javap `FireBlock.tick(state,
    /// level, pos, random)` over the real 26.2 jar, the walk the
    /// tick's own pc map keeps in ORDER:
    ///
    /// 1. pc 0-10 `scheduleTick(pos, this, getFireTickDelay(random))`
    ///    — the re-arm FIRST (30 + nextInt(10), the queue's own Set
    ///    law the duplicate the slot claims);
    /// 2. pc 13-21 `canSpreadFireAround` — the FIRE_SPREAD_RADIUS
    ///    gamerule + the player-proximity gate the offline world's own
    ///    single player never refuses (the fold the gamerule's own -1
    ///    default the radius-less world wears);
    /// 3. pc 22-37 `canSurvive` — the removal the failed verdict
    ///    serves... the vanilla walk CONTINUES (the pc 38+ fall-through
    ///    the early return never wears): the rain arm's own draw and
    ///    the ladder's own roll the stream parity banks, the re-place
    ///    the flag-260 setBlock serves the isValid step's own
    ///    removeBlock the SAME walk erases (the AIR end-state the two
    ///    walks share — the fold the early return's own draws keep);
    /// 4. pc 77-127 the rain arm — `!infiniburn && isRaining &&
    ///    isNearRain && nextFloat() < 0.2F + age*0.03F` the removal
    ///    wears. THE FOLD the offline world keeps: isRaining the
    ///    `rain_level > 0.0` verdict speaks, the isNearRain walk's
    ///    canSeeSky/heightmap reads (javap `isNearRain` — the pos +
    ///    the four horizontal neighbors' isRainingAt the five reads
    ///    speak) the open-sky procedural plains the TRUE fold answers
    ///    (the heightmap walk the offline world never carries — the
    ///    crop lane's own light-gate fold the precedent wears);
    /// 5. pc 128-180 the ladder — `min(15, age + nextInt(3)/2)` the
    ///    new age the setBlock(260) stores; THE WALK'S OWN QUIRK the
    ///    bytecode's locals bank: the pc 62-75 `istore 7` age the
    ///    DOWNSTREAM reads all wear (pc 221/233/320/543/602 the
    ///    iload-7 family speaks) — the ladder's own product the NEXT
    ///    tick's pc 62-75 read alone answers, this walk's decisions
    ///    the PRE-LADDER age keeps;
    /// 6. pc 186-232 the isValid arm — `!isValidFireLocation` the
    ///    six-way canBurn scan refuses (the below cell the sturdy
    ///    UP-face row answers — the collision Full verdict the
    ///    [`fire_can_survive` ] law's own arm keeps): the `sturdy &&
    ///    age <= 3` pair the survival speaks, the else the removal
    ///    + return wears (the young flame the sturdy floor holds,
    ///    the grown flame the burnless floor douses);
    /// 7. pc 233-273 the age-15 arm — `age == 15 && nextInt(4) == 0
    ///    && !canBurn(below)` the removal wears (the draw the age-15
    ///    gate alone serves, the canBurn read the roll's own verdict
    ///    follows);
    /// 8. pc 274-419 the burn-out lane — the SIX checkBurnOut calls
    ///    the east/west/below/above/north/south order the bytecode's
    ///    own walk keeps (chance 300/300/250/250/300/300, the
    ///    INCREASED_FIRE_BURNOUT -50 bonus the nether attribute the
    ///    offline overworld never arms): every call ROLLS (the draw
    ///    the zero-odds neighbor never skips), the roll
    ///    `nextInt(chance) < burnOdds` the [`fire_burn_odds` ] table
    ///    answers; the inner walk (javap `checkBurnOut` pc 31-99):
    ///    `nextInt(age + 10) < 5 && !isRainingAt` the FIRE
    ///    placement's own arm serves (`min(15, age + nextInt(5)/4)`
    ///    the new cell's age wears — the onPlace law's own schedule
    ///    the queue's Set dedup banks), else the AIR removal; the
    ///    pre-removal state the TNT verdict reads (pc 100-120) —
    ///    `TntBlock.prime` the entity spawn the caller's own lane
    ///    answers (javap the private prime: the TNT_EXPLODES gate,
    ///    the `new PrimedTnt(level, x+0.5, y, z+0.5, null)` ctor +
    ///    the addFreshEntity + the TNT_PRIMED BLOCKS 1.0F/1.0F voice —
    ///    the removal the inner walk ALREADY served, the prime's own
    ///    walk no removeBlock wears);
    /// 9. pc 420-655 the spread loop — the x outer (-1..=1), z middle
    ///    (-1..=1), y inner (-1..=4) nesting the bytecode's own var
    ///    order banks (var12/13/14 the setWithOffset(x, y, z) args
    ///    speak), the (0,0,0) self the skip keeps, the chance
    ///    `100 + max(0, y-1)*100` the height falloff wears (the y=2..4
    ///    candidates the 200/300/400 chances speak); the candidate's
    ///    own walk (javap `getIgniteOdds(LevelReader, BlockPos)`):
    ///    the STRICT-AIR gate (the isEmptyBlock the pc 0-11 pair
    ///    serves — the plants/fluids the refuse keeps), the max over
    ///    the six neighbors the [`fire_ignite_odds` ] table answers,
    ///    the threshold `(odds + 40 + difficulty*7) / (age + 30)` the
    ///    integer division truncates (the difficulty the world's own
    ///    dial speaks), the `nextInt(chance) >= threshold` the roll
    ///    refuses, the rain the candidate skips (the isNearRain fold
    ///    the raining world wears), `min(15, age + nextInt(5)/4)` the
    ///    spread age the getStateWithAge's own FIRE the offline
    ///    soul-less palette banks (the SOUL_FIRE_BASE_BLOCKS tag the
    ///    palette's kind rows never name — the plain FIRE the verdict
    ///    always keeps, the flint round's own law the spread inherits).
    ///
    /// Returns the walk's own fruits: `changed` the remesh+relight
    /// batch the caller speaks once, `new_fire` the placed cells the
    /// onPlace law's own schedule the walk itself serves, `tnt_primed`
    /// the burnt TNT cells the caller's own entity lane spawns.
    pub fn fire_tick(
        &mut self,
        now: u64,
        x: i32,
        y: i32,
        z: i32,
        rng: &mut crate::drops::Lcg,
    ) -> FireTickOutcome {
        let mut out = FireTickOutcome::default();
        // THE BLOCK-MATCH GATE (the scheduled tick's own verdict — the
        // `block == getBlockState(pos).getBlock()` check the
        // ServerLevel's own walk keeps): the cell the fire kinds alone
        // walk, the dead cell's stale entry the drain's own gate drops
        if !is_fire(self.get(x, y, z)) {
            self.fire_age.remove(&(x, y, z));
            return out;
        }
        // 1. THE RE-ARM FIRST (pc 0-10 — the draw the walk's own
        // opening serves, the queue's Set law the duplicate refuses)
        let due = now + fire_tick_delay(rng);
        self.schedule_fire_tick(due, x, y, z);
        let raining = self.rain_level > 0.0;
        // THE WALK'S OWN QUIRK (pc 62-75 `istore 7` — the bytecode's
        // own local the DOWNSTREAM reads all wear): the pre-ladder age
        // every decision this walk keeps, the ladder's product the
        // NEXT tick's read alone answers
        let age = self.fire_age.get(&(x, y, z)).copied().unwrap_or(0) as i32;
        let below = self.get(x, y - 1, z);
        let neighbors = [
            self.get(x, y - 1, z), // DOWN
            self.get(x, y + 1, z), // UP
            self.get(x, y, z - 1), // NORTH
            self.get(x, y, z + 1), // SOUTH
            self.get(x - 1, y, z), // WEST
            self.get(x + 1, y, z), // EAST
        ];
        // 3. THE SURVIVAL'S OWN REFUSAL (pc 22-37 — the removal the
        // failed verdict serves, the vanilla's own fall-through the
        // draws the stream parity keeps: the rain arm's own nextFloat
        // and the ladder's own nextInt(3) the removed cell's walk
        // still banks — the re-place the flag-260 setBlock serves the
        // isValid step's own removeBlock the same walk erases)
        if !fire_can_survive(below, neighbors) {
            self.set(x, y, z, AIR);
            self.fire_age.remove(&(x, y, z));
            out.changed = true;
            if raining {
                let _ = rng.next_f32();
            }
            let _ = rng.next_int(3);
            return out;
        }
        // 4. THE RAIN ARM (pc 77-127 — the draw the gate's own verdict
        // alone serves, the 0.2F + age*0.03F fold the bytecode's own
        // arithmetic keeps)
        if raining && rng.next_f32() < 0.2 + age as f32 * 0.03 {
            self.set(x, y, z, AIR);
            self.fire_age.remove(&(x, y, z));
            out.changed = true;
            return out;
        }
        // 5. THE LADDER (pc 128-180 — the pre-ladder age the walk's
        // own decisions keep, the new age the store alone wears)
        let new_age = ((age + rng.next_int(3) / 2) as u8).min(FIRE_MAX_AGE);
        if new_age as i32 != age {
            self.fire_age.insert((x, y, z), new_age);
        }
        // 6. THE isValid ARM (pc 186-232 — the six-way scan the burn
        // burnable neighbors answer, the young flame the sturdy floor
        // holds, the grown the burnless floor douses)
        if !neighbors.iter().any(|&n| fire_can_burn(n)) {
            let sturdy = collision_shape(below) == CollisionShape::Full;
            if !(sturdy && age <= 3) {
                self.set(x, y, z, AIR);
                self.fire_age.remove(&(x, y, z));
                out.changed = true;
                return out;
            }
        }
        // 7. THE AGE-15 ARM (pc 233-273 — the roll the age-15 gate
        // alone serves — the vanilla's own sequence the isValid arm's
        // verdict NEVER gates: the arm runs whether the isValid scan
        // refused or passed, the young flames the gate's own age
        // verdict refuses), the canBurn read the roll's own verdict
        // follows
        if age == FIRE_MAX_AGE as i32
            && rng.next_int(4) == 0
            && !fire_can_burn(below)
        {
            self.set(x, y, z, AIR);
            self.fire_age.remove(&(x, y, z));
            out.changed = true;
            return out;
        }
        // 8. THE BURN-OUT LANE (pc 274-419 — the six calls the
        // bytecode's own order keeps, every call the roll wears)
        self.fire_check_burn_out(now, x + 1, y, z, 300, rng, age, &mut out); // east
        self.fire_check_burn_out(now, x - 1, y, z, 300, rng, age, &mut out); // west
        self.fire_check_burn_out(now, x, y - 1, z, 250, rng, age, &mut out); // below
        self.fire_check_burn_out(now, x, y + 1, z, 250, rng, age, &mut out); // above
        self.fire_check_burn_out(now, x, y, z - 1, 300, rng, age, &mut out); // north
        self.fire_check_burn_out(now, x, y, z + 1, 300, rng, age, &mut out); // south
        // 9. THE SPREAD LOOP (pc 420-655 — the x/z/y nesting the
        // bytecode's own var order banks, the height falloff the
        // chance arithmetic wears)
        let difficulty_id = self.difficulty.id() as i32;
        for dx in -1..=1i32 {
            for dz in -1..=1i32 {
                for dy in -1..=4i32 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue; // the (0,0,0) self the skip keeps
                    }
                    let chance = 100 + if dy > 1 { (dy - 1) * 100 } else { 0 };
                    let tx = x + dx;
                    let ty = y + dy;
                    let tz = z + dz;
                    if self.get(tx, ty, tz) != AIR {
                        continue; // the strict-AIR gate — no draw
                    }
                    let odds = [
                        self.get(tx, ty - 1, tz),
                        self.get(tx, ty + 1, tz),
                        self.get(tx, ty, tz - 1),
                        self.get(tx, ty, tz + 1),
                        self.get(tx - 1, ty, tz),
                        self.get(tx + 1, ty, tz),
                    ]
                    .into_iter()
                    .map(fire_ignite_odds)
                    .max()
                    .unwrap_or(0);
                    if odds <= 0 {
                        continue; // no draw
                    }
                    let threshold = (odds + 40 + difficulty_id * 7) / (age + 30);
                    if threshold <= 0 {
                        continue; // no draw
                    }
                    if rng.next_int(chance as u64) >= threshold {
                        continue;
                    }
                    if raining {
                        continue; // the isNearRain fold the raining world keeps
                    }
                    let spread_age =
                        ((age + rng.next_int(5) / 4) as u8).min(FIRE_MAX_AGE);
                    self.set(tx, ty, tz, FIRE);
                    self.fire_age.insert((tx, ty, tz), spread_age);
                    self.schedule_fire_tick(now + fire_tick_delay(rng), tx, ty, tz);
                    out.new_fire.push((tx, ty, tz));
                    out.changed = true;
                }
            }
        }
        out
    }

    /// THE BURN-OUT'S OWN ROLL (javap `checkBurnOut(level, pos, chance,
    /// random, age)` — the odds read the pc 0-9 pair serves, the roll
    /// the pc 11-21 compare wears, the inner walk the pc 24-120 body
    /// keeps): the roll `nextInt(chance) < burnOdds` the burnable
    /// table answers (the zero-odds neighbor the always-lose roll
    /// wears — the draw the stream keeps), the inner `nextInt(age+10)
    /// < 5 && !isRainingAt` the FIRE arm serves (the new cell's own
    /// age the `min(15, age + nextInt(5)/4)` walk banks, the onPlace
    /// law's own schedule the queue speaks), else the AIR removal; the
    /// pre-removal state the TNT verdict reads — the prime the
    /// caller's own entity lane answers (the removal the inner walk
    /// ALREADY served — the prime's own walk no removeBlock wears).
    fn fire_check_burn_out(
        &mut self,
        now: u64,
        x: i32,
        y: i32,
        z: i32,
        chance: i32,
        rng: &mut crate::drops::Lcg,
        age: i32,
        out: &mut FireTickOutcome,
    ) {
        let burn_odds = fire_burn_odds(self.get(x, y, z));
        if rng.next_int(chance as u64) >= burn_odds {
            return; // the roll's own refusal — the cell the fire never eats
        }
        let pre = self.get(x, y, z);
        if rng.next_int((age + 10) as u64) < 5 && self.rain_level <= 0.0 {
            let spread_age = ((age + rng.next_int(5) / 4) as u8).min(FIRE_MAX_AGE);
            self.set(x, y, z, FIRE);
            self.fire_age.insert((x, y, z), spread_age);
            self.schedule_fire_tick(now + fire_tick_delay(rng), x, y, z);
            out.new_fire.push((x, y, z));
            // THE FLAG-3 CHAIN THE FIRE ARM FIRES (pc 85 `iconst_3` —
            // the setBlock the broadcast keeps): the fire cells the
            // burnt block held (the log the cell above stood on) the
            // placement the SAME beat douses — the burnable the cell
            // wore the flame replaces, the canSurvive verdict the loss
            // the chain re-reads
            let _ = self.fire_shape_updates((x, y, z));
            // THE BAMBOO'S OWN RE-READ (the same flag-3 broadcast the
            // pair's updateShape serves — the stalk the burnt block
            // held the scheduleTick lag arms, the sapling the AIR the
            // same beat serves)
            let _ = self.bamboo_shape_updates((x, y, z), now + 1);
        } else {
            self.set(x, y, z, AIR);
            self.fire_age.remove(&(x, y, z));
            // THE REMOVAL'S OWN CHAIN (javap `Level.removeBlock` pc 12
            // `iconst_3` — the flag-3 setBlock the broadcast keeps):
            // the fire cells the burnt block held the removal the same
            // beat douses (the updateShape walk the scheduled tick's
            // 30..=39 lag never waits)
            let _ = self.fire_shape_updates((x, y, z));
            // THE BAMBOO'S OWN RE-READ (the same removal the pair's
            // updateShape serves — the support the fire ate the fall
            // arms)
            let _ = self.bamboo_shape_updates((x, y, z), now + 1);
        }
        out.changed = true;
        if pre == TNT {
            out.tnt_primed.push((x, y, z));
        }
    }

    /// The block clip — the vanilla `Level.clip` COLLIDER law over full
    /// cubes: the voxel traversal (Amanatides & Woo's own DDA, the
    /// `BlockGetter.clip` shape) from the eye along the view direction
    /// up to the interaction reach (the BLOCK_INTERACTION_RANGE
    /// attribute's 4.5, javap), skipping the non-colliding cells (the
    /// air and the fluid — the ClipContext.Fluid.NONE law: the water
    /// passes). The hit face is the ENTRY face (a +x step enters through
    /// the WEST face — the stepped axis's negative direction); the
    /// location sits exactly on the crossed boundary (the face plane's
    /// own coordinate — the cursor floats then name the face 0.0/1.0
    /// the vanilla's shape clip lands too). The start inside a solid
    /// cell IS the hit with `inside` = true (the vanilla's own
    /// eye-inside branch). `None` = the reach ended in open air (the
    /// miss — the honest no-hit).
    pub fn clip(
        &self,
        origin: (f64, f64, f64),
        dir: (f64, f64, f64),
        reach: f64,
    ) -> Option<ClipHit> {
        // THE FLUID-NONE LAW (the ClipContext.Fluid.NONE the block clip
        // speaks): neither fluid ever decides the hit — the water AND
        // the lava stay transparent to the pick ray
        self.clip_by(origin, dir, reach, |id| id != AIR && id != WATER && id != LAVA)
    }

    /// THE FLUID-ANY CLIP (the ClipContext.Fluid.ANY the boat's POV
    /// walk speaks — javap Item.getPlayerPOVHitResult's third arg): the
    /// fluids COUNT — the ray stops at the water's own surface (and the
    /// lava's), the solids the same lane keeps. The hit location rides
    /// the same DDA boundary the block walk speaks: for the full-cube
    /// fluid cell the project's vocabulary keeps, the shape's entry IS
    /// the cell boundary.
    pub fn clip_fluid_any(
        &self,
        origin: (f64, f64, f64),
        dir: (f64, f64, f64),
        reach: f64,
    ) -> Option<ClipHit> {
        self.clip_by(origin, dir, reach, |id| id != AIR)
    }

    /// THE FLUID-SOURCE CLIP (the `ClipContext.Fluid.SOURCE_ONLY` the
    /// falling body's fast-fall walk speaks — javap FallingBlockEntity.
    /// tick over the 26.2 jar: the `new ClipContext(new Vec3(xo, yo,
    /// zo), position(), COLLIDER, SOURCE_ONLY, this)` ray pc 154-198
    /// keeps): the ray stops at the collision shapes AND the SOURCE
    /// fluids — the client's kind-only world the every-fluid-a-source
    /// equivalence keeps (the flowing states the palette never banks),
    /// the Empty-shape cells (the plants, the torch, the tufts) the
    /// ray passes through, the water AND the lava the stop arms speak
    /// (the caller's own FluidTags.WATER read the lava's hit refuses —
    /// the post-check the tag's own verdict wears).
    pub fn clip_fluid_source_only(
        &self,
        origin: (f64, f64, f64),
        dir: (f64, f64, f64),
        reach: f64,
    ) -> Option<ClipHit> {
        self.clip_by(origin, dir, reach, |id| {
            id == WATER || id == LAVA || collision_shape(id) != CollisionShape::Empty
        })
    }

    /// The shared DDA walk (the clip's own traversal): the predicate
    /// decides the hit cells, everything else — the boundary math, the
    /// entry face, the exact location — the one walk both laws speak.
    fn clip_by(
        &self,
        origin: (f64, f64, f64),
        dir: (f64, f64, f64),
        reach: f64,
        solid: impl Fn(u16) -> bool,
    ) -> Option<ClipHit> {
        let len = (dir.0 * dir.0 + dir.1 * dir.1 + dir.2 * dir.2).sqrt();
        if len < 1e-9 {
            return None;
        }
        let dx = dir.0 / len;
        let dy = dir.1 / len;
        let dz = dir.2 / len;
        let mut x = origin.0.floor() as i32;
        let mut y = origin.1.floor() as i32;
        let mut z = origin.2.floor() as i32;
        if solid(self.get(x, y, z)) {
            return Some(ClipHit {
                pos: (x, y, z),
                face: 1, // UP — the eye-inside edge's own stand-in
                location: origin,
                inside: true,
            });
        }
        let step_x = if dx > 0.0 { 1i32 } else { -1i32 };
        let step_y = if dy > 0.0 { 1i32 } else { -1i32 };
        let step_z = if dz > 0.0 { 1i32 } else { -1i32 };
        let t_delta = |d: f64| if d != 0.0 { (1.0 / d).abs() } else { f64::INFINITY };
        let (t_dx, t_dy, t_dz) = (t_delta(dx), t_delta(dy), t_delta(dz));
        let next_t = |o: f64, d: f64, i: i32, step: i32| {
            if d == 0.0 {
                f64::INFINITY
            } else {
                let bound = if step > 0 { (i + 1) as f64 } else { i as f64 };
                (bound - o) / d
            }
        };
        let mut t_max_x = next_t(origin.0, dx, x, step_x);
        let mut t_max_y = next_t(origin.1, dy, y, step_y);
        let mut t_max_z = next_t(origin.2, dz, z, step_z);
        loop {
            // the nearest boundary decides the next cell; a crossing
            // beyond the reach is the miss (the traversal's own end)
            if t_max_x <= t_max_y && t_max_x <= t_max_z {
                if t_max_x > reach {
                    return None;
                }
                x += step_x;
                let t = t_max_x;
                t_max_x += t_dx;
                if solid(self.get(x, y, z)) {
                    let bx = if step_x > 0 { x as f64 } else { (x + 1) as f64 };
                    return Some(ClipHit {
                        pos: (x, y, z),
                        face: if step_x > 0 { 4 } else { 5 }, // WEST : EAST
                        location: (bx, origin.1 + dy * t, origin.2 + dz * t),
                        inside: false,
                    });
                }
            } else if t_max_y <= t_max_z {
                if t_max_y > reach {
                    return None;
                }
                y += step_y;
                let t = t_max_y;
                t_max_y += t_dy;
                if solid(self.get(x, y, z)) {
                    let by = if step_y > 0 { y as f64 } else { (y + 1) as f64 };
                    return Some(ClipHit {
                        pos: (x, y, z),
                        face: if step_y > 0 { 0 } else { 1 }, // DOWN : UP
                        location: (origin.0 + dx * t, by, origin.2 + dz * t),
                        inside: false,
                    });
                }
            } else {
                if t_max_z > reach {
                    return None;
                }
                z += step_z;
                let t = t_max_z;
                t_max_z += t_dz;
                if solid(self.get(x, y, z)) {
                    let bz = if step_z > 0 { z as f64 } else { (z + 1) as f64 };
                    return Some(ClipHit {
                        pos: (x, y, z),
                        face: if step_z > 0 { 2 } else { 3 }, // NORTH : SOUTH
                        location: (origin.0 + dx * t, origin.1 + dy * t, bz),
                        inside: false,
                    });
                }
            }
        }
    }

    fn height(&self, wx: i32, wz: i32) -> i32 {
        // Coarse 4x4 grid -> terraced, Minecraft-like rolling hills.
        let gx = wx.div_euclid(4);
        let gz = wz.div_euclid(4);
        8 + (hash2(gx as i64, gz as i64, self.seed) % 7) as i32 // 8..=14
    }

    fn generate(&mut self) {
        let n = self.size_blocks();
        for wx in 0..n {
            for wz in 0..n {
                let top = self.height(wx, wz);
                let beach = top <= WATER_LEVEL + 1;
                for y in 0..=top {
                    let id = if y == top {
                        if beach { SAND } else { GRASS }
                    } else if y >= top - 2 {
                        if beach { SAND } else { DIRT }
                    } else {
                        STONE
                    };
                    self.set(wx, y, wz, id);
                }
                if top < WATER_LEVEL {
                    for y in (top + 1)..=WATER_LEVEL {
                        self.set(wx, y, wz, WATER);
                    }
                } else if hash2(wx as i64, wz as i64, self.seed ^ 0x5EED) % 61 == 0 {
                    let lx = wx.rem_euclid(16);
                    let lz = wz.rem_euclid(16);
                    if (2..=13).contains(&lx) && (2..=13).contains(&lz) {
                        self.tree(wx, top, wz);
                    }
                }
            }
        }
        // THE PLAINS' OWN TALL GRASS (the autopush-413 round — the
        // patch_tall_grass_2 placed feature's honest stand-in — javap
        // the jar's worldgen json the noise machinery the client's own
        // hash lane never carries): every above-water grass column
        // rolls (the tree roll's own hash precedent, the seed's own
        // salt). The DENSITY the feature's own walk band: the
        // noise_threshold_count (7 above / 0 below the -0.8 line) x the
        // rarity-32 gate x the count-96 repeats over the trapezoid
        // spread — the ~21 placements per 16x16 chunk the favorable
        // band answers, the ~1/12 grass-column share the 256 columns
        // wear. The pass runs BEFORE the tuft walk (the vanilla step-9
        // row's own order — patch_tall_grass_2 the patch_bush/
        // flower_plains/patch_grass_plain row's FIRST speak, the AIR
        // gate the first-feature-wins order keeps). THE PAIR the
        // placeAt walk wears (javap DoublePlantBlock.placeAt: the
        // LOWER state the clicked cell setBlock speaks, the UPPER the
        // `pos.above()` setBlock adds — both cells the AIR gate the
        // feature's matching_block_tag predicate keeps).
        for wx in 0..n {
            for wz in 0..n {
                let top = self.height(wx, wz);
                if top <= WATER_LEVEL {
                    continue; // the beach/water columns the patch never plants
                }
                if self.get(wx, top, wz) != GRASS {
                    continue; // the canSurvive substrate the LOWER half demands
                }
                if hash2(wx as i64, wz as i64, self.seed ^ 0x5EED_F30) % 12 != 0 {
                    continue;
                }
                if self.get(wx, top + 1, wz) == AIR && self.get(wx, top + 2, wz) == AIR {
                    self.set(wx, top + 1, wz, TALL_GRASS);
                    self.set(wx, top + 2, wz, TALL_GRASS_UPPER);
                }
            }
        }
        // THE PLAINS' OWN GRASS TUFT (the autopush-412 round — the
        // patch_grass_plain + patch_bush placed features' honest
        // stand-in — javap the jar's worldgen json the noise machinery
        // the client's own hash lane never carries): every above-water
        // grass column rolls (the tree roll's own hash precedent, the
        // seed's own salt). The DENSITY the two features' own walk
        // bands: patch_grass_plain's noise_threshold_count (10 above /
        // 5 below the -0.8 line) x the count-32 repeats over the
        // random_offset spread — the ~1/3 grass-column coverage the
        // vanilla plains look wears; patch_bush's rarity-4 gate x the
        // count-24 spread — the ~1/43 the sparse rows keep. The pass
        // runs BEFORE the flower walk (the AIR gate the first-feature-
        // wins order keeps — the vanilla step-9 list's own shape).
        for wx in 0..n {
            for wz in 0..n {
                let top = self.height(wx, wz);
                if top <= WATER_LEVEL {
                    continue; // the beach/water columns the patch never plants
                }
                if self.get(wx, top, wz) != GRASS {
                    continue; // the canSurvive substrate the tuft family demands
                }
                let kind = if hash2(wx as i64, wz as i64, self.seed ^ 0x5EED_F20) % 43 == 0 {
                    BUSH
                } else if hash2(wx as i64, wz as i64, self.seed ^ 0x5EED_F21) % 3 == 0 {
                    SHORT_GRASS
                } else {
                    continue;
                };
                if self.get(wx, top + 1, wz) == AIR {
                    self.set(wx, top + 1, wz, kind);
                }
            }
        }
        // THE PLAINS' OWN FLOWER PATCH (the flower_plain placed feature's
        // honest stand-in — javap the jar's worldgen json the noise
        // machinery the client's own hash lane never carries): every
        // above-water grass column rolls (the tree roll's own hash
        // precedent, the seed's own salt), the pass the trees AFTER
        // walks (the trunk's own cell the AIR gate refuses — the flowers
        // the canopy's shade never refuses, the vanilla's own under-tree
        // blooms keep). The KIND the noise_threshold_provider's own
        // word answers: the default dandelion the else arm wears, the
        // 0.33333334 high-chance third the Util.getRandom pick keeps —
        // the second hash the uniform index stands in for (the
        // provider's own high_states list order the rows wear: poppy,
        // azure_bluet, oxeye_daisy, cornflower — the autopush-414 round
        // banks; the low-band tulip rows the `d0 < threshold` arm the
        // noise port's own round plants, the hash lane the walk never
        // carries).
        for wx in 0..n {
            for wz in 0..n {
                let top = self.height(wx, wz);
                if top <= WATER_LEVEL {
                    continue; // the beach/water columns the patch never plants
                }
                let roll =
                    hash2(wx as i64, wz as i64, self.seed ^ 0x5EED_F10) % 83;
                if roll != 0 {
                    continue;
                }
                let kind = if hash2(wx as i64, wz as i64, self.seed ^ 0x5EED_F11) % 3 == 0 {
                    match hash2(wx as i64, wz as i64, self.seed ^ 0x5EED_F12) % 4 {
                        1 => AZURE_BLUET,
                        2 => OXEYE_DAISY,
                        3 => CORNFLOWER,
                        _ => POPPY,
                    }
                } else {
                    DANDELION
                };
                if self.get(wx, top + 1, wz) == AIR && self.get(wx, top, wz) == GRASS {
                    self.set(wx, top + 1, wz, kind);
                }
            }
        }
    }

    fn tree(&mut self, wx: i32, ground: i32, wz: i32) {
        let trunk_top = ground + 4;
        for y in (ground + 1)..=trunk_top {
            self.set(wx, y, wz, LOG);
        }
        for dy in 2..=4i32 {
            for dx in -2..=2i32 {
                for dz in -2..=2i32 {
                    let dist = dx.abs() + dz.abs() + if dy == 4 { 1 } else { 0 };
                    if dist <= 3 && !(dx == 0 && dz == 0 && dy < 4) {
                        let (x, y, z) = (wx + dx, trunk_top - 4 + dy + 2, wz + dz);
                        if self.get(x, y, z) == AIR {
                            self.set(x, y, z, LEAVES);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // THE COLLISION SHAPE'S OWN ROW LAW (the table the placement's
    // obstruction walk consumes — the rows the enum's doc carries): the
    // noCollision() family the EMPTY shape answers, the farmland/dirt
    // path the 15/16 column, the full-cube registration the rest.
    #[test]
    fn the_collision_shape_rows_wear_the_no_collision_law() {
        // THE EMPTY FAMILY: the fluids, the cross plants, the crops,
        // the torch, the ground cover, the fire kinds (the blocks
        // clinit's noCollision() rows the two fire registrations
        // speak — the replaceable pass-through the body walks
        // THROUGH, the entityInside walk the move tail then brands)
        for id in [
            AIR, WATER, LAVA, TORCH, DANDELION, POPPY, AZURE_BLUET, OXEYE_DAISY, CORNFLOWER,
            ORANGE_TULIP, RED_TULIP, PINK_TULIP, WHITE_TULIP, SHORT_GRASS, BUSH, TALL_GRASS,
            TALL_GRASS_UPPER, LARGE_FERN, LARGE_FERN_UPPER, LEAF_LITTER, WHEAT, WHEAT_STAGE7,
            FIRE, SOUL_FIRE,
        ] {
            assert_eq!(
                collision_shape(id),
                CollisionShape::Empty,
                "the noCollision row {} wears",
                NAME_OF[id as usize]
            );
        }
        // THE COLUMN FAMILY: the farmland and the dirt path — the
        // Block.column(16, 0, 15) the two SHAPE constants speak
        assert_eq!(collision_shape(FARMLAND), CollisionShape::Column(15));
        assert_eq!(collision_shape(DIRT_PATH), CollisionShape::Column(15));
        // THE SOUL SAND'S OWN COLUMN (the autopush-598 round — javap
        // SoulSandBlock the 26.2 jar: the SHAPE
        // `Block.column(16.0d, 0.0d, 14.0d)` pcs 11-21 banks — the full
        // 0..1 footprint under the 14/16 band the 2px sink the feel
        // keeps, the getBlockSupportShape/getVisualShape the full
        // answers the support law never splits)
        assert_eq!(collision_shape(SOUL_SAND), CollisionShape::Column(14));
        // THE CAMPFIRE FAMILY'S OWN COLUMN (the dev-6 round — javap
        // CampfireBlock.getShape the SHAPE field returns, the static
        // init's `Block.column(16.0, 0.0, 7.0)` walk banks: the full
        // 0..1 footprint under the 0..7/16 band the Column row's own
        // vocabulary speaks — the four kinds the two registrations'
        // getShape overrides all answer)
        assert_eq!(collision_shape(CAMPFIRE), CollisionShape::Column(7));
        assert_eq!(collision_shape(CAMPFIRE_LIT), CollisionShape::Column(7));
        assert_eq!(collision_shape(SOUL_CAMPFIRE), CollisionShape::Column(7));
        assert_eq!(
            collision_shape(SOUL_CAMPFIRE_LIT),
            CollisionShape::Column(7)
        );
        // THE CANDLE CAKE PAIR'S OWN COLUMN (the autopush-523 round —
        // javap CandleCakeBlock.getShape the SHAPE field returns, the
        // static init's `Shapes.or(column(2.0, 8.0, 14.0), column(14.0,
        // 0.0, 8.0))` walk banks: the cake slab's 0..8/16 band the
        // Column row speaks — the walk-over feel the auto-step keeps;
        // the stick band the per-cell fold never speaks)
        assert_eq!(collision_shape(CANDLE_CAKE), CollisionShape::Column(8));
        assert_eq!(
            collision_shape(CANDLE_CAKE_LIT),
            CollisionShape::Column(8)
        );
        // THE CAKE LADDER'S OWN COLUMN (the autopush-524 round — javap
        // CakeBlock.getShape the SHAPES[bites] field returns, the
        // static init's `Block.boxes(MAX_BITES + 1, lambda$static$0)`
        // walk banks: the lambda's `Block.box(1 + bites*2, 0, 1, 15,
        // 8, 15)` the y-band 0..8 the Column row speaks, the x-start's
        // per-bite narrowing the per-cell vocabulary never carries —
        // the same documented gap the per-cell fold keeps)
        for id in [CAKE, CAKE_SLICE1, CAKE_SLICE2, CAKE_SLICE3, CAKE_SLICE4, CAKE_SLICE5, CAKE_SLICE6] {
            assert_eq!(
                collision_shape(id),
                CollisionShape::Column(8),
                "the cake band {} wears",
                NAME_OF[id as usize]
            );
        }
        // THE FULL-CUBE FAMILY: the default registration — the terrain,
        // the foliage cubes, the machines
        for id in [STONE, DIRT, GRASS, LOG, LEAVES, PALE_LEAVES, SAND, TNT, CRAFTING_TABLE] {
            assert_eq!(
                collision_shape(id),
                CollisionShape::Full,
                "the full-cube row {} wears",
                NAME_OF[id as usize]
            );
        }
    }

    #[test]
    fn the_cake_family_speaks_the_bites_ladder() {
        // THE LADDER'S OWN KINDS (the autopush-524 round — javap the
        // 26.2 jar: CakeBlock's ctor the DEFAULT state registers
        // BITES=0, the blockstate's "bites=N" rows block/cake_sliceN
        // resolve, the kind-only vocabulary the SEVEN stages wears):
        // the full kind the item places, the bitten stages the eat
        // lane's own kind swaps wear
        assert_eq!(CAKE, 67);
        assert_eq!(CAKE_SLICE6, 73);
        for id in [CAKE, CAKE_SLICE1, CAKE_SLICE2, CAKE_SLICE3, CAKE_SLICE4, CAKE_SLICE5, CAKE_SLICE6] {
            assert!(is_cake(id), "the family predicate {id} answers");
        }
        assert!(!is_cake(CANDLE_CAKE), "the candle cake the family never carries");
        assert!(!is_cake(STONE), "the stone the family never carries");
        // THE SHARED-ROW LAW (the world.rs NAME_OF rows the generated
        // REGISTRY_NAMES the cross-check asserts — every stage the
        // BASE "cake" row the dig/drop/sound/item lanes keep)
        for id in [CAKE, CAKE_SLICE1, CAKE_SLICE2, CAKE_SLICE3, CAKE_SLICE4, CAKE_SLICE5, CAKE_SLICE6] {
            assert_eq!(NAME_OF[id as usize], "cake", "stage {id} the base row wears");
        }
        // THE ITEM FORM THE REGISTRATION CARRIES (javap the blocks
        // clinit's BlockItemIds.CAKE row — the register walk the item
        // id speaks, the pick lane the getCloneItemStack's answer
        // grants): the placeable lookup the reverse walk the FULL
        // row's first position answers
        assert_eq!(World::placeable_block("cake"), Some(CAKE));
        let cake_item = mc_registry::item_id_by_name("cake").unwrap();
        for id in [CAKE, CAKE_SLICE1, CAKE_SLICE2, CAKE_SLICE3, CAKE_SLICE4, CAKE_SLICE5, CAKE_SLICE6] {
            assert_eq!(
                mc_registry::item_id_by_name(NAME_OF[id as usize]),
                Some(cake_item),
                "stage {id} the pick lane the cake item grants"
            );
        }
        // THE DIG ROW THE CLINIT ARM SPEAKS (the strength(0.5f) the
        // ldc 0.5f the dig lane answers, the WOOL row the sounds
        // table speaks — the generated tables' own rows)
        let (speed, _) = crate::dig::DigState::block_dig_row(CAKE);
        assert!((speed - 0.5).abs() < 1e-4, "the 0.5 strength the dig row wears");
        // THE DROPS THE LOOT TABLE SPEAKS (javap the jar's own
        // data/minecraft/loot_table/blocks/cake.json — the type +
        // random_sequence row the POOLS never carries: the cake the
        // dig lane alone answers, the bite the eat lane's own verb
        // serves — the honest absence the drops lookup keeps, the
        // self-entry the cake never drops)
        assert!(
            mc_registry::generated::block_drops::drops("cake").is_none(),
            "the empty loot table the honest absence keeps"
        );
        // THE BITE MAP THE EAT LANE RIDES (javap CakeBlock.eat pc
        // 55-106 — the bites<6 branch the +1 kind swap answers, the
        // max-bite branch the None the AIR clear wears, every
        // non-cake cell the PASS keeps)
        assert_eq!(cake_bite_next(CAKE), Some(CAKE_SLICE1));
        assert_eq!(cake_bite_next(CAKE_SLICE1), Some(CAKE_SLICE2));
        assert_eq!(cake_bite_next(CAKE_SLICE2), Some(CAKE_SLICE3));
        assert_eq!(cake_bite_next(CAKE_SLICE3), Some(CAKE_SLICE4));
        assert_eq!(cake_bite_next(CAKE_SLICE4), Some(CAKE_SLICE5));
        assert_eq!(cake_bite_next(CAKE_SLICE5), Some(CAKE_SLICE6));
        assert_eq!(cake_bite_next(CAKE_SLICE6), None, "the max bite the removeBlock wears");
        assert_eq!(cake_bite_next(STONE), None, "the stone the PASS keeps");
        assert_eq!(cake_bite_next(CANDLE_CAKE), None, "the candle cake the map never speaks");
    }

    /// THE FLINT'S OWN ODDS LAW (javap FireBlock's static init +
    /// canSurvive over the 26.2 jar — the setFlammable rows the
    /// ignite-odds map banks, the canSurvive walk the two arms ORs):
    /// the burnable rows the canBurn verdict wears (the odds > 0 the
    /// getIgniteOdds read answers), the NO-ROW silence the crop and
    /// the table keep, the sturdy arm the full-cube collision row
    /// speaks (the 15/16 columns the sub-one height refuses), the
    /// location arm the six-way scan banks (the Direction.values()
    /// order — the burnable neighbor ANY slot answers, the below
    /// cell the scan's own read repeats).
    #[test]
    fn the_fire_survival_the_odds_law_keeps() {
        // THE BURNABLE ROWS (the static init's own constants: the log
        // 5/5, the leaves 30/60, the litter 60/100, the tnt 15/100,
        // the flora 60/100 — the upper halves the SAME block the
        // odds never split)
        for id in [
            LOG, LEAVES, PALE_LEAVES, LEAF_LITTER, TNT, SHORT_GRASS, BUSH, TALL_GRASS,
            TALL_GRASS_UPPER, LARGE_FERN, LARGE_FERN_UPPER, DANDELION, POPPY, AZURE_BLUET,
            OXEYE_DAISY, CORNFLOWER, ORANGE_TULIP, RED_TULIP, PINK_TULIP, WHITE_TULIP,
        ] {
            assert!(
                fire_can_burn(id),
                "the ignite row {} the canBurn verdict wears",
                NAME_OF[id as usize]
            );
        }
        // THE SILENCE THE VERDICT KEEPS (the static init's own walk:
        // WHEAT and CRAFTING_TABLE ride NO setFlammable row — the
        // stone/dirt/ore/fluid kinds the same refusal wears)
        for id in [
            WHEAT, WHEAT_STAGE7, CRAFTING_TABLE, STONE, DIRT, GRASS, SAND, WATER, LAVA, BEDROCK,
        ] {
            assert!(
                !fire_can_burn(id),
                "the no-row silence {} keeps",
                NAME_OF[id as usize]
            );
        }
        // THE STURDY ARM: the full-cube below the UP face the verdict
        // wears — no burnable neighbor the scan needs
        let none = [AIR; 6];
        assert!(
            fire_can_survive(STONE, none),
            "the full-cube below the sturdy arm speaks"
        );
        // THE 15/16 COLUMN'S OWN REFUSAL (the farmland/dirt_path the
        // full-face math refuses — the sub-one height the shape's own
        // isFaceFull verdict keeps)
        assert!(
            !fire_can_survive(FARMLAND, none),
            "the 15/16 column the sturdy arm refuses"
        );
        // THE LOCATION ARM: the burnable neighbor ANY of the six slots
        // answers (the DOWN..EAST enum order the array's own walk
        // keeps — the below cell the scan's own read repeats)
        let mut with_log = none;
        with_log[5] = LOG; // EAST
        assert!(
            fire_can_survive(FARMLAND, with_log),
            "the east log the location arm speaks"
        );
        let mut with_below = none;
        with_below[0] = LEAVES; // DOWN — the sturdy refusal the burnable odds answer
        assert!(
            fire_can_survive(FARMLAND, with_below),
            "the below leaves the scan's own read speaks"
        );
        let mut with_upper = none;
        with_upper[1] = TALL_GRASS_UPPER; // UP — the half-kind word the same odds wear
        assert!(
            fire_can_survive(AIR, with_upper),
            "the above tuft the half-kind odds speak"
        );
        // THE STONE NEIGHBORHOOD'S OWN REFUSAL (no sturdy below, no
        // burnable neighbor — the FAIL the vanilla keeps)
        let mut stony = none;
        stony[2] = STONE;
        stony[3] = DIRT;
        stony[4] = CRAFTING_TABLE; // the no-row silence the burnable scan refuses
        assert!(
            !fire_can_survive(AIR, stony),
            "the stone neighborhood the survival refuses"
        );
    }

    /// THE FIRE'S OWN ANIMATE-TICK VERDICT — javap `BaseFireBlock.
    /// animateTick` (the 26.2 jar) the walk the BOTH fire kinds inherit:
    /// the 1/24 sound gate the two floats the passing body alone keeps,
    /// the below arm the canBurn || sturdy disjunction takes (the THREE
    /// smokes the y + d*0.5 + 0.5 interior band folds, the pc 193 goto
    /// return the side arms never reaches), the five side arms the
    /// skipped-below lane alone serves (the TWO smokes the passing
    /// gate's own ±0.1 inset folds speak), the soul fire's own verdict
    /// the EVERY-block answer burns (javap `SoulFireBlock.canBurn` —
    /// the iconst_1 the override speaks).
    #[test]
    fn the_fire_animate_tick_the_arms_the_folds_wear() {
        // THE BELOW ARM (the sturdy stone floor the gate's TRUE answer
        // wears): the THREE smokes the interior band folds, the side
        // arms the passing gate never reaches (the west log the lane's
        // own refusal proves — no fourth smoke the skipped arms speak)
        let mut body = crate::drops::Lcg::new(0xF117_F1A);
        let mut v = fire_animate_tick(false, STONE, [LOG, AIR, AIR, AIR, AIR], &mut body);
        assert_eq!(v.smokes.len(), 3, "the below arm's own three");
        let mut mirror = crate::drops::Lcg::new(0xF117_F1A);
        let gate = mirror.next_int(24) == 0;
        assert_eq!(v.sound, gate, "the 1/24 gate the draw speaks");
        if gate {
            let vol = 1.0 + mirror.next_f32();
            let pitch = mirror.next_f32() * 0.7 + 0.3;
            assert_eq!(v.volume, vol, "the volume the 1.0+f fold keeps");
            assert_eq!(v.pitch, pitch, "the pitch the f*0.7+0.3 fold keeps");
        }
        for _ in 0..3 {
            let dx = mirror.next_f64();
            let dy = mirror.next_f64();
            let dz = mirror.next_f64();
            let expect = (dx, dy * 0.5 + 0.5, dz);
            assert_eq!(
                v.smokes.remove(0), expect,
                "the interior fold (x+d, y+d*0.5+0.5, z+d) the bytecode speaks"
            );
        }
        // THE SIDE ARMS (the air floor the below-skip lane wears): the
        // west log the FIRST gate passes — the TWO smokes the x+d*0.1
        // inset folds keep, the east/north/south/above air the same
        // silence the canBurn refusal keeps
        let mut body = crate::drops::Lcg::new(0xF117_F1B);
        let mut v = fire_animate_tick(false, AIR, [LOG, AIR, AIR, AIR, AIR], &mut body);
        assert_eq!(v.smokes.len(), 2, "the west arm's own pair");
        let mut mirror = crate::drops::Lcg::new(0xF117_F1B);
        let _gate = mirror.next_int(24);
        for _ in 0..2 {
            let dx = mirror.next_f64();
            let dy = mirror.next_f64();
            let dz = mirror.next_f64();
            let expect = (dx * 0.1, dy, dz);
            assert_eq!(
                v.smokes.remove(0), expect,
                "the west inset (x + d*0.1, y+d, z+d) the pc 236 dadd keeps"
            );
        }
        // THE EAST ARM'S OWN dsub (the pc 333 the subtraction speaks):
        // the smoke the (x+1) - d*0.1 face folds — the offset the 1.0
        // ceiling the inset never passes
        let mut body = crate::drops::Lcg::new(0xF117_F1C);
        let mut v = fire_animate_tick(false, AIR, [AIR, LOG, AIR, AIR, AIR], &mut body);
        assert_eq!(v.smokes.len(), 2, "the east arm's own pair");
        let mut mirror = crate::drops::Lcg::new(0xF117_F1C);
        let _gate = mirror.next_int(24);
        for _ in 0..2 {
            let dx = mirror.next_f64();
            let dy = mirror.next_f64();
            let dz = mirror.next_f64();
            let expect = (1.0 - dx * 0.1, dy, dz);
            assert_eq!(
                v.smokes.remove(0), expect,
                "the east inset ((x+1) - d*0.1, y+d, z+d) the dsub keeps"
            );
        }
        // THE NORTH/SOUTH/ABOVE ARMS' OWN FOLDS (the z+d*0.1 dadd, the
        // (z+1) - d*0.1 and (y+1) - d*0.1 dsubs the pc 458/555/637
        // speak): the three faces the same pair law keeps
        for (seed, sides, fold, name) in [
            (
                0xF117_F1Du64,
                [AIR, AIR, LOG, AIR, AIR],
                2usize,
                "the north inset (x+d, y+d, z + d*0.1)",
            ),
            (
                0xF117_F1E,
                [AIR, AIR, AIR, LOG, AIR],
                3,
                "the south inset (x+d, y+d, (z+1) - d*0.1)",
            ),
            (
                0xF117_F1F,
                [AIR, AIR, AIR, AIR, LOG],
                4,
                "the above inset (x+d, (y+1) - d*0.1, z+d)",
            ),
        ] {
            let mut body = crate::drops::Lcg::new(seed);
            let mut v = fire_animate_tick(false, AIR, sides, &mut body);
            assert_eq!(v.smokes.len(), 2, "the arm's own pair");
            let mut mirror = crate::drops::Lcg::new(seed);
            let _gate = mirror.next_int(24);
            for _ in 0..2 {
                let dx = mirror.next_f64();
                let dy = mirror.next_f64();
                let dz = mirror.next_f64();
                let expect = match fold {
                    2 => (dx, dy, dz * 0.1),
                    3 => (dx, dy, 1.0 - dz * 0.1),
                    _ => (dx, 1.0 - dy * 0.1, dz),
                };
                assert_eq!(v.smokes.remove(0), expect, "{}", name);
            }
        }
        // THE SIDE ARMS' OWN MULTI-GATE (the all-burnable lane the five
        // faces speak): the TEN smokes the five pairs keep, the bytecode's
        // own west-east-north-south-above order the folds name
        let mut body = crate::drops::Lcg::new(0xF117_F20);
        let mut v = fire_animate_tick(false, AIR, [LOG; 5], &mut body);
        assert_eq!(v.smokes.len(), 10, "the five arms' own ten");
        let mut mirror = crate::drops::Lcg::new(0xF117_F20);
        let _gate = mirror.next_int(24);
        let folds: [fn(f64, f64, f64) -> (f64, f64, f64); 5] = [
            |dx, dy, dz| (dx * 0.1, dy, dz),
            |dx, dy, dz| (1.0 - dx * 0.1, dy, dz),
            |dx, dy, dz| (dx, dy, dz * 0.1),
            |dx, dy, dz| (dx, dy, 1.0 - dz * 0.1),
            |dx, dy, dz| (dx, 1.0 - dy * 0.1, dz),
        ];
        for fold in folds {
            for _ in 0..2 {
                let dx = mirror.next_f64();
                let dy = mirror.next_f64();
                let dz = mirror.next_f64();
                assert_eq!(v.smokes.remove(0), fold(dx, dy, dz), "the arm order");
            }
        }
        // THE SOUL FIRE'S OWN VERDICT (javap `SoulFireBlock.canBurn` —
        // the iconst_1 the EVERY-block answer speaks): the AIR floor the
        // below arm's own TRUE gate wears, the west log the unreachable
        // side lane never names
        let mut body = crate::drops::Lcg::new(0xF117_F21);
        let mut v = fire_animate_tick(true, AIR, [LOG, AIR, AIR, AIR, AIR], &mut body);
        assert_eq!(v.smokes.len(), 3, "the soul's own below arm");
        let mut mirror = crate::drops::Lcg::new(0xF117_F21);
        let _gate = mirror.next_int(24);
        for _ in 0..3 {
            let dx = mirror.next_f64();
            let dy = mirror.next_f64();
            let dz = mirror.next_f64();
            assert_eq!(
                v.smokes.remove(0),
                (dx, dy * 0.5 + 0.5, dz),
                "the soul fire's own interior fold"
            );
        }
        // THE SOUND GATE'S OWN BOUNDARY (the nextInt(24) the draw always
        // spends, the two floats the passing body alone keeps): the many
        // seeds the verdict's own laws keep — the volume the 1.0..2.0
        // band, the pitch the 0.3..1.0 band, the failed gate the zero
        // pair never draws
        let mut passed = 0;
        let mut failed = 0;
        for seed in 0u64..2000 {
            let mut body = crate::drops::Lcg::new(seed);
            let v = fire_animate_tick(false, STONE, [AIR; 5], &mut body);
            if v.sound {
                passed += 1;
                assert!(
                    (1.0..2.0).contains(&v.volume),
                    "the volume band {} the 1.0+f fold keeps",
                    v.volume
                );
                assert!(
                    (0.3..1.0).contains(&v.pitch),
                    "the pitch band {} the f*0.7+0.3 fold keeps",
                    v.pitch
                );
            } else {
                failed += 1;
                assert_eq!(v.volume, 0.0, "the failed gate the volume keeps");
                assert_eq!(v.pitch, 0.0, "the failed gate the pitch keeps");
            }
            // THE DRAW PARITY (the streams advance the same walk): the
            // body's post-verdict draw the mirror's own next answers —
            // the gate 1 + the passing floats 2 + the smokes 9 the count
            // the below arm spends
            let mut mirror = crate::drops::Lcg::new(seed);
            let gate = mirror.next_int(24) == 0;
            if gate {
                let _ = mirror.next_f32();
                let _ = mirror.next_f32();
            }
            for _ in 0..9 {
                let _ = mirror.next_f64();
            }
            assert_eq!(
                body.next_f64(),
                mirror.next_f64(),
                "the draw parity seed {} keeps",
                seed
            );
        }
        assert!(passed > 0 && failed > 0, "both gate answers the walk speaks");
        // THE 1/24 EXPECTED RATE (the gate's own odds the many-seed walk
        // answers — 1/24 the ~83 the 2000-seed count speaks)
        let rate = passed as f64 / (passed + failed) as f64;
        assert!(
            (rate - 1.0 / 24.0).abs() < 0.02,
            "the gate rate {} the 1/24 odds keep",
            rate
        );
    }


    /// THE FRAME SCAN'S OWN WALKS (javap `PortalShape` over the 26.2
    /// jar — the autopush-606 round's doc block): the minimal 2x3
    /// frame the corners the vanilla itself never demands, the EMPTY
    /// predicate the fire cell the scan's own start admits, the axis
    /// trials the X first the Z the failed filter's else keeps, the
    /// descent the clicked-above-floor cell wears, the rejects the
    /// narrow/tall/stone-floor/missing-top walks keep, the fill the
    /// interior the NETHER_PORTAL kind wears (the FIRE CELL the
    /// interior member the fill OVERWRITES, the lit gate the re-scan
    /// never refills).
    #[test]
    fn the_frame_scan_the_walks_the_fills_wear() {
        // THE CLEAR BOX (the generated terrain the frame's own
        // neighborhood never names — the scan's walks the cleared air
        // alone reads, the frame the explicit sets speak)
        let clear = |w: &mut World| {
            for x in 6..=13 {
                for y in 8..=16 {
                    for z in 1..=12 {
                        w.set(x, y, z, AIR);
                    }
                }
            }
        };
        // THE X-PLANE'S OWN FRAME: the floor row y=10 x in 8..=11, the
        // side columns x=8/x=11 y 11..=13, the top row y=14 x in
        // 8..=11 — the interior rows 11..=13 the ring's own three the
        // 2x3 shape wears, the CORNER CELLS the full-width rows keep
        // (the frame predicate the ring cells alone tests — the
        // corners the vanilla's own law never demands, the presence
        // the optional the scan never reads)
        let frame_x = |w: &mut World| {
            for y in 11..=13 {
                w.set(8, y, 4, OBSIDIAN);
                w.set(11, y, 4, OBSIDIAN);
            }
            for x in 8..=11 {
                w.set(x, 10, 4, OBSIDIAN);
                w.set(x, 14, 4, OBSIDIAN);
            }
        };
        let mut w = World::new(1, 0x600D);
        clear(&mut w);
        frame_x(&mut w);
        // THE FIRE'S OWN START: the scan the fire cell the interior
        // member admits (the EMPTY predicate the BlockTags.FIRE row
        // speaks — the just-placed flame the igniter arms bank)
        w.set(9, 11, 4, FIRE);
        let fill = w
            .find_empty_portal_shape((9, 11, 4))
            .expect("the 2x3 frame the scan answers");
        assert_eq!(fill.width, 2, "the interior the two columns keeps");
        assert_eq!(fill.height, 3, "the interior the three rows keeps");
        assert!(fill.axis_x, "the x-extending frame the first trial wears");
        assert_eq!(
            fill.bottom_left, (10, 11, 4),
            "the bottom-left the interior corner the EAST-edge walk \
             answers (javap calculateBottomLeft pc 73-81 — the \
             relative(opposite, d) the last interior column keeps)"
        );
        assert_eq!(
            fill.num_portal_blocks, 0,
            "the empty interior the count keeps"
        );
        assert!(fill.is_valid(), "the 2x3 the validity conjunction passes");
        // THE FILL (check_portal_spawn — javap BaseFireBlock.onPlace
        // pc 13-50): the interior the portal kind wears — the FIRE
        // CELL the scan's own start the interior member the fill
        // OVERWRITES
        assert!(
            w.check_portal_spawn((9, 11, 4)),
            "the fill the verdict speaks"
        );
        for (x, y) in [(9, 11), (10, 11), (9, 12), (10, 12), (9, 13), (10, 13)] {
            assert_eq!(
                w.get(x, y, 4),
                NETHER_PORTAL,
                "the interior cell ({x},{y}) the portal kind wears"
            );
        }
        // THE LIT GATE'S OWN REFUSAL (javap lambda$findEmptyPortal
        // Shape$0 — the numPortalBlocks == 0 the filled interior the
        // predicate refuses): the re-scan the same frame never
        // refills (the portal cells the EMPTY predicate admits — the
        // interior the count the gate reads)
        assert!(
            !w.check_portal_spawn((9, 12, 4)),
            "the filled interior the re-scan refuses"
        );
        for (x, y) in [(9, 11), (10, 11), (9, 12), (10, 12), (9, 13), (10, 13)] {
            assert_eq!(
                w.get(x, y, 4),
                NETHER_PORTAL,
                "the refused re-scan the cells keep"
            );
        }
        // THE Z-PLANE'S OWN FRAME (javap findPortalShape pc 26-58 —
        // the failed X-trial the Z-axis trial the else branch speaks,
        // the SOUTH rightDir the axis-Z arm answers)
        let mut wz = World::new(1, 0x600D);
        clear(&mut wz);
        for z in 6..=9 {
            wz.set(8, 10, z, OBSIDIAN);
            wz.set(8, 14, z, OBSIDIAN);
        }
        for y in 11..=13 {
            wz.set(8, y, 6, OBSIDIAN);
            wz.set(8, y, 9, OBSIDIAN);
        }
        wz.set(8, 11, 7, FIRE);
        let fill_z = wz
            .find_empty_portal_shape((8, 11, 7))
            .expect("the z-frame the scan answers");
        assert!(
            !fill_z.axis_x,
            "the z-extending frame the second trial wears"
        );
        assert_eq!(
            (fill_z.width, fill_z.height),
            (2, 3),
            "the z-interior the same band keeps"
        );
        assert_eq!(
            fill_z.bottom_left, (8, 11, 7),
            "the z-descent the NORTH-edge walk the adjacent cell keeps"
        );
        // THE NARROW'S OWN REFUSAL (javap calculateWidth pc 7-20 — the
        // d < 2 band the 0 keeps): the 1-wide interior the width walk
        // refuses
        let mut wn = World::new(1, 0x600D);
        clear(&mut wn);
        for y in 11..=13 {
            wn.set(8, y, 4, OBSIDIAN);
            wn.set(10, y, 4, OBSIDIAN);
        }
        for x in 8..=10 {
            wn.set(x, 10, 4, OBSIDIAN);
            wn.set(x, 14, 4, OBSIDIAN);
        }
        wn.set(9, 11, 4, FIRE);
        assert!(
            wn.find_empty_portal_shape((9, 11, 4)).is_none(),
            "the 1-wide interior the width band refuses"
        );
        // THE SHORT'S OWN REFUSAL (javap calculateHeight pc 22-49 —
        // the d < 3 band the 0 keeps): the 2-tall interior the height
        // walk refuses (the interior rows 11..=12 the top row y=13
        // the row walk's third the obsidian keeps)
        let mut ws = World::new(1, 0x600D);
        clear(&mut ws);
        for y in 11..=12 {
            ws.set(8, y, 4, OBSIDIAN);
            ws.set(11, y, 4, OBSIDIAN);
        }
        for x in 8..=11 {
            ws.set(x, 10, 4, OBSIDIAN);
            ws.set(x, 13, 4, OBSIDIAN);
        }
        ws.set(9, 11, 4, FIRE);
        assert!(
            ws.find_empty_portal_shape((9, 11, 4)).is_none(),
            "the 2-tall interior the height band refuses"
        );
        // THE STONE FLOOR'S OWN REFUSAL (javap getDistanceUntilEdge
        // AboveFrame pc 65-97 — the below-FRAME check the non-obsidian
        // floor the 0 keeps, the -1 the null the degenerate shape
        // wears): the stone under one interior cell the scan refuses
        let mut wf = World::new(1, 0x600D);
        clear(&mut wf);
        frame_x(&mut wf);
        wf.set(10, 10, 4, STONE); // the stone the interior floor cell wears
        wf.set(9, 11, 4, FIRE);
        assert!(
            wf.find_empty_portal_shape((9, 11, 4)).is_none(),
            "the stone floor the edge walk refuses"
        );
        // THE MISSING TOP'S OWN REFUSAL (javap hasTopFrame pc 0-62 —
        // the row ABOVE the measured height the every-column FRAME
        // test answers, the missing top the 0 the refusal keeps): the
        // side columns the y=13 the ring's own last the top row
        // y=14 never speaks
        let mut wt = World::new(1, 0x600D);
        clear(&mut wt);
        for y in 11..=13 {
            wt.set(8, y, 4, OBSIDIAN);
            wt.set(11, y, 4, OBSIDIAN);
        }
        for x in 8..=11 {
            wt.set(x, 10, 4, OBSIDIAN);
        }
        wt.set(9, 11, 4, FIRE);
        assert!(
            wt.find_empty_portal_shape((9, 11, 4)).is_none(),
            "the missing top row the hasTopFrame refuses"
        );
        // THE MISSING SIDE'S OWN REFUSAL (javap getDistanceUntilEdge
        // AboveFrame — the empty cells the below-FRAME floor the
        // cleared air the walk refuses): the absent east column the
        // opposite-edge walk exhausts
        let mut we = World::new(1, 0x600D);
        clear(&mut we);
        for y in 11..=13 {
            we.set(8, y, 4, OBSIDIAN); // the west column only
        }
        for x in 8..=11 {
            we.set(x, 10, 4, OBSIDIAN);
            we.set(x, 14, 4, OBSIDIAN);
        }
        we.set(9, 11, 4, FIRE);
        assert!(
            we.find_empty_portal_shape((9, 11, 4)).is_none(),
            "the missing east column the edge walk refuses"
        );
        // THE DESCENT'S OWN WALK (javap calculateBottomLeft pc 17-46 —
        // the while isEmpty(below) loop the air column descends): the
        // scan from the interior's TOP row the same shape answers —
        // the clicked-above-floor cell the same bottom-left keeps
        let mut wd = World::new(1, 0x600D);
        clear(&mut wd);
        frame_x(&mut wd);
        let fill_d = wd
            .find_empty_portal_shape((9, 13, 4))
            .expect("the top-start scan the descent answers");
        assert_eq!(
            fill_d.bottom_left.1, 11,
            "the descent the interior floor keeps"
        );
        assert_eq!(
            (fill_d.width, fill_d.height),
            (2, 3),
            "the descended walk the same band keeps"
        );
        assert_eq!(
            fill_d.bottom_left, (10, 11, 4),
            "the descended walk the same interior corner keeps"
        );
    }

    /// THE PORTAL'S OWN UPDATE-SHAPE DOUSE (javap NetherPortalBlock.
    /// updateShape pc 0-93 over the real 26.2 jar — the changed
    /// neighbor's own beat the douse walk serves): the flag11 arm the
    /// in-front block keeps, the intact frame the isComplete arm the
    /// every-cell count keeps, the side/top frame's own loss the
    /// whole interior the cascade douses, the interior block the
    /// count the w*h equality refuses, the axis store the fill
    /// banks the douse drops.
    #[test]
    fn the_portal_the_frame_the_douse_the_chain_wears() {
        // THE STAGE CLOSURES (the frame-scan test's own shapes the
        // same law wears): the 2x3 interior x-plane frame the fill
        // walk answers
        let clear = |w: &mut World| {
            for x in 6..=13 {
                for y in 8..=16 {
                    for z in 1..=12 {
                        w.set(x, y, z, AIR);
                    }
                }
            }
        };
        let frame_x = |w: &mut World| {
            for y in 11..=13 {
                w.set(8, y, 4, OBSIDIAN);
                w.set(11, y, 4, OBSIDIAN);
            }
            for x in 8..=11 {
                w.set(x, 10, 4, OBSIDIAN);
                w.set(x, 14, 4, OBSIDIAN);
            }
        };
        let lit = |w: &mut World| {
            clear(w);
            frame_x(w);
            w.set(9, 11, 4, FIRE);
            assert!(w.check_portal_spawn((9, 11, 4)), "the fill the verdict speaks");
        };
        // THE FLAG11 ARM'S OWN KEEP (javap pc 19-66 — the changed
        // neighbor the Z-direction the portal's own X-axis the flag
        // the in-front block's own placement wears): the placed block
        // the plane's own face the verdict never douses
        let mut w = World::new(1, 0x600D);
        lit(&mut w);
        w.set(9, 12, 5, STONE);
        assert!(
            !w.portal_shape_updates((9, 12, 5)),
            "the in-front block the flag11 arm keeps (axis X vs the Z change)"
        );
        for (x, y) in [(9, 11), (10, 11), (9, 12), (10, 12), (9, 13), (10, 13)] {
            assert_eq!(
                w.get(x, y, 4),
                NETHER_PORTAL,
                "the kept portal cell ({x},{y}) the fill wears"
            );
        }
        // THE IS-COMPLETE CONJUNCTION'S OWN UNIT (javap PortalShape.
        // isComplete pc 0-28 — `isValid() && numPortalBlocks == width
        // * height` the pair the verdict keeps): the lit interior the
        // every-cell count the equality keeps, the doused cell the
        // count the walk refuses
        let mut w = World::new(1, 0x600D);
        lit(&mut w);
        let fill = w.portal_find_any_shape((9, 12, 4), true);
        assert!(
            fill.is_complete(),
            "the lit interior the every-cell count the w*h equality keeps"
        );
        assert_eq!(
            fill.num_portal_blocks,
            fill.width * fill.height,
            "the count the equality's own product speaks"
        );
        // THE STONE-PATCHED FLOOR'S OWN DOUSE (javap getDistanceUntil
        // EdgeAboveFrame pc 65-97 — the empty cell the below-FRAME
        // check the non-obsidian floor the walk refuses): the bottom
        // frame's stone patch the width walk refuses, the verdict the
        // douse serves (the stone floor the scan's own refusal the
        // frame-scan test's own law wears)
        w.set(9, 10, 4, STONE);
        assert!(
            w.portal_shape_updates((9, 10, 4)),
            "the stone-patched floor the below-FRAME check refuses"
        );
        for (x, y) in [(9, 11), (10, 11), (9, 12), (10, 12), (9, 13), (10, 13)] {
            assert_eq!(
                w.get(x, y, 4),
                AIR,
                "the interior cell ({x},{y}) the refused scan douses"
            );
        }
        // THE IS() ARM'S OWN KEEP (javap pc 46-52 — the neighborState
        // .is(this) verdict the sibling portal cell keeps, the
        // disjunction's own order the arm's TRUE the isComplete
        // consult never reaches): the direct-set portal cell the
        // bottom frame replaces, the above cell the arm keeps
        let mut w = World::new(1, 0x600D);
        lit(&mut w);
        w.set(9, 10, 4, NETHER_PORTAL); // a portal cell the frame cell replaces
        assert!(
            !w.portal_shape_updates((9, 10, 4)),
            "the sibling portal cell the is() arm keeps"
        );
        assert_eq!(
            w.get(9, 11, 4),
            NETHER_PORTAL,
            "the kept cell the fill wears"
        );
        // THE SIDE FRAME'S OWN LOSS (javap pc 87-93 — the failed
        // verdict the AIR return serves): the broken wall cell the
        // width walk the gap reads, the height the 1 the band refuses,
        // the WHOLE interior the cascade douses (the doused cell the
        // changed block the neighbors re-read), the ring the fill
        // never touches, the axis entries the map drops
        let mut w = World::new(1, 0x600D);
        lit(&mut w);
        assert_eq!(
            w.portal_axis.len(),
            6,
            "the fill the every-cell axis entry banks"
        );
        w.set(8, 12, 4, AIR); // the west wall's middle cell the dig breaks
        assert!(
            w.portal_shape_updates((8, 12, 4)),
            "the broken wall the douse verdict speaks"
        );
        for (x, y) in [(9, 11), (10, 11), (9, 12), (10, 12), (9, 13), (10, 13)] {
            assert_eq!(
                w.get(x, y, 4),
                AIR,
                "the interior cell ({x},{y}) the cascade douses"
            );
        }
        assert!(w.portal_axis.is_empty(), "the dead cells' entries the map drops");
        // THE RING'S OWN INTEGRITY (the douse the frame cells never
        // touches — the dug gap the only loss the walk wears)
        for y in 11..=13 {
            assert_eq!(w.get(11, y, 4), OBSIDIAN, "the east wall the walk keeps");
        }
        for (x, y) in [(8, 11), (8, 13), (9, 10), (10, 10), (9, 14), (10, 14)] {
            assert_eq!(w.get(x, y, 4), OBSIDIAN, "the ring cell ({x},{y}) the walk keeps");
        }
        // THE TOP FRAME'S OWN LOSS (javap calculateHeight pc 35-46 —
        // the hasTopFrame consult the missing top the 0 keeps, the
        // height band the invalid shape the verdict refuses): the
        // Y-direction change the flag11 never passes, the top gap the
        // whole interior the cascade douses
        let mut w = World::new(1, 0x600D);
        lit(&mut w);
        w.set(9, 14, 4, AIR); // the top row's middle cell the dig breaks
        assert!(
            w.portal_shape_updates((9, 14, 4)),
            "the missing top the douse verdict speaks"
        );
        for (x, y) in [(9, 11), (10, 11), (9, 12), (10, 12), (9, 13), (10, 13)] {
            assert_eq!(
                w.get(x, y, 4),
                AIR,
                "the interior cell ({x},{y}) the top-gap cascade douses"
            );
        }
        // THE INTERIOR BLOCK'S OWN REFUSAL (javap pc 55-66 — the
        // count the w*h equality the placed cell the walk refuses):
        // the replaced portal cell the sibling cells the cascade
        // douses
        let mut w = World::new(1, 0x600D);
        lit(&mut w);
        w.set(9, 12, 4, STONE);
        assert!(
            w.portal_shape_updates((9, 12, 4)),
            "the interior block the count refuses"
        );
        for (x, y) in [(10, 11), (10, 12), (10, 13), (9, 11), (9, 13)] {
            assert_eq!(
                w.get(x, y, 4),
                AIR,
                "the sibling cell ({x},{y}) the short count douses"
            );
        }
        assert_eq!(w.get(9, 12, 4), STONE, "the placed block the walk keeps");
        // THE Z-PLANE'S OWN DOUSE (the z-extending portal the same law
        // wears — the axis store the second trial banks, the front
        // block the X-direction change the flag11 keeps): the z-frame
        // the single column x=8 the interior z 7..=8 wears
        let mut wz = World::new(1, 0x600D);
        clear(&mut wz);
        for z in 6..=9 {
            wz.set(8, 10, z, OBSIDIAN);
            wz.set(8, 14, z, OBSIDIAN);
        }
        for y in 11..=13 {
            wz.set(8, y, 6, OBSIDIAN);
            wz.set(8, y, 9, OBSIDIAN);
        }
        wz.set(8, 11, 7, FIRE);
        assert!(wz.check_portal_spawn((8, 11, 7)), "the z-fill the verdict speaks");
        wz.set(8, 12, 10, STONE); // the block in FRONT of the z-plane the X arm reads
        assert!(
            !wz.portal_shape_updates((8, 12, 10)),
            "the z-portal's front block the flag11 arm keeps (axis Z vs the X change)"
        );
        wz.set(8, 12, 6, AIR); // the z-frame's near wall the dig breaks
        assert!(
            wz.portal_shape_updates((8, 12, 6)),
            "the broken z-wall the douse verdict speaks"
        );
        for z in 7..=8 {
            for y in 11..=13 {
                assert_eq!(
                    wz.get(8, y, z),
                    AIR,
                    "the z-interior cell (8,{y},{z}) the cascade douses"
                );
            }
        }
    }

    /// THE NETHER'S OWN WORLD BUILD (the dimension swap's own
    /// destination the new build answers): the has_skylight false the
    /// light walk's sky-column skip reads (javap DimensionType
    /// .hasSkylight — the nether's false the pass never runs), the
    /// bedrock floor y=0 + ceiling y=47 the real dimension's roof the
    /// 48-tall grid scales, the lava sea the level below fills, the
    /// sky grid the ZERO the gate keeps (the overworld's own the
    /// contrast speaks).
    #[test]
    fn the_nether_the_world_the_skylight_the_sea_wears() {
        let w = World::new_nether(2, 0xC0FFEE);
        assert!(!w.has_skylight, "the nether's own flag the build banks");
        assert_eq!(
            w.get(0, 0, 0),
            BEDROCK,
            "the floor the bedrock kind keeps"
        );
        assert_eq!(
            w.get(7, 47, 3),
            BEDROCK,
            "the ceiling the bedrock kind keeps"
        );
        // THE SEA'S OWN FILL (the level below the cells the lava kind
        // keeps — the ocean the overworld's water walk never wears)
        let mut sea = 0usize;
        for x in 0..w.size_blocks() {
            for z in 0..w.size_blocks() {
                for y in 1..NETHER_LAVA_LEVEL {
                    if w.get(x, y, z) == LAVA {
                        sea += 1;
                    }
                }
            }
        }
        assert!(sea > 0, "the lava sea the level's own cells keep");
        // THE SKY GRID'S OWN CONTRAST (the same grid the two builds
        // wear): the overworld's open column the 15 speaks, the
        // nether's own the ZERO the gate keeps — the compute walk the
        // has_skylight flag reads
        let ow = World::new(2, 0xC0FFEE);
        let ow_light = crate::light::compute(&ow);
        let n_light = crate::light::compute(&w);
        // the top open cell (1, 47, 1): the overworld's own the 15
        // keeps, the nether's ceiling cell the 0 wears
        let cell = 1 + 1 * CHUNK_X + 47 * CHUNK_X * CHUNK_Z;
        assert_eq!(ow_light.sky[0][cell], 15, "the overworld sky lights");
        assert_eq!(n_light.sky[0][cell], 0, "the nether sky stays dark");
        // THE BLOCK LIGHT'S OWN SURVIVAL (the gate the sky pass alone
        // gates — the lava's own 15 the flood keeps)
        let mut lava_glow = false;
        for ci in 0..n_light.block.len() {
            if n_light.block[ci].iter().any(|&v| v == 15) {
                lava_glow = true;
                break;
            }
        }
        assert!(lava_glow, "the lava's own glow the block flood keeps");
    }

    /// THE END'S OWN WORLD BUILD (the autopush-612 round — the
    /// dimension's own row the registry banks, the deterministic
    /// fiction the nether build's own shape wears): the island the
    /// disc the center raises, the taper the cone thins, the void the
    /// rim's outside keeps, the pillars the ring raises, the exit
    /// pedestal the bedrock rows the center wears, the dimension's
    /// own light rows the jar's stated values stamp.
    #[test]
    fn the_end_the_world_the_island_the_void_wears() {
        let w = World::new_end(2, 0xC0FFEE);
        let n = w.size_blocks();
        // THE DIMENSION'S OWN LIGHT ROWS (the jar's the_end.json the
        // registry row banks — the has_skylight TRUE the 26.2 data
        // keeps, the stated factor/color/ambient the build stamps)
        assert!(w.has_skylight, "the end's own sky gate the jar row keeps");
        assert_eq!(
            w.sky_light_factor,
            Some(0.0),
            "the stated sky factor the sky term pins"
        );
        assert_eq!(
            w.sky_light_color,
            Some([172, 96, 205]),
            "the #ac60cd the stated moonlight base keeps"
        );
        assert_eq!(
            w.ambient_light,
            Some([63, 71, 63]),
            "the #3f473f the stated ambient base keeps"
        );
        // THE ISLAND'S OWN CELLS (the center the pedestal wears — the
        // bedrock rows the fountain's shape keeps, the island body
        // the end_stone kind below keeps, the column the cap the AIR
        // above answers)
        let (cx, cz) = ((n as f64 / 2.0).round() as i32, (n as f64 / 2.0).round() as i32);
        // THE PORTAL'S OWN DIAMOND (the dev-29 round — the interior the
        // END_PORTAL cells the exit portal's own shape keeps: the
        // center + the four orthogonal arms the starry field wears,
        // the ring the bedrock cells keep — the fountain's own base
        // rows the vanilla structure's law banks)
        assert_eq!(w.get(cx, END_ISLAND_TOP, cz), END_PORTAL, "the portal's own center cell");
        assert_eq!(w.get(cx + 1, END_ISLAND_TOP, cz), END_PORTAL, "the portal's own east arm");
        assert_eq!(w.get(cx - 1, END_ISLAND_TOP, cz), END_PORTAL, "the portal's own west arm");
        assert_eq!(w.get(cx, END_ISLAND_TOP, cz + 1), END_PORTAL, "the portal's own south arm");
        assert_eq!(w.get(cx, END_ISLAND_TOP, cz - 1), END_PORTAL, "the portal's own north arm");
        assert_eq!(w.get(cx, END_ISLAND_TOP + 3, cz), BEDROCK, "the column's own head");
        assert_eq!(
            w.get(cx, END_ISLAND_TOP + 4, cz),
            AIR,
            "the column the AIR cap keeps"
        );
        assert_eq!(
            w.get(cx, END_ISLAND_TOP - 1, cz),
            END_STONE,
            "the island body the pedestal's below keeps"
        );
        // THE PEDESTAL'S OWN RING (the manhattan band the ±2 cells
        // keep, the +3 cell the band's outside the island keeps)
        assert_eq!(
            w.get(cx - 2, END_ISLAND_TOP, cz),
            BEDROCK,
            "the ring's own arm the band keeps"
        );
        assert_ne!(
            w.get(cx - 3, END_ISLAND_TOP, cz),
            BEDROCK,
            "the band's outside the ring refuses"
        );
        // THE VOID'S OWN LAW (the corner the rim never fills — NO
        // bedrock floor the dimension keeps, every depth the AIR
        // answers)
        for y in 1..CHUNK_Y {
            assert_eq!(
                w.get(0, y as i32, 0),
                AIR,
                "the void's own column the y={} keeps",
                y
            );
        }
        // THE TAPER'S OWN CONE (the center's full depth the rim's
        // thin keeps — the island's bottom the taper's own band
        // banks)
        let dist_center = ((cx as f64 - (n as f64 - 1.0) / 2.0).powi(2)
            + (cz as f64 - (n as f64 - 1.0) / 2.0).powi(2))
        .sqrt();
        assert!(dist_center < 1.0, "the center cell the island's own middle keeps");
        let depth = ((n as f64 / 4.0 - dist_center) / (n as f64 / 4.0)
            * END_ISLAND_DEPTH as f64)
            .ceil() as i32;
        let bottom = (END_ISLAND_TOP - depth).max(1);
        assert_eq!(
            w.get(cx, bottom, cz),
            END_STONE,
            "the cone's own bottom the center's depth keeps"
        );
        assert_eq!(
            w.get(cx, bottom - 1, cz),
            AIR,
            "the cone's below the taper never fills"
        );
        // THE PILLAR RING'S OWN COLUMNS (the obsidian the ring raises
        // — the θ=0 arm the ring's own band keeps, the heights the
        // hash roll varies, the island's TOP the ground the build
        // keeps)
        let mut obsidian = 0usize;
        for x in 0..n {
            for z in 0..n {
                for y in 0..CHUNK_Y {
                    if w.get(x, y as i32, z) == OBSIDIAN {
                        obsidian += 1;
                    }
                }
            }
        }
        assert!(
            obsidian >= 10 * 2 * 2 * 4,
            "the ring's ten 2x2 columns the ≥4-tall band raises — got {}",
            obsidian
        );
        // the θ=0 arm's own column (the ring px = round(cx' + ring) the
        // build's own walk computes — the obsidian the top cell keeps,
        // the head the AIR above answers)
        let ring = n as f64 / 4.0 * 0.72;
        let px = ((n as f64 - 1.0) / 2.0 + ring).round() as i32;
        let pz = ((n as f64 - 1.0) / 2.0).round() as i32;
        assert_eq!(
            w.get(px, END_ISLAND_TOP, pz),
            OBSIDIAN,
            "the θ=0 pillar's own base the ring raises"
        );
        // THE KIND'S OWN ROWS (the palette id, the NAME_OF row, the
        // generated atlas mirror the regen re-binds, the dig row
        // (3.0, true) the clinit arm speaks, the drops self-entry the
        // generated table banks, the item the reverse walk answers)
        assert_eq!(END_STONE, 89);
        assert_eq!(NAME_OF[END_STONE as usize], "end_stone");
        assert_eq!(
            mc_registry::generated::atlas::KIND_NAMES[END_STONE as usize],
            "end_stone",
            "the atlas mirror the regen re-binds"
        );
        assert_eq!(
            crate::dig::DigState::block_dig_row(END_STONE),
            (3.0, true),
            "the clinit's strength(3.0f, 9.0f) + requiresCorrectToolForDrops the rows bank"
        );
        assert!(
            mc_registry::generated::block_drops::drops("end_stone").is_some(),
            "the self-entry the drops table banks"
        );
        assert!(
            mc_registry::item_id_by_name("end_stone").is_some(),
            "the item form the placeable lookup answers"
        );
        // THE DIG FRACTIONS' OWN PAIR (javap the destroy walk — the
        // bare hand's 3.0/100 crawl the correct-tool 30 divisor
        // serves)
        let bare = crate::dig::DigState::destroy_progress(1.0, 3.0, false);
        let pick = crate::dig::DigState::destroy_progress(1.0, 3.0, true);
        assert!((bare - 1.0 / 300.0).abs() < 1e-6, "the bare crawl the 100 keeps");
        assert!((pick - 1.0 / 90.0).abs() < 1e-6, "the pick divisor the 30 serves");
        assert!(pick > bare, "the correct tool the progress hastens");
    }

    /// THE END'S OWN SKY GRID (the autopush-612 round — the
    /// has_skylight TRUE the column pass runs, the void's own column
    /// the 15 keeps, the stated factor 0.0 the lightmap's sky term
    /// the mesher's multiply pins — the grid fills, the brightness
    /// the zero never feeds; the nether's own zero the gate keeps).
    #[test]
    fn the_end_the_sky_grid_the_factor_pins() {
        let end = World::new_end(2, 0xC0FFEE);
        let nether = World::new_nether(2, 0xC0FFEE);
        let end_light = crate::light::compute(&end);
        let nether_light = crate::light::compute(&nether);
        // the void corner's open column (1, 47, 1): the end's own the
        // 15 keeps (the flag true the pass runs), the nether's the 0
        // wears (the flag false the pass never runs)
        let cell = 1 + 1 * CHUNK_X + 47 * CHUNK_X * CHUNK_Z;
        assert_eq!(
            end_light.sky[0][cell],
            15,
            "the end's void column the sky pass fills"
        );
        assert_eq!(
            nether_light.sky[0][cell],
            0,
            "the nether's ceiling cell the gate keeps dark"
        );
        // THE ISLAND'S OWN HEAD (the open cell above the pillar tops
        // the chunk (1,1) keeps — the sky the island's own columns
        // never shade)
        let head = 4 + 4 * CHUNK_X + 47 * CHUNK_X * CHUNK_Z;
        assert_eq!(
            end_light.sky[3][head],
            15,
            "the island's own head the sky fills"
        );
        // THE FACTOR'S OWN PIN (the mesher's sky-term multiply the
        // world's stated row feeds — the unwrap_or default 1.0 the
        // overworld keeps, the end's 0.0 the lightmap's sky term the
        // zero wears)
        assert_eq!(end.sky_light_factor.unwrap_or(1.0), 0.0);
        assert_eq!(
            World::new(2, 0xC0FFEE).sky_light_factor.unwrap_or(1.0),
            1.0,
            "the overworld's None default the identity keeps"
        );
    }

    /// THE RETURN PORTAL'S OWN BUILD (the PortalForcer fiction — the
    /// swap's deterministic stand-in the vanilla createPortal's own
    /// shape keeps): the 4x5 obsidian frame the interior 2x3 the
    /// platform floor wears, the interior the
    /// check_portal_spawn walk's own scan lights (the num_portal_blocks
    /// 0 the empty-shape verdict banks, the 6 cells the fill speaks).
    #[test]
    fn the_frame_the_builder_the_interior_the_fill_wears() {
        let mut w = World::new(2, 7);
        // THE CLEAR STAGE (the flat pad the build's own site keeps)
        for x in 4..=12 {
            for y in 6..=16 {
                for z in 4..=12 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        for x in 4..=12 {
            for z in 4..=12 {
                w.set(x, 5, z, STONE);
            }
        }
        let (ix, iy, iz) = w.build_portal_frame((6, 6, 8));
        assert_eq!((ix, iy, iz), (7, 7, 8), "the interior's start cell");
        // THE FRAME'S OWN CELLS (the 4x5 obsidian the shape keeps)
        for i in 0..4 {
            assert_eq!(w.get(6 + i, 6, 8), OBSIDIAN, "the bottom row");
            assert_eq!(w.get(6 + i, 10, 8), OBSIDIAN, "the top row");
        }
        for j in 1..4 {
            assert_eq!(w.get(6, 6 + j, 8), OBSIDIAN, "the west column");
            assert_eq!(w.get(9, 6 + j, 8), OBSIDIAN, "the east column");
        }
        // THE PLATFORM'S OWN ROW (the descent's stop the build wears)
        for i in 0..4 {
            assert_eq!(w.get(6 + i, 5, 8), NETHERRACK, "the platform row");
        }
        // THE FILL (the check_portal_spawn walk the built frame
        // answers — the empty interior the verdict speaks)
        assert!(
            w.check_portal_spawn((ix, iy, iz)),
            "the fill the verdict speaks"
        );
        for (x, y) in [(7, 7), (8, 7), (7, 8), (8, 8), (7, 9), (8, 9)] {
            assert_eq!(
                w.get(x, y, 8),
                NETHER_PORTAL,
                "the interior cell ({x},{y}) the fill wears"
            );
        }
        // THE AXIS STORE'S OWN BANK (the fill's own walk the winning
        // trial's axis keeps — the X trial the first answer wears)
        assert!(
            w.portal_axis.contains_key(&(7, 7, 8)),
            "the axis the store banks"
        );
    }

    /// THE PORTAL'S OWN ANIMATE VERDICT (javap `NetherPortalBlock.
    /// animateTick` over the 26.2 jar): the 1/100 sound gate the FIXED
    /// 0.5 volume + the [0.8, 1.2) pitch band the passing body keeps,
    /// the four-iteration particle arm the base trio + the velocity
    /// trio + the ±1 pick + the neighbor arm's replacement draw the
    /// exact folds pin, the draw parity the streams' own advance keeps.
    #[test]
    fn the_portal_animate_tick_the_arms_the_folds_wear() {
        // THE X-EXTENDING ARM (the west neighbor the gate's own answer
        // wears): the FOUR purples the no-gate arm keeps — the z the
        // 0.5 + 0.25*±1 snap folds, the vz the nextFloat*2*±1 the
        // out-of-plane push carries, the x/y the raw doubles speak
        let mut body = crate::drops::Lcg::new(0xF117_F30);
        let mut v = portal_animate_tick(true, false, &mut body);
        assert_eq!(v.purples.len(), 4, "the four iterations the arm keeps");
        let mut mirror = crate::drops::Lcg::new(0xF117_F30);
        let gate = mirror.next_int(100) == 0;
        assert_eq!(v.sound, gate, "the 1/100 gate the draw speaks");
        if gate {
            let pitch = mirror.next_f32() * 0.4 + 0.8;
            assert_eq!(v.pitch, pitch, "the pitch the f*0.4+0.8 fold keeps");
        }
        for _ in 0..4 {
            let dx = mirror.next_f64();
            let dy = mirror.next_f64();
            let _dz = mirror.next_f64();
            let vx = (f64::from(mirror.next_f32()) - 0.5) * 0.5;
            let vy = (f64::from(mirror.next_f32()) - 0.5) * 0.5;
            let _vz = (f64::from(mirror.next_f32()) - 0.5) * 0.5;
            let pick = mirror.next_int(2) as i64 * 2 - 1;
            let pick = pick as f64;
            let rvz = f64::from(mirror.next_f32() * 2.0f32 * pick as f32);
            let expect = (dx, dy, 0.5 + 0.25 * pick, vx, vy, rvz);
            assert_eq!(
                v.purples.remove(0), expect,
                "the x-arm fold (x+d, y+d, z+0.5+0.25*±1, vx, vy, r*2*±1) the bytecode speaks"
            );
        }
        // THE Z-EXTENDING ARM (no portal neighbors the else lane wears):
        // the x the 0.5 + 0.25*±1 snap folds, the vx the replacement
        // push carries, the z the raw double keeps
        let mut body = crate::drops::Lcg::new(0xF117_F31);
        let mut v = portal_animate_tick(false, false, &mut body);
        assert_eq!(v.purples.len(), 4, "the four iterations the else arm keeps");
        let mut mirror = crate::drops::Lcg::new(0xF117_F31);
        assert_eq!(v.sound, mirror.next_int(100) == 0, "the 1/100 gate the same seed speaks");
        if v.sound {
            assert_eq!(v.pitch, mirror.next_f32() * 0.4 + 0.8, "the pitch the fold keeps");
        }
        for _ in 0..4 {
            let _dx = mirror.next_f64();
            let dy = mirror.next_f64();
            let dz = mirror.next_f64();
            let _vx = (f64::from(mirror.next_f32()) - 0.5) * 0.5;
            let vy = (f64::from(mirror.next_f32()) - 0.5) * 0.5;
            let vz = (f64::from(mirror.next_f32()) - 0.5) * 0.5;
            let pick = mirror.next_int(2) as i64 * 2 - 1;
            let pick = pick as f64;
            let rvx = f64::from(mirror.next_f32() * 2.0f32 * pick as f32);
            let expect = (0.5 + 0.25 * pick, dy, dz, rvx, vy, vz);
            assert_eq!(
                v.purples.remove(0), expect,
                "the z-arm fold (x+0.5+0.25*±1, y+d, z+d, r*2*±1, vy, vz) the pc 258-291 keeps"
            );
        }
        // THE EAST SHORT-CIRCUIT (the east neighbor alone the same
        // X-extending verdict wears — the || the west read the skipped
        // lane the verdict's own args never consult)
        let mut body = crate::drops::Lcg::new(0xF117_F32);
        let mut v = portal_animate_tick(false, true, &mut body);
        let mut mirror = crate::drops::Lcg::new(0xF117_F32);
        assert_eq!(v.sound, mirror.next_int(100) == 0, "the 1/100 gate the same seed speaks");
        if v.sound {
            assert_eq!(v.pitch, mirror.next_f32() * 0.4 + 0.8, "the pitch the fold keeps");
        }
        for _ in 0..4 {
            let dx = mirror.next_f64();
            let dy = mirror.next_f64();
            let _dz = mirror.next_f64();
            let vx = (f64::from(mirror.next_f32()) - 0.5) * 0.5;
            let vy = (f64::from(mirror.next_f32()) - 0.5) * 0.5;
            let _vz = (f64::from(mirror.next_f32()) - 0.5) * 0.5;
            let pick = mirror.next_int(2) as i64 * 2 - 1;
            let pick = pick as f64;
            let rvz = f64::from(mirror.next_f32() * 2.0f32 * pick as f32);
            assert_eq!(
                v.purples.remove(0),
                (dx, dy, 0.5 + 0.25 * pick, vx, vy, rvz),
                "the east arm's own z-snap the same fold keeps"
            );
        }
        // THE SOUND GATE'S OWN BOUNDARY (the nextInt(100) the draw
        // always spends, the one float the passing body alone keeps):
        // the many seeds the verdict's own laws keep — the pitch the
        // 0.8..1.2 band, the failed gate the zero pitch never draws
        let mut passed = 0;
        let mut failed = 0;
        for seed in 0u64..2000 {
            let mut body = crate::drops::Lcg::new(seed);
            let v = portal_animate_tick(false, false, &mut body);
            assert_eq!(v.purples.len(), 4, "the four purples seed {seed} keeps");
            if v.sound {
                passed += 1;
                assert!(
                    (0.8..1.2).contains(&v.pitch),
                    "the pitch band {} the f*0.4+0.8 fold keeps",
                    v.pitch
                );
            } else {
                failed += 1;
                assert_eq!(v.pitch, 0.0, "the failed gate the pitch keeps");
            }
            // THE DRAW PARITY (the streams advance the same walk): the
            // body's post-verdict draw the mirror's own next answers —
            // the gate 1 + the passing float 1 + the purples 32 the
            // count the four iterations spend
            let mut mirror = crate::drops::Lcg::new(seed);
            if mirror.next_int(100) == 0 {
                let _ = mirror.next_f32();
            }
            for _ in 0..32 {
                let _ = mirror.next_f64();
            }
            assert_eq!(
                body.next_f64(),
                mirror.next_f64(),
                "the draw parity seed {} keeps",
                seed
            );
        }
        assert!(passed > 0 && failed > 0, "both gate answers the walk speaks");
        // THE 1/100 EXPECTED RATE (the gate's own odds the many-seed
        // walk answers — 1/100 the ~20 the 2000-seed count speaks)
        let rate = passed as f64 / (passed + failed) as f64;
        assert!(
            (rate - 1.0 / 100.0).abs() < 0.02,
            "the gate rate {} the 1/100 odds keep",
            rate
        );
    }

    /// THE ODDS TABLES' OWN ROWS — javap `FireBlock.bootStrap()` the
    /// 175 rows the static init's own walk banks (the `setFlammable(
    /// block, ignite, burn)` pairs the dump speaks): the ignite column
    /// the spread threshold's own numerator wears, the burn column the
    /// checkBurnOut roll's own verdict reads, the canBurn verdict the
    /// odds' own truth (`getIgniteOdds(state) > 0`) the parity keeps —
    /// every kind the tables and the verdict agree, the no-row silence
    /// the stone/dirt/fluid/crop family wears.
    #[test]
    fn the_fire_tables_bank_the_bootstrap_odds() {
        // THE IGNITE COLUMN (the pairs' first half: the log 5, the
        // leaves family 30, the litter 60, the tnt 15, the flora 60)
        assert_eq!(fire_ignite_odds(LOG), 5, "the log the 5/5 pair keeps");
        assert_eq!(fire_ignite_odds(LEAVES), 30);
        assert_eq!(fire_ignite_odds(PALE_LEAVES), 30);
        assert_eq!(fire_ignite_odds(LEAF_LITTER), 60);
        assert_eq!(fire_ignite_odds(TNT), 15, "the tnt the 15/100 pair keeps");
        // THE WOOL FAMILY'S OWN ROW (the dev-22 round — the collection
        // lambda's 30/60 pair the odds columns speak)
        assert_eq!(fire_ignite_odds(WHITE_WOOL), 30, "the wool the 30/60 pair keeps");
        assert_eq!(fire_can_burn(WHITE_WOOL), true, "the wool the burnable verdict keeps");
        // THE WEB'S OWN SILENCE (the setFlammable walk the Web row never
        // speaks — the cobweb the zero odds the refusal wears)
        assert_eq!(fire_ignite_odds(COBWEB), 0, "the web the silence keeps");
        assert_eq!(fire_burn_odds(COBWEB), 0, "the web the silence keeps");
        assert_eq!(fire_can_burn(COBWEB), false, "the web the refusal keeps");
        // THE SOUL SAND'S OWN SILENCE (the autopush-598 round — javap
        // FireBlock the 26.2 jar: NO SOUL_SAND row the full dump's grep
        // answers, the setFlammable walk the nether sand never wears —
        // the nether's own stone the same silence keeps)
        assert_eq!(fire_ignite_odds(SOUL_SAND), 0, "the soul sand the silence keeps");
        assert_eq!(fire_burn_odds(SOUL_SAND), 0, "the soul sand the silence keeps");
        assert_eq!(fire_can_burn(SOUL_SAND), false, "the soul sand the refusal keeps");
        for id in [
            SHORT_GRASS, BUSH, TALL_GRASS, TALL_GRASS_UPPER, LARGE_FERN, LARGE_FERN_UPPER,
            DANDELION, POPPY, AZURE_BLUET, OXEYE_DAISY, CORNFLOWER, ORANGE_TULIP, RED_TULIP,
            PINK_TULIP, WHITE_TULIP,
        ] {
            assert_eq!(
                fire_ignite_odds(id),
                60,
                "the flora row {} the 60/100 pair keeps",
                NAME_OF[id as usize]
            );
        }
        // THE BURN COLUMN (the pairs' second half: the log 5, the
        // leaves family 60, the litter 100, the tnt 100, the flora
        // 100 — the trunk the slow coal the 5 speaks)
        assert_eq!(fire_burn_odds(LOG), 5);
        assert_eq!(fire_burn_odds(LEAVES), 60);
        assert_eq!(fire_burn_odds(PALE_LEAVES), 60);
        assert_eq!(fire_burn_odds(TNT), 100, "the tnt the prime chain's own 100 wears");
        assert_eq!(fire_burn_odds(WHITE_WOOL), 60, "the wool the 60 burn keeps");
        assert_eq!(fire_burn_odds(LEAF_LITTER), 100);
        for id in [
            SHORT_GRASS, BUSH, TALL_GRASS, TALL_GRASS_UPPER, LARGE_FERN, LARGE_FERN_UPPER,
            DANDELION, POPPY, AZURE_BLUET, OXEYE_DAISY, CORNFLOWER, ORANGE_TULIP, RED_TULIP,
            PINK_TULIP, WHITE_TULIP,
        ] {
            assert_eq!(
                fire_burn_odds(id),
                100,
                "the flora row {} the 100 burn keeps",
                NAME_OF[id as usize]
            );
        }
        // THE SILENCE THE TABLES KEEP (the no-row kinds the zero odds
        // wear — the crop, the table, the stones, the fluids, the fire
        // kinds themselves the flammable scan never names)
        for id in [
            STONE, DIRT, GRASS, SAND, WATER, LAVA, WHEAT, WHEAT_STAGE7, CRAFTING_TABLE,
            FARMLAND, FIRE, SOUL_FIRE,
        ] {
            assert_eq!(
                fire_ignite_odds(id),
                0,
                "the no-row silence {} the ignite keeps",
                NAME_OF[id as usize]
            );
            assert_eq!(
                fire_burn_odds(id),
                0,
                "the no-row silence {} the burn keeps",
                NAME_OF[id as usize]
            );
        }
        // THE PARITY LAW (javap `canBurn(BlockState)` — the
        // `getIgniteOdds(state) > 0` verdict the pc 0-13 pair serves):
        // every palette kind the verdict and the table agree
        for id in 0..(NAME_OF.len() as u16) {
            assert_eq!(
                fire_can_burn(id),
                fire_ignite_odds(id) > 0,
                "the parity {} the canBurn verdict keeps",
                NAME_OF[id as usize]
            );
        }
    }

    /// THE FIRE TICK'S OWN SMALL LAWS — javap `FireBlock.tick` the
    /// walk's own opening arms: the re-arm the queue's ONE-entry law
    /// keeps (the Set dedup the schedule's duplicate refuses), the
    /// 30..=39 window the delay draw banks, the ladder the pre-ladder
    /// quirk honors, the survival refusal the floating cell's own
    /// walk serves, the stream parity the draws' own count keeps (the
    /// re-arm + the ladder + the six burn-out rolls the lone stone
    /// stage speaks — the spread the zero-odds candidates never draws).
    #[test]
    fn the_fire_tick_the_ladder_the_rearm_and_the_survival_keep() {
        // THE SET'S OWN DEDUP (the LevelChunkTicks.schedule law — the
        // duplicate the add refuses): two schedules one entry
        let mut w = World::new(1, 0xF117);
        w.schedule_fire_tick(50, 4, 31, 4);
        w.schedule_fire_tick(60, 4, 31, 4);
        assert_eq!(w.fire_ticks.len(), 1, "the Set's own law the duplicate refuses");
        assert_eq!(w.fire_ticks[0], (50, 4, 31, 4), "the first the slot holds");
        // THE FLOATING FIRE'S OWN REFUSAL (pc 22-37 — the canSurvive
        // verdict the air-below cell fails, the removal the walk
        // serves): the parity draws the survival path keeps (the
        // re-arm's nextInt(10) + the ladder's nextInt(3) — the rain
        // arm the dry world never draws). The dedup probe's own entry
        // the fresh stage clears.
        w.fire_ticks.clear();
        let mut rng = crate::drops::Lcg::new(0xF117_A1);
        let mut probe = rng.clone();
        let _ = probe.next_int(10); // the re-arm's own draw
        let _ = probe.next_int(3); // the ladder's own draw (the parity)
        w.set(4, 40, 4, FIRE);
        let out = w.fire_tick(100, 4, 40, 4, &mut rng);
        assert!(out.changed, "the floating removal the walk speaks");
        assert_eq!(w.get(4, 40, 4), AIR, "the canSurvive refusal the cell clears");
        assert!(!w.fire_age.contains_key(&(4, 40, 4)), "the removal the entry drops");
        assert_eq!(
            w.fire_ticks.len(),
            1,
            "the re-arm the stale entry the dead cell leaves"
        );
        // the stale entry the next drain's own gate drops (the
        // block-match verdict the non-fire cell answers)
        let due = w.fire_ticks[0].0;
        let cells = w.drain_due_fire_ticks(due);
        assert_eq!(cells, vec![(4, 40, 4)]);
        let out = w.fire_tick(due, 4, 40, 4, &mut rng);
        assert!(!out.changed, "the dead cell's own walk the silence keeps");
        assert!(w.fire_ticks.is_empty(), "the gate the stale entry drops");
        // THE LONE STONE FIRE'S OWN LADDER (the grounded cell the
        // sturdy floor holds, no burnable neighbor the isValid scan
        // refuses): the age the pre-ladder quirk honors — the walk's
        // own decisions the pc 62-75 local wears, the death the
        // age>3 walk serves, the parity draws the eight the lone
        // stage speaks (re-arm + ladder + six burn-out rolls — the
        // spread the zero-odds candidates never draws)
        let mut w = World::new(1, 0xF117);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 30, z, STONE);
            }
        }
        for y in 31..=38 {
            for x in 0..16 {
                for z in 0..16 {
                    w.set(x, y, z, AIR); // the open box the stage clears
                }
            }
        }
        w.set(4, 31, 4, FIRE); // the grounded flame the stone holds
        w.fire_age.insert((4, 31, 4), 0);
        let mut rng = crate::drops::Lcg::new(0xF117_A2);
        let mut probe = rng.clone();
        let window = probe.next_int(10); // the re-arm's own draw
        let roll = probe.next_int(3); // the ladder's own roll
        let _ = probe.next_int(300); // east — the six burn-out rolls
        let _ = probe.next_int(300); // west
        let _ = probe.next_int(250); // below
        let _ = probe.next_int(250); // above
        let _ = probe.next_int(300); // north
        let _ = probe.next_int(300); // south
        let out = w.fire_tick(1000, 4, 31, 4, &mut rng);
        assert!(!out.changed, "the young flame the sturdy floor holds");
        assert_eq!(w.get(4, 31, 4), FIRE, "the surviving cell the walk keeps");
        // THE RE-ARM'S OWN WINDOW (the 30..=39 the delay draw banks)
        assert_eq!(
            w.fire_ticks.len(),
            1,
            "the ONE-entry law the walk's own re-arm keeps"
        );
        assert_eq!(
            w.fire_ticks[0].0,
            1000 + FIRE_TICK_DELAY_BASE + window as u64,
            "the due the re-arm's own draw banks"
        );
        // THE LADDER'S OWN PRODUCT (min(15, 0 + roll/2) — the nonzero
        // step alone re-stores, the placement's own age-0 entry the
        // zero step keeps)
        match (roll, w.fire_age.get(&(4, 31, 4))) {
            (0, Some(&0)) | (1, Some(&0)) => {}
            (2, Some(&1)) => {}
            other => panic!("the ladder's own law the pair {:?} refuses", other),
        }
        // THE STREAM PARITY (the eight draws the lone stage speaks)
        assert_eq!(rng.next_int(1000), probe.next_int(1000), "the stream rejoins");
        // THE PRE-LADDER QUIRK'S OWN DEATH: the walks the age climbs
        // (monotone, +0 or +1 per walk), the death the age>3 walk's
        // own removal serves (the sturdy arm the grown flame refuses)
        let mut prev_age = 0i32;
        for step in 1..=30 {
            let due = w.fire_ticks[0].0;
            let cells = w.drain_due_fire_ticks(due);
            assert_eq!(cells, vec![(4, 31, 4)]);
            let out = w.fire_tick(due, 4, 31, 4, &mut rng);
            let alive = w.get(4, 31, 4) == FIRE;
            if alive {
                assert!(!out.changed, "the survivor the silence keeps");
                let age = *w.fire_age.get(&(4, 31, 4)).unwrap_or(&0) as i32;
                assert!(
                    age >= prev_age && age - prev_age <= 1,
                    "the ladder's own law the step {} wears ({} -> {})",
                    step,
                    prev_age,
                    age
                );
                prev_age = age;
            } else {
                assert!(out.changed, "the death the walk speaks");
                assert!(
                    prev_age > 3,
                    "the death the age>3 walk serves (the age {} the ladder wore)",
                    prev_age
                );
                return;
            }
        }
        panic!("the lone stone fire the walks never doused");
    }

    #[test]
    fn the_signal_fire_the_hay_below_the_verdict_wears() {
        // THE SMOKE SOURCE'S OWN LAW (javap CampfireBlock.isSmokeSource
        // pc 228-231 over the 26.2 jar — the `state.is(Blocks.HAY_BLOCK)`
        // read the SIGNAL_FIRE property carries): the campfire's below
        // cell the hay bale ALONE answers true — the stone the dirt the
        // whole palette the verdict refuses, the below-world read the
        // honest false keeps.
        let mut w = World::new(1, 0xF117);
        let (x, y, z) = (4, 40, 4);
        w.set(x, y, z, CAMPFIRE_LIT);
        // the stone below: the property the vanilla placement walk
        // banks false — the verdict refuses
        assert!(!w.is_signal_fire(x, y, z), "the stone below the verdict refuses");
        // the hay bale: the isSmokeSource read the SIGNAL arm answers
        w.set(x, y - 1, z, HAY_BLOCK);
        assert!(w.is_signal_fire(x, y, z), "the hay below the verdict answers");
        // the hay removed: the updateShape's DOWN-neighbor re-answer the
        // live read tracks (the offline read-each-time law the property
        // cache never drifts from)
        w.set(x, y - 1, z, AIR);
        assert!(!w.is_signal_fire(x, y, z), "the hay removal the verdict re-answers");
        // THE SOUL CAMPFIRE'S OWN VERDICT: the same below-read the
        // signal law keeps (the vanilla property the two campfire
        // families share)
        w.set(x, y - 1, z, HAY_BLOCK);
        w.set(x, y, z, SOUL_CAMPFIRE_LIT);
        assert!(w.is_signal_fire(x, y, z), "the soul campfire the same verdict wears");
    }

    #[test]
    fn the_campfire_queue_the_set_maintenance_the_rearm_wear() {
        // THE SET'S OWN TRANSITIONS (the fire queue's own shape the BE
        // tick's every-tick cadence wears): the lit transition the
        // entry pushes (due 0 — the next drain picks it up), the
        // same-state silence the duplicate never pushes, the un-lit
        // transition the entry drops, the re-lit the entry re-banks.
        let mut w = World::new(1, 0xF117);
        let (x, y, z) = (4, 40, 4);
        w.set(x, y, z, CAMPFIRE_LIT);
        assert_eq!(w.campfire_ticks, vec![(0, x, y, z)], "the lit transition the entry pushes");
        // the same-state silence (the set's own early return the
        // maintenance never re-spends)
        w.set(x, y, z, CAMPFIRE_LIT);
        assert_eq!(w.campfire_ticks.len(), 1, "the same state the queue never doubles");
        // the soul campfire's own swap: lit -> lit stays (the
        // is_lit -> is_lit no-transition the maintenance skips)
        w.set(x, y, z, SOUL_CAMPFIRE_LIT);
        assert_eq!(w.campfire_ticks.len(), 1, "the lit-to-lit swap the entry keeps");
        // the douse: the un-lit transition the entry drops (the set's
        // own maintenance the walk never drains a dead cell)
        w.set(x, y, z, CAMPFIRE);
        assert!(w.campfire_ticks.is_empty(), "the douse the entry drops");
        // the re-lit: the entry re-banks (the ignite lane's own
        // product the queue serves again)
        w.set(x, y, z, CAMPFIRE_LIT);
        assert_eq!(w.campfire_ticks, vec![(0, x, y, z)], "the re-lit the entry re-banks");
        // THE DRAIN'S OWN SHAPE (the fire queue's own drain the time
        // order the stable sort banks): the due entry the drain gives
        // up, the re-arm the due+1 entry banks, the duplicate the add
        // refuses
        let due = w.drain_due_campfire_ticks(100);
        assert_eq!(due, vec![(x, y, z)], "the due-0 entry the drain gives up");
        assert!(w.campfire_ticks.is_empty());
        w.rearm_campfire_tick(101, x, y, z);
        w.rearm_campfire_tick(102, x, y, z);
        assert_eq!(w.campfire_ticks, vec![(101, x, y, z)], "the one-live-entry law the duplicate refuses");
        // the not-yet-due entry the drain keeps
        assert!(w.drain_due_campfire_ticks(100).is_empty(), "the future entry the drain keeps");
        assert_eq!(w.drain_due_campfire_ticks(101), vec![(x, y, z)]);
    }


    /// THE FIRE'S OWN LIFE — javap `FireBlock.tick` the spread, the
    /// burn-out and the prime chain the seeded drive answers: the TNT
    /// neighbor the 100-odds roll eats (the prime the walk's own fruit
    /// carries, the cell the inner walk's own product wears), the grass
    /// tuft the 100-odds roll consumes, the spread the odds gate's own
    /// rolls place (the burnable neighbor's own air the threshold
    /// answers), the floor the no-row silence keeps (the grass_block
    /// the fire never eats), the end the walk's own verdict speaks —
    /// the fireless stage the queue's own drain clears.
    #[test]
    fn the_fire_the_spread_the_burnout_and_the_prime_land() {
        let mut w = World::new(1, 0xF17E);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 30, z, GRASS); // the floor the no-row silence keeps
            }
        }
        for y in 31..=38 {
            for x in 0..16 {
                for z in 0..16 {
                    w.set(x, y, z, AIR); // the open box the stage clears
                }
            }
        }
        let (fx, fy, fz) = (8i32, 31i32, 8i32);
        let (tx, ty, tz) = (9i32, 31i32, 8i32); // the east neighbor the TNT wears
        let (gx, gy, gz) = (7i32, 31i32, 8i32); // the west neighbor the tuft wears
        w.set(tx, ty, tz, TNT);
        w.set(gx, gy, gz, SHORT_GRASS);
        w.set(fx, fy, fz, FIRE);
        w.fire_age.insert((fx, fy, fz), 0);
        // THE ONPLACE LAW'S OWN FIRST SCHEDULE (the placement arm's own
        // shape — the age-0 entry + the 30..=39 due)
        let mut rng = crate::drops::Lcg::new(0xF17E_117);
        w.schedule_fire_tick(1000 + fire_tick_delay(&mut rng), fx, fy, fz);
        let (mut saw_new_fire, mut saw_prime, mut steps) = (false, false, 0);
        while steps < 400 {
            let Some(min_due) = w.fire_ticks.iter().map(|&(at, _, _, _)| at).min() else {
                break; // the queue's own silence — the walk's end speaks
            };
            let cells = w.drain_due_fire_ticks(min_due);
            steps += 1;
            for (x, y, z) in cells {
                let out = w.fire_tick(min_due, x, y, z, &mut rng);
                saw_new_fire |= !out.new_fire.is_empty();
                saw_prime |= out.tnt_primed.contains(&(tx, ty, tz));
            }
            // THE END'S OWN VERDICT: the TNT the prime ate, the tuft
            // the roll consumed, no fire left, the queue clean
            if saw_prime
                && w.get(tx, ty, tz) != TNT
                && w.get(gx, gy, gz) != SHORT_GRASS
                && !w.fire_ticks.is_empty()
            {
                // the stale entries the dead cells leave — the drains
                // the gate clears, the loop's own walk answers
                continue;
            }
            if saw_prime && w.get(tx, ty, tz) != TNT && w.get(gx, gy, gz) != SHORT_GRASS {
                // drive the stale entries out — the queue's own end
                let mut guard = 0;
                while !w.fire_ticks.is_empty() && guard < 64 {
                    let due = w.fire_ticks.iter().map(|&(at, _, _, _)| at).min().unwrap();
                    let cells = w.drain_due_fire_ticks(due);
                    for (x, y, z) in cells {
                        let _ = w.fire_tick(due, x, y, z, &mut rng);
                    }
                    guard += 1;
                }
                break;
            }
        }
        assert!(saw_prime, "the TNT the prime chain ate (steps {})", steps);
        assert!(
            w.get(tx, ty, tz) != TNT,
            "the burnt TNT cell the prime's own product wears"
        );
        assert!(
            w.get(gx, gy, gz) != SHORT_GRASS,
            "the tuft the burn-out roll consumed"
        );
        assert!(saw_new_fire, "the spread's own rolls placed fire");
        assert!(
            !is_fire(w.get(fx, fy, fz)) && !is_fire(w.get(tx, ty, tz)) && !is_fire(w.get(gx, gy, gz)),
            "the end the walk's own verdict speaks — the fireless stage"
        );
        assert!(
            w.fire_ticks.is_empty(),
            "the queue the stale drains clear"
        );
        // THE FLOOR'S OWN SILENCE (the no-row grass_block the fire
        // never eats — the burnable scan the floor's own kind refuses)
        for x in 0..16 {
            for z in 0..16 {
                assert_eq!(
                    w.get(x, 30, z),
                    GRASS,
                    "the floor {} the no-row silence keeps",
                    NAME_OF[w.get(x, 30, z) as usize]
                );
            }
        }
    }

    /// THE RAIN'S OWN DOUSE — javap `FireBlock.tick` pc 77-127 the arm
    /// the rain gate serves (`isRaining && isNearRain && nextFloat() <
    /// 0.2F + age*0.03F` — the fold the offline world's own rain_level
    /// and the open-sky plains wear): the rain the dry world's fire
    /// never draws, the wet world's fire the 0.2+ ladder douses.
    #[test]
    fn the_rain_the_fire_douses() {
        let mut w = World::new(1, 0xF117);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 30, z, STONE);
            }
        }
        for y in 31..=38 {
            for x in 0..16 {
                for z in 0..16 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        w.set(4, 31, 4, FIRE);
        w.fire_age.insert((4, 31, 4), 0);
        let mut rng = crate::drops::Lcg::new(0xF117_A3);
        // THE DRY WORLD'S OWN SILENCE: the fire the arm never touches
        // (the rain_level 0.0 the procedural world keeps)
        w.schedule_fire_tick(1000 + fire_tick_delay(&mut rng), 4, 31, 4);
        for _ in 0..3 {
            let due = w.fire_ticks.iter().map(|&(at, _, _, _)| at).min().unwrap();
            let cells = w.drain_due_fire_ticks(due);
            for (x, y, z) in cells {
                let _ = w.fire_tick(due, x, y, z, &mut rng);
            }
        }
        assert_eq!(w.get(4, 31, 4), FIRE, "the dry world's fire the arm never douses");
        // THE WET WORLD'S OWN DOUSE: the 0.2+ draw the arm's own roll
        // serves — the ladder's own young ages the 0.2 floor wears
        w.rain_level = 1.0;
        let mut doused = false;
        for _ in 0..60 {
            let Some(due) = w.fire_ticks.iter().map(|&(at, _, _, _)| at).min() else {
                break;
            };
            let cells = w.drain_due_fire_ticks(due);
            for (x, y, z) in cells {
                let _ = w.fire_tick(due, x, y, z, &mut rng);
            }
            if w.get(4, 31, 4) != FIRE {
                doused = true;
                break;
            }
        }
        assert!(doused, "the rain the arm's own roll douses");
    }

    /// THE FIRE'S OWN SHAPE UPDATE — javap `FireBlock.updateShape` the
    /// pc 0-8 canSurvive verdict the pc 32-38 AIR branch serves, the
    /// chain the flag-3 broadcast keeps: the FIRE arm's own flip (the
    /// log the cell above stood on the flame replaces — the below
    /// burnable the canSurvive verdict loses, the loss the chain the
    /// SAME beat douses), the NEIGHBOR-SCAN arm (the six-way canBurn
    /// read the horizontal log the verdict holds, the log's loss the
    /// chain speaks), the STURDY arm the grounded flame keeps (the
    /// burnable's loss the below-Full verdict never gates), the
    /// CHANGED CELL the walk never reads (the neighbors alone the
    /// broadcast serves — the placed flame the chain's own silence
    /// keeps).
    #[test]
    fn the_support_the_chain_the_fire_arm_wears() {
        // (a) THE FIRE ARM'S OWN FLIP: the log the cell above stood on
        // the flame replaces (pc 85 `iconst_3` — the setBlock the
        // broadcast keeps), the chain the fire above the SAME beat
        // douses
        let mut w = World::new(1, 0xF117);
        w.set(4, 30, 4, STONE);
        w.set(4, 31, 4, LOG);
        w.set(4, 32, 4, FIRE);
        w.fire_age.insert((4, 32, 4), 0);
        w.set(4, 31, 4, FIRE); // the burn-out's own product the arm places
        let changed = w.fire_shape_updates((4, 31, 4));
        assert!(changed, "the fire the log held the chain douses");
        assert_eq!(
            w.get(4, 32, 4),
            AIR,
            "the below-burnable the flame replaces, the verdict the loss refuses"
        );
        assert!(
            !w.fire_age.contains_key(&(4, 32, 4)),
            "the dead cell's age entry the chain drops"
        );
        assert_eq!(
            w.get(4, 31, 4),
            FIRE,
            "the changed cell the walk never reads — the placed flame stays"
        );
        // (b) THE NEIGHBOR-SCAN ARM: the floating flame the horizontal
        // log the canBurn scan holds — the surviving branch the walk
        // keeps (false the fold returns), the log's loss the chain
        // speaks
        let mut w = World::new(1, 0xF117);
        w.set(8, 32, 8, FIRE); // the below cell the air the sturdy arm refuses
        w.fire_age.insert((8, 32, 8), 3);
        w.set(8, 32, 9, LOG); // the SOUTH burnable the scan holds
        let changed = w.fire_shape_updates((8, 32, 9));
        assert!(!changed, "the surviving branch the walk keeps");
        assert_eq!(w.get(8, 32, 8), FIRE, "the scan's own hold the flame keeps");
        w.set(8, 32, 9, AIR); // the log's own loss
        let changed = w.fire_shape_updates((8, 32, 9));
        assert!(changed, "the burnable's loss the chain speaks");
        assert_eq!(w.get(8, 32, 8), AIR, "the floating flame the scan's loss douses");
        assert!(!w.fire_age.contains_key(&(8, 32, 8)), "the age entry the death drops");
        // (c) THE STURDY ARM: the grounded flame the below-Full verdict
        // holds — the burnable neighbor's own loss the scan never gates
        let mut w = World::new(1, 0xF117);
        w.set(4, 30, 4, STONE);
        w.set(4, 31, 4, FIRE);
        w.fire_age.insert((4, 31, 4), 0);
        w.set(4, 31, 5, SHORT_GRASS);
        w.set(4, 31, 5, AIR); // the tuft's own loss
        let changed = w.fire_shape_updates((4, 31, 5));
        assert!(
            !changed,
            "the sturdy floor the verdict keeps — the burnable's loss the scan never gates"
        );
        assert_eq!(w.get(4, 31, 4), FIRE, "the grounded flame stands");
    }

    /// THE BURN-OUT'S OWN CHAIN THROUGH THE WALK — the seeded drive the
    /// below call's own 250-chance roll forces (the LOG's own burn odds
    /// 5 the hit wears): the inner arm the FIRE cell or the AIR removal
    /// serves, EVERY arm the flag-3 broadcast fires (pc 85 the setBlock,
    /// pc 95-96 the removeBlock), the fire stack the burnt cell held the
    /// chain the SAME walk douses — the scheduled tick's own 30..=39 lag
    /// the vanilla never waits.
    #[test]
    fn the_burnout_the_chain_the_fire_stack_wears() {
        let stage = || {
            let mut w = World::new(1, 0xF117);
            for x in 0..16 {
                for z in 0..16 {
                    w.set(x, 30, z, STONE);
                }
            }
            for y in 31..=38 {
                for x in 0..16 {
                    for z in 0..16 {
                        w.set(x, y, z, AIR);
                    }
                }
            }
            w.set(4, 31, 4, LOG); // the only burnable the stage banks
            w.set(4, 32, 4, FIRE); // the flame the log's own loss alone holds
            w.fire_age.insert((4, 32, 4), 0);
            w
        };
        // the brute drive: some seed the below call hits (2%/tick), the
        // inner arm's own split the two products each land — every hit
        // the chain's own douse wears
        let (mut fire_arm, mut air_arm) = (None, None);
        for seed in 0..4000u64 {
            let mut w = stage();
            let mut rng = crate::drops::Lcg::new(seed);
            let out = w.fire_tick(1000, 4, 32, 4, &mut rng);
            let below = w.get(4, 31, 4);
            if below == LOG {
                continue; // the roll refused — the cell the fire never ate
            }
            // THE CHAIN'S OWN VERDICT (the walk's own mid-beat douse):
            // the fire above the burnt cell DIES the same walk, the age
            // entry the death drops, the changed flag the walk's own
            // burn-out wears
            assert_eq!(
                w.get(4, 32, 4),
                AIR,
                "the stack the chain douses (seed {seed}, the arm {:?})",
                below
            );
            assert!(
                !w.fire_age.contains_key(&(4, 32, 4)),
                "the dead flame's age entry the chain drops (seed {seed})"
            );
            assert!(out.changed, "the burn-out the changed flag wears");
            if below == FIRE && fire_arm.is_none() {
                fire_arm = Some(seed); // pc 85 — the setBlock arm's own chain
            }
            if below == AIR && air_arm.is_none() {
                air_arm = Some(seed); // pc 95-96 — the removeBlock arm's own chain
            }
            if fire_arm.is_some() && air_arm.is_some() {
                break;
            }
        }
        assert!(
            fire_arm.is_some(),
            "the FIRE arm's own hit the drive never forced"
        );
        assert!(
            air_arm.is_some(),
            "the AIR arm's own hit the drive never forced"
        );
    }

    /// THE CONCRETE'S OWN TOUCHES-LIQUID WALK (javap
    /// ConcretePowderBlock.touchesLiquid over the 26.2 jar — the six
    /// Direction.values() arms the raw disasm keeps): the SIDE water the
    /// flag arms, the ABOVE water the same verdict speaks, THE DOWN
    /// ARM'S OWN GATE the own-cell read arms — the powder's own cell the
    /// below scan refuses (the vanilla's own famous asymmetry: the
    /// powder NEVER solidifies from the water below it), the WATER
    /// own-cell the gate admits (the placement-into-water moment the
    /// touchesLiquid arm serves), the empty scan the verdict keeps.
    #[test]
    fn the_powder_the_touches_liquid_walk_wears() {
        let mut world = World::new(1, 42);
        let (x, y, z) = (10, 12, 10);
        // THE EMPTY SCAN: the dry air the verdict keeps
        assert!(
            !concrete_touches_liquid(&world, x, y, z),
            "the dry cell the walk refuses"
        );
        // THE SIDE WATER (the WEST arm — the Direction.values() order
        // the shared arms wear): the never-sturdy water the flag arms
        world.set(x - 1, y, z, WATER);
        assert!(
            concrete_touches_liquid(&world, x, y, z),
            "the side water the arm speaks"
        );
        world.set(x - 1, y, z, AIR);
        // THE ABOVE WATER (the UP arm — the sturdy read the DOWN-face
        // argument the vanilla's own pair keeps)
        world.set(x, y + 1, z, WATER);
        assert!(
            concrete_touches_liquid(&world, x, y, z),
            "the above water the arm speaks"
        );
        world.set(x, y + 1, z, AIR);
        // THE DOWN WATER ALONE: the powder's own cell the gate refuses —
        // the below scan the walk never spends (the asymmetry the
        // falling lane's own landing law keeps — the powder rests on
        // the water the solidify the gravity answers instead)
        world.set(x, y - 1, z, WATER);
        assert!(
            !concrete_touches_liquid(&world, x, y, z),
            "the below water the DOWN gate refuses"
        );
        // THE WATER OWN-CELL: the gate ADMITS the below scan (the
        // placement-into-water moment — the clicked cell the state the
        // shouldSolidify's touchesLiquid arm reads)
        world.set(x, y, z, WATER);
        assert!(
            concrete_touches_liquid(&world, x, y, z),
            "the water own cell the gate admits"
        );
        world.set(x, y, z, AIR);
        // THE STURDY PAIR'S OWN REFUSAL: the canSolidify(neighbor) arm
        // the water's never-sturdy shape alone speaks — the stone
        // neighbor the canSolidify read refuses (the water tag the
        // solid kinds never carry)
        world.set(x + 1, y, z, STONE);
        assert!(
            !concrete_touches_liquid(&world, x, y, z),
            "the solid neighbor the canSolidify read refuses"
        );
    }

    /// One biome layer over a single chunk: every voxel resolved id 0,
    /// the tint table given.
    fn layer(tints: Vec<[Option<u32>; 4]>) -> BiomeLayer {
        BiomeLayer {
            ids: vec![vec![0; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            tints,
            climates: Vec::new(), // the tint law's own tests: no climate opinion
        }
    }

    #[test]
    fn procedural_world_has_no_biome_opinion() {
        let w = World::new(1, 42);
        assert!(w.biomes.is_none());
        assert_eq!(
            w.face_tint(GRASS, 2, 8, 12, 8),
            None,
            "no layer -> the baked look, bit for bit"
        );
    }

    #[test]
    fn the_dwell_clock_banks_the_128_center_radius() {
        // THE 26.2 SERVER WALK'S OWN LAW (anyPlayerCloseEnoughForSpawning
        // → euclideanDistanceSquared < 16384.0 — the (cx*16+8) center
        // the sectionToBlockCoord(pos, 8) walk keeps, the dcmpg the
        // strict `<` wears): a 16x16 grid spreads the chunks past the
        // radius, the body ON chunk (8,0)'s own center (136, 8) the
        // banks the proximity speaks
        let mut w = World::new(16, 42);
        w.tick_inhabited(136.0, 8.0);
        let banked = w.inhabited.as_ref().expect("the host world's own lane");
        // the center chunk: distance 0, the +1 the tick banks
        assert_eq!(banked[8], 1, "the body's own chunk the +1 banks");
        // chunk (0,0)'s center (8, 8) sits EXACTLY 128 away — 16384.0
        // the dcmpg arm refuses (the strict < the bytecode keeps, the
        // boundary the exact f64 the no-dust walk answers)
        assert_eq!(banked[0], 0, "the exact-128 chunk the strict law keeps");
        // chunk (7,0)'s center (120, 8): 16 away — inside the radius
        assert_eq!(banked[7], 1);
        // chunk (8,1)'s center (136, 24): 16 away — inside
        assert_eq!(banked[8 + 16], 1);
        // chunk (0,1)'s center (8, 24): sqrt(128^2 + 16^2) — beyond
        assert_eq!(banked[16], 0);

        // the far corner (15,15)'s center (248, 248): beyond
        assert_eq!(banked[15 + 15 * 16], 0);
        // the SECOND tick banks again (the cumulative walk the
        // incrementInhabitedTime banks)
        w.tick_inhabited(136.0, 8.0);
        let banked = w.inhabited.as_ref().unwrap();
        assert_eq!(banked[8], 2);
        assert_eq!(banked[0], 0);

        // THE NONE LANE (the networked world's no-opinion — the walk
        // the honest no-op keeps)
        let mut net = World::new(2, 42);
        net.inhabited = None;
        net.tick_inhabited(8.0, 8.0);
        assert!(net.inhabited.is_none());
    }

    /// THE PLAINS' OWN FLOWER PATCH (the flower_plain placed feature's
    /// honest stand-in — the decorate pass the generate walk serves):
    /// the deterministic seed replay the same world wears, the blooms
    /// the grass cells ALONE carry (the beach/water columns the patch
    /// never plants), the placement the AIR gate keeps (the trunk's own
    /// cell the tree lanes never lose), the KIND the
    /// noise_threshold_provider's own word answers — the default
    /// dandelion, the 0.33333334 high-chance third the Util.getRandom
    /// pick keeps (the provider's own four high rows the autopush-414
    /// round banks; the tulip band the noise walk never plants).
    #[test]
    fn the_flower_patch_wears_the_decorate_law() {
        let w = World::new(2, 42);
        let replay = World::new(2, 42);
        assert_eq!(w, replay, "the seed's replay the same world wears");
        let mut blooms = 0;
        let mut dandelions = 0;
        for x in 0..w.size_blocks() {
            for z in 0..w.size_blocks() {
                for y in 0..w.size_blocks() {
                    let id = w.get(x, y, z);
                    if !is_flower(id) {
                        continue;
                    }
                    assert!(
                        id == DANDELION || is_flower_high(id),
                        "the default + the four high rows the roll picks — got {id}"
                    );
                    // THE SUPPORT'S OWN GATE (the below cell the grass
                    // block keeps — the beach/water columns the patch
                    // never plants)
                    assert_eq!(
                        w.get(x, y - 1, z),
                        GRASS,
                        "the bloom the grass cell alone wears"
                    );
                    assert!(y > WATER_LEVEL);
                    if id == DANDELION {
                        dandelions += 1;
                    }
                    blooms += 1;
                }
            }
        }
        // the density band (the ~2-chunk world's above-water grass
        // columns × the 1/83 roll — the handful the patch answers)
        assert!(
            blooms >= 1 && blooms <= 12,
            "the patch density the band keeps: {} blooms",
            blooms
        );
        assert!(dandelions >= 1, "the default state the noise provider wears");
    }

    /// THE FLOWER PROVIDER'S OWN FULL PALETTE (the autopush-414 round —
    /// javap the jar's worldgen json the noise_threshold_provider's own
    /// lists bank): the family fns the provider's rows answer (the high
    /// band the [poppy, azure_bluet, oxeye_daisy, cornflower] list
    /// wears, the low band the four tulips, the ids the non-contiguous
    /// rows the tuft/double-plant kinds ride between), the bonemeal
    /// fold's Util.getRandom pick the four high rows plants across the
    /// seed search (the discriminator the uniform index keeps), the
    /// default dandelion the else arm keeps, the tulip band the walk
    /// never plants (the `d0 < threshold` arm the NormalNoise walk
    /// serves — the hash lane the low rows never fires), the decorate
    /// pass the same four rows the stand-in index wears across seeds.
    #[test]
    fn the_flower_provider_wears_the_full_palette_laws() {
        // THE FAMILY FN'S OWN ROWS: the nine flower kinds is_flower
        // answers, the sibling families and the cubes it refuses
        assert!(is_flower(DANDELION));
        assert!(is_flower(POPPY));
        for &id in &[AZURE_BLUET, OXEYE_DAISY, CORNFLOWER] {
            assert!(is_flower(id), "the high row {id} the family wears");
        }
        for &id in &[ORANGE_TULIP, RED_TULIP, PINK_TULIP, WHITE_TULIP] {
            assert!(is_flower(id), "the low row {id} the family wears");
        }
        assert!(!is_flower(GRASS));
        assert!(!is_flower(STONE));
        assert!(!is_flower(SHORT_GRASS));
        assert!(!is_flower(TALL_GRASS));
        assert!(!is_flower(TALL_GRASS_UPPER));
        assert!(!is_flower(LARGE_FERN_UPPER));
        // THE PROVIDER'S OWN LISTS: the high band the four rows the
        // json order keeps, the low band the four tulips — the
        // dandelion neither list's row (the default the else arm wears)
        assert!(is_flower_high(POPPY));
        assert!(is_flower_high(AZURE_BLUET));
        assert!(is_flower_high(OXEYE_DAISY));
        assert!(is_flower_high(CORNFLOWER));
        assert!(!is_flower_high(DANDELION));
        assert!(!is_flower_high(SHORT_GRASS), "the non-contiguous ids the ride between");
        assert!(!is_flower_high(LARGE_FERN_UPPER));
        assert!(is_flower_low(ORANGE_TULIP));
        assert!(is_flower_low(RED_TULIP));
        assert!(is_flower_low(PINK_TULIP));
        assert!(is_flower_low(WHITE_TULIP));
        assert!(!is_flower_low(POPPY));
        assert!(!is_flower_low(CORNFLOWER));
        // THE OFFSET'S OWN ROW: every new kind the XZ row the flower
        // family carries (the offsetType(XZ) chains the clinit speaks)
        for id in (AZURE_BLUET..=WHITE_TULIP).chain(std::iter::once(DANDELION)) {
            assert_eq!(
                plant_offset_type(id),
                PlantOffset::XZ,
                "the flower kind {id} the XZ row wears"
            );
        }
        // THE PLACEMENT BAND ACROSS SEEDS: the bonemeal fold's high
        // pick the four rows plants (the union the uniform index
        // answers), every planted flower the allowed set keeps, the
        // tulips the walk NEVER speaks (the low band the noise port's
        // own round plants)
        let flat = |seed: u64| {
            let mut w = World::new(1, seed);
            let size = w.size_blocks();
            let surface = CHUNK_Y as i32 / 2;
            for x in 0..size {
                for z in 0..size {
                    for y in 0..surface {
                        w.set(x, y, z, STONE);
                    }
                    w.set(x, surface, z, GRASS);
                    for y in (surface + 1)..CHUNK_Y as i32 {
                        w.set(x, y, z, AIR);
                    }
                }
            }
            (w, size / 2, surface)
        };
        let mut high_rows_seen = [false; 4]; // poppy, azure, oxeye, cornflower
        let mut dandelion_seen = false;
        for seed in 0..64u64 {
            let (mut w, c, surface) = flat(1000 + seed);
            let mut rng = crate::drops::Lcg::new(9000 + seed);
            bonemeal_grass_walk(&mut w, c, surface, c, &mut rng);
            for x in 0..w.size_blocks() {
                for y in 0..CHUNK_Y as i32 {
                    for z in 0..w.size_blocks() {
                        let id = w.get(x, y, z);
                        if !is_flower(id) {
                            continue;
                        }
                        if id == DANDELION {
                            dandelion_seen = true;
                        } else if is_flower_high(id) {
                            let row = match id {
                                POPPY => 0,
                                AZURE_BLUET => 1,
                                OXEYE_DAISY => 2,
                                _ => 3,
                            };
                            high_rows_seen[row] = true;
                        } else {
                            panic!("the tulip band the walk never plants — got {id}");
                        }
                    }
                }
            }
        }
        assert!(
            dandelion_seen,
            "the default the else arm plants across seeds"
        );
        assert!(
            high_rows_seen.iter().all(|&seen| seen),
            "the four high rows the uniform pick wears across seeds: {high_rows_seen:?}"
        );
        // THE DECORATE PASS'S OWN STAND-IN: the same four rows the
        // second hash's index wears across the seed search (the world's
        // blooms the allowed set alone speaks, the union the four-way
        // split answers)
        let mut world_rows_seen = [false; 4];
        let mut world_dandelion_seen = false;
        for seed in 0..64u64 {
            let w = World::new(2, 7000 + seed);
            for x in 0..w.size_blocks() {
                for z in 0..w.size_blocks() {
                    for y in 0..w.size_blocks() {
                        let id = w.get(x, y, z);
                        if !is_flower(id) {
                            continue;
                        }
                        if id == DANDELION {
                            world_dandelion_seen = true;
                        } else if is_flower_high(id) {
                            let row = match id {
                                POPPY => 0,
                                AZURE_BLUET => 1,
                                OXEYE_DAISY => 2,
                                _ => 3,
                            };
                            world_rows_seen[row] = true;
                        } else {
                            panic!(
                                "the world's blooms the tulip band never speaks — got {id}"
                            );
                        }
                    }
                }
            }
        }
        assert!(world_dandelion_seen, "the default the worlds wear");
        assert!(
            world_rows_seen.iter().all(|&seen| seen),
            "the four high rows the stand-in index wears across worlds: {world_rows_seen:?}"
        );
    }

    /// THE GRASS BLOCK'S OWN BONE MEAL WALK — the 128-law the pure fn
    /// wears (javap the 26.2 GrassBlock.performBonemeal): the seed's
    /// replay the same world wears, the placements the walk's own kinds
    /// alone speak on the air cells the grass-below gate answers, the
    /// full-cube cap the walk never speaks (the abort the whole
    /// i-iteration keeps), the height band the top rows refuse, the
    /// spread the shuffled order the stone-below refusal the
    /// Fisher-Yates keeps (the walked tuft's own BonemealableBlock
    /// dispatch the 1/10 row wears).
    #[test]
    fn the_bonemeal_grass_walk_wears_the_128_laws() {
        // THE FLAT PLAINS the walk's own gates wear: the stone body
        // the grass skin, every column the same law answers (the
        // seed's own terrain the total fill overwrites — the walk's
        // scenarios the controlled surface carries)
        let flat = |seed: u64| {
            let mut w = World::new(1, seed);
            let size = w.size_blocks();
            let surface = CHUNK_Y as i32 / 2;
            for x in 0..size {
                for z in 0..size {
                    for y in 0..surface {
                        w.set(x, y, z, STONE);
                    }
                    w.set(x, surface, z, GRASS);
                    for y in (surface + 1)..CHUNK_Y as i32 {
                        w.set(x, y, z, AIR);
                    }
                }
            }
            (w, size / 2, surface)
        };

        // THE REPLAY: the seed's two runs the same world wears
        let (mut a, c, surface) = flat(7);
        let mut rng_a = crate::drops::Lcg::new(1234);
        bonemeal_grass_walk(&mut a, c, surface, c, &mut rng_a);
        let (mut b, _, _) = flat(7);
        let mut rng_b = crate::drops::Lcg::new(1234);
        bonemeal_grass_walk(&mut b, c, surface, c, &mut rng_b);
        assert_eq!(a, b, "the seed's replay the same world wears");

        // THE PLACEMENT BAND: every changed cell the walk's own kinds
        // alone speak (the 7/8 grass row the short tuft wears, the 1/8
        // flower fold the default dandelion + the provider's own four
        // high rows answer — the tulip band the noise walk never
        // plants), the placement the air cells alone wears, the below
        // the walk's own grass gate keeps (the spread's dirt/farmland
        // rows the flat world never speaks)
        let (base, _, _) = flat(7);
        let mut placed = 0;
        for x in 0..a.size_blocks() {
            for y in 0..CHUNK_Y as i32 {
                for z in 0..a.size_blocks() {
                    let (now, was) = (a.get(x, y, z), base.get(x, y, z));
                    if now != was {
                        assert!(
                            now == SHORT_GRASS
                                || now == DANDELION
                                || is_flower_high(now),
                            "the placement the walk's own kinds alone speak — got {now}"
                        );
                        assert_eq!(was, AIR, "the placement the air cells alone wears");
                        assert_eq!(
                            a.get(x, y - 1, z),
                            GRASS,
                            "the planted cell the grass-below gate keeps"
                        );
                        placed += 1;
                    }
                }
            }
        }
        assert!(placed >= 1, "the walk the origin cell alone plants");
        // THE ORIGIN ROW: the origin-above cell the first 16 rows
        // plant (the 7/8 grass row the 1/8 flower fold answers — the
        // 1/32 skip the honest unbanked picks wear)
        let origin = a.get(c, surface + 1, c);
        assert!(
            origin == SHORT_GRASS || origin == DANDELION || origin == POPPY || origin == AIR,
            "the origin cell the placement band answers — got {origin}"
        );

        // THE FULL-CUBE ABORT: the stone cap the walk never speaks
        // (the isCollisionShapeFullBlock refusal the air rows never
        // cross), the above-cap cells the below-gate the partial
        // walks refuse
        let (mut w, c, surface) = flat(3);
        w.set(c, surface + 1, c, STONE);
        let mut rng = crate::drops::Lcg::new(99);
        bonemeal_grass_walk(&mut w, c, surface, c, &mut rng);
        assert_eq!(
            w.get(c, surface + 1, c),
            STONE,
            "the stone cap the walk never speaks"
        );
        assert_eq!(
            w.get(c, surface + 2, c),
            AIR,
            "the above-cap cell the below-gate refuses"
        );

        // THE HEIGHT BAND: the top-row grass the above-band the rows
        // refuse (the isInsideBuildHeight the top arm keeps)
        let (mut w, c, _) = flat(3);
        let top = CHUNK_Y as i32 - 1;
        w.set(c, top, c, GRASS);
        let mut rng = crate::drops::Lcg::new(99);
        bonemeal_grass_walk(&mut w, c, top, c, &mut rng);
        let (mut expect, _, _) = flat(3);
        expect.set(c, top, c, GRASS);
        assert_eq!(w, expect, "the top-band origin the walk never speaks");

        // THE SPREAD'S OWN SHUFFLE LAW: the pre-placed tuft the 1/10
        // row reads, the north neighbour's below the stone refusal
        // carries (the canSurvive the SUPPORTS_VEGETATION subset
        // keeps — the walk's own below-gate the same cell refuses) —
        // the N cell stays air across every seed, the shuffled order
        // the placement wears (the Util.shuffle the three draws
        // spends), the S/W/E cells the shuffle's own rotations speak
        // across the 256 seeds, the 1/10 row the seeds fire
        let mut the_spread_fires = false;
        let (mut s_fires, mut w_fires, mut e_fires) = (false, false, false);
        for seed in 0..256u64 {
            let (mut w, c, surface) = flat(seed);
            w.set(c, surface + 1, c, SHORT_GRASS);
            w.set(c, surface, c - 1, STONE);
            let mut rng = crate::drops::Lcg::new(seed * 31 + 7);
            bonemeal_grass_walk(&mut w, c, surface, c, &mut rng);
            assert_eq!(
                w.get(c, surface + 1, c),
                SHORT_GRASS,
                "the pre-placed tuft the walk never removes"
            );
            assert_eq!(
                w.get(c, surface + 1, c - 1),
                AIR,
                "the N cell the stone-below refusal the order law keeps"
            );
            if w.get(c, surface + 1, c + 1) == SHORT_GRASS {
                the_spread_fires = true;
                s_fires = true;
            }
            if w.get(c - 1, surface + 1, c) == SHORT_GRASS {
                w_fires = true;
            }
            if w.get(c + 1, surface + 1, c) == SHORT_GRASS {
                e_fires = true;
            }
        }
        assert!(the_spread_fires, "the 1/10 row the 256 seeds fire");
        assert!(
            s_fires && w_fires && e_fires,
            "the shuffled order the S/W/E rotations speak across the seeds"
        );
    }

    /// THE SPREAD'S OWN SHUFFLE LAW (javap BushBlock.performBonemeal →
    /// BonemealableBlock.findSpreadableNeighbourPos →
    /// Direction$Plane.shuffledCopy → Util.shuffledCopy → Util.shuffle
    /// over the real 26.2 jar — the walked tuft's own
    /// BonemealableBlock dispatch the grass walk's 1/10 row wears):
    /// the gate the enum-order presence speaks (the N/S/W/E stream the
    /// first-valid answer keeps, NO draw the verdict spends), the
    /// Fisher-Yates the manual replay matches (the size-4 faces the
    /// THREE draws spend — nextInt(4), nextInt(3), nextInt(2), the
    /// swap(i-1, j) descent), the first valid neighbour the shuffled
    /// order plants (the N refusal the order skips even when the
    /// shuffle names it), the walk's own fired row the predicted cell
    /// plants (the 1/10 draw + the three shuffle draws the same stream
    /// spends), the walked BUSH the block-identity compare refuses
    /// (the sibling instance the row never spreads — the ring the
    /// below-stone refusals keep every later iteration aborted, the
    /// world the walk never moves).
    #[test]
    fn the_walked_tufts_spread_wears_the_shuffle_law() {
        // THE FLAT PLAINS the same harness the 128-walk test wears
        let flat = |seed: u64| {
            let mut w = World::new(1, seed);
            let size = w.size_blocks();
            let surface = CHUNK_Y as i32 / 2;
            for x in 0..size {
                for z in 0..size {
                    for y in 0..surface {
                        w.set(x, y, z, STONE);
                    }
                    w.set(x, surface, z, GRASS);
                    for y in (surface + 1)..CHUNK_Y as i32 {
                        w.set(x, y, z, AIR);
                    }
                }
            }
            (w, size / 2, surface)
        };

        // THE GATE'S OWN ENUM ORDER: the N/S cells the stone-below
        // refusals wear, the W cell the first-valid answer keeps — the
        // enum order's own stream the presence check rides (the N/S/W/E
        // rows the plane keeps, the stone-below N and S the walk skips)
        let (mut w, c, surface) = flat(7);
        w.set(c, surface + 1, c, SHORT_GRASS);
        w.set(c, surface, c - 1, STONE);
        w.set(c, surface, c + 1, STONE);
        assert_eq!(
            spreadable_neighbour(&w, c, surface + 1, c, &HORIZONTAL_ENUM),
            Some((c - 1, surface + 1, c)),
            "the N/S refusals the W row the enum order answers"
        );
        // THE EXHAUSTED GATE: the four below-stone refusals the None
        // keeps — the isValidBonemealTarget PASS the empty walk speaks
        for (dx, dz) in HORIZONTAL_ENUM {
            w.set(c + dx, surface, c + dz, STONE);
        }
        assert_eq!(
            spreadable_neighbour(&w, c, surface + 1, c, &HORIZONTAL_ENUM),
            None,
            "the four refusals the empty verdict keeps"
        );

        // THE FISHER-YATES THE MANUAL REPLAY MATCHES: the same seed the
        // same three draws spends, the swap(i-1, j) descent the shuffle
        // keeps, the shuffled order the helper answers
        let mut expect = HORIZONTAL_ENUM;
        let mut replay = crate::drops::Lcg::new(20250922);
        for i in (2..=4usize).rev() {
            let j = replay.next_int(i as u64) as usize;
            expect.swap(i - 1, j);
        }
        assert_eq!(
            shuffled_horizontal(&mut crate::drops::Lcg::new(20250922)),
            expect,
            "the three draws the swap descent wears"
        );

        // THE WALK'S OWN WIRING: the seed the 1/10 row's first draw
        // fires AND the shuffled first-valid the W cell the enum
        // order's S answer never wears (the discriminator the old law
        // fails) — the ring the below-stone refusals keep every later
        // iteration aborted (the walked cell's GRASS-below gate the
        // ring refuses — the diagonal and the up steps the air belows
        // answer), the DIRT-below S/W cells the spread's own
        // SUPPORTS_VEGETATION subset alone answers, the walk's own run
        // the predicted cell plants and the S cell the shuffle never
        // names stays air
        let mut found = false;
        for seed in 0..4096u64 {
            let mut probe = crate::drops::Lcg::new(seed);
            if probe.next_int(10) != 0 {
                continue;
            }
            let mut dirs = HORIZONTAL_ENUM;
            for i in (2..=4usize).rev() {
                let j = probe.next_int(i as u64) as usize;
                dirs.swap(i - 1, j);
            }
            // the valid set the DIRT-below S/W cells wears (the N/E
            // cells the stone-below refusals keep) — the first valid
            // the shuffled order names the placement wears
            let predicted = *dirs
                .iter()
                .find(|off| **off == (0, 1) || **off == (-1, 0))
                .unwrap();
            if predicted == (-1, 0) {
                let (mut w, c, surface) = flat(seed);
                w.set(c, surface + 1, c, SHORT_GRASS);
                for dx in -1..=1 {
                    for dz in -1..=1 {
                        if dx != 0 || dz != 0 {
                            w.set(c + dx, surface, c + dz, STONE);
                        }
                    }
                }
                w.set(c, surface, c + 1, DIRT); // the S row the subset answers
                w.set(c - 1, surface, c, DIRT); // the W row the subset answers
                bonemeal_grass_walk(&mut w, c, surface, c, &mut crate::drops::Lcg::new(seed));
                assert_eq!(
                    w.get(c - 1, surface + 1, c),
                    SHORT_GRASS,
                    "the shuffled first-valid the walk's own row plants"
                );
                found = true;
                break;
            }
        }
        assert!(found, "the shuffled first-valid the seeds never spoke");

        // THE WALKED BUSH THE ROW NEVER SPREADS: the block-identity
        // compare the sibling instance refuses (the 1/10 row the
        // SHORT_GRASS block alone wears) — the ring the below-stone
        // refusals keep every later iteration aborted (the diagonal
        // and the up steps the air/stone belows answer), the bush cell
        // the rows never speak, the world the walk never moves
        let (mut w, c, surface) = flat(11);
        w.set(c, surface + 1, c, BUSH);
        for dx in -1..=1 {
            for dz in -1..=1 {
                if dx != 0 || dz != 0 {
                    w.set(c + dx, surface, c + dz, STONE);
                }
            }
        }
        let expect = w.clone();
        bonemeal_grass_walk(&mut w, c, surface, c, &mut crate::drops::Lcg::new(5));
        assert_eq!(w, expect, "the walked bush the row never spreads");
        // THE SAME RING THE TUFT ROW SPEAKS: the walked SHORT_GRASS
        // the 1/10 row answers — the DIRT-below S cell the spread's own
        // subset alone answers (the rest of the ring the walked
        // GRASS-below gate aborts), the fired seed the ONE placement
        // wears, the unfired seed the walk never speaks (the world's
        // own delta the row's placement alone wears across the seeds)
        let mut the_row_fires = false;
        for seed in 0..64u64 {
            let (mut w, c, surface) = flat(seed);
            w.set(c, surface + 1, c, SHORT_GRASS);
            for dx in -1..=1 {
                for dz in -1..=1 {
                    if dx != 0 || dz != 0 {
                        w.set(c + dx, surface, c + dz, STONE);
                    }
                }
            }
            w.set(c, surface, c + 1, DIRT); // the S row the subset answers
            let base = w.clone();
            bonemeal_grass_walk(&mut w, c, surface, c, &mut crate::drops::Lcg::new(seed * 13 + 1));
            assert_eq!(
                w.get(c, surface + 1, c),
                SHORT_GRASS,
                "the pre-placed tuft the walk never removes"
            );
            let mut changed = 0;
            for x in 0..w.size_blocks() {
                for y in 0..CHUNK_Y as i32 {
                    for z in 0..w.size_blocks() {
                        if w.get(x, y, z) != base.get(x, y, z) {
                            changed += 1;
                        }
                    }
                }
            }
            assert!(
                changed <= 1,
                "the ring the row's ONE placement alone speaks — got {changed}"
            );
            if changed == 1 {
                assert_eq!(
                    w.get(c, surface + 1, c + 1),
                    SHORT_GRASS,
                    "the S cell the DIRT-below subset alone answers"
                );
                the_row_fires = true;
            }
        }
        assert!(the_row_fires, "the ringed tuft row the 64 seeds fire");
    }

    /// THE SUPPORT TAG'S OWN SUBSET (javap VegetationBlock.mayPlaceOn
    /// the SUPPORTS_VEGETATION tag read — the dirt/grass_blocks/
    /// farmland union the palette's own rows name): the two flower
    /// kinds the family fn answers, everything else refuses.
    #[test]
    fn the_support_tag_subset_wears_the_placeable_rows() {
        assert!(is_flower(DANDELION));
        assert!(is_flower(POPPY));
        assert!(!is_flower(GRASS));
        assert!(!is_flower(STONE));
        assert!(!is_flower(WHEAT));
        assert!(!is_flower(TNT));
        // THE TUFT FAMILY the replaceable verdict answers (the
        // autopush-412 round — the `.replaceable()` rows the
        // short_grass/bush chains wear, the flower chains never do)
        assert!(is_tuft(SHORT_GRASS));
        assert!(is_tuft(BUSH));
        assert!(!is_tuft(DANDELION));
        assert!(!is_tuft(GRASS));
        assert!(!is_tuft(WHEAT));
        assert!(is_replaceable(SHORT_GRASS));
        assert!(is_replaceable(BUSH));
        assert!(!is_replaceable(AIR));
        assert!(!is_replaceable(WATER));
        assert!(!is_replaceable(DANDELION));
    }

    /// THE TORCH'S OWN LAW WALKS (the dev-32 round): the kind's own
    /// registry row, the survival gate the canSupportCenter verdict
    /// keeps, the placeable lookup the item's own block answers, the
    /// honest refusals the vanilla's own canSurvive speaks.
    #[test]
    fn the_torch_wears_the_survival_and_registry_laws() {
        // THE KIND'S OWN ROWS: the palette id, the NAME_OF row, the
        // generated atlas mirror (the KIND_NAMES row the regen
        // re-binds), the family fn the ONE member keeps
        assert_eq!(TORCH, 54);
        assert_eq!(NAME_OF[TORCH as usize], "torch");
        assert_eq!(
            mc_registry::generated::atlas::KIND_NAMES[TORCH as usize],
            "torch",
            "the atlas mirror the regen re-binds"
        );
        assert!(is_torch(TORCH));
        assert!(!is_torch(STONE) && !is_torch(AIR) && !is_torch(POPPY));
        // THE REGISTRY'S OWN ITEM — the pick lane's id walk answers
        assert_eq!(
            mc_registry::item_id_by_name("torch").is_some(),
            true,
            "the torch item the craft chain grants"
        );
        // THE PLACEABLE LOOKUP: the item's own block the reverse walk
        // answers, the torch NOT replaceable (the vanilla
        // canBeReplaced absence — the torch cell never gives way)
        assert_eq!(World::placeable_block("torch"), Some(TORCH));
        assert!(!is_replaceable(TORCH));
        // THE SURVIVAL LAW (javap BaseTorchBlock.canSurvive -> the
        // canSupportCenter(below, UP) delegate): the full-cube
        // collision cells the sturdy answer keeps — the ground, the
        // sand, the farmland, the leaves (the vanilla's own
        // torch-on-leaves verdict the full-cube law wears), the
        // crafting table; the air, the fluids, the noCollision plants
        // and the torch itself the walk-through verdict refuses
        assert!(can_support_torch(STONE));
        assert!(can_support_torch(GRASS));
        assert!(can_support_torch(DIRT));
        assert!(can_support_torch(SAND));
        assert!(can_support_torch(FARMLAND));
        assert!(can_support_torch(LEAVES));
        assert!(can_support_torch(CRAFTING_TABLE));
        assert!(!can_support_torch(AIR), "the floating torch never survives");
        assert!(!can_support_torch(WATER) && !can_support_torch(LAVA));
        assert!(!can_support_torch(WHEAT) && !can_support_torch(DANDELION));
        assert!(!can_support_torch(SHORT_GRASS) && !can_support_torch(TALL_GRASS));
        assert!(!can_support_torch(TORCH), "the torch the torch never supports");
    }

    /// THE GRASS TUFT'S OWN DECORATE WALK (the autopush-412 round — the
    /// patch_grass_plain + patch_bush placed features' honest stand-in
    /// — javap the jar's worldgen json the noise machinery the client's
    /// own hash lane never carries): the seed's replay the same world
    /// wears, the tufts the grass cells alone carry (the beach/water
    /// columns the patch never plants, the canSurvive substrate the
    /// tuft family demands), the density the two features' own walk
    /// bands keep (the ~1/3 grass-column coverage the
    /// noise_threshold_count x count-32 spread answers, the ~1/43 the
    /// bush's rarity-4 x count-24 row keeps), the AIR gate the
    /// first-feature-wins order keeps (the trunk and the bloom the
    /// cell never share), the kinds the two state providers name.
    #[test]
    fn the_tuft_patch_wears_the_decorate_law() {
        let w = World::new(2, 42);
        let replay = World::new(2, 42);
        assert_eq!(w, replay, "the seed's replay the same world wears");
        let mut short_grass = 0;
        let mut bush = 0;
        let mut grass_columns = 0;
        for x in 0..w.size_blocks() {
            for z in 0..w.size_blocks() {
                for y in 0..w.size_blocks() {
                    let id = w.get(x, y, z);
                    if is_flower(id) && w.get(x, y - 1, z) == GRASS {
                        // THE FIRST-FEATURE-WINS ORDER: the tuft pass
                        // plants BEFORE the flower walk — a column both
                        // rolls keep the tuft (the AIR gate the flower
                        // walk's own refusal keeps)
                        assert!(
                            !is_tuft(w.get(x, y, z)),
                            "the cell the two decorations never share"
                        );
                    }
                    if !is_tuft(id) {
                        continue;
                    }
                    assert!(id == SHORT_GRASS || id == BUSH, "the two kinds the roll picks");
                    // THE SUPPORT'S OWN GATE (the below cell the grass
                    // block keeps — the beach/water columns the patch
                    // never plants, the canSurvive substrate the tuft
                    // family demands)
                    assert_eq!(
                        w.get(x, y - 1, z),
                        GRASS,
                        "the tuft the grass cell alone wears"
                    );
                    assert!(y > WATER_LEVEL);
                    // THE CELL'S OWN EXCLUSIVITY (the AIR gate — the
                    // tuft never replaces the trunk or the bloom)
                    if id == SHORT_GRASS {
                        short_grass += 1;
                    } else {
                        bush += 1;
                    }
                }
            }
            for z in 0..w.size_blocks() {
                // the grass-column census rides the same x pass (the
                // top-surface read the decorate walk answers)
                let top = w.height(x as i32, z as i32);
                if top > WATER_LEVEL && w.get(x as i32, top, z as i32) == GRASS {
                    grass_columns += 1;
                }
            }
        }
        // the density bands (the ~2-chunk world's grass columns): the
        // short_grass ~1/3 the patch_grass_plain walk answers, the bush
        // ~1/43 the patch_bush rarity keeps
        let grass = short_grass + bush;
        assert!(
            grass >= 1,
            "the tufts the patch plants: {short_grass}+{bush} over {grass_columns} columns"
        );
        assert!(
            short_grass >= 1,
            "the simple_state_provider's short_grass the dominant row keeps"
        );
        let share = short_grass as f64 / grass_columns.max(1) as f64;
        assert!(
            (0.05..=0.65).contains(&share),
            "the short_grass share the band keeps: {share}"
        );
        if bush == 0 {
            // the rarity-4 x count-24 walk may land zero tufts in a
            // two-chunk world — the honest sparse row
            assert!(short_grass >= 1, "the grass row alone still speaks");
        }
    }

    /// THE DOUBLE PLANT'S OWN DECORATE + REGISTRY LAWS (the autopush-413
    /// round — the patch_tall_grass_2 placed feature's honest stand-in +
    /// the audited family walks): the seed's replay the same world wears,
    /// the tall grass the grass cells alone carry, the pair the TWO
    /// cells wear (the LOWER the substrate answers, the UPPER the
    /// below-read the placeAt law keeps), the ~1/12 band the
    /// noise_threshold_count x rarity-32 x count-96 walk answers, the
    /// first-feature-wins order the tuft/flower cells never cross, the
    /// family fns the four kinds answer (the mate swap, the shared
    /// NAME_OF row, the replaceable verdict, the placeable lookup the
    /// LOWER kind alone wears).
    #[test]
    fn the_double_plant_patch_wears_the_decorate_law() {
        // THE FAMILY'S OWN REGISTRY ROWS (the audited walks): the four
        // kinds the range keeps, the mate the two pairs swap, the
        // NAME_OF the BASE name both halves speak, the replaceable the
        // clinit's `.replaceable()` rows answer, the placeable lookup
        // the FIRST match keeps (the lower kind the placement lane
        // wears)
        assert!(is_double_plant(TALL_GRASS) && is_double_plant(TALL_GRASS_UPPER));
        assert!(is_double_plant(LARGE_FERN) && is_double_plant(LARGE_FERN_UPPER));
        assert!(!is_double_plant(SHORT_GRASS) && !is_double_plant(GRASS));
        assert!(is_double_plant_lower(TALL_GRASS) && !is_double_plant_lower(TALL_GRASS_UPPER));
        assert!(is_double_plant_upper(TALL_GRASS_UPPER) && !is_double_plant_upper(LARGE_FERN));
        assert_eq!(double_plant_mate(TALL_GRASS), Some(TALL_GRASS_UPPER));
        assert_eq!(double_plant_mate(TALL_GRASS_UPPER), Some(TALL_GRASS));
        assert_eq!(double_plant_mate(LARGE_FERN), Some(LARGE_FERN_UPPER));
        assert_eq!(double_plant_mate(LARGE_FERN_UPPER), Some(LARGE_FERN));
        assert_eq!(double_plant_mate(DANDELION), None);
        assert_eq!(NAME_OF[TALL_GRASS as usize], "tall_grass");
        assert_eq!(NAME_OF[TALL_GRASS_UPPER as usize], "tall_grass", "the shared row");
        assert_eq!(NAME_OF[LARGE_FERN as usize], "large_fern");
        assert_eq!(NAME_OF[LARGE_FERN_UPPER as usize], "large_fern", "the shared row");
        assert!(is_replaceable(TALL_GRASS) && is_replaceable(LARGE_FERN_UPPER));
        assert!(!is_replaceable(DANDELION));
        assert_eq!(
            World::placeable_block("tall_grass"),
            Some(TALL_GRASS),
            "the first match the LOWER kind keeps"
        );

        let w = World::new(2, 42);
        let replay = World::new(2, 42);
        assert_eq!(w, replay, "the seed's replay the same world wears");
        let mut lower_cells = 0;
        let mut upper_cells = 0;
        let mut grass_columns = 0;
        for x in 0..w.size_blocks() {
            for z in 0..w.size_blocks() {
                for y in 0..w.size_blocks() {
                    let id = w.get(x, y, z);
                    if id == TALL_GRASS {
                        // THE SUBSTRATE'S OWN GATE (the canSurvive
                        // fallback the VegetationBlock base wears — the
                        // below cell the grass block keeps)
                        assert_eq!(
                            w.get(x, y - 1, z),
                            GRASS,
                            "the lower half the grass cell alone wears"
                        );
                        // THE PAIR'S OWN READ (the placeAt walk — the
                        // UPPER the `pos.above()` cell keeps)
                        assert_eq!(
                            w.get(x, y + 1, z),
                            TALL_GRASS_UPPER,
                            "the pair the two cells wear"
                        );
                        lower_cells += 1;
                    } else if id == TALL_GRASS_UPPER {
                        // THE UPPER'S OWN SUBSTRATE (the canSurvive's
                        // upper arm: the below read the same block at
                        // half=lower keeps)
                        assert_eq!(
                            w.get(x, y - 1, z),
                            TALL_GRASS,
                            "the upper half the lower's own cell pairs"
                        );
                        upper_cells += 1;
                    }
                    // THE FIRST-FEATURE-WINS ORDER (the vanilla step-9
                    // row's own shape — the tall grass pass plants
                    // BEFORE the tuft/flower walks, the taken cell the
                    // later features' AIR gate refuses: a tuft/flower
                    // column never wears the pair's cells)
                    if (is_tuft(id) || is_flower(id)) && w.get(x, y - 1, z) == GRASS {
                        assert_ne!(
                            w.get(x, y + 1, z),
                            TALL_GRASS_UPPER,
                            "the pair's upper the tuft column never shares"
                        );
                    }
                }
                let top = w.height(x as i32, z as i32);
                if top > WATER_LEVEL && w.get(x as i32, top, z as i32) == GRASS {
                    grass_columns += 1;
                }
            }
        }
        assert_eq!(
            lower_cells, upper_cells,
            "the pair the two cells ALWAYS wears"
        );
        assert!(lower_cells >= 1, "the tall grass the patch plants");
        // the density band (the ~2-chunk world's grass columns): the
        // ~1/12 the noise_threshold_count x rarity-32 x count-96 walk
        // answers
        let share = lower_cells as f64 / grass_columns.max(1) as f64;
        assert!(
            (0.02..=0.25).contains(&share),
            "the tall grass share the band keeps: {share} ({lower_cells}/{grass_columns})"
        );
    }

    /// A controlled flat world: one chunk, a stone slab at y=8, the
    /// air above (the generated terrain cleared out of the ray's way).
    fn slab_world() -> World {
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                for y in 5..24 {
                    w.set(x, y, z, AIR);
                }
                w.set(x, 8, z, STONE);
            }
        }
        w
    }

    /// THE CLIP LAW — the vanilla Level.clip's COLLIDER shape over full
    /// cubes: the entry face is the stepped axis's negative direction,
    /// the location sits exactly on the crossed boundary, the fluid
    /// passes (the ClipContext.Fluid.NONE law), the reach ends the
    /// traversal (the miss), the eye inside a solid cell is the hit
    /// with isInside.
    #[test]
    fn the_clip_law_names_the_entry_face_and_the_exact_boundary() {
        let w = slab_world();
        // the straight-down ray from (8.5, 12.0, 8.5): 3.0 to the
        // surface, inside the 4.5 reach
        let hit = w
            .clip((8.5, 12.0, 8.5), (0.0, -1.0, 0.0), BLOCK_INTERACTION_RANGE)
            .expect("the floor is under the eye");
        assert_eq!(hit.pos, (8, 8, 8));
        assert_eq!(hit.face, 1, "UP — the ray entered through the slab's top");
        assert_eq!(hit.location, (8.5, 9.0, 8.5), "the face plane's own y");
        assert!(!hit.inside);
        // the cursor the wire speaks: the location minus the origin
        assert_eq!(
            (hit.location.0 - 8.0, hit.location.1 - 8.0, hit.location.2 - 8.0),
            (0.5, 1.0, 0.5)
        );
        // the reach ends the traversal: 5.0 to the surface from y=14
        assert!(
            w.clip((8.5, 14.0, 8.5), (0.0, -1.0, 0.0), 3.0).is_none(),
            "5 > 3 — the honest miss"
        );
        // THE FLUID PASSES: a water cell above the slab does not clip
        let mut w2 = slab_world();
        w2.set(8, 10, 8, WATER);
        let hit2 = w2
            .clip((8.5, 12.0, 8.5), (0.0, -1.0, 0.0), BLOCK_INTERACTION_RANGE)
            .expect("the water is skipped, the slab clips");
        assert_eq!(hit2.pos, (8, 8, 8), "the fluid never decided the hit");
        // THE SIDE FACE: the horizontal ray enters through the WEST face
        // of the wall column it steps into (+x)
        let mut w3 = slab_world();
        for y in 8..12 {
            w3.set(12, y, 8, STONE);
        }
        let hit3 = w3
            .clip((9.5, 9.5, 8.5), (1.0, 0.0, 0.0), BLOCK_INTERACTION_RANGE)
            .expect("the wall is within the reach");
        assert_eq!(hit3.pos, (12, 9, 8));
        assert_eq!(hit3.face, 4, "WEST — the +x step's entry face");
        assert_eq!(hit3.location.0, 12.0, "the boundary plane exactly");
        // THE EYE-INSIDE: the eye in a solid cell is the hit itself
        let mut w4 = slab_world();
        w4.set(4, 10, 4, STONE);
        let hit4 = w4
            .clip((4.5, 10.5, 4.5), (0.0, -1.0, 0.0), BLOCK_INTERACTION_RANGE)
            .expect("the eye's own cell");
        assert!(hit4.inside);
        assert_eq!(hit4.pos, (4, 10, 4));
        // THE DIAGONAL: the (-1,-1,-1) ray crosses the x/z boundaries
        // BEFORE the y one — the face names the LAST-stepped axis's
        // entry (UP, the slab's top)
        let hit5 = w5_diag(&w);
        assert_eq!(hit5.pos, (7, 8, 7));
        assert_eq!(hit5.face, 1, "the -y boundary is the one the ray crossed last");
        assert_eq!(hit5.location, (7.5, 9.0, 7.5));
    }

    /// THE FLUID-ANY CLIP (the boat's POV walk — javap
    /// Item.getPlayerPOVHitResult's `ClipContext.Fluid.ANY`): the
    /// fluids COUNT — the ray stops at the water's own surface (and
    /// the lava's), the solids the same lane keeps, the open air the
    /// honest miss keeps. The block clip's own NONE law (the water the
    /// pick ray passes through) rides the same walk for the contrast.
    #[test]
    fn the_fluid_any_clip_speaks_the_surface_law() {
        let mut w = slab_world();
        // the water cell above the slab — the pool the hull spawns on
        w.set(8, 11, 8, WATER);
        // THE FLUID'S OWN SURFACE: the straight-down ray stops at the
        // water cell's top boundary (the fluid the NONE law passes)
        let hit = w
            .clip_fluid_any((8.5, 14.0, 8.5), (0.0, -1.0, 0.0), BLOCK_INTERACTION_RANGE)
            .expect("the water decides the hit on this lane");
        assert_eq!(hit.pos, (8, 11, 8), "the water cell the ray entered");
        assert_eq!(hit.face, 1, "UP — the fluid's own entry face");
        assert_eq!(hit.location, (8.5, 12.0, 8.5), "the surface plane exactly");
        assert!(!hit.inside);
        // THE CONTRAST: the NONE law's own walk the same pool speaks —
        // the water transparent, the slab's top the hit (the ray from
        // 12.0: 3.0 to the slab, inside the reach)
        let none = w
            .clip((8.5, 12.0, 8.5), (0.0, -1.0, 0.0), BLOCK_INTERACTION_RANGE)
            .expect("the slab decides the NONE walk");
        assert_eq!(none.pos, (8, 8, 8), "the fluid never decided the NONE hit");
        // THE LAVA COUNTS TOO: the ANY law's own set the fluid family keeps
        let mut w2 = slab_world();
        w2.set(3, 9, 3, LAVA);
        let lava = w2
            .clip_fluid_any((3.5, 12.0, 3.5), (0.0, -1.0, 0.0), BLOCK_INTERACTION_RANGE)
            .expect("the lava decides the hit");
        assert_eq!(lava.pos, (3, 9, 3));
        assert_eq!(lava.location, (3.5, 10.0, 3.5));
        // THE SOLID STILL COUNTS: the stone's own lane the ANY walk keeps
        let stone = w2
            .clip_fluid_any((8.5, 12.0, 8.5), (0.0, -1.0, 0.0), BLOCK_INTERACTION_RANGE)
            .expect("the slab decides the ANY walk too");
        assert_eq!(stone.pos, (8, 8, 8));
        // THE OPEN AIR: the honest miss both laws speak
        let mut w3 = slab_world();
        for x in 0..16 {
            for z in 0..16 {
                w3.set(x, 8, z, AIR);
            }
        }
        assert!(
            w3.clip_fluid_any((8.5, 12.0, 8.5), (0.0, -1.0, 0.0), BLOCK_INTERACTION_RANGE)
                .is_none(),
            "no fluid, no solid — the ray ends in the open air"
        );
    }

    /// The diagonal's helper — the ray from (9.5, 11.0, 9.5) along
    /// (-1, -1, -1)/sqrt(3): the x/z boundaries cross at t≈0.87/1.73/2.6,
    /// the y plane at t≈3.46 (2√3, inside the reach) — the landing sits
    /// on the slab's top plane at (7.5, 9.0, 7.5).
    fn w5_diag(w: &World) -> ClipHit {
        w.clip((9.5, 11.0, 9.5), (-1.0, -1.0, -1.0), BLOCK_INTERACTION_RANGE)
            .expect("the slab is within the diagonal's reach")
    }

    /// THE PLACEABLE TABLE — the vanilla BlockItem's own pairing: the
    /// item's registry name IS its block's for the simple cubes; the
    /// void and the non-block items refuse.
    #[test]
    fn the_placeable_table_is_the_vocabularys_reverse() {
        assert_eq!(World::placeable_block("stone"), Some(STONE));
        assert_eq!(World::placeable_block("dirt"), Some(DIRT));
        assert_eq!(World::placeable_block("grass_block"), Some(GRASS));
        assert_eq!(World::placeable_block("oak_log"), Some(LOG));
        assert_eq!(World::placeable_block("oak_leaves"), Some(LEAVES));
        assert_eq!(World::placeable_block("sand"), Some(SAND));
        assert_eq!(World::placeable_block("deepslate"), Some(DEEPSLATE));
        assert_eq!(World::placeable_block("air"), None, "the void is never a stack");
        assert_eq!(World::placeable_block("diamond"), None, "an item is not a block");
        assert_eq!(World::placeable_block("water"), None, "no water item places");
    }

    #[test]
    fn the_furnace_facing_byte_the_looks_wear() {
        use std::f32::consts::FRAC_PI_2;
        use std::f32::consts::PI;
        // THE SOUTH LOOK (cam.yaw = PI — the vanilla yaw 0 the
        // wire_angles conversion speaks): the getDirection SOUTH
        // verdict the BY_2D_DATA[0] row keeps, the opposite the
        // NORTH byte 1 answers — the front toward the player
        assert_eq!(
            furnace_facing_byte(PI),
            1,
            "the south look faces the front north"
        );
        // THE NORTH LOOK (cam.yaw 0 — the vanilla -180): the NORTH
        // verdict the BY_2D_DATA[2] row keeps, the opposite the
        // SOUTH byte 3 wears
        assert_eq!(
            furnace_facing_byte(0.0),
            3,
            "the north look faces the front south"
        );
        // THE EAST LOOK (cam.yaw FRAC_PI_2 — the vanilla -90): the
        // EAST verdict the BY_2D_DATA[3] row keeps, the opposite the
        // WEST byte 4 wears
        assert_eq!(
            furnace_facing_byte(FRAC_PI_2),
            4,
            "the east look faces the front west"
        );
        // THE WEST LOOK (cam.yaw -FRAC_PI_2 — the vanilla -270): the
        // WEST verdict the BY_2D_DATA[1] row keeps, the opposite the
        // EAST byte 2 wears
        assert_eq!(
            furnace_facing_byte(-FRAC_PI_2),
            2,
            "the west look faces the front east"
        );
        // THE WRAP (the same look the full turn carries — the floor
        // walk the &3 the negative index answers): -PI the south
        // look again (the forward walk the -cos(-PI) +Z speaks),
        // 3*PI the same south look the +360 wears, -5*FRAC_PI_2 the
        // west look the -630 floor the &3=1 row keeps
        assert_eq!(
            furnace_facing_byte(-PI),
            1,
            "the -PI look is the same south the byte keeps"
        );
        assert_eq!(
            furnace_facing_byte(3.0 * PI),
            1,
            "the wrapped south look the same byte wears"
        );
        assert_eq!(
            furnace_facing_byte(-5.0 * FRAC_PI_2),
            2,
            "the wrapped west look the east byte wears"
        );
    }

    #[test]
    fn the_variant_stamp_the_lazy_layer_the_bounds_wear() {
        let mut w = World::new(2, 42);
        // THE LAYER'S OWN ABSENCE (the procedural scene the offline
        // ctor keeps — the variant read the 0 no-opinion answers)
        assert!(w.variants.is_none(), "the fresh world carries no layer");
        assert_eq!(w.variant(3, 4, 5), 0, "the layerless read answers none");
        // THE STAMP'S OWN ALLOCATION (the furnace byte the lazy layer
        // banks — the materialize walk's own layout the shapes keep)
        w.set_variant(3, 4, 5, 2);
        assert!(w.variants.is_some(), "the stamp the layer banks");
        assert_eq!(w.variant(3, 4, 5), 2, "the stamped byte the read answers");
        assert_eq!(w.variant(4, 4, 5), 0, "the neighbor the no-opinion keeps");
        // THE OVERWRITE (the re-placement facing the byte re-speaks)
        w.set_variant(3, 4, 5, 1);
        assert_eq!(w.variant(3, 4, 5), 1, "the new facing the byte wears");
        // THE BOUNDS (the outside-grid cell the honest no-write
        // carries — the layer the allocation never pays, the panics
        // the walks never speak)
        w.set_variant(3, -1, 5, 3);
        w.set_variant(3, 48, 5, 3); // CHUNK_Y the top bound refuses
        w.set_variant(100, 4, 5, 3); // the unsent chunk the guard keeps
        w.set_variant(3, 4, -9, 3);
        assert_eq!(w.variant(3, 4, 5), 1, "the refused writes the byte keeps");
        // THE BOUNDLESS READ (the outside walks the 0 no-opinion
        // answers — the variant read's own law the same shape keeps)
        assert_eq!(w.variant(100, 4, 5), 0, "the unsent chunk reads none");
        assert_eq!(w.variant(3, -1, 5), 0, "the below reads none");
    }

    #[test]
    fn face_tint_folds_only_a_changed_color() {
        let baked = atlas::TINTS[WATER as usize][0];
        assert_ne!(baked, 0, "water is a tinted texture");
        assert_eq!(baked, 0x3F76E4, "the baked water default IS vanilla plains");
        let mut w = World::new(1, 42);
        // id 0 = plains (water == the baked constant), id 1 = swamp-ish
        w.biomes = Some(layer(vec![[None, None, Some(baked), None], [None, None, Some(0x617B64), None]]));
        assert_eq!(
            w.face_tint(WATER, 0, 8, 10, 8),
            None,
            "a tint matching the baked default is a no-op"
        );
        // switch the voxel's cell to id 1 — the changed color folds
        let idx = 8 + 8 * CHUNK_X + 10 * CHUNK_X * CHUNK_Z;
        w.biomes.as_mut().unwrap().ids[0][idx] = 1;
        assert_eq!(w.face_tint(WATER, 0, 8, 10, 8), Some(0x617B64));
        // an untinted face never tints, with or without an entry
        assert_eq!(w.face_tint(STONE, 2, 8, 10, 8), None);
        // grass top without an override keeps the baked plains grass
        assert_eq!(w.face_tint(GRASS, 2, 8, 12, 8), None);
        assert_eq!(w.face_tint(GRASS, 0, 8, 12, 8), None, "grass side untinted");
    }

    #[test]
    fn face_tint_walks_overrides_and_unknowns() {
        let mut w = World::new(1, 42);
        // the badlands family: real overrides on both tint slots
        w.biomes = Some(layer(vec![[Some(0x90814D), Some(0x9E814D), Some(0x3F76E4), None]]));
        assert_eq!(w.face_tint(GRASS, 2, 0, 0, 0), Some(0x90814D), "the grass override");
        assert_eq!(w.face_tint(LEAVES, 2, 0, 20, 0), Some(0x9E814D), "the foliage override");
        // above the world and outside the grid: no opinion
        assert_eq!(w.face_tint(LEAVES, 2, 0, CHUNK_Y as i32, 0), None);
        assert_eq!(w.face_tint(LEAVES, 2, 20, 0, 0), None, "past chunks_per_side");
        assert_eq!(w.face_tint(LEAVES, 2, 0, -1, 0), None, "below the world");
        // an id the tint table never named: the baked look
        w.biomes.as_mut().unwrap().ids[0][0] = 7;
        assert_eq!(w.face_tint(GRASS, 2, 0, 0, 0), None);
        // the UNKNOWN cell: the baked look
        w.biomes.as_mut().unwrap().ids[0][0] = BIOME_UNKNOWN;
        assert_eq!(w.face_tint(GRASS, 2, 0, 0, 0), None);
    }

    /// THE GROWTH LADDER — the CropBlock.randomTick law over chosen
    /// seeds: the isolated field's modulus 13 the mirror pair pins
    /// (the fresh-seed replay speaks the roll the tick consumed), the
    /// ladder climbs one kind per success to the stage-7 ceiling, the
    /// max-age branch and the non-crop cell draw NOTHING (the clone
    /// probe the draw-count discipline keeps).
    #[test]
    fn the_crop_grows_the_ladder_climbs() {
        // THE CONTROLLED FIELD: the cleared volume the formula's own
        // reads isolate — farmland center + the air ring below, the
        // same-layer neighbors air (the isolated field f = 2.0 keeps)
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                for y in 16..32 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        w.set(8, 19, 8, FARMLAND);
        w.set(8, 20, 8, WHEAT);
        // THE SEED WALK (the sheep round's own discipline): a seed the
        // roll lands within the isolated field's modulus 13
        let mut landed = None;
        for s in 0..8192u64 {
            let mut rng = crate::drops::Lcg::new(s);
            if w.crop_random_tick(&mut rng, 8, 20, 8) {
                landed = Some(s);
                break;
            }
        }
        let s = landed.expect("a modulus-13 seed within the walk");
        assert_eq!(w.get(8, 20, 8), WHEAT_STAGE1, "the ladder's first rung");
        // THE MIRROR PAIR: the fresh-seed replay's first draw IS the
        // roll — nextInt(13) == 0 the tick spoke (the f2i truncation
        // 25.0/2.0 -> 12 -> 13 the javap law keeps)
        let mut mirror = crate::drops::Lcg::new(s);
        assert_eq!(mirror.next_int(13), 0, "the isolated field's own modulus");
        // THE LADDER CLIMBS: one kind per success — the audited
        // getStateForAge(age + 1) setBlock the walk re-wears
        for stage in 1..7u16 {
            let mut hit = false;
            for s in 0..8192u64 {
                let mut rng = crate::drops::Lcg::new(s);
                if w.crop_random_tick(&mut rng, 8, 20, 8) {
                    hit = true;
                    break;
                }
            }
            assert!(hit, "the stage {stage} cell grew");
            assert_eq!(w.get(8, 20, 8), WHEAT + stage + 1, "the next rung");
        }
        assert_eq!(w.get(8, 20, 8), WHEAT_STAGE7, "the ceiling reached");
        // THE CEILING DRAWS NOTHING (the if_icmpge branch the roll
        // never reaches — the clone probe the zero-draw verdict keeps)
        let mut rng = crate::drops::Lcg::new(7);
        let mut probe = rng.clone();
        assert!(!w.crop_random_tick(&mut rng, 8, 20, 8), "the max age refuses");
        assert_eq!(rng.next_int(1000), probe.next_int(1000), "zero draws at 7");
        // THE NON-CROP CELL DRAWS NOTHING (the isRandomlyTicking gate)
        let mut rng = crate::drops::Lcg::new(7);
        let mut probe = rng.clone();
        assert!(!w.crop_random_tick(&mut rng, 8, 20, 9), "the air cell refuses");
        assert_eq!(rng.next_int(1000), probe.next_int(1000), "zero draws on air");
    }

    /// THE GROWTH SPEED — the audited getGrowthSpeed formula's dyadic
    /// anchors: the isolated dry farmland 2.0, the 3x3 farmland ring
    /// 4.0 (the quartered non-center cells), one cardinal crop
    /// neighbor 2.5 then one diagonal 1.75 (the two sequential
    /// (f+1)/2 halvings the javap tail's fold chain keeps), the
    /// farmland-less below-plane 1.0; the family helpers the ladder's
    /// own vocabulary wears.
    #[test]
    fn the_growth_speed_the_neighbors_keep() {
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                for y in 16..32 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        // the isolated dry farmland: 1.0 + the center hit 1.0
        w.set(8, 19, 8, FARMLAND);
        w.set(8, 20, 8, WHEAT);
        assert_eq!(w.crop_growth_speed(8, 20, 8), 2.0);
        // the 3x3 farmland ring: 1.0 + 1.0 + 8 quarters = 4.0
        for x in 7..=9 {
            for z in 7..=9 {
                w.set(x, 19, z, FARMLAND);
            }
        }
        assert_eq!(w.crop_growth_speed(8, 20, 8), 4.0);
        // one cardinal crop neighbor (a mid-ladder stage — the .is
        // law the age never gates): (4.0 + 1.0) / 2.0
        w.set(7, 20, 8, WHEAT_STAGE3);
        assert_eq!(w.crop_growth_speed(8, 20, 8), 2.5);
        // + one diagonal: (2.5 + 1.0) / 2.0
        w.set(7, 20, 7, WHEAT_STAGE5);
        assert_eq!(w.crop_growth_speed(8, 20, 8), 1.75);
        // the farmland-less below-plane (the floating crop's own law —
        // randomTick never re-checks survival, the speed the bare 1.0
        // keeps): modulus (int)(25.0/1.0)+1 = 26
        w.set(12, 20, 12, WHEAT);
        assert_eq!(w.crop_growth_speed(12, 20, 12), 1.0);
        // THE FAMILY HELPERS: the ages 0..7 the kinds name, the
        // GROWS_CROPS tag the sole farmland row the json speaks
        assert_eq!(wheat_age(WHEAT), Some(0));
        assert_eq!(wheat_age(WHEAT_STAGE7), Some(7));
        assert_eq!(wheat_age(FARMLAND), None);
        assert!(grows_crops(FARMLAND));
        assert!(!grows_crops(DIRT), "the tag json's sole row is farmland");
        // THE PLACEABLE REFUSAL the whole family keeps (the harvest
        // item's no-BlockItem law — no stage the item lane plants)
        assert_eq!(World::placeable_block("wheat"), None);
        assert_eq!(World::placeable_block("wheat_stage1"), None);
        assert_eq!(World::placeable_block("wheat_stage7"), None);
    }

    /// THE HYDRATION'S OWN BOX — the isNearWater scan's exact bounds
    /// (javap: the betweenClosed(pos.offset(-4, 0, -4), pos.offset(4, 1,
    /// 4)) the 162 cells wear): the water at the ring's own edge the wet
    /// verdict speaks, the one-past cell the dry keeps, the y the
    /// farmland's own level + the one above the box wears, the
    /// below-cell water the box never scans.
    #[test]
    fn the_hydration_the_water_the_box_scans() {
        let mut w = World::new(1, 7);
        for x in 0..16 {
            for z in 0..16 {
                for y in 10..32 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        w.set(8, 19, 8, FARMLAND);
        assert!(
            !w.is_near_water(8, 19, 8),
            "the cleared world the dry keeps"
        );
        // THE RING'S OWN EDGE: the dx=+4 cell the box scans
        w.set(12, 19, 8, WATER);
        assert!(
            w.is_near_water(8, 19, 8),
            "the dx=4 edge the betweenClosed box scans"
        );
        w.set(12, 19, 8, AIR);
        // ONE PAST THE EDGE: the dx=+5 cell the box never scans
        w.set(13, 19, 8, WATER);
        assert!(
            !w.is_near_water(8, 19, 8),
            "the dx=5 cell the box never scans"
        );
        w.set(13, 19, 8, AIR);
        // THE Y-BAND: the cell ABOVE the farmland level (the flooded
        // furrow) the box scans, the below-cell water it never wears
        // (the betweenClosed's y the offset(4, 1, 4) corner keeps)
        w.set(8, 20, 8, WATER);
        assert!(
            w.is_near_water(8, 19, 8),
            "the y+1 cell the box scans"
        );
        w.set(8, 20, 8, AIR);
        w.set(8, 18, 8, WATER);
        assert!(
            !w.is_near_water(8, 19, 8),
            "the y-1 cell the box never scans"
        );
        w.set(8, 18, 8, AIR);
        // THE Z EDGE: the dz=+4 cell the box scans
        w.set(8, 19, 12, WATER);
        assert!(
            w.is_near_water(8, 19, 8),
            "the dz=4 edge the betweenClosed box scans"
        );
    }

    /// THE WET ARM'S OWN SPEEDS — the getGrowthSpeed's moisture law the
    /// hydration scan answers: the isolated wet farmland 4.0 (the below
    /// hit the 3.0 speaks), the dry boundary the out-of-box water keeps
    /// (2.0 — the anchor the water-present world re-pins), the 3x3 wet
    /// ring 10.0 (the center 3.0 + the eight quarters 0.75 — the water
    /// the intersection corner all nine boxes scan), the corner water
    /// the honest mixed verdict wears (8.0: the four near cells the wet
    /// quarter speaks, the four far cells the dry quarter keeps).
    #[test]
    fn the_growth_speed_the_wet_arm_speaks() {
        let mut w = World::new(1, 43);
        for x in 0..16 {
            for z in 0..16 {
                for y in 16..32 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        // the isolated WET farmland: 1.0 + 3.0 = 4.0
        w.set(8, 19, 8, FARMLAND);
        w.set(8, 20, 8, WHEAT);
        w.set(4, 19, 8, WATER); // the dx=-4 edge the box scans
        assert_eq!(w.crop_growth_speed(8, 20, 8), 4.0);
        // the DRY BOUNDARY: the water one-past the box the 1.0 arm keeps
        w.set(4, 19, 8, AIR);
        w.set(2, 19, 8, WATER);
        assert_eq!(
            w.crop_growth_speed(8, 20, 8),
            2.0,
            "the far ring the dry arm keeps"
        );
        w.set(2, 19, 8, AIR);
        // the 3x3 WET ring: 1.0 + 3.0 + 8 x 0.75 = 10.0 — the water the
        // intersection corner (5, 5) every farmland cell's box scans
        for x in 7..=9 {
            for z in 7..=9 {
                w.set(x, 19, z, FARMLAND);
            }
        }
        w.set(5, 19, 5, WATER);
        assert_eq!(
            w.crop_growth_speed(8, 20, 8),
            10.0,
            "the wet ring the 3.0 quarters wear"
        );
        // THE MIXED CORNER: the water at (4, 4) — the center + the
        // three near ring cells wet (3.0 + 3 x 0.75), the five far
        // ring cells dry (5 x 0.25) — 1.0 + 3.0 + 2.25 + 1.25 = 7.5
        w.set(5, 19, 5, AIR);
        w.set(4, 19, 4, WATER);
        assert_eq!(
            w.crop_growth_speed(8, 20, 8),
            7.5,
            "the corner water the honest mixed verdict wears"
        );
    }

    /// THE WET FIELD'S OWN HASTE — the random tick's modulus the speed
    /// speaks: the isolated wet field (25.0/4.0 -> 6 -> 7) the dry
    /// isolated (25.0/2.0 -> 12 -> 13) the same seed the divergent roll
    /// wears (the seed the wet grows the dry keeps), the determinism the
    /// replay keeps, the one-draw law the post-tick streams pair (the
    /// modulus the only lane that moved).
    #[test]
    fn the_wet_field_the_random_tick_hastes() {
        // THE DIVERGENT SEED: the fresh-stream first draws — the wet's
        // next_int(7) the zero speaks, the dry's next_int(13) the
        // non-zero keeps (two probes the fresh pair wear)
        let mut seed = 1u64;
        loop {
            let mut p1 = crate::drops::Lcg::new(seed);
            let mut p2 = crate::drops::Lcg::new(seed);
            if p1.next_int(7) == 0 && p2.next_int(13) != 0 {
                break;
            }
            seed += 1;
        }
        let build = |water: bool| -> World {
            let mut w = World::new(1, 44);
            for x in 0..16 {
                for z in 0..16 {
                    for y in 16..32 {
                        w.set(x, y, z, AIR);
                    }
                }
            }
            w.set(8, 19, 8, FARMLAND);
            w.set(8, 20, 8, WHEAT);
            if water {
                w.set(4, 19, 8, WATER); // the dx=-4 edge the box scans
            }
            w
        };
        let mut wet = build(true);
        let mut dry = build(false);
        let mut rng_wet = crate::drops::Lcg::new(seed);
        let mut rng_dry = crate::drops::Lcg::new(seed);
        assert!(
            wet.crop_random_tick(&mut rng_wet, 8, 20, 8),
            "the wet roll the growth speaks"
        );
        assert!(
            !dry.crop_random_tick(&mut rng_dry, 8, 20, 8),
            "the dry modulus the growth never speaks"
        );
        assert_eq!(
            wet.get(8, 20, 8),
            WHEAT_STAGE1,
            "the wet crop the ladder climbed"
        );
        assert_eq!(
            dry.get(8, 20, 8),
            WHEAT,
            "the dry crop the stage keeps"
        );
        // THE ONE-DRAW LAW: the moduli 7 and 13 the raw draw the same
        // count spends — the post-tick streams re-join
        assert_eq!(
            rng_wet.next_int(97),
            rng_dry.next_int(97),
            "the one draw the streams pair"
        );
        // THE DETERMINISM: the same seed the same verdict the replay keeps
        let mut wet2 = build(true);
        let mut replay = crate::drops::Lcg::new(seed);
        assert!(
            wet2.crop_random_tick(&mut replay, 8, 20, 8),
            "the same seed the same growth the replay wears"
        );
        assert_eq!(
            wet2.get(8, 20, 8),
            WHEAT_STAGE1,
            "the replay the same stage wears"
        );
    }

    /// THE RANDOM TICK WALK — the ServerLevel's per-tick lane: the
    /// wheat-less world's exact 27-draw budget the mirror pair pins
    /// (3 sections x 3 picks x 3 axis draws the one chunk speaks), the
    /// field the walk lives on (growth events the family law keeps),
    /// the pre-worn stage-7 cells the ceiling holds, the farmland
    /// floor the walk never eats.
    #[test]
    fn the_walk_the_field_the_ceiling_keeps() {
        // THE BUDGET MIRROR: one chunk = 3 sections (CHUNK_Y 48), the
        // randomTickSpeed=3 the 9 picks speak, the 27 draws the clone
        // replays — the streams re-join on the next draw
        let mut bare = World::new(1, 5);
        let mut rng = crate::drops::Lcg::new(99);
        let mut probe = rng.clone();
        assert!(!bare.random_tick_walk(&mut rng), "no crop, no growth");
        for _ in 0..27 {
            probe.next_int(16);
        }
        assert_eq!(
            rng.next_int(16),
            probe.next_int(16),
            "the walk's exact 27-draw budget"
        );
        // THE FIELD: the 16x16 farmland plane the stage-0 crops wear,
        // two pre-worn stage-7 cells the ceiling pin asks
        let mut w = World::new(1, 0xC709);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 24, z, FARMLAND);
                w.set(x, 25, z, WHEAT);
            }
        }
        w.set(3, 25, 3, WHEAT_STAGE7);
        w.set(12, 25, 12, WHEAT_STAGE7);
        let mut rng = crate::drops::Lcg::new(0xC709_10CA);
        let mut grew_any = false;
        for _ in 0..6000 {
            grew_any |= w.random_tick_walk(&mut rng);
        }
        assert!(grew_any, "the walk lived — the field grew");
        // THE FAMILY LAW: every field cell wears a stage kind — the
        // ladder never leaves the family, the max-age cells the
        // ceiling holds
        for x in 0..16 {
            for z in 0..16 {
                let id = w.get(x, 25, z);
                assert!(
                    is_wheat(id),
                    "the cell {x},{z} left the family: {id}"
                );
                if (x, z) == (3, 3) || (x, z) == (12, 12) {
                    assert_eq!(id, WHEAT_STAGE7, "the stage-7 cell stayed");
                }
            }
        }
        // THE FLOOR THE WALK NEVER EATS (the randomTick the crop cell
        // alone mutates — the setBlock's own pos the soil keeps)
        assert_eq!(w.get(0, 24, 0), FARMLAND);
        assert_eq!(w.get(15, 24, 15), FARMLAND);
    }

    /// THE BAMBOO'S OWN GROWTH — the two classes' randomTick laws over
    /// chosen seeds (the autopush-616 round): the sapling arm the
    /// roll->empty->light order the pc 0-33 walk keeps (the roll the
    /// occupied cell still consumes), the growth pair the store's net
    /// state wears (the base conversion + the top the growBamboo
    /// places), the stalk arm the height law the 16 ceiling speaks,
    /// the dispatcher the one-tick one-arm law keeps, the walk the
    /// planted field lives.
    #[test]
    fn the_bamboo_the_growth_the_arms_the_wear() {
        // THE CONTROLLED FIELD: the void world the reads isolate (the
        // light fold the pass verdict keeps — no grid the gates read)
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..48 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        w.set(8, 20, 8, BAMBOO_SAPLING);
        // THE SEED WALK: a seed the modulus-3 roll lands (the pc 0-8
        // FIRST draw the sapling's own law speaks)
        let mut landed = None;
        for s in 0..64u64 {
            let mut rng = crate::drops::Lcg::new(s);
            if w.bamboo_random_tick(&mut rng, 8, 20, 8) {
                landed = Some(s);
                break;
            }
        }
        let s = landed.expect("a modulus-3 seed within the walk");
        // THE PAIR THE GROWTH WEARS: the sapling's growBamboo the
        // BAMBOO top places (pc 0-28), the updateShape conversion the
        // base speaks (pc 17-43) — the store's net state the pair keeps
        assert_eq!(
            w.get(8, 21, 8),
            BAMBOO,
            "the SMALL-leaved top growBamboo places"
        );
        assert_eq!(
            w.get(8, 20, 8),
            BAMBOO,
            "the base the updateShape conversion wears"
        );
        // THE MIRROR PAIR: the fresh-seed replay's first draw IS the roll
        let mut mirror = crate::drops::Lcg::new(s);
        assert_eq!(
            mirror.next_int(3),
            0,
            "the sapling's own modulus-3 roll"
        );
        // THE OCCUPIED-ABOVE REFUSAL still consumes the roll (the pc
        // 11-19 empty check the roll PRECEDES): the grown pair's base —
        // the roll lands, the empty verdict refuses, ONE draw spent
        let mut rng = crate::drops::Lcg::new(s);
        assert!(
            !w.bamboo_random_tick(&mut rng, 8, 20, 8),
            "the occupied above refuses"
        );
        let mut probe = crate::drops::Lcg::new(s);
        probe.next_int(3); // the roll the refusal consumed
        assert_eq!(
            rng.next_int(1000),
            probe.next_int(1000),
            "the one roll the occupied refusal spends"
        );
        // THE STALK'S OWN GROWTH: the pair's top cell grows the third
        // block (the same roll->empty->light walk the stalk arm keeps)
        let mut grew_top = false;
        for s2 in 0..64u64 {
            let mut rng = crate::drops::Lcg::new(s2);
            if w.bamboo_random_tick(&mut rng, 8, 21, 8) {
                grew_top = true;
                break;
            }
        }
        assert!(grew_top, "the 2-tall top grew the third block");
        assert_eq!(
            w.get(8, 22, 8),
            BAMBOO,
            "the new top the pos.above cell wears"
        );
        // THE MAX HEIGHT LAW (the pc 63-67 verdict): a roll-landing
        // seed the probes wear — the 16-tall column refuses, the
        // 15-tall grows the 16th
        let mut roll_seed = None;
        for s3 in 0..64u64 {
            let mut rng = crate::drops::Lcg::new(s3);
            if rng.next_int(3) == 0 {
                roll_seed = Some(s3);
                break;
            }
        }
        let s3 = roll_seed.expect("a modulus-3 seed within the walk");
        for y in 20..36 {
            w.set(12, y, 8, BAMBOO); // the 16-tall column 20..=35
        }
        let mut rng = crate::drops::Lcg::new(s3);
        assert!(
            !w.bamboo_random_tick(&mut rng, 12, 35, 8),
            "the 16-tall plant refuses"
        );
        assert_eq!(
            w.get(12, 36, 8),
            AIR,
            "the ceiling the new top never places"
        );
        for y in 20..35 {
            w.set(14, y, 8, BAMBOO); // the 15-tall column 20..=34
        }
        let mut rng = crate::drops::Lcg::new(s3);
        assert!(
            w.bamboo_random_tick(&mut rng, 14, 34, 8),
            "the 15-tall grows the 16th"
        );
        assert_eq!(
            w.get(14, 35, 8),
            BAMBOO,
            "the 16th block the growth places"
        );
        // THE DISPATCH'S OWN ROUTES (the one-tick one-arm law): the
        // wheat cell the crop arm, the rest the zero-draw refusal keeps
        w.set(4, 20, 4, WHEAT);
        let mut routed = false;
        for s4 in 0..8192u64 {
            let mut rng = crate::drops::Lcg::new(s4);
            if w.random_tick_block(&mut rng, 4, 20, 4) {
                routed = true;
                break;
            }
        }
        assert!(routed, "the wheat cell the crop arm routed");
        assert_eq!(
            w.get(4, 20, 4),
            WHEAT + 1,
            "the crop arm the ladder climbed"
        );
        let mut rng = crate::drops::Lcg::new(7);
        let mut probe = rng.clone();
        assert!(
            !w.random_tick_block(&mut rng, 8, 30, 8),
            "the air cell the dispatcher refuses"
        );
        assert_eq!(
            rng.next_int(1000),
            probe.next_int(1000),
            "zero draws on air"
        );
        // THE WALK'S OWN SWEEP: the planted bamboo the walk lives on —
        // the sapling cell the picks eventually serve, the growth the
        // pair wears (the 9 picks per walk the one chunk speaks)
        let mut w2 = World::new(1, 0xB00B00);
        for y in 22..40 {
            w2.set(6, y, 6, AIR);
        }
        w2.set(6, 20, 6, GRASS);
        w2.set(6, 21, 6, BAMBOO_SAPLING);
        let mut rng = crate::drops::Lcg::new(0xB00B10CA);
        let mut grew_any = false;
        for _ in 0..20000 {
            grew_any |= w2.random_tick_walk(&mut rng);
            if grew_any {
                break;
            }
        }
        assert!(grew_any, "the walk lived — the bamboo grew");
        assert_eq!(
            w2.get(6, 21, 6),
            BAMBOO,
            "the walk's growth the base conversion wears"
        );
        assert_eq!(
            w2.get(6, 22, 6),
            BAMBOO,
            "the walk's growth the top places"
        );
    }

    #[test]
    fn the_bamboo_the_dig_the_cascade_the_wears() {
        // THE CONTROLLED FIELD: the void world the reads isolate (the
        // growth lane's own staging the walk keeps)
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..48 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        // THE FOUR-TALL COLUMN: the grass floor the stalks the
        // supports_bamboo tag the pair the plant wears
        w.set(8, 19, 8, GRASS);
        for y in 20..24 {
            w.set(8, y, 8, BAMBOO);
        }
        // THE DIG'S OWN BROADCAST (the flag-3 beat the dug cell's own
        // walk serves — the base the hand's own dig removes): the
        // ONLY unsupported stalk the direct-above cell is — the
        // second level's below the FIRST stalk's kind keeps (the
        // supports_bamboo BAMBOO row the verdict passes), the upper
        // two the same support the verdict answers
        w.set(8, 20, 8, AIR);
        assert!(
            !w.bamboo_shape_updates((8, 20, 8), 101),
            "the schedule the mesh never re-speaks"
        );
        assert_eq!(
            w.bamboo_ticks.len(),
            1,
            "the ONE live entry the Set's own dedup keeps"
        );
        assert_eq!(
            w.bamboo_ticks[0],
            (101, 8, 21, 8),
            "the direct-above stalk the scheduleTick lag arms"
        );
        assert_eq!(w.get(8, 21, 8), BAMBOO, "the lag the stalk keeps");
        // THE ONE-PER-TICK CASCADE (the pop's own re-broadcast the
        // next level's schedule the rhythm keeps): the drain's due
        // list the tick's own law answers, the bottom-up crackle the
        // vanilla's own cadence speaks
        assert!(
            w.drain_due_bamboo_ticks(100).is_empty(),
            "the not-yet-due entry the drain refuses"
        );
        assert_eq!(
            w.drain_due_bamboo_ticks(101),
            vec![(8, 21, 8)],
            "the due cell the drain gives up"
        );
        assert!(
            w.bamboo_tick(101, 8, 21, 8),
            "the failed verdict the pop serves"
        );
        assert_eq!(w.get(8, 21, 8), AIR, "the pop the AIR wears");
        assert_eq!(
            w.bamboo_ticks[0],
            (102, 8, 22, 8),
            "the pop's own re-broadcast the next level arms"
        );
        assert_eq!(
            w.drain_due_bamboo_ticks(102),
            vec![(8, 22, 8)],
            "the second level the next tick pops"
        );
        assert!(w.bamboo_tick(102, 8, 22, 8), "the cascade climbs");
        assert_eq!(
            w.drain_due_bamboo_ticks(103),
            vec![(8, 23, 8)],
            "the top the third tick pops"
        );
        assert!(w.bamboo_tick(103, 8, 23, 8), "the column gone");
        assert!(w.bamboo_ticks.is_empty(), "the queue the walk drains");
        for y in 21..24 {
            assert_eq!(w.get(8, y, 8), AIR, "the fall the column clears");
        }
        assert_eq!(
            w.get(8, 19, 8),
            GRASS,
            "the floor the walk never reads"
        );
        // THE REPAIRED SUPPORT THE QUEUE NEVER POPS (the tick's own
        // surviving arm the re-check keeps): the scheduled stalk the
        // dirt beneath the placement's own repair answers
        w.set(8, 20, 8, BAMBOO);
        w.set(8, 21, 8, BAMBOO);
        assert!(
            w.bamboo_can_survive(8, 21, 8),
            "the stalk-below the tag row keeps"
        );
        w.set(8, 20, 8, AIR);
        w.bamboo_shape_updates((8, 20, 8), 201);
        w.set(8, 20, 8, DIRT);
        assert!(
            w.bamboo_can_survive(8, 21, 8),
            "the dirt the supports_bamboo row keeps"
        );
        assert_eq!(
            w.drain_due_bamboo_ticks(201),
            vec![(8, 21, 8)],
            "the scheduled entry the drain still gives up"
        );
        assert!(
            !w.bamboo_tick(201, 8, 21, 8),
            "the repaired support the surviving arm keeps"
        );
        assert_eq!(
            w.get(8, 21, 8),
            BAMBOO,
            "the stalk the repair saves"
        );
        // THE STALE ROW THE BLOCK-MATCH GATE DROPS (the re-dig the
        // queue's stale entry the verdict refuses)
        w.set(8, 21, 8, AIR);
        assert!(
            !w.bamboo_tick(202, 8, 21, 8),
            "the stale row the gate drops"
        );
        // THE SAPLING'S OWN SAME-BEAT DEATH (the pc 0-16 AIR return
        // the one-tick lag never waits): the dug floor the sapling
        // the SAME walk clears, the queue the sapling never fills
        w.set(12, 19, 8, GRASS);
        w.set(12, 20, 8, BAMBOO_SAPLING);
        w.set(12, 19, 8, AIR);
        assert!(
            w.bamboo_shape_updates((12, 19, 8), 301),
            "the sapling's death the mesh re-speaks"
        );
        assert_eq!(
            w.get(12, 20, 8),
            AIR,
            "the sapling the same-beat AIR serves"
        );
        assert!(w.bamboo_ticks.is_empty(), "the sapling never schedules");
        // THE DEATH'S OWN CHAIN (the dying cell's own broadcast the
        // queue extends): the stalk the sapling held the schedule arms
        // (the sapling the supports_bamboo row the stalk's verdict
        // keeps — the support's loss the schedule speaks)
        w.set(12, 19, 8, GRASS);
        w.set(12, 20, 8, BAMBOO_SAPLING);
        w.set(12, 21, 8, BAMBOO);
        w.set(12, 19, 8, AIR);
        w.bamboo_shape_updates((12, 19, 8), 401);
        assert_eq!(w.get(12, 20, 8), AIR, "the sapling the same beat dies");
        assert_eq!(
            w.bamboo_ticks[0],
            (401, 12, 21, 8),
            "the held stalk the chain's schedule arms"
        );
        // THE CONVERSION ARM (the sapling's own updateShape pc 17-42 —
        // the UP-neighbor bamboo the placed pair the sapling itself
        // becomes): the placed bamboo the changed cell's own kind
        // arms, the sapling the stalk the conversion wears
        w.set(14, 19, 8, DIRT);
        w.set(14, 20, 8, BAMBOO_SAPLING);
        w.set(14, 21, 8, BAMBOO);
        assert!(
            w.bamboo_shape_updates((14, 21, 8), 501),
            "the conversion the mesh re-speaks"
        );
        assert_eq!(
            w.get(14, 20, 8),
            BAMBOO,
            "the sapling itself the conversion wears"
        );
        // THE DIRT-UNDER REFUSAL THE CONVERSION NEVER ARMS (the pc 33
        // is(BAMBOO) read the placed kind alone passes): the dirt the
        // changed cell wears the sapling the AIR verdict keeps — the
        // below-support read the sapling's own survival answers
        w.set(14, 20, 8, BAMBOO_SAPLING);
        w.set(14, 21, 8, DIRT);
        assert!(
            !w.bamboo_shape_updates((14, 21, 8), 601),
            "the non-bamboo neighbor the conversion refuses"
        );
        assert_eq!(
            w.get(14, 20, 8),
            BAMBOO_SAPLING,
            "the supported sapling the walk keeps"
        );
    }

    #[test]
    fn the_plant_offset_wears_the_bit_slice_laws() {
        // THE PLANT'S OWN RENDER OFFSET (the dev-27 round — the javap
        // 26.2 law the world.rs walk keeps): the seed the Mth.getSeed
        // column hash wears, the offset the two lambda bodies speak,
        // the NONE kinds the zero triple keeps, the pair the rigid
        // column ride.

        // THE SEED LAW'S OWN VECTORS (the ctypes i64 chain the audit
        // cross-checks — the wrapping i32 X branch, the long Z branch,
        // the l*l*42317861 + l*11 square, the >>16 arithmetic tail)
        assert_eq!(mth_get_seed(8, 0, 8), -90_067_312_761_588);
        assert_eq!(mth_get_seed(0, 0, 0), 0, "the origin the zero hash wears");
        assert_eq!(mth_get_seed(1, 0, 0), 133_076_631_897_947);
        assert_eq!(mth_get_seed(5, 0, 7), -115_421_699_722_986);
        assert_eq!(mth_get_seed(12, 0, 3), 101_377_481_606_222);
        // THE Y THE HASH ITSELF CARRIES — the getSeed(III) walk the
        // `iload_1; i2l; lxor` chain speaks (the y the hash mixes);
        // the Y-ZERO law lives at the OFFSET LAMBDA'S CALL SITE (the
        // `getSeed(getX(), 0, getZ())` iconst_0 walk — plant_offset
        // the same pass wears, the pair's rigid ride below proves)
        assert_eq!(mth_get_seed(8, 9, 8), -79_511_424_355_496);
        assert_eq!(mth_get_seed(8, 200, 8), -24_401_587_200_254);
        // the i32-wrap branch the wide X wears (the imul; i2l order —
        // the wrapping product the long Z branch never shares)
        assert_eq!(mth_get_seed(1_048_576, 0, 1_048_576), 93_879_231_486_944);

        // THE KIND ROWS (the audited Properties chains): the flowers
        // and the double plants the XZ row wear, the tufts the XYZ
        // row wear, every other kind the NONE verdict keeps
        assert_eq!(plant_offset_type(DANDELION), PlantOffset::XZ);
        assert_eq!(plant_offset_type(POPPY), PlantOffset::XZ);
        assert_eq!(plant_offset_type(TALL_GRASS), PlantOffset::XZ);
        assert_eq!(plant_offset_type(TALL_GRASS_UPPER), PlantOffset::XZ);
        assert_eq!(plant_offset_type(LARGE_FERN), PlantOffset::XZ);
        assert_eq!(plant_offset_type(LARGE_FERN_UPPER), PlantOffset::XZ);
        assert_eq!(plant_offset_type(SHORT_GRASS), PlantOffset::XYZ);
        assert_eq!(plant_offset_type(BUSH), PlantOffset::XYZ);
        assert_eq!(plant_offset_type(GRASS), PlantOffset::None);
        assert_eq!(plant_offset_type(STONE), PlantOffset::None);
        assert_eq!(plant_offset_type(WHEAT), PlantOffset::None);
        assert_eq!(plant_offset_type(WHEAT_STAGE7), PlantOffset::None);

        // THE XZ LAMBDA'S OWN FOLD (the exact f32/f64 chain the
        // literals embed — the slice k/15.0f32 the float division
        // runs, the (v-0.5)*0.5 the double lane wears, the clamp the
        // ±0.25 bounds keep): the flower cell at (8, 9, 8) the audit's
        // own triple answers
        let (dx, dy, dz) = plant_offset(DANDELION, 8, 9, 8);
        assert_eq!(dx, 0.150_000_005_960_464_48);
        assert_eq!(dy, 0.0, "the XZ row the zero Y keeps");
        assert_eq!(dz, -0.083_333_328_366_279_6);
        // the clamp's own boundary the origin cell speaks (k=0 ->
        // (0-0.5)*0.5 = -0.25 the min bound equals) and the far
        // column the max bound answers (the lattice the clamp fits)
        let (dx0, _, dz0) = plant_offset(POPPY, 0, 5, 0);
        assert_eq!(dx0, -0.25);
        assert_eq!(dz0, -0.25);
        let (dxm, _, _) = plant_offset(POPPY, 100, 5, -50);
        assert_eq!(dxm, 0.25);

        // THE XYZ LAMBDA'S OWN FOLD: the tuft the vertical band wears
        // — (v-1.0) * 0.2f32 the grounded sink speaks, the [-maxV, 0]
        // range the dsub keeps
        let (dx, dy, dz) = plant_offset(SHORT_GRASS, 8, 9, 8);
        assert_eq!(dx, 0.150_000_005_960_464_48, "the x the XZ rows share");
        assert_eq!(dy, -0.200_000_002_980_232_24);
        assert_eq!(dz, -0.083_333_328_366_279_6);
        let (_, dy, _) = plant_offset(BUSH, 1, 3, 0);
        assert_eq!(dy, -0.133_333_333_333_333_3);
        // the tuft's y the seed's own slice wears — the same seed the
        // Y-zero law hashes, the column's tufts the same sink speak
        let (_, dy0, _) = plant_offset(SHORT_GRASS, 8, 0, 8);
        let (_, dy1, _) = plant_offset(SHORT_GRASS, 8, 40, 8);
        assert_eq!(dy0, dy1);

        // THE PAIR'S OWN RIGID RIDE: the two-cell plant the SAME
        // offset wears — the lower and the upper cell the column
        // shares, the tall grass the two quads never tears
        let lower = plant_offset(TALL_GRASS, 8, 9, 8);
        let upper = plant_offset(TALL_GRASS_UPPER, 8, 10, 8);
        assert_eq!(lower, upper, "the pair the rigid offset rides");

        // THE DETERMINISM THE WORLD NEVER RE-ROLLS (the replay the
        // same triple wears — the render the seed's own law keeps)
        for (x, z) in [(3i32, 11i32), (-7, 4), (41, -19), (256, 256)] {
            for kind in [DANDELION, SHORT_GRASS, TALL_GRASS, TALL_GRASS_UPPER] {
                let a = plant_offset(kind, x, 9, z);
                let b = plant_offset(kind, x, 9, z);
                assert_eq!(a, b, "the replay the same offset wears ({kind} @ {x},{z})");
                // the bounds the law keeps: x/z inside ±0.25, the y the
                // XZ kinds the zero wear and the tufts the sink band
                let (dx, dy, dz) = a;
                assert!((-0.25..=0.25).contains(&dx), "the x the band keeps");
                assert!((-0.25..=0.25).contains(&dz), "the z the band keeps");
                if plant_offset_type(kind) == PlantOffset::XZ {
                    assert_eq!(dy, 0.0);
                } else {
                    assert!((-0.200_000_002_980_232_24..=0.0).contains(&dy));
                }
            }
        }

        // THE LATTICE'S OWN SPREAD (the 16-level slice the vanilla
        // look keeps — the offsets the sample the multiple levels
        // speak, the degenerate zero-hash world the constant field
        // never wears)
        let mut levels = std::collections::HashSet::new();
        for x in 0..64 {
            let (dx, _, _) = plant_offset(TALL_GRASS, x, 9, x * 7 + 1);
            levels.insert(
                (dx / 0.5 * 15.0).round() as i64, // the k the slice answers
            );
        }
        assert!(
            levels.len() >= 8,
            "the x offsets the lattice's levels speak: {levels:?}"
        );

        // THE NONE KINDS THE ZERO TRIPLE KEEPS (the cubes, the crop
        // ladder, the fluids — the cross path the plants alone wear)
        assert_eq!(plant_offset(STONE, 8, 9, 8), (0.0, 0.0, 0.0));
        assert_eq!(plant_offset(WHEAT, 8, 9, 8), (0.0, 0.0, 0.0));
        assert_eq!(plant_offset(WHEAT_STAGE7, 8, 9, 8), (0.0, 0.0, 0.0));
        assert_eq!(plant_offset(GRASS, 8, 9, 8), (0.0, 0.0, 0.0));
        assert_eq!(plant_offset(WATER, 8, 9, 8), (0.0, 0.0, 0.0));
    }

    /// THE BAMBOO PAIR'S OWN PALETTE ROWS (the autopush-608 round —
    /// javap Blocks clinit pc 23084-23141 the sapling + pc 23144-23212
    /// the stalk the 26.2 jar): the tail slots the palette wake banks,
    /// the dig rows (1.0, false) the instabreak-then-strength(1.0f)
    /// override pairs the generated tables bank, the SWORD lane the
    /// f32::MAX instant rows the block_sword table banks (the
    /// SWORD_INSTANTLY_MINES tag the autopush-595 round pinned — the
    /// NAME_OF rows the lane finally wakes), the drops the generated
    /// rows speak (the stalk the self-entry, the sapling the BAMBOO
    /// item entry the loot serves), the sounds the BAMBOO families the
    /// name-driven lane reads, the XZ offset rows the offsetType(XZ)
    /// clinit calls keep, the collision rows the SHAPE_COLLISION
    /// column(3.0) + the noCollision() pc 23110 speak, the SUPPORTS_
    /// BAMBOO gate the tag's palette subset keeps, the placement law
    /// the three-way getStateForPlacement walk serves, the placeable
    /// lookups the item form's own law answers.
    #[test]
    fn the_bamboo_pair_the_palette_rows_the_wake_wears() {
        // THE TAIL SLOTS the palette wake banks
        assert_eq!(BAMBOO, 87, "the stalk the 87th slot banks");
        assert_eq!(BAMBOO_SAPLING, 88, "the sapling the 88th slot banks");
        assert_eq!(NAME_OF[BAMBOO as usize], "bamboo");
        assert_eq!(NAME_OF[BAMBOO_SAPLING as usize], "bamboo_sapling");
        // THE DIG ROWS the instabreak-then-strength pairs bank (the
        // generated tables' own dumps the clinit walks agree)
        assert_eq!(
            mc_registry::generated::block_dig::dig_row("bamboo"),
            Some((1.0, false)),
            "the stalk the (1.0, false) the strength(1.0f) override banks"
        );
        assert_eq!(
            mc_registry::generated::block_dig::dig_row("bamboo_sapling"),
            Some((1.0, false)),
            "the sapling the same pair the override banks"
        );
        // THE SWORD LANE the f32::MAX instant rows the NAME_OF wakes
        assert_eq!(
            mc_registry::generated::block_sword::sword_rule("bamboo"),
            Some((3.4028235e38, false)),
            "the stalk the instant MAX the SWORD_INSTANTLY_MINES row banks"
        );
        assert_eq!(
            mc_registry::generated::block_sword::sword_rule("bamboo_sapling"),
            Some((3.4028235e38, false)),
            "the sapling the same instant row banks"
        );
        // THE DROPS the generated rows speak (the stalk the self-entry,
        // the sapling the bamboo item the loot serves)
        let stalk_drops = mc_registry::generated::block_drops::drops("bamboo")
            .expect("the stalk's drops row");
        assert!(
            stalk_drops.iter().any(|p| p.entries.iter().any(|e| e.item == "bamboo")),
            "the stalk the self-entry drops"
        );
        let sapling_drops = mc_registry::generated::block_drops::drops("bamboo_sapling")
            .expect("the sapling's drops row");
        assert!(
            sapling_drops.iter().any(|p| p.entries.iter().any(|e| e.item == "bamboo")),
            "the sapling the bamboo item drops (the item-less block the loot law keeps)"
        );
        // THE SOUNDS the name-driven lane reads (the break events the
        // BAMBOO families carry)
        let stalk_idx = mc_registry::generated::block_sounds::sound_index("bamboo")
            .expect("the stalk in the sound table");
        assert_eq!(
            mc_registry::generated::block_sounds::SOUND_BREAK[stalk_idx],
            "block.bamboo.break",
            "the stalk the BAMBOO family banks"
        );
        let sapling_idx = mc_registry::generated::block_sounds::sound_index("bamboo_sapling")
            .expect("the sapling in the sound table");
        assert_eq!(
            mc_registry::generated::block_sounds::SOUND_BREAK[sapling_idx],
            "block.bamboo_sapling.break",
            "the sapling the BAMBOO_SAPLING family banks"
        );
        // THE XZ OFFSET ROWS the offsetType(XZ) clinit calls keep
        assert_eq!(plant_offset_type(BAMBOO), PlantOffset::XZ);
        assert_eq!(plant_offset_type(BAMBOO_SAPLING), PlantOffset::XZ);
        // THE COLLISION ROWS the SHAPE_COLLISION + noCollision speak
        assert_eq!(
            collision_shape(BAMBOO),
            CollisionShape::InsetColumn(16, 6),
            "the stalk the 3/16 column the inset-6 band the sweep contains"
        );
        assert_eq!(
            collision_shape(BAMBOO_SAPLING),
            CollisionShape::Empty,
            "the sapling the noCollision row keeps"
        );
        // THE SUPPORTS GATE the tag's palette subset keeps
        assert!(supports_bamboo(GRASS), "the grass_block the tag banks");
        assert!(supports_bamboo(DIRT), "the dirt the #dirt subtag banks");
        assert!(supports_bamboo(SAND), "the sand the #sand subtag banks");
        assert!(supports_bamboo(GRAVEL), "the gravel the tag banks");
        assert!(supports_bamboo(BAMBOO), "the stalk the tag's own member keeps");
        assert!(
            supports_bamboo(BAMBOO_SAPLING),
            "the sapling the tag's own member keeps"
        );
        assert!(!supports_bamboo(STONE), "the stone the tag refuses");
        assert!(!supports_bamboo(AIR), "the air the tag refuses");
        assert!(!supports_bamboo(WATER), "the water the tag refuses");
        // THE PLACEMENT LAW the three-way walk serves (pc 46-73 the
        // sapling-below stalk arm, pc 74-125 the bamboo-below arm,
        // pc 126-184 the plain-ground sapling arm, pc 21-43 the gate)
        assert_eq!(
            bamboo_placement_kind(GRASS),
            Some(BAMBOO_SAPLING),
            "the plain ground the SAPLING arm serves (the vanilla's own quirk)"
        );
        assert_eq!(
            bamboo_placement_kind(BAMBOO_SAPLING),
            Some(BAMBOO),
            "the sapling-below the STALK arm serves"
        );
        assert_eq!(
            bamboo_placement_kind(BAMBOO),
            Some(BAMBOO),
            "the bamboo-below the STALK arm serves (the age the kind-only vocabulary never stores)"
        );
        assert_eq!(
            bamboo_placement_kind(STONE),
            None,
            "the stone the gate's refusal keeps"
        );
        // THE PLACEABLE LOOKUPS the item form's own law answers
        assert_eq!(
            World::placeable_block("bamboo"),
            Some(BAMBOO),
            "the bamboo item the BlockItem pairing places"
        );
        assert_eq!(
            World::placeable_block("bamboo_sapling"),
            None,
            "the sapling the NO-item block the lookup refuses"
        );
    }
}

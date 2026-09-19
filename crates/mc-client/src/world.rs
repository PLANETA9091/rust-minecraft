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
pub const WHEAT_STAGE1: u16 = 30;
pub const WHEAT_STAGE2: u16 = 31;
pub const WHEAT_STAGE3: u16 = 32;
pub const WHEAT_STAGE4: u16 = 33;
pub const WHEAT_STAGE5: u16 = 34;
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

/// Block -> texture-group mapping used by the real-colors LUT.
pub const NAME_OF: [&str; 38] = [
    "air", "stone", "dirt", "grass_block", "oak_log", "oak_leaves", "water", "sand",
    "granite", "diorite", "andesite", "deepslate", "cobbled_deepslate", "gravel",
    "bedrock", "coal_ore", "copper_ore", "iron_ore", "gold_ore", "redstone_ore",
    "lapis_ore", "diamond_ore", "emerald_ore", "netherrack", "pale_oak_leaves",
    "leaf_litter", "lava", "farmland", "dirt_path", "wheat",
    "wheat_stage1", "wheat_stage2", "wheat_stage3", "wheat_stage4",
    "wheat_stage5", "wheat_stage6", "wheat_stage7", "crafting_table",
];

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
        let mut w = Self {
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
            thunder_level: 0.0, // the procedural scene: no storm the clouds speak
        };
        w.generate();
        w
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
            self.chunks[ci][lx + lz * CHUNK_X + wy as usize * CHUNK_X * CHUNK_Z] = id;
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
        let id = self.biome_id(layer, wx, wy, wz)?;
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
    /// `None` = the item places nothing this client's world vocabulary
    /// can render — the honest no-place, never a guessed voxel.
    pub fn placeable_block(item_name: &str) -> Option<u16> {
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

    /// THE RANDOM TICK WALK (the ServerLevel's per-tick lane the vanilla
    /// game law speaks: `randomTickSpeed = 3` — THREE random cells per
    /// 16³ chunk section per game tick, each the block's
    /// isRandomlyTicking/randomTick the crop lane answers). The offline
    /// world's own authority (the networked world the server ticks —
    /// the same split the mob lane speaks): every section the walk
    /// sweeps (the chunk-major order the tickChunk walk keeps), three
    /// `nextInt(16)` draws per axis the pick wears, the wheat cell the
    /// [`World::crop_random_tick`] law answers. Returns whether ANY
    /// crop grew (the caller's remesh the batch grew speaks once).
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
                        grew |= self.crop_random_tick(rng, wx, wy, wz);
                    }
                }
            }
        }
        grew
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
}

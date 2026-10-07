//! `net/minecraft/world/level/gameevent/vibrations/VibrationSystem` —
//! THE SCULK LANE'S OWN VOCABULARY (javap, verbatim): the vibration
//! frequency map the static lambda fills and the redstone strength fold
//! the sensor's hearing rides.
//!
//! - `VIBRATION_FREQUENCY_FOR_EVENT` — the
//!   `Reference2IntOpenHashMap<ResourceKey<GameEvent>>` the
//!   `lambda$static$0` fills ONCE: `defaultReturnValue(0)` (the
//!   NO_VIBRATION_FREQUENCY the unknown events answer), then the 41
//!   `(event, frequency)` pairs the bytecode walk banks (the getstatic
//!   field → `key()` → the iconst/bipush value → `put` — the pairing
//!   the javap dump serves, the registry names the GameEvent static
//!   init's ldc strings speak).
//! - `getGameEventFrequency(Holder)` — the `unwrapKey().map(freq
//!   lookup).orElse(0)` walk: the map's default the unknown key
//!   answers.
//! - `getRedstoneStrengthForDistance(float distance, int range)` —
//!   javap verbatim: `d2 = 15.0 / (double) range; return Math.max(1,
//!   15 - Mth.floor(d2 * (double) distance))` — pcs 0-21: the
//!   `ldc2_w 15.0d; iload_1; i2d; ddiv; dstore_2` the divisor walk
//!   banks, the `iconst_1` the MAX's own first arg (the fold NEVER
//!   returns 0 — the zero the sensor's own active gate keeps, the
//!   caller's is-active answer), the `bipush 15` the `isub`'s own head,
//!   the `Mth.floor` the toward-negative-infinity fold the d2i carries.
//!   The sculk sensor's own caller (`SculkSensorBlockEntity$VibrationUser`)
//!   passes the `bipush 8` listen radius — getListenerRadius()'s own
//!   answer.
//!
//! The record lane's `f{frequency}` suffix the map answers (the drain
//! walk calls `vibration_frequency` — the literal the prior round wore
//! the map's own verdict retires); the redstone fold the listener round
//! rides (the sensor block's own hearing a future round lands).

/// `VibrationSystem.NO_VIBRATION_FREQUENCY` — the interface constant and
/// the map's own default: the unknown events answer 0 (the
/// `defaultReturnValue(0)` the lambda's head banks).
pub const NO_VIBRATION_FREQUENCY: i32 = 0;

/// THE VIBRATION FREQUENCY MAP (javap `lambda$static$0`, verbatim — the
/// 41 pairs the getstatic→key→iconst/bipush→put walk banks, the order
/// the bytecode speaks). The registry names the GameEvent static init's
/// ldc strings carry (the lowercase of the field names, the
/// `unequip`/`resonate_*` tails the same walk answers).
const VIBRATION_FREQUENCIES: &[(&str, i32)] = &[
    ("step", 1),
    ("swim", 1),
    ("flap", 1),
    ("projectile_land", 2),
    ("hit_ground", 2),
    ("splash", 2),
    ("bounce", 2),
    ("item_interact_finish", 3),
    ("projectile_shoot", 3),
    ("instrument_play", 3),
    ("entity_action", 4),
    ("elytra_glide", 4),
    ("unequip", 4),
    ("entity_dismount", 5),
    ("equip", 5),
    ("entity_interact", 5),
    ("shear", 6),
    ("entity_mount", 6),
    ("entity_damage", 7),
    ("drink", 8),
    ("eat", 8),
    ("container_close", 9),
    ("block_close", 9),
    ("block_deactivate", 9),
    ("block_detach", 9),
    ("container_open", 10),
    ("block_open", 10),
    ("block_activate", 10),
    ("block_attach", 10),
    ("prime_fuse", 10),
    ("note_block_play", 10),
    ("block_change", 11),
    ("block_destroy", 12),
    ("fluid_pickup", 12),
    ("block_place", 13),
    ("fluid_place", 13),
    ("entity_place", 14),
    ("lightning_strike", 14),
    ("teleport", 14),
    ("entity_die", 15),
    ("explode", 15),
];

/// THE SENSOR'S OWN LISTEN RADIUS (javap
/// `SculkSensorBlockEntity$VibrationUser.getListenerRadius` — the
/// `bipush 8` the walk answers): the in-range gate the
/// `getPostableListenerPosition` walk spends (the radius² the distSqr
/// bounds) AND the redstone fold's own range arg the
/// onReceiveVibration pair serves.
pub const SCULK_SENSOR_LISTEN_RADIUS: i32 = 8;

/// THE SENSOR'S OWN RE-ARM WINDOW (javap SculkSensorBlock — the
/// activate walk's `scheduleTick(pos, block, getActiveTicks())` the
/// `bipush 30` answers, the deactivate walk's `scheduleTick(pos,
/// block, 10)` the `bipush 10` cooldown answers): the ACTIVE 30 +
/// the COOLDOWN 10 the INACTIVE phase the window's tail restores —
/// the sensor hears again only when the phase walks home.
pub const SCULK_SENSOR_REARM_TICKS: u64 = 40;

/// THE VIBRATIONS TAG (the 26.2 jar's
/// `data/minecraft/tags/game_event/vibrations.json`, verbatim — the 56
/// values the User.isValidVibration default's tag consult reads; the
/// shriek/jukebox_play/item_interact_start trio the tag NEVER carries,
/// the resonate family the tail keeps).
const VIBRATION_TAG: &[&str] = &[
    "block_attach",
    "block_change",
    "block_close",
    "block_destroy",
    "block_detach",
    "block_open",
    "block_place",
    "block_activate",
    "block_deactivate",
    "bounce",
    "container_close",
    "container_open",
    "drink",
    "eat",
    "elytra_glide",
    "entity_damage",
    "entity_die",
    "entity_dismount",
    "entity_interact",
    "entity_mount",
    "entity_place",
    "entity_action",
    "equip",
    "explode",
    "fluid_pickup",
    "fluid_place",
    "hit_ground",
    "instrument_play",
    "item_interact_finish",
    "lightning_strike",
    "note_block_play",
    "prime_fuse",
    "projectile_land",
    "projectile_shoot",
    "shear",
    "splash",
    "step",
    "swim",
    "teleport",
    "unequip",
    "resonate_1",
    "resonate_2",
    "resonate_3",
    "resonate_4",
    "resonate_5",
    "resonate_6",
    "resonate_7",
    "resonate_8",
    "resonate_9",
    "resonate_10",
    "resonate_11",
    "resonate_12",
    "resonate_13",
    "resonate_14",
    "resonate_15",
    "flap",
];

/// `VibrationSystem$User.isValidVibration`'s own answer (the default
/// walk the tag consult serves): the event the VIBRATIONS tag keeps
/// hears, the untagged the refusal keeps.
pub fn is_valid_vibration(event: &str) -> bool {
    VIBRATION_TAG.contains(&event)
}

/// THE OCCLUDES TAG'S OWN ROWS (the jar's
/// `data/minecraft/tags/block/occludes_vibration_signals.json` the
/// `#minecraft:wool` REFERENCE answers — the wool tag's own 16 rows
/// the reference resolves to, the exact names the json wears, bare
/// the VIBRATION_TAG's own format keeps).
const OCCLUDES_VIBRATION_SIGNALS: &[&str] = &[
    "white_wool",
    "orange_wool",
    "magenta_wool",
    "light_blue_wool",
    "yellow_wool",
    "lime_wool",
    "pink_wool",
    "gray_wool",
    "light_gray_wool",
    "cyan_wool",
    "purple_wool",
    "blue_wool",
    "brown_wool",
    "green_wool",
    "red_wool",
    "black_wool",
];

/// The clip predicate's own answer (javap `lambda$isOccluded$0` — the
/// `state.is(BlockTags.OCCLUDES_VIBRATION_SIGNALS)` the clip's stop
/// test wears): the tag's row the true answers, the rest the false.
pub fn occludes_vibration_signals(name: &str) -> bool {
    OCCLUDES_VIBRATION_SIGNALS.contains(&name)
}

/// The world-cell fold the clip's predicate wears: the block state's
/// registry name the prefix strip answers (the `minecraft:` the
/// generated range's own names wear, the bare tag the lookup keeps).
pub fn block_occludes(state_id: u32) -> bool {
    match mc_registry::generated::block_ids::block_name_of(state_id) {
        Some(name) => {
            let bare = name.strip_prefix("minecraft:").unwrap_or(name);
            occludes_vibration_signals(bare)
        }
        None => false,
    }
}

/// THE VIBRATION_RESONATORS TAG (the jar's
/// `data/minecraft/tags/block/vibration_resonators.json` — the ONE row
/// the json wears: the amethyst the sensor's own activation makes
/// sing; the shrieker/catalyst the tag NEVER carries — the resonate
/// the neighbors alone answer).
const VIBRATION_RESONATORS: &[&str] = &["amethyst_block"];

/// The resonate scan's own predicate (javap tryResonateVibration pc
/// 43-51 — the `state.is(BlockTags.VIBRATION_RESONATORS)` the neighbor
/// test wears): the tag's row the true answers, the rest the false.
pub fn is_vibration_resonator(name: &str) -> bool {
    VIBRATION_RESONATORS.contains(&name)
}

/// The world-cell fold the resonate scan wears: the block state's
/// registry name the prefix strip answers (the block_occludes fold
/// the same shape keeps).
pub fn block_resonates(state_id: u32) -> bool {
    match mc_registry::generated::block_ids::block_name_of(state_id) {
        Some(name) => {
            let bare = name.strip_prefix("minecraft:").unwrap_or(name);
            is_vibration_resonator(bare)
        }
        None => false,
    }
}

/// THE RESONANCE EVENTS (javap VibrationSystem — the RESONANCE_EVENTS
/// list the clinit's RESONATE_1..15 key() walk fills, the
/// `getResonanceEventByFrequency`'s `List.get(f - 1)` the isub
/// serves).
const RESONANCE_EVENTS: [&str; 15] = [
    "resonate_1",
    "resonate_2",
    "resonate_3",
    "resonate_4",
    "resonate_5",
    "resonate_6",
    "resonate_7",
    "resonate_8",
    "resonate_9",
    "resonate_10",
    "resonate_11",
    "resonate_12",
    "resonate_13",
    "resonate_14",
    "resonate_15",
];

/// `VibrationSystem.getResonanceEventByFrequency(int)` (javap,
/// verbatim): `RESONANCE_EVENTS.get(f - 1)` — the frequency the index
/// serves (the tagged hearings' own 1..=15 band the caller keeps; the
/// JVM's IOOBE the out-of-band index throws, the usize underflow the
/// same refusal wears).
pub fn resonance_event_by_frequency(frequency: i32) -> &'static str {
    RESONANCE_EVENTS[(frequency - 1) as usize]
}

/// THE RESONANCE PITCH BEND (javap lambda$static$0 — the 16 semitone
/// steps the fill banks, the `NoteBlock.getPitchFromNote` fold the
/// loop wears).
const RESONANCE_PITCH_BEND_NOTES: [i32; 16] = [
    0, 0, 2, 4, 6, 7, 9, 10, 12, 14, 15, 18, 19, 21, 22, 24,
];

/// `NoteBlock.getPitchFromNote(int)` (javap, verbatim):
/// `(float)Math.pow(2.0, (note - 12) / 12.0)` — the bend the
/// RESONANCE_PITCH_BEND[frequency] draw wears (the amethyst's own
/// voice the pitch keeps; no draw the deterministic fold spends).
pub fn resonance_pitch_bend(frequency: usize) -> f32 {
    let note = RESONANCE_PITCH_BEND_NOTES[frequency];
    2.0f64.powf((note - 12) as f64 / 12.0) as f32
}

/// THE OCCLUSION WALK (javap `VibrationSystem$Listener.isOccluded` —
/// the FULL transcription): the endpoints the block-center folds keep
/// (pc 0-87 — the `Mth.floor + 0.5` both ends), the SIX rays the
/// `Direction.values()` loop serves (pc 89-165), each start the
/// `relative(direction, 9.999999747378752E-6)` nudge wears (pc 116-122
/// — the epsilon the widened float keeps), the clip the
/// `Level.isBlockInLine` walk rides (the OCCLUDES predicate the clip's
/// own tag test wears): the ANY-clean-ray rule the `!= BLOCK → false`
/// return serves (pc 144-157), the all-blocked verdict the loop's exit
/// speaks (pc 164: iconst_1).
pub fn vibration_occluded(
    from: (f64, f64, f64),
    to: (f64, f64, f64),
    mut occludes_at: impl FnMut(i32, i32, i32) -> bool,
) -> bool {
    let from_c = (
        crate::mth::floor(from.0) as f64 + 0.5,
        crate::mth::floor(from.1) as f64 + 0.5,
        crate::mth::floor(from.2) as f64 + 0.5,
    );
    let to_c = (
        crate::mth::floor(to.0) as f64 + 0.5,
        crate::mth::floor(to.1) as f64 + 0.5,
        crate::mth::floor(to.2) as f64 + 0.5,
    );
    // Direction.values() — the vanilla enum order the loop keeps (the
    // boolean the order never reads — the ANY rule the set alone serves)
    const DIRS: [(f64, f64, f64); 6] = [
        (0.0, -1.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, 0.0, -1.0),
        (0.0, 0.0, 1.0),
        (-1.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
    ];
    for dir in DIRS {
        let start = (
            from_c.0 + dir.0 * 9.999999747378752e-6,
            from_c.1 + dir.1 * 9.999999747378752e-6,
            from_c.2 + dir.2 * 9.999999747378752e-6,
        );
        if !clip_hits_occluder(start, to_c, &mut occludes_at) {
            return false;
        }
    }
    true
}

/// `BlockGetter.traverseBlocks` (javap, the FULL DDA the clip walk
/// rides) wearing the OCCLUDES predicate: the -1e-7 lerp both
/// endpoints wear (pc 17-111), the FROM-side block the walk's head
/// checks FIRST (pc 113-167), the step/t-max trio the frac folds keep
/// (pc 189-341), the loop the smallest-t axis steps (pc 342-450) and
/// every entered cell checks (pc 452-479), the clean end the t>1 exit
/// speaks (pc 342-360 → 483-491).
fn clip_hits_occluder(
    from: (f64, f64, f64),
    to: (f64, f64, f64),
    occludes_at: &mut impl FnMut(i32, i32, i32) -> bool,
) -> bool {
    if from == to {
        return false;
    }
    let d0 = crate::mth::lerp(-1.0e-7, to.0, from.0);
    let d1 = crate::mth::lerp(-1.0e-7, to.1, from.1);
    let d2 = crate::mth::lerp(-1.0e-7, to.2, from.2);
    let d3 = crate::mth::lerp(-1.0e-7, from.0, to.0);
    let d4 = crate::mth::lerp(-1.0e-7, from.1, to.1);
    let d5 = crate::mth::lerp(-1.0e-7, from.2, to.2);
    let (mut i, mut j, mut k) = (
        crate::mth::floor(d3),
        crate::mth::floor(d4),
        crate::mth::floor(d5),
    );
    // the START cell the walk's head check fires (pc 149-167)
    if occludes_at(i, j, k) {
        return true;
    }
    let (dx, dy, dz) = (d0 - d3, d1 - d4, d2 - d5);
    let (sx, sy, sz) = (
        crate::mth::sign(dx),
        crate::mth::sign(dy),
        crate::mth::sign(dz),
    );
    let step_x = if sx == 0 { f64::MAX } else { sx as f64 / dx };
    let step_y = if sy == 0 { f64::MAX } else { sy as f64 / dy };
    let step_z = if sz == 0 { f64::MAX } else { sz as f64 / dz };
    let mut t_max_x = step_x
        * if sx > 0 {
            1.0 - crate::mth::frac(d3)
        } else {
            crate::mth::frac(d3)
        };
    let mut t_max_y = step_y
        * if sy > 0 {
            1.0 - crate::mth::frac(d4)
        } else {
            crate::mth::frac(d4)
        };
    let mut t_max_z = step_z
        * if sz > 0 {
            1.0 - crate::mth::frac(d5)
        } else {
            crate::mth::frac(d5)
        };
    loop {
        // the clean end the t>1 exit speaks — the ray reached `to`
        // without ever touching an occluding cell
        if t_max_x > 1.0 && t_max_y > 1.0 && t_max_z > 1.0 {
            return false;
        }
        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                i += sx;
                t_max_x += step_x;
            } else {
                k += sz;
                t_max_z += step_z;
            }
        } else if t_max_y < t_max_z {
            j += sy;
            t_max_y += step_y;
        } else {
            k += sz;
            t_max_z += step_z;
        }
        // the entered cell the check serves (pc 452-479)
        if occludes_at(i, j, k) {
            return true;
        }
    }
}

/// THE SENSOR'S OWN HEARING (javap the 26.2 dispatch walk — the four
/// laws the struct's fields keep): the acceptance the walk's own order
/// keeps —
/// 1. the VIBRATIONS tag (the isValidVibration default the tag consult
///    answers),
/// 2. the IN-RANGE GATE (javap
///    `EuclideanGameEventListenerRegistry.getPostableListenerPosition`
///    — the INTEGER block-pos distSqr the listener's own radius²
///    bounds: `distSqr > radius * radius` the refusal wears, the EQUAL
///    case the `dcmpl; ifle` walk passes),
/// 3. the VIBRATION (javap `Listener.scheduleVibration` — the
///    VibrationInfo ctor's distance the `Vec3.distanceTo → d2f` fold
///    banks: the event's own Vec3 to the sensor's BlockPositionSource
///    answer, the `Vec3.atCenterOf` the +0.5 fold speaks),
/// 4. the ARRIVAL (javap `User.calculateTravelTimeInTicks` default —
///    the `Mth.floor(F)` the f2d→Math.floor→d2i walk answers, the
///    vibration 1 block/tick the floor speaks),
/// 5. the STRENGTH (javap
///    `SculkSensorBlockEntity$VibrationUser.onReceiveVibration` — the
///    `distanceBetweenInBlocks` the integer-coord sqrt folds, the
///    redstone fold's own range arg the bipush-8 radius keeps).
/// The OCCLUSION walk the call site's own ray serves (javap
/// `VibrationSystem$Listener.isOccluded` — the six-ray clip the wool
/// tag answers, the `vibration_occluded` walk the world's cells
/// wears); the PHASE gate the call site's free-at window keeps (the
/// data-busy + the active/cooldown pair the same fold subsumes).
pub struct SensorHearing {
    /// the vibration's travel distance — the VibrationInfo ctor's
    /// `distanceTo` answer in f32 (the d2f the fold speaks)
    pub distance: f32,
    /// the arrival tick count — the `Mth.floor(distance)` the travel
    /// law keeps (1 block/tick)
    pub arrival_ticks: i32,
    /// the sensor's last-vibration frequency — the map's own answer
    /// (the setLastVibrationFrequency walk the onReceive pair banks)
    pub frequency: i32,
    /// the redstone power the activation carries — the fold's own
    /// answer (the POWER property the activate walk wears)
    pub redstone_strength: i32,
}

/// `VibrationSystem$Listener.distanceBetweenInBlocks(BlockPos, BlockPos)`
/// (javap, verbatim): `sqrt(BlockPos.distSqr(Vec3i))` as f32 — the
/// INTEGER coordinate distance the redstone fold's own arg keeps.
pub fn distance_between_in_blocks(a: (i32, i32, i32), b: (i32, i32, i32)) -> f32 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    let dz = (a.2 - b.2) as f64;
    ((dx * dx + dy * dy + dz * dz).sqrt()) as f32
}

/// THE WORLD'S OWN SENSOR CELL (the listener registry's analog — the
/// vanilla's block entity the chunk-section registry walk registers,
/// the cell the App's vec stands in): the pos the block cell keeps,
/// the free-at the phase window's own head banks (the earliest tick
/// the cell hears again — the arrival + the ACTIVE 30 + the COOLDOWN
/// 10 the re-arm law folds), the pending voice window the activation's
/// own walk banks — the CLICKING the arrival tick answers (javap
/// SculkSensorBlock.activate pcs 73-141: the WATERLOGGED `ifne` gate
/// the dry cell passes, the DDD `playSound(null, x+0.5, y+0.5, z+0.5,
/// SCULK_CLICKING, BLOCKS, 1.0F, random.nextFloat() * 0.2F + 0.8F)`
/// the level's own RandomSource the draw rides), the CLICKING_STOP
/// the re-arm tail speaks (javap tick pcs 56-81: the COOLDOWN→INACTIVE
/// setBlock the voice follows, the SAME 1.0F / nextFloat()*0.2F+0.8F
/// pair the ServerLevel's own random keeps).
pub struct SculkSensorCell {
    pub pos: (i32, i32, i32),
    pub free_at: u64,
    /// the arrival tick the SCULK_CLICKING voice answers — the
    /// scheduleVibration's own travel the activation rides (the
    /// onReceiveVibration walk the arrival tick speaks, the post
    /// tick the voice never wears)
    pub clicking_at: Option<u64>,
    /// the re-arm tail tick the SCULK_CLICKING_STOP voice answers —
    /// the free-at's own twin (the COOLDOWN→INACTIVE setBlock the
    /// voice follows, the tick walk's WATERLOGGED gate the dry cell
    /// passes)
    pub stop_at: Option<u64>,
    /// the vibration's own frequency the hearing armed — the activate
    /// walk's arg6 the resonate walk rides (the
    /// tryResonateVibration's iload_3 the faload index keeps; 0 = no
    /// hearing has armed the cell)
    pub resonate_frequency: i32,
}

/// THE HEARING WALK (the laws the SensorHearing doc names, the order
/// the vanilla dispatch keeps): the untagged event the None keeps, the
/// out-of-range sensor the None keeps, the hearing the Some speaks.
pub fn sensor_hearing(
    event: &str,
    event_pos: (f64, f64, f64),
    sensor: (i32, i32, i32),
) -> Option<SensorHearing> {
    // THE TAG GATE (the isValidVibration default the tag consult keeps)
    if !is_valid_vibration(event) {
        return None;
    }
    // THE IN-RANGE GATE (getPostableListenerPosition — the block-pos
    // distSqr the radius² bounds; the BlockPos.containing the floor
    // fold serves, the EQUAL case the strict `>` refusal passes)
    let eb = (
        crate::mth::floor(event_pos.0),
        crate::mth::floor(event_pos.1),
        crate::mth::floor(event_pos.2),
    );
    let dx = (eb.0 - sensor.0) as f64;
    let dy = (eb.1 - sensor.1) as f64;
    let dz = (eb.2 - sensor.2) as f64;
    let dist_sqr = dx * dx + dy * dy + dz * dz;
    let radius_sqr = (SCULK_SENSOR_LISTEN_RADIUS * SCULK_SENSOR_LISTEN_RADIUS) as f64;
    if dist_sqr > radius_sqr {
        return None;
    }
    // THE VIBRATION'S OWN DISTANCE (the VibrationInfo ctor's
    // distanceTo → d2f fold — the sensor's listener position the
    // BlockPositionSource's atCenterOf answer keeps)
    let lx = event_pos.0 - (sensor.0 as f64 + 0.5);
    let ly = event_pos.1 - (sensor.1 as f64 + 0.5);
    let lz = event_pos.2 - (sensor.2 as f64 + 0.5);
    let distance = ((lx * lx + ly * ly + lz * lz).sqrt()) as f32;
    // THE ARRIVAL (the calculateTravelTimeInTicks default — the
    // Mth.floor(F) the f2d→Math.floor→d2i walk answers)
    let arrival_ticks = crate::mth::floor(distance as f64);
    // THE STRENGTH (the onReceiveVibration pair — the
    // distanceBetweenInBlocks the integer-coord sqrt folds, the
    // redstone fold the bipush-8 radius rides)
    let redstone_strength =
        redstone_strength_for_distance(distance_between_in_blocks(eb, sensor), SCULK_SENSOR_LISTEN_RADIUS);
    Some(SensorHearing {
        distance,
        arrival_ticks,
        frequency: vibration_frequency(event),
        redstone_strength,
    })
}

/// `VibrationSystem.getGameEventFrequency` (javap, verbatim): the
/// holder's key the map answers — the unknown event the default's zero
/// keeps (the `.orElse(0)` walk the interface's own fold speaks).
pub fn vibration_frequency(event: &str) -> i32 {
    // THE RESONATE ROWS the clinit's own loop banks (javap
    // VibrationSystem <clinit> — the `map.put(getResonanceEventByFrequency(i), i)`
    // walk the lambda's 41 the tail appends: resonate_N → N the map
    // carries, the gap the lambda-only table wore the loop retires)
    if let Some(idx) = RESONANCE_EVENTS.iter().position(|&n| n == event) {
        return idx as i32 + 1;
    }
    VIBRATION_FREQUENCIES
        .iter()
        .find(|(name, _)| *name == event)
        .map(|(_, freq)| *freq)
        .unwrap_or(NO_VIBRATION_FREQUENCY)
}

/// `VibrationSystem.getRedstoneStrengthForDistance(float, int)` (javap,
/// verbatim): `Math.max(1, 15 - Mth.floor((15.0 / range) * distance))`
/// — the iconst_1 the MAX's own floor (the fold never answers 0; the
/// zero the sensor's own active gate keeps), the 15.0/range the f64
/// divisor the dstore banks, the Mth.floor the toward-negative-infinity
/// fold the mth::floor module speaks. The sensor's own caller passes
/// the bipush-8 listen radius.
#[allow(dead_code)] // wired when the sculk listener round lands
pub fn redstone_strength_for_distance(distance: f32, range: i32) -> i32 {
    let d = 15.0f64 / range as f64;
    std::cmp::max(1, 15 - crate::mth::floor(d * distance as f64))
}

// ===== THE SHRIEKER LANE (the listen tag the clicking alone grants,
// the r8 the user's own radius keeps, the shriek walk the arrival
// arms — the dispatch's own gates the shrieker cell answers) =====

/// `data/minecraft/tags/game_event/shrieker_can_listen.json` (the
/// 26.2 jar, verbatim): the ONE row the tag carries — the sensor's
/// own tendrils-clicking broadcast the shrieker alone listens to
/// (the glide's own events the tag never grants, the resonate rows
/// the tag never carries — the shrieker's ear the sensor's activation
/// alone wakes).
pub const SHRIEKER_CAN_LISTEN: &[&str] = &["sculk_sensor_tendrils_clicking"];

/// `SculkShriekerBlockEntity$VibrationUser.getListenerRadius()` (javap):
/// the `bipush 8` the listen radius banks — the sensor's own r16
/// broadcast the shrieker hears at HALF the sensor's ear wears.
pub const SHRIEKER_LISTEN_RADIUS: i32 = 8;

/// `SculkShriekerBlockEntity.shriek` pc 40 (javap): the `bipush 90`
/// `scheduleTick(pos, block, 90)` the SHRIEKING window banks — the
/// blockstate the shriek arms, the block's own tick walk clears 90
/// ticks later (the canReceiveVibration gate the in-window hearing
/// refuses).
pub const SHRIEKER_SHRIEK_TICKS: u64 = 90;

/// `GameEvent.SHRIEK` (javap GameEvent <clinit> pc 324-331): the
/// `register("shriek", 32)` two-arg walk the `bipush 32` speaks —
/// the WIDEST radius the registry carries (the register(String)
/// single-arg walks the r16 default keep); the shriek event the
/// vibrations tag never carries (the json the row never wears) —
/// the shriek's own broadcast the dispatch's isValidVibration gate
/// refuses, no cascade the walk ever serves.
pub const SHRIEK_RADIUS: i32 = 32;

/// The tag consult the default `isValidVibration`'s own head keeps
/// (javap VibrationSystem$User — the `event.is(getListenableEvents())`
/// arm the shrieker's tag answers): the clicking row the true speaks,
/// every other event the refusal wears.
pub fn shrieker_can_listen(event: &str) -> bool {
    SHRIEKER_CAN_LISTEN.contains(&event)
}

/// THE SHRIEKER'S OWN CELL (the listener registry's analog — the
/// vanilla's block entity the chunk-section registry walk registers,
/// the cell the App's vec stands in): the pos the block cell keeps,
/// the free-at the SHRIEKING window's own tail banks (the scheduleTick
/// 90 walk the shriek arms — the earliest tick the cell hears again),
/// the pending arrival the scheduled vibration banks (the travel law
/// the sensor's own arrival keeps; Some = the data busy the second
/// hearing refuses — the dispatch's own pc 22-31 the busy walk keeps).
pub struct SculkShriekerCell {
    pub pos: (i32, i32, i32),
    pub free_at: u64,
    pub shriek_at: Option<u64>,
}

/// THE SHRIEKER'S HEARING WALK (the gates the dispatch's own ladder
/// keeps — the tag the default isValidVibration's head consults, the
/// range the dispatcher's radius² bound serves, the distance the
/// VibrationInfo ctor's d2f fold speaks): the untagged event the None
/// keeps, the out-of-range shrieker the None keeps, the hearing the
/// Some(distance) speaks — the frequency/redstone fields the shrieker
/// lane never wears (the map no row carries, the POWER property the
/// shrieker's state never banks). The SHRIEKING + player-source gates
/// the caller's own walk keeps (the canReceiveVibration pair the
/// handleGameEvent ladder answers AFTER the dispatcher's range walk).
pub fn shrieker_hearing(
    event: &str,
    event_pos: (f64, f64, f64),
    shrieker: (i32, i32, i32),
) -> Option<f32> {
    // THE TAG GATE (the isValidVibration default the shrieker's own
    // tag consult keeps — the ONE row the clicking alone grants)
    if !shrieker_can_listen(event) {
        return None;
    }
    // THE IN-RANGE GATE (the dispatcher's block-pos distSqr the
    // radius² bounds; the BlockPos.containing the floor fold serves,
    // the EQUAL case the strict `>` refusal passes)
    let eb = (
        crate::mth::floor(event_pos.0),
        crate::mth::floor(event_pos.1),
        crate::mth::floor(event_pos.2),
    );
    let dx = (eb.0 - shrieker.0) as f64;
    let dy = (eb.1 - shrieker.1) as f64;
    let dz = (eb.2 - shrieker.2) as f64;
    let dist_sqr = dx * dx + dy * dy + dz * dz;
    let radius_sqr = (SHRIEKER_LISTEN_RADIUS * SHRIEKER_LISTEN_RADIUS) as f64;
    if dist_sqr > radius_sqr {
        return None;
    }
    // THE VIBRATION'S OWN DISTANCE (the VibrationInfo ctor's
    // distanceTo → d2f fold — the shrieker's listener position the
    // BlockPositionSource's atCenterOf answer keeps)
    let lx = event_pos.0 - (shrieker.0 as f64 + 0.5);
    let ly = event_pos.1 - (shrieker.1 as f64 + 0.5);
    let lz = event_pos.2 - (shrieker.2 as f64 + 0.5);
    Some(((lx * lx + ly * ly + lz * lz).sqrt()) as f32)
}

/// `SculkCatalystBlockEntity$CatalystListener.getListenerRadius()`
/// (javap): the `bipush 8` the listen radius banks — the same half-ear
/// the shrieker's VibrationUser wears, the plain GameEventListener the
/// catalyst IS (no VibrationSystem the dispatch's tag/data walks
/// serve — the listener the radius² bound ALONE gates).
#[cfg_attr(not(test), expect(dead_code))]
pub const SCULK_CATALYST_LISTEN_RADIUS: i32 = 8;

/// `SculkCatalystBlockEntity$CatalystListener.bloom` pc 21-31 (javap):
/// the `bipush 8` `scheduleTick(pos, block, 8)` the PULSE window banks
/// — the blockstate's bloom flag the setBlock walk arms (pc 0-20, the
/// `setValue(PULSE, true)` + flag-3 walk), the block's own tick walk
/// (`SculkCatalystBlock.tick` pc 13-36) clears 8 ticks later (the
/// `setValue(PULSE, false)` the same flag-3 walk wears). The PULSE
/// property IS `BlockStateProperties.BLOOM` (the block's clinit pc
/// 11-14 speaks).
#[cfg_attr(not(test), expect(dead_code))]
pub const CATALYST_PULSE_TICKS: u64 = 8;

/// `SculkSpreader.addCursors` pc 4-8 (javap): the `sipush 1000`
/// `Math.min(charge, 1000)` the per-cursor cap banks — the loop
/// `while (charge > 0)` walks (pc 0-1 the `ifle 32` head), each pass
/// `new ChargeCursor(pos, capped)` the list add rides and `charge -=
/// capped` the tail answers; a 5-xp death one cursor keeps, a 1500
/// death the 1000 + 500 pair wears.
pub const SCULK_SPREADER_MAX_CHARGE: i32 = 1000;

/// `SculkSpreader.addCursor` pc 4-11 (javap): the `bipush 32`
/// `cursors.size() >= 32` the list cap banks — the 33rd cursor the
/// cap silently DROPS (the `if_icmplt 15` the refusal speaks, the add
/// the 15 pc wears); the NBT load's own `sizeLimitedListOf(32)` the
/// same cap keeps.
#[cfg_attr(not(test), expect(dead_code))]
pub const SCULK_SPREADER_MAX_CURSORS: usize = 32;

/// `SculkSpreader.createLevelSpreader` (javap — the level walk's own
/// arg quartet): `new SculkSpreader(false, SCULK_REPLACEABLE, 10, 4,
/// 10, 5)` — the `bipush 10` growthSpawnCost, the `iconst_4`
/// noGrowthRadius, the `bipush 10` chargeDecayRate, the `iconst_5`
/// additionalDecayRate the walk banks (the worldgen twin the
/// `createWorldGenSpreader` speaks — 50/1/5/10 — the isWorldGen flag
/// the iconst_0 head keeps false).
#[cfg_attr(not(test), expect(dead_code))]
pub const SCULK_SPREADER_GROWTH_SPAWN_COST: i32 = 10;
#[cfg_attr(not(test), expect(dead_code))]
pub const SCULK_SPREADER_NO_GROWTH_RADIUS: i32 = 4;
#[cfg_attr(not(test), expect(dead_code))]
pub const SCULK_SPREADER_CHARGE_DECAY_RATE: i32 = 10;
#[cfg_attr(not(test), expect(dead_code))]
pub const SCULK_SPREADER_ADDITIONAL_DECAY_RATE: i32 = 5;

/// ONE CHARGE CURSOR (the `SculkSpreader$ChargeCursor` the offline
/// lane's own stand-in): the pos the death's own walk banks (the
/// `BlockPos.containing(eventPos.relative(UP, 0.5))` the
/// handleGameEvent pc 86-99 serves — the UP half-block the death
/// point's ceiling cell wears), the charge the addCursors loop's
/// `Math.min` fold speaks, the updateDelay the spread countdown
/// rides, the decayDelay the DEFAULT charge law reads, the facings
/// the `Optional.empty()` fresh cursor keeps (the multiface set the
/// movement walk's availableFaces arms — the lane never wears yet).
#[derive(Clone, Debug, PartialEq)]
pub struct SculkCursor {
    pub pos: (i32, i32, i32),
    pub charge: i32,
    pub update_delay: i32,
    pub decay_delay: i32,
    pub facings: Option<u8>,
}

impl SculkCursor {
    /// THE PUBLIC CTOR'S OWN LAW (javap `new ChargeCursor(pos,
    /// charge)` pc 0-11: the private walk the `iconst_1` decayDelay +
    /// the `iconst_0` updateDelay + the `Optional.empty()` facings
    /// the defaults speak).
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn fresh(pos: (i32, i32, i32), charge: i32) -> SculkCursor {
        SculkCursor {
            pos,
            charge,
            update_delay: 0,
            decay_delay: CURSOR_DECAY_DELAY_START,
            facings: None,
        }
    }
}

/// THE CATALYST'S OWN CELL (the CatalystListener's analog — the
/// vanilla's block entity + spreader the cell stands in): the pos the
/// block cell keeps, the PULSE window's tail the bloom walk banks
/// (the scheduleTick 8 the earliest tick the block's own tick clears
/// — Some = the bloom flag ON), the cursors the spreader's list
/// banks (the addCursors walk the deaths grow, the 32 cap the
/// addCursor refusal keeps; the updateCursors drain the tick sweep
/// serves).
pub struct SculkCatalystCell {
    pub pos: (i32, i32, i32),
    pub pulse_off_at: Option<u64>,
    pub cursors: Vec<SculkCursor>,
}

/// THE CATALYST'S HEARING WALK (javap
/// `CatalystListener.handleGameEvent` pc 0-23 — the `event.is(
/// GameEvent.ENTITY_DIE)` head the ONLY event the gate keeps, the
/// `sourceEntity() instanceof LivingEntity` pair the source gate
/// speaks, the dispatcher's radius² bound the range walk wears): the
/// other events the None keeps, the out-of-range catalyst the None
/// keeps, the death the Some speaks. NO occlusion walk the plain
/// listener's own body carries (no isOccluded the catalyst never
/// wears — the vibration-walk gates the VibrationSystem's users keep,
/// the catalyst NOT one), NO data-busy window the second death the
/// same tick answers (the pulsing catalyst the fresh hearing keeps).
#[cfg_attr(not(test), expect(dead_code))]
pub fn catalyst_hearing(
    event: &str,
    event_pos: (f64, f64, f64),
    catalyst: (i32, i32, i32),
) -> Option<()> {
    // THE EVENT GATE (the Holder.is(ENTITY_DIE) head — the ONE event
    // the catalyst's own body answers)
    if event != "entity_die" {
        return None;
    }
    // THE IN-RANGE GATE (the dispatcher's block-pos distSqr the
    // radius² bounds; the BlockPos.containing the floor fold serves,
    // the EQUAL case the strict `>` refusal passes)
    let eb = (
        crate::mth::floor(event_pos.0),
        crate::mth::floor(event_pos.1),
        crate::mth::floor(event_pos.2),
    );
    let dx = (eb.0 - catalyst.0) as f64;
    let dy = (eb.1 - catalyst.1) as f64;
    let dz = (eb.2 - catalyst.2) as f64;
    let dist_sqr = dx * dx + dy * dy + dz * dz;
    let radius_sqr = (SCULK_CATALYST_LISTEN_RADIUS * SCULK_CATALYST_LISTEN_RADIUS) as f64;
    if dist_sqr > radius_sqr {
        return None;
    }
    Some(())
}

/// THE ADDCURSORS WALK (javap SculkSpreader.addCursors + addCursor —
/// the `while (charge > 0)` loop the 1000-cap fold serves, the 32
/// list cap the tail drops): the xp's own per-cursor charges the walk
/// banks — the sub-1000 death ONE cursor keeps, the 1500 death the
/// 1000 + 500 pair wears, the 33000 death the 33-walk the 32-cap's
/// own tail trims (the 33rd cursor the refusal speaks). The
/// `existing` arg the list's own head keeps (the death the prior
/// deaths' cursors ahead rides — the cap the JOINT list bounds).
#[cfg_attr(not(test), expect(dead_code))]
pub fn sculk_cursor_charges(xp: i32, existing: usize) -> Vec<i32> {
    let mut out = Vec::new();
    if xp <= 0 {
        return out;
    }
    let mut charge = xp;
    while charge > 0 {
        if existing + out.len() >= SCULK_SPREADER_MAX_CURSORS {
            break;
        }
        let capped = charge.min(SCULK_SPREADER_MAX_CHARGE);
        out.push(capped);
        charge -= capped;
    }
    out
}

/// `SculkSpreader$ChargeCursor.isPosUnreasonable` (javap pc 0-19): the
/// `distChessboard > 1024` refusal the `sipush 1024` speaks — the
/// chessboard distance (the max of the axis deltas the vanilla
/// BlockPos.distChessboard folds) beyond the bound the cursor's own
/// walk never serves.
pub const SCULK_SPREADER_MAX_CURSOR_DISTANCE: i32 = 1024;

/// `SculkBehaviour.getSculkSpreadDelay` default (javap the `iconst_1`
/// answer): the updateDelay the cursor's own walk re-arms each tick —
/// the spread the countdown paces.
pub const SCULK_SPREAD_DELAY: i32 = 1;

/// THE PUBLIC CTOR'S OWN DECAY START (javap `new ChargeCursor(pos,
/// charge)` pc 3: the `iconst_1` the private walk stores as
/// decayDelay — the DEFAULT charge law the first tick keeps, the
/// second answers).
pub const CURSOR_DECAY_DELAY_START: i32 = 1;

/// THE SURVIVOR POP'S OWN DIVISOR (javap `updateCursors` pc 365: the
/// `ldc2_w 2.299999952316284` the log1p fold divides — the widened
/// float the `(int)(Math.log1p(total) / 2.3) + 1` decay law carries).
pub const CHARGE_POP_DECAY_DIVISOR: f64 = 2.3;

/// ONE 3006 POP THE DRAIN BANKS (the `levelEvent(3006, pos, data)`
/// word the updateCursors walk speaks — the data the client's own
/// decode splits: `data >> 6` the charge count, `data & 63` the face
/// byte, the 0 word the dead cursor's own pop wears).
#[derive(Clone, Debug, PartialEq)]
pub struct SculkChargePop {
    pub pos: (i32, i32, i32),
    pub data: i32,
}

/// THE UNREASONABLE SKIP (javap pc 0-19 — the chessboard distance the
/// 1024 bound refuses): the max of the axis deltas the bound's own
/// comparison keeps, the strict `>` the true verdict wears.
pub fn cursor_is_pos_unreasonable(pos: (i32, i32, i32), catalyst: (i32, i32, i32)) -> bool {
    let dx = (pos.0 - catalyst.0).abs();
    let dy = (pos.1 - catalyst.1).abs();
    let dz = (pos.2 - catalyst.2).abs();
    dx.max(dy).max(dz) > SCULK_SPREADER_MAX_CURSOR_DISTANCE
}

/// THE DEFAULT CHARGE LAW (javap `SculkBehaviour$1.attemptUseCharge`
/// pc 0-15 — the non-sculk host the offline worlds wear: the
/// `decayDelay > 0` gate the charge keeps, the elapsed decay the ZERO
/// answer wears — the cursor the host's own decay kills).
pub fn default_attempt_use_charge(decay_delay: i32, charge: i32) -> i32 {
    if decay_delay > 0 {
        charge
    } else {
        0
    }
}

/// THE DEFAULT DECAY WALK (javap `SculkBehaviour$1.updateDecayDelay`
/// pc 0-7 verbatim: `Math.max(decayDelay - 1, 0)` — the countdown the
/// floor keeps).
pub fn default_update_decay_delay(decay_delay: i32) -> i32 {
    std::cmp::max(decay_delay - 1, 0)
}

/// THE SURVIVOR POP'S OWN DECAY FOLD (javap `updateCursors` pc
/// 349-372: `(int)(Math.log1p(total) / 2.3) + 1` — the ln(1+x) fold
/// the divisor speaks, the +1 tail the data word's high bits wear).
pub fn charge_pop_decay(total: i32) -> i32 {
    ((total as f64).ln_1p() / CHARGE_POP_DECAY_DIVISOR) as i32 + 1
}

/// THE MOVEMENT'S OWN NEIGHBOUR TABLE (javap
/// `SculkSpreader$ChargeCursor` static init: the
/// `BlockPos.betweenClosedStream(new BlockPos(-1,-1,-1), new
/// BlockPos(1,1,1))` walk the `lambda$static$0` fills, the
/// `lambda$static$1` filter the rejects speak — the all-three-nonzero
/// corners the stream's own 8 drops, the `ZERO` center the
/// `equals(BlockPos.ZERO)` refusal keeps — the `bipush 18` capacity
/// the count answers): the 18 offsets the x-fastest / z-middle /
/// y-outer betweenClosed order banks, the shuffle the walk's own
/// random re-orders.
pub const NON_CORNER_NEIGHBOURS: [(i32, i32, i32); 18] = [
    (0, -1, -1),             // y=-1 z=-1 x=0
    (-1, -1, 0),             // y=-1 z=0 x=-1
    (0, -1, 0),              // y=-1 z=0 x=0
    (1, -1, 0),              // y=-1 z=0 x=1
    (0, -1, 1),              // y=-1 z=1 x=0
    (-1, 0, -1),             // y=0 z=-1 x=-1
    (0, 0, -1),              // y=0 z=-1 x=0
    (1, 0, -1),              // y=0 z=-1 x=1
    (-1, 0, 0),              // y=0 z=0 x=-1
    (1, 0, 0),               // y=0 z=0 x=1
    (-1, 0, 1),              // y=0 z=1 x=-1
    (0, 0, 1),               // y=0 z=1 x=0
    (1, 0, 1),               // y=0 z=1 x=1
    (0, 1, -1),              // y=1 z=-1 x=0
    (-1, 1, 0),              // y=1 z=0 x=-1
    (0, 1, 0),               // y=1 z=0 x=0
    (1, 1, 0),               // y=1 z=0 x=1
    (0, 1, 1),               // y=1 z=1 x=0
];

/// `data/minecraft/tags/block/sculk_replaceable.json` (the 26.2 jar —
/// the json the substrate gate's `BlockTags.SCULK_REPLACEABLE` consult
/// wears): the nested refs the tag's own walk expands —
/// `#base_stone_overworld` (stone, granite, diorite, andesite, tuff,
/// deepslate), `#substrate_overworld` (#dirt: dirt, coarse_dirt,
/// rooted_dirt; #mud: mud, muddy_mangrove_roots; #moss_blocks:
/// moss_block, pale_moss_block; #grass_blocks: grass_block, podzol,
/// mycelium), `#terracotta` (the plain + the 16 colored),
/// `#nylium` (crimson, warped), `#base_stone_nether` (netherrack,
/// basalt, blackstone) — and the 14 direct rows. 52 names the flat
/// expansion banks (the membership the vein's substrate arm answers).
pub const SCULK_REPLACEABLE: &[&str] = &[
    "stone",
    "granite",
    "diorite",
    "andesite",
    "tuff",
    "deepslate",
    "dirt",
    "coarse_dirt",
    "rooted_dirt",
    "mud",
    "muddy_mangrove_roots",
    "moss_block",
    "pale_moss_block",
    "grass_block",
    "podzol",
    "mycelium",
    "terracotta",
    "white_terracotta",
    "orange_terracotta",
    "magenta_terracotta",
    "light_blue_terracotta",
    "yellow_terracotta",
    "lime_terracotta",
    "pink_terracotta",
    "gray_terracotta",
    "light_gray_terracotta",
    "cyan_terracotta",
    "purple_terracotta",
    "blue_terracotta",
    "brown_terracotta",
    "green_terracotta",
    "red_terracotta",
    "black_terracotta",
    "crimson_nylium",
    "warped_nylium",
    "netherrack",
    "basalt",
    "blackstone",
    "sand",
    "red_sand",
    "gravel",
    "soul_sand",
    "soul_soil",
    "calcite",
    "smooth_basalt",
    "clay",
    "dripstone_block",
    "end_stone",
    "red_sandstone",
    "sandstone",
    "sulfur",
    "cinnabar",
];

/// THE DIRECTIONS THE SUBSTRATE SCAN WALKS (`Direction.values()` order
/// — down, up, north, south, west, east) with the `MultifaceBlock.pack`
/// bits the ordinal law banks (`1 << dir.ordinal()` — javap pack pc
/// 28-37: the `ishl` the ordinal wears): the face bitmask the probe
/// carries, the substrate neighbor the same index answers.
const SUBSTRATE_DIRECTIONS: [(i32, i32, i32); 6] = [
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
    (-1, 0, 0),
    (1, 0, 0),
];
const SUBSTRATE_DIRECTION_BITS: [u8; 6] = [1, 2, 4, 8, 16, 32];

/// ONE CELL THE MOVEMENT'S WORLD ANSWERS (the `getBlockState` read the
/// candidate walk spends — the offline lane's own probe): the block's
/// registry name (the bare form — `sculk`, `sculk_vein`, `air`) the
/// SculkBehaviour + vein gates read, the face bitmask the multiface
/// state's six boolean props keep (the `1 << ordinal` bits — down 1,
/// up 2, north 4, south 8, west 16, east 32; the non-multiface block
/// the ZERO set keeps). The offline world's own cells the face-less
/// probe banks — the real vein faces the vein-spread round carries.
#[derive(Debug, Clone, PartialEq)]
pub struct SculkMovementProbe {
    pub name: &'static str,
    pub faces: u8,
}

/// THE SHUFFLED OFFSETS (javap `getRandomizedNonCornerNeighbourOffsets`
/// pc 0-7: `Util.shuffledCopy(NON_CORNER_NEIGHBOURS, random)` — the
/// copy the shuffle re-orders; javap `Util.shuffle` pc 7-53:
/// `for (i = size; i > 1; i--) { j = random.nextInt(i); swap(i-1, j) }`
/// — the Fisher-Yates walk the 18-element list serves, SEVENTEEN
/// `nextInt` draws the shuffle spends (nextInt(18) down to
/// nextInt(2)), EVERY movement attempt the full list re-orders).
pub fn shuffled_non_corner_offsets(rng: &mut crate::drops::Lcg) -> Vec<(i32, i32, i32)> {
    let mut list = NON_CORNER_NEIGHBOURS.to_vec();
    for i in (2..=list.len()).rev() {
        let j = rng.next_int(i as u64) as usize;
        list.swap(i - 1, j);
    }
    list
}

/// THE SCULKBEHAVIOUR PAIR (javap `getBlockBehaviour`'s instanceof
/// head — the block itself the gate consults): `SculkBlock` and
/// `SculkVeinBlock` the two implementors the registry keeps — the
/// DEFAULT fallback (`SculkBehaviour$1`) the non-sculk state's own
/// behaviour the drain's charge law reads.
fn is_sculk_behaviour_name(name: &str) -> bool {
    name == "sculk" || name == "sculk_vein"
}

/// THE STURDINESS VERDICT THE MOVEMENT'S OWN WALK KEEPS (javap
/// `BlockState.isFaceSturdy(world, pos, opposite)` the
/// `isUnobstructed` walk consults — the FULL-CUBE support shape the
/// verdict answers): the lane's own [`collision_full_block`] name law
/// the full-cube families speak, the AIR cell the exclusion keeps —
/// vanilla's air state isFaceSturdy NEVER answers (no support the
/// empty cell carries), so the obstruction walk the air neighbor the
/// UNOBSTRUCTED answer serves — and the FLUID PAIR the same exclusion
/// wears (the vein-spread round: vanilla water/lava isFaceSturdy
/// NEVER answers — the empty collision the fluids carry, the cursor's
/// movement walks THROUGH the lake cell, the vein's canAttachTo the
/// water substrate refuses). (The pop arm's own
/// isCollisionShapeFullBlock read the SAME name law shares — the
/// absent cell the full-cube majority answers THERE, the 40-burst the
/// arm's own verdict banks — the two walks' verdicts the two vanilla
/// reads keep, each its own documented law.)
fn is_face_sturdy(name: &str) -> bool {
    name != "air"
        && name != "water"
        && name != "lava"
        && crate::world_model::collision_full_block(name)
}

/// `BlockState.canBeReplaced()` — the no-arg overload the vein
/// override's `stateCanBeReplaced` tail consults (javap pc 131-136:
/// `state.canBeReplaced()` the `ifne 153` pass wears): the replaceable
/// property's own families the lane's vocabulary keeps — the airs, the
/// fluids, the snow layer, the tuft/plant family, the vine/lichen
/// climbs, the light and structure_void placeholders. The FIRE cell
/// never reaches the arm (the tag's own refusal the walk keeps first).
pub fn can_be_replaced(name: &str) -> bool {
    matches!(
        name,
        "air"
            | "cave_air"
            | "void_air"
            | "water"
            | "lava"
            | "snow"
            | "short_grass"
            | "tall_grass"
            | "fern"
            | "large_fern"
            | "dead_bush"
            | "seagrass"
            | "tall_seagrass"
            | "vine"
            | "glow_lichen"
            | "hanging_roots"
            | "light"
            | "structure_void"
            | "bubble_column"
    )
}

/// `SculkBehaviour.canChangeBlockStateOnSpread()` — the DEFAULT the
/// interface speaks (javap SculkBehaviour pc 0-1: `iconst_1` — TRUE)
/// and the SCULK BLOCK'S OWN OVERRIDE refuses (javap SculkBlock pc
/// 0-1: `iconst_0` — FALSE): the spread's post-walk re-read (ChargeCursor
/// update pc 83-110) the non-sculk hosts alone wear. The vein inherits
/// the default (no override the class carries), the air/stone cells
/// the DEFAULT behaviour answers — the sculk block alone the silent
/// state keeps.
pub fn can_change_block_state_on_spread(name: &str) -> bool {
    name != "sculk"
}

/// THE MOVEMENT'S OWN OBSTRUCTION WALK (javap
/// `SculkSpreader$ChargeCursor.isMovementUnobstructed` — the
/// `distManhattan == 1` head pc 0-10 the face-step the TRUE answer
/// keeps BEFORE any probe, the delta's three axis directions the
/// `fromAxisAndDirection(X|Y|Z, delta < 0 ? NEGATIVE : POSITIVE)` walk
/// banks — the zero delta the POSITIVE fallback keeps — then the
/// three arms: `delta.x == 0` the Y/Z pair answers, `delta.y == 0` the
/// X/Z pair, else the X/Y pair (the javap `ifeq` chain pc 89-180)).
/// Each arm's half (javap `isUnobstructed` pc 0-30): the neighbor AT
/// `from.relative(dir)` the state read answers, the
/// `isFaceSturdy(world, pos, dir.getOpposite())` verdict the
/// obstruction speaks — the lane's own full-cube approximation the
/// [`crate::world_model::collision_full_block`] name law keeps (the
/// same verdict the pop arm's count/spread split banks): the
/// full-cube neighbor sturdy → obstructed (false), the non-full
/// neighbor (vein, slab, air) → unobstructed (true).
pub fn movement_unobstructed<F>(from: (i32, i32, i32), to: (i32, i32, i32), world: &mut F) -> bool
where
    F: FnMut(i32, i32, i32, Option<u8>) -> SculkMovementProbe,
{
    let manhattan =
        (to.0 - from.0).abs() + (to.1 - from.1).abs() + (to.2 - from.2).abs();
    if manhattan == 1 {
        return true;
    }
    let delta = (to.0 - from.0, to.1 - from.1, to.2 - from.2);
    let sx = if delta.0 < 0 { -1 } else { 1 };
    let sy = if delta.1 < 0 { -1 } else { 1 };
    let sz = if delta.2 < 0 { -1 } else { 1 };
    let mut unobstructed = |x: i32, y: i32, z: i32| -> bool {
        let probe = world(from.0 + x, from.1 + y, from.2 + z, None);
        !is_face_sturdy(probe.name)
    };
    if delta.0 == 0 {
        unobstructed(0, sy, 0) || unobstructed(0, 0, sz)
    } else if delta.1 == 0 {
        unobstructed(sx, 0, 0) || unobstructed(0, 0, sz)
    } else {
        unobstructed(sx, 0, 0) || unobstructed(0, sy, 0)
    }
}

/// THE VEIN'S OWN SUBSTRATE GATE (javap
/// `SculkVeinBlock.hasSubstrateAccess` pc 0-75 — the
/// `state.is(Blocks.SCULK_VEIN)` head the NON-vein target refuses
/// (the sculk block the gate's `iconst_0` answer keeps — the
/// SculkBehaviour instanceof the candidate walk's FIRST gate alone
/// wears), then the `DIRECTIONS` scan: a face the
/// `MultifaceBlock.hasFace(state, dir)` read banks AND the neighbor AT
/// `pos.relative(dir)` the `BlockTags.SCULK_REPLACEABLE` `is` answers
/// — the face-toward-replaceable pair the true return wears).
pub fn has_substrate_access<F>(
    probe: &SculkMovementProbe,
    pos: (i32, i32, i32),
    world: &mut F,
) -> bool
where
    F: FnMut(i32, i32, i32, Option<u8>) -> SculkMovementProbe,
{
    if probe.name != "sculk_vein" {
        return false;
    }
    for (dir, bit) in SUBSTRATE_DIRECTIONS
        .iter()
        .zip(SUBSTRATE_DIRECTION_BITS.iter())
    {
        if probe.faces & bit != 0 {
            let neighbor = world(pos.0 + dir.0, pos.1 + dir.1, pos.2 + dir.2, None);
            if SCULK_REPLACEABLE.contains(&neighbor.name) {
                return true;
            }
        }
    }
    false
}

/// THE MOVEMENT WALK (javap
/// `SculkSpreader$ChargeCursor.getValidMovementPos` pc 0-121): the
/// shuffled offsets the walk's own random re-orders (SEVENTEEN draws
/// every attempt spends — the shuffle BEFORE any candidate reads),
/// each candidate the `setWithOffset(sourcePos, offset)` walk banks,
/// the THREE gates the loop arms — the SculkBehaviour instanceof the
/// block pair keeps, [`movement_unobstructed`] the axis walks answer,
/// [`has_substrate_access`] the vein's face-toward-replaceable pair
/// wears — the FIRST pass the `found.set(cursor)` + break banks (pc
/// 84-102), the never-found walk the `found.equals(sourcePos)` head
/// the null answer keeps (pc 108-121). The candidate the charge-carrying
/// cursor MOVES to (`update` pc 173-210: the onDischarged at the old
/// pos the DEFAULT no-op keeps — javap SculkBehaviour pc 0-4 `return`,
/// the `this.pos = newPos.immutable()` the move wears; the
/// world-gen's `closerThan(..., 15.0)` charge-kill the level lane
/// never arms — the `isWorldGeneration` false the spreader's own
/// createLevelSpreader keeps).
pub fn get_valid_movement_pos<F>(
    from: (i32, i32, i32),
    rng: &mut crate::drops::Lcg,
    world: &mut F,
) -> Option<(i32, i32, i32)>
where
    F: FnMut(i32, i32, i32, Option<u8>) -> SculkMovementProbe,
{
    let mut found = from;
    for (ox, oy, oz) in shuffled_non_corner_offsets(rng) {
        let candidate = (from.0 + ox, from.1 + oy, from.2 + oz);
        let probe = world(candidate.0, candidate.1, candidate.2, None);
        if is_sculk_behaviour_name(probe.name)
            && movement_unobstructed(from, candidate, world)
            && has_substrate_access(&probe, candidate, world)
        {
            found = candidate;
            break;
        }
    }
    if found == from {
        None
    } else {
        Some(found)
    }
}

/// THE SPREAD TYPES (javap `MultifaceSpreader$SpreadType` — the THREE
/// constants the `$values` init banks — SAME_POSITION, SAME_PLANE,
/// WRAP_AROUND — the abstract `getSpreadPos(BlockPos, Direction
/// toFace, Direction fromFace)` the three anonymous impls answer):
/// the veinSpreader's config the DEFAULT_SPREAD_ORDER passes verbatim
/// (javap MultifaceSpreader static init pc 0-25 — the same order the
/// array literal wears).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpreadKind {
    /// `SpreadType$1` — `new SpreadPos(pos, toFace)`: the SAME cell
    /// gains the toFace (the vein cell's own face-add, the air cell
    /// the replaceable target carries).
    SamePosition,
    /// `SpreadType$2` — `new SpreadPos(pos.relative(toFace), fromFace)`:
    /// the neighbor cell in the toFace direction, attaching on the
    /// fromFace side (the sideways crawl the same plane keeps).
    SamePlane,
    /// `SpreadType$3` — `new SpreadPos(pos.relative(toFace)
    /// .relative(fromFace), toFace.getOpposite())`: the diagonal wrap
    /// around the corner, attaching on the toFace's opposite.
    WrapAround,
}

/// `MultifaceSpreader.DEFAULT_SPREAD_ORDER` — the static init's array
/// literal (javap pc 0-25), the veinSpreader's own config order.
pub const SPREAD_KINDS: [SpreadKind; 3] = [
    SpreadKind::SamePosition,
    SpreadKind::SamePlane,
    SpreadKind::WrapAround,
];

/// `Direction.getOpposite()` — the six-direction table's own pairing
/// (down↔up, north↔south, west↔east — every pair differs in bit 0,
/// the XOR the index law wears).
pub fn opposite_face(face: usize) -> usize {
    face ^ 1
}

/// `Direction.getAxis()` — the axis identity the same-axis refusal
/// compares (Y for down/up, Z for north/south, X for west/east — the
/// `face >> 1` fold the table's own order banks).
pub fn face_axis(face: usize) -> usize {
    face >> 1
}

/// `MultifaceBlock.hasFace(state, dir)` — the `1 << ordinal` bit the
/// face property's boolean answers (the SUBSTRATE_DIRECTION_BITS the
/// probe's face word carries).
fn has_face_bit(faces: u8, face: usize) -> bool {
    faces & SUBSTRATE_DIRECTION_BITS[face] != 0
}

/// THE THREE SPREAD POS LAWS (javap the three `SpreadType` impls'
/// `getSpreadPos` — the call `t.getSpreadPos(pos, toFace, fromFace)`
/// the `getSpreadFromFaceTowardDirection` walk serves, pc 97-104):
/// each kind the pos/face pair its own impl banks.
pub fn spread_pos_for(
    kind: SpreadKind,
    pos: (i32, i32, i32),
    to_face: usize,
    from_face: usize,
) -> ((i32, i32, i32), usize) {
    let (dx, dy, dz) = SUBSTRATE_DIRECTIONS[to_face];
    let (fx, fy, fz) = SUBSTRATE_DIRECTIONS[from_face];
    match kind {
        SpreadKind::SamePosition => (pos, to_face),
        SpreadKind::SamePlane => ((pos.0 + dx, pos.1 + dy, pos.2 + dz), from_face),
        SpreadKind::WrapAround => (
            (
                pos.0 + dx + fx,
                pos.1 + dy + fy,
                pos.2 + dz + fz,
            ),
            opposite_face(to_face),
        ),
    }
}

/// `MultifaceSpreader$SpreadConfig.canSpreadFrom` — the DEFAULT law
/// (javap pc 0-26: `isOtherBlockValidAsSource(state) || hasFace(state,
/// dir)`), the vein config's own `isOtherBlockValidAsSource` the
/// `!state.is(SCULK_VEIN)` refusal keeps (javap SculkVeinSpreaderConfig
/// pc 0-15): a NON-vein source state (the sculk block, the air cell the
/// cursor sits in) spreads from EVERY face, the vein source only the
/// faces its own state carries.
pub fn can_spread_from(source: &SculkMovementProbe, from_face: usize) -> bool {
    source.name != "sculk_vein" || has_face_bit(source.faces, from_face)
}

/// THE VEIN CONFIG'S OWN REPLACEMENT LAW (javap
/// `SculkVeinSpreaderConfig.stateCanBeReplaced` — the walk verbatim):
/// - the BEHIND cell (`spreadPos.pos.relative(face)`) the
///   sculk/catalyst/moving_piston trio refuses (pc 14-48 — no face
///   the sculk-family substrate hosts),
/// - the manhattan-2 spread (the wrap-around's diagonal) the
///   BETWEEN-cell sturdiness refuses (pc 49-89 —
///   `sourcePos.relative(face.getOpposite())` the cell the
///   isFaceSturdy(world, cell, face) verdict answers),
/// - the non-water fluid refuses (pc 90-117 — the `!isEmpty() &&
///   !is(WATER)` pair the lava cell wears),
/// - the FIRE tag refuses (pc 118-130),
/// - the replaceable tail (pc 131-157):
///   `state.canBeReplaced() || super.stateCanBeReplaced(...)` — the
///   super arm the air/vein/non-source-water trio answers.
pub fn state_can_be_replaced<F>(
    source_pos: (i32, i32, i32),
    spread_pos: (i32, i32, i32),
    face: usize,
    target: &SculkMovementProbe,
    world: &mut F,
) -> bool
where
    F: FnMut(i32, i32, i32, Option<u8>) -> SculkMovementProbe,
{
    // pc 0-7: the behind read — the SAME cell the attach verdict
    // reads (one read, two laws: the sculk-family NAME refusal here,
    // the canAttachTo full verdict the isValidStateForPlacement walk
    // serves)
    let (bx, by, bz) = SUBSTRATE_DIRECTIONS[face];
    let behind = world(
        spread_pos.0 + bx,
        spread_pos.1 + by,
        spread_pos.2 + bz,
        None,
    );
    // pc 14-48: the sculk/catalyst/moving_piston trio the NAME
    // refusal wears
    if behind.name == "sculk" || behind.name == "sculk_catalyst" || behind.name == "moving_piston"
    {
        return false;
    }
    // pc 49-89: the manhattan-2 diagonal the between-cell sturdiness
    // refuses (`sourcePos.relative(face.getOpposite())` the cell the
    // walk reads, the isFaceSturdy verdict the lane's own full-cube
    // law answers)
    if source_pos.0.abs_diff(spread_pos.0)
        + source_pos.1.abs_diff(spread_pos.1)
        + source_pos.2.abs_diff(spread_pos.2)
        == 2
    {
        let (ox, oy, oz) = SUBSTRATE_DIRECTIONS[opposite_face(face)];
        let between = world(
            source_pos.0 + ox,
            source_pos.1 + oy,
            source_pos.2 + oz,
            None,
        );
        if is_face_sturdy(between.name) {
            return false;
        }
    }
    // pc 90-117: the non-water fluid refuses (the lane's vocabulary
    // carries lava as the only non-empty non-water fluid)
    if target.name == "lava" {
        return false;
    }
    // pc 118-130: the FIRE tag refuses (the lane's own fire family)
    if target.name == "fire" || target.name == "soul_fire" {
        return false;
    }
    // pc 131-157: `canBeReplaced() || super...` — the super arm the
    // air/vein/water trio answers (the `state.is(this.block)` the
    // vein config's own block field keeps, the non-source water the
    // grid's single water word serves)
    can_be_replaced(target.name)
        || target.name == "air"
        || target.name == "sculk_vein"
        || target.name == "water"
}

/// `MultifaceBlock.isValidStateForPlacement` (javap pc 0-52) — the
/// duplicate-face refusal (`state.is(this) && hasFace(state, face)`
/// the already-veined face keeps) and the canAttachTo verdict (pc
/// 28-52: the attach cell `pos.relative(face)` the isFaceFull of the
/// support/collision shape answers — the lane's own full-cube law the
/// [`is_face_sturdy`] verdict keeps).
fn is_valid_state_for_placement<F>(
    target: &SculkMovementProbe,
    spread_pos: (i32, i32, i32),
    face: usize,
    world: &mut F,
) -> bool
where
    F: FnMut(i32, i32, i32, Option<u8>) -> SculkMovementProbe,
{
    if target.name == "sculk_vein" && has_face_bit(target.faces, face) {
        return false;
    }
    let (bx, by, bz) = SUBSTRATE_DIRECTIONS[face];
    let attach = world(
        spread_pos.0 + bx,
        spread_pos.1 + by,
        spread_pos.2 + bz,
        None,
    );
    is_face_sturdy(attach.name)
}

/// `MultifaceSpreader$SpreadConfig.canSpreadInto` (javap pc 0-57):
/// the target state the pos read banks, the replacement law AND the
/// placement validity the AND fold answers.
pub fn can_spread_into<F>(
    source_pos: (i32, i32, i32),
    spread_pos: (i32, i32, i32),
    face: usize,
    world: &mut F,
) -> bool
where
    F: FnMut(i32, i32, i32, Option<u8>) -> SculkMovementProbe,
{
    let target = world(spread_pos.0, spread_pos.1, spread_pos.2, None);
    state_can_be_replaced(source_pos, spread_pos, face, &target, world)
        && is_valid_state_for_placement(&target, spread_pos, face, world)
}

/// THE ONE-CANDIDATE WALK (javap
/// `MultifaceSpreader.getSpreadFromFaceTowardDirection` pc 0-138):
/// the same-axis refusal (pc 0-16 — `toFace.getAxis() ==
/// fromFace.getAxis()` the empty keeps), the isOther/hasFace arm
/// (pc 17-63 — proceed iff `isOtherBlockValidAsSource(state) ||
/// (hasFace(state, fromFace) && !hasFace(state, toFace))` — the vein
/// source the already-carried toFace refuses, the new toFace the
/// loop arms), then the SpreadType loop (pc 64-135 — the FIRST
/// winning kind the `Optional.of` return banks).
pub fn get_spread_from_face_toward_direction<F>(
    source: &SculkMovementProbe,
    pos: (i32, i32, i32),
    from_face: usize,
    to_face: usize,
    world: &mut F,
) -> Option<((i32, i32, i32), usize)>
where
    F: FnMut(i32, i32, i32, Option<u8>) -> SculkMovementProbe,
{
    if face_axis(to_face) == face_axis(from_face) {
        return None;
    }
    if source.name != "sculk_vein" {
        // pc 27 `ifne 64`: the non-vein source the face checks skips
        // (the isOtherBlockValidAsSource TRUE the loop arms)
    } else {
        let has_from = has_face_bit(source.faces, from_face);
        let has_to = has_face_bit(source.faces, to_face);
        if !has_from {
            // pc 42 `ifeq 60`: the from-face the vein never carried —
            // the empty answer (the canSpreadFrom filter already
            // refused, the walk's own arm the same verdict keeps)
            return None;
        }
        if has_to {
            // pc 57 `ifeq 64`: the to-face the vein ALREADY carries —
            // the empty answer (the duplicate the guard refuses)
            return None;
        }
    }
    for kind in SPREAD_KINDS {
        let (spread_pos, spread_face) = spread_pos_for(kind, pos, to_face, from_face);
        if can_spread_into(pos, spread_pos, spread_face, world) {
            return Some((spread_pos, spread_face));
        }
    }
    None
}

/// `SculkBehaviour.attemptSpreadVein`'s OWN WALK (javap the interface
/// default pc 0-27: `Blocks.SCULK_VEIN.getSpreader().spreadAll(state,
/// level, pos, markForUpdates) > 0` — the vein does NOT override, the
/// sculk block neither, the cursor's own facings argument the default
/// NEVER reads): the `spreadAll` walk (javap pc 0-51 — the
/// `Direction.stream()` outer filter `canSpreadFrom`, the inner
/// `spreadFromFaceTowardAllDirections` the per-toFace walk serves,
/// the `.count()` fold the total wears) — each winning (from, to)
/// pair the `spreadToFace` → `placeBlock` → `setBlock(pos, state, 2)`
/// walk the lane's OWN WRITE CALLBACK banks (`Some(face_bit)` — the
/// vein-face placement the offline lane's ledger carries, the
/// immediate visibility the next read observes the vanilla setBlock's
/// own law keeps). The walk spends NO random draws (the fixed
/// v1.0/p1.0 voice the update arm's own playSound speaks, no random
/// the overload carries).
pub fn spread_all<F>(source: &SculkMovementProbe, pos: (i32, i32, i32), world: &mut F) -> u64
where
    F: FnMut(i32, i32, i32, Option<u8>) -> SculkMovementProbe,
{
    let mut count = 0u64;
    for from_face in 0..SUBSTRATE_DIRECTIONS.len() {
        if !can_spread_from(source, from_face) {
            continue;
        }
        for to_face in 0..SUBSTRATE_DIRECTIONS.len() {
            if let Some((spread_pos, spread_face)) =
                get_spread_from_face_toward_direction(source, pos, from_face, to_face, world)
            {
                // THE PLACEMENT (spreadToFace → placeBlock →
                // setBlock(pos, newState, 2) — the lane's own ledger
                // write the face bit carries)
                world(spread_pos.0, spread_pos.1, spread_pos.2, Some(1 << spread_face));
                count += 1;
            }
        }
    }
    count
}

/// THE SPREAD VOICE'S OWN WORD (javap ChargeCursor.update pc 112-126
/// — the `playSound(null, pos, SCULK_BLOCK_SPREAD, BLOCKS, 1.0F,
/// 1.0F)` the BlockPos overload speaks: the FIXED volume/pitch pair
/// the fconst_1 double wears, NO random the overload carries): the
/// record line + the positioned voice the app walk serves (the
/// `sculk_spread` line the bloom/shriek conventions keep, the +0.5
/// center the atCenterOf fold wears).
#[derive(Debug, Clone, PartialEq)]
pub struct SculkSpreadVoice {
    pub pos: (i32, i32, i32),
    pub count: u64,
}

/// THE SPREADER'S OWN DRAIN (javap `SculkSpreader.updateCursors` —
/// the serverTick's `updateCursors(level, pos, random, true)` the
/// block entity's own tick serves): the walk transcribed over the
/// cursors the cell banks —
/// - the EMPTY head (pc 0-12 the return keeps),
/// - the UNREASONABLE skip (pc 73-82 — the chessboard bound the
///   cursor silently drops, NO pop the refusal wears),
/// - the UPDATE walk (ChargeCursor.update: the `shouldUpdate` head
///   the zero-charge cursor refuses — the pop the loop's own check
///   wears; the updateDelay countdown the spread paces — the walk the
///   countdown path EARLY-RETURNS, no spread no movement no draws it
///   spends; the DEFAULT charge law the decayDelay reads — the charge
///   the first tick keeps, the elapsed decay the ZERO answer wears;
///   the decay `max(dd-1, 0)` the countdown folds; the updateDelay the
///   spread delay re-arms),
/// - the VEIN SPREAD ARM (update pc 33-130 — the level lane's
///   onlyGrowers the serverTick's `iconst_1` speaks: the state read
///   at the cursor pos (pc 33-50), the `attemptSpreadVein` call
///   (pc 57-80 — the interface DEFAULT the spreadAll walk serves,
///   the placements the write callback banks with the immediate
///   visibility the next read observes), the `canChangeBlockStateOn`
///   Spread` re-read (pc 83-110 — the sculk block alone the silent
///   state keeps — the re-read the lane's own formality, the charge
///   law the cursor alone reads), the `SCULK_BLOCK_SPREAD` v1.0/p1.0
///   voice (pc 112-126 — the fixed pair the fconst_1 double wears,
///   NO random the overload carries) — the arm the spread>0 verdict
///   alone voices),
/// - the MOVEMENT WALK (update pc 173-271 — the charge>0 tail the
///   full update alone serves: the shuffled non-corner offsets the
///   SEVENTEEN draws re-order, the three gates the candidate answers,
///   the pass the cursor's pos the new cell banks),
/// - the FACINGS REFRESH (update pc 272-291 — the state read at the
///   possibly-moved pos the walk banks, the SculkBehaviour pair the
///   gate keeps, `MultifaceBlock.availableFaces` the multiface set
///   banks — the non-multiface sculk block the EMPTY set answers
///   (Some(0) the lane's own word), the vein the face bits wear; the
///   air cell the gate refuses — the fresh cursor's None the None
///   keeps),
/// - the CHARGE MAP (pc 122-145 — the computeInt fold the total
///   feeds, EVERY alive cursor's charge rides — the MOVED pos the
///   fold's own key keeps, the `cursor.getPos()` read the update's
///   return answers),
/// - the MERGE fold (pc 146-259 — the first-kept cursor the map
///   banks, the ≤1000 pair the mergeWith law folds (the charge adds,
///   the other vanishes, the updateDelay the min keeps), the over-cap
///   pair the BOTH cursors keep (the weaker one the map's facings
///   source swaps to — pc 234-258)),
/// - the SURVIVOR POP walk (pc 262-400 — the log1p decay fold the
///   total wears, the `(decay << 6) + pack(faces)` data word the pop
///   carries; the facings-None cursor the gate skips — the refreshed
///   Some the pop the survivor's own word wears).
///
/// THE WALK THE VEIN SPREAD ROUND WEARS: the spread arm (above — the
/// spreadAll walk the write callback serves, the placements the
/// movement's own substrate arms then answer IN the world the probe
/// reads). The drain's own draws: the movement's SEVENTEEN
/// `nextInt` draws EVERY full update spends (the countdown and the
/// dead-cursor paths the early returns keep — no draws they spend;
/// the SPREAD ARM zero draws the fixed voice speaks), the
/// `level.getRandom()` stream the caller's own order rides.
pub fn update_cursors<F>(
    cell: &mut SculkCatalystCell,
    rng: &mut crate::drops::Lcg,
    mut world: F,
) -> (Vec<SculkChargePop>, Vec<SculkSpreadVoice>)
where
    F: FnMut(i32, i32, i32, Option<u8>) -> SculkMovementProbe,
{
    if cell.cursors.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let mut pops: Vec<SculkChargePop> = Vec::new();
    let mut spreads: Vec<SculkSpreadVoice> = Vec::new();
    let mut kept: Vec<SculkCursor> = Vec::new();
    // the chargeMap the survivor pop rides (the computeInt fold —
    // the totals the insertion order banks, the offline lane's own
    // deterministic convention)
    let mut totals: Vec<((i32, i32, i32), i32)> = Vec::new();
    // the cursorMap the merge + the facings source keep (the kept
    // index the entry stores)
    let mut map_cursor: Vec<((i32, i32, i32), usize)> = Vec::new();
    for cursor in std::mem::take(&mut cell.cursors) {
        // THE UNREASONABLE SKIP (the chessboard bound — the cursor
        // the list silently drops)
        if cursor_is_pos_unreasonable(cursor.pos, cell.pos) {
            continue;
        }
        // THE shouldUpdate HEAD (the zero-charge cursor the update
        // walk refuses — the loop's own charge check the pop wears)
        if cursor.charge <= 0 {
            pops.push(SculkChargePop {
                pos: cursor.pos,
                data: 0,
            });
            continue;
        }
        // THE UPDATE WALK (the countdown the spread paces, the
        // DEFAULT charge law the decay reads)
        let updated = if cursor.update_delay > 0 {
            SculkCursor {
                update_delay: cursor.update_delay - 1,
                ..cursor
            }
        } else {
            // pc 33-50: the state + behaviour read (the spread arm's
            // own probe the walk banks BEFORE the charge walk)
            let state = world(cursor.pos.0, cursor.pos.1, cursor.pos.2, None);
            // pc 52-80: THE VEIN SPREAD ARM (the level lane's
            // onlyGrowers the serverTick speaks — the attemptSpreadVein
            // call the spreadAll walk serves, the placements the write
            // callback banks, the immediate visibility the movement's
            // own reads then observe)
            let spread = spread_all(&state, cursor.pos, &mut world);
            if spread > 0 {
                // pc 83-110: the re-read arm (the
                // `canChangeBlockStateOnSpread` gate the sculk block's
                // own FALSE answer keeps — the re-read the state the
                // walk re-banks, the charge law the cursor alone
                // reads; NO draw the formality spends)
                if can_change_block_state_on_spread(state.name) {
                    let _ = world(cursor.pos.0, cursor.pos.1, cursor.pos.2, None);
                }
                // pc 112-126: the fixed v1.0/p1.0 voice
                spreads.push(SculkSpreadVoice {
                    pos: cursor.pos,
                    count: spread,
                });
            }
            let charge = default_attempt_use_charge(cursor.decay_delay, cursor.charge);
            if charge <= 0 {
                // THE DEAD CURSOR'S OWN POP (pc 104-114 — the data 0
                // word the pop arm wears; the walk the dead cursor
                // EARLY-RETURNS — no movement no draws it spends)
                pops.push(SculkChargePop {
                    pos: cursor.pos,
                    data: 0,
                });
                continue;
            }
            let mut updated = SculkCursor {
                charge,
                decay_delay: default_update_decay_delay(cursor.decay_delay),
                update_delay: SCULK_SPREAD_DELAY,
                ..cursor
            };
            // THE MOVEMENT WALK (update pc 173-210 — the charge>0 tail
            // the full update serves: the shuffle the SEVENTEEN draws
            // spends, the three gates the candidate answers, the pass
            // the cursor's pos the new cell banks; the onDischarged at
            // the old pos the DEFAULT no-op keeps, the world-gen
            // charge-kill the level lane never arms)
            if let Some(new_pos) = get_valid_movement_pos(updated.pos, rng, &mut world) {
                updated.pos = new_pos;
            }
            // THE FACINGS REFRESH (update pc 272-291 — the state read
            // at the POSSIBLY-MOVED pos the walk banks, the
            // SculkBehaviour pair the gate keeps, the availableFaces
            // the multiface set banks: the sculk block the EMPTY set
            // word wears (Some(0)), the vein the face bits, the air
            // cell the fresh cursor's None keeps)
            let here = world(updated.pos.0, updated.pos.1, updated.pos.2, None);
            if is_sculk_behaviour_name(here.name) {
                updated.facings = Some(here.faces);
            }
            updated
        };
        // THE CHARGE MAP (the total the computeInt fold feeds)
        match totals.iter_mut().find(|(p, _)| *p == updated.pos) {
            Some((_, t)) => *t += updated.charge,
            None => totals.push((updated.pos, updated.charge)),
        }
        // THE MERGE WALK (the first-kept cursor the map banks)
        let seen = map_cursor.iter().find(|(p, _)| *p == updated.pos).map(|(_, i)| *i);
        match seen {
            None => {
                map_cursor.push((updated.pos, kept.len()));
                kept.push(updated);
            }
            Some(i) => {
                let existing_charge = kept[i].charge;
                if updated.charge + existing_charge <= SCULK_SPREADER_MAX_CHARGE {
                    // THE MERGEWITH LAW (the charge adds, the other
                    // vanishes, the updateDelay the min keeps)
                    kept[i].charge += updated.charge;
                    kept[i].update_delay = kept[i].update_delay.min(updated.update_delay);
                } else {
                    // THE OVER-CAP PAIR (both the list keeps, the
                    // weaker one the map's facings source swaps to)
                    if updated.charge < existing_charge {
                        if let Some(entry) = map_cursor.iter_mut().find(|(p, _)| *p == updated.pos)
                        {
                            entry.1 = kept.len();
                        }
                    }
                    kept.push(updated);
                }
            }
        }
    }
    // THE SURVIVOR POP WALK (the log1p fold the total wears — the
    // facings-None cursor the gate skips)
    for (pos, total) in &totals {
        if *total <= 0 {
            continue;
        }
        let facing = map_cursor
            .iter()
            .find(|(p, _)| p == pos)
            .and_then(|(_, i)| kept[*i].facings);
        if let Some(faces) = facing {
            let decay = charge_pop_decay(*total);
            pops.push(SculkChargePop {
                pos: *pos,
                data: (decay << 6) + faces as i32,
            });
        }
    }
    cell.cursors = kept;
    (pops, spreads)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// THE MAP THE LAMBDA BANKS — the sampled pairs the javap dump's
    /// own pairing keeps: the 1-band (step/swim/flap), the 2-band
    /// (projectile_land/hit_ground/splash/bounce), the 3-band
    /// (item_interact_finish), the 4-band (entity_action/ELYTRA_GLIDE/
    /// unequip — the glide event's own answer the record lane's f4
    /// speaks), the 5-band (entity_dismount/equip/entity_interact),
    /// the 6/7 (shear, entity_mount, entity_damage), the 8 (drink/eat),
    /// the 9-band (the close/deactivate/detach quartet), the 10-band
    /// (the open/activate/attach/fuse/note sextet), the 11-15 tails
    /// (block_change, block_destroy/fluid_pickup, block_place/
    /// fluid_place, entity_place/lightning_strike/teleport,
    /// entity_die/explode). The unknown event the NO_VIBRATION_FREQUENCY
    /// answers; every known answer rides the 1..=15 band.
    #[test]
    fn the_frequency_map_the_entries_the_lambda_banks() {
        assert_eq!(vibration_frequency("step"), 1);
        assert_eq!(vibration_frequency("swim"), 1);
        assert_eq!(vibration_frequency("flap"), 1);
        assert_eq!(vibration_frequency("projectile_land"), 2);
        assert_eq!(vibration_frequency("hit_ground"), 2);
        assert_eq!(vibration_frequency("splash"), 2);
        assert_eq!(vibration_frequency("bounce"), 2);
        assert_eq!(vibration_frequency("item_interact_finish"), 3);
        assert_eq!(vibration_frequency("projectile_shoot"), 3);
        assert_eq!(vibration_frequency("instrument_play"), 3);
        assert_eq!(vibration_frequency("entity_action"), 4);
        assert_eq!(
            vibration_frequency("elytra_glide"),
            4,
            "the glide event's own answer the record lane's f4 speaks"
        );
        assert_eq!(vibration_frequency("unequip"), 4);
        assert_eq!(vibration_frequency("entity_dismount"), 5);
        assert_eq!(vibration_frequency("equip"), 5);
        assert_eq!(vibration_frequency("entity_interact"), 5);
        assert_eq!(vibration_frequency("shear"), 6);
        assert_eq!(vibration_frequency("entity_mount"), 6);
        assert_eq!(vibration_frequency("entity_damage"), 7);
        assert_eq!(vibration_frequency("drink"), 8);
        assert_eq!(vibration_frequency("eat"), 8);
        assert_eq!(vibration_frequency("container_close"), 9);
        assert_eq!(vibration_frequency("block_close"), 9);
        assert_eq!(vibration_frequency("block_deactivate"), 9);
        assert_eq!(vibration_frequency("block_detach"), 9);
        assert_eq!(vibration_frequency("container_open"), 10);
        assert_eq!(vibration_frequency("block_open"), 10);
        assert_eq!(vibration_frequency("block_activate"), 10);
        assert_eq!(vibration_frequency("block_attach"), 10);
        assert_eq!(vibration_frequency("prime_fuse"), 10);
        assert_eq!(vibration_frequency("note_block_play"), 10);
        assert_eq!(vibration_frequency("block_change"), 11);
        assert_eq!(vibration_frequency("block_destroy"), 12);
        assert_eq!(vibration_frequency("fluid_pickup"), 12);
        assert_eq!(vibration_frequency("block_place"), 13);
        assert_eq!(vibration_frequency("fluid_place"), 13);
        assert_eq!(vibration_frequency("entity_place"), 14);
        assert_eq!(vibration_frequency("lightning_strike"), 14);
        assert_eq!(vibration_frequency("teleport"), 14);
        assert_eq!(vibration_frequency("entity_die"), 15);
        assert_eq!(vibration_frequency("explode"), 15);
        assert_eq!(vibration_frequency("jukebox_play"), NO_VIBRATION_FREQUENCY);
        assert_eq!(
            vibration_frequency("shriek"),
            NO_VIBRATION_FREQUENCY,
            "the shriek the default's zero keeps — the map's own answer"
        );
        assert_eq!(VIBRATION_FREQUENCIES.len(), 41, "the lambda's own count");
        assert!(
            VIBRATION_FREQUENCIES
                .iter()
                .all(|(_, f)| (1..=15).contains(f)),
            "every known answer the 1..=15 band keeps"
        );
    }

    /// THE REDSTONE FOLD (javap pcs 0-21, verbatim): the zero distance
    /// the 15 ceiling answers (floor(0) = 0), the sensor's own radius
    /// the range rides (bipush 8 — getListenerRadius()'s own answer),
    /// the exact-range walk the floor's own edge banks (15.0/8 · 8 =
    /// 15.0 → floor 15 → 15-15 = 0 → the MAX's iconst_1 the FLOOR
    /// answers — the fold NEVER returns 0, the sensor's own active gate
    /// the zero keeps), the beyond-range walk the same floor keeps, the
    /// midpoints the f64 divisor's own answer speaks (15.0/8 · 4 = 7.5
    /// → floor 7 → 8), the sub-1 distance the 15 keeps (1.875 · 0.5 =
    /// 0.9375 → floor 0), the negative distance the floor's
    /// toward-negative-infinity fold the 16 answers (15 − (−1) — the
    /// truncation the plain cast wears would answer 15, the Mth.floor
    /// the difference names).
    #[test]
    fn the_redstone_fold_the_max_the_floor_keeps() {
        assert_eq!(redstone_strength_for_distance(0.0, 8), 15);
        assert_eq!(
            redstone_strength_for_distance(8.0, 8),
            1,
            "the exact range the max's own floor answers"
        );
        assert_eq!(
            redstone_strength_for_distance(100.0, 8),
            1,
            "the beyond-range walk the same floor keeps"
        );
        assert_eq!(
            redstone_strength_for_distance(4.0, 8),
            8,
            "the midpoint the f64 divisor's own answer speaks"
        );
        assert_eq!(
            redstone_strength_for_distance(0.5, 8),
            15,
            "the sub-1 product the floor's zero keeps"
        );
        assert_eq!(
            redstone_strength_for_distance(-0.5, 8),
            16,
            "the negative distance the floor's own fold answers — the \
             truncation the cast wears would answer 15"
        );
        // the alternate radius the fold's own generality keeps (the
        // divisor walk the range arg banks): range 16 · distance 8 →
        // 15.0/16 · 8 = 7.5 → floor 7 → 8
        assert_eq!(redstone_strength_for_distance(8.0, 16), 8);
    }

    /// THE TAG THE JSON BANKS — the count the 56 rows keep, the
    /// sampled rows the elytra_glide acceptance keeps (the glide
    /// event's own hearing rides it), the untagged trio the refusal
    /// keeps (shriek, jukebox_play, item_interact_start — the json
    /// the rows never carried), the resonate family the tail keeps.
    #[test]
    fn the_vibration_tag_the_rows_the_json_keeps() {
        assert_eq!(VIBRATION_TAG.len(), 56, "the json's own count");
        assert!(is_valid_vibration("elytra_glide"));
        assert!(is_valid_vibration("step"));
        assert!(is_valid_vibration("resonate_15"));
        assert!(is_valid_vibration("flap"));
        assert!(!is_valid_vibration("shriek"));
        assert!(!is_valid_vibration("jukebox_play"));
        assert!(!is_valid_vibration("item_interact_start"));
    }

    /// THE OCCLUDES TAG'S OWN ROWS — the jar's occludes json the
    /// `#minecraft:wool` reference answers, the wool tag's own 16 rows
    /// the resolution keeps, the registry fold the world's cells wear.
    #[test]
    fn the_occludes_tag_the_wool_rows_the_json_keeps() {
        assert_eq!(
            OCCLUDES_VIBRATION_SIGNALS.len(),
            16,
            "the wool tag's own count"
        );
        assert!(occludes_vibration_signals("white_wool"), "the first row");
        assert!(occludes_vibration_signals("black_wool"), "the last row");
        assert!(
            occludes_vibration_signals("light_gray_wool"),
            "the sampled row"
        );
        assert!(
            !occludes_vibration_signals("stone"),
            "the untagged the refusal keeps"
        );
        assert!(
            !occludes_vibration_signals("moss_block"),
            "the near-miss the refusal keeps"
        );
        let wool_id = mc_registry::generated::block_ids::BLOCK_RANGES
            .iter()
            .find(|(_, _, n)| *n == "minecraft:white_wool")
            .map(|(s, _, _)| *s)
            .expect("white_wool rides the registry");
        assert!(
            block_occludes(wool_id),
            "the wool cell the registry fold answers"
        );
        let stone_id = mc_registry::generated::block_ids::BLOCK_RANGES
            .iter()
            .find(|(_, _, n)| *n == "minecraft:stone")
            .map(|(s, _, _)| *s)
            .expect("stone rides the registry");
        assert!(
            !block_occludes(stone_id),
            "the stone cell the refusal keeps"
        );
    }

    /// THE OCCLUSION WALK (the six-ray clip the isOccluded
    /// transcription serves) — the clear ray the clean answer keeps,
    /// the wool wall the all-blocked verdict wears, the ANY-clean-ray
    /// rule the escape arm speaks, the START cell the clip's own head
    /// check fires, the listener's own cell the walk enters the hit
    /// keeps.
    #[test]
    fn the_occlusion_walk_the_rays_the_wool_blocks() {
        let from = (0.5, 64.5, 0.5);
        let to = (0.5, 56.5, 0.5);
        assert!(
            !vibration_occluded(from, to, |_, _, _| false),
            "the open air the clean ray keeps"
        );
        // the wool wall the column keeps — every ray the drift the same
        // x/z block rides, the y=60 cell ALL SIX enter
        assert!(
            vibration_occluded(from, to, |x, y, z| x == 0 && y == 60 && z == 0),
            "the wool wall the six rays the blocked verdict wears"
        );
        // the ring of wool AROUND the column, the column cell itself
        // air — the vertical rays the clean answer wears (the
        // ANY-clean-ray rule the boolean speaks)
        assert!(
            !vibration_occluded(from, to, |x, y, z| y == 60 && !(x == 0 && z == 0)),
            "the ring the vertical rays the clean answer keeps"
        );
        // the event's own block the clip's head check fires — the nudge
        // the start the same block keeps, the BLOCK the walk wears
        assert!(
            vibration_occluded(from, to, |x, y, z| x == 0 && y == 64 && z == 0),
            "the event's own block the start check the blocked verdict wears"
        );
        // the listener's own block the walk enters the hit keeps — the
        // sensor in wool the hearing the vanilla walk refuses
        assert!(
            vibration_occluded(from, to, |x, y, z| x == 0 && y == 56 && z == 0),
            "the listener's own block the entered cell the blocked verdict wears"
        );
    }

    /// THE RESONATORS' OWN VOCABULARY — the tag the amethyst alone
    /// carries (the shrieker/catalyst the json never grants), the
    /// registry fold the world's cells wear, the bend notes the
    /// lambda's fill banks, the getPitchFromNote fold the pitch keeps,
    /// the resonance events the isub fold serves, the map's resonate
    /// rows the clinit loop banks (the round-trip the gap retires).
    #[test]
    fn the_resonators_the_amethyst_the_tag_keeps() {
        assert!(
            is_vibration_resonator("amethyst_block"),
            "the one row the tag carries"
        );
        assert!(
            !is_vibration_resonator("sculk_shrieker"),
            "the shrieker the tag never carries"
        );
        assert!(
            !is_vibration_resonator("sculk_catalyst"),
            "the catalyst the tag never carries"
        );
        assert!(
            !is_vibration_resonator("white_wool"),
            "the wool the resonate scan never answers"
        );
        let amethyst_id = mc_registry::generated::block_ids::BLOCK_RANGES
            .iter()
            .find(|(_, _, n)| *n == "minecraft:amethyst_block")
            .map(|(s, _, _)| *s)
            .expect("amethyst_block rides the registry");
        assert!(
            block_resonates(amethyst_id),
            "the amethyst cell the registry fold answers"
        );
        let stone_id = mc_registry::generated::block_ids::BLOCK_RANGES
            .iter()
            .find(|(_, _, n)| *n == "minecraft:stone")
            .map(|(s, _, _)| *s)
            .expect("stone rides the registry");
        assert!(
            !block_resonates(stone_id),
            "the stone cell the refusal keeps"
        );
        // the bend notes the fill banks — the sampled rows the lambda's
        // own iastore pairs pin (the 16 the count keeps)
        assert_eq!(
            resonance_pitch_bend(0),
            2.0f64.powf(-12.0 / 12.0) as f32,
            "the note 0 the half-pitch the fold answers"
        );
        assert_eq!(
            resonance_pitch_bend(1),
            2.0f64.powf(-12.0 / 12.0) as f32,
            "the note 0 the SAME half-pitch keeps"
        );
        assert_eq!(
            resonance_pitch_bend(4),
            2.0f64.powf(-6.0 / 12.0) as f32,
            "the note 6 the tritone-down the glide's own voice wears"
        );
        assert_eq!(
            resonance_pitch_bend(15),
            2.0f64.powf(12.0 / 12.0) as f32,
            "the note 24 the double pitch the top answers"
        );
        assert!(
            (resonance_pitch_bend(4) - 0.7071).abs() < 1e-3,
            "the 2^-0.5 the tritone keeps"
        );
        // the resonance events the isub fold serves
        assert_eq!(
            resonance_event_by_frequency(1),
            "resonate_1",
            "the first the isub keeps"
        );
        assert_eq!(
            resonance_event_by_frequency(4),
            "resonate_4",
            "the glide's own frequency the resonate_4 answers"
        );
        assert_eq!(
            resonance_event_by_frequency(15),
            "resonate_15",
            "the top the list's tail keeps"
        );
        // the map's resonate rows the clinit loop banks — the
        // round-trip the gap retires, the lambda's own 41 the same
        // table keeps
        assert_eq!(
            vibration_frequency("resonate_4"),
            4,
            "the resonate row the clinit's put answers"
        );
        for f in 1..=15 {
            let event = resonance_event_by_frequency(f);
            assert_eq!(
                vibration_frequency(event),
                f,
                "the round-trip the loop's own put keeps: {event}"
            );
        }
        assert_eq!(
            vibration_frequency("elytra_glide"),
            4,
            "the lambda's own row the same lookup keeps"
        );
        assert_eq!(
            vibration_frequency("sculk_sensor_tendrils_clicking"),
            0,
            "the untagged the default's zero keeps"
        );
    }

    /// THE HEARING WALK (the dispatch's own laws the javap serves) —
    /// the GATE: the event (0,60,0) the sensor (0,56,0) hears (the
    /// integer distSqr 16 ≤ 64), the exact fields the reference folds
    /// pin: the distance the Vec3→CENTER fold banks (the +0.5 the
    /// atCenterOf law speaks), the arrival the floor keeps, the
    /// frequency the map's f4 answers, the strength the
    /// integer-sqrt + the fold pair answers (sqrt(16) = 4.0 → 7.5 →
    /// floor 7 → 8); the EDGE: the sensor (8,0,0) from (0,0,0) the
    /// distSqr 64 the EQUAL case passes (the strict `>` the refusal
    /// wears), the strength the fold's own floor answers 1; the
    /// REFUSAL: the sensor (9,0,0) the distSqr 81 > 64 the None keeps;
    /// the TAG: the untagged event the None keeps even in range.
    #[test]
    fn the_sensor_hearing_the_gates_the_folds_keep() {
        // the IN-RANGE hearing, the reference folds the exact answers
        let h = sensor_hearing("elytra_glide", (0.0, 60.0, 0.0), (0, 56, 0))
            .expect("the gate the hearing passes");
        let expect_d = (((0.0f64 - 0.5) * (0.0f64 - 0.5)
            + (60.0f64 - 56.5) * (60.0f64 - 56.5)
            + (0.0f64 - 0.5) * (0.0f64 - 0.5))
            .sqrt()) as f32;
        assert_eq!(h.distance, expect_d, "the Vec3→CENTER fold the +0.5 keeps");
        assert_eq!(h.arrival_ticks, 3, "the floor of the travel distance");
        assert_eq!(h.frequency, 4);
        assert_eq!(
            h.redstone_strength, 8,
            "the integer sqrt 4.0 the fold's 7.5→7→8 walk answers"
        );
        // the EQUAL-edge: distSqr 64 the strict refusal passes
        let h8 = sensor_hearing("step", (0.0, 0.0, 0.0), (8, 0, 0))
            .expect("the 64 distSqr the gate passes");
        assert_eq!(h8.redstone_strength, 1, "the exact range the max's floor keeps");
        assert_eq!(
            h8.arrival_ticks,
            8,
            "the center distance the floor banks — the event (0,0,0) to the \
             (8,0,0) center (8.5,0.5,0.5): sqrt(72.75) ≈ 8.529 → 8"
        );
        // the OUT-OF-RANGE refusal
        assert!(
            sensor_hearing("step", (0.0, 0.0, 0.0), (9, 0, 0)).is_none(),
            "the 81 distSqr the gate refuses"
        );
        // the untagged event the tag gate refuses even in range
        assert!(
            sensor_hearing("shriek", (0.0, 60.0, 0.0), (0, 56, 0)).is_none(),
            "the untagged event the None keeps"
        );
        // the distance helper the integer-coord law keeps
        assert_eq!(distance_between_in_blocks((0, 0, 0), (8, 0, 0)), 8.0);
        assert_eq!(distance_between_in_blocks((0, 60, 0), (0, 56, 0)), 4.0);
    }

    /// THE SHRIEKER LANE THE TAG THE RADII KEEP — the ONE row the
    /// shrieker_can_listen json carries (the clicking alone the
    /// shrieker's ear grants), the r8 the VibrationUser's own bipush
    /// banks, the 90 the shriek walk's scheduleTick keeps, the r32
    /// the register(name, 32)'s own bipush wears (the WIDEST the
    /// registry carries), the shriek the map the row never carries
    /// (the f0 the default's own silence speaks) AND the vibrations
    /// tag the json the row never wears (the no-cascade the dispatch's
    /// isValidVibration gate serves); the HEARING the gates the folds
    /// keep: the tagged in-range the Some(distance) speaks (the
    /// Vec3→CENTER fold the +0.5 law wears), the equal-edge the
    /// distSqr 64 passes, the out-of-range the None keeps, the
    /// untagged event the tag gate refuses even in range.
    #[test]
    fn the_shrieker_tag_the_clicking_the_radii_keep() {
        // THE VOCABULARY (the tag the ONE row the json carries)
        assert_eq!(
            SHRIEKER_CAN_LISTEN,
            &["sculk_sensor_tendrils_clicking"],
            "the shrieker_can_listen json the ONE row the jar wears"
        );
        assert!(shrieker_can_listen("sculk_sensor_tendrils_clicking"));
        assert!(
            !shrieker_can_listen("elytra_glide"),
            "the glide's own event the tag never grants"
        );
        assert!(
            !shrieker_can_listen("shriek"),
            "the shriek's own broadcast the tag the row never carries"
        );
        assert!(
            !shrieker_can_listen("resonate_4"),
            "the resonate rows the tag never carries"
        );
        assert!(
            !shrieker_can_listen("entity_die"),
            "the map's own rows the shrieker's ear never grants"
        );
        // THE RADII/TICKS (the bipush constants the javap walks bank)
        assert_eq!(SHRIEKER_LISTEN_RADIUS, 8, "the user's own bipush 8");
        assert_eq!(SHRIEKER_SHRIEK_TICKS, 90, "the shriek's own bipush 90");
        assert_eq!(SHRIEK_RADIUS, 32, "the register(name, 32)'s bipush 32");
        // THE NO-CASCADE PAIR (the shriek the map AND the vibrations
        // tag alike the row never carry)
        assert_eq!(
            vibration_frequency("shriek"),
            0,
            "the map's defaultReturnValue the shriek row never wears"
        );
        assert!(
            !is_valid_vibration("shriek"),
            "the vibrations tag the json the row never carries — no cascade"
        );
        // THE HEARING (the tagged in-range the Some(distance) speaks —
        // the event center (0.5, 56.5, 0.5), the shrieker (2, 56, 0)
        // center (2.5, 56.5, 0.5): the distance 2.0 the fold keeps)
        let d = shrieker_hearing("sculk_sensor_tendrils_clicking", (0.5, 56.5, 0.5), (2, 56, 0))
            .expect("the clicking the tag grants");
        assert_eq!(d, 2.0, "the Vec3→CENTER fold the exact answer keeps");
        // the EQUAL-edge: distSqr 64 the strict refusal passes
        let d8 = shrieker_hearing("sculk_sensor_tendrils_clicking", (0.5, 0.5, 0.5), (8, 0, 0))
            .expect("the 64 distSqr the gate passes");
        assert_eq!(
            d8, 8.0,
            "the center distance the fold keeps — (0.5,0.5,0.5) to (8.5,0.5,0.5)"
        );
        // the OUT-OF-RANGE refusal: distSqr 81 > 64
        assert!(
            shrieker_hearing("sculk_sensor_tendrils_clicking", (0.5, 0.5, 0.5), (9, 0, 0))
                .is_none(),
            "the 81 distSqr the gate refuses"
        );
        // the UNTAGGED event the tag gate refuses even in range
        assert!(
            shrieker_hearing("elytra_glide", (0.5, 56.5, 0.5), (2, 56, 0)).is_none(),
            "the glide's own event the tag the refusal wears"
        );
    }

    /// THE CATALYST'S OWN HEARING + THE CURSOR WALK (javap the
    /// CatalystListener's gates the death the radius² bound keeps, the
    /// addCursors loop the 1000-cap + the 32 list cap the charges
    /// speak): the entity_die the ONLY event the gate keeps, the
    /// equal-edge distSqr 64 the refusal passes, the 81 the gate
    /// refuses, the OTHER events the head refuses even in range, the
    /// no-occlusion law the plain listener's own body keeps (the
    /// function takes no world — the signature IS the evidence); the
    /// cursor charges the loop's own folds speak — the 5 death one
    /// cursor, the 1500 the 1000+500 pair, the 33000 the 33-walk the
    /// 32-cap trims, the zero-xp death NO cursor the caller's own
    /// gate keeps, the existing head the joint list bounds.
    #[test]
    fn the_catalyst_hearing_the_death_the_cursors_keep() {
        // THE EVENT GATE (the Holder.is(ENTITY_DIE) head)
        assert!(
            catalyst_hearing("entity_die", (0.5, 56.5, 0.5), (2, 56, 0)).is_some(),
            "the death the in-range catalyst answers"
        );
        assert!(
            catalyst_hearing("elytra_glide", (0.5, 56.5, 0.5), (2, 56, 0)).is_none(),
            "the glide's own event the head refuses — the ONE event the gate keeps"
        );
        assert!(
            catalyst_hearing("shriek", (0.5, 56.5, 0.5), (2, 56, 0)).is_none(),
            "the shriek the same refusal wears"
        );
        // THE RANGE GATE (the dispatcher's radius² bound — the
        // equal-edge 64 passes, the 81 refuses)
        assert!(
            catalyst_hearing("entity_die", (0.5, 0.5, 0.5), (8, 0, 0)).is_some(),
            "the 64 distSqr the gate passes"
        );
        assert!(
            catalyst_hearing("entity_die", (0.5, 0.5, 0.5), (9, 0, 0)).is_none(),
            "the 81 distSqr the gate refuses"
        );
        // THE CONSTANT TRIO (the javap bipushes the walk banks)
        assert_eq!(SCULK_CATALYST_LISTEN_RADIUS, 8, "getListenerRadius's bipush 8");
        assert_eq!(CATALYST_PULSE_TICKS, 8, "the bloom scheduleTick's bipush 8");
        assert_eq!(
            SCULK_SPREADER_MAX_CHARGE, 1000,
            "addCursors' sipush 1000 the per-cursor cap keeps"
        );
        assert_eq!(
            SCULK_SPREADER_MAX_CURSORS, 32,
            "addCursor's bipush 32 the list cap keeps"
        );
        // createLevelSpreader's own arg quartet (the level walk — NOT
        // the worldgen twin's 50/1/5/10)
        assert_eq!(SCULK_SPREADER_GROWTH_SPAWN_COST, 10);
        assert_eq!(SCULK_SPREADER_NO_GROWTH_RADIUS, 4);
        assert_eq!(SCULK_SPREADER_CHARGE_DECAY_RATE, 10);
        assert_eq!(SCULK_SPREADER_ADDITIONAL_DECAY_RATE, 5);
        // THE CURSOR CHARGES (the loop's own folds)
        assert_eq!(
            sculk_cursor_charges(5, 0),
            vec![5],
            "the sub-1000 death ONE cursor keeps"
        );
        assert_eq!(
            sculk_cursor_charges(1000, 0),
            vec![1000],
            "the exact cap ONE full cursor wears"
        );
        assert_eq!(
            sculk_cursor_charges(1001, 0),
            vec![1000, 1],
            "the over-cap death the remainder cursor keeps"
        );
        assert_eq!(
            sculk_cursor_charges(1500, 0),
            vec![1000, 500],
            "the 1500 death the 1000+500 pair wears"
        );
        assert!(
            sculk_cursor_charges(0, 0).is_empty(),
            "the zero-xp death NO cursor — the caller's own xp>0 gate keeps"
        );
        assert!(
            sculk_cursor_charges(-3, 0).is_empty(),
            "the negative walk the same refusal keeps"
        );
        assert_eq!(
            sculk_cursor_charges(33000, 0).len(),
            32,
            "the 33-cursor walk the 32-cap's own tail trims"
        );
        assert_eq!(
            sculk_cursor_charges(33000, 0).iter().sum::<i32>(),
            32000,
            "the trimmed bank the 32 x 1000 charge keeps"
        );
        // THE EXISTING HEAD (the joint list the cap bounds — 32 ahead
        // the fresh death nothing banks)
        assert!(
            sculk_cursor_charges(5, SCULK_SPREADER_MAX_CURSORS).is_empty(),
            "the full list the fresh cursor the cap refuses"
        );
        assert_eq!(
            sculk_cursor_charges(5, SCULK_SPREADER_MAX_CURSORS - 1),
            vec![5],
            "the one-free slot the exact fit banks"
        );
    }

    /// THE SPREADER'S OWN DRAIN (javap `updateCursors` + the DEFAULT
    /// `SculkBehaviour$1` walks over the 26.2 jar): the lifecycle the
    /// ticks wear — the fresh cursor the charge walk keeps (the
    /// decayDelay 1 the law reads, the decay countdown folds, the
    /// spread delay re-arms), the countdown the charge walk skips,
    /// the elapsed decay the ZERO answer wears (the cursor the host's
    /// own decay kills — the data-0 pop the dead cursor speaks), the
    /// merge fold the same-pos pair keeps (the charge adds, the other
    /// vanishes, the total the survivor pop rides), the log1p fold
    /// the data word's high bits wear, the unreasonable skip the
    /// chessboard bound serves (the cursor silently drops), the
    /// over-cap pair both the list keeps.
    #[test]
    fn the_drain_the_decay_the_merge_the_pop_word_keeps() {
        // THE CONSTANTS (the javap bipushes + the ldc2_w the walk banks)
        assert_eq!(
            SCULK_SPREADER_MAX_CURSOR_DISTANCE, 1024,
            "isPosUnreasonable's sipush keeps"
        );
        assert_eq!(SCULK_SPREAD_DELAY, 1, "getSculkSpreadDelay's iconst_1 keeps");
        assert_eq!(
            CURSOR_DECAY_DELAY_START, 1,
            "the public ctor's iconst_1 decayDelay keeps"
        );
        assert_eq!(
            CHARGE_POP_DECAY_DIVISOR, 2.3,
            "the log1p fold's ldc2_w keeps"
        );
        // THE FRESH CTOR (the ctor law verbatim)
        let fresh = SculkCursor::fresh((2, 33, 0), 5);
        assert_eq!(
            (fresh.pos, fresh.charge, fresh.update_delay, fresh.decay_delay, fresh.facings),
            ((2, 33, 0), 5, 0, 1, None),
            "the (pos, charge, 0, 1, empty) tuple the ctor law keeps"
        );
        // THE FOLD (the log1p law the total wears)
        assert_eq!(charge_pop_decay(1), 1, "the ln(2)/2.3 floor the +1 keeps");
        assert_eq!(
            charge_pop_decay(10),
            2,
            "the ln(11)/2.3 = 1.04 the floor +1 wears"
        );
        assert_eq!(
            charge_pop_decay(1000),
            4,
            "the ln(1001)/2.3 = 3.0038 the floor +1 wears"
        );
        // THE UNREASONABLE SKIP (the chessboard bound)
        assert!(!cursor_is_pos_unreasonable((1024, 32, 0), (0, 32, 0)));
        assert!(cursor_is_pos_unreasonable((1025, 32, 0), (0, 32, 0)));
        // THE LIFECYCLE — TICK 1: the fresh cursors the charge walk
        // keeps (the merge folds the same-pos pair, the survivor pop
        // the facings source wears)
        let mut cell = SculkCatalystCell {
            pos: (0, 32, 0),
            pulse_off_at: None,
            cursors: vec![
                SculkCursor::fresh((2, 33, 0), 5),
                SculkCursor::fresh((2, 33, 0), 5),
                // the facings-armed twin the survivor pop's data word
                // pins (the map's first cursor the facings source keeps)
                SculkCursor {
                    facings: Some(2),
                    ..SculkCursor::fresh((2, 34, 0), 10)
                },
            ],
        };
        let mut rng = crate::drops::Lcg::new(1);
        let (pops, _) = update_cursors(&mut cell, &mut rng, |_, _, _, _| SculkMovementProbe { name: "air", faces: 0 });
        // THE MERGE (the 5+5 pair the ≤1000 fold folds — the list the
        // merged cursor + the facings twin keeps)
        assert_eq!(cell.cursors.len(), 2, "the merge the pair folds");
        assert_eq!(cell.cursors[0].pos, (2, 33, 0));
        assert_eq!(cell.cursors[0].charge, 10, "the charge the mergeWith adds");
        assert_eq!(
            cell.cursors[0].update_delay, 1,
            "the spread delay the walk re-arms"
        );
        assert_eq!(
            cell.cursors[0].decay_delay, 0,
            "the decay countdown the first tick folds"
        );
        // THE FACINGS TWIN (the second pos the map's own cursor keeps)
        assert_eq!(cell.cursors[1].pos, (2, 34, 0));
        assert_eq!(cell.cursors[1].charge, 10);
        // THE SURVIVOR POPS (the log1p decay the totals wear — the
        // 10-charge pair the decay 2 word speaks, the UP-facing twin
        // the (2<<6)+2 word banks; the merged pos the FIRST cursor's
        // facings the None keeps — NO pop the gate skips)
        assert_eq!(pops.len(), 1, "the facings-None gate the merged pos skips");
        assert_eq!(pops[0].pos, (2, 34, 0));
        assert_eq!(
            pops[0].data,
            (2 << 6) + 2,
            "the (decay << 6) + pack(faces) word the UP bit wears"
        );
        // TICK 2: the countdown the charge walk skips (the decay the
        // countdown never touches); the SURVIVOR POP the facings twin
        // re-fires (the walk the EVERY drain serves — the vein-flight
        // word the living cursor keeps ticking)
        let mut rng = crate::drops::Lcg::new(1);
        let (pops, _) = update_cursors(&mut cell, &mut rng, |_, _, _, _| SculkMovementProbe { name: "air", faces: 0 });
        assert_eq!(pops.len(), 1, "the facings twin the survivor pop re-wears");
        assert_eq!(pops[0].pos, (2, 34, 0));
        assert_eq!(pops[0].data, (2 << 6) + 2);
        assert_eq!(cell.cursors[0].update_delay, 0, "the countdown folds");
        assert_eq!(cell.cursors[0].decay_delay, 0, "the decay the countdown keeps");
        assert_eq!(cell.cursors[0].charge, 10, "the charge the countdown keeps");
        // TICK 3: the elapsed decay the ZERO answer wears — the data-0
        // pop the dead cursor speaks, the list the drain empties
        let mut rng = crate::drops::Lcg::new(1);
        let (pops, _) = update_cursors(&mut cell, &mut rng, |_, _, _, _| SculkMovementProbe { name: "air", faces: 0 });
        assert_eq!(pops.len(), 2, "both cursors the decay kills");
        assert_eq!(pops[0].pos, (2, 33, 0));
        assert_eq!(pops[0].data, 0, "the dead cursor's own data-0 word");
        assert_eq!(pops[1].pos, (2, 34, 0));
        assert_eq!(pops[1].data, 0);
        assert!(cell.cursors.is_empty(), "the drain the list empties");
        // THE EMPTY HEAD (pc 0-12 — the drain the empty list returns)
        let mut rng = crate::drops::Lcg::new(1);
        assert!(
            update_cursors(&mut cell, &mut rng, |_, _, _, _| SculkMovementProbe { name: "air", faces: 0 })
                .0
                .is_empty()
        );
        // THE OVER-CAP PAIR (the 600+600 pair the ≤1000 fold refuses —
        // both the list keeps)
        let mut cell = SculkCatalystCell {
            pos: (0, 32, 0),
            pulse_off_at: None,
            cursors: vec![
                SculkCursor::fresh((2, 33, 0), 600),
                SculkCursor::fresh((2, 33, 0), 600),
            ],
        };
        let mut rng = crate::drops::Lcg::new(1);
        let (pops, _) = update_cursors(&mut cell, &mut rng, |_, _, _, _| SculkMovementProbe { name: "air", faces: 0 });
        assert!(pops.is_empty());
        assert_eq!(cell.cursors.len(), 2, "the over-cap pair the list keeps");
        // THE UNREASONABLE DROP (the chessboard bound the cursor
        // silently drops — NO pop the refusal wears)
        let mut cell = SculkCatalystCell {
            pos: (0, 32, 0),
            pulse_off_at: None,
            cursors: vec![SculkCursor::fresh((2000, 32, 0), 5)],
        };
        let mut rng = crate::drops::Lcg::new(1);
        let (pops, _) = update_cursors(&mut cell, &mut rng, |_, _, _, _| SculkMovementProbe { name: "air", faces: 0 });
        assert!(pops.is_empty(), "the refusal the pop never wears");
        assert!(cell.cursors.is_empty(), "the unreasonable cursor drops");
    }

    /// THE MOVEMENT WALK'S OWN TEST — the table the static init's
    /// filter keeps, the shuffle the Fisher-Yates replay pins, the
    /// gates the synthetic worlds answer, the moved cursor the drain's
    /// own fold banks, the draw count the SEVENTEEN law wears.
    #[test]
    fn the_movement_walk_the_shuffle_the_substrate_keeps() {
        // THE TABLE (the betweenClosedStream order the static init
        // keeps — the all-three-nonzero corners the lambda$static$1
        // drops, the ZERO the equals refusal keeps, the bipush 18 the
        // count answers)
        assert_eq!(NON_CORNER_NEIGHBOURS.len(), 18);
        for corner in [
            (1, 1, 1),
            (-1, -1, -1),
            (1, -1, 1),
            (-1, 1, -1),
            (1, 1, -1),
            (-1, -1, 1),
            (1, -1, -1),
            (-1, 1, 1),
        ] {
            assert!(
                !NON_CORNER_NEIGHBOURS.contains(&corner),
                "the corner the filter drops"
            );
        }
        assert!(
            !NON_CORNER_NEIGHBOURS.contains(&(0, 0, 0)),
            "the ZERO the equals refusal keeps"
        );
        // the EDGE midpoints the two-nonzero law KEEPS (the filter's
        // own reach — only the all-three-nonzero corners reject)
        assert!(NON_CORNER_NEIGHBOURS.contains(&(1, -1, 0)));
        assert!(NON_CORNER_NEIGHBOURS.contains(&(-1, 0, -1)));
        assert!(NON_CORNER_NEIGHBOURS.contains(&(0, 1, 1)));
        // the six face neighbours the table keeps
        for face in [(0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1), (-1, 0, 0), (1, 0, 0)] {
            assert!(NON_CORNER_NEIGHBOURS.contains(&face));
        }

        // THE SHUFFLE — the Fisher-Yates replay (javap Util.shuffle:
        // `for (i = size; i > 1; i--) { j = nextInt(i); swap(i-1, j) }`
        // — the SAME-seed reference fold the identical permutation
        // wears, the bounds the 18-down-to-2 order keeps)
        let mut rng = crate::drops::Lcg::new(42);
        let shuffled = shuffled_non_corner_offsets(&mut rng);
        let mut sorted = shuffled.clone();
        sorted.sort();
        let mut table_sorted = NON_CORNER_NEIGHBOURS.to_vec();
        table_sorted.sort();
        assert_eq!(
            sorted, table_sorted,
            "the swap walk the permutation keeps"
        );
        let mut reference = crate::drops::Lcg::new(42);
        let mut folded = NON_CORNER_NEIGHBOURS.to_vec();
        for i in (2..=18u64).rev() {
            let j = reference.next_int(i) as usize;
            folded.swap(i as usize - 1, j);
        }
        assert_eq!(
            shuffled, folded,
            "the 17-draw bounds walk the identical order wears"
        );
        // THE DRAW COUNT (the seventeen `next()` calls the shuffle
        // alone spends — the bounds the raw draw's own law never
        // changes)
        let mut rng = crate::drops::Lcg::new(7);
        let mut expected = rng.clone();
        shuffled_non_corner_offsets(&mut rng);
        for _ in 0..17 {
            expected.next();
        }
        assert_eq!(rng, expected, "the shuffle the SEVENTEEN draws spends");

        // THE OBSTRUCTION WALK — the face-step the distManhattan head
        // TRUE answers BEFORE any probe (pc 5-10 — the full-cube world
        // the early return keeps)
        let air = SculkMovementProbe { name: "air", faces: 0 };
        let stone = SculkMovementProbe { name: "stone", faces: 0 };
        let mut full_world =
            |_: i32, _: i32, _: i32, _: Option<u8>| SculkMovementProbe { name: "stone", faces: 0 };
        assert!(movement_unobstructed((0, 0, 0), (1, 0, 0), &mut full_world));
        // THE DELTA ARMS (pc 89-180): delta.x == 0 the Y/Z pair
        // answers, delta.y == 0 the X/Z pair, else the X/Y pair — the
        // full-cube neighbours the obstructed answer wears, the air
        // half the true keeps, the NOT-consulted axis the OR's own
        // scope never reads
        assert!(!movement_unobstructed((0, 0, 0), (0, 2, 0), &mut full_world));
        assert!(movement_unobstructed(
            (0, 0, 0),
            (0, 2, 0),
            &mut |x: i32, y: i32, z: i32, _: Option<u8>| {
                if (x, y, z) == (0, 1, 0) {
                    air.clone()
                } else {
                    stone.clone()
                }
            }
        ));
        assert!(movement_unobstructed(
            (0, 0, 0),
            (0, 2, 0),
            &mut |x: i32, y: i32, z: i32, _: Option<u8>| {
                if (x, y, z) == (0, 0, 1) {
                    air.clone()
                } else {
                    stone.clone()
                }
            }
        ));
        assert!(!movement_unobstructed((0, 0, 0), (2, 0, 0), &mut full_world));
        assert!(movement_unobstructed(
            (0, 0, 0),
            (2, 0, 0),
            &mut |x: i32, y: i32, z: i32, _: Option<u8>| {
                if (x, y, z) == (1, 0, 0) {
                    air.clone()
                } else {
                    stone.clone()
                }
            }
        ));
        // the else arm (delta.x != 0 && delta.y != 0) the X/Y pair
        // consults — the Z cell the arm's own scope never reads
        assert!(!movement_unobstructed(
            (0, 0, 0),
            (2, 2, 0),
            &mut |x: i32, y: i32, z: i32, _: Option<u8>| {
                if (x, y, z) == (0, 0, 1) {
                    air.clone()
                } else {
                    stone.clone()
                }
            }
        ));
        assert!(movement_unobstructed(
            (0, 0, 0),
            (2, 2, 0),
            &mut |x: i32, y: i32, z: i32, _: Option<u8>| {
                if (x, y, z) == (1, 0, 0) {
                    air.clone()
                } else {
                    stone.clone()
                }
            }
        ));
        // the NEGATIVE deltas the axis-direction sign wears (the < 0
        // NEGATIVE arm), the zero delta the POSITIVE fallback keeps
        assert!(movement_unobstructed(
            (0, 0, 0),
            (0, -2, 0),
            &mut |x: i32, y: i32, z: i32, _: Option<u8>| {
                if (x, y, z) == (0, -1, 0) {
                    air.clone()
                } else {
                    stone.clone()
                }
            }
        ));
        assert!(movement_unobstructed(
            (0, 0, 0),
            (0, 0, 2),
            &mut |x: i32, y: i32, z: i32, _: Option<u8>| {
                if (x, y, z) == (0, 1, 0) {
                    air.clone()
                } else {
                    stone.clone()
                }
            }
        ));

        // THE SUBSTRATE GATE (javap hasSubstrateAccess — the vein head
        // the non-vein refuses, the face-toward-replaceable pair the
        // scan answers)
        let vein_east = SculkMovementProbe { name: "sculk_vein", faces: 32 };
        let mut substrate = |x: i32, _: i32, _: i32, _: Option<u8>| {
            if x == 1 {
                stone.clone()
            } else {
                air.clone()
            }
        };
        assert!(has_substrate_access(&vein_east, (0, 0, 0), &mut substrate));
        let mut barren = |_: i32, _: i32, _: i32, _: Option<u8>| air.clone();
        assert!(!has_substrate_access(&vein_east, (0, 0, 0), &mut barren));
        // the faceless vein the scan refuses
        let vein_bare = SculkMovementProbe { name: "sculk_vein", faces: 0 };
        assert!(!has_substrate_access(&vein_bare, (0, 0, 0), &mut substrate));
        // the sculk block the VEIN head refuses (the SculkBehaviour
        // instanceof the candidate walk's first gate alone wears)
        let sculk_cell = SculkMovementProbe { name: "sculk", faces: 0 };
        assert!(!has_substrate_access(&sculk_cell, (0, 0, 0), &mut substrate));
        // the down-face vein the dirt-below pair answers (the tag's
        // own rows the substrate reads — the sampled pass + the
        // sampled refusals the tag's scope keeps)
        let vein_down = SculkMovementProbe { name: "sculk_vein", faces: 1 };
        let mut dirt_below = |_: i32, y: i32, _: i32, _: Option<u8>| {
            if y == -1 {
                SculkMovementProbe { name: "dirt", faces: 0 }
            } else {
                air.clone()
            }
        };
        assert!(has_substrate_access(&vein_down, (0, 0, 0), &mut dirt_below));
        let mut glass_below = |_: i32, y: i32, _: i32, _: Option<u8>| {
            if y == -1 {
                SculkMovementProbe { name: "glass", faces: 0 }
            } else {
                air.clone()
            }
        };
        assert!(!has_substrate_access(&vein_down, (0, 0, 0), &mut glass_below));

        // THE SINGLE-CANDIDATE LAW — whatever the shuffle order
        // serves, the ONLY valid candidate the walk returns (the vein
        // the west-face + stone-substrate pair arms, the rest air)
        let mut single = |x: i32, y: i32, z: i32, _: Option<u8>| -> SculkMovementProbe {
            if (x, y, z) == (3, 33, 0) {
                SculkMovementProbe { name: "sculk_vein", faces: 16 }
            } else if (x, y, z) == (2, 33, 0) {
                stone.clone()
            } else {
                air.clone()
            }
        };
        let mut rng = crate::drops::Lcg::new(9);
        assert_eq!(
            get_valid_movement_pos((2, 33, 0), &mut rng, &mut single),
            Some((3, 33, 0)),
            "the lone valid candidate the walk returns"
        );
        // the air world the never-found walk the None answer keeps
        let mut rng = crate::drops::Lcg::new(9);
        assert_eq!(
            get_valid_movement_pos((2, 33, 0), &mut rng, &mut |_: i32,
                _: i32,
                _: i32,
                _: Option<u8>|
                -> SculkMovementProbe { air.clone() }),
            None,
            "the air walk the null answer keeps"
        );
        // the OBSTRUCTED vein the walk refuses (the candidate the
        // delta's arms block — the Y/Z pair full, the vein's own
        // substrate arm never reaches)
        let mut blocked = |x: i32, y: i32, z: i32, _: Option<u8>| -> SculkMovementProbe {
            if (x, y, z) == (2, 35, 0) {
                SculkMovementProbe { name: "sculk_vein", faces: 32 }
            } else if (x, y, z) == (3, 35, 0) {
                stone.clone()
            } else if (x, y, z) == (2, 34, 0) || (x, y, z) == (2, 33, 0) {
                stone.clone()
            } else {
                air.clone()
            }
        };
        let mut rng = crate::drops::Lcg::new(9);
        assert_eq!(
            get_valid_movement_pos((2, 33, 0), &mut rng, &mut blocked),
            None,
            "the obstructed vein the gates refuse"
        );

        // THE DRAIN'S OWN MOVEMENT — the cursor the pass MOVES, the
        // fold the NEW pos banks, the facings the moved cell's vein
        // wears, the survivor pop the moved word speaks
        let mut cell = SculkCatalystCell {
            pos: (0, 32, 0),
            pulse_off_at: None,
            cursors: vec![SculkCursor::fresh((2, 33, 0), 5)],
        };
        let mut rng = crate::drops::Lcg::new(11);
        let (pops, _) = update_cursors(&mut cell, &mut rng, &mut single);
        assert_eq!(cell.cursors[0].pos, (3, 33, 0), "the pass the cursor moves");
        assert_eq!(
            cell.cursors[0].facings,
            Some(16),
            "the moved cell's vein the facings wear"
        );
        assert_eq!(cell.cursors[0].decay_delay, 0);
        assert_eq!(cell.cursors[0].update_delay, 1);
        assert_eq!(pops.len(), 1, "the refreshed Some the survivor pop wears");
        assert_eq!(pops[0].pos, (3, 33, 0), "the fold the MOVED pos banks");
        assert_eq!(
            pops[0].data,
            (1 << 6) + 16,
            "the decay-1 + west-face word the moved cursor wears"
        );
        // THE DRAW ACCOUNTING — the full update the SEVENTEEN draws
        // alone spends
        let mut expected = crate::drops::Lcg::new(11);
        for _ in 0..17 {
            expected.next();
        }
        assert_eq!(rng, expected, "the movement the shuffle's 17 draws spends");
        // THE COUNTDOWN SPENDS NOTHING (the early return the walk
        // keeps — no shuffle the pacing tick draws; the FOLD the
        // alive cursor still rides — the survivor pop the drain
        // RE-WEARS every tick, the 541 law the same word keeps)
        let before = rng.clone();
        let (pops, _) = update_cursors(&mut cell, &mut rng, &mut single);
        assert_eq!(rng, before, "the countdown tick the walk never spends");
        assert_eq!(cell.cursors[0].update_delay, 0, "the countdown folds");
        assert_eq!(
            pops.len(),
            1,
            "the survivor pop the fold re-wears every drain"
        );
        assert_eq!(pops[0].data, (1 << 6) + 16);
        // THE AIR WORLD — the shuffle draws + the cursor stays + the
        // facings the air cell's None keeps (the refresh the gate
        // refuses, the survivor pop the None gate skips)
        let mut cell = SculkCatalystCell {
            pos: (0, 32, 0),
            pulse_off_at: None,
            cursors: vec![SculkCursor::fresh((2, 33, 0), 5)],
        };
        let mut rng = crate::drops::Lcg::new(13);
        let (pops, _) = update_cursors(&mut cell, &mut rng, |_, _, _, _| SculkMovementProbe {
            name: "air",
            faces: 0,
        });
        assert_eq!(cell.cursors[0].pos, (2, 33, 0), "the air walk the cursor keeps");
        assert_eq!(cell.cursors[0].facings, None, "the air cell the refresh refuses");
        assert!(pops.is_empty(), "the facings-None gate the pop skips");
        let mut expected = crate::drops::Lcg::new(13);
        for _ in 0..17 {
            expected.next();
        }
        assert_eq!(rng, expected, "the air walk the 17 draws still spends");
        // THE SCULK-BLOCK CELL — the facings the EMPTY set word wears
        // (Some(0) the availableFaces' non-multiface answer), the
        // survivor pop the pack-0 word speaks
        let mut cell = SculkCatalystCell {
            pos: (0, 32, 0),
            pulse_off_at: None,
            cursors: vec![SculkCursor::fresh((2, 33, 0), 5)],
        };
        let mut rng = crate::drops::Lcg::new(17);
        let (pops, _) = update_cursors(&mut cell, &mut rng, |x: i32, y: i32, z: i32, _: Option<u8>| {
            if (x, y, z) == (2, 33, 0) {
                SculkMovementProbe { name: "sculk", faces: 0 }
            } else {
                air.clone()
            }
        });
        assert_eq!(
            cell.cursors[0].facings,
            Some(0),
            "the sculk cell the EMPTY set word wears"
        );
        assert_eq!(pops.len(), 1);
        assert_eq!(pops[0].data, 1 << 6, "the pack-0 word the empty set wears");
        // THE COUNTDOWN RIDES FIRST (the pacing tick the walk keeps —
        // no shuffle it wears; the fold the Some(0) cursor still
        // rides — the pack-0 pop the drain re-wears)
        let before = rng.clone();
        let (pops, _) = update_cursors(&mut cell, &mut rng, |_, _, _, _| SculkMovementProbe {
            name: "air",
            faces: 0,
        });
        assert_eq!(rng, before, "the countdown tick the walk never spends");
        assert_eq!(pops.len(), 1, "the re-fired pop the fold wears");
        assert_eq!(pops[0].data, 1 << 6);
        assert_eq!(cell.cursors[0].update_delay, 0);
        // THE DEAD-CURSOR PATH SPENDS NOTHING (the pop the early
        // return wears — no shuffle the death tick draws)
        let (pops, _) = update_cursors(&mut cell, &mut rng, |_, _, _, _| SculkMovementProbe {
            name: "air",
            faces: 0,
        });
        assert_eq!(rng, before, "the dead cursor the walk never spends");
        assert_eq!(pops[0].data, 0, "the data-0 pop the decay kills");
        assert!(cell.cursors.is_empty());
    }

    /// THE VEIN SPREAD (javap the `SculkSpreader$ChargeCursor.update`
    /// onlyGrowers arm pc 33-130 + the `SculkBehaviour.attemptSpreadVein`
    /// default + the `MultifaceSpreader` walk verbatim): the THREE
    /// spread kinds' pos laws, the same-axis refusal, the
    /// canSpreadFrom law (the non-vein source every face, the vein
    /// source only its own), the replacement chain (the behind-cell
    /// sculk trio, the manhattan-2 between-cell sturdiness, the
    /// lava/fire refusals, the replaceable tail), the duplicate-face
    /// guard the IMMEDIATE visibility wears, the placement the
    /// ledger's OR-merge banks — and the drain arm's own order (the
    /// spread BEFORE the charge walk, the countdown tick the arm
    /// skips, the fixed v1.0/p1.0 voice the spread>0 verdict alone
    /// wears, the ZERO draws the walk never spends).
    #[test]
    fn the_vein_spread_the_faces_the_wrap_the_ledger_keeps() {
        // THE SPREAD POS LAWS (javap the three SpreadType impls —
        // SAME_POSITION the (pos, toFace) pair, SAME_PLANE the
        // (pos.relative(toFace), fromFace) pair, WRAP_AROUND the
        // diagonal + the toFace's opposite)
        assert_eq!(spread_pos_for(SpreadKind::SamePosition, (5, 6, 7), 2, 0), ((5, 6, 7), 2));
        assert_eq!(
            spread_pos_for(SpreadKind::SamePlane, (5, 6, 7), 2, 0),
            ((5, 6, 6), 0),
            "the north step the from-face pair wears"
        );
        assert_eq!(
            spread_pos_for(SpreadKind::WrapAround, (5, 6, 7), 2, 0),
            ((5, 5, 6), 3),
            "the diagonal wrap the south-face word wears"
        );
        // THE FACE LAWS (the opposite the XOR wears, the axis the >> 1
        // fold banks — the same-axis refusal the Y/Z/X identity keeps)
        for (face, opp) in [(0usize, 1usize), (1, 0), (2, 3), (3, 2), (4, 5), (5, 4)] {
            assert_eq!(opposite_face(face), opp, "the pair the XOR wears");
        }
        assert_eq!(face_axis(0), face_axis(1), "down/up the Y axis shares");
        assert_eq!(face_axis(2), face_axis(3), "north/south the Z axis shares");
        assert_eq!(face_axis(4), face_axis(5), "west/east the X axis shares");
        assert_ne!(face_axis(0), face_axis(2));
        assert_ne!(face_axis(2), face_axis(4));

        // THE CANSPREADFROM LAW (javap SpreadConfig.canSpreadFrom —
        // `isOtherBlockValidAsSource(state) || hasFace(state, dir)`,
        // the vein config's isOther the `!state.is(SCULK_VEIN)`
        // refusal keeps)
        let vein_up = SculkMovementProbe { name: "sculk_vein", faces: 2 };
        assert!(can_spread_from(&vein_up, 1), "the carried face the pass wears");
        assert!(!can_spread_from(&vein_up, 0), "the un-carried face the refusal wears");
        assert!(!can_spread_from(&vein_up, 4));
        let air_cell = SculkMovementProbe { name: "air", faces: 0 };
        for face in 0..6usize {
            assert!(
                can_spread_from(&air_cell, face),
                "the non-vein source EVERY face the isOther answer wears"
            );
        }
        let sculk_cell = SculkMovementProbe { name: "sculk", faces: 0 };
        assert!(can_spread_from(&sculk_cell, 3));

        // THE RE-READ LAW (javap SculkBlock.canChangeBlockStateOnSpread
        // pc 0-1 `iconst_0` — the sculk block alone the FALSE answer
        // keeps, the DEFAULT the interface `iconst_1` wears)
        assert!(!can_change_block_state_on_spread("sculk"));
        assert!(can_change_block_state_on_spread("sculk_vein"));
        assert!(can_change_block_state_on_spread("air"));

        // THE REPLACEABLE TAIL (javap pc 131-157 — the canBeReplaced
        // families the lane's vocabulary keeps)
        assert!(can_be_replaced("air"));
        assert!(can_be_replaced("water"));
        assert!(can_be_replaced("short_grass"));
        assert!(can_be_replaced("vine"));
        assert!(!can_be_replaced("stone"));
        assert!(!can_be_replaced("dirt"));
        assert!(!can_be_replaced("sculk"));

        // THE SPREAD_ALL WALK — the fresh cursor's AIR source the
        // stone floor+wall world answers: THREE placements the walk
        // banks (the down face on the cell itself, the north face the
        // duplicate guard already wears, the wrap-around the stone's
        // east face the between-cell pass arms)
        let mut ledger: Vec<(i32, i32, i32, u8)> = Vec::new();
        let mut world = |x: i32, y: i32, z: i32, place: Option<u8>| -> SculkMovementProbe {
            if let Some(bit) = place {
                match ledger
                    .iter_mut()
                    .find(|(px, py, pz, _)| *px == x && *py == y && *pz == z)
                {
                    Some((_, _, _, f)) => *f |= bit,
                    None => ledger.push((x, y, z, bit)),
                }
            }
            if let Some((_, _, _, f)) = ledger
                .iter()
                .find(|(px, py, pz, _)| *px == x && *py == y && *pz == z)
            {
                return SculkMovementProbe {
                    name: "sculk_vein",
                    faces: *f,
                };
            }
            match (x, y, z) {
                (0, -1, 0) | (0, 0, -1) => SculkMovementProbe { name: "stone", faces: 0 },
                _ => SculkMovementProbe { name: "air", faces: 0 },
            }
        };
        let count = spread_all(&air_cell, (0, 0, 0), &mut world);
        assert_eq!(count, 8, "the eight winning pairs the count banks");
        assert_eq!(
            ledger,
            vec![
                (0, 0, 0, 5),
                (0, -1, 1, 4),
                (-1, -1, 0, 32),
                (1, -1, 0, 16),
                (0, 1, -1, 1),
                (-1, 0, -1, 32),
                (1, 0, -1, 16),
            ],
            "the self-faces the OR-merge banks + the six wraps the stones' sides wear"
        );
        // THE VEIN SOURCE (the faces the source carries the filter
        // alone spreads — the up-face vein the two placements the
        // down/north pairs bank, the un-carried from-faces the filter
        // refuses)
        ledger.clear();
        let mut world = |x: i32, y: i32, z: i32, place: Option<u8>| -> SculkMovementProbe {
            if let Some(bit) = place {
                match ledger
                    .iter_mut()
                    .find(|(px, py, pz, _)| *px == x && *py == y && *pz == z)
                {
                    Some((_, _, _, f)) => *f |= bit,
                    None => ledger.push((x, y, z, bit)),
                }
            }
            if let Some((_, _, _, f)) = ledger
                .iter()
                .find(|(px, py, pz, _)| *px == x && *py == y && *pz == z)
            {
                return SculkMovementProbe {
                    name: "sculk_vein",
                    faces: *f,
                };
            }
            match (x, y, z) {
                (0, 0, 0) => SculkMovementProbe { name: "sculk_vein", faces: 2 },
                (0, -1, 0) | (0, 0, -1) => SculkMovementProbe { name: "stone", faces: 0 },
                _ => SculkMovementProbe { name: "air", faces: 0 },
            }
        };
        let count = spread_all(&vein_up, (0, 0, 0), &mut world);
        assert_eq!(
            count, 1,
            "the up-face source the ONE sideways placement wears (the down target the same-axis refusal keeps)"
        );
        assert_eq!(
            ledger,
            vec![(0, 0, 0, 4)],
            "the north face-add the ledger banks"
        );
        ledger.clear();

        // THE REPLACEMENT CHAIN's refusals (javap
        // SculkVeinSpreaderConfig.stateCanBeReplaced): the lava/fire
        // targets the fluid+tag refusals keep, the behind-cell sculk
        // the trio refusal wears
        let mut world = |x: i32, y: i32, z: i32, _: Option<u8>| -> SculkMovementProbe {
            match (x, y, z) {
                (0, 0, -1) => SculkMovementProbe { name: "lava", faces: 0 },
                (1, 0, -1) => SculkMovementProbe { name: "fire", faces: 0 },
                (0, -1, -1) => SculkMovementProbe { name: "sculk", faces: 0 },
                (0, 0, 0) => SculkMovementProbe { name: "stone", faces: 0 },
                _ => SculkMovementProbe { name: "air", faces: 0 },
            }
        };
        // the lava target the SAME_POSITION north face refuses
        assert!(!can_spread_into((0, 0, 0), (0, 0, -1), 2, &mut world));
        // the fire target the tag refusal wears (the east face)
        assert!(!can_spread_into((0, 0, 0), (1, 0, -1), 2, &mut world));
        // the behind-cell sculk the trio refusal keeps (the north
        // face's substrate the sculk cell answers)
        assert!(!can_spread_into((0, 0, 0), (0, -1, -1), 2, &mut world));
        // the stone target the replaceable tail REFUSES (the full
        // cube the spread never replaces)
        assert!(!can_spread_into((0, 0, 0), (0, 0, 0), 2, &mut world));

        // THE DRAIN ARM (the spread BEFORE the charge walk — the voice
        // the spread>0 verdict alone wears, the countdown tick the arm
        // skips, the ZERO draws the walk never spends, the ledger the
        // probe's own answers the movement then reads)
        let mut cell = SculkCatalystCell {
            pos: (0, 32, 0),
            pulse_off_at: None,
            cursors: vec![SculkCursor::fresh((2, 33, 0), 5)],
        };
        let mut ledger: Vec<(i32, i32, i32, u8)> = Vec::new();
        let mut world = |x: i32, y: i32, z: i32, place: Option<u8>| -> SculkMovementProbe {
            if let Some(bit) = place {
                match ledger
                    .iter_mut()
                    .find(|(px, py, pz, _)| *px == x && *py == y && *pz == z)
                {
                    Some((_, _, _, f)) => *f |= bit,
                    None => ledger.push((x, y, z, bit)),
                }
            }
            if let Some((_, _, _, f)) = ledger
                .iter()
                .find(|(px, py, pz, _)| *px == x && *py == y && *pz == z)
            {
                return SculkMovementProbe {
                    name: "sculk_vein",
                    faces: *f,
                };
            }
            if (x, y, z) == (2, 32, 0) {
                SculkMovementProbe { name: "stone", faces: 0 }
            } else {
                SculkMovementProbe { name: "air", faces: 0 }
            }
        };
        let mut rng = crate::drops::Lcg::new(23);
        let (pops, spreads) = update_cursors(&mut cell, &mut rng, &mut world);
        // THE VOICE (the five placements the count wears — the down
        // face the cell itself banks, the FOUR WRAPS the stone's sides
        // the crawl wears — the vein the spread's own product the
        // movement's candidate reads then answer)
        assert_eq!(spreads.len(), 1, "the spread>0 verdict the voice wears");
        assert_eq!(spreads[0].pos, (2, 33, 0));
        assert_eq!(spreads[0].count, 5);
        // THE ZERO-DRAW LAW (the shuffle's SEVENTEEN draws the only
        // spend — the spread the walk never draws)
        let mut expected = crate::drops::Lcg::new(23);
        for _ in 0..17 {
            expected.next();
        }
        assert_eq!(rng, expected, "the spread the zero-draw law keeps");
        // THE CHARGE WALK UNCHANGED (the decay law the spread never
        // touches — the charge the first tick keeps, the delays the
        // same re-arm banks)
        assert_eq!(cell.cursors[0].charge, 5);
        assert_eq!(cell.cursors[0].decay_delay, 0);
        assert_eq!(cell.cursors[0].update_delay, 1);
        assert_eq!(pops.len(), 1, "the survivor pop the fold still wears");
        // THE COUNTDOWN TICK (the arm the early return skips — no
        // spread, no voice, no draw)
        let before = rng.clone();
        let (_, spreads) = update_cursors(&mut cell, &mut rng, &mut world);
        assert!(spreads.is_empty(), "the countdown tick the spread skips");
        assert_eq!(rng, before, "the countdown the zero-draw keeps");
        // THE LEDGER (the five placements the crawl banks — the down
        // self-face + the four side wraps, the walk's own order)
        assert_eq!(
            ledger,
            vec![
                (2, 32, -1, 8),
                (2, 32, 1, 4),
                (1, 32, 0, 32),
                (3, 32, 0, 16),
                (2, 33, 0, 1),
            ],
            "the four side wraps the stone's sides + the down self-face the ledger banks"
        );
        // THE AIR WORLD (the silence the no-substrate world wears —
        // the spreadAll the zero count answers, no voice the drain
        // speaks)
        let mut cell = SculkCatalystCell {
            pos: (0, 32, 0),
            pulse_off_at: None,
            cursors: vec![SculkCursor::fresh((2, 33, 0), 5)],
        };
        let mut ledger: Vec<(i32, i32, i32, u8)> = Vec::new();
        let mut world = |x: i32, y: i32, z: i32, place: Option<u8>| -> SculkMovementProbe {
            if let Some(bit) = place {
                match ledger
                    .iter_mut()
                    .find(|(px, py, pz, _)| *px == x && *py == y && *pz == z)
                {
                    Some((_, _, _, f)) => *f |= bit,
                    None => ledger.push((x, y, z, bit)),
                }
            }
            if let Some((_, _, _, f)) = ledger
                .iter()
                .find(|(px, py, pz, _)| *px == x && *py == y && *pz == z)
            {
                return SculkMovementProbe {
                    name: "sculk_vein",
                    faces: *f,
                };
            }
            SculkMovementProbe { name: "air", faces: 0 }
        };
        let mut rng = crate::drops::Lcg::new(29);
        let (_, spreads) = update_cursors(&mut cell, &mut rng, &mut world);
        assert!(spreads.is_empty(), "the air world the spread silences");
        assert!(ledger.is_empty());
        let mut expected = crate::drops::Lcg::new(29);
        for _ in 0..17 {
            expected.next();
        }
        assert_eq!(rng, expected, "the air walk the 17 draws still spend");
    }
}

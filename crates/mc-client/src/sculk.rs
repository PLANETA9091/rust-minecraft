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
/// The OCCLUSION walk the later round serves (the wool path the
/// open-air hearing this round never reads); the PHASE gate the call
/// site's free-at window keeps (the data-busy + the active/cooldown
/// pair the same fold subsumes).
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
}

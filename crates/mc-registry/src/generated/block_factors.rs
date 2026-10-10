// GENERATED from the client.jar's Blocks clinit (javap the
// BlockBehaviour$Properties speedFactor/jumpFactor registration rows
// the 26.2 jar keeps) by scripts/extract_block_factors.py — the
// Entity.getBlockSpeedFactor / Entity.getBlockJumpFactor walks' data
// rows: the 26.2 clinit speaks exactly TWO rows (soul_sand
// speedFactor 0.4f pcs 10667-10670; honey_block speedFactor 0.4f +
// jumpFactor 0.5f pcs 26877-26885 — the makeStuckInBlock family the
// older versions carried RETIRED: the SoulSandBlock the entityInside
// override never carries, the HoneyBlock's own the slide-down
// achievements the HalfTransparentBlock super speaks), the unmatched
// block the 1.0 Properties default rides.

pub const SPEED_FACTOR_COUNT: usize = 2;

pub const SPEED_FACTOR_NAMES: [&str; SPEED_FACTOR_COUNT] = [
    "honey_block",
    "soul_sand",
];

pub const SPEED_FACTOR_VALUES: [f32; SPEED_FACTOR_COUNT] = [
    0.4, 0.4,
];

pub const JUMP_FACTOR_COUNT: usize = 1;

pub const JUMP_FACTOR_NAMES: [&str; JUMP_FACTOR_COUNT] = [
    "honey_block",
];

pub const JUMP_FACTOR_VALUES: [f32; JUMP_FACTOR_COUNT] = [
    0.5,
];

/// The block's own speed factor (the 1.0 default the unmatched
/// name rides — the Properties default the clinit's silent rows
/// keep); the getBlockSpeedFactor walk's layered read consumes (the
/// feet cell first, the WATER pass-through arm, the below-cell
/// fallback the default-1.0 feet wears).
pub fn speed_factor(name: &str) -> f32 {
    SPEED_FACTOR_NAMES
        .binary_search(&name)
        .ok()
        .map(|i| SPEED_FACTOR_VALUES[i])
        .unwrap_or(1.0)
}

/// The block's own jump factor (the same 1.0 default — the
/// getBlockJumpFactor walk's layered read consumes: the feet cell,
/// the below-cell fallback, NO water arm the jump walk carries).
pub fn jump_factor(name: &str) -> f32 {
    JUMP_FACTOR_NAMES
        .binary_search(&name)
        .ok()
        .map(|i| JUMP_FACTOR_VALUES[i])
        .unwrap_or(1.0)
}

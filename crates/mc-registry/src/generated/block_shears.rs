// GENERATED from the client.jar's tags (#minecraft:leaves,
// #minecraft:wool, the glow_lichen+vine minors, the cobweb direct
// rule javap ShearsItem.createToolProperties keeps) by
// scripts/extract_mineable_tags.py — the SHEARS TOOL component's
// rule rows the dig lane's getMiningSpeed walk reads: (speed,
// correct_for_drops) — the cobweb's TRUE the minesAndDrops arm
// keeps, the overrideSpeed rules the no-verdict False wear, the
// unmatched block the defaultMiningSpeed 1.0 default rides.
pub const SHEARS_BLOCK_COUNT: usize = 30;

pub const SHEARS_NAMES: [&str; SHEARS_BLOCK_COUNT] = [
    "acacia_leaves",
    "azalea_leaves",
    "birch_leaves",
    "black_wool",
    "blue_wool",
    "brown_wool",
    "cherry_leaves",
    "cobweb",
    "cyan_wool",
    "dark_oak_leaves",
    "flowering_azalea_leaves",
    "glow_lichen",
    "gray_wool",
    "green_wool",
    "jungle_leaves",
    "light_blue_wool",
    "light_gray_wool",
    "lime_wool",
    "magenta_wool",
    "mangrove_leaves",
    "oak_leaves",
    "orange_wool",
    "pale_oak_leaves",
    "pink_wool",
    "purple_wool",
    "red_wool",
    "spruce_leaves",
    "vine",
    "white_wool",
    "yellow_wool",
];

/// (speed, correct_for_drops) per name — the first matching rule
/// the component walk keeps.
pub const SHEARS_SPEED: [f32; SHEARS_BLOCK_COUNT] = [
    15.0, 15.0, 15.0, 5.0, 5.0, 5.0, 15.0, 15.0, 5.0, 15.0, 15.0, 2.0, 5.0, 5.0, 15.0, 5.0, 5.0, 5.0, 5.0, 15.0, 15.0, 5.0, 15.0, 5.0, 5.0, 5.0, 15.0, 2.0, 5.0, 5.0
];

pub const SHEARS_CORRECT: [bool; SHEARS_BLOCK_COUNT] = [
    false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false
];

/// The shears' own rule row (the None the unmatched block the
/// defaultMiningSpeed 1.0 default rides).
pub fn shears_rule(name: &str) -> Option<(f32, bool)> {
    let pos = SHEARS_NAMES.binary_search(&name).ok()?;
    Some((SHEARS_SPEED[pos], SHEARS_CORRECT[pos]))
}

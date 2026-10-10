// GENERATED from the client.jar's tags (#minecraft:sword_instantly_mines,
// #minecraft:sword_efficient, the cobweb direct rule javap
// ToolMaterial.applySwordProperties keeps) by
// scripts/extract_mineable_tags.py — the SWORD TOOL component's
// rule rows the dig lane's getMiningSpeed walk reads: (speed,
// correct_for_drops) — the cobweb's TRUE the minesAndDrops arm
// keeps, the overrideSpeed rules the no-verdict False wear, the
// unmatched block the defaultMiningSpeed 1.0 default rides.
pub const SWORD_BLOCK_COUNT: usize = 25;

pub const SWORD_NAMES: [&str; SWORD_BLOCK_COUNT] = [
    "acacia_leaves",
    "azalea_leaves",
    "bamboo",
    "bamboo_sapling",
    "big_dripleaf",
    "big_dripleaf_stem",
    "birch_leaves",
    "carved_pumpkin",
    "cherry_leaves",
    "chorus_flower",
    "chorus_plant",
    "cobweb",
    "cocoa",
    "dark_oak_leaves",
    "flowering_azalea_leaves",
    "glow_lichen",
    "jack_o_lantern",
    "jungle_leaves",
    "mangrove_leaves",
    "melon",
    "oak_leaves",
    "pale_oak_leaves",
    "pumpkin",
    "spruce_leaves",
    "vine",
];

/// (speed, correct_for_drops) per name — the first matching rule
/// the component walk keeps.
pub const SWORD_SPEED: [f32; SWORD_BLOCK_COUNT] = [
    1.5, 1.5, 3.4028235e+38, 3.4028235e+38, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 15.0, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5
];

pub const SWORD_CORRECT: [bool; SWORD_BLOCK_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false
];

/// The sword's own rule row (the None the unmatched block the
/// defaultMiningSpeed 1.0 default rides).
pub fn sword_rule(name: &str) -> Option<(f32, bool)> {
    let pos = SWORD_NAMES.binary_search(&name).ok()?;
    Some((SWORD_SPEED[pos], SWORD_CORRECT[pos]))
}

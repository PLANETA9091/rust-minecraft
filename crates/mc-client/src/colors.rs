//! Face shading + block palette colors (defaults + real-texture averages).

use crate::world::*;

/// Face order: 0:+X 1:-X 2:+Y 3:-Y 4:+Z 5:-Z (classic MC light multipliers).
pub const FACE_SHADE: [f32; 6] = [0.6, 0.6, 1.0, 0.5, 0.8, 0.8];

pub const FACE_NAMES: [&str; 6] = ["east", "west", "top", "bottom", "south", "north"];

/// Default palette (used when game textures are not available locally).
pub fn default_color(kind: u16, face: usize) -> [f32; 3] {
    let top = [0.40, 0.66, 0.26];
    let grass_side = [0.52, 0.42, 0.30];
    let dirt = [0.55, 0.42, 0.30];
    let stone = [0.58, 0.58, 0.58];
    let log_side = [0.43, 0.34, 0.20];
    let log_top = [0.62, 0.51, 0.31];
    let leaves = [0.24, 0.50, 0.21];
    let water = [0.28, 0.48, 0.74];
    let sand = [0.84, 0.79, 0.61];
    let granite = [0.58, 0.44, 0.38];
    let diorite = [0.85, 0.85, 0.86];
    let andesite = [0.62, 0.63, 0.63];
    let deepslate = [0.32, 0.32, 0.35];
    let cobbled_deepslate = [0.35, 0.35, 0.37];
    let gravel = [0.48, 0.45, 0.44];
    let bedrock = [0.35, 0.35, 0.35];
    let coal_ore = [0.52, 0.52, 0.52];
    let copper_ore = [0.60, 0.50, 0.45];
    let iron_ore = [0.65, 0.55, 0.48];
    let gold_ore = [0.63, 0.57, 0.42];
    let redstone_ore = [0.55, 0.40, 0.38];
    let lapis_ore = [0.45, 0.50, 0.62];
    let diamond_ore = [0.58, 0.66, 0.66];
    let emerald_ore = [0.52, 0.62, 0.48];
    let netherrack = [0.42, 0.20, 0.20];
    match kind {
        GRASS => match face {
            2 => top,
            3 => dirt,
            _ => grass_side,
        },
        DIRT => dirt,
        STONE => stone,
        LOG => match face {
            2 | 3 => log_top,
            _ => log_side,
        },
        LEAVES => leaves,
        WATER => water,
        SAND => sand,
        GRANITE => granite,
        DIORITE => diorite,
        ANDESITE => andesite,
        DEEPSLATE => deepslate,
        COBBLED_DEEPSLATE => cobbled_deepslate,
        GRAVEL => gravel,
        BEDROCK => bedrock,
        COAL_ORE => coal_ore,
        COPPER_ORE => copper_ore,
        IRON_ORE => iron_ore,
        GOLD_ORE => gold_ore,
        REDSTONE_ORE => redstone_ore,
        LAPIS_ORE => lapis_ore,
        DIAMOND_ORE => diamond_ore,
        EMERALD_ORE => emerald_ore,
        NETHERRACK => netherrack,
        _ => [1.0, 0.0, 1.0], // magenta = missing block (debug-friendly)
    }
}

/// Official texture file stem per block kind + face (without directory and
/// extension). The LUT joins it into `minecraft/textures/block/<stem>.png`.
pub fn texture_stem(kind: u16, face: usize) -> Option<&'static str> {
    match kind {
        GRASS => match face {
            2 => Some("grass_block_top"),
            3 => Some("dirt"),
            _ => Some("grass_block_side"),
        },
        DIRT => Some("dirt"),
        STONE => Some("stone"),
        LOG => match face {
            2 | 3 => Some("oak_log_top"),
            _ => Some("oak_log"),
        },
        LEAVES => Some("oak_leaves"),
        WATER => Some("water_still"),
        SAND => Some("sand"),
        GRANITE => Some("granite"),
        DIORITE => Some("diorite"),
        ANDESITE => Some("andesite"),
        DEEPSLATE => match face {
            2 | 3 => Some("deepslate_top"),
            _ => Some("deepslate"),
        },
        COBBLED_DEEPSLATE => Some("cobbled_deepslate"),
        GRAVEL => Some("gravel"),
        BEDROCK => Some("bedrock"),
        COAL_ORE => Some("coal_ore"),
        COPPER_ORE => Some("copper_ore"),
        IRON_ORE => Some("iron_ore"),
        GOLD_ORE => Some("gold_ore"),
        REDSTONE_ORE => Some("redstone_ore"),
        LAPIS_ORE => Some("lapis_ore"),
        DIAMOND_ORE => Some("diamond_ore"),
        EMERALD_ORE => Some("emerald_ore"),
        NETHERRACK => Some("netherrack"),
        _ => None,
    }
}

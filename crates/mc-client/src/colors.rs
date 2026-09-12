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
        _ => None,
    }
}

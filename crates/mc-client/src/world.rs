//! Deterministic voxel world for the reference renderer.
//! Block ids match the compact runtime palette (mirrored by mc2rs-gen's
//! atlas generator); `mc-registry` supplies the full version registry —
//! the original survival-classic subset generates the deterministic scene,
//! generated members extend it for network terrain.

pub const CHUNK_X: usize = 16;
pub const CHUNK_Y: usize = 48;
pub const CHUNK_Z: usize = 16;
pub const WATER_LEVEL: i32 = 10;

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

/// Block -> texture-group mapping used by the real-colors LUT.
pub const NAME_OF: [&str; 24] = [
    "air", "stone", "dirt", "grass_block", "oak_log", "oak_leaves", "water", "sand",
    "granite", "diorite", "andesite", "deepslate", "cobbled_deepslate", "gravel",
    "bedrock", "coal_ore", "copper_ore", "iron_ore", "gold_ore", "redstone_ore",
    "lapis_ore", "diamond_ore", "emerald_ore", "netherrack",
];

/// A plain data grid — derives follow the data (the join result carries a
/// World through the `Debug/Clone/PartialEq/Eq` LoginResult surface).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    pub chunks_per_side: usize,
    pub seed: u64,
    /// chunk-major: chunk = cx + cz * chunks_per_side
    pub chunks: Vec<Vec<u16>>,
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

    fn set(&mut self, wx: i32, wy: i32, wz: i32, id: u16) {
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

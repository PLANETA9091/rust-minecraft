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

/// Block -> texture-group mapping used by the real-colors LUT.
pub const NAME_OF: [&str; 26] = [
    "air", "stone", "dirt", "grass_block", "oak_log", "oak_leaves", "water", "sand",
    "granite", "diorite", "andesite", "deepslate", "cobbled_deepslate", "gravel",
    "bedrock", "coal_ore", "copper_ore", "iron_ore", "gold_ore", "redstone_ore",
    "lapis_ore", "diamond_ore", "emerald_ore", "netherrack", "pale_oak_leaves",
    "leaf_litter",
];

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
/// dry-foliage color — see [`crate::tint`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BiomeLayer {
    /// per-chunk per-voxel registry biome ids; [`BIOME_UNKNOWN`] where unsent
    pub ids: Vec<Vec<u32>>,
    /// per registry id (the arrival order IS the network id): the tint
    /// quad [grass override, foliage override, water color, dry-foliage
    /// color]
    pub tints: Vec<[Option<u32>; 4]>,
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
        BiomeLayer { ids: vec![vec![0; CHUNK_X * CHUNK_Y * CHUNK_Z]], tints }
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
}

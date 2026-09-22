//! THE OFFLINE LIGHT ENGINE'S OWN GRID — the sky column + the block
//! flood the procedural world never carried (the deferred lane the
//! torch rounds named: the offline torch rendered UNLIT, the caves
//! the player dug stayed full-bright). The engine computes BOTH
//! channels the vanilla light engine's own shape speaks (26.2 jar:
//! `LightEngine`'s SkyLightEngine column + propagate walk,
//! BlockLightEngine's emissive flood) into the [`LightGrid`] the
//! mesher's smooth-lighting path ALREADY consumes — the wire world's
//! own consumer (`World::light`), so the offline terrain lights
//! through the exact lane the server's chunks light.
//!
//! THE LAWS (javap the 26.2 jar):
//! - THE SKY COLUMN: level 15 walks straight down through opacity-0
//!   cells and STOPS at the first attenuator — the heightmap's direct
//!   light, the column never re-starts below the stop.
//! - THE DOWNWARD FREE RIDE: a 15 arriving anywhere keeps 15 going
//!   down through opacity-0 cells (the propagation rule the cave
//!   mouth's own light shaft speaks) — the SkyLightEngine's down-edge
//!   law, applied in the flood exactly as the column walk applies it.
//! - EVERY OTHER EDGE: `level - max(1, opacity(target))` — the
//!   1-per-step floor the attenuators ride, the opaque wall (15) the
//!   light never enters.
//! - THE BLOCK FLOOD: the emissive blocks seed their own luminance
//!   (the torch 14, the lava 15 — the Blocks clinit's lightEmission
//!   rows) and flood the SAME `max(1, opacity)` edges, NO downward
//!   free ride (the block light the downward rule never favors).
//!
//! THE OPACITY LAW (the jar's `lightBlock` rows over the client's
//! palette): the air, the torch, the cross plants (the flower/tuft/
//! double-plant/wheat families), and the ground cover speak 0; the
//! water and the leaf family (oak + pale oak) speak 1; every
//! full-cube member speaks 15. THE EMISSION LAW: the torch 14, the
//! lava 15, the rest of the palette dark.
//!
//! THE WORLD'S OWN TRUTH: the grid fills EVERY in-world cell (no
//! [`crate::world::LIGHT_UNKNOWN`] survives the walk) — the mob lane's
//! `eye_light` reads the grid verbatim (the burn law and the spider's
//! dark gate answer the REAL shadows), the mesher's known-only corner
//! averages never meet an unknown cell inside the world, and the
//! world's edge/ceiling keeps the honest `None` the grid's own
//! conventions speak.

use crate::world::{LightGrid, World, CHUNK_X, CHUNK_Y, CHUNK_Z};

/// The six flood directions: +X -X +Y -Y +Z -Z (the mesher's own
/// NEIGHBOR order — the pair the two walks share).
const NEIGHBORS: [(i32, i32, i32); 6] =
    [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];

/// The cell's own sky opacity — `BlockState.lightBlock` over the
/// client's palette (the rows the module doc names). The families the
/// mesher's `is_transparent` verdict keeps answer 0 here too, with
/// ONE deliberate split: the water and the leaves ATTENUATE (the
/// vanilla 1-opacity family) where the mesher's cull lane renders
/// them — the light the fluids and the foliage shade, the faces the
/// same cells still draw.
fn sky_opacity(id: u16) -> u8 {
    use crate::world::*;
    if id == AIR
        || is_torch(id)
        || is_flower(id)
        || is_tuft(id)
        || is_double_plant(id)
        || is_wheat(id)
        || id == LEAF_LITTER
    {
        0
    } else if id == WATER || id == LEAVES || id == PALE_LEAVES {
        1
    } else {
        15
    }
}

/// The cell's own emission — `BlockState.lightEmission` over the
/// client's palette: the torch 14, the lava 15, the rest dark.
fn emission(id: u16) -> u8 {
    use crate::world::*;
    if is_torch(id) {
        14
    } else if id == LAVA {
        15
    } else {
        0
    }
}

/// The flood's own frontier — a plain Vec the head index drains (the
/// improvement re-push the relaxation law needs; each cell rides the
/// queue once per climb, the monotone convergence the FIFO order
/// keeps).
struct Flood {
    queue: Vec<(i32, i32, i32)>,
    head: usize,
}

impl Flood {
    fn new() -> Self {
        Flood {
            queue: Vec::new(),
            head: 0,
        }
    }

    fn push(&mut self, x: i32, y: i32, z: i32) {
        self.queue.push((x, y, z));
    }

    fn pop(&mut self) -> Option<(i32, i32, i32)> {
        let item = self.queue.get(self.head).copied();
        if item.is_some() {
            self.head += 1;
        }
        item
    }
}

/// Compute BOTH light channels over the world's own voxels — the grid
/// [`crate::world::World::relight`] stores. The flat scratch arrays
/// index `(y * n + z) * n + x` (the column walk's own layout), the
/// pack re-walks the chunk layout the [`LightGrid`] banks.
pub fn compute(world: &World) -> LightGrid {
    let side = world.chunks_per_side;
    let n = (side * CHUNK_X) as i32;
    let h = CHUNK_Y as i32;
    let cells = (n * n * h) as usize;
    let idx = |x: i32, y: i32, z: i32| ((y * n + z) * n + x) as usize;

    let mut sky = vec![0u8; cells];
    let mut block = vec![0u8; cells];

    // THE SKY COLUMN PASS (the direct light the heightmap speaks):
    // from the world's top down, 15 through opacity-0 cells, the
    // column STOPS at the first attenuator — the cells below the stop
    // the flood alone answers.
    let mut flood = Flood::new();
    for z in 0..n {
        for x in 0..n {
            for y in (0..h).rev() {
                if sky_opacity(world.get(x, y, z)) != 0 {
                    break;
                }
                sky[idx(x, y, z)] = 15;
                flood.push(x, y, z);
            }
        }
    }

    // THE SKY FLOOD: the seeded 15-cells relax their neighbors the
    // vanilla edges speak — the down-edge's free ride (15 through
    // opacity-0 keeps 15), every other edge the max(1, opacity) step,
    // the opaque wall the flood never enters.
    while let Some((x, y, z)) = flood.pop() {
        let level = sky[idx(x, y, z)];
        for &(dx, dy, dz) in &NEIGHBORS {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if nx < 0 || nz < 0 || nx >= n || nz >= n || ny < 0 || ny >= h {
                continue;
            }
            let op = sky_opacity(world.get(nx, ny, nz));
            if op >= 15 {
                continue;
            }
            let next = if dy == -1 && level == 15 && op == 0 {
                15
            } else {
                level.saturating_sub(op.max(1))
            };
            let ni = idx(nx, ny, nz);
            if next > sky[ni] {
                sky[ni] = next;
                flood.push(nx, ny, nz);
            }
        }
    }

    // THE BLOCK FLOOD: the emissive cells seed their own luminance
    // (the torch 14, the lava 15), the flood the SAME max(1, opacity)
    // edges walks — NO downward free ride (the block light's own
    // physics).
    let mut flood = Flood::new();
    for y in 0..h {
        for z in 0..n {
            for x in 0..n {
                let e = emission(world.get(x, y, z));
                if e > 0 {
                    block[idx(x, y, z)] = e;
                    flood.push(x, y, z);
                }
            }
        }
    }
    while let Some((x, y, z)) = flood.pop() {
        let level = block[idx(x, y, z)];
        for &(dx, dy, dz) in &NEIGHBORS {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if nx < 0 || nz < 0 || nx >= n || nz >= n || ny < 0 || ny >= h {
                continue;
            }
            let op = sky_opacity(world.get(nx, ny, nz));
            if op >= 15 {
                continue;
            }
            let next = level.saturating_sub(op.max(1));
            let ni = idx(nx, ny, nz);
            if next > block[ni] {
                block[ni] = next;
                flood.push(nx, ny, nz);
            }
        }
    }

    // THE PACK (the chunk layout the grid banks): every in-world cell
    // carries its computed level — no LIGHT_UNKNOWN survives.
    let mut grid = LightGrid {
        sky: vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]; side * side],
        block: vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]; side * side],
    };
    for cz in 0..side {
        for cx in 0..side {
            let ci = cx + cz * side;
            for y in 0..CHUNK_Y {
                for lz in 0..CHUNK_Z {
                    for lx in 0..CHUNK_X {
                        let wx = (cx * CHUNK_X + lx) as i32;
                        let wz = (cz * CHUNK_Z + lz) as i32;
                        let cell = lx + lz * CHUNK_X + y * CHUNK_X * CHUNK_Z;
                        grid.sky[ci][cell] = sky[idx(wx, y as i32, wz)];
                        grid.block[ci][cell] = block[idx(wx, y as i32, wz)];
                    }
                }
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::*;

    /// The EMPTY stage: every voxel the procedural generate would
    /// speak is carved back to AIR — the light laws' own clean room
    /// (the tests stamp their scenes onto it).
    fn empty_world() -> World {
        let mut w = World::new(1, 0);
        for y in 0..CHUNK_Y as i32 {
            for z in 0..16 {
                for x in 0..16 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        w
    }

    /// The light AT a world cell through the grid's own read (the
    /// packed layout the mesher's consumer rides).
    fn light_at(w: &World, x: i32, y: i32, z: i32) -> (Option<u8>, Option<u8>) {
        w.light.as_ref().expect("relit").cell(w.chunks_per_side, x, y, z)
    }

    #[test]
    fn the_open_sky_column_wears_the_fifteen() {
        let mut w = empty_world();
        w.relight();
        // THE OPEN LAW: every air cell in the empty stage reads 15 —
        // the direct column the whole sky speaks, top to floor.
        for y in 0..CHUNK_Y as i32 {
            assert_eq!(light_at(&w, 5, y, 5).0, Some(15), "open column at y={y}");
        }
        // THE BLOCK CHANNEL'S DARKNESS: no emissive block stands —
        // the flood never speaks.
        assert_eq!(light_at(&w, 5, 5, 5).1, Some(0));
    }

    #[test]
    fn the_solid_roof_stops_the_column_and_shades_the_cave() {
        let mut w = empty_world();
        // THE CAVE: a stone floor at y=4, a stone roof at y=8 spanning
        // x 2..14 — the mouth at x 0..1 the open sky keeps.
        for z in 0..16 {
            for x in 0..16 {
                w.set(x, 4, z, STONE);
            }
        }
        for z in 2..14 {
            for x in 2..14 {
                w.set(x, 8, z, STONE);
            }
        }
        w.relight();
        // THE COLUMN'S STOP: above the roof 15, the roof cell itself
        // the opaque 0, below the roof the flood's answer.
        assert_eq!(light_at(&w, 6, 9, 6).0, Some(15), "above the roof");
        assert_eq!(light_at(&w, 6, 8, 6).0, Some(0), "the roof cell blocks");
        // THE MOUTH'S SHAFT: the open columns at x 0..1 keep 15 down
        // to the floor (the direct column unbroken).
        assert_eq!(light_at(&w, 0, 5, 8).0, Some(15), "the mouth's column");
        assert_eq!(light_at(&w, 1, 7, 8).0, Some(15), "the mouth's shaft top");
        // THE OVERHANG'S EDGE: one step under the roof the sideways
        // step from the mouth's 15 the flood pays — 14 at the edge,
        // EVERY depth (the horizontal gradient the cave's mouth wears,
        // the vertical one the roof's shadow refuses).
        assert_eq!(light_at(&w, 2, 7, 8).0, Some(14), "the edge's step");
        assert_eq!(light_at(&w, 2, 5, 8).0, Some(14), "the edge's depth");
        // THE CAVE'S DEPTH: the roof's center column sits 6 cells from
        // the mouth's shaft — the horizontal steps the flood pays.
        assert_eq!(light_at(&w, 7, 7, 8).0, Some(9), "the cave's dim interior");
        // THE FLOOR'S OWN DARK: the stone under the roof's center —
        // the flood the opaque cells never enter.
        assert_eq!(light_at(&w, 8, 4, 8).0, Some(0), "the cave floor blocks");
    }

    #[test]
    fn the_water_column_attenuates_by_one_per_block() {
        let mut w = empty_world();
        // THE POND: water fills y 5..=8 over a 3x3 bed (x 3..5,
        // z 3..5), the stone floor beneath — the center column's own
        // depth the walls' shade speaks.
        for z in 3..=5 {
            for x in 3..=5 {
                for y in 5..=8 {
                    w.set(x, y, z, WATER);
                }
                w.set(x, 4, z, STONE);
            }
        }
        w.relight();
        // THE SURFACE WATER CELL: the 15 air above, the opacity-1
        // water the first step shades to 14 — the vanilla's own
        // underwater dim.
        assert_eq!(light_at(&w, 4, 8, 4).0, Some(14), "the surface water");
        // THE POND'S WALL CELL: the open sky beside the pond the
        // sideways step shades — 14 at EVERY depth the outside column
        // keeps.
        assert_eq!(light_at(&w, 3, 6, 4).0, Some(14), "the wall cell's side step");
        // THE CENTER'S DEPTH: the walls' 14 one more step shades — 13
        // the pond's middle keeps at every depth below the surface
        // (the sideways path through the water the two steps costs).
        assert_eq!(light_at(&w, 4, 7, 4).0, Some(13), "one under the surface");
        assert_eq!(light_at(&w, 4, 5, 4).0, Some(13), "the pond's bottom water");
        // THE AIR BESIDE THE POND: the open sky's 15 the column keeps.
        assert_eq!(light_at(&w, 2, 6, 4).0, Some(15));
    }

    #[test]
    fn the_leaves_attenuate_the_canopy_law() {
        let mut w = empty_world();
        // THE CANOPY: a 3x3 leaf slab floats in the open sky — the
        // center column's own shadow the slab's spread speaks.
        for z in 6..=8 {
            for x in 6..=8 {
                w.set(x, 10, z, LEAVES);
            }
        }
        w.relight();
        // THE LEAF CELL: the direct column stops at the first
        // attenuator — the leaf itself the flood's 14 answers.
        assert_eq!(light_at(&w, 7, 10, 7).0, Some(14), "the leaf's own cell");
        // THE AIR ABOVE: the column's 15 unbroken to the canopy.
        assert_eq!(light_at(&w, 7, 11, 7).0, Some(15));
        // THE AIR BELOW THE SLAB'S CENTER: the canopy's shade the
        // flood spreads — the slab's leaves 14, one more step down 13
        // (the open sky's sideways reach the slab's width refuses).
        assert_eq!(light_at(&w, 7, 9, 7).0, Some(13), "under the canopy");
        // THE SLAB'S OWN EDGE BELOW: the outside columns' 15 the one
        // sideways step shades — 14 the canopy's rim keeps.
        assert_eq!(light_at(&w, 6, 9, 7).0, Some(14), "the rim's side step");
    }

    #[test]
    fn the_torch_floods_the_fourteen_law() {
        let mut w = empty_world();
        w.set(8, 6, 8, TORCH);
        w.relight();
        // THE SOURCE: the torch cell's own block light 14 — the
        // lightEmission row the flame rides.
        assert_eq!(light_at(&w, 8, 6, 8).1, Some(14));
        // THE FALLOFF: one per Manhattan step through the open air.
        assert_eq!(light_at(&w, 9, 6, 8).1, Some(13), "one east");
        assert_eq!(light_at(&w, 8, 5, 8).1, Some(13), "one below");
        assert_eq!(light_at(&w, 10, 6, 8).1, Some(12), "two east");
        // THE WALL'S OWN SHADOW: a stone wall one cell wide (the
        // x=12 column at EVERY height) two cells east stops the flood
        // — the cells behind it the detour around the wall's edge
        // answers.
        for y in 0..CHUNK_Y as i32 {
            w.set(12, y, 8, STONE);
        }
        w.relight();
        assert_eq!(light_at(&w, 11, 6, 8).1, Some(11), "three steps before the wall");
        assert_eq!(light_at(&w, 13, 6, 8).1, Some(7),
            "the wall's far side the around-detour's reach speaks");
        // THE SKY CHANNEL NEVER MOVES: the torch emits no sky light.
        assert_eq!(light_at(&w, 8, 6, 8).0, Some(15));
    }

    #[test]
    fn the_lava_speaks_the_fifteen_emission() {
        let mut w = empty_world();
        w.set(3, 6, 3, LAVA);
        w.relight();
        assert_eq!(light_at(&w, 3, 6, 3).1, Some(15), "the lava's own cell");
        assert_eq!(light_at(&w, 4, 6, 3).1, Some(14), "one east");
        assert_eq!(light_at(&w, 3, 7, 3).1, Some(14), "one above");
    }

    #[test]
    fn the_plants_speak_the_zero_opacity() {
        let mut w = empty_world();
        // THE GROUND COVER: leaf_litter under the open sky — the
        // column's 15 rides straight through (the plant never shades).
        w.set(5, 5, 5, LEAF_LITTER);
        w.set(7, 5, 5, DANDELION);
        w.set(9, 5, 5, TORCH);
        w.relight();
        assert_eq!(light_at(&w, 5, 5, 5).0, Some(15), "the litter's column");
        assert_eq!(light_at(&w, 5, 4, 5).0, Some(15), "under the litter");
        assert_eq!(light_at(&w, 7, 5, 5).0, Some(15), "the flower's column");
        assert_eq!(light_at(&w, 7, 4, 5).0, Some(15), "under the flower");
        // THE TORCH'S COLUMN: the stick never shades the sky either.
        assert_eq!(light_at(&w, 9, 4, 5).0, Some(15), "under the torch");
    }

    #[test]
    fn the_grid_packs_the_chunk_layout_the_consumer_reads() {
        let mut w = World::new(2, 0);
        for y in 0..CHUNK_Y as i32 {
            for z in 0..32 {
                for x in 0..32 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        // THE FAR CHUNK: a torch at (20, 6, 20) — chunk (1, 1) — the
        // grid's per-chunk arrays the readback answers.
        w.set(20, 6, 20, TORCH);
        w.relight();
        let grid = w.light.as_ref().expect("relit");
        assert_eq!(grid.sky.len(), 4, "2x2 chunks pack four layers");
        // THE READBACK through the grid's own cell() walk (the world
        // coordinates the consumer speaks).
        let (s, b) = grid.cell(2, 20, 6, 20);
        assert_eq!(s, Some(15));
        assert_eq!(b, Some(14));
        let (_, b2) = grid.cell(2, 21, 6, 20);
        assert_eq!(b2, Some(13), "the flood crosses into the neighbor cell");
        // THE OUTSIDE: above the world's height the honest None pair.
        assert_eq!(grid.cell(2, 20, CHUNK_Y as i32 + 1, 20), (None, None));
        assert_eq!(grid.cell(2, 20, 6, -1), (None, None));
    }

    #[test]
    fn the_relit_world_relights_the_edit_law() {
        let mut w = empty_world();
        // THE PLACE: the torch lands, the relight speaks its flood.
        w.set(8, 6, 8, TORCH);
        w.relight();
        assert_eq!(light_at(&w, 10, 6, 8).1, Some(12));
        // THE BREAK: the torch leaves, the relight's silence the same
        // grid re-walk answers — the stale glow the second compute
        // never keeps.
        w.set(8, 6, 8, AIR);
        w.relight();
        assert_eq!(light_at(&w, 10, 6, 8).1, Some(0), "the torch's flood gone");
        assert_eq!(light_at(&w, 8, 6, 8).1, Some(0), "the source's cell dark");
    }

    #[test]
    fn the_compute_never_names_an_unknown_cell() {
        let mut w = World::new(1, 42); // the REAL terrain — trees, water, grass
        w.relight();
        let grid = w.light.as_ref().expect("relit");
        // THE FULL LAW: every in-world cell carries a computed level —
        // the LIGHT_UNKNOWN byte the packed layers never keep (the
        // mob lane's fallback the full grid never meets).
        for ci in 0..grid.sky.len() {
            for &v in &grid.sky[ci] {
                assert_ne!(v, LIGHT_UNKNOWN, "unknown sky at chunk {ci}");
            }
            for &v in &grid.block[ci] {
                assert_ne!(v, LIGHT_UNKNOWN, "unknown block at chunk {ci}");
            }
        }
    }
}

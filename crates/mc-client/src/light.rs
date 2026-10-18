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

use crate::world::{LightGrid, World, CHUNK_X, CHUNK_Y, CHUNK_Z, LIGHT_UNKNOWN};

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
        // THE DOOR FAMILY'S OWN ROW (the autopush-655 round — the
        // 3/16 panel the light the flood passes: the DoorBlock's own
        // isTransparent-true registration the vanilla lightBlock 0
        // speaks, the closed door the light's own cell the flood
        // never blocks — the neighbors' cells the same reach keeps)
        || is_door(id)
        // THE GLASS FAMILY'S OWN ROW (the dev-57 round — the 17
        // kinds the noOcclusion() registration the lightBlock 0 the
        // vanilla law speaks: the plain glass pc 3782 the clinit
        // chain banks, the stained ladder the collection lambda's own
        // chain the same row wears — the window the sky the flood
        // passes, the interior the daylight the glass never shades,
        // the vanilla skylight's own law the windows answer)
        || is_glass(id)
        // THE PANE FAMILY'S OWN ROW (the dev-58 round — the 18 kinds
        // the noOcclusion() registration the lightBlock 0 the same
        // law speaks: the glass_pane pc 12459 + the iron_bars pc
        // 12322 the clinit chains bank, the stained pane ladder the
        // collection lambda's own chain wears — the thin strip the
        // light the flood passes, the CrossCollisionBlock's
        // propagatesSkylightDown walk the vanilla skylight's own
        // verdict keeps)
        || is_glass_pane(id)
        || id == IRON_BARS
        // THE COPPER BARS FAMILY'S OWN ROW (the dev-59 round — javap
        // lambda$static$163 the Properties chain the noOcclusion()
        // row pc 21 the eight oxidization rungs speak: the thin strip
        // the light the flood passes, the propagatesSkylightDown law
        // the vanilla skylight's own verdict keeps)
        || is_copper_bars(id)
    {
        0
    } else if id == WATER || id == LEAVES || id == PALE_LEAVES {
        1
    } else {
        15
    }
}

/// The cell's own emission — `BlockState.lightEmission` over the
/// client's palette: the torch 14, the lava 15, the fire kinds the
/// clinit's own lambdas answer (the `lambda$static$67` bipush 15 the
/// fire registration's lightLevel call banks, the `lambda$static$68`
/// bipush 10 the soul fire's row keeps — the constants the two
/// registrations wear, the palette's default-state fold the client's
/// kind-only world speaks), the rest dark.
fn emission(id: u16) -> u8 {
    use crate::world::*;
    if is_torch(id) {
        14
    } else if id == LAVA {
        15
    } else if id == FIRE {
        15
    } else if id == SOUL_FIRE {
        10
    } else if id == REDSTONE_TORCH {
        // THE LIT REDSTONE TORCH'S OWN ROW (the autopush-666 round —
        // javap the blocks clinit's REDSTONE_TORCH registration over
        // the 26.2 jar: the `bipush 7; litBlockEmission(I)` lightLevel
        // call the putstatic REDSTONE_TORCH walk pc 10087-10092 banks
        // — the lit-state read the emission's own verdict keeps, the
        // 7 the lit row's own answer, the OFF kind the dark 0 the
        // litBlockEmission's own else arm keeps)
        7
    } else if id == CANDLE_LIT {
        // THE LIT CANDLE'S OWN ROW (the autopush-522 round — javap
        // CandleBlock.lambda$static$0 over the 26.2 jar: the
        // `lit ? 3 * candles : 0` fold the lightLevel registration
        // banks, the 1-candle default state the kind-only palette
        // speaks — the LIT row's 3 the imul's own answer, the UNLIT
        // kind the dark 0 the else arm keeps)
        3
    } else if id == CANDLE_CAKE_LIT {
        // THE LIT CANDLE CAKE'S OWN ROW (the autopush-523 round —
        // javap the blocks clinit's candle_cake registration over
        // the 26.2 jar: the `iconst_3; litBlockEmission(I)`
        // lightLevel call the ofLegacyCopy(CAKE) props chain banks —
        // the lit-state read the emission's own verdict keeps, the
        // 3 the dimmest of the lit family the candle's own row
        // shares, the UNLIT kind the dark 0 the else arm keeps)
        3
    } else if id == CAMPFIRE_LIT {
        // THE LIT CAMPFIRE'S OWN ROW (the dev-6 round — javap the
        // blocks clinit's campfire registration over the 26.2 jar:
        // the `bipush 15; litBlockEmission(I)` lightLevel call the
        // lambda$static$310 props chain banks — the lit-state read
        // the emission's own verdict keeps, the 15 the lit row's
        // own answer, the UNLIT kind the dark 0 the else arm keeps)
        15
    } else if id == SOUL_CAMPFIRE_LIT {
        // THE LIT SOUL CAMPFIRE'S OWN ROW (the same round — the
        // lambda$static$311 props chain the `bipush 10` lightLevel
        // call banks: the soul flame the dimmer row the vanilla's
        // own two registrations speak)
        10
    } else if id == FURNACE_LIT {
        // THE LIT FURNACE'S OWN ROW (the autopush-601 round — javap
        // the blocks clinit's furnace registration over the 26.2 jar:
        // the `bipush 13; litBlockEmission(I)` lightLevel call the
        // putstatic FURNACE walk pc 7188-7190 banks — the lit-state
        // read the emission's own verdict keeps, the 13 the lit row's
        // own answer, the UNLIT kind the dark 0 the else arm keeps)
        13
    } else if id == crate::world::BLAST_FURNACE_LIT {
        // THE LIT BLAST FURNACE'S OWN ROW (the dev-26 round — the same
        // `bipush 13; litBlockEmission(I)` lightLevel call the
        // putstatic BLAST_FURNACE walk pc 24287-24292 banks, the
        // furnace's exact registration the same constant keeps, the
        // UNLIT kind the dark 0 the else arm keeps)
        13
    } else if id == crate::world::SMOKER_LIT {
        // THE LIT SMOKER'S OWN ROW (the dev-26 round — the same
        // `bipush 13; litBlockEmission(I)` lightLevel call the
        // putstatic SMOKER walk pc 24241-24246 banks, the UNLIT kind
        // the dark 0 the else arm keeps)
        13
    } else if id == crate::world::REDSTONE_LAMP_LIT {
        // THE LIT LAMP'S OWN ROW (the autopush-672 round — the
        // blocks clinit's redstone_lamp registration the lightLevel
        // `bipush 15; litBlockEmission(I)` fold the lit rows' own
        // shared law banks, the LIT=true arm the amber row the
        // full-brightness 15 the verdict keeps, the UNLIT kind the
        // dark 0 the else arm keeps)
        15
    } else if id == crate::world::NETHER_PORTAL {
        // THE PORTAL'S OWN ROW (the autopush-605 round — javap the
        // blocks clinit's nether_portal registration over the 26.2
        // jar: the lightLevel call pc 11109-11114 the bootstrap #219
        // lambda walks, lambda$static$143 the `bipush 11; ireturn`
        // body the constant the EVERY-state read answers — the
        // unbreakable gate's own purple glow the light grid paints,
        // the axis property the emission never reads)
        11
    } else if id == crate::world::END_PORTAL {
        // THE END PORTAL'S OWN ROW (the dev-29 round — the blocks
        // clinit's end_portal registration the lightLevel lambda
        // bipush 15 the properties chain banks: the starry field the
        // full brightness the light grid paints, the unbreakable gate
        // the same every-state read keeps)
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
    // the flood alone answers. THE DIMENSION'S OWN GATE (javap
    // `DimensionType.hasSkylight` — the nether's false the pass never
    // runs: the sky layer the zero keeps, the grid the block flood
    // alone answers; the overworld's true the default build speaks).
    let mut flood = Flood::new();
    if world.has_skylight {
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

// ---------------------------------------------------------------------------
// THE EDIT'S OWN INCREMENTAL WALK — the vanilla LightEngine's block-change
// answer (the 26.2 jar's SkyLightEngine/BlockLightEngine checkBlock update
// the one setBlock speaks), the two-queue removal/increase flood the FULL
// recompute's per-edit cost retires.
// ---------------------------------------------------------------------------

/// The grid layer's own read at world-grid coordinates — `None` outside
/// the world's extent (the flood's own wall, the compute walk's same
/// bounds the chunk arithmetic speaks).
fn layer_get(layer: &[Vec<u8>], side: usize, x: i32, y: i32, z: i32) -> Option<u8> {
    if y < 0 || y >= CHUNK_Y as i32 {
        return None;
    }
    let cx = x.div_euclid(CHUNK_X as i32);
    let cz = z.div_euclid(CHUNK_Z as i32);
    if cx < 0 || cz < 0 {
        return None;
    }
    let s = side as i32;
    if cx >= s || cz >= s {
        return None;
    }
    let ci = (cx + cz * s) as usize;
    let idx = x.rem_euclid(CHUNK_X as i32) as usize
        + z.rem_euclid(CHUNK_Z as i32) as usize * CHUNK_X
        + y as usize * CHUNK_X * CHUNK_Z;
    Some(layer[ci][idx])
}

fn layer_set(layer: &mut [Vec<u8>], side: usize, x: i32, y: i32, z: i32, v: u8) {
    let cx = x.div_euclid(CHUNK_X as i32);
    let cz = z.div_euclid(CHUNK_Z as i32);
    let s = side as i32;
    if cx < 0 || cz < 0 || cx >= s || cz >= s || y < 0 || y >= CHUNK_Y as i32 {
        return;
    }
    let ci = (cx + cz * s) as usize;
    let idx = x.rem_euclid(CHUNK_X as i32) as usize
        + z.rem_euclid(CHUNK_Z as i32) as usize * CHUNK_X
        + y as usize * CHUNK_X * CHUNK_Z;
    layer[ci][idx] = v;
}

/// The walk's own level read — the honest [`LIGHT_UNKNOWN`] the wire
/// grid carries reads as `None` (the offline grid the full compute
/// spoke never carries one; the guard the server-owned light keeps).
fn level_at(layer: &[Vec<u8>], side: usize, x: i32, y: i32, z: i32) -> Option<u8> {
    match layer_get(layer, side, x, y, z) {
        None | Some(LIGHT_UNKNOWN) => None,
        Some(v) => Some(v),
    }
}

/// THE ONE CELL'S OWN LIGHT ANSWER — the grid the edit's voxel change
/// wears, updated in place: the old light the removal BFS un-makes,
/// the boundary + the new sources the increase BFS re-lights, the
/// final grid the full [`compute`] would speak BIT FOR BIT (the
/// equality law the randomized walk pins). The two channels walk the
/// SAME shape the full compute keeps: the sky column's down free ride
/// (the 15-through-opacity-0 edge, its removal the same law undoes),
/// every other edge `level - max(1, opacity)`, the opaque wall the
/// flood never enters; the block channel the emissive seed + the same
/// max(1, opacity) edges, NO free ride.
pub fn update_cell(world: &World, grid: &mut LightGrid, x: i32, y: i32, z: i32) {
    let side = world.chunks_per_side;
    let h = CHUNK_Y as i32;

    // THE HONEST UNKNOWN'S OWN SKIP: the wire world's server-owned
    // light the walk never rewrites.
    if level_at(&grid.sky, side, x, y, z).is_none()
        || level_at(&grid.block, side, x, y, z).is_none()
    {
        return;
    }
    let op_here = sky_opacity(world.get(x, y, z));

    // ================= THE SKY CHANNEL =================
    // THE REMOVAL PASS: the cell's old level seeds the darkening BFS —
    // the neighbors the old level fed (< level) join zeroed, the
    // boundary (the cells the edit never fed, >= level) the increase
    // queue re-seeds, the DOWN FREE RIDE the same law undoes (the 15
    // column the cell fed straight down the removal cascade speaks).
    let mut dark: Vec<(i32, i32, i32, u8)> = Vec::new();
    let mut bright: Vec<(i32, i32, i32)> = Vec::new();

    let old_sky = level_at(&grid.sky, side, x, y, z).unwrap_or(0);
    layer_set(&mut grid.sky, side, x, y, z, 0);
    if old_sky > 0 {
        dark.push((x, y, z, old_sky));
    }
    let mut head = 0;
    while head < dark.len() {
        let (cx, cy, cz, lv) = dark[head];
        head += 1;
        for &(dx, dy, dz) in &NEIGHBORS {
            let (nx, ny, nz) = (cx + dx, cy + dy, cz + dz);
            let Some(nl) = level_at(&grid.sky, side, nx, ny, nz) else {
                continue;
            };
            if nl == 0 {
                continue;
            }
            let op = sky_opacity(world.get(nx, ny, nz));
            if op >= 15 {
                continue;
            }
            if dy == -1 && lv == 15 && nl == 15 && op == 0 {
                // THE DOWN FREE RIDE'S OWN UNDOING — the column below
                // the cell the old 15 fed, the removal cascade walks.
                layer_set(&mut grid.sky, side, nx, ny, nz, 0);
                dark.push((nx, ny, nz, 15));
            } else if nl < lv {
                layer_set(&mut grid.sky, side, nx, ny, nz, 0);
                dark.push((nx, ny, nz, nl));
            } else {
                bright.push((nx, ny, nz));
            }
        }
    }

    // THE RE-LIGHT SEEDS: the removal's boundary + the SIX neighbors
    // the transparency's own change rides — the cliff face's new air
    // cell the side's own 15 lights (the no-removal edit the dark pass
    // never seeded), the ceiling cell the direct walk itself speaks.
    if op_here < 15 {
        for &(dx, dy, dz) in &NEIGHBORS {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if level_at(&grid.sky, side, nx, ny, nz).is_some() {
                bright.push((nx, ny, nz));
            }
        }
        if y == h - 1 && op_here == 0 {
            // THE COLUMN'S OWN CEILING CELL — the topmost transparent
            // cell the direct 15 wears (the up-neighbor the lower
            // cells' carrier speaks the rest).
            layer_set(&mut grid.sky, side, x, y, z, 15);
            bright.push((x, y, z));
        }
    }

    // THE INCREASE PASS (the compute flood's own edges verbatim): the
    // down free ride, the max(1, opacity) step, the strict-improvement
    // gate — the monotone convergence the FIFO order keeps.
    let mut head = 0;
    while head < bright.len() {
        let (cx, cy, cz) = bright[head];
        head += 1;
        let Some(level) = level_at(&grid.sky, side, cx, cy, cz) else {
            continue;
        };
        if level == 0 {
            continue;
        }
        for &(dx, dy, dz) in &NEIGHBORS {
            let (nx, ny, nz) = (cx + dx, cy + dy, cz + dz);
            let Some(cur) = level_at(&grid.sky, side, nx, ny, nz) else {
                continue;
            };
            let op = sky_opacity(world.get(nx, ny, nz));
            if op >= 15 {
                continue;
            }
            let next = if dy == -1 && level == 15 && op == 0 {
                15
            } else {
                level.saturating_sub(op.max(1))
            };
            if next > cur {
                layer_set(&mut grid.sky, side, nx, ny, nz, next);
                bright.push((nx, ny, nz));
            }
        }
    }

    // ================= THE BLOCK CHANNEL =================
    // THE REMOVAL PASS: the same darkening walk, NO down free ride (the
    // block light's own physics the compute flood keeps).
    let mut dark: Vec<(i32, i32, i32, u8)> = Vec::new();
    let mut bright: Vec<(i32, i32, i32)> = Vec::new();

    let old_block = level_at(&grid.block, side, x, y, z).unwrap_or(0);
    layer_set(&mut grid.block, side, x, y, z, 0);
    if old_block > 0 {
        dark.push((x, y, z, old_block));
    }
    let mut head = 0;
    while head < dark.len() {
        let (cx, cy, cz, lv) = dark[head];
        head += 1;
        for &(dx, dy, dz) in &NEIGHBORS {
            let (nx, ny, nz) = (cx + dx, cy + dy, cz + dz);
            let Some(nl) = level_at(&grid.block, side, nx, ny, nz) else {
                continue;
            };
            if nl == 0 {
                continue;
            }
            let op = sky_opacity(world.get(nx, ny, nz));
            if op >= 15 {
                continue;
            }
            if nl < lv {
                layer_set(&mut grid.block, side, nx, ny, nz, 0);
                dark.push((nx, ny, nz, nl));
            } else {
                bright.push((nx, ny, nz));
            }
        }
    }

    // THE SEEDS: the new emission the edited cell speaks (the placed
    // torch's own 14, the lava's 15), the removal's boundary, the SIX
    // neighbors the transparency's change rides (the cave wall's new
    // air the torch's own flood lights — the opaque cell's edit the
    // removal alone answers).
    let e = emission(world.get(x, y, z));
    layer_set(&mut grid.block, side, x, y, z, e);
    if e > 0 {
        bright.push((x, y, z));
    }
    if op_here < 15 {
        for &(dx, dy, dz) in &NEIGHBORS {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if level_at(&grid.block, side, nx, ny, nz).is_some() {
                bright.push((nx, ny, nz));
            }
        }
    }

    // THE INCREASE PASS (the compute block flood's own edges — the
    // max(1, opacity) step, no free ride).
    let mut head = 0;
    while head < bright.len() {
        let (cx, cy, cz) = bright[head];
        head += 1;
        let Some(level) = level_at(&grid.block, side, cx, cy, cz) else {
            continue;
        };
        if level == 0 {
            continue;
        }
        for &(dx, dy, dz) in &NEIGHBORS {
            let (nx, ny, nz) = (cx + dx, cy + dy, cz + dz);
            let Some(cur) = level_at(&grid.block, side, nx, ny, nz) else {
                continue;
            };
            let op = sky_opacity(world.get(nx, ny, nz));
            if op >= 15 {
                continue;
            }
            let next = level.saturating_sub(op.max(1));
            if next > cur {
                layer_set(&mut grid.block, side, nx, ny, nz, next);
                bright.push((nx, ny, nz));
            }
        }
    }
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
    fn the_candle_pair_speak_the_lightlevel_lambda() {
        // THE CANDLE PAIR'S OWN ROWS (the autopush-522 round — javap
        // CandleBlock.lambda$static$0 over the 26.2 jar: the `lit ? 3 *
        // candles : 0` fold the lightLevel registration banks, the
        // 1-candle default state the kind-only palette speaks — the
        // UNLIT kind the dark 0 the else arm keeps, the LIT kind the
        // imul's own 3 answers, the flood the relight answers)
        let mut w = empty_world();
        w.set(3, 6, 3, CANDLE);
        w.set(8, 6, 3, CANDLE_LIT);
        w.relight();
        assert_eq!(light_at(&w, 3, 6, 3).1, Some(0), "the unlit kind dark");
        assert_eq!(light_at(&w, 8, 6, 3).1, Some(3), "the lit kind's own cell");
        assert_eq!(light_at(&w, 9, 6, 3).1, Some(2), "one east of the candle");
        assert_eq!(light_at(&w, 8, 7, 3).1, Some(2), "one above the candle");
    }

    #[test]
    fn the_candle_cake_pair_speaks_the_lightlevel_lambda() {
        // THE CANDLE CAKE PAIR'S OWN ROWS (the autopush-523 round —
        // javap the blocks clinit's candle_cake registration over the
        // 26.2 jar: the `iconst_3; litBlockEmission(I)` lightLevel
        // call the ofLegacyCopy(CAKE) props chain banks — the
        // lit-state read the emission's own verdict keeps, the UNLIT
        // kind the dark 0 the else arm keeps, the LIT kind the 3 the
        // candle's own row shares, the flood's own spread the
        // relight answers)
        let mut w = empty_world();
        w.set(3, 6, 3, CANDLE_CAKE);
        w.set(8, 6, 3, CANDLE_CAKE_LIT);
        w.relight();
        assert_eq!(light_at(&w, 3, 6, 3).1, Some(0), "the unlit kind dark");
        assert_eq!(
            light_at(&w, 8, 6, 3).1,
            Some(3),
            "the lit kind's own cell"
        );
        assert_eq!(light_at(&w, 9, 6, 3).1, Some(2), "one east of the cake");
        assert_eq!(light_at(&w, 8, 7, 3).1, Some(2), "one above the cake");
    }

    #[test]
    fn the_fire_kinds_speak_the_clinit_lambdas_emission() {
        // THE FIRE KINDS' OWN ROWS (the autopush-515 round — the
        // blocks clinit's lightLevel lambdas the two registrations
        // wear: fire `lambda$static$67`'s `bipush 15; ireturn`, soul
        // fire `lambda$static$68`'s `bipush 10; ireturn` — the
        // constants the palette's default-state fold speaks, the
        // flood's own spread the relight answers)
        let mut w = empty_world();
        w.set(3, 6, 3, FIRE);
        w.set(8, 6, 3, SOUL_FIRE);
        w.relight();
        assert_eq!(light_at(&w, 3, 6, 3).1, Some(15), "the fire's own cell");
        assert_eq!(light_at(&w, 4, 6, 3).1, Some(14), "one east of the fire");
        assert_eq!(light_at(&w, 8, 6, 3).1, Some(10), "the soul fire's own cell");
        assert_eq!(light_at(&w, 9, 6, 3).1, Some(9), "one east of the soul");
        assert_eq!(light_at(&w, 8, 7, 3).1, Some(9), "one above the soul");
    }

    #[test]
    fn the_campfires_speak_the_lightlevel_lambdas() {
        // THE CAMPFIRE FAMILY'S OWN ROWS (the dev-6 round — javap the
        // blocks clinit's campfire/soul_campfire registrations over
        // the 26.2 jar: the `bipush 15; litBlockEmission(I)` and the
        // `bipush 10` lightLevel calls the lambda$static$310/311
        // props chains bank — the lit-state read the emission's own
        // verdict keeps, the UNLIT kinds the dark 0 the else arm
        // keeps, the flood's own spread the relight answers)
        let mut w = empty_world();
        w.set(3, 6, 3, CAMPFIRE);
        w.set(8, 6, 3, CAMPFIRE_LIT);
        w.set(14, 6, 3, SOUL_CAMPFIRE_LIT);
        w.relight();
        assert_eq!(light_at(&w, 3, 6, 3).1, Some(0), "the unlit kind dark");
        assert_eq!(
            light_at(&w, 8, 6, 3).1,
            Some(15),
            "the lit campfire's own cell"
        );
        assert_eq!(light_at(&w, 9, 6, 3).1, Some(14), "one east of the flame");
        assert_eq!(light_at(&w, 8, 7, 3).1, Some(14), "one above the flame");
        assert_eq!(
            light_at(&w, 14, 6, 3).1,
            Some(10),
            "the soul campfire's own cell"
        );
        assert_eq!(light_at(&w, 15, 6, 3).1, Some(9), "one east of the soul");
    }

    #[test]
    fn the_furnace_speaks_the_litblockemission_thirteen() {
        // THE LIT FURNACE'S OWN ROW (the autopush-601 round — javap
        // the blocks clinit's furnace registration over the 26.2 jar:
        // the `bipush 13; litBlockEmission(I)` lightLevel call the
        // putstatic FURNACE walk pc 7188-7190 banks — the lit-state
        // read the emission's own verdict keeps, the UNLIT kind the
        // dark 0 the else arm keeps, the flood's own spread the
        // relight answers)
        let mut w = empty_world();
        w.set(3, 6, 3, FURNACE);
        w.set(8, 6, 3, FURNACE_LIT);
        w.relight();
        assert_eq!(light_at(&w, 3, 6, 3).1, Some(0), "the unlit kind dark");
        assert_eq!(
            light_at(&w, 8, 6, 3).1,
            Some(13),
            "the lit furnace's own cell"
        );
        assert_eq!(light_at(&w, 9, 6, 3).1, Some(12), "one east of the flame");
        assert_eq!(light_at(&w, 8, 7, 3).1, Some(12), "one above the flame");
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

    // ------------------------------------------------------------------
    // THE EDIT'S OWN INCREMENTAL WALK — the laws the one-cell answer
    // wears (the full compute the walk must speak BIT FOR BIT).
    // ------------------------------------------------------------------

    /// The walk's own EQUALITY HAMMER — the real terrain (the trees,
    /// the water, the flowers, the cave mouths the generate rolls) the
    /// randomized edit sequence rides, the grid after EVERY edit the
    /// full compute would speak bit for bit. The set walk's own
    /// incremental answer ([`update_cell`]) the placement/dig lanes
    /// wear — the equality law the pixel goldens inherit.
    #[test]
    fn the_incremental_walk_the_full_compute_wears() {
        const PALETTE: [u16; 12] = [
            AIR, STONE, DIRT, WATER, LEAVES, PALE_LEAVES, LEAF_LITTER, LAVA, TORCH, DANDELION,
            POPPY, SHORT_GRASS,
        ];
        for seed in [7u64, 1337, 90_210] {
            let mut w = World::new(2, seed); // the 32x32 real terrain
            w.relight();
            let mut rng = crate::drops::Lcg::new(seed * 31 + 17);
            for i in 0..80 {
                let x = rng.next_int(32);
                let y = rng.next_int(CHUNK_Y as u64);
                let z = rng.next_int(32);
                let id = PALETTE[rng.next_int(PALETTE.len() as u64) as usize];
                w.set(x, y, z, id);
                let expected = crate::light::compute(&w);
                assert_eq!(
                    w.light.as_ref().expect("the walk's grid"),
                    &expected,
                    "seed {seed} edit {i}: ({x},{y},{z}) -> {id} the walk the compute never splits"
                );
            }
        }
    }

    /// THE PLACED ROOF'S OWN SHADOW — the stone slab the open field's
    /// column wore mid-air: the cells below the direct 15 the removal
    /// cascade un-makes (the DOWN FREE RIDE the removal law undoes),
    /// the slab's rim the boundary re-flood speaks, the dig-back the
    /// column the same walk restores. The full compute the grid after
    /// every step never splits.
    #[test]
    fn the_placed_roof_the_shadow_the_removal_wears() {
        let mut w = empty_world();
        w.relight();
        // THE PLACE: the slab lands mid-air, the shadow column below
        // the removal speaks (the open field's 15s the slab's cell
        // fed straight down).
        w.set(8, 20, 8, STONE);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        // THE OPEN FIELD'S OWN ANSWER: the direct 15 the slab retired,
        // the side-flood's 14 the rim's boundary speaks (the open
        // columns the shaft's own cells lean on), the opaque cell the
        // zero verdict keeps.
        assert_eq!(light_at(&w, 8, 20, 8).0, Some(0), "the slab's own cell dark");
        assert_eq!(light_at(&w, 8, 19, 8).0, Some(14), "the shaft the side-flood lights");
        assert_eq!(light_at(&w, 8, 10, 8).0, Some(14), "the deep shaft the same flood keeps");
        assert_eq!(light_at(&w, 8, 21, 8).0, Some(15), "the slab's own top open");
        assert_eq!(light_at(&w, 9, 19, 8).0, Some(15), "the open column beside");
        // THE DIG BACK: the slab leaves, the column the direct walk
        // restores — the 15s the whole shaft speaks again.
        w.set(8, 20, 8, AIR);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        assert_eq!(light_at(&w, 8, 10, 8).0, Some(15), "the shaft's own daylight back");
    }

    /// THE CAVE TORCH'S OWN FLOOD — the sealed chamber the torch's
    /// block light wears: the placed torch the emissive seed speaks
    /// (the 14 the source cell carries, the falloff the walls ride),
    /// the removal the darkening BFS un-makes, the never-lit world's
    /// guard the full compute keeps (the honest None the set walk
    /// never touches).
    #[test]
    fn the_cave_torch_the_incremental_flood_wears() {
        let mut w = empty_world();
        // THE SEALED CHAMBER: the stone shell the sky's own column
        // never enters (the interior the dark the shell keeps).
        for y in 3..8 {
            for z in 3..10 {
                for x in 3..10 {
                    let shell = x == 3 || x == 9 || z == 3 || z == 9 || y == 3 || y == 7;
                    w.set(x, y, z, if shell { STONE } else { AIR });
                }
            }
        }
        w.relight();
        assert_eq!(light_at(&w, 6, 5, 6).0, Some(0), "the sealed interior dark");
        // THE PLACE: the torch lands, the flood the seed speaks — the
        // grid the full compute would speak bit for bit.
        w.set(6, 5, 6, TORCH);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        assert_eq!(light_at(&w, 6, 5, 6).1, Some(14), "the torch's own cell");
        assert_eq!(light_at(&w, 8, 5, 6).1, Some(12), "the falloff two cells east");
        assert_eq!(light_at(&w, 6, 5, 6).0, Some(0), "the sky the shell still blocks");
        // THE BREAK: the torch leaves, the darkening BFS the flood
        // un-makes — the chamber the dark the shell keeps again.
        w.set(6, 5, 6, AIR);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        assert_eq!(light_at(&w, 8, 5, 6).1, Some(0), "the stale glow gone");
    }

    /// THE CLIFF FACE'S OWN NEW AIR — the wall cell the set walk
    /// transparent-izes where NO removal runs (the opaque cell's own
    /// sky the dark pass never seeds): the side's own 15 the boundary
    /// seeding lights (the 14 the one step down speaks), the grid the
    /// full compute bit for bit keeps.
    #[test]
    fn the_cliff_face_the_new_air_the_boundary_lights() {
        let mut w = empty_world();
        // THE CLIFF: the stone half the open half's column never
        // enters (the wall face the sky's own shadow wears).
        for y in 0..20 {
            for z in 0..16 {
                for x in 8..16 {
                    w.set(x, y, z, STONE);
                }
            }
        }
        w.relight();
        assert_eq!(light_at(&w, 8, 10, 5).0, Some(0), "the wall's own cell dark");
        // THE CARVE: the face cell leaves, no removal ran (the opaque
        // cell carried nothing) — the side's open 15 the seed re-floods
        // the one step down.
        w.set(8, 10, 5, AIR);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        assert_eq!(light_at(&w, 8, 10, 5).0, Some(14), "the face's own side-lit air");
        assert_eq!(light_at(&w, 7, 10, 5).0, Some(15), "the open neighbor's column");
    }

    /// THE LAVA'S OWN EMISSION EDIT — the opaque emitter the both
    /// channels wear: the placed lava the sky's own column AND the
    /// block's own 15 the seeds speak, the stone the emission's flood
    /// retires, the grid the full compute never splits.
    #[test]
    fn the_lava_the_emission_edit_wears() {
        let mut w = empty_world();
        w.relight();
        // THE PLACE: the lava lands mid-air — the sky column STOPS at
        // the opaque cell (the shadow below the column walk speaks)
        // and the block 15 floods the neighbors the emissive seed
        // lights.
        w.set(4, 12, 4, LAVA);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        assert_eq!(light_at(&w, 4, 12, 4).0, Some(0), "the opaque cell's own sky");
        // the column STOPS at the lava — the direct 15 the cell below
        // never wears, the side-flood's own 14 the open columns lean in
        assert_eq!(light_at(&w, 4, 11, 4).0, Some(14), "the column the lava stops");
        assert_eq!(light_at(&w, 4, 12, 4).1, Some(15), "the lava's own emission");
        assert_eq!(light_at(&w, 6, 12, 4).1, Some(13), "the flood's own falloff");
        // THE RETIRE: the stone takes the lava's cell — the emission's
        // flood the removal un-makes, the sky the opaque verdict keeps.
        w.set(4, 12, 4, STONE);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        assert_eq!(light_at(&w, 6, 12, 4).1, Some(0), "the emission's flood gone");
        assert_eq!(light_at(&w, 4, 12, 4).1, Some(0), "the stone's own cell dark");
    }

    /// THE WATER COLUMN'S OWN PARTIAL EDIT — the attenuator the
    /// opacity-1 family wears: the placed water the column's 14 the
    /// one-step-down flood speaks (the free ride the partial opacity
    /// never rides), the dug-out the direct 15 the column restores,
    /// the leaves the same law carries. The full compute the grid
    /// after every edit bit for bit keeps.
    #[test]
    fn the_water_column_the_partial_edit_wears() {
        let mut w = empty_world();
        w.relight();
        // THE PLACE: the water cell takes the open column's top — the
        // column STOPS at the attenuator (the cell itself the flood's
        // own 14 wears, the direct 15 the partial opacity never rides).
        w.set(4, 20, 4, WATER);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        assert_eq!(light_at(&w, 4, 20, 4).0, Some(14), "the water's own attenuated sky");
        // THE BELOW: the side-flood's own 14 the open columns lean in
        // (the water's own 13 the one step down speaks, the rim's 14
        // the max verdict wears).
        assert_eq!(light_at(&w, 4, 19, 4).0, Some(14), "the below the side-flood lights");
        // THE DIG OUT: the water leaves, the direct column the free
        // ride restores — the 15s the whole shaft speaks.
        w.set(4, 20, 4, AIR);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        assert_eq!(light_at(&w, 4, 20, 4).0, Some(15), "the direct column back");
        assert_eq!(light_at(&w, 4, 19, 4).0, Some(15), "the free ride back");
        // THE LEAVES: the canopy cell the same partial law wears.
        w.set(4, 20, 4, LEAVES);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        assert_eq!(light_at(&w, 4, 20, 4).0, Some(14), "the canopy's own shade");
    }

    /// THE CEILING CELL'S OWN DIRECT SEED — the world's topmost
    /// transparent cell the up-neighbor never carries (the column's
    /// own ceiling the direct walk seeds): the placed air the 15
    /// speaks, the placed water the ceiling seed refuses (the partial
    /// opacity the column walk stops at), the grid the full compute
    /// bit for bit keeps.
    #[test]
    fn the_ceiling_cell_the_direct_seed_wears() {
        let mut w = empty_world();
        // THE CAP: the stone lid the whole world's top wears — every
        // column the ceiling stops.
        for z in 0..16 {
            for x in 0..16 {
                w.set(x, CHUNK_Y as i32 - 1, z, STONE);
            }
        }
        w.relight();
        let top = CHUNK_Y as i32 - 1;
        assert_eq!(light_at(&w, 8, top, 8).0, Some(0), "the capped cell dark");
        assert_eq!(light_at(&w, 8, top - 1, 8).0, Some(0), "the below the cap dark");
        // THE CARVE: the ceiling cell leaves — the direct seed the
        // topmost transparent cell speaks (the up-neighbor the lower
        // cells' carrier), the free ride the column below floods.
        w.set(8, top, 8, AIR);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        assert_eq!(light_at(&w, 8, top, 8).0, Some(15), "the ceiling's own direct 15");
        assert_eq!(light_at(&w, 8, top - 1, 8).0, Some(15), "the free ride below");
        // THE WATER LID: the attenuator takes the ceiling cell — the
        // direct seed the opacity refuses AND the old 15's own column
        // the removal cascade un-makes (the free ride the top cell fed
        // the whole shaft, the capped world's dark the flood restores).
        w.set(8, top, 8, WATER);
        let expected = crate::light::compute(&w);
        assert_eq!(w.light.as_ref().expect("walked"), &expected);
        assert_eq!(light_at(&w, 8, top, 8).0, Some(0), "the capped water no direct seed");
        assert_eq!(light_at(&w, 8, top - 1, 8).0, Some(0), "the free ride the lid removed");
    }

    /// THE BATCH'S OWN TRENCH — the explosion-shaped multi-cell dig
    /// the one tick wears: every per-set walk the compositionality the
    /// full compute after EACH edit never splits (the lane's own
    /// batched rhythm the per-set answer carries).
    #[test]
    fn the_batch_trench_the_per_edit_walk_wears() {
        let mut w = empty_world();
        // THE GROUND: the stone floor the trench digs into.
        for z in 0..16 {
            for x in 0..16 {
                w.set(x, 0, z, STONE);
                w.set(x, 1, z, STONE);
            }
        }
        w.relight();
        // THE TRENCH: the L-shaped dig the explosion lane's own batch
        // wears — after EVERY set the grid the full compute keeps.
        let cells = [(4, 1, 4), (5, 1, 4), (6, 1, 4), (6, 1, 5), (6, 0, 4), (4, 0, 4)];
        for (i, (x, y, z)) in cells.iter().enumerate() {
            w.set(*x, *y, *z, AIR);
            let expected = crate::light::compute(&w);
            assert_eq!(
                w.light.as_ref().expect("walked"),
                &expected,
                "trench set {i} the walk the compute never splits"
            );
        }
        assert_eq!(light_at(&w, 4, 1, 4).0, Some(15), "the trench's own daylight");
        assert_eq!(light_at(&w, 6, 0, 4).0, Some(15), "the deep end the shaft lights");
    }
}

//! THE HULL'S OWN PHYSICS — javap AbstractBoat's authoritative lane
//! over the 26.2 jar: the status detection (getStatus's own walk), the
//! float folds (floatBoat's own words), and the y-move's collision
//! clamp (Entity.move's axis over full cubes). The OFFLINE hulls the
//! App's own tick rides; the wire's hulls the server's snapshots own —
//! the walk never touches them.

use crate::world::{AIR, LAVA, WATER};

/// THE BOAT FAMILY'S OWN BOX (the same EntityType constants the ride
/// height's doc speaks — the 1.375 x 0.562 the makeBoundingBox centers
/// on the hull's position, the yaw the AABB never rotates).
pub const HULL_WIDTH: f64 = crate::world_view::BOAT_DIMENSIONS_WIDTH as f64;
pub const HULL_HEIGHT: f64 = crate::world_view::BOAT_DIMENSIONS_HEIGHT as f64;

/// getDefaultGravity's own word (the 0.04 the ldc2_w speaks — the
/// POSITIVE attribute default the folds divide by 0.65; the -getGravity()
/// the float's tail adds rides the same constant's negation).
pub const HULL_GRAVITY: f64 = 0.04;

/// THE SPLASH-DOWN'S OWN OFFSET (the 0.101 the dadd the arm's d6
/// fold speaks).
pub const SPLASH_DOWN: f64 = 0.101;

/// THE WIRE HULL'S OWN LERPTO STATE — the client's eased copy of the
/// server-owned hull (the vanilla Entity.lerpTo law the remote bodies
/// speak: the target the latest sync arms, the 3-tick walk eases, the
/// prev trio the render's partial-tick lerp anchors). The offline
/// hulls keep no lerp — their own tick IS the truth.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HullLerp {
    /// the pose the last tick eased FROM (the render's own anchor)
    pub prev: (f64, f64, f64),
    /// the latest sync's word the walk eases toward
    pub target: (f64, f64, f64),
    /// the steps the walk has left (the vanilla lerpSteps' own 3 the
    /// sync's arrival arms; the 0 the exhausted walk keeps)
    pub steps: u8,
}

/// THE LERPTO'S OWN STEP (the vanilla Entity.lerp's walk the mobs
/// speak): `pos += (target - pos) / steps`, then the count's descent.
/// The zero-steps silence the exhausted walk keeps (the hull rests at
/// its word until the next sync re-arms).
pub fn hull_lerp_axes(
    cur: (f64, f64, f64),
    target: (f64, f64, f64),
    steps: u8,
) -> ((f64, f64, f64), u8) {
    if steps == 0 {
        return (cur, steps);
    }
    let s = steps as f64;
    (
        (
            cur.0 + (target.0 - cur.0) / s,
            cur.1 + (target.1 - cur.1) / s,
            cur.2 + (target.2 - cur.2) / s,
        ),
        steps - 1,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HullStatus {
    InWater,
    UnderWater,
    /// the flowing arm our full-cube vocabulary never speaks (the
    /// source's own word IS the whole family) — the honest dead arm
    /// the walk still encodes
    UnderFlowingWater,
    OnLand,
    InAir,
}

/// THE DEGREE'S OWN RADIAN (the controlBoat fold's ldc word — the f32
/// `0.01745329238474369` the π/180 rounding keeps, the sin/cos angles
/// the thrust's direction folds multiply).
pub const HULL_RAD: f32 = 0.01745329238474369;

/// One hull's own physics state — the AbstractBoat fields the
/// authoritative lane keeps: the delta's three words (the y the float's
/// lift keeps, the x/z the controlBoat thrust feeds and the friction
/// decays — the placed hull's zeros the silence keeps), the delta
/// rotation the friction decays and the ±1 folds turn, the two
/// statuses the tick's head walks, the water level the detection's
/// side effects write, the lastYd the move's checkFallDamage stores.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HullPhys {
    pub delta_x: f64,
    pub delta_y: f64,
    pub delta_z: f64,
    pub delta_rotation: f32,
    pub status: HullStatus,
    pub old_status: HullStatus,
    pub water_level: f64,
    pub last_yd: f64,
}

impl Default for HullPhys {
    fn default() -> Self {
        Self {
            delta_x: 0.0,
            delta_y: 0.0,
            delta_z: 0.0,
            delta_rotation: 0.0,
            status: HullStatus::InAir,
            old_status: HullStatus::InAir,
            water_level: 0.0,
            last_yd: 0.0,
        }
    }
}

/// the collision vocabulary the move's clamp walks (the fluids never
/// block — the vanilla's own empty collision shapes)
fn solid(id: u16) -> bool {
    id != AIR && id != WATER && id != LAVA
}

/// the strict intersect the block lane's own shape keeps (the AABB
/// law: `lo < chi && hi > clo` — the touching boundary never
/// intersects)
fn span_hits(clo: f64, chi: f64, lo: f64, hi: f64) -> bool {
    lo < chi && hi > clo
}

/// THE STATUS WORD (javap getStatus's own walk): the UNDER band first
/// (the top band's water the surface law keeps — the maxY the
/// waterLevel's side effect writes), the base band's water (the
/// checkInWater walk — the max surface the waterLevel's own word, the
/// -DBL_MAX init the no-water arm keeps), the ground friction's slab
/// (the OnLand the friction's freshness keeps), the IN_AIR the honest
/// silence keeps. `prev_wl` the field's own word the non-water arms
/// keep. Returns (status, water_level, ground_friction) — the
/// friction's NaN (0.0/0.0) the vanilla's own IN_AIR verdict speaks.
pub fn hull_status(
    box6: (f64, f64, f64, f64, f64, f64),
    prev_wl: f64,
    cell: &impl Fn(i32, i32, i32) -> u16,
) -> (HullStatus, f64, f32) {
    let (min_x, min_y, min_z, max_x, max_y, max_z) = box6;
    // the fluid walks' own half-open spans: [floor(min), ceil(max)) —
    // the boundary-touching max cell the ceil's own exclusion keeps
    let x_span = (min_x.floor() as i32)..(max_x.ceil() as i32);
    let z_span = (min_z.floor() as i32)..(max_z.ceil() as i32);
    // THE UNDER BAND (isUnderwater's own walk): the top band's cells
    // floor(maxY)..ceil(maxY + 0.001) exclusive
    let d2 = max_y + 0.001;
    'top: for cy in (max_y.floor() as i32)..(d2.ceil() as i32) {
        for cx in x_span.clone() {
            for cz in z_span.clone() {
                // the water's own surface (the full-cube cell the
                // project's vocabulary keeps — the height the 1.0
                // stands in for): above the box's top the hull is
                // UNDER (the source's own arm; the flowing branch the
                // vocabulary never speaks)
                if cell(cx, cy, cz) == WATER && d2 < cy as f64 + 1.0 {
                    return (HullStatus::UnderWater, max_y, 0.0);
                }
            }
        }
    }
    // THE BASE BAND (checkInWater's own walk): the bottom row(s)
    // floor(minY)..ceil(minY + 0.001) exclusive, the max surface the
    // waterLevel's side effect keeps
    let mut wl = f64::NEG_INFINITY;
    let mut in_water = false;
    for cy in (min_y.floor() as i32)..((min_y + 0.001).ceil() as i32) {
        for cx in x_span.clone() {
            for cz in z_span.clone() {
                if cell(cx, cy, cz) == WATER && min_y < cy as f64 + 1.0 {
                    in_water = true;
                    wl = wl.max(cy as f64 + 1.0);
                }
            }
        }
    }
    if in_water {
        return (HullStatus::InWater, wl, 0.0);
    }
    // THE GROUND SLAB (getGroundFriction's own walk): the band
    // [minY - 0.001, minY] inflated one cell, the ring's own edge
    // exemption (the edge==2 columns skip whole, the edge>0 columns
    // skip their y-edge rows — the corner-less ring the bytecode
    // speaks), the LilyPad arm the vocabulary never speaks, the
    // friction the 0.6 every solid in the set keeps, the count
    // division the vanilla's own NaN (0.0/0.0) the IN_AIR verdict
    // keeps
    let slab_top = min_y;
    let slab_lo = min_y - 0.001;
    let x0 = min_x.floor() as i32 - 1;
    let x1 = max_x.ceil() as i32 + 1;
    let y0 = slab_lo.floor() as i32 - 1;
    let y1 = slab_top.ceil() as i32 + 1;
    let z0 = min_z.floor() as i32 - 1;
    let z1 = max_z.ceil() as i32 + 1;
    let mut sum = 0.0f32;
    let mut count = 0u32;
    for cx in x0..x1 {
        for cz in z0..z1 {
            let edge = (cx == x0 || cx == x1 - 1) as u32 + (cz == z0 || cz == z1 - 1) as u32;
            if edge == 2 {
                continue;
            }
            for cy in y0..y1 {
                if edge > 0 && (cy == y0 || cy == y1 - 1) {
                    continue;
                }
                if solid(cell(cx, cy, cz)) {
                    sum += 0.6; // the vocabulary's own friction word
                    count += 1;
                }
            }
        }
    }
    let friction = sum / count as f32;
    if friction > 0.0 {
        return (HullStatus::OnLand, prev_wl, friction);
    }
    (HullStatus::InAir, prev_wl, friction)
}

/// getWaterLevelAbove's own walk over the full-cube vocabulary: the
/// rows floor(maxY)..ceil(maxY - lastYd) exclusive, the water row the
/// climb keeps (the f10 >= 1.0 the next-row goto speaks), the dry
/// row's own y the return keeps, the all-water span the y1 + 1 word
/// returns.
fn water_level_above(
    x: f64,
    y: f64,
    z: f64,
    last_yd: f64,
    cell: &impl Fn(i32, i32, i32) -> u16,
) -> f64 {
    let min_x = x - HULL_WIDTH / 2.0;
    let max_x = x + HULL_WIDTH / 2.0;
    let min_z = z - HULL_WIDTH / 2.0;
    let max_z = z + HULL_WIDTH / 2.0;
    let max_y = y + HULL_HEIGHT;
    let y0 = max_y.floor() as i32;
    let y1 = (max_y - last_yd).ceil() as i32;
    for cy in y0..y1 {
        let mut wet = false;
        'row: for cx in (min_x.floor() as i32)..(max_x.ceil() as i32) {
            for cz in (min_z.floor() as i32)..(max_z.ceil() as i32) {
                if cell(cx, cy, cz) == WATER {
                    wet = true;
                    break 'row;
                }
            }
        }
        if !wet {
            return cy as f64;
        }
    }
    (y1 + 1) as f64
}

/// the splash-down's own noCollision gate (the box the teleport's
/// target keeps — no solid cell strictly intersects)
fn box_free(x: f64, y: f64, z: f64, cell: &impl Fn(i32, i32, i32) -> u16) -> bool {
    let hw = HULL_WIDTH / 2.0;
    let (min_x, max_x) = (x - hw, x + hw);
    let (min_z, max_z) = (z - hw, z + hw);
    for cy in (y.floor() as i32)..=((y + HULL_HEIGHT).floor() as i32 + 1) {
        for cx in (min_x.floor() as i32)..=(max_x.floor() as i32) {
            for cz in (min_z.floor() as i32)..=(max_z.floor() as i32) {
                if solid(cell(cx, cy, cz))
                    && span_hits(cx as f64, cx as f64 + 1.0, min_x, max_x)
                    && span_hits(cz as f64, cz as f64 + 1.0, min_z, max_z)
                    && span_hits(cy as f64, cy as f64 + 1.0, y, y + HULL_HEIGHT)
                {
                    return false;
                }
            }
        }
    }
    true
}

/// Entity.move's y axis over full cubes: the target box's solid cells
/// the strict intersect speaks, the falling walk the highest solid top
/// clamps, the rising walk the lowest solid bottom, the rest the
/// honest pass keeps.
fn y_move(
    x: f64,
    y: f64,
    z: f64,
    dy: f64,
    cell: &impl Fn(i32, i32, i32) -> u16,
) -> f64 {
    if dy == 0.0 {
        return y;
    }
    let hw = HULL_WIDTH / 2.0;
    let (min_x, max_x) = (x - hw, x + hw);
    let (min_z, max_z) = (z - hw, z + hw);
    let ny = y + dy;
    // the swept rows (the superset the strict test trims): the target
    // box's own span plus the start's
    let row_lo = ny.min(y).floor() as i32;
    let row_hi = ny.max(y).floor() as i32 + 1;
    let mut best: Option<f64> = None;
    for cy in row_lo..=row_hi {
        for cx in (min_x.floor() as i32)..=(max_x.floor() as i32) {
            for cz in (min_z.floor() as i32)..=(max_z.floor() as i32) {
                if !solid(cell(cx, cy, cz)) {
                    continue;
                }
                if !span_hits(cx as f64, cx as f64 + 1.0, min_x, max_x)
                    || !span_hits(cz as f64, cz as f64 + 1.0, min_z, max_z)
                    || !span_hits(cy as f64, cy as f64 + 1.0, ny, ny + HULL_HEIGHT)
                {
                    continue;
                }
                best = Some(match (best, dy < 0.0) {
                    (Some(b), true) => b.max(cy as f64 + 1.0),
                    (Some(b), false) => b.min(cy as f64),
                    (None, true) => cy as f64 + 1.0,
                    (None, false) => cy as f64,
                });
            }
        }
    }
    match best {
        Some(c_hi) if dy < 0.0 => ny.max(c_hi),
        Some(c_lo) => ny.min(c_lo - HULL_HEIGHT),
        None => ny,
    }
}

/// THE HULL'S OWN TICK (javap AbstractBoat.tick's authoritative lane):
/// the status detection, the floatBoat folds, the y-move's clamp.
/// Returns the hull's new y — the caller's own write.
pub fn hull_tick(
    phys: &mut HullPhys,
    x: f64,
    y: f64,
    z: f64,
    cell: &impl Fn(i32, i32, i32) -> u16,
) -> f64 {
    let old_status = phys.status;
    let (status, wl, friction) = hull_status(
        (
            x - HULL_WIDTH / 2.0,
            y,
            z - HULL_WIDTH / 2.0,
            x + HULL_WIDTH / 2.0,
            y + HULL_HEIGHT,
            z + HULL_WIDTH / 2.0,
        ),
        phys.water_level,
        cell,
    );
    phys.old_status = old_status;
    phys.status = status;
    phys.water_level = wl;

    // THE FLOAT'S OWN FOLDS (the bytecode's own locals: the gravity
    // the -getGravity() speaks, the lift the 0.0 rest keeps, the
    // friction the 0.05 the clinit's default arms)
    let mut d0 = -HULL_GRAVITY;
    let mut d2 = 0.0f64;
    let mut f5 = 0.05f32;

    if old_status == HullStatus::InAir
        && status != HullStatus::InAir
        && status != HullStatus::OnLand
    {
        // THE SPLASH-DOWN ARM: the water level the getY(1.0) word
        // keeps, the d6 the getWaterLevelAbove's own fold, the
        // noCollision the teleport gate keeps (the delta's y the
        // multiply(1,0,1) zeroes, the lastYd the 0.0 the arm writes),
        // the status the IN_WATER the arm's tail speaks — and the
        // move NEVER runs this tick (the bytecode's own goto return)
        phys.water_level = y + 1.0;
        let d6 = water_level_above(x, y, z, phys.last_yd, cell) - HULL_HEIGHT + SPLASH_DOWN;
        phys.status = HullStatus::InWater;
        if box_free(x, d6, z, cell) {
            phys.delta_y = 0.0;
            phys.last_yd = 0.0;
            return d6;
        }
        return y;
    }
    match status {
        HullStatus::InWater => {
            // the lift's own word: the submersion fraction the
            // waterLevel's own field keeps
            d2 = (phys.water_level - y) / HULL_HEIGHT;
            f5 = 0.9;
        }
        HullStatus::UnderFlowingWater => {
            d0 = -0.0007;
            f5 = 0.9;
        }
        HullStatus::UnderWater => {
            d2 = 0.01;
            f5 = 0.45;
        }
        HullStatus::InAir => {
            f5 = 0.9;
        }
        HullStatus::OnLand => {
            // the friction the detection's own freshness keeps; the
            // player-rider's halving the offline hull never speaks
            // (no riders the offline lane keeps)
            f5 = friction;
        }
    }
    // THE COMMON TAIL: the x/z the friction multiplies (the placed
    // hull's zeros stay zeros), the y the gravity's own word adds, the
    // delta rotation the friction decays, the lift the 0.75 damping
    // folds (the d2 > 0 gate the bytecode keeps)
    phys.delta_x *= f5 as f64;
    phys.delta_y += d0;
    phys.delta_z *= f5 as f64;
    phys.delta_rotation *= f5;
    if d2 > 0.0 {
        phys.delta_y = (phys.delta_y + d2 * HULL_GRAVITY / 0.65) * 0.75;
    }
    // THE MOVE (the y axis the delta carries): the sweep's own clamp,
    // the blocked axis the delta's zero keeps, the lastYd the
    // collided word the checkFallDamage stores
    let dy = phys.delta_y;
    let ny = y_move(x, y, z, dy, cell);
    phys.last_yd = ny - y;
    if ny != y + dy {
        phys.delta_y = 0.0;
    }
    ny
}

/// THE RIDER'S OWN INPUT (the controlBoat walk's four raw booleans —
/// the KeyboardInput record the last_sent_input baseline keeps: the
/// A/D the turn arms, the W the forward thrust, the S the reverse).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HullInput {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

/// javap controlBoat's own walk (raw_disasm over the 26.2 jar — the
/// authoritative hull the offline ride speaks): the `!isVehicle` gate
/// the CALLER's own Option keeps (no rider the walk never arms), the
/// deltaRotation the ±1 folds turn (the f32 field the friction's tail
/// decayed BEFORE this walk — the vanilla order the composition keeps),
/// the single-turn thrust the 0.005 the `right != left && !up && !down`
/// band speaks, the yaw the `setYRot(getYRot() + deltaRotation)` fold
/// carries (BEFORE the up/down thrust reads — the bytecode's own
/// order), the W the 0.04 adds, the S the 0.005 subtracts, the delta
/// movement the `sin(-yaw·rad)·d0 / cos(yaw·rad)·d0` products absorb
/// (the Mth table's own f32 cells, the products the f2d widens), and
/// the paddle state the setPaddleState law speaks — the swapped
/// mapping the boat's own sides keep (`paddleLeft = inputRight &&
/// !inputLeft || inputUp`: the A key strokes the RIGHT paddle, the
/// turn the law carries — the same words the wire lane's override
/// speaks). Returns the new yaw + the two paddle booleans.
pub fn hull_control(phys: &mut HullPhys, input: HullInput, yaw: f32) -> (f32, bool, bool) {
    // THE DELTA ROTATION'S OWN FOLDS (the f32 field the ±1 turns)
    if input.left {
        phys.delta_rotation -= 1.0;
    }
    if input.right {
        phys.delta_rotation += 1.0;
    }
    // THE THRUST LOCAL (the f32 var the bytecode's own slot 1 keeps)
    let mut d0: f32 = 0.0;
    if input.right != input.left && !input.up && !input.down {
        // the single-turn's own 0.005 (the ldc 0.004999999888241291
        // the f32 word widens)
        d0 += 0.005;
    }
    // THE YAW FOLD (the setYRot the sum carries — the thrust's
    // direction the NEW yaw reads)
    let new_yaw = yaw + phys.delta_rotation;
    if input.up {
        // the forward's own 0.04 (the ldc 0.03999999910593033)
        d0 += 0.04;
    }
    if input.down {
        // the reverse's own 0.005
        d0 -= 0.005;
    }
    // THE DELTA MOVEMENT'S OWN ABSORB (the Mth.sin(D)F / Mth.cos(D)F
    // the table's own cells keep, the f32 products the f2d widens)
    let rad = new_yaw * HULL_RAD;
    phys.delta_x += (crate::world_model::narrow_sin(-rad) as f32 * d0) as f64;
    phys.delta_z += (crate::world_model::narrow_cos(rad) as f32 * d0) as f64;
    // THE PADDLE STATE (the setPaddleState law — the swapped mapping)
    let paddle_left = (input.right && !input.left) || input.up;
    let paddle_right = (input.left && !input.right) || input.up;
    (new_yaw, paddle_left, paddle_right)
}

/// Entity.move's x/z axes over full cubes (the compound's own
/// y-then-x-then-z order the caller's composition keeps — the y the
/// hull_tick moved first, the x the z's own sweep resolves): the swept
/// span the strict intersect trims, the +axis walk the nearest
/// blocking cell's near plane minus the half width clamps, the -axis
/// walk the far plane plus the half width, the blocked axis the
/// delta's zero speaks. Returns the clamped (x, z).
pub fn hull_move_xz(
    phys: &mut HullPhys,
    x: f64,
    y: f64,
    z: f64,
    cell: &impl Fn(i32, i32, i32) -> u16,
) -> (f64, f64) {
    let hw = HULL_WIDTH / 2.0;
    // THE X SWEEP (the box's z span the pre-move pose carries)
    let dx = phys.delta_x;
    let nx = x + dx;
    let final_x = if dx == 0.0 {
        x
    } else {
        let (min_z, max_z) = (z - hw, z + hw);
        let (min_x, max_x) = (nx.min(x) - hw, nx.max(x) + hw);
        let mut best: Option<f64> = None;
        for cx in (min_x.floor() as i32)..=(max_x.floor() as i32) {
            for cy in (y.floor() as i32)..=((y + HULL_HEIGHT).floor() as i32 + 1) {
                for cz in (min_z.floor() as i32)..=(max_z.floor() as i32) {
                    if !solid(cell(cx, cy, cz)) {
                        continue;
                    }
                    if !span_hits(cx as f64, cx as f64 + 1.0, nx - hw, nx + hw)
                        || !span_hits(cy as f64, cy as f64 + 1.0, y, y + HULL_HEIGHT)
                        || !span_hits(cz as f64, cz as f64 + 1.0, min_z, max_z)
                    {
                        continue;
                    }
                    best = Some(match (best, dx < 0.0) {
                        (Some(b), true) => b.max(cx as f64 + 1.0),
                        (Some(b), false) => b.min(cx as f64),
                        (None, true) => cx as f64 + 1.0,
                        (None, false) => cx as f64,
                    });
                }
            }
        }
        match best {
            Some(b_lo) if dx < 0.0 => nx.max(b_lo + hw),
            Some(b_hi) => nx.min(b_hi - hw),
            None => nx,
        }
    };
    if final_x != x + dx {
        phys.delta_x = 0.0;
    }
    // THE Z SWEEP (the box's x span the x-resolved position carries)
    let dz = phys.delta_z;
    let nz = z + dz;
    let final_z = if dz == 0.0 {
        z
    } else {
        let (min_x, max_x) = (final_x - hw, final_x + hw);
        let (min_z, max_z) = (nz.min(z) - hw, nz.max(z) + hw);
        let mut best: Option<f64> = None;
        for cz in (min_z.floor() as i32)..=(max_z.floor() as i32) {
            for cy in (y.floor() as i32)..=((y + HULL_HEIGHT).floor() as i32 + 1) {
                for cx in (min_x.floor() as i32)..=(max_x.floor() as i32) {
                    if !solid(cell(cx, cy, cz)) {
                        continue;
                    }
                    if !span_hits(cz as f64, cz as f64 + 1.0, nz - hw, nz + hw)
                        || !span_hits(cy as f64, cy as f64 + 1.0, y, y + HULL_HEIGHT)
                        || !span_hits(cx as f64, cx as f64 + 1.0, min_x, max_x)
                    {
                        continue;
                    }
                    best = Some(match (best, dz < 0.0) {
                        (Some(b), true) => b.max(cz as f64 + 1.0),
                        (Some(b), false) => b.min(cz as f64),
                        (None, true) => cz as f64 + 1.0,
                        (None, false) => cz as f64,
                    });
                }
            }
        }
        match best {
            Some(b_lo) if dz < 0.0 => nz.max(b_lo + hw),
            Some(b_hi) => nz.min(b_hi - hw),
            None => nz,
        }
    };
    if final_z != z + dz {
        phys.delta_z = 0.0;
    }
    (final_x, final_z)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// THE POOL FIXTURE: a water cell at (0, 10, 0), the stone floor at
    /// row 9 (the pool's own bed), everything else air.
    fn pool_cell(x: i32, y: i32, z: i32) -> u16 {
        if x == 0 && z == 0 {
            if y == 10 {
                return WATER;
            }
            if y == 9 {
                return crate::world::STONE;
            }
        }
        AIR
    }

    /// THE DRY LAND FIXTURE: the stone row 9, everything else air —
    /// the ground the hull rests on, no water in sight.
    fn land_cell(x: i32, y: i32, z: i32) -> u16 {
        if x == 0 && z == 0 && y == 9 {
            return crate::world::STONE;
        }
        AIR
    }

    /// THE X WALL FIXTURE: the solid column at x=2, everything else
    /// air — the wall the x sweep's own clamp speaks.
    fn wall_cell(x: i32, _y: i32, _z: i32) -> u16 {
        if x == 2 {
            return crate::world::STONE;
        }
        AIR
    }

    /// THE Z WALL FIXTURE: the solid column at z=2 — the z sweep's own
    /// clamp speaks (the box's x span the x-resolved position carries).
    fn zwall_cell(_x: i32, _y: i32, z: i32) -> u16 {
        if z == 2 {
            return crate::world::STONE;
        }
        AIR
    }

    #[test]
    fn the_control_walks_the_bytecode_folds() {
        // THE D KEY (the right's own +1, the yaw the sum carries, the
        // single-turn's 0.005 thrust the band arms — the direction the
        // NEW yaw's sin/cos cells read)
        let mut p = HullPhys::default();
        let (yaw, pl, pr) = hull_control(
            &mut p,
            HullInput {
                left: false,
                right: true,
                up: false,
                down: false,
            },
            0.0,
        );
        assert_eq!(p.delta_rotation, 1.0, "the right's own +1");
        assert_eq!(yaw, 1.0, "the yaw the sum carries");
        assert_eq!(
            p.delta_z,
            (crate::world_model::narrow_cos(yaw * HULL_RAD) as f32 * 0.005f32) as f64,
            "the thrust the cos cell carries"
        );
        assert_eq!(
            p.delta_x,
            (crate::world_model::narrow_sin(-(yaw * HULL_RAD)) as f32 * 0.005f32) as f64,
            "the drift the sin's negated word carries"
        );
        assert!(pl && !pr, "the LEFT paddle the D key strokes (the swapped law)");
        // THE W THRUST AT YAW 0 (the table's own 0/1 cells: sin 0.0,
        // cos 1.0 — the f32 products the exact words keep)
        let mut p = HullPhys::default();
        let (yaw, pl, pr) = hull_control(
            &mut p,
            HullInput {
                left: false,
                right: false,
                up: true,
                down: false,
            },
            0.0,
        );
        assert_eq!(p.delta_x, 0.0, "the sin cell the zero yaw silences");
        assert_eq!(
            p.delta_z,
            0.04f32 as f64,
            "the forward's own 0.04 the cos 1.0 keeps (the f32's own widen)"
        );
        assert_eq!(yaw, 0.0, "the straight run the yaw never turns");
        assert_eq!(p.delta_rotation, 0.0, "the no-turn the rotation keeps");
        assert!(pl && pr, "the W the both paddles strokes");
        // THE REVERSE (the 0.005 the S subtracts)
        let mut p = HullPhys::default();
        let _ = hull_control(
            &mut p,
            HullInput {
                left: false,
                right: false,
                up: false,
                down: true,
            },
            0.0,
        );
        assert_eq!(p.delta_z, -0.005f32 as f64, "the reverse's own word");
        // THE BOTH-KEYS BRAKE (the band the right == left refuses — no
        // thrust, no turn, the paddles the silence keeps)
        let mut p = HullPhys::default();
        let (_, pl, pr) = hull_control(
            &mut p,
            HullInput {
                left: true,
                right: true,
                up: false,
                down: false,
            },
            0.0,
        );
        assert_eq!(p.delta_x + p.delta_z, 0.0, "the both-keys the thrust never arms");
        assert_eq!(p.delta_rotation, 0.0, "the ±1 the cancels keep");
        assert!(!pl && !pr, "the both-keys the paddles silence");
        // THE A KEY (the left's own −1 over the yaw 5, the RIGHT paddle
        // the swap strokes)
        let mut p = HullPhys::default();
        let (yaw, pl, pr) = hull_control(
            &mut p,
            HullInput {
                left: true,
                right: false,
                up: false,
                down: false,
            },
            5.0,
        );
        assert_eq!(p.delta_rotation, -1.0, "the left's own −1");
        assert_eq!(yaw, 4.0, "the yaw the sum carries");
        assert!(!pl && pr, "the A the right paddle strokes");
        // THE W + A TOGETHER (the band the up refuses — the 0.005 never
        // arms, the 0.04 alone speaks, the turn the ±1 keeps)
        let mut p = HullPhys::default();
        let (yaw, pl, pr) = hull_control(
            &mut p,
            HullInput {
                left: true,
                right: false,
                up: true,
                down: false,
            },
            0.0,
        );
        assert_eq!(p.delta_rotation, -1.0, "the turn the W never blocks");
        assert_eq!(yaw, -1.0, "the yaw the sum carries");
        assert_eq!(
            p.delta_z,
            (crate::world_model::narrow_cos(yaw * HULL_RAD) as f32 * 0.04f32) as f64,
            "the 0.04 alone the band's skip keeps"
        );
        assert!(pl && pr, "the W the both paddles stroke through the turn");
    }

    #[test]
    fn the_move_xz_clamps_the_walls() {
        // THE +X WALK (the wall the near plane keeps: the box's right
        // edge touches cx=2's left plane exactly — the touching
        // boundary never intersects — the delta the block zeroes)
        let mut p = HullPhys::default();
        p.delta_x = 1.5;
        let (nx, nz) = hull_move_xz(&mut p, 0.0, 10.0, 0.0, &wall_cell);
        assert_eq!(nx, 2.0 - HULL_WIDTH / 2.0, "the near plane's own clamp");
        assert_eq!(nz, 0.0, "the idle axis the pass keeps");
        assert_eq!(p.delta_x, 0.0, "the blocked axis the zero keeps");
        // THE -X WALK (the far plane plus the half width — the box's
        // left edge touches the wall's right plane exactly)
        let mut p = HullPhys::default();
        p.delta_x = -0.4;
        let (nx, _) = hull_move_xz(&mut p, 3.7, 10.0, 0.0, &wall_cell);
        assert_eq!(nx, 3.0 + HULL_WIDTH / 2.0, "the far plane's own clamp");
        assert_eq!(p.delta_x, 0.0, "the blocked axis the zero keeps");
        // THE Z SWEEP (the x the resolved position carries — the z wall
        // the box's own x span tests)
        let mut p = HullPhys::default();
        p.delta_x = 1.5;
        p.delta_z = 1.5;
        let (nx, nz) = hull_move_xz(&mut p, 0.0, 10.0, 0.0, &zwall_cell);
        assert_eq!(nx, 1.5, "the free x the pass keeps");
        assert_eq!(nz, 2.0 - HULL_WIDTH / 2.0, "the z wall's own clamp");
        assert_eq!(p.delta_z, 0.0, "the blocked z the zero keeps");
        assert_eq!(p.delta_x, 1.5, "the free x the delta keeps");
        // THE OPEN AIR (no blocker the honest pass keeps — the deltas
        // the walk never touches)
        let mut p = HullPhys::default();
        p.delta_x = 0.3;
        p.delta_z = -0.2;
        let (nx, nz) = hull_move_xz(&mut p, 0.0, 10.0, 0.0, &wall_cell);
        assert_eq!((nx, nz), (0.3, -0.2), "the open air's own pass");
        assert_eq!(p.delta_x, 0.3, "the pass the delta keeps");
        // THE DIAGONAL (the x the wall clamps, the z the free walk
        // carries — the compound's own per-axis shape)
        let mut p = HullPhys::default();
        p.delta_x = 1.5;
        p.delta_z = 0.3;
        let (nx, nz) = hull_move_xz(&mut p, 0.0, 10.0, 0.0, &wall_cell);
        assert_eq!(nx, 2.0 - HULL_WIDTH / 2.0, "the wall the x clamps");
        assert_eq!(nz, 0.3, "the z the free axis carries");
    }

    #[test]
    fn the_friction_decays_the_thrust_words() {
        // THE FLOAT'S OWN TAIL (the InAir f5 0.9 the x/z multiplies —
        // the decay the control's own add PRECEDES, the vanilla order
        // the composition keeps)
        let mut p = HullPhys::default();
        p.delta_x = 0.04;
        p.delta_z = 0.04;
        let y = hull_tick(&mut p, 5.0, 20.0, 5.0, &pool_cell);
        // THE F32'S OWN WIDENING (the Java double×float the f64 carry
        // keeps — 0.9f32's widen the exact multiplier speaks)
        let f5 = 0.9f32 as f64;
        assert_eq!(p.delta_x, 0.04 * f5, "the x the friction decays");
        assert_eq!(p.delta_z, 0.04 * f5, "the z the friction decays");
        assert_eq!(y, 20.0 - 0.04, "the air fall the y keeps");
    }

    #[test]
    fn the_lerp_axes_walk_the_vanilla_fold() {
        // THE LERPTO'S OWN THIRDS (the vanilla Entity.lerp's `/steps`
        // fold the remote bodies speak): 1/3, then 1/2 of the rest,
        // then the whole — the third step lands the word exactly, the
        // count's descent the walk keeps, the zero-steps silence the
        // exhausted walk rests at
        let (c, s) = hull_lerp_axes((0.0, 0.0, 0.0), (3.0, 6.0, 9.0), 3);
        assert_eq!(c, (1.0, 2.0, 3.0), "the first third");
        assert_eq!(s, 2, "the count's descent");
        let (c, s) = hull_lerp_axes(c, (3.0, 6.0, 9.0), s);
        assert_eq!(c, (2.0, 4.0, 6.0), "the half of the rest");
        assert_eq!(s, 1);
        let (c, s) = hull_lerp_axes(c, (3.0, 6.0, 9.0), s);
        assert_eq!(c, (3.0, 6.0, 9.0), "the last step lands the word");
        assert_eq!(s, 0);
        let (c, s) = hull_lerp_axes(c, (3.0, 6.0, 9.0), s);
        assert_eq!(c, (3.0, 6.0, 9.0), "the exhausted walk rests at its word");
        assert_eq!(s, 0, "the silence the zero keeps");
    }

    /// THE STATUS WORD — the bands the detection walks: the base
    /// band's water the IN_WATER verdict speaks (the surface the
    /// waterLevel's own word), the top band's water the UNDER verdict
    /// speaks, the ground slab's solids the ON_LAND verdict speaks
    /// (the 0.6 the vocabulary's friction keeps), the open air the
    /// IN_AIR silence keeps.
    #[test]
    fn the_hull_status_walks_the_bands() {
        // THE IN WATER: the hull's base in the water cell's span — the
        // surface the max word keeps
        let (st, wl, _f) = hull_status(
            (
                -HULL_WIDTH / 2.0,
                10.6,
                -HULL_WIDTH / 2.0,
                HULL_WIDTH / 2.0,
                10.6 + HULL_HEIGHT,
                HULL_WIDTH / 2.0,
            ),
            0.0,
            &pool_cell,
        );
        assert_eq!(st, HullStatus::InWater);
        assert_eq!(wl, 11.0, "the water cell's own top plane");
        // THE UNDER: the hull's top row covered by the water's surface
        let (st, wl, _f) = hull_status(
            (
                -HULL_WIDTH / 2.0,
                10.2,
                -HULL_WIDTH / 2.0,
                HULL_WIDTH / 2.0,
                10.2 + HULL_HEIGHT,
                HULL_WIDTH / 2.0,
            ),
            0.0,
            &pool_cell,
        );
        assert_eq!(st, HullStatus::UnderWater);
        assert_eq!(wl, 10.2 + HULL_HEIGHT, "the maxY the side effect keeps");
        // THE ON LAND: the hull resting on the dry stone row (the
        // stone the slab band counts, the friction the 0.6)
        let (st, _wl, f) = hull_status(
            (
                -HULL_WIDTH / 2.0,
                10.0,
                -HULL_WIDTH / 2.0,
                HULL_WIDTH / 2.0,
                10.0 + HULL_HEIGHT,
                HULL_WIDTH / 2.0,
            ),
            0.0,
            &land_cell,
        );
        assert_eq!(st, HullStatus::OnLand);
        assert_eq!(f, 0.6);
        // THE IN AIR: the hull high above the pool — the NaN the empty
        // count keeps (the vanilla's own 0.0/0.0)
        let (st, wl, f) = hull_status(
            (
                -HULL_WIDTH / 2.0,
                20.0,
                -HULL_WIDTH / 2.0,
                HULL_WIDTH / 2.0,
                20.0 + HULL_HEIGHT,
                HULL_WIDTH / 2.0,
            ),
            7.5,
            &pool_cell,
        );
        assert_eq!(st, HullStatus::InAir);
        assert_eq!(wl, 7.5, "the prev word the non-water arms keep");
        assert!(f.is_nan(), "the empty count's own NaN");
    }

    /// THE FLOAT'S OWN FOLDS — the bytecode's exact words: the IN_WATER
    /// lift the (delta + d2*g/0.65)*0.75 fold speaks, the air fall the
    /// -0.04 the tail adds, the splash-down's 0.101 teleport the
    /// getWaterLevelAbove fold keeps, the settle the fixed point
    /// reaches (the 0.65 submersion fraction the equilibrium speaks).
    #[test]
    fn the_hull_float_folds_the_bytecode_words() {
        // THE AIR FALL: the fresh hull high above — one tick of the
        // -0.04 the gravity's own word speaks
        let mut phys = HullPhys::default();
        let y = hull_tick(
            &mut phys,
            0.0,
            20.0,
            0.0,
            &|_x, _y, _z| AIR,
        );
        assert_eq!(y, 20.0 - 0.04);
        assert_eq!(phys.delta_y, -0.04);
        assert_eq!(phys.status, HullStatus::InAir);
        // the collided dy the checkFallDamage stores — the subtraction's
        // own word the move's double math keeps (the 19.96's
        // representation the f64 speaks)
        assert_eq!(phys.last_yd, (20.0 - 0.04) - 20.0, "the checkFallDamage's own word");
        // THE SPLASH-DOWN: the falling hull's base band catches the
        // pool's surface — the teleport the d6 fold speaks (the water
        // level above 11.0, the -bbHeight, the +0.101), the delta the
        // zero keeps, the move the return skips
        let mut phys = HullPhys {
            delta_x: 0.0,
            delta_y: -0.04,
            delta_z: 0.0,
            status: HullStatus::InAir,
            old_status: HullStatus::InAir,
            water_level: 0.0,
            last_yd: -0.04,
            delta_rotation: 0.0,
        };
        let y = hull_tick(&mut phys, 0.0, 10.9, 0.0, &pool_cell);
        assert_eq!(y, 11.0 - HULL_HEIGHT + SPLASH_DOWN, "the d6's own word");
        assert_eq!(phys.delta_y, 0.0, "the multiply(1,0,1) the arm speaks");
        assert_eq!(phys.last_yd, 0.0);
        assert_eq!(phys.status, HullStatus::InWater);
        // THE LIFT: the IN_WATER tick's exact fold — the fresh InAir
        // birth the splash-down arm keeps (the previous pin), the
        // settled hull's own walk the InWater word speaks
        let mut phys = HullPhys {
            status: HullStatus::InWater,
            old_status: HullStatus::InWater,
            ..Default::default()
        };
        let y0 = 10.6;
        let y = hull_tick(&mut phys, 0.0, y0, 0.0, &pool_cell);
        let d2 = (11.0 - y0) / HULL_HEIGHT;
        let expected = (-0.04 + d2 * HULL_GRAVITY / 0.65) * 0.75;
        assert_eq!(phys.delta_y, expected, "the lift's own fold");
        assert_eq!(y, y0 + expected);
        // THE SETTLE: the fixed point the equilibrium speaks — the
        // 0.65 submersion fraction the delta's zero keeps
        let mut phys = HullPhys::default();
        let mut y = 10.6;
        for _ in 0..400 {
            y = hull_tick(&mut phys, 0.0, y, 0.0, &pool_cell);
        }
        let y_star = 11.0 - 0.65 * HULL_HEIGHT;
        assert!(
            (y - y_star).abs() < 1.0e-4,
            "the settled y {y} vs the fixed point {y_star}"
        );
        assert_eq!(phys.status, HullStatus::InWater);
    }

    /// THE MOVE'S OWN CLAMP — the Entity.move's y axis over full
    /// cubes: the ON_LAND fall the ground's top clamps (the delta the
    /// blocked axis zeroes, the rest the boundary keeps), the IN_WATER
    /// lift the ceiling's bottom clamps (the hull the water rises, the
    /// stone the boundary keeps).
    #[test]
    fn the_hull_move_clamps_the_ground() {
        // THE GROUND REST: the hull ON the dry stone row — the -0.04
        // the gravity adds, the clamp the 10.0 the stone's top keeps,
        // the delta the zero the blocked axis speaks
        let mut phys = HullPhys::default();
        let y = hull_tick(&mut phys, 0.0, 10.0, 0.0, &land_cell);
        assert_eq!(y, 10.0, "the stone's own top plane");
        assert_eq!(phys.delta_y, 0.0);
        assert_eq!(phys.status, HullStatus::OnLand);
        assert_eq!(phys.last_yd, 0.0);
        // THE CEILING: the water row 10 under a stone lid at row 11 —
        // the IN_WATER lift the hull rises with, the stone's own
        // bottom the box's top clamps at (the boundary the rest keeps)
        let ceiling = |x: i32, y: i32, z: i32| {
            if x == 0 && z == 0 {
                if y == 11 {
                    return crate::world::STONE;
                }
                if y == 10 {
                    return WATER;
                }
            }
            AIR
        };
        let mut phys = HullPhys::default();
        let mut y = 10.5;
        for _ in 0..40 {
            y = hull_tick(&mut phys, 0.0, y, 0.0, &ceiling);
        }
        assert_eq!(y, 11.0 - HULL_HEIGHT, "the ceiling's own clamp");
        assert_eq!(phys.delta_y, 0.0);
    }
}

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
    #[expect(dead_code)] // the flowing arm the full-cube vocabulary never speaks
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
/// block — the vanilla's own empty collision shapes; the WHEAT AGE
/// LADDER joins the pass — the 15th audit + the growth round: the
/// `.noCollision()` row the Blocks clinit speaks, the hull floats
/// THROUGH the crop's every stage)
fn solid(id: u16) -> bool {
    id != AIR
        && id != WATER
        && id != LAVA
        && !crate::world::is_wheat(id)
        && !crate::world::is_flower(id)
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
    for cy in (max_y.floor() as i32)..(d2.ceil() as i32) {
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
    // friction the 0.05 the clinit's default arms — every arm of the
    // status match re-arms the friction before the common tail reads
    // it, so the local stays un-armed here)
    let mut d0 = -HULL_GRAVITY;
    let mut d2 = 0.0f64;
    let f5;

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

/// THE COLUMN'S OWN AUTHORITY STATE (the AbstractBoat fields the
/// server-side walk keeps — javap the 26.2 jar's
/// `onAboveBubbleColumn` + `tickBubbleColumn` putfields: the
/// `isAboveBubbleColumn` the fresh mouth's arm speaks, the
/// `bubbleColumnDirectionIsDown` the drag bit carries, the
/// `bubbleTime` the 60 the mouth's zero clock arms — the offline
/// hull's own words, the singleplayer's integrated server the local
/// authority stands in for).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HullBubble {
    pub bubble_time: i32,
    pub is_above: bool,
    pub down: bool,
}

/// ONE OVERLAPPED COLUMN CELL'S OWN WORDS (the entityInside branch's
/// decoded law): the drag bit the variant byte keeps (1 = the
/// whirlpool's `drag=true`, 2 = the upward's `drag=false` — the
/// [`crate::world_view::column_variant_at`] read), the MOUTH the
/// above-cell check speaks (`getCollisionShape.isEmpty() &&
/// getFluidState.isEmpty()` — the AIR the grid's honest word).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BubbleCell {
    pub drag: bool,
    pub mouth: bool,
}

/// THE LAUNCH'S OWN VERDICT (the fire tick's two words — the DOWN the
/// `add(0, −0.7, 0)` + the `ejectPassengers()` the caller folds, the
/// UP the y the `hasPassenger(lambda) ? 2.7 : 0.6` REPLACE keeps, the
/// lambda `it instanceof Player` the rider's own word speaks).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BubbleLaunch {
    Down,
    Up { passenger_player: bool },
}

/// THE BASE-TICK BEAT (javap `BubbleColumnBlock.entityInside` — the
/// direct calls the boat's checkInsideBlocks walk fires per overlapped
/// column cell, BEFORE the float's own folds the vanilla tick's
/// `super.tick()` order keeps): the MOUTH cell the above walk arms
/// (the server fields' own words — the isAbove fresh true, the down
/// the drag bit, the bubbleTime the 60 the zero clock keeps) and the
/// NON-MOUTH cells the INSIDE law speaks (the steady drag on the y —
/// [`crate::body::bubble_column_drag`]'s own body words the Entity
/// walk keeps, sequential per cell the live delta reads, the x/z the
/// laws never touch, the fall-distance reset the inside tail speaks —
/// no fall field the hull keeps, the reset the honest no-op). Returns
/// the mouth's presence — the client arm's own gate (the 1/100 splash
/// the caller folds).
pub fn hull_bubble_inside(
    phys: &mut HullPhys,
    bubble: &mut HullBubble,
    cells: &[BubbleCell],
) -> bool {
    let mut mouth = false;
    for cell in cells {
        if cell.mouth {
            mouth = true;
            bubble.is_above = true;
            bubble.down = cell.drag;
            if bubble.bubble_time == 0 {
                bubble.bubble_time = 60;
            }
        } else {
            phys.delta_y = crate::body::bubble_column_drag(phys.delta_y, cell.drag, false);
        }
    }
    mouth
}

/// THE COLUMN'S OWN TAIL (javap `tickBubbleColumn`'s authority walk —
/// the vanilla tick's LAST call, the launch the NEXT tick's move
/// consumes): the stale above-bit the silence clears (`!isAbove →
/// setBubbleTime(0)` the mouth's own absence keeps), the countdown the
/// armed clock walks (the −1 the `setBubbleTime(getBubbleTime() - 1)`
/// store speaks), the fire tick the zero crossing keeps (the
/// `60 - time - 1 > 0 && time == 0` band the bytecode's own words —
/// the zero the decrement reaches): the DOWN the `motion.add(0,
/// -0.7, 0)` word ADDS, the UP the y the player-rider's 2.7 / the
/// empty hull's 0.6 REPLACES (the x/z the words never touch), the
/// above-bit the tail's own false keeps (the next baseTick's mouth
/// re-arms the fresh word). Returns the launch's verdict — the eject
/// the DOWN arm carries.
pub fn hull_bubble_tail(
    phys: &mut HullPhys,
    bubble: &mut HullBubble,
    passenger_player: bool,
) -> Option<BubbleLaunch> {
    if !bubble.is_above {
        bubble.bubble_time = 0;
    }
    let mut fired = None;
    if bubble.bubble_time > 0 {
        bubble.bubble_time -= 1;
        let time = bubble.bubble_time;
        if 60 - time - 1 > 0 && time == 0 {
            bubble.bubble_time = 0;
            if bubble.down {
                phys.delta_y += -0.7;
                fired = Some(BubbleLaunch::Down);
            } else {
                phys.delta_y = if passenger_player { 2.7 } else { 0.6 };
                fired = Some(BubbleLaunch::Up { passenger_player });
            }
        }
    }
    bubble.is_above = false;
    fired
}

/// THE SWEPT INSIDE-BLOCKS WALK'S OWN BUDGET (the bipush 16 the outer
/// walk's own word keeps — the crossing steps the pass's segments
/// share, the `step >= budget` gate the visitor lambda speaks).
pub const INSIDE_SWEEP_BUDGET: i32 = 16;

/// ONE SEGMENT'S OWN CELLS (the per-axis yield the deflated sweep
/// walks): the moving axis' swept layers (the deflated edges at BOTH
/// segment ends the floor keeps — the layers the box's continuous
/// travel covers, the ε=1e-5 insets the static pass keeps) × the
/// fixed axes' footprint (the box at the SEGMENT END — the chained
/// segments the earlier axes' moves the fixed coords already carry;
/// the single-axis segment the fixed axes never move — the START
/// pose's footprint the same cells keep), in the travel order the
/// DDA's own crossings keep (the start pose's layers first, the end
/// pose's last). THE EXACT STEP INDICES the betweenCorners machinery
/// keeps (javap the corner-set walk): the start pose's overlapped
/// layers the ONE step-0 visit shares (the corner set the step-0 walk
/// speaks), the leading edge's crossings the +1 each — the
/// layer-batch simplification the exact indices refine (the straddled
/// start the budget's own word the share shifts). The lane's z→y→x
/// loop order the moving axis' own position respects.
fn sweep_segment_cells(
    cur: (f64, f64, f64),
    nxt: (f64, f64, f64),
    axis: usize,
) -> Vec<([i32; 3], i32)> {
    let e = 1e-5f64;
    let hw = HULL_WIDTH / 2.0;
    let (cur_a, nxt_a) = match axis {
        0 => (cur.0, nxt.0),
        1 => (cur.1, nxt.1),
        _ => (cur.2, nxt.2),
    };
    let (rel_lo, rel_hi) = match axis {
        0 | 2 => (-hw, hw),
        _ => (0.0, HULL_HEIGHT),
    };
    // THE TRAVEL ORDER (the leading edge's own walk: the ascending
    // layers the upward segment speaks, the descending the downward
    // keeps — the swept span the deflated edges at both ends cover)
    let layers: Vec<i32> = if nxt_a >= cur_a {
        (((cur_a + rel_lo + e).floor() as i32)..=((nxt_a + rel_hi - e).floor() as i32)).collect()
    } else {
        (((nxt_a + rel_lo + e).floor() as i32)..=((cur_a + rel_hi - e).floor() as i32))
            .rev()
            .collect()
    };
    // THE FIXED AXES' FOOTPRINT (the box at the segment end, the same
    // ε the static pass insets)
    let (x0, x1) = ((nxt.0 - hw + e).floor() as i32, (nxt.0 + hw - e).floor() as i32);
    let (y0, y1) = (
        (nxt.1 + e).floor() as i32,
        (nxt.1 + HULL_HEIGHT - e).floor() as i32,
    );
    let (z0, z1) = ((nxt.2 - hw + e).floor() as i32, (nxt.2 + hw - e).floor() as i32);
    // THE START CORNERS' OWN SPAN (the start pose's overlapped layers
    // — the swept span's own head BOTH directions keep: the travel
    // begins at the start pose, the first `start_count` entries the
    // layers vec carries are the corner set's own cells)
    let start_lo = (cur_a + rel_lo + e).floor() as i32;
    let start_hi = (cur_a + rel_hi - e).floor() as i32;
    let start_count = (start_hi - start_lo + 1).max(0) as usize;
    let mut out = Vec::new();
    // THE STEP THE CORNER SET SHARES (the start layers the ONE step-0
    // visit keeps, the crossings the +1 each — the exact indices the
    // budget's gate reads)
    let step_of = |i: usize| -> i32 {
        if i < start_count {
            0
        } else {
            (i - start_count + 1) as i32
        }
    };
    match axis {
        0 => {
            for (i, wx) in layers.iter().enumerate() {
                let step = step_of(i);
                for cz in z0..=z1 {
                    for cy in y0..=y1 {
                        out.push(([*wx, cy, cz], step));
                    }
                }
            }
        }
        1 => {
            for (i, wy) in layers.iter().enumerate() {
                let step = step_of(i);
                for cz in z0..=z1 {
                    for cx in x0..=x1 {
                        out.push(([cx, *wy, cz], step));
                    }
                }
            }
        }
        _ => {
            for (i, wz) in layers.iter().enumerate() {
                let step = step_of(i);
                for cy in y0..=y1 {
                    for cx in x0..=x1 {
                        out.push(([cx, cy, *wz], step));
                    }
                }
            }
        }
    }
    out
}

/// THE SWEPT INSIDE-BLOCKS WALK (javap `Entity.checkInsideBlocks(List)`
/// — the outer pass the boat's own tick runs AFTER the move: the
/// Movement's from→to the `Direction.axisStepOrder` decomposes — Y
/// ALWAYS first, then the horizontal pair the `|x| < |z|` test orders
/// (YZX when the z's absolute leads, YXZ else) —, the per-axis
/// segments the box's deflated sweep walks, the fresh set the pass's
/// own dedup keeps, the 16 the budget caps (the `step >= budget` gate
/// the visitor speaks, the counter+1 the inner walk returns), the
/// exhausted budget the FINAL degenerate walk at `to` guarantees (the
/// static box overlap the landing pose keeps). The bubble column's
/// full-block `getEntityInsideCollisionShape` shape arms the apply
/// for EVERY yielded cell — the `moved` flag the OR keeps never gates
/// the lane's own words (the launch's 2.7 jump the static pass skips
/// the swept layers fold). The degenerate segment (no displacement)
/// the static pass's own word keeps — the empty walk returns.
pub fn hull_sweep_cells(from: (f64, f64, f64), to: (f64, f64, f64)) -> Vec<[i32; 3]> {
    let d = (to.0 - from.0, to.1 - from.1, to.2 - from.2);
    if d.0 == 0.0 && d.1 == 0.0 && d.2 == 0.0 {
        return Vec::new();
    }
    // THE AXIS STEP ORDER (the |x| < |z| test the YZX order arms, the
    // YXZ the else keeps — the Y the both orders lead)
    let axes: [usize; 3] = if d.0.abs() < d.2.abs() { [1, 2, 0] } else { [1, 0, 2] };
    let mut seen: std::collections::HashSet<[i32; 3]> = std::collections::HashSet::new();
    let mut out = Vec::new();
    let mut budget = INSIDE_SWEEP_BUDGET;
    let mut cur = from;
    for axis in axes {
        let dv = match axis {
            0 => d.0,
            1 => d.1,
            _ => d.2,
        };
        if dv == 0.0 {
            continue;
        }
        let nxt = match axis {
            0 => (cur.0 + dv, cur.1, cur.2),
            1 => (cur.0, cur.1 + dv, cur.2),
            _ => (cur.0, cur.1, cur.2 + dv),
        };
        let mut max_step = -1i32;
        for (cell, step) in sweep_segment_cells(cur, nxt, axis) {
            if step >= budget {
                break;
            }
            if seen.insert(cell) {
                out.push(cell);
            }
            if step > max_step {
                max_step = step;
            }
        }
        // THE COUNTER+1 THE INNER WALK RETURNS (the AtomicInteger the
        // max step keeps — the +1 the called segment's own word adds)
        budget -= max_step.max(0) + 1;
        cur = nxt;
    }
    if budget <= 0 {
        // THE FINAL DEGENERATE WALK (the budget<=0 branch — the static
        // box overlap at `to`, the fresh 1 the call passes)
        let e = 1e-5f64;
        let hw = HULL_WIDTH / 2.0;
        let (x0, x1) = ((to.0 - hw + e).floor() as i32, (to.0 + hw - e).floor() as i32);
        let (y0, y1) = (
            (to.1 + e).floor() as i32,
            (to.1 + HULL_HEIGHT - e).floor() as i32,
        );
        let (z0, z1) = ((to.2 - hw + e).floor() as i32, (to.2 + hw - e).floor() as i32);
        for cz in z0..=z1 {
            for cy in y0..=y1 {
                for cx in x0..=x1 {
                    let cell = [cx, cy, cz];
                    if seen.insert(cell) {
                        out.push(cell);
                    }
                }
            }
        }
    }
    out
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
    fn the_sweep_walks_the_launch_layers() {
        // THE 2.7 JUMP (the launch's own displacement the static pass
        // skips): from y 20.3 to y 23.0 — the swept layers 20..=23
        // (the deflated edges at both ends the floor keeps) × the 3×3
        // footprint the 1.375-wide box keeps — the travel order the
        // ascending layers speak
        let cells = hull_sweep_cells((10.5, 20.3, 10.5), (10.5, 23.0, 10.5));
        assert_eq!(cells.len(), 36, "the 4 layers × the 3×3 footprint");
        assert_eq!(cells[0], [9, 20, 9], "the lowest layer's own first cell");
        assert_eq!(cells.last(), Some(&[11, 23, 11]), "the top layer's own last cell");
        // THE PURE VERTICAL (the x/z the zero displacement keeps — the
        // single Y segment the axis order leads)
        let mut seen = std::collections::HashSet::new();
        for c in &cells {
            assert!(seen.insert(*c), "the dup the pass's dedup keeps: {c:?}");
        }
    }

    #[test]
    fn the_sweep_decomposes_the_axis_order() {
        // THE |x| >= |z| WORD (dx 1.5, dz 0.4 → the YXZ order the else
        // keeps — the Y the zero skip, the X the first mover)
        let cells = hull_sweep_cells((10.5, 20.0, 10.5), (12.0, 20.0, 10.9));
        // THE X SEGMENT (the first mover): the x layers 9..=12 (one
        // step per layer, the footprint's cells the step's batch
        // keeps), the y/z footprint at the segment end (y 20, z 9..=11)
        assert_eq!(cells[0], [9, 20, 9], "the X segment's own first cell");
        assert_eq!(cells[3], [10, 20, 9], "the second step's own first cell");
        assert_eq!(cells[9], [12, 20, 9], "the x layers ascend 9..=12");
        // THE Z SEGMENT (the second mover): the z layers 9..=11, the x
        // footprint at the segment end 11..=12 (the earlier x cells the
        // dedup keeps silent)
        let z_cells: Vec<[i32; 3]> = cells.iter().copied().filter(|c| c[0] >= 11).collect();
        assert!(
            z_cells.iter().any(|c| *c == [12, 20, 11]),
            "the Z segment's own far corner: {z_cells:?}"
        );
        let mut seen = std::collections::HashSet::new();
        for c in &cells {
            assert!(seen.insert(*c), "the dup the dedup keeps: {c:?}");
        }
        // THE |x| < |z| WORD (dx 0.4, dz 1.5 → the YZX order arms — the
        // Z the first horizontal mover)
        let flipped = hull_sweep_cells((10.5, 20.0, 10.5), (10.9, 20.0, 12.0));
        assert_eq!(flipped[0], [9, 20, 9], "the Z segment leads the horizontal pair");
    }

    #[test]
    fn the_sweep_budgets_the_steps_and_walks_the_final_pose() {
        // THE 30-BLOCK TRAVEL (the synthetic long sweep the budget's
        // gate arms): the steps 0..=15 push (the 16 the budget keeps),
        // the step 16 the gate stops, the counter+1 the budget zeroes,
        // the FINAL DEGENERATE WALK the to-pose cells pushes
        let cells = hull_sweep_cells((10.5, 20.3, 10.5), (10.5, 50.3, 10.5));
        // the pushed: 16 layers (20..=35) × the 3×3 footprint = 144;
        // the final walk: the to-pose box (y 50 only) — 9 fresh cells
        assert_eq!(cells.len(), 144 + 9, "the budget's cap + the final walk");
        assert_eq!(cells[143], [11, 35, 11], "the last budgeted layer");
        assert!(
            cells[144..].contains(&[9, 50, 9]),
            "the final walk's own cells: {:?}",
            &cells[144..]
        );
        // THE DOWNWARD TRAVEL (the dive's own descending order — the
        // leading edge the top layers first)
        let dive = hull_sweep_cells((10.5, 23.0, 10.5), (10.5, 20.3, 10.5));
        assert_eq!(dive[0], [9, 23, 9], "the dive's own first layer (the top)");
        assert_eq!(dive.last(), Some(&[11, 20, 11]), "the dive's own last layer");
    }

    #[test]
    fn the_sweep_the_degenerate_silence_keeps() {
        // THE ZERO DISPLACEMENT (the static pass's own word — the empty
        // walk the degenerate segment returns)
        assert!(hull_sweep_cells((10.5, 20.0, 10.5), (10.5, 20.0, 10.5)).is_empty());
    }

    #[test]
    fn the_start_corners_share_the_zero_step() {
        // THE STRADDLED START (the box the boundary sits across): the
        // start pose's TWO layers the ONE step-0 visit shares (the
        // corner set the betweenCorners walk speaks), the leading
        // edge's crossing the +1 keeps — the travel order the layers
        // ascend
        let seg = sweep_segment_cells((10.5, 20.9, 10.5), (10.5, 22.3, 10.5), 1);
        assert_eq!(seg.len(), 27, "the 3 layers × the 3×3 footprint");
        assert!(
            seg[..18].iter().all(|(_, s)| *s == 0),
            "the start pose's own two layers the zero step shares: {:?}",
            &seg[..18]
        );
        assert!(
            seg[18..].iter().all(|(_, s)| *s == 1),
            "the crossing's own +1 the walk keeps"
        );
        assert_eq!(seg[0].0, [9, 20, 9], "the travel order the start keeps");
        assert_eq!(seg[18].0, [9, 22, 9], "the crossing's own layer");
        // THE DOWNWARD STRADDLE (the dive's own start the boundary sits
        // across — the start span the travel's head keeps in the
        // descending order, the descent the crossings the +1 walks)
        let dive = sweep_segment_cells((10.5, 21.9, 10.5), (10.5, 20.3, 10.5), 1);
        assert_eq!(dive.len(), 27, "the 3 layers × the 3×3 footprint");
        assert!(
            dive[..18].iter().all(|(_, s)| *s == 0),
            "the downward start's own two layers the zero step shares"
        );
        assert!(
            dive[18..].iter().all(|(_, s)| *s == 1),
            "the descent's own crossing the +1 keeps"
        );
        assert_eq!(dive[0].0, [9, 22, 9], "the descent's own first layer (the top)");
        assert_eq!(dive[18].0, [9, 20, 9], "the descent's crossing the bottom keeps");
    }

    #[test]
    fn the_start_sharing_shifts_the_budget_word() {
        // THE LONG STRADDLED TRAVEL (the 2-layer start the budget's own
        // arithmetic shifts): the step-0 batch the TWO layers carries,
        // the crossings 22..=41 the steps 1..=20 walk, the gate the
        // step 16 stops (the layers 20..=36 the 17 the batch keeps —
        // 153 cells), the counter+1 the budget zeroes, and the FINAL
        // DEGENERATE WALK the gated layers 40..=41 fresh pushes (the
        // batch shape's 162 the exact indices retire)
        let cells = hull_sweep_cells((10.5, 20.9, 10.5), (10.5, 40.9, 10.5));
        assert_eq!(cells.len(), 153 + 18, "the shared start's own budget word");
        assert_eq!(cells[152], [11, 36, 11], "the last budgeted layer");
        assert!(
            cells[153..].contains(&[9, 40, 9]),
            "the final walk's own fresh cells: {:?}",
            &cells[153..]
        );
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

    /// THE INSIDE CELLS' OWN DRAG (javap `Entity.handleOnInsideBubbleColumn`
    /// — the steady words the non-mouth cells speak): the whirlpool's
    /// `max(−0.3, y − 0.03)` floor, the upward's `min(0.7, y + 0.06)`
    /// cap, the x/z the laws never touch, the mouth cell the above walk
    /// arms (the 60 the zero clock keeps, the isAbove the fresh true,
    /// the down the drag bit — and NO drag fold the mouth's own branch
    /// speaks), the two-cell overlap the SECOND fold's live delta reads
    /// (the saturated cap the second application keeps).
    #[test]
    fn the_inside_cells_fold_the_steady_words() {
        let mut phys = HullPhys::default();
        let mut bubble = HullBubble::default();
        phys.delta_x = 0.25;
        phys.delta_z = -0.5;
        // THE WHIRLPOOL'S OWN PULL (the non-mouth cell): the first fold
        // the −0.03 word speaks, the −0.29 pose the floor's own −0.3
        // word keeps (the max the steady fold is)
        phys.delta_y = 0.0;
        let mouth = hull_bubble_inside(
            &mut phys,
            &mut bubble,
            &[BubbleCell {
                drag: true,
                mouth: false,
            }],
        );
        assert!(!mouth, "the inside cell's own silence");
        assert_eq!(phys.delta_y, -0.03, "the first fold's own word");
        phys.delta_y = -0.29;
        hull_bubble_inside(
            &mut phys,
            &mut bubble,
            &[BubbleCell {
                drag: true,
                mouth: false,
            }],
        );
        assert_eq!(phys.delta_y, -0.3, "the floor's own word");
        assert_eq!(phys.delta_x, 0.25, "the x the law never touches");
        assert_eq!(phys.delta_z, -0.5, "the z the law never touches");
        // THE UPWARD'S OWN PUSH (the cap the second fold saturates)
        let mut bubble_up = HullBubble::default();
        let mut phys_up = HullPhys::default();
        phys_up.delta_y = 0.0;
        hull_bubble_inside(
            &mut phys_up,
            &mut bubble_up,
            &[BubbleCell {
                drag: false,
                mouth: false,
            }],
        );
        assert_eq!(phys_up.delta_y, 0.06, "the push's own word");
        phys_up.delta_y = 0.69;
        hull_bubble_inside(
            &mut phys_up,
            &mut bubble_up,
            &[BubbleCell {
                drag: false,
                mouth: false,
            }],
        );
        assert_eq!(phys_up.delta_y, 0.7, "the cap's own word");
        // THE MOUTH'S OWN ARM (no drag the mouth branch speaks — the
        // above walk's words the fields keep)
        let mut phys_mouth = HullPhys::default();
        phys_mouth.delta_y = 0.4;
        let mut bubble_mouth = HullBubble::default();
        let mouth = hull_bubble_inside(
            &mut phys_mouth,
            &mut bubble_mouth,
            &[BubbleCell {
                drag: true,
                mouth: true,
            }],
        );
        assert!(mouth, "the mouth's own presence");
        assert_eq!(phys_mouth.delta_y, 0.4, "the mouth never drags");
        assert_eq!(bubble_mouth.bubble_time, 60, "the armed clock's own word");
        assert!(bubble_mouth.is_above, "the fresh above-bit");
        assert!(bubble_mouth.down, "the drag bit the mouth carries");
        // THE SEQUENTIAL WALK (the box's two cells: the mouth first the
        // z→y→x order lands, the inside cell the live delta folds — the
        // mouth's arm the drag's fold never overwrites)
        let mut bubble_pair = HullBubble::default();
        let mouth = hull_bubble_inside(
            &mut phys_mouth,
            &mut bubble_pair,
            &[
                BubbleCell {
                    drag: true,
                    mouth: true,
                },
                BubbleCell {
                    drag: false,
                    mouth: false,
                },
            ],
        );
        assert!(mouth);
        assert_eq!(bubble_pair.bubble_time, 60);
        assert_eq!(phys_mouth.delta_y, 0.46, "the upward fold the live delta reads");
    }

    /// THE TAIL'S OWN WALK (javap `tickBubbleColumn`'s authority branch —
    /// the countdown the armed clock walks, the fire tick the zero
    /// crossing keeps): the DOWN launch the `add(0, −0.7, 0)` word
    /// speaks, the UP the y the 2.7/0.6 REPLACE keeps, the stale
    /// above-bit the clock's reset keeps, the above-bit the tail's own
    /// false clears, the re-arm the fresh mouth's 60 speaks.
    #[test]
    fn the_tail_counts_the_sixty_clock() {
        let mut phys = HullPhys::default();
        let mut bubble = HullBubble::default();
        // THE ARM + THE COUNTDOWN (the mouth's words the inside walk
        // keeps, the tail's −1 the same tick's own decrement speaks)
        hull_bubble_inside(
            &mut phys,
            &mut bubble,
            &[BubbleCell {
                drag: true,
                mouth: true,
            }],
        );
        assert!(hull_bubble_tail(&mut phys, &mut bubble, false).is_none());
        assert_eq!(bubble.bubble_time, 59, "the first decrement's own word");
        // THE RIDE'S OWN COUNT (59 more tails the zero crossing reaches
        // — the 60th tick of riding the launch fires)
        for _ in 0..58 {
            bubble.is_above = true; // the fresh mouth's own re-arm bit
            assert!(hull_bubble_tail(&mut phys, &mut bubble, false).is_none());
        }
        assert_eq!(bubble.bubble_time, 1, "the last silent tick's own word");
        bubble.is_above = true;
        phys.delta_y = 0.2;
        let fired = hull_bubble_tail(&mut phys, &mut bubble, false);
        assert_eq!(fired, Some(BubbleLaunch::Down), "the DOWN verdict");
        assert_eq!(phys.delta_y, 0.2 - 0.7, "the ADD word the DOWN keeps");
        assert_eq!(bubble.bubble_time, 0, "the fired clock's own rest");
        assert!(!bubble.is_above, "the tail's own clear");
        // THE STALE SILENCE (no mouth the next tail resets — the clock
        // the absence keeps at zero, no re-fire the silence speaks)
        assert!(hull_bubble_tail(&mut phys, &mut bubble, false).is_none());
        assert_eq!(bubble.bubble_time, 0);
        // THE UP'S OWN REPLACE (the y the verdict REPLACES — the
        // pre-fire word irrelevant, the x/z untouched; the rider's own
        // 2.7 the player predicate speaks, the empty hull's 0.6)
        let mut bubble_up = HullBubble::default();
        let mut phys_up = HullPhys::default();
        phys_up.delta_y = -0.25;
        hull_bubble_inside(
            &mut phys_up,
            &mut bubble_up,
            &[BubbleCell {
                drag: false,
                mouth: true,
            }],
        );
        for _ in 0..59 {
            bubble_up.is_above = true;
            assert!(hull_bubble_tail(&mut phys_up, &mut bubble_up, false).is_none());
        }
        bubble_up.is_above = true;
        phys_up.delta_x = 0.1;
        let fired = hull_bubble_tail(&mut phys_up, &mut bubble_up, true);
        assert_eq!(
            fired,
            Some(BubbleLaunch::Up { passenger_player: true }),
            "the UP verdict"
        );
        assert_eq!(phys_up.delta_y, 2.7, "the REPLACE word the ridden launch keeps");
        assert_eq!(phys_up.delta_x, 0.1, "the x the launch never touches");
        // THE EMPTY HULL'S OWN WORD (the predicate's false the 0.6 speaks)
        let mut bubble_empty = HullBubble::default();
        let mut phys_empty = HullPhys::default();
        hull_bubble_inside(
            &mut phys_empty,
            &mut bubble_empty,
            &[BubbleCell {
                drag: false,
                mouth: true,
            }],
        );
        for _ in 0..59 {
            bubble_empty.is_above = true;
            assert!(hull_bubble_tail(&mut phys_empty, &mut bubble_empty, false).is_none());
        }
        bubble_empty.is_above = true;
        let fired = hull_bubble_tail(&mut phys_empty, &mut bubble_empty, false);
        assert_eq!(fired, Some(BubbleLaunch::Up { passenger_player: false }));
        assert_eq!(phys_empty.delta_y, 0.6, "the empty launch's own word");
    }
}

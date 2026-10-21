//! THE MINECART'S OWN RAIL WALK — javap NewMinecartBehavior's
//! authoritative lane over the 26.2 jar (pinned sha1 2dc72797): the
//! cart the rails CARRY — the exits table, the per-cell step, the
//! slope's own push, the halt/boost pair the powered rail wears, the
//! max-speed fraction, the natural slowdown — byte-for-byte the
//! bytecode's arithmetic. The cart that owns this lane MOVES like the
//! original moves — the same 8/20 fraction, the same 0.06 boost, the
//! same √2 slope stretch, the same V-rest.
//!
//! Pinned sources (the class -> the law):
//! - `AbstractMinecart.lambda$static$0` — the EXITS table: every
//!   RailShape the Pair<Vec3i, Vec3i> of its two exit directions
//!   speaks (the ascents the below-Vec3i the LOW side wears).
//! - `AbstractMinecart.getDefaultGravity` — the dry arm's 0.04 (the
//!   water arm's own branch the isInWater walk keeps).
//! - `NewMinecartBehavior.getMaxSpeed` — the gamerule
//!   MAX_MINECART_SPEED (default 8) * (water? 0.5 : 1.0) / 20.0 — the
//!   0.4 blocks-per-tick flat cap the vanilla cart cruises.
//! - `NewMinecartBehavior.getSlowdownFactor` — ridden 0.997 / empty
//!   0.975; `AbstractMinecart.applyNaturalSlowdown` — the multiply
//!   (f, 0.0, f) the Y component zeroes (the rail walk owns the y),
//!   the water's extra *0.95.
//! - `NewMinecartBehavior.calculateSlopeSpeed` — the downhill push
//!   g = max(0.0078125, |motion|.horizontalDistance() * 0.02) (water
//!   *0.2), the axis arm the shape's low side wears.
//! - `NewMinecartBehavior.calculateHaltTrackSpeed` — the unpowered
//!   powered rail's brake: |motion| < 0.03 -> zero, else * 0.5.
//! - `NewMinecartBehavior.calculateBoostTrackSpeed` — the powered
//!   rail's +0.06 the moving cart wears (the |motion| > 0.01 arm; the
//!   stalled launch arm the getRedstoneDirection law owns — the v2
//!   round's bank).
//! - `NewMinecartBehavior.stepAlongTrack` — the per-cell walk: the
//!   favored exit the motion's dot picks, the target the bottom
//!   center + half-exit + 0.1 rides, the slope's √2 stretch, the
//!   target-reached clamp, the y-follow the descent wears, the V-rest.
//! - `NewMinecartBehavior.restAtVShape` — the |delta|² < 0.005 + the
//!   slope's own isSlope + the descend pair the V law keeps.
//! - `NewMinecartBehavior.adjustToRails` — the yaw the favored exit's
//!   atan2 speaks: 128.0 − atan2(z, x)·180/π (the flipped cart's own
//!   +128 the isFlipped arm adds).
//! - `NewMinecartBehavior$TrackIteration.shouldIterate` — first
//!   iteration || movementLeft > 1.0E-5 (the stepAlongTrack epsilon
//!   the ldc2_w 1.0E-5f family speaks).
//!
//! THE V1 SCOPE the round banks: the rail walk's own laws (exits,
//! step, speeds, halt/boost, slope, V-rest, y-follow, yaw stamp, the
//! off-rail gravity fall the getDefaultGravity default wears). The v2
//! round's candidates: the collision-carrying move (the Entity.move
//! fold), the stalled-launch arm (getRedstoneDirection), the
//! adjustToRails' position projection, the getCurrentBlockPosOrRailBelow
//! hang-off arm, the riding mount/dismount lane, the entity-sheet
//! render.

use crate::world::{is_rail, World, RAIL_SHAPE_MASK, POWERED_RAIL, RAIL, RAIL_POWERED_BIT, WATER};

/// THE CART'S OWN BOX (the vanilla EntityType.MINECART constants the
/// boat-dims precedent bakes — 0.98 wide × 0.7 tall).
pub const CART_WIDTH: f32 = 0.98;
pub const CART_HEIGHT: f32 = 0.7;

/// THE DEFAULT GRAVITY (the AbstractMinecart.getDefaultGravity dry
/// arm's own ldc2_w — the fall the off-rail branch wears).
pub const CART_GRAVITY: f64 = 0.04;

/// THE WALK'S OWN EPSILON (the ldc2_w 1.0E-5f family — the
/// stepAlongTrack's distance floor, the stall law's motion floor, the
/// shouldIterate's budget floor; the float-widened exact value).
pub const STEP_EPS: f64 = 1.0e-5f32 as f64;

/// THE RAIL RIDE HEIGHT (the stepAlongTrack pc 164's own ldc2_w — the
/// 0.1 the target rides above the rail cell's bottom center).
pub const RAIL_RIDE_HEIGHT: f64 = 0.1;

/// THE HALF-EXIT SCALE (the stepAlongTrack pc 101's own #259 — the
/// half a block the exit offset walks).
pub const EXIT_SCALE: f64 = 0.5;

/// THE SLOPE PUSH'S OWN PAIR (the calculateSlopeSpeed's floor
/// 0.0078125 and the |motion|·0.02 factor — the max() the bytecode
/// speaks; the water arm's own 0.2 multiplier).
pub const SLOPE_GRAVITY_FLOOR: f64 = 0.0078125;
pub const SLOPE_ACCEL: f64 = 0.02;
pub const SLOPE_WATER_MULT: f64 = 0.2;

/// THE MAX-SPEED FRACTION (the getMaxSpeed walk: the gamerule default
/// 8, the water's own 0.5, the /20.0 the bytecode divides).
pub const MAX_MINECART_SPEED_GAMERULE: f64 = 8.0;
pub const MAX_SPEED_WATER_MULT: f64 = 0.5;
pub const MAX_SPEED_DIVISOR: f64 = 20.0;

/// THE NATURAL SLOWDOWN PAIR (the getSlowdownFactor's own ldc2_w pair
/// — the ridden cart's 0.997 the isVehicle arm keeps, the empty cart's
/// 0.975) and the water's extra 0.95 (the applyNaturalSlowdown tail).
pub const SLOWDOWN_RIDDEN: f64 = 0.997;
pub const SLOWDOWN_EMPTY: f64 = 0.975;
pub const SLOWDOWN_WATER_MULT: f64 = 0.949999988079071;

/// THE HALT PAIR (the calculateHaltTrackSpeed's own floor 0.03 — the
/// |motion| under which the unpowered powered rail zeroes — and the
/// *0.5 the alive motion halves by).
pub const HALT_SPEED_FLOOR: f64 = 0.03;
pub const HALT_BRAKE: f64 = 0.5;

/// THE BOOST PAIR (the calculateBoostTrackSpeed's own 0.06 the moving
/// cart's speed gains and the 0.01 the stalled arm's floor wears).
pub const BOOST_ACCEL: f64 = 0.06;
pub const BOOST_STALLED_FLOOR: f64 = 0.01;

/// THE V-REST'S OWN SPEED FLOOR (the restAtVShape pc 7's own #566 —
/// the |delta|² under which the V law may zero the motion).
pub const V_REST_SPEED_SQR_FLOOR: f64 = 0.005;

/// THE SLOPE'S OWN STRETCH (the Mth.SQRT_OF_TWO the f2d widens — the
/// 1:1 slope's diagonal the step distance wears).
pub const SQRT_OF_TWO: f64 = 1.4142135f32 as f64;

/// THE YAW STAMP'S OWN BASE (the adjustToRails pc 153's own #285 —
/// the 128.0f the atan2 walk subtracts from, the flipped arm adds).
pub const YAW_BASE: f32 = 128.0;

pub type V3 = [f64; 3];

#[inline]
fn v_add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn v_sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn v_scale(a: V3, s: f64) -> V3 {
    [a[0] * s, a[1] * s, a[2] * s]
}

#[inline]
fn v_dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn v_len(a: V3) -> f64 {
    v_dot(a, a).sqrt()
}

#[inline]
fn v_horizontal(a: V3) -> V3 {
    [a[0], 0.0, a[2]]
}

#[inline]
fn v_horizontal_distance(a: V3) -> f64 {
    (a[0] * a[0] + a[2] * a[2]).sqrt()
}

#[inline]
fn v_length_sqr(a: V3) -> f64 {
    v_dot(a, a)
}

#[inline]
fn v_normalize(a: V3) -> V3 {
    let l = v_len(a);
    if l == 0.0 {
        [0.0, 0.0, 0.0]
    } else {
        v_scale(a, 1.0 / l)
    }
}

#[inline]
fn v_dist_sqr(a: V3, b: V3) -> f64 {
    v_length_sqr(v_sub(a, b))
}

/// THE EXITS TABLE (the lambda$static$0's own pairs — the Vec3i the
/// Direction.getUnitVec3i speaks, the ascents' LOW side the .below()
/// arm wears). The shape byte the world's RAIL_SHAPE_MASK nibble
/// keeps (the RailShape ordinals 0..9 the world.rs banks).
///
///   W = (−1,0,0)  E = (1,0,0)  N = (0,0,−1)  S = (0,0,1)
fn rail_exits(shape: u8) -> (V3, V3) {
    let w = [-1.0, 0.0, 0.0];
    let e = [1.0, 0.0, 0.0];
    let n = [0.0, 0.0, -1.0];
    let s = [0.0, 0.0, 1.0];
    let wb = [-1.0, -1.0, 0.0];
    let eb = [1.0, -1.0, 0.0];
    let nb = [0.0, -1.0, -1.0];
    let sb = [0.0, -1.0, 1.0];
    match shape {
        0 => (n, s),    // NORTH_SOUTH
        1 => (w, e),    // EAST_WEST
        2 => (wb, e),   // ASCENDING_EAST
        3 => (w, eb),   // ASCENDING_WEST
        4 => (n, sb),   // ASCENDING_NORTH
        5 => (nb, s),   // ASCENDING_SOUTH
        6 => (s, e),    // SOUTH_EAST
        7 => (s, w),    // SOUTH_WEST
        8 => (n, w),    // NORTH_WEST
        9 => (n, e),    // NORTH_EAST
        _ => (n, s),
    }
}

/// THE DESCENT VERDICT (the isDecending tableswitch — the four
/// ascending shapes the motion's axis sign reads: the cart moving
/// toward the shape's LOW side descends; every other shape never).
pub fn is_descending(motion: V3, shape: u8) -> bool {
    match shape {
        2 => motion[0] < 0.0, // ASCENDING_EAST: the west exit below
        3 => motion[0] > 0.0, // ASCENDING_WEST: the east exit below
        4 => motion[2] > 0.0, // ASCENDING_NORTH: the south exit below
        5 => motion[2] < 0.0, // ASCENDING_SOUTH: the north exit below
        _ => false,
    }
}

/// THE SLOPE'S OWN PUSH (the calculateSlopeSpeed verbatim): the
/// gravity floor the max() keeps, the |motion| factor the horizontal
/// distance scales, the water's own 0.2, the axis arm the shape's LOW
/// side pushes (the downhill direction the motion gains).
pub fn slope_speed(motion: V3, shape: u8, in_water: bool) -> V3 {
    let mut g = SLOPE_GRAVITY_FLOOR.max(v_horizontal_distance(motion) * SLOPE_ACCEL);
    if in_water {
        g *= SLOPE_WATER_MULT;
    }
    match shape {
        2 => [motion[0] - g, motion[1], motion[2]],
        3 => [motion[0] + g, motion[1], motion[2]],
        4 => [motion[0], motion[1], motion[2] + g],
        5 => [motion[0], motion[1], motion[2] - g],
        _ => motion,
    }
}

/// THE HALT LAW (the calculateHaltTrackSpeed verbatim): the powered
/// rail the boost lane owns — the walk never touches it; the
/// unpowered powered rail the brake wears: under 0.03 the motion
/// zeroes, alive the motion halves.
pub fn halt_speed(motion: V3, state_is_powered_rail: bool, state_powered: bool) -> V3 {
    // pc 0: the OTHER RAILS the walk never brakes — the plain rail
    // the motion keeps.
    if !state_is_powered_rail {
        return motion;
    }
    // pc 26: the POWERED rail the boost lane owns — the walk never
    // touches it.
    if state_powered {
        return motion;
    }
    // pc 28-50: the UNPOWERED powered rail the brake wears — under
    // 0.03 the zero, alive the *0.5.
    if v_len(motion) < HALT_SPEED_FLOOR {
        return [0.0, 0.0, 0.0];
    }
    v_scale(motion, HALT_BRAKE)
}

/// THE BOOST LAW (the calculateBoostTrackSpeed's moving arm verbatim):
/// the powered rail the speed gains +0.06 by; the stalled cart (the
/// |motion| <= 0.01 arm) the getRedstoneDirection launch owns — the
/// v2 round's bank the walk never serves.
pub fn boost_speed(motion: V3, state_is_powered_rail: bool, state_powered: bool) -> V3 {
    if !state_is_powered_rail || !state_powered {
        return motion;
    }
    let len = v_len(motion);
    if len > BOOST_STALLED_FLOOR {
        return v_scale(v_normalize(motion), len + BOOST_ACCEL);
    }
    motion
}

/// THE MAX SPEED (the getMaxSpeed verbatim): the gamerule fraction —
/// 8 * (water? 0.5 : 1.0) / 20.0 — the 0.4 blocks-per-tick the flat
/// cap speaks.
pub fn max_speed(in_water: bool) -> f64 {
    MAX_MINECART_SPEED_GAMERULE * (if in_water { MAX_SPEED_WATER_MULT } else { 1.0 })
        / MAX_SPEED_DIVISOR
}

/// THE NATURAL SLOWDOWN (the applyNaturalSlowdown verbatim): the
/// multiply (f, 0.0, f) — the Y component zeroes (the rail walk owns
/// the y), the horizontal the factor scales; the water the extra 0.95
/// multiplies.
pub fn natural_slowdown(motion: V3, ridden: bool, in_water: bool) -> V3 {
    let f = if ridden { SLOWDOWN_RIDDEN } else { SLOWDOWN_EMPTY };
    let mut out = [motion[0] * f, 0.0, motion[2] * f];
    if in_water {
        out = v_scale(out, SLOWDOWN_WATER_MULT);
    }
    out
}

/// THE V-REST (the restAtVShape verbatim): the slow cart (the
/// |delta|² < 0.005) that rolls downhill on the CURRENT shape onto a
/// slope whose own descent the motion never continues — the motion
/// zeroes, the walk rests.
pub fn rest_at_v_shape(delta: V3, current_shape: u8, next_shape: u8) -> bool {
    if v_length_sqr(delta) >= V_REST_SPEED_SQR_FLOOR {
        return false;
    }
    if !is_slope(next_shape) {
        return false;
    }
    if !is_descending(delta, current_shape) {
        return false;
    }
    if is_descending(delta, next_shape) {
        return false;
    }
    true
}

/// THE SLOPE VERDICT (the RailShape.isSlope law — the ascents alone).
pub fn is_slope(shape: u8) -> bool {
    (2..=5).contains(&shape)
}

/// THE CART'S OWN STATE (the offline entity the App's tick rides —
/// the fields the wire's snapshots will own in the v2 round).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Minecart {
    /// The entity's own position (the eye of the walk — the vanilla
    /// position() the step targets).
    pub pos: V3,
    /// The deltaMovement the speeds fold.
    pub delta: V3,
    /// The yaw the adjustToRails stamp speaks (degrees).
    pub yaw_deg: f32,
    /// The on-rails flag the transition arms.
    pub on_rails: bool,
    /// The isVehicle fold (the ridden cart's own slowdown arm).
    pub ridden: bool,
}

impl Minecart {
    pub fn new(pos: V3) -> Self {
        Minecart {
            pos,
            delta: [0.0, 0.0, 0.0],
            yaw_deg: 0.0,
            on_rails: false,
            ridden: false,
        }
    }
}

/// THE RAIL'S OWN KIND CHECK (the isRail law the world.rs banks — the
/// 764..767 band the plain/powered/detector/activator family wears).
#[inline]
fn rail_shape_of(world: &World, x: i32, y: i32, z: i32) -> Option<u8> {
    if !is_rail(world.get(x, y, z)) {
        return None;
    }
    Some(world.variant(x, y, z) & RAIL_SHAPE_MASK)
}

#[inline]
fn in_water_at(world: &World, pos: V3) -> bool {
    world.get(
        pos[0].floor() as i32,
        pos[1].floor() as i32,
        pos[2].floor() as i32,
    ) == WATER
}

/// THE FAVORED EXIT (the stepAlongTrack pc 132-152 walk — the
/// half-exit whose dot with the motion wins; the tie the FIRST exit
/// keeps).
fn favored_half_exit(motion_h: V3, shape: u8) -> V3 {
    let (first, second) = rail_exits(shape);
    let half1 = v_horizontal(v_scale(first, EXIT_SCALE));
    let half2 = v_horizontal(v_scale(second, EXIT_SCALE));
    if v_dot(motion_h, half1) < v_dot(motion_h, half2) {
        half2
    } else {
        half1
    }
}

/// THE PER-CELL STEP (the stepAlongTrack verbatim): the remaining
/// budget in, the motion's favored exit picks the target, the √2 the
/// slopes stretch, the reached clamp the budget spends, the y-follow
/// the descent wears, the V-rest the slope pair keeps. Returns the
/// remaining distance the loop still owns.
fn step_along_track(world: &World, cart: &mut Minecart, rail_pos: (i32, i32, i32), shape: u8, mut distance_left: f64) -> f64 {
    if distance_left < STEP_EPS {
        return 0.0;
    }
    let horizontal = v_horizontal(cart.delta);
    if v_len(horizontal) < STEP_EPS {
        cart.delta = [0.0, 0.0, 0.0];
        return 0.0;
    }
    let is_slope_rail = is_slope(shape);
    // THE FAVORED HALF-EXIT (the dot walk — the half-exit the motion
    // favors; the tie the first keeps).
    let half_exit = favored_half_exit(horizontal, shape);
    // THE TARGET (the atBottomCenterOf law — the cell's own CENTER
    // (x+0.5, y, z+0.5) + the favored half-exit + the 0.1 ride height
    // + a hair further along the exit — the next cell's edge the
    // boundary the target rides at).
    let bottom_center = [
        rail_pos.0 as f64 + 0.5,
        rail_pos.1 as f64,
        rail_pos.2 as f64 + 0.5,
    ];
    let mut target = bottom_center;
    target = v_add(target, half_exit);
    target = v_add(target, [0.0, RAIL_RIDE_HEIGHT, 0.0]);
    target = v_add(target, v_scale(v_normalize(half_exit), STEP_EPS));
    // THE CLIMB ARM (the isSlope && !isDecending walk — the target one
    // block higher the uphill cart wears).
    if is_slope_rail && !is_descending(horizontal, shape) {
        target = v_add(target, [0.0, 1.0, 0.0]);
    }
    // THE STEP VECTOR (the direction to the target the horizontal
    // magnitude keeps).
    let dir = v_normalize(v_sub(target, cart.pos));
    let dir_h = v_horizontal_distance(dir);
    let step_dir = if dir_h == 0.0 {
        dir
    } else {
        v_scale(dir, v_len(horizontal) / dir_h)
    };
    // THE NEXT POSITION (the budget the slope's √2 stretches).
    let stretch = if is_slope_rail { SQRT_OF_TWO } else { 1.0 };
    let mut next = v_add(cart.pos, v_scale(v_normalize(step_dir), distance_left * stretch));
    // THE REACHED CLAMP (the distanceToSqr walk — the target inside
    // the step's reach the budget spends and the position takes).
    if v_dist_sqr(cart.pos, target) <= v_dist_sqr(cart.pos, next) {
        distance_left = v_horizontal_distance(v_sub(target, next));
        next = target;
    } else {
        distance_left = 0.0;
    }
    // THE MOVE (the MoverType.SELF walk — the v1 the collision-free
    // direct fold the rail cells' own clearance serves).
    cart.pos = v_add(cart.pos, v_sub(next, cart.pos));
    // THE SLOPE'S OWN TAIL: the V-rest the next rail's shape keeps,
    // the y-follow the descent wears.
    if is_slope_rail {
        let next_cell = (
            cart.pos[0].floor() as i32,
            cart.pos[1].floor() as i32,
            cart.pos[2].floor() as i32,
        );
        if let Some(next_shape) = rail_shape_of(world, next_cell.0, next_cell.1, next_cell.2) {
            if rest_at_v_shape(cart.delta, shape, next_shape) {
                cart.delta = [0.0, 0.0, 0.0];
                return 0.0;
            }
        }
        // THE Y-FOLLOW (the pc 389-462 walk): the run the horizontal
        // distance keeps, the descent the + arm wears, the snap the
        // raise-only law keeps.
        let run = v_horizontal_distance(v_sub(target, cart.pos));
        let y_snap = target[1]
            + if is_descending(horizontal, shape) {
                run
            } else {
                -run
            };
        if cart.pos[1] < y_snap {
            cart.pos[1] = y_snap;
        }
    }
    cart.delta = step_dir;
    distance_left
}

/// THE YAW STAMP (the adjustToRails pc 153-201 verbatim): the favored
/// exit's atan2 the 128.0 base subtracts, the flipped cart's own +128
/// the isFlipped arm adds (the v1 the unflipped cart serves).
pub fn adjust_to_rails_yaw(shape: u8, delta: V3, flipped: bool) -> f32 {
    let (first, second) = rail_exits(shape);
    let half1 = v_horizontal(v_scale(first, EXIT_SCALE));
    let half2 = v_horizontal(v_scale(second, EXIT_SCALE));
    let mut favored = half2;
    if v_len(delta) > STEP_EPS {
        if v_dot(delta, half1) >= v_dot(delta, half2) && !is_descending(delta, shape) {
            favored = half1;
        }
    } else {
        favored = half1;
    }
    let mut yaw = YAW_BASE - ((favored[2]).atan2(favored[0]) * 180.0 / std::f64::consts::PI) as f32;
    if flipped {
        yaw += YAW_BASE;
    }
    yaw
}

/// THE CART'S OWN TICK (the moveAlongTrack verbatim): the iteration
/// the budget keeps, the rail the shape speaks, the speed chain the
/// calculateTrackSpeed orders (slope -> halt -> slowdown+cap ->
/// boost), the step the budget spends, the off-rail the gravity fall
/// wears.
pub fn cart_tick(world: &World, cart: &mut Minecart) {
    let in_water = in_water_at(world, cart.pos);
    let mut first_iteration = true;
    let mut movement_left: f64 = 0.0;
    let mut has_gained_slope_speed = false;
    let mut has_halted = false;
    let mut has_boosted = false;
    loop {
        if !(first_iteration || movement_left > STEP_EPS) {
            break;
        }
        // THE RAIL UNDER THE CART (the getCurrentBlockPosOrRailBelow's
        // containing arm — the hang-off walk the v2 round banks).
        let cx = cart.pos[0].floor() as i32;
        let cy = cart.pos[1].floor() as i32;
        let cz = cart.pos[2].floor() as i32;
        let shape = rail_shape_of(world, cx, cy, cz);
        match shape {
            Some(shape) => {
                if !cart.on_rails {
                    cart.on_rails = true;
                    cart.yaw_deg = adjust_to_rails_yaw(shape, cart.delta, false);
                }
                // THE SPEED CHAIN (the calculateTrackSpeed verbatim —
                // the flags the once-per-tick arms wear).
                let mut delta = cart.delta;
                if !has_gained_slope_speed {
                    let s = slope_speed(delta, shape, in_water);
                    if s != delta {
                        has_gained_slope_speed = true;
                        delta = s;
                    }
                }
                if !has_halted {
                    let id = world.get(cx, cy, cz);
                    let powered = (world.variant(cx, cy, cz) & RAIL_POWERED_BIT) != 0;
                    let s = halt_speed(delta, id == POWERED_RAIL, powered);
                    if s != delta {
                        has_halted = true;
                        delta = s;
                    }
                }
                if first_iteration {
                    delta = natural_slowdown(delta, cart.ridden, in_water);
                    let cap = max_speed(in_water);
                    if v_length_sqr(delta) > 0.0 {
                        let c = v_len(delta).min(cap);
                        delta = v_scale(v_normalize(delta), c);
                    }
                }
                if !has_boosted {
                    let id = world.get(cx, cy, cz);
                    let powered = (world.variant(cx, cy, cz) & RAIL_POWERED_BIT) != 0;
                    let s = boost_speed(delta, id == POWERED_RAIL, powered);
                    if s != delta {
                        has_boosted = true;
                        delta = s;
                    }
                }
                if first_iteration {
                    movement_left = v_horizontal_distance(delta);
                }
                cart.delta = delta;
                movement_left =
                    step_along_track(world, cart, (cx, cy, cz), shape, movement_left);
            }
            None => {
                // THE OFF-RAIL FALL (the comeOffTrack branch's v1
                // honest fold — the default gravity the
                // getDefaultGravity dry arm wears, the motion the
                // position carries).
                cart.on_rails = false;
                cart.delta[1] -= CART_GRAVITY;
                cart.pos = v_add(cart.pos, cart.delta);
                movement_left = 0.0;
            }
        }
        first_iteration = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// THE WORLD THE CART WALKS (the flat test yard: the rails the
    /// cells carry, the y the cart rides — the world's own seed the
    /// terrain never touches, the cells the walk reads set explicitly).
    fn yard() -> World {
        World::new(1, 0xCA2Cu64)
    }

    // THE YARD'S OWN FLOOR (the world's CHUNK_Y=48 the walk stays
    // under — the y the tests' rails wear).
    const Y: i32 = 20;

    #[test]
    fn the_exits_table_the_railshapes_own_pairs_wear() {
        // THE lambda$static$0's own rows — the Pair the EXITS map
        // banks, the ascents the LOW side's below-Vec3i wears.
        let rows: [(u8, V3, V3); 10] = [
            (0, [0.0, 0.0, -1.0], [0.0, 0.0, 1.0]),    // NORTH_SOUTH: N, S
            (1, [-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),    // EAST_WEST: W, E
            (2, [-1.0, -1.0, 0.0], [1.0, 0.0, 0.0]),   // ASCENDING_EAST: Wbelow, E
            (3, [-1.0, 0.0, 0.0], [1.0, -1.0, 0.0]),   // ASCENDING_WEST: W, Ebelow
            (4, [0.0, 0.0, -1.0], [0.0, -1.0, 1.0]),   // ASCENDING_NORTH: N, Sbelow
            (5, [0.0, -1.0, -1.0], [0.0, 0.0, 1.0]),   // ASCENDING_SOUTH: Nbelow, S
            (6, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),     // SOUTH_EAST: S, E
            (7, [0.0, 0.0, 1.0], [-1.0, 0.0, 0.0]),    // SOUTH_WEST: S, W
            (8, [0.0, 0.0, -1.0], [-1.0, 0.0, 0.0]),   // NORTH_WEST: N, W
            (9, [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),    // NORTH_EAST: N, E
        ];
        for (shape, first, second) in rows {
            let (f, s) = rail_exits(shape);
            assert_eq!(f, first, "shape {shape} first exit");
            assert_eq!(s, second, "shape {shape} second exit");
        }
    }

    #[test]
    fn the_max_speed_the_gamerule_fraction_wears() {
        // THE getMaxSpeed walk: 8 * 1.0 / 20.0 = 0.4 the flat cap, the
        // water's own 0.5 the 0.2 the wet cap speaks.
        assert_eq!(max_speed(false), 0.4);
        assert_eq!(max_speed(true), 0.4 * MAX_SPEED_WATER_MULT);
    }

    #[test]
    fn the_natural_slowdown_the_factors_and_the_y_zero_wear() {
        // THE applyNaturalSlowdown: the multiply (f, 0.0, f) — the Y
        // component zeroes, the horizontal the factor scales; the
        // water the extra 0.95 wears.
        let delta = [0.4, 0.7, 0.2];
        let empty = natural_slowdown(delta, false, false);
        assert_eq!(empty[0], 0.4 * SLOWDOWN_EMPTY);
        assert_eq!(empty[1], 0.0);
        assert_eq!(empty[2], 0.2 * SLOWDOWN_EMPTY);
        let ridden = natural_slowdown(delta, true, false);
        assert_eq!(ridden[0], 0.4 * SLOWDOWN_RIDDEN);
        let wet = natural_slowdown([0.4, 0.0, 0.0], false, true);
        assert_eq!(wet[0], 0.4 * SLOWDOWN_EMPTY * SLOWDOWN_WATER_MULT);
    }

    #[test]
    fn the_halt_law_the_unpowered_rail_brakes_and_floors() {
        // THE calculateHaltTrackSpeed: the powered rail the boost owns
        // (the walk never touches it), the unpowered the *0.5 the
        // alive motion wears, under 0.03 the zero the floor speaks.
        let alive = [0.4, 0.0, 0.0];
        assert_eq!(halt_speed(alive, true, true), alive);
        assert_eq!(halt_speed(alive, true, false), [0.2, 0.0, 0.0]);
        let braking = halt_speed([0.2, 0.0, 0.0], true, false);
        assert_eq!(braking, [0.1, 0.0, 0.0]);
        let near = halt_speed([0.029, 0.0, 0.0], true, false);
        assert_eq!(near, [0.0, 0.0, 0.0]);
        // THE OTHER RAILS the walk never brakes (the plain rail the
        // motion keeps — the halt the powered-rail family alone wears).
        assert_eq!(halt_speed(alive, false, false), alive);
    }

    #[test]
    fn the_boost_law_the_powered_rail_adds_the_six_hundredths() {
        // THE calculateBoostTrackSpeed's moving arm: the powered rail
        // the |motion| + 0.06 the direction keeps, the unpowered the
        // motion never touches, the stalled arm the v2 law owns.
        let moving = [0.4, 0.0, 0.0];
        assert_eq!(boost_speed(moving, true, true), [0.46, 0.0, 0.0]);
        assert_eq!(boost_speed(moving, true, false), moving);
        assert_eq!(boost_speed(moving, false, false), moving);
        let stalled = [0.005, 0.0, 0.0];
        assert_eq!(boost_speed(stalled, true, true), stalled);
    }

    #[test]
    fn the_slope_push_the_downhill_gain_and_the_floor_wear() {
        // THE calculateSlopeSpeed: the floor the max() keeps (the slow
        // cart the 0.0078125 wears), the |motion|·0.02 the fast cart
        // speaks, the axis arm the shape's LOW side pushes.
        let slow = [0.05, 0.0, 0.0];
        let pushed = slope_speed(slow, 2, false); // ASCENDING_EAST: the west exit low
        assert_eq!(pushed[0], 0.05 - SLOPE_GRAVITY_FLOOR);
        let fast = [0.4, 0.0, 0.0];
        let pushed_fast = slope_speed(fast, 2, false);
        assert_eq!(pushed_fast[0], 0.4 - 0.4 * SLOPE_ACCEL);
        // THE AXES: the north ascent the +z pushes (the south exit
        // low), the south ascent the −z.
        let nz = slope_speed([0.0, 0.0, 0.4], 4, false);
        assert_eq!(nz[2], 0.4 + 0.4 * SLOPE_ACCEL);
        let sz = slope_speed([0.0, 0.0, 0.4], 5, false);
        assert_eq!(sz[2], 0.4 - 0.4 * SLOPE_ACCEL);
        // THE FLAT RAILS the push never touches.
        assert_eq!(slope_speed(fast, 0, false), fast);
        assert_eq!(slope_speed(fast, 6, false), fast);
    }

    #[test]
    fn the_descent_verdict_the_low_side_signs_wear() {
        // THE isDecending tableswitch: the four ascents the axis sign
        // reads, the flat/corner shapes never descend.
        assert!(is_descending([-0.1, 0.0, 0.0], 2)); // west of ASCENDING_EAST low
        assert!(!is_descending([0.1, 0.0, 0.0], 2));
        assert!(is_descending([0.1, 0.0, 0.0], 3));
        assert!(is_descending([0.0, 0.0, 0.1], 4)); // south of ASCENDING_NORTH low
        assert!(is_descending([0.0, 0.0, -0.1], 5));
        assert!(!is_descending([0.0, 0.0, 0.1], 0));
        assert!(!is_descending([1.0, 0.0, 0.0], 6));
    }

    #[test]
    fn the_v_rest_the_v_shape_zeroes_the_slow_cart() {
        // THE restAtVShape: the slow downhill roll onto a slope whose
        // descent the motion never continues — the zero the V law
        // speaks; the fast cart the law never rests.
        let delta = [0.05, 0.0, 0.0]; // the |delta|² under 0.005
        // Moving −x (descending ASCENDING_EAST) onto ASCENDING_EAST:
        // the SAME slope the descent continues — the walk never rests;
        // the OPPOSITE slope (ASCENDING_WEST, the +x descent) the
        // motion never continues — the V law rests.
        assert!(!rest_at_v_shape(delta, 2, 2));
        assert!(rest_at_v_shape(delta, 3, 2));
        // THE FAST CART the rest never wears.
        assert!(!rest_at_v_shape([0.3, 0.0, 0.0], 2, 2));
        // THE FLAT NEXT the law never fires.
        assert!(!rest_at_v_shape(delta, 2, 1));
    }

    #[test]
    fn the_step_walk_the_straight_rail_advances_the_budget() {
        // THE stepAlongTrack: the favored exit the +x motion picks,
        // the target the bottom center + half-exit + 0.1 rides, the
        // budget the horizontal magnitude keeps, the y the ride height
        // snaps.
        let mut world = yard();
        let (rx, ry, rz) = (2, Y, 2);
        world.set(rx, ry, rz, RAIL);
        world.set_variant(rx, ry, rz, 1); // EAST_WEST
        let bottom = [rx as f64 + 0.5, ry as f64, rz as f64 + 0.5];
        // THE CART mid-cell (the x −0.1 off center), moving +x at the
        // flat cap, the target the cell's east edge speaks.
        let mut cart = Minecart::new([bottom[0] - 0.1, bottom[1] + RAIL_RIDE_HEIGHT, bottom[2]]);
        cart.delta = [0.4, 0.0, 0.0];
        let remaining = step_along_track(&world, &mut cart, (rx, ry, rz), 1, 0.4);
        // THE BUDGET SPENT: the target 0.6 + eps ahead — the 0.4 step
        // never reaches it, the remaining zeroes.
        assert_eq!(remaining, 0.0);
        // THE ADVANCE: the full 0.4 the x wears, the y the ride height
        // keeps (the flat rail the y-follow never touches), the z the
        // rail line keeps.
        assert!((cart.pos[0] - (bottom[0] - 0.1 + 0.4)).abs() < 1e-9);
        assert!((cart.pos[1] - (bottom[1] + RAIL_RIDE_HEIGHT)).abs() < 1e-9);
        assert!((cart.pos[2] - bottom[2]).abs() < 1e-9);
        // THE DELTA the step direction re-stamps (the same 0.4 the
        // horizontal keeps).
        assert!((cart.delta[0] - 0.4).abs() < 1e-9);
    }

    #[test]
    fn the_step_walk_the_corner_turns_the_exit_law() {
        // THE SOUTH_EAST curve: the +z (south) motion the corner
        // redirects toward +x (east) — the favored exit the dot walk
        // picks, the target the east half-exit speaks.
        let mut world = yard();
        let (rx, ry, rz) = (5, Y, 5);
        world.set(rx, ry, rz, RAIL);
        world.set_variant(rx, ry, rz, 6); // SOUTH_EAST
        let bottom = [rx as f64 + 0.5, ry as f64, rz as f64 + 0.5];
        // THE CART at the west edge of the cell moving south — the
        // corner the east exit wears.
        let mut cart = Minecart::new([bottom[0] - 0.49, bottom[1] + RAIL_RIDE_HEIGHT, bottom[2] + 0.01]);
        cart.delta = [0.0, 0.0, 0.4];
        let remaining = step_along_track(&world, &mut cart, (rx, ry, rz), 6, 0.4);
        assert_eq!(remaining, 0.0);
        // THE TURN: the z advanced (the south the motion wore), the x
        // the eastward step gained (the corner's redirect the delta
        // keeps), the y the flat ride keeps.
        assert!(cart.pos[2] > bottom[2] + 0.01);
        assert!(cart.pos[0] > bottom[0] - 0.49);
        assert!((cart.pos[1] - (bottom[1] + RAIL_RIDE_HEIGHT)).abs() < 1e-9);
        assert!(cart.delta[0] > 0.0);
    }

    #[test]
    fn the_step_walk_the_ascent_stretches_the_sqrt_two() {
        // THE ASCENDING_EAST climb: the +x motion the √2 stretch wears,
        // the climb arm the target one block higher speaks, the cart
        // the climb the y wears.
        let mut world = yard();
        let (rx, ry, rz) = (7, Y, 7);
        world.set(rx, ry, rz, RAIL);
        world.set_variant(rx, ry, rz, 2); // ASCENDING_EAST (the east high)
        let bottom = [rx as f64 + 0.5, ry as f64, rz as f64 + 0.5];
        let mut cart = Minecart::new([bottom[0] - 0.1, bottom[1] + RAIL_RIDE_HEIGHT, bottom[2]]);
        cart.delta = [0.4, 0.0, 0.0];
        let remaining = step_along_track(&world, &mut cart, (rx, ry, rz), 2, 0.4);
        assert_eq!(remaining, 0.0);
        // THE STRETCH: the 0.4·√2 diagonal the x−y plane wears — the x
        // advance 0.4, the y the climb the same share speaks (the 1:1
        // slope the diagonal law keeps).
        assert!(cart.pos[0] > bottom[0] - 0.1 + 0.28);
        assert!(cart.pos[1] > bottom[1] + RAIL_RIDE_HEIGHT + 0.28);
    }

    #[test]
    fn the_descent_y_follow_the_run_the_height_keeps() {
        // THE y-follow (the pc 389-462 walk): the descending cart the
        // step the √2 diagonal wears — the y the descent's own share
        // carries, the raise-only snap the lower y never forces.
        let mut world = yard();
        let (rx, ry, rz) = (9, Y, 9);
        world.set(rx, ry, rz, RAIL);
        world.set_variant(rx, ry, rz, 2); // ASCENDING_EAST
        let bottom = [rx as f64 + 0.5, ry as f64, rz as f64 + 0.5];
        // THE CART high on the slope moving −x (descending).
        let mut cart = Minecart::new([
            bottom[0] - 0.1,
            bottom[1] + RAIL_RIDE_HEIGHT + 0.5,
            bottom[2],
        ]);
        cart.delta = [-0.4, 0.0, 0.0];
        let remaining = step_along_track(&world, &mut cart, (rx, ry, rz), 2, 0.4);
        assert_eq!(remaining, 0.0);
        // THE ADVANCE: the west step the x wore, the y the descent's
        // diagonal share dropped (the slope's own geometry), the
        // raise-only snap the y above the ride height keeps.
        assert!(cart.pos[0] < bottom[0] - 0.1 - 0.2);
        assert!(cart.pos[1] < bottom[1] + RAIL_RIDE_HEIGHT + 0.5);
        assert!(cart.pos[1] > bottom[1] + RAIL_RIDE_HEIGHT - 1e-9);
        assert!(cart.delta[1] < 0.0);
    }

    #[test]
    fn the_yaw_stamp_the_atan2_base_wears() {
        // THE adjustToRails yaw: the favored exit's atan2 the 128.0
        // base subtracts — the east exit the 128.0 speaks (atan2(0,1)
        // = 0), the south exit the 128 − 90 = 38.0 wears.
        let east = adjust_to_rails_yaw(1, [0.4, 0.0, 0.0], false);
        assert!((east - (YAW_BASE - 0.0)).abs() < 1e-4);
        let south = adjust_to_rails_yaw(0, [0.0, 0.0, 0.4], false);
        assert!((south - (YAW_BASE - 90.0)).abs() < 1e-4);
        // THE FLIPPED arm the +128 adds.
        let flipped = adjust_to_rails_yaw(1, [0.4, 0.0, 0.0], true);
        assert!((flipped - (YAW_BASE + YAW_BASE)).abs() < 1e-4);
    }

    #[test]
    fn the_off_rail_fall_the_default_gravity_wears() {
        // THE comeOffTrack branch's v1 fold: the cart off the rails
        // the 0.04 gravity the y velocity wears, the position the
        // motion carries.
        let world = yard();
        let mut cart = Minecart::new([12.5, 70.1, 12.5]);
        cart.delta = [0.0, 0.0, 0.0];
        cart_tick(&world, &mut cart);
        assert!(!cart.on_rails);
        assert!((cart.delta[1] - (-CART_GRAVITY)).abs() < 1e-12);
        assert!((cart.pos[1] - (70.1 - CART_GRAVITY)).abs() < 1e-12);
    }

    #[test]
    fn the_tick_walk_the_rail_ride_the_full_law_wears() {
        // THE cart_tick on a straight flat rail: the spawn the yaw
        // stamps, the ride the max cap the horizontal keeps, the cart
        // the rail line the y stays.
        let mut world = yard();
        let y = Y;
        for x in 2..12 {
            world.set(x, y, 3, RAIL);
            world.set_variant(x, y, 3, 1); // EAST_WEST
        }
        let mut cart = Minecart::new([2.3, y as f64 + RAIL_RIDE_HEIGHT, 3.5]);
        cart.delta = [0.4, 0.0, 0.0];
        cart_tick(&world, &mut cart);
        assert!(cart.on_rails);
        // THE SPEED: the cap the 0.4 keeps (the natural slowdown the
        // 0.975 the first tick wears — 0.39).
        assert!((v_horizontal_distance(cart.delta) - (0.4 * SLOWDOWN_EMPTY)).abs() < 1e-9);
        // THE RIDE: the y the rail ride height keeps, the z the rail
        // line never leaves.
        assert!((cart.pos[1] - (y as f64 + RAIL_RIDE_HEIGHT)).abs() < 1e-9);
        assert!((cart.pos[2] - 3.5).abs() < 1e-9);
        assert!(cart.pos[0] > 2.3);
    }

    #[test]
    fn the_halt_tick_the_unpowered_rail_stops_the_cart() {
        // THE full tick law: the cart rolling onto an unpowered
        // powered rail the brake wears — the speed halves, under the
        // floor the next tick zeroes.
        let mut world = yard();
        let y = Y;
        for x in 2..8 {
            world.set(x, y, 3, POWERED_RAIL);
            world.set_variant(x, y, 3, 1); // EAST_WEST, unpowered
        }
        let mut cart = Minecart::new([2.5, y as f64 + RAIL_RIDE_HEIGHT, 3.5]);
        cart.delta = [0.4, 0.0, 0.0];
        cart_tick(&world, &mut cart);
        // THE BRAKE: the halt the *0.5 wore (0.4 → 0.2), then the
        // slowdown+cap the first-iteration arm keeps (0.2 · 0.975 —
        // under the cap the halved·slowdown speed stays).
        assert!((v_horizontal_distance(cart.delta) - 0.2 * SLOWDOWN_EMPTY).abs() < 1e-9);
    }
}

//! The first-person BODY: the vanilla player's own look, input, travel
//! and collision laws, byte-for-byte the bytecode's arithmetic (javap,
//! tools/jdk-25.0.4.1+1, the cached jar's Mojang-mapped classes). The
//! client that owns this body MOVES like the original moves — the same
//! gravity, the same friction, the same jump arc, the same sneak
//! edge-guard — and speaks the same movement wire (sendPosition's own
//! dispatch law, the 2.0E-4 threshold and the 20-tick reminder).
//!
//! Pinned sources (the class -> the law):
//! - `MouseHandler.turnPlayer` + `Entity.turn` — the look curve:
//!   `f = sensitivity * 0.6 + 0.2` cubed, `* 8.0`, the per-pixel
//!   degrees `* 0.15`; pitch clamped to ±90.0.
//! - `KeyboardInput.tick` — the impulses are ±1 per axis, the move
//!   vector `Vec2(left, forward).normalized()` (zero below the 1e-4
//!   length; the diagonal normalized to 1.0).
//! - `LivingEntity.travelInAir` + `handleRelativeFrictionAndCalculateMovement`
//!   + `getFrictionInfluencedSpeed` — block friction 0.6 on the ground
//!   (1.0 in the air), the move impulse BEFORE the collide, the gravity
//!   0.08 AFTER it, the vertical drag 0.98, the horizontal drag
//!   `friction * 0.91`.
//! - `LivingEntity.setSprinting` — the sprint modifier 0.3
//!   ADD_MULTIPLIED_TOTAL on MOVEMENT_SPEED (0.1 -> 0.13); the sneak
//!   rides the SNEAKING_SPEED attribute (0.3, the Player's own default)
//!   the same operation (0.1 -> 0.07); the using-item slowdown is the
//!   documented 0.2 multiplier on the same attribute.
//! - `LivingEntity.jumpFromGround` + the aiStep gate — the jump sets
//!   `dy = max(dy, 0.42)` (JUMP_STRENGTH), the sprint adds the look
//!   direction's 0.2 boost, the repeat rides the 10-tick noJumpDelay.
//! - `Entity.collide` + `collideWithShapes` + `Direction.axisStepOrder`
//!   + `Shapes.collide` — the axis order is Y FIRST, then the dominant
//!   horizontal (`|dx| >= |dz|` -> X else Z); per shape the movement is
//!   clipped to the contact face (the epsilon 1e-7 both laws speak).
//! - `Entity.maybeBackOffFromEdge` — the sneak edge-guard: a movement
//!   that would leave no floor within maxUpStep (0.6) below is zeroed
//!   in 0.05 steps.
//! - `LocalPlayer.sendPosition` — pos changed iff
//!   `Mth.lengthSquared(dx, dy, dz) > Mth.square(2.0E-4)` or the
//!   position reminder hit 20; rot changed iff either angle differs;
//!   both -> PosRot, pos -> Pos, rot -> Rot, else the on-ground /
//!   horizontal-collision change -> StatusOnly.
//! - `Avatar.POSES` — the crouch pose is `scalable(0.6, 1.5)` with the
//!   eye 1.27 (the standing 1.8 / 1.62 the placement lane already
//!   speaks).

use crate::world::{World, AIR, WATER};

// THE CONSTANTS (the bytecode's own numbers).
pub const MOVEMENT_SPEED: f32 = 0.10000000149011612;
pub const SPRINT_MULTIPLIER: f32 = 1.3; // the 0.3 ADD_MULTIPLIED_TOTAL
pub const SNEAK_MULTIPLIER: f32 = 0.7; // 1 - the SNEAKING_SPEED 0.3
pub const USING_MULTIPLIER: f32 = 0.2; // the using-item slowdown
pub const BLOCK_FRICTION: f32 = 0.6;
pub const AIR_FRICTION: f32 = 1.0;
pub const GROUND_DRAG: f32 = 0.91; // the horizontal drag's constant part
pub const VERTICAL_DRAG: f32 = 0.98;
pub const GRAVITY: f64 = 0.08;
pub const FLYING_SPEED: f32 = 0.02;
pub const JUMP_POWER: f32 = 0.42; // JUMP_STRENGTH
pub const SPRINT_JUMP_BOOST: f32 = 0.2;
pub const JUMP_REPEAT_TICKS: i32 = 10; // noJumpDelay
pub const CUBE_SPEED_NUMERATOR: f32 = 0.21600002; // the ice-slip law's own
pub const WIDTH: f64 = 0.6;
pub const STANDING_HEIGHT: f64 = 1.8;
pub const CROUCH_HEIGHT: f64 = 1.5;
pub const STANDING_EYE: f64 = 1.62;
pub const CROUCH_EYE: f64 = 1.27;
pub const MAX_UP_STEP: f64 = 0.6; // the sneak edge-guard's probe depth
pub const EDGE_STEP: f64 = 0.05; // the guard's own zeroing step
pub const LOOK_SCALE: f64 = 0.15; // Entity.turn's degrees per unit
pub const LOOK_GAIN: f64 = 8.0; // turnPlayer's post-cube gain
pub const PITCH_LIMIT: f32 = 90.0;
pub const POS_EPSILON: f64 = 2.0e-4; // sendPosition's threshold
pub const POS_REMINDER: i32 = 20; // the forced position refresh
pub const COLLISION_EPSILON: f64 = 1e-7;
/// The overlap skin: an overlap thinner than this on the OTHER two
/// axes is the box STANDING ON a face (the f32 camera boundary's own
/// 1.2e-7 artifact), never a sideways collision — the vanilla's
/// findIndex bounds speak the same exclusion over its exact f64
/// poses.
pub const COLLISION_SKIN: f64 = 1e-6;

/// The sensitivity f: the vanilla OptionInstance's 0..1 slider. The
/// default 0.5 gives the famous 0.15 deg/px.
fn look_gain(sensitivity: f64) -> f64 {
    let f = sensitivity * 0.6000000238418579 + 0.20000000298023224;
    f * f * f * LOOK_GAIN * LOOK_SCALE
}

/// The vanilla default sensitivity (the Options slider's own default).
pub const LOOK_SENSITIVITY_DEFAULT: f64 = 0.5;

/// One collision sweep result — the allowed movement per axis plus the
/// flags the wire and the ground state speak. The collided flags ride
/// the restitution law: the collided components of the carried
/// velocity ZERO after the move (the non-bouncy player's own
/// `restituteMovementAfterCollisions` verdict — the bounciness 0).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Collision {
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub horizontal_collision: bool,
    pub vertical_collision_below: bool,
    pub collided_x: bool,
    pub collided_y: bool,
    pub collided_z: bool,
}

/// The player's bounding box (the feet-anchored AABB the collision
/// sweeps).
fn box_at(x: f64, y: f64, z: f64, height: f64) -> (f64, f64, f64, f64, f64, f64) {
    let h = WIDTH / 2.0;
    (x - h, y, z - h, x + h, y + height, z + h)
}

/// The solid predicate — the clip lane's own (the fluids never decide
/// the hit, the air is the air).
fn solid(world: &World, x: i32, y: i32, z: i32) -> bool {
    let id = world.get(x, y, z);
    id != AIR && id != WATER
}

/// The per-axis clip (the classic AABB clipXCollide law the VoxelShape
/// sweep reduces to over full cubes): the movement along `axis` (0=X,
/// 1=Y, 2=Z) is clipped to the contact face of every solid cell
/// overlapping the box on the OTHER two axes. An already-intersecting
/// cell stops the movement dead (the vanilla bitset sweep's own
/// verdict); the touching contact still moves.
fn clip_axis(world: &World, b: (f64, f64, f64, f64, f64, f64), axis: usize, d: f64) -> f64 {
    if d.abs() < COLLISION_EPSILON {
        return 0.0;
    }
    let (bm, bx) = match axis {
        0 => (b.0, b.3),
        1 => (b.1, b.4),
        _ => (b.2, b.5),
    };
    // the other two axes' box ranges (the overlap region)
    let (a1lo, a1hi, a2lo, a2hi) = match axis {
        0 => (b.1, b.4, b.2, b.5),
        1 => (b.0, b.3, b.2, b.5),
        _ => (b.0, b.3, b.1, b.4),
    };
    // the swept range on the movement axis (the box grown by d)
    let (swept_lo, swept_hi) = if d > 0.0 { (bm, bx + d) } else { (bm + d, bx) };
    // the WORLD-space cell ranges: the swept axis' range rides its own
    // coordinate, the other two ride the box extents
    let (xlo, xhi, ylo, yhi, zlo, zhi) = match axis {
        0 => (swept_lo, swept_hi, a1lo, a1hi, a2lo, a2hi),
        1 => (a1lo, a1hi, swept_lo, swept_hi, a2lo, a2hi),
        _ => (a1lo, a1hi, a2lo, a2hi, swept_lo, swept_hi),
    };
    let mut allowed = d;
    for cx in xlo.floor() as i32..=(xhi - 1e-9).floor() as i32 {
        for cy in ylo.floor() as i32..=(yhi - 1e-9).floor() as i32 {
            for cz in zlo.floor() as i32..=(zhi - 1e-9).floor() as i32 {
                if !solid(world, cx, cy, cz) {
                    continue;
                }
                // the cell's slabs on the other two axes must strictly
                // overlap the box (the epsilon the vanilla's
                // findIndex bounds carry)
                let (c1lo, c1hi) = match axis {
                    1 => (cx as f64, (cx + 1) as f64),
                    _ => (cy as f64, (cy + 1) as f64),
                };
                let (c2lo, c2hi) = match axis {
                    1 => (cz as f64, (cz + 1) as f64),
                    0 => (cz as f64, (cz + 1) as f64),
                    _ => (cx as f64, (cx + 1) as f64),
                };
                if !(c1lo < a1hi - COLLISION_SKIN && c1hi > a1lo + COLLISION_SKIN) {
                    continue;
                }
                if !(c2lo < a2hi - COLLISION_SKIN && c2hi > a2lo + COLLISION_SKIN) {
                    continue;
                }
                let (c_lo, c_hi) = match axis {
                    0 => (cx as f64, (cx + 1) as f64),
                    1 => (cy as f64, (cy + 1) as f64),
                    _ => (cz as f64, (cz + 1) as f64),
                };
                let overlapping = bm < c_hi - COLLISION_EPSILON && bx > c_lo + COLLISION_EPSILON;
                if d > 0.0 {
                    if overlapping {
                        return 0.0; // already inside this cell's slab
                    }
                    if bx <= c_lo + COLLISION_EPSILON {
                        let gap = c_lo - bx;
                        if gap < allowed {
                            allowed = gap;
                        }
                    }
                } else {
                    if overlapping {
                        return 0.0;
                    }
                    if bm >= c_hi - COLLISION_EPSILON {
                        let gap = c_hi - bm;
                        if gap > allowed {
                            allowed = gap;
                        }
                    }
                }
            }
        }
    }
    allowed
}

/// The full collide: the axisStepOrder sweep (Y first, then the
/// dominant horizontal), the so-far offset applied per axis.
pub fn collide(world: &World, b: (f64, f64, f64, f64, f64, f64), mv: (f64, f64, f64)) -> Collision {
    let mut off = (0.0f64, 0.0f64, 0.0f64);
    let mut axis_allowed = [mv.0, mv.1, mv.2];
    // the order: Y always first, then the larger horizontal component
    let order: [usize; 3] = if mv.0.abs() >= mv.2.abs() {
        [1, 0, 2]
    } else {
        [1, 2, 0]
    };
    for &axis in &order {
        let d = axis_allowed[axis];
        if d == 0.0 {
            continue;
        }
        let cur = (
            b.0 + off.0,
            b.1 + off.1,
            b.2 + off.2,
            b.3 + off.0,
            b.4 + off.1,
            b.5 + off.2,
        );
        let allowed = clip_axis(world, cur, axis, d);
        axis_allowed[axis] = allowed;
        match axis {
            0 => off.0 = allowed,
            1 => off.1 = allowed,
            _ => off.2 = allowed,
        }
    }
    Collision {
        dx: axis_allowed[0],
        dy: axis_allowed[1],
        dz: axis_allowed[2],
        horizontal_collision: axis_allowed[0] != mv.0 || axis_allowed[2] != mv.2,
        vertical_collision_below: mv.1 < 0.0 && axis_allowed[1] != mv.1,
        collided_x: axis_allowed[0] != mv.0,
        collided_y: axis_allowed[1] != mv.1,
        collided_z: axis_allowed[2] != mv.2,
    }
}

/// The movement wire's dispatch — sendPosition's own shape.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveWire {
    PosRot {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
        horizontal_collision: bool,
    },
    Pos {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
        horizontal_collision: bool,
    },
    Rot {
        yaw: f32,
        pitch: f32,
        on_ground: bool,
        horizontal_collision: bool,
    },
    StatusOnly {
        on_ground: bool,
        horizontal_collision: bool,
    },
}

/// The vanilla player body.
#[derive(Debug, Clone)]
pub struct PlayerBody {
    /// The FEET position (the AABB's bottom center) in world blocks.
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// The velocity (the vanilla deltaMovement).
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    /// The vanilla angles (degrees — the wire's own convention).
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
    pub horizontal_collision: bool,
    /// The RESOLVED pose (the key state folded with the can-stand law
    /// — the ceiling never embeds the grown body).
    pub crouching: bool,
    /// The crouch KEY state (the resolved pose may stay crouched after
    /// the key releases when the standing box does not fit).
    pub want_crouch: bool,
    pub sprinting: bool,
    /// The input impulses (the KeyboardInput's own ±1 shape).
    pub forward: f32,
    pub left: f32,
    pub jumping: bool,
    no_jump_delay: i32,
    /// THE FOOTSTEP ACCUMULATION (Entity.move's own fields): the walk
    /// distance grows by the horizontal movement × 0.6, the step fires
    /// when it crosses `next_step` (nextStep() = (int)moveDist + 1.0F —
    /// the vanilla's one-footstep-per-~1.667-blocks cadence). The App
    /// drains `step_due` after the tick and speaks the block under the
    /// feet's own voice.
    pub move_dist: f32,
    pub next_step: f32,
    /// the tick's step fired (the block under was solid) — the App
    /// clears it after the sound folds
    pub step_due: bool,
    // sendPosition's bookkeeping
    position_reminder: i32,
    x_last: f64,
    y_last: f64,
    z_last: f64,
    yaw_last: f32,
    pitch_last: f32,
    last_on_ground: bool,
    last_horizontal_collision: bool,
}

impl PlayerBody {
    /// A standing body at the feet position, facing the vanilla angles.
    pub fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        PlayerBody {
            x,
            y,
            z,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            yaw,
            pitch,
            on_ground: false,
            horizontal_collision: false,
            crouching: false,
            want_crouch: false,
            sprinting: false,
            forward: 0.0,
            left: 0.0,
            jumping: false,
            no_jump_delay: 0,
            move_dist: 0.0,
            next_step: 1.0,
            step_due: false,
            position_reminder: 0,
            x_last: x,
            y_last: y,
            z_last: z,
            yaw_last: yaw,
            pitch_last: pitch,
            last_on_ground: false,
            last_horizontal_collision: false,
        }
    }

    /// THE LOOK: the mouse deltas through the turnPlayer curve. The
    /// yaw is unbounded (the vanilla wraps at ±360 by the client's own
    /// normalization habit — the degrees ride the wire raw), the pitch
    /// clamps to ±90.
    pub fn turn(&mut self, dx_px: f64, dy_px: f64, sensitivity: f64) {
        let k = look_gain(sensitivity);
        self.pitch = (self.pitch + (dy_px * k) as f32).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        self.yaw += (dx_px * k) as f32;
    }

    /// The pose's height (the POSES table's own numbers).
    pub fn height(&self) -> f64 {
        if self.crouching {
            CROUCH_HEIGHT
        } else {
            STANDING_HEIGHT
        }
    }

    /// The pose's eye height (the camera rides it).
    pub fn eye_height(&self) -> f64 {
        if self.crouching {
            CROUCH_EYE
        } else {
            STANDING_EYE
        }
    }

    /// The speed attribute's current value (the base, the sprint's
    /// 1.3, the sneak's 0.7, the using slowdown's 0.2 — the
    /// ADD_MULTIPLIED_TOTAL fold the attribute system speaks).
    fn speed(&self, using_item: bool) -> f32 {
        let mut s = MOVEMENT_SPEED;
        if self.sprinting {
            s *= SPRINT_MULTIPLIER;
        }
        if self.crouching {
            s *= SNEAK_MULTIPLIER;
        }
        if using_item {
            s *= USING_MULTIPLIER;
        }
        s
    }

    /// The ground under the moved box probe (the sneak edge-guard's
    /// `noCollision(box.move(dx, -maxUpStep, dz))` question): TRUE when
    /// NOTHING collides with the box sunk by maxUpStep.
    fn no_floor_below(&self, world: &World, dx: f64, dz: f64) -> bool {
        let b = box_at(self.x + dx, self.y, self.z + dz, self.height());
        let sunk = (b.0, b.1 - MAX_UP_STEP, b.2, b.3, b.4 - MAX_UP_STEP, b.5);
        // any solid cell intersecting the sunk box? (the overlap test —
        // the strict edges the vanilla AABB carries)
        let (x0, y0, z0, x1, y1, z1) = sunk;
        let mut hit = false;
        for cx in x0.floor() as i32..=((x1 - 1e-9).floor()) as i32 {
            for cy in y0.floor() as i32..=((y1 - 1e-9).floor()) as i32 {
                for cz in z0.floor() as i32..=((z1 - 1e-9).floor()) as i32 {
                    if solid(world, cx, cy, cz)
                        && (cx as f64) < x1
                        && ((cx + 1) as f64) > x0
                        && (cy as f64) < y1
                        && ((cy + 1) as f64) > y0
                        && (cz as f64) < z1
                        && ((cz + 1) as f64) > z0
                    {
                        hit = true;
                    }
                }
            }
        }
        !hit
    }

    /// The can-stand probe: does the STANDING box fit at the current
    /// feet (the un-crouch law — a low ceiling keeps the pose crouched)?
    fn can_stand(&self, world: &World, x: f64, z: f64) -> bool {
        let b = box_at(x, self.y, z, STANDING_HEIGHT);
        let (x0, y0, z0, x1, y1, z1) = b;
        for cx in x0.floor() as i32..=((x1 - 1e-9).floor()) as i32 {
            for cy in y0.floor() as i32..=((y1 - 1e-9).floor()) as i32 {
                for cz in z0.floor() as i32..=((z1 - 1e-9).floor()) as i32 {
                    if solid(world, cx, cy, cz)
                        && (cx as f64) < x1
                        && ((cx + 1) as f64) > x0
                        && (cy as f64) < y1
                        && ((cy + 1) as f64) > y0
                        && (cz as f64) < z1
                        && ((cz + 1) as f64) > z0
                    {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// THE TICK: one vanilla 50ms step — the jump gate, the move
    /// impulse, the collide, the gravity and the drags. `using_item`
    /// speaks the use arc's own slowdown.
    pub fn tick(&mut self, world: &World, using_item: bool) {
        if self.no_jump_delay > 0 {
            self.no_jump_delay -= 1;
        }
        // THE POSE (updatePlayerPose's own core): the crouch holds
        // while the key holds AND un-crouches only when the standing
        // box fits (the ceiling never embeds the grown body)
        self.crouching = self.want_crouch || !self.can_stand(world, self.x, self.z);
        // THE JUMP GATE (the aiStep's own): held jump + ground + the
        // delay elapsed -> jumpFromGround
        if self.jumping && self.on_ground && self.no_jump_delay == 0 {
            self.vy = (self.vy as f64).max(JUMP_POWER as f64);
            if self.sprinting {
                let rad = self.yaw as f64 * std::f64::consts::PI / 180.0;
                self.vx += -(rad.sin()) * SPRINT_JUMP_BOOST as f64;
                self.vz += rad.cos() * SPRINT_JUMP_BOOST as f64;
            }
            self.no_jump_delay = JUMP_REPEAT_TICKS;
        }
        // THE MOVE IMPULSE (getInputVector's law): the input vector
        // (left, forward) normalized past 1.0, scaled by the
        // friction-influenced speed (the GROUND control is the
        // attribute — the cube correction only past friction 0.6, the
        // ice law our vocabulary never speaks; the AIR control is the
        // flyingSpeed 0.02), rotated by the yaw
        let mut ix = self.left as f64;
        let mut iz = self.forward as f64;
        let len_sq = ix * ix + iz * iz;
        if len_sq > 1e-7 {
            if len_sq > 1.0 {
                let len = len_sq.sqrt();
                ix /= len;
                iz /= len;
            }
            let friction = if self.on_ground { BLOCK_FRICTION } else { AIR_FRICTION };
            let speed = if self.on_ground {
                if friction > 0.6 {
                    self.speed(using_item) * CUBE_SPEED_NUMERATOR / (friction * friction * friction)
                } else {
                    self.speed(using_item)
                }
            } else {
                FLYING_SPEED
            };
            let speed = speed as f64;
            let rad = self.yaw as f64 * std::f64::consts::PI / 180.0;
            let (s, c) = (rad.sin(), rad.cos());
            self.vx += (ix * c - iz * s) * speed;
            self.vz += (iz * c + ix * s) * speed;
        }
        // THE COLLIDE (maybeBackOffFromEdge first: the sneak never
        // walks off its ledge)
        let mut mv = (self.vx, self.vy, self.vz);
        if self.crouching && self.on_ground {
            // the X probe, the Z probe, then the vanilla's JOINT
            // diagonal probe — each zeroes the movement stepwise while
            // the sunk box finds no floor
            while mv.0 != 0.0 && self.no_floor_below(world, mv.0, 0.0) {
                if mv.0.abs() <= EDGE_STEP {
                    mv.0 = 0.0;
                } else if mv.0 > 0.0 {
                    mv.0 -= EDGE_STEP;
                } else {
                    mv.0 += EDGE_STEP;
                }
            }
            while mv.2 != 0.0 && self.no_floor_below(world, 0.0, mv.2) {
                if mv.2.abs() <= EDGE_STEP {
                    mv.2 = 0.0;
                } else if mv.2 > 0.0 {
                    mv.2 -= EDGE_STEP;
                } else {
                    mv.2 += EDGE_STEP;
                }
            }
            while mv.0 != 0.0 && mv.2 != 0.0 && self.no_floor_below(world, mv.0, mv.2) {
                if mv.0.abs() <= EDGE_STEP {
                    mv.0 = 0.0;
                } else if mv.0 > 0.0 {
                    mv.0 -= EDGE_STEP;
                } else {
                    mv.0 += EDGE_STEP;
                }
                if mv.2.abs() <= EDGE_STEP {
                    mv.2 = 0.0;
                } else if mv.2 > 0.0 {
                    mv.2 -= EDGE_STEP;
                } else {
                    mv.2 += EDGE_STEP;
                }
            }
        }
        let b = box_at(self.x, self.y, self.z, self.height());
        let col = collide(world, b, mv);
        self.x += col.dx;
        self.y += col.dy;
        self.z += col.dz;
        self.horizontal_collision = col.horizontal_collision;
        self.on_ground = col.vertical_collision_below;
        // THE FOOTSTEP ACCUMULATION (Entity.move's own law over the
        // ACTUAL moved delta): moveDist += horizontalDistance × 0.6 (the
        // climbable branch's full-length lane aside — our vocabulary
        // climbs nothing yet); the step fires when moveDist > nextStep
        // AND the block under the feet (getOnPos: floor(x), floor(y -
        // 0.2), floor(z)) is not air — the vanilla's own isAir gate —
        // and the counter resets to (int)moveDist + 1.0F.
        self.step_due = false;
        let horizontal = ((col.dx * col.dx + col.dz * col.dz).sqrt()) as f32;
        self.move_dist += horizontal * 0.6;
        if self.move_dist > self.next_step {
            let under = world.get(
                self.x.floor() as i32,
                (self.y - 0.2).floor() as i32,
                self.z.floor() as i32,
            );
            if under != AIR {
                self.next_step = (self.move_dist as i32) as f32 + 1.0;
                self.step_due = true;
            }
        }
        // THE RESTITUTION (restituteMovementAfterCollisions with the
        // player's own bounciness 0): the collided components of the
        // carried velocity zero — the resting rhythm stays the vanilla
        // constant (the wall slide keeps the tangent axis)
        if col.collided_x {
            self.vx = 0.0;
        }
        if col.collided_y {
            self.vy = 0.0;
        }
        if col.collided_z {
            self.vz = 0.0;
        }
        // THE DRAGS + THE GRAVITY (travelInAir's own order): the
        // horizontal decay `friction * 0.91`, the vertical 0.98 with
        // the gravity subtracted BEFORE it
        let friction = if self.on_ground { BLOCK_FRICTION } else { AIR_FRICTION };
        self.vy = (self.vy - GRAVITY) * VERTICAL_DRAG as f64;
        let drag = (friction * GROUND_DRAG) as f64;
        self.vx *= drag;
        self.vz *= drag;
    }

    /// THE WIRE (sendPosition): computes what this tick speaks and
    /// advances the bookkeeping. Call once per tick AFTER move.
    pub fn take_move_wire(&mut self) -> Option<MoveWire> {
        let dx = self.x - self.x_last;
        let dy = self.y - self.y_last;
        let dz = self.z - self.z_last;
        let yaw_d = self.yaw - self.yaw_last;
        let pitch_d = self.pitch - self.pitch_last;
        self.position_reminder += 1;
        let len_sq = dx * dx + dy * dy + dz * dz;
        let pos_changed =
            len_sq > POS_EPSILON * POS_EPSILON || self.position_reminder >= POS_REMINDER;
        let rot_changed = yaw_d != 0.0 || pitch_d != 0.0;
        let out = if pos_changed && rot_changed {
            Some(MoveWire::PosRot {
                x: self.x,
                y: self.y,
                z: self.z,
                yaw: self.yaw,
                pitch: self.pitch,
                on_ground: self.on_ground,
                horizontal_collision: self.horizontal_collision,
            })
        } else if pos_changed {
            Some(MoveWire::Pos {
                x: self.x,
                y: self.y,
                z: self.z,
                on_ground: self.on_ground,
                horizontal_collision: self.horizontal_collision,
            })
        } else if rot_changed {
            Some(MoveWire::Rot {
                yaw: self.yaw,
                pitch: self.pitch,
                on_ground: self.on_ground,
                horizontal_collision: self.horizontal_collision,
            })
        } else if self.last_on_ground != self.on_ground
            || self.last_horizontal_collision != self.horizontal_collision
        {
            Some(MoveWire::StatusOnly {
                on_ground: self.on_ground,
                horizontal_collision: self.horizontal_collision,
            })
        } else {
            None
        };
        if pos_changed {
            self.x_last = self.x;
            self.y_last = self.y;
            self.z_last = self.z;
            self.position_reminder = 0;
        }
        if rot_changed {
            self.yaw_last = self.yaw;
            self.pitch_last = self.pitch;
        }
        self.last_on_ground = self.on_ground;
        self.last_horizontal_collision = self.horizontal_collision;
        out
    }

    /// The camera pose (the snapshot_camera conversion's own inverse:
    /// camera yaw rad = vanilla yaw rad + pi, camera pitch rad = -
    /// vanilla pitch rad).
    pub fn camera(&self) -> crate::camera::Camera {
        crate::camera::Camera {
            pos: crate::math::Vec3::new(
                self.x as f32,
                (self.y + self.eye_height()) as f32,
                self.z as f32,
            ),
            yaw: self.yaw.to_radians() + std::f32::consts::PI,
            pitch: -self.pitch.to_radians(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A flat world: stone at y=0, air above (10x10 columns).
    fn flat_world() -> World {
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, if y == 0 { crate::world::STONE } else { crate::world::AIR });
                }
            }
        }
        w
    }

    /// A one-block wall at x=5 (y 1..=2) the body walks into.
    fn walled_world() -> World {
        let mut w = flat_world();
        w.set(5, 1, 5, crate::world::STONE);
        w.set(5, 2, 5, crate::world::STONE);
        w
    }

    /// Settles the body onto the floor (the vanilla spawn rhythm: the
    /// first tick has no vertical movement yet — the gravity applies
    /// AFTER the collide — so the ground flag arrives on tick 2).
    fn settle(b: &mut PlayerBody, w: &World) {
        for _ in 0..4 {
            b.tick(w, false);
        }
    }

    /// THE FOOTSTEP CADENCE (Entity.move's own law): the walk distance
    /// grows by the horizontal movement × 0.6, a step fires per whole
    /// moveDist unit over solid ground (the isAir gate never trips on
    /// the flat stone), the counter resets to (int)moveDist + 1.0F.
    #[test]
    fn the_footstep_cadence_is_entity_move_law() {
        // a WIDE flat floor: the walk travels ~8 blocks — the 16x16
        // helper would drop the body off its edge into the void (the
        // void's own isAir silence is a different test's story)
        let mut w = World::new(4, 42);
        for x in 0..64 {
            for z in 0..64 {
                for y in 0..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, if y == 0 { crate::world::STONE } else { crate::world::AIR });
                }
            }
        }
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        b.forward = 1.0;
        let mut steps = 0;
        for _ in 0..40 {
            b.tick(&w, false);
            if b.step_due {
                steps += 1;
                assert_eq!(
                    b.next_step,
                    (b.move_dist as i32) as f32 + 1.0,
                    "the reset rides nextStep()'s law ({})",
                    b.next_step
                );
                assert!(
                    b.move_dist > b.next_step - 1.0,
                    "the step fired at the crossing (moveDist {})",
                    b.move_dist
                );
                // the block under the feet on the flat world is stone —
                // the App folds the step voice through the STONE row
                // (the sound lane's own tests); the body only names the
                // cadence
            }
        }
        assert!(b.move_dist > 3.0, "the walk accumulated ({})", b.move_dist);
        assert_eq!(
            steps,
            b.move_dist as i32 as usize,
            "one step per whole moveDist unit over solid ground ({})",
            b.move_dist
        );
    }

    #[test]
    fn look_curve_is_the_turnplayer_law() {
        // the default sensitivity 0.5: f = 0.5*0.6+0.2 = 0.5, cubed
        // 0.125, *8 = 1.0, *0.15 = 0.15 deg/px — the famous number
        let mut b = PlayerBody::new(0.0, 1.0, 0.0, 0.0, 0.0);
        b.turn(1.0, 0.0, 0.5);
        assert!((b.yaw - 0.15).abs() < 1e-6, "yaw {}", b.yaw);
        assert_eq!(b.pitch, 0.0);
        // the pitch clamps at +-90
        let mut b = PlayerBody::new(0.0, 1.0, 0.0, 0.0, 0.0);
        b.turn(0.0, -100000.0, 0.5);
        assert_eq!(b.pitch, -PITCH_LIMIT);
        b.turn(0.0, 200000.0, 0.5);
        assert_eq!(b.pitch, PITCH_LIMIT);
        // the yaw is unbounded (wraps only at the display side)
        let mut b = PlayerBody::new(0.0, 1.0, 0.0, 0.0, 0.0);
        b.turn(100000.0, 0.0, 0.5);
        assert!(b.yaw > 1000.0);
        // the sensitivity curve: s=0.0 -> f=0.2, cubed 0.008, *8*0.15
        // = 0.0096 deg/px
        let mut b = PlayerBody::new(0.0, 1.0, 0.0, 0.0, 0.0);
        b.turn(1.0, 0.0, 0.0);
        assert!((b.yaw - 0.0096).abs() < 1e-6, "yaw {}", b.yaw);
    }

    #[test]
    fn fall_lands_on_the_floor_and_flags_the_ground() {
        let w = flat_world();
        // 3 blocks above the floor: the fall takes a few ticks
        let mut b = PlayerBody::new(8.0, 4.0, 8.0, 0.0, 0.0);
        for _ in 0..40 {
            b.tick(&w, false);
        }
        assert!(
            (b.y - 1.0).abs() < 1e-9,
            "the feet rest ON the floor surface, got {}",
            b.y
        );
        assert!(b.on_ground, "the landed body is on the ground");
        // resting: the velocity holds near the terminal -0.0784 rhythm
        assert!(b.vy.abs() < 0.1);
    }

    #[test]
    fn walk_converges_to_the_vanilla_rhythm() {
        let w = flat_world();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w); // the settle ticks (gravity -> ground)
        assert!(b.on_ground);
        b.forward = 1.0;
        // the drag scales the CARRIED velocity; the MOVE integrates the
        // carried + impulse: the carried vz converges to
        // 0.1*0.546/(1-0.546) = 0.1203, the per-tick position delta to
        // 0.2203 blocks/tick — the famous ~4.4 m/s walk
        let mut v = 0.0f64;
        let mut last_z = b.z;
        let mut move_delta = 0.0f64;
        for _ in 0..60 {
            b.tick(&w, false);
            v = b.vz;
            move_delta = b.z - last_z;
            last_z = b.z;
        }
        let carried = 0.1 * (BLOCK_FRICTION * GROUND_DRAG) as f64
            / (1.0 - (BLOCK_FRICTION * GROUND_DRAG) as f64);
        assert!((v - carried).abs() < 1e-3, "carried vz {} vs {}", v, carried);
        assert!(
            (move_delta - (carried + 0.1)).abs() < 1e-3,
            "the per-tick move {} vs {}",
            move_delta,
            carried + 0.1
        );
        assert!(b.z > 8.0, "the body moved south");
    }

    #[test]
    fn diagonal_walk_normalizes_to_one() {
        let w = flat_world();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        b.forward = 1.0;
        b.left = 1.0;
        let (x0, z0) = (b.x, b.z);
        b.tick(&w, false);
        // the impulse vector (1,1) normalized to length 1 before the
        // speed scale; the yaw 0 rotation maps left to +x and forward
        // to +z equally: the per-tick MOVE's horizontal magnitude is
        // the walk speed 0.1, split evenly
        let h = ((b.x - x0).powi(2) + (b.z - z0).powi(2)).sqrt();
        assert!((h - 0.1).abs() < 1e-6, "the normalized diagonal move {}", h);
        assert!((b.x - x0 - (b.z - z0)).abs() < 1e-9);
    }

    #[test]
    fn wall_stops_the_body_at_the_face() {
        let w = walled_world();
        // walking east (vanilla yaw 270 = +X) into the wall at x=5
        let mut b = PlayerBody::new(3.0, 1.0, 5.5, 270.0, 0.0);
        settle(&mut b, &w);
        b.forward = 1.0;
        for _ in 0..40 {
            b.tick(&w, false);
        }
        // the box half-width 0.3 rests against the face at x=5
        assert!(
            (b.x - (5.0 - WIDTH / 2.0)).abs() < 1e-9,
            "x {} rests at the wall face",
            b.x
        );
        assert!(b.horizontal_collision, "the horizontal flag rides");
    }

    #[test]
    fn ceiling_stops_the_rise() {
        // a ceiling cell at y=3 over the spawn column: the standing head
        // (1.0 + 1.8 = 2.8) has only 0.2 of clearance — the jump's rise
        // clips at the ceiling's face and the restitution zeroes the
        // carried vy (the non-bouncy player's own verdict)
        let mut w = flat_world();
        w.set(8, 3, 8, crate::world::STONE);
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        assert!(b.on_ground);
        b.jumping = true;
        let y0 = b.y;
        b.tick(&w, false); // the jump starts and clips at once
        assert!(
            (b.y - y0 - 0.2).abs() < 1e-9,
            "the rise clipped at the ceiling face, got {}",
            b.y - y0
        );
        assert!(
            (b.vy - (0.0 - GRAVITY) * VERTICAL_DRAG as f64).abs() < 1e-9,
            "the restitution zeroed the collided vy; the gravity rhythm carried it on ({})",
            b.vy
        );
        for _ in 0..30 {
            b.tick(&w, false);
        }
        // the standing head never enters the ceiling slab
        assert!(b.y + STANDING_HEIGHT <= 3.0 + 1e-9, "the head stopped at y {}", b.y);
    }

    #[test]
    fn jump_arc_is_the_vanilla_law() {
        let w = flat_world();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        let y0 = b.y;
        b.jumping = true;
        b.tick(&w, false);
        // the move integrated the pre-drag jump velocity 0.42; the
        // carried vy rode the gravity/drag to (0.42-0.08)*0.98
        assert!((b.y - y0 - JUMP_POWER as f64).abs() < 1e-9, "the jump rise {}", b.y - y0);
        assert_eq!(
            b.vy,
            (JUMP_POWER as f64 - GRAVITY) * VERTICAL_DRAG as f64,
            "the carried vy rode the gravity and the drag"
        );
        assert_eq!(b.no_jump_delay, JUMP_REPEAT_TICKS, "the 10-tick delay rides");
        // holding the jump: no re-jump inside the delay
        b.tick(&w, false);
        let vy_after = b.vy;
        b.jumping = true;
        b.tick(&w, false);
        assert!(b.vy < vy_after + 0.4, "the delay gates the re-jump");
    }

    #[test]
    fn sprint_jump_boosts_the_look_direction() {
        let w = flat_world();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 270.0, 0.0); // east
        settle(&mut b, &w);
        b.sprinting = true;
        b.jumping = true;
        let (x0, z0) = (b.x, b.z);
        b.tick(&w, false);
        // yaw 270: rad = 270*pi/180 = 4.712; -sin = 1.0, cos ~ 0.0 —
        // the boost rides the pre-drag move: dx = 0.2, dz ~ 0
        assert!(
            (b.x - x0 - SPRINT_JUMP_BOOST as f64).abs() < 1e-9,
            "the east sprint boost dx {}",
            b.x - x0
        );
        assert!((b.z - z0).abs() < 1e-6);
    }

    #[test]
    fn speeds_fold_the_attribute_multipliers() {
        let w = flat_world();
        // the multipliers ride the PRE-DRAG move (the position delta
        // each tick speaks the attribute's own value)
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        b.forward = 1.0;
        b.sprinting = true;
        let z0 = b.z;
        b.tick(&w, false);
        let expected = MOVEMENT_SPEED * SPRINT_MULTIPLIER;
        assert!(
            (b.z - z0 - expected as f64).abs() < 1e-9,
            "the first sprint move {}",
            b.z - z0
        );
        // the sneak: shift folds 0.7 (the KEY state resolves the pose)
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        b.want_crouch = true;
        b.forward = 1.0;
        let z0 = b.z;
        b.tick(&w, false);
        assert!(b.crouching, "the pose resolves from the key");
        let expected = MOVEMENT_SPEED * SNEAK_MULTIPLIER;
        assert!((b.z - z0 - expected as f64).abs() < 1e-9, "the sneak move {}", b.z - z0);
        // the using-item slowdown: the bow pull walks at 20%
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        b.forward = 1.0;
        let z0 = b.z;
        b.tick(&w, true);
        let expected = MOVEMENT_SPEED * USING_MULTIPLIER;
        assert!((b.z - z0 - expected as f64).abs() < 1e-9, "the using move {}", b.z - z0);
    }

    #[test]
    fn sneak_never_walks_off_the_ledge() {
        // a ledge: the floor only where x < 8
        let mut w = flat_world();
        for x in 8..16 {
            for z in 0..16 {
                w.set(x, 0, z, crate::world::AIR);
                w.set(x, 1, z, crate::world::AIR);
                w.set(x, 2, z, crate::world::AIR);
                w.set(x, 3, z, crate::world::AIR);
            }
        }
        let mut b = PlayerBody::new(7.0, 1.0, 8.0, 270.0, 0.0); // facing the edge
        settle(&mut b, &w);
        assert!(b.on_ground);
        b.want_crouch = true;
        b.forward = 1.0;
        for _ in 0..60 {
            b.tick(&w, false);
        }
        // the body may hang its box over the edge but never LEAVE the
        // ledge (the last floor cell ends at x=8; the box max x may
        // reach past it — the vanilla law — but the FEET stay on the
        // cell: x - 0.3 < 8)
        assert!(b.x - WIDTH / 2.0 < 8.0, "the feet x {} stayed on the ledge", b.x);
        assert!(b.on_ground, "never fell");
    }

    #[test]
    fn wire_dispatch_speaks_the_sendposition_law() {
        let w = flat_world();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        // the settle: the position never moved past 2e-4 but the
        // on-ground flag rose — the first wire is the StatusOnly
        match b.take_move_wire() {
            Some(MoveWire::StatusOnly { on_ground, .. }) => assert!(on_ground),
            other => panic!("expected the status-only first wire, got {:?}", other),
        }
        // a turn-only tick speaks Rot
        let expected_yaw = {
            b.turn(4.0, 0.0, 0.5); // 0.6 degrees
            b.yaw
        };
        match b.take_move_wire() {
            Some(MoveWire::Rot { yaw, .. }) => assert_eq!(yaw, expected_yaw),
            other => panic!("expected the rot wire, got {:?}", other),
        }
        // a walk tick speaks Pos (no rotation since the last send)
        b.forward = 1.0;
        b.tick(&w, false);
        match b.take_move_wire() {
            Some(MoveWire::Pos { z, on_ground, .. }) => {
                assert!(on_ground);
                assert!(z > 8.0, "the position moved");
                // the bookkeeping: the last-sent matches now
                assert_eq!(b.z_last, z);
            }
            other => panic!("expected the pos wire, got {:?}", other),
        }
        // turning AND walking: PosRot
        let (expected_yaw, expected_pitch) = {
            b.turn(4.0, -2.0, 0.5);
            (b.yaw, b.pitch)
        };
        b.tick(&w, false);
        match b.take_move_wire() {
            Some(MoveWire::PosRot { yaw, pitch, .. }) => {
                assert_eq!(yaw, expected_yaw);
                assert_eq!(pitch, expected_pitch);
            }
            other => panic!("expected the posrot wire, got {:?}", other),
        }
    }

    #[test]
    fn wire_reminder_forces_the_position_every_20_ticks() {
        let w = flat_world();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        let _ = b.take_move_wire(); // the settle status
        // stand still: no wire until the reminder hits 20 (the counter
        // rides from the constructor's zero — the exact tick is pinned
        // by the constant, the quiet stretch lasts until then)
        let mut spoken_pos = 0;
        for i in 0..25 {
            b.tick(&w, false);
            if matches!(b.take_move_wire(), Some(MoveWire::Pos { .. })) {
                spoken_pos += 1;
                assert!(i >= 16, "the reminder spoke at tick {} — too early", i);
                assert!(i <= 20, "the reminder spoke at tick {} — too late", i);
                break;
            }
        }
        assert_eq!(spoken_pos, 1, "the reminder forces the position");
    }

    #[test]
    fn the_crouch_holds_when_the_ceiling_blocks_the_stand() {
        // a 1.5-clearance pocket: the crouch fits, the stand does not
        let mut w = flat_world();
        w.set(8, 2, 8, crate::world::STONE); // the ceiling slab [2,3]
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        b.want_crouch = true;
        b.tick(&w, false);
        assert!(b.crouching);
        b.want_crouch = false; // the key released
        b.tick(&w, false);
        assert!(b.crouching, "the low ceiling keeps the pose crouched");
        // in the open the pose stands back up (the SECOND body lives
        // on a ceiling-free column — the pocket world above keeps its
        // own body crouched forever)
        let mut b = PlayerBody::new(5.0, 1.0, 5.0, 0.0, 0.0);
        settle(&mut b, &w);
        b.want_crouch = true;
        b.tick(&w, false);
        b.want_crouch = false;
        b.tick(&w, false);
        assert!(!b.crouching, "the open sky stands the pose up");
    }

    #[test]
    fn camera_conversion_is_the_snapshot_inverse() {
        let b = PlayerBody::new(1.0, 2.0, 3.0, 270.0, -10.0);
        let cam = b.camera();
        // the feet + the standing eye
        assert_eq!(cam.pos.y, 2.0 + STANDING_EYE as f32);
        // the yaw 270 -> rad + pi = 4.712 + 3.1416 = 7.854 = 2.5pi
        assert!((cam.yaw - (270.0f32.to_radians() + std::f32::consts::PI)).abs() < 1e-6);
        // the pitch -10 -> +10 deg in rad
        assert!((cam.pitch - 10.0f32.to_radians()).abs() < 1e-6);
        // the round trip through wire_angles restores the vanilla
        // degrees
        let (yaw, pitch) = crate::world_view::wire_angles(&cam);
        assert!((yaw - 270.0).abs() < 1e-3, "yaw {}", yaw);
        assert!((pitch - -10.0).abs() < 1e-3, "pitch {}", pitch);
    }

    #[test]
    fn crouch_pose_changes_the_heights_and_the_collision() {
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        assert_eq!(b.height(), STANDING_HEIGHT);
        assert_eq!(b.eye_height(), STANDING_EYE);
        b.crouching = true;
        assert_eq!(b.height(), CROUCH_HEIGHT);
        assert_eq!(b.eye_height(), CROUCH_EYE, "the Avatar POSES crouch eye");
    }
}

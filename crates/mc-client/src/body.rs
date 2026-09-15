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

use crate::math::Vec3;
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

// THE FALL-DAMAGE ARC's constants (javap, the same jar):
/// `Attributes.SAFE_FALL_DISTANCE`'s default — the fall's own grace.
/// javap `LivingEntity.calculateFallPower`: `(distance + 1.0E-6) -
/// getAttributeValue(SAFE_FALL_DISTANCE)`.
pub const SAFE_FALL_DISTANCE: f64 = 3.0;
/// `calculateFallPower`'s own epsilon (the ldc2_w 1.0E-6d) — the
/// exactly-three-block fall stays safe through the floor.
pub const FALL_POWER_EPSILON: f64 = 1.0e-6;
/// `Attributes.FALL_DAMAGE_MULTIPLIER`'s default — the attribute the
/// damage law multiplies through (the MobEffects slow-falling's -1.0
/// rides it; our vocabulary carries no effects yet).
pub const FALL_DAMAGE_MULTIPLIER: f64 = 1.0;
/// `Entity.checkFallDistanceAccumulation`'s own gate — the rising
/// body's accumulation clamp threshold (the deltaMovement.y() the
/// law compares: `> -0.5` while `fallDistance > 1.0` folds it to 1.0).
pub const FALL_ACCUM_VY_GATE: f64 = -0.5;

/// The landed block's fall-damage multiplier — javap
/// `BlockBehaviour.getFallDamageMultiplier`'s default 1.0 (the
/// overrides: slime block 0.0, hay/bed/honey 0.2 — none in this
/// client's palette, the honest absence the table's single row
/// speaks: every palette block multiplies by 1.0, the water never
/// accumulates at all).
pub fn fall_damage_multiplier(_block: u16) -> f64 {
    1.0
}

/// THE FALL DAMAGE LAW (javap `LivingEntity.calculateFallDamage`):
/// `Mth.floor(fallPower * blockMultiplier * FALL_DAMAGE_MULTIPLIER)`
/// where `fallPower = (distance + 1.0E-6) - SAFE_FALL_DISTANCE`
/// (the attribute defaults 3.0 / 1.0). The EXACTLY-3-block fall
/// stays safe (the epsilon pushes power to 1e-6, the floor floors
/// it to 0); every full block past 3.0 speaks one damage.
pub fn calculate_fall_damage(distance: f64, block_multiplier: f64) -> i32 {
    let power = (distance + FALL_POWER_EPSILON) - SAFE_FALL_DISTANCE;
    (power * block_multiplier * FALL_DAMAGE_MULTIPLIER).floor() as i32
}

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
/// the hit, the air is the air). The drop lane's un-stuck and the
/// item physics speak the same law.
pub fn solid(world: &World, x: i32, y: i32, z: i32) -> bool {
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

/// THE WALK ANIMATION STORE (javap `WalkAnimationState`, the field
/// values verbatim): the speed blends toward the tick's move distance
/// (`speed += (target - speed) * 0.4`), the position accumulates the
/// blended speed, and the renderer samples both at the frame's partial
/// tick. `position_scale` rides the isBaby ? 3.0 : 1.0 fold (the
/// player is never a baby — 1.0 stands).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WalkAnimationState {
    pub speed_old: f32,
    pub speed: f32,
    pub position: f32,
    pub position_scale: f32,
}

impl WalkAnimationState {
    /// `setSpeed(f)` — the hurt lane's own kick
    /// (`LivingEntity.handleDamageEvent`: `walkAnimation.setSpeed(1.5)`).
    pub fn set_speed(&mut self, f: f32) {
        self.speed = f;
    }

    /// `update(f, partial, scale)`: speedOld = speed, the 0.4 blend
    /// toward `f`, `position += speed`, the scale stored.
    pub fn update(&mut self, f: f32, partial: f32, scale: f32) {
        self.speed_old = self.speed;
        self.speed += (f - self.speed) * partial;
        self.position += self.speed;
        self.position_scale = scale;
    }

    /// `stop()`: all three zero.
    pub fn stop(&mut self) {
        self.speed_old = 0.0;
        self.speed = 0.0;
        self.position = 0.0;
    }

    /// `speed(partial)`: `Math.min(Math.abs(speedOld + (speed -
    /// speedOld) * partial), 1.0)` — the render state's
    /// walkAnimationSpeed.
    pub fn speed_at(&self, partial: f32) -> f32 {
        let blend = self.speed_old + (self.speed - self.speed_old) * partial;
        blend.abs().min(1.0)
    }

    /// `position(partial)`: `(position - speed * (1 - partial)) *
    /// positionScale` — the backward-extrapolated walk phase the
    /// setupAnim cosines speak.
    pub fn position_at(&self, partial: f32) -> f32 {
        (self.position - self.speed * (1.0 - partial)) * self.position_scale
    }
}

/// The walk-distance law's own constants (`LivingEntity
/// .calculateEntityAnimation` / `.updateWalkAnimation`, javap): the
/// tick's 3D move distance feeds `min(distance * 4.0, 1.0)` into the
/// store's 0.4 blend.
pub const WALK_DISTANCE_SCALE: f32 = 4.0;
pub const WALK_BLEND: f32 = 0.4;
/// `LivingEntity.handleDamageEvent`'s own flinch: the hurt kick lands
/// the walk speed at 1.5 (the limbs flail through the hurt tick).
pub const HURT_WALK_SPEED: f32 = 1.5;

/// The `Level.clip` stand-in the detached camera's `getMaxZoom` speaks
/// (the `ClipContext.Block.VISUAL` / `Fluid.NONE` lane over this
/// client's full-cube `solid()` vocabulary): the ray's first solid-cell
/// ENTRY point, the Amanatides-Woo voxel walk `BlockGetter.clip`
/// performs. `None` = the ray misses every solid cell (the full zoom
/// stands). A ray STARTING inside a solid cell hits at its own start
/// (the shape's interior clip verdict).
pub fn clip_ray(world: &World, from: Vec3, to: Vec3) -> Option<Vec3> {
    let dx = (to.x - from.x) as f64;
    let dy = (to.y - from.y) as f64;
    let dz = (to.z - from.z) as f64;
    let len = (dx * dx + dy * dy + dz * dz).sqrt();
    if len < 1e-9 {
        let (cx, cy, cz) = (from.x.floor() as i32, from.y.floor() as i32, from.z.floor() as i32);
        return if solid(world, cx, cy, cz) { Some(from) } else { None };
    }
    let (fx, fy, fz) = (from.x as f64, from.y as f64, from.z as f64);
    let (mut cx, mut cy, mut cz) = (fx.floor() as i32, fy.floor() as i32, fz.floor() as i32);
    let step_x = if dx > 0.0 { 1 } else { -1 };
    let step_y = if dy > 0.0 { 1 } else { -1 };
    let step_z = if dz > 0.0 { 1 } else { -1 };
    // t to the next x/y/z boundary, and the t span per whole cell
    let mut t_max_x = if dx != 0.0 {
        ((if dx > 0.0 { cx as f64 + 1.0 - fx } else { fx - cx as f64 }) / dx.abs())
    } else {
        f64::INFINITY
    };
    let mut t_max_y = if dy != 0.0 {
        ((if dy > 0.0 { cy as f64 + 1.0 - fy } else { fy - cy as f64 }) / dy.abs())
    } else {
        f64::INFINITY
    };
    let mut t_max_z = if dz != 0.0 {
        ((if dz > 0.0 { cz as f64 + 1.0 - fz } else { fz - cz as f64 }) / dz.abs())
    } else {
        f64::INFINITY
    };
    let t_delta_x = if dx != 0.0 { 1.0 / dx.abs() } else { f64::INFINITY };
    let t_delta_y = if dy != 0.0 { 1.0 / dy.abs() } else { f64::INFINITY };
    let t_delta_z = if dz != 0.0 { 1.0 / dz.abs() } else { f64::INFINITY };
    let unit = Vec3::new((dx / len) as f32, (dy / len) as f32, (dz / len) as f32);
    // the start cell speaks first (the interior clip verdict)
    if solid(world, cx, cy, cz) {
        return Some(from);
    }
    loop {
        // step to the next cell boundary
        let (t_next, axis) = if t_max_x < t_max_y && t_max_x < t_max_z {
            (t_max_x, 0)
        } else if t_max_y < t_max_z {
            (t_max_y, 1)
        } else {
            (t_max_z, 2)
        };
        if t_next > len || !t_next.is_finite() {
            return None; // the ray ended inside the last open cell
        }
        match axis {
            0 => {
                cx += step_x;
                t_max_x += t_delta_x;
            }
            1 => {
                cy += step_y;
                t_max_y += t_delta_y;
            }
            _ => {
                cz += step_z;
                t_max_z += t_delta_z;
            }
        }
        if solid(world, cx, cy, cz) {
            // the t parameter spans the UNNORMALIZED ray vector (t=1 is
            // the full from->to span) — the hit point scales by t*len
            // over the unit direction
            let hit_t = t_next * len;
            return Some(from.add(unit.scale(hit_t as f32)));
        }
    }
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
    /// THE WADER'S FLAVOR (Player.playStepSound's own isInWater
    /// branch): true = the step_due fired while the body touches
    /// water — the drain speaks the swim voice + the MUFFLED block
    /// step instead of the plain one (the override's own pair, the
    /// waterSwimSound then playMuffledStepSound order). Cleared with
    /// step_due.
    pub step_in_water: bool,
    /// THE DRIFT'S OWN CROSSING (Entity.move's else-if water branch —
    /// the ground verdict failed but the body is in water): the
    /// crossing resets nextStep and speaks the swim voice alone (the
    /// wading lane's step never spoke here — the airborne drift rides
    /// this branch). The App clears it after the sound fold.
    pub swim_due: bool,
    /// THE SWIM VOICE'S OWN VOLUME (waterSwimSound's fold): min(1.0,
    /// sqrt(dx²·0.2 + dy² + dz²·0.2) · 0.35) over the collided
    /// movement — the getDeltaMovement read at the sound block is the
    /// collided delta the moveDist law itself speaks. Set at each
    /// water firing (the wading step and the drift speak the same
    /// law), read by the drain's play_swim call.
    pub swim_volume: f32,
    /// THE TOUCHING-WATER STATE (Entity.wasTouchingWater's own field,
    /// updateFluidInteraction's tail write): the body's fluid verdict
    /// last tick carried — the ENTRY SPLASH's own gate reads it.
    pub was_touching_water: bool,
    /// THE FIRST-TICK GATE (Entity.firstTick's own field): a body
    /// born in the water never splashes — the spawn-in-a-lake rhythm
    /// stays silent the vanilla's own way. Cleared at the tick's end.
    pub first_tick: bool,
    /// THE ENTRY SPLASH (doWaterSplashEffect's own verdict, fired
    /// through updateFluidInteraction's gate inWater &&
    /// !wasTouchingWater && !firstTick): the water's big welcome. The
    /// App clears it after the sound fold.
    pub splash_due: bool,
    /// THE SPEED VARIANT (the < 0.25 gate): false speaks the plain
    /// PLAYER_SPLASH, true the HIGH_SPEED one — the fast fall's own
    /// louder welcome.
    pub splash_high_speed: bool,
    /// THE SPLASH VOLUME (doWaterSplashEffect's fold): min(1.0,
    /// d2f(sqrt(dx²·0.2 + dy² + dz²·0.2)) * 0.2F) over the CARRIED
    /// velocity — the baseTick read, last move's post-collision
    /// result, the entry speed the body fell in with.
    pub splash_volume: f32,
    /// THE WALK ANIMATION STORE (WalkAnimationState, javap): the
    /// speed/position pair the third-person avatar's limb cosines
    /// speak. Updated every tick from the 3D move distance (the
    /// calculateEntityAnimation law), kicked to 1.5 by the hurt lane.
    pub walk: WalkAnimationState,
    /// THE FALL DISTANCE (Entity.fallDistance's own double): the
    /// descent accumulates `-(dy as f32)` per tick (the f32 round-trip
    /// the bytecode's d2f/f2d speaks), the water lane never
    /// accumulates, the landing folds the damage through
    /// calculateFallDamage and resets (checkFallDamage's own law).
    pub fall_distance: f64,
    /// The landing's damage the fall law spoke this tick (javap
    /// causeFallDamage's `calculateFallDamage > 0` beat) — the App
    /// drains it into the hurt pipeline (the voices, the cooldown, the
    /// health) exactly like the step_due lane it sits beside. `None` =
    /// no hurt this tick; the body never clears it, the reader does.
    pub fall_event: Option<i32>,
    /// The EXHAUSTION SOURCES' own bank this tick (javap
    /// ServerPlayer.jumpFromGround's 0.05/0.2 and
    /// checkMovementStatistics's cm law) — the App drains it into the
    /// stand-in server's FoodData (the offline lane's own authority;
    /// the networked lane's exhaustion is the server's observation,
    /// the client folds nothing). The body never clears it, the reader
    /// does.
    pub exhaustion_event: f32,
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
            step_in_water: false,
            swim_due: false,
            swim_volume: 0.0,
            was_touching_water: false,
            first_tick: true,
            splash_due: false,
            splash_high_speed: false,
            splash_volume: 0.0,
            walk: WalkAnimationState {
                speed_old: 0.0,
                speed: 0.0,
                position: 0.0,
                position_scale: 1.0,
            },
            fall_distance: 0.0,
            fall_event: None,
            exhaustion_event: 0.0,
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
        // the per-tick event flags clear at the head (the App drains
        // what THIS tick spoke): the splash's own clear rides here —
        // the fluid update fires it AFTER this line, the step block's
        // own clears never see it
        self.splash_due = false;
        // the pre-tick position — the walk distance law's own source
        // (calculateEntityAnimation's getX() - xo fold)
        let (x0, y0, z0) = (self.x, self.y, self.z);
        // THE RISING BODY'S ACCUMULATION CLAMP (javap
        // LivingEntity.updateFallFlying's own first call —
        // checkFallDistanceAccumulation: `vy > -0.5 && fallDistance >
        // 1.0` folds the banked descent to 1.0 — the launch arc's slow
        // apex can never bank more than one block of fall)
        if self.vy > FALL_ACCUM_VY_GATE && self.fall_distance > 1.0 {
            self.fall_distance = 1.0;
        }
        // THE FLUID LANE'S OWN UPDATE (javap
        // Entity.updateFluidInteraction: the per-tick fluid scan ->
        // `isInFluid(WATER) || isInFluid(LAVA)` -> resetFallDistance —
        // every tick in a fluid the banked descent folds to zero, the
        // water landing never hurts — AND the ENTRY SPLASH's own gate:
        // inWater && !wasTouchingWater && !firstTick ->
        // doWaterSplashEffect, the tail write wasTouchingWater =
        // inWater)
        let in_water = self.touches_water(world);
        if in_water {
            self.fall_distance = 0.0;
            if !self.was_touching_water && !self.first_tick {
                // THE ENTRY SPLASH (doWaterSplashEffect's own law): the
                // self volume base 0.2F (the passenger's 0.9F aside —
                // the client player carries no rider), v = min(1.0,
                // d2f(sqrt(dx²·0.2 + dy² + dz²·0.2)) * 0.2F) over the
                // CARRIED velocity (the baseTick read — last move's
                // post-collision result, the entry speed the body fell
                // in with); the < 0.25 gate picks the plain PLAYER_SPLASH,
                // the >= 0.25 the high-speed variant, the pitch 1.0 +
                // (r - r) * 0.4F both ways. The splash PARTICLES' spread
                // loop (the dimensions' width * 20 walk) has no
                // vocabulary here — no particle lane.
                let v = ((((self.vx * self.vx * 0.2
                    + self.vy * self.vy
                    + self.vz * self.vz * 0.2)
                    .sqrt()) as f32)
                    * 0.2)
                    .min(1.0);
                self.splash_due = true;
                self.splash_volume = v;
                self.splash_high_speed = v >= 0.25;
            }
        }
        self.was_touching_water = in_water;
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
            // THE JUMP'S OWN COST (javap ServerPlayer.jumpFromGround: the
            // super's arc, then the isSprinting split — the sprint
            // jump's 0.2, the plain jump's 0.05, FoodConstants' own
            // inlined constants the bytecode speaks)
            self.exhaustion_event += if self.sprinting {
                crate::food::EXHAUSTION_SPRINT_JUMP
            } else {
                crate::food::EXHAUSTION_JUMP
            };
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
        // THE FALL DISTANCE (doCheckFallDamage's own law over the SWEPT
        // delta — the accumulation, the water's honest gate, the
        // landing's fold-and-reset)
        self.check_fall_damage(world, col.dy);
        // THE FOOTSTEP ACCUMULATION (Entity.move's own law over the
        // ACTUAL moved delta): moveDist += horizontalDistance × 0.6 (the
        // climbable branch's full-length lane aside — our vocabulary
        // climbs nothing yet); the step fires when moveDist > nextStep
        // AND the block under the feet (getOnPos: floor(x), floor(y -
        // 0.2), floor(z)) is not air — the vanilla's own isAir gate —
        // and the counter resets to (int)moveDist + 1.0F.
        self.step_due = false;
        self.swim_due = false;
        let horizontal = ((col.dx * col.dx + col.dz * col.dz).sqrt()) as f32;
        // THE MOVEMENT'S OWN COST (javap ServerPlayer.checkMovementStatistics
        // over the stats' cm law: cm = Math.round(distance * 100), the
        // exhaustion rate * cm * 0.01 — the in-water lane's 0.01 per
        // block over the horizontal sweep, the on-ground sprint lane's
        // 0.1 per block, the walk/crouch lanes' honest 0.0 constants;
        // the lanes' own order: the water beats the ground, the water
        // gate rides the BODY's fluid state the tick already knows)
        let cm = (horizontal * 100.0).round() as i32;
        if cm > 0 {
            if self.touches_water(world) {
                self.exhaustion_event +=
                    crate::food::EXHAUSTION_SWIM * cm as f32 * 0.01;
            } else if self.on_ground && self.sprinting {
                self.exhaustion_event +=
                    crate::food::EXHAUSTION_SPRINT * cm as f32 * 0.01;
            }
            // the WALK and CROUCH lanes' EXHAUSTION_WALK /
            // EXHAUSTION_CROUCH — the modern law's own 0.0, the
            // constants the walk speaks and the arithmetic drops
        }
        self.move_dist += horizontal * 0.6;
        if self.move_dist > self.next_step {
            let under = world.get(
                self.x.floor() as i32,
                (self.y - 0.2).floor() as i32,
                self.z.floor() as i32,
            );
            // THE SWIM VOICE'S OWN VOLUME (waterSwimSound's own fold
            // over getDeltaMovement — the read lands at the sound
            // block, the collided movement the moveDist law itself
            // speaks): min(1.0, d2f(sqrt(dx²·0.2 + dy² + dz²·0.2)) *
            // 0.35F) — the bytecode's own d2f-then-fmul order, the y
            // axis riding un-attenuated (the swim bob's own emphasis).
            let swim_vol = (((col.dx * col.dx * 0.2
                + col.dy * col.dy
                + col.dz * col.dz * 0.2)
                .sqrt()) as f32
                * 0.35)
                .min(1.0);
            // THE GROUND GATE (vibrationAndSoundEffectsFromBlock's own
            // verdict: onGround, the climbable/rails branches aside —
            // the crouch-hover branch (crouching && dy == 0.0) speaks
            // for the sneaker; the airborne walk NEVER steps, the
            // accumulated crossing waits for the landing). The reset
            // rides nextStep() = (int)moveDist + 1.0F. The step itself
            // fires THROUGH Player.playStepSound's override: in water
            // the wader hears the swim voice + the MUFFLED block step
            // (the drain folds the pair), the dry walk the plain one.
            if under != AIR && (self.on_ground || (self.crouching && col.dy == 0.0)) {
                self.next_step = (self.move_dist as i32) as f32 + 1.0;
                self.step_due = true;
                // THE WADER'S FLAVOR (the override's own isInWater
                // branch): the flag rides the body's fluid state at
                // the firing moment — wasTouchingWater the bytecode
                // reads
                self.step_in_water = self.touches_water(world);
                if self.step_in_water {
                    self.swim_volume = swim_vol;
                }
            } else if self.touches_water(world) {
                // THE WATER BRANCH (move's else-if: the verdict failed,
                // the body drifts IN water): the crossing resets and
                // speaks the swim voice alone (emitsSounds' own gate —
                // the player's emission is ALL). The gameEvent(SWIM)
                // lane has no vocabulary here (no sculk sensors).
                self.next_step = (self.move_dist as i32) as f32 + 1.0;
                self.swim_due = true;
                self.swim_volume = swim_vol;
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
        // THE WALK ANIMATION (LivingEntity.calculateEntityAnimation(true)
        // + updateWalkAnimation, javap): the tick's 3D move distance,
        // `min(distance * 4.0, 1.0)` into the store's 0.4 blend.
        let dist = ((self.x - x0) * (self.x - x0)
            + (self.y - y0) * (self.y - y0)
            + (self.z - z0) * (self.z - z0))
            .sqrt() as f32;
        if dist.is_finite() {
            self.walk.update((dist * WALK_DISTANCE_SCALE).min(1.0), WALK_BLEND, 1.0);
        } else {
            self.walk.stop();
        }
        // THE FIRST-TICK GATE (Entity.firstTick's own clearing): the
        // body's first 50ms beat spoke its lanes splash-free — the
        // gate opens
        self.first_tick = false;
    }

    /// THE FALL DISTANCE LAW (javap `Entity.checkFallDamage` over the
    /// swept delta, the `doCheckFallDamage` final wrapper's own shape):
    /// the water lane never accumulates; a descent accumulates
    /// `fallDistance -= (float)dy` (the d2f/f2d round-trip the
    /// bytecode speaks); the landing folds the banked descent through
    /// `fallOn` -> `calculateFallDamage` (the damage lands in
    /// `fall_event` for the App's hurt pipeline), plays the
    /// hit-ground game event's beat, and resets. The landed block
    /// names through `mainSupportingBlockPos`'s honest stand-in — the
    /// exact-boundary feet read the cell BELOW (the collision sweep's
    /// own verdict).
    fn check_fall_damage(&mut self, world: &World, dy: f64) {
        let in_water = self.touches_water(world);
        if !in_water && dy < 0.0 {
            self.fall_distance -= (dy as f32) as f64;
        }
        if self.on_ground {
            if self.fall_distance > 0.0 {
                // the landed block (the feet's exact boundary reads the
                // cell below — the support position the sweep named)
                let land_y = if self.y == self.y.floor() {
                    self.y as i32 - 1
                } else {
                    self.y.floor() as i32
                };
                let block = world.get(self.x.floor() as i32, land_y, self.z.floor() as i32);
                let dmg = calculate_fall_damage(
                    self.fall_distance,
                    fall_damage_multiplier(block),
                );
                if dmg > 0 {
                    self.fall_event = Some(dmg);
                }
            }
            self.fall_distance = 0.0;
        }
    }

    /// The water lane's own gate (the vanilla `isInWater`'s box shape):
    /// TRUE when any cell the body box overlaps is water — the
    /// palette's own fluid kind.
    fn touches_water(&self, world: &World) -> bool {
        let (x0, y0, z0, x1, y1, z1) = box_at(self.x, self.y, self.z, self.height());
        for cy in y0.floor() as i32..=((y1 - 1e-9).floor()) as i32 {
            for cx in x0.floor() as i32..=((x1 - 1e-9).floor()) as i32 {
                for cz in z0.floor() as i32..=((z1 - 1e-9).floor()) as i32 {
                    if world.get(cx, cy, cz) == WATER {
                        return true;
                    }
                }
            }
        }
        false
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
        // THE FALL LAW's own verdict: the exactly-3-block descent (the
        // feet at 4.0, the floor at 1.0) lands SAFE — the epsilon keeps
        // the floor() at zero, the vanilla's own grace
        assert_eq!(b.fall_distance, 0.0, "the landing reset the banked descent");
        assert_eq!(b.fall_event, None, "the 3-block fall never hurts");
    }

    /// THE FALL DAMAGE TABLE (javap calculateFallDamage +
    /// calculateFallPower's own arithmetic):
    /// `floor((distance + 1e-6 - 3.0) * 1.0 * 1.0)` — the safe
    /// exactly-3, the full blocks past 3.0, the half block's floor.
    #[test]
    fn the_fall_damage_table_speaks_the_floor_law() {
        let m = fall_damage_multiplier(crate::world::STONE);
        assert_eq!(m, 1.0, "the palette's stone multiplies by the default");
        // the grace: exactly three blocks and everything below
        assert_eq!(calculate_fall_damage(3.0, m), 0);
        // the LAW's own negatives (the floor of a negative power — the
        // caller's `i > 0` gate refuses them, the hurt never speaks)
        assert_eq!(calculate_fall_damage(1.0, m), -2);
        assert_eq!(calculate_fall_damage(0.0, m), -3);
        // the half block floors away: 3.5 falls safe
        assert_eq!(calculate_fall_damage(3.5, m), 0);
        // every whole block past 3.0 speaks one
        assert_eq!(calculate_fall_damage(4.0, m), 1);
        assert_eq!(calculate_fall_damage(8.0, m), 5);
        assert_eq!(calculate_fall_damage(23.0, m), 20, "the pure law's own kill count");
        // the epsilon's own beat: 2.999999 falls safe, the 4.0 boundary
        // speaks one (the floor's own arithmetic)
        assert_eq!(calculate_fall_damage(2.999999, m), 0);
        assert_eq!(calculate_fall_damage(3.9999999, m), 1);
        assert_eq!(calculate_fall_damage(4.000001, m), 1);
    }

    /// THE LONG FALL SPEAKS (the 10-block descent banks 10.0 through
    /// the swept deltas MINUS the rising clamp's own 0.1435 — the
    /// checkFallDistanceAccumulation law the tick head speaks — so the
    /// landing folds damage 6: floor(9.8565 + 1e-6 - 3.0) = 6, the
    /// bytecode's own arithmetic over the whole arc).
    #[test]
    fn the_ten_block_fall_hurts_seven() {
        let w = flat_world();
        // the feet at 11.0, the floor at 1.0: a 10-block descent
        let mut b = PlayerBody::new(8.0, 11.0, 8.0, 0.0, 0.0);
        let mut banked_mid = 0.0f64;
        for i in 0..60 {
            b.tick(&w, false);
            if i == 12 {
                banked_mid = b.fall_distance;
            }
            if b.fall_event.is_some() {
                break;
            }
        }
        assert_eq!(b.fall_event, Some(6), "floor(banked 9.8565 + 1e-6 - 3.0) = 6");
        assert_eq!(b.fall_distance, 0.0, "the landing reset");
        // the bank grew through the descent (the mid-flight body had a
        // positive bank — the accumulation the law speaks)
        assert!(banked_mid > 0.0, "the descent banks the fall distance");
        // a second landing (the resting body) never speaks again — the
        // event the reader must drain stays the one the landing spoke
        for _ in 0..10 {
            b.tick(&w, false);
        }
        assert_eq!(b.fall_event, Some(6), "the rest is silent");
        assert_eq!(b.fall_distance, 0.0);
    }

    /// THE WATER LANE NEVER HURTS (javap updateFluidInteraction's
    /// per-tick reset + the accumulation gate): the banked descent
    /// folds to zero the moment the body box enters the fluid, the
    /// pool-bottom landing lands safe.
    #[test]
    fn the_water_pool_lands_safe() {
        let mut w = flat_world();
        // a 2-deep pool (y 1..=2) over the stone floor
        for x in 6..10 {
            for z in 6..10 {
                for y in 1..3 {
                    w.set(x, y, z, WATER);
                }
            }
        }
        // the feet at 12.0 above the pool at (8, 8)
        let mut b = PlayerBody::new(8.0, 12.0, 8.0, 0.0, 0.0);
        for _ in 0..200 {
            b.tick(&w, false);
            assert_ne!(
                b.fall_event, Some(1),
                "the water landing never speaks a hurt"
            );
            if b.on_ground {
                break;
            }
        }
        assert!(b.on_ground, "the body settled to the pool floor");
        assert_eq!(b.fall_event, None, "no hurt event ever spoke");
        assert_eq!(b.fall_distance, 0.0, "the fluid's own reset");
    }

    /// THE RISING BODY'S CLAMP (javap checkFallDistanceAccumulation):
    /// `vy > -0.5 && fallDistance > 1.0` folds the bank to 1.0 — the
    /// launch apex can never bank more than one block.
    #[test]
    fn the_rising_clamp_folds_the_bank() {
        let w = flat_world();
        let mut b = PlayerBody::new(8.0, 2.0, 8.0, 0.0, 0.0);
        b.tick(&w, false); // the landing reset folds to 0 first
        b.fall_distance = 5.0; // a banked descent the launch carried
        b.vy = 0.3; // a rising body (vy > -0.5)
        b.tick(&w, false);
        assert_eq!(b.fall_distance, 1.0, "the clamp folded the bank");
        // a FAST faller (vy <= -0.5) never clamps
        let mut c = PlayerBody::new(8.0, 2.0, 8.0, 0.0, 0.0);
        c.tick(&w, false);
        c.fall_distance = 5.0;
        c.vy = -0.8;
        c.tick(&w, false);
        assert!(c.fall_distance > 1.0, "the fast descent banks past 1.0");
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

    /// THE JUMP'S OWN COST (javap ServerPlayer.jumpFromGround): the
    /// sprint jump banks 0.2, the plain jump 0.05 — the FoodConstants
    /// constants the isSprinting split speaks, banked on the SAME tick
    /// the jump arc fired.
    #[test]
    fn the_jump_banks_the_exhaustion_split_by_the_sprint() {
        let w = flat_world();
        // the plain jump: the body settles on the floor first (the
        // jump gate needs the ground state), the walk costs nothing
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        for _ in 0..5 {
            b.tick(&w, false);
        }
        b.jumping = true;
        b.tick(&w, false);
        assert_eq!(b.exhaustion_event, crate::food::EXHAUSTION_JUMP);
        // the sprint jump: the sprint's own 0.2 (the airborne sweep
        // costs nothing — the movement lanes need the ground)
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        for _ in 0..5 {
            b.tick(&w, false);
        }
        b.jumping = true;
        b.sprinting = true;
        b.tick(&w, false);
        assert_eq!(b.exhaustion_event, crate::food::EXHAUSTION_SPRINT_JUMP);
    }

    /// THE SPRINT MOVE'S OWN COST (javap ServerPlayer.checkMovementStatistics
    /// over the stats' cm law): the on-ground sprint lane pays
    /// 0.1 * cm * 0.01 per tick's swept distance — one block's sweep
    /// banks exactly 0.1, the walk lane banks the honest 0.0.
    #[test]
    fn the_sprint_move_costs_a_tenth_per_block_the_walk_is_free() {
        let w = flat_world();
        // the resting body walks nothing — the cm > 0 gate
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        for _ in 0..10 {
            b.tick(&w, false);
        }
        assert_eq!(b.exhaustion_event, 0.0, "the resting body banks nothing");
        // the sprint body's swept delta speaks the cm law — the
        // per-tick sweep s (blocks) costs 0.1 * round(s * 100) * 0.01;
        // drive a real sprint and fold the bank over the run
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        b.sprinting = true;
        b.forward = 1.0;
        let mut swept = 0.0f64;
        for _ in 0..20 {
            let before = (b.x, b.z);
            b.tick(&w, false);
            let dx = b.x - before.0;
            let dz = b.z - before.1;
            swept += (dx * dx + dz * dz).sqrt();
        }
        assert!(b.exhaustion_event > 0.0, "the sprint's own cost speaks");
        // the per-block law's own shape: the bank ≈ 0.1 per swept
        // block (the cm rounding's own error under half a block's
        // worth over the 20-tick run)
        let expected = swept as f32 * 0.1;
        assert!(
            (b.exhaustion_event - expected).abs() < 0.05,
            "the bank {} tracks 0.1/block over {} swept (expected {})",
            b.exhaustion_event,
            swept,
            expected
        );
        // the WALK lane's honest 0.0: the same run without the sprint
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        b.forward = 1.0;
        for _ in 0..20 {
            b.tick(&w, false);
        }
        assert_eq!(b.exhaustion_event, 0.0, "the walk is free (the modern law)");
    }

    /// THE WADER'S OWN VOICE (Player.playStepSound's isInWater branch
    /// over the vibration verdict's playStepSound call): the step on
    /// solid ground while the body touches water fires step_due with
    /// step_in_water — the drain speaks the swim voice + the muffled
    /// block step instead of the plain one — and the swim volume rides
    /// the waterSwimSound fold (min(1, d2f(sqrt(dx²·0.2 + dy² +
    /// dz²·0.2)) * 0.35) over the collided movement). The drift branch
    /// never speaks here (the verdict passed, move's else-if is dead).
    #[test]
    fn the_wading_step_speaks_the_water_override() {
        // the whole flat world is a SHALLOW pool: water at y=1 over the
        // stone floor — the wader walks the pond bottom, every step in
        // the fluid, no edge to ever leave
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 0, z, crate::world::STONE);
                w.set(x, 1, z, WATER);
                for y in 2..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, AIR);
                }
            }
        }
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        assert!(b.on_ground, "the wader stands on the pond bottom");
        assert!(b.touches_water(&w), "the box touches the fluid");
        b.forward = 1.0;
        let mut steps = 0;
        for _ in 0..40 {
            let prev = b.move_dist;
            b.tick(&w, false);
            if b.step_due {
                steps += 1;
                assert!(
                    b.step_in_water,
                    "the wading step carries the water flavor (moveDist {})",
                    b.move_dist
                );
                assert!(!b.swim_due, "the ground verdict passed, the drift branch is dead");
                assert_eq!(
                    b.next_step,
                    (b.move_dist as i32) as f32 + 1.0,
                    "the reset rides nextStep()'s law"
                );
                // THE VOLUME LAW over the collided movement: the flat
                // walk's dy rides 0 (the grounded stride), so the fold
                // reduces to h * sqrt(0.2) * 0.35 — the d2f-then-fmul
                // order, the min(1.0) cap above it
                let h = (b.move_dist - prev) / 0.6;
                let expected = (h * 0.2f32.sqrt() * 0.35).min(1.0);
                assert!(
                    (b.swim_volume - expected).abs() < 1e-4,
                    "the volume {} tracks the fold's {} (h {})",
                    b.swim_volume,
                    expected,
                    h
                );
            }
        }
        assert!(steps >= 2, "the wading walk crossed its steps ({})", steps);
    }

    /// THE DRIFT'S OWN CROSSING (Entity.move's else-if water branch —
    /// the ground verdict failed, the body is in water): the crossing
    /// resets nextStep and flags swim_due — the swim voice alone, no
    /// step, no reset to the plain lane. The airborne-over-land walk
    /// never speaks it (the water gate refuses).
    #[test]
    fn the_drift_swim_speaks_the_water_branch() {
        // a DEEP pool: water y=1..2 over the stone floor — the body
        // dropped mid-pool with a forward push drifts through the
        // fluid before the pond-bottom landing
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 0, z, crate::world::STONE);
                w.set(x, 1, z, WATER);
                w.set(x, 2, z, WATER);
                for y in 3..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, AIR);
                }
            }
        }
        let mut b = PlayerBody::new(8.0, 2.5, 8.0, 0.0, 0.0);
        b.forward = 1.0;
        // the pending crossing the harness carries in: a body that
        // already walked 1.5 blocks before the drift speaks the branch
        // on the first 0.1 of fluid travel (the water's own drag makes
        // the ctor-fresh 1.0F a long wait this test doesn't need)
        b.move_dist = 0.9;
        let mut drift_swims = 0;
        let mut drift_steps = 0;
        let mut swept = 0.0f64;
        for _ in 0..300 {
            let before = (b.x, b.z);
            b.tick(&w, false);
            swept += ((b.x - before.0).powi(2) + (b.z - before.1).powi(2)).sqrt();
            if !b.on_ground {
                if b.swim_due {
                    drift_swims += 1;
                    assert_eq!(
                        b.next_step,
                        (b.move_dist as i32) as f32 + 1.0,
                        "the drift's reset rides the same nextStep() law"
                    );
                    assert!(
                        b.swim_volume > 0.0 && b.swim_volume <= 1.0,
                        "the volume rides the speed fold ({})",
                        b.swim_volume
                    );
                }
                if b.step_due {
                    drift_steps += 1;
                }
            }
        }
        assert!(
            drift_swims >= 1,
            "the drift crossed its swim voice ({} swims, swept {}, move_dist {})",
            drift_swims,
            swept,
            b.move_dist
        );
        assert_eq!(
            drift_steps, 0,
            "the airborne drift never speaks the ground verdict"
        );
    }

    /// THE DRY WALK KEEPS THE PLAIN LANE (the regression the water
    /// lanes must never touch): the solid-ground steps stay dry, the
    /// drift branch never speaks over land, the airborne crossing
    /// stays pending for the landing.
    #[test]
    fn the_dry_walk_keeps_the_plain_lane() {
        let mut w = World::new(4, 42);
        for x in 0..64 {
            for z in 0..64 {
                w.set(x, 0, z, crate::world::STONE);
                for y in 1..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, AIR);
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
                assert!(!b.step_in_water, "the dry step never carries the water flavor");
            }
            assert!(
                !b.swim_due,
                "the drift branch never speaks over dry land"
            );
        }
        assert!(steps >= 2, "the dry walk crossed its steps ({})", steps);
    }

    /// THE ENTRY SPLASH (updateFluidInteraction's own gate over
    /// doWaterSplashEffect's law): the tall fall into the pool speaks
    /// ONE splash at the entry tick — the carried velocity's own fold
    /// (min(1, d2f(sqrt(dx²·0.2 + dy² + dz²·0.2)) * 0.2F)), the tall
    /// fall's speed picks the HIGH_SPEED variant (>= 0.25), and the
    /// in-water ticks after never speak it again.
    #[test]
    fn the_tall_entry_splashes_once_high_speed() {
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 0, z, crate::world::STONE);
                for y in 1..4 {
                    w.set(x, y as i32, z, WATER);
                }
                for y in 4..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, AIR);
                }
            }
        }
        let mut b = PlayerBody::new(8.0, 20.0, 8.0, 0.0, 0.0);
        let mut splashes = 0;
        let mut tick = 0;
        while tick < 400 {
            let (prev_vx, prev_vy, prev_vz) = (b.vx, b.vy, b.vz);
            b.tick(&w, false);
            tick += 1;
            if b.splash_due {
                splashes += 1;
                assert!(
                    splashes == 1,
                    "the entry speaks once ({splashes} splashes)"
                );
                // THE VOLUME LAW over the carried velocity — the
                // straight drop's vx/vz ride zero, the fold reduces to
                // |vy| * 0.2
                let expected = ((prev_vx * prev_vx * 0.2
                    + prev_vy * prev_vy
                    + prev_vz * prev_vz * 0.2)
                    .sqrt() as f32
                    * 0.2)
                    .min(1.0);
                assert!(
                    (b.splash_volume - expected).abs() < 1e-5,
                    "the volume {} tracks the fold's {} (vy {})",
                    b.splash_volume,
                    expected,
                    prev_vy
                );
                // THE TALL FALL's own speed: the entry vy past the
                // 0.25 gate (volume 0.05 = vy 0.25), the HIGH_SPEED
                // variant speaks
                assert!(
                    prev_vy < -1.0,
                    "the tall fall carried speed in ({})",
                    prev_vy
                );
                assert!(
                    b.splash_high_speed,
                    "the tall fall's splash rides the high-speed variant (v {})",
                    b.splash_volume
                );
            }
            if b.on_ground && splashes == 1 {
                break;
            }
        }
        assert_eq!(splashes, 1, "the entry spoke exactly once");
        assert!(b.on_ground, "the body landed on the pool floor");
        assert_eq!(b.fall_event, None, "the water landing never hurts");
    }

    /// THE BORN-WET BODY STAYS SILENT (the firstTick gate + the
    /// wasTouchingWater carry): a body spawned inside the water never
    /// splashes — not on the first tick, not after.
    #[test]
    fn the_born_wet_body_never_splashes() {
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 0, z, crate::world::STONE);
                for y in 1..3 {
                    w.set(x, y as i32, z, WATER);
                }
                for y in 3..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, AIR);
                }
            }
        }
        let mut b = PlayerBody::new(8.0, 2.5, 8.0, 0.0, 0.0);
        assert!(b.touches_water(&w), "the spawn sits in the fluid");
        for _ in 0..30 {
            b.tick(&w, false);
            assert!(!b.splash_due, "the born-wet body stays silent");
        }
        assert!(!b.first_tick, "the gate opened after tick one");
        assert!(b.was_touching_water, "the fluid state carried");
    }

    /// THE EXIT AND THE RE-ENTRY (the wasTouchingWater carry's own
    /// shape): walking out of the water never splashes, walking back
    /// in speaks the splash again — the slow wade-in rides the PLAIN
    /// variant (the entry speed under the 0.25 gate).
    #[test]
    fn the_reentry_splashes_the_plain_variant() {
        // the west half is a shallow pool, the east half dry stone
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 0, z, crate::world::STONE);
                if x < 4 {
                    w.set(x, 1, z, WATER);
                } else {
                    // the procedural terrain's own y=1 never cleared
                    // itself — the dry side's floor line must speak air
                    w.set(x, 1, z, AIR);
                }
                for y in 2..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, AIR);
                }
            }
        }
        let mut b = PlayerBody::new(3.0, 1.0, 8.0, -90.0, 0.0); // facing east (+x)
        settle(&mut b, &w);
        assert!(b.touches_water(&w), "the wader starts in the pool");
        // walk east — the exit crosses (the water's own drag makes the
        // wade a slow one — the box must clear x=4's column entirely)
        b.forward = 1.0;
        let mut splashes = 0;
        let mut exited = false;
        for _ in 0..400 {
            b.tick(&w, false);
            if b.splash_due {
                splashes += 1;
            }
            if !b.touches_water(&w) && !exited {
                exited = true;
                assert!(!b.splash_due, "the exit never splashes");
            }
            // the dry walk is fast — stop the eastward run before the
            // body walks off into the world's own void
            if exited && b.x > 8.0 {
                break;
            }
        }
        assert!(b.x > 3.0, "the walk moved east ({})", b.x);
        assert!(exited, "the walk left the pool (x {})", b.x);
        assert_eq!(splashes, 0, "the exit stays silent");
        // turn west, walk back — the re-entry speaks the PLAIN variant
        b.yaw = 90.0; // facing west (-x)
        let mut reentry_splash = false;
        for _ in 0..400 {
            b.tick(&w, false);
            if b.splash_due {
                reentry_splash = true;
                assert!(
                    !b.splash_high_speed,
                    "the slow wade-in speaks the plain variant (v {})",
                    b.splash_volume
                );
                assert!(
                    b.splash_volume < 0.25,
                    "the volume sits under the gate ({})",
                    b.splash_volume
                );
            }
        }
        assert!(reentry_splash, "the re-entry spoke its splash");
    }
}

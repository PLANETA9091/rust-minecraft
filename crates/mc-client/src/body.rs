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
/// `Camera.tickFov`'s own words (the 26.2 jar's raw disasm, the
/// scripts/camera_tickfov.txt dump): the half-step the tick arms toward
/// the getFieldOfViewModifier target, then the 0.1/1.5 clamp the
/// bytecode pins.
pub const FOV_TICK_BLEND: f32 = 0.5;
pub const FOV_MODIFIER_MIN: f32 = 0.10000000149011612;
pub const FOV_MODIFIER_MAX: f32 = 1.5;
pub const BLOCK_FRICTION: f32 = 0.6;
pub const AIR_FRICTION: f32 = 1.0;
pub const GROUND_DRAG: f32 = 0.91; // the horizontal drag's constant part
pub const VERTICAL_DRAG: f32 = 0.98;
pub const GRAVITY: f64 = 0.08;
/// `LivingEntity.getWaterSlowDown`'s own constant — the `ldc_w 0.8f`
/// the pcs 0-3 freturn returns (the travelInWater drag seed the
/// non-sprint body wears).
pub const WATER_DRAG: f32 = 0.8;
/// travelInWater's own sprint arm — the `ldc_w 0.9f` pc 7 stores
/// (the sprint-swim the faster decay seed keeps).
pub const WATER_DRAG_SPRINT: f32 = 0.9;
/// travelInWater's own Dolphins Grace override — the `ldc_w 0.96f`
/// pc 95 stores (the drag the effect's glide speaks — the swim the
/// ×0.04 decay the vanilla stride carries).
pub const DOLPHINS_GRACE_WATER_DRAG: f32 = 0.96;
/// travelInWater's own input seed — the `ldc_w 0.02f` pc 19 stores
/// (the WATER_MOVEMENT_EFFICIENCY lerp the bare player's 0.0
/// attribute default never folds — the depth-strider enchant the
/// stacks' data absence keeps, the sprint body the SAME seed wears).
pub const WATER_INPUT_SPEED: f32 = 0.02;
pub const FLYING_SPEED: f32 = 0.02;
/// THE FLY SPEED (javap Abilities <init> the 26.2 jar speaks — the
/// `ldc #13` the flyingSpeed field's constructor default carries:
/// 0.05). The air-control speed the flying body's horizontal impulse
/// rides (Player.getFlyingSpeed's flying branch) AND the vertical
/// thrust's own scale word (LocalPlayer.aiStep pcs 881-888 — the
/// abilities' getFlyingSpeed the thrust multiplies, never the
/// Player's override).
pub const FLY_SPEED: f32 = 0.05;
/// THE SPRINT-FLY SPEED (javap Player.getFlyingSpeed the 26.2 jar —
/// the flying branch's own split: `isSprinting() -> abilities
/// .getFlyingSpeed() * 2.0` the fconst_2 fold carries, the plain fly
/// 0.05 the else arm keeps — the FOLD the field the wheel's own
/// adjustment rides, the 0.1 the DEFAULT field's ×2 value answers: the
/// constant the fold's own shape the `flying_speed * 2.0` walk
/// carries).
/// THE SPRINT-AIR SPEED (javap Player.getFlyingSpeed the 26.2 jar —
/// the NOT-flying branch's own split: `isSprinting() -> ldc #2292`
/// = 0.026, the plain walk-air the #55 = 0.02 arm keeps — the
/// sprint-jump's longer arc the +30% air control speaks).
pub const SPRINT_AIR_SPEED: f32 = 0.026;
/// THE FLY THRUST SCALE (javap LocalPlayer.aiStep the 26.2 jar —
/// pcs 815-894: the int local the shift/jump keys fold ±1, the
/// `i2f * abilities.getFlyingSpeed() * ldc_w #1270` fold the delta
/// y adds — the ×3.0 the constant pool speaks).
pub const FLY_THRUST_SCALE: f64 = 3.0;
/// THE FLY VY RESTORE (javap Player.travel the 26.2 jar — pcs
/// 121-163: the pre-travel delta y the flying branch saves, the
/// super walk the gravity+drag fold spends, the
/// `with(Axis.Y, d2 * ldc2_w #1640)` the carried vy REPLACES —
/// the ×0.6 the hover law speaks: the gravity the fold spent dies
/// in the overwrite, the carried vy decays 0.6 a tick).
pub const FLY_VY_RESTORE: f64 = 0.6;
/// THE FLY FOV (javap AbstractClientPlayer.getFieldOfViewModifier the
/// 26.2 jar — pcs 2-16: `f = 1.0; if abilities.flying f *= ldc #165`
/// = 1.1, the walking-speed ratio fold the product continues — the
/// flying camera's own zoom-out the sprint ratio multiplies).
pub const FLY_FOV: f32 = 1.1;
pub const JUMP_POWER: f32 = 0.42; // JUMP_STRENGTH

/// THE SPEED EFFECT'S OWN MOVEMENT MODIFIER (javap MobEffects clinit
/// pc 22 — the `double 0.20000000298023224` the SPEED row's
/// addAttributeModifier(MOVEMENT_SPEED, "effect.speed", ..,
/// ADD_MULTIPLIED_TOTAL) call carries; the AttributeTemplate.create
/// pc 8-16 the amount the `(amplifier + 1)` factor scales — the
/// f64-as-f32 form the f32 fold keeps).
pub const SPEED_EFFECT_MOVEMENT_MODIFIER: f32 = 0.2;

/// THE SLOWNESS EFFECT'S OWN MOVEMENT MODIFIER (javap MobEffects
/// clinit pc 59 — the `double -0.15000000596046448` the SLOWNESS
/// row's same call carries, the SAME ADD_MULTIPLIED_TOTAL operation
/// the fold wears).
pub const SLOWNESS_EFFECT_MOVEMENT_MODIFIER: f32 = -0.15;

/// THE JUMP BOOST EFFECT'S OWN POWER PER LEVEL (javap
/// `LivingEntity.getJumpBoostPower` pcs 10-26 — the `float 0.1f` the
/// `(amplifier + 1)` factor scales: the jump power the DIRECT effect
/// read the getJumpPower fold adds, the attribute modifier the SAFE
/// FALL lane alone carries — the 26.2 jump_boost row the
/// SAFE_FALL_DISTANCE modifier alone wears).
pub const JUMP_BOOST_POWER_PER_LEVEL: f32 = 0.1;
pub const SPRINT_JUMP_BOOST: f32 = 0.2;
pub const JUMP_REPEAT_TICKS: i32 = 10; // noJumpDelay
pub const CUBE_SPEED_NUMERATOR: f32 = 0.21600002; // the ice-slip law's own
/// THE ELYTRA'S OWN MAX DAMAGE (the vanilla elytra item registration's
/// own 432 — the durability the hurt walk's isBroken arm compares,
/// the nextDamageWillBreak refusal the max - 1 keeps).
pub const ELYTRA_MAX_DAMAGE: i32 = 432;

// THE WINGS ANIM'S OWN CONSTANTS (javap `ElytraAnimationState` the 26.2
// jar — the <clinit>'s own ldc pair the DEFAULT_X_ROT/DEFAULT_Z_ROT
// fields keep, the glide/crouch branch constants the tick walk banks:
// the lerp pair the falling fold speaks, the crouch triple the
// isCrouching arm carries, the 0.3F ease the walk's tail wears).
/// `DEFAULT_X_ROT` — the resting wings' own pitch (15°).
pub const ELYTRA_WING_DEFAULT_X: f32 = 0.2617994;
/// `DEFAULT_Z_ROT` — the resting wings' own roll (-15°).
pub const ELYTRA_WING_DEFAULT_Z: f32 = -0.2617994;
/// The falling fold's own x-lerp pair: `Mth.lerp(f, 0.2617994F,
/// 0.34906584F)` — the steeper pitch the fast descent wears (20°).
pub const ELYTRA_WING_GLIDE_X_END: f32 = 0.34906584;
/// The falling fold's own z-lerp pair: `Mth.lerp(f, -0.2617994F,
/// -1.5707964F)` — the wings' flat-out spread the dive reaches (-90°).
pub const ELYTRA_WING_GLIDE_Z_END: f32 = -1.5707964;
/// The crouch branch's own triple: x 0.6981317F (40°), z -0.7853982F
/// (-45°), y 0.08726646F (5°) — the folded-back pose the sneak wears.
pub const ELYTRA_WING_CROUCH_X: f32 = 0.6981317;
pub const ELYTRA_WING_CROUCH_Y: f32 = 0.08726646;
pub const ELYTRA_WING_CROUCH_Z: f32 = -0.7853982;
/// The ease's own factor (`(target - rot) * 0.3F` the walk's tail
/// speaks, the rotY lane the same factor keeps).
pub const ELYTRA_WING_EASE: f32 = 0.3;
/// The falling fold's own exponent (`Math.pow(-v.normalize().y, 1.5D)`
/// the d2f narrowing keeps — the descent steepness the lerp driver
/// wears).
pub const ELYTRA_WING_POW: f64 = 1.5;

pub const WIDTH: f64 = 0.6;
pub const STANDING_HEIGHT: f64 = 1.8;
pub const CROUCH_HEIGHT: f64 = 1.5;
pub const STANDING_EYE: f64 = 1.62;
pub const CROUCH_EYE: f64 = 1.27;
/// The eye walk's own per-tick blend (javap the pose section's
/// `eyeHeight += (f - eyeHeight) * 0.5` — the crouch eases over a few
/// ticks, never snaps).
pub const EYE_HEIGHT_BLEND: f64 = 0.5;
/// The vanilla step the ladder and the sneak probe both speak
/// (javap LivingEntity.maxUpStep — the Attributes.STEP_HEIGHT's own
/// 0.6 default the getAttributeValue fold carries; the base Entity's
/// own 0.0 the drops/TNT/xp orbs wear — their lanes never step). The
/// ladder ([`collide_stepped`]) lifts the grounded body's horizontal
/// move by this much across the colliders' own tops, the sneak
/// edge-guard the same depth the sunk-box probe sinks.
pub const MAX_UP_STEP: f64 = 0.6;
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

/// THE COLUMN'S OWN DRAG (javap `Entity.handleOnInsideBubbleColumn` +
/// `Entity.handleOnAboveBubbleColumn` verbatim — the two laws the
/// bubble column's entityInside branch keeps): the whirlpool's steady
/// pull `y - 0.03` floored at −0.3 (the body cell) or −0.9 (the
/// surface cell — the column's mouth), the upward's steady push
/// `y + 0.06` capped at 0.7 (the body) or the `y + 0.1` launch capped
/// at 1.8 (the surface). The x/z the laws never touch — the column's
/// pull is VERTICAL; the fall-distance reset the INSIDE law's own tail
/// speaks (the surface law carries none), the reset the caller arms.
pub fn bubble_column_drag(vy: f64, drag: bool, surface: bool) -> f64 {
    if drag {
        if surface {
            (vy - 0.03).max(-0.9)
        } else {
            (vy - 0.03).max(-0.3)
        }
    } else if surface {
        (vy + 0.1).min(1.8)
    } else {
        (vy + 0.06).min(0.7)
    }
}

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

// THE FIRE ARC's constants (javap, the same jar — the lava lane):
/// `Entity.lavaIgnite`'s own `igniteForSeconds(15.0F)` — the lava's
/// brand. javap: fireImmune aside (the bare player never is), the
/// float 15.0f rides igniteForSeconds.
#[expect(dead_code)] // the FireImmune check's own 15s — the client's honest lane never burns
pub const LAVA_IGNITE_SECONDS: f32 = 15.0;
/// `igniteForSeconds(15.0F)`'s own fold — `igniteForTicks(Mth.floor(
/// 15.0F * 20.0F))` = 300 ticks (the precomputed floor the constant
/// documents; the law computes it live through the two folds).
pub const LAVA_FIRE_TICKS: i32 = 300;
/// `Entity.lavaHurt`'s own dose — `hurtServer(level, lava(), 4.0F)`
/// (the ldc_w 4.0f the bytecode speaks).
pub const LAVA_DAMAGE: f32 = 4.0;
/// `Entity.baseTick`'s fire clock's own dose — `hurtServer(level,
/// onFire(), 1.0F)` (the fconst_1 the bytecode speaks), gated
/// `remainingFireTicks % 20 == 0 && !isInLava()`.
pub const ON_FIRE_DAMAGE: f32 = 1.0;
/// The fire clock's own cadence — the `bipush 20; irem` the baseTick
/// bytecode speaks (the same 20 the mob lane's FIRE_DAMAGE_INTERVAL
/// names; the body keeps its own — the laws speak per-owner).
pub const FIRE_DAMAGE_INTERVAL: i32 = 20;

// THE FIRE BLOCK'S OWN ARC (javap the 26.2 jar — the fire kinds the
// entityInside walk reads):
/// `BaseFireBlock::fireIgnite`'s own floor — the igniteForSeconds(8.0F)
/// the `if remainingFireTicks >= 0` arm speaks (the ldc 8.0f), folded
/// `igniteForTicks(Mth.floor(8.0F * 20.0F))` = 160 ticks. The law
/// LIFTS the clock to the floor when under — never lowers it (the
/// `remainingFireTicks < 160` verdict the igniteForTicks bytecode
/// keeps). Both fire kinds ride the SAME parent law — the entityInside
/// the SoulFireBlock never overrides.
pub const FIRE_IGNITE_TICKS: i32 = 160;
/// `BaseFireBlock.entityInside`'s runAfter tail's own dose for the
/// plain fire — the `hurt(inFire(), fireDamage)` the applier queue
/// speaks, the FireBlock ctor's `super(properties, 1.0F)` the
/// fconst_1 the bytecode banks (the fireDamage field the lambda
/// consumes). The hurt pipeline's own cooldown answers it — the
/// in-window repeats the vanilla's invulnerable beat buries.
pub const IN_FIRE_DAMAGE: f32 = 1.0;
/// The SAME entityInside tail's own dose for the soul fire — the
/// SoulFireBlock ctor's `super(properties, 2.0F)` the fconst_2 the
/// bytecode banks: the soul fire's cell the shared walk doubles.
pub const IN_SOUL_FIRE_DAMAGE: f32 = 2.0;

/// THE LIT CAMPFIRE'S OWN DOSE (the dev-6 round — javap
/// CampfireBlock.entityInside over the 26.2 jar: the LIT verdict +
/// the LivingEntity gate the `hurt(damageSources.campfire(),
/// fireDamage)` call serves — the DIRECT hurt the BaseFireBlock's
/// ignite-then-hurt applier phases never wear, the fireDamage int
/// the ctor's (true, 1) args row keeps). The campfire damage type's
/// IS_FIRE tag the pipeline's own refusal arms read — the NO ignite
/// lift the bytecode's own shape keeps (the block walk sets NO fire
/// clock, the burning-after the BaseFireBlock walk alone serves).
pub const CAMPFIRE_DAMAGE: f32 = 1.0;
/// THE LIT SOUL CAMPFIRE'S OWN DOSE (the same round — the ctor's
/// (false, 2) args row the fireDamage int keeps: the soul campfire's
/// cell the plain's amount doubles — the BaseFireBlock walk's own
/// float split the same beat wears).
pub const SOUL_CAMPFIRE_DAMAGE: f32 = 2.0;

// THE AIR ARC's constants (javap, the same jar — the drowning lane):
/// `Entity.getMaxAirSupply`'s own value — the `sipush 300; ireturn`
/// the bytecode speaks (the store the DATA_AIR_SUPPLY_ID define folds).
pub const TOTAL_AIR_SUPPLY: i32 = 300;
/// `LivingEntity.increaseAirSupply`'s own fold — `Math.min(air + 4,
/// max)`: the dry refill's own step (the `iadd 4` the bytecode
/// speaks), the surfaced body's air climbs four a tick.
pub const AIR_REFILL_PER_TICK: i32 = 4;
/// `LivingEntity.shouldTakeDrowningDamage`'s own floor —
/// `getAirSupply() <= -20` (the `bipush -20` the bytecode speaks):
/// the twenty ticks below zero the dose's rhythm speaks.
pub const DROWN_HURT_FLOOR: i32 = -20;
/// `LivingEntity.baseTick`'s drown dose — `hurtServer(level, drown(),
/// 2.0F)` (the `fconst_2` the bytecode speaks), the broadcast event
/// 67's own frame (the drowning particles the client-only lane never
/// renders).
pub const DROWN_DAMAGE: f32 = 2.0;

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
/// where `fallPower = (distance + 1.0E-6) - safeFallDistance` — the
/// threshold the SAFE_FALL_DISTANCE attribute's value carries (the
/// default 3.0; the jump_boost's ADD_VALUE modifier the +1.0×(amp+1)
/// lift the template law speaks). The EXACTLY-3-block fall stays
/// safe (the epsilon pushes power to 1e-6, the floor floors it to
/// 0); every full block past the threshold speaks one damage.
pub fn calculate_fall_damage(
    distance: f64,
    block_multiplier: f64,
    safe_fall_distance: f64,
) -> i32 {
    let power = (distance + FALL_POWER_EPSILON) - safe_fall_distance;
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
pub(crate) fn box_at(x: f64, y: f64, z: f64, height: f64) -> (f64, f64, f64, f64, f64, f64) {
    let h = WIDTH / 2.0;
    (x - h, y, z - h, x + h, y + height, z + h)
}

/// THE SOLID PREDICATE THE SHAPE TABLE ITSELF ANSWERS (autopush-420 —
/// the hand-list the families grew one round at a time the
/// [`crate::world::collision_shape`] table retires: ONE source the
/// placement's `isUnobstructed` walk and the physics walk BOTH speak,
/// the divergence the two vocabularies never keep again — the hull
/// lane's own visual predicate the vanilla getShape/getCollisionShape
/// split still keeps). The EMPTY rows the body walks through — the
/// air, the fluids (LAVA joins: the body wades INTO it, the fire laws
/// answer), the whole AGE LADDER the farmer's field keeps, the
/// FLOWER/TUFT/DOUBLE-PLANT families, the torch stick, and the LEAF
/// LITTER row the unification GAINS (the ground cover the hand-list
/// never carried — the vanilla `.noCollision()` row the table already
/// banked, the player the cover walked through it kept). The COLUMN
/// rows (the farmland, the dirt path) and the full-cube roster the
/// solid verdict keeps — the COLUMN rows' 15/16 BAND the sweep's own
/// slabs wear ([`clip_axis`], autopush-421: the landing the band top
/// speaks, the horizontal the band the strict overlap reads, the
/// full-cell verdict the band retires). The drop lane's un-stuck and
/// the item physics speak the same law.
pub fn solid(world: &World, x: i32, y: i32, z: i32) -> bool {
    !matches!(
        crate::world::collision_shape(world.get(x, y, z)),
        crate::world::CollisionShape::Empty
    )
}

/// The per-axis clip (the classic AABB clipXCollide law the VoxelShape
/// sweep reduces to over full cubes): the movement along `axis` (0=X,
/// 1=Y, 2=Z) is clipped to the contact face of every solid cell
/// overlapping the box on the OTHER two axes. An already-intersecting
/// cell stops the movement dead (the vanilla bitset sweep's own
/// verdict); the touching contact still moves.
fn clip_axis(world: &World, b: (f64, f64, f64, f64, f64, f64), axis: usize, d: f64) -> f64 {
    if d.abs() < COLLISION_EPSILON {
        // THE EPSILON-TINY MOVE THE DELTA ITSELF KEEPS (the caller's
        // own flags the honest fold reads — the 0.0 substitute the
        // delta's residue never equals: the due-east walk's
        // cos(-pi/2) 1.8e-17 vz the axis sweep early-returns, the
        // substituted zero the horizontal_collision flag the caller's
        // `allowed != mv` compare FALSELY arms — the grounded walker
        // the auto-jump's own word never stops hopping). The vanilla
        // collide the sub-epsilon move the real delta applies — the
        // return the delta itself speaks, the flags the honest
        // no-collision keep.
        return d;
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
                // THE CELL'S OWN SHAPE THE TABLE ANSWERS (autopush-421 —
                // the COLUMN rows' 15/16 band the full-cell sweep the
                // prior round's walk named: `Block.column(16, 0, 15)`
                // the FarmlandBlock/DirtPathBlock SHAPE constants speak,
                // the cell's Y extent the band's own top wears). The
                // EMPTY rows the walk-through skip keeps, the COLUMN
                // row's Y range the band top speaks (the landing the
                // 15/16 floor wears, the horizontal sweep the band the
                // strict overlap reads), the X/Z footprint the full
                // cell keeps, the full-cube roster the whole cell.
                let shape = crate::world::collision_shape(world.get(cx, cy, cz));
                if matches!(shape, crate::world::CollisionShape::Empty) {
                    continue;
                }
                let (y_lo, y_hi) = match shape {
                    crate::world::CollisionShape::Column(h) => {
                        (cy as f64, cy as f64 + h as f64 / 16.0)
                    }
                    _ => (cy as f64, (cy + 1) as f64),
                };
                // the cell's slabs on the other two axes must strictly
                // overlap the box (the epsilon the vanilla's
                // findIndex bounds carry) — the Y slab the shape's own
                // band wears, the X/Z footprint the full cell keeps
                let (c1lo, c1hi) = match axis {
                    1 => (cx as f64, (cx + 1) as f64),
                    _ => (y_lo, y_hi),
                };
                let (c2lo, c2hi) = match axis {
                    2 => (cx as f64, (cx + 1) as f64),
                    _ => (cz as f64, (cz + 1) as f64),
                };
                if !(c1lo < a1hi - COLLISION_SKIN && c1hi > a1lo + COLLISION_SKIN) {
                    continue;
                }
                if !(c2lo < a2hi - COLLISION_SKIN && c2hi > a2lo + COLLISION_SKIN) {
                    continue;
                }
                let (c_lo, c_hi) = match axis {
                    0 => (cx as f64, (cx + 1) as f64),
                    1 => (y_lo, y_hi),
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

/// The axis-box shift the ladder's lanes speak (the AABB.move the
/// vanilla walk carries).
fn box_shift(
    b: (f64, f64, f64, f64, f64, f64),
    dx: f64,
    dy: f64,
    dz: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    (
        b.0 + dx,
        b.1 + dy,
        b.2 + dz,
        b.3 + dx,
        b.4 + dy,
        b.5 + dz,
    )
}

/// THE STEP LADDER (javap Entity.collide the 26.2 jar verbatim — the
/// ladder the 26.2 refactor folded INTO the collide walk, the raw
/// disassembly's own walk this round speaks): the first sweep's
/// horizontal block the grounded (or falling) body the candidate
/// heights the colliders' own Y break-points speak, the lowest
/// strictly-further height the swept horizontal takes, the net Y the
/// pre-lift fold answers. THE GATE (`maxUpStep() > 0 && (fall ||
/// onGround) && (xBlocked || zBlocked)` — the bytecode's own order,
/// pcs 126-154): the FALL lane (the request's dy the sweep clipped
/// AND the request negative) pre-lifts the box by the kept dy (the
/// body virtually lands, the horizontal the landed box sweeps — the
/// staircase arc the run carries, the grounded walk's gravity clip
/// the same lane's zero lift wears), the grounded-no-fall lane the
/// box as-is keeps (the -1e-5 skirt the collider sweep the
/// exact-height collection catches). THE HEIGHTS
/// (collectCandidateStepUpHeights verbatim): every solid cell the
/// swept region holds contributes its Y break-points — the cell's
/// bottom and the shape table's own top (the COLUMN rows the 15/16
/// band speak) — the below-feet coords the `< 0` skip keeps, the
/// first sweep's kept dy the `==` gate skips (the floor's own top
/// never a candidate), the `> maxUpStep` coords the cell's remaining
/// break refuses; the set the ascending sort wears (the FloatArraySet
/// law). THE SWEEP (the lowest winner's law): for each height
/// ascending, the collide (the FULL requested horizontal, the height
/// the Y delta wears — the axisStepOrder's Y-first lift the
/// horizontal rides) whose horizontal strictly beats the first
/// sweep's returns FIRST (`>` the dcmpl/ifle pair speaks, the
/// ascending order the lowest climb keeps); the net dy the pre-lift
/// fold speaks (`cand.subtract(0, aabb.minY - box.minY, 0)` — the
/// fall lane's kept dy the fold carries back out, the grounded lane
/// the zero fold keeps), the flags the honest final delta vs the
/// request wears. THE FULL WALL the ladder never climbs (the 1.0
/// cell's top the `> maxUpStep` gate refuses — the jump the wall's
/// own answer), the 1/16 band the field edge the climb speaks, the
/// 0.5 slab the step the same law carries. The base Entity's own 0.0
/// step the drops/TNT/xp orbs never arm — the LivingEntity lanes
/// (the player, the mobs) the 0.6 attribute the callers speak.
pub fn collide_stepped(
    world: &World,
    b: (f64, f64, f64, f64, f64, f64),
    mv: (f64, f64, f64),
    on_ground: bool,
    max_up_step: f64,
) -> Collision {
    let first = collide(world, b, mv);
    if max_up_step <= 0.0 {
        return first;
    }
    let x_blocked = first.dx != mv.0;
    let z_blocked = first.dz != mv.2;
    let falling = first.dy != mv.1 && mv.1 < 0.0;
    if !(falling || on_ground) || (!x_blocked && !z_blocked) {
        return first;
    }
    // the ladder's own box: the fall lane pre-lifts by the dy the
    // first sweep kept (the landed box the horizontal sweeps), the
    // grounded lane the box as-is wears
    let lifted = if falling {
        box_shift(b, 0.0, first.dy, 0.0)
    } else {
        b
    };
    // the collider sweep's region (the expandTowards union the vanilla
    // walk carries: the requested horizontal drift, the maxUpStep of
    // headroom; the grounded-no-fall lane the -1e-5 skirt adds — the
    // exact-height floor the collection catches)
    let (lx0, ly0, lz0, lx1, ly1, lz1) = lifted;
    let skirt = if falling { 0.0 } else { 1e-5 };
    let swept = (
        lx0.min(lx0 + mv.0),
        ly0 - skirt,
        lz0.min(lz0 + mv.2),
        lx1.max(lx1 + mv.0),
        ly1 + max_up_step,
        lz1.max(lz1 + mv.2),
    );
    // THE CANDIDATE HEIGHTS: every solid cell the region holds
    // contributes its Y break-points (the cell's bottom, the shape
    // table's own top — the COLUMN row the 15/16 band speaks), the
    // below-feet and the kept-dy and the beyond-step coords the gates
    // refuse, the dedup the ascending sort wears
    let base_y = first.dy;
    let mut heights: Vec<f64> = Vec::new();
    for cx in swept.0.floor() as i32..=(swept.3 - 1e-9).floor() as i32 {
        for cy in swept.1.floor() as i32..=(swept.4 - 1e-9).floor() as i32 {
            for cz in swept.2.floor() as i32..=(swept.5 - 1e-9).floor() as i32 {
                let shape = crate::world::collision_shape(world.get(cx, cy, cz));
                if matches!(shape, crate::world::CollisionShape::Empty) {
                    continue;
                }
                let top = match shape {
                    crate::world::CollisionShape::Column(h) => h as f64 / 16.0,
                    _ => 1.0,
                };
                for f in [cy as f64 - ly0, cy as f64 + top - ly0] {
                    if f < 0.0 {
                        continue;
                    }
                    if f == base_y {
                        continue;
                    }
                    if f > max_up_step {
                        break;
                    }
                    if !heights.contains(&f) {
                        heights.push(f);
                    }
                }
            }
        }
    }
    heights.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let best = first.dx * first.dx + first.dz * first.dz;
    for h in heights {
        let cand = collide(world, lifted, (mv.0, h, mv.2));
        let cand_h = cand.dx * cand.dx + cand.dz * cand.dz;
        if cand_h > best {
            // the net delta the pre-lift fold answers (`aabb.minY -
            // box.minY` the subtract speaks: the fall lane's kept dy
            // the fold carries back out, the grounded lane the zero
            // fold keeps) — the flags the final delta vs the request
            let dy = cand.dy - (b.1 - lifted.1);
            return Collision {
                dx: cand.dx,
                dy,
                dz: cand.dz,
                horizontal_collision: cand.dx != mv.0 || cand.dz != mv.2,
                vertical_collision_below: mv.1 < 0.0 && dy != mv.1,
                collided_x: cand.dx != mv.0,
                collided_y: dy != mv.1,
                collided_z: cand.dz != mv.2,
            };
        }
    }
    first
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
        (if dx > 0.0 { cx as f64 + 1.0 - fx } else { fx - cx as f64 }) / dx.abs()
    } else {
        f64::INFINITY
    };
    let mut t_max_y = if dy != 0.0 {
        (if dy > 0.0 { cy as f64 + 1.0 - fy } else { fy - cy as f64 }) / dy.abs()
    } else {
        f64::INFINITY
    };
    let mut t_max_z = if dz != 0.0 {
        (if dz > 0.0 { cz as f64 + 1.0 - fz } else { fz - cz as f64 }) / dz.abs()
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
        if t_next > 1.0 || !t_next.is_finite() {
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
    /// The previous tick's feet (javap `Entity.tick`'s own `xo = x`
    /// head — the pre-move pose the render's interpolation walks
    /// FROM). Saved at the tick's own head, before the physics; the
    /// lerp the fixed clock's partial rides.
    pub prev_x: f64,
    pub prev_y: f64,
    pub prev_z: f64,
    /// THE EYE'S OWN WALK (javap `LivingEntity.eyeHeight` / `oEyeHeight`):
    /// the SMOOTHED eye height the pose blends toward (`eyeHeight +=
    /// (f - eyeHeight) * 0.5` per tick — the crouch eases, the snap
    /// never speaks) and the pre-tick pose the render's lerp walks
    /// FROM. The camera reads the lerped pair; the gameplay reads keep
    /// the pose target `eye_height()`.
    pub eye_height: f64,
    pub prev_eye_height: f64,
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
    /// THE ABILITIES' OWN FLIGHT STATE (javap Abilities.flying the
    /// 26.2 law — the field the LocalPlayer.aiStep walk owns: the
    /// double-tap toggle the App's fly law speaks, the spectator
    /// sync forces, the landing retire clears, the Player.travel
    /// restore the physics folds): true = the fly movement the tick
    /// wears — the thrust lane, the fly speeds, the hover, the fall
    /// distance the reset keeps silent, the fov the 1.1 fold speaks.
    pub flying: bool,
    /// THE NOPHYSICS FLAG (javap `Entity.noPhysics` — the field
    /// Player.tick's head writes fresh every frame:
    /// `this.noPhysics = this.isSpectator()`, the spectator the raw
    /// walk wears, the F4 flick the bank feeds instantly). The move's
    /// raw branch the flag alone gates (javap Entity.move pcs 0-58:
    /// the direct position add, the collision flags zeroed, the
    /// collide walk the early return skips — the fall damage, the
    /// footsteps, the moveDist, the movement exhaustion, the
    /// restitution, the walk animation), and the inside-blocks walks
    /// the isAffectedByBlocks verdict refuses (javap:
    /// `!isRemoved() && !noPhysics` — the bubble drag, the lava
    /// ignite/hurt, the water extinguish the gate keeps silent).
    pub no_physics: bool,
    /// THE INVULNERABLE ABILITY (javap `Abilities.invulnerable` — the
    /// grant `GameType.updatePlayerAbilities`'s putfield walk wears:
    /// CREATIVE and SPECTATOR the flag banks, survival/adventure the
    /// zero walk keeps — the audit the mpgamemode_262.txt evidence
    /// carries). The App feeds it the same per-frame head the
    /// noPhysics write rides, the F4 flick the bank feeds instantly.
    /// The hurt pipeline's own head the flag alone refuses (javap
    /// `LivingEntity.hurtServer` pcs 0-10 — the
    /// `isInvulnerableTo(level, source)` verdict the pipeline answers
    /// BEFORE the cooldown speaks), and the air clock's decrease gate
    /// the flag the never-drain law wears (javap `LivingEntity.baseTick`
    /// pcs 258-280 — the player's arm `(!bl2 || !abilities.invulnerable)`
    /// folds, the invulnerable body the underwater refill arm rides).
    pub invulnerable: bool,
    /// THE FLYING SPEED ABILITY (javap `Abilities.flyingSpeed` — the
    /// field the <init> defaults 0.05, the `FLY_SPEED` const's own value
    /// the default carries). The folds read the FIELD the
    /// `Player.getFlyingSpeed` walk answers (the flying branch's own
    /// split — the sprint-fly the ×2.0 fold carries, the plain fly the
    /// field, the aiStep's vertical thrust the field × 3.0 the
    /// `ldc_w #1270` fold multiplies), and the WHEEL adjusts it (javap
    /// `MouseHandler.onScroll` pcs 316-364 — the spectator arm's own
    /// flying-speed lane: `flyingSpeed + y * 0.005` the `ldc_w #365`
    /// step carries, the `clamp(0.0, 0.2)` the `fconst_0`/`ldc_w #366`
    /// pair speaks). The default the <init> keeps — the wheel's
    /// adjustability the spectator body alone wears, the survival walk
    /// byte-identical beneath (the field the default's own value
    /// reads).
    pub flying_speed: f32,
    /// THE CAMERA'S OWN FOV STATE (Camera's fovModifier/oldFovModifier
    /// pair — tickFov's own two fields): the modifier the projection's
    /// 70° option multiplies — the sprint's 1.15 the speed ratio speaks,
    /// the half-step the tick keeps smooth, the pair the frame's partial
    /// lerp walks.
    pub fov_modifier: f32,
    pub prev_fov_modifier: f32,
    /// The input impulses (the KeyboardInput's own ±1 shape).
    pub forward: f32,
    pub left: f32,
    pub jumping: bool,
    no_jump_delay: i32,
    /// THE EFFECT ATTRIBUTES' OWN MOVEMENT FOLD (javap
    /// `MobEffect.addAttributeModifiers` + `AttributeTemplate.create`
    /// pc 8-16 + `AttributeInstance.calculateValue` pcs 101-145 — the
    /// ADD_MULTIPLIED_TOTAL modifiers the amount×(amplifier+1) law
    /// carries, the ×(1+amount) fold EACH modifier its own step
    /// wears, the sanitizeValue [0, 1024] clamp the negative floor
    /// refuses): the speed/slowness rows' PRODUCT the App's tick
    /// banks from the effects store (the attribute MAP the documented
    /// absence keeps — the value the fold's OWN verdict speaks). The
    /// speed() walk and the FOV's own ratio both read it — the
    /// sprint-zoom the effect doses the same law widens.
    pub effect_speed_mult: f32,
    /// THE JUMP BOOST EFFECT'S OWN POWER (javap
    /// `LivingEntity.getJumpBoostPower` — the `0.1f * (amplifier + 1)`
    /// the hasEffect gate answers, the DIRECT effect read the
    /// 26.2's jump_boost row the attribute map never carries): the
    /// App's tick banks it from the store, the jumpFromGround floor
    /// the JUMP_STRENGTH value adds it.
    pub jump_boost_power: f32,
    /// THE BLOCK JUMP FACTOR'S OWN BANK (javap Entity.getBlockJumpFactor
    /// pcs 0-48 the 26.2 jar — the feet cell's getJumpFactor the
    /// generated table answers, the DEFAULT 1.0 the below cell falls
    /// through): the tick's head the walk banks, the jumpFromGround
    /// floor the JUMP_STRENGTH value multiplies — the 1.0 default the
    /// plain ground wears, the fly-law's launch hop the same-tick bank
    /// reads (the App's pc 628 call the world-free fold keeps).
    pub block_jump_factor: f32,
    /// THE JUMP BOOST EFFECT'S OWN SAFE FALL (javap the jump_boost
    /// row's `addAttributeModifier(SAFE_FALL_DISTANCE,
    /// "effect.jump_boost", 1.0, ADD_VALUE)` — the amount×(amp+1)
    /// the template law scales, the ADD_VALUE the calculate fold
    /// adds): the fall-grace threshold the App's tick banks, the
    /// calculateFallDamage walk reads it.
    pub safe_fall_distance: f64,
    /// THE FOOTSTEP ACCUMULATION (Entity.move's own fields): the walk
    /// distance grows by the horizontal movement × 0.6, the step fires
    /// when it crosses `next_step` (nextStep() = (int)moveDist + 1.0F —
    /// the vanilla's one-footstep-per-~1.667-blocks cadence). The App
    /// drains `step_due` after the tick and speaks the block under the
    /// feet's own voice.
    pub move_dist: f32,
    pub next_step: f32,
    /// THE ENTITY'S OWN TICK COUNTER (javap `ServerLevel.tickNonPassenger`
    /// pc 8-15 — the `entity.tickCount++` the tick's own HEAD serves
    /// (before `entity.tick()`'s walk), the post-tick value the render's
    /// partial-0 boundary reads; the darkness pulse's cosine fold the
    /// count's lifetime speaks). The ctor zeroes; the tick walk's head
    /// increments.
    pub tick_count: i32,
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
    /// THE FALL-FLYING STATE (javap `Entity.isFallFlying`'s own shared
    /// flag 7 — `startFallFlying`'s `setSharedFlag(7, true)` the
    /// 26.2 walk speaks, `stopFallFlying`'s true-then-false pair the
    /// dirty-bit propagation guarantees): the elytra glide the body
    /// wears. The App's chest verdict feeds [`Self::chest_glides`];
    /// the start walk and the stop arm the tick's own aiStep order
    /// keeps, the glide folds the travel lane wears.
    pub fall_flying: bool,
    /// THE FALL-FLYING WEAR CLOCK (javap `LivingEntity.tick` the 26.2
    /// walk — the `fallFlyTicks` field the glide-gated pair keeps: the
    /// pc 496-504 gate the isFallFlying win `updateFallFlying` speaks
    /// (the local `i = fallFlyTicks + 1` fold the `i % 10 == 0` gate
    /// the gameEvent ELYTRA_GLIDE answers, the `(i / 10) % 2 == 0`
    /// gate the hurtAndBreak(1) rides), the pc 598-620 tail the field
    /// itself banks — `isFallFlying() ? ++fallFlyTicks :
    /// fallFlyTicks = 0`, the reset arm the NON-gliding tick speaks).
    /// The grounded tick the clock zeroes — the fresh glide the
    /// rhythm restarts (the prior round's "never resets" verdict the
    /// updateFallFlying-only view wore — that walk's istore_1 the
    /// LOCAL alone banks, the tick()'s own putfield walk the re-exam
    /// retires).
    pub fall_fly_ticks: i32,
    /// THE ELYTRA'S OWN DAMAGE BANK (javap `ItemStack.hurtAndBreak` —
    /// the `damage` the applyDamage walk banks, the elytra item's own
    /// 432 the vanilla registration carries): the ONE damageable stack
    /// the glide wears (the chest slot). The (id, count) tuples the
    /// App's slots carry no damage component — the bank rides the
    /// body (the single-player walk's own truth), the chest-item
    /// change the App's feed resets (the stack swap the fresh
    /// elytra's zero speaks).
    pub elytra_damage: i32,
    /// THE BREAK'S OWN DUE FLAG (javap `ItemStack.hurtAndBreak` — the
    /// isBroken arm the shrink + onEquippedItemBroken pair answers:
    /// the entityEvent 47 the CHEST break broadcasts, the client's
    /// breakItem the sound + the item particles speak). Set in the
    /// hurt walk the damage crossing speaks, the App drains it (the
    /// slot clears, the voice lands, the pieces spawn). The glide's
    /// own reach never triggers it — the nextDamageWillBreak gate
    /// refuses the LAST point first (the torn elytra the glide never
    /// rides), the arm the shared law keeps for the other sources.
    pub elytra_broke_due: bool,
    /// THE GLIDE GAME-EVENT'S DUE FLAG (javap `updateFallFlying` —
    /// the `gameEvent(GameEvent.ELYTRA_GLIDE)` the i%10 gate speaks:
    /// the every-10-tick world event the sculk's own hearing rides).
    /// Set every tenth gliding tick, the App drains it (the sculk
    /// lane's record the drain speaks).
    pub elytra_glide_event_due: bool,
    /// THE GLIDE GAME-EVENT'S OWN POSITION STASH (javap
    /// `Entity.gameEvent(Holder, Entity)` — the `getfield position`
    /// the Level.gameEvent call carries: the event posts at the
    /// entity's Vec3 THE MOMENT the gate speaks — the wear walk the
    /// tick's PRE-travel head runs, the move the position never
    /// touched yet). The gate walk banks it beside the flag, the
    /// App's drain lane reads the pair.
    pub elytra_glide_event_pos: (f64, f64, f64),
    /// THE WINGS ANIM'S OWN STATE (javap `ElytraAnimationState` — the
    /// rotX/rotY/rotZ + the rotXOld/rotYOld/rotZOld field pair the
    /// tick walk banks, the render's `getRotX(partialTick)` lerp the
    /// extraction serves): the elytra wings' pose clock the third-
    /// person lane reads. Java field initializers keep all six at 0.0
    /// — the fresh spawn's wings ease OPEN from flat (the same law
    /// vanilla's fresh entity speaks), the ease never snaps.
    pub elytra_wing_rot_x: f32,
    pub elytra_wing_rot_y: f32,
    pub elytra_wing_rot_z: f32,
    pub elytra_wing_rot_x_old: f32,
    pub elytra_wing_rot_y_old: f32,
    pub elytra_wing_rot_z_old: f32,
    /// THE INFINITE MATERIALS GATE (javap `Player.hasInfiniteMaterials`
    /// — the abilities.instabuild the creative mode speaks): the
    /// processDurabilityChange arm the creative stack never carries —
    /// the hurt walk's own zero answer. The App's game-mode feed
    /// sets it each tick.
    pub infinite_materials: bool,
    /// THE LAST-SEEN CHEST ITEM (the App's feed walk banks it — the
    /// stack-swap detector the damage reset speaks: the elytra's
    /// equip/unequip/replace the fresh bank answers, the break's own
    /// drain the cleared slot rides). The (id, count) tuples the
    /// slots carry no damage component — the change walk the honest
    /// stand-in keeps.
    pub chest_item: i32,
    /// THE CHEST'S OWN GLIDE VERDICT (javap `canGlideUsing` the 26.2
    /// walk — the GLIDER component the elytra item carries + the
    /// equippable slot pair + the nextDamageWillBreak gate, the
    /// component-less stack the id stand-in answers): the App feeds
    /// it each tick from the chest slot (the offline armor row's
    /// CHEST index, the live store's window slot 7), the canGlide
    /// walks read.
    pub chest_glides: bool,
    /// THE GLIDE START'S OWN WIRE ECHO (javap `LocalPlayer.aiStep`
    /// pcs 679-694 — the tryToStart verdict's success the
    /// ServerboundPlayerCommandPacket ride speaks): set in the tick
    /// the chord wins, the App drains it and the live session the
    /// START_FALL_FLYING command speaks (the offline lane no wire
    /// spends). Cleared by the reader.
    pub glide_started_due: bool,
    /// THE SPEED VARIANT (the < 0.25 gate): false speaks the plain
    /// PLAYER_SPLASH, true the HIGH_SPEED one — the fast fall's own
    /// louder welcome.
    pub splash_high_speed: bool,
    /// THE SPLASH VOLUME (doWaterSplashEffect's fold): min(1.0,
    /// d2f(sqrt(dx²·0.2 + dy² + dz²·0.2)) * 0.2F) over the CARRIED
    /// velocity — the baseTick read, last move's post-collision
    /// result, the entry speed the body fell in with.
    pub splash_volume: f32,
    /// THE SPLASH RINGS' OWN POSE (doWaterSplashEffect's loops read
    /// the entity's live position + delta movement at the call —
    /// captured the sound's own moment, the drain replays the two
    /// walks the width * 20 bound speaks). The App takes it after the
    /// sound fold; the tick-head clears what no drain spoke.
    pub splash_pose: Option<(f64, f64, f64, f64, f64, f64)>,
    /// THE FIRE CLOCK (Entity.remainingFireTicks's own field): the
    /// brand the lavaIgnite speaks (the lava contact's
    /// igniteForSeconds(15) fold — only when the clock is UNDER the
    /// brand's value), the decrement the baseTick's fire clock speaks
    /// every tick while positive, the water douse's clearFire kills.
    /// The on-fire render flag rides `> 0` (the setSharedFlagOnFire
    /// write the bytecode speaks).
    pub remaining_fire_ticks: i32,
    /// THE TICK'S LAVA VERDICT (isInLava's own shape): any cell the
    /// body box overlaps is lava — the tick-head scan the fire
    /// clock's !isInLava gate and the baseTick fallDistance fold
    /// read.
    pub in_lava: bool,
    /// THE LAVA HURT (LavaFluid.entityInside's runAfter tail — the
    /// lavaHurt the contact fires every tick it overlaps): the App
    /// answers with the 4.0 hurt through the pipeline's cooldown and
    /// speaks the GENERIC_BURN on the hurt's success (the playSound's
    /// own ifeq gate). The body never clears it, the reader does.
    pub lava_hurt_due: bool,
    /// THE FIRE CLOCK'S HURT (baseTick's own onFire verdict:
    /// remainingFireTicks % 20 == 0 && !isInLava -> hurtServer(
    /// onFire(), 1.0F)): the burn's own dose once the body LEFT the
    /// lava (the in-lava ticks the lava's own 4.0 speaks instead).
    /// The App answers through the same pipeline, the BURNING voice
    /// the DamageEffects speaks.
    pub on_fire_hurt_due: bool,
    /// THE IN-FIRE HURT (BaseFireBlock.entityInside's runAfter tail —
    /// the `hurt(inFire(), fireDamage)` the fire cell's contact fires
    /// every tick it overlaps): the FireBlock ctor's fconst_1 the
    /// 1.0 dose speaks. The App answers through the same pipeline
    /// the lava lane wears (the GENERIC_BURN voice the landed hurt
    /// speaks — the same DamageEffects.BURNING the inFire type
    /// carries). The body never clears it, the reader does.
    pub in_fire_hurt_due: bool,
    /// THE IN-SOUL-FIRE HURT (the SAME entityInside tail — the
    /// SoulFireBlock ctor's fconst_2 the 2.0 dose speaks): the
    /// soul fire's cell the shared walk doubles. The body never
    /// clears it, the reader does.
    pub in_soul_fire_hurt_due: bool,
    /// THE IN-CAMPFIRE HURT (the dev-6 round — javap
    /// CampfireBlock.entityInside: the LIT verdict + the LivingEntity
    /// gate the `hurt(campfire(), fireDamage)` DIRECT call serves —
    /// NO ignite phase the BaseFireBlock applier wears): the lit
    /// campfire's cell the 7-px column the standing body overlaps
    /// (the Column(7) collision the walk-onto tops, the walk-in the
    /// overlap brands). The body never clears it, the reader does.
    pub in_campfire_hurt_due: bool,
    /// THE IN-SOUL-CAMPFIRE HURT (the same walk — the ctor's (false,
    /// 2) row the fireDamage int keeps, the soul campfire's dose the
    /// plain's amount doubles).
    pub in_soul_campfire_hurt_due: bool,
    /// THE WATER DOUSE (InsideBlockEffectType.EXTINGUISH =
    /// Entity::clearFire, the water block's own inside-effect): the
    /// burning body touching water dies to min(0, remaining) and the
    /// transition plays the fizz (playEntityOnFireExtinguishedSound,
    /// the volume 0.7 / pitch 1.6 + (r - r) * 0.4 fold). Set ONLY on
    /// the douse tick (a wet body carries no clock to kill).
    pub fire_doused_due: bool,
    /// THE AIR CLOCK (Entity's own airSupply store — the 300 the
    /// getMaxAirSupply sipush speaks): the EYE's water drains it one
    /// per tick (decreaseAirSupply's `air - 1` — the OXYGEN_BONUS
    /// random the bytecode speaks sleeps on the attribute's honest
    /// absence), the -20 floor folds the dose (the clock resets to 0,
    /// the drown_hurt_due flag rides), the dry else-arm refills four a
    /// tick (increaseAirSupply's `Math.min(air + 4, max)`). Vanilla
    /// shape (LivingEntity.baseTick, the 26.2 jar): the eye-in-water
    /// arm drains, the else arm refills; the bubble-column and
    /// water-breathing gates the vanilla arm carries ride vocabulary
    /// this body never speaks (no bubble blocks in the palette, no
    /// effects store here) — the bare player's own path.
    pub air_supply: i32,
    /// THE EYE'S WATER VERDICT (isEyeInFluid(WATER)'s own query — the
    /// eye POINT's cell, not the feet box the touches_water walk
    /// scans): the air clock's gate reads it, the HUD's bubble row
    /// reads it. Set every tick at the clock's own station.
    pub eye_in_water: bool,
    /// THE DROWN DOSE (shouldTakeDrowningDamage's own verdict —
    /// `air <= -20` folds `setAirSupply(0)` + the broadcast event 67
    /// + `hurtServer(drown(), 2.0F)`): the App answers through the
    /// hurt pipeline's cooldown, the DROWNING voice the damage type's
    /// effects speak. The body never clears it, the reader does.
    pub drown_hurt_due: bool,
    /// THE WATER-BREATHING GATE (javap `MobEffectUtil.hasWaterBreathing`
    /// pcs 0-35 — the WATER_BREATHING ∨ CONDUIT_POWER ∨ NAUTILUS
    /// triple): the air clock's drain arm reads it — the true body the
    /// eye-under-water drain refuses (the baseTick pcs 244-255 the
    /// `!canBreatheUnderwater() && !hasWaterBreathing(this)` conjuncts
    /// keep — the bare player's own canBreatheUnderwater the base's
    /// 0-ireturn false rides). Set every tick from the effects store.
    pub water_breathing: bool,
    /// THE DOLPHINS GRACE VERDICT (javap `LivingEntity.travelInWater`
    /// pcs 85-98 — the hasEffect(DOLPHINS_GRACE) call the drag
    /// override answers): the water lane's drag fold reads it — the
    /// true body the 0.96 glide wears. Set every tick from the
    /// effects store.
    pub dolphins_grace: bool,
    /// THE WEAVING VERDICT (javap `WebBlock.entityInside` pc 35-41 —
    /// the hasEffect(WEAVING) call the wider stuck clamp answers): the
    /// web beat reads it — the true body the 0.5/0.25/0.5 clamp wears,
    /// the bare body the 0.25/0.05/0.25 pair keeps. Set every tick
    /// from the effects store (the LivingEntity instanceof the
    /// bytecode's own gate — the player body IS one, the verdict the
    /// effect store alone carries).
    pub weaving: bool,
    /// THE UNDERWATER REFILL GATE (javap `MobEffectUtil
    /// .shouldEffectsRefillAirsupply` pcs 0-35 — the ¬NAUTILUS ∨
    /// WATER_BREATHING ∨ CONDUIT_POWER walk): the air clock's
    /// underwater refill arm reads it (the baseTick pcs 335-362 the
    /// `air < max && shouldEffectsRefillAirsupply` pair keeps) — the
    /// nautilus-ONLY body the air the FREEZE holds, the effect-less
    /// body the nautilus-absent true arm the constructor's default
    /// wears. Set every tick from the effects store.
    pub effects_refill_air_supply: bool,
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
    /// THE STUCK SLOT (javap `Entity.stuckSpeedMultiplier`'s own Vec3 —
    /// the field the WebBlock.entityInside's makeStuckInBlock write
    /// banks): the next move's HEAD the lengthSqr > 1.0E-7 gate reads,
    /// the movement the ×clamp fold scales, the slot the ZERO reset
    /// and the deltaMovement the ZERO write clear (Entity.move pcs
    /// 97-137). The web beat banks it each overlapping tick, the move
    /// consumes it the NEXT tick — the vanilla's own rhythm, the
    /// momentum the zero-write never lets accumulate. ZERO = no stuck
    /// this tick (the default the ctor speaks).
    pub stuck_speed: (f64, f64, f64),
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
            prev_x: x,
            prev_y: y,
            prev_z: z,
            eye_height: STANDING_EYE,
            prev_eye_height: STANDING_EYE,
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
            flying: false,
            no_physics: false,
            invulnerable: false,
            flying_speed: FLY_SPEED,
            fov_modifier: 1.0,
            prev_fov_modifier: 1.0,
            forward: 0.0,
            left: 0.0,
            jumping: false,
            no_jump_delay: 0,
            effect_speed_mult: 1.0,
            water_breathing: false,
            dolphins_grace: false,
            weaving: false,
            effects_refill_air_supply: true,
            jump_boost_power: 0.0,
            block_jump_factor: 1.0,
            safe_fall_distance: SAFE_FALL_DISTANCE,
            move_dist: 0.0,
            next_step: 1.0,
            tick_count: 0,
            step_due: false,
            step_in_water: false,
            swim_due: false,
            swim_volume: 0.0,
            was_touching_water: false,
            first_tick: true,
            splash_due: false,
            splash_high_speed: false,
            splash_volume: 0.0,
            splash_pose: None,
            fall_flying: false,
            chest_glides: false,
            glide_started_due: false,
            fall_fly_ticks: 0,
            elytra_damage: 0,
            elytra_broke_due: false,
            elytra_glide_event_due: false,
            elytra_glide_event_pos: (0.0, 0.0, 0.0),
            elytra_wing_rot_x: 0.0,
            elytra_wing_rot_y: 0.0,
            elytra_wing_rot_z: 0.0,
            elytra_wing_rot_x_old: 0.0,
            elytra_wing_rot_y_old: 0.0,
            elytra_wing_rot_z_old: 0.0,
            infinite_materials: false,
            chest_item: 0,
            remaining_fire_ticks: 0,
            in_lava: false,
            lava_hurt_due: false,
            on_fire_hurt_due: false,
            in_fire_hurt_due: false,
            in_soul_fire_hurt_due: false,
            in_campfire_hurt_due: false,
            in_soul_campfire_hurt_due: false,
            fire_doused_due: false,
            air_supply: TOTAL_AIR_SUPPLY,
            eye_in_water: false,
            drown_hurt_due: false,
            walk: WalkAnimationState {
                speed_old: 0.0,
                speed: 0.0,
                position: 0.0,
                position_scale: 1.0,
            },
            fall_distance: 0.0,
            stuck_speed: (0.0, 0.0, 0.0),
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

    /// `jumpFromGround`'s own fold (javap ServerPlayer.jumpFromGround /
    /// LivingEntity.jumpFromGround the 26.2 law): the vy the
    /// JUMP_STRENGTH floor raises, the sprint jump's directional boost
    /// the yaw fold speaks, the exhaustion the isSprinting split banks
    /// (the sprint jump's 0.2, the plain jump's 0.05 — FoodConstants'
    /// inlined constants), and the noJumpDelay = 10 the repeat gate
    /// wears. The tick's jump gate AND the App's fly law (the
    /// double-tap toggle-on's launch hop — LocalPlayer.aiStep pc 628
    /// the `jumpFromGround()` call the audited walk carries) both
    /// speak it — the same method the vanilla's two lanes call.
    pub fn jump_from_ground(&mut self) {
        // THE JUMP BOOST'S OWN POWER (javap getJumpPower(float) pcs
        // 0-19 — the JUMP_STRENGTH attribute value the
        // getJumpBoostPower() direct read ADDS: the
        // `attribute * mult * blockJumpFactor + 0.1f * (amp + 1)` walk
        // the mult's 1.0 the jumpFromGround call passes — the
        // blockJumpFactor the banked pair multiplies, the f2d widen
        // the Math.max keeps — the banked 1.0 the plain ground wears)
        self.vy = (self.vy as f64)
            .max(((JUMP_POWER * self.block_jump_factor) + self.jump_boost_power) as f64);
        if self.sprinting {
            let rad = self.yaw as f64 * std::f64::consts::PI / 180.0;
            self.vx += -(rad.sin()) * SPRINT_JUMP_BOOST as f64;
            self.vz += rad.cos() * SPRINT_JUMP_BOOST as f64;
        }
        self.exhaustion_event += if self.sprinting {
            crate::food::EXHAUSTION_SPRINT_JUMP
        } else {
            crate::food::EXHAUSTION_JUMP
        };
        self.no_jump_delay = JUMP_REPEAT_TICKS;
    }

    /// The pose's height (the POSES table's own numbers).
    pub fn height(&self) -> f64 {
        if self.crouching {
            CROUCH_HEIGHT
        } else {
            STANDING_HEIGHT
        }
    }

    /// The POSE's eye height (the target the smooth store blends
    /// toward — the gameplay reads speak it: the eye cell, the look
    /// targets, the reach). The CAMERA rides `lerped_eye_height` —
    /// the smoothed walk the crouch eases along.
    pub fn eye_height(&self) -> f64 {
        if self.crouching {
            CROUCH_EYE
        } else {
            STANDING_EYE
        }
    }

    /// THE CAMERA'S OWN EYE (javap the render's
    /// `Mth.lerp(partialTicks, oEyeHeight, eyeHeight)` fold): the
    /// smoothed eye the frame rides — the previous tick's pose walked
    /// `partial` of the way to the current. The partial >= 1.0 arm
    /// speaks the CURRENT eye bit for bit (the headless lanes'
    /// byte-identical contract).
    pub fn lerped_eye_height(&self, partial: f32) -> f64 {
        if partial >= 1.0 {
            return self.eye_height;
        }
        let t = partial as f64;
        self.prev_eye_height + (self.eye_height - self.prev_eye_height) * t
    }

    /// The speed attribute's current value (the base, the sprint's
    /// 1.3, the sneak's 0.7, the using slowdown's 0.2 — the
    /// ADD_MULTIPLIED_TOTAL fold the attribute system speaks). THE
    /// EFFECT MODIFIERS the attribute value the base fold carries —
    /// the speed/slowness rows' ×(1+amount) product the effect_speed_mult
    /// field banks, the sanitizeValue [0, ..] clamp the negative
    /// floor refuses (the extreme slowness the ZERO speed wears, the
    /// body the wall the vanilla law never walks through).
    fn speed(&self, using_item: bool) -> f32 {
        let mut s = MOVEMENT_SPEED * self.effect_speed_mult;
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

    /// `AbstractClientPlayer.getFieldOfViewModifier`'s own walk (the
    /// 26.2 jar's raw disasm, the scripts/abstractclientplayer_getfovmodifier.txt
    /// dump): f = 1.0, the flying *1.1 the abilities' word keeps (the
    /// body's no-flight lane never arms it), the speed ratio the
    /// ATTRIBUTE speaks — `f *= (speed / walkSpeed + 1.0) / 2.0` over
    /// the same attribute law `speed()` folds, so the sprint's 1.3
    /// answers 1.15, the sneak's 0.7 answers 0.85, the using slowdown's
    /// 0.2 answers 0.6 — the Abilities.getWalkingSpeed default the
    /// MOVEMENT_SPEED base speaks. The bow's draw zoom and the
    /// spyglass's 0.1 the client's own arms never draw (no bow, no
    /// spyglass the inventory keeps); the fovEffectScale lerp the
    /// no-option client keeps at its own 1.0 default (lerp(1.0, 1.0, f)
    /// = f — the modifier rides unblended).
    fn fov_target(&self, using_item: bool) -> f32 {
        if MOVEMENT_SPEED == 0.0 {
            return 1.0;
        }
        let ratio = self.speed(using_item) / MOVEMENT_SPEED;
        // THE FLY FOLD (javap AbstractClientPlayer.getFieldOfViewModifier
        // the 26.2 jar — pcs 2-16: `f = 1.0; if abilities.flying f *=
        // 1.1`, the walking-speed ratio product the fold continues —
        // the flying camera's own zoom-out the sprint ratio rides)
        let fly = if self.flying { FLY_FOV } else { 1.0 };
        fly * ((ratio + 1.0) / 2.0)
    }

    /// `Camera.tickFov`'s own beat (the scripts/camera_tickfov.txt
    /// dump): the old store first (the pair the frame's lerp walks),
    /// the half-step toward the target, then the 0.1/1.5 clamp the
    /// bytecode pins. The body ticks it at the tail — the camera state
    /// the projection reads, the render lerp smooths.
    fn tick_fov(&mut self, target: f32) {
        self.prev_fov_modifier = self.fov_modifier;
        self.fov_modifier = (self.fov_modifier
            + (target - self.fov_modifier) * FOV_TICK_BLEND)
            .clamp(FOV_MODIFIER_MIN, FOV_MODIFIER_MAX);
    }

    /// `Camera.calculateFov`'s own read (the
    /// scripts/camera_calculatefov.txt dump): the frame's modifier the
    /// 70° option multiplies — the lerp the partial walks between the
    /// two stores, the partial >= 1.0 arm the headless lanes'
    /// byte-identical contract keeps (the same shape the lerped eye
    /// speaks). The panoramic 90.0 and the death/fluid modifier lanes
    /// the repo's camera never arms.
    pub fn fov_modifier_at(&self, partial: f32) -> f32 {
        if partial >= 1.0 {
            return self.fov_modifier;
        }
        self.prev_fov_modifier + (self.fov_modifier - self.prev_fov_modifier) * partial
    }

    /// The ground under the moved box probe (the sneak edge-guard's
    /// `noCollision(box.move(dx, -maxUpStep, dz))` question): TRUE when
    /// NOTHING collides with the box sunk by maxUpStep. THE CELL'S OWN
    /// SLAB THE SHAPE TABLE ANSWERS (autopush-423 — the sunk box the
    /// full-cell overlap the 1/16 window wore: the band cell's 15/16
    /// top the sunk bottom rises past the full cube's own verdict
    /// kept, the vanilla noCollision the collision shapes speak). The
    /// probe the [`clip_axis`]'s own vocabulary NOW wears — the ONE
    /// collision vocabulary the sneak guard and the sweep both speak:
    /// the EMPTY rows the skip keeps, the COLUMN row's Y range the
    /// band top speaks (`Block.column(16, 0, 15)` the SHAPE constants
    /// carry), the X/Z footprint the full cell keeps, the strict edges
    /// the vanilla AABB carries.
    fn no_floor_below(&self, world: &World, dx: f64, dz: f64) -> bool {
        let b = box_at(self.x + dx, self.y, self.z + dz, self.height());
        let sunk = (b.0, b.1 - MAX_UP_STEP, b.2, b.3, b.4 - MAX_UP_STEP, b.5);
        // any solid cell's own slab intersecting the sunk box? (the
        // strict edges the vanilla AABB carries)
        let (x0, y0, z0, x1, y1, z1) = sunk;
        let mut hit = false;
        for cx in x0.floor() as i32..=((x1 - 1e-9).floor()) as i32 {
            for cy in y0.floor() as i32..=((y1 - 1e-9).floor()) as i32 {
                for cz in z0.floor() as i32..=((z1 - 1e-9).floor()) as i32 {
                    let shape = crate::world::collision_shape(world.get(cx, cy, cz));
                    if matches!(shape, crate::world::CollisionShape::Empty) {
                        continue;
                    }
                    let (y_lo, y_hi) = match shape {
                        crate::world::CollisionShape::Column(h) => {
                            (cy as f64, cy as f64 + h as f64 / 16.0)
                        }
                        _ => (cy as f64, (cy + 1) as f64),
                    };
                    if (cx as f64) < x1
                        && ((cx + 1) as f64) > x0
                        && y_lo < y1
                        && y_hi > y0
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
        // THE ENTITY'S OWN TICK COUNTER (javap tickNonPassenger pc 8-15
        // — the `tickCount++` the walk's own HEAD serves, before the
        // physics the vanilla order keeps)
        self.tick_count += 1;
        // the per-tick event flags clear at the head (the App drains
        // what THIS tick spoke): the splash's own clear rides here —
        // the fluid update fires it AFTER this line, the step block's
        // own clears never see it. THE FIRE ARC's three ride the same
        // head (the same lesson the splash's zero-forever bug spoke:
        // the tick-head clear, never a later block's)
        self.splash_due = false;
        self.splash_pose = None;
        self.lava_hurt_due = false;
        self.on_fire_hurt_due = false;
        self.in_fire_hurt_due = false;
        self.in_soul_fire_hurt_due = false;
        self.in_campfire_hurt_due = false;
        self.in_soul_campfire_hurt_due = false;
        self.fire_doused_due = false;
        self.drown_hurt_due = false;
        // the pre-tick position — the walk distance law's own source
        // (calculateEntityAnimation's getX() - xo fold) AND the render
        // interpolation's own source (the prev the frame's lerp walks
        // FROM)
        let (x0, y0, z0) = (self.x, self.y, self.z);
        self.prev_x = x0;
        self.prev_y = y0;
        self.prev_z = z0;
        // the eye's own walk head (javap LivingEntity.tick's own
        // `oEyeHeight = eyeHeight` fold): the current smoothed pose
        // becomes the previous, the blend below then eases the crouch
        // the camera's lerp walks between
        self.prev_eye_height = self.eye_height;
        // THE FLYING BODY'S OWN RESET (javap Player.aiStep the 26.2
        // jar — pcs 28-46: `abilities.flying && !isPassenger() ->
        // resetFallDistance()` — the flight the fall distance never
        // banks, the landing hurt the fold never earns; the
        // !passenger gate the ride lanes' own body-sleep speaks
        // structurally — the ride branches never reach this tick)
        if self.flying {
            self.fall_distance = 0.0;
        }
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
        // inWater). THE SECOND FLUID rides the same scan (the lava
        // lane): the verdict the fire clock's !isInLava gate and the
        // fallDistance fold read.
        let in_water = self.touches_water(world);
        let in_lava = self.touches_lava(world);
        self.in_lava = in_lava;
        if in_water || in_lava {
            self.fall_distance = 0.0;
        }
        if in_water {
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
                // THE SPLASH RINGS' OWN POSE (the loops' own read —
                // the live position + the carried velocity at the
                // call, the exact moment the volume's fold speaks)
                self.splash_pose =
                    Some((self.x, self.y, self.z, self.vx, self.vy, self.vz));
            }
        }
        self.was_touching_water = in_water;
        // THE FIRE CLOCK (javap `Entity.baseTick`'s ServerLevel branch —
        // the vanilla's own position right after the fluid update): a
        // positive clock folds the onFire hurt when the cadence gate
        // speaks — `remainingFireTicks % 20 == 0 && !isInLava()` (the
        // in-lava ticks the lava's own 4.0 speaks instead) — then
        // decrements one. The fireImmune arm never speaks here (the
        // bare player never is); the client-side clearFire arm (the
        // level NOT a ServerLevel) never speaks — this body is the
        // stand-in server's own.
        if self.remaining_fire_ticks > 0 {
            if self.remaining_fire_ticks % FIRE_DAMAGE_INTERVAL == 0 && !in_lava {
                self.on_fire_hurt_due = true;
            }
            self.remaining_fire_ticks -= 1;
        }
        // THE BELT-AND-SUSPENDERS FOLD (javap baseTick, the fire
        // clock's own tail): `isInLava() -> fallDistance *= 0.5` —
        // after the fluid scan's reset it rides a zero (the vanilla's
        // own dead math, kept for the shape's honesty).
        if in_lava {
            self.fall_distance *= 0.5;
        }
        // THE AIR CLOCK (javap `LivingEntity.baseTick`'s own branch —
        // the vanilla's position right after the fire clock the
        // super.baseTick chain speaks): the EYE's point cell the gate
        // reads (isEyeInFluid's own query — the eye height the pose
        // speaks, not the feet box the fluid scan walks), the
        // eye-in-water arm drains one per tick (decreaseAirSupply's
        // `air - 1` — the clock goes UNDER zero, the -20 floor the
        // dose's rhythm needs), the floor fold resets the store and
        // rides the dose flag; the else arm refills four a tick
        // (increaseAirSupply's `Math.min(air + 4, max)`). THE DRAIN'S
        // OWN GATES (javap pcs 244-280): the
        // `!canBreatheUnderwater() && !hasWaterBreathing(this) &&
        // (!isPlayer || !abilities.invulnerable)` flag the drain arm
        // stores — the water-breathing body (the MobEffectUtil triple
        // the store's read answers) and the invulnerable body the
        // drain never speak, the bubble-column check the vanilla arm
        // carries rides vocabulary this body never speaks (no bubble
        // blocks in the palette — the honest absence). THE REFILL'S
        // OWN GATES (javap pcs 335-362 the underwater arm, pcs 396-416
        // the dry arm): the DRY body (and the bubble column) the
        // unconditional `air < max` refill keeps, the UNDERWATER
        // drain-refused body the `air < max &&
        // shouldEffectsRefillAirsupply` pair wears — the nautilus-ONLY
        // body the freeze holds (the triple refuses the drain, the
        // ¬NAUTILUS ∨ WB ∨ CP walk refuses the refill), the
        // water-breathing body the refill the true arm grants.
        let eye_water = world.get(
            self.x.floor() as i32,
            (self.y + self.eye_height()).floor() as i32,
            self.z.floor() as i32,
        ) == WATER;
        self.eye_in_water = eye_water;
        if eye_water && !self.invulnerable && !self.water_breathing {
            self.air_supply -= 1;
            if self.air_supply <= DROWN_HURT_FLOOR {
                self.air_supply = 0;
                self.drown_hurt_due = true;
            }
        } else if self.air_supply < TOTAL_AIR_SUPPLY
            && (!eye_water || self.effects_refill_air_supply)
        {
            self.air_supply = (self.air_supply + AIR_REFILL_PER_TICK).min(TOTAL_AIR_SUPPLY);
        }
        if self.no_jump_delay > 0 {
            self.no_jump_delay -= 1;
        }
        // THE POSE (updatePlayerPose's own core): the crouch holds
        // while the key holds AND un-crouches only when the standing
        // box fits (the ceiling never embeds the grown body)
        self.crouching = self.want_crouch || !self.can_stand(world, self.x, self.z);
        // THE EYE'S OWN BLEND (javap the pose section's own tail —
        // `eyeHeight += (f - eyeHeight) * 0.5` the f the pose's eye
        // speaks): the smoothed store walks the crouch over a few
        // ticks, the camera's lerp rides the frame's partial between
        // the two stores
        let eye_target = if self.crouching {
            CROUCH_EYE
        } else {
            STANDING_EYE
        };
        self.eye_height += (eye_target - self.eye_height) * EYE_HEIGHT_BLEND;
        // THE FLYING THRUST (javap LocalPlayer.aiStep pcs 815-894 —
        // the audited 26.2 walk: `abilities.flying &&
        // isControlledCamera()` gates, the int local folds
        // `shift -> -1 / jump -> +1` (the wide-iinc operands the raw
        // bytes pack: local 7, consts -1/+1), and the non-zero fold
        // adds `f * abilities.getFlyingSpeed() * 3.0` to the delta y —
        // the ±0.15 a tick the 0.05 default speaks. The lane rides
        // BEFORE the super's jump fold (the LocalPlayer body precedes
        // the AbstractClientPlayer.aiStep call at pc 1095) — the raw
        // keys the input record carries (jump the `jumping` field,
        // shift the `want_crouch` key state), and the
        // isControlledCamera gate the body's own stand answers: this
        // body IS the camera while its physics walk (the ride lanes
        // never reach the tick, the headless scenes never feed one).
        if self.flying {
            let thrust = (self.jumping as i32) - (self.want_crouch as i32);
            if thrust != 0 {
                // the abilities' own field the thrust multiplies (javap
                // LocalPlayer.aiStep pcs 881-888 — the
                // getFlyingSpeed the ldc_w #1270 fold carries, the
                // wheel the field's own adjustment the spectator
                // arm speaks — the thrust rides the ADJUSTED speed)
                self.vy += thrust as f64 * self.flying_speed as f64 * FLY_THRUST_SCALE;
            }
        }
        // THE BLOCK JUMP FACTOR'S OWN BANK (javap Entity.getBlockJumpFactor
        // pcs 0-48 the 26.2 jar — the feet cell the blockPosition read,
        // the below cell the getBlockPosBelowThatAffectsMyMovement
        // fallback the default-1.0 feet wears, the layered walk the
        // standing-on-sand body answers): the bank the jump gate AND
        // the fly-law's launch hop both read (the same tick's state
        // the App's world-free fold answers — the frame the world
        // never splits).
        self.block_jump_factor = self.block_jump_factor(world);
        // THE JUMP GATE (the aiStep's own): held jump + ground + the
        // delay elapsed -> jumpFromGround
        if self.jumping && self.on_ground && self.no_jump_delay == 0 {
            self.jump_from_ground();
        }
        // THE FALL-FLYING START WALK (javap `LocalPlayer.aiStep` pcs
        // 644-694 the 26.2 jar — the jump key the HELD state reads
        // (the `Input.jump()` the keyPresses record carries), the
        // ride/climb gates the walk's own order keeps, and
        // `Player.tryToStartFallFlying`'s own verdict the start
        // speaks: `!isFallFlying && canGlide() && !isInWater()` — the
        // canGlide the Player's own override answers (the
        // abilities.flying arm the creative flight refuses) over the
        // LivingEntity law (`!onGround && !isPassenger &&
        // !hasEffect(LEVITATION)` + the equipment's canGlideUsing),
        // the levitation effect the vocabulary never banks, the ride
        // lanes the body-tick never reaches). The win the shared flag
        // 7 sets and the wire echo banks.
        if self.jumping
            && !self.fall_flying
            && !self.flying
            && !self.on_ground
            && self.chest_glides
            && !self.was_touching_water
        {
            self.fall_flying = true;
            self.glide_started_due = true;
        }
        // THE FALL-FLYING STOP ARM (javap `LivingEntity.updateFallFlying`
        // the 26.2 walk — the aiStep's own call right before the
        // travel: the server arm's `!canGlide() -> setSharedFlag(7,
        // false)` verdict the single-player body runs directly — the
        // ground the glide retires, the elytra's unequip the verdict
        // answers, the creative flight the Player's own override
        // refuses). The fluid touch the verdict never reads — the
        // flag the water keeps, the exit mid-air the glide resumes
        // (the travel's own fluid-first routing the physics skips
        // while the flag stays).
        if self.fall_flying && !self.fall_flying_can_glide() {
            self.fall_flying = false;
        }
        // THE FALL-FLYING WEAR (javap `LivingEntity.tick` the 26.2
        // walk — the pc 496-504 gate the isFallFlying win the
        // updateFallFlying call speaks right before the travel: the
        // server arm the canGlide win banks, the local `i =
        // fallFlyTicks + 1` fold the `i % 10 == 0` gate the gameEvent
        // ELYTRA_GLIDE answers, the `(i / 10) % 2 == 0` gate the
        // hurtAndBreak(1) rides — the every-20-tick wear the ONE
        // qualifying slot's stack carries: the gliding-slot list the
        // canGlideUsing filter banks holds the CHEST alone, the
        // Util.getRandom pick the single-element list never shakes).
        // The retire tick the stop arm claims — the wear never rides
        // it (the vanilla's gate-then-return shape the same order
        // keeps). The field walk itself the tick()'s own pc 598-620
        // tail banks — the else arm below the reset speaks.
        if self.fall_flying {
            self.fall_fly_ticks += 1;
            let i = self.fall_fly_ticks;
            if i % 10 == 0 {
                if (i / 10) % 2 == 0 {
                    self.elytra_hurt_and_break(1);
                }
                // THE EVENT'S OWN POST (javap updateFallFlying pcs
                // 105-109 — the gameEvent call the i%10 gate's tail
                // answers, the hurt parity the gate never reads: the
                // odd j the hurt refuses, the event STILL speaks) —
                // the position the PRE-travel field the Entity's own
                // `getfield position` walk banks
                self.elytra_glide_event_due = true;
                self.elytra_glide_event_pos = (self.x, self.y, self.z);
            }
        } else {
            // THE RESET ARM (javap `LivingEntity.tick` pcs 598-620 —
            // the `ifeq 618 / iconst_0 / putfield fallFlyTicks` walk
            // the non-gliding tick answers: the grounded tick the
            // clock zeroes, the fresh glide the rhythm restarts — the
            // every-10 gate the FRESH count speaks, the stale field
            // the old walk's across-glide ride the re-exam retires).
            self.fall_fly_ticks = 0;
        }
        // THE GLIDE'S OWN ROUTE (javap `LivingEntity.travelFallFlying`
        // the 26.2 walk — the travelVec arg the glide lane NEVER
        // reads: the input impulse the air lane adds the glide never
        // speaks, the look-steered folds the velocity alone wear; the
        // fluid-first routing the touch lane answers — the folds the
        // touching tick skips, the flag the exit keeps). The folds
        // run BEFORE the move the travel's own order keeps.
        let glide_active = self.fall_flying && !self.no_physics && !self.in_lava && !in_water;
        if glide_active {
            self.glide_folds();
        } else if !glide_active && self.fall_flying {
            // the fluid-touching glide tick — the impulse the air lane
            // adds the glide never speaks (the travelVec arg the fluid
            // route never reads either)
        } else {
        // THE MOVE IMPULSE (getInputVector's law): the input vector
        // (left, forward) normalized past 1.0, scaled by the
        // friction-influenced speed (the GROUND control is the
        // attribute — the cube correction only past friction 0.6, the
        // ice law our vocabulary never speaks; the AIR control is
        // Player.getFlyingSpeed's own split the 26.2 jar speaks: the
        // FLYING body the abilities' 0.05 wears — the sprint-fly the
        // ×2 0.10 arm carries — the walking body the 0.02, the
        // sprint-air the 0.026 arm), rotated by the yaw
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
            let speed = if self.on_ground && !in_water {
                if friction > 0.6 {
                    self.speed(using_item) * CUBE_SPEED_NUMERATOR / (friction * friction * friction)
                } else {
                    self.speed(using_item)
                }
            } else if self.flying {
                if self.sprinting {
                    // the field the sprint-fly's ×2 fold carries (the
                    // getFlyingSpeed's own split — the default 0.05
                    // the 0.10 arm doubles, the wheel the field's own
                    // adjustment the sprint rides too)
                    self.flying_speed * 2.0
                } else {
                    self.flying_speed
                }
            } else if in_water {
                // THE WATER INPUT SEED (javap travelInWater pcs 19-22
                // — the 0.02 fload the seed keeps): the fluid route
                // the on_ground verdict never gates (the wader the
                // SAME seed wears), the WATER_MOVEMENT_EFFICIENCY
                // lerp the bare player's 0.0 attribute default never
                // folds
                WATER_INPUT_SPEED
            } else if self.sprinting {
                SPRINT_AIR_SPEED
            } else {
                FLYING_SPEED
            };
            let speed = speed as f64;
            let rad = self.yaw as f64 * std::f64::consts::PI / 180.0;
            let (s, c) = (rad.sin(), rad.cos());
            self.vx += (ix * c - iz * s) * speed;
            self.vz += (iz * c + ix * s) * speed;
            }
        }
        // THE PRE-TRAVEL VY (javap Player.travel pcs 131-138 — the
        // flying branch's own save: `d2 = getDeltaMovement().y` the
        // super walk's fold the restore reads — the bank the thrust
        // and the jump fold spoke, the move never yet consumed)
        let vy_pre_travel = self.vy;
        // THE COLLIDE (maybeBackOffFromEdge first: the sneak never
        // walks off its ledge)
        let mut mv = (self.vx, self.vy, self.vz);
        // THE STUCK WALK (javap `Entity.move` pcs 97-137 the 26.2 jar —
        // the move's own head, the edge walk the pcs 143 call never
        // precedes): the stuck slot's lengthSqr > 1.0E-7 gate the
        // movement the ×stuck fold scales — the MoverType the != PISTON
        // branch admits (the self-move lane the client body always
        // wears — the piston lane no palette row carries), the slot the
        // ZERO reset, the deltaMovement the ZERO write clear — the
        // momentum the zero-write never lets accumulate (the web's own
        // rhythm: each tick's velocity is the input/gravity the tick
        // itself armed, the drag the post-move walk the ZEROED fields
        // folds). The noPhysics move the pcs 0-58 early return the walk
        // never reaches — the spectator body the slot never banks and
        // the raw add the scale never folds. The edge walk and the
        // collide the SCALED movement consume — the vanilla's own order.
        if !self.no_physics {
            let (sx, sy, sz) = self.stuck_speed;
            if sx * sx + sy * sy + sz * sz > 1.0e-7 {
                mv = (mv.0 * sx, mv.1 * sy, mv.2 * sz);
                self.stuck_speed = (0.0, 0.0, 0.0);
                self.vx = 0.0;
                self.vy = 0.0;
                self.vz = 0.0;
            }
        }
        // THE SNEAK EDGE (maybeBackOffFromEdge first: the sneak never
        // walks off its ledge)
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
        let col = if self.no_physics {
            // THE NOPHYSICS MOVE (javap Entity.move pcs 0-58 — the
            // early return the flag speaks): the RAW position add the
            // collide walk never answers, the four collision flags the
            // branch zeroes — the client's on_ground the
            // verticalCollisionBelow fold speaks, the Player.tick
            // head's spectator arm the same zero wears (pcs 8-26:
            // spectator || passenger -> setOnGround(false)) — and the
            // zeroed Collision the skipped walks never speak: the
            // restitution the empty flags, the step walk the
            // move_dist's own growth gate, the movement exhaustion the
            // cm>0 gate, the fall damage the dy=0 sweep. The travel
            // folds below (the drag, the gravity, the flying restore)
            // the vanilla walk keeps.
            self.x += self.vx;
            self.y += self.vy;
            self.z += self.vz;
            self.horizontal_collision = false;
            self.on_ground = false;
            Collision {
                dx: 0.0,
                dy: 0.0,
                dz: 0.0,
                horizontal_collision: false,
                vertical_collision_below: false,
                collided_x: false,
                collided_y: false,
                collided_z: false,
            }
        } else {
            // THE STEP LADDER'S OWN CALL (the 26.2 collide's ladder the
            // walk itself wears now — the pre-move ground flag the
            // gate reads, the LivingEntity's own 0.6 the step speaks)
            let col = collide_stepped(world, b, mv, self.on_ground, MAX_UP_STEP);
            self.x += col.dx;
            self.y += col.dy;
            self.z += col.dz;
            self.horizontal_collision = col.horizontal_collision;
            self.on_ground = col.vertical_collision_below;
            // THE FALL DISTANCE (doCheckFallDamage's own law over the SWEPT
            // delta — the accumulation, the water's honest gate, the
            // landing's fold-and-reset)
            self.check_fall_damage(world, col.dy);
            col
        };
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
        // THE COLUMN'S OWN DRAG (javap `BubbleColumnBlock.entityInside`'s
        // own beat — the inside-blocks walk the move's tail speaks: the
        // DIRECT onInsideBubbleColumn/onAboveBubbleColumn calls the walk
        // fires per overlapped column cell, BEFORE the applier queue the
        // lava/water effects ride): the whirlpool's pull and the
        // upward's push fold the carried velocity here — the NEXT
        // tick's move consumes it (the vanilla's own rhythm), the
        // inside law's fall-distance reset the walk's tail speaks. The
        // ride gate the App keeps (the rideTick's law: the body's
        // physics never walk while a boat holds the player) means this
        // beat speaks for the STANDING body only — the vanilla's own
        // passenger lane rides the vehicle's walk.
        // THE INSIDE-BLOCKS WALKS THE FLAG REFUSES (javap
        // isAffectedByBlocks — `!isRemoved() && !noPhysics`: the
        // noPhysics body the whole walk skips, the vanilla spectator
        // the three beats never hears)
        if !self.no_physics {
            self.apply_bubble_column_drag(world);
        }
        // THE WEB'S OWN STUCK (javap `WebBlock.entityInside` the 26.2
        // jar — the inside-blocks walk the move's tail speaks, the
        // DIRECT makeStuckInBlock call the bubble column's drag calls
        // the same per-cell lane wears): the overlapped web cell the
        // clamp vec banks — the LivingEntity arm the WEAVING verdict
        // the (0.5, 0.25, 0.5) wider clamp speaks, the bare body the
        // (0.25, 0.05000000074505806, 0.25) pair keeps (the 0.05F the
        // bytecode's own widened double — the fconst's d2f-less literal
        // the ldc2_w banks) — and the makeStuckInBlock walk's own tail
        // the fall distance RESETS (resetFallDistance the dconst_0
        // putfield keeps — the web the fall's bank never lets stack,
        // the landing hurt the walk denies). The NEXT move's head the
        // clamp consumes (the stuck walk's own law), the overlap the
        // every-tick walk re-banks (the stationary body the box arm
        // keeps — the bubble's own comment the rhythm speaks).
        if !self.no_physics && self.touches_web(world) {
            self.apply_web_stuck();
        }
        // THE LAVA CONTACT (javap `LavaFluid.entityInside`'s own beat —
        // the aiStep tail's applyEffectsFromBlocks the tick's move
        // speaks AFTER, the InsideBlockEffectApplier's order kept:
        // CLEAR_FREEZE (no vocabulary — no freeze lane), then
        // LAVA_IGNITE, then the runAfter tail): the box overlaps a
        // lava cell THIS tick -> lavaIgnite (fireImmune aside, the
        // bare player never is — igniteForSeconds(15) ->
        // igniteForTicks(floor(15 * 20) = 300): ONLY when the clock is
        // under the brand's value) and lavaHurt (the 4.0 the App's
        // hurt pipeline answers through its own cooldown, the
        // GENERIC_BURN riding the hurt's success — the playSound's
        // own ifeq gate).
        if !self.no_physics && self.touches_lava(world) {
            if self.remaining_fire_ticks < LAVA_FIRE_TICKS {
                self.remaining_fire_ticks = LAVA_FIRE_TICKS;
            }
            self.lava_hurt_due = true;
        }
        // THE FIRE KINDS' OWN ENTITY-INSIDE (javap
        // `BaseFireBlock.entityInside` the 26.2 jar — the same
        // inside-blocks walk the lava/water beats ride, the InsideBlock
        // EffectApplier's order kept: CLEAR_FREEZE (no vocabulary —
        // the freeze lane the repo never carries), then FIRE_IGNITE =
        // `BaseFireBlock::fireIgnite` (fireImmune aside — the bare
        // player never is; the negative arm the no-freeze clock never
        // speaks; the ServerPlayer +1..2 arm the rng-less replay the
        // splash's own (r - r) precedent folds — the arm the body's
        // walk never banks; the igniteForSeconds(8.0F) ->
        // igniteForTicks(floor(8 * 20) = 160) the `remainingFireTicks
        // < 160` lift the verdict wears — never a lower), then the
        // runAfter hurt(inFire(), fireDamage) the hurt pipeline's own
        // cooldown answers through its own invulnerable beat. The
        // SOUL FIRE the shared parent walk keeps VERBATIM — the kind
        // the fireDamage float splits (the 2.0 the ctor's fconst_2
        // speaks). The cells the noCollision shape the body walks
        // THROUGH brand it standing (the box overlap the scan
        // speaks); the box walk's own cell order the vanilla's
        // per-block entityInside fires ride — the two kinds' verdicts
        // the pair of scans speaks, the water douse the NEXT beat
        // still wins (the applier's queue order the inside walk
        // keeps).
        if !self.no_physics && self.touches_fire(world) {
            if self.remaining_fire_ticks < FIRE_IGNITE_TICKS {
                self.remaining_fire_ticks = FIRE_IGNITE_TICKS;
            }
            self.in_fire_hurt_due = true;
        }
        if !self.no_physics && self.touches_soul_fire(world) {
            if self.remaining_fire_ticks < FIRE_IGNITE_TICKS {
                self.remaining_fire_ticks = FIRE_IGNITE_TICKS;
            }
            self.in_soul_fire_hurt_due = true;
        }
        // THE LIT CAMPFIRES' OWN DOSE (the dev-6 round — javap
        // CampfireBlock.entityInside: the LIT verdict + the
        // LivingEntity gate the DIRECT hurt call serves — the NO
        // ignite lift the BaseFireBlock walk's applier phases alone
        // wear, the fireDamage int the plain/soul ctor rows split):
        // the lit cells the box overlap the dose flags bank, the
        // doused kinds and the dark cells the scan never touches
        // (the !LIT verdict the kind-only palette the unlit rows
        // keep). The DOSE WINS over nothing — the water douse the
        // NEXT beat still wins (the applier's queue order the
        // inside walk keeps).
        if !self.no_physics && self.touches_campfire(world) {
            self.in_campfire_hurt_due = true;
        }
        if !self.no_physics && self.touches_soul_campfire(world) {
            self.in_soul_campfire_hurt_due = true;
        }
        // THE WATER'S EXTINGUISH (javap `WaterFluid.entityInside`'s own
        // InsideBlockEffectType.EXTINGUISH = `Entity::clearFire` — the
        // same contact beat the lava's brand rides, the inside-effect
        // order the applier queue speaks): the burning body touching
        // water dies to min(0, remaining) — for a positive clock the
        // fold speaks zero — and the transition plays the fizz (the
        // volume 0.7 / pitch 1.6 + (r - r) * 0.4 fold the drain
        // speaks). A wet body carries no clock to kill — the gate
        // stays silent. The DOUSE WINS over the brand (the water-after-
        // lava order — the jump-into-the-pond exit the game itself
        // speaks).
        if !self.no_physics && self.remaining_fire_ticks > 0 && self.touches_water(world) {
            self.remaining_fire_ticks = 0;
            self.fire_doused_due = true;
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
        // THE BLOCK SPEED FACTOR (javap Entity.move's own TAIL the 26.2
        // jar — pcs 652-672: getBlockSpeedFactor the fstore 12 banks,
        // setDeltaMovement(getDeltaMovement().multiply(f, 1.0, f)) the
        // dconst_1 y the multiply keeps): the POST-MOVE cells the
        // tail's own read answers (the strided body the new cell's
        // factor the same tick banks — the feet the air above the sand
        // wears, the below cell the default-1.0 feet falls through),
        // the x/z ONLY the fold scales, the drag walk the scaled pair
        // folds (the travelInAir's friction the same single-tick order
        // after — the vanilla move INSIDE the travel's own walk sits).
        // The noPhysics move the pcs 0-58 early return the tail never
        // reaches — the spectator body the factor never folds.
        if !self.no_physics {
            let f = self.block_speed_factor(world) as f64;
            self.vx *= f;
            self.vz *= f;
        }
        // THE DRAGS + THE GRAVITY (travelInAir's own order): the
        // horizontal decay `friction * 0.91`, the vertical 0.98 with
        // the gravity subtracted BEFORE it. THE GLIDE LANE THE FOLD
        // SKIPS (travelFallFlying's own tail — the glide's own
        // 0.99/0.98 drag the velocity fold itself spoke, the air
        // lane's post-move walk the glide never wears; the
        // fluid-touching tick the folds skipped the air stand-in
        // wears, the body's fluid lane the vocabulary keeps)
        let friction = if self.on_ground { BLOCK_FRICTION } else { AIR_FRICTION };
        if !glide_active {
            if in_water && !self.flying {
                // THE WATER LANE (javap `LivingEntity.travelInWater` the
                // 26.2 jar — pcs 0-186 the FULL walk the fluid route
                // keeps): the drag seed the sprint 0.9 /
                // getWaterSlowDown 0.8 pair wears, the Dolphins Grace
                // override the 0.96 glide speaks, the
                // WATER_MOVEMENT_EFFICIENCY lerp the bare player's 0.0
                // attribute default never folds (the depth-strider
                // enchant the stacks' data absence keeps), the
                // post-move multiply(drag, 0.8, drag) fold the
                // velocity decay carries.
                let mut drag = if self.sprinting {
                    WATER_DRAG_SPRINT as f64
                } else {
                    WATER_DRAG as f64
                };
                if self.dolphins_grace {
                    drag = DOLPHINS_GRACE_WATER_DRAG as f64;
                }
                self.vx *= drag;
                self.vz *= drag;
                self.vy *= 0.8f32 as f64;
                // THE FALLING-ADJUSTED GRAVITY (javap
                // getFluidFallingAdjustedMovement pcs 0-100): the
                // sprint body the fold SKIPS (pc 7 the isSprinting
                // ifne 98 keeps — the sprint-swim the buoyancy never
                // pays), the falling flag the pre-impulse vy the
                // travelInFluid read answers (vy_pre_travel — the
                // impulse never touches vy), the terminal-sink clamp
                // the |dy-0.005| / |dy-g/16| pair answers.
                if !self.sprinting {
                    let dy = self.vy;
                    self.vy = if vy_pre_travel <= 0.0
                        && (dy - 0.005).abs() >= 0.003
                        && (dy - GRAVITY / 16.0).abs() < 0.003
                    {
                        -0.003
                    } else {
                        dy - GRAVITY / 16.0
                    };
                }
            } else {
                self.vy = (self.vy - GRAVITY) * VERTICAL_DRAG as f64;
                let drag = (friction * GROUND_DRAG) as f64;
                self.vx *= drag;
                self.vz *= drag;
            }
        }
        if self.flying {
            // THE PLAYER.TRAVEL RESTORE (javap the 26.2 jar pcs
            // 121-163 — the flying branch's own tail: `d2 = the saved
            // pre-travel y` the super walk spent the gravity+drag fold
            // through, then `setDeltaMovement(delta.with(Axis.Y, d2 *
            // 0.6))` the carried vy REPLACES — the fold's own vy dies
            // in the overwrite, the hover the restore alone speaks:
            // the gravity never banks, the carried vy decays 0.6 a
            // tick, the thrust the next tick's lane re-adds)
            self.vy = vy_pre_travel * FLY_VY_RESTORE;
        }
        // THE WALK ANIMATION (LivingEntity.calculateEntityAnimation(true)
        // + updateWalkAnimation, javap): the tick's 3D move distance,
        // `min(distance * 4.0, 1.0)` into the store's 0.4 blend. The
        // noPhysics move the store never feeds (javap Entity.move's
        // early return skips walkAnimation.update — the store holds,
        // the flying pose the spectator keeps)
        if !self.no_physics {
            let dist = ((self.x - x0) * (self.x - x0)
                + (self.y - y0) * (self.y - y0)
                + (self.z - z0) * (self.z - z0))
                .sqrt() as f32;
            if dist.is_finite() {
                self.walk.update((dist * WALK_DISTANCE_SCALE).min(1.0), WALK_BLEND, 1.0);
            } else {
                self.walk.stop();
            }
        }
        // THE CAMERA'S OWN FOV BEAT (Camera.tickFov — the modifier the
        // projection's 70° option multiplies): the target the speed
        // attribute's ratio speaks, the half-step the tick arms, the
        // clamp the bytecode pins.
        self.tick_fov(self.fov_target(using_item));
        // THE FIRST-TICK GATE (Entity.firstTick's own clearing): the
        // body's first 50ms beat spoke its lanes splash-free — the
        // gate opens
        self.first_tick = false;
        // THE WINGS ANIM'S OWN TICK (javap LivingEntity.tick the 26.2
        // jar — the `elytraAnimationState.tick()` call the tail's own
        // walk keeps, pcs 639-643 right after the fallFlyTicks walk +
        // the isSleeping xRot reset + the refreshDirtyAttributes
        // answer, the RETURN the next fold never reaches): EVERY tick
        // the state eases — gliding, crouching, or standing.
        self.tick_elytra_wing_anim();
    }

    /// THE WINGS ANIM'S OWN TICK (javap `ElytraAnimationState.tick` the
    /// 26.2 jar — the exact branch order the bytecode keeps):
    /// 1) the old fields bank the current (`rotXOld = rotX`, the y/z
    ///    pair the same order keeps) — the render lerp's own source;
    /// 2) THE GLIDE ARM — `isFallFlying()`: `f = 1.0F`, the falling
    ///    velocity (`v.y < 0`) the normalize + `1.0F - (float)Math.pow(
    ///    -v.normalize().y, 1.5D)` fold eases it — the steeper the
    ///    descent the further the lerp drives;
    ///    the targets `Mth.lerp(f, 0.2617994F, 0.34906584F)` (x),
    ///    `Mth.lerp(f, -0.2617994F, -1.5707964F)` (z), y 0.0;
    /// 3) THE CROUCH ARM — the triple (0.6981317, 0.08726646,
    ///    -0.7853982) the folded-back pose wears;
    /// 4) THE REST ARM — the DEFAULT pair (0.2617994, 0, -0.2617994);
    /// 5) THE EASE — `rot += (target - rot) * 0.3F`, the x/y/z order
    ///    the bytecode keeps (the y target the f3 slot speaks, the z
    ///    the f2 — the branch's own store order the reads mirror).
    fn tick_elytra_wing_anim(&mut self) {
        self.elytra_wing_rot_x_old = self.elytra_wing_rot_x;
        self.elytra_wing_rot_y_old = self.elytra_wing_rot_y;
        self.elytra_wing_rot_z_old = self.elytra_wing_rot_z;
        let (tx, ty, tz) = if self.fall_flying {
            let mut f = 1.0_f32;
            if self.vy < 0.0 {
                // the f64 normalize the dcmpg branch keeps (the Vec3
                // math never narrows until the d2f the pow's tail
                // speaks)
                let len = (self.vx * self.vx + self.vy * self.vy + self.vz * self.vz).sqrt();
                if len > 0.0 {
                    f = 1.0 - (-(self.vy / len)).powf(ELYTRA_WING_POW) as f32;
                }
            }
            (
                ELYTRA_WING_DEFAULT_X + f * (ELYTRA_WING_GLIDE_X_END - ELYTRA_WING_DEFAULT_X),
                0.0,
                ELYTRA_WING_DEFAULT_Z + f * (ELYTRA_WING_GLIDE_Z_END - ELYTRA_WING_DEFAULT_Z),
            )
        } else if self.crouching {
            (
                ELYTRA_WING_CROUCH_X,
                ELYTRA_WING_CROUCH_Y,
                ELYTRA_WING_CROUCH_Z,
            )
        } else {
            (ELYTRA_WING_DEFAULT_X, 0.0, ELYTRA_WING_DEFAULT_Z)
        };
        self.elytra_wing_rot_x += (tx - self.elytra_wing_rot_x) * ELYTRA_WING_EASE;
        self.elytra_wing_rot_y += (ty - self.elytra_wing_rot_y) * ELYTRA_WING_EASE;
        self.elytra_wing_rot_z += (tz - self.elytra_wing_rot_z) * ELYTRA_WING_EASE;
    }

    /// THE RENDER'S OWN LERP (javap `ElytraAnimationState.getRotX/Y/Z`
    /// — `Mth.lerp(partialTick, rotXOld, rotX)` the extraction walk
    /// serves): the wings' pose at the frame's own moment, the sub-tick
    /// trail the partial eases.
    pub fn elytra_wing_rot_at(&self, partial: f32) -> (f32, f32, f32) {
        let lerp = |o: f32, c: f32| o + partial * (c - o);
        (
            lerp(self.elytra_wing_rot_x_old, self.elytra_wing_rot_x),
            lerp(self.elytra_wing_rot_y_old, self.elytra_wing_rot_y),
            lerp(self.elytra_wing_rot_z_old, self.elytra_wing_rot_z),
        )
    }

    /// THE GLIDE VERDICT (javap `LivingEntity.canGlide` the 26.2 walk
    /// over the Player's own override: `abilities.flying` the creative
    /// flight refuses FIRST, then `!onGround() && !isPassenger() &&
    /// !hasEffect(LEVITATION)` + the equipment's `canGlideUsing` —
    /// the levitation effect the vocabulary never banks, the ride
    /// lanes the body-tick never reaches, the equipment the chest
    /// verdict's feed answers).
    /// THE LAST POINT'S OWN REFUSAL (javap `canGlideUsing` the 26.2
    /// walk — the nextDamageWillBreak gate the walk's tail keeps:
    /// `damage >= maxDamage - 1` the verdict refuses, the torn elytra
    /// the glide never rides — the elytra at its last durability
    /// point retires the glide, the break the glide's own reach
    /// never speaks).
    fn fall_flying_can_glide(&self) -> bool {
        !self.flying
            && !self.on_ground
            && self.chest_glides
            && self.elytra_damage < ELYTRA_MAX_DAMAGE - 1
    }

    /// THE STACK'S OWN VERDICT (javap `ItemStack.nextDamageWillBreak`
    /// the 26.2 walk — the isDamageableItem gate the ifeq answers, the
    /// `damage >= maxDamage - 1` threshold the if_icmplt compares,
    /// pc 17 the crossing speaks): the `minecraft:broken` item-model
    /// condition's own truth (the client's
    /// `properties.conditional.Broken.get` the raw verdict reads) —
    /// the worn elytra's ICON the verdict wears (the inventory's
    /// armor row the draw consults, the elytra_broken model the
    /// on_true branch carries), the glide the same threshold refuses
    /// (fall_flying_can_glide's own tail — the torn elytra's two
    /// evidences the one law pairs).
    pub fn elytra_next_damage_will_break(&self) -> bool {
        self.elytra_damage >= ELYTRA_MAX_DAMAGE - 1
    }

    /// THE HURT LAW (javap `ItemStack.hurtAndBreak` the 26.2 walk —
    /// the processDurabilityChange's hasInfiniteMaterials gate the
    /// creative stack never carries, the applyDamage's `damage +=
    /// amount` the setDamageValue answer, the isBroken arm (`damage
    /// >= maxDamage`) the shrink + onEquippedItemBroken pair speaks:
    /// the entityEvent 47 the CHEST break broadcasts, the client's
    /// breakItem the sound (v0.8, p0.8+r*0.4) + the item particles
    /// (spawnItemParticles(stack, 5)) answer — the App's drain the
    /// due flag rides). The enchantment fold the banked vocabulary
    /// never carries — the unbreaking-less stack the raw damage
    /// keeps.
    fn elytra_hurt_and_break(&mut self, amount: i32) {
        if self.infinite_materials {
            return;
        }
        self.elytra_damage += amount;
        if self.elytra_damage >= ELYTRA_MAX_DAMAGE {
            self.elytra_broke_due = true;
        }
    }

    /// THE GLIDE'S OWN VELOCITY FOLD (javap
    /// `LivingEntity.updateFallFlyingMovement` the 26.2 walk — the
    /// exact bytecode order the travel lane wears, the look vector
    /// the throw lane's own convention speaks):
    /// A) the gravity fold — `vy += gravity * (-1.0 + 0.75 *
    ///    cos²(pitch))` (the getEffectiveGravity read, the bare
    ///    player's 0.08 the attribute base keeps; the slow-falling
    ///    min the effect lane never banks);
    /// B) the dive conversion — falling AND a horizontal look:
    ///    `t = vy * -0.1 * cos²` (positive while falling), the
    ///    velocity `+= (look.x * t / d4, t, look.z * t / d4)` — the
    ///    descent's own speed the forward thrust and the lift wear
    ///    (pc 80-135: the Y add the `dload 12` ALONE speaks);
    /// C) the climb arm — a negative pitch (the look UP) and a
    ///    horizontal look: `u = horizSpeed * -sin(pitch) * 0.04`,
    ///    the velocity `+= (-look.x * u / d4, u * 3.2, -look.z * u /
    ///    d4)` — the backward drag and the 3.2-scaled lift the
    ///    climb's own deceleration speaks (the Mth.sin f32 the
    ///    narrowing keeps);
    /// D) the alignment — a horizontal look: the horizontal velocity
    ///    steers toward the look DIRECTION at the entry horizontal
    ///    speed's magnitude, `+= (target - v) * 0.1` per axis (the
    ///    d6 the ENTRY snapshot keeps, the folds' own adds the
    ///    retarget never reads);
    /// E) the drag — the velocity `*= (0.99, 0.98, 0.99)` (the f32
    ///    constants the promotion's own bits keep).
    fn glide_folds(&mut self) {
        let xrad = self.pitch * 0.017453_292_f32;
        let yrad = self.yaw * 0.017453_292_f32;
        // the look vector (the degrees the same narrowing the throw
        // lane's f8..f11 walk speaks: the f64 sin/cos narrowed to f32;
        // the Y component the folds never read — the pitch's cos² and
        // the vy the walk's own forms carry)
        let (lx, _ly, lz) = (
            (-(yrad as f64).sin() * (xrad as f64).cos()) as f64,
            -((xrad as f64).sin()),
            ((yrad as f64).cos() * (xrad as f64).cos()),
        );
        let d4 = lx * lx + lz * lz; // the square the sqrt gate reads
        let d4 = if d4 > 0.0 { d4.sqrt() } else { 0.0 };
        // d6 — the ENTRY horizontal speed (the snapshot the alignment
        // reads, the folds' own adds never re-read)
        let d6 = (self.vx * self.vx + self.vz * self.vz).sqrt();
        let cos2 = {
            let c = (xrad as f64).cos();
            c * c
        };
        // A) the gravity fold
        self.vy += GRAVITY * (-1.0 + 0.75 * cos2);
        // B) the dive conversion (the post-A vy the gate reads)
        if self.vy < 0.0 && d4 > 0.0 {
            let t = self.vy * -0.1 * cos2;
            self.vx += lx * t / d4;
            self.vy += t;
            self.vz += lz * t / d4;
        }
        // C) the climb arm (the f32 pitch gate the fcmpg keeps)
        if xrad < 0.0 && d4 > 0.0 {
            let u = d6 * (-((xrad as f64).sin() as f32) as f64) * 0.04;
            self.vx += -lx * u / d4;
            self.vy += u * 3.2;
            self.vz += -lz * u / d4;
        }
        // D) the alignment
        if d4 > 0.0 {
            self.vx += ((lx / d4) * d6 - self.vx) * 0.1;
            self.vz += ((lz / d4) * d6 - self.vz) * 0.1;
        }
        // E) the drag (the f32 constants the f64 promotion wears)
        self.vx *= 0.99_f32 as f64;
        self.vy *= 0.98_f32 as f64;
        self.vz *= 0.99_f32 as f64;
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
                    self.safe_fall_distance,
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

    /// The lava lane's own gate (the vanilla `isInLava`'s box shape —
    /// the same box-overlap walk the water law speaks): TRUE when any
    /// cell the body box overlaps is lava — the SECOND fluid kind the
    /// palette carries.
    fn touches_lava(&self, world: &World) -> bool {
        let (x0, y0, z0, x1, y1, z1) = box_at(self.x, self.y, self.z, self.height());
        for cy in y0.floor() as i32..=((y1 - 1e-9).floor()) as i32 {
            for cx in x0.floor() as i32..=((x1 - 1e-9).floor()) as i32 {
                for cz in z0.floor() as i32..=((z1 - 1e-9).floor()) as i32 {
                    if world.get(cx, cy, cz) == crate::world::LAVA {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// THE FIRE CELL'S OWN GATE (the vanilla `isInBlock`'s box shape —
    /// the SAME box-overlap walk the lava lane speaks): TRUE when any
    /// cell the body box overlaps is a fire kind — the entityInside
    /// walk the move's tail speaks (the FireBlock's noCollision shape
    /// the body walks THROUGH, the contact the walk then brands).
    fn touches_fire(&self, world: &World) -> bool {
        let (x0, y0, z0, x1, y1, z1) = box_at(self.x, self.y, self.z, self.height());
        for cy in y0.floor() as i32..=((y1 - 1e-9).floor()) as i32 {
            for cx in x0.floor() as i32..=((x1 - 1e-9).floor()) as i32 {
                for cz in z0.floor() as i32..=((z1 - 1e-9).floor()) as i32 {
                    if world.get(cx, cy, cz) == crate::world::FIRE {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// THE SOUL FIRE CELL'S OWN GATE (the same box-overlap walk — the
    /// kind the damage doubles, the ignite the shared parent keeps).
    fn touches_soul_fire(&self, world: &World) -> bool {
        let (x0, y0, z0, x1, y1, z1) = box_at(self.x, self.y, self.z, self.height());
        for cy in y0.floor() as i32..=((y1 - 1e-9).floor()) as i32 {
            for cx in x0.floor() as i32..=((x1 - 1e-9).floor()) as i32 {
                for cz in z0.floor() as i32..=((z1 - 1e-9).floor()) as i32 {
                    if world.get(cx, cy, cz) == crate::world::SOUL_FIRE {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// THE LIT CAMPFIRE'S OWN GATE (the dev-6 round — the body lane's
    /// own box-overlap scan the fire scans wear: javap
    /// CampfireBlock.entityInside the LIT verdict the lit kind alone
    /// admits — the doused cells and the dark unlit kind the scan
    /// never touches, the `!LIT` verdict the kind-only palette the
    /// unlit rows keep).
    fn touches_campfire(&self, world: &World) -> bool {
        let (x0, y0, z0, x1, y1, z1) = box_at(self.x, self.y, self.z, self.height());
        for cy in y0.floor() as i32..=((y1 - 1e-9).floor()) as i32 {
            for cx in x0.floor() as i32..=((x1 - 1e-9).floor()) as i32 {
                for cz in z0.floor() as i32..=((z1 - 1e-9).floor()) as i32 {
                    if world.get(cx, cy, cz) == crate::world::CAMPFIRE_LIT {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// THE LIT SOUL CAMPFIRE'S OWN GATE (the same box-overlap walk —
    /// the soul kind the fireDamage int doubles).
    fn touches_soul_campfire(&self, world: &World) -> bool {
        let (x0, y0, z0, x1, y1, z1) = box_at(self.x, self.y, self.z, self.height());
        for cy in y0.floor() as i32..=((y1 - 1e-9).floor()) as i32 {
            for cx in x0.floor() as i32..=((x1 - 1e-9).floor()) as i32 {
                for cz in z0.floor() as i32..=((z1 - 1e-9).floor()) as i32 {
                    if world.get(cx, cy, cz) == crate::world::SOUL_CAMPFIRE_LIT {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// THE WEB CELL'S OWN GATE (the vanilla `isInBlock`'s box shape —
    /// the SAME box-overlap walk the lava/fire scans wear): TRUE when
    /// any cell the body box overlaps is the cobweb — the noCollision
    /// shape the body walks THROUGH (the collision's Empty row the
    /// palette wake keeps), the contact the inside walk brands.
    fn touches_web(&self, world: &World) -> bool {
        let (x0, y0, z0, x1, y1, z1) = box_at(self.x, self.y, self.z, self.height());
        for cy in y0.floor() as i32..=((y1 - 1e-9).floor()) as i32 {
            for cx in x0.floor() as i32..=((x1 - 1e-9).floor()) as i32 {
                for cz in z0.floor() as i32..=((z1 - 1e-9).floor()) as i32 {
                    if world.get(cx, cy, cz) == crate::world::COBWEB {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// THE WEB'S OWN MAKE-STUCK (javap `WebBlock.entityInside` pcs 0-70
    /// the 26.2 jar + `Entity.makeStuckInBlock` pcs 0-9): the clamp vec
    /// the LivingEntity/WEAVING arm picks — the bare body the
    /// `(0.25, 0.05000000074505806, 0.25)` pair keeps (the 0.05F the
    /// bytecode's own widened double — the float literal the dconst
    /// never rounds), the weaving body the `(0.5, 0.25, 0.5)` wider
    /// arm speaks — and the stuck walk's own head the fall distance
    /// RESETS (resetFallDistance the dconst_0 putfield keeps) BEFORE
    /// the slot the vec write banks. The player body IS the
    /// LivingEntity the instanceof gate admits (the verdict the
    /// weaving flag alone carries — the store sync the App's tick
    /// banks).
    pub fn apply_web_stuck(&mut self) {
        // resetFallDistance() — the dconst_0 putfield the bytecode
        // keeps (the web the banked descent never lets land)
        self.fall_distance = 0.0;
        // the WEAVING arm's Vec3 first (the pc 44-57 walk — the
        // astore 7 the wider pair banks), the bare pair the pc 0-16
        // walk keeps (the ldc2_w 0.05000000074505806 the widened
        // float rides)
        self.stuck_speed = if self.weaving {
            (0.5, 0.25, 0.5)
        } else {
            (0.25, 0.05000000074505806, 0.25)
        };
    }

    /// THE TWO CELLS THE FACTOR LANES READ (javap getBlockSpeedFactor /
    /// getBlockJumpFactor the 26.2 jar): the FEET cell the
    /// blockPosition() floor speaks, the BELOW cell the
    /// getOnPos(0.500001f) fold answers — the mainSupportingBlockPos
    /// lane the body's own x/z fold keeps (the supporting-pos lane a
    /// future round serves — the flat ground the two answers never
    /// split), the offset the widen keeps (the > 0.5 arm the fence
    /// family's tag walks never reach — the palette the fences never
    /// carried), the below-world the STONE row the world.get speaks.
    fn factor_cells(&self, world: &World) -> (u16, u16) {
        let fx = self.x.floor() as i32;
        let fz = self.z.floor() as i32;
        let feet = world.get(fx, self.y.floor() as i32, fz);
        let below = world.get(fx, (self.y - 0.500001f64).floor() as i32, fz);
        (feet, below)
    }

    /// THE KIND'S OWN FACTOR READ (the networked vocabulary the stamped
    /// ids never names — the synthesized kinds beyond the palette the
    /// NAME_OF bound guards, the 1.0 default the anonymous stamp rides:
    /// the bubble column's own drag the direct lane carries, the
    /// factor the no-name default keeps — the vanilla column's own
    /// getSpeedFactor the same 1.0 answers).
    fn kind_speed_factor(id: u16) -> f32 {
        crate::world::NAME_OF
            .get(id as usize)
            .map(|name| mc_registry::generated::block_factors::speed_factor(name))
            .unwrap_or(1.0)
    }

    /// THE KIND'S OWN JUMP READ (the same bound the stamped ids keep).
    fn kind_jump_factor(id: u16) -> f32 {
        crate::world::NAME_OF
            .get(id as usize)
            .map(|name| mc_registry::generated::block_factors::jump_factor(name))
            .unwrap_or(1.0)
    }

    /// THE BLOCK SPEED FACTOR (javap Entity.getBlockSpeedFactor pcs
    /// 0-70 the 26.2 jar): the feet cell's getSpeedFactor the
    /// generated table answers — the WATER cell the pass-through arm
    /// keeps (the below-cell fallback the in-water body never
    /// reaches: the pond above the sand the swim never slows — the
    /// BUBBLE_COLUMN kind the palette never carried, the arm's second
    /// disjunct the silence wears); the DEFAULT 1.0 feet the BELOW
    /// cell's factor falls through (the layered walk the
    /// standing-on-sand body answers — the feet the air above wears,
    /// the sand the below cell banks); the LivingEntity override's
    /// MOVEMENT_EFFICIENCY lerp the bare body's 0.0 attribute never
    /// folds (lerp(0, f, 1.0) = f — the attribute lane the stacks'
    /// data absence keeps). THE MOVE'S OWN TAIL consumes (Entity.move
    /// pcs 652-672): the x/z ONLY, the post-move cells the tail's own
    /// read answers.
    pub fn block_speed_factor(&self, world: &World) -> f32 {
        let (feet, below) = self.factor_cells(world);
        if feet == crate::world::WATER {
            return Self::kind_speed_factor(feet);
        }
        let f = Self::kind_speed_factor(feet);
        if f == 1.0 {
            return Self::kind_speed_factor(below);
        }
        f
    }

    /// THE BLOCK JUMP FACTOR (javap Entity.getBlockJumpFactor pcs
    /// 0-48 the 26.2 jar): the feet cell's getJumpFactor, the DEFAULT
    /// 1.0 the below cell falls through — NO water arm the walk
    /// carries (the jump the in-water body the on_ground gate never
    /// arms — the lane the gate never reaches). The getJumpPower's own
    /// multiply the jump_from_ground fold consumes (the attribute ×
    /// mult × factor + boost order the f2d widen keeps).
    pub fn block_jump_factor(&self, world: &World) -> f32 {
        let (feet, below) = self.factor_cells(world);
        let f = Self::kind_jump_factor(feet);
        if f == 1.0 {
            return Self::kind_jump_factor(below);
        }
        f
    }

    /// THE COLUMN'S OWN WALK (javap `BubbleColumnBlock.entityInside`'s
    /// own branch over the inside-blocks walk the move's tail speaks —
    /// the lambda$checkInsideBlocks$0's own flag the box's overlap
    /// answers: `moved > 1 block || box.intersects(cell)` — the
    /// stationary body the box-overlap arm keeps DRAGGING): the body's
    /// (deflated 1e-5) box over the column cells, the per-cell law the
    /// drag bit keeps — the SURFACE cell (the above cell air-like: the
    /// empty collision shape AND the empty fluid the bytecode checks)
    /// the launch/sink caps speak, the body cell the steady drag — the
    /// fall distance the INSIDE law resets (the surface law never does:
    /// the launch keeps the fall's honest arc the landing reads). The
    /// AIR-LIKE read rides the project's own vocabulary: the empty cell
    /// the grid names AIR (the plant shapes the collision lane never
    /// arms). The both-vocabularies read (the offline's real ids, the
    /// networked WATER + variant stamp) the materialize walk keeps.
    /// The x/z the law never touches — the column's pull is VERTICAL.
    pub fn apply_bubble_column_drag(&mut self, world: &World) {
        let e = 1e-5f64;
        let (bx0, by0, bz0, bx1, by1, bz1) = box_at(self.x, self.y, self.z, self.height());
        let x0 = (bx0 + e).floor() as i32;
        let x1 = (bx1 - e).floor() as i32;
        let y0 = (by0 + e).floor() as i32;
        let y1 = (by1 - e).floor() as i32;
        let z0 = (bz0 + e).floor() as i32;
        let z1 = (bz1 - e).floor() as i32;
        for wz in z0..=z1 {
            for wy in y0..=y1 {
                for wx in x0..=x1 {
                    let cell = world.get(wx, wy, wz);
                    let variant = crate::world_view::column_variant_at(world, wx, wy, wz, cell);
                    if variant == 0 {
                        continue;
                    }
                    let surface = world.get(wx, wy + 1, wz) == AIR;
                    self.vy = bubble_column_drag(self.vy, variant == 1, surface);
                    if !surface {
                        self.fall_distance = 0.0;
                    }
                }
            }
        }
    }

    /// THE SPLASH RINGS' OWN DRAIN (doWaterSplashEffect's loops the
    /// App replays after the sound fold): the pose the entry gate
    /// captured, taken once.
    pub fn take_splash_pose(&mut self) -> Option<(f64, f64, f64, f64, f64, f64)> {
        self.splash_pose.take()
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

    /// THE RENDER FOLD (javap `Entity.render`'s own lerp — `Mth.lerp(
    /// partialTicks, xo, x)` the renderer's pose rides): the feet the
    /// frame walks ON — the previous tick's feet carried `partial` of
    /// the way to the current. The partial >= 1.0 arm speaks the
    /// CURRENT feet bit for bit (the headless lanes' byte-identical
    /// contract — the fixed clock's remainder never touches them).
    pub fn lerped_pos(&self, partial: f32) -> (f64, f64, f64) {
        if partial >= 1.0 {
            return (self.x, self.y, self.z);
        }
        let t = partial as f64;
        (
            self.prev_x + (self.x - self.prev_x) * t,
            self.prev_y + (self.y - self.prev_y) * t,
            self.prev_z + (self.z - self.prev_z) * t,
        )
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

    /// THE SEGMENT'S OWN HONESTY (the t parameter the walk serves —
    /// the hit's own comment speaks t=1 IS the full from->to span):
    /// a wall BEYOND the segment's end never clips (the walk stops
    /// at t=1 — the len-compare the march spoke ran the ray |delta|
    /// times its own length, the arrow the wall two segments away
    /// "hit" mid-flight). The wall at x=5 stands 0.5 past the first
    /// segment's end: the honest walk answers None; the reaching
    /// segment clips AT the wall's own face.
    #[test]
    fn clip_ray_never_marches_past_the_segment_end() {
        let w = walled_world();
        let short = clip_ray(&w, Vec3::new(0.5, 1.5, 5.5), Vec3::new(4.5, 1.5, 5.5));
        assert_eq!(short, None, "the wall beyond the segment never clips");
        let reaching = clip_ray(&w, Vec3::new(0.5, 1.5, 5.5), Vec3::new(5.5, 1.5, 5.5));
        let h = reaching.expect("the wall within the segment clips");
        assert!(
            (h.x - 5.0).abs() < 1e-6,
            "the hit rides the wall's own face, got x={}"
        , h.x);
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
        assert_eq!(calculate_fall_damage(3.0, m, SAFE_FALL_DISTANCE), 0);
        // the LAW's own negatives (the floor of a negative power — the
        // caller's `i > 0` gate refuses them, the hurt never speaks)
        assert_eq!(calculate_fall_damage(1.0, m, SAFE_FALL_DISTANCE), -2);
        assert_eq!(calculate_fall_damage(0.0, m, SAFE_FALL_DISTANCE), -3);
        // the half block floors away: 3.5 falls safe
        assert_eq!(calculate_fall_damage(3.5, m, SAFE_FALL_DISTANCE), 0);
        // every whole block past 3.0 speaks one
        assert_eq!(calculate_fall_damage(4.0, m, SAFE_FALL_DISTANCE), 1);
        assert_eq!(calculate_fall_damage(8.0, m, SAFE_FALL_DISTANCE), 5);
        assert_eq!(
            calculate_fall_damage(23.0, m, SAFE_FALL_DISTANCE),
            20,
            "the pure law's own kill count"
        );
        // the epsilon's own beat: 2.999999 falls safe, the 4.0 boundary
        // speaks one (the floor's own arithmetic)
        assert_eq!(calculate_fall_damage(2.999999, m, SAFE_FALL_DISTANCE), 0);
        assert_eq!(calculate_fall_damage(3.9999999, m, SAFE_FALL_DISTANCE), 1);
        assert_eq!(calculate_fall_damage(4.000001, m, SAFE_FALL_DISTANCE), 1);
        // THE JUMP BOOST'S OWN GRACE (the +1.0×(amp+1) ADD_VALUE
        // modifier the threshold lifts — the 4.0 fall the level-0
        // dose the SAFE fall wears)
        assert_eq!(
            calculate_fall_damage(4.0, m, SAFE_FALL_DISTANCE + 1.0),
            0,
            "the jump_boost I the 4-block fall the grace wears"
        );
        assert_eq!(
            calculate_fall_damage(6.0, m, SAFE_FALL_DISTANCE + 3.0),
            0,
            "the jump_boost III the 6-block fall the grace wears"
        );
        assert_eq!(
            calculate_fall_damage(7.0, m, SAFE_FALL_DISTANCE + 3.0),
            1,
            "the jump_boost III the 7th block the next damage speaks"
        );
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

    /// THE NOPHYSICS WALK (javap Entity.move pcs 0-58 — the early
    /// return the flag speaks): the same wall walk the raw position
    /// add wears — the body phases THROUGH the wall the collide body
    /// rests against, the collision flags the branch zeroes, the
    /// ground the raw walk never claims, and the pace the wall's
    /// inside never steals (the collide walk the return skips whole).
    #[test]
    fn the_nophysics_walk_phases_through_the_wall() {
        let w = walled_world();
        // THE COLLIDE CONTROL (the vanilla walk the wall stops — the
        // existing law's own shape)
        let mut b = PlayerBody::new(3.0, 1.0, 5.5, 270.0, 0.0);
        settle(&mut b, &w);
        b.forward = 1.0;
        for _ in 0..40 {
            b.tick(&w, false);
        }
        assert!(
            (b.x - (5.0 - WIDTH / 2.0)).abs() < 1e-9,
            "the face holds at {:.3}",
            b.x
        );
        assert!(b.horizontal_collision, "the wall spoke the flag");
        // THE NOPHYSICS BODY: the forced flight the spectator sync
        // speaks rides the flag — the raw walk phases through
        let mut p = PlayerBody::new(3.0, 1.0, 5.5, 270.0, 0.0);
        settle(&mut p, &w);
        p.flying = true; // the spectator sync's own force (the fly law)
        p.no_physics = true;
        p.forward = 1.0;
        for _ in 0..40 {
            p.tick(&w, false);
            assert!(!p.on_ground, "the ground never claims the raw body");
            assert!(!p.horizontal_collision, "the flag the branch zeroes");
            assert_eq!(
                p.fall_distance, 0.0,
                "the fall bank the skipped check never folds"
            );
        }
        assert!(p.x > 6.0, "the walk passed the wall at {:.3}", p.x);
        // the wall's own inside: the raw walk keeps its pace (the
        // collide walk the early return never answers)
        let x_mid = p.x;
        p.tick(&w, false);
        assert!(p.x > x_mid, "the raw walk keeps its pace");
    }

    /// THE NOPHYSICS FALL (the same audited branch): the flag's raw
    /// walk never lands — the floor the body phases beneath, the fall
    /// bank never folds (the check the early return skips), the
    /// collide control the ground claims.
    #[test]
    fn the_nophysics_fall_the_floor_never_claims() {
        let w = flat_world();
        // THE COLLIDE CONTROL: the falling body lands, the ground claims
        let mut b = PlayerBody::new(8.0, 6.0, 8.0, 0.0, 0.0);
        for _ in 0..100 {
            b.tick(&w, false);
            if b.on_ground {
                break;
            }
        }
        assert!(b.on_ground, "the collide body landed");
        assert!((b.y - 1.0).abs() < 0.01, "the feet rest on the floor");
        // THE NOPHYSICS BODY: the raw fall walks THROUGH the floor —
        // no landing, no bank, the descend the collide walk never sees
        let mut p = PlayerBody::new(8.0, 6.0, 8.0, 0.0, 0.0);
        p.no_physics = true;
        for _ in 0..100 {
            p.tick(&w, false);
            assert!(!p.on_ground, "the ground never claims the raw body");
            assert_eq!(p.fall_event, None, "the fall hurt never folds");
            assert_eq!(p.fall_distance, 0.0, "the bank the skip never feeds");
        }
        assert!(p.y < 0.0, "the body phased beneath the floor at {:.3}", p.y);
    }

    /// THE INSIDE-BLOCKS REFUSALS (javap isAffectedByBlocks —
    /// `!isRemoved() && !noPhysics`): the noPhysics body rides the
    /// lava cell without the ignite or the hurt, the water never
    /// douses the carried clock (the walk the flag keeps silent) —
    /// the collide controls the vanilla beats wear.
    #[test]
    fn the_nophysics_body_the_inside_walks_refuse() {
        // THE LAVA LANE: the collide control lights and hurts
        let mut w = flat_world();
        for x in 7..10 {
            for z in 7..10 {
                w.set(x, 1, z, crate::world::LAVA);
            }
        }
        let mut c = PlayerBody::new(8.0, 2.0, 8.0, 0.0, 0.0);
        for _ in 0..10 {
            c.tick(&w, false);
        }
        assert!(c.lava_hurt_due, "the inside walk hurt the control");
        assert!(c.remaining_fire_ticks > 0, "the brand rode the control");
        // THE NOPHYSICS BODY: the same cell the walk never speaks
        let mut p = PlayerBody::new(8.0, 2.0, 8.0, 0.0, 0.0);
        p.no_physics = true;
        for _ in 0..10 {
            p.tick(&w, false);
        }
        assert_eq!(p.remaining_fire_ticks, 0, "the ignite the walk refuses");
        assert!(!p.lava_hurt_due, "the hurt the walk refuses");
        // THE DOUSE LANE: the carried clock the water never kills
        let mut w2 = flat_world();
        for x in 7..10 {
            for z in 7..10 {
                w2.set(x, 1, z, WATER);
            }
        }
        let mut d = PlayerBody::new(8.0, 2.0, 8.0, 0.0, 0.0);
        d.no_physics = true;
        d.remaining_fire_ticks = 99; // the %20 cadence's silent lane
        d.tick(&w2, false);
        assert_eq!(
            d.remaining_fire_ticks, 98,
            "the baseTick clock keeps its decrement, the douse never kills"
        );
        assert!(!d.fire_doused_due, "the douse the walk refuses");
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

    /// THE FLY PHYSICS (the audited 26.2 flight laws the tick wears
    /// whole — the thrust the LocalPlayer.aiStep pcs 815-894 lane
    /// speaks, the hover the Player.travel pcs 121-163 restore keeps,
    /// the fall distance the Player.aiStep pcs 28-46 reset retires,
    /// the fly speeds the Player.getFlyingSpeed split carries, the
    /// fov the getFieldOfViewModifier's 1.1 fold speaks, and the hop
    /// the jumpFromGround fold rides).
    #[test]
    fn the_fly_physics_the_travel_restore_speaks() {
        let w = flat_world();
        // THE HOVER (the restore law): the flying body with no keys
        // never falls — the gravity the drag fold spends dies in the
        // `vy = pre * 0.6` overwrite, the carried rest stays rest
        let mut b = PlayerBody::new(0.5, 3.0, 0.5, 0.0, 0.0);
        b.flying = true;
        for i in 0..20 {
            b.tick(&w, false);
            assert!(b.y > 2.9, "the hover never falls (tick {i}) y={}", b.y);
            assert!(
                b.vy.abs() < 1e-9,
                "the hover decays to rest (tick {i}) vy={}",
                b.vy
            );
        }
        // THE ASCEND (the thrust lane): the space key folds +1, the
        // delta y gains `+1 * 0.05 * 3.0` = 0.15 a tick BEFORE the
        // move — the restore keeps the pre-travel vy * 0.6 — the
        // climb the two ticks speak
        let (y_before, vy_1) = (b.y, {
            b.jumping = true;
            b.tick(&w, false);
            b.vy
        });
        assert!(y_before > 2.9);
        assert!(
            (vy_1 - (FLY_SPEED as f64 * FLY_THRUST_SCALE * FLY_VY_RESTORE)).abs() < 1e-9,
            "the first tick's restore vy {vy_1}"
        );
        assert!(b.y > y_before, "the climb rides");
        // THE BOTH-KEYS NET (the ±1 fold's own cancel): the space and
        // the shift together the thrust never speaks — the fresh
        // hover the exact rest keeps
        let mut nb = PlayerBody::new(0.5, 3.0, 0.5, 0.0, 0.0);
        nb.flying = true;
        nb.jumping = true;
        nb.want_crouch = true;
        nb.tick(&w, false);
        assert!(nb.vy.abs() < 1e-9, "the net thrust never speaks vy={}", nb.vy);
        // THE DESCEND (the thrust lane's -1 arm): the shift key the
        // want_crouch snapshot carries — the sink the symmetric fold
        // speaks (the first thrust tick's restore: -0.15 * 0.6)
        nb.jumping = false;
        nb.tick(&w, false);
        assert!(nb.vy < 0.0, "the shift sinks, vy={}", nb.vy);
        // THE FALL DISTANCE (the reset law): the banked descent the
        // flight never keeps — the head reset retires the 5.0 bank
        // before the move, the only carry the tick's own descent the
        // move may re-accumulate (the vanilla order: Player.aiStep's
        // resetFallDistance precedes the super's accumulation walk)
        nb.fall_distance = 5.0;
        nb.tick(&w, false);
        assert!(
            nb.fall_distance < 1.0,
            "the banked descent the reset retires, got {}",
            nb.fall_distance
        );
        // THE FLY SPEED (Player.getFlyingSpeed's flying branch): the
        // airborne impulse 0.05 forward, the sprint-fly the x2 0.10
        // arm, the walk-air the 0.02 arm, the sprint-air the 0.026
        // arm — the four splits the yaw-zero fold speaks as the PRE-DRAG
        // position delta (the drag fold the tail speaks never touches
        // the move the impulse rode)
        let vz_of = |flying: bool, sprinting: bool| -> f64 {
            let mut bb = PlayerBody::new(0.5, 3.0, 0.5, 0.0, 0.0);
            bb.forward = 1.0;
            bb.flying = flying;
            bb.sprinting = sprinting;
            let z0 = bb.z;
            bb.tick(&w, false);
            bb.z - z0
        };
        assert!(
            (vz_of(true, false) - FLY_SPEED as f64).abs() < 1e-9,
            "the fly impulse"
        );
        assert!(
            (vz_of(true, true) - FLY_SPEED as f64 * 2.0).abs() < 1e-9,
            "the sprint-fly impulse"
        );
        assert!(
            (vz_of(false, false) - FLYING_SPEED as f64).abs() < 1e-9,
            "the walk-air impulse"
        );
        assert!(
            (vz_of(false, true) - SPRINT_AIR_SPEED as f64).abs() < 1e-9,
            "the sprint-air impulse"
        );
        // THE FOV (getFieldOfViewModifier's own head): the flying
        // modifier the 1.1 fold carries, the standing body the plain
        // 1.0 keeps
        let mut fb = PlayerBody::new(0.5, 3.0, 0.5, 0.0, 0.0);
        assert!((fb.fov_target(false) - 1.0).abs() < 1e-6, "the walk fov");
        fb.flying = true;
        assert!(
            (fb.fov_target(false) - FLY_FOV).abs() < 1e-6,
            "the fly fov rides the 1.1"
        );
        // THE LAUNCH HOP (jumpFromGround's own fold — the toggle-on's
        // ground arm the App's fly law calls): the vy the JUMP_POWER
        // floor raises, the plain jump's 0.05 exhaustion the
        // unsprinting body banks
        let mut hb = PlayerBody::new(0.5, 3.0, 0.5, 0.0, 0.0);
        hb.on_ground = true;
        hb.jump_from_ground();
        assert!((hb.vy - JUMP_POWER as f64).abs() < 1e-9, "the hop's vy");
        assert!(
            (hb.exhaustion_event - crate::food::EXHAUSTION_JUMP).abs() < 1e-6,
            "the hop's own cost"
        );
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

    /// THE SUNK PROBE THE BAND SLAB THE TABLE WEARS (autopush-423 —
    /// the 1/16 window the full-cell overlap kept the band's own top
    /// retires): the sunk box's bottom risen past the COLUMN row's
    /// 15/16 band the shape-aware probe the honest no-floor verdict
    /// speaks — the full cube's own [cy, cy+1] the old walk counted
    /// the vanilla noCollision's collision-shape law never asks. The
    /// SAME geometry the full-cube cell the floor verdict keeps (the
    /// refusal the BAND's own, never the geometry's), the band
    /// STANDING body's own support the probe still counts (the sunk
    /// bottom well inside the band the floor speaks), the DIRT PATH
    /// row the same law wears.
    #[test]
    fn the_sunk_probe_the_band_slab_the_table_wears() {
        // the farmland the y=0 layer holds (the stone the farm's own
        // support wears), the air above
        let mut w = flat_world();
        w.set(4, 0, 4, crate::world::FARMLAND);
        assert!(matches!(
            crate::world::collision_shape(w.get(4, 0, 4)),
            crate::world::CollisionShape::Column(15)
        ));
        // THE WINDOW: the body's feet in (cy + 1.5375, cy + 1.6) —
        // the sunk bottom 0.95 past the band's 0.9375 top, under the
        // full cube's 1.0. The shape-aware probe the NO-floor verdict
        // speaks (the band top the sunk bottom refuses); the full-cell
        // walk the old verdict kept would count the cube.
        let b = PlayerBody::new(4.5, 1.55, 4.5, 0.0, 0.0);
        assert!(
            b.no_floor_below(&w, 0.0, 0.0),
            "the band top the sunk bottom 0.95 refuses — no floor"
        );
        // THE FULL CUBE'S OWN VERDICT the same geometry keeps: the
        // stone the band's place wears, the sunk bottom 0.95 the cube's
        // [0, 1] the floor speaks — the refusal the BAND's own.
        let w_full = flat_world();
        let b_full = PlayerBody::new(4.5, 1.55, 4.5, 0.0, 0.0);
        assert!(
            !b_full.no_floor_below(&w_full, 0.0, 0.0),
            "the full cube the same sunk bottom the floor keeps"
        );
        // THE BAND-STANDING BODY'S OWN SUPPORT the probe counts: the
        // feet on the band top 0.9375, the sunk bottom 0.3375 well
        // inside the band — the floor the sneak march along the field
        // keeps.
        let b_band = PlayerBody::new(4.5, 0.9375, 4.5, 0.0, 0.0);
        assert!(
            !b_band.no_floor_below(&w, 0.05, 0.0),
            "the band the sunk bottom 0.3375 the floor speaks"
        );
        // THE DIRT PATH the same law wears.
        let mut w_path = flat_world();
        w_path.set(4, 0, 4, crate::world::DIRT_PATH);
        let b_path = PlayerBody::new(4.5, 1.55, 4.5, 0.0, 0.0);
        assert!(
            b_path.no_floor_below(&w_path, 0.0, 0.0),
            "the path band the same window refuses"
        );
    }

    /// THE SNEAK MARCH THE FIELD THE BAND FLOOR KEEPS (autopush-423 —
    /// the probe's shape vocabulary the march's own lane keeps): the
    /// band-standing body sneaking ALONG the farmland field the guard
    /// never bothers — the sunk box's bottom well inside the next
    /// cell's band the floor verdict speaks, the march crosses the
    /// cell boundary the full-cell walk always wore (the regression
    /// guard the unification never breaks).
    #[test]
    fn the_sneak_march_the_field_the_band_floor_keeps() {
        // the field: the farmland the y=1 layer holds (the stone the
        // support below wears), the band tops at 1.9375
        let mut w = flat_world();
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 1, z, crate::world::FARMLAND);
            }
        }
        let mut b = PlayerBody::new(2.5, 1.9375, 8.0, 270.0, 0.0); // facing +x
        settle(&mut b, &w);
        assert!(b.on_ground, "the band the settle's ground keeps");
        assert!((b.y - 1.9375).abs() < 1e-9, "the band top the feet wear");
        b.want_crouch = true;
        b.forward = 1.0;
        for _ in 0..12 {
            b.tick(&w, false);
        }
        // the march crossed the cell boundary 3.0 — the band the
        // sunk box's own floor the guard never zeroed
        assert!(b.x > 3.0, "the field march crossed: x={}", b.x);
        assert!((b.y - 1.9375).abs() < 1e-9, "the band the feet never leave");
        assert!(b.on_ground, "the march the ground keeps");
    }

    /// THE SNEAK NEVER WALKS OFF THE FIELD EDGE (autopush-423 — the
    /// guard's shape-aware probe the band edge wears): the body
    /// sneaking toward the farmland field's edge the guard backs off —
    /// the stone one below the band-standing body's sunk bottom
    /// refuses (the 15/16 drop the probe's own verdict speaks), the
    /// band cells the march's floor the edge never crosses. The feet
    /// stay on the field, the body never falls, the y the band top
    /// keeps.
    #[test]
    fn the_sneak_never_walks_off_the_field_edge() {
        // the field ends at x=8: the farmland the y=1 layer where
        // x < 8, the bare stone the ground beyond (the band top's own
        // 15/16 drop the edge speaks)
        let mut w = flat_world();
        for x in 0..8 {
            for z in 0..16 {
                w.set(x, 1, z, crate::world::FARMLAND);
            }
        }
        let mut b = PlayerBody::new(7.0, 1.9375, 8.0, 270.0, 0.0); // facing the edge
        settle(&mut b, &w);
        assert!(b.on_ground);
        b.want_crouch = true;
        b.forward = 1.0;
        for _ in 0..60 {
            b.tick(&w, false);
        }
        // the body may hang its box over the edge but never LEAVE the
        // field (the vanilla law the ledge test speaks — the FEET stay
        // on the cell: x - 0.3 < 8)
        assert!(b.x - WIDTH / 2.0 < 8.0, "the feet x {} stayed on the field", b.x);
        assert!((b.y - 1.9375).abs() < 1e-9, "the band top the feet keep: y={}", b.y);
        assert!(b.on_ground, "never fell");
    }

    /// THE TORCH'S OWN WALK-THROUGH (the dev-32 round — the
    /// noCollision row the BaseTorchBlock Properties chain speaks,
    /// the solid() verdict the passable set keeps): the torch cell
    /// the body marches straight through — the same march the stone
    /// wall the face stops (the contrast the noCollision law keeps).
    #[test]
    fn the_torch_never_blocks_the_body() {
        // the torch standing on the floor the walk path holds
        let mut w = flat_world();
        w.set(5, 1, 5, crate::world::TORCH);
        assert!(!solid(&w, 5, 1, 5), "the torch the walk-through verdict keeps");
        let mut b = PlayerBody::new(2.5, 1.0, 5.5, 270.0, 0.0); // facing +x
        settle(&mut b, &w);
        b.forward = 1.0;
        for _ in 0..40 {
            b.tick(&w, false);
        }
        assert!(b.x > 5.5, "the torch cell the body walked through: x={}", b.x);
        // THE STONE'S OWN CLIP — the same march the solid wall stops
        // dead at the face (the law the torch's noCollision contrasts)
        let mut w2 = flat_world();
        w2.set(5, 1, 5, crate::world::STONE);
        w2.set(5, 2, 5, crate::world::STONE);
        assert!(solid(&w2, 5, 1, 5));
        let mut b2 = PlayerBody::new(2.5, 1.0, 5.5, 270.0, 0.0);
        settle(&mut b2, &w2);
        b2.forward = 1.0;
        for _ in 0..40 {
            b2.tick(&w2, false);
        }
        assert!(b2.x < 5.0, "the stone wall the march stops at: x={}", b2.x);
    }

    /// THE PREDICATE THE TABLE ITSELF ANSWERS (autopush-420 — the
    /// single-source law: the solid walk consumes the shape table the
    /// placement's `isUnobstructed` walk already wears, the two walks
    /// the divergence never keeps again). Every EMPTY row the
    /// walk-through verdict keeps — the LEAF LITTER row the
    /// unification GAINS (the ground cover the old hand-list SOLID
    /// kept, the drift the table retires) — and the COLUMN and FULL
    /// rows the solid verdict keeps.
    #[test]
    fn the_solid_predicate_wears_the_shape_table() {
        let mut w = flat_world();
        // THE EMPTY ROWS: the whole noCollision family the table
        // banks — the probe rides the cell (1,1,1), each kind the
        // verdict wears
        for id in [
            crate::world::WATER,
            crate::world::LAVA,
            crate::world::TORCH,
            crate::world::DANDELION,
            crate::world::POPPY,
            crate::world::AZURE_BLUET,
            crate::world::OXEYE_DAISY,
            crate::world::CORNFLOWER,
            crate::world::ORANGE_TULIP,
            crate::world::RED_TULIP,
            crate::world::PINK_TULIP,
            crate::world::WHITE_TULIP,
            crate::world::SHORT_GRASS,
            crate::world::BUSH,
            crate::world::TALL_GRASS,
            crate::world::TALL_GRASS_UPPER,
            crate::world::LARGE_FERN,
            crate::world::LARGE_FERN_UPPER,
            crate::world::LEAF_LITTER,
            crate::world::WHEAT,
            crate::world::WHEAT_STAGE7,
        ] {
            w.set(1, 1, 1, id);
            assert!(
                !solid(&w, 1, 1, 1),
                "the empty row {} the walk-through keeps",
                crate::world::NAME_OF[id as usize]
            );
        }
        // THE COLUMN AND FULL ROWS: the solid verdict the collision
        // keeps (the farmland/dirt path the 15/16 band the full-cell
        // sweep absorbs, the roster the default registration wears)
        for id in [
            crate::world::FARMLAND,
            crate::world::DIRT_PATH,
            crate::world::STONE,
            crate::world::GRASS,
            crate::world::LOG,
            crate::world::CRAFTING_TABLE,
        ] {
            w.set(1, 1, 1, id);
            assert!(
                solid(&w, 1, 1, 1),
                "the solid row {} keeps",
                crate::world::NAME_OF[id as usize]
            );
        }
    }

    /// THE GROUND COVER'S OWN WALK-THROUGH (autopush-420 — the leaf
    /// litter row the shape-table unification gains): the litter cell
    /// the body marches straight through AND the fall the floor's own
    /// top wears — the cover never lifts the feet (the vanilla
    /// `.noCollision()` row the table banks), the stone the march the
    /// face stops (the contrast the collision law keeps).
    #[test]
    fn the_leaf_litter_never_blocks_the_body() {
        // THE MARCH: the litter ON the floor the walk path holds, the
        // body the cover cell straight through (the torch's own march
        // the same lane speaks)
        let mut w = flat_world();
        w.set(5, 1, 5, crate::world::LEAF_LITTER);
        assert!(!solid(&w, 5, 1, 5), "the litter the walk-through verdict keeps");
        let mut b = PlayerBody::new(2.5, 1.0, 5.5, 270.0, 0.0); // facing +x
        settle(&mut b, &w);
        b.forward = 1.0;
        for _ in 0..40 {
            b.tick(&w, false);
        }
        assert!(b.x > 5.5, "the litter cell the body walked through: x={}", b.x);
        // THE FALL'S OWN LANDING: the body dropped onto the litter
        // cell the FLOOR'S own top wears (y=1.0) — the cover the feet
        // never lifts, the y=2.0 landing the old full-cube verdict
        // spoke
        let mut w2 = flat_world();
        w2.set(5, 1, 5, crate::world::LEAF_LITTER);
        let mut b2 = PlayerBody::new(5.5, 5.0, 5.5, 0.0, 0.0);
        for _ in 0..60 {
            b2.tick(&w2, false);
        }
        assert!(
            (b2.y - 1.0).abs() < 1e-9,
            "the fall the floor's top wears: y={}",
            b2.y
        );
    }

    /// THE SWEEP'S OWN BAND VERDICTS (autopush-421 — the unit law the
    /// ladder stands on): the fall onto the farmland cell the 15/16
    /// floor clips (the full-cube y=2.0 verdict the band retires), the
    /// SAME box's march the band cell frees (the strict overlap the
    /// band's own top speaks) and the full cube the face stops (the
    /// touching contact the zero allowed wears).
    #[test]
    fn the_sweep_the_band_verdicts_speak() {
        let mut w = flat_world();
        w.set(5, 1, 5, crate::world::FARMLAND);
        // THE FALL: the box above the farm cell, the downward move the
        // band top clips (the stone floor's own top the deeper gap
        // never reaches)
        let b = (5.1, 3.0, 5.1, 5.9, 4.8, 5.9);
        let c = collide(&w, b, (0.0, -2.0, 0.0));
        assert!(
            (c.dy - (1.0 + 15.0 / 16.0 - 3.0)).abs() < 1e-9,
            "the fall the band clips: dy={}",
            c.dy
        );
        assert!(c.vertical_collision_below, "the band the ground flag arms");
        // THE MARCH'S OWN PAIR — the same box, the same move: the band
        // cell the strict overlap frees, the full cube the face stops
        let walker = (6.0, 1.0 + 15.0 / 16.0, 5.1, 7.0, 1.0 + 15.0 / 16.0 + 1.8, 5.9);
        w.set(7, 1, 5, crate::world::FARMLAND);
        let free = collide(&w, walker, (0.3, 0.0, 0.0));
        assert!(
            (free.dx - 0.3).abs() < 1e-9,
            "the band-level march the neighbor band frees: dx={}",
            free.dx
        );
        w.set(7, 1, 5, crate::world::STONE);
        let stopped = collide(&w, walker, (0.3, 0.0, 0.0));
        assert!(
            stopped.dx.abs() < 1e-9,
            "the full cube the band-level march stops: dx={}",
            stopped.dx
        );
    }

    /// THE COLUMN BAND'S OWN LANDING (autopush-421 — the 15/16 band the
    /// sweep's Y extent speaks): the body dropped onto the farmland
    /// cell the BAND'S own top wears (y = 1 + 15/16) — the full-cube
    /// y=2.0 landing the old verdict spoke the band retires; the dirt
    /// path the same verdict wears; the stone the y=2.0 keeps (the
    /// contrast the full-cube roster speaks).
    #[test]
    fn the_column_band_the_landing_wears() {
        // THE FARMLAND: the band the landing speaks
        let mut w = flat_world();
        w.set(5, 1, 5, crate::world::FARMLAND);
        let mut b = PlayerBody::new(5.5, 4.5, 5.5, 0.0, 0.0);
        for _ in 0..60 {
            b.tick(&w, false);
        }
        assert!(
            (b.y - (1.0 + 15.0 / 16.0)).abs() < 1e-9,
            "the farmland the band's own top wears: y={}",
            b.y
        );
        assert!(b.on_ground, "the band the ground verdict arms");
        // THE DIRT PATH the same band wears
        let mut w2 = flat_world();
        w2.set(5, 1, 5, crate::world::DIRT_PATH);
        let mut b2 = PlayerBody::new(5.5, 4.5, 5.5, 0.0, 0.0);
        for _ in 0..60 {
            b2.tick(&w2, false);
        }
        assert!(
            (b2.y - (1.0 + 15.0 / 16.0)).abs() < 1e-9,
            "the path the band wears: y={}",
            b2.y
        );
        // THE STONE the full-cube top keeps (the contrast)
        let mut w3 = flat_world();
        w3.set(5, 1, 5, crate::world::STONE);
        let mut b3 = PlayerBody::new(5.5, 4.5, 5.5, 0.0, 0.0);
        for _ in 0..60 {
            b3.tick(&w3, false);
        }
        assert!(
            (b3.y - 2.0).abs() < 1e-9,
            "the stone the full top wears: y={}",
            b3.y
        );
    }

    /// THE FIELD EDGE THE STEP CLIMBS (autopush-421 — the ladder the
    /// 26.2 collide's own candidate heights speak): the body ON the
    /// farmland band marching into the dirt shelf the 1/16 rise the
    /// step climbs (the feet the shelf's own top wears, the march the
    /// face never stops, the jump never asked); the shelf the ground
    /// verdict keeps (the step's net rise the honest flag arms).
    #[test]
    fn the_field_edge_the_step_climbs() {
        // the farmland bed [3..5) the dirt shelf [5..14) speaks: the
        // body starts ON the band (feet 1.9375), marches +x into the
        // shelf whose top is the feet's own 1/16 rise
        let mut w = flat_world();
        for x in 3..5 {
            w.set(x, 1, 5, crate::world::FARMLAND);
        }
        for x in 5..14 {
            w.set(x, 1, 5, crate::world::DIRT);
        }
        let mut b = PlayerBody::new(3.5, 1.0 + 15.0 / 16.0, 5.5, 270.0, 0.0); // facing +x
        settle(&mut b, &w);
        assert!(
            (b.y - (1.0 + 15.0 / 16.0)).abs() < 1e-9,
            "the band the stance keeps: y={}",
            b.y
        );
        b.forward = 1.0;
        for _ in 0..40 {
            b.tick(&w, false);
        }
        assert!(b.x > 6.0, "the field edge the march crossed: x={}", b.x);
        assert!(
            (b.y - 2.0).abs() < 1e-9,
            "the shelf top the step wears: y={}",
            b.y
        );
        assert!(b.on_ground, "the shelf the ground verdict keeps");
    }

    /// THE FIELD MARCH THE BAND NEVER STICKS (autopush-421 — the
    /// same-level walk the band's strict overlap frees): the body ON
    /// the farm field marching across the cell boundaries the neighbors
    /// never block (the feet the band's own top the c1hi the strict
    /// skin reads) — the farm the walkable field the vanilla speaks,
    /// the feet the ONE band height the whole march wears.
    #[test]
    fn the_field_march_the_band_never_sticks() {
        let mut w = flat_world();
        for x in 3..14 {
            w.set(x, 1, 5, crate::world::FARMLAND);
        }
        let mut b = PlayerBody::new(3.5, 1.0 + 15.0 / 16.0, 5.5, 270.0, 0.0); // facing +x
        settle(&mut b, &w);
        b.forward = 1.0;
        for _ in 0..40 {
            b.tick(&w, false);
        }
        assert!(
            b.x > 6.0,
            "the field the march crossed the boundaries: x={}",
            b.x
        );
        assert!(
            (b.y - (1.0 + 15.0 / 16.0)).abs() < 1e-9,
            "the band the feet the whole march wears: y={}",
            b.y
        );
    }

    /// THE LADDER THE FULL WALL NEVER CLIMBS (autopush-421 — the
    /// contrast the maxUpStep gate keeps): the 1.0 cell's top the `>
    /// maxUpStep` gate refuses (no candidate the ladder arms) — the
    /// march the face stops, the feet the floor keeps, the jump the
    /// wall's own answer the field's 1/16 climb contrasts.
    #[test]
    fn the_ladder_the_full_wall_never_climbs() {
        let mut w = flat_world();
        w.set(5, 1, 5, crate::world::STONE);
        let mut b = PlayerBody::new(2.5, 1.0, 5.5, 270.0, 0.0); // facing +x
        settle(&mut b, &w);
        b.forward = 1.0;
        for _ in 0..40 {
            b.tick(&w, false);
        }
        assert!(b.x < 5.0, "the wall the march stops at: x={}", b.x);
        assert!(
            (b.y - 1.0).abs() < 1e-9,
            "the feet the floor the wall keeps: y={}",
            b.y
        );
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

    /// THE EFFECT ATTRIBUTES' OWN MOTION (javap MobEffects clinit the
    /// ±0.2/−0.15 ADD_MULTIPLIED_TOTAL rows + AttributeTemplate.create
    /// the amount×(amp+1) law + AttributeInstance.calculateValue the
    /// ×(1+amount) fold + getJumpBoostPower the 0.1×(amp+1) read):
    /// the speed() the product folds, the FOV's own ratio the sprint
    /// zoom the effect doses widens, the jump floor the power lifts,
    /// the WALK the real displacement the multiplier scales.
    #[test]
    fn the_effect_attributes_the_motion_the_fov_wears() {
        // THE ATTRIBUTE VALUE (the speed() the base×mult chain folds —
        // the sprint/sneak/use multipliers the same chain rides)
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        b.effect_speed_mult = 1.0;
        assert_eq!(b.speed(false), MOVEMENT_SPEED);
        b.effect_speed_mult = 1.2;
        assert_eq!(b.speed(false), MOVEMENT_SPEED * 1.2);
        b.sprinting = true;
        assert_eq!(b.speed(false), MOVEMENT_SPEED * 1.2 * SPRINT_MULTIPLIER);
        // THE PAIR (speed I × slowness III — 1.4 × 0.4 the two
        // ×(1+amount) steps the calculate law keeps)
        let pair = (1.0 + SPEED_EFFECT_MOVEMENT_MODIFIER * 2.0)
            * (1.0 + SLOWNESS_EFFECT_MOVEMENT_MODIFIER * 4.0);
        b.effect_speed_mult = pair;
        b.sprinting = false;
        assert!(
            (b.speed(false) - MOVEMENT_SPEED * pair).abs() < 1e-9,
            "the pair fold speaks the base×product"
        );
        // THE SANITIZE FLOOR (the extreme slowness the ZERO speed
        // wears — the value the negative never walks)
        b.effect_speed_mult = 0.0;
        assert_eq!(b.speed(false), 0.0);
        // THE FOV RATIO (getFieldOfViewModifier's (speed/base + 1)/2 —
        // the speed I dose the 1.2 ratio the 1.1 zoom answers, the
        // slowness the 0.85 the 0.925 wears)
        b.effect_speed_mult = 1.2;
        b.sprinting = false;
        assert!((b.fov_target(false) - 1.1).abs() < 1e-6);
        b.effect_speed_mult = 0.85;
        assert!((b.fov_target(false) - 0.925).abs() < 1e-6);
        // THE JUMP FLOOR (getJumpPower's attribute + the boost power —
        // the plain 0.42, the level-0 dose 0.52, the amp-2 dose 0.72)
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        b.jump_from_ground();
        assert_eq!(b.vy, JUMP_POWER as f64);
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        b.jump_boost_power = JUMP_BOOST_POWER_PER_LEVEL;
        b.jump_from_ground();
        assert_eq!(b.vy, (JUMP_POWER + JUMP_BOOST_POWER_PER_LEVEL) as f64);
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        b.jump_boost_power = JUMP_BOOST_POWER_PER_LEVEL * 3.0;
        b.jump_from_ground();
        assert_eq!(b.vy, (JUMP_POWER + JUMP_BOOST_POWER_PER_LEVEL * 3.0) as f64);
        // THE REAL MOTION (the flat walk the displacement the
        // multiplier scales — the drag the same linear walk the
        // terminal speed the input speed the proportional law keeps)
        let w = flat_world();
        let walk = |mult: f32| {
            let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
            b.effect_speed_mult = mult;
            b.forward = 1.0;
            for _ in 0..5 {
                b.tick(&w, false); // the settle
            }
            b.forward = 1.0;
            b.effect_speed_mult = mult;
            let (x0, z0) = (b.x, b.z);
            for _ in 0..40 {
                b.tick(&w, false);
                b.forward = 1.0;
                b.effect_speed_mult = mult;
            }
            ((b.x - x0) * (b.x - x0) + (b.z - z0) * (b.z - z0)).sqrt()
        };
        let base = walk(1.0);
        let boosted = walk(1.2);
        assert!(
            ((boosted / base) - 1.2).abs() < 0.02,
            "the 1.2x dose the displacement scales: {base} -> {boosted}"
        );
        let slowed = walk(0.85);
        assert!(
            ((slowed / base) - 0.85).abs() < 0.02,
            "the 0.85x dose the displacement scales: {base} -> {slowed}"
        );
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

    /// THE WATER LANE'S OWN WALK (javap `LivingEntity.travelInWater`
    /// the 26.2 jar — pcs 0-186 the FULL walk the fluid route keeps):
    /// the sink the ×0.8 + the buoyancy fold wears (the terminal
    /// −0.025 the old air stand-in never spoke), the Dolphins Grace
    /// body the 0.96 glide wears (the swept ladder the two drags
    /// speak), the sprint body the buoyancy fold SKIPS (the ×0.8
    /// alone the vy wears).
    #[test]
    fn the_water_lane_the_sink_and_the_glide_speak() {
        // THE DEEP POOL (the 64-wide world the glide's runway keeps)
        let mut w = World::new(4, 42);
        for x in 0..64 {
            for z in 0..64 {
                w.set(x, 0, z, crate::world::STONE);
                for y in 1..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, WATER);
                }
            }
        }
        // THE SINK (the falling-adjusted fold's own terminal — the
        // slow descent the ×0.8 decay keeps, the mid-water verdict
        // the never-grounded body wears)
        let mut b = PlayerBody::new(8.0, 2.5, 8.0, 0.0, 0.0);
        b.vy = 0.0;
        for _ in 0..40 {
            b.tick(&w, false);
        }
        assert!(
            b.vy > -0.05 && b.vy < 0.0,
            "the slow sink the buoyancy fold keeps ({})",
            b.vy
        );
        // THE GLIDE LADDER (the Dolphins Grace body the 0.96 drag
        // wears — the SAME impulse the two drags the swept distance
        // speaks)
        let mut plain = PlayerBody::new(8.0, 2.5, 8.0, 0.0, 0.0);
        plain.vy = 0.0;
        plain.forward = 1.0;
        let mut dg = PlayerBody::new(8.0, 2.5, 8.0, 0.0, 0.0);
        dg.vy = 0.0;
        dg.forward = 1.0;
        dg.dolphins_grace = true;
        for _ in 0..60 {
            plain.tick(&w, false);
            dg.tick(&w, false);
        }
        let swept_p = ((plain.x - 8.0) * (plain.x - 8.0) + (plain.z - 8.0) * (plain.z - 8.0))
            .sqrt();
        let swept_d = ((dg.x - 8.0) * (dg.x - 8.0) + (dg.z - 8.0) * (dg.z - 8.0)).sqrt();
        assert!(
            swept_d > swept_p * 2.0,
            "the glide the 0.96 drag speaks ({} vs {})",
            swept_d,
            swept_p
        );
        // THE SPRINT SKIP (getFluidFallingAdjustedMovement pc 7 — the
        // isSprinting body the buoyancy fold never pays, the ×0.8
        // alone the vy wears)
        let mut s = PlayerBody::new(8.0, 2.5, 8.0, 0.0, 0.0);
        s.vy = -0.1;
        s.sprinting = true;
        s.tick(&w, false);
        assert!(
            (s.vy - (-0.1 * (0.8f32 as f64))).abs() < 1e-12,
            "the sprint body the fold skips ({})",
            s.vy
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
                // THE RINGS' OWN POSE (the loops' own read — the live
                // position + the carried velocity at the call, the
                // same moment the volume's fold speaks)
                let pose = b.take_splash_pose();
                assert!(
                    pose.is_some(),
                    "the entry carries the rings' own pose"
                );
                let (px, py, pz, pvx, pvy, pvz) = pose.unwrap();
                assert_eq!((px, pz), (8.0, 8.0), "the pose x/z");
                assert_eq!((pvx, pvy, pvz), (prev_vx, prev_vy, prev_vz));
                // the pose's y rode the capture moment (the fluid
                // lane's own read — before the tick's move walked the
                // body deeper)
                assert!(py.is_finite(), "the pose y rode the capture");
                // the drain takes once — the next tick never replays
                assert_eq!(b.take_splash_pose(), None, "the drain takes once");
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

    /// THE GLIDE'S OWN FOLD (javap
    /// `updateFallFlyingMovement` the 26.2 walk — the level look's
    /// one-tick walk the exact f64 values pin: A) the gravity fold
    /// `vy += 0.08 * (-1.0 + 0.75)`, B) the dive conversion `t =
    /// vy * -0.1` (the Y add the `dload 12` ALONE speaks, the +Z
    /// look the Z thrust answers), D) the alignment (the horizontal
    /// speed steers toward the look at the ENTRY magnitude's
    /// direction, the 0.1 gain), E) the drag `(0.99, 0.98, 0.99)`
    /// the f32-promoted constants keep).
    #[test]
    fn the_glide_folds_the_law_wears() {
        let mut b = PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0);
        // the look +Z (yaw 0), level (pitch 0): lx = 0, lz = 1, cos²=1
        b.vx = 1.0;
        b.vy = -0.3;
        b.vz = 0.0;
        b.glide_folds();
        // A+B: vy = -0.3 + 0.08*(-0.25) = -0.32, then t = 0.032:
        // vy = -0.288, vz = 0.032, vx untouched (the look's X is zero)
        // D: vx += (0*1 - 1)*0.1 = -0.1 -> 0.9; vz += (1*1 - 0.032)*0.1
        // E: the f32-promoted drag multiplies
        assert_eq!(b.vx, 0.8910000085830688, "the X the alignment+drag wear");
        assert_eq!(b.vy, -0.2822400054931641, "the Y the gravity+dive+drag wear");
        assert_eq!(b.vz, 0.1275120012283325, "the Z the dive+align+drag wear");
    }

    /// THE GLIDE CHORD'S OWN START (javap `Player.tryToStartFallFlying`
    /// + `LocalPlayer.aiStep` pcs 644-694 the 26.2 walk): the jump key
    /// the HELD state reads, the airborne body the canGlide's own
    /// !onGround arm requires, the chest verdict the equipment
    /// answers — the elytra-less body never starts, the creative
    /// flight the abilities arm refuses, the grounded body the
    /// verdict refuses.
    #[test]
    fn the_glide_chord_the_flag_wears() {
        let mut w = World::new(1, 42);
        for x in 0..8 {
            for z in 0..8 {
                for y in 0..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, if y == 0 { crate::world::STONE } else { AIR });
                }
            }
        }
        // THE ELYTRA CHORD: the body walks off the ledge mid-air with
        // the chest wearing and jump held — the flag the verdict
        // speaks (and the wire echo banks)
        let mut b = PlayerBody::new(3.0, 40.0, 3.0, 0.0, 0.0);
        b.chest_glides = true;
        b.jumping = true;
        b.tick(&w, false);
        assert!(b.fall_flying, "the airborne chord the glide starts");
        assert!(b.glide_started_due, "the start's own wire echo banks");
        b.glide_started_due = false;
        assert!(!b.glide_started_due, "the echo the next tick never re-speaks");
        // THE BARE CHEST: the same chord without the elytra the
        // canGlideUsing arm refuses
        let mut b = PlayerBody::new(3.0, 40.0, 3.0, 0.0, 0.0);
        b.jumping = true;
        b.tick(&w, false);
        assert!(!b.fall_flying, "the elytra-less body never glides");
        // THE CREATIVE FLIGHT: the abilities.flying arm the Player's
        // own override refuses (the same held jump mid-air)
        let mut b = PlayerBody::new(3.0, 40.0, 3.0, 0.0, 0.0);
        b.chest_glides = true;
        b.jumping = true;
        b.flying = true;
        b.tick(&w, false);
        assert!(!b.fall_flying, "the creative flight the glide never starts");
        // THE GROUNDED BODY: the jump key UP — the start walk's own
        // held-key gate refuses (the canGlide's !onGround arm reads
        // only while the chord rides)
        let mut b = PlayerBody::new(3.0, 1.0, 3.0, 0.0, 0.0);
        b.chest_glides = true;
        settle(&mut b, &w);
        assert!(!b.fall_flying, "the grounded body never glides");
    }

    /// THE FALL-FLYING WEAR'S OWN CLOCK (javap `LivingEntity.tick` the
    /// 26.2 walk): the gliding body's wear walk — the damage banks at
    /// the i%20 gates (20, 40 — the (i/10)%2 == 0 arm speaks), the
    /// gameEvent due at EVERY i%10 gate (10, 20, 30, 40), the stopped
    /// tick the pc 598-620 reset arm zeroes (the fresh glide the count
    /// restarts — the second ride the 1..20 walk banks, the event
    /// rhythm the same gates lands), the last point's refusal the
    /// canGlideUsing arm speaks (the 431-damage body the stop arm
    /// retires, the wear the retire tick never rides, the break the
    /// glide's own reach never triggers — the isBroken arm the direct
    /// hurt call answers).
    #[test]
    fn the_elytra_wear_the_clock_keeps() {
        let mut w = World::new(1, 42);
        for x in 0..8 {
            for z in 0..8 {
                for y in 0..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, if y == 0 { crate::world::STONE } else { AIR });
                }
            }
        }
        let mut b = PlayerBody::new(2.0, 64.0, 2.0, 0.0, 0.0);
        b.fall_flying = true;
        b.chest_glides = true;
        let mut event_ticks = Vec::new();
        for t in 1..=40 {
            b.tick(&w, false);
            assert!(b.fall_flying, "the airborne glide never retires (tick {t})");
            if b.elytra_glide_event_due {
                b.elytra_glide_event_due = false;
                event_ticks.push(t);
            }
            // the stop at the halfway point: the counter the restart
            // never resets (the second glide rides the same clock)
            if t == 20 {
                b.fall_flying = false;
                b.tick(&w, false);
                assert!(!b.elytra_glide_event_due, "the stopped tick the wear never rides");
                b.fall_flying = true;
            }
        }
        // the event rhythm: EVERY i%10 gate speaks (the stopped tick
        // the reset arm zeroes — the fresh glide the count restarts,
        // the second ride's own 10th tick the t=30 gate lands — the
        // gates at 10, 20, 30, 40 all keep)
        assert_eq!(event_ticks, vec![10, 20, 30, 40], "the every-10 event rhythm");
        // the damage rhythm: the (i/10)%2 == 0 arm — the gates at 20
        // and 40, NOT at 10 or 30 (the fresh ride's i=20 the t=40
        // gate wears)
        assert_eq!(
            b.fall_fly_ticks, 20,
            "the stopped tick the reset arm zeroes — the second glide the fresh clock rides"
        );
        assert_eq!(b.elytra_damage, 2, "the every-20 wear: two points the 40 ticks speak");
    }

    /// THE GLIDE CLOCK'S OWN RESET ARM (javap `LivingEntity.tick` the
    /// 26.2 walk — the pc 598-620 field tail the glide's own gate
    /// keeps: `isFallFlying() ? ++fallFlyTicks : fallFlyTicks = 0`,
    /// the grounded tick the zero the fresh glide the count restarts,
    /// the fresh ride's own 10th gate the event speaks, the stale
    /// cumulative count the hurt's j=2 answer never rides).
    #[test]
    fn the_glide_clock_the_reset_arm_zeroes() {
        let mut w = World::new(1, 42);
        for x in 0..8 {
            for z in 0..8 {
                for y in 0..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, if y == 0 { crate::world::STONE } else { AIR });
                }
            }
        }
        // THE FIRST RIDE: 9 gliding ticks — the clock the 9 count
        // banks, the every-10 gate the silence keeps
        let mut b = PlayerBody::new(2.0, 64.0, 2.0, 0.0, 0.0);
        b.fall_flying = true;
        b.chest_glides = true;
        for _ in 0..9 {
            b.tick(&w, false);
            assert!(b.fall_flying, "the airborne glide never retires");
        }
        assert_eq!(b.fall_fly_ticks, 9, "the first ride the 9 count banks");
        assert!(!b.elytra_glide_event_due, "the 9 count the gate never speaks");
        // THE GROUNDED TICK: the flag the drop answers — the reset arm
        // the zero speaks (the old walk's cumulative 20 the fresh
        // clock refuses — the every-10 gate the stale field never
        // rides), the grounded tick the wear walk never enters
        b.fall_flying = false;
        b.tick(&w, false);
        assert_eq!(b.fall_fly_ticks, 0, "the grounded tick the arm zeroes");
        assert!(!b.elytra_glide_event_due, "the grounded tick no event speaks");
        // THE FRESH RIDE: 10 gliding ticks — the event the FRESH
        // clock's own 10th gate speaks (the cumulative count the old
        // walk rode the reset retires), the hurt gate the fresh j=1
        // refuses (the (i/10)%2 arm the odd answer keeps — the old
        // walk's i=20 j=2 hurt the zeroed clock never wears)
        let mut event_tick = 0;
        for t in 1..=10 {
            b.fall_flying = true;
            b.tick(&w, false);
            assert!(b.fall_flying, "the airborne glide never retires (tick {t})");
            if b.elytra_glide_event_due {
                b.elytra_glide_event_due = false;
                event_tick = t;
            }
        }
        assert_eq!(event_tick, 10, "the fresh ride the own 10th gate speaks");
        assert_eq!(b.fall_fly_ticks, 10, "the fresh clock the 10 count keeps");
        assert_eq!(b.elytra_damage, 0, "the fresh j=1 the hurt gate refuses");
    }

    /// THE LAST POINT'S OWN REFUSAL (javap `canGlideUsing`'s
    /// nextDamageWillBreak gate the 26.2 walk — the damage >= max - 1
    /// stack the verdict refuses) + THE HURT LAW'S OWN ARMS (javap
    /// `ItemStack.hurtAndBreak` — the isBroken arm the 432 crossing
    /// speaks, the hasInfiniteMaterials gate the creative stack never
    /// carries).
    #[test]
    fn the_elytra_wear_the_refusal_the_break_the_creative() {
        let mut w = World::new(1, 42);
        for x in 0..8 {
            for z in 0..8 {
                for y in 0..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, if y == 0 { crate::world::STONE } else { AIR });
                }
            }
        }
        // THE REFUSAL: the 431-damage body the glide retires — the
        // stop arm the next tick's verdict speaks, the wear the retire
        // tick never rides, the break the glide's own reach never
        // triggers (the vanilla's torn elytra the glide never rides)
        let mut b = PlayerBody::new(2.0, 64.0, 2.0, 0.0, 0.0);
        b.fall_flying = true;
        b.chest_glides = true;
        b.elytra_damage = 431;
        b.tick(&w, false);
        assert!(!b.fall_flying, "the last point the stop arm retires");
        assert_eq!(b.fall_fly_ticks, 0, "the retire tick the wear never rides");
        assert!(!b.elytra_broke_due, "the break the glide's own reach never speaks");
        // THE PENULTIMATE POINT: the 430-damage body still glides (the
        // 430 < 431 gate the verdict keeps)
        let mut b = PlayerBody::new(2.0, 64.0, 2.0, 0.0, 0.0);
        b.fall_flying = true;
        b.chest_glides = true;
        b.elytra_damage = 430;
        b.tick(&w, false);
        assert!(b.fall_flying, "the penultimate point the glide keeps");
        // THE ISBROKEN ARM (the direct hurt call — the shared law the
        // other sources the lane may speak): 431 + 1 the 432 crossing
        // answers, the due flag the App's drain reads
        let mut b = PlayerBody::new(0.0, 0.0, 0.0, 0.0, 0.0);
        b.elytra_damage = 431;
        b.elytra_hurt_and_break(1);
        assert_eq!(b.elytra_damage, 432, "the applyDamage's += the walk keeps");
        assert!(b.elytra_broke_due, "the isBroken arm the 432 crossing speaks");
        // THE CREATIVE ARM: the infinite-materials stack the damage
        // never carries (the processDurabilityChange's own zero)
        let mut b = PlayerBody::new(0.0, 0.0, 0.0, 0.0, 0.0);
        b.infinite_materials = true;
        b.elytra_damage = 100;
        b.elytra_hurt_and_break(1);
        assert_eq!(b.elytra_damage, 100, "the creative gate the damage never rides");
        assert!(!b.elytra_broke_due, "the creative stack never breaks");
    }

    /// THE BROKEN VERDICT'S OWN LAW (javap
    /// `ItemStack.nextDamageWillBreak` the 26.2 walk — the
    /// `isDamageableItem && damage >= maxDamage - 1` threshold the
    /// if_icmplt compares, pc 17 the crossing speaks): the
    /// `minecraft:broken` item-model condition's own truth — the 430
    /// stack the resting verdict keeps, the 431 stack the torn verdict
    /// wears (ONE point before the break), the crossed stack stays
    /// true, and the verdict the glide's refusal pairs (the SAME
    /// threshold `fall_flying_can_glide`'s tail speaks — the torn
    /// elytra's two evidences the one law keeps).
    #[test]
    fn the_broken_verdict_the_last_point_wears() {
        let mut b = PlayerBody::new(0.0, 0.0, 0.0, 0.0, 0.0);
        b.chest_glides = true;
        b.fall_flying = true;
        // THE FRESH-PENULTIMATE CROSSING (429..430 the resting arm)
        b.elytra_damage = 429;
        assert!(!b.elytra_next_damage_will_break(), "the 429 stack the resting verdict keeps");
        b.elytra_damage = 430;
        assert!(!b.elytra_next_damage_will_break(), "the 430 stack the resting verdict keeps");
        assert!(b.fall_flying_can_glide(), "the 430 stack the glide still rides");
        // THE TORN POINT (431 — the verdict flips, the glide refuses)
        b.elytra_damage = 431;
        assert!(b.elytra_next_damage_will_break(), "the 431 stack the torn verdict wears");
        assert!(!b.fall_flying_can_glide(), "the 431 stack the glide's gate refuses");
        // THE CROSSED STACK (432 — the broken stack stays true, the
        // verdict the post-break reads never lies)
        b.elytra_damage = 432;
        assert!(b.elytra_next_damage_will_break(), "the crossed stack the verdict keeps true");
    }

    /// THE WINGS ANIM'S OWN LAW (javap `ElytraAnimationState.tick` +
    /// `getRotX/Y/Z` the 26.2 walk — the rest/crouch/glide targets the
    /// branch constants keep, the 0.3 ease the tail wears, the old/cur
    /// bank the render lerp reads).
    #[test]
    fn the_elytra_wing_anim_the_law_wears() {
        let w = flat_world();
        // THE REST ARM: the standing body's wings converge to the
        // DEFAULT pair — from the Java fields' own 0.0 start the ease
        // walks, never snaps
        let mut b = PlayerBody::new(2.0, 1.0, 2.0, 0.0, 0.0);
        assert_eq!(b.elytra_wing_rot_x, 0.0, "the fresh state's Java zero");
        for _ in 0..80 {
            b.tick(&w, false);
        }
        let (rx, ry, rz) = b.elytra_wing_rot_at(1.0);
        assert!(
            (rx - ELYTRA_WING_DEFAULT_X).abs() < 1.0e-5,
            "the rest x the default keeps, got {rx}"
        );
        assert!(ry.abs() < 1.0e-5, "the rest y the zero keeps, got {ry}");
        assert!(
            (rz - ELYTRA_WING_DEFAULT_Z).abs() < 1.0e-5,
            "the rest z the default keeps, got {rz}"
        );
        // THE FIRST-TICK EASE: the fresh body's single tick — old 0
        // banked, cur = (target - 0) * 0.3 (the ease's own walk)
        let mut b = PlayerBody::new(2.0, 1.0, 2.0, 0.0, 0.0);
        b.tick(&w, false);
        assert_eq!(b.elytra_wing_rot_x_old, 0.0, "the old the bank keeps");
        assert!(
            (b.elytra_wing_rot_x - ELYTRA_WING_DEFAULT_X * ELYTRA_WING_EASE).abs() < 1.0e-6,
            "the first ease the 0.3 speaks, got {}",
            b.elytra_wing_rot_x
        );
        // THE RENDER LERP (getRotX(partialTick) = lerp(partial, old,
        // cur)): halfway the exact midpoint the formula keeps
        let mid = b.elytra_wing_rot_at(0.5).0;
        assert!(
            (mid - b.elytra_wing_rot_x * 0.5).abs() < 1.0e-6,
            "the half-tick lerp the exact midpoint keeps"
        );
        // THE CROUCH ARM: the folded-back triple the isCrouching branch
        // wears (x 40°, y 5°, z -45°), the convergence the ease walks
        let mut b = PlayerBody::new(2.0, 1.0, 2.0, 0.0, 0.0);
        // the pose walk the tick's own verdict keeps (the want the
        // key speaks, the crouching the updatePlayerPose answers)
        b.want_crouch = true;
        for _ in 0..80 {
            b.tick(&w, false);
        }
        let (rx, ry, rz) = b.elytra_wing_rot_at(1.0);
        assert!((rx - ELYTRA_WING_CROUCH_X).abs() < 1.0e-5, "the crouch x, got {rx}");
        assert!((ry - ELYTRA_WING_CROUCH_Y).abs() < 1.0e-5, "the crouch y, got {ry}");
        assert!((rz - ELYTRA_WING_CROUCH_Z).abs() < 1.0e-5, "the crouch z, got {rz}");
        // THE GLIDE ARM: the gliding body the spread wears — the dive
        // the velocity's own fold banks (the falling body's forward
        // thrust the glide folds speak: ny < 1 the f > 0 answers, the
        // z target below the default the flat-out spread keeps)
        let mut b = PlayerBody::new(2.0, 40.0, 2.0, 0.0, 0.0);
        b.chest_glides = true;
        b.fall_flying = true;
        for _ in 0..40 {
            b.tick(&w, false);
            assert!(b.fall_flying, "the airborne glide never retires");
        }
        let (_, ry_g, rz_g) = b.elytra_wing_rot_at(1.0);
        assert!(
            ry_g.abs() < 1.0e-5,
            "the glide arm's y target the zero keeps, got {ry_g}"
        );
        assert!(
            rz_g < ELYTRA_WING_DEFAULT_Z - 0.1,
            "the glide z the spread wears (past the default), got {rz_g}"
        );
        // THE STOP'S OWN RETURN (the unequip arm — the chest verdict
        // the next tick's stop speaks): the wings ease BACK to the
        // default (the same convergence the rest arm speaks, the
        // animation never freezes at the spread)
        b.chest_glides = false;
        for _ in 0..80 {
            b.tick(&w, false);
            assert!(!b.fall_flying, "the unequip the stop arm retires");
        }
        let (rx, _, rz) = b.elytra_wing_rot_at(1.0);
        assert!(
            (rx - ELYTRA_WING_DEFAULT_X).abs() < 1.0e-5 && (rz - ELYTRA_WING_DEFAULT_Z).abs() < 1.0e-5,
            "the stop the wings ease back, got ({rx}, {rz})"
        );
    }

    /// THE GLIDE'S OWN RETIRE (javap `LivingEntity.updateFallFlying`'s
    /// own !canGlide verdict the 26.2 walk — the ground the landing
    /// speaks, the elytra's unequip the verdict answers, the water
    /// touch the verdict NEVER reads — the flag the fluid keeps, the
    /// exit mid-air the glide resumes).
    #[test]
    fn the_glide_the_ground_the_unequip_retire() {
        let mut w = World::new(1, 42);
        for x in 0..8 {
            for z in 0..8 {
                for y in 0..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, if y == 0 { crate::world::STONE } else { AIR });
                }
            }
        }
        // THE UNEQUIP: the gliding body the chest verdict loses —
        // the flag the next tick's stop arm clears
        let mut b = PlayerBody::new(3.0, 40.0, 3.0, 0.0, 0.0);
        b.fall_flying = true;
        b.tick(&w, false);
        assert!(!b.fall_flying, "the unequip the stop arm retires");
        // THE FLUID KEEP: the gliding body over the pool the flag
        // KEEPS (the canGlide verdict reads no fluid — the exit
        // mid-air the glide resumes)
        let mut w = World::new(1, 42);
        for x in 0..8 {
            for z in 0..8 {
                for y in 0..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, if y == 0 { crate::world::STONE } else { AIR });
                }
                w.set(x, 1, z, WATER);
            }
        }
        let mut b = PlayerBody::new(3.0, 2.5, 3.0, 0.0, 0.0);
        b.chest_glides = true;
        b.fall_flying = true;
        settle(&mut b, &w);
        assert!(b.fall_flying, "the water touch the flag never clears");
        // THE GROUND RETIRE: the landed body the verdict refuses —
        // the flag the landing tick clears
        let mut w = World::new(1, 42);
        for x in 0..8 {
            for z in 0..8 {
                for y in 0..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, if y == 0 { crate::world::STONE } else { AIR });
                }
            }
        }
        let mut b = PlayerBody::new(3.0, 1.0, 3.0, 0.0, 0.0);
        b.on_ground = true;
        b.chest_glides = true;
        b.fall_flying = true;
        settle(&mut b, &w);
        assert!(!b.fall_flying, "the landing the stop arm retires");
    }

    /// THE ENTRY'S BRAND (LavaFluid.entityInside's own beat — the
    /// box overlaps a lava cell): the clock rises to the
    /// igniteForSeconds(15) fold's 300 only when UNDER it, the
    /// lava_hurt_due fires, and the NEXT tick's head sees in_lava —
    /// the onFire hurt never speaks while the body stands in the
    /// pool (the !isInLava gate), the clock holds at the brand's
    /// value (the contact re-ignites what the clock drained).
    #[test]
    fn the_lava_entry_ignites_and_holds_the_brand() {
        // a 3x3 lava shelf in a stone floor: lava at y=1, the body
        // walks in off the surrounding ground (the spawn side)
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 0, z, crate::world::STONE);
                for y in 1..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, AIR);
                }
            }
        }
        for x in 7..=9 {
            for z in 7..=9 {
                w.set(x, 1, z, crate::world::LAVA);
            }
        }
        let mut b = PlayerBody::new(6.0, 1.0, 8.0, -90.0, 0.0); // east, toward the pool
        settle(&mut b, &w);
        assert!(!b.touches_lava(&w), "the body starts off the shelf");
        b.forward = 1.0;
        let mut entry_hurt = false;
        // 12 ticks: the walk carries the body INTO the 3-wide shelf and
        // stops mid-pool (the ~0.216/tick stride the walk law speaks —
        // the entry lands at tick ~3, the far lip sits 3.3 blocks out)
        for _ in 0..12 {
            b.tick(&w, false);
            if b.lava_hurt_due {
                entry_hurt = true;
                assert_eq!(
                    b.remaining_fire_ticks, LAVA_FIRE_TICKS,
                    "the brand rides the igniteForSeconds(15) fold's 300"
                );
            }
            if b.in_lava {
                assert!(
                    !b.on_fire_hurt_due,
                    "the in-lava ticks never speak the onFire hurt (the !isInLava gate)"
                );
            }
        }
        assert!(entry_hurt, "the entry spoke its lavaHurt");
        assert!(b.in_lava, "the body stands in the pool");
        assert_eq!(
            b.remaining_fire_ticks, LAVA_FIRE_TICKS,
            "the clock holds at the brand's value (the contact re-ignites)"
        );
        assert!(b.fall_distance == 0.0, "the fluid scan's reset holds");
    }

    /// THE EXIT'S OWN CLOCK (baseTick's fire law once the body LEFT
    /// the lava): the onFire hurt speaks at the % 20 cadence's own
    /// beats (300, 280, 260 — the remainingFireTicks the clock walks
    /// down), the in-between ticks stay silent, and the brand drains
    /// to zero at the 300-tick end.
    #[test]
    fn the_exit_burns_on_the_twenty_tick_cadence() {
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 0, z, crate::world::STONE);
                for y in 1..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, AIR);
                }
            }
        }
        // NO lava anywhere — the body carries the brand alone (the
        // freshly-exited wader the laws speak)
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        b.remaining_fire_ticks = LAVA_FIRE_TICKS;
        let mut hurt_ticks = Vec::new();
        for t in 0..300i64 {
            b.tick(&w, false);
            if b.on_fire_hurt_due {
                hurt_ticks.push(t);
                // the hurt fires at the clock's % 20 beat — the tick
                // the countdown STARTED the 300 from (the head's read
                // speaks the pre-decrement value)
                assert_eq!(
                    (b.remaining_fire_ticks + 1) % FIRE_DAMAGE_INTERVAL,
                    0,
                    "the beat rides the pre-decrement clock (tick {t}, now {})",
                    b.remaining_fire_ticks
                );
            }
            assert!(b.remaining_fire_ticks >= 0, "the clock never goes under");
        }
        assert_eq!(b.remaining_fire_ticks, 0, "the 300-tick brand drains out");
        assert_eq!(hurt_ticks.len(), 15, "the onFire hurt spoke every 20th tick");
    }

    /// THE WATER'S EXTINGUISH (WaterFluid.entityInside's own
    /// EXTINGUISH = Entity::clearFire): the burning body touching
    /// water dies to zero and fires fire_doused_due — the fizz the
    /// drain speaks — and a wet body with no clock stays silent.
    #[test]
    fn the_water_douse_kills_the_brand() {
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
        // the burning wader: the brand rides in, the pool's own
        // inside-effect kills it the same tick the water touches
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        assert!(b.touches_water(&w));
        b.remaining_fire_ticks = LAVA_FIRE_TICKS;
        b.tick(&w, false);
        assert!(b.fire_doused_due, "the douse spoke its flag");
        assert_eq!(b.remaining_fire_ticks, 0, "the clearFire fold's zero");
        // the second wet tick: no clock to kill — the flag stays dark
        b.tick(&w, false);
        assert!(!b.fire_doused_due, "a wet body carries no clock to kill");
        // the pre-doused body: the flag never speaks
        let mut c = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut c, &w);
        c.tick(&w, false);
        assert!(!c.fire_doused_due, "the unburnt wader never fizzes");
    }

    /// THE FIRE CELL'S OWN BRAND (javap `BaseFireBlock.entityInside`
    /// the 26.2 jar — the inside walk the move's tail speaks): the
    /// body standing in the fire cell the contact ignites — the
    /// fireIgnite fold lifts the clock to the igniteForSeconds(8)'s
    /// 160 and the hurt flag fires — and the SOUL FIRE's cell the
    /// SAME parent walk keeps with its own doubled-dose flag (the
    /// 2.0 the ctor's fconst_2 speaks). The clock holds at the floor
    /// (the contact re-lifts what the tick drained), the onFire beat
    /// rides the % 20 cadence while the body stands (the !isInLava
    /// gate the fire lane never arms), and the WATER douse the NEXT
    /// beat still wins (the applier's queue order the inside walk
    /// keeps — the fire-then-water pair the straddling box speaks).
    #[test]
    fn the_fire_cell_ignites_the_standing_body() {
        // a stone floor: fire at the center cell, soul fire beside,
        // water the straddle's own douse witness
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 0, z, crate::world::STONE);
                for y in 1..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, AIR);
                }
            }
        }
        w.set(8, 1, 8, crate::world::FIRE);
        w.set(12, 1, 8, crate::world::SOUL_FIRE);
        // THE PLAIN FIRE'S ENTRY: the walk carries the body INTO the
        // cell (the ~0.216/tick stride the walk law speaks — the
        // 12-tick arc lands the box mid-flame)
        let mut b = PlayerBody::new(6.0, 1.0, 8.0, -90.0, 0.0); // east, toward the flame
        settle(&mut b, &w);
        assert!(!b.touches_fire(&w), "the body starts off the flame");
        b.forward = 1.0;
        let mut entry_hurt = false;
        for _ in 0..12 {
            b.tick(&w, false);
            if b.in_fire_hurt_due {
                entry_hurt = true;
                assert_eq!(
                    b.remaining_fire_ticks, FIRE_IGNITE_TICKS,
                    "the brand rides the igniteForSeconds(8) fold's 160"
                );
            }
            assert!(
                !b.in_soul_fire_hurt_due,
                "the plain flame never speaks the soul's flag"
            );
        }
        assert!(entry_hurt, "the entry spoke its entityInside hurt");
        assert!(b.touches_fire(&w), "the body stands in the flame");
        assert_eq!(
            b.remaining_fire_ticks, FIRE_IGNITE_TICKS,
            "the clock holds at the floor (the contact re-lifts)"
        );
        // THE STANDING HOLD: the body resting IN the flame cell speaks
        // the hurt flag every tick and the clock never leaves the
        // floor; the onFire beat rides the % 20 cadence's own head
        // read (160 the beat the countdown walks)
        let mut e = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0); // centered IN the flame
        settle(&mut e, &w);
        assert!(e.touches_fire(&w), "the settled body rests in the cell");
        let mut on_fire_beats = 0;
        for _ in 0..20 {
            e.tick(&w, false);
            assert!(
                e.in_fire_hurt_due,
                "the contact fires the hurt every tick it overlaps"
            );
            assert_eq!(e.remaining_fire_ticks, FIRE_IGNITE_TICKS);
            if e.on_fire_hurt_due {
                on_fire_beats += 1;
            }
        }
        assert!(on_fire_beats >= 1, "the onFire beat spoke while standing");
        // THE SOUL FIRE'S OWN DOSE (the same walk, the doubled flag):
        // the body standing in the soul cell speaks the soul's flag —
        // the plain flame's flag silent
        let mut c = PlayerBody::new(12.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut c, &w);
        c.tick(&w, false);
        assert!(c.touches_soul_fire(&w), "the body stands in the soul flame");
        assert!(
            c.in_soul_fire_hurt_due,
            "the soul cell speaks its doubled dose"
        );
        assert!(
            !c.in_fire_hurt_due,
            "the soul flame never speaks the plain fire's flag"
        );
        assert_eq!(
            c.remaining_fire_ticks, FIRE_IGNITE_TICKS,
            "the shared parent law lifts the same 160"
        );
        // THE WATER DOSE WINS (the queue order the straddle speaks):
        // the box over fire AND water the same tick — the fire beat
        // arms (the flag the drain reads), the water's clearFire
        // kills the clock to zero, the fizz speaks
        let mut w2 = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                w2.set(x, 0, z, crate::world::STONE);
                for y in 1..crate::world::CHUNK_Y {
                    w2.set(x, y as i32, z, AIR);
                }
            }
        }
        w2.set(8, 1, 8, crate::world::FIRE);
        w2.set(9, 1, 8, WATER);
        let mut d = PlayerBody::new(8.9, 1.0, 8.0, 0.0, 0.0); // the box straddles both cells
        settle(&mut d, &w2);
        d.tick(&w2, false);
        assert!(
            d.in_fire_hurt_due,
            "the fire beat armed before the water's own"
        );
        assert!(d.fire_doused_due, "the douse spoke its fizz");
        assert_eq!(
            d.remaining_fire_ticks, 0,
            "the clearFire fold's zero wins the tick"
        );
    }

    /// THE LAVA LANDING'S OWN GRACE (the fluid scan's reset the
    /// water lane shares): a body falling INTO the lava pool never
    /// banks the descent — the fall_event never speaks, the lava's
    /// own 4.0 is the welcome the pool gives.
    #[test]
    fn the_lava_landing_never_hurts_the_fall() {
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 0, z, crate::world::STONE);
                w.set(x, 1, z, crate::world::LAVA);
                for y in 2..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, AIR);
                }
            }
        }
        let mut b = PlayerBody::new(8.0, 6.0, 8.0, 0.0, 0.0); // dropped INTO the pool
        let mut lava_hurt = false;
        for _ in 0..40 {
            b.tick(&w, false);
            if b.lava_hurt_due {
                lava_hurt = true;
            }
            assert!(
                b.fall_event.is_none(),
                "the fluid landing never folds the fall damage"
            );
        }
        assert!(lava_hurt, "the pool's own welcome spoke");
        assert!(b.on_ground, "the body settled on the pool floor");
    }

    /// THE DIVER'S OWN POOL: a 3-deep water column over stone — the
    /// body sunk to the floor rides its eye (feet + 1.62) TWO cells
    /// under the surface (the isEyeInFluid query the air clock gates).
    fn diver_pool() -> World {
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
        w
    }

    /// THE EYE'S DRAIN (javap `LivingEntity.baseTick`'s own air
    /// branch): the eye under the surface speaks eye_in_water and the
    /// clock walks DOWN one per tick — below zero the -20 floor the
    /// dose's rhythm needs (the store goes UNDER the zero the drain
    /// never clamps at); the dry body's verdict stays honest.
    #[test]
    fn the_eye_under_water_drains_the_air_clock() {
        let w = diver_pool();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        b.vy = 0.0;
        b.on_ground = true;
        b.tick(&w, false);
        assert!(b.eye_in_water, "the diver's own eye reads water");
        assert_eq!(b.air_supply, TOTAL_AIR_SUPPLY - 1, "the drain's first tick");
        for _ in 0..40 {
            b.tick(&w, false);
        }
        assert_eq!(
            b.air_supply,
            TOTAL_AIR_SUPPLY - 41,
            "the one-per-tick walk ({})",
            b.air_supply
        );
        assert!(
            !b.drown_hurt_due,
            "the drain alone never speaks the dose (the -20 floor unmet)"
        );
        // the dry body: the verdict off, the clock untouched
        let d = flat_world();
        let mut c = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut c, &d);
        c.tick(&d, false);
        assert!(!c.eye_in_water, "the dry body's own eye reads air");
        assert_eq!(c.air_supply, TOTAL_AIR_SUPPLY, "the full store stays full");
    }

    /// THE DRY REFILL (javap `increaseAirSupply`'s own fold —
    /// `Math.min(air + 4, max)`): the surfaced body's clock climbs
    /// four a tick and caps at the 300 the getMaxAirSupply sipush
    /// speaks — the +4 walk never overshoots.
    #[test]
    fn the_dry_body_refills_four_a_tick() {
        let w = flat_world();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        settle(&mut b, &w);
        b.air_supply = 100;
        b.tick(&w, false);
        assert_eq!(b.air_supply, 104, "the +4 fold's own step");
        for _ in 0..60 {
            b.tick(&w, false);
        }
        assert_eq!(
            b.air_supply, TOTAL_AIR_SUPPLY,
            "the cap rides the 300 the sipush names"
        );
    }

    /// THE BREATH EFFECTS' OWN GATES (javap `MobEffectUtil` the pair
    /// + baseTick pcs 244-362): the water-breathing body the
    /// eye-under-water drain refuses and the refill the gate's true
    /// arm grants (the +4 climb wears), the nautilus-ONLY body the
    /// FREEZE keeps (the drain the triple refuses, the refill the
    /// gate refuses — the clock the stillness speaks), the
    /// invulnerable body the drain refused and the refill the
    /// nautilus-absent true arm keeps.
    #[test]
    fn the_breath_effects_the_air_clock_gates_wear() {
        // THE WATER-BREATHING BODY: no drain, the +4 climb
        let w = diver_pool();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        b.vy = 0.0;
        b.on_ground = true;
        b.water_breathing = true;
        b.tick(&w, false);
        assert!(b.eye_in_water, "the diver's own eye reads water");
        assert_eq!(
            b.air_supply, TOTAL_AIR_SUPPLY,
            "the breathing body the drain refuses"
        );
        b.air_supply = 100;
        b.tick(&w, false);
        assert_eq!(
            b.air_supply, 104,
            "the underwater refill the gate's true arm grants"
        );
        // THE NAUTILUS-ONLY BODY: the freeze (no drain, no refill)
        let mut n = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        n.vy = 0.0;
        n.on_ground = true;
        n.water_breathing = true;
        n.effects_refill_air_supply = false;
        n.air_supply = 150;
        n.tick(&w, false);
        assert_eq!(n.air_supply, 150, "the nautilus-ONLY body the freeze holds");
        n.tick(&w, false);
        assert_eq!(n.air_supply, 150, "the freeze the second tick keeps");
        // THE INVULNERABLE BODY: the drain refused, the refill the
        // gate's true (the nautilus-absent) arm keeps
        let mut i = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        i.vy = 0.0;
        i.on_ground = true;
        i.invulnerable = true;
        i.air_supply = 100;
        i.tick(&w, false);
        assert_eq!(i.air_supply, 104, "the invulnerable body the refill keeps");
    }

    /// THE DOSE'S OWN RHYTHM (javap shouldTakeDrowningDamage's -20
    /// floor + the fold's setAirSupply(0)): from the seeded 1 the
    /// drain walks 21 ticks to the floor — the dose flag rides, the
    /// store resets to zero — and the NEXT dose waits its own 20 (the
    /// 2.0F beats the rhythm speaks, the flag cleared at the head
    /// each tick).
    #[test]
    fn the_drowned_floor_folds_the_twenty_tick_dose() {
        let w = diver_pool();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        b.vy = 0.0;
        b.on_ground = true;
        b.tick(&w, false); // 299
        b.air_supply = 1;
        let mut doses = Vec::new();
        for t in 0..45 {
            b.tick(&w, false);
            if b.drown_hurt_due {
                doses.push(t);
                assert_eq!(b.air_supply, 0, "the floor's own reset (tick {t})");
            }
        }
        assert_eq!(
            doses,
            vec![20, 40],
            "the -20 floor folds the dose every 20 ticks after the reset"
        );
    }

    /// The tick-head save (javap `Entity.tick`'s own `xo = x` head):
    /// the pre-move feet land in prev_* before the physics walks, the
    /// ctor arms prev at the spawn, and the render fold's lerp rides
    /// the half-way point at partial 0.5 while the partial >= 1.0 arm
    /// speaks the CURRENT feet bit for bit (the headless contract).
    #[test]
    fn the_tick_head_saves_the_prev_and_the_lerp_walks_it() {
        let mut w = World::new(1, 42);
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, if y == 0 { crate::world::STONE } else { crate::world::AIR });
                }
            }
        }
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        // the ctor arm: the spawn is its own first prev
        assert_eq!((b.prev_x, b.prev_y, b.prev_z), (8.0, 1.0, 8.0));
        settle(&mut b, &w);
        b.forward = 1.0;
        // the walk impulse builds momentum (the vanilla movement's own
        // friction walk) — tick until the feet MOVED, then the fold's
        // laws speak on a real delta. The yaw-0 walk's own axis: the
        // HORIZONTAL distance is the honest moved read, whichever axis
        // the facing speaks
        let moved = |b: &PlayerBody| ((b.x - 8.0).powi(2) + (b.z - 8.0).powi(2)).sqrt();
        for _ in 0..40 {
            b.tick(&w, false);
            if moved(&b) > 1e-6 {
                break;
            }
        }
        // the body MOVED (the walk impulse speaks), the midpoint rides
        // half the delta, and 1.0 = the current feet
        assert!(moved(&b) > 1e-6, "the walk moved the body: {}", moved(&b));
        let (lx, _, _) = b.lerped_pos(0.5);
        let half = b.prev_x + (b.x - b.prev_x) * 0.5;
        assert!((lx - half).abs() < 1e-12, "midpoint: {lx} vs {half}");
        assert_eq!(b.lerped_pos(1.0), (b.x, b.y, b.z));
        assert_eq!(b.lerped_pos(1.5), (b.x, b.y, b.z));
        // the clamp arm: partial 0.0 walks FROM the previous feet
        assert_eq!(b.lerped_pos(0.0), (b.prev_x, b.prev_y, b.prev_z));
    }

    #[test]
    fn the_walk_cosines_ride_the_frame_partial() {
        // the two sampling laws the render state speaks (javap
        // WalkAnimationState.speed/position — the backward
        // extrapolation the setupAnim cosines walk): the store MOVED,
        // so the partial chooses the phase the frame renders
        let mut w = WalkAnimationState::default();
        w.speed_old = 0.2;
        w.speed = 0.6;
        w.position = 3.0;
        w.position_scale = 1.0;
        // speed(partial): the old->current blend, clamped [0, 1]
        assert!((w.speed_at(0.0) - 0.2).abs() < 1e-6);
        assert!((w.speed_at(0.5) - 0.4).abs() < 1e-6);
        assert!((w.speed_at(1.0) - 0.6).abs() < 1e-6);
        w.speed = 9.0; // the clamp arm pins at 1.0
        assert_eq!(w.speed_at(1.0), 1.0);
        // position(partial): position - speed * (1 - partial) —
        // 0.0 the pre-move phase whole, 1.0 the current phase
        w.speed = 0.6;
        assert!((w.position_at(0.0) - 2.4).abs() < 1e-6);
        assert!((w.position_at(0.5) - 2.7).abs() < 1e-6);
        assert!((w.position_at(1.0) - 3.0).abs() < 1e-6);
        // the scale arm: the baby's 3.0 fold rides the product
        w.position_scale = 3.0;
        assert!((w.position_at(1.0) - 9.0).abs() < 1e-6);
    }

    #[test]
    fn the_eye_height_walks_the_crouch_smooth() {
        // the ctor starts settled at the standing eye
        let w = flat_world();
        let mut b = PlayerBody::new(3.0, 1.0, 3.0, 0.0, 0.0);
        assert_eq!(b.eye_height, STANDING_EYE);
        assert_eq!(b.lerped_eye_height(0.5), STANDING_EYE);
        // the crouch key arms: one tick blends HALF the gap toward the
        // crouch eye (the 0.5 fold), the head keeps the standing eye
        // the lerp walks FROM
        b.want_crouch = true;
        b.tick(&w, false);
        assert!(b.crouching, "the pose resolved");
        assert_eq!(b.prev_eye_height, STANDING_EYE, "the head saved");
        let half = STANDING_EYE + (CROUCH_EYE - STANDING_EYE) * EYE_HEIGHT_BLEND;
        assert!((b.eye_height - half).abs() < 1e-12, "the blend's own half");
        // the camera's lerp folds the midpoint; >= 1.0 the current
        // bit for bit, 0.0 the prev whole
        let mid = STANDING_EYE + (b.eye_height - STANDING_EYE) * 0.5;
        assert!((b.lerped_eye_height(0.5) - mid).abs() < 1e-12);
        assert_eq!(b.lerped_eye_height(1.0), b.eye_height);
        assert_eq!(b.lerped_eye_height(0.0), b.prev_eye_height);
        // the settled walk: the blend converges to the crouch eye,
        // the release walks it back up
        for _ in 0..12 {
            b.tick(&w, false);
        }
        assert!((b.eye_height - CROUCH_EYE).abs() < 0.001, "the crouch settled");
        b.want_crouch = false;
        for _ in 0..12 {
            b.tick(&w, false);
        }
        assert!((b.eye_height - STANDING_EYE).abs() < 0.001, "the stand settled");
    }

    // ===== THE CAMERA'S OWN FOV (Camera.tickFov + the
    // getFieldOfViewModifier law the 26.2 jar's raw disasms speak) =====

    /// The idle walk's ratio speaks 1.0 — the modifier the walk lane
    /// never moves (the walk-speed denominator the attribute's own
    /// base keeps), the pair of stores the 1.0 the whole walk keeps.
    #[test]
    fn the_idle_walk_keeps_the_ones_modifier() {
        let w = flat_world();
        let mut b = PlayerBody::new(2.5, 1.0, 2.5, 90.0, 0.0);
        settle(&mut b, &w);
        for _ in 0..10 {
            b.tick(&w, false);
        }
        assert_eq!(b.fov_modifier, 1.0, "the idle walk the 1.0 keeps");
        assert_eq!(b.prev_fov_modifier, 1.0, "the old store the 1.0 keeps");
    }

    /// The sprint's 1.3 attribute the ratio law speaks — the target
    /// 1.15 the (1.3 + 1)/2 fold answers, the half-step the first
    /// beat arms (1.075 the bytecode's own blend), the convergence
    /// the ticks speak, the old store the pre-tick value keeps.
    #[test]
    fn the_sprint_target_speaks_the_ratio_law() {
        let w = flat_world();
        let mut b = PlayerBody::new(2.5, 1.0, 2.5, 90.0, 0.0);
        settle(&mut b, &w);
        b.sprinting = true;
        let prev = b.fov_modifier;
        b.tick(&w, false);
        assert_eq!(
            b.prev_fov_modifier, prev,
            "the old store the pre-tick value keeps"
        );
        assert!(
            (b.fov_modifier - 1.075).abs() < 1e-5,
            "the first beat the half-step speaks: {}",
            b.fov_modifier
        );
        for _ in 0..24 {
            b.tick(&w, false);
        }
        assert!(
            (b.fov_modifier - 1.15).abs() < 1e-5,
            "the convergence the 1.15 speaks: {}",
            b.fov_modifier
        );
    }

    /// The sneak's 0.7 attribute the ratio law speaks — the target
    /// 0.85 the (0.7 + 1)/2 fold answers, the narrower frame the
    /// crouched walk converges to.
    #[test]
    fn the_sneak_target_speaks_the_ratio_law() {
        let w = flat_world();
        let mut b = PlayerBody::new(2.5, 1.0, 2.5, 90.0, 0.0);
        settle(&mut b, &w);
        b.want_crouch = true;
        b.tick(&w, false);
        assert!(b.crouching, "the flat ground the crouch resolves");
        for _ in 0..24 {
            b.tick(&w, false);
        }
        assert!(
            (b.fov_modifier - 0.85).abs() < 1e-4,
            "the crouch walk the 0.85 speaks: {}",
            b.fov_modifier
        );
    }

    /// The using slowdown's 0.2 the ratio law speaks — the target 0.6
    /// the (0.2 + 1)/2 fold answers, the zoom-in the eating frame
    /// converges to.
    #[test]
    fn the_using_slowdown_narrows_the_frame() {
        let w = flat_world();
        let mut b = PlayerBody::new(2.5, 1.0, 2.5, 90.0, 0.0);
        settle(&mut b, &w);
        for _ in 0..24 {
            b.tick(&w, true);
        }
        assert!(
            (b.fov_modifier - 0.6).abs() < 1e-4,
            "the using walk the 0.6 speaks: {}",
            b.fov_modifier
        );
    }

    /// The clamp the bytecode pins (0.1/1.5): an extreme target the
    /// half-step never lets through — the 1.5 ceiling the raw 2.0
    /// meets, the 0.1 floor the fourth low beat meets.
    #[test]
    fn the_fov_clamp_walks_the_bytecode_bounds() {
        let mut b = PlayerBody::new(2.5, 1.0, 2.5, 90.0, 0.0);
        b.tick_fov(3.0);
        assert_eq!(
            b.fov_modifier, FOV_MODIFIER_MAX,
            "the ceiling the raw 2.0 meets"
        );
        let mut b = PlayerBody::new(2.5, 1.0, 2.5, 90.0, 0.0);
        for _ in 0..4 {
            b.tick_fov(0.0);
        }
        // the beats walk 0.5, 0.25, 0.125, 0.0625 — the floor the
        // fourth meets
        assert_eq!(
            b.fov_modifier, FOV_MODIFIER_MIN,
            "the floor the low beats meet"
        );
    }

    /// The frame's read the calculateFov lerp speaks: the partial the
    /// walk between the two stores keeps, the >= 1.0 arm the current
    /// bit for bit (the headless lanes' byte-identical contract), the
    /// 0.0 the old store whole.
    #[test]
    fn the_fov_read_walks_the_frame_partial() {
        let mut b = PlayerBody::new(2.5, 1.0, 2.5, 90.0, 0.0);
        b.prev_fov_modifier = 1.0;
        b.fov_modifier = 1.075;
        assert_eq!(
            b.fov_modifier_at(1.0),
            1.075,
            "the full partial the current keeps"
        );
        assert!(
            (b.fov_modifier_at(0.5) - 1.0375).abs() < 1e-6,
            "the half partial the midpoint speaks"
        );
        assert!(
            (b.fov_modifier_at(0.0) - 1.0).abs() < 1e-6,
            "the zero partial the old store keeps"
        );
    }

    /// A column world: the whirlpool's (15294) or the upward's (15295)
    /// real ids stacked x=8, y=low..high (the surface cell at `high`,
    /// the air above), the variants layer the honest empty.
    fn column_world(id: u16, low: i32, high: i32) -> World {
        let mut w = flat_world();
        w.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 1]);
        for y in low..=high {
            w.set(8, y, 8, id);
        }
        w
    }

    #[test]
    fn the_column_drag_folds_the_bytecode_caps() {
        // THE WHIRLPOOL'S STEADY PULL: `y - 0.03` floored at -0.3 the
        // body cell keeps, -0.9 the surface cell's mouth keeps
        assert!((bubble_column_drag(0.0, true, false) - (-0.03)).abs() < 1e-12);
        assert_eq!(bubble_column_drag(-0.5, true, false), -0.3, "the body's floor");
        assert!((bubble_column_drag(-0.5, true, true) - (-0.53)).abs() < 1e-12);
        assert_eq!(bubble_column_drag(-1.0, true, true), -0.9, "the surface's floor");
        // THE UPWARD'S STEADY PUSH: `y + 0.06` capped at 0.7 the body
        // cell keeps, the `y + 0.1` launch capped at 1.8 the surface
        assert!((bubble_column_drag(0.0, false, false) - 0.06).abs() < 1e-12);
        assert_eq!(bubble_column_drag(1.0, false, false), 0.7, "the body's cap");
        assert!((bubble_column_drag(0.0, false, true) - 0.1).abs() < 1e-12);
        assert_eq!(bubble_column_drag(2.0, false, true), 1.8, "the launch's cap");
    }

    #[test]
    fn the_body_walks_the_column_drag_laws() {
        // THE WHIRLPOOL'S OWN SINK (the two body cells the standing box
        // overlaps both speak the pull): the double -0.03 the per-cell
        // walk folds, the fall distance the inside law resets
        let w = column_world(15294, 8, 11);
        let mut b = PlayerBody::new(8.0, 9.0, 8.0, 90.0, 0.0);
        b.fall_distance = 3.0;
        b.apply_bubble_column_drag(&w);
        let expected = (-0.03f64).max(-0.3);
        let expected = (expected - 0.03).max(-0.3);
        assert!(
            (b.vy - expected).abs() < 1e-12,
            "the two-cell pull: {} vs {}",
            b.vy,
            expected
        );
        assert_eq!(b.fall_distance, 0.0, "the inside law's reset");
        // THE UPWARD'S OWN LIFT (the same two-cell body): the double
        // +0.06 the walk folds
        let w = column_world(15295, 8, 11);
        let mut b = PlayerBody::new(8.0, 9.0, 8.0, 90.0, 0.0);
        b.fall_distance = 3.0;
        b.apply_bubble_column_drag(&w);
        let expected = (0.0f64 + 0.06).min(0.7);
        let expected = (expected + 0.06).min(0.7);
        assert!((b.vy - expected).abs() < 1e-12, "the two-cell lift: {}", b.vy);
        assert_eq!(b.fall_distance, 0.0);
        // THE SURFACE'S OWN LAUNCH (the top cell's above air — the
        // +0.1 the 1.8 cap keeps): the fall distance the SURFACE law
        // never resets (the launch keeps the fall's honest arc)
        let mut b = PlayerBody::new(8.0, 11.0, 8.0, 90.0, 0.0);
        b.fall_distance = 3.0;
        b.apply_bubble_column_drag(&w);
        assert!((b.vy - 0.1).abs() < 1e-12, "the launch's push: {}", b.vy);
        assert_eq!(b.fall_distance, 3.0, "the surface law never resets");
        // the whirlpool's mouth: the -0.9 floor the surface cell keeps
        let w = column_world(15294, 8, 11);
        let mut b = PlayerBody::new(8.0, 11.0, 8.0, 90.0, 0.0);
        b.vy = -1.0;
        b.apply_bubble_column_drag(&w);
        assert!((b.vy - (-0.9)).abs() < 1e-12, "the mouth's floor: {}", b.vy);
        // THE PLAIN WATER'S OWN SILENCE (variant 0): the drag never
        // speaks for the un-stamped cell
        let mut w = flat_world();
        w.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 1]);
        w.set(8, 9, 8, crate::world::WATER);
        let mut b = PlayerBody::new(8.0, 9.0, 8.0, 90.0, 0.0);
        b.apply_bubble_column_drag(&w);
        assert_eq!(b.vy, 0.0, "the plain water the drag skips");
        // THE STRICT EDGE (the inside walk's 1e-5 deflated box): the
        // box's max edge landing exactly on the integer EXCLUDES the
        // boundary cell — the whirlpool one cell east never speaks;
        // the 0.01 nudge east and the pull fires
        let w = column_world(15294, 8, 11);
        let mut b = PlayerBody::new(7.7, 9.0, 8.0, 90.0, 0.0);
        b.apply_bubble_column_drag(&w);
        assert_eq!(b.vy, 0.0, "the exact boundary cell the strict edge excludes");
        let mut b = PlayerBody::new(7.71, 9.0, 8.0, 90.0, 0.0);
        b.apply_bubble_column_drag(&w);
        assert!(
            (b.vy - (-0.06)).abs() < 1e-12,
            "the nudge the overlap arm answers: {}",
            b.vy
        );
    }

    #[test]
    fn the_body_drag_walks_both_vocabularies() {
        // THE NETWORKED VOCABULARY (the materialized WATER cells the
        // variants layer stamps): the same drags answer
        let mut w = flat_world();
        w.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 1]);
        for y in 8..=11 {
            w.set(8, y, 8, crate::world::WATER);
            w.variants.as_mut().expect("built above")[0]
                [8 + 8 * 16 + (y as usize) * 256] = 2;
        }
        let mut b = PlayerBody::new(8.0, 9.0, 8.0, 90.0, 0.0);
        b.apply_bubble_column_drag(&w);
        let expected = (0.0f64 + 0.06).min(0.7);
        let expected = (expected + 0.06).min(0.7);
        assert!(
            (b.vy - expected).abs() < 1e-12,
            "the materialized column's lift: {}",
            b.vy
        );
        // THE FULL TICK'S OWN RIDE (the walk sits at the effect beat —
        // after the move, before the drags/gravity): one tick in the
        // whirlpool folds the pull into the carried velocity the
        // gravity line then decays
        let w = column_world(15294, 8, 11);
        let mut b = PlayerBody::new(8.0, 9.0, 8.0, 90.0, 0.0);
        b.tick(&w, false);
        // the post-drag vy the gravity fold rides: (max(-0.3, -0.03)
        // twice) then (vy - 0.08) * 0.98
        let dragged = (-0.03f64).max(-0.3);
        let dragged = (dragged - 0.03).max(-0.3);
        let expected = (dragged - GRAVITY) * VERTICAL_DRAG as f64;
        assert!(
            (b.vy - expected).abs() < 1e-12,
            "the tick's carried velocity: {} vs {}",
            b.vy,
            expected
        );
    }

    /// A flat stone floor with the COBWEB cell the body stands in (the
    /// web's own y the feet cell keeps — the noCollision shape the
    /// body walks through, the stone the ground law keeps).
    fn webbed_world() -> World {
        let mut w = flat_world();
        w.set(8, 1, 8, crate::world::COBWEB);
        w
    }

    /// THE WEB BEAT'S OWN PAIR (javap WebBlock.entityInside pcs 0-70 —
    /// the clamp vecs the two arms keep, the makeStuckInBlock walk's
    /// resetFallDistance the banked descent clears): the standing body
    /// the web cell overlaps banks the (0.25, 0.05000000074505806,
    /// 0.25) pair VERBATIM (the widened float the dconst never
    /// rounds), the weaving verdict the (0.5, 0.25, 0.5) wider arm
    /// speaks, the web-less cell the slot never banks, the overlap
    /// the every-tick walk re-banks (the stationary body the box arm
    /// keeps).
    #[test]
    fn the_web_beat_banks_the_clamp_vecs_verbatim() {
        // THE BARE PAIR (the standing body the web cell overlaps): the
        // exact vec the beat banks, the banked descent the beat clears
        let w = webbed_world();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 90.0, 0.0);
        b.fall_distance = 3.0;
        assert!(b.touches_web(&w), "the standing box the web cell overlaps");
        b.tick(&w, false);
        assert_eq!(b.stuck_speed, (0.25, 0.05000000074505806, 0.25));
        // THE FALL RESET THE BEAT ALONE EXPLAINS (the fresh body the
        // first tick never grounds — the zero move the collision never
        // arms, the grounded reset arm the walk never fires): the
        // banked 3.0 the makeStuckInBlock walk's own tail clears
        assert!(!b.on_ground, "the fresh body the first tick never grounds");
        assert_eq!(
            b.fall_distance, 0.0,
            "the banked descent the beat's resetFallDistance clears"
        );
        // THE WEAVING WIDER ARM (the hasEffect verdict the flag the
        // App's store sync banks — the LivingEntity instanceof the
        // player body IS)
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 90.0, 0.0);
        b.weaving = true;
        b.tick(&w, false);
        assert_eq!(b.stuck_speed, (0.5, 0.25, 0.5), "the weave the wider arm wears");
        // THE WEB-LESS SILENCE (the flat stone never banks): the slot
        // the ctor's ZERO keeps
        let w = flat_world();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 90.0, 0.0);
        b.tick(&w, false);
        assert_eq!(b.stuck_speed, (0.0, 0.0, 0.0), "no web the slot never banks");
        // THE RE-BANK (the stationary body the box arm keeps): the
        // consumed slot the overlapping tick restores
        let w = webbed_world();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 90.0, 0.0);
        b.tick(&w, false); // the bank
        b.tick(&w, false); // the head consumed, the tail re-banked
        assert_eq!(
            b.stuck_speed,
            (0.25, 0.05000000074505806, 0.25),
            "the overlap the every-tick walk re-banks"
        );
    }

    /// THE STUCK WALK'S OWN LAW (javap Entity.move pcs 97-137): the
    /// slot's lengthSqr > 1.0E-7 gate the movement the ×clamp fold
    /// scales, the slot the ZERO reset, the deltaMovement the ZERO
    /// write clear — the drag the ZEROED fields the gravity alone
    /// re-arms, the consumed slot the NEXT tick's move never sees.
    #[test]
    fn the_stuck_move_scales_once_and_zeroes_the_fields() {
        let w = flat_world();
        let mut b = PlayerBody::new(8.0, 5.0, 8.0, 0.0, 0.0);
        // the banked slot the web beat's own vec wears, the descent
        // the pre-web fall armed
        b.stuck_speed = (0.25, 0.05000000074505806, 0.25);
        b.vy = -0.5;
        b.tick(&w, false);
        // the move the ×0.05 fold scales (the air lane no collision —
        // the scaled dy the collide hands through)
        let expected_y = 5.0 + (-0.5 * 0.05000000074505806);
        assert_eq!(b.y, expected_y, "the descent the clamp folds");
        // the slot the ZERO reset, the fields the ZERO write clear —
        // the gravity the post-move walk the zeroed vy alone re-arms
        assert_eq!(b.stuck_speed, (0.0, 0.0, 0.0), "the slot consumed");
        let expected_vy = (0.0f64 - GRAVITY) * VERTICAL_DRAG as f64;
        assert_eq!(b.vy, expected_vy, "the zeroed fields the gravity re-arms");
        assert_eq!(b.vx, 0.0);
        assert_eq!(b.vz, 0.0);
        // the ONE-SHOT (the consumed slot the next move never sees):
        // the second tick's descent the FULL vy speaks — the clamp
        // never folds it again
        let y_after_stuck = b.y;
        b.tick(&w, false);
        let expected_y2 = y_after_stuck + expected_vy;
        assert_eq!(b.y, expected_y2, "the next move the consumed slot never scales");
    }

    /// THE WEB CRAWL'S OWN RHYTHM (the e2e walk the vanilla speak):
    /// the stuck tick's velocity the input the tick itself arms (the
    /// zero-write the momentum never lets accumulate) — the per-tick
    /// crawl the impulse × 0.25 constant keeps, the weave the ×0.5
    /// wider crawl wears, the falling body the per-tick descent the
    /// (gravity × drag) × 0.05 constant keeps and the fall bank the
    /// beat's own reset never lets stack.
    #[test]
    fn the_web_crawl_the_rearmed_rhythm_the_fall_bank_denies() {
        // THE STEADY CRAWL (the settled body the web cell holds): the
        // control's first walk tick the raw impulse measures, the web
        // body's every tick the impulse × 0.25 constant wears
        let w_flat = flat_world();
        let mut control = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        for _ in 0..4 {
            control.tick(&w_flat, false);
        }
        control.forward = 1.0;
        control.tick(&w_flat, false);
        let impulse = control.z - 8.0;
        assert!(impulse > 0.0, "the raw impulse the control measures: {}", impulse);
        let w = webbed_world();
        let mut b = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        for _ in 0..4 {
            b.tick(&w, false);
        }
        b.forward = 1.0;
        let zs: Vec<f64> = (0..4)
            .map(|_| {
                b.tick(&w, false);
                b.z
            })
            .collect();
        let dz = |prev: f64, cur: f64| cur - prev;
        let d1 = dz(8.0, zs[0]);
        let d2 = dz(zs[0], zs[1]);
        let d3 = dz(zs[1], zs[2]);
        let d4 = dz(zs[2], zs[3]);
        assert_eq!(d1, impulse * 0.25, "the crawl the clamp keeps");
        assert_eq!(d1, d2, "the re-armed rhythm the constant keeps");
        assert_eq!(d2, d3, "the third tick the same impulse speaks");
        assert_eq!(d3, d4, "the fourth tick the same");
        // THE WEAVE'S WIDER CRAWL (the ×0.5 arm the wider vec keeps)
        let mut wb = PlayerBody::new(8.0, 1.0, 8.0, 0.0, 0.0);
        wb.weaving = true;
        for _ in 0..4 {
            wb.tick(&w, false);
        }
        wb.forward = 1.0;
        wb.tick(&w, false); // the narrow-vec warmup the settle banked
        let z_warm = wb.z;
        wb.tick(&w, false);
        let dz_wide = wb.z - z_warm;
        assert_eq!(dz_wide, impulse * 0.5, "the weave the wider crawl wears");
        // THE FALLING BODY'S OWN CONSTANTS (the web column the descent
        // crawls): the per-tick dy the (gravity × drag) × 0.05
        // constant keeps from the second stuck tick, the fall bank the
        // beat's reset clears (the control's free fall the bank stacks)
        let mut wcol = flat_world();
        for y in 1..=6 {
            wcol.set(8, y, 8, crate::world::COBWEB);
        }
        let mut fb = PlayerBody::new(8.0, 5.0, 8.0, 0.0, 0.0);
        let mut dys: Vec<f64> = Vec::new();
        let mut prev_y = fb.y;
        for _ in 0..6 {
            fb.tick(&wcol, false);
            dys.push(prev_y - fb.y);
            prev_y = fb.y;
        }
        let expected_fall =
            -(((0.0f64 - GRAVITY) * VERTICAL_DRAG as f64) * 0.05000000074505806);
        assert!(dys[0] < expected_fall, "the entry tick the fresh slot never scales");
        // the position subtraction's own ulp (the y the sum rounding
        // carries — the 8.9e-16 ulp at 5.0 the noise keeps): the
        // constant descent the 1e-15 band the arithmetic speaks
        assert!((dys[3] - expected_fall).abs() < 1e-15, "the third tick the constant descent: {} vs {}", dys[3], expected_fall);
        assert!((dys[4] - dys[3]).abs() < 1e-15, "the fourth the same");
        assert!((dys[5] - dys[3]).abs() < 1e-15, "the fifth the same");
        assert!(
            fb.fall_distance < 0.01,
            "the fall bank the beat's reset clears: {}",
            fb.fall_distance
        );
        // the control's free fall the bank stacks (the same spawn, the
        // web-less column the beat never speaks)
        let mut cb = PlayerBody::new(8.0, 5.0, 8.0, 0.0, 0.0);
        for _ in 0..6 {
            cb.tick(&w_flat, false);
        }
        assert!(
            cb.fall_distance > 0.1,
            "the free fall the bank stacks: {}",
            cb.fall_distance
        );
    }

    /// THE LANDING HURT THE WEB DENIES (the e2e fall the vanilla's own
    /// mercy speaks): the seven-block descent the web cell catches —
    /// the banked descent the beat's resetFallDistance clears and the
    /// crawl the landing dy shrinks — the fall event the pipeline
    /// never sees; the web-less control the SAME spawn the hurt the
    /// landing arms.
    #[test]
    fn the_web_fall_the_landing_hurt_denies() {
        let w = webbed_world();
        let mut b = PlayerBody::new(8.0, 8.0, 8.0, 0.0, 0.0);
        let mut saw_event = false;
        for _ in 0..300 {
            b.tick(&w, false);
            if b.fall_event.is_some() {
                saw_event = true;
                b.fall_event = None;
            }
            if b.on_ground {
                break;
            }
        }
        assert!(b.on_ground, "the caught body the floor lands");
        assert!(
            !saw_event,
            "the web the fall event denies (the bank the beat clears)"
        );
        // THE CONTROL'S OWN HURT (the web-less column the same spawn):
        // the landing the event arms
        let w = flat_world();
        let mut c = PlayerBody::new(8.0, 8.0, 8.0, 0.0, 0.0);
        let mut event = None;
        for _ in 0..60 {
            c.tick(&w, false);
            if c.fall_event.is_some() {
                event = c.fall_event.take();
                break;
            }
        }
        assert!(
            matches!(event, Some(d) if (3..=5).contains(&d)),
            "the free fall the landing hurt arms: {:?}",
            event
        );
    }

    /// THE SOUL SAND'S OWN PATCH (the autopush-598 round — the y=0 row
    /// the sand replaces, the patch the walk stands on: the resting
    /// body the 14/16 column top wears, the feet cell the SAND cell
    /// itself speaks — the floor(y) of the 0.875 rest the blockPosition
    /// read answers).
    fn soul_sanded_world() -> World {
        let mut w = flat_world();
        for x in 6..10 {
            for z in 6..10 {
                w.set(x, 0, z, crate::world::SOUL_SAND);
            }
        }
        w
    }

    /// THE BLOCK SPEED FACTOR'S OWN LANE (javap Entity.getBlockSpeedFactor
    /// pcs 0-70 + Entity.move tail pcs 652-672 the 26.2 jar): the
    /// layered read the standing body answers — the sand-resting body
    /// the FEET cell's 0.4 speaks (the floor(0.875) the sand cell is),
    /// the stone-resting body the air feet the BELOW cell falls through
    /// (getBlockPosBelowThatAffectsMyMovement — the getOnPos(0.500001f)
    /// fold), the 0.4f32 the widened f64 the move tail multiplies —
    /// and the e2e walk the x/z ONLY fold wears (the vy the dconst_1
    /// multiply keeps), the drag the same single-tick order after.
    #[test]
    fn the_soul_sand_the_below_cell_the_walk_slows() {
        let w = soul_sanded_world();
        // THE SAND REST (the body the column top settles — the feet the
        // sand cell itself wears, the factor the feet arm banks)
        let mut b = PlayerBody::new(8.5, 1.0, 8.5, 0.0, 0.0);
        for _ in 0..5 {
            b.tick(&w, false);
        }
        assert!(b.on_ground, "the sand rest the column top grounds");
        assert_eq!(
            b.block_speed_factor(&w),
            0.4,
            "the resting body the feet cell's own 0.4 wears"
        );
        // THE STONE CONTROL (the flat world the same procedure — the
        // air feet the below cell falls through, the no-row default
        // the 1.0 keeps)
        let flat = flat_world();
        let mut c = PlayerBody::new(8.5, 1.0, 8.5, 0.0, 0.0);
        for _ in 0..5 {
            c.tick(&flat, false);
        }
        assert!(c.on_ground, "the stone rest the full top grounds");
        assert_eq!(
            c.block_speed_factor(&flat),
            1.0,
            "the stone below the default keeps"
        );
        // THE AIR-FEET LAYERED READ (the body the cube top floats —
        // y 1.0 the feet cell the air above speaks, the below cell the
        // sand banks: the getOnPos fold the standing-on-full-cube case
        // answers)
        assert_eq!(
            PlayerBody::new(8.5, 1.0, 8.5, 0.0, 0.0).block_speed_factor(&w),
            0.4,
            "the air feet the sand below the layered walk banks"
        );
        // THE E2E WALK (the re-armed vx the tick's own law folds — the
        // factor lane the x/z ONLY scales, the drag the same tick
        // after): the exact f64 chain the same fold order computes —
        // the settled y the pair keeps (both grounded, the SAME drag
        // lane the fold shares)
        let drag = (BLOCK_FRICTION * GROUND_DRAG) as f64;
        b.vx = 1.0;
        b.x = 8.5;
        b.tick(&w, false);
        let expected_sand = (1.0f64 * (0.4f32 as f64)) * drag;
        assert_eq!(
            b.vx, expected_sand,
            "the sand walk the factor × drag the bit-exact chain keeps"
        );
        c.vx = 1.0;
        c.x = 8.5;
        c.tick(&flat, false);
        let expected_stone = 1.0f64 * drag;
        assert_eq!(
            c.vx, expected_stone,
            "the stone walk the factor-less chain keeps"
        );
        // THE VERTICAL THE MULTIPLY NEVER TOUCHES (the dconst_1 y the
        // vanilla multiply keeps — both grounded, the gravity the same
        // pair re-arms, the x/z the factor alone splits)
        assert_eq!(
            b.vy, c.vy,
            "the y the factor lane never folds — the gravity the same pair re-arms"
        );
    }

    /// THE WATER FEET'S OWN PASS-THROUGH (javap getBlockSpeedFactor pcs
    /// 20-41 the 26.2 jar — the WATER cell the arm keeps: the below
    /// cell the in-water body NEVER falls through, the pond above the
    /// sand the swim never slows — the BUBBLE_COLUMN kind the palette
    /// never carried, the arm's second disjunct the silence wears).
    #[test]
    fn the_water_feet_the_below_fallback_denies() {
        let mut w = soul_sanded_world();
        w.set(8, 1, 8, crate::world::WATER);
        // THE SUBMERGED READ (the feet cell the water speaks — the
        // pass-through arm the 1.0 keeps, the sand below the walk never
        // reaches)
        let b = PlayerBody::new(8.5, 1.0, 8.5, 0.0, 0.0);
        assert_eq!(
            b.block_speed_factor(&w),
            1.0,
            "the water feet the pass-through arm keeps — the sand below the fallback denies"
        );
        // THE AIR-FEET CONTROL (the same pond's edge the air feet wear —
        // the layered walk the sand below banks)
        let mut w2 = soul_sanded_world();
        w2.set(8, 2, 8, crate::world::WATER);
        let c = PlayerBody::new(8.5, 1.0, 8.5, 0.0, 0.0);
        assert_eq!(
            c.block_speed_factor(&w2),
            0.4,
            "the air feet the water above never banks — the sand below the layered walk keeps"
        );
    }

    /// THE BLOCK JUMP FACTOR'S OWN BANK (javap Entity.getBlockJumpFactor
    /// pcs 0-48 + getJumpPower pcs 0-19 the 26.2 jar): the banked 1.0
    /// the sand's no-row default keeps (the getJumpPower fold the
    /// JUMP_STRENGTH value the exact 0.42 wears), the honey wake's 0.5
    /// the banked fold the `(0.42 × 0.5) + boost` f32 chain the f2d
    /// widen keeps — the no-row sand the same law the bit-exact 0.42
    /// preserves.
    #[test]
    fn the_jump_factor_the_bank_the_power_wears() {
        let w = soul_sanded_world();
        let mut b = PlayerBody::new(8.5, 1.0, 8.5, 0.0, 0.0);
        for _ in 0..5 {
            b.tick(&w, false);
        }
        // THE BANKED DEFAULT (the sand the no-row 1.0 the tick's own
        // walk banks — the jump the exact 0.42 the bit-exact law keeps)
        assert_eq!(
            b.block_jump_factor, 1.0,
            "the sand the no-row default banks"
        );
        b.vy = 0.0;
        b.jump_from_ground();
        assert_eq!(
            b.vy,
            ((JUMP_POWER * 1.0f32) + 0.0f32) as f64,
            "the 1.0 fold the exact 0.42 preserves"
        );
        // THE HONEY WAKE'S OWN VALUE (the banked 0.5 the table's row
        // banks — the fold the ×0.5 chain wears: the f32 multiply the
        // f2d widen keeps)
        b.block_jump_factor = 0.5;
        b.vy = 0.0;
        b.jump_from_ground();
        assert_eq!(
            b.vy,
            ((JUMP_POWER * 0.5f32) + 0.0f32) as f64,
            "the honey fold the ×0.5 chain keeps"
        );
    }
}

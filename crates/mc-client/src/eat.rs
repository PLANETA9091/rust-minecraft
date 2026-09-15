//! THE EAT ARC — the food items' use lane (javap, the cached jar's
//! Mojang-mapped classes): `Consumable`'s own component law, the chew
//! cadence, the completion chain, the first-person pose, and the item
//! piece particles' physics — every number the bytecode speaks.
//!
//! The wire's shape (javap verbatim):
//! - `Consumable.canConsume`: the FOOD component present && the entity a
//!   player -> `player.canEat(food.canAlwaysEat())` (the Player's own
//!   law: `abilities.invulnerable || ignoreHunger || foodData.needsFood()`);
//!   a consumable WITHOUT food (the potion) consumes unconditionally.
//! - `Consumable.startConsuming`: `consumeTicks() > 0` -> `startUsingItem`
//!   (the hold lane); zero -> the instant `onConsume`.
//! - `LivingEntity.updateUsingItem` (every tick of the hold):
//!   `ItemStack.onUseTick(level, entity, remaining)` -> the CONSUMABLE's
//!   `shouldEmitParticlesAndSounds(remaining)` -> the 5-piece chew burst,
//!   THEN `useItemRemaining--`, and at zero the `completeUsingItem`.
//! - `Consumable.shouldEmitParticlesAndSounds(remaining)`: `remaining % 4
//!   == 0 && consumeTicks() - remaining > (int)(consumeTicks() *
//!   0.21875f)` — the apple's 32 ticks speak the burst window
//!   `remaining in {24, 20, 16, 12, 8, 4}` (six chews, the last quarter
//!   silent — the swallow).
//! - `Consumable.emitParticlesAndSounds(random, entity, stack, count)`:
//!   the volume `nextBoolean() ? 0.5 : 1.0` (the DRINK folds the fixed
//!   0.5), the EAT pitch `triangle(1.0, 0.2)`, the DRINK pitch
//!   `randomBetween(0.9, 1.0)` — ALL FOUR draws happen unconditionally
//!   (the bytecode's own order: the boolean, the two triangle floats,
//!   the randomBetween float) — then `spawnItemParticles(stack, count)`
//!   (only when `has_consume_particles`), then the consumable's sound.
//! - `LivingEntity.completeUsingItem` -> `ItemStack.finishUsingItem` ->
//!   `Consumable.onConsume`: the FINAL 16-piece burst (the same sound
//!   law), the `ConsumableListener` stream (the FOOD's
//!   `FoodProperties.onConsume`: the fixed-volume gulp at volume 1.0 /
//!   pitch `triangle(1.0, 0.4)` — then `FoodData.eat(props)` for the
//!   player — then THE BURP: `PLAYER_BURP` at volume 0.5, pitch
//!   `randomBetween(0.9, 1.0)`), the server-side effect list, the
//!   EAT/DRINK game event, and `stack.consume(1, entity)` (the
//!   survival's own decrement; the infinite-materials gate the client's
//!   creative stand-in rides).
//! - `LivingEntity.spawnItemParticles(stack, count)`: per piece the
//!   offset `Vec3((r-0.5)*0.1, r*0.1+0.1, 0)` and the velocity
//!   `Vec3((r-0.5)*0.3, -r*0.6-0.3, 0.6)`, both rotated by
//!   `xRot(-pitch*PI/180)` then `yRot(-yaw*PI/180)`, spawned at the
//!   EYE position + offset with the velocity — the pieces fly INTO
//!   the mouth.
//! - The piece's own physics (javap Particle + BreakingItemParticle):
//!   the parent constructor folds `xd = vx + (r*2-1)*0.4` then the
//!   normalize step `xd = xd/len * d0 * 0.4 + xd * 0.2` (d0 = (r + r +
//!   1) * 0.15), BreakingItemParticle multiplies by 0.1 and adds the
//!   raw velocity back (`xd = xd * 0.1 + vx`) — the piece rides the
//!   mouthward velocity with a whisper of jitter. The tick: gravity 0
//!   (BreakingItemParticle never sets it), the move, the 0.98 friction;
//!   the lifetime `(int)(4.0 / (r * 0.9 + 1.0 * 0.1))` ticks.
//! - `ItemInHandRenderer.applyEatTransform` (the first-person pose):
//!   `f = remaining - partial + 1`, `f1 = f / duration`; while
//!   `f1 < 0.8` the bite bob `translate(0, |cos(f / 4 * PI) * 0.1|,
//!   0)` — a bite every 4 ticks; then `f3 = 1 - pow(f1, 27)` and the
//!   pull-in `translate(f3 * 0.6 * side, f3 * -0.5, 0)` with the
//!   rotations `YP(side * f3 * 90)`, `XP(f3 * 10)`, `ZP(side * f3 *
//!   30)`.

use mc_registry::generated::foods::{FoodDef, DEFAULT_CONSUME_SECONDS};

/// `Consumable`'s own chew-burst piece count (javap ItemStack.onUseTick:
/// `emitParticlesAndSounds(random, entity, stack, 5)`).
pub const CHEW_BURST_COUNT: u32 = 5;
/// The completion's own burst (javap Consumable.onConsume: `..., 16`).
pub const FINISH_BURST_COUNT: u32 = 16;
/// The chew cadence's own stride (`shouldEmitParticlesAndSounds`:
/// `arg % 4 == 0`).
pub const CHEW_STRIDE: u32 = 4;
/// `CONSUME_EFFECTS_START_FRACTION`'s inlined constant (javap: `0.21875f`
/// — the last quarter-ish of the hold stays silent, the swallow).
pub const START_FRACTION: f32 = 0.21875;

/// `RandomSource.triangle(mode, deviation)` (javap default method:
/// `mode + (nextFloat() - nextFloat()) * deviation`).
pub fn triangle(r1: f32, r2: f32, mode: f32, deviation: f32) -> f32 {
    mode + (r1 - r2) * deviation
}

/// `Mth.randomBetween(random, min, max)` (javap:
/// `nextFloat() * (max - min) + min`).
pub fn random_between(r: f32, min: f32, max: f32) -> f32 {
    r * (max - min) + min
}

/// `Consumable.shouldEmitParticlesAndSounds(remaining)` — javap
/// verbatim: `consumeTicks() - remaining > (int)(consumeTicks() *
/// 0.21875f) && remaining % 4 == 0`. The arg is the REMAINING tick
/// count the `updateUsingItem` passes (the decrement speaks after the
/// burst).
pub fn should_emit(consume_ticks: u32, remaining: u32) -> bool {
    let start = (consume_ticks as f32 * START_FRACTION) as u32;
    consume_ticks.saturating_sub(remaining) > start && remaining % CHEW_STRIDE == 0
}

/// The eat hold's total tick count — `Item.getUseDuration`'s own
/// CONSUMABLE branch (`(int)(consumeSeconds * 20f)`).
pub fn consume_ticks(def: &FoodDef) -> u32 {
    def.consume_ticks() as u32
}

/// One chew burst's acoustic fold (javap emitParticlesAndSounds): the
/// volume `nextBoolean() ? 0.5 : 1.0` (the DRINK folds the fixed 0.5 —
/// the boolean still draws, the bytecode computes it unconditionally),
/// the EAT pitch `triangle(1.0, 0.2)`, the DRINK pitch
/// `randomBetween(0.9, 1.0)` (drawn unconditionally too). Returns the
/// (volume, pitch) and the three float draws' honest count — the
/// caller feeds THIS engine's LCG in the bytecode's own order: the
/// boolean's draw, the two triangle floats, the randomBetween float.
pub fn chew_voice(animation: Anim, r_bool: bool, r1: f32, r2: f32, r3: f32) -> (f32, f32) {
    let base = if r_bool { 0.5 } else { 1.0 };
    let eat_pitch = triangle(r1, r2, 1.0, 0.2);
    let drink_pitch = random_between(r3, 0.9, 1.0);
    match animation {
        Anim::Drink => (0.5, drink_pitch),
        Anim::Eat => (base, eat_pitch),
    }
}

/// The completion's own voices (javap Consumable.onConsume then
/// FoodProperties.onConsume — the FOOD listener's lane): the final
/// 16-piece burst rides the chew fold, THEN the gulp at the fixed
/// volume 1.0 / pitch `triangle(1.0, 0.4)`, THEN the burp at volume
/// 0.5 / pitch `randomBetween(0.9, 1.0)`. The draw order the bytecode
/// speaks: the burst's four, the gulp's two, the burp's one.
pub const FINISH_GULP_VOLUME: f32 = 1.0;
pub const FINISH_GULP_DEVIATION: f32 = 0.4;
pub const BURP_VOLUME: f32 = 0.5;
pub const BURP_PITCH_MIN: f32 = 0.9;
pub const BURP_PITCH_MAX: f32 = 1.0;

/// `LivingEntity.spawnItemParticles`'s own geometry — one piece's
/// spawn point offset and initial velocity, computed verbatim from the
/// eye position and the view angles (degrees). The offset
/// `Vec3((r-0.5)*0.1, r*0.1+0.1, 0)` and the velocity
/// `Vec3((r-0.5)*0.3, -r*0.6-0.3, 0.6)`, both folded through
/// `xRot(-pitch * PI/180)` then `yRot(-yaw * PI/180)`.
///
/// The rotation here mirrors `Vec3.xRot`/`Vec3.yRot` (javap):
/// `xRot(a)`: (x, y*cos - z*sin, y*sin + z*cos) for the angle
/// `-pitch_rad` carried in; `yRot(a)`: (x*cos + z*sin, y,
/// z*cos - x*sin).
pub fn piece_offset_and_velocity(
    r_off_x: f32,
    r_off_y: f32,
    r_vel_x: f32,
    r_vel_y: f32,
    yaw_deg: f32,
    pitch_deg: f32,
) -> ([f64; 3], [f64; 3]) {
    let rad = std::f32::consts::PI / 180.0;
    // the offset vec, pre-rotation
    let mut off = [
        ((r_off_x - 0.5) * 0.1) as f64,
        (r_off_y * 0.1 + 0.1) as f64,
        0.0,
    ];
    // the velocity vec, pre-rotation (the d0 = -r*0.6 - 0.3 fold)
    let d0 = (-(r_vel_y as f64)) * 0.6 - 0.3;
    let mut vel = [
        ((r_vel_x - 0.5) * 0.3) as f64,
        d0,
        0.6,
    ];
    // xRot(-pitch * PI/180): y' = y*cos(a) - z*sin(a), z' = y*sin(a) +
    // z*cos(a) with a = the passed angle (Vec3.xRot's own convention)
    let rot_x = |v: &mut [f64; 3], a: f64| {
        let (s, c) = a.sin_cos();
        let y = v[1] * c - v[2] * s;
        let z = v[1] * s + v[2] * c;
        v[1] = y;
        v[2] = z;
    };
    // yRot(a): x' = x*cos(a) + z*sin(a), z' = z*cos(a) - x*sin(a)
    let rot_y = |v: &mut [f64; 3], a: f64| {
        let (s, c) = a.sin_cos();
        let x = v[0] * c + v[2] * s;
        let z = v[2] * c - v[0] * s;
        v[0] = x;
        v[2] = z;
    };
    let pa = (-pitch_deg * rad) as f64;
    let ya = (-yaw_deg * rad) as f64;
    rot_x(&mut off, pa);
    rot_y(&mut off, ya);
    rot_x(&mut vel, pa);
    rot_y(&mut vel, ya);
    (off, vel)
}

/// The `Particle(ClientLevel, x, y, z, vx, vy, vz)` parent
/// constructor's velocity fold (javap): `xd = vx + (nextFloat()*2 - 1)
/// * 0.4` per axis, then the normalize step `xd = xd/len * d0 * 0.4 +
/// xd * 0.2` with `d0 = (nextFloat() + nextFloat() + 1.0) * 0.15`.
/// The draw order: two floats per axis (x, y, z), then d0's two.
pub fn parent_velocity(
    vx: f32,
    vy: f32,
    vz: f32,
    jx: f32,
    jy: f32,
    jz: f32,
    r1: f32,
    r2: f32,
) -> [f64; 3] {
    // per-axis jitter fold: vx + (r*2 - 1) * 0.4
    let jit = |v: f32, r: f32| (v as f64) + (((r * 2.0 - 1.0) * 0.4) as f64);
    let mut x = jit(vx, jx);
    let mut y = jit(vy, jy);
    let mut z = jit(vz, jz);
    // d0 = (r + r + 1) * 0.15 (the f32 adds then the f64 product —
    // the bytecode's fadd/ldc 0.15/fmul chain)
    let d0 = (((r1 + r2 + 1.0) * 0.15) as f64) as f64;
    let len = (x * x + y * y + z * z).sqrt();
    if len > 0.0 {
        x = x / len * d0 * 0.4 + x * 0.2;
        y = y / len * d0 * 0.4 + y * 0.2;
        z = z / len * d0 * 0.4 + z * 0.2;
    }
    [x, y, z]
}

/// `BreakingItemParticle`'s own fold (javap): `xd *= 0.1` per axis,
/// then `xd += xSpeed` (the RAW param) — the piece rides the
/// mouthward velocity with a whisper of the parent's jitter.
pub fn breaking_item_velocity(parent: [f64; 3], vx: f32, vy: f32, vz: f32) -> [f64; 3] {
    [
        parent[0] * 0.1 + vx as f64,
        parent[1] * 0.1 + vy as f64,
        parent[2] * 0.1 + vz as f64,
    ]
}

/// The default particle lifetime (javap Particle's 4-arg constructor
/// tail: `(int)(4.0 / (nextFloat() * 0.9 + 0.1))` — [5, 40] ticks).
pub fn particle_lifetime(r: f32) -> u32 {
    (4.0f32 / (r * 0.9 + 0.1)) as u32
}

/// One item piece — the eat arc's own particle (the
/// `BreakingItemParticle` the CLIENT lane speaks).
#[derive(Debug, Clone, PartialEq)]
pub struct ItemPiece {
    /// The spawn origin (the eye + the rotated offset).
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub age: u32,
    pub lifetime: u32,
    /// The held item's registry id (the texture the piece samples).
    pub item: i32,
    /// The 4x4-pixel texture slice's top-left (the uo/vo law:
    /// `nextFloat() * 3.0` each — the piece shows a random corner of
    /// the sprite).
    pub uo: f32,
    pub vo: f32,
}

impl ItemPiece {
    /// The tick (javap Particle.tick): the age, the remove at the
    /// lifetime, the gravity lane (0.04 * gravity — the item pieces
    /// never set gravity), the move, the 0.98 friction, the
    /// onGround's 0.7 lane (the honest absence: the client's pieces
    /// carry no collision — the mouthward flight outlives the
    /// lifetime first).
    pub fn tick(&mut self) -> bool {
        self.age += 1;
        if self.age >= self.lifetime {
            return false;
        }
        let friction = 0.98f64;
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
        self.vx *= friction;
        self.vy *= friction;
        self.vz *= friction;
        true
    }
}

/// The first-person eat pose (javap `applyEatTransform` verbatim) —
/// the translation and the three rotations the held item's matrix
/// folds AFTER the vanilla anchor. `remaining` counts DOWN from
/// `total`; `partial` is the frame's own fraction (the client's tick
/// == frame model speaks 0.5, the swing clock's own convention).
/// `side` is +1 for the right hand, -1 for the left.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EatPose {
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
    pub rot_y_deg: f32,
    pub rot_x_deg: f32,
    pub rot_z_deg: f32,
}

pub fn apply_eat_transform(remaining: u32, partial: f32, total: u32, side: f32) -> EatPose {
    // f = remaining - partial + 1 (the hold's own 1-tick head)
    let f = remaining as f32 - partial + 1.0;
    // f1 = f / duration (the remaining fraction)
    let f1 = f / total as f32;
    // the pull-in curve: 1 - pow(f1, 27)
    let f3 = 1.0 - (f1 as f64).powi(27) as f32;
    // the bite bob while the fraction is under 0.8:
    // |cos(f / 4 * PI)| * 0.1 — a bite every 4 ticks
    let mut dy = f3 * -0.5;
    if f1 < 0.8 {
        let bob = (((f / 4.0 * std::f32::consts::PI) as f64).cos() * 0.1).abs() as f32;
        dy += bob;
    }
    EatPose {
        dx: f3 * 0.6 * side,
        dy,
        dz: f3 * 0.0,
        rot_y_deg: side * f3 * 90.0,
        rot_x_deg: f3 * 10.0,
        rot_z_deg: side * f3 * 30.0,
    }
}

/// THE EAT LANE — one consumable use's own state (the vanilla's
/// `useItem` + `useItemRemaining` narrowed to the food's hold).
#[derive(Debug, Clone, PartialEq)]
pub struct EatActive {
    pub item: i32,
    /// The ticks elapsed (the client's use clock speaks it — the
    /// vanilla's `useItemRemaining` counts down from `total`).
    pub ticks: u32,
    pub total: u32,
    /// The row's snapshot fields the completion folds (the registry's
    /// statics — copied ids, no lifetimes).
    pub animation: Anim,
    pub sound: &'static str,
    pub has_consume_particles: bool,
    pub nutrition: i32,
    pub saturation: f32,
    pub can_always_eat: bool,
    pub use_remainder: Option<&'static str>,
}

/// The animation the pose/sound lane reads (the registry enum's
/// lifetimes-free twin).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anim {
    Eat,
    Drink,
}

impl EatActive {
    /// Whether the FOOD component rode the row (the completion's own
    /// split: the potion never speaks the FoodData fold or the burp).
    pub fn has_food(&self) -> bool {
        self.nutrition >= 0
    }

    /// `Consumable.startConsuming`'s own build: the hold lane opens
    /// when `consumeTicks() > 0` (a zero-tick consumable completes
    /// instantly — none rides the 26.2 table, the guard stays).
    pub fn start(item: i32, def: &FoodDef) -> Option<EatActive> {
        let total = consume_ticks(def);
        if total == 0 {
            return None;
        }
        Some(EatActive {
            item,
            ticks: 0,
            total,
            animation: match def.animation {
                mc_registry::generated::foods::ConsumableAnim::Eat => Anim::Eat,
                mc_registry::generated::foods::ConsumableAnim::Drink => Anim::Drink,
            },
            sound: def.sound,
            has_consume_particles: def.has_consume_particles,
            nutrition: def.nutrition,
            saturation: def.saturation,
            can_always_eat: def.can_always_eat,
            use_remainder: def.use_remainder,
        })
    }

    /// The REMAINING ticks (the vanilla `useItemRemaining` the cadence
    /// and the pose read).
    pub fn remaining(&self) -> u32 {
        self.total.saturating_sub(self.ticks)
    }

    /// The cadence's own verdict for THIS tick's burst (the caller
    /// passes the remaining count BEFORE the decrement — the
    /// updateUsingItem's own order: the burst, then the fold).
    pub fn burst_due(&self) -> bool {
        should_emit(self.total, self.remaining())
    }

    /// The completion's own gate: the remaining hit zero.
    pub fn complete_due(&self) -> bool {
        self.remaining() == 0
    }
}

/// `Player.canEat(canAlwaysEat)` (javap verbatim):
/// `abilities.invulnerable || ignoreHunger || foodData.needsFood()`.
/// The creative flag rides the caller (the client's survival lane
/// speaks false).
pub fn can_eat(can_always_eat: bool, invulnerable: bool, needs_food: bool) -> bool {
    invulnerable || can_always_eat || needs_food
}

/// The default consumable seconds (the table's fallback the codec
/// speaks).
pub const DEFAULT_SECONDS: f32 = DEFAULT_CONSUME_SECONDS;

#[cfg(test)]
mod tests {
    use super::*;

    fn apple() -> FoodDef {
        *mc_registry::generated::foods::food_def("minecraft:apple").unwrap()
    }
    fn honey() -> FoodDef {
        *mc_registry::generated::foods::food_def("minecraft:honey_bottle").unwrap()
    }
    fn milk() -> FoodDef {
        *mc_registry::generated::foods::food_def("minecraft:milk_bucket").unwrap()
    }
    fn gapple() -> FoodDef {
        *mc_registry::generated::foods::food_def("minecraft:golden_apple").unwrap()
    }

    /// THE HOLD LENGTH: `(int)(consumeSeconds * 20)` — the apple's
    /// 1.6s speaks 32, the honey's 2.0s speaks 40.
    #[test]
    fn the_hold_length_speaks_the_seconds() {
        assert_eq!(consume_ticks(&apple()), 32);
        assert_eq!(consume_ticks(&honey()), 40);
        assert_eq!(consume_ticks(&milk()), 32);
    }

    /// THE CADENCE TABLE (javap shouldEmitParticlesAndSounds over the
    /// apple's 32): the bursts ride `remaining in {24, 20, 16, 12, 8,
    /// 4}` — six chews; the first quarter (remaining 28: elapsed 4)
    /// stays silent (the window's own start), the last quarter (the
    /// swallow) too, and the odd remainders never speak. The
    /// remaining 0 is the bytecode's own true but unreachable corner —
    /// the completion intercepts at the decrement's own head.
    #[test]
    fn the_chew_window_speaks_the_swallow() {
        let t = 32u32;
        for rem in 0..=32 {
            let due = should_emit(t, rem);
            let expected = (t - rem) > (t as f32 * 0.21875) as u32 && rem % 4 == 0;
            assert_eq!(due, expected, "remaining {rem}");
        }
        for rem in [24u32, 20, 16, 12, 8, 4] {
            assert!(should_emit(t, rem), "remaining {rem} must burst");
        }
        assert!(!should_emit(t, 28), "the first quarter is silent");
        assert!(!should_emit(t, 31), "the odd remaining never speaks");
        assert!(!should_emit(t, 32), "the hold's head never speaks");
        // the pure law's own corner: the bytecode's arithmetic returns
        // true at remaining 0 (the subtraction saturates past the
        // window) — the REAL flow never passes it (the completion
        // intercepts at the decrement's own head), the caller's gate
        // reads complete_due() first
        // the honey's 40: the window's start = (int)(40 * 0.21875) = 8 —
        // the elapsed must EXCEED it, so the first stride (remaining 36,
        // elapsed 4) is silent too; bursts at 28..4 — seven chews
        let t = 40u32;
        for rem in [28u32, 24, 20, 16, 12, 8, 4] {
            assert!(should_emit(t, rem), "honey remaining {rem} must burst");
        }
        assert!(!should_emit(t, 36), "the honey's first stride is silent");
        assert!(!should_emit(t, 32), "the honey's second stride is silent");
    }

    /// THE CHEW FOLD (javap emitParticlesAndSounds): the EAT's volume
    /// rides the boolean (0.5/1.0), the pitch the ±0.2 triangle; the
    /// DRINK folds the fixed 0.5 and the [0.9, 1.0] pitch — all four
    /// draws land regardless of the animation.
    #[test]
    fn the_chew_voice_folds_the_bytecode_law() {
        let a = Anim::Eat;
        let (v, p) = chew_voice(a, true, 0.5, 0.5, 0.7);
        assert_eq!(v, 0.5, "the boolean's 0.5 lane");
        assert!((p - 1.0).abs() < 1e-6, "the triangle's midpoint speaks 1.0");
        let (v, p) = chew_voice(a, false, 0.0, 0.5, 0.7);
        assert_eq!(v, 1.0, "the boolean's 1.0 lane");
        assert!((p - 0.9).abs() < 1e-6, "the triangle's (0.0 - 0.5) * 0.2 fold");
        let m = Anim::Drink;
        let (v, p) = chew_voice(m, true, 0.5, 0.5, 0.7);
        assert_eq!(v, 0.5, "the DRINK's fixed volume");
        assert!((p - (0.9 + 0.7 * 0.1)).abs() < 1e-6, "the DRINK's randomBetween pitch");
        // the triangle's own shape: mode + (r1 - r2) * dev
        assert!((triangle(0.25, 0.75, 1.0, 0.2) - 0.9).abs() < 1e-6);
        assert!((random_between(0.3, 0.9, 1.0) - 0.93).abs() < 1e-6);
    }

    /// THE COMPLETION'S OWN PITCH LAWS: the gulp's ±0.4 triangle at
    /// the fixed volume 1.0, the burp's [0.9, 1.0] at volume 0.5.
    #[test]
    fn the_completion_voices_speak_their_constants() {
        assert_eq!(FINISH_GULP_VOLUME, 1.0);
        assert_eq!(FINISH_GULP_DEVIATION, 0.4);
        assert_eq!(BURP_VOLUME, 0.5);
        // the burp's pitch range: 0.9..1.0
        for r in [0.0f32, 0.25, 0.5, 1.0] {
            let p = random_between(r, BURP_PITCH_MIN, BURP_PITCH_MAX);
            assert!((0.9..=1.0).contains(&p), "the burp pitch rides the band: {p}");
        }
    }

    /// THE PIECE GEOMETRY (javap spawnItemParticles, the straight-on
    /// view yaw=0/pitch=0): the offset's Z stays 0 and the velocity's
    /// Z stays 0.6 — the piece spawns a whisper off the eye and flies
    /// INTO the mouth's -Z... the rotations negated at the zero angles
    /// leave the frame's own axes. The Y velocity folds
    /// `-r * 0.6 - 0.3` (the downward arc into the mouth).
    #[test]
    fn the_piece_geometry_flies_into_the_mouth() {
        let (off, vel) = piece_offset_and_velocity(0.5, 1.0, 0.5, 0.0, 0.0, 0.0);
        assert!((off[0] - 0.0).abs() < 1e-6, "the x-center offset: (0.5-0.5)*0.1");
        assert!((off[1] - 0.2).abs() < 1e-6, "the y: r*0.1 + 0.1 at r=1.0");
        assert!((off[2]).abs() < 1e-6, "the z pre-rotation is 0");
        assert!((vel[0] - 0.0).abs() < 1e-6, "the x-center velocity");
        assert!((vel[1] + 0.3).abs() < 1e-6, "the y: -0*0.6 - 0.3");
        assert!((vel[2] - 0.6).abs() < 1e-6, "the z: the mouthward 0.6");
        // the corner case: the offsets span ±0.05 / [0.1, 0.2], the
        // velocities ±0.15 / [-0.9, -0.3]
        let (off, vel) = piece_offset_and_velocity(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert!((off[0] - 0.05).abs() < 1e-6 && (off[1] - 0.1).abs() < 1e-6);
        assert!((vel[0] + 0.15).abs() < 1e-6, "the x: (0-0.5)*0.3");
        assert!((vel[1] + 0.9).abs() < 1e-6, "the y: -1*0.6 - 0.3");
    }

    /// THE PIECE PHYSICS CHAIN (javap Particle then
    /// BreakingItemParticle): the parent's jitter+normalize fold, the
    /// 0.1 fold and the raw-velocity add. A straight-on spawn
    /// (r=0.5's zero jitter, the normalized d0's midpoint) rides
    /// ~1.1x the input velocity.
    #[test]
    fn the_piece_velocity_rides_the_mouthward_vector() {
        // the parent fold with the midpoint draws: the jitter (0.5*2-1)*0.4 = 0,
        // d0 = (0.5+0.5+1)*0.15 = 0.3, the normalized 0.3*0.4 = 0.12
        let parent = parent_velocity(0.6, -0.3, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5);
        // the input (0.6, -0.3, 0): the jitter zeros it — the normalize
        // speaks: len = sqrt(0.45) — the direction preserved, the
        // magnitude 0.12, then 0.2x rides
        let len_in = (0.6f64 * 0.6 + 0.3 * 0.3).sqrt();
        let expected_x = 0.6 / len_in * 0.12 + 0.6 * 0.2;
        let expected_y = -0.3 / len_in * 0.12 + -0.3 * 0.2;
        assert!((parent[0] - expected_x).abs() < 1e-6, "{} vs {}", parent[0], expected_x);
        assert!((parent[1] - expected_y).abs() < 1e-6);
        // the BreakingItem fold: 0.1x the parent + the raw param
        let final_v = breaking_item_velocity(parent, 0.6, -0.3, 0.0);
        assert!((final_v[0] - (expected_x * 0.1 + 0.6)).abs() < 1e-6);
        assert!((final_v[1] - (expected_y * 0.1 + -0.3)).abs() < 1e-6);
        // the final velocity rides the mouthward 0.6 (the z axis)
        assert!(final_v[0] > 0.5, "the mouthward dominance");
    }

    /// THE PIECE LIFETIME: `(int)(4.0 / (r * 0.9 + 0.1))` — [5, 40]
    /// ticks, the midpoint's ~7.
    #[test]
    fn the_piece_lifetime_speaks_the_four_over_the_draw() {
        assert_eq!(particle_lifetime(0.0), 40, "the 0.1 denominator's ceiling");
        assert_eq!(particle_lifetime(1.0), 4, "the 1.0 denominator's floor");
        assert_eq!(particle_lifetime(0.5), 7, "4.0 / 0.55");
    }

    /// THE PIECE TICK: the age, the lifetime's removal, the move, the
    /// 0.98 friction (the gravity lane silent — the item pieces never
    /// set gravity).
    #[test]
    fn the_piece_tick_folds_the_friction_and_the_clock() {
        let mut p = ItemPiece {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.6,
            vy: -0.3,
            vz: 0.0,
            age: 0,
            lifetime: 3,
            item: 0,
            uo: 0.0,
            vo: 0.0,
        };
        assert!(p.tick(), "alive at 1");
        assert!((p.x - 0.6).abs() < 1e-9);
        assert!((p.vx - 0.6 * 0.98).abs() < 1e-9, "the friction's own fold");
        assert!(p.tick(), "alive at 2");
        assert!(!p.tick(), "the lifetime's own removal at 3");
    }

    /// THE CAN-EAT GATE (javap Player.canEat): invulnerable OR the
    /// food's canAlwaysEat OR the bar's needsFood.
    #[test]
    fn the_can_eat_gate_speaks_the_player_law() {
        assert!(!can_eat(false, false, false), "the full bar refuses the plain apple");
        assert!(can_eat(false, false, true), "the hungry body eats");
        assert!(can_eat(true, false, false), "the golden apple always");
        assert!(can_eat(false, true, false), "the creative bypass");
    }

    /// THE LANE: the start builds the hold, the remaining counts down,
    /// the burst/complete gates speak the cadence's own table.
    #[test]
    fn the_eat_lane_walks_the_hold() {
        let mut lane = EatActive::start(1, &apple()).unwrap();
        assert_eq!(lane.total, 32);
        assert_eq!(lane.remaining(), 32);
        assert!(!lane.burst_due(), "the head is silent");
        // walk to the first burst (ticks 0..8: the remaining 24)
        for t in 1..8 {
            lane.ticks = t;
            assert!(!lane.burst_due(), "tick {t} silent");
        }
        lane.ticks = 8;
        assert!(lane.burst_due(), "the remaining 24 speaks");
        // the completion at the 32nd tick
        lane.ticks = 32;
        assert_eq!(lane.remaining(), 0);
        assert!(lane.complete_due());
        // the remainder and the food split ride the row
        assert_eq!(lane.use_remainder, None);
        assert_eq!(lane.nutrition, 4);
        assert!((lane.saturation - 2.4).abs() < 1e-6);
    }

    /// THE MILK LANE: no FOOD component (the nutrition's negative
    /// marks it), the DRINK animation, the bucket remainder — the
    /// completion never folds FoodData (no listener) and never burps.
    #[test]
    fn the_milk_lane_drinks_without_food() {
        let def = milk();
        assert!(!def.has_food(), "the potion's own split");
        assert_eq!(def.animation, mc_registry::generated::foods::ConsumableAnim::Drink);
        assert!(!def.has_consume_particles, "the drink carries no pieces");
        assert_eq!(def.use_remainder, Some("minecraft:bucket"));
        let lane = EatActive::start(2, &def).unwrap();
        assert_eq!(lane.nutrition, -1);
        assert_eq!(lane.use_remainder, Some("minecraft:bucket"));
    }

    /// THE GOLDEN APPLE'S PAYLOAD: the table carries the ApplyStatusEffects
    /// rows verbatim (the Regeneration II 100t + the Absorption 2400t)
    /// and the canAlwaysEat gate.
    #[test]
    fn the_golden_apple_carries_its_effects() {
        let def = gapple();
        assert!(def.can_always_eat, "the full bar eats it anyway");
        assert_eq!(def.effects.len(), 1);
        let e = &def.effects[0];
        assert_eq!(e.kind, mc_registry::generated::foods::EffectKind::ApplyStatusEffects);
        assert!((e.probability - 1.0).abs() < 1e-6);
        assert_eq!(e.rows.len(), 2);
        assert_eq!(e.rows[0].effect, "minecraft:regeneration");
        assert_eq!(e.rows[0].duration, 100);
        assert_eq!(e.rows[0].amplifier, 1, "the Regeneration II");
        assert_eq!(e.rows[1].effect, "minecraft:absorption");
        assert_eq!(e.rows[1].duration, 2400);
    }

    /// THE FIRST-PERSON POSE (javap applyEatTransform): the pull-in
    /// curve `1 - pow(f1, 27)` — zero at the hold's head, ~1 by the
    /// midpoint — the bite bob rides the cos law while f1 < 0.8, and
    /// the rotations fold the side.
    #[test]
    fn the_eat_pose_pulls_the_item_to_the_mouth() {
        // the hold's head: f1 = (32 - 0.5 + 1)/32 > 0.8 — the bob is
        // SILENT there (the item just rises); the pull-in still speaks
        let head = apply_eat_transform(32, 0.5, 32, 1.0);
        let f1_head: f32 = (32.0 - 0.5 + 1.0) / 32.0;
        let f3_head = 1.0 - f1_head.powi(27);
        assert!(f1_head >= 0.8);
        assert!((head.dy - (f3_head * -0.5)).abs() < 1e-6, "the head's bob silenced");
        assert!((head.dx - f3_head * 0.6).abs() < 1e-6);
        // the midpoint: f1 = (16 - 0.5 + 1)/32 ≈ 0.5156 → f3 ≈ 1
        let mid = apply_eat_transform(16, 0.5, 32, 1.0);
        let f1_mid: f32 = (16.0 - 0.5 + 1.0) / 32.0;
        let f3_mid = 1.0 - f1_mid.powi(27);
        assert!((mid.dx - f3_mid * 0.6).abs() < 1e-6);
        assert!(mid.dx > 0.55, "the item pulled to the mouth: {}", mid.dx);
        assert!((mid.dy - (f3_mid * -0.5 + mid.dy - (f3_mid * -0.5))).abs() < 1e-6);
        // the bite bob at the midpoint (f1 < 0.8): |cos(f/4*PI)| * 0.1
        // — the impl's f32 rounding mirrored (the f/4 product rounds in
        // f32 BEFORE the f64 cos)
        let bob = ((16.5f32 / 4.0 * std::f32::consts::PI) as f64).cos() * 0.1;
        let bob = bob.abs() as f32;
        assert!((mid.dy - (f3_mid * -0.5 + bob)).abs() < 1e-6, "the bob rides the Y");
        // the rotations: YP(side*f3*90), XP(f3*10), ZP(side*f3*30)
        assert!((mid.rot_y_deg - f3_mid * 90.0).abs() < 1e-6);
        assert!((mid.rot_x_deg - f3_mid * 10.0).abs() < 1e-6);
        assert!((mid.rot_z_deg - f3_mid * 30.0).abs() < 1e-6);
        // the left hand mirrors the Y and Z rotations
        let left = apply_eat_transform(16, 0.5, 32, -1.0);
        assert!((left.dx + mid.dx).abs() < 1e-6);
        assert!((left.rot_y_deg + mid.rot_y_deg).abs() < 1e-6);
        // the swallow's tail: f1 < 0.8 — the bob SPEAKS (the bite
        // rides to the last tick); the f3 curve ≈ 1 by now
        let tail = apply_eat_transform(4, 0.5, 32, 1.0);
        let f1_tail: f32 = (4.0 - 0.5 + 1.0) / 32.0;
        assert!(f1_tail < 0.8);
        let f3_tail = 1.0 - f1_tail.powi(27);
        let tail_bob = ((4.5f32 / 4.0 * std::f32::consts::PI) as f64).cos() * 0.1;
        let tail_bob = tail_bob.abs() as f32;
        assert!((tail.dy - (f3_tail * -0.5 + tail_bob)).abs() < 1e-6, "the tail's bob speaks");
    }

    /// THE 47-ROW TABLE: the probe's own size, the apple/bread/steak
    /// rows verbatim, the saturation ABSOLUTE (the 26.2 record's own
    /// field — the wiki's modifier*2 fold is the legacy overload's).
    #[test]
    fn the_foods_table_speaks_the_probe_verbatim() {
        assert_eq!(mc_registry::generated::foods::FOODS.len(), 47);
        let a = apple();
        assert_eq!(a.nutrition, 4);
        assert!((a.saturation - 2.4).abs() < 1e-6, "the ABSOLUTE saturation");
        assert!(!a.can_always_eat);
        assert_eq!(a.sound, "minecraft:entity.generic.eat");
        assert!(a.has_consume_particles);
        let b = mc_registry::generated::foods::food_def("minecraft:bread").unwrap();
        assert_eq!(b.nutrition, 5);
        assert!((b.saturation - 6.0).abs() < 1e-6);
        let s = mc_registry::generated::foods::food_def("minecraft:cooked_beef").unwrap();
        assert_eq!(s.nutrition, 8);
        assert!((s.saturation - 12.8).abs() < 1e-6);
        // the rotten flesh's 80% hunger payload
        let r = mc_registry::generated::foods::food_def("minecraft:rotten_flesh").unwrap();
        assert!((r.effects[0].probability - 0.8).abs() < 1e-6);
        assert_eq!(r.effects[0].rows[0].effect, "minecraft:hunger");
        assert_eq!(r.effects[0].rows[0].duration, 600);
        // the chorus's teleport kind
        let c = mc_registry::generated::foods::food_def("minecraft:chorus_fruit").unwrap();
        assert_eq!(c.effects[0].kind, mc_registry::generated::foods::EffectKind::TeleportRandomly);
    }

    /// THE DEFAULT SECONDS: the codec's own 1.6 fallback the table's
    /// rows all speak.
    #[test]
    fn the_default_seconds_speak_the_codec() {
        assert_eq!(DEFAULT_SECONDS, 1.6);
        assert_eq!(DEFAULT_CONSUME_SECONDS, 1.6);
    }
}

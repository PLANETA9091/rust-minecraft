//! THE FIRST MOB — the zombie, the client's first live entity.
//!
//! Every number here is javap-evidenced from the official 26.2 client
//! jar (sha1 `2dc72797acbc1b63fc16a11c4ac393605f453754`), the same
//! clean-room discipline the whole client speaks:
//!
//! - `EntityType$Builder.sized(0.6, 1.95).eyeHeight(1.74)` — the ZOMBIE
//!   registration block's own ldc constants (EntityTypes clinit).
//! - `Zombie.createAttributes()` — FOLLOW_RANGE 35.0, MOVEMENT_SPEED
//!   0.23000000417232513 (the f32 0.23 promoted), ATTACK_DAMAGE 3.0,
//!   ARMOR 2.0 (the Monster base adds only ATTACK_DAMAGE; the health
//!   rides the LivingEntity base's 20.0 MAX_HEALTH).
//! - `registerGoals()`/`addBehaviourGoals()` — the goal machine: prio 3
//!   `ZombieAttackGoal(this, 1.0, false)`, prio 7
//!   `WaterAvoidingRandomStrollGoal(this, 1.0)`, prio 8
//!   `LookAtPlayerGoal(this, Player, 8.0)` + `RandomLookAroundGoal`;
//!   targetSelector prio 2 `NearestAttackableTargetGoal(Player, true)`.
//! - `MeleeAttackGoal` — attackInterval 20 (the ctor's bipush 20; the
//!   canUse re-check gates every 20 ticks, the resetAttackCooldown
//!   refills 20), `canPerformAttack` = isTimeToAttack &&
//!   isWithinMeleeAttackRange && hasLineOfSight.
//! - `ZombieAttackGoal.tick` — raiseArmTicks++; `raiseArmTicks >= 5 &&
//!   getTicksUntilNextAttack() < getAttackInterval()/2` drives
//!   `setAggressive(true)` (the raised-arms law the render state
//!   speaks); `stop()` clears it.
//! - `RandomStrollGoal` — the canUse gate's interval 120 (bipush 120).
//! - `LookAtPlayerGoal` — the ctor's 8.0 range, the 0.02 probability
//!   ldc, the 40-tick look clock (the two bipush 40s).
//! - `Mob.isAggressive` — the DATA_MOB_FLAGS_ID byte's bit 0x4.
//! - `LivingEntity.tickDeath` — deathTime++ per tick, removal (the
//!   broadcast 60 event) at deathTime >= 20.
//! - `LivingEntity.knockback(strength, x, z)` — vx = vx/2 - nx*strength,
//!   vz = vz/2 - nz*strength (the (x,0,z) direction normalized), and
//!   the grounded vertical bump `vy = min(0.4, vy/2 + strength)`; the
//!   zero-direction random kick (the while (x²+z² < 1e-5) branch) draws
//!   two nextDoubles scaled 0.01.
//! - `ZombieRenderer` — `textures/entity/zombie/zombie.png` (the
//!   static init's ldc), the model rides HumanoidModel + the zombie
//!   arm branch the avatar module implements.
//! - THE BURN LAW (the daylight lane, this run's javap audit): 26.2
//!   moved the sun-burn into `Mob.aiStep` — `if is(BURN_IN_DAYLIGHT)
//!   burnUndead()` (the zombie rides the tag) — whose gate
//!   `isSunBurnTick` reads the TIMELINE attribute
//!   `EnvironmentAttributes.MONSTERS_BURN` (the `gameplay/monsters_burn`
//!   boolean hold track of `data/minecraft/timeline/day.json`:
//!   true 23460..12542 wrapping, false through the night), then
//!   `getLightLevelDependentMagicValue(eye) > 0.5` (the raw
//!   brightness through the lightmap's own `f/(4-3f)` ramp), then the
//!   draw `random.nextFloat() * 30 < (magic - 0.4) * 2`, then the
//!   water/powder gate (`isInWaterOrRain` blocks), then
//!   `level.canSeeSky(eye)` (the SKY channel == 15 at the eye).
//!   The hit: the HEAD slot's helmet branch (this lane's zombie never
//!   wears one — the branch rides the empty-stack return) falls
//!   through to `igniteForSeconds(8.0)` = `igniteForTicks(floor(160))`:
//!   `remainingFireTicks = max(remaining, 160)`.
//! - THE FIRE CLOCK (`Entity.baseTick`): `remainingFireTicks > 0 &&
//!   !fireImmune` — every tick `remainingFireTicks % 20 == 0 &&
//!   !isInLava` deals the `onFire` hurt 1.0 (the armor law absorbs —
//!   the zombie's 2.0 armor takes it to 0.94), then the counter
//!   decrements; the on-fire flag rides `remainingFireTicks > 0`. The
//!   corpse keeps its clock (LivingEntity.tick runs baseTick before
//!   tickDeath) but the dying refuse the hurt.
//! - THE EXTINGUISH: the water block's own inside-block effect
//!   (javap `InsideBlockEffectType.EXTINGUISH` = `Entity::clearFire`,
//!   remainingFireTicks = 0) — and the transition plays
//!   `GENERIC_EXTINGUISH_FIRE` ("entity.generic.extinguish_fire",
//!   random/fizz.ogg) at volume 0.7, pitch `1.6 + (nextFloat() -
//!   nextFloat()) * 0.4` (javap playEntityOnFireExtinguishedSound).
//!   `isInWaterOrRain` also blocks the burn roll outright.
//! - THE SKY DARKEN (`Level.updateSkyBrightness`): `skyDarken =
//!   (int)(15 - gameplay/sky_light_level)` (the track timeline.rs
//!   pins); the raw brightness = max(block, sky - darken)
//!   (LevelLightEngine.getRawBrightness).
//! - THE FLAME RENDER: `FlameFeatureRenderer.prepare` — the pose
//!   scales by `bbWidth * 1.4`, rotates by the billboard quaternion
//!   (`EntityRenderDispatcher.submit`: `Mth.rotationAroundAxis(Y_AXIS,
//!   camera.orientation)` = normalize((0, q.y, 0, q.w)) = the camera
//!   YAW alone — the pitch drops out), translates `(0, 0, 0.3 -
//!   (int)h * 0.02)` with `h = bbHeight / (bbWidth * 1.4)`, then
//!   stacks rings of 1.4-tall quads while `h > 0`: per ring the width
//!   x = 0.5 * 0.9^i, the base y rises 0.45, the z leans -0.03, the
//!   texture alternates FIRE_0 (even rings) / FIRE_1 (odd) off the
//!   blocks atlas, the (i/2)%2 pairs mirror the U, the color is
//!   white and the light's block half is forced 15 (the fire glows
//!   at night) — the frames ride the day tick (fire_0's mcmeta
//!   playlist [16..31, 0..15], fire_1 sequential).

use crate::body::{collide, WalkAnimationState, AIR_FRICTION, BLOCK_FRICTION, CUBE_SPEED_NUMERATOR, GRAVITY, GROUND_DRAG, JUMP_POWER, VERTICAL_DRAG, WALK_BLEND, WALK_DISTANCE_SCALE};
use crate::drops::Lcg;
use crate::hand::SwingState;
use crate::world::World;

/// The ZOMBIE registration's own `sized(0.6F, 1.95F)` (EntityTypes).
pub const ZOMBIE_WIDTH: f64 = 0.6;
pub const ZOMBIE_HEIGHT: f64 = 1.95;
/// The registration's `eyeHeight(1.74F)`.
pub const ZOMBIE_EYE_HEIGHT: f64 = 1.74;
/// `Zombie.createAttributes` — MOVEMENT_SPEED 0.23000000417232513.
pub const ZOMBIE_MOVEMENT_SPEED: f64 = 0.230_000_004_172_325_13;
/// The LivingEntity base MAX_HEALTH (the monster base adds none).
pub const ZOMBIE_MAX_HEALTH: f32 = 20.0;
/// `Zombie.createAttributes` — ATTACK_DAMAGE 3.0.
pub const ZOMBIE_ATTACK_DAMAGE: f64 = 3.0;
/// `Zombie.createAttributes` — ARMOR 2.0 (the hurt pipeline's own
/// reduction; the zombie never wears anything).
pub const ZOMBIE_ARMOR: f64 = 2.0;
/// `Zombie.createAttributes` — FOLLOW_RANGE 35.0 (the target scan).
pub const ZOMBIE_FOLLOW_RANGE: f64 = 35.0;
/// THE KILL'S OWN REWARD (javap `LivingEntity.dropExperience` →
/// `Mob.getBaseExperienceReward`): the bare zombie's flat `xpReward` 5 —
/// the equipment-slot walk adds nothing (the bare body wears none), the
/// player-kill gate the fist's `lastHurtByPlayerMemoryTime` arms speaks
/// the whole law.
pub const ZOMBIE_XP_REWARD: i32 = 5;
/// `MeleeAttackGoal`'s ctor bipush 20 — the attack cadence.
pub const ATTACK_INTERVAL_TICKS: i32 = 20;
/// `MeleeAttackGoal.canUse`'s own 20-tick re-check gate.
pub const ATTACK_RECHECK_TICKS: i64 = 20;
/// `ZombieAttackGoal.tick`'s `raiseArmTicks >= 5`.
pub const RAISE_ARM_DELAY: i32 = 5;
/// `RandomStrollGoal`'s interval (the canUse gate's bipush 120).
pub const STROLL_INTERVAL_TICKS: i32 = 120;
/// `LookAtPlayerGoal`'s ctor float 8.0 — the look range.
pub const LOOK_RANGE: f64 = 8.0;
/// `LookAtPlayerGoal`'s 0.02 probability.
pub const LOOK_PROBABILITY: f32 = 0.02;
/// `LookAtPlayerGoal`'s look clock (the bipush 40 pair).
pub const LOOK_INTERVAL_TICKS: i32 = 40;
/// `LivingEntity.tickDeath` — the removal gate's bipush 20.
pub const DEATH_DURATION_TICKS: i32 = 20;
/// The hurt flash's own duration (LivingEntity.hurt's hurtTime = 10).
pub const HURT_DURATION_TICKS: i32 = 10;
/// The hurt knockback's strength (the hurt pipeline's own 0.4).
pub const KNOCKBACK_STRENGTH: f64 = 0.4;
/// The ambient voice's interval seed (javap `Mob.resetAmbientSoundTime`
/// — `ambientSoundTime = -getAmbientSoundInterval()`, the bipush 80).
pub const AMBIENT_INTERVAL_TICKS: i32 = 80;
/// The ambient draw's bound (javap `Mob.baseTick` — the sipush 1000 the
/// `random.nextInt(1000) < ambientSoundTime++` gate speaks).
pub const AMBIENT_DRAW_BOUND: u64 = 1000;
/// The player-kill rare's own chance (the zombie loot table's
/// `random_chance_with_enchanted_bonus` unenchanted term — 0.025;
/// the looting bonus the bare fist never speaks).
pub const RARE_DROP_CHANCE: f32 = 0.025;
/// The rare pool's THREE unweighted entries (the table's own order —
/// the pick rides the nextInt(3) law the unweighted weights speak).
pub const RARE_DROP_ITEMS: [&str; 3] = ["iron_ingot", "carrot", "potato"];
/// `Mob.burnUndead`'s own `igniteForSeconds(8.0F)` — the sun's brand.
pub const BURN_IGNITE_SECONDS: f32 = 8.0;
/// `Entity.baseTick`'s on-fire hurt cadence (`remainingFireTicks % 20 == 0`).
pub const FIRE_DAMAGE_INTERVAL: i32 = 20;
/// `Entity.baseTick`'s `hurtServer(onFire, 1.0F)` — the burn's own dose.
pub const FIRE_DAMAGE: f64 = 1.0;
/// `isSunBurnTick`'s light gate (`f > 0.5F` — the magic value).
pub const BURN_LIGHT_GATE: f32 = 0.5;
/// `isSunBurnTick`'s roll: `nextFloat() * 30 < (magic - 0.4) * 2`.
pub const BURN_ROLL_SCALE: f32 = 30.0;
pub const BURN_ROLL_BIAS: f32 = 0.4;
pub const BURN_ROLL_GAIN: f32 = 2.0;
/// The magic value's ramp base — the lightmap's own `f/(4-3f)` curve
/// (LevelReader.getLightLevelDependentMagicValue over ambient light 0.0).
pub const BURN_MAGIC_DIVISOR_BASE: f32 = 4.0;
pub const BURN_MAGIC_DIVISOR_GAIN: f32 = 3.0;
/// The extinguish fizz's own fold (javap playEntityOnFireExtinguishedSound).
pub const EXTINGUISH_VOLUME: f32 = 0.7;
pub const EXTINGUISH_PITCH_BASE: f32 = 1.6;
pub const EXTINGUISH_PITCH_SPREAD: f32 = 0.4;
/// `Zombie.playStepSound`'s own flat fold (javap: `playSound(
/// getStepSound(), 0.15F, 1.0F)` — the ZOMBIE_STEP voice, no randoms).
pub const ZOMBIE_STEP_VOLUME: f32 = 0.15;
pub const ZOMBIE_STEP_PITCH: f32 = 1.0;

/// The wire id the plain item name rides (the ITEM_WIRE_ORDER's own
/// reverse lookup — `None` when the name misses the table: the honest
/// drop vanishes rather than inventing an id).
pub fn wire_id(name: &str) -> Option<i32> {
    mc_registry::generated::item_order::ITEM_WIRE_ORDER
        .iter()
        .position(|&n| n == name)
        .map(|p| p as i32)
}

/// The horizontal capture the stroll target walks to (the vanilla
/// navigation's arrival slack).
const STROLL_ARRIVAL: f64 = 0.5;
/// The monster category's own spawn cap (MobCategory.MONSTER — the
/// client-side spawner respects the same ceiling).
pub const MONSTER_CAP: usize = 70;
/// The spawner's cadence (the vanilla spawn cycle's 1-in-N per-chunk
/// stand-in the client speaks at the world-tick layer).
pub const SPAWN_CYCLE_TICKS: i64 = 40;
/// The spawn distance window (the MONSTER category's own 24..128 ring,
/// the near half the client renders honestly).
pub const SPAWN_MIN_DIST: f64 = 24.0;
pub const SPAWN_MAX_DIST: f64 = 48.0;

/// What the zombie's tick wants the world to do — the Mob.aiStep's
/// out-edges (the attack the player's hurt pipeline answers, the
/// ambient voice the Mob.baseTick's own gate speaks).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MobEvents {
    pub attack: Option<MobAttack>,
    /// the ambient groan's own beat (the baseTick gate fired this tick)
    pub ambient: bool,
    /// the fire clock's own onFire hurt LANDED this tick (the armor
    /// law took its share) — the caller speaks the hurt voice
    pub fire_hurt: bool,
    /// the fire death the fire clock caused this tick (health <= 0
    /// under the fire_hurt event — the caller speaks the death voice
    /// and rolls the loot, the player-kill gate closed)
    pub fire_death: bool,
    /// the extinguish fizz's own pitch (Some when the fire cleared
    /// under the water effect this tick — the caller plays
    /// entity.generic.extinguish_fire at 0.7 / the pitch)
    pub extinguished: Option<f32>,
    /// the footstep's own beat (the Entity.move cadence crossed
    /// nextStep over solid ground this tick — the caller plays
    /// entity.zombie.step at 0.15F / 1.0F flat)
    pub step: bool,
}

/// The zombie's attack out-edge (the doHurtTarget the melee goal
/// speaks — the damage attribute and the knockback direction).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MobAttack {
    pub damage: f64,
    /// the knockback direction (the victim-minus-attacker law's own
    /// (x, 0, z) the knockback call normalizes)
    pub dir_x: f64,
    pub dir_z: f64,
}

/// The player slice the zombie's AI reads (the NearestAttackableTarget
/// goal's own view).
#[derive(Debug, Clone, Copy)]
pub struct PlayerView {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub eye_y: f64,
    pub dead: bool,
    /// the player's own bbox width (the melee range law's target term)
    pub width: f64,
}

/// THE ZOMBIE — the fields the vanilla entity's own state speaks.
pub struct Zombie {
    /// the entity id (the client's own counter — the wire ids stay the
    /// networked world's)
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// The previous tick's feet (javap `Entity.tick`'s own `xo = x`
    /// head — the pre-move pose the render's interpolation walks
    /// FROM). Saved at the tick's own head, before the AI physics.
    pub prev_x: f64,
    pub prev_y: f64,
    pub prev_z: f64,
    /// THE WIRE'S OWN LERP TARGET (javap `Entity.lerpTo` + the
    /// `lerpSteps` walk — the REMOTE entity's law): the server's
    /// absolute pose the client EASES to over 3 ticks (the 250ms
    /// refresh cadence's own pacing). `None` for the AI mobs — the
    /// goal-driven bodies own their physics, the lerp never speaks.
    pub lerp_target: Option<(f64, f64, f64)>,
    pub lerp_steps: u8,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    /// the body's rotation (degrees, the vanilla yaw convention)
    pub yaw: f32,
    /// the head's look offset from the body (the netHeadYaw clamp
    /// +-75° the LivingEntityRenderer extraction speaks)
    pub head_yaw_off: f32,
    /// the head's pitch (positive = down, the model's xRot)
    pub head_pitch: f32,
    /// the head's O pose (javap LivingEntity's `yHeadRotO` + Entity's
    /// `xRotO` — the pre-move head the render's lerp walks FROM).
    /// Saved at the tick's own head, before the AI's look resolves.
    pub prev_head_yaw_off: f32,
    pub prev_head_pitch: f32,
    pub on_ground: bool,
    /// the LivingEntity health (the MAX_HEALTH attribute's own float)
    pub health: f32,
    /// the hurt flash clock (hurtTime — 10 at the hit, walks to 0)
    pub hurt_time: i32,
    /// the death clock (deathTime — 0 alive, 20 removes)
    pub death_time: i32,
    /// the swing clock (the attack anim the render state's attackTime
    /// samples — the same LivingEntity law the player's hand speaks)
    pub swing: SwingState,
    /// the walk animation store (the render state's walk fields)
    pub walk: WalkAnimationState,
    /// `ZombieRenderState.isAggressive` (the mob flag bit 0x4)
    pub aggressive: bool,
    /// `ZombieAttackGoal.raiseArmTicks`
    raise_arm_ticks: i32,
    /// `MeleeAttackGoal.ticksUntilNextAttack`
    ticks_until_attack: i32,
    /// the canUse re-check clock (lastCanUseCheck)
    attack_recheck: i64,
    /// the stroll's target (the navigation's own destination stand-in)
    stroll_target: Option<(f64, f64)>,
    /// the look goal's remaining clock (the lookTime)
    look_ticks: i32,
    /// the age clock (the bob's ageInTicks, 20 tps)
    pub age_ticks: u32,
    /// the ambient voice's clock (the Mob.baseTick's own
    /// ambientSoundTime — starts at -80, walks up, fires + resets)
    ambient_time: i32,
    /// the entity's own RNG (the vanilla RandomSource's stand-in the
    /// loot lane speaks — the same LCG constants)
    rng: Lcg,
    /// `Entity.remainingFireTicks` — the sun-burn brand's own clock:
    /// set 160 by `igniteForSeconds(8)`, decremented per tick, the
    /// onFire hurt rides the %20 gate, the water effect clears it
    pub fire_ticks: i32,
    /// THE FOOTSTEP ACCUMULATION (Entity.move's own fields — the same
    /// pair the player's body speaks): moveDist grows by the collided
    /// horizontal × 0.6, the step fires past nextStep (the ctor's own
    /// 1.0F store) over the solid block under the feet.
    pub move_dist: f32,
    pub next_step: f32,
}

impl Zombie {
    pub fn new(id: i32, x: f64, y: f64, z: f64, seed: u64) -> Self {
        Zombie {
            id,
            x,
            y,
            z,
            prev_x: x,
            prev_y: y,
            prev_z: z,
            lerp_target: None,
            lerp_steps: 0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            yaw: 0.0,
            head_yaw_off: 0.0,
            head_pitch: 0.0,
            prev_head_yaw_off: 0.0,
            prev_head_pitch: 0.0,
            on_ground: false,
            health: ZOMBIE_MAX_HEALTH,
            hurt_time: 0,
            death_time: 0,
            swing: SwingState::default(),
            walk: WalkAnimationState::default(),
            aggressive: false,
            raise_arm_ticks: 0,
            ticks_until_attack: 0,
            attack_recheck: 0,
            stroll_target: None,
            look_ticks: 0,
            age_ticks: 0,
            ambient_time: -AMBIENT_INTERVAL_TICKS,
            rng: Lcg::new(seed),
            fire_ticks: 0,
            move_dist: 0.0,
            next_step: 1.0,
        }
    }

    /// The zombie's feet-anchored bbox (the registration's own dims).
    pub fn box_at(&self) -> (f64, f64, f64, f64, f64, f64) {
        let h = ZOMBIE_WIDTH / 2.0;
        (self.x - h, self.y, self.z - h, self.x + h, self.y + ZOMBIE_HEIGHT, self.z + h)
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

    /// THE HEAD POSE'S OWN RENDER FOLD (LivingEntityRenderer's
    /// `Mth.lerp(partialTicks, yHeadRotO, yHeadRot)` the netHeadYaw
    /// speaks): the previous tick's head carried `partial` of the way
    /// to the current — the AI's new look target the head EASES to,
    /// never snaps. The partial >= 1.0 arm speaks the CURRENT head
    /// bit for bit (the headless lanes' byte-identical contract).
    pub fn lerped_head_pose(&self, partial: f32) -> (f32, f32) {
        if partial >= 1.0 {
            return (self.head_yaw_off, self.head_pitch);
        }
        let t = partial;
        (
            self.prev_head_yaw_off
                + (self.head_yaw_off - self.prev_head_yaw_off) * t,
            self.prev_head_pitch + (self.head_pitch - self.prev_head_pitch) * t,
        )
    }

    /// The flat distance² to the player (the melee range law's own
    /// horizontal read — the vanilla sqrDistToEntity pairs the Y into
    /// the box distance; the melee gate rides the horizontal walk).
    fn dist2_xz(&self, p: &PlayerView) -> f64 {
        let dx = p.x - self.x;
        let dz = p.z - self.z;
        dx * dx + dz * dz
    }

    /// `PathfinderMob.isWithinMeleeAttackRange`'s own law (javap over
    /// the LivingEntity variant the MeleeAttackGoal calls): the squared
    /// horizontal distance under `bbWidth*2*bbWidth*2 + target.width`
    /// — for the zombie: (0.6*2)² + 0.6 = 2.04.
    fn within_melee_range(&self, p: &PlayerView) -> bool {
        let reach2 = ZOMBIE_WIDTH * 2.0 * ZOMBIE_WIDTH * 2.0 + p.width;
        self.dist2_xz(p) < reach2
    }

    /// THE PLAYER'S HIT — the player attack's own out-edge (the hurt
    /// pipeline's law: the hurtTime restarts at 10, the health walks
    /// down through the ARMOR LAW (javap `CombatRules.getDamageAfterAbsorb`
    /// — `f5 = 2.0 + toughness/4` (the zombie's toughness rides the
    /// base 0.0), `f6 = clamp(armor - dmg/f5, armor*0.2, 20)`, the
    /// survivor `dmg * (1 - f6/25)`), 0 -> dying).
    pub fn hurt(&mut self, amount: f64) {
        // the dying never re-hurt (the vanilla's own `dead` gate — the
        // isDeadOrDying body the hurt pipeline refuses)
        if self.death_time > 0 || self.health <= 0.0 {
            return;
        }
        let amount = amount as f32;
        // the ARMOR LAW (CombatRules.getDamageAfterAbsorb, the bare
        // weapon branch — no piercing item the fist speaks)
        let f5 = 2.0_f32; // 2.0 + ARMOR_TOUGHNESS(0.0)/4
        let f6 = (ZOMBIE_ARMOR as f32 - amount / f5).clamp(ZOMBIE_ARMOR as f32 * 0.2, 20.0);
        self.health -= amount * (1.0 - f6 / 25.0);
        self.hurt_time = HURT_DURATION_TICKS;
        self.walk.set_speed(crate::body::HURT_WALK_SPEED);
    }

    /// `LivingEntity.knockback(strength, x, z)` verbatim — the (x, 0, z)
    /// direction normalized (the zero-length branch draws the 0.01
    /// random kick), the halves, the grounded 0.4 bump.
    pub fn knockback(&mut self, strength: f64, dx: f64, dz: f64) {
        let (mut dx, mut dz) = (dx, dz);
        if dx * dx + dz * dz < 1e-5 {
            dx = (self.rng.next_f64() - self.rng.next_f64()) * 0.01;
            dz = (self.rng.next_f64() - self.rng.next_f64()) * 0.01;
        }
        let len = (dx * dx + dz * dz).sqrt();
        let nx = dx / len * strength;
        let nz = dz / len * strength;
        self.vx = self.vx / 2.0 - nx;
        self.vz = self.vz / 2.0 - nz;
        if self.on_ground {
            self.vy = (self.vy / 2.0 + strength).min(0.4);
        }
    }

    /// THE AI + PHYSICS TICK (one game tick, the frame==tick law the
    /// client's world loop speaks). The goals run in the vanilla
    /// priority order — the attack goal owns the mob, the stroll fills
    /// the rest, the look fills the head. The AMBIENT VOICE rides the
    /// Mob.baseTick's own gate (`nextInt(1000) < ambientSoundTime++`,
    /// reset to -80) BEFORE the goals. `day_time` feeds the burn lane
    /// (the MONSTERS_BURN track + the sky darken the light law rides).
    pub fn tick(&mut self, world: &World, player: Option<&PlayerView>, day_time: i64) -> MobEvents {
        let mut events = MobEvents::default();
        // THE TICK'S OWN HEAD (javap `Entity.tick`'s `xo = x` fold): the
        // pre-move feet the render's interpolation walks FROM
        self.prev_x = self.x;
        self.prev_y = self.y;
        self.prev_z = self.z;
        // the head's own O pair (LivingEntity's `yHeadRotO = yHeadRot`
        // + Entity.baseTick's `xRotO = xRot` — the pre-move head the
        // render's lerp walks FROM, saved BEFORE the AI's look resolves
        // a new target)
        self.prev_head_yaw_off = self.head_yaw_off;
        self.prev_head_pitch = self.head_pitch;
        // THE FIRE CLOCK (Entity.baseTick's own block, run BEFORE the
        // death gate — the corpse keeps its clock but the dying refuse
        // the hurt): remainingFireTicks > 0 && !fireImmune -> the %20
        // onFire hurt (not in lava — no lava in this lane), then the
        // decrement.
        if self.fire_ticks > 0 {
            let alive = self.death_time == 0 && self.health > 0.0;
            if self.fire_ticks % FIRE_DAMAGE_INTERVAL == 0 && alive {
                self.hurt(FIRE_DAMAGE);
                events.fire_hurt = true;
                events.fire_death = self.health <= 0.0;
            }
            self.fire_ticks -= 1;
        }
        // THE EXTINGUISH (the water block's inside-block effect:
        // InsideBlockEffectType.EXTINGUISH = Entity::clearFire) — the
        // body in water clears the brand, the transition speaks the
        // fizz at its own fold.
        if self.in_water(world) {
            if self.fire_ticks > 0 {
                self.fire_ticks = 0;
                events.extinguished = Some(
                    EXTINGUISH_PITCH_BASE
                        + (self.rng.next_f32() - self.rng.next_f32()) * EXTINGUISH_PITCH_SPREAD,
                );
            }
        }
        // THE DEATH CLOCK (LivingEntity.tickDeath): the isDeadOrDying
        // body's own gate (health <= 0), deathTime++ per tick, removal
        // at 20 (the broadcast 60 event rides the caller).
        if self.health <= 0.0 {
            self.death_time += 1;
            self.swing.tick();
            return events;
        }
        // THE AMBIENT VOICE (Mob.baseTick's own law: alive -> the
        // nextInt(1000) draw under the climbing clock, the play + the
        // -80 reset; the draw rides the entity's own LCG)
        self.ambient_time += 1;
        if self.rng.next() % AMBIENT_DRAW_BOUND < self.ambient_time.max(0) as u64 {
            self.ambient_time = -AMBIENT_INTERVAL_TICKS;
            events.ambient = true;
        }
        // THE HURT CLOCK (LivingEntity.tick's own decrement)
        if self.hurt_time > 0 {
            self.hurt_time -= 1;
        }
        self.swing.tick();
        self.age_ticks = self.age_ticks.wrapping_add(1);

        let mut want_move = false;
        let (mut ix, mut iz) = (0.0f64, 0.0f64);

        // THE TARGET SCAN (NearestAttackableTargetGoal(Player, mustSee)
        // — the FOLLOW_RANGE 35 ring, the dead never retarget)
        let target = player.filter(|p| !p.dead).filter(|p| {
            let dx = p.x - self.x;
            let dz = p.z - self.z;
            dx * dx + dz * dz <= ZOMBIE_FOLLOW_RANGE * ZOMBIE_FOLLOW_RANGE
        });

        if let Some(p) = target {
            // ZOMBIEATTACKGOAL (prio 3): the canUse re-check rides the
            // 20-tick gate; the goal's own tick runs every tick while
            // the target lives in range.
            self.ticks_until_attack -= 1;
            // the melee approach: the navigation's straight-line stand-in
            // walks the mob at its attribute speed toward the target
            let dx = p.x - self.x;
            let dz = p.z - self.z;
            let len = (dx * dx + dz * dz).sqrt().max(1e-9);
            self.yaw = (-(dx / len)).atan2(dz / len).to_degrees() as f32;
            ix = 0.0;
            iz = 1.0; // the forward impulse rides the body's own yaw
            want_move = true;
            // THE HEAD (the mob's own look-at-target law — the body
            // faces and the head rides the clamp)
            self.look_at(p.x, p.eye_y, p.z);
            // THE ATTACK (canPerformAttack: isTimeToAttack &&
            // isWithinMeleeAttackRange && hasLineOfSight) — the hit
            // lands, the swing rides, the cooldown refills 20.
            let los = crate::body::clip_ray(
                world,
                crate::math::Vec3::new(
                    self.x as f32,
                    (self.y + ZOMBIE_EYE_HEIGHT) as f32,
                    self.z as f32,
                ),
                crate::math::Vec3::new(p.x as f32, p.eye_y as f32, p.z as f32),
            )
            .is_none();
            if self.ticks_until_attack <= 0 && self.within_melee_range(p) && los {
                // doHurtTarget: the damage speaks the ATTACK_DAMAGE
                // attribute, the swing rides the main hand, the
                // knockback direction runs victim-minus-attacker
                let (kx, kz) = (p.x - self.x, p.z - self.z);
                events.attack = Some(MobAttack {
                    damage: ZOMBIE_ATTACK_DAMAGE,
                    dir_x: kx,
                    dir_z: kz,
                });
                self.swing.start();
                self.ticks_until_attack = ATTACK_INTERVAL_TICKS;
            }
            // THE RAISED ARMS (ZombieAttackGoal.tick's own law: the
            // raiseArmTicks gate 5, then the half-interval compare —
            // getTicksUntilNextAttack() < getAttackInterval()/2)
            self.raise_arm_ticks += 1;
            self.aggressive = self.raise_arm_ticks >= RAISE_ARM_DELAY
                && self.ticks_until_attack < ATTACK_INTERVAL_TICKS / 2;
            // the goal's own start() zeroes the raise clock when the
            // target changes — the fresh scan re-arms it (the stop()
            // clears the aggressive flag on the way out)
        } else {
            // the goal's stop(): the arms lower, the raise clock resets
            self.aggressive = false;
            self.raise_arm_ticks = 0;
            // THE STROLL GOAL (prio 7): the canUse gate rides the
            // 120-tick interval (the RandomStrollGoal's own), the
            // target walks until the arrival slack speaks
            if let Some((tx, tz)) = self.stroll_target {
                let dx = tx - self.x;
                let dz = tz - self.z;
                let d2 = dx * dx + dz * dz;
                if d2 < STROLL_ARRIVAL * STROLL_ARRIVAL {
                    self.stroll_target = None;
                } else {
                    let len = d2.sqrt();
                    self.yaw = (-(dx / len)).atan2(dz / len).to_degrees() as f32;
                    ix = 0.0;
                    iz = 1.0;
                    want_move = true;
                }
            } else if self.rng.next() % STROLL_INTERVAL_TICKS as u64 == 0 {
                // the random stroll target (the LandRandomPos ring's
                // own +-10 the goal's getPosition speaks)
                let ang = self.rng.next_f64() * std::f64::consts::TAU;
                let dist = self.rng.next_f64() * 10.0;
                self.stroll_target = Some((self.x + ang.cos() * dist, self.z + ang.sin() * dist));
            }
            // THE LOOK GOAL (prio 8): the look-at-player branch rides
            // the 0.02 probability per 40-tick cycle (the ctor's own
            // constants), the look holds 40 ticks
            if self.look_ticks > 0 {
                self.look_ticks -= 1;
                if let Some(p) = player.filter(|p| !p.dead) {
                    self.look_at(p.x, p.eye_y, p.z);
                }
            } else if self.rng.next() % LOOK_INTERVAL_TICKS as u64 == 0
                && self.rng.next_f32() < LOOK_PROBABILITY
            {
                if let Some(p) = player.filter(|p| {
                    !p.dead
                        && (p.x - self.x) * (p.x - self.x) + (p.z - self.z) * (p.z - self.z)
                            <= LOOK_RANGE * LOOK_RANGE
                }) {
                    self.look_ticks = LOOK_INTERVAL_TICKS;
                    self.look_at(p.x, p.eye_y, p.z);
                }
            } else if self.rng.next() % LOOK_INTERVAL_TICKS as u64 == 0 {
                // the RandomLookAround branch: a random head yaw holds
                self.head_yaw_off = (self.rng.next_f64() * 2.0 - 1.0) as f32 * 75.0;
                self.head_pitch = 0.0;
            }
        }

        // THE BURN ROLL (Mob.aiStep's BURN_IN_DAYLIGHT branch ->
        // burnUndead -> isSunBurnTick, the short-circuit order the
        // bytecode speaks: the track gate, the water gate, the light
        // gate, THE DRAW, the sky gate — the helmet branch never fires
        // on this lane's bare-headed zombie, so the hit ignites):
        // igniteForSeconds(8.0) = remainingFireTicks = max(remaining,
        // 160).
        if crate::timeline::monsters_burn(day_time)
            && !self.in_water(world)
            && self.burn_light_gate(world, day_time)
            && self.rng.next_f32() * BURN_ROLL_SCALE
                < (self.burn_magic(world, day_time) - BURN_ROLL_BIAS) * BURN_ROLL_GAIN
            && self.can_see_sky(world)
        {
            let ticks = (BURN_IGNITE_SECONDS * 20.0) as i32;
            self.fire_ticks = self.fire_ticks.max(ticks);
        }

        // THE MOVE IMPULSE (the travel law's own getInputVector — the
        // zombie's ground control rides its own attribute, the friction
        // correction the player's law speaks verbatim)
        let (x0, y0, z0) = (self.x, self.y, self.z);
        if want_move && (ix * ix + iz * iz) > 1e-7 {
            let friction = if self.on_ground { BLOCK_FRICTION } else { AIR_FRICTION };
            let speed = if friction > 0.6 {
                ZOMBIE_MOVEMENT_SPEED * CUBE_SPEED_NUMERATOR as f64
                    / (friction * friction * friction) as f64
            } else {
                ZOMBIE_MOVEMENT_SPEED
            };
            let rad = self.yaw as f64 * std::f64::consts::PI / 180.0;
            let (s, c) = (rad.sin(), rad.cos());
            self.vx += (ix * c - iz * s) * speed;
            self.vz += (iz * c + ix * s) * speed;
        }
        // THE COLLIDE (the same sweep the player's law speaks)
        let col = collide(world, self.box_at(), (self.vx, self.vy, self.vz));
        self.x += col.dx;
        self.y += col.dy;
        self.z += col.dz;
        let was_on_ground = self.on_ground;
        self.on_ground = col.vertical_collision_below;
        // THE FOOTSTEP CADENCE (Entity.move -> applyMovementEmissionAnd
        // PlaySound verbatim): moveDist grows by the collided movement's
        // own horizontal × 0.6 (the multiply rides the double like the
        // bytecode's dmul before the d2f; the climbable branch's
        // full-length lane aside — this walker climbs nothing), the step
        // fires when moveDist crosses nextStep (the ctor's own 1.0F
        // store) with the block under the feet (getOnPos: floor(x),
        // floor(y - 0.2), floor(z)) not air AND the ground gate passed
        // (vibrationAndSoundEffectsFromBlock's own verdict — onGround;
        // the climbable/rails/crouch branches don't speak for this
        // walker, and the swim branch never fires: the zombie never
        // isSwimming). The reset rides nextStep() = (int)moveDist + 1.0F
        // — one footstep per ~1.667 blocks, the first one spoken the
        // moment the walk crosses the ctor's 1.0F.
        self.move_dist += ((col.dx * col.dx + col.dz * col.dz).sqrt() * 0.6) as f32;
        if self.move_dist > self.next_step {
            let under = world.get(
                self.x.floor() as i32,
                (self.y - 0.2).floor() as i32,
                self.z.floor() as i32,
            );
            if under != crate::world::AIR && self.on_ground {
                self.next_step = (self.move_dist as i32) as f32 + 1.0;
                events.step = true;
            }
        }
        // THE AUTO-JUMP (LivingEntity.aiStep's own: the horizontal
        // collision + the ground -> jumpFromGround — the zombie hops
        // the one-block wall the navigation can't path around)
        if col.horizontal_collision && was_on_ground {
            self.vy = JUMP_POWER as f64;
            self.on_ground = false;
        }
        // THE RESTITUTION + DRAGS + GRAVITY (the travel tail verbatim)
        if col.collided_x {
            self.vx = 0.0;
        }
        if col.collided_y {
            self.vy = 0.0;
        }
        if col.collided_z {
            self.vz = 0.0;
        }
        let friction = if was_on_ground { BLOCK_FRICTION } else { AIR_FRICTION };
        self.vy = (self.vy - GRAVITY) * VERTICAL_DRAG as f64;
        let drag = (friction * GROUND_DRAG) as f64;
        self.vx *= drag;
        self.vz *= drag;
        // THE WALK ANIMATION (calculateEntityAnimation's own law — the
        // same 4.0 scale and 0.4 blend the player's store speaks)
        let dist = ((self.x - x0) * (self.x - x0)
            + (self.y - y0) * (self.y - y0)
            + (self.z - z0) * (self.z - z0))
            .sqrt() as f32;
        if dist.is_finite() {
            self.walk
                .update((dist * WALK_DISTANCE_SCALE).min(1.0), WALK_BLEND, 1.0);
        } else {
            self.walk.stop();
        }
        events
    }

    /// THE WATER GATE — `Entity.isInWater`'s stand-in: any block the
    /// bbox spans is WATER (the vanilla's fluid-height-over-box law
    /// degrades honestly on this grid: the feet column wades, the body
    /// block swims).
    fn in_water(&self, world: &World) -> bool {
        let (x0, y0, z0, x1, y1, z1) = self.box_at();
        for wx in (x0.floor() as i32)..=(x1.floor() as i32) {
            for wy in (y0.floor() as i32)..=(y1.floor() as i32) {
                for wz in (z0.floor() as i32)..=(z1.floor() as i32) {
                    if world.get(wx, wy, wz) == crate::world::WATER {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// The eye cell's (sky, block) levels — `BlockPos.containing(x,
    /// getEyeY(), z)` over the light engine's two channels. The
    /// networked world answers from its own grid (the server's engine
    /// said it); the procedural scene falls back to the DIRECT sky
    /// column (15 down the open air, -1 per attenuating water/leaves
    /// block, 0 under anything opaque) with block light 0 — the
    /// offline terrain states no light-emitting block, and the honest
    /// unknown beats an invented grid.
    fn eye_light(&self, world: &World) -> (u8, u8) {
        let ex = self.x.floor() as i32;
        let ey = (self.y + ZOMBIE_EYE_HEIGHT).floor() as i32;
        let ez = self.z.floor() as i32;
        if let Some(grid) = &world.light {
            let (s, b) = grid.cell(world.chunks_per_side, ex, ey, ez);
            let fallback = self.sky_column(world, ex, ey, ez);
            return (s.unwrap_or(fallback), b.unwrap_or(0));
        }
        (self.sky_column(world, ex, ey, ez), 0)
    }

    /// The direct sky-light column above the eye — 15 through the open
    /// air, one level lost per attenuating block (water/leaves speak
    /// opacity 1), zero under anything opaque.
    fn sky_column(&self, world: &World, ex: i32, ey: i32, ez: i32) -> u8 {
        let mut light = 15u8;
        let mut wy = ey + 1;
        while wy < crate::world::CHUNK_Y as i32 {
            let id = world.get(ex, wy, ez);
            if id != crate::world::AIR {
                if id == crate::world::WATER || id == crate::world::LEAVES {
                    light = light.saturating_sub(1);
                } else {
                    return 0;
                }
            }
            wy += 1;
        }
        light
    }

    /// `LevelReader.getLightLevelDependentMagicValue(eye)` — the raw
    /// brightness max(block, sky - darken) through the lightmap's own
    /// `f/(4-3f)` ramp (the mesher's `light_brightness` is the SAME
    /// curve — LevelReader's ambient-light lerp lands on 0.0 for the
    /// overworld, so the identity is exact).
    fn burn_magic(&self, world: &World, day_time: i64) -> f32 {
        let (sky, block) = self.eye_light(world);
        let darken = crate::timeline::sky_darken(day_time);
        let raw = (block as i32).max(sky as i32 - darken).clamp(0, 15) as f32;
        let f = raw / 15.0;
        f / (BURN_MAGIC_DIVISOR_BASE - BURN_MAGIC_DIVISOR_GAIN * f)
    }

    /// The light gate half of `isSunBurnTick` — `magic > 0.5F`
    /// (raw 13..15 pass, raw 12 sits exactly ON the gate and fails).
    fn burn_light_gate(&self, world: &World, day_time: i64) -> bool {
        self.burn_magic(world, day_time) > BURN_LIGHT_GATE
    }

    /// `BlockAndLightGetter.canSeeSky(eye)` — the SKY channel at 15
    /// (the direct column unbroken).
    fn can_see_sky(&self, world: &World) -> bool {
        let (sky, _) = self.eye_light(world);
        sky >= 15
    }

    /// The head's look law: the yaw offset rides the +-75° clamp the
    /// LivingEntityRenderer extraction speaks, the pitch rides the
    /// model's own positive-down convention.
    fn look_at(&mut self, tx: f64, ty: f64, tz: f64) {
        let dx = tx - self.x;
        let dz = tz - self.z;
        let dy = ty - (self.y + ZOMBIE_EYE_HEIGHT);
        let horiz = (dx * dx + dz * dz).sqrt();
        let head_abs = (-(dx.max(1e-9) / horiz.max(1e-9))).atan2(dz / horiz.max(1e-9)).to_degrees();
        let body = self.yaw as f64;
        let mut off = head_abs - body;
        while off > 180.0 {
            off -= 360.0;
        }
        while off < -180.0 {
            off += 360.0;
        }
        self.head_yaw_off = off.clamp(-75.0, 75.0) as f32;
        self.head_pitch = (-dy).atan2(horiz).to_degrees() as f32;
    }

    /// The render state's deathTime + partialTick reading (the
    /// LivingEntityRenderer extraction's own gate — 0 when alive).
    pub fn death_time_at(&self, partial: f32) -> f32 {
        if self.death_time > 0 {
            self.death_time as f32 + partial
        } else {
            0.0
        }
    }

    /// `LivingEntity.getVoicePitch` — the hurt/death/ambient voices'
    /// pitch law, verbatim: `(r - r) * 0.2 + 1.0` over the entity's
    /// own random stream (the two nextFloat draws in order).
    pub fn voice_pitch(&mut self) -> f32 {
        (self.rng.next_f32() - self.rng.next_f32()) * 0.2 + 1.0
    }

    /// THE DEATH LOOT (javap `dropAllDeathLoot` — the
    /// `lastHurtByPlayerMemoryTime > 0` flag rides killed_by_player,
    /// the table `data/minecraft/loot_table/entities/zombie.json`
    /// speaks the pools verbatim): POOL 1 always rolls the
    /// rotten_flesh (`set_count` uniform 0.0..2.0 —
    /// `Mth.nextInt(random, 0, 2)` over the ConstantNumberProvider
    /// getInts, the looting bonus absent from the bare fist); POOL 3
    /// speaks the player-kill rare (`killed_by_player` +
    /// `random_chance_with_enchanted_bonus`'s 0.025 unenchanced, one
    /// of the THREE unweighted entries — iron_ingot / carrot /
    /// potato); the zombie_horse and baby-chicken pools never fire in
    /// this lane's vocabulary, and the potato's furnace_smelt branch
    /// never does either (the zombie never burns here). The draws
    /// ride the entity's own random stream, in the pools' own order.
    /// Returns (wire item id, count) pairs — the caller spawns them
    /// at the entity's feet through the spawnAtLocation law.
    pub fn death_loot(&mut self, killed_by_player: bool) -> Vec<(i32, i32)> {
        // the rolls ride the death itself (the caller only calls this
        // at the health<=0 crossing — the belt for the honest path)
        if self.health > 0.0 {
            return Vec::new();
        }
        let mut out: Vec<(i32, i32)> = Vec::new();
        // POOL 1: the flesh — the uniform count 0..2
        if let Some(flesh) = wire_id("rotten_flesh") {
            let count = self.rng.next_int_inclusive(0, 2);
            if count > 0 {
                out.push((flesh, count));
            }
        }
        // POOL 3: the player-kill rare — the 0.025 chance, then one of
        // the three unweighted entries (the pool's own rolls 1)
        if killed_by_player && self.rng.next_f32() < RARE_DROP_CHANCE {
            let pick = self.rng.next_int_inclusive(0, 2);
            if let Some(item) = wire_id(RARE_DROP_ITEMS[pick as usize]) {
                out.push((item, 1));
            }
        }
        out
    }
}

/// THE SPAWNER'S ATTEMPT — the MONSTER category's own surface law: a
/// random ring point, the topmost solid's head-room read, the spawn
/// lands on the surface (the light/night gate rides the caller — the
/// client's sky lane owns the celestial clock).
pub fn try_spawn(
    mobs: &mut Vec<Zombie>,
    next_id: &mut i32,
    px: f64,
    pz: f64,
    world: &World,
    rng: &mut Lcg,
    night: bool,
) -> bool {
    if !night || mobs.len() >= MONSTER_CAP {
        return false;
    }
    let ang = rng.next_f64() * std::f64::consts::TAU;
    let dist = SPAWN_MIN_DIST + rng.next_f64() * (SPAWN_MAX_DIST - SPAWN_MIN_DIST);
    let sx = (px + ang.cos() * dist).floor() as i32;
    let sz = (pz + ang.sin() * dist).floor() as i32;
    // the topmost solid under the sky (the surface law's own scan)
    let mut sy: Option<i32> = None;
    for y in (0..world.size_blocks()).rev() {
        if crate::body::solid(world, sx, y, sz) {
            sy = Some(y);
            break;
        }
    }
    let Some(sy) = sy else {
        return false;
    };
    let fy = sy + 1;
    // the head-room read (the spawn box must fit — the vanilla's own
    // no-collision rule the spawn speaks)
    for dy in 0..2 {
        if crate::body::solid(world, sx, fy + dy, sz) {
            return false;
        }
    }
    let mut id = *next_id;
    *next_id += 1;
    mobs.push(Zombie::new(
        id,
        sx as f64 + 0.5,
        fy as f64,
        sz as f64 + 0.5,
        (*next_id as u64) << 16 ^ rng.next(),
    ));
    true
}

/// THE ENTITY PICK's own ray-box test (javap `AABB.clip` — the slab
/// method over the 1/Δ inverses, the f64 infinities folding the
/// parallel axes the same way Java's math speaks). The eye segment's
/// entry point into the box: `Some((point, t))` with t in [0, 1], or
/// `None` (the segment misses, or starts outside and stays out).
pub fn aabb_clip(
    from: (f64, f64, f64),
    to: (f64, f64, f64),
    min: (f64, f64, f64),
    max: (f64, f64, f64),
) -> Option<((f64, f64, f64), f64)> {
    let (fx, fy, fz) = from;
    let dx = to.0 - fx;
    let dy = to.1 - fy;
    let dz = to.2 - fz;
    let (d0, d1, d2) = (1.0 / dx, 1.0 / dy, 1.0 / dz);
    // the x slab (javap: (min - from) * inverse per axis, min/max fold)
    let t1x = (min.0 - fx) * d0;
    let t2x = (max.0 - fx) * d0;
    let t1y = (min.1 - fy) * d1;
    let t2y = (max.1 - fy) * d1;
    let t1z = (min.2 - fz) * d2;
    let t2z = (max.2 - fz) * d2;
    let tmin = t1x.min(t2x).max(t1y.min(t2y)).max(t1z.min(t2z));
    let tmax = t1x.max(t2x).min(t1y.max(t2y)).min(t1z.max(t2z));
    // the javap gate: `!(tmax < tmin) && tmin <= 1.0 && tmax >= 0.0`
    if tmax < tmin || tmin > 1.0 || tmax < 0.0 {
        return None;
    }
    let t = tmin.max(0.0);
    Some(((fx + dx * t, fy + dy * t, fz + dz * t), t))
}

/// THE CROSSHAIR'S MOB PICK (javap `LocalPlayer.pick`'s entity half —
/// `ProjectileUtil.getEntityHitResult` over the shooter's query box):
/// the ray from the eye along the view, each mob's bbox clipped, the
/// NEAREST candidate wins, and the hit must sit inside the search
/// sphere (the `Mth.square(searchRange)` gate — searchRange =
/// max(block 4.5, entity 3.0)). Returns the mob's index + the hit's
/// distance² from the eye (the closer-than-the-block compare's own
/// term). The start-inside-a-box branch speaks the vanilla's
/// `contains(start)` verdict (t = 0, the own start).
pub fn pick_entity(
    mobs: &[Zombie],
    eye: (f64, f64, f64),
    view: (f64, f64, f64),
    search_range: f64,
) -> Option<(usize, f64)> {
    let end = (
        eye.0 + view.0 * search_range,
        eye.1 + view.1 * search_range,
        eye.2 + view.2 * search_range,
    );
    let square = search_range * search_range;
    let mut best: Option<(usize, f64)> = None;
    for (i, mob) in mobs.iter().enumerate() {
        if mob.death_time > 0 {
            continue; // the corpses never pick (the CAN_BE_PICKED gate)
        }
        let (bx0, by0, bz0, bx1, by1, bz1) = mob.box_at();
        let min = (bx0, by0, bz0);
        let max = (bx1, by1, bz1);
        // the eye inside the box: the vanilla contains(start) law
        let inside = eye.0 >= min.0
            && eye.0 <= max.0
            && eye.1 >= min.1
            && eye.1 <= max.1
            && eye.2 >= min.2
            && eye.2 <= max.2;
        let d2 = if inside {
            0.0
        } else {
            match aabb_clip(eye, end, min, max) {
                Some((point, _)) => {
                    let (px, py, pz) = point;
                    let (ex, ey, ez) = (px - eye.0, py - eye.1, pz - eye.2);
                    ex * ex + ey * ey + ez * ez
                }
                None => continue,
            }
        };
        if d2 < square && best.map_or(true, |(_, b)| d2 < b) {
            best = Some((i, d2));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The tests' night tick — the MONSTERS_BURN track says false, so
    /// the legacy behaviors (the ambient draw cadence, the goal draws)
    /// run bit-for-bit as they did before the burn lane landed.
    const NIGHT: i64 = 18_000;
    /// The tests' noon tick — the burn window's heart.
    const NOON: i64 = 6_000;

    fn player_at(x: f64, y: f64, z: f64) -> PlayerView {
        PlayerView {
            x,
            y,
            z,
            eye_y: y + crate::body::STANDING_EYE,
            dead: false,
            width: crate::body::WIDTH,
        }
    }

    fn ground_world() -> World {
        let world = World::new(1, 42);
        world
    }

    /// A 4x4-chunk stone floor with open sky — wide enough that the
    /// stroll goal's +-10 ring can never walk the zombie off the edge
    /// (the void's below-world STONE reads sky 0 and would silently
    /// mask the burn gates).
    fn wide_flat_world() -> World {
        let mut world = World::new(4, 42);
        for x in 0..world.size_blocks() {
            for z in 0..world.size_blocks() {
                for y in 5..crate::world::CHUNK_Y as i32 {
                    world.set(x, y, z, crate::world::AIR);
                }
                for y in 0..5 {
                    world.set(x, y, z, crate::world::STONE);
                }
            }
        }
        world
    }

    /// The melee gate's own threshold (javap: `bbWidth*2*bbWidth*2 +
    /// target.width` — for the zombie: 1.44 + 0.6 = 2.04).
    #[test]
    fn the_melee_range_law_speaks_2_04() {
        let mut z = Zombie::new(1, 0.0, 0.0, 0.0, 7);
        // 1.4 blocks away: dist² 1.96 < 2.04 — inside the melee ring
        assert!(z.within_melee_range(&player_at(1.4, 0.0, 0.0)));
        // 1.44 blocks: dist² 2.0736 — outside it
        assert!(!z.within_melee_range(&player_at(1.44, 0.0, 0.0)));
        // the law is horizontal-only (the y rides the box distance)
        let _ = &mut z;
    }

    /// The ARMOR LAW (CombatRules.getDamageAfterAbsorb over the
    /// zombie's 2.0 armor, toughness 0.0): `f6 = clamp(2 - dmg/2,
    /// 0.4, 20)`, the survivor `dmg * (1 - f6/25)` — a 2.0 fist lands
    /// 1.92 through the armor.
    #[test]
    fn the_armor_law_speaks_combat_rules() {
        let mut z = Zombie::new(1, 0.0, 0.0, 0.0, 7);
        z.hurt(2.0);
        assert!((z.health - (20.0 - 1.92)).abs() < 1e-4, "health {}", z.health);
        assert_eq!(z.hurt_time, HURT_DURATION_TICKS, "the 10-tick flash");
        // the walk store's own hurt kick (the 1.5 speed the setSpeed
        // law banks — the sampler's own min(.., 1.0) clamp rides the
        // render read, the STORE carries the kick verbatim)
        assert_eq!(z.walk.speed, crate::body::HURT_WALK_SPEED);
    }

    /// `LivingEntity.knockback` verbatim: the impulse rides the minus
    /// of the normalized (x, 0, z), the halves, the grounded bump
    /// `min(0.4, vy/2 + strength)`.
    #[test]
    fn the_knockback_pushes_away_and_halves() {
        let mut z = Zombie::new(1, 0.0, 0.0, 0.0, 7);
        z.on_ground = true;
        z.vy = 0.0;
        // the attacker at +x: the (x, z) runs attacker-minus-victim,
        // the minus law pushes the mob to -x (away)
        z.knockback(0.4, 2.0, 0.0);
        assert!((z.vx - (-0.4)).abs() < 1e-9, "vx {}", z.vx);
        assert!((z.vy - 0.4).abs() < 1e-9, "vy {}", z.vy);
        // the grounded bump clamps at 0.4 even over a big strength
        let mut z2 = Zombie::new(2, 0.0, 0.0, 0.0, 7);
        z2.on_ground = true;
        z2.knockback(2.0, 0.0, 1.0);
        assert!(z2.vy <= 0.4 + 1e-9, "vy {}", z2.vy);
    }

    /// THE DEATH CLOCK (LivingEntity.tickDeath): the health at zero
    /// arms the clock, deathTime++ per tick walks 1..20, the dying
    /// never re-hurt, the corpse stops thinking (no attacks, no AI).
    #[test]
    fn the_death_clock_walks_to_removal() {
        let mut z = Zombie::new(1, 0.0, 0.0, 0.0, 7);
        let world = ground_world();
        z.hurt(1000.0);
        assert!(z.health <= 0.0, "the hit kills");
        // the dying never re-hurt (the isDeadOrDying gate)
        let health_after = z.health;
        z.hurt(5.0);
        assert_eq!(z.health, health_after);
        // the clock walks 1, 2, ... and the corpse never attacks
        let ev = z.tick(&world, Some(&player_at(1.0, 0.0, 0.0)), NIGHT);
        assert_eq!(z.death_time, 1);
        assert!(ev.attack.is_none() && !ev.ambient);
        for _ in 0..30 {
            z.tick(&world, None, NIGHT);
        }
        assert_eq!(z.death_time, 31, "the caller removes at 20 (retain <20)");
    }

    /// THE AMBIENT VOICE (Mob.baseTick's own gate): the clock starts
    /// at -80, the `nextInt(1000) < ambientSoundTime++` draw fires
    /// and resets to -80 — the groans arrive roughly every 540 ticks.
    #[test]
    fn the_ambient_clock_fires_and_resets() {
        let mut z = Zombie::new(1, 0.0, 0.0, 0.0, 7);
        let world = ground_world();
        let mut fires = 0;
        for _ in 0..6000 {
            if z.tick(&world, None, NIGHT).ambient {
                fires += 1;
            }
        }
        assert!((30..=70).contains(&fires), "ambient fires {fires} in 6000");
    }

    /// A flat stone pad (the chase test's own arena — the terrain
    /// generator's hills would eat the line-of-sight ray): five solid
    /// layers, the air above, the surface at y = 5.
    fn flat_world() -> World {
        let mut world = World::new(1, 42);
        for x in 0..world.size_blocks() {
            for z in 0..world.size_blocks() {
                for y in 5..world.size_blocks() {
                    world.set(x, y, z, crate::world::AIR);
                }
                for y in 0..5 {
                    world.set(x, y, z, crate::world::STONE);
                }
            }
        }
        world
    }

    /// THE CHASE (the goal machine's own walk): the zombie yaws at the
    /// player, closes the gap on its attribute speed, and lands the
    /// 3.0-damage hit inside the melee ring (the LOS rides the pad's
    /// clear ray).
    #[test]
    fn the_zombie_chases_and_hits_the_player() {
        let world = flat_world();
        let mut z = Zombie::new(1, 8.0, 5.0, 8.0, 7);
        z.on_ground = true;
        let player = player_at(5.0, 5.0, 8.0); // 3 blocks west, on the pad
        let mut landed = None;
        for _ in 0..400 {
            let ev = z.tick(&world, Some(&player), NIGHT);
            if let Some(atk) = ev.attack {
                assert!((atk.damage - 3.0).abs() < 1e-9, "damage {}", atk.damage);
                landed = Some(atk);
                break;
            }
        }
        assert!(landed.is_some(), "the melee hit never landed (x {})", z.x);
        let atk = landed.unwrap();
        // the knockback direction runs victim-minus-attacker
        assert!(atk.dir_x < 0.0, "the player sits west of the zombie");
        // THE RAISED ARMS (ZombieAttackGoal.tick's own law): at the hit
        // the fresh 20-tick cooldown speaks aggressive=false — the
        // arms raise when the cooldown walks under half (the next
        // wind-up), 5+ raise ticks in
        assert!(!z.aggressive, "the fresh cooldown lowers the arms");
        let mut raised = false;
        for _ in 0..30 {
            if z.tick(&world, Some(&player), NIGHT).attack.is_some() {
                raised = z.aggressive || raised;
                break;
            }
            if z.aggressive {
                raised = true;
                break;
            }
        }
        assert!(raised, "the arms raise on the wind-up");
    }

    /// THE FOOTSTEP CADENCE (Entity.move's own law over the walking
    /// zombie): the chase walk accumulates moveDist at the collided
    /// horizontal × 0.6, the step fires past nextStep (the ctor's own
    /// 1.0F) over solid ground, the count stays inside the whole-unit
    /// bound and the first step never speaks before the ctor's 1.0F.
    #[test]
    fn the_walking_zombie_speaks_the_footstep_cadence() {
        let world = flat_world();
        let mut z = Zombie::new(1, 8.0, 5.0, 8.0, 7);
        z.on_ground = true;
        let player = player_at(2.0, 5.0, 8.0); // 6 blocks west, on the pad — the walk crosses the ctor's 1.0F
        let mut steps = 0;
        let mut first_step_move_dist = None;
        for _ in 0..400 {
            let ev = z.tick(&world, Some(&player), NIGHT);
            if ev.step {
                steps += 1;
                if first_step_move_dist.is_none() {
                    first_step_move_dist = Some(z.move_dist);
                }
            }
            if ev.attack.is_some() {
                break;
            }
        }
        let md = z.move_dist;
        assert!(md > 1.0, "the walk accumulated the vanilla law ({md})");
        let first = first_step_move_dist.expect("the walk spoke at least one step");
        assert!(
            first > 1.0,
            "the ctor's own 1.0F gates the first step ({first})"
        );
        assert!(
            steps <= md as i32 as usize,
            "the cadence bound: {steps} steps for moveDist {md}"
        );
        // the reset rode nextStep(): the last threshold sits one whole
        // unit past the walked distance
        assert!(
            z.next_step > md - 1.0 && z.next_step <= md + 1.0,
            "the reset law: nextStep {} for moveDist {}",
            z.next_step,
            md
        );
    }

    /// THE AIRBORNE GATE (vibrationAndSoundEffectsFromBlock's own
    /// verdict): the hovered zombie's walk accumulates moveDist but the
    /// steps stay silent — the feet left the ground — and the landing
    /// speaks the whole accumulated crossing at once (the reset rides
    /// nextStep()).
    #[test]
    fn the_airborne_zombie_never_steps_until_the_landing() {
        let world = flat_world();
        let mut z = Zombie::new(1, 8.0, 5.0, 8.0, 7);
        z.on_ground = true;
        let player = player_at(2.0, 5.0, 8.0);
        let mut steps = 0;
        for _ in 0..25 {
            // the hover-leg: the jump impulse re-applied every tick —
            // the chase still drives the horizontal walk
            z.vy = crate::body::JUMP_POWER as f64;
            if z.tick(&world, Some(&player), NIGHT).step {
                steps += 1;
            }
        }
        assert!(
            z.move_dist > 1.0,
            "the air walk accumulated the law ({})",
            z.move_dist
        );
        assert_eq!(steps, 0, "the airborne gate holds the steps");
        // the landing: the feet touch, the whole crossing speaks
        for _ in 0..120 {
            if z.tick(&world, Some(&player), NIGHT).step {
                steps += 1;
            }
        }
        assert!(steps >= 1, "the landing fired the accumulated step");
    }

    /// THE PICK's own slab test (javap `AABB.clip`): the ray through
    /// the box hits, the parallel axis folds honestly, the miss and
    /// the start-inside branch speak their verdicts.
    #[test]
    fn the_aabb_clip_hits_and_misses() {
        let min = (-0.3, 0.0, -0.3);
        let max = (0.3, 1.95, 0.3);
        // a ray through the box's middle, from 5 blocks out
        let hit = aabb_clip((-5.0, 1.0, 0.0), (5.0, 1.0, 0.0), min, max);
        let (p, t) = hit.expect("the ray enters");
        assert!((p.0 - (-0.3)).abs() < 1e-9, "entry x {}", p.0);
        assert!((t - 4.7 / 10.0).abs() < 1e-9, "t {t}");
        // the parallel view along the box's plane misses it
        assert!(aabb_clip((5.0, 1.0, 5.0), (5.0, 1.0, -5.0), min, max).is_none());
        // the start inside speaks t = 0 at the own position
        let inside = aabb_clip((0.0, 1.0, 0.0), (5.0, 1.0, 0.0), min, max);
        let (_, t) = inside.expect("inside");
        assert!(t.abs() < 1e-9, "inside t {t}");
    }

    /// THE CROSSHAIR'S PICK (ProjectileUtil's own verdict): the
    /// nearest box wins, the corpses never pick, the search sphere
    /// eats the far ring.
    #[test]
    fn the_pick_nearest_wins_and_corpses_never_pick() {
        let mut near = Zombie::new(1, 2.0, 0.0, 0.0, 7);
        let far = Zombie::new(2, 4.0, 0.0, 0.0, 7);
        near.y = 0.0;
        let mobs = vec![near, far];
        let eye = (0.0, 1.0, 0.0);
        let view = (1.0, -0.2 / 2.0, 0.0); // level-out: the boxes at y 0..1.95
        // aim level at the bodies' middle (y 1.0 eye, the ray rides x)
        let view = (1.0, 0.0, 0.0);
        let picked = pick_entity(&mobs, eye, view, 4.5);
        assert_eq!(picked.unwrap().0, 0, "the nearer zombie wins");
        // the dead never pick: kill the near one, the far speaks
        let mut dead = Zombie::new(3, 2.0, 0.0, 0.0, 7);
        dead.health = 0.0;
        dead.death_time = 5;
        let mobs = vec![dead, Zombie::new(4, 4.0, 0.0, 0.0, 7)];
        let picked = pick_entity(&mobs, eye, view, 4.5);
        assert_eq!(picked.unwrap().0, 1, "the corpse never picks");
        // beyond the search sphere: nothing
        let mobs = vec![Zombie::new(5, 10.0, 0.0, 0.0, 7)];
        assert!(pick_entity(&mobs, eye, view, 4.5).is_none());
    }

    /// THE SPAWNER'S SURFACE LAW: the attempt lands on the topmost
    /// solid's head-room, inside the 24..48 ring, and the day gate
    /// refuses (night=false speaks nothing).
    #[test]
    fn the_spawner_lands_on_the_surface_with_head_room() {
        let world = World::new(4, 42);
        let (px, pz) = (32.0, 32.0);
        let mut spawned = None;
        for seed in 0u64..64 {
            let mut mobs = Vec::new();
            let mut next_id = 1;
            let mut rng = Lcg::new(seed);
            if try_spawn(&mut mobs, &mut next_id, px, pz, &world, &mut rng, true) {
                spawned = Some(mobs.pop().unwrap());
                break;
            }
        }
        let z = spawned.expect("a night attempt lands within 64 seeds");
        // the ring law: 24..48 blocks from the player
        let dx = z.x - px;
        let dz = z.z - pz;
        let d = (dx * dx + dz * dz).sqrt();
        assert!(
            d >= SPAWN_MIN_DIST - 1.5 && d <= SPAWN_MAX_DIST + 1.5,
            "ring distance {d}"
        );
        // the surface law: the feet ride the topmost solid + 1
        let (bx, bz) = (z.x.floor() as i32, z.z.floor() as i32);
        let mut top = 0;
        for y in (0..world.size_blocks()).rev() {
            if crate::body::solid(&world, bx, y, bz) {
                top = y;
                break;
            }
        }
        assert_eq!(z.y, top as f64 + 1.0, "the feet on the surface");
        // the head-room law: two free cells above
        assert!(!crate::body::solid(&world, bx, top + 1, bz));
        assert!(!crate::body::solid(&world, bx, top + 2, bz));
        // the day gate: night=false never spawns
        let mut mobs = Vec::new();
        let mut next_id = 1;
        let mut rng = Lcg::new(99);
        assert!(!try_spawn(
            &mut mobs, &mut next_id, px, pz, &world, &mut rng, false
        ));
        assert!(mobs.is_empty());
    }

    /// THE DEATH LOOT's own distribution (the zombie table's pools:
    /// the flesh's uniform 0..2, the player-kill rare's 0.025 chance
    /// over the THREE unweighted entries, the no-player-kill gate).
    #[test]
    fn the_death_loot_speaks_the_table() {
        let flesh = wire_id("rotten_flesh").expect("the flesh rides the wire");
        let iron = wire_id("iron_ingot").expect("the ingot rides the wire");
        let carrot = wire_id("carrot").expect("the carrot rides the wire");
        let potato = wire_id("potato").expect("the potato rides the wire");
        let rares = [iron, carrot, potato];
        let n = 20000u32;
        let (mut zeros, mut ones, mut twos, mut rares_n) = (0i32, 0i32, 0i32, 0i32);
        for seed in 0..n {
            let mut z = Zombie::new(1, 0.0, 0.0, 0.0, seed as u64);
            z.health = 0.0; // the dead body (the caller's contract)
            let loot = z.death_loot(true);
            let flesh_found = loot.iter().find(|(item, _)| *item == flesh);
            match flesh_found {
                Some((_, 1)) => ones += 1,
                Some((_, 2)) => twos += 1,
                Some((_, c)) => panic!("the flesh count {c} escapes 0..2"),
                None => zeros += 1,
            }
            for (item, count) in &loot {
                if *item != flesh {
                    assert!(
                        rares.contains(item) && *count == 1,
                        "the rare speaks one of the three, count 1"
                    );
                    rares_n += 1;
                }
            }
        }
        // the uniform law: the three counts split ~evenly (the wide
        // band over the seed spread)
        let third = (n / 3) as i32;
        assert!(((third - 700)..=(third + 700)).contains(&zeros), "the zero band {zeros}");
        assert!(((third - 700)..=(third + 700)).contains(&ones), "the one band {ones}");
        assert!(((third - 700)..=(third + 700)).contains(&twos), "the two band {twos}");
        // the rare: 0.025 of the player kills (the wide band)
        let expect = n as f64 * RARE_DROP_CHANCE as f64;
        assert!(((expect - 70.0)..=(expect + 70.0)).contains(&(rares_n as f64)), "the rare band {rares_n}");
        // the no-player-kill gate: the rare never fires
        let mut rare_without_player = 0;
        for seed in 0..n {
            let mut z = Zombie::new(1, 0.0, 0.0, 0.0, seed as u64);
            z.health = 0.0;
            for (item, _) in z.death_loot(false) {
                if item != flesh {
                    rare_without_player += 1;
                }
            }
        }
        assert_eq!(rare_without_player, 0, "the rare rides the player kill");
        // the living body never rolls (the caller's contract)
        let mut z = Zombie::new(1, 0.0, 0.0, 0.0, 7);
        assert!(z.death_loot(true).is_empty());
    }

    /// THE SUN'S BRAND (Mob.aiStep's BURN_IN_DAYLIGHT ->
    /// burnUndead -> isSunBurnTick -> igniteForSeconds(8)): a
    /// bare-headed zombie on the open surface at noon ignites within
    /// a few seconds' draws, the brand rides the max(160) law, and
    /// the fire clock deals the armor-folded 0.94 a second.
    #[test]
    fn the_sun_brands_the_bare_head() {
        let world = wide_flat_world();
        let mut z = Zombie::new(1, 32.0, 5.0, 32.0, 7);
        let mut ignited_at = None;
        for t in 0..300 {
            z.tick(&world, None, NOON);
            if z.fire_ticks > 0 {
                ignited_at = Some(t);
                break;
            }
        }
        let ignited_at = ignited_at.expect("the noon surface ignites the zombie");
        assert!(ignited_at < 300, "the brand lands inside the window");
        assert_eq!(z.fire_ticks, 160, "igniteForSeconds(8) = 160 exactly");
        // the re-ignite rides max(): the 4%-per-tick roll tops the
        // brand up faster than the 1-per-tick drain — the fire stays
        // lit across its own 160-tick span (P(silent span) ~ 0.002
        // for this seed)
        let mut ever_died_out = false;
        for _ in 0..160 {
            z.tick(&world, None, NOON);
            if z.fire_ticks == 0 {
                ever_died_out = true;
            }
        }
        assert!(!ever_died_out, "the noon surface keeps the brand lit");
        // the fire clock: the health walked off 20.0 by the onFire
        // hurts (0.94 per landed dose through the 2.0 armor)
        assert!(z.health < ZOMBIE_MAX_HEALTH, "the fire hurts");
        let burn = ZOMBIE_MAX_HEALTH - z.health;
        let doses = (burn / 0.94).round();
        assert!((doses * 0.94 - burn).abs() < 0.01, "the armor fold {burn}");
    }

    /// THE NIGHT HIDES THEM — the MONSTERS_BURN track false, the same
    /// noon-shaped surface, three hundred ticks: no brand, no fire
    /// hurt, and the roll's draw never fires (the short-circuit keeps
    /// the RNG stream exactly the pre-burn shape).
    #[test]
    fn the_night_hides_the_zombie_from_the_sun() {
        let world = wide_flat_world();
        let mut z = Zombie::new(1, 32.0, 5.0, 32.0, 7);
        for _ in 0..300 {
            let ev = z.tick(&world, None, NIGHT);
            assert!(!ev.fire_hurt && z.fire_ticks == 0);
        }
        assert_eq!(z.health, ZOMBIE_MAX_HEALTH);
    }

    /// THE DUSK LIGHT EDGE — the two gates interact: the MONSTERS_BURN
    /// track holds true past 12000, but the sky-light ramp drains the
    /// raw brightness (sky - darken) below the magic gate mid-fade.
    /// Tick 12000 brands (raw 15, the 4%-per-tick roll); tick 12600
    /// never does — the darken has walked to 4 (raw 11), whose magic
    /// f32s to 0.4074 < 0.5 (the raw-12 step right below still passes
    /// at 0.50000006 — the f32 identity both the client and the
    /// bytecode share). The window's true dusk edge is the LIGHT law's,
    /// not the track's.
    #[test]
    fn the_dusk_light_edge_frees_the_zombie() {
        let world = wide_flat_world();
        // 12000: the track says burn AND the light still passes — the
        // brand lands inside 400 draws (the 4%-per-tick roll)
        let mut z = Zombie::new(1, 32.0, 5.0, 32.0, 7);
        let mut ignited = false;
        for _ in 0..400 {
            z.tick(&world, None, 12_000);
            if z.fire_ticks > 0 {
                ignited = true;
                break;
            }
        }
        assert!(ignited, "12000 still brands");
        // 12600: the track still true (12542 is its own edge) but raw
        // 11 -> magic 0.4074 -> the strict gate refuses, zero draws
        // reach a pass
        let mut z2 = Zombie::new(2, 32.0, 5.0, 32.0, 11);
        for _ in 0..400 {
            z2.tick(&world, None, 12_600);
            assert_eq!(z2.fire_ticks, 0, "the raw-11 dusk never brands");
        }
        assert_eq!(z2.health, ZOMBIE_MAX_HEALTH);
    }

    /// THE CANOPY SHIELD — a single leaves block above the eye dims
    /// the sky channel to 14: canSeeSky fails (the track's == 15 law)
    /// and the zombie never ignites, however long the noon lasts.
    #[test]
    fn the_canopy_shields_the_zombie() {
        let mut world = wide_flat_world();
        // a leaves ROOF two over the eye column, across the whole
        // reach (the eye cell sits at y 6; the leaf layer at 8 dims
        // the direct column to 14 wherever the stroll wanders)
        let size = world.size_blocks();
        for x in 0..size {
            for z in 0..size {
                world.set(x, 8, z, crate::world::LEAVES);
            }
        }
        let mut z = Zombie::new(1, 32.0, 5.0, 32.0, 7);
        for _ in 0..400 {
            z.tick(&world, None, NOON);
            assert_eq!(z.fire_ticks, 0, "the dimmed sky never brands");
        }
        assert_eq!(z.health, ZOMBIE_MAX_HEALTH);
    }

    /// THE WATER DROWNS THE BRAND — the inside-block effect clears
    /// the fire (clearFire = remaining 0) and the transition speaks
    /// the fizz at 0.7 volume / the 1.6 +- 0.4 pitch fold; the wet
    /// body never re-ignites while it stands in the pool.
    #[test]
    fn the_water_drowns_the_brand() {
        let mut world = wide_flat_world();
        // a walled pool cage: the interior cell (32, 5..6, 32) water,
        // the ring walls 4 tall so the stroll can never walk the zombie
        // out — the water effect clears the brand and keeps it cleared
        for wy in 5..9 {
            for wx in 31..34 {
                for wz in 31..34 {
                    if wx == 32 && wz == 32 && wy <= 6 {
                        world.set(wx, wy, wz, crate::world::WATER);
                    } else if wx != 32 || wz != 32 {
                        world.set(wx, wy, wz, crate::world::STONE);
                    }
                }
            }
        }
        let mut z = Zombie::new(1, 32.5, 5.0, 32.5, 7);
        z.fire_ticks = 160;
        let ev = z.tick(&world, None, NOON);
        assert_eq!(z.fire_ticks, 0, "the water effect cleared the fire");
        let pitch = ev.extinguished.expect("the fizz spoke");
        assert!(
            (1.6 - 0.4..=1.6 + 0.4).contains(&pitch),
            "the pitch fold {pitch}"
        );
        // the wet body never re-ignites
        for _ in 0..200 {
            z.tick(&world, None, NOON);
            assert_eq!(z.fire_ticks, 0, "the pool keeps it out");
        }
    }

    /// THE CORPSE KEEPS ITS CLOCK — the fire clock runs before the
    /// death gate (baseTick precedes tickDeath), the dying refuse the
    /// onFire hurt, and the brand drains out over the corpse.
    #[test]
    fn the_corpse_keeps_its_fire_clock() {
        let world = flat_world();
        let mut z = Zombie::new(1, 8.0, 5.0, 8.0, 7);
        z.health = 0.0;
        z.fire_ticks = 41;
        for _ in 0..41 {
            let ev = z.tick(&world, None, NOON);
            assert!(!ev.fire_hurt, "the dying refuse the hurt");
            assert_eq!(ev.extinguished, None);
        }
        assert_eq!(z.fire_ticks, 0, "the clock drained to zero");
        assert_eq!(z.death_time, 41, "the death clock walked on");
    }

    /// THE FIRE DEATH SPEAKS ITS EVENT — the last onFire dose flips
    /// the fire_death flag (the caller rolls the loot with the
    /// player-kill gate closed) and the flesh table answers.
    #[test]
    fn the_fire_death_flips_its_event() {
        let world = wide_flat_world();
        let mut z = Zombie::new(1, 32.0, 5.0, 32.0, 7);
        z.fire_ticks = 160;
        z.health = 0.5;
        let mut died = false;
        for _ in 0..25 {
            let ev = z.tick(&world, None, NOON);
            if ev.fire_death {
                died = true;
                break;
            }
        }
        assert!(died, "the fire kills inside its first second");
        assert!(z.health <= 0.0);
        // the fire death's loot: killed_by_player false — the rare
        // pool never fires (the death_loot contract the loot lane pins)
        let flesh = wire_id("rotten_flesh").expect("the rotten flesh rides the wire");
        assert!(z.death_loot(false).iter().all(|(item, _)| *item == flesh));
    }

    /// THE TICK'S OWN HEAD SAVES THE PREV (javap `Entity.tick`'s `xo =
    /// x` fold): the ctor arms the spawn, the AI tick re-arms it before
    /// the walk moves the feet, and the render fold's lerp rides the
    /// half-way point at partial 0.5 while the partial >= 1.0 arm
    /// speaks the CURRENT feet bit for bit (the headless contract).
    #[test]
    fn the_tick_head_saves_the_prev_and_the_lerp_walks_it() {
        let world = wide_flat_world();
        let mut z = Zombie::new(1, 32.0, 5.0, 32.0, 7);
        // the ctor arm: the spawn is its own first prev
        assert_eq!((z.prev_x, z.prev_y, z.prev_z), (32.0, 5.0, 32.0));
        // the aggro walk: the player nearby pulls the chase goal — the
        // feet move this tick, the head saved the pre-move pose first
        let pv = PlayerView {
            x: 36.0,
            y: 5.0,
            z: 32.0,
            eye_y: 6.0,
            dead: false,
            width: 0.6,
        };
        let _ = z.tick(&world, Some(&pv), NOON);
        assert_eq!(z.prev_x, 32.0, "the pre-move feet the head saved");
        // the chase may arm over a few goals' rechecks — walk until the
        // feet MOVED, then the fold's laws speak on a real delta
        for _ in 0..60 {
            if (z.x - 32.0).abs() > 1e-6 {
                break;
            }
            let _ = z.tick(&world, Some(&pv), NOON);
        }
        assert!(
            (z.x - 32.0).abs() > 1e-6,
            "the chase moved the zombie: {}",
            z.x
        );
        let (lx, _, _) = z.lerped_pos(0.5);
        let half = z.prev_x + (z.x - z.prev_x) * 0.5;
        assert!((lx - half).abs() < 1e-12, "midpoint: {lx} vs {half}");
        assert_eq!(z.lerped_pos(1.0), (z.x, z.y, z.z));
        assert_eq!(z.lerped_pos(0.0), (z.prev_x, z.prev_y, z.prev_z));
    }

    /// THE HEAD POSE'S OWN RENDER FOLD (LivingEntityRenderer's
    /// `Mth.lerp(partialTicks, yHeadRotO, yHeadRot)` the netHeadYaw
    /// speaks): the AI's new look target the head EASES to — the
    /// previous tick's head carried `partial` of the way to the
    /// current, the 0.0 arm speaks the previous pose, the >= 1.0 arm
    /// the current bit for bit (the headless contract).
    #[test]
    fn the_head_pose_eases_to_the_look_target() {
        let mut z = Zombie::new(1, 10.0, 64.0, 10.0, 7);
        // the ctor arms the settled head: the O pair rides the spawn's
        // own 0 pose (the vanilla O fields start at the ctor's)
        assert_eq!(z.prev_head_yaw_off, 0.0);
        assert_eq!(z.prev_head_pitch, 0.0);
        // the AI swung the head to a look target this tick: the old
        // pose the O pair holds, the new the current speaks
        z.prev_head_yaw_off = 0.0;
        z.prev_head_pitch = 0.0;
        z.head_yaw_off = 40.0;
        z.head_pitch = 12.0;
        // the mid frame rides BETWEEN the stores
        let (yaw, pitch) = z.lerped_head_pose(0.5);
        assert_eq!(yaw, 20.0, "the head yaw eases half way");
        assert_eq!(pitch, 6.0, "the head pitch eases half way");
        // the exactness arms: the 0.0 prev pose, the >= 1.0 current
        assert_eq!(z.lerped_head_pose(0.0), (0.0, 0.0));
        assert_eq!(z.lerped_head_pose(1.0), (40.0, 12.0));
        assert_eq!(z.lerped_head_pose(1.5), (40.0, 12.0));
    }

    /// THE TICK HEAD SAVES THE HEAD'S O PAIR (LivingEntity's
    /// `yHeadRotO = yHeadRot` + Entity.baseTick's `xRotO = xRot`):
    /// the pre-look pose the tick's own head saved BEFORE the goals
    /// resolved a new look target — the render walks FROM it.
    #[test]
    fn the_tick_head_saves_the_head_pose() {
        let world = wide_flat_world();
        let mut z = Zombie::new(1, 32.0, 5.0, 32.0, 7);
        z.head_yaw_off = -30.0;
        z.head_pitch = 8.0;
        let _ = z.tick(&world, None, NOON);
        // whatever the goals resolved the head TO this tick, the O
        // pair holds the pre-look pose the lerp walks FROM
        assert_eq!(z.prev_head_yaw_off, -30.0, "the pre-look yaw saved");
        assert_eq!(z.prev_head_pitch, 8.0, "the pre-look pitch saved");
    }
}

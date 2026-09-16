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
//!   targetSelector prio 2 `NearestAttackableTargetGoal(Player, true)`;
//!   the animals' own registerGoals the panic's prio-1 flee fills (the
//!   Sheep's/Pig's `addGoal(1, new PanicGoal(this, 1.25))`, the
//!   AbstractCow's `addGoal(1, new PanicGoal(this, 2.0))` — the cow's
//!   goals the parent class fills) — the zombie's speaks none.
//! - `MeleeAttackGoal` — attackInterval 20 (the ctor's bipush 20; the
//!   canUse re-check gates every 20 ticks, the resetAttackCooldown
//!   refills 20), `canPerformAttack` = isTimeToAttack &&
//!   isWithinMeleeAttackRange && hasLineOfSight.
//! - `ZombieAttackGoal.tick` — raiseArmTicks++; `raiseArmTicks >= 5 &&
//!   getTicksUntilNextAttack() < getAttackInterval()/2` drives
//!   `setAggressive(true)` (the raised-arms law the render state
//!   speaks); `stop()` clears it.
//! - `RandomStrollGoal` — the canUse gate's interval 120 (bipush 120);
//!   the WaterAvoiding override's own getPosition the stroll target
//!   picks: the in-water arm the 15/7 land ring rides (LandRandomPos —
//!   the lift out of solid + the isWater word + the stable-destination
//!   ground), the null the super's raw pick answers, the dry arm the
//!   0.001 probability roll splits (the super's DefaultRandomPos the
//!   low roll keeps, the 10/7 land ring the rest walk — the ten-try
//!   loop `RandomPos.generateRandomPos` speaks, each try its
//!   three-draw direction the `generateRandomDirection` bytecode
//!   pins: x `nextInt(2x+1)-x`, y `nextInt(2z+1)-z`, z `nextInt(2x+1)-x`).
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

use crate::body::{
    collide, WalkAnimationState, AIR_FRICTION, BLOCK_FRICTION, CUBE_SPEED_NUMERATOR, GRAVITY,
    GROUND_DRAG, JUMP_POWER, VERTICAL_DRAG, WALK_BLEND, WALK_DISTANCE_SCALE,
};
use crate::drops::Lcg;
use crate::hand::SwingState;
use crate::world::World;

/// The ZOMBIE registration's own `sized(0.6F, 1.95F)` (EntityTypes).
pub const ZOMBIE_WIDTH: f64 = 0.6;
pub const ZOMBIE_HEIGHT: f64 = 1.95;
/// The PIG registration's own `sized(0.9F, 0.9F)` (EntityTypes) — the
/// second drawn body's own bbox (the flame ring's dims the render asks).
pub const PIG_WIDTH: f64 = 0.9;
pub const PIG_HEIGHT: f64 = 0.9;
/// The COW registration's own `sized(0.9F, 1.4F)` (EntityTypes clinit's
/// javap block — the third drawn body's own bbox the flame ring asks).
pub const COW_WIDTH: f64 = 0.9;
pub const COW_HEIGHT: f64 = 1.4;
/// The COW registration's own `eyeHeight(1.3F)` (the same block).
pub const COW_EYE_HEIGHT: f64 = 1.3;

/// The SHEEP registration's own `sized(0.8999999761581421F,
/// 1.2999999523162842F)` (EntityTypes clinit's own builder chain — the
/// fourth drawn body's bbox the flame ring asks).
pub const SHEEP_WIDTH: f64 = 0.9;
pub const SHEEP_HEIGHT: f64 = 1.3;
/// The SHEEP registration's own `eyeHeight(1.2350000143051147F)` (the
/// same chain).
pub const SHEEP_EYE_HEIGHT: f64 = 1.235;

/// `AgeableMob.getBabyStartAge` (javap: `return -24000` — the grow-up
/// countdown the child's birth arms; the age walks UP toward 0 the
/// aiStep's own walk speaks, the adult the zero crossing names).
pub const BABY_START_AGE: i32 = -24000;
/// `finalizeSpawnChildFromBreeding`'s own cooldown (javap: the
/// `setAge(6000)` the walk folds on BOTH parents — the breed delay
/// the aiStep's own decrement drains, the gate the feed arm reads).
pub const BREED_COOLDOWN_TICKS: i32 = 6000;
/// `AgeableMob.ageUp(IZ)`'s own forcedAgeTimer mark (the bipush 40 the
/// over-grow arm speaks — the HAPPY_VILLAGER sparkles the aiStep's
/// `% 4` cadence walks over the next 40 ticks, ten green flashes).
pub const FEED_FORCED_AGE_TIMER: i32 = 40;
/// `AgeableMob.setAgeLockedData`'s own ageLockParticleTimer mark (the
/// bipush 40 the toggle's tail speaks — the PAUSE/RESET particles the
/// aiStep's makeAgeLockedParticle walk spawns over the next 40 ticks
/// at the `% 2` cadence, twenty glint flashes; the window's own drain
/// the canUseGoldenDandelion re-use gate requires).
pub const AGE_LOCK_PARTICLE_WINDOW: i32 = 40;
/// `EatBlockGoal.start()`'s own window (the bipush 40 the
/// adjustedTickDelay speaks — the graze holds the head down and the
/// MOVE/LOOK/JUMP flags for the next 40 ticks; the consume the
/// tick's `== 4` landing answers at the window's tail).
pub const EAT_ANIMATION_TICKS: i32 = 40;
/// `EatBlockGoal.tick()`'s own consume landing (the adjustedTickDelay(4)
/// the post-decrement compare reads — the 36th tick of the 40-tick
/// window, the bite the block swap and the ate() walk answer).
pub const EAT_CONSUME_TICK: i32 = 4;
/// `EatBlockGoal.canUse()`'s own denominators (the ifeq branch the
/// isBaby word splits: `nextInt(adjustedTickDelay(isBaby ? 50 : 1000))
/// == 0` — the baby grazes twenty times the adult's own odds).
pub const EAT_GATE_BABY: u64 = 50;
pub const EAT_GATE_ADULT: u64 = 1000;
/// `Sheep.ate()`'s own ageUp (the bipush 60 the canAgeUp gate speaks —
/// the +1200-tick jump the grazing baby's growth rides, the zero
/// ceiling's crossing the grow-up answers).
pub const EAT_AGEUP_TICKS: i32 = 60;
/// `Sheep.getHeadEatPositionScale/getHeadEatAngleScale`'s own constants
/// (the cp_probe walk the ldc operands resolve): the position's 4.0
/// divisor the ease bands speak, the angle's 32.0 divisor the chew
/// wobble rides, `0.62831855F` (pi/5) the hold/peak the window keeps,
/// `0.21991149F * sin(f * 28.7F)` the graze's own chew, `0.017453292F`
/// the idle xRot's deg->rad fold.
pub const EAT_POSITION_DIV: f32 = 4.0;
pub const EAT_ANGLE_DIV: f32 = 32.0;
pub const EAT_ANGLE_BASE: f32 = 0.62831855;
pub const EAT_ANGLE_AMP: f32 = 0.21991149;
pub const EAT_ANGLE_FREQ: f32 = 28.7;
pub const EAT_IDLE_RAD: f32 = 0.017453292;
/// `EatBlockGoal.IS_EDIBLE`'s own tag (`minecraft:edible_for_sheep` —
/// the tag json the jar keeps: short_grass, short_dry_grass,
/// tall_dry_grass, fern). The repo's world table admits none of the
/// four (NAME_OF keeps 27 ids, no plant block), the feet arm the
/// honest still answers — the below-grass-block arm the live one.
pub const EDIBLE_FOR_SHEEP: [&str; 4] =
    ["short_grass", "short_dry_grass", "tall_dry_grass", "fern"];
/// The BreedGoal's own 8.0 ring (the AABB inflate the goal's pairing
/// search speaks — the class source's constant the chicken/pig
/// registrations' `BreedGoal(animal, 1.0)` clinit entries name the
/// goal for; the canMate law the animal dump carries is the round's
/// own bytecode evidence, the ring the goal's own shape keeps).
pub const BREED_SEARCH_RANGE: f64 = 8.0;
/// THE FOLLOWPARENTGOAL'S OWN VOCABULARY (javap `FollowParentGoal` the
/// followparentgoal dump keeps — canUse/canContinueToUse/tick): the
/// scan the baby's own bbox `inflate(8.0, 4.0, 8.0)` names (the
/// getEntitiesOfClass ring — the same-class adults the box keeps),
/// the 9.0 the close ring speaks (`d0 < 9.0 -> false` the start
/// refuses, canContinueToUse's own `d < 9.0` the stop arms — the
/// 3.0 the squared word), the speed 1.1 the Sheep's own registerGoals
/// clinit keeps (`addGoal(4, new FollowParentGoal(this, 1.1))` —
/// prio 4, the stroll's 7 the priority owns). The 256.0 give-up ring
/// canContinueToUse speaks the re-arm's own box law folds — the
/// per-frame scan the parent's drift answers before the 16.0 ring
/// the navigation could ever speak.
pub const FOLLOW_PARENT_RANGE: f64 = 8.0;
pub const FOLLOW_PARENT_HEIGHT: f64 = 4.0;
pub const FOLLOW_PARENT_CLOSE_SQ: f64 = 9.0;
pub const FOLLOW_PARENT_SPEED: f64 = 1.1;
/// THE BREEDGOAL'S OWN VOCABULARY (javap `BreedGoal` the breedgoal
/// dump keeps — canUse/canContinueToUse/tick/breed/getFreePartner):
/// the partner scan the animal's own bbox `inflate(8.0)` names (the
/// getNearbyEntities ring — the same 8.0 BREED_SEARCH_RANGE the
/// registerGoals search speaks), the 9.0 the close ring speaks (the
/// tick's own `distanceToSqr(partner) < 9.0` the breed() trigger
/// keeps — the 3.0 the squared word), the 60 the court clock rides
/// (the tick's `loveTime >= adjustedTickDelay(60)` — the canUse
/// start's own zero the stop law resets), the speed 1.0 the Sheep's
/// own registerGoals clinit keeps (`addGoal(2, new
/// BreedGoal(this, 1.0))` — prio 2, the follow's 4 the priority
/// owns). The goal's own lifecycle the dump answers: canUse arms
/// while `isInLove()` and the nearest mate answers, the tick walks
/// `moveTo(partner, speedModifier)` and counts loveTime, the 60 miss
/// beyond the ring the stop+re-arm law resets (canContinueToUse's
/// own `loveTime < 60` the gate keeps), stop() the partner nulls and
/// the clock zeroes.
pub const BREED_CLOSE_SQ: f64 = 9.0;
pub const BREED_COURT_TICKS: i32 = 60;
pub const BREED_SPEED: f64 = 1.0;
/// THE PANICGOAL'S OWN VOCABULARY (javap `PanicGoal` the panicgoal
/// dump keeps — canUse/shouldPanic/findRandomPosition/start/
/// canContinueToUse/stop): the shouldPanic gate the last damage
/// source's own 40-tick memory speaks (`Entity.getLastDamageSource`'s
/// `tickCount - lastDamageStamp > 40 -> null` window the hurt stamp
/// arms), the causes the jar's own PANIC_CAUSES tag json names —
/// #panic_environmental_causes (in_fire/on_fire the fire lane's own
/// brand rides) plus mob_attack and #is_player_attack (the melee hits
/// speak) — EVERY hurt source this lane keeps is a member, the flee
/// target the `DefaultRandomPos.getPos(mob, 5, 4)` ring names (the
/// on-fire arm the `lookForWater(level, mob, 5)` lane speaks FIRST —
/// the water cell the flee walks TO), the speeds the registerGoals
/// clinits keep (the Sheep's/Pig's 1.25, the AbstractCow's 2.0 —
/// `addGoal(1, new PanicGoal(this, D))` each speaks, the zombie's own
/// registerGoals fills no panic goal the kind word keeps). The goal's
/// own lifecycle the dump answers: canUse arms while the memory lives
/// (the water arm first, else the random ring draw), start() the
/// moveTo + the isRunning word sets, canContinueToUse the
/// navigation-done walk keeps, stop() the isRunning clears (the
/// PathfinderMob.isPanicking fallback's own read — the brain lane the
/// repo never keeps, the running-PanicGoal walk the method's own else
/// answers).
pub const PANIC_DAMAGE_MEMORY_TICKS: i32 = 40;
pub const PANIC_RING_XZ: f64 = 5.0;
pub const PANIC_WATER_RING: i32 = 5;
pub const PANIC_SPEED_SHEEP: f64 = 1.25;
pub const PANIC_SPEED_COW: f64 = 2.0;
pub const PANIC_SPEED_PIG: f64 = 1.25;
/// `AgeableMob.getScale`'s own baby arm (the `isBaby() ? 0.5F : 1.0F`
/// the entity dims' getDefaultDimensions arm carries — the half the
/// bbox ask folds; the RENDER's age the dedicated baby models the
/// AgeableMobRenderer's babyModel field picks already speak — the
/// pose-scale fold the model swap replaced).
pub const BABY_RENDER_SCALE: f32 = 0.5;

/// THE WOOL BYTE'S OWN DECODE (Sheep.isSheared/getColor's own fold —
/// the 26.2 bytecode: `byteValue() & 0x10 != 0` the sheared bit rides,
/// `DyeColor.byId(byteValue() & 0xF)` the low nibble the color speaks).
pub const SHEEP_WOOL_SHEARED_BIT: u8 = 0x10;
pub const SHEEP_WOOL_COLOR_MASK: u8 = 0x0F;
/// THE DATA_WOOL_ID'S OWN INDEX — the SynchedEntityData id the class
/// tree speaks: Entity's ctor defines 8 accessors (0..7, the shared
/// flags 0), LivingEntity's clinit definesId 7 more in order FLAGS(8),
/// HEALTH(9), EFFECT_PARTICLES(10), EFFECT_AMBIENCE(11), ARROWS(12),
/// STINGERS(13), SLEEPING(14), AgeableMob BABY(15) + AGE_LOCKED(16),
/// and Sheep's own defineId lands the wool byte at 17.
pub const SHEEP_DATA_WOOL_ID: u8 = 17;

/// THE CUSTOM NAME'S OWN INDEX — the SynchedEntityData id the class
/// tree speaks: Entity's clinit defineId walk lands DATA_CUSTOM_NAME
/// at 2 (SHARED_FLAGS 0, AIR_SUPPLY 1, CUSTOM_NAME 2, NAME_VISIBLE 3,
/// SILENT 4, NO_GRAVITY 5, POSE 6, TICKS_FROZEN 7 — the walk the raw
/// disassembly speaks). The wire rides it as the OPTIONAL_COMPONENT
/// serializer (the flag byte, then the bare component tag only if
/// present) — the plain-text form a TAG_String, the structured form a
/// TAG_Compound whose `text` key carries the words. MobRenderer's
/// checkMagicName reads it for every body (the accessor is the
/// ENTITY's own — the sheep's jeb_ gate is only one consumer).
pub const DATA_CUSTOM_NAME_ID: u8 = 2;

/// THE NAMEPLATE'S OWN VISIBILITY WORD (Entity's clinit defineId walk
/// the same tree lands: SHARED_FLAGS 0, AIR_SUPPLY 1, CUSTOM_NAME 2,
/// NAME_VISIBLE 3 — the Boolean serializer the read rides). The
/// byte's own law (Entity.shouldShowName: `isCustomNameVisible()`
/// IS the gate's first arm — the entityData Boolean verbatim):
/// a named body whose server never raised the bit keeps its tag
/// unsaid; the raised bit arms the floating text the NameTagFeature
/// renderer draws.
pub const DATA_CUSTOM_NAME_VISIBLE: u8 = 3;

/// THE WOOL TINT'S OWN WORD (javap ColorLerper$Type.SHEEP.getColor's
/// law — the colorByDye map the ctor's lambda pre-speaks through
/// ColorLerper.getModifiedColor(dye, 0.75F)): WHITE never multiplies —
/// the method's own first branch returns the literal 0xFFE6E6E6 (the
/// 230-grey the vanilla white wool wears); every other dye's
/// textureDiffuseColor rides the 0.75F modifier per channel, the
/// Mth.floor + Mth.clamp(0,255) fold the bytecode speaks. The repo's
/// earlier wool rode the RAW diffuse ints — the table the class tree
/// reads is the MODIFIED one, the tint one step darker than the raw
/// word (red 191 not 255, white 230 not 255).
pub fn sheep_wool_tint(dye_id: u8) -> u32 {
    if dye_id == 0 {
        return 0xE6E6E6; // the WHITE branch's own literal (alpha dropped)
    }
    let c = DYE_TEXTURE_COLORS[(dye_id & 0x0F) as usize];
    let ch = |v: u32| ((v as f32 * 0.75).floor() as i32).clamp(0, 255) as u32;
    (ch(c >> 16) << 16) | (ch(c >> 8 & 0xFF) << 8) | ch(c & 0xFF)
}

/// THE JEB_ RAINBOW'S OWN WORD (javap ColorLerper.getLerpedColor's
/// law — the isJebSheep branch SheepRenderState.getWoolColor speaks,
/// the input the state's ageInTicks): the 16-color cycle walks the
/// MODIFIED table (25 ticks per color, the 400-tick full rainbow),
/// `i = Mth.floor(time)`, the window `(i / 25) % 16` and its `(i+1)`
/// successor, and the blend `t = ((i % 25) + Mth.frac(time)) / 25.0`
/// — the ramp that rides 0→1 across each 25-tick window. The lerp is
/// ARGB.srgbLerp's own component walk: `Mth.lerpInt(t, a, b) =
/// a + Mth.floor(t * (b - a))` per channel (the two alphas are both
/// 255 — the word drops them). The jeb_ sheep named by
/// MobRenderer.checkMagicName("jeb_") renders its fur through THIS
/// clock — the color never holds still.
pub fn jeb_rainbow_rgb(age_ticks: f32) -> u32 {
    let i = age_ticks.floor() as i32;
    let idx = (i / 25).rem_euclid(16) as usize;
    let next = ((i / 25) + 1).rem_euclid(16) as usize;
    let frac = age_ticks - age_ticks.floor();
    let t = ((i % 25) as f32 + frac) / 25.0;
    let a = sheep_wool_tint(idx as u8);
    let b = sheep_wool_tint(next as u8);
    // Mth.lerpInt's own walk: a + floor(t * (b - a)) per channel —
    // the floor is Math.floor semantics (the negative half matters:
    // t * (b - a) dips below zero when the target darkens)
    let lerp = |x: u32, y: u32| -> u32 {
        let d = y as i32 - x as i32;
        (x as i32 + (t * d as f32).floor() as i32).clamp(0, 255) as u32
    };
    (lerp(a >> 16, b >> 16) << 16)
        | (lerp(a >> 8 & 0xFF, b >> 8 & 0xFF) << 8)
        | lerp(a & 0xFF, b & 0xFF)
}

/// THE SHEAR LOOT'S OWN NAME LAW (javap DyeColor's clinit — the 16
/// enum names the ctor's own String argument speaks, WHITE ordinal 0
/// .. BLACK ordinal 15, the lowercase the registry path rides):
/// the shearing loot tables the 26.2 jar's data/minecraft
/// /loot_table/shearing/sheep/<name>.json speaks — one table per
/// dye, the item `<name>_wool` the table's own entry names, the
/// count the uniform 1.0..3.0 rolls (the Mth.nextInt inclusive law
/// the block_drops generator pins: min + nextInt(max - min + 1)).
pub const DYE_NAMES: [&str; 16] = [
    "white",
    "orange",
    "magenta",
    "light_blue",
    "yellow",
    "lime",
    "pink",
    "gray",
    "light_gray",
    "cyan",
    "purple",
    "blue",
    "brown",
    "green",
    "red",
    "black",
];

/// The mob.interact table's own verdict (javap `InteractionResult` —
/// the refactor the 26.2 jar speaks: `Success` carries a
/// `SwingSource` (NONE / CLIENT / SERVER), the clinit's three
/// constructs SUCCESS = CLIENT, SUCCESS_SERVER = SERVER, CONSUME =
/// NONE). The walk's PASS answers the honest fall-through the right
/// press speaks; the Shear carries the loot the shear rolled — the
/// copyWithCount(1) singles the lambda's own loop spawns (one vec
/// entry per spawned ItemEntity, count 1 each).
#[derive(Debug, Clone, PartialEq)]
pub enum MobInteract {
    /// the empty table's own PASS (the walk falls to the item lane,
    /// the BLOCK case never reached — the vanilla never places
    /// through the body the crosshair picked)
    Pass,
    /// SUCCESS_SERVER — the shear the shears' gate speaks; the vec
    /// rides the shearing/sheep/<color> table's roll, each entry one
    /// single-item entity (the lambda$0's copyWithCount(1) law)
    Shear(Vec<i32>),
    /// the FOOD arm the `Animal.mobInteract` super speaks (the 26.2
    /// bytecode: `isFood(item)` hit, the ADULT arm `getAge() == 0 &&
    /// canFallInLove()` — `usePlayerItem` + `setInLove(player)` +
    /// `playEatingSound()`, the SUCCESS_SERVER return — the love arm
    /// rides `love: true`; the BABY arm the `canAgeUp` gate arms —
    /// `usePlayerItem` + `ageUp(getSpeedUpSecondsWhenFeeding(-age),
    /// true)` the ten-percent growth law walks + `playEatingSound()`,
    /// the SUCCESS return — rides `grow: true` (the grow-up the
    /// ageUp's own walk carries, the swing the SUCCESS verdict
    /// speaks, the heart burst the setInLove arm the baby never
    /// rides); the fallbacks the false pair keeps — in love already,
    /// the press the CONSUME verdict consumes, `SwingSource.NONE` no
    /// swing). The `usePlayerItem` consume the repo's item lane never
    /// speaks (no inventory — the held stack the caller's stand-in
    /// keeps). The eat voice the `playEatingSound` override the jar
    /// keeps — the PIG only (the Sheep/Cow the base's own empty law
    /// keeps silent), the caller's sound lane plays it.
    Feed {
        /// SUCCESS_SERVER (the setInLove arm) vs CONSUME (the
        /// already-in-love fallback the canFallInLove gate refuses)
        love: bool,
        /// SUCCESS (the baby arm) — the grow-up the ageUp law walked
        /// this press, the swing the SUCCESS verdict speaks
        grow: bool,
    },
    /// THE GOLDEN-DANDELION'S OWN VERDICT (the `AgeableMob.mobInteract`
    /// arm the 26.2 full disasm speaks: `canUseGoldenDandelion(stack,
    /// isBaby(), ageLockParticleTimer, this)` hit —
    /// `setAgeLocked(this, this::setAgeLockedData, player, stack,
    /// consumer)` the SUCCESS return — the consumer the toggle+reset
    /// walk runs, `stack.consume(1, player)` the dandelion the press
    /// spends, the `isAgeLocked()` post-toggle read the sound pick
    /// carries: `GOLDEN_DANDELION_USE` the locked arm plays,
    /// `GOLDEN_DANDELION_UNUSE` the unlock speaks — v1.0 p1.0, the
    /// PLAYERS source). Rides `locked` = the POST-toggle lock state.
    /// The persistence arm (`setPersistenceRequired` the locked read
    /// guards) the repo's despawn-less world honest-skips; the
    /// consume the inventory-less lane the Feed verdict's own note
    /// keeps.
    AgeLock {
        /// the post-toggle `isAgeLocked()` — the sound pick's own word
        locked: bool,
    },
}

impl MobInteract {
    /// THE VERDICT'S OWN LOVE WORD — the setInLove arm the
    /// SUCCESS_SERVER verdict carries (the swing the
    /// SwingSource.SERVER starts, the heart burst the event 18
    /// broadcast speaks), the CONSUME fallback the false keeps.
    pub fn love(&self) -> bool {
        matches!(self, MobInteract::Feed { love: true, .. })
    }
}

/// THE DYE TINT TABLE (javap DyeColor's clinit — the 16
/// textureDiffuseColor ints the ctor's own last argument speaks, the
/// ColorLerper$Type.SHEEP map's values the wool layer multiplies):
/// WHITE..BLACK in the enum's own order.
pub const DYE_TEXTURE_COLORS: [u32; 16] = [
    0xFFFFFF, // WHITE
    0xFF681F, // ORANGE
    0xFF00FF, // MAGENTA
    0x9AC0CD, // LIGHT_BLUE
    0xFFFF00, // YELLOW
    0xBFFF00, // LIME
    0xFF69B4, // PINK
    0x808080, // GRAY
    0xD3D3D3, // LIGHT_GRAY
    0x00FFFF, // CYAN
    0xA020A0, // PURPLE
    0x0000FF, // BLUE
    0x8B4513, // BROWN
    0x00FF00, // GREEN
    0xFF0000, // RED
    0x000000, // BLACK
];

/// The registration's `eyeHeight(1.74F)`.
pub const ZOMBIE_EYE_HEIGHT: f64 = 1.74;
/// `Zombie.createAttributes` — MOVEMENT_SPEED 0.23000000417232513.
pub const ZOMBIE_MOVEMENT_SPEED: f64 = 0.230_000_004_172_325_13;
/// THE MOB'S OWN MODEL KIND (the renderer's variant table the vanilla
/// EntityRenderDispatcher walks): the humanoid's zombie branch the
/// avatar model owns, the quadruped's pig mesh the pig model owns.
/// The AI lane spawns zombies (the ctor's own default); the wire's
/// word — the Add Entity type through the synced registry's name —
/// arms the kind at the merge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobKind {
    /// the humanoid pose branch (javap AbstractZombieModel's own chain)
    Zombie,
    /// the quadruped mesh (javap PigModel.createBodyLayer's own
    /// numbers — head 8x8x8 + snout, the rotX barrel body, 4 legs)
    Pig,
    /// the bovine mesh (javap CowModel.createBaseCowModel's own
    /// numbers — head 8x8x6 + muzzle + horns, the rotX barrel +
    /// udder, 4 long legs the LEFT pair's mirror rides)
    Cow,
    /// the ovine mesh (javap SheepModel.createBodyLayer's own numbers
    /// — head 6x6x8 the ovine's own replacement, the rotX 8x16x6
    /// barrel, 4 long legs the RIGHT pair's mirror the
    /// createLegs(loc1=false, loc2=true) walk rides) PLUS the eat
    /// fold the own setupAnim speaks (the head the *9.0 drop and the
    /// eat-angle replacement the cow never carried)
    Sheep,
}

impl MobKind {
    /// THE REGISTRY NAME'S OWN MAPPING (the entity_type table the
    /// client keeps). The honest default is the zombie — the lane's
    /// first body; the projection only admits the drawn names, so the
    /// merge never sees an undrawn one.
    pub fn from_wire_name(name: &str) -> Self {
        if name == "minecraft:pig" {
            MobKind::Pig
        } else if name == "minecraft:cow" {
            MobKind::Cow
        } else if name == "minecraft:sheep" {
            MobKind::Sheep
        } else {
            MobKind::Zombie
        }
    }

    /// THE MONSTER CHAIN'S OWN WORD (the class hierarchy the javap
    /// dumps keep: Zombie → Monster → PathfinderMob; the animals'
    /// own chain Sheep/Cow/Pig → Animal → AgeableMob → PathfinderMob
    /// never reaches the Monster branch). The targetSelector the
    /// zombie's own registerGoals fills (zombie_262.txt: getfield
    /// targetSelector, `new NearestAttackableTargetGoal(Mob, Class, Z)`
    /// the prio-2 walk three arms speak) the animals' registerGoals
    /// never touches — the sheep's own prio walk the dump keeps
    /// (0 float, 1 panic, 2 breed, 3 tempt, 4 follow, 5 eat, 6 stroll,
    /// 7 look, 8 look-around — every addGoal the goalSelector's own
    /// field rides) speaks no target goal at all. The scan the monster
    /// kinds gate; the animals the goal-else chain's own peace keeps.
    pub fn is_monster(self) -> bool {
        matches!(self, MobKind::Zombie)
    }

    /// THE PANIC GOAL'S OWN PER-KIND WORD (the registerGoals clinits
    /// the dumps keep: the Sheep's and the Pig's own `addGoal(1, new
    /// PanicGoal(this, 1.25))`, the AbstractCow's own `addGoal(1, new
    /// PanicGoal(this, 2.0))` — the cow's goals the parent class
    /// fills; the zombie's own registerGoals speaks no panic goal —
    /// the None the monster's lane keeps).
    pub fn panic_speed(self) -> Option<f64> {
        match self {
            MobKind::Zombie => None,
            MobKind::Sheep => Some(PANIC_SPEED_SHEEP),
            MobKind::Cow => Some(PANIC_SPEED_COW),
            MobKind::Pig => Some(PANIC_SPEED_PIG),
        }
    }

    /// THE VOICE TABLE (the vanilla rows the four drawn kinds speak —
    /// each the class's own `getAmbientSound` / `getHurtSound` /
    /// `getDeathSound` return: the zombie's groan, the sheep's baa,
    /// the cow's moo, the pig's oink). The registry names the audio
    /// lane's clip table answers; the honest pairing: the voice only
    /// where the body draws (`from_wire_name` admits these four — an
    /// undrawn kind never reaches the table).
    pub fn voice_event(self, slot: VoiceSlot) -> &'static str {
        match (self, slot) {
            (MobKind::Zombie, VoiceSlot::Ambient) => "entity.zombie.ambient",
            (MobKind::Zombie, VoiceSlot::Hurt) => "entity.zombie.hurt",
            (MobKind::Zombie, VoiceSlot::Death) => "entity.zombie.death",
            (MobKind::Sheep, VoiceSlot::Ambient) => "entity.sheep.ambient",
            (MobKind::Sheep, VoiceSlot::Hurt) => "entity.sheep.hurt",
            (MobKind::Sheep, VoiceSlot::Death) => "entity.sheep.death",
            (MobKind::Cow, VoiceSlot::Ambient) => "entity.cow.ambient",
            (MobKind::Cow, VoiceSlot::Hurt) => "entity.cow.hurt",
            (MobKind::Cow, VoiceSlot::Death) => "entity.cow.death",
            (MobKind::Pig, VoiceSlot::Ambient) => "entity.pig.ambient",
            (MobKind::Pig, VoiceSlot::Hurt) => "entity.pig.hurt",
            (MobKind::Pig, VoiceSlot::Death) => "entity.pig.death",
        }
    }

    /// THE STEP VOICE (the playStepSound law the kinds split — javap
    /// the 26.2 jar speaks both arms): the zombie's own override the
    /// FLAT fold keeps (`Zombie.playStepSound`: `playSound(
    /// getStepSound(), 0.15F, 1.0F)` — no block state the voice reads,
    /// every surface the same word, no randoms ride); the animals
    /// override NOTHING — Sheep/Cow/Pig carry no playStepSound the
    /// classfile dumps answer — so the `Entity.playStepSound(pos,
    /// state)` base impl speaks: `soundtype = state.getSoundType();
    /// playSound(soundtype.getStepSound(), soundtype.getVolume() *
    /// 0.15F, soundtype.getPitch())` — the block under the feet
    /// (getOnPos) the voice reads, the SoundType's own base (volume,
    /// pitch) the generated table carries, the `Lane::Step` fold the
    /// *0.15 walks. `None` keeps the silent lanes honest: the air
    /// block the cadence's own gate refuses, the unknown block the
    /// table never names (the vanilla never hears them either).
    pub fn step_voice(self, under: Option<&str>) -> Option<(&'static str, f32, f32)> {
        match self {
            MobKind::Zombie => Some(("entity.zombie.step", ZOMBIE_STEP_VOLUME, ZOMBIE_STEP_PITCH)),
            MobKind::Sheep | MobKind::Cow | MobKind::Pig => {
                use mc_registry::generated::block_sounds as bs;
                let idx = bs::sound_index(under?)?;
                let (volume, pitch) =
                    crate::sound::Lane::Step.fold(bs::SOUND_VOLUME[idx], bs::SOUND_PITCH[idx]);
                Some((crate::sound::Lane::Step.event_of(idx), volume, pitch))
            }
        }
    }
}

/// The voice slots (LivingEntity's own trio — the three playSound
/// moments the entity's voice answers: the ambient the Mob.baseTick
/// gate speaks, the hurt the hurt pipeline's makeSound, the death the
/// die() broadcast).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceSlot {
    Ambient,
    Hurt,
    Death,
}

/// THE VOICE RNG'S OWN SEED (the wire spawn's stand-in for the vanilla
/// independent per-entity random: `RandomSource.create()` seeds from
/// the level's random — the wire spawn has none, so the mob's OWN id
/// the Add Entity packet speaks is the honest per-entity word, mixed
/// through the splitmix constant the client's own walks keep). Same
/// id, same seed; different mobs never groan in unison.
pub fn entity_voice_seed(id: i32) -> u64 {
    0x5EED ^ (id as i64 as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

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
/// `WaterAvoidingRandomStrollGoal`'s own ctor constant (the ldc 0.001
/// the 26.2 dump speaks — the two-arg ctor folds
/// `this(mob, speed, 0.001)` and the field the dry-arm roll reads).
pub const STROLL_LAND_PROBABILITY: f32 = 0.001;
/// `RandomPos.generateRandomPos`'s own loop bound (the bipush 10 the
/// Supplier fold speaks): the tries the stroll's pick draws — each
/// try its direction (the supplier the loop calls all ten times, the
/// rng stream the flow pins).
const STROLL_LAND_TRIES: i32 = 10;
/// The stroll's own land rings (the getPosition calls the dump
/// speaks): the dry arm's `LandRandomPos.getPos(mob, 10, 7)`, the
/// in-water flee's `getPos(mob, 15, 7)` — the first int the
/// HORIZONTAL radius the two ±x draws speak, the second the VERTICAL
/// spread the middle ±z draw carries (the BlockPos(draw_x, draw_y,
/// draw_z) ctor the bytecode pins: the x and z components both draw
/// `nextInt(2*x+1) - x`, the y component `nextInt(2*z+1) - z`).
const STROLL_LAND_RING: i64 = 10;
const STROLL_LAND_RING_IN_WATER: i64 = 15;
/// The vertical spread the second getPosition int carries (the ±7 the
/// middle draw answers).
const STROLL_LAND_SPREAD: i64 = 7;
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
/// THE STROLL CANDIDATE'S OWN SOLID WORD (the world clip's own law —
/// `solid = id != AIR && id != WATER && id != LAVA` the ray's
/// fluid-NONE read speaks): the cell the lift climbs out of, the
/// ground the stable-destination word reads. The below-world the
/// y<0 law keeps NO ground here — the vanilla isOutsideLimits word
/// the out-of-world candidate dies on, the void column's phantom
/// stone the world.get read can never stand on.
/// `AgeableMob.getSpeedUpSecondsWhenFeeding(I)` verbatim (the 26.2 raw
/// disasm speaks): `(int) ((float) (negativeGrowingAge / 20) * 0.1F)`
/// — the feeding's own ten-percent law, the remaining baby time the
/// SECONDS the ageUp's `* 20` refold answers.
fn speed_up_seconds_when_feeding(negative_growing_age: i32) -> i32 {
    ((negative_growing_age / 20) as f32 * 0.1) as i32
}

fn stroll_cell_solid(world: &World, x: i32, y: i32, z: i32) -> bool {
    if y < 0 {
        return false;
    }
    let id = world.get(x, y, z);
    id != crate::world::AIR && id != crate::world::WATER && id != crate::world::LAVA
}
/// `EatBlockGoal.IS_EDIBLE`'s own test (the lambda the <clinit> folds:
/// `state.is(EDIBLE_FOR_SHEEP)` — the tag json's four members the
/// cp_probe walk names). The repo's world table keeps none of the four
/// (NAME_OF's 27 ids carry no plant block), the feet arm the honest
/// still answers — the predicate the law keeps for the worlds to come.
fn edible_for_sheep(id: u16) -> bool {
    let name = crate::world::NAME_OF.get(id as usize).copied().unwrap_or("air");
    EDIBLE_FOR_SHEEP.contains(&name)
}
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
    /// nextStep over solid ground this tick — the caller plays the
    /// kind's own voice, `MobKind::step_voice` the law speaks: the
    /// zombie's flat override, the animals' block law)
    pub step: bool,
    /// THE FEED SPARKLE'S OWN BEAT (the aiStep's `% 4` cadence fired
    /// this tick — the position the `getRandomX(1.0) / getRandomY() +
    /// 0.5 / getRandomZ(1.0)` walk carried, the caller spawns the
    /// happy villager's green the glint texture speaks)
    pub happy_sparkle: Option<(f64, f64, f64)>,
    /// THE AGE-LOCK PARTICLE'S OWN BEAT (the aiStep's makeAgeLockedParticle
    /// walk — the `% 2` cadence the even timer ticks speak, the position
    /// `getRandomX(1.0) / getRandomY(0.2) + getBbHeight() + lift /
    /// getRandomZ(1.0)` carries — the lift the lock state's own word
    /// keeps: 0.2 the PAUSE arm, 0.0 the RESET) — the bool the
    /// post-toggle lock state rides, the particle kind the caller
    /// picks: PAUSE_MOB_GROWTH the locked speaks, RESET_MOB_GROWTH the
    /// unlock's re-baby answers (both the glint sprite the happy
    /// sparkles share — the same quad the render draws)
    pub age_lock_sparkle: Option<(f64, f64, f64, bool)>,
    /// THE GRAZE'S OWN BITE (the EatBlockGoal.tick's `== 4` landing —
    /// the feet arm the edible cell's destroyBlock keeps (AIR), the
    /// below arm the grass block's setBlock(DIRT) rides; the
    /// levelEvent(2001) break particles the repo's dig lane owns the
    /// honest note the swap itself renders — the caller applies the
    /// id at the carried cell, the tick's &World read the immutable
    /// borrow keeps). The ate() walk the tick answers IN the mob —
    /// the wool regrow and the baby's ageUp the event never carries.
    pub eat_block: Option<(i32, i32, i32, u16)>,
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
    /// THE LOVE CLOCK (javap `Animal.inLove` — the private int the
    /// `setInLove` walk arms at 600, the `aiStep` own decrement the
    /// dump speaks: `inLove > 0 → inLove--`, the baby's
    /// `customServerAiStep` reset the age != 0 arm owns). The
    /// `canFallInLove()` gate reads it (`inLove <= 0`), the feed
    /// verdict's own arms key on it.
    pub in_love: i32,
    /// THE AGE CLOCK (javap `AgeableMob.age` — the private int the ctor
    /// zeroes, `getBabyStartAge()`'s -24000 the birth arms, the aiStep's
    /// own walk: the baby's age climbs toward 0 (the grow-up), the
    /// adult's cooldown drains toward 0 (the breed delay), the zero the
    /// `canFallInLove`/`mobInteract` gates read. `isBaby()` the
    /// `age < 0` word speaks — the vanilla's DATA_BABY_ID bit + age
    /// split the wire channel keeps, the single field the same verdicts
    /// the client's own store answers).
    pub age: i32,
    /// `AgeableMob.forcedAge` — the feed's own growth accumulator (the
    /// ageUp's over-grow arm adds the APPLIED delta — the new-minus-old
    /// fold the bytecode speaks, POSITIVE for the growth the walk made;
    /// the age-lock lane the repo keeps none of — the honest note the
    /// server arm's walk never fires).
    pub forced_age: i32,
    /// `AgeableMob.forcedAgeTimer` — the 40-tick sparkle window the
    /// ageUp's over-grow arm marks (the aiStep's `% 4` cadence the
    /// HAPPY_VILLAGER spawns speak, the decrement the window drains).
    pub forced_age_timer: i32,
    /// `AgeableMob`'s `AGE_LOCKED` synched data (the accessor the
    /// defineSynchedData walk speaks — the golden dandelion's own
    /// word): the baby's growth the lock PAUSES (the canAgeUp gate
    /// the natural walk reads — the locked baby the walk never grows),
    /// the toggle the golden-dandelion press walks both directions.
    pub age_locked: bool,
    /// `AgeableMob.ageLockParticleTimer` — the 40-tick PAUSE/RESET
    /// window the toggle's own walk marks (the aiStep's `% 2` cadence
    /// the makeAgeLockedParticle spawns speak — the glint sprite the
    /// happy sparkles share, the lock state the particle kind reads —
    /// PAUSE the locked press leaves, RESET the unlock's re-baby
    /// speaks; the decrement the window drains on every tick it
    /// counts). The gate the re-use reads: the timer's own zero the
    /// canUseGoldenDandelion word requires.
    pub age_lock_particle_timer: i32,
    /// the swing clock (the attack anim the render state's attackTime
    /// samples — the same LivingEntity law the player's hand speaks)
    pub swing: SwingState,
    /// the walk animation store (the render state's walk fields)
    pub walk: WalkAnimationState,
    /// `ZombieRenderState.isAggressive` (the mob flag bit 0x4)
    pub aggressive: bool,
    /// the shared-flags accessor's bit 1 (Entity.isCrouching's own
    /// query — the pose word the wire's mobs speak): the avatar's
    /// crouch pose the render folds. The AI lane never arms it (the
    /// zombie's own goals never crouch) — the wire's word is the
    /// only writer the lane owes.
    pub crouching: bool,
    /// THE WOOL'S OWN STATE (Sheep.isSheared/getColor's decode the
    /// merge law speaks from the DATA_WOOL_ID byte the Set Entity
    /// Data store keeps): the sheared bit's word the fur layer's gate
    /// reads, the dye id the tint table indexes. The ctor's honest
    /// default = the SynchedEntityData's own (byte 0: unsheared, WHITE
    /// — the vanilla client's local default the define speaks).
    pub sheared: bool,
    pub wool_color: u8,
    /// THE GRAZE'S OWN CLOCK (Sheep.eatAnimationTick — the goal's
    /// start() the 40 window arms, the tick's decrement the window
    /// drains, the handleEntityEvent(10) the client copy re-arms: the
    /// offline merge the one field both laws keep — the goal walk the
    /// server side speaks, the render folds the client copy reads).
    pub eat_animation_tick: i32,
    /// THE NAME'S OWN WORD (MobRenderer.checkMagicName's own read —
    /// Entity.getCustomName the DATA_CUSTOM_NAME accessor 2 carries,
    /// the component's plain text): the jeb_ sheep's arm the rainbow
    /// gate reads, the accessor never spoken is the honest unnamed
    /// (the ctor's own None = the SynchedEntityData default).
    pub custom_name: Option<String>,
    /// THE NAMEPLATE'S OWN ARM (Entity's DATA_CUSTOM_NAME_VISIBLE
    /// accessor 3 — the shouldShowName gate's first arm the merge
    /// carries): the byte the server raised says the floating text,
    /// the silent byte keeps the name unsaid.
    pub name_visible: bool,
    /// `ZombieAttackGoal.raiseArmTicks`
    raise_arm_ticks: i32,
    /// THE BODY'S OWN KIND (the renderer's variant table): which mesh
    /// the render lane draws — the AI lane's ctor speaks Zombie, the
    /// wire's registry name re-arms the word at the merge.
    pub kind: MobKind,
    /// `MeleeAttackGoal.ticksUntilNextAttack`
    ticks_until_attack: i32,
    /// the canUse re-check clock (lastCanUseCheck)
    attack_recheck: i64,
    /// the stroll's target (the navigation's own destination stand-in)
    stroll_target: Option<(f64, f64)>,
    /// THE FOLLOWPARENTGOAL'S OWN ARM (the goal's own `parent` field
    /// the canUse walk stores): the parent's id + the position
    /// snapshot the tick's straight-line stand-in steers to — the
    /// None the disarmed goal keeps (the caller's per-frame re-arm
    /// the re-evaluation speaks)
    follow_parent: Option<(i32, f64, f64, f64)>,
    /// THE BREEDGOAL'S OWN ARM (the goal's own `partner` field the
    /// canUse walk stores): the mate's id + the position snapshot the
    /// tick's `moveTo(partner, speedModifier)` stand-in steers to —
    /// the None the disarmed goal keeps (the caller's per-frame
    /// re-arm the re-evaluation speaks)
    breed_partner: Option<(i32, f64, f64, f64)>,
    /// THE GOAL'S OWN COURT CLOCK (javap `BreedGoal.loveTime` — the
    /// private int the tick's `loveTime++` counts, the 60 the
    /// `adjustedTickDelay(60)` gate reads, the stop() law zeroes).
    /// The staged pair the feed-and-leave walk's completed court the
    /// staging's own stand-in arms (the --breed law the in_love 600
    /// stage keeps company).
    pub breed_love_time: i32,
    /// THE GOAL'S OWN SPEED MODIFIER (the moveTo's `speedModifier`
    /// double the FollowParentGoal's 1.1 rides — the tick resets it
    /// to 1.0, the armed goal the multiplication speaks)
    move_speed_modifier: f64,
    /// THE PANICGOAL'S OWN FLEE (the goal's posXD/YD/ZD fields the
    /// start()'s moveTo names): the flee walk the armed target keeps —
    /// the Some the goal's isRunning word stands for (the
    /// isPanicking fallback's own read), the None the stop law keeps
    pub panic_target: Option<(f64, f64, f64)>,
    /// THE LAST DAMAGE SOURCE'S OWN WINDOW (the shouldPanic gate the
    /// `getLastDamageSource` 40-tick law keeps — the hurt stamp arms,
    /// the tick's own drain closes)
    pub panic_memory_ticks: i32,
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
            in_love: 0,
            age: 0,
            forced_age: 0,
            forced_age_timer: 0,
            age_locked: false,
            age_lock_particle_timer: 0,
            swing: SwingState::default(),
            walk: WalkAnimationState::default(),
            aggressive: false,
            crouching: false,
            sheared: false,
            wool_color: 0,
            eat_animation_tick: 0,
            custom_name: None,
            name_visible: false,
            kind: MobKind::Zombie,
            raise_arm_ticks: 0,
            ticks_until_attack: 0,
            attack_recheck: 0,
            stroll_target: None,
            follow_parent: None,
            breed_partner: None,
            breed_love_time: 0,
            move_speed_modifier: 1.0,
            panic_target: None,
            panic_memory_ticks: 0,
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
        (
            self.x - h,
            self.y,
            self.z - h,
            self.x + h,
            self.y + ZOMBIE_HEIGHT,
            self.z + h,
        )
    }

    /// THE LOVE WALK'S OWN BOX — the feet-anchored bbox the body's
    /// OWN registration dims speak (javap `Entity.getRandomX` /
    /// `getRandomY`: the spread rides `getBbWidth() * 0.5` and
    /// `getBbHeight()` — the EntityTypes `sized(w, h)` each kind's
    /// own clinit arms: the sheep 0.9x1.3, the cow 0.9x1.4, the pig
    /// 0.9x0.9, the zombie 0.6x1.95 — the same table the render's
    /// flame feature reads).
    pub fn box_for(&self) -> (f64, f64, f64, f64, f64, f64) {
        let (w, h) = match self.kind {
            MobKind::Sheep => (SHEEP_WIDTH, SHEEP_HEIGHT),
            MobKind::Cow => (COW_WIDTH, COW_HEIGHT),
            MobKind::Pig => (PIG_WIDTH, PIG_HEIGHT),
            MobKind::Zombie => (ZOMBIE_WIDTH, ZOMBIE_HEIGHT),
        };
        // THE BABY'S OWN DIMENSIONS (javap `Sheep.getDefaultDimensions`
        // the isBaby arm the dump names — the half-sized box the baby
        // body occupies, the same fold the render pose's 0.5 scale
        // speaks)
        let (w, h) = if self.is_baby() {
            (w * 0.5, h * 0.5)
        } else {
            (w, h)
        };
        let h2 = w / 2.0;
        (
            self.x - h2,
            self.y,
            self.z - h2,
            self.x + h2,
            self.y + h,
            self.z + h2,
        )
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
            self.prev_head_yaw_off + (self.head_yaw_off - self.prev_head_yaw_off) * t,
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
        // THE PANIC STAMP (PanicGoal.shouldPanic's own gate — the last
        // damage source the 40-tick window keeps; the jar's own
        // PANIC_CAUSES tag names every source this lane speaks — the
        // melee the mob_attack/#is_player_attack members ride, the
        // fire lane the in_fire/on_fire members keep)
        self.panic_memory_ticks = PANIC_DAMAGE_MEMORY_TICKS;
    }

    /// THE ISPANICKING WORD (javap `PathfinderMob.isPanicking` the
    /// dump keeps — the brain lane the repo never keeps, the method's
    /// own brain-dead fallback walk the running-goal law answers: any
    /// RUNNING goal that is a PanicGoal speaks true — the flee walk
    /// the armed target the isRunning word stands for).
    pub fn is_panicking(&self) -> bool {
        self.panic_target.is_some()
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
        // -80 reset; the draw rides the entity's own LCG — the gate
        // method the wire lane's own tick shares)
        if self.ambient_voice() {
            events.ambient = true;
        }
        // THE HURT CLOCK (LivingEntity.tick's own decrement)
        if self.hurt_time > 0 {
            self.hurt_time -= 1;
        }
        // THE LOVE CLOCK (javap `Animal.aiStep`'s own walk: the
        // `inLove > 0` arm the decrement speaks — the 600 the
        // setInLove armed walks to 0, the gate reopens)
        if self.in_love > 0 {
            self.in_love -= 1;
        }
        // THE AGE'S OWN LOVE RESET (javap `Animal.customServerAiStep`: the
        // `getAge() != 0` arm the reset speaks — the baby and the breed
        // cooldown both never carry the love word, the tick the same
        // walk the decrement's tail arms)
        if self.age != 0 {
            self.in_love = 0;
        }
        // THE AGEABLEMOB'S OWN WALK (javap `AgeableMob.aiStep`'s
        // server arm the full disasm speaks: `i = getAge(); if
        // (canAgeUp()) setAge(i + 1); else if (i > 0) setAge(i - 1);`
        // — BOTH arms route through setAge, the grow-up crossing the
        // -1 -> 0 beat the boundary branch arms (the canAgeUp gate =
        // isBaby && !isAgeLocked — the lock the golden dandelion's
        // own word PAUSES: the locked baby the walk never grows, the
        // drain the same gate the positive age keeps); the positive
        // drain the adult's cooldown walks, the 1 -> 0 the same-sign
        // set the boundary never arms)
        if self.can_age_up() {
            self.set_age(self.age + 1);
        } else if self.age > 0 {
            self.set_age(self.age - 1);
        }
        // THE FEED'S OWN SPARKLES (javap `AgeableMob.aiStep`'s client
        // arm verbatim: the `forcedAgeTimer > 0` gate the ageUp's
        // over-grow arm marks, the `% 4` cadence the HAPPY_VILLAGER
        // spawn speaks at `getRandomX(1.0) / getRandomY() + 0.5 /
        // getRandomZ(1.0)` — the x and z both `+ (2.0 * nextDouble() -
        // 1.0) * 1.0`, the y `+ nextDouble() + 0.5` the no-arg
        // getRandomY's `getY(nextDouble())` law carries — the spawn
        // BEFORE the decrement, the ten flashes the 40-tick window
        // keeps; the server's forcedAge arm the age-lock lane the
        // repo keeps none of, the honest note the walk never fires)
        if self.forced_age_timer > 0 {
            if self.forced_age_timer % 4 == 0 {
                let sx = self.x + (2.0 * self.rng.next_f64() - 1.0);
                let sy = self.y + self.rng.next_f64() + 0.5;
                let sz = self.z + (2.0 * self.rng.next_f64() - 1.0);
                events.happy_sparkle = Some((sx, sy, sz));
            }
            self.forced_age_timer -= 1;
        }
        // THE AGE-LOCK PARTICLE'S OWN WALK (javap `AgeableMob.aiStep`'s
        // tail — `ageLockParticleTimer = makeAgeLockedParticle(level(),
        // this, ageLockParticleTimer, isAgeLocked())` the BOTH-sides
        // call speaks: `if (timer > 0) { if (isClientSide() && timer %
        // 2 == 0) { spawn } timer--; } return timer;` the lossless dump
        // answers — the spawn the even timer ticks keep, the decrement
        // EVERY tick the window counts). The position law the Vec3 ctor
        // speaks: `getRandomX(1.0)` the x, `getRandomY(0.2) +
        // getBbHeight() + (locked ? 0.2F : 0.0F)` the y — the lift the
        // PAUSE arm's own ldc 0.2 speaks — `getRandomZ(1.0)` the z,
        // each the symmetric triangle `(2.0 * nextDouble() - 1.0) *
        // scale` the Entity.getRandomX/getRandomY(D)/getRandomZ disasm
        // pins (the x/z draws 1.0, the y draw 0.2 — the three draws the
        // ctor's left-to-right order keeps). The particle pick the
        // lock state's own word: PAUSE_MOB_GROWTH the locked speaks,
        // RESET_MOB_GROWTH the unlock's re-baby answers. The timer's
        // zero the re-use gate reads (the canUseGoldenDandelion word).
        if self.age_lock_particle_timer > 0 {
            if self.age_lock_particle_timer % 2 == 0 {
                // the bb top the box's own fold answers (the
                // registration dims the isBaby half-scale speaks)
                let bb_h = {
                    let (_, _, _, _, y1, _) = self.box_for();
                    y1 - self.y
                };
                let lift: f64 = if self.age_locked { 0.2 } else { 0.0 };
                let px = self.x + (2.0 * self.rng.next_f64() - 1.0) * 1.0;
                let py =
                    self.y + (2.0 * self.rng.next_f64() - 1.0) * 0.2 + bb_h + lift;
                let pz = self.z + (2.0 * self.rng.next_f64() - 1.0) * 1.0;
                events.age_lock_sparkle = Some((px, py, pz, self.age_locked));
            }
            self.age_lock_particle_timer -= 1;
        }
        self.swing.tick();
        self.age_ticks = self.age_ticks.wrapping_add(1);

        let mut want_move = false;
        let (mut ix, mut iz) = (0.0f64, 0.0f64);
        // THE GOALS' OWN SPEED (the moveTo's speedModifier the tick
        // re-resolves — the armed follow the 1.1 speaks, every other
        // goal the plain attribute keeps)
        self.move_speed_modifier = 1.0;

        // THE TARGET SCAN (NearestAttackableTargetGoal(Player, mustSee)
        // — the FOLLOW_RANGE 35 ring, the dead never retarget). THE
        // ANIMALS' OWN PEACE: the targetSelector the zombie's
        // registerGoals fills the sheep's/cow's/pig's own registerGoals
        // never fills — the scan the monster kinds gate, the animals
        // the goal-else chain's own rest keeps (the fed sheep the
        // in-range player no longer charges — the empty targetSelector
        // the animal chain speaks)
        let target = if self.kind.is_monster() {
            player.filter(|p| !p.dead).filter(|p| {
                let dx = p.x - self.x;
                let dz = p.z - self.z;
                dx * dx + dz * dz <= ZOMBIE_FOLLOW_RANGE * ZOMBIE_FOLLOW_RANGE
            })
        } else {
            None
        };

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
            // THE PANICGOAL'S OWN FLEE (prio 1 — the Sheep's and the
            // Pig's own registerGoals clinit speaks `addGoal(1, new
            // PanicGoal(this, 1.25))`, the AbstractCow's own
            // `addGoal(1, new PanicGoal(this, 2.0))` the parent's
            // clinit keeps, the breed's 2 the priority owns, the
            // zombie's own registerGoals fills no panic goal the kind
            // word keeps): the tick's straight-line stand-in walks the
            // armed flee target — the yaw faces it, the forward
            // impulse rides the goal's own speedModifier (the
            // per-kind word the registerGoals clinits keep), the
            // arrival the canContinueToUse's own navigation-done law
            // stops (the isRunning word the None answers — the canUse
            // re-check the memory's window re-arms while it lives)
            if let Some((tx, _ty, tz)) = self.panic_target {
                let dx = tx - self.x;
                let dz = tz - self.z;
                if dx * dx + dz * dz < STROLL_ARRIVAL * STROLL_ARRIVAL {
                    self.panic_target = None;
                } else {
                    let len = (dx * dx + dz * dz).sqrt();
                    self.yaw = (-(dx / len)).atan2(dz / len).to_degrees() as f32;
                    ix = 0.0;
                    iz = 1.0;
                    want_move = true;
                    self.move_speed_modifier = self.kind.panic_speed().unwrap_or(1.0);
                }
                // the goal's own continued run the lower arms' stop law
                // keeps (the flag-conflict the running walk owns — the
                // breed's own stop the court's clock zeroes, the graze's
                // own stop the window's timer zeroes)
                self.stroll_target = None;
                self.follow_parent = None;
                self.eat_animation_tick = 0;
                if self.breed_partner.take().is_some() {
                    self.breed_love_time = 0;
                }
            } else if self.panic_memory_ticks > 0 && self.kind.panic_speed().is_some() {
                // THE PANICGOAL'S OWN CANUSE (shouldPanic the memory's
                // window answers — the kinds whose registerGoals fills
                // the goal speak the arm): the on-fire arm the
                // lookForWater(5) lane speaks FIRST — the water cell
                // the flee walks TO — else the random ring the
                // DefaultRandomPos.getPos(mob, 5, 4) draw keeps (the
                // polar fold the stroll's own stand-in speaks). The
                // goal's own start the lower arms replaces (the
                // GoalSelector's flag-conflict law the higher priority
                // keeps — the breed's own stop the court's clock zeroes)
                let water_arm = if self.fire_ticks > 0 {
                    self.look_for_water(world)
                } else {
                    None
                };
                match water_arm {
                    Some((wx, wy, wz)) => self.panic_target = Some((wx, wy, wz)),
                    None => {
                        let ang = self.rng.next_f64() * std::f64::consts::TAU;
                        let dist = self.rng.next_f64() * PANIC_RING_XZ;
                        self.panic_target =
                            Some((self.x + ang.cos() * dist, self.y, self.z + ang.sin() * dist));
                    }
                }
                self.stroll_target = None;
                self.follow_parent = None;
                self.eat_animation_tick = 0;
                if self.breed_partner.take().is_some() {
                    // the breed goal's own stop the court's clock zeroes
                    self.breed_love_time = 0;
                }
            } else if let Some((_id, px, py, pz)) = self.breed_partner {
                // THE BREEDGOAL'S OWN TICK (prio 2 — the Sheep's own
                // registerGoals clinit speaks `addGoal(2, new
                // BreedGoal(this, 1.0))`, the follow's 4 the priority
                // owns, the armed scan the caller's pre-pass keeps): the
                // `moveTo(partner, speedModifier)` the navigation's
                // straight-line stand-in walks — the yaw faces the
                // partner, the forward impulse rides the 1.0 the goal's
                // own ctor speaks, the stroll's target and the follow's
                // own arm the higher priority replaces — and the court's
                // own clock the `loveTime++` counts: the 60 the
                // `adjustedTickDelay(60)` gate reads, the close ring the
                // breed() trigger's `distanceToSqr(partner) < 9.0` the
                // caller's post-tick scan answers, the 60-miss beyond the
                // ring the canContinueToUse's own stop law resets (the
                // goal re-arms, the court restarts — the pair's own drift
                // the per-frame re-arm answers)
                let dx = px - self.x;
                let dy = py - self.y;
                let dz = pz - self.z;
                let len = (dx * dx + dz * dz).sqrt().max(1e-9);
                self.yaw = (-(dx / len)).atan2(dz / len).to_degrees() as f32;
                ix = 0.0;
                iz = 1.0;
                want_move = true;
                self.move_speed_modifier = BREED_SPEED;
                self.stroll_target = None;
                self.follow_parent = None;
                self.eat_animation_tick = 0;
                self.breed_love_time += 1;
                if self.breed_love_time >= BREED_COURT_TICKS
                    && dx * dx + dy * dy + dz * dz >= BREED_CLOSE_SQ
                {
                    // the court expired beyond the close ring — the
                    // goal's own stop+re-arm the 60-clock restarts
                    self.breed_love_time = 0;
                }
            } else if let Some((_id, px, _py, pz)) = self.follow_parent {
                let dx = px - self.x;
                let dz = pz - self.z;
                let len = (dx * dx + dz * dz).sqrt().max(1e-9);
                self.yaw = (-(dx / len)).atan2(dz / len).to_degrees() as f32;
                ix = 0.0;
                iz = 1.0;
                want_move = true;
                self.move_speed_modifier = FOLLOW_PARENT_SPEED;
                self.stroll_target = None;
                self.eat_animation_tick = 0;
            } else if self.eat_animation_tick > 0 {
                // THE EATBLOCKGOAL'S OWN TICK (prio 5 — the Sheep's own
                // registerGoals clinit speaks `addGoal(5,
                // this.eatBlockGoal)` the follow's 4 the priority owns,
                // the stroll's 6/7 the flags stop): the RUNNING graze
                // holds the MOVE/LOOK/JUMP flags the ctor keeps — the
                // want_move stays false the nav-stop's still keeps, the
                // stroll's target the conflict clears. The goal's own
                // tick walk: the decrement max(0, -1) the client's
                // aiStep arm shares, the consume the `==
                // adjustedTickDelay(4)` landing answers: the feet cell
                // the IS_EDIBLE tag test reads (the edible_for_sheep
                // members the world table none keeps — the honest
                // still), else the below cell the GRASS_BLOCK arm swaps
                // to dirt; the MOB_GRIEFING the default-true the
                // gamerule table speaks; the ate() walk BOTH arms ride
                // (the wool regrow, the baby's ageUp(60) the canAgeUp
                // gate keeps — the byte-level law the mob-local state
                // answers in-tick).
                self.stroll_target = None;
                self.eat_animation_tick = (self.eat_animation_tick - 1).max(0);
                if self.eat_animation_tick == EAT_CONSUME_TICK {
                    let (fx, fy, fz) =
                        (self.x.floor() as i32, self.y.floor() as i32, self.z.floor() as i32);
                    if edible_for_sheep(world.get(fx, fy, fz)) {
                        events.eat_block = Some((fx, fy, fz, crate::world::AIR));
                        self.ate();
                    } else if world.get(fx, fy - 1, fz) == crate::world::GRASS {
                        // levelEvent(2001, below, Block.getId(grass)) —
                        // the break particles the dig lane owns
                        events.eat_block = Some((fx, fy - 1, fz, crate::world::DIRT));
                        self.ate();
                    }
                }
            } else if self.kind == MobKind::Sheep
                && self.rng.next()
                    % (if self.is_baby() { EAT_GATE_BABY } else { EAT_GATE_ADULT })
                    == 0
                && (edible_for_sheep(
                    world.get(self.x.floor() as i32, self.y.floor() as i32, self.z.floor() as i32),
                ) || world.get(
                    self.x.floor() as i32,
                    self.y.floor() as i32 - 1,
                    self.z.floor() as i32,
                ) == crate::world::GRASS)
            {
                // THE EATBLOCKGOAL'S OWN CANUSE+START (prio 5 — the
                // Sheep's own registerGoals the goal the kind arms, the
                // cow's/pig's own clinit the goal never fills): the
                // nextInt gate the isBaby word splits (the baby's 50,
                // the adult's 1000 — the adjustedTickDelay the identity
                // keeps), the block reads the gate's truth arms — the
                // feet edible OR the grass block below. The start():
                // the 40 window arms, the broadcast(10) the offline
                // merge the direct field write keeps, the navigation's
                // stop the repo's no-navigator still answers, the LOOK
                // flag the look goal's clock zeroes.
                self.eat_animation_tick = EAT_ANIMATION_TICKS;
                self.stroll_target = None;
                self.look_ticks = 0;
            } else if let Some((tx, tz)) = self.stroll_target {
                // THE STROLL GOAL (prio 7): the canUse gate rides the
                // 120-tick interval (the RandomStrollGoal's own), the
                // target walks until the arrival slack speaks
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
                // THE WATER-AVOIDING PICK (javap
                // `WaterAvoidingRandomStrollGoal.getPosition` the 26.2
                // jar speaks): the in-water arm rides the 15/7 land
                // ring — the null the SUPER's raw pick answers (the
                // getPos(mob, 15, 7) fallback the ifnonnull fold
                // speaks); the dry arm the 0.001 probability roll
                // splits — the low roll keeps the super pick (the
                // iflt fold), the rest walk the 10/7 land ring whose
                // null the canUse gate answers with NO stroll this
                // cycle (the vec3 == null law the goal's own use
                // speaks).
                let target = if self.in_water(world) {
                    match self.land_random_pos(world, STROLL_LAND_RING_IN_WATER, STROLL_LAND_SPREAD)
                    {
                        Some(t) => Some(t),
                        None => Some(
                            self.default_random_pos(STROLL_LAND_RING_IN_WATER, STROLL_LAND_SPREAD),
                        ),
                    }
                } else if self.rng.next_f32() < STROLL_LAND_PROBABILITY {
                    Some(self.default_random_pos(STROLL_LAND_RING, STROLL_LAND_SPREAD))
                } else {
                    self.land_random_pos(world, STROLL_LAND_RING, STROLL_LAND_SPREAD)
                };
                if let Some((tx, tz)) = target {
                    self.stroll_target = Some((tx, tz));
                }
            }
            // THE LOOK GOAL (prio 8): the look-at-player branch rides
            // the 0.02 probability per 40-tick cycle (the ctor's own
            // constants), the look holds 40 ticks
            if self.eat_animation_tick > 0 {
                // THE GRAZE'S OWN LOOK (the LOOK flag the eat goal's
                // ctor keeps — the running graze the look goals' own
                // stop law keeps silent, the head the graze's own
                // scales own)
            } else if self.look_ticks > 0 {
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

        // THE PANIC MEMORY'S OWN DRAIN (Entity.getLastDamageSource's
        // own window — the `tickCount - lastDamageStamp > 40 -> null`
        // law the countdown answers: the goals' own read the window
        // keeps alive the 40 ticks past the stamp, the drain the
        // walk's own end closes)
        if self.panic_memory_ticks > 0 {
            self.panic_memory_ticks -= 1;
        }

        // THE BURN ROLL (Mob.aiStep's BURN_IN_DAYLIGHT branch ->
        // shouldBurnInDay -> burnUndead -> isSunBurnTick, the
        // short-circuit order the bytecode speaks: THE KIND'S OWN WORD
        // first — the Mob base speaks false, the undead classes'
        // override the true the sun's brand rides (the animals the
        // base's own false keeps: the noon sky the sheep never
        // brands), then the track gate, the water gate, the light
        // gate, THE DRAW, the sky gate — the helmet branch never fires
        // on this lane's bare-headed zombie, so the hit ignites):
        // igniteForSeconds(8.0) = remainingFireTicks = max(remaining,
        // 160).
        if self.kind.is_monster()
            && crate::timeline::monsters_burn(day_time)
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
            let friction = if self.on_ground {
                BLOCK_FRICTION
            } else {
                AIR_FRICTION
            };
            let speed = if friction > 0.6 {
                ZOMBIE_MOVEMENT_SPEED * CUBE_SPEED_NUMERATOR as f64
                    / (friction * friction * friction) as f64
            } else {
                ZOMBIE_MOVEMENT_SPEED
            } * self.move_speed_modifier;
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
        let friction = if was_on_ground {
            BLOCK_FRICTION
        } else {
            AIR_FRICTION
        };
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

    /// THE PANICGOAL'S OWN WATER SCAN (javap `lookForWater(level, mob,
    /// radius)` the canUse dump keeps — `BlockPos.findClosestMatch(
    /// blockPosition(), radius, radius, the fluid-state WATER word)`:
    /// the closest-match walk the chebyshev rings answer — the mob's
    /// own block the first ring keeps, each shell the radius speaks,
    /// the first water cell the nearest the match returns).
    fn look_for_water(&self, world: &World) -> Option<(f64, f64, f64)> {
        let bx = self.x.floor() as i32;
        let by = self.y.floor() as i32;
        let bz = self.z.floor() as i32;
        for r in 0..=PANIC_WATER_RING {
            for dy in -r..=r {
                for dz in -r..=r {
                    for dx in -r..=r {
                        if dx.abs().max(dz.abs()).max(dy.abs()) != r {
                            continue; // the shell the ring keeps
                        }
                        if world.get(bx + dx, by + dy, bz + dz) == crate::world::WATER {
                            return Some(((bx + dx) as f64, (by + dy) as f64, (bz + dz) as f64));
                        }
                    }
                }
            }
        }
        None
    }

    /// THE WATER-AVOIDING STROLL'S OWN DIRECTION DRAW (javap
    /// `RandomPos.generateRandomDirection(RandomSource, II)` the 26.2
    /// jar speaks): THREE nextInt draws in the order the istore chain
    /// pins — the x component `nextInt(2*x+1) - x`, the y component
    /// `nextInt(2*z+1) - z` (the SECOND arg the vertical spread
    /// carries), the z component `nextInt(2*x+1) - x` again (the
    /// bytecode's second ±x draw) — folded into BlockPos(draw_x,
    /// draw_y, draw_z). The modulo the repo's goal clocks already
    /// speak stands in for nextInt's own fold (the honest stand-in).
    fn generate_random_direction(&mut self, rx: i64, ry: i64) -> (i64, i64, i64) {
        let dx = (self.rng.next() % (2 * rx + 1) as u64) as i64 - rx;
        let dy = (self.rng.next() % (2 * ry + 1) as u64) as i64 - ry;
        let dz = (self.rng.next() % (2 * rx + 1) as u64) as i64 - rx;
        (dx, dy, dz)
    }

    /// THE LAND PICK (javap `LandRandomPos.getPos(mob, x, z)` over
    /// `RandomPos.generateRandomPos`'s ten-try loop): every try draws
    /// its direction (the supplier the loop calls all ten times — the
    /// rng stream the flow pins, the early hit the loop never
    /// short-circuits), the first DRY STANDABLE candidate the score-0
    /// fold keeps. The candidate's own gates the helper classes speak:
    /// the lift (movePosUpOutOfSolid) walks the cell up out of the
    /// solid it landed in, the isWater word (GoalUtils.isWater: the
    /// fluid state the lifted cell carries) rejects the pool surface
    /// the lift surfaces into, and the stable-destination word
    /// (isNotStable's honest stand-in — the simplified walker keeps
    /// no navigator) rejects the unlifted candidate whose ground
    /// below is not solid (the floating cell the real navigator's
    /// walk never answers). The pathfinding malus table the repo
    /// keeps none of — the honest unknown the fold never invents.
    /// None after the ten — the vanilla null the stroll's canUse
    /// answers with NO walk this cycle.
    fn land_random_pos(&mut self, world: &World, rx: i64, ry: i64) -> Option<(f64, f64)> {
        let bx = self.x.floor() as i64;
        let by = self.y.floor() as i64;
        let bz = self.z.floor() as i64;
        let mut picked: Option<(f64, f64)> = None;
        for _ in 0..STROLL_LAND_TRIES {
            let (dx, dy, dz) = self.generate_random_direction(rx, ry);
            if picked.is_some() {
                continue;
            }
            let (cx, cz) = (bx + dx, bz + dz);
            // THE LIFT (movePosUpOutOfSolid): up out of the solid cell
            // the candidate landed in, the world's ceiling the bound
            // keeps.
            let mut cy = by + dy;
            let mut lifted = false;
            while cy < crate::world::CHUNK_Y as i64
                && stroll_cell_solid(world, cx as i32, cy as i32, cz as i32)
            {
                cy += 1;
                lifted = true;
            }
            // THE ISWATER WORD: the pool surface the lift spoke.
            if world.get(cx as i32, cy as i32, cz as i32) == crate::world::WATER {
                continue;
            }
            // THE STABLE-DESTINATION WORD: the unlifted candidate the
            // solid ground below keeps.
            if !lifted && !stroll_cell_solid(world, cx as i32, (cy - 1) as i32, cz as i32) {
                continue;
            }
            picked = Some((cx as f64 + 0.5, cz as f64 + 0.5));
        }
        picked
    }

    /// THE SUPER PICK (javap `DefaultRandomPos.getPos(mob, 10, 7)` —
    /// the RandomStrollGoal.getPosition the WaterAvoiding override
    /// falls back to): the same ten-try loop, each try its direction
    /// draw, the FIRST candidate the score-0 fold keeps — the
    /// outside/restricted/stable/malus gates the simplified walker
    /// keeps no leash, no navigator and no malus table for, so every
    /// candidate passes (the honest note the vanilla's own null never
    /// speaks here). The water the gate never reads — the raw ring's
    /// own law, the 0.001 arm and the in-water fallback ride it
    /// verbatim.
    fn default_random_pos(&mut self, rx: i64, ry: i64) -> (f64, f64) {
        let bx = self.x.floor() as i64;
        let bz = self.z.floor() as i64;
        let (dx, _dy, dz) = self.generate_random_direction(rx, ry);
        // the nine remaining tries draw their directions too (the
        // supplier the loop calls all ten times)
        for _ in 1..STROLL_LAND_TRIES {
            let _ = self.generate_random_direction(rx, ry);
        }
        ((bx + dx) as f64 + 0.5, (bz + dz) as f64 + 0.5)
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
        let head_abs = (-(dx.max(1e-9) / horiz.max(1e-9)))
            .atan2(dz / horiz.max(1e-9))
            .to_degrees();
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

    /// THE AMBIENT VOICE'S OWN GATE (Mob.baseTick's law the local tick
    /// and the wire lane's tick share, verbatim): the clock climbs one
    /// per tick, the `nextInt(1000)` draw under it — when the draw
    /// lands BELOW the climbing clock the voice's own tick speaks and
    /// the clock re-arms at the -80 the resetAmbientSoundTime bipush
    /// keeps. The draw rides the entity's own LCG (one per tick, alive
    /// ticks only — the caller's isAlive gate; the dying body's clock
    /// freezes, the same silence the local tick's early return speaks).
    pub fn ambient_voice(&mut self) -> bool {
        self.ambient_time += 1;
        if self.rng.next() % AMBIENT_DRAW_BOUND < self.ambient_time.max(0) as u64 {
            self.ambient_time = -AMBIENT_INTERVAL_TICKS;
            true
        } else {
            false
        }
    }

    /// `LivingEntity.getVoicePitch` — the hurt/death/ambient voices'
    /// pitch law, verbatim: `(r - r) * 0.2 + 1.0` over the entity's
    /// own random stream (the two nextFloat draws in order).
    pub fn voice_pitch(&mut self) -> f32 {
        (self.rng.next_f32() - self.rng.next_f32()) * 0.2 + 1.0
    }

    /// The mob.interact table's own walk (javap `Sheep.mobInteract` —
    /// the 26.2 bytecode the dump keeps: `loc2 = player.getItemInHand
    /// (hand)`, the gate `loc2.is(Items.SHEARS) && readyForShearing()`,
    /// the server arm `shear(level, SoundSource.PLAYERS, loc2)` +
    /// `gameEvent(GameEvent.SHEAR, player)` + the shears'
    /// `hurtAndBreak(1, player, asEquipmentSlot())`, the SUCCESS_SERVER
    /// return; the client arm CONSUME; the miss falls to
    /// `Animal.mobInteract`). The repo's held stack rides the caller's
    /// unified id (the item_name the id resolves); the shears' own
    /// durability the repo's item vocabulary never speaks (no
    /// damage-component lane — the hurtAndBreak arm honest-skipped).
    /// THE FEEDING LANE the `Animal.mobInteract` super walks (the 26.2
    /// bytecode the dump keeps: the `isFood(item)` hit, the ADULT arm
    /// `getAge() == 0 && canFallInLove()` — `usePlayerItem` +
    /// `setInLove(player)` + `playEatingSound()`, the SUCCESS_SERVER
    /// return; the fallback the in-love adult keeps — the CONSUME
    /// return; the BABY arm the `canAgeUp` gate arms the repo never
    /// walks — isBaby the honest constant false): the tag tables the
    /// jar's own `tags/item/*_food.json` speak (SHEEP_FOOD/COW_FOOD the
    /// wheat, PIG_FOOD the carrot/potato/beetroot), the zombie's chain
    /// never reaching the Animal super (the monster the food gate
    /// refuses). The voice `playEatingSound` the override the jar
    /// keeps — the PIG only; the playSound rides the caller's sound
    /// lane. The loot's lane the shear's own shape keeps (the drops
    /// spawn the caller's walk speaks).
    pub fn mob_interact(&mut self, held_item: &str, rng: &mut Lcg) -> MobInteract {
        match held_item {
            "shears" if self.kind == MobKind::Sheep && self.ready_for_shearing() => {
                MobInteract::Shear(self.shear(rng))
            }
            // the FOOD arm the Animal super walks (the miss the
            // Sheep's own shears gate speaks falls here — the vanilla
            // order the chain keeps): the isFood tag tables, the
            // canFallInLove gate the in_love clock reads, the
            // setInLove the 600 arms
            _ if self.is_food(held_item) => self.feed(),
            // THE GOLDEN-DANDELION'S OWN ARM (the AgeableMob.mobInteract
            // the full disasm speaks — the chain's own order the super
            // call keeps: Sheep's shears, Animal's food, AgeableMob's
            // dandelion, PathfinderMob's pass — the gate the
            // canUseGoldenDandelion word arms: the GOLDEN_DANDELION
            // item the stack's own getItem answers, the isBaby the
            // grown-up the gate refuses, the timer's zero the
            // 40-tick window's own drain requires, the
            // CANNOT_BE_AGE_LOCKED tag the zoo's AgeableMob kinds —
            // Sheep/Cow/Pig — the jar's own tag json never carries;
            // the ZOMBIE the monster chain the AgeableMob super never
            // reaches, the kind gate the monster's own chain speaks)
            _ if self.can_use_golden_dandelion(held_item) => self.use_golden_dandelion(),
            // the miss arms: the wrong stack, the wrong body, the
            // already-sheared wool (readyForShearing's own false), the
            // food the monster's chain never reaches — the super's
            // own PASS the walk returns
            _ => MobInteract::Pass,
        }
    }

    /// THE FOOD TAG TABLES (javap `Sheep.isFood` / `AbstractCow.isFood`
    /// / `Pig.isFood` — the `ItemStack.is(ItemTags.*)` walk the dumps
    /// keep — and the jar's own `data/minecraft/tags/item/*.json` the
    /// values speak verbatim: SHEEP_FOOD = [wheat], COW_FOOD = [wheat],
    /// PIG_FOOD = [carrot, potato, beetroot]; the Chicken the repo's
    /// mesh never drew, its CHICKEN_FOOD tag the table honest-skips).
    /// The ZOMBIE the gate refuses outright — the monster's chain
    /// (Zombie → Monster → PathfinderMob → Mob) never reaches the
    /// `Animal.mobInteract` super the food arm lives in.
    pub fn is_food(&self, held_item: &str) -> bool {
        match self.kind {
            MobKind::Sheep | MobKind::Cow => held_item == "wheat",
            MobKind::Pig => {
                held_item == "carrot" || held_item == "potato" || held_item == "beetroot"
            }
            // the monster (and the kinds the mesh lane never drew) —
            // the food gate closed
            MobKind::Zombie => false,
        }
    }

    /// THE SET-IN-LOVE WALK (javap `Animal.mobInteract`'s ADULT arm —
    /// the 26.2 bytecode the dump keeps: the `getAge() == 0 &&
    /// canFallInLove()` gate, `usePlayerItem(player, hand, item)` the
    /// consume the repo's inventory-less lane honest-skips,
    /// `setInLove(player)` — the `inLove = 600` the field walk arms
    /// + the loveCause + the broadcastEntityEvent(18) the HEART
    /// burst rides, `playEatingSound()`, the SUCCESS_SERVER return;
    /// the fallback the `canFallInLove` false adult keeps — the
    /// CONSUME return, `SwingSource.NONE` no swing): the love clock
    /// arms 600 the gate reopens the decrement's walk, the verdict
    /// the love word carries. The HEART particles the event 18
    /// broadcast spawns the repo's render never draws yet (the
    /// particle lane the next round owes) — the state the verdict's
    /// own walk keeps.
    fn feed(&mut self) -> MobInteract {
        // THE AGE GATE (javap `Animal.mobInteract`'s own arm: `isFood`
        // then `getAge() == 0` the adult arm speaks — the baby the
        // canAgeUp arm takes, the breed cooldown the CONSUME keeps)
        if self.age == 0 && self.in_love <= 0 {
            // the setInLove walk: inLove = 600 (the loveCause the
            // EntityReference ride and the event 18 burst the
            // offline walk's own verdict stands for)
            self.in_love = 600;
            MobInteract::Feed { love: true, grow: false }
        } else if self.age < 0 {
            // THE BABY ARM (javap `Animal.mobInteract`'s canAgeUp arm
            // verbatim: `usePlayerItem` (the consume the offline
            // walk's own verdict stands for) + `ageUp(
            // getSpeedUpSecondsWhenFeeding(-age), true)` the
            // ten-percent growth law walks + `playEatingSound()` (the
            // base's own empty law the Sheep/Cow keep silent) — the
            // SUCCESS verdict the swing speaks, the heart burst the
            // setInLove arm the baby never rides)
            let amount = speed_up_seconds_when_feeding(-self.age);
            self.age_up(amount, true);
            MobInteract::Feed { love: false, grow: true }
        } else {
            // the already-in-love adult (or the cooldown the age gate
            // refuses): the CONSUME the isClientSide arm returns, the
            // press consumed the swing never starts
            MobInteract::Feed { love: false, grow: false }
        }
    }

    /// `AgeableMob.ageUp(IZ)` verbatim (the 26.2 full disasm speaks):
    /// `i = age; i += amount * 20; if (i > 0) i = 0;` — the ZERO
    /// CEILING the grow-up crossing never overshoots — then
    /// `setAge(i)` (the BOUNDARY BRANCH the sign flip arms — the
    /// crossing the setAge's own walk speaks), the over-grow arm
    /// `forcedAge += (the APPLIED delta — the new-minus-old fold,
    /// POSITIVE for the growth)` + the 40-tick sparkle window the
    /// zero timer marks — and the TAIL the lossless dump folds:
    /// `if (getAge() == 0) setAge(forcedAge);` — the BANKED SURPLUS
    /// the zero landing spends (the big jump the ceiling lands, the
    /// banked growth the positive age keeps — the natural tick's own
    /// drain the cooldown walks back to 0). The ten-percent feed the
    /// jump never crosses (the +n/10 shave the remaining walk keeps
    /// negative) — the crossing the ceiling's big jump or the
    /// natural tick's own beat owes. The boat lane the offline walk
    /// never rides (ageBoundaryReached the vehicle-less world
    /// keeps).
    fn age_up(&mut self, amount: i32, over_grow: bool) {
        let i = self.age;
        let mut j = i;
        j += amount * 20;
        if j > 0 {
            j = 0;
        }
        let k = j - i;
        self.set_age(j);
        if over_grow {
            self.forced_age += k;
            if self.forced_age_timer == 0 {
                self.forced_age_timer = FEED_FORCED_AGE_TIMER;
            }
        }
        // THE TAIL (the lossless disasm pcs 60-72: `getAge(); ifne
        // +75; setAge(forcedAge)` — the gate reads getAge AFTER the
        // over-grow arm, the spend the plain walk answers too —
        // forcedAge 0 the setAge(0) the same-sign set keeps)
        if self.age == 0 {
            self.set_age(self.forced_age);
        }
    }

    /// `AgeableMob.canAgeUp()` verbatim (the 26.2 full disasm speaks):
    /// `isBaby() && !isAgeLocked()` — the baby the age's own sign
    /// speaks, the lock the golden dandelion's word the gate reads —
    /// the locked baby the natural walk never grows (the PAUSE the
    /// dandelion's press speaks), the unlocked the everyday walk
    /// answers, the adult the gate refuses outright.
    pub fn can_age_up(&self) -> bool {
        self.age < 0 && !self.age_locked
    }

    /// THE SHEEP'S OWN ATE (Sheep.ate() verbatim — the 26.2 full disasm
    /// speaks: `super.ate(); setSheared(false); if (canAgeUp())
    /// ageUp(60)`). The super's own word (Mob.ate():
    /// `gameEvent(GameEvent.EAT)` the getstatic/invokevirtual pair
    /// speaks) the offline world's game-event lane keeps none of — the
    /// honest note the sculk-less world records. The sheared reset
    /// BOTH ages speak (the baby's wool the render's isBaby gate
    /// never draws anyway), the ageUp the one-arg arm's own call
    /// (ageUp(60) = ageUp(60, false) — the forcedAge lane the feed
    /// walk's over-grow keeps silent) the canAgeUp gate rides: the
    /// unlocked baby's +1200-tick jump the zero ceiling's own walk
    /// keeps (the crossing the grow-up answers — the graze the
    /// babyhood's own clock spends), the locked baby the gate refuses,
    /// the adult the gate refuses outright.
    fn ate(&mut self) {
        self.sheared = false;
        if self.can_age_up() {
            self.age_up(EAT_AGEUP_TICKS, false);
        }
    }

    /// `Sheep.getHeadEatPositionScale(F)F` verbatim (the 26.2 full
    /// disasm speaks): the `tick <= 0` arm the fconst_0 keeps, the
    /// `4 <= tick <= 36` window the iconst_1 keeps (the head's full
    /// drop the graze holds), the `tick < 4` ease-in
    /// `(tick - partial) / 4.0` the head's own descent speaks, the
    /// tail `-(tick - 40 - partial) / 4.0` the ease-out the window's
    /// own rise answers — the partial the render's frame rides.
    pub fn head_eat_position_at(&self, partial: f32) -> f32 {
        let t = self.eat_animation_tick;
        if t <= 0 {
            return 0.0;
        }
        if (EAT_CONSUME_TICK..=36).contains(&t) {
            return 1.0;
        }
        if t < EAT_CONSUME_TICK {
            (t as f32 - partial) / EAT_POSITION_DIV
        } else {
            -((t as f32 - EAT_ANIMATION_TICKS as f32 - partial) / EAT_POSITION_DIV)
        }
    }

    /// `Sheep.getHeadEatAngleScale(F)F` verbatim (the 26.2 full disasm
    /// speaks): the `4 < tick <= 36` window the chew wobble keeps —
    /// `0.62831855F + 0.21991149F * sin(((tick - 4 - partial) / 32.0) *
    /// 28.7F)` the graze's own chew (the 32.0 divisor the cp_probe
    /// walk names), the `0 < tick <= 4` arm the pi/5 hold speaks (the
    /// head pitched while the position eases in), the idle arm
    /// `getXRot(partial) * 0.017453292F` the head's own look pitch
    /// carries — the deg->rad fold the lerped pair answers.
    pub fn head_eat_angle_at(&self, partial: f32) -> f32 {
        let t = self.eat_animation_tick;
        if t > EAT_CONSUME_TICK && t <= 36 {
            let f = (t as f32 - EAT_CONSUME_TICK as f32 - partial) / EAT_ANGLE_DIV;
            EAT_ANGLE_BASE + EAT_ANGLE_AMP * f32::sin(f * EAT_ANGLE_FREQ)
        } else if t > 0 {
            EAT_ANGLE_BASE
        } else {
            let lerped = self.prev_head_pitch + (self.head_pitch - self.prev_head_pitch) * partial;
            lerped * EAT_IDLE_RAD
        }
    }

    /// `AgeableMob.canUseGoldenDandelion(ItemStack, Z, I, Mob)` verbatim
    /// (the 26.2 full disasm speaks): `stack.getItem() ==
    /// Items.GOLDEN_DANDELION && baby && timer == 0 &&
    /// !mob.getType().is(CANNOT_BE_AGE_LOCKED)` — the AND-chain the
    /// four branches speak (if_acmpne the item, ifeq the baby,
    /// ifne the timer's nonzero, ifne the in-tag refusal — all
    /// `start+32` the iconst_0 arm). The `baby` arg the caller's
    /// `isBaby()` read rides (the `age < 0` word the derived fold
    /// answers), the `timer` arg the `ageLockParticleTimer` field
    /// speaks — the 40-tick window's own drain the re-use gate
    /// requires. The CANNOT_BE_AGE_LOCKED tag the zoo's AgeableMob
    /// kinds (Sheep/Cow/Pig) the jar's own tag json never carries —
    /// the honest always-pass the zoo's kinds keep; the ZOMBIE the
    /// monster chain never reaches the AgeableMob arm (the kind gate
    /// the chain's own walk speaks).
    pub fn can_use_golden_dandelion(&self, held_item: &str) -> bool {
        match self.kind {
            MobKind::Sheep | MobKind::Cow | MobKind::Pig => {
                held_item == "golden_dandelion" && self.age < 0
                    && self.age_lock_particle_timer == 0
            }
            MobKind::Zombie => false,
        }
    }

    /// THE GOLDEN-DANDELION'S OWN WALK (javap `AgeableMob.setAgeLocked`
    /// static + `setAgeLockedData` the consumer the full disasm speaks):
    /// the consumer `setAgeLockedData()` runs FIRST —
    /// `setAgeLocked(!isAgeLocked())` the TOGGLE (ifne the locked arm's
    /// iconst_0, the unlocked's iconst_1 — both directions the same
    /// call answers), `setAge(getBabyStartAge())` the UNCONDITIONAL
    /// re-baby the BOTH directions walk (the -24000 the lock freezes,
    /// the unlock's RESET the fresh babyhood speaks), `ageLockParticleTimer
    /// = 40` the PAUSE/RESET window the both arms mark — then the
    /// static's own tail: `stack.consume(1, player)` the dandelion the
    /// press spends (the inventory-less lane the Feed verdict's own
    /// note honest-skips), the `isAgeLocked()` post-toggle read the
    /// persistence + sound picks carry (`setPersistenceRequired` the
    /// locked arm the despawn-less world honest-skips; the playSound
    /// `locked ? GOLDEN_DANDELION_USE : GOLDEN_DANDELION_UNUSE` the
    /// PLAYERS source v1.0 p1.0 keeps — the caller's sound lane plays).
    /// The SUCCESS verdict the swing speaks.
    fn use_golden_dandelion(&mut self) -> MobInteract {
        // THE CONSUMER (setAgeLockedData verbatim — the toggle, the
        // re-baby, the window)
        self.age_locked = !self.age_locked;
        self.set_age(self.baby_start_age());
        self.age_lock_particle_timer = AGE_LOCK_PARTICLE_WINDOW;
        MobInteract::AgeLock { locked: self.age_locked }
    }

    /// `AgeableMob.getBabyStartAge()` verbatim (the disasm's own
    /// `return -24000` — the same walk setBaby/finalizeSpawn ride).
    fn baby_start_age(&self) -> i32 {
        -24000
    }

    /// `AgeableMob.setAge(I)` verbatim (the 26.2 full disasm speaks):
    /// the old age the loc2 fold saves, the field the new writes, the
    /// BOUNDARY BRANCH the sign flip arms — `old < 0 != new < 0` —
    /// the DATA_BABY_ID write the derived is_baby() read keeps + the
    /// ageBoundaryReached() call the crossing speaks. Returns whether
    /// the set crossed the zero boundary (the adult↔baby flip the
    /// wire flag's own walk names — every setAge caller rides it:
    /// the natural tick's grow-up, the feed's big jump, the birth's
    /// -24000, the parents' 6000 cooldown the same-sign set the
    /// branch never arms).
    pub fn set_age(&mut self, new_age: i32) -> bool {
        let old = self.age;
        self.age = new_age;
        let crossed = (old < 0) != (new_age < 0);
        if crossed {
            self.age_boundary_reached();
        }
        crossed
    }

    /// `AgeableMob.ageBoundaryReached()` verbatim (the 26.2 full
    /// disasm speaks): `if (!isBaby() && isPassenger() && getVehicle()
    /// instanceof AbstractBoat boat && !boat.hasEnoughSpaceFor(this))
    /// stopRiding();` — the grown-up the boat's own room gate
    /// dismounts (the isBaby FIRST gate the re-babying crossing
    /// rides — the still-baby the arm never speaks). The repo's
    /// offline walk keeps no passengers and no boats — the gate
    /// chain the vehicle-less world answers with the honest still
    /// (every term the walk keeps false), the call chain the
    /// crossing's own walk keeps for the vehicle rounds to come.
    fn age_boundary_reached(&mut self) {
        // the verbatim gate chain: !is_baby() && passenger && boat
        // && !hasEnoughSpaceFor -> stopRiding() — the no-vehicle
        // world the empty arm answers.
    }

    /// `AgeableMob.isBaby` — the gate's own word (the `age < 0` the
    /// single field speaks: the birth's -24000 the baby names, the
    /// grow-up the zero crossing closes; the vanilla's DATA_BABY_ID
    /// bit + age split the wire channel keeps, the verdicts the same
    /// law answers).
    pub fn is_baby(&self) -> bool {
        self.age < 0
    }

    /// `Sheep.readyForShearing` — the gate's own walk (javap: `return
    /// !isSheared() && !isBaby()`): the wool byte's sheared bit the
    /// first arm, the age's baby word the second (the growth vocabulary
    /// the breed lane arms — the baby's coat the shear never takes).
    pub fn ready_for_shearing(&self) -> bool {
        !self.sheared && !self.is_baby()
    }

    /// THE SHEAR'S OWN WALK (javap `Sheep.shear(ServerLevel,
    /// SoundSource, ItemStack)` — the 26.2 bytecode the dump keeps:
    /// `playSound(null, this, SoundEvents.SHEEP_SHEAR, source, 1.0F,
    /// 1.0F)`, `dropFromShearingLootTable(level,
    /// BuiltInLootTables.SHEAR_SHEEP, stack, lambda$0)`,
    /// `setSheared(true)`): the wool byte's sheared bit sets first
    /// (the fur layer's own gate closes), the loot rides the
    /// shearing/sheep/<color> table the getColor keys — the pool's
    /// uniform 1.0..3.0 rolls (the inclusive nextInt law), the
    /// lambda's own loop the spawned entities walk (`copyWithCount
    /// (1)` per spawn — the singles the vec speaks). The playSound
    /// rides the caller's voice lane.
    fn shear(&mut self, rng: &mut Lcg) -> Vec<i32> {
        self.sheared = true;
        // the pool's own roll (uniform 1..3 — min + nextInt(max - min + 1))
        let count = rng.next_int_inclusive(1, 3);
        // the table's own item: `<name>_wool` the DyeColor names
        let name = format!("{}_wool", DYE_NAMES[(self.wool_color & 0x0F) as usize]);
        match wire_id(&name) {
            Some(id) => vec![id; count as usize],
            // the honest vanish: the name the registry never drew
            // invents no id (the wire_id law the drop lane speaks)
            None => Vec::new(),
        }
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

/// THE DYE-PAIR MIX TABLE (javap DyeColor.getMixedColor — the 26.2
/// jar's Sheep.getBreedOffspring walk the sheep_262 dump keeps: the
/// parents' dyes the crafting grid pairs, the recipe the result dye
/// speaks — the nine two-dye recipes the dye crafting law keeps,
/// the unordered pairs stored (low, high) the walk's own sort
/// answers). WHITE+GRAY the light-gray doubles, PURPLE+PINK the
/// magenta doubles, the rest the single result dye — every row the
/// shapeless grid speaks both ways.
pub const DYE_MIX_TABLE: [((u8, u8), u8); 9] = [
    ((0, 8), 8),    // WHITE + LIGHT_GRAY -> LIGHT_GRAY
    ((0, 11), 3),   // WHITE + BLUE -> LIGHT_BLUE
    ((0, 13), 5),   // WHITE + GREEN -> LIME
    ((0, 14), 6),   // WHITE + RED -> PINK
    ((0, 15), 7),   // WHITE + BLACK -> GRAY
    ((4, 14), 1),   // YELLOW + RED -> ORANGE
    ((6, 10), 2),   // PINK + PURPLE -> MAGENTA
    ((11, 13), 9),  // BLUE + GREEN -> CYAN
    ((11, 14), 10), // BLUE + RED -> PURPLE
];

/// THE OFFSPRING'S OWN COLOR WALK (javap `Sheep.getBreedOffspring`:
/// `setColor(DyeColor.getMixedColor(level, a, b))` — the parents'
/// dyes the crafting grid pairs, the recipe result the baby speaks;
/// the recipe-less pair the `nextBoolean` coin the fallback arm
/// keeps — one parent's dye the coin picks, the same-dye pair the
/// identical wool answers). The mix arm the coin never reaches
/// (the else arm's own draw the vanilla skips — the stream the xp
/// roll rides keeps its two-draw span).
pub fn mixed_wool_color(first: u8, second: u8, rng: &mut Lcg) -> u8 {
    let (lo, hi) = if first <= second {
        (first, second)
    } else {
        (second, first)
    };
    for ((a, b), out) in DYE_MIX_TABLE {
        if a == lo && b == hi {
            return out;
        }
    }
    // the fallback arm's own coin (javap: nextBoolean ? first : second)
    if rng.next() & 1 == 0 {
        first
    } else {
        second
    }
}

/// THE BREED'S OWN OUT-EDGE (the spawn walk the caller answers): the
/// child the addFreshEntityWithPassengers arms, the HEART burst the
/// event 18 broadcast spawns (the spawning parent's own box the
/// broadcast names), the XP orb the MOB_DROPS gamerule's roll owes
/// (`nextInt(7)` — the 0..6 band the zero roll honestly speaks — no
/// orb, exactly the vanilla's own sometimes-silent breed).
pub struct BreedSpawn {
    pub child: Zombie,
    /// the spawning parent's bbox snapshot (the event 18 broadcast's
    /// own ask — the caller spawns the burst hearts over it)
    pub heart_bbox: (f64, f64, f64, f64, f64, f64),
    /// the orb the walk drops: (x, y, z, `nextInt(7)`) — the zero roll
    /// the caller skips (no orb, the vanilla's own honest silence)
    pub orb: (f64, f64, f64, i32),
}

/// THE BREEDING WALK (javap `Animal` the 26.2 dump keeps): the
/// `canMate` law — same class, both `isInLove()` — the pair the scan
/// answers, the `spawnChildFromBreeding` walk — `getBreedOffspring`
/// the child creates, `setBaby(true)` the -24000 age speaks, the
/// `snapTo` at THIS parent's own x/y/z — and the
/// `finalizeSpawnChildFromBreeding` tail — `setAge(6000)` BOTH
/// parents, `resetLove()` both, `broadcastEntityEvent(18)` the HEARTS,
/// the MOB_DROPS-gated `ExperienceOrb` at the parent's own spot. The
/// `bred` flags the double-pair the same pass refuses (an in-love
/// parent breeds once per scan — the goal's own one-shot law). The
/// child's RNG seed, the wool arm's coin (the recipe-less fallback
/// only — DYE_MIX_TABLE the recipe arm answers, the mix the coin
/// draw never consumes) and the orb roll draw the shared stream.
/// THE BREEDGOAL'S OWN TRIGGER the spawn gate reads (javap
/// `BreedGoal.tick` the breedgoal dump keeps): the court the goal's
/// own `loveTime >= adjustedTickDelay(60)` speaks, the meet the
/// `distanceToSqr(partner) < 9.0` close ring answers — the pair the
/// court WALKED together, the child the meet's own tick leaves. The
/// caller's atomic scan the mutual court's both-counters fold keeps
/// (the symmetric goals arm together, the clocks count together;
/// the pairing the proximity law the canMate gate answers — the
/// armed partner identity the goal's own walk steers, the spawn the
/// same verdict the one-shot law folds).
pub fn try_breed(mobs: &mut [Zombie], next_id: &mut i32, rng: &mut Lcg) -> Vec<BreedSpawn> {
    let mut out = Vec::new();
    let mut bred = vec![false; mobs.len()];
    for i in 0..mobs.len() {
        for j in (i + 1)..mobs.len() {
            if bred[i] || bred[j] {
                continue;
            }
            let (a, b) = (&mobs[i], &mobs[j]);
            // THE CANMATE LAW (javap: `other != this`, the same class,
            // both `isInLove()` — the kinds the same variant names)
            if a.kind != b.kind || a.in_love <= 0 || b.in_love <= 0 {
                continue;
            }
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            let dz = a.z - b.z;
            // THE BREED TRIGGER'S OWN VERDICT (javap `BreedGoal.tick`:
            // the `loveTime >= adjustedTickDelay(60)` the court
            // speaks, the `distanceToSqr(partner) < 9.0` the meet the
            // close ring answers — the approach walk the goal's own
            // moveTo closes, the miss the re-court law restarts)
            if dx * dx + dy * dy + dz * dz >= BREED_CLOSE_SQ {
                continue;
            }
            if mobs[i].breed_love_time < BREED_COURT_TICKS
                || mobs[j].breed_love_time < BREED_COURT_TICKS
            {
                continue;
            }
            // THE PAIR COOLS (javap `finalizeSpawnChildFromBreeding`:
            // `setAge(6000)` both, `resetLove()` both — the setAge
            // walk the boundary branch rides: adult 0 -> 6000 the
            // same-sign set the crossing never arms)
            mobs[i].set_age(BREED_COOLDOWN_TICKS);
            mobs[i].in_love = 0;
            mobs[j].set_age(BREED_COOLDOWN_TICKS);
            mobs[j].in_love = 0;
            // THE CHILD (javap `spawnChildFromBreeding`:
            // `setBaby(true)`, the `snapTo` at THIS parent's own x/y/z,
            // the `getBabyStartAge` -24000 the birth arms — the setAge
            // walk the boundary branch rides: the fresh body 0 ->
            // -24000 the crossing arms, the still-baby the
            // ageBoundaryReached's isBaby gate keeps silent — the
            // vanilla's own order the boat lane never hears)
            let mut child = Zombie::new(*next_id, mobs[i].x, mobs[i].y, mobs[i].z, rng.next());
            *next_id += 1;
            child.kind = mobs[i].kind;
            child.set_age(BABY_START_AGE);
            child.on_ground = true;
            // THE OFFSPRING'S OWN COLOR (javap `Sheep.getBreedOffspring`:
            // `setColor(DyeColor.getMixedColor(level, a, b))` — the
            // parents' dyes the crafting recipe pairs, the recipe-less
            // pair the nextBoolean coin the fallback arm keeps)
            if child.kind == MobKind::Sheep {
                child.wool_color = mixed_wool_color(mobs[i].wool_color, mobs[j].wool_color, rng);
            }
            // THE ORB ROLL (javap: `random.nextInt(7)` — the 0..6 the
            // MOB_DROPS gate keeps)
            let xp_roll = (rng.next() % 7) as i32;
            out.push(BreedSpawn {
                child,
                heart_bbox: mobs[i].box_for(),
                orb: (mobs[i].x, mobs[i].y, mobs[i].z, xp_roll),
            });
            bred[i] = true;
            bred[j] = true;
        }
    }
    out
}

/// THE FOLLOWPARENTGOAL'S OWN SCAN (javap `canUse` the followparentgoal
/// dump keeps — the canUse walk the caller's pre-pass answers before
/// the ticks, the goals' own moment): every BABY (`getAge() < 0` the
/// first gate) scans the same-kind ADULTS (`getAge() >= 0` the loop's
/// own skip, `isAlive` the `health > 0.0` word) whose box intersects
/// the baby's own bbox `inflate(8.0, 4.0, 8.0)` (the getEntitiesOfClass
/// ring the AABB.intersects inclusive law keeps), the nearest by the
/// feet-to-feet `distanceToSqr` (the `d1 > d0` strict-greater skip —
/// the first minimum the list order keeps), the arm only past the
/// close ring (`d0 >= 9.0` — the 3.0 the squared word speaks). The
/// armed tuple (the parent's id + the position snapshot) the tick's
/// straight-line stand-in steers to; every other baby the disarmed
/// None keeps — the panicked baby the goal's own MOVEL flag the
/// panic's prio-1 walk owns (the GoalSelector's conflict law the stop
/// answers). Returns the armed count (the caller's evidence).
pub fn arm_follow_parents(mobs: &mut [Zombie]) -> usize {
    // the adults' own snapshot (the level query's stand-in — the read
    // the per-entity walk's borrow keeps)
    let adults: Vec<(i32, MobKind, f64, f64, f64, (f64, f64, f64, f64, f64, f64))> = mobs
        .iter()
        .filter(|m| m.age >= 0 && m.health > 0.0)
        .map(|m| (m.id, m.kind, m.x, m.y, m.z, m.box_for()))
        .collect();
    let mut armed = 0;
    for i in 0..mobs.len() {
        if mobs[i].age >= 0 {
            continue;
        }
        if mobs[i].is_panicking() {
            // the goal's own MOVEL flag the panic's prio-1 walk owns
            // (the GoalSelector's conflict law the flag keeps) — the
            // stop the disarmed None answers
            mobs[i].follow_parent = None;
            continue;
        }
        let baby = &mobs[i];
        let (bminx, bminy, bminz, bmaxx, bmaxy, bmaxz) = baby.box_for();
        // the scan box (the inflate(8.0, 4.0, 8.0) the canUse speaks)
        let (minx, miny, minz) = (
            bminx - FOLLOW_PARENT_RANGE,
            bminy - FOLLOW_PARENT_HEIGHT,
            bminz - FOLLOW_PARENT_RANGE,
        );
        let (maxx, maxy, maxz) = (
            bmaxx + FOLLOW_PARENT_RANGE,
            bmaxy + FOLLOW_PARENT_HEIGHT,
            bmaxz + FOLLOW_PARENT_RANGE,
        );
        let mut best: Option<(i32, f64, f64, f64, f64)> = None;
        for (id, kind, ax, ay, az, abox) in &adults {
            if *id == baby.id || *kind != baby.kind {
                continue;
            }
            // the box law (AABB.intersects — the inclusive fold)
            let (aminx, aminy, aminz, amaxx, amaxy, amaxz) = *abox;
            if amaxx < minx
                || aminx > maxx
                || amaxy < miny
                || aminy > maxy
                || amaxz < minz
                || aminz > maxz
            {
                continue;
            }
            let dx = ax - baby.x;
            let dy = ay - baby.y;
            let dz = az - baby.z;
            let d2 = dx * dx + dy * dy + dz * dz;
            // the nearest wins (d1 > d0 the skip — ties keep the first)
            if best.map_or(true, |(_, bd2, _, _, _)| d2 <= bd2) {
                best = Some((*id, d2, *ax, *ay, *az));
            }
        }
        mobs[i].follow_parent = match best {
            Some((id, d2, x, y, z)) if d2 >= FOLLOW_PARENT_CLOSE_SQ => {
                armed += 1;
                Some((id, x, y, z))
            }
            _ => None,
        };
    }
    armed
}

/// THE BREEDGOAL'S OWN SCAN (javap `canUse` + `getFreePartner` the
/// breedgoal dump keeps — the canUse walk the caller's pre-pass
/// answers before the ticks, the goals' own moment): every IN-LOVE
/// ADULT (the `isInLove()` first gate, the `getAge() >= 0` the
/// canMate law keeps, the `isAlive` health word) scans the same-kind
/// in-love adults whose box intersects its own bbox `inflate(8.0)`
/// (the getNearbyEntities ring the AABB.intersects inclusive law
/// keeps — the uniform 8.0 the single-double inflate speaks), the
/// nearest by the feet-to-feet `distanceToSqr` (the `d1 > d0`
/// strict-greater skip — the first minimum the list order keeps),
/// the isPanicking gate the getFreePartner's own candidate walk
/// speaks (the breedgoal dump's `candidate.isPanicking() -> skip`
/// line — the panicked mate the pairing refuses; the breeder's own
/// panic the goal's MOVEL flag the prio-1 walk owns, the
/// GoalSelector's conflict law the court's stop answers). The armed
/// tuple (the mate's id + the position
/// snapshot) the tick's `moveTo(partner, speedModifier)` stand-in
/// steers to; the court's own clock the arm ride keeps (the goal's
/// lifetime — only the disarm or the 60-expiry the tick's own stop
/// law resets), every other breeder the disarmed None + the zeroed
/// clock (the stop() law: the partner nulls, the loveTime zeroes —
/// the cooled pair the love word's own gate refuses). Returns the
/// armed count (the caller's evidence).
pub fn arm_breeders(mobs: &mut [Zombie]) -> usize {
    // the mates' own snapshot (the level query's stand-in — the read
    // the per-entity walk's borrow keeps)
    let mates: Vec<(i32, MobKind, f64, f64, f64, (f64, f64, f64, f64, f64, f64))> = mobs
        .iter()
        .filter(|m| m.age >= 0 && m.health > 0.0 && m.in_love > 0 && !m.is_panicking())
        .map(|m| (m.id, m.kind, m.x, m.y, m.z, m.box_for()))
        .collect();
    let mut armed = 0;
    for i in 0..mobs.len() {
        // the canUse first gate (isInLove) — the baby's age word and
        // the dying body the canMate law keeps: the goal's stop the
        // disarmed None answers (the cooled pair's own clock rests)
        if mobs[i].age < 0
            || mobs[i].in_love <= 0
            || mobs[i].health <= 0.0
            || mobs[i].is_panicking()
        {
            mobs[i].breed_partner = None;
            mobs[i].breed_love_time = 0;
            continue;
        }
        let breeder = &mobs[i];
        let (bminx, bminy, bminz, bmaxx, bmaxy, bmaxz) = breeder.box_for();
        // the scan box (the inflate(8.0) the getFreePartner speaks —
        // the uniform ring the single-double law keeps)
        let (minx, miny, minz) = (
            bminx - BREED_SEARCH_RANGE,
            bminy - BREED_SEARCH_RANGE,
            bminz - BREED_SEARCH_RANGE,
        );
        let (maxx, maxy, maxz) = (
            bmaxx + BREED_SEARCH_RANGE,
            bmaxy + BREED_SEARCH_RANGE,
            bmaxz + BREED_SEARCH_RANGE,
        );
        let mut best: Option<(i32, f64, f64, f64, f64)> = None;
        for (id, kind, mx, my, mz, mbox) in &mates {
            if *id == breeder.id || *kind != breeder.kind {
                continue;
            }
            // the box law (AABB.intersects — the inclusive fold)
            let (aminx, aminy, aminz, amaxx, amaxy, amaxz) = *mbox;
            if amaxx < minx
                || aminx > maxx
                || amaxy < miny
                || aminy > maxy
                || amaxz < minz
                || aminz > maxz
            {
                continue;
            }
            let dx = mx - breeder.x;
            let dy = my - breeder.y;
            let dz = mz - breeder.z;
            let d2 = dx * dx + dy * dy + dz * dz;
            // the nearest wins (d1 > d0 the skip — ties keep the first)
            if best.map_or(true, |(_, bd2, _, _, _)| d2 <= bd2) {
                best = Some((*id, d2, *mx, *my, *mz));
            }
        }
        match best {
            Some((id, _d2, x, y, z)) => {
                // the arm (or the re-arm) — the fresh snapshot the
                // per-frame re-arm speaks, the court's own clock the
                // goal's lifetime keeps
                mobs[i].breed_partner = Some((id, x, y, z));
                armed += 1;
            }
            None => {
                // the partner-less canUse the stop law answers
                mobs[i].breed_partner = None;
                mobs[i].breed_love_time = 0;
            }
        }
    }
    armed
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
        assert!(
            (z.health - (20.0 - 1.92)).abs() < 1e-4,
            "health {}",
            z.health
        );
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

    /// THE VOICE TABLE (the vanilla `getAmbientSound` / `getHurtSound`
    /// / `getDeathSound` rows the four drawn kinds speak): the twelve
    /// registry names the audio lane's clip table answers, the
    /// from_wire_name round trip the wire spawn rides, and the seed
    /// law's own decorrelation — different ids never groan in unison.
    #[test]
    fn the_voice_table_speaks_the_vanilla_names() {
        use crate::mob::{entity_voice_seed, MobKind, VoiceSlot};
        assert_eq!(
            MobKind::Zombie.voice_event(VoiceSlot::Ambient),
            "entity.zombie.ambient"
        );
        assert_eq!(
            MobKind::Zombie.voice_event(VoiceSlot::Hurt),
            "entity.zombie.hurt"
        );
        assert_eq!(
            MobKind::Zombie.voice_event(VoiceSlot::Death),
            "entity.zombie.death"
        );
        assert_eq!(
            MobKind::Sheep.voice_event(VoiceSlot::Ambient),
            "entity.sheep.ambient"
        );
        assert_eq!(
            MobKind::Sheep.voice_event(VoiceSlot::Hurt),
            "entity.sheep.hurt"
        );
        assert_eq!(
            MobKind::Sheep.voice_event(VoiceSlot::Death),
            "entity.sheep.death"
        );
        assert_eq!(
            MobKind::Cow.voice_event(VoiceSlot::Ambient),
            "entity.cow.ambient"
        );
        assert_eq!(MobKind::Cow.voice_event(VoiceSlot::Hurt), "entity.cow.hurt");
        assert_eq!(
            MobKind::Cow.voice_event(VoiceSlot::Death),
            "entity.cow.death"
        );
        assert_eq!(
            MobKind::Pig.voice_event(VoiceSlot::Ambient),
            "entity.pig.ambient"
        );
        assert_eq!(MobKind::Pig.voice_event(VoiceSlot::Hurt), "entity.pig.hurt");
        assert_eq!(
            MobKind::Pig.voice_event(VoiceSlot::Death),
            "entity.pig.death"
        );
        // the round trip: the wire spawn's own walk (the registry name
        // the Add Entity spoke -> the kind -> the voice)
        for (name, kind) in [
            ("minecraft:zombie", MobKind::Zombie),
            ("minecraft:sheep", MobKind::Sheep),
            ("minecraft:cow", MobKind::Cow),
            ("minecraft:pig", MobKind::Pig),
        ] {
            assert_eq!(MobKind::from_wire_name(name), kind);
            assert_eq!(
                MobKind::from_wire_name(name).voice_event(VoiceSlot::Hurt),
                kind.voice_event(VoiceSlot::Hurt),
            );
        }
        // THE SEED LAW'S OWN DECORRELATION (the vanilla independent
        // per-entity random the wire lane's stand-in answers): the same
        // id replays the same stream, different ids never share one
        assert_eq!(entity_voice_seed(7), entity_voice_seed(7));
        assert_ne!(entity_voice_seed(7), entity_voice_seed(9));
        assert_ne!(entity_voice_seed(0), entity_voice_seed(1));
    }

    /// THE AMBIENT GATE'S OWN METHOD (the shared form the wire lane's
    /// tick rides): the law replayed by hand over the same seed — the
    /// clock starts at -80, one draw per tick, the draw under the
    /// climbing clock fires and re-arms at the -80 the reset speaks.
    /// (The method's own stream, not the full tick's — the local tick
    /// spends its own LCG on the goals' draws before the gate.)
    #[test]
    fn the_ambient_gate_method_speaks_the_base_tick_law() {
        let seed = 11u64;
        let mut z = Zombie::new(1, 0.0, 0.0, 0.0, seed);
        let mut replay = crate::drops::Lcg::new(seed);
        let mut clock: i32 = -AMBIENT_INTERVAL_TICKS;
        let mut fires = 0;
        for _ in 0..3000 {
            clock += 1;
            let draw = replay.next() % AMBIENT_DRAW_BOUND;
            let expect_fire = draw < clock.max(0) as u64;
            if expect_fire {
                clock = -AMBIENT_INTERVAL_TICKS;
            }
            assert_eq!(z.ambient_voice(), expect_fire, "one law, one answer");
            if expect_fire {
                fires += 1;
            }
        }
        assert!(fires > 0, "the gate fired within the walk");
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

    /// A grass pad (the graze tests' own arena — the flat stone pad's
    /// sibling: the grass block the sheep's own below-cell arms) —
    /// stone 0..3, the grass block at 4, the air above, the surface
    /// top at y = 5: the feet cell (x, 5, z) the AIR keeps, the below
    /// cell (x, 4, z) the GRASS speaks.
    fn graze_world() -> World {
        let mut world = World::new(1, 42);
        for x in 0..world.size_blocks() {
            for z in 0..world.size_blocks() {
                for y in 5..world.size_blocks() {
                    world.set(x, y, z, crate::world::AIR);
                }
                world.set(x, 4, z, crate::world::GRASS);
                for y in 0..4 {
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

    /// THE STEP VOICE'S OWN SPLIT (javap the 26.2 jar's playStepSound
    /// pair): the zombie's own override the FLAT fold keeps — every
    /// surface (even the air the caller never passes) the same voice,
    /// no block state the fold reads; the animals override NOTHING —
    /// the Entity base impl the state's SoundType speaks (the
    /// getStepSound() event at getVolume() * 0.15F / getPitch()), the
    /// grass pad (1.0, 1.0) the fold walks to block.grass.step at
    /// 0.15/1.0, the stone pad the same base pair answers. The air
    /// and the unknown block the None keeps — the vanilla never
    /// hears them either.
    #[test]
    fn the_step_voice_splits_the_kinds_the_block_law_speaks() {
        // the zombie's flat override — two surfaces, one voice
        for under in [None, Some("grass_block"), Some("stone")] {
            assert_eq!(
                MobKind::Zombie.step_voice(under),
                Some(("entity.zombie.step", ZOMBIE_STEP_VOLUME, ZOMBIE_STEP_PITCH)),
                "the zombie's flat fold answers every surface"
            );
        }
        // the animals' block law — the SoundType's own base the fold walks
        for kind in [MobKind::Sheep, MobKind::Cow, MobKind::Pig] {
            let (event, v, p) = kind
                .step_voice(Some("grass_block"))
                .expect("the grass pad's voice");
            assert_eq!(event, "block.grass.step", "the grass pad's event");
            assert_eq!((v, p), (0.15, 1.0), "the grass base pair the fold walks");
            let (event, v, p) = kind
                .step_voice(Some("stone"))
                .expect("the stone pad's voice");
            assert_eq!(event, "block.stone.step", "the stone pad's event");
            assert_eq!((v, p), (0.15, 1.0), "the stone base pair the fold walks");
            // the silent lanes the None keeps
            assert_eq!(kind.step_voice(None), None, "the air never speaks");
            assert_eq!(
                kind.step_voice(Some("not_a_block")),
                None,
                "the unknown block the table never names"
            );
        }
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
            &mut mobs,
            &mut next_id,
            px,
            pz,
            &world,
            &mut rng,
            false
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
        assert!(
            ((third - 700)..=(third + 700)).contains(&zeros),
            "the zero band {zeros}"
        );
        assert!(
            ((third - 700)..=(third + 700)).contains(&ones),
            "the one band {ones}"
        );
        assert!(
            ((third - 700)..=(third + 700)).contains(&twos),
            "the two band {twos}"
        );
        // the rare: 0.025 of the player kills (the wide band)
        let expect = n as f64 * RARE_DROP_CHANCE as f64;
        assert!(
            ((expect - 70.0)..=(expect + 70.0)).contains(&(rares_n as f64)),
            "the rare band {rares_n}"
        );
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
    /// body never re-ignites while it stands in the water. The
    /// all-surface ocean keeps the walk wet — the stroll's own
    /// in-water flee the land pick never answers (every column
    /// water, the vanilla null the canUse gate speaks), the burn
    /// chain's !isInWater gate the wet body never passes.
    #[test]
    fn the_water_drowns_the_brand() {
        let mut world = wide_flat_world();
        for x in 0..world.size_blocks() {
            for z in 0..world.size_blocks() {
                world.set(x, 5, z, crate::world::WATER);
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
            assert_eq!(z.fire_ticks, 0, "the ocean keeps it out");
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

    /// THE WOOL TINT'S MODIFIED TABLE (javap ColorLerper.getModifiedColor's
    /// own law — the colorByDye map the ColorLerper$Type.SHEEP ctor
    /// pre-speaks): WHITE never multiplies (the method's first branch
    /// returns the literal — the 230-grey the vanilla white wool
    /// wears), every other dye's diffuse color rides the 0.75F
    /// modifier per channel, the floor the bytecode speaks (red 191
    /// not 255 — the raw table the earlier wool rode one step bright).
    #[test]
    fn the_wool_tint_rides_the_modified_table() {
        assert_eq!(
            sheep_wool_tint(0),
            0xE6E6E6,
            "the WHITE branch's own literal"
        );
        // RED 0xFF0000: floor(255 * 0.75) = 191 = 0xBF, the others 0
        assert_eq!(sheep_wool_tint(14), 0xBF0000, "the red the modifier speaks");
        // BLACK 0x000000: floor(0 * 0.75) = 0 — the same black both words speak
        assert_eq!(sheep_wool_tint(15), 0x000000, "the black stands");
        // GREEN 0x00FF00: floor(255 * 0.75) = 191 the green channel rides
        assert_eq!(sheep_wool_tint(13), 0x00BF00, "the green's own fold");
        // ORANGE 0xFF681F: (191, 78, 23) the floor walk speaks
        assert_eq!(sheep_wool_tint(1), 0xBF4E17, "the orange's three floors");
    }

    /// THE JEB_ RAINBOW'S OWN RAMP (javap ColorLerper.getLerpedColor +
    /// ARGB.srgbLerp + Mth.lerpInt — the exact laws the raw
    /// disassembly speaks): the window (floor(time)/25) % 16 arms, the
    /// blend ((floor(time) % 25) + frac(time)) / 25 rides 0→1 across
    /// each 25-tick window, and the lerp is a + floor(t * (b - a)) per
    /// channel (the floor's negative half the darken steps ride).
    #[test]
    fn the_jeb_rainbow_lerps_the_modified_colors() {
        // t = 0 at the window's own head: the color HOLDS at the boundary
        assert_eq!(jeb_rainbow_rgb(0.0), 0xE6E6E6, "the white the cycle opens");
        assert_eq!(
            jeb_rainbow_rgb(25.0),
            0xBF4E17,
            "the orange the second window opens (the color never skips)"
        );
        // mid-window 0.5: 230 + floor(0.5 * (191 - 230)) = 230 - 20 = 210
        // (the negative half floors DOWN — the -0.5 the truncation never
        // rides), 230 - 76 = 154, 230 - 104 = 126
        assert_eq!(
            jeb_rainbow_rgb(12.5),
            0xD29A7E,
            "the half-step the floor lerp speaks"
        );
        // the cycle's own tail: i = 399, the BLACK→WHITE window at
        // t = (24 + 0.9) / 25 = 0.996 — 0 + floor(0.996 * 230) = 229
        assert_eq!(
            jeb_rainbow_rgb(399.9),
            0xE5E5E5,
            "the 400-tick cycle's own return to white"
        );
        // the second lap rides the same 16 windows (i / 25 % 16 wraps)
        assert_eq!(
            jeb_rainbow_rgb(425.0),
            jeb_rainbow_rgb(25.0),
            "the lap's own wrap"
        );
    }

    #[test]
    fn the_dye_names_speak_the_shear_tables() {
        // the jar's own table walk: shearing/sheep/<name>.json, one
        // table per dye (the loot_table dir's own 16 entries), the
        // item `<name>_wool` each table's entry names — every name
        // the registry draws (the wire_id law: the miss invents no id)
        for name in DYE_NAMES.iter() {
            assert!(
                wire_id(&format!("{name}_wool")).is_some(),
                "{name}_wool the item table draws"
            );
        }
        // the count the table dir speaks (one per DyeColor ordinal)
        assert_eq!(DYE_NAMES.len(), 16);
        // the two ends the tint test speaks (0 white, 14 red, 15 black)
        assert_eq!(DYE_NAMES[0], "white");
        assert_eq!(DYE_NAMES[14], "red");
        assert_eq!(DYE_NAMES[15], "black");
    }

    #[test]
    fn the_shear_walks_the_sheep_law() {
        // Sheep.mobInteract's own verdict: the shears the held stack
        // speaks, the readyForShearing gate open — the Shear the
        // shearing/sheep/red table rolls (wool_color 14 the dye's own
        // ordinal), the count the uniform 1..3, the singles the
        // lambda's copyWithCount(1) speaks, the sheared bit set after
        // (setSheared(true) — the second press the closed gate refuses)
        let mut mob = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        mob.kind = MobKind::Sheep;
        mob.wool_color = 14;
        let red = wire_id("red_wool").expect("red_wool the table draws");
        let verdict = mob.mob_interact("shears", &mut Lcg::new(42));
        match verdict {
            MobInteract::Shear(drops) => {
                assert!(
                    (1..=3).contains(&drops.len()),
                    "the uniform 1..3 the pool speaks, got {}",
                    drops.len()
                );
                assert!(drops.iter().all(|d| *d == red), "the red table's own item");
                // the seed's own roll: Lcg(42)'s first next() % 3 = 1
                // — the uniform's min + roll speaks 2
                assert_eq!(drops.len(), 2, "the seed 42 roll the law speaks");
            }
            other => panic!("expected the Shear, got {other:?}"),
        }
        assert!(mob.sheared, "setSheared(true) the walk closes with");
        assert!(
            !mob.ready_for_shearing(),
            "readyForShearing's own false the bit arms"
        );
        assert_eq!(
            mob.mob_interact("shears", &mut Lcg::new(42)),
            MobInteract::Pass,
            "the sheared sheep the gate refuses"
        );
    }

    #[test]
    fn the_shear_gate_refuses_the_wrong_arms() {
        // the miss arms the super's PASS speaks: the zombie's own
        // table empty (the held shears the body's kind refuses), the
        // sheep's FEEDING lane the Animal super now WALKS (the wheat
        // the food arm answers — the setInLove the 600 arms), the
        // bare hand the gate refuses
        let mut sheep = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        sheep.kind = MobKind::Sheep;
        let mut zombie = Zombie::new(8, 0.0, 64.0, 0.0, 262);
        assert_eq!(
            zombie.mob_interact("shears", &mut Lcg::new(1)),
            MobInteract::Pass,
            "the zombie's table the kind's own empty"
        );
        assert_eq!(
            sheep.mob_interact("wheat", &mut Lcg::new(1)),
            MobInteract::Feed { love: true, grow: false },
            "the feeding lane the Animal super walks"
        );
        assert_eq!(
            sheep.mob_interact("dirt", &mut Lcg::new(1)),
            MobInteract::Pass,
            "the block item the gate refuses"
        );
        assert!(!sheep.sheared, "the PASS arms no wool bit");
    }

    #[test]
    fn the_food_tables_speak_the_tag_values() {
        // the jar's own `data/minecraft/tags/item/*.json` the table
        // keeps verbatim: SHEEP_FOOD = [wheat], COW_FOOD = [wheat],
        // PIG_FOOD = [carrot, potato, beetroot]; the zombie the
        // monster's chain refuses (Mob the Animal super never its
        // parent — the food gate closed outright)
        let mut sheep = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        sheep.kind = MobKind::Sheep;
        let mut cow = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        cow.kind = MobKind::Cow;
        let mut pig = Zombie::new(10, 0.0, 64.0, 0.0, 262);
        pig.kind = MobKind::Pig;
        let mut zombie = Zombie::new(8, 0.0, 64.0, 0.0, 262);
        assert!(sheep.is_food("wheat"), "SHEEP_FOOD the wheat speaks");
        assert!(
            !sheep.is_food("carrot"),
            "the pig's food the sheep's tag refuses"
        );
        assert!(cow.is_food("wheat"), "COW_FOOD the wheat speaks");
        assert!(!cow.is_food("carrot"), "COW_FOOD the carrot refuses");
        assert!(pig.is_food("carrot"), "PIG_FOOD the carrot speaks");
        assert!(pig.is_food("potato"), "PIG_FOOD the potato speaks");
        assert!(pig.is_food("beetroot"), "PIG_FOOD the beetroot speaks");
        assert!(
            !pig.is_food("wheat"),
            "the sheep's food the pig's tag refuses"
        );
        assert!(!zombie.is_food("wheat"), "the monster the gate refuses");
    }

    #[test]
    fn the_feed_walks_the_animal_law() {
        // Animal.mobInteract's own walk: the isFood hit, the ADULT
        // arm `getAge() == 0 && canFallInLove()` — the setInLove the
        // inLove = 600 arms, the Feed verdict the SUCCESS_SERVER
        // love word carries; the second press the canFallInLove
        // false adult keeps — the CONSUME the fallback returns (no
        // swing the SwingSource.NONE speaks, the clock unchanged);
        // the aiStep the decrement walks (600 → 599)
        let mut sheep = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        sheep.kind = MobKind::Sheep;
        assert_eq!(
            sheep.mob_interact("wheat", &mut Lcg::new(1)),
            MobInteract::Feed { love: true, grow: false },
            "the ADULT arm the SUCCESS_SERVER speaks"
        );
        assert_eq!(sheep.in_love, 600, "setInLove's own 600 the walk arms");
        assert_eq!(
            sheep.mob_interact("wheat", &mut Lcg::new(1)),
            MobInteract::Feed { love: false, grow: false },
            "the in-love adult the CONSUME fallback keeps"
        );
        assert_eq!(sheep.in_love, 600, "the CONSUME arm no clock change");
        // the aiStep's own decrement (the in_love > 0 arm walks)
        sheep.in_love = 1;
        let world = World::new(1, 42);
        sheep.tick(&world, None, NIGHT);
        assert_eq!(sheep.in_love, 0, "the decrement the aiStep speaks");
        assert!(
            sheep.mob_interact("wheat", &mut Lcg::new(1)).love(),
            "the gate reopened the zero clock keeps"
        );
    }

    #[test]
    fn the_feed_gate_refuses_the_wrong_arms() {
        // the food gate's own order: the shears the Sheep's own gate
        // speaks FIRST (the vanilla chain the mobInteract walk keeps
        // — the shear the food arm never reaches), the wrong stack
        // the tag refuses, the zombie the monster's chain refuses
        let mut sheep = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        sheep.kind = MobKind::Sheep;
        sheep.wool_color = 14;
        let mut zombie = Zombie::new(8, 0.0, 64.0, 0.0, 262);
        assert_eq!(
            sheep.mob_interact("shears", &mut Lcg::new(42)),
            MobInteract::Shear(vec![
                wire_id("red_wool").expect("red_wool"),
                wire_id("red_wool").expect("red_wool")
            ]),
            "the shears the chain's own first gate keeps"
        );
        assert_eq!(
            sheep.mob_interact("dirt", &mut Lcg::new(1)),
            MobInteract::Pass,
            "the wrong stack the tag refuses"
        );
        assert_eq!(
            zombie.mob_interact("wheat", &mut Lcg::new(1)),
            MobInteract::Pass,
            "the monster's chain the Animal super never reaches"
        );
        assert_eq!(zombie.in_love, 0, "the zombie no love clock arms");
    }

    #[test]
    fn the_age_walks_the_grow_up_and_the_cooldown() {
        // javap AgeableMob.aiStep: the baby's age climbs toward 0 (the
        // grow-up the zero crossing names), the adult's cooldown drains
        // toward 0 (the gate the feed arm reads)
        let mut child = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        child.age = BABY_START_AGE;
        assert!(child.is_baby(), "the negative age the baby names");
        let world = World::new(1, 42);
        for _ in 0..5 {
            child.tick(&world, None, NIGHT);
        }
        assert_eq!(
            child.age,
            BABY_START_AGE + 5,
            "the grow-up the aiStep's own walk speaks"
        );
        child.age = -1;
        child.tick(&world, None, NIGHT);
        assert_eq!(child.age, 0, "the zero crossing the adult names");
        assert!(!child.is_baby(), "the baby word the zero closes");
        child.age = BREED_COOLDOWN_TICKS;
        child.tick(&world, None, NIGHT);
        assert_eq!(
            child.age,
            BREED_COOLDOWN_TICKS - 1,
            "the cooldown the aiStep drains"
        );
    }

    #[test]
    fn the_nonzero_age_resets_the_love_word() {
        // javap Animal.customServerAiStep: `getAge() != 0 -> inLove = 0`
        // — the in-love state the breed cooldown's first tick strips,
        // the baby's the same walk keeps at zero
        let mut sheep = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        sheep.kind = MobKind::Sheep;
        sheep.age = BREED_COOLDOWN_TICKS;
        sheep.in_love = 600;
        let world = World::new(1, 42);
        sheep.tick(&world, None, NIGHT);
        assert_eq!(sheep.in_love, 0, "the cooldown the love word resets");
        // the zero-age adult's own clock the plain decrement walks
        sheep.age = 0;
        sheep.in_love = 600;
        sheep.tick(&world, None, NIGHT);
        assert_eq!(
            sheep.in_love, 599,
            "the adult's clock the plain decrement walks"
        );
    }

    #[test]
    fn the_feed_and_shear_gates_refuse_the_baby() {
        // javap Animal.mobInteract (the getAge() == 0 arm) +
        // Sheep.readyForShearing (!isSheared() && !isBaby()): the baby
        // the feed's love word never arms — the BABY arm the canAgeUp
        // gate takes instead (the SUCCESS verdict the grow word
        // carries), the coat the shear never takes
        let mut baby = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        baby.kind = MobKind::Sheep;
        baby.age = -1;
        assert_eq!(
            baby.mob_interact("wheat", &mut Lcg::new(1)),
            MobInteract::Feed { love: false, grow: true },
            "the baby the canAgeUp gate takes (the SUCCESS arm)"
        );
        assert_eq!(baby.in_love, 0, "the baby no love clock arms");
        // the ten-percent law's own floor: the 1/20 = 0.05 the f32
        // trunc rounds to ZERO seconds — the age the ceiling keeps,
        // the sparkle window the over-grow arm still marks
        assert_eq!(baby.age, -1, "the zero-amount ageUp the law keeps");
        assert_eq!(
            baby.forced_age_timer, FEED_FORCED_AGE_TIMER,
            "the window the over-grow arm marks"
        );
        assert!(
            !baby.ready_for_shearing(),
            "the baby's coat the shear never takes"
        );
    }

    /// THE FEEDING'S OWN TEN-PERCENT LAW (javap
    /// `Animal.mobInteract`'s canAgeUp arm + `AgeableMob.ageUp(IZ)` +
    /// `getSpeedUpSecondsWhenFeeding`): the wheat press the baby arm
    /// takes — the ageUp the `(-age / 20) * 0.1` seconds answers (24000
    /// / 20 = 1200, x 0.1 = 120, the * 20 refold = 2400 ticks — the
    /// ten percent of the remaining walk), the forcedAge the APPLIED
    /// delta -2400 carries, the 40-tick sparkle window the over-grow
    /// arm marks, the SUCCESS verdict the grow word carries, the love
    /// clock the setInLove arm the baby never rides.
    #[test]
    fn the_feeding_walks_the_ten_percent_law() {
        let mut child = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        child.kind = MobKind::Sheep;
        child.age = BABY_START_AGE;
        let verdict = child.mob_interact("wheat", &mut Lcg::new(1));
        assert_eq!(
            verdict,
            MobInteract::Feed { love: false, grow: true },
            "the baby arm the SUCCESS verdict speaks"
        );
        assert_eq!(
            child.age,
            BABY_START_AGE + 2400,
            "the ten-percent jump the ageUp's own fold answers"
        );
        assert!(child.is_baby(), "the baby the walk never crosses");
        assert_eq!(
            child.forced_age, 2400,
            "the applied delta the forcedAge accumulator carries (the
             new-minus-old fold the bytecode speaks: the growth POSITIVE)"
        );
        assert_eq!(
            child.forced_age_timer, FEED_FORCED_AGE_TIMER,
            "the window the over-grow arm marks"
        );
        assert_eq!(child.in_love, 0, "the baby no love clock arms");
        // the f32 trunc's own floor: the 1/20 = 0.05 the fold rounds
        // to zero seconds — the age the zero-amount ageUp keeps, the
        // window the arm still marks
        let mut edge = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        edge.kind = MobKind::Sheep;
        edge.age = -1;
        edge.mob_interact("wheat", &mut Lcg::new(1));
        assert_eq!(edge.age, -1, "the zero-amount ageUp the law keeps");
        assert_eq!(edge.forced_age, 0, "the zero delta the fold carries");
        assert_eq!(
            edge.forced_age_timer, FEED_FORCED_AGE_TIMER,
            "the window the zero-amount arm still marks"
        );
    }

    /// THE AGEUP'S OWN ZERO CEILING (javap `ageUp(IZ)` verbatim: the
    /// `if (i > 0) i = 0` fold the grow-up crossing never overshoots —
    /// the direct walk the overgrow amount answers — and the TAIL the
    /// lossless dump folds: `if (getAge() == 0) setAge(forcedAge);` —
    /// the ceiling's zero landing the BANKED SURPLUS spends, the
    /// positive age the adult keeps (the natural tick's own drain the
    /// cooldown walks back to 0)).
    #[test]
    fn the_ageup_never_crosses_the_zero_ceiling() {
        let mut child = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        child.age = BABY_START_AGE;
        // 1300 seconds the * 20 refold = 26000 ticks past the -24000 —
        // the ceiling the zero answers (the crossing the isBaby flip
        // closes), the applied delta the +24000 the fold carries — and
        // the TAIL the zero landing speaks: setAge(forcedAge) the
        // banked surplus spends, the adult the +24000 cooldown keeps
        // (the vanilla's own arithmetic the verbatim walk keeps)
        child.age_up(1300, true);
        assert_eq!(
            child.age, 24000,
            "the tail the banked surplus spends — the positive age \
             the zero landing keeps"
        );
        assert!(!child.is_baby(), "the baby word the zero closes");
        assert_eq!(
            child.forced_age, 24000,
            "the applied delta the verbatim fold carries"
        );
        // the natural tick the positive drain walks: the cooldown the
        // -1 step answers, the 1 -> 0 beat the boundary never arms
        let world = World::new(1, 42);
        child.tick(&world, None, NIGHT);
        assert_eq!(child.age, 23999, "the drain the natural tick walks");
        // the over-grow false arm: the window the mark never arms, the
        // accumulator the fold never touches — the zero landing the
        // EMPTY bank keeps (setAge(0) the same-sign set the no-op
        // answers)
        let mut dry = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        dry.age = -1000;
        dry.age_up(50, false);
        assert_eq!(dry.age, 0, "the ceiling the plain walk answers too");
        assert_eq!(dry.forced_age, 0, "the plain arm the accumulator keeps");
        assert_eq!(dry.forced_age_timer, 0, "the plain arm no window marks");
    }

    /// THE SETAGE'S OWN BOUNDARY BRANCH (javap `setAge(I)` the full
    /// disasm speaks: `old = getAge(); age = new; if ((old < 0) !=
    /// (new < 0)) { DATA_BABY_ID = new < 0; ageBoundaryReached(); }` —
    /// the sign flip the crossing arms, the same-sign set the branch
    /// never arms, the derived is_baby() the flag's own read keeps).
    #[test]
    fn the_setage_boundary_the_sign_flip_speaks() {
        let mut mob = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        mob.kind = MobKind::Sheep;
        mob.age = -100;
        // the same-sign sets the boundary never arms
        assert!(!mob.set_age(-50), "the still-baby set no crossing names");
        assert!(!mob.set_age(-1), "the walk's own -1 step no crossing arms");
        // the negative -> zero the grow-up crossing speaks
        assert!(mob.set_age(0), "the grow-up the sign flip arms");
        assert!(!mob.is_baby(), "the adult the zero names");
        // the zero -> zero and the zero -> positive the same-sign sets
        // the branch never arms (the cooldown the setAge(6000) walk)
        assert!(!mob.set_age(0), "the still-zero the boundary keeps");
        assert!(!mob.set_age(6000), "the cooldown the same-sign keeps");
        // the positive -> negative the re-babying the boundary arms —
        // the isBaby FIRST gate the ageBoundaryReached's own walk
        // rides (the still-baby the boat lane never hears)
        assert!(mob.set_age(BABY_START_AGE), "the re-babying the flip arms");
        assert!(mob.is_baby(), "the baby the -24000 names");
    }

    /// THE NATURAL TICK'S OWN CROSSING (javap `aiStep` the full disasm
    /// speaks: both arms route through setAge — the baby's `setAge(i
    /// + 1)` the -1 -> 0 beat the boundary branch arms, the adult's
    /// cooldown the same-sign sets never arm — the grow-up the
    /// is_baby flip the dims the adult box answers).
    #[test]
    fn the_natural_tick_crosses_the_boundary_the_adult_walks() {
        let mut mob = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        mob.kind = MobKind::Sheep;
        mob.age = -2;
        let world = World::new(1, 42);
        // the first tick: -2 -> -1 the still-baby the same-sign keeps
        mob.tick(&world, None, NIGHT);
        assert_eq!(mob.age, -1, "the climb the first tick walks");
        assert!(mob.is_baby(), "the baby the -1 keeps");
        // the second tick: -1 -> 0 the CROSSING the boundary arms (the
        // everyday grow-up the natural walk speaks), the adult the
        // zero names
        mob.tick(&world, None, NIGHT);
        assert_eq!(mob.age, 0, "the grow-up the crossing names");
        assert!(!mob.is_baby(), "the adult the zero keeps");
        // the dims the adult box answers (the baby's own half the
        // getDefaultDimensions arm keeps — the sheep 0.9x1.3 the
        // registration speaks, the baby the half-sized box)
        let (x0, y0, _z0, x1, y1, _z1) = mob.box_for();
        assert!(
            (x1 - x0 - 0.9).abs() < 1e-9 && (y1 - y0 - 1.3).abs() < 1e-9,
            "the sheep's own adult box the zero keeps: ({x1}-{x0}, {y1}-{y0})"
        );
    }

    /// THE TEN-PERCENT LAW'S OWN HONEST LIMIT (javap
    /// `getSpeedUpSecondsWhenFeeding` the +n/10 shave the remaining
    /// walk keeps — the feed the jump never crosses: `amount * 20 =
    /// 20 * floor(n/200) <= n/10 < n` — the baby the wheat never
    /// grows UP, the crossing the natural tick's own beat or the
    /// ceiling's big jump owes — the growth the shave still speaks,
    /// the age the climb answers).
    #[test]
    fn the_ten_percent_feed_never_crosses_the_zero() {
        let mut mob = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        mob.kind = MobKind::Sheep;
        mob.age = BABY_START_AGE;
        for i in 0..300 {
            let amount = speed_up_seconds_when_feeding(-mob.age);
            mob.age_up(amount, true);
            assert!(
                mob.is_baby(),
                "the feed the law never crosses (feed {i}, age {})" ,
                mob.age
            );
        }
        assert!(
            mob.age > BABY_START_AGE,
            "the growth the shave still speaks: {}",
            mob.age
        );
        // the stall the small-remainder fold keeps: the sub-200
        // remainder the floor rounds to zero seconds — the age the
        // further feeds never shave (the zero-amount edge the window
        // still marks)
        let stalled = mob.age;
        mob.age_up(speed_up_seconds_when_feeding(-mob.age), true);
        assert_eq!(
            mob.age, stalled,
            "the stall the zero-amount fold keeps"
        );
    }

    /// THE GOLDEN-DANDELION'S OWN TOGGLE (javap `setAgeLockedData` +
    /// the static `setAgeLocked` the full disasm speaks:
    /// `setAgeLocked(!isAgeLocked())` the toggle, `setAge(
    /// getBabyStartAge())` the UNCONDITIONAL re-baby BOTH directions
    /// walk, `ageLockParticleTimer = 40` the window the both arms
    /// mark — the lock the canAgeUp gate PAUSES, the unlock the fresh
    /// babyhood RESETS, the natural walk the unlock's own climb
    /// resumes).
    #[test]
    fn the_golden_dandelion_toggles_the_lock_and_rebabies() {
        let mut mob = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        mob.kind = MobKind::Sheep;
        mob.age = -1000;
        let mut rng = Lcg::new(7);
        // THE LOCK PRESS: the toggle arms, the re-baby the -24000
        // walks, the window the 40 marks
        assert_eq!(
            mob.mob_interact("golden_dandelion", &mut rng),
            MobInteract::AgeLock { locked: true },
            "the lock press the SUCCESS verdict speaks"
        );
        assert!(mob.age_locked, "the lock the toggle arms");
        assert_eq!(mob.age, BABY_START_AGE, "the re-baby the fresh walk arms");
        assert_eq!(
            mob.age_lock_particle_timer, AGE_LOCK_PARTICLE_WINDOW,
            "the window the toggle marks"
        );
        assert!(
            !mob.can_age_up(),
            "the lock the canAgeUp gate closes — the walk never grows"
        );
        // THE FROZEN WALK: the natural tick the locked baby keeps —
        // the age the -24000 the PAUSE answers, the even timer tick
        // the first spawn speaks
        let world = World::new(1, 42);
        mob.tick(&world, None, NIGHT);
        assert_eq!(
            mob.age, BABY_START_AGE,
            "the locked baby the natural walk never grows"
        );
        assert_eq!(
            mob.age_lock_particle_timer, AGE_LOCK_PARTICLE_WINDOW - 1,
            "the window the tick drains"
        );
        // THE RE-USE GATE: the window's own drain the canUse arm
        // requires — the press the mid-window keeps the PASS
        assert_eq!(
            mob.mob_interact("golden_dandelion", &mut rng),
            MobInteract::Pass,
            "the mid-window press the timer's nonzero refuses"
        );
        // the drain the window empties (the remaining 39 ticks)
        for _ in 1..AGE_LOCK_PARTICLE_WINDOW {
            mob.tick(&world, None, NIGHT);
        }
        assert_eq!(mob.age_lock_particle_timer, 0, "the window the drain empties");
        assert_eq!(mob.age, BABY_START_AGE, "the frozen walk the whole window keeps");
        // THE UNLOCK PRESS: the toggle walks back, the RESET the fresh
        // babyhood re-arms, the window the both arms mark
        assert_eq!(
            mob.mob_interact("golden_dandelion", &mut rng),
            MobInteract::AgeLock { locked: false },
            "the unlock press the post-toggle false carries"
        );
        assert!(!mob.age_locked, "the lock the toggle walks back");
        assert_eq!(
            mob.age, BABY_START_AGE,
            "the RESET the fresh babyhood BOTH directions walks"
        );
        assert_eq!(mob.age_lock_particle_timer, AGE_LOCK_PARTICLE_WINDOW, "the window the unlock marks too");
        assert!(mob.can_age_up(), "the unlock the natural walk resumes");
        // the resumed climb: the tick the -24000 -> -23999 beat walks
        mob.tick(&world, None, NIGHT);
        assert_eq!(
            mob.age,
            BABY_START_AGE + 1,
            "the resumed walk the unlocked baby climbs"
        );
    }

    /// THE DANDELION GATE'S OWN FOUR BRANCHES (javap
    /// `canUseGoldenDandelion` the full disasm speaks: the item's
    /// if_acmpne, the baby's ifeq, the timer's ifne, the in-tag's
    /// ifne — all the iconst_0 arm — AND the chain order the
    /// mobInteract super-calls keep: the shears, the food, the
    /// dandelion, the pass — the ZOMBIE the monster chain the
    /// AgeableMob arm never reaches).
    #[test]
    fn the_dandelion_gate_the_four_branches_arms() {
        let mut rng = Lcg::new(11);
        // THE BABY + THE DANDELION + the drained timer: the gate arms
        let mut baby = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        baby.kind = MobKind::Sheep;
        baby.age = -1600;
        assert!(
            baby.can_use_golden_dandelion("golden_dandelion"),
            "the baby the dandelion the drained window arms"
        );
        assert_eq!(
            baby.mob_interact("golden_dandelion", &mut rng),
            MobInteract::AgeLock { locked: true },
            "the press the lock verdict speaks"
        );
        // THE GROWN-UP the baby branch refuses (the ifeq the false
        // keeps) — the press the super's own PASS answers
        let mut adult = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        adult.kind = MobKind::Sheep;
        assert!(
            !adult.can_use_golden_dandelion("golden_dandelion"),
            "the grown-up the baby branch refuses"
        );
        assert_eq!(
            adult.mob_interact("golden_dandelion", &mut rng),
            MobInteract::Pass,
            "the adult press the chain's own pass answers"
        );
        // THE WRONG STACK the item branch refuses — the wheat the FOOD
        // arm's own order the chain keeps (the Animal super the
        // dandelion arm AHEAD walks)
        let mut baby2 = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        baby2.kind = MobKind::Sheep;
        baby2.age = -1600;
        assert!(
            !baby2.can_use_golden_dandelion("wheat"),
            "the wrong stack the item branch refuses"
        );
        assert!(
            matches!(
                baby2.mob_interact("wheat", &mut rng),
                MobInteract::Feed { .. }
            ),
            "the wheat press the FOOD arm's own order takes"
        );
        // THE ZOMBIE the monster chain the AgeableMob super never
        // reaches — the kind gate the chain's own walk speaks
        let mut zombie = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        zombie.age = -1600;
        assert!(
            !zombie.can_use_golden_dandelion("golden_dandelion"),
            "the monster the chain's own walk refuses"
        );
        assert_eq!(
            zombie.mob_interact("golden_dandelion", &mut rng),
            MobInteract::Pass,
            "the monster press the pass keeps"
        );
        // THE PIG the same zoo arms (the kind's own gate the tag json
        // never carries — the CANNOT_BE_AGE_LOCKED honest pass)
        let mut pig = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        pig.kind = MobKind::Pig;
        pig.age = -1600;
        assert!(
            pig.can_use_golden_dandelion("golden_dandelion"),
            "the pig the tag's honest pass arms"
        );
    }

    /// THE AGE-LOCK WINDOW'S OWN CADENCE (javap `makeAgeLockedParticle`
    /// the full disasm speaks: the timer's own gate `if (timer > 0)`,
    /// the spawn the even ticks keep (`timer % 2 == 0` the irem/ifne
    /// fold — twenty glint flashes the 40-tick window walks), the
    /// position the Vec3 ctor's own law carries — the x/z the unit
    /// triangle, the y `getRandomY(0.2) + getBbHeight() + lift` the
    /// lock's own word keeps — the decrement EVERY tick, the pick the
    /// lock state's own bool: PAUSE the locked speaks, RESET the
    /// unlock answers).
    #[test]
    fn the_age_lock_window_the_even_cadence_walks() {
        let mut mob = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        mob.kind = MobKind::Sheep;
        mob.age = BABY_START_AGE;
        mob.age_locked = true;
        mob.age_lock_particle_timer = AGE_LOCK_PARTICLE_WINDOW;
        let world = World::new(1, 42);
        let bb_h = {
            let (_, _, _, _, y1, _) = mob.box_for();
            y1 - mob.y
        };
        let mut flashes = 0;
        for _ in 0..44 {
            // the spawn walk the tick's head carries: the particle's
            // position the PRE-MOVE pose reads (the physics the tail
            // walks) — the pose the sparkle test's own capture keeps
            let (x0, y0, z0) = (mob.x, mob.y, mob.z);
            let ev = mob.tick(&world, None, NIGHT);
            if let Some((lx, ly, lz, locked)) = ev.age_lock_sparkle {
                flashes += 1;
                assert!(locked, "the locked walk the PAUSE pick speaks");
                assert!(
                    (lx - x0).abs() <= 1.0 && (lz - z0).abs() <= 1.0,
                    "the x/z the unit triangle keeps: ({lx}, {lz})"
                );
                // the y the bb-top + 0.2 lift the PAUSE arm keeps, the
                // ±0.2 triangle the getRandomY(0.2) draw speaks
                let y_lo = y0 + bb_h + 0.2 - 0.2;
                let y_hi = y0 + bb_h + 0.2 + 0.2;
                assert!(
                    (y_lo..y_hi).contains(&ly),
                    "the y the bb-top band keeps: {ly} in [{y_lo}, {y_hi})"
                );
            }
        }
        assert_eq!(
            flashes, 20,
            "the twenty flashes the even ticks the 40-window keeps"
        );
        assert_eq!(
            mob.age_lock_particle_timer, 0,
            "the window the drain empties"
        );
        // the empty tail the drained window keeps silent
        for _ in 0..10 {
            let ev = mob.tick(&world, None, NIGHT);
            assert!(
                ev.age_lock_sparkle.is_none(),
                "the drained window the walk never arms again"
            );
        }
        // THE UNLOCK'S OWN RESET WALK: the window the both arms mark,
        // the pick the lock state's own false speaks (the RESET the
        // fresh babyhood answers), the lift the 0.0 arm keeps
        mob.age_lock_particle_timer = AGE_LOCK_PARTICLE_WINDOW;
        mob.age_locked = false;
        let ev = mob.tick(&world, None, NIGHT);
        let Some((_, ly, _, locked)) = ev.age_lock_sparkle else {
            panic!("the re-armed window the first even tick speaks");
        };
        assert!(!locked, "the unlock walk the RESET pick speaks");
        let (_, _, _, _, y1, _) = mob.box_for();
        let adult_band = (mob.y + (y1 - mob.y) - 0.2..mob.y + (y1 - mob.y) + 0.2);
        assert!(
            adult_band.contains(&ly),
            "the RESET lift the 0.0 arm keeps: {ly}"
        );
    }

    /// THE FEED'S OWN SPARKLES (javap `AgeableMob.aiStep`'s client arm
    /// verbatim: the `% 4` cadence the HAPPY_VILLAGER spawn speaks, the
    /// position the `getRandomX(1.0) / getRandomY() + 0.5 /
    /// getRandomZ(1.0)` walk carries (x and z the `2d - 1` unit box, y
    /// the `d + 0.5` half-band), the spawn BEFORE the decrement — the
    /// ten flashes the 40-tick window keeps, the empty ticks the
    /// non-multiple answers).
    #[test]
    fn the_forced_age_timer_spawns_the_happy_villager_walk() {
        let mut child = Zombie::new(7, 0.0, 64.0, 0.0, 262);
        child.kind = MobKind::Sheep;
        child.age = BABY_START_AGE + 1600;
        child.forced_age_timer = FEED_FORCED_AGE_TIMER;
        let world = World::new(1, 42);
        let mut flashes = 0;
        for _ in 0..44 {
            // the spawn walk the tick's head carries: the sparkle's
            // position the PRE-MOVE pose reads (the physics the tail
            // walks), the band the spawn tick's own y keeps
            let y0 = child.y;
            let x0 = child.x;
            let z0 = child.z;
            let ev = child.tick(&world, None, NIGHT);
            if let Some((sx, sy, sz)) = ev.happy_sparkle {
                flashes += 1;
                assert!(
                    (sx - x0).abs() <= 1.0 && (sz - z0).abs() <= 1.0,
                    "the x/z the unit box keeps: ({sx}, {sz})"
                );
                assert!(
                    (y0 + 0.5..y0 + 1.5).contains(&sy),
                    "the y the half-band keeps: {sy} over {y0}"
                );
            }
        }
        assert_eq!(flashes, 10, "the ten flashes the 40-tick window keeps");
        assert_eq!(
            child.forced_age_timer, 0,
            "the window the drain empties"
        );
        // the empty tail: the drained window the walk never arms again
        for _ in 0..20 {
            let ev = child.tick(&world, None, NIGHT);
            assert!(
                ev.happy_sparkle.is_none(),
                "the drained window the walk keeps silent"
            );
        }
    }

    /// THE SPARKLE'S OWN PARTICLE (javap `SuspendedTownParticle` + the
    /// `$HappyVillagerProvider` verbatim: the lifetime the `20.0 /
    /// (nf * 0.8 + 0.2)` fold answers [20, 100], the size the `0.02 *
    /// (nf * 0.6 + 0.5)` draw answers [0.01, 0.022], the zero velocity
    /// the aiStep's own (0, 0, 0) the ctor's `* 0.02` fold keeps, the
    /// friction the tick's own `* 0.99` speaks, the gravity the law
    /// never arms, the white tint the provider's setColor(1,1,1)
    /// passes — the glint texture's own green the quad samples whole).
    #[test]
    fn the_sparkle_rides_the_suspended_town_laws() {
        for seed in 0..64u64 {
            let mut rng = Lcg::new(seed + 500);
            let p = crate::particles::happy_villager_sparkle(1.5, 5.6, 2.5, &mut rng);
            assert_eq!(p.x, 1.5, "the aiStep position the caller carried");
            assert_eq!(p.y, 5.6, "the aiStep position the caller carried");
            assert_eq!(p.z, 2.5, "the aiStep position the caller carried");
            assert_eq!(p.vx, 0.0, "the zero drift the aiStep's own (0,0,0) keeps");
            assert_eq!(p.vy, 0.0, "the zero drift the aiStep's own (0,0,0) keeps");
            assert_eq!(p.vz, 0.0, "the zero drift the aiStep's own (0,0,0) keeps");
            assert!(
                (20..=100).contains(&p.lifetime),
                "the lifetime the fold's own range keeps: {}",
                p.lifetime
            );
            assert!(
                (0.01..=0.022).contains(&p.quad_size),
                "the size the draw's own range keeps: {}",
                p.quad_size
            );
            assert_eq!(p.col, [1.0, 1.0, 1.0], "the white the provider passes");
            assert_eq!(p.friction, 0.99, "the tick's own friction fold");
            assert_eq!(p.gravity, 0.0, "the gravity the law never arms");
        }
    }

    #[test]
    fn the_breeding_walk_spawns_the_child_and_cools_the_pair() {
        // javap Animal.canMate (same class, both isInLove) +
        // spawnChildFromBreeding (the child at THIS parent's own spot,
        // the -24000 the birth arms) + finalizeSpawnChildFromBreeding
        // (the setAge(6000) both, the resetLove() both, the event-18
        // hearts, the nextInt(7) orb roll)
        let mut a = Zombie::new(1, 10.0, 64.0, 10.0, 262);
        a.kind = MobKind::Sheep;
        a.in_love = 600;
        let mut b = Zombie::new(2, 12.0, 64.0, 10.0, 263);
        b.kind = MobKind::Sheep;
        b.in_love = 600;
        // THE COURT the goal's own walk completed (the loveTime 60 the
        // BreedGoal's own gate reads — the approach the pair walked,
        // the meet the spawn's own tick answers)
        a.breed_love_time = BREED_COURT_TICKS;
        b.breed_love_time = BREED_COURT_TICKS;
        let mut mobs = vec![a, b];
        let mut next_id = 3;
        let spawns = try_breed(&mut mobs, &mut next_id, &mut Lcg::new(42));
        assert_eq!(spawns.len(), 1, "the pair the canMate law answers");
        let spawn = &spawns[0];
        assert_eq!(
            spawn.child.kind,
            MobKind::Sheep,
            "the child the same kind names"
        );
        assert_eq!(
            spawn.child.age, BABY_START_AGE,
            "getBabyStartAge the birth arms"
        );
        assert!(spawn.child.is_baby(), "the birth's baby word stands");
        assert_eq!(
            (spawn.child.x, spawn.child.y, spawn.child.z),
            (10.0, 64.0, 10.0),
            "the snapTo at THIS parent's own spot the walk speaks"
        );
        assert_eq!(spawn.child.id, 3, "the client's own counter the id rides");
        assert_eq!(next_id, 4, "the counter advanced one");
        assert!(
            (0..=6).contains(&spawn.orb.3),
            "the nextInt(7) band the orb roll keeps: {}",
            spawn.orb.3
        );
        assert_eq!(
            (spawn.orb.0, spawn.orb.1, spawn.orb.2),
            (10.0, 64.0, 10.0),
            "the orb at the parent's own spot the ctor rides"
        );
        // the parents the finalize walk cools (the in-place mutation
        // the scan speaks)
        assert_eq!(
            mobs[0].age, BREED_COOLDOWN_TICKS,
            "setAge(6000) the first parent"
        );
        assert_eq!(mobs[0].in_love, 0, "resetLove the first parent");
        assert_eq!(
            mobs[1].age, BREED_COOLDOWN_TICKS,
            "setAge(6000) the second parent"
        );
        assert_eq!(mobs[1].in_love, 0, "resetLove the second parent");
        // the hearts' bbox the spawning parent's own box names
        let expected = mobs[0].box_for();
        assert_eq!(
            spawn.heart_bbox, expected,
            "the event-18 ask the parent's own box"
        );
    }

    #[test]
    fn the_dye_pair_mix_table_speaks_the_nine_recipes() {
        // javap DyeColor.getMixedColor's crafting walk: every pair the
        // table keeps the recipe result speaks — both orders (the
        // shapeless grid the walk answers), the same-dye pair the
        // recipe-less pass the identical wool keeps
        for ((a, b), out) in DYE_MIX_TABLE {
            assert_eq!(
                mixed_wool_color(a, b, &mut Lcg::new(1)),
                out,
                "the recipe the ({a},{b}) pair answers"
            );
            assert_eq!(
                mixed_wool_color(b, a, &mut Lcg::new(1)),
                out,
                "the shapeless grid the reversed pair answers"
            );
        }
        // the identical pair: no recipe — the fallback coin picks one
        // parent's dye, the identical wool the same value speaks
        assert_eq!(mixed_wool_color(0, 0, &mut Lcg::new(1)), 0);
        assert_eq!(mixed_wool_color(12, 12, &mut Lcg::new(1)), 12);
    }

    #[test]
    fn the_breeding_mix_walks_the_recipe_arm() {
        // the red + blue pair the purple child speaks — the coin draw
        // the mix arm never consumes (the stream the xp roll rides
        // keeps its two-draw span: the child's seed, the orb roll)
        let mut a = Zombie::new(1, 10.0, 64.0, 10.0, 262);
        a.kind = MobKind::Sheep;
        a.in_love = 600;
        a.wool_color = 14; // RED
        let mut b = Zombie::new(2, 12.0, 64.0, 10.0, 263);
        b.kind = MobKind::Sheep;
        b.in_love = 600;
        b.wool_color = 11; // BLUE
                           // THE COURT the goal's own walk completed (the loveTime 60 the
                           // BreedGoal's own gate reads — the approach the pair walked,
                           // the meet the spawn's own tick answers)
        a.breed_love_time = BREED_COURT_TICKS;
        b.breed_love_time = BREED_COURT_TICKS;
        let mut mobs = vec![a, b];
        let mut next_id = 3;
        let spawns = try_breed(&mut mobs, &mut next_id, &mut Lcg::new(42));
        assert_eq!(spawns.len(), 1, "the pair the canMate law answers");
        assert_eq!(
            spawns[0].child.wool_color, 10,
            "the purple the crafting recipe speaks"
        );
        // the parents' own coats the walk never touches
        assert_eq!(mobs[0].wool_color, 14, "the first parent's dye stands");
        assert_eq!(mobs[1].wool_color, 11, "the second parent's dye stands");
        // the stream's own span: the child's seed then the orb roll —
        // no coin draw between (the probe the same seed replays)
        let mut probe = Lcg::new(42);
        let _child_seed = probe.next();
        let expected_xp = (probe.next() % 7) as i32;
        assert_eq!(
            spawns[0].orb.3, expected_xp,
            "the two-draw span the mix arm keeps"
        );
    }

    #[test]
    fn the_recipeless_pair_keeps_the_coin_fallback() {
        // green + brown: no recipe the crafting grid speaks — the
        // nextBoolean coin one parent's dye picks (the fallback arm
        // the vanilla keeps), the stream the three-draw span rides
        // (the child's seed, the coin, the orb roll)
        let mut a = Zombie::new(1, 10.0, 64.0, 10.0, 262);
        a.kind = MobKind::Sheep;
        a.in_love = 600;
        a.wool_color = 13; // GREEN
        let mut b = Zombie::new(2, 12.0, 64.0, 10.0, 263);
        b.kind = MobKind::Sheep;
        b.in_love = 600;
        b.wool_color = 12; // BROWN
                           // THE COURT the goal's own walk completed (the loveTime 60 the
                           // BreedGoal's own gate reads — the approach the pair walked,
                           // the meet the spawn's own tick answers)
        a.breed_love_time = BREED_COURT_TICKS;
        b.breed_love_time = BREED_COURT_TICKS;
        let mut mobs = vec![a, b];
        let mut next_id = 3;
        let spawns = try_breed(&mut mobs, &mut next_id, &mut Lcg::new(42));
        assert_eq!(spawns.len(), 1, "the pair the canMate law answers");
        let coat = spawns[0].child.wool_color;
        assert!(
            coat == 13 || coat == 12,
            "the fallback one parent's dye picks: {coat}"
        );
        // the stream's own span: the coin the fallback consumed (the
        // probe the same seed replays — the third draw the orb rolls)
        let mut probe = Lcg::new(42);
        let _child_seed = probe.next();
        let _coin = probe.next();
        let expected_xp = (probe.next() % 7) as i32;
        assert_eq!(
            spawns[0].orb.3, expected_xp,
            "the three-draw span the fallback keeps"
        );
    }

    #[test]
    fn the_follow_parent_scan_arms_the_babys_nearest_adult() {
        // javap FollowParentGoal.canUse: the baby the age word speaks,
        // the same-kind adults the inflate(8,4,8) box keeps, the
        // nearest the d1>d0 skip answers, the armed tuple the
        // steering's snapshot keeps
        let mut baby = Zombie::new(1, 0.0, 5.0, 0.0, 262);
        baby.kind = MobKind::Sheep;
        baby.age = BABY_START_AGE;
        baby.on_ground = true;
        let mut near = Zombie::new(2, 5.0, 5.0, 0.0, 263);
        near.kind = MobKind::Sheep;
        let mut far = Zombie::new(3, 8.0, 5.0, 0.0, 264);
        far.kind = MobKind::Sheep;
        let mut mobs = vec![baby, near, far];
        let armed = arm_follow_parents(&mut mobs);
        assert_eq!(armed, 1, "the one baby the scan arms");
        let (id, x, y, z) = mobs[0]
            .follow_parent
            .expect("the armed tuple the nearest adult speaks");
        assert_eq!(id, 2, "the nearest the d1>d0 skip answers");
        assert_eq!(
            (x, y, z),
            (5.0, 5.0, 0.0),
            "the snapshot the steering walks"
        );
        assert!(
            mobs[1].follow_parent.is_none(),
            "the adult the baby word refuses"
        );
        assert!(
            mobs[2].follow_parent.is_none(),
            "the second adult the same law refuses"
        );
    }

    /// THE DIRECTION DRAW'S OWN AXES (javap
    /// `RandomPos.generateRandomDirection(RandomSource, II)` the
    /// bytecode pins): the x and z components both draw
    /// `nextInt(2*x+1) - x` (the SECOND ±x draw the istore loc5
    /// speaks), the y component `nextInt(2*z+1) - z` — the second
    /// arg the VERTICAL spread carries. The full range each axis
    /// walks over the seeds the spread's own bounds keep.
    #[test]
    fn the_direction_draw_splits_the_axes() {
        for seed in 0..64u64 {
            let mut z = Zombie::new(1, 0.0, 5.0, 0.0, seed + 700);
            let (dx, dy, dz) = z.generate_random_direction(STROLL_LAND_RING, STROLL_LAND_SPREAD);
            assert!(
                (-STROLL_LAND_RING..=STROLL_LAND_RING).contains(&dx),
                "the x draw the horizontal ring keeps: {dx}"
            );
            assert!(
                (-STROLL_LAND_SPREAD..=STROLL_LAND_SPREAD).contains(&dy),
                "the y draw the vertical spread keeps: {dy}"
            );
            assert!(
                (-STROLL_LAND_RING..=STROLL_LAND_RING).contains(&dz),
                "the z draw the second horizontal ring keeps: {dz}"
            );
        }
        // the FULL range each axis answers over the seed walk — the
        // bounds the modulo fold never clips silent
        let mut seen_x = (i64::MAX, i64::MIN);
        let mut seen_y = (i64::MAX, i64::MIN);
        for seed in 0..512u64 {
            let mut z = Zombie::new(1, 0.0, 5.0, 0.0, seed + 700);
            let (dx, dy, _) = z.generate_random_direction(STROLL_LAND_RING, STROLL_LAND_SPREAD);
            seen_x.0 = seen_x.0.min(dx);
            seen_x.1 = seen_x.1.max(dx);
            seen_y.0 = seen_y.0.min(dy);
            seen_y.1 = seen_y.1.max(dy);
        }
        assert_eq!(
            seen_x,
            (-10, 10),
            "the horizontal ring the full walk speaks"
        );
        assert_eq!(seen_y, (-7, 7), "the vertical spread the full walk speaks");
    }

    /// THE LAND STROLL NEVER TARGETS THE WATER (javap
    /// `LandRandomPos.getPos` + `GoalUtils.isWater`): the lake
    /// columns the isWater word keeps (the lift the pool surface
    /// walks into, the floating cell the stable-destination ground
    /// refuses), every accepted target the dry column's own top
    /// answers; the OCEAN the ten tries never answers — the vanilla
    /// null the canUse gate answers with no stroll.
    #[test]
    fn the_land_stroll_never_targets_the_water() {
        let column_top = |world: &World, tx: i32, tz: i32| -> u16 {
            for y in (0..crate::world::CHUNK_Y as i32).rev() {
                let id = world.get(tx, y, tz);
                if id != crate::world::AIR {
                    return id;
                }
            }
            crate::world::AIR
        };
        // a stone plain with a 9x9 lake carved at (12..21): the mob
        // on the dry shore, the ring 10 both worlds reach
        let mut world = wide_flat_world();
        for x in 12..21 {
            for z in 12..21 {
                world.set(x, 5, z, crate::world::WATER);
            }
        }
        for seed in 0..64u64 {
            let mut s = Zombie::new(1, 8.5, 5.0, 8.5, seed + 800);
            if let Some((tx, tz)) = s.land_random_pos(&world, STROLL_LAND_RING, STROLL_LAND_SPREAD)
            {
                let (itx, itz) = (tx.floor() as i32, tz.floor() as i32);
                assert_ne!(
                    column_top(&world, itx, itz),
                    crate::world::WATER,
                    "the seed {} the water column the land pick refused",
                    seed
                );
            }
        }
        // the OCEAN: every column water — the ten tries the null
        // keeps (the isWater + the floating-ground words answer all)
        let mut ocean = wide_flat_world();
        for x in 0..ocean.size_blocks() {
            for z in 0..ocean.size_blocks() {
                ocean.set(x, 5, z, crate::world::WATER);
            }
        }
        let mut s = Zombie::new(1, 8.5, 5.0, 8.5, 42);
        assert!(
            s.land_random_pos(&ocean, STROLL_LAND_RING, STROLL_LAND_SPREAD)
                .is_none(),
            "the ocean the ten tries never answers"
        );
    }

    /// THE TEN-TRY STREAM THE FLOW PINS (javap
    /// `RandomPos.generateRandomPos`'s bipush-10 loop): the supplier
    /// the loop calls ALL TEN times — the land pick's null the ocean
    /// answers still advances the mob's stream exactly thirty draws
    /// (three per try), the super pick the same thirty — the early
    /// hit the loop never short-circuits.
    #[test]
    fn the_stroll_stream_draws_the_ten_tries() {
        let mut ocean = wide_flat_world();
        for x in 0..ocean.size_blocks() {
            for z in 0..ocean.size_blocks() {
                ocean.set(x, 5, z, crate::world::WATER);
            }
        }
        let mut s = Zombie::new(1, 8.5, 5.0, 8.5, 42);
        let _ = s.land_random_pos(&ocean, STROLL_LAND_RING, STROLL_LAND_SPREAD);
        // the reference stream advanced the same thirty raw draws
        let mut reference = Lcg::new(42);
        for _ in 0..30 {
            reference.next();
        }
        assert_eq!(
            s.rng.next(),
            reference.next(),
            "the land pick's thirty draws the ocean null never skips"
        );
        // the super pick: the same thirty (the first candidate the
        // score-0 fold keeps, the nine the loop still draws)
        let mut s2 = Zombie::new(1, 8.5, 5.0, 8.5, 42);
        let _ = s2.default_random_pos(STROLL_LAND_RING, STROLL_LAND_SPREAD);
        let mut reference2 = Lcg::new(42);
        for _ in 0..30 {
            reference2.next();
        }
        assert_eq!(
            s2.rng.next(),
            reference2.next(),
            "the super pick's thirty draws the first-kept candidate never clips"
        );
    }

    /// THE IN-WATER STROLL FLEES TO THE LAND (the tick's own
    /// in-water arm): the pooled body the 15/7 ring rides, the
    /// stroll target the tick sets lands on a DRY column within the
    /// ring — the pool the isWater word keeps, the flee the walk
    /// answers.
    #[test]
    fn the_in_water_stroll_flees_to_the_land() {
        let mut world = wide_flat_world();
        // a 9x9 pool centered at (24, 24): the surface water at y 5,
        // the dry plain all around — the 15 ring reaches land
        for x in 20..29 {
            for z in 20..29 {
                world.set(x, 5, z, crate::world::WATER);
            }
        }
        // the brute-forced seed whose stroll gate arms on tick 1 (the
        // interval gate the 120 fold speaks — the first stream draw
        // the tick's own order keeps)
        let mut found: Option<(u64, (f64, f64))> = None;
        for seed in 0..2000u64 {
            let mut z = Zombie::new(1, 24.5, 5.0, 24.5, seed);
            z.tick(&world, None, NOON);
            if let Some(t) = z.stroll_target {
                found = Some((seed, t));
                break;
            }
        }
        let (seed, (tx, tz)) = found.expect("the seed walk the stroll gate arms within");
        let mut z = Zombie::new(1, 24.5, 5.0, 24.5, seed);
        assert!(z.in_water(&world), "the pooled body the arm rides");
        let (itx, itz) = (tx.floor() as i32, tz.floor() as i32);
        // the dry column the land ring keeps (the pool the gate
        // refuses), the ring bound the 15/7 call speaks
        let mut top = crate::world::AIR;
        for y in (0..crate::world::CHUNK_Y as i32).rev() {
            let id = world.get(itx, y, itz);
            if id != crate::world::AIR {
                top = id;
                break;
            }
        }
        assert_ne!(top, crate::world::WATER, "the flee the dry column answers");
        assert!(
            (itx as f64 - 24.5).abs() <= 15.0 && (itz as f64 - 24.5).abs() <= 15.0,
            "the ring bound the 15/7 call keeps"
        );
    }

    #[test]
    fn the_follow_scan_refuses_the_wrong_rings() {
        // the 9.0 close ring the start refuses (the 3.0 the squared
        // word speaks), the boundary the >= law arms, the cross-kind
        // the class law refuses, the baby sibling the adult word
        // refuses, the dying parent the isAlive word refuses, the far
        // adult the box law refuses
        let baby = || {
            let mut b = Zombie::new(1, 0.0, 5.0, 0.0, 262);
            b.kind = MobKind::Sheep;
            b.age = BABY_START_AGE;
            b.on_ground = true;
            b
        };

        // the adult INSIDE the close ring (distSq 6.25 < 9.0)
        let mut close = Zombie::new(2, 2.5, 5.0, 0.0, 263);
        close.kind = MobKind::Sheep;
        let mut mobs = vec![baby(), close];
        assert_eq!(
            arm_follow_parents(&mut mobs),
            0,
            "the close ring the start refuses"
        );
        assert!(mobs[0].follow_parent.is_none(), "the disarmed None keeps");

        // the boundary: exactly 3.0 out — the `d0 < 9.0 -> false` the
        // >= arm answers
        let mut edge = Zombie::new(2, 3.0, 5.0, 0.0, 263);
        edge.kind = MobKind::Sheep;
        let mut mobs = vec![baby(), edge];
        assert_eq!(
            arm_follow_parents(&mut mobs),
            1,
            "the boundary the >= law arms"
        );

        // the cross-kind adult the class law refuses
        let mut cow = Zombie::new(2, 5.0, 5.0, 0.0, 263);
        cow.kind = MobKind::Cow;
        let mut mobs = vec![baby(), cow];
        assert_eq!(
            arm_follow_parents(&mut mobs),
            0,
            "the cross-kind the class law refuses"
        );

        // the baby sibling the adult word refuses
        let mut sibling = Zombie::new(2, 5.0, 5.0, 0.0, 263);
        sibling.kind = MobKind::Sheep;
        sibling.age = BABY_START_AGE;
        let mut mobs = vec![baby(), sibling];
        assert_eq!(
            arm_follow_parents(&mut mobs),
            0,
            "the baby sibling the adult word refuses"
        );

        // the dying parent the isAlive word refuses
        let mut dying = Zombie::new(2, 5.0, 5.0, 0.0, 263);
        dying.kind = MobKind::Sheep;
        dying.health = 0.0;
        let mut mobs = vec![baby(), dying];
        assert_eq!(
            arm_follow_parents(&mut mobs),
            0,
            "the dying parent the isAlive word refuses"
        );

        // the far adult the box law refuses (the inflate ring the
        // bbox-intersects speaks)
        let mut far = Zombie::new(2, 20.0, 5.0, 0.0, 263);
        far.kind = MobKind::Sheep;
        let mut mobs = vec![baby(), far];
        assert_eq!(
            arm_follow_parents(&mut mobs),
            0,
            "the far adult the box law refuses"
        );
    }

    #[test]
    fn the_babys_own_walk_folds_the_parent_steering() {
        // the follow tick: the yaw faces the parent, the impulse rides
        // the 1.1 the goal's ctor speaks (the stroll control the same
        // walk at the plain attribute keeps — the displacement ratio
        // the modifier pins), the stroll's target the higher priority
        // replaces
        let world = wide_flat_world();
        let mut follow = Zombie::new(1, 8.0, 5.0, 8.0, 262);
        follow.kind = MobKind::Sheep;
        follow.age = BABY_START_AGE;
        follow.on_ground = true;
        follow.follow_parent = Some((2, 13.0, 5.0, 8.0)); // 5 east
        follow.stroll_target = Some((-2.0, 8.0));
        let ev = follow.tick(&world, None, NOON);
        assert!(ev.attack.is_none(), "the follow no attack speaks");
        assert!(
            (follow.yaw - (-90.0)).abs() < 1e-6,
            "the yaw the parent's east speaks ({})",
            follow.yaw
        );
        assert!(
            follow.stroll_target.is_none(),
            "the stroll's target the higher priority replaces"
        );
        let follow_dx = follow.x - 8.0;
        assert!(follow_dx > 0.0, "the walk the parent's east speaks");

        // the control: the same 5-east walk the stroll's own impulse
        // rides — the ratio the 1.1 the moveTo's speedModifier pins
        let mut stroll = Zombie::new(1, 8.0, 5.0, 8.0, 262);
        stroll.kind = MobKind::Sheep;
        stroll.age = BABY_START_AGE;
        stroll.on_ground = true;
        stroll.stroll_target = Some((13.0, 8.0));
        stroll.tick(&world, None, NOON);
        let stroll_dx = stroll.x - 8.0;
        assert!(stroll_dx > 0.0, "the stroll's own walk the control speaks");
        let ratio = follow_dx / stroll_dx;
        assert!(
            (ratio - FOLLOW_PARENT_SPEED).abs() < 1e-9,
            "the 1.1 the modifier speaks ({ratio})"
        );
    }

    #[test]
    fn the_breed_scan_arms_the_love_pairs_nearest_mate() {
        // getFreePartner's own law: the in-love adult the nearest
        // same-kind in-love mate the inflate(8.0) box keeps — the
        // mutual pair the symmetric arms answer, the love-less
        // neighbour the canMate gate refuses, the far in-love sheep
        // the box law refuses
        let mut a = Zombie::new(1, 10.0, 5.0, 10.0, 262);
        a.kind = MobKind::Sheep;
        a.in_love = 600;
        let mut b = Zombie::new(2, 14.0, 5.0, 10.0, 263); // 4 east — inside the ring
        b.kind = MobKind::Sheep;
        b.in_love = 600;
        let mut stranger = Zombie::new(3, 11.0, 5.0, 10.0, 264); // nearer, never fed
        stranger.kind = MobKind::Sheep;
        let mut far = Zombie::new(4, 40.0, 5.0, 10.0, 265); // in love, beyond the box
        far.kind = MobKind::Sheep;
        far.in_love = 600;
        let mut mobs = vec![a, b, stranger, far];
        assert_eq!(
            arm_breeders(&mut mobs),
            2,
            "the mutual pair the scan arms, the ring-less two the gates refuse"
        );
        assert_eq!(
            mobs[0].breed_partner,
            Some((2, 14.0, 5.0, 10.0)),
            "the first's mate the nearest in-love adult speaks"
        );
        assert_eq!(
            mobs[1].breed_partner,
            Some((1, 10.0, 5.0, 10.0)),
            "the second's mate the mirror answers"
        );
        assert_eq!(
            mobs[2].breed_partner, None,
            "the love-less neighbour the canMate gate refuses"
        );
        assert_eq!(
            mobs[3].breed_partner, None,
            "the far sheep the inflate box refuses"
        );
        // the court clocks the arm ride keeps (the goal's own
        // lifetime — the re-arm the per-frame re-scan answers never
        // restarts the count)
        mobs[0].breed_love_time = 17;
        arm_breeders(&mut mobs);
        assert_eq!(
            mobs[0].breed_love_time, 17,
            "the re-arm the court's own count keeps"
        );
    }

    #[test]
    fn the_breed_scan_stops_the_broken_courts() {
        // the canContinueToUse + stop law the per-frame scan answers:
        // the dying mate the isAlive word refuses, the cooled breeder
        // the love word refuses — the stop the partner nulls and the
        // court clock zeroes (the re-court the fresh arm restarts)
        let mut a = Zombie::new(1, 10.0, 5.0, 10.0, 262);
        a.kind = MobKind::Sheep;
        a.in_love = 600;
        a.breed_love_time = 30; // mid-court
        a.breed_partner = Some((2, 14.0, 5.0, 10.0));
        let mut dying = Zombie::new(2, 14.0, 5.0, 10.0, 263);
        dying.kind = MobKind::Sheep;
        dying.in_love = 600;
        dying.health = 0.0; // the fire's own kill the clock spoke
        let mut mobs = vec![a, dying];
        assert_eq!(
            arm_breeders(&mut mobs),
            0,
            "the dying mate the isAlive word refuses"
        );
        assert_eq!(
            mobs[0].breed_partner, None,
            "the broken court the stop law disarms"
        );
        assert_eq!(
            mobs[0].breed_love_time, 0,
            "the stop's own zero the clock keeps"
        );

        // the cooled breeder (resetLove spoke — the bred pair's own
        // tail): the love word's own gate the scan refuses — and the
        // in-love mate whose only candidate cooled the partner-less
        // canUse refuses too (the stop law the disarmed None keeps)
        let mut cooled = Zombie::new(1, 10.0, 5.0, 10.0, 262);
        cooled.kind = MobKind::Sheep;
        cooled.in_love = 0;
        cooled.breed_love_time = 45; // the stale count the stop zeroes
        let mut mate = Zombie::new(2, 12.0, 5.0, 10.0, 263);
        mate.kind = MobKind::Sheep;
        mate.in_love = 600;
        let mut mobs = vec![cooled, mate];
        assert_eq!(
            arm_breeders(&mut mobs),
            0,
            "the cooled pair the love word's own gate refuses"
        );
        assert_eq!(
            mobs[0].breed_partner, None,
            "the cooled breeder the love word disarms"
        );
        assert_eq!(
            mobs[0].breed_love_time, 0,
            "the stale count the stop zeroes"
        );
        assert_eq!(
            mobs[1].breed_partner, None,
            "the mate-less canUse the stop law answers"
        );
    }

    #[test]
    fn the_breeders_own_walk_folds_the_partner_steering() {
        // the breed tick: the yaw faces the partner, the impulse rides
        // the 1.0 the goal's ctor speaks (the stroll control the same
        // attribute keeps — the displacement ratio the modifier pins),
        // the stroll's target and the follow's own stale arm the
        // higher priority replaces, the court clock the loveTime++
        // counts
        let world = wide_flat_world();
        let mut breed = Zombie::new(1, 8.0, 5.0, 8.0, 262);
        breed.kind = MobKind::Sheep;
        breed.in_love = 600;
        breed.on_ground = true;
        breed.breed_partner = Some((2, 13.0, 5.0, 8.0)); // 5 east
        breed.stroll_target = Some((-2.0, 8.0));
        breed.follow_parent = Some((3, 0.0, 5.0, 8.0)); // the stale arm the prio owns
        let ev = breed.tick(&world, None, NOON);
        assert!(ev.attack.is_none(), "the breed no attack speaks");
        assert!(
            (breed.yaw - (-90.0)).abs() < 1e-6,
            "the yaw the partner's east speaks ({})",
            breed.yaw
        );
        assert!(
            breed.stroll_target.is_none(),
            "the stroll's target the higher priority replaces"
        );
        assert!(
            breed.follow_parent.is_none(),
            "the follow's stale arm the priority replaces"
        );
        assert_eq!(
            breed.breed_love_time, 1,
            "the court clock the loveTime++ counts"
        );
        let breed_dx = breed.x - 8.0;
        assert!(breed_dx > 0.0, "the walk the partner's east speaks");

        // the control: the same 5-east walk the stroll's own impulse
        // rides — the ratio the 1.0 the moveTo's speedModifier pins
        let mut stroll = Zombie::new(1, 8.0, 5.0, 8.0, 262);
        stroll.kind = MobKind::Sheep;
        stroll.in_love = 600;
        stroll.on_ground = true;
        stroll.stroll_target = Some((13.0, 8.0));
        stroll.tick(&world, None, NOON);
        let stroll_dx = stroll.x - 8.0;
        assert!(stroll_dx > 0.0, "the stroll's own walk the control speaks");
        let ratio = breed_dx / stroll_dx;
        assert!(
            (ratio - BREED_SPEED).abs() < 1e-9,
            "the 1.0 the modifier speaks ({ratio})"
        );

        // THE 60-MISS the canContinueToUse stop law answers: the court
        // expired beyond the close ring — the clock zeroes (the
        // re-court the fresh arm restarts), the walk keeps
        let mut late = Zombie::new(1, 8.0, 5.0, 8.0, 262);
        late.kind = MobKind::Sheep;
        late.in_love = 600;
        late.on_ground = true;
        late.breed_partner = Some((2, 20.0, 5.0, 8.0)); // 12 east — beyond the 3.0 ring
        late.breed_love_time = 59;
        late.tick(&world, None, NOON);
        assert_eq!(late.breed_love_time, 0, "the 60-miss the stop law resets");

        // THE MET COURT: the 60 lands inside the close ring — the
        // clock keeps (the gate the caller's post-tick scan reads)
        let mut met = Zombie::new(1, 8.0, 5.0, 8.0, 262);
        met.kind = MobKind::Sheep;
        met.in_love = 600;
        met.on_ground = true;
        met.breed_partner = Some((2, 10.0, 5.0, 8.0)); // 2 east — inside the ring
        met.breed_love_time = 59;
        met.tick(&world, None, NOON);
        assert_eq!(
            met.breed_love_time, 60,
            "the met court the gate's own count keeps"
        );
    }

    /// THE ANIMALS' OWN PEACE (the registerGoals dumps' own verdict):
    /// the targetSelector the zombie's registerGoals fills
    /// (NearestAttackableTargetGoal the prio-2 walk speaks) the
    /// animals' registerGoals never fills — the scan the monster
    /// kinds gate: the melee-ring player the sheep/cow/pig never
    /// targets, never charges, never hits (the aggressive arms the
    /// monster's own word keeps lowered), the zombie the same
    /// staging the chase's own yaw answers.
    #[test]
    fn the_animals_own_peace_gates_the_target_scan() {
        let world = flat_world();
        // the is_monster table (the class chain the dumps keep)
        assert!(MobKind::Zombie.is_monster());
        assert!(!MobKind::Sheep.is_monster());
        assert!(!MobKind::Cow.is_monster());
        assert!(!MobKind::Pig.is_monster());

        // the melee-ring player the monster's scan answers — the
        // animals the same staging the peace keeps over the whole
        // walk (no hit, no raised arms; the stroll/look rest the
        // goal-else chain keeps, the 120-gate's own rolls ride)
        for kind in [MobKind::Sheep, MobKind::Cow, MobKind::Pig] {
            let mut animal = Zombie::new(1, 8.0, 5.0, 8.0, 262);
            animal.kind = kind;
            animal.on_ground = true;
            let player = player_at(5.0, 5.0, 8.0); // 3 west — the chase test's own ring
            for tick in 0..200 {
                let ev = animal.tick(&world, Some(&player), NIGHT);
                assert!(
                    ev.attack.is_none(),
                    "the {kind:?} the player never hits (tick {tick})"
                );
                assert!(
                    !animal.aggressive && animal.raise_arm_ticks == 0,
                    "the {kind:?} the monster's arms never raises (tick {tick})"
                );
            }
        }

        // the control: the same staging the zombie's own scan answers —
        // the body yaw swings at the player the first tick speaks
        let mut z = Zombie::new(1, 8.0, 5.0, 8.0, 7);
        z.on_ground = true;
        let player = player_at(5.0, 5.0, 8.0);
        z.tick(&world, Some(&player), NIGHT);
        assert!(
            (z.yaw - 90.0).abs() < 1e-6,
            "the zombie's scan the player targets (yaw {})",
            z.yaw
        );
    }

    /// THE NOON BRAND THE ANIMALS NEVER WEARS (shouldBurnInDay's own
    /// override walk): the Mob base speaks false, the undead classes'
    /// override the true the sun's brand rides — the sheep/cow/pig
    /// the same noon surface the brand never lands (the kind word the
    /// short-circuit's first gate keeps: the roll's draw never fires,
    /// the stream the goal chain's own shape keeps). The zombie's own
    /// brand the noon test pins beside.
    #[test]
    fn the_noon_brand_the_animals_never_wears() {
        let world = wide_flat_world();
        for kind in [MobKind::Sheep, MobKind::Cow, MobKind::Pig] {
            let mut animal = Zombie::new(1, 32.0, 5.0, 32.0, 262);
            animal.kind = kind;
            animal.on_ground = true;
            for _ in 0..600 {
                animal.tick(&world, None, NOON);
                assert_eq!(animal.fire_ticks, 0, "the {kind:?} the noon never brands");
            }
            assert_eq!(
                animal.health, ZOMBIE_MAX_HEALTH,
                "the {kind:?} the fire hurt never speaks"
            );
        }
    }

    /// THE FED SHEEP'S COURT THE PEACE KEEPS (the staging lane's own
    /// law the kind gate speaks): the armed breed walk the in-range
    /// player never steals — the goal-else chain the breed arm's prio
    /// owns, the body yaw the partner's east speaks (the pre-fix scan
    /// the player's west would have charged), the court's own clock
    /// the loveTime++ counts, the arms the peace keeps lowered.
    #[test]
    fn the_fed_sheeps_court_survives_the_in_range_player() {
        let world = wide_flat_world();
        let mut sheep = Zombie::new(1, 8.0, 5.0, 8.0, 262);
        sheep.kind = MobKind::Sheep;
        sheep.in_love = 600;
        sheep.on_ground = true;
        sheep.breed_partner = Some((2, 13.0, 5.0, 8.0)); // 5 east
        let player = player_at(3.0, 5.0, 8.0); // 5 west — the old scan would charge
        let ev = sheep.tick(&world, Some(&player), NOON);
        assert!(ev.attack.is_none(), "the peace the hit refuses");
        assert!(
            (sheep.yaw - (-90.0)).abs() < 1e-6,
            "the body yaw the partner's east speaks ({})",
            sheep.yaw
        );
        assert_eq!(sheep.breed_love_time, 1, "the court's clock counts");
        assert!(sheep.x > 8.0, "the walk the partner's east speaks");
        assert!(!sheep.aggressive, "the arms the peace keeps lowered");
    }

    #[test]
    fn the_breeders_own_court_gates_the_spawn() {
        // the BreedGoal.tick's own verdict the spawn gate reads: the
        // uncourted pair the loveTime 60 refuses (the court the
        // approach walk owes), the courted pair beyond the close ring
        // the meet refuses (the walk the distance closes), the met
        // court the child answers
        let mut next_id = 3;

        // the uncourted pair — 2.0 apart, the meet inside the ring,
        // the clocks the walk never counted
        let mut a = Zombie::new(1, 10.0, 64.0, 10.0, 262);
        a.kind = MobKind::Sheep;
        a.in_love = 600;
        let mut b = Zombie::new(2, 12.0, 64.0, 10.0, 263);
        b.kind = MobKind::Sheep;
        b.in_love = 600;
        let mut mobs = vec![a, b];
        assert!(
            try_breed(&mut mobs, &mut next_id, &mut Lcg::new(42)).is_empty(),
            "the uncourted pair the 60-clock refuses"
        );
        assert!(
            mobs[0].in_love == 600 && mobs[1].in_love == 600,
            "the refusal no clocks touch"
        );

        // the courted pair beyond the close ring — 5.0 apart, the
        // court counted, the meet the walk still owes
        let mut a = Zombie::new(1, 10.0, 64.0, 10.0, 262);
        a.kind = MobKind::Sheep;
        a.in_love = 600;
        a.breed_love_time = BREED_COURT_TICKS;
        let mut b = Zombie::new(2, 15.0, 64.0, 10.0, 263);
        b.kind = MobKind::Sheep;
        b.in_love = 600;
        b.breed_love_time = BREED_COURT_TICKS;
        let mut mobs = vec![a, b];
        assert!(
            try_breed(&mut mobs, &mut next_id, &mut Lcg::new(42)).is_empty(),
            "the distant court the meet's own ring refuses"
        );

        // the met court — the clocks the 60 keep, the meet the ring
        // answers, the child the spawn walk leaves
        let mut a = Zombie::new(1, 10.0, 64.0, 10.0, 262);
        a.kind = MobKind::Sheep;
        a.in_love = 600;
        a.breed_love_time = BREED_COURT_TICKS;
        let mut b = Zombie::new(2, 12.0, 64.0, 10.0, 263);
        b.kind = MobKind::Sheep;
        b.in_love = 600;
        b.breed_love_time = BREED_COURT_TICKS;
        let mut mobs = vec![a, b];
        let spawns = try_breed(&mut mobs, &mut next_id, &mut Lcg::new(42));
        assert_eq!(spawns.len(), 1, "the met court the child answers");
    }

    #[test]
    fn the_hurts_own_stamp_arms_the_panic_memory() {
        // shouldPanic's own gate: the last damage source the 40-tick
        // memory keeps (Entity.getLastDamageSource's own window), the
        // countdown the tick's own drain closes, the dead the hurt
        // gate refuses
        let world = ground_world();
        let mut sheep = Zombie::new(1, 8.0, 5.0, 8.0, 262);
        sheep.kind = MobKind::Sheep;
        assert_eq!(
            sheep.panic_memory_ticks, 0,
            "the peace the unhurt body keeps"
        );
        sheep.hurt(2.0);
        assert_eq!(
            sheep.panic_memory_ticks, PANIC_DAMAGE_MEMORY_TICKS,
            "the 40 the stamp arms"
        );
        // the goals' own read the window keeps alive, the drain the
        // walk's own end speaks
        sheep.tick(&world, None, NOON);
        assert_eq!(
            sheep.panic_memory_ticks,
            PANIC_DAMAGE_MEMORY_TICKS - 1,
            "the window the tick drains"
        );
        for _ in 0..(PANIC_DAMAGE_MEMORY_TICKS - 1) {
            sheep.tick(&world, None, NOON);
        }
        assert_eq!(sheep.panic_memory_ticks, 0, "the window the drain closes");
        // the dead the hurt gate refuses (the dying never re-hurt —
        // the stamp the corpse never re-arms)
        let mut corpse = Zombie::new(2, 8.0, 5.0, 8.0, 263);
        corpse.kind = MobKind::Sheep;
        corpse.health = 0.0;
        corpse.hurt(2.0);
        assert_eq!(
            corpse.panic_memory_ticks, 0,
            "the dead the hurt gate refuses"
        );
    }

    #[test]
    fn the_panic_scan_arms_the_flee_ring() {
        // canUse's own law: the memory's window the shouldPanic gate
        // answers, the flee target the DefaultRandomPos.getPos(mob, 5,
        // 4) ring keeps, the zombie's own registerGoals the kind gate
        // keeps (the monster's lane the panic never fills)
        let world = wide_flat_world();
        let mut sheep = Zombie::new(1, 10.0, 5.0, 10.0, 262);
        sheep.kind = MobKind::Sheep;
        sheep.on_ground = true;
        sheep.hurt(2.0);
        sheep.tick(&world, None, NOON);
        let (tx, ty, tz) = sheep
            .panic_target
            .expect("the flee target the canUse draw arms");
        let d2 = (tx - 10.0) * (tx - 10.0) + (tz - 10.0) * (tz - 10.0);
        assert!(
            d2 <= PANIC_RING_XZ * PANIC_RING_XZ + 1e-9,
            "the ring the 5.0 speaks ({d2})"
        );
        assert_eq!(ty, 5.0, "the ground plane the walk keeps");

        // the zombie's own registerGoals speaks no panic goal — the
        // memory the hurt stamp arms never draws
        let mut zombie = Zombie::new(2, 10.0, 5.0, 10.0, 262);
        zombie.hurt(2.0);
        zombie.tick(&world, None, NOON);
        assert!(
            zombie.panic_target.is_none(),
            "the monster's own registerGoals the panic never fills"
        );

        // the unhurt body the shouldPanic gate refuses — no draw, no
        // flee
        let mut calm = Zombie::new(3, 10.0, 5.0, 10.0, 262);
        calm.kind = MobKind::Sheep;
        calm.on_ground = true;
        calm.tick(&world, None, NOON);
        assert!(
            calm.panic_target.is_none(),
            "the unhurt body the shouldPanic gate refuses"
        );
    }

    #[test]
    fn the_panic_walk_folds_the_flee_steering() {
        // the tick's flee walk: the yaw faces the target, the impulse
        // rides the 1.25 the Sheep's own clinit speaks (the stroll
        // control the same walk at the plain attribute keeps — the
        // ratio pins), the cow's own 2.0 the AbstractCow clinit keeps,
        // the breed's stale arm the flag-conflict law replaces (the
        // court's own clock the stop zeroes)
        let world = wide_flat_world();
        let mut flee = Zombie::new(1, 8.0, 5.0, 8.0, 262);
        flee.kind = MobKind::Sheep;
        flee.on_ground = true;
        flee.panic_target = Some((13.0, 5.0, 8.0)); // 5 east
        flee.stroll_target = Some((-2.0, 8.0));
        flee.breed_partner = Some((2, 14.0, 5.0, 10.0));
        flee.breed_love_time = 30;
        let ev = flee.tick(&world, None, NOON);
        assert!(ev.attack.is_none(), "the flee no attack speaks");
        assert!(
            (flee.yaw - (-90.0)).abs() < 1e-6,
            "the yaw the target's east speaks ({})",
            flee.yaw
        );
        assert!(
            flee.stroll_target.is_none(),
            "the stroll's target the higher priority replaces"
        );
        assert!(
            flee.breed_partner.is_none(),
            "the breed's stale arm the start replaces"
        );
        assert_eq!(
            flee.breed_love_time, 0,
            "the court's own clock the stop zeroes"
        );
        let flee_dx = flee.x - 8.0;
        assert!(flee_dx > 0.0, "the walk the target's east speaks");

        // the control: the same 5-east walk the stroll's own impulse
        // rides — the ratio the 1.25 the moveTo's speedModifier pins
        let mut stroll = Zombie::new(1, 8.0, 5.0, 8.0, 262);
        stroll.kind = MobKind::Sheep;
        stroll.on_ground = true;
        stroll.stroll_target = Some((13.0, 8.0));
        stroll.tick(&world, None, NOON);
        let stroll_dx = stroll.x - 8.0;
        assert!(stroll_dx > 0.0, "the stroll's own walk the control speaks");
        let ratio = flee_dx / stroll_dx;
        assert!(
            (ratio - PANIC_SPEED_SHEEP).abs() < 1e-9,
            "the 1.25 the modifier speaks ({ratio})"
        );

        // the cow's own 2.0 the AbstractCow clinit keeps
        let mut cow_flee = Zombie::new(3, 8.0, 5.0, 8.0, 262);
        cow_flee.kind = MobKind::Cow;
        cow_flee.on_ground = true;
        cow_flee.panic_target = Some((13.0, 5.0, 8.0));
        cow_flee.tick(&world, None, NOON);
        let mut cow_stroll = Zombie::new(3, 8.0, 5.0, 8.0, 262);
        cow_stroll.kind = MobKind::Cow;
        cow_stroll.on_ground = true;
        cow_stroll.stroll_target = Some((13.0, 8.0));
        cow_stroll.tick(&world, None, NOON);
        let cow_ratio = (cow_flee.x - 8.0) / (cow_stroll.x - 8.0);
        assert!(
            (cow_ratio - PANIC_SPEED_COW).abs() < 1e-9,
            "the 2.0 the AbstractCow speaks ({cow_ratio})"
        );
    }

    #[test]
    fn the_panic_walk_stops_the_arrival_the_window_rearms() {
        // canContinueToUse's own law: the navigation's done the walk
        // stops (the isRunning word the None answers), the canUse
        // re-check the memory's window re-arms while it lives — the
        // fresh draw the walk's own restart speaks — the expired
        // window the re-arm refuses
        let world = wide_flat_world();
        let mut sheep = Zombie::new(1, 8.0, 5.0, 8.0, 262);
        sheep.kind = MobKind::Sheep;
        sheep.on_ground = true;
        // the arrival inside the slack — the stop law answers, the
        // dead window the re-arm refuses
        sheep.panic_target = Some((8.1, 5.0, 8.0));
        sheep.tick(&world, None, NOON);
        assert!(
            sheep.panic_target.is_none(),
            "the arrival the stop law answers"
        );
        // the live window the canUse re-arms — the fresh draw the next
        // tick's own canUse keeps
        sheep.panic_memory_ticks = 5;
        sheep.tick(&world, None, NOON);
        assert!(
            sheep.panic_target.is_some(),
            "the live window the canUse re-arms"
        );
    }

    #[test]
    fn the_breed_scan_refuses_the_panicked_court() {
        // getFreePartner's own isPanicking gate (the breedgoal dump's
        // candidate walk speaks): the panicked mate the pairing
        // refuses — and the panicked breeder the goal's own MOVEL flag
        // the panic's prio-1 walk owns (the GoalSelector's conflict
        // law the court's stop answers); the panicked baby the
        // follow's own flag law refuses the same way
        let mut a = Zombie::new(1, 10.0, 5.0, 10.0, 262);
        a.kind = MobKind::Sheep;
        a.in_love = 600;
        let mut b = Zombie::new(2, 14.0, 5.0, 10.0, 263);
        b.kind = MobKind::Sheep;
        b.in_love = 600;
        b.panic_target = Some((20.0, 5.0, 10.0)); // the panicked mate
        let mut mobs = vec![a, b];
        assert_eq!(
            arm_breeders(&mut mobs),
            0,
            "the panicked mate the candidate walk refuses"
        );
        assert!(mobs[0].breed_partner.is_none(), "the disarmed None keeps");

        // the panicked breeder's own court the flag law stops
        let mut c = Zombie::new(3, 10.0, 5.0, 10.0, 262);
        c.kind = MobKind::Sheep;
        c.in_love = 600;
        c.breed_partner = Some((4, 14.0, 5.0, 10.0));
        c.breed_love_time = 30;
        c.panic_target = Some((20.0, 5.0, 10.0)); // the breeder panics
        let mut mobs = vec![c];
        assert_eq!(
            arm_breeders(&mut mobs),
            0,
            "the panicked breeder the flag law refuses"
        );
        assert!(
            mobs[0].breed_partner.is_none(),
            "the court the stop law disarms"
        );
        assert_eq!(mobs[0].breed_love_time, 0, "the clock the stop zeroes");

        // the panicked baby the follow's own flag law refuses
        let mut baby = Zombie::new(5, 0.0, 5.0, 0.0, 262);
        baby.kind = MobKind::Sheep;
        baby.age = BABY_START_AGE;
        baby.on_ground = true;
        let mut parent = Zombie::new(6, 5.0, 5.0, 0.0, 263);
        parent.kind = MobKind::Sheep;
        baby.panic_target = Some((20.0, 5.0, 0.0));
        let mut mobs = vec![baby, parent];
        assert_eq!(
            arm_follow_parents(&mut mobs),
            0,
            "the panicked baby the flag law refuses"
        );
        assert!(
            mobs[0].follow_parent.is_none(),
            "the follow the stop law disarms"
        );
    }

    #[test]
    fn the_fire_lane_flees_the_water_cell() {
        // canUse's own on-fire arm: the lookForWater(5) lane the
        // burning body speaks FIRST — the water cell the flee walks
        // TO (the closest-match law the nearest cell answers), the
        // water-less ring the random draw answers
        let mut world = wide_flat_world();
        world.set(32, 5, 32, crate::world::WATER);
        world.set(32, 6, 32, crate::world::WATER);
        let mut burning = Zombie::new(1, 30.0, 5.0, 32.0, 262);
        burning.kind = MobKind::Sheep;
        burning.on_ground = true;
        // the brand burns — the fire lane's own hurt the stamp keeps
        burning.fire_ticks = 160;
        burning.tick(&world, None, NOON);
        assert_eq!(
            burning.panic_target,
            Some((32.0, 5.0, 32.0)),
            "the water cell the closest match answers"
        );

        // the water-less ring the random draw keeps — the flee target
        // the 5.0 ring answers (the pool the ring never reaches)
        let mut dry = Zombie::new(2, 8.0, 5.0, 8.0, 262);
        dry.kind = MobKind::Sheep;
        dry.on_ground = true;
        dry.fire_ticks = 160;
        dry.tick(&world, None, NOON);
        let (tx, _ty, tz) = dry.panic_target.expect("the draw the canUse keeps");
        let d2 = (tx - 8.0) * (tx - 8.0) + (tz - 8.0) * (tz - 8.0);
        assert!(
            d2 <= PANIC_RING_XZ * PANIC_RING_XZ + 1e-9,
            "the ring the 5.0 speaks ({d2})"
        );
    }

    #[test]
    fn the_breed_scan_refuses_the_wrong_pairs() {
        // canMate's own arms: the cross-kind pair the class law
        // refuses, the lone in-love mob the partner-less scan refuses,
        // the far pair the ring refuses, the third mob the one-shot
        // law leaves un-bred
        let mut sheep = Zombie::new(1, 10.0, 64.0, 10.0, 262);
        sheep.kind = MobKind::Sheep;
        sheep.in_love = 600;
        let mut zombie = Zombie::new(2, 11.0, 64.0, 10.0, 263);
        zombie.in_love = 600;
        let mut next_id = 3;
        let mut mobs = vec![sheep, zombie];
        assert!(
            try_breed(&mut mobs, &mut next_id, &mut Lcg::new(42)).is_empty(),
            "the cross-kind pair the class law refuses"
        );
        assert!(
            mobs[0].in_love == 600 && mobs[1].in_love == 600,
            "the refusal no clocks touch"
        );

        let mut lone = Zombie::new(1, 10.0, 64.0, 10.0, 262);
        lone.kind = MobKind::Sheep;
        lone.in_love = 600;
        let mut other = Zombie::new(2, 11.0, 64.0, 10.0, 263);
        other.kind = MobKind::Sheep;
        let mut mobs = vec![lone, other];
        assert!(
            try_breed(&mut mobs, &mut next_id, &mut Lcg::new(42)).is_empty(),
            "the partner-less pair the love gate refuses"
        );

        let mut far_a = Zombie::new(1, 10.0, 64.0, 10.0, 262);
        far_a.kind = MobKind::Sheep;
        far_a.in_love = 600;
        far_a.breed_love_time = BREED_COURT_TICKS;
        let mut far_b = Zombie::new(2, 30.0, 64.0, 10.0, 263);
        far_b.kind = MobKind::Sheep;
        far_b.in_love = 600;
        far_b.breed_love_time = BREED_COURT_TICKS;
        let mut mobs = vec![far_a, far_b];
        assert!(
            try_breed(&mut mobs, &mut next_id, &mut Lcg::new(42)).is_empty(),
            "the far pair the meet's own 3.0 ring refuses"
        );

        // the one-shot law: three in-love sheep — ONE pair breeds, the
        // third's clocks the scan never touches
        let mut trio: Vec<Zombie> = (0..3)
            .map(|i| {
                let mut m = Zombie::new(i as i32 + 1, 10.0 + i as f64, 64.0, 10.0, 262 + i as u64);
                m.kind = MobKind::Sheep;
                m.in_love = 600;
                m.breed_love_time = BREED_COURT_TICKS;
                m
            })
            .collect();
        let spawns = try_breed(&mut trio, &mut next_id, &mut Lcg::new(42));
        assert_eq!(spawns.len(), 1, "the one-shot law the single pair speaks");
        let bred_count = trio
            .iter()
            .filter(|m| m.age == BREED_COOLDOWN_TICKS)
            .count();
        assert_eq!(bred_count, 2, "the bred pair the cooldown names");
        let third = trio.iter().find(|m| m.age == 0).expect("the un-bred third");
        assert_eq!(
            third.in_love, 600,
            "the third's love clock the scan never touches"
        );
    }

    /// THE GATE'S OWN DENOMINATOR SPLIT (javap `EatBlockGoal.canUse`:
    /// `nextInt(adjustedTickDelay(isBaby ? 50 : 1000)) == 0` the ifeq
    /// branch the bipush-50/sipush-1000 pair speaks) — the SAME seed
    /// the twin streams keep (both mobs draw the identical values),
    /// the is_baby word the denominator picks: a draw that answers
    /// `% 50 == 0` but `% 1000 != 0` the BABY's graze starts and the
    /// ADULT's own gate refuses — the twenty-fold odds the babyhood
    /// speaks. The brute-forced seed the pin walks (the shared world,
    /// the grass block the below-cell arms).
    #[test]
    fn the_eat_gate_splits_the_denominators() {
        // the seed hunt: the gate draw answers % 50 == 0 && % 1000 != 0
        // (the second next() the tick's chain walks — the ambient's
        // first draw the baseTick law keeps)
        let mut found = None;
        for seed in 0u64..20_000 {
            let mut probe = Zombie::new(9, 0.0, 5.0, 0.0, seed);
            probe.kind = MobKind::Sheep;
            probe.age = -100;
            probe.ambient_voice(); // the first draw the tick's head keeps
            let mut adult_probe = Zombie::new(9, 0.0, 5.0, 0.0, seed);
            adult_probe.kind = MobKind::Sheep;
            adult_probe.ambient_voice();
            let baby_draw = probe.rng.next();
            let adult_draw = adult_probe.rng.next();
            assert_eq!(baby_draw, adult_draw, "the twin streams the seed keeps");
            if baby_draw % EAT_GATE_BABY == 0 && baby_draw % EAT_GATE_ADULT != 0 {
                found = Some((seed, probe, adult_probe));
                break;
            }
        }
        let (seed, _probe_b, _probe_a) = found.expect("the split seed the hunt answers");
        let world = graze_world();
        // the FRESH TWINS the tick's own stream keeps (the probe's two
        // draws the hunt spent — the tick's ambient + poll the same
        // two draws the fresh ctor re-walks)
        let mut baby = Zombie::new(9, 0.0, 5.0, 0.0, seed);
        baby.kind = MobKind::Sheep;
        // the BABY: the gate the 50 answers — the start() the 40 arms
        baby.age = -100;
        baby.tick(&world, None, NIGHT);
        assert_eq!(
            baby.eat_animation_tick, EAT_ANIMATION_TICKS,
            "the baby's gate the 1/50 the seed answers (seed {seed})"
        );
        // the ADULT: the same draw the 1000 refuses — the gate the
        // odds the adult's own walk keeps (the fresh twin the same
        // seed keeps — the poll's draw the split speaks)
        let mut adult = Zombie::new(9, 0.0, 5.0, 0.0, seed);
        adult.kind = MobKind::Sheep;
        adult.tick(&world, None, NIGHT);
        assert_eq!(
            adult.eat_animation_tick, 0,
            "the adult's gate the same draw refuses (seed {seed})"
        );
    }

    /// THE WINDOW'S OWN WALK (javap `EatBlockGoal.start/tick`): the
    /// start() the 40 window arms, the tick's decrement max(0, -1)
    /// the window drains, the hold the MOVE flag keeps (the position
    /// the frozen feet answer — no stroll, no impulse), the consume
    /// the `== adjustedTickDelay(4)` landing answers: the grass block
    /// below the DIRT speaks, the event the swap carries, the ate()
    /// walk the sheared bit resets (the wool regrow the adult's own
    /// word keeps), the window's tail the idle keeps silent.
    #[test]
    fn the_graze_window_the_consume_at_four_walks() {
        let world = graze_world();
        let mut mob = Zombie::new(9, 0.0, 5.0, 0.0, 262);
        mob.kind = MobKind::Sheep;
        // the shorn sheep the regrow's own payoff arms
        mob.sheared = true;
        // the start() the goal's own arm — the window the 40 speaks
        mob.eat_animation_tick = EAT_ANIMATION_TICKS;
        mob.stroll_target = Some((4.0, 4.0));
        let (sx, sz) = (mob.x, mob.z);
        // the window's walk: 36 ticks land the timer on 4 (the
        // consume the 36th tick answers), the position the hold keeps
        let mut consume_event = None;
        for i in 0..36 {
            let ev = mob.tick(&world, None, NIGHT);
            assert_eq!(
                mob.eat_animation_tick,
                EAT_ANIMATION_TICKS - i - 1,
                "the decrement the tick {i} walks"
            );
            assert!(
                (mob.x - sx).abs() < 1e-9 && (mob.z - sz).abs() < 1e-9,
                "the hold the MOVE flag keeps at tick {i}"
            );
            if i + 1 < 36 {
                assert!(
                    ev.eat_block.is_none(),
                    "the window's walk no swap the pre-landing keeps (tick {i})"
                );
            } else {
                consume_event = ev.eat_block;
            }
        }
        assert_eq!(
            mob.eat_animation_tick, EAT_CONSUME_TICK,
            "the landing the 36th tick walks"
        );
        assert_eq!(
            consume_event,
            Some((0, 4, 0, crate::world::DIRT)),
            "the consume the below-grass arm carries — the DIRT the setBlock speaks"
        );
        assert!(
            !mob.sheared,
            "the ate() walk the sheared bit resets — the wool regrow"
        );
        assert_eq!(
            mob.age, 0,
            "the adult the canAgeUp gate refuses — the age the zero keeps"
        );
        // the stroll target the conflict cleared — the graze's own
        // start the flag law keeps
        assert!(
            mob.stroll_target.is_none(),
            "the stroll the flag conflict stopped"
        );
        // the window's tail: the drain the zero keeps, the next tick
        // the idle walk answers (no swap — the re-armed poll the
        // 1/1000 draw the seed's own walk answers or refuses)
        let ev = mob.tick(&world, None, NIGHT);
        assert!(ev.eat_block.is_none(), "the drained window keeps silent");
    }

    /// THE SHEEP'S OWN ATE WORDS (javap `Sheep.ate()`: `super.ate();
    /// setSheared(false); if (canAgeUp()) ageUp(60)` the three calls
    /// the walk speaks) — the sheared reset BOTH ages answer, the
    /// +1200-tick jump the unlocked baby's growth rides (the -100
    /// body the ceiling's crossing grows UP — the age the zero
    /// lands, the adult the boundary arms), the locked baby the
    /// canAgeUp gate refuses (the PAUSE the dandelion's lock keeps —
    /// the sheared bit still resets), the adult the gate refuses
    /// outright.
    #[test]
    fn the_ate_walk_the_sheeps_own_words() {
        // THE UNLOCKED BABY: the ageUp the crossing grows UP
        let mut baby = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        baby.kind = MobKind::Sheep;
        baby.age = -100;
        baby.sheared = true;
        baby.ate();
        assert!(!baby.sheared, "the sheared reset the baby speaks");
        assert_eq!(
            baby.age, 0,
            "the +1200 jump the ceiling's crossing lands — the grow-up"
        );
        assert!(!baby.is_baby(), "the adult the crossing names");
        // THE LOCKED BABY: the gate refuses, the sheared bit still resets
        let mut locked = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        locked.kind = MobKind::Sheep;
        locked.age = -100;
        locked.age_locked = true;
        locked.sheared = true;
        locked.ate();
        assert!(!locked.sheared, "the sheared reset the lock never gates");
        assert_eq!(locked.age, -100, "the PAUSE the canAgeUp refusal keeps");
        // THE ADULT: the gate refuses outright, the sheared bit resets
        let mut adult = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        adult.kind = MobKind::Sheep;
        adult.sheared = true;
        adult.ate();
        assert!(!adult.sheared, "the sheared reset the adult speaks");
        assert_eq!(adult.age, 0, "the adult's age the gate never moves");
    }

    /// THE RENDER FOLDS' OWN SCALES (javap
    /// `Sheep.getHeadEatPositionScale/getHeadEatAngleScale`): the
    /// position's arms the window/ease bands speak (the 4..=36 full
    /// drop, the <4 ease-in, the >36 ease-out), the angle's chew
    /// wobble the sin fold answers, the pi/5 hold the 0<tick<=4
    /// keeps, the idle arm the xRot's deg->rad fold carries.
    #[test]
    fn the_render_folds_the_scales_speak() {
        let mut mob = Zombie::new(9, 0.0, 64.0, 0.0, 262);
        mob.kind = MobKind::Sheep;
        // THE IDLE ARM (tick 0): the lerped xRot the deg->rad fold keeps
        mob.head_pitch = 90.0;
        mob.prev_head_pitch = 90.0;
        mob.eat_animation_tick = 0;
        assert_eq!(mob.head_eat_position_at(1.0), 0.0, "the idle the 0.0 keeps");
        let idle_angle = mob.head_eat_angle_at(1.0);
        assert!(
            (idle_angle - 90.0 * EAT_IDLE_RAD).abs() < 1e-6,
            "the idle arm the xRot fold answers: {idle_angle}"
        );
        // THE WINDOW: the 4..=36 band the full drop keeps
        mob.eat_animation_tick = 36;
        assert_eq!(mob.head_eat_position_at(1.0), 1.0, "the window the 1.0 keeps");
        mob.eat_animation_tick = 4;
        assert_eq!(mob.head_eat_position_at(1.0), 1.0, "the band's head the 1.0 keeps");
        // THE EASE-IN (tick < 4): (tick - partial) / 4
        mob.eat_animation_tick = 2;
        assert!(
            (mob.head_eat_position_at(1.0) - 0.25).abs() < 1e-6,
            "the ease-in the (2-1)/4 answers"
        );
        // THE EASE-OUT (tick > 36): -(tick - 40 - partial) / 4
        mob.eat_animation_tick = 37;
        assert!(
            (mob.head_eat_position_at(1.0) - 1.0).abs() < 1e-6,
            "the ease-out's head the (37-40-1)/-4 answers"
        );
        mob.eat_animation_tick = 40;
        assert!(
            (mob.head_eat_position_at(1.0) - 0.25).abs() < 1e-6,
            "the window's own start the partial ease speaks"
        );
        // THE CHEW WOBBLE (4 < tick <= 36): the sin fold the constants keep
        mob.eat_animation_tick = 20;
        let chew = mob.head_eat_angle_at(1.0);
        let expect = EAT_ANGLE_BASE + EAT_ANGLE_AMP * ((20.0 - 4.0 - 1.0) / 32.0 * EAT_ANGLE_FREQ).sin();
        assert!(
            (chew - expect).abs() < 1e-6,
            "the chew wobble the fold answers: {chew} vs {expect}"
        );
        // THE HOLD (0 < tick <= 4): the pi/5 the graze's own pitch keeps
        mob.eat_animation_tick = 2;
        assert!(
            (mob.head_eat_angle_at(1.0) - EAT_ANGLE_BASE).abs() < 1e-6,
            "the hold the pi/5 keeps"
        );
    }

    /// THE PRIORITY'S OWN LAW (the GoalSelector's flag-conflict walk
    /// the goal chain speaks): the panic's prio-1 the RUNNING graze
    /// replaces (the eat's own stop the timer zeroes), the graze's
    /// own start the look goal's clock zeroes (the LOOK flag the
    /// conflict answers), the kind gate the cow's own walk keeps
    /// silent (the EatBlockGoal the Sheep's registerGoals only fills).
    #[test]
    fn the_graze_the_priority_law_walks() {
        let world = graze_world();
        // THE PANIC REPLACES THE RUNNING GRAZE (prio 1 over 5 — the
        // eat's own stop() the zero keeps)
        let mut mob = Zombie::new(9, 0.0, 5.0, 0.0, 262);
        mob.kind = MobKind::Sheep;
        mob.eat_animation_tick = EAT_ANIMATION_TICKS;
        mob.panic_memory_ticks = PANIC_DAMAGE_MEMORY_TICKS;
        mob.tick(&world, None, NIGHT);
        assert_eq!(
            mob.eat_animation_tick, 0,
            "the panic the graze replaces — the stop's own zero speaks"
        );
        assert!(
            mob.panic_target.is_some(),
            "the panic's own flee the memory arms"
        );
        // THE START ZEROES THE LOOK CLOCK (the LOOK flag the graze
        // holds — the look goal's own stop the conflict answers):
        // the twin-seed trick the denominator test keeps — brute-force
        // a seed the adult's 1/1000 answers
        let mut found = None;
        for seed in 0u64..20_000 {
            let mut probe = Zombie::new(9, 2.0, 5.0, 2.0, seed);
            probe.kind = MobKind::Sheep;
            probe.ambient_voice();
            let draw = probe.rng.next();
            if draw % EAT_GATE_ADULT == 0 {
                found = Some((seed, probe));
                break;
            }
        }
        let (seed, _probe) = found.expect("the adult gate seed the hunt answers");
        let mut gated = Zombie::new(9, 2.0, 5.0, 2.0, seed);
        gated.kind = MobKind::Sheep;
        gated.look_ticks = LOOK_INTERVAL_TICKS;
        gated.tick(&world, None, NIGHT);
        assert_eq!(
            gated.eat_animation_tick, EAT_ANIMATION_TICKS,
            "the adult's gate the seed answers (seed {seed})"
        );
        assert_eq!(
            gated.look_ticks, 0,
            "the start the look clock zeroes — the LOOK flag the graze holds"
        );
        // THE KIND GATE: the cow the goal never fills — the same draw
        // the cow's chain never polls (the eat gate the Sheep only
        // arms), the timer the zero keeps
        let mut cow = Zombie::new(9, 2.0, 5.0, 2.0, seed);
        cow.kind = MobKind::Cow;
        cow.tick(&world, None, NIGHT);
        assert_eq!(
            cow.eat_animation_tick, 0,
            "the cow the EatBlockGoal never fills — the kind gate keeps"
        );
    }
}


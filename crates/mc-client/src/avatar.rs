//! THE THIRD-PERSON AVATAR: the player's own body render — the vanilla
//! `HumanoidModel`/`PlayerModel` geometry and `setupAnim` pose laws,
//! byte-for-byte the bytecode's arithmetic (javap,
//! tools/jdk-25.0.4.1+1, the cached jar's Mojang-mapped classes). The
//! F5 lane's own payload: when the camera detaches, the body the
//! camera watches is THIS model, posed by the walk/hurt/crouch state
//! the first-person lanes already speak.
//!
//! Pinned sources (the class -> the law):
//! - `HumanoidModel.createMesh` / `PlayerModel.createMesh` (javap):
//!   the parts' texOffs, boxes and pivots (below, verbatim). The wide
//!   branch's arms keep the HumanoidModel's own 4-wide boxes; the
//!   overlay layers (hat/jacket/sleeves/pants) are CHILDREN of their
//!   base parts (the swing rotations propagate) and inflate +0.25,
//!   the hat +0.5.
//! - `ModelPart.translateAndRotate` (javap): `translate(pivot/16)`
//!   then `rotationZYX(zRot, yRot, xRot)` — the cube vertices are
//!   pivot-relative, so a rotation is ABOUT THE PIVOT (rotate then add
//!   the pivot). Units: model texels / 16 = blocks. The vector sees
//!   Rx first, then Ry, then Rz (the quaternion's own order).
//! - `LivingEntityRenderer.render` (javap): the entity chain is
//!   `T(entityPos) · R_y(180 - bodyRot) · S(-1, -1, 1) · T(0, -1.501, 0)`
//!   — the model's y grows DOWNWARD from 24 texels (1.501 blocks)
//!   above the feet, the x is mirrored (the model's right arm lands on
//!   the world's right), and the net yaw rotation is `(180 - bodyRot)`
//!   degrees about +Y (the joml YP fold: `x' = x cos a + z sin a; z' =
//!   -x sin a + z cos a`).
//! - `HumanoidModel.setupAnim` (javap, 26.2): the head speaks
//!   `xRot = state.xRot * (π/180)`, `yRot = state.yRot * (π/180)`
//!   (the head yaw is the RELATIVE head-body yaw — the local player's
//!   head rides its own body, so 0); the walk law:
//!   `rightArm.xRot = cos(pos*0.6662 + π) * 2 * speed * 0.5 /
//!   speedValue`, `leftArm.xRot = cos(pos*0.6662) * 2 * speed * 0.5 /
//!   speedValue`, `rightLeg.xRot = cos(pos*0.6662) * 1.4 * speed /
//!   speedValue`, `leftLeg.xRot = cos(pos*0.6662 + π) * 1.4 * speed /
//!   speedValue` — speedValue defaults 1.0 (the HumanoidRenderState
//!   constructor's own). The legs' yRot/zRot stand at ±0.005. The idle
//!   bob (`AnimationUtils.bobModelPart`, always unless SPYGlass):
//!   `zRot += s * (cos(age*0.09) * 0.05 + 0.05)`, `xRot += s *
//!   sin(age*0.067) * 0.05`, s = 1 the right arm, s = -1 the left.
//! - `HumanoidModel.setupAnim`'s crouch branch (javap): `body.xRot =
//!   0.5`, both arms `xRot += 0.4`, both leg pivots `z += 4.0`, the
//!   head pivot `y += 4.2`, the body and both arm pivots `y += 3.2`.
//!   The renderer's own offset (`AvatarRenderer.getRenderOffset`):
//!   the crouched avatar draws `scale * -2 / 16` lower.
//! - `HumanoidModel.setupAttackAnimation` (javap, 26.2): the swing
//!   law — the SHARED folds every kind keeps: `body.yRot = 0.2 *
//!   sin(sqrt(attackTime) * 2π)` (negated when the attacking arm is
//!   the left), the four arm-pivot orbits (`x = ±cos(body.yRot)·5·
//!   ageScale`, `z = ±sin(body.yRot)·5·ageScale`), the three rotation
//!   adds (`rightArm.yRot += body.yRot`, `leftArm.yRot +=`,
//!   `leftArm.xRot +=`); then the `$SwitchMap$SwingAnimationType`
//!   dispatch — WHACK: `xRot -= sin(outQuart(attackTime)·π)·1.2 +
//!   0.75·sin(attackTime·π)·(0.7 − head.xRot)`, `yRot += body.yRot·2`,
//!   `zRot += sin(attackTime·π)·−0.4`; NONE: the bare goto; STAB:
//!   `SpearAnimations.thirdPersonAttackHand` (the three adds undone,
//!   the xRot window `(90·f4 − 120·f5 + 30·f6)·(π/180)` the
//!   inOutSine/inQuad/inOutExpo windows fold).
//! - `LivingEntityRenderer.extractRenderState` (javap): the walk
//!   sampling — `walkAnimationPos = walk.position(partial)`,
//!   `walkAnimationSpeed = walk.speed(partial)` (the min(abs, 1)
//!   blend).
//! - The entity light (`rendertype_entity` shader + `ShaderInstance`,
//!   the shader source): the diffuse is the two-direction mix —
//!   `clamp(0.5 + max(0, dot(L0, n)) + max(0, dot(L1, n)), 0, 1)`
//!   with L0 = normalize(0.2, 1.0, -0.7), L1 = normalize(-0.2, 1.0,
//!   0.7) in WORLD space (the GameRenderer rotates them by the camera
//!   rotation into view space each frame), n the face's world-space
//!   normal (the model normal through the entity chain — the
//!   S(-1,-1,1) fold flips x and y).
//! - `WalkAnimationState` (body.rs, javap): the store the body owns —
//!   `speed(partial) = min(abs(speedOld + (speed - speedOld) *
//!   partial), 1)`, `position(partial) = (position - speed * (1 -
//!   partial)) * positionScale`.
//!
//! The skin pixels stream from the runtime asset root and are
//! sha1-verified against the generated `skin` table before a single
//! texel is trusted; an absent or mismatched file renders NOTHING
//! (the reference renderer must not lie about its data).

use std::path::Path;

use anyhow::Context;
use mc_registry::generated::skin;
use sha1::Digest;

use crate::math::{Mat4, Vec3};

/// `Mth.cos(double)` over the jar's own f2d call-site widening — the
/// 65536-entry SIN table the Mth body keeps (the `mth` module's law,
/// the angle the table quantizes).
fn mth_cos(x: f32) -> f32 {
    crate::mth::cos(x as f64)
}

/// `Mth.sin(double)` over the jar's own f2d call-site widening — the
/// 65536-entry SIN table the Mth body keeps.
fn mth_sin(x: f32) -> f32 {
    crate::mth::sin(x as f64)
}

/// `Mth.sqrt(float)` — the f64 sqrt narrowed back to f32.
fn mth_sqrt(x: f32) -> f32 {
    (x as f64).sqrt() as f32
}

/// The vanilla two-direction entity light (ShaderInstance's
/// LIGHT0/LIGHT1_DIRECTION constants, normalized — the world-space
/// law the view-space shader uniform folds).
fn light_dirs() -> ([f32; 3], [f32; 3]) {
    let norm = |v: [f32; 3]| {
        let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        [v[0] / l, v[1] / l, v[2] / l]
    };
    (norm([0.2, 1.0, -0.7]), norm([-0.2, 1.0, 0.7]))
}

/// The entity diffuse law: `clamp(0.5 + max(0, dot(L0, n)) +
/// max(0, dot(L1, n)), 0, 1)` (the minecraft_mix_light fold) over the
/// world normal taken into view space (`d_view = Rx(-pitch) ·
/// Ry(yaw) · d` — the inverse of the camera's own rotation).
pub fn entity_diffuse(n_world: [f32; 3], cam_yaw: f32, cam_pitch: f32) -> f32 {
    let (l0, l1) = light_dirs();
    let (sy, cy) = cam_yaw.sin_cos();
    let (sp, cp) = cam_pitch.sin_cos();
    // Ry(yaw) first ...
    let nx = n_world[0] * cy + n_world[2] * sy;
    let ny = n_world[1];
    let nz = -n_world[0] * sy + n_world[2] * cy;
    // ... then Rx(-pitch)
    let vy = ny * cp - nz * sp;
    let vz = ny * (-sp) + nz * cp;
    let nv = [nx, vy, vz];
    let d0 = (l0[0] * nv[0] + l0[1] * nv[1] + l0[2] * nv[2]).max(0.0);
    let d1 = (l1[0] * nv[0] + l1[1] * nv[1] + l1[2] * nv[2]).max(0.0);
    (0.5 + d0 + d1).clamp(0.0, 1.0)
}

/// One textured world-space quad of the avatar (the SampledQuad feed;
/// `uv` in skin texels, the sampler walks the model's own pixels).
pub struct AvatarQuad {
    pub pts: [[f32; 3]; 4],
    pub uv: [[f32; 2]; 4],
    /// the entity diffuse multiplier (the mix_light fold)
    pub shade: f32,
}

/// The pose inputs — the vanilla render state's own fields, gathered
/// from the body the camera watches.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AvatarState {
    /// vanilla yaw degrees (the body's own)
    pub yaw_deg: f32,
    /// vanilla pitch degrees, positive = DOWN (the head's xRot)
    pub pitch_deg: f32,
    /// walkAnimationPos (the backward-extrapolated walk phase)
    pub walk_pos: f32,
    /// walkAnimationSpeed (the min(abs, 1) blend)
    pub walk_speed: f32,
    pub crouching: bool,
    /// the attack-swing progress (getAttackAnim, 0 = rest)
    pub attack_time: f32,
    /// the swing rides the main hand (the right — the client's own)
    pub attack_right: bool,
    /// ageInTicks (the idle bob's clock)
    pub age_ticks: f32,
    /// THE FIRST MOB's pose branch: the zombie arms overwrite the
    /// humanoid arms (javap `AbstractZombieModel.setupAnim`'s own
    /// call chain — HumanoidModel.setupAnim, then animateZombieArms).
    pub zombie: bool,
    /// `ZombieRenderState.isAggressive` (the render state's own field —
    /// the attack goal's raiseArmTicks law drives it).
    pub zombie_aggressive: bool,
    /// THE THIN HUMANOID's branch (javap `SkeletonModel.setupAnim` the
    /// 26.2 bytecode speaks): the skeleton's own thin boxes ride the
    /// humanoid walk the flag arms — the arms/legs swap the 2x12x2
    /// cubes and the (40,16)/(0,16) tex rows the
    /// createDefaultSkeletonMesh keeps, the leg pivots the ±2 the
    /// PartPose.offset speaks (the wide ±1.9 the humanoid never
    /// shares).
    pub skeleton: bool,
    /// `SkeletonRenderState.isAggressive` (the RangedBowAttackGoal's
    /// own start()/stop() law — the goal runs while the target lives).
    pub skeleton_aggressive: bool,
    /// `SkeletonRenderState.isHoldingBow` (the main-hand item the BOW
    /// is — the gate the setupAnim branch reads: the aggressive body
    /// holding the bow never raises the bare arms, the BOW_AND_ARROW
    /// aim the HumanoidModel's own pose speaks instead).
    pub skeleton_bow: bool,
    /// The head's yaw offset from the body (the netHeadYaw the
    /// LivingEntityRenderer extraction clamps to +-75°; the player
    /// avatar's body rides the look yaw so the offset stays 0).
    pub head_yaw_off_deg: f32,
    /// `LivingEntityRenderState.deathTime` (the entity's deathTime +
    /// partialTick when dying, 0 alive) — the flip law's clock.
    pub death_time: f32,
    /// `HumanoidRenderState.swingAnimationType` (the held stack's
    /// SWING_ANIMATION component the setupAttackAnimation's $SwitchMap
    /// dispatch reads — WHACK the outQuart arc, NONE the bare return,
    /// STAB `SpearAnimations.thirdPersonAttackHand`). The render state
    /// the held item answers (the DEFAULT (WHACK, 6) the component
    /// getOrDefault keeps).
    pub swing_kind: crate::hand::SwingKind,
    /// `LivingEntityRenderState.ageScale` (the baby's 0.5, the adult's
    /// 1.0) — the four arm-pivot orbit folds the swing shares multiply
    /// the client's adults keep at 1.0 (the baby humanoid the render
    /// state's own scale the lanes never vary yet).
    pub age_scale: f32,
    /// `HumanoidRenderState.isUsingItem` — the use walk's own gate the
    /// THIRD-person lanes read (the poseArms branch matrix, the
    /// thirdPersonHandUse early-return, the per-arm ticks the item
    /// lane's animateUseItem call carries).
    pub is_using_item: bool,
    /// `HumanoidRenderState.useItemHand` — MAIN_HAND the ctor's own
    /// default speaks; the client's `true` = MAIN, `false` = OFF. The
    /// poseArms matrix and both use gates the pair drives.
    pub use_item_main_hand: bool,
    /// `HumanoidRenderState.ticksUsingItem` (the FIELD — the
    /// `timeHeld` clock `ticks - 1 + partialTick` the first person's
    /// own law keeps). The thirdPersonHandUse gate reads the FIELD
    /// both arms share; the item lane's per-arm method the client's
    /// own `(useItemHand == MAIN) == (arm == mainArm)` zero law
    /// answers.
    pub ticks_using_item: f32,
    /// `HumanoidRenderState.isFallFlying` — the elytra glide the
    /// thirdPersonHandUse base pose's extra xRot subtraction rides.
    pub fall_flying: bool,
    /// `HumanoidRenderState.swimAmount` — the swim blend the same
    /// base-pose branch reads (`fallFlying || swimAmount > 0`).
    pub swim_amount: f32,
    /// `HumanoidRenderState.rightArmPose == ArmPose.SPEAR` — the
    /// getArmPose law's own verdict (the SPEARS tag the seven tiers
    /// ride: any spear stack the pose keeps, swinging or using or
    /// neither — the SWING_ANIMATION STAB and the SPEAR use-anim gates
    /// the tag law subsumes).
    pub right_spear: bool,
    /// `HumanoidRenderState.leftArmPose == ArmPose.SPEAR` — the
    /// offhand stack's own verdict (the same tag law).
    pub left_spear: bool,
    /// `HumanoidRenderState.rightArmPose == ArmPose.ITEM` — the
    /// avatar law's own fall-through verdict (AvatarRenderer.
    /// getArmPose pc 197-200: the non-empty stack the spear verdict
    /// never answers, the use-anim gate the four mapped names the
    /// using hand excludes — shield/bow/crossbow/goat_horn the
    /// BLOCK/BOW_AND_ARROW/CROSSBOW_CHARGE/TOOT_HORN poses the jar
    /// answers instead; the food's NONE the fall-through keeps).
    /// THE MOB LAW THE VERDICT NEVER WEARS: HumanoidMobRenderer.
    /// getArmPose pc 59-62 answers EMPTY for EVERY non-spear stack —
    /// the mob lanes the ITEM fold the avatar's own lanes alone ride.
    pub right_item: bool,
    /// `HumanoidRenderState.leftArmPose == ArmPose.ITEM` — the
    /// offhand stack's own verdict (the same fall-through law).
    pub left_item: bool,
    /// `HumanoidRenderState.rightArmPose == ArmPose.BOW_AND_ARROW` —
    /// the use gate's own verdict (AvatarRenderer.getArmPose
    /// pcs 39-56 + 70-80: the hand the `getUsedItemHand()` match the
    /// `getUseItemRemainingTicks() > 0` window rides, the stack's
    /// `getUseAnimation() == BOW` the acmp answers — the aim the
    /// USING hand alone wears, the idle bow the ITEM fall-through
    /// keeps). THE MOB LAW THE VERDICT NEVER WEARS: HumanoidMobRenderer.
    /// getArmPose answers EMPTY for every stack — the skeleton's own
    /// aggressive bow the SkeletonModel branch arms, never this lane.
    pub right_bow: bool,
    /// `HumanoidRenderState.leftArmPose == ArmPose.BOW_AND_ARROW` —
    /// the offhand stack's own verdict (the same use-window law).
    pub left_bow: bool,
    /// `HumanoidRenderState.rightArmPose == ArmPose.BLOCK` — the use
    /// gate's OWN FIRST branch (AvatarRenderer.getArmPose pcs 59-69:
    /// the hand the `getUsedItemHand()` match rides, the remaining
    /// ticks the window keeps, the stack's `getUseAnimation() ==
    /// BLOCK` the acmp the shield alone answers — the RMB hold the
    /// verdict wears). THE CLINIT'S OWN FLAG PAIR: BLOCK(2,0,0) —
    /// twoHanded 0, affectsOffhandPose 0 — the poseBlockingArm body
    /// the ONE arm poses, the gate the second posing never cancels.
    /// THE MOB LAW THE VERDICT NEVER WEARS: HumanoidMobRenderer.
    /// getArmPose answers EMPTY for every non-spear stack — the
    /// zombie's own shieldless lane never rides.
    pub right_block: bool,
    /// `HumanoidRenderState.leftArmPose == ArmPose.BLOCK` — the
    /// offhand stack's own verdict (the same use-window law, the
    /// side flag the +30 the poseBlockingArm's ifeq answers).
    pub left_block: bool,
    /// `HumanoidRenderState.rightArmPose == ArmPose.SPYGLASS` — the
    /// use gate's own verdict (AvatarRenderer.getArmPose the
    /// eight-branch walk: the hand the `getUsedItemHand()` match
    /// rides, the remaining ticks the window keeps, the stack's
    /// `getUseAnimation() == SPYGLASS` the acmp the spyglass alone
    /// answers — the RMB hold the verdict wears, the idle spyglass
    /// the ITEM fall-through keeps). THE CLINIT'S OWN FLAG PAIR:
    /// SPYGLASS(7,0,0) — twoHanded 0, affectsOffhandPose 0 — the
    /// gate the second posing never cancels. THE MOB LAW THE
    /// VERDICT NEVER WEARS: HumanoidMobRenderer.getArmPose answers
    /// EMPTY for every non-spear stack — the mob lanes the lane
    /// never ride.
    pub right_spyglass: bool,
    /// `HumanoidRenderState.leftArmPose == ArmPose.SPYGLASS` — the
    /// offhand stack's own verdict (the same use-window law, the
    /// left body's fadd the mirror keeps).
    pub left_spyglass: bool,
    /// `HumanoidRenderState.rightArmPose == ArmPose.TOOT_HORN` — the
    /// use gate's own verdict (the stack's `getUseAnimation() ==
    /// TOOT_HORN` the acmp the goat horn alone answers — the horn's
    /// RMB the mouth pose wears, the idle horn the ITEM fall-through
    /// keeps). THE CLINIT'S OWN FLAG PAIR: TOOT_HORN(8,0,0) —
    /// twoHanded 0, affectsOffhandPose 0 — the gate the second
    /// posing never cancels. THE MOB LAW THE VERDICT NEVER WEARS:
    /// HumanoidMobRenderer.getArmPose answers EMPTY for every
    /// non-spear stack.
    pub right_horn: bool,
    /// `HumanoidRenderState.leftArmPose == ArmPose.TOOT_HORN` — the
    /// offhand stack's own verdict (the same use-window law, the
    /// left body's fadd the mirror keeps).
    pub left_horn: bool,
    /// `HumanoidRenderState.rightArmPose == ArmPose.CROSSBOW_CHARGE`
    /// — the use gate's own verdict (the hand match + the remaining
    /// ticks the window rides, the stack's `getUseAnimation() ==
    /// CROSSBOW` the acmp the charging crossbow alone answers — the
    /// RMB hold the verdict wears). THE CLINIT'S OWN FLAG PAIR:
    /// CROSSBOW_CHARGE(5,1,1) — twoHanded 1, affectsOffhandPose 1 —
    /// the animateCrossbowCharge body the BOTH arms poses, the gate
    /// the second posing CANCELS (the bow's true row the same law
    /// keeps). THE MOB LAW THE VERDICT NEVER WEARS: HumanoidMobRenderer
    /// .getArmPose answers EMPTY for every non-spear stack.
    pub right_crossbow_charge: bool,
    /// `HumanoidRenderState.leftArmPose == ArmPose.CROSSBOW_CHARGE`
    /// — the offhand stack's own verdict (the same use-window law,
    /// the iconst_0 flag the mirror keeps).
    pub left_crossbow_charge: bool,
    /// `HumanoidRenderState.rightArmPose == ArmPose.CROSSBOW_HOLD` —
    /// the verdict the use gate BEFORE the swinging gate answers
    /// (AvatarRenderer.getArmPose pcs 11-38: `!avatar.swinging &&
    /// stack.is(CROSSBOW) && CrossbowItem.isCharged(stack)` the
    /// loaded crossbow alone answers — the idle hold the walk wears,
    /// the swing's own arc the verdict excludes). THE CLINIT'S OWN
    /// FLAG PAIR: CROSSBOW_HOLD(6,1,1) — twoHanded 1,
    /// affectsOffhandPose 1 — the animateCrossbowHold body the BOTH
    /// arms poses, the gate the second posing CANCELS.
    pub right_crossbow_hold: bool,
    /// `HumanoidRenderState.leftArmPose == ArmPose.CROSSBOW_HOLD` —
    /// the offhand stack's own verdict (the same !swinging + charged
    /// law, the iconst_0 flag the mirror keeps).
    pub left_crossbow_hold: bool,
    /// `HumanoidRenderState.rightArmPose == ArmPose.THROW_TRIDENT`
    /// — the use gate's own TRIDENT branch (AvatarRenderer.getArmPose
    /// pcs 81-91: the hand match + the remaining-ticks window the
    /// use-anim walk rides, the stack's `getUseAnimation() ==
    /// TRIDENT` the acmp the charging trident alone answers — the
    /// RMB charge the verdict wears, the idle trident the SPEARS tag
    /// the row table excludes the ITEM fall-through keeps). THE
    /// CLINIT'S OWN FLAG PAIR: THROW_TRIDENT(4,0,1) — twoHanded 0,
    /// affectsOffhandPose 1 — the case-4 body the ONE arm poses, the
    /// gate the second posing's CHAIN CANCELS (the bow's true row
    /// the same law keeps). THE MOB LAW THE VERDICT NEVER WEARS:
    /// HumanoidMobRenderer.getArmPose answers EMPTY for every
    /// non-spear stack — the drowned's own trident the mob lanes
    /// never ride.
    pub right_trident: bool,
    /// `HumanoidRenderState.leftArmPose == ArmPose.THROW_TRIDENT`
    /// — the offhand stack's own verdict (the same use-window law,
    /// the poseLeftArm pc 119-148 mirror the same constants keeps).
    pub left_trident: bool,
    /// `HumanoidRenderState.rightArmPose == ArmPose.BRUSH` — the use
    /// gate's own BRUSH branch (AvatarRenderer.getArmPose pcs 125-135:
    /// the hand match + the remaining-ticks window the use-anim walk
    /// rides, the stack's `getUseAnimation() == BRUSH` the acmp the
    /// brushing stack alone answers — the RMB brush hold the verdict
    /// wears, the idle brush no verdict the ITEM fall-through keeps).
    /// THE CLINIT'S OWN FLAG PAIR: BRUSH(9,0,0) — twoHanded 0,
    /// affectsOffhandPose 0 — the case-9 body the ONE arm poses, the
    /// gate the second posing never cancels (the FALSE row the BLOCK
    /// pair shares). THE MOB LAW THE VERDICT NEVER WEARS:
    /// HumanoidMobRenderer.getArmPose answers EMPTY for every
    /// non-spear stack — the mob lanes the brush never ride.
    pub right_brush: bool,
    /// `HumanoidRenderState.leftArmPose == ArmPose.BRUSH` — the
    /// offhand stack's own verdict (the same use-window law, the
    /// poseLeftArm pc 294-323 mirror the same constants keeps).
    pub left_brush: bool,
    /// `HumanoidRenderState.maxCrossbowChargeDuration` — the charge
    /// body's own denominator (CrossbowItem.getChargeDuration:
    /// `floor(modifyCrossbowChargingTime(stack, entity, 1.25) × 20.0)`
    /// — the unmodified 1.25s × 20 the 25 ticks the client's
    /// CROSSBOW_CHARGE_TICKS keeps). The render state's ctor default
    /// 0.0 the rest walk keeps.
    pub max_crossbow_charge_duration: f32,
    /// `SpearAnimations$UseParams.fromKineticWeapon(useStack, ticks)`
    /// — the USING stack's own ten floats, precomputed the screens
    /// lane's item-name knowledge keeps (the render state the jar's
    /// stack component answers; the client the row table reads). Both
    /// third-person folds share the one pair. `None` = not using, or
    /// the using stack carries no KINETIC_WEAPON — the folds' own
    /// kinetic null-return the jar's pcs 163-168 keeps.
    pub kinetic_use_params: Option<crate::hand::SpearUseParams>,
    /// `ArmedEntityRenderState.ticksSinceKineticHitFeedback` — the hit
    /// jolt's clock (`(gameTime - last) + partialTick`, the negative
    /// stamp the 0.0 keeps) the item lane's hitFeedbackAmount reads.
    pub ticks_since_kinetic_hit: f32,
    /// `ArmedEntityRenderState.getMainHandItemStack().get(KINETIC_WEAPON)
    /// != null` — the MAIN hand stack's own component verdict the STAB
    /// attack-item lane's forward reads (`thirdPersonAttackItem` pcs
    /// 10-36: the component the stack carries answers the shared
    /// 0.38, the null the 0.0 the rotation-only fold wears). The row
    /// table the seven kinetic tiers keep the client's own verdict
    /// — the SWING_ANIMATION STAB the gate reads the SEPARATE
    /// component the stack may alone carry (the two components the
    /// jar reads independently, the client the two verdicts keeps).
    pub main_hand_kinetic: bool,
}

impl AvatarState {
    /// The rest state at a pose — the deterministic scenes' own (the
    /// walk store empty, no swing, standing).
    pub fn rest(yaw_deg: f32, pitch_deg: f32, age_ticks: f32) -> Self {
        AvatarState {
            yaw_deg,
            pitch_deg,
            walk_pos: 0.0,
            walk_speed: 0.0,
            crouching: false,
            attack_time: 0.0,
            attack_right: true,
            age_ticks,
            zombie: false,
            zombie_aggressive: false,
            skeleton: false,
            skeleton_aggressive: false,
            skeleton_bow: false,
            head_yaw_off_deg: 0.0,
            death_time: 0.0,
            swing_kind: crate::hand::SwingKind::Whack,
            age_scale: 1.0,
            is_using_item: false,
            // `HumanoidRenderState`'s ctor default — `useItemHand =
            // MAIN_HAND` the field init speaks
            use_item_main_hand: true,
            ticks_using_item: 0.0,
            fall_flying: false,
            swim_amount: 0.0,
            right_spear: false,
            left_spear: false,
            right_item: false,
            left_item: false,
            right_bow: false,
            left_bow: false,
            right_block: false,
            left_block: false,
            right_spyglass: false,
            left_spyglass: false,
            right_horn: false,
            left_horn: false,
            right_crossbow_charge: false,
            left_crossbow_charge: false,
            right_crossbow_hold: false,
            left_crossbow_hold: false,
            right_trident: false,
            left_trident: false,
            right_brush: false,
            left_brush: false,
            max_crossbow_charge_duration: 0.0,
            kinetic_use_params: None,
            ticks_since_kinetic_hit: 0.0,
            main_hand_kinetic: false,
        }
    }
}

/// One pivot-relative cube: texOffs, the box (texels) and the
/// `CubeDeformation` inflation (the overlay layers' extend), plus the
/// builder's MIRROR flag (javap `CubeListBuilder.mirror` — the
/// quadruped's left legs speak it: every face rect's u axis flips).
struct Cube {
    tex: (f32, f32),
    origin: [f32; 3],
    size: [f32; 3],
    inflate: f32,
    mirror: bool,
}

impl Clone for Cube {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Cube {}

/// One model part: the pivot (texels), the setupAnim rotation
/// (radians), the cubes and the overlay children (the children ride
/// the parent's rotation — PartPose.ZERO relative to it).
struct Part {
    pivot: [f32; 3],
    x_rot: f32,
    y_rot: f32,
    z_rot: f32,
    cubes: Vec<Cube>,
    children: Vec<Part>,
}

impl Part {
    /// The part's matrix — `parent · T(pivot/16) · Rz · Ry · Rx` (the
    /// translateAndRotate bytecode's own order: the vector sees Rx
    /// first, then Ry, then Rz, then the pivot translate).
    fn matrix(&self, parent: &Mat4) -> Mat4 {
        let m = Mat4::translation(self.pivot[0] / 16.0, self.pivot[1] / 16.0, self.pivot[2] / 16.0)
            .mul(&Mat4::rot_z(self.z_rot))
            .mul(&Mat4::rot_y(self.y_rot))
            .mul(&Mat4::rot_x(self.x_rot));
        parent.mul(&m)
    }
}

/// The full player model (wide Steve), the parts' pivots/boxes the
/// createMesh bytecode speaks.
pub struct AvatarModel {
    rgba: Vec<u8>,
    /// The texture's own height (the humanoid's 64x64 and the
    /// skeleton's 64x32 the two LayerDefinition.create calls speak —
    /// the pixel read clamps v to the model's OWN height the layout
    /// keeps, the 64x32 rows the shared 64-wide stride walks).
    side_v: u32,
}

impl AvatarModel {
    /// Load + sha1-verify the wide Steve skin (the same runtime file
    /// and generated sha1 the first-person arm speaks) — `None` = not
    /// drawn (the renderer must not lie about its data).
    pub fn load(root: &Path) -> Option<AvatarModel> {
        let bytes = std::fs::read(root.join(skin::RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::STEVE_WIDE_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != skin::SKIN_SIDE
            || h != skin::SKIN_SIDE
            || rgba.len() != (skin::SKIN_SIDE as usize) * (skin::SKIN_SIDE as usize) * 4
        {
            return None;
        }
        Some(AvatarModel {
            rgba,
            side_v: skin::SKIN_SIDE,
        })
    }

    /// Load + sha1-verify the ZOMBIE skin (the first mob's own runtime
    /// file — javap `AbstractZombieRenderer`'s ZOMBIE_LOCATION, the same
    /// 64x64 wide humanoid layout the geometry unwraps). `None` = the
    /// mob never renders (the same honesty the player path speaks).
    pub fn load_zombie(root: &Path) -> Option<AvatarModel> {
        let bytes = std::fs::read(root.join(skin::ZOMBIE_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::ZOMBIE_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != skin::SKIN_SIDE
            || h != skin::SKIN_SIDE
            || rgba.len() != (skin::SKIN_SIDE as usize) * (skin::SKIN_SIDE as usize) * 4
        {
            return None;
        }
        Some(AvatarModel {
            rgba,
            side_v: skin::SKIN_SIDE,
        })
    }

    /// Load + sha1-verify the SKELETON skin (the thin humanoid's own
    /// runtime file — javap `AbstractSkeletonRenderer`'s SKELETON_
    /// LOCATION, the 64x32 HALF-HEIGHT layout the SkeletonModel's own
    /// `LayerDefinition.create(64, 32)` speaks). `None` = the mob never
    /// renders (the same honesty the other kinds keep).
    pub fn load_skeleton(root: &Path) -> Option<AvatarModel> {
        let bytes = std::fs::read(root.join(skin::SKELETON_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::SKELETON_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != skin::SKIN_SIDE
            || h != 32
            || rgba.len() != (skin::SKIN_SIDE as usize) * 32 * 4
        {
            return None;
        }
        Some(AvatarModel { rgba, side_v: 32 })
    }

    /// The skin pixel at (u, v) — the RGBA row-major words, v down.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let side = skin::SKIN_SIDE as f32;
        let side_v = self.side_v as f32;
        let (uu, vv) = (u.clamp(0.0, side - 1e-4), v.clamp(0.0, side_v - 1e-4));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * skin::SKIN_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the world-space textured quads of the whole
    /// model at `state` — the entity chain folded per vertex (see the
    /// module docs for the exact matrix order). `feet` = the body's
    /// feet position; `cam_yaw/cam_pitch` speak the view rotation the
    /// entity diffuse folds.
    pub fn quads(
        &self,
        state: &AvatarState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_posed_parts(state);
        // THE ENTITY CHAIN (LivingEntityRenderer.render) — the shared
        // fold every living model rides (the crouch render offset is
        // the humanoid's own branch, the quadruped never speaks it)
        let crouch_drop = if state.crouching { -2.0 / 16.0 } else { 0.0 };
        let (fold_point, fold_normal) =
            entity_folds(state.yaw_deg, state.death_time, crouch_drop, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(12 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// THE ENTITY CHAIN (LivingEntityRenderer.render), folded per resolved
/// model point: T(0, -1.501, 0) first, then S(-1,-1,1), then the DEATH
/// FLIP (javap: the deathTime branch's ZP rotation —
/// sqrt(min(1, (deathTime-1)/20 * 1.6)) * 90°, riding BETWEEN the S
/// and the R_y), then R_y(180 - bodyRot), then the feet translation
/// (+ any pre-forgone render offset the caller's own branch speaks).
/// Returns the point and normal folds EVERY living model shares.
fn entity_folds(
    yaw_deg: f32,
    death_time: f32,
    crouch_drop: f32,
    feet: Vec3,
) -> (
    impl Fn([f32; 3]) -> [f32; 3],
    impl Fn([f32; 3]) -> [f32; 3],
) {
    let a = (180.0f32 - yaw_deg).to_radians();
    let (sa, ca) = a.sin_cos();
    // THE DEATH FLIP (the same javap branch): the state clock rides
    // the entity's deathTime + partialTick (0 = alive — the flip 0)
    let flip_rad = if death_time > 0.0 {
        let f5 = ((death_time - 1.0) / 20.0 * 1.6).max(0.0);
        mth_sqrt(f5).min(1.0) * std::f32::consts::FRAC_PI_2
    } else {
        0.0
    };
    let (sf, cf) = flip_rad.sin_cos();
    let fold_point = move |p: [f32; 3]| -> [f32; 3] {
        let mx = -p[0]; // the S(-1,-1,1) x flip (blocks)
        let my = -p[1]; // the y flip over the -1.501 line
        let mz = p[2];
        // the ZP death flip (between the S and the R_y)
        let fx = mx * cf - my * sf;
        let fy = mx * sf + my * cf;
        let px = fx * ca + mz * sa; // the joml YP yaw fold
        let pz = -fx * sa + mz * ca;
        [
            feet.x + px,
            feet.y as f32 + (fy + 1.501) + crouch_drop,
            feet.z + pz,
        ]
    };
    let fold_normal = move |n: [f32; 3]| -> [f32; 3] {
        let nx = -n[0];
        let ny = -n[1];
        let nz = n[2];
        let fx = nx * cf - ny * sf;
        let fy = nx * sf + ny * cf;
        [fx * ca + nz * sa, fy, -fx * sa + nz * ca]
    };
    (fold_point, fold_normal)
}

/// THE ARROW MODEL (javap `ArrowModel.createBodyLayer` +
/// `ArrowRenderer.submit` the 26.2 bytecode speaks): the flat
/// cross-quads the 32x32 arrow texture unwraps — the root the 0.9
/// scale the transformed mesh keeps, the `back` plane the 5x5
/// zero-width box (0,-2.5,-2.5) the pivot (-11,0,0) + the 0.8 scale
/// rides, the two `cross` planes the 16x4 zero-depth boxes the ±45°
/// /135° X rotations swing (the texScale (1, 0.8) the addBox's own
/// delta pair speaks — the sampled rects the (1,1) default the rest
/// of the models keep). The renderer's own chain: RotY(yRot − 90°)
/// then RotZ(xRot), the shake the root's zRot the setupAnim adds.
pub struct ArrowModel {
    rgba: Vec<u8>,
}

impl ArrowModel {
    /// Load + sha1-verify the arrow's own texture (javap
    /// `TippableArrowRenderer`'s TEXTURE_0 — the 32x32 the
    /// LayerDefinition.create mesh speaks). `None` = the arrows never
    /// render (the honesty the other kinds keep).
    pub fn load(root: &Path) -> Option<ArrowModel> {
        let bytes = std::fs::read(root.join("entity/projectiles/arrow.png")).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != "38d8df11d81c579d0f4b3b9c900f604dc9bb784a" {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 32 || h != 32 || rgba.len() != 32 * 32 * 4 {
            return None;
        }
        Some(ArrowModel { rgba })
    }

    /// The texture pixel at (u, v) — the RGBA row-major words, v down.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 31.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 32 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the world-space textured quads the flat
    /// cross speaks at the arrow's own pose — `pos` the lerped
    /// position, `y_rot/x_rot` the flight rotation the extract's
    /// partial read carries, `shake` the stuck arrow's own clock.
    pub fn quads(
        &self,
        pos: Vec3,
        y_rot: f32,
        x_rot: f32,
        shake: f32,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        // THE RENDERER'S OWN CHAIN (javap ArrowRenderer.submit: the
        // YP yaw (yRot − 90°) then the ZP pitch) + the shake the
        // ArrowModel.setupAnim's own zRot adds (-sin(shake·3)·shake
        // degrees the 0.017453292 fold rides)
        let shake_rad = if shake > 0.0 {
            -mth_sin(shake * 3.0) * shake * 0.017_453_292
        } else {
            0.0
        };
        let base = Mat4::translation(pos.x, pos.y, pos.z)
            .mul(&Mat4::rot_y((y_rot - 90.0).to_radians()))
            .mul(&Mat4::rot_z(x_rot.to_radians()))
            .mul(&Mat4::rot_z(shake_rad));
        // THE PARTS (createBodyLayer's own numbers — the boxes the
        // UNSCALED texel dims keep: the UV rects the box dims answer
        // BEFORE the model transform's scale the geometry folds, the
        // texScale the rect's own division speaks)
        let back_cube = Cube {
            tex: (0.0, 0.0),
            origin: [0.0, -2.5, -2.5],
            size: [0.0, 5.0, 5.0],
            inflate: 0.0,
            mirror: false,
        };
        let cross_cube = Cube {
            tex: (0.0, 0.0),
            origin: [-12.0, -2.0, 0.0],
            size: [16.0, 4.0, 0.0],
            inflate: 0.0,
            mirror: false,
        };
        // the transform chain: the ROOT's 0.9 scale the base carries,
        // the back's own pivot (-11, 0, 0) + the 0.8 part scale the
        // PartPose.offsetAndRotation.withScale ride, the cross pair's
        // ±45°/135° X rotations the PartPose.rotation speaks
        let root = base.mul(&Mat4::scale_3d(0.9, 0.9, 0.9));
        let back_m = root
            .mul(&Mat4::translation(-11.0 / 16.0, 0.0, 0.0))
            .mul(&Mat4::scale_3d(0.8, 0.8, 0.8));
        let cross1_m = root.mul(&Mat4::rot_x(0.785_398_2));
        let cross2_m = root.mul(&Mat4::rot_x(2.356_194_5));
        let parts = [
            (&back_cube, &back_m, (1.0f32, 1.0f32)),
            (&cross_cube, &cross1_m, (1.0f32, 0.8f32)),
            (&cross_cube, &cross2_m, (1.0f32, 0.8f32)),
        ];
        // THE IDENTITY FOLD (the matrix the world transform carries —
        // the projectile the LivingEntity's own S/flip chain never
        // rides)
        let identity = |p: [f32; 3]| -> [f32; 3] { p };
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(18);
        for (cube, m, tex_scale) in &parts {
            emit_cube(cube, m, &identity, &identity, cam_yaw, cam_pitch, *tex_scale, &mut out);
        }
        out
    }
}

/// The QUADRUPED's pose inputs — the pig render state's own fields
/// (javap `QuadrupedModel.setupAnim(LivingEntityRenderState)`: the
/// walk fold + the head's degree->radian pair the pool's 0.017453
/// speaks; no crouch, no arms — the quadruped has neither branch).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PigState {
    /// vanilla yaw degrees (the body's own)
    pub yaw_deg: f32,
    /// vanilla pitch degrees, positive = DOWN (the head's xRot)
    pub pitch_deg: f32,
    /// the head's yaw offset from the body (the netHeadYaw clamp)
    pub head_yaw_off_deg: f32,
    /// walkAnimationPos (the backward-extrapolated walk phase)
    pub walk_pos: f32,
    /// walkAnimationSpeed (the min(abs, 1) blend)
    pub walk_speed: f32,
    /// ageInTicks (the pig's idle owns no bob — carried for parity)
    pub age_ticks: f32,
    /// `LivingEntityRenderState.deathTime` (the same flip law's clock
    /// the avatar fold speaks — one chain, every living model)
    pub death_time: f32,
}

/// The full pig model — the 26.2 `PigModel.createBodyLayer`'s own
/// numbers, bytecode-proven off the official jar (the classfile's
/// constant pool + the createBasePigModel walk): the head 8x8x8 at
/// pivot (0, 12, -6) with the snout cube (uv 16,16), the body
/// 10x16x8 rotated x=PI/2 at pivot (0, 11, 0), the four 4x6x4 legs at
/// pivots (+-3, 18, +7/-5), `LayerDefinition.create(64, 64)`.
pub struct PigModel {
    rgba: Vec<u8>,
}

impl PigModel {
    /// Load + sha1-verify the PIG's temperate skin (the PigRenderer's
    /// variant table's default — the generated table's own constant).
    /// `None` = the pig never renders (the same honesty the zombie
    /// path speaks: the renderer must not lie about its data).
    pub fn load(root: &Path) -> Option<PigModel> {
        Self::load_skin(
            root,
            skin::PIG_TEMPERATE_RUNTIME_PATH,
            skin::PIG_TEMPERATE_PNG_SHA1,
        )
    }

    /// THE WARM VARIANT'S OWN SKIN (warm.json's asset_id — the plain
    /// adult GEOMETRY the warm texture wears, the pig's warm.json the
    /// model field never names): the same load walk, the variant
    /// table's own sha1-verdict. `None` = the warm pigs stay unseen.
    pub fn load_warm(root: &Path) -> Option<PigModel> {
        Self::load_skin(
            root,
            skin::PIG_WARM_RUNTIME_PATH,
            skin::PIG_WARM_PNG_SHA1,
        )
    }

    /// The variant skin's own load walk (the path + sha1 pair the
    /// caller names, the 64x64 layout the decode verifies).
    fn load_skin(root: &Path, path: &str, sha1_hex: &str) -> Option<PigModel> {
        let bytes = std::fs::read(root.join(path)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != sha1_hex {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 64 || rgba.len() != 64 * 64 * 4 {
            return None;
        }
        Some(PigModel { rgba })
    }

    /// The skin pixel at (u, v) — the RGBA row-major words, v down.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 63.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the world-space textured quads of the pig at
    /// `state` — the SAME entity chain fold the avatar speaks (one
    /// law, every living model), the quadruped's own parts beneath.
    pub fn quads(
        &self,
        state: &PigState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_pig_parts(state, false);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// THE COLD PIG'S OWN MODEL — the 26.2 `ColdPigModel.createBodyLayer`'s
/// own replace walk, bytecode-proven: the cold layer rebuilds the base
/// mesh (`createBasePigModel(deformation)` the shared walk keeps) then
/// REPLACES the body — the thick hide coat (texOffs (28, 32), the SAME
/// 10x16x8 box the `CubeDeformation(0.5)` inflate the addBox speaks)
/// rides the base barrel; the head and the legs the base keeps.
/// `LayerDefinition.create(mesh, 64, 64)`. The setupAnim the
/// QuadrupedModel parent inherits rides UNCHANGED.
pub struct ColdPigModel {
    rgba: Vec<u8>,
}

impl ColdPigModel {
    /// Load + sha1-verify the PIG's cold skin (cold.json's asset_id —
    /// the variant table's own pair). The `None` = the cold pigs stay
    /// unseen (the same honesty the other variants keep).
    pub fn load(root: &Path) -> Option<ColdPigModel> {
        let bytes = std::fs::read(root.join(skin::PIG_COLD_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::PIG_COLD_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 64 || rgba.len() != 64 * 64 * 4 {
            return None;
        }
        Some(ColdPigModel { rgba })
    }

    /// The skin pixel at (u, v) — the same 64x64 layout's own stride.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 63.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the cold pig's own parts (the coat the
    /// builder's cold arm speaks) beneath the same chain fold.
    pub fn quads(
        &self,
        state: &PigState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_pig_parts(state, true);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The quadruped's parts, posed by `state` — the javap-proven mesh:
/// `PigModel.createBasePigModel` overrides the head over
/// `QuadrupedModel.createBodyMesh(6, true, false, deformation)`'s own
/// stand (the snout rides the SAME part's cube list — the fluent
/// chain's second texOffs/addBox pair), the body and the legs speak
/// the quadruped base verbatim. THE WALK (javap
/// `QuadrupedModel.setupAnim`): the DIAGONAL pairs swing together —
/// right-hind + left-front in phase, the other pair +PI, the pool's
/// own 0.6662 / 1.4 the fold speaks.
fn build_pig_parts(state: &PigState, cold: bool) -> Vec<Part> {
    let pos = state.walk_pos;
    let speed = state.walk_speed;
    let leg_rh = mth_cos(pos * 0.6662) * 1.4 * speed;
    let leg_lh = mth_cos(pos * 0.6662 + std::f32::consts::PI) * 1.4 * speed;
    let head_x = state.pitch_deg.to_radians();
    let head_y = state.head_yaw_off_deg.to_radians();
    let leg_cube = |mirror: bool| Cube {
        tex: (0.0, 16.0),
        origin: [-2.0, 0.0, -2.0],
        size: [4.0, 6.0, 4.0],
        inflate: 0.0,
        mirror,
    };
    vec![
        // head (+ the snout — the same part's second cube)
        Part {
            pivot: [0.0, 12.0, -6.0],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![
                Cube {
                    tex: (0.0, 0.0),
                    origin: [-4.0, -4.0, -8.0],
                    size: [8.0, 8.0, 8.0],
                    inflate: 0.0,
                    mirror: false,
                },
                Cube {
                    tex: (16.0, 16.0),
                    origin: [-2.0, 0.0, -9.0],
                    size: [4.0, 3.0, 1.0],
                    inflate: 0.0,
                    mirror: false,
                },
            ],
            children: vec![],
        },
        // body (the horizontal barrel — the rotX PI/2 the PartPose
        // offsetAndRotation speaks). THE COLD ARM the ColdPigModel
        // .createBodyLayer's own replace keeps: the thick hide coat
        // (texOffs (28, 32) — the SAME 10x16x8 box the
        // CubeDeformation(0.5) inflate the bytecode's addBox speaks)
        // rides the same barrel
        Part {
            pivot: [0.0, 11.0, 0.0],
            x_rot: std::f32::consts::FRAC_PI_2,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: if cold {
                vec![
                    Cube {
                        tex: (28.0, 8.0),
                        origin: [-5.0, -10.0, -7.0],
                        size: [10.0, 16.0, 8.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (28.0, 32.0),
                        origin: [-5.0, -10.0, -7.0],
                        size: [10.0, 16.0, 8.0],
                        inflate: 0.5,
                        mirror: false,
                    },
                ]
            } else {
                vec![Cube {
                    tex: (28.0, 8.0),
                    origin: [-5.0, -10.0, -7.0],
                    size: [10.0, 16.0, 8.0],
                    inflate: 0.0,
                    mirror: false,
                }]
            },
            children: vec![],
        },
        // right hind leg (the builder's own unwrap — no mirror)
        Part {
            pivot: [-3.0, 18.0, 7.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(false)],
            children: vec![],
        },
        // left hind leg (the mirrored builder — the u axis flips)
        Part {
            pivot: [3.0, 18.0, 7.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(true)],
            children: vec![],
        },
        // right front leg (anti-phase of the right hind — the diagonal
        // pair the setupAnim's own cos fold speaks)
        Part {
            pivot: [-3.0, 18.0, -5.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(false)],
            children: vec![],
        },
        // left front leg (in phase with the right hind)
        Part {
            pivot: [3.0, 18.0, -5.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(true)],
            children: vec![],
        },
    ]
}

/// THE QUADRUPED POSE the bovine shares (the javap-proven law: 26.2's
/// `CowModel` carries NO setupAnim override — the dump's own method
/// table lists only the layer factory + `getHead`, so the cow speaks
/// `QuadrupedModel.setupAnim`'s diagonal fold verbatim — the SAME walk
/// store, the SAME head pose inputs, every field the pig state owns).
pub type QuadrupedState = PigState;

/// The full cow model — the 26.2 `CowModel.createBaseCowModel`'s own
/// numbers, bytecode-proven off the official jar (scripts/classfile_dump.py:
/// the constant pool + the createBaseCowModel walk): the head 8x8x6
/// (uv 0,0) at pivot (0, 4, -8) with the muzzle cube (uv 1,33) and the
/// two horn cubes (uv 22,0) riding the SAME part's fluent chain, the
/// body 12x18x10 (uv 18,4) with the udder cube (uv 52,0) rotated x=PI/2
/// at pivot (0, 5, 2), the four 4x12x4 legs (uv 0,16) at pivots
/// (+-4, 12, +7/-5) — the LEFT pair's builders the MIRROR flag rides
/// (the astore/aload walk's own pairing), `LayerDefinition.create(64, 64)`.
pub struct CowModel {
    rgba: Vec<u8>,
}

impl CowModel {
    /// Load + sha1-verify the COW's temperate skin (the CowRenderer's
    /// variant table's default — the generated table's own constant).
    /// `None` = the cow never renders (the same honesty the pig path
    /// speaks: the renderer must not lie about its data).
    pub fn load(root: &Path) -> Option<CowModel> {
        let bytes = std::fs::read(root.join(skin::COW_TEMPERATE_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::COW_TEMPERATE_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 64 || rgba.len() != 64 * 64 * 4 {
            return None;
        }
        Some(CowModel { rgba })
    }

    /// The skin pixel at (u, v) — the RGBA row-major words, v down.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 63.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the world-space textured quads of the cow at
    /// `state` — the SAME entity chain fold the avatar speaks (one
    /// law, every living model), the bovine's own parts beneath.
    pub fn quads(
        &self,
        state: &QuadrupedState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_cow_parts(state, crate::mob::ChickenVariantKind::Temperate);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// THE COLD/WARM COWS' OWN MODELS — the 26.2 `ColdCowModel`/
/// `WarmCowModel.createBodyLayer`'s own replace walks, bytecode-proven:
/// the COLD layer rebuilds the base mesh then replaces the body (the
/// thick hide coat texOffs (20, 32) — the SAME 12x18x10 box the
/// `CubeDeformation(0.5)` inflate the addBox speaks — rides the base
/// barrel) and the head (the (9, 33) tuft the horns DROPS); the WARM
/// layer replaces the head (the BIG horns (27, 0) 4x2x2 + (39, 0)
/// 2x2x2 pairs the mirrored walk rides — the base's 1x3x1 pair the
/// replace never names); `LayerDefinition.create(mesh, 64, 64)`. The
/// setupAnim the QuadrupedModel parent inherits rides UNCHANGED.
pub struct ColdCowModel {
    rgba: Vec<u8>,
}

pub struct WarmCowModel {
    rgba: Vec<u8>,
}

impl ColdCowModel {
    /// Load + sha1-verify the COW's cold skin (cold.json's asset_id).
    /// `None` = the cold cows stay unseen (the same honesty the other
    /// variants keep).
    pub fn load(root: &Path) -> Option<ColdCowModel> {
        let bytes = std::fs::read(root.join(skin::COW_COLD_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::COW_COLD_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 64 || rgba.len() != 64 * 64 * 4 {
            return None;
        }
        Some(ColdCowModel { rgba })
    }

    /// The skin pixel at (u, v) — the same 64x64 layout's own stride.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 63.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the cold cow's own parts (the coat + the
    /// horn-less head the cold arms speak) beneath the same chain
    /// fold.
    pub fn quads(
        &self,
        state: &QuadrupedState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_cow_parts(state, crate::mob::ChickenVariantKind::Cold);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

impl WarmCowModel {
    /// Load + sha1-verify the COW's warm skin (warm.json's asset_id).
    /// `None` = the warm cows stay unseen.
    pub fn load(root: &Path) -> Option<WarmCowModel> {
        let bytes = std::fs::read(root.join(skin::COW_WARM_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::COW_WARM_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 64 || rgba.len() != 64 * 64 * 4 {
            return None;
        }
        Some(WarmCowModel { rgba })
    }

    /// The skin pixel at (u, v) — the same 64x64 layout's own stride.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 63.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the warm cow's own parts (the big horns the
    /// warm arm speaks) beneath the same chain fold.
    pub fn quads(
        &self,
        state: &QuadrupedState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_cow_parts(state, crate::mob::ChickenVariantKind::Warm);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The bovine's parts, posed by `state` — the javap-proven mesh:
/// `CowModel.createBaseCowModel`'s own fluent chains (the head's four
/// cubes, the body's two, the shared leg builders the LEFT pair's
/// mirror rides). THE WALK: the same `QuadrupedModel.setupAnim`
/// diagonal fold the pig speaks — right-hind + left-front in phase,
/// the other pair +PI (the cow never overrides it).
fn build_cow_parts(state: &QuadrupedState, variant: crate::mob::ChickenVariantKind) -> Vec<Part> {
    let pos = state.walk_pos;
    let speed = state.walk_speed;
    let leg_rh = mth_cos(pos * 0.6662) * 1.4 * speed;
    let leg_lh = mth_cos(pos * 0.6662 + std::f32::consts::PI) * 1.4 * speed;
    let head_x = state.pitch_deg.to_radians();
    let head_y = state.head_yaw_off_deg.to_radians();
    let leg_cube = |mirror: bool| Cube {
        tex: (0.0, 16.0),
        origin: [-2.0, 0.0, -2.0],
        size: [4.0, 12.0, 4.0],
        inflate: 0.0,
        mirror,
    };
    vec![
        // head (+ the muzzle + the two horns — the same part's fluent
        // chain the bytecode's addBox walk speaks). THE VARIANT ARMS
        // the 26.2 replaces keep: the COLD arm the tuft (9, 33) the
        // horns DROPS (the cold replace's head = the base + the
        // (9, 33) 6x3x1 pair the bytecode speaks), the WARM arm the
        // BIG horns (27, 0) 4x2x2 + (39, 0) 2x2x2 pairs the mirrored
        // walk rides (the base's 1x3x1 pair the replace never names)
        Part {
            pivot: [0.0, 4.0, -8.0],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: match variant {
                crate::mob::ChickenVariantKind::Cold => vec![
                    Cube {
                        tex: (0.0, 0.0),
                        origin: [-4.0, -4.0, -6.0],
                        size: [8.0, 8.0, 6.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (9.0, 33.0),
                        origin: [-3.0, 1.0, -7.0],
                        size: [6.0, 3.0, 1.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                ],
                crate::mob::ChickenVariantKind::Warm => vec![
                    Cube {
                        tex: (0.0, 0.0),
                        origin: [-4.0, -4.0, -6.0],
                        size: [8.0, 8.0, 6.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (1.0, 33.0),
                        origin: [-3.0, 1.0, -7.0],
                        size: [6.0, 3.0, 1.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (27.0, 0.0),
                        origin: [-8.0, -3.0, -5.0],
                        size: [4.0, 2.0, 2.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (39.0, 0.0),
                        origin: [-8.0, -5.0, -5.0],
                        size: [2.0, 2.0, 2.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (27.0, 0.0),
                        origin: [4.0, -3.0, -5.0],
                        size: [4.0, 2.0, 2.0],
                        inflate: 0.0,
                        mirror: true,
                    },
                    Cube {
                        tex: (39.0, 0.0),
                        origin: [6.0, -5.0, -5.0],
                        size: [2.0, 2.0, 2.0],
                        inflate: 0.0,
                        mirror: true,
                    },
                ],
                crate::mob::ChickenVariantKind::Temperate => vec![
                    Cube {
                        tex: (0.0, 0.0),
                        origin: [-4.0, -4.0, -6.0],
                        size: [8.0, 8.0, 6.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (1.0, 33.0),
                        origin: [-3.0, 1.0, -7.0],
                        size: [6.0, 3.0, 1.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (22.0, 0.0),
                        origin: [-5.0, -5.0, -5.0],
                        size: [1.0, 3.0, 1.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (22.0, 0.0),
                        origin: [4.0, -5.0, -5.0],
                        size: [1.0, 3.0, 1.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                ],
            },
            children: vec![],
        },
        // body (the horizontal barrel — the rotX PI/2 the PartPose
        // offsetAndRotation speaks) + the udder (the same part's
        // second cube). THE COLD ARM the ColdCowModel's own replace
        // keeps: the thick hide coat (texOffs (20, 32) — the SAME
        // 12x18x10 box the CubeDeformation(0.5) inflate the bytecode's
        // addBox speaks) rides the base barrel
        Part {
            pivot: [0.0, 5.0, 2.0],
            x_rot: std::f32::consts::FRAC_PI_2,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: if variant == crate::mob::ChickenVariantKind::Cold {
                vec![
                    Cube {
                        tex: (20.0, 32.0),
                        origin: [-6.0, -10.0, -7.0],
                        size: [12.0, 18.0, 10.0],
                        inflate: 0.5,
                        mirror: false,
                    },
                    Cube {
                        tex: (18.0, 4.0),
                        origin: [-6.0, -10.0, -7.0],
                        size: [12.0, 18.0, 10.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (52.0, 0.0),
                        origin: [-2.0, 2.0, -8.0],
                        size: [4.0, 6.0, 1.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                ]
            } else {
                vec![
                    Cube {
                        tex: (18.0, 4.0),
                        origin: [-6.0, -10.0, -7.0],
                        size: [12.0, 18.0, 10.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (52.0, 0.0),
                        origin: [-2.0, 2.0, -8.0],
                        size: [4.0, 6.0, 1.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                ]
            },
            children: vec![],
        },
        // right hind leg (the plain builder's own unwrap — no mirror)
        Part {
            pivot: [-4.0, 12.0, 7.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(false)],
            children: vec![],
        },
        // left hind leg (the mirrored builder — the u axis flips)
        Part {
            pivot: [4.0, 12.0, 7.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(true)],
            children: vec![],
        },
        // right front leg (anti-phase of the right hind — the diagonal
        // pair the setupAnim's own cos fold speaks)
        Part {
            pivot: [-4.0, 12.0, -5.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(false)],
            children: vec![],
        },
        // left front leg (in phase with the right hind)
        Part {
            pivot: [4.0, 12.0, -5.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(true)],
            children: vec![],
        },
    ]
}

/// THE OVINE POSE INPUTS — the quadruped's shared store PLUS the eat
/// fold's own pair (javap `SheepModel.setupAnim(SheepRenderState)`, the
/// dump's own two-field word: `head.y += headEatPositionScale * 9.0 *
/// ageScale` then `head.xRot = headEatAngleScale` — the eat angle
/// REPLACES the head pitch the quadruped fold spoke, the server's
/// Sheep.getHeadEatAngleScale the renderer bakes verbatim in radians).
/// The cow carried no override — the sheep is the first body whose own
/// setupAnim the lane rides beyond the shared diagonal walk. The wire's
/// pitch word is NOT an input: the sheep model's own putfield the
/// replacement speaks leaves the look-pitch unrendered (the honest
/// reading of the bytecode — the field the struct never carried).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SheepState {
    /// vanilla yaw degrees (the body's own)
    pub yaw_deg: f32,
    /// the head's yaw offset from the body (the netHeadYaw clamp — the
    /// yRot the quadruped fold's own state read speaks)
    pub head_yaw_off_deg: f32,
    /// walkAnimationPos (the backward-extrapolated walk phase)
    pub walk_pos: f32,
    /// walkAnimationSpeed (the min(abs, 1) blend)
    pub walk_speed: f32,
    /// ageInTicks (the idle owns no bob — carried for parity)
    pub age_ticks: f32,
    /// `LivingEntityRenderState.deathTime` (the same flip law's clock
    /// the avatar fold speaks — one chain, every living model)
    pub death_time: f32,
    /// `SheepRenderState.headEatPositionScale` (the graze's head-drop
    /// scale — the Sheep.getHeadEatPositionScale fold the mob's own
    /// timer drives, the ease bands the window speaks)
    pub head_eat_position: f32,
    /// `SheepRenderState.headEatAngleScale` (the graze's pitch the
    /// server bakes in radians — the xRot the model's own assign
    /// REPLACES the quadruped's with)
    pub head_eat_angle: f32,
    /// `SheepRenderState.ageScale` (the adult's 1.0, the baby's
    /// 0.5 — AgeableMob.getScale's own arm the eat drop divides by;
    /// the baby lane the dedicated BabySheepModel arms)
    pub age_scale: f32,
}

/// The full sheep model — the 26.2 `SheepModel.createBodyLayer`'s own
/// numbers, bytecode-proven off the official jar (scripts/classfile_dump.py:
/// the constant pool + the createBodyLayer walk): the head 6x6x8 (uv
/// 0,0) at pivot (0, 6, -8) — the ovine's own REPLACEMENT over
/// `QuadrupedModel.createBodyMesh(12, false, true, NONE)`'s 8x8x8
/// default, the body 8x16x6 (uv 28,8) rotated x=PI/2 at pivot (0, 5,
/// 2), the four 4x12x4 legs (uv 0,16) at pivots (+-3, 12, +7/-5) — the
/// createLegs(loc1=false, loc2=true) walk's own pairing the RIGHT
/// pair's mirror rides (the pig/cow LEFT-pair convention INVERTED, the
/// bytecode's own word), `LayerDefinition.create(64, 32)` — the
/// HALF-HEIGHT layout the 64x64 quadrupeds' replaced.
pub struct SheepModel {
    rgba: Vec<u8>,
}

impl SheepModel {
    /// Load + sha1-verify the SHEEP's skin (the SheepRenderer's
    /// `getTextureLocation` — the single sheep.png, no biome variants).
    /// `None` = the sheep never renders (the same honesty the pig/cow
    /// paths speak: the renderer must not lie about its data).
    pub fn load(root: &Path) -> Option<SheepModel> {
        let bytes = std::fs::read(root.join(skin::SHEEP_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::SHEEP_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 || rgba.len() != 64 * 32 * 4 {
            return None;
        }
        Some(SheepModel { rgba })
    }

    /// The skin pixel at (u, v) — the RGBA row-major words, v down
    /// (the 64x32 layout's own stride the load verified).
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the world-space textured quads of the sheep at
    /// `state` — the SAME entity chain fold the avatar speaks (one
    /// law, every living model), the ovine's own parts + the eat fold
    /// the head's pivot rides beneath.
    pub fn quads(
        &self,
        state: &SheepState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_sheep_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The ovine's parts, posed by `state` — the javap-proven mesh: the
/// head/body the createBodyLayer's own fluent chains speak, the legs
/// the shared `QuadrupedModel.createLegs` walk lays (the RIGHT pair's
/// mirror the loc2=true ride). THE WALK: the same `QuadrupedModel.setupAnim`
/// diagonal fold the pig speaks — right-hind + left-front in phase,
/// the other pair +PI (the sheep's own override only re-poses the
/// head). THE EAT (the override's own word): the head's pivot y the
/// `headEatPositionScale * 9.0 * ageScale` fold drops, the head's
/// xRot the eat angle the server bakes REPLACES the quadruped's pitch.
fn build_sheep_parts(state: &SheepState) -> Vec<Part> {
    let pos = state.walk_pos;
    let speed = state.walk_speed;
    let leg_rh = mth_cos(pos * 0.6662) * 1.4 * speed;
    let leg_lh = mth_cos(pos * 0.6662 + std::f32::consts::PI) * 1.4 * speed;
    // THE EAT FOLD (SheepModel.setupAnim's own two statements — the
    // head's y offset rides the model units the *9.0 speaks, the
    // eatScale the adult's own 1.0 divides nothing)
    let head_pivot_y = 6.0 + state.head_eat_position * 9.0 * state.age_scale;
    let head_x = state.head_eat_angle;
    let head_y = state.head_yaw_off_deg.to_radians();
    let leg_cube = |mirror: bool| Cube {
        tex: (0.0, 16.0),
        origin: [-2.0, 0.0, -2.0],
        size: [4.0, 12.0, 4.0],
        inflate: 0.0,
        mirror,
    };
    vec![
        // head (the ovine's own replacement — one cube, no snout)
        Part {
            pivot: [0.0, head_pivot_y, -8.0],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 0.0),
                origin: [-3.0, -4.0, -6.0],
                size: [6.0, 6.0, 8.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // body (the horizontal barrel — the rotX PI/2 the PartPose
        // offsetAndRotation speaks)
        Part {
            pivot: [0.0, 5.0, 2.0],
            x_rot: std::f32::consts::FRAC_PI_2,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (28.0, 8.0),
                origin: [-4.0, -10.0, -7.0],
                size: [8.0, 16.0, 6.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // right hind leg (the MIRRORED builder — the loc2=true word the
        // createLegs walk speaks, the pig/cow convention inverted)
        Part {
            pivot: [-3.0, 12.0, 7.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(true)],
            children: vec![],
        },
        // left hind leg (the plain builder)
        Part {
            pivot: [3.0, 12.0, 7.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(false)],
            children: vec![],
        },
        // right front leg (anti-phase of the right hind — the diagonal
        // pair the setupAnim's own cos fold speaks)
        Part {
            pivot: [-3.0, 12.0, -5.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(true)],
            children: vec![],
        },
        // left front leg (in phase with the right hind)
        Part {
            pivot: [3.0, 12.0, -5.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(false)],
            children: vec![],
        },
    ]
}

/// The full SHEEP WOOL model — the 26.2 `SheepFurModel.createFurLayer`'s
/// own numbers, bytecode-proven off the official jar (scripts/
/// classfile_dump.py: the constant pool + the createFurLayer walk):
/// the HEAD FUR 6x6x6 (uv 0,0, box -3,-4,-4 — SHORTER in z than the
/// body head's 6x6x8, the inflate 0.6 the tan face's own hood gap
/// speaks: the head's front z -6..-4.6 stays fur-less, the face pokes
/// out), the BODY FUR 8x16x6 (uv 28,8, box -4,-10,-7) at the inflate
/// 1.75 the wool barrel rides, the four LEG FURS 4x6x4 (uv 0,16, box
/// -2,0,-2 — the pig-height the wool boots speak, the UPPER half of
/// the 12-texel legs only) at the inflate 0.5, all four riding the ONE
/// shared CubeListBuilder the dump's own astore speaks (NO mirror —
/// the fur never unwraps), the same pivots the body speaks, `LayerDefinition.
/// create(64, 32)` over the sheep_wool.png the dye color tints.
pub struct SheepWoolModel {
    rgba: Vec<u8>,
}

impl SheepWoolModel {
    /// Load + sha1-verify the WOOL texture (the SheepWoolLayer's
    /// SHEEP_WOOL_LOCATION). `None` = the wool never renders (the same
    /// honesty every lane speaks — a failed overlay keeps only the
    /// fur unseen, the sheared bodies keep drawing).
    pub fn load(root: &Path) -> Option<SheepWoolModel> {
        let bytes = std::fs::read(root.join(skin::SHEEP_WOOL_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::SHEEP_WOOL_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 || rgba.len() != 64 * 32 * 4 {
            return None;
        }
        Some(SheepWoolModel { rgba })
    }

    /// The wool pixel at (u, v) — the RGBA row-major words, v down.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED OVERLAY: the world-space textured quads of the wool at
    /// `state` — the SAME entity chain fold the body speaks, the fur's
    /// own inflated cubes beneath. THE POSE (the fur model's own law):
    /// SheepFurModel extends QuadrupedModel but its setupAnim re-speaks
    /// the sheep's own eat fold verbatim (the dump's own two statements)
    /// — the hood follows the body head's drop and pitch exactly, the
    /// walk the shared diagonal fold swings.
    pub fn quads(
        &self,
        state: &SheepState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_sheep_fur_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 4);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The full SHEEP WOOL UNDERCOAT model — the 26.2
/// `SheepWoolUndercoatLayer`'s own lane, bytecode-proven off the
/// official jar (scripts/classfile_dump.py: the layer's <clinit> the
/// SHEEP_WOOL_UNDERCOAT_LOCATION static speaks, the submit walk the
/// gate's exact jumps keep). THE TEXTURE: `sheep_wool_undercoat.png`,
/// the greyscale the wool color tints. THE MESH: NOT a fur inflation —
/// LayerDefinitions bakes the SHEEP_WOOL_UNDERCOAT entry from
/// `SheepModel.createBodyLayer()` (the SAME body layer the base skin's
/// mesh speaks), so the undercoat hugs the body's own shape and rides
/// `build_sheep_parts` (the walk SheepModel::quads speaks) — the
/// SheepFurModel.createFurLayer inflate the wool fur keeps is a mesh
/// the undercoat never grows.
///
/// THE GATE (SheepWoolUndercoatLayer.submit's own jumps): render iff
/// `!isInvisible && !isBaby && (isJebSheep || woolColor != WHITE)` —
/// the isSheared field is NEVER read (the bytecode keeps NO isSheared
/// getfield), so a SHEARED dyed sheep keeps showing its dye color
/// through the undercoat the wool fur's removal uncovered — the lane's
/// whole gameplay word. The white sheep's undercoat the gate skips (the
/// base skin's pale look IS the white coat), the baby the isBaby arm
/// returns (the baby wool the sheep_wool_baby lane tints), the jeb_
/// sheep the isJebSheep short-circuit arms (the rainbow the
/// getWoolColor's ColorLerper arm speaks).
pub struct SheepWoolUndercoatModel {
    rgba: Vec<u8>,
}

impl SheepWoolUndercoatModel {
    /// Load + sha1-verify the UNDERCOAT texture (the
    /// SheepWoolUndercoatLayer's SHEEP_WOOL_UNDERCOAT_LOCATION).
    /// `None` = the undercoat never renders (the same honesty every
    /// lane speaks — a failed overlay keeps the body drawing).
    pub fn load(root: &Path) -> Option<SheepWoolUndercoatModel> {
        let bytes = std::fs::read(root.join(skin::SHEEP_WOOL_UNDERCOAT_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::SHEEP_WOOL_UNDERCOAT_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 || rgba.len() != 64 * 32 * 4 {
            return None;
        }
        Some(SheepWoolUndercoatModel { rgba })
    }

    /// The undercoat pixel at (u, v) — the RGBA row-major words, v down
    /// (the 64x32 layout the body layer bakes, the load verified).
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED OVERLAY: the world-space textured quads of the
    /// undercoat at `state` — the BODY mesh's own walk (the
    /// `build_sheep_parts` chain SheepModel::quads speaks: the same
    /// parts, the same entity folds, the same collect_part — one pose
    /// law, body and undercoat), so the undercoat's quads EXACTLY
    /// overlay the body's own quads the base skin draws beneath.
    pub fn quads(
        &self,
        state: &SheepState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_sheep_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The ovine's fur parts, posed by `state` — the bytecode-proven mesh
/// (the head fur's own z-shorter box, the body fur's 1.75 inflate, the
/// leg furs' shared no-mirror builder), the SAME pose law the body
/// speaks (the walk's diagonal pairs + the eat fold the setupAnim's
/// own copy rides).
fn build_sheep_fur_parts(state: &SheepState) -> Vec<Part> {
    let pos = state.walk_pos;
    let speed = state.walk_speed;
    let leg_rh = mth_cos(pos * 0.6662) * 1.4 * speed;
    let leg_lh = mth_cos(pos * 0.6662 + std::f32::consts::PI) * 1.4 * speed;
    // THE EAT FOLD (SheepFurModel.setupAnim's own copy — the hood
    // follows the body head the same two statements speak)
    let head_pivot_y = 6.0 + state.head_eat_position * 9.0 * state.age_scale;
    let head_x = state.head_eat_angle;
    let head_y = state.head_yaw_off_deg.to_radians();
    // the ONE shared builder the dump's astore speaks: one Cube spec,
    // four pivots, no mirror
    let fur_leg = Cube {
        tex: (0.0, 16.0),
        origin: [-2.0, 0.0, -2.0],
        size: [4.0, 6.0, 4.0],
        inflate: 0.5,
        mirror: false,
    };
    vec![
        // head fur (the z-shorter hood — the face gap the inflate math
        // speaks: fur z -4.6..2.6 vs head z -6..2)
        Part {
            pivot: [0.0, head_pivot_y, -8.0],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 0.0),
                origin: [-3.0, -4.0, -4.0],
                size: [6.0, 6.0, 6.0],
                inflate: 0.6,
                mirror: false,
            }],
            children: vec![],
        },
        // body fur (the wool barrel — the rotX PI/2 the body's own
        // PartPose rides, the 1.75 inflate the coat's bulk speaks)
        Part {
            pivot: [0.0, 5.0, 2.0],
            x_rot: std::f32::consts::FRAC_PI_2,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (28.0, 8.0),
                origin: [-4.0, -10.0, -7.0],
                size: [8.0, 16.0, 6.0],
                inflate: 1.75,
                mirror: false,
            }],
            children: vec![],
        },
        // right hind fur (the shared builder — no mirror)
        Part {
            pivot: [-3.0, 12.0, 7.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![fur_leg],
            children: vec![],
        },
        // left hind fur
        Part {
            pivot: [3.0, 12.0, 7.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![fur_leg],
            children: vec![],
        },
        // right front fur
        Part {
            pivot: [-3.0, 12.0, -5.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![fur_leg],
            children: vec![],
        },
        // left front fur
        Part {
            pivot: [3.0, 12.0, -5.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![fur_leg],
            children: vec![],
        },
    ]
}

/// The full BABY SHEEP model — the 26.2 `BabySheepModel.createBodyLayer`'s
/// own numbers, bytecode-proven off the official jar (scripts/
/// classfile_dump.py + scripts/babysheepmodel.txt: the constant pool +
/// the createBodyLayer walk): the dedicated BABY GEOMETRY the
/// AgeableMobRenderer's own babyModel field speaks — NOT a scaled adult
/// (javap SheepRenderer: `babyModel = BabySheepModel(bakeLayer(SHEEP_BABY))`,
/// the isBaby word picks it, the render pose's scale fold the dedicated
/// layer REPLACES): the head 5x5x5 (uv 0,0) at pivot (0, 15.5, -2.5) —
/// the PROPORTIONALLY BIGGER head the baby look speaks (5/16 box on a
/// 13/16-tall body vs the adult's 6x6x8 on 21/16), the body 6x4x9 (uv
/// 0,10) at pivot (0, 17, 0.5) with NO x-rotation (the horizontal box
/// the adult's rotX PI/2 barrel never needed), the four 2x5x2 legs (uv
/// 0,23 / 24,12 / 8,23 / 24,5 — FOUR distinct rects the adult's ONE
/// shared 0,16 builder never spoke, all four NO mirror) at pivots
/// (+-2, 19, 3/-2), `LayerDefinition.create(64, 32)` over the
/// sheep_baby.png the baby lane's own texture. BabySheepModel extends
/// SheepModel with ONLY the layer overridden — the setupAnim walk/eat
/// laws the adult's own copy rides verbatim.
pub struct BabySheepModel {
    rgba: Vec<u8>,
}

impl BabySheepModel {
    /// Load + sha1-verify the BABY sheep's skin (the SheepRenderer's
    /// own `sheep_baby.png` — the dedicated baby lane's texture).
    /// `None` = the baby never renders (the same honesty the adult
    /// lanes speak — the renderer must not lie about its data).
    pub fn load(root: &Path) -> Option<BabySheepModel> {
        let bytes = std::fs::read(root.join(skin::SHEEP_BABY_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::SHEEP_BABY_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 || rgba.len() != 64 * 32 * 4 {
            return None;
        }
        Some(BabySheepModel { rgba })
    }

    /// The baby skin pixel at (u, v) — the RGBA row-major words, v down
    /// (the 64x32 layout's own stride the load verified).
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the world-space textured quads of the baby at
    /// `state` — the SAME entity chain fold the adult speaks (one law,
    /// every living model, babies included — the death flip and the
    /// walk lerp never aged).
    pub fn quads(
        &self,
        state: &SheepState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_baby_sheep_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The full BABY SHEEP WOOL model — the SheepWoolLayer's baby arm's own
/// overlay (javap SheepWoolLayer: `babyModel = SheepFurModel(bakeLayer
/// (SHEEP_BABY_WOOL))`, and javap LayerDefinitions: the SHEEP_BABY_WOOL
/// layer bakes `BabySheepModel.createBodyLayer()` — the SAME loc49 the
/// SHEEP_BABY layer rides, the baby wool layer's own geometry IS the
/// baby body's) — the boxes the baby body speaks, textured with the
/// sheep_wool_baby.png the SHEEP_WOOL_BABY_LOCATION names, the dye
/// color tint the same getWoolColor word multiplies.
pub struct BabySheepWoolModel {
    rgba: Vec<u8>,
}

impl BabySheepWoolModel {
    /// Load + sha1-verify the BABY wool texture (the SheepWoolLayer's
    /// `sheep_wool_baby.png`). `None` = the baby's wool never renders
    /// (the same honesty every overlay lane speaks).
    pub fn load(root: &Path) -> Option<BabySheepWoolModel> {
        let bytes = std::fs::read(root.join(skin::SHEEP_WOOL_BABY_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::SHEEP_WOOL_BABY_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 || rgba.len() != 64 * 32 * 4 {
            return None;
        }
        Some(BabySheepWoolModel { rgba })
    }

    /// The baby wool pixel at (u, v) — the RGBA row-major words, v down.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED OVERLAY: the baby's wool quads — the baby body's own
    /// builder the SHEEP_BABY_WOOL layer bakes, the tint the caller
    /// carries (the same law the adult's overlay speaks).
    pub fn quads(
        &self,
        state: &SheepState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_baby_sheep_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The full BABY COW model — the 26.2 `BabyCowModel.createBodyLayer`'s
/// own numbers, bytecode-proven off the official jar (scripts/
/// babycowmodel.txt): the head 6x6x5 (uv 0,18) at pivot (0, 13.569,
/// -5.1667) with the TWO HORN cubes (right uv 8,29 at (3, -5.569,
/// -3.8333), LEFT uv 4,29 the builder's own .mirror() ride at (-4,
/// -5.569, -3.8333)) and the muzzle cube (uv 12,29 at (-2, -1.569,
/// -5.8333)) — the calf's horns the adult's variant table never
/// dropped, the body 8x6x12 (uv 0,0) at pivot (3, 19, -5) with NO
/// x-rotation (the horizontal box the adult's rotX PI/2 barrel never
/// needed), the four 3x6x3 legs (uv 22,18 / 34,18 / 22,27 / 34,27 —
/// four distinct rects, no mirror) at pivots (+-2.5, 18, -3.5/3.5),
/// `LayerDefinition.create(64, 64)` over the cow_temperate_baby.png.
/// BabyCowModel extends CowModel with only the layer overridden — the
/// QuadrupedModel.setupAnim walk the adult's fold rides verbatim.
pub struct BabyCowModel {
    rgba: Vec<u8>,
}

impl BabyCowModel {
    /// Load + sha1-verify the BABY cow's skin (the CowRenderer's
    /// temperate variant table's `_baby` entry). `None` = the baby
    /// never renders (the same honesty the adult lane speaks).
    pub fn load(root: &Path) -> Option<BabyCowModel> {
        Self::load_skin(
            root,
            skin::COW_TEMPERATE_BABY_RUNTIME_PATH,
            skin::COW_TEMPERATE_BABY_PNG_SHA1,
        )
    }

    /// THE COLD VARIANT'S OWN BABY SKIN (cold.json's baby_asset_id —
    /// the same 64x64 calf geometry the cold texture wears). `None` =
    /// the cold calves stay unseen.
    pub fn load_cold(root: &Path) -> Option<BabyCowModel> {
        Self::load_skin(
            root,
            skin::COW_COLD_BABY_RUNTIME_PATH,
            skin::COW_COLD_BABY_PNG_SHA1,
        )
    }

    /// THE WARM VARIANT'S OWN BABY SKIN (warm.json's baby_asset_id).
    /// `None` = the warm calves stay unseen.
    pub fn load_warm(root: &Path) -> Option<BabyCowModel> {
        Self::load_skin(
            root,
            skin::COW_WARM_BABY_RUNTIME_PATH,
            skin::COW_WARM_BABY_PNG_SHA1,
        )
    }

    /// The variant skin's own load walk (the path + sha1 pair the
    /// caller names, the 64x64 layout the decode verifies).
    fn load_skin(root: &Path, path: &str, sha1_hex: &str) -> Option<BabyCowModel> {
        let bytes = std::fs::read(root.join(path)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != sha1_hex {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 64 || rgba.len() != 64 * 64 * 4 {
            return None;
        }
        Some(BabyCowModel { rgba })
    }

    /// The baby skin pixel at (u, v) — the RGBA row-major words, v down.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 63.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the baby cow's quads — the same entity chain
    /// fold, the same walk store the adult speaks.
    pub fn quads(
        &self,
        state: &QuadrupedState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_baby_cow_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 7);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The full BABY PIG model — the 26.2 `BabyPigModel.createBodyLayer`'s
/// own numbers, bytecode-proven off the official jar (scripts/
/// babypigmodel.txt): the body 7x6x9 (uv 0,0) at pivot (0, 19, 0.5)
/// with NO x-rotation, the head 7x6x6 (uv 0,15, inflate 0.025) at
/// pivot (0, 19, -2) with the snout cube (uv 6,27, inflate 0.015, at
/// (-1.5, -1.975, -6)) — the inflate deformations the adult's NONE
/// never spoke, the four STUB legs 2x2x2 (uv 0,0 / 23,0 / 0,4 / 23,4 —
/// four distinct rects, no mirror) at pivots (+-2.5, 22, -3/4),
/// `LayerDefinition.create(32, 32)` over the pig_temperate_baby.png —
/// the baby's OWN half-size texture the adult's 64x64 never shared.
/// BabyPigModel extends PigModel with only the layer overridden — the
/// QuadrupedModel.setupAnim walk the adult's fold rides verbatim.
pub struct BabyPigModel {
    rgba: Vec<u8>,
}

impl BabyPigModel {
    /// Load + sha1-verify the BABY pig's skin (the PigRenderer's
    /// temperate variant table's `_baby` entry). `None` = the baby
    /// never renders (the same honesty the adult lane speaks).
    pub fn load(root: &Path) -> Option<BabyPigModel> {
        Self::load_skin(
            root,
            skin::PIG_TEMPERATE_BABY_RUNTIME_PATH,
            skin::PIG_TEMPERATE_BABY_PNG_SHA1,
        )
    }

    /// THE COLD VARIANT'S OWN BABY SKIN (cold.json's baby_asset_id —
    /// the SAME 32x32 chick geometry the cold texture wears).
    /// `None` = the cold piglets stay unseen.
    pub fn load_cold(root: &Path) -> Option<BabyPigModel> {
        Self::load_skin(
            root,
            skin::PIG_COLD_BABY_RUNTIME_PATH,
            skin::PIG_COLD_BABY_PNG_SHA1,
        )
    }

    /// THE WARM VARIANT'S OWN BABY SKIN (warm.json's baby_asset_id).
    /// `None` = the warm piglets stay unseen.
    pub fn load_warm(root: &Path) -> Option<BabyPigModel> {
        Self::load_skin(
            root,
            skin::PIG_WARM_BABY_RUNTIME_PATH,
            skin::PIG_WARM_BABY_PNG_SHA1,
        )
    }

    /// The variant skin's own load walk (the path + sha1 pair the
    /// caller names, the 32x32 layout the decode verifies).
    fn load_skin(root: &Path, path: &str, sha1_hex: &str) -> Option<BabyPigModel> {
        let bytes = std::fs::read(root.join(path)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != sha1_hex {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 32 || h != 32 || rgba.len() != 32 * 32 * 4 {
            return None;
        }
        Some(BabyPigModel { rgba })
    }

    /// The baby skin pixel at (u, v) — the RGBA row-major words, v down
    /// (the 32x32 layout's own stride the load verified).
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 31.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 32 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the baby pig's quads — the same entity chain
    /// fold, the same walk store the adult speaks.
    pub fn quads(
        &self,
        state: &PigState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_baby_pig_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The ovine BABY's parts, posed by `state` — the BabySheepModel's own
/// bytecode-proven mesh (the 5x5x5 head, the horizontal 6x4x9 body, the
/// four 2x5x2 legs on their FOUR distinct rects), the SAME pose law the
/// adult speaks (the walk's diagonal pairs + the eat fold —
/// BabySheepModel inherits SheepModel's setupAnim verbatim; the eat
/// drop's base rides the baby's own 15.5 pivot the layer speaks).
fn build_baby_sheep_parts(state: &SheepState) -> Vec<Part> {
    let pos = state.walk_pos;
    let speed = state.walk_speed;
    let leg_rh = mth_cos(pos * 0.6662) * 1.4 * speed;
    let leg_lh = mth_cos(pos * 0.6662 + std::f32::consts::PI) * 1.4 * speed;
    // THE EAT FOLD (the inherited setupAnim's own two statements over
    // the baby's own pivot — the *9.0*ageScale drop the adult speaks)
    let head_pivot_y = 15.5 + state.head_eat_position * 9.0 * state.age_scale;
    let head_x = state.head_eat_angle;
    let head_y = state.head_yaw_off_deg.to_radians();
    let leg_cube = |tex: (f32, f32)| Cube {
        tex,
        origin: [-1.0, 0.0, -1.0],
        size: [2.0, 5.0, 2.0],
        inflate: 0.0,
        mirror: false,
    };
    vec![
        // head (the baby's own 5x5x5 — the big-headed look the
        // dedicated layer speaks)
        Part {
            pivot: [0.0, head_pivot_y, -2.5],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 0.0),
                origin: [-2.5, -4.5, -3.5],
                size: [5.0, 5.0, 5.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // body (the horizontal box — NO rotX the adult's barrel rides)
        Part {
            pivot: [0.0, 17.0, 0.5],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 10.0),
                origin: [-3.0, -2.0, -4.5],
                size: [6.0, 4.0, 9.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // right hind leg (uv 0,23 — no mirror the dump speaks)
        Part {
            pivot: [-2.0, 19.0, 3.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((0.0, 23.0))],
            children: vec![],
        },
        // left hind leg (uv 24,12)
        Part {
            pivot: [2.0, 19.0, 3.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((24.0, 12.0))],
            children: vec![],
        },
        // right front leg (uv 8,23)
        Part {
            pivot: [-2.0, 19.0, -2.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((8.0, 23.0))],
            children: vec![],
        },
        // left front leg (uv 24,5)
        Part {
            pivot: [2.0, 19.0, -2.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((24.0, 5.0))],
            children: vec![],
        },
    ]
}

/// The bovine BABY's parts, posed by `state` — the BabyCowModel's own
/// bytecode-proven mesh (the four-cube head with the mirrored left
/// horn, the horizontal 8x6x12 body at the +3 x pivot the layer
/// speaks, the four 3x6x3 legs on four distinct rects), the SAME pose
/// law the adult speaks (the walk's diagonal pairs — CowModel carries
/// no setupAnim override, the QuadrupedModel base rides verbatim).
fn build_baby_cow_parts(state: &QuadrupedState) -> Vec<Part> {
    let pos = state.walk_pos;
    let speed = state.walk_speed;
    let leg_rh = mth_cos(pos * 0.6662) * 1.4 * speed;
    let leg_lh = mth_cos(pos * 0.6662 + std::f32::consts::PI) * 1.4 * speed;
    let head_x = state.pitch_deg.to_radians();
    let head_y = state.head_yaw_off_deg.to_radians();
    let leg_cube = |tex: (f32, f32)| Cube {
        tex,
        origin: [-1.5, 0.0, -1.5],
        size: [3.0, 6.0, 3.0],
        inflate: 0.0,
        mirror: false,
    };
    vec![
        // head (the four cubes the fluent chain speaks — main, right
        // horn, LEFT horn mirrored, muzzle)
        Part {
            pivot: [0.0, 13.569, -5.1667],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![
                Cube {
                    tex: (0.0, 18.0),
                    origin: [-3.0, -4.569, -4.8333],
                    size: [6.0, 6.0, 5.0],
                    inflate: 0.0,
                    mirror: false,
                },
                Cube {
                    tex: (8.0, 29.0),
                    origin: [3.0, -5.569, -3.8333],
                    size: [1.0, 2.0, 1.0],
                    inflate: 0.0,
                    mirror: false,
                },
                Cube {
                    tex: (4.0, 29.0),
                    origin: [-4.0, -5.569, -3.8333],
                    size: [1.0, 2.0, 1.0],
                    inflate: 0.0,
                    mirror: true,
                },
                Cube {
                    tex: (12.0, 29.0),
                    origin: [-2.0, -1.569, -5.8333],
                    size: [4.0, 3.0, 1.0],
                    inflate: 0.0,
                    mirror: false,
                },
            ],
            children: vec![],
        },
        // body (the horizontal box at the +3 x pivot — NO rotX)
        Part {
            pivot: [3.0, 19.0, -5.0],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 0.0),
                origin: [-7.0, -7.0, -1.0],
                size: [8.0, 6.0, 12.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // right front leg (uv 22,18)
        Part {
            pivot: [-2.5, 18.0, -3.5],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((22.0, 18.0))],
            children: vec![],
        },
        // left front leg (uv 34,18)
        Part {
            pivot: [2.5, 18.0, -3.5],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((34.0, 18.0))],
            children: vec![],
        },
        // right hind leg (uv 22,27)
        Part {
            pivot: [-2.5, 18.0, 3.5],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((22.0, 27.0))],
            children: vec![],
        },
        // left hind leg (uv 34,27)
        Part {
            pivot: [2.5, 18.0, 3.5],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((34.0, 27.0))],
            children: vec![],
        },
    ]
}

// ===========================================================================
// THE CHICKEN (the fifth drawn body — the avian's own chain)
// ===========================================================================

/// The AVIAN's pose inputs — the 26.2 `ChickenModel.setupAnim`'s own
/// field list: the walk fold the legs read, the flap pair the wings'
/// zRot folds (`(sin(flap)+1)*flapSpeed` the flapAngle the bytecode's
/// f2d/sin/fadd/fmul walk keeps), and NO head arm — the setupAnim the
/// classfile keeps speaks legs and wings ONLY (the head the PartPose
/// holds, the look the whole-body yaw carries; the QuadrupedModel's
/// own head fold the pig/cow/sheep ride the chicken's class never
/// inherited — the javap dump the honest pose keeps). The baby model
/// inherits the same setupAnim (BabyChickenModel extends
/// ChickenModel), the same state drives both.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChickenState {
    /// vanilla yaw degrees (the body's own)
    pub yaw_deg: f32,
    /// walkAnimationPos (the backward-extrapolated walk phase)
    pub walk_pos: f32,
    /// walkAnimationSpeed (the min(abs, 1) blend)
    pub walk_speed: f32,
    /// `ChickenRenderState.flap` — the wing clock's sin source
    pub flap: f32,
    /// `ChickenRenderState.flapSpeed` — the wing clock's speed
    pub flap_speed: f32,
    /// ageInTicks (the chicken's idle owns no bob — carried for parity)
    pub age_ticks: f32,
    /// `LivingEntityRenderState.deathTime` (the same flip law's clock
    /// the avatar fold speaks — one chain, every living model)
    pub death_time: f32,
}

/// The full ADULT chicken model — the 26.2
/// `AdultChickenModel.createBaseChickenModel`'s own numbers,
/// bytecode-proven off the official jar: the head 4x6x3 at pivot
/// (0, 15, -4) with the beak 4x2x2 (uv 14,0) and the red_thing 2x2x2
/// (uv 14,4) ZERO-pose children the head's own cube list folds (the
/// same fold the pig's snout rides), the rotX PI/2 barrel body 6x8x6
/// (uv 0,9), the 3x5x3 legs (uv 26,0) at pivots (-2, 19, 1) /
/// (1, 19, 1), the 1x4x6 wings (uv 24,13) at pivots (∓4, 13, 0),
/// `LayerDefinition.create(mesh, 64, 32)`.
pub struct ChickenModel {
    rgba: Vec<u8>,
}

impl ChickenModel {
    /// Load + sha1-verify the CHICKEN's temperate skin (the
    /// ChickenRenderer's variant table's default — the plain hen). The
    /// `None` = the chicken never renders (the same honesty the
    /// pig/cow/sheep paths speak: the renderer must not lie about its
    /// data).
    pub fn load(root: &Path) -> Option<ChickenModel> {
        Self::load_skin(
            root,
            skin::CHICKEN_TEMPERATE_RUNTIME_PATH,
            skin::CHICKEN_TEMPERATE_PNG_SHA1,
        )
    }

    /// THE WARM VARIANT'S OWN SKIN (the warm.json asset_id the
    /// ChickenRenderer's pair picks — the plain adult GEOMETRY the
    /// warm texture wears, the warm.json's model field the normal
    /// arm keeps): the same load walk, the variant table's own
    /// sha1-verdict. `None` = the warm hens stay unseen.
    pub fn load_warm(root: &Path) -> Option<ChickenModel> {
        Self::load_skin(
            root,
            skin::CHICKEN_WARM_RUNTIME_PATH,
            skin::CHICKEN_WARM_PNG_SHA1,
        )
    }

    /// The variant skin's own load walk (the path + sha1 pair the
    /// caller names, the 64x32 layout the decode verifies).
    fn load_skin(root: &Path, path: &str, sha1_hex: &str) -> Option<ChickenModel> {
        let bytes = std::fs::read(root.join(path)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != sha1_hex {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 || rgba.len() != 64 * 32 * 4 {
            return None;
        }
        Some(ChickenModel { rgba })
    }

    /// The skin pixel at (u, v) — the RGBA row-major words, v down
    /// (the 64x32 layout's own stride the load verified).
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the world-space textured quads of the hen at
    /// `state` — the SAME entity chain fold the avatar speaks (one
    /// law, every living model), the avian's own parts beneath.
    pub fn quads(
        &self,
        state: &ChickenState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_chicken_parts(state, false);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The avian's parts, posed by `state` — the javap-proven mesh: the
/// head the PartPose holds (NO setupAnim arm the chicken's class
/// speaks), the body the rotX PI/2 barrel lays, the legs the shared
/// 0.6662/1.4 law swings (the ChickenModel.setupAnim's own
/// cos(walkAnimationPos * 0.6662) * 1.4 * walkAnimationSpeed the
/// bytecode keeps — BOTH legs the +PI mirror rides, the quadruped's
/// diagonal pair the bird's two-leg walk replaces), and the wings the
/// flap angle MIRRORS (right +zRot, left -zRot — the putfield pair
/// the setupAnim's own tail speaks).
fn build_chicken_parts(state: &ChickenState, cold: bool) -> Vec<Part> {
    let pos = state.walk_pos;
    let speed = state.walk_speed;
    // THE FLAP ANGLE (ChickenModel.setupAnim's own head: `flapAngle =
    // (Mth.sin(flap) + 1.0F) * flapSpeed` — the sin the f2d the
    // Mth.sin(double) call keeps, the +1.0 the fadd folds, the speed
    // the fmul speaks)
    let flap_angle = (state.flap.sin() + 1.0) * state.flap_speed;
    let leg_r = mth_cos(pos * 0.6662) * 1.4 * speed;
    let leg_l = mth_cos(pos * 0.6662 + std::f32::consts::PI) * 1.4 * speed;
    let leg_cube = || Cube {
        tex: (26.0, 0.0),
        origin: [-1.0, 0.0, -3.0],
        size: [3.0, 5.0, 3.0],
        inflate: 0.0,
        mirror: false,
    };
    let wing_cube = |mirror_x: f32| Cube {
        tex: (24.0, 13.0),
        origin: [mirror_x, 0.0, -3.0],
        size: [1.0, 4.0, 6.0],
        inflate: 0.0,
        mirror: false,
    };
    vec![
        // head (+ the beak and the red_thing — the ZERO-pose children
        // the head's own cube list folds, the same fold the pig's
        // snout rides; NO x_rot/y_rot — the setupAnim the 26.2
        // ChickenModel keeps speaks no head arm). THE COLD ARM the
        // ColdChickenModel.createBodyLayer's own replace keeps: the
        // fluffy face plate (texOffs 44,0 — the 6x3x4 box at
        // (-3, -7, -2.015) the bytecode's addBox speaks) REPLACES the
        // beak and the red_thing the cold pair never names
        Part {
            pivot: [0.0, 15.0, -4.0],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: if cold {
                vec![
                    Cube {
                        tex: (0.0, 0.0),
                        origin: [-2.0, -6.0, -2.0],
                        size: [4.0, 6.0, 3.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (44.0, 0.0),
                        origin: [-3.0, -7.0, -2.015000104904175],
                        size: [6.0, 3.0, 4.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                ]
            } else {
                vec![
                    Cube {
                        tex: (0.0, 0.0),
                        origin: [-2.0, -6.0, -2.0],
                        size: [4.0, 6.0, 3.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (14.0, 0.0),
                        origin: [-2.0, -4.0, -4.0],
                        size: [4.0, 2.0, 2.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (14.0, 4.0),
                        origin: [-1.0, -2.0, -3.0],
                        size: [2.0, 2.0, 2.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                ]
            },
            children: vec![],
        },
        // body (the rotX PI/2 barrel — the PartPose.offsetAndRotation
        // the bytecode keeps: offset(0, 16, 0), xRot 1.5707964). THE
        // COLD ARM the cold layer's own replace keeps: the flat tail
        // plate (texOffs 38,9 — the 0-wide 0x3x5 box at (0, 3, -1)
        // the bytecode's addBox speaks) rides the same barrel
        Part {
            pivot: [0.0, 16.0, 0.0],
            x_rot: std::f32::consts::FRAC_PI_2,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: if cold {
                vec![
                    Cube {
                        tex: (0.0, 9.0),
                        origin: [-3.0, -4.0, -3.0],
                        size: [6.0, 8.0, 6.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    Cube {
                        tex: (38.0, 9.0),
                        origin: [0.0, 3.0, -1.0],
                        size: [0.0, 3.0, 5.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                ]
            } else {
                vec![Cube {
                    tex: (0.0, 9.0),
                    origin: [-3.0, -4.0, -3.0],
                    size: [6.0, 8.0, 6.0],
                    inflate: 0.0,
                    mirror: false,
                }]
            },
            children: vec![],
        },
        // right leg (the shared 0.6662/1.4 swing)
        Part {
            pivot: [-2.0, 19.0, 1.0],
            x_rot: leg_r,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube()],
            children: vec![],
        },
        // left leg (the +PI mirror)
        Part {
            pivot: [1.0, 19.0, 1.0],
            x_rot: leg_l,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube()],
            children: vec![],
        },
        // right wing (the +flap zRot — the setupAnim's putfield)
        Part {
            pivot: [-4.0, 13.0, 0.0],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: flap_angle,
            cubes: vec![wing_cube(0.0)],
            children: vec![],
        },
        // left wing (the NEGATED zRot — the fneg the bytecode keeps)
        Part {
            pivot: [4.0, 13.0, 0.0],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: -flap_angle,
            cubes: vec![wing_cube(-1.0)],
            children: vec![],
        },
    ]
}

/// THE COLD CHICKEN'S OWN MODEL — the 26.2 `ColdChickenModel
/// .createBodyLayer`'s own replace walk, bytecode-proven: the cold
/// layer builds the base mesh (`createBaseChickenModel` the shared
/// walk keeps) then REPLACES the body and the head — the body gains
/// the flat tail plate (texOffs (38, 9), the 0-wide 0x3x5 box at
/// (0, 3, -1) the addBox speaks) and the head gains the fluffy face
/// plate (texOffs (44, 0), the 6x3x4 box at (-3, -7, -2.015) the
/// addBox speaks) INSTEAD of the beak and the red_thing the cold
/// pair's cube list never names; `LayerDefinition.create(mesh, 64,
/// 32)` the same half-height layout keeps. The setupAnim the
/// AdultChickenModel parent inherits rides UNCHANGED (the same leg
/// swing + wing flap the base folds).
pub struct ChickenColdModel {
    rgba: Vec<u8>,
}

impl ChickenColdModel {
    /// Load + sha1-verify the CHICKEN's cold skin (cold.json's
    /// asset_id — the variant table's own pair). The `None` = the
    /// cold hens stay unseen (the same honesty the other variants
    /// keep).
    pub fn load(root: &Path) -> Option<ChickenColdModel> {
        let bytes =
            std::fs::read(root.join(skin::CHICKEN_COLD_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::CHICKEN_COLD_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 || rgba.len() != 64 * 32 * 4 {
            return None;
        }
        Some(ChickenColdModel { rgba })
    }

    /// The skin pixel at (u, v) — the same 64x32 layout's own stride.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the cold hen's own parts (the replace walk
    /// the builder's cold arm speaks) beneath the same chain fold.
    pub fn quads(
        &self,
        state: &ChickenState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_chicken_parts(state, true);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The full BABY chicken model — the 26.2 `BabyChickenModel.
/// createBodyLayer`'s own numbers, bytecode-proven: the TWO-cube body
/// (uv 0,0 4x4x4 over uv 10,8 2x1x1 — the head-less chick the layer
/// keeps, the body cube the whole silhouette draws) at pivot
/// (0, 20.25, -1.25), the flat-plate legs (the 1x2x0 shin over the
/// 1x0x1 foot the two texOffs speak) at pivots (∓1, 22, 0.5), the
/// flat 1x0x2 wings at pivots (∓2, 20, 0), `LayerDefinition.create(
/// mesh, 16, 16)` — the SQUARE 16x16 layout the smallest baby lane
/// keeps. The setupAnim the ChickenModel parent speaks rides
/// UNCHANGED (the same leg swing + wing flap the adult folds).
pub struct BabyChickenModel {
    rgba: Vec<u8>,
}

impl BabyChickenModel {
    /// Load + sha1-verify the CHICKEN's baby skin (the
    /// ChickenRenderer's AdultAndBabyModelPair's baby arm). The `None`
    /// = the chick never renders (the same honesty the other baby
    /// lanes speak).
    pub fn load(root: &Path) -> Option<BabyChickenModel> {
        Self::load_skin(
            root,
            skin::CHICKEN_TEMPERATE_BABY_RUNTIME_PATH,
            skin::CHICKEN_TEMPERATE_BABY_PNG_SHA1,
        )
    }

    /// THE COLD VARIANT'S OWN BABY SKIN (cold.json's baby_asset_id —
    /// the SAME 16x16 chick geometry the cold texture wears; the
    /// cold.json's model field names the ADULT geometry only). `None`
    /// = the cold chicks stay unseen.
    pub fn load_cold(root: &Path) -> Option<BabyChickenModel> {
        Self::load_skin(
            root,
            skin::CHICKEN_COLD_BABY_RUNTIME_PATH,
            skin::CHICKEN_COLD_BABY_PNG_SHA1,
        )
    }

    /// THE WARM VARIANT'S OWN BABY SKIN (warm.json's baby_asset_id —
    /// the same 16x16 chick geometry the warm texture wears). `None`
    /// = the warm chicks stay unseen.
    pub fn load_warm(root: &Path) -> Option<BabyChickenModel> {
        Self::load_skin(
            root,
            skin::CHICKEN_WARM_BABY_RUNTIME_PATH,
            skin::CHICKEN_WARM_BABY_PNG_SHA1,
        )
    }

    /// The variant skin's own load walk (the path + sha1 pair the
    /// caller names, the 16x16 layout the decode verifies).
    fn load_skin(root: &Path, path: &str, sha1_hex: &str) -> Option<BabyChickenModel> {
        let bytes = std::fs::read(root.join(path)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != sha1_hex {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 16 || h != 16 || rgba.len() != 16 * 16 * 4 {
            return None;
        }
        Some(BabyChickenModel { rgba })
    }

    /// The skin pixel at (u, v) — the 16x16 layout's own stride.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 15.999_9), v.clamp(0.0, 15.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 16 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the chick's own parts beneath the same chain
    /// fold (the inherited setupAnim the parent's law rides).
    pub fn quads(
        &self,
        state: &ChickenState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_baby_chicken_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// The chick's parts — the SAME pose law the adult speaks (the
/// inherited setupAnim: legs 0.6662/1.4, wings ±flap angle), the
/// baby's own flat-plate geometry beneath.
fn build_baby_chicken_parts(state: &ChickenState) -> Vec<Part> {
    let pos = state.walk_pos;
    let speed = state.walk_speed;
    let flap_angle = (state.flap.sin() + 1.0) * state.flap_speed;
    let leg_r = mth_cos(pos * 0.6662) * 1.4 * speed;
    let leg_l = mth_cos(pos * 0.6662 + std::f32::consts::PI) * 1.4 * speed;
    vec![
        // body (the two-cube chick — the 4x4x4 barrel over the 2x1x1
        // tail the second texOffs speaks; NO head part the layer
        // keeps)
        Part {
            pivot: [0.0, 20.25, -1.25],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![
                Cube {
                    tex: (0.0, 0.0),
                    origin: [-2.0, -2.25, -0.75],
                    size: [4.0, 4.0, 4.0],
                    inflate: 0.0,
                    mirror: false,
                },
                Cube {
                    tex: (10.0, 8.0),
                    origin: [-1.0, -0.25, -1.75],
                    size: [2.0, 1.0, 1.0],
                    inflate: 0.0,
                    mirror: false,
                },
            ],
            children: vec![],
        },
        // left leg (the flat-plate shin + the foot plate)
        Part {
            pivot: [1.0, 22.0, 0.5],
            x_rot: leg_l,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![
                Cube {
                    tex: (2.0, 2.0),
                    origin: [-0.5, 0.0, 0.0],
                    size: [1.0, 2.0, 0.0],
                    inflate: 0.0,
                    mirror: false,
                },
                Cube {
                    tex: (0.0, 1.0),
                    origin: [-0.5, -2.0, -1.0],
                    size: [1.0, 0.0, 1.0],
                    inflate: 0.0,
                    mirror: false,
                },
            ],
            children: vec![],
        },
        // right leg (the same plates, the other texOffs pair)
        Part {
            pivot: [-1.0, 22.0, 0.5],
            x_rot: leg_r,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![
                Cube {
                    tex: (0.0, 2.0),
                    origin: [-0.5, 0.0, 0.0],
                    size: [1.0, 2.0, 0.0],
                    inflate: 0.0,
                    mirror: false,
                },
                Cube {
                    tex: (0.0, 0.0),
                    origin: [-0.5, -2.0, -1.0],
                    size: [1.0, 0.0, 1.0],
                    inflate: 0.0,
                    mirror: false,
                },
            ],
            children: vec![],
        },
        // right wing (the +flap zRot the inherited setupAnim keeps)
        Part {
            pivot: [2.0, 20.0, 0.0],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: flap_angle,
            cubes: vec![Cube {
                tex: (6.0, 8.0),
                origin: [0.0, 0.0, -1.0],
                size: [1.0, 0.0, 2.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // left wing (the negated zRot)
        Part {
            pivot: [-2.0, 20.0, 0.0],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: -flap_angle,
            cubes: vec![Cube {
                tex: (4.0, 8.0),
                origin: [-1.0, 0.0, -1.0],
                size: [1.0, 0.0, 2.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
    ]
}

/// THE SPIDER'S POSE STATE (the LivingEntityRenderState fields the
/// 26.2 `SpiderModel.setupAnim` reads — the head pair rides the
/// SAME netHeadYaw/pitch words the quadruped's own state wears, the
/// walk pair the WalkAnimationState feeds, the death clock the
/// entity fold's own input): the arachnid carries NO age lane (the
/// Monster base is no AgeableMob — no baby model the kind keeps).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpiderState {
    pub yaw_deg: f32,
    pub pitch_deg: f32,
    pub head_yaw_off_deg: f32,
    pub walk_pos: f32,
    pub walk_speed: f32,
    pub death_time: f32,
}

/// The full spider model — the 26.2 `SpiderModel.createSpiderBodyLayer`'s
/// own numbers, bytecode-proven off the official jar
/// (scripts/full_disasm.py: the createSpiderBodyLayer walk + the
/// setupAnim fold): the head 8x8x8 (uv 32,4) at pivot (0, 15, -3)
/// with the box reaching -8 z (the fangs' forward reach the PartPose
/// keeps), the neck cube 6x6x6 (uv 0,0) at pivot (0, 15, 0), the
/// abdomen 10x8x12 (uv 0,12) at pivot (0, 15, 9), and the EIGHT
/// 16x2x2 legs (uv 18,0) — the right boxes origin (-15,-1,-1) the
/// plain builder speaks, the LEFT pair's origin (-1,-1,-1) the
/// `.mirror()` u-flip rides — at pivots (∓4, 15, 2/1/0/-1) with the
/// PartPose.offsetAndRotation's diagonal stances
/// (±0.7853981852531433 / ±0.39269909262657166 y, the ±0.5811946392059326
/// middle z the bytecodes keep), `LayerDefinition.create(mesh, 64, 32)`.
pub struct SpiderModel {
    rgba: Vec<u8>,
}

impl SpiderModel {
    /// Load + sha1-verify the SPIDER's skin (the SpiderRenderer's
    /// SPIDER_LOCATION). The `None` = the spider never renders (the
    /// same honesty the other bodies speak: the renderer must not lie
    /// about its data).
    pub fn load(root: &Path) -> Option<SpiderModel> {
        let bytes = std::fs::read(root.join(skin::SPIDER_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::SPIDER_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 || rgba.len() != 64 * 32 * 4 {
            return None;
        }
        Some(SpiderModel { rgba })
    }

    /// The skin pixel at (u, v) — the RGBA row-major words, v down
    /// (the 64x32 layout's own stride the load verified).
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the world-space textured quads of the spider
    /// at `state` — the SAME entity chain fold the avatar speaks (one
    /// law, every living model), the arachnid's own parts beneath.
    pub fn quads(
        &self,
        state: &SpiderState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_spider_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 11);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// THE SPIDER'S PARTS — the 26.2 `SpiderModel.setupAnim`'s own walk
/// the bytecode folds verbatim: the head rides the look pair (the
/// yRot/xRot * PI/180 the ldcs speak), and the eight legs swing the
/// TWO-CLOCK law the fstores keep — the yRot deltas
/// `-cos(2*f2 + phase) * 0.4 * f3` with the phases 0 / PI / PI/2 /
/// 3PI/2 across the hind→front pairs (the right legs +delta, the
/// left -delta), and the zRot curls `|sin(f2 + phase)| * 0.4 * f3`
/// with the SAME phases (the right +, the left -) — where
/// `f2 = walkAnimationPos * 0.6661999821662903` and
/// `f3 = walkAnimationSpeed` the state fields feed. The read-modify
/// write the dup/fadd walk speaks: the deltas ADD to the
/// PartPose.offsetAndRotation's base stances.
fn build_spider_parts(state: &SpiderState) -> Vec<Part> {
    let f2 = state.walk_pos * 0.666_199_98;
    let f3 = state.walk_speed;
    // THE YROT SWINGS (the four cos folds — the phases the ldc
    // constants 3.1415927410125732 / 1.5707963705062866 /
    // 4.71238899230957 keep)
    let dy_hind = -mth_cos(f2 * 2.0) * 0.4 * f3;
    let dy_mid_hind = -mth_cos(f2 * 2.0 + std::f32::consts::PI) * 0.4 * f3;
    let dy_mid_front = -mth_cos(f2 * 2.0 + std::f32::consts::FRAC_PI_2) * 0.4 * f3;
    let dy_front = -mth_cos(f2 * 2.0 + 3.0 * std::f32::consts::FRAC_PI_2) * 0.4 * f3;
    // THE ZROT CURLS (the four |sin| folds — the same phases, the
    // abs() the invokestatic answers)
    let dz_hind = mth_sin(f2).abs() * 0.4 * f3;
    let dz_mid_hind = mth_sin(f2 + std::f32::consts::PI).abs() * 0.4 * f3;
    let dz_mid_front = mth_sin(f2 + std::f32::consts::FRAC_PI_2).abs() * 0.4 * f3;
    let dz_front = mth_sin(f2 + 3.0 * std::f32::consts::FRAC_PI_2).abs() * 0.4 * f3;
    let head_x = state.pitch_deg.to_radians();
    let head_y = state.head_yaw_off_deg.to_radians();
    // THE LEG BOXES (the right pair's plain builder vs the left
    // pair's `.mirror()` — the u-flip the flag rides, the box
    // origins the bytecodes keep: right (-15,-1,-1) reaches +x
    // from the pivot, left (-1,-1,-1) the mirrored reach)
    let right_leg_cube = Cube {
        tex: (18.0, 0.0),
        origin: [-15.0, -1.0, -1.0],
        size: [16.0, 2.0, 2.0],
        inflate: 0.0,
        mirror: false,
    };
    let left_leg_cube = Cube {
        tex: (18.0, 0.0),
        origin: [-1.0, -1.0, -1.0],
        size: [16.0, 2.0, 2.0],
        inflate: 0.0,
        mirror: true,
    };
    vec![
        // head (the look pair — the fangs' cube the forward reach)
        Part {
            pivot: [0.0, 15.0, -3.0],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (32.0, 4.0),
                origin: [-4.0, -4.0, -8.0],
                size: [8.0, 8.0, 8.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // neck body (the zero-pose cube the walk never touches)
        Part {
            pivot: [0.0, 15.0, 0.0],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 0.0),
                origin: [-3.0, -3.0, -3.0],
                size: [6.0, 6.0, 6.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // abdomen (the zero-pose barrel)
        Part {
            pivot: [0.0, 15.0, 9.0],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 12.0),
                origin: [-5.0, -4.0, -6.0],
                size: [10.0, 8.0, 12.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // right hind leg (base y +0.7853981852531433, z -0.7853981852531433)
        Part {
            pivot: [-4.0, 15.0, 2.0],
            x_rot: 0.0,
            y_rot: 0.785_398_2 + dy_hind,
            z_rot: -0.785_398_2 + dz_hind,
            cubes: vec![right_leg_cube.clone()],
            children: vec![],
        },
        // left hind leg (the mirrored stance — the deltas subtract)
        Part {
            pivot: [4.0, 15.0, 2.0],
            x_rot: 0.0,
            y_rot: -0.785_398_2 - dy_hind,
            z_rot: 0.785_398_2 - dz_hind,
            cubes: vec![left_leg_cube.clone()],
            children: vec![],
        },
        // right middle-hind leg (base y +0.39269909262657166, z -0.5811946392059326)
        Part {
            pivot: [-4.0, 15.0, 1.0],
            x_rot: 0.0,
            y_rot: 0.392_699_1 + dy_mid_hind,
            z_rot: -0.581_194_64 + dz_mid_hind,
            cubes: vec![right_leg_cube.clone()],
            children: vec![],
        },
        // left middle-hind leg
        Part {
            pivot: [4.0, 15.0, 1.0],
            x_rot: 0.0,
            y_rot: -0.392_699_1 - dy_mid_hind,
            z_rot: 0.581_194_64 - dz_mid_hind,
            cubes: vec![left_leg_cube.clone()],
            children: vec![],
        },
        // right middle-front leg (base y -0.39269909262657166, z -0.5811946392059326)
        Part {
            pivot: [-4.0, 15.0, 0.0],
            x_rot: 0.0,
            y_rot: -0.392_699_1 + dy_mid_front,
            z_rot: -0.581_194_64 + dz_mid_front,
            cubes: vec![right_leg_cube.clone()],
            children: vec![],
        },
        // left middle-front leg
        Part {
            pivot: [4.0, 15.0, 0.0],
            x_rot: 0.0,
            y_rot: 0.392_699_1 - dy_mid_front,
            z_rot: 0.581_194_64 - dz_mid_front,
            cubes: vec![left_leg_cube.clone()],
            children: vec![],
        },
        // right front leg (base y -0.7853981852531433, z -0.7853981852531433)
        Part {
            pivot: [-4.0, 15.0, -1.0],
            x_rot: 0.0,
            y_rot: -0.785_398_2 + dy_front,
            z_rot: -0.785_398_2 + dz_front,
            cubes: vec![right_leg_cube],
            children: vec![],
        },
        // left front leg
        Part {
            pivot: [4.0, 15.0, -1.0],
            x_rot: 0.0,
            y_rot: 0.785_398_2 - dy_front,
            z_rot: 0.785_398_2 - dz_front,
            cubes: vec![left_leg_cube],
            children: vec![],
        },
    ]
}

/// THE CREEPER'S POSE STATE (the LivingEntityRenderState fields the
/// 26.2 `CreeperModel.setupAnim` reads — the head pair rides the
/// SAME netHeadYaw/pitch words the quadruped's own state wears — the
/// MobRenderer super's own extraction fills the render state's yRot/
/// xRot the extractRenderState walk never overrides (the Creeper
/// renderer's own walk the swelling + isPowered pair alone sets) —
/// the walk pair the WalkAnimationState feeds, the death clock the
/// entity fold's own input): the creeper carries NO age lane (the
/// Monster base is no AgeableMob — no baby model the kind keeps).
/// THE SWELLING the render state's own field carries (javap
/// `CreeperRenderer.extractRenderState` the 26.2 jar speaks — the
/// `getSwelling(partialTick)` the swelling word banks, the
/// `Mth.lerp(partialTick, oldSwell, swell) / maxSwell` the entity's
/// own fold answers): the scale fold AND the white overlay the same
/// word reads — the zero the plain body's 1.0 arms wear.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CreeperState {
    pub yaw_deg: f32,
    pub pitch_deg: f32,
    pub head_yaw_off_deg: f32,
    pub walk_pos: f32,
    pub walk_speed: f32,
    pub death_time: f32,
    pub swelling: f32,
}

/// THE SWELLING'S OWN SCALE FOLD (javap `CreeperRenderer.scale` the
/// 26.2 jar speaks — the raw bytecode the four float constants pin:
/// the `g = 1.0F + Mth.sin(f * 100.0F) * f * 0.01F` the wobble factor
/// keeps (the f2d the Mth.sin's double arg wears), the clamp 0..1 the
/// f2i-less arm answers, the TWO `f *= f` squarings the f⁴ law
/// speaks — the 26.2 the older f² walks replaced, the
/// 0.009999999776482582 / 0.4000000059604645 / 0.10000000149011612
/// the ldc keeps — the `pose.scale((1.0 + f * 0.4F) * g, (1.0 + f *
/// 0.1F) / g, (1.0 + f * 0.4F) * g)` the X/Y/Z triple answers).
/// Returns the (x/z, y) pair the quad transform wears.
pub fn creeper_swelling_scale(swelling: f32) -> (f32, f32) {
    let f = swelling;
    let g = 1.0 + ((f * 100.0) as f64).sin() as f32 * f * 0.009_999_999_776_482_582;
    let f = f.clamp(0.0, 1.0);
    let f = f * f;
    let f = f * f;
    let x = (1.0 + f * 0.400_000_005_960_464_5) * g;
    let y = (1.0 + f * 0.100_000_001_490_116_12) / g;
    (x, y)
}

/// THE WHITE OVERLAY'S OWN FLASH (javap
/// `CreeperRenderer.getWhiteOverlayProgress` the 26.2 jar speaks —
/// the `(int)(f * 10.0F) % 2 == 0 ? 0.0F : Mth.clamp(f, 0.5F, 1.0F)`
/// the raw bytecode keeps: the even decade the silence wears, the odd
/// the clamp's own band answers — the fuse's end the flash's own
/// cadence quickens, the full-white the last decades wear). The
/// progress > 0.0 the model's white render speaks (the overlay
/// texture's own white row the whole model covers).
pub fn creeper_white_overlay(swelling: f32) -> f32 {
    let f = swelling;
    if (f * 10.0) as i32 % 2 == 0 {
        0.0
    } else {
        f.clamp(0.5, 1.0)
    }
}

/// THE SWELLING'S OWN QUAD FOLD (the pose.scale walk's own chain the
/// render pipeline wears — the translate-to-entity the pose's own
/// origin keeps, the scale the model's local frame rides: the points
/// the feet anchor scales — the x/z the X factor wears, the y the Y
/// factor rides).
pub fn apply_creeper_swelling(
    quads: &mut [AvatarQuad],
    feet: Vec3,
    swelling: f32,
) {
    let (sx, sy) = creeper_swelling_scale(swelling);
    for q in quads {
        for pt in &mut q.pts {
            pt[0] = feet.x + (pt[0] - feet.x) * sx;
            pt[1] = feet.y + (pt[1] - feet.y) * sy;
            pt[2] = feet.z + (pt[2] - feet.z) * sx;
        }
    }
}

/// The full creeper model — the 26.2 `CreeperModel.createBodyLayer`
/// own numbers, bytecode-proven off the official jar (the
/// createBodyLayer(CubeDeformation) walk + the setupAnim fold): the
/// head 8x8x8 (uv 0,0) at pivot (0, 6, 0) — the box reaching -8 y
/// the crown's own reach keeps — the body 8x12x4 (uv 16,16) at
/// pivot (0, 6, 0), and the FOUR 4x6x4 legs (uv 0,16) at pivots
/// (∓2, 18, ±4) the plain builder speaks,
/// `LayerDefinition.create(mesh, 64, 32)`.
pub struct CreeperModel {
    rgba: Vec<u8>,
}

impl CreeperModel {
    /// Load + sha1-verify the CREEPER's skin (the CreeperRenderer's
    /// CREEPER_LOCATION — `textures/entity/creeper/creeper.png` the
    /// clinit's withDefaultNamespace names). The `None` = the creeper
    /// never renders (the same honesty the other bodies speak: the
    /// renderer must not lie about its data).
    pub fn load(root: &Path) -> Option<CreeperModel> {
        let bytes = std::fs::read(root.join(skin::CREEPER_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::CREEPER_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 || rgba.len() != 64 * 32 * 4 {
            return None;
        }
        Some(CreeperModel { rgba })
    }

    /// The skin pixel at (u, v) — the RGBA row-major words, v down
    /// (the 64x32 layout's own stride the load verified).
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the world-space textured quads of the
    /// creeper at `state` — the SAME entity chain fold the avatar
    /// speaks (one law, every living model), the creeper's own parts
    /// beneath.
    pub fn quads(
        &self,
        state: &CreeperState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_creeper_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 6);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// THE CREEPER'S PARTS — the 26.2 `CreeperModel.setupAnim`'s own walk
/// the bytecode folds verbatim: the head rides the look pair (the
/// yRot/xRot * 0.01745329238474369 the two ldc keeps), and the four
/// legs swing the quadruped walk the cos folds speak —
/// `cos(walkAnimationPos * 0.6661999821662903 [+ PI]) *
/// 1.399999976158142 * walkAnimationSpeed` — the right-hind and the
/// left-front the PLAIN phase keep, the left-hind and the right-front
/// the +3.1415927410125732 anti-phase ride (the diagonal gait the
/// four putfield walks bank, the float consts the raw scan names).
fn build_creeper_parts(state: &CreeperState) -> Vec<Part> {
    let f2 = state.walk_pos * 0.666_199_98;
    let f3 = state.walk_speed;
    // THE LEG SWINGS (the four cos folds — the phases the raw walk
    // keeps: right-hind plain, left-hind +PI, right-front +PI,
    // left-front plain)
    let leg_plain = mth_cos(f2) * 1.4 * f3;
    let leg_anti = mth_cos(f2 + std::f32::consts::PI) * 1.4 * f3;
    let head_x = state.pitch_deg.to_radians();
    let head_y = state.head_yaw_off_deg.to_radians();
    // THE LEG BOX (the one cube the four legs share — the
    // `loc3` reuse the createBodyLayer walk speaks, the same
    // texOffs(0,16) addBox(-2, 0, -2, 4, 6, 4) each pivot rides)
    let leg_cube = Cube {
        tex: (0.0, 16.0),
        origin: [-2.0, 0.0, -2.0],
        size: [4.0, 6.0, 4.0],
        inflate: 0.0,
        mirror: false,
    };
    vec![
        // head (the look pair — the crown's cube the -8 reach wears)
        Part {
            pivot: [0.0, 6.0, 0.0],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 0.0),
                origin: [-4.0, -8.0, -4.0],
                size: [8.0, 8.0, 8.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // body (the zero-pose prism)
        Part {
            pivot: [0.0, 6.0, 0.0],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (16.0, 16.0),
                origin: [-4.0, 0.0, -2.0],
                size: [8.0, 12.0, 4.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // right hind leg (the plain phase — the pivot (-2, 18, 4) the
        // PartPose.offset keeps)
        Part {
            pivot: [-2.0, 18.0, 4.0],
            x_rot: leg_plain,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube.clone()],
            children: vec![],
        },
        // left hind leg (the +PI anti-phase — the pivot (2, 18, 4))
        Part {
            pivot: [2.0, 18.0, 4.0],
            x_rot: leg_anti,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube.clone()],
            children: vec![],
        },
        // right front leg (the +PI anti-phase — the pivot (-2, 18, -4))
        Part {
            pivot: [-2.0, 18.0, -4.0],
            x_rot: leg_anti,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube.clone()],
            children: vec![],
        },
        // left front leg (the plain phase — the pivot (2, 18, -4))
        Part {
            pivot: [2.0, 18.0, -4.0],
            x_rot: leg_plain,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube],
            children: vec![],
        },
    ]
}

/// 26.2 `WolfModel.setupAnim` reads — the head pair rides the
/// netHeadYaw/pitch words the quadruped's own state wears, the walk
/// pair the WalkAnimationState feeds, the death clock the entity
/// fold's own input. THE TAIL'S OWN WORD the caller's extraction
/// carries (javap `Wolf.getTailAngle` — the angry 1.539380431175232
/// / the tamed health fold / the wild 0.6283185482025146 the three
/// arms speak, the state the tailAngleF field the renderer's own
/// extraction rides — BOTH models the same word wear). THE SIT + THE
/// ANGER the tame machine's own words the tame lanes arm: the pose
/// the SitWhenOrderedToGoal's running word, the anger the prio-4
/// lane's clock — the WAG the setupAnim's own isAngry branch stills
/// (`tail.yRot = 0` the ifne arm keeps), the legs the isSitting
/// branch the setSittingPose walk swaps the swing folds for.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WolfState {
    pub yaw_deg: f32,
    pub pitch_deg: f32,
    pub head_yaw_off_deg: f32,
    pub walk_pos: f32,
    pub walk_speed: f32,
    pub death_time: f32,
    /// the `getTailAngle()` word the caller's own extraction speaks
    pub tail_angle: f32,
    /// the `isInSittingPose()` word — the SitWhenOrderedToGoal's
    /// running bit
    pub sitting: bool,
    /// the `isAngry()` word — the prio-4 lane's clock the >0 word
    /// speaks; the wag the branch stills
    pub angry: bool,
}

/// THE SIT POSE'S OWN WORDS (javap `WolfModel.setSittingPose` the 26.2
/// bytecode keeps verbatim — the body `yF += 4.0 * ageScale` +
/// `zF -= 2.0 * ageScale` + `xRotF = 0.7853981852531433` (PI/4, the
/// putfield the base xRot REPLACES), the tail `yF += 9.0` +
/// `zF -= 2.0`, the hind legs `yF += 6.699999809265137` +
/// `zF -= 5.0` + `xRotF = 4.71238899230957` (3PI/2), the front legs
/// `xRotF = 5.811946868896484` + the ±0.009999999776482582 x eps +
/// `yF += 1.0` — the LEG SWINGS the isSitting branch's own else
/// never runs, the tail xRot the tailAngleF the setupAnim's own tail
/// line re-wears AFTER the pose).
pub const WOLF_SIT_BODY_X_ROT: f32 = 0.785_398_2;
pub const WOLF_SIT_LEG_HIND_X_ROT: f32 = 4.712_389;
pub const WOLF_SIT_LEG_FRONT_X_ROT: f32 = 5.811_946_9;
pub const WOLF_SIT_FRONT_LEG_X_EPS: f32 = 0.01;
/// THE BABY'S OWN SIT CORRECTION (javap `BabyWolfModel.
/// setSittingPose` the override keeps — the super's walk the body
/// xRot `-= 1.5707963705062866` the tail subtracts: the sit value
/// the PI/2 the negation answers).
pub const BABY_WOLF_SIT_BODY_X_ROT_SUB: f32 = 1.570_796_4;

/// The full adult wolf model — the 26.2 `AdultWolfModel.createBodyLayer`'s
/// own numbers, bytecode-proven off the official jar (classfile_dump +
/// raw_disasm: the createBodyLayer walk + the WolfModel.setupAnim fold):
/// the head pivot (-1, 13.5, -7) with the CHILD real_head (PartPose.ZERO
/// — the four cubes: the 6x6x4 skull (uv 0,0), the two 2x2x1 ears (uv
/// 16,14) at ±2 x, the 3x3x4 muzzle (uv 0,10) at the -0.001 y the
/// ldc's epsilon keeps), the body 6x9x6 (uv 18,14) at pivot (0, 14, 2)
/// rotated 1.5707963705062866 x, the upper_body 8x6x7 (uv 21,0) at
/// pivot (-1, 14, -3) with the SAME x-rot, the FOUR 2x8x2 legs (uv
/// 0,18) — the right pair's `.mirror()` u-flip, the pivots
/// (-2.5/0.5, 16, 7/-4) — and the tail pivot (-1, 12, 8) with the
/// base 0.6283185482025146 x-rot, the child real_tail (uv 9,18) the
/// 2x8x2 brush wears.
pub struct WolfModel {
    rgba: Vec<u8>,
}

impl WolfModel {
    /// Load + sha1-verify the WOLF's skin (the WolfRenderer's
    /// getTextureLocation the variant table's pale word). The `None` =
    /// the wolf never renders (the same honesty the other bodies speak:
    /// the renderer must not lie about its data).
    pub fn load(root: &Path) -> Option<WolfModel> {
        let bytes = std::fs::read(root.join(skin::WOLF_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::WOLF_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 || rgba.len() != 64 * 32 * 4 {
            return None;
        }
        Some(WolfModel { rgba })
    }

    /// The skin pixel at (u, v) — the RGBA row-major words, v down
    /// (the 64x32 layout's own stride the load verified).
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 63.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the world-space textured quads of the wolf
    /// at `state` — the SAME entity chain fold the avatar speaks (one
    /// law, every living model), the canine's own parts beneath.
    pub fn quads(
        &self,
        state: &WolfState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_wolf_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 9);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// THE WOLF'S PARTS — the 26.2 `WolfModel.setupAnim`'s own walk the
/// bytecode folds verbatim: the head rides the look pair (the
/// 0.01745329238474369 PI/180 fold), the tail yaws the wag
/// `cos(f2 * 0.6661999821662903) * 1.399999976158142 * f3` the ldc
/// pair keeps, the four legs swing the SAME multiplier (the diagonal
/// pairs the +3.1415927410125732 phase splits: the right-hind and
/// the left-front in phase, the left-hind and the right-front
/// counter-phase), the tail's xRot the WILD TAIL ANGLE the
/// getTailAngle word wears, and the body's zRot the shakeOffWater's
/// `getBodyRollAngle(-0.1599999964237213)` the NO-WETNESS lane's 0.0
/// answers (the wet-shake the client's water lane never tracks).
fn build_wolf_parts(state: &WolfState) -> Vec<Part> {
    let f2 = state.walk_pos;
    let f3 = state.walk_speed;
    // THE WAG + THE LEG SWINGS (the four cos folds — the diagonal
    // phases the ldc 3.1415927410125732 keeps). THE WAG the setupAnim's
    // own isAngry branch stills (offsets 15-30: the `tail.yRot = 0`
    // the ifne arm keeps — the angry body the wag the zero word
    // wears). THE LEG SWINGS the isSitting branch's own else never
    // runs (offsets 53-65: the sitting body setSittingPose rides, the
    // swing folds the branch skips — the sit xRots the pose's own
    // constants wear).
    let (wag, leg_rh, leg_lh, leg_rf, leg_lf) = if state.angry {
        (0.0, 0.0, 0.0, 0.0, 0.0)
    } else {
        (
            mth_cos(f2 * 0.666_199_98) * 1.399_999_98 * f3,
            mth_cos(f2 * 0.666_199_98) * 1.399_999_98 * f3,
            mth_cos(f2 * 0.666_199_98 + 3.141_592_7) * 1.399_999_98 * f3,
            mth_cos(f2 * 0.666_199_98 + 3.141_592_7) * 1.399_999_98 * f3,
            mth_cos(f2 * 0.666_199_98) * 1.399_999_98 * f3,
        )
    };
    let head_x = state.pitch_deg.to_radians();
    let head_y = state.head_yaw_off_deg.to_radians();
    // THE LEG BOXES (the right pair's `.mirror()` u-flip the builder
    // walk keeps, the box (0,0,-1) 2x8x2 the four share)
    let leg_cube = |mirror: bool| Cube {
        tex: (0.0, 18.0),
        origin: [0.0, 0.0, -1.0],
        size: [2.0, 8.0, 2.0],
        inflate: 0.0,
        mirror,
    };
    // THE SIT POSE'S OWN PIVOT + ROT WORDS (javap `setSittingPose`
    // verbatim — the ageScale the ADULT's own 1.0 the plain folds
    // keep; the CHEST the walk's own field list never names — the
    // upper_body the unmoved pivot the vanilla law keeps)
    let (body_pivot, body_x_rot) = if state.sitting {
        ([0.0, 18.0, 0.0], WOLF_SIT_BODY_X_ROT)
    } else {
        ([0.0, 14.0, 2.0], 1.570_796_4)
    };
    let (hind_pivot, hind_x_rot) = if state.sitting {
        ([-2.5, 22.7, 2.0], WOLF_SIT_LEG_HIND_X_ROT)
    } else {
        ([-2.5, 16.0, 7.0], leg_rh)
    };
    let (hind_pivot_l, hind_x_rot_l) = if state.sitting {
        ([0.5, 22.7, 2.0], WOLF_SIT_LEG_HIND_X_ROT)
    } else {
        ([0.5, 16.0, 7.0], leg_lh)
    };
    let (front_pivot_r, front_x_rot_r) = if state.sitting {
        (
            [-2.5 + WOLF_SIT_FRONT_LEG_X_EPS, 17.0, -4.0],
            WOLF_SIT_LEG_FRONT_X_ROT,
        )
    } else {
        ([-2.5, 16.0, -4.0], leg_rf)
    };
    let (front_pivot_l, front_x_rot_l) = if state.sitting {
        (
            [0.5 - WOLF_SIT_FRONT_LEG_X_EPS, 17.0, -4.0],
            WOLF_SIT_LEG_FRONT_X_ROT,
        )
    } else {
        ([0.5, 16.0, -4.0], leg_lf)
    };
    let tail_pivot = if state.sitting {
        [-1.0, 21.0, 6.0]
    } else {
        [-1.0, 12.0, 8.0]
    };
    vec![
        // head (the look pair rides; the child real_head the four
        // cubes wears — the ZERO offset the parent matrix carries)
        Part {
            pivot: [-1.0, 13.5, -7.0],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![],
            children: vec![Part {
                pivot: [0.0, 0.0, 0.0],
                x_rot: 0.0,
                y_rot: 0.0,
                z_rot: 0.0,
                cubes: vec![
                    // the skull (uv 0,0)
                    Cube {
                        tex: (0.0, 0.0),
                        origin: [-2.0, -3.0, -2.0],
                        size: [6.0, 6.0, 4.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    // the left ear (uv 16,14)
                    Cube {
                        tex: (16.0, 14.0),
                        origin: [-2.0, -5.0, 0.0],
                        size: [2.0, 2.0, 1.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    // the right ear (uv 16,14)
                    Cube {
                        tex: (16.0, 14.0),
                        origin: [2.0, -5.0, 0.0],
                        size: [2.0, 2.0, 1.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                    // the muzzle (uv 0,10 — the y epsilon the ldc keeps)
                    Cube {
                        tex: (0.0, 10.0),
                        origin: [-0.5, -0.001_000_000_5, -5.0],
                        size: [3.0, 3.0, 4.0],
                        inflate: 0.0,
                        mirror: false,
                    },
                ],
                children: vec![],
            }],
        },
        // body (the 90-degree x-rot the ctor's PartPose speaks —
        // the barrel lies along the spine; the shake-off roll the
        // no-wetness word zeroes; the SIT the setSittingPose's own
        // putfield the xRot REPLACES — PI/4 the sit word speaks —
        // and the pivot the y+4/z-2 folds shift)
        Part {
            pivot: body_pivot,
            x_rot: body_x_rot,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (18.0, 14.0),
                origin: [-3.0, -2.0, -3.0],
                size: [6.0, 9.0, 6.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // upper body (the chest, the SAME x-rot)
        Part {
            pivot: [-1.0, 14.0, -3.0],
            x_rot: 1.570_796_4,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (21.0, 0.0),
                origin: [-3.0, -3.0, -3.0],
                size: [8.0, 6.0, 7.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // right hind leg (the mirror u-flip; the plain phase; the
        // SIT the y+6.7/z-5 pivots + the 3PI/2 xRot the pose speaks)
        Part {
            pivot: hind_pivot,
            x_rot: hind_x_rot,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(true)],
            children: vec![],
        },
        // left hind leg (the plain builder; the counter phase)
        Part {
            pivot: hind_pivot_l,
            x_rot: hind_x_rot_l,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(false)],
            children: vec![],
        },
        // right front leg (the mirror; the counter phase)
        Part {
            pivot: front_pivot_r,
            x_rot: front_x_rot_r,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(true)],
            children: vec![],
        },
        // left front leg (the plain; the plain phase)
        Part {
            pivot: front_pivot_l,
            x_rot: front_x_rot_l,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube(false)],
            children: vec![],
        },
        // tail (the wag yaws — the ANGRY branch the zero word wears;
        // the xRot the STATE'S OWN tail angle the getTailAngle walk
        // speaks — the angry/tame/wild arms the caller's extraction
        // carries; the SIT the y+9/z-2 pivots the pose shifts — the
        // child real_tail the brush carries)
        Part {
            pivot: tail_pivot,
            x_rot: state.tail_angle,
            y_rot: wag,
            z_rot: 0.0,
            cubes: vec![],
            children: vec![Part {
                pivot: [0.0, 0.0, 0.0],
                x_rot: 0.0,
                y_rot: 0.0,
                z_rot: 0.0,
                cubes: vec![Cube {
                    tex: (9.0, 18.0),
                    origin: [0.0, 0.0, -1.0],
                    size: [2.0, 8.0, 2.0],
                    inflate: 0.0,
                    mirror: false,
                }],
                children: vec![],
            }],
        },
    ]
}

/// THE WOLF BABY'S OWN MODEL — the 26.2 `BabyWolfModel(bakeLayer(WOLF_BABY))`
/// the AgeableMobRenderer's babyModel field the isBaby word picks (NOT a
/// scaled adult — the renderer's own second model the 0.5 age scale
/// pairs). The mesh the BabyWolfModel.createBodyLayer's
/// `LayerDefinition.create(mesh, 32, 32)` keeps: the bigger-looking head
/// (the 6x5x5 skull uv (0,12) with the 0.02500000037252903 deformation,
/// the 3x2x2 muzzle uv (17,12)), the two 2x2x1 ears as the head's
/// CHILDREN (uv (0,5)/(20,5) at the ±2 x, -4.25 y pivots), the 6x4x8
/// body (uv 0,0), the four 2x3x2 stub legs (the four DIFFERENT uv
/// origins (0,22)/(8,22)/(0,0)/(20,0) the plain builder keeps), the
/// tail pivot (0, 19, 3) with the base -0.5235999822616577 x-rot — the
/// base the INHERITED setupAnim's tailAngle override swallows (the
/// BabyWolfModel extends WolfModel: the same walk/wag/look law runs,
/// the wild 0.6283185482025146 the xRot wears) — and the tail_r1 child
/// (the pivot (0, -0.6, 0.2), the x-rot -3.0999999046325684, the
/// 2x6x2 brush uv (22,16)).
pub struct BabyWolfModel {
    rgba: Vec<u8>,
}

impl BabyWolfModel {
    /// Load + sha1-verify the BABY wolf's skin (the wolf_baby.png the
    /// variant table's baby entry). `None` = the baby never renders.
    pub fn load(root: &Path) -> Option<BabyWolfModel> {
        let bytes = std::fs::read(root.join(skin::WOLF_BABY_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::WOLF_BABY_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 32 || h != 32 || rgba.len() != 32 * 32 * 4 {
            return None;
        }
        Some(BabyWolfModel { rgba })
    }

    /// The baby skin pixel at (u, v) — the 32x32 layout's own stride.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 31.999_9), v.clamp(0.0, 31.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 32 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the baby's quads — the SAME walk/wag/look
    /// law the adult's build runs (the BabyWolfModel extends
    /// WolfModel: no override the class keeps), the baby's own mesh
    /// beneath.
    pub fn quads(
        &self,
        state: &WolfState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_baby_wolf_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 10);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// the wolf BABY's parts, posed by `state` — the BabyWolfModel's own
/// bytecode-proven mesh (the deform head, the child ears, the stub
/// legs on four distinct uvs, the curled tail the child pivot
/// carries), the SAME anim law the adult speaks.
fn build_baby_wolf_parts(state: &WolfState) -> Vec<Part> {
    let f2 = state.walk_pos;
    let f3 = state.walk_speed;
    // THE INHERITED ANIM (the BabyWolfModel extends WolfModel — no
    // setupAnim override the class keeps: the wag the isAngry branch
    // stills, the leg swings the isSitting branch's own else skips,
    // the sit pose the OVERRIDDEN setSittingPose's own words wear —
    // the super's folds the same constants ride, the body xRot the
    // `-= 1.5707963705062866` the tail subtracts)
    let (wag, leg_rh, leg_lh, leg_rf, leg_lf) = if state.angry {
        (0.0, 0.0, 0.0, 0.0, 0.0)
    } else {
        (
            mth_cos(f2 * 0.666_199_98) * 1.399_999_98 * f3,
            mth_cos(f2 * 0.666_199_98) * 1.399_999_98 * f3,
            mth_cos(f2 * 0.666_199_98 + 3.141_592_7) * 1.399_999_98 * f3,
            mth_cos(f2 * 0.666_199_98 + 3.141_592_7) * 1.399_999_98 * f3,
            mth_cos(f2 * 0.666_199_98) * 1.399_999_98 * f3,
        )
    };
    let head_x = state.pitch_deg.to_radians();
    let head_y = state.head_yaw_off_deg.to_radians();
    let (body_pivot, body_x_rot) = if state.sitting {
        (
            [0.0, 23.0, -2.0],
            WOLF_SIT_BODY_X_ROT - BABY_WOLF_SIT_BODY_X_ROT_SUB,
        )
    } else {
        ([0.0, 19.0, 0.0], 0.0)
    };
    let (hind_pivot, hind_x_rot) = if state.sitting {
        ([-1.5, 27.7, -2.0], WOLF_SIT_LEG_HIND_X_ROT)
    } else {
        ([-1.5, 21.0, 3.0], leg_rh)
    };
    let (hind_pivot_l, hind_x_rot_l) = if state.sitting {
        ([1.5, 27.7, -2.0], WOLF_SIT_LEG_HIND_X_ROT)
    } else {
        ([1.5, 21.0, 3.0], leg_lh)
    };
    let (front_pivot_r, front_x_rot_r) = if state.sitting {
        (
            [-1.5 + WOLF_SIT_FRONT_LEG_X_EPS, 22.0, -3.0],
            WOLF_SIT_LEG_FRONT_X_ROT,
        )
    } else {
        ([-1.5, 21.0, -3.0], leg_rf)
    };
    let (front_pivot_l, front_x_rot_l) = if state.sitting {
        (
            [1.5 - WOLF_SIT_FRONT_LEG_X_EPS, 22.0, -3.0],
            WOLF_SIT_LEG_FRONT_X_ROT,
        )
    } else {
        ([1.5, 21.0, -3.0], leg_lf)
    };
    let tail_pivot = if state.sitting {
        [0.0, 28.0, 1.0]
    } else {
        [0.0, 19.0, 3.0]
    };
    vec![
        // head (the deform skull + the muzzle; the two ears ride as
        // the children the pivots carry)
        Part {
            pivot: [0.0, 18.25, -4.0],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![
                Cube {
                    tex: (0.0, 12.0),
                    origin: [-2.990_000_0, -3.25, -3.0],
                    size: [6.0, 5.0, 5.0],
                    inflate: 0.025_000_000,
                    mirror: false,
                },
                Cube {
                    tex: (17.0, 12.0),
                    origin: [-1.5, -0.239_999_99, -5.0],
                    size: [3.0, 2.0, 2.0],
                    inflate: 0.0,
                    mirror: false,
                },
            ],
            children: vec![
                Part {
                    pivot: [-2.0, -4.25, -0.5],
                    x_rot: 0.0,
                    y_rot: 0.0,
                    z_rot: 0.0,
                    cubes: vec![Cube {
                        tex: (0.0, 5.0),
                        origin: [-1.0, -1.0, -0.5],
                        size: [2.0, 2.0, 1.0],
                        inflate: 0.0,
                        mirror: false,
                    }],
                    children: vec![],
                },
                Part {
                    pivot: [2.0, -4.25, -0.5],
                    x_rot: 0.0,
                    y_rot: 0.0,
                    z_rot: 0.0,
                    cubes: vec![Cube {
                        tex: (20.0, 5.0),
                        origin: [-1.0, -1.0, -0.5],
                        size: [2.0, 2.0, 1.0],
                        inflate: 0.0,
                        mirror: false,
                    }],
                    children: vec![],
                },
            ],
        },
        // body
        Part {
            pivot: body_pivot,
            x_rot: body_x_rot,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 0.0),
                origin: [-3.0, -2.0, -4.0],
                size: [6.0, 4.0, 8.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // right hind leg (uv (0,22))
        Part {
            pivot: hind_pivot,
            x_rot: hind_x_rot,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 22.0),
                origin: [-1.0, 0.0, -1.0],
                size: [2.0, 3.0, 2.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // left hind leg (uv (8,22))
        Part {
            pivot: hind_pivot_l,
            x_rot: hind_x_rot_l,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (8.0, 22.0),
                origin: [-1.0, 0.0, -1.0],
                size: [2.0, 3.0, 2.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // right front leg (uv (0,0))
        Part {
            pivot: front_pivot_r,
            x_rot: front_x_rot_r,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 0.0),
                origin: [-1.0, 0.0, -1.0],
                size: [2.0, 3.0, 2.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // left front leg (uv (20,0))
        Part {
            pivot: front_pivot_l,
            x_rot: front_x_rot_l,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (20.0, 0.0),
                origin: [-1.0, 0.0, -1.0],
                size: [2.0, 3.0, 2.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // tail (the base -0.5235999822616577 the INHERITED tailAngle
        // override swallows — the STATE'S OWN angle the xRot wears,
        // the same word the adult's tail carries; the curled brush
        // the tail_r1 child carries; the SIT the y+9/z-2 pivots the
        // inherited pose shifts)
        Part {
            pivot: tail_pivot,
            x_rot: state.tail_angle,
            y_rot: wag,
            z_rot: 0.0,
            cubes: vec![],
            children: vec![Part {
                pivot: [0.0, -0.600_000_02, 0.200_000_00],
                x_rot: -3.099_999_9,
                y_rot: 0.0,
                z_rot: 0.0,
                cubes: vec![Cube {
                    tex: (22.0, 16.0),
                    origin: [-1.0, -5.699_999_8, -1.0],
                    size: [2.0, 6.0, 2.0],
                    inflate: 0.0,
                    mirror: false,
                }],
                children: vec![],
            }],
        },
    ]
}

/// the porcine BABY's parts, posed by `state` — the BabyPigModel's own
/// bytecode-proven mesh (the 7x6x9 horizontal body, the two-cube head
/// with its inflate deformations, the four 2x2x2 stub legs on four
/// distinct rects), the SAME pose law the adult speaks (the walk's
/// diagonal pairs — PigModel carries no setupAnim override either).
fn build_baby_pig_parts(state: &PigState) -> Vec<Part> {
    let pos = state.walk_pos;
    let speed = state.walk_speed;
    let leg_rh = mth_cos(pos * 0.6662) * 1.4 * speed;
    let leg_lh = mth_cos(pos * 0.6662 + std::f32::consts::PI) * 1.4 * speed;
    let head_x = state.pitch_deg.to_radians();
    let head_y = state.head_yaw_off_deg.to_radians();
    let leg_cube = |tex: (f32, f32)| Cube {
        tex,
        origin: [-1.0, 0.0, -1.0],
        size: [2.0, 2.0, 2.0],
        inflate: 0.0,
        mirror: false,
    };
    vec![
        // head (+ the snout — the same part's two cubes, the inflate
        // deformations the baby layer speaks)
        Part {
            pivot: [0.0, 19.0, -2.0],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![
                Cube {
                    tex: (0.0, 15.0),
                    origin: [-3.5, -5.0, -5.0],
                    size: [7.0, 6.0, 6.0],
                    inflate: 0.025,
                    mirror: false,
                },
                Cube {
                    tex: (6.0, 27.0),
                    origin: [-1.5, -1.975, -6.0],
                    size: [3.0, 2.0, 1.0],
                    inflate: 0.015,
                    mirror: false,
                },
            ],
            children: vec![],
        },
        // body (the horizontal box — NO rotX)
        Part {
            pivot: [0.0, 19.0, 0.5],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 0.0),
                origin: [-3.5, -3.0, -4.5],
                size: [7.0, 6.0, 9.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        },
        // left front leg (uv 0,0 — the stub)
        Part {
            pivot: [2.5, 22.0, -3.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((0.0, 0.0))],
            children: vec![],
        },
        // right front leg (uv 23,0)
        Part {
            pivot: [-2.5, 22.0, -3.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((23.0, 0.0))],
            children: vec![],
        },
        // left hind leg (uv 0,4)
        Part {
            pivot: [2.5, 22.0, 4.0],
            x_rot: leg_lh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((0.0, 4.0))],
            children: vec![],
        },
        // right hind leg (uv 23,4)
        Part {
            pivot: [-2.5, 22.0, 4.0],
            x_rot: leg_rh,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![leg_cube((23.0, 4.0))],
            children: vec![],
        },
    ]
}

/// `SpearAnimations.thirdPersonHandUse(ModelPart arm, ModelPart head,
/// boolean right, ItemStack, HumanoidRenderState)` — the SPEAR ArmPose's
/// own model-lane fold, verbatim from the 26.2 bytecode (the dump
/// /tmp/spear.txt, 314 bytes): the BASE pose every SPEAR-posed arm
/// wears (the rest raise the spear-holding body never drops — the jar's
/// `getArmPose` the SPEARS tag law speaks: swinging, using, or neither),
/// then the use gate and the kinetic sway the USING arm adds. The base:
/// `yRot += -0.1·side`, `xRot += -π/2` then `+= 0.8` (the TWO fadds the
/// putfield pair keeps), the fall-flying/swim extra `xRot -=
/// 0.9599311` the `fallFlying || swimAmount > 0` branch rides, then the
/// DEGREE CLAMPS both rotations re-normalize through — `yRot = DEG ·
/// clamp(yRot·57.2957763671875, -60, 60)`, `xRot = DEG · clamp(xRot·
/// 57.2957763671875, -120, 30)` (the f32 180/π the exact decimal
/// speaks). The use gate (pcs 111-150): the FIELD `ticksUsingItem > 0`
/// and NOT (`isUsingItem` and `useItemHand` != this arm's hand — the
/// `right ? MAIN_HAND : OFF_HAND` mapping the `iload_2` branch keeps);
/// the kinetic null-return (pcs 163-168) the `None` params answer. The
/// kinetic sway (pcs 181-310): `yRot += -side·swayScaleFast·DEG·
/// swayIntensity·1.0`, `zRot += -side·swayScaleSlow·DEG·swayIntensity·
/// 0.5`, `xRot += DEG·(-40·raiseStart + 30·raiseMiddle - 20·raiseEnd +
/// 20·lower + 10·raiseBack + 0.6·swayScaleSlow·swayIntensity)` — the
/// jar's own DOUBLE intensity multiplication the getters' record keeps
/// (`swayScaleFast` already rides ×intensity, the fold multiplies
/// `swayIntensity()` AGAIN — the bytecode's own verdict the client
/// mirrors, never "fixes").
pub(crate) fn third_person_hand_use(
    x_rot: &mut f32,
    y_rot: &mut f32,
    z_rot: &mut f32,
    right: bool,
    fall_flying: bool,
    swim_amount: f32,
    ticks_using_item: f32,
    is_using_item: bool,
    use_item_main_hand: bool,
    use_params: Option<&crate::hand::SpearUseParams>,
) {
    const DEG: f32 = 0.017_453_292; // f32 bits 0x3c8efa35 — the CP float 0.01745329238474369
    const TO_DEG: f32 = 57.295_776_367_187_5; // the f32 180/π the exact decimal round-trips
    let side = if right { 1.0f32 } else { -1.0f32 };
    // pcs 11-48: the base raise — the yRot side nudge, the xRot's TWO
    // separate fadds (-π/2 then +0.8) the two putfield/getfield
    // round-trips wear
    *y_rot += -0.100_000_001_490_116_12 * side;
    *x_rot += -1.570_796_370_506_286_6;
    *x_rot += 0.800_000_011_920_929;
    // pcs 40-68: the fall-flying/swim extra — the `isFallFlying ||
    // swimAmount > 0` branch the ifne/ifle pair drives
    if fall_flying || swim_amount > 0.0 {
        *x_rot -= 0.959_931_075_572_967_5;
    }
    // pcs 69-108: the degree clamps — the rotation the degrees round
    // trip re-speaks (the window the spear's raised rest pose wears)
    *y_rot = DEG * (*y_rot * TO_DEG).clamp(-60.0, 60.0);
    *x_rot = DEG * (*x_rot * TO_DEG).clamp(-120.0, 30.0);
    // pcs 111-150: the use gate — the FIELD ticks the both-arms clock
    // keeps, the `!isUsingItem || useItemHand == hand` disjunction the
    // ifeq/goto chain answers
    let hand_is_main = right; // the `right ? MAIN_HAND : OFF_HAND` branch
    if !(ticks_using_item > 0.0 && (!is_using_item || use_item_main_hand == hand_is_main)) {
        return;
    }
    // pcs 151-168: the use stack's KINETIC_WEAPON null-return
    let Some(p) = use_params else {
        return;
    };
    // pcs 181-208: the yRot sway — the jar's own double intensity the
    // record's getters keep
    *y_rot += -side * p.sway_scale_fast * DEG * p.sway_intensity * 1.0;
    // pcs 211-239: the zRot sway — the slow scale, the ×0.5 the ldc
    // folds
    *z_rot += -side * p.sway_scale_slow * DEG * p.sway_intensity * 0.5;
    // pcs 242-310: the xRot kinetic window — the six-term sum the
    // stack's own order builds, the DEG the whole sum multiplies
    *x_rot += DEG
        * (-40.0 * p.raise_start
            + 30.0 * p.raise_middle
            + -20.0 * p.raise_end
            + 20.0 * p.lower
            + 10.0 * p.raise_back
            + 0.6 * p.sway_scale_slow * p.sway_intensity);
}

/// The six base parts with their overlay children, posed by `state`
/// (the setupAnim + setupAttackAnimation + crouch laws, verbatim).
fn build_posed_parts(state: &AvatarState) -> Vec<Part> {
    let pos = state.walk_pos;
    let speed = state.walk_speed;
    // speedValue's default 1.0 divides the classic terms (kept
    // explicit — the render state's own default)
    let speed_value = 1.0f32;
    let arm_r_x = mth_cos(pos * 0.6662 + std::f32::consts::PI) * 2.0 * speed * 0.5 / speed_value;
    let arm_l_x = mth_cos(pos * 0.6662) * 2.0 * speed * 0.5 / speed_value;
    let leg_r_x = mth_cos(pos * 0.6662) * 1.4 * speed / speed_value;
    let leg_l_x = mth_cos(pos * 0.6662 + std::f32::consts::PI) * 1.4 * speed / speed_value;
    // the idle bob (bobModelPart, always unless SPYGlass): the right
    // arm rides +1, the left -1
    let age = state.age_ticks;
    let bob = |s: f32| {
        (
            s * (mth_cos(age * 0.09) * 0.05 + 0.05),
            s * mth_sin(age * 0.067) * 0.05,
        )
    };
    let (bob_r_z, bob_r_x) = bob(1.0);
    let (bob_l_z, bob_l_x) = bob(-1.0);
    let mut arm_r_x = arm_r_x + bob_r_x;
    let mut arm_r_y = 0.0f32;
    let mut arm_r_z = 0.1 + bob_r_z;
    let mut arm_l_x = arm_l_x + bob_l_x;
    let mut arm_l_y = 0.0f32;
    let mut arm_l_z = -0.1 + bob_l_z;
    let mut body_y = 0.0f32;
    let mut body_x = 0.0f32;
    let head_x = state.pitch_deg.to_radians();
    let head_y = state.head_yaw_off_deg.to_radians();
    let leg_r_y = 0.005f32;
    let leg_r_z = 0.005f32;
    let leg_l_y = -0.005f32;
    let leg_l_z = -0.005f32;
    // the pivot nudges the crouch speaks (model texels). THE THIN
    // HUMANOID's leg pivot x (javap `SkeletonModel.
    // createDefaultSkeletonMesh` the PartPose.offset(±2, 12, 0) the
    // bytecode speaks — the wide ±1.9 the humanoid keeps) — the crouch
    // z nudge below the same array shares.
    let leg_pivot_x: f32 = if state.skeleton { 2.0 } else { 1.9 };
    let mut arm_r_pivot = [-5.0f32, 2.0, 0.0];
    let mut arm_l_pivot = [5.0f32, 2.0, 0.0];
    let mut body_pivot = [0.0f32, 0.0, 0.0];
    let mut head_pivot = [0.0f32, 0.0, 0.0];
    let mut leg_r_pivot = [-leg_pivot_x, 12.0, 0.0];
    let mut leg_l_pivot = [leg_pivot_x, 12.0, 0.0];
    if state.crouching {
        body_x = 0.5;
        arm_r_x += 0.4;
        arm_l_x += 0.4;
        leg_r_pivot[2] += 4.0;
        leg_l_pivot[2] += 4.0;
        head_pivot[1] += 4.2;
        body_pivot[1] += 3.2;
        arm_l_pivot[1] += 3.2;
        arm_r_pivot[1] += 3.2;
    }
    // THE POSE ARMS (HumanoidModel.setupAnim pcs 375-534, the 26.2
    // bytecode): mainArm RIGHT the client's own, the branch matrix the
    // isUsingItem/useItemHand/isTwoHanded flags drive — poseRightArm
    // first when the use hand matches the main arm, poseLeftArm first
    // otherwise (the idle branch the `mainRight != twoHanded` test
    // answers: no two-handed pose the round folds, the false the
    // SPEAR/EMPTY rows keep). FOUR folds the matrix dispatches (the
    // tableswitch the four verdicts the client's lanes answer): the
    // SPEAR case 10 — `thirdPersonHandUse(arm, head, right,
    // getUseItemStackForArm(arm), state)` the poseRightArm/poseLeftArm
    // pc-168 pair speaks — the BOW_AND_ARROW case 3 (pcs 174-249, the
    // constants CP #176 = -0.1 / #391 = 0.1 / #326 = 0.4 / #392 =
    // -pi/2 the cp_float resolve) the BOTH-ARMS assignment the
    // affectsOffhandPose flag wears — poseRightArm's body
    // `right.yRot = #176 + head.yRot; left.yRot = (#391 + head.yRot)
    // + #326; both xRot = #392 + head.xRot` the putfield order keeps,
    // poseLeftArm's the MIRROR the 0.4 the NON-bow arm carries
    // (`right.yRot = (#176 + head.yRot) - #326; left.yRot = #391 +
    // head.yRot` — the two bodies the f32 chains differ, never one
    // shared fold) — the BLOCK case 2 the poseBlockingArm call the
    // pcs 75-84 pair speaks (`poseBlockingArm(arm, right)` — the ONE-
    // ARM body the iconst flag answers: xRot = xRot·0.5 − 54° +
    // clamp(head.xRot, −80°, 25°), yRot = ±30°·DEG2RAD +
    // clamp(head.yRot, ±30°), the shield arm the RMB hold poses) —
    // and the ITEM case 1 the avatar law's own
    // fall-through wears (poseRightArm pc 87-116: `xRot = xRot·0.5 −
    // 0.31415927, yRot = 0` — the walk xRot the crouch's own +0.4 the
    // fold reads, the EMPTY case 0 the yRot zero the walk's own zeroes
    // never observe; the mob law the ITEM never answers —
    // HumanoidMobRenderer.getArmPose pc 59-62 the EMPTY the non-spear
    // stacks keep). The `affectsOffhandPose` skips: SPEAR and
    // BOW_AND_ARROW the true rows the reachable poses keep (the
    // clinit's flag pairs the dump banks — the bow's case-3 body the
    // off arm's posing already IS, the gate the second call cancels),
    // the other arm's posing the flag's truth cancels — BLOCK the
    // (2,0,0) pair the FALSE row keeps, the shield arm's one-arm
    // body the second posing never cancels.
    {
        let use_params = state.kinetic_use_params.as_ref();
        // THE TABLESWITCH'S OWN EIGHT CASES (the poseRightArm/
        // poseLeftArm pair the matrix dispatches): `Spear` the case-10
        // use lane rides, `Bow` the case-3 both-arms assignment,
        // `Block` the case-2 poseBlockingArm call, `Spyglass` the
        // case-7 clamp lane, `TootHorn` the case-8 clamp lane,
        // `CrossbowCharge` the case-5 both-arms body,
        // `CrossbowHold` the case-6 both-arms body, `ThrowTrident`
        // the case-4 π fold, `Brush` the case-9 π/5 fold, `Item` the
        // case-1 fold — the one closure the whole switch wears, the
        // f32 chains the jar's own bytecode keeps
        #[derive(Clone, Copy)]
        enum PoseCase {
            Spear,
            Bow,
            Block,
            Spyglass,
            TootHorn,
            CrossbowCharge,
            CrossbowHold,
            ThrowTrident,
            Brush,
            Item,
        }
        let mut fold = |right_arm: bool, kind: PoseCase| {
            match kind {
                PoseCase::Spear => {
                    let (x, y, z) = if right_arm {
                        (&mut arm_r_x, &mut arm_r_y, &mut arm_r_z)
                    } else {
                        (&mut arm_l_x, &mut arm_l_y, &mut arm_l_z)
                    };
                    crate::avatar::third_person_hand_use(
                        x,
                        y,
                        z,
                        right_arm,
                        state.fall_flying,
                        state.swim_amount,
                        state.ticks_using_item,
                        state.is_using_item,
                        state.use_item_main_hand,
                        use_params,
                    );
                }
                PoseCase::Bow => {
                    // THE BOW AND ARROW'S OWN BODIES (poseRightArm/
                    // poseLeftArm case 3, pcs 174-249 — the CP floats
                    // the cp_float_483 resolve: #176 = -0.1
                    // (0xBDCCCCCD), #391 = 0.1 (0x3DCCCCCD), #326 =
                    // 0.4 (0x3ECCCCCD), #392 = -1.5707963705062866
                    // (0xBFC90FDB)): the ASSIGNMENTS the putfield chain
                    // keeps — the walk's own swing the both arms' x/y
                    // the body wipes (the zRot the pose never touches),
                    // the head's pitch/yaw the four sums ride. The yRot
                    // chains the two bodies DIFFER: the right method's
                    // left arm the TWO fadds ((0.1 + hy) + 0.4), the
                    // left method's right arm the fadd-then-fsub
                    // ((-0.1 + hy) - 0.4) — the 0.4 the NON-bow arm's
                    // spread the jar's own bytecode keeps, the f32
                    // rounding each chain's own order wears.
                    const BOW_PITCH: f32 = -1.570_796_370_506_286_6; // CP #392
                    let hy = head_y;
                    let hx = head_x;
                    if right_arm {
                        arm_r_y = -0.100_000_001_490_116_12 + hy; // CP #176 + head.yRot
                        arm_l_y = (0.100_000_001_490_116_12 + hy) + 0.400_000_005_960_464_5; // (#391 + hy) + #326
                        arm_r_x = BOW_PITCH + hx; // CP #392 + head.xRot
                        arm_l_x = BOW_PITCH + hx;
                    } else {
                        arm_r_y = (-0.100_000_001_490_116_12 + hy) - 0.400_000_005_960_464_5; // (#176 + hy) - #326
                        arm_l_y = 0.100_000_001_490_116_12 + hy; // CP #391 + head.yRot
                        arm_r_x = BOW_PITCH + hx;
                        arm_l_x = BOW_PITCH + hx;
                    }
                }
                PoseCase::Block => {
                    // THE SHIELD'S OWN BODY (HumanoidModel.
                    // poseBlockingArm, the case-2 call the
                    // poseRightArm/poseLeftArm pcs 75-84 speak — the
                    // right arm the iconst_1 the left the iconst_0
                    // rides, the ONE-ARM body the flag answers; the
                    // CP floats the cp_float_484 resolve: #418 =
                    // 0.9424779 = 54° (0x3F71463B), #419 = -1.3962634
                    // = -80° (0xBFB2B8C2), #420 = 0.43633232 = 25°
                    // (0x3EDF66F3), #421 = -30.0, #422 = 30.0, #242 =
                    // 0.017453292 = DEG2RAD (0x3C8EFA35), #423/#417 =
                    // ∓0.5235988 = 30° (0xBF060A92/0x3F060A92)): the
                    // TWO putfields the body keeps — xRot = xRot·0.5
                    // − 54° + clamp(head.xRot, −80°, 25°), yRot =
                    // ±30°·DEG2RAD + clamp(head.yRot, ±30°) — the ±30
                    // the ARM SIDE the right flag answers (right the
                    // iconst_1 the −30 rides, the ifeq the left's +30
                    // keeps), the ×0.5 the exact exponent walk, the
                    // zRot the pose never touches. THE SIDE THE
                    // DEGREES SPEAK: −30·#242 the one fmul rounds to
                    // the SAME bits #423 keeps (0xBF060A92) — the
                    // constant pool the degrees pre-converted never
                    // stored, the fmul the conversion wears.
                    const BLOCK_TILT: f32 = 0.942_477_9; // CP #418, 54 deg
                    const PITCH_MIN: f32 = -1.396_263_4; // CP #419, -80 deg
                    const PITCH_MAX: f32 = 0.436_332_32; // CP #420, 25 deg
                    const YAW_CLAMP: f32 = 0.523_598_8; // CP #417/#423, 30 deg
                    const DEG2RAD: f32 = 0.017_453_292; // CP #242
                    let (x, y) = if right_arm {
                        (&mut arm_r_x, &mut arm_r_y)
                    } else {
                        (&mut arm_l_x, &mut arm_l_y)
                    };
                    *x = (*x * 0.5 - BLOCK_TILT)
                        + head_x.clamp(PITCH_MIN, PITCH_MAX);
                    let side = if right_arm { -30.0f32 } else { 30.0f32 };
                    *y = (side * DEG2RAD)
                        + head_y.clamp(-YAW_CLAMP, YAW_CLAMP);
                }
                PoseCase::Spyglass => {
                    // THE SPYGLASS'S OWN BODY (poseRightArm/
                    // poseLeftArm case 7, pcs 326-386 — the CP floats
                    // the cp_float_485 resolve: #408 = 1.9198622
                    // (0x3FF5BE0B), #409 = 0.2617994 (0x3E860A92),
                    // #410 = −2.4 (0xC019999A), #411 = 3.3
                    // (0x40533333)): the PURE ASSIGNMENT the head's
                    // own pitch drives (no walk read the value chain
                    // keeps — the swing the body wipes), the crouch's
                    // own 15° the isCrouching branch adds, the clamp
                    // the −2.4/3.3 window rides. THE YROT THE SIDE
                    // MIRRORS: the right method the fsub (head.yRot −
                    // #409), the left the fadd (head.yRot + #409) —
                    // the jar's own asymmetry speaks, the zRot the
                    // pose never touches.
                    const PITCH_OFF: f32 = 1.919_862_2; // CP #408, 110 deg
                    const CROUCH_OFF: f32 = 0.261_799_4; // CP #409, 15 deg
                    const CLAMP_MIN: f32 = -2.4; // CP #410
                    const CLAMP_MAX: f32 = 3.3; // CP #411
                    let (x, y) = if right_arm {
                        (&mut arm_r_x, &mut arm_r_y)
                    } else {
                        (&mut arm_l_x, &mut arm_l_y)
                    };
                    *x = (head_x
                        - PITCH_OFF
                        - if state.crouching {
                            CROUCH_OFF
                        } else {
                            0.0
                        })
                    .clamp(CLAMP_MIN, CLAMP_MAX);
                    *y = if right_arm {
                        head_y - CROUCH_OFF
                    } else {
                        head_y + CROUCH_OFF
                    };
                }
                PoseCase::TootHorn => {
                    // THE HORN'S OWN BODY (case 8, pcs 389-430 — CP
                    // #187 = −1.2 (0xBF99999A), #415 = 1.2
                    // (0x3F99999A), #416 = 1.4835298 (0x3FBDE44E),
                    // #417 = 0.5235988 (0x3F060A92) — the BLOCK's
                    // #417 the same bits the 30° keeps): xRot =
                    // clamp(head.xRot, ±1.2) − 1.4835298 the pure
                    // assignment rides (no walk read the chain keeps),
                    // the yRot the side mirrors (the right the fsub,
                    // the left the fadd — the ±30° the mouth's aim
                    // speaks), the zRot the pose never touches.
                    const CLAMP_MIN: f32 = -1.2; // CP #187
                    const CLAMP_MAX: f32 = 1.2; // CP #415
                    const PITCH_OFF: f32 = 1.483_529_8; // CP #416
                    const YAW_OFF: f32 = 0.523_598_8; // CP #417, 30 deg
                    let (x, y) = if right_arm {
                        (&mut arm_r_x, &mut arm_r_y)
                    } else {
                        (&mut arm_l_x, &mut arm_l_y)
                    };
                    *x = head_x.clamp(CLAMP_MIN, CLAMP_MAX) - PITCH_OFF;
                    *y = if right_arm {
                        head_y - YAW_OFF
                    } else {
                        head_y + YAW_OFF
                    };
                }
                PoseCase::CrossbowCharge => {
                    // THE CHARGE'S OWN BODY (AnimationUtils.
                    // animateCrossbowCharge, the 58-byte whole the
                    // full_disasm banks — the CP floats the
                    // cp_float_486 resolves: #25 = −0.8 (0xBF4CCCCD),
                    // #26 = 0.8 (0x3F4CCCCD), #27 = −0.9707963
                    // (0xBF78861C), #32 = 0.4 (0x3ECCCCCD), #33 =
                    // 0.85 (0x3F59999A), #19 = −1.5707963705062866
                    // (0xBFC90FDB) = −π/2): the BOTH-ARMS body the
                    // ONE call carries (the (5,1,1) pair the
                    // twoHanded row keeps) — the charge arm the right
                    // flag answers (the poseRightArm call the
                    // iconst_1, the poseLeftArm the iconst_0), the
                    // other arm the SUPPORT the body's own second
                    // half poses. THE LERP THE JAR'S OWN CHAIN KEEPS
                    // (Mth.lerp(delta, start, end) = f32(start +
                    // f32(delta · f32(end − start))) — the fsub/fmul/
                    // fadd one rounding each wears): the pull the
                    // f1 = f32(clamp(ticks, 0.0, max) / max) drives.
                    const CHARGE_Y_R: f32 = -0.8; // CP #25
                    const CHARGE_Y_L: f32 = 0.8; // CP #26
                    const CHARGE_PITCH: f32 = -0.970_796_346_664_428_7; // CP #27
                    const LERP_START: f32 = 0.4; // CP #32
                    const LERP_END: f32 = 0.85; // CP #33
                    const HOLD_PITCH: f32 = -1.570_796_370_506_286_6; // CP #19
                    let (charge_x, charge_y, other_x, other_y) = if right_arm {
                        (
                            &mut arm_r_x,
                            &mut arm_r_y,
                            &mut arm_l_x,
                            &mut arm_l_y,
                        )
                    } else {
                        (
                            &mut arm_l_x,
                            &mut arm_l_y,
                            &mut arm_r_x,
                            &mut arm_r_y,
                        )
                    };
                    *charge_y = if right_arm {
                        CHARGE_Y_R
                    } else {
                        CHARGE_Y_L
                    };
                    *charge_x = CHARGE_PITCH;
                    // the other arm's xRot the charge arm's own
                    // pitch the float copy keeps (pc 48-55)
                    *other_x = CHARGE_PITCH;
                    let max = state.max_crossbow_charge_duration;
                    let clamped = state
                        .ticks_using_item
                        .clamp(0.0, max);
                    let f1 = clamped / max;
                    // the yRot the lerp(f1, 0.4, 0.85) the side's ±1
                    // the fmul answers (pc 72-95)
                    let spread = LERP_START + f1 * (LERP_END - LERP_START);
                    *other_y = spread * if right_arm { 1.0f32 } else { -1.0f32 };
                    // the xRot the lerp(f1, other.xRot(= #27), −π/2)
                    // the pull walks (pc 98-112)
                    *other_x = CHARGE_PITCH + f1 * (HOLD_PITCH - CHARGE_PITCH);
                }
                PoseCase::CrossbowHold => {
                    // THE HOLD'S OWN BODY (AnimationUtils.
                    // animateCrossbowHold, the 49-byte whole — CP #7 =
                    // −0.3 (0xBE99999A), #8 = 0.3 (0x3E99999A), #15 =
                    // 0.6 (0x3F19999A), #16 = −0.6 (0xBF19999A), #19
                    // = −π/2, #23 = 0.1 (0x3DCCCCCD), #24 = −1.5
                    // (0xBFC00000)): the BOTH-ARMS body the ONE call
                    // carries — the head's own pitch/yaw the four
                    // sums ride, the charge arm's xRot the (#19 +
                    // head.xRot) + #23 the TWO fadds keeps, the other
                    // arm the #24 + head.xRot the one fadd speaks.
                    const HOLD_Y_R: f32 = -0.3; // CP #7
                    const HOLD_Y_L: f32 = 0.3; // CP #8
                    const OTHER_Y_R: f32 = 0.6; // CP #15
                    const OTHER_Y_L: f32 = -0.6; // CP #16
                    const CHARGE_PITCH: f32 = -1.570_796_370_506_286_6; // CP #19
                    const PITCH_ADD: f32 = 0.1; // CP #23
                    const OTHER_PITCH: f32 = -1.5; // CP #24
                    let (charge_x, charge_y, other_x, other_y) = if right_arm {
                        (
                            &mut arm_r_x,
                            &mut arm_r_y,
                            &mut arm_l_x,
                            &mut arm_l_y,
                        )
                    } else {
                        (
                            &mut arm_l_x,
                            &mut arm_l_y,
                            &mut arm_r_x,
                            &mut arm_r_y,
                        )
                    };
                    *charge_y = (if right_arm {
                        HOLD_Y_R
                    } else {
                        HOLD_Y_L
                    }) + head_y;
                    *other_y = (if right_arm {
                        OTHER_Y_R
                    } else {
                        OTHER_Y_L
                    }) + head_y;
                    *charge_x = (CHARGE_PITCH + head_x) + PITCH_ADD;
                    *other_x = OTHER_PITCH + head_x;
                }
                PoseCase::ThrowTrident => {
                    // THE THROW TRIDENT'S OWN BODY (poseRightArm/
                    // poseLeftArm case 4, pcs 119-148 — the CP floats
                    // the cp_527 resolves: #95 = 0.5 (0x3F000000),
                    // #260 = 3.1415927410125732 = π (0x40490FDB)): the
                    // ONE-ARM fold the walk's own xRot READS — xRot =
                    // xRot·0.5 − π the fmul/fsub chain the putfield
                    // order keeps (the ×0.5 the exact exponent walk,
                    // the fsub one rounding; the crouch's own +0.4
                    // raise the fold reads the walk law the ITEM fold
                    // shares), the yRot zero the fconst_0 putfield
                    // replays, the zRot the pose never touches, the
                    // head the body NEVER reads. THE VERDICT THE USE
                    // GATE'S TRIDENT BRANCH ANSWERS (AvatarRenderer.
                    // getArmPose pcs 81-91 — the use-anim walk's third
                    // acmp, the BLOCK/BOW branches the first two
                    // wear): the USING stack's getUseAnimation() ==
                    // TRIDENT the RMB charge the verdict wears, the
                    // idle trident the SPEARS tag the row table
                    // excludes the ITEM fall-through keeps.
                    const THROW_PI: f32 = 3.141_592_741_012_573_2; // CP #260, pi
                    let (x, y) = if right_arm {
                        (&mut arm_r_x, &mut arm_r_y)
                    } else {
                        (&mut arm_l_x, &mut arm_l_y)
                    };
                    *x = *x * 0.5 - THROW_PI;
                    *y = 0.0;
                }
                PoseCase::Brush => {
                    // THE BRUSH'S OWN BODY (poseRightArm/poseLeftArm
                    // case 9, pcs 294-325 — the constants the dump
                    // banks: 0.5 (0x3F000000), 0.6283185482025146 =
                    // π/5 (0x3F20D97C)): the ONE-ARM fold the walk's
                    // own xRot READS — xRot = xRot·0.5 − π/5 the
                    // fmul/fsub chain the putfield order keeps (the
                    // ×0.5 the exact exponent walk, the fsub one
                    // rounding; the crouch's own +0.4 raise the fold
                    // reads the walk law the ITEM fold shares), the
                    // yRot zero the fconst_0 putfield replays, the
                    // zRot the pose never touches, the head the body
                    // NEVER reads. THE VERDICT THE USE GATE'S BRUSH
                    // BRANCH ANSWERS (AvatarRenderer.getArmPose pcs
                    // 125-135 — the use-anim walk's sixth acmp, the
                    // TOOT_HORN branch the fifth wears, the SPEAR
                    // branch the seventh keeps): the USING stack's
                    // getUseAnimation() == BRUSH the brushing hold
                    // the verdict wears, the idle brush no verdict
                    // the ITEM fall-through keeps. THE CLINIT'S OWN
                    // FLAG PAIR: BRUSH(9,0,0) — twoHanded 0,
                    // affectsOffhandPose 0 — the gate the second
                    // posing never cancels (the FALSE row the BLOCK
                    // pair shares).
                    const BRUSH_PI_5: f32 = 0.628_318_548_202_514_6; // CP, pi/5
                    let (x, y) = if right_arm {
                        (&mut arm_r_x, &mut arm_r_y)
                    } else {
                        (&mut arm_l_x, &mut arm_l_y)
                    };
                    *x = *x * 0.5 - BRUSH_PI_5;
                    *y = 0.0;
                }
                PoseCase::Item => {
                    // the ITEM case-1 fold — the f32 chain the jar's
                    // fmul/fsub keeps (×0.5 the exact exponent walk,
                    // the fsub one rounding; the yRot zero the walk's
                    // own 0.0 the putfield replays, both arms the same
                    // constants — poseLeftArm pc 87-116 the mirror
                    // speaks)
                    let (x, y) = if right_arm {
                        (&mut arm_r_x, &mut arm_r_y)
                    } else {
                        (&mut arm_l_x, &mut arm_l_y)
                    };
                    *x = *x * 0.5 - 0.314_159_274_101_257_3;
                    *y = 0.0;
                }
            }
        };
        if state.is_using_item && state.use_item_main_hand {
            // pcs 416-438: useMain == mainRight — poseRightArm, then
            // the left the right pose's affectsOffhandPose gates
            if state.right_bow {
                // case 3: the BOTH-ARMS body — the left arm's posing
                // the body itself carries, the gate below the second
                // call cancels
                fold(true, PoseCase::Bow);
            } else if state.right_block {
                // case 2: the ONE-ARM body — the shield arm alone
                // poses, the affectsOffhand flag FALSE the gate below
                // the second posing never cancels
                fold(true, PoseCase::Block);
            } else if state.right_spyglass {
                // case 7: the (7,0,0) pair — the FALSE row the second
                // posing never cancels
                fold(true, PoseCase::Spyglass);
            } else if state.right_horn {
                // case 8: the (8,0,0) pair — the FALSE row the second
                // posing never cancels
                fold(true, PoseCase::TootHorn);
            } else if state.right_crossbow_charge {
                // case 5: the (5,1,1) pair — the BOTH-ARMS body the
                // one call carries, the affectsOffhand flag TRUE the
                // gate below the second posing CANCELS (the bow's
                // true row the same law keeps; the two-arg getArmPose
                // walk's isTwoHanded rewrite the off verdict the
                // cancelled posing never reads)
                fold(true, PoseCase::CrossbowCharge);
            } else if state.right_crossbow_hold {
                // case 6: the (6,1,1) pair — the BOTH-ARMS body, the
                // gate the second posing CANCELS
                fold(true, PoseCase::CrossbowHold);
            } else if state.right_trident {
                // case 4: the (4,0,1) pair — the ONE-ARM body the
                // walk's own xRot the π fold reads, the
                // affectsOffhand flag TRUE the gate below the second
                // posing CANCELS (the use gate's own TRIDENT branch
                // the verdict wears)
                fold(true, PoseCase::ThrowTrident);
            } else if state.right_brush {
                // case 9: the (9,0,0) pair — the ONE-ARM body the
                // walk's own xRot the π/5 fold reads, the
                // affectsOffhand flag FALSE the gate below the
                // second posing never cancels (the use gate's own
                // BRUSH branch the verdict wears)
                fold(true, PoseCase::Brush);
            } else if state.right_spear {
                fold(true, PoseCase::Spear);
            } else if state.right_item {
                fold(true, PoseCase::Item);
            }
            if !state.right_spear
                && !state.right_bow
                && !state.right_crossbow_charge
                && !state.right_crossbow_hold
                && !state.right_trident
            {
                if state.left_bow {
                    fold(false, PoseCase::Bow);
                } else if state.left_block {
                    fold(false, PoseCase::Block);
                } else if state.left_spyglass {
                    fold(false, PoseCase::Spyglass);
                } else if state.left_horn {
                    fold(false, PoseCase::TootHorn);
                } else if state.left_crossbow_charge {
                    fold(false, PoseCase::CrossbowCharge);
                } else if state.left_crossbow_hold {
                    fold(false, PoseCase::CrossbowHold);
                } else if state.left_spear {
                    fold(false, PoseCase::Spear);
                } else if state.left_item {
                    fold(false, PoseCase::Item);
                }
            }
        } else {
            // pcs 446-463 (the off-hand use) and pcs 469-514 (the idle
            // branch) both speak poseLeftArm first — the same order
            // the left pose's affectsOffhandPose gates
            if state.left_bow {
                // case 3 mirror: the left method's own body
                fold(false, PoseCase::Bow);
            } else if state.left_block {
                // case 2 mirror: the iconst_0 the +30 side rides
                fold(false, PoseCase::Block);
            } else if state.left_spyglass {
                // case 7 mirror: the fadd the left body's own rides
                fold(false, PoseCase::Spyglass);
            } else if state.left_horn {
                // case 8 mirror: the fadd the left body's own rides
                fold(false, PoseCase::TootHorn);
            } else if state.left_crossbow_charge {
                // case 5 mirror: the iconst_0 the left the CHARGE arm
                // rides, the affectsOffhand TRUE the gate the second
                // posing cancels
                fold(false, PoseCase::CrossbowCharge);
            } else if state.left_crossbow_hold {
                // case 6 mirror: the iconst_0 the left the HOLD arm
                // rides, the gate the second posing cancels
                fold(false, PoseCase::CrossbowHold);
            } else if state.left_trident {
                // case 4 mirror: the ONE-ARM body the left method's
                // own pc 119-148 the same constants keeps, the
                // affectsOffhand TRUE the gate the second posing
                // cancels
                fold(false, PoseCase::ThrowTrident);
            } else if state.left_brush {
                // case 9 mirror: the left method's own pc 294-323
                // the same constants keeps, the affectsOffhand FALSE
                // the gate the second posing never cancels
                fold(false, PoseCase::Brush);
            } else if state.left_spear {
                fold(false, PoseCase::Spear);
            } else if state.left_item {
                fold(false, PoseCase::Item);
            }
            if !state.left_spear
                && !state.left_bow
                && !state.left_crossbow_charge
                && !state.left_crossbow_hold
                && !state.left_trident
            {
                if state.right_bow {
                    fold(true, PoseCase::Bow);
                } else if state.right_block {
                    fold(true, PoseCase::Block);
                } else if state.right_spyglass {
                    fold(true, PoseCase::Spyglass);
                } else if state.right_horn {
                    fold(true, PoseCase::TootHorn);
                } else if state.right_crossbow_charge {
                    fold(true, PoseCase::CrossbowCharge);
                } else if state.right_crossbow_hold {
                    fold(true, PoseCase::CrossbowHold);
                } else if state.right_trident {
                    fold(true, PoseCase::ThrowTrident);
                } else if state.right_brush {
                    fold(true, PoseCase::Brush);
                } else if state.right_spear {
                    fold(true, PoseCase::Spear);
                } else if state.right_item {
                    fold(true, PoseCase::Item);
                }
            }
        }
    }
    // THE SWING (setupAttackAnimation, 26.2 verbatim): the shared folds
    // run for EVERY swing kind (the body's yRot twist, the arms' orbit
    // pivots pc 64-155, the three rotation adds pc 158-212), then the
    // $SwitchMap$SwingAnimationType dispatch speaks (WHACK 1 the
    // outQuart arc at pc 252, NONE 2 the bare goto at pc 372, STAB 3
    // `SpearAnimations.thirdPersonAttackHand` at pc 375-377).
    let at = state.attack_time;
    if at > 0.0 {
        let mut body_rot = 0.2 * mth_sin(mth_sqrt(at) * 6.2831855);
        if !state.attack_right {
            body_rot = -body_rot;
        }
        body_y = body_rot;
        // THE ORBIT (pc 64-155): the four pivot ASSIGNMENTS — the arms
        // swing AROUND the body's twist (±cos/±sin of body.yRot, the
        // ldc 5.0 the texel radius, the ageScale the render state's
        // own f3 the four folds share)
        arm_r_pivot[2] = mth_sin(body_y) * 5.0 * state.age_scale;
        arm_r_pivot[0] = -(mth_cos(body_y)) * 5.0 * state.age_scale;
        arm_l_pivot[2] = -(mth_sin(body_y)) * 5.0 * state.age_scale;
        arm_l_pivot[0] = mth_cos(body_y) * 5.0 * state.age_scale;
        // THE SHARED ROTATIONS (pc 158-212): rightArm.yRot += body.yRot,
        // leftArm.yRot += body.yRot, leftArm.xRot += body.yRot — the
        // three adds the STAB undo then replays (the putfield/getfield
        // round-trip the jar's own chain keeps, the f32 the drift wears)
        arm_r_y += body_y;
        arm_l_y += body_y;
        arm_l_x += body_y;
        // THE DISPATCH (HumanoidModel$1 clinit: $SwitchMap[WHACK] = 1,
        // [NONE] = 2, [STAB] = 3 — the tableswitch [1->252, 2->372,
        // 3->375] the dump speaks)
        match state.swing_kind {
            crate::hand::SwingKind::Whack => {
                // THE WHACK ARC (pc 252-369): f5 = sin(outQuart(t)·π)
                // (the Ease.outQuart the 26.2 verdict — the 1.8-era
                // sin(sqrt(t)·π) the old law kept the jar retired), f6
                // = sin(t·π)·(0.7 − head.xRot)·0.75 the head's own
                // ride, the attack arm the getArm(attackArm) answers
                let f5 = mth_sin(crate::hand::ease_out_quart(at) * std::f32::consts::PI);
                let f6 = mth_sin(at * std::f32::consts::PI) * (0.7 - head_x) * 0.75;
                if state.attack_right {
                    arm_r_x -= f5 * 1.2 + f6;
                    arm_r_y += body_y * 2.0;
                    arm_r_z += mth_sin(at * std::f32::consts::PI) * -0.4;
                } else {
                    arm_l_x -= f5 * 1.2 + f6;
                    arm_l_y += body_y * 2.0;
                    arm_l_z += mth_sin(at * std::f32::consts::PI) * -0.4;
                }
            }
            crate::hand::SwingKind::None => {
                // pc 372: the bare goto 380 — the shared folds the kind
                // keeps, no arc the switch adds
            }
            crate::hand::SwingKind::Stab => {
                // THE STAB (SpearAnimations.thirdPersonAttackHand pcs
                // 0-133): the three shared adds UNDO (rightArm.yRot -=,
                // leftArm.yRot -=, leftArm.xRot -= body.yRot — the
                // round-trip the f32 double-rounding the jar's own
                // putfield chain wears), then the xRot window
                // (90·f4 - 120·f5 + 30·f6)·(π/180) the helper folds —
                // the getArm(attackArm) the ONLY arm the window rides
                arm_r_y -= body_y;
                arm_l_y -= body_y;
                arm_l_x -= body_y;
                let delta = crate::hand::third_person_stab_x_rot_delta(at);
                if state.attack_right {
                    arm_r_x += delta;
                } else {
                    arm_l_x += delta;
                }
            }
        }
    }
    // THE ZOMBIE ARMS (javap `AnimationUtils.animateZombieArms` +
    // `animateAttackArms` on the official 26.2 jar): the attack-arms
    // branch ASSIGNS both arms' rotations — the humanoid walk swing
    // and the first bob ride gets wiped, the arms hold the vanilla
    // outstretched pose (base -PI/(aggressive ? 1.5 : 2.25)) with the
    // attackTime bump, then bobArms runs a SECOND pass INVERTED (the
    // animateZombieArms call orders (left, right) — the left rides
    // +1, the right -1, opposite the setupAnim pass). The body's
    // swing fold and the crouch pivots survive untouched.
    if state.zombie {
        let f5 = mth_sin(at * std::f32::consts::PI);
        let f6 = mth_sin((at - at * at) * std::f32::consts::PI);
        let base = -std::f32::consts::PI
            / if state.zombie_aggressive { 1.5 } else { 2.25 };
        let x_rot = base + f5 * 1.2 - f6 * 0.4;
        let f8 = 0.1 - f5 * 0.6;
        // the inverted second bob (the right rides -1, the left +1)
        let (zb_r, xb_r) = bob(-1.0);
        let (zb_l, xb_l) = bob(1.0);
        arm_r_x = x_rot + xb_r;
        arm_r_y = -f8;
        arm_r_z = zb_r;
        arm_l_x = x_rot + xb_l;
        arm_l_y = f8;
        arm_l_z = zb_l;
    }
    // THE SKELETON'S OWN POSES (javap `SkeletonModel.setupAnim` the
    // 26.2 bytecode speaks — the super's humanoid walk ran above, the
    // branch the render state's two words gate). THE AGGRESSIVE BOW
    // (isAggressive && isHoldingBow — the SkeletonModel's own branch
    // never runs, the BOW_AND_ARROW ArmPose the renderer's getArmPose
    // arms — mainArm == RIGHT the default keeps, isTwoHanded() the
    // equal-flag branch keeps, affectsOffhandPose() the poseLeftArm
    // skip rides): the aim overwrites BOTH arms' xRot/yRot the
    // poseRightArm case-3 body assigns (head.xRot the pitch rides,
    // head.yRot the yaw offset — the head terms the +0.4 left yRot
    // shares), the zRot the walk's bob keeps (the pose never touches).
    if state.skeleton && state.skeleton_aggressive && state.skeleton_bow {
        let hy = head_y;
        let hx = head_x;
        arm_r_x = -1.5707964 + hx;
        arm_r_y = -0.1 + hy;
        arm_l_x = -1.5707964 + hx;
        arm_l_y = 0.1 + hy + 0.4;
    }
    // THE AGGRESSIVE BARE (isAggressive && !isHoldingBow — the
    // SkeletonModel's OWN branch the bytecode assigns: the f1 the
    // attackTime's sin(πt), the f2 the sin((1 - (1-t)²)·π) —
    // `fconst_1 fsub fconst_1 fsub` the (1-t)(1-t) fold — the zRot
    // ZEROED, the yRot the ∓(0.1 - f1·0.6) mirror arms, the xRot
    // -π/2 - f1·1.2 - f2·0.4 the dup-getfield-sub-sub walks, then the
    // bobArms(right, left, age) the NORMAL sign order adds (the right
    // +1, the left −1 — the zombie's INVERTED second bob the call
    // never shares)).
    if state.skeleton && state.skeleton_aggressive && !state.skeleton_bow {
        let f1 = mth_sin(at * std::f32::consts::PI);
        let f2 = mth_sin((1.0 - (1.0 - at) * (1.0 - at)) * std::f32::consts::PI);
        let raise = -1.5707964 - f1 * 1.2 - f2 * 0.4;
        let (bz_r, bx_r) = bob(1.0);
        let (bz_l, bx_l) = bob(-1.0);
        arm_r_z = bz_r;
        arm_l_z = bz_l;
        arm_r_y = -(0.1 - f1 * 0.6);
        arm_l_y = 0.1 - f1 * 0.6;
        arm_r_x = raise + bx_r;
        arm_l_x = raise + bx_l;
    }
    // THE PARTS (createMesh, verbatim). The overlays are CHILDREN
    // (PartPose.ZERO — the parent's pivot space) with the SAME box
    // inflated by the extend. THE THIN HUMANOID's boxes (javap
    // `SkeletonModel.createDefaultSkeletonMesh` the 26.2 bytecode
    // speaks): the arms 2x12x2 at tex (40,16) — the LEFT the builder's
    // own .mirror() the u flip rides (the Cube.mirror flag the
    // quadruped legs speak) — the legs 2x12x2 at (0,16) the same
    // mirror law keeps, the sleeve/pants overlay children the thin
    // builder never fills (the addOrReplaceChild REPLACES the limb —
    // no child the half-height layout could unwrap). The wide limbs
    // the humanoid's own rows keep (the left pair the modern 64x64
    // rects unwrapped — no mirror the own rect never asks).
    // the RIGHT arm the wide layout's own row keeps (the box spans
    // x -3..1 the wide right arm speaks — the LEFT the -1..3 row — the
    // thin limbs the symmetric ±1 both sides share).
    let (arm_r_box, arm_w) = if state.skeleton {
        ([-1.0f32, -2.0, -1.0], [2.0f32, 12.0, 2.0])
    } else {
        ([-3.0f32, -2.0, -2.0], [4.0f32, 12.0, 4.0])
    };
    let arm_l_box: [f32; 3] = if state.skeleton {
        [-1.0, -2.0, -1.0]
    } else {
        [-1.0, -2.0, -2.0]
    };
    let (leg_box, leg_w) = if state.skeleton {
        ([-1.0f32, 0.0, -1.0], [2.0f32, 12.0, 2.0])
    } else {
        ([-2.0f32, 0.0, -2.0], [4.0f32, 12.0, 4.0])
    };
    vec![
        // head (+ hat child)
        Part {
            pivot: head_pivot,
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 0.0),
                origin: [-4.0, -8.0, -4.0],
                size: [8.0, 8.0, 8.0],
                inflate: 0.0,
                        mirror: false,
                        }],
            children: vec![Part {
                pivot: [0.0, 0.0, 0.0],
                x_rot: 0.0,
                y_rot: 0.0,
                z_rot: 0.0,
                cubes: vec![Cube {
                    tex: (40.0, 0.0),
                    origin: [-4.0, -8.0, -4.0],
                    size: [8.0, 8.0, 8.0],
                    inflate: 0.5,
                                mirror: false,
                                }],
                children: vec![],
            }],
        },
        // body (+ jacket child)
        Part {
            pivot: body_pivot,
            x_rot: body_x,
            y_rot: body_y,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (16.0, 16.0),
                origin: [-4.0, 0.0, -2.0],
                size: [8.0, 12.0, 4.0],
                inflate: 0.0,
                        mirror: false,
                        }],
            children: vec![Part {
                pivot: [0.0, 0.0, 0.0],
                x_rot: 0.0,
                y_rot: 0.0,
                z_rot: 0.0,
                cubes: vec![Cube {
                    tex: (16.0, 32.0),
                    origin: [-4.0, 0.0, -2.0],
                    size: [8.0, 12.0, 4.0],
                    inflate: 0.25,
                                mirror: false,
                                }],
                children: vec![],
            }],
        },
        // right arm (+ right sleeve child — the thin limb never fills)
        Part {
            pivot: arm_r_pivot,
            x_rot: arm_r_x,
            y_rot: arm_r_y,
            z_rot: arm_r_z,
            cubes: vec![Cube {
                tex: (40.0, 16.0),
                origin: arm_r_box,
                size: arm_w,
                inflate: 0.0,
                        mirror: false,
                        }],
            children: if state.skeleton {
                vec![]
            } else {
                vec![Part {
                    pivot: [0.0, 0.0, 0.0],
                    x_rot: 0.0,
                    y_rot: 0.0,
                    z_rot: 0.0,
                    cubes: vec![Cube {
                        tex: (40.0, 32.0),
                        origin: [-3.0, -2.0, -2.0],
                        size: [4.0, 12.0, 4.0],
                        inflate: 0.25,
                                    mirror: false,
                                    }],
                    children: vec![],
                }]
            },
        },
        // left arm (+ left sleeve child — the thin limb rides the
        // builder's own .mirror() at the SHARED (40,16) row)
        Part {
            pivot: arm_l_pivot,
            x_rot: arm_l_x,
            y_rot: arm_l_y,
            z_rot: arm_l_z,
            cubes: vec![Cube {
                tex: if state.skeleton { (40.0, 16.0) } else { (32.0, 48.0) },
                origin: if state.skeleton { arm_l_box } else { [-1.0, -2.0, -2.0] },
                size: arm_w,
                inflate: 0.0,
                        mirror: state.skeleton,
                        }],
            children: if state.skeleton {
                vec![]
            } else {
                vec![Part {
                    pivot: [0.0, 0.0, 0.0],
                    x_rot: 0.0,
                    y_rot: 0.0,
                    z_rot: 0.0,
                    cubes: vec![Cube {
                        tex: (48.0, 48.0),
                        origin: [-1.0, -2.0, -2.0],
                        size: [4.0, 12.0, 4.0],
                        inflate: 0.25,
                                    mirror: false,
                                    }],
                    children: vec![],
                }]
            },
        },
        // right leg (+ right pants child — the thin limb never fills)
        Part {
            pivot: leg_r_pivot,
            x_rot: leg_r_x,
            y_rot: leg_r_y,
            z_rot: leg_r_z,
            cubes: vec![Cube {
                tex: (0.0, 16.0),
                origin: leg_box,
                size: leg_w,
                inflate: 0.0,
                        mirror: false,
                        }],
            children: if state.skeleton {
                vec![]
            } else {
                vec![Part {
                    pivot: [0.0, 0.0, 0.0],
                    x_rot: 0.0,
                    y_rot: 0.0,
                    z_rot: 0.0,
                    cubes: vec![Cube {
                        tex: (0.0, 32.0),
                        origin: [-2.0, 0.0, -2.0],
                        size: [4.0, 12.0, 4.0],
                        inflate: 0.25,
                                    mirror: false,
                                    }],
                    children: vec![],
                }]
            },
        },
        // left leg (+ left pants child — the thin limb rides the
        // builder's own .mirror() at the SHARED (0,16) row)
        Part {
            pivot: leg_l_pivot,
            x_rot: leg_l_x,
            y_rot: leg_l_y,
            z_rot: leg_l_z,
            cubes: vec![Cube {
                tex: if state.skeleton { (0.0, 16.0) } else { (16.0, 48.0) },
                origin: if state.skeleton { leg_box } else { [-2.0, 0.0, -2.0] },
                size: leg_w,
                inflate: 0.0,
                        mirror: state.skeleton,
                        }],
            children: if state.skeleton {
                vec![]
            } else {
                vec![Part {
                    pivot: [0.0, 0.0, 0.0],
                    x_rot: 0.0,
                    y_rot: 0.0,
                    z_rot: 0.0,
                    cubes: vec![Cube {
                        tex: (0.0, 48.0),
                        origin: [-2.0, 0.0, -2.0],
                        size: [4.0, 12.0, 4.0],
                        inflate: 0.25,
                                    mirror: false,
                                    }],
                    children: vec![],
                }]
            },
        },
    ]
}

// ---------------------------------------------------------------------------
// THE BOAT MODEL (AbstractBoatRenderer.submit + the model/object/boat family,
// the 26.2 jar's own bytecode): the boat entity's hull the world draws — the
// BoatModel's five hull parts (bottom/back/front/right/left) + the two
// paddles, the RaftModel's two-plank bottom (the bamboo family's own hull),
// the chest variant's three cubes riding the chest texture's own UV band.
// ---------------------------------------------------------------------------

/// The boat textures' own sha1 table (the 26.2 jar's verbatim bytes, hashed
/// by scripts: `sha1sum` over the extracted entity/boat + entity/chest_boat
/// PNGs) — the load gate every other model speaks (a drifted extract keeps
/// the boat unseen, the honest missing-asset law). The wood the registry
/// named resolves here: an unknown wood (`None`) never draws.
fn boat_texture_sha1(chest: bool, wood: &str) -> Option<&'static str> {
    if chest {
        Some(match wood {
            "acacia" => "66035134c4a6331af983a9608b3c7ecd00bb0572",
            "bamboo" => "14e6b35ecd5c2285c866286d5d3782eb43ecdfec",
            "birch" => "c9e7a72e030ec598fb1871f322c0e74d992a46eb",
            "cherry" => "df74908ef192bba9bb2748dcdf9c2befbf2d718f",
            "dark_oak" => "fe1ff0360e30cf34241f9b8849a325cc54e025d4",
            "jungle" => "bdf9554c065cf91e0e9322a10457e1b1382f1adc",
            "mangrove" => "884bcf6c6b57f512a1a05ac61e232f5b0de15c9a",
            "oak" => "4bd1b1b519a6d1a626e77f8de476cd13d07ac0cc",
            "pale_oak" => "b5ff45affe5766ba8f89e9b328595296f0711f33",
            "spruce" => "5c32e44c3718fc6cb6430758097ee13fb126705a",
            _ => return None,
        })
    } else {
        Some(match wood {
            "acacia" => "7ecc55e150d2b789a1acf0fe0ae161e599bfccbb",
            "bamboo" => "5047fea8d58093e9bed42cc45b8c3c122b6f73df",
            "birch" => "cdf73b9d074b06629dfa7a37519a28a6247252a6",
            "cherry" => "2407397c8e5dc242adf4ffa155a4e980118dc6ae",
            "dark_oak" => "501ad53fd32e2f055f6d110217eb50eb2f76fb0d",
            "jungle" => "1d05d12cc638136825a7ff395370e09f716e6a4f",
            "mangrove" => "738326734408547ceb76c54692bfe56cc9f29ef3",
            "oak" => "97b86c0d78d778d8976f08b6051ed6615c616852",
            "pale_oak" => "0d20521e5af0b06ed79cdd2f92aec9eb46066d94",
            "spruce" => "03a8ff8cdb7ce79848533cdfac09a2735c738119",
            _ => return None,
        })
    }
}

/// THE SUBMIT FOLD'S OWN LIFT (the AbstractBoatRenderer.submit bytecode's
/// first pose word: `PoseStack.translate(0.0, 0.375, 0.0)` — the hull rides
/// 6/16 of a block above the entity's origin).
pub const BOAT_SUBMIT_LIFT: f32 = 0.375;

/// THE PADDLE POSE (AbstractBoatModel.animatePaddle's own bytecode —
/// `animatePaddle(F rowingTime, I side, ModelPart paddle)`): the phase
/// folds through `clampedLerp((sin(rowingTime)+1)/2, ...)`, the xRot
/// between −60° and −15°, the yRot between −45° and +45°, and the
/// SIDE-1 FOLD the right paddle speaks (`yRot = π − yRot`). The
/// rowingTime-0 word IS the rest pose the previous lane pinned:
/// sin 0 → frac 0.5 → xRot −0.6545 (the −37.5° midpoint), yRot 0
/// (the right paddle's π). The phase the CLIENT'S OWN CLOCK feeds
/// — the AbstractBoat.tick walk's paddlePositions, the wire's
/// paddle booleans the moving gate speaks (see [`rowing_time`] in
/// the screens lane).
pub fn animate_paddle(rowing_time: f32, side: i32) -> (f32, f32) {
    const PI_F: f32 = std::f32::consts::PI;
    // Mth.clampedLerp(delta, start, end): start + delta*(end-start),
    // the delta the [0,1] clamp keeps — the sin fold's frac rides
    // [0,1] inherently, the clamp the vanilla helper's own shape
    let frac = ((rowing_time.sin() + 1.0) / 2.0).clamp(0.0, 1.0);
    let x = -1.0471975803375244 + frac * (-0.2617993950843811 - -1.0471975803375244);
    let mut y = -0.7853981852531433 + frac * (0.7853981852531433 - -0.7853981852531433);
    if side == 1 {
        y = PI_F - y;
    }
    (x, y)
}

/// The full boat/raft model — the 26.2 `BoatModel`/`RaftModel` +
/// `AbstractBoatModel` bytecode's own numbers (scripts/classfile_dump.py:
/// the addCommonParts + createChestBoatModel + createChestRaftModel walks),
/// textured by the boat/chest_boat PNG the wood named. The fold is the
/// renderer's OWN chain (`submit`): T(0, 0.375, 0) · R_y(180 − yRot) ·
/// S(−1,−1,1) · R_y(90°) — the living models' −1.501 line and death flip
/// the boat never speaks.
pub struct BoatModel {
    rgba: Vec<u8>,
    /// the texture's own band (the 64-tall boat layout, the 128-tall
    /// chest band — the pixel sampler's row clamp the layout speaks)
    tex_h: usize,
    /// the raft hull (the RaftModel's two-plank bottom, no rails — the
    /// bamboo family's own geometry the model kind names)
    raft: bool,
}

impl BoatModel {
    /// Load + sha1-verify the boat texture the wood named (`entity/boat/`
    /// for the bare family, `entity/chest_boat/` for the chest variant —
    /// the boat PNGs 128×64, the chest band the 128×128 layout speaks).
    /// `None` = the boat never renders (the same honesty the mob paths
    /// speak: the renderer must not lie about its data).
    pub fn load(root: &Path, raft: bool, chest: bool, wood: &str) -> Option<BoatModel> {
        let sha = boat_texture_sha1(chest, wood)?;
        let rel = if chest {
            format!("entity/chest_boat/{wood}.png")
        } else {
            format!("entity/boat/{wood}.png")
        };
        let bytes = std::fs::read(root.join(rel)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != sha {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        let (ew, eh): (u32, u32) = if chest { (128, 128) } else { (128, 64) };
        if w != ew || h != eh || rgba.len() != (ew * eh * 4) as usize {
            return None;
        }
        Some(BoatModel {
            rgba,
            tex_h: eh as usize,
            raft,
        })
    }

    /// The texture pixel at (u, v) — the RGBA row-major words, v down
    /// (the row the texture's own height clamps — the 64 and the 128
    /// bands one sampler speaks).
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (
            u.clamp(0.0, 127.999_9),
            v.clamp(0.0, self.tex_h as f32 - 0.000_1),
        );
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 128 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE SUBMIT RENDER: the world-space textured quads of the boat at
    /// `feet` — the submit fold above (R_y(90) first, the S flip, the
    /// R_y(180 − yaw), the 0.375 lift), the part tree the model kind and
    /// the chest bit pick, the paddles the rowing times pose (the
    /// animatePaddle walk — 0.0 both, the rest the silence speaks).
    /// `cam_yaw/cam_pitch` speak the view rotation the entity diffuse
    /// folds (the same mix_light every entity rides). `rock` the bubble
    /// column's diagonal wobble (0.0 the silence), `hurt` the hit's R_x
    /// wobble in degrees (0.0 the closed gate — see [`boat_folds`]).
    pub fn quads(
        &self,
        yaw_deg: f32,
        chest: bool,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
        rowing_left: f32,
        rowing_right: f32,
        rock: f32,
        hurt: f32,
    ) -> Vec<AvatarQuad> {
        let parts = build_boat_parts(self.raft, chest, rowing_left, rowing_right);
        let (fold_point, fold_normal) = boat_folds(yaw_deg, feet, rock, hurt);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(72);
        let ident = Mat4::IDENT;
        for part in &parts {
            collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        }
        out
    }
}

/// THE SUBMIT FOLD (AbstractBoatRenderer.submit's own pose chain), folded
/// per resolved model point: R_y(90°) first (the innermost mulPose — the
/// model's +x axis the fold swings to the bow), then the S(−1,−1,1) flip
/// (the entity models' +y-down the flip stands up), then THE ROCK (the
/// bubble-column tilt the submit's own gate speaks — !isUnderWater &&
/// !Mth.equal(bubbleAngle, 0) the caller's zero keeps, the setAngleAxis
/// (rock·0.01745329238474369 rad, axis (1, 0, 1) the joml normalizes —
/// the DIAGONAL the characteristic wobble pivots around) riding BETWEEN
/// the S and the R_y, exactly the mulPose order the bytecode keeps),
/// then R_y(180 − yRot) (the joml YP fold the living chain shares), then
/// the 0.375 lift + the feet translation. No death flip, no −1.501 line —
/// the boat's own words. THE HURT TILT (the second arm — the mulPose the
/// bytecode's offset 44 speaks, the yaw chain's mulPose and the rock's
/// own the sibling slots bookend): the R_x the hurt degrees speak, the
/// gate `hurtTime > 0` the caller's 0.0 rest keeps, the origin the
/// pivot — riding BETWEEN the rock and the R_y(180 − yRot), exactly the
/// mulPose order the bytecode keeps.
fn boat_folds(
    yaw_deg: f32,
    feet: Vec3,
    rock: f32,
    hurt: f32,
) -> (
    impl Fn([f32; 3]) -> [f32; 3],
    impl Fn([f32; 3]) -> [f32; 3],
) {
    let a = (180.0f32 - yaw_deg).to_radians();
    let (sa, ca) = a.sin_cos();
    let lift = BOAT_SUBMIT_LIFT;
    // THE HURT TILT'S OWN FOLD: the R_x the XP axis keeps (the
    // rotationDegrees' deg→rad fold), the standard walk x' = x,
    // y' = y·cos − z·sin, z' = y·sin + z·cos.
    let theta_h = hurt * (std::f32::consts::PI / 180.0);
    let (hs, hc) = theta_h.sin_cos();
    let hurt_point = move |p: [f32; 3]| -> [f32; 3] {
        [p[0], p[1] * hc - p[2] * hs, p[1] * hs + p[2] * hc]
    };
    // THE ROCK'S OWN ANGLE-AXIS (setAngleAxis's internal normalize: the
    // (1, 0, 1) axis the 1/sqrt(2) half-steps speak — the Rodrigues
    // walk v' = v·cos + (k×v)·sin + k·(k·v)·(1−cos) the fold keeps)
    let theta = rock * 0.01745329238474369;
    let (rs, rc) = theta.sin_cos();
    let ka = std::f32::consts::FRAC_1_SQRT_2;
    let rock_point = move |p: [f32; 3]| -> [f32; 3] {
        let (x, y, z) = (p[0] as f32, p[1] as f32, p[2] as f32);
        let kdot = ka * (x + z);
        let m = 1.0 - rc;
        [
            x * rc + (-ka * y) * rs + ka * kdot * m,
            y * rc + (ka * (x - z)) * rs,
            z * rc + (ka * y) * rs + ka * kdot * m,
        ]
    };
    let rock_normal = move |n: [f32; 3]| -> [f32; 3] {
        let (x, y, z) = (n[0], n[1], n[2]);
        let kdot = ka * (x + z);
        let m = 1.0 - rc;
        [
            x * rc + (-ka * y) * rs + ka * kdot * m,
            y * rc + (ka * (x - z)) * rs,
            z * rc + (ka * y) * rs + ka * kdot * m,
        ]
    };
    let fold_point = move |p: [f32; 3]| -> [f32; 3] {
        // R_y(90°): x' = x·cos90 + z·sin90 = z; z' = −x·sin90 + z·cos90 = −x
        let (qx, qy, qz) = (p[2], p[1], -p[0]);
        // the S(−1,−1,1) flip
        let (sx, sy, sz) = (-qx, -qy, qz);
        // THE ROCK (the bubble tilt — the origin the pivot: the
        // translated feet + lift the PoseStack's own order keeps)
        let [rx, ry, rz] = rock_point([sx, sy, sz]);
        // THE HURT TILT (the R_x the hit's word speaks — the slot the
        // mulPose order keeps: the yaw chain the tilt bookends, the
        // rock the scale's own does)
        let [hx, hy, hz] = hurt_point([rx, ry, rz]);
        // the R_y(180 − yaw) fold
        let px = hx * ca + hz * sa;
        let pz = -hx * sa + hz * ca;
        [
            feet.x + px,
            feet.y as f32 + hy + lift,
            feet.z + pz,
        ]
    };
    let fold_normal = move |n: [f32; 3]| -> [f32; 3] {
        let (qx, qy, qz) = (n[2], n[1], -n[0]);
        let (sx, sy, sz) = (-qx, -qy, qz);
        let [rx, ry, rz] = rock_normal([sx, sy, sz]);
        let [hx, hy, hz] = hurt_point([rx, ry, rz]);
        [hx * ca + hz * sa, hy, -hx * sa + hz * ca]
    };
    (fold_point, fold_normal)
}

/// THE HULL PARTS (the addCommonParts walk's own boxes — every number the
/// bytecode's constant pool speaks): the boat's five-piece hull + the
/// raft's two-plank bottom (no rails — the bamboo family's own open deck),
/// the paddles the rowing times pose, the chest variant's three cubes riding
/// the chest UV band (the lid/lock texOffs (0,59), the bottom (0,76)).
fn build_boat_parts(raft: bool, chest: bool, rowing_left: f32, rowing_right: f32) -> Vec<Part> {
    const PI_F: f32 = std::f32::consts::PI;
    const HALF_PI: f32 = 1.5707963705062866;
    const PADDLE_TILT: f32 = 0.19634954631328583;
    let pose_left = animate_paddle(rowing_left, 0);
    let mut parts: Vec<Part> = Vec::with_capacity(8);
    if raft {
        // RaftModel.addCommonParts 'bottom': TWO stacked planks, the
        // wider 28x20x4 over the 28x16x4, pivot (0, −2.1, 1), xRot ≈ π/2
        parts.push(Part {
            pivot: [0.0, -2.0999999046325684, 1.0],
            x_rot: 1.5707999467849731,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![
                Cube {
                    tex: (0.0, 0.0),
                    origin: [-14.0, -11.0, -4.0],
                    size: [28.0, 20.0, 4.0],
                    inflate: 0.0,
                    mirror: false,
                },
                Cube {
                    tex: (0.0, 0.0),
                    origin: [-14.0, -9.0, -8.0],
                    size: [28.0, 16.0, 4.0],
                    inflate: 0.0,
                    mirror: false,
                },
            ],
            children: vec![],
        });
    } else {
        // BoatModel 'bottom': the 28x16x3 floor slab, pivot (0, 3, 1)
        parts.push(Part {
            pivot: [0.0, 3.0, 1.0],
            x_rot: HALF_PI,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 0.0),
                origin: [-14.0, -9.0, -3.0],
                size: [28.0, 16.0, 3.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        });
        // 'back': the stern rail, pivot (−15, 4, 4), yRot 3π/2
        parts.push(Part {
            pivot: [-15.0, 4.0, 4.0],
            x_rot: 0.0,
            y_rot: 4.71238899230957,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 19.0),
                origin: [-13.0, -7.0, -1.0],
                size: [18.0, 6.0, 2.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        });
        // 'front': the bow rail, pivot (15, 4, 0), yRot π/2
        parts.push(Part {
            pivot: [15.0, 4.0, 0.0],
            x_rot: 0.0,
            y_rot: HALF_PI,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 27.0),
                origin: [-8.0, -7.0, -1.0],
                size: [16.0, 6.0, 2.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        });
        // 'right': the starboard rail, pivot (0, 4, −9), yRot π
        parts.push(Part {
            pivot: [0.0, 4.0, -9.0],
            x_rot: 0.0,
            y_rot: PI_F,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 35.0),
                origin: [-14.0, -7.0, -1.0],
                size: [28.0, 6.0, 2.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        });
        // 'left': the port rail, pivot (0, 4, 9), yRot 0
        parts.push(Part {
            pivot: [0.0, 4.0, 9.0],
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 43.0),
                origin: [-14.0, -7.0, -1.0],
                size: [28.0, 6.0, 2.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        });
    }
    // THE PADDLES (the AbstractBoatModel.setupAnim's own walk —
    // animatePaddle(rowingTimeLeft, 0, leftPaddle) then
    // animatePaddle(rowingTimeRight, 1, rightPaddle), the phase the
    // client's paddlePositions clock feeds): the boat's texOffs
    // (62,0)/(62,20) at pivot y −5, the raft's (0,24)/(40,24) at y −4,
    // the right paddle's yRot the side-1 fold's π − yRot
    let (tex_l, tex_r, pad_pivot_y) = if raft {
        ((0.0f32, 24.0f32), (40.0f32, 24.0f32), -4.0f32)
    } else {
        ((62.0f32, 0.0f32), (62.0f32, 20.0f32), -5.0f32)
    };
    let paddle = |tex: (f32, f32), pivot_y: f32, pivot_z: f32, pose: (f32, f32)| Part {
        pivot: [3.0, pivot_y, pivot_z],
        x_rot: pose.0,
        y_rot: pose.1,
        z_rot: PADDLE_TILT,
        cubes: vec![
            Cube {
                tex,
                origin: [-1.0, 0.0, -5.0],
                size: [2.0, 2.0, 18.0],
                inflate: 0.0,
                mirror: false,
            },
            Cube {
                tex,
                origin: [-1.0010000467300415, -3.0, 8.0],
                size: [1.0, 6.0, 7.0],
                inflate: 0.0,
                mirror: false,
            },
        ],
        children: vec![],
    };
    parts.push(paddle(tex_l, pad_pivot_y, 9.0, pose_left));
    parts.push(paddle(
        tex_r,
        pad_pivot_y,
        -9.0,
        animate_paddle(rowing_right, 1),
    ));
    // THE CHEST VARIANT (createChestBoatModel / createChestRaftModel's own
    // three cubes): the bottom (0,76) 12x8x12, the lid (0,59) 12x4x12, the
    // lock (0,59) 2x4x1 — all yRot −π/2, the pivots the model kind's own
    // boat (−5/−9/−6) vs raft (−10.1/−14.1/−11.1) walk speaks
    if chest {
        let (by, ly, ky) = if raft {
            (-10.100000381469727f32, -14.100000381469727f32, -11.100000381469727f32)
        } else {
            (-5.0f32, -9.0f32, -6.0f32)
        };
        parts.push(Part {
            pivot: [-2.0, by, -6.0],
            x_rot: 0.0,
            y_rot: -HALF_PI,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 76.0),
                origin: [0.0, 0.0, 0.0],
                size: [12.0, 8.0, 12.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        });
        parts.push(Part {
            pivot: [-2.0, ly, -6.0],
            x_rot: 0.0,
            y_rot: -HALF_PI,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 59.0),
                origin: [0.0, 0.0, 0.0],
                size: [12.0, 4.0, 12.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        });
        parts.push(Part {
            pivot: [-1.0, ky, -1.0],
            x_rot: 0.0,
            y_rot: -HALF_PI,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (0.0, 59.0),
                origin: [0.0, 0.0, 0.0],
                size: [2.0, 4.0, 1.0],
                inflate: 0.0,
                mirror: false,
            }],
            children: vec![],
        });
    }
    parts
}

// ---------------------------------------------------------------------------
// THE ARMOR LAYERS (HumanoidArmorLayer.renderArmorPiece, the 26.2 jar's own
// bytecode): the store's worn piece rides the humanoid armor model — the
// SAME createMesh part tree the skin speaks, re-cubed by the slot's
// `ArmorModelSet.get(slot)` mesh and textured by the equipment asset the
// Equippable component names.
// ---------------------------------------------------------------------------

/// The store's armor-row indexes (the SLOT_IDS loop's own order —
/// HEAD, CHEST, LEGS, FEET, the container indices 39-k).
pub const ARMOR_SLOT_HEAD: usize = 0;
pub const ARMOR_SLOT_CHEST: usize = 1;
pub const ARMOR_SLOT_LEGS: usize = 2;
pub const ARMOR_SLOT_FEET: usize = 3;

/// THE SLOT'S DEFORMATION (javap `LayerDefinitions.<clinit>`): the
/// OUTER_ARMOR_DEFORMATION 1.0 for the helmet/chestplate/boots slots'
/// models, the INNER_ARMOR_DEFORMATION 0.5 for the leggings — the
/// `usesInnerModel` verdict (slot == LEGS) the bytecode speaks.
fn armor_inflate(slot: usize) -> f32 {
    if slot == ARMOR_SLOT_LEGS {
        0.5
    } else {
        1.0
    }
}

/// THE SLOT'S PART SET (javap `HumanoidModel.ADULT_ARMOR_PARTS_PER_SLOT`,
/// the <clinit>'s own Set.of walk): HEAD {head}, CHEST {body, right_arm,
/// left_arm}, LEGS {body, right_leg, left_leg}, FEET {right_leg,
/// left_leg} — the retainExactParts/retainPartsAndChildren folds the
/// armor mesh's own builder keeps (the overlays the skin model carries
/// NEVER ride: the armor mesh has no hat/jacket/sleeve/pants parts).
/// The cube table is `createBaseArmorMesh`: createMesh's own boxes, the
/// LEGS pair replaced (texOffs (0, 16), the deformation extended -0.1
/// the boots/leggings stack's own clearance speaks), the LEFT pair the
/// mirror flag speaks (the 64x32 armor layout the skin's dedicated
/// left-side UVs never shared).
fn armor_parts(state: &AvatarState, slot: usize) -> Vec<Part> {
    let def = armor_inflate(slot);
    let leg_def = def - 0.1;
    // the posed walk's own index law: 0 head, 1 body, 2 right_arm,
    // 3 left_arm, 4 right_leg, 5 left_leg — the armor part rides the
    // SAME pivot and setupAnim rotation (copyPropertiesTo's own walk)
    let posed = build_posed_parts(state);
    let mk = |src: &Part, cubes: Vec<Cube>| Part {
        pivot: src.pivot,
        x_rot: src.x_rot,
        y_rot: src.y_rot,
        z_rot: src.z_rot,
        cubes,
        children: vec![],
    };
    match slot {
        ARMOR_SLOT_HEAD => vec![mk(
            &posed[0],
            vec![Cube {
                tex: (0.0, 0.0),
                origin: [-4.0, -8.0, -4.0],
                size: [8.0, 8.0, 8.0],
                inflate: def,
                mirror: false,
            }],
        )],
        ARMOR_SLOT_CHEST => vec![
            mk(
                &posed[1],
                vec![Cube {
                    tex: (16.0, 16.0),
                    origin: [-4.0, 0.0, -2.0],
                    size: [8.0, 12.0, 4.0],
                    inflate: def,
                    mirror: false,
                }],
            ),
            mk(
                &posed[2],
                vec![Cube {
                    tex: (40.0, 16.0),
                    origin: [-3.0, -2.0, -2.0],
                    size: [4.0, 12.0, 4.0],
                    inflate: def,
                    mirror: false,
                }],
            ),
            mk(
                &posed[3],
                vec![Cube {
                    tex: (40.0, 16.0),
                    origin: [-1.0, -2.0, -2.0],
                    size: [4.0, 12.0, 4.0],
                    inflate: def,
                    mirror: true,
                }],
            ),
        ],
        ARMOR_SLOT_LEGS => vec![
            mk(
                &posed[1],
                vec![Cube {
                    tex: (16.0, 16.0),
                    origin: [-4.0, 0.0, -2.0],
                    size: [8.0, 12.0, 4.0],
                    inflate: def,
                    mirror: false,
                }],
            ),
            mk(
                &posed[4],
                vec![Cube {
                    tex: (0.0, 16.0),
                    origin: [-2.0, 0.0, -2.0],
                    size: [4.0, 12.0, 4.0],
                    inflate: leg_def,
                    mirror: false,
                }],
            ),
            mk(
                &posed[5],
                vec![Cube {
                    tex: (0.0, 16.0),
                    origin: [-2.0, 0.0, -2.0],
                    size: [4.0, 12.0, 4.0],
                    inflate: leg_def,
                    mirror: true,
                }],
            ),
        ],
        ARMOR_SLOT_FEET => vec![
            mk(
                &posed[4],
                vec![Cube {
                    tex: (0.0, 16.0),
                    origin: [-2.0, 0.0, -2.0],
                    size: [4.0, 12.0, 4.0],
                    inflate: leg_def,
                    mirror: false,
                }],
            ),
            mk(
                &posed[5],
                vec![Cube {
                    tex: (0.0, 16.0),
                    origin: [-2.0, 0.0, -2.0],
                    size: [4.0, 12.0, 4.0],
                    inflate: leg_def,
                    mirror: true,
                }],
            ),
        ],
        _ => vec![],
    }
}

/// The armor layer's world-space quads at `state` — the skin's own
/// entity chain (`AvatarModel::quads`'s fold verbatim) over the slot's
/// armor parts. `feet` = the body's feet position (the mannequin
/// speaks Vec3::ZERO — the PiP's own body space).
pub fn armor_quads(
    state: &AvatarState,
    feet: Vec3,
    cam_yaw: f32,
    cam_pitch: f32,
    slot: usize,
) -> Vec<AvatarQuad> {
    let parts = armor_parts(state, slot);
    let crouch_drop = if state.crouching { -2.0 / 16.0 } else { 0.0 };
    let (fold_point, fold_normal) =
        entity_folds(state.yaw_deg, state.death_time, crouch_drop, feet);
    let mut out: Vec<AvatarQuad> = Vec::new();
    let ident = Mat4::IDENT;
    for part in &parts {
        collect_part(part, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
    }
    out
}

/// THE ITEM'S EQUIPMENT ASSET (the vanilla datapack's default EQUIPPABLE
/// components the armor family speaks — the same name fold
/// `equippable_loop_k` keeps for the slots): the material prefixes the
/// registry names carry fold to the EquipmentAssets keys the 26.2
/// clinit pinned (iron→iron, golden→gold, chainmail→chainmail,
/// diamond→diamond, copper→copper, netherite→netherite, leather→
/// leather), the turtle helmet to its own scute. `None` for the HEAD
/// family the asset never textures (carved pumpkin, the skull family —
/// vanilla's own `assetId` empty Optional the shouldRender gate
/// refuses) and the elytra (the wings layer the humanoid slots never
/// speak).
pub fn armor_asset_name(name: Option<&str>) -> Option<&'static str> {
    let name = name?;
    if name == "turtle_helmet" {
        return Some("turtle_scute");
    }
    let asset = match name.split_once('_')?.0 {
        "iron" => "iron",
        "golden" => "gold",
        "chainmail" => "chainmail",
        "diamond" => "diamond",
        "copper" => "copper",
        "netherite" => "netherite",
        "leather" => "leather",
        _ => return None,
    };
    Some(asset)
}

/// THE WINGS LANE'S TEXTURE (javap `WingsLayer` the 26.2 jar — the
/// elytra chest stack the `chestEquipment` field reads, the
/// EQUIPPABLE's assetId the empty-Optional return refuses, the
/// `getPlayerElytraTexture` walk the texture serves: the AvatarRender
/// State's `skin.elytra()` first, the `skin.cape()` + showCape pair
/// next, NULL the default steve's neither-kept answer — the NULL the
/// EquipmentLayerRenderer's LAYER texture keeps, the
/// equipment/elytra.json's own `minecraft:elytra` row the WINGS dir
/// resolves). The `use_player_texture: true` the json speaks rides the
/// same walk — the client's default skin carries neither an elytra nor
/// a cape texture, the layer png stays.
pub struct WingsTex {
    rgba: Vec<u8>,
}

impl WingsTex {
    /// Load + sha1-verify the wings texture (the generated
    /// `WINGS_ELYTRA_SHA1` the generator's own walk banks), decode,
    /// shape-check 64x32. `None` = the wings never render (the same
    /// honesty every texture lane keeps).
    pub fn load(root: &Path) -> Option<WingsTex> {
        let bytes = std::fs::read(
            root.join(mc_registry::generated::equipment::WINGS_DIR).join("elytra.png"),
        )
        .ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes))
            != mc_registry::generated::equipment::WINGS_ELYTRA_SHA1
        {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 {
            return None;
        }
        Some(WingsTex { rgba })
    }

    /// The layer pixel at (u, v) in wing texels — the skin's own
    /// clamp-to-edge nearest-texel law, 64x32.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 64.0 - 1e-4), v.clamp(0.0, 32.0 - 1e-4));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// THE WINGS LANE'S GEOMETRY (javap `ElytraModel.createLayer` +
/// `setupAnim` the 26.2 jar — the exact part walk the bytecode keeps):
/// the texture 64x32 (`LayerDefinition.create(mesh, 64, 32)`), the
/// `CubeDeformation(1.0F)` the shared inflate keeps, the two wings the
/// root carries —
/// * LEFT: `texOffs(22, 0)`, `addBox(-10, 0, 0, 10, 20, 2, def 1.0)`,
///   pivot (5, 0, 0);
/// * RIGHT: the SAME texOffs `.mirror()`ed, `addBox(0, 0, 0, 10, 20,
///   2, def 1.0)`, pivot (-5, 0, 0).
/// `setupAnim`'s own overwrite (the PartPose rotations the anim state
/// replaces every frame — the rot inputs the walk's own law speaks):
/// left `(rotX, rotY, rotZ)`, right `(rotX, -rotY, -rotZ)` (the fneg
/// pair the yRot/zRot reads keep, the xRot the SAME both wings wear);
/// the crouch wing's own `y = 3.0F` the pivot drop speaks (the model
/// space y-down, the wings sag 3 texels). THE LAYER'S OWN TRANSLATE
/// (javap `WingsLayer.submit` — `poseStack.translate(0.0F, 0.0F,
/// 0.125F)` the push/pop pair keeps): the whole root shifts 2 texels
/// toward the model's +z — the BACK side the fold's own z law never
/// flips (the body's face the -z plane wears, the wings hang behind).
fn elytra_wing_parts(
    crouching: bool,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
) -> Vec<Part> {
    let def = 1.0;
    let y_drop = if crouching { 3.0 } else { 0.0 };
    let cube = |ox: f32, mirror: bool| Cube {
        tex: (22.0, 0.0),
        origin: [ox, 0.0, 0.0],
        size: [10.0, 20.0, 2.0],
        inflate: def,
        mirror,
    };
    vec![
        Part {
            pivot: [5.0, y_drop, 0.0],
            x_rot: rot_x,
            y_rot: rot_y,
            z_rot: rot_z,
            cubes: vec![cube(-10.0, false)],
            children: vec![],
        },
        Part {
            pivot: [-5.0, y_drop, 0.0],
            x_rot: rot_x,
            y_rot: -rot_y,
            z_rot: -rot_z,
            cubes: vec![cube(0.0, true)],
            children: vec![],
        },
    ]
}

/// The wings layer's world-space quads at `state` — the skin's own
/// entity chain (`armor_quads`'s fold verbatim) over the layer's
/// translate + the wing parts. `rot_*` = the animation state's own
/// render lerp (`elytra_wing_rot_at` the body speaks), the pose the
/// extraction's partial-tick law wears.
pub fn wings_quads(
    state: &AvatarState,
    feet: Vec3,
    cam_yaw: f32,
    cam_pitch: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
) -> Vec<AvatarQuad> {
    let parts = elytra_wing_parts(state.crouching, rot_x, rot_y, rot_z);
    let crouch_drop = if state.crouching { -2.0 / 16.0 } else { 0.0 };
    let (fold_point, fold_normal) =
        entity_folds(state.yaw_deg, state.death_time, crouch_drop, feet);
    // THE LAYER'S OWN TRANSLATE (WingsLayer.submit's
    // `translate(0, 0, 0.125)` — the entity-rotated frame the pose
    // stack carries, the model-space +z the fold's own law keeps)
    let parent = Mat4::translation(0.0, 0.0, 0.125);
    let mut out: Vec<AvatarQuad> = Vec::new();
    for part in &parts {
        collect_part(part, &parent, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
    }
    out
}

/// One head-block quad: the CustomHeadLayer item branch's geometry over
/// the element bake — the texture id rides for the sampler (the atlas
/// lane the GUI icons speak), the shade is the block's own face law.
pub struct HeadBlockQuad {
    pub pts: [[f32; 3]; 4],
    pub uv: [[f32; 2]; 4],
    pub shade: f32,
    pub texture: &'static str,
}

/// The element quad's model-space face shade (the block renderer's own
/// law — Level.getShade's constants, NOT the entity diffuse the skin
/// and the armor pieces ride; a block on the head shades per its MODEL
/// face and never re-shades as the head turns — the baked quad's own
/// direction the render speaks). `"shade": false` elements ride bare.
fn block_face_shade(direction: mc_registry::generated::elements::Cull, shade: bool) -> f32 {
    use mc_registry::generated::elements::Cull;
    if !shade {
        return 1.0;
    }
    let idx = match direction {
        Cull::East => 0,
        Cull::West => 1,
        Cull::Up => 2,
        Cull::Down => 3,
        Cull::South => 4,
        Cull::North => 5,
    };
    crate::colors::FACE_SHADE[idx]
}

/// THE HEAD-BLOCK LANE (CustomHeadLayer.submit's item branch — the
/// 26.2 bytecode the evidence walk pinned): the head stack the
/// extraction keeps (the skull family the `wornHeadType` branch
/// clears, the helmet the `HumanoidArmorLayer.shouldRender` gate
/// clears, the ASSETLESS plain block the `updateForLiving` HEAD
/// context fills) renders the item's model through the head's own
/// posed chain — `head.translateAndRotate` (the posed head part's
/// matrix over the identity root), then translateToHead's law
/// `translate(0, -0.25+yOffset, 0)` [the DEFAULT Transforms yOffset 0],
/// `mulPose(rotY 180°)`, `scale(0.625, -0.625, -0.625)`, then the
/// item's quads at the HEAD display context — NO_TRANSFORM's own bare
/// centering `translate(-0.5, -0.5, -0.5)` the item pipeline's
/// apply_transform(None) speaks (the block model's [0..1] bake folds
/// centered, the pumpkin's center lands EXACTLY on the head's own).
/// The block vertices ride BLOCK units (no /16 — the bake's own space,
/// not the texel space the cubes speak). `None` = the model carries no
/// element geometry (the skull family's special models — the honest
/// not-this-lane, the follow-up names).
pub fn head_block_quads(
    state: &AvatarState,
    feet: Vec3,
    _cam_yaw: f32,
    _cam_pitch: f32,
    model_id: &str,
) -> Option<Vec<HeadBlockQuad>> {
    let quads = mc_registry::element_quads::derive_model_inherited(model_id)?;
    let posed = build_posed_parts(state);
    let head = &posed[0]; // the head part's pivot + setupAnim rotation
    let m_head = head.matrix(&Mat4::IDENT);
    // translateToHead + the HEAD context's bare centering, one chain
    let layer = Mat4::translation(0.0, -0.25, 0.0)
        .mul(&Mat4::rot_y(std::f32::consts::PI))
        .mul(&Mat4::scale_3d(0.625, -0.625, -0.625))
        .mul(&Mat4::translation(-0.5, -0.5, -0.5));
    let m = m_head.mul(&layer);
    let crouch_drop = if state.crouching { -2.0 / 16.0 } else { 0.0 };
    let (fold_point, _fold_normal) =
        entity_folds(state.yaw_deg, state.death_time, crouch_drop, feet);
    let mut out: Vec<HeadBlockQuad> = Vec::new();
    for q in quads {
        let pts = q.vertices.map(|v| {
            let p = m.transform_point(Vec3::new(v[0], v[1], v[2]));
            fold_point([p[0], p[1], p[2]])
        });
        let shade = block_face_shade(q.direction, q.shade);
        out.push(HeadBlockQuad {
            pts,
            uv: q.uvs,
            shade,
            texture: q.texture,
        });
    }
    Some(out)
}

/// THE WORN-SKULL VERDICT (the extraction's `AbstractSkullBlock` check
/// the 26.2 extractRenderState bytecode speaks — the seven heads the
/// SkullBlock family names): these ride the `wornHeadType` branch of
/// the CustomHeadLayer, NEVER the item branch (the pumpkin's lane) and
/// never the armor layers (the asset fold refuses them all).
pub const SKULL_ITEMS: [&str; 7] = [
    "skeleton_skull",
    "wither_skeleton_skull",
    "zombie_head",
    "player_head",
    "creeper_head",
    "piglin_head",
    "dragon_head",
];

pub fn skull_item(name: Option<&str>) -> bool {
    match name {
        Some(n) => SKULL_ITEMS.contains(&n),
        None => false,
    }
}

/// The entity-model face's model-space normal (the dump's face table
/// the cube order speaks — the SAME convention the emit_cube's own
/// table keeps; the entity fold maps it to the world normal the
/// diffuse law samples).
fn entity_face_normal(direction: mc_registry::generated::elements::Cull) -> [f32; 3] {
    use mc_registry::generated::elements::Cull;
    match direction {
        Cull::Down => [0.0, -1.0, 0.0],
        Cull::Up => [0.0, 1.0, 0.0],
        Cull::West => [-1.0, 0.0, 0.0],
        Cull::North => [0.0, 0.0, -1.0],
        Cull::East => [1.0, 0.0, 0.0],
        Cull::South => [0.0, 0.0, 1.0],
    }
}

/// THE WORN-SKULL LANE (CustomHeadLayer.submit's wornHeadType branch —
/// the 26.2 bytecode the evidence walk pinned): the head chain rides
/// first (the SAME head part's matrix the item branch speaks), then
/// `translate(0, skullYOffset, 0)` [the DEFAULT Transforms skullYOffset
/// 0], `scale(1.1875, 1.1875, 1.1875)` — and SkullBlockRenderer's own
/// submitSkull goes STRAIGHT (the bytecode builds the State
/// (animationPos) and submits the model AS-IS — no direction rotation
/// the block-entity wall/ground paths keep, no item stanza, no
/// centering: the skull models bake CENTERED — the cube +-0.25, y
/// -0.5..0 — and the 1.1875 rides the entity model's own 0.09375
/// inflation all around the head's own box). The seven heads the dump
/// carries render whole: the piglin's ears (the part poses +-4.5/-6),
/// the dragon's head pose + jaw (the skull juts the face side), the
/// zombie/player hat. The shade rides the ENTITY DIFFUSE (the skull is
/// an entity model — the `shade: true` the dump bake speaks, the skin's
/// own law), NOT the block face constants the pumpkin lane keeps.
/// `None` = a non-skull item (the honest not-this-lane).
pub fn skull_quads(
    state: &AvatarState,
    feet: Vec3,
    cam_yaw: f32,
    cam_pitch: f32,
    item_id: i32,
) -> Option<Vec<HeadBlockQuad>> {
    if !skull_item(mc_registry::item_name(item_id)) {
        return None;
    }
    let render = mc_registry::special_item_renderer(item_id)?;
    let posed = build_posed_parts(state);
    let m_head = posed[0].matrix(&Mat4::IDENT);
    // translate(0, skullYOffset, 0) · scale(1.1875) — the DEFAULT
    // Transforms' skullYOffset speaks 0.0 (the clinit's (0,0,1))
    let layer = Mat4::translation(0.0, 0.0, 0.0)
        .mul(&Mat4::scale_3d(SKULL_SCALE, SKULL_SCALE, SKULL_SCALE));
    let m = m_head.mul(&layer);
    let crouch_drop = if state.crouching { -2.0 / 16.0 } else { 0.0 };
    let (fold_point, fold_normal) =
        entity_folds(state.yaw_deg, state.death_time, crouch_drop, feet);
    let mut out: Vec<HeadBlockQuad> = Vec::new();
    for item_layer in &render.layers {
        for q in mc_registry::display::special_layer_bake(item_layer.layer, item_layer.texture) {
            let pts = q.vertices.map(|v| {
                let p = m.transform_point(Vec3::new(v[0], v[1], v[2]));
                fold_point([p[0], p[1], p[2]])
            });
            // the entity diffuse over the FOLDED world normal (the
            // same law the skin and the armor pieces ride)
            let n = entity_face_normal(q.direction);
            let wn = fold_normal(n);
            let shade = entity_diffuse(wn, cam_yaw, cam_pitch);
            out.push(HeadBlockQuad {
                pts,
                uv: q.uvs,
                shade,
                texture: q.texture,
            });
        }
    }
    Some(out)
}

pub const SKULL_SCALE: f32 = 1.1875;

/// THE MANNEQUIN'S HELD-ITEM LANE (ItemInHandLayer.submitArmWithItem —
/// the 26.2 bytecode the evidence walk pinned): the main-hand stack the
/// extraction fills (`ArmedEntityRenderState.rightHandItemStack` — the
/// entity's own main hand, the mannequin's SELECTED hotbar stack here)
/// rides the ARM chain the `ArmedModel.translateToHand` walk speaks —
/// the HumanoidModel's two `translateAndRotate` calls (the root's
/// identity + the right arm's own posed chain, the same part matrix the
/// armor pieces ride) — then the layer's own law: `mulPose(rotX -90°)`,
/// `mulPose(rotY 180°)`, the adult offset `translate(1/16, 2/16,
/// -10/16)` (the baby gate the isBaby check keeps is no mannequin's
/// branch), the swing/using animations the rest state's exact zeros
/// silence. The item's quads are the bake through the
/// `thirdperson_righthand` display stanza the ItemStackRenderState
/// submit speaks (the same composition the first-person lane's
/// `posed_item_layer_quads` folds, the OTHER context). The shade rides
/// the block face constants the quads' own direction/shade flag carries
/// — the same law the head-block lane pins (the same
/// ItemStackRenderState.submit family). `None` = the model carries no
/// bake (the honest not-this-lane).
pub fn held_item_quads(
    state: &AvatarState,
    feet: Vec3,
    cam_yaw: f32,
    cam_pitch: f32,
    model_id: &str,
) -> Option<Vec<HeadBlockQuad>> {
    hand_item_quads(state, feet, cam_yaw, cam_pitch, model_id, false)
}

/// THE MANNEQUIN'S OFFHAND LANE (ItemInHandLayer.submit's SECOND
/// `submitArmWithItem` — the bytecode loc 22-41 the walk speaks: the
/// leftHandItemState/leftHandItemStack pair with `HumanoidArm.LEFT`).
/// The extraction fills that pair the offhand's own way
/// (`ArmedEntityRenderState.extractArmedEntityRenderState`: the LEFT
/// arm's stack through `updateForLiving(..., THIRD_PERSON_LEFT_HAND)`
/// — the mannequin's OFFHAND slot 45 stack here). The chain is the
/// main hand's own three folds with the TWO laws the left arm speaks:
/// the `translateToHand` arm the `getArm(LEFT)` check answers (the
/// `leftArm` part's own posed chain — the SAME two translateAndRotate
/// calls), and the adult translate's X the bytecode's own sign branch
/// keeps (`arm == LEFT ? -1 : 1` over the loc 124-151 fdiv walk — the
/// Y 2/16 and the Z -10/16 the branch never touches). The display
/// stanza rides the THIRDPERSON_LEFTHAND context the
/// `ItemDisplayContext.leftHand` flag speaks (the registry's own
/// mirrored application — the tx/ry/rz negation the build_pose walk
/// pins). The rest is the main hand's law verbatim: the
/// rotX(-90)+rotY(180) both arms share, the block face shade, the
/// entity fold. `None` = the model carries no bake.
pub fn offhand_item_quads(
    state: &AvatarState,
    feet: Vec3,
    cam_yaw: f32,
    cam_pitch: f32,
    model_id: &str,
) -> Option<Vec<HeadBlockQuad>> {
    hand_item_quads(state, feet, cam_yaw, cam_pitch, model_id, true)
}

/// The TWO HANDS' shared composition (ItemInHandLayer.submitArmWithItem
/// is arm-agnostic except the two laws above — the walk keeps one
/// chain, the arm index + the translate X sign + the display context
/// are the branch's own three terms).
fn hand_item_quads(
    state: &AvatarState,
    feet: Vec3,
    cam_yaw: f32,
    cam_pitch: f32,
    model_id: &str,
    left: bool,
) -> Option<Vec<HeadBlockQuad>> {
    let _ = (cam_yaw, cam_pitch);
    let ctx = if left {
        mc_registry::display::THIRDPERSON_LEFTHAND
    } else {
        mc_registry::display::THIRDPERSON_RIGHTHAND
    };
    let quads = match mc_registry::display::posed_item_layer_quads(model_id, None, ctx) {
        // a broken display stanza binds nothing (vanilla warns + drops)
        Some(Ok(q)) => q,
        _ => return None,
    };
    let posed = build_posed_parts(state);
    // translateToHand: the root's identity + the arm's own chain (the
    // HumanoidModel bytecode's two translateAndRotate calls — the
    // posed part matrix the armor walk's copyPropertiesTo keeps); the
    // getArm check the LEFT/RIGHT constants answer (leftArm = the
    // parts' [3], rightArm = [2])
    let m_arm = posed[if left { 3 } else { 2 }].matrix(&Mat4::IDENT);
    // the layer's own law: rotX(-90), rotY(180) both arms share, the
    // adult (±1/16, 2/16, -10/16) — the bytecode's sign branch rides
    // the X alone
    let x_off = if left { -1.0 } else { 1.0 } / 16.0;
    let layer = Mat4::rot_x(-std::f32::consts::FRAC_PI_2)
        .mul(&Mat4::rot_y(std::f32::consts::PI))
        .mul(&Mat4::translation(x_off, 2.0 / 16.0, -10.0 / 16.0));
    let mut m = m_arm.mul(&layer);
    // THE ATTACK-ITEM LANE (ItemInHandLayer.submitArmWithItem pcs
    // 154-185, the 26.2 bytecode): `attackTime > 0` the fcmpl/ifle
    // speaks, `attackArm == arm` the if_acmpne (the attack_right the
    // client's own attackArm verdict — the right the main hand rides),
    // `swingAnimationType == STAB` the second if_acmpne — then
    // `thirdPersonAttackItem(state, poseStack)` the jar calls BEFORE
    // the use lane below (pcs 188+ the same walk speaks): the two
    // lanes STACK when both gates hold (the swing and the use the
    // same arm rides), the jar's own order the composition keeps.
    // The forward the MAIN hand stack's KINETIC_WEAPON answers
    // (pcs 10-36 the getMainHandItemStack walk — the null the 0.0
    // the rotation-only fold wears, the component the shared 0.38
    // the seven tiers ride).
    let attack_arm = state.attack_right == !left;
    if state.attack_time > 0.0
        && attack_arm
        && state.swing_kind == crate::hand::SwingKind::Stab
    {
        let forward = if state.main_hand_kinetic {
            crate::hand::KINETIC_FORWARD_MOVEMENT
        } else {
            0.0
        };
        let aim = crate::hand::spear_third_person_attack_item_matrix(
            state.attack_time,
            forward,
        );
        m = m.mul(&aim);
    }
    // THE USE LANE (ItemInHandLayer.submitArmWithItem pcs 188-232, the
    // 26.2 bytecode): `f12 = state.ticksUsingItem(arm)` — the per-arm
    // method the HumanoidRenderState override keeps (`isUsingItem &&
    // (useItemHand == MAIN) == (arm == mainArm)` the field ticks, else
    // 0) — then the arm's own pose `animateUseItem(state, poseStack,
    // f12, arm, stack)`: SPEAR the only override, its body the
    // `thirdPersonUseItem` the ArmPose$1 delegates. The STAB
    // attack-item lane (pcs 154-185, `thirdPersonAttackItem`) the
    // jar's own order speaks BEFORE this fold — the composition above
    // wears it, the two gates the independent clocks keep.
    let pose_spear = if left { state.left_spear } else { state.right_spear };
    // `(useItemHand == MAIN_HAND) == (arm == mainArm)` — mainArm RIGHT
    // the client's own, the arm==mainArm test the left flag answers
    let ticks_arm = if state.is_using_item && state.use_item_main_hand == !left {
        state.ticks_using_item
    } else {
        0.0
    };
    if ticks_arm != 0.0 && pose_spear {
        if let Some(p) = &state.kinetic_use_params {
            let amount =
                crate::hand::spear_hit_feedback_amount(state.ticks_since_kinetic_hit);
            let side = if left { -1.0f32 } else { 1.0f32 };
            let use_m = crate::hand::spear_third_person_use_matrix(
                p,
                state.attack_time,
                amount,
                side,
                crate::hand::KINETIC_FORWARD_MOVEMENT,
            );
            m = m.mul(&use_m);
        }
    }
    let crouch_drop = if state.crouching { -2.0 / 16.0 } else { 0.0 };
    let (fold_point, _fold_normal) =
        entity_folds(state.yaw_deg, state.death_time, crouch_drop, feet);
    let mut out: Vec<HeadBlockQuad> = Vec::new();
    for q in quads {
        let pts = q.vertices.map(|v| {
            let p = m.transform_point(Vec3::new(v[0], v[1], v[2]));
            fold_point([p[0], p[1], p[2]])
        });
        let shade = block_face_shade(q.direction, q.shade);
        out.push(HeadBlockQuad {
            pts,
            uv: q.uvs,
            shade,
            texture: q.texture,
        });
    }
    Some(out)
}

/// The leather layer's UNDYED tint (the client info JSON's
/// `color_when_undyed: -6265536` = 0xFFA06540 — the classic brown the
/// tint law unpacks; the store's bare (id, count) carries no dye
/// component, so the undyed default IS the color).
pub const LEATHER_UNDYED_COLOR: u32 = 0xFFA06540u32;

/// The equipment texture — one humanoid / humanoid_leggings layer png,
/// sha1-verified against the generated `equipment` table before a
/// single texel is trusted (the skin's own honesty law), shape-checked
/// 64x32 (the armor layout every layer png speaks).
pub struct ArmorTex {
    rgba: Vec<u8>,
}

impl ArmorTex {
    /// Load from the runtime cache root, verify the table's sha1 for
    /// the layer dir, decode, shape-check. `None` = the layer never
    /// renders (the renderer must not lie about its data).
    pub fn load(
        root: &Path,
        layer_dir: &str,
        tex: &mc_registry::generated::equipment::EquipmentTex,
    ) -> Option<ArmorTex> {
        use mc_registry::generated::equipment;
        let sha1 = if layer_dir == equipment::LEGGINGS_DIR {
            tex.leggings?
        } else {
            tex.humanoid
        };
        let bytes = std::fs::read(root.join(layer_dir).join(format!("{}.png", tex.name))).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != sha1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 64 || h != 32 {
            return None;
        }
        Some(ArmorTex { rgba })
    }

    /// The layer pixel at (u, v) in armor texels — the skin's own
    /// clamp-to-edge nearest-texel law, 64x32.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 64.0 - 1e-4), v.clamp(0.0, 32.0 - 1e-4));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 64 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// Walk one part tree node: the part's matrix, then every cube's six
/// faces through the entity-chain fold, then the children (the
/// overlays ride the base part's swing).
fn collect_part(
    part: &Part,
    parent: &Mat4,
    fold_point: &dyn Fn([f32; 3]) -> [f32; 3],
    fold_normal: &dyn Fn([f32; 3]) -> [f32; 3],
    cam_yaw: f32,
    cam_pitch: f32,
    out: &mut Vec<AvatarQuad>,
) {
    let m = part.matrix(parent);
    for cube in &part.cubes {
        emit_cube(cube, &m, fold_point, fold_normal, cam_yaw, cam_pitch, (1.0, 1.0), out);
    }
    for child in &part.children {
        collect_part(child, &m, fold_point, fold_normal, cam_yaw, cam_pitch, out);
    }
}

/// One cube's six faces — the `ModelPart$Cube` corner/uv table the
/// first-person arm already speaks (hand.rs's own decode), inflated by
/// the `CubeDeformation` extend, folded into world space.
#[allow(clippy::too_many_arguments)]
fn emit_cube(
    cube: &Cube,
    m: &Mat4,
    fold_point: &dyn Fn([f32; 3]) -> [f32; 3],
    fold_normal: &dyn Fn([f32; 3]) -> [f32; 3],
    cam_yaw: f32,
    cam_pitch: f32,
    tex_scale: (f32, f32),
    out: &mut Vec<AvatarQuad>,
) {
    let (u0, v0) = cube.tex;
    let [ix, iy, iz] = cube.origin;
    let [w, h, d] = cube.size;
    let inf = cube.inflate;
    let (x0, y0, z0) = (ix - inf, iy - inf, iz - inf);
    let (x1, y1, z1) = (ix + w + inf, iy + h + inf, iz + d + inf);
    let px = |p: [f32; 3]| -> [f32; 3] {
        let q = m.transform_point(Vec3::new(p[0] / 16.0, p[1] / 16.0, p[2] / 16.0));
        [q[0], q[1], q[2]]
    };
    // the face table: (corners, rect origin, rect size, model normal)
    let faces: [(&[[f32; 3]; 4], (f32, f32), (f32, f32), [f32; 3]); 6] = [
        (
            &[[x1, y0, z1], [x0, y0, z1], [x0, y0, z0], [x1, y0, z0]],
            (u0 + d, v0),
            (w, d),
            [0.0, -1.0, 0.0], // DOWN
        ),
        (
            &[[x1, y1, z0], [x0, y1, z0], [x0, y1, z1], [x1, y1, z1]],
            (u0 + d + w, v0 + d),
            (w, -d),
            [0.0, 1.0, 0.0], // UP (the rect's v runs lower->upper: V-FLIPPED)
        ),
        (
            &[[x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0]],
            (u0, v0 + d),
            (d, h),
            [-1.0, 0.0, 0.0], // WEST
        ),
        (
            &[[x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0]],
            (u0 + d, v0 + d),
            (w, h),
            [0.0, 0.0, -1.0], // NORTH (the skin's face side)
        ),
        (
            &[[x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1]],
            (u0 + d + w, v0 + d),
            (d, h),
            [1.0, 0.0, 0.0], // EAST
        ),
        (
            &[[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]],
            (u0 + d + w + d, v0 + d),
            (w, h),
            [0.0, 0.0, 1.0], // SOUTH
        ),
    ];
    for (corners, (ru, rv), (rw, rh), normal) in faces {
        let (ru1, rv1) = (ru + rw, rv + rh);
        // the Polygon constructor's fixed rect corners: k=0 -> (u1,v0),
        // 1 -> (u0,v0), 2 -> (u0,v1), 3 -> (u1,v1); the MIRROR flag
        // (javap ModelPart$Cube's own `mirror` branch) flips every
        // face rect's u axis — the polygon corners sample it reversed
        // the Polygon ctor's own texScale fold — the UV corners the
        // (texScaleU, texScaleV) pair divides (the arrow's shaft the
        // (1, 0.8) row speaks, the other models the (1,1) default keeps)
        let (tu, tv) = tex_scale;
        let uv = if cube.mirror {
            [[ru / tu, rv / tv], [ru1 / tu, rv / tv], [ru1 / tu, rv1 / tv], [ru / tu, rv1 / tv]]
        } else {
            [[ru1 / tu, rv / tv], [ru / tu, rv / tv], [ru / tu, rv1 / tv], [ru1 / tu, rv1 / tv]]
        };
        let pts = [
            fold_point(px(corners[0])),
            fold_point(px(corners[1])),
            fold_point(px(corners[2])),
            fold_point(px(corners[3])),
        ];
        let shade = entity_diffuse(fold_normal(normal), cam_yaw, cam_pitch);
        out.push(AvatarQuad { pts, uv, shade });
    }
}

/// The skin PNG's RGBA decode (the hand.rs twin: EXPAND + STRIP_16,
/// any missing alpha channel widened to 255).
fn decode_rgba(bytes: &[u8]) -> anyhow::Result<(u32, u32, Vec<u8>)> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().context("skin png read_info")?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).context("skin png decode")?;
    let (w, h) = (info.width, info.height);
    let samples = info.color_type.samples();
    let mut rgba = vec![255u8; (w * h * 4) as usize];
    for p in 0..(w * h) as usize {
        let s = &buf[p * samples..p * samples + samples];
        let o = p * 4;
        match samples {
            4 => rgba[o..o + 4].copy_from_slice(s),
            3 => {
                rgba[o] = s[0];
                rgba[o + 1] = s[1];
                rgba[o + 2] = s[2];
            }
            2 => {
                // GRAYSCALE + ALPHA (the sweep arc's own layout — the
                // png crate's EXPAND keeps the two samples): the gray
                // rides all three channels, the ALPHA the fourth — the
                // cutout the 0.5 threshold reads (the pre-filled 255
                // would paint every transparent texel opaque black)
                rgba[o] = s[0];
                rgba[o + 1] = s[0];
                rgba[o + 2] = s[0];
                rgba[o + 3] = s[1];
            }
            _ => {
                rgba[o] = s[0];
                rgba[o + 1] = s[0];
                rgba[o + 2] = s[0];
            }
        }
    }
    Ok((w, h, rgba))
}

/// THE FLAME FEATURE — the burn lane's render half (`FlameFeatureRenderer`,
/// javap verbatim): a burning entity stacks 1.4-tall billboard quads
/// around its bounding box while `h = bbHeight / (bbWidth * 1.4) > 0`,
/// alternating the FIRE_0 / FIRE_1 strips per ring, the U mirrored on
/// every second PAIR of rings (`(i/2) % 2 == 0` keeps the straight
/// orientation), the width x = 0.5 * 0.9^i narrowing upward, the base
/// y rising 0.45 per ring, the z leaning -0.03 per ring, the whole
/// pose scaled by `bbWidth * 1.4`, pre-translated `(0, 0, 0.3 -
/// (int)h * 0.02)` and rotated by the CAMERA-YAW-ONLY billboard
/// (`EntityRenderDispatcher.submit`: `Mth.rotationAroundAxis(Y_AXIS,
/// camera.orientation)` = `normalize((0, q.y, 0, q.w))` — the pitch
/// drops out of the quaternion, the flame stays vertical while it
/// faces the viewer). The color is white (no entity diffuse — the
/// fire's own light) and the block-light half of the light coords is
/// forced 15: the brand glows through the night.
///
/// The client's billboard angle: the camera's forward is
/// `(sin y, 0, -cos y)` (cam.yaw 0 = -Z), so the quad's +Z face turns
/// toward the viewer at `a = -cam.yaw` through the joml YP fold the
/// entity chain speaks (`x' = x cos a + z sin a; z' = -x sin a + z
/// cos a`).
///
/// The frame law: 32 stacked 16x16 tiles per 16x512 strip; fire_0
/// rides its mcmeta playlist `[16..31, 0..15]`, fire_1 the sequential
/// 0..31, one frame per world tick.
pub const FIRE_STRIP_SIDE: u32 = 16;
pub const FIRE_STRIP_FRAMES: u32 = 32;

/// fire_0's current frame tile at a world tick (the playlist's
/// rotation: the strip plays 16, 17, ... 31, 0, 1, ... 15).
pub fn fire0_frame(day_time: i64) -> i64 {
    (day_time.rem_euclid(FIRE_STRIP_FRAMES as i64) + 16) % FIRE_STRIP_FRAMES as i64
}

/// fire_1's current frame tile at a world tick (the plain `% 32`).
pub fn fire1_frame(day_time: i64) -> i64 {
    day_time.rem_euclid(FIRE_STRIP_FRAMES as i64)
}

/// One posed flame quad — the world-space corners and the strip-space
/// pixel UVs (u in 0..16, v over the full 512 strip).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlameQuad {
    pub pts: [[f32; 3]; 4],
    pub uv: [[f32; 2]; 4],
}

/// The two strips' decoded pixels, loaded and sha1-verified the same
/// stream the skins speak (the burn lane stays silent when either
/// misses — the honest unseen).
pub struct FireStrips {
    pub fire0: Vec<u8>,
    pub fire1: Vec<u8>,
}

impl FireStrips {
    /// Load both strips from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1s, decode, and
    /// shape-check the 16x512 layout.
    pub fn load(root: &std::path::Path) -> Option<FireStrips> {
        let fire0_bytes = std::fs::read(root.join(mc_registry::generated::skin::FIRE_0_RUNTIME_PATH))
            .ok()?;
        if hex::encode(sha1::Sha1::digest(&fire0_bytes))
            != mc_registry::generated::skin::FIRE_0_PNG_SHA1
        {
            return None;
        }
        let (w0, h0, fire0) = decode_rgba(&fire0_bytes).ok()?;
        if w0 != FIRE_STRIP_SIDE || h0 != FIRE_STRIP_SIDE * FIRE_STRIP_FRAMES {
            return None;
        }
        let fire1_bytes = std::fs::read(root.join(mc_registry::generated::skin::FIRE_1_RUNTIME_PATH))
            .ok()?;
        if hex::encode(sha1::Sha1::digest(&fire1_bytes))
            != mc_registry::generated::skin::FIRE_1_PNG_SHA1
        {
            return None;
        }
        let (w1, h1, fire1) = decode_rgba(&fire1_bytes).ok()?;
        if w1 != FIRE_STRIP_SIDE || h1 != FIRE_STRIP_SIDE * FIRE_STRIP_FRAMES {
            return None;
        }
        Some(FireStrips { fire0, fire1 })
    }

    /// The strip pixel at (u, v) in pixel space — v over the full
    /// 512-tall strip; `None` outside (the caller's cutout).
    pub fn pixel(&self, strip: usize, u: f32, v: f32) -> [u8; 4] {
        let rgba = if strip == 0 { &self.fire0 } else { &self.fire1 };
        let w = FIRE_STRIP_SIDE as f32;
        let h = (FIRE_STRIP_SIDE * FIRE_STRIP_FRAMES) as f32;
        let (uu, vv) = (u.clamp(0.0, w - 1e-4), v.clamp(0.0, h - 1e-4));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * FIRE_STRIP_SIDE as usize + x) * 4;
        [rgba[o], rgba[o + 1], rgba[o + 2], rgba[o + 3]]
    }
}

/// THE XP ORB's decoded sprite sheet — the award orbs' pixel source,
/// loaded and sha1-verified the same stream the skins and the flame
/// strips speak (javap `ExperienceOrbRenderer`'s EXPERIENCE_ORB_LOCATION
/// static init: `textures/entity/experience/experience_orb.png`, the
/// 64x64 sheet the 4x4 grid of 16x16 icons tiles — the icon variant
/// index the `getIcon` band law speaks, the uv walk `(icon % 4) * 16 /
/// 64` and `(icon / 4) * 16 / 64` the submit's own f6..f9 fold). The
/// render lane stays honest the same way: a missing or tampered file
/// keeps the orbs invisible.
pub struct OrbSheet {
    pub rgba: Vec<u8>,
}

/// The sheet's own layout (javap the submit's `/ 64.0F` folds — the
/// 4x4 grid of 16x16 icons over the 64x64 sheet).
pub const ORB_SHEET_SIDE: u32 = 64;
#[expect(dead_code)] // the 26.2 ledger's icon tile side — the HUD rows read their own
pub const ORB_ICON_SIDE: u32 = 16;

impl OrbSheet {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 64x64 layout.
    pub fn load(root: &std::path::Path) -> Option<OrbSheet> {
        let bytes =
            std::fs::read(root.join(skin::EXPERIENCE_ORB_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::EXPERIENCE_ORB_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != ORB_SHEET_SIDE || h != ORB_SHEET_SIDE {
            return None;
        }
        Some(OrbSheet { rgba })
    }

    /// The sheet pixel at (u, v) in the 0..16 SPRITE SPACE the client's
    /// sampled lanes speak (the sheet's own 64px spans the 16-unit
    /// space — one icon's 16px rides the 4-unit window), nearest texel
    /// over the clamp-to-edge law.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let scale = ORB_SHEET_SIDE as f32 / 16.0;
        let x = ((u * scale).floor().clamp(0.0, ORB_SHEET_SIDE as f32 - 1.0)) as usize;
        let y = ((v * scale).floor().clamp(0.0, ORB_SHEET_SIDE as f32 - 1.0)) as usize;
        let o = (y * ORB_SHEET_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's crit star texture — the 8x8 the crit quad
/// samples (javap: the `crit` particle type's own sprite, the jar's
/// `textures/particle/critical_hit.png`), loaded and sha1-verified the
/// same stream the orb sheet speaks. A missing or tampered file keeps
/// the stars invisible (the honest absence the other lanes speak).
pub struct CritSprite {
    pub rgba: Vec<u8>,
}

/// The crit texture's own layout (the jar's 8x8 the IHDR speaks).
pub const CRIT_SPRITE_SIDE: u32 = 8;

impl CritSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<CritSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_CRIT_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_CRIT_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != CRIT_SPRITE_SIDE || h != CRIT_SPRITE_SIDE {
            return None;
        }
        Some(CritSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..8 the
    /// particle quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, CRIT_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, CRIT_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * CRIT_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The happy villager's sparkle (javap: the `happy_villager` particle
/// type's sprite set — the jar's `particles/happy_villager.json` lists
/// ONE texture, `minecraft:glint`; the
/// `SuspendedTownParticle$HappyVillagerProvider`'s `SpriteSet.get(random)`
/// returns it whatever the draw and the `setColor(1.0F, 1.0F, 1.0F)`
/// override passes the texture's own green whole — the feed sparkle the
/// `AgeableMob.aiStep`'s forcedAgeTimer walk spawns).
pub struct GlintSprite {
    pub rgba: Vec<u8>,
}

/// The glint texture's own layout (the jar's 8x8 the IHDR speaks).
pub const GLINT_SPRITE_SIDE: u32 = 8;

impl GlintSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<GlintSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_GLINT_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_GLINT_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != GLINT_SPRITE_SIDE || h != GLINT_SPRITE_SIDE {
            return None;
        }
        Some(GlintSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..8 the
    /// particle quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, GLINT_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, GLINT_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * GLINT_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's poof frames — the death cloud's 8-frame walk
/// (javap: the `poof` particle type's sprite set, the jar's
/// `textures/particle/generic_0..7.png`, the `MutableSpriteSet.get`
/// law `sprites.get(age * (size - 1) / lifetime)` the frame index
/// speaks), each file loaded and sha1-verified the same stream. A
/// missing or tampered frame keeps the cloud silent.
pub struct PoofFrames {
    pub frames: Vec<Vec<u8>>,
}

impl PoofFrames {
    /// Load the 8 frames from the runtime cache root, verify each
    /// sha1, decode, shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<PoofFrames> {
        let mut frames = Vec::with_capacity(crate::particles::POOF_FRAME_COUNT);
        for i in 0..crate::particles::POOF_FRAME_COUNT {
            let bytes = std::fs::read(root.join(skin::PARTICLE_POOF_RUNTIME_PATHS[i])).ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_POOF_PNG_SHA1S[i] {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != CRIT_SPRITE_SIDE || h != CRIT_SPRITE_SIDE {
                return None;
            }
            frames.push(rgba);
        }
        Some(PoofFrames { frames })
    }

    /// The frame pixel at (u, v) in TEXTURE UNITS (the 0..8), nearest
    /// texel.
    pub fn pixel(&self, frame: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.frames.get(frame) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, CRIT_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, CRIT_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * CRIT_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The FIREWORK PARTICLE LANE's spark frames — the 8x8 the rocket's
/// trail quad samples (javap: the `firework` particle type's own
/// SpriteSet, the jar's `textures/particle/spark_0..7.png` pool the
/// firework.json's DESCENDING list orders — the `SparkParticle` the
/// trail walk speaks), loaded and sha1-verified the same stream the
/// poof bank speaks. A missing or tampered file keeps the trail
/// invisible (the honest absence the other lanes speak).
pub struct SparkFrames {
    frames: Vec<Vec<u8>>,
}

impl SparkFrames {
    /// Load the 8 frames from the runtime cache root, verify each
    /// sha1, decode, shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<SparkFrames> {
        let mut frames = Vec::with_capacity(crate::particles::SPARK_FRAME_COUNT);
        for i in 0..crate::particles::SPARK_FRAME_COUNT {
            let bytes = std::fs::read(root.join(skin::PARTICLE_SPARK_RUNTIME_PATHS[i])).ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_SPARK_PNG_SHA1S[i] {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != CRIT_SPRITE_SIDE || h != CRIT_SPRITE_SIDE {
                return None;
            }
            frames.push(rgba);
        }
        Some(SparkFrames { frames })
    }

    /// The frame pixel at (u, v) in TEXTURE UNITS (the 0..8), nearest
    /// texel.
    pub fn pixel(&self, frame: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.frames.get(frame) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, CRIT_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, CRIT_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * CRIT_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The PARTICLE LANE's drown bubble texture — the 8x8 the bubble quad
/// samples (javap: the `bubble` particle type's own sprite, the jar's
/// `textures/particle/bubble.png`, the `BubbleParticle` the broadcast
/// 67's makeDrownParticles law speaks), loaded and sha1-verified the
/// same stream the crit star speaks. A missing or tampered file keeps
/// the bubbles invisible (the honest absence the other lanes speak).
pub struct BubbleSprite {
    pub rgba: Vec<u8>,
}

/// The bubble texture's own layout (the jar's 8x8 the IHDR speaks).
pub const BUBBLE_SPRITE_SIDE: u32 = 8;

impl BubbleSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<BubbleSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_BUBBLE_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_BUBBLE_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != BUBBLE_SPRITE_SIDE || h != BUBBLE_SPRITE_SIDE {
            return None;
        }
        Some(BubbleSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..8 the
    /// particle quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, BUBBLE_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, BUBBLE_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * BUBBLE_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's lava drip sprite — the single 8x8 the `lava`
/// particle type's sprite set holds (javap: the `particles/lava.json`
/// the jar speaks — ONE texture, `minecraft:lava`, the 8x8 the IHDR
/// verifies; the `LavaParticle$Provider`'s `SpriteSet.get(random)`
/// returns that one sprite whatever the draw), loaded and
/// sha1-verified the same stream the bubble sprite speaks. A missing
/// or tampered file keeps the drips invisible (the honest absence the
/// other lanes speak).
pub struct DripHangSprite {
    pub rgba: Vec<u8>,
}

/// The drip-hang sprite's own layout (the jar's 8x8 the IHDR speaks).
pub const DRIP_HANG_SPRITE_SIDE: u32 = 8;

impl DripHangSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<DripHangSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_DRIP_HANG_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_DRIP_HANG_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != DRIP_HANG_SPRITE_SIDE || h != DRIP_HANG_SPRITE_SIDE {
            return None;
        }
        Some(DripHangSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..8 the
    /// particle quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, DRIP_HANG_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, DRIP_HANG_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * DRIP_HANG_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's drip-fall sprite — the single 8x8 the
/// `falling_dripstone_water` / `falling_dripstone_lava` particle
/// types' sprite sets hold (javap: the jsons the 26.2 jar speaks —
/// ONE texture each, `minecraft:drip_fall`), loaded and
/// sha1-verified the same stream the drip-hang sprite speaks. A
/// missing or tampered file keeps the falls invisible.
pub struct DripFallSprite {
    pub rgba: Vec<u8>,
}

/// The drip-fall sprite's own layout (the jar's 8x8 the IHDR speaks).
pub const DRIP_FALL_SPRITE_SIDE: u32 = 8;

impl DripFallSprite {
    /// Load from the runtime cache root, verify the generated sha1,
    /// decode, and shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<DripFallSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_DRIP_FALL_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_DRIP_FALL_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != DRIP_FALL_SPRITE_SIDE || h != DRIP_FALL_SPRITE_SIDE {
            return None;
        }
        Some(DripFallSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS, nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, DRIP_FALL_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, DRIP_FALL_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * DRIP_FALL_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's drip-land sprite — the single 8x8 the
/// `landing_lava` particle type's sprite set holds (javap: the json
/// the 26.2 jar speaks — ONE texture, `minecraft:drip_land`), loaded
/// and sha1-verified the same stream the drip-fall sprite speaks. A
/// missing or tampered file keeps the land splats invisible.
pub struct DripLandSprite {
    pub rgba: Vec<u8>,
}

/// The drip-land sprite's own layout (the jar's 8x8 the IHDR speaks).
pub const DRIP_LAND_SPRITE_SIDE: u32 = 8;

impl DripLandSprite {
    /// Load from the runtime cache root, verify the generated sha1,
    /// decode, and shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<DripLandSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_DRIP_LAND_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_DRIP_LAND_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != DRIP_LAND_SPRITE_SIDE || h != DRIP_LAND_SPRITE_SIDE {
            return None;
        }
        Some(DripLandSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS, nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, DRIP_LAND_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, DRIP_LAND_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * DRIP_LAND_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's torch flame sprite — the single 8x8 the `flame`
/// particle type's sprite set holds (javap: the `particles/flame.json`
/// the 26.2 jar speaks — ONE texture, `minecraft:flame`, the 8x8 the
/// IHDR verifies; the `FlameParticle$Provider`'s `SpriteSet.get(random)`
/// returns that one sprite whatever the draw, the nextInt(1) the
/// one-element list answers), loaded and sha1-verified the same stream
/// the lava sprite speaks. A missing or tampered file keeps the flames
/// invisible (the honest absence the other lanes speak).
pub struct LavaSprite {
    pub rgba: Vec<u8>,
}

/// The lava texture's own layout (the jar's 8x8 the IHDR speaks).
pub const LAVA_SPRITE_SIDE: u32 = 8;

impl LavaSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<LavaSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_LAVA_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_LAVA_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != LAVA_SPRITE_SIDE || h != LAVA_SPRITE_SIDE {
            return None;
        }
        Some(LavaSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..8 the
    /// particle quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, LAVA_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, LAVA_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * LAVA_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's torch flame sprite — the single 8x8 the `flame`
/// particle type's sprite set holds (javap: the `particles/flame.json`
/// the 26.2 jar speaks — ONE texture, `minecraft:flame`, the 8x8 the
/// IHDR verifies; the `FlameParticle$Provider`'s `SpriteSet.get(random)`
/// returns that one sprite whatever the draw, the nextInt(1) the
/// one-element list answers), loaded and sha1-verified the same stream
/// the lava sprite speaks. A missing or tampered file keeps the flames
/// invisible (the honest absence the other lanes speak).
pub struct FlameSprite {
    pub rgba: Vec<u8>,
}

/// The flame texture's own layout (the jar's 8x8 the IHDR speaks).
pub const FLAME_SPRITE_SIDE: u32 = 8;

impl FlameSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<FlameSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_FLAME_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_FLAME_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != FLAME_SPRITE_SIDE || h != FLAME_SPRITE_SIDE {
            return None;
        }
        Some(FlameSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..8 the
    /// particle quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, FLAME_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, FLAME_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * FLAME_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's soul fire flame sprite — the single 8x8 the
/// `soul_fire_flame` particle type's sprite set holds (javap: the
/// `particles/soul_fire_flame.json` the 26.2 jar speaks — ONE
/// texture, `minecraft:soul_fire_flame`, the 8x8 the IHDR verifies;
/// the `FlameParticle$SmallFlameProvider`'s `SpriteSet.get(random)`
/// returns that one sprite whatever the draw, the nextInt(1) the
/// one-element list answers, the scale(0.5F) tail the class share
/// keeps), loaded and sha1-verified the same stream the torch flame
/// sprite speaks. A missing or tampered file keeps the soul flames
/// invisible (the honest absence the other lanes speak).
pub struct SoulFlameSprite {
    pub rgba: Vec<u8>,
}

/// The soul flame texture's own layout (the jar's 8x8 the IHDR
/// speaks).
pub const SOUL_FLAME_SPRITE_SIDE: u32 = 8;

impl SoulFlameSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<SoulFlameSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_SOUL_FIRE_FLAME_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_SOUL_FIRE_FLAME_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != SOUL_FLAME_SPRITE_SIDE || h != SOUL_FLAME_SPRITE_SIDE {
            return None;
        }
        Some(SoulFlameSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..8 the
    /// particle quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, SOUL_FLAME_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, SOUL_FLAME_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * SOUL_FLAME_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's wax-trio sprite — the 8x8 the `wax_on` /
/// `wax_off` / `scrape` particle types' own sets hold (javap: the
/// three jsons the 26.2 jar banks ALL list ONE texture,
/// `minecraft:glow` — the `GlowParticle$WaxOnProvider` /
/// `$WaxOffProvider` / `$ScrapeProvider` `SpriteSet.get(random)` calls
/// return it whatever the draw, the LevelEvent 3003/3004/3005 arms'
/// own particle the `GlowParticle` class speaks), loaded and
/// sha1-verified the same stream the flame sprite speaks. A missing
/// or tampered file keeps the wax bursts invisible (the honest
/// absence the other lanes speak).
pub struct GlowSprite {
    pub rgba: Vec<u8>,
}

/// The PARTICLE LANE's sculk shriek sprite — the LevelEvent-3007
/// arm's ten-delayed ring's single sprite (javap: the `shriek`
/// particle type's own texture, the jar's
/// `textures/particle/shriek.png`, the `SpriteSet.get(random)` pick
/// the Provider speaks — the `BitRandomSource.nextInt(1)` draw the
/// power-of-2 arm still spends), loaded and sha1-verified the same
/// stream the glow sprite speaks. A missing or tampered file keeps
/// the shriek rings invisible (the honest absence the other lanes
/// speak).
pub struct ShriekSprite {
    pub rgba: Vec<u8>,
}

/// The shriek texture's own layout (the jar's 13x13 the IHDR speaks —
/// the odd size the lane verifies).
pub const SHRIEK_SPRITE_SIDE: u32 = 13;

impl ShriekSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 13x13 layout.
    pub fn load(root: &std::path::Path) -> Option<ShriekSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_SHRIEK_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_SHRIEK_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != SHRIEK_SPRITE_SIDE || h != SHRIEK_SPRITE_SIDE {
            return None;
        }
        Some(ShriekSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..13 the
    /// 13x13 quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, SHRIEK_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, SHRIEK_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * SHRIEK_SPRITE_SIDE as usize + x) * 4;
        [
            self.rgba[o],
            self.rgba[o + 1],
            self.rgba[o + 2],
            self.rgba[o + 3],
        ]
    }
}

/// The glow texture's own layout (the jar's 8x8 the IHDR speaks).
pub const GLOW_SPRITE_SIDE: u32 = 8;

/// The PARTICLE LANE's firework flash sprite — the Starter cadence's
/// own overlay (javap: the `flash` particle type's single texture, the
/// jar's `textures/particle/flash.png`, the `FireworkParticles$
/// FlashProvider`'s `SpriteSet.get(random)` pick the
/// `BitRandomSource.nextInt(1)` draw answers whatever the roll — the
/// one-texture set the flash.json keeps, the `OverlayParticle` the
/// Starter's per-explosion tail spawns), loaded and sha1-verified the
/// same stream the shriek sprite speaks. A missing or tampered file
/// keeps the flashes invisible (the honest absence the other lanes
/// speak — the dye colors' alpha-0 law paints nothing either way).
pub struct FlashSprite {
    pub rgba: Vec<u8>,
}

/// The flash texture's own layout (the jar's 32x32 the IHDR speaks).
pub const FLASH_SPRITE_SIDE: u32 = 32;

impl FlashSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 32x32 layout.
    pub fn load(root: &std::path::Path) -> Option<FlashSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_FLASH_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_FLASH_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != FLASH_SPRITE_SIDE || h != FLASH_SPRITE_SIDE {
            return None;
        }
        Some(FlashSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..32 the
    /// flash quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, FLASH_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, FLASH_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * FLASH_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

impl GlowSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<GlowSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_GLOW_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_GLOW_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != GLOW_SPRITE_SIDE || h != GLOW_SPRITE_SIDE {
            return None;
        }
        Some(GlowSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..8 the
    /// particle quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, GLOW_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, GLOW_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * GLOW_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's damage indicator sprite — the 8x8 the
/// `damage` particle type's own set holds (javap: the jar's
/// `particles/damage_indicator.json` lists ONE texture,
/// `minecraft:damage` — the `CritParticle$DamageIndicatorProvider`'s
/// `SpriteSet.get(random)` returns it whatever the draw; the WHITE
/// star the hit's own burst rides, the crit's critical_hit.png
/// sibling the atlas lane landed), loaded and sha1-verified the same
/// stream the lava sprite speaks. A missing or tampered file keeps
/// the indicators invisible (the honest absence the other lanes
/// speak).
pub struct DamageSprite {
    pub rgba: Vec<u8>,
}

/// The damage texture's own layout (the jar's 8x8 the IHDR speaks).
pub const DAMAGE_SPRITE_SIDE: u32 = 8;

impl DamageSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<DamageSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_DAMAGE_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_DAMAGE_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != DAMAGE_SPRITE_SIDE || h != DAMAGE_SPRITE_SIDE {
            return None;
        }
        Some(DamageSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..8 the
    /// particle quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, DAMAGE_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, DAMAGE_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * DAMAGE_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's heart sprite — the love burst's own red (javap:
/// the `heart` particle type's single sprite, the jar's
/// `textures/particle/heart.png` the `HeartParticle$Provider`'s
/// `SpriteSet.get(random)` returns whatever the draw — the one-texture
/// set the damage sprite's own json speaks, the `HeartParticle` the
/// event-18 burst and the aiStep ambient drip both spawn).
pub struct HeartSprite {
    pub rgba: Vec<u8>,
}

/// The heart texture's own layout (the jar's 8x8 the IHDR speaks).
pub const HEART_SPRITE_SIDE: u32 = 8;

impl HeartSprite {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 8x8 layout (the damage loader's own walk).
    pub fn load(root: &std::path::Path) -> Option<HeartSprite> {
        let bytes = std::fs::read(root.join(skin::PARTICLE_HEART_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_HEART_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != HEART_SPRITE_SIDE || h != HEART_SPRITE_SIDE {
            return None;
        }
        Some(HeartSprite { rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..8 the
    /// particle quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, HEART_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, HEART_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * HEART_SPRITE_SIDE as usize + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PARTICLE LANE's sweep arc frames — the sword's own swing's
/// 8-frame sprite set (javap: the `sweep_attack` particle type's own
/// sprites, the jar's `textures/particle/sweep_0..7.png`, the
/// `sweep_attack.json`'s ASCENDING list the index law reads — the
/// list index IS the file index, the poof json's descending order the
/// mirror image; each frame the 32x32 the IHDR verifies, the
/// `AttackSweepParticle`'s `setSpriteFromAge` walk the
/// `MutableSpriteSet.get` law `sprites.get(age * (size - 1) /
/// lifetime)` speaks), each file loaded and sha1-verified the same
/// stream the splash frames speak. A missing or tampered frame keeps
/// the arc invisible (the honest absence the other lanes speak).
pub struct SweepFrames {
    pub frames: Vec<Vec<u8>>,
}

/// The sweep texture's own layout (the jar's 32x32 the IHDR speaks).
pub const SWEEP_SPRITE_SIDE: u32 = 32;

/// The PARTICLE LANE's splash-potion ring frames — the TWO 8-frame
/// sprite sets the case-2002 arm's ring rides (javap: the `effect`
/// particle type's own set the jar's `particles/effect.json` lists
/// `effect_7..effect_0`, the `instant_effect` type's the
/// `particles/instant_effect.json` lists `spell_7..spell_0`, BOTH
/// descending — the `SpellParticle`'s `setSpriteFromAge` walk the
/// frame fold answers, the set the `InstantProvider`'s shared lambda
/// wires per type), each file loaded and sha1-verified the same
/// stream the sweep frames speak. A missing or tampered frame keeps
/// that set's ring invisible (the honest absence the other lanes
/// speak).
pub struct SpellFrames {
    pub frames: Vec<Vec<u8>>,
}

/// The spell texture's own layout (the jar's 8x8 the IHDR speaks).
pub const SPELL_SPRITE_SIDE: u32 = 8;

impl SpellFrames {
    /// Load the 8 frames of ONE set from the runtime cache root, verify
    /// each sha1, decode, shape-check the 8x8 layout. The `instant`
    /// flag picks the set: the `spell_N` files the instant json lists,
    /// the `effect_N` files the regular splash rides.
    pub fn load(root: &std::path::Path, instant: bool) -> Option<SpellFrames> {
        let mut frames = Vec::with_capacity(crate::particles::SPELL_FRAME_COUNT);
        for i in 0..crate::particles::SPELL_FRAME_COUNT {
            let path = if instant {
                skin::PARTICLE_SPELL_RUNTIME_PATHS[i]
            } else {
                skin::PARTICLE_EFFECT_RUNTIME_PATHS[i]
            };
            let sha1 = if instant {
                skin::PARTICLE_SPELL_PNG_SHA1S[i]
            } else {
                skin::PARTICLE_EFFECT_PNG_SHA1S[i]
            };
            let bytes = std::fs::read(root.join(path)).ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes)) != sha1 {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != SPELL_SPRITE_SIDE || h != SPELL_SPRITE_SIDE {
                return None;
            }
            frames.push(rgba);
        }
        Some(SpellFrames { frames })
    }

    /// The frame pixel at (u, v) in TEXTURE UNITS (the 0..8 the ring
    /// quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, frame: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.frames.get(frame) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, SPELL_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, SPELL_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * SPELL_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The PARTICLE LANE's splash-potion debris sprites — the TWO 16x16
/// item textures the case-2002 arm's shard burst alternates (javap:
/// the `BreakingItemParticle$ItemParticleProvider.getSprite` walk's
/// `ItemStackRenderState.pickParticleMaterial` fold —
/// `layers[random.nextInt(activeLayerCount)]` over the splash_potion
/// model's two layers, layer0 the `potion_overlay` glass, layer1 the
/// `splash_potion` liquid the model's own stanza keeps), each file
/// loaded and sha1-verified the same stream the spell frames speak.
/// A missing or tampered sprite keeps the debris invisible.
pub struct PotionSprites {
    /// The model's two layers: 0 the glass outline, 1 the liquid.
    pub layers: Vec<Vec<u8>>,
}

/// The item texture's own layout (the jar's 16x16 the IHDR speaks).
pub const POTION_SPRITE_SIDE: u32 = 16;

impl PotionSprites {
    /// Load the two layers from the runtime cache root, verify each
    /// sha1, decode, shape-check the 16x16 layout.
    pub fn load(root: &std::path::Path) -> Option<PotionSprites> {
        let mut layers = Vec::with_capacity(2);
        for (path, sha1) in [
            (
                skin::ITEM_POTION_OVERLAY_RUNTIME_PATH,
                skin::ITEM_POTION_OVERLAY_PNG_SHA1,
            ),
            (
                skin::ITEM_SPLASH_POTION_RUNTIME_PATH,
                skin::ITEM_SPLASH_POTION_PNG_SHA1,
            ),
        ] {
            let bytes = std::fs::read(root.join(path)).ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes)) != sha1 {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != POTION_SPRITE_SIDE || h != POTION_SPRITE_SIDE {
                return None;
            }
            layers.push(rgba);
        }
        Some(PotionSprites { layers })
    }

    /// The layer pixel at (u, v) in TEXTURE UNITS (the 0..16 the
    /// shard quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, layer: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.layers.get(layer) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, POTION_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, POTION_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * POTION_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The PARTICLE LANE's eye-of-ender shatter sprite — the ONE 16x16
/// item texture the case-2003 arm's shard burst wears (javap: the
/// `BreakingItemParticle`'s own render the `ItemParticleOption(
/// ParticleTypes.ITEM, Items.ENDER_EYE)` walk feeds — the eye's own
/// model the generated `item/ender_eye` stanza keeps, the single
/// layer0 the `item/generated` parent speaks, the `pickParticleMaterial`'s
/// `nextInt(1)` pick the walk's draw still spends), the file loaded
/// and sha1-verified the same stream the potion sprites speak. A
/// missing or tampered sprite keeps the debris invisible.
pub struct EyeSprites {
    /// The eye's own sprite pixels (the 16x16 RGBA the decode keeps).
    pub sprite: Vec<u8>,
}

/// The eye texture's own layout (the jar's 16x16 the IHDR speaks).
pub const EYE_SPRITE_SIDE: u32 = 16;

impl EyeSprites {
    /// Load the sprite from the runtime cache root, verify the sha1,
    /// decode, shape-check the 16x16 layout.
    pub fn load(root: &std::path::Path) -> Option<EyeSprites> {
        let bytes = std::fs::read(root.join(skin::ITEM_ENDER_EYE_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::ITEM_ENDER_EYE_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != EYE_SPRITE_SIDE || h != EYE_SPRITE_SIDE {
            return None;
        }
        Some(EyeSprites { sprite: rgba })
    }

    /// The sprite pixel at (u, v) in TEXTURE UNITS (the 0..16 the
    /// shard quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor().clamp(0.0, EYE_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, EYE_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * EYE_SPRITE_SIDE as usize + x) * 4;
        [self.sprite[o], self.sprite[o + 1], self.sprite[o + 2], self.sprite[o + 3]]
    }
}

impl SweepFrames {
    /// Load the 8 frames from the runtime cache root, verify each
    /// sha1, decode, shape-check the 32x32 layout.
    pub fn load(root: &std::path::Path) -> Option<SweepFrames> {
        let mut frames = Vec::with_capacity(crate::particles::SWEEP_FRAME_COUNT);
        for i in 0..crate::particles::SWEEP_FRAME_COUNT {
            let bytes =
                std::fs::read(root.join(skin::PARTICLE_SWEEP_RUNTIME_PATHS[i])).ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_SWEEP_PNG_SHA1S[i] {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != SWEEP_SPRITE_SIDE || h != SWEEP_SPRITE_SIDE {
                return None;
            }
            frames.push(rgba);
        }
        Some(SweepFrames { frames })
    }

    /// The frame pixel at (u, v) in TEXTURE UNITS (the 0..32 the
    /// 32x32 quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, frame: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.frames.get(frame) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, SWEEP_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, SWEEP_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * SWEEP_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The PARTICLE LANE's explosion frames — the dragon's own
/// block-break burst's 16-frame sprite set (javap: the `explosion`
/// particle type's own sprites, the jar's
/// `textures/particle/explosion_0..15.png`, the `explosion.json`'s
/// ASCENDING list the index law reads — the list index IS the file
/// index, the same ascending fold the sweep json speaks; each frame
/// the 32x32 the IHDR verifies, the `HugeExplosionParticle`'s
/// `setSpriteFromAge` walk the `MutableSpriteSet.get` law
/// `sprites.get(age * (size - 1) / lifetime)` speaks — the
/// `LevelEventHandler` case-2008 arm's ONE
/// `addParticle(EXPLOSION, x + 0.5, y + 0.5, z + 0.5, 0, 0, 0)` walk
/// the burst feeds), each file loaded and sha1-verified the same
/// stream the sweep frames speak. A missing or tampered frame keeps
/// the burst invisible (the honest absence the other lanes speak).
pub struct ExplosionFrames {
    pub frames: Vec<Vec<u8>>,
}

/// The PARTICLE LANE's sculk charge frames — the LevelEvent-3006 face
/// walk's 7-frame sprite set (javap: the `sculk_charge` particle
/// type's own sprites, the jar's
/// `textures/particle/sculk_charge_0..6.png`, the
/// `SculkChargeParticle`'s `setSpriteFromAge` walk the age law rides),
/// each file loaded and sha1-verified the same stream the explosion
/// frames speak. A missing or tampered frame keeps the sculk pulses
/// invisible (the honest absence the other lanes speak).
pub struct SculkChargeFrames {
    pub frames: Vec<Vec<u8>>,
}

/// The sculk charge texture's own layout (the jar's 16x16 the IHDR
/// speaks).
pub const SCULK_CHARGE_SPRITE_SIDE: u32 = 16;

impl SculkChargeFrames {
    /// Load the 7 frames from the runtime cache root, verify each
    /// sha1, decode, shape-check the 16x16 layout.
    pub fn load(root: &std::path::Path) -> Option<SculkChargeFrames> {
        let mut frames = Vec::with_capacity(crate::particles::SCULK_CHARGE_FRAME_COUNT);
        for i in 0..crate::particles::SCULK_CHARGE_FRAME_COUNT {
            let bytes =
                std::fs::read(root.join(skin::PARTICLE_SCULK_CHARGE_RUNTIME_PATHS[i])).ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes))
                != skin::PARTICLE_SCULK_CHARGE_PNG_SHA1S[i]
            {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != SCULK_CHARGE_SPRITE_SIDE || h != SCULK_CHARGE_SPRITE_SIDE {
                return None;
            }
            frames.push(rgba);
        }
        Some(SculkChargeFrames { frames })
    }

    /// The frame pixel at (u, v) in TEXTURE UNITS (the 0..16 the
    /// 16x16 quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, frame: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.frames.get(frame) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, SCULK_CHARGE_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, SCULK_CHARGE_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * SCULK_CHARGE_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The PARTICLE LANE's sculk charge pop frames — the LevelEvent-3006
/// pop arm's 4-frame sprite set (javap: the `sculk_charge_pop`
/// particle type's own sprites, the jar's
/// `textures/particle/sculk_charge_pop_0..3.png`, the
/// `SculkChargePopParticle`'s `setSpriteFromAge` walk the age law
/// rides), each file loaded and sha1-verified the same stream the
/// charge frames speak. A missing or tampered frame keeps the pops
/// invisible (the honest absence the other lanes speak).
pub struct SculkPopFrames {
    pub frames: Vec<Vec<u8>>,
}

/// The sculk pop texture's own layout (the jar's 16x16 the IHDR
/// speaks).
pub const SCULK_POP_SPRITE_SIDE: u32 = 16;

impl SculkPopFrames {
    /// Load the 4 frames from the runtime cache root, verify each
    /// sha1, decode, shape-check the 16x16 layout.
    pub fn load(root: &std::path::Path) -> Option<SculkPopFrames> {
        let mut frames = Vec::with_capacity(crate::particles::SCULK_POP_FRAME_COUNT);
        for i in 0..crate::particles::SCULK_POP_FRAME_COUNT {
            let bytes = std::fs::read(root.join(skin::PARTICLE_SCULK_POP_RUNTIME_PATHS[i])).ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_SCULK_POP_PNG_SHA1S[i] {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != SCULK_POP_SPRITE_SIDE || h != SCULK_POP_SPRITE_SIDE {
                return None;
            }
            frames.push(rgba);
        }
        Some(SculkPopFrames { frames })
    }

    /// The frame pixel at (u, v) in TEXTURE UNITS (the 0..16 the
    /// 16x16 quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, frame: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.frames.get(frame) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, SCULK_POP_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, SCULK_POP_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * SCULK_POP_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The PARTICLE LANE's SONIC BOOM frames — the warden's boom beam's
/// own 16-frame sprite set (javap `SonicBoomParticle`: the
/// `sonic_boom` particle type's json lists ASCENDING
/// `sonic_boom_0..15`, the HugeExplosionParticle chain the
/// `setSpriteFromAge` walk the age law rides — the
/// `MutableSpriteSet.get(age, lifetime)` fold the frame index
/// serves), each file loaded and sha1-verified the same stream the
/// sculk banks speak. A missing or tampered frame keeps the boom
/// beam invisible (the honest absence the other lanes speak).
pub struct SonicBoomFrames {
    pub frames: Vec<Vec<u8>>,
}

/// The sonic boom texture's own layout (the jar's 32x32 the IHDR
/// speaks).
pub const SONIC_BOOM_SPRITE_SIDE: u32 = 32;

impl SonicBoomFrames {
    /// Load the 16 frames from the runtime cache root, verify each
    /// sha1, decode, shape-check the 32x32 layout.
    pub fn load(root: &std::path::Path) -> Option<SonicBoomFrames> {
        let mut frames = Vec::with_capacity(crate::particles::SONIC_BOOM_FRAME_COUNT);
        for i in 0..crate::particles::SONIC_BOOM_FRAME_COUNT {
            let bytes =
                std::fs::read(root.join(skin::PARTICLE_SONIC_BOOM_RUNTIME_PATHS[i])).ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes))
                != skin::PARTICLE_SONIC_BOOM_PNG_SHA1S[i]
            {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != SONIC_BOOM_SPRITE_SIDE || h != SONIC_BOOM_SPRITE_SIDE {
                return None;
            }
            frames.push(rgba);
        }
        Some(SonicBoomFrames { frames })
    }

    /// The frame pixel at (u, v) in TEXTURE UNITS (the 0..32 the
    /// 32x32 quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, frame: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.frames.get(frame) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, SONIC_BOOM_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, SONIC_BOOM_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * SONIC_BOOM_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The PARTICLE LANE's SCULK_SOUL frames — the catalyst bloom's own
/// 11-frame sprite set (javap: the
/// `SculkCatalystBlockEntity$CatalystListener.bloom`'s
/// `sendParticles(SCULK_SOUL, ...)` post the `sculk_soul` particle
/// type's own set feeds, the jar's `textures/particle/sculk_soul_0..
/// 10.png` pool the json's ASCENDING list orders — the
/// `SoulParticle`'s `SpriteSet` the `first()` + `setSpriteFromAge`
/// walks serve, each file loaded and sha1-verified the same stream
/// the sculk banks speak). A missing or tampered frame keeps the
/// bloom wisps invisible (the honest absence the other lanes speak).
pub struct SculkSoulFrames {
    pub frames: Vec<Vec<u8>>,
}

/// The sculk soul texture's own layout (the jar's 16x16 the IHDR
/// speaks).
pub const SCULK_SOUL_SPRITE_SIDE: u32 = 16;

impl SculkSoulFrames {
    /// Load the 11 frames from the runtime cache root, verify each
    /// sha1, decode, shape-check the 16x16 layout.
    pub fn load(root: &std::path::Path) -> Option<SculkSoulFrames> {
        let mut frames = Vec::with_capacity(crate::particles::SCULK_SOUL_FRAME_COUNT);
        for i in 0..crate::particles::SCULK_SOUL_FRAME_COUNT {
            let bytes = std::fs::read(root.join(skin::PARTICLE_SCULK_SOUL_RUNTIME_PATHS[i]))
                .ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes))
                != skin::PARTICLE_SCULK_SOUL_PNG_SHA1S[i]
            {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != SCULK_SOUL_SPRITE_SIDE || h != SCULK_SOUL_SPRITE_SIDE {
                return None;
            }
            frames.push(rgba);
        }
        Some(SculkSoulFrames { frames })
    }

    /// The frame pixel at (u, v) in TEXTURE UNITS (the 0..16 the
    /// 16x16 quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, frame: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.frames.get(frame) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, SCULK_SOUL_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, SCULK_SOUL_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * SCULK_SOUL_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The PARTICLE LANE's campfire smoke frames — the dowse arm's cosy
/// column's 12-frame sprite set (javap: the `campfire_cosy_smoke` +
/// `campfire_signal_smoke` particle types' own sprites, the jar's
/// `textures/particle/big_smoke_0..11.png` pool the two jsons' shared
/// ASCENDING lists order — the `CampfireSmokeParticle`'s Cosy/Signal
/// providers the `SpriteSet.get(random)` pick speaks, ONE frame static
/// the whole life the tick override's own no-sprite walk keeps), each
/// file loaded and sha1-verified the same stream the sculk banks
/// speak. A missing or tampered frame keeps the cosy column invisible
/// (the honest absence the other lanes speak).
pub struct BigSmokeFrames {
    pub frames: Vec<Vec<u8>>,
}

/// The campfire smoke texture's own layout (the jar's 16x16 the IHDR
/// speaks).
pub const BIG_SMOKE_SPRITE_SIDE: u32 = 16;

impl BigSmokeFrames {
    /// Load the 12 frames from the runtime cache root, verify each
    /// sha1, decode, shape-check the 16x16 layout.
    pub fn load(root: &std::path::Path) -> Option<BigSmokeFrames> {
        let mut frames =
            Vec::with_capacity(crate::particles::CAMPFIRE_BIG_SMOKE_FRAME_COUNT);
        for i in 0..crate::particles::CAMPFIRE_BIG_SMOKE_FRAME_COUNT {
            let bytes =
                std::fs::read(root.join(skin::PARTICLE_BIG_SMOKE_RUNTIME_PATHS[i])).ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes))
                != skin::PARTICLE_BIG_SMOKE_PNG_SHA1S[i]
            {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != BIG_SMOKE_SPRITE_SIDE || h != BIG_SMOKE_SPRITE_SIDE {
                return None;
            }
            frames.push(rgba);
        }
        Some(BigSmokeFrames { frames })
    }

    /// The frame pixel at (u, v) in TEXTURE UNITS (the 0..16 the
    /// 16x16 quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, frame: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.frames.get(frame) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, BIG_SMOKE_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, BIG_SMOKE_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * BIG_SMOKE_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The explosion texture's own layout (the jar's 32x32 the IHDR
/// speaks).
pub const EXPLOSION_SPRITE_SIDE: u32 = 32;

impl ExplosionFrames {
    /// Load the 16 frames from the runtime cache root, verify each
    /// sha1, decode, shape-check the 32x32 layout.
    pub fn load(root: &std::path::Path) -> Option<ExplosionFrames> {
        let mut frames = Vec::with_capacity(crate::particles::EXPLOSION_FRAME_COUNT);
        for i in 0..crate::particles::EXPLOSION_FRAME_COUNT {
            let bytes = std::fs::read(root.join(skin::PARTICLE_EXPLOSION_RUNTIME_PATHS[i]))
                .ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_EXPLOSION_PNG_SHA1S[i] {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != EXPLOSION_SPRITE_SIDE || h != EXPLOSION_SPRITE_SIDE {
                return None;
            }
            frames.push(rgba);
        }
        Some(ExplosionFrames { frames })
    }

    /// The frame pixel at (u, v) in TEXTURE UNITS (the 0..32 the
    /// 32x32 quad's uv walk speaks), nearest texel.
    pub fn pixel(&self, frame: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.frames.get(frame) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, EXPLOSION_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, EXPLOSION_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * EXPLOSION_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The PARTICLE LANE's splash droplet frames — the entry splash's
/// second ring's 4-frame sprite set (javap: the `splash` particle
/// type's own sprites, the jar's `textures/particle/splash_0..3.png`,
/// the `SpriteSet.get(random)` pick the SplashParticle$Provider
/// speaks, the frame static for the particle's own life), each file
/// loaded and sha1-verified the same stream the poof frames speak. A
/// missing or tampered frame keeps the ring silent.
pub struct SplashFrames {
    pub frames: Vec<Vec<u8>>,
}
impl SplashFrames {
    /// Load the 4 frames from the runtime cache root, verify each
    /// sha1, decode, shape-check the 8x8 layout.
    pub fn load(root: &std::path::Path) -> Option<SplashFrames> {
        let mut frames = Vec::with_capacity(crate::particles::SPLASH_FRAME_COUNT);
        for i in 0..crate::particles::SPLASH_FRAME_COUNT {
            let bytes =
                std::fs::read(root.join(skin::PARTICLE_SPLASH_RUNTIME_PATHS[i])).ok()?;
            if hex::encode(sha1::Sha1::digest(&bytes)) != skin::PARTICLE_SPLASH_PNG_SHA1S[i] {
                return None;
            }
            let (w, h, rgba) = decode_rgba(&bytes).ok()?;
            if w != CRIT_SPRITE_SIDE || h != CRIT_SPRITE_SIDE {
                return None;
            }
            frames.push(rgba);
        }
        Some(SplashFrames { frames })
    }

    /// The frame pixel at (u, v) in TEXTURE UNITS (the 0..8), nearest
    /// texel.
    pub fn pixel(&self, frame: usize, u: f32, v: f32) -> [u8; 4] {
        let empty = [0u8; 4];
        let f = match self.frames.get(frame) {
            Some(f) => f,
            None => return empty,
        };
        let x = (u.floor().clamp(0.0, CRIT_SPRITE_SIDE as f32 - 1.0)) as usize;
        let y = (v.floor().clamp(0.0, CRIT_SPRITE_SIDE as f32 - 1.0)) as usize;
        let o = (y * CRIT_SPRITE_SIDE as usize + x) * 4;
        [f[o], f[o + 1], f[o + 2], f[o + 3]]
    }
}

/// The WATER SCREEN's decoded pixels — the underwater overlay's pixel
/// source, loaded and sha1-verified the same stream the skins and the
/// flame strips speak (javap `ScreenEffectRenderer.submitWater`'s
/// UNDERWATER_LOCATION: `textures/misc/underwater.png`, the 16x16 blue
/// the eye-under-water pass tiles across the screen). The burn lane
/// stays honest the same way: a missing or tampered file keeps the
/// overlay silent.
pub struct UnderwaterScreen {
    pub rgba: Vec<u8>,
}

impl UnderwaterScreen {
    /// Load from the runtime cache root (the generator's
    /// `assets-cache/jar/`), verify the generated sha1, decode, and
    /// shape-check the 16x16 layout.
    pub fn load(root: &std::path::Path) -> Option<UnderwaterScreen> {
        let bytes =
            std::fs::read(root.join(skin::UNDERWATER_RUNTIME_PATH)).ok()?;
        if hex::encode(sha1::Sha1::digest(&bytes)) != skin::UNDERWATER_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != UNDERWATER_SIDE || h != UNDERWATER_SIDE {
            return None;
        }
        Some(UnderwaterScreen { rgba })
    }

    /// The texture pixel at (u, v) in TEXTURE UNITS — the vanilla
    /// sampler's REPEAT wrap (the screen quad's uv ride the yaw/pitch
    /// scroll and span 4 full wraps; the shader repeats the 16x16
    /// tile), nearest texel: `floor` then `rem_euclid` (the uv run
    /// negative when the yaw/pitch offsets do).
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.floor() as i32).rem_euclid(UNDERWATER_SIDE as i32) as usize;
        let y = (v.floor() as i32).rem_euclid(UNDERWATER_SIDE as i32) as usize;
        let o = (y * UNDERWATER_SIDE as usize + x) * 4;
        [
            self.rgba[o],
            self.rgba[o + 1],
            self.rgba[o + 2],
            self.rgba[o + 3],
        ]
    }
}

/// The underwater screen texture's side (javap verbatim 16x16 — three
/// translucent blues, the alpha 135/146).
pub const UNDERWATER_SIDE: u32 = 16;

/// The flame quad stack around one burning entity — the `prepare`
/// bytecode's own loop, the ring law verbatim.
pub fn flame_quads(
    feet: Vec3,
    bb_width: f32,
    bb_height: f32,
    cam_yaw: f32,
    day_time: i64,
) -> Vec<FlameQuad> {
    let w = bb_width * 1.4;
    let h = bb_height / w;
    // the pre-translate: 0.3 - (int)h * 0.02 (the f2i truncation)
    let tz = 0.3f32 - (h as i32) as f32 * 0.02;
    // the billboard: the camera's yaw alone, negated into the client's
    // -Z-forward convention
    let a = -cam_yaw;
    let (sa, ca) = a.sin_cos();
    // the fold: feet + w * R_y(a) * (v + (0, 0, tz)) — the scale
    // commutes with the rotation (uniform), the bytecode's M = T·S·R·Tt
    let fold = |p: [f32; 3]| -> [f32; 3] {
        let x = p[0];
        let y = p[1];
        let z = p[2] + tz;
        let px = x * ca + z * sa;
        let pz = -x * sa + z * ca;
        [feet.x + w * px, feet.y as f32 + w * y, feet.z + w * pz]
    };
    let mut out = Vec::new();
    let mut ring_h = h;
    let mut x = 0.5f32;
    let mut base = 0.0f32;
    let mut depth = 0.0f32;
    let mut i = 0u32;
    while ring_h > 0.0 {
        // the strip tile: fire_0 on the even rings, fire_1 on the odd;
        // the (i/2)%2 pairs mirror the U (the straight orientation on
        // the even pairs)
        let frame = if i % 2 == 0 {
            fire0_frame(day_time)
        } else {
            fire1_frame(day_time)
        } as f32;
        let (v0, v1) = (frame * 16.0, (frame + 1.0) * 16.0);
        let (ua, ub) = if (i / 2) % 2 == 0 { (0.0f32, 16.0f32) } else { (16.0f32, 0.0f32) };
        // the four corners: p1(-x, bottom, ua), p2(+x, bottom, ub),
        // p3(+x, top, ub), p4(-x, top, ua) — the bottom y = -base
        // (base runs 0, -0.45, ...), the top y = 1.4 - base
        let pts = [
            fold([-x, -base, depth]),
            fold([x, -base, depth]),
            fold([x, 1.4 - base, depth]),
            fold([-x, 1.4 - base, depth]),
        ];
        let uv = [
            [ua, v1],
            [ub, v1],
            [ub, v0],
            [ua, v0],
        ];
        out.push(FlameQuad { pts, uv });
        ring_h -= 0.45;
        base -= 0.45;
        x *= 0.9;
        depth -= 0.03;
        i += 1;
    }
    out
}

// === THE WARDEN (javap `WardenModel` the 26.2 jar speaks — the
// twelve-part hierarchy the `createBodyLayer` walk banks, the four
// setupAnim folds the walk serves: the head look pair, the walk's
// own damped swing, the idle sway, the tendril shiver the clock
// wears) ===

/// 26.2 `WardenModel.setupAnim` reads — the look pair rides the
/// netHeadYaw/pitch words the humanoid's own state wears, the walk
/// pair the WalkAnimationState feeds, the age clock the idle +
/// tendril folds consult, the TENDRIL ANIMATION the
/// `getTendrilAnimation(partialTick)` word the extractRenderState
/// walk carries (the lerp/10.0 getter the sculk lane speaks). NO
/// death clock the state wears — the warden's removal lane the
/// health gate serves, the corpse the list never keeps. THE ROAR
/// the keyframe lane wears — the WARDEN_ROAR definition the
/// `WardenAnimation` clinit banks, the pose window the elapsed
/// clock speaks (the other five — attack/sonic-boom/dig/emerge/
/// sniff — the never-started states the apply no-op keeps, the
/// fresh warden the stopped states wear).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WardenState {
    pub yaw_deg: f32,
    pub pitch_deg: f32,
    pub head_yaw_off_deg: f32,
    pub walk_pos: f32,
    pub walk_speed: f32,
    pub age_ticks: f32,
    pub tendril_animation: f32,
    /// THE ROAR'S OWN CLOCK (javap `Warden.onSyncedDataUpdated` pc
    /// 10-63 — the DATA_POSE sync the ROARING pose's own arrival
    /// wears, the `roarAnimationState.start(tickCount)` pc 52-60 the
    /// pose arm banks): the elapsed TICKS since the pose's own
    /// start (the window's own `84 - roar_ticks` walk the caller
    /// derives, the partial the lerp adds — the
    /// `AnimationState.getTimeInMillis` `(age - startTick) * 50` the
    /// millis fold keeps, the KeyframeAnimation's `/1000.0f` the
    /// elapsed seconds speak). The `None` the never-armed state
    /// keeps — the `ifStarted` gate the apply no-op wears (the
    /// pose's own STANDING the stop banks, the window's own end the
    /// clock retires).
    pub roar_time: Option<f32>,
    /// THE SNIFF'S OWN CLOCK (javap `Warden.onSyncedDataUpdated` pc
    /// 66-77 — the DATA_POSE sync the SNIFFING pose's own arrival
    /// wears, the `sniffAnimationState.start(tickCount)` pc 66-74 the
    /// pose arm banks): the elapsed TICKS since the pose's own
    /// start (the window's own `84 - sniff_ticks` walk the caller
    /// derives, the partial the lerp adds — the
    /// `AnimationState.getTimeInMillis` `(age - startTick) * 50` the
    /// millis fold keeps, the KeyframeAnimation's `/1000.0f` the
    /// elapsed seconds speak). The `None` the never-armed state
    /// keeps — the `ifStarted` gate the apply no-op wears (the
    /// pose's own STANDING the stop clears, the clock the None the
    /// no-op keeps — the definition's own 3.32s rest tail the past-end
    /// walk the same pixels wears).
    pub sniff_time: Option<f32>,
    /// THE EMERGE'S OWN CLOCK (javap `Warden.onSyncedDataUpdated` pc
    /// 80-91 — the DATA_POSE sync the EMERGING pose's own arrival
    /// wears, the `emergeAnimationState.start(tickCount)` pc 80-88 the
    /// pose arm banks): the elapsed TICKS since the pose's own
    /// start (the window's own `134 - emerge_ticks` walk the caller
    /// derives, the partial the lerp adds — the
    /// `AnimationState.getTimeInMillis` `(age - startTick) * 50` the
    /// millis fold keeps, the KeyframeAnimation's `/1000.0f` the
    /// elapsed seconds speak). The `None` the never-armed state
    /// keeps — the `ifStarted` gate the apply no-op wears (the
    /// pose's own STANDING the stop clears, the clock the None the
    /// no-op keeps — the definition's own 6.64s rest tail the past-end
    /// walk the same pixels wears).
    pub emerge_time: Option<f32>,
    /// THE DIG'S OWN CLOCK (javap `Warden.onSyncedDataUpdated`
    /// tableswitch case 2 — the DATA_POSE sync the DIGGING pose's own
    /// arrival wears, the `diggingAnimationState.start(tickCount)`
    /// pc 94-102 the pose arm banks): the elapsed TICKS since the
    /// pose's own start (the window's own `100 - dig_ticks` walk the
    /// caller derives, the partial the lerp adds — the
    /// `AnimationState.getTimeInMillis` `(age - startTick) * 50` the
    /// millis fold keeps, the KeyframeAnimation's `/1000.0f` the
    /// elapsed seconds speak). The `None` the never-armed state
    /// keeps — the `ifStarted` gate the apply no-op wears (the
    /// removal the window's own end the world serves, the clock the
    /// retired body never reads).
    pub dig_time: Option<f32>,
    /// THE ATTACK'S OWN CLOCK (javap `Warden.handleEntityEvent` case
    /// 4 — the `attackAnimationState.start(tickCount)` pc 12-20 the
    /// melee-swing event arm banks): the elapsed TICKS since the
    /// start (the start tick the `attack_start` field keeps, the
    /// `tick_count - attack_start` the elapsed answers, the partial
    /// the lerp adds — the `AnimationState.getTimeInMillis` the
    /// millis fold keeps). The `None` the never-armed state keeps —
    /// the swing the AnimationState stop call NEVER carries, the
    /// stale start the definition's own rest tail the past-end walk
    /// the same zeros the never-armed state speaks.
    pub attack_time: Option<f32>,
}

/// The full warden model — the 26.2 `WardenModel.createBodyLayer`
/// own numbers, bytecode-proven off the official jar (the twelve
/// parts' pivots/boxes the addOrReplaceChild chain speaks,
/// `LayerDefinition.create(mesh, 128, 128)` the tail keeps): the
/// root `bone` pivot (0, 24, 0) no cubes carries, the `body`
/// 18x21x11 (uv 0,0) rides the bone at offset (0, -21, 0), the TWO
/// zero-width ribcages (uv 90,11 — the left the mirror flag wears)
/// hang off the body at (∓7, -2, -4), the head 16x16x10 (uv 0,32)
/// rides the body at (0, -13, 0), the TWO zero-width tendrils (uv
/// 52,32 / 58,0) ride the head at (∓8, -12, 0), the TWO 8x28x8 arms
/// (uv 44,50 / 0,58) ride the body at (∓13, -13, 1), the TWO
/// 6x13x6 legs (uv 76,48 / 76,76 — the 3.1/2.9 the x origins wear)
/// ride the bone at (∓5.9, -13, 0).
pub struct WardenModel {
    rgba: Vec<u8>,
}

impl WardenModel {
    /// Load + sha1-verify the WARDEN's skin (the WardenRenderer's
    /// TEXTURE — `textures/entity/warden/warden.png` the clinit
    /// names). The `None` = the warden never renders (the same
    /// honesty the other bodies speak: the renderer must not lie
    /// about its data).
    pub fn load(root: &Path) -> Option<WardenModel> {
        let bytes = std::fs::read(root.join(skin::WARDEN_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::WARDEN_PNG_SHA1 {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 128 || h != 128 || rgba.len() != 128 * 128 * 4 {
            return None;
        }
        Some(WardenModel { rgba })
    }

    /// The skin pixel at (u, v) — the RGBA row-major words, v down
    /// (the 128x128 layout's own stride the load verified).
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let (uu, vv) = (u.clamp(0.0, 127.999_9), v.clamp(0.0, 127.999_9));
        let x = uu as usize;
        let y = vv as usize;
        let o = (y * 128 + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }

    /// THE POSED RENDER: the world-space textured quads of the
    /// warden at `state` — the SAME entity chain fold the avatar
    /// speaks (one law, every living model), the warden's own
    /// hierarchy beneath (the bone root the parts ride).
    pub fn quads(
        &self,
        state: &WardenState,
        feet: Vec3,
        cam_yaw: f32,
        cam_pitch: f32,
    ) -> Vec<AvatarQuad> {
        let bone = build_warden_parts(state);
        let (fold_point, fold_normal) = entity_folds(state.yaw_deg, 0.0, 0.0, feet);
        let mut out: Vec<AvatarQuad> = Vec::with_capacity(6 * 10);
        let ident = Mat4::IDENT;
        collect_part(&bone, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
        out
    }
}

/// THE WARDEN'S OWN FOLD WORDS (the 26.2 `WardenModel.setupAnim`'s
/// per-part rotations the four private walks serve — the shared
/// computation BOTH the full body mesh and the emissive layers'
/// pruned meshes consult: the vanilla layer models bake their OWN
/// WardenModel instances whose setupAnim runs the same folds).
struct WardenFolds {
    head_x: f32,
    head_y: f32,
    head_z: f32,
    body_x: f32,
    body_z: f32,
    left_leg_x: f32,
    right_leg_x: f32,
    left_arm_x: f32,
    right_arm_x: f32,
    /// the tendril shiver swing (the left the plain arm, the right
    /// the negation keeps)
    swing: f32,
}

/// THE FOLD COMPUTATION (the 26.2 `WardenModel.setupAnim` walk the
/// bytecode speaks, verbatim, in the walk's OWN order): the head
/// look pair FIRST (`animateHeadLookTarget` — the xRot/yRot the
/// 0.017453292 multiplier wears), the walk swing SECOND
/// (`animateWalk` — the damping `min(0.5, 3.0 * speed)` f3 keeps,
/// the phase the 0.8662 scalar banks, the head's zRot the += ride,
/// the body's OWN assignments the `putfield` pair speaks, the legs
/// the anti-phase cos pair swings, the arms the 0.8 counter-swing
/// keeps), the idle sway THIRD (`animateIdlePose` — the 0.1-time
/// clock the +0.06 head / +0.025 body adds wear), the tendril
/// shiver LAST (`animateTendrils` — the swing the
/// `tendrilAnimation * cos(age * 2.25) * PI * 0.1` fold speaks, the
/// Math.cos the REAL double cos the javap keeps — NOT the Mth table
/// — the left the plain arm wears, the right the negation keeps).
fn warden_folds(state: &WardenState) -> WardenFolds {
    // THE LOOK PAIR (animateHeadLookTarget pc 0-24 — the
    // 0.017453292f the two fmul keeps)
    let deg2rad = 0.017_453_292_f32;
    let mut head_x = state.pitch_deg * deg2rad;
    let head_y = state.head_yaw_off_deg * deg2rad;
    let mut head_z = 0.0_f32;
    // THE WALK (animateWalk pc 0-208)
    let f3 = 0.5_f32.min(3.0 * state.walk_speed); // the damping
    let f4 = state.walk_pos * 0.866_2; // the phase
    let f5 = mth_cos(f4); // Mth.cos
    let f6 = mth_sin(f4); // Mth.sin
    let f7 = 0.35_f32.min(f3);
    head_z += 0.3 * f6 * f3;
    head_x += 1.2 * mth_cos(f4 + 1.570_796_4) * f7;
    let body_z = 0.1 * f6 * f3;
    let mut body_x = 1.0 * f5 * f7;
    let left_leg_x = 1.0 * f5 * f3;
    let right_leg_x = 1.0 * mth_cos(f4 + std::f32::consts::PI) * f3;
    let left_arm_x = -(0.8 * f5 * f3);
    let right_arm_x = -(0.8 * f6 * f3);
    // THE IDLE SWAY (animateIdlePose pc 0-89 — the age * 0.1 clock
    // the Mth.cos/sin pair consults, the head the 0.06 adds keep,
    // the body the 0.025 adds serve)
    let f2 = state.age_ticks * 0.1;
    let idle_c = mth_cos(f2);
    let idle_s = mth_sin(f2);
    head_z += 0.06 * idle_c;
    head_x += 0.06 * idle_s;
    body_x += 0.025 * idle_c;
    let body_z = body_z + 0.025 * idle_s;
    // THE TENDRIL SHIVER (animateTendrils pc 0-41 — the
    // java/lang/Math.cos the REAL double cos keeps, the
    // 0.10000000149011612 the ldc2_w wears, the left the plain arm
    // serves, the right the fneg answers)
    let swing = state.tendril_animation
        * (((state.age_ticks as f64) * 2.25).cos() * std::f64::consts::PI
            * 0.100_000_001_490_116_12) as f32;
    WardenFolds {
        head_x,
        head_y,
        head_z,
        body_x,
        body_z,
        left_leg_x,
        right_leg_x,
        left_arm_x,
        right_arm_x,
        swing,
    }
}

// === THE WARDEN'S KEYFRAME ANIMATIONS (javap `WardenAnimation` the
// 26.2 jar speaks — the `AnimationDefinition`s the clinit bakes,
// the `KeyframeAnimation.bake(model, root)` the WardenModel ctor's
// `AnimationDefinition.bake(root)` walk banks, the `apply(state,
// ageInTicks)` the setupAnim's own tail serves — the additive
// offsets the walk/idle/tendril folds the AHEAD banks speak) ===

/// ONE KEYFRAME (javap `Keyframe` the record — the timestamp the
/// f32 keeps, the vec the `degreeVec`/`posVec` bake wears, the
/// interpolation the CATMULLROM choice the ROAR always speaks).
/// The vecs the RAW definition words keep — the degrees the
/// 0.017453292 bake multiplies, the POSITION's y the posVec fneg
/// negates — the conversion the EVAL's own head serves (the
/// vanilla bake the clinit's own order wears: convert FIRST, the
/// interpolation the radians speak).
struct WardenKeyframe {
    t: f32,
    v: [f32; 3],
}

/// THE WARDEN_ROAR DEFINITION (javap `WardenAnimation` clinit pc
/// 7771-9138, verbatim — the `withLength(4.2f)` head the 84-tick
/// window the `Mth.ceil(84.0f)` ROAR_DURATION wears, the four
/// bones the addAnimation chain names — body, head, right_arm,
/// left_arm — each its ROTATION + POSITION pair the 8/5, 8/6,
/// 7/5, 7/5 keyframe arrays bank; EVERY keyframe the CATMULLROM
/// interpolation wears). THE TIMELINE the pose window wears: the
/// wind-up the -25° body tilt banks (t=1.24s = window tick 24.8 —
/// the roar sound the 25th tick answers), the shake the head's
/// own ±26° zRot pairs wear (t=1.6-2.44s), the settle the 3.0-4.2
/// tail speaks — the final keyframe the (0,0,0) rest banks, the
/// window's own 84th tick the pose's STANDING clears.
const WARDEN_ROAR_BODY_ROT: [WardenKeyframe; 8] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 1.24, v: [-25.0, 0.0, 0.0] },
    WardenKeyframe { t: 1.6, v: [32.5, 0.0, -7.5] },
    WardenKeyframe { t: 1.84, v: [38.33, 0.0, 2.99] },
    WardenKeyframe { t: 2.08, v: [40.97, 0.0, -4.3] },
    WardenKeyframe { t: 2.36, v: [44.41, 0.0, 6.29] },
    WardenKeyframe { t: 3.0, v: [47.5, 0.0, 0.0] },
    WardenKeyframe { t: 4.2, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ROAR_BODY_POS: [WardenKeyframe; 5] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 1.24, v: [0.0, -1.0, 3.0] },
    WardenKeyframe { t: 1.6, v: [0.0, -3.0, -6.0] },
    WardenKeyframe { t: 3.0, v: [0.0, -3.0, -6.0] },
    WardenKeyframe { t: 4.2, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ROAR_HEAD_ROT: [WardenKeyframe; 8] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 1.24, v: [-32.5, 0.0, 0.0] },
    WardenKeyframe { t: 1.6, v: [-32.5, 0.0, -27.5] },
    WardenKeyframe { t: 1.8, v: [-32.5, 0.0, 26.0] },
    WardenKeyframe { t: 2.04, v: [-32.5, 0.0, -27.5] },
    WardenKeyframe { t: 2.44, v: [-32.5, 0.0, 26.0] },
    WardenKeyframe { t: 2.84, v: [-5.0, 0.0, -12.5] },
    WardenKeyframe { t: 4.2, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ROAR_HEAD_POS: [WardenKeyframe; 6] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 1.24, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 1.6, v: [0.0, -2.0, -6.0] },
    WardenKeyframe { t: 2.2, v: [0.0, -2.0, -6.0] },
    WardenKeyframe { t: 2.48, v: [0.0, -2.0, -6.0] },
    WardenKeyframe { t: 4.2, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ROAR_RIGHT_ARM_ROT: [WardenKeyframe; 7] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.72, v: [-120.0, 0.0, -20.0] },
    WardenKeyframe { t: 1.24, v: [-77.5, 3.75, 15.0] },
    WardenKeyframe { t: 1.48, v: [67.5, -32.5, 20.0] },
    WardenKeyframe { t: 2.48, v: [37.5, -32.5, 25.0] },
    WardenKeyframe { t: 2.88, v: [27.6, -17.1, 32.5] },
    WardenKeyframe { t: 4.2, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ROAR_RIGHT_ARM_POS: [WardenKeyframe; 5] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.72, v: [3.0, -2.0, 0.0] },
    WardenKeyframe { t: 1.48, v: [4.0, -2.0, 0.0] },
    WardenKeyframe { t: 2.48, v: [4.0, -2.0, 0.0] },
    WardenKeyframe { t: 4.2, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ROAR_LEFT_ARM_ROT: [WardenKeyframe; 7] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.72, v: [-125.0, 0.0, 20.0] },
    WardenKeyframe { t: 1.24, v: [-76.25, -17.5, -7.5] },
    WardenKeyframe { t: 1.48, v: [62.5, 42.5, -12.5] },
    WardenKeyframe { t: 2.48, v: [37.5, 27.5, -27.5] },
    WardenKeyframe { t: 2.88, v: [25.0, 18.4, -30.0] },
    WardenKeyframe { t: 4.2, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ROAR_LEFT_ARM_POS: [WardenKeyframe; 5] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.72, v: [-3.0, -2.0, 0.0] },
    WardenKeyframe { t: 1.48, v: [-4.0, -2.0, 0.0] },
    WardenKeyframe { t: 2.48, v: [-4.0, -2.0, 0.0] },
    WardenKeyframe { t: 4.2, v: [0.0, 0.0, 0.0] },
];
/// (the definition's own `withLength(4.2f)` head — the 84-tick
/// window the 20-tick second wears — the final keyframe's own
/// timestamp the length equals, the tail loop the test banks)

/// THE WARDEN_SNIFF DEFINITION (javap `WardenAnimation` clinit pc
/// 9141-9798, verbatim — the `withLength(4.16f)` head the
/// `Mth.ceil(83.2f)` SNIFFING_DURATION the 84-tick window wears, the
/// four bones the addAnimation chain names — body, head, right_arm,
/// left_arm — each its ROTATION channel the 6/9/5/5 keyframe arrays
/// bank (the SNIFF def NO POSITION channel carries), EVERY keyframe
/// the CATMULLROM interpolation wears — the 25/25 the bytecode's
/// own ldc rows speak). THE TIMELINE the pose window wears: the
/// lunge the body's 17.5/32.5 pair the 0.56s peak banks, the sway
/// the head's ±40/20/35 yRot pairs the 0.68-1.76s window rides, the
/// settle the 2.2-3.32 tail speaks — the final keyframe the (0,0,0)
/// rest banks, the definition's own 3.32s tail the 4.16s window the
/// rest keeps (the past-end walk the Entry.apply's own law speaks).
const WARDEN_SNIFF_BODY_ROT: [WardenKeyframe; 6] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.56, v: [17.5, 32.5, 0.0] },
    WardenKeyframe { t: 0.96, v: [0.0, 32.5, 0.0] },
    WardenKeyframe { t: 2.2, v: [10.0, 0.0, 0.0] },
    WardenKeyframe { t: 2.8, v: [10.0, -30.0, 0.0] },
    WardenKeyframe { t: 3.32, v: [0.0, 0.0, 0.0] },
];
const WARDEN_SNIFF_HEAD_ROT: [WardenKeyframe; 9] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.68, v: [0.0, 40.0, 0.0] },
    WardenKeyframe { t: 0.96, v: [-22.5, 40.0, 0.0] },
    WardenKeyframe { t: 1.24, v: [0.0, 20.0, 0.0] },
    WardenKeyframe { t: 1.52, v: [-35.0, 20.0, 0.0] },
    WardenKeyframe { t: 1.76, v: [0.0, 20.0, 0.0] },
    WardenKeyframe { t: 2.28, v: [0.0, -20.0, 0.0] },
    WardenKeyframe { t: 2.88, v: [0.0, -20.0, 0.0] },
    WardenKeyframe { t: 3.32, v: [0.0, 0.0, 0.0] },
];
const WARDEN_SNIFF_RIGHT_ARM_ROT: [WardenKeyframe; 5] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.96, v: [17.5, 0.0, 0.0] },
    WardenKeyframe { t: 2.2, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 2.76, v: [-15.0, 0.0, 0.0] },
    WardenKeyframe { t: 3.32, v: [0.0, 0.0, 0.0] },
];
const WARDEN_SNIFF_LEFT_ARM_ROT: [WardenKeyframe; 5] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.96, v: [-15.0, 0.0, 0.0] },
    WardenKeyframe { t: 2.2, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 2.76, v: [17.5, 0.0, 0.0] },
    WardenKeyframe { t: 3.32, v: [0.0, 0.0, 0.0] },
];

/// THE WARDEN_EMERGE DEFINITION (javap `WardenAnimation` clinit pc
/// 0-4836, verbatim — the `withLength(6.68f)` head the `Mth.ceil
/// (133.59999f)` EMERGE_DURATION the 134-tick window wears, the SIX
/// bones the addAnimation chain names — body, head, right_arm,
/// left_arm, right_leg, left_leg — each its ROTATION + POSITION pair
/// the 16/17, 18/17, 19/17, 22/19, 11/17, 12/15 keyframe arrays bank,
/// EVERY keyframe the CATMULLROM interpolation wears). THE TIMELINE
/// the pose window wears: the RISE the body's -63 → -32 texel lift
/// the first 1.2s banks (the body the GROUND the 63-texel depth
/// climbs), the arms the -152.5/-167.5/-180 grabs the 1.2-1.68s
/// window raises, the struggle the head's -67.5 dips the 1.16-2.64s
/// pair wears, the roar-fold the 25/35/47.5 xRot peaks the 3.76-4.68s
/// tail speaks, the settle the 70-degree head + the 70-degree body
/// the 5.0-5.8s peak banks, the final keyframe the (0,0,0) rest —
/// the window's own 6.64s tail the 6.68s length the rest keeps.
const WARDEN_EMERGE_BODY_ROT: [WardenKeyframe; 16] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.52, v: [0.0, 0.0, -22.5] },
    WardenKeyframe { t: 1.2, v: [0.0, 0.0, -7.5] },
    WardenKeyframe { t: 1.68, v: [0.0, 0.0, 10.0] },
    WardenKeyframe { t: 1.8, v: [0.0, 0.0, 10.0] },
    WardenKeyframe { t: 2.28, v: [0.0, 0.0, 10.0] },
    WardenKeyframe { t: 2.88, v: [0.0, 0.0, 10.0] },
    WardenKeyframe { t: 3.76, v: [25.0, 0.0, -7.5] },
    WardenKeyframe { t: 3.92, v: [35.0, 0.0, -7.5] },
    WardenKeyframe { t: 4.08, v: [25.0, 0.0, -7.5] },
    WardenKeyframe { t: 4.44, v: [47.5, 0.0, 0.0] },
    WardenKeyframe { t: 4.56, v: [47.5, 0.0, 0.0] },
    WardenKeyframe { t: 4.68, v: [47.5, 0.0, 0.0] },
    WardenKeyframe { t: 5.0, v: [70.0, 0.0, 2.5] },
    WardenKeyframe { t: 5.8, v: [70.0, 0.0, 2.5] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];
const WARDEN_EMERGE_BODY_POS: [WardenKeyframe; 17] = [
    WardenKeyframe { t: 0.0, v: [0.0, -63.0, 0.0] },
    WardenKeyframe { t: 0.52, v: [0.0, -56.0, 0.0] },
    WardenKeyframe { t: 1.2, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 1.68, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 1.8, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 2.28, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 2.88, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 3.16, v: [0.0, -27.0, 0.0] },
    WardenKeyframe { t: 3.76, v: [0.0, -14.0, 0.0] },
    WardenKeyframe { t: 3.92, v: [0.0, -11.0, 0.0] },
    WardenKeyframe { t: 4.08, v: [0.0, -14.0, 0.0] },
    WardenKeyframe { t: 4.44, v: [0.0, -6.0, -3.0] },
    WardenKeyframe { t: 4.56, v: [0.0, -4.0, -3.0] },
    WardenKeyframe { t: 4.68, v: [0.0, -6.0, -3.0] },
    WardenKeyframe { t: 5.0, v: [0.0, -3.0, -4.0] },
    WardenKeyframe { t: 5.8, v: [0.0, -3.0, -4.0] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];
const WARDEN_EMERGE_HEAD_ROT: [WardenKeyframe; 18] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.52, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.92, v: [0.74, 0.0, -40.38] },
    WardenKeyframe { t: 1.16, v: [-67.5, 0.0, -2.5] },
    WardenKeyframe { t: 1.24, v: [-67.5, 0.0, -2.5] },
    WardenKeyframe { t: 1.32, v: [-47.5, 0.0, -2.5] },
    WardenKeyframe { t: 1.4, v: [-67.5, 0.0, -2.5] },
    WardenKeyframe { t: 1.68, v: [-67.5, 0.0, 15.0] },
    WardenKeyframe { t: 1.76, v: [-67.5, 0.0, -5.0] },
    WardenKeyframe { t: 1.84, v: [-52.5, 0.0, -5.0] },
    WardenKeyframe { t: 1.92, v: [-67.5, 0.0, -5.0] },
    WardenKeyframe { t: 2.64, v: [-17.5, 0.0, -10.0] },
    WardenKeyframe { t: 3.76, v: [70.0, 0.0, 12.5] },
    WardenKeyframe { t: 4.04, v: [70.0, 0.0, 12.5] },
    WardenKeyframe { t: 4.12, v: [80.0, 0.0, 12.5] },
    WardenKeyframe { t: 4.24, v: [70.0, 0.0, 12.5] },
    WardenKeyframe { t: 5.0, v: [77.5, 0.0, -2.5] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];
const WARDEN_EMERGE_HEAD_POS: [WardenKeyframe; 17] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.52, v: [-8.0, -11.0, 0.0] },
    WardenKeyframe { t: 0.92, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 1.24, v: [0.0, 0.47, -0.95] },
    WardenKeyframe { t: 1.32, v: [0.0, 0.47, -0.95] },
    WardenKeyframe { t: 1.4, v: [0.0, 0.47, -0.95] },
    WardenKeyframe { t: 1.68, v: [0.0, 1.0, -2.0] },
    WardenKeyframe { t: 1.76, v: [0.0, 1.0, -2.0] },
    WardenKeyframe { t: 1.84, v: [0.0, 1.0, -2.0] },
    WardenKeyframe { t: 1.92, v: [0.0, 1.0, -2.0] },
    WardenKeyframe { t: 2.64, v: [0.0, -2.0, -2.0] },
    WardenKeyframe { t: 3.76, v: [0.0, -4.0, 1.0] },
    WardenKeyframe { t: 4.04, v: [0.0, -1.0, 1.0] },
    WardenKeyframe { t: 4.12, v: [0.0, -1.0, 1.0] },
    WardenKeyframe { t: 4.24, v: [0.0, -1.0, 1.0] },
    WardenKeyframe { t: 5.0, v: [0.0, -1.0, 1.0] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];
const WARDEN_EMERGE_RIGHT_ARM_ROT: [WardenKeyframe; 19] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.52, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 1.2, v: [-152.5, 2.5, 7.5] },
    WardenKeyframe { t: 1.68, v: [-180.0, 12.5, -10.0] },
    WardenKeyframe { t: 1.8, v: [-90.0, 12.5, -10.0] },
    WardenKeyframe { t: 2.28, v: [-90.0, 12.5, -10.0] },
    WardenKeyframe { t: 2.88, v: [-90.0, 12.5, -10.0] },
    WardenKeyframe { t: 3.08, v: [-95.0, 12.5, -10.0] },
    WardenKeyframe { t: 3.24, v: [-83.93, 3.93, 5.71] },
    WardenKeyframe { t: 3.36, v: [-80.0, 7.5, 17.5] },
    WardenKeyframe { t: 3.76, v: [-67.5, 2.5, 0.0] },
    WardenKeyframe { t: 4.08, v: [-67.5, 2.5, 0.0] },
    WardenKeyframe { t: 4.44, v: [-55.0, 2.5, 0.0] },
    WardenKeyframe { t: 4.56, v: [-60.0, 2.5, 0.0] },
    WardenKeyframe { t: 4.68, v: [-55.0, 2.5, 0.0] },
    WardenKeyframe { t: 5.0, v: [-67.5, 0.0, 0.0] },
    WardenKeyframe { t: 5.56, v: [-50.45, 0.0, 2.69] },
    WardenKeyframe { t: 6.08, v: [-62.72, 0.0, 4.3] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];
const WARDEN_EMERGE_RIGHT_ARM_POS: [WardenKeyframe; 17] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.52, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 1.2, v: [0.0, -21.0, 9.0] },
    WardenKeyframe { t: 1.68, v: [2.0, -2.0, 0.0] },
    WardenKeyframe { t: 1.8, v: [2.0, 0.0, 0.0] },
    WardenKeyframe { t: 2.28, v: [2.0, 0.0, 0.0] },
    WardenKeyframe { t: 2.88, v: [2.0, 0.0, 0.0] },
    WardenKeyframe { t: 3.08, v: [2.0, -2.0, 0.0] },
    WardenKeyframe { t: 3.24, v: [2.0, 2.71, 3.86] },
    WardenKeyframe { t: 3.36, v: [2.0, 1.0, 5.0] },
    WardenKeyframe { t: 3.76, v: [2.0, 3.0, 3.0] },
    WardenKeyframe { t: 4.08, v: [2.0, 3.0, 3.0] },
    WardenKeyframe { t: 4.44, v: [2.67, 4.0, 0.0] },
    WardenKeyframe { t: 4.56, v: [2.67, 0.0, 0.0] },
    WardenKeyframe { t: 4.68, v: [2.67, 4.0, 0.0] },
    WardenKeyframe { t: 5.0, v: [0.67, 3.0, 4.0] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];
const WARDEN_EMERGE_LEFT_ARM_ROT: [WardenKeyframe; 22] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.12, v: [-167.5, -17.5, -7.5] },
    WardenKeyframe { t: 0.6, v: [-167.5, -17.5, -7.5] },
    WardenKeyframe { t: 0.88, v: [-175.0, -17.5, 15.0] },
    WardenKeyframe { t: 1.16, v: [-190.0, -17.5, 5.0] },
    WardenKeyframe { t: 1.28, v: [-90.0, -5.0, 5.0] },
    WardenKeyframe { t: 1.68, v: [-90.0, -17.5, -12.5] },
    WardenKeyframe { t: 1.8, v: [-90.0, -17.5, -12.5] },
    WardenKeyframe { t: 2.28, v: [-90.0, -17.5, -12.5] },
    WardenKeyframe { t: 2.88, v: [-90.0, -17.5, -12.5] },
    WardenKeyframe { t: 3.04, v: [-81.29, -10.64, -14.21] },
    WardenKeyframe { t: 3.16, v: [-83.5, -5.5, -15.5] },
    WardenKeyframe { t: 3.76, v: [-62.5, -7.5, 5.0] },
    WardenKeyframe { t: 3.92, v: [-58.75, -3.75, 5.0] },
    WardenKeyframe { t: 4.08, v: [-55.0, 0.0, 0.0] },
    WardenKeyframe { t: 4.44, v: [-52.5, 0.0, 5.0] },
    WardenKeyframe { t: 4.56, v: [-50.0, 0.0, 5.0] },
    WardenKeyframe { t: 4.68, v: [-52.5, 0.0, 5.0] },
    WardenKeyframe { t: 5.0, v: [-72.5, -2.5, 5.0] },
    WardenKeyframe { t: 5.56, v: [-57.5, -4.54, 2.99] },
    WardenKeyframe { t: 6.08, v: [-70.99, -5.77, 1.78] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];
const WARDEN_EMERGE_LEFT_ARM_POS: [WardenKeyframe; 19] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.12, v: [0.0, -8.0, 0.0] },
    WardenKeyframe { t: 0.6, v: [0.0, -8.0, 0.0] },
    WardenKeyframe { t: 0.88, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 1.2, v: [-2.0, 0.0, 0.0] },
    WardenKeyframe { t: 1.68, v: [-4.0, 3.0, 0.0] },
    WardenKeyframe { t: 1.8, v: [-4.0, 3.0, 0.0] },
    WardenKeyframe { t: 2.28, v: [-4.0, 3.0, 0.0] },
    WardenKeyframe { t: 2.88, v: [-4.0, 3.0, 0.0] },
    WardenKeyframe { t: 3.04, v: [-3.23, 5.7, 4.97] },
    WardenKeyframe { t: 3.16, v: [-1.49, 2.22, 5.25] },
    WardenKeyframe { t: 3.76, v: [-1.14, 1.71, 1.86] },
    WardenKeyframe { t: 3.92, v: [-1.14, 1.21, 3.86] },
    WardenKeyframe { t: 4.08, v: [-1.14, 2.71, 4.86] },
    WardenKeyframe { t: 4.44, v: [-1.0, 1.0, 3.0] },
    WardenKeyframe { t: 4.56, v: [0.0, 1.0, 1.0] },
    WardenKeyframe { t: 4.68, v: [0.0, 1.0, 3.0] },
    WardenKeyframe { t: 5.0, v: [-2.0, 0.0, 4.0] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];
const WARDEN_EMERGE_RIGHT_LEG_ROT: [WardenKeyframe; 11] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.52, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 2.28, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 2.88, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 3.36, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 4.32, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 4.48, v: [55.0, 0.0, 0.0] },
    WardenKeyframe { t: 4.6, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 5.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 5.8, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];
const WARDEN_EMERGE_RIGHT_LEG_POS: [WardenKeyframe; 17] = [
    WardenKeyframe { t: 0.0, v: [0.0, -63.0, 0.0] },
    WardenKeyframe { t: 0.52, v: [0.0, -56.0, 0.0] },
    WardenKeyframe { t: 1.2, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 1.68, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 1.8, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 2.28, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 2.88, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 3.36, v: [0.0, -22.0, 0.0] },
    WardenKeyframe { t: 3.76, v: [0.0, -12.28, 2.48] },
    WardenKeyframe { t: 3.92, v: [0.0, -9.28, 2.48] },
    WardenKeyframe { t: 4.08, v: [0.0, -12.28, 2.48] },
    WardenKeyframe { t: 4.32, v: [0.0, -4.14, 4.14] },
    WardenKeyframe { t: 4.48, v: [0.0, -0.57, -8.43] },
    WardenKeyframe { t: 4.6, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 5.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 5.8, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];
const WARDEN_EMERGE_LEFT_LEG_ROT: [WardenKeyframe; 12] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.52, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 2.28, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 2.88, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 3.36, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 3.84, v: [20.0, 0.0, -17.5] },
    WardenKeyframe { t: 4.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 4.68, v: [20.0, 0.0, 0.0] },
    WardenKeyframe { t: 4.84, v: [10.0, 0.0, 0.0] },
    WardenKeyframe { t: 5.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 5.8, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];
const WARDEN_EMERGE_LEFT_LEG_POS: [WardenKeyframe; 15] = [
    WardenKeyframe { t: 0.0, v: [0.0, -63.0, 0.0] },
    WardenKeyframe { t: 0.52, v: [0.0, -56.0, 0.0] },
    WardenKeyframe { t: 1.2, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 1.68, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 1.8, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 2.28, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 2.88, v: [0.0, -32.0, 0.0] },
    WardenKeyframe { t: 3.36, v: [0.0, -22.0, 0.0] },
    WardenKeyframe { t: 3.84, v: [-4.0, 2.0, -7.0] },
    WardenKeyframe { t: 4.0, v: [-4.0, 0.0, -5.0] },
    WardenKeyframe { t: 4.68, v: [-4.0, 0.0, -9.0] },
    WardenKeyframe { t: 4.84, v: [-2.0, 2.0, -3.5] },
    WardenKeyframe { t: 5.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 5.8, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 6.64, v: [0.0, 0.0, 0.0] },
];

/// THE MTH.CATMULLROM (javap `Mth.catmullrom` pc 0-59, verbatim —
/// the 0.5 the head push keeps, the chain the bytecode's own
/// mul/add order speaks: the 2b the first add banks, the
/// (c-a)*t the second, the quadratic `*t*t` and the cubic
/// `*t*t*t` the LEFT-assoc powers wear, the 0.5 the tail's own
/// fmul answers)
fn mth_catmullrom(t: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    0.5
        * (2.0 * b
            + (c - a) * t
            + (2.0 * a - 5.0 * b + 4.0 * c - d) * t * t
            + (3.0 * b - a - 3.0 * c + d) * t * t * t)
}

/// THE KEYFRAME'S OWN WALK (javap `KeyframeAnimation$Entry.apply`
/// pc 0-138, verbatim): the `Mth.binarySearch(0, n, e <= kf.t)`
/// the LOWER BOUND keeps (the first index the predicate answers —
/// the `Mth.binarySearch` halving walk the `i -= j + 1` the
/// never-found `min` = n wears), the `max(bs - 1, 0)` the
/// segment's own start banks, the `min(n - 1, i + 1)` the end
/// answers, the delta the `clamp((e - t_i)/(t_j - t_i), 0, 1)`
/// speaks (the j==i the ZERO keeps — the past-end walk the rest
/// pose banks), the CATMULLROM the END keyframe's own choice
/// wears — the p0 the `max(i-1, 0)` clamps, the p3 the
/// `min(n-1, j+1)` answers, the SCALE the 1.0 the apply's own
/// fconst_1 keeps. The `pos` flag the posVec's own bake wears —
/// the y the fneg negates; the rotation bake the degreeVec's
/// 0.017453292 multiplies — the CONVERT-FIRST order the clinit's
/// own bake speaks (the interpolation the baked vecs answer).
fn warden_roar_channel(kfs: &[WardenKeyframe], elapsed: f32, pos: bool) -> [f32; 3] {
    // THE BINARY SEARCH (Mth.binarySearch pc 0-43, verbatim: the
    // `i = max - min` the length keeps, the `j = i / 2` the half
    // banks, the pred the `elapsed <= kf.t` wears, the true the
    // `i = j` halves, the false the `min = k + 1` the tail walks)
    let n = kfs.len();
    let mut lo = 0usize;
    let mut len = n;
    while len > 0 {
        let half = len / 2;
        let mid = lo + half;
        if elapsed <= kfs[mid].t {
            len = half;
        } else {
            lo = mid + 1;
            len -= half + 1;
        }
    }
    let i = lo.saturating_sub(1);
    let j = (i + 1).min(n - 1);
    let delta = if j == i {
        0.0_f32
    } else {
        ((elapsed - kfs[i].t) / (kfs[j].t - kfs[i].t)).clamp(0.0, 1.0)
    };
    // THE BAKE (the degreeVec/posVec the clinit serves — the
    // rotation the 0.017453292 xyz multiply keeps, the position
    // the y-only fneg wears)
    let bake = |kf: &WardenKeyframe| -> [f32; 3] {
        if pos {
            [kf.v[0], -kf.v[1], kf.v[2]]
        } else {
            [
                kf.v[0] * 0.017_453_292_f32,
                kf.v[1] * 0.017_453_292_f32,
                kf.v[2] * 0.017_453_292_f32,
            ]
        }
    };
    let p0 = bake(&kfs[i.saturating_sub(1)]);
    let p1 = bake(&kfs[i]);
    let p2 = bake(&kfs[j]);
    let p3 = bake(&kfs[(j + 1).min(n - 1)]);
    [
        mth_catmullrom(delta, p0[0], p1[0], p2[0], p3[0]),
        mth_catmullrom(delta, p0[1], p1[1], p2[1], p3[1]),
        mth_catmullrom(delta, p0[2], p1[2], p2[2], p3[2]),
    ]
}

/// THE ROAR'S OWN DELTAS (the WARDEN_ROAR definition the 8
/// channels evaluate — the additive offsets the vanilla
/// `ModelPart.offsetRotation`/`offsetPos` walk speaks, the four
/// folds the setupAnim's own order AHEAD banks): the `None` the
/// never-armed state keeps (the `ifStarted` gate the ALL-ZERO
/// no-op wears — the fresh warden the stopped states speak), the
/// `Some(ticks)` the `getTimeInMillis` *50 /1000 the elapsed
/// seconds answer.
struct WardenRoarDeltas {
    body_rot: [f32; 3],
    body_pos: [f32; 3],
    head_rot: [f32; 3],
    head_pos: [f32; 3],
    right_arm_rot: [f32; 3],
    right_arm_pos: [f32; 3],
    left_arm_rot: [f32; 3],
    left_arm_pos: [f32; 3],
}

fn warden_roar_deltas(state: &WardenState) -> WardenRoarDeltas {
    let zero = [0.0_f32; 3];
    let Some(roar_ticks) = state.roar_time else {
        // the never-started state the apply no-op wears
        return WardenRoarDeltas {
            body_rot: zero,
            body_pos: zero,
            head_rot: zero,
            head_pos: zero,
            right_arm_rot: zero,
            right_arm_pos: zero,
            left_arm_rot: zero,
            left_arm_pos: zero,
        };
    };
    // THE ELAPSED SECONDS (the `(age - startTick) * 50` the millis
    // fold keeps, the `/1000.0f` the seconds speak — the two-step
    // the bytecode's own order wears)
    let elapsed = (roar_ticks * 50.0) / 1000.0;
    WardenRoarDeltas {
        body_rot: warden_roar_channel(&WARDEN_ROAR_BODY_ROT, elapsed, false),
        body_pos: warden_roar_channel(&WARDEN_ROAR_BODY_POS, elapsed, true),
        head_rot: warden_roar_channel(&WARDEN_ROAR_HEAD_ROT, elapsed, false),
        head_pos: warden_roar_channel(&WARDEN_ROAR_HEAD_POS, elapsed, true),
        right_arm_rot: warden_roar_channel(&WARDEN_ROAR_RIGHT_ARM_ROT, elapsed, false),
        right_arm_pos: warden_roar_channel(&WARDEN_ROAR_RIGHT_ARM_POS, elapsed, true),
        left_arm_rot: warden_roar_channel(&WARDEN_ROAR_LEFT_ARM_ROT, elapsed, false),
        left_arm_pos: warden_roar_channel(&WARDEN_ROAR_LEFT_ARM_POS, elapsed, true),
    }
}

/// THE SNIFF'S OWN DELTAS (the WARDEN_SNIFF definition the 4
/// ROTATION channels evaluate — the SAME Entry.apply walk the roar
/// lanes shares, the additive offsets the `ModelPart
/// .offsetRotation` walk speaks, the setupAnim's own order the
/// sniff apply the roar's own tail FOLLOWS): the `None` the
/// never-armed state keeps (the `ifStarted` gate the ALL-ZERO
/// no-op wears — the fresh warden the stopped states speak), the
/// `Some(ticks)` the `getTimeInMillis` *50 /1000 the elapsed
/// seconds answer. NO POSITION channel the definition carries —
/// the pivots the sniff deltas never move.
struct WardenSniffDeltas {
    body_rot: [f32; 3],
    head_rot: [f32; 3],
    right_arm_rot: [f32; 3],
    left_arm_rot: [f32; 3],
}

fn warden_sniff_deltas(state: &WardenState) -> WardenSniffDeltas {
    let zero = [0.0_f32; 3];
    let Some(sniff_ticks) = state.sniff_time else {
        // the never-started state the apply no-op wears
        return WardenSniffDeltas {
            body_rot: zero,
            head_rot: zero,
            right_arm_rot: zero,
            left_arm_rot: zero,
        };
    };
    // THE ELAPSED SECONDS (the `(age - startTick) * 50` the millis
    // fold keeps, the `/1000.0f` the seconds speak — the two-step
    // the bytecode's own order wears)
    let elapsed = (sniff_ticks * 50.0) / 1000.0;
    WardenSniffDeltas {
        body_rot: warden_roar_channel(&WARDEN_SNIFF_BODY_ROT, elapsed, false),
        head_rot: warden_roar_channel(&WARDEN_SNIFF_HEAD_ROT, elapsed, false),
        right_arm_rot: warden_roar_channel(&WARDEN_SNIFF_RIGHT_ARM_ROT, elapsed, false),
        left_arm_rot: warden_roar_channel(&WARDEN_SNIFF_LEFT_ARM_ROT, elapsed, false),
    }
}

/// THE EMERGE'S OWN DELTAS (the WARDEN_EMERGE definition the 12
/// channels evaluate — the SAME Entry.apply walk the roar/sniff
/// lanes share, the additive offsets the `ModelPart
/// .offsetRotation`/`offsetPos` walk speaks, the setupAnim's own
/// order the dig→emerge pair the roar's own apply PRECEDES): the
/// `None` the never-armed state keeps (the `ifStarted` gate the
/// ALL-ZERO no-op wears — the fresh warden the stopped states
/// speak), the `Some(ticks)` the `getTimeInMillis` *50 /1000 the
/// elapsed seconds answer. THE LEGS the def the rot + pos pairs
/// carries — the risen body the legs the SAME depth the pivot
/// wears.
struct WardenEmergeDeltas {
    body_rot: [f32; 3],
    body_pos: [f32; 3],
    head_rot: [f32; 3],
    head_pos: [f32; 3],
    right_arm_rot: [f32; 3],
    right_arm_pos: [f32; 3],
    left_arm_rot: [f32; 3],
    left_arm_pos: [f32; 3],
    right_leg_rot: [f32; 3],
    right_leg_pos: [f32; 3],
    left_leg_rot: [f32; 3],
    left_leg_pos: [f32; 3],
}

fn warden_emerge_deltas(state: &WardenState) -> WardenEmergeDeltas {
    let zero = [0.0_f32; 3];
    let Some(emerge_ticks) = state.emerge_time else {
        // the never-started state the apply no-op wears
        return WardenEmergeDeltas {
            body_rot: zero,
            body_pos: zero,
            head_rot: zero,
            head_pos: zero,
            right_arm_rot: zero,
            right_arm_pos: zero,
            left_arm_rot: zero,
            left_arm_pos: zero,
            right_leg_rot: zero,
            right_leg_pos: zero,
            left_leg_rot: zero,
            left_leg_pos: zero,
        };
    };
    // THE ELAPSED SECONDS (the `(age - startTick) * 50` the millis
    // fold keeps, the `/1000.0f` the seconds speak — the two-step
    // the bytecode's own order wears)
    let elapsed = (emerge_ticks * 50.0) / 1000.0;
    WardenEmergeDeltas {
        body_rot: warden_roar_channel(&WARDEN_EMERGE_BODY_ROT, elapsed, false),
        body_pos: warden_roar_channel(&WARDEN_EMERGE_BODY_POS, elapsed, true),
        head_rot: warden_roar_channel(&WARDEN_EMERGE_HEAD_ROT, elapsed, false),
        head_pos: warden_roar_channel(&WARDEN_EMERGE_HEAD_POS, elapsed, true),
        right_arm_rot: warden_roar_channel(&WARDEN_EMERGE_RIGHT_ARM_ROT, elapsed, false),
        right_arm_pos: warden_roar_channel(&WARDEN_EMERGE_RIGHT_ARM_POS, elapsed, true),
        left_arm_rot: warden_roar_channel(&WARDEN_EMERGE_LEFT_ARM_ROT, elapsed, false),
        left_arm_pos: warden_roar_channel(&WARDEN_EMERGE_LEFT_ARM_POS, elapsed, true),
        right_leg_rot: warden_roar_channel(&WARDEN_EMERGE_RIGHT_LEG_ROT, elapsed, false),
        right_leg_pos: warden_roar_channel(&WARDEN_EMERGE_RIGHT_LEG_POS, elapsed, true),
        left_leg_rot: warden_roar_channel(&WARDEN_EMERGE_LEFT_LEG_ROT, elapsed, false),
        left_leg_pos: warden_roar_channel(&WARDEN_EMERGE_LEFT_LEG_POS, elapsed, true),
    }
}

/// ONE DIG KEYFRAME (javap `Keyframe` the record the interpolation
/// field the END keyframe's own choice wears — the DIG definition
/// the CATMULLROM walk the MAJORITY keeps, the NINE final keyframes
/// the LINEAR choice bank: the `Interpolations.LINEAR` the plain
/// lerp speaks, the `getstatic #273 LINEAR` the head POS /
/// left_arm POS / the settle tails the bytecode's own rows wear).
/// The vecs the RAW definition words keep — the degrees the
/// 0.017453292 bake multiplies, the POSITION's y the posVec fneg
/// negates — the conversion the EVAL's own head serves.
struct WardenDigKeyframe {
    t: f32,
    v: [f32; 3],
    /// the END keyframe's own interpolation — the vanilla
    /// `Entry.apply` the `keyframe.interpolation()` read speaks: the
    /// false the CATMULLROM quartet answers, the true the LINEAR
    /// lerp keeps
    lin: bool,
}

/// THE WARDEN_DIG DEFINITION (javap `WardenAnimation` clinit pc
/// 4841-7765, verbatim — the `withLength(5.0f)` head the
/// `Mth.ceil(100.0f)` DIGGING_DURATION the 100-tick window wears,
/// the SIX bones the addAnimation chain names — body, head,
/// right_arm, left_arm, right_leg, left_leg — each its ROTATION +
/// POSITION pair the 19/11, 7/2, 10/5, 12/8, 9/8, 9/8 keyframe
/// arrays bank (108 kfs total); the NINE settle keyframes the
/// LINEAR choice wear — the body's (147.5) dive + (0,-40,-8) sink,
/// the head POS (0,0,0) rest, the left arm (-120) fold + (0,0,4)
/// tuck, the legs' (90) stand + (0,-28,-5) drag — the rest the
/// CATMULLROM keeps). THE TIMELINE the pose window wears: the
/// lunge the body's 50° pitch the 0.5s bite banks, the dig the
/// 82-121° shake pairs the 1.0-3.0s window rides, the dive the
/// 147° head-down the 3.4-4.5 tail speaks, the sink the (0,-40,-8)
/// LINEAR the body's own grave wears — the final keyframes the
/// window's own end the removal answers, the warden the world
/// retires the same tick the pose holds.
const WARDEN_DIG_BODY_ROT: [WardenDigKeyframe; 19] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.25, v: [4.13441, 0.94736, 1.2694], lin: false },
    WardenDigKeyframe { t: 0.5, v: [50.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.708, v: [54.4541, -13.5394, -18.1418], lin: false },
    WardenDigKeyframe { t: 1.042, v: [59.4644, -10.8885, 35.7954], lin: false },
    WardenDigKeyframe { t: 1.333, v: [82.2826, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 1.625, v: [53.2361, 10.0472, -29.7293], lin: false },
    WardenDigKeyframe { t: 2.208, v: [-17.7174, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 2.542, v: [112.283, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 2.667, v: [116.069, 5.11581, -24.5012], lin: false },
    WardenDigKeyframe { t: 2.833, v: [121.562, -4.17248, 19.5774], lin: false },
    WardenDigKeyframe { t: 3.042, v: [138.569, 5.11581, -24.5012], lin: false },
    WardenDigKeyframe { t: 3.25, v: [144.062, -4.17248, 19.5774], lin: false },
    WardenDigKeyframe { t: 3.375, v: [147.283, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 3.625, v: [147.283, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 3.875, v: [134.362, 8.81113, -8.90172], lin: false },
    WardenDigKeyframe { t: 4.042, v: [132.06, -8.35927, 9.70506], lin: false },
    WardenDigKeyframe { t: 4.25, v: [134.362, 8.81113, -8.90172], lin: false },
    WardenDigKeyframe { t: 4.5, v: [147.5, 0.0, 0.0], lin: true },
];
const WARDEN_DIG_BODY_POS: [WardenDigKeyframe; 11] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.5, v: [0.0, -16.4845, -6.5784], lin: false },
    WardenDigKeyframe { t: 0.708, v: [0.0, -16.4845, -6.5784], lin: false },
    WardenDigKeyframe { t: 1.042, v: [0.0, -16.97, -7.11], lin: false },
    WardenDigKeyframe { t: 1.625, v: [0.0, -13.97, -7.11], lin: false },
    WardenDigKeyframe { t: 2.208, v: [0.0, -11.4845, -0.5784], lin: false },
    WardenDigKeyframe { t: 2.542, v: [0.0, -16.4845, -6.5784], lin: false },
    WardenDigKeyframe { t: 2.667, v: [0.0, -20.27, -5.42], lin: false },
    WardenDigKeyframe { t: 3.375, v: [0.0, -21.4845, -5.5784], lin: false },
    WardenDigKeyframe { t: 4.042, v: [0.0, -22.4845, -5.5784], lin: false },
    WardenDigKeyframe { t: 4.5, v: [0.0, -40.0, -8.0], lin: true },
];
const WARDEN_DIG_HEAD_ROT: [WardenDigKeyframe; 7] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.667, v: [12.5, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 1.208, v: [12.5, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 1.75, v: [45.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 2.375, v: [-22.5, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 2.542, v: [67.5, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 4.375, v: [67.5, 0.0, 0.0], lin: false },
];
const WARDEN_DIG_HEAD_POS: [WardenDigKeyframe; 2] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 4.375, v: [0.0, 0.0, 0.0], lin: true },
];
const WARDEN_DIG_RIGHT_ARM_ROT: [WardenDigKeyframe; 10] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.5, v: [-101.804, -21.2959, 30.6148], lin: false },
    WardenDigKeyframe { t: 0.708, v: [-101.804, -21.2959, 30.6148], lin: false },
    WardenDigKeyframe { t: 1.0, v: [48.7585, -17.6194, 9.9865], lin: false },
    WardenDigKeyframe { t: 1.167, v: [48.7585, -17.6194, 9.9865], lin: false },
    WardenDigKeyframe { t: 1.458, v: [-101.804, -21.2959, 30.6148], lin: false },
    WardenDigKeyframe { t: 1.75, v: [-89.0499, -4.19657, -1.47845], lin: false },
    WardenDigKeyframe { t: 2.208, v: [-158.307, 3.7152, -1.52352], lin: false },
    WardenDigKeyframe { t: 2.542, v: [-89.0499, -4.19657, -1.47845], lin: false },
    WardenDigKeyframe { t: 4.375, v: [-120.0, 0.0, 0.0], lin: false },
];
const WARDEN_DIG_RIGHT_ARM_POS: [WardenDigKeyframe; 5] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.708, v: [2.22, 0.0, 0.86], lin: false },
    WardenDigKeyframe { t: 1.0, v: [3.12, 0.0, 4.29], lin: false },
    WardenDigKeyframe { t: 2.208, v: [1.0, 0.0, 4.0], lin: false },
    WardenDigKeyframe { t: 4.375, v: [0.0, 0.0, 4.0], lin: false },
];
const WARDEN_DIG_LEFT_ARM_ROT: [WardenDigKeyframe; 12] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.292, v: [-63.8929, -0.52011, 2.09491], lin: false },
    WardenDigKeyframe { t: 0.5, v: [-63.8929, -0.52011, 2.09491], lin: false },
    WardenDigKeyframe { t: 0.708, v: [-62.8786, 15.1506, 9.97445], lin: false },
    WardenDigKeyframe { t: 0.917, v: [-86.9364, 17.4503, 4.05284], lin: false },
    WardenDigKeyframe { t: 1.167, v: [-86.9364, 17.4503, 4.05284], lin: false },
    WardenDigKeyframe { t: 1.458, v: [-86.9364, 17.4503, 4.05284], lin: false },
    WardenDigKeyframe { t: 1.667, v: [63.0984, 8.83573, -8.71284], lin: false },
    WardenDigKeyframe { t: 1.833, v: [35.5984, 8.83573, -8.71284], lin: false },
    WardenDigKeyframe { t: 2.208, v: [-153.275, -0.02953, 3.5235], lin: false },
    WardenDigKeyframe { t: 2.542, v: [-87.0775, -0.02625, 3.132], lin: false },
    WardenDigKeyframe { t: 4.375, v: [-120.0, 0.0, 0.0], lin: true },
];
const WARDEN_DIG_LEFT_ARM_POS: [WardenDigKeyframe; 8] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.5, v: [-0.28, 5.0, 10.0], lin: false },
    WardenDigKeyframe { t: 0.708, v: [-1.51, 4.35, 4.33], lin: false },
    WardenDigKeyframe { t: 0.917, v: [-0.6, 3.61, 4.63], lin: false },
    WardenDigKeyframe { t: 1.167, v: [-0.6, 3.61, 0.63], lin: false },
    WardenDigKeyframe { t: 1.667, v: [-2.85, -0.1, 3.33], lin: false },
    WardenDigKeyframe { t: 2.208, v: [-1.0, 0.0, 4.0], lin: false },
    WardenDigKeyframe { t: 4.375, v: [0.0, 0.0, 4.0], lin: true },
];
const WARDEN_DIG_RIGHT_LEG_ROT: [WardenDigKeyframe; 9] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.5, v: [113.27, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.708, v: [113.27, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 3.333, v: [113.27, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 3.583, v: [182.5, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 3.833, v: [120.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 4.083, v: [182.5, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 4.292, v: [120.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 4.5, v: [90.0, 0.0, 0.0], lin: true },
];
const WARDEN_DIG_RIGHT_LEG_POS: [WardenDigKeyframe; 8] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.5, v: [0.0, -13.98, -2.37], lin: false },
    WardenDigKeyframe { t: 0.708, v: [0.0, -13.98, -2.37], lin: false },
    WardenDigKeyframe { t: 3.333, v: [0.0, -13.98, -2.37], lin: false },
    WardenDigKeyframe { t: 3.583, v: [0.0, -7.0, -3.0], lin: false },
    WardenDigKeyframe { t: 3.833, v: [0.0, -9.0, -3.0], lin: false },
    WardenDigKeyframe { t: 4.083, v: [0.0, -16.71, -3.69], lin: false },
    WardenDigKeyframe { t: 4.292, v: [0.0, -28.0, -5.0], lin: true },
];
const WARDEN_DIG_LEFT_LEG_ROT: [WardenDigKeyframe; 9] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.5, v: [114.98, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.708, v: [114.98, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 3.333, v: [114.98, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 3.583, v: [90.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 3.833, v: [172.5, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 4.083, v: [90.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 4.292, v: [197.5, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 4.5, v: [90.0, 0.0, 0.0], lin: true },
];
const WARDEN_DIG_LEFT_LEG_POS: [WardenDigKeyframe; 8] = [
    WardenDigKeyframe { t: 0.0, v: [0.0, 0.0, 0.0], lin: false },
    WardenDigKeyframe { t: 0.5, v: [0.0, -14.01, -2.35], lin: false },
    WardenDigKeyframe { t: 0.708, v: [0.0, -14.01, -2.35], lin: false },
    WardenDigKeyframe { t: 3.333, v: [0.0, -14.01, -2.35], lin: false },
    WardenDigKeyframe { t: 3.583, v: [0.0, -5.0, -4.0], lin: false },
    WardenDigKeyframe { t: 3.833, v: [0.0, -7.0, -4.0], lin: false },
    WardenDigKeyframe { t: 4.083, v: [0.0, -15.5, -3.76], lin: false },
    WardenDigKeyframe { t: 4.292, v: [0.0, -28.0, -5.0], lin: true },
];

/// THE WARDEN_ATTACK DEFINITION (javap `WardenAnimation` clinit pc
/// 9801-10684, verbatim — the `withLength(0.33333f)` head the
/// melee-swing window the AnimationState's own startTick wears, the
/// FOUR bones the addAnimation chain names — body, head, right_arm,
/// left_arm — each its ROTATION + POSITION pair the 4/4, 4/4, 4/4,
/// 4/4 keyframe arrays bank (32 kfs total); EVERY keyframe the
/// CATMULLROM interpolation wears — the plain `WardenKeyframe`
/// record the ROAR/EMERGE channels share). THE TIMELINE the swing
/// wears: the wind-up the -22.5° body dip + the arms' -120.36°
/// grab pair bank (t=0.0417s — the swing's own 0.83-tick beat),
/// the strike the +22.5° body lunge + the right arm's -90/-45
/// smash + the left arm's -61.16/+42.86/+11.52 cross + the body's
/// (0,-1,-2) drop bank (t=0.1667-0.2083s), the head's +22.5 dip →
/// -30.17 recover + the (0,-2,-2) head drop the 0.25 beat rides,
/// the 0.3333 the (0,0,0) rest the window's own end banks — the
/// past-end walk the same zeros the stale start keeps.
const WARDEN_ATTACK_BODY_ROT: [WardenKeyframe; 4] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.0417, v: [-22.5, 0.0, 0.0] },
    WardenKeyframe { t: 0.2083, v: [22.5, 0.0, 0.0] },
    WardenKeyframe { t: 0.3333, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ATTACK_BODY_POS: [WardenKeyframe; 4] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.0417, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.2083, v: [0.0, -1.0, -2.0] },
    WardenKeyframe { t: 0.3333, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ATTACK_HEAD_ROT: [WardenKeyframe; 4] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.0417, v: [22.5, 0.0, 0.0] },
    WardenKeyframe { t: 0.25, v: [-30.17493, 0.0, 0.0] },
    WardenKeyframe { t: 0.3333, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ATTACK_HEAD_POS: [WardenKeyframe; 4] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.0417, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.25, v: [0.0, -2.0, -2.0] },
    WardenKeyframe { t: 0.3333, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ATTACK_RIGHT_ARM_ROT: [WardenKeyframe; 4] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.0417, v: [-120.36119, 40.78947, -20.94102] },
    WardenKeyframe { t: 0.1667, v: [-90.0, -45.0, 0.0] },
    WardenKeyframe { t: 0.3333, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ATTACK_RIGHT_ARM_POS: [WardenKeyframe; 4] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.0417, v: [4.0, 0.0, 5.0] },
    WardenKeyframe { t: 0.1667, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.3333, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ATTACK_LEFT_ARM_ROT: [WardenKeyframe; 4] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.0417, v: [-120.36119, -40.78947, 20.94102] },
    WardenKeyframe { t: 0.1667, v: [-61.1632, 42.85882, 11.52421] },
    WardenKeyframe { t: 0.3333, v: [0.0, 0.0, 0.0] },
];
const WARDEN_ATTACK_LEFT_ARM_POS: [WardenKeyframe; 4] = [
    WardenKeyframe { t: 0.0, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.0417, v: [-4.0, 0.0, 5.0] },
    WardenKeyframe { t: 0.1667, v: [0.0, 0.0, 0.0] },
    WardenKeyframe { t: 0.3333, v: [0.0, 0.0, 0.0] },
];
/// (the definition's own `withLength(5.0f)` head — the 100-tick
/// window the 20-tick second wears — the final LINEAR keyframes'
/// own timestamps the settle speaks, the removal the window's own
/// end the world serves)

/// THE DIG'S OWN CHANNEL WALK (javap `KeyframeAnimation$Entry.apply`
/// pc 0-138 the SAME binary-search walk the roar lane shares — the
/// END keyframe's own interpolation the delta's apply wears: the
/// `keyframe.interpolation()` read the LIN flag banks, the LINEAR
/// the plain lerp keeps (javap `Interpolations$1.apply` — the
/// `offset(start, end, delta)` the `start + (end - start) * delta`
/// fold speaks), the CATMULLROM the quartet the clamped p0/p3 the
/// same walk answers)
fn warden_dig_channel(kfs: &[WardenDigKeyframe], elapsed: f32, pos: bool) -> [f32; 3] {
    // THE BINARY SEARCH (Mth.binarySearch pc 0-43, verbatim)
    let n = kfs.len();
    let mut lo = 0usize;
    let mut len = n;
    while len > 0 {
        let half = len / 2;
        let mid = lo + half;
        if elapsed <= kfs[mid].t {
            len = half;
        } else {
            lo = mid + 1;
            len -= half + 1;
        }
    }
    let i = lo.saturating_sub(1);
    let j = (i + 1).min(n - 1);
    let delta = if j == i {
        0.0_f32
    } else {
        ((elapsed - kfs[i].t) / (kfs[j].t - kfs[i].t)).clamp(0.0, 1.0)
    };
    // THE BAKE (the degreeVec/posVec the clinit serves — the
    // rotation the 0.017453292 xyz multiply keeps, the position
    // the y-only fneg wears)
    let bake = |kf: &WardenDigKeyframe| -> [f32; 3] {
        if pos {
            [kf.v[0], -kf.v[1], kf.v[2]]
        } else {
            [
                kf.v[0] * 0.017_453_292_f32,
                kf.v[1] * 0.017_453_292_f32,
                kf.v[2] * 0.017_453_292_f32,
            ]
        }
    };
    let p1 = bake(&kfs[i]);
    let p2 = bake(&kfs[j]);
    // THE END KEYFRAME'S OWN INTERPOLATION (the vanilla
    // `Entry.apply` the end kf's read wears): the LINEAR the plain
    // lerp keeps, the CATMULLROM the quartet the clamped
    // neighbors answer
    if kfs[j].lin {
        return [
            p1[0] + (p2[0] - p1[0]) * delta,
            p1[1] + (p2[1] - p1[1]) * delta,
            p1[2] + (p2[2] - p1[2]) * delta,
        ];
    }
    let p0 = bake(&kfs[i.saturating_sub(1)]);
    let p3 = bake(&kfs[(j + 1).min(n - 1)]);
    [
        mth_catmullrom(delta, p0[0], p1[0], p2[0], p3[0]),
        mth_catmullrom(delta, p0[1], p1[1], p2[1], p3[1]),
        mth_catmullrom(delta, p0[2], p1[2], p2[2], p3[2]),
    ]
}

/// THE DIG'S OWN DELTAS (the WARDEN_DIG definition the 12 channels
/// evaluate — the SAME Entry.apply walk the roar/sniff/emerge lanes
/// share, the additive offsets the `ModelPart
/// .offsetRotation`/`offsetPos` walk speaks, the setupAnim's own
/// order the dig apply the EMERGE's own apply PRECEDES): the `None`
/// the never-armed state keeps (the `ifStarted` gate the ALL-ZERO
/// no-op wears — the standing warden the dig clock the None keeps),
/// the `Some(ticks)` the `getTimeInMillis` *50 /1000 the elapsed
/// seconds answer.
struct WardenDigDeltas {
    body_rot: [f32; 3],
    body_pos: [f32; 3],
    head_rot: [f32; 3],
    head_pos: [f32; 3],
    right_arm_rot: [f32; 3],
    right_arm_pos: [f32; 3],
    left_arm_rot: [f32; 3],
    left_arm_pos: [f32; 3],
    right_leg_rot: [f32; 3],
    right_leg_pos: [f32; 3],
    left_leg_rot: [f32; 3],
    left_leg_pos: [f32; 3],
}

fn warden_dig_deltas(state: &WardenState) -> WardenDigDeltas {
    let zero = [0.0_f32; 3];
    let Some(dig_ticks) = state.dig_time else {
        // the never-started state the apply no-op wears
        return WardenDigDeltas {
            body_rot: zero,
            body_pos: zero,
            head_rot: zero,
            head_pos: zero,
            right_arm_rot: zero,
            right_arm_pos: zero,
            left_arm_rot: zero,
            left_arm_pos: zero,
            right_leg_rot: zero,
            right_leg_pos: zero,
            left_leg_rot: zero,
            left_leg_pos: zero,
        };
    };
    // THE ELAPSED SECONDS (the `(age - startTick) * 50` the millis
    // fold keeps, the `/1000.0f` the seconds speak — the two-step
    // the bytecode's own order wears)
    let elapsed = (dig_ticks * 50.0) / 1000.0;
    WardenDigDeltas {
        body_rot: warden_dig_channel(&WARDEN_DIG_BODY_ROT, elapsed, false),
        body_pos: warden_dig_channel(&WARDEN_DIG_BODY_POS, elapsed, true),
        head_rot: warden_dig_channel(&WARDEN_DIG_HEAD_ROT, elapsed, false),
        head_pos: warden_dig_channel(&WARDEN_DIG_HEAD_POS, elapsed, true),
        right_arm_rot: warden_dig_channel(&WARDEN_DIG_RIGHT_ARM_ROT, elapsed, false),
        right_arm_pos: warden_dig_channel(&WARDEN_DIG_RIGHT_ARM_POS, elapsed, true),
        left_arm_rot: warden_dig_channel(&WARDEN_DIG_LEFT_ARM_ROT, elapsed, false),
        left_arm_pos: warden_dig_channel(&WARDEN_DIG_LEFT_ARM_POS, elapsed, true),
        right_leg_rot: warden_dig_channel(&WARDEN_DIG_RIGHT_LEG_ROT, elapsed, false),
        right_leg_pos: warden_dig_channel(&WARDEN_DIG_RIGHT_LEG_POS, elapsed, true),
        left_leg_rot: warden_dig_channel(&WARDEN_DIG_LEFT_LEG_ROT, elapsed, false),
        left_leg_pos: warden_dig_channel(&WARDEN_DIG_LEFT_LEG_POS, elapsed, true),
    }
}

/// THE ATTACK'S OWN DELTAS (the WARDEN_ATTACK definition the 8
/// channels evaluate — the SAME Entry.apply walk the roar/sniff/
/// emerge/dig lanes share, the additive offsets the `ModelPart
/// .offsetRotation`/`offsetPos` walk speaks, the setupAnim's own
/// order the ATTACK apply the sonicBoom/dig/emerge/roar/sniff
/// applies PRECEDES — the FIRST keyframe lane the chain wears): the
/// `None` the never-armed state keeps (the `ifStarted` gate the
/// ALL-ZERO no-op wears — the standing warden the attack clock the
/// None keeps), the `Some(ticks)` the `getTimeInMillis` *50 /1000
/// the elapsed seconds answer — the past-end walk the rest tail
/// the same zeros the swing's own end keeps (the AnimationState
/// the stop call never carries, the stale start the no-op wears).
struct WardenAttackDeltas {
    body_rot: [f32; 3],
    body_pos: [f32; 3],
    head_rot: [f32; 3],
    head_pos: [f32; 3],
    right_arm_rot: [f32; 3],
    right_arm_pos: [f32; 3],
    left_arm_rot: [f32; 3],
    left_arm_pos: [f32; 3],
}

fn warden_attack_deltas(state: &WardenState) -> WardenAttackDeltas {
    let zero = [0.0_f32; 3];
    let Some(attack_ticks) = state.attack_time else {
        // the never-started state the apply no-op wears
        return WardenAttackDeltas {
            body_rot: zero,
            body_pos: zero,
            head_rot: zero,
            head_pos: zero,
            right_arm_rot: zero,
            right_arm_pos: zero,
            left_arm_rot: zero,
            left_arm_pos: zero,
        };
    };
    // THE ELAPSED SECONDS (the `(age - startTick) * 50` the millis
    // fold keeps, the `/1000.0f` the seconds speak — the two-step
    // the bytecode's own order wears)
    let elapsed = (attack_ticks * 50.0) / 1000.0;
    WardenAttackDeltas {
        body_rot: warden_roar_channel(&WARDEN_ATTACK_BODY_ROT, elapsed, false),
        body_pos: warden_roar_channel(&WARDEN_ATTACK_BODY_POS, elapsed, true),
        head_rot: warden_roar_channel(&WARDEN_ATTACK_HEAD_ROT, elapsed, false),
        head_pos: warden_roar_channel(&WARDEN_ATTACK_HEAD_POS, elapsed, true),
        right_arm_rot: warden_roar_channel(&WARDEN_ATTACK_RIGHT_ARM_ROT, elapsed, false),
        right_arm_pos: warden_roar_channel(&WARDEN_ATTACK_RIGHT_ARM_POS, elapsed, true),
        left_arm_rot: warden_roar_channel(&WARDEN_ATTACK_LEFT_ARM_ROT, elapsed, false),
        left_arm_pos: warden_roar_channel(&WARDEN_ATTACK_LEFT_ARM_POS, elapsed, true),
    }
}

fn build_warden_parts(state: &WardenState) -> Part {
    let f = warden_folds(state);
    // THE ATTACK'S OWN OFFSETS (the setupAnim's own order the ATTACK
    // apply the sonicBoom/dig/emerge/roar/sniff applies PRECEDES —
    // the FIRST keyframe lane the additive chain wears; the swing
    // the body/head/arms the def names, the legs the attack never
    // moves)
    let a = warden_attack_deltas(state);
    // THE ROAR'S OWN OFFSETS (the keyframe deltas the folds' own
    // tail ADDS — the `offsetRotation`/`offsetPos` the vanilla walk
    // speaks, the setupAnim order the keyframe applies the LAST
    // keeps; the None the all-zero the no-op wears — the fresh
    // warden the same numbers the folds alone speak)
    let r = warden_roar_deltas(state);
    // THE SNIFF'S OWN OFFSETS (the setupAnim's own tail order the
    // sniff apply the roar's own apply FOLLOWS — each the additive
    // walk wears, the two states the pose law never co-arms — the
    // sum the active one's deltas the pair speaks)
    let s = warden_sniff_deltas(state);
    // THE EMERGE'S OWN OFFSETS (the setupAnim's own order the
    // dig→emerge pair the roar's apply PRECEDES — the additive walk
    // the same law wears; the legs the def the rot + pos pairs the
    // body's own depth carries)
    let e = warden_emerge_deltas(state);
    // THE DIG'S OWN OFFSETS (the setupAnim's own order the dig apply
    // the EMERGE's own apply PRECEDES — the additive walk the same
    // law wears; the dive the body's own 147° pitch the sink's
    // (0,-40,-8) grave serves)
    let d = warden_dig_deltas(state);
    // THE BODY CUBE (the one cube the two ribcages' parent wears —
    // texOffs(0,0) addBox(-9,-13,-4,18,21,11))
    let body_part = Part {
        pivot: [
            0.0 + a.body_pos[0] + d.body_pos[0] + r.body_pos[0] + e.body_pos[0],
            -21.0 + a.body_pos[1] + d.body_pos[1] + r.body_pos[1] + e.body_pos[1],
            0.0 + a.body_pos[2] + d.body_pos[2] + r.body_pos[2] + e.body_pos[2],
        ],
        x_rot: f.body_x + a.body_rot[0] + d.body_rot[0] + e.body_rot[0] + r.body_rot[0]
            + s.body_rot[0],
        y_rot: a.body_rot[1] + d.body_rot[1] + e.body_rot[1] + r.body_rot[1] + s.body_rot[1],
        z_rot: f.body_z + a.body_rot[2] + d.body_rot[2] + e.body_rot[2] + r.body_rot[2]
            + s.body_rot[2],
        cubes: vec![Cube {
            tex: (0.0, 0.0),
            origin: [-9.0, -13.0, -4.0],
            size: [18.0, 21.0, 11.0],
            inflate: 0.0,
            mirror: false,
        }],
        children: vec![
            // THE HEAD (texOffs(0,32) addBox(-8,-16,-5,16,16,10) at
            // (0,-13,0) — the tendrils its children ride)
            Part {
                pivot: [
                    0.0 + a.head_pos[0] + d.head_pos[0] + r.head_pos[0] + e.head_pos[0],
                    -13.0 + a.head_pos[1] + d.head_pos[1] + r.head_pos[1] + e.head_pos[1],
                    0.0 + a.head_pos[2] + d.head_pos[2] + r.head_pos[2] + e.head_pos[2],
                ],
                x_rot: f.head_x + a.head_rot[0] + d.head_rot[0] + e.head_rot[0]
                    + r.head_rot[0] + s.head_rot[0],
                y_rot: f.head_y + a.head_rot[1] + d.head_rot[1] + e.head_rot[1]
                    + r.head_rot[1] + s.head_rot[1],
                z_rot: f.head_z + a.head_rot[2] + d.head_rot[2] + e.head_rot[2]
                    + r.head_rot[2] + s.head_rot[2],
                cubes: vec![Cube {
                    tex: (0.0, 32.0),
                    origin: [-8.0, -16.0, -5.0],
                    size: [16.0, 16.0, 10.0],
                    inflate: 0.0,
                    mirror: false,
                }],
                children: vec![
                    // THE RIGHT TENDRIL (texOffs(52,32)
                    // addBox(-16,-13,0,16,16,0) at (-8,-12,0))
                    Part {
                        pivot: [-8.0, -12.0, 0.0],
                        x_rot: f.swing,
                        y_rot: 0.0,
                        z_rot: 0.0,
                        cubes: vec![Cube {
                            tex: (52.0, 32.0),
                            origin: [-16.0, -13.0, 0.0],
                            size: [16.0, 16.0, 0.0],
                            inflate: 0.0,
                            mirror: false,
                        }],
                        children: vec![],
                    },
                    // THE LEFT TENDRIL (texOffs(58,0)
                    // addBox(0,-13,0,16,16,0) at (8,-12,0))
                    Part {
                        pivot: [8.0, -12.0, 0.0],
                        x_rot: -f.swing,
                        y_rot: 0.0,
                        z_rot: 0.0,
                        cubes: vec![Cube {
                            tex: (58.0, 0.0),
                            origin: [0.0, -13.0, 0.0],
                            size: [16.0, 16.0, 0.0],
                            inflate: 0.0,
                            mirror: false,
                        }],
                        children: vec![],
                    },
                ],
            },
            // THE RIGHT RIBCAGE (texOffs(90,11)
            // addBox(-2,-11,-0.1,9,21,0) at (-7,-2,-4))
            Part {
                pivot: [-7.0, -2.0, -4.0],
                x_rot: 0.0,
                y_rot: 0.0,
                z_rot: 0.0,
                cubes: vec![Cube {
                    tex: (90.0, 11.0),
                    origin: [-2.0, -11.0, -0.1],
                    size: [9.0, 21.0, 0.0],
                    inflate: 0.0,
                    mirror: false,
                }],
                children: vec![],
            },
            // THE LEFT RIBCAGE (the mirror flag the builder wears —
            // addBox(-7,-11,-0.1,9,21,0) at (7,-2,-4), the
            // `mirror(false)` reset the tail keeps)
            Part {
                pivot: [7.0, -2.0, -4.0],
                x_rot: 0.0,
                y_rot: 0.0,
                z_rot: 0.0,
                cubes: vec![Cube {
                    tex: (90.0, 11.0),
                    origin: [-7.0, -11.0, -0.1],
                    size: [9.0, 21.0, 0.0],
                    inflate: 0.0,
                    mirror: true,
                }],
                children: vec![],
            },
            // THE RIGHT ARM (texOffs(44,50) addBox(-4,0,-4,8,28,8)
            // at (-13,-13,1))
            Part {
                pivot: [
                    -13.0 + a.right_arm_pos[0] + d.right_arm_pos[0] + r.right_arm_pos[0]
                        + e.right_arm_pos[0],
                    -13.0 + a.right_arm_pos[1] + d.right_arm_pos[1] + r.right_arm_pos[1]
                        + e.right_arm_pos[1],
                    1.0 + a.right_arm_pos[2] + d.right_arm_pos[2] + r.right_arm_pos[2]
                        + e.right_arm_pos[2],
                ],
                x_rot: f.right_arm_x + a.right_arm_rot[0] + d.right_arm_rot[0]
                    + e.right_arm_rot[0] + r.right_arm_rot[0] + s.right_arm_rot[0],
                y_rot: a.right_arm_rot[1] + d.right_arm_rot[1] + e.right_arm_rot[1]
                    + r.right_arm_rot[1] + s.right_arm_rot[1],
                z_rot: a.right_arm_rot[2] + d.right_arm_rot[2] + e.right_arm_rot[2]
                    + r.right_arm_rot[2] + s.right_arm_rot[2],
                cubes: vec![Cube {
                    tex: (44.0, 50.0),
                    origin: [-4.0, 0.0, -4.0],
                    size: [8.0, 28.0, 8.0],
                    inflate: 0.0,
                    mirror: false,
                }],
                children: vec![],
            },
            // THE LEFT ARM (texOffs(0,58) addBox(-4,0,-4,8,28,8) at
            // (13,-13,1))
            Part {
                pivot: [
                    13.0 + a.left_arm_pos[0] + d.left_arm_pos[0] + r.left_arm_pos[0]
                        + e.left_arm_pos[0],
                    -13.0 + a.left_arm_pos[1] + d.left_arm_pos[1] + r.left_arm_pos[1]
                        + e.left_arm_pos[1],
                    1.0 + a.left_arm_pos[2] + d.left_arm_pos[2] + r.left_arm_pos[2]
                        + e.left_arm_pos[2],
                ],
                x_rot: f.left_arm_x + a.left_arm_rot[0] + d.left_arm_rot[0]
                    + e.left_arm_rot[0] + r.left_arm_rot[0] + s.left_arm_rot[0],
                y_rot: a.left_arm_rot[1] + d.left_arm_rot[1] + e.left_arm_rot[1]
                    + r.left_arm_rot[1] + s.left_arm_rot[1],
                z_rot: a.left_arm_rot[2] + d.left_arm_rot[2] + e.left_arm_rot[2]
                    + r.left_arm_rot[2] + s.left_arm_rot[2],
                cubes: vec![Cube {
                    tex: (0.0, 58.0),
                    origin: [-4.0, 0.0, -4.0],
                    size: [8.0, 28.0, 8.0],
                    inflate: 0.0,
                    mirror: false,
                }],
                children: vec![],
            },
        ],
    };
    // THE ROOT BONE (PartPose.offset(0, 24, 0) no cubes keeps — the
    // body and the legs its children ride, the hierarchy the
    // addOrReplaceChild chain banks)
    Part {
        pivot: [0.0, 24.0, 0.0],
        x_rot: 0.0,
        y_rot: 0.0,
        z_rot: 0.0,
        cubes: vec![],
        children: vec![
            body_part,
            // THE RIGHT LEG (texOffs(76,48) addBox(-3.1,0,-3,6,13,6)
            // at (-5.9,-13,0) — the EMERGE def the leg channels the
            // risen depth carries)
            Part {
                pivot: [
                    -5.9 + d.right_leg_pos[0] + e.right_leg_pos[0],
                    -13.0 + d.right_leg_pos[1] + e.right_leg_pos[1],
                    0.0 + d.right_leg_pos[2] + e.right_leg_pos[2],
                ],
                x_rot: f.right_leg_x + d.right_leg_rot[0] + e.right_leg_rot[0],
                y_rot: d.right_leg_rot[1] + e.right_leg_rot[1],
                z_rot: d.right_leg_rot[2] + e.right_leg_rot[2],
                cubes: vec![Cube {
                    tex: (76.0, 48.0),
                    origin: [-3.1, 0.0, -3.0],
                    size: [6.0, 13.0, 6.0],
                    inflate: 0.0,
                    mirror: false,
                }],
                children: vec![],
            },
            // THE LEFT LEG (texOffs(76,76) addBox(-2.9,0,-3,6,13,6)
            // at (5.9,-13,0) — the EMERGE def the leg channels the
            // risen depth carries)
            Part {
                pivot: [
                    5.9 + d.left_leg_pos[0] + e.left_leg_pos[0],
                    -13.0 + d.left_leg_pos[1] + e.left_leg_pos[1],
                    0.0 + d.left_leg_pos[2] + e.left_leg_pos[2],
                ],
                x_rot: f.left_leg_x + d.left_leg_rot[0] + e.left_leg_rot[0],
                y_rot: d.left_leg_rot[1] + e.left_leg_rot[1],
                z_rot: d.left_leg_rot[2] + e.left_leg_rot[2],
                cubes: vec![Cube {
                    tex: (76.0, 76.0),
                    origin: [-2.9, 0.0, -3.0],
                    size: [6.0, 13.0, 6.0],
                    inflate: 0.0,
                    mirror: false,
                }],
                children: vec![],
            },
        ],
    }
}

// === THE WARDEN'S EMISSIVE LAYERS (javap `WardenRenderer`'s ctor +
// `LivingEntityEmissiveLayer.submit` the 26.2 jar speaks — the FIVE
// fullbright overlays the ctor's addLayer order keeps, each its own
// pruned mesh + texture + alpha function, the `RenderType.eyes` the
// bufferProvider lambda serves) ===

/// ONE EMISSIVE LANE (javap `WardenRenderer`'s ctor the 26.2 jar
/// speaks — the addLayer ORDER the draw sequence wears, each layer's
/// pruned mesh the `retainExactParts` walk banks: the NAMED parts
/// keep their cubes (`clearRecursively` the descendants drop), the
/// unnamed parts become POSE-ONLY stubs the
/// `addOrReplaceChild(String, PartDefinition)` merge law re-parents —
/// the kept cubes ride the FULL ancestor chain the stubs preserve).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WardenLayer {
    /// layer 1 — the `retainExactParts(Set.of("head", "left_arm",
    /// "right_arm", "left_leg", "right_leg"))` mesh, the
    /// BIOLUMINESCENT texture, the alpha-1.0 const lambda
    /// (`lambda$new$1: fconst_1`) the constant glow speaks
    Bioluminescent,
    /// layer 2 — the `retainExactParts(Set.of("body", "head", arms,
    /// legs))` mesh, the SPOTS-1 texture, the
    /// `max(0.25, Mth.cos(age * 0.045) * 0.25)` alpha lambda
    /// (`lambda$new$3`)
    PulsatingSpots1,
    /// layer 3 — the SAME pruned mesh, the SPOTS-2 texture, the
    /// ANTI-PHASE `max(0.25, Mth.cos(age * 0.045 + 3.1415927f) *
    /// 0.25)` lambda (`lambda$new$5`)
    PulsatingSpots2,
    /// layer 4 — the `retainExactParts(Set.of("left_tendril",
    /// "right_tendril"))` mesh (the head the pose-only stub keeps —
    /// the tendrils its re-parented children ride), the BASE warden
    /// texture (`lambda$new$6` returns TEXTURE), the
    /// `tendrilAnimation` word the getter serves (`lambda$new$7`)
    Tendrils,
    /// layer 5 — the `retainExactParts(Set.of("body"))` mesh, the
    /// HEART texture, the `heartAnimation` word (`lambda$new$9`)
    Heart,
}

impl WardenLayer {
    /// THE LAYER'S OWN TEXTURE (javap the texture lambdas — the
    /// tendrils layer the BASE warden.png wears, the others their OWN
    /// emissive textures keep).
    pub fn runtime_path(self) -> Option<&'static str> {
        match self {
            WardenLayer::Bioluminescent => Some(skin::WARDEN_BIOLUM_RUNTIME_PATH),
            WardenLayer::PulsatingSpots1 => Some(skin::WARDEN_SPOTS1_RUNTIME_PATH),
            WardenLayer::PulsatingSpots2 => Some(skin::WARDEN_SPOTS2_RUNTIME_PATH),
            WardenLayer::Tendrils => None, // the BASE texture the warden body wears
            WardenLayer::Heart => Some(skin::WARDEN_HEART_RUNTIME_PATH),
        }
    }

    /// THE LAYER'S OWN SHA1 (the same lambdas' texture words).
    pub fn png_sha1(self) -> Option<&'static str> {
        match self {
            WardenLayer::Bioluminescent => Some(skin::WARDEN_BIOLUM_PNG_SHA1),
            WardenLayer::PulsatingSpots1 => Some(skin::WARDEN_SPOTS1_PNG_SHA1),
            WardenLayer::PulsatingSpots2 => Some(skin::WARDEN_SPOTS2_PNG_SHA1),
            WardenLayer::Tendrils => Some(skin::WARDEN_PNG_SHA1),
            WardenLayer::Heart => Some(skin::WARDEN_HEART_PNG_SHA1),
        }
    }

    /// THE LAYER'S OWN ALPHA FUNCTION (javap `LivingEntityEmissiveLayer
    /// .submit` pc 16-32 — the AlphaFunction the render state's age
    /// clock consults; the `<= 1.0E-5` gate the caller's skip keeps):
    /// the bioluminescent the 1.0 const keeps, the TWO spot layers the
    /// `max(0.25, Mth.cos(age * 0.045 [* + 3.1415927]) * 0.25)` pair
    /// wears — the cos halves the max-0.25 floor CLAMPS (the pulse the
    /// bytecode's own arithmetic keeps: the product never exceeds the
    /// floor, the constant 0.25 the honest answer), the tendrils the
    /// shiver clock's getter word, the heart the heartbeat's.
    pub fn alpha_at(self, state: &WardenState, tendril: f32, heart: f32) -> f32 {
        match self {
            WardenLayer::Bioluminescent => 1.0,
            WardenLayer::PulsatingSpots1 => {
                0.25f32.max(mth_cos(state.age_ticks * 0.045) * 0.25)
            }
            WardenLayer::PulsatingSpots2 => {
                0.25f32.max(mth_cos(state.age_ticks * 0.045 + std::f32::consts::PI) * 0.25)
            }
            WardenLayer::Tendrils => tendril,
            WardenLayer::Heart => heart,
        }
    }
}

/// THE LAYER'S OWN PRUNED MESH (javap `PartDefinition.retainExactParts`
/// the 26.2 jar speaks, verbatim): the NAMED parts keep their cubes
/// (`clearRecursively` the descendants drop), the unnamed parts become
/// POSE-ONLY stubs the two-arg `addOrReplaceChild` merge law
/// re-parents (`newChild.children.putAll(old.children)`) — the kept
/// cubes ride the FULL ancestor chain. The ribcages NO layer keeps —
/// their cubes every pruned mesh drops. The FULLBRIGHT pass the eyes
/// shader serves: the entity diffuse NEVER applies (the rendertype_eyes
/// vertex color the model color alone keeps), the shade the 1.0 the
/// caller's tint multiplies.
pub fn warden_layer_quads(
    state: &WardenState,
    feet: Vec3,
    cam_yaw: f32,
    cam_pitch: f32,
    layer: WardenLayer,
) -> Vec<AvatarQuad> {
    let f = warden_folds(state);
    // THE ROAR'S OWN OFFSETS (the layer models bake their OWN
    // WardenModel instances whose setupAnim runs the SAME keyframe
    // applies — the deltas the folds' own tail ADDS here too)
    let r = warden_roar_deltas(state);
    // THE SNIFF'S OWN OFFSETS (the layer models' own setupAnim the
    // SAME tail order wears — the sniff apply the additive walk
    // keeps here too)
    let s = warden_sniff_deltas(state);
    // THE EMERGE'S OWN OFFSETS (the layer models' own setupAnim the
    // SAME dig→emerge order wears — the additive walk the same law
    // keeps here too)
    let e = warden_emerge_deltas(state);
    // THE DIG'S OWN OFFSETS (the setupAnim's own order the dig apply
    // the EMERGE's own apply PRECEDES — the additive walk the same
    // law wears; the dive the body's own 147° pitch the sink's
    // (0,-40,-8) grave serves)
    let d = warden_dig_deltas(state);
    // THE ATTACK'S OWN OFFSETS (the layer models' own setupAnim the
    // SAME attack-first order wears — the ATTACK apply the
    // sonicBoom/dig/emerge/roar/sniff applies PRECEDES, the
    // additive walk the same law keeps here too)
    let a = warden_attack_deltas(state);
    // the retain law's per-part verdicts (the named sets the lambdas
    // bank: biolum {head, arms, legs}, spots {body, head, arms, legs},
    // tendrils {the tendril pair}, heart {body})
    let spots = matches!(
        layer,
        WardenLayer::PulsatingSpots1 | WardenLayer::PulsatingSpots2
    );
    let body_kept = spots || layer == WardenLayer::Heart;
    let head_kept = spots || layer == WardenLayer::Bioluminescent;
    let limbs_kept = spots || layer == WardenLayer::Bioluminescent;
    let tendril_kept = layer == WardenLayer::Tendrils;
    let empty: Vec<Cube> = Vec::new();
    // THE BODY (the stub law: the unnamed part's cubes drop, the pose
    // rides — the head/limb children the merge law re-parents)
    let body_part = Part {
        pivot: [
            0.0 + a.body_pos[0] + d.body_pos[0] + r.body_pos[0] + e.body_pos[0],
            -21.0 + a.body_pos[1] + d.body_pos[1] + r.body_pos[1] + e.body_pos[1],
            0.0 + a.body_pos[2] + d.body_pos[2] + r.body_pos[2] + e.body_pos[2],
        ],
        x_rot: f.body_x + a.body_rot[0] + d.body_rot[0] + e.body_rot[0] + r.body_rot[0]
            + s.body_rot[0],
        y_rot: a.body_rot[1] + d.body_rot[1] + e.body_rot[1] + r.body_rot[1] + s.body_rot[1],
        z_rot: f.body_z + a.body_rot[2] + d.body_rot[2] + e.body_rot[2] + r.body_rot[2]
            + s.body_rot[2],
        cubes: if body_kept {
            vec![Cube {
                tex: (0.0, 0.0),
                origin: [-9.0, -13.0, -4.0],
                size: [18.0, 21.0, 11.0],
                inflate: 0.0,
                mirror: false,
            }]
        } else {
            empty.clone()
        },
        children: vec![
            // THE HEAD (kept: the tendril children the
            // clearRecursively law drops — the stub: the tendrils the
            // re-parent law keeps riding)
            Part {
                pivot: [
                    0.0 + a.head_pos[0] + d.head_pos[0] + r.head_pos[0] + e.head_pos[0],
                    -13.0 + a.head_pos[1] + d.head_pos[1] + r.head_pos[1] + e.head_pos[1],
                    0.0 + a.head_pos[2] + d.head_pos[2] + r.head_pos[2] + e.head_pos[2],
                ],
                x_rot: f.head_x + a.head_rot[0] + d.head_rot[0] + e.head_rot[0]
                    + r.head_rot[0] + s.head_rot[0],
                y_rot: f.head_y + a.head_rot[1] + d.head_rot[1] + e.head_rot[1]
                    + r.head_rot[1] + s.head_rot[1],
                z_rot: f.head_z + a.head_rot[2] + d.head_rot[2] + e.head_rot[2]
                    + r.head_rot[2] + s.head_rot[2],
                cubes: if head_kept {
                    vec![Cube {
                        tex: (0.0, 32.0),
                        origin: [-8.0, -16.0, -5.0],
                        size: [16.0, 16.0, 10.0],
                        inflate: 0.0,
                        mirror: false,
                    }]
                } else {
                    empty.clone()
                },
                children: vec![
                    Part {
                        pivot: [-8.0, -12.0, 0.0],
                        x_rot: f.swing,
                        y_rot: 0.0,
                        z_rot: 0.0,
                        cubes: if tendril_kept {
                            vec![Cube {
                                tex: (52.0, 32.0),
                                origin: [-16.0, -13.0, 0.0],
                                size: [16.0, 16.0, 0.0],
                                inflate: 0.0,
                                mirror: false,
                            }]
                        } else {
                            empty.clone()
                        },
                        children: vec![],
                    },
                    Part {
                        pivot: [8.0, -12.0, 0.0],
                        x_rot: -f.swing,
                        y_rot: 0.0,
                        z_rot: 0.0,
                        cubes: if tendril_kept {
                            vec![Cube {
                                tex: (58.0, 0.0),
                                origin: [0.0, -13.0, 0.0],
                                size: [16.0, 16.0, 0.0],
                                inflate: 0.0,
                                mirror: false,
                            }]
                        } else {
                            empty.clone()
                        },
                        children: vec![],
                    },
                ],
            },
            // THE RIBCAGES (NO layer's set keeps them — the stubs the
            // pose keeps, the cubes every mesh drops)
            Part {
                pivot: [-7.0, -2.0, -4.0],
                x_rot: 0.0,
                y_rot: 0.0,
                z_rot: 0.0,
                cubes: vec![],
                children: vec![],
            },
            Part {
                pivot: [7.0, -2.0, -4.0],
                x_rot: 0.0,
                y_rot: 0.0,
                z_rot: 0.0,
                cubes: vec![],
                children: vec![],
            },
            // THE ARMS
            Part {
                pivot: [
                    -13.0 + a.right_arm_pos[0] + d.right_arm_pos[0] + r.right_arm_pos[0]
                        + e.right_arm_pos[0],
                    -13.0 + a.right_arm_pos[1] + d.right_arm_pos[1] + r.right_arm_pos[1]
                        + e.right_arm_pos[1],
                    1.0 + a.right_arm_pos[2] + d.right_arm_pos[2] + r.right_arm_pos[2]
                        + e.right_arm_pos[2],
                ],
                x_rot: f.right_arm_x + a.right_arm_rot[0] + d.right_arm_rot[0]
                    + e.right_arm_rot[0] + r.right_arm_rot[0] + s.right_arm_rot[0],
                y_rot: a.right_arm_rot[1] + d.right_arm_rot[1] + e.right_arm_rot[1]
                    + r.right_arm_rot[1] + s.right_arm_rot[1],
                z_rot: a.right_arm_rot[2] + d.right_arm_rot[2] + e.right_arm_rot[2]
                    + r.right_arm_rot[2] + s.right_arm_rot[2],
                cubes: if limbs_kept {
                    vec![Cube {
                        tex: (44.0, 50.0),
                        origin: [-4.0, 0.0, -4.0],
                        size: [8.0, 28.0, 8.0],
                        inflate: 0.0,
                        mirror: false,
                    }]
                } else {
                    empty.clone()
                },
                children: vec![],
            },
            Part {
                pivot: [
                    13.0 + a.left_arm_pos[0] + d.left_arm_pos[0] + r.left_arm_pos[0]
                        + e.left_arm_pos[0],
                    -13.0 + a.left_arm_pos[1] + d.left_arm_pos[1] + r.left_arm_pos[1]
                        + e.left_arm_pos[1],
                    1.0 + a.left_arm_pos[2] + d.left_arm_pos[2] + r.left_arm_pos[2]
                        + e.left_arm_pos[2],
                ],
                x_rot: f.left_arm_x + a.left_arm_rot[0] + d.left_arm_rot[0]
                    + e.left_arm_rot[0] + r.left_arm_rot[0] + s.left_arm_rot[0],
                y_rot: a.left_arm_rot[1] + d.left_arm_rot[1] + e.left_arm_rot[1]
                    + r.left_arm_rot[1] + s.left_arm_rot[1],
                z_rot: a.left_arm_rot[2] + d.left_arm_rot[2] + e.left_arm_rot[2]
                    + r.left_arm_rot[2] + s.left_arm_rot[2],
                cubes: if limbs_kept {
                    vec![Cube {
                        tex: (0.0, 58.0),
                        origin: [-4.0, 0.0, -4.0],
                        size: [8.0, 28.0, 8.0],
                        inflate: 0.0,
                        mirror: false,
                    }]
                } else {
                    empty.clone()
                },
                children: vec![],
            },
        ],
    };
    // THE ROOT BONE (the pose-only stub the legs re-parented ride)
    let bone = Part {
        pivot: [0.0, 24.0, 0.0],
        x_rot: 0.0,
        y_rot: 0.0,
        z_rot: 0.0,
        cubes: vec![],
        children: vec![
            body_part,
            Part {
                pivot: [
                    -5.9 + d.right_leg_pos[0] + e.right_leg_pos[0],
                    -13.0 + d.right_leg_pos[1] + e.right_leg_pos[1],
                    0.0 + d.right_leg_pos[2] + e.right_leg_pos[2],
                ],
                x_rot: f.right_leg_x + d.right_leg_rot[0] + e.right_leg_rot[0],
                y_rot: d.right_leg_rot[1] + e.right_leg_rot[1],
                z_rot: d.right_leg_rot[2] + e.right_leg_rot[2],
                cubes: if limbs_kept {
                    vec![Cube {
                        tex: (76.0, 48.0),
                        origin: [-3.1, 0.0, -3.0],
                        size: [6.0, 13.0, 6.0],
                        inflate: 0.0,
                        mirror: false,
                    }]
                } else {
                    empty.clone()
                },
                children: vec![],
            },
            Part {
                pivot: [
                    5.9 + d.left_leg_pos[0] + e.left_leg_pos[0],
                    -13.0 + d.left_leg_pos[1] + e.left_leg_pos[1],
                    0.0 + d.left_leg_pos[2] + e.left_leg_pos[2],
                ],
                x_rot: f.left_leg_x + d.left_leg_rot[0] + e.left_leg_rot[0],
                y_rot: d.left_leg_rot[1] + e.left_leg_rot[1],
                z_rot: d.left_leg_rot[2] + e.left_leg_rot[2],
                cubes: if limbs_kept {
                    vec![Cube {
                        tex: (76.0, 76.0),
                        origin: [-2.9, 0.0, -3.0],
                        size: [6.0, 13.0, 6.0],
                        inflate: 0.0,
                        mirror: false,
                    }]
                } else {
                    empty.clone()
                },
                children: vec![],
            },
        ],
    };
    let (fold_point, fold_normal) = entity_folds(state.yaw_deg, 0.0, 0.0, feet);
    let mut out: Vec<AvatarQuad> = Vec::new();
    let ident = Mat4::IDENT;
    collect_part(&bone, &ident, &fold_point, &fold_normal, cam_yaw, cam_pitch, &mut out);
    // THE FULLBRIGHT LAW (the rendertype_eyes shader the entity
    // diffuse never calls — the shade the 1.0 the model color's white
    // RGB keeps)
    for q in &mut out {
        q.shade = 1.0;
    }
    out
}

impl WardenModel {
    /// ONE LAYER TEXTURE'S OWN LOAD (the sha1 the same honesty the
    /// body load keeps — the `None` the layer's absence keeps unseen,
    /// the OTHER layers keep drawing). The tendrils layer the BASE
    /// warden texture wears — the body's own model serves.
    pub fn load_layer(root: &Path, layer: WardenLayer) -> Option<WardenModel> {
        let bytes = std::fs::read(root.join(layer.runtime_path()?)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != layer.png_sha1()? {
            return None;
        }
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        if w != 128 || h != 128 || rgba.len() != 128 * 128 * 4 {
            return None;
        }
        Some(WardenModel { rgba })
    }
}

#[cfg(test)]#[cfg(test)]
mod tests {
    use super::*;

    /// THE QUADRUPED'S OWN EXTENTS (javap PigModel.createBodyLayer's
    /// own numbers): the mesh spans the ground (model y 24) to the
    /// head's top (model y 8) — the feet epsilon the entity chain's
    /// own -1.501 speaks, the 1.0-block body above it.
    #[test]
    fn the_pig_mesh_spans_the_quadruped_height() {
        let pig = PigModel {
            rgba: vec![255u8; 64 * 64 * 4],
        };
        let feet = Vec3::new(10.0, 64.0, 20.0);
        let state = PigState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let quads = pig.quads(&state, feet, 0.0, 0.0);
        assert!(!quads.is_empty(), "the mesh speaks quads");
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        for q in &quads {
            for p in q.pts {
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
                min_x = min_x.min(p[0]);
                max_x = max_x.max(p[0]);
            }
        }
        // model y 24 (the ground) -> feet + 0.001 (the fold's own
        // epsilon); model y 8 (the head's top) -> feet + 1.001
        assert!(
            (min_y - (feet.y as f32 + 0.001)).abs() < 1e-4,
            "the feet ride the ground: {min_y}"
        );
        assert!(
            (max_y - (feet.y as f32 + 1.001)).abs() < 1e-4,
            "the head's top rides 1.0 above the feet: {max_y}"
        );
        // the body's barrel spans -5/16..5/16 (10 texels wide), the
        // head -4/16..4/16 — the widest word is the body's own
        assert!(
            (min_x - (feet.x as f32 - 5.0 / 16.0)).abs() < 1e-4,
            "the body's west face: {min_x}"
        );
        assert!(
            (max_x - (feet.x as f32 + 5.0 / 16.0)).abs() < 1e-4,
            "the body's east face: {max_x}"
        );
    }

    /// THE BOAT'S OWN EXTENTS (the AbstractBoatRenderer.submit fold's
    /// own words): the five-piece hull + two paddles speak 54 quads,
    /// the chest variant's three cubes +18, the raft's two-plank
    /// bottom (no rails) 36 — and the hull floor the RIDE HEIGHT's
    /// own level (the passenger attachment y the height/3 law keeps
    /// = the model's floor slab the S flip stands up).
    #[test]
    fn the_boat_mesh_spans_the_hull_and_rides_the_lift() {
        let boat = BoatModel {
            rgba: vec![255u8; 128 * 64 * 4],
            tex_h: 64,
            raft: false,
        };
        let feet = Vec3::new(10.0, 64.0, -20.0);
        let quads = boat.quads(0.0, false, feet, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(quads.len(), 54, "5 hull parts + 2 paddles, 6 quads a cube");
        let chest = boat.quads(0.0, true, feet, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(chest.len(), 72, "the chest variant's three cubes ride +18");
        let raft = BoatModel {
            rgba: vec![255u8; 128 * 64 * 4],
            tex_h: 64,
            raft: true,
        };
        let raft_quads = raft.quads(0.0, false, feet, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(raft_quads.len(), 36, "the two-plank bottom, no rails");
        let raft_chest = raft.quads(0.0, true, feet, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(raft_chest.len(), 54, "the chest raft rides +18 too");

        // THE HULL'S VERTICAL SPAN (the model numbers' own word): the
        // bottom slab dips to the entity origin (the model y −9 the
        // +0.375 lift cancels), the rails' top at +0.5625 — the 9/16
        // the EntityType's height constant speaks. The PADDLES dip a
        // whisker below (the blades the water's own lane keeps — the
        // rest pose's xRot −0.6545 the rowingTime-0 fold speaks)
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        for q in &quads {
            for p in q.pts {
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
            }
        }
        assert!(
            min_y > 63.95 && min_y <= 64.0 + 1e-4,
            "the keel rides the origin, the paddles dip below: {min_y}"
        );
        // the PADDLE HANDLES rise above the rails (the held oar the
        // rest pose keeps — the rowingTime-0 fold's own silhouette)
        assert!(
            max_y > 64.0 + 9.0 / 16.0 && max_y < 65.0,
            "the handles above the gunwale: {max_y}"
        );
        // the RAILS' own top rides exactly 9/16 above (the hull's own
        // word — the membership pin the extremes' walk keeps)
        let rail_top = quads
            .iter()
            .flat_map(|q| q.pts)
            .find(|p| (p[1] - (64.0 + 9.0 / 16.0)).abs() < 1e-4);
        assert!(rail_top.is_some(), "the rails' top rides 9/16 above");

        // THE FOLD'S OWN CORNER PIN (the 'bottom' part's cube corner
        // (−14,−9,−3) at yaw 0): Rx(π/2) + the pivot (0,3,1) lands
        // model (−0.875, 0.375, −0.5); R_y(90)·S·R_y(180−0) folds it
        // to world (feet − 0.5, feet, feet − 0.875)
        let pin = quads
            .iter()
            .flat_map(|q| q.pts)
            .find(|p| {
                (p[0] - 9.5).abs() < 1e-4 && (p[1] - 64.0).abs() < 1e-4 && (p[2] + 20.875).abs() < 1e-4
            });
        assert!(pin.is_some(), "the keel corner rides the submit fold");
    }

    /// THE PADDLE'S OWN SWING (javap AbstractBoatModel.animatePaddle's
    /// own walk — `xRot = clampedLerp((sin(t)+1)/2, −60°, −15°)`,
    /// `yRot = clampedLerp((sin(t)+1)/2, −45°, +45°)`, the side-1 fold
    /// `yRot = π − yRot`): the rowingTime-0 word IS the rest pose the
    /// previous lane pinned, the phase extremes the sin fold's 0/1
    /// fracs speak, and the ANIMATED quads differ from the rest (the
    /// rowing oar the client's own clock rows).
    #[test]
    fn the_paddle_animates_the_rowing_clock() {
        // THE REST (t = 0): the previous lane's own pins — the left
        // paddle's yRot 0, the right's the side-1 fold's π, both
        // xRot the −37.5° midpoint
        let (rest_x, rest_y_left) = animate_paddle(0.0, 0);
        let (rest_x_r, rest_y_right) = animate_paddle(0.0, 1);
        assert!(
            (rest_x - -0.6544984877109527).abs() < 1e-6,
            "the −0.6545 rest the rowingTime-0 fold speaks: {rest_x}"
        );
        assert!((rest_x_r - rest_x).abs() < 1e-6, "both paddles one rest xRot");
        assert!(rest_y_left.abs() < 1e-6, "the left paddle's yRot 0");
        assert!(
            (rest_y_right - std::f32::consts::PI).abs() < 1e-6,
            "the right paddle's side-1 fold: {rest_y_right}"
        );
        // THE PHASE EXTREMES (the sin fold's own fracs): t = π/2 →
        // frac 1 → xRot −15°, yRot +45° (the right's π − 45°); t =
        // −π/2 → frac 0 → xRot −60°, yRot −45°
        let (peak_x, peak_y) = animate_paddle(std::f32::consts::FRAC_PI_2, 0);
        assert!((peak_x - -0.2617993950843811).abs() < 1e-6, "the −15° peak: {peak_x}");
        assert!((peak_y - 0.7853981852531433).abs() < 1e-6, "the +45° peak: {peak_y}");
        let (low_x, low_y) = animate_paddle(-std::f32::consts::FRAC_PI_2, 0);
        assert!((low_x - -1.0471975803375244).abs() < 1e-6, "the −60° trough: {low_x}");
        assert!((low_y - -0.7853981852531433).abs() < 1e-6, "the −45° trough: {low_y}");
        // THE SIDE-1 FOLD AT THE PEAK (π − 45° the right paddle's own word)
        let (_, peak_y_r) = animate_paddle(std::f32::consts::FRAC_PI_2, 1);
        assert!(
            (peak_y_r - (std::f32::consts::PI - 0.7853981852531433)).abs() < 1e-6,
            "the side-1 fold's own peak: {peak_y_r}"
        );
        // THE ANIMATED MESH (the swing the client's clock rows): the
        // counts the rest keeps (the pose never adds a quad) and the
        // paddle corner the phase MOVES (the rowing oar the rest never
        // freezes)
        let boat = BoatModel {
            rgba: vec![255u8; 128 * 64 * 4],
            tex_h: 64,
            raft: false,
        };
        let feet = Vec3::new(10.0, 64.0, -20.0);
        let rest = boat.quads(0.0, false, feet, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(rest.len(), 54, "the swing never adds a quad");
        let swung = boat.quads(0.0, false, feet, 0.0, 0.0, std::f32::consts::FRAC_PI_2, 0.0, 0.0, 0.0);
        assert_eq!(swung.len(), 54, "the swing keeps the count");
        let moved = rest
            .iter()
            .zip(&swung)
            .any(|(r, s)| r.pts != s.pts);
        assert!(moved, "the rowing phase moves the left paddle's own points");
    }

    /// THE ROCK'S OWN FOLD (the bubble tilt the submit's gate speaks):
    /// the zero rock the pre-rock chain bit-for-bit keeps, the 90°
    /// rock the diagonal's Rodrigues walk moves — the local (0, 0, 1)
    /// bow word through the composed chain lands at the hand-pinned
    /// world spot (the R_y(90) -> S -> R_rock -> R_y(180-yaw) ->
    /// lift+feet order the mulPose sequence keeps).
    #[test]
    fn the_rock_tilts_the_hull_the_diagonal_speaks() {
        let feet = Vec3::new(10.0, 64.0, -20.0);
        let (fold_point, _) = boat_folds(0.0, feet, 0.0, 0.0);
        let rest = fold_point([0.0, 0.0, 1.0]);
        // the rock-free word: R_y(90) -> S -> R_y(180) -> lift + feet —
        // [11.0, 64.375, -20.0] the existing hull pins speak
        assert!((rest[0] - 11.0).abs() < 1e-5, "the rest x: {}", rest[0]);
        assert!((rest[1] - 64.375).abs() < 1e-5, "the rest y: {}", rest[1]);
        assert!((rest[2] - (-20.0)).abs() < 1e-5, "the rest z: {}", rest[2]);
        // THE 90° ROCK (the local word through the diagonal: the
        // Rodrigues walk v' = v·cos + (k×v)·sin + k·(k·v)·(1−cos) over
        // k = (1,0,1)/sqrt(2)) — the bow word swings off its axis
        let (fold_rock, _) = boat_folds(0.0, feet, 90.0, 0.0);
        let rocked = fold_rock([0.0, 0.0, 1.0]);
        assert!((rocked[0] - 10.5).abs() < 1e-4, "the rocked x: {}", rocked[0]);
        assert!(
            (rocked[1] - 63.667892).abs() < 1e-4,
            "the rocked y (the diagonal carries the tilt down): {}",
            rocked[1]
        );
        assert!((rocked[2] - (-19.5)).abs() < 1e-4, "the rocked z: {}", rocked[2]);
        let tilted = rest.iter().zip(&rocked).any(|(r, s)| (r - s).abs() > 1e-5);
        assert!(tilted, "the rock moves the point the rest keeps");
    }

    /// THE HURT TILT'S OWN PIVOT (the submit chain's second arm — the
    /// R_x the hurt degrees speak, riding BETWEEN the rock and the
    /// R_y(180−yaw) the mulPose order keeps): the rest word (hurt 0.0,
    /// the closed gate) the pre-wobble chain bit-for-bit keeps, the
    /// wobble word the X-axis pivot walks — the y the cos fold carries,
    /// the z the sin walk swings, the negate the dir's own flip the
    /// mirror speaks (the y the same, the z the sign's own mirror).
    #[test]
    fn the_hurt_tilt_pivots_the_x_axis() {
        let feet = Vec3::new(10.0, 20.0, 10.0);
        // THE REST (the closed gate the 0.0 keeps): the rock-free chain
        // the existing pins speak — [10.0, 19.375, 10.0] the (0,1,0)
        // local word through R_y(90) -> S -> R_y(180) -> lift+feet
        let (fold_point, _) = boat_folds(0.0, feet, 0.0, 0.0);
        let rest = fold_point([0.0, 1.0, 0.0]);
        assert!((rest[0] - 10.0).abs() < 1e-5, "the rest x: {}", rest[0]);
        assert!((rest[1] - 19.375).abs() < 1e-5, "the rest y: {}", rest[1]);
        assert!((rest[2] - 10.0).abs() < 1e-5, "the rest z: {}", rest[2]);
        // THE WOBBLE (the hit's word the extract+fold speaks — the
        // submit's angle at time 10 / damage 90 / partial 0 the
        // screens' pin carries): the X-axis pivot — the y the cos walk
        // carries to 19.718…, the z the sin walk swings off 10.0
        let (fold_hurt, _) = boat_folds(0.0, feet, 0.0, -48.95623016357422);
        let hurt = fold_hurt([0.0, 1.0, 0.0]);
        assert!((hurt[0] - 10.0).abs() < 1e-5, "the pivot keeps the x: {}", hurt[0]);
        assert!(
            (hurt[1] - 19.718364715576172).abs() < 1e-5,
            "the cos fold's y: {}",
            hurt[1]
        );
        assert!(
            (hurt[2] - 9.2457914352417).abs() < 1e-5,
            "the sin walk's z: {}",
            hurt[2]
        );
        // THE MIRROR (the dir's own flip the negate speaks): the same
        // cos y, the z the sign's own mirror swings the other way
        let (fold_flip, _) = boat_folds(0.0, feet, 0.0, 48.95623016357422);
        let flip = fold_flip([0.0, 1.0, 0.0]);
        assert!(
            (flip[1] - hurt[1]).abs() < 1e-6,
            "the negate keeps the cos walk: {}",
            flip[1]
        );
        assert!(
            (flip[2] - 10.7542085647583).abs() < 1e-5,
            "the sin walk's mirrored z: {}",
            flip[2]
        );
        // THE MOVED POINT (the wobble moves what the rest keeps)
        let moved = rest.iter().zip(&hurt).any(|(r, s)| (r - s).abs() > 1e-5);
        assert!(moved, "the hurt moves the point the rest keeps");
    }

    /// THE BOAT TEXTURE GATE (the load's own honesty — the runtime
    /// extract the sha1s verify): the oak loads (128×64), the chest
    /// band loads (128×128), the unknown wood refuses.
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_boat_model_loads_the_wood_gate() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let oak = BoatModel::load(&root, false, false, "oak").expect("the oak texture loads");
        let quads = oak.quads(0.0, false, Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(quads.len(), 54, "the real texture draws the real hull");
        let chest_oak = BoatModel::load(&root, false, true, "oak").expect("the chest band loads");
        let chest_quads = chest_oak.quads(0.0, true, Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(chest_quads.len(), 72, "the chest cubes ride the real band");
        assert!(
            BoatModel::load(&root, false, false, "unknown_wood").is_none(),
            "the unknown wood refuses"
        );
        let raft = BoatModel::load(&root, true, false, "bamboo").expect("the raft loads");
        let raft_quads = raft.quads(0.0, false, Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(raft_quads.len(), 36, "the plank deck the raft speaks");
    }

    /// THE QUADRUPED WALK (javap QuadrupedModel.setupAnim's own fold):
    /// the DIAGONAL pairs swing together — right-hind + left-front in
    /// phase, left-hind + right-front +PI — the legs' top corners
    /// ride opposite z offsets the mirrored x_rots speak.
    #[test]
    fn the_pig_walk_swings_the_diagonal_pairs() {
        let pig = PigModel {
            rgba: vec![255u8; 64 * 64 * 4],
        };
        let feet = Vec3::new(0.0, 64.0, 0.0);
        let rest = PigState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let walked = PigState {
            walk_pos: 0.0,
            walk_speed: 1.0,
            ..rest
        };
        let rest_quads = pig.quads(&rest, feet, 0.0, 0.0);
        let walked_quads = pig.quads(&walked, feet, 0.0, 0.0);
        assert_eq!(rest_quads.len(), walked_quads.len(), "same mesh, moved legs");
        // THE DIAGONAL PAIR: the right-hind leg's top front corner
        // (pivot (-3, 18, 7) + the box's front-top edge) rides the
        // +1.4 rad swing, the left-hind the -1.4 — their z offsets
        // from rest land OPPOSITE (the cos(+PI) fold)
        let rest_z: Vec<f32> = rest_quads.iter().flat_map(|q| q.pts.iter().map(|p| p[2])).collect();
        let rest_min_z = rest_z.iter().cloned().fold(f32::MAX, f32::min);
        let rest_max_z = rest_z.iter().cloned().fold(f32::MIN, f32::max);
        let walk_z: Vec<f32> = walked_quads
            .iter()
            .flat_map(|q| q.pts.iter().map(|p| p[2]))
            .collect();
        let walk_min_z = walk_z.iter().cloned().fold(f32::MAX, f32::min);
        let walk_max_z = walk_z.iter().cloned().fold(f32::MIN, f32::max);
        // the swing's backward kick passes the rest stand (the legs'
        // own reach — the forward kick never beats the snout's reach,
        // the max the head's word holds)
        assert!(
            walk_min_z < rest_min_z - 0.05,
            "the swing reaches past the stand: rest [{rest_min_z}, {rest_max_z}] walked [{walk_min_z}, {walk_max_z}]"
        );
        // and the legs MOVED — some quad corner left its rest spot
        let moved = walked_quads
            .iter()
            .zip(rest_quads.iter())
            .any(|(wq, rq)| wq.pts.iter().zip(rq.pts.iter()).any(|(wp, rp)| (wp[2] - rp[2]).abs() > 1e-4));
        assert!(moved, "the walk's own swing moved the legs");
    }

    /// THE MIRROR UNWRAP (javap CubeListBuilder.mirror — the LEFT
    /// legs' builders speak it): the mirrored cube's front face
    /// samples the leg rect reversed — the u corners swap, the v
    /// corners stand.
    #[test]
    fn the_pig_left_legs_ride_the_mirror_unwrap() {
        let pig = PigModel {
            rgba: vec![255u8; 64 * 64 * 4],
        };
        let state = PigState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let quads = pig.quads(&state, Vec3::new(0.0, 64.0, 0.0), 0.0, 0.0);
        // the legs' unwrap region: texOffs (0,16), box 4x6x4 — every
        // face rect inside u [0,16), v [16,26)
        let leg_quads: Vec<&AvatarQuad> = quads
            .iter()
            .filter(|q| {
                q.uv.iter().all(|c| c[0] >= 0.0 && c[0] <= 16.0 && c[1] >= 16.0 && c[1] <= 26.0)
            })
            .collect();
        assert_eq!(leg_quads.len(), 24, "4 legs x 6 faces");
        // the unmirrored front face (k=0 -> (u1,v0)): [[8,20],[4,20],[4,26],[8,26]]
        let plain_front = leg_quads.iter().any(|q| {
            q.uv[0] == [8.0, 20.0]
                && q.uv[1] == [4.0, 20.0]
                && q.uv[2] == [4.0, 26.0]
                && q.uv[3] == [8.0, 26.0]
        });
        // the mirrored front face: the u axis flipped
        let mirrored_front = leg_quads.iter().any(|q| {
            q.uv[0] == [4.0, 20.0]
                && q.uv[1] == [8.0, 20.0]
                && q.uv[2] == [8.0, 26.0]
                && q.uv[3] == [4.0, 26.0]
        });
        assert!(plain_front, "the right legs' own unwrap rides");
        assert!(mirrored_front, "the left legs' mirror flips the u axis");
    }

    /// THE PIG LOADS — the real runtime cache (the generator's
    /// extraction) decodes the 64x64 temperate skin the sha1 the
    /// generated table pins (the PigRenderer's variant default).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_pig_model_loads_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let pig = PigModel::load(&root).expect("the pig loads from the runtime cache");
        // the temperate pig's pink: the head's front face (the unwrap
        // rect (8,8)-(16,16) the 8x8x8 head speaks) reads pink —
        // red over blue, the opaque texel the palette carries
        let px = pig.pixel(12.0, 12.0);
        assert!(px[0] > px[2], "the pink reads red-over-blue: {px:?}");
        assert_eq!(px[3], 255, "the texel is opaque");
    }

    /// THE BOVINE'S OWN EXTENTS (javap CowModel.createBaseCowModel's
    /// own numbers): the mesh spans the ground (model y 24 — the legs'
    /// 12-texel reach rides the same 24px plane the pig speaks) to the
    /// horns' crown (model y -1 — the 1-texel poke above the head's
    /// own top the y -5 box origin speaks).
    #[test]
    fn the_cow_mesh_spans_the_bovine_height() {
        let cow = CowModel {
            rgba: vec![255u8; 64 * 64 * 4],
        };
        let feet = Vec3::new(10.0, 64.0, 20.0);
        let state = QuadrupedState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let quads = cow.quads(&state, feet, 0.0, 0.0);
        assert!(!quads.is_empty(), "the mesh speaks quads");
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        for q in &quads {
            for p in q.pts {
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
                min_x = min_x.min(p[0]);
                max_x = max_x.max(p[0]);
            }
        }
        // model y 24 (the legs' ground) -> feet + 0.001 (the fold's own
        // epsilon); model y -1 (the horns' crown) -> feet + 1.5635
        assert!(
            (min_y - (feet.y as f32 + 0.001)).abs() < 1e-4,
            "the feet ride the ground: {min_y}"
        );
        assert!(
            (max_y - (feet.y as f32 + 1.5635)).abs() < 1e-4,
            "the horns' crown rides 1.5625 above the feet: {max_y}"
        );
        // the body's barrel spans -6/16..6/16 (12 texels wide), the
        // legs' pivots +-4/16 carry the same +-6/16 reach — the widest
        // word is the barrel's own
        assert!(
            (min_x - (feet.x as f32 - 6.0 / 16.0)).abs() < 1e-4,
            "the body's west face: {min_x}"
        );
        assert!(
            (max_x - (feet.x as f32 + 6.0 / 16.0)).abs() < 1e-4,
            "the body's east face: {max_x}"
        );
    }

    /// THE BOVINE WALK (the same javap QuadrupedModel.setupAnim fold —
    /// the 26.2 CowModel carries NO setupAnim override): the DIAGONAL
    /// pairs swing together — the longer 12-texel legs' backward kick
    /// reaches past the rest stand the same law the pig speaks.
    #[test]
    fn the_cow_walk_swings_the_diagonal_pairs() {
        let cow = CowModel {
            rgba: vec![255u8; 64 * 64 * 4],
        };
        let feet = Vec3::new(0.0, 64.0, 0.0);
        let rest = QuadrupedState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let walked = QuadrupedState {
            walk_pos: 0.0,
            walk_speed: 1.0,
            ..rest
        };
        let rest_quads = cow.quads(&rest, feet, 0.0, 0.0);
        let walked_quads = cow.quads(&walked, feet, 0.0, 0.0);
        assert_eq!(rest_quads.len(), walked_quads.len(), "same mesh, moved legs");
        // the swing's backward kick (the 12-texel legs' own reach)
        // passes the rest stand — the hind legs' rest back edge the
        // yaw fold's -z holds
        let rest_z: Vec<f32> = rest_quads.iter().flat_map(|q| q.pts.iter().map(|p| p[2])).collect();
        let rest_min_z = rest_z.iter().cloned().fold(f32::MAX, f32::min);
        let walk_z: Vec<f32> = walked_quads
            .iter()
            .flat_map(|q| q.pts.iter().map(|p| p[2]))
            .collect();
        let walk_min_z = walk_z.iter().cloned().fold(f32::MAX, f32::min);
        assert!(
            walk_min_z < rest_min_z - 0.05,
            "the swing reaches past the stand: rest {rest_min_z} walked {walk_min_z}"
        );
        // and the legs MOVED — some quad corner left its rest spot
        let moved = walked_quads
            .iter()
            .zip(rest_quads.iter())
            .any(|(wq, rq)| wq.pts.iter().zip(rq.pts.iter()).any(|(wp, rp)| (wp[2] - rp[2]).abs() > 1e-4));
        assert!(moved, "the walk's own swing moved the legs");
    }

    /// THE MIRROR UNWRAP (the bytecode's own astore/aload pairing —
    /// the LEFT legs' builders speak the mirror()): the mirrored
    /// cube's front face samples the leg rect reversed — the u corners
    /// swap, the v corners stand.
    #[test]
    fn the_cow_left_legs_ride_the_mirror_unwrap() {
        let cow = CowModel {
            rgba: vec![255u8; 64 * 64 * 4],
        };
        let state = QuadrupedState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let quads = cow.quads(&state, Vec3::new(0.0, 64.0, 0.0), 0.0, 0.0);
        // the legs' unwrap region: texOffs (0,16), box 4x12x4 — every
        // face rect inside u [0,16), v [16,32)
        let leg_quads: Vec<&AvatarQuad> = quads
            .iter()
            .filter(|q| {
                q.uv.iter().all(|c| c[0] >= 0.0 && c[0] <= 16.0 && c[1] >= 16.0 && c[1] <= 32.0)
            })
            .collect();
        assert_eq!(leg_quads.len(), 24, "4 legs x 6 faces");
        // the unmirrored front face (k=0 -> (u1,v0)): the 12-texel
        // leg's front rect (4,20)-(8,32) — [[8,20],[4,20],[4,32],[8,32]]
        let plain_front = leg_quads.iter().any(|q| {
            q.uv[0] == [8.0, 20.0]
                && q.uv[1] == [4.0, 20.0]
                && q.uv[2] == [4.0, 32.0]
                && q.uv[3] == [8.0, 32.0]
        });
        // the mirrored front face: the u axis flipped
        let mirrored_front = leg_quads.iter().any(|q| {
            q.uv[0] == [4.0, 20.0]
                && q.uv[1] == [8.0, 20.0]
                && q.uv[2] == [8.0, 32.0]
                && q.uv[3] == [4.0, 32.0]
        });
        assert!(plain_front, "the right legs' own unwrap rides");
        assert!(mirrored_front, "the left legs' mirror flips the u axis");
    }

    /// THE AVIAN'S OWN EXTENTS (javap AdultChickenModel's own numbers):
    /// the mesh spans the ground (model y 24 — the 5-texel legs ride
    /// the same 24px plane every quadruped speaks) to the head's crown
    /// (model y 9 — the 6-texel head above its y-15 pivot; the drawn
    /// bird the 0.9375 the 15/16 world span keeps, taller than the
    /// 0.7 hitbox the EntityTypes speaks — the model the model's own
    /// truth draws, the bbox the registration's own).
    #[test]
    fn the_chicken_mesh_spans_the_avian_height() {
        let chicken = ChickenModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let feet = Vec3::new(10.0, 64.0, 20.0);
        let state = ChickenState {
            yaw_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            flap: 0.0,
            flap_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let quads = chicken.quads(&state, feet, 0.0, 0.0);
        assert!(!quads.is_empty(), "the mesh speaks quads");
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        for q in &quads {
            for p in q.pts {
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
                min_x = min_x.min(p[0]);
                max_x = max_x.max(p[0]);
            }
        }
        // model y 24 (the legs' ground) -> feet + 0.001 (the fold's own
        // epsilon); model y 9 (the head's crown) -> feet + 0.9385
        assert!(
            (min_y - (feet.y as f32 + 0.001)).abs() < 1e-4,
            "the feet ride the ground: {min_y}"
        );
        assert!(
            (max_y - (feet.y as f32 + 0.9385)).abs() < 1e-4,
            "the head's crown rides 0.9375 above the feet: {max_y}"
        );
        // the wings' rest reach: the pivots +-4/16 the widest word
        // speaks (the body's barrel only +-3/16)
        assert!(
            (min_x - (feet.x as f32 - 4.0 / 16.0)).abs() < 1e-4,
            "the right wing's rest reach: {min_x}"
        );
        assert!(
            (max_x - (feet.x as f32 + 4.0 / 16.0)).abs() < 1e-4,
            "the left wing's rest reach: {max_x}"
        );
    }

    /// THE AVIAN FLAP (the ChickenModel.setupAnim's own law the 26.2
    /// bytecode keeps: `flapAngle = (sin(flap)+1) * flapSpeed`, the
    /// right wing the +zRot the left the NEGATED — the beat the
    /// mirrored pair swings, the rest the speed zero never lifts).
    #[test]
    fn the_chicken_flap_swing_mirrors_the_wings() {
        let chicken = ChickenModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let feet = Vec3::new(0.0, 64.0, 0.0);
        let rest = ChickenState {
            yaw_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            flap: 0.0,
            flap_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        // flap = PI/2 (sin = 1), speed 1 -> the flapAngle 2.0 rad the
        // wings' zRot folds
        let flapping = ChickenState {
            flap: std::f32::consts::FRAC_PI_2,
            flap_speed: 1.0,
            ..rest
        };
        let rest_quads = chicken.quads(&rest, feet, 0.0, 0.0);
        let flap_quads = chicken.quads(&flapping, feet, 0.0, 0.0);
        assert_eq!(rest_quads.len(), flap_quads.len(), "same mesh, moved wings");
        let rest_x: f32 = rest_quads
            .iter()
            .flat_map(|q| q.pts.iter().map(|p| p[0]))
            .fold(f32::MIN, f32::max);
        let flap_x: f32 = flap_quads
            .iter()
            .flat_map(|q| q.pts.iter().map(|p| p[0]))
            .fold(f32::MIN, f32::max);
        // the left wing's tip the 2-rad lift swings PAST the rest
        // reach (the +x wing the -zRot rotates, the y extent the
        // sin(2) fold carries into x)
        assert!(
            flap_x > rest_x + 0.1,
            "the flap swing lifts the wing tip: {flap_x} vs {rest_x}"
        );
    }

    /// THE CHICK'S OWN EXTENTS (javap BabyChickenModel's own numbers):
    /// the head-less chick the 16x16 layout draws — the two-cube body
    /// (model y 18..22) over the flat-plate legs (22..24), the whole
    /// silhouette the 6-texel span keeps.
    #[test]
    fn the_baby_chicken_mesh_spans_the_chick_height() {
        let baby = BabyChickenModel {
            rgba: vec![255u8; 16 * 16 * 4],
        };
        let feet = Vec3::new(0.0, 64.0, 0.0);
        let state = ChickenState {
            yaw_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            flap: 0.0,
            flap_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let quads = baby.quads(&state, feet, 0.0, 0.0);
        assert!(!quads.is_empty(), "the mesh speaks quads");
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        for q in &quads {
            for p in q.pts {
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
            }
        }
        // model y 24 (the legs' ground) -> feet + 0.001; model y 18
        // (the body's crown) -> feet + 6/16 + 0.001 = 0.376
        assert!(
            (min_y - (feet.y as f32 + 0.001)).abs() < 1e-4,
            "the chick's feet ride the ground: {min_y}"
        );
        assert!(
            (max_y - (feet.y as f32 + 0.376)).abs() < 1e-4,
            "the chick's crown rides 0.375 above the feet: {max_y}"
        );
    }

    /// THE CHICKEN LOADS — the real runtime cache (the generator's
    /// extraction) decodes the 64x32 temperate skin over the 16x16
    /// baby skin the sha1s the generated table pins (the
    /// ChickenRenderer's AdultAndBabyModelPair the isBaby word picks).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_chicken_model_loads_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let chicken = ChickenModel::load(&root).expect("the chicken loads from the runtime cache");
        // the temperate hen's white: the head's front face (the unwrap
        // rect (2,2)-(6,5) the 4x6x3 head speaks) reads the pale rows
        // the opaque texel keeps
        let px = chicken.pixel(3.0, 3.0);
        assert_eq!(px[3], 255, "the texel is opaque");
        let baby =
            BabyChickenModel::load(&root).expect("the baby chicken loads from the runtime cache");
        let bpx = baby.pixel(8.0, 8.0);
        assert_eq!(bpx[3], 255, "the baby texel is opaque");
    }

    /// THE CHICKEN VARIANT MODELS LOAD — the real runtime cache (the
    /// generator's extraction) decodes the cold/warm pairs the sha1s
    /// the generated table pins (cold.json/warm.json's asset_id pair
    /// the ChickenRenderer's model pick wears); the cold model's own
    /// replace walk the quad count speaks (the tail plate + the fluff
    /// plate the temperate mesh never carries)
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_chicken_variant_models_load_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let cold = ChickenColdModel::load(&root).expect("the cold chicken loads");
        let warm = ChickenModel::load_warm(&root).expect("the warm chicken loads");
        let cold_baby =
            BabyChickenModel::load_cold(&root).expect("the cold baby chicken loads");
        let warm_baby =
            BabyChickenModel::load_warm(&root).expect("the warm baby chicken loads");
        // THE PIG/COW VARIANTS' OWN LOADS (the same sha1 walk the
        // chicken's pairs keep — the cold/warm skins + the babies)
        let cold_pig = ColdPigModel::load(&root).expect("the cold pig loads");
        let _warm_pig = PigModel::load_warm(&root).expect("the warm pig loads");
        let _cold_pig_baby = BabyPigModel::load_cold(&root).expect("the cold piglet loads");
        let _warm_pig_baby = BabyPigModel::load_warm(&root).expect("the warm piglet loads");
        let cold_cow = ColdCowModel::load(&root).expect("the cold cow loads");
        let warm_cow = WarmCowModel::load(&root).expect("the warm cow loads");
        let _cold_cow_baby = BabyCowModel::load_cold(&root).expect("the cold calf loads");
        let _warm_cow_baby = BabyCowModel::load_warm(&root).expect("the warm calf loads");
        // the pig/cow loads' own sha1 verdicts ARE the content pins
        // (the 64x64 layouts the decode verifies — the per-texel
        // probes the pig/cow unwrap rects the existing loads tests
        // each name); the cold pig's hide coat the quad walk reads
        // below the mesh difference speaks
        // the variant texels: the loads' own sha1 verdicts the pixel
        // reads re-speak (the opaque words the layouts keep)
        assert_eq!(cold.pixel(3.0, 3.0)[3], 255, "the cold texel is opaque");
        assert_eq!(warm.pixel(3.0, 3.0)[3], 255, "the warm texel is opaque");
        assert_eq!(cold_baby.pixel(8.0, 8.0)[3], 255, "the cold baby texel is opaque");
        assert_eq!(warm_baby.pixel(8.0, 8.0)[3], 255, "the warm baby texel is opaque");
        // THE COLD MESH'S OWN EXTRAS (the replace walk the builder's
        // cold arm keeps — the tail plate + the face plate the
        // temperate's cube lists never carry): the quad count the
        // extra boxes speak
        let feet = Vec3::new(0.0, 64.0, 0.0);
        let state = ChickenState {
            yaw_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            flap: 0.0,
            flap_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let temperate = ChickenModel::load(&root).expect("the temperate loads");
        let base = temperate.quads(&state, feet, 0.0, 0.0);
        let cold_quads = cold.quads(&state, feet, 0.0, 0.0);
        // THE REPLACE WALK'S OWN BUDGET (the beak + the red_thing pair
        // the fluff + the tail pair replaces — the same cube-face
        // count, the DIFFERENT mesh): the vertex sets disagree, the
        // cold plates' geometry the temperate never draws
        assert_eq!(
            cold_quads.len(),
            base.len(),
            "the face budget the replace walk keeps"
        );
        let verts = |quads: &[AvatarQuad]| {
            let mut v: Vec<[i32; 3]> = quads
                .iter()
                .flat_map(|q| {
                    q.pts.iter().map(|p| {
                        [
                            (p[0] * 16.0).round() as i32,
                            (p[1] * 16.0).round() as i32,
                            (p[2] * 16.0).round() as i32,
                        ]
                    })
                })
                .collect();
            v.sort_unstable();
            v.dedup();
            v
        };
        assert_ne!(verts(&cold_quads), verts(&base), "the cold mesh's plates the temperate never draws");
        // THE PIG/COW VARIANTS' OWN MESHES (the coat boxes the cold
        // pair keeps, the big horns the warm cow wears — the vertex
        // sets the temperate meshes never speak)
        let quad_state = QuadrupedState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let pig_state = PigState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let base_pig = PigModel::load(&root).expect("the temperate pig loads");
        assert_ne!(
            verts(&cold_pig.quads(&pig_state, feet, 0.0, 0.0)),
            verts(&base_pig.quads(&pig_state, feet, 0.0, 0.0)),
            "the cold pig's coat the temperate never draws"
        );
        let base_cow = CowModel::load(&root).expect("the temperate cow loads");
        assert_ne!(
            verts(&cold_cow.quads(&quad_state, feet, 0.0, 0.0)),
            verts(&base_cow.quads(&quad_state, feet, 0.0, 0.0)),
            "the cold cow's coat the temperate never draws"
        );
        assert_ne!(
            verts(&warm_cow.quads(&quad_state, feet, 0.0, 0.0)),
            verts(&base_cow.quads(&quad_state, feet, 0.0, 0.0)),
            "the warm cow's big horns the temperate never draws"
        );
    }

    /// THE COW LOADS — the real runtime cache (the generator's
    /// extraction) decodes the 64x64 temperate skin the sha1 the
    /// generated table pins (the CowRenderer's variant default).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_cow_model_loads_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let cow = CowModel::load(&root).expect("the cow loads from the runtime cache");
        // the temperate cow's brown: the head's front face (the unwrap
        // rect (6,6)-(14,14) the 8x8x6 head speaks) reads brown —
        // red over blue, the opaque texel the palette carries
        let px = cow.pixel(10.0, 10.0);
        assert!(px[0] > px[2], "the brown reads red-over-blue: {px:?}");
        assert_eq!(px[3], 255, "the texel is opaque");
    }

    /// THE ARACHNID'S OWN EXTENTS (javap SpiderModel.createSpiderBodyLayer's
    /// own numbers): 11 parts — the head, the neck cube, the abdomen,
    /// the EIGHT legs — each one cube, 66 quads the bake speaks; the
    /// rest crown rides the head's top (model y 11 — the 8x8x8 head at
    /// pivot y 15 the box's +4 reach speaks — world 0.8125 above the
    /// feet), the walk clock swings the legs (the phase law the
    /// setupAnim fold keeps), and the head's look turns the fangs.
    #[test]
    fn the_spider_mesh_bakes_the_eight_legs() {
        let spider = SpiderModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let feet = Vec3::new(10.0, 64.0, 20.0);
        let rest = SpiderState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            death_time: 0.0,
        };
        let quads = spider.quads(&rest, feet, 0.0, 0.0);
        assert_eq!(quads.len(), 66, "11 parts x 6 faces the bake speaks");
        let mut max_y = f32::MIN;
        let mut min_y = f32::MAX;
        for q in &quads {
            for p in q.pts {
                max_y = max_y.max(p[1]);
                min_y = min_y.min(p[1]);
            }
        }
        // the head's crown: model y 11 -> world 13/16 above the feet
        // (the zero-pose head the look never lifts — the entity
        // fold's own 0.001 epsilon the top surface rides)
        assert!(
            (max_y - (feet.y as f32 + 0.8135)).abs() < 1e-4,
            "the crown rides the head box's +4 reach: {max_y}"
        );
        // the legs' tips dip well below the bodies' underside (model
        // y 19 — world 0.3125): the diagonal stances the zRot folds
        // speak reach the ground's neighborhood
        assert!(
            min_y < feet.y as f32 + 0.25 && min_y > feet.y as f32 - 0.25,
            "the legs' tips the ground's neighborhood reach: {min_y}"
        );
        // THE WALK CLOCK: the speed-1 stride the legs swing (the rest
        // pose the speed zero keeps — the pose fold the sums differ)
        let pose_sum = |qs: &[AvatarQuad]| -> f64 {
            qs.iter()
                .flat_map(|q| q.pts.iter().flatten())
                .map(|v| *v as f64)
                .sum()
        };
        let walking = SpiderState { walk_pos: 1.0, walk_speed: 1.0, ..rest };
        let walk_quads = spider.quads(&walking, feet, 0.0, 0.0);
        assert_ne!(pose_sum(&quads), pose_sum(&walk_quads), "the walk clock the legs swing");
        // THE HEAD'S LOOK: the yaw offset turns the fangs' cube
        let looking = SpiderState { head_yaw_off_deg: 30.0, ..rest };
        let look_quads = spider.quads(&looking, feet, 0.0, 0.0);
        assert_ne!(pose_sum(&quads), pose_sum(&look_quads), "the head's look the fangs turn");
        // THE DEATH FOLD: the topple the death clock speaks
        let dying = SpiderState { death_time: 5.0, ..rest };
        let die_quads = spider.quads(&dying, feet, 0.0, 0.0);
        assert_ne!(pose_sum(&quads), pose_sum(&die_quads), "the death fold the body topples");
    }

    /// THE SPIDER LOADS — the real runtime cache (the generator's
    /// extraction) decodes the 64x32 skin the sha1 the generated table
    /// pins (the SpiderRenderer's SPIDER_LOCATION).
    #[test]
    fn the_spider_model_loads_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let spider = SpiderModel::load(&root).expect("the spider loads from the runtime cache");
        // the arachnid's dark hide: the census the opaque rows keep
        // (the body texels the 64x32 canvas paints)
        let mut opaque = 0;
        for y in 0..32 {
            for x in 0..64 {
                if spider.pixel(x as f32, y as f32)[3] == 255 {
                    opaque += 1;
                }
            }
        }
        assert!(opaque > 500, "the hide's texels the canvas keeps: {opaque}");
    }

    /// THE CREEPER MESH BAKES THE FOUR LEGS (javap CreeperModel's own
    /// numbers): the 6 parts x 6 faces the bake speaks, the head's
    /// crown at model y -2 (world 1.625 — the head box's -8 reach the
    /// PartPose.offset(0, 6, 0) pivot wears), the legs' tips at model
    /// 24 (the ground's own plane the 24px law keeps), the walk clock
    /// the diagonal phases swing, the head's look the fangs' cube
    /// turns, the death fold the body topples.
    #[test]
    fn the_creeper_mesh_bakes_the_four_legs() {
        let creeper = CreeperModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let feet = Vec3::new(10.0, 64.0, 20.0);
        let rest = CreeperState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            death_time: 0.0,
            swelling: 0.0,
        };
        let quads = creeper.quads(&rest, feet, 0.0, 0.0);
        assert_eq!(quads.len(), 36, "6 parts x 6 faces the bake speaks");
        let mut max_y = f32::MIN;
        let mut min_y = f32::MAX;
        for q in &quads {
            for p in q.pts {
                max_y = max_y.max(p[1]);
                min_y = min_y.min(p[1]);
            }
        }
        // the head's crown: model y -2 -> world (24 - (-2))/16 = 1.625
        // (+ the entity fold's own 0.001 epsilon the top surface rides)
        assert!(
            (max_y - (feet.y as f32 + 1.626)).abs() < 1e-4,
            "the crown rides the head box's -8 reach: {max_y}"
        );
        // the legs' tips at model y 24 — the ground's own plane (the
        // zero walk the legs keep straight, the fold's 0.001 epsilon
        // the bottom surface rides)
        assert!(
            (min_y - (feet.y as f32 + 0.001)).abs() < 1e-4,
            "the legs' tips the ground's plane keeps: {min_y}"
        );
        // THE WALK CLOCK: the speed-1 stride the legs swing (the rest
        // pose the speed zero keeps — the diagonal phases the sums
        // differ)
        let pose_sum = |qs: &[AvatarQuad]| -> f64 {
            qs.iter()
                .flat_map(|q| q.pts.iter().flatten())
                .map(|v| *v as f64)
                .sum()
        };
        let walking = CreeperState { walk_pos: 1.0, walk_speed: 1.0, ..rest };
        let walk_quads = creeper.quads(&walking, feet, 0.0, 0.0);
        assert_ne!(pose_sum(&quads), pose_sum(&walk_quads), "the walk clock the legs swing");
        // THE HEAD'S LOOK: the yaw offset turns the crown's cube — the
        // symmetric 8x8 prism the center-axis rotation the SUM law
        // cancels (the corner radii the same squares keep), the X
        // EXTENT the 30-degree turn widens (the max |x| the 4 -> 5.464
        // the rotated corner reaches)
        let looking = CreeperState { head_yaw_off_deg: 30.0, ..rest };
        let look_quads = creeper.quads(&looking, feet, 0.0, 0.0);
        let x_extent = |qs: &[AvatarQuad]| -> f32 {
            let (mut lo, mut hi) = (f32::MAX, f32::MIN);
            for q in qs {
                for p in q.pts {
                    lo = lo.min(p[0]);
                    hi = hi.max(p[0]);
                }
            }
            hi - lo
        };
        assert!(
            x_extent(&look_quads) > x_extent(&quads) + 0.1,
            "the head's look the crown turns: {} vs {}",
            x_extent(&quads),
            x_extent(&look_quads)
        );
        // THE DEATH FOLD: the topple the death clock speaks
        let dying = CreeperState { death_time: 5.0, ..rest };
        let die_quads = creeper.quads(&dying, feet, 0.0, 0.0);
        assert_ne!(pose_sum(&quads), pose_sum(&die_quads), "the death fold the body topples");
    }

    /// THE CREEPER'S OWN SWELL FOLD (javap `CreeperRenderer.scale` +
    /// `getWhiteOverlayProgress` the 26.2 jar speaks — the f⁴ law the
    /// raw bytecode's TWO squarings keeps, the g wobble the sin * 100
    /// fold answers, the flash the odd decade's clamp speaks, the quad
    /// transform the feet anchor the X/Y pair scales).
    #[test]
    fn the_creeper_renderer_wears_the_swell_fold() {
        // THE SCALE (the zero the plain body's 1.0 keeps, the f⁴ the
        // 26.2's two squarings speak — sw 0.5: f = 0.5⁴ = 0.0625, the
        // x the (1.025) * g wears, the y the (1.00625) / g rides; the
        // f² law the OLDER walks kept would read 0.25 — the g the
        // same word keeps)
        let (x0, y0) = creeper_swelling_scale(0.0);
        assert_eq!(x0, 1.0, "the plain body the 1.0 keeps");
        assert_eq!(y0, 1.0, "the plain body the 1.0 keeps");
        let sw = 0.5f32;
        let g = 1.0 + ((sw * 100.0) as f64).sin() as f32 * sw * 0.009_999_999_776_482_582;
        let f4 = 0.5f32 * 0.5 * 0.5 * 0.5;
        let (x1, y1) = creeper_swelling_scale(sw);
        assert_eq!(x1, (1.0 + f4 * 0.400_000_005_960_464_5) * g, "the f⁴ x the raw consts keep");
        assert_eq!(y1, (1.0 + f4 * 0.100_000_001_490_116_12) / g, "the f⁴ y the raw consts keep");
        // the f⁴ DISTINCTION (the f² law the older versions wore would
        // read 0.25 — the x the 0.1 apart the two laws speak)
        let f2 = 0.25f32;
        let x_old = (1.0 + f2 * 0.400_000_005_960_464_5) * g;
        assert!((x1 - x_old).abs() > 0.05, "the f⁴ the older f² retires: {x1} vs {x_old}");
        // THE FULL FUSE (sw 1.0: f = 1.0, the x the (1.0 + 0.4f32) * g
        // the swell's own peak wears, the y the (1.0 + 0.1f32) / g the
        // tall body's own word keeps — the g the sw's own sin rides,
        // the f32 sums the literal 1.4 never equals, the same ops the
        // java fold speaks)
        let g1 = 1.0 + ((1.0f32 * 100.0) as f64).sin() as f32 * 1.0 * 0.009_999_999_776_482_582;
        let (x2, y2) = creeper_swelling_scale(1.0);
        assert_eq!(x2, (1.0 + 1.0f32 * 0.400_000_005_960_464_5) * g1, "the peak the swell wears");
        assert_eq!(y2, (1.0 + 1.0f32 * 0.100_000_001_490_116_12) / g1, "the peak the swell wears");
        // THE FLASH (the (int)(f * 10) % 2 law the raw bytecode keeps —
        // the even decade the silence wears, the odd the clamp's own
        // band answers, the peak's odd decade the full progress keeps)
        assert_eq!(creeper_white_overlay(0.0), 0.0, "the rest the silence keeps");
        assert_eq!(creeper_white_overlay(0.05), 0.0, "the 0-decade the silence keeps");
        assert_eq!(creeper_white_overlay(0.1), 0.5, "the 1st decade the 0.5 floor wears");
        assert_eq!(creeper_white_overlay(0.95), 0.95, "the 9th decade the full keeps");
        // THE QUAD TRANSFORM (the feet anchor the X/Y pair scales —
        // the crown the Y factor lifts, the ground's plane the anchor
        // keeps)
        let feet = Vec3::new(10.0, 64.0, 20.0);
        let mut quads = vec![AvatarQuad {
            pts: [
                [feet.x as f32, feet.y as f32, feet.z as f32],
                [feet.x as f32 + 1.0, feet.y as f32, feet.z as f32],
                [feet.x as f32, feet.y as f32 + 2.0, feet.z as f32],
                [feet.x as f32 + 1.0, feet.y as f32 + 2.0, feet.z as f32],
            ],
            uv: [[0.0; 2]; 4],
            shade: 1.0,
        }];
        let (sx, sy) = creeper_swelling_scale(1.0);
        apply_creeper_swelling(&mut quads, feet, 1.0);
        let q = &quads[0];
        assert!((q.pts[0][0] - feet.x as f32).abs() < 1e-5, "the anchor keeps");
        assert!((q.pts[1][0] - (feet.x as f32 + sx)).abs() < 1e-5, "the +1 x the sx scales");
        assert!((q.pts[2][1] - (feet.y as f32 + 2.0 * sy)).abs() < 1e-5, "the +2 y the sy scales");
        assert!((q.pts[3][2] - feet.z as f32).abs() < 1e-5, "the flat z the anchor keeps");
    }

    /// THE CREEPER LOADS — the real runtime cache (the generator's
    /// extraction) decodes the 64x32 skin the sha1 the generated table
    /// pins (the CreeperRenderer's CREEPER_LOCATION).
    #[test]
    fn the_creeper_model_loads_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let creeper = CreeperModel::load(&root).expect("the creeper loads from the runtime cache");
        // the charged-green hide: the census the opaque rows keep
        // (the body texels the 64x32 canvas paints)
        let mut opaque = 0;
        for y in 0..32 {
            for x in 0..64 {
                if creeper.pixel(x as f32, y as f32)[3] == 255 {
                    opaque += 1;
                }
            }
        }
        assert!(opaque > 500, "the hide's texels the canvas keeps: {opaque}");
    }

    /// THE OVINE'S OWN EXTENTS (javap SheepModel.createBodyLayer's own
    /// numbers): the mesh spans the ground (model y 24 — the legs'
    /// 12-texel reach rides the same 24px plane the pig/cow speak) to
    /// the head's top (model y 2 — the 6x6x8 head at pivot y 6 the
    /// ovine's own replacement speaks), and the LEGS carry the widest
    /// x (the ±5/16 the pivots +-3 + the 4-texel box speak — the
    /// 8-texel barrel's ±4/16 stays inside).
    #[test]
    fn the_sheep_mesh_spans_the_ovine_height() {
        let sheep = SheepModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let feet = Vec3::new(10.0, 64.0, 20.0);
        let state = SheepState {
            yaw_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
            head_eat_position: 0.0,
            head_eat_angle: 0.0,
            age_scale: 1.0,
        };
        let quads = sheep.quads(&state, feet, 0.0, 0.0);
        assert!(!quads.is_empty(), "the mesh speaks quads");
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        for q in &quads {
            for p in q.pts {
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
                min_x = min_x.min(p[0]);
                max_x = max_x.max(p[0]);
            }
        }
        // model y 24 (the legs' ground) -> feet + 0.001 (the fold's own
        // epsilon); model y 2 (the head's top) -> feet + 1.376
        assert!(
            (min_y - (feet.y as f32 + 0.001)).abs() < 1e-4,
            "the feet ride the ground: {min_y}"
        );
        assert!(
            (max_y - (feet.y as f32 + 1.376)).abs() < 1e-4,
            "the head's top rides 1.375 above the feet: {max_y}"
        );
        // the legs' own ±5/16 (pivots +-3 + the 4-texel box) — the
        // 8-texel barrel's ±4/16 the narrower word stays inside
        assert!(
            (min_x - (feet.x as f32 - 5.0 / 16.0)).abs() < 1e-4,
            "the legs' west face: {min_x}"
        );
        assert!(
            (max_x - (feet.x as f32 + 5.0 / 16.0)).abs() < 1e-4,
            "the legs' east face: {max_x}"
        );
    }

    /// THE OVINE WALK (the same javap QuadrupedModel.setupAnim fold —
    /// the sheep's own override only re-poses the HEAD): the DIAGONAL
    /// pairs swing together — the 12-texel legs' backward kick reaches
    /// past the rest stand the same law the pig/cow speak.
    #[test]
    fn the_sheep_walk_swings_the_diagonal_pairs() {
        let sheep = SheepModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let feet = Vec3::new(0.0, 64.0, 0.0);
        let rest = SheepState {
            yaw_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
            head_eat_position: 0.0,
            head_eat_angle: 0.0,
            age_scale: 1.0,
        };
        let walked = SheepState {
            walk_pos: 0.0,
            walk_speed: 1.0,
            ..rest
        };
        let rest_quads = sheep.quads(&rest, feet, 0.0, 0.0);
        let walked_quads = sheep.quads(&walked, feet, 0.0, 0.0);
        assert_eq!(rest_quads.len(), walked_quads.len(), "same mesh, moved legs");
        // the swing's backward kick (the 12-texel legs' own reach)
        // passes the rest stand — the hind legs' rest back edge the
        // yaw fold's -z holds
        let rest_z: Vec<f32> = rest_quads.iter().flat_map(|q| q.pts.iter().map(|p| p[2])).collect();
        let rest_min_z = rest_z.iter().cloned().fold(f32::MAX, f32::min);
        let walk_z: Vec<f32> = walked_quads
            .iter()
            .flat_map(|q| q.pts.iter().map(|p| p[2]))
            .collect();
        let walk_min_z = walk_z.iter().cloned().fold(f32::MAX, f32::min);
        assert!(
            walk_min_z < rest_min_z - 0.05,
            "the swing reaches past the stand: rest {rest_min_z} walked {walk_min_z}"
        );
        // and the legs MOVED — some quad corner left its rest spot
        let moved = walked_quads
            .iter()
            .zip(rest_quads.iter())
            .any(|(wq, rq)| wq.pts.iter().zip(rq.pts.iter()).any(|(wp, rp)| (wp[2] - rp[2]).abs() > 1e-4));
        assert!(moved, "the walk's own swing moved the legs");
    }

    /// THE MIRROR UNWRAP (the bytecode's own createLegs(loc1=false,
    /// loc2=true) walk — the RIGHT legs' builders speak the mirror(),
    /// the pig/cow LEFT-pair convention INVERTED): the mirrored cube's
    /// front face samples the leg rect reversed — the u corners swap,
    /// the v corners stand.
    #[test]
    fn the_sheep_right_legs_ride_the_mirror_unwrap() {
        let sheep = SheepModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let state = SheepState {
            yaw_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
            head_eat_position: 0.0,
            head_eat_angle: 0.0,
            age_scale: 1.0,
        };
        let quads = sheep.quads(&state, Vec3::new(0.0, 64.0, 0.0), 0.0, 0.0);
        // the legs' unwrap region: texOffs (0,16), box 4x12x4 — every
        // face rect inside u [0,16), v [16,32)
        let leg_quads: Vec<&AvatarQuad> = quads
            .iter()
            .filter(|q| {
                q.uv.iter().all(|c| c[0] >= 0.0 && c[0] <= 16.0 && c[1] >= 16.0 && c[1] <= 32.0)
            })
            .collect();
        assert_eq!(leg_quads.len(), 24, "4 legs x 6 faces");
        // the unmirrored front face (k=0 -> (u1,v0)): the 12-texel
        // leg's front rect (4,20)-(8,32) — [[8,20],[4,20],[4,32],[8,32]]
        let plain_front = leg_quads.iter().any(|q| {
            q.uv[0] == [8.0, 20.0]
                && q.uv[1] == [4.0, 20.0]
                && q.uv[2] == [4.0, 32.0]
                && q.uv[3] == [8.0, 32.0]
        });
        // the mirrored front face: the u axis flipped
        let mirrored_front = leg_quads.iter().any(|q| {
            q.uv[0] == [4.0, 20.0]
                && q.uv[1] == [8.0, 20.0]
                && q.uv[2] == [8.0, 32.0]
                && q.uv[3] == [4.0, 32.0]
        });
        assert!(plain_front, "the left legs' own unwrap rides");
        assert!(mirrored_front, "the right legs' mirror flips the u axis");
    }

    /// THE EAT FOLD (javap SheepModel.setupAnim's own two statements —
    /// the override the cow never spoke): the head's pivot the
    /// `headEatPositionScale * 9.0 * ageScale` fold DROPS (model y 6
    /// -> 10.5 at the half-eat — the head's top face 1.376 -> 1.095
    /// above the feet), and the eat angle the head's xRot REPLACES the
    /// quadruped's pitch with pitches the head (some corner moves).
    #[test]
    fn the_sheep_eat_fold_drops_and_pitches_the_head() {
        let sheep = SheepModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let feet = Vec3::new(0.0, 64.0, 0.0);
        let rest = SheepState {
            yaw_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
            head_eat_position: 0.0,
            head_eat_angle: 0.0,
            age_scale: 1.0,
        };
        let max_y_of = |state: &SheepState| {
            sheep
                .quads(state, feet, 0.0, 0.0)
                .iter()
                .flat_map(|q| q.pts.iter().map(|p| p[1]))
                .fold(f32::MIN, f32::max)
        };
        // the rest stand's own top: the head's model y 2 -> 1.376
        let rest_max = max_y_of(&rest);
        assert!(
            (rest_max - (feet.y as f32 + 1.376)).abs() < 1e-4,
            "the rest top the head's own: {rest_max}"
        );
        // the quarter-eat: pivot 6 -> 6 + 0.25*9.0*1.0 = 8.25, the
        // head's top model y 2 -> 4.25 (still the mesh's top — the
        // 8-texel barrel's own top model y 6 stays below) -> feet +
        // 1.235375 (the fold's own epsilon the ground speaks)
        let eating = SheepState {
            head_eat_position: 0.25,
            ..rest
        };
        let eat_max = max_y_of(&eating);
        assert!(
            (eat_max - (feet.y as f32 + 1.235_375)).abs() < 1e-4,
            "the quarter-eat drops the head the *9.0 fold speaks: {eat_max}"
        );
        // the ageScale the fold rides: the half-adult's drop halves
        let half_grown = SheepState {
            head_eat_position: 0.25,
            age_scale: 0.5,
            ..rest
        };
        let half_max = max_y_of(&half_grown);
        assert!(
            half_max > eat_max && half_max < rest_max,
            "the ageScale halves the drop: {half_max} between {eat_max} and {rest_max}"
        );
        // the eat angle pitches the head (the xRot the assign replaces
        // the quadruped's pitch with) — some corner moves vs the rest
        let pitched = SheepState {
            head_eat_angle: 0.9,
            ..rest
        };
        let moved = sheep
            .quads(&pitched, feet, 0.0, 0.0)
            .iter()
            .zip(sheep.quads(&rest, feet, 0.0, 0.0).iter())
            .any(|(pq, rq)| pq.pts.iter().zip(rq.pts.iter()).any(|(pp, rp)| (pp[2] - rp[2]).abs() > 1e-4));
        assert!(moved, "the eat angle's own pitch moved the head");
    }

    /// THE SHEEP LOADS — the real runtime cache (the generator's
    /// extraction) decodes the 64x32 skin the sha1 the generated table
    /// pins (the SheepRenderer's getTextureLocation).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_sheep_model_loads_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let sheep = SheepModel::load(&root).expect("the sheep loads from the runtime cache");
        // the ovine's tan face: the head's front face (the unwrap rect
        // (8,8)-(14,14) the 6x6x8 head speaks) reads tan — red over
        // blue, the opaque texel the palette carries
        let px = sheep.pixel(11.0, 11.0);
        assert!(px[0] > px[2], "the tan reads red-over-blue: {px:?}");
        assert_eq!(px[3], 255, "the texel is opaque");
    }

    /// THE WOOL MESH (javap SheepFurModel.createFurLayer's own
    /// numbers): 6 parts x 6 faces (the hood + the barrel + the four
    /// wool boots), the 1.75-inflated barrel the
    /// WIDEST word (+-5.75/16 the legs' +-5/16 stays inside), the fur
    /// crown at model y 1.4 (the hood's inflate 0.6 rides ABOVE the
    /// body head's own top 2), and the walk the shared diagonal fold
    /// swings (the fur legs the body's own leg x_rots ride).
    #[test]
    fn the_sheep_wool_mesh_inflates_over_the_body() {
        let wool = SheepWoolModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let feet = Vec3::new(0.0, 64.0, 0.0);
        let rest = SheepState {
            yaw_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
            head_eat_position: 0.0,
            head_eat_angle: 0.0,
            age_scale: 1.0,
        };
        let quads = wool.quads(&rest, feet, 0.0, 0.0);
        assert_eq!(quads.len(), 36, "6 fur parts x 6 faces");
        let mut max_y = f32::MIN;
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        for q in &quads {
            for p in q.pts {
                max_y = max_y.max(p[1]);
                min_x = min_x.min(p[0]);
                max_x = max_x.max(p[0]);
            }
        }
        // the 1.75-inflated barrel: x -5.75..5.75 model -> +-5.75/16
        assert!(
            (min_x - (feet.x as f32 - 5.75 / 16.0)).abs() < 1e-4,
            "the wool barrel's west face: {min_x}"
        );
        assert!(
            (max_x - (feet.x as f32 + 5.75 / 16.0)).abs() < 1e-4,
            "the wool barrel's east face: {max_x}"
        );
        // the hood's crown: pivot 6 - 4 - 0.6 = model y 1.4 -> 1.4135
        assert!(
            (max_y - (feet.y as f32 + 1.4135)).abs() < 1e-4,
            "the fur crown rides above the body head: {max_y}"
        );
        // the walk the shared fold swings — some corner moves
        let walked = SheepState {
            walk_pos: 0.0,
            walk_speed: 1.0,
            ..rest
        };
        let moved = wool
            .quads(&walked, feet, 0.0, 0.0)
            .iter()
            .zip(quads.iter())
            .any(|(wq, rq)| wq.pts.iter().zip(rq.pts.iter()).any(|(wp, rp)| (wp[2] - rp[2]).abs() > 1e-4));
        assert!(moved, "the fur rides the body's own diagonal walk");
    }

    /// THE WOOL LOADS — the real runtime cache decodes the 64x32 wool
    /// the sha1 the generated table pins (the SheepWoolLayer's
    /// SHEEP_WOOL_LOCATION), the greyscale the dye tint multiplies.
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_sheep_wool_loads_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let wool = SheepWoolModel::load(&root).expect("the wool loads from the runtime cache");
        // the head fur's front face (the unwrap rect (6,6)-(12,12) the
        // 6x6x6 fur box speaks) reads GREYSCALE — the equal channels
        // the dye tint multiplies, the opaque texel
        let px = wool.pixel(9.0, 9.0);
        assert_eq!(px[0], px[1], "the greyscale's own channels: {px:?}");
        assert_eq!(px[1], px[2], "the greyscale's own channels: {px:?}");
        assert_eq!(px[3], 255, "the texel is opaque");
    }

    /// THE UNDERCOAT'S MESH (javap LayerDefinitions clinit loc 2824:
    /// `SHEEP_WOOL_UNDERCOAT -> SheepModel.createBodyLayer()` — the
    /// SAME body layer the base skin's mesh speaks, NOT the
    /// SheepFurModel.createFurLayer inflate the wool fur keeps): the
    /// undercoat's quads EXACTLY overlay the body's own quads —
    /// point-for-point at rest AND walked (the one pose law both
    /// rides) — while the FUR mesh the same state arms stays the wider
    /// inflated walk the wool test pins.
    #[test]
    fn the_sheep_undercoat_rides_the_body_mesh() {
        let undercoat = SheepWoolUndercoatModel {
            rgba: vec![200u8; 64 * 32 * 4],
        };
        let body = SheepModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let feet = Vec3::new(0.0, 64.0, 0.0);
        let rest = SheepState {
            yaw_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
            head_eat_position: 0.0,
            head_eat_angle: 0.0,
            age_scale: 1.0,
        };
        let uc = undercoat.quads(&rest, feet, 0.0, 0.0);
        let bq = body.quads(&rest, feet, 0.0, 0.0);
        assert_eq!(uc.len(), bq.len(), "the same 6 body parts x 6 faces");
        for (uq, rq) in uc.iter().zip(bq.iter()) {
            for (up, rp) in uq.pts.iter().zip(rq.pts.iter()) {
                assert!(
                    (up[0] - rp[0]).abs() < 1e-5
                        && (up[1] - rp[1]).abs() < 1e-5
                        && (up[2] - rp[2]).abs() < 1e-5,
                    "the undercoat's corner rides the body's own walk: {up:?} vs {rp:?}"
                );
            }
        }
        // the walked + eating state: the SAME identity (the eat fold
        // the head's pivot rides, the diagonal walk the legs swing —
        // both meshes the one setupAnim law re-poses)
        let moved = SheepState {
            walk_pos: 1.7,
            walk_speed: 0.9,
            head_eat_position: 0.8,
            head_eat_angle: 0.4,
            ..rest
        };
        let uc2 = undercoat.quads(&moved, feet, 0.0, 0.0);
        let bq2 = body.quads(&moved, feet, 0.0, 0.0);
        assert_eq!(uc2.len(), bq2.len());
        for (uq, rq) in uc2.iter().zip(bq2.iter()) {
            for (up, rp) in uq.pts.iter().zip(rq.pts.iter()) {
                assert!(
                    (up[0] - rp[0]).abs() < 1e-5
                        && (up[1] - rp[1]).abs() < 1e-5
                        && (up[2] - rp[2]).abs() < 1e-5,
                    "the undercoat's walked corner rides the body's own walk: {up:?} vs {rp:?}"
                );
            }
        }
        // the FUR the same state arms stays WIDER (the undercoat never
        // the fur's mesh — the 1.75-inflated barrel the wool test pins
        // rides past the body's own 8-texel barrel the undercoat hugs)
        let fur = SheepWoolModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let fur_quads = fur.quads(&rest, feet, 0.0, 0.0);
        let max_x = |quads: &[AvatarQuad]| {
            quads
                .iter()
                .flat_map(|q| q.pts.iter())
                .map(|p| p[0])
                .fold(f32::MIN, f32::max)
        };
        let (uc_max_x, fur_max_x) = (max_x(&uc), max_x(&fur_quads));
        assert!(
            fur_max_x > uc_max_x + 0.5 / 16.0,
            "the fur's inflate rides past the body the undercoat hugs: {fur_max_x} vs {uc_max_x}"
        );
    }

    /// THE UNDERCOAT LOADS — the real runtime cache decodes the 64x32
    /// undercoat the sha1 the generated table pins (the
    /// SheepWoolUndercoatLayer's SHEEP_WOOL_UNDERCOAT_LOCATION): the
    /// head's top band the grey undercoat paints, the FACE HOLE the
    /// undercoat keeps transparent (the sheared sheep's tan face the
    /// body lane draws beneath shows through the cutout).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_sheep_undercoat_loads_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let uc =
            SheepWoolUndercoatModel::load(&root).expect("the undercoat loads from the runtime cache");
        // the face hole: the head's front-face unwrap rect (8,8)-(14,14)
        // the 6x6x8 head speaks — the texel (11,11) the undercoat keeps
        // TRANSPARENT (alpha 0, the PIL ground truth probe walked it)
        let hole = uc.pixel(11.0, 11.0);
        assert_eq!(hole[3], 0, "the face hole stays open: {hole:?}");
        // the head's top band: the texel (8,0) the undercoat paints —
        // the grey the dye tint multiplies, the opaque texel
        let top = uc.pixel(8.0, 0.0);
        assert_eq!(top[0], top[1], "the grey band's own channels: {top:?}");
        assert_eq!(top[1], top[2], "the grey band's own channels: {top:?}");
        assert_eq!(top[3], 255, "the texel is opaque");
    }

    #[test]
    fn the_orb_sheet_pixel_rides_the_16_unit_space() {
        // the sheet's own sampler: the 0..16 sprite space maps over the
        // 64px sheet (the 4x scale the /16 law speaks), the icon
        // windows' edges speak their own texels
        let mut rgba = vec![0u8; 64 * 64 * 4];
        // the (17, 17) texel speaks a known green — uv (4.25, 4.25)
        rgba[(17 * 64 + 17) * 4] = 0x80;
        rgba[(17 * 64 + 17) * 4 + 1] = 0xFF;
        let sheet = OrbSheet { rgba };
        let px = sheet.pixel(4.25, 4.25);
        assert_eq!((px[0], px[1]), (0x80, 0xFF));
        // the icon 0's window spans uv 0..4 — the (15, 15) texel is the
        // window's far edge (uv (3.75, 3.75) lands there)
        let mut rgba = vec![0u8; 64 * 64 * 4];
        rgba[(15 * 64 + 15) * 4 + 2] = 0x20;
        let sheet = OrbSheet { rgba };
        let px = sheet.pixel(3.75, 3.75);
        assert_eq!(px[2], 0x20);
    }

    /// A yaw-0 body (facing +Z south): the model's right arm lands on
    /// the world's -X (the cross product's own verdict for a +Z
    /// facing), the head top at 2.001 (the 24-texel line + the head's
    /// 8 texels), the feet line at 0.001.
    #[test]
    fn the_rest_pose_lands_where_the_entity_chain_speaks() {
        let model = AvatarModel {
            rgba: vec![255; 64 * 64 * 4],
            side_v: 64,
        };
        let feet = Vec3::new(10.0, 0.0, 20.0);
        let quads = model.quads(&AvatarState::rest(0.0, 0.0, 0.0), feet, 0.0, 0.0);
        assert_eq!(quads.len(), 12 * 6, "the whole model: 12 cubes, 6 faces each");
        // the world-space y span: the head's own top face at 2.001, the
        // hat's inflated top at 2.03125 above it; the feet line at
        // 0.001 with the legs' ±0.005 zRot dipping the inflated pants
        // corner a hair below (the vanilla pose's own numbers)
        let ys: Vec<f32> = quads
            .iter()
            .flat_map(|q| q.pts.iter().map(|p| p[1]))
            .collect();
        let ymin = ys.iter().copied().fold(f32::INFINITY, f32::min);
        let ymax = ys.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        assert!(ymin > -0.02 && ymin < 0.001, "feet line {ymin}");
        assert!((ymax - 2.03225).abs() < 1e-4, "hat top {ymax}");
        // the head's own top cube face at 2.001, x span -0.25..0.25
        // (the 8-texel head under the hat)
        let head_top: Vec<&AvatarQuad> = quads
            .iter()
            .filter(|q| q.pts.iter().all(|p| (p[1] - 2.001).abs() < 1e-3))
            .collect();
        assert!(!head_top.is_empty(), "the head's top face sits at 2.001");
        let xs: Vec<f32> = head_top
            .iter()
            .flat_map(|q| q.pts.iter().map(|p| p[0] - 10.0))
            .collect();
        let xmin = xs.iter().copied().fold(f32::INFINITY, f32::min);
        let xmax = xs.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        assert!(
            (xmin - -0.25).abs() < 1e-4 && (xmax - 0.25).abs() < 1e-4,
            "head x {xmin}..{xmax}"
        );
    }

    /// The face side: a yaw-0 body's NORTH cube face (the skin's face
    /// texels at rect (8,8)) lands as the world +Z face — the side the
    /// camera sees from the front view.
    #[test]
    fn the_models_north_face_becomes_the_worlds_facing_side() {
        let model = AvatarModel {
            rgba: vec![255; 64 * 64 * 4],
            side_v: 64,
        };
        let feet = Vec3::new(0.0, 0.0, 0.0);
        let quads = model.quads(&AvatarState::rest(0.0, 0.0, 0.0), feet, 0.0, 0.0);
        // the head's face quad: z == +0.25 (the 4-texel half-depth),
        // u spans the face rect (8..16 in texels)
        let face = quads
            .iter()
            .find(|q| {
                q.pts.iter().all(|p| (p[2] - 0.25).abs() < 1e-4)
                    && q.pts.iter().all(|p| p[1] > 1.4 && p[1] < 2.1)
                    && q.uv.iter().all(|uv| uv[0] >= 8.0 && uv[0] <= 16.0)
            })
            .expect("the head's face quad at the world's +Z");
        let us: Vec<f32> = face.uv.iter().map(|uv| uv[0]).collect();
        assert!(
            us.iter().copied().fold(f32::INFINITY, f32::min) - 8.0 < 1e-4
                && us.iter().copied().fold(f32::NEG_INFINITY, f32::max) - 16.0 < 1e-4,
            "the face rect's own u span"
        );
    }

    /// The walk law: a mid-stride walk phase swings the legs — the
    /// left leg's xRot is the cos(pos*0.6662 + π) branch, and the
    /// world-space parts leave the standing pose.
    #[test]
    fn the_walk_cosines_swing_the_legs() {
        let model = AvatarModel {
            rgba: vec![255; 64 * 64 * 4],
            side_v: 64,
        };
        let feet = Vec3::new(0.0, 0.0, 0.0);
        let mut state = AvatarState::rest(0.0, 0.0, 0.0);
        state.walk_pos = 0.6662_f32.acos() / 0.6662; // cos(pos*0.6662) = 0
        state.walk_speed = 1.0;
        let walking = model.quads(&state, feet, 0.0, 0.0);
        let standing = model.quads(&AvatarState::rest(0.0, 0.0, 0.0), feet, 0.0, 0.0);
        let diff = walking
            .iter()
            .zip(standing.iter())
            .any(|(a, b)| (a.pts[0][0] - b.pts[0][0]).abs() > 1e-4);
        assert!(diff, "the walk pose moves the parts");
    }

    /// The crouch law: the pivots' nudges + the renderer's own -2/16
    /// drop lower the whole model, and the body's 0.5 xRot tilts it.
    #[test]
    fn the_crouch_drops_and_tilts_the_model() {
        let model = AvatarModel {
            rgba: vec![255; 64 * 64 * 4],
            side_v: 64,
        };
        let feet = Vec3::new(0.0, 0.0, 0.0);
        let mut state = AvatarState::rest(0.0, 0.0, 0.0);
        state.crouching = true;
        let crouched = model.quads(&state, feet, 0.0, 0.0);
        let standing = model.quads(&AvatarState::rest(0.0, 0.0, 0.0), feet, 0.0, 0.0);
        let ymax = |qs: &[AvatarQuad]| {
            qs.iter()
                .flat_map(|q| q.pts.iter().map(|p| p[1]))
                .fold(f32::NEG_INFINITY, f32::max)
        };
        assert!(
            ymax(&crouched) < ymax(&standing),
            "the crouched model stands lower ({} < {})",
            ymax(&crouched),
            ymax(&standing)
        );
    }

    /// The entity diffuse: the mix_light law's own numbers — a face
    /// normal aligned with both lights saturates at 1.0, one facing
    /// away sits at the 0.5 base, and the fold speaks the camera.
    #[test]
    fn the_entity_diffuse_speaks_the_mix_light_law() {
        // straight up: dot(L0, up) = L0.y, dot(L1, up) = L1.y — both
        // positive, the sum saturates at 1.0
        assert!((entity_diffuse([0.0, 1.0, 0.0], 0.0, 0.0) - 1.0).abs() < 1e-4);
        // straight down: both dots negative -> the 0.5 base
        assert!((entity_diffuse([0.0, -1.0, 0.0], 0.0, 0.0) - 0.5).abs() < 1e-4);
        // the camera-relative fold: the same normal from the opposite
        // yaw rotates the view-space dot products
        let a = entity_diffuse([1.0, 0.0, 0.0], 0.0, 0.0);
        let b = entity_diffuse([1.0, 0.0, 0.0], std::f32::consts::PI, 0.0);
        assert!(a != b, "the view-space fold speaks the camera");
    }

    /// The swing law: attackTime 0.5 bends the attacking arm past the
    /// standing pose (the WHACK arc's own numbers).
    #[test]
    fn the_swing_bends_the_attacking_arm() {
        let model = AvatarModel {
            rgba: vec![255; 64 * 64 * 4],
            side_v: 64,
        };
        let feet = Vec3::new(0.0, 0.0, 0.0);
        let mut state = AvatarState::rest(0.0, 0.0, 0.0);
        state.attack_time = 0.5;
        let swung = model.quads(&state, feet, 0.0, 0.0);
        let standing = model.quads(&AvatarState::rest(0.0, 0.0, 0.0), feet, 0.0, 0.0);
        let moved = swung
            .iter()
            .zip(standing.iter())
            .any(|(a, b)| (a.pts[0][1] - b.pts[0][1]).abs() > 1e-4);
        assert!(moved, "the swing moved the arm");
    }

    /// THE PART INDEX THE SWING TESTS READ: build_posed_parts walks
    /// head, body, right arm, left arm, right leg, left leg — the
    /// createMesh order the vec construction keeps.
    fn arm_parts(parts: &[Part]) -> (&Part, &Part) {
        (&parts[2], &parts[3])
    }

    /// THE WHACK ARC'S 26.2 VERDICT (javap setupAttackAnimation pc
    /// 252-369 + the shared folds pcs 64-212, the f32 chain
    /// scripts/pins_479.py computed over the SIN table): the shared
    /// orbit pivots (±cos/±sin of body.yRot the ldc 5.0 radius wears),
    /// the shared rotation adds (the off arm the +body.yRot pair
    /// keeps), and the arc the outQuart speaks — the sin(sqrt(t)·π)
    /// term the old law rode the jar retired (at t=0.5 the two laws
    /// never agree: 0.19509032 vs 0.70710677).
    #[test]
    fn the_whack_arc_the_out_quart_verdict_speaks() {
        let mut state = AvatarState::rest(0.0, 0.0, 0.0);
        state.attack_time = 0.5;
        state.swing_kind = crate::hand::SwingKind::Whack;
        let parts = build_posed_parts(&state);
        let (r, l) = arm_parts(&parts);
        // THE ORBIT (pc 64-155): body_y = 0.2·sin(sqrt(0.5)·2π) =
        // -0.19277565, the pivots the assignments the swing owns
        assert_eq!(r.pivot[0].to_bits(), 0xc09d_090f, "right pivot x = -cos·5"); // -4.907355785369873
        assert_eq!(r.pivot[2].to_bits(), 0xbf75_23e8, "right pivot z = sin·5"); // -0.9575791358947754
        assert_eq!(l.pivot[0].to_bits(), 0x409d_090f, "left pivot x = cos·5");
        assert_eq!(l.pivot[2].to_bits(), 0x3f75_23e8, "left pivot z = -sin·5");
        // THE SHARED ADDS (pc 158-212): the off (left) arm yRot/xRot
        // the +body.yRot pair keeps (the rest pose the walk keeps 0 —
        // the bob's x term the sin(age·0.067) speaks is 0 at age 0)
        assert_eq!(l.y_rot.to_bits(), 0xbe45_66fb, "left yRot = body_y"); // -0.1927756518125534
        assert_eq!(l.x_rot.to_bits(), 0xbe45_66fb, "left xRot = bob(0) + body_y"); // -0.1927756518125534
        // THE ARC (pc 252-369) on the attack arm:
        // xRot = 0 (the bob) - (sin(outQuart(0.5)·π)·1.2 + 0.525)
        assert_eq!(r.x_rot.to_bits(), 0xbf42_54ed, "right xRot the outQuart wears"); // -0.7591083645820618
        assert_eq!(r.y_rot.to_bits(), 0xbf14_0d3c, "right yRot = body_y·3"); // -0.578326940536499
        assert_eq!(r.z_rot.to_bits(), 0xbe4c_cccd, "right zRot the sin(πt)·-0.4 keeps"); // -0.2
        // THE OLD LAW THE JAR RETIRED: sin(sqrt(0.5)·π) = 0.70710677 —
        // the arc xRot the sqrt law kept (-(0.70710677·1.2 + 0.525) =
        // -1.4735) the outQuart verdict (-0.7591) never shares
        let old = -(0.707_106_77_f32 * 1.2 + 0.525);
        assert!(r.x_rot != old, "the retired sqrt law the arc never keeps");
    }

    /// THE STAB'S THIRD-PERSON DISPATCH (javap
    /// `SpearAnimations.thirdPersonAttackHand` pcs 0-133 + the
    /// HumanoidModel$1 $SwitchMap the STAB→3 the clinit banks): the
    /// three shared adds UNDO (the f32 putfield/getfield round-trip
    /// the drift wears — (−0.1 + body_y) − body_y lands a hair off
    /// the −0.1 the naive identity keeps), the orbit pivots the
    /// shared folds never undo, and the xRot window rides the ATTACK
    /// arm only (the 90/120/30-degree fold the helper computes).
    #[test]
    fn the_stab_third_person_the_undo_and_the_window_speak() {
        let mut state = AvatarState::rest(0.0, 0.0, 0.0);
        state.attack_time = 0.5;
        state.swing_kind = crate::hand::SwingKind::Stab;
        let parts = build_posed_parts(&state);
        let (r, l) = arm_parts(&parts);
        // the undo (pc 10-64): rightArm.yRot -= body.yRot — the
        // (0 + body_y) - body_y chain the exact zero lands (the rest
        // pose the bob keeps 0 — the f32 the negation addition is
        // exact when the start rides zero)
        assert_eq!(r.y_rot.to_bits(), 0.0f32.to_bits(), "right yRot the undo zeroes");
        assert_eq!(l.y_rot.to_bits(), 0.0f32.to_bits(), "left yRot the undo zeroes");
        // the left xRot round-trip: (0 + body_y) - body_y = 0 exact
        assert_eq!(l.x_rot.to_bits(), 0.0f32.to_bits(), "left xRot the round-trip zeroes");
        // the window on the attack arm: 0 (the bob) + (-0.52102184)
        assert_eq!(r.x_rot.to_bits(), 0xbf05_61b0, "right xRot the window rides"); // -0.521021842956543
        // the z the stab never folds (the jar's own scope: xRot only)
        assert_eq!(r.z_rot.to_bits(), 0.2f32.to_bits(), "right zRot untouched");
        // THE ORBIT the undo never touches (the pivots the shared
        // assignments own — identical to the whack's)
        assert_eq!(r.pivot[0].to_bits(), 0xc09d_090f);
        assert_eq!(r.pivot[2].to_bits(), 0xbf75_23e8);
        // THE MIRROR (getArm(attackArm)): the left-attacking body the
        // window rides the LEFT arm, the right arm the bare bob keeps
        state.attack_right = false;
        let parts = build_posed_parts(&state);
        let (r, l) = arm_parts(&parts);
        assert_eq!(r.x_rot.to_bits(), 0.0f32.to_bits(), "right xRot the bare bob keeps");
        assert_eq!(l.x_rot.to_bits(), 0xbf05_61b0, "left xRot the window rides"); // -0.521021842956543
        // the LEFT flip the body fold wears (negated body_y = +0.19277565)
        assert_eq!(l.y_rot.to_bits(), 0.0f32.to_bits());
    }

    /// THE NONE KIND'S BARE GOTO (javap pc 372: the switch's case 2
    /// the bare `goto 380` speaks): the shared folds the kind keeps
    /// (the orbit pivots + the three rotation adds), no arc any arm
    /// wears — the WHACK arc's zRot/extra yRot the none never adds.
    #[test]
    fn the_none_kind_the_bare_shared_folds_keep() {
        let mut state = AvatarState::rest(0.0, 0.0, 0.0);
        state.attack_time = 0.5;
        state.swing_kind = crate::hand::SwingKind::None;
        let parts = build_posed_parts(&state);
        let (r, l) = arm_parts(&parts);
        // the shared adds only: yRot = body_y, xRot = bob(0) + body_y (left)
        assert_eq!(r.y_rot.to_bits(), 0xbe45_66fb, "right yRot = body_y");
        assert_eq!(r.x_rot.to_bits(), 0.0f32.to_bits(), "right xRot the bare bob keeps");
        assert_eq!(r.z_rot.to_bits(), 0.2f32.to_bits(), "right zRot the bare bob keeps");
        assert_eq!(l.y_rot.to_bits(), 0xbe45_66fb, "left yRot = body_y");
        assert_eq!(l.x_rot.to_bits(), 0xbe45_66fb, "left xRot = bob(0) + body_y");
        // the orbit pivots the shared fold owns (same as every kind)
        assert_eq!(r.pivot[0].to_bits(), 0xc09d_090f);
        assert_eq!(r.pivot[2].to_bits(), 0xbf75_23e8);
    }

    /// THE DEATH FLIP (javap: the deathTime branch's ZP rotation riding
    /// between the S and the R_y — `sqrt(min(1, (deathTime-1)/20*1.6))
    /// * 90°`): at the 20-tick mark the model lies flat, the y span
    /// collapses from the standing ~2.03 to the body's own thickness.
    #[test]
    fn the_death_flip_lays_the_model_flat() {
        let model = AvatarModel {
            rgba: vec![255; 64 * 64 * 4],
            side_v: 64,
        };
        let feet = Vec3::new(0.0, 0.0, 0.0);
        let mut state = AvatarState::rest(0.0, 0.0, 0.0);
        state.death_time = 20.0;
        let flipped = model.quads(&state, feet, 0.0, 0.0);
        let standing = model.quads(&AvatarState::rest(0.0, 0.0, 0.0), feet, 0.0, 0.0);
        let span = |qs: &[AvatarQuad]| {
            let ys: Vec<f32> = qs.iter().flat_map(|q| q.pts.iter().map(|p| p[1])).collect();
            ys.iter().copied().fold(f32::NEG_INFINITY, f32::max)
                - ys.iter().copied().fold(f32::INFINITY, f32::min)
        };
        let (flipped_span, standing_span) = (span(&flipped), span(&standing));
        assert!(
            flipped_span < standing_span * 0.65 && standing_span > 2.0,
            "the flip collapses the y span ({flipped_span} vs standing {standing_span})"
        );
        // the alive state never flips (the deathTime 0 gate)
        let alive = model.quads(&AvatarState::rest(0.0, 0.0, 0.0), feet, 0.0, 0.0);
        assert!((span(&alive) - standing_span).abs() < 1e-5);
    }

    /// THE ZOMBIE ARMS (javap `AnimationUtils.animateZombieArms` +
    /// `animateAttackArms`): the outstretched pose overwrites the
    /// humanoid swing — the arms reach forward, the model's z span
    /// grows well past the resting body's own depth, and the
    /// aggressive (-PI/1.5) pose reaches FARTHER than the calm
    /// (-PI/2.25) one.
    #[test]
    fn the_zombie_arms_hold_the_outstretched_pose() {
        let model = AvatarModel {
            rgba: vec![255; 64 * 64 * 4],
            side_v: 64,
        };
        let feet = Vec3::new(0.0, 0.0, 0.0);
        let rest = model.quads(&AvatarState::rest(0.0, 0.0, 0.0), feet, 0.0, 0.0);
        let mut calm = AvatarState::rest(0.0, 0.0, 0.0);
        calm.zombie = true;
        let mut mad = calm.clone();
        mad.zombie_aggressive = true;
        let calm_quads = model.quads(&calm, feet, 0.0, 0.0);
        let mad_quads = model.quads(&mad, feet, 0.0, 0.0);
        let zspan = |qs: &[AvatarQuad]| {
            let zs: Vec<f32> = qs.iter().flat_map(|q| q.pts.iter().map(|p| p[2])).collect();
            zs.iter().copied().fold(f32::NEG_INFINITY, f32::max)
                - zs.iter().copied().fold(f32::INFINITY, f32::min)
        };
        let (rest_span, calm_span, mad_span) = (zspan(&rest), zspan(&calm_quads), zspan(&mad_quads));
        assert!(
            calm_span > rest_span + 0.2,
            "the arms reach forward ({calm_span} > rest {rest_span})"
        );
        assert!(
            mad_span < calm_span,
            "the aggressive arms LIFT (-120°) shortening the forward reach ({mad_span} < {calm_span})"
        );
    }

    /// THE FRAME LAW — fire_0 rides the mcmeta playlist [16..31, 0..15]
    /// (the strip plays tile 16 at tick 0), fire_1 the sequential % 32;
    /// both wrap with the day tick.
    #[test]
    fn the_fire_frames_speak_the_mcmeta_playlist() {
        assert_eq!(fire0_frame(0), 16, "tick 0 plays tile 16");
        assert_eq!(fire0_frame(15), 31, "the playlist's first half climbs");
        assert_eq!(fire0_frame(16), 0, "the wrap to tile 0");
        assert_eq!(fire0_frame(48), 0, "the 32-tick period wraps");
        assert_eq!(fire0_frame(-1), 15, "the negative tick lands on 15");
        assert_eq!(fire1_frame(0), 0, "fire_1 is sequential");
        assert_eq!(fire1_frame(17), 17);
        assert_eq!(fire1_frame(32), 0, "the same 32-tick period");
    }

    /// THE RING LAW — the zombie's bbox (0.6 x 1.95) scales the pose by
    /// 0.84, spawns 6 rings (h = 1.95/0.84 = 2.32 ticks down 0.45 per
    /// ring), the first spans x +-0.42, rises 0.378 per ring, narrows
    /// 0.9x per ring, leans back 0.0252 per ring; the strips alternate
    /// fire_0/fire_1 and the U mirrors on the second ring PAIR.
    #[test]
    fn the_flame_rings_speak_the_prepare_bytecode() {
        let feet = Vec3::new(10.0, 5.0, 20.0);
        let quads = flame_quads(feet, 0.6, 1.95, 0.0, 0);
        // h = 2.3214..., the ring heights 2.3214, 1.8714, ... 0.0714,
        // 0.0714 - 0.45 < 0 stops -> 6 rings
        assert_eq!(quads.len(), 6, "the zombie's ring count");
        let w = 0.6 * 1.4;
        let tz = 0.3 - 2.0 * 0.02; // (int)2.3214 = 2
        // ring 0, camera yaw 0: the straight orientation, x = +-0.5*0.84
        let q0 = &quads[0];
        let xs: Vec<f32> = q0.pts.iter().map(|p| p[0]).collect();
        assert!(
            xs.iter().all(|x| (*x - (10.0 - 0.5 * w)).abs() < 1e-4
                || (*x - (10.0 + 0.5 * w)).abs() < 1e-4),
            "the ring's corners span the narrowed width: {xs:?}"
        );
        // the base y = 5.0 (the -base fold at base 0) and the top at
        // 5.0 + 1.4*0.84 — the z at feet + (0 + tz) * w
        assert!((q0.pts[0][1] - 5.0).abs() < 1e-4, "the ring's base sits at the feet");
        assert!((q0.pts[2][1] - (5.0 + 1.4 * w)).abs() < 1e-4, "the 1.4-tall quad");
        assert!((q0.pts[0][2] - (20.0 + tz * w)).abs() < 1e-4, "the pre-translate");
        // the uv: fire_0's frame 16 tile -> v 256..272, u 0..16 straight
        assert_eq!(q0.uv[0], [0.0, 272.0], "the bottom-left straight uv");
        assert_eq!(q0.uv[2], [16.0, 256.0], "the top-right straight uv");
        // ring 2 (the odd pair): the U mirrored — ua/ub swapped
        let q2 = &quads[2];
        assert_eq!(q2.uv[0][0], 16.0, "the mirrored pair's left texel");
        assert_eq!(q2.uv[1][0], 0.0);
        // the rise: ring 1's base sits 0.45*0.84 above the feet line
        let q1 = &quads[1];
        assert!((q1.pts[0][1] - (5.0 + 0.45 * w)).abs() < 1e-4, "the 0.45 rise");
        // the narrowing: ring 1's half width = 0.5*0.9*0.84
        let half = 0.5 * 0.9 * w;
        assert!(
            q1.pts.iter().all(|p| (p[0] - (10.0 - half)).abs() < 1e-4
                || (p[0] - (10.0 + half)).abs() < 1e-4),
            "the 0.9x narrowing"
        );
    }

    /// THE BILLBOARD — the camera yaw turns the ring stack: at yaw
    /// +90° (facing +X in the client's -Z-forward convention) the
    /// quad's face normal (+Z native) swings to -X, the corners spread
    /// along Z instead of X.
    #[test]
    fn the_flame_billboard_turns_with_the_camera() {
        let feet = Vec3::new(10.0, 5.0, 20.0);
        let yaw90 = std::f32::consts::FRAC_PI_2;
        let quads = flame_quads(feet, 0.6, 1.95, yaw90, 0);
        let q0 = &quads[0];
        let w = 0.6 * 1.4;
        let tz = 0.3 - 2.0 * 0.02; // (int)2.3214 = 2
        // the ring 0 corners: the +-0.5 native x maps to world +-z (the
        // a = -yaw fold), the native z lands on the world x
        let zs: Vec<f32> = q0.pts.iter().map(|p| p[2]).collect();
        assert!(
            zs.iter().all(|z| (*z - (20.0 - 0.5 * w)).abs() < 1e-4
                || (*z - (20.0 + 0.5 * w)).abs() < 1e-4),
            "the ring spreads along Z under the +90° yaw: {zs:?}"
        );
        assert!(
            q0.pts.iter().all(|p| (p[0] - (10.0 - w * tz)).abs() < 1e-4),
            "the x folds to the depth line (the native z lands on it)"
        );
    }

    /// THE STRIPS LOAD — the real runtime cache (the generator's
    /// extraction) decodes both 16x512 strips and the first fire_0
    /// frame's pixels land orange-flame colors on a transparent
    /// background.
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_fire_strips_load_sha1_verified() {
        // the test cwd sits in the crate dir — walk up to the runtime
        // root the generator populated
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let strips = FireStrips::load(&root).expect("the strips load from the runtime cache");
        let px = strips.pixel(0, 8.0, 8.0);
        assert_eq!(px[3], 255, "the fire's core pixel is opaque");
        assert!(px[0] > px[2], "the orange reads red-over-blue");
        let hole = strips.pixel(1, 0.0, 0.0);
        assert_eq!(hole, [0, 0, 0, 0], "the cutout hole decodes transparent");
    }

    /// THE WATER SCREEN's REPEAT wrap — the sampler's own law: the uv
    /// ride the yaw/pitch scroll (negative when the offsets do) and
    /// span 4 full wraps, every fetch folds to the 16x16 tile's
    /// nearest texel.
    #[test]
    fn the_underwater_screen_wraps_the_tile() {
        let mut rgba = vec![0u8; 16 * 16 * 4];
        let put = |rgba: &mut [u8], x: usize, y: usize, c: [u8; 4]| {
            let o = (y * 16 + x) * 4;
            rgba[o..o + 4].copy_from_slice(&c);
        };
        put(&mut rgba, 0, 0, [1, 2, 3, 4]);
        put(&mut rgba, 15, 15, [5, 6, 7, 8]);
        put(&mut rgba, 3, 9, [9, 10, 11, 12]);
        let screen = UnderwaterScreen { rgba };
        assert_eq!(screen.pixel(0.0, 0.0), [1, 2, 3, 4], "the origin texel");
        assert_eq!(screen.pixel(16.0, 32.0), [1, 2, 3, 4], "the full wraps land home");
        assert_eq!(screen.pixel(-0.5, -0.5), [5, 6, 7, 8], "the negative uv wrap to the far corner");
        assert_eq!(screen.pixel(3.9, 9.2), [9, 10, 11, 12], "the nearest floor picks the texel");
        assert_eq!(screen.pixel(19.2, 25.2), [9, 10, 11, 12], "the wraps ride the same fold");
    }

    /// THE WATER SCREEN LOADS — the real runtime cache (the generator's
    /// extraction) decodes the 16x16 blue and the pixels land
    /// blue-over-red (the underwater tile's own verdict).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_underwater_screen_loads_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let screen = UnderwaterScreen::load(&root).expect("the screen loads from the runtime cache");
        let px = screen.pixel(8.0, 8.0);
        assert!(px[2] > px[0], "the blue reads blue-over-red: {px:?}");
        assert!(px[3] > 0 && px[3] < 255, "the texel is translucent: {px:?}");
    }

    /// THE BABY SHEEP'S OWN GEOMETRY (javap BabySheepModel.createBodyLayer's
    /// own numbers — the DEDICATED baby layer the AgeableMobRenderer's
    /// babyModel field bakes, NOT a scaled adult): the 5x5x5 head at
    /// pivot (0, 15.5, -2.5), the horizontal 6x4x9 body with NO x-rot
    /// at pivot (0, 17, 0.5), the four 2x5x2 legs on their FOUR
    /// distinct rects (0,23 / 24,12 / 8,23 / 24,5) with NO mirror.
    #[test]
    fn the_baby_sheeps_own_geometry_the_layer_bakes() {
        let state = SheepState {
            yaw_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
            head_eat_position: 0.0,
            head_eat_angle: 0.0,
            age_scale: 0.5,
        };
        let parts = build_baby_sheep_parts(&state);
        assert_eq!(parts.len(), 6, "the baby layer bakes six parts");
        let head = &parts[0];
        assert_eq!(head.pivot, [0.0, 15.5, -2.5], "the head's own pivot");
        assert_eq!(head.cubes.len(), 1);
        assert_eq!(head.cubes[0].tex, (0.0, 0.0), "the head's uv rect");
        assert_eq!(head.cubes[0].size, [5.0, 5.0, 5.0], "the baby head's own 5x5x5");
        let body = &parts[1];
        assert_eq!(body.pivot, [0.0, 17.0, 0.5], "the body's own pivot");
        assert_eq!(body.x_rot, 0.0, "NO rotX — the horizontal box the adult's barrel never needed");
        assert_eq!(body.cubes[0].tex, (0.0, 10.0));
        assert_eq!(body.cubes[0].size, [6.0, 4.0, 9.0]);
        // the four legs: the pivots the layer speaks, the FOUR uv
        // rects the adult's shared builder never had, no mirror
        let expect: [([f32; 3], (f32, f32)); 4] = [
            ([-2.0, 19.0, 3.0], (0.0, 23.0)),
            ([2.0, 19.0, 3.0], (24.0, 12.0)),
            ([-2.0, 19.0, -2.0], (8.0, 23.0)),
            ([2.0, 19.0, -2.0], (24.0, 5.0)),
        ];
        for (i, (pivot, tex)) in expect.iter().enumerate() {
            let leg = &parts[2 + i];
            assert_eq!(leg.pivot, *pivot, "leg {i}'s own pivot");
            assert_eq!(leg.cubes[0].tex, *tex, "leg {i}'s own uv rect");
            assert_eq!(leg.cubes[0].size, [2.0, 5.0, 2.0], "leg {i}'s stub");
            assert!(!leg.cubes[0].mirror, "leg {i} rides no mirror");
        }
    }

    /// THE BABY COW'S OWN GEOMETRY (javap BabyCowModel.createBodyLayer's
    /// own numbers): the four-cube head at pivot (0, 13.569, -5.1667) —
    /// main 6x6x5 (uv 0,18), right horn 1x2x1 (uv 8,29), LEFT horn
    /// 1x2x1 the builder's own .mirror() ride (uv 4,29), muzzle 4x3x1
    /// (uv 12,29) — the horizontal 8x6x12 body at pivot (3, 19, -5)
    /// with NO x-rot, the four 3x6x3 legs on four distinct rects.
    #[test]
    fn the_baby_cows_own_horns_the_mirror_speaks() {
        let state = QuadrupedState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let parts = build_baby_cow_parts(&state);
        assert_eq!(parts.len(), 6, "the baby layer bakes six parts");
        let head = &parts[0];
        assert_eq!(head.pivot, [0.0, 13.569, -5.1667], "the head's own pivot");
        assert_eq!(head.cubes.len(), 4, "main + two horns + muzzle");
        assert_eq!(head.cubes[0].tex, (0.0, 18.0));
        assert_eq!(head.cubes[0].size, [6.0, 6.0, 5.0]);
        assert_eq!(head.cubes[1].tex, (8.0, 29.0), "the right horn's rect");
        assert_eq!(head.cubes[1].origin, [3.0, -5.569, -3.8333]);
        assert!(!head.cubes[1].mirror, "the right horn rides no mirror");
        assert_eq!(head.cubes[2].tex, (4.0, 29.0), "the left horn's rect");
        assert_eq!(head.cubes[2].origin, [-4.0, -5.569, -3.8333]);
        assert!(head.cubes[2].mirror, "the LEFT horn the builder's own mirror speaks");
        assert_eq!(head.cubes[3].tex, (12.0, 29.0), "the muzzle's rect");
        assert_eq!(head.cubes[3].size, [4.0, 3.0, 1.0]);
        let body = &parts[1];
        assert_eq!(body.pivot, [3.0, 19.0, -5.0], "the body's own +3 x pivot");
        assert_eq!(body.x_rot, 0.0, "NO rotX — the horizontal box");
        assert_eq!(body.cubes[0].tex, (0.0, 0.0));
        assert_eq!(body.cubes[0].size, [8.0, 6.0, 12.0]);
        let expect: [([f32; 3], (f32, f32)); 4] = [
            ([-2.5, 18.0, -3.5], (22.0, 18.0)),
            ([2.5, 18.0, -3.5], (34.0, 18.0)),
            ([-2.5, 18.0, 3.5], (22.0, 27.0)),
            ([2.5, 18.0, 3.5], (34.0, 27.0)),
        ];
        for (i, (pivot, tex)) in expect.iter().enumerate() {
            let leg = &parts[2 + i];
            assert_eq!(leg.pivot, *pivot, "leg {i}'s own pivot");
            assert_eq!(leg.cubes[0].tex, *tex, "leg {i}'s own uv rect");
            assert_eq!(leg.cubes[0].size, [3.0, 6.0, 3.0], "leg {i}'s own stub");
        }
    }

    /// THE BABY PIG'S OWN GEOMETRY (javap BabyPigModel.createBodyLayer's
    /// own numbers): the horizontal 7x6x9 body at pivot (0, 19, 0.5)
    /// with NO x-rot, the two-cube head at pivot (0, 19, -2) — the
    /// inflate deformations 0.025 / 0.015 the adult's NONE never spoke
    /// — and the four 2x2x2 STUB legs on four distinct rects, the
    /// baby's own 32x32 texture the load law pins.
    #[test]
    fn the_baby_pigs_own_stubs_the_inflate_speaks() {
        let state = PigState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let parts = build_baby_pig_parts(&state);
        assert_eq!(parts.len(), 6, "the baby layer bakes six parts");
        let head = &parts[0];
        assert_eq!(head.pivot, [0.0, 19.0, -2.0], "the head's own pivot");
        assert_eq!(head.cubes.len(), 2, "head + snout");
        assert_eq!(head.cubes[0].tex, (0.0, 15.0));
        assert_eq!(head.cubes[0].size, [7.0, 6.0, 6.0]);
        assert!(
            (head.cubes[0].inflate - 0.025).abs() < 1e-6,
            "the head's own inflate the baby layer speaks"
        );
        assert_eq!(head.cubes[1].tex, (6.0, 27.0), "the snout's rect");
        assert_eq!(head.cubes[1].size, [3.0, 2.0, 1.0]);
        assert!(
            (head.cubes[1].inflate - 0.015).abs() < 1e-6,
            "the snout's own inflate"
        );
        let body = &parts[1];
        assert_eq!(body.pivot, [0.0, 19.0, 0.5]);
        assert_eq!(body.x_rot, 0.0, "NO rotX — the horizontal box");
        assert_eq!(body.cubes[0].tex, (0.0, 0.0));
        assert_eq!(body.cubes[0].size, [7.0, 6.0, 9.0]);
        let expect: [([f32; 3], (f32, f32)); 4] = [
            ([2.5, 22.0, -3.0], (0.0, 0.0)),
            ([-2.5, 22.0, -3.0], (23.0, 0.0)),
            ([2.5, 22.0, 4.0], (0.0, 4.0)),
            ([-2.5, 22.0, 4.0], (23.0, 4.0)),
        ];
        for (i, (pivot, tex)) in expect.iter().enumerate() {
            let leg = &parts[2 + i];
            assert_eq!(leg.pivot, *pivot, "leg {i}'s own pivot");
            assert_eq!(leg.cubes[0].tex, *tex, "leg {i}'s own uv rect");
            assert_eq!(leg.cubes[0].size, [2.0, 2.0, 2.0], "leg {i}'s own stub");
        }
    }

    /// THE BABY WALK'S OWN PAIRING (the inherited
    /// QuadrupedModel.setupAnim the baby models ride verbatim): the
    /// DIAGONAL pairs swing together — right-hind + left-front in
    /// phase, left-hind + right-front +PI — the same law the adult's
    /// legs speak, the baby's own stubs beneath.
    #[test]
    fn the_baby_pigs_walk_rides_the_diagonal_pairs() {
        let rest = PigState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let walked = PigState {
            walk_pos: 0.0,
            walk_speed: 1.0,
            ..rest
        };
        let rest_parts = build_baby_pig_parts(&rest);
        let walked_parts = build_baby_pig_parts(&walked);
        // the leg swing the fold speaks: cos(0) * 1.4 at the walk_pos 0
        let rh = mth_cos(0.0) * 1.4;
        assert!(
            (walked_parts[2].x_rot.abs() - rh.abs()) < 1e-5,
            "the legs' swing magnitude rides the 1.4 the law speaks"
        );
        // the DIAGONAL pairing: right-hind (part 5) + left-front (part
        // 2) in phase, left-hind (part 3) + right-front (part 4) +PI
        let (lf, rf, lh, rhind) = (
            &walked_parts[2],
            &walked_parts[3],
            &walked_parts[4],
            &walked_parts[5],
        );
        assert_eq!(
            lf.x_rot, rhind.x_rot,
            "right-hind + left-front the diagonal pair in phase"
        );
        assert_eq!(
            lh.x_rot, rf.x_rot,
            "left-hind + right-front the anti pair in phase"
        );
        assert!(
            (lf.x_rot + lh.x_rot).abs() < 1e-5,
            "the two pairs ride the +PI the cos fold speaks"
        );
        assert!(rest_parts[2].x_rot == 0.0, "the rest stand never swings");
    }

    /// THE BABY MODELS' OWN STATURE (the entity chain's world-space
    /// verdict): the baby meshes stand the ground the legs' own 24
    /// model-y reaches, the heads' tops the DEDICATED geometry's own
    /// heights speak — the baby sheep 13/16 above the feet (model y 11
    /// head top vs the adult's 22/16), the baby pig 10/16 + inflate.
    #[test]
    fn the_baby_models_span_the_dedicated_heights() {
        let feet = Vec3::new(10.0, 64.0, 20.0);
        let sheep_state = SheepState {
            yaw_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
            head_eat_position: 0.0,
            head_eat_angle: 0.0,
            age_scale: 0.5,
        };
        let baby_sheep = BabySheepModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let quads = baby_sheep.quads(&sheep_state, feet, 0.0, 0.0);
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        for q in &quads {
            for p in q.pts {
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
            }
        }
        // legs' bottoms at model y 24 (the ground), head's top at
        // model y 11 (pivot 15.5 - 4.5) -> feet + 13/16
        assert!((min_y - (feet.y as f32 + 0.001)).abs() < 1e-4, "the baby sheep stands the ground: {min_y}");
        assert!(
            (max_y - (feet.y as f32 + 13.0 / 16.0 + 0.001)).abs() < 1e-4,
            "the baby sheep's head top rides 13/16: {max_y}"
        );
        // the ADULT comparison the dedicated layer's own proportions
        // speak: the adult head top at model y 2 -> feet + 22/16 — the
        // baby stands under HALF the adult's stature
        let adult_state = SheepState { age_scale: 1.0, ..sheep_state };
        let adult = SheepModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let adult_quads = adult.quads(&adult_state, feet, 0.0, 0.0);
        let adult_max = adult_quads
            .iter()
            .flat_map(|q| q.pts.iter().map(|p| p[1]))
            .fold(f32::MIN, f32::max);
        assert!(
            max_y < adult_max - 0.25,
            "the baby's stature the dedicated geometry keeps under the adult's: {max_y} vs {adult_max}"
        );
        // THE BABY PIG: head top at model y 14 - 0.025 inflate -> feet
        // + 10/16 (the 0.625 the stub-legged piglet speaks)
        let pig_state = PigState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
        };
        let baby_pig = BabyPigModel {
            rgba: vec![255u8; 32 * 32 * 4],
        };
        let pig_quads = baby_pig.quads(&pig_state, feet, 0.0, 0.0);
        let pig_max = pig_quads
            .iter()
            .flat_map(|q| q.pts.iter().map(|p| p[1]))
            .fold(f32::MIN, f32::max);
        assert!(
            (pig_max - (feet.y as f32 + 10.025 / 16.0 + 0.001)).abs() < 1e-3,
            "the baby pig's head top rides 10/16 + the inflate: {pig_max}"
        );
    }

    /// THE BABY WOOL'S OWN BOXES (javap LayerDefinitions: the
    /// SHEEP_BABY_WOOL layer bakes BabySheepModel.createBodyLayer —
    /// the SAME loc49 the SHEEP_BABY layer rides): the wool overlay's
    /// quads the BABY body's own geometry speak, pixel-identical
    /// positions the baby body's builder leaves — the adult fur's
    /// inflate boxes the baby never wears.
    #[test]
    fn the_baby_wool_rides_the_babys_own_boxes() {
        let state = SheepState {
            yaw_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            death_time: 0.0,
            head_eat_position: 0.0,
            head_eat_angle: 0.0,
            age_scale: 0.5,
        };
        let feet = Vec3::new(1.0, 64.0, 2.0);
        let wool = BabySheepWoolModel {
            rgba: vec![200u8; 64 * 32 * 4],
        };
        let body = BabySheepModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let wool_quads = wool.quads(&state, feet, 0.0, 0.0);
        let body_quads = body.quads(&state, feet, 0.0, 0.0);
        assert_eq!(wool_quads.len(), body_quads.len(), "the same mesh the layer bakes");
        for (wq, bq) in wool_quads.iter().zip(body_quads.iter()) {
            assert_eq!(wq.pts, bq.pts, "the wool rides the baby body's own boxes");
        }
        // the sampler's own verdict: the baby wool texture the pixel
        // read answers (the uniform fixture the constructor carries —
        // every channel the 200 word fills, alpha included)
        assert_eq!(
            wool.pixel(5.0, 5.0),
            [200, 200, 200, 200],
            "the baby wool texture the sampler reads"
        );
    }

    /// THE BABY SKINS LOAD (the runtime cache's own verdict — the
    /// generator's extraction decodes the four _baby textures the
    /// sha1s pin, each its OWN layout the baby layer defs speak).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_baby_skins_load_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let sheep = BabySheepModel::load(&root).expect("the baby sheep loads");
        // the baby sheep's skin reads blue-less warm texels — the
        // opaque verdict the decode law keeps
        let px = sheep.pixel(8.0, 8.0);
        assert_eq!(px[3], 255, "the baby sheep's texel is opaque");
        let wool = BabySheepWoolModel::load(&root).expect("the baby wool loads");
        let wpx = wool.pixel(20.0, 10.0);
        assert_eq!(wpx[0], wpx[1], "the greyscale wool rides equal channels");
        let cow = BabyCowModel::load(&root).expect("the baby cow loads");
        assert_eq!(cow.pixel(16.0, 20.0)[3], 255, "the baby cow's texel is opaque");
        let pig = BabyPigModel::load(&root).expect("the baby pig loads");
        assert_eq!(pig.pixel(8.0, 18.0)[3], 255, "the baby pig's texel is opaque");
    }

    /// THE ARMOR LAYERS' OWN TABLES (javap 26.2 the round pinned): the
    /// slot's part set the ADULT_ARMOR_PARTS_PER_SLOT clinit speaks
    /// (HEAD {head}, CHEST {body, right_arm, left_arm}, LEGS {body,
    /// right_leg, left_leg}, FEET {right_leg, left_leg}), the
    /// deformation pair the LayerDefinitions clinit speaks (the OUTER
    /// 1.0 the helmet/chest/boots models ride, the INNER 0.5 the LEGS
    /// slot's usesInnerModel verdict keeps, the legs pair the -0.1
    /// extend createBaseArmorMesh speaks), and the mirror flags the
    /// 64x32 armor layout's shared left UVs keep.
    #[test]
    fn the_armor_parts_pin_the_slot_tables() {
        let state = AvatarState::rest(0.0, 0.0, 0.0);
        // THE HELMET: the head alone (no hat part — the armor mesh has
        // no overlays), the OUTER deformation 1.0, texOffs (0, 0)
        let head = armor_parts(&state, ARMOR_SLOT_HEAD);
        assert_eq!(head.len(), 1, "the helmet keeps the head alone");
        assert_eq!(head[0].pivot, [0.0, 0.0, 0.0]);
        assert_eq!(head[0].children.len(), 0, "the armor mesh has no overlays");
        assert_eq!((head[0].cubes[0].tex, head[0].cubes[0].inflate), ((0.0, 0.0), 1.0));
        // THE CHESTPLATE: the body + BOTH arms, the left the MIRROR
        // flag speaks (the skin's dedicated left UVs the armor layout
        // never shared), the arms' pivots the createMesh walk keeps
        let chest = armor_parts(&state, ARMOR_SLOT_CHEST);
        assert_eq!(chest.len(), 3, "the chest keeps body + both arms");
        assert_eq!((chest[0].cubes[0].tex, chest[0].cubes[0].inflate), ((16.0, 16.0), 1.0));
        assert_eq!(
            (chest[1].pivot, chest[1].cubes[0].tex, chest[1].cubes[0].mirror),
            ([-5.0, 2.0, 0.0], (40.0, 16.0), false),
            "the right arm speaks no mirror"
        );
        assert_eq!(
            (chest[2].pivot, chest[2].cubes[0].tex, chest[2].cubes[0].mirror),
            ([5.0, 2.0, 0.0], (40.0, 16.0), true),
            "the left arm rides the shared-UV mirror"
        );
        // THE LEGGINGS: the INNER model (0.5) — the body AND both legs
        // (the setPartVisibility walk the LEGS arm speaks), the legs'
        // -0.1 extend 0.4 keeps
        let legs = armor_parts(&state, ARMOR_SLOT_LEGS);
        assert_eq!(legs.len(), 3, "the leggings keep body + both legs");
        assert_eq!(legs[0].cubes[0].inflate, 0.5, "the body rides the INNER deformation");
        assert_eq!(
            (legs[1].pivot, legs[1].cubes[0].tex, legs[1].cubes[0].inflate),
            ([-1.9, 12.0, 0.0], (0.0, 16.0), 0.4),
            "the right leg the extend(-0.1) speaks"
        );
        assert_eq!(legs[2].cubes[0].mirror, true, "the left leg mirrors");
        // THE BOOTS: the legs pair alone, the OUTER model's own 0.9
        // (1.0 - the 0.1 extend)
        let feet = armor_parts(&state, ARMOR_SLOT_FEET);
        assert_eq!(feet.len(), 2, "the boots keep the legs pair alone");
        assert_eq!(feet[0].cubes[0].inflate, 0.9);
        assert_eq!(feet[1].cubes[0].inflate, 0.9);
        // the unknown slot never speaks
        assert!(armor_parts(&state, 9).is_empty());
    }

    /// THE WINGS LANE'S GEOMETRY (javap `ElytraModel.createLayer` +
    /// `setupAnim` + `WingsLayer.submit` the 26.2 walk — the 12 quads
    /// the two cubes speak, the ±5 pivots the root offsets keep, the
    /// 0.125 z translate the layer's own push wears, the mirror the
    /// right wing's UV rect flips, the crouch the 3-texel drop + the
    /// entity fold's own -2 the pair speaks, the glide spread the
    /// z-rot's swing wears).
    #[test]
    fn the_elytra_wings_the_model_wears() {
        // THE RAW GEOMETRY (the rot 0 the part pose's own pre-anim
        // state keeps — the pivot/inflate/translate pins the bare box
        // speaks; the anim rotations the spread arm below wears)
        let state = AvatarState::rest(180.0, 0.0, 0.0);
        let quads = wings_quads(&state, Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(quads.len(), 12, "two cubes, six faces each");
        // THE BACK'S OWN TRANSLATE: every wing point rides the +z
        // 0.125 the layer keeps (the wing z the 0.0625..0.3125 band
        // wears — the cube's -1..3 texel z the inflate speaks, the
        // 2-texel shift the 0.125*16 adds)
        let mut zs = Vec::new();
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for q in &quads {
            for p in q.pts {
                zs.push(p[2]);
                xs.push(p[0]);
                ys.push(p[1]);
            }
        }
        let zmin = zs.iter().cloned().fold(f32::MAX, f32::min);
        let zmax = zs.iter().cloned().fold(f32::MIN, f32::max);
        assert!((zmin - 0.0625).abs() < 1.0e-4, "the wing z floor, got {zmin}");
        assert!((zmax - 0.3125).abs() < 1.0e-4, "the wing z ceiling, got {zmax}");
        // THE ±5 PIVOTS: the x extent the two wings' mirrored cubes
        // wear — the cube's -11..1 texel span the inflate keeps, the
        // pivots the ±5 the root offsets speak (x -6/16..6/16)
        let xmin = xs.iter().cloned().fold(f32::MAX, f32::min);
        let xmax = xs.iter().cloned().fold(f32::MIN, f32::max);
        assert!((xmin + 6.0 / 16.0).abs() < 1.0e-4, "the left extent, got {xmin}");
        assert!((xmax - 6.0 / 16.0).abs() < 1.0e-4, "the right extent, got {xmax}");
        // THE NECK'S OWN RIDE: the model y 0..20 (+1 inflate both
        // ways) hangs from the y=0 neck line DOWN (the y-down law the
        // fold's 1.501 keeps) — the ceiling 1/16 above the neck, the
        // floor 21/16 below
        let ymax = ys.iter().cloned().fold(f32::MIN, f32::max);
        let ymin = ys.iter().cloned().fold(f32::MAX, f32::min);
        assert!((ymax - (1.501 + 1.0 / 16.0)).abs() < 1.0e-4, "the wing top, got {ymax}");
        assert!((ymin - (1.501 - 21.0 / 16.0)).abs() < 1.0e-4, "the wing bottom, got {ymin}");
        // THE MIRROR'S OWN FLAGS (the CubeListBuilder's own mirror
        // call the right wing rides — the left the plain builder
        // keeps)
        let left = elytra_wing_parts(false, 0.0, 0.0, 0.0);
        assert!(!left[0].cubes[0].mirror && left[1].cubes[0].mirror, "the mirror flags");
        // THE CROUCH DROP: the 3-texel pivot drop the setupAnim's own
        // y speaks + the entity fold's own -2/16 — the pair the
        // ceiling's 5/16 descent wears
        let crouched = AvatarState {
            crouching: true,
            ..state
        };
        let cq = wings_quads(&crouched, Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, 0.0, 0.0);
        let cymax = cq
            .iter()
            .flat_map(|q| q.pts)
            .map(|p| p[1])
            .fold(f32::MIN, f32::max);
        assert!(
            (cymax - (ymax - 5.0 / 16.0)).abs() < 1.0e-4,
            "the crouch drop the pair speaks, got {cymax} vs {ymax}"
        );
        // THE GLIDE SPREAD (javap setupAnim's zRot overwrite — the
        // -90° z the flat-out spread wears): the tip the 20-texel
        // swing lands horizontal — the x extent past the ±6/16 band
        let spread = wings_quads(
            &state,
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            0.0,
            0.34906584,
            0.0,
            -1.5707964,
        );
        let sxmax = spread
            .iter()
            .flat_map(|q| q.pts)
            .map(|p| p[0].abs())
            .fold(f32::MIN, f32::max);
        assert!(sxmax > 1.3, "the spread the tip speaks, got {sxmax}");
        let symin = spread
            .iter()
            .flat_map(|q| q.pts)
            .map(|p| p[1])
            .fold(f32::MAX, f32::min);
        assert!(
            symin > ymin + 0.5,
            "the flattened wings the vertical extent sheds, got {symin} vs {ymin}"
        );
    }

    /// THE WINGS LANE'S TEXTURE (the generated sha1 the generator's
    /// own walk banks — the real cache file loads, the 64x32 the
    /// shape-check keeps).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_wings_tex_the_sha1_verifies() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let tex = WingsTex::load(&root).expect("the wings texture loads");
        let [r, g, b, a] = tex.pixel(22.0, 0.0);
        _ = (r, g, b, a);
        // the out-of-range clamp the nearest-texel law keeps
        let [_, _, _, a2] = tex.pixel(100.0, 50.0);
        _ = a2;
    }

    /// THE ARMOR'S POSE (copyPropertiesTo's own walk): the armor part
    /// rides the skin part's EXACT pivot and setupAnim rotation — the
    /// posed quads' centroid the symmetric inflate leaves unmoved, and
    /// the helmet's crown rides exactly the 1-texel OUTER inflation
    /// above the skin's own crown (the fold the entity chain keeps
    /// verbatim).
    #[test]
    fn the_armor_quads_ride_the_skin_pose() {
        // a LIVING pose (the walk swing, the attack arc, the pitched
        // head — the copyPropertiesTo walk copies them all)
        let mut state = AvatarState::rest(25.0, 31.0, 77.0);
        state.walk_pos = 2.5;
        state.walk_speed = 0.8;
        state.attack_time = 0.4;
        let feet = Vec3::new(3.0, 64.0, -2.0);
        // the skin's own head part (the base cube alone — the hat child
        // dropped the armor's part-set law keeps)
        let posed = build_posed_parts(&state);
        let skin_head = Part {
            pivot: posed[0].pivot,
            x_rot: posed[0].x_rot,
            y_rot: posed[0].y_rot,
            z_rot: posed[0].z_rot,
            cubes: posed[0].cubes.clone(),
            children: vec![],
        };
        let mut skin_quads = Vec::new();
        let ident = Mat4::IDENT;
        let (fp, fno) = entity_folds(state.yaw_deg, state.death_time, 0.0, feet);
        collect_part(&skin_head, &ident, &fp, &fno, 0.0, 0.0, &mut skin_quads);
        let armor_head = armor_quads(&state, feet, 0.0, 0.0, ARMOR_SLOT_HEAD);
        assert_eq!(armor_head.len(), skin_quads.len(), "the same cube walk, 6 faces");
        let centroid = |quads: &[AvatarQuad]| -> [f64; 3] {
            let mut acc = [0.0f64; 3];
            let mut n = 0.0f64;
            for q in quads {
                for p in q.pts.iter() {
                    for k in 0..3 {
                        acc[k] += p[k] as f64;
                    }
                    n += 1.0;
                }
            }
            [acc[0] / n, acc[1] / n, acc[2] / n]
        };
        let (sc, ac) = (centroid(&skin_quads), centroid(&armor_head));
        for k in 0..3 {
            assert!(
                (sc[k] - ac[k]).abs() < 1e-4,
                "the armor rides the skin's own pose: {sc:?} vs {ac:?}"
            );
        }
        // the crown's 1-texel lift (the OUTER deformation 1.0) — the
        // REST pose keeps the head axis-aligned, so the crown rides
        // exactly the one texel the inflation speaks
        let rest = AvatarState::rest(0.0, 0.0, 0.0);
        let rest_feet = Vec3::new(0.0, 64.0, 0.0);
        let (rfp, rfno) = entity_folds(rest.yaw_deg, rest.death_time, 0.0, rest_feet);
        let mut rest_skin = Vec::new();
        let rest_posed = build_posed_parts(&rest);
        let rest_head = Part {
            pivot: rest_posed[0].pivot,
            x_rot: rest_posed[0].x_rot,
            y_rot: rest_posed[0].y_rot,
            z_rot: rest_posed[0].z_rot,
            cubes: rest_posed[0].cubes.clone(),
            children: vec![],
        };
        collect_part(&rest_head, &ident, &rfp, &rfno, 0.0, 0.0, &mut rest_skin);
        let rest_armor = armor_quads(&rest, rest_feet, 0.0, 0.0, ARMOR_SLOT_HEAD);
        let top = |quads: &[AvatarQuad]| -> f64 {
            quads
                .iter()
                .flat_map(|q| q.pts.iter().map(|p| p[1] as f64))
                .fold(f64::MIN, f64::max)
        };
        assert!(
            ((top(&rest_armor) - top(&rest_skin)) - 1.0 / 16.0).abs() < 1e-4,
            "the helmet rides the OUTER inflate: {} vs {}",
            top(&rest_armor),
            top(&rest_skin)
        );
    }

    /// THE ITEM'S ASSET FOLD (the vanilla datapack's default EQUIPPABLE
    /// components the armor family speaks): the material prefixes fold
    /// to the EquipmentAssets keys the 26.2 clinit pinned, the turtle
    /// helmet to its own scute, the HEAD family the asset never
    /// textures (the carved pumpkin, the skulls) and the elytra stay
    /// None (the shouldRender gate's empty-asset verdict).
    #[test]
    fn the_armor_asset_fold_pins_the_equipment_keys() {
        assert_eq!(armor_asset_name(Some("iron_helmet")), Some("iron"));
        assert_eq!(armor_asset_name(Some("golden_chestplate")), Some("gold"));
        assert_eq!(armor_asset_name(Some("chainmail_leggings")), Some("chainmail"));
        assert_eq!(armor_asset_name(Some("diamond_boots")), Some("diamond"));
        assert_eq!(armor_asset_name(Some("copper_helmet")), Some("copper"));
        assert_eq!(armor_asset_name(Some("netherite_chestplate")), Some("netherite"));
        assert_eq!(armor_asset_name(Some("leather_leggings")), Some("leather"));
        assert_eq!(armor_asset_name(Some("turtle_helmet")), Some("turtle_scute"));
        // the components the humanoid slots never texture
        assert_eq!(armor_asset_name(Some("elytra")), None, "the wings ride their own lane");
        assert_eq!(armor_asset_name(Some("carved_pumpkin")), None);
        assert_eq!(armor_asset_name(Some("skeleton_skull")), None);
        assert_eq!(armor_asset_name(Some("stone")), None);
        assert_eq!(armor_asset_name(None), None);
    }

    /// THE LEATHER UNDYED TINT (the equipment client info JSON's own
    /// `color_when_undyed: -6265536`): the packed word the tint law
    /// unpacks speaks the classic brown — 0xA06540 the base texture's
    /// grayscale multiplies (the store's bare (id, count) carries no
    /// dye component, the undyed default IS the color).
    #[test]
    fn the_leather_undyed_tint_pins_the_client_info() {
        assert_eq!(LEATHER_UNDYED_COLOR, (-6265536i32) as u32, "the JSON's own word");
        assert_eq!(LEATHER_UNDYED_COLOR, 0xFFA06540);
        let t = crate::tint::unpack(LEATHER_UNDYED_COLOR);
        let want = [160.0 / 255.0, 101.0 / 255.0, 64.0 / 255.0];
        for k in 0..3 {
            assert!((t[k] - want[k]).abs() < 1e-9, "the undyed brown: {t:?}");
        }
    }

    /// THE HEAD-BLOCK CHAIN (the CustomHeadLayer item branch's own law
    /// — the carved pumpkin the HEAD family's only plain block speaks):
    /// the rest pose's bounding box EXACTLY the centered 0.625 block
    /// over the head's own center, the carved face (the model's north —
    /// the carved_pumpkin texture) riding the skin's face side with its
    /// own face shade, the Level.getShade set the other faces speak,
    /// the head's pitch the block rides, and the element-less models
    /// the lane honestly refuses (the skull family's special-model
    /// domain — the follow-up names).
    #[test]
    fn the_head_block_rides_the_head_chain() {
        let rest = AvatarState::rest(180.0, 0.0, 0.0);
        let quads = head_block_quads(&rest, Vec3::ZERO, 0.0, 0.0, "block/carved_pumpkin")
            .expect("the carved pumpkin's block model speaks");
        // the vanilla cube: the 6 faces the block/cube chain bakes
        assert_eq!(quads.len(), 6);
        let mut lo = [f32::MAX; 3];
        let mut hi = [f32::MIN; 3];
        for q in &quads {
            for p in q.pts.iter() {
                for k in 0..3 {
                    lo[k] = lo[k].min(p[k]);
                    hi[k] = hi[k].max(p[k]);
                }
            }
        }
        // the centered 0.625 block over the head's own center: x/z
        // spans -0.3125..0.3125 (the fold's mirror keeps the span), y
        // 1.4385..2.0635 — the head's center 1.751 +- 0.3125
        assert!((lo[0] + 0.3125).abs() < 1e-4, "lo.x = {}", lo[0]);
        assert!((lo[2] + 0.3125).abs() < 1e-4, "lo.z = {}", lo[2]);
        assert!((lo[1] - 1.4385).abs() < 1e-4, "lo.y = {}", lo[1]);
        assert!((hi[0] - 0.3125).abs() < 1e-4, "hi.x = {}", hi[0]);
        assert!((hi[2] - 0.3125).abs() < 1e-4, "hi.z = {}", hi[2]);
        assert!((hi[1] - 2.0635).abs() < 1e-4, "hi.y = {}", hi[1]);
        // THE CARVED FACE (the model's north — the skin's face side):
        // one quad, the carved texture, centroid z -0.3125 (in front of
        // the skin's own -0.25 plane — the pumpkin sits proud), the
        // NORTH face's own 0.8 shade the block law speaks
        let carved: Vec<_> = quads
            .iter()
            .filter(|q| q.texture.ends_with("block/carved_pumpkin"))
            .collect();
        assert_eq!(carved.len(), 1, "the carved texture names ONE face");
        let cz = carved[0].pts.iter().map(|p| p[2]).sum::<f32>() / 4.0;
        assert!(cz < -0.3, "the carved face rides the face side: {cz}");
        assert!((carved[0].shade - 0.8).abs() < 1e-6, "the north face's shade");
        // the Level.getShade set the other faces speak (the up 1.0,
        // the down 0.5, the east/west 0.6 — the sides the pumpkin_top
        // / pumpkin_side textures paint)
        let up = quads.iter().find(|q| q.texture.ends_with("pumpkin_top") && (q.shade - 1.0).abs() < 1e-6)
            .expect("the up face shades 1.0");
        assert!(up.pts.iter().all(|p| p[1] > 2.0), "the up face rides the crown");
        assert!(
            quads.iter().any(|q| (q.shade - 0.5).abs() < 1e-6),
            "the down face shades 0.5"
        );
        assert_eq!(
            quads.iter().filter(|q| q.texture.ends_with("pumpkin_side")).count(),
            3,
            "the three plain sides"
        );
        // THE HEAD'S PITCH the block rides (the head part's own
        // rotation — looking down swings the block forward and drops it)
        let down = AvatarState::rest(180.0, 80.0, 0.0);
        let dq = head_block_quads(&down, Vec3::ZERO, 0.0, 0.0, "block/carved_pumpkin")
            .expect("the pitched pose speaks");
        let mut z_sum = 0.0f32;
        let mut y_sum = 0.0f32;
        let mut n = 0.0f32;
        for q in &dq {
            for p in q.pts.iter() {
                z_sum += p[2];
                y_sum += p[1];
                n += 1.0;
            }
        }
        assert!(z_sum / n < -0.15, "the block swung to the face side: {z_sum}");
        assert!(y_sum / n < 1.7, "the block dropped with the head: {y_sum}");
        // THE ELEMENT-LESS REFUSAL: the skull template the family's
        // items speak carries no elements — the lane honestly None
        // (the SkullBlockRenderer's special models the follow-up keeps)
        assert!(head_block_quads(&rest, Vec3::ZERO, 0.0, 0.0, "item/template_skull").is_none());
        assert!(head_block_quads(&rest, Vec3::ZERO, 0.0, 0.0, "block/no_such_model").is_none());
    }

    /// THE WORN-SKULL CHAIN (the CustomHeadLayer wornHeadType branch's
    /// own law): the skeleton skull's box EXACTLY the head's own box
    /// x1.1875 (the 0.09375 inflation all around), the mob's own entity
    /// skin the texture speaks, the non-skull items the verdict
    /// refuses, the piglin's ears and the dragon's jaw the part-tree
    /// poses ride, the zombie/player hats the second cube speaks.
    #[test]
    fn the_worn_skull_rides_the_1_1875_chain() {
        let rest = AvatarState::rest(180.0, 0.0, 0.0);
        let id = |name: &str| mc_registry::item_id_by_name(name).expect("the registry speaks");
        let bbox = |quads: &[HeadBlockQuad]| -> ([f32; 3], [f32; 3]) {
            let mut lo = [f32::MAX; 3];
            let mut hi = [f32::MIN; 3];
            for q in quads {
                for p in q.pts.iter() {
                    for k in 0..3 {
                        lo[k] = lo[k].min(p[k]);
                        hi[k] = hi[k].max(p[k]);
                    }
                }
            }
            (lo, hi)
        };
        // THE SKELETON SKULL: one cube, six faces, the exact 1.1875 box
        let quads = skull_quads(&rest, Vec3::ZERO, 0.0, 0.0, id("skeleton_skull"))
            .expect("the skeleton skull speaks");
        assert_eq!(quads.len(), 6);
        let (lo, hi) = bbox(&quads);
        assert!((lo[0] + 0.296875).abs() < 1e-4, "lo.x = {}", lo[0]);
        assert!((hi[0] - 0.296875).abs() < 1e-4, "hi.x = {}", hi[0]);
        assert!((lo[2] + 0.296875).abs() < 1e-4, "lo.z = {}", lo[2]);
        assert!((lo[1] - 1.501).abs() < 1e-4, "lo.y = {}", lo[1]);
        assert!((hi[1] - 2.09475).abs() < 1e-4, "hi.y = {}", hi[1]);
        assert!(quads.iter().all(|q| q.texture == "entity/skeleton/skeleton"));
        // the shade rides the entity diffuse (0.5..1.0, never the block's 0.5 flat)
        assert!(quads.iter().all(|q| q.shade > 0.4 && q.shade <= 1.0));
        // THE NON-SKULL REFUSAL (the pumpkin rides its own lane)
        assert!(skull_quads(&rest, Vec3::ZERO, 0.0, 0.0, id("carved_pumpkin")).is_none());
        // THE FAMILY'S SEVEN (each speaks its own skin)
        let expect = [
            ("wither_skeleton_skull", "entity/skeleton/wither_skeleton", 6),
            ("zombie_head", "entity/zombie/zombie", 12),
            ("player_head", "entity/player/slim/steve", 12),
            ("creeper_head", "entity/creeper/creeper", 6),
        ];
        for (name, tex, count) in expect {
            let quads = skull_quads(&rest, Vec3::ZERO, 0.0, 0.0, id(name))
                .unwrap_or_else(|| panic!("{name} speaks"));
            assert_eq!(quads.len(), count, "{name}'s cube walk");
            assert!(quads.iter().all(|q| q.texture == tex), "{name}'s skin");
        }
        // THE PIGLIN'S EARS (the part poses +-4.5/-6 the composition
        // rides — six cubes, the box wider than the bare skull's)
        let pig = skull_quads(&rest, Vec3::ZERO, 0.0, 0.0, id("piglin_head"))
            .expect("the piglin speaks");
        assert_eq!(pig.len(), 36);
        let (_plo, phi) = bbox(&pig);
        assert!(phi[0] > 0.33, "the ears widen the box: {}", phi[0]);
        // THE DRAGON'S JAW (the head pose + the jaw part — the snout
        // juts the face side past a block's own length)
        let dragon = skull_quads(&rest, Vec3::ZERO, 0.0, 0.0, id("dragon_head"))
            .expect("the dragon speaks");
        assert_eq!(dragon.len(), 42);
        let (dlo, _) = bbox(&dragon);
        assert!(dlo[2] < -1.2, "the snout juts: {}", dlo[2]);
    }

    /// THE HELD-ITEM CHAIN (the ItemInHandLayer.submitArmWithItem law
    /// the evidence walk pinned): the stone's block bake through the
    /// block/block stanza's thirdperson_righthand ([75,45,0] x0.375 +
    /// [0, 2.5/16, 0]) rides the ARM chain — the exact box the oracle
    /// walk speaks (the hand's own neighborhood: x 0.202..0.794 the
    /// fold's mirror puts the mannequin's right arm on the world +x
    /// side, y 0.531..1.021 the hanging hand's band, z -0.586..0.023
    /// in front of the body), the block face shade set the quads' own
    /// directions keep, the head's pitch the item does NOT ride (the
    /// arm chain, not the head chain — the head-block lane's own
    /// contrast), the flat sword's generated bake the same chain
    /// carries, and the unknown model the honest None.
    #[test]
    fn the_held_item_rides_the_third_person_chain() {
        let rest = AvatarState::rest(180.0, 0.0, 0.0);
        let quads = held_item_quads(&rest, Vec3::ZERO, 0.0, 0.0, "block/stone")
            .expect("the stone's block bake speaks");
        assert_eq!(quads.len(), 6, "the cube bake's six faces");
        let mut lo = [f32::MAX; 3];
        let mut hi = [f32::MIN; 3];
        for q in &quads {
            for p in q.pts.iter() {
                for k in 0..3 {
                    lo[k] = lo[k].min(p[k]);
                    hi[k] = hi[k].max(p[k]);
                }
            }
        }
        // the oracle walk's exact box (the composite the chain speaks)
        assert!((lo[0] - 0.20206).abs() < 1e-4, "lo.x = {}", lo[0]);
        assert!((lo[1] - 0.53111).abs() < 1e-4, "lo.y = {}", lo[1]);
        assert!((lo[2] + 0.58591).abs() < 1e-4, "lo.z = {}", lo[2]);
        assert!((hi[0] - 0.79378).abs() < 1e-4, "hi.x = {}", hi[0]);
        assert!((hi[1] - 1.02064).abs() < 1e-4, "hi.y = {}", hi[1]);
        assert!((hi[2] - 0.02341).abs() < 1e-4, "hi.z = {}", hi[2]);
        // the block face shade set the quads' own directions keep (the
        // up 1.0, the down 0.5, the sides 0.8/0.6 — the stone's own
        // texture everywhere)
        assert!(quads.iter().all(|q| q.texture.ends_with("block/stone")));
        for want in [1.0f32, 0.5, 0.8, 0.6] {
            assert!(
                quads.iter().any(|q| (q.shade - want).abs() < 1e-6),
                "the shade {want} rides the set"
            );
        }
        // THE HEAD'S PITCH the item does NOT ride (the arm chain, not
        // the head chain): the pitched pose's box EXACTLY the rest's
        let down = AvatarState::rest(180.0, 80.0, 0.0);
        let dq = held_item_quads(&down, Vec3::ZERO, 0.0, 0.0, "block/stone")
            .expect("the pitched pose speaks");
        let mut dlo = [f32::MAX; 3];
        let mut dhi = [f32::MIN; 3];
        for q in &dq {
            for p in q.pts.iter() {
                for k in 0..3 {
                    dlo[k] = dlo[k].min(p[k]);
                    dhi[k] = dhi[k].max(p[k]);
                }
            }
        }
        assert_eq!(lo, dlo, "the pitch never swings the held item");
        assert_eq!(hi, dhi, "the pitch never swings the held item");
        // THE FLAT SWORD (the item/generated bake the handheld stanza's
        // own thirdperson [0,-90,55] x0.85 carries): quads speak, the
        // texture names the sword
        let sword = held_item_quads(&rest, Vec3::ZERO, 0.0, 0.0, "item/iron_sword")
            .expect("the sword's bake speaks");
        assert!(!sword.is_empty(), "the extrusion's quads");
        assert!(
            sword.iter().all(|q| q.texture.ends_with("item/iron_sword")),
            "the sword's own layer texture"
        );
        // THE UNKNOWN MODEL'S HONEST NONE
        assert!(held_item_quads(&rest, Vec3::ZERO, 0.0, 0.0, "item/no_such_model").is_none());
    }

    /// THE ZOMBIE'S OWN HAND (the mob-held weapon lane the render round
    /// wires): the spawn roll's word the registry walk names, the SAME
    /// third-person chain the player's lane speaks — the aggressive
    /// arms-up pose the weapon follows (the animateZombieArms law the
    /// right arm raises, the item rides the posed arm chain), the world
    /// feet the fold rides (the box the feet translate shifts by the
    /// exact offset), the spear's own in-hand model the layered def
    /// answers (the 26.2's own item the RESOLVED_ITEM_LAYERS row
    /// speaks — item/iron_spear_in_hand, the item/spear_in_hand
    /// display json the stanza carries).
    #[test]
    fn the_zombie_held_weapon_rides_the_arm_and_the_feet() {
        let mut rest = AvatarState::rest(180.0, 0.0, 0.0);
        rest.zombie = true;
        let box_of =
            |qs: &Vec<HeadBlockQuad>| -> ([f32; 3], [f32; 3]) {
                let mut lo = [f32::MAX; 3];
                let mut hi = [f32::MIN; 3];
                for q in qs {
                    for p in q.pts.iter() {
                        for k in 0..3 {
                            lo[k] = lo[k].min(p[k]);
                            hi[k] = hi[k].max(p[k]);
                        }
                    }
                }
                (lo, hi)
            };
        let quads = held_item_quads(&rest, Vec3::ZERO, 0.0, 0.0, "item/iron_sword")
            .expect("the zombie's sword bake speaks");
        assert!(!quads.is_empty(), "the extrusion's quads");
        assert!(
            quads.iter().all(|q| q.texture.ends_with("item/iron_sword")),
            "the sword's own layer texture"
        );
        let rest_box = box_of(&quads);
        // THE AGGRESSIVE RAISE the weapon follows: the arms-up pose the
        // rightArm matrix speaks — the weapon's box MOVES (the rest and
        // the aggressive the same walk never keeps)
        let mut aggro = rest;
        aggro.zombie_aggressive = true;
        let aq = held_item_quads(&aggro, Vec3::ZERO, 0.0, 0.0, "item/iron_sword")
            .expect("the aggressive bake speaks");
        let aggro_box = box_of(&aq);
        assert_ne!(rest_box, aggro_box, "the raised arm the weapon rides");
        // THE FEET FOLD the world rides: the standing zombie's box the
        // (5, 64, -3) offset shifts by the EXACT feet (the entity
        // chain's own translate the fold speaks)
        let moved = held_item_quads(&rest, Vec3::new(5.0, 64.0, -3.0), 0.0, 0.0, "item/iron_sword")
            .expect("the moved bake speaks");
        let (mlo, mhi) = box_of(&moved);
        let feet = [5.0f32, 64.0, -3.0];
        for k in 0..3 {
            assert!(
                (mlo[k] - (rest_box.0[k] + feet[k])).abs() < 1e-4,
                "lo[{k}] the feet's own shift keeps"
            );
            assert!(
                (mhi[k] - (rest_box.1[k] + feet[k])).abs() < 1e-4,
                "hi[{k}] the feet's own shift keeps"
            );
        }
        // THE SPEAR the layered def answers: the in-hand model's bake
        // speaks (the item/spear_in_hand stanza's thirdperson the walk
        // resolves — the 26.2's own rotation/scale/translation the
        // quads wear), the texture the spear's own layer names
        let spear = held_item_quads(&rest, Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
            .expect("the spear's in-hand bake speaks");
        assert!(!spear.is_empty(), "the spear's quads");
        assert!(
            spear.iter().all(|q| q.texture.ends_with("item/iron_spear_in_hand")),
            "the spear's own layer texture"
        );
        assert_ne!(
            box_of(&spear),
            rest_box,
            "the spear's own stanza the sword's box never shares"
        );
    }

    /// THE ARROW MODEL's own flat cross (javap `ArrowModel.
    /// createBodyLayer` + `ArrowRenderer.submit` the 26.2 bytecode
    /// speaks): the three parts' quads the 32x32 texture unwraps, the
    /// texScale (1, 0.8) the cross rects' v divides (the 4-texel face
    /// samples 5 rows), the back's own 5x5 the (1,1) default keeps,
    /// the load's sha1 gate the missing file the None keeps.
    #[test]
    fn the_arrow_model_the_flat_cross_speaks() {
        assert!(
            ArrowModel::load(std::path::Path::new("no-such-root-for-the-test")).is_none(),
            "the missing file the honest None keeps"
        );
        let model = ArrowModel {
            rgba: vec![9; 32 * 32 * 4],
        };
        // the rest pose: the identity rotations the flat cross holds
        let quads = model.quads(
            Vec3::ZERO,
            90.0,
            0.0,
            0.0,
            0.0,
            0.0,
        );
        // the 6-face unwrap the three parts keep (the degenerate
        // zero-area faces the table never skips — the raster the
        // empty span never paints)
        assert_eq!(quads.len(), 18, "the three parts' six faces each");
        // THE CROSS's NORTH face (the front): the rect (0,0,16,4) the
        // texScale (1, 0.8) divides — the v corners the 0..5 span
        // keeps (the sampled rows the division answers)
        let north = &quads[4 * 3 + 3]; // part 1 (cross_1), face index 3 (NORTH)
        let vs: Vec<f32> = north.uv.iter().map(|uv| uv[1]).collect();
        assert!(
            vs.iter().all(|v| (*v - 0.0).abs() < 1e-4 || (*v - 5.0).abs() < 1e-4),
            "the texScale's own division the v rows keep: {vs:?}"
        );
        let us: Vec<f32> = north.uv.iter().map(|uv| uv[0]).collect();
        assert!(
            us.iter().all(|u| (*u - 0.0).abs() < 1e-4 || (*u - 16.0).abs() < 1e-4),
            "the cross front's u span the rect keeps: {us:?}"
        );
        // THE BACK's WEST face: the rect (0,5,5,5) the (1,1) default
        // keeps — the sampled span the plain rect answers
        let west = &quads[2]; // part 0 (back), face index 2 (WEST)
        let us: Vec<f32> = west.uv.iter().map(|uv| uv[0]).collect();
        let vs: Vec<f32> = west.uv.iter().map(|uv| uv[1]).collect();
        assert!(
            us.iter().all(|u| (*u - 0.0).abs() < 1e-4 || (*u - 5.0).abs() < 1e-4),
            "the back's west u span keeps: {us:?}"
        );
        assert!(
            vs.iter().all(|v| (*v - 5.0).abs() < 1e-4 || (*v - 10.0).abs() < 1e-4),
            "the back's west v span keeps: {vs:?}"
        );
        // THE CROSS's SOUTH face: the rect (16,0,16,4) the texScale
        // divides — the u the 16..32 span keeps
        let south = &quads[4 * 3 + 5]; // part 1 (cross_1), face index 5 (SOUTH)
        let us: Vec<f32> = south.uv.iter().map(|uv| uv[0]).collect();
        assert!(
            us.iter().all(|u| (*u - 16.0).abs() < 1e-4 || (*u - 32.0).abs() < 1e-4),
            "the cross south's u span keeps: {us:?}"
        );
        // the pixel read: the 32x32 the clamp keeps
        assert_eq!(model.pixel(0.0, 31.9), [9u8, 9, 9, 9]);
    }

    /// THE SKELETON'S OWN MESH (javap `SkeletonModel.
    /// createDefaultSkeletonMesh` the 26.2 bytecode speaks): the thin
    /// 2x12x2 limbs the (40,16)/(0,16) rows unwrap — the LEFT pair the
    /// builder's own .mirror() the u flip rides — the leg pivots the
    /// ±2 PartPose.offset keeps (the wide ±1.9 never shares), the
    /// sleeve/pants children the addOrReplaceChild REPLACE never
    /// fills, the wide limbs the humanoid's own rows the same builder
    /// keeps. THE AIM POSE (isAggressive && isHoldingBow — the
    /// BOW_AND_ARROW the poseRightArm case-3 body assigns): the xRot
    /// -π/2 + head.xRot, the yRot the ∓0.1 + head.yRot the +0.4 left
    /// rides. THE AGGRESSIVE BARE (isAggressive && !isHoldingBow — the
    /// SkeletonModel's OWN branch): at attack_time 0 / age 0 the f1,
    /// f2 and the bob's sin terms all zero — the xRot -π/2 the raise
    /// keeps, the zRot the bobArms' cos term the +1/−1 signs split,
    /// the attackTime the f1/f2 the raise deepens. THE 64x32 SKIN: the
    /// load's sha1 gate the missing file the None keeps, the pixel's
    /// own height clamp the v-beyond-32 the half row keeps.
    #[test]
    fn the_skeleton_mesh_the_thin_laws_speak() {
        let mut sk = AvatarState::rest(180.0, 0.0, 0.0);
        sk.skeleton = true;
        let parts = build_posed_parts(&sk);
        // parts order: head, body, right arm, left arm, right leg, left leg
        assert_eq!(parts.len(), 6, "the thin mesh the same six parts keeps");
        let ra = &parts[2];
        let la = &parts[3];
        let rl = &parts[4];
        let ll = &parts[5];
        // the arms: 2x12x2 at (40,16) — the LEFT the .mirror() the u
        // flip rides — the pivot x ±5 the humanoid shares, NO children
        assert_eq!(ra.cubes[0].tex, (40.0, 16.0));
        assert_eq!(ra.cubes[0].origin, [-1.0, -2.0, -1.0]);
        assert_eq!(ra.cubes[0].size, [2.0, 12.0, 2.0]);
        assert!(!ra.cubes[0].mirror);
        assert!(ra.children.is_empty(), "the thin limb the sleeve never fills");
        assert_eq!(ra.pivot[0], -5.0);
        assert_eq!(la.cubes[0].tex, (40.0, 16.0));
        assert!(la.cubes[0].mirror, "the LEFT arm the builder's own .mirror() speaks");
        assert_eq!(la.pivot[0], 5.0);
        assert!(la.children.is_empty());
        // the legs: 2x12x2 at (0,16), the pivots ±2 the PartPose.offset
        // speaks, the LEFT the same mirror law rides
        assert_eq!(rl.cubes[0].tex, (0.0, 16.0));
        assert_eq!(rl.cubes[0].origin, [-1.0, 0.0, -1.0]);
        assert_eq!(rl.cubes[0].size, [2.0, 12.0, 2.0]);
        assert_eq!(rl.pivot, [-2.0, 12.0, 0.0], "the thin leg pivot the ±2 keeps");
        assert!(rl.children.is_empty());
        assert_eq!(ll.pivot, [2.0, 12.0, 0.0]);
        assert!(ll.cubes[0].mirror, "the LEFT leg the builder's own .mirror() speaks");
        // the WIDE humanoid the same builder never shares (the ±1.9
        // pivots, the 4x12x4 limbs, the left pair the own rects unwrap)
        let wide = build_posed_parts(&AvatarState::rest(180.0, 0.0, 0.0));
        assert_eq!(wide[4].pivot[0], -1.9);
        assert_eq!(wide[2].cubes[0].size, [4.0, 12.0, 4.0]);
        assert!(!wide[5].cubes[0].mirror, "the wide left leg the own rect unwraps");
        assert!(!wide[5].children.is_empty(), "the wide limb the pants child keeps");

        // THE AIM POSE (isAggressive && isHoldingBow): the head terms
        // the pitch/yaw-offset the case-3 body adds
        let mut aim = AvatarState::rest(180.0, 30.0, 0.0);
        aim.skeleton = true;
        aim.skeleton_aggressive = true;
        aim.skeleton_bow = true;
        aim.head_yaw_off_deg = 20.0;
        let parts = build_posed_parts(&aim);
        let hx = 30.0f32.to_radians();
        let hy = 20.0f32.to_radians();
        assert!((parts[2].x_rot - (-1.5707964 + hx)).abs() < 1e-6);
        assert!((parts[2].y_rot - (-0.1 + hy)).abs() < 1e-6);
        assert!((parts[3].x_rot - (-1.5707964 + hx)).abs() < 1e-6);
        assert!(
            (parts[3].y_rot - (0.1 + hy + 0.4)).abs() < 1e-6,
            "the +0.4 the left arm's own yRot keeps"
        );

        // THE AGGRESSIVE BARE (isAggressive && !isHoldingBow): the
        // zero-clock the xRot -π/2, the yRot the ∓0.1, the zRot the
        // bobArms' cos term the +1/−1 signs split
        let mut bare = AvatarState::rest(180.0, 0.0, 0.0);
        bare.skeleton = true;
        bare.skeleton_aggressive = true;
        let parts = build_posed_parts(&bare);
        assert!((parts[2].x_rot - -1.5707964).abs() < 1e-6);
        assert!((parts[2].y_rot - -0.1).abs() < 1e-6);
        assert!((parts[3].y_rot - 0.1).abs() < 1e-6);
        assert!((parts[2].z_rot - 0.1).abs() < 1e-6, "the bobArms the +1 right rides");
        assert!((parts[3].z_rot - -0.1).abs() < 1e-6, "the bobArms the −1 left rides");
        // the swing the attackTime rides: the f1/f2 the raise deepens
        let mut shooting = bare;
        shooting.attack_time = 0.5;
        let parts = build_posed_parts(&shooting);
        assert!(parts[2].x_rot < -1.5707964, "the f1/f2 the raise deepens");

        // THE 64x32 SKIN (the load's sha1 gate + the pixel's own
        // height clamp): the missing root the None keeps, the
        // in-memory half-height texture the v-beyond-32 the 31 row
        // clamps (no panic, no drift)
        assert!(
            AvatarModel::load_skeleton(std::path::Path::new("no-such-root-for-the-test"))
                .is_none(),
            "the missing file the honest None keeps"
        );
        let model = AvatarModel {
            rgba: vec![7; 64 * 32 * 4],
            side_v: 32,
        };
        let px = model.pixel(0.0, 31.9);
        let clamped = model.pixel(0.0, 63.9);
        assert_eq!(px, clamped, "the v beyond 32 the half-height row clamps");
        // the bake: the thin body the quads speak
        let quads = model.quads(&sk, Vec3::ZERO, 0.0, 0.0);
        assert!(!quads.is_empty(), "the thin mesh bakes");
    }

    /// THE ZOMBIE'S OWN ARMOR (the HumanoidArmorLayer the mob render
    /// wears — the store's armor rows the render lane's walk draws):
    /// the four slots' meshes the armor_parts fold speaks, the helmet
    /// the head rides, the chestplate the body + arms the CHEST set
    /// keeps, the aggressive raise the ARMS' armor follows (the
    /// animateZombieArms the posed matrix the pieces wear), the feet
    /// the same world fold rides, the LEGS slot the leggings inflate
    /// the inner deformation keeps.
    #[test]
    fn the_zombie_armor_rides_the_pose_and_the_feet() {
        let mut rest = AvatarState::rest(180.0, 0.0, 0.0);
        rest.zombie = true;
        let box_of = |qs: &Vec<AvatarQuad>| -> ([f32; 3], [f32; 3]) {
            let mut lo = [f32::MAX; 3];
            let mut hi = [f32::MIN; 3];
            for q in qs {
                for p in q.pts.iter() {
                    for k in 0..3 {
                        lo[k] = lo[k].min(p[k]);
                        hi[k] = hi[k].max(p[k]);
                    }
                }
            }
            (lo, hi)
        };
        // the FOUR slots' meshes non-empty (the ADULT_ARMOR_PARTS_PER_SLOT
        // the <clinit>'s own Set.of walk keeps: HEAD {head}, CHEST
        // {body, right_arm, left_arm}, LEGS {body, legs}, FEET {legs})
        let head = armor_quads(&rest, Vec3::ZERO, 0.0, 0.0, ARMOR_SLOT_HEAD);
        let chest = armor_quads(&rest, Vec3::ZERO, 0.0, 0.0, ARMOR_SLOT_CHEST);
        let legs = armor_quads(&rest, Vec3::ZERO, 0.0, 0.0, ARMOR_SLOT_LEGS);
        let feet = armor_quads(&rest, Vec3::ZERO, 0.0, 0.0, ARMOR_SLOT_FEET);
        assert!(!head.is_empty(), "the helmet's mesh");
        assert!(!chest.is_empty(), "the chestplate's mesh");
        assert!(!legs.is_empty(), "the leggings' mesh");
        assert!(!feet.is_empty(), "the boots' mesh");
        // THE AGGRESSIVE RAISE the arms' armor follows: the chestplate
        // rides the body + arms (the CHEST set the posed matrices
        // speak) — the arms-up pose the piece's box MOVES; the helmet
        // the head's own pose the raise never touches (the head stays)
        let mut aggro = rest;
        aggro.zombie_aggressive = true;
        let aggro_chest = armor_quads(&aggro, Vec3::ZERO, 0.0, 0.0, ARMOR_SLOT_CHEST);
        let aggro_head = armor_quads(&aggro, Vec3::ZERO, 0.0, 0.0, ARMOR_SLOT_HEAD);
        assert_ne!(
            box_of(&chest),
            box_of(&aggro_chest),
            "the raised arms the chestplate rides"
        );
        assert_eq!(
            box_of(&head),
            box_of(&aggro_head),
            "the raise the helmet never moves"
        );
        // THE FEET FOLD the world rides: the standing zombie's helmet
        // box the (5, 64, -3) offset shifts by the EXACT feet (the
        // entity chain's own translate the fold speaks)
        let moved = armor_quads(&rest, Vec3::new(5.0, 64.0, -3.0), 0.0, 0.0, ARMOR_SLOT_HEAD);
        let (mlo, mhi) = box_of(&moved);
        let (rlo, rhi) = box_of(&head);
        let feet_shift = [5.0f32, 64.0, -3.0];
        for k in 0..3 {
            assert!(
                (mlo[k] - (rlo[k] + feet_shift[k])).abs() < 1e-4,
                "lo[{k}] the feet's own shift keeps"
            );
            assert!(
                (mhi[k] - (rhi[k] + feet_shift[k])).abs() < 1e-4,
                "hi[{k}] the feet's own shift keeps"
            );
        }
        // THE INFLATE LAW the slots speak: the LEGS mesh the INNER
        // deformation 0.5 keeps (the usesInnerModel verdict), the
        // helmet the OUTER 1.0 — the leggings' box the helmet's own
        // never shares (the deformation the meshes' own widths speak)
        let (_, lhi) = box_of(&legs);
        let (_, hhi) = box_of(&head);
        assert_ne!(lhi, hhi, "the inflate the slots' own boxes speak");
    }

    /// THE ZOMBIE'S OWN LEFT HAND (the offhand lane the mob render
    /// wears — ItemInHandLayer's SECOND submitArmWithItem the LEFT arm
    /// renders): the same item both hands the x-mirror the sign
    /// branch keeps (the world -x side the mainhand's +x box the
    /// mirror pairs), the aggressive raise the LEFT arm the weapon
    /// follows (the animateZombieArms the BOTH arms the inverted bob
    /// speaks), the feet the same world fold rides.
    #[test]
    fn the_zombie_offhand_rides_the_left_arm() {
        let mut rest = AvatarState::rest(180.0, 0.0, 0.0);
        rest.zombie = true;
        let box_of = |qs: &Vec<HeadBlockQuad>| -> ([f32; 3], [f32; 3]) {
            let mut lo = [f32::MAX; 3];
            let mut hi = [f32::MIN; 3];
            for q in qs {
                for p in q.pts.iter() {
                    for k in 0..3 {
                        lo[k] = lo[k].min(p[k]);
                        hi[k] = hi[k].max(p[k]);
                    }
                }
            }
            (lo, hi)
        };
        // the SAME item both hands: the boxes the mirror pairs
        let main = held_item_quads(&rest, Vec3::ZERO, 0.0, 0.0, "item/iron_sword")
            .expect("the mainhand's bake speaks");
        let off = offhand_item_quads(&rest, Vec3::ZERO, 0.0, 0.0, "item/iron_sword")
            .expect("the offhand's bake speaks");
        let (mlo, mhi) = box_of(&main);
        let (olo, ohi) = box_of(&off);
        // the x-mirror the sign branch keeps (the left arm the -1/16
        // the mainhand's +1/16 the world mirror answers)
        assert!(
            (ohi[0] + mlo[0]).abs() < 1e-3,
            "the offhand's +x edge the mainhand's -x edge mirrors"
        );
        assert!(
            (olo[0] + mhi[0]).abs() < 1e-3,
            "the offhand's -x edge the mainhand's +x edge mirrors"
        );
        // the y/z bands the arms share (the same translate Y/Z the
        // sign branch never touches)
        assert!(
            (olo[1] - mlo[1]).abs() < 1e-3 && (ohi[1] - mhi[1]).abs() < 1e-3,
            "the y band the branch keeps"
        );
        assert!(
            (olo[2] - mlo[2]).abs() < 1e-3 && (ohi[2] - mhi[2]).abs() < 1e-3,
            "the z band the branch keeps"
        );
        // THE AGGRESSIVE RAISE the LEFT arm follows (the both-arms
        // assignment the zombie walk speaks)
        let mut aggro = rest;
        aggro.zombie_aggressive = true;
        let aggro_off = offhand_item_quads(&aggro, Vec3::ZERO, 0.0, 0.0, "item/iron_sword")
            .expect("the aggressive bake speaks");
        assert_ne!(
            box_of(&aggro_off),
            (olo, ohi),
            "the raised left arm the weapon rides"
        );
        // THE FEET the same world fold rides (the (5,64,-3) the exact
        // shift the mainhand's own test speaks)
        let moved = offhand_item_quads(&rest, Vec3::new(5.0, 64.0, -3.0), 0.0, 0.0, "item/iron_sword")
            .expect("the moved bake speaks");
        let (mvlo, mvhi) = box_of(&moved);
        let feet = [5.0f32, 64.0, -3.0];
        for k in 0..3 {
            assert!((mvlo[k] - (olo[k] + feet[k])).abs() < 1e-4, "lo[{k}] the feet's shift keeps");
            assert!((mvhi[k] - (ohi[k] + feet[k])).abs() < 1e-4, "hi[{k}] the feet's shift keeps");
        }
    }

    /// THE OFFHAND'S LEFT-ARM CHAIN (the oracle walk
    /// `scripts/offhand_item_oracle.py` speaks — the RIGHT chain's
    /// honesty gate reproduces the main-hand pins first, the LEFT
    /// numbers land here): the leftArm part's own matrix the
    /// `getArm(LEFT)` branch answers, the layer's -1/16 the sign
    /// branch keeps (Y/Z untouched), the mirrored stanza application
    /// the left-hand flag speaks. The stone's box lands the WORLD -x
    /// side — the exact x-mirror the main hand's +x box pins (the
    /// block stanza's tx=0 keeps the corner set mirror-symmetric).
    #[test]
    fn the_offhand_item_rides_the_left_arm_chain() {
        let rest = AvatarState::rest(180.0, 0.0, 0.0);
        let quads = offhand_item_quads(&rest, Vec3::ZERO, 0.0, 0.0, "block/stone")
            .expect("the stone's block bake speaks");
        assert_eq!(quads.len(), 6, "the cube bake's six faces");
        let mut lo = [f32::MAX; 3];
        let mut hi = [f32::MIN; 3];
        for q in &quads {
            for p in q.pts.iter() {
                for k in 0..3 {
                    lo[k] = lo[k].min(p[k]);
                    hi[k] = hi[k].max(p[k]);
                }
            }
        }
        // the oracle walk's exact box (the mirrored main-hand box)
        assert!((lo[0] + 0.79378).abs() < 1e-4, "lo.x = {}", lo[0]);
        assert!((lo[1] - 0.53111).abs() < 1e-4, "lo.y = {}", lo[1]);
        assert!((lo[2] + 0.58591).abs() < 1e-4, "lo.z = {}", lo[2]);
        assert!((hi[0] + 0.20206).abs() < 1e-4, "hi.x = {}", hi[0]);
        assert!((hi[1] - 1.02064).abs() < 1e-4, "hi.y = {}", hi[1]);
        assert!((hi[2] - 0.02341).abs() < 1e-4, "hi.z = {}", hi[2]);
        // the offhand's own side: the whole box on the WORLD -x half
        assert!(hi[0] < 0.0, "the offhand rides the -x side (hi.x = {})", hi[0]);
        // the block face shade set the shared law keeps (the same six
        // faces the main hand speaks)
        assert!(quads.iter().all(|q| q.texture.ends_with("block/stone")));
        for want in [1.0f32, 0.5, 0.8, 0.6] {
            assert!(
                quads.iter().any(|q| (q.shade - want).abs() < 1e-6),
                "the shade {want} rides the set"
            );
        }
        // THE FLAT SWORD rides the mirrored handheld stanza too
        let sword = offhand_item_quads(&rest, Vec3::ZERO, 0.0, 0.0, "item/iron_sword")
            .expect("the sword's bake speaks");
        assert!(!sword.is_empty(), "the extrusion's quads");
        assert!(
            sword.iter().all(|q| q.texture.ends_with("item/iron_sword")),
            "the sword's own layer texture"
        );
        // THE UNKNOWN MODEL'S HONEST NONE (the same verdict both hands)
        assert!(offhand_item_quads(&rest, Vec3::ZERO, 0.0, 0.0, "item/no_such_model").is_none());
    }

    /// THE EQUIPMENT TEXTURES LOAD (the runtime cache's own verdict —
    /// the generator's extraction carries the humanoid and
    /// humanoid_leggings layer pngs the sha1 table pins, each the
    /// 64x32 layout the armor UVs speak). The decode's honesty rides
    /// the EXACT opaque-texel count per layer — the png crate's EXPAND
    /// decode must reproduce every layer's alpha mask VERBATIM (the
    /// PIL ground truth the round's own walk spoke): the chainmail's
    /// mesh holes and the leather overlay's trim-only coverage are
    /// REAL vanilla transparency — the former per-texel probe wrongly
    /// demanded every layer opaque at (8,8) and the mesh's holes
    /// refused — the count law replaces it.
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn the_equipment_textures_load_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        use mc_registry::generated::equipment;
        // the EXACT opaque-texel counts the 64x32 layers speak (the
        // RGBA decode's own alpha mask, texel-verbatim)
        const HUMANOID_OPAQUE: &[(&str, usize)] = &[
            ("chainmail", 389),
            ("copper", 648),
            ("diamond", 632),
            ("gold", 648),
            ("iron", 648),
            ("leather", 660),
            ("leather_overlay", 52),
            ("netherite", 699),
            ("turtle_scute", 216),
        ];
        const LEGGINGS_OPAQUE: &[(&str, usize)] = &[
            ("chainmail", 137),
            ("copper", 280),
            ("diamond", 280),
            ("gold", 280),
            ("iron", 280),
            ("leather", 280),
            ("leather_overlay", 108),
            ("netherite", 272),
        ];
        let opaque_count = |t: &ArmorTex| t.rgba.chunks_exact(4).filter(|p| p[3] == 255).count();
        for tex in equipment::EQUIPMENT_ASSETS {
            let humanoid = ArmorTex::load(&root, equipment::HUMANOID_DIR, tex)
                .expect("the humanoid layer loads");
            let want = HUMANOID_OPAQUE
                .iter()
                .find(|(n, _)| *n == tex.name)
                .map(|(_, c)| *c)
                .unwrap_or_else(|| panic!("{} missing from the count table", tex.name));
            assert_eq!(
                opaque_count(&humanoid),
                want,
                "the {} humanoid layer's opaque mask",
                tex.name
            );
            if let Some(_) = tex.leggings {
                let leggings = ArmorTex::load(&root, equipment::LEGGINGS_DIR, tex)
                    .expect("the leggings layer loads");
                let want = LEGGINGS_OPAQUE
                    .iter()
                    .find(|(n, _)| *n == tex.name)
                    .map(|(_, c)| *c)
                    .unwrap_or_else(|| panic!("{} missing from the count table", tex.name));
                assert_eq!(
                    opaque_count(&leggings),
                    want,
                    "the {} leggings layer's opaque mask",
                    tex.name
                );
            }
        }
        // the honest miss: the leggings dir the table never named
        let turtle = equipment::find("turtle_scute").expect("the scute speaks");
        assert!(
            ArmorTex::load(&root, equipment::LEGGINGS_DIR, turtle).is_none(),
            "the turtle scute has no leggings layer — the load refuses"
        );
    }

    /// THE CANINE'S OWN MESH (javap AdultWolfModel.createBodyLayer +
    /// WolfModel.setupAnim the 26.2 bytecode speaks): the ELEVEN cubes
    /// the bake folds (the real_head's skull/ears/muzzle, the body,
    /// the upper_body, the four legs, the real_tail brush), the rest
    /// pose the WILD TAIL ANGLE keeps, the walk the diagonal pairs
    /// swing, the look the head turns, the death the body topples.
    #[test]
    fn the_wolf_mesh_bakes_the_canine_parts() {
        let wolf = WolfModel {
            rgba: vec![255u8; 64 * 32 * 4],
        };
        let feet = Vec3::new(10.0, 64.0, 20.0);
        let rest = WolfState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            death_time: 0.0,
            tail_angle: crate::mob::WOLF_WILD_TAIL_ANGLE,
            sitting: false,
            angry: false,
        };
        let quads = wolf.quads(&rest, feet, 0.0, 0.0);
        assert_eq!(quads.len(), 66, "11 cubes x 6 faces the bake speaks");
        let pose_sum = |qs: &[AvatarQuad]| -> f64 {
            qs.iter()
                .flat_map(|q| q.pts.iter().flatten())
                .map(|v| *v as f64)
                .sum()
        };
        // THE WALK CLOCK (the 0.6661999821662903 fold the legs swing;
        // the diagonal pairs the phase law splits — the sum alone the
        // change wears, the same evidence the spider's walk keeps)
        let walking = WolfState { walk_pos: 1.0, walk_speed: 1.0, ..rest };
        assert_ne!(pose_sum(&quads), pose_sum(&wolf.quads(&walking, feet, 0.0, 0.0)));
        // THE HEAD'S LOOK: the pitch turns the skull
        let looking = WolfState { pitch_deg: 25.0, ..rest };
        assert_ne!(pose_sum(&quads), pose_sum(&wolf.quads(&looking, feet, 0.0, 0.0)));
        // THE DEATH FOLD
        let dying = WolfState { death_time: 5.0, ..rest };
        assert_ne!(pose_sum(&quads), pose_sum(&wolf.quads(&dying, feet, 0.0, 0.0)));
    }

    /// THE CANINE BABY'S OWN MESH (javap BabyWolfModel.createBodyLayer
    /// the 32x32 layer the byte dims keep): the TEN cubes the bake
    /// folds (the deform skull + the muzzle, the two child ears, the
    /// body, the four stub legs, the curled tail_r1), the INHERITED
    /// walk law the same swing wears (the BabyWolfModel extends
    /// WolfModel — no override the class keeps).
    #[test]
    fn the_baby_wolf_mesh_bakes_the_stub_parts() {
        let baby = BabyWolfModel {
            rgba: vec![255u8; 32 * 32 * 4],
        };
        let feet = Vec3::new(10.0, 64.0, 20.0);
        let rest = WolfState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            death_time: 0.0,
            tail_angle: crate::mob::WOLF_WILD_TAIL_ANGLE,
            sitting: false,
            angry: false,
        };
        let quads = baby.quads(&rest, feet, 0.0, 0.0);
        assert_eq!(quads.len(), 60, "10 cubes x 6 faces the bake speaks");
        let pose_sum = |qs: &[AvatarQuad]| -> f64 {
            qs.iter()
                .flat_map(|q| q.pts.iter().flatten())
                .map(|v| *v as f64)
                .sum()
        };
        let walking = WolfState { walk_pos: 1.0, walk_speed: 1.0, ..rest };
        assert_ne!(pose_sum(&quads), pose_sum(&baby.quads(&walking, feet, 0.0, 0.0)));
        let looking = WolfState { head_yaw_off_deg: 40.0, ..rest };
        assert_ne!(pose_sum(&quads), pose_sum(&baby.quads(&looking, feet, 0.0, 0.0)));
    }

    /// THE WOLF MODELS LOAD — the real runtime cache (the generator's
    /// extraction) decodes the 64x32 adult + the 32x32 baby the shas
    /// the generated table pins (the WolfRenderer's getTextureLocation
    /// the pale variant's own words).
    #[test]
    fn the_wolf_models_load_sha1_verified() {
        let mut dir = std::env::current_dir().unwrap();
        let root = loop {
            let cand = dir.join("assets-cache/jar");
            if cand.exists() {
                break cand;
            }
            dir = dir.parent().expect("the runtime root above").to_path_buf();
        };
        let wolf = WolfModel::load(&root).expect("the wolf loads from the runtime cache");
        let mut opaque = 0;
        for y in 0..32 {
            for x in 0..64 {
                if wolf.pixel(x as f32, y as f32)[3] == 255 {
                    opaque += 1;
                }
            }
        }
        assert!(opaque > 400, "the hide's texels the canvas keeps: {opaque}");
        let baby = BabyWolfModel::load(&root).expect("the baby loads from the runtime cache");
        let mut baby_opaque = 0;
        for y in 0..32 {
            for x in 0..32 {
                if baby.pixel(x as f32, y as f32)[3] == 255 {
                    baby_opaque += 1;
                }
            }
        }
        assert!(baby_opaque > 100, "the baby hide's texels keep: {baby_opaque}");
    }

    /// THE HAND-USE BASE'S OWN WALK (javap `thirdPersonHandUse` pcs
    /// 11-108, scripts/pins_480.py the f32 chain oracle): the walk-zero
    /// arm's rest raise — the -0.1 side nudge, the TWO xRot fadds, the
    /// degree clamps the round trip re-speaks (the raw sum the bits the
    /// round-trip moves), the left arm's mirror, the clamps' bite at
    /// the ±60°/30° windows, the fall-flying/swim extra the branch
    /// rides.
    #[test]
    fn the_third_person_hand_use_the_base_raise_the_clamps_speak() {
        // THE WALK-ZERO REST (right): x bits 0xbf4552e8, y bits
        // 0xbdcccccc, the zRot the fold never touches
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        let mut z = 7.5f32; // the sentinel the untouched law keeps
        third_person_hand_use(&mut x, &mut y, &mut z, true, false, 0.0, 0.0, false, true, None);
        assert_eq!(x.to_bits(), 0xbf45_52e8, "the right base xRot: {x}");
        assert_eq!(y.to_bits(), 0xbdcc_cccc, "the right base yRot: {y}");
        assert_eq!(z.to_bits(), 7.5f32.to_bits(), "the zRot the base never folds");
        // THE DEGREES ROUND-TRIP: the raw sum the clamp's own degree
        // walk moves (the f32 the 57.2957764/0.0174533 pair re-rounds)
        let raw = 0.0f32 + -1.570_796_370_506_286_6 + 0.800_000_011_920_929;
        assert_ne!(x.to_bits(), raw.to_bits(), "the round-trip the bits wear");
        // THE LEFT MIRROR: the yRot the -side flips, the xRot the side
        // never touches
        let mut lx = 0.0f32;
        let mut ly = 0.0f32;
        let mut lz = 0.0f32;
        third_person_hand_use(&mut lx, &mut ly, &mut lz, false, false, 0.0, 0.0, false, true, None);
        assert_eq!(lx.to_bits(), 0xbf45_52e8, "the left base xRot: {lx}");
        assert_eq!(ly.to_bits(), 0x3dcc_cccc, "the left base yRot: {ly}");
        // THE CLAMPS' BITE (x0 = 2.0, y0 = 1.5): the 30°/60° windows the
        // values the oracle speaks
        let mut bx = 2.0f32;
        let mut by = 1.5f32;
        let mut bz = 0.0f32;
        third_person_hand_use(&mut bx, &mut by, &mut bz, true, false, 0.0, 0.0, false, true, None);
        assert_eq!(bx.to_bits(), 0x3f06_0a92, "the xRot's 30° ceiling: {bx}");
        assert_eq!(by.to_bits(), 0x3f86_0a92, "the yRot's 60° ceiling: {by}");
        // THE FALL-FLYING EXTRA (and the swim blend the same branch rides)
        let mut fx = 0.0f32;
        let mut fy = 0.0f32;
        let mut fz = 0.0f32;
        third_person_hand_use(&mut fx, &mut fy, &mut fz, true, true, 0.0, 0.0, false, true, None);
        assert_eq!(fx.to_bits(), 0xbfdd_8879, "the fly extra's xRot: {fx}");
        let mut sx = 0.0f32;
        let mut sy = 0.0f32;
        let mut sz = 0.0f32;
        third_person_hand_use(&mut sx, &mut sy, &mut sz, true, false, 0.5, 0.0, false, true, None);
        assert_eq!(sx.to_bits(), 0xbfdd_8879, "the swim extra's xRot: {sx}");
    }

    /// THE HAND-USE GATE'S OWN MATRIX (javap pcs 111-168): the FIELD
    /// ticks the both-arms clock keeps, the `!isUsingItem ||
    /// useItemHand == hand` disjunction, the kinetic null-return — the
    /// sway the gate alone admits.
    #[test]
    fn the_hand_use_the_gate_the_sway_the_jar_keeps() {
        let params = crate::hand::SpearUseParams {
            raise: 0.5,
            raise_start: 0.25,
            raise_middle: 0.5,
            raise_end: 0.75,
            sway_progress: 0.125,
            lower: 0.0,
            raise_back: 0.375,
            sway_intensity: 0.75,
            sway_scale_slow: 1.25,
            sway_scale_fast: -0.5,
        };
        let base = |right: bool| {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            third_person_hand_use(&mut x, &mut y, &mut z, right, false, 0.0, 0.0, false, true, None);
            (x, y, z)
        };
        // THE SILENCE: no use, no ticks — the params the gate never
        // reaches (the kinetic null-return's twin)
        let (x0, y0, z0) = base(true);
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        third_person_hand_use(
            &mut x, &mut y, &mut z, true, false, 0.0, 0.0, false, true, Some(&params),
        );
        assert_eq!((x.to_bits(), y.to_bits(), z.to_bits()), (x0.to_bits(), y0.to_bits(), z0.to_bits()));
        // THE WRONG HAND (main uses, the LEFT arm): the gate refuses —
        // the LEFT arm's own base the mirror answers
        let (xl0, yl0, zl0) = base(false);
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        third_person_hand_use(
            &mut x, &mut y, &mut z, false, false, 0.0, 1.0, true, true, Some(&params),
        );
        assert_eq!((x.to_bits(), y.to_bits(), z.to_bits()), (xl0.to_bits(), yl0.to_bits(), zl0.to_bits()));
        // THE ZERO TICKS (using, the right hand): the `> 0.0` refuses
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        third_person_hand_use(
            &mut x, &mut y, &mut z, true, false, 0.0, 0.0, true, true, Some(&params),
        );
        assert_eq!((x.to_bits(), y.to_bits(), z.to_bits()), (x0.to_bits(), y0.to_bits(), z0.to_bits()));
        // THE ADMITTED SWAY (using, the right hand, ticks 1): the
        // oracle's own three adds — the xRot the side never mirrors,
        // the y/z the -side flips
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        third_person_hand_use(
            &mut x, &mut y, &mut z, true, false, 0.0, 1.0, true, true, Some(&params),
        );
        assert_eq!(x.to_bits(), 0xbf5e_bc60, "the right sway's xRot: {x}");
        assert_eq!(y.to_bits(), 0xbdbf_6557, "the right sway's yRot: {y}");
        assert_eq!(z.to_bits(), 0xbc06_0a92, "the right sway's zRot: {z}");
        // THE LEFT'S OWN SWAY (using OFF — the use_item_main_hand false
        // the hand mapping admits): the y/z the mirror flips, the xRot
        // the same window wears
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        third_person_hand_use(
            &mut x, &mut y, &mut z, false, false, 0.0, 1.0, true, false, Some(&params),
        );
        assert_eq!(x.to_bits(), 0xbf5e_bc60, "the left sway's xRot: {x}");
        assert_eq!(y.to_bits(), 0x3dbf_6557, "the left sway's yRot: {y}");
        assert_eq!(z.to_bits(), 0x3c06_0a92, "the left sway's zRot: {z}");
        // THE LEFTOVER TICKS (not using, the field ticks ride on): the
        // `!isUsingItem` disjunction admits — the SAME bits the using
        // right-hand case keeps
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        third_person_hand_use(
            &mut x, &mut y, &mut z, true, false, 0.0, 1.0, false, true, Some(&params),
        );
        assert_eq!(x.to_bits(), 0xbf5e_bc60);
        assert_eq!(y.to_bits(), 0xbdbf_6557);
        assert_eq!(z.to_bits(), 0xbc06_0a92);
    }

    /// THE POSE ARMS' OWN BRANCH MATRIX (javap `HumanoidModel.setupAnim`
    /// pcs 375-534, the SPEAR case 10 the only fold): the affectsOffhand
    /// skips the idle and using branches wear, the pose order the
    /// isUsingItem/useItemHand pair drives — build_posed_parts the
    /// parts' own rotations answer.
    #[test]
    fn the_pose_arms_the_branch_matrix_the_spear_rest_wears() {
        let walk = AvatarState::rest(0.0, 0.0, 0.0);
        let plain = build_posed_parts(&walk);
        // THE IDLE BRANCH (not using): the right SPEAR folds the right
        // arm (the base raise), the left the affectsOffhand skip keeps
        // BIT-IDENTICAL — and the plain walk's right the fold moves
        let mut rs = AvatarState::rest(0.0, 0.0, 0.0);
        rs.right_spear = true;
        let posed = build_posed_parts(&rs);
        assert_ne!(
            (posed[2].x_rot.to_bits(), posed[2].y_rot.to_bits()),
            (plain[2].x_rot.to_bits(), plain[2].y_rot.to_bits()),
            "the right spear's raise the walk arm moves"
        );
        assert_eq!(
            (posed[3].x_rot.to_bits(), posed[3].y_rot.to_bits(), posed[3].z_rot.to_bits()),
            (plain[3].x_rot.to_bits(), plain[3].y_rot.to_bits(), plain[3].z_rot.to_bits()),
            "the affectsOffhand skip the left arm keeps"
        );
        // THE LEFT-ONLY IDLE: the left folds (poseLeftArm first), the
        // right the left pose's affectsOffhand CANCELS — the plain
        // walk's own rotations the right keeps
        let mut ls = AvatarState::rest(0.0, 0.0, 0.0);
        ls.left_spear = true;
        let posed = build_posed_parts(&ls);
        assert_ne!(
            (posed[3].x_rot.to_bits(), posed[3].y_rot.to_bits()),
            (plain[3].x_rot.to_bits(), plain[3].y_rot.to_bits()),
            "the left spear's raise the walk arm moves"
        );
        assert_eq!(
            (posed[2].x_rot.to_bits(), posed[2].y_rot.to_bits(), posed[2].z_rot.to_bits()),
            (plain[2].x_rot.to_bits(), plain[2].y_rot.to_bits(), plain[2].z_rot.to_bits()),
            "the left's affectsOffhand the right arm cancels"
        );
        // THE USING BRANCH (main hand): poseRightArm first — the right
        // folds, the left the right's affectsOffhand skip keeps even
        // when the left wears SPEAR too
        let mut us = AvatarState::rest(0.0, 0.0, 0.0);
        us.is_using_item = true;
        us.use_item_main_hand = true;
        us.ticks_using_item = 1.0;
        us.right_spear = true;
        us.left_spear = true;
        us.kinetic_use_params = Some(crate::hand::SpearUseParams {
            raise: 0.5,
            raise_start: 0.25,
            raise_middle: 0.5,
            raise_end: 0.75,
            sway_progress: 0.125,
            lower: 0.0,
            raise_back: 0.375,
            sway_intensity: 0.75,
            sway_scale_slow: 1.25,
            sway_scale_fast: -0.5,
        });
        let posed = build_posed_parts(&us);
        // THE LEFT ARM THE SKIP KEEPS: the right pose's affectsOffhand
        // cancels the poseLeftArm call — the plain walk's own rotations
        // the left bit-identical keeps
        assert_eq!(
            (posed[3].x_rot.to_bits(), posed[3].y_rot.to_bits(), posed[3].z_rot.to_bits()),
            (plain[3].x_rot.to_bits(), plain[3].y_rot.to_bits(), plain[3].z_rot.to_bits()),
            "the using branch the left arm skips (the right's affectsOffhand)"
        );
        // ...and the RIGHT arm the sway admits — the same values the
        // direct fold answers (the wiring the bits re-speak)
        let (mut mx, mut my, mut mz) = (plain[2].x_rot, plain[2].y_rot, plain[2].z_rot);
        third_person_hand_use(
            &mut mx, &mut my, &mut mz, true, false, 0.0, 1.0, true, true,
            us.kinetic_use_params.as_ref(),
        );
        assert_eq!(posed[2].x_rot.to_bits(), mx.to_bits());
        assert_eq!(posed[2].y_rot.to_bits(), my.to_bits());
        assert_eq!(posed[2].z_rot.to_bits(), mz.to_bits());
    }

    /// THE ITEM POSE'S OWN FOLD (javap `HumanoidModel.poseRightArm`
    /// pc 87-116 the case-1 walk, the CP #95/#279 the constants
    /// resolve — poseLeftArm the mirror the same pair wears): the
    /// standing avatar's held stack the half-the-tilt fold wears —
    /// xRot the walk's own 0.0 the −0.31415927 the tilt speaks, the
    /// f32 chain the fmul/fsub keeps, the yRot the zero the walk
    /// replays; the walking avatar the folded walk cosines the bits
    /// re-speak; the crouch's own +0.4 the fold the raised xRot reads.
    #[test]
    fn the_item_pose_the_half_the_tilt_the_bits_speak() {
        let plain = build_posed_parts(&AvatarState::rest(0.0, 0.0, 0.0));
        // THE STANDING RIGHT: 0.0·0.5 − C — the constant's own bits the
        // fsub rounds (the ×0.5 the exact exponent walk, the fsub one)
        let mut s = AvatarState::rest(0.0, 0.0, 0.0);
        s.right_item = true;
        let posed = build_posed_parts(&s);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            (-0.314_159_274_101_257_3_f32).to_bits(),
            "the standing ITEM tilt the constant's own bits wear"
        );
        assert_eq!(posed[2].y_rot.to_bits(), 0.0f32.to_bits());
        assert_eq!(
            posed[2].z_rot.to_bits(),
            plain[2].z_rot.to_bits(),
            "the ITEM fold the walk zRot keeps"
        );
        // THE LEFT ARM THE VERDICT FALSE KEEPS — the walk's own zeroes
        // the EMPTY verdict the bits replay
        assert_eq!(
            (
                posed[3].x_rot.to_bits(),
                posed[3].y_rot.to_bits(),
                posed[3].z_rot.to_bits()
            ),
            (
                plain[3].x_rot.to_bits(),
                plain[3].y_rot.to_bits(),
                plain[3].z_rot.to_bits()
            ),
            "the left arm the EMPTY verdict the walk keeps"
        );
        // THE STANDING LEFT (the mirror — poseLeftArm pc 87-116 the
        // same constants the left lane wears)
        let mut l = AvatarState::rest(0.0, 0.0, 0.0);
        l.left_item = true;
        let posed = build_posed_parts(&l);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            (-0.314_159_274_101_257_3_f32).to_bits(),
            "the left lane the same constant the mirror wears"
        );
        assert_eq!(posed[3].y_rot.to_bits(), 0.0f32.to_bits());
        // THE WALKING VARIANT: the fold the walk+bob xRot reads — the
        // chain the plain part's own x the f32 ops replay
        let mut w = AvatarState::rest(0.0, 0.0, 0.0);
        w.walk_pos = 2.75;
        w.walk_speed = 1.0;
        w.age_ticks = 5.0;
        let walk_plain = build_posed_parts(&w);
        w.right_item = true;
        let walk_item = build_posed_parts(&w);
        let expected = walk_plain[2].x_rot * 0.5 - 0.314_159_274_101_257_3_f32;
        assert_eq!(
            walk_item[2].x_rot.to_bits(),
            expected.to_bits(),
            "the walking fold the walk xRot the chain re-speaks"
        );
        assert_eq!(walk_item[2].y_rot.to_bits(), 0.0f32.to_bits());
        // THE CROUCH: the walk section's own +0.4 the fold reads (the
        // raised xRot the poseArms folds — the jar's own order keeps)
        let mut c = AvatarState::rest(0.0, 0.0, 0.0);
        c.crouching = true;
        c.right_item = true;
        let posed = build_posed_parts(&c);
        let expected = 0.4f32 * 0.5 - 0.314_159_274_101_257_3_f32;
        assert_eq!(
            posed[2].x_rot.to_bits(),
            expected.to_bits(),
            "the crouch's raise the fold the raised xRot reads"
        );
    }

    /// THE ITEM LANE'S OWN GATES (the poseArms matrix the two verdicts
    /// dispatch — pcs 416-514 the branch walk): the using-main branch
    /// the right SPEAR first poses, the left the affectsOffhand skip
    /// the left ITEM keeps; the right ITEM the lane's own fold wears;
    /// the idle branch the left first poses (the left ITEM the fold
    /// wears), the right ITEM the open gate the fold rides; the left
    /// SPEAR the right ITEM the affectsOffhand cancels.
    #[test]
    fn the_item_pose_the_gates_the_matrix_speaks() {
        let plain = build_posed_parts(&AvatarState::rest(0.0, 0.0, 0.0));
        let item_bits = |x: f32| (x * 0.5 - 0.314_159_274_101_257_3_f32).to_bits();
        // LANE A (using main): the right SPEAR the right poses, the
        // left ITEM the affectsOffhand skip the walk keeps
        let mut a = AvatarState::rest(0.0, 0.0, 0.0);
        a.is_using_item = true;
        a.use_item_main_hand = true;
        a.ticks_using_item = 1.0;
        a.right_spear = true;
        a.left_item = true;
        let posed = build_posed_parts(&a);
        assert_eq!(
            (
                posed[3].x_rot.to_bits(),
                posed[3].y_rot.to_bits(),
                posed[3].z_rot.to_bits()
            ),
            (
                plain[3].x_rot.to_bits(),
                plain[3].y_rot.to_bits(),
                plain[3].z_rot.to_bits()
            ),
            "the using branch the left ITEM the skip keeps"
        );
        assert_ne!(
            posed[2].x_rot.to_bits(),
            plain[2].x_rot.to_bits(),
            "the right SPEAR the use lane the fold wears"
        );
        // LANE A the right ITEM (the main stack the spear verdict never
        // answers — the item the fold wears), the left the EMPTY the
        // walk keeps
        let mut b = AvatarState::rest(0.0, 0.0, 0.0);
        b.is_using_item = true;
        b.use_item_main_hand = true;
        b.ticks_using_item = 1.0;
        b.right_item = true;
        let posed = build_posed_parts(&b);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            item_bits(plain[2].x_rot),
            "the using-main right ITEM the fold wears"
        );
        assert_eq!(
            posed[3].x_rot.to_bits(),
            plain[3].x_rot.to_bits(),
            "the left EMPTY the walk keeps"
        );
        // THE IDLE BRANCH: the left ITEM the left-first fold wears, the
        // right ITEM the open gate (the ITEM affectsOffhandPose false
        // the clinit's flags keep) the right fold rides
        let mut c = AvatarState::rest(0.0, 0.0, 0.0);
        c.left_item = true;
        c.right_item = true;
        let posed = build_posed_parts(&c);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            item_bits(plain[3].x_rot),
            "the idle left ITEM the fold wears"
        );
        assert_eq!(
            posed[2].x_rot.to_bits(),
            item_bits(plain[2].x_rot),
            "the open gate the right ITEM the fold rides"
        );
        // THE LEFT SPEAR THE RIGHT ITEM CANCELS: the left's
        // affectsOffhandPose true the poseRightArm call the walk
        // zeroes keep — the ITEM fold the gate never leaks
        let mut d = AvatarState::rest(0.0, 0.0, 0.0);
        d.left_spear = true;
        d.right_item = true;
        let posed = build_posed_parts(&d);
        assert_eq!(
            (
                posed[2].x_rot.to_bits(),
                posed[2].y_rot.to_bits(),
                posed[2].z_rot.to_bits()
            ),
            (
                plain[2].x_rot.to_bits(),
                plain[2].y_rot.to_bits(),
                plain[2].z_rot.to_bits()
            ),
            "the left SPEAR's affectsOffhand the right ITEM cancels"
        );
        assert_ne!(
            posed[3].x_rot.to_bits(),
            plain[3].x_rot.to_bits(),
            "the left SPEAR the use lane the fold wears"
        );
    }

    /// THE BOW AND ARROW'S OWN BODIES (javap `HumanoidModel.
    /// poseRightArm` pc 174-249 the case-3 walk, poseLeftArm the
    /// mirror — the CP #176/#391/#326/#392 the cp_float_483
    /// resolves, the pins_483 oracle the f32 chains the bits
    /// speak): the aim the BOTH arms the putfield assignments wear
    /// — the walk's own swing the x/y the body wipes (the zRot the
    /// pose never touches), the head's pitch/yaw the four sums
    /// ride, the 0.4 the NON-bow arm's spread the two bodies their
    /// own chains keep (the right method's left arm the two fadds,
    /// the left method's right arm the fadd-then-fsub — the f32
    /// rounding the orders differ, the bits the oracle re-speaks).
    #[test]
    fn the_bow_pose_the_aim_the_bodies_speak() {
        // THE RIGHT BODY (lane A: the main hand the bow uses): the
        // head pitch 30°, the yaw offset 20° the sums the f32
        // chains ride — the pins_483 oracle the bits pin
        let mut s = AvatarState::rest(0.0, 30.0, 0.0);
        s.head_yaw_off_deg = 20.0;
        s.is_using_item = true;
        s.use_item_main_hand = true;
        s.ticks_using_item = 1.0;
        s.right_bow = true;
        let posed = build_posed_parts(&s);
        // right.yRot = f32(-0.1 + hy) — CP #176 the one fadd
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0x3E7F_0B1E,
            "the right body the bow arm's yRot the one-fadd chain wears"
        );
        // left.yRot = f32(f32(0.1 + hy) + 0.4) — the TWO fadds the
        // non-bow arm's spread rides
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x3F59_5C61,
            "the right body the spread arm's yRot the two-fadd chain wears"
        );
        // both xRot = f32(-1.5707963705062866 + hx) — CP #392 the
        // pitch rides
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBF86_0A92,
            "the right body the bow arm's xRot the pitch wears"
        );
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBF86_0A92,
            "the right body the spread arm's xRot the pitch wears"
        );
        // THE ZROT THE WALK'S BOB KEEPS — the case-3 body the pose
        // never touches (the plain state the same age the bits
        // replay)
        let mut p = AvatarState::rest(0.0, 30.0, 0.0);
        p.head_yaw_off_deg = 20.0;
        let plain = build_posed_parts(&p);
        assert_eq!(
            posed[2].z_rot.to_bits(),
            plain[2].z_rot.to_bits(),
            "the right body the walk zRot the bow never touches"
        );
        assert_eq!(
            posed[3].z_rot.to_bits(),
            plain[3].z_rot.to_bits(),
            "the right body the left zRot the bob keeps"
        );
        // THE WALK THE BODY WIPES: the swing's own cosines the
        // ASSIGNMENTS replace — the walk's posed arms the aim's bits
        // the both arms wear (the putfield the += never was)
        let mut w = AvatarState::rest(0.0, 30.0, 0.0);
        w.head_yaw_off_deg = 20.0;
        w.walk_pos = 2.75;
        w.walk_speed = 1.0;
        w.age_ticks = 5.0;
        let walk_plain = build_posed_parts(&w);
        w.right_bow = true;
        w.is_using_item = true;
        w.use_item_main_hand = true;
        w.ticks_using_item = 1.0;
        let walk_bow = build_posed_parts(&w);
        assert_eq!(
            walk_bow[2].x_rot.to_bits(),
            0xBF86_0A92,
            "the walk the bow arm's xRot the assignment wipes"
        );
        assert_eq!(
            walk_bow[3].x_rot.to_bits(),
            0xBF86_0A92,
            "the walk the spread arm's xRot the assignment wipes"
        );
        assert_ne!(
            walk_plain[2].x_rot.to_bits(),
            0xBF86_0A92,
            "the walk's own swing the assignment the bits differ"
        );
        assert_eq!(
            walk_bow[2].z_rot.to_bits(),
            walk_plain[2].z_rot.to_bits(),
            "the walk the zRot the bow keeps"
        );
        // THE LEFT BODY (lanes B+C: the off hand the bow uses) — the
        // MIRROR the 0.4 the right (non-bow) arm the
        // fadd-then-fsub carries, the left arm the one fadd keeps
        let mut l = AvatarState::rest(0.0, 30.0, 0.0);
        l.head_yaw_off_deg = 20.0;
        l.is_using_item = true;
        l.use_item_main_hand = false;
        l.ticks_using_item = 1.0;
        l.left_bow = true;
        let posed = build_posed_parts(&l);
        // right.yRot = f32(f32(-0.1 + hy) - 0.4) — the fsub the
        // rounding the right body's left arm never shared
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0xBE1A_8E7C,
            "the left body the spread arm's yRot the fadd-fsub chain wears"
        );
        // left.yRot = f32(0.1 + hy) — the bow arm the one fadd
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x3EE5_EBF5,
            "the left body the bow arm's yRot the one-fadd chain wears"
        );
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBF86_0A92,
            "the left body the spread arm's xRot the pitch wears"
        );
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBF86_0A92,
            "the left body the bow arm's xRot the pitch wears"
        );
        assert_eq!(
            posed[2].z_rot.to_bits(),
            plain[2].z_rot.to_bits(),
            "the left body the right zRot the bow never touches"
        );
    }

    /// THE BOW LANE'S OWN GATES (the poseArms matrix the three
    /// verdicts dispatch — the affectsOffhandPose skip the bow's
    /// both-arms body the second posing cancels, the using hand the
    /// gate's verdict reads): the right bow the left ITEM the body
    /// already carries (the ITEM fold the gate never leaks), the
    /// left bow the right SPEAR the mirror keeps, the idle bow the
    /// verdict the ITEM fall-through answers, the off-branch the
    /// bow verdict the fold never wears.
    #[test]
    fn the_bow_pose_the_gates_the_matrix_speak() {
        // LANE A: the right bow the BOTH-ARMS body poses — the left
        // stack the sword verdict (left_item) the case-3 bits keep,
        // the ITEM fold the affectsOffhand gate the leak never wears
        let mut a = AvatarState::rest(0.0, 30.0, 0.0);
        a.head_yaw_off_deg = 20.0;
        a.is_using_item = true;
        a.use_item_main_hand = true;
        a.ticks_using_item = 1.0;
        a.right_bow = true;
        a.left_item = true;
        let posed = build_posed_parts(&a);
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x3F59_5C61,
            "the right bow the left ITEM the case-3 body the bits keep"
        );
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBF86_0A92,
            "the right bow the left ITEM the aim the gate keeps"
        );
        // LANES B+C: the left bow the left method's body poses — the
        // right stack the spear verdict the case-3 bits keep, the
        // SPEAR use fold the gate the leak never wears (the spear's
        // thirdPersonHandUse the xRot the aim's bits the value
        // differs)
        let mut b = AvatarState::rest(0.0, 30.0, 0.0);
        b.head_yaw_off_deg = 20.0;
        b.is_using_item = true;
        b.use_item_main_hand = false;
        b.ticks_using_item = 1.0;
        b.left_bow = true;
        b.right_spear = true;
        let posed = build_posed_parts(&b);
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0xBE1A_8E7C,
            "the left bow the right SPEAR the case-3 body the bits keep"
        );
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBF86_0A92,
            "the left bow the right SPEAR the aim the gate keeps"
        );
        // THE IDLE BOW THE VERDICT NEVER WEARS: the jar's use gate
        // the `getUseItemRemainingTicks() > 0` window rides — the
        // not-using walk the ITEM fall-through answers (the client's
        // own verdict the excluded hands the honest FALSE keeps: the
        // row table the idle bow the FALSE keeps, the right chain
        // the ITEM verdict the fall-through wears)
        let mut plain_idle = AvatarState::rest(0.0, 30.0, 0.0);
        plain_idle.head_yaw_off_deg = 20.0;
        let idle_plain = build_posed_parts(&plain_idle);
        let mut c = AvatarState::rest(0.0, 30.0, 0.0);
        c.head_yaw_off_deg = 20.0;
        c.right_item = true;
        let posed = build_posed_parts(&c);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            (idle_plain[2].x_rot * 0.5 - 0.314_159_274_101_257_3_f32).to_bits(),
            "the idle bow the ITEM fall-through the fold wears"
        );
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0.0f32.to_bits(),
            "the idle bow the ITEM fold the yRot zero wears"
        );
        assert_ne!(
            posed[2].x_rot.to_bits(),
            0xBF86_0A92,
            "the idle bow the aim the matrix never rides"
        );
        // THE FIELD THE TABLESWITCH READS (the block round's own law
        // — autopush-484): the jar's poseRightArm the rightArmPose
        // field WHATEVER it holds reads, no lane guard the bytecode
        // keeps — the BOW_AND_ARROW field the case-3 body wears
        // regardless of the using walk (the using window the VERDICT
        // construction gates — the row table the idle bow the FALSE
        // keeps — the dispatch the field alone reads)
        let mut d = AvatarState::rest(0.0, 30.0, 0.0);
        d.head_yaw_off_deg = 20.0;
        d.right_bow = true;
        let posed = build_posed_parts(&d);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBF86_0A92,
            "the field verdict the case-3 body the dispatch wears"
        );
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0x3E7F_0B1E,
            "the field verdict the one-fadd yRot wears"
        );
    }

    /// THE SHIELD'S OWN BODY (javap `HumanoidModel.poseBlockingArm`
    /// the 69-byte whole, the case-2 call the poseRightArm/
    /// poseLeftArm pcs 75-84 pair speaks — the right arm the
    /// iconst_1 the left the iconst_0 rides): the TWO putfields the
    /// body keeps — xRot = xRot·0.5 − 54° + clamp(head.xRot, −80°,
    /// 25°), yRot = ±30°·DEG2RAD + clamp(head.yRot, ±30°) — the ±30
    /// the ARM SIDE the right flag answers, the clamps the head
    /// pitch/yaw the ±80/25/±30 windows ride, the zRot the pose
    /// never touches. THE PINS the pins_484 oracle computes (the f32
    /// chains the jar's own fmul/fsub/fadd order keeps — every op
    /// rounds).
    #[test]
    fn the_block_pose_the_shield_the_arm_bends_speak() {
        // SCENE R (lane A: the main hand the shield uses): the head
        // pitch +30° the clamp MAX lane rides (30° > 25° the jar's
        // own window), the yaw offset +20° the ±30° window keeps —
        // the standing idle arm the 0.0 start the fold reads
        let mut s = AvatarState::rest(0.0, 30.0, 0.0);
        s.head_yaw_off_deg = 20.0;
        s.is_using_item = true;
        s.use_item_main_hand = true;
        s.ticks_using_item = 1.0;
        s.right_block = true;
        let posed = build_posed_parts(&s);
        // right.xRot = f32(f32(0.0·0.5 − 54°) + clamp(30°, −80°, 25°))
        // = f32(−0.9424779 + 0.43633232) — the clamp MAX the bits pin
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBF01_92C2,
            "the right body the shield arm's xRot the clamp-max lane wears"
        );
        // right.yRot = f32(−30°·DEG2RAD + clamp(20°, ±30°)) — the −30
        // the one fmul the #423's own bits rounds to, the fadd the
        // head's yaw rides
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0xBE32_B8C4,
            "the right body the shield arm's yRot the minus-30 side wears"
        );
        // THE ZROT THE POSE NEVER TOUCHES — the plain state the same
        // age the bits replay
        let mut p = AvatarState::rest(0.0, 30.0, 0.0);
        p.head_yaw_off_deg = 20.0;
        let plain = build_posed_parts(&p);
        assert_eq!(
            posed[2].z_rot.to_bits(),
            plain[2].z_rot.to_bits(),
            "the right body the walk zRot the block never touches"
        );
        // SCENE C (the clamp MIN lanes): the head pitch −90° the
        // −80° floor rides, the yaw offset −40° the −30° floor rides
        let mut c = AvatarState::rest(0.0, -90.0, 0.0);
        c.head_yaw_off_deg = -40.0;
        c.is_using_item = true;
        c.use_item_main_hand = true;
        c.ticks_using_item = 1.0;
        c.right_block = true;
        let posed = build_posed_parts(&c);
        // right.xRot = f32(−0.9424779 + (−1.3962634)) — the floor
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xC015_ADF0,
            "the clamp the pitch floor the shield arm's xRot wears"
        );
        // right.yRot = f32(−30°·DEG2RAD + (−30°)) — the yaw floor
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0xBF86_0A92,
            "the clamp the yaw floor the shield arm's yRot wears"
        );
        // SCENE CR (the ×0.5 chain the non-zero start reads): the
        // crouch's own +0.4 the walk arm rides, the half the exact
        // exponent walk the 0.2 keeps
        let mut cr = AvatarState::rest(0.0, 30.0, 0.0);
        cr.head_yaw_off_deg = 20.0;
        cr.crouching = true;
        cr.is_using_item = true;
        cr.use_item_main_hand = true;
        cr.ticks_using_item = 1.0;
        cr.right_block = true;
        let posed = build_posed_parts(&cr);
        // right.xRot = f32(f32(0.4·0.5 − 54°) + clamp(30°, −80°, 25°))
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBE9C_BF1D,
            "the crouch the ×0.5 chain the raised start reads"
        );
        // SCENE L (lanes B+C: the off hand the shield uses): the
        // iconst_0 the +30 side the ifeq answers — the MIRROR the
        // same xRot chain the left arm keeps
        let mut l = AvatarState::rest(0.0, 30.0, 0.0);
        l.head_yaw_off_deg = 20.0;
        l.is_using_item = true;
        l.use_item_main_hand = false;
        l.ticks_using_item = 1.0;
        l.left_block = true;
        let posed = build_posed_parts(&l);
        // left.xRot = f32(0.0·0.5 − 54° + clamp(30°, −80°, 25°)) —
        // the ONE-ARM body the start the left arm's own keeps
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBF01_92C2,
            "the left body the shield arm's xRot the same chain wears"
        );
        // left.yRot = f32(+30°·DEG2RAD + clamp(20°, ±30°)) — the +30
        // the #422 the iconst_0 rides
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x3F5F_66F3,
            "the left body the shield arm's yRot the plus-30 side wears"
        );
        assert_eq!(
            posed[3].z_rot.to_bits(),
            plain[3].z_rot.to_bits(),
            "the left body the zRot the block never touches"
        );
    }

    /// THE BLOCK LANE'S OWN GATES (the poseArms matrix the FOUR
    /// verdicts dispatch — the BLOCK the clinit's (2,0,0) pair the
    /// affectsOffhandPose FALSE row keeps, the one-arm body the
    /// second posing never cancels): the right shield the left ITEM
    /// the gate opens, the left shield the right ITEM the mirror
    /// keeps, the idle shield the verdict the ITEM fall-through
    /// answers (the use gate the using window alone wears).
    #[test]
    fn the_block_pose_the_gates_the_matrix_speak() {
        // LANE A: the right shield the ONE-ARM body poses — the left
        // stack the sword verdict (left_item) the ITEM fold keeps,
        // the affectsOffhand flag FALSE the gate the second posing
        // never cancels (the (2,0,0) pair the SPEAR/BOW true rows
        // the shield alone misses)
        let mut a = AvatarState::rest(0.0, 30.0, 0.0);
        a.head_yaw_off_deg = 20.0;
        a.is_using_item = true;
        a.use_item_main_hand = true;
        a.ticks_using_item = 1.0;
        a.right_block = true;
        a.left_item = true;
        let posed = build_posed_parts(&a);
        // the left arm's ITEM fold: f32(0.0·0.5 − 0.31415927) — the
        // gate the fold the bits keep
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBEA0_D97C,
            "the right shield the left ITEM the gate-open the fold wears"
        );
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0.0f32.to_bits(),
            "the right shield the left ITEM the yRot zero wears"
        );
        // LANES B+C: the left shield the mirror poses — the right
        // stack the ITEM verdict the gate-open the fold keeps
        let mut b = AvatarState::rest(0.0, 30.0, 0.0);
        b.head_yaw_off_deg = 20.0;
        b.is_using_item = true;
        b.use_item_main_hand = false;
        b.ticks_using_item = 1.0;
        b.left_block = true;
        b.right_item = true;
        let posed = build_posed_parts(&b);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBEA0_D97C,
            "the left shield the right ITEM the mirror the fold wears"
        );
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0.0f32.to_bits(),
            "the left shield the right ITEM the yRot zero wears"
        );
        // THE IDLE SHIELD THE VERDICT NEVER WEARS: the jar's use gate
        // the `getUseItemRemainingTicks() > 0` window rides — the
        // not-using walk the ITEM fall-through answers (the idle
        // shield the BLOCK fold never reaches, the same law the idle
        // bow the matrix keeps)
        let mut idle = AvatarState::rest(0.0, 30.0, 0.0);
        idle.head_yaw_off_deg = 20.0;
        idle.right_item = true;
        let posed = build_posed_parts(&idle);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBEA0_D97C,
            "the idle shield the ITEM fall-through the fold wears"
        );
        assert_ne!(
            posed[2].x_rot.to_bits(),
            0xBF01_92C2,
            "the idle shield the block fold the matrix never rides"
        );
        // THE BLOCK THE BOW THE VERDICTS EXCLUDE: the one using walk
        // the one anim carries — the shield name the BLOCK answers,
        // the bow name the BOW_AND_ARROW — the verdicts the using
        // window alone answers the matrix dispatches (the client's
        // row table the exclusive names the flags keep, the fold the
        // right flag's own verdict reads)
        let mut both = AvatarState::rest(0.0, 30.0, 0.0);
        both.head_yaw_off_deg = 20.0;
        both.is_using_item = true;
        both.use_item_main_hand = true;
        both.ticks_using_item = 1.0;
        both.right_block = true;
        both.right_bow = true;
        let posed = build_posed_parts(&both);
        // the bow verdict the chain's FIRST branch the fold picks —
        // the aim's bits the shield's clamp lane never wears (the
        // chain order the exclusive verdicts the defensive keeps,
        // the jar's own row the one anim never both answers)
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBF86_0A92,
            "the both-flags state the bow branch the chain picks"
        );
    }

    /// THE SPYGLASS'S OWN BODY (javap `HumanoidModel.poseRightArm/
    /// poseLeftArm` the case-7 wholes pcs 326-386, the CP floats the
    /// cp_float_485 resolves — #408 = 1.9198622 (0x3FF5BE0B), #409 =
    /// 0.2617994 (0x3E860A92), #410 = −2.4 (0xC019999A), #411 = 3.3
    /// (0x40533333)): the PURE ASSIGNMENT the head's own pitch
    /// drives — no walk read the value chain keeps (the crouch's own
    /// +0.4 the raised arm the body WIPES, the isCrouching branch
    /// the 15° the chain itself adds), the clamp the −2.4/3.3 window
    /// rides, the yRot the side mirrors (the right the fsub, the
    /// left the fadd — the jar's own asymmetry speaks), the zRot the
    /// pose never touches. THE PINS the pins_485 oracle computes.
    #[test]
    fn the_spyglass_pose_the_clamp_the_crouch_speak() {
        // SCENE R (lane A: the main hand the spyglass uses): the
        // head pitch +30° the chain rides, the yaw offset +20° the
        // mirror sides answer — the standing idle arm the 0.0 start
        // the body WIPES (the pure assignment the walk read never
        // keeps)
        let mut s = AvatarState::rest(0.0, 30.0, 0.0);
        s.head_yaw_off_deg = 20.0;
        s.is_using_item = true;
        s.use_item_main_hand = true;
        s.ticks_using_item = 1.0;
        s.right_spyglass = true;
        let posed = build_posed_parts(&s);
        // right.xRot = clamp(f32(30° − 1.9198622) − 0.0, −2.4, 3.3)
        // = −1.3962634 — the bits the BLOCK's own #419 keeps (the
        // CP the one value the two constants store)
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBFB2_B8C2,
            "the right body the spyglass arm's xRot the chain wears"
        );
        // right.yRot = f32(20° − 0.2617994) — the fsub the right
        // body's own rides
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0x3DB2_B8C0,
            "the right body the fsub side the mirror's minus wears"
        );
        // THE LEFT ARM THE NO-VERDICT THE WALK KEEPS: the jar's
        // case-7 body the ONE arm poses (the rightArm putfield alone
        // — the left's own verdict absent the walk zeroes the fold
        // never leaves), the same bits the plain state replays
        let mut p = AvatarState::rest(0.0, 30.0, 0.0);
        p.head_yaw_off_deg = 20.0;
        let plain = build_posed_parts(&p);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            plain[3].x_rot.to_bits(),
            "the left arm the no-verdict the walk zeroes keep"
        );
        assert_eq!(
            posed[3].y_rot.to_bits(),
            plain[3].y_rot.to_bits(),
            "the left arm the no-verdict the walk yRot keeps"
        );
        // THE ZROT THE POSE NEVER TOUCHES — the plain state the same
        // age the bits replay
        assert_eq!(
            posed[2].z_rot.to_bits(),
            plain[2].z_rot.to_bits(),
            "the right body the walk zRot the spyglass never touches"
        );
        // SCENE C (the clamp MIN lane + the crouch the wipe keeps):
        // the head pitch −90° the floor rides — AND the crouching
        // walk the +0.4 the raised start keeps, the PURE ASSIGNMENT
        // the wipe the bits prove (the same bits the standing floor
        // wears — the isCrouching branch the 15° the chain adds
        // BEFORE the clamp, the raised walk start the body never
        // reads)
        let mut c = AvatarState::rest(0.0, -90.0, 0.0);
        c.crouching = true;
        c.is_using_item = true;
        c.use_item_main_hand = true;
        c.ticks_using_item = 1.0;
        c.right_spyglass = true;
        let posed = build_posed_parts(&c);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xC019_999A,
            "the clamp the pitch floor the crouch's wipe the bits keep"
        );
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0xBE86_0A92,
            "the crouch the fsub side the minus-15 the bits wear"
        );
        // SCENE Z (the clamp MAX lane): the head pitch 310° the
        // ceiling rides — the jar's own window the +3.3 the bits
        // keep
        let mut z = AvatarState::rest(0.0, 310.0, 0.0);
        z.is_using_item = true;
        z.use_item_main_hand = true;
        z.ticks_using_item = 1.0;
        z.right_spyglass = true;
        let posed = build_posed_parts(&z);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0x4053_3333,
            "the clamp the pitch ceiling the spyglass arm's xRot wears"
        );
        // SCENE L (lanes B+C: the off hand the spyglass uses): the
        // fadd the left body's own rides, the right arm the walk
        // zeroes the no-verdict keeps
        let mut l = AvatarState::rest(0.0, 30.0, 0.0);
        l.head_yaw_off_deg = 20.0;
        l.is_using_item = true;
        l.use_item_main_hand = false;
        l.ticks_using_item = 1.0;
        l.left_spyglass = true;
        let posed = build_posed_parts(&l);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBFB2_B8C2,
            "the left body the off-hand lane the xRot chain wears"
        );
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x3F1C_61AA,
            "the left body the off-hand lane the fadd side wears"
        );
        assert_eq!(
            posed[2].x_rot.to_bits(),
            plain[2].x_rot.to_bits(),
            "the right arm the no-verdict the walk zeroes keep"
        );
    }

    /// THE HORN'S OWN BODY AND THE GATES' MATRIX (javap the case-8
    /// wholes pcs 389-430, the CP floats the cp_float_485 resolves —
    /// #187 = −1.2 (0xBF99999A), #415 = 1.2 (0x3F99999A), #416 =
    /// 1.4835298 (0x3FBDE44E), #417 = 0.5235988 (0x3F060A92) — the
    /// BLOCK's own #417 the same bits the 30° keeps): xRot =
    /// clamp(head.xRot, ±1.2) − 1.4835298 the pure assignment rides,
    /// the yRot the side mirrors, the (8,0,0) clinit pair the
    /// affectsOffhand flag FALSE the second posing never cancels.
    /// THE PINS the pins_485 oracle computes.
    #[test]
    fn the_horn_pose_the_gates_the_matrix_speak() {
        // SCENE R (lane A: the main hand the goat horn uses): the
        // head pitch 0° the clamp the zero keeps, the yaw offset
        // +20° the mirror sides answer
        let mut s = AvatarState::rest(0.0, 0.0, 0.0);
        s.head_yaw_off_deg = 20.0;
        s.is_using_item = true;
        s.use_item_main_hand = true;
        s.ticks_using_item = 1.0;
        s.right_horn = true;
        let posed = build_posed_parts(&s);
        // right.xRot = f32(clamp(0, ±1.2) − 1.4835298) — the
        // subtrahend's own bits the chain keeps
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBFBD_E44E,
            "the right body the horn arm's xRot the subtrahend wears"
        );
        // right.yRot = f32(20° − 0.5235988) — the fsub the right
        // body's own rides (the −30° the mouth's aim speaks)
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0xBE32_B8C4,
            "the right body the fsub side the minus-30 wears"
        );
        // THE AFFECTSOFFHAND FALSE THE GATE THE SECOND POSING NEVER
        // CANCELS (the (8,0,0) pair the FALSE row keeps): the left
        // stack the sword verdict the ITEM fold keeps — the right
        // horn's first posing the gate never blocks
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0.0f32.to_bits(),
            "the left stack the no-verdict the walk zeroes keep"
        );
        let mut a = AvatarState::rest(0.0, 0.0, 0.0);
        a.head_yaw_off_deg = 20.0;
        a.is_using_item = true;
        a.use_item_main_hand = true;
        a.ticks_using_item = 1.0;
        a.right_horn = true;
        a.left_item = true;
        let posed = build_posed_parts(&a);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBEA0_D97C,
            "the right horn the left ITEM the gate-open the fold wears"
        );
        // SCENE C (the clamp MAX lane): the head pitch +90° the
        // ceiling rides — xRot = f32(1.2 − 1.4835298)
        let mut c = AvatarState::rest(0.0, 90.0, 0.0);
        c.is_using_item = true;
        c.use_item_main_hand = true;
        c.ticks_using_item = 1.0;
        c.right_horn = true;
        let posed = build_posed_parts(&c);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBE91_2AD0,
            "the clamp the pitch ceiling the horn arm's xRot wears"
        );
        // SCENE C2 (the clamp MIN lane): the head pitch −90° the
        // floor rides — xRot = f32(−1.2 − 1.4835298)
        let mut c2 = AvatarState::rest(0.0, -90.0, 0.0);
        c2.is_using_item = true;
        c2.use_item_main_hand = true;
        c2.ticks_using_item = 1.0;
        c2.right_horn = true;
        let posed = build_posed_parts(&c2);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xC02B_BEF4,
            "the clamp the pitch floor the horn arm's xRot wears"
        );
        // LANES B+C: the left horn the mirror poses — the fadd the
        // left body's own rides, the right ITEM verdict the
        // gate-open the fold keeps
        let mut b = AvatarState::rest(0.0, 0.0, 0.0);
        b.head_yaw_off_deg = 20.0;
        b.is_using_item = true;
        b.use_item_main_hand = false;
        b.ticks_using_item = 1.0;
        b.left_horn = true;
        b.right_item = true;
        let posed = build_posed_parts(&b);
        // left.yRot = f32(20° + 0.5235988) — the fadd the left
        // body's own rides (the +30° the mirror speaks)
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBFBD_E44E,
            "the left body the horn fold the same xRot chain wears"
        );
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x3F5F_66F3,
            "the left body the fadd side the plus-30 wears"
        );
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBEA0_D97C,
            "the left horn the right ITEM the gate-open the fold wears"
        );
        // THE IDLE HORN THE VERDICT NEVER WEARS: the jar's use gate
        // the remaining-ticks window rides — the not-using walk the
        // ITEM fall-through answers (the idle horn the horn fold
        // never reaches)
        let mut idle = AvatarState::rest(0.0, 0.0, 0.0);
        idle.head_yaw_off_deg = 20.0;
        idle.right_item = true;
        let posed = build_posed_parts(&idle);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBEA0_D97C,
            "the idle horn the ITEM fall-through the fold wears"
        );
        assert_ne!(
            posed[2].x_rot.to_bits(),
            0xBFBD_E44E,
            "the idle horn the horn fold the matrix never rides"
        );
        // THE SPYGLASS THE HORN THE VERDICTS EXCLUDE: the one using
        // walk the one anim carries — the chain order the exclusive
        // verdicts the defensive keeps (the spyglass branch first
        // the fold picks, the horn's bits the clamp lane never
        // wears) — the head pitch 0° the chain the pitch-0 bits
        // answers: clamp(f32(0 − 1.9198622), −2.4, 3.3) =
        // −1.9198622
        let mut both = AvatarState::rest(0.0, 0.0, 0.0);
        both.head_yaw_off_deg = 20.0;
        both.is_using_item = true;
        both.use_item_main_hand = true;
        both.ticks_using_item = 1.0;
        both.right_spyglass = true;
        both.right_horn = true;
        let posed = build_posed_parts(&both);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBFF5_BE0B,
            "the both-flags state the spyglass branch the chain picks"
        );
    }

    /// THE CROSSBOW CHARGE'S OWN BODY (javap `AnimationUtils.
    /// animateCrossbowCharge` the 58-byte whole, the CP floats the
    /// cp_float_486 resolves — #25 = −0.8 (0xBF4CCCCD), #26 = 0.8
    /// (0x3F4CCCCD), #27 = −0.9707963 (0xBF78861C), #32 = 0.4
    /// (0x3ECCCCCD), #33 = 0.85 (0x3F59999A), #19 = −π/2
    /// (0xBFC90FDB)): the BOTH-ARMS body the ONE call carries — the
    /// charge arm the right flag answers, the other arm the body's
    /// own second half poses (the float copy the pc 48-55 keeps,
    /// the pull the f1 = clamp(ticks, 0, max)/max drives, the lerp
    /// the jar's own fsub/fmul/fadd chain keeps). THE PINS the
    /// pins_486 oracle computes.
    #[test]
    fn the_crossbow_charge_the_both_arms_the_lerp_speak() {
        // SCENE R (lane A: the main hand the crossbow charges): the
        // ticks 1.0/25.0 the f1 = 0.04 the early pull keeps — the
        // charge arm the constants alone wear (the head the body
        // NEVER reads), the other arm the lerp walks
        let mut s = AvatarState::rest(0.0, 0.0, 0.0);
        s.head_yaw_off_deg = 20.0;
        s.is_using_item = true;
        s.use_item_main_hand = true;
        s.ticks_using_item = 1.0;
        s.max_crossbow_charge_duration = 25.0;
        s.right_crossbow_charge = true;
        let posed = build_posed_parts(&s);
        // charge arm (right): the constants' own bits
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBF78_861C,
            "the charge arm the #27 pitch the constant wears"
        );
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0xBF4C_CCCD,
            "the charge arm the minus-0.8 the right flag wears"
        );
        // other arm (left): yRot = f32(lerp(0.04, 0.4, 0.85) × 1.0),
        // xRot = lerp(0.04, #27, −π/2) — the pull's early walk
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x3ED6_0419,
            "the other arm the lerp spread the fmul side wears"
        );
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBF7E_AAF9,
            "the other arm the lerp pitch the pull walks"
        );
        // THE GATE THE SECOND POSING CANCELS — yet the BOTH-ARMS
        // body the left arm itself poses: the affectsoffhand TRUE
        // the left CHAIN skips, the body's own second half the
        // support arm wears (the walk zeroes the bits disprove)
        let mut p = AvatarState::rest(0.0, 0.0, 0.0);
        p.head_yaw_off_deg = 20.0;
        let plain = build_posed_parts(&p);
        assert_ne!(
            posed[3].x_rot.to_bits(),
            plain[3].x_rot.to_bits(),
            "the body's own second half the support arm poses"
        );
        // SCENE F (the full charge): the ticks 30 the clamp the 25
        // rides, the f1 = 1.0 the lerp's own ends wear — the
        // two-rounding chain the lerp(1.0, start, end) the ends'
        // own bits keep
        let mut f = AvatarState::rest(0.0, 0.0, 0.0);
        f.head_yaw_off_deg = 20.0;
        f.is_using_item = true;
        f.use_item_main_hand = true;
        f.ticks_using_item = 30.0;
        f.max_crossbow_charge_duration = 25.0;
        f.right_crossbow_charge = true;
        let posed = build_posed_parts(&f);
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x3F59_999A,
            "the full charge the lerp end the spread wears"
        );
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBFC9_0FDB,
            "the full charge the −π/2 the pitch wears"
        );
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBF78_861C,
            "the charge arm the clamp the constants keep"
        );
        // SCENE L (lanes B+C: the off hand the crossbow charges): the
        // iconst_0 the LEFT arm the charge arm rides, the other the
        // right — the side's ±1 the fmul the mirror speaks
        let mut l = AvatarState::rest(0.0, 0.0, 0.0);
        l.head_yaw_off_deg = 20.0;
        l.is_using_item = true;
        l.use_item_main_hand = false;
        l.ticks_using_item = 12.0;
        l.max_crossbow_charge_duration = 25.0;
        l.left_crossbow_charge = true;
        let posed = build_posed_parts(&l);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBF78_861C,
            "the left charge arm the same #27 the mirror wears"
        );
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x3F4C_CCCD,
            "the left charge arm the plus-0.8 the iconst_0 wears"
        );
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0xBF1D_B22D,
            "the other arm the minus side the fmul wears"
        );
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBFA1_203D,
            "the other arm the 12-tick pull the lerp walks"
        );
    }

    /// THE CROSSBOW HOLD'S OWN BODY AND THE GATES' MATRIX (javap
    /// `AnimationUtils.animateCrossbowHold` the 49-byte whole — CP #7
    /// = −0.3 (0xBE99999A), #8 = 0.3 (0x3E99999A), #15 = 0.6
    /// (0x3F19999A), #16 = −0.6 (0xBF19999A), #19 = −π/2, #23 = 0.1
    /// (0x3DCCCCCD), #24 = −1.5 (0xBFC00000)): the BOTH-ARMS body
    /// the ONE call carries — the head's own pitch/yaw the four sums
    /// ride, the (6,1,1) pair the affectsOffhand flag TRUE the
    /// second posing's CHAIN cancels (the body's own support arm
    /// the fold never loses). THE PINS the pins_486 oracle computes.
    #[test]
    fn the_crossbow_hold_the_gates_the_matrix_speak() {
        // SCENE R (the idle walk the else branch speaks — the loaded
        // crossbow the main hand holds): the left chain the no
        // verdict keeps, the gate opens, the right chain the hold
        // fold wears — the head pitch 0/yaw 20 the sums read
        let mut s = AvatarState::rest(0.0, 0.0, 0.0);
        s.head_yaw_off_deg = 20.0;
        s.right_crossbow_hold = true;
        let posed = build_posed_parts(&s);
        // charge arm (right): xRot = f32(f32(−π/2 + 0) + 0.1), yRot =
        // f32(−0.3 + 20°) — the two-fadd chain the bits keep
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBFBC_430E,
            "the charge arm the (−π/2 + pitch) + 0.1 the chain wears"
        );
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0x3D48_F940,
            "the charge arm the minus-0.3 + yaw the fadd wears"
        );
        // other arm (left): xRot = f32(−1.5 + 0), yRot = f32(0.6 +
        // 20°) — the body's own second half the support poses
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBFC0_0000,
            "the other arm the −1.5 pitch the one fadd wears"
        );
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x3F72_F5FB,
            "the other arm the plus-0.6 + yaw the fadd wears"
        );
        // SCENE C (the head's own pitch the sums read): the head
        // pitch −90° the chains walk — charge.xRot = f32(f32(−π/2 +
        // −1.5707964) + 0.1), other.xRot = f32(−1.5 + −1.5707964)
        let mut c = AvatarState::rest(0.0, -90.0, 0.0);
        c.right_crossbow_hold = true;
        let posed = build_posed_parts(&c);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xC042_A975,
            "the head pitch the charge arm's chain reads"
        );
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xC044_87EE,
            "the head pitch the other arm's fadd reads"
        );
        // SCENE L (the loaded crossbow the off hand holds): the left
        // chain FIRST (the idle branch poseLeftArm first speaks) —
        // the iconst_0 the left arm the charge arm rides, the gate
        // `!left_crossbow_hold` the right CHAIN cancels, the body's
        // own other arm the right side wears
        let mut l = AvatarState::rest(0.0, 0.0, 0.0);
        l.head_yaw_off_deg = 20.0;
        l.left_crossbow_hold = true;
        let posed = build_posed_parts(&l);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBFBC_430E,
            "the left charge arm the same chain the mirror wears"
        );
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x3F26_292E,
            "the left charge arm the plus-0.3 + yaw wears"
        );
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBFC0_0000,
            "the body's own other arm the right side wears"
        );
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0xBE80_7A72,
            "the body's own other arm the minus-0.6 + yaw wears"
        );
        // THE AFFECTSOFFHAND TRUE THE LEFT CHAIN CANCELS (the (5,1,1)
        // pair the gate law — the CONTRAST the spyglass/horn FALSE
        // rows speak): the right charge the lane A first poses, the
        // left stack's ITEM verdict the CHAIN never reaches — the
        // body's OWN second half the support arm still wears (the
        // float copy + the lerp walk the scene R bits replay), the
        // ITEM fold the bits disprove
        let mut g = AvatarState::rest(0.0, 0.0, 0.0);
        g.head_yaw_off_deg = 20.0;
        g.is_using_item = true;
        g.use_item_main_hand = true;
        g.ticks_using_item = 1.0;
        g.max_crossbow_charge_duration = 25.0;
        g.right_crossbow_charge = true;
        g.left_item = true;
        let posed = build_posed_parts(&g);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBF7E_AAF9,
            "the affectsOffhand TRUE the left chain cancels — the body's own support arm wears"
        );
        assert_ne!(
            posed[3].x_rot.to_bits(),
            0xBEA0_D97C,
            "the left stack's ITEM fold the cancelled chain never reaches"
        );
        // THE CHARGE THE HOLD THE CHAIN ORDER EXCLUDES: the one
        // stack the one verdict carries — the charge branch first
        // the fold picks (the defensive order the exclusive
        // verdicts keep)
        let mut both = AvatarState::rest(0.0, 0.0, 0.0);
        both.head_yaw_off_deg = 20.0;
        both.is_using_item = true;
        both.use_item_main_hand = true;
        both.ticks_using_item = 1.0;
        both.max_crossbow_charge_duration = 25.0;
        both.right_crossbow_charge = true;
        both.right_crossbow_hold = true;
        let posed = build_posed_parts(&both);
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0xBF4C_CCCD,
            "the both-flags state the charge branch the chain picks"
        );
    }

    /// THE THROW TRIDENT'S OWN BODY (javap `HumanoidModel.
    /// poseRightArm/poseLeftArm` case 4, pcs 119-148 — CP #95 = 0.5
    /// (0x3F000000), #260 = 3.1415927410125732 = π (0x40490FDB)):
    /// the ONE-ARM fold the walk's own xRot READS — xRot = xRot·0.5 −
    /// π the fmul/fsub chain the putfield order keeps, the yRot zero
    /// the fconst_0 putfield replays, the head the body never reads,
    /// the zRot the pose never touches. THE VERDICT the use gate's
    /// own TRIDENT branch answers (AvatarRenderer.getArmPose
    /// pcs 81-91): the USING stack the RMB charge the verdict wears.
    /// THE PINS the pins_527 oracle computes.
    #[test]
    fn the_trident_pose_the_throw_the_fold_speaks() {
        // SCENE Z (the rest walk the fold reads: the walk's own zero
        // the bob's +0.0 the sum keeps): xRot = f32(f32(0.0·0.5) − π)
        // = −π the exact bits, yRot = 0.0 the fconst_0 replays
        let mut s = AvatarState::rest(0.0, 0.0, 0.0);
        s.head_yaw_off_deg = 20.0;
        s.is_using_item = true;
        s.use_item_main_hand = true;
        s.ticks_using_item = 10.0;
        s.right_trident = true;
        let posed = build_posed_parts(&s);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xC049_0FDB,
            "the throw fold the walk's own zero the −π wears"
        );
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0x0000_0000,
            "the throw fold the yRot zero the fconst_0 replays"
        );
        // THE ONE-ARM BODY (the poseLeftArm the walk's own zeroes the
        // second posing the trident lane never reads — the walk's
        // plain bits the plain state keeps): the other arm the fold
        // never touches, the head's own yaw the body never reads
        let mut p = AvatarState::rest(0.0, 0.0, 0.0);
        p.head_yaw_off_deg = 20.0;
        let plain = build_posed_parts(&p);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            plain[3].x_rot.to_bits(),
            "the one-arm body the other arm the walk's own keeps"
        );
        assert_eq!(
            posed[3].y_rot.to_bits(),
            plain[3].y_rot.to_bits(),
            "the one-arm body the other arm's yRot the walk keeps"
        );
        // SCENE C (the crouch's own +0.4 raise the fold READS — the
        // walk law the ITEM fold shares): xRot = f32(f32(0.4·0.5) − π)
        // the two-rounding chain the bits keep
        let mut c = AvatarState::rest(0.0, 0.0, 0.0);
        c.head_yaw_off_deg = 20.0;
        c.crouching = true;
        c.is_using_item = true;
        c.use_item_main_hand = true;
        c.ticks_using_item = 10.0;
        c.right_trident = true;
        let posed = build_posed_parts(&c);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xC03C_430E,
            "the crouch raise the fold reads — the f32 chain the bits wear"
        );
        // SCENE L (lanes B+C: the off hand the trident charges — the
        // poseLeftArm first the idle/use walk speaks): the LEFT arm
        // the fold wears, the gate below the right CHAIN cancels
        let mut l = AvatarState::rest(0.0, 0.0, 0.0);
        l.head_yaw_off_deg = 20.0;
        l.is_using_item = true;
        l.use_item_main_hand = false;
        l.ticks_using_item = 10.0;
        l.left_trident = true;
        l.right_item = true;
        let posed = build_posed_parts(&l);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xC049_0FDB,
            "the left throw arm the same −π the mirror wears"
        );
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x0000_0000,
            "the left throw arm the yRot zero the mirror replays"
        );
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0x0000_0000,
            "the gate the right chain cancels — the walk's own zero keeps"
        );
    }

    /// THE THROW TRIDENT'S OWN GATES' MATRIX (the clinit's (4,0,1)
    /// pair — twoHanded 0, affectsOffhandPose 1 — the gate the
    /// second posing's CHAIN cancels the bow's true row the same law
    /// keeps): the idle trident the ITEM fall-through the verdicts
    /// keep (the SPEARS tag the row table the seven tiers the
    /// trident never rides), the both-flags state the chain's
    /// defensive order the trident branch picks. THE PINS the
    /// pins_527 oracle computes.
    #[test]
    fn the_trident_pose_the_gates_the_matrix_speak() {
        // GATE R (the affectsOffhand TRUE the left CHAIN cancels —
        // the CONTRAST the spyglass/horn FALSE rows speak): the right
        // trident the lane A first poses, the left stack's ITEM
        // verdict the CHAIN never reaches — the walk's own zero the
        // plain state keeps, the ITEM fold the bits disprove
        let mut g = AvatarState::rest(0.0, 0.0, 0.0);
        g.head_yaw_off_deg = 20.0;
        g.is_using_item = true;
        g.use_item_main_hand = true;
        g.ticks_using_item = 10.0;
        g.right_trident = true;
        g.left_item = true;
        let posed = build_posed_parts(&g);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0x0000_0000,
            "the affectsOffhand TRUE the left chain cancels — the walk's own keeps"
        );
        assert_ne!(
            posed[3].x_rot.to_bits(),
            0xBEA0_D97C,
            "the left stack's ITEM fold the cancelled chain never reaches"
        );
        // THE IDLE TRIDENT (the ITEM fall-through the verdicts keep —
        // the use gate the idle stack never rides, the SPEARS tag the
        // row table the trident excludes): the ITEM lane the fold
        // wears, the −π/10 the walk's own zero the bits speak
        let mut i = AvatarState::rest(0.0, 0.0, 0.0);
        i.head_yaw_off_deg = 20.0;
        i.right_item = true;
        let posed = build_posed_parts(&i);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBEA0_D97C,
            "the idle trident the ITEM fall-through the −π/10 wears"
        );
        // THE BOTH-FLAGS DEFENSE (the chain's own order the trident
        // branch picks — the spear lane the tag law the same stack
        // never carries, the defensive order the both-flags state
        // alone reaches): the throw fold the −π the branch wears
        let mut both = AvatarState::rest(0.0, 0.0, 0.0);
        both.head_yaw_off_deg = 20.0;
        both.is_using_item = true;
        both.use_item_main_hand = true;
        both.ticks_using_item = 10.0;
        both.right_trident = true;
        both.right_spear = true;
        let posed = build_posed_parts(&both);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xC049_0FDB,
            "the both-flags state the trident branch the chain picks"
        );
    }

    /// THE BRUSH'S OWN BODY (javap `HumanoidModel.poseRightArm/
    /// poseLeftArm` case 9, pcs 294-325 — CP = 0.5 (0x3F000000),
    /// #407 = 0.6283185482025146 = π/5 (0x3F20D97C)): the ONE-ARM
    /// fold the walk's own xRot READS — xRot = xRot·0.5 − π/5 the
    /// fmul/fsub chain the putfield order keeps, the yRot zero the
    /// fconst_0 putfield replays, the head the body never reads, the
    /// zRot the pose never touches. THE VERDICT the use gate's own
    /// BRUSH branch answers (AvatarRenderer.getArmPose pcs 125-135):
    /// the USING stack the RMB brush hold the verdict wears. THE
    /// PINS the pins_528 oracle computes.
    #[test]
    fn the_brush_pose_the_fold_speaks() {
        // SCENE Z (the rest walk the fold reads: the walk's own zero
        // the bob's +0.0 the sum keeps): xRot = f32(f32(0.0·0.5) −
        // π/5) = −π/5 the exact bits, yRot = 0.0 the fconst_0 replays
        let mut s = AvatarState::rest(0.0, 0.0, 0.0);
        s.head_yaw_off_deg = 20.0;
        s.is_using_item = true;
        s.use_item_main_hand = true;
        s.ticks_using_item = 10.0;
        s.right_brush = true;
        let posed = build_posed_parts(&s);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBF20_D97C,
            "the brush fold the walk's own zero the −π/5 wears"
        );
        assert_eq!(
            posed[2].y_rot.to_bits(),
            0x0000_0000,
            "the brush fold the yRot zero the fconst_0 replays"
        );
        // THE ONE-ARM BODY (the poseLeftArm the walk's own zeroes the
        // second posing the brush lane never reads — the walk's
        // plain bits the plain state keeps): the other arm the fold
        // never touches, the head's own yaw the body never reads
        let mut p = AvatarState::rest(0.0, 0.0, 0.0);
        p.head_yaw_off_deg = 20.0;
        let plain = build_posed_parts(&p);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            plain[3].x_rot.to_bits(),
            "the one-arm body the other arm the walk's own keeps"
        );
        assert_eq!(
            posed[3].y_rot.to_bits(),
            plain[3].y_rot.to_bits(),
            "the one-arm body the other arm's yRot the walk keeps"
        );
        // SCENE C (the crouch's own +0.4 raise the fold READS — the
        // walk law the ITEM fold shares): xRot = f32(f32(0.4·0.5) −
        // π/5) the two-rounding chain the bits keep
        let mut c = AvatarState::rest(0.0, 0.0, 0.0);
        c.head_yaw_off_deg = 20.0;
        c.crouching = true;
        c.is_using_item = true;
        c.use_item_main_hand = true;
        c.ticks_using_item = 10.0;
        c.right_brush = true;
        let posed = build_posed_parts(&c);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBEDB_4C92,
            "the crouch raise the fold reads — the f32 chain the bits wear"
        );
        // SCENE L (the off hand the brush holds — the poseLeftArm
        // first the idle/use walk speaks): the LEFT arm the fold
        // wears, the affectsOffhand FALSE the gate below the right
        // CHAIN never cancels — the right stack's ITEM verdict the
        // chain reaches (the FALSE row the BLOCK pair shares, the
        // trident TRUE row the contrast keeps)
        let mut l = AvatarState::rest(0.0, 0.0, 0.0);
        l.head_yaw_off_deg = 20.0;
        l.is_using_item = true;
        l.use_item_main_hand = false;
        l.ticks_using_item = 10.0;
        l.left_brush = true;
        l.right_item = true;
        let posed = build_posed_parts(&l);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBF20_D97C,
            "the left brush arm the same −π/5 the mirror wears"
        );
        assert_eq!(
            posed[3].y_rot.to_bits(),
            0x0000_0000,
            "the left brush arm the yRot zero the mirror replays"
        );
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBEA0_D97C,
            "the affectsOffhand FALSE the right ITEM chain reaches — the −π/10 wears"
        );
    }

    /// THE BRUSH'S OWN GATES' MATRIX (the clinit's (9,0,0) pair —
    /// twoHanded 0, affectsOffhandPose 0 — the gate the second
    /// posing's chain NEVER cancels the BLOCK/SPYGLASS/HORN FALSE
    /// rows the same law keeps): the idle brush the ITEM
    /// fall-through the verdicts keep (the use gate the idle stack
    /// never rides), the using brush the mapped gate the ITEM
    /// fall-through excludes (the mapped names the SEVEN the brush
    /// joins), the both-flags state the chain's defensive order the
    /// brush branch picks. THE PINS the pins_528 oracle computes.
    #[test]
    fn the_brush_pose_the_gates_the_matrix_speak() {
        // GATE R (the affectsOffhand FALSE the left chain NEVER
        // cancels — the CONTRAST the trident TRUE row speaks): the
        // right brush the lane A first poses, the left stack's ITEM
        // verdict the chain REACHES — the ITEM fold the bits prove
        // (the walk's own zero the −π/10 wears)
        let mut g = AvatarState::rest(0.0, 0.0, 0.0);
        g.head_yaw_off_deg = 20.0;
        g.is_using_item = true;
        g.use_item_main_hand = true;
        g.ticks_using_item = 10.0;
        g.right_brush = true;
        g.left_item = true;
        let posed = build_posed_parts(&g);
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBEA0_D97C,
            "the affectsOffhand FALSE the left chain reaches — the ITEM fold wears"
        );
        // THE IDLE BRUSH (the ITEM fall-through the verdicts keep —
        // the use gate the idle stack never rides, no SPEARS-tag row
        // the brush carries): the ITEM lane the fold wears, the
        // −π/10 the walk's own zero the bits speak
        let mut i = AvatarState::rest(0.0, 0.0, 0.0);
        i.head_yaw_off_deg = 20.0;
        i.right_item = true;
        let posed = build_posed_parts(&i);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBEA0_D97C,
            "the idle brush the ITEM fall-through the −π/10 wears"
        );
        // THE BOTH-FLAGS DEFENSE (the chain's own order the brush
        // branch picks — the spear lane the tag law the same stack
        // never carries, the defensive order the both-flags state
        // alone reaches): the brush fold the −π/5 the branch wears
        let mut both = AvatarState::rest(0.0, 0.0, 0.0);
        both.head_yaw_off_deg = 20.0;
        both.is_using_item = true;
        both.use_item_main_hand = true;
        both.ticks_using_item = 10.0;
        both.right_brush = true;
        both.right_spear = true;
        let posed = build_posed_parts(&both);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBF20_D97C,
            "the both-flags state the brush branch the chain picks"
        );
        // THE SECOND-POSING'S OWN BRUSH LANE (the off-hand-use walk's
        // gate the right chain the brush lane rides — the right arm
        // the fold wears, the left brush arm the mirror keeps): the
        // brush lane the chain order holds (hold → trident → brush →
        // spear), the −π/5 the branch wears
        let mut s2 = AvatarState::rest(0.0, 0.0, 0.0);
        s2.head_yaw_off_deg = 20.0;
        s2.is_using_item = true;
        s2.use_item_main_hand = false;
        s2.ticks_using_item = 10.0;
        s2.left_item = true;
        s2.right_brush = true;
        let posed = build_posed_parts(&s2);
        assert_eq!(
            posed[2].x_rot.to_bits(),
            0xBF20_D97C,
            "the second posing's brush lane the chain order rides"
        );
        assert_eq!(
            posed[3].x_rot.to_bits(),
            0xBEA0_D97C,
            "the left stack's ITEM the first pose wears — the chain the lanes keep"
        );
    }


    /// THE ITEM LANE'S OWN USE FOLD (javap `ItemInHandLayer.
    /// submitArmWithItem` pcs 188-232 over the SPEAR pose's
    /// animateUseItem): the per-arm ticks the `(useItemHand == MAIN) ==
    /// (arm == mainArm)` zero law keeps, the SPEAR-only override the
    /// matrix composes — the using spear arm's payload the geometry
    /// moves, the idle spear's and the wrong arm's the anchor keeps.
    #[test]
    fn the_third_person_item_use_lane_the_geometry_speaks() {
        let centroid = |quads: &Vec<HeadBlockQuad>| -> [f64; 3] {
            let mut acc = [0.0f64; 3];
            let mut n = 0.0f64;
            for q in quads {
                for p in q.pts.iter() {
                    for k in 0..3 {
                        acc[k] += p[k] as f64;
                    }
                }
                n += 4.0;
            }
            if n > 0.0 {
                [acc[0] / n, acc[1] / n, acc[2] / n]
            } else {
                [0.0; 3]
            }
        };
        let using = |is_using: bool| {
            let mut s = AvatarState::rest(0.0, 0.0, 0.0);
            s.right_spear = true;
            s.is_using_item = is_using;
            s.use_item_main_hand = true;
            s.ticks_using_item = if is_using { 1.0 } else { 0.0 };
            s.kinetic_use_params = Some(crate::hand::SpearUseParams {
                raise: 1.0,
                raise_start: 1.0,
                raise_middle: 1.0,
                raise_end: 1.0,
                sway_progress: 0.0,
                lower: 0.0,
                raise_back: 0.0,
                sway_intensity: 0.0,
                sway_scale_slow: 0.0,
                sway_scale_fast: 0.0,
            });
            s
        };
        let idle = held_item_quads(&using(false), Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
            .expect("the spear's own stanza the bake carries");
        let raised =
            held_item_quads(&using(true), Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
                .expect("the using state the same bake keeps");
        let (ci, cr) = (centroid(&idle), centroid(&raised));
        let moved = (ci[0] - cr[0]).abs() + (ci[1] - cr[1]).abs() + (ci[2] - cr[2]).abs();
        assert!(moved > 0.05, "the use matrix the payload moves: {moved}");
        // THE WRONG ARM'S OWN ZERO (the left lane rides the per-arm
        // ticks the right-hand use zeroes — the anchor the matrix
        // never reaches; the idle state the same folded left arm wears
        // — only the gate the two states separate)
        let mut wrong = using(true);
        wrong.right_spear = false;
        wrong.left_spear = true;
        wrong.use_item_main_hand = true;
        let mut idle_left = using(false);
        idle_left.right_spear = false;
        idle_left.left_spear = true;
        let wrong_quads =
            offhand_item_quads(&wrong, Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
                .expect("the left arm's own bake");
        let left_idle =
            offhand_item_quads(&idle_left, Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
                .expect("the left arm's idle bake");
        assert_eq!(
            centroid(&wrong_quads),
            centroid(&left_idle),
            "the right-hand use the left arm's payload never moves"
        );
    }

    /// The STAB attack-item lane's own gates (ItemInHandLayer pcs
    /// 154-185): the zero clock the fcmpl/ifle refuses, the wrong arm
    /// the attackArm if_acmpne refuses, the kind the
    /// swingAnimationType if_acmpne refuses — and the pass the
    /// forward's own slide speaks (the component-less main hand the
    /// rotation-only fold wears, the kinetic the 0.38 adds — the two
    /// centroids the shared tilt never confounds, only the translate
    /// the two states separate).
    #[test]
    fn the_attack_item_lane_the_gate_the_forward_speak() {
        let centroid = |quads: &Vec<HeadBlockQuad>| -> [f64; 3] {
            let mut acc = [0.0f64; 3];
            let mut n = 0.0f64;
            for q in quads {
                for p in q.pts.iter() {
                    for k in 0..3 {
                        acc[k] += p[k] as f64;
                    }
                }
                n += 4.0;
            }
            if n > 0.0 {
                [acc[0] / n, acc[1] / n, acc[2] / n]
            } else {
                [0.0; 3]
            }
        };
        let stabbing = |kinetic: bool, t: f32, kind: crate::hand::SwingKind| {
            let mut s = AvatarState::rest(0.0, 0.0, 0.0);
            s.attack_time = t;
            s.swing_kind = kind;
            s.main_hand_kinetic = kinetic;
            s
        };
        // THE ZERO CLOCK (attackTime 0 — the lane's own first gate the
        // rest state fails; the kind the state carries the zero the
        // swing block also silences — the two states the same anchor
        // quad set wear)
        let idle =
            held_item_quads(&stabbing(true, 0.0, crate::hand::SwingKind::Stab), Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
                .expect("the spear's own stanza the bake carries");
        let bare =
            held_item_quads(&stabbing(false, 0.0, crate::hand::SwingKind::Whack), Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
                .expect("the bare bake");
        assert_eq!(centroid(&idle), centroid(&bare), "the zero clock the anchor keeps");
        // THE WRONG ARM (the attackArm the if_acmpne refuses: the LEFT
        // lane the right-hand attack mismatches — the kinetic flag the
        // refused lane never READS, the toggle the SAME state's quads
        // never moves; a lane the gate leaked into would speak the
        // forward's own slide the two centroids wear)
        let left_kin =
            offhand_item_quads(&stabbing(true, 0.3, crate::hand::SwingKind::Stab), Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
                .expect("the left arm's own bake");
        let left_nk =
            offhand_item_quads(&stabbing(false, 0.3, crate::hand::SwingKind::Stab), Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
                .expect("the left arm's refused bake");
        assert_eq!(
            centroid(&left_kin),
            centroid(&left_nk),
            "the attackArm mismatch the left lane never reads the forward"
        );
        // THE MIRROR PASS (attackArm LEFT the offhand's own gate speaks:
        // attack_right = false the left lane the STAB gate PASSES — the
        // kinetic flag the passing lane now reads, the slide the two
        // centroids wear — the pose the shared folds the flag never
        // touches, the delta the translate's own magnitude rides)
        let mut left_att = stabbing(true, 0.3, crate::hand::SwingKind::Stab);
        left_att.attack_right = false;
        let mut left_att_nk = stabbing(false, 0.3, crate::hand::SwingKind::Stab);
        left_att_nk.attack_right = false;
        let lp =
            offhand_item_quads(&left_att, Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
                .expect("the left-attack's own bake");
        let lpn =
            offhand_item_quads(&left_att_nk, Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
                .expect("the left-attack's refused bake");
        let slide = (centroid(&lp)[0] - centroid(&lpn)[0]).abs()
            + (centroid(&lp)[1] - centroid(&lpn)[1]).abs()
            + (centroid(&lp)[2] - centroid(&lpn)[2]).abs();
        assert!(slide > 0.02, "the left-attack's own slide the left lane wears: {slide}");
        // THE KIND + THE FORWARD (the RIGHT lane the STAB gate passes:
        // the component-less main hand the rotation-only fold wears —
        // the tilt alone moves the payload; the kinetic main hand the
        // 0.38 slide adds — the delta the two states separate the
        // translate's own magnitude rides)
        let nk =
            held_item_quads(&stabbing(false, 0.3, crate::hand::SwingKind::Stab), Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
                .expect("the rotation-only bake");
        let wk =
            held_item_quads(&stabbing(true, 0.3, crate::hand::SwingKind::Stab), Vec3::ZERO, 0.0, 0.0, "item/iron_spear_in_hand")
                .expect("the kinetic bake");
        let (nk_c, wk_c) = (centroid(&nk), centroid(&wk));
        let tilt = (nk_c[0] - wk_c[0]).abs() + (nk_c[1] - wk_c[1]).abs() + (nk_c[2] - wk_c[2]).abs();
        assert!(tilt > 0.02, "the forward's own slide the centroids speak: {tilt}");
        let moved = (wk_c[0] - centroid(&bare)[0]).abs()
            + (wk_c[1] - centroid(&bare)[1]).abs()
            + (wk_c[2] - centroid(&bare)[2]).abs();
        assert!(moved > 0.05, "the STAB lane the payload moves: {moved}");
    }
    /// THE WARDEN MESH'S OWN LAYER (javap `WardenModel.createBodyLayer`
    /// the 26.2 jar speaks — the TWELVE parts the hierarchy banks, the
    /// ten cubes the six-face fold renders): the sha1-verified 128x128
    /// load the quads answer, the TEN cubes x SIX faces the count
    /// keeps, the extents the pivots' own chain wears — the feet the
    /// bone's 24-texel line serves, the crown the head box's -26
    /// texel reach wears (the 3.125 the 50-texel stack speaks), the
    /// tendrils the head's children ride.
    #[test]
    fn the_warden_mesh_the_body_layer_keeps() {
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join(skin::WARDEN_RUNTIME_PATH).exists());
        let Some(root) = root else { return };
        let Some(model) = WardenModel::load(root) else {
            return; // the sha1 gate — the honest skip
        };
        // THE REST POSE (age 0, the tendril clock stopped): the idle
        // sway the zero-sin keeps, the shiver the zero clock stills
        let state = WardenState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 0.0,
            tendril_animation: 0.0,
            roar_time: None,
            sniff_time: None,
            emerge_time: None,
            dig_time: None,
            attack_time: None,
        };
        let feet = Vec3::new(10.0, 32.0, -4.0);
        let quads = model.quads(&state, feet, 0.0, 0.0);
        // THE TEN CUBES (bone none, body 1, head 1, tendrils 2,
        // ribcages 2, arms 2, legs 2) the six-face fold renders
        assert_eq!(
            quads.len(),
            60,
            "the twelve parts' ten cubes the sixty faces wear"
        );
        // THE EXTENTS (the pivots' own chain): the TENDRIL TIPS the
        // head's own children wear (bone 24 - body 21 - head 13 -
        // tendril pivot 12 - box 13 = -35 texels -> 24+35 = 59 texels
        // = 3.6875, the idle 0.06/0.025 tilts the small wobble the
        // reach adds), the crown the head box's -26 texel reach keeps
        // (3.125) BELOW the tendril tips, the feet the bone line's
        // own floor speaks (the legs' 24-texel bottom)
        let mut max_y = f32::MIN;
        let mut min_y = f32::MAX;
        for q in &quads {
            for pt in &q.pts {
                max_y = max_y.max(pt[1]);
                min_y = min_y.min(pt[1]);
            }
        }
        assert!(
            max_y > feet.y + 3.6 && max_y < feet.y + 3.9,
            "the tendril tips' own reach the fifty-nine-texel stack speaks: {max_y}"
        );
        assert!(
            min_y > feet.y - 0.05 && min_y < feet.y + 0.1,
            "the feet the bone's own 24-texel line keeps: {min_y}"
        );
        // THE WIDTH (the TENDRIL TIPS' own lateral reach: the left
        // tendril's box runs 0..16 texels past its +8 pivot = 24
        // texels = 1.5 blocks half-width — the arms' 17-texel reach
        // the SECOND widest keeps — the idle tilts the small swing
        // the tolerance wears)
        let mut max_x = f32::MIN;
        for q in &quads {
            for pt in &q.pts {
                max_x = max_x.max(pt[0]);
            }
        }
        assert!(
            max_x > feet.x + 1.5 && max_x < feet.x + 1.7,
            "the tendril tips' own 24-texel half-width wears: {max_x}"
        );
        // THE TENDRIL SHIVER (the swing the clock arms — the shiver
        // clock the crown's own zero-width planes the x_rot swings):
        // the filled clock the quads MOVE the rest pose never wears
        let shiver_state = WardenState { tendril_animation: 1.0, age_ticks: 0.0, ..state };
        let shiver_quads = model.quads(&shiver_state, feet, 0.0, 0.0);
        let moved: f32 = shiver_quads
            .iter()
            .zip(quads.iter())
            .map(|(s, r)| {
                s.pts
                    .iter()
                    .zip(r.pts.iter())
                    .map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs())
                    .sum::<f32>()
            })
            .sum();
        assert!(moved > 0.5, "the shiver clock the tendrils swing: {moved}");
    }

    /// THE WARDEN'S OWN WALK FOLDS (javap `WardenModel.animateWalk`
    /// pc 0-208, verbatim): the damping the min(0.5, 3.0 * speed)
    /// law keeps, the legs the anti-phase cos pair swings (the left
    /// the plain phase, the right the +PI answers — the mirror the
    /// negation speaks), the arms the 0.8 counter-swing keeps, the
    /// phase the 0.8662 scalar banks, the head the += ride the body
    /// the putfield assignment serves.
    #[test]
    fn the_warden_walk_the_folds_wear() {
        // THE DAMPING (min(0.5, 3.0 * speed)): the slow 0.1 walk the
        // 0.3 damping keeps, the fast 1.0 walk the 0.5 ceiling clamps
        let slow = WardenState {
            walk_pos: 2.0,
            walk_speed: 0.1,
            age_ticks: 0.0,
            tendril_animation: 0.0,
            ..WardenState::rest()
        };
        let fast = WardenState { walk_speed: 1.0, ..slow };
        let f4 = 2.0 * 0.8662;
        let slow_damp = 0.5_f32.min(3.0 * 0.1);
        let fast_damp = 0.5_f32.min(3.0 * 1.0);
        // THE LEG PAIR (x_rot = cos(phase) * damping — the left the
        // plain phase, the right the +PI anti-phase): the parts the
        // private builder banks, the roots the fold's own legs speak
        let slow_parts = build_warden_parts(&slow);
        let fast_parts = build_warden_parts(&fast);
        let (slow_body, slow_rleg, slow_lleg) = {
            // the bone's children: [body, right_leg, left_leg]
            let body = &slow_parts.children[0];
            (
                body.x_rot,
                slow_parts.children[1].x_rot,
                slow_parts.children[2].x_rot,
            )
        };
        let expected_lleg = mth_cos(f4) * slow_damp;
        let expected_rleg = mth_cos(f4 + std::f32::consts::PI) * slow_damp;
        assert_eq!(slow_lleg, expected_lleg, "the left leg the plain phase wears");
        assert_eq!(slow_rleg, expected_rleg, "the right leg the +PI anti-phase wears");
        assert!(
            (slow_lleg + slow_rleg).abs() < 1e-4,
            "the anti-phase the mirror law keeps"
        );
        // THE BODY (x_rot = cos(phase) * min(0.35, damping) + the
        // idle 0.025*cos(age*0.1) add — age 0 the 0.025 keeps)
        let expected_body = mth_cos(f4) * 0.35_f32.min(slow_damp) + 0.025;
        assert_eq!(slow_body, expected_body, "the body the f7-damped fold wears");
        // THE DAMPING'S CEILING: the fast walk the legs NEVER exceed
        // the 0.5 clamp the min law keeps
        let fast_rleg = fast_parts.children[1].x_rot;
        assert_eq!(fast_rleg, mth_cos(f4 + std::f32::consts::PI) * fast_damp);
        assert!(fast_rleg.abs() <= 0.5 + 1e-4, "the clamp the fast walk keeps");
        // THE ARMS (x_rot = -0.8 * cos/sin * damping — the left the
        // plain cos, the right the sin answers): the body's children
        // [head, ribcages, arms] the pivot pair names — the RIGHT
        // arm the pair's first slot keeps (the builder's own order:
        // head, ribs, right, left), the LEFT the second answers
        let slow_arms = &slow_parts.children[0].children;
        let right_arm = &slow_arms[3];
        let left_arm = &slow_arms[4];
        assert_eq!(left_arm.x_rot, -(0.8 * mth_cos(f4) * slow_damp));
        assert_eq!(right_arm.x_rot, -(0.8 * mth_sin(f4) * slow_damp));
        assert_eq!(left_arm.pivot, [13.0, -13.0, 1.0]);
        assert_eq!(right_arm.pivot, [-13.0, -13.0, 1.0]);
    }

    /// THE ROAR DEFINITION'S OWN BANK (javap `WardenAnimation` clinit
    /// pc 7771-9138 — the channel bones the addAnimation chain names,
    /// the keyframe counts the anewarray sizes wear, the spot
    /// constants the ldc floats speak, the `withLength(4.2f)` the
    /// 84-tick window keeps).
    #[test]
    fn the_roar_definition_the_jar_banks() {
        // THE LENGTH (the withLength head — the ROAR_DURATION 84 the
        // 20-tick second wears; the final keyframe's own timestamp
        // the length equals)
        assert_eq!(
            WARDEN_ROAR_BODY_ROT[7].t, 4.2,
            "the 84-tick window the tail keeps"
        );
        // THE CHANNEL COUNTS (the anewarray sizes: body 8/5, head
        // 8/6, right_arm 7/5, left_arm 7/5)
        assert_eq!(WARDEN_ROAR_BODY_ROT.len(), 8, "the body's rot pair banks");
        assert_eq!(WARDEN_ROAR_BODY_POS.len(), 5, "the body's pos pair banks");
        assert_eq!(WARDEN_ROAR_HEAD_ROT.len(), 8, "the head's rot pair banks");
        assert_eq!(WARDEN_ROAR_HEAD_POS.len(), 6, "the head's pos pair banks");
        assert_eq!(
            WARDEN_ROAR_RIGHT_ARM_ROT.len(),
            7,
            "the right arm's rot pair banks"
        );
        assert_eq!(
            WARDEN_ROAR_RIGHT_ARM_POS.len(),
            5,
            "the right arm's pos pair banks"
        );
        assert_eq!(
            WARDEN_ROAR_LEFT_ARM_ROT.len(),
            7,
            "the left arm's rot pair banks"
        );
        assert_eq!(
            WARDEN_ROAR_LEFT_ARM_POS.len(),
            5,
            "the left arm's pos pair banks"
        );
        // THE SPOT CONSTANTS (the ldc floats the javap speaks): the
        // wind-up apex, the shake pair, the arm raises, the settle
        assert_eq!(WARDEN_ROAR_BODY_ROT[1].t, 1.24, "the wind-up's own tick");
        assert_eq!(
            WARDEN_ROAR_BODY_ROT[1].v,
            [-25.0, 0.0, 0.0],
            "the -25 body tilt the apex banks"
        );
        assert_eq!(WARDEN_ROAR_BODY_ROT[6].v, [47.5, 0.0, 0.0], "the 47.5 peak");
        assert_eq!(
            WARDEN_ROAR_HEAD_ROT[3].v,
            [-32.5, 0.0, 26.0],
            "the head shake the +26 zRot wears"
        );
        assert_eq!(
            WARDEN_ROAR_HEAD_ROT[4].v,
            [-32.5, 0.0, -27.5],
            "the anti-phase shake the -27.5 answers"
        );
        assert_eq!(
            WARDEN_ROAR_RIGHT_ARM_ROT[1].v,
            [-120.0, 0.0, -20.0],
            "the right arm's -120 raise"
        );
        assert_eq!(
            WARDEN_ROAR_LEFT_ARM_ROT[1].v,
            [-125.0, 0.0, 20.0],
            "the left arm's -125 raise"
        );
        assert_eq!(
            WARDEN_ROAR_LEFT_ARM_ROT[5].v,
            [25.0, 18.4, -30.0],
            "the settle's own arm words"
        );
        assert_eq!(
            WARDEN_ROAR_BODY_POS[1].v,
            [0.0, -1.0, 3.0],
            "the body's pos lift the raw words keep"
        );
        assert_eq!(
            WARDEN_ROAR_HEAD_POS[2].v,
            [0.0, -2.0, -6.0],
            "the head's pos the shake window banks"
        );
        // THE REST TAIL (every channel's final keyframe the (0,0,0)
        // rest banks — the window's own end the pose clears)
        for (name, kfs) in [
            ("body rot", &WARDEN_ROAR_BODY_ROT[..]),
            ("body pos", &WARDEN_ROAR_BODY_POS[..]),
            ("head rot", &WARDEN_ROAR_HEAD_ROT[..]),
            ("head pos", &WARDEN_ROAR_HEAD_POS[..]),
            ("right arm rot", &WARDEN_ROAR_RIGHT_ARM_ROT[..]),
            ("right arm pos", &WARDEN_ROAR_RIGHT_ARM_POS[..]),
            ("left arm rot", &WARDEN_ROAR_LEFT_ARM_ROT[..]),
            ("left arm pos", &WARDEN_ROAR_LEFT_ARM_POS[..]),
        ] {
            let last = &kfs[kfs.len() - 1];
            assert_eq!(last.t, 4.2, "the {name} tail the definition's length keeps");
            assert_eq!(last.v, [0.0, 0.0, 0.0], "the {name} tail the rest banks");
        }
    }

    /// THE KEYFRAME CHANNEL'S OWN WALK (javap `KeyframeAnimation$Entry
    /// .apply` pc 0-138 — the binary search the segment pair keeps, the
    /// delta the clamp speaks, the CATMULLROM the end keyframe's own
    /// choice wears, the posVec's y the fneg negates, the degreeVec
    /// the 0.017453292 multiplies — the convert-first bake the clinit's
    /// own order speaks).
    #[test]
    fn the_roar_channel_the_segment_walk_keeps() {
        let deg = 0.017_453_292_f32;
        // THE WIND-UP'S OWN START (elapsed 0 — the segment [0, 1.24]
        // the delta-zero keeps, the catmullrom(0) the p1 = kf0 speaks)
        assert_eq!(
            warden_roar_channel(&WARDEN_ROAR_BODY_ROT, 0.0, false),
            [0.0, 0.0, 0.0],
            "the start the rest banks"
        );
        // THE APEX (elapsed 1.24 — the binary search the kf1 timestamp
        // answers, the delta the 1.0 clamp keeps, the catmullrom(1) the
        // p2 = kf1 speaks — the a=p0=b=p1 the ZERO vecs keep, the
        // c=kf2/d=kf3 the neighbors wear; the inline chain the f32
        // cancellation's own 1-ulp word speaks — the raw -25° rad the
        // chain NEVER equals bitwise)
        let t = 1.0_f32;
        let (a, b, c, d) = (
            0.0_f32,
            0.0_f32,
            -25.0_f32 * deg,
            32.5_f32 * deg,
        );
        let expected_apex = 0.5
            * (2.0 * b
                + (c - a) * t
                + (2.0 * a - 5.0 * b + 4.0 * c - d) * t * t
                + (3.0 * b - a - 3.0 * c + d) * t * t * t);
        let got = warden_roar_channel(&WARDEN_ROAR_BODY_ROT, 1.24, false);
        assert_eq!(got[0], expected_apex, "the apex the catmullrom endpoint wears");
        // THE POS BAKE (elapsed 1.24 — the raw (0,-1,3) the y-only fneg
        // wears: (0, +1, +3) — the z the posVec never negates)
        assert_eq!(
            warden_roar_channel(&WARDEN_ROAR_BODY_POS, 1.24, true),
            [0.0, 1.0, 3.0],
            "the pos the y-neg bake keeps"
        );
        // THE MID-SEGMENT (elapsed 1.4 — the segment [1.24, 1.6], the
        // p0..p3 the kf0..kf3 answers, the inline chain the bytecode's
        // own f32 order replicates: 0.5 * (2b + (c-a)t + quad*t*t +
        // cubic*t*t*t), the powers the LEFT-assoc keeps)
        let t = (1.4_f32 - 1.24) / (1.6 - 1.24);
        // THE ROT X (a=0, b=-25, c=32.5, d=38.33 — the radians the
        // convert-first bake wears)
        let (a, b, c, d) = (
            0.0_f32 * deg,
            -25.0_f32 * deg,
            32.5_f32 * deg,
            38.33_f32 * deg,
        );
        let expected = 0.5
            * (2.0 * b
                + (c - a) * t
                + (2.0 * a - 5.0 * b + 4.0 * c - d) * t * t
                + (3.0 * b - a - 3.0 * c + d) * t * t * t);
        let got = warden_roar_channel(&WARDEN_ROAR_BODY_ROT, 1.4, false);
        assert_eq!(got[0], expected, "the mid-segment rot the catmullrom wears");
        // THE POS Y (a=0, b=+1, c=+3, d=+3 — the baked words)
        let (a, b, c, d) = (0.0_f32, 1.0_f32, 3.0_f32, 3.0_f32);
        let expected_y = 0.5
            * (2.0 * b
                + (c - a) * t
                + (2.0 * a - 5.0 * b + 4.0 * c - d) * t * t
                + (3.0 * b - a - 3.0 * c + d) * t * t * t);
        let got = warden_roar_channel(&WARDEN_ROAR_BODY_POS, 1.4, true);
        assert_eq!(got[1], expected_y, "the mid-segment pos the bake keeps");
        // THE PAST-END REST (elapsed 5.0 — the binary search the n
        // never-found wears, the j==i the delta-zero keeps, the value
        // the last keyframe's own (0,0,0) banks)
        assert_eq!(
            warden_roar_channel(&WARDEN_ROAR_BODY_ROT, 5.0, false),
            [0.0, 0.0, 0.0],
            "the past-end the rest speaks"
        );
        assert_eq!(
            warden_roar_channel(&WARDEN_ROAR_BODY_POS, 5.0, true),
            [0.0, 0.0, 0.0],
            "the past-end pos the rest answers"
        );
    }

    /// THE ROAR'S OWN RENDER LAW (javap `WardenModel.setupAnim` pc
    /// 106-118 — the roarAnimation the state consults, the apply the
    /// ADDITIVE tail wears; the pose window the `84 - roar_ticks`
    /// derivation the screens pass banks — the sound tick the 25th
    /// window tick answers).
    #[test]
    fn the_roar_deltas_the_pose_window_banks() {
        // THE NO-OP (the never-armed None — the parts the rest pose
        // keeps: the +0.0 sums the folds' own numbers exactly wear)
        let calm = build_warden_parts(&WardenState::rest());
        let roared = build_warden_parts(&WardenState {
            roar_time: Some(0.0),
            ..WardenState::rest()
        });
        assert_eq!(calm.x_rot, roared.x_rot, "the elapsed-0 wind-up the zero wears");
        let calm_body = &calm.children[0];
        let roared_body = &roared.children[0];
        assert_eq!(
            calm_body.pivot,
            roared_body.pivot,
            "the elapsed-0 pos the zero keeps"
        );
        assert_eq!(calm_body.x_rot, roared_body.x_rot, "the elapsed-0 rot the zero speaks");
        // THE WINDOW'S OWN DERIVATION (the sound tick: 84 - 59 = 25
        // ticks — the elapsed the (25 * 50) / 1000 = 1.25s the
        // getTimeInMillis fold keeps): the body the -25°→32.5° segment
        // rides, the delta the (1.25-1.24)/0.36 speaks
        let warden_roar_ticks = crate::sculk::WARDEN_ROAR_DURATION - 59;
        assert_eq!(warden_roar_ticks, 25, "the sound tick the window wears");
        let elapsed = (25.0_f32 * 50.0) / 1000.0;
        assert_eq!(elapsed, 1.25, "the elapsed the *50/1000 fold keeps");
        let parts = build_warden_parts(&WardenState {
            roar_time: Some(25.0),
            ..WardenState::rest()
        });
        let body = &parts.children[0];
        let t = (1.25_f32 - 1.24) / (1.6 - 1.24);
        let deg = 0.017_453_292_f32;
        let (a, b, c, d) = (
            0.0_f32 * deg,
            -25.0_f32 * deg,
            32.5_f32 * deg,
            38.33_f32 * deg,
        );
        let expected_x = 0.5
            * (2.0 * b
                + (c - a) * t
                + (2.0 * a - 5.0 * b + 4.0 * c - d) * t * t
                + (3.0 * b - a - 3.0 * c + d) * t * t * t);
        // THE IDLE'S OWN ADD (the rest state's age-0 clock the idle
        // sway wears — the body_x the +0.025*cos(0) the folds' own
        // tail banks, the roar delta the AFTER speaks: the setupAnim
        // order the idle AHEAD of the keyframe applies keeps)
        assert_eq!(
            body.x_rot,
            0.025_f32 + expected_x,
            "the sound-tick tilt the idle + the catmullrom wear"
        );
        // THE POS SHIFT (the baked (0,+1,+3) apex the segment rides —
        // the pivot the -21.0 base + the delta keeps; the z the (0,3,
        // -6) baked words the same chain speaks)
        let (a, b, c, d) = (0.0_f32, 1.0_f32, 3.0_f32, 3.0_f32);
        let expected_y = 0.5
            * (2.0 * b
                + (c - a) * t
                + (2.0 * a - 5.0 * b + 4.0 * c - d) * t * t
                + (3.0 * b - a - 3.0 * c + d) * t * t * t);
        let (a, b, c, d) = (0.0_f32, 3.0_f32, -6.0_f32, -6.0_f32);
        let expected_z = 0.5
            * (2.0 * b
                + (c - a) * t
                + (2.0 * a - 5.0 * b + 4.0 * c - d) * t * t
                + (3.0 * b - a - 3.0 * c + d) * t * t * t);
        assert_eq!(
            body.pivot,
            [0.0, -21.0 + expected_y, expected_z],
            "the body pivot the pos delta wears"
        );
        // THE ARMS' OWN RAISE (the right arm at 1.25s — the binary
        // search the [1.24, 1.48] segment keeps (the 1.25 the 1.24
        // timestamp OUTGROWS), the p0..p3 the kf1..kf4 answers)
        let right_arm = &body.children[3];
        let ta = (1.25_f32 - 1.24) / (1.48 - 1.24);
        let deg = 0.017_453_292_f32;
        let (a, b, c, d) = (
            -120.0_f32 * deg,
            -77.5_f32 * deg,
            67.5_f32 * deg,
            37.5_f32 * deg,
        );
        let expected_ra = 0.5
            * (2.0 * b
                + (c - a) * ta
                + (2.0 * a - 5.0 * b + 4.0 * c - d) * ta * ta
                + (3.0 * b - a - 3.0 * c + d) * ta * ta * ta);
        assert_eq!(
            right_arm.x_rot, expected_ra,
            "the arm raise the segment walk wears"
        );
        // THE ADDITIVITY (the roaring WALKER — the deltas the walk
        // folds' own tail ADDS, the setupAnim order the keyframe
        // applies the LAST keeps; the calm walker the folds alone
        // speak)
        let walker = WardenState {
            walk_pos: 1.0,
            walk_speed: 0.5,
            ..WardenState::rest()
        };
        let roaring_walker = WardenState {
            roar_time: Some(25.0),
            ..walker
        };
        let calm_parts = build_warden_parts(&walker);
        let roar_parts = build_warden_parts(&roaring_walker);
        let calm_body = &calm_parts.children[0];
        let roar_body = &roar_parts.children[0];
        let f4 = 1.0_f32 * 0.8662;
        let damp = 0.5_f32.min(3.0 * 0.5);
        let walk_fold = mth_cos(f4) * 0.35_f32.min(damp) + 0.025;
        assert_eq!(
            roar_body.x_rot,
            walk_fold + expected_x,
            "the roar the walk fold's own tail adds"
        );
        assert_eq!(calm_body.x_rot, walk_fold, "the calm walker the folds alone speak");
    }

    /// THE WARDEN_SNIFF DEFINITION (javap `WardenAnimation` clinit pc
    /// 9141-9798 — the channel bones the addAnimation chain names, the
    /// keyframe counts the anewarray sizes wear, the spot constants
    /// the ldc floats speak, the `withLength(4.16f)` the
    /// `Mth.ceil(83.2f)` SNIFFING_DURATION the 84-tick window keeps,
    /// NO POSITION channel the def carries).
    #[test]
    fn the_sniff_definition_the_jar_banks() {
        // THE CHANNEL COUNTS (the anewarray sizes: body rot 6, head
        // rot 9, right_arm rot 5, left_arm rot 5 — the SNIFF def no
        // pos pair banks)
        assert_eq!(WARDEN_SNIFF_BODY_ROT.len(), 6, "the body's rot bank");
        assert_eq!(WARDEN_SNIFF_HEAD_ROT.len(), 9, "the head's rot bank");
        assert_eq!(WARDEN_SNIFF_RIGHT_ARM_ROT.len(), 5, "the right arm's rot bank");
        assert_eq!(WARDEN_SNIFF_LEFT_ARM_ROT.len(), 5, "the left arm's rot bank");
        // THE SPOT CONSTANTS (the ldc floats the javap speaks): the
        // lunge pair, the sway pairs, the settle
        assert_eq!(WARDEN_SNIFF_BODY_ROT[1].t, 0.56, "the lunge's own tick");
        assert_eq!(
            WARDEN_SNIFF_BODY_ROT[1].v,
            [17.5, 32.5, 0.0],
            "the 17.5/32.5 lunge the body banks"
        );
        assert_eq!(
            WARDEN_SNIFF_BODY_ROT[4].v,
            [10.0, -30.0, 0.0],
            "the -30 sway the settle banks"
        );
        assert_eq!(WARDEN_SNIFF_HEAD_ROT[1].t, 0.68, "the sway's own tick");
        assert_eq!(
            WARDEN_SNIFF_HEAD_ROT[1].v,
            [0.0, 40.0, 0.0],
            "the +40 yRot the head banks"
        );
        assert_eq!(
            WARDEN_SNIFF_HEAD_ROT[2].v,
            [-22.5, 40.0, 0.0],
            "the -22.5 xRot the tilt keeps"
        );
        assert_eq!(
            WARDEN_SNIFF_HEAD_ROT[4].v,
            [-35.0, 20.0, 0.0],
            "the -35 second dip the sway banks"
        );
        assert_eq!(
            WARDEN_SNIFF_HEAD_ROT[6].v,
            [0.0, -20.0, 0.0],
            "the -20 tail sway answers"
        );
        assert_eq!(
            WARDEN_SNIFF_RIGHT_ARM_ROT[1].v,
            [17.5, 0.0, 0.0],
            "the right arm's 17.5 raise"
        );
        assert_eq!(
            WARDEN_SNIFF_LEFT_ARM_ROT[1].v,
            [-15.0, 0.0, 0.0],
            "the left arm's -15 raise"
        );
        assert_eq!(
            WARDEN_SNIFF_RIGHT_ARM_ROT[3].v,
            [-15.0, 0.0, 0.0],
            "the arms' anti-phase pair the tail swaps"
        );
        // THE REST TAIL (every channel's final keyframe the (0,0,0)
        // rest banks — the definition's own 3.32s tail the 4.16s
        // window the rest keeps)
        for (name, kfs) in [
            ("body rot", &WARDEN_SNIFF_BODY_ROT[..]),
            ("head rot", &WARDEN_SNIFF_HEAD_ROT[..]),
            ("right arm rot", &WARDEN_SNIFF_RIGHT_ARM_ROT[..]),
            ("left arm rot", &WARDEN_SNIFF_LEFT_ARM_ROT[..]),
        ] {
            let last = &kfs[kfs.len() - 1];
            assert_eq!(last.t, 3.32, "the {name} tail the definition's rest keeps");
            assert_eq!(last.v, [0.0, 0.0, 0.0], "the {name} tail the rest banks");
        }
        // THE LENGTH (the withLength head the 4.16f speaks — the
        // 83.2-tick float the Mth.ceil the 84-tick SNIFFING_DURATION
        // banks): the rest tail the window INSIDE the definition's
        // own 4.16 the length wears
        assert_eq!(
            crate::sculk::WARDEN_SNIFFING_DURATION_TICKS,
            84,
            "the Mth.ceil(83.2f) the window keeps"
        );
    }

    /// THE SNIFF CHANNEL'S OWN WALK (the SAME Entry.apply walk the
    /// roar lanes shares — the binary search the segment pair keeps,
    /// the CATMULLROM the end keyframe's own choice wears, the
    /// degreeVec the 0.017453292 multiplies, the pos flag the def
    /// never wears — the SNIFF no pos channel banks).
    #[test]
    fn the_sniff_channel_the_segment_walk_keeps() {
        let deg = 0.017_453_292_f32;
        // THE ELAPSED-0 ZEROS (the first keyframe the segment start
        // banks — the delta the zero the pose keeps)
        let v = warden_roar_channel(&WARDEN_SNIFF_BODY_ROT, 0.0, false);
        assert_eq!(v, [0.0, 0.0, 0.0], "the elapsed-0 the rest pose keeps");
        // THE 0.56 APEX (the keyframe's own endpoint the exact values
        // wear — the catmullrom the endpoint the identity answers, the
        // degreeVec the radians bake)
        let v = warden_roar_channel(&WARDEN_SNIFF_BODY_ROT, 0.56, false);
        let want = [17.5 * deg, 32.5 * deg, 0.0];
        assert!(
            (v[0] - want[0]).abs() < 1e-6 && (v[1] - want[1]).abs() < 1e-6,
            "the 0.56 apex the endpoint bake wears: {v:?}"
        );
        // THE MID-SEGMENT WALK (the 0.28 the [0, 0.56] segment's own
        // half wears — the catmullrom the quartet the bytecode's own
        // order replicates: the inline chain the same f32 words keep)
        let v = warden_roar_channel(&WARDEN_SNIFF_BODY_ROT, 0.28, false);
        let p0 = 0.0_f32;
        let p1 = 0.0_f32;
        let p2 = 17.5 * deg;
        let p3 = 0.0_f32;
        let t = 0.5_f32;
        let want_x = 0.5
            * (2.0 * p1
                + (p2 - p0) * t
                + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t * t
                + (3.0 * p1 - p0 - 3.0 * p2 + p3) * t * t * t);
        assert!(
            (v[0] - want_x).abs() < 1e-6,
            "the mid-segment the catmullrom quartet answers: {v:?} vs {want_x}"
        );
        // THE HEAD'S OWN SEGMENT (the 0.68-0.96 pair the sway rides —
        // the 0.82 the half-walk the quartet answers: the yRot the
        // 40-plateau pair the neighbors OVERSHOOT — the catmullrom's
        // own curve the 43.75 apex wears)
        let v = warden_roar_channel(&WARDEN_SNIFF_HEAD_ROT, 0.82, false);
        let want_y = mth_catmullrom(
            0.5,
            0.0 * deg,
            40.0 * deg,
            40.0 * deg,
            20.0 * deg,
        );
        assert!(
            (v[1] - want_y).abs() < 1e-5,
            "the head's yRot the quartet's own curve wears: {:?} vs {:?}",
            v[1] / deg,
            want_y / deg
        );
        // THE PAST-END REST (the 3.4 the final keyframe's own tail the
        // window wears — the j==i the zero delta the rest pose banks)
        let v = warden_roar_channel(&WARDEN_SNIFF_BODY_ROT, 3.4, false);
        assert_eq!(v, [0.0, 0.0, 0.0], "the past-end the rest banks");
    }

    /// THE SNIFF DELTAS' OWN POSE WINDOW (the WardenSniffDeltas the
    /// sniff_time serves — the None the all-zero no-op wears, the
    /// elapsed the *50/1000 two-step keeps, the additive order the
    /// roar deltas the ahead banks, the layer quads the SAME deltas
    /// the shared setupAnim speaks).
    #[test]
    fn the_sniff_deltas_the_pose_window_banks() {
        // THE NO-OP (the never-armed None — the parts the rest pose
        // keeps: the +0.0 sums the folds' own numbers exactly wear)
        let calm = build_warden_parts(&WardenState::rest());
        let sniffing = build_warden_parts(&WardenState {
            sniff_time: Some(0.0),
            ..WardenState::rest()
        });
        assert_eq!(calm.x_rot, sniffing.x_rot, "the elapsed-0 wind-up the zero wears");
        let calm_body = &calm.children[0];
        let sniff_body = &sniffing.children[0];
        assert_eq!(
            calm_body.x_rot, sniff_body.x_rot,
            "the elapsed-0 rot the zero speaks"
        );
        // THE WINDOW'S OWN DERIVATION (the mid-walk: 84 - 44 = 40
        // ticks — the elapsed the (40 * 50) / 1000 = 2.0s the
        // getTimeInMillis fold keeps — the head the [0.68, 0.96]
        // segment's own walk answers, the settle xRot the plain
        // 10-degree tail the 2.2 kf wears)
        let elapsed = (40.0_f32 * 50.0) / 1000.0;
        assert_eq!(elapsed, 2.0, "the elapsed the *50/1000 fold keeps");
        let parts = build_warden_parts(&WardenState {
            sniff_time: Some(40.0),
            ..WardenState::rest()
        });
        let body = &parts.children[0];
        let delta = (2.0 - 0.96) / (2.2 - 0.96);
        // the quartet the segment's own wear banks: p0 = kfs[1], p1 =
        // kfs[2], p2 = kfs[3], p3 = kfs[4] — the lower-bound walk the
        // Entry.apply speaks
        let want_body_x = mth_catmullrom(
            delta,
            WARDEN_SNIFF_BODY_ROT[1].v[0] * 0.017_453_292_f32,
            WARDEN_SNIFF_BODY_ROT[2].v[0] * 0.017_453_292_f32,
            WARDEN_SNIFF_BODY_ROT[3].v[0] * 0.017_453_292_f32,
            WARDEN_SNIFF_BODY_ROT[4].v[0] * 0.017_453_292_f32,
        );
        assert!(
            (body.x_rot - 0.025 - want_body_x).abs() < 1e-5,
            "the body the [0.96, 2.2] segment the idle fold's own tail wears: {:?} vs {:?}",
            body.x_rot - 0.025,
            want_body_x
        );
        // THE ADDITIVITY (the sniffing WALKER — the deltas the walk
        // folds' own tail ADDS, the setupAnim order the sniff apply
        // the tail keeps; the calm walker the folds alone speak)
        let walker = WardenState {
            walk_pos: 1.0,
            walk_speed: 0.5,
            ..WardenState::rest()
        };
        let sniffing_walker = WardenState {
            sniff_time: Some(40.0),
            ..walker
        };
        let calm_parts = build_warden_parts(&walker);
        let sniff_parts = build_warden_parts(&sniffing_walker);
        let calm_body = &calm_parts.children[0];
        let sniff_body = &sniff_parts.children[0];
        let f4 = 1.0_f32 * 0.8662;
        let damp = 0.5_f32.min(3.0 * 0.5);
        let walk_fold = mth_cos(f4) * 0.35_f32.min(damp) + 0.025;
        assert_eq!(
            sniff_body.x_rot,
            walk_fold + want_body_x,
            "the sniff the walk fold's own tail adds"
        );
        assert_eq!(calm_body.x_rot, walk_fold, "the calm walker the folds alone speak");
        // THE LAYER QUADS THE SAME DELTAS (the layer models' own
        // setupAnim the shared walk wears — the sniffing head the
        // biolum layer's own stub the same delta answers)
        let feet = Vec3::new(3.0, 20.0, -7.0);
        let calm_layer =
            warden_layer_quads(&WardenState::rest(), feet, 0.0, 0.0, WardenLayer::Bioluminescent);
        let sniff_layer = warden_layer_quads(
            &WardenState {
                sniff_time: Some(14.0),
                ..WardenState::rest()
            },
            feet,
            0.0,
            0.0,
            WardenLayer::Bioluminescent,
        );
        assert_eq!(calm_layer.len(), sniff_layer.len(), "the layer the mesh keeps");
        let moved: f32 = sniff_layer
            .iter()
            .zip(calm_layer.iter())
            .map(|(s, r)| {
                s.pts
                    .iter()
                    .zip(r.pts.iter())
                    .map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs())
                    .sum::<f32>()
            })
            .sum();
        assert!(moved > 0.5, "the sniff the layer quads move: {moved}");
    }

    /// THE WARDEN_EMERGE DEFINITION (javap `WardenAnimation` clinit pc
    /// 0-4836 — the channel bones the addAnimation chain names, the
    /// keyframe counts the anewarray sizes wear, the spot constants
    /// the ldc floats speak, the `withLength(6.68f)` the `Mth.ceil
    /// (133.59999f)` EMERGE_DURATION the 134-tick window keeps).
    #[test]
    fn the_emerge_definition_the_jar_banks() {
        // THE CHANNEL COUNTS (the anewarray sizes: body 16/17, head
        // 18/17, right_arm 19/17, left_arm 22/19, right_leg 11/17,
        // left_leg 12/15)
        assert_eq!(WARDEN_EMERGE_BODY_ROT.len(), 16, "the body's rot bank");
        assert_eq!(WARDEN_EMERGE_BODY_POS.len(), 17, "the body's pos bank");
        assert_eq!(WARDEN_EMERGE_HEAD_ROT.len(), 18, "the head's rot bank");
        assert_eq!(WARDEN_EMERGE_HEAD_POS.len(), 17, "the head's pos bank");
        assert_eq!(WARDEN_EMERGE_RIGHT_ARM_ROT.len(), 19, "the right arm's rot bank");
        assert_eq!(WARDEN_EMERGE_RIGHT_ARM_POS.len(), 17, "the right arm's pos bank");
        assert_eq!(WARDEN_EMERGE_LEFT_ARM_ROT.len(), 22, "the left arm's rot bank");
        assert_eq!(WARDEN_EMERGE_LEFT_ARM_POS.len(), 19, "the left arm's pos bank");
        assert_eq!(WARDEN_EMERGE_RIGHT_LEG_ROT.len(), 11, "the right leg's rot bank");
        assert_eq!(WARDEN_EMERGE_RIGHT_LEG_POS.len(), 17, "the right leg's pos bank");
        assert_eq!(WARDEN_EMERGE_LEFT_LEG_ROT.len(), 12, "the left leg's rot bank");
        assert_eq!(WARDEN_EMERGE_LEFT_LEG_POS.len(), 15, "the left leg's pos bank");
        // THE SPOT CONSTANTS (the ldc floats the javap speaks): the
        // buried start, the climb, the grabs, the settle
        assert_eq!(
            WARDEN_EMERGE_BODY_POS[0].v,
            [0.0, -63.0, 0.0],
            "the -63 texel buried start the body banks"
        );
        assert_eq!(WARDEN_EMERGE_BODY_POS[2].t, 1.2, "the climb's own tick");
        assert_eq!(
            WARDEN_EMERGE_BODY_POS[2].v,
            [0.0, -32.0, 0.0],
            "the -32 texel mid-climb the plateau keeps"
        );
        assert_eq!(
            WARDEN_EMERGE_BODY_ROT[7].v,
            [25.0, 0.0, -7.5],
            "the 25-degree struggle the body banks"
        );
        assert_eq!(
            WARDEN_EMERGE_BODY_ROT[13].v,
            [70.0, 0.0, 2.5],
            "the 70-degree settle the body banks"
        );
        assert_eq!(
            WARDEN_EMERGE_HEAD_ROT[3].v,
            [-67.5, 0.0, -2.5],
            "the -67.5 struggle dip the head banks"
        );
        assert_eq!(
            WARDEN_EMERGE_HEAD_ROT[14].v,
            [80.0, 0.0, 12.5],
            "the 80-degree shake the head answers"
        );
        assert_eq!(
            WARDEN_EMERGE_RIGHT_ARM_ROT[2].v,
            [-152.5, 2.5, 7.5],
            "the -152.5 grab the right arm banks"
        );
        assert_eq!(
            WARDEN_EMERGE_LEFT_ARM_ROT[1].v,
            [-167.5, -17.5, -7.5],
            "the -167.5 grab the left arm banks"
        );
        assert_eq!(
            WARDEN_EMERGE_RIGHT_LEG_ROT[6].v,
            [55.0, 0.0, 0.0],
            "the 55-degree leg kick the climb banks"
        );
        assert_eq!(
            WARDEN_EMERGE_RIGHT_LEG_POS[0].v,
            [0.0, -63.0, 0.0],
            "the legs the body's own depth the rise shares"
        );
        // THE REST TAIL (every channel's final keyframe the (0,0,0)
        // rest banks — the window's own 6.64s tail the 6.68s length
        // the rest keeps)
        for (name, kfs) in [
            ("body rot", &WARDEN_EMERGE_BODY_ROT[..]),
            ("body pos", &WARDEN_EMERGE_BODY_POS[..]),
            ("head rot", &WARDEN_EMERGE_HEAD_ROT[..]),
            ("head pos", &WARDEN_EMERGE_HEAD_POS[..]),
            ("right arm rot", &WARDEN_EMERGE_RIGHT_ARM_ROT[..]),
            ("right arm pos", &WARDEN_EMERGE_RIGHT_ARM_POS[..]),
            ("left arm rot", &WARDEN_EMERGE_LEFT_ARM_ROT[..]),
            ("left arm pos", &WARDEN_EMERGE_LEFT_ARM_POS[..]),
            ("right leg rot", &WARDEN_EMERGE_RIGHT_LEG_ROT[..]),
            ("right leg pos", &WARDEN_EMERGE_RIGHT_LEG_POS[..]),
            ("left leg rot", &WARDEN_EMERGE_LEFT_LEG_ROT[..]),
            ("left leg pos", &WARDEN_EMERGE_LEFT_LEG_POS[..]),
        ] {
            let last = &kfs[kfs.len() - 1];
            assert_eq!(last.t, 6.64, "the {name} tail the definition's length keeps");
            assert_eq!(last.v, [0.0, 0.0, 0.0], "the {name} tail the rest banks");
        }
        // THE LENGTH (the withLength head the 6.68f speaks — the
        // 133.59999-tick float the Mth.ceil the 134-tick
        // EMERGE_DURATION banks)
        assert_eq!(
            crate::sculk::WARDEN_EMERGE_DURATION_TICKS,
            134,
            "the Mth.ceil(133.59999f) the window keeps"
        );
    }

    /// THE EMERGE DELTAS' OWN POSE WINDOW (the WardenEmergeDeltas the
    /// emerge_time serves — the None the all-zero no-op wears, the
    /// elapsed the *50/1000 two-step keeps, the posVec's y the fneg
    /// negates (the -63 raw the +63 baked words speak), the additive
    /// order the roar/sniff deltas the same tail wears, the layer
    /// quads the SAME deltas the shared setupAnim speaks).
    #[test]
    fn the_emerge_deltas_the_pose_window_banks() {
        // THE NO-OP (the never-armed None — the parts the rest pose
        // keeps: the +0.0 sums the folds' own numbers exactly wear)
        let calm = build_warden_parts(&WardenState::rest());
        let emerging = build_warden_parts(&WardenState {
            emerge_time: Some(0.0),
            dig_time: None,
            ..WardenState::rest()
        });
        let calm_body = &calm.children[0];
        let emerging_body = &emerging.children[0];
        // THE BURIED START (the elapsed-0 the definition's own first
        // keyframe the raw -63 wears — the posVec's y the fneg the
        // +63 baked speaks, the pivot the -21 + 63 the ground wears)
        assert_eq!(
            emerging_body.pivot[1],
            -21.0 + 63.0,
            "the elapsed-0 the buried start the +63 baked wears"
        );
        assert_eq!(
            calm_body.pivot[1],
            -21.0,
            "the calm body the rest pivot keeps"
        );
        // THE PLATEAU WALK (the elapsed 2.0s = 40 ticks the
        // [1.68, 2.28] rot plateau rides — the quartet the equal
        // neighbors the EXACT value the identity answers, the pos the
        // -32 plateau the same law keeps)
        let parts = build_warden_parts(&WardenState {
            emerge_time: Some(40.0),
            dig_time: None,
            ..WardenState::rest()
        });
        let body = &parts.children[0];
        let want_z = 10.0_f32 * 0.017_453_292_f32;
        assert!(
            (body.z_rot - want_z).abs() < 1e-5,
            "the 2.0s the zRot plateau the exact value wears: {:?} vs {:?}",
            body.z_rot,
            want_z
        );
        let want_y = -(-32.0_f32); // the posVec's y-neg bake
        assert!(
            (body.pivot[1] - (-21.0 + want_y)).abs() < 1e-5,
            "the 2.0s the pos plateau the -32 baked wears: {:?}",
            body.pivot[1]
        );
        // THE ADDITIVITY (the emerging WALKER — the deltas the walk
        // folds' own tail ADDS, the setupAnim order the emerge apply
        // the folds' own tail keeps; the walk fold the xRot the body
        // wears, the zRot the delta ALONE carries — the x the 0
        // plateau the folds' own word the exact sum keeps)
        let walker = WardenState {
            walk_pos: 1.0,
            walk_speed: 0.5,
            ..WardenState::rest()
        };
        let emerging_walker = WardenState {
            emerge_time: Some(40.0),
            dig_time: None,
            ..walker
        };
        let calm_parts = build_warden_parts(&walker);
        let emerge_parts = build_warden_parts(&emerging_walker);
        let calm_body = &calm_parts.children[0];
        let emerge_body = &emerge_parts.children[0];
        let f4 = 1.0_f32 * 0.8662;
        let damp = 0.5_f32.min(3.0 * 0.5);
        let walk_fold = mth_cos(f4) * 0.35_f32.min(damp) + 0.025;
        assert_eq!(
            emerge_body.x_rot,
            walk_fold,
            "the emerge's 0-x plateau the walk fold the exact sum keeps"
        );
        assert_eq!(calm_body.x_rot, walk_fold, "the calm walker the folds alone speak");
        assert_eq!(
            emerge_body.z_rot,
            calm_body.z_rot + want_z,
            "the zRot the delta the walk's own z the additive split keeps"
        );
        assert_eq!(
            emerge_body.pivot[1],
            -21.0 + want_y,
            "the pivot the delta carries — the walk the pivot never moves"
        );
        // THE LEGS THE SAME DEPTH (the right leg the body's own -63
        // start the pivot wears — the elapsed-0 the +63 baked the leg
        // pivot keeps)
        let emerging = build_warden_parts(&WardenState {
            emerge_time: Some(0.0),
            dig_time: None,
            ..WardenState::rest()
        });
        let right_leg = &emerging.children[1];
        assert_eq!(
            right_leg.pivot[1],
            -13.0 + 63.0,
            "the leg the body's own buried depth wears"
        );
        // THE LAYER QUADS THE SAME DELTAS (the biolum layer the
        // legs the mesh keeps — the emerging stub the same delta
        // answers)
        let feet = Vec3::new(3.0, 20.0, -7.0);
        let calm_layer =
            warden_layer_quads(&WardenState::rest(), feet, 0.0, 0.0, WardenLayer::Bioluminescent);
        let emerge_layer = warden_layer_quads(
            &WardenState {
                emerge_time: Some(14.0),
                dig_time: None,
                ..WardenState::rest()
            },
            feet,
            0.0,
            0.0,
            WardenLayer::Bioluminescent,
        );
        assert_eq!(calm_layer.len(), emerge_layer.len(), "the layer the mesh keeps");
        let moved: f32 = emerge_layer
            .iter()
            .zip(calm_layer.iter())
            .map(|(s, r)| {
                s.pts
                    .iter()
                    .zip(r.pts.iter())
                    .map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs())
                    .sum::<f32>()
            })
            .sum();
        assert!(moved > 0.5, "the emerge the layer quads move: {moved}");
    }

    /// THE WARDEN_DIG DEFINITION (javap `WardenAnimation` clinit pc
    /// 4841-7765): the twelve channels the 19/11, 7/2, 10/5, 12/8,
    /// 9/8, 9/8 arrays bank, the spot constants the ldc rows wear,
    /// the NINE settle keyframes the LINEAR choice bank, the
    /// withLength(5.0f) the final keyframes' own timestamps keep.
    #[test]
    fn the_dig_definition_the_jar_banks() {
        // THE CHANNEL COUNTS (the anewarray sizes the bytecode
        // speaks)
        assert_eq!(WARDEN_DIG_BODY_ROT.len(), 19, "the body's rot bank");
        assert_eq!(WARDEN_DIG_BODY_POS.len(), 11, "the body's pos bank");
        assert_eq!(WARDEN_DIG_HEAD_ROT.len(), 7, "the head's rot bank");
        assert_eq!(WARDEN_DIG_HEAD_POS.len(), 2, "the head's pos bank");
        assert_eq!(WARDEN_DIG_RIGHT_ARM_ROT.len(), 10, "the right arm's rot bank");
        assert_eq!(WARDEN_DIG_RIGHT_ARM_POS.len(), 5, "the right arm's pos bank");
        assert_eq!(WARDEN_DIG_LEFT_ARM_ROT.len(), 12, "the left arm's rot bank");
        assert_eq!(WARDEN_DIG_LEFT_ARM_POS.len(), 8, "the left arm's pos bank");
        assert_eq!(WARDEN_DIG_RIGHT_LEG_ROT.len(), 9, "the right leg's rot bank");
        assert_eq!(WARDEN_DIG_RIGHT_LEG_POS.len(), 8, "the right leg's pos bank");
        assert_eq!(WARDEN_DIG_LEFT_LEG_ROT.len(), 9, "the left leg's rot bank");
        assert_eq!(WARDEN_DIG_LEFT_LEG_POS.len(), 8, "the left leg's pos bank");
        // THE SPOT CONSTANTS (the ldc float rows the clinit wears)
        assert_eq!(WARDEN_DIG_BODY_ROT[2].v, [50.0, 0.0, 0.0], "the 0.5s lunge");
        assert_eq!(
            WARDEN_DIG_BODY_ROT[8].v,
            [112.283, 0.0, 0.0],
            "the 2.542s dive pair the body wears"
        );
        assert_eq!(
            WARDEN_DIG_BODY_ROT[18].v,
            [147.5, 0.0, 0.0],
            "the 4.5s settle the head-down dive keeps"
        );
        assert_eq!(
            WARDEN_DIG_BODY_POS[10].v,
            [0.0, -40.0, -8.0],
            "the 4.5s sink the body's own grave wears"
        );
        assert_eq!(WARDEN_DIG_HEAD_ROT[5].v, [67.5, 0.0, 0.0], "the head's dig glance");
        assert_eq!(
            WARDEN_DIG_RIGHT_ARM_ROT[1].v,
            [-101.804, -21.2959, 30.6148],
            "the right arm's dig pair"
        );
        assert_eq!(
            WARDEN_DIG_LEFT_ARM_ROT[9].v,
            [-153.275, -0.02953, 3.5235],
            "the left arm's deep grab"
        );
        assert_eq!(WARDEN_DIG_RIGHT_LEG_ROT[1].v, [113.27, 0.0, 0.0], "the leg's burrow kick");
        assert_eq!(WARDEN_DIG_LEFT_LEG_ROT[1].v, [114.98, 0.0, 0.0], "the left kick the anti-phase keeps");
        assert_eq!(
            WARDEN_DIG_RIGHT_LEG_POS[7].v,
            [0.0, -28.0, -5.0],
            "the settle drag the leg wears"
        );
        // THE LINEAR SETTLES (the `getstatic #273 LINEAR` rows the
        // NINE final keyframes wear)
        let lin_count = [
            &WARDEN_DIG_BODY_ROT[..],
            &WARDEN_DIG_BODY_POS[..],
            &WARDEN_DIG_HEAD_ROT[..],
            &WARDEN_DIG_HEAD_POS[..],
            &WARDEN_DIG_RIGHT_ARM_ROT[..],
            &WARDEN_DIG_RIGHT_ARM_POS[..],
            &WARDEN_DIG_LEFT_ARM_ROT[..],
            &WARDEN_DIG_LEFT_ARM_POS[..],
            &WARDEN_DIG_RIGHT_LEG_ROT[..],
            &WARDEN_DIG_RIGHT_LEG_POS[..],
            &WARDEN_DIG_LEFT_LEG_ROT[..],
            &WARDEN_DIG_LEFT_LEG_POS[..],
        ]
        .iter()
        .map(|c| c.iter().filter(|kf| kf.lin).count())
        .sum::<usize>();
        assert_eq!(lin_count, 9, "the NINE LINEAR settles the definition banks");
        assert!(WARDEN_DIG_BODY_ROT[18].lin, "the body's settle the LINEAR wears");
        assert!(!WARDEN_DIG_BODY_ROT[17].lin, "the mid kfs the CATMULLROM keeps");
        // THE TIMESTAMPS ASCEND (the binary-search walk the ordered
        // arrays the pred consults — the monotone the every channel
        // keeps) + THE WITHLENGTH (the 5.0f the final kf the
        // timestamp never exceeds)
        for (name, c) in [
            ("body rot", &WARDEN_DIG_BODY_ROT[..]),
            ("body pos", &WARDEN_DIG_BODY_POS[..]),
            ("head rot", &WARDEN_DIG_HEAD_ROT[..]),
            ("head pos", &WARDEN_DIG_HEAD_POS[..]),
            ("right arm rot", &WARDEN_DIG_RIGHT_ARM_ROT[..]),
            ("right arm pos", &WARDEN_DIG_RIGHT_ARM_POS[..]),
            ("left arm rot", &WARDEN_DIG_LEFT_ARM_ROT[..]),
            ("left arm pos", &WARDEN_DIG_LEFT_ARM_POS[..]),
            ("right leg rot", &WARDEN_DIG_RIGHT_LEG_ROT[..]),
            ("right leg pos", &WARDEN_DIG_RIGHT_LEG_POS[..]),
            ("left leg rot", &WARDEN_DIG_LEFT_LEG_ROT[..]),
            ("left leg pos", &WARDEN_DIG_LEFT_LEG_POS[..]),
] {
            for w in c.windows(2) {
                assert!(
                    w[0].t < w[1].t,
                    "the {name} timestamps ascend: {} vs {}",
                    w[0].t,
                    w[1].t
                );
            }
            assert!(
                c.last().expect("the non-empty bank").t <= 5.0,
                "the {name} tail the withLength(5.0f) keeps"
            );
        }
    }

    /// THE DIG'S OWN CHANNEL WALK (javap `KeyframeAnimation$Entry
    /// .apply` + the END keyframe's own interpolation): the LINEAR
    /// settle the plain lerp keeps, the CATMULLROM segments the
    /// quartet the bytecode's own f32 order replicates, the past-end
    /// walk the final LINEAR kf the rest wears, the pos y-neg bake
    /// the sink serves.
    #[test]
    fn the_dig_channel_the_linear_the_catmullrom_keeps() {
        // THE ELAPSED-0 (the fresh start the (0,0,0) rest keeps)
        let zero = warden_dig_channel(&WARDEN_DIG_BODY_ROT, 0.0, false);
        assert_eq!(zero, [0.0, 0.0, 0.0], "the elapsed-0 the rest keeps");
        // THE CATMULLROM SEGMENT (the [1.333, 1.625] rot pair rides —
        // elapsed 1.4 the delta walk the quartet answers, the
        // bytecode's own f32 order the inline chain replicates)
        let got = warden_dig_channel(&WARDEN_DIG_BODY_ROT, 1.4, false);
        let deg = 0.017_453_292_f32;
        let p0 = WARDEN_DIG_BODY_ROT[4].v[0] * deg;
        let p1 = WARDEN_DIG_BODY_ROT[5].v[0] * deg;
        let p2 = WARDEN_DIG_BODY_ROT[6].v[0] * deg;
        let p3 = WARDEN_DIG_BODY_ROT[7].v[0] * deg;
        let d = ((1.4 - WARDEN_DIG_BODY_ROT[5].t)
            / (WARDEN_DIG_BODY_ROT[6].t - WARDEN_DIG_BODY_ROT[5].t))
            .clamp(0.0, 1.0);
        let want = mth_catmullrom(d, p0, p1, p2, p3);
        assert!(
            (got[0] - want).abs() < 1e-6,
            "the catmullrom segment the quartet wears: {:?} vs {want:?}",
            got[0]
        );
        // THE LINEAR SETTLE (the [4.25, 4.5] rot pair the END kf's
        // own LINEAR choice wears — elapsed 4.375 the delta 0.5 the
        // plain lerp answers, the quartet the settle NEVER reads)
        let got = warden_dig_channel(&WARDEN_DIG_BODY_ROT, 4.375, false);
        let start = WARDEN_DIG_BODY_ROT[17].v[0] * deg;
        let end = WARDEN_DIG_BODY_ROT[18].v[0] * deg;
        let want = start + (end - start) * 0.5;
        assert!(
            (got[0] - want).abs() < 1e-6,
            "the LINEAR settle the plain lerp wears: {:?} vs {want:?}",
            got[0]
        );
        // THE ENDPOINT (the elapsed the final kf's own timestamp the
        // delta 1.0 answers — the settle the exact value wears)
        let got = warden_dig_channel(&WARDEN_DIG_BODY_ROT, 4.5, false);
        assert!(
            (got[0] - end).abs() < 1e-6,
            "the endpoint the settle value keeps: {:?}",
            got[0]
        );
        // THE PAST-END REST (the j==i the delta 0 keeps — the final
        // LINEAR kf the rest wears, the walk the never-outrun law)
        let got = warden_dig_channel(&WARDEN_DIG_BODY_ROT, 5.0, false);
        assert!(
            (got[0] - end).abs() < 1e-6,
            "the past-end walk the rest pose keeps: {:?}",
            got[0]
        );
        // THE POS Y-NEG BAKE (the posVec's own fneg — the sink the
        // (0,-40,-8) the +40 baked wears)
        let got = warden_dig_channel(&WARDEN_DIG_BODY_POS, 4.5, true);
        assert_eq!(got, [0.0, 40.0, -8.0], "the sink the y-neg bake wears");
    }

    /// THE DIG'S OWN DELTAS (the WARDEN_DIG definition the 12
    /// channels evaluate): the None the all-zero no-op keeps, the
    /// mid-dig the dive the parts wear, the additivity the walker's
    /// own fold the sum keeps, the layer the same deltas the shared
    /// walk speaks.
    #[test]
    fn the_dig_deltas_the_pose_window_banks() {
        // THE NO-OP (the never-armed None — the parts the rest pose
        // keeps: the +0.0 sums the folds' own numbers exactly wear)
        let calm = build_warden_parts(&WardenState::rest());
        let digging = build_warden_parts(&WardenState {
            dig_time: Some(0.0),
            ..WardenState::rest()
        });
        let calm_body = &calm.children[0];
        let dig_body = &digging.children[0];
        // THE START (the elapsed-0 the definition's own first
        // keyframe the (0,0,0) rest wears — the pivot the folds the
        // same numbers keep)
        assert_eq!(
            dig_body.pivot[1], calm_body.pivot[1],
            "the elapsed-0 the rest start the same pivot wears"
        );
        // THE DIVE (the elapsed 2.5s the [2.208, 2.542] segment the
        // catmullrom walk answers — the xRot the deg-baked wears,
        // the elapsed the *50/1000 two-step derives)
        let parts = build_warden_parts(&WardenState {
            dig_time: Some(50.0),
            ..WardenState::rest()
        });
        let body = &parts.children[0];
        let deg = 0.017_453_292_f32;
        let d = ((2.5 - WARDEN_DIG_BODY_ROT[7].t)
            / (WARDEN_DIG_BODY_ROT[8].t - WARDEN_DIG_BODY_ROT[7].t))
            .clamp(0.0, 1.0);
        let p0 = WARDEN_DIG_BODY_ROT[6].v[0] * deg;
        let p1 = WARDEN_DIG_BODY_ROT[7].v[0] * deg;
        let p2 = WARDEN_DIG_BODY_ROT[8].v[0] * deg;
        let p3 = WARDEN_DIG_BODY_ROT[9].v[0] * deg;
        let want = mth_catmullrom(d, p0, p1, p2, p3);
        // the idle sway's own +0.025 the age-0 clock wears — the
        // cos(0) the fold's ahead word banks (the same law the sniff
        // deltas' tests keep)
        assert!(
            (body.x_rot - 0.025 - want).abs() < 1e-5,
            "the 2.5s the dive the catmullrom wears: {:?} vs {want:?}",
            body.x_rot - 0.025
        );
        // THE SINK (the elapsed 5.0s the past-end rest the (0,+40,-8)
        // baked wears — the pivot the -21 + 40 the grave speaks)
        let sunk = build_warden_parts(&WardenState {
            dig_time: Some(100.0),
            ..WardenState::rest()
        });
        let body = &sunk.children[0];
        assert!(
            (body.pivot[1] - (-21.0 + 40.0)).abs() < 1e-5,
            "the sink the +40 baked wears: {:?}",
            body.pivot[1]
        );
        // THE ADDITIVITY (the digging WALKER — the deltas the walk
        // folds' own tail ADDS, the xRot the exact sum the
        // floats keep)
        let walker = WardenState {
            walk_pos: 1.0,
            walk_speed: 0.5,
            ..WardenState::rest()
        };
        let digging_walker = WardenState {
            dig_time: Some(50.0),
            ..walker
        };
        let plain = build_warden_parts(&walker);
        let dug = build_warden_parts(&digging_walker);
        let fold = plain.children[0].x_rot;
        let want = fold + want;
        assert!(
            (dug.children[0].x_rot - want).abs() < 1e-5,
            "the walker's own fold the dig deltas the sum wears"
        );
        // THE LEGS THE SAME DEPTH (the settle drag the leg pivots
        // wear — the y the -13 + 28 the grave keeps)
        let legs_right = &sunk.children[1];
        assert!(
            (legs_right.pivot[1] - (-13.0 + 28.0)).abs() < 1e-5,
            "the settle drag the leg pivot wears: {:?}",
            legs_right.pivot[1]
        );
        // THE BIOLUM LAYER THE SAME DELTAS (the layer models' own
        // setupAnim the SAME walk serves — the head pivot the body
        // build's own number keeps)
        let feet = crate::math::Vec3::new(10.5, 32.0, 3.5);
        let dug_layer = warden_layer_quads(
            &digging_walker,
            feet,
            0.0,
            0.0,
            WardenLayer::Bioluminescent,
        );
        let calm_layer = warden_layer_quads(
            &walker,
            feet,
            0.0,
            0.0,
            WardenLayer::Bioluminescent,
        );
        assert_eq!(calm_layer.len(), dug_layer.len(), "the layer the mesh keeps");
        let moved: f32 = dug_layer
            .iter()
            .zip(calm_layer.iter())
            .map(|(s, r)| {
                s.pts
                    .iter()
                    .zip(r.pts.iter())
                    .map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs())
                    .sum::<f32>()
            })
            .sum();
        assert!(moved > 0.5, "the dig the layer quads move: {moved}");
    }

    /// THE WARDEN_ATTACK DEFINITION (javap `WardenAnimation` clinit
    /// pc 9801-10684 — the `withLength(0.33333f)` head, the FOUR
    /// bones the addAnimation chain names, each its ROTATION +
    /// POSITION pair the 4/4 arrays bank, EVERY keyframe the
    /// CATMULLROM interpolation wears — the plain `WardenKeyframe`
    /// record the ROAR/EMERGE channels share): the counts the
    /// anewarray sizes keep, the spot constants the ldc rows bank,
    /// the timestamps the ascending window the melee-swing beats
    /// wear — the wind-up 0.0417s, the strikes 0.1667-0.25s, the
    /// rest 0.3333s.
    #[test]
    fn the_attack_definition_the_jar_banks() {
        // THE CHANNEL COUNTS (the anewarray sizes the bytecode
        // speaks — the FOUR bones the rot+pos pairs bank)
        assert_eq!(WARDEN_ATTACK_BODY_ROT.len(), 4, "the body's rot bank");
        assert_eq!(WARDEN_ATTACK_BODY_POS.len(), 4, "the body's pos bank");
        assert_eq!(WARDEN_ATTACK_HEAD_ROT.len(), 4, "the head's rot bank");
        assert_eq!(WARDEN_ATTACK_HEAD_POS.len(), 4, "the head's pos bank");
        assert_eq!(WARDEN_ATTACK_RIGHT_ARM_ROT.len(), 4, "the right arm's rot bank");
        assert_eq!(WARDEN_ATTACK_RIGHT_ARM_POS.len(), 4, "the right arm's pos bank");
        assert_eq!(WARDEN_ATTACK_LEFT_ARM_ROT.len(), 4, "the left arm's rot bank");
        assert_eq!(WARDEN_ATTACK_LEFT_ARM_POS.len(), 4, "the left arm's pos bank");
        // THE SPOT CONSTANTS (the ldc float rows the clinit wears)
        assert_eq!(
            WARDEN_ATTACK_BODY_ROT[1].v,
            [-22.5, 0.0, 0.0],
            "the 0.0417s wind-up the body dip wears"
        );
        assert_eq!(
            WARDEN_ATTACK_BODY_ROT[2].v,
            [22.5, 0.0, 0.0],
            "the 0.2083s strike the body lunge keeps"
        );
        assert_eq!(
            WARDEN_ATTACK_BODY_POS[2].v,
            [0.0, -1.0, -2.0],
            "the strike drop the body pos wears"
        );
        assert_eq!(
            WARDEN_ATTACK_HEAD_ROT[1].v,
            [22.5, 0.0, 0.0],
            "the head's wind-up dip the anti-phase wears"
        );
        assert_eq!(
            WARDEN_ATTACK_HEAD_ROT[2].v,
            [-30.17493, 0.0, 0.0],
            "the head's 0.25s recover keeps"
        );
        assert_eq!(
            WARDEN_ATTACK_HEAD_POS[2].v,
            [0.0, -2.0, -2.0],
            "the head drop the 0.25s beat wears"
        );
        assert_eq!(
            WARDEN_ATTACK_RIGHT_ARM_ROT[1].v,
            [-120.36119, 40.78947, -20.94102],
            "the right arm's wind-up grab trio"
        );
        assert_eq!(
            WARDEN_ATTACK_RIGHT_ARM_ROT[2].v,
            [-90.0, -45.0, 0.0],
            "the right arm's 0.1667s smash keeps"
        );
        assert_eq!(
            WARDEN_ATTACK_RIGHT_ARM_POS[1].v,
            [4.0, 0.0, 5.0],
            "the right arm's 0.0417s reach wears"
        );
        assert_eq!(
            WARDEN_ATTACK_LEFT_ARM_ROT[1].v,
            [-120.36119, -40.78947, 20.94102],
            "the left arm's wind-up grab the mirror trio keeps"
        );
        assert_eq!(
            WARDEN_ATTACK_LEFT_ARM_ROT[2].v,
            [-61.1632, 42.85882, 11.52421],
            "the left arm's 0.1667s cross wears"
        );
        assert_eq!(
            WARDEN_ATTACK_LEFT_ARM_POS[1].v,
            [-4.0, 0.0, 5.0],
            "the left arm's 0.0417s reach the mirror keeps"
        );
        // THE REST TAILS (the 0.3333 final keyframes the (0,0,0)
        // zeros the window's own end banks — ALL EIGHT channels the
        // same rest wears)
        for (name, ch) in [
            ("body rot", &WARDEN_ATTACK_BODY_ROT[..]),
            ("body pos", &WARDEN_ATTACK_BODY_POS[..]),
            ("head rot", &WARDEN_ATTACK_HEAD_ROT[..]),
            ("head pos", &WARDEN_ATTACK_HEAD_POS[..]),
            ("right arm rot", &WARDEN_ATTACK_RIGHT_ARM_ROT[..]),
            ("right arm pos", &WARDEN_ATTACK_RIGHT_ARM_POS[..]),
            ("left arm rot", &WARDEN_ATTACK_LEFT_ARM_ROT[..]),
            ("left arm pos", &WARDEN_ATTACK_LEFT_ARM_POS[..]),
        ] {
            assert_eq!(
                ch[3].v,
                [0.0, 0.0, 0.0],
                "the {name} rest the window's own end banks"
            );
            assert_eq!(ch[3].t, 0.3333, "the {name} tail the 0.3333s window keeps");
            // THE ASCENDING TIMESTAMPS (the builder's own order the
            // binary-search walk consults)
            assert!(ch[0].t < ch[1].t && ch[1].t < ch[2].t && ch[2].t < ch[3].t,
                "the {name} timestamps the ascending order keeps");
        }
        // THE WIND-UP/STRIKE BEATS (the shared keyframe timestamps
        // the 0.0417/0.1667/0.2083/0.25 rows wear)
        assert_eq!(WARDEN_ATTACK_BODY_ROT[1].t, 0.0417, "the wind-up beat");
        assert_eq!(WARDEN_ATTACK_RIGHT_ARM_ROT[2].t, 0.1667, "the right smash beat");
        assert_eq!(WARDEN_ATTACK_BODY_ROT[2].t, 0.2083, "the body strike beat");
        assert_eq!(WARDEN_ATTACK_HEAD_ROT[2].t, 0.25, "the head recover beat");
    }

    /// THE ATTACK'S OWN DELTAS (the WARDEN_ATTACK definition the 8
    /// channels evaluate — the additive offsets the
    /// build_warden_parts + the layer walk ADDS, the ATTACK the
    /// setupAnim's own FIRST apply wears): the None no-op, the
    /// wind-up/strike window the catmullrom walk answers, the
    /// past-end rest the zeros the stale start keeps, the additivity
    /// the walker's own fold the sum wears, the legs the attack
    /// never moves, the biolum layer the same deltas speaks.
    #[test]
    fn the_attack_deltas_the_pose_window_banks() {
        let deg = 0.017_453_292_f32;
        // THE NO-OP (the never-armed None — the parts the rest pose
        // keeps: the +0.0 sums the folds' own numbers exactly wear)
        let calm = build_warden_parts(&WardenState::rest());
        let started = build_warden_parts(&WardenState {
            attack_time: Some(0.0),
            ..WardenState::rest()
        });
        let calm_body = &calm.children[0];
        let start_body = &started.children[0];
        assert_eq!(
            start_body.pivot[1], calm_body.pivot[1],
            "the elapsed-0 the rest start the same pivot wears"
        );
        // THE WIND-UP (the elapsed 0.0417s = 0.834 ticks the
        // catmullrom's own overshoot wears — the segment the
        // [0.0, 0.0417] window banks, the quartet's f32 order the
        // vanilla Entry.apply walk keeps; the elapsed the *50/1000
        // two-step derives)
        let elapsed = 0.834_f32;
        let secs = (elapsed * 50.0) / 1000.0;
        let d = (secs / WARDEN_ATTACK_BODY_ROT[1].t).clamp(0.0, 1.0);
        let want_x = mth_catmullrom(d, 0.0, 0.0, -22.5 * deg, 22.5 * deg);
        let swung = build_warden_parts(&WardenState {
            attack_time: Some(elapsed),
            ..WardenState::rest()
        });
        let body = &swung.children[0];
        // the idle sway's own +0.025 the age-0 clock wears (the same
        // law the dig deltas' test keeps)
        assert!(
            (body.x_rot - 0.025 - want_x).abs() < 1e-5,
            "the wind-up the body dip the catmullrom wears: {:?} vs {want_x:?}",
            body.x_rot - 0.025
        );
        // THE PAST-END REST (the elapsed 10 ticks the 0.3333s tail
        // outlasts — the deltas the (0,0,0) zeros the stale start
        // keeps, the pivot the folds' own numbers wear)
        let stale = build_warden_parts(&WardenState {
            attack_time: Some(10.0),
            ..WardenState::rest()
        });
        let calm_parts = build_warden_parts(&WardenState::rest());
        assert_eq!(
            stale.children[0].pivot,
            calm_parts.children[0].pivot,
            "the past-end rest the stale start the zeros keeps"
        );
        // THE ADDITIVITY (the attacking WALKER — the deltas the walk
        // folds' own tail ADDS, the strike's own xRot the exact sum
        // the floats keep)
        let walker = WardenState {
            walk_pos: 1.0,
            walk_speed: 0.5,
            ..WardenState::rest()
        };
        let striking_walker = WardenState {
            attack_time: Some(elapsed),
            ..walker.clone()
        };
        let plain = build_warden_parts(&walker);
        let struck = build_warden_parts(&striking_walker);
        let fold = plain.children[0].x_rot;
        assert!(
            (struck.children[0].x_rot - (fold + want_x)).abs() < 1e-5,
            "the walker's own fold the attack deltas the sum wears"
        );
        // THE RIGHT ARM'S WIND-UP GRAB (the elapsed-0.834 the
        // [0.0, 0.0417] segment the trio's own catmullrom walk
        // answers — the x component the -120.36° bank wears)
        let want_arm_x = mth_catmullrom(d, 0.0, 0.0, -120.36119 * deg, -90.0 * deg);
        let arm = &struck.children[0].children[3];
        let arm_fold = &plain.children[0].children[3];
        assert!(
            (arm.x_rot - (arm_fold.x_rot + want_arm_x)).abs() < 1e-5,
            "the wind-up the right arm's grab the sum wears: {:?} vs {want_arm_x:?}",
            arm.x_rot - arm_fold.x_rot
        );
        // THE ARM POS DELTA (the [4, 0, 5] reach the pivot ADDS —
        // the elapsed 0.834 the segment's own d wears)
        let want_pos = [
            mth_catmullrom(d, 0.0, 0.0, 4.0, 0.0),
            mth_catmullrom(d, 0.0, 0.0, 0.0, 0.0),
            mth_catmullrom(d, 0.0, 0.0, 5.0, 0.0),
        ];
        for i in 0..3 {
            let got = arm.pivot[i] - arm_fold.pivot[i];
            assert!(
                (got - want_pos[i]).abs() < 1e-5,
                "the arm pos delta the [{i}] the reach wears: {got:?} vs {:?}",
                want_pos[i]
            );
        }
        // THE LEGS THE ATTACK NEVER MOVES (the def the FOUR bones
        // names — the legs the walker's own pivots the exact words
        // keep)
        assert_eq!(
            struck.children[1].pivot,
            plain.children[1].pivot,
            "the right leg the attack never moves"
        );
        assert_eq!(
            struck.children[2].pivot,
            plain.children[2].pivot,
            "the left leg the attack never moves"
        );
        // THE BIOLUM LAYER THE SAME DELTAS (the layer models' own
        // setupAnim the SAME walk serves — the attack the layer
        // quads move)
        let feet = crate::math::Vec3::new(10.5, 32.0, 3.5);
        let struck_layer = warden_layer_quads(
            &striking_walker,
            feet,
            0.0,
            0.0,
            WardenLayer::Bioluminescent,
        );
        let plain_layer = warden_layer_quads(
            &walker,
            feet,
            0.0,
            0.0,
            WardenLayer::Bioluminescent,
        );
        assert_eq!(plain_layer.len(), struck_layer.len(), "the layer the mesh keeps");
        let moved: f32 = struck_layer
            .iter()
            .zip(plain_layer.iter())
            .map(|(s, r)| {
                s.pts
                    .iter()
                    .zip(r.pts.iter())
                    .map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs())
                    .sum::<f32>()
            })
            .sum();
        assert!(moved > 0.5, "the attack the layer quads move: {moved}");
    }

    impl WardenState {
        /// THE STANDING REST (the zero pair the fresh warden's
        /// stillness wears — the look pair zero, the walk pair zero,
        /// the clocks stopped, the age the zero tick keeps).
        fn rest() -> WardenState {
            WardenState {
                yaw_deg: 0.0,
                pitch_deg: 0.0,
                head_yaw_off_deg: 0.0,
                walk_pos: 0.0,
                walk_speed: 0.0,
                age_ticks: 0.0,
                tendril_animation: 0.0,
                roar_time: None,
                sniff_time: None,
                emerge_time: None,
                dig_time: None,
                attack_time: None,
            }
        }
    }


    /// THE EMISSIVE LAYERS' OWN RETAIN LAW (javap
    /// `PartDefinition.retainExactParts` the 26.2 jar speaks): the
    /// NAMED parts keep their cubes, the unnamed parts the pose-only
    /// stubs become — the per-layer quad counts the cube sets wear
    /// (the heart the body cube ONLY, the tendrils the zero-width
    /// pair's two faces EACH, the bioluminescent the five cubes, the
    /// spots the body + the five limbs), the FULLBRIGHT shade the
    /// eyes shader's no-diffuse law keeps, the transform chain the
    /// stub poses preserve (the heart's body quads the full mesh's
    /// own body quads the EXACT points answer).
    #[test]
    fn the_warden_layers_the_retain_law_keeps() {
        let state = WardenState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 40.0,
            tendril_animation: 0.5,
            roar_time: None,
            sniff_time: None,
            emerge_time: None,
            dig_time: None,
            attack_time: None,
        };
        let feet = Vec3::new(3.0, 20.0, -7.0);
        // THE HEART (Set.of("body")): the body cube's six faces the
        // ONLY quads the pruned mesh keeps
        let heart = warden_layer_quads(&state, feet, 0.0, 0.0, WardenLayer::Heart);
        assert_eq!(heart.len(), 6, "the heart layer the body cube alone keeps");
        // THE TENDRILS (Set.of(left, right)): the TWO zero-width
        // planes the six-face table each wears (the degenerate faces
        // the raster's own area check culls — the LIST the twelve
        // entries keeps)
        let tendrils = warden_layer_quads(&state, feet, 0.0, 0.0, WardenLayer::Tendrils);
        assert_eq!(tendrils.len(), 12, "the tendril layer the pair's cubes keep");
        // THE BIOLUMINESCENT (head + arms + legs): the five cubes the
        // thirty faces wear (the body the stub drops, the ribcages
        // NO layer keeps)
        let biolum =
            warden_layer_quads(&state, feet, 0.0, 0.0, WardenLayer::Bioluminescent);
        assert_eq!(biolum.len(), 30, "the biolum layer the five cubes keep");
        // THE SPOTS (body + head + arms + legs): the SIX cubes the
        // thirty-six faces wear (the retain set the six names bank —
        // the ribcages no set keeps)
        let spots1 =
            warden_layer_quads(&state, feet, 0.0, 0.0, WardenLayer::PulsatingSpots1);
        let spots2 =
            warden_layer_quads(&state, feet, 0.0, 0.0, WardenLayer::PulsatingSpots2);
        assert_eq!(spots1.len(), 36, "the spots layer the six cubes keep");
        assert_eq!(spots2.len(), 36, "the spots-2 layer the same mesh wears");
        // THE FULLBRIGHT (the eyes shader's no-diffuse law): every
        // layer quad's shade the 1.0 keeps
        for q in biolum.iter().chain(heart.iter()) {
            assert_eq!(q.shade, 1.0, "the layer quads the fullbright shade keep");
        }
        // THE TRANSFORM CHAIN (the stub poses preserve): the heart's
        // body quads the FULL mesh's body quads the same world points
        // wear — the body is the full tree's FIRST cube, its six
        // quads the chain's head
        let full = {
            let model_parts = build_warden_parts(&state);
            let (fold_point, fold_normal) = entity_folds(state.yaw_deg, 0.0, 0.0, feet);
            let mut out: Vec<AvatarQuad> = Vec::new();
            let ident = Mat4::IDENT;
            collect_part(&model_parts, &ident, &fold_point, &fold_normal, 0.0, 0.0, &mut out);
            out
        };
        for (l, f) in heart.iter().zip(full.iter()) {
            assert_eq!(l.pts, f.pts, "the heart layer's body the full mesh's transform rides");
        }
        // THE SHIVER RIDE (the tendril stubs the folds serve): the
        // filled shiver clock the tendril quads MOVE the drained
        // stillness never wears
        let shiver_state = WardenState { tendril_animation: 1.0, ..state };
        let moved =
            warden_layer_quads(&shiver_state, feet, 0.0, 0.0, WardenLayer::Tendrils);
        let still = warden_layer_quads(&state, feet, 0.0, 0.0, WardenLayer::Tendrils);
        let total: f32 = moved
            .iter()
            .zip(still.iter())
            .map(|(m, s)| {
                m.pts
                    .iter()
                    .zip(s.pts.iter())
                    .map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs())
                    .sum::<f32>()
            })
            .sum();
        assert!(total > 0.2, "the shiver clock the tendril layer swings: {total}");
    }

    /// THE LAYERS' OWN ALPHA LAMBDAS (javap `WardenRenderer` the 26.2
    /// jar speaks — `lambda$new$1` the 1.0 const, `lambda$new$3` /
    /// `lambda$new$5` the `max(0.25, Mth.cos(age * 0.045 [+ PI]) *
    /// 0.25)` pair, `lambda$new$7` / `lambda$new$9` the clock words):
    /// the bioluminescent the constant keeps, the spot pair the
    /// max-0.25 floor CLAMPS (the cos product never exceeds the
    /// floor), the tendrils/heart the getters' own words wear.
    #[test]
    fn the_warden_layer_alphas_the_lambdas_wear() {
        let state = WardenState {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            head_yaw_off_deg: 0.0,
            walk_pos: 0.0,
            walk_speed: 0.0,
            age_ticks: 137.0,
            tendril_animation: 0.0,
            roar_time: None,
            sniff_time: None,
            emerge_time: None,
            dig_time: None,
            attack_time: None,
        };
        // THE CONST (the always-on glow)
        assert_eq!(
            WardenLayer::Bioluminescent.alpha_at(&state, 0.3, 0.7),
            1.0,
            "the biolum lambda the 1.0 const keeps"
        );
        // THE SPOT PAIR (the max-0.25 floor — the sweep the clamp
        // pins: cos ranges the [-1, 1] band, the product the [-0.25,
        // 0.25] one, the max the 0.25 floor keeps)
        for age in [0.0f32, 3.5, 7.0, 11.3, 40.0, 137.0, 900.0] {
            let s = WardenState { age_ticks: age, ..state };
            let a1 = WardenLayer::PulsatingSpots1.alpha_at(&s, 0.0, 0.0);
            let a2 = WardenLayer::PulsatingSpots2.alpha_at(&s, 0.0, 0.0);
            assert_eq!(a1, 0.25, "the spots-1 lambda the max floor keeps at {age}");
            assert_eq!(a2, 0.25, "the spots-2 lambda the max floor keeps at {age}");
        }
        // THE CLOCK WORDS (the pass-through the getters serve)
        assert_eq!(
            WardenLayer::Tendrils.alpha_at(&state, 0.55, 0.7),
            0.55,
            "the tendrils lambda the shiver word wears"
        );
        assert_eq!(
            WardenLayer::Heart.alpha_at(&state, 0.55, 0.7),
            0.7,
            "the heart lambda the heartbeat word wears"
        );
        // THE GATE WORDS (the drained clocks the 0.0 the `<= 1.0E-5`
        // skip keeps)
        assert_eq!(
            WardenLayer::Tendrils.alpha_at(&state, 0.0, 0.0),
            0.0,
            "the drained shiver the zero the gate refuses"
        );
        assert_eq!(
            WardenLayer::Heart.alpha_at(&state, 0.0, 0.0),
            0.0,
            "the drained heart the zero the gate refuses"
        );
    }

    /// THE LAYER TEXTURES' OWN LOADS (the sha1 the same honesty the
    /// body load keeps — the four emissive PNGs the 128x128 layout
    /// wears, the glow texels the alpha rows keep, the TENDRILS
    /// layer's own texture word the BASE warden.png the lambda bank
    /// speaks).
    #[test]
    fn the_warden_layer_textures_the_shas_keep() {
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join(skin::WARDEN_RUNTIME_PATH).exists());
        let Some(root) = root else { return };
        // THE FOUR LANE LOADS (each its own sha1 the load verifies)
        let heart = WardenModel::load_layer(root, WardenLayer::Heart);
        let biolum = WardenModel::load_layer(root, WardenLayer::Bioluminescent);
        let spots1 = WardenModel::load_layer(root, WardenLayer::PulsatingSpots1);
        let spots2 = WardenModel::load_layer(root, WardenLayer::PulsatingSpots2);
        let (Some(heart), Some(biolum), Some(spots1), Some(spots2)) =
            (heart, biolum, spots1, spots2)
        else {
            return; // the sha1 gate — the honest skip
        };
        // THE GLOW TEXELS (the emissive rows the layers blend): the
        // heart's chest dots, the biolum's spots, the spots pair's
        // own rows — each texture OPAQUE texels carries
        for (name, model) in [
            ("heart", &heart),
            ("biolum", &biolum),
            ("spots1", &spots1),
            ("spots2", &spots2),
        ] {
            let lit: usize = (0..128usize)
                .map(|v| {
                    (0..128usize)
                        .filter(|&u| model.pixel(u as f32, v as f32)[3] > 0)
                        .count()
                })
                .sum();
            assert!(lit > 0, "the {name} texture the glow texels keep");
        }
        // THE TENDRILS' OWN WORD (the BASE texture the layer wears —
        // the png_sha1 the body's own row answers, the runtime path
        // the None keeps)
        assert_eq!(
            WardenLayer::Tendrils.png_sha1(),
            Some(skin::WARDEN_PNG_SHA1),
            "the tendril layer the base texture's sha wears"
        );
        assert_eq!(WardenLayer::Tendrils.runtime_path(), None);
    }

}

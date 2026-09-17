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
//! - `HumanoidModel.setupAttackAnimation` (javap): the swing law —
//!   `body.yRot = 0.2 * sin(sqrt(attackTime) * 2π)` (negated when the
//!   attacking arm is the left), the attacking arm:
//!   `xRot -= sin(sqrt(attackTime) * π) * 1.2 + 0.75 * sin(attackTime
//!   * π) * (0.7 - head.xRot)`, `yRot += body.yRot * 2`, `zRot +=
//!   sin(attackTime * π) * -0.4`.
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

/// `Mth.cos(float)` — the f64 cos narrowed back to f32 (the Mth body).
fn mth_cos(x: f32) -> f32 {
    (x as f64).cos() as f32
}

/// `Mth.sin(float)` — the f64 sin narrowed back to f32.
fn mth_sin(x: f32) -> f32 {
    (x as f64).sin() as f32
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
    /// The head's yaw offset from the body (the netHeadYaw the
    /// LivingEntityRenderer extraction clamps to +-75°; the player
    /// avatar's body rides the look yaw so the offset stays 0).
    pub head_yaw_off_deg: f32,
    /// `LivingEntityRenderState.deathTime` (the entity's deathTime +
    /// partialTick when dying, 0 alive) — the flip law's clock.
    pub death_time: f32,
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
            head_yaw_off_deg: 0.0,
            death_time: 0.0,
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
        Some(AvatarModel { rgba })
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
        Some(AvatarModel { rgba })
    }

    /// The skin pixel at (u, v) — the RGBA row-major words, v down.
    pub fn pixel(&self, u: f32, v: f32) -> [u8; 4] {
        let side = skin::SKIN_SIDE as f32;
        let (uu, vv) = (u.clamp(0.0, side - 1e-4), v.clamp(0.0, side - 1e-4));
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
        let bytes = std::fs::read(root.join(skin::PIG_TEMPERATE_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::PIG_TEMPERATE_PNG_SHA1 {
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
        let parts = build_pig_parts(state);
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
fn build_pig_parts(state: &PigState) -> Vec<Part> {
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
        // offsetAndRotation speaks)
        Part {
            pivot: [0.0, 11.0, 0.0],
            x_rot: std::f32::consts::FRAC_PI_2,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![Cube {
                tex: (28.0, 8.0),
                origin: [-5.0, -10.0, -7.0],
                size: [10.0, 16.0, 8.0],
                inflate: 0.0,
                mirror: false,
            }],
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
        let parts = build_cow_parts(state);
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
fn build_cow_parts(state: &QuadrupedState) -> Vec<Part> {
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
        // chain the bytecode's addBox walk speaks)
        Part {
            pivot: [0.0, 4.0, -8.0],
            x_rot: head_x,
            y_rot: head_y,
            z_rot: 0.0,
            cubes: vec![
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
            children: vec![],
        },
        // body (the horizontal barrel — the rotX PI/2 the PartPose
        // offsetAndRotation speaks) + the udder (the same part's
        // second cube)
        Part {
            pivot: [0.0, 5.0, 2.0],
            x_rot: std::f32::consts::FRAC_PI_2,
            y_rot: 0.0,
            z_rot: 0.0,
            cubes: vec![
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
            ],
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
        let bytes = std::fs::read(root.join(skin::COW_TEMPERATE_BABY_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::COW_TEMPERATE_BABY_PNG_SHA1 {
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
        let bytes = std::fs::read(root.join(skin::PIG_TEMPERATE_BABY_RUNTIME_PATH)).ok()?;
        let digest = sha1::Sha1::digest(&bytes);
        if hex::encode(digest) != skin::PIG_TEMPERATE_BABY_PNG_SHA1 {
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
    let mut leg_r_y = 0.005f32;
    let leg_r_z = 0.005f32;
    let mut leg_l_y = -0.005f32;
    let leg_l_z = -0.005f32;
    // the pivot nudges the crouch speaks (model texels)
    let mut arm_r_pivot = [-5.0f32, 2.0, 0.0];
    let mut arm_l_pivot = [5.0f32, 2.0, 0.0];
    let mut body_pivot = [0.0f32, 0.0, 0.0];
    let mut head_pivot = [0.0f32, 0.0, 0.0];
    let mut leg_r_pivot = [-1.9f32, 12.0, 0.0];
    let mut leg_l_pivot = [1.9f32, 12.0, 0.0];
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
    // THE SWING (setupAttackAnimation): the body's yRot fold and the
    // attacking arm's WHACK arc
    let at = state.attack_time;
    if at > 0.0 {
        let f = mth_sin(mth_sqrt(at) * std::f32::consts::PI);
        let mut body_rot = 0.2 * mth_sin(mth_sqrt(at) * 6.2831855);
        if !state.attack_right {
            body_rot = -body_rot;
        }
        body_y = body_rot;
        let head_adjust = 0.75 * mth_sin(at * std::f32::consts::PI) * (0.7 - head_x);
        if state.attack_right {
            arm_r_x -= f * 1.2 + head_adjust;
            arm_r_y += body_y * 2.0;
            arm_r_z += mth_sin(at * std::f32::consts::PI) * -0.4;
        } else {
            arm_l_x -= f * 1.2 + head_adjust;
            arm_l_y += body_y * 2.0;
            arm_l_z += mth_sin(at * std::f32::consts::PI) * -0.4;
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
    // THE PARTS (createMesh, verbatim). The overlays are CHILDREN
    // (PartPose.ZERO — the parent's pivot space) with the SAME box
    // inflated by the extend.
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
        // right arm (+ right sleeve child)
        Part {
            pivot: arm_r_pivot,
            x_rot: arm_r_x,
            y_rot: arm_r_y,
            z_rot: arm_r_z,
            cubes: vec![Cube {
                tex: (40.0, 16.0),
                origin: [-3.0, -2.0, -2.0],
                size: [4.0, 12.0, 4.0],
                inflate: 0.0,
                        mirror: false,
                        }],
            children: vec![Part {
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
            }],
        },
        // left arm (+ left sleeve child)
        Part {
            pivot: arm_l_pivot,
            x_rot: arm_l_x,
            y_rot: arm_l_y,
            z_rot: arm_l_z,
            cubes: vec![Cube {
                tex: (32.0, 48.0),
                origin: [-1.0, -2.0, -2.0],
                size: [4.0, 12.0, 4.0],
                inflate: 0.0,
                        mirror: false,
                        }],
            children: vec![Part {
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
            }],
        },
        // right leg (+ right pants child)
        Part {
            pivot: leg_r_pivot,
            x_rot: leg_r_x,
            y_rot: leg_r_y,
            z_rot: leg_r_z,
            cubes: vec![Cube {
                tex: (0.0, 16.0),
                origin: [-2.0, 0.0, -2.0],
                size: [4.0, 12.0, 4.0],
                inflate: 0.0,
                        mirror: false,
                        }],
            children: vec![Part {
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
            }],
        },
        // left leg (+ left pants child)
        Part {
            pivot: leg_l_pivot,
            x_rot: leg_l_x,
            y_rot: leg_l_y,
            z_rot: leg_l_z,
            cubes: vec![Cube {
                tex: (16.0, 48.0),
                origin: [-2.0, 0.0, -2.0],
                size: [4.0, 12.0, 4.0],
                inflate: 0.0,
                        mirror: false,
                        }],
            children: vec![Part {
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
            }],
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
    /// folds (the same mix_light every entity rides).
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
    ) -> Vec<AvatarQuad> {
        let parts = build_boat_parts(self.raft, chest, rowing_left, rowing_right);
        let (fold_point, fold_normal) = boat_folds(yaw_deg, feet, rock);
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
/// the boat's own words.
fn boat_folds(
    yaw_deg: f32,
    feet: Vec3,
    rock: f32,
) -> (
    impl Fn([f32; 3]) -> [f32; 3],
    impl Fn([f32; 3]) -> [f32; 3],
) {
    let a = (180.0f32 - yaw_deg).to_radians();
    let (sa, ca) = a.sin_cos();
    let lift = BOAT_SUBMIT_LIFT;
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
        // the R_y(180 − yaw) fold
        let px = rx * ca + rz * sa;
        let pz = -rx * sa + rz * ca;
        [
            feet.x + px,
            feet.y as f32 + ry + lift,
            feet.z + pz,
        ]
    };
    let fold_normal = move |n: [f32; 3]| -> [f32; 3] {
        let (qx, qy, qz) = (n[2], n[1], -n[0]);
        let (sx, sy, sz) = (-qx, -qy, qz);
        let [rx, ry, rz] = rock_normal([sx, sy, sz]);
        [rx * ca + rz * sa, ry, -rx * sa + rz * ca]
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
pub fn armor_parts(state: &AvatarState, slot: usize) -> Vec<Part> {
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
    cam_yaw: f32,
    cam_pitch: f32,
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
    let m = m_arm.mul(&layer);
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
        emit_cube(cube, &m, fold_point, fold_normal, cam_yaw, cam_pitch, out);
    }
    for child in &part.children {
        collect_part(child, &m, fold_point, fold_normal, cam_yaw, cam_pitch, out);
    }
}

/// One cube's six faces — the `ModelPart$Cube` corner/uv table the
/// first-person arm already speaks (hand.rs's own decode), inflated by
/// the `CubeDeformation` extend, folded into world space.
fn emit_cube(
    cube: &Cube,
    m: &Mat4,
    fold_point: &dyn Fn([f32; 3]) -> [f32; 3],
    fold_normal: &dyn Fn([f32; 3]) -> [f32; 3],
    cam_yaw: f32,
    cam_pitch: f32,
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
        let uv = if cube.mirror {
            [[ru, rv], [ru1, rv], [ru1, rv1], [ru, rv1]]
        } else {
            [[ru1, rv], [ru, rv], [ru, rv1], [ru1, rv1]]
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

#[cfg(test)]
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
        let quads = boat.quads(0.0, false, feet, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(quads.len(), 54, "5 hull parts + 2 paddles, 6 quads a cube");
        let chest = boat.quads(0.0, true, feet, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(chest.len(), 72, "the chest variant's three cubes ride +18");
        let raft = BoatModel {
            rgba: vec![255u8; 128 * 64 * 4],
            tex_h: 64,
            raft: true,
        };
        let raft_quads = raft.quads(0.0, false, feet, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(raft_quads.len(), 36, "the two-plank bottom, no rails");
        let raft_chest = raft.quads(0.0, true, feet, 0.0, 0.0, 0.0, 0.0, 0.0);
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
        let rest = boat.quads(0.0, false, feet, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(rest.len(), 54, "the swing never adds a quad");
        let swung = boat.quads(0.0, false, feet, 0.0, 0.0, std::f32::consts::FRAC_PI_2, 0.0, 0.0);
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
        let (fold_point, _) = boat_folds(0.0, feet, 0.0);
        let rest = fold_point([0.0, 0.0, 1.0]);
        // the rock-free word: R_y(90) -> S -> R_y(180) -> lift + feet —
        // [11.0, 64.375, -20.0] the existing hull pins speak
        assert!((rest[0] - 11.0).abs() < 1e-5, "the rest x: {}", rest[0]);
        assert!((rest[1] - 64.375).abs() < 1e-5, "the rest y: {}", rest[1]);
        assert!((rest[2] - (-20.0)).abs() < 1e-5, "the rest z: {}", rest[2]);
        // THE 90° ROCK (the local word through the diagonal: the
        // Rodrigues walk v' = v·cos + (k×v)·sin + k·(k·v)·(1−cos) over
        // k = (1,0,1)/sqrt(2)) — the bow word swings off its axis
        let (fold_rock, _) = boat_folds(0.0, feet, 90.0);
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
        let quads = oak.quads(0.0, false, Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(quads.len(), 54, "the real texture draws the real hull");
        let chest_oak = BoatModel::load(&root, false, true, "oak").expect("the chest band loads");
        let chest_quads = chest_oak.quads(0.0, true, Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(chest_quads.len(), 72, "the chest cubes ride the real band");
        assert!(
            BoatModel::load(&root, false, false, "unknown_wood").is_none(),
            "the unknown wood refuses"
        );
        let raft = BoatModel::load(&root, true, false, "bamboo").expect("the raft loads");
        let raft_quads = raft.quads(0.0, false, Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, 0.0, 0.0);
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

    /// THE DEATH FLIP (javap: the deathTime branch's ZP rotation riding
    /// between the S and the R_y — `sqrt(min(1, (deathTime-1)/20*1.6))
    /// * 90°`): at the 20-tick mark the model lies flat, the y span
    /// collapses from the standing ~2.03 to the body's own thickness.
    #[test]
    fn the_death_flip_lays_the_model_flat() {
        let model = AvatarModel {
            rgba: vec![255; 64 * 64 * 4],
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
        let (plo, phi) = bbox(&pig);
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
}

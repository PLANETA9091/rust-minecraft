//! The first-person hand: the empty main-hand arm, rendered from the
//! official player skin (wide Steve). Every constant below is 26.2
//! bytecode evidence — no invented numbers:
//!
//! - `ItemInHandRenderer.renderPlayerArm` (javap): the rest pose for an
//!   empty hand is `translate(side*0.64000005, -0.6, -0.71999997)`,
//!   `rotY(side*45°)`, then (after the zero swing terms)
//!   `translate(side*-1, 3.6, 3.5)`, `rotZ(side*120°)`, `rotX(200°)`,
//!   `rotY(side*-135°)`, `translate(side*5.6, 0, 0)`.
//! - `GameRenderer.renderItemInHand` (javap): the hand pose stack is
//!   multiplied by the camera rotation's INVERSE — hand space IS view
//!   space (+x right, +y up, -z forward), view matrix = identity.
//! - `AvatarRenderer.renderHand` (javap): the arm part gets
//!   `resetPose()` + `rightArm.zRot = 0.1` (radians) and submits with
//!   `RenderTypes.entityTranslucent` (two-sided — no backface cull).
//! - `HumanoidModel.createMesh` (javap): `right_arm` = texOffs(40, 16),
//!   `addBox(-3, -2, -2, 4, 12, 4)`, pivot `(-5, 2, 0)` (model units,
//!   /16 at render — `ModelPart$Vertex.worldX/Y/Z`).
//! - `ModelPart$Cube` + `$Polygon` (javap): the six faces' corner
//!   orders and their UV-rect corners (below, verbatim).
//!
//! The skin pixels stream from the runtime asset root and are
//! sha1-verified against the generated `skin` table before a single
//! texel is trusted; an absent or mismatched file renders NOTHING
//! (the reference renderer must not lie about its data).

use std::path::Path;

use anyhow::Context;
use mc_registry::element_quads::Quad;
use mc_registry::generated::skin;
use mc_registry::sprite_uv::{self, Sample};
use sha1::Digest;

use crate::math::Mat4;

/// One view-space textured quad of the arm box (the model corner order
/// and uv assignment are `ModelPart$Cube`'s, verbatim).
pub struct HandQuad {
    pub pts: [[f32; 3]; 4],
    /// texel coords into the 64x64 skin (u right, v down)
    pub uv: [[f32; 2]; 4],
}

pub struct ArmModel {
    /// decoded RGBA8 (64*64*4), the runtime file's verbatim pixels
    rgba: Vec<u8>,
    #[cfg_attr(not(test), expect(dead_code))]
    quads: Vec<HandQuad>,
    /// which skin variant this model carries (the quads were built for)
    kind: ArmKind,
}

/// The player-skin variant. Vanilla picks it from the profile's texture
/// metadata (the `slim` flag on the skin's textures property); this
/// client has no profile yet, so the variant is an EXPLICIT selection
/// (`--slim-arm`, the same deterministic stand-in pattern as
/// `--held-item`) — the default stays the wide classic the goldens see.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmKind {
    /// The classic model: right_arm addBox(-3, -2, -2, 4, 12, 4) —
    /// HumanoidModel.createMesh, the goldens' arm.
    Wide,
    /// The slim model: right_arm addBox(-2, -2, -2, 3, 12, 4) —
    /// PlayerModel.createMesh(slim = true), javap (26.2): texOffs (40, 16)
    /// UNCHANGED, pivot (-5, 2, 0) UNCHANGED — only the box shrinks.
    Slim,
}

/// The model-space box (the addBox origin and the (w, h, d) extents) of
/// one arm kind, in model units (/16 at transform time).
fn arm_box(kind: ArmKind) -> ([f32; 3], [f32; 3]) {
    match kind {
        ArmKind::Wide => {
            let [w, h, d] = skin::ARM_BOX_PX;
            ([-3.0, -2.0, -2.0], [w as f32, h as f32, d as f32])
        }
        ArmKind::Slim => {
            let [w, h, d] = skin::SLIM_ARM_BOX_PX;
            let [x, y, z] = skin::SLIM_ARM_BOX_ORIGIN_PX;
            ([x as f32, y as f32, z as f32], [w as f32, h as f32, d as f32])
        }
    }
}

/// Load + sha1-verify the WIDE skin from the runtime asset root, then
/// build the vanilla arm geometry. `None` = not drawn.
#[cfg_attr(not(test), expect(dead_code))]
pub fn load(root: &Path) -> Option<ArmModel> {
    load_kind(root, ArmKind::Wide)
}

/// Load + sha1-verify the skin of ONE kind (wide Steve or slim Alex —
/// each verifies against its own generated sha1) and build the matching
/// vanilla geometry. `None` = not drawn (absent or mismatched file —
/// the renderer must not lie about its data).
pub fn load_kind(root: &Path, kind: ArmKind) -> Option<ArmModel> {
    let (runtime_path, png_sha1) = match kind {
        ArmKind::Wide => (skin::RUNTIME_PATH, skin::STEVE_WIDE_PNG_SHA1),
        ArmKind::Slim => (skin::SLIM_RUNTIME_PATH, skin::SLIM_ALEX_PNG_SHA1),
    };
    let bytes = std::fs::read(root.join(runtime_path)).ok()?;
    let digest = sha1::Sha1::digest(&bytes);
    if hex::encode(digest) != png_sha1 {
        return None;
    }
    let (w, h, rgba) = decode_rgba(&bytes).ok()?;
    if w != skin::SKIN_SIDE
        || h != skin::SKIN_SIDE
        || rgba.len() != (skin::SKIN_SIDE as usize) * (skin::SKIN_SIDE as usize) * 4
    {
        return None;
    }
    Some(ArmModel {
        rgba,
        quads: build_quads_kind(kind),
        kind,
    })
}

impl ArmModel {
    /// The hand pass at an attack-animation progress (0 = rest; the
    /// deterministic scenes never swing, so the goldens always see 0).
    /// Clears depth (vanilla clears depth between the level and the hand)
    /// and draws the arm in view space under the world's projection (same
    /// fov — vanilla's hand fov follows the world's).
    pub fn draw(&self, fb: &mut crate::raster::Framebuf, proj: Mat4, swing: f32) {
        self.draw_side(fb, proj, swing, false);
    }

    /// The hand pass on EITHER side (the submitArmWithItem walk's own
    /// two-lane shape — the OFF_HAND call the renderPlayerArm walk
    /// answers with the side*-1 fold): `left = true` draws the Player
    /// Model's OWN left arm (the texOffs (32, 48) rebuild, the +5
    /// pivot, the cp #689 zRot -0.1 — the bare offhand the empty
    /// offhand slot wears). Clears depth and draws in view space under
    /// the world's projection (same fov — vanilla's hand fov follows
    /// the world's).
    pub fn draw_side(&self, fb: &mut crate::raster::Framebuf, proj: Mat4, swing: f32, left: bool) {
        fb.clear_depth();
        let quads = build_arm_quads_at(self.kind, swing, 0.0, left);
        fb.draw_hand_quads(proj, &quads, &self.rgba, skin::SKIN_SIDE as usize);
    }
}

/// `net/minecraft/world/item/SwingAnimationType` — the enum the held
/// stack's `SwingAnimation` component carries (the clinit's own rows:
/// NONE id 0 / WHACK id 1 / STAB id 2 — the `$values` walk's order, the
/// STREAM_CODEC the BY_ID continuous map speaks). The render dispatch
/// (`ItemInHandRenderer.submitArmWithItem`'s own switch over the held
/// stack's component type) rides it: WHACK walks the classic
/// swingArm arc, STAB walks `SpearAnimations.firstPersonAttack`, NONE
/// applies no swing transform at all (the switch's bare goto).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwingKind {
    /// the enum's own id 0 — the KINETIC use lane's bare anchor rides
    /// it (the SPEAR use branch's own translate, the swing dispatch the
    /// using walk never reaches)
    None,
    Whack,
    Stab,
}

/// `net/minecraft/world/item/component/SwingAnimation` — the (type,
/// duration) pair the held item's component keeps (the ctor's two
/// fields; `ItemStack.getSwingAnimation` = the SWING_ANIMATION
/// component `getOrDefault(DEFAULT)`). DEFAULT = `(WHACK, 6)` — the
/// static init's `getstatic WHACK; bipush 6` pair. The SPEAR factory
/// (`Item$Properties.spear`) is the jar's ONLY declarer: the first
/// float (the swing seconds the ATTACK_SPEED modifier shares) folds
/// `(int)(seconds * 20.0f)` into the duration with the STAB type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwingAnim {
    pub kind: SwingKind,
    pub duration: i32,
}

impl Default for SwingAnim {
    fn default() -> Self {
        Self {
            kind: SwingKind::Whack,
            duration: 6,
        }
    }
}

/// The vanilla swing machinery, verbatim from LivingEntity's bytecode
/// (`swing`, `updateSwingTime`, `getAttackAnim`, `getCurrentSwingDuration`
/// + `SwingAnimation.DEFAULT = (WHACK, 6)` — the fist swing is 6 ticks).
/// Our client advances one game tick per frame, so `tick()` rides the
/// frame loop; the renderer samples `get_attack_anim(partial_tick)` —
/// the fixed clock's own remainder the frame renders with.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwingState {
    pub swinging: bool,
    /// `swingTime` — LivingEntity.swing() resets it to **-1** (the next
    /// tick's +1 lands on 0 with attackAnim = 0).
    pub swing_time: i32,
    /// `oAttackAnim` (previous tick's progress — the lerp's from).
    pub o_attack_anim: f32,
    /// `attackAnim` = swingTime / duration (assigned every tick).
    pub attack_anim: f32,
    /// `getCurrentSwingDuration()`'s live read — the held stack's own
    /// `SwingAnimation.duration` (the swingingArm stack's component, the
    /// DEFAULT (WHACK, 6) the empty stack falls back to). The App
    /// refreshes it each tick from the held item (the jar reads the
    /// stack live per call; the haste/fatigue folds ride the duration's
    /// own gates — the client tracks neither, the plain component is the
    /// honest law here).
    pub duration: i32,
}

impl Default for SwingState {
    fn default() -> Self {
        Self {
            swinging: false,
            swing_time: 0,
            o_attack_anim: 0.0,
            attack_anim: 0.0,
            duration: 6,
        }
    }
}

impl SwingState {
    /// The fist swing's tick duration: `SwingAnimation.DEFAULT` is
    /// `(SwingAnimationType.WHACK, 6)` — bipush 6 in the static init.
    pub const DEFAULT_DURATION: i32 = 6;

    /// `LivingEntity.swing(hand)`'s trigger logic: a re-trigger is IGNORED
    /// while a swing is still in its first half (swingTime in
    /// [0, getCurrentSwingDuration()/2)); otherwise (or fresh) swingTime
    /// = -1 and swinging = true. The gate's divisor is the LIVE read —
    /// the held item's own component duration (`self.duration`).
    pub fn start(&mut self) {
        if !(self.swinging && self.swing_time < self.duration / 2 && self.swing_time >= 0) {
            self.swing_time = -1;
            self.swinging = true;
        }
    }

    /// `LivingEntity.updateSwingTime()` (one game tick): the duration is
    /// the LIVE `getCurrentSwingDuration()` read (the jar's local 1 —
    /// re-read every tick), advance swingTime while swinging (reset to
    /// rest at the duration), pin it to 0 at rest, then attackAnim =
    /// swingTime / duration. oAttackAnim takes the previous tick's value
    /// first (the tick's lerp bookkeeping).
    pub fn tick(&mut self) {
        let duration = self.duration;
        self.o_attack_anim = self.attack_anim;
        if self.swinging {
            self.swing_time += 1;
            if self.swing_time >= duration {
                self.swing_time = 0;
                self.swinging = false;
            }
        } else {
            self.swing_time = 0;
        }
        self.attack_anim = self.swing_time as f32 / duration as f32;
    }

    /// `LivingEntity.getAttackAnim(partialTick)` verbatim: the o/attack
    /// lerp with the wrap guard for the 1 -> 0 progress jump.
    pub fn get_attack_anim(&self, partial_tick: f32) -> f32 {
        let mut f = self.attack_anim - self.o_attack_anim;
        if f < 0.0 {
            f += 1.0;
        }
        self.o_attack_anim + f * partial_tick
    }

    /// The deterministic stand-in for the swing clock (the same role
    /// `--held-item` plays for inventory state): the client has no
    /// gameplay input wired to the swing in the headless scenes, so a
    /// scene that must PIN a mid-swing pose freezes the timeline at a
    /// fixed progress instead. `o_attack_anim` and `attack_anim` both
    /// carry the progress, so `get_attack_anim(partial)` returns it
    /// exactly at every partial tick — the vanilla lerp is the identity
    /// on a frame with no tick-to-tick change (a legal timeline slice:
    /// the state as the camera would sample it across a held frame).
    ///
    /// Only the non-ticking headless scenes hold this state stable; a
    /// real `tick()` resumes the vanilla timeline from here. Progress 0
    /// IS the rest pose (bit-identical to `default()`'s sample).
    pub fn frozen_at(progress: f32) -> Self {
        Self {
            swinging: progress > 0.0,
            swing_time: 0,
            o_attack_anim: progress,
            attack_anim: progress,
            // the DEFAULT component's clock (the scenes' freeze rides the
            // empty-stack fallback — the WHACK lane's own 6-tick shape)
            duration: Self::DEFAULT_DURATION,
        }
    }
}

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
            _ => {
                rgba[o] = s[0];
                rgba[o + 1] = s[0];
                rgba[o + 2] = s[0];
            }
        }
    }
    Ok((w, h, rgba))
}

/// The rest-pose transform (see the module docs for the bytecode source).
/// `side` is +1 for the RIGHT main hand, -1 for the LEFT offhand (the
/// 26.2 renderPlayerArm walk factors the arm's own ±1 through EVERY
/// term — scripts/jar_work/handrenderer_51.txt pcs 106-277: the
/// `loc8=-1.0 *` fold the LEFT walk carries verbatim). `swing` is the
/// attack animation progress (`getAttackAnim`, 0 = rest), `equip` the
/// equip animation progress (0 — no item switching exists). Both are
/// the float params of `ItemInHandRenderer.renderPlayerArm` (the javap
/// decode: slot 5 = swing, slot 4 = equip); at rest every swing term
/// collapses to exactly zero, so this IS the rest pose bit-for-bit.
/// THE TAIL (the translateToHand + renderHand walk — scripts/jar_work/
/// playermodel_51.txt + avatarrenderer_51.txt: the PlayerModel's OWN
/// arm parts the player model rebuilds, the root + part
/// translateAndRotate pair, the renderHand's cp-pinned zRot constants
/// — leftArm -0.1 = cp #689, rightArm +0.1 = cp #693): the pivot the
/// arm side speaks (right -5/16, left +5/16 — the PartPose.offset the
/// createMesh walk pins), the zRot the side's own constant wears.
fn arm_transform(swing: f32, equip: f32, side: f32) -> Mat4 {
    let t = |x: f32, y: f32, z: f32| Mat4::translation(x, y, z);
    let deg = |d: f32| d.to_radians();
    let [f10, f11, f12, f13, f14] = swing_terms(swing);
    Mat4::IDENT
        .mul(&t(side * (0.640_000_05 + f10), (f11 - 0.6) + equip * -0.6, f12 + -0.719_999_97))
        .mul(&Mat4::rot_y(deg(side * 45.0)))
        .mul(&Mat4::rot_y(deg(side * f14 * 70.0)))
        .mul(&Mat4::rot_z(deg(side * f13 * -20.0)))
        .mul(&t(-side, 3.6, 3.5))
        .mul(&Mat4::rot_z(deg(side * 120.0)))
        .mul(&Mat4::rot_x(deg(200.0)))
        .mul(&Mat4::rot_y(deg(side * -135.0)))
        .mul(&t(side * 5.6, 0.0, 0.0))
        // the PlayerModel's own arm part: the pivot the side speaks
        // (right (-5, 2, 0), left (+5, 2, 0) — the createMesh walk's
        // PartPose.offset pair), the zRot the renderHand constants
        // wear (right +0.1 = cp #693, left -0.1 = cp #689 — the s=±1
        // idle bob the avatar law banks)
        .mul(&t(side * -5.0 / 16.0, 2.0 / 16.0, 0.0))
        .mul(&Mat4::rot_z(side * 0.1))
}

/// The five swing terms of `renderPlayerArm` — f10..f14 exactly as the
/// bytecode computes them (Mth.sin/Mth.sqrt are f64 sin/sqrt narrowed to
/// f32 — Mth's own bodies), extracted so the sweep test can pin the
/// whole curve bit-exactly against an independent oracle without
/// rebuilding the matrix.
fn swing_terms(swing: f32) -> [f32; 5] {
    let sq = mth_sqrt(swing);
    [
        -0.3f32 * mth_sin(sq * std::f32::consts::PI), // f10: X wobble
        0.4f32 * mth_sin(sq * 6.2831855), // f11: Y bob (6.2831855 = the bytecode's 2π f32)
        -0.4f32 * mth_sin(swing * std::f32::consts::PI), // f12: Z pull
        mth_sin(swing * swing * std::f32::consts::PI), // f13: rotZ amount
        mth_sin(sq * std::f32::consts::PI), // f14: extra rotY amount
    ]
}

/// `Mth.sin(float)`: `Math.sin((double)x)` narrowed back to f32 (the Mth
/// body — the game never sins in f32).
fn mth_sin(x: f32) -> f32 {
    (x as f64).sin() as f32
}

/// `Mth.sqrt(float)`: `Math.sqrt((double)x)` narrowed back to f32.
fn mth_sqrt(x: f32) -> f32 {
    (x as f64).sqrt() as f32
}

/// Build the six hand quads. The corner/uv pairs are the javap decode of
/// `ModelPart$Cube`: DOWN = (x1,y0,z1),(x0,y0,z1),(x0,y0,z0),(x1,y0,z0)
/// over rect (u+d, v); UP = (x1,y1,z0),(x0,y1,z0),(x0,y1,z1),(x1,y1,z1)
/// over rect (u+d+w, v); WEST = (x0,y0,z0),(x0,y0,z1),(x0,y1,z1),
/// (x0,y1,z0) over (u, v+d); NORTH = (x1,y0,z0),(x0,y0,z0),(x0,y1,z0),
/// (x1,y1,z0) over (u+d, v+d); EAST = (x1,y0,z1),(x1,y0,z0),(x1,y1,z0),
/// (x1,y1,z1) over (u+d+w, v+d); SOUTH = (x0,y0,z1),(x1,y0,z1),(x1,y1,z1),
/// (x0,y1,z1) over (u+d+w+d, v+d). Each face's corner k takes the
/// Polygon constructor's fixed rect corner: k=0 -> (u1,v0), 1 -> (u0,v0),
/// 2 -> (u0,v1), 3 -> (u1,v1).
#[cfg_attr(not(test), expect(dead_code))]
fn build_quads() -> Vec<HandQuad> {
    build_quads_kind(ArmKind::Wide)
}

/// The WIDE quads at an arbitrary progress — the pre-variant name the
/// bit-identity and sweep tests ride (the default arm kind).
#[cfg_attr(not(test), expect(dead_code))]
fn build_quads_at(swing: f32, equip: f32) -> Vec<HandQuad> {
    build_quads_kind_at(ArmKind::Wide, swing, equip)
}

fn build_quads_kind(kind: ArmKind) -> Vec<HandQuad> {
    build_quads_kind_at(kind, 0.0, 0.0)
}

/// The six hand quads of ONE arm kind at an arbitrary attack/equip
/// animation progress — the same `ModelPart$Cube` face table, transformed
/// by the full renderPlayerArm chain (the swing terms above). At (0, 0)
/// this is the rest pose, bit-for-bit (every added rotation is the exact
/// identity). The kind changes ONLY the model-space box (origin +
/// extents); texOffs, the unwrap layout and the transform chain are
/// shared, so the wide path performs the same arithmetic it always did.
fn build_quads_kind_at(kind: ArmKind, swing: f32, equip: f32) -> Vec<HandQuad> {
    build_arm_quads_at(kind, swing, equip, false)
}

/// The LEFT arm's own cube — javap PlayerModel.createMesh (scripts/
/// jar_work/playermodel_51.txt): the player model REBUILDS the
/// HumanoidModel's arms without the mirror flag — texOffs (32, 48) for
/// BOTH kinds, the wide box (-1, -2, -2, 4, 12, 4), the slim box
/// (-1, -2, -2, 3, 12, 4), the pivot (+5, 2, 0) the PartPose.offset
/// pins. renderHand's own zRot constants (cp #689) dress the tail.
fn left_arm_box(kind: ArmKind) -> ([f32; 3], [f32; 3]) {
    let [w, h, d] = match kind {
        ArmKind::Wide => skin::ARM_BOX_PX,
        ArmKind::Slim => skin::SLIM_ARM_BOX_PX,
    };
    ([-1.0, -2.0, -2.0], [w as f32, h as f32, d as f32])
}

/// The LEFT arm's texOffs (the PlayerModel's own rebuild — the shared
/// (32, 48) band the createMesh walk pins for both kinds).
const LEFT_ARM_TEX_OFFS: (u32, u32) = (32, 48);

/// The six hand quads of ONE arm at an arbitrary progress, EITHER side
/// — the same `ModelPart$Cube` face table (the corner k takes the
/// Polygon ctor's fixed rect corner; the UV mapping the right arm's
/// golden-verified decode carries, the mirror the PlayerModel's own
/// rebuild never arms). `side` = -1 builds the offhand's left arm: the
/// box the createMesh left_arm walk speaks, the texOffs (32, 48), the
/// transform the side*-1 fold wears (the renderPlayerArm walk's own
/// `loc8=-1.0 *` terms).
fn build_arm_quads_at(kind: ArmKind, swing: f32, equip: f32, left: bool) -> Vec<HandQuad> {
    let (u0, v0) = if left { LEFT_ARM_TEX_OFFS } else { (skin::ARM_TEX_OFFS.0 as u32, skin::ARM_TEX_OFFS.1 as u32) };
    let (origin, box_px) = if left { left_arm_box(kind) } else { arm_box(kind) };
    let side = if left { -1.0f32 } else { 1.0f32 };
    let [x0, y0, z0] = origin;
    let [w, h, d] = box_px;
    let (u0, v0) = (u0 as f32, v0 as f32);
    let (x1, y1, z1) = (x0 + w, y0 + h, z0 + d);
    let m = arm_transform(swing, equip, side);
    let px = |p: [f32; 3]| {
        let q = m.transform_point(crate::math::Vec3::new(p[0] / 16.0, p[1] / 16.0, p[2] / 16.0));
        [q[0], q[1], q[2]]
    };
    let faces: [(&[[f32; 3]; 4], (f32, f32), (f32, f32)); 6] = [
        (
            &[[x1, y0, z1], [x0, y0, z1], [x0, y0, z0], [x1, y0, z0]],
            (u0 + d, v0),
            (w, d),
        ), // DOWN  (the arm's shoulder end) — rect (u+d, v)
        (
            &[[x1, y1, z0], [x0, y1, z0], [x0, y1, z1], [x1, y1, z1]],
            (u0 + d + w, v0 + d),
            (w, -d),
        ), // UP    (the arm's wrist end) — rect (u+d+w, v) V-FLIPPED
           // (the Polygon call passes v0 = the strip's LOWER edge, v1 = the
           // upper one — the rect's v runs 20 -> 16, hence the negative h)
        (
            &[[x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0]],
            (u0, v0 + d),
            (d, h),
        ), // WEST  (outer) — rect (u, v+d)
        (
            &[[x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0]],
            (u0 + d, v0 + d),
            (w, h),
        ), // NORTH — rect (u+d, v+d)
        (
            &[[x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1]],
            (u0 + d + w, v0 + d),
            (d, h),
        ), // EAST  (inner) — rect (u+d+w, v+d)
        (
            &[[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]],
            (u0 + d + w + d, v0 + d),
            (w, h),
        ), // SOUTH — rect (u+d+w+d, v+d)
    ];
    let mut quads = Vec::with_capacity(6);
    for (corners, (ru, rv), (rw, rh)) in faces {
        let (ru1, rv1) = (ru + rw, rv + rh);
        let uv = [[ru1, rv], [ru, rv], [ru, rv1], [ru1, rv1]];
        quads.push(HandQuad {
            pts: [
                px(corners[0]),
                px(corners[1]),
                px(corners[2]),
                px(corners[3]),
            ],
            uv,
        });
    }
    quads
}

/// The held item: the first-person consumer of the registry's display
/// arc (item id -> model row -> bake quads -> display pose -> sampled
/// texels). Vanilla's `submitArmWithItem` (javap) draws the ITEM instead
/// of the arm whenever the main-hand stack is non-empty, so the client
/// renders exactly one of the two per frame.
///
/// The item's own pose (`posed_item_quads` — the ItemTransform.apply of
/// the model's display stanza, oracle-pinned bit-for-bit) lands the
/// block-unit bake vertices around the origin; the HAND ANCHOR moves
/// them into view space, verbatim from ItemInHandRenderer's bytecode:
///
/// - `applyItemArmTransform`: `translate(side*0.56, -0.52, equip*-0.6 +
///   -0.72)` (equip = the mainHandHeight lerp — 0, no item switching
///   exists).
/// - `swingArm` (the SwingAnimation.DEFAULT = WHACK arc — called
///   UNCONDITIONALLY for whack items, even at rest):
///   `translate(side*-0.4*sin(sqrt(s)*PI), 0.2*sin(sqrt(s)*2PI),
///   -0.2*sin(s*PI))` — exact zeros at rest.
/// - `applyItemArmAttackTransform`: `f5 = sin(s*s*PI)`,
///   `rotY(side*(45 + f5*-20)deg)`, `f6 = sin(sqrt(s)*PI)`,
///   `rotZ(side*f6*-20deg)`, `rotX(f6*-80deg)`, `rotY(side*-45deg)`.
///
/// The texels stream from the GENERATED texture namespace (embedded in
/// the registry, sha-pinned at generation time) through
/// `sprite_uv::sample_uv` — no runtime file, nothing unverified.
pub struct HeldItem {
    /// the posed quads (block units, view-space pose of the context)
    quads: Vec<Quad>,
}

/// Load the posed quads of one model path for the first-person right
/// hand — the single-path convenience (the offline `--held-item` golden
/// lane's own vocabulary) over [`load_item_layers`]. `None` = unknown
/// model or outside the bake domain — nothing to draw (the caller
/// renders the empty-hand arm instead).
pub fn load_item(model_id: &str) -> Option<HeldItem> {
    match mc_registry::display::posed_item_layer_quads(
        model_id,
        None,
        mc_registry::display::FIRSTPERSON_RIGHTHAND,
    ) {
        Some(Ok(quads)) => Some(HeldItem { quads }),
        // a broken display stanza binds nothing (vanilla warns + drops)
        Some(Err(_)) => None,
        None => None,
    }
}

/// The ctx-general single-path loader (the load_item walk's own
/// shape, the display context the caller names).
#[cfg_attr(not(test), expect(dead_code))]
pub fn load_item_ctx(model_id: &str, ctx: usize) -> Option<HeldItem> {
    match mc_registry::display::posed_item_layer_quads(model_id, None, ctx) {
        Some(Ok(quads)) => Some(HeldItem { quads }),
        Some(Err(_)) => None,
        None => None,
    }
}

/// Load the posed quads of one RESTING-HAND layer list — the vanilla
/// `ItemStackRenderState` arc's own shape: every layer renders through
/// its `LayerRenderState` (the local transform on the raw bake vertices,
/// the display stanza on top, the hand anchor outside), the layers'
/// quads concatenate in order. ANY unknown model or broken stanza binds
/// NOTHING (the arm shows) — the honest unknown, never a half-rendered
/// guess.
pub fn load_item_layers(layers: &[mc_registry::ItemModelLayer]) -> Option<HeldItem> {
    if layers.is_empty() {
        return None;
    }
    let mut quads = Vec::new();
    for layer in layers {
        match mc_registry::display::posed_item_layer_quads(
            layer.model,
            layer.transform,
            mc_registry::display::FIRSTPERSON_RIGHTHAND,
        ) {
            Some(Ok(mut q)) => quads.append(&mut q),
            // a broken display stanza binds nothing (vanilla warns + drops)
            Some(Err(_)) => return None,
            None => return None,
        }
    }
    Some(HeldItem { quads })
}

/// The ctx-general layers loader (the resting-hand layers on EITHER
/// side — the display stanza the context names, the left-hand mirror
/// the registry's own left-hand walk applies).
pub fn load_item_layers_ctx(
    layers: &[mc_registry::ItemModelLayer],
    ctx: usize,
) -> Option<HeldItem> {
    if layers.is_empty() {
        return None;
    }
    let mut quads = Vec::new();
    for layer in layers {
        match mc_registry::display::posed_item_layer_quads(layer.model, layer.transform, ctx) {
            Some(Ok(mut q)) => quads.append(&mut q),
            Some(Err(_)) => return None,
            None => return None,
        }
    }
    Some(HeldItem { quads })
}

/// Load the posed quads of one SPECIAL renderer item by its WIRE id —
/// the 63 block-entity / entity-model items (shield, trident, the
/// chests, the shulker boxes, the banners, the heads, the conduit, the
/// decorated pot, the golem statues). The geometry is the running
/// client's OWN LayerDefinitions polygons (the dump's bit-exact bake),
/// the display stanza comes from the payload's base model, and the
/// pixels ride the generated namespace's entity textures (the banner
/// flag's dye multiply rode generation). `None` = unknown id or a
/// broken base stanza — the bare arm, never a half-render.
pub fn load_special_item(id: i32) -> Option<HeldItem> {
    match mc_registry::display::special_item_quads(
        id,
        mc_registry::display::FIRSTPERSON_RIGHTHAND,
    ) {
        Some(Ok(quads)) => Some(HeldItem { quads }),
        // a broken display stanza binds nothing (vanilla warns + drops)
        Some(Err(_)) => None,
        None => None,
    }
}

/// The ctx-general SPECIAL loader (the offhand shield's own lane —
/// the same block-entity bake on the display context the caller
/// names). `None` = unknown id or a broken base stanza — the bare
/// left arm, never a half-render.
pub fn load_special_item_ctx(id: i32, ctx: usize) -> Option<HeldItem> {
    match mc_registry::display::special_item_quads(id, ctx) {
        Some(Ok(quads)) => Some(HeldItem { quads }),
        Some(Err(_)) => None,
        None => None,
    }
}

/// Load the posed quads of one item's USING-branch SPECIAL payload by
/// its WIRE id — the shield's blocking pose (the shield_blocking base
/// poses the geometry, the tree's mirror transform rides the local
/// decode) and the trident's throwing pose. The same honest law: an
/// unknown id, a non-special using lane or a broken stanza binds
/// NOTHING (the caller renders the bare arm) — never a half-render.
pub fn load_special_using_item(id: i32) -> Option<HeldItem> {
    load_special_using_item_ctx(id, mc_registry::display::FIRSTPERSON_RIGHTHAND)
}

/// The ctx-general USING-branch SPECIAL loader (the offhand shield's
/// own lane — the same blocking base through the display context the
/// caller names: the FIRSTPERSON_LEFTHAND the off hand wears). The
/// same honest law: an unknown id, a non-special using lane or a
/// broken stanza binds NOTHING (the caller renders the bare arm) —
/// never a half-render.
pub fn load_special_using_item_ctx(id: i32, ctx: usize) -> Option<HeldItem> {
    match mc_registry::display::special_item_using_quads(id, ctx) {
        Some(Ok(quads)) => Some(HeldItem { quads }),
        Some(Err(_)) => None,
        None => None,
    }
}

/// Load the posed quads of one item's CHARGE-branch state by its WIRE
/// id — the loaded crossbow's resting render (the CHARGED_PROJECTILES
/// component's own lane, javap Charge.get): the crossbow_arrow bolt
/// stands on the pulled limbs, the crossbow_firework rides the firework
/// star. The layers go through the same posed-arc as the resting lane
/// (the parent-chain bake, the display stanza, the hand anchor
/// outside). `None` = no charge lane, a broken stanza or an unknown
/// model — the caller renders the bare arm, never a half-render.
pub fn load_charge_item(id: i32, kind: mc_registry::ChargeKind) -> Option<HeldItem> {
    let layers = mc_registry::item_charge_layers(id, kind)?;
    load_item_layers(&layers)
}

/// The item-in-hand anchor chain, `side` = +1 (the RIGHT main hand, the
/// default offline player's). At the rest pose (swing 0, equip 0) the
/// only non-trivial terms are the attack transform's rotY(side*45deg) ..
/// rotY(side*-45deg) pair — vanilla composes them every frame, so we do
/// too (op-for-op; not collapsed to identity). The swing KIND rides the
/// held stack's own `SwingAnimation` component (`submitArmWithItem`'s
/// own switch — javap pcs 1427-1494): WHACK walks the classic
/// swingArm+attack chain, STAB replaces BOTH with
/// `SpearAnimations.firstPersonAttack`, NONE stops at the bare
/// applyItemArmTransform anchor (the switch's bare goto).
fn item_hand_anchor(kind: SwingKind, swing: f32, equip: f32, side: f32) -> Mat4 {
    let t = |x: f32, y: f32, z: f32| Mat4::translation(x, y, z);
    let deg = |d: f32| d.to_radians();
    // applyItemArmTransform
    let a = t(side * 0.56, -0.52, equip * -0.6 + -0.72);
    match kind {
        SwingKind::None => a,
        SwingKind::Stab => a.mul(&stab_attack_matrix(swing, side)),
        SwingKind::Whack => {
            // swingArm's translate terms (the WHACK arc; exact zeros at
            // rest), then applyItemArmAttackTransform's two rotation
            // amounts — the five-term item-side curve, extracted for the
            // sweep test
            let [f5s, f6s, f7s, f5, f6] = item_swing_terms(swing);
            let b = t(side * f5s, f6s, f7s);
            a.mul(&b)
                .mul(&Mat4::rot_y(deg(side * (45.0 + f5 * -20.0))))
                .mul(&Mat4::rot_z(deg(side * f6 * -20.0)))
                .mul(&Mat4::rot_x(deg(f6 * -80.0)))
                .mul(&Mat4::rot_y(deg(side * -45.0)))
        }
    }
}

/// The five swing terms of the held-item path — swingArm's three
/// translate terms (f5s/f6s/f7s, the WHACK arc) plus
/// applyItemArmAttackTransform's two rotation amounts (f5/f6), exactly
/// as the bytecode computes them (the same f64 sin/sqrt narrowed to
/// f32). Extracted alongside [`swing_terms`] so the sweep pins BOTH
/// first-person curves — the bare arm's and the held item's — from one
/// oracle.
fn item_swing_terms(swing: f32) -> [f32; 5] {
    let sq = mth_sqrt(swing);
    [
        -0.4f32 * mth_sin(sq * std::f32::consts::PI), // f5s: X wobble
        0.2f32 * mth_sin(sq * 6.2831855), // f6s: Y bob
        -0.2f32 * mth_sin(swing * std::f32::consts::PI), // f7s: Z pull
        mth_sin(swing * swing * std::f32::consts::PI), // f5: rotY amount
        mth_sin(sq * std::f32::consts::PI), // f6: rotZ/rotX amount
    ]
}

/// `net/minecraft/util/Ease.inOutSine` (javap, verbatim): `ldc π (f32) ·
/// x` (an f32 multiply), the product widened to double for `Mth.cos`,
/// then the fold `(1 - c) / 2` (the `fsub`'s c−1 the `fneg` flips — the
/// JVM's operand order the dump speaks). `Mth.cos(D)F` is the 65536-entry
/// SIN table lookup (the index `(long)(v·10430.378 + 16384.0) & 65535`) —
/// the client's swing lanes speak the `Math.sin/cos` fold (the same
/// convention the WHACK arc's `mth_sin` rides, the f64 call narrowed to
/// f32; the table quantizes the ANGLE instead — a sub-pixel divergence,
/// a table-fidelity round can swap both lanes together).
fn ease_in_out_sine(x: f32) -> f32 {
    let c = ((std::f32::consts::PI * x) as f64).cos() as f32;
    (1.0 - c) / 2.0
}

/// `net/minecraft/util/Ease.outBack` (javap, verbatim): the f32 fold
/// `1 + 2.70158·(x-1)³ + 1.70158·(x-1)²` — `Mth.cube`/`Mth.square` are
/// the plain `x*x*x` / `x*x` f32 bodies (the dump's own two fmul walks),
/// the constants the CP's `ldc #7 = 1.70158f` / `ldc #8 = 2.70158f`.
/// The overshoot peaks past 1.0 (the recovery's own back-swing).
fn ease_out_back(x: f32) -> f32 {
    let t = x - 1.0;
    1.0f32 + 2.701_58f32 * (t * t * t) + 1.701_58f32 * (t * t)
}

/// `net/minecraft/util/Ease.inOutExpo` (javap, verbatim): the two-branch
/// exponential — `x < 0.5` (the `fcmpg 0.5` branch): 0 at exactly 0, else
/// `2^(20x - 10) / 2` (a f64 `Math.pow` folded, `d2f` narrowed); else 1 at
/// exactly 1, else `(2 - 2^(-20x + 10)) / 2`. Continuous at the 0.5 seam
/// (both arms speak 0.5 there).
fn ease_in_out_expo(x: f32) -> f32 {
    if x < 0.5 {
        if x == 0.0 {
            0.0
        } else {
            ((2.0f64).powf(20.0 * x as f64 - 10.0) / 2.0) as f32
        }
    } else if x == 1.0 {
        1.0
    } else {
        ((2.0 - (2.0f64).powf(-20.0 * x as f64 + 10.0)) / 2.0) as f32
    }
}

/// `net/minecraft/util/Ease.outCubic` (javap, verbatim): `1 - cube(1-x)`
/// — the `Mth.cube` plain f32 body (`x*x*x`), the `1-x` the fsub speaks.
fn ease_out_cubic(x: f32) -> f32 {
    let t = 1.0f32 - x;
    1.0f32 - (t * t * t)
}

/// `net/minecraft/util/Ease.inOutElastic` (javap, verbatim): the f64
/// walk the dump speaks — 0/1 pinned at the ends, the phase
/// `sin((20.0·x - 11.125) · 1.3962634801864624)` the `ldc2_w #83/#88/#90`
/// triple banks (the 1.3962634… the f32-rounded 2π/4.5 the CP keeps),
/// then the two pow branches: `x < 0.5` the `-(2^(20x-10)·s)/2` fold,
/// else `(2^(-20x+10)·s)/2 + 1` — the `Math.pow`/`Math.sin` f64 bodies
/// narrowed by `d2f` (the client's established Math convention).
fn ease_in_out_elastic(x: f32) -> f32 {
    if x == 0.0 {
        return 0.0;
    }
    if x == 1.0 {
        return 1.0;
    }
    let s = ((20.0 * x as f64 - 11.125) * 1.396_263_480_186_462_4).sin();
    if x < 0.5 {
        (-(2.0f64.powf(20.0 * x as f64 - 10.0) * s) / 2.0) as f32
    } else {
        (2.0f64.powf(-20.0 * x as f64 + 10.0) * s / 2.0 + 1.0) as f32
    }
}

/// `net/minecraft/util/Ease.outCirc` (javap, verbatim): `sqrt(1 -
/// square(x-1))` — the square the f32 body keeps, the sqrt the f64
/// `Math.sqrt` call narrowed by `d2f`.
fn ease_out_circ(x: f32) -> f32 {
    let t = 1.0f32 - x;
    ((1.0f32 - t * t) as f64).sqrt() as f32
}

/// `net/minecraft/util/Ease.inCirc` (javap, verbatim): `1 - sqrt(1 -
/// x·x)` — the sqrt the f64 call narrows, the f32 result the `fconst_1`
/// fadd lifts (the 1 adds AFTER the d2f, the dump's own order).
fn ease_in_circ(x: f32) -> f32 {
    (-((1.0f32 - x * x) as f64).sqrt() as f32) + 1.0f32
}

/// `net/minecraft/util/Ease.inOutBack` (javap, verbatim): the CP
/// constants the two branches bank — `c2 = 2.5949094` (the `fstore_2`
/// ldc), `2(c2+1) = 7.1898188` / `(c2+1) = 3.5949094` (the first
/// branch's `#93`/second's `#94`): `x < 0.5` the
/// `4x²·(#93·x - c2)/2` fold (the dead `c3 = 1.70158` store the dump
/// keeps but never reads — the compiler folded it), else `t = 2x-2`
/// and `(t²·(#94·t + c2) + 2)/2` — the `+2` INSIDE the division the
/// bytecode's `fconst_2; fadd; fconst_2; fdiv` order speaks.
fn ease_in_out_back(x: f32) -> f32 {
    const C2: f32 = 2.594_909_4;
    if x < 0.5 {
        (4.0f32 * x * x * (7.189_818_8f32 * x - C2)) / 2.0
    } else {
        let t = 2.0f32 * x - 2.0;
        (t * t * (3.594_909_4f32 * t + C2) + 2.0) / 2.0
    }
}

/// `net/minecraft/util/Ease.outQuart` (javap, verbatim): `1 -
/// square(square(1-x))` — the two f32 square bodies the nested
/// `invokestatic` pair speaks.
fn ease_out_quart(x: f32) -> f32 {
    let t = 1.0f32 - x;
    1.0f32 - (t * t) * (t * t)
}

/// `SpearAnimations.progress(FFF)F` — `Mth.clamp(Mth.inverseLerp(v, min,
/// max), 0, 1)`, the inverseLerp the `(v - min) / (max - min)` f32 body
/// speaks (the dump's own fsub/fdiv pair).
fn eased_progress(value: f32, min: f32, max: f32) -> f32 {
    ((value - min) / (max - min)).clamp(0.0, 1.0)
}

/// The three eased phases of `SpearAnimations.firstPersonAttack` —
/// f4 = inOutSine(progress(p, 0.0, 0.05)) (the thrust's own snap),
/// f5 = outBack(progress(p, 0.05, 0.2)) (the recovery's overshoot),
/// f6 = inOutExpo(progress(p, 0.4, 1.0)) (the slow tilt unwind).
/// Extracted so the sweep test pins the whole stab curve from one
/// oracle (the same shape [`item_swing_terms`] rides for WHACK).
fn stab_eases(p: f32) -> [f32; 3] {
    [
        ease_in_out_sine(eased_progress(p, 0.0, 0.05)),
        ease_out_back(eased_progress(p, 0.05, 0.2)),
        ease_in_out_expo(eased_progress(p, 0.4, 1.0)),
    ]
}

/// `SpearAnimations.firstPersonAttack(progress, poseStack, side, arm)`
/// — the STAB lane's own transform, verbatim from the bytecode: the
/// three eased phases fold into translate → rotX → translate
/// (`side·0.1·(f4-f5), -0.075·(f4-f6), 0.65·(f4-f5)` — the CP floats
/// 0.1/-0.075/0.65 — then `Axis.XP` `-70°·(f4-f6)`, then the f64
/// translate `(0, 0, -0.25·(f6-f5))`; the ×0.25 exact-power fold the
/// f32 compose reproduces bit-for-bit). At rest (p = 0) all three eases
/// speak 0 — the identity the item's raw anchor keeps; at the 0.05 peak
/// the item sits at the full thrust (side·0.1, -0.075, 0.65) tilted
/// -70°; at p = 1 the fold returns to the anchor (the f4-f5/f4-f6 terms
/// re-collide at zero). The matrix composes left-to-right like the
/// PoseStack's own push walk (the anchor's convention).
fn stab_attack_matrix(p: f32, side: f32) -> Mat4 {
    let [f4, f5, f6] = stab_eases(p);
    let t = |x: f32, y: f32, z: f32| Mat4::translation(x, y, z);
    t(side * 0.1 * (f4 - f5), -0.075 * (f4 - f6), 0.65 * (f4 - f5))
        .mul(&Mat4::rot_x((-70.0f32 * (f4 - f6)).to_radians()))
        .mul(&t(0.0, 0.0, -0.25 * (f6 - f5)))
}

/// `SpearAnimations$UseParams` — the ten floats the
/// `fromKineticWeapon` ctor walk banks (the putfield order the dump
/// speaks): the raise ladder over the delay window, the sway pair the
/// sin folds ride, the elastic lower, the back-rise over the damage
/// window's own last five ticks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpearUseParams {
    pub raise: f32,
    pub raise_start: f32,
    pub raise_middle: f32,
    pub raise_end: f32,
    pub sway_progress: f32,
    pub lower: f32,
    pub raise_back: f32,
    pub sway_intensity: f32,
    pub sway_scale_slow: f32,
    pub sway_scale_fast: f32,
}

/// `SpearAnimations$UseParams.fromKineticWeapon(KineticWeapon, F)`
/// verbatim (the 26.2 dump): `timeHeld` the F param — the caller's
/// `getUseDuration - getUseItemRemainingTicks + partialTick - 1` fold —
/// against the windows the KineticWeapon's own fields keep:
/// `delayTicks` the raise window's end, the three conditions'
/// `maxDurationTicks + delay` the dismount/knockback/damage starts (the
/// `Condition::maxDurationTicks` lambda the invokedynamic binds, the
/// `orElse(0)` the missing condition wears), the sway the
/// `[dismount-20, dismount]` window the last 20 ticks ramps, the lower
/// the elastic over `[knock-40, knock]` evaluated at `timeHeld - 20`
/// (the ldc 20.0 the fsub speaks), the back-rise the damage window's
/// own `[damage-5, damage]` last five ticks, the sway pump
/// `2·outCirc(sway) - 2·inCirc(back)` and the two sin folds
/// `sin(timeHeld·19.0/30.0 · 0.0174532924)` the CP degrees→radians
/// constant rides.
pub fn spear_use_params(
    time_held: f32,
    delay: f32,
    dismount_start: f32,
    knock_start: f32,
    damage_start: f32,
) -> SpearUseParams {
    let raise = eased_progress(time_held, 0.0, delay);
    let raise_start = eased_progress(raise, 0.0, 0.5);
    let raise_middle = eased_progress(raise, 0.5, 0.8);
    let raise_end = eased_progress(raise, 0.8, 1.0);
    let sway_progress = eased_progress(time_held, dismount_start - 20.0, dismount_start);
    let lower = ease_out_cubic(ease_in_out_elastic(eased_progress(
        time_held - 20.0,
        knock_start - 40.0,
        knock_start,
    )));
    let raise_back = eased_progress(time_held, damage_start - 5.0, damage_start);
    let sway_intensity =
        2.0f32 * ease_out_circ(sway_progress) - 2.0f32 * ease_in_circ(raise_back);
    let sway_scale_slow =
        mth_sin(time_held * 19.0f32 * 0.017_453_292_4) * sway_intensity;
    let sway_scale_fast =
        mth_sin(time_held * 30.0f32 * 0.017_453_292_4) * sway_intensity;
    SpearUseParams {
        raise,
        raise_start,
        raise_middle,
        raise_end,
        sway_progress,
        lower,
        raise_back,
        sway_intensity,
        sway_scale_slow,
        sway_scale_fast,
    }
}

/// `SpearAnimations.hitFeedbackAmount(F)F` verbatim: `0.4 ·
/// (outQuart(progress(t, 1.0, 3.0)) - inOutSine(progress(t, 3.0,
/// 10.0)))` — the fsub the operand order speaks (below - top: the
/// outQuart the minuend keeps), the 0.4 the leading ldc folds. The
/// jolt rises across the hit's own first three ticks and settles back
/// to zero by the tenth — the steady state the outQuart(1) =
/// inOutSine(1) collision answers.
pub fn spear_hit_feedback_amount(ticks_since: f32) -> f32 {
    0.4f32
        * (ease_out_quart(eased_progress(ticks_since, 1.0, 3.0))
            - ease_in_out_sine(eased_progress(ticks_since, 3.0, 10.0)))
}

/// `PoseStack.rotateAround(quat, ox, oy, oz)` — the rotation about the
/// pivot point the JOML call composes: `T(o) · R · T(-o)` post-multiplied
/// onto the current pose (the point the pivot frame enters, turns,
/// leaves — the vanilla walk's own local-space read).
fn rot_around(r: Mat4, ox: f32, oy: f32, oz: f32) -> Mat4 {
    Mat4::translation(ox, oy, oz)
        .mul(&r)
        .mul(&Mat4::translation(-ox, -oy, -oz))
}

/// `SpearAnimations.firstPersonUse(F, PoseStack, F, HumanoidArm,
/// ItemStack)` — the KINETIC use lane's own transform, verbatim from
/// the 26.2 bytecode (pcs 41-262): the raise/sway translate — `dx =
/// side·(raise·0.15 + raise_end·(-0.05) + sway·(-0.1) + slow·0.005)`,
/// `dy = raise·(-0.075) + middle·0.075 + fast·0.01`, `dz` the f64 fold
/// `raise_start·0.05 + raise_end·(-0.05)` the ldc2_w pair banks plus
/// the f32 `slow·0.005` — then `rotateAround(XP.rotationDegrees(-65·
/// inOutBack(raise) - 35·lower + 100·raise_back - 0.5·fast), 0, 0.1,
/// 0)`, then `rotateAround(YN.rotationDegrees(side·(-90·progress(raise,
/// 0.5, 0.55) + 90·sway + 2·slow)), side·0.15, 0, 0)` — the YN axis
/// the rotation about -Y the +Y-by--angle fold answers — then the hit
/// feedback translate `(0, -amount, 0)`. At the press frame
/// (`timeHeld` < 0) every term speaks zero: the pose IS the identity,
/// the raise fades in from the tick after the press.
pub fn spear_use_matrix(p: &SpearUseParams, feedback_amount: f32, side: f32) -> Mat4 {
    let t = |x: f32, y: f32, z: f32| Mat4::translation(x, y, z);
    let dx = side
        * (p.raise * 0.15f32
            + p.raise_end * -0.05f32
            + p.sway_progress * -0.1f32
            + p.sway_scale_slow * 0.005f32);
    let dy = p.raise * -0.075f32 + p.raise_middle * 0.075f32 + p.sway_scale_fast * 0.01f32;
    let dz = ((p.raise_start as f64) * 0.05
        + (p.raise_end as f64) * (-0.05)
        + ((p.sway_scale_slow * 0.005f32) as f64)) as f32;
    let rot_x_deg = -65.0f32 * ease_in_out_back(p.raise) - 35.0f32 * p.lower
        + 100.0f32 * p.raise_back
        + -0.5f32 * p.sway_scale_fast;
    let rot_y_deg = side
        * (-90.0f32 * eased_progress(p.raise, 0.5, 0.55)
            + 90.0f32 * p.sway_progress
            + 2.0f32 * p.sway_scale_slow);
    t(dx, dy, dz)
        .mul(&rot_around(
            Mat4::rot_x(rot_x_deg.to_radians()),
            0.0,
            0.1,
            0.0,
        ))
        .mul(&rot_around(
            // `Axis.YN.rotationDegrees(v)` — the rotation about the
            // NEGATIVE Y axis the +Y rotation by -v answers
            Mat4::rot_y((-rot_y_deg).to_radians()),
            side * 0.15f32,
            0.0,
            0.0,
        ))
        .mul(&t(0.0, -feedback_amount, 0.0))
}

impl HeldItem {
    /// The loaded quad count — the tests' honest size probe (the
    /// geometry's own shape, no internals exposed).
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn quad_len(&self) -> usize {
        self.quads.len()
    }

    /// Whether ANY quad binds the named texture — the tests' honest
    /// texture-binding probe.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn binds_texture(&self, tex: &str) -> bool {
        self.quads.iter().any(|q| q.texture == tex)
    }

    /// The quads' vertex centroid — the tests' honest geometry probe
    /// (the pose's own shape: the using branch's stanza moves the
    /// payload, the probe reads WHERE it moved without exposing
    /// internals).
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn centroid(&self) -> [f32; 3] {
        let mut c = [0.0f64; 3];
        let mut n = 0.0f64;
        for q in &self.quads {
            for v in &q.vertices {
                c[0] += v[0] as f64;
                c[1] += v[1] as f64;
                c[2] += v[2] as f64;
                n += 1.0;
            }
        }
        if n > 0.0 {
            [(c[0] / n) as f32, (c[1] / n) as f32, (c[2] / n) as f32]
        } else {
            [0.0; 3]
        }
    }

    /// The anchored, view-space quads at an attack animation progress
    /// (0 = rest; the deterministic scenes never swing, so the goldens —
    /// which hold no item — never see a nonzero one). The eat pose rides
    /// AFTER the anchor (the vanilla renderArmWithItem's own order: the
    /// applyEatTransform folds over the item-arm transform chain).
    #[cfg_attr(not(test), expect(dead_code))]
    fn sampled(&self, swing: f32) -> Vec<crate::raster::SampledQuad<'_>> {
        self.sampled_with_pose(swing, None, 1.0, SwingKind::Whack, None)
    }

    /// The sampled variant with the eat pose and the hand side (the
    /// consumable's own first-person law — the pull-in + the bite bob +
    /// the three rotations, the eat module's verbatim fold; the side
    /// the anchor chain carries, -1 the offhand's). The swing kind is
    /// the held stack's own `SwingAnimation` type — the component the
    /// submitArmWithItem switch reads every frame. The use pose — the
    /// KINETIC lane's `firstPersonUse` matrix — REPLACES the swing
    /// dispatch when present (the vanilla branch order: the using walk
    /// the swing switch never reaches), the bare anchor it composes
    /// over (SPEAR's own `customArmTransform=true` the shared walk
    /// skips, the case-9 translate the only anchor).
    fn sampled_with_pose(
        &self,
        swing: f32,
        eat: Option<crate::eat::EatPose>,
        side: f32,
        kind: SwingKind,
        use_pose: Option<&Mat4>,
    ) -> Vec<crate::raster::SampledQuad<'_>> {
        let m = match use_pose {
            Some(u) => item_hand_anchor(SwingKind::None, swing, 0.0, side).mul(u),
            None => {
                let anchor = item_hand_anchor(kind, swing, 0.0, side);
                match eat {
                    None => anchor,
                    Some(p) => anchor
                        .mul(&crate::math::Mat4::translation(p.dx, p.dy, p.dz))
                        .mul(&crate::math::Mat4::rot_y(p.rot_y_deg.to_radians()))
                        .mul(&crate::math::Mat4::rot_x(p.rot_x_deg.to_radians()))
                        .mul(&crate::math::Mat4::rot_z(p.rot_z_deg.to_radians())),
                }
            }
        };
        self.quads
            .iter()
            .map(|q| crate::raster::SampledQuad {
                pts: q.vertices.map(|v| {
                    let p = m.transform_point(crate::math::Vec3::new(v[0], v[1], v[2]));
                    [p[0], p[1], p[2]]
                }),
                uv: q.uvs,
                tex: q.texture,
                tint: [1.0, 1.0, 1.0],
            })
            .collect()
    }

    /// The hand pass for a held item: clears depth (vanilla clears depth
    /// between the level and the hand) and draws the item in view space
    /// under the world's projection (the hand fov follows the world's).
    /// Vanilla also equips/swings here — the swing terms ride
    /// `item_hand_anchor` above; the equip progress is always 0. The
    /// convenience keeps the DEFAULT component's WHACK lane (the tests'
    /// rest poses — every swing term zero).
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn draw(&self, fb: &mut crate::raster::Framebuf, proj: Mat4, swing: f32) {
        self.draw_with_pose(fb, proj, swing, None, SwingKind::Whack, None);
    }

    /// The draw with the eat pose (the consumable's hold: the item
    /// lifts to the mouth and wobbles — the applyEatTransform's own
    /// law; `None` keeps the rest pose). The MAIN hand's side. The kind
    /// is the held stack's own component type (the STAB spears ride
    /// [`stab_attack_matrix`], everything else the DEFAULT WHACK arc).
    /// The use pose replaces both when present (the KINETIC lane's own
    /// branch).
    pub fn draw_with_pose(
        &self,
        fb: &mut crate::raster::Framebuf,
        proj: Mat4,
        swing: f32,
        eat: Option<crate::eat::EatPose>,
        kind: SwingKind,
        use_pose: Option<&Mat4>,
    ) {
        self.draw_with_pose_side(fb, proj, swing, eat, 1.0, kind, use_pose);
    }

    /// The draw on EITHER hand (the submitArmWithItem walk's own
    /// two-lane shape — scripts/jar_work/handrenderer_51.txt: the
    /// OFF_HAND call the same walk answers with the side*-1 fold, the
    /// item quads themselves the FIRST_PERSON_LEFT_HAND display
    /// context poses — the registry's own left-hand mirror). `side`
    /// = -1 renders the offhand item; the eat pose the MAIN hand's
    /// lane only rides. The kind rides the stack the lane renders —
    /// the offhand's own component, never the main hand's. The use
    /// pose (the `using_item` hand's KINETIC lane) rides the lane the
    /// use gates — `getUsedItemHand() == hand` the vanilla conjunct
    /// speaks, the OTHER lane the swing dispatch keeps.
    pub fn draw_with_pose_side(
        &self,
        fb: &mut crate::raster::Framebuf,
        proj: Mat4,
        swing: f32,
        eat: Option<crate::eat::EatPose>,
        side: f32,
        kind: SwingKind,
        use_pose: Option<&Mat4>,
    ) {
        fb.clear_depth();
        let quads = self.sampled_with_pose(swing, eat, side, kind, use_pose);
        fb.draw_sampled_quads(proj, &quads, &mut |tex, u, v| {
            match sprite_uv::sample_uv(tex, u, v) {
                Sample::Pixel(c) => Some(c),
                Sample::Missing(c) => Some(c),
                Sample::Unknown => None,
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The six quads' UV corners, LITERALLY — the javap decode of
    /// ModelPart$Cube for texOffs (40,16), box 4x12x4 (the UP face's rect
    /// rides v-flipped in the Polygon call). If the unwrap drifts, this
    /// pin names the face.
    #[test]
    fn the_arm_quads_carry_the_vanilla_unwrap() {
        let quads = build_quads();
        assert_eq!(quads.len(), 6);
        let expected_uv: [[[f32; 2]; 4]; 6] = [
            [[48.0, 16.0], [44.0, 16.0], [44.0, 20.0], [48.0, 20.0]], // DOWN
            [[52.0, 20.0], [48.0, 20.0], [48.0, 16.0], [52.0, 16.0]], // UP (v-flipped)
            [[44.0, 20.0], [40.0, 20.0], [40.0, 32.0], [44.0, 32.0]], // WEST
            [[48.0, 20.0], [44.0, 20.0], [44.0, 32.0], [48.0, 32.0]], // NORTH
            [[52.0, 20.0], [48.0, 20.0], [48.0, 32.0], [52.0, 32.0]], // EAST
            [[56.0, 20.0], [52.0, 20.0], [52.0, 32.0], [56.0, 32.0]], // SOUTH
        ];
        for (q, e) in quads.iter().zip(expected_uv) {
            for (c, ec) in q.uv.iter().zip(e) {
                assert_eq!(*c, ec, "uv corner must be the vanilla one");
            }
        }
        // geometry sanity: the whole arm sits in front of the camera, in
        // the bottom-right half of view space (the vanilla rest pose)
        for q in &quads {
            let mut centroid = [0f32; 3];
            for p in &q.pts {
                assert!(p.iter().all(|v| v.is_finite()), "corner must be finite");
                assert!(p[2] < -0.05, "the arm is in front of the camera");
                centroid = [centroid[0] + p[0] / 4.0, centroid[1] + p[1] / 4.0, centroid[2] + p[2] / 4.0];
            }
            assert!(centroid[0] > 0.0, "right of center");
            assert!(centroid[1] < 0.0, "below center");
        }
    }

    /// A synthetic 64x64 RGBA png painted with one distinctive color.
    fn skin_png(color: [u8; 4]) -> Vec<u8> {
        let mut rgba = vec![0u8; 64 * 64 * 4];
        for px in rgba.chunks_exact_mut(4) {
            px.copy_from_slice(&color);
        }
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, 64, 64);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().unwrap();
            wr.write_image_data(&rgba).unwrap();
        }
        out
    }

    #[test]
    fn the_hand_refuses_an_unverified_skin() {
        // a well-formed but WRONG skin: the sha1 gate must reject it so the
        // renderer never draws pixels it cannot vouch for
        let dir = std::env::temp_dir().join(format!("mc2rs-hand-{}", std::process::id()));
        let dest = dir.join("entity/player/wide");
        std::fs::create_dir_all(&dest).unwrap();
        std::fs::write(dest.join("steve.png"), skin_png([250, 10, 10, 255])).unwrap();
        assert!(load(&dir).is_none(), "unverified skin must not render");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn the_hand_loads_the_real_verified_skin_when_present() {
        // the runtime root produced by mc2rs-gen (present locally and in CI;
        // the sha1 gate makes this exact, not best-effort)
        // anchored at the workspace root (assets-cache lives there; cargo
        // runs the test binary from the crate dir, the relative form would
        // silently skip)
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("assets-cache/jar");
        if !dir.join(skin::RUNTIME_PATH).exists() {
            return; // hermetic environments skip: no asset, no assertion
        }
        let arm = load(&dir).expect("the real jar skin verifies");
        assert_eq!(arm.quads.len(), 6);
    }

    #[test]
    fn the_slim_arm_carries_the_javap_box() {
        // PlayerModel.createMesh(CubeDeformation, slim = true), javap on the
        // official 26.2 jar: right_arm texOffs(40, 16) UNCHANGED,
        // addBox(-2, -2, -2, 3, 12, 4), pivot (-5, 2, 0) UNCHANGED (the
        // transform chain is kind-independent). Only the box shrinks.
        assert_eq!(skin::SLIM_ARM_BOX_PX, [3, 12, 4]);
        assert_eq!(skin::SLIM_ARM_BOX_ORIGIN_PX, [-2, -2, -2]);
        assert_eq!(skin::ARM_TEX_OFFS, (40, 16), "shared texOffs");
        // and the generated slim unwrap is the same ModelPart layout at
        // width 3: strip right(outer) -> front -> left(inner) -> back
        assert_eq!(
            [
                (skin::SLIM_ARM_FACES[0].u, skin::SLIM_ARM_FACES[0].v, skin::SLIM_ARM_FACES[0].w, skin::SLIM_ARM_FACES[0].h),
                (skin::SLIM_ARM_FACES[1].u, skin::SLIM_ARM_FACES[1].v, skin::SLIM_ARM_FACES[1].w, skin::SLIM_ARM_FACES[1].h),
                (skin::SLIM_ARM_FACES[2].u, skin::SLIM_ARM_FACES[2].v, skin::SLIM_ARM_FACES[2].w, skin::SLIM_ARM_FACES[2].h),
                (skin::SLIM_ARM_FACES[3].u, skin::SLIM_ARM_FACES[3].v, skin::SLIM_ARM_FACES[3].w, skin::SLIM_ARM_FACES[3].h),
                (skin::SLIM_ARM_FACES[4].u, skin::SLIM_ARM_FACES[4].v, skin::SLIM_ARM_FACES[4].w, skin::SLIM_ARM_FACES[4].h),
                (skin::SLIM_ARM_FACES[5].u, skin::SLIM_ARM_FACES[5].v, skin::SLIM_ARM_FACES[5].w, skin::SLIM_ARM_FACES[5].h),
            ],
            [
                (44, 16, 3, 4), // top
                (47, 16, 3, 4), // bottom
                (40, 20, 4, 12), // right (outer)
                (44, 20, 3, 12), // front
                (47, 20, 4, 12), // left (inner)
                (51, 20, 3, 12), // back
            ]
        );
    }

    /// The (u, v, w, h) rect a HandQuad's corner assignment spans (the
    /// Polygon constructor's fixed rect corners; min/max reconstructs it
    /// even for the V-flipped wrist face, whose rect height is negative).
    fn uv_rect(q: &HandQuad) -> (u32, u32, u32, u32) {
        let us: Vec<f32> = q.uv.iter().map(|c| c[0]).collect();
        let vs: Vec<f32> = q.uv.iter().map(|c| c[1]).collect();
        let (umin, umax) = (us.iter().cloned().fold(f32::MAX, f32::min), us.iter().cloned().fold(f32::MIN, f32::max));
        let (vmin, vmax) = (vs.iter().cloned().fold(f32::MAX, f32::min), vs.iter().cloned().fold(f32::MIN, f32::max));
        (
            umin as u32,
            vmin as u32,
            (umax - umin) as u32,
            (vmax - vmin) as u32,
        )
    }

    #[test]
    fn the_slim_quads_unfold_the_three_wide_strip() {
        let slim = build_quads_kind(ArmKind::Slim);
        let wide = build_quads_kind(ArmKind::Wide);
        assert_eq!(slim.len(), 6);
        // every slim quad spans EXACTLY its generated slim face rect
        for (q, r) in slim.iter().zip(skin::SLIM_ARM_FACES.iter()) {
            assert_eq!(uv_rect(q), (r.u as u32, r.v as u32, r.w as u32, r.h as u32));
        }
        // and the wide quads still span the wide rects (the default path
        // is untouched by the parameterization)
        for (q, r) in wide.iter().zip(skin::ARM_FACES.iter()) {
            assert_eq!(uv_rect(q), (r.u as u32, r.v as u32, r.w as u32, r.h as u32));
        }
    }

    #[test]
    fn the_slim_box_moves_the_inner_plane_not_the_outer() {
        // model-space geometry: the right arm's OUTER plane is x = +1 in
        // BOTH kinds (wide -3+4, slim -2+3) — the outer face's corners are
        // IDENTICAL after the same transform; the inner plane moves
        // x = -3 -> -2, so every other face's inner corners move.
        let wide = build_quads_kind(ArmKind::Wide);
        let slim = build_quads_kind(ArmKind::Slim);
        // face 4 = EAST = the x1 plane (shared)
        assert_eq!(
            wide[4].pts, slim[4].pts,
            "the outer plane's corners are kind-independent"
        );
        assert_ne!(
            wide[4].uv, slim[4].uv,
            "the outer face's TEXELS differ (the unwrap shifts with w)"
        );
        // face 2 = WEST = the x0 plane (moved)
        assert_ne!(
            wide[2].pts, slim[2].pts,
            "the inner plane's corners move with the box origin"
        );
    }

    #[test]
    fn the_slim_arm_loads_the_real_alex_skin_when_present() {
        // the same hermetic contract as the wide skin: sha1-verified or
        // not drawn; the slim runtime file rides the generator's
        // materialize step like steve.png does
        // anchored at the workspace root (assets-cache lives there; cargo
        // runs the test binary from the crate dir, the relative form would
        // silently skip)
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("assets-cache/jar");
        if !dir.join(skin::SLIM_RUNTIME_PATH).exists() {
            return; // hermetic environments skip: no asset, no assertion
        }
        let arm = load_kind(&dir, ArmKind::Slim).expect("the real jar alex skin verifies");
        assert_eq!(arm.quads.len(), 6);
        // the gate is per-kind: the slim file must NOT verify as wide and
        // vice versa (different sha1s, different paths)
        assert_ne!(skin::SLIM_ALEX_PNG_SHA1, skin::STEVE_WIDE_PNG_SHA1);
    }

    #[test]
    fn the_slim_arm_renders_a_different_pixel_set_probe_pinned() {
        // PROBE-FIRST discipline (the v0.3.78/81 lesson): the slim arm's
        // rest pose renders a DIFFERENT pixel set than the wide arm (the
        // box shrank one model unit inward) — the exact counts below were
        // probed with this very render before being pinned.
        // anchored at the workspace root (assets-cache lives there; cargo
        // runs the test binary from the crate dir, the relative form would
        // silently skip)
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("assets-cache/jar");
        if !dir.join(skin::RUNTIME_PATH).exists() || !dir.join(skin::SLIM_RUNTIME_PATH).exists() {
            return; // hermetic environments skip: no assets, no assertion
        }
        let wide = load_kind(&dir, ArmKind::Wide).expect("wide verifies");
        let slim = load_kind(&dir, ArmKind::Slim).expect("slim verifies");
        let proj = Mat4::perspective(70.0f32.to_radians(), 320.0 / 180.0, 0.1, 512.0);
        let mut wfb = crate::raster::Framebuf::new(320, 180);
        wfb.clear([0.45, 0.66, 0.94], [0.74, 0.85, 0.95]);
        wide.draw(&mut wfb, proj, 0.0);
        let mut sfb = crate::raster::Framebuf::new(320, 180);
        sfb.clear([0.45, 0.66, 0.94], [0.74, 0.85, 0.95]);
        slim.draw(&mut sfb, proj, 0.0);
        let mut changed = 0usize;
        for i in 0..320 * 180 {
            let o = i * 3;
            if wfb.color[o..o + 3] != sfb.color[o..o + 3] {
                changed += 1;
            }
        }
        // the probe-observed count (in-test, plain-sky background): the
        // moved inner plane + the shifted unwrap texels touch 2534 pixels
        assert_eq!(changed, 2534, "the slim rest pose moves exact pixels");
        // sanity: the variant is a real geometry change, not a recolor
        assert!(changed > 500, "a one-unit box shrink must move real pixels");
    }
}


#[cfg(test)]
mod swing_tests {
    use super::*;

    /// The vanilla swing timeline over 7 ticks after start(): swingTime
    /// walks -1 -> 0 -> 1 -> ... -> 6 (reset to rest at the 6-tick
    /// duration), attackAnim = swingTime/6 exactly.
    #[test]
    fn the_swing_state_walks_the_vanilla_timeline() {
        let mut s = SwingState::default();
        assert!(!s.swinging);
        assert_eq!(s.get_attack_anim(0.5), 0.0, "rest: no swing");
        s.start();
        assert!(s.swinging);
        assert_eq!(s.swing_time, -1, "LivingEntity.swing pins swingTime to -1");
        let expected: [f32; 7] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 0.0]
            .map(|k| k / 6.0); // the bytecode's swingTime/duration fdiv
        for e in expected {
            s.tick();
            assert_eq!(s.attack_anim, e, "attackAnim = swingTime/6");
        }
        assert!(!s.swinging, "the swing ended at the 6-tick duration");
        assert_eq!(s.swing_time, 0);
    }

    /// LivingEntity.swing's guard: an early re-trigger (swingTime in
    /// [0, dur/2)) is IGNORED; past the half-point it restarts.
    #[test]
    fn the_swing_guard_ignores_early_retrigger() {
        let mut s = SwingState::default();
        s.start();
        s.tick(); // swingTime 0
        s.tick(); // swingTime 1
        s.start(); // 1 < 6/2 -> ignored
        assert_eq!(s.swing_time, 1, "early re-trigger must be ignored");
        s.tick();
        s.tick(); // swingTime 3
        s.start(); // 3 >= 6/2 -> restarts
        assert_eq!(s.swing_time, -1, "past the half-point the swing restarts");
        s.tick();
        assert_eq!(s.swing_time, 0);
    }

    /// The rest pose is EXACTLY the zero-swing pose: every swing term is
    /// zero at 0 and the two added rotations are the exact identity — the
    /// goldens (always rendered at rest) cannot drift.
    #[test]
    fn the_zero_swing_pose_is_bit_identical_to_rest() {
        let rest = build_quads();
        let at_zero = build_quads_at(0.0, 0.0);
        assert_eq!(rest.len(), at_zero.len());
        for (r, z) in rest.iter().zip(&at_zero) {
            assert_eq!(r.pts, z.pts, "corner positions must be bit-identical");
            assert_eq!(r.uv, z.uv);
        }
    }

    /// The swing terms move the arm (same UV unwrap, different geometry).
    #[test]
    fn the_swing_transform_moves_the_arm() {
        let rest = build_quads();
        let mid = build_quads_at(0.5, 0.0);
        let mut moved = 0;
        for (r, m) in rest.iter().zip(&mid) {
            assert_eq!(r.uv, m.uv, "the UV unwrap never changes");
            if r.pts != m.pts {
                moved += 1;
            }
        }
        assert!(moved > 0, "the mid-swing pose must differ from rest");
    }

    /// getAttackAnim's lerp + the wrap guard (progress 5/6 -> 0 must wrap
    /// to +1/6, not lerp backwards across the seam).
    #[test]
    fn the_attack_anim_lerps_with_the_wrap_guard() {
        let mut s = SwingState::default();
        s.tick();
        s.start();
        s.tick(); // swingTime 0, attackAnim 0, o = 0
        s.tick(); // swingTime 1, attackAnim 1/6, o = 0
        let a = s.get_attack_anim(0.5);
        assert!((a - (1.0f32 / 6.0) * 0.5).abs() < 1e-6, "mid-tick lerp");
        // walk to the seam: the reset tick itself sets o = 5/6, attackAnim = 0
        while s.swinging {
            s.tick();
        }
        let wrapped = s.get_attack_anim(0.5);
        assert!(
            (wrapped - (5.0 / 6.0 + (1.0 / 6.0) * 0.5)).abs() < 1e-6,
            "the 1->0 jump must wrap forward, not lerp backwards: {wrapped}"
        );
    }

    /// The frozen stand-in samples EXACTLY its progress at every partial
    /// tick (the o/attack lerp is the identity on a no-change frame), and
    /// progress 0 samples exactly like the rest default.
    #[test]
    fn the_frozen_swing_state_samples_its_progress() {
        for p in [0.0f32, 0.25, 0.5, 1.0] {
            let s = SwingState::frozen_at(p);
            for partial in [0.0f32, 0.5, 1.0] {
                assert_eq!(
                    s.get_attack_anim(partial),
                    p,
                    "frozen_at({p}) must sample {p} at partial {partial}"
                );
            }
        }
        let rest = SwingState::default();
        assert_eq!(
            SwingState::frozen_at(0.0).get_attack_anim(0.5),
            rest.get_attack_anim(0.5),
            "frozen 0 IS the rest pose"
        );
    }
}

#[cfg(test)]
mod held_tests {
    use super::*;
    use crate::raster::Framebuf;

    #[test]
    fn the_held_item_loads_the_vanilla_sword() {
        // the display arc's end-to-end over the canonical fixture: the
        // ItemModelGenerator bake (82 quads, the census pin) through the
        // handheld firstperson_righthand pose
        let item = load_item("item/iron_sword").expect("iron_sword is in the bake domain");
        assert_eq!(item.quads.len(), 82);
        // the element path bakes the block models too — stone rides the
        // cube chain's one element (the full cube), its face refs resolved
        let stone = load_item("block/stone").expect("the element bake covers the cube");
        assert_eq!(stone.quads.len(), 6, "the full cube's six faces");
        assert!(stone.quads.iter().all(|q| !q.texture.starts_with('#')));
        // unknown -> nothing to draw
        assert!(load_item("item/definitely_not_a_thing").is_none());
    }

    #[test]
    fn the_layers_load_the_complex_definitions_and_the_beds() {
        // the bow (922): the condition's on_false branch — the resting
        // frame, the single plain layer
        let layers = mc_registry::item_model_layers(922).expect("the bow resolves now");
        assert_eq!(layers.len(), 1);
        assert_eq!(layers[0].model, "item/bow");
        let item = load_item_layers(&layers).expect("the resting bow loads");
        assert!(!item.quads.is_empty());

        // the bed (1130): the composite's two layers ride ONE HeldItem —
        // the head, then the foot shifted by the translation, both baked
        // through the element path (the template inheritance)
        let layers = mc_registry::item_model_layers(1130).expect("the bed resolves");
        assert_eq!(layers.len(), 2);
        assert_eq!(layers[0].model, "block/black_bed_head");
        assert_eq!(layers[1].model, "block/black_bed_foot");
        let bed = load_item_layers(&layers).expect("both layers bake");
        let head = load_item_layers(&layers[..1]).expect("the head alone bakes");
        assert_eq!(bed.quads.len(), head.quads.len() * 2, "head + foot");
        // the foot's quads ARE the head's geometry displaced: same quad
        // count, different vertices (the z=1 translation rode)
        assert!(
            bed.quads[head.quads.len()..]
                .iter()
                .zip(head.quads.iter())
                .any(|(f, h)| f.vertices != h.vertices),
            "the foot's vertices moved"
        );
        // the element path resolved the face refs to texture ids (the
        // sampler's vocabulary)
        for q in &bed.quads {
            assert!(!q.texture.starts_with('#'), "resolved, not a ref: {}", q.texture);
        }

        // the spear (1329): the select's fallback — the in_hand model,
        // the swap scale riding the resolved row
        let layers = mc_registry::item_model_layers(1329).expect("the spear resolves");
        assert_eq!(layers[0].model, "item/iron_spear_in_hand");

        // the shield (1325): the special renderer's honest None
        assert!(mc_registry::item_model_layers(1325).is_none());
        assert!(load_item_layers(&[]).is_none(), "nothing to load");
    }

    /// The LEFT arm's own unwrap — javap PlayerModel.createMesh (scripts/
    /// jar_work/playermodel_51.txt): the player model's left arm the
    /// texOffs (32, 48) band wears (the HumanoidModel's mirror() pair the
    /// rebuild replaces), the box (-1, -2, -2, 4, 12, 4). The same
    /// fixed corner->rect mapping the right arm's golden-verified decode
    /// carries.
    #[test]
    fn the_left_arm_carries_the_player_model_unwrap() {
        let quads = build_arm_quads_at(ArmKind::Wide, 0.0, 0.0, true);
        assert_eq!(quads.len(), 6);
        // the (32, 48) band's own six rects (d=4, w=4, h=12)
        let expected_uv: [[[f32; 2]; 4]; 6] = [
            [[40.0, 48.0], [36.0, 48.0], [36.0, 52.0], [40.0, 52.0]], // DOWN
            [[44.0, 52.0], [40.0, 52.0], [40.0, 48.0], [44.0, 48.0]], // UP (v-flipped)
            [[36.0, 52.0], [32.0, 52.0], [32.0, 64.0], [36.0, 64.0]], // WEST (outer)
            [[40.0, 52.0], [36.0, 52.0], [36.0, 64.0], [40.0, 64.0]], // NORTH
            [[44.0, 52.0], [40.0, 52.0], [40.0, 64.0], [44.0, 64.0]], // EAST (inner)
            [[48.0, 52.0], [44.0, 52.0], [44.0, 64.0], [48.0, 64.0]], // SOUTH
        ];
        for (q, e) in quads.iter().zip(expected_uv) {
            for (c, ec) in q.uv.iter().zip(e) {
                assert_eq!(*c, ec, "the left arm's uv corner must be the (32, 48) one");
            }
        }
    }

    /// The LEFT arm's rest pose (the renderPlayerArm walk's own LEFT
    /// lane — scripts/jar_work/handrenderer_51.txt the `loc8=-1.0 *`
    /// terms): every corner in front of the camera, in the BOTTOM-LEFT
    /// half of view space (the mirror of the right arm's own law).
    #[test]
    fn the_left_arm_rests_in_the_bottom_left() {
        let quads = build_arm_quads_at(ArmKind::Wide, 0.0, 0.0, true);
        for q in &quads {
            let mut centroid = [0f32; 3];
            for p in &q.pts {
                assert!(p.iter().all(|v| v.is_finite()), "corner must be finite");
                assert!(p[2] < -0.05, "the arm is in front of the camera");
                centroid = [
                    centroid[0] + p[0] / 4.0,
                    centroid[1] + p[1] / 4.0,
                    centroid[2] + p[2] / 4.0,
                ];
            }
            assert!(centroid[0] < 0.0, "left of center (the offhand's own side)");
            assert!(centroid[1] < 0.0, "below center");
        }
    }

    /// The anchor chain's own x-mirror symmetry (the side*-1 fold the
    /// 26.2 walk carries: the LEFT chain the RIGHT one's mirror is —
    /// the M∘chain(+1)∘M = chain(-1) law the side-factored terms speak;
    /// the rot_x(200°) the x-flip's own fixed point keeps). The left
    /// anchor applied to the mirrored point equals the mirrored right
    /// result — the two hands the one walk renders.
    #[test]
    fn the_offhand_anchor_mirrors_the_main_anchor() {
        let right = item_hand_anchor(SwingKind::Whack, 0.0, 0.0, 1.0);
        let left = item_hand_anchor(SwingKind::Whack, 0.0, 0.0, -1.0);
        let mirror = |p: [f32; 3]| [-p[0], p[1], p[2]];
        for p in [[0.5f32, -0.3f32, -0.7f32], [-1.2f32, 2.0f32, 0.4f32], [0.0f32, 0.0f32, 0.0f32]] {
            let mp = mirror(p);
            let via_left = left.transform_point(crate::math::Vec3::new(mp[0], mp[1], mp[2]));
            let via_right = right.transform_point(crate::math::Vec3::new(p[0], p[1], p[2]));
            let mirrored_right = mirror([via_right[0], via_right[1], via_right[2]]);
            for k in 0..3 {
                assert!(
                    (via_left[k] - mirrored_right[k]).abs() < 1e-4,
                    "the left anchor the right one's mirror is: axis {k}"
                );
            }
        }
    }

    /// The OFFHAND ITEM's own lane (the FIRST_PERSON_LEFT_HAND display
    /// context the registry's left-hand mirror applies, the side*-1
    /// anchor the draw rides): the sword's left quads rest in the
    /// bottom-left half of view space — the vanilla dual-wield look.
    #[test]
    fn the_offhand_item_rests_in_the_bottom_left() {
        let item = load_item_ctx("item/iron_sword", mc_registry::display::FIRSTPERSON_LEFTHAND)
            .expect("the left-hand sword bakes");
        let m = item_hand_anchor(SwingKind::Whack, 0.0, 0.0, -1.0);
        let mut mean_y = 0.0f32;
        let mut n = 0.0f32;
        for q in &item.quads {
            for v in &q.vertices {
                let p = m.transform_point(crate::math::Vec3::new(v[0], v[1], v[2]));
                assert!(p.iter().all(|c| c.is_finite()), "finite corners");
                assert!(p[2] < 0.0, "in front of the camera: {p:?}");
                mean_y += p[1];
                n += 1.0;
                assert!(p[0] < 0.0 || p[0].abs() < 0.35, "left of the view's center: {p:?}");
            }
        }
        assert!(mean_y / n < 0.0, "the bulk below center");
    }

    #[test]
    fn the_anchored_item_rests_in_the_bottom_right() {
        // the anchor chain (applyItemArmTransform + the at-rest swing
        // terms) must land every corner in front of the camera, right of
        // center; the BULK sits below center (the blade tip quads may
        // cross the horizontal midline — the vanilla held-sword look)
        let item = load_item("item/iron_sword").expect("iron_sword loads");
        let m = item_hand_anchor(SwingKind::Whack, 0.0, 0.0, 1.0);
        let mut mean_y = 0.0f32;
        for q in &item.quads {
            let mut centroid = [0f32; 3];
            for v in &q.vertices {
                let p = m.transform_point(crate::math::Vec3::new(v[0], v[1], v[2]));
                assert!(p.iter().all(|c| c.is_finite()), "finite corners");
                assert!(p[2] < 0.0, "in front of the camera: {p:?}");
                centroid = [
                    centroid[0] + p[0] / 4.0,
                    centroid[1] + p[1] / 4.0,
                    centroid[2] + p[2] / 4.0,
                ];
            }
            assert!(centroid[0] > 0.0, "right of center: {}", centroid[0]);
            mean_y += centroid[1] / item.quads.len() as f32;
        }
        assert!(mean_y < 0.0, "bulk below center: {mean_y}");
    }

    #[test]
    fn the_item_swing_moves_the_quads() {
        let item = load_item("item/iron_sword").expect("iron_sword loads");
        let rest = item.sampled(0.0);
        let mid = item.sampled(0.5);
        assert_eq!(rest.len(), mid.len());
        let mut moved = 0;
        for (r, m) in rest.iter().zip(&mid) {
            assert_eq!(r.uv, m.uv, "the UV unwrap never changes");
            assert_eq!(r.tex, m.tex);
            if r.pts != m.pts {
                moved += 1;
            }
        }
        assert!(moved > 0, "the mid-swing pose must differ from rest");
    }

    #[test]
    fn the_held_item_draws_pixels_in_its_view_region() {
        let item = load_item("item/iron_sword").expect("iron_sword loads");
        let proj = Mat4::perspective(
            70.0f32.to_radians(),
            320.0 / 180.0,
            0.1,
            512.0,
        );
        let mut base = Framebuf::new(320, 180);
        base.clear([0.45, 0.66, 0.94], [0.74, 0.85, 0.95]);
        let mut drawn = Framebuf::new(320, 180);
        drawn.clear([0.45, 0.66, 0.94], [0.74, 0.85, 0.95]);
        item.draw(&mut drawn, proj, 0.0);
        // every changed pixel sits right of center (all view-space x > 0
        // through a symmetric perspective), the item actually rasterized
        let (mut left, mut right) = (0usize, 0usize);
        for y in 0..180 {
            for x in 0..320 {
                let i = (y * 320 + x) * 3;
                if base.color[i..i + 3] != drawn.color[i..i + 3] {
                    if x < 160 {
                        left += 1;
                    } else {
                        right += 1;
                    }
                }
            }
        }
        assert!(right > 0, "the sword must rasterize on the right half");
        assert_eq!(left, 0, "no pixels left of center");
    }

    /// The swing golden's premise, proven at the pixel level BEFORE the
    /// pin (the same proof the held golden made against plain world): the
    /// mid-swing scene really changes pixels vs the rest scene. The BULK
    /// of the change is right of center; a FEW pixels may flip left of it
    /// — the blade tip quads already cross the midline at rest (the
    /// vanilla held-sword look), so they swing there too.
    #[test]
    fn the_swing_scene_changes_pixels_against_the_rest_scene() {
        let item = load_item("item/iron_sword").expect("iron_sword loads");
        let proj = Mat4::perspective(
            70.0f32.to_radians(),
            320.0 / 180.0,
            0.1,
            512.0,
        );
        let mut rest = Framebuf::new(320, 180);
        rest.clear([0.45, 0.66, 0.94], [0.74, 0.85, 0.95]);
        item.draw(&mut rest, proj, 0.0);
        let mut mid = Framebuf::new(320, 180);
        mid.clear([0.45, 0.66, 0.94], [0.74, 0.85, 0.95]);
        item.draw(&mut mid, proj, 0.5);
        let (mut changed_left, mut changed_right) = (0usize, 0usize);
        for y in 0..180 {
            for x in 0..320 {
                let i = (y * 320 + x) * 3;
                if rest.color[i..i + 3] != mid.color[i..i + 3] {
                    if x < 160 {
                        changed_left += 1;
                    } else {
                        changed_right += 1;
                    }
                }
            }
        }
        assert!(changed_right > 0, "the swing must move pixels on the right half");
        assert!(
            changed_left < changed_right,
            "the change bulk stays right of center (the blade tip may cross): {changed_left} vs {changed_right}"
        );
    }

    /// The swing-arc SWEEP: both first-person term curves (the bare
    /// arm's f10..f14 and the held item's f5s/f6s/f7s/f5/f6) pinned
    /// BIT-EXACTLY at seven progresses against an independently computed
    /// oracle (a numpy f32 emulation of the f64-sin/sqrt-narrowed-to-f32
    /// chain, scripts/swing_terms_oracle.py). Three facts the tables
    /// lock: (1) rest carries EXACT zeros — including the two NEGATIVE
    /// zeros of the -0.3/-0.4 multiplied terms, which an IEEE == cannot
    /// distinguish but from_bits can; (2) progress 1 is NOT rest at the
    /// term level — sin(pi_f32) is -8.742278e-8, the f64-narrowing
    /// fingerprint a f32-sin rewrite would flip; (3) the mid-arc shape:
    /// the rotY amount hits EXACT 1.0 at p=0.25 (sin(pi/2)) and the Z
    /// pull EXACT -0.4/-0.2 at p=0.5 (sin(pi/2) again), the peak rotZ
    /// 0.98078525 near p=0.75.
    #[test]
    fn the_swing_term_sweep_pins_both_curves_bit_exactly() {
        // [f10, f11, f12, f13, f14] = swing_terms, as f32 bit patterns
        const ARM: [(u32, [u32; 5]); 7] = [
            (0.0f32.to_bits(), [0x80000000, 0x00000000, 0x80000000, 0x00000000, 0x00000000]),
            (0.1f32.to_bits(), [0xbe80b44f, 0x3ebb52b7, 0xbdfd2590, 0x3d00a892, 0x3f5681d9]),
            (0.25f32.to_bits(), [0xbe99999a, 0xb31630f2, 0xbe90d0c3, 0x3e47c5c2, 0x3f800000]),
            (0.5f32.to_bits(), [0xbe746fdc, 0xbec56841, 0xbecccccd, 0x3f3504f3, 0x3f4bb28c]),
            (0.75f32.to_bits(), [0xbdfb077c, 0xbe98bf39, 0xbe90d0c3, 0x3f7b14be, 0x3ed130e7]),
            (0.9f32.to_bits(), [0xbd453ed3, 0xbe01cab3, 0xbdfd2592, 0x3f0fe4b4, 0x3e245f05]),
            (1.0f32.to_bits(), [0x32e1496b, 0x339630f2, 0x331630f2, 0xb3bbbd2e, 0xb3bbbd2e]),
        ];
        // [f5s, f6s, f7s, f5, f6] = item_swing_terms
        const ITEM: [(u32, [u32; 5]); 7] = [
            (0.0f32.to_bits(), [0x80000000, 0x00000000, 0x80000000, 0x00000000, 0x00000000]),
            (0.1f32.to_bits(), [0xbeab9b14, 0x3e3b52b7, 0xbd7d2590, 0x3d00a892, 0x3f5681d9]),
            (0.25f32.to_bits(), [0xbecccccd, 0xb29630f2, 0xbe10d0c3, 0x3e47c5c2, 0x3f800000]),
            (0.5f32.to_bits(), [0xbea2f53d, 0xbe456841, 0xbe4ccccd, 0x3f3504f3, 0x3f4bb28c]),
            (0.75f32.to_bits(), [0xbe275a53, 0xbe18bf39, 0xbe10d0c3, 0x3f7b14be, 0x3ed130e7]),
            (0.9f32.to_bits(), [0xbd837f37, 0xbd81cab3, 0xbd7d2592, 0x3f0fe4b4, 0x3e245f05]),
            (1.0f32.to_bits(), [0x331630f2, 0x331630f2, 0x329630f2, 0xb3bbbd2e, 0xb3bbbd2e]),
        ];
        for (p_bits, want) in ARM {
            let p = f32::from_bits(p_bits);
            let got: [u32; 5] = swing_terms(p).map(|t| t.to_bits());
            assert_eq!(got, want, "arm terms at p={p}");
        }
        for (p_bits, want) in ITEM {
            let p = f32::from_bits(p_bits);
            let got: [u32; 5] = item_swing_terms(p).map(|t| t.to_bits());
            assert_eq!(got, want, "item terms at p={p}");
        }
    }

    /// The progress-1 pose is NOT the rest pose at the geometry level —
    /// every term carries the pi_f32 epsilon — yet the RENDERED frame is
    /// bit-identical to rest (the sweep's next test pins that): the
    /// rasterizer's sample grid absorbs a ~1e-7 vertex delta. The UV
    /// unwrap never moves (the same invariant the 0.5 test asserts).
    #[test]
    fn the_one_pose_terms_are_not_the_rest_terms() {
        let rest_terms = swing_terms(0.0);
        let one_terms = swing_terms(1.0);
        assert_ne!(
            rest_terms.map(|t| t.to_bits()),
            one_terms.map(|t| t.to_bits()),
            "p=1 must carry the pi_f32 epsilon, not exact zeros"
        );
        let rest = build_quads();
        let one = build_quads_at(1.0, 0.0);
        assert_eq!(rest.len(), one.len());
        let mut uv_equal = true;
        let mut pts_differ = false;
        for (r, o) in rest.iter().zip(&one) {
            if r.uv != o.uv {
                uv_equal = false;
            }
            if r.pts != o.pts {
                pts_differ = true;
            }
        }
        assert!(uv_equal, "the UV unwrap never changes");
        assert!(pts_differ, "the epsilon terms must move the corner bits");
    }

    /// The pixel-arc sweep over the held-item scene: progress 0.25 and
    /// 0.75 really move pixels (the bulk right of center — 0.75's arc
    /// never crosses the midline at all, 0.25's blade tip does), and
    /// progress 1.0 is BIT-IDENTICAL to rest at the pixel level — the
    /// closed arc: every vanilla swing term is a sin multiple that
    /// vanishes at both ends, and the ~1e-8 pi_f32 residue the floats
    /// carry changes no rasterized pixel. The sweep's in-test counts are
    /// pinned exactly (probe-observed then frozen); the binary probe over
    /// the full world scene agrees structurally — 6240/5583/0 at
    /// 0.25/0.75/1.0 — and its LEFT counts match these to the pixel
    /// (337/0), the coverage flips being background-independent.
    #[test]
    fn the_swing_scene_sweep_pins_the_pixel_arc() {
        let item = load_item("item/iron_sword").expect("iron_sword loads");
        let proj = Mat4::perspective(
            70.0f32.to_radians(),
            320.0 / 180.0,
            0.1,
            512.0,
        );
        let mut rest = Framebuf::new(320, 180);
        rest.clear([0.45, 0.66, 0.94], [0.74, 0.85, 0.95]);
        item.draw(&mut rest, proj, 0.0);
        for p in [0.25f32, 0.75, 1.0] {
            let mut scene = Framebuf::new(320, 180);
            scene.clear([0.45, 0.66, 0.94], [0.74, 0.85, 0.95]);
            item.draw(&mut scene, proj, p);
            let (mut changed_left, mut changed_right) = (0usize, 0usize);
            for y in 0..180 {
                for x in 0..320 {
                    let i = (y * 320 + x) * 3;
                    if rest.color[i..i + 3] != scene.color[i..i + 3] {
                        if x < 160 {
                            changed_left += 1;
                        } else {
                            changed_right += 1;
                        }
                    }
                }
            }
            if p == 1.0 {
                assert_eq!(
                    (changed_left, changed_right),
                    (0, 0),
                    "the arc closes: p=1 must render bit-identical to rest"
                );
            } else {
                // the frozen probe-observed counts (in-test, plain-sky
                // background)
                let want = if p == 0.25 { (337, 6146) } else { (0, 5684) };
                assert_eq!(
                    (changed_left, changed_right),
                    want,
                    "the pixel arc at p={p} is pinned exactly"
                );
                assert!(
                    changed_left < changed_right,
                    "the bulk stays right of center at p={p}"
                );
            }
        }
    }

    /// THE STAB'S OWN EASES (javap `SpearAnimations.firstPersonAttack`
    /// pcs 0-35 + the `Ease` bodies): the three windows the f32 folds
    /// keep — inOutSine snaps shut by p=0.05, outBack overshoots to
    /// 1.0877 mid-recovery and settles by 0.2, inOutExpo splits the slow
    /// unwind at exactly 0.5 when p rides the window's own midpoint
    /// (0.7). The windows the `progress(p, min, max)` clamps name: 0.05
    /// below the first window's floor speaks 0, 0.3 between the recovery
    /// and the unwind holds [1, 1, 0].
    #[test]
    fn the_stab_eases_the_three_windows_speak() {
        // the raw ease laws (the folds the jar's Ease class keeps)
        assert_eq!(ease_in_out_sine(0.0), 0.0);
        assert_eq!(ease_in_out_sine(0.5), 0.5);
        assert_eq!(ease_in_out_sine(1.0), 1.0);
        assert_eq!(ease_out_back(0.0), 0.0);
        assert_eq!(ease_out_back(1.0), 1.0);
        // the overshoot the back-swing speaks (the 2.70158 cube term)
        assert_eq!(ease_out_back(0.5), 1.087_697_5);
        assert_eq!(ease_out_back(0.25), 0.817_409_75);
        assert_eq!(ease_in_out_expo(0.0), 0.0);
        assert_eq!(ease_in_out_expo(1.0), 1.0);
        assert_eq!(ease_in_out_expo(0.5), 0.5);
        // the progress clamp (SpearAnimations.progress over Mth.inverseLerp)
        assert_eq!(eased_progress(-0.5, 0.0, 0.05), 0.0);
        assert_eq!(eased_progress(1.5, 0.0, 0.05), 1.0);
        // the three-phase walk over the swing progress
        let at = |p: f32| {
            let [f4, f5, f6] = stab_eases(p);
            (f4, f5, f6)
        };
        assert_eq!(at(0.0), (0.0, 0.0, 0.0), "the rest the identity keeps");
        assert_eq!(at(0.05), (1.0, 0.0, 0.0), "the thrust snaps shut by 0.05");
        assert_eq!(at(0.125), (1.0, 1.087_697_5, 0.0), "the recovery overshoots");
        assert_eq!(at(0.2), (1.0, 1.0, 0.0), "the recovery settles");
        assert_eq!(at(0.3), (1.0, 1.0, 0.0), "the hold between the windows");
        // the f32 window: (0.7-0.4)/(1.0-0.4) lands a hair BELOW 0.5 (the
        // f32 subtract/divide round), the first expo branch the pow keeps
        assert_eq!(at(0.7), (1.0, 1.0, 0.499_999_58), "the unwind's midpoint");
        assert_eq!(at(1.0), (1.0, 1.0, 1.0), "the fold returns to the anchor");
    }

    /// THE STAB ANCHOR'S OWN SHAPE (javap `firstPersonAttack` pcs 37-103
    /// over the anchor walk): at rest the three eases speak zero — the
    /// matrix collapses to the bare applyItemArmTransform the NONE arm
    /// carries (bit-for-bit); at the 0.05 peak the origin sits at the
    /// full thrust (side·0.1, -0.075, 0.65) with the item's up-axis
    /// swung -70° about X; at p=1 the fold re-collides at zero (the
    /// closed stab, the identity again); the side flips the X term only
    /// (the mirror the offhand lane wears).
    #[test]
    fn the_stab_anchor_the_identity_and_the_peak_speak() {
        // the rest: STAB == NONE == the bare anchor (the switch's two
        // arms agree bit-for-bit when the swing is silent)
        let rest_stab = item_hand_anchor(SwingKind::Stab, 0.0, 0.0, 1.0);
        let rest_none = item_hand_anchor(SwingKind::None, 0.0, 0.0, 1.0);
        assert_eq!(rest_stab.0, rest_none.0, "the rest pose the identity keeps");
        // the peak: the origin rides the thrust translate OVER the anchor
        // (applyItemArmTransform's (0.56, -0.52, -0.72) the chain composes
        // first — the jar's own order: the anchor walk, then
        // firstPersonAttack), the rotation never moves the origin
        let peak = item_hand_anchor(SwingKind::Stab, 0.05, 0.0, 1.0);
        let o = peak.transform_point(crate::math::Vec3::new(0.0, 0.0, 0.0));
        // the sums the f32 additions round (the literal 0.66 the x fold
        // keeps; the y/z the nearest-f32 folds the expressions speak)
        assert_eq!(o[0], 0.66, "the thrust's X over the anchor (0.56 + 0.1)");
        assert_eq!(o[1], -0.52f32 + -0.075f32, "the thrust's Y dip (-0.52 - 0.075)");
        assert_eq!(o[2], -0.72f32 + 0.65f32, "the thrust's Z reach (-0.72 + 0.65)");
        let up = peak.transform_point(crate::math::Vec3::new(0.0, 1.0, 0.0));
        assert!((up[0] - 0.66).abs() < 1e-4, "the X the rotation never touches");
        assert!(
            (up[1] - -0.252_979_85).abs() < 1e-3,
            "the up-axis tilts (cos -70° fold over the anchor, got {})",
            up[1]
        );
        assert!(
            (up[2] - -1.009_692_6).abs() < 1e-3,
            "the tip swings with the -70° X fold (0.65 + sin -70° - 0.72, got {})",
            up[2]
        );
        // the close: p=1 re-collides at zero — the identity the rest keeps
        let close = item_hand_anchor(SwingKind::Stab, 1.0, 0.0, 1.0);
        assert_eq!(close.0, rest_none.0, "the closed stab returns to rest");
        // the left mirror: the X term flips, the Y/Z terms stand
        let peak_left = item_hand_anchor(SwingKind::Stab, 0.05, 0.0, -1.0);
        let ol = peak_left.transform_point(crate::math::Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(ol[0], -0.66, "the mirrored thrust");
        assert_eq!(ol[1], -0.52f32 + -0.075f32, "the Y the mirror never touches");
        assert_eq!(ol[2], -0.72f32 + 0.65f32, "the Z the mirror never touches");
    }

    /// THE USE EASES' OWN FOLDS (javap `Ease.outCubic/outCirc/inCirc/
    /// inOutBack/outQuart/inOutElastic` — the 26.2 dumps): the plain
    /// f32 bodies the CP constants pin — outCubic the cube fold,
    /// outCirc/inCirc the f64 sqrt pair (the circ the 1-sqrt mirrors),
    /// outQuart the nested squares, inOutBack the seam-continuous 0.5
    /// the two branches share (the `+2` INSIDE the division the
    /// bytecode's own fadd/fdiv order keeps), inOutElastic the
    /// end-pinned phase fold (the 0.5 midpoint the sin(−π/2) collision
    /// answers, the 0.25/0.75 the pow branches speak).
    #[test]
    fn the_use_eases_the_folds_the_dump_speaks() {
        assert_eq!(ease_out_cubic(0.5), 0.875, "1 - cube(0.5)");
        assert_eq!(ease_out_cubic(1.0), 1.0, "the closed end");
        assert_eq!(ease_out_cubic(0.0), 0.0, "the open end");
        let circ = 0.866_025_4;
        assert!((ease_out_circ(0.5) - circ).abs() < 1e-6, "sqrt(1-0.25)");
        assert!((ease_in_circ(0.5) - (1.0 - circ)).abs() < 1e-6, "1 - the outCirc mirror");
        assert_eq!(ease_out_circ(1.0), 1.0, "the closed circle");
        assert_eq!(ease_in_circ(0.0), 0.0, "the circle's own start");
        assert_eq!(ease_in_circ(1.0), 1.0, "the circle's own close");
        assert_eq!(ease_out_quart(0.5), 0.9375, "1 - (0.25)^2");
        assert_eq!(ease_out_quart(1.0), 1.0, "the closed quart");
        // inOutBack: the ends pinned, the 0.5 seam the two arms share
        assert_eq!(ease_in_out_back(0.0), 0.0, "the back's open end");
        assert_eq!(ease_in_out_back(0.5), 0.5, "the seam both branches speak");
        assert_eq!(ease_in_out_back(1.0), 1.0, "the back's closed end");
        // the overshoot: the second branch peaks past 1 (the back's own law)
        let over = ease_in_out_back(0.8);
        assert!(over > 1.0, "the inOutBack overshoots past 1 (got {})", over);
        // inOutElastic: the ends pinned, the midpoint the phase collision
        assert_eq!(ease_in_out_elastic(0.0), 0.0, "the elastic's open end");
        assert_eq!(ease_in_out_elastic(1.0), 1.0, "the elastic's closed end");
        assert!((ease_in_out_elastic(0.5) - 0.5).abs() < 1e-6, "sin(-pi/2) the midpoint folds");
        assert!(
            (ease_in_out_elastic(0.25) - 0.011_969_440).abs() < 1e-6,
            "the first pow branch (got {})",
            ease_in_out_elastic(0.25)
        );
        assert!(
            (ease_in_out_elastic(0.75) - 0.988_030_55).abs() < 1e-6,
            "the second pow branch (got {})",
            ease_in_out_elastic(0.75)
        );
    }

    /// THE USE PARAMS' OWN WINDOWS (javap `UseParams.fromKineticWeapon`
    /// over the iron row — delay 10, the dismount/knockback/damage
    /// starts the `maxDurationTicks + delay` fold keeps): the raise
    /// ladder over [0, 10] the three sub-windows split (0.5/0.8), the
    /// sway the [40, 60] window ramps, the lower the elastic [105, 145]
    /// at `timeHeld - 20` folds, the back-rise the [230, 235] last five
    /// ticks, the pump `2·outCirc(sway) - 2·inCirc(back)` and the two
    /// sin folds the 19°/30° frequencies ride.
    #[test]
    fn the_use_params_the_windows_the_tiers_keep() {
        // THE IRON ROW's own starts: 50+10 / 135+10 / 225+10
        let (d, ds, ks, gs) = (10.0f32, 60.0f32, 145.0f32, 235.0f32);
        // THE PRESS FRAME (timeHeld = 0 - 1 + partial 1.0): everything zero
        let rest = spear_use_params(0.0, d, ds, ks, gs);
        assert_eq!(rest.raise, 0.0);
        assert_eq!(rest.raise_start, 0.0);
        assert_eq!(rest.raise_middle, 0.0);
        assert_eq!(rest.raise_end, 0.0);
        assert_eq!(rest.sway_progress, 0.0);
        assert_eq!(rest.lower, 0.0, "the elastic's clamped open end");
        assert_eq!(rest.raise_back, 0.0);
        assert_eq!(rest.sway_intensity, 0.0, "2·outCirc(0) - 2·inCirc(0)");
        assert_eq!(rest.sway_scale_slow, 0.0);
        assert_eq!(rest.sway_scale_fast, 0.0);
        // THE RAISE'S MIDPOINT (th = 5): raise 0.5 — the start window
        // closed, the middle/end not yet opened
        let mid = spear_use_params(5.0, d, ds, ks, gs);
        assert_eq!(mid.raise, 0.5);
        assert_eq!(mid.raise_start, 1.0, "the [0, 0.5] window closed");
        assert_eq!(mid.raise_middle, 0.0, "the [0.5, 0.8] window not yet");
        assert_eq!(mid.raise_end, 0.0, "the [0.8, 1] window not yet");
        // THE RAISE'S CLOSE (th = 10): the full ladder at 1
        let full = spear_use_params(10.0, d, ds, ks, gs);
        assert_eq!(full.raise, 1.0);
        assert_eq!(full.raise_start, 1.0);
        assert_eq!(full.raise_middle, 1.0);
        assert_eq!(full.raise_end, 1.0);
        // THE SWAY'S OWN WINDOW (th = 60 — the dismount start): the
        // sway ramp closed, the back not yet — the pump 2·outCirc(1) = 2
        let sway = spear_use_params(60.0, d, ds, ks, gs);
        assert_eq!(sway.sway_progress, 1.0, "the [40, 60] ramp closed");
        assert_eq!(sway.raise_back, 0.0, "the [230, 235] window far");
        assert_eq!(sway.sway_intensity, 2.0, "2·outCirc(1) - 2·inCirc(0)");
        assert!(
            (sway.sway_scale_slow - 1.732_050_7).abs() < 1e-6,
            "sin(60·19°)·2 (got {})",
            sway.sway_scale_slow
        );
        assert!(
            sway.sway_scale_fast.abs() < 1e-5,
            "sin(60·30°) the full-circle phase (got {})",
            sway.sway_scale_fast
        );
        // THE SWAY'S MIDPOINT (th = 50): progress 0.5 — the pump
        // 2·outCirc(0.5), the sines the 950/1500-degree phases speak
        let sway_mid = spear_use_params(50.0, d, ds, ks, gs);
        assert_eq!(sway_mid.sway_progress, 0.5);
        assert!(
            (sway_mid.sway_intensity - 1.732_050_8).abs() < 1e-6,
            "2·outCirc(0.5) (got {})",
            sway_mid.sway_intensity
        );
        assert!(
            (sway_mid.sway_scale_slow - -1.326_827_8).abs() < 1e-5,
            "sin(50·19°)·pump (got {})",
            sway_mid.sway_scale_slow
        );
        assert!(
            (sway_mid.sway_scale_fast - 1.499_999_8).abs() < 1e-5,
            "sin(50·30°)·pump (got {})",
            sway_mid.sway_scale_fast
        );
        // THE LOWER'S OWN ELASTIC (th = 135/145/165 — the knock window
        // [105, 145] at th-20): the clamped open, the phase midpoint,
        // the closed end
        assert!(
            (spear_use_params(135.0, d, ds, ks, gs).lower - 0.035_480_26).abs() < 1e-6,
            "outCubic(elastic(0.25)) (got {})",
            spear_use_params(135.0, d, ds, ks, gs).lower
        );
        assert!(
            (spear_use_params(145.0, d, ds, ks, gs).lower - 0.875).abs() < 1e-6,
            "the elastic midpoint the outCubic closes"
        );
        assert_eq!(
            spear_use_params(165.0, d, ds, ks, gs).lower, 1.0,
            "the elastic's closed end"
        );
        // THE BACK-RISE (th = 232.5 — the [230, 235] window's half)
        assert_eq!(
            spear_use_params(232.5, d, ds, ks, gs).raise_back, 0.5,
            "the last five ticks ramp"
        );
    }

    /// THE USE MATRIX'S OWN POSE (javap `firstPersonUse` pcs 41-262):
    /// the press frame the identity the raise's zero terms keep — the
    /// pose IS the bare anchor until the first tick lands — and the
    /// raise's close (th = 10, iron) the peak the two pivots speak:
    /// the origin rides `T(side·0.1, 0, 0)` over the rotAround folds
    /// (the XP −65° about (0, 0.1, 0), the YN −90·side about
    /// (side·0.15, 0, 0) — the −90·side the YN axis the +Y-by-+90·side
    /// fold answers), the left mirror the X-only flip keeps.
    #[test]
    fn the_use_matrix_the_rest_identity_and_the_peak_speak() {
        let (d, ds, ks, gs) = (10.0f32, 60.0f32, 145.0f32, 235.0f32);
        // THE PRESS FRAME: the identity — every term the zero raise
        // keeps (the pose the bare anchor never moves until the tick)
        let rest = spear_use_params(0.0, d, ds, ks, gs);
        let m = spear_use_matrix(&rest, 0.0, 1.0);
        assert_eq!(m.0, Mat4::translation(0.0, 0.0, 0.0).0, "the rest IS the identity");
        // THE RAISE'S CLOSE: the peak the origin map speaks — the f64
        // walk: YN fold first (rot_y(+90°) about (0.15, 0, 0) maps the
        // origin to (0.15, 0, 0.15)), then the XP tilt (rot_x(-65°)
        // about (0, 0.1, 0) to (0.15, 0.19368434, 0.15402352)), then
        // the translate (side·0.1, 0, 0) — (0.25, 0.19368434, 0.15402352)
        let peak = spear_use_params(10.0, d, ds, ks, gs);
        let mp = spear_use_matrix(&peak, 0.0, 1.0);
        let o = mp.transform_point(crate::math::Vec3::new(0.0, 0.0, 0.0));
        assert!((o[0] - 0.25).abs() < 1e-4, "the X over both pivots (got {})", o[0]);
        assert!(
            (o[1] - 0.193_684_34).abs() < 1e-4,
            "the XP tilt lifts (got {})",
            o[1]
        );
        assert!(
            (o[2] - 0.154_023_52).abs() < 1e-4,
            "the YN turn reaches (got {})",
            o[2]
        );
        // the hit feedback rides LAST in the call order — the
        // innermost transform the point meets first: the dipped origin
        // (0, -0.375, 0) the two pivots then swing — the y the tilts
        // lift to 0.0352, the z the YN turn reaches 0.4939 (the f64
        // walk: R_y(90°) of (-0.15, -0.375, 0) → (0.15, -0.375, 0.15);
        // R_x(-65°) → (0.15, -0.0648, 0.4939); + the pivots, + the
        // translate)
        let mf = spear_use_matrix(&peak, 0.375, 1.0);
        let of = mf.transform_point(crate::math::Vec3::new(0.0, 0.0, 0.0));
        assert!((of[0] - 0.25).abs() < 1e-4, "the X the dips never move");
        assert!(
            (of[1] - 0.035_202_49).abs() < 1e-4,
            "the dipped origin the tilt lifts (got {})",
            of[1]
        );
        assert!(
            (of[2] - 0.493_888_94).abs() < 1e-4,
            "the dipped origin the turn reaches (got {})",
            of[2]
        );
        // the left mirror: the X flips, the Y/Z stand
        let ml = spear_use_matrix(&peak, 0.0, -1.0);
        let ol = ml.transform_point(crate::math::Vec3::new(0.0, 0.0, 0.0));
        assert!((ol[0] + 0.25).abs() < 1e-4, "the mirrored X");
        assert!((ol[1] - 0.193_684_34).abs() < 1e-4, "the Y the mirror never touches");
        assert!((ol[2] - 0.154_023_52).abs() < 1e-4, "the Z the mirror never touches");
    }

    /// THE HIT FEEDBACK'S OWN JOLT (javap `SpearAnimations
    /// .hitFeedbackAmount` pcs 0-25): `0.4·(outQuart(progress(t,1,3)) -
    /// inOutSine(progress(t,3,10)))` — the fsub the operand order pins
    /// (the outQuart the minuend keeps), the rise the first three
    /// ticks speak, the settle the tenth closes, the steady state the
    /// outQuart(1) = inOutSine(1) collision answers.
    #[test]
    fn the_hit_feedback_the_jolt_the_settle_speaks() {
        assert_eq!(spear_hit_feedback_amount(0.0), 0.0, "the pre-window silence");
        assert_eq!(spear_hit_feedback_amount(2.0), 0.375, "0.4·outQuart(0.5)");
        assert_eq!(spear_hit_feedback_amount(1.0), 0.0, "the rise's own start");
        assert!(
            (spear_hit_feedback_amount(5.0) - 0.324_697_97).abs() < 1e-6,
            "the settle's midpoint (got {})",
            spear_hit_feedback_amount(5.0)
        );
        assert_eq!(spear_hit_feedback_amount(10.0), 0.0, "the settle's close");
        assert_eq!(spear_hit_feedback_amount(100.0), 0.0, "the steady state");
    }

    /// THE SWING CLOCK'S OWN DURATION (javap `getCurrentSwingDuration`
    /// over the component + `updateSwingTime`'s live local 1): the iron
    /// spear's (STAB, 19) component the clock wears — the trigger's
    /// -1 lands, 19 ticks advance, the reset at the duration; the
    /// attackAnim the swingTime/19 fold speaks; the re-trigger gate the
    /// LIVE duration's own half keeps (blocked below 9, allowed at 9 —
    /// the fist's 6-tick gate the DEFAULT component answers).
    #[test]
    fn the_swing_clock_the_component_duration_wears() {
        let mut s = SwingState::default();
        s.duration = 19; // the iron spear's component the App refreshes
        s.start();
        assert_eq!(s.swing_time, -1, "the trigger arms at -1");
        s.tick();
        assert_eq!(s.swing_time, 0, "the first tick lands on 0");
        assert!((s.attack_anim - 0.0).abs() < 1e-6);
        for expect in 1..=18 {
            s.tick();
            assert_eq!(s.swing_time, expect);
            let want = expect as f32 / 19.0;
            assert!(
                (s.attack_anim - want).abs() < 1e-6,
                "attackAnim the /19 fold speaks at {expect}"
            );
        }
        assert!(s.swinging, "18 < 19 the swing keeps");
        s.tick();
        assert_eq!(s.swing_time, 0, "the reset at the duration");
        assert!(!s.swinging);
        // the re-trigger gate: blocked inside the first half (swing_time
        // 0..8 after the re-arm), allowed from the duration's own 9
        s.start();
        s.tick(); // lands 0
        assert!(s.swinging);
        // a fresh trigger while the swing sits at 5 (of 19): blocked
        s.swing_time = 5;
        s.start();
        assert_eq!(s.swing_time, 5, "the half-gate the re-trigger refuses");
        s.swing_time = 9;
        s.start();
        assert_eq!(s.swing_time, -1, "the half-gate at 9 re-arms");
        // the fist's own clock: the DEFAULT (WHACK, 6) the empty hand keeps
        let mut fist = SwingState::default();
        assert_eq!(fist.duration, 6);
        fist.start();
        fist.tick();
        for _ in 0..5 {
            fist.tick();
        }
        assert_eq!(fist.swing_time, 5);
        assert!(fist.swinging);
        fist.tick();
        assert!(!fist.swinging, "the fist resets at 6");
        // the gate's own half: 3 for the fist (blocked at 2, allowed at 3)
        fist.start();
        fist.tick();
        fist.swing_time = 2;
        fist.start();
        assert_eq!(fist.swing_time, 2, "the fist's half-gate refuses at 2");
        fist.swing_time = 3;
        fist.start();
        assert_eq!(fist.swing_time, -1, "the fist's half-gate at 3 re-arms");
    }
}

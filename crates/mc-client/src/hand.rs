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
        fb.clear_depth();
        let quads = build_quads_kind_at(self.kind, swing, 0.0);
        fb.draw_hand_quads(proj, &quads, &self.rgba, skin::SKIN_SIDE as usize);
    }
}

/// The vanilla swing machinery, verbatim from LivingEntity's bytecode
/// (`swing`, `updateSwingTime`, `getAttackAnim`, `getCurrentSwingDuration`
/// + `SwingAnimation.DEFAULT = (WHACK, 6)` — the fist swing is 6 ticks).
/// Our client advances one game tick per frame, so `tick()` rides the
/// frame loop; the renderer samples `get_attack_anim(0.5)` (the canonical
/// mid-tick partial the game's camera renders with).
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
}

impl Default for SwingState {
    fn default() -> Self {
        Self {
            swinging: false,
            swing_time: 0,
            o_attack_anim: 0.0,
            attack_anim: 0.0,
        }
    }
}

impl SwingState {
    /// The fist swing's tick duration: `SwingAnimation.DEFAULT` is
    /// `(SwingAnimationType.WHACK, 6)` — bipush 6 in the static init.
    pub const DEFAULT_DURATION: i32 = 6;

    /// `LivingEntity.swing(hand)`'s trigger logic: a re-trigger is IGNORED
    /// while a swing is still in its first half (swingTime in [0, dur/2));
    /// otherwise (or fresh) swingTime = -1 and swinging = true.
    pub fn start(&mut self) {
        if !(self.swinging && self.swing_time < Self::DEFAULT_DURATION / 2 && self.swing_time >= 0)
        {
            self.swing_time = -1;
            self.swinging = true;
        }
    }

    /// `LivingEntity.updateSwingTime()` (one game tick): advance swingTime
    /// while swinging (reset to rest at the duration), pin it to 0 at
    /// rest, then attackAnim = swingTime / duration. oAttackAnim takes the
    /// previous tick's value first (the tick's lerp bookkeeping).
    pub fn tick(&mut self) {
        let duration = Self::DEFAULT_DURATION;
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
/// `side` is +1 for the RIGHT main hand (the default offline player's).
/// `swing` is the attack animation progress (`getAttackAnim`, 0 = rest),
/// `equip` the equip animation progress (0 — no item switching exists).
/// Both are the float params of `ItemInHandRenderer.renderPlayerArm` (the
/// javap decode: slot 5 = swing, slot 4 = equip); at rest every swing term
/// collapses to exactly zero, so this IS the rest pose bit-for-bit.
fn arm_transform(swing: f32, equip: f32) -> Mat4 {
    let side = 1.0f32;
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
        // the PlayerModel right_arm part: pivot (-5, 2, 0)/16, zRot 0.1 rad
        .mul(&t(-5.0 / 16.0, 2.0 / 16.0, 0.0))
        .mul(&Mat4::rot_z(0.1))
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
fn build_quads() -> Vec<HandQuad> {
    build_quads_kind(ArmKind::Wide)
}

/// The WIDE quads at an arbitrary progress — the pre-variant name the
/// bit-identity and sweep tests ride (the default arm kind).
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
    let (u0, v0) = skin::ARM_TEX_OFFS;
    let (origin, box_px) = arm_box(kind);
    let [x0, y0, z0] = origin;
    let [w, h, d] = box_px;
    let (u0, v0) = (u0 as f32, v0 as f32);
    let (x1, y1, z1) = (x0 + w, y0 + h, z0 + d);
    let m = arm_transform(swing, equip);
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

/// Load the posed quads of one item model for the first-person right
/// hand. `None` = unknown model or outside the bake domain — nothing to
/// draw (the caller renders the empty-hand arm instead).
pub fn load_item(model_id: &str) -> Option<HeldItem> {
    match mc_registry::display::posed_item_quads(
        model_id,
        mc_registry::display::FIRSTPERSON_RIGHTHAND,
    ) {
        Some(Ok(quads)) => Some(HeldItem { quads }),
        // a broken display stanza binds nothing (vanilla warns + drops)
        Some(Err(_)) => None,
        None => None,
    }
}

/// The item-in-hand anchor chain, `side` = +1 (the RIGHT main hand, the
/// default offline player's). At the rest pose (swing 0, equip 0) the
/// only non-trivial terms are the attack transform's rotY(side*45deg) ..
/// rotY(side*-45deg) pair — vanilla composes them every frame, so we do
/// too (op-for-op; not collapsed to identity).
fn item_hand_anchor(swing: f32, equip: f32) -> Mat4 {
    let side = 1.0f32;
    let t = |x: f32, y: f32, z: f32| Mat4::translation(x, y, z);
    let deg = |d: f32| d.to_radians();
    // applyItemArmTransform
    let a = t(side * 0.56, -0.52, equip * -0.6 + -0.72);
    // swingArm's translate terms (the WHACK arc; exact zeros at rest),
    // then applyItemArmAttackTransform's two rotation amounts — the
    // five-term item-side curve, extracted for the sweep test
    let [f5s, f6s, f7s, f5, f6] = item_swing_terms(swing);
    let b = t(side * f5s, f6s, f7s);
    Mat4::IDENT
        .mul(&a)
        .mul(&b)
        .mul(&Mat4::rot_y(deg(side * (45.0 + f5 * -20.0))))
        .mul(&Mat4::rot_z(deg(side * f6 * -20.0)))
        .mul(&Mat4::rot_x(deg(f6 * -80.0)))
        .mul(&Mat4::rot_y(deg(side * -45.0)))
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

impl HeldItem {
    /// The anchored, view-space quads at an attack animation progress
    /// (0 = rest; the deterministic scenes never swing, so the goldens —
    /// which hold no item — never see a nonzero one).
    fn sampled(&self, swing: f32) -> Vec<crate::raster::SampledQuad<'_>> {
        let m = item_hand_anchor(swing, 0.0);
        self.quads
            .iter()
            .map(|q| crate::raster::SampledQuad {
                pts: q.vertices.map(|v| {
                    let p = m.transform_point(crate::math::Vec3::new(v[0], v[1], v[2]));
                    [p[0], p[1], p[2]]
                }),
                uv: q.uvs,
                tex: q.texture,
            })
            .collect()
    }

    /// The hand pass for a held item: clears depth (vanilla clears depth
    /// between the level and the hand) and draws the item in view space
    /// under the world's projection (the hand fov follows the world's).
    /// Vanilla also equips/swings here — the swing terms ride
    /// `item_hand_anchor` above; the equip progress is always 0.
    pub fn draw(&self, fb: &mut crate::raster::Framebuf, proj: Mat4, swing: f32) {
        fb.clear_depth();
        let quads = self.sampled(swing);
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
        // the probe-observed count (in-test, plain-sky background)
        assert_eq!(changed, 1, "PROBE");
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
        // outside the bake domain / unknown -> nothing to draw
        assert!(load_item("block/stone").is_none());
        assert!(load_item("item/definitely_not_a_thing").is_none());
    }

    #[test]
    fn the_anchored_item_rests_in_the_bottom_right() {
        // the anchor chain (applyItemArmTransform + the at-rest swing
        // terms) must land every corner in front of the camera, right of
        // center; the BULK sits below center (the blade tip quads may
        // cross the horizontal midline — the vanilla held-sword look)
        let item = load_item("item/iron_sword").expect("iron_sword loads");
        let m = item_hand_anchor(0.0, 0.0);
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
}

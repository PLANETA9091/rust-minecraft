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
use mc_registry::generated::skin;
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
}

/// Load + sha1-verify the skin from the runtime asset root, then build
/// the vanilla arm geometry. `None` = not drawn.
pub fn load(root: &Path) -> Option<ArmModel> {
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
    Some(ArmModel {
        rgba,
        quads: build_quads(),
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
        let quads = build_quads_at(swing, 0.0);
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
    // the swing terms, exactly as the renderPlayerArm bytecode computes them
    // (Mth.sin/Mth.sqrt are f64 sin/sqrt narrowed to f32 — Mth's own bodies)
    let sq = mth_sqrt(swing);
    let f10 = -0.3f32 * mth_sin(sq * std::f32::consts::PI); // X wobble
    let f11 = 0.4f32 * mth_sin(sq * 6.2831855); // Y bob (6.2831855 = the bytecode's 2π f32)
    let f12 = -0.4f32 * mth_sin(swing * std::f32::consts::PI); // Z pull
    let f13 = mth_sin(swing * swing * std::f32::consts::PI); // rotZ amount
    let f14 = mth_sin(sq * std::f32::consts::PI); // extra rotY amount
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
    build_quads_at(0.0, 0.0)
}

/// The six hand quads at an arbitrary attack/equip animation progress —
/// the same `ModelPart$Cube` face table, transformed by the full
/// renderPlayerArm chain (the swing terms above). At (0, 0) this is the
/// rest pose, bit-for-bit (every added rotation is the exact identity).
fn build_quads_at(swing: f32, equip: f32) -> Vec<HandQuad> {
    let (u0, v0) = skin::ARM_TEX_OFFS;
    let [w, h, d] = skin::ARM_BOX_PX;
    let (u0, v0, w, h, d) = (u0 as f32, v0 as f32, w as f32, h as f32, d as f32);
    // model-space box bounds (addBox), /16 at transform time
    let (x0, x1) = (-3.0f32, -3.0 + w);
    let (y0, y1) = (-2.0f32, -2.0 + h);
    let (z0, z1) = (-2.0f32, -2.0 + d);
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
        let dir = std::path::Path::new("assets-cache/jar");
        if !dir.join(skin::RUNTIME_PATH).exists() {
            return; // hermetic environments skip: no asset, no assertion
        }
        let arm = load(dir).expect("the real jar skin verifies");
        assert_eq!(arm.quads.len(), 6);
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
}

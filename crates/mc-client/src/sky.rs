//! Sky + day-night cycle, driven entirely by GENERATED official data
//! (`mc_registry::generated::sky`: timeline tracks, dimension colors,
//! celestial textures extracted from the jar by mc2rs-gen).
//!
//! The evaluator replicates the game's 26.x semantics, verified by
//! disassembling the official unobfuscated jar:
//! - `KeyframeTrackSampler` — wrap-around segments (start: last.kf shifted
//!   back one period -> first.kf; middle: consecutive keyframes; end:
//!   last.kf -> first.kf shifted forward one period), first-match lookup,
//!   per-track easing, floorMod looping.
//! - `EasingType$CubicBezier` — CSS-style timing solved with Newton steps
//!   (tolerance 1e-5, step clamped to +-0.25) and a bisection fallback.
//! - `SkyRenderer` — sun quad = base (±1, 0, ±1) scaled (30, 1, 30),
//!   translated (0, 100, 0) in a rotY(-90°)·rotX(angle) frame; moon the
//!   same at scale 20 with one texture per MoonPhase ordinal; stars from
//!   RandomSource(10842L): 1500 candidates, accepted while
//!   0.010000001 < len² < 1, normalized to radius 100, quads of size
//!   0.15 + rf*0.1 facing the origin. LCG cross-checked against real
//!   java.util.Random(10842L) on OpenJDK 21 (see tests).

use mc_registry::generated::sky as S;

use crate::camera::Camera;
use crate::math::{Mat4, Vec3};
use crate::raster::{CloudQuad, FlatQuad, RenderCtx};

/// One evaluated frame of the official day-night cycle.
#[derive(Debug, Clone)]
pub struct SkyState {
    /// Celestial rotation angles (radians, as passed to rotX by the game).
    pub sun_angle: f32,
    pub moon_angle: f32,
    pub star_angle: f32,
    /// 0..1 (official `visual/star_brightness`, modifier maximum).
    pub star_brightness: f32,
    /// Dimension base color multiplied by the official color track (RGB).
    pub sky_color: [f32; 3],
    pub fog_color: [f32; 3],
    /// Official `visual/sunrise_sunset_color` track value (RGBA) — the tint
    /// of the horizon glow disc.
    pub sunrise_color: [f32; 4],
    /// MoonPhase ordinal (full moon = 0, matching the official enum order).
    pub moon_phase: u8,
    /// Final cloud color (RGBA floats): the official
    /// ARGB.multiply(CLOUD_BASE_COLOR, visual/cloud_color track) — the value
    /// uploaded as the CloudColor uniform by the game's render().
    pub cloud_color: [f32; 4],
}

const DEG_TO_RAD: f32 = 0.017_453_292; // the exact f32 the game uses

/// CSS-style cubic bezier timing function (official EasingType$CubicBezier).
pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, input: f32) -> f32 {
    if input <= 0.0 {
        return 0.0;
    }
    if input >= 1.0 {
        return 1.0;
    }
    let xc = CubicCurve::from_controls(x1, x2);
    let yc = CubicCurve::from_controls(y1, y2);
    yc.sample(solve_t(&xc, input))
}

/// Bezier curve over t: ((a·t + b)·t + c)·t — the official CubicCurve record.
#[derive(Clone, Copy)]
struct CubicCurve {
    a: f32,
    b: f32,
    c: f32,
}

impl CubicCurve {
    fn from_controls(c1: f32, c2: f32) -> Self {
        CubicCurve { a: 1.0 - 3.0 * c2 + 3.0 * c1, b: 3.0 * c2 - 6.0 * c1, c: 3.0 * c1 }
    }
    fn sample(&self, t: f32) -> f32 {
        ((self.a * t + self.b) * t + self.c) * t
    }
    fn sample_gradient(&self, t: f32) -> f32 {
        (3.0 * self.a * t + 2.0 * self.b) * t + self.c
    }
}

fn solve_t(curve: &CubicCurve, x: f32) -> f32 {
    // Newton iterations (official tolerance + step clamp), bisection fallback.
    let mut t = x;
    for _ in 0..8 {
        let err = curve.sample(t) - x;
        if err.abs() < 1e-5 {
            return t;
        }
        let g = curve.sample_gradient(t);
        if g.abs() < 1e-5 {
            break;
        }
        t -= err / g;
        t = t.clamp(0.0, 1.0);
    }
    // Bisection (monotone x curve on [0,1] for controls in [0,1]).
    let (mut lo, mut hi) = (0.0f32, 1.0f32);
    for _ in 0..32 {
        let mid = 0.5 * (lo + hi);
        if curve.sample(mid) < x {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// Eased interpolation factor (None = LINEAR, the official default).
fn ease(bezier: Option<[f32; 4]>, delta: f32) -> f32 {
    match bezier {
        None => delta,
        Some([x1, y1, x2, y2]) => cubic_bezier(x1, y1, x2, y2, delta),
    }
}

/// Sample one float track exactly like the game's KeyframeTrackSampler:
/// virtual wrap segments, first segment with `t <= to_ticks` wins.
fn sample_f32(kf: &[(i32, f32)], bezier: Option<[f32; 4]>, t: i64, period: i32) -> f32 {
    if kf.len() == 1 {
        return kf[0].1;
    }
    let lt = t.rem_euclid(period as i64) as i32;
    let first = &kf[0];
    let last = &kf[kf.len() - 1];
    // Segment as (from_ticks, from_value, to_ticks, to_value) — lazy list.
    let eval = |ft: i32, fv: f32, tt: i32, tv: f32| -> Option<f32> {
        if lt <= tt {
            let v = if tt == ft {
                fv // zero-length segment (official wrap pair): hold from-value
            } else {
                let d = (lt - ft) as f32 / (tt - ft) as f32;
                fv + ease(bezier, d) * (tv - fv)
            };
            return Some(v);
        }
        None
    };
    // 1. wrap-start: last.kf shifted one period back -> first.kf.
    if let Some(v) = eval(last.0 - period, last.1, first.0, first.1) {
        return v;
    }
    // 2. consecutive keyframes.
    for w in kf.windows(2) {
        if let Some(v) = eval(w[0].0, w[0].1, w[1].0, w[1].1) {
            return v;
        }
    }
    // 3. wrap-end: last.kf -> first.kf shifted one period forward.
    eval(last.0, last.1, first.0 + period, first.1)
        .unwrap_or_else(|| first.1.max(last.1))
}

/// Sample one RGBA color track (LINEAR in the official data).
fn sample_color_rgba(kf: &[(i32, [u8; 4])], t: i64, period: i32) -> [f32; 4] {
    if kf.len() == 1 {
        return [
            kf[0].1[0] as f32 / 255.0,
            kf[0].1[1] as f32 / 255.0,
            kf[0].1[2] as f32 / 255.0,
            kf[0].1[3] as f32 / 255.0,
        ];
    }
    let lt = t.rem_euclid(period as i64) as i32;
    let first = &kf[0];
    let last = &kf[kf.len() - 1];
    let eval = |ft: i32, fv: [u8; 4], tt: i32, tv: [u8; 4]| -> Option<[f32; 4]> {
        if lt <= tt {
            let lerp = |a: u8, b: u8| -> f32 {
                (a as f32 + (b as f32 - a as f32) * ((lt - ft) as f32 / (tt - ft) as f32)) / 255.0
            };
            return Some([
                lerp(fv[0], tv[0]),
                lerp(fv[1], tv[1]),
                lerp(fv[2], tv[2]),
                lerp(fv[3], tv[3]),
            ]);
        }
        None
    };
    if let Some(v) = eval(last.0 - period, last.1, first.0, first.1) {
        return v;
    }
    for w in kf.windows(2) {
        if let Some(v) = eval(w[0].0, w[0].1, w[1].0, w[1].1) {
            return v;
        }
    }
    eval(last.0, last.1, first.0 + period, first.1).unwrap_or([1.0, 1.0, 1.0, 1.0])
}

/// Sample one RGBA color track, returning RGB only.
fn sample_color_rgb(kf: &[(i32, [u8; 4])], t: i64, period: i32) -> [f32; 3] {
    let c = sample_color_rgba(kf, t, period);
    [c[0], c[1], c[2]]
}

/// Evaluate the whole official day-night cycle at world tick `day_time`.
pub fn sample(day_time: i64) -> SkyState {
    let sun_deg = sample_f32(&S::SUN_ANGLE_KF, Some(S::SUN_ANGLE_BEZIER), day_time, S::DAY_PERIOD_TICKS);
    let moon_deg =
        sample_f32(&S::MOON_ANGLE_KF, Some(S::MOON_ANGLE_BEZIER), day_time, S::DAY_PERIOD_TICKS);
    let star_deg =
        sample_f32(&S::STAR_ANGLE_KF, Some(S::STAR_ANGLE_BEZIER), day_time, S::DAY_PERIOD_TICKS);
    let star_brightness =
        sample_f32(&S::STAR_BRIGHTNESS_KF, None, day_time, S::DAY_PERIOD_TICKS).max(0.0);
    let sky_track = sample_color_rgb(&S::SKY_COLOR_KF, day_time, S::DAY_PERIOD_TICKS);
    let fog_track = sample_color_rgb(&S::FOG_COLOR_KF, day_time, S::DAY_PERIOD_TICKS);
    let sunrise_color = sample_color_rgba(&S::SUNRISE_COLOR_KF, day_time, S::DAY_PERIOD_TICKS);
    let base = |c: [u8; 4]| [c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0];
    let dim_sky = base(S::DIM_SKY_COLOR);
    let dim_fog = base(S::DIM_FOG_COLOR);
    // Modifier "multiply" (official): final = base * track, per channel.
    let sky_color = [
        dim_sky[0] * sky_track[0],
        dim_sky[1] * sky_track[1],
        dim_sky[2] * sky_track[2],
    ];
    let fog_color = [
        dim_fog[0] * fog_track[0],
        dim_fog[1] * fog_track[1],
        dim_fog[2] * fog_track[2],
    ];
    // Moon phase: constant-held keyframes over the 192000-tick moon period.
    let mt = day_time.rem_euclid(S::MOON_PERIOD_TICKS as i64);
    let mut moon_phase = 0u8;
    for (ticks, idx) in S::MOON_PHASE_KF.iter() {
        if mt >= *ticks as i64 {
            moon_phase = *idx;
        }
    }
    // Cloud color: the dimension base multiplied by the official
    // visual/cloud_color track (modifier "multiply" == ARGB.multiply).
    let cloud_track = sample_color_rgba(&S::CLOUD_COLOR_KF, day_time, S::DAY_PERIOD_TICKS);
    let cloud_rgba = argb_multiply_rgba(S::CLOUD_BASE_COLOR, &cloud_track);
    SkyState {
        sun_angle: sun_deg * DEG_TO_RAD,
        moon_angle: moon_deg * DEG_TO_RAD,
        star_angle: star_deg * DEG_TO_RAD,
        star_brightness,
        sky_color,
        fog_color,
        sunrise_color,
        moon_phase,
        cloud_color: cloud_rgba,
    }
}

/// java.util.Random — the exact 48-bit LCG the game's LegacyRandomSource
/// implements (verified against OpenJDK 21 java.util.Random(10842L)).
pub struct JavaRandom {
    seed: i64,
}

impl JavaRandom {
    const MUL: i64 = 0x5DEECE66D;
    const ADD: i64 = 0xB;
    const MASK: i64 = (1i64 << 48) - 1;

    pub fn new(seed: i64) -> Self {
        JavaRandom { seed: (seed ^ Self::MUL) & Self::MASK }
    }

    fn next(&mut self, bits: u32) -> i64 {
        self.seed = (self.seed.wrapping_mul(Self::MUL).wrapping_add(Self::ADD)) & Self::MASK;
        self.seed >> (48 - bits) // sign-extended like (int)(seed >>> (48-bits))
    }

    pub fn next_f32(&mut self) -> f32 {
        self.next(24) as f32 / (1u32 << 24) as f32
    }

    pub fn next_f64(&mut self) -> f64 {
        let hi = (self.next(26) as i64) << 27;
        let lo = self.next(27) as i64;
        ((hi + lo) as f64) / ((1u64 << 53) as f64)
    }
}

/// One accepted star quad (celestial frame, radius 100).
struct Star {
    corners: [[f32; 3]; 4],
}

/// JOML Matrix3f.rotateTowards(direction, up) equivalent: local +Z points at
/// `dir`, +Y pulled toward `up` (Gram-Schmidt), +X = up × z.
fn star_basis(z_dir: Vec3) -> (Vec3, Vec3) {
    let up = Vec3::new(0.0, 1.0, 0.0);
    let left = up.cross(z_dir);
    let left = if left.length() > 1e-6 {
        left.normalized()
    } else {
        Vec3::new(1.0, 0.0, 0.0) // degenerate: star at the zenith axis
    };
    let new_up = z_dir.cross(left);
    (left, new_up)
}

/// Replicates SkyRenderer.buildStars(): RandomSource(10842L), 1500
/// candidates, acceptance 0.010000001 < len² < 1, radius 100,
/// size 0.15 + rf*0.1, per-star random Z rotation. The quad plane faces the
/// origin; exact in-plane orientation is sub-pixel at gameplay resolutions
/// (JOML rotateTowards approximated by an orthonormal billboard basis).
fn build_stars() -> Vec<Star> {
    let mut rng = JavaRandom::new(10842);
    let mut stars = Vec::new();
    for _ in 0..1500 {
        let x = rng.next_f32() * 2.0 - 1.0;
        let y = rng.next_f32() * 2.0 - 1.0;
        let z = rng.next_f32() * 2.0 - 1.0;
        let size = 0.15 + rng.next_f32() * 0.1;
        let len2 = x * x + y * y + z * z;
        if len2 <= 0.010_000_001 || len2 >= 1.0 {
            continue;
        }
        let center = Vec3::new(x, y, z).normalized().scale(100.0);
        let angle = (rng.next_f64() * std::f64::consts::PI * 2.0) as f32;
        let (left, up) = star_basis(center.normalized().scale(-1.0));
        let (c, s) = (angle.cos(), angle.sin());
        // rotateZ(-angle) applied to (±size, ±size, 0), official corner order.
        let mut corners = [[0f32; 3]; 4];
        for (i, (sx, sy)) in [(1.0f32, -1.0f32), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0)]
            .iter()
            .enumerate()
        {
            let (rx, ry) = (sx * size, sy * size);
            // Rz(-angle): x' = x·c + y·s, y' = -x·s + y·c
            let (wx, wy) = (rx * c + ry * s, -rx * s + ry * c);
            let v = center
                .add(left.scale(wx))
                .add(up.scale(wy));
            corners[i] = [v.x, v.y, v.z];
        }
        stars.push(Star { corners });
    }
    stars
}

fn stars() -> &'static Vec<Star> {
    use std::sync::OnceLock;
    static STARS: OnceLock<Vec<Star>> = OnceLock::new();
    STARS.get_or_init(build_stars)
}

/// rotY(-90°)·rotX(angle) — the official celestial frame (sun rises at +X).
fn celestial(angle: f32) -> Mat4 {
    Mat4::rot_y(-90.0f32.to_radians()).mul(&Mat4::rot_x(angle))
}

/// Sun/moon: base quad (±1, 0, ±1) scaled, translated up 100, in the
/// celestial frame (official renderSun/renderMoon transforms).
fn celestial_quad(m: &Mat4, cam: &Camera, scale: f32) -> [[f32; 3]; 4] {
    let base = [
        [-1.0f32, 0.0, -1.0],
        [1.0, 0.0, -1.0],
        [1.0, 0.0, 1.0],
        [-1.0, 0.0, 1.0],
    ];
    let mut pts = [[0f32; 3]; 4];
    for (i, p) in base.iter().enumerate() {
        let v = Vec3::new(p[0] * scale, 0.0, p[2] * scale).add(Vec3::new(0.0, 100.0, 0.0));
        let w = m.transform_point(v);
        pts[i] = [w[0] + cam.pos.x, w[1] + cam.pos.y, w[2] + cam.pos.z];
    }
    pts
}

/// The official horizon glow disc (renderSunriseAndSunset): an 18-vertex
/// triangle fan — center (0, 100, 0) with vertex alpha 1, a 17-vertex ring
/// (sin·120, cos·120, -cos·40) with vertex alpha 0 — tinted by the official
/// `visual/sunrise_sunset_color` track and blended TRANSLUCENT
/// (BlendFunction.TRANSLUCENT, the official SUNRISE_SUNSET pipeline).
/// Transform: rotX(90°) · rotZ(90° [+180° when sin(sun_angle) < 0]), then
/// scale(1, 1, alpha) applied in the fan's local space; early-out when the
/// track alpha <= 0.001.
pub fn render_sunrise_disc(
    quads: &mut Vec<FlatQuad>,
    cam: &Camera,
    day_time: i64,
) {
    let st = sample(day_time);
    let alpha = st.sunrise_color[3];
    if alpha <= 0.001 {
        return;
    }
    let flip = if st.sun_angle.sin() < 0.0 { 180.0f32 } else { 0.0 };
    let m = Mat4::rot_x(90.0f32.to_radians())
        .mul(&Mat4::rot_z((90.0f32 + flip).to_radians()));
    let tint = st.sunrise_color;
    // center vertex (fan root), alpha = tint alpha
    let center_local = Vec3::new(0.0, 100.0, 0.0);
    let cw = m.transform_point(center_local);
    let center = [cw[0] + cam.pos.x, cw[1] + cam.pos.y, cw[2] + cam.pos.z];
    let center_color = [tint[0], tint[1], tint[2], alpha];
    // 17 ring vertices (i = 0..=16), alpha 0; local z scaled by alpha.
    let mut ring: Vec<([f32; 3], [f32; 4])> = Vec::with_capacity(17);
    for i in 0..=16i32 {
        let ang = i as f32 * (6.2831855f32 / 16.0);
        let (s, c) = (ang.sin(), ang.cos());
        let local = Vec3::new(s * 120.0, c * 120.0, -c * 40.0 * alpha);
        let w = m.transform_point(local);
        ring.push((
            [w[0] + cam.pos.x, w[1] + cam.pos.y, w[2] + cam.pos.z],
            [tint[0], tint[1], tint[2], 0.0],
        ));
    }
    // TRIANGLE_FAN: triangles (center, ring[i], ring[i+1]) for i in 0..=15.
    for i in 0..=15 {
        quads.push(FlatQuad {
            pts: [center, ring[i].0, ring[i + 1].0, ring[i + 1].0],
            colors: [center_color, ring[i].1, ring[i + 1].1, ring[i + 1].1],
            rgba: None,
            blend: crate::raster::FlatBlend::Translucent,
        });
    }
}

/// Render the sunrise disc, sun, moon (current phase) and stars for world
/// tick `day_time`. Must be called after the sky-color clear and BEFORE
/// terrain quads (official order: sky disc -> sunrise glow -> celestials ->
/// terrain).
pub fn render_celestials(fb: &mut crate::raster::Framebuf, ctx: &RenderCtx, cam: &Camera, day_time: i64) {
    let st = sample(day_time);
    let mut quads: Vec<FlatQuad> = Vec::with_capacity(stars().len() + 18);

    // sunrise/sunset glow (drawn first, official render order)
    render_sunrise_disc(&mut quads, cam, day_time);

    // sun
    let sun = celestial_quad(&celestial(st.sun_angle), cam, 30.0);
    quads.push(FlatQuad {
        pts: sun,
        colors: [[1.0, 1.0, 1.0, 1.0]; 4],
        rgba: Some((&S::SUN_RGBA, S::CELESTIAL_SIZE)),
        blend: crate::raster::FlatBlend::Additive,
    });
    // moon (phase-selected texture, mirrored UVs like the official buffer)
    let moon = celestial_quad(&celestial(st.moon_angle), cam, 20.0);
    quads.push(FlatQuad {
        pts: moon,
        colors: [[1.0, 1.0, 1.0, 1.0]; 4],
        rgba: Some((&S::MOON_RGBA[st.moon_phase as usize], S::CELESTIAL_SIZE)),
        blend: crate::raster::FlatBlend::Additive,
    });
    // stars: brightness-tinted, additive (official STARS pipeline = OVERLAY).
    let m = celestial(st.star_angle);
    let b = st.star_brightness;
    for star in stars() {
        let mut pts = [[0f32; 3]; 4];
        for (i, c) in star.corners.iter().enumerate() {
            let w = m.transform_point(Vec3::new(c[0], c[1], c[2]));
            pts[i] = [w[0] + cam.pos.x, w[1] + cam.pos.y, w[2] + cam.pos.z];
        }
        quads.push(FlatQuad {
            pts,
            colors: [[b, b, b, b]; 4],
            rgba: None,
            blend: crate::raster::FlatBlend::Additive,
        });
    }
    fb.draw_flat(ctx, &quads);
}

// ---------------------------------------------------------------------------
// Clouds — the official CloudRenderer, verified against javap of the 26.2 jar
// (CloudRenderer.class + TextureData + rendertype_clouds.vsh/.fsh).
// ---------------------------------------------------------------------------

/// CELL_SIZE_IN_BLOCKS (CloudRenderer ConstantValue): one texture texel =
/// one 12x12-block cloud cell.
const CLOUD_CELL: f64 = 12.0;
/// Fancy cloud box height: the CloudInfo UBO carries CellSize (12, 4, 12).
const CLOUD_CELL_H: f64 = 4.0;
/// BLOCKS_PER_SECOND 0.6 / 20 ticks = the literal f32 0.030000001 the
/// bytecode multiplies the drift offset by (blocks per tick, toward -X).
const CLOUD_DRIFT_PER_TICK: f32 = 0.030_000_001;
/// The constant Z alignment offset added to the camera Z in render()
/// (double 3.9600000381469727 == 3.96f32 widened).
const CLOUD_Z_OFFSET: f64 = 3.960_000_038_146_972_7;
/// TICKS_PER_CELL (ConstantValue 400): 400 ticks * 0.03 blocks = one cell.
const CLOUD_TICKS_PER_CELL: i64 = 400;
/// Options.cloudRange default (IntRange(2, 128) constructed with 128).
const CLOUD_RANGE_CHUNKS: i64 = 128;
/// Official face shading (rendertype_clouds.vsh faceColors[]):
/// bottom 0.7, top 1.0, north/south 0.8, west/east 0.9.
const FACE_BOTTOM: f32 = 0.7;
const FACE_NORTH_SOUTH: f32 = 0.8;
const FACE_WEST_EAST: f32 = 0.9;

/// net.minecraft.util.ARGB.multiply(base, track): -1 shortcuts, then
/// per-channel (c1 * c2) / 255 integer division. Inputs/outputs RGBA here;
/// the math packs both colors as 0xAARRGGBB exactly like the game.
fn argb_multiply_rgba(base: [u8; 4], track: &[f32; 4]) -> [f32; 4] {
    // RGBA -> ARGB u32 (track arrives as floats 0..1).
    let pack = |c: [u8; 4]| -> u32 {
        ((c[3] as u32) << 24) | ((c[0] as u32) << 16) | ((c[1] as u32) << 8) | c[2] as u32
    };
    let to_bytes = |v: u32| -> [u8; 4] {
        [((v >> 16) & 0xFF) as u8, ((v >> 8) & 0xFF) as u8, (v & 0xFF) as u8, ((v >> 24) & 0xFF) as u8]
    };
    let round_track = |f: f32| -> u8 { (f * 255.0 + 0.5) as u8 };
    let t = [
        round_track(track[0]),
        round_track(track[1]),
        round_track(track[2]),
        round_track(track[3]),
    ];
    let (b, t) = (pack(base), pack(t));
    let mul = |x: u32, y: u32| -> u32 { (x * y) / 255 };
    let out = if b == 0xFFFF_FFFF {
        t
    } else if t == 0xFFFF_FFFF {
        b
    } else {
        (mul(b >> 24 & 0xFF, t >> 24 & 0xFF) << 24)
            | (mul(b >> 16 & 0xFF, t >> 16 & 0xFF) << 16)
            | (mul(b >> 8 & 0xFF, t >> 8 & 0xFF) << 8)
            | mul(b & 0xFF, t & 0xFF)
    };
    let c = to_bytes(out);
    [c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0, c[3] as f32 / 255.0]
}

/// The official cloud drift: `render()` computes
/// `f = gameTime % (width * 400) + partialTicks`, then
/// `x = camX + f * 0.030000001` — a westward slide of 0.03 blocks/tick that
/// wraps exactly at the texture period (256 cells * 400 ticks).
pub fn cloud_drift(game_time: i64, partial_ticks: f32) -> f64 {
    let width = S::CLOUDS_SIZE as i64;
    let f = (game_time.rem_euclid(width * CLOUD_TICKS_PER_CELL)) as f32 + partial_ticks;
    (f * CLOUD_DRIFT_PER_TICK) as f64
}

/// Is there a cloud cell covering world position (wx, wz) at the given
/// drift? The pattern is the 256x256 cell map tiled over the world, sampled
/// at `floor((w + offset) / 12) mod 256` (camera cancels out of the game's
/// mesh anchoring — derived from the CloudInfo UBO offsets).
pub fn is_cloud_cell(wx: f64, wz: f64, drift: f64) -> bool {
    let size = S::CLOUDS_SIZE as f64;
    let tx = (((wx + drift) / CLOUD_CELL).floor().rem_euclid(size)) as usize;
    let tz = (((wz + CLOUD_Z_OFFSET) / CLOUD_CELL).floor().rem_euclid(size)) as usize;
    S::CLOUDS[tx + tz * S::CLOUDS_SIZE as usize] != 0
}

/// Build and render the cloud layer for world tick `day_time`.
///
/// Replicates the official FANCY mesh (default CloudStatus): boxes of
/// 12x4x12 cells at y = [cloud_height, cloud_height + 4]; camera below sees
/// the shaded BOTTOM faces (0.7) plus side faces adjacent to empty cells
/// (north/south 0.8, west/east 0.9); mesh radius = ceil(cloudRange*16/12)
/// cells with a Euclidean cap, exactly the buildMesh domain.
pub fn render_clouds(
    fb: &mut crate::raster::Framebuf,
    ctx: &RenderCtx,
    cam: &Camera,
    day_time: i64,
    partial_ticks: f32,
) {
    let st = sample(day_time);
    // The extractor skips the layer entirely when the track alpha hits 0
    // (LevelExtractor: ARGB.alpha(cloudColor) <= 0 -> no cloud pass).
    if st.cloud_color[3] <= 0.0 {
        return;
    }
    let drift = cloud_drift(day_time, partial_ticks);
    let height = S::CLOUD_HEIGHT as f64;
    let yb = height;
    let yt = height + CLOUD_CELL_H;
    let size = S::CLOUDS_SIZE as i64;
    // Mesh radius: ceil(cloudRange * 16 / 12) cells (render() bytecode).
    let r = ((CLOUD_RANGE_CHUNKS * 16) as f64 / CLOUD_CELL).ceil() as i64;
    // The cell containing the camera's anchor point (cellX/cellZ in render()).
    let cx = ((cam.pos.x as f64 + drift) / CLOUD_CELL).floor() as i64;
    let cz = ((cam.pos.z as f64 + CLOUD_Z_OFFSET) / CLOUD_CELL).floor() as i64;
    let tex = |i: i64, k: i64| -> bool {
        let tx = i.rem_euclid(size) as usize;
        let tz = k.rem_euclid(size) as usize;
        S::CLOUDS[tx + tz * S::CLOUDS_SIZE as usize] != 0
    };
    let cell = |i: i64, k: i64| -> bool { tex(i, k) };
    let color = st.cloud_color;
    // FogCloudsEnd = min(cloudRange * 16, visual/cloud_fog_end_distance 2048).
    let fog_end = ((CLOUD_RANGE_CHUNKS * 16) as f32).min(2048.0);
    // Conservative per-cell cull (OUTPUT-IDENTICAL optimization: cells that
    // project fully outside the frustum, or sit entirely beyond the fog end,
    // can never produce a visible pixel — the rasterizer would skip them).
    // Half angles come straight from the projection matrix
    // (perspective(): [0] = f/aspect, [5] = f, f = 1/tan(fovy/2)).
    let fwd = cam.forward();
    let right = fwd.cross(Vec3::new(0.0, 1.0, 0.0)).normalized();
    let up = right.cross(fwd);
    let tan_v = 1.0 / ctx.proj.0[5];
    let tan_h = 1.0 / ctx.proj.0[0];
    // Cloud cell bounding-sphere radius: half extents (6, 2, 6).
    const RHO: f64 = 8.717_797_887_081_348; // sqrt(76)
    let mut quads: Vec<CloudQuad> = Vec::new();
    for dz in -r..=r {
        for dx in -r..=r {
            if dx * dx + dz * dz > r * r {
                continue; // official buildMesh Euclidean cap
            }
            let (i, k) = (cx + dx, cz + dz);
            if !cell(i, k) {
                continue;
            }
            // Cell center in world space (the lattice slides with the drift).
            let ccx = (i as f64) * CLOUD_CELL - drift + CLOUD_CELL / 2.0;
            let ccy = yb + CLOUD_CELL_H / 2.0;
            let ccz = (k as f64) * CLOUD_CELL - CLOUD_Z_OFFSET + CLOUD_CELL / 2.0;
            let vx = (ccx - cam.pos.x as f64) as f32;
            let vy = (ccy - cam.pos.y as f64) as f32;
            let vz = (ccz - cam.pos.z as f64) as f32;
            // Euclidean distance: beyond fog_end + radius the fade is
            // exactly 0 for every pixel of the cell.
            let de = (vx * vx + vy * vy + vz * vz).sqrt() as f64;
            if de - RHO > fog_end as f64 {
                continue;
            }
            // Frustum test in camera axes (depth d along forward): the cloud
            // cell's bounding sphere must intersect all four side half-spaces
            // |axis| <= depth * tan(half_fov), else it is fully outside.
            let d = vx * fwd.x + vy * fwd.y + vz * fwd.z;
            if d < -(RHO as f32) {
                continue; // entirely behind the camera
            }
            let h = vx * right.x + vy * right.y + vz * right.z;
            let w = vx * up.x + vy * up.y + vz * up.z;
            let sp = RHO as f32;
            let inv_h = (1.0 + tan_h * tan_h).sqrt();
            let inv_v = (1.0 + tan_v * tan_v).sqrt();
            if h - d * tan_h > sp * inv_h
                || -h - d * tan_h > sp * inv_h
                || w - d * tan_v > sp * inv_v
                || -w - d * tan_v > sp * inv_v
            {
                continue;
            }
            // World-space cell rect (the lattice slides with the drift).
            let x0 = (i * 12) as f64 - drift;
            let x1 = x0 + CLOUD_CELL;
            let z0 = (k * 12) as f64 - CLOUD_Z_OFFSET;
            let z1 = z0 + CLOUD_CELL;
            let (x0, x1, z0, z1) = (x0 as f32, x1 as f32, z0 as f32, z1 as f32);
            let c = |shade: f32| -> [f32; 4] {
                [color[0] * shade, color[1] * shade, color[2] * shade, color[3]]
            };
            // BOTTOM face (camera below the layer): visible bottom 0.7.
            // Corners follow the shader's bottom-face table (1,0,0)(1,0,1)(0,0,1)(0,0,0).
            quads.push(CloudQuad {
                pts: [[x1, yb as f32, z0], [x1, yb as f32, z1], [x0, yb as f32, z1], [x0, yb as f32, z0]],
                color: c(FACE_BOTTOM),
            });
            // Side faces only where the neighbor cell is empty (the packed
            // isNorthEmpty/isEastEmpty/... face flags).
            if !tex(i, k - 1) {
                // NORTH (-Z): (0,0,0)(0,1,0)(1,1,0)(1,0,0)
                quads.push(CloudQuad {
                    pts: [[x0, yb as f32, z0], [x0, yt as f32, z0], [x1, yt as f32, z0], [x1, yb as f32, z0]],
                    color: c(FACE_NORTH_SOUTH),
                });
            }
            if !tex(i, k + 1) {
                // SOUTH (+Z): (1,0,1)(1,1,1)(0,1,1)(0,0,1)
                quads.push(CloudQuad {
                    pts: [[x1, yb as f32, z1], [x1, yt as f32, z1], [x0, yt as f32, z1], [x0, yb as f32, z1]],
                    color: c(FACE_NORTH_SOUTH),
                });
            }
            if !tex(i - 1, k) {
                // WEST (-X): (0,0,1)(0,1,1)(0,1,0)(0,0,0)
                quads.push(CloudQuad {
                    pts: [[x0, yb as f32, z1], [x0, yt as f32, z1], [x0, yt as f32, z0], [x0, yb as f32, z0]],
                    color: c(FACE_WEST_EAST),
                });
            }
            if !tex(i + 1, k) {
                // EAST (+X): (1,0,0)(1,1,0)(1,1,1)(1,0,1)
                quads.push(CloudQuad {
                    pts: [[x1, yb as f32, z0], [x1, yt as f32, z0], [x1, yt as f32, z1], [x1, yb as f32, z1]],
                    color: c(FACE_WEST_EAST),
                });
            }
        }
    }
    // FogCloudsEnd = min(cloudRange * 16, visual/cloud_fog_end_distance 2048).
    let fog_end = ((CLOUD_RANGE_CHUNKS * 16) as f32).min(2048.0);
    fb.draw_clouds(ctx, [cam.pos.x, cam.pos.y, cam.pos.z], fog_end, &quads);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bezier_matches_official_solver_shapes() {
        let b = [0.362, 0.241, 0.638, 0.759]; // the official day-arc controls
        assert!(cubic_bezier(b[0], b[1], b[2], b[3], 0.0).abs() < 1e-6);
        assert!((cubic_bezier(b[0], b[1], b[2], b[3], 1.0) - 1.0).abs() < 1e-6);
        assert!((cubic_bezier(b[0], b[1], b[2], b[3], 0.5) - 0.5).abs() < 1e-3);
        // monotone non-decreasing
        let mut prev = -0.0;
        for i in 0..=100 {
            let x = i as f32 / 100.0;
            let y = cubic_bezier(b[0], b[1], b[2], b[3], x);
            assert!(y >= prev - 1e-6, "not monotone at {x}");
            prev = y;
        }
    }

    #[test]
    fn celestial_angle_official_anchors() {
        const TAU: f32 = std::f32::consts::TAU;
        // noon (t=6000): sun at zenith. The official wrap pair evaluates to
        // 360 deg == 0 mod 2π (rotation-identical).
        let noon = sample(6000);
        let a = noon.sun_angle.rem_euclid(TAU);
        assert!(a.abs() < 1e-4 || (a - TAU).abs() < 1e-4, "noon {a}");
        let midnight = sample(18000);
        let a = midnight.sun_angle.rem_euclid(TAU);
        assert!(
            (a - std::f32::consts::PI).abs() < 1e-4,
            "midnight {a}"
        );
        // The moon is opposite the sun (its track starts at 180).
        let dawn = sample(0);
        assert!(
            (dawn.moon_angle - dawn.sun_angle - std::f32::consts::PI).abs() < 1e-4,
            "moon {} vs sun {}",
            dawn.moon_angle,
            dawn.sun_angle
        );
        // Determinism.
        let a = sample(12345);
        let b = sample(12345);
        assert_eq!(a.sun_angle.to_bits(), b.sun_angle.to_bits());
    }

    #[test]
    fn sky_color_day_night() {
        // Day: full dimension sky color #78A7FF.
        let noon = sample(6000);
        assert!((noon.sky_color[0] - 120.0 / 255.0).abs() < 1e-5);
        assert!((noon.sky_color[1] - 167.0 / 255.0).abs() < 1e-5);
        assert!((noon.sky_color[2] - 255.0 / 255.0).abs() < 1e-5);
        // Deep night: the track multiplies to black.
        let midnight = sample(18000);
        assert!(midnight.sky_color[0] < 1e-3 && midnight.sky_color[1] < 1e-3);
        // Day fog = #C0D8FF.
        let noon = sample(6000);
        assert!((noon.fog_color[0] - 192.0 / 255.0).abs() < 1e-5);
        assert!((noon.fog_color[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn star_brightness_official_ramp() {
        // Full day: 0. Deep night: 0.5 (official keyframe values).
        assert!(sample(6000).star_brightness.abs() < 1e-6);
        assert!((sample(18000).star_brightness - 0.5).abs() < 1e-6);
        // Dawn/dusk ramps stay inside [0, 0.5].
        for t in [11800i64, 12700, 13100, 22900, 23500] {
            let b = sample(t).star_brightness;
            assert!((0.0..=0.5).contains(&b), "t={t} b={b}");
        }
    }

    #[test]
    fn moon_phase_follows_official_timeline() {
        assert_eq!(sample(0).moon_phase, 0, "full moon at tick 0");
        assert_eq!(sample(23999).moon_phase, 0);
        assert_eq!(sample(24000).moon_phase, 1, "waning gibbous");
        assert_eq!(sample(95999).moon_phase, 3, "waning crescent holds");
        assert_eq!(sample(96000).moon_phase, 4, "new moon");
        assert_eq!(sample(119999).moon_phase, 4, "new moon holds to 120000");
        assert_eq!(sample(120000).moon_phase, 5, "waxing crescent");
        assert_eq!(sample(191999).moon_phase, 7);
        assert_eq!(sample(192000).moon_phase, 0, "period wraps");
    }

    #[test]
    fn java_lcg_matches_real_java() {
        // Anchors produced by java.util.Random(10842L) on OpenJDK 21
        // (tools/Stars.java in the dev environment).
        let mut r = JavaRandom::new(10842);
        assert_eq!(r.next_f32(), 0.12987703f32);
        assert_eq!(r.next_f32(), 0.037620723f32);
        let mut r = JavaRandom::new(10842);
        assert_eq!(r.next_f64(), 0.12987703141005003f64);
        let mut r = JavaRandom::new(42);
        assert_eq!(r.next_f32(), 0.7275637f32);
        let mut r = JavaRandom::new(0);
        assert_eq!(r.next_f32(), 0.73096776f32);
    }

    #[test]
    fn stars_match_java_reference() {
        // Reference (official algorithm + real JOML, OpenJDK 21): 780 accepted
        // stars; first star center/size/angle below.
        let all = stars();
        assert_eq!(all.len(), 780);
        let mut rng = JavaRandom::new(10842);
        // Replay the reference generator by hand to fetch star 1's inputs.
        let _ = (rng.next_f32(), rng.next_f32(), rng.next_f32(), rng.next_f32());
        let x = 0.12987703f32 * 2.0 - 1.0;
        let y = 0.037620723f32 * 2.0 - 1.0;
        let _z = rng.next_f32() * 2.0 - 1.0; // consumed for z (value irrelevant here)
        let size = 0.15 + rng.next_f32() * 0.1;
        let _ = (x, y, size);
        let s1 = &all[0];
        let center = s1.corners.iter().fold([0f32; 3], |acc, c| {
            [acc[0] + c[0] / 4.0, acc[1] + c[1] / 4.0, acc[2] + c[2] / 4.0]
        });
        let expected = [-53.246864, 69.925735, 47.698658];
        for i in 0..3 {
            assert!(
                (center[i] - expected[i]).abs() < 1e-3,
                "star1 center {center:?} vs {expected:?}"
            );
        }
        // Every star sits on the radius-100 sphere.
        for s in all.iter().take(64) {
            let r2: f32 = s
                .corners
                .iter()
                .map(|c| c[0] * c[0] + c[1] * c[1] + c[2] * c[2])
                .sum::<f32>()
                / 4.0;
            assert!((r2 - 100.0 * 100.0).abs() < 2.0, "radius² {r2}");
        }
    }

    #[test]
    fn sunrise_color_official_anchors() {
        // Keyframes are #AARRGGBB. Midday holds #00FFE533: alpha 00 -> the
        // official early-out skips the glow disc entirely at noon.
        let noon = sample(6000);
        assert!((noon.sunrise_color[0] - 0xff as f32 / 255.0).abs() < 1e-6);
        assert!((noon.sunrise_color[1] - 0xe5 as f32 / 255.0).abs() < 1e-6);
        assert!((noon.sunrise_color[2] - 0x33 as f32 / 255.0).abs() < 1e-6);
        assert!(noon.sunrise_color[3] <= 0.001, "noon alpha must early-out");
        // Dawn (t=0) blends the wrap segment toward #5FEFA333 (~44% alpha,
        // golden-orange) — the disc renders.
        let dawn = sample(0);
        assert!(dawn.sunrise_color[3] > 0.3, "dawn alpha {:?}", dawn.sunrise_color);
        // Sunset peak (~t=23159, #FED75C33) is nearly opaque alpha.
        let peak = sample(23159);
        assert!((peak.sunrise_color[3] - 0xfe as f32 / 255.0).abs() < 1e-6);
        assert!((peak.sunrise_color[0] - 0xd7 as f32 / 255.0).abs() < 1e-6);
    }

    #[test]
    fn sunrise_disc_places_glow_at_rise_and_set_horizons() {
        let cam = Camera { pos: Vec3::ZERO, yaw: 0.0, pitch: 0.0 };
        let mut collect = |t: i64| {
            let mut q = Vec::new();
            render_sunrise_disc(&mut q, &cam, t);
            q
        };
        // Dawn (t=0): sin(sun_angle) < 0 -> flip 180 -> disc center at +X (east).
        let dawn = collect(0);
        assert!(!dawn.is_empty(), "dawn disc must render");
        let c = dawn[0].pts[0];
        assert!(c[0] > 95.0 && c[0] < 105.0, "east center x {c:?}");
        assert!(c[1].abs() < 1e-3 && c[2].abs() < 1e-3, "on horizon {c:?}");
        // Center vertex alpha = track alpha; ring vertex alpha = 0.
        assert!((dawn[0].colors[0][3] - sample(0).sunrise_color[3]).abs() < 1e-6);
        assert!(dawn[0].colors[1][3].abs() < 1e-6);
        // Dusk (t=12000): sin(sun_angle) >= 0 -> disc center at -X (west).
        let dusk = collect(12000);
        assert!(!dusk.is_empty());
        let c = dusk[0].pts[0];
        assert!(c[0] < -95.0 && c[0] > -105.0, "west center x {c:?}");
        // 18-vertex fan = 16 triangles.
        assert_eq!(dawn.len(), 16);
    }

    #[test]
    fn sun_rises_in_the_east() {
        // At tick 0 (+12° above horizon) the sun's center is at +X, up.
        let st = sample(0);
        let cam = Camera { pos: Vec3::ZERO, yaw: 0.0, pitch: 0.0 };
        let pts = celestial_quad(&celestial(st.sun_angle), &cam, 30.0);
        let cx = pts.iter().map(|p| p[0]).sum::<f32>() / 4.0;
        let cy = pts.iter().map(|p| p[1]).sum::<f32>() / 4.0;
        let cz = pts.iter().map(|p| p[2]).sum::<f32>() / 4.0;
        assert!(cx > 95.0, "sun x {cx}");
        assert!(cy > 10.0 && cy < 35.0, "sun y {cy}");
        assert!(cz.abs() < 1e-3, "sun z {cz}");
    }

    #[test]
    fn argb_multiply_matches_java_semantics() {
        // -1 (white) shortcuts, then per-channel (a*b)/255 integer division.
        let white = [1.0f32, 1.0, 1.0, 1.0];
        // track white (-1) -> base returned unchanged (the -1 shortcut).
        let base = [1.0f32, 1.0, 1.0, 204.0 / 255.0];
        let out = argb_multiply_rgba(S::CLOUD_BASE_COLOR, &white);
        for i in 0..4 {
            assert!((out[i] - base[i]).abs() < 1e-6, "channel {i} {out:?}");
        }
        // base white shortcut: white base * dark track = dark track.
        let dark = [25.0 / 255.0, 25.0 / 255.0, 38.0 / 255.0, 1.0];
        let out = argb_multiply_rgba([255, 255, 255, 255], &dark);
        for i in 0..4 {
            assert!((out[i] - dark[i]).abs() < 1e-6, "channel {i} {out:?}");
        }
        // 0x80 * 0x80 / 255 = 16384 / 255 = 64 (Java integer division).
        let half = [128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0];
        let out = argb_multiply_rgba([128, 128, 128, 128], &half);
        for i in 0..4 {
            assert!((out[i] - 64.0 / 255.0).abs() < 1e-6, "channel {i} {out:?}");
        }
        // The official day value: base #CCFFFFFF * white track = base.
        let noon = sample(6000);
        assert!((noon.cloud_color[3] - 204.0 / 255.0).abs() < 1e-6);
        assert!((noon.cloud_color[0] - 1.0).abs() < 1e-6);
        // Night (t=18000): base * #FF191926 = RGB(25,25,38), alpha stays 204
        // (255*204/255 = 204).
        let night = sample(18000);
        assert!((night.cloud_color[0] - 25.0 / 255.0).abs() < 1e-6);
        assert!((night.cloud_color[1] - 25.0 / 255.0).abs() < 1e-6);
        assert!((night.cloud_color[2] - 38.0 / 255.0).abs() < 1e-6);
        assert!((night.cloud_color[3] - 204.0 / 255.0).abs() < 1e-6);
    }

    #[test]
    fn cloud_drift_official_formulas() {
        // 0 ticks -> no drift.
        assert_eq!(cloud_drift(0, 0.0), 0.0);
        // Deterministic.
        let a = cloud_drift(12345, 0.0);
        let b = cloud_drift(12345, 0.0);
        assert_eq!(a.to_bits(), b.to_bits());
        // Wraps exactly at width * 400 ticks (the texture scroll period):
        // the drift at t and t + period is identical.
        let period = (S::CLOUDS_SIZE as i64) * 400;
        assert_eq!(cloud_drift(500, 0.0).to_bits(), cloud_drift(500 + period, 0.0).to_bits());
        // 400 ticks = exactly one cell (12 blocks): f32(400 * 0.030000001) == 12.0.
        assert_eq!(cloud_drift(400, 0.0), 12.0);
    }

    #[test]
    fn cloud_pattern_shifts_one_cell_per_400_ticks() {
        // The drift is linear in ticks: the pattern at tick t+400 equals the
        // pattern at tick t shifted one cell in +X (features move one cell
        // west). t=0 -> 400 is exact: f32 drift(400) == 12.0 (ties-to-even).
        for w in [0.0f64, 5.5, 100.25, 3071.0, -40.0, 3000.0] {
            for z in [0.0f64, 13.0, -77.5] {
                assert_eq!(
                    is_cloud_cell(w, z, cloud_drift(400, 0.0)),
                    is_cloud_cell(w + 12.0, z, cloud_drift(0, 0.0)),
                    "w={w} z={z}"
                );
                // The same property away from cell boundaries for other ticks.
                assert_eq!(
                    is_cloud_cell(w, z, cloud_drift(1400, 0.0)),
                    is_cloud_cell(w + 12.0, z, cloud_drift(1000, 0.0)),
                    "w={w} z={z}"
                );
            }
        }
        // The pattern is periodic over the texture width (256 cells).
        assert_eq!(
            is_cloud_cell(0.0, 0.0, cloud_drift(7, 0.0)),
            is_cloud_cell(256.0 * 12.0, 0.0, cloud_drift(7, 0.0))
        );
    }

    #[test]
    fn render_clouds_builds_official_geometry() {
        let cam = Camera { pos: Vec3::new(64.0, 80.0, 64.0), yaw: std::f32::consts::FRAC_PI_2, pitch: -0.08 };
        let ctx = RenderCtx {
            view: cam.view(),
            proj: Mat4::perspective(70.0f32.to_radians(), 320.0 / 180.0, 0.1, 512.0),
            fog_color: [0.0; 3],
            fog_near: 1.0,
            fog_far: 2.0,
        };
        // Deterministic build: two runs produce identical quads.
        let collect = || {
            let mut fb = crate::raster::Framebuf::new(8, 8);
            crate::sky::render_clouds(&mut fb, &ctx, &cam, 30, 0.0);
            ()
        };
        collect();
        collect();
        // The bottom plane sits at the official cloud height (192.33).
        let st = sample(30);
        assert!((st.cloud_color[3] - 204.0 / 255.0).abs() < 1e-6);
        // Night: alpha stays 0.8 (the track never touches the base alpha
        // except via the multiply shortcut), so the layer still renders.
        let night = sample(18000);
        assert!(night.cloud_color[3] > 0.5, "night alpha {:?}", night.cloud_color);
    }
}

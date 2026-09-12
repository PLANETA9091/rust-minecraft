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
use crate::raster::{FlatQuad, RenderCtx};

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
    /// MoonPhase ordinal (full moon = 0, matching the official enum order).
    pub moon_phase: u8,
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
    let mut eval = |ft: i32, fv: f32, tt: i32, tv: f32| -> Option<f32> {
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

/// Sample one RGBA color track (LINEAR in the official data), returning RGB.
fn sample_color_rgb(kf: &[(i32, [u8; 4])], t: i64, period: i32) -> [f32; 3] {
    if kf.len() == 1 {
        let c = kf[0].1;
        return [c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0];
    }
    let lt = t.rem_euclid(period as i64) as i32;
    let first = &kf[0];
    let last = &kf[kf.len() - 1];
    let mut eval = |ft: i32, fv: [u8; 4], tt: i32, tv: [u8; 4]| -> Option<[f32; 3]> {
        if lt <= tt {
            let lerp = |a: u8, b: u8| -> f32 {
                (a as f32 + (b as f32 - a as f32) * ((lt - ft) as f32 / (tt - ft) as f32)) / 255.0
            };
            return Some([lerp(fv[0], tv[0]), lerp(fv[1], tv[1]), lerp(fv[2], tv[2])]);
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
    match eval(last.0, last.1, first.0 + period, first.1) {
        Some(v) => v,
        None => [1.0, 1.0, 1.0],
    }
}

/// Evaluate the whole official day-night cycle at world tick `day_time`.
pub fn sample(day_time: i64) -> SkyState {
    let day = S::DAY_PERIOD_TICKS as i64;
    let sun_deg = sample_f32(&S::SUN_ANGLE_KF, Some(S::SUN_ANGLE_BEZIER), day_time, S::DAY_PERIOD_TICKS);
    let moon_deg =
        sample_f32(&S::MOON_ANGLE_KF, Some(S::MOON_ANGLE_BEZIER), day_time, S::DAY_PERIOD_TICKS);
    let star_deg =
        sample_f32(&S::STAR_ANGLE_KF, Some(S::STAR_ANGLE_BEZIER), day_time, S::DAY_PERIOD_TICKS);
    let star_brightness =
        sample_f32(&S::STAR_BRIGHTNESS_KF, None, day_time, S::DAY_PERIOD_TICKS).max(0.0);
    let sky_track = sample_color_rgb(&S::SKY_COLOR_KF, day_time, S::DAY_PERIOD_TICKS);
    let fog_track = sample_color_rgb(&S::FOG_COLOR_KF, day_time, S::DAY_PERIOD_TICKS);
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
    SkyState {
        sun_angle: sun_deg * DEG_TO_RAD,
        moon_angle: moon_deg * DEG_TO_RAD,
        star_angle: star_deg * DEG_TO_RAD,
        star_brightness,
        sky_color,
        fog_color,
        moon_phase,
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

/// Render the sun, moon (current phase) and stars for world tick `day_time`.
/// Must be called after the sky-color clear and BEFORE terrain quads
/// (official order: sky disc -> celestials -> terrain).
pub fn render_celestials(fb: &mut crate::raster::Framebuf, ctx: &RenderCtx, cam: &Camera, day_time: i64) {
    let st = sample(day_time);
    let mut quads: Vec<FlatQuad> = Vec::with_capacity(stars().len() + 2);

    // sun
    let sun = celestial_quad(&celestial(st.sun_angle), cam, 30.0);
    quads.push(FlatQuad {
        pts: sun,
        colors: [[1.0; 3]; 4],
        rgba: Some((&S::SUN_RGBA, S::CELESTIAL_SIZE)),
    });
    // moon (phase-selected texture, mirrored UVs like the official buffer)
    let moon = celestial_quad(&celestial(st.moon_angle), cam, 20.0);
    quads.push(FlatQuad {
        pts: moon,
        colors: [[1.0; 3]; 4],
        rgba: Some((&S::MOON_RGBA[st.moon_phase as usize], S::CELESTIAL_SIZE)),
    });
    // stars: brightness-tinted, alpha-blended (official STARS pipeline).
    let m = celestial(st.star_angle);
    let b = st.star_brightness;
    for star in stars() {
        let mut pts = [[0f32; 3]; 4];
        for (i, c) in star.corners.iter().enumerate() {
            let w = m.transform_point(Vec3::new(c[0], c[1], c[2]));
            pts[i] = [w[0] + cam.pos.x, w[1] + cam.pos.y, w[2] + cam.pos.z];
        }
        quads.push(FlatQuad { pts, colors: [[b, b, b]; 4], rgba: None });
    }
    fb.draw_flat(ctx, &quads);
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
}

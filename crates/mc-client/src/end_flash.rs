//! The END skybox's flash — the brief sheet of light the end sky shows
//! every so often. Everything here is javap-pinned from the 26.2 jar
//! (`EndFlashState`, `SingleThreadedRandomSource`, `Mth`) and is a
//! DETERMINISTIC function of the world's game time: the per-window
//! randomness is a SEEDED draw (`RandomSource.createThreadLocalInstance(
//! gameTime / 600)`), so a pixel-deterministic client reproduces it
//! exactly — no documented drift, no dropped shimmer.
//!
//! The chain (javap, in order):
//! - `tick(gameTime)`: the flash window is `gameTime / 600`
//!   (FLASH_INTERVAL_IN_TICKS); when the window index CHANGES, the
//!   parameters redraw from a fresh LCG seeded by the window index. The
//!   draw order is fixed and observable: one DISCARDED `nextFloat`, the
//!   offset `randomBetweenInclusive(0, 200)`, the duration
//!   `randomBetweenInclusive(100, min(380, 600 - offset))`, then the
//!   angles `randomBetween(-60, 10)` and `randomBetween(-180, 180)`.
//!   NOTE the vanilla defaults: the state starts with `flashSeed == 0`
//!   and window(0) == 0, so the FIRST window (gameTime 0..599) never
//!   recalculates — it renders nothing, and the first real flash is
//!   window 1.
//! - intensity at tick `t` inside the window: 0 outside
//!   `[offset, offset + duration]`, else the table sin hump
//!   `Mth.sin((t - offset) * PI / duration)` — the famous 65536-entry
//!   SIN table (values `sin(i / 10430.378350470453)`), NOT libm sin.
//! - `getIntensity(partial)` = `Mth.lerp(partial, oldIntensity,
//!   intensity)` — `old + partial * (new - old)`, the last-tick fade.
//!   This client renders integral ticks (the frame == tick model every
//!   other sky consumer rides), so the draw reads the tick-exact CURRENT
//!   intensity — [`EndFlashState::current`]; the lerp still exists and
//!   is pinned, the vanilla shape intact.

use std::sync::OnceLock;

use crate::sky::JavaRandom;

/// FLASH_INTERVAL_IN_TICKS (javap EndFlashState): the window length the
/// flash seed rides.
pub const FLASH_INTERVAL_IN_TICKS: i64 = 600;

/// `Mth.SIN` — the 65536-entry table (javap Mth's static init):
/// `SIN[i] = (float) Math.sin(i / 10430.378350470453)` — the DIVISION the
/// bytecode does, each value rounded once to f32.
fn sin_table() -> &'static [f32; 65536] {
    static TABLE: OnceLock<[f32; 65536]> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut table = [0f32; 65536];
        for (i, v) in table.iter_mut().enumerate() {
            *v = (i as f64 / 10430.378_350_470_453).sin() as f32;
        }
        table
    })
}

/// `Mth.sin(double)` (javap): `SIN[(long)(x * 10430.378350470453) & 65535]`
/// — the multiply in f64, `as i64` truncating toward zero exactly like the
/// JVM's d2l (NaN saturates to 0 in both), then the 16-bit mask.
fn mth_sin(x: f64) -> f32 {
    let idx = (x * 10430.378_350_470_453) as i64 & 65535;
    sin_table()[idx as usize]
}

/// The END sky's flash state — the vanilla `EndFlashState` fields and
/// methods, deterministic in the world's game time.
pub struct EndFlashState {
    flash_seed: i64,
    offset: i32,
    duration: i32,
    intensity: f32,
    old_intensity: f32,
    x_angle: f32,
    y_angle: f32,
}

impl Default for EndFlashState {
    fn default() -> Self {
        Self::new()
    }
}

impl EndFlashState {
    /// The vanilla field defaults: everything zero — and because
    /// `flash_seed == 0 == window(0)`, the first window never redraws.
    pub fn new() -> Self {
        EndFlashState {
            flash_seed: 0,
            offset: 0,
            duration: 0,
            intensity: 0.0,
            old_intensity: 0.0,
            x_angle: 0.0,
            y_angle: 0.0,
        }
    }

    /// One level tick (vanilla `EndFlashState.tick(gameTime)`): redraw the
    /// window's parameters when the window index changed, then roll
    /// `old <- current` and recompute the current intensity. Idempotent
    /// for a repeated `game_time`.
    pub fn tick(&mut self, game_time: i64) {
        let window = game_time / FLASH_INTERVAL_IN_TICKS;
        if window != self.flash_seed {
            let mut rng = JavaRandom::new(window);
            rng.next_f32(); // the discarded draw (javap: nextFloat, pop)
            self.offset = rng.next_int(201); // randomBetweenInclusive(0, 200)
            let max = 380.min(FLASH_INTERVAL_IN_TICKS as i32 - self.offset);
            self.duration = rng.next_int(max - 100 + 1) + 100;
            self.x_angle = rng.next_f32() * (10.0f32 - -60.0f32) + -60.0f32;
            self.y_angle = rng.next_f32() * (180.0f32 - -180.0f32) + -180.0f32;
            self.flash_seed = window;
        }
        self.old_intensity = self.intensity;
        self.intensity = self.calculate_intensity(game_time);
    }

    /// `calculateIntensity` (javap): 0 outside the flash span, else the
    /// table-sin hump over it — the f32 arithmetic the bytecode does
    /// (`(t - offset) as f32 * PI / duration as f32`, promoted to f64 for
    /// the table multiply).
    fn calculate_intensity(&self, game_time: i64) -> f32 {
        let t = game_time % FLASH_INTERVAL_IN_TICKS;
        let offset = self.offset as i64;
        let end = offset + self.duration as i64;
        if t < offset || t > end {
            return 0.0;
        }
        let v = ((t - offset) as f32 * 3.141_592_7_f32) / self.duration as f32;
        mth_sin(v as f64)
    }

    /// `getIntensity(partialTick)`: the last-tick fade,
    /// `old + partial * (current - old)` (Mth.lerp, f32 ops).
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn intensity(&self, partial_tick: f32) -> f32 {
        self.old_intensity + partial_tick * (self.intensity - self.old_intensity)
    }

    /// The tick-exact current intensity — what this client draws (the
    /// integral-tick frame model; see the module notes).
    pub fn current(&self) -> f32 {
        self.intensity
    }

    /// `getXAngle()` — the flash bearing, degrees (constant per window).
    pub fn x_angle(&self) -> f32 {
        self.x_angle
    }

    /// `getYAngle()` — the flash bearing, degrees (constant per window).
    pub fn y_angle(&self) -> f32 {
        self.y_angle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_java_lcg_matches_the_jvm_sequence() {
        // the shared sky::JavaRandom LCG pins, hand-derived from the javap
        // constants (the scramble, the step, the truncating top-bits take)
        // and cross-checked against the Python oracle.
        let mut r = JavaRandom::new(0);
        // the discarded nextFloat window 0 draws: 0.7309677600860596
        assert!((r.next_f32() - 0.730_967_76_f32).abs() < 1e-9);
        // the power-of-two shortcut: (bound * next(31)) >> 31
        let mut r = JavaRandom::new(7);
        let mut expect = JavaRandom::new(7);
        let n = expect.next(31);
        assert_eq!(r.next_int(1024), ((1024i64 * n) >> 31) as i32);
        // the rejection loop over a non-power-of-two bound (201 = 8 bits)
        let mut r = JavaRandom::new(0);
        r.next_f32();
        // window 0's would-be draw (never used by the state — see tick()):
        // offset 34 from the oracle
        assert_eq!(r.next_int(201), 34);
    }

    #[test]
    fn the_first_window_never_redraws_the_defaults() {
        // the vanilla defaults ride window 0: flashSeed 0 == window(0), so
        // tick(0) and the whole first window render NOTHING (offset 0,
        // duration 0, angles 0) — the first real flash is window 1.
        let mut st = EndFlashState::new();
        st.tick(0);
        assert_eq!(st.x_angle(), 0.0);
        assert_eq!(st.y_angle(), 0.0);
        assert_eq!(st.current(), 0.0, "sin(0 * PI / 0) lands on SIN[0] = 0");
        st.tick(599);
        assert_eq!(st.current(), 0.0, "t=599 is outside the empty [0, 0] span");
        // window 1 arrives at gameTime 600: the parameters redraw
        st.tick(600);
        assert_eq!(st.x_angle(), -31.479_215_6_f32, "oracle window 1");
        assert_eq!(st.y_angle(), -105.222_67_f32, "oracle window 1");
        assert_eq!(st.current(), 0.0, "t=600 % 600 = 0 is before offset 133");
    }

    #[test]
    fn the_window_one_flash_matches_the_oracle() {
        // the oracle's window 1: offset 133, duration 311 — the hump rises
        // from 0 at t=133, peaks ~1.0 mid-flash, and is 0 outside the span.
        let mut st = EndFlashState::new();
        st.tick(600);
        st.tick(733); // t = 133: the flash's first tick — sin(0) = 0
        assert_eq!(st.current(), 0.0, "the hump starts at zero");
        st.tick(733 + 155); // the hump's middle: sin(~PI/2)
        let mid = st.current();
        assert!((mid - 1.0).abs() < 1e-3, "the hump peaks at 1, got {mid}");
        st.tick(600 + 133 + 311); // t = offset + duration: the last span tick
        assert!(st.current().abs() < 1e-4, "sin(PI) lands on the table's ~0");
        st.tick(600 + 133 + 312); // one past the span: exactly 0
        assert_eq!(st.current(), 0.0);
        // the angles stay the window's own for the whole window
        assert_eq!(st.x_angle(), -31.479_215_6_f32);
    }

    #[test]
    fn window_two_redraws_and_the_lerp_fades_between_ticks() {
        // window 2 (gameTime 1200): offset 138, duration 296 (the oracle)
        let mut st = EndFlashState::new();
        st.tick(600);
        st.tick(1200);
        assert_eq!(st.x_angle(), -59.709_076_f32, "oracle window 2");
        // the lerp: after tick(1200) old = intensity(1199) = 0.0 (outside
        // window 1's span... actually t=599 of window 1), current = 0.0
        // (t=0 of window 2) — both 0; tick into the flash and the fade
        // interpolates the two tick values.
        st.tick(1338); // window 2's first flash tick: current = 0.0 (sin 0)
        assert_eq!(st.intensity(0.0), st.old_intensity_at_last_tick());
        // partial 1.0 reads the current tick exactly, partial 0.0 the last
        st.tick(1339); // one tick into the hump
        let half = st.intensity(0.5);
        assert!(half > 0.0 && half < st.intensity(1.0), "the fade sits between");
        assert_eq!(st.intensity(1.0), st.current());
    }

    impl EndFlashState {
        /// test accessor: the old-intensity slot (the lerp's left end).
        fn old_intensity_at_last_tick(&self) -> f32 {
            self.old_intensity
        }
    }
}

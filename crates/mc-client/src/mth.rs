//! `net/minecraft/util/Mth` — THE SIN TABLE's own class, the lookup the
//! whole game's trigonometry rides (javap, verbatim).
//!
//! - the static init fills the 65536 entries once (the
//!   `lambda$static$0` Consumer the invokedynamic spins):
//!   `SIN[i] = (float) Math.sin(i / 10430.378350470453)` — the DIVISION
//!   the bytecode speaks (`i2d; ldc2_w #17; ddiv; invokestatic
//!   Math.sin:(D)D; d2f`), never a multiply by `2π/65536`.
//! - `Mth.sin(double) = SIN[(long)(d · 10430.378350470453) & 65535]` —
//!   the multiply in f64 (`dmul`), the `d2l` truncation toward zero
//!   (NaN → 0, ±∞ saturate — the semantics Rust's `as i64` shares),
//!   the 16-bit mask, `l2i`, the `faload`.
//! - `Mth.cos(double) = SIN[(long)(d · 10430.378350470453 + 16384.0) &
//!   65535]` — the SAME table, the quarter-turn offset rides INSIDE the
//!   single `d2l` (`dmul; ldc2_w #21; dadd; d2l`), never a second array.
//! - the scale the CP keeps: `#17` the f64 bits `0x40c45f306dc9c883` —
//!   65536 / 2π the real value names, the decimal literal round-trips.
//!
//! The swing lanes speak THIS lookup now — the third-person avatar's
//! walk/whack folds and the first-person hand's arcs the `Mth.sin/cos`
//! calls feed (`HumanoidModel.setupAttackAnimation`'s every
//! `invokestatic #1.sin:(D)F`, `Ease.inOutSine`'s own cos), the
//! end-flash lane rode the table from its own round. The `Math.sin/cos`
//! convention the client rode before quantized the VALUE instead of the
//! ANGLE — the sub-pixel divergence the autopush-476 round banked, this
//! round the table retires.

use std::sync::OnceLock;

/// The CP's `ldc2_w #17` — the f64 the decimal literal round-trips to
/// (65536 / 2π the real value names).
const TABLE_SCALE: f64 = 10430.378_350_470_453;

/// `Mth.SIN` — the 65536-entry table (javap Mth's static init):
/// `SIN[i] = (float) Math.sin(i / 10430.378350470453)` — the DIVISION
/// the bytecode does, each value rounded once to f32.
pub fn sin_table() -> &'static [f32; 65536] {
    static TABLE: OnceLock<[f32; 65536]> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut table = [0f32; 65536];
        for (i, v) in table.iter_mut().enumerate() {
            *v = (i as f64 / TABLE_SCALE).sin() as f32;
        }
        table
    })
}

/// `Mth.sin(double)` (javap, verbatim): `SIN[(long)(d · C) & 65535]` —
/// the multiply in f64, the `d2l` truncation toward zero exactly like
/// the JVM's (NaN saturates to 0 in both), then the 16-bit mask.
pub fn sin(d: f64) -> f32 {
    sin_table()[((d * TABLE_SCALE) as i64 & 65535) as usize]
}

/// `Mth.cos(double)` (javap, verbatim): `SIN[(long)(d · C + 16384.0) &
/// 65535]` — the quarter-turn offset the single `d2l` folds.
pub fn cos(d: f64) -> f32 {
    sin_table()[((d * TABLE_SCALE + 16384.0) as i64 & 65535) as usize]
}

/// `Mth.floor(double)` (javap, verbatim): `Math.floor(d)` then `d2i` —
/// the toward-negative-infinity fold the JVM's own Math.floor speaks
/// (the truncation-toward-zero the plain `as` cast wears NEVER rides),
/// the d2i the NaN answers 0 (the saturation Rust's `as` shares).
pub fn floor(d: f64) -> i32 {
    d.floor() as i32
}

/// `Mth.frac(double)` — the fractional part the DDA's t-max init keeps
/// (the same Math.floor the walk serves): `d - floor(d)`, ALWAYS in
/// 0..1 — the negative edge the toward-negative fold keeps positive.
pub fn frac(d: f64) -> f64 {
    d - floor(d) as f64
}

/// `Mth.sign(double) → int` — the per-axis step direction the DDA's
/// march wears: the positive the 1 answers, the negative the -1, the
/// zero (and the negative zero) the 0 keeps.
pub fn sign(d: f64) -> i32 {
    if d > 0.0 {
        1
    } else if d < 0.0 {
        -1
    } else {
        0
    }
}

/// `Mth.lerp(delta, start, end)` — `start + delta * (end - start)`, the
/// -1e-7 endpoint nudge the traverseBlocks walk's own endpoint folds
/// wear (the boundary cells the nudge keeps unambiguous).
pub fn lerp(delta: f64, start: f64, end: f64) -> f64 {
    start + delta * (end - start)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// THE FILL LAW — the special rows the exact values speak: the
    /// zero row stays 0.0, the quarter table at 16384 sins to 1.0
    /// exactly (the division lands the double π/2), the half at 32768
    /// keeps the famous 1.2246e-16 (sin of the double closest to π —
    /// `32768 / C` the same double the constant names), the
    /// three-quarter at 49152 sins to -1.0.
    #[test]
    fn the_sin_table_the_fill_the_special_rows_keep() {
        assert_eq!(sin_table()[0], 0.0);
        assert_eq!(sin_table()[16384], 1.0);
        // 32768 / C IS the f64 closest to π — the identity the rows share
        assert_eq!(32768f64 / TABLE_SCALE, std::f64::consts::PI);
        assert_eq!(
            sin_table()[32768],
            std::f64::consts::PI.sin() as f32,
            "the famous sin(double π) the row keeps"
        );
        assert_eq!(sin_table()[49152], -1.0);
    }

    /// THE DIVISION the fill speaks — the sampled rows equal the
    /// reference fold `(i as f64 / C).sin() as f32` bit-for-bit; the
    /// multiplication by 2π/65536 the bytecode never does (the last-bit
    /// difference the alternate fold would wear the equality refuses).
    #[test]
    fn the_sin_table_the_division_the_reference_keeps() {
        for i in [1usize, 2, 3, 5, 8192, 12345, 40009, 65535] {
            assert_eq!(sin_table()[i], (i as f64 / TABLE_SCALE).sin() as f32);
        }
        // the division's own verdict at the eighth row — 0.70710678 the
        // f32 keeps (the sqrt(2)/2 the eye names, the last ulp the
        // double π/4 never shares)
        assert_eq!(sin_table()[8192], (8192f64 / TABLE_SCALE).sin() as f32);
    }

    /// THE INDEX FOLD — the d2l truncation toward zero and the 16-bit
    /// mask the wrap wears: the sampled doubles' indices equal the
    /// reference fold bit-for-bit, the negative angle wraps to the
    /// table's top (−0.0001 · C truncates to −1, the mask lifts it to
    /// 65535), the full turn re-enters at the zero row, NaN speaks the
    /// zero row (the d2l the NaN answers 0 in both JVM and Rust).
    #[test]
    fn the_sin_the_index_the_truncation_the_wrap_keeps() {
        for d in [
            0.0f64, 0.5, -0.5, 1.0, -1.0, 25.7, -25.7, 100.3, 65536.0, -65536.0,
        ] {
            let idx = (d * TABLE_SCALE) as i64 & 65535;
            assert_eq!(sin(d), sin_table()[idx as usize]);
        }
        // the negative wrap the mask lifts
        assert_eq!((-0.0001f64 * TABLE_SCALE) as i64, -1);
        assert_eq!(sin(-0.0001), sin_table()[65535]);
        // the full turn the mask wraps
        assert_eq!(sin(std::f64::consts::TAU), sin_table()[0]);
        // NaN the zero row answers (the d2l fold's own verdict)
        assert_eq!(sin(f64::NAN), 0.0);
    }

    /// THE COS the SAME table wears — the quarter-turn offset rides
    /// INSIDE the truncation (`d·C + 16384.0` the single d2l folds),
    /// the zero angle the 16384 row answers (1.0 exactly), the sampled
    /// doubles' indices equal the reference fold bit-for-bit.
    #[test]
    fn the_cos_the_offset_the_same_table_keeps() {
        assert_eq!(cos(0.0), 1.0);
        for d in [0.0f64, 0.5, -0.5, 1.5707964, 3.1415927, 25.7, -25.7] {
            let idx = (d * TABLE_SCALE + 16384.0) as i64 & 65535;
            assert_eq!(cos(d), sin_table()[idx as usize]);
        }
        // the quarter-turn law the real identity names: cos(d) lands the
        // same row sin(d + π/2) does (the shared index the offset builds)
        let d = 0.77f64;
        assert_eq!(
            cos(d),
            sin_table()[(((d + std::f64::consts::FRAC_PI_2) * TABLE_SCALE) as i64 & 65535)
                as usize]
        );
    }

    /// THE CONSTANT the CP keeps — the decimal literal round-trips to
    /// the f64 bits 0x40c45f306dc9c883 (65536/2π the real value names,
    /// the jar's `ldc2_w #17` the same bits banks).
    #[test]
    fn the_scale_the_bits_the_decimal_roundtrips() {
        assert_eq!(TABLE_SCALE.to_bits(), 0x40c4_5f30_6dc9_c883);
    }

    /// THE FLOOR the d2i answers — the toward-negative-infinity fold
    /// the Math.floor speaks: the positive fraction truncates DOWN
    /// (7.5 → 7, the truncation the plain cast wears answers the same
    /// HERE), the negative fraction the floor lifts to the next-lowest
    /// integer (−0.9375 → −1, the truncation the cast wears answers 0 —
    /// the last-bit difference the vibration fold's own answer rides),
    /// the exact integer the identity keeps, the NaN the d2i's zero.
    #[test]
    fn the_floor_the_negative_edge_the_d2i_keeps() {
        assert_eq!(floor(7.5), 7);
        assert_eq!(floor(15.0), 15);
        assert_eq!(floor(-0.9375), -1, "the floor the next-lowest answers");
        assert_eq!((-0.9375f64) as i32, 0, "the plain cast the truncation wears");
        assert_eq!(floor(-1.0), -1);
        assert_eq!(floor(f64::NAN), 0, "the d2i the NaN answers zero");
        assert_eq!(floor(0.9375), 0);
    }
}

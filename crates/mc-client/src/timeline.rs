//! The day/night TIMELINE — the data-driven time system of 26.2.
//!
//! The audit trail (javap + jar data, this run): the client's sky
//! darkening is no longer a formula — `Level.updateSkyBrightness()`
//! reads `15 - SKY_LIGHT_LEVEL` from the dimension's ENVIRONMENT
//! ATTRIBUTES, and those attributes are TIMELINE TRACKS: per-dimension
//! keyframe curves over the dimension's default clock (the dimension
//! type's `default_clock` / `timelines` fields, overworld =
//! `minecraft:overworld` clock + `#minecraft:in_overworld` timelines).
//! The `WORLD_CLOCK` and `TIMELINE` registries are both in
//! `RegistryDataLoader`'s synced list — they ride the configuration
//! Registry Data to the client.
//!
//! The RENDER side consumes a separate track: the lightmap fragment
//! shader (`assets/minecraft/shaders/core/lightmap.fsh`, read verbatim
//! from the jar) computes
//! `sky_brightness = get_brightness(sky_level) * SkyFactor` — additive
//! with the block term — and `LightmapRenderState.skyFactor` is fed
//! from the timeline's `minecraft:visual/sky_light_factor` track.
//!
//! This module pins the overworld day timeline's sky_light_factor
//! keyframes as extracted from the OFFICIAL 26.2 jar
//! (`data/minecraft/timeline/day.json` — the `#minecraft:in_overworld`
//! tag's primary timeline, bound to the overworld clock with
//! `period_ticks: 24000`):
//!
//! ```text
//! ticks  730 -> 1.0    (sunrise)
//! ticks 11270 -> 1.0   (sunset edge)
//! ticks 13140 -> 0.24  (night)
//! ticks 22860 -> 0.24  (pre-sunrise)
//! ```
//!
//! Keyframes interpolate LINEARLY between their ticks and the curve
//! wraps with the period (the segment 22860 -> 730+24000 completes the
//! cycle). The registry-driven curve (parse the TIMELINE registry from
//! the configuration payload like any other synced registry) is future
//! work; these constants are jar-extracted today, and `day.json`'s
//! `minecraft:gameplay/sky_light_level` track (1.0 day / 0.2666.. night
//! = level 4 — the same 11 the old cos-formula peaked at) corroborates
//! the gameplay-side `skyDarken = 15 - 15 * value` semantics while the
//! RENDER factor above is what this module feeds the mesher.

/// The overworld day timeline's period (day.json `period_ticks`).
pub const DAY_PERIOD_TICKS: i64 = 24_000;

/// The night plateau of the visual/sky_light_factor track.
const NIGHT_FACTOR: f32 = 0.24;

/// (tick, value) keyframes of `minecraft:visual/sky_light_factor` in
/// `data/minecraft/timeline/day.json`, sorted by tick, cyclic with
/// [`DAY_PERIOD_TICKS`]. This IS the compile-time snapshot of the
/// synced TIMELINE registry — the fallback a server with no timeline
/// knowledge falls back to (see `WorldModel::sky_factor`).
const SKY_LIGHT_FACTOR_KEYS: [(i64, f32); 4] =
    [(730, 1.0), (11_270, 1.0), (13_140, NIGHT_FACTOR), (22_860, NIGHT_FACTOR)];

/// Cyclic-linear evaluation of one keyframe curve (sorted by tick):
/// linear between keyframes, and the wrap segment (last keyframe ->
/// first keyframe one period later) completes the cycle. A curve
/// whose timeline carries no `period_ticks` is evaluated NON-cyclic:
/// clamped at its end keyframes (the honest non-repeating case — real
/// vanilla's curve-carrier `minecraft:day` has period 24000). The
/// float ops are the ones `sky_light_factor` has pinned since its
/// golden-regenerating runs — the generalization changed the SHAPE,
/// never the arithmetic.
pub fn evaluate(keyframes: &[(i64, f32)], period: Option<i64>, tick: i64) -> f32 {
    let keys = keyframes;
    if keys.is_empty() {
        // a curve with no keyframes carries no opinion: full day
        return 1.0;
    }
    match period {
        Some(period) => {
            let t = tick.rem_euclid(period) as f64;
            for i in 0..keys.len() {
                let (t0, v0) = keys[i];
                let (t1, v1) = if i + 1 < keys.len() {
                    keys[i + 1]
                } else {
                    // the wrap segment: last keyframe -> first keyframe one
                    // period later
                    (keys[0].0 + period, keys[0].1)
                };
                let t1 = t1 as f64;
                if t >= t0 as f64 && t < t1 {
                    let f = (t - t0 as f64) / (t1 - t0 as f64);
                    return (v0 as f64 + f * (v1 - v0) as f64) as f32;
                }
            }
            // t == period - 0 (or a float edge): the last segment's
            // open end is the first keyframe's value one period later —
            // the day value.
            keys[0].1
        }
        None => {
            // non-cyclic: clamp at the end keyframes, interpolate inside
            if tick <= keys[0].0 {
                return keys[0].1;
            }
            let last = keys[keys.len() - 1];
            if tick >= last.0 {
                return last.1;
            }
            for i in 0..keys.len() - 1 {
                let (t0, v0) = keys[i];
                let (t1, v1) = keys[i + 1];
                if tick >= t0 && tick < t1 {
                    let f = (tick - t0) as f64 / (t1 - t0) as f64;
                    return (v0 as f64 + f * (v1 - v0) as f64) as f32;
                }
            }
            last.1
        }
    }
}

/// The timeline sky factor at a day tick (total ticks mod the day
/// period): 1.0 across the full day, 0.24 through the night plateau,
/// linearly interpolated through the two dawn/dusk ramps — including
/// the wrap-around segment from the last keyframe into the first.
/// The compile-time snapshot: the registry-driven path evaluates the
/// SAME keyframes through [`evaluate`] when the synced TIMELINE
/// registry can name the dimension's curve.
pub fn sky_light_factor(day_tick: i64) -> f32 {
    evaluate(&SKY_LIGHT_FACTOR_KEYS, Some(DAY_PERIOD_TICKS), day_tick)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor_follows_the_day_timeline_keyframes() {
        // plateau values straight from day.json
        assert_eq!(sky_light_factor(6_000), 1.0, "noon: full day");
        assert_eq!(sky_light_factor(18_000), NIGHT_FACTOR, "midnight: the night plateau");
        assert_eq!(sky_light_factor(730), 1.0, "the sunrise keyframe itself");
        assert_eq!(sky_light_factor(22_860), NIGHT_FACTOR, "the last keyframe itself");
        // the day plateau spans the two 1.0 keyframes exactly
        assert_eq!(sky_light_factor(11_270), 1.0);
        // linear ramps: the midpoint of the 11270..13140 dusk ramp
        let mid = sky_light_factor((11_270 + 13_140) / 2);
        assert!((mid - 0.62).abs() < 1e-6, "dusk midpoint {mid} = (1.0 + 0.24)/2");
        // wrap-around: tick 23_430 sits (23430-22860)/(24730-22860) of
        // the way through the dawn segment 22_860 -> 730 (+24_000)
        let f = (23_430 - 22_860) as f64 / (24_730 - 22_860) as f64;
        let wrap = sky_light_factor(23_430);
        assert!((wrap - (0.24 + f as f32 * 0.76)).abs() < 1e-6, "dawn wrap {wrap}");
        // and one full period is the same moment as tick 0
        assert!((sky_light_factor(24_000) - sky_light_factor(0)).abs() < 1e-9);
    }

    #[test]
    fn evaluate_serves_the_registry_driven_curves() {
        // the constants ARE a valid curve input: the day.json keyframes
        // through the general path equal the pinned fn exactly
        assert_eq!(evaluate(&SKY_LIGHT_FACTOR_KEYS, Some(DAY_PERIOD_TICKS), 18_000), 0.24);
        assert_eq!(
            evaluate(&SKY_LIGHT_FACTOR_KEYS, Some(DAY_PERIOD_TICKS), 6_000),
            sky_light_factor(6_000)
        );
        // a NON-cyclic curve (a timeline without period_ticks) clamps at
        // its ends and interpolates inside
        let curve = [(0i64, 1.0f32), (12_000, 0.5)];
        assert_eq!(evaluate(&curve, None, -40), 1.0, "clamped below the first keyframe");
        assert_eq!(evaluate(&curve, None, 99_999), 0.5, "clamped past the last keyframe");
        assert_eq!(evaluate(&curve, None, 6_000), 0.75, "linear inside");
        // an empty curve carries no opinion: the day
        assert_eq!(evaluate(&[], Some(24_000), 18_000), 1.0);
    }

    #[test]
    fn the_pinned_snapshot_matches_the_generated_timeline_table() {
        // the compile-time snapshot and the generator's built-in
        // minecraft:timeline table are the SAME jar data — this pin fails
        // the moment either side drifts (the hand-kept render constants
        // are only honest while they byte-match the generated table)
        let day = mc_registry::generated::worldgen::timeline("minecraft:day")
            .expect("the generated table carries the day timeline");
        assert_eq!(&SKY_LIGHT_FACTOR_KEYS[..], day.sky_light_factor);
        assert_eq!(Some(DAY_PERIOD_TICKS), day.period_ticks);
    }
}

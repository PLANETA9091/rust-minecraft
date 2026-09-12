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
/// [`DAY_PERIOD_TICKS`].
const SKY_LIGHT_FACTOR_KEYS: [(i64, f32); 4] =
    [(730, 1.0), (11_270, 1.0), (13_140, NIGHT_FACTOR), (22_860, NIGHT_FACTOR)];

/// The timeline sky factor at a day tick (total ticks mod the day
/// period): 1.0 across the full day, 0.24 through the night plateau,
/// linearly interpolated through the two dawn/dusk ramps — including
/// the wrap-around segment from the last keyframe into the first.
pub fn sky_light_factor(day_tick: i64) -> f32 {
    let t = day_tick.rem_euclid(DAY_PERIOD_TICKS) as f64;
    let keys = SKY_LIGHT_FACTOR_KEYS;
    for i in 0..keys.len() {
        let (t0, v0) = keys[i];
        let (t1, v1) = if i + 1 < keys.len() {
            keys[i + 1]
        } else {
            // the wrap segment: last keyframe -> first keyframe one
            // period later
            (keys[0].0 + DAY_PERIOD_TICKS, keys[0].1)
        };
        let t1 = t1 as f64;
        if t >= t0 as f64 && t < t1 {
            let f = (t - t0 as f64) / (t1 - t0 as f64);
            return (v0 as f64 + f * (v1 - v0) as f64) as f32;
        }
    }
    // t == DAY_PERIOD_TICKS - 0 (or a float edge): the last segment's
    // open end is the first keyframe's value one period later — the
    // day value.
    keys[0].1
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
        // wrap-around: tick 23_430 is halfway through 22_860..24_730
        let wrap = sky_light_factor(23_430);
        assert!((wrap - 0.62).abs() < 1e-6, "dawn wrap midpoint {wrap}");
        // and one full period is the same moment as tick 0
        assert!((sky_light_factor(24_000) - sky_light_factor(0)).abs() < 1e-9);
    }
}

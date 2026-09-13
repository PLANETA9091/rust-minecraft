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

/// (tick, raw 0..=255 channel value) of `minecraft:visual/sky_light_color`
/// in `data/minecraft/timeline/day.json` (extracted from the official
/// 26.2 jar, this run's javap/jar audit), sorted by tick, cyclic with
/// [`DAY_PERIOD_TICKS`]. Modifier `multiply` over the attribute DEFAULT:
/// the javap evidence (`EnvironmentAttributes.<init>`, jdk-25) pins the
/// `visual/sky_light_color` default to `-1` = 0xFFFFFFFF = WHITE. The
/// overworld dimension type carries NO base — for it the track IS the
/// whole value; the END and the NETHER state their own bases (#ac60cd /
/// #7a7aff — the moonlight pairs, the generated table's honest rows),
/// which the consumer's fold multiplies UNDER this track per channel.
/// White across the day; `#7a7aff` (the moonlight blue) through the
/// night plateau — the vanilla lightmap shader's `SkyLightColor`.
const SKY_LIGHT_COLOR_R: [(i64, f32); 4] =
    [(730, 255.0), (11_270, 255.0), (13_140, 122.0), (22_860, 122.0)];
const SKY_LIGHT_COLOR_G: [(i64, f32); 4] =
    [(730, 255.0), (11_270, 255.0), (13_140, 122.0), (22_860, 122.0)];
const SKY_LIGHT_COLOR_B: [(i64, f32); 4] =
    [(730, 255.0), (11_270, 255.0), (13_140, 255.0), (22_860, 255.0)];

/// The lightmap's SkyLightColor at a day tick, normalized 0..=1 per
/// channel: white by day, `#7a7aff` (122, 122, 255) through the night
/// plateau, linear ramps between — the same cyclic-linear walk
/// [`evaluate`] gives the factor, in raw 0..=255 channel space with ONE
/// final division (the exact convention the sky.rs color tracks pin:
/// interpolate raw, normalize last). The blue moonlight lands ONLY on
/// the lightmap's sky term — the sky dome's own color is a different
/// track (`visual/sky_color`, sky.rs's chain).
pub fn sky_light_color(day_tick: i64) -> [f32; 3] {
    [
        evaluate(&SKY_LIGHT_COLOR_R, Some(DAY_PERIOD_TICKS), day_tick) / 255.0,
        evaluate(&SKY_LIGHT_COLOR_G, Some(DAY_PERIOD_TICKS), day_tick) / 255.0,
        evaluate(&SKY_LIGHT_COLOR_B, Some(DAY_PERIOD_TICKS), day_tick) / 255.0,
    ]
}

/// The same track sample in BYTE space — the integer operand the
/// attribute-system fold multiplies the STATED base against. Vanilla's
/// track sample lands on an `Integer` before the fold (javap
/// AttributeTrackSampler.applyTimeBased -> AttributeModifier.apply ->
/// ARGB.multiply(int, int): the -1 white short-circuit, else per-channel
/// `imul` / `idiv 255` — the probe's folded value IS an Integer the
/// extractor unpacks through `ARGB.vector3fFromRGB24`), so the stated
/// fold multiplies in this space, never in floats. Between keyframes the
/// raw-space lerp rounds to the byte here (the same `evaluate` the float
/// sampler reads; vanilla's keyframe lerp is `ARGB.srgbLerp`, the
/// documented sampler-level approximation).
pub fn sky_light_color_bytes(day_tick: i64) -> [u8; 3] {
    [
        evaluate(&SKY_LIGHT_COLOR_R, Some(DAY_PERIOD_TICKS), day_tick).round() as u8,
        evaluate(&SKY_LIGHT_COLOR_G, Some(DAY_PERIOD_TICKS), day_tick).round() as u8,
        evaluate(&SKY_LIGHT_COLOR_B, Some(DAY_PERIOD_TICKS), day_tick).round() as u8,
    ]
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
    fn sky_light_color_tracks_the_moonlight_tint() {
        // day plateau: the white default — the track IS the whole value
        assert_eq!(sky_light_color(6_000), [1.0; 3], "noon: white moonlight-none");
        // the golden night tick (14010 + frame) and the riding tests'
        // midnight: the #7a7aff plateau, exact channels
        let blue = [122.0f32 / 255.0, 122.0 / 255.0, 1.0];
        assert_eq!(sky_light_color(14_010), blue, "the golden night tick is #7a7aff");
        assert_eq!(sky_light_color(18_000), blue, "midnight plateau is #7a7aff");
        // the sunrise keyframe itself and the day plateau edges
        assert_eq!(sky_light_color(730), [1.0; 3]);
        assert_eq!(sky_light_color(11_270), [1.0; 3]);
        // linear dusk ramp: the 11270->13140 midpoint per channel
        // (raw-space lerp: (255 + 0.5*(122-255))/255, blue stays 255)
        let mid = sky_light_color((11_270 + 13_140) / 2);
        let half = (255.0f32 + 0.5 * (122.0 - 255.0)) / 255.0;
        assert!((mid[0] - half).abs() < 1e-6, "dusk midpoint r {mid:?}");
        assert!((mid[1] - half).abs() < 1e-6, "dusk midpoint g {mid:?}");
        assert!((mid[2] - 1.0).abs() < 1e-6, "blue channel holds 255 {mid:?}");
        // wrap-around dawn: tick 23_430 sits (23430-22860)/1870 of the
        // way from #7a7aff back to white
        let f = (23_430 - 22_860) as f64 / (24_730 - 22_860) as f64;
        let wrap = sky_light_color(23_430);
        let expect_r = (122.0f32 + f as f32 * 133.0) / 255.0;
        assert!((wrap[0] - expect_r).abs() < 1e-6, "dawn wrap r {wrap:?}");
        assert!((wrap[2] - 1.0).abs() < 1e-6, "blue wraps at 255 {wrap:?}");
        // one full period is the same moment as tick 0
        assert!((sky_light_color(24_000)[0] - sky_light_color(0)[0]).abs() < 1e-9);
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

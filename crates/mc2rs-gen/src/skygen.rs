//! Sky extraction: the 26.x day-night cycle is DATA in the official jar.
//!
//! - `data/minecraft/timeline/day.json` — keyframe tracks for the sun/moon/
//!   star angles (cubic-bezier eased), star brightness and sky/fog color
//!   (linear), all over `period_ticks: 24000`.
//! - `data/minecraft/timeline/moon.json` — the 8 moon phases over a
//!   192000-tick period (full moon first, matching MoonPhase.ordinal).
//! - `data/minecraft/dimension_type/overworld.json` — base
//!   `visual/sky_color` / `visual/fog_color` the color tracks multiply.
//! - `assets/minecraft/textures/environment/celestial/*.png` — the real
//!   sun + 8 moon-phase textures (32x32 each in 26.2).
//!
//! Output: `crates/mc-registry/src/generated/sky.rs`. The client evaluator
//! (`mc-client/src/sky.rs`) replicates the game's KeyframeTrackSampler
//! semantics against this data — verified against javap disassembly of the
//! official jar (SkyRenderer, KeyframeTrackSampler, EasingType).

use anyhow::{bail, Context, Result};
use serde_json::Value;

use crate::atlas::decode_rgba;
use crate::extract::read_entry;

/// One float keyframe track: angles use `ease: cubic_bezier`, star
/// brightness uses the default LINEAR.
#[derive(Debug, Clone)]
pub struct FloatTrack {
    pub keyframes: Vec<(i32, f32)>,
    /// cubic_bezier control points when the track eased with one.
    pub bezier: Option<[f32; 4]>,
}

/// One ARGB color keyframe track (sky/fog: default LINEAR).
#[derive(Debug, Clone)]
pub struct ColorTrack {
    /// ARGB stored as [r, g, b, a].
    pub keyframes: Vec<(i32, [u8; 4])>,
}

pub struct SkyData {
    pub day_period: i32,
    pub moon_period: i32,
    pub sun_angle: FloatTrack,
    pub moon_angle: FloatTrack,
    pub star_angle: FloatTrack,
    pub star_brightness: FloatTrack,
    pub sky_color: ColorTrack,
    pub fog_color: ColorTrack,
    /// Official `visual/sunrise_sunset_color` — the tint of the horizon glow
    /// disc rendered by the game's renderSunriseAndSunset (ARGB, LINEAR).
    pub sunrise_color: ColorTrack,
    /// Dimension base colors the multiply-tracks apply over (RGBA).
    pub dim_sky_color: [u8; 4],
    pub dim_fog_color: [u8; 4],
    /// Phase file names in MoonPhase.ordinal order (full_moon first).
    pub moon_phase_names: Vec<String>,
    /// tick -> phase index, sorted by tick (period-wrap handled by the evaluator).
    pub moon_phase_kf: Vec<(i32, u8)>,
    pub sun_rgba: (u32, u32, Vec<u8>),
    /// One RGBA texture per phase, same order as `moon_phase_names`.
    pub moon_rgba: Vec<(u32, u32, Vec<u8>)>,
}

/// Parse "#rrggbb" (opaque) / "#aarrggbb" / signed ARGB int into RGBA.
pub fn parse_color(v: &Value) -> Result<[u8; 4]> {
    if let Some(s) = v.as_str() {
        let hex = s.strip_prefix('#').with_context(|| format!("color {s}: no #"))?;
        let un = |i: usize| -> Result<u8> {
            u8::from_str_radix(&hex[i..i + 2], 16).with_context(|| format!("color {s}: bad hex"))
        };
        return match hex.len() {
            6 => Ok([un(0)?, un(2)?, un(4)?, 0xFF]),
            8 => Ok([un(2)?, un(4)?, un(6)?, un(0)?]), // #AARRGGBB
            _ => bail!("color {s}: expected 6 or 8 hex digits"),
        };
    }
    if let Some(i) = v.as_i64() {
        let argb = i as u32; // JSON negative ints are signed ARGB
        return Ok([
            ((argb >> 16) & 0xFF) as u8,
            ((argb >> 8) & 0xFF) as u8,
            (argb & 0xFF) as u8,
            ((argb >> 24) & 0xFF) as u8,
        ]);
    }
    bail!("unsupported color value {v}");
}

fn parse_float_track(v: &Value) -> Result<FloatTrack> {
    let kfs = v["keyframes"]
        .as_array()
        .context("track: no keyframes")?;
    if kfs.is_empty() {
        bail!("track: empty keyframes");
    }
    let mut keyframes = Vec::with_capacity(kfs.len());
    for kf in kfs {
        let ticks = kf["ticks"].as_i64().context("keyframe: no ticks")? as i32;
        let value = kf["value"].as_f64().context("keyframe: no float value")? as f32;
        keyframes.push((ticks, value));
    }
    // ease: missing/"linear" => LINEAR; {"cubic_bezier": [x1,y1,x2,y2]} => bezier.
    let bezier = match v.get("ease") {
        None => None,
        Some(Value::String(s)) if s == "linear" => None,
        Some(Value::String(s)) => bail!("unsupported ease {s}"),
        Some(Value::Object(o)) => {
            let b = o
                .get("cubic_bezier")
                .and_then(|b| b.as_array())
                .context("ease: expected cubic_bezier")?;
            if b.len() != 4 {
                bail!("cubic_bezier needs 4 controls");
            }
            Some([
                b[0].as_f64().unwrap_or(0.0) as f32,
                b[1].as_f64().unwrap_or(0.0) as f32,
                b[2].as_f64().unwrap_or(0.0) as f32,
                b[3].as_f64().unwrap_or(0.0) as f32,
            ])
        }
        Some(other) => bail!("unsupported ease {other}"),
    };
    Ok(FloatTrack { keyframes, bezier })
}

fn parse_color_track(v: &Value) -> Result<ColorTrack> {
    let kfs = v["keyframes"]
        .as_array()
        .context("color track: no keyframes")?;
    let mut keyframes = Vec::with_capacity(kfs.len());
    for kf in kfs {
        let ticks = kf["ticks"].as_i64().context("keyframe: no ticks")? as i32;
        keyframes.push((ticks, parse_color(&kf["value"])?));
    }
    Ok(ColorTrack { keyframes })
}

fn track<'a>(tracks: &'a Value, day: &str, name: &str) -> Result<&'a Value> {
    tracks
        .get(&format!("minecraft:{name}"))
        .with_context(|| format!("{day}: no track minecraft:{name}"))
}

pub fn generate<R: std::io::Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
) -> Result<SkyData> {
    let day: Value = serde_json::from_slice(
        &read_entry(archive, "data/minecraft/timeline/day.json")
            .context("timeline day.json")?,
    )?;
    let moon: Value = serde_json::from_slice(
        &read_entry(archive, "data/minecraft/timeline/moon.json")
            .context("timeline moon.json")?,
    )?;
    let dim: Value = serde_json::from_slice(
        &read_entry(archive, "data/minecraft/dimension_type/overworld.json")
            .context("dimension_type overworld.json")?,
    )?;

    let day_period = day["period_ticks"].as_i64().context("day period_ticks")? as i32;
    let moon_period = moon["period_ticks"].as_i64().context("moon period_ticks")? as i32;
    let dt = &day["tracks"];
    let mt = &moon["tracks"];

    let sun_angle = parse_float_track(track(dt, "day.json", "visual/sun_angle")?)?;
    let moon_angle = parse_float_track(track(dt, "day.json", "visual/moon_angle")?)?;
    let star_angle = parse_float_track(track(dt, "day.json", "visual/star_angle")?)?;
    let star_brightness = parse_float_track(track(dt, "day.json", "visual/star_brightness")?)?;
    let sky_color = parse_color_track(track(dt, "day.json", "visual/sky_color")?)?;
    let fog_color = parse_color_track(track(dt, "day.json", "visual/fog_color")?)?;
    let sunrise_color = parse_color_track(track(dt, "day.json", "visual/sunrise_sunset_color")?);

    let sunrise_color = match sunrise_color {
        Ok(c) => c,
        Err(e) => bail!("day.json sunrise_sunset_color: {e}"),
    };

    let attrs = &dim["attributes"];
    let dim_sky = attrs
        .get("minecraft:visual/sky_color")
        .context("dimension overworld: visual/sky_color")?;
    let dim_fog = attrs
        .get("minecraft:visual/fog_color")
        .context("dimension overworld: visual/fog_color")?;

    // Moon phases: keyframes sorted by tick define the phase order/indices.
    let phase_kfs = track(mt, "moon.json", "visual/moon_phase")?["keyframes"]
        .as_array()
        .context("moon.json: no moon_phase keyframes")?
        .clone();
    let mut phase_kf: Vec<(i32, u8)> = Vec::with_capacity(phase_kfs.len());
    let mut moon_phase_names: Vec<String> = Vec::with_capacity(phase_kfs.len());
    for kf in &phase_kfs {
        let ticks = kf["ticks"].as_i64().context("phase: no ticks")? as i32;
        let name = kf["value"]
            .as_str()
            .context("phase: no name")?
            .to_string();
        let idx = moon_phase_names.iter().position(|n| *n == name);
        let idx = match idx {
            Some(i) => i as u8,
            None => {
                moon_phase_names.push(name.clone());
                (moon_phase_names.len() - 1) as u8
            }
        };
        phase_kf.push((ticks, idx));
    }

    // Real celestial textures.
    let sun_bytes = read_entry(
        archive,
        "assets/minecraft/textures/environment/celestial/sun.png",
    )
    .context("celestial sun.png")?;
    let sun_rgba = decode_rgba(&sun_bytes)?;
    let mut moon_rgba = Vec::with_capacity(moon_phase_names.len());
    for name in &moon_phase_names {
        let bytes = read_entry(
            archive,
            &format!("assets/minecraft/textures/environment/celestial/moon/{name}.png"),
        )
        .with_context(|| format!("celestial moon/{name}.png"))?;
        moon_rgba.push(decode_rgba(&bytes)?);
    }

    Ok(SkyData {
        day_period,
        moon_period,
        sun_angle,
        moon_angle,
        star_angle,
        star_brightness,
        sky_color,
        fog_color,
        sunrise_color,
        dim_sky_color: parse_color(dim_sky)?,
        dim_fog_color: parse_color(dim_fog)?,
        moon_phase_names,
        moon_phase_kf: phase_kf,
        sun_rgba,
        moon_rgba,
    })
}

fn f32_lit(v: f32) -> String {
    // Round-trip-safe f32 literal emission (Debug for floats).
    format!("{v:?}")
}

/// Emit `generated/sky.rs`.
pub fn emit(data: &SkyData) -> Result<String> {
    let mut out = String::with_capacity(256 << 10);
    out.push_str(&format!(
        "//! GENERATED by mc2rs-gen from official 26.x jar data — DO NOT EDIT BY HAND.\n\
         //! Sources: data/minecraft/timeline/day.json, data/minecraft/timeline/moon.json,\n\
         //! data/minecraft/dimension_type/overworld.json,\n\
         //! assets/minecraft/textures/environment/celestial/*.png\n\
         //! Angle tracks are cubic-bezier eased (the game's 26.x day arc);\n\
         //! star brightness and the color tracks are LINEAR with modifiers\n\
         //! maximum / multiply respectively (evaluated by mc-client/src/sky.rs).\n\n\
         pub const DAY_PERIOD_TICKS: i32 = {};\n\
         pub const MOON_PERIOD_TICKS: i32 = {};\n\
         /// Celestial textures are square (32x32 in 26.2).\n\
         pub const CELESTIAL_SIZE: u32 = {};\n\n\
         /// Overworld dimension base visual/sky_color and visual/fog_color (RGBA).\n\
         pub static DIM_SKY_COLOR: [u8; 4] = {:?};\n\
         pub static DIM_FOG_COLOR: [u8; 4] = {:?};\n\n",
        data.day_period,
        data.moon_period,
        data.sun_rgba.0,
        data.dim_sky_color,
        data.dim_fog_color,
    ));

    for (label, tr, suffix) in [
        ("SUN_ANGLE", &data.sun_angle, Some(1usize)),
        ("MOON_ANGLE", &data.moon_angle, Some(2)),
        ("STAR_ANGLE", &data.star_angle, Some(3)),
    ] {
        let _ = suffix;
        out.push_str(&format!(
            "/// visual/{} keyframes (tick, degrees). The doubled tick marks the\n\
             /// official wrap pair; segments wrap over DAY_PERIOD_TICKS.\n\
             pub static {label}_KF: [(i32, f32); {}] = [",
            label.to_lowercase(),
            tr.keyframes.len(),
        ));
        for (t, v) in &tr.keyframes {
            out.push_str(&format!("({}, {}), ", t, f32_lit(*v)));
        }
        out.push_str("];\n");
        match &tr.bezier {
            Some(b) => out.push_str(&format!(
                "pub static {label}_BEZIER: [f32; 4] = [{}, {}, {}, {}];\n\n",
                f32_lit(b[0]),
                f32_lit(b[1]),
                f32_lit(b[2]),
                f32_lit(b[3]),
            )),
            None => out.push('\n'),
        }
    }

    out.push_str(&format!(
        "/// visual/star_brightness keyframes (tick, 0..1) — LINEAR, modifier maximum.\n\
         pub static STAR_BRIGHTNESS_KF: [(i32, f32); {}] = [",
        data.star_brightness.keyframes.len()
    ));
    for (t, v) in &data.star_brightness.keyframes {
        out.push_str(&format!("({}, {}), ", t, f32_lit(*v)));
    }
    out.push_str("];\n\n");

    for (label, tr) in [
        ("SKY_COLOR", &data.sky_color),
        ("FOG_COLOR", &data.fog_color),
        ("SUNRISE_COLOR", &data.sunrise_color),
    ] {
        out.push_str(&format!(
            "/// visual/{} keyframes (tick, RGBA) — LINEAR (sunrise_sunset_color\n\
             /// tints the official horizon glow disc).\n\
             pub static {label}_KF: [(i32, [u8; 4]); {}] = [",
            label.to_lowercase(),
            tr.keyframes.len()
        ));
        for (t, c) in &tr.keyframes {
            out.push_str(&format!("({}, {:?}), ", t, c));
        }
        out.push_str("];\n\n");
    }

    out.push_str(&format!(
        "/// visual/moon_phase keyframes (tick, phase index) over MOON_PERIOD_TICKS.\n\
         pub static MOON_PHASE_KF: [(i32, u8); {}] = [",
        data.moon_phase_kf.len()
    ));
    for (t, p) in &data.moon_phase_kf {
        out.push_str(&format!("({}, {}), ", t, p));
    }
    out.push_str("];\n");
    out.push_str("/// Phase texture names in MoonPhase.ordinal order (index == quad index).\n");
    out.push_str(&format!(
        "pub static MOON_PHASE_NAMES: [&str; {}] = [",
        data.moon_phase_names.len()
    ));
    for n in &data.moon_phase_names {
        out.push_str(&format!("\"{n}\", "));
    }
    out.push_str("];\n\n");

    let (w, h, sun) = &data.sun_rgba;
    out.push_str(&format!(
        "/// environment/celestial/sun.png ({w}x{h} RGBA).\n\
         pub static SUN_RGBA: [u8; {}] = [",
        sun.len()
    ));
    for b in sun.iter() {
        out.push_str(&format!("{b}, "));
    }
    out.push_str("];\n");

    out.push_str(&format!(
        "/// environment/celestial/moon/<phase>.png per MOON_PHASE_NAMES index.\n\
         pub static MOON_RGBA: [[u8; {}]; {}] = [\n",
        sun.len(),
        data.moon_rgba.len()
    ));
    for (_, _, tex) in &data.moon_rgba {
        out.push_str("    [");
        for b in tex.iter() {
            out.push_str(&format!("{b}, "));
        }
        out.push_str("],\n");
    }
    out.push_str("];\n");

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_hex_and_int_colors() {
        assert_eq!(parse_color(&json!("#78a7ff")).unwrap(), [0x78, 0xa7, 0xff, 0xFF]);
        assert_eq!(parse_color(&json!("#5fefa333")).unwrap(), [0xef, 0xa3, 0x33, 0x5f]);
        // -15132378 == 0xFF191926 as ARGB
        assert_eq!(parse_color(&json!(-15132378i64)).unwrap(), [0x19, 0x19, 0x26, 0xFF]);
    }

    #[test]
    fn parses_bezier_and_linear_tracks() {
        let t = json!({
            "ease": {"cubic_bezier": [0.362, 0.241, 0.638, 0.759]},
            "keyframes": [{"ticks": 6000, "value": 360.0}, {"ticks": 6000, "value": 0.0}]
        });
        let tr = parse_float_track(&t).unwrap();
        assert_eq!(tr.keyframes, vec![(6000, 360.0), (6000, 0.0)]);
        assert_eq!(tr.bezier, Some([0.362, 0.241, 0.638, 0.759]));
        let lin = json!({"keyframes": [{"ticks": 0, "value": 0.5}]});
        assert_eq!(parse_float_track(&lin).unwrap().bezier, None);
    }

    #[test]
    fn missing_track_is_an_error() {
        let tracks = json!({"minecraft:visual/sky_color": {"keyframes": []}});
        assert!(track(&tracks, "day.json", "visual/sun_angle").is_err());
        assert!(track(&tracks, "day.json", "visual/sky_color").is_ok());
    }
}

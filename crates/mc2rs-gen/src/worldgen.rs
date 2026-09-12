//! Worldgen generation: parses the official `data/minecraft/worldgen/` JSON
//! resources (biomes with real climate/colors, noise settings, density
//! functions, features) into Rust tables — the client's world is driven by
//! the same data the Java game loads, nothing hand-invented.

use std::collections::BTreeMap;

use anyhow::Result;
use serde_json::Value;

use crate::extract::read_entry;

/// One entry of the official biome background-music data: the 26.x
/// `attributes["minecraft:audio/background_music"]` object of one biome
/// maps situational contexts ("default" | "creative" | "underwater") to a
/// music event + real delay bounds in game ticks.
#[derive(Debug, Clone)]
pub struct BiomeMusicEntry {
    pub biome: String,
    pub context: String,
    /// bare event id ("minecraft:" prefix stripped, matches the sounds table)
    pub event: String,
    pub min_delay: u32,
    pub max_delay: u32,
}

#[derive(Debug, Clone)]
pub struct BiomeDef {
    pub name: String,
    pub has_precipitation: bool,
    pub temperature: f32,
    pub downfall: f32,
    pub sky_color: u32,
    pub fog_color: u32,
    pub water_color: u32,
    pub water_fog_color: u32,
    /// optional overrides present in the biome JSON
    pub grass_color: Option<u32>,
    pub foliage_color: Option<u32>,
}

#[derive(Debug, Default)]
pub struct WorldgenData {
    pub biomes: BTreeMap<String, BiomeDef>,
    /// situational background music per biome, straight from the official
    /// biome JSONs (empty for versions without the attribute)
    pub biome_music: Vec<BiomeMusicEntry>,
    pub noise_settings: Vec<String>,
    pub density_functions: usize,
    pub noises: usize,
    pub configured_features: usize,
    pub placed_features: usize,
}

fn hex_color(s: &str) -> Option<u32> {
    let s = s.strip_prefix('#')?;
    u32::from_str_radix(s, 16).ok()
}

/// Biome colors are integers in vanilla JSON; some packs use "#rrggbb".
fn color_value(v: Option<&Value>) -> Option<u32> {
    match v {
        Some(Value::Number(n)) => n.as_u64().map(|x| x as u32),
        Some(Value::String(s)) => hex_color(s),
        _ => None,
    }
}

pub fn generate<R: std::io::Read + std::io::Seek>(archive: &mut zip::ZipArchive<R>) -> Result<WorldgenData> {
    let mut data = WorldgenData::default();
    let names: Vec<String> = archive
        .file_names()
        .filter(|n| n.starts_with("data/minecraft/worldgen/"))
        .map(String::from)
        .collect();
    for name in names {
        if let Some(rest) = name.strip_prefix("data/minecraft/worldgen/biome/") {
            if rest.ends_with(".json") {
                let bytes = read_entry(archive, &name)?;
                if let Ok(v) = serde_json::from_slice::<Value>(&bytes) {
                    let id = rest.trim_end_matches(".json");
                    let fx = |k: &str, d: f32| v.get(k).and_then(Value::as_f64).unwrap_or(d as f64) as f32;
                    // 26.x layout: display colors moved to attributes["minecraft:visual/*"]
                    // as "#rrggbb" strings; classic effects.* integers still accepted.
                    let attrs = || v.get("attributes").and_then(Value::as_object);
                    let attr_color = |key: &str| {
                        attrs()
                            .and_then(|a| a.get(key))
                            .and_then(|val| val.as_str())
                            .and_then(hex_color)
                    };
                    let ax = |key: &str, k: &str| {
                        attr_color(key).or_else(|| color_value(v.get("effects").and_then(|e| e.get(k)))).unwrap_or(0)
                    };
                    // 26.x situational music moved into attributes:
                    // {"default": {"sound": "minecraft:music.overworld…",
                    //  "min_delay": 12000, "max_delay": 24000}, …}
                    if let Some(music) = v
                        .get("attributes")
                        .and_then(|a| a.get("minecraft:audio/background_music"))
                        .and_then(Value::as_object)
                    {
                        for (ctx, spec) in music {
                            let Some(sound) = spec.get("sound").and_then(Value::as_str) else {
                                continue;
                            };
                            data.biome_music.push(BiomeMusicEntry {
                                biome: format!("minecraft:{id}"),
                                context: ctx.clone(),
                                event: sound.strip_prefix("minecraft:").unwrap_or(sound).to_string(),
                                min_delay: spec.get("min_delay").and_then(Value::as_u64).unwrap_or(0) as u32,
                                max_delay: spec.get("max_delay").and_then(Value::as_u64).unwrap_or(0) as u32,
                            });
                        }
                    }
                    data.biomes.insert(
                        id.to_string(),
                        BiomeDef {
                            name: format!("minecraft:{id}"),
                            has_precipitation: v.get("has_precipitation").and_then(Value::as_bool).unwrap_or(true),
                            temperature: fx("temperature", 0.5),
                            downfall: fx("downfall", 0.5),
                            sky_color: ax("minecraft:visual/sky_color", "sky_color"),
                            fog_color: ax("minecraft:visual/fog_color", "fog_color"),
                            water_color: color_value(v.get("effects").and_then(|e| e.get("water_color"))).unwrap_or(0),
                            water_fog_color: color_value(v.get("effects").and_then(|e| e.get("water_fog_color"))).unwrap_or(0),
                            grass_color: color_value(
                                v.get("effects").and_then(|e| e.get("grass_color")),
                            ),
                            foliage_color: color_value(
                                v.get("effects").and_then(|e| e.get("foliage_color")),
                            ),
                        },
                    );
                }
            }
        } else if let Some(rest) = name.strip_prefix("data/minecraft/worldgen/noise_settings/") {
            if rest.ends_with(".json") {
                data.noise_settings.push(rest.trim_end_matches(".json").to_string());
            }
        } else if name.starts_with("data/minecraft/worldgen/density_function/") && name.ends_with(".json") {
            data.density_functions += 1;
        } else if name.starts_with("data/minecraft/worldgen/noise/") && name.ends_with(".json") {
            data.noises += 1;
        } else if name.starts_with("data/minecraft/worldgen/configured_feature/") && name.ends_with(".json") {
            data.configured_features += 1;
        } else if name.starts_with("data/minecraft/worldgen/placed_feature/") && name.ends_with(".json") {
            data.placed_features += 1;
        }
    }
    data.noise_settings.sort();
    // deterministic emit order (and a lookup-ready table): biome, then context
    data.biome_music
        .sort_by(|a, b| (&a.biome, &a.context).cmp(&(&b.biome, &b.context)));
    Ok(data)
}

fn rust_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn emit(data: &WorldgenData) -> Result<String> {
    let mut out = String::with_capacity(128 << 10);
    out.push_str("//! GENERATED by mc2rs-gen from data/minecraft/worldgen/ of the official jar.\n");
    out.push_str("//! Real biome climates/colors and worldgen resource inventory.\n\n");

    out.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\npub struct BiomeDef {\n    pub name: &'static str,\n    pub has_precipitation: bool,\n    pub temperature: f32,\n    pub downfall: f32,\n    pub sky_color: u32,\n    pub fog_color: u32,\n    pub water_color: u32,\n    pub water_fog_color: u32,\n    pub grass_color: Option<u32>,\n    pub foliage_color: Option<u32>,\n}\n\n");

    out.push_str("pub static BIOMES: [BiomeDef; ");
    out.push_str(&data.biomes.len().to_string());
    out.push_str("] = [\n");
    for b in data.biomes.values() {
        let gc = |c: &Option<u32>| match c {
            Some(v) => format!("Some({v})"),
            None => "None".to_string(),
        };
        out.push_str(&format!(
            "    BiomeDef {{ name: \"{}\", has_precipitation: {}, temperature: {:?}, downfall: {:?}, sky_color: {}, fog_color: {}, water_color: {}, water_fog_color: {}, grass_color: {}, foliage_color: {} }},\n",
            rust_str(&b.name),
            b.has_precipitation,
            b.temperature,
            b.downfall,
            b.sky_color,
            b.fog_color,
            b.water_color,
            b.water_fog_color,
            gc(&b.grass_color),
            gc(&b.foliage_color),
        ));
    }
    out.push_str("];\n\n");

    out.push_str("/// Biome by registry id (\"minecraft:plains\").\n");
    out.push_str("pub fn biome(name: &str) -> Option<&'static BiomeDef> {\n");
    out.push_str("    BIOMES.binary_search_by_key(&name, |b| b.name).ok().map(|i| &BIOMES[i])\n}\n\n");

    out.push_str("/// One situational background-music entry from the official biome data\n");
    out.push_str("/// (attributes[\"minecraft:audio/background_music\"]).\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\npub struct BiomeMusicDef {\n    pub biome: &'static str,\n    pub context: &'static str,\n    pub event: &'static str,\n    pub min_delay: u32,\n    pub max_delay: u32,\n}\n\n");
    out.push_str("pub static BIOME_MUSIC: [BiomeMusicDef; ");
    out.push_str(&data.biome_music.len().to_string());
    out.push_str("] = [\n");
    for m in &data.biome_music {
        out.push_str(&format!(
            "    BiomeMusicDef {{ biome: \"{}\", context: \"{}\", event: \"{}\", min_delay: {}, max_delay: {} }},\n",
            rust_str(&m.biome),
            rust_str(&m.context),
            rust_str(&m.event),
            m.min_delay,
            m.max_delay
        ));
    }
    out.push_str("];\n\n");
    out.push_str("/// Situational music of one biome context (\"minecraft:plains\", \"default\"),\n");
    out.push_str("/// sorted for binary search. None = the official biome ships no music for\n");
    out.push_str("/// that context (the client falls back to the vanilla in-game default).\n");
    out.push_str("pub fn biome_music(biome: &str, context: &str) -> Option<&'static BiomeMusicDef> {\n");
    out.push_str("    BIOME_MUSIC\n        .binary_search_by_key(&(biome, context), |m| (m.biome, m.context))\n        .ok()\n        .map(|i| &BIOME_MUSIC[i])\n}\n\n");

    out.push_str("pub static NOISE_SETTINGS: [&str; ");
    out.push_str(&data.noise_settings.len().to_string());
    out.push_str("] = [");
    for (i, n) in data.noise_settings.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("\"{}\"", rust_str(n)));
    }
    out.push_str("];\n\n");

    out.push_str(&format!(
        "/// Worldgen resource counts (real inventory of the version jar).\npub const DENSITY_FUNCTIONS: usize = {};\npub const NOISES: usize = {};\npub const CONFIGURED_FEATURES: usize = {};\npub const PLACED_FEATURES: usize = {};\n",
        data.density_functions, data.noises, data.configured_features, data.placed_features
    ));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zip_of(entries: &[(&str, Vec<u8>)]) -> zip::ZipArchive<std::io::Cursor<Vec<u8>>> {
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut w = zip::ZipWriter::new(&mut buf);
            let opts = zip::write::SimpleFileOptions::default();
            for (name, data) in entries {
                w.start_file(*name, opts).unwrap();
                std::io::Write::write_all(&mut w, data).unwrap();
            }
            w.finish().unwrap();
        }
        zip::ZipArchive::new(buf).unwrap()
    }

    #[test]
    fn parses_biome_colors_and_counts() {
        let plains = br#"{"temperature":0.8,"downfall":0.4,"has_precipitation":true,"attributes":{"minecraft:audio/background_music":{"default":{"sound":"minecraft:music.game","min_delay":12000,"max_delay":24000},"underwater":{"sound":"minecraft:music.under_water","min_delay":12000,"max_delay":24000}}},"effects":{"fog_color":12638463,"sky_color":7946167,"water_color":4159204,"water_fog_color":329011}}"#;
        let desert = br#"{"temperature":2.0,"downfall":0.0,"has_precipitation":false,"attributes":{"minecraft:audio/background_music":{"default":{"sound":"minecraft:music.overworld.desert","min_delay":12000,"max_delay":24000}}},"effects":{"fog_color":12638463,"sky_color":7946167,"water_color":4159204,"water_fog_color":329011,"grass_color":11590961}}"#;
        let mut zip = zip_of(&[
            ("data/minecraft/worldgen/biome/plains.json", plains.to_vec()),
            ("data/minecraft/worldgen/biome/desert.json", desert.to_vec()),
            ("data/minecraft/worldgen/noise_settings/overworld.json", b"{}".to_vec()),
            ("data/minecraft/worldgen/density_function/overworld/depth.json", b"{}".to_vec()),
            ("data/minecraft/worldgen/noise/jagged.json", b"{}".to_vec()),
            ("data/minecraft/worldgen/configured_feature/oak.json", b"{}".to_vec()),
            ("data/minecraft/worldgen/placed_feature/trees_plains.json", b"{}".to_vec()),
        ]);
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.biomes.len(), 2);
        let p = &data.biomes["plains"];
        assert!((p.temperature - 0.8).abs() < 1e-6);
        assert!(p.has_precipitation);
        assert_eq!(p.sky_color, 7946167);
        let d = &data.biomes["desert"];
        assert!(!d.has_precipitation);
        assert_eq!(d.grass_color, Some(11590961));
        assert_eq!(data.noise_settings, vec!["overworld"]);
        assert_eq!(data.density_functions, 1);
        assert_eq!(data.noises, 1);
        assert_eq!((data.configured_features, data.placed_features), (1, 1));

        let emitted = emit(&data).unwrap();
        assert!(emitted.contains("name: \"minecraft:plains\""));
        assert!(emitted.contains("pub fn biome(name: &str)"));
        assert!(emitted.contains("pub const PLACED_FEATURES: usize = 1;"));
        // the situational music table: 3 entries, sorted, contexts kept,
        // "minecraft:" stripped from the event id
        assert_eq!(data.biome_music.len(), 3);
        assert_eq!(data.biome_music[0].biome, "minecraft:desert");
        assert_eq!(data.biome_music[0].event, "music.overworld.desert");
        assert_eq!((data.biome_music[0].min_delay, data.biome_music[0].max_delay), (12000, 24000));
        assert!(emitted.contains("event: \"music.under_water\""));
        assert!(emitted.contains("pub fn biome_music(biome: &str, context: &str)"));
        assert!(emitted.contains("pub static BIOME_MUSIC: [BiomeMusicDef; 3]"));
    }
}

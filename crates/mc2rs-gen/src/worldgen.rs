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

/// The `DimensionType$Skybox` string-representable — the vanilla enum
/// ("none" | "overworld" | "end") the client's sky pipeline gates on
/// (SkyRenderer.extractRenderState: NONE returns before ANY sky state is
/// extracted; END extracts only the end-flash triple).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skybox {
    None,
    Overworld,
    End,
}

/// One built-in `minecraft:dimension_type` entry: the jar's
/// `data/minecraft/dimension_type/*.json` — the world layout (min_y/height)
/// and environment (skylight, fixed time, ambient light) a server's synced
/// dimension registry mirrors per element.
#[derive(Debug, Clone)]
pub struct DimensionTypeEntry {
    pub name: String,
    pub min_y: i32,
    pub height: u32,
    pub logical_height: u32,
    pub has_skylight: bool,
    pub has_fixed_time: bool,
    pub ambient_light: f32,
    pub coordinate_scale: f64,
    /// the `timelines` reference — a tag ("#minecraft:in_overworld") in the
    /// real jar; inline list references are not emitted (none exist there)
    pub timelines: Option<String>,
    pub default_clock: Option<String>,
    /// `attributes/minecraft:visual/ambient_light_color` — the lightmap's    /// AmbientColor base term (the shader's `max(AmbientColor,
    /// nightVisionColor)` starting color). ABSENT = the bytecode default
    /// (`EnvironmentAttributes.<clinit>`: `defaultValue(-16777216)` =
    /// 0xFF000000): black. The real jar states it per dimension
    /// (overworld #0a0a0a, nether #302821, end #3f473f).
    pub ambient_light_color: [u8; 3],
    /// the `skybox` field — the vanilla codec is
    /// `optionalFieldOf("skybox", OVERWORLD)` (the DimensionType
    /// bytecode): an ABSENT key is the Overworld default (the real jar:
    /// the overworld pair states nothing, the end states "end", the
    /// nether states "none"). An unknown string would have the vanilla
    /// codec reject the entry outright (StringRepresentable.fromEnum),
    /// so no vanilla built-in can carry one — the parse falls back to
    /// the codec default.
    pub skybox: Skybox,
    /// The STATED fog distances (visual/fog_start_distance /
    /// visual/fog_end_distance, blocks) — `None` when the JSON states
    /// none, which is the bytecode defaults' shape (start 0.0 / end
    /// 1024.0, javap EnvironmentAttributes): only the nether states the
    /// pair in the real jar (10.0 / 96.0 — the fogged walls).
    pub fog_start_distance: Option<f32>,
    pub fog_end_distance: Option<f32>,
    /// The STATED cloud-fog end (visual/cloud_fog_end_distance, blocks)
    /// — `None` when the JSON states none, the bytecode default's shape
    /// (2048.0, javap EnvironmentAttributes): NO vanilla dimension
    /// declares it, so the field rides only a custom server's statement.
    pub cloud_fog_end_distance: Option<f32>,
    /// The STATED block-light tint (visual/block_light_tint) — `None`
    /// when the JSON states none, the bytecode default's shape
    /// (-10100 = 0xFFFFD88C = #FFD88C, the warm torch cast, javap
    /// EnvironmentAttributes): NO vanilla dimension declares it, so the
    /// field rides only a custom server's statement.
    pub block_light_tint: Option<[u8; 3]>,
}

/// The per-biome tint extras the jar states beyond the classic color
/// surface: `effects.grass_color_modifier` and `effects.dry_foliage_color`
/// (both OPTIONAL in the biome JSON — sparse, only biomes that state at
/// least one get an entry). The modifier is the vanilla enum name verbatim
/// ("dark_forest" | "swamp"; "none" states the absence and is dropped);
/// the dry-foliage color is the override `Biome.getDryFoliageColor()`
/// orElseGets before the climate grid.
#[derive(Debug, Clone)]
pub struct BiomeTintExtraEntry {
    pub biome: String,
    pub grass_color_modifier: Option<String>,
    pub dry_foliage_color: Option<u32>,
}

/// One built-in `minecraft:timeline` entry: the jar's
/// `data/minecraft/timeline/*.json` — a keyframe curve set over a world
/// clock. The per-track data the CLIENT consumes today is the
/// `minecraft:visual/sky_light_factor` render track (the SkyFactor the
/// lightmap multiplies the sky term by); every other track rides the count.
#[derive(Debug, Clone)]
pub struct TimelineEntry {
    pub name: String,
    pub clock: String,
    pub period_ticks: Option<i64>,
    pub tracks: usize,
    /// (tick, value) pairs of `minecraft:visual/sky_light_factor`, sorted by
    /// tick — empty when the timeline carries no such track
    pub sky_light_factor: Vec<(i64, f32)>,
}

#[derive(Debug, Default)]
pub struct WorldgenData {
    pub biomes: BTreeMap<String, BiomeDef>,
    /// situational background music per biome, straight from the official
    /// biome JSONs (empty for versions without the attribute)
    pub biome_music: Vec<BiomeMusicEntry>,
    /// the built-in `minecraft:dimension_type` registry entries, sorted by id
    pub dimension_types: BTreeMap<String, DimensionTypeEntry>,
    /// the built-in `minecraft:timeline` registry entries, sorted by id
    pub timelines: BTreeMap<String, TimelineEntry>,
    /// `data/minecraft/tags/timeline/*.json` verbatim (name -> values; the
    /// values may be nested "#tag" references)
    pub timeline_tags: BTreeMap<String, Vec<String>>,
    /// `data/minecraft/tags/worldgen/biome/*.json` verbatim (name -> values;
    /// the values mix concrete biome ids and nested "#tag" references —
    /// the SAME raw form the timeline tags use)
    pub biome_tags: BTreeMap<String, Vec<String>>,
    /// per-biome tint extras (grass_color_modifier / dry_foliage_color),
    /// sparse — only the biomes whose JSON states at least one
    pub biome_tint_extras: BTreeMap<String, BiomeTintExtraEntry>,
    /// the built-in `minecraft:world_clock` registry names (id -> full
    /// name); the 26.2 JSONs are EMPTY objects — the names are the data
    pub world_clocks: BTreeMap<String, String>,
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
    // the whole data/minecraft/ tree: the worldgen/ side AND the built-in
    // registry JSONs beside it (dimension_type/, timeline/, tags/timeline/) —
    // every entry is classified by the strict-prefix branches below
    let names: Vec<String> = archive
        .file_names()
        .filter(|n| n.starts_with("data/minecraft/"))
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
                    // the tint extras: the modifier enum name ("none" IS the
                    // absence — dropped) and the dry-foliage override (the
                    // "#rrggbb" string form the real jar uses). A modifier the
                    // vanilla enum does not define is a REFUSAL — guessing a
                    // shape the jar does not state would put a lie in the table.
                    let modifier = v
                        .get("effects")
                        .and_then(|e| e.get("grass_color_modifier"))
                        .and_then(Value::as_str);
                    let modifier = match modifier {
                        Some("none") | None => None,
                        Some(m @ ("dark_forest" | "swamp")) => Some(m.to_string()),
                        Some(other) => {
                            anyhow::bail!(
                                "biome {id}: unknown grass_color_modifier {other:?} \
                                 (the vanilla enum states none/dark_forest/swamp)"
                            );
                        }
                    };
                    let dry = color_value(v.get("effects").and_then(|e| e.get("dry_foliage_color")));
                    if modifier.is_some() || dry.is_some() {
                        data.biome_tint_extras.insert(
                            id.to_string(),
                            BiomeTintExtraEntry {
                                biome: format!("minecraft:{id}"),
                                grass_color_modifier: modifier,
                                dry_foliage_color: dry,
                            },
                        );
                    }
                }
            }
        } else if let Some(rest) = name.strip_prefix("data/minecraft/world_clock/") {
            if rest.ends_with(".json") {
                let bytes = read_entry(archive, &name)?;
                if let Ok(v) = serde_json::from_slice::<Value>(&bytes) {
                    let id = rest.trim_end_matches(".json").to_string();
                    // the 26.2 world_clock JSONs are EMPTY objects — the
                    // clock behavior lives code-side, the names are the
                    // data. A stated field would be DROPPED data, so a
                    // non-empty payload is a refusal until the parser
                    // grows the fields consciously.
                    if !v.as_object().map_or(false, |o| o.is_empty()) {
                        anyhow::bail!(
                            "world_clock {id}: unexpected payload (the 26.2 jar \
                             states empty objects — extend the parser first)"
                        );
                    }
                    let name = format!("minecraft:{id}");
                    data.world_clocks.insert(id, name);
                }
            }
        } else if let Some(rest) = name.strip_prefix("data/minecraft/dimension_type/") {
            if rest.ends_with(".json") {
                let bytes = read_entry(archive, &name)?;
                if let Ok(v) = serde_json::from_slice::<Value>(&bytes) {
                    let id = rest.trim_end_matches(".json").to_string();
                    let height = v.get("height").and_then(Value::as_u64).unwrap_or(384) as u32;
                    data.dimension_types.insert(
                        id.clone(),
                        DimensionTypeEntry {
                            name: format!("minecraft:{id}"),
                            min_y: v.get("min_y").and_then(Value::as_i64).unwrap_or(0) as i32,
                            height,
                            logical_height: v
                                .get("logical_height")
                                .and_then(Value::as_u64)
                                .unwrap_or(height as u64) as u32,
                            has_skylight: v.get("has_skylight").and_then(Value::as_bool).unwrap_or(true),
                            has_fixed_time: v.get("has_fixed_time").and_then(Value::as_bool).unwrap_or(false),
                            ambient_light: v
                                .get("ambient_light")
                                .and_then(Value::as_f64)
                                .unwrap_or(0.0) as f32,
                            coordinate_scale: v
                                .get("coordinate_scale")
                                .and_then(Value::as_f64)
                                .unwrap_or(1.0),
                            // the lightmap's AmbientColor base: the dimension
                            // JSON's stated attribute, else the bytecode
                            // default (black — the absent-attribute shape).
                            // The attributes compound is FLAT — the keys are
                            // the full namespaced paths verbatim
                            // ("minecraft:visual/ambient_light_color")
                            ambient_light_color: v
                                .get("attributes")
                                .and_then(|a| a.get("minecraft:visual/ambient_light_color"))
                                .and_then(Value::as_str)
                                .and_then(hex_color)
                                .map(|c| [(c >> 16) as u8, (c >> 8) as u8, c as u8])
                                .unwrap_or([0, 0, 0]),
                            // the ABSENT key is the codec default OVERWORLD
                            // (optionalFieldOf — the DimensionType bytecode);
                            // an unknown string the vanilla codec would
                            // reject outright, so none can ride real data
                            skybox: match v.get("skybox").and_then(Value::as_str) {
                                Some("none") => Skybox::None,
                                Some("end") => Skybox::End,
                                _ => Skybox::Overworld,
                            },
                            // the STATED fog distances (blocks): only the
                            // nether states the pair in the real jar; the
                            // absent shape reads None (the bytecode defaults
                            // start 0.0 / end 1024.0 — the consumer's
                            // documented window treatment covers those)
                            fog_start_distance: v
                                .get("attributes")
                                .and_then(|a| a.get("minecraft:visual/fog_start_distance"))
                                .and_then(Value::as_f64)
                                .map(|f| f as f32),
                            fog_end_distance: v
                                .get("attributes")
                                .and_then(|a| a.get("minecraft:visual/fog_end_distance"))
                                .and_then(Value::as_f64)
                                .map(|f| f as f32),
                            // the STATED cloud-fog end (blocks): a single
                            // syncable FLOAT — the bytecode default 2048.0
                            // when the JSON states none (no vanilla
                            // dimension declares it)
                            cloud_fog_end_distance: v
                                .get("attributes")
                                .and_then(|a| a.get("minecraft:visual/cloud_fog_end_distance"))
                                .and_then(Value::as_f64)
                                .map(|f| f as f32),
                            // the STATED block-light tint: a syncable
                            // RGB_COLOR attribute (the "#rrggbb" string);
                            // the bytecode default -10100 = #FFD88C when
                            // the JSON states none
                            block_light_tint: v
                                .get("attributes")
                                .and_then(|a| a.get("minecraft:visual/block_light_tint"))
                                .and_then(Value::as_str)
                                .and_then(hex_color)
                                .map(|c| [(c >> 16) as u8, (c >> 8) as u8, c as u8]),
                            // string tag references only — the inline list
                            // form does not occur in the real jar
                            timelines: v
                                .get("timelines")
                                .and_then(Value::as_str)
                                .map(String::from),
                            default_clock: v
                                .get("default_clock")
                                .and_then(Value::as_str)
                                .map(String::from),
                        },
                    );
                }
            }
        } else if let Some(rest) = name.strip_prefix("data/minecraft/timeline/") {
            if rest.ends_with(".json") {
                let bytes = read_entry(archive, &name)?;
                if let Ok(v) = serde_json::from_slice::<Value>(&bytes) {
                    let id = rest.trim_end_matches(".json").to_string();
                    let tracks = v
                        .get("tracks")
                        .and_then(Value::as_object)
                        .map(|t| t.len())
                        .unwrap_or(0);
                    let mut sky_light_factor: Vec<(i64, f32)> = v
                        .get("tracks")
                        .and_then(|t| t.get("minecraft:visual/sky_light_factor"))
                        .and_then(|t| t.get("keyframes"))
                        .and_then(Value::as_array)
                        .map(|kfs| {
                            kfs.iter()
                                .filter_map(|kf| {
                                    let t = kf.get("ticks")?.as_i64()?;
                                    let val = kf.get("value")?.as_f64()?;
                                    Some((t, val as f32))
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    sky_light_factor.sort_by_key(|(t, _)| *t);
                    let entry = TimelineEntry {
                        name: format!("minecraft:{id}"),
                        clock: v
                            .get("clock")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string(),
                        period_ticks: v.get("period_ticks").and_then(Value::as_i64),
                        tracks,
                        sky_light_factor,
                    };
                    data.timelines.insert(id, entry);
                }
            }
        } else if let Some(rest) = name.strip_prefix("data/minecraft/tags/timeline/") {
            if rest.ends_with(".json") {
                let bytes = read_entry(archive, &name)?;
                if let Ok(v) = serde_json::from_slice::<Value>(&bytes) {
                    let values: Vec<String> = v
                        .get("values")
                        .and_then(Value::as_array)
                        .map(|a| {
                            a.iter()
                                .filter_map(|x| x.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();
                    data.timeline_tags
                        .insert(rest.trim_end_matches(".json").to_string(), values);
                }
            }
        } else if let Some(rest) = name.strip_prefix("data/minecraft/tags/worldgen/biome/") {
            if rest.ends_with(".json") {
                let bytes = read_entry(archive, &name)?;
                if let Ok(v) = serde_json::from_slice::<Value>(&bytes) {
                    let values: Vec<String> = v
                        .get("values")
                        .and_then(Value::as_array)
                        .map(|a| {
                            a.iter()
                                .filter_map(|x| x.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();
                    data.biome_tags
                        .insert(rest.trim_end_matches(".json").to_string(), values);
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

    // ---- built-in dimension_type / timeline registry tables ----
    // The data/minecraft/{dimension_type,timeline,tags/timeline} side of the
    // jar: the registry entries a vanilla server syncs and a matching client
    // could claim as known packs. Appended here so the biome tables above
    // stay byte-stable across the arc.
    out.push_str("/// The `DimensionType$Skybox` enum — the vanilla string-representable\n");
    out.push_str("/// (\"none\" | \"overworld\" | \"end\") the client's sky pipeline gates on\n");
    out.push_str("/// (SkyRenderer.extractRenderState: NONE returns before ANY sky state,\n");
    out.push_str("/// END extracts only the end-flash triple and returns). The ABSENT\n");
    out.push_str("/// JSON key is the codec default OVERWORLD (optionalFieldOf — the\n");
    out.push_str("/// DimensionType bytecode).\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum Skybox {\n    None,\n    Overworld,\n    End,\n}\n\n");
    out.push_str("/// One built-in `minecraft:dimension_type` registry entry (the jar's\n");
    out.push_str("/// `data/minecraft/dimension_type/*.json`): world layout + environment.\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\npub struct DimensionTypeDef {\n    pub name: &'static str,\n    pub min_y: i32,\n    pub height: u32,\n    pub logical_height: u32,\n    pub has_skylight: bool,\n    pub has_fixed_time: bool,\n    pub ambient_light: f32,\n    pub coordinate_scale: f64,\n    pub timelines: Option<&'static str>,\n    pub default_clock: Option<&'static str>,\n    /// The lightmap's AmbientColor base (visual/ambient_light_color;\n    /// the bytecode default black when the JSON states none).\n    pub ambient_light_color: [u8; 3],\n    /// The sky-pipeline gate (SkyRenderer.extractRenderState's skybox).\n    pub skybox: Skybox,\n    /// The STATED fog distances (blocks) — None when the JSON states\n    /// none (the bytecode defaults' shape: start 0.0 / end 1024.0).\n    pub fog_start_distance: Option<f32>,\n    pub fog_end_distance: Option<f32>,\n    /// The STATED cloud-fog end (blocks) — None when the JSON states\n    /// none (the bytecode default 2048.0; no vanilla dimension\n    /// declares it, the consumer's `visual/cloud_fog_end_distance`).\n    pub cloud_fog_end_distance: Option<f32>,\n    /// The STATED block-light tint — None when the JSON states\n    /// none (the bytecode default -10100 = 0xFFFFD88C = #FFD88C,\n    /// the warm torch cast; no vanilla dimension declares it, the\n    /// consumer's `visual/block_light_tint`).\n    pub block_light_tint: Option<[u8; 3]>,\n}\n\n");
    out.push_str("pub static DIMENSION_TYPES: [DimensionTypeDef; ");
    out.push_str(&data.dimension_types.len().to_string());
    out.push_str("] = [\n");
    for d in data.dimension_types.values() {
        let opt = |s: &Option<String>| match s {
            Some(v) => format!("Some(\"{}\")", rust_str(v)),
            None => "None".to_string(),
        };
        out.push_str(&format!(
            "    DimensionTypeDef {{ name: \"{}\", min_y: {}, height: {}, logical_height: {}, has_skylight: {}, has_fixed_time: {}, ambient_light: {:?}, coordinate_scale: {:?}, timelines: {}, default_clock: {}, ambient_light_color: {:?}, skybox: Skybox::{:?}, fog_start_distance: {:?}, fog_end_distance: {:?}, cloud_fog_end_distance: {:?}, block_light_tint: {:?} }},\n",
            rust_str(&d.name),
            d.min_y,
            d.height,
            d.logical_height,
            d.has_skylight,
            d.has_fixed_time,
            d.ambient_light,
            d.coordinate_scale,
            opt(&d.timelines),
            opt(&d.default_clock),
            d.ambient_light_color,
            d.skybox,
            d.fog_start_distance,
            d.fog_end_distance,
            d.cloud_fog_end_distance,
            d.block_light_tint,
        ));
    }
    out.push_str("];\n\n");
    out.push_str("/// Dimension type by registry id (\"minecraft:overworld\"), sorted for binary search.\n");
    out.push_str("pub fn dimension_type(name: &str) -> Option<&'static DimensionTypeDef> {\n");
    out.push_str("    DIMENSION_TYPES.binary_search_by_key(&name, |d| d.name).ok().map(|i| &DIMENSION_TYPES[i])\n}\n\n");

    out.push_str("/// One built-in `minecraft:timeline` registry entry (the jar's\n");
    out.push_str("/// `data/minecraft/timeline/*.json`): keyframe tracks over a world clock.\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\npub struct TimelineDef {\n    pub name: &'static str,\n    pub clock: &'static str,\n    pub period_ticks: Option<i64>,\n    pub tracks: usize,\n    /// `minecraft:visual/sky_light_factor` keyframes, sorted by tick — the\n    /// render track the lightmap's SkyFactor multiplies the sky term by\n    /// (empty when the timeline carries no such track)\n    pub sky_light_factor: &'static [(i64, f32)],\n}\n\n");
    for (id, t) in &data.timelines {
        if t.sky_light_factor.is_empty() {
            continue;
        }
        let ident = format!(
            "TL_{}_SKY_LIGHT_FACTOR",
            id.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect::<String>().to_uppercase()
        );
        out.push_str(&format!("pub static {ident}: [(i64, f32); {}] = [", t.sky_light_factor.len()));
        for (i, (tick, val)) in t.sky_light_factor.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(&format!("({tick}, {val:?})"));
        }
        out.push_str("];\n");
    }
    out.push_str("pub static TIMELINES: [TimelineDef; ");
    out.push_str(&data.timelines.len().to_string());
    out.push_str("] = [\n");
    for t in data.timelines.values() {
        let skf = if t.sky_light_factor.is_empty() {
            "&[]".to_string()
        } else {
            let ident = format!(
                "TL_{}_SKY_LIGHT_FACTOR",
                t.name.trim_start_matches("minecraft:").chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect::<String>().to_uppercase()
            );
            format!("&{ident}")
        };
        out.push_str(&format!(
            "    TimelineDef {{ name: \"{}\", clock: \"{}\", period_ticks: {}, tracks: {}, sky_light_factor: {} }},\n",
            rust_str(&t.name),
            rust_str(&t.clock),
            match t.period_ticks {
                Some(p) => format!("Some({p})"),
                None => "None".to_string(),
            },
            t.tracks,
            skf,
        ));
    }
    out.push_str("];\n\n");
    out.push_str("/// Timeline by registry id (\"minecraft:day\"), sorted for binary search.\n");
    out.push_str("pub fn timeline(name: &str) -> Option<&'static TimelineDef> {\n");
    out.push_str("    TIMELINES.binary_search_by_key(&name, |t| t.name).ok().map(|i| &TIMELINES[i])\n}\n\n");

    out.push_str("/// The jar's `data/minecraft/tags/timeline/*.json` verbatim — (tag id\n");
    out.push_str("/// with the minecraft: prefix, values; \"#\"-prefixed values name other tags).\n");
    out.push_str("pub static TIMELINE_TAGS: [(&str, &[&str]); ");
    out.push_str(&data.timeline_tags.len().to_string());
    out.push_str("] = [\n");
    for (id, values) in &data.timeline_tags {
        let vals: Vec<String> = values.iter().map(|v| format!("\"{}\"", rust_str(v))).collect();
        out.push_str(&format!(
            "    (\"minecraft:{}\", &[{}]),\n",
            rust_str(id),
            vals.join(", ")
        ));
    }
    out.push_str("];\n\n");
    out.push_str("/// Timeline tag values by full tag id (\"minecraft:in_overworld\").\n");
    out.push_str("pub fn timeline_tag(name: &str) -> Option<&'static [&'static str]> {\n");
    out.push_str("    TIMELINE_TAGS.binary_search_by_key(&name, |t| t.0).ok().map(|i| TIMELINE_TAGS[i].1)\n}\n");

    // ---- per-biome tint extras ----
    // The biome JSON surface the classic color table does not carry: the
    // grass_color_modifier enum name and the dry_foliage_color override.
    // Appended LAST so every table above stays byte-stable across the arc.
    out.push_str("\n/// The per-biome tint extras the jar states beyond the classic color\n");
    out.push_str("/// surface (`effects.grass_color_modifier`, `effects.dry_foliage_color`)\n");
    out.push_str("/// — sparse, only biomes that state at least one. The skip-sync tint\n");
    out.push_str("/// path joins this with `biome(name)` to resolve the exact verdicts the\n");
    out.push_str("/// synced `BiomeEntry` path would (the modifier blend, the dry override).\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\npub struct BiomeTintExtraDef {\n    pub biome: &'static str,\n    pub grass_color_modifier: Option<&'static str>,\n    pub dry_foliage_color: Option<u32>,\n}\n\n");
    out.push_str("pub static BIOME_TINT_EXTRAS: [BiomeTintExtraDef; ");
    out.push_str(&data.biome_tint_extras.len().to_string());
    out.push_str("] = [\n");
    for e in data.biome_tint_extras.values() {
        let modifier = match &e.grass_color_modifier {
            Some(m) => format!("Some(\"{}\")", rust_str(m)),
            None => "None".to_string(),
        };
        let dry = match e.dry_foliage_color {
            Some(c) => format!("Some({c})"),
            None => "None".to_string(),
        };
        out.push_str(&format!(
            "    BiomeTintExtraDef {{ biome: \"{}\", grass_color_modifier: {}, dry_foliage_color: {} }},\n",
            rust_str(&e.biome),
            modifier,
            dry,
        ));
    }
    out.push_str("];\n\n");
    out.push_str("/// Tint extras by registry id (\"minecraft:dark_forest\"), sorted for binary search.\n");
    out.push_str("pub fn biome_tint_extra(name: &str) -> Option<&'static BiomeTintExtraDef> {\n");
    out.push_str("    BIOME_TINT_EXTRAS.binary_search_by_key(&name, |e| e.biome).ok().map(|i| &BIOME_TINT_EXTRAS[i])\n}\n");

    // ---- built-in world_clock registry names ----
    // The data/minecraft/world_clock/ side of the jar: the registry the
    // synced world_clock table mirrors and the skip-sync day-tick path
    // needs a data-side twin for. Appended LAST (byte-stable).
    out.push_str("\n/// The built-in `minecraft:world_clock` registry names (the jar's\n");
    out.push_str("/// `data/minecraft/world_clock/*.json` file set). The 26.2 entries are\n");
    out.push_str("/// EMPTY objects — the clock behavior is code-side, the names are the\n");
    out.push_str("/// data. The skip-sync day-tick path maps a dimension's default_clock\n");
    out.push_str("/// name to its wire id through the order table; this table is the\n");
    out.push_str("/// data-side twin that pins the order table's member set.\n");
    out.push_str("pub static WORLD_CLOCKS: [&str; ");
    out.push_str(&data.world_clocks.len().to_string());
    out.push_str("] = [\n");
    for (id, name) in &data.world_clocks {
        out.push_str(&format!("    \"{}\", // data/minecraft/world_clock/{id}.json\n", rust_str(name)));
    }
    out.push_str("];\n");

    // ---- biome tags ----
    // The jar's data/minecraft/tags/worldgen/biome/*.json verbatim — the
    // SAME raw form the timeline tags carry (concrete minecraft: members
    // and nested "#"-references side by side; the 26.2 jar states 68
    // tags / 390 members and NOT ONE "replace" key). Appended LAST —
    // the prefix rule: every table above stays byte-stable across the
    // arc, so the previous generated file verifies as a byte-exact
    // prefix of this one.
    out.push_str("\n/// The jar's `data/minecraft/tags/worldgen/biome/*.json` verbatim — (tag id\n");
    out.push_str("/// with the minecraft: prefix, values; \"#\"-prefixed values name other tags).\n");
    out.push_str("pub static BIOME_TAGS: [(&str, &[&str]); ");
    out.push_str(&data.biome_tags.len().to_string());
    out.push_str("] = [\n");
    for (id, values) in &data.biome_tags {
        let vals: Vec<String> = values.iter().map(|v| format!("\"{}\"", rust_str(v))).collect();
        out.push_str(&format!(
            "    (\"minecraft:{}\", &[{}]),\n",
            rust_str(id),
            vals.join(", ")
        ));
    }
    out.push_str("];\n\n");
    out.push_str("/// Biome tag values by full tag id (\"minecraft:is_forest\").\n");
    out.push_str("pub fn biome_tag(name: &str) -> Option<&'static [&'static str]> {\n");
    out.push_str("    BIOME_TAGS.binary_search_by_key(&name, |t| t.0).ok().map(|i| BIOME_TAGS[i].1)\n}\n");
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
    fn parses_the_biome_tags_with_their_nested_members() {
        // the REAL 26.2 shape: concrete minecraft: members and nested
        // "#"-references side by side; the raw-string separator is ## —
        // the JSONs contain '"#' sequences the single-# form would eat
        let mut zip = zip_of(&[
            (
                "data/minecraft/tags/worldgen/biome/is_forest.json",
                br##"{"values":["minecraft:forest","minecraft:dark_forest","minecraft:pale_garden"]}"##
                    .to_vec(),
            ),
            (
                "data/minecraft/tags/worldgen/biome/buried_treasure.json",
                br##"{"values":["#minecraft:is_beach","minecraft:stony_shore"]}"##.to_vec(),
            ),
            (
                "data/minecraft/tags/worldgen/biome/is_beach.json",
                br##"{"values":["minecraft:beach"]}"##.to_vec(),
            ),
            // a biome json must NOT be picked up as a tag (the strict
            // prefix classification: only tags/worldgen/biome/ parses here)
            (
                "data/minecraft/worldgen/biome/forest.json",
                br##"{"temperature":0.7,"downfall":0.8}"##.to_vec(),
            ),
        ]);
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.biome_tags.len(), 3);
        assert_eq!(
            data.biome_tags["is_forest"],
            vec![
                "minecraft:forest".to_string(),
                "minecraft:dark_forest".to_string(),
                "minecraft:pale_garden".to_string(),
            ]
        );
        assert_eq!(
            data.biome_tags["buried_treasure"],
            vec![
                "#minecraft:is_beach".to_string(),
                "minecraft:stony_shore".to_string(),
            ],
            "the nested reference rides VERBATIM"
        );
        let emitted = emit(&data).unwrap();
        assert!(emitted.contains("pub static BIOME_TAGS: [(&str, &[&str]); 3]"));
        assert!(
            emitted.contains(
                "(\"minecraft:buried_treasure\", &[\"#minecraft:is_beach\", \"minecraft:stony_shore\"]),"
            ),
            "the members emit in the jar's stored order, full minecraft: tag id"
        );
        assert!(emitted.contains("pub fn biome_tag(name: &str)"));
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

    #[test]
    fn parses_the_tint_extras_the_real_jar_states() {
        // the REAL jar forms: the modifier is the vanilla enum name, the
        // dry-foliage override a "#rrggbb" string (dark_forest.json /
        // swamp.json / mangrove_swamp.json / pale_garden.json, 26.2)
        let dark = br##"{"temperature":0.7,"downfall":0.8,"has_precipitation":true,"effects":{"fog_color":12638463,"sky_color":7972607,"water_color":4159204,"water_fog_color":329011,"grass_color_modifier":"dark_forest","dry_foliage_color":"#7b5334"}}"##;
        let pale = br##"{"temperature":0.7,"downfall":0.8,"has_precipitation":true,"effects":{"fog_color":8484720,"sky_color":12171705,"water_color":7768221,"water_fog_color":329011,"grass_color":7832178,"foliage_color":8883574,"dry_foliage_color":"#a0a69c"}}"##;
        let swampy = br##"{"temperature":0.8,"downfall":0.9,"has_precipitation":true,"effects":{"fog_color":12638463,"sky_color":7907327,"water_color":6388580,"water_fog_color":329011,"foliage_color":6975545,"grass_color_modifier":"swamp","dry_foliage_color":"#7b5334"}}"##;
        let none_form = br#"{"temperature":0.5,"downfall":0.5,"has_precipitation":true,"effects":{"fog_color":12638463,"sky_color":7946167,"water_color":4159204,"water_fog_color":329011,"grass_color_modifier":"none"}}"#;
        let bare = br#"{"temperature":0.8,"downfall":0.4,"has_precipitation":true,"effects":{"fog_color":12638463,"sky_color":7946167,"water_color":4159204,"water_fog_color":329011}}"#;
        let mut zip = zip_of(&[
            ("data/minecraft/worldgen/biome/dark_forest.json", dark.to_vec()),
            ("data/minecraft/worldgen/biome/pale_garden.json", pale.to_vec()),
            ("data/minecraft/worldgen/biome/swamp.json", swampy.to_vec()),
            ("data/minecraft/worldgen/biome/none_biome.json", none_form.to_vec()),
            ("data/minecraft/worldgen/biome/plains.json", bare.to_vec()),
        ]);
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.biomes.len(), 5, "every biome parsed");
        // sparse: "none" states the absence and a biome without either field
        // gets no entry — only the three real carriers
        assert_eq!(data.biome_tint_extras.len(), 3);
        let d = &data.biome_tint_extras["dark_forest"];
        assert_eq!(d.biome, "minecraft:dark_forest");
        assert_eq!(d.grass_color_modifier.as_deref(), Some("dark_forest"));
        assert_eq!(d.dry_foliage_color, Some(0x7B5334), "the #rrggbb form parses");
        assert_eq!(data.biome_tint_extras["swamp"].grass_color_modifier.as_deref(), Some("swamp"));
        let p = &data.biome_tint_extras["pale_garden"];
        assert_eq!(p.grass_color_modifier, None, "no modifier stated");
        assert_eq!(p.dry_foliage_color, Some(0xA0A69C), "the pale garden override");

        let emitted = emit(&data).unwrap();
        assert!(emitted.contains("pub static BIOME_TINT_EXTRAS: [BiomeTintExtraDef; 3]"));
        assert!(emitted.contains(
            "BiomeTintExtraDef { biome: \"minecraft:dark_forest\", grass_color_modifier: Some(\"dark_forest\"), dry_foliage_color: Some(8082228) }"
        ));
        assert!(emitted.contains("pub fn biome_tint_extra(name: &str)"));
    }

    #[test]
    fn refuses_a_modifier_the_vanilla_enum_does_not_state() {
        let odd = br#"{"temperature":0.5,"downfall":0.5,"has_precipitation":true,"effects":{"fog_color":1,"sky_color":2,"water_color":3,"water_fog_color":4,"grass_color_modifier":"pixelated"}}"#;
        let mut zip = zip_of(&[("data/minecraft/worldgen/biome/odd.json", odd.to_vec())]);
        let err = generate(&mut zip).unwrap_err().to_string();
        assert!(err.contains("unknown grass_color_modifier"), "{err}");
        assert!(err.contains("pixelated"), "{err}");
    }

    #[test]
    fn parses_the_world_clock_names_and_refuses_a_payload() {
        // the REAL 26.2 form: empty objects — the names are the data
        let mut zip = zip_of(&[
            (
                "data/minecraft/world_clock/overworld.json",
                b"{}".to_vec(),
            ),
            ("data/minecraft/world_clock/the_end.json", b"{}".to_vec()),
        ]);
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.world_clocks.len(), 2);
        assert_eq!(data.world_clocks["overworld"], "minecraft:overworld");
        assert_eq!(data.world_clocks["the_end"], "minecraft:the_end");
        let emitted = emit(&data).unwrap();
        assert!(emitted.contains("pub static WORLD_CLOCKS: [&str; 2]"));
        assert!(emitted.contains("\"minecraft:overworld\", // data/minecraft/world_clock/overworld.json"));
        // a stated field would be dropped data — a refusal, not a guess
        let payload = br#"{"period_ticks":24000}"#;
        let mut zip2 = zip_of(&[(
            "data/minecraft/world_clock/overworld.json",
            payload.to_vec(),
        )]);
        let err = generate(&mut zip2).unwrap_err().to_string();
        assert!(err.contains("unexpected payload"), "{err}");
    }

    #[test]
    fn parses_dimension_types_timelines_and_tags() {
        // the REAL 26.2 shapes: the nether's logical_height < height, the
        // overworld carries default_clock while the nether does not, the day
        // timeline's sky_light_factor rides numeric keyframes next to
        // string-valued visual tracks, and tags nest "#"-references
        let nether = br##"{"ambient_light":0.1,"cardinal_light":"nether","coordinate_scale":8.0,"has_ceiling":true,"has_fixed_time":true,"has_skylight":false,"height":256,"logical_height":128,"min_y":0,"skybox":"none","timelines":"#minecraft:in_nether"}"##;
        let overworld = br##"{"ambient_light":0.0,"coordinate_scale":1.0,"default_clock":"minecraft:overworld","has_skylight":true,"height":384,"logical_height":384,"min_y":-64,"timelines":"#minecraft:in_overworld","attributes":{"minecraft:visual/ambient_light_color":"#0a0a0a"}}"##;
        // the stated "end" shape and the unknown-string shape (the
        // vanilla codec would reject the latter outright — the parse
        // falls back to the codec default, never a guess)
        let the_end = br##"{"ambient_light":0.25,"coordinate_scale":1.0,"has_fixed_time":true,"has_skylight":true,"height":256,"logical_height":256,"min_y":0,"skybox":"end","timelines":"#minecraft:in_end"}"##;
        let custom = br##"{"ambient_light":0.0,"coordinate_scale":1.0,"has_skylight":true,"height":384,"logical_height":384,"min_y":-64,"skybox":"weird","timelines":"#minecraft:in_overworld","attributes":{"minecraft:visual/cloud_fog_end_distance":512.0,"minecraft:visual/block_light_tint":"#302821"}}"##;
        let day = br##"{"clock":"minecraft:overworld","period_ticks":24000,"tracks":{"minecraft:gameplay/monsters_burn":{"keyframes":[{"ticks":12542,"value":true},{"ticks":23460,"value":false}],"modifier":"or"},"minecraft:visual/sky_color":{"keyframes":[{"ticks":133,"value":"#ffffff"},{"ticks":11867,"value":"#000000"}],"modifier":"multiply"},"minecraft:visual/sky_light_factor":{"keyframes":[{"ticks":22860,"value":0.24},{"ticks":730,"value":1.0},{"ticks":11270,"value":1.0},{"ticks":13140,"value":0.24}],"modifier":"multiply"}}}"##;
        let early_game = br##"{"clock":"minecraft:overworld","tracks":{"minecraft:gameplay/can_sleep":{"keyframes":[{"ticks":0,"value":true}]}}}"##;
        let mut zip = zip_of(&[
            ("data/minecraft/dimension_type/the_nether.json", nether.to_vec()),
            ("data/minecraft/dimension_type/overworld.json", overworld.to_vec()),
            ("data/minecraft/dimension_type/the_end.json", the_end.to_vec()),
            ("data/minecraft/dimension_type/custom.json", custom.to_vec()),
            ("data/minecraft/timeline/day.json", day.to_vec()),
            ("data/minecraft/timeline/early_game.json", early_game.to_vec()),
            ("data/minecraft/tags/timeline/universal.json", br##"{"values":["minecraft:villager_schedule"]}"##.to_vec()),
            ("data/minecraft/tags/timeline/in_overworld.json", br##"{"values":["#minecraft:universal","minecraft:day","minecraft:early_game"]}"##.to_vec()),
        ]);
        let data = generate(&mut zip).unwrap();

        let n = &data.dimension_types["the_nether"];
        assert_eq!((n.min_y, n.height, n.logical_height), (0, 256, 128));
        assert!(n.has_fixed_time && !n.has_skylight);
        assert_eq!((n.ambient_light, n.coordinate_scale), (0.1, 8.0));
        assert_eq!(n.timelines.as_deref(), Some("#minecraft:in_nether"));
        assert_eq!(n.default_clock, None);
        let o = &data.dimension_types["overworld"];
        assert_eq!((o.min_y, o.height), (-64, 384));
        assert!(!o.has_fixed_time, "absent has_fixed_time defaults false");
        assert_eq!(o.default_clock.as_deref(), Some("minecraft:overworld"));
        // the stated ambient attribute parses per channel (overworld
        // #0a0a0a); the nether fixture states none — the bytecode
        // default (black) fills, the absent-attribute shape
        assert_eq!(o.ambient_light_color, [0x0a, 0x0a, 0x0a]);
        assert_eq!(n.ambient_light_color, [0, 0, 0]);
        // the skybox parse: the stated "none"/"end" map to the enum, the
        // ABSENT key is the codec default OVERWORLD, the unknown string
        // falls back to the codec default (the vanilla rejection shape)
        assert_eq!(n.skybox, Skybox::None);
        assert_eq!(data.dimension_types["the_end"].skybox, Skybox::End);
        assert_eq!(o.skybox, Skybox::Overworld);
        assert_eq!(data.dimension_types["custom"].skybox, Skybox::Overworld);
        // the cloud-fog end: a stated FLOAT rides through, the ABSENT
        // key (every vanilla dimension) reads None — the bytecode
        // default's shape, the consumer's 2048.0 verdict
        assert_eq!(
            data.dimension_types["custom"].cloud_fog_end_distance,
            Some(512.0),
            "the stated attribute parses verbatim"
        );
        assert_eq!(
            n.cloud_fog_end_distance, None,
            "no vanilla dimension declares it — the absent shape"
        );
        // the block-light tint: a stated rgb_color string parses per
        // channel; the ABSENT key (every vanilla dimension) reads None
        // — the bytecode default's shape (#FFD88C, the consumer's
        // verdict, never baked here)
        assert_eq!(
            data.dimension_types["custom"].block_light_tint,
            Some([0x30, 0x28, 0x21]),
            "the stated attribute parses per channel"
        );
        assert_eq!(
            n.block_light_tint, None,
            "no vanilla dimension declares it — the absent shape"
        );

        let d = &data.timelines["day"];
        assert_eq!(d.clock, "minecraft:overworld");
        assert_eq!(d.period_ticks, Some(24_000));
        assert_eq!(d.tracks, 3);
        // unsorted keyframes arrive sorted; the hex-string visual track is
        // not the numeric render track
        assert_eq!(
            d.sky_light_factor,
            vec![(730, 1.0), (11_270, 1.0), (13_140, 0.24), (22_860, 0.24)]
        );
        assert!(data.timelines["early_game"].sky_light_factor.is_empty());
        assert_eq!(data.timelines["early_game"].period_ticks, None);
        assert_eq!(
            data.timeline_tags["in_overworld"],
            vec!["#minecraft:universal", "minecraft:day", "minecraft:early_game"]
        );

        let emitted = emit(&data).unwrap();
        assert!(emitted.contains("pub static DIMENSION_TYPES: [DimensionTypeDef; 4]"));
        assert!(emitted.contains("min_y: -64, height: 384, logical_height: 384"));
        assert!(emitted.contains("ambient_light: 0.1, coordinate_scale: 8.0"));
        assert!(emitted.contains("ambient_light_color: [10, 10, 10]"));
        assert!(emitted.contains("ambient_light_color: [0, 0, 0]"));
        assert!(emitted.contains("pub enum Skybox {"));
        assert!(emitted.contains("skybox: Skybox::None"));
        assert!(emitted.contains("skybox: Skybox::End"));
        assert!(emitted.contains("skybox: Skybox::Overworld"));
        assert!(emitted.contains("pub static TL_DAY_SKY_LIGHT_FACTOR: [(i64, f32); 4] = [(730, 1.0), (11270, 1.0), (13140, 0.24), (22860, 0.24)];"));
        assert!(emitted.contains("pub static TIMELINES: [TimelineDef; 2]"));
        assert!(emitted.contains("period_ticks: Some(24000), tracks: 3"));
        assert!(emitted.contains("pub fn dimension_type(name: &str)"));
        assert!(emitted.contains("pub fn timeline(name: &str)"));
        assert!(emitted.contains("(\"minecraft:in_overworld\", &[\"#minecraft:universal\", \"minecraft:day\", \"minecraft:early_game\"])"));
        assert!(emitted.contains("pub fn timeline_tag(name: &str)"));
    }
}

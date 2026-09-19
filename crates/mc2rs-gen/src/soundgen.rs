//! Sound parsing: reads the official `sounds.json` (delivered through
//! the asset index, not inside the jar) into the working sound-event
//! map the generator's own download walk speaks — the fetch of every
//! music/UI/biome/mob/player clip the runtime cache keeps. THE
//! GENERATED RUST TABLE THE MIGRATION RETIRED (the generated/ ->
//! runtime-load migration's third lane): the same bytes the fetch
//! copies verbatim into the runtime cache
//! (assets-cache/jar/assets/minecraft/sounds.json), the CLIENT's own
//! loader the only parser of the shipped file — the parse law here
//! feeds the download walk only, nothing the published tree holds.

use std::collections::BTreeMap;

use anyhow::{Context, Result};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SoundFile {
    /// path as given by sounds.json ("music/menu/menu1")
    pub name: String,
    pub volume: f32,
    #[expect(dead_code)] // the sound json's own pitch column — the report keeps
    pub pitch: f32,
    #[expect(dead_code)] // the sound json's own weight column — the report keeps
    pub weight: u32,
    pub stream: bool,
}

#[derive(Debug, Clone)]
pub struct SoundEvent {
    pub id: String,
    pub subtitle: Option<String>,
    pub files: Vec<SoundFile>,
}

#[derive(Debug, Default)]
pub struct SoundData {
    pub events: BTreeMap<String, SoundEvent>,
}

impl SoundData {
    pub fn event(&self, id: &str) -> Option<&SoundEvent> {
        self.events.get(id)
    }

    /// All events under a namespace prefix, sorted ("music.menu." etc).
    pub fn events_with_prefix(&self, prefix: &str) -> Vec<&SoundEvent> {
        self.events
            .values()
            .filter(|e| e.id.starts_with(prefix))
            .collect()
    }
}

pub fn generate(bytes: &[u8]) -> Result<SoundData> {
    let v: Value = serde_json::from_slice(bytes).context("sounds.json parse")?;
    let obj = v.as_object().context("sounds.json is not an object")?;
    let mut data = SoundData::default();
    for (id, ev) in obj {
        let subtitle = ev.get("subtitle").and_then(Value::as_str).map(String::from);
        let mut files = Vec::new();
        match ev.get("sounds") {
            Some(Value::Array(list)) => {
                for s in list {
                    match s {
                        Value::String(name) => files.push(SoundFile {
                            name: name.clone(),
                            volume: 1.0,
                            pitch: 1.0,
                            weight: 1,
                            stream: false,
                        }),
                        Value::Object(o) => {
                            let Some(name) = o.get("name").and_then(Value::as_str) else { continue };
                            files.push(SoundFile {
                                name: name.to_string(),
                                volume: o.get("volume").and_then(Value::as_f64).unwrap_or(1.0) as f32,
                                pitch: o.get("pitch").and_then(Value::as_f64).unwrap_or(1.0) as f32,
                                weight: o.get("weight").and_then(Value::as_u64).unwrap_or(1) as u32,
                                stream: o.get("stream").and_then(Value::as_bool).unwrap_or(false),
                            })
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        data.events.insert(id.clone(), SoundEvent { id: id.clone(), subtitle, files });
    }
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_string_and_object_sounds() {
        let json = br#"{
            "music.menu.menu1": {"sounds":[{"name":"music/menu/menu1","stream":true,"volume":0.5}]},
            "ui.button.click": {"sounds":["click1","click2"],"subtitle":"subtitles.ui.click"}
        }"#;
        let data = generate(json).unwrap();
        assert_eq!(data.events.len(), 2);
        let m = data.event("music.menu.menu1").unwrap();
        assert_eq!(m.files.len(), 1);
        assert!(m.files[0].stream);
        assert_eq!(m.files[0].volume, 0.5);
        let ui = data.event("ui.button.click").unwrap();
        assert_eq!(ui.subtitle.as_deref(), Some("subtitles.ui.click"));
        assert_eq!(ui.files[1].name, "click2");
        assert_eq!(ui.files[1].volume, 1.0);

        let menu = data.events_with_prefix("music.menu.");
        assert_eq!(menu.len(), 1);
    }
}

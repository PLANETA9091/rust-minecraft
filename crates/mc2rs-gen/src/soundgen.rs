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
    #[cfg_attr(not(test), expect(dead_code))]
    pub volume: f32,
    #[expect(dead_code)] // the sound json's own pitch column — the report keeps
    pub pitch: f32,
    #[expect(dead_code)] // the sound json's own weight column — the report keeps
    pub weight: u32,
    #[cfg_attr(not(test), expect(dead_code))]
    pub stream: bool,
}

#[derive(Debug, Clone)]
pub struct SoundEvent {
    pub id: String,
    #[cfg_attr(not(test), expect(dead_code))]
    pub subtitle: Option<String>,
    pub files: Vec<SoundFile>,
}

#[derive(Debug, Default)]
pub struct SoundData {
    pub events: BTreeMap<String, SoundEvent>,
}

impl SoundData {
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn event(&self, id: &str) -> Option<&SoundEvent> {
        self.events.get(id)
    }

    /// All events under a namespace prefix, sorted ("music.menu." etc).
    #[cfg_attr(not(test), expect(dead_code))]
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

/// THE MOB VOICE LANE'S OWN EVENTS (the ids the download walk's `wanted`
/// gate admits beside the block family — every file of every event the
/// sha1-verified asset-index fetch copies into the runtime cache). Two
/// drawn kinds the client's own tables speak:
///
/// - the zombie's four voices (the ambient groan, the hurt, the death,
///   and the playStepSound footstep the cadence lane speaks — the FLAT
///   fold the class's own override keeps);
/// - the wolf's four voices (the 26.2 wolf_sound_variant's CLASSIC
///   default the client's `voice_event`/`step_voice` tables name — the
///   ambient bark, the hurt, the death, and the shared step event the
///   variant directories' five files answer). THE FETCH THE CLIENT'S
///   TABLES OWE: the rows the voice tables speak the download walk
///   arms — an unfetched id the Rodio path's missing-file skip keeps
///   silent (the honest refusal the audio lane never breaks the frame
///   with), the drawn wolf the barkless body the oggs' absence wore;
/// - the wolf's THREE STATE VOICES (the getAmbientSound four-arm walk
///   the client's `ambient_voice_event` selection speaks — the angry
///   body's growl, the tamed-hurt whine, the idle pant; the ids the
///   kind's flat ambient row never named, the state arms the oggs
///   now answer).
///
/// Every file rides the same sha1-verified asset-index fetch the music
/// and block lanes speak.
pub const MOB_VOICE_EVENTS: [&str; 11] = [
    "entity.zombie.ambient",
    "entity.zombie.hurt",
    "entity.zombie.death",
    "entity.zombie.step",
    "entity.wolf.ambient",
    "entity.wolf.hurt",
    "entity.wolf.death",
    "entity.wolf.step",
    "entity.wolf.growl",
    "entity.wolf.whine",
    "entity.wolf.pant",
];

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

    /// THE MOB VOICE LANE'S OWN PIN (the fetch list the client's voice
    /// tables name — the zombie's four the FLAT fold speaks, the wolf's
    /// four the CLASSIC default answers, the state trio the ambient
    /// four-arm walk wears): the eleven ids the download walk admits,
    /// the order the kinds' rows keep, no third kind the list never
    /// draws (the undrawn body the honest absence wears — the fetch
    /// the client's own tables alone define). The ids' real sounds.json
    /// admission the client's asset-gated walk proves (the runtime
    /// table the cache keeps — the ignored lane's own word).
    #[test]
    fn mob_voice_events_name_the_drawn_kinds_four_voices() {
        assert_eq!(
            MOB_VOICE_EVENTS,
            [
                "entity.zombie.ambient",
                "entity.zombie.hurt",
                "entity.zombie.death",
                "entity.zombie.step",
                "entity.wolf.ambient",
                "entity.wolf.hurt",
                "entity.wolf.death",
                "entity.wolf.step",
                "entity.wolf.growl",
                "entity.wolf.whine",
                "entity.wolf.pant",
            ]
        );
        // the two drawn kinds the list's rows pair — no duplicates, no
        // third kind's id the fetch never arms
        let kinds: Vec<&str> = MOB_VOICE_EVENTS.iter().map(|id| {
            id.strip_prefix("entity.")
                .and_then(|rest| rest.split('.').next())
                .expect("every id the entity.<kind>.<voice> shape wears")
        }).collect();
        assert_eq!(kinds, vec!["zombie", "zombie", "zombie", "zombie", "wolf", "wolf", "wolf", "wolf", "wolf", "wolf", "wolf"]);
        let mut sorted = MOB_VOICE_EVENTS.to_vec();
        sorted.sort_unstable();
        let uniq = sorted.len();
        sorted.dedup();
        assert_eq!(sorted.len(), uniq, "no duplicate ids the fetch would double-count");
    }
}

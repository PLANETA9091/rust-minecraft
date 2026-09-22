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
/// sha1-verified asset-index fetch copies into the runtime cache). Eight
/// drawn kinds the client's own tables speak:
///
/// - the zombie's four voices (the ambient groan, the hurt, the death,
///   and the playStepSound footstep the cadence lane speaks — the FLAT
///   fold the class's own override keeps);
/// - the wolf's four voices (the 26.2 wolf_sound_variant's CLASSIC
///   default the client's `voice_event`/`step_voice` tables name — the
///   ambient bark, the hurt, the death, and the shared step event the
///   variant directories' five files answer) PLUS the THREE STATE
///   VOICES (the getAmbientSound four-arm walk the client's
///   `ambient_voice_event` selection speaks — the angry body's growl,
///   the tamed-hurt whine, the idle pant);
/// - the sheep's flat trio (the ambient baa, the hurt, the death — the
///   step voice the BLOCK lane keeps, `Entity.playStepSound`'s
///   soundtype walk the client's block-sound table answers, no
///   entity.sheep.step event the 26.2 sounds.json banks);
/// - the cow's classic AND moody quads (the farm_sound_variant tables
///   the synced name picks — each set's own ambient/hurt/death/step
///   rows the sounds.json banks);
/// - the pig's classic quint + big quad + baby quint (the
///   `getSoundSet().stepSound()`/`playEatingSound` overrides the eat
///   and step rows speak — the classic step row BOTH adult sets keep,
///   the baby's own step the baby set banks; no entity.pig_big.step
///   event the json names, the honest absence the client's step table
///   mirrors);
/// - the chicken's classic quad + picky trio + baby quad (the
///   chicken_sound_variant's both sets, the baby's own rows);
/// - the skeleton's and the spider's flat quads (the AbstractSkeleton
///   and Spider FLAT folds the `playSound(..., 0.15F, 1.0F)` walk
///   keeps — the ambient/hurt/death/step rows the 26.2 sounds.json
///   banks);
/// - the creeper's own PAIR (the getHurtSound/getDeathSound getters
///   the sounds.json's entity.creeper rows name — the say-clip set
///   the hurt banks, the death the own file; NO ambient the
///   getAmbientSound override never carries, the step the base block
///   law the block-sound table serves).
///
/// THE FETCH THE CLIENT'S TABLES OWE: every row the client's own
/// voice/step/variant/eat tables speak the download walk arms — an
/// unfetched id the Rodio path's missing-file skip keeps silent (the
/// honest refusal the audio lane never breaks the frame with), the
/// drawn world the mute bodies the oggs' absence wore. Every file
/// rides the same sha1-verified asset-index fetch the music and block
/// lanes speak.
pub const MOB_VOICE_EVENTS: [&str; 57] = [
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
    "entity.sheep.ambient",
    "entity.sheep.hurt",
    "entity.sheep.death",
    "entity.cow.ambient",
    "entity.cow.hurt",
    "entity.cow.death",
    "entity.cow.step",
    "entity.cow_moody.ambient",
    "entity.cow_moody.hurt",
    "entity.cow_moody.death",
    "entity.cow_moody.step",
    "entity.pig.ambient",
    "entity.pig.hurt",
    "entity.pig.death",
    "entity.pig.step",
    "entity.pig.eat",
    "entity.pig_big.ambient",
    "entity.pig_big.hurt",
    "entity.pig_big.death",
    "entity.pig_big.eat",
    "entity.baby_pig.ambient",
    "entity.baby_pig.hurt",
    "entity.baby_pig.death",
    "entity.baby_pig.step",
    "entity.baby_pig.eat",
    "entity.chicken.ambient",
    "entity.chicken.hurt",
    "entity.chicken.death",
    "entity.chicken.step",
    "entity.chicken_picky.ambient",
    "entity.chicken_picky.hurt",
    "entity.chicken_picky.death",
    "entity.baby_chicken.ambient",
    "entity.baby_chicken.hurt",
    "entity.baby_chicken.death",
    "entity.baby_chicken.step",
    "entity.skeleton.ambient",
    "entity.skeleton.hurt",
    "entity.skeleton.death",
    "entity.skeleton.step",
    "entity.spider.ambient",
    "entity.spider.hurt",
    "entity.spider.death",
    "entity.spider.step",
    // the creeper's own PAIR (the getHurtSound/getDeathSound rows the
    // class's own getters speak — the say-clip set the hurt names, the
    // death the own file answers; NO ambient the getAmbientSound
    // override the class never carries, NO step the base block law
    // the block-sound table serves)
    "entity.creeper.hurt",
    "entity.creeper.death",
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
    /// four + the state trio the CLASSIC default and the ambient
    /// four-arm walk wear, the six remaining kinds' rows the flat and
    /// variant/eat tables bank): the fifty-five ids the download walk
    /// admits, the order the kinds' rows keep, no undrawn kind's id the
    /// list never names (the fetch the client's own tables alone
    /// define). The ids' real sounds.json admission the client's
    /// asset-gated walk proves (the runtime table the cache keeps —
    /// the ignored lane's own word).
    #[test]
    fn mob_voice_events_name_the_drawn_kinds_voice_rows() {
        assert_eq!(
            MOB_VOICE_EVENTS,
            [
                // the zombie's flat quad
                "entity.zombie.ambient",
                "entity.zombie.hurt",
                "entity.zombie.death",
                "entity.zombie.step",
                // the wolf's flat quad + the state trio
                "entity.wolf.ambient",
                "entity.wolf.hurt",
                "entity.wolf.death",
                "entity.wolf.step",
                "entity.wolf.growl",
                "entity.wolf.whine",
                "entity.wolf.pant",
                // the sheep's flat trio (the step the block lane keeps)
                "entity.sheep.ambient",
                "entity.sheep.hurt",
                "entity.sheep.death",
                // the cow's classic + moody quads
                "entity.cow.ambient",
                "entity.cow.hurt",
                "entity.cow.death",
                "entity.cow.step",
                "entity.cow_moody.ambient",
                "entity.cow_moody.hurt",
                "entity.cow_moody.death",
                "entity.cow_moody.step",
                // the pig's classic quint + big quad + baby quint
                "entity.pig.ambient",
                "entity.pig.hurt",
                "entity.pig.death",
                "entity.pig.step",
                "entity.pig.eat",
                "entity.pig_big.ambient",
                "entity.pig_big.hurt",
                "entity.pig_big.death",
                "entity.pig_big.eat",
                "entity.baby_pig.ambient",
                "entity.baby_pig.hurt",
                "entity.baby_pig.death",
                "entity.baby_pig.step",
                "entity.baby_pig.eat",
                // the chicken's classic quad + picky trio + baby quad
                "entity.chicken.ambient",
                "entity.chicken.hurt",
                "entity.chicken.death",
                "entity.chicken.step",
                "entity.chicken_picky.ambient",
                "entity.chicken_picky.hurt",
                "entity.chicken_picky.death",
                "entity.baby_chicken.ambient",
                "entity.baby_chicken.hurt",
                "entity.baby_chicken.death",
                "entity.baby_chicken.step",
                // the skeleton's and the spider's flat quads
                "entity.skeleton.ambient",
                "entity.skeleton.hurt",
                "entity.skeleton.death",
                "entity.skeleton.step",
                "entity.spider.ambient",
                "entity.spider.hurt",
                "entity.spider.death",
                "entity.spider.step",
                // the creeper's own pair (the hurt the say-clip set, the
                // death the own file — no ambient the getter never names)
                "entity.creeper.hurt",
                "entity.creeper.death",
            ]
        );
        // the nine drawn kinds the list's rows pair — no duplicates,
        // no undrawn kind's id the fetch never arms; the variant and
        // baby ids the PARENT kind's own table rows ride (the
        // pig_big/baby_pig rows the PIG's sets bank, the picky/baby
        // chicken rows the CHICKEN's sets, the moody rows the COW's —
        // the alias map the client's farm_sound_variant tables keep,
        // every other first segment the kind's own name answers)
        fn kind_of(id: &str) -> &str {
            let first = id
                .strip_prefix("entity.")
                .and_then(|rest| rest.split('.').next())
                .expect("every id the entity.<kind>.<voice> shape wears");
            match first {
                "pig_big" | "baby_pig" => "pig",
                "chicken_picky" | "baby_chicken" => "chicken",
                "cow_moody" => "cow",
                other => other,
            }
        }
        let kinds: Vec<&str> = MOB_VOICE_EVENTS.iter().map(|id| kind_of(id)).collect();
        assert_eq!(kinds, vec![
            "zombie", "zombie", "zombie", "zombie",
            "wolf", "wolf", "wolf", "wolf", "wolf", "wolf", "wolf",
            "sheep", "sheep", "sheep",
            "cow", "cow", "cow", "cow",
            "cow", "cow", "cow", "cow",
            "pig", "pig", "pig", "pig", "pig",
            "pig", "pig", "pig", "pig",
            "pig", "pig", "pig", "pig", "pig",
            "chicken", "chicken", "chicken", "chicken",
            "chicken", "chicken", "chicken",
            "chicken", "chicken", "chicken", "chicken",
            "skeleton", "skeleton", "skeleton", "skeleton",
            "spider", "spider", "spider", "spider",
            "creeper", "creeper",
        ]);
        let mut sorted = MOB_VOICE_EVENTS.to_vec();
        sorted.sort_unstable();
        let uniq = sorted.len();
        sorted.dedup();
        assert_eq!(sorted.len(), uniq, "no duplicate ids the fetch would double-count");
    }
}

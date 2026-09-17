//! THE SOUND TABLE'S OWN LOADER (the generated/ -> runtime-load migration's
//! third lane — the 1MB+ sound-event table the compiled source no longer
//! carries): the official `sounds.json` the runtime cache keeps VERBATIM
//! (`assets-cache/jar/assets/minecraft/sounds.json` — the asset-index fetch
//! the generator's own walk sha1-verifies and copies, no parsing at the
//! generator beyond the download walk's own bookkeeping), the parse law the
//! old generator's emit walk spoke — the string sounds the default folds
//! (volume 1.0, pitch 1.0, weight 1, stream false), the object sounds the
//! explicit fields' unwrap_or arms, the name-less objects the skip law
//! keeps, the BTreeMap the sorted order the old emitted table held (the
//! binary_search the same lookup walks, the menu_tracks order the same list
//! keeps). The absent/broken file the honest empty table keeps — the
//! generator's own empty-table warning arm the same boot walks, the every
//! call site's Option arm answers. The OnceLock the load-once law keeps —
//! the 'static borrows the static's own lifetime answers (the
//! tracks_for/menu_tracks &'static str signatures and the pick law the
//! tests pin).

#[derive(Debug, Clone, PartialEq)]
pub struct SoundFileDef {
    pub name: String,
    pub volume: f32,
    pub pitch: f32,
    pub weight: u32,
    pub stream: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoundEventDef {
    pub id: String,
    pub subtitle: Option<String>,
    pub first: usize,
    pub len: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SoundTable {
    pub files: Vec<SoundFileDef>,
    pub events: Vec<SoundEventDef>,
}

/// THE PARSE LAW (the old generator's walk verbatim — the same defaults,
/// the same skip, the same sorted order the BTreeMap keeps, the broken
/// bytes the honest empty table keeps).
fn parse_sounds(bytes: &[u8]) -> SoundTable {
    let v: serde_json::Value = match serde_json::from_slice(bytes) {
        Ok(v) => v,
        Err(_) => return SoundTable::default(),
    };
    let obj = match v.as_object() {
        Some(o) => o,
        None => return SoundTable::default(),
    };
    let mut events = std::collections::BTreeMap::new();
    for (id, ev) in obj {
        let subtitle = ev
            .get("subtitle")
            .and_then(serde_json::Value::as_str)
            .map(String::from);
        let mut files = Vec::new();
        if let Some(serde_json::Value::Array(list)) = ev.get("sounds") {
            for s in list {
                match s {
                    serde_json::Value::String(name) => files.push(SoundFileDef {
                        name: name.clone(),
                        volume: 1.0,
                        pitch: 1.0,
                        weight: 1,
                        stream: false,
                    }),
                    serde_json::Value::Object(o) => {
                        let Some(name) = o.get("name").and_then(serde_json::Value::as_str) else {
                            continue;
                        };
                        files.push(SoundFileDef {
                            name: name.to_string(),
                            volume: o
                                .get("volume")
                                .and_then(serde_json::Value::as_f64)
                                .unwrap_or(1.0) as f32,
                            pitch: o
                                .get("pitch")
                                .and_then(serde_json::Value::as_f64)
                                .unwrap_or(1.0) as f32,
                            weight: o
                                .get("weight")
                                .and_then(serde_json::Value::as_u64)
                                .unwrap_or(1) as u32,
                            stream: o
                                .get("stream")
                                .and_then(serde_json::Value::as_bool)
                                .unwrap_or(false),
                        });
                    }
                    _ => {}
                }
            }
        }
        events.insert(id.clone(), (subtitle, files));
    }
    // THE FLATTEN WALK (the old emit's own law): the sorted events one
    // span each, (first, len) into the flat file table the files() reads —
    // the same order the old SOUND_EVENTS/SOUND_FILES arrays held.
    let mut table = SoundTable::default();
    for (id, (subtitle, files)) in events {
        let first = table.files.len();
        let len = files.len();
        table.files.extend(files);
        table.events.push(SoundEventDef { id, subtitle, first, len });
    }
    table
}

fn load_sounds() -> SoundTable {
    // The roots the splash/lang loaders' own walk speaks: the workspace
    // root the binary's cwd keeps, the crate dir the cargo test cwd one
    // and two steps below.
    for prefix in [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ] {
        if let Ok(bytes) =
            std::fs::read(std::path::Path::new(prefix).join("assets/minecraft/sounds.json"))
        {
            return parse_sounds(&bytes);
        }
    }
    SoundTable::default()
}

/// THE LOADED TABLE (the OnceLock the load-once law keeps — the 'static
/// borrows the static's own lifetime answers).
fn table() -> &'static SoundTable {
    static TABLE: std::sync::OnceLock<SoundTable> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        let t = load_sounds();
        // THE TEST BOOT'S OWN ARM (the CI's pre-generate test job the
        // cache never exists at — cargo test runs BEFORE any generate):
        // the synthetic fixture table the voice laws exercise everywhere.
        // The release boot the honest cache-or-empty law keeps — the
        // synthetic table never ships.
        #[cfg(test)]
        {
            if t.events.is_empty() {
                return test_fixture_table();
            }
        }
        t
    })
}

/// THE TEST BOOT'S OWN TABLE: the ids the client's own voices name —
/// every event the compiled block SoundType rows speak (hit/break/place/
/// step/fall), every event the official biome-music table references,
/// and the entity/player/ui literals the lanes play — two default files
/// each, the sorted order the BTreeMap keeps. The weighted picks, the
/// folds and the cadences the voice laws read speak the same shapes the
/// real table holds (the folds ride the default v1.0/p1.0 the string
/// entries carry). The real inventory the loaded-table test speaks from
/// the ACTUAL cache state (the cache-present boot); the parse law its
/// own test keeps.
#[cfg(test)]
fn test_fixture_table() -> SoundTable {
    use mc_registry::generated::block_sounds as bs;
    let mut ids: std::collections::BTreeSet<&'static str> = std::collections::BTreeSet::new();
    for row in bs::SOUND_HIT
        .iter()
        .chain(bs::SOUND_BREAK.iter())
        .chain(bs::SOUND_PLACE.iter())
        .chain(bs::SOUND_STEP.iter())
        .chain(bs::SOUND_FALL.iter())
    {
        ids.insert(*row);
    }
    for m in mc_registry::generated::worldgen::BIOME_MUSIC {
        ids.insert(m.event);
    }
    for id in [
        "ambient.cave",
        "block.bubble_column.upwards_inside",
        "block.bubble_column.whirlpool_inside",
        "block.glass.break",
        "block.lava.ambient",
        "block.lava.pop",
        "entity.cow.ambient",
        "entity.cow.death",
        "entity.cow.hurt",
        "entity.experience_orb.pickup",
        "entity.generic.burn",
        "entity.generic.drink",
        "entity.generic.eat",
        "entity.generic.extinguish_fire",
        "entity.item.pickup",
        "entity.pig.ambient",
        "entity.pig.death",
        "entity.pig.eat",
        "entity.pig.hurt",
        "entity.player.attack.crit",
        "entity.player.attack.strong",
        "entity.player.attack.sweep",
        "entity.player.attack.weak",
        "entity.player.big_fall",
        "entity.player.burp",
        "entity.player.death",
        "entity.player.hurt",
        "entity.player.hurt_drown",
        "entity.player.hurt_on_fire",
        "entity.player.levelup",
        "entity.player.small_fall",
        "entity.player.splash",
        "entity.player.splash.high_speed",
        "entity.player.swim",
        "entity.sheep.ambient",
        "entity.sheep.death",
        "entity.sheep.hurt",
        "entity.sheep.shear",
        "entity.zombie.ambient",
        "entity.zombie.death",
        "entity.zombie.hurt",
        "entity.zombie.step",
        "item.golden_dandelion.unuse",
        "item.golden_dandelion.use",
        "music.game",
        "music.menu",
        "music.overworld.swamp",
        "music.under_water",
        "item.honey_bottle.drink",
        "ui.button.click",
        "ui.hud.bubble_pop",
    ] {
        ids.insert(id);
    }
    let mut t = SoundTable::default();
    for id in ids {
        let first = t.files.len();
        t.files.push(SoundFileDef {
            name: format!("{id}/a"),
            volume: 1.0,
            pitch: 1.0,
            weight: 1,
            stream: false,
        });
        t.files.push(SoundFileDef {
            name: format!("{id}/b"),
            volume: 1.0,
            pitch: 1.0,
            weight: 1,
            stream: false,
        });
        t.events.push(SoundEventDef {
            id: id.to_string(),
            subtitle: None,
            first,
            len: 2,
        });
    }
    t
}

/// Lookup a sound event by id (the binary_search the old generated fn kept,
/// the sorted order the parse law's BTreeMap holds).
pub fn event(id: &str) -> Option<&'static SoundEventDef> {
    let t = table();
    t.events
        .binary_search_by(|e| e.id.as_str().cmp(id))
        .ok()
        .map(|i| &t.events[i])
}

/// Files of an event (the span the flat table holds).
pub fn files(e: &SoundEventDef) -> &[SoundFileDef] {
    &table().files[e.first..e.first + e.len]
}

/// Every event, the sorted order the old table held (the tracks_for walk).
pub fn events() -> &'static [SoundEventDef] {
    &table().events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sound_table_loader_speaks_the_parse_law() {
        // THE MIGRATION'S OWN LAW (the parse the old generator's emit walk
        // spoke, now the loader's): the string sounds the default folds,
        // the object sounds the explicit fields' unwrap_or arms, the
        // name-less object the skip keeps, the BTreeMap the sorted order
        // the old emitted table held, the broken/non-object bytes the
        // honest empty table keeps
        let t = parse_sounds(
            br#"{
                "ui.button.click": {"sounds":["click1",{"name":"click2","volume":0.5,"pitch":1.2,"weight":3,"stream":true}],"subtitle":"subtitles.ui.click"},
                "ambient.cave": {"sounds":[{"volume":0.5}]},
                "music.menu": {"sounds":["menu1"],"subtitle":42}
            }"#,
        );
        // the sorted order the BTreeMap keeps (the old table's own law)
        let ids: Vec<&str> = t.events.iter().map(|e| e.id.as_str()).collect();
        assert_eq!(ids, ["ambient.cave", "music.menu", "ui.button.click"]);
        // the string sound the default folds keep
        let ui = event_from(&t, "ui.button.click");
        assert_eq!(ui.len, 2, "string + object sounds both land");
        assert_eq!(ui.files_of(&t)[0].name, "click1");
        assert_eq!(ui.files_of(&t)[0].volume, 1.0);
        assert_eq!(ui.files_of(&t)[0].weight, 1);
        assert!(!ui.files_of(&t)[0].stream);
        // the object sound the explicit fields carry
        assert_eq!(ui.files_of(&t)[1].name, "click2");
        assert_eq!(ui.files_of(&t)[1].volume, 0.5);
        assert_eq!(ui.files_of(&t)[1].pitch, 1.2);
        assert_eq!(ui.files_of(&t)[1].weight, 3);
        assert!(ui.files_of(&t)[1].stream);
        assert_eq!(
            ui.subtitle.as_deref(),
            Some("subtitles.ui.click"),
            "the subtitle the option keeps"
        );
        // the name-less object the skip law keeps
        let cave = event_from(&t, "ambient.cave");
        assert_eq!(cave.len, 0, "the name-less entry the skip keeps");
        assert_eq!(cave.first, 0, "the span rides the flat table's own walk");
        // the non-string subtitle the honest None keeps
        assert_eq!(event_from(&t, "music.menu").subtitle, None);
        // the flatten walk: the flat files the spans address
        assert_eq!(t.files.len(), 3, "3 real files, the skipped one lands nowhere");
        // the broken/non-object bytes the honest empty table keeps
        assert!(parse_sounds(b"not json").events.is_empty());
        assert!(parse_sounds(b"[]").events.is_empty());
        assert!(parse_sounds(b"").events.is_empty());
    }

    /// (test helper) the event the parsed table holds.
    fn event_from<'a>(t: &'a SoundTable, id: &str) -> &'a SoundEventDef {
        t.events
            .iter()
            .find(|e| e.id == id)
            .unwrap_or_else(|| panic!("{id} in the parsed table"))
    }

    impl SoundEventDef {
        /// (test helper) the span's files.
        fn files_of<'a>(&self, t: &'a SoundTable) -> &'a [SoundFileDef] {
            &t.files[self.first..self.first + self.len]
        }
    }

    #[test]
    fn the_loaded_table_speaks_the_actual_state() {
        // THE MIGRATION'S OWN LAW (the assert computes the expected from
        // the ACTUAL loaded state — the cache-present boot the official
        // shape answers, the cache-absent boot the empty table the
        // every-lookup's None arm keeps — the CI's pre-generate test job
        // the same arm walks)
        let loaded = !events().is_empty();
        assert_eq!(loaded, event("music.menu").is_some());
        assert_eq!(loaded, event("block.stone.break").is_some());
        // the miss the honest None keeps either way
        assert!(event("not.a.sound").is_none());
        if loaded {
            // the official shape the cache file speaks: the sorted order,
            // the spans inside the flat table, the real pool sizes
            let all = events();
            assert!(all.windows(2).all(|w| w[0].id < w[1].id), "sorted");
            assert!(all.iter().all(|e| e.first + e.len <= files_flat_len()));
            let menu = event("music.menu").unwrap();
            assert!(menu.len > 0, "the menu pool the pick law needs");
        }
    }

    fn files_flat_len() -> usize {
        table().files.len()
    }
}

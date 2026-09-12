//! Music scheduler driven by the GENERATED sound-event tables
//! (official sounds.json via the asset index). Implements the vanilla menu
//! music behavior: pick a track from `music.menu`, then (like the vanilla
//! sound engine) a weighted-random FILE inside that event, play it, then
//! wait a randomized delay before the next one. Per-file lengths come from
//! the REAL decoded .ogg data when the runtime assets are present (see
//! audio.rs); the output backend plugs in on top of this schedule
//! (headless/CI runs the deterministic silent backend).

use std::collections::BTreeMap;

use mc_registry::generated::sounds as snd;

#[derive(Debug, Clone, PartialEq)]
pub enum MusicState {
    Idle,
    /// (event id, start frame, length frames)
    Playing(&'static str, u64, u64),
    /// waiting until this frame
    Delay(u64),
}

/// Which situation the scheduler is serving — vanilla's situational music:
/// the title screen plays `music.menu`, a loaded world plays `music.game`
/// (biome-specific pools are a future step).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicMode {
    Menu,
    Game,
}

/// Table key for one file of an event: "music.menu#3" (index = position in
/// the event's official file list). Shared by the scheduler, the audio
/// loader and the event log so all three stay in lockstep.
pub fn file_key(event: &str, idx: usize) -> String {
    format!("{event}#{idx}")
}

pub struct MusicScheduler {
    mode: MusicMode,
    menu_tracks: Vec<&'static str>,
    game_tracks: Vec<&'static str>,
    state: MusicState,
    rng: u64,
    current: usize,
    tick: u64,
    /// real per-file lengths (frames) from decoded OGG data, keyed file_key()
    file_frames: BTreeMap<String, u64>,
    /// event log (deterministic; tests + audio backend consume it)
    pub events: Vec<String>,
}

/// Fallback file length until real OGG data is loaded (audio.rs injects the
/// decoded lengths): menu tracks are several minutes; the scheduler only
/// needs relative time.
const DEFAULT_TRACK_FRAMES: u64 = 60 * 20;
const MIN_DELAY_FRAMES: u64 = 30 * 20;
const DELAY_SPAN_FRAMES: u64 = 90 * 20;

impl MusicScheduler {
    pub fn new(seed: u64) -> Self {
        // real menu + game tracks from the official sounds.json
        let menu_tracks = Self::tracks_for("music.menu");
        let game_tracks = Self::tracks_for("music.game");
        Self {
            mode: MusicMode::Menu,
            menu_tracks,
            game_tracks,
            state: MusicState::Idle,
            rng: seed ^ 0x5EED_C0FF_EE12_3456,
            current: 0,
            tick: 0,
            file_frames: BTreeMap::new(),
            events: Vec::new(),
        }
    }

    /// Official event ids under a prefix (from the generated sounds.json
    /// table), only events that actually ship files.
    fn tracks_for(prefix: &str) -> Vec<&'static str> {
        snd::SOUND_EVENTS
            .iter()
            .filter(|e| e.id.starts_with(prefix) && e.len > 0)
            .map(|e| e.id)
            .collect()
    }

    pub fn state(&self) -> &MusicState {
        &self.state
    }

    pub fn mode(&self) -> MusicMode {
        self.mode
    }

    /// The official menu track ids (from the generated sounds.json table).
    pub fn menu_tracks(&self) -> &[&'static str] {
        &self.menu_tracks
    }

    /// The official in-game track ids (music.game — the 26.2 overworld pool).
    pub fn game_tracks(&self) -> &[&'static str] {
        &self.game_tracks
    }

    /// Real file length in frames from the decoded official .ogg (key from
    /// file_key()); overrides the fallback default so the schedule matches
    /// the actual audio. Zero/garbage lengths are rejected.
    pub fn set_track_frames(&mut self, key: &str, frames: u64) {
        if frames > 0 {
            self.file_frames.insert(key.to_string(), frames);
        }
    }

    /// The menu opened: start the first track right away (vanilla behavior).
    /// Switches back to the menu situation: any in-game track is interrupted.
    pub fn on_menu_open(&mut self) {
        self.mode = MusicMode::Menu;
        if matches!(self.state, MusicState::Playing(..)) {
            self.events.push(format!("stop @{}", self.tick));
            self.state = MusicState::Idle;
        }
        if !self.menu_tracks.is_empty() {
            self.play_current();
        }
    }

    /// A world was entered: the menu track stops AT ONCE (vanilla interrupts
    /// music on situation change) and game music waits out a randomized
    /// delay before its first track, like the vanilla music engine's gap
    /// between tracks (exact Musics constants remain a javap-recon step).
    pub fn on_world_enter(&mut self) {
        self.mode = MusicMode::Game;
        if matches!(self.state, MusicState::Playing(..)) {
            self.events.push(format!("stop @{}", self.tick));
        }
        let delay = MIN_DELAY_FRAMES + self.next_rand() % DELAY_SPAN_FRAMES;
        self.events.push(format!("delay {} @{}", delay, self.tick));
        self.state = MusicState::Delay(self.tick + delay);
    }

    fn next_rand(&mut self) -> u64 {
        self.rng = self
            .rng
            .wrapping_mul(0x5851_F42D_4C95_7F2D)
            .wrapping_add(0x1405_7B7E_F767_814F);
        self.rng >> 16
    }

    /// Weighted-random file pick, exactly like the vanilla sound engine:
    /// roll over the sum of the official `weight` fields, walk the list.
    /// Returns (file index, file count).
    fn pick_file(&mut self, event: &'static str) -> (usize, usize) {
        match snd::event(event) {
            Some(ev) if ev.len > 0 => {
                let files = snd::files(ev);
                let total: u32 = files.iter().map(|f| f.weight).sum();
                let mut roll = if total > 0 {
                    (self.next_rand() % total as u64) as u32
                } else {
                    0
                };
                for (i, f) in files.iter().enumerate() {
                    if roll < f.weight {
                        return (i, files.len());
                    }
                    roll -= f.weight;
                }
                (files.len() - 1, files.len())
            }
            _ => (0, 1),
        }
    }

    fn play_current(&mut self) {
        let tracks = match self.mode {
            MusicMode::Menu => &self.menu_tracks,
            MusicMode::Game => &self.game_tracks,
        };
        if tracks.is_empty() {
            return;
        }
        let track = tracks[self.current % tracks.len()];
        let (file_idx, _) = self.pick_file(track);
        let key = file_key(track, file_idx);
        let len = self.file_frames.get(&key).copied().unwrap_or(DEFAULT_TRACK_FRAMES);
        self.events.push(format!("play {key} @{}", self.tick));
        self.state = MusicState::Playing(track, self.tick, len);
    }

    fn advance_track(&mut self) {
        self.current += 1;
        let delay = MIN_DELAY_FRAMES + self.next_rand() % DELAY_SPAN_FRAMES;
        self.events.push(format!("delay {} @{}", delay, self.tick));
        self.state = MusicState::Delay(self.tick + delay);
    }

    /// Tick one frame.
    pub fn update(&mut self) {
        self.tick += 1;
        match self.state {
            MusicState::Idle => {}
            MusicState::Playing(_, start, len) => {
                if self.tick - start >= len {
                    self.advance_track();
                }
            }
            MusicState::Delay(until) => {
                if self.tick >= until {
                    self.play_current();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parse "play <key> @<tick>" -> the file key ("music.menu#4").
    fn played_key(s: &MusicScheduler) -> String {
        let ev = s
            .events
            .iter()
            .find(|e| e.starts_with("play "))
            .expect("a play event");
        ev.strip_prefix("play ")
            .and_then(|s| s.split(" @").next())
            .expect("file key")
            .to_string()
    }

    #[test]
    fn menu_tracks_come_from_official_sounds_json() {
        let s = MusicScheduler::new(1);
        // 26.2 ships music.menu as one event with 9 weighted files
        assert!(
            s.menu_tracks.iter().any(|t| t.starts_with("music.menu")),
            "official sounds.json must provide music.menu events: {:?}",
            s.menu_tracks
        );
    }

    #[test]
    fn menu_open_starts_playing() {
        let mut s = MusicScheduler::new(42);
        assert_eq!(s.state(), &MusicState::Idle);
        s.on_menu_open();
        match s.state() {
            MusicState::Playing(t, _, _) => assert!(t.starts_with("music.menu")),
            other => panic!("expected playing, got {:?}", other),
        }
        assert!(
            s.events.first().map(String::as_str).unwrap_or("").starts_with("play music.menu#"),
            "event log must record the picked file: {:?}",
            s.events
        );
    }

    #[test]
    fn game_tracks_come_from_official_sounds_json() {
        let s = MusicScheduler::new(1);
        // 26.2 ships one `music.game` event — the 32-file overworld pool
        assert_eq!(s.game_tracks(), &["music.game"] as &[&str]);
        assert!(
            !s.game_tracks.is_empty() && s.game_tracks[0].starts_with("music.game"),
            "official sounds.json must provide music.game: {:?}",
            s.game_tracks
        );
    }

    #[test]
    fn world_enter_stops_menu_and_schedules_game_music() {
        let mut s = MusicScheduler::new(42);
        s.on_menu_open();
        assert!(matches!(s.state(), MusicState::Playing(t, _, _) if t.starts_with("music.menu")));
        s.on_world_enter();
        assert_eq!(s.mode(), MusicMode::Game);
        // the menu track was interrupted at once...
        assert!(
            s.events.iter().any(|e| e.starts_with("stop")),
            "entering a world stops the playing menu track: {:?}",
            s.events
        );
        // ...and game music waits out its randomized first delay
        assert!(matches!(s.state(), MusicState::Delay(_)), "{:?}", s.state());
        for _ in 0..(MIN_DELAY_FRAMES + DELAY_SPAN_FRAMES + 1) {
            s.update();
        }
        match s.state() {
            MusicState::Playing(t, _, _) => assert!(
                t.starts_with("music.game"),
                "in-game situation must play music.game, got {t}"
            ),
            other => panic!("expected game music playing, got {:?}", other),
        }
    }

    #[test]
    fn returning_to_title_plays_menu_again() {
        let mut s = MusicScheduler::new(42);
        s.on_world_enter();
        for _ in 0..(MIN_DELAY_FRAMES + DELAY_SPAN_FRAMES + 1) {
            s.update();
        }
        assert!(matches!(s.state(), MusicState::Playing(t, _, _) if t.starts_with("music.game")));
        let stops_before = s.events.iter().filter(|e| e.starts_with("stop")).count();
        s.on_menu_open();
        // the game track is interrupted and a menu track starts immediately
        assert_eq!(s.mode(), MusicMode::Menu);
        assert_eq!(
            s.events.iter().filter(|e| e.starts_with("stop")).count(),
            stops_before + 1,
            "returning to title stops the game track: {:?}",
            s.events
        );
        assert!(matches!(s.state(), MusicState::Playing(t, _, _) if t.starts_with("music.menu")));
    }

    #[test]
    fn weighted_file_pick_stays_in_range() {
        // a fixed seed must pick a legal file index of the official pool
        let mut s = MusicScheduler::new(20260214);
        s.on_menu_open();
        let key = played_key(&s);
        let idx: usize = key.split('#').nth(1).expect("#idx").parse().expect("idx");
        let ev = snd::event("music.menu").expect("music.menu event");
        assert!(idx < ev.len, "picked file {idx} out of range (pool {})", ev.len);
        // determinism: same seed, same pick
        let mut s2 = MusicScheduler::new(20260214);
        s2.on_menu_open();
        assert_eq!(played_key(&s2), key);
    }

    #[test]
    fn track_ends_then_delay_then_next() {
        let mut s = MusicScheduler::new(1);
        s.on_menu_open();
        for _ in 0..(DEFAULT_TRACK_FRAMES + 1) {
            s.update();
        }
        assert!(matches!(s.state(), MusicState::Delay(_)), "after track length a delay starts: {:?}", s.state());
        let delay_events = s.events.iter().filter(|e| e.starts_with("delay")).count();
        assert_eq!(delay_events, 1);
        // fast-forward past the max possible delay: a new track must start
        for _ in 0..(MIN_DELAY_FRAMES + DELAY_SPAN_FRAMES + 1) {
            s.update();
        }
        assert!(matches!(s.state(), MusicState::Playing(_, _, _)), "next track plays after delay: {:?}", s.state());
        assert_eq!(s.events.iter().filter(|e| e.starts_with("play")).count(), 2);
    }

    #[test]
    fn determinism_same_seed_same_schedule() {
        let mut a = MusicScheduler::new(99);
        let mut b = MusicScheduler::new(99);
        a.on_menu_open();
        b.on_menu_open();
        for _ in 0..64 {
            a.update();
            b.update();
        }
        assert_eq!(a.events, b.events);
    }

    #[test]
    fn real_file_length_from_decoded_ogg_overrides_default() {
        let mut s = MusicScheduler::new(7);
        // the audio backend injects the real decoded length for EVERY file of
        // the event (it cannot know which one the weighted pick will choose)
        let id = s.menu_tracks()[0];
        let ev = snd::event(id).expect("event");
        for i in 0..ev.len {
            s.set_track_frames(&file_key(id, i), 132_300_000);
        }
        s.on_menu_open();
        match s.state() {
            MusicState::Playing(t, start, len) => {
                assert_eq!(*t, id);
                assert_eq!(*start, 0);
                assert_eq!(*len, 132_300_000, "real decoded length must be used");
            }
            other => panic!("expected playing, got {:?}", other),
        }
        // zero/garbage lengths are rejected (never create instant loops)
        for i in 0..ev.len {
            s.set_track_frames(&file_key(id, i), 0);
        }
        match s.state() {
            MusicState::Playing(_, _, len) => assert_eq!(*len, 132_300_000),
            other => panic!("expected still playing, got {:?}", other),
        }
    }
}

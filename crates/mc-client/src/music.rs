//! Music scheduler driven by the GENERATED sound-event tables
//! (official sounds.json via the asset index). Implements the vanilla menu
//! music behavior: pick a track from `music.menu`, play it, then wait a
//! randomized delay before the next one. Track lengths come from the REAL
//! decoded .ogg data when the runtime assets are present (see audio.rs);
//! the output backend is injected on top of this schedule (headless/CI runs
//! the deterministic silent backend).

use std::collections::BTreeMap;

use mc_registry::generated::sounds as snd;

#[derive(Debug, Clone, PartialEq)]
pub enum MusicState {
    Idle,
    /// (track name, start frame, length frames)
    Playing(&'static str, u64, u64),
    /// waiting until this frame
    Delay(u64),
}

pub struct MusicScheduler {
    menu_tracks: Vec<&'static str>,
    state: MusicState,
    rng: u64,
    current: usize,
    tick: u64,
    /// real track lengths (frames) from decoded OGG data, keyed by event id
    track_frames: BTreeMap<&'static str, u64>,
    /// event log (deterministic; tests + audio backend consume it)
    pub events: Vec<String>,
}

/// Fallback track length until real OGG data is loaded (audio.rs injects
/// the decoded length): menu tracks are several minutes; the scheduler only
/// needs relative time.
const DEFAULT_TRACK_FRAMES: u64 = 60 * 20;
const MIN_DELAY_FRAMES: u64 = 30 * 20;
const DELAY_SPAN_FRAMES: u64 = 90 * 20;

impl MusicScheduler {
    pub fn new(seed: u64) -> Self {
        // real menu tracks from the official sounds.json
        let menu_tracks = snd::SOUND_EVENTS
            .iter()
            .filter(|e| e.id.starts_with("music.menu") && e.len > 0)
            .map(|e| e.id)
            .collect();
        Self {
            menu_tracks,
            state: MusicState::Idle,
            rng: seed ^ 0x5EED_C0FF_EE12_3456,
            current: 0,
            tick: 0,
            track_frames: BTreeMap::new(),
            events: Vec::new(),
        }
    }

    pub fn state(&self) -> &MusicState {
        &self.state
    }

    /// The official menu track ids (from the generated sounds.json table).
    pub fn menu_tracks(&self) -> &[&'static str] {
        &self.menu_tracks
    }

    /// Real track length in frames from the decoded official .ogg; overrides
    /// the fallback default so the schedule matches the actual audio.
    pub fn set_track_frames(&mut self, id: &'static str, frames: u64) {
        if frames > 0 {
            self.track_frames.insert(id, frames);
        }
    }

    /// The menu opened: start the first track right away (vanilla behavior).
    pub fn on_menu_open(&mut self) {
        if matches!(self.state, MusicState::Idle) && !self.menu_tracks.is_empty() {
            self.play_current();
        }
    }

    fn next_rand(&mut self) -> u64 {
        self.rng = self
            .rng
            .wrapping_mul(0x5851_F42D_4C95_7F2D)
            .wrapping_add(0x1405_7B7E_F767_814F);
        self.rng >> 16
    }

    fn play_current(&mut self) {
        if self.menu_tracks.is_empty() {
            return;
        }
        let track = self.menu_tracks[self.current % self.menu_tracks.len()];
        let len = self.track_frames.get(track).copied().unwrap_or(DEFAULT_TRACK_FRAMES);
        self.events.push(format!("play {} @{}", track, self.tick));
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

    #[test]
    fn menu_tracks_come_from_official_sounds_json() {
        let s = MusicScheduler::new(1);
        // 26.2 ships music.menu tracks (menu1..menu4 + stacked variants)
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
        assert_eq!(s.events.first().map(String::as_str), Some("play music.menu @0"));
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
    fn real_track_length_from_decoded_ogg_overrides_default() {
        let mut s = MusicScheduler::new(7);
        let id = s.menu_tracks()[0];
        // the audio backend injects the real decoded length (e.g. ~3 min at
        // 44.1 kHz) and the schedule must honor it, not the default
        s.set_track_frames(id, 132_300_000);
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
        s.set_track_frames(id, 0);
        match s.state() {
            MusicState::Playing(_, _, len) => assert_eq!(*len, 132_300_000),
            other => panic!("expected still playing, got {:?}", other),
        }
    }
}

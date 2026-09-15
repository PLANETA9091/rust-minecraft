//! THE BLOCK-SOUND LANE — the official SoundType laws over the generated
//! table, the deterministic one-shot engine the interaction lanes speak.
//!
//! Every fold below is javap of the real 26.2 classes (the same evidence
//! chain the dig lane rides):
//! - the DIG BEAT: `MultiPlayerGameMode.continueDestroyBlock` — the hit
//!   sound plays when `destroyTicks % 4.0F == 0.0F` (checked BEFORE the
//!   tick increments — beats at ticks 0, 4, 8, ...), at
//!   `(getVolume() + 1.0F) / 8.0F` volume and `getPitch() * 0.5F` pitch.
//! - the BREAK: `BlockBehaviour.playerWillDestroy` -> `level.levelEvent(
//!   2001, ...)` -> `LevelEventHandler.levelEvent` case 2001 — the break
//!   sound at `(getVolume() + 1.0F) / 2.0F` volume and `getPitch() * 0.8F`
//!   pitch (the same fold the placement speaks).
//! - the PLACE: `BlockItem.place` — `level.playSound(...,
//!   soundtype.getPlaceSound(), BLOCKS, (getVolume() + 1.0F) / 2.0F,
//!   getPitch() * 0.8F)`.
//! - the FOOTSTEP: `Entity.playStepSound` — the step sound at
//!   `getVolume() * 0.15F` volume and `getPitch()` pitch; the cadence is
//!   `Entity.move`'s own: `moveDist += horizontalDistance * 0.6` (the
//!   climbable branch's full-length lane aside — our vocabulary climbs
//!   nothing yet), the step when `moveDist > nextStep`, with `nextStep()`
//!   resetting to `(int)moveDist + 1.0F` — one footstep per ~1.667 blocks
//!   of horizontal travel.
//!
//! The engine is the music scheduler's own shape: a seeded LCG (the same
//! constants music.rs speaks), the vanilla sound engine's weighted file
//! pick inside the event (the official `weight` fields), and an event log
//! the real audio backend mirrors ("block.stone.hit @<tick> #<file>
//! v<volume> p<pitch>"). Headless runs stay silent and deterministic; the
//! Rodio path decodes the official .ogg the log names, folds the gain and
//! the pitch resample in, and hands the one-shot to the output.

use mc_registry::generated::block_sounds as bs;
use mc_registry::generated::sounds as snd;

/// Which vanilla voice a lane speaks (the SoundType event the law plays).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lane {
    /// the dig beat's hit (`getHitSound()`)
    Hit,
    /// the completion's break (`getBreakSound()`, the 2001 event)
    Break,
    /// the placement (`getPlaceSound()`)
    Place,
    /// the footsteps (`getStepSound()`)
    Step,
}

impl Lane {
    /// The event id this lane speaks for a block row (the generated
    /// parallel arrays, indexed through `bs::sound_index`).
    pub fn event_of(self, idx: usize) -> &'static str {
        match self {
            Lane::Hit => bs::SOUND_HIT[idx],
            Lane::Break => bs::SOUND_BREAK[idx],
            Lane::Place => bs::SOUND_PLACE[idx],
            Lane::Step => bs::SOUND_STEP[idx],
        }
    }

    /// The law's fold: the SoundType's base (volume, pitch) into the
    /// final one-shot numbers. The javap-quoted constants above.
    pub fn fold(self, volume: f32, pitch: f32) -> (f32, f32) {
        match self {
            Lane::Hit => ((volume + 1.0) / 8.0, pitch * 0.5),
            Lane::Break | Lane::Place => ((volume + 1.0) / 2.0, pitch * 0.8),
            Lane::Step => (volume * 0.15, pitch),
        }
    }
}

/// The dig beat's own law: `destroyTicks % 4.0F == 0.0F`, checked before
/// the tick increments (javap MultiPlayerGameMode.continueDestroyBlock
/// offsets 197-207: frem 4.0f, fcmpl, ifne). dig.rs's machine keeps the
/// counter; the caller tests the beat BEFORE the tick.
pub fn beat_due(ticks: f32) -> bool {
    ticks % 4.0 == 0.0
}

/// The footstep cadence's own law: `nextStep() = (int)moveDist + 1.0F`
/// (javap Entity.nextStep: f2i, iconst_1, iadd) — the reset after each
/// step; the step itself fires when `moveDist > nextStep` with the block
/// under the feet (getOnPos: floor(x), floor(y - 0.2), floor(z)) not air.
pub fn next_step(move_dist: f32) -> f32 {
    (move_dist as i32) as f32 + 1.0
}

/// The deterministic block-sound engine: the log the tests pin and the
/// audio backend mirrors. The RNG is the music scheduler's own LCG (the
/// same seed scramble, the same step constants) — the whole audio lane
/// speaks one generator.
pub struct BlockSounds {
    rng: u64,
    /// one entry per played one-shot:
    /// `"{event} @{tick} #{file} v{volume} p{pitch}"`
    pub events: Vec<String>,
}

impl BlockSounds {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: seed ^ 0x5EED_C0FF_EE12_3456,
            events: Vec::new(),
        }
    }

    fn next_rand(&mut self) -> u64 {
        self.rng = self
            .rng
            .wrapping_mul(0x5851_F42D_4C95_7F2D)
            .wrapping_add(0x1405_7B7E_F767_814F);
        self.rng >> 16
    }

    /// Weighted-random file pick, exactly like the vanilla sound engine
    /// (and music.rs's own pick): roll over the sum of the official
    /// `weight` fields, walk the list. Returns the file index.
    fn pick_file(&mut self, event: &'static str) -> Option<usize> {
        let ev = snd::event(event)?;
        let files = snd::files(ev);
        if files.is_empty() {
            return None;
        }
        let total: u32 = files.iter().map(|f| f.weight).sum();
        let mut roll = if total > 0 {
            (self.next_rand() % total as u64) as u32
        } else {
            0
        };
        for (i, f) in files.iter().enumerate() {
            if roll < f.weight {
                return Some(i);
            }
            roll -= f.weight;
        }
        Some(files.len() - 1)
    }

    /// Play one block voice: the block's name (the palette's bare word)
    /// resolves through the generated table, the lane folds the law, the
    /// weighted pick names the official file, the log records it all.
    /// Unknown blocks or events (rows speaking "none") stay silent — the
    /// vanilla never hears them either.
    pub fn play(&mut self, tick: u64, block: &str, lane: Lane) {
        let Some(idx) = bs::sound_index(block) else {
            return;
        };
        let event = lane.event_of(idx);
        let (volume, pitch) = lane.fold(bs::SOUND_VOLUME[idx], bs::SOUND_PITCH[idx]);
        let Some(file) = self.pick_file(event) else {
            return;
        };
        self.events.push(format!(
            "{event} @{tick} #{file} v{:.6} p{:.6}",
            volume, pitch
        ));
    }

    /// The pickup voice (javap ClientPacketListener.handleTakeItemEntity's
    /// ITEM_PICKUP branch): the `entity.item.pickup` event at the
    /// PLAYERS source, volume 0.2, pitch `(r - r) * 1.4 + 2.0` — the two
    /// randoms ride THIS engine's LCG (the same generator the weighted
    /// pick speaks; the draw order: pitch randoms first, then the file
    /// pick). The log rides the same format the block voices speak (the
    /// audio backend mirrors it untouched).
    pub fn play_pickup(&mut self, tick: u64) {
        let r1 = self.next_rand_f32();
        let r2 = self.next_rand_f32();
        let pitch = crate::drops::pickup_pitch(r1, r2);
        let Some(file) = self.pick_file("entity.item.pickup") else {
            return;
        };
        self.events.push(format!(
            "entity.item.pickup @{tick} #{file} v{:.6} p{:.6}",
            crate::drops::PICKUP_VOLUME, pitch
        ));
    }

    /// A [0,1) float from the engine's own LCG (the pickup pitch's two
    /// draws speak it — the same 24-bit shape the file pick rolls).
    fn next_rand_f32(&mut self) -> f32 {
        (self.next_rand() % (1u64 << 24)) as f32 / (1u64 << 24) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_laws_fold_the_javap_constants() {
        // the base pairs the dump speaks for the big families (1.0, 1.0):
        // the hit lands at 0.25 volume / 0.5 pitch, the break and the
        // place at 1.0 / 0.8, the step at 0.15 / 1.0 — the vanilla's own
        // numbers through the folds
        let (v, p) = Lane::Hit.fold(1.0, 1.0);
        assert_eq!((v, p), (0.25, 0.5));
        let (v, p) = Lane::Break.fold(1.0, 1.0);
        assert_eq!((v, p), (1.0, 0.8));
        let (v, p) = Lane::Place.fold(1.0, 1.0);
        assert_eq!((v, p), (1.0, 0.8));
        let (v, p) = Lane::Step.fold(1.0, 1.0);
        assert_eq!((v, p), (0.15, 1.0));
        // a non-trivial base rides the algebra, not a constant
        let (v, p) = Lane::Hit.fold(0.5, 1.25);
        assert_eq!((v, p), (1.5 / 8.0, 0.625));
    }

    #[test]
    fn the_beat_and_the_cadence_speak_the_bytecode() {
        // beats at ticks 0, 4, 8 — never 3 or 5
        for t in [0.0f32, 4.0, 8.0, 12.0] {
            assert!(beat_due(t), "tick {t}");
        }
        for t in [1.0f32, 2.0, 3.0, 5.0, 7.0, 149.0] {
            assert!(!beat_due(t), "tick {t}");
        }
        // nextStep resets one whole unit past the walked distance
        assert_eq!(next_step(0.0), 1.0);
        assert_eq!(next_step(0.132), 1.0);
        assert_eq!(next_step(1.0), 2.0);
        assert_eq!(next_step(2.66), 3.0);
    }

    #[test]
    fn the_pickup_voice_speaks_the_take_law() {
        // the ITEM_PICKUP fold: volume 0.2, the pitch in (r-r)*1.4+2.0's
        // own range [0.6, 3.4], the log format the backend mirrors
        let mut a = BlockSounds::new(31);
        a.play_pickup(9);
        assert_eq!(a.events.len(), 1);
        let e = &a.events[0];
        assert!(e.starts_with("entity.item.pickup @9 #"), "the entry: {e}");
        assert!(e.contains(" v0.200000 p"), "the volume: {e}");
        let pitch: f32 = e.split(" p").nth(1).unwrap().parse().unwrap();
        assert!((0.6..=3.4).contains(&pitch), "the pitch range: {pitch}");
        // determinism: the same seed speaks the same voice
        let mut b = BlockSounds::new(31);
        b.play_pickup(9);
        assert_eq!(a.events, b.events);
    }

    #[test]
    fn stone_voices_resolve_through_the_table() {
        let idx = bs::sound_index("stone").expect("stone in the table");
        assert_eq!(Lane::Hit.event_of(idx), "block.stone.hit");
        assert_eq!(Lane::Break.event_of(idx), "block.stone.break");
        assert_eq!(Lane::Place.event_of(idx), "block.stone.place");
        assert_eq!(Lane::Step.event_of(idx), "block.stone.step");
        // dirt speaks the GRAVEL SoundType (the vanilla's own pairing)
        let idx = bs::sound_index("dirt").expect("dirt in the table");
        assert_eq!(Lane::Step.event_of(idx), "block.gravel.step");
    }

    #[test]
    fn the_engine_logs_the_law_and_stays_deterministic() {
        let mut a = BlockSounds::new(7);
        let mut b = BlockSounds::new(7);
        for _ in 0..4 {
            a.play(3, "stone", Lane::Hit);
            b.play(3, "stone", Lane::Hit);
        }
        assert_eq!(a.events, b.events, "seeded runs stay identical");
        assert_eq!(a.events.len(), 4);
        let e = &a.events[0];
        assert!(
            e.starts_with("block.stone.hit @3 #"),
            "the log names the event, tick and file: {e}"
        );
        // the fold's numbers ride the entry (0.25 volume, 0.5 pitch)
        assert!(e.contains(" v0.250000 p0.500000"), "the entry: {e}");
        // unknown blocks never speak
        let mut c = BlockSounds::new(7);
        c.play(0, "not_a_block", Lane::Hit);
        assert!(c.events.is_empty());
        // a weighted pick never leaves the official range
        let mut d = BlockSounds::new(9);
        for _ in 0..32 {
            d.play(0, "glass", Lane::Break);
        }
        let ev = snd::event("block.glass.break").expect("the event exists");
        for line in &d.events {
            let file: usize = line
                .split(" #")
                .nth(1)
                .and_then(|s| s.split(' ').next())
                .and_then(|s| s.parse().ok())
                .expect("the log carries the file index");
            assert!(file < ev.len, "file {file} inside {} files", ev.len);
        }
    }
}

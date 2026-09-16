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
use crate::sound_table as snd;

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
    /// THE MUFFLED FOOTSTEP (playMuffledStepSound's own fold — the
    /// wading step's block voice): the same getStepSound() event at
    /// the *0.05/*0.8 fold, the water's own dampening the vanilla
    /// speaks (javap Entity.playMuffledStepSound: getVolume() * 0.05F,
    /// getPitch() * 0.8F).
    StepMuffled,
}

impl Lane {
    /// The event id this lane speaks for a block row (the generated
    /// parallel arrays, indexed through `bs::sound_index`).
    pub fn event_of(self, idx: usize) -> &'static str {
        match self {
            Lane::Hit => bs::SOUND_HIT[idx],
            Lane::Break => bs::SOUND_BREAK[idx],
            Lane::Place => bs::SOUND_PLACE[idx],
            Lane::Step | Lane::StepMuffled => bs::SOUND_STEP[idx],
        }
    }

    /// The law's fold: the SoundType's base (volume, pitch) into the
    /// final one-shot numbers. The javap-quoted constants above.
    pub fn fold(self, volume: f32, pitch: f32) -> (f32, f32) {
        match self {
            Lane::Hit => ((volume + 1.0) / 8.0, pitch * 0.5),
            Lane::Break | Lane::Place => ((volume + 1.0) / 2.0, pitch * 0.8),
            Lane::Step => (volume * 0.15, pitch),
            // THE WADER'S MUFFLE (javap Entity.playMuffledStepSound
            // offsets 5-29: getVolume() * 0.05F, getPitch() * 0.8F —
            // the same getStepSound() event, the water's own quiet)
            Lane::StepMuffled => (volume * 0.05, pitch * 0.8),
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
    fn pick_file(&mut self, event: &str) -> Option<usize> {
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

    /// The ORB pickup voice (javap ClientPacketListener.handleTakeItemEntity's
    /// EXPERIENCE_ORB_PICKUP branch — the client's own playback the take
    /// packet's orb arm speaks): the `entity.experience_orb.pickup` event
    /// at the PLAYERS source, volume 0.1, pitch `0.9 + (r - r) * 0.35` —
    /// the two randoms ride THIS engine's LCG (the draw order: the pitch
    /// randoms first, then the file pick — the pickup voice's own order).
    pub fn play_orb_pickup(&mut self, tick: u64) {
        let r1 = self.next_rand_f32();
        let r2 = self.next_rand_f32();
        let pitch = 0.9 + (r1 - r2) * 0.35;
        let Some(file) = self.pick_file("entity.experience_orb.pickup") else {
            return;
        };
        self.events.push(format!(
            "entity.experience_orb.pickup @{tick} #{file} v{:.6} p{:.6}",
            0.1, pitch
        ));
    }

    /// A [0,1) float from the engine's own LCG (the pickup pitch's two
    /// draws speak it — the same 24-bit shape the file pick rolls).
    fn next_rand_f32(&mut self) -> f32 {
        (self.next_rand() % (1u64 << 24)) as f32 / (1u64 << 24) as f32
    }

    /// The FALL VOICE (javap `LivingEntity.causeFallDamage`'s own
    /// call): `playSound(getFallDamageSound(damage), 1.0F, 1.0F)` —
    /// the FIXED volume and pitch lane (no randoms, the fall's own
    /// deadpan thud), the small fall under five damage, the big one
    /// at and past it (`getFallDamageSound(i): i > 4 ? big : small`,
    /// the Player's fallsounds `PLAYER_SMALL_FALL` / `PLAYER_BIG_FALL`).
    pub fn play_fall_voice(&mut self, tick: u64, damage: i32) {
        let event = if damage > 4 {
            "entity.player.big_fall"
        } else {
            "entity.player.small_fall"
        };
        let Some(file) = self.pick_file(event) else {
            return;
        };
        self.events.push(format!(
            "{event} @{tick} #{file} v{:.6} p{:.6}",
            1.0, 1.0
        ));
    }

    /// The HURT VOICE (javap `LivingEntity.handleDamageEvent`'s own
    /// call — the client's own damage law): `playSound(getHurtSound(),
    /// getSoundVolume(), (r - r) * 0.2F + 1.0F)` — volume 1.0, the
    /// pitch `(r - r) * 0.2 + 1.0` in [0.8, 1.2], the two randoms
    /// riding THIS engine's LCG (the draw order: the pitch randoms
    /// first, then the file pick — the pickup voice's own order).
    pub fn play_hurt_voice(&mut self, tick: u64) {
        let r1 = self.next_rand_f32();
        let r2 = self.next_rand_f32();
        let pitch = (r1 - r2) * 0.2 + 1.0;
        let Some(file) = self.pick_file("entity.player.hurt") else {
            return;
        };
        self.events.push(format!(
            "entity.player.hurt @{tick} #{file} v{:.6} p{:.6}",
            1.0, pitch
        ));
    }

    /// The DEATH VOICE (javap the hurtServer tail's `makeSound(
    /// getDeathSound())` — the Player's own `entity.player.death`):
    /// the volume 1.0, the pitch the voice's (r - r) * 0.2 + 1.0 law,
    /// the same LCG and draw order the hurt voice speaks.
    pub fn play_death_voice(&mut self, tick: u64) {
        let r1 = self.next_rand_f32();
        let r2 = self.next_rand_f32();
        let pitch = (r1 - r2) * 0.2 + 1.0;
        let Some(file) = self.pick_file("entity.player.death") else {
            return;
        };
        self.events.push(format!(
            "entity.player.death @{tick} #{file} v{:.6} p{:.6}",
            1.0, pitch
        ));
    }

    /// THE SWIM VOICE (javap `Entity.playSwimSound(float)` — both the
    /// wading step's override branch and the drift crossing speak it):
    /// `playSound(getSwimSound(), volume, 1.0F + (r - r) * 0.4F)` — the
    /// Player's own `entity.player.swim` event (the PLAYER_SWIM the
    /// getSwimSound override names), the pitch (r - r) * 0.4 + 1.0 in
    /// [0.6, 1.4], the two randoms riding THIS engine's LCG (the draw
    /// order: the pitch randoms first, then the file pick — the hurt
    /// voice's own order). The VOLUME is the caller's speed fold (the
    /// waterSwimSound's own min(1.0, speed * 0.35) law the body's tick
    /// speaks).
    pub fn play_swim(&mut self, tick: u64, volume: f32) {
        let r1 = self.next_rand_f32();
        let r2 = self.next_rand_f32();
        let pitch = (r1 - r2) * 0.4 + 1.0;
        let Some(file) = self.pick_file("entity.player.swim") else {
            return;
        };
        self.events.push(format!(
            "entity.player.swim @{tick} #{file} v{:.6} p{:.6}",
            volume, pitch
        ));
    }

    /// THE ENTRY SPLASH (javap `Entity.doWaterSplashEffect` — fired
    /// through updateFluidInteraction's gate inWater &&
    /// !wasTouchingWater && !firstTick): the water's big welcome, the
    /// speed picks the variant — volume < 0.25 speaks the plain
    /// `entity.player.splash` (getSwimSplashSound's own PLAYER_SPLASH
    /// override), volume >= 0.25 the high-speed one
    /// (getSwimHighSpeedSplashSound's PLAYER_SPLASH_HIGH_SPEED). The
    /// pitch 1.0 + (r - r) * 0.4F in [0.6, 1.4] both ways, the two
    /// randoms riding THIS engine's LCG (the draw order: the pitch
    /// randoms first, then the file pick — the hurt voice's own
    /// order). The VOLUME is the caller's speed fold (the self base
    /// 0.2F the bytecode speaks).
    pub fn play_splash(&mut self, tick: u64, volume: f32, high_speed: bool) {
        let r1 = self.next_rand_f32();
        let r2 = self.next_rand_f32();
        let pitch = (r1 - r2) * 0.4 + 1.0;
        let event = if high_speed {
            "entity.player.splash.high_speed"
        } else {
            "entity.player.splash"
        };
        let Some(file) = self.pick_file(event) else {
            return;
        };
        self.events.push(format!(
            "{event} @{tick} #{file} v{:.6} p{:.6}",
            volume, pitch
        ));
    }

    /// THE BURN VOICE (javap `Entity.lavaHurt`'s own tail — the
    /// playSound inside the `hurtServer(lava(), 4.0F)` SUCCESS branch):
    /// `playSound(GENERIC_BURN, 0.4F, 2.0F + nextFloat() * 0.4F)` —
    /// the fixed 0.4 volume, the pitch 2.0 + r * 0.4 in [2.0, 2.4],
    /// ONE random (the single nextFloat the bytecode speaks — not the
    /// (r - r) pair the hurt voice folds). The draw order: the pitch
    /// random first, then the file pick — the engine's own two draws.
    pub fn play_burn(&mut self, tick: u64) {
        let r = self.next_rand_f32();
        let pitch = 2.0 + r * 0.4;
        let Some(file) = self.pick_file("entity.generic.burn") else {
            return;
        };
        self.events.push(format!(
            "entity.generic.burn @{tick} #{file} v{:.6} p{:.6}",
            0.4, pitch
        ));
    }

    /// THE EXTINGUISH FIZZ (javap
    /// `playEntityOnFireExtinguishedSound` — the water douse's own
    /// transition): `playSound(ENTITY_GENERIC_EXTINGUISH_FIRE,
    /// 0.7F, 1.6F + (r - r) * 0.4F)` — the volume 0.7, the pitch
    /// 1.6 + (r - r) * 0.4 in [1.2, 2.0], the two randoms riding THIS
    /// engine's LCG (the same constants the mob lane's
    /// EXTINGUISH_* names — one law, the javap's own numbers).
    pub fn play_extinguish(&mut self, tick: u64) {
        let r1 = self.next_rand_f32();
        let r2 = self.next_rand_f32();
        let pitch =
            crate::mob::EXTINGUISH_PITCH_BASE + (r1 - r2) * crate::mob::EXTINGUISH_PITCH_SPREAD;
        let Some(file) = self.pick_file("entity.generic.extinguish_fire") else {
            return;
        };
        self.events.push(format!(
            "entity.generic.extinguish_fire @{tick} #{file} v{:.6} p{:.6}",
            crate::mob::EXTINGUISH_VOLUME, pitch
        ));
    }

    /// THE BURNING HURT VOICE (javap the 26.2 `Player.getHurtSound`
    /// override's own shape — `source.type().effects().sound()`:
    /// the DamageEffects.BURNING constant carries PLAYER_HURT_ON_FIRE,
    /// the voice BOTH the lava and the onFire damage types speak):
    /// `playSound(getHurtSound(), getSoundVolume(), (r - r) * 0.2F +
    /// 1.0F)` — volume 1.0, the pitch (r - r) * 0.2 + 1.0 in
    /// [0.8, 1.2], the same LCG and draw order the plain hurt voice
    /// speaks, the on-fire event the burning sources name.
    pub fn play_hurt_on_fire_voice(&mut self, tick: u64) {
        let r1 = self.next_rand_f32();
        let r2 = self.next_rand_f32();
        let pitch = (r1 - r2) * 0.2 + 1.0;
        let Some(file) = self.pick_file("entity.player.hurt_on_fire") else {
            return;
        };
        self.events.push(format!(
            "entity.player.hurt_on_fire @{tick} #{file} v{:.6} p{:.6}",
            1.0, pitch
        ));
    }

    /// THE DROWNING VOICE (javap the baseTick air branch's own tail —
    /// the `hurtServer(level, drown(), 2.0F)` the getHurtSound
    /// override answers through the damage type's effects:
    /// DamageEffects.DROWNING names the entity.player.hurt_drown
    /// event): the volume 1.0, the pitch (r - r) * 0.2 + 1.0 in
    /// [0.8, 1.2], the same LCG and draw order the plain hurt voice
    /// speaks, the drowning event the water's own dose names.
    pub fn play_hurt_drown_voice(&mut self, tick: u64) {
        let r1 = self.next_rand_f32();
        let r2 = self.next_rand_f32();
        let pitch = (r1 - r2) * 0.2 + 1.0;
        let Some(file) = self.pick_file("entity.player.hurt_drown") else {
            return;
        };
        self.events.push(format!(
            "entity.player.hurt_drown @{tick} #{file} v{:.6} p{:.6}",
            1.0, pitch
        ));
    }

    /// THE BUBBLE POP (javap `Hud.playAirBubblePoppedSound` — the
    /// bursting sprite's own companion): `playSound(BUBBLE_POP,
    /// 0.5F + 0.1F * max(0, empty - 2), 1.0F + 0.1F * max(0, empty -
    /// 4))` — the constants NUM_AIR_BUBBLE_POPPED_BEFORE_SOUND_-
    /// VOLUME_INCREASE 3 / _PITCH_INCREASE 5 the -3+1/-5+1 folds
    /// speak, no randoms (the fixed fold playSound's own shape), the
    /// ui.hud.bubble_pop event the BUBBLE_POP constant names. The
    /// caller's dedup (the lastBubblePopSoundPlayed law) rides
    /// upstream — one voice per popped bubble index.
    pub fn play_bubble_pop(&mut self, tick: u64, empty_air_bubbles: i32) {
        let volume = 0.5 + 0.1 * (empty_air_bubbles - 2).max(0) as f32;
        let pitch = 1.0 + 0.1 * (empty_air_bubbles - 4).max(0) as f32;
        let Some(file) = self.pick_file("ui.hud.bubble_pop") else {
            return;
        };
        self.events.push(format!(
            "ui.hud.bubble_pop @{tick} #{file} v{:.6} p{:.6}",
            volume, pitch
        ));
    }

    /// The FIXED-VOICE lane (the fall voice's generalized shape — the
    /// eat arc's completion speaks it): the caller's volume/pitch fold
    /// (the entity random's own verdict, the bytecode's draw order
    /// kept upstream) rides the log with THIS engine's file pick — the
    /// engine's only honest draw is the weighted file roll.
    pub fn play_voice(&mut self, tick: u64, event: &str, volume: f32, pitch: f32) {
        let Some(file) = self.pick_file(event) else {
            return;
        };
        self.events.push(format!(
            "{event} @{tick} #{file} v{:.6} p{:.6}",
            volume, pitch
        ));
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
    fn the_orb_pickup_voice_speaks_the_take_law() {
        // the EXPERIENCE_ORB_PICKUP fold (javap handleTakeItemEntity's
        // orb branch): volume 0.1, the pitch in 0.9+(r-r)*0.35's own
        // range [0.55, 1.25], the log format the backend mirrors
        let mut a = BlockSounds::new(31);
        a.play_orb_pickup(9);
        assert_eq!(a.events.len(), 1);
        let e = &a.events[0];
        assert!(
            e.starts_with("entity.experience_orb.pickup @9 #"),
            "the entry: {e}"
        );
        assert!(e.contains(" v0.100000 p"), "the volume: {e}");
        let pitch: f32 = e.split(" p").nth(1).unwrap().parse().unwrap();
        assert!((0.55..=1.25).contains(&pitch), "the pitch range: {pitch}");
        // determinism: the same seed speaks the same voice
        let mut b = BlockSounds::new(31);
        b.play_orb_pickup(9);
        assert_eq!(a.events, b.events);
    }

    /// THE FALL VOICE's deadpan: the FIXED 1.0/1.0 fold (javap
    /// causeFallDamage's own literals), the small/big boundary rides
    /// the damage 5 (getFallDamageSound: `i > 4 ? big : small`).
    #[test]
    fn the_fall_voice_speaks_the_fixed_lane() {
        let mut a = BlockSounds::new(3);
        a.play_fall_voice(11, 3);
        assert!(
            a.events[0].starts_with("entity.player.small_fall @11 #"),
            "the entry: {}",
            a.events[0]
        );
        assert!(
            a.events[0].contains(" v1.000000 p1.000000"),
            "the fixed fold: {}",
            a.events[0]
        );
        // the boundary: four stays small, five goes big
        let mut b = BlockSounds::new(3);
        b.play_fall_voice(0, 4);
        assert!(b.events[0].starts_with("entity.player.small_fall"));
        let mut c = BlockSounds::new(3);
        c.play_fall_voice(0, 5);
        assert!(c.events[0].starts_with("entity.player.big_fall"));
    }

    /// THE HURT AND DEATH VOICES' pitch law: `(r - r) * 0.2 + 1.0`
    /// (javap handleDamageEvent + the hurtServer tail's own fold) —
    /// in [0.8, 1.2], the same LCG the pickup speaks, the log format
    /// the backend mirrors.
    #[test]
    fn the_hurt_and_death_voices_speak_the_pitch_law() {
        let mut a = BlockSounds::new(13);
        a.play_hurt_voice(5);
        assert_eq!(a.events.len(), 1);
        assert!(
            a.events[0].starts_with("entity.player.hurt @5 #"),
            "the entry: {}",
            a.events[0]
        );
        assert!(a.events[0].contains(" v1.000000 p"), "the volume: {}", a.events[0]);
        let pitch: f32 = a.events[0].split(" p").nth(1).unwrap().parse().unwrap();
        assert!((0.8..=1.2).contains(&pitch), "the pitch range: {pitch}");
        a.play_death_voice(6);
        assert!(
            a.events[1].starts_with("entity.player.death @6 #"),
            "the death's own event: {}",
            a.events[1]
        );
        // determinism: the same seed speaks the same two voices
        let mut b = BlockSounds::new(13);
        b.play_hurt_voice(5);
        b.play_death_voice(6);
        assert_eq!(a.events, b.events);
    }

    /// THE SWIM VOICE's law: `entity.player.swim` at the caller's
    /// speed-folded volume, the pitch (r - r) * 0.4 + 1.0 in
    /// [0.6, 1.4], the same LCG and draw order the hurt voice speaks,
    /// the log format the backend mirrors.
    #[test]
    fn the_swim_voice_speaks_the_play_swim_law() {
        let mut a = BlockSounds::new(21);
        a.play_swim(9, 0.156_52);
        assert_eq!(a.events.len(), 1);
        assert!(
            a.events[0].starts_with("entity.player.swim @9 #"),
            "the entry: {}",
            a.events[0]
        );
        assert!(a.events[0].contains(" v0.156520 p"), "the volume: {}", a.events[0]);
        let pitch: f32 = a.events[0].split(" p").nth(1).unwrap().parse().unwrap();
        assert!((0.6..=1.4).contains(&pitch), "the pitch range: {pitch}");
        // determinism: the same seed speaks the same voice
        let mut b = BlockSounds::new(21);
        b.play_swim(9, 0.156_52);
        assert_eq!(a.events, b.events);
    }

    /// THE MUFFLED FOLD's law (javap playMuffledStepSound): the same
    /// getStepSound() event the plain step speaks, at the *0.05/*0.8
    /// fold — the wading block voice the water dampens.
    #[test]
    fn the_muffled_fold_speaks_the_water_quiet() {
        let (v, p) = Lane::StepMuffled.fold(1.0, 1.0);
        assert_eq!((v, p), (0.05, 0.8));
        // a non-trivial base rides the algebra (the halving rides the
        // f32's own exact power-of-two scale — 0.5 * 0.05)
        let (v, p) = Lane::StepMuffled.fold(0.5, 1.25);
        assert_eq!((v, p), (0.025, 1.0));
        // the muffle speaks the SAME event the plain step does
        let idx = bs::sound_index("stone").expect("stone in the table");
        assert_eq!(Lane::StepMuffled.event_of(idx), Lane::Step.event_of(idx));
    }

    /// THE ENTRY SPLASH's variant law: the 0.25 gate picks the plain
    /// splash under it, the high-speed one at and past it (javap
    /// doWaterSplashEffect: fcmpg 0.25F, ifge — the >= branch names
    /// the high-speed event).
    #[test]
    fn the_splash_variant_rides_the_speed_gate() {
        // the slow wade-in: the plain PLAYER_SPLASH
        let mut a = BlockSounds::new(5);
        a.play_splash(3, 0.02, false);
        assert!(
            a.events[0].starts_with("entity.player.splash @3 #"),
            "the plain event: {}",
            a.events[0]
        );
        // the tall fall's entry: the HIGH_SPEED variant
        let mut b = BlockSounds::new(5);
        b.play_splash(3, 0.32, true);
        assert!(
            b.events[0].starts_with("entity.player.splash.high_speed @3 #"),
            "the high-speed event: {}",
            b.events[0]
        );
        assert!(b.events[0].contains(" v0.320000 p"), "the volume: {}", b.events[0]);
        let pitch: f32 = b.events[0].split(" p").nth(1).unwrap().parse().unwrap();
        assert!((0.6..=1.4).contains(&pitch), "the pitch range: {pitch}");
        // determinism: the same seed speaks the same splash
        let mut c = BlockSounds::new(5);
        c.play_splash(3, 0.32, true);
        assert_eq!(b.events, c.events);
    }

    /// THE BURN VOICE's law: the fixed 0.4 volume, the pitch
    /// 2.0 + r * 0.4 in [2.0, 2.4] (the SINGLE random the lavaHurt
    /// tail speaks), the log format the backend mirrors.
    #[test]
    fn the_burn_voice_speaks_the_single_random_law() {
        let mut a = BlockSounds::new(17);
        a.play_burn(8);
        assert_eq!(a.events.len(), 1);
        assert!(
            a.events[0].starts_with("entity.generic.burn @8 #"),
            "the entry: {}",
            a.events[0]
        );
        assert!(a.events[0].contains(" v0.400000 p"), "the volume: {}", a.events[0]);
        let pitch: f32 = a.events[0].split(" p").nth(1).unwrap().parse().unwrap();
        assert!((2.0..=2.4).contains(&pitch), "the pitch range: {pitch}");
        // determinism: the same seed speaks the same burn
        let mut b = BlockSounds::new(17);
        b.play_burn(8);
        assert_eq!(a.events, b.events);
    }

    /// THE EXTINGUISH FIZZ's law: the volume 0.7, the pitch
    /// 1.6 + (r - r) * 0.4 in [1.2, 2.0] (the mob lane's own constants
    /// the fold re-speaks), the same LCG and draw order.
    #[test]
    fn the_extinguish_fizz_speaks_the_douse_law() {
        let mut a = BlockSounds::new(23);
        a.play_extinguish(4);
        assert_eq!(a.events.len(), 1);
        assert!(
            a.events[0].starts_with("entity.generic.extinguish_fire @4 #"),
            "the entry: {}",
            a.events[0]
        );
        assert!(a.events[0].contains(" v0.700000 p"), "the volume: {}", a.events[0]);
        let pitch: f32 = a.events[0].split(" p").nth(1).unwrap().parse().unwrap();
        assert!((1.2..=2.0).contains(&pitch), "the pitch range: {pitch}");
        // determinism: the same seed speaks the same fizz
        let mut b = BlockSounds::new(23);
        b.play_extinguish(4);
        assert_eq!(a.events, b.events);
    }

    /// THE BURNING HURT VOICE's law: the on-fire event (the
    /// DamageEffects.BURNING sound both fire sources speak), the plain
    /// hurt voice's own pitch law (r - r) * 0.2 + 1.0 in [0.8, 1.2].
    #[test]
    fn the_burning_hurt_voice_speaks_the_on_fire_event() {
        let mut a = BlockSounds::new(29);
        a.play_hurt_on_fire_voice(6);
        assert_eq!(a.events.len(), 1);
        assert!(
            a.events[0].starts_with("entity.player.hurt_on_fire @6 #"),
            "the entry: {}",
            a.events[0]
        );
        assert!(a.events[0].contains(" v1.000000 p"), "the volume: {}", a.events[0]);
        let pitch: f32 = a.events[0].split(" p").nth(1).unwrap().parse().unwrap();
        assert!((0.8..=1.2).contains(&pitch), "the pitch range: {pitch}");
        // determinism: the same seed speaks the same voice
        let mut b = BlockSounds::new(29);
        b.play_hurt_on_fire_voice(6);
        assert_eq!(a.events, b.events);
    }

    /// THE DROWNING VOICE's law: the hurt_drown event (the
    /// DamageEffects.DROWNING sound the -20 fold's dose speaks), the
    /// plain hurt voice's own pitch law (r - r) * 0.2 + 1.0 in
    /// [0.8, 1.2], the same LCG and draw order.
    #[test]
    fn the_drowning_hurt_voice_speaks_the_drown_event() {
        let mut a = BlockSounds::new(31);
        a.play_hurt_drown_voice(6);
        assert_eq!(a.events.len(), 1);
        assert!(
            a.events[0].starts_with("entity.player.hurt_drown @6 #"),
            "the entry: {}",
            a.events[0]
        );
        assert!(a.events[0].contains(" v1.000000 p"), "the volume: {}", a.events[0]);
        let pitch: f32 = a.events[0].split(" p").nth(1).unwrap().parse().unwrap();
        assert!((0.8..=1.2).contains(&pitch), "the pitch range: {pitch}");
        // determinism: the same seed speaks the same voice
        let mut b = BlockSounds::new(31);
        b.play_hurt_drown_voice(6);
        assert_eq!(a.events, b.events);
    }

    /// THE BUBBLE POP's law (playAirBubblePoppedSound's own fold):
    /// volume 0.5 + 0.1*max(0, empty - 2), pitch 1.0 + 0.1*max(0,
    /// empty - 4) — no randoms, the fixed fold playSound's shape —
    /// the ui.hud.bubble_pop event the BUBBLE_POP constant names.
    #[test]
    fn the_bubble_pop_speaks_the_fixed_fold() {
        let mut a = BlockSounds::new(37);
        a.play_bubble_pop(9, 4);
        assert_eq!(a.events.len(), 1);
        assert!(
            a.events[0].starts_with("ui.hud.bubble_pop @9 #"),
            "the entry: {}",
            a.events[0]
        );
        // empty=4: volume 0.5 + 0.1*max(0,2) = 0.7, pitch 1.0 + 0.1*0 = 1.0
        assert!(a.events[0].contains(" v0.700000 p1.000000"), "the fold: {}", a.events[0]);
        // the increments' own station: empty=8 folds 1.1 / 1.4
        a.events.clear();
        a.play_bubble_pop(9, 8);
        assert!(
            a.events[0].contains(" v1.100000 p1.400000"),
            "the empty=8 fold: {}",
            a.events[0]
        );
        // the floor's own silence: empty=2 speaks the bare base
        a.events.clear();
        a.play_bubble_pop(9, 2);
        assert!(a.events[0].contains(" v0.500000 p1.000000"), "the base: {}", a.events[0]);
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
        // a weighted pick never leaves the official range (THE MIGRATION'S
        // OWN LAW: the assert computes the expected from the ACTUAL loaded
        // state — the cache-absent boot the empty table the silent lane
        // keeps, the cache-present boot the range walk speaks)
        let mut d = BlockSounds::new(9);
        for _ in 0..32 {
            d.play(0, "glass", Lane::Break);
        }
        let Some(ev) = snd::event("block.glass.break") else {
            assert!(
                d.events.is_empty(),
                "the empty table the silent lane keeps"
            );
            return;
        };
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

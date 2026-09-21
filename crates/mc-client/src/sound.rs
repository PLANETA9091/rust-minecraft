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
#[cfg_attr(not(test), expect(dead_code))]
pub fn next_step(move_dist: f32) -> f32 {
    (move_dist as i32) as f32 + 1.0
}

/// THE SOUND ENGINE'S OWN DISTANCE LAW (javap SoundEngine.play +
/// Channel.linearAttenuation the 26.2 jar speaks — the positioned
/// voices' own fold):
///
/// * the play walk arms the channel with `Math.max(getVolume(), 1.0F) *
///   (float)Sound.getAttenuationDistance()` (the play() bytecode's
///   fmax/fmul into the linearAttenuation call's float arg) — THE
///   ARMED MAX DISTANCE: the louder the voice the farther it carries,
///   the quiet never shrinks the reach;
/// * `Channel.linearAttenuation` sets the source's distance model
///   AL_LINEAR_DISTANCE (alSourcei 53248 -> 53251), the reference
///   distance 0.0 (alSourcef 4128), the rolloff factor 1.0 (alSourcef
///   4129) and the armed max (alSourcef 4131);
/// * the OpenAL 1.1 spec's linear law folds the channel gain
///   `1 - rolloff * (dist - ref) / (max - ref)`, which at the 0.0
///   reference and the 1.0 rolloff reads `1 - dist / max` — the full
///   voice at the ear, the zero crossing at the armed max, the silence
///   beyond it the AL_MIN_GAIN 0.0 floor keeps;
/// * the output chain's AL_MAX_GAIN 1.0 ceiling caps the loud packets'
///   fold (the 4.0F voice the ghast row speaks never plays louder than
///   the ear can take — it merely carries farther).
///
/// The default `attenuation_distance` the sounds.json rows keep is 16
/// (`Sound`'s own ctor default — the mob voices' rows name no
/// override), so a volume-1.0 voice goes silent exactly 16 blocks out.
pub const DEFAULT_ATTENUATION_DISTANCE: f64 = 16.0;

/// THE ARMED MAX (the play walk's own `Math.max(getVolume(), 1.0F) *
/// getAttenuationDistance()`): the louder the voice the farther the
/// zero crossing lands, the quiet never shrinks the reach.
pub fn voice_max_distance(volume: f32) -> f64 {
    f64::from(volume.max(1.0)) * DEFAULT_ATTENUATION_DISTANCE
}

/// THE LINEAR FOLD (the spec's `1 - rolloff * (d - ref) / (max - ref)`
/// at the 0.0 reference and the 1.0 rolloff the linearAttenuation
/// arms): the floor the AL_MIN_GAIN 0.0 keeps, the ceiling the
/// AL_MAX_GAIN 1.0 keeps.
pub fn attenuate(volume: f32, dist: f64, max_distance: f64) -> f32 {
    let linear = (1.0 - dist / max_distance).max(0.0);
    (f64::from(volume) * linear).min(1.0) as f32
}

/// THE LATERAL PAN LAW (OpenAL's own stereo fold the positioned voices
/// speak — the AL_POSITION source + the AL_ORIENTATION listener pair
/// the real engine's `Listener` tick arms; the `Channel.linearAttenuation`
/// source the distance law above speaks rides the same pipeline):
///
/// * the ear→source direction the listener's RIGHT axis answers: the
///   vanilla yaw's own degrees the entity rotations speak (0 = +Z, the
///   forward the `(-sin, cos)` fold wears — the same pair the boat
///   paddle's `paddle_side_offset` and the eat aim's `-yaw * rad` ride),
///   the right axis the yaw + 90° step wears — `(-cos, -sin)` on the
///   (x, z) plane (facing south the west answers the right hand, the
///   compass the vanilla map's own layout keeps);
/// * the per-ear gains the LINEAR family the distance law keeps: the
///   pure-side source (+1.0) the far speaker silences, the front and
///   the behind (0.0) both ears keep — the vertical lift never pans
///   (the horizontal axis the right vector wears, the straight-overhead
///   source the center pair speaks);
/// * the degenerate zero-length direction (the source INSIDE the ear)
///   the honest center 0.0 keeps — the same arm the reach's no-NaN
///   note wears;
/// * the f32 rounding the [-1.0, 1.0] clamp never lets a hair past the
///   pure sides.
///
/// The pan rides the log's ` s{:+.6}` field the backend's stereo fold
/// answers (the mono file the two-speaker pair becomes, the gains the
/// same linear law's per-ear slice keeps).
pub fn lateral_pan(dx: f64, dy: f64, dz: f64, yaw_deg: f32) -> f32 {
    let len2 = dx * dx + dy * dy + dz * dz;
    if len2 == 0.0 {
        return 0.0;
    }
    let inv = 1.0 / len2.sqrt();
    let yaw = f64::from(yaw_deg).to_radians();
    // the right axis (x, z): the yaw + 90° fold the forward's
    // `(-sin, cos)` pair wears — facing south the west answers the
    // right hand
    let (rx, rz) = (-yaw.cos(), -yaw.sin());
    let pan = (dx * inv * rx + dz * inv * rz) as f32;
    pan.clamp(-1.0, 1.0)
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
    /// draws speak it — the same 24-bit shape the file pick rolls). The
    /// paddle voice's caller reads it too — the boat's own RandomSource
    /// the `0.8F + 0.4F * random.nextFloat()` pitch fold draws stands
    /// in, the entity random the client's copy never carried.
    pub fn next_rand_f32(&mut self) -> f32 {
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

    /// THE BUBBLE COLUMN'S INSIDE VOICE (javap
    /// `BubbleColumnAmbientSoundHandler.tick` — the
    /// LocalPlayer.ambientSoundHandlers entry the tick's own tail
    /// walks): the edge-triggered one-shot the column ENTRY speaks —
    /// `player.playSound(BUBBLE_COLUMN_WHIRLPOOL_INSIDE, 1.0F, 1.0F)`
    /// the drag=true column answers, `BUBBLE_COLUMN_UPWARDS_INSIDE,
    /// 1.0F, 1.0F` the drag=false one — the volume/pitch BOTH the
    /// fixed 1.0 the bytecode's fconst_1 pair speaks, NO randoms (the
    /// fixed fold playSound's own shape). The play rides the player's
    /// own position (the ear itself — the dist-0 voice the gain's
    /// identity keeps, no attenuation the self lane folds). The
    /// caller's edge trigger (the !wasInBubbleColumn && !firstTick
    /// gate the handler's walk keeps) rides upstream — one voice per
    /// entry, the hold never repeats.
    pub fn play_bubble_inside(&mut self, tick: u64, whirlpool: bool) {
        let event = if whirlpool {
            "block.bubble_column.whirlpool_inside"
        } else {
            "block.bubble_column.upwards_inside"
        };
        let Some(file) = self.pick_file(event) else {
            return;
        };
        self.events.push(format!(
            "{event} @{tick} #{file} v1.000000 p1.000000"
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

    /// THE POSITIONED VOICE (the SoundEngine's own distance law the
    /// mob voices ride first): the source's volume the gain the fold
    /// speaks, the armed max the play walk's own `Math.max(v, 1.0F) *
    /// getAttenuationDistance()` names — the log line records the
    /// FOLDED gain (the audio backend's own mirror plays it).
    pub fn play_voice_at(&mut self, tick: u64, event: &str, volume: f32, pitch: f32, dist: f64) {
        let gain = attenuate(volume, dist, voice_max_distance(volume));
        self.play_voice(tick, event, gain, pitch);
    }

    /// THE POSITIONED VOICE'S STEREO FOLD (the lateral pan law the
    /// `lateral_pan` doc speaks — the AL_POSITION + AL_ORIENTATION pair
    /// the real engine's listener answers): the distance law's gain the
    /// reach folds first, the pan the log's ` s{:+.6}` tail carries —
    /// the backend's stereo fold the mono file's two-speaker split
    /// plays. The CENTER voice (pan 0.0) the identity the plain
    /// positioned lane keeps: the same 4-field shape the unmigrated
    /// lanes wear, no field the fold's no-op never needs.
    pub fn play_voice_at_pan(
        &mut self,
        tick: u64,
        event: &str,
        volume: f32,
        pitch: f32,
        dist: f64,
        pan: f32,
    ) {
        if pan == 0.0 {
            return self.play_voice_at(tick, event, volume, pitch, dist);
        }
        let gain = attenuate(volume, dist, voice_max_distance(volume));
        let Some(file) = self.pick_file(event) else {
            return;
        };
        self.events.push(format!(
            "{event} @{tick} #{file} v{gain:.6} p{pitch:.6} s{pan:+.6}"
        ));
    }

    /// THE POSITIONED BLOCK LANE (the 2001 event's own walk the mob's
    /// bite rides — `LevelEventHandler.levelEvent` case 2001 the dig
    /// lane's completion speaks, the sheep's EatBlockGoal the same
    /// event the below arm's setBlock walk plays): the block's own
    /// sound row the lane folds first ((v+1)/2, p*0.8 the break law
    /// keeps), then the distance law's gain the positioned voices
    /// speak — the ear the SoundEngine's camera keeps, the source the
    /// block's own cell. The file pick rides the engine's own LCG the
    /// one-shot lanes share.
    pub fn play_lane_at(&mut self, tick: u64, block: &str, lane: Lane, dist: f64) {
        let Some(idx) = bs::sound_index(block) else {
            return;
        };
        let event = lane.event_of(idx);
        let (volume, pitch) = lane.fold(bs::SOUND_VOLUME[idx], bs::SOUND_PITCH[idx]);
        let gain = attenuate(volume, dist, voice_max_distance(volume));
        self.play_voice(tick, event, gain, pitch);
    }

    /// THE POSITIONED BLOCK LANE'S STEREO FOLD (the pan law the
    /// `play_voice_at_pan` doc speaks — the same tail the block lanes
    /// wear once the callers fold): the row's own `(v+1)/2, p*0.8`
    /// fold the plain lane keeps first, the gain the same reach fold
    /// speaks, the pan the log's ` s{:+.6}` tail carries. The CENTER
    /// voice (pan 0.0) the identity the plain positioned lane keeps:
    /// the same 4-field shape the unmigrated callers wear, no field
    /// the no-op never needs.
    pub fn play_lane_at_pan(&mut self, tick: u64, block: &str, lane: Lane, dist: f64, pan: f32) {
        if pan == 0.0 {
            return self.play_lane_at(tick, block, lane, dist);
        }
        let Some(idx) = bs::sound_index(block) else {
            return;
        };
        let event = lane.event_of(idx);
        let (volume, pitch) = lane.fold(bs::SOUND_VOLUME[idx], bs::SOUND_PITCH[idx]);
        let gain = attenuate(volume, dist, voice_max_distance(volume));
        let Some(file) = self.pick_file(event) else {
            return;
        };
        self.events.push(format!(
            "{event} @{tick} #{file} v{gain:.6} p{pitch:.6} s{pan:+.6}"
        ));
    }
}

/// THE CAVE MOOD'S OWN BANK (javap `AmbientMoodSettings.LEGACY_CAVE_SETTINGS`
/// <clinit> offsets 11-29, the 26.2 jar): the record the world's
/// AMBIENT_SOUNDS environment attribute answers — the sound event the
/// `SoundEvents.AMBIENT_CAVE` constant holds, the 6000-tick darkness
/// dividend (`sipush 6000`), the ±8 sample extent (`bipush 8`), the 2.0
/// play offset (`ldc2_w 2.0`). No biome in the 26.2 jar overrides the
/// attribute (the 66 worldgen biome jsons' `effects` carry colors only —
/// zero `ambient_sound` rows), so the LEGACY settings ARE the live machine.
pub const MOOD_EVENT: &str = "ambient.cave";
pub const MOOD_TICK_DELAY: i64 = 6000;
pub const MOOD_BLOCK_SEARCH_EXTENT: i32 = 8;
pub const MOOD_SOUND_POSITION_OFFSET: f64 = 2.0;

/// THE MOOD MACHINE (javap `BiomeAmbientSoundsHandler.lambda$tick$1`, the
/// 26.2 jar — the tick() lambda the `AmbientSounds.mood()` ifPresent arm
/// feeds, one walk per game tick):
///
/// * the SAMPLE: three `random.nextInt(extent * 2 + 1)` draws in the x,
///   y, z order the bytecode speaks (offsets 1-86), each `player.<axis> +
///   draw - extent`, `BlockPos.containing` the floor — ONE random cell
///   around the eye per tick;
/// * the ACCUMULATOR (`moodiness`): a sky-lit sample spends
///   `sky / 15.0F * 0.001F` (offsets 105-120 — the daylight's own calm),
///   a sky-dark sample spends `(blockLight - 1) / tickDelay` (offsets
///   126-150) — the NEGATIVE spend the complete darkness answers (block
///   0: `(0 - 1) / 6000`, the subtract of a negative the += 1/6000 the
///   mood rides; block 1 spends nothing; block 2+ spends back), so the
///   pitch-dark tick's walk banks 1/6000 toward the mood;
/// * the FIRE (`moodiness >= 1.0F`, offsets 153-159): the sampled cell's
///   center rides the eye→cell ray `soundPositionOffset` (2.0) PAST the
///   spot (`SimpleSoundInstance.forAmbientMood(event, random, player +
///   dir * (dist + 2.0))`, offsets 164-329), the one-shot the
///   SoundSource.AMBIENT source keeps at volume 1.0 pitch 1.0 with the
///   LINEAR attenuation (forAmbientMood's own fconst_1 pair — no pitch
///   shake, the AbstractSoundInstance ctor's 1.0/1.0 defaults stand), and
///   the moodiness resets to 0.0 (offsets 344-346);
/// * the CALM (`moodiness < 1.0F`): `Math.max(moodiness, 0.0F)` (offsets
///   352-361) — the lit world's spend never drives the mood negative.
///
/// THE CLIENT'S OWN WALK: the machine is pure — the light read the
/// caller's world answers (the `world.light` grid the networked columns
/// keep, the column walk the procedural scene's honest fallback speaks —
/// the same pair the F3 debug lane's eye light reads), the eye anchor the
/// body's own `x, y + eyeHeight(), z` walk keeps, the listener's own yaw
/// the camera's read answers (the stereo pan the ray's lateral split
/// speaks), and the one-shot the positioned lane logs (the reach — the
/// play position's own distance from the ear — the linear gain fold the
/// distance law speaks, the lateral pan the ray's right-axis split the
/// `lateral_pan` law answers).
pub struct CaveMood {
    rng: u64,
    moodiness: f32,
}

impl CaveMood {
    pub fn new(seed: u64) -> Self {
        Self {
            // the mood's own stream tag (the machines' distinct scrambles
            // keep the determinism pins' streams apart — the mob_rng's
            // 0x5EED_10CA, the level ambient's 0x5EED_7E11, the engine's
            // 0x5EED_C0FF_EE12_3456 all speak their own lanes)
            rng: seed ^ 0x5EED_C4FE_C0DE_0001,
            moodiness: 0.0,
        }
    }

    fn next_rand(&mut self) -> u64 {
        self.rng = self
            .rng
            .wrapping_mul(0x5851_F42D_4C95_7F2D)
            .wrapping_add(0x1405_7B7E_F767_814F);
        self.rng >> 16
    }

    /// THE MACHINE'S OWN TICK (the lambda's walk, verbatim): one sample,
    /// one spend, the fire or the calm.
    #[allow(clippy::too_many_arguments)]
    pub fn tick(
        &mut self,
        tick: u64,
        light: &dyn Fn(i32, i32, i32) -> (u8, u8),
        px: f64,
        eye_y: f64,
        pz: f64,
        yaw: f32,
        sounds: &mut BlockSounds,
    ) {
        // the sample extent's own range: `blockSearchExtent * 2 + 1` —
        // the draws' bound the lambda's first arm speaks (offsets 0-8)
        let range = (MOOD_BLOCK_SEARCH_EXTENT * 2 + 1) as u64;
        let extent = f64::from(MOOD_BLOCK_SEARCH_EXTENT);
        // the three draws, x the y the z order (offsets 9-86); the
        // RandomSource.nextInt(bound) the engine's LCG modulo answers —
        // the same shape every deterministic draw here wears
        let sx = px + (self.next_rand() % range) as f64 - extent;
        let sy = eye_y + (self.next_rand() % range) as f64 - extent;
        let sz = pz + (self.next_rand() % range) as f64 - extent;
        let bx = sx.floor() as i32;
        let by = sy.floor() as i32;
        let bz = sz.floor() as i32;
        // the light pair the caller's world answers (the SKY then the
        // BLOCK layer the getBrightness pair speaks)
        let (sky, block) = light(bx, by, bz);
        if sky > 0 {
            // the daylight's own calm (offsets 105-120): the sky level's
            // fifteenth * 0.001F
            self.moodiness -= (f32::from(sky) / 15.0) * 0.001;
        } else {
            // the darkness' own bank (offsets 126-150): the block light's
            // minus-one over the tick delay — block 0 banks 1/6000, block
            // 1 spends nothing, block 2+ spends back
            self.moodiness -= (f32::from(block) - 1.0) / MOOD_TICK_DELAY as f32;
        }
        if self.moodiness >= 1.0 {
            // the fire (offsets 162-346): the cell center, the eye→cell
            // ray walked 2.0 past the spot, the positioned one-shot —
            // volume 1.0 pitch 1.0 (forAmbientMood's own fold), the reach
            // the linear gain's own distance speaks
            let cx = f64::from(bx) + 0.5;
            let cy = f64::from(by) + 0.5;
            let cz = f64::from(bz) + 0.5;
            let dx = cx - px;
            let dy = cy - eye_y;
            let dz = cz - pz;
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            // the play position rides the ray `dir * (dist + 2.0)` past
            // the spot — the vanilla walk's own `player + dx / dist *
            // (dist + 2.0)` triple (offsets 198-328). The play position
            // lies ON the eye→spot ray, so the ray's own direction the
            // stereo pan's split answers too (the same vector the
            // vanilla's dir/dist divide walks — the listener's yaw the
            // caller's camera read carries, the lateral split the
            // `lateral_pan` law's right-axis fold speaks, the degenerate
            // zero-length ray the honest center 0.0 keeps — no NaN the
            // eye-cell sample could speak, reach 2.0 the floor answers).
            let reach = dist + MOOD_SOUND_POSITION_OFFSET;
            let pan = lateral_pan(dx, dy, dz, yaw);
            sounds.play_voice_at_pan(tick, MOOD_EVENT, 1.0, 1.0, reach, pan);
            self.moodiness = 0.0;
        } else {
            // the calm (offsets 352-361): the floor the zero keeps
            self.moodiness = self.moodiness.max(0.0);
        }
    }

    /// The accumulator's own read (the tests' honest window — the
    /// `getMoodiness()` accessor the handler's own class speaks).
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn moodiness(&self) -> f32 {
        self.moodiness
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
    fn the_distance_law_speaks_the_channel_folds() {
        // THE ARMED MAX (the play walk's own `Math.max(v, 1.0F) *
        // getAttenuationDistance()`): the default 16 the volume-1.0
        // voices arm, the louder the farther (the 4.0F packet carries
        // to 64), the quiet never shrinks the reach
        assert_eq!(voice_max_distance(1.0), 16.0);
        assert_eq!(voice_max_distance(4.0), 64.0);
        assert_eq!(voice_max_distance(0.5), 16.0);
        // THE LINEAR FOLD at the 0.0 reference and the 1.0 rolloff:
        // the full voice at the ear, the half at half the max, the
        // zero crossing at the max, the silence beyond it the
        // AL_MIN_GAIN floor keeps
        assert_eq!(attenuate(1.0, 0.0, 16.0), 1.0);
        assert_eq!(attenuate(1.0, 4.0, 16.0), 0.75);
        assert_eq!(attenuate(1.0, 8.0, 16.0), 0.5);
        assert_eq!(attenuate(1.0, 16.0, 16.0), 0.0);
        assert_eq!(attenuate(1.0, 24.0, 16.0), 0.0);
        // the quiet voice rides the same law at its own gain
        assert_eq!(attenuate(0.8, 4.0, 16.0), 0.6);
        // the loud packet's fold: the cap the AL_MAX_GAIN 1.0 keeps
        // at the ear, the stretch the armed 64 speaks at 56 out
        assert_eq!(attenuate(4.0, 0.0, 64.0), 1.0);
        assert_eq!(attenuate(4.0, 56.0, 64.0), 0.5);
        assert_eq!(attenuate(4.0, 64.0, 64.0), 0.0);
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

    /// THE BUBBLE COLUMN'S INSIDE VOICE's law (the handler's own fold):
    /// the volume/pitch BOTH the fixed 1.0 the bytecode's fconst_1 pair
    /// speaks, no randoms — the drag bit picks the event, the
    /// whirlpool_inside the drag=true column answers, the
    /// upwards_inside the drag=false one.
    #[test]
    fn the_bubble_inside_voice_speaks_the_fixed_fold() {
        let mut a = BlockSounds::new(37);
        a.play_bubble_inside(9, true);
        assert_eq!(a.events.len(), 1);
        assert!(
            a.events[0].starts_with("block.bubble_column.whirlpool_inside @9 #"),
            "the whirlpool entry: {}",
            a.events[0]
        );
        assert!(
            a.events[0].contains(" v1.000000 p1.000000"),
            "the fixed fold: {}",
            a.events[0]
        );
        // the upward's own event word (a fresh engine — the first draw
        // the file pick consumes is the determinism's own state)
        let mut b = BlockSounds::new(37);
        b.play_bubble_inside(9, false);
        assert!(
            b.events[0].starts_with("block.bubble_column.upwards_inside @9 #"),
            "the upward entry: {}",
            b.events[0]
        );
        assert!(
            b.events[0].contains(" v1.000000 p1.000000"),
            "the fixed fold: {}",
            b.events[0]
        );
        // determinism: the same seed speaks the same voice
        let mut c = BlockSounds::new(37);
        c.play_bubble_inside(9, true);
        assert_eq!(a.events, c.events);
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
    fn the_positioned_block_lane_folds_then_attenuates() {
        // THE GRAZE'S OWN VOICE (the 2001 event the mob's bite rides —
        // `LevelEventHandler` case 2001 the dig lane's completion
        // speaks): the block's own row the lane folds first ((v+1)/2,
        // p*0.8 the break law keeps), the distance law's gain the
        // positioned voices speak — the log the folded gain records
        let mut s = BlockSounds::new(7);
        s.play_lane_at(36, "grass_block", Lane::Break, 0.0);
        assert_eq!(s.events.len(), 1, "the ear-at-source voice");
        let e = &s.events[0];
        assert!(
            e.starts_with("block.grass.break @36 #"),
            "the grass block's own break voice: {e}"
        );
        assert!(e.contains(" v1.000000 p0.800000"), "the fold: {e}");
        // THE MID WALK: the armed max the volume's own 16 speaks, the
        // linear law 1 - dist/max folds (the 4-out ear 0.75 answers)
        let mut s = BlockSounds::new(7);
        s.play_lane_at(36, "grass_block", Lane::Break, 4.0);
        assert!(
            s.events[0].contains(" v0.750000 p0.800000"),
            "the 4-out fold: {}",
            s.events[0]
        );
        // THE SILENCE BEYOND (the zero crossing the armed max names —
        // the line the folded zero records, the backend the silence
        // plays)
        let mut s = BlockSounds::new(7);
        s.play_lane_at(36, "grass_block", Lane::Break, 20.0);
        assert!(
            s.events[0].contains(" v0.000000 p0.800000"),
            "the beyond-max fold: {}",
            s.events[0]
        );
        // THE TABLE'S OWN MISS: the unknown block the silent lane keeps
        let mut s = BlockSounds::new(7);
        s.play_lane_at(36, "not_a_block", Lane::Break, 0.0);
        assert!(s.events.is_empty(), "the miss never speaks");
        // determinism: the same seed speaks the same positioned voice
        let mut a = BlockSounds::new(31);
        let mut b = BlockSounds::new(31);
        a.play_lane_at(9, "grass_block", Lane::Break, 5.0);
        b.play_lane_at(9, "grass_block", Lane::Break, 5.0);
        assert_eq!(a.events, b.events, "seeded runs stay identical");
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

    /// THE LATERAL PAN LAW'S OWN WALK (the right-axis fold the
    /// `lateral_pan` doc speaks): the cardinal faces the compass
    /// answers, the diagonal the half-power split speaks, the vertical
    /// lift and the degenerate zero-length ray the center keeps.
    #[test]
    fn the_lateral_pan_splits_the_listener_right_axis() {
        // facing the vanilla south (yaw 0): the west source the right
        // hand answers (+1.0), the east the left (-1.0), the south (in
        // front) and the north (behind) both ears keep — the front/back
        // symmetry the no-HRTF stereo fold speaks
        assert!((lateral_pan(-2.0, 0.0, 0.0, 0.0) - 1.0).abs() < 1e-6);
        assert!((lateral_pan(3.0, 0.0, 0.0, 0.0) + 1.0).abs() < 1e-6);
        assert!(lateral_pan(0.0, 0.0, 5.0, 0.0).abs() < 1e-6);
        assert!(lateral_pan(0.0, 0.0, -5.0, 0.0).abs() < 1e-6);
        // facing the vanilla west (yaw 90): the north source the right
        // hand answers (+1.0), the south the left
        assert!((lateral_pan(0.0, 0.0, -4.0, 90.0) - 1.0).abs() < 1e-6);
        assert!((lateral_pan(0.0, 0.0, 4.0, 90.0) + 1.0).abs() < 1e-6);
        // the 45° diagonal the sqrt(0.5) lateral speaks (the 3D
        // normalize + the horizontal right axis keeps)
        let diag = lateral_pan(-1.0, 0.0, 1.0, 0.0);
        assert!((diag - 0.5f32.sqrt()).abs() < 1e-5, "{diag}");
        // the straight-overhead ray the center keeps (the right axis the
        // horizontal plane wears — the vertical lift never pans)
        assert!(lateral_pan(0.0, 7.0, 0.0, 0.0).abs() < 1e-6);
        // the degenerate zero-length direction (the source INSIDE the
        // ear) the honest center keeps — no NaN the fold ever speaks
        assert!(lateral_pan(0.0, 0.0, 0.0, 123.0).abs() < 1e-6);
        assert!(!lateral_pan(0.0, 0.0, 0.0, 0.0).is_nan());
        // the horizontal length the inv fold cancels: the far west ray
        // the same +1.0 the near one speaks
        assert!((lateral_pan(-100.0, 0.0, 0.0, 0.0) - 1.0).abs() < 1e-6);
        // the vertical offset the 3D normalize dilutes honestly: the
        // above-side ray speaks the SMALLER lateral the geometry keeps
        // (dx -100, dy 400 -> the lateral the len's own share answers)
        let above_side = lateral_pan(-100.0, 400.0, 0.0, 0.0);
        let want = (100.0f64 / (170_000.0f64).sqrt()) as f32;
        assert!((above_side - want).abs() < 1e-5, "{above_side} vs {want}");
    }

    /// THE PAN LANE'S OWN LOG WALK: the center voice the identity the
    /// plain positioned lane keeps (the same 4-field shape, the same
    /// gain fold), the panned voice the ` s` tail carries — the same
    /// reach fold the gain rides, the determinism the seeded pair keeps.
    #[test]
    fn the_pan_lane_logs_the_center_identity_and_the_stereo_tail() {
        // the center voice: the identical line the plain lane speaks
        let mut a = BlockSounds::new(11);
        let mut b = BlockSounds::new(11);
        a.play_voice_at(36, "entity.cow.ambient", 1.0, 1.0, 4.0);
        b.play_voice_at_pan(36, "entity.cow.ambient", 1.0, 1.0, 4.0, 0.0);
        assert_eq!(a.events, b.events, "the center the identity keeps");
        assert_eq!(a.events.len(), 1);
        assert!(!a.events[0].contains(" s"), "no tail the no-op needs");

        // the panned voice: the ` s` tail rides, the gain the same reach
        // fold speaks
        let mut c = BlockSounds::new(11);
        c.play_voice_at_pan(36, "entity.cow.ambient", 1.0, 1.0, 4.0, 0.5);
        let gain = attenuate(1.0, 4.0, voice_max_distance(1.0));
        let line = &c.events[0];
        assert!(
            line.contains(&format!(" v{gain:.6} p1.000000 s+0.500000")),
            "the tail the pan law speaks: {line}"
        );

        // the negative side the signed tail carries, the file pick the
        // same seed's own LCG draw answers (the prefix before the gain
        // field the determinism pins)
        let mut d = BlockSounds::new(11);
        d.play_voice_at_pan(36, "entity.cow.ambient", 1.0, 1.0, 4.0, -0.25);
        assert!(d.events[0].contains(" s-0.250000"), "{}", d.events[0]);
        assert_eq!(
            c.events[0].split(" v").next(),
            d.events[0].split(" v").next(),
            "the same event/tick/file the same seed speaks"
        );
    }

    /// THE BLOCK LANE'S OWN STEREO FOLD (the pan law the positioned
    /// block lanes' migration wears): the row's own fold the plain
    /// lane keeps first, the gain the same reach speaks, the pan the
    /// log's ` s{:+.6}` tail carries; the center voice the identity
    /// the plain positioned lane keeps.
    #[test]
    fn the_block_lane_pan_keeps_the_fold_and_wears_the_tail() {
        // the center voice: the identical line the plain lane speaks
        let mut a = BlockSounds::new(11);
        let mut b = BlockSounds::new(11);
        a.play_lane_at(36, "stone", Lane::Break, 4.0);
        b.play_lane_at_pan(36, "stone", Lane::Break, 4.0, 0.0);
        assert_eq!(a.events, b.events, "the center the identity keeps");
        assert_eq!(a.events.len(), 1);
        assert!(!a.events[0].contains(" s"), "no tail the no-op needs");

        // the panned voice: the break row's own fold the plain lane
        // keeps ((v+1)/2, p*0.8), the gain the same reach folds, the
        // s tail the signed side wears
        let idx = bs::sound_index("stone").expect("the stone row the table keeps");
        let (volume, pitch) = Lane::Break.fold(bs::SOUND_VOLUME[idx], bs::SOUND_PITCH[idx]);
        let mut c = BlockSounds::new(11);
        c.play_lane_at_pan(36, "stone", Lane::Break, 4.0, 0.5);
        let gain = attenuate(volume, 4.0, voice_max_distance(volume));
        let line = &c.events[0];
        assert!(
            line.contains(&format!(" v{gain:.6} p{pitch:.6} s+0.500000")),
            "the break fold + the tail the pan law speaks: {line}"
        );
        assert_eq!(
            a.events[0].split(" v").next(),
            c.events[0].split(" v").next(),
            "the same event/tick/file the same seed speaks"
        );

        // the negative side the signed tail carries
        let mut d = BlockSounds::new(11);
        d.play_lane_at_pan(36, "stone", Lane::Break, 4.0, -0.25);
        assert!(d.events[0].contains(" s-0.250000"), "{}", d.events[0]);
    }


    /// THE MOOD'S OWN LAW WALK (the lambda's arithmetic the tests bank):
    /// the darkness banks 1/6000 a tick, the fire at 6000 dark ticks, the
    /// reset after, the daylight and the torches spend back, the floor
    /// the zero keeps.
    #[test]
    fn the_cave_mood_banks_the_darkness_and_fires_the_legacy_numbers() {
        // THE COMPLETE DARKNESS: every sample (sky 0, block 0) banks
        // +1/6000 — the fire lands on the walk the f32 arithmetic speaks,
        // the spot the recorder keeps (the reach law's own honest pin
        // rides the last sampled cell)
        let seen = std::cell::RefCell::new((0i32, 0i32, 0i32));
        let record = |x: i32, y: i32, z: i32| -> (u8, u8) {
            *seen.borrow_mut() = (x, y, z);
            (0u8, 0u8)
        };
        let mut m = CaveMood::new(7);
        let mut log = BlockSounds::new(1);
        // the fire tick: the strict f32 walk (the JVM's own IEEE binary32
        // semantics — every intermediate a float) lands EXACTLY on the
        // 6000th bank: f32(1/6000) * 6000 rounds back to 1.0, the >= 1.0
        // arm speaks, the fire rides the 6000-tick tickDelay the
        // LEGACY_CAVE_SETTINGS keeps
        let mut fire_tick: Option<u64> = None;
        for i in 0..(MOOD_TICK_DELAY + 16) as u64 {
            m.tick(i, &record, 10.3, 20.7, 30.9, 0.0, &mut log);
            if !log.events.is_empty() {
                fire_tick = Some(i);
                break;
            }
            assert!(m.moodiness() < 1.0);
            assert!(m.moodiness() >= 0.0, "the floor the zero keeps");
        }
        assert_eq!(fire_tick, Some(6000), "the strict f32 walk's own fire tick");
        // the v/p pair the forAmbientMood's own fold speaks — pitch 1.0,
        // the volume the linear gain's own reach fold (the fired spot the
        // recorder kept: dist(center, eye) + 2.0, the 16 the armed max
        // keeps, the floor the AL_MIN_GAIN zero speaks)
        let (sx, sy, sz) = *seen.borrow();
        let dx = f64::from(sx) + 0.5 - 10.3;
        let dy = f64::from(sy) + 0.5 - 20.7;
        let dz = f64::from(sz) + 0.5 - 30.9;
        let reach = (dx * dx + dy * dy + dz * dz).sqrt() + MOOD_SOUND_POSITION_OFFSET;
        let want_gain = attenuate(1.0, reach, voice_max_distance(1.0));
        let line = &log.events[0];
        let v: f32 = line
            .split(" v")
            .nth(1)
            .and_then(|s| s.split(' ').next())
            .and_then(|s| s.parse().ok())
            .expect("the log carries the volume");
        let p: f32 = line
            .split(" p")
            .nth(1)
            .and_then(|s| s.split(' ').next())
            .and_then(|s| s.parse().ok())
            .expect("the log carries the pitch");
        assert!((v - want_gain).abs() < 1e-6, "gain {v} vs reach {reach}");
        assert!((p - 1.0).abs() < 1e-6, "the pitch the 1.0 law keeps");
        // the stereo tail the lateral pan law speaks: the listener faces
        // the vanilla south (yaw 0.0), the ray's lateral split the same
        // dx/dy/dz the reach fold walked answers — the .5-cell center vs
        // the .3 eye keeps the ray's x reach OFF zero (dx = k - 9.8, the
        // integer k the cell grid keeps), so the ` s` field the fire
        // always wears here
        let want_pan = lateral_pan(dx, dy, dz, 0.0);
        assert!(want_pan != 0.0, "the test's own geometry never centers");
        assert!(
            line.ends_with(&format!(" s{want_pan:+.6}")),
            "the lateral tail the ray's split speaks: {line}"
        );
        // the reset: the next dark walk starts from zero again
        assert_eq!(m.moodiness(), 0.0);
    }

    /// THE LIGHT'S OWN SPEND: a torch-lit sample (block 2+) and a
    /// sky-lit sample both spend the mood DOWN — the pitch dark the only
    /// banker.
    #[test]
    fn the_cave_mood_spends_the_light_back() {
        let mut log = BlockSounds::new(1);
        // the torch: block 2 spends 1/6000 a tick — the darkness' own
        // rate mirrored, the mood never leaves the floor
        let torch = |_: i32, _: i32, _: i32| (0u8, 2u8);
        let mut m = CaveMood::new(7);
        for i in 0..20_000u64 {
            m.tick(i, &torch, 10.3, 20.7, 30.9, 0.0, &mut log);
        }
        assert!(log.events.is_empty(), "the torch never banks");
        assert_eq!(m.moodiness(), 0.0);
        // the sky: a full-sky sample spends 15/15 * 0.001 = 0.001 a tick
        // — the daylight's own calm, the floor the zero keeps
        let sky = |_: i32, _: i32, _: i32| (15u8, 0u8);
        let mut m = CaveMood::new(7);
        for i in 0..5_000u64 {
            m.tick(i, &sky, 10.3, 20.7, 30.9, 0.0, &mut log);
        }
        assert_eq!(m.moodiness(), 0.0);
        // the block-1 sample spends NOTHING ((1-1)/6000 = 0) — the dim
        // glow neither banks nor spends
        let dim = |_: i32, _: i32, _: i32| (0u8, 1u8);
        let mut m = CaveMood::new(7);
        for i in 0..5_999u64 {
            m.tick(i, &dim, 10.3, 20.7, 30.9, 0.0, &mut log);
        }
        assert_eq!(m.moodiness(), 0.0);
        // the mixed world: the dark ticks bank 1/6000, the lit ticks
        // spend 0.001 — the spend outruns the bank (one lit tick swallows
        // six dark banks' worth), the mood never fires inside the walk
        let world = |x: i32, _: i32, _: i32| {
            if x % 2 == 0 {
                (0u8, 0u8)
            } else {
                (15u8, 0u8)
            }
        };
        let mut m = CaveMood::new(7);
        for i in 0..6_000u64 {
            m.tick(i, &world, 10.3, 20.7, 30.9, 0.0, &mut log);
        }
        assert!(log.events.is_empty(), "the lit world never fires");
        assert!(m.moodiness() < 1.0);
    }

    /// THE SAMPLE'S OWN WALK: the three draws ride [-extent, +extent]
    /// around the anchor — the sampled cells the floor the containing
    /// keeps, the eye anchor the body's own eye answers.
    #[test]
    fn the_cave_mood_samples_within_the_block_search_extent() {
        let seen = std::cell::RefCell::new(Vec::new());
        {
            let seen_ref = &seen;
            // the probe world rides the sample taps: the machine's own
            // light closure records the sampled cells (the darkness the
            // 6000-tick walk banks — one fire the record's tail keeps)
            let mut m = CaveMood::new(31);
            let mut log = BlockSounds::new(1);
            let record = |x: i32, y: i32, z: i32| -> (u8, u8) {
                seen_ref.borrow_mut().push((x, y, z));
                (0u8, 0u8)
            };
            for i in 0..MOOD_TICK_DELAY {
                m.tick(i as u64, &record, 100.5, 50.5, 100.5, 0.0, &mut log);
            }
        }
        let cells = seen.borrow();
        assert_eq!(cells.len(), MOOD_TICK_DELAY as usize, "one sample a tick");
        // the extents: every cell within ±8 of the anchor (the anchor
        // rides the block centers — the containing floor the eye's own
        // cell keeps, the draws' bound the 17 the extent*2+1 speaks)
        for (x, y, z) in cells.iter() {
            assert!(
                (*x - 100).abs() <= MOOD_BLOCK_SEARCH_EXTENT as i32,
                "x {x} outside the extent"
            );
            assert!(
                (*y - 50).abs() <= MOOD_BLOCK_SEARCH_EXTENT as i32,
                "y {y} outside the extent"
            );
            assert!(
                (*z - 100).abs() <= MOOD_BLOCK_SEARCH_EXTENT as i32,
                "z {z} outside the extent"
            );
        }
        // the walk spreads (the draws' variety the 6000 samples speak —
        // 4913 cells inside the box, a healthy share the walk visits)
        let mut uniq = cells.clone();
        uniq.sort();
        uniq.dedup();
        assert!(uniq.len() > 1000, "the walk visited {} cells", uniq.len());
    }
}

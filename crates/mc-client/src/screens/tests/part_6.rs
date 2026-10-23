use super::*;


    #[test]
    fn the_wire_fixed_voice_words_the_center_voices_wear() {
        // THE FIXED-VOICE SHELF'S OWN WIRE LANE (javap the cases
        // 1033/1034/1035/1049/1050 → offsets 680/700/660/796/816 over
        // the 26.2 jar): the FIVE `playLocalSound(pos, <voice>,
        // BLOCKS, 1.0F, 1.0F, false)` walks the case bodies spend —
        // each voice the fixed `fconst_1` pair carries, the BLOCK
        // CENTER the playLocalSound(BlockPos, ...) resolve places, NO
        // PARTICLES the cases speak (the voices alone the broadcasts
        // render), the level ambient stream the fixed folds never
        // spend.
        let mut app = App::new(2);
        app.enter_world();
        // THE STREAM'S OWN PIN (the fresh LCG the swap banks — no
        // draw the fixed 1.0/1.0 folds cost, the `fconst_1` pairs the
        // randoms never ride)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(4321));
        let particles_before = app.particles.len();
        // THE FIVE VOICES (each word the packet's own effect id — the
        // sound key the official sounds.json table folds, one voice
        // per arm)
        let five = [
            (mc_protocol::SOUND_CHORUS_GROW_EFFECT, "block.chorus_flower.grow"),
            (mc_protocol::SOUND_CHORUS_DEATH_EFFECT, "block.chorus_flower.death"),
            (mc_protocol::SOUND_BREWING_STAND_BREW_EFFECT, "block.brewing_stand.brew"),
            (mc_protocol::SOUND_CRAFTER_CRAFT_EFFECT, "block.crafter.craft"),
            (mc_protocol::SOUND_CRAFTER_FAIL_EFFECT, "block.crafter.fail"),
        ];
        app.live_fixed_voice_events_updated(
            five.iter()
                .map(|(effect, _)| crate::world_view::WireFixedVoiceEvent {
                    x: 10,
                    y: 20,
                    z: 10,
                    voice: *effect,
                })
                .collect(),
        );
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        assert_eq!(
            spent,
            crate::drops::Lcg::new(4321),
            "the fixed folds the ambient stream never spends"
        );
        assert_eq!(
            app.particles.len(),
            particles_before,
            "no particles the case bodies speak"
        );
        // THE VOICES (each case's own body its key speaks — the
        // volume/pitch BOTH the fixed 1.0 the `fconst_1` pairs keep,
        // the file picks the engine's own LCG banks)
        if sounds_loaded() {
            for (effect, key) in five {
                let voices = app
                    .sounds
                    .events
                    .iter()
                    .filter(|e| e.starts_with(&format!("{key} @")))
                    .count();
                assert_eq!(voices, 1, "the ONE playLocalSound case {effect} keeps ({key})");
                let voice = app
                    .sounds
                    .events
                    .iter()
                    .find(|e| e.starts_with(&format!("{key} @")))
                    .expect("the voice the case speaks");
                let pitch: f32 = voice
                    .split(" p")
                    .nth(1)
                    .and_then(|t| t[..6].parse().ok())
                    .unwrap_or(0.0);
                assert!(
                    (pitch - 1.0).abs() < 1e-6,
                    "the fixed pitch the fconst_1 keeps: {pitch}"
                );
                let volume: f32 = voice
                    .split(" v")
                    .nth(1)
                    .and_then(|t| t[..6].parse().ok())
                    .unwrap_or(0.0);
                assert!(
                    (volume - 1.0).abs() < 1e-6,
                    "the fixed volume the fconst_1 keeps: {volume}"
                );
            }
        }
        // THE UNKNOWN WORD (the model's own arm the five ids alone
        // enqueues — the defensive arm the honest walk never banks,
        // the sound lane's silence keeps)
        if sounds_loaded() {
            let voices_before = app.sounds.events.len();
            app.live_fixed_voice_events_updated(vec![
                crate::world_view::WireFixedVoiceEvent {
                    x: 1,
                    y: 2,
                    z: 3,
                    voice: 4242,
                },
            ]);
            assert_eq!(
                app.sounds.events.len(),
                voices_before,
                "the unknown word the walk never banks"
            );
        } else {
            app.live_fixed_voice_events_updated(vec![
                crate::world_view::WireFixedVoiceEvent {
                    x: 1,
                    y: 2,
                    z: 3,
                    voice: 4242,
                },
            ]);
        }
        // THE SECOND ARM (the next broadcast the same walks wear —
        // each word ships exactly one voice per arrival)
        if sounds_loaded() {
            let voices_before = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.crafter.fail @"))
                .count();
            app.live_fixed_voice_events_updated(vec![
                crate::world_view::WireFixedVoiceEvent {
                    x: 0,
                    y: 0,
                    z: 0,
                    voice: mc_protocol::SOUND_CRAFTER_FAIL_EFFECT,
                },
            ]);
            let voices_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.crafter.fail @"))
                .count();
            assert_eq!(
                voices_after,
                voices_before + 1,
                "each arm exactly one voice the walk keeps"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        if sounds_loaded() {
            let voices_before = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.crafter.fail @"))
                .count();
            app.live_fixed_voice_events_updated(vec![]);
            let voices_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.crafter.fail @"))
                .count();
            assert_eq!(
                voices_after, voices_before,
                "the empty mailbox ships nothing"
            );
        } else {
            app.live_fixed_voice_events_updated(vec![]);
        }
    }


    #[test]
    fn the_hostile_voice_serve_the_volume_the_pitch_wear() {
        // THE HOSTILE-VOICE SHELF'S OWN SERVE (javap the cases
        // 1019/1020/1021/1022 — offsets 4573/4693/4653/4613 — the ONE
        // `playLocalSound(pos, <voice>, HOSTILE, 2.0F, (nf - nf) *
        // 0.2F + 1.0F, false)` walk each body spends: the fconst_2
        // volume the DOUBLED gain keeps, the TWO pitch draws the
        // [0.8, 1.2] band the 0.2F scale gives, the four sound keys
        // the runtime table resolves, no particles the voice-only
        // shelf speaks, the empty mailbox the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE BODY 24 BLOCKS OFF (the ear the doubled reach's own
        // linear answers — a 2.0-volume voice's max distance the
        // `Math.max(getVolume(), 1.0F) * 16.0F` fold speaks: 32.0
        // blocks the fconst_2 doubles, the gain the linear keeps at
        // 2.0 x (1 - 24/32) = 0.5 — a 1.0-volume voice the SAME
        // distance the zero gain kills)
        app.body = Some(crate::body::PlayerBody::new(
            -16.5,
            70.5 - crate::body::STANDING_EYE,
            123.5,
            0.0,
            0.0,
        ));
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x1019));
        let before = app.particles.len();
        app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
            x: -41,
            y: 70,
            z: 123,
            voice: mc_protocol::SOUND_ZOMBIE_WOODEN_DOOR_EFFECT,
        }]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE VOICE-ONLY SHELF (no particles the cases speak)
        assert_eq!(app.particles.len(), before, "the voice alone renders");
        // THE EXACT TWO DRAWS (the pitch pair the case body's own
        // fsub/fmul/fadd chain answers)
        let mut replay = crate::drops::Lcg::new(0x1019);
        let _ = replay.next_f32();
        let _ = replay.next_f32();
        assert_eq!(spent, replay, "the case body's own exact stream");
        if sounds_loaded() {
            // THE DOUBLED REACH (the dist-24 ear the 0.5 gain the
            // fconst_2's own linear speaks — the 32-block max the
            // 2.0-volume fold arms)
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.zombie.attack_wooden_door @"))
                .expect("the wooden-door voice the lane logs");
            assert!(
                line.contains(" v0.500000 p"),
                "the fconst_2's own doubled reach the dist-24 ear speaks: {line}"
            );
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE DIST-0 CEILING (the ear at the block center the
        // attenuation's own clamp keeps — the doubled gain the
        // AL_MAX_GAIN 1.0 ceiling caps at FULL loudness the vanilla
        // engine's own gain law speaks)
        app.body = Some(crate::body::PlayerBody::new(
            -40.5,
            70.5 - crate::body::STANDING_EYE,
            123.5,
            0.0,
            0.0,
        ));
        app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
            x: -41,
            y: 70,
            z: 123,
            voice: mc_protocol::SOUND_ZOMBIE_WOODEN_DOOR_EFFECT,
        }]);
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .rev()
                .find(|e| e.starts_with("entity.zombie.attack_wooden_door @"))
                .expect("the dist-0 voice the lane logs");
            assert!(
                line.contains(" v1.000000 p"),
                "the doubled gain's own ceiling the dist-0 ear keeps: {line}"
            );
        }
        // THE THREE SIBLINGS (the same walk each word's own key
        // wears — the iron-door attack, the door crash, the wither's
        // block-break the shelf banks)
        for (word, key) in [
            (
                mc_protocol::SOUND_ZOMBIE_IRON_DOOR_EFFECT,
                "entity.zombie.attack_iron_door",
            ),
            (
                mc_protocol::SOUND_ZOMBIE_DOOR_CRASH_EFFECT,
                "entity.zombie.break_wooden_door",
            ),
            (
                mc_protocol::SOUND_WITHER_BLOCK_BREAK_EFFECT,
                "entity.wither.break_block",
            ),
        ] {
            let before_voices = app.sounds.events.len();
            app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
                x: -41,
                y: 70,
                z: 123,
                voice: word,
            }]);
            if sounds_loaded() {
                assert_eq!(
                    app.sounds.events.len(),
                    before_voices + 1,
                    "each arm exactly one voice the walk keeps"
                );
                let line = app
                    .sounds
                    .events
                    .last()
                    .expect("the sibling voice the lane logs");
                assert!(
                    line.starts_with(key),
                    "the sibling key the word folds: {line}"
                );
                assert!(
                    line.contains(" v1.000000 p"),
                    "the doubled gain's own ceiling every sibling keeps: {line}"
                );
            }
        }
        // THE UNBANKED WORD: the arm the render never voices (the
        // model's own arm the four words alone enqueues — the early
        // return the unknown word answers consumes NO voice, NO draw)
        let voices_before_unknown = app.sounds.events.len();
        let saved2 = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0xBEEF));
        app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
            x: 1,
            y: 2,
            z: 3,
            voice: 4242,
        }]);
        let spent2 = std::mem::replace(&mut app.level_ambient_rng, saved2);
        assert_eq!(
            app.sounds.events.len(),
            voices_before_unknown,
            "the unknown word no voice ever banks"
        );
        assert_eq!(
            spent2,
            crate::drops::Lcg::new(0xBEEF),
            "the unknown word no draw the early return keeps"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let voices_before = app.sounds.events.len();
        app.live_hostile_voice_events_updated(vec![]);
        assert_eq!(
            app.sounds.events.len(),
            voices_before,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_hostile_voice_serve_the_projectile_volume_wear() {
        // THE PROJECTILE-VOICE QUARTET'S OWN SERVE (javap the cases
        // offsets 4447/4531/4489/4733 the same walk verbatim): the
        // `ldc2_w` 10.0F the LOUD trio carries — the reach the
        // `Math.max(getVolume(), 1.0F) * 16.0F` fold turns into 160
        // blocks, the dist-152 ear the v0.500000 gain speaks
        // (10 x (1 - 152/160) = 0.5 — a 2.0-volume voice the SAME
        // distance the ZERO gain keeps, the 32-block reach the
        // fconst_2 doubles is dead at 152) — the blaze's `fconst_2`
        // 2.0F the door-shelf shape keeps, the TWO pitch draws the
        // [0.8, 1.2] band each serve answers, the keys the runtime
        // table resolves, the unknown word no voice no draw the
        // early return keeps, the empty mailbox the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE BODY 152 BLOCKS OFF (the far ear the loud trio's own
        // reach answers: the 10.0-volume max distance the fold
        // speaks is 160.0 blocks — 10 x (1 - 152/160) = 0.5 the
        // linear keeps; the 2.0-volume voices' 32.0-block reach the
        // SAME ear the zero gain gives)
        app.body = Some(crate::body::PlayerBody::new(
            111.5,
            70.5 - crate::body::STANDING_EYE,
            123.5,
            0.0,
            0.0,
        ));
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x1015));
        // THE GHAST'S CHARGE-WARN (the 10.0F the loud reach wears)
        app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
            x: -41,
            y: 70,
            z: 123,
            voice: mc_protocol::SOUND_GHAST_WARNING_EFFECT,
        }]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT TWO DRAWS (the pitch pair the case body's own
        // fsub/fmul/fadd chain answers)
        let mut replay = crate::drops::Lcg::new(0x1015);
        let _ = replay.next_f32();
        let _ = replay.next_f32();
        assert_eq!(spent, replay, "the case body's own exact stream");
        if sounds_loaded() {
            // THE LOUD REACH (the dist-152 ear the 0.5 gain the
            // ldc2_w's own 160-block max speaks — the door shelf's
            // doubled 2.0 the same distance is DEAD)
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.ghast.warn @"))
                .expect("the ghast-warn voice the lane logs");
            assert!(
                line.contains(" v0.500000 p"),
                "the ldc2_w's own loud reach the dist-152 ear speaks: {line}"
            );
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE BLAZE AT THE SAME FAR EAR (the fconst_2's own 32-block
        // reach the dist-152 ear kills — the zero gain the linear
        // floor keeps, the line the queue still carries)
        app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
            x: -41,
            y: 70,
            z: 123,
            voice: mc_protocol::SOUND_BLAZE_FIREBALL_EFFECT,
        }]);
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .rev()
                .find(|e| e.starts_with("entity.blaze.shoot @"))
                .expect("the blaze voice the lane logs");
            assert!(
                line.contains(" v0.000000 p"),
                "the doubled gain's own reach the far ear kills: {line}"
            );
        }
        // THE BLAZE AT THE DOOR-SHELF REACH (the dist-24 ear the 0.5
        // gain the fconst_2's own linear speaks — 2 x (1 - 24/32))
        app.body = Some(crate::body::PlayerBody::new(
            -16.5,
            70.5 - crate::body::STANDING_EYE,
            123.5,
            0.0,
            0.0,
        ));
        app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
            x: -41,
            y: 70,
            z: 123,
            voice: mc_protocol::SOUND_BLAZE_FIREBALL_EFFECT,
        }]);
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .rev()
                .find(|e| e.starts_with("entity.blaze.shoot @"))
                .expect("the dist-24 blaze voice the lane logs");
            assert!(
                line.contains(" v0.500000 p"),
                "the fconst_2's own doubled reach the dist-24 ear speaks: {line}"
            );
        }
        // THE SHOOT SIBLINGS (the same walk each word's own key
        // wears — the ghast's fireball shot, the dragon's strafe
        // fireball the loud trio banks; the dist-24 ear the 10.0
        // volume the AL_MAX_GAIN ceiling clamps at FULL loudness)
        for (word, key) in [
            (
                mc_protocol::SOUND_GHAST_FIREBALL_EFFECT,
                "entity.ghast.shoot",
            ),
            (
                mc_protocol::SOUND_DRAGON_FIREBALL_EFFECT,
                "entity.ender_dragon.shoot",
            ),
        ] {
            let before_voices = app.sounds.events.len();
            app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
                x: -41,
                y: 70,
                z: 123,
                voice: word,
            }]);
            if sounds_loaded() {
                assert_eq!(
                    app.sounds.events.len(),
                    before_voices + 1,
                    "each arm exactly one voice the walk keeps"
                );
                let line = app
                    .sounds
                    .events
                    .last()
                    .expect("the sibling voice the lane logs");
                assert!(
                    line.starts_with(key),
                    "the sibling key the word folds: {line}"
                );
                assert!(
                    line.contains(" v1.000000 p"),
                    "the loud trio's own ceiling the dist-24 ear keeps: {line}"
                );
            }
        }
        // THE UNBANKED WORD: the arm the render never voices (the
        // model's own arm the eight words alone enqueues — the early
        // return the unknown word answers consumes NO voice, NO draw)
        let voices_before_unknown = app.sounds.events.len();
        let saved2 = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0xBEEF));
        app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
            x: 1,
            y: 2,
            z: 3,
            voice: 4242,
        }]);
        let spent2 = std::mem::replace(&mut app.level_ambient_rng, saved2);
        assert_eq!(
            app.sounds.events.len(),
            voices_before_unknown,
            "the unknown word no voice ever banks"
        );
        assert_eq!(
            spent2,
            crate::drops::Lcg::new(0xBEEF),
            "the unknown word no draw the early return keeps"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let voices_before = app.sounds.events.len();
        app.live_hostile_voice_events_updated(vec![]);
        assert_eq!(
            app.sounds.events.len(),
            voices_before,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_hostile_voice_serve_the_zombie_flock_the_volume_wear() {
        // THE ZOMBIE-FLOCK BATCH'S OWN SERVE (javap the cases
        // offsets 4773/4813/4853/4893/4933 the same walk verbatim):
        // the `fconst_2` 2.0F the five words carry — the reach the
        // `Math.max(getVolume(), 1.0F) * 16.0F` fold turns into 32
        // blocks, the dist-24 ear the v0.500000 gain speaks
        // (2 x (1 - 24/32) = 0.5 — the door-shelf shape the batch
        // shares), the TWO pitch draws the [0.8, 1.2] band each
        // serve answers, the keys the runtime table resolves (the
        // wither shoot + the zombie infect + the villager conversion
        // + the drowned + the husk the official sounds.json rows
        // bank), the unknown word no voice no draw the early return
        // keeps, the empty mailbox the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE BODY 24 BLOCKS OFF (the door-shelf ear the batch's own
        // reach answers: the 2.0-volume max distance the fold speaks
        // is 32.0 blocks — 2 x (1 - 24/32) = 0.5 the linear keeps)
        app.body = Some(crate::body::PlayerBody::new(
            -16.5,
            70.5 - crate::body::STANDING_EYE,
            123.5,
            0.0,
            0.0,
        ));
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x1024));
        // THE WITHER BOSS'S OWN SHOOT (the 2.0F the flock's reach
        // wears — the launch tail the performRangedAttack walk fires)
        app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
            x: -41,
            y: 70,
            z: 123,
            voice: mc_protocol::SOUND_WITHER_BOSS_SHOOT_EFFECT,
        }]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT TWO DRAWS (the pitch pair the case body's own
        // fsub/fmul/fadd chain answers)
        let mut replay = crate::drops::Lcg::new(0x1024);
        let _ = replay.next_f32();
        let _ = replay.next_f32();
        assert_eq!(spent, replay, "the case body's own exact stream");
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.wither.shoot @"))
                .expect("the wither-shoot voice the lane logs");
            assert!(
                line.contains(" v0.500000 p"),
                "the fconst_2's own doubled reach the dist-24 ear speaks: {line}"
            );
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE FOUR SIBLINGS (the same walk each word's own key wears
        // — the zombie infect, the villager conversion, the drowned,
        // the husk the flock banks; the dist-24 ear the 2.0 volume
        // the 0.5 gain the linear speaks)
        for (word, key) in [
            (
                mc_protocol::SOUND_ZOMBIE_INFECTED_EFFECT,
                "entity.zombie.infect",
            ),
            (
                mc_protocol::SOUND_ZOMBIE_CONVERTED_EFFECT,
                "entity.zombie_villager.converted",
            ),
            (
                mc_protocol::SOUND_ZOMBIE_TO_DROWNED_EFFECT,
                "entity.zombie.converted_to_drowned",
            ),
            (
                mc_protocol::SOUND_HUSK_TO_ZOMBIE_EFFECT,
                "entity.husk.converted_to_zombie",
            ),
        ] {
            let before_voices = app.sounds.events.len();
            app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
                x: -41,
                y: 70,
                z: 123,
                voice: word,
            }]);
            if sounds_loaded() {
                assert_eq!(
                    app.sounds.events.len(),
                    before_voices + 1,
                    "each arm exactly one voice the walk keeps"
                );
                let line = app
                    .sounds
                    .events
                    .last()
                    .expect("the sibling voice the lane logs");
                assert!(
                    line.starts_with(key),
                    "the sibling key the word folds: {line}"
                );
                assert!(
                    line.contains(" v0.500000 p"),
                    "the flock's own doubled gain the dist-24 ear keeps: {line}"
                );
            }
        }
        // THE UNBANKED WORD: the arm the render never voices (the
        // model's own arm the fourteen words alone enqueues — the
        // early return the unknown word answers consumes NO voice,
        // NO draw)
        let voices_before_unknown = app.sounds.events.len();
        let saved2 = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0xBEEF));
        app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
            x: 1,
            y: 2,
            z: 3,
            voice: 4242,
        }]);
        let spent2 = std::mem::replace(&mut app.level_ambient_rng, saved2);
        assert_eq!(
            app.sounds.events.len(),
            voices_before_unknown,
            "the unknown word no voice ever banks"
        );
        assert_eq!(
            spent2,
            crate::drops::Lcg::new(0xBEEF),
            "the unknown word no draw the early return keeps"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let voices_before = app.sounds.events.len();
        app.live_hostile_voice_events_updated(vec![]);
        assert_eq!(
            app.sounds.events.len(),
            voices_before,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_pitch_draw_serve_the_gentle_band_the_volume_wear() {
        // THE ONE-DRAW PITCH SHELF'S OWN SERVE (javap the eleven case
        // arms the same walk verbatim): the SINGLE nextFloat draw the
        // `nf * 0.1F + 0.9F` chain answers — the [0.9, 1.0] band the
        // 0.1F scale gives (NOT the hostile shelf's two-draw stream),
        // the per-word volume the cases' own constants keep (the
        // `fconst_1` 1.0F five, the `ldc2_w` 0.3F the land + the
        // phantom's bite, the `fconst_2` 2.0F the dripstone/cauldron/
        // sulfur quartet), the reach the `Math.max(getVolume(), 1.0F)
        // * 16.0F` fold speaks (16 for the 1.0 + the 0.3 — the fold's
        // own floor — 32 for the 2.0), the keys the runtime table
        // resolves (the eleven official sounds.json rows bank), the
        // unknown word no voice no draw the early return keeps, the
        // empty mailbox the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE BODY 24 BLOCKS OFF (the 2.0-volume ear the fold's own
        // doubled reach answers: 2 x (1 - 24/32) = 0.5 the linear
        // keeps — the sulfur spike's landing the dist-24 ear speaks)
        app.body = Some(crate::body::PlayerBody::new(
            -16.5,
            70.5 - crate::body::STANDING_EYE,
            123.5,
            0.0,
            0.0,
        ));
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x461));
        // THE SULFUR SPIKE'S OWN LANDING (the 2.0F the fconst_2's own
        // reach wears — the SulfurSpikeBlock landing word the
        // getStalactiteLandingSound walk fires)
        app.live_pitch_draw_voice_events_updated(vec![crate::world_view::WirePitchDrawVoiceEvent {
            x: -41,
            y: 70,
            z: 123,
            voice: mc_protocol::SOUND_SULFUR_SPIKE_LAND_EFFECT,
        }]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT ONE DRAW (the pitch the case body's own fmul/fadd
        // chain answers — the single draw the gentle law keeps)
        let mut replay = crate::drops::Lcg::new(0x461);
        let _ = replay.next_f32();
        assert_eq!(spent, replay, "the case body's own exact stream");
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.sulfur_spike.land @"))
                .expect("the sulfur-spike voice the lane logs");
            assert!(
                line.contains(" v0.500000 p"),
                "the fconst_2's own doubled reach the dist-24 ear speaks: {line}"
            );
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.9..=1.0).contains(&pitch),
                "the 0.1F-scale gentle pitch band: {}",
                pitch
            );
        }
        // THE ELEVEN WORDS (the same walk each word's own key wears —
        // the body now EIGHT blocks off: the 1.0-volume words the
        // 0.5 gain the linear speaks (1 x (1 - 8/16)), the 0.3-volume
        // words the 0.15 gain (0.3 x (1 - 8/16) — the fold's own
        // floor the reach keeps at 16), the 2.0-volume words the
        // AL_MAX_GAIN clamp keeps at 1.000000 (2 x (1 - 8/32) = 1.5
        // the ceiling answers); each serve the ONE draw the stream
        // replays)
        app.body = Some(crate::body::PlayerBody::new(
            -32.5,
            70.5 - crate::body::STANDING_EYE,
            123.5,
            0.0,
            0.0,
        ));
        let words: [(i32, &str, f32); 11] = [
            (mc_protocol::SOUND_ANVIL_BROKEN_EFFECT, "block.anvil.destroy", 0.5),
            (mc_protocol::SOUND_ANVIL_USED_EFFECT, "block.anvil.use", 0.5),
            (mc_protocol::SOUND_ANVIL_LAND_EFFECT, "block.anvil.land", 0.15),
            (mc_protocol::SOUND_PHANTOM_BITE_EFFECT, "entity.phantom.bite", 0.15),
            (mc_protocol::SOUND_GRINDSTONE_USED_EFFECT, "block.grindstone.use", 0.5),
            (mc_protocol::SOUND_PAGE_TURN_EFFECT, "item.book.page_turn", 0.5),
            (
                mc_protocol::SOUND_SMITHING_TABLE_USED_EFFECT,
                "block.smithing_table.use",
                0.5,
            ),
            (
                mc_protocol::SOUND_POINTED_DRIPSTONE_LAND_EFFECT,
                "block.pointed_dripstone.land",
                1.0,
            ),
            (
                mc_protocol::SOUND_DRIP_LAVA_INTO_CAULDRON_EFFECT,
                "block.pointed_dripstone.drip_lava_into_cauldron",
                1.0,
            ),
            (
                mc_protocol::SOUND_DRIP_WATER_INTO_CAULDRON_EFFECT,
                "block.pointed_dripstone.drip_water_into_cauldron",
                1.0,
            ),
            (
                mc_protocol::SOUND_SULFUR_SPIKE_LAND_EFFECT,
                "block.sulfur_spike.land",
                1.0,
            ),
        ];
        for (i, (word, key, want_gain)) in words.iter().enumerate() {
            let saved_i = std::mem::replace(
                &mut app.level_ambient_rng,
                crate::drops::Lcg::new(0x461 + i as u64),
            );
            let voices_before_i = app.sounds.events.len();
            app.live_pitch_draw_voice_events_updated(vec![crate::world_view::WirePitchDrawVoiceEvent {
                x: -41,
                y: 70,
                z: 123,
                voice: *word,
            }]);
            let spent_i = std::mem::replace(&mut app.level_ambient_rng, saved_i);
            // THE EXACT ONE DRAW (the single nextFloat the case body's
            // own gentle law answers — never the hostile pair)
            let mut replay_i = crate::drops::Lcg::new(0x461 + i as u64);
            let _ = replay_i.next_f32();
            assert_eq!(spent_i, replay_i, "the one-draw law word {word}");
            if sounds_loaded() {
                assert_eq!(
                    app.sounds.events.len(),
                    voices_before_i + 1,
                    "each arm exactly one voice the walk keeps"
                );
                let line = app
                    .sounds
                    .events
                    .last()
                    .expect("the gentle voice the lane logs");
                assert!(
                    line.starts_with(key),
                    "the gentle key the word folds: {line}"
                );
                assert!(
                    line.contains(&format!(" v{want_gain:.6} p")),
                    "the {want_gain:.6} gain the dist-8 ear speaks: {line}"
                );
            }
        }
        // THE UNBANKED WORD: the arm the render never voices (the
        // model's own arm the eleven words alone enqueues — the early
        // return the unknown word answers consumes NO voice, NO draw)
        let voices_before_unknown = app.sounds.events.len();
        let saved2 = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0xBEEF));
        app.live_pitch_draw_voice_events_updated(vec![crate::world_view::WirePitchDrawVoiceEvent {
            x: 1,
            y: 2,
            z: 3,
            voice: 4242,
        }]);
        let spent2 = std::mem::replace(&mut app.level_ambient_rng, saved2);
        assert_eq!(
            app.sounds.events.len(),
            voices_before_unknown,
            "the unknown word no voice ever banks"
        );
        assert_eq!(
            spent2,
            crate::drops::Lcg::new(0xBEEF),
            "the unknown word no draw the early return keeps"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let voices_before = app.sounds.events.len();
        app.live_pitch_draw_voice_events_updated(vec![]);
        assert_eq!(
            app.sounds.events.len(),
            voices_before,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_voice_serve_the_stray_and_bat_the_volume_wear() {
        // THE CONVERSION + NEUTRAL ROUND'S OWN SERVE (javap the cases
        // offsets 5307 + 4973 the same walks verbatim): the stray
        // swap the `fconst_2` 2.0F the flock's reach wears — the
        // dist-24 ear the v0.500000 gain speaks (2 x (1 - 24/32) =
        // 0.5 — the door-shelf shape the flock shares), the bat's
        // liftoff the `ldc2_w` 0.05F the wingbeat's own whisper keeps
        // (the reach the max(volume, 1.0) * 16 fold still turns into
        // 16 blocks — the 1.0 floor — the dist-8 ear the v0.025000
        // gain speaks: 0.05 x (1 - 8/16)), the TWO pitch draws the
        // [0.8, 1.2] band each serve answers, the keys the runtime
        // table resolves (the converted-to-stray + the bat takeoff
        // the official sounds.json rows bank), the unknown word no
        // voice no draw the early return keeps, the empty mailbox the
        // no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE BODY 24 BLOCKS OFF (the flock's ear the stray's own
        // reach answers)
        app.body = Some(crate::body::PlayerBody::new(
            -16.5,
            70.5 - crate::body::STANDING_EYE,
            123.5,
            0.0,
            0.0,
        ));
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x1048));
        // THE STRAY SWAP (the 2.0F the flock's own form wears — the
        // freeze-completion tail the Skeleton lambda fires)
        app.live_hostile_voice_events_updated(vec![crate::world_view::WireHostileVoiceEvent {
            x: -41,
            y: 70,
            z: 123,
            voice: mc_protocol::SOUND_SKELETON_TO_STRAY_EFFECT,
        }]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT TWO DRAWS (the pitch pair the case body's own
        // fsub/fmul/fadd chain answers)
        let mut replay = crate::drops::Lcg::new(0x1048);
        let _ = replay.next_f32();
        let _ = replay.next_f32();
        assert_eq!(spent, replay, "the case body's own exact stream");
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.skeleton.converted_to_stray @"))
                .expect("the stray voice the lane logs");
            assert!(
                line.contains(" v0.500000 p"),
                "the fconst_2's own doubled reach the dist-24 ear speaks: {line}"
            );
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE BODY 8 BLOCKS OFF (the wingbeat ear the 0.05F whisper
        // answers — the 1.0-floor reach the 16-block band keeps)
        app.body = Some(crate::body::PlayerBody::new(
            -32.5,
            70.5 - crate::body::STANDING_EYE,
            123.5,
            0.0,
            0.0,
        ));
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x1025));
        // THE BAT LIFTOFF (the 0.05F the ldc2_w speaks — the wingbeat
        // the wake-up tail fires)
        app.live_neutral_voice_events_updated(vec![crate::world_view::WireNeutralVoiceEvent {
            x: -41,
            y: 70,
            z: 123,
            voice: mc_protocol::SOUND_BAT_LIFTOFF_EFFECT,
        }]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT TWO DRAWS (the SAME chain the hostile law keeps)
        let mut replay = crate::drops::Lcg::new(0x1025);
        let _ = replay.next_f32();
        let _ = replay.next_f32();
        assert_eq!(spent, replay, "the wingbeat's own exact stream");
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.bat.takeoff @"))
                .expect("the bat voice the lane logs");
            assert!(
                line.contains(" v0.025000 p"),
                "the 0.05F whisper the dist-8 ear speaks: {line}"
            );
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE UNBANKED WORD: the arm the render never voices (the
        // model's own arm the one word alone enqueues — the early
        // return the unknown word answers consumes NO voice, NO draw)
        let voices_before_unknown = app.sounds.events.len();
        let saved2 = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0xBEEF));
        app.live_neutral_voice_events_updated(vec![crate::world_view::WireNeutralVoiceEvent {
            x: 1,
            y: 2,
            z: 3,
            voice: 4242,
        }]);
        let spent2 = std::mem::replace(&mut app.level_ambient_rng, saved2);
        assert_eq!(
            app.sounds.events.len(),
            voices_before_unknown,
            "the unknown word no voice ever banks"
        );
        assert_eq!(
            spent2,
            crate::drops::Lcg::new(0xBEEF),
            "the unknown word no draw the early return keeps"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let voices_before = app.sounds.events.len();
        app.live_neutral_voice_events_updated(vec![]);
        assert_eq!(
            app.sounds.events.len(),
            voices_before,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_wind_and_portal_serve_the_fdiv_and_local_laws() {
        // THE WIND + PORTAL ROUND'S OWN SERVE (javap the cases offsets
        // 878 + 720 the same walks verbatim): the wind-charge shot the
        // `ldc1 0.5` volume the fdiv law wears — the dist-8 ear the
        // v0.250000 gain speaks (0.5 x (1 - 8/16) — the 16-block reach
        // the max(volume, 1.0) fold keeps), the ONE pitch draw the
        // `0.4F / (nf * 0.4F + 0.8F)` chain answers (the (1/3, 0.5]
        // band the [0.8, 1.2) denominator gives), the portal travel
        // the LOCAL ambience the listener ride keeps (the dist-0 ear
        // the v0.250000 gain the 0.25 volume itself speaks, the ONE
        // pitch draw the [0.8, 1.2) band the 0.4F scale gives, the
        // panless lane the local instance answers), the keys the
        // runtime table resolves (the wind-charge throw + the portal
        // travel the official sounds.json rows bank), the unknown word
        // no voice no draw the early return keeps, the empty mailbox
        // the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE BODY 8 BLOCKS OFF (the dispenser ear the wind charge's
        // own reach answers)
        app.body = Some(crate::body::PlayerBody::new(
            -32.5,
            70.5 - crate::body::STANDING_EYE,
            123.5,
            0.0,
            0.0,
        ));
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x1051));
        // THE WIND-CHARGE SHOT (the 0.5F the pc-889 push wears — the
        // dispenser shot the override fires)
        app.live_wind_charge_shoot_events_updated(vec![
            crate::world_view::WireWindChargeShootEvent {
                x: -41,
                y: 70,
                z: 123,
                voice: mc_protocol::SOUND_WIND_CHARGE_SHOOT_EFFECT,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT ONE DRAW (the pitch the case body's own fdiv chain
        // answers)
        let mut replay = crate::drops::Lcg::new(0x1051);
        let _ = replay.next_f32();
        assert_eq!(spent, replay, "the case body's own exact stream");
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.wind_charge.throw @"))
                .expect("the wind voice the lane logs");
            assert!(
                line.contains(" v0.250000 p"),
                "the 0.5F volume the dist-8 ear speaks: {line}"
            );
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.3333..=0.5).contains(&pitch),
                "the fdiv law's own pitch band: {}",
                pitch
            );
        }
        // THE PORTAL TRAVEL (the LOCAL ambience the listener ride
        // keeps — the dist-0/pan-0 pair the fold answers, the body
        // position the local lane never reads)
        app.body = None;
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x1032));
        app.live_portal_travel_events_updated(vec![crate::world_view::WirePortalTravelEvent {}]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT ONE DRAW (the pitch the case body's own fmul/fadd
        // chain answers)
        let mut replay = crate::drops::Lcg::new(0x1032);
        let _ = replay.next_f32();
        assert_eq!(spent, replay, "the local lane's own exact stream");
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.portal.travel @"))
                .expect("the portal voice the lane logs");
            assert!(
                line.contains(" v0.250000 p"),
                "the 0.25F volume the dist-0 ear the local lane speaks: {line}"
            );
            assert!(
                !line.contains(" s"),
                "the local ambience the panless lane keeps (no stereo split): {line}"
            );
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.4F-scale pitch band: {}",
                pitch
            );
        }
        // THE UNBANKED WORD: the arm the render never voices (the
        // model's own arm the one word alone enqueues — the early
        // return the unknown word answers consumes NO voice, NO draw)
        let voices_before_unknown = app.sounds.events.len();
        let saved2 = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0xBEEF));
        app.live_wind_charge_shoot_events_updated(vec![
            crate::world_view::WireWindChargeShootEvent {
                x: 1,
                y: 2,
                z: 3,
                voice: 4242,
            },
        ]);
        let spent2 = std::mem::replace(&mut app.level_ambient_rng, saved2);
        assert_eq!(
            app.sounds.events.len(),
            voices_before_unknown,
            "the unknown word no voice ever banks"
        );
        assert_eq!(
            spent2,
            crate::drops::Lcg::new(0xBEEF),
            "the unknown word no draw the early return keeps"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps (both
        // lanes the empty vec answers with nothing)
        let voices_before = app.sounds.events.len();
        app.live_wind_charge_shoot_events_updated(vec![]);
        app.live_portal_travel_events_updated(vec![]);
        assert_eq!(
            app.sounds.events.len(),
            voices_before,
            "the empty mailboxes ship nothing"
        );
    }


    #[test]
    fn the_wire_spawn_word_the_forty_particle_walk_wears() {
        // THE MOB-SPAWN CLOUD'S OWN WIRE LANE (javap case 2004 →
        // offset 1689 over the 26.2 jar): the twenty SMOKE + FLAME
        // pairs the `bipush 20` loop fires, each pair AT THE SAME
        // full-block jitter spot the `(nextDouble() - 0.5) * 2.0` fold
        // places, the zero velocity args the `dconst_0` trio speaks,
        // the ctor chains the torch's ambient pair already banks.
        let mut app = App::new(2);
        app.enter_world();
        app.live_mob_spawn_events_updated(vec![crate::world_view::WireMobSpawnEvent {
            x: 10,
            y: 20,
            z: 10,
        }]);
        assert_eq!(
            app.particles.len(),
            40,
            "the bipush 20 the pair loop keeps (20 x SMOKE + FLAME)"
        );
        for i in 0..20 {
            // THE PAIR ORDER (the SMOKE addParticle first, the FLAME
            // second — the case's own call order the vec's own order
            // keeps)
            let smoke = &app.particles[i * 2];
            let flame = &app.particles[i * 2 + 1];
            assert!(
                matches!(smoke.kind, crate::particles::ParticleKind::Smoke),
                "the SMOKE type the provider answers first"
            );
            assert!(
                matches!(flame.kind, crate::particles::ParticleKind::Flame),
                "the FLAME type the provider answers second"
            );
            // THE PAIR'S OWN SPOT (the same px/py/pz the fold places —
            // the flame's ctor jitter the ±0.05 band adds, the smoke's
            // position the fold verbatim keeps)
            assert!(
                (smoke.x - flame.x).abs() < 0.06,
                "the shared spot the pair's own fold keeps (the flame's ctor jitter alone)"
            );
            assert!(
                (smoke.y - flame.y).abs() < 0.06,
                "the shared spot the pair's own fold keeps"
            );
            assert!(
                (smoke.z - flame.z).abs() < 0.06,
                "the shared spot the pair's own fold keeps"
            );
            // THE FULL-BLOCK SWEEP (the ±1.0 band around the block's
            // own center — the whole cell the vanilla loop sweeps)
            for p in [smoke, flame] {
                assert!(
                    p.x > 9.5 && p.x < 11.5,
                    "the X sweep the (draw - 0.5) * 2.0 fold gives"
                );
                assert!(
                    p.y > 19.5 && p.y < 21.5,
                    "the Y sweep the same fold gives"
                );
                assert!(
                    p.z > 9.5 && p.z < 11.5,
                    "the Z sweep the same fold gives"
                );
            }
            // THE ZERO-ARG WALKS (the smoke's drift the ctor spread *
            // 0.1 fold gives — tiny, with the +0.001 rise the bump
            // scales to; the flame's the * 0.01 fold's own whisper)
            assert!(smoke.vx.abs() < 0.02 && smoke.vz.abs() < 0.02, "the smoke's spread whisper the zero args keep");
            assert!(smoke.vy > -0.009 && smoke.vy < 0.029, "the smoke's rise the +0.1 bump scales to (+0.01, the spread the band sweeps)");
            assert!(flame.vx.abs() < 0.002 && flame.vz.abs() < 0.002, "the flame's spread whisper the zero args keep");
            assert!(flame.vy > -0.0009 && flame.vy < 0.0029, "the flame's rise the +0.1 bump scales to (+0.001, the spread the band sweeps)");
            // THE TWO WALKS' OWN LAWS (the smoke's −0.1 gravity the
            // ash ctor passes, the flame's none the RisingParticle
            // never arms; the 0.96 friction both ctors set)
            assert!((smoke.gravity + 0.1).abs() < 1e-9, "the smoke's gravity the ash ctor passes");
            assert!((flame.gravity).abs() < 1e-9, "the flame's no-gravity the RisingParticle keeps");
            assert!((smoke.friction - 0.96).abs() < 1e-9);
            assert!((flame.friction - 0.96).abs() < 1e-9);
            // THE TINTS (the smoke's grey the r*0.3 draw, the flame's
            // white the ctor defaults speak)
            assert_eq!(smoke.col[0], smoke.col[1], "the smoke's shared grey");
            assert_eq!(smoke.col[1], smoke.col[2], "the smoke's shared grey");
            assert!(smoke.col[0] < 0.3, "the r*0.3 scale the draw speaks");
            assert_eq!(flame.col[0], 1.0, "the flame's white the ctor speaks");
            assert_eq!(flame.col[1], 1.0, "the flame's white the ctor speaks");
            assert_eq!(flame.col[2], 1.0, "the flame's white the ctor speaks");
        }
        // THE DETERMINISM (the same walk replayed — the fixed-seed
        // stream the replay speaks, the count the loops alone speak)
        app.particles.clear();
        app.play_mob_spawn(4, 5, 6);
        assert_eq!(app.particles.len(), 40, "the replay the loop answers");
        // THE DATA WORD THE WALK NEVER READS (the arm the anchor alone
        // carries — the serve the mailbox's own verbatim keeps; the
        // data the wire spoke never reaches the walk)
        app.particles.clear();
        app.live_mob_spawn_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    #[test]
    fn the_wire_splash_word_the_three_walks_wear() {
        // THE SPLASH-POTION EFFECT'S OWN WIRE LANE (javap case 2002 →
        // the shared arm offset 1199 over the 26.2 jar): the eight
        // shards the `bipush 8` loop fires at the bottom-center anchor,
        // the hundred ring puffs the `bipush 100` loop answers with the
        // color the data word carries, the glass voice the
        // playLocalSound tail speaks — the three walks the one event
        // wears, the stream the 117-draw order keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE REGULAR SPLASH (data 0xFF0000: the red channel the >>16
        // band reads FULL, the green and the blue the decode keeps
        // dark — the color the ring's own tint carries)
        app.live_splash_events_updated(vec![crate::world_view::WireSplashEvent {
            x: 10,
            y: 20,
            z: 10,
            data: 0xFF_0000,
            instant: false,
        }]);
        assert_eq!(
            app.particles.len(),
            108,
            "the eight shards + the hundred puffs the two loops speak"
        );
        // THE SHARDS (the first eight — the anchor the bottom-center
        // walk keeps EXACTLY, the gravity 1.0 the shard ctor passes,
        // the base 0.98 friction, the layer the sprite pick's two rows
        // keep, the white tint the item pixels wear)
        for p in &app.particles[..8] {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::PotionItem { .. }),
                "the shard kind the item option answers"
            );
            assert!(
                (p.x - 10.5).abs() < 1e-9 && (p.y - 20.0).abs() < 1e-9 && (p.z - 10.5).abs() < 1e-9,
                "the bottom-center anchor the atBottomCenterOf walk keeps"
            );
            assert!(
                (p.gravity - 1.0).abs() < 1e-9,
                "the gravity 1.0 the shard ctor passes"
            );
            assert!(
                (p.friction - 0.98).abs() < 1e-9,
                "the base friction the shard keeps"
            );
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::PotionItem { layer } if layer < 2),
                "the layer pick the two-layer model's own band keeps"
            );
        }
        // THE RING (the last hundred — the Spell kind the option's own
        // type answers, the +0.3 lift the anchor's own fold gives, the
        // 0.1-scaled radial offset the d2/d4 place, the color the
        // f-scale fold carries)
        for p in &app.particles[8..] {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Spell { instant: false }),
                "the regular splash the Effect type keeps"
            );
            assert!(
                (p.y - (20.0 + 0.3)).abs() < 1e-9,
                "the +0.3 lift the ring's own fold gives"
            );
            assert!(
                (p.x - 10.5).abs() <= 0.4 + 1e-9 && (p.z - 10.5).abs() <= 0.4 + 1e-9,
                "the radial offset the d2*0.1/d4*0.1 band keeps"
            );
            assert!(
                (p.gravity + 0.1).abs() < 1e-9,
                "the −0.1 gravity the spell ctor passes"
            );
            assert!(
                (p.friction - 0.96).abs() < 1e-9,
                "the 0.96 friction the spell ctor sets"
            );
            // THE COLOR (the red channel the full band the decode
            // reads — the f ∈ [0.75, 1.0) scale the loop's own draw
            // gives, the green and the blue the dark decode keeps)
            assert!(p.col[0] >= 0.75 && p.col[0] < 1.0, "the red the f-scale band keeps: {}", p.col[0]);
            assert_eq!(p.col[1], 0.0, "the dark green the decode keeps");
            assert_eq!(p.col[2], 0.0, "the dark blue the decode keeps");
        }
        // THE GLASS VOICE (the tail's playLocalSound — the pitch the
        // 0.9..1.0 band the draw gives)
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.splash_potion.break @"))
                .expect("the glass voice the tail speaks");
            let pitch: f32 = voice
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                pitch >= 0.9 && pitch < 1.0,
                "the pitch band the nextFloat * 0.1 + 0.9 fold gives: {pitch}"
            );
        }
        // THE INSTANT WORD (the 2007 walk the same arm's branch
        // answers — the Spell kind's own flag splits the sprite set)
        app.particles.clear();
        app.live_splash_events_updated(vec![crate::world_view::WireSplashEvent {
            x: 4,
            y: 5,
            z: 6,
            data: 0x00FF00,
            instant: true,
        }]);
        assert_eq!(app.particles.len(), 108, "the same walk the instant word wears");
        for p in &app.particles[8..] {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Spell { instant: true }),
                "the instant flag the type split banks"
            );
            // the GREEN word (the green channel the >>8 band reads
            // full — the f-scale band the same fold gives)
            assert!(p.col[1] >= 0.75 && p.col[1] < 1.0, "the green the decode keeps");
            assert_eq!(p.col[0], 0.0, "the dark red the decode keeps");
        }
        // THE DETERMINISM (the same walk replayed — the fixed-seed
        // stream the replay speaks, the shards' anchor the EXACT fold
        // keeps, the count the loops alone speak)
        app.particles.clear();
        app.play_splash(4, 5, 6, 0x00FF00, true);
        assert_eq!(app.particles.len(), 108, "the replay the loops answer");
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.particles.clear();
        app.live_splash_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    /// THE SHOVEL'S OWN FLATTEN ARM — the straight-down aim the vanilla
    /// useOn's SUCCESS speaks (javap ShovelItem.useOn over the real 26.2
    /// jar): the held shovel the surface grass looks up (the
    /// FLATTENABLES map's own GRASS_BLOCK row), the inline above-air
    /// gate the standing cell's air answers, the DIRT_PATH lands IN the
    /// clicked cell (the setBlock(pos, DIRT_PATH, 11) — the useOn's own
    /// set), the mesh re-speaks, the SHOVEL_FLATTEN voice the log rides,
    /// the press consumed.
    #[test]
    fn the_shovel_flattens_the_grass_the_path_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        // the hit cell the GRASS row keeps (the beach's sand never
        // speaks — the test pins the flattenable kind itself)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::GRASS);
        app.offline_held_updated(Some("item/wooden_shovel"));
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        let quads_before = app.world_quads.as_ref().unwrap().clone();
        assert!(app.on_right_press(0, 0), "the flatten consumed the press");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface, c),
            crate::world::DIRT_PATH,
            "the grass cell wears the dirt path"
        );
        assert_ne!(
            app.world_quads.as_ref().unwrap(),
            &quads_before,
            "the mesh re-spoke with the flattened voxel"
        );
        // THE FLATTEN'S VOICE (the assert computes the expected from the
        // ACTUAL loaded state — the cache-absent boot the silent lane keeps)
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.shovel.flatten @")),
                "the SHOVEL_FLATTEN voice the useOn's own order rode"
            );
        }
    }


    /// THE BOW'S OWN LANE (javap `BowItem.use` + `releaseUsing` the
    /// 26.2 bytecode speaks): the EMPTY quiver the use gate FAILS
    /// (the draw never starts), the loaded quiver the draw arms, the
    /// full draw the 20-tick clock speaks and the RELEASE the shot
    /// answers — the arrow the eye-minus-0.1 spawn rides, the -z aim
    /// the 3.0 velocity keeps, the CRIT the full-draw power arms, the
    /// stack the survival draw shrinks — and the FLIGHT the mob hit
    /// answers (the hurt the arrow's own damage speaks, the discard
    /// the hit removal keeps).
    #[test]
    fn the_bow_lane_the_use_gate_the_release_shot_speaks() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let bow_id = wire("bow");
        let arrow_id = wire("arrow");
        let slot = app.selected_hotbar;
        let quiver = (slot + 1) % 9;
        // the spawn band's air (the terrain's own tops never reach),
        // the level -z look the camera's own forward keeps
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let eye_y = app.spawn_y + 7;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: 0.0,
        });
        // THE EMPTY QUIVER: the use gate the FAIL speaks (the draw
        // never starts, the press the false verdict keeps)
        app.inventory.items[slot] = (bow_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(
            !app.on_right_press(0, 0),
            "the empty quiver the FAIL keeps"
        );
        assert!(app.using_item.is_none(), "the draw the gate refuses");
        // THE QUIVER RESTORED: the draw arms (the use-item the bow
        // speaks)
        app.inventory.items[quiver] = (arrow_id, 3);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(app.on_right_press(0, 0), "the draw the arrow speaks");
        assert!(app.using_item.is_some(), "the draw starts");
        assert_eq!(
            app.using_item.as_ref().unwrap().item,
            bow_id,
            "the draw the held bow carries"
        );
        // THE FULL DRAW: the use clock the 20 ticks the cap speaks
        for _ in 0..21 {
            app.update();
        }
        assert!(
            app.using_item.as_ref().unwrap().ticks >= 20,
            "the draw clock the update advances"
        );
        // THE RELEASE: the shot the full power speaks
        app.on_right_release();
        assert!(app.using_item.is_none(), "the release the use ends");
        assert_eq!(app.arrows.len(), 1, "the shot the arrow speaks");
        let a = &app.arrows[0];
        assert!(a.player_shot, "the owner lane the player speaks");
        assert!(a.crit, "the full draw the crit arms");
        // the spawn position the eye-minus-0.1 walk keeps (the
        // camera's own pos the headless fallback answers)
        assert!(
            (a.x - (c as f64 + 0.5)).abs() < 1e-6
                && (a.y - (eye_y as f64 - 0.1)).abs() < 1e-6
                && (a.z - (c as f64 + 0.5)).abs() < 1e-6,
            "the spawn the eye walk keeps ({},{},{})",
            a.x,
            a.y,
            a.z
        );
        // the velocity the -z aim the 3.0 scale keeps (the spread
        // window the ±0.0172275 × 3.0 speaks)
        assert!(
            a.vz < -2.948 && a.vz > -3.052,
            "the -z the aim speaks ({})",
            a.vz
        );
        assert!(
            a.vx.abs() < 0.052 && a.vy.abs() < 0.052,
            "the spread the window keeps ({},{})",
            a.vx,
            a.vy
        );
        // THE CONSUME: the stack the survival draw shrinks (the 3 the
        // 2 speaks, the HUD row the every-mutation law syncs)
        assert_eq!(
            app.inventory.items[quiver],
            (arrow_id, 2),
            "the survival draw the stack shrinks"
        );
        // THE FLIGHT + THE MOB HIT (the full loop the vanilla speaks):
        // the cow 2.5 blocks out the path carries — the next update
        // the hurt speaks, the discard the removal answers
        let mut cow = crate::mob::Zombie::new(101, c as f64 + 0.5, (eye_y - 1) as f64, c as f64 - 2.0, 21);
        cow.kind = crate::mob::MobKind::Cow;
        let pre = cow.health;
        app.mobs.push(cow);
        app.update();
        assert!(
            app.arrows.is_empty(),
            "the hit the discard answers (the miss the flight keeps)"
        );
        let hit = app.mobs.iter().find(|m| m.id == 101).unwrap();
        assert!(
            hit.health < pre,
            "the hurt the arrow's own damage speaks ({} -> {})",
            pre,
            hit.health
        );
        // THE EMPTY QUIVER AGAIN: the spent stack the gate refuses
        // (the shot the next draw never arms)
        app.inventory.items[quiver] = (0, 0);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(!app.on_right_press(0, 0), "the spent quiver the FAIL keeps");
    }


    /// THE OFFHAND BOW'S OWN OFFLINE ARC — the release walk the HAND
    /// never reads (the active stack's own item the gate resolves —
    /// `held_word_of(active.item)` the OFF bow's own id answers — the
    /// mirror the main lane's walk already wears, the candidate the
    /// round PINS): the off-hand bow the empty main hand backs, the
    /// draw the LEFT pulling layers re-bakes (the FIRST_PERSON_LEFT_
    /// HAND ctx the payload wears), the full draw the release answers
    /// — the arrow the eye-minus-0.1 spawn rides, the -z aim the 3.0
    /// velocity keeps, the CRIT the full-draw power arms, the stack
    /// the survival draw shrinks, the MAIN lane the OFF shot never
    /// leaks, the LEFT render the standby restores — and the SPENT
    /// quiver the gate refuses (the OFF turn's own FAIL the press's
    /// silence keeps).
    #[test]
    fn the_offhand_bow_the_offline_shot_the_release_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let bow_id = wire("bow");
        let arrow_id = wire("arrow");
        let slot = app.selected_hotbar;
        let quiver = (slot + 1) % 9;
        // the spawn band's air (the terrain's own tops never reach),
        // the level -z look the camera's own forward keeps
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let eye_y = app.spawn_y + 7;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: 0.0,
        });
        // THE OFFHAND BOW the offline slot wears, the MAIN hand the
        // empty inventory speaks (the MAIN turn's PASS the OFF turn
        // answers)
        app.inventory.items[slot] = (0, 0);
        app.inventory.offhand[0] = (bow_id, 1);
        app.offhand_held_refreshed();
        app.inventory.items[quiver] = (arrow_id, 3);
        app.hotbar_slots = app.inventory.hotbar();
        // THE DRAW: the OFF turn's own pull the LEFT layers re-bakes
        assert!(
            app.on_right_press(0, 0),
            "the OFF draw the arrow speaks"
        );
        assert!(
            matches!(
                app.using_item.as_ref().map(|u| u.hand),
                Some(UseHand::Off)
            ),
            "the OFF hand the draw wears"
        );
        let pulling = app.held_offhand.as_ref().expect("the LEFT pulling loads");
        assert!(
            pulling.binds_texture("item/bow_pulling_0"),
            "the LEFT ctx the using-branch layers wear"
        );
        // THE FULL DRAW: the use clock the 20 ticks the cap speaks
        for _ in 0..21 {
            app.update();
        }
        assert!(
            app.using_item.as_ref().unwrap().ticks >= 20,
            "the draw clock the update advances"
        );
        // THE RELEASE: the shot the full power speaks (the walk the
        // hand never reads — the active stack's own item the gate
        // resolves)
        app.on_right_release();
        assert!(app.using_item.is_none(), "the release the use ends");
        assert_eq!(app.arrows.len(), 1, "the OFF shot the arrow speaks");
        let a = &app.arrows[0];
        assert!(a.player_shot, "the owner lane the player speaks");
        assert!(a.crit, "the full draw the crit arms");
        assert!(
            (a.x - (c as f64 + 0.5)).abs() < 1e-6
                && (a.y - (eye_y as f64 - 0.1)).abs() < 1e-6
                && (a.z - (c as f64 + 0.5)).abs() < 1e-6,
            "the spawn the eye walk keeps ({},{},{})",
            a.x,
            a.y,
            a.z
        );
        assert!(
            a.vz < -2.948 && a.vz > -3.052,
            "the -z the aim speaks ({})",
            a.vz
        );
        // THE CONSUME: the stack the survival draw shrinks
        assert_eq!(
            app.inventory.items[quiver],
            (arrow_id, 2),
            "the survival draw the stack shrinks"
        );
        // THE VOICE (the level's own random the pitch draw rides)
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("entity.arrow.shoot @")),
                "the ARROW_SHOOT voice the release rode"
            );
        }
        // THE RESTORE: the LEFT render the plain bow wears, the MAIN
        // lane the OFF arc the leak never speaks
        let plain = app.held_offhand.as_ref().expect("the LEFT plain returns");
        assert!(
            plain.binds_texture("item/bow"),
            "the resting bow returns through the LEFT ctx (the bow the standby model never keeps)"
        );
        assert!(
            app.held_charge.is_none(),
            "the OFF shot the MAIN lane never leaks"
        );
        // THE SPENT QUIVER: the OFF turn's own gate the FAIL keeps
        app.inventory.items[quiver] = (0, 0);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(
            !app.on_right_press(0, 0),
            "the spent quiver the OFF FAIL keeps"
        );
        assert!(app.using_item.is_none(), "the gate the window never opens");
    }


    /// THE OFFHAND BOW'S OWN EMPTY-QUIVER GATE (javap `BowItem.use` the
    /// OFF turn's own arc the offline lane keeps — the autopush-488
    /// gate the live lane wore, the offline source the same law reads):
    /// the arrow-less offhand the draw never starts, the press the FAIL
    /// verdict keeps, the LEFT render the resting bow the pull never
    /// re-bakes, and the window the silence keeps (the using_item the
    /// gate never opens).
    #[test]
    fn the_offhand_bow_the_offline_empty_quiver_the_gate_keeps() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let bow_id = wire("bow");
        let slot = app.selected_hotbar;
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let eye_y = app.spawn_y + 7;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: 0.0,
        });
        // THE OFFHAND BOW the arrow-less lane wears (the MAIN hand the
        // empty inventory speaks)
        app.inventory.items[slot] = (0, 0);
        app.inventory.offhand[0] = (bow_id, 1);
        app.offhand_held_refreshed();
        let resting = app.held_offhand.as_ref().expect("the LEFT resting bow");
        assert!(
            resting.binds_texture("item/bow"),
            "the resting bow the LEFT ctx wears (the bow the standby model never keeps)"
        );
        // THE EMPTY QUIVER: the OFF turn's own gate the FAIL speaks
        assert!(
            !app.on_right_press(0, 0),
            "the empty quiver the OFF FAIL keeps"
        );
        assert!(
            app.using_item.is_none(),
            "the gate the window never opens"
        );
        let still = app.held_offhand.as_ref().expect("the LEFT render stands");
        assert!(
            still.binds_texture("item/bow"),
            "the pull the render never re-bakes"
        );
    }


    /// THE FLATTEN'S OWN DOWN-FACE REFUSAL — the useOn's
    /// `getClickedFace() == DOWN` the second PASS return speaks (the
    /// clicked underside never flattens): the camera under the grass
    /// ceiling looks UP, the hit face the block's DOWN face keeps, the
    /// gate the PASS speaks — the path never lands. THE HOE THE SAME
    /// CEILING TILLS (the HoeItem's useOn the face never reads — the
    /// vanilla's own asymmetry the two PASS/one PASS shapes speak): the
    /// farmland the underside wears, the pair the asymmetry pins.
    #[test]
    fn the_flatten_the_down_face_refuses() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        // THE CEILING: the grass block the standing cell's two-above
        // keeps (the ray the open column walks — the tree the center
        // never spoke per the placement lane's own aim)
        let ceiling = app.spawn_y + 2;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, ceiling, c, crate::world::GRASS);
        app.offline_held_updated(Some("item/wooden_shovel"));
        // THE UP AIM: the eye the standing cell's own floor rides, the
        // pitch +90 the forward's straight-up speaks — the hit voxel
        // the ceiling cell, the DOWN face the entry keeps
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 0.5, c as f32 + 0.5),
            yaw: 0.0,
            pitch: std::f32::consts::FRAC_PI_2,
        });
        assert!(
            !app.on_right_press(0, 0),
            "the DOWN face the flatten never consumes"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, ceiling, c),
            crate::world::GRASS,
            "the ceiling stays grass"
        );
        // THE HOE'S OWN FACE-LESS WALK: the same underside the till
        // never refuses (the above cell the strict air answers) — the
        // farmland the asymmetry's own verdict wears
        app.offline_held_updated(Some("item/wooden_hoe"));
        assert!(app.on_right_press(0, 0), "the hoe the underside tills");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, ceiling, c),
            crate::world::FARMLAND,
            "the ceiling wears the farmland"
        );
    }


    /// THE FLATTEN'S OWN DIRT ROW + MAP MISS — the FLATTENABLES
    /// lookup's rows (javap: the builder's GRASS_BLOCK/DIRT rows the
    /// vocabulary speaks, every other block the PASS keeps): the DIRT
    /// row the path wears, the stone the map miss the PASS keeps.
    #[test]
    fn the_flatten_the_dirt_row_the_map_miss_keeps() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::STONE);
        app.offline_held_updated(Some("item/wooden_shovel"));
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(
            !app.on_right_press(0, 0),
            "the stone the map miss the PASS keeps"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface, c),
            crate::world::STONE,
            "the stone stays stone"
        );
        // THE DIRT ROW: the second flattenable the path wears
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::DIRT);
        app.offline_held_updated(Some("item/iron_shovel"));
        assert!(app.on_right_press(0, 0), "the dirt row the SUCCESS speaks");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface, c),
            crate::world::DIRT_PATH,
            "the dirt cell wears the dirt path"
        );
    }


    /// THE BLOCK-USE LANE's NETWORKED ARC — the clip runs over the net
    /// world, the UseItemOn echo rides the command channel (the
    /// worker's serverbound packet), and the LOCAL voxels stay
    /// untouched (the server's own chunk refresh is the authority on
    /// this lane).
    #[test]
    fn the_block_use_lane_echoes_the_networked_world() {
        use crate::world_view::ContainerCommand;
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        // the net world with a stone floor under the spawn column
        let mut net = air_net_world(2, 0);
        let c = net.size_blocks() / 2;
        let floor_y = net.size_blocks() / 4;
        for x in 0..net.size_blocks() {
            for z in 0..net.size_blocks() {
                net.set(x, floor_y, z, crate::world::STONE);
            }
        }
        app.net_world = Some(net);
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.live_held_updated(Some((wire("dirt"), 1)));
        // the camera: the eye 3.0 above the floor plane, straight down
        app.net_camera = Some(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, floor_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the block lane answers the press");
        match click_rx.try_recv().expect("the useItemOn echo rode the lane") {
            ContainerCommand::UseItemOn {
                hand: 0,
                pos,
                face,
                cursor,
                inside,
            } => {
                // the `hand: 0` pattern the MAIN turn's own ctx asserts
                assert_eq!(pos, (c, floor_y, c), "the hit voxel");
                assert_eq!(face, 1, "UP — the floor's entry face");
                // the cursor rides the float cast's own tail (the
                // vanilla's double math + f32 wire cast speaks the same
                // epsilon) — the near-exact face-centered offsets
                assert!((cursor.0 - 0.5).abs() < 1e-3, "cursor x {cursor:?}");
                assert!((cursor.1 - 1.0).abs() < 1e-3, "cursor y {cursor:?}");
                assert!((cursor.2 - 0.5).abs() < 1e-3, "cursor z {cursor:?}");
                assert!(!inside);
            }
            other => panic!("expected a UseItemOn echo, got {other:?}"),
        }
        // the local voxels are untouched — the server's refresh owns the
        // networked terrain
        assert_eq!(
            app.net_world.as_ref().unwrap().get(c, floor_y + 1, c),
            crate::world::AIR,
            "no local prediction on the networked lane"
        );
    }


    /// THE BOAT'S OWN USE WALK — the OFFLINE arc (javap BoatItem.use):
    /// the Fluid-ANY POV clip finds the pool, the eye gate the clean
    /// water keeps silent, the noCollision fold the open water speaks,
    /// and the local hull lands at the surface hit (the singleplayer's
    /// own addFreshEntity): the position the hit location verbatim, the
    /// yaw the player's own, the shape bits the item name's family
    /// parse keeps, the survival consume the stack rides down, the
    /// swing the SUCCESS verdict arms.
    #[test]
    fn the_boat_use_walk_spawns_the_local_hull_offline() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        // the held stack: the oak boat's five-count the hotbar's own
        // selection carries (the consume's own lane)
        let boat_id = wire("oak_boat");
        app.inventory.items[app.selected_hotbar] = (boat_id, 5);
        app.hotbar_slots = app.inventory.hotbar();
        // the pool: a water cell floating in the spawn column's air
        // band — every cell the spawned box spans stays air (the
        // terrain's own tops never reach this high)
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let pool_y = app.spawn_y + 6;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, pool_y, c, crate::world::WATER);
        // the eye 3.0 above the water's top boundary (pool_y + 1), the
        // straight-down aim, the yaw the vanilla fold keeps honest
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, (pool_y + 4) as f32, c as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the boat use consumed the press");
        // THE LOCAL HULL: one offline boat at the surface hit, the
        // player's yaw (the camera's yaw - PI in the vanilla degrees —
        // 0.0 here), the oak shape bits the family parse keeps, the
        // rest words the never-ridden hull's own defaults
        assert_eq!(app.offline_boats.len(), 1, "the local hull spawned");
        let hull = &app.offline_boats[0];
        // the surface hit's own words: the y the exact boundary plane,
        // the x/z the honest ray's own drift (the camera forward's
        // f32 yaw the pitch fold carries — the vanilla's own ray)
        assert_eq!(hull.y, (pool_y + 1) as f64, "the surface plane exactly");
        assert!(
            (hull.x - (c as f64 + 0.5)).abs() < 1e-3,
            "hull x {}",
            hull.x
        );
        assert!(
            (hull.z - (c as f64 + 0.5)).abs() < 1e-3,
            "hull z {}",
            hull.z
        );
        assert_eq!(hull.id, 1, "the offline spawner's own counter");
        assert_eq!(hull.yaw, 0.0, "the camera yaw - PI in the vanilla degrees");
        assert!(!hull.raft && !hull.chest, "the plain oak boat");
        assert_eq!(hull.wood, "oak");
        assert!(!hull.paddle_left && !hull.paddle_right, "the never-ridden rest");
        assert_eq!(
            (hull.bubble_time, hull.hurt_time, hull.hurt_dir, hull.damage),
            (0, 0, 1, 0.0),
            "the define's own defaults the absent words keep"
        );
        // THE SURVIVAL CONSUME: the stack rode down one
        assert_eq!(app.inventory.items[app.selected_hotbar], (boat_id, 4));
        // THE SWING: the SUCCESS verdict's own arm
        assert!(app.swing.swinging, "the swing started");
    }


    /// THE BOAT'S OWN USE WALK — the EYE-INSIDE GATE (the
    /// getEntities/CAN_BE_PICKED walk's own refusal) BEHIND the entity
    /// lane's own start-inside law: the eye inside a HULL's box the
    /// ENTITY PICK's d²=0 word speaks (the ProjectileUtil walk's own
    /// start-inside arm) — the interact lane OUTRANKS the item lane
    /// (the vanilla startUseItem's own else-if), the wire hull the
    /// interact send consumes, the local prediction the wire body
    /// never runs. The MOB the same pick picks (the
    /// LivingEntity.isPickable law's own family) — the mob arm's Pass
    /// verdict the ITEM lane falls through to, and THERE the eye gate
    /// speaks: the eye inside any pickable body's box the PASS keeps,
    /// the water stays hull-less.
    /// THE MINECART'S OWN USE-ON WALK — the OFFLINE arc (javap
    /// MinecartItem.useOn): the rail cell the RAILS tag consult keeps,
    /// the spawn Vec3 the (x+0.5, y+0.0625, z+0.5) fold wears, the
    /// adjustToRails snap the createMinecart factory rides (the flat
    /// rail's own center law — the y the CELL's own y re-answers), the
    /// survival consume the stack rides down, the swing the SUCCESS
    /// verdict arms.
    #[test]
    fn the_minecart_use_walk_spawns_the_snapped_cart_offline() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        // the held stack: the plain minecart's five-count the hotbar's
        // own selection carries (the consume's own lane)
        let cart_id = wire("minecart");
        app.inventory.items[app.selected_hotbar] = (cart_id, 5);
        app.hotbar_slots = app.inventory.hotbar();
        // the flat EAST_WEST rail floating in the spawn column's air
        // band — every cell the spawned walk reads stays air
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1); // EAST_WEST
        // THE USE-ON: the clicked rail cell the walk reads (the block
        // lane's own clip hands the hit the arm serves)
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert!(
            app.on_right_press_minecart_arm(&hit) == Some(true),
            "the useOn consumed the press"
        );
        // THE LOCAL CART: one offline cart, the id the counter keeps,
        // the kind the item name speaks, the snap the flat rail's own
        // center law banks (x the carried coord, y the CELL's own y,
        // z the center), the yaw the first (west) exit's 180.0 base
        // stamps, the pitch the flat rail zeroes
        assert_eq!(app.offline_carts.len(), 1, "the local cart spawned");
        let entry = &app.offline_carts[0];
        assert_eq!(entry.id, 1, "the offline spawner's own counter");
        assert_eq!(entry.kind, "minecart");
        let cart = &entry.cart;
        assert!((cart.pos[0] - (c as f64 + 0.5)).abs() < 1e-9, "x carried");
        assert!((cart.pos[1] - ry as f64).abs() < 1e-9, "y the cell's own");
        assert!((cart.pos[2] - (c as f64 + 0.5)).abs() < 1e-9, "z centered");
        // the first (west) exit's atan2 the 180.0 base stamps:
        // atan2(0, −0.5) = π → 180 − 180 = 0.0
        assert!((cart.yaw_deg - 0.0).abs() < 1e-4);
        assert!((cart.pitch_deg - 0.0).abs() < 1e-4);
        assert!(cart.on_rails);
        // THE SURVIVAL CONSUME: the stack rode down one
        assert_eq!(app.inventory.items[app.selected_hotbar], (cart_id, 4));
        // THE SWING: the SUCCESS verdict's own arm
        assert!(app.swing.swinging, "the swing started");
    }


    /// THE MINECART'S OWN USE-ON WALK — the SLOPE arc: the yOffset the
    /// isSlope read lifts (the 0.5 the ascending rails wear), the snap
    /// the dist + 0.1 the y gain keeps, the lean the −45.0 the plain
    /// cart's own descent pitch banks.
    #[test]
    fn the_minecart_use_walk_the_slope_spawns_the_lifted_lean() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        app.inventory.items[app.selected_hotbar] = (wire("minecart"), 3);
        app.hotbar_slots = app.inventory.hotbar();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 2); // ASCENDING_EAST (the east high)
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        let cart = &app.offline_carts[0].cart;
        // THE SNAP'S OWN WORDS: the x/z the straight arm's center law
        // keeps, the y the dist (0.5) + 0.1 lift wears, the lean the
        // −45.0 speaks
        assert!((cart.pos[0] - (c as f64 + 0.5)).abs() < 1e-9);
        assert!((cart.pos[1] - (ry as f64 + 0.6)).abs() < 1e-9);
        assert!((cart.pos[2] - (c as f64 + 0.5)).abs() < 1e-9);
        assert!((cart.pitch_deg - (-45.0)).abs() < 1e-4);
    }


    /// THE MINECART'S OWN OVERLAP GATE — the FAIL the press never
    /// consumes: the spawned box the existing cart's own dims keep,
    /// the intersection verdict the second useOn speaks, the stack
    /// the FAIL never shrinks, the list the second cart never gains.
    #[test]
    fn the_minecart_use_walk_the_overlap_fail_never_consumes() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let cart_id = wire("minecart");
        app.inventory.items[app.selected_hotbar] = (cart_id, 5);
        app.hotbar_slots = app.inventory.hotbar();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1);
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        app.swing.swinging = false; // the first SUCCESS's arm the baseline clears
        // THE SECOND PRESS: the overlap the FAIL speaks — no cart, no
        // consume, no swing
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(false));
        assert_eq!(app.offline_carts.len(), 1, "the second cart never spawns");
        assert_eq!(app.inventory.items[app.selected_hotbar], (cart_id, 4));
        assert!(!app.swing.swinging, "the FAIL the swing never arms");
    }


    /// THE MINECART'S OWN GATES — the non-rail click the None keeps
    /// (the FAIL verdict the chain's fall-through never distinguishes
    /// from the PASS — the placement lane's own walk follows), and the
    /// WIRE SESSION the server owns (the client predicts NOTHING: no
    /// local cart, the echo the block lane's own head already spoke,
    /// the swing the client's own SUCCESS verdict keeps).
    #[test]
    fn the_minecart_use_walk_the_gates_speak() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/minecart"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        // THE NON-RAIL CELL: the dirt the RAILS tag consult refuses
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::DIRT);
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), None);
        assert!(app.offline_carts.is_empty());
        // THE WIRE SESSION: the net world's own rail (the same
        // WORLD_CHUNKS span the offline world's own coords keep), the
        // local spawn the server's Add Entity owns
        let mut net = crate::world::World::new(crate::screens::WORLD_CHUNKS, 0xCA2Cu64);
        net.set(c, ry, c, crate::world::RAIL);
        net.set_variant(c, ry, c, 1);
        app.net_world = Some(net);
        app.world_is_network = true;
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        assert!(app.offline_carts.is_empty(), "the wire lane never spawns");
        assert!(app.swing.swinging, "the client's own SUCCESS arm");
    }


    /// THE DETECTOR RAIL'S OWN CART PASS (the autopush-115 round —
    /// javap DetectorRailBlock the 26.2 jar: the entityInside pc 0-32
    /// the cart's own cell the UNARMED consult serves, the
    /// checkPressed pc 57-123 arm the flag-3 setBlock the POWERED bit
    /// speaks [the updatePowerToConnected the connected rails'
    /// neighborChanged walk serves, the updateNeighborsAt pair the
    /// broadcast speaks], the pc 191-203 tail the scheduleTick(pos,
    /// this, 20) the held beat banks, the pc 124-190 release the
    /// vacated beat the bit drops — the wire the armed 15 the
    /// component's own re-read wears [the ownSignal the dir never
    /// reads — the six arms the no-facing gate keeps], the dead pulse
    /// the same walk re-answers). THE FULL CYCLE the offline cart
    /// walk's own beat serves: the use-on spawn the snap law banks,
    /// the pass the arm the bit + the wire + the queue bank, the held
    /// beat the re-arm the ONE live entry the dedup keeps, the
    /// boom'd/vacated cart the release the bit + the wire + the queue
    /// re-answers.
    #[test]
    fn the_detector_rail_the_cart_pass_the_bit_and_the_wire_wear() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let cart_id = wire("minecart");
        app.inventory.items[app.selected_hotbar] = (cart_id, 5);
        app.hotbar_slots = app.inventory.hotbar();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        {
            let w = app.offline_world.as_mut().unwrap();
            // THE DETECTOR'S OWN CELL (the E-W look-stamp the run wears)
            w.set(c, ry, c, crate::world::DETECTOR_RAIL);
            w.set_variant(c, ry, c, 1);
            // THE WIRE'S OWN SEAT (the east neighbor the solid support
            // the placement law keeps — the rung the armed detector's
            // own 15 the component's flood banks)
            w.set(c, ry - 1, c + 1, crate::world::STONE);
            w.set(c, ry, c + 1, crate::world::REDSTONE_WIRE);
        }
        // THE USE-ON: the clicked detector cell the walk reads — the
        // cart the snap law banks (the same seam the use-walk tests wear)
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(
            app.on_right_press_minecart_arm(&hit),
            Some(true),
            "the useOn consumed the press"
        );
        assert_eq!(app.offline_carts.len(), 1, "the local cart spawned");
        // THE UNARMED REST (the bit the beat never banks before the walk)
        assert_eq!(
            app.offline_world.as_ref().unwrap().variant(c, ry, c)
                & crate::world::RAIL_POWERED_BIT,
            0,
            "the pre-tick rest the bit keeps"
        );
        // THE CART'S OWN PASS — the tick walk the entityInside consult
        // serves: the flag-3 setBlock the bit arms, the wire's own
        // re-read the ON kind wears, the schedule the held beat banks
        app.tick_offline_carts();
        {
            let w = app.offline_world.as_ref().unwrap();
            assert_eq!(
                w.variant(c, ry, c) & crate::world::RAIL_POWERED_BIT,
                crate::world::RAIL_POWERED_BIT,
                "the cart's pass the bit arms"
            );
            assert_eq!(
                w.get(c, ry, c + 1),
                crate::world::REDSTONE_WIRE_ON,
                "the wire the armed 15 wears"
            );
            assert_eq!(
                w.detector_ticks.len(),
                1,
                "the held beat the queue banks"
            );
        }
        // THE HELD BEAT'S OWN RE-ARM (the cart-present verdict the keep
        // the schedule banks — the ONE live entry the dedup keeps, the
        // bit the re-check the same verdict holds)
        app.frame += crate::world::DETECTOR_PRESSED_CHECK_PERIOD;
        app.tick_offline_carts();
        {
            let w = app.offline_world.as_ref().unwrap();
            assert_eq!(
                w.variant(c, ry, c) & crate::world::RAIL_POWERED_BIT,
                crate::world::RAIL_POWERED_BIT,
                "the held beat the bit keeps"
            );
            assert_eq!(w.detector_ticks.len(), 1, "the re-arm the dedup keeps");
        }
        // THE RELEASE (the vacated beat — the cart the boom'd lane
        // removes, the 20-tick beat the !flag verdict serves: the bit
        // drops, the wire the dead pulse re-answers, the queue retires)
        app.offline_carts.clear();
        app.frame += crate::world::DETECTOR_PRESSED_CHECK_PERIOD;
        app.tick_offline_carts();
        {
            let w = app.offline_world.as_ref().unwrap();
            assert_eq!(
                w.variant(c, ry, c) & crate::world::RAIL_POWERED_BIT,
                0,
                "the vacated beat the bit drops"
            );
            assert_eq!(
                w.get(c, ry, c + 1),
                crate::world::REDSTONE_WIRE,
                "the wire the dead pulse keeps"
            );
            assert!(
                w.detector_ticks.is_empty(),
                "the release beat the queue retires"
            );
        }
    }


    /// THE CARTS' OWN OFFLINE TICK — the walk the App's 20-TPS beat
    /// rides: the spawned slope cart the gravity push wears (the
    /// ASCENDING_EAST's low side speaks — the west the delta gains),
    /// the position the rail line's own walk moves.
    #[test]
    fn the_offline_carts_the_app_tick_rolls_downhill() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/minecart"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        // the slope: the rail pair the descent's own line keeps (the
        // west cell the low side the push wears, the east cell the
        // high side the pair completes)
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 2); // ASCENDING_EAST
        app.offline_world.as_mut().unwrap().set(c - 1, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c - 1, ry, c, 1); // EAST_WEST continues west
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        let x0 = app.offline_carts[0].cart.pos[0];
        // THREE TICKS the App's own walk speaks — the push the slope
        // wears, the step the west line moves
        for _ in 0..3 {
            app.tick_offline_carts();
        }
        assert!(
            app.offline_carts[0].cart.pos[0] < x0,
            "the cart rolled west (downhill): {} -> {}",
            x0,
            app.offline_carts[0].cart.pos[0]
        );
    }


    /// THE FRESH SESSION'S OWN HONESTY — the previous offline
    /// session's carts drop with the world that carried them (the
    /// enter_world walk the boats' clear law keeps).
    #[test]
    fn the_offline_carts_the_fresh_world_clears() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/minecart"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1);
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        assert_eq!(app.offline_carts.len(), 1);
        // the REBUILD path the fresh world's own honesty wears (the
        // world_quads=None the rebuild branch forces — the same walk
        // the boats' clear law keeps)
        app.world_quads = None;
        app.enter_world();
        assert!(app.offline_carts.is_empty(), "the fresh world cleared");
    }


    /// THE MINECART RENDER LANE (the AbstractMinecartRenderer.submit
    /// walk through the frame): the spawned cart the east-facing eye
    /// watches, the lazy slot the sha1-kept model fills, the frame the
    /// cart's quads recolor against the cart-free twin scene.
    #[test]
    fn the_offline_carts_the_render_lane_draws_the_model() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/minecart"));
        let w0 = app.offline_world.as_ref().unwrap();
        let c = w0.size_blocks() / 2;
        let sy = app.spawn_y;
        // THE VIEW CORRIDOR: the eastward column scan the zombie lane's
        // own determinism speaks — the first flat surface (spawn_y) in
        // 2..=8 the east-facing eye's frame keeps
        let mut dx = None;
        for d in 2..=8i32 {
            let surface = crate::screens::top_solid_y(w0, c + d, c);
            let clear = (1..=3).all(|dy| w0.get(c + d, sy + dy, c) == 0);
            if surface == sy && clear {
                dx = Some(d);
                break;
            }
        }
        let Some(dx) = dx else {
            return; // the seed-3 terrain keeps no corridor — the honest skip
        };
        // the rail one above the flat surface, the cart the useOn walk
        // spawns on it
        app.offline_world.as_mut().unwrap().set(c + dx, sy + 1, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c + dx, sy + 1, c, 1); // EAST_WEST
        let hit = crate::world::ClipHit {
            pos: (c + dx, sy + 1, c),
            face: 1,
            location: (c as f64 + dx as f64 + 0.5, sy as f64 + 2.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        assert_eq!(app.offline_carts.len(), 1, "the local cart spawned");
        // THE EAST-FACING EYE (the seed_scene's own body walk — the
        // spawn column's feet, the -90 yaw the sun-rise facing keeps)
        crate::headless::seed_scene(
            &mut app,
            &crate::headless::SceneOpts {
                body: true,
                perspective: Some(crate::camera::CameraType::FirstPerson),
                ..crate::headless::SceneOpts::default()
            },
        )
        .unwrap();
        assert!(app.cart_model.is_none(), "the lazy slot sleeps before the frame");
        let mut fb = crate::raster::Framebuf::new(64, 64);
        let mut assets = GuiAssets::open(assets_root());
        app.render_world_3d(&mut fb, &mut assets);
        assert!(app.cart_model.is_some(), "the lazy load fired — the sha1 kept");
        // THE TWIN SCENE: the same seed, the same rail, NO cart — the
        // frame diff the cart's own quads speak
        let mut twin = App::new(3);
        twin.enter_world();
        twin.offline_world.as_mut().unwrap().set(c + dx, sy + 1, c, crate::world::RAIL);
        twin.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c + dx, sy + 1, c, 1);
        crate::headless::seed_scene(
            &mut twin,
            &crate::headless::SceneOpts {
                body: true,
                perspective: Some(crate::camera::CameraType::FirstPerson),
                ..crate::headless::SceneOpts::default()
            },
        )
        .unwrap();
        let mut fb2 = crate::raster::Framebuf::new(64, 64);
        twin.render_world_3d(&mut fb2, &mut assets);
        let changed = fb
            .color
            .iter()
            .zip(fb2.color.iter())
            .filter(|(a, b)| a != b)
            .count();
        assert!(changed > 0, "the cart's quads recolor {} bytes of the frame", changed);
    }


    /// THE CART'S OWN MOUNT (javap Minecart.interact — the startRiding
    /// the ride belief arms): the 0.1875 attachment the EntityTypes
    /// row speaks, the seat the single-rider 0.0 keeps, the rotation
    /// anchor the client arm snapshots (the first drag delta the sync
    /// zeroes), the overlay word the lanes share, the ALREADY-RIDING
    /// gate the second ask's false keeps, the unknown id the honest
    /// false keeps.
    #[test]
    fn the_minecart_ride_the_mount_arms_the_belief() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/minecart"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1); // EAST_WEST
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        assert!(app.mount_offline_cart(1), "the startRiding's own true");
        let r = app.offline_ride.as_ref().expect("the ride belief armed");
        assert_eq!(r.vehicle_id, 1, "the cart's own id");
        assert_eq!(r.seat_x, 0.0, "the single rider's own seat");
        assert_eq!(
            r.ride_height,
            crate::cart::MINECART_PASSENGER_ATTACHMENT,
            "the seat height the EntityTypes row speaks"
        );
        assert_eq!(
            app.offline_cart_drag,
            Some((1, 0.0)),
            "the rotation anchor the mount snapshots"
        );
        assert_eq!(
            app.overlay_message.as_deref(),
            Some("Press Left Shift to Dismount"),
            "the mount's own overlay word"
        );
        // THE ALREADY-RIDING GATE (the isVehicle walk the interact
        // keeps — the occupied cart the second ask refuses)
        assert!(!app.mount_offline_cart(1), "the second ask the false keeps");
        // THE UNKNOWN ID (the honest false)
        assert!(!app.mount_offline_cart(999), "the unknown id's own false");
    }


    /// THE CART RIDE'S OWN CARRY (the positionRider walk the offline
    /// tick speaks): the cart's pose the carried feet ride (the
    /// 0.1875 the seat height adds), the body's own sleep the ride
    /// keeps (no impulses, no gravity), the ridden flag the isVehicle
    /// fold stamps (the slowdown arm reads), the SHIFT edge the ride
    /// drops (the drag anchor the dismount clears), the physics the
    /// next tick's walk resumes.
    #[test]
    fn the_minecart_ride_the_carry_follows_the_tick() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/minecart"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1);
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            ry as f64 + 2.0,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        assert!(app.mount_offline_cart(1), "the mount arms");
        // THE CARRY (the update walk's own order — the cart's tick the
        // ride's stamp arms, the carried feet the FRESH pose rides)
        app.update();
        let cart = &app.offline_carts[0].cart;
        assert!(cart.ridden, "the isVehicle fold the ride stamps");
        assert_eq!(cart.delta, [0.0, 0.0, 0.0], "the flat rail's own stillness");
        let b = app.body.as_ref().unwrap();
        assert_eq!(b.x, c as f64 + 0.5, "the cart's own x");
        assert_eq!(b.z, c as f64 + 0.5, "the cart's own z");
        assert!(
            (b.y - (ry as f64 + crate::cart::MINECART_PASSENGER_ATTACHMENT as f64)).abs() < 1e-9,
            "the seat height the carried feet ride (b.y {})",
            b.y
        );
        assert_eq!(b.vx, 0.0, "the body's own sleep");
        assert_eq!(b.vy, 0.0, "the gravity the ride never arms");
        assert!(!b.on_ground, "the ground never claims the rider");
        // THE SHIFT EDGE (the dismount's own ask): the ride drops, the
        // drag anchor the dismount clears, the ridden flag the next
        // tick's stamp clears
        app.on_movement_input(0.0, 0.0, false, true, false);
        assert!(app.offline_ride.is_none(), "the edge's own drop");
        assert!(app.offline_cart_drag.is_none(), "the anchor the drop clears");
        app.update();
        assert!(
            !app.offline_carts[0].cart.ridden,
            "the empty cart the next stamp clears"
        );
    }


    /// THE CART RIDE'S OWN ROTATION DRAG (Minecart.positionRider's
    /// client arm): the rider's yaw the cart's own turn drags — the
    /// delta the anchor's word speaks, the wrap the degrees law keeps,
    /// the anchor the carry re-arms (the next delta the fresh word
    /// reads), the mount's own first-tick silence the sync keeps.
    #[test]
    fn the_minecart_ride_the_yaw_drag() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/minecart"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1);
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            ry as f64 + 2.0,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        assert!(app.mount_offline_cart(1), "the mount arms");
        // THE FIRST TICK'S OWN SILENCE: the cart's yaw the mount
        // recorded — no turn, no drag (the sync the snapshot keeps)
        app.update();
        assert_eq!(app.body.as_ref().unwrap().yaw, 0.0, "the sync's own silence");
        // THE CURVE'S OWN DRAG: the cart's yaw the rail transition
        // stamps (the adjust_to_rails word the tick would re-wear) —
        // the rider's view the delta carries
        app.offline_carts[0].cart.yaw_deg = 30.0;
        app.update();
        assert!(
            (app.body.as_ref().unwrap().yaw - 30.0).abs() < 1e-4,
            "the drag the +30 curve speaks"
        );
        assert_eq!(
            app.offline_cart_drag,
            Some((1, 30.0)),
            "the anchor the carry re-arms"
        );
        // THE WRAP'S OWN WORD: the anchor 170, the cart -170 — the +20
        // delta the wrap speaks (the degrees law the ±180 band keeps),
        // the body's own 190 the ±180 band folds to -170 (the drag the
        // rider's accumulated view wears: 170 + 20 = 190 ≡ -170)
        app.offline_carts[0].cart.yaw_deg = 170.0;
        app.update();
        app.offline_carts[0].cart.yaw_deg = -170.0;
        app.update();
        assert!(
            (app.body.as_ref().unwrap().yaw - -170.0).abs() < 1e-4,
            "the wrapped delta the ±180 band keeps (yaw {})",
            app.body.as_ref().unwrap().yaw
        );
    }


    /// THE VARIANT CART'S OWN DISPLAY FRAME (the dev-110 walk's App
    /// lane): the chest cart the useOn walk spawns draws the carried
    /// block the special wrapper serves — the frame the chest's box
    /// paints above the hull walls the plain cart's own frame never
    /// speaks (the same rail, the same eye, the twin diff the display
    /// pass's own pixels answer).
    #[test]
    fn the_variant_carts_the_display_paints_the_frame() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/chest_minecart"));
        let w0 = app.offline_world.as_ref().unwrap();
        let c = w0.size_blocks() / 2;
        let sy = app.spawn_y;
        // THE VIEW CORRIDOR (the render-lane test's own scan — the
        // first flat eastward surface the eye's frame keeps)
        let mut dx = None;
        for d in 2..=8i32 {
            let surface = crate::screens::top_solid_y(w0, c + d, c);
            let clear = (1..=3).all(|dy| w0.get(c + d, sy + dy, c) == 0);
            if surface == sy && clear {
                dx = Some(d);
                break;
            }
        }
        let Some(dx) = dx else {
            return; // the seed-3 terrain keeps no corridor — the honest skip
        };
        app.offline_world.as_mut().unwrap().set(c + dx, sy + 1, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c + dx, sy + 1, c, 1); // EAST_WEST
        let hit = crate::world::ClipHit {
            pos: (c + dx, sy + 1, c),
            face: 1,
            location: (c as f64 + dx as f64 + 0.5, sy as f64 + 2.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        assert_eq!(app.offline_carts.len(), 1, "the chest cart spawned");
        assert_eq!(
            app.offline_carts[0].kind, "chest_minecart",
            "the kind the item name's family speaks"
        );
        crate::headless::seed_scene(
            &mut app,
            &crate::headless::SceneOpts {
                body: true,
                perspective: Some(crate::camera::CameraType::FirstPerson),
                ..crate::headless::SceneOpts::default()
            },
        )
        .unwrap();
        let mut fb = crate::raster::Framebuf::new(64, 64);
        let mut assets = GuiAssets::open(assets_root());
        app.render_world_3d(&mut fb, &mut assets);
        // THE TWIN: the SAME rail, the PLAIN cart — the display pass
        // the AIR gate skips, the frame the hull alone paints
        let mut twin = App::new(3);
        twin.enter_world();
        twin.offline_held_updated(Some("item/minecart"));
        twin.offline_world.as_mut().unwrap().set(c + dx, sy + 1, c, crate::world::RAIL);
        twin.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c + dx, sy + 1, c, 1);
        let hit2 = crate::world::ClipHit {
            pos: (c + dx, sy + 1, c),
            face: 1,
            location: (c as f64 + dx as f64 + 0.5, sy as f64 + 2.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(twin.on_right_press_minecart_arm(&hit2), Some(true));
        crate::headless::seed_scene(
            &mut twin,
            &crate::headless::SceneOpts {
                body: true,
                perspective: Some(crate::camera::CameraType::FirstPerson),
                ..crate::headless::SceneOpts::default()
            },
        )
        .unwrap();
        let mut fb2 = crate::raster::Framebuf::new(64, 64);
        twin.render_world_3d(&mut fb2, &mut assets);
        let changed = fb
            .color
            .iter()
            .zip(fb2.color.iter())
            .filter(|(a, b)| a != b)
            .count();
        assert!(
            changed > 0,
            "the chest's own box recolors {} bytes the hull never speaks",
            changed
        );
    }


    /// THE CART'S OWN ATTACK WALK (the dev-111 round's App lane): the
    /// punches the hurtServer verdict wears — four fists the exact
    /// 40.0 the strict `>` refuses, the FIFTH the destroy verdict
    /// arms — the cart the offline vec drops, the drop the
    /// getDropItem item the cart's own position carries, the ride the
    /// destroyed vehicle the eject clears, the wobble decade the
    /// surviving cart's own fields speak.
    #[test]
    fn the_cart_attack_the_hurt_and_destroy_walk() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/minecart"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1);
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        assert_eq!(app.offline_carts.len(), 1, "the cart spawned");
        let cart_pos = app.offline_carts[0].cart.pos;
        // THE FULL CHARGE THE FIST'S OWN CLOCK READS (the ticker the
        // max the meter keeps — the 2.0 attribute the (0.2+0.8·1²)
        // scale the exact 2.0 strike answers, the +20 per punch the
        // hurtServer's scale speaks)
        app.attack_ticker = 100;
        // THE TWO SURVIVING PUNCHES (the fist's 2.0 the +20 each
        // speaks — the strict `>` the exact 40.0 the second punch's
        // sum refuses, the wobble decade each hit re-arms, the flip
        // pairs the hits keep)
        for i in 0..2 {
            // THE METER'S OWN RE-ARM (the reset the punch's walk wore —
            // the full charge the waited-out clock re-reads)
            app.attack_ticker = 100;
            let pre_dir = app.offline_carts[0].hurt_dir;
            app.attack_cart(1);
            assert_eq!(app.offline_carts.len(), 1, "the punch {} survives", i + 1);
            assert_eq!(app.offline_carts[0].hurt_dir, -pre_dir, "the flip");
            assert_eq!(app.offline_carts[0].hurt_time, 10, "the decade armed");
            assert!(
                (app.offline_carts[0].damage - (20.0 * (i + 1) as f32)).abs() < 1e-4,
                "the damage the punch {} wears",
                i + 1
            );
        }
        // THE WOBBLE'S OWN BURN (the 20-TPS beat the countdown serves)
        app.update();
        assert_eq!(app.offline_carts[0].hurt_time, 9, "the tick burned one");
        // THE THIRD PUNCH'S DESTROY (the 60.0 the threshold crosses —
        // the cart the vec drops, the drop the minecart item the
        // cart's own position wears, the entity-loot spawn the
        // pickup-delay arms)
        app.attack_ticker = 100;
        app.attack_cart(1);
        assert!(app.offline_carts.is_empty(), "the destroyed cart left");
        assert_eq!(app.drops.len(), 1, "the drop spawned");
        let drop = &app.drops[0];
        assert_eq!(
            mc_registry::item_name(drop.item),
            Some("minecart"),
            "the getDropItem item"
        );
        assert_eq!(drop.count, 1);
        assert!((drop.x - cart_pos[0]).abs() < 1e-6, "the cart's own x");
        assert!((drop.y - cart_pos[1]).abs() < 1e-6, "the cart's own y");
        assert!((drop.z - cart_pos[2]).abs() < 1e-6, "the cart's own z");
        // THE RIDE'S OWN EJECT (the ride belief the destroyed vehicle
        // drops — the drag anchor the eject clears)
        let mut app2 = App::new(3);
        app2.enter_world();
        app2.offline_held_updated(Some("item/minecart"));
        app2.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app2.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1);
        let hit2 = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app2.on_right_press_minecart_arm(&hit2), Some(true));
        app2.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            ry as f64 + 2.0,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        assert!(app2.mount_offline_cart(1), "the mount arms");
        for _ in 0..3 {
            app2.attack_ticker = 100;
            app2.attack_cart(1);
        }
        assert!(app2.offline_carts.is_empty(), "the cart destroyed under the rider");
        assert!(app2.offline_ride.is_none(), "the ride ejected");
        assert!(app2.offline_cart_drag.is_none(), "the drag cleared");
    }


    /// THE BOAT'S OWN PUNCH LANE (javap `VehicleEntity.hurtServer` the
    /// 26.2 jar speaks — the walk the AbstractBoat's own method table
    /// keeps NO override of): the wobble the decade arms (the client's
    /// copy the seen-tracker adopts, the decay the tick burns), the
    /// damage the amount·10 ramps, the destroy at the THIRD full-charge
    /// punch (the 60.0 the strict > crosses — the 2.0 attribute the
    /// meter's full charge wears), the drop the EntityTypes clinit's
    /// own lambda name spawns, the raw word the server's own arm keeps
    /// (the decay the CLIENT's own copy speaks, the wire's word the
    /// offline lane freezes).
    #[test]
    fn the_boat_the_punch_wobbles_and_the_third_breaks() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_boats.push(crate::world_view::WireBoat {
            id: 7,
            x: 10.0,
            y: 64.0,
            z: -3.0,
            yaw: 90.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        });
        app.body = Some(crate::body::PlayerBody::new(10.5, 65.0, -2.0, 0.0, 0.0));
        // THE TWO SURVIVING PUNCHES (the meter's own re-arm the punch's
        // walk wore — the full charge the waited-out clock re-reads)
        for i in 0..2 {
            app.attack_ticker = 100;
            let pre_dir = app.offline_boats[0].hurt_dir;
            app.attack_boat(7);
            assert_eq!(app.offline_boats.len(), 1, "the punch {} survives", i + 1);
            assert_eq!(app.offline_boats[0].hurt_dir, -pre_dir, "the flip");
            assert_eq!(app.offline_boats[0].hurt_time, 10, "the decade armed");
            assert!(
                (app.offline_boats[0].damage - (20.0 * (i + 1) as f32)).abs() < 1e-4,
                "the damage the punch {} wears",
                i + 1
            );
            // THE CLIENT'S OWN COPY (the seen-tracker the wobble's word
            // arms — the same trio the raw word wears)
            let h = app.hurt_state.get(&7).expect("the wobble state armed");
            assert_eq!(h.time, 10, "the client's decade");
            assert_eq!(h.dir, -pre_dir, "the client's flip");
            assert!(
                (h.damage - (20.0 * (i + 1) as f32)).abs() < 1e-4,
                "the client's ramp"
            );
        }
        // THE DECAY'S OWN BURN (the AbstractBoat.tick's client arm — the
        // −1 step the time speaks, the raw word the server's own keeps;
        // the ramp the two punches' 40.0 wears — the client's copy the
        // −1.0 the beat falls)
        app.update();
        assert_eq!(
            app.hurt_state.get(&7).unwrap().time,
            9,
            "the tick burned one"
        );
        assert!(
            (app.hurt_state.get(&7).unwrap().damage - 39.0).abs() < 1e-4,
            "the ramp fell one"
        );
        assert_eq!(
            app.offline_boats[0].hurt_time, 10,
            "the raw word the server's own keeps"
        );
        // THE THIRD PUNCH'S DESTROY (the 60.0 the threshold crosses —
        // the hull the vec drops, the drop the oak_boat item the
        // EntityTypes clinit's own lambda name wears, the hull's own
        // position the entity-loot spawn carries — the pose the float's
        // own tick settled)
        let (bx, by, bz) = (
            app.offline_boats[0].x,
            app.offline_boats[0].y,
            app.offline_boats[0].z,
        );
        app.attack_ticker = 100;
        app.attack_boat(7);
        assert!(app.offline_boats.is_empty(), "the destroyed hull left");
        assert_eq!(app.drops.len(), 1, "the drop spawned");
        let drop = &app.drops[0];
        assert_eq!(
            mc_registry::item_name(drop.item),
            Some("oak_boat"),
            "the getDropItem item"
        );
        assert_eq!(drop.count, 1);
        assert!((drop.x - bx).abs() < 1e-6, "the hull's own x");
        assert!((drop.y - by).abs() < 1e-6, "the hull's own y");
        assert!((drop.z - bz).abs() < 1e-6, "the hull's own z");
    }


    /// THE DESTROYED BOAT'S OWN EJECT (the VehicleEntity.destroy
    /// chain's own tail — the ride belief the destroyed hull drops, the
    /// rider the standing walk the next tick resumes)
    #[test]
    fn the_destroyed_boat_the_rider_ejects() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_boats.push(crate::world_view::WireBoat {
            id: 7,
            x: 10.0,
            y: 64.0,
            z: -3.0,
            yaw: 90.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        });
        app.body = Some(crate::body::PlayerBody::new(10.5, 65.0, -2.0, 0.0, 0.0));
        assert!(app.mount_offline_hull(7), "the mount arms");
        for _ in 0..3 {
            app.attack_ticker = 100;
            app.attack_boat(7);
        }
        assert!(
            app.offline_boats.is_empty(),
            "the hull destroyed under the rider"
        );
        assert!(app.offline_ride.is_none(), "the ride ejected");
    }


    /// THE TNT CART'S OWN WORLD DAMAGE (javap `MinecartTNT.explode` the
    /// 26.2 jar speaks — the `level.explode(this, source, null, x, y,
    /// z, power, false, ExplosionInteraction.TNT)` call the ray machine
    /// serves: the DESTROY_WITH_DECAY interaction the gamerule's true
    /// default keeps, the ray scan the power law drives, the shuffle +
    /// the decay the loot roll serves, the TNT cells the wasExploded
    /// chain primes, the cart's own discard the tick's tail serves).
    #[test]
    fn the_tnt_cart_the_detonation_breaks_the_world() {
        // THE TABLES' OWN ROWS (the blast arc's own data — the
        // getExplosionResistance query the runtime dumper banked, the
        // decay rows the loot JSONs bank)
        assert!(
            (mc_registry::generated::block_resistances::resistance("stone") - 6.0).abs() < 1e-4,
            "the clinit's own strength row"
        );
        assert!(
            (mc_registry::generated::block_resistances::resistance("water") - 100.0).abs() < 1e-3,
            "the water block's own 100.0 row"
        );
        assert!(
            (mc_registry::generated::block_resistances::resistance("obsidian") - 1200.0).abs()
                < 1e-2,
            "the explosionResistance override the one row speaks"
        );
        assert_eq!(
            mc_registry::generated::block_resistances::resistance("wheat_stage3"),
            0.0,
            "the registry-foreign name the 0.0 default keeps"
        );
        assert!(
            mc_registry::generated::block_explosion_decay::has_explosion_decay("iron_ore"),
            "the ore rows the decay wears"
        );
        assert!(
            !mc_registry::generated::block_explosion_decay::has_explosion_decay("stone"),
            "the cobblestone arm the decay never wore"
        );

        let mut app = App::new(3);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        // THE BLAST'S OWN STAGE (the cleared box the walk's own rays
        // march — the pad the dirt rows speak, the TNT cell the chain
        // queue wears)
        for dx in -4..=4 {
            for dy in -3..=3 {
                for dz in -4..=4 {
                    app.offline_world
                        .as_mut()
                        .unwrap()
                        .set(c + dx, ry + dy, c + dz, crate::world::AIR);
                }
            }
        }
        for dx in -2..=2 {
            for dz in -2..=2 {
                app.offline_world
                    .as_mut()
                    .unwrap()
                    .set(c + dx, ry - 1, c + dz, crate::world::DIRT);
            }
        }
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c + 2, ry, c, crate::world::TNT);
        // THE CART'S OWN SPAWN (the minecart arm the right-press walk
        // serves)
        app.offline_held_updated(Some("item/tnt_minecart"));
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        assert_eq!(app.offline_carts.len(), 1, "the cart spawned");
        // THE FUSE-0 BEAT (the prime the 80 wears, the zero the arm the
        // next tick's own verdict keeps — the honest walk the burn's
        // own law wears)
        crate::cart::prime_tnt_cart(&mut app.offline_carts[0]);
        app.offline_carts[0].fuse = 0;
        app.update();
        // THE CART'S OWN LEAVE (the discard the tail serves)
        assert!(app.offline_carts.is_empty(), "the cart consumed");
        // THE WORLD'S OWN CRATER (the cells the ray walk took — the
        // pad's center + the near edge the intensity always folds, the
        // TNT cell the chain queue wears)
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(c, ry - 1, c),
            crate::world::AIR,
            "the pad's own center"
        );
        assert_eq!(
            world.get(c + 1, ry - 1, c),
            crate::world::AIR,
            "the pad's near edge"
        );
        assert_eq!(
            world.get(c + 2, ry, c),
            crate::world::AIR,
            "the TNT cell the chain took"
        );
        // THE CHAIN'S OWN PRIME (the wasExploded walk the short-fuse
        // roll serves — the PrimedTnt the queue spawns at the cell's
        // own center)
        assert!(
            app.primed_tnts
                .iter()
                .any(|t| (t.x - (c as f64 + 2.5)).abs() < 1e-3),
            "the chain's own prime"
        );
        // THE PAD'S OWN DROPS (the dirt rows the loot eval wears — the
        // decay the table never wears, the count the definite pass
        // keeps)
        assert!(!app.drops.is_empty(), "the pad's own drop");
    }


    /// THE TNT CART'S OWN CRASH (javap `MinecartTNT.tick`'s
    /// `horizontalCollision && hsqr >= 0.01` arm — the flag the
    /// Entity.move collision fold stores, the arm the fuse state
    /// NEVER reads): the powered rail's launch the unprimed cart
    /// rolls east, the plain rails the speed chain carries, the east
    /// wall the box's over-band clip the crash speaks — the SAME
    /// tick's fuse arm the detonation answers, the world the blast
    /// eats, the stone wall the 6.0 resistance keeps.
    #[test]
    fn the_tnt_cart_the_crash_the_wall_detonates() {
        let mut app = App::new(3);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        // THE STAGE: the cleared corridor, the dirt pad the blast
        // eats, the powered rail the west wall launches (the
        // getRedstoneDirection's conductor read speaks), the plain
        // rails carry, the east wall the fast cart meets.
        for dx in -4..=6 {
            for dy in -3..=3 {
                for dz in -4..=4 {
                    app.offline_world
                        .as_mut()
                        .unwrap()
                        .set(c + dx, ry + dy, c + dz, crate::world::AIR);
                }
            }
        }
        for dx in -2..=4 {
            for dz in -2..=2 {
                app.offline_world
                    .as_mut()
                    .unwrap()
                    .set(c + dx, ry - 1, c + dz, crate::world::DIRT);
            }
        }
        let w = app.offline_world.as_mut().unwrap();
        w.set(c - 1, ry, c, crate::world::STONE); // the launch's own wall
        w.set(c, ry, c, crate::world::POWERED_RAIL);
        w.set_variant(c, ry, c, 1 | crate::world::RAIL_POWERED_BIT);
        w.set(c + 1, ry, c, crate::world::RAIL);
        w.set_variant(c + 1, ry, c, 1);
        w.set(c + 2, ry, c, crate::world::RAIL);
        w.set_variant(c + 2, ry, c, 1);
        w.set(c + 3, ry, c, crate::world::STONE); // the crash wall
        // THE SPAWN (the useOn arm the minecart walk serves — the
        // UNPRIMED fuse the iconst_m1 keeps)
        app.offline_held_updated(Some("item/tnt_minecart"));
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        assert_eq!(app.offline_carts.len(), 1, "the cart spawned");
        assert_eq!(app.offline_carts[0].fuse, crate::cart::TNT_FUSE_UNPRIMED);
        // THE ROLL: the launch the 0.2 east speaks, the speed chain
        // the wall approach carries — the box the over-band clip the
        // crash tick wears, the flag the same beat's fuse arm reads
        // (the detonation the update's explosion block consumes).
        for _ in 0..14 {
            if app.offline_carts.is_empty() {
                break;
            }
            app.update();
        }
        // THE DETONATION'S OWN WORDS: the cart the discard takes, the
        // pad the blast eats (the 0.5 resistance gives way), the
        // stone wall the (res + 0.3)·0.3 fold takes (the 6.0's own
        // 1.89 erosion the over-2.34 intensities beat — the weakest
        // ray's 4.0·0.7 − the 0.45 air march = 2.35 the wall never
        // keeps), the loot the shuffle roll wears.
        assert!(app.offline_carts.is_empty(), "the crash consumed the cart");
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(c, ry - 1, c),
            crate::world::AIR,
            "the pad's center the blast took"
        );
        assert_eq!(
            world.get(c + 1, ry - 1, c),
            crate::world::AIR,
            "the pad's east edge the blast took"
        );
        assert_eq!(
            world.get(c + 3, ry, c),
            crate::world::AIR,
            "the wall the blast's own fold took"
        );
        assert!(!app.drops.is_empty(), "the pad's own drops the loot roll wears");
    }


    /// THE EXPLOSION'S OWN FLASH (javap `ClientPacketListener
    /// .handleExplosion` pc 100-125 + `ServerExplosion.isSmall` pc
    /// 0-21 the 26.2 jar — the addParticle arm the voice's tail
    /// keeps, the isSmall fork the radius compare arms: the large
    /// blast the EXPLOSION_EMITTER seed wears, the small blast the
    /// single HugeExplosion flash answers).
    #[test]
    fn the_explosion_flash_the_center_the_small_fork_wears() {
        // THE LARGE ARM (the emitter seed the invisible engine keeps
        // — the 8-tick countdown the ctor's own write banks, the six
        // children per tick the same walk serves)
        let mut rng = crate::drops::Lcg::new(0x600D);
        let big = crate::particles::explosion_flash(10.0, 64.0, 10.0, 4.0, &mut rng);
        assert!(
            matches!(
                big.kind,
                crate::particles::ParticleKind::ExplosionSeed
            ),
            "the large blast the emitter seed wears"
        );
        assert_eq!(
            big.lifetime, 8,
            "the 8-tick countdown the ctor's own write keeps"
        );
        assert_eq!(
            big.quad_size, 0.0,
            "the invisible engine the no-quad lane keeps"
        );
        assert_eq!(
            (big.x, big.y, big.z),
            (10.0, 64.0, 10.0),
            "the center's exact coords the packet's own Vec3 wears"
        );
        // THE SMALL ARM (the single flash the full quad wears — the
        // `6 + nextInt(4)` lifetime band the ctor's own write keeps,
        // the age-0 size-speed the 2.0 fold answers, the grey tint
        // band the ONE draw gives)
        let small = crate::particles::explosion_flash(5.0, 64.0, 5.0, 1.5, &mut rng);
        assert!(
            matches!(small.kind, crate::particles::ParticleKind::Explosion),
            "the small blast the single flash answers"
        );
        assert!(
            (6..=9).contains(&small.lifetime),
            "the 6 + nextInt(4) band the ctor's own write keeps"
        );
        assert!(
            (small.quad_size - 2.0).abs() < 1e-6,
            "the age-0 size-speed the full quad answers"
        );
        assert!(
            small.col[0] >= 0.4 - 1e-6 && small.col[0] <= 1.0,
            "the grey tint band the ONE draw keeps"
        );
        assert_eq!(
            (small.x, small.y, small.z),
            (5.0, 64.0, 5.0),
            "the small blast's own center keeps"
        );
        // THE WALK (the cart boom's flash the same center the
        // handleExplosion arm plants — the sound the same packet walk
        // serves)
        let mut app = App::new(3);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        let (fx, fy, fz) = (c as f64 + 0.5, ry as f64 + 0.5, c as f64 + 0.5);
        app.pending_cart_booms.push((fx, fy, fz, 4.0));
        for _ in 0..2 {
            app.update();
        }
        assert!(
            app.particles.iter().any(|p| matches!(
                p.kind,
                crate::particles::ParticleKind::ExplosionSeed
            ) && (p.x - fx).abs() < 1e-9
                && (p.y - fy).abs() < 1e-9
                && (p.z - fz).abs() < 1e-9),
            "the boom's own flash the center plants"
        );
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.contains("entity.generic.explode")),
                "the boom's own voice the same packet walk plays"
            );
        }
    }


    /// THE TNT CART'S OWN DESTROY FORK (javap `MinecartTNT.destroy`
    /// the 26.2 jar speaks — the slow unprimed cart the Drop arm
    /// answers, the fast cart the `hsqr >= 0.01` gate the priming
    /// arm arms: the `nextInt(20) + nextInt(20)` reroll the fuse
    /// overwrites, the cart the scene KEEPS, the drop the priming
    /// arm never spawns, the burn the next beat serves).
    #[test]
    fn the_tnt_cart_the_slow_punch_drops_the_fast_punch_primes() {
        // THE SLOW TWIN (the resting cart the drop wears — the same
        // walk the plain family's own destroy keeps)
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/tnt_minecart"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        app.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1);
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        assert_eq!(app.offline_carts.len(), 1, "the tnt cart spawned");
        assert_eq!(app.offline_carts[0].kind, "tnt_minecart");
        assert_eq!(
            app.offline_carts[0].fuse,
            crate::cart::TNT_FUSE_UNPRIMED,
            "the <init> the unprimed −1 keeps"
        );
        for _ in 0..3 {
            app.attack_ticker = 100;
            app.attack_cart(1);
        }
        assert!(app.offline_carts.is_empty(), "the slow cart the drop wears");
        assert_eq!(app.drops.len(), 1, "the drop spawned");
        assert_eq!(
            mc_registry::item_name(app.drops[0].item),
            Some("tnt_minecart"),
            "the getDropItem item"
        );
        // THE FAST TWIN (the rolling cart the priming arm arms — the
        // delta the test's own push wears, the destroy verdict the
        // same three punches reads)
        let mut app2 = App::new(3);
        app2.enter_world();
        app2.offline_held_updated(Some("item/tnt_minecart"));
        app2.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app2.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1);
        let hit2 = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app2.on_right_press_minecart_arm(&hit2), Some(true));
        app2.offline_carts[0].cart.delta = [0.3, 0.0, 0.0];
        for _ in 0..3 {
            app2.attack_ticker = 100;
            app2.attack_cart(1);
        }
        assert_eq!(app2.offline_carts.len(), 1, "the priming arm the cart KEEPS");
        assert!(app2.drops.is_empty(), "the priming arm the drop refuses");
        let fused = app2.offline_carts[0].fuse;
        assert!(
            (0..crate::cart::TNT_DESTROY_REROLL_BOUND * 2).contains(&fused),
            "the nextInt(20)+nextInt(20) reroll the 0..38 keeps ({fused})"
        );
        assert!(
            (app2.offline_carts[0].damage - 60.0).abs() < 1e-4,
            "the hurt walk's own accumulator the priming arm keeps"
        );
        // THE DETERMINISM TWIN (the same seed the same reroll wears —
        // the Lcg the App::new(3) the same 0x5EED_10CA arms)
        let mut app3 = App::new(3);
        app3.enter_world();
        app3.offline_held_updated(Some("item/tnt_minecart"));
        app3.offline_world.as_mut().unwrap().set(c, ry, c, crate::world::RAIL);
        app3.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1);
        let hit3 = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app3.on_right_press_minecart_arm(&hit3), Some(true));
        app3.offline_carts[0].cart.delta = [0.3, 0.0, 0.0];
        for _ in 0..3 {
            app3.attack_ticker = 100;
            app3.attack_cart(1);
        }
        assert_eq!(
            app3.offline_carts[0].fuse, fused,
            "the same seed the same reroll speaks"
        );
        // THE BURN'S OWN TAIL (the test's own steering the known fuse
        // wears — the 5 the five smoke beats burn, the sixth the
        // fuse-0 explode answers: the cart the discard takes, the
        // drop the consumed entity never spawns)
        app2.offline_carts[0].fuse = 5;
        let mut smokes = 0usize;
        for beat in 0..6 {
            let before = app2.particles.len();
            app2.update();
            smokes += app2.particles.len() - before;
            if beat < 5 {
                assert_eq!(app2.offline_carts.len(), 1, "the burn the beat {} wears", beat);
            }
        }
        assert!(smokes >= 5, "the SMOKE puff the burn beats wear ({smokes})");
        assert!(app2.offline_carts.is_empty(), "the fuse-0 explode the discard speaks");
        // THE CONSUMED CART'S OWN DROP (the destroy chain the explode
        // walk never serves — the getDropItem item the consumed entity
        // never spawns; the WORLD'S own drops the blast's loot roll
        // wears — the destroyed cells' own loot the shuffle spawns)
        assert!(
            app2
                .drops
                .iter()
                .all(|d| mc_registry::item_name(d.item) != Some("minecart")),
            "the consumed cart the drop never spawns"
        );
    }


    /// THE ACTIVATOR RAIL'S OWN PRIME (javap the `activateMinecart`
    /// walk the inside-blocks pass serves — the powered pass the
    /// `fuse < 0` gate the primeFuse(null) arms, the unpowered pass
    /// the silence keeps, the burn the same-beat fuse the SMOKE
    /// speaks, the fuse-0 explode the discard answers).
    #[test]
    fn the_tnt_cart_the_activator_rail_primes_and_burns() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/tnt_minecart"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let ry = app.spawn_y + 6;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, ry, c, crate::world::ACTIVATOR_RAIL);
        app.offline_world.as_mut().unwrap().set_variant(
            c,
            ry,
            c,
            1 | crate::world::RAIL_POWERED_BIT,
        );
        let hit = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_minecart_arm(&hit), Some(true));
        assert_eq!(app.offline_carts.len(), 1, "the cart spawned on the pass");
        assert_eq!(app.offline_carts[0].fuse, crate::cart::TNT_FUSE_UNPRIMED);
        // THE POWERED PASS (the beat the prime + the same-beat burn
        // wears — the checkInsideBlocks the prime arms BEFORE the
        // fuse arms the bytecode's own super.tick order keeps, the
        // 80→79 the same beat speaks, the SMOKE the puff serves)
        let before = app.particles.len();
        app.update();
        assert_eq!(app.offline_carts[0].fuse, 79, "the prime the same beat burns");
        assert!(app.particles.len() > before, "the SMOKE the beat wears");
        // THE BURN'S OWN TAIL (the 79 beats the countdown serves —
        // the fuse-0 the last burn keeps, the 80th the explode
        // answers)
        for _ in 0..79 {
            app.update();
        }
        assert_eq!(
            app.offline_carts.len(),
            1,
            "the fuse-0 cart the last burn keeps"
        );
        assert_eq!(app.offline_carts[0].fuse, 0, "the zero the burn reached");
        app.update();
        assert!(app.offline_carts.is_empty(), "the explode the discard speaks");
        // THE CONSUMED CART'S OWN DROP (the destroy chain the explode
        // walk never serves — the getDropItem item the consumed entity
        // never spawns; the WORLD'S own drops the blast's loot roll
        // wears — the destroyed cells' own loot the shuffle spawns)
        assert!(
            app.drops
                .iter()
                .all(|d| mc_registry::item_name(d.item) != Some("minecart")),
            "the consumed cart the drop never spawns"
        );
        // THE UNPOWERED TWIN (the silent pass the prime refuses — the
        // fuse the unprimed −1 the same walk keeps)
        let mut app2 = App::new(3);
        app2.enter_world();
        app2.offline_held_updated(Some("item/tnt_minecart"));
        app2.offline_world
            .as_mut()
            .unwrap()
            .set(c, ry, c, crate::world::ACTIVATOR_RAIL);
        app2.offline_world
            .as_mut()
            .unwrap()
            .set_variant(c, ry, c, 1);
        let hit2 = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app2.on_right_press_minecart_arm(&hit2), Some(true));
        app2.update();
        assert_eq!(
            app2.offline_carts[0].fuse,
            crate::cart::TNT_FUSE_UNPRIMED,
            "the unpowered pass the silence keeps"
        );
        // THE RIDER'S OWN EJECT (the exploding cart the rider's
        // belief drops — the drag anchor the eject clears)
        let mut app3 = App::new(3);
        app3.enter_world();
        app3.offline_held_updated(Some("item/tnt_minecart"));
        app3.offline_world
            .as_mut()
            .unwrap()
            .set(c, ry, c, crate::world::ACTIVATOR_RAIL);
        app3.offline_world.as_mut().unwrap().set_variant(
            c,
            ry,
            c,
            1 | crate::world::RAIL_POWERED_BIT,
        );
        let hit3 = crate::world::ClipHit {
            pos: (c, ry, c),
            face: 1,
            location: (c as f64 + 0.5, ry as f64 + 1.0, c as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app3.on_right_press_minecart_arm(&hit3), Some(true));
        app3.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            ry as f64 + 2.0,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        assert!(app3.mount_offline_cart(1), "the mount arms");
        for _ in 0..81 {
            app3.update();
        }
        assert!(app3.offline_carts.is_empty(), "the exploded cart left");
        assert!(app3.offline_ride.is_none(), "the ride the explode ejects");
        assert!(app3.offline_cart_drag.is_none(), "the drag the eject clears");
    }


    /// THE ITEM FRAME'S OWN LANES (javap HangingEntityItem.useOn +
    /// ItemFrame.interact/setItem/setRotation the 26.2 jar — the
    /// placement the horizontal face + the survives law wears, the
    /// store lane the count-1 + the consume + the add_item voice
    /// banks, the rotate lane the +1 %8 + the rotate_item voice
    /// serves, the sneak slot lane the swap the hand keeps, the
    /// analog row the map the World banks, the broadcast the
    /// through-wall comparator the +2 schedule re-arms).
    #[test]
    fn the_item_frame_the_place_store_rotate_take_lanes_wear() {
        let mut app = App::new(3);
        app.enter_world();
        let w0 = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let y = app.spawn_y + 6;
        // THE WALL (the support cell the collision-bearing kind keeps)
        let mut w = app.offline_world.as_mut().unwrap();
        w.set(w0, y, w0, crate::world::STONE);
        w.set(w0, y, w0 - 1, crate::world::AIR); // the frame cell the air keeps
        drop(w);
        // THE PLACE (the face NORTH = 2 — the frame cell = the wall's
        // north neighbor, the place voice the playPlacementSound speaks)
        let hit = crate::world::ClipHit {
            pos: (w0, y, w0),
            face: 2,
            location: (w0 as f64 + 0.5, y as f64 + 0.5, w0 as f64),
            inside: false,
        };
        assert_eq!(
            app.on_right_press_item_frame_arm(&hit),
            Some(true),
            "the wall click the frame spawns"
        );
        assert_eq!(app.item_frames.len(), 1, "the one frame the spawn keeps");
        assert_eq!(app.item_frames[0].cell, (w0, y, w0 - 1), "the cell the +step law keeps");
        assert_eq!(app.item_frames[0].facing, 2, "the facing the clicked face wears");
        assert_eq!(app.item_frames[0].rotation, 0, "the rotation the default 0 keeps");
        assert!(app
            .sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.item_frame.place ")),
            "the place voice the playPlacementSound speaks");
        // THE ANALOG ROW (the empty frame the 0 the map banks)
        assert_eq!(
            app.offline_world.as_ref().unwrap().frame_analog.get(&(w0, y, w0 - 1)),
            Some(&0),
            "the empty frame the 0 row banks"
        );
        // THE VERTICAL FACE'S OWN REFUSAL (the mayPlace law — the UP
        // click the None the chain keeps)
        let up_hit = crate::world::ClipHit {
            pos: (w0, y, w0),
            face: 1,
            location: (w0 as f64 + 0.5, y as f64 + 1.0, w0 as f64 + 0.5),
            inside: false,
        };
        assert_eq!(
            app.on_right_press_item_frame_arm(&up_hit),
            None,
            "the floor click the vertical face the chain refuses"
        );
        // THE SUPPORT'S OWN REFUSAL (the replaceable cell the survives
        // law refuses — the air wall the FAIL speaks)
        app.offline_world.as_mut().unwrap().set(w0 + 4, y, w0, crate::world::AIR);
        let air_hit = crate::world::ClipHit {
            pos: (w0 + 4, y, w0),
            face: 2,
            location: ((w0 + 4) as f64 + 0.5, y as f64 + 0.5, w0 as f64),
            inside: false,
        };
        assert_eq!(
            app.on_right_press_item_frame_arm(&air_hit),
            Some(false),
            "the air support the survives law refuses"
        );

        // THE STORE LANE (the held stack the count-1 copy the frame
        // banks, the consume the survival shrink wears, the add_item
        // voice the setItem's own arm speaks, the analog row the +1
        // fold banks)
        let ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
        app.inventory.items[app.selected_hotbar] = (ingot, 5);
        app.hotbar_slots = app.inventory.hotbar();
        let fid = app.item_frames[0].id;
        assert!(
            app.on_right_press_item_frame_interact(fid),
            "the store lane the SUCCESS verdict carries"
        );
        assert_eq!(app.item_frames[0].item, (ingot, 1), "the count-1 the copyWithCount fold wears");
        assert_eq!(app.inventory.items[app.selected_hotbar], (ingot, 4), "the consume the shrink rides");
        assert_eq!(
            app.offline_world.as_ref().unwrap().frame_analog.get(&(w0, y, w0 - 1)),
            Some(&1),
            "the analog the rotation+1 the 1 banks"
        );
        assert!(app
            .sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.item_frame.add_item ")),
            "the add_item voice the setItem walk speaks");

        // THE ROTATE LANE (the empty hand the +1 %8 the setRotation
        // banks — the rotate_item voice opens)
        app.inventory.items[app.selected_hotbar] = (0, 0);
        app.hotbar_slots = app.inventory.hotbar();
        app.offline_held = None;
        assert!(
            app.on_right_press_item_frame_interact(fid),
            "the rotate lane the SUCCESS verdict carries"
        );
        assert_eq!(app.item_frames[0].rotation, 1, "the rotation the +1 the wrap banks");
        assert_eq!(
            app.offline_world.as_ref().unwrap().frame_analog.get(&(w0, y, w0 - 1)),
            Some(&2),
            "the analog the rotation 1 the 2 banks"
        );
        assert!(app
            .sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.item_frame.rotate_item ")),
            "the rotate_item voice the interact walk speaks");

        // THE SNEAK SLOT LANE (the filled frame the swap the hand
        // keeps — the frame empties, the item returns)
        app.shift_down = true;
        assert!(
            app.on_right_press_item_frame_interact(fid),
            "the sneak take the SUCCESS verdict carries"
        );
        assert_eq!(app.item_frames[0].item, (0, 0), "the frame the take empties");
        assert_eq!(
            app.inventory.items[app.selected_hotbar],
            (ingot, 1),
            "the hand the frame's own item keeps"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().frame_analog.get(&(w0, y, w0 - 1)),
            Some(&0),
            "the analog the empty frame the 0 re-banks"
        );
        // the held swap: the EMPTY frame + the held stack the STORE
        // lane answers (the sneak lane the filled frame alone serves)
        app.inventory.items[app.selected_hotbar] = (ingot, 3);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(app.on_right_press_item_frame_interact(fid), "the store the SUCCESS carries");
        assert_eq!(app.item_frames[0].item, (ingot, 1), "the store the count-1 the frame banks");
        assert_eq!(
            app.inventory.items[app.selected_hotbar],
            (ingot, 2),
            "the consume the stack's remainder keeps"
        );
    }


    /// THE ITEM FRAME'S OWN PUNCH LANES (javap ItemFrame.hurtServer the
    /// 26.2 jar — the FILLED frame the drop walk the frame item + the
    /// contained item the pair spawns, the remove_item voice the arm
    /// plays, the EMPTY frame the super's kill the break voice + the
    /// frame item's own drop keeps, the analog row the death's own map
    /// ride drops).
    #[test]
    fn the_item_frame_the_punch_lanes_the_drops_and_voices_wear() {
        let mut app = App::new(3);
        app.enter_world();
        let w0 = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let y = app.spawn_y + 6;
        let mut w = app.offline_world.as_mut().unwrap();
        w.set(w0, y, w0, crate::world::STONE);
        w.set(w0, y, w0 - 1, crate::world::AIR); // the frame cell the air keeps
        drop(w);
        let hit = crate::world::ClipHit {
            pos: (w0, y, w0),
            face: 2,
            location: (w0 as f64 + 0.5, y as f64 + 0.5, w0 as f64),
            inside: false,
        };
        assert_eq!(app.on_right_press_item_frame_arm(&hit), Some(true));
        let fid = app.item_frames[0].id;
        let cell = app.item_frames[0].cell;
        // THE EMPTY ARM (the break voice the 2-arg dropItem plays, the
        // frame item the survival drop speaks)
        app.offline_held = None;
        app.attack_item_frame(fid);
        assert!(app.item_frames.is_empty(), "the punch the empty frame kills");
        assert_eq!(
            app.offline_world.as_ref().unwrap().frame_analog.get(&cell),
            None,
            "the death the analog row the map drops"
        );
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("entity.item_frame.break ")),
            "the break voice the super's kill speaks"
        );
        assert_eq!(
            app.drops
                .iter()
                .filter(|d| mc_registry::item_name(d.item) == Some("item_frame"))
                .count(),
            1,
            "the frame item the survival drop speaks"
        );
        // THE FILLED ARM (the frame item + the contained item the pair
        // spawns, the remove_item voice the arm plays)
        let before = app.drops.len();
        assert_eq!(app.on_right_press_item_frame_arm(&hit), Some(true));
        let fid2 = app.item_frames[0].id;
        let ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
        app.inventory.items[app.selected_hotbar] = (ingot, 2);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(app.on_right_press_item_frame_interact(fid2), "the store the press consumes");
        app.attack_item_frame(fid2);
        assert!(app.item_frames.is_empty(), "the punch the filled frame kills");
        let new_drops = &app.drops[before..];
        assert!(
            new_drops.iter().any(|d| mc_registry::item_name(d.item) == Some("item_frame")),
            "the frame item the drop pair speaks"
        );
        assert!(
            new_drops.iter().any(|d| mc_registry::item_name(d.item) == Some("iron_ingot")),
            "the contained item the drop pair speaks"
        );
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("entity.item_frame.remove_item ")),
            "the remove_item voice the filled arm speaks"
        );
    }


    /// THE FRAME BANK'S OWN ROUNDTRIP (the autopush-136 round — the
    /// "item_frames" list the bank walk writes, the parse walk the
    /// same rows reads: the BTree cell order the deterministic bytes
    /// keep, the id the session's counter owns [the parse the zero
    /// re-mints], the poll clock the fresh walk re-mints, the empty
    /// frame the OPTIONAL empty compound wears).
    #[test]
    fn the_item_frames_bank_parse_roundtrip_wears() {
        let stick = mc_registry::item_id_by_name("stick").unwrap();
        let frames = vec![
            ItemFrameEnt {
                id: 7,
                cell: (3, 64, -7),
                facing: 2,
                rotation: 7,
                item: (stick, 1),
                glow: true,
                check_interval: 55,
            },
            ItemFrameEnt {
                id: 9,
                cell: (-2, 70, 11),
                facing: 5,
                rotation: 0,
                item: (0, 0),
                glow: false,
                check_interval: 0,
            },
        ];
        let bank = item_frames_bank_to_nbt(&frames);
        let root = mc_protocol::nbt::Nbt::Compound(vec![("item_frames".to_string(), bank)]);
        let parsed = item_frames_from_nbt(&root);
        assert_eq!(parsed.len(), 2, "the two frames the bank keeps");
        // THE BTREE ORDER (the cell sort the deterministic bytes keep —
        // the -2,70,11 row the smaller x the first slot wears)
        assert_eq!(parsed[0].cell, (-2, 70, 11));
        assert_eq!(parsed[1].cell, (3, 64, -7));
        // THE EMPTY FRAME'S OWN ROW (the OPTIONAL empty compound the
        // parse the (0,0) stack wears)
        assert_eq!(parsed[0].facing, 5);
        assert_eq!(parsed[0].rotation, 0);
        assert!(!parsed[0].glow);
        assert_eq!(
            parsed[0].item,
            (0, 0),
            "the empty frame the OPTIONAL empty compound keeps"
        );
        // THE FILLED GLOW'S OWN ROW
        assert_eq!(parsed[1].facing, 2);
        assert_eq!(parsed[1].rotation, 7);
        assert!(parsed[1].glow);
        assert_eq!(parsed[1].item, (stick, 1), "the count-1 the bank wears");
        // THE SESSION'S OWN COUNTERS (the id + the poll clock the boot
        // walk re-mints — the file the cell identity banks)
        assert!(
            parsed.iter().all(|f| f.id == 0),
            "the id the session's counter owns"
        );
        assert!(
            parsed.iter().all(|f| f.check_interval == 0),
            "the poll clock the fresh walk re-mints"
        );
    }


    /// THE FRAME PARSE'S OWN TOLERANCE (the same walk the store parse
    /// wears — the missing coordinate the entry skip keeps, the
    /// out-of-band facing the mayPlace law skips, the unknown item
    /// name the empty frame keeps [never a guessed id], the rotation
    /// the setRotation's %8 wrap banks, the absent glow the false
    /// wears, the absent count the fallback-1 keeps, the missing list
    /// the honest empties keep).
    #[test]
    fn the_item_frames_parse_tolerant_the_corrupt_rows_skip() {
        let stick = mc_registry::item_id_by_name("stick").unwrap();
        let kv = |k: &str, v: mc_protocol::nbt::Nbt| (k.to_string(), v);
        let xyz = || {
            vec![
                kv("x", mc_protocol::nbt::Nbt::Int(4)),
                kv("y", mc_protocol::nbt::Nbt::Int(65)),
                kv("z", mc_protocol::nbt::Nbt::Int(-3)),
            ]
        };
        let root = mc_protocol::nbt::Nbt::Compound(vec![kv(
            "item_frames",
            mc_protocol::nbt::Nbt::List(vec![
                // THE MISSING COORDINATE the entry skip keeps
                mc_protocol::nbt::Nbt::Compound(vec![kv(
                    "facing",
                    mc_protocol::nbt::Nbt::Int(2),
                )]),
                // THE OUT-OF-BAND FACING the mayPlace law skips (the
                // vertical 0/1 + the nonsense 6 the corrupt rows wear)
                mc_protocol::nbt::Nbt::Compound({
                    let mut f = xyz();
                    f.push(kv("facing", mc_protocol::nbt::Nbt::Int(0)));
                    f
                }),
                mc_protocol::nbt::Nbt::Compound({
                    let mut f = xyz();
                    f.push(kv("facing", mc_protocol::nbt::Nbt::Int(6)));
                    f
                }),
                // THE UNKNOWN ITEM NAME the empty frame keeps — never
                // a guessed id the parse invents
                mc_protocol::nbt::Nbt::Compound({
                    let mut f = xyz();
                    f.push(kv("facing", mc_protocol::nbt::Nbt::Int(2)));
                    f.push(kv(
                        "Item",
                        mc_protocol::nbt::Nbt::Compound(vec![
                            kv(
                                "id",
                                mc_protocol::nbt::Nbt::String("minecraft:no_such_thing".into()),
                            ),
                            kv("count", mc_protocol::nbt::Nbt::Int(1)),
                        ]),
                    ));
                    f
                }),
                // THE WRAP ROW (the 9 the %8 wears) + THE ABSENT COUNT
                // the fallback-1 keeps + THE ABSENT GLOW the false
                mc_protocol::nbt::Nbt::Compound({
                    let mut f = xyz();
                    f.push(kv("facing", mc_protocol::nbt::Nbt::Int(3)));
                    f.push(kv("rotation", mc_protocol::nbt::Nbt::Int(9)));
                    f.push(kv(
                        "Item",
                        mc_protocol::nbt::Nbt::Compound(vec![kv(
                            "id",
                            mc_protocol::nbt::Nbt::String("minecraft:stick".into()),
                        )]),
                    ));
                    f
                }),
                // THE NEGATIVE ROTATION the euclid wrap keeps
                mc_protocol::nbt::Nbt::Compound({
                    let mut f = xyz();
                    f.push(kv("facing", mc_protocol::nbt::Nbt::Int(4)));
                    f.push(kv("rotation", mc_protocol::nbt::Nbt::Int(-1)));
                    f
                }),
            ]),
        )]);
        let parsed = item_frames_from_nbt(&root);
        assert_eq!(
            parsed.len(),
            3,
            "the coord-less + the two out-of-band rows the skips keep"
        );
        assert_eq!(parsed[0].item, (0, 0), "the unknown name the empty frame keeps");
        assert_eq!(parsed[1].rotation, 1, "the 9 the %8 wrap banks");
        assert_eq!(parsed[1].item, (stick, 1), "the absent count the 1 keeps");
        assert!(!parsed[1].glow, "the absent glow the false wears");
        assert_eq!(parsed[1].facing, 3);
        assert_eq!(parsed[2].rotation, 7, "the -1 the euclid wrap banks");
        // THE MISSING LIST the honest empties keep + the wrong tag too
        assert!(item_frames_from_nbt(&mc_protocol::nbt::Nbt::Int(1)).is_empty());
        assert!(item_frames_from_nbt(&mc_protocol::nbt::Nbt::Compound(vec![])).is_empty());
    }


    /// THE FRAME'S OWN PERSISTENCE ARC (the autopush-136 round — the
    /// placement/store/rotate lanes' save rides the containers file
    /// banks, the fresh app's load-once the frames re-mints [the
    /// analog rows the same parse banks, the ids the counter
    /// re-mints], the second life's own turn the file re-banks, the
    /// punch the file's row drops, the restart the empty truth keeps
    /// — the RESTART the frames OUTLIVE).
    #[test]
    fn the_item_frame_the_persistence_arc_the_restart_outlives() {
        let dir = std::env::temp_dir().join(format!("mc2rs-framepersist-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("the temp dir");
        let file = dir.join("frames-arc.nbt");
        let _ = std::fs::remove_file(&file);
        // THE SHARED CELLS (the same seed the same wall wears — the
        // support the collision-bearing kind, the frame cell the air)
        let (w0, y) = {
            let mut p = App::new(3);
            p.enter_world();
            let w0 = p.offline_world.as_ref().unwrap().size_blocks() / 2;
            (w0, p.spawn_y + 6)
        };
        let set_cells = |app: &mut App| {
            let mut w = app.offline_world.as_mut().unwrap();
            w.set(w0, y, w0, crate::world::STONE);
            w.set(w0, y, w0 - 1, crate::world::AIR);
            drop(w);
        };

        // THE FIRST LIFE: the place + the store + the rotate, the
        // save rides the file banks at every lane
        let mut app = App::new(3);
        app.enter_world();
        app.container_stores_file = Some(file.clone());
        set_cells(&mut app);
        let hit = crate::world::ClipHit {
            pos: (w0, y, w0),
            face: 2,
            location: (w0 as f64 + 0.5, y as f64 + 0.5, w0 as f64),
            inside: false,
        };
        assert_eq!(app.on_right_press_item_frame_arm(&hit), Some(true));
        assert!(file.exists(), "the placement lane the save ride banks");
        let ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
        app.inventory.items[app.selected_hotbar] = (ingot, 5);
        app.hotbar_slots = app.inventory.hotbar();
        let fid = app.item_frames[0].id;
        assert!(app.on_right_press_item_frame_interact(fid), "the store lane");
        app.inventory.items[app.selected_hotbar] = (0, 0);
        app.hotbar_slots = app.inventory.hotbar();
        app.offline_held = None;
        assert!(app.on_right_press_item_frame_interact(fid), "the rotate lane");
        assert_eq!(app.item_frames[0].rotation, 1, "the turn the +1 banks");
        assert_eq!(
            app.offline_world
                .as_ref()
                .unwrap()
                .frame_analog
                .get(&(w0, y, w0 - 1)),
            Some(&2),
            "the filled frame's analog the rotation+1 wears"
        );

        // THE RESTART: the fresh app the load-once the frame re-mints
        let mut born = App::new(3);
        born.enter_world();
        born.container_stores_file = Some(file.clone());
        assert!(born.item_frames.is_empty(), "the fresh app the empty list keeps");
        born.container_stores_load_once();
        assert_eq!(born.item_frames.len(), 1, "the boot parse the frame re-mints");
        let f = born.item_frames[0].clone();
        assert_eq!(f.cell, (w0, y, w0 - 1), "the cell the file keeps");
        assert_eq!(f.facing, 2, "the facing the file keeps");
        assert_eq!(f.rotation, 1, "the turn the file banks");
        assert_eq!(f.item, (ingot, 1), "the count-1 the file wears");
        assert!(!f.glow, "the plain frame the glow-less row keeps");
        assert_ne!(f.id, 0, "the id the counter re-mints");
        assert_eq!(
            born.offline_world
                .as_ref()
                .unwrap()
                .frame_analog
                .get(&(w0, y, w0 - 1)),
            Some(&2),
            "the analog row the boot parse banks"
        );

        // THE SECOND LIFE'S OWN TURN: the rotate the file re-banks
        let bid = born.item_frames[0].id;
        assert!(
            born.on_right_press_item_frame_interact(bid),
            "the rotate lane the second life rides"
        );
        assert_eq!(born.item_frames[0].rotation, 2, "the second turn the +1 banks");

        // THE THIRD LIFE: the turn the file wears
        let mut aged = App::new(3);
        aged.enter_world();
        aged.container_stores_file = Some(file.clone());
        aged.container_stores_load_once();
        assert_eq!(
            aged.item_frames[0].rotation, 2,
            "the second turn the file wears"
        );
        assert_eq!(
            aged.offline_world
                .as_ref()
                .unwrap()
                .frame_analog
                .get(&(w0, y, w0 - 1)),
            Some(&3),
            "the analog the +1 the re-bank wears"
        );

        // THE PUNCH THE FILE'S ROW DROPS (the removal the save ride
        // banks, the restart the empty truth keeps)
        let aid = aged.item_frames[0].id;
        aged.attack_item_frame(aid);
        assert!(aged.item_frames.is_empty(), "the punch the frame kills");
        let mut last = App::new(3);
        last.enter_world();
        last.container_stores_file = Some(file.clone());
        last.container_stores_load_once();
        assert!(
            last.item_frames.is_empty(),
            "the punch the file's row drops"
        );
        assert!(
            last.offline_world
                .as_ref()
                .unwrap()
                .frame_analog
                .get(&(w0, y, w0 - 1))
                .is_none(),
            "the analog row the punch's own file ride drops"
        );
        let _ = std::fs::remove_file(&file);
    }


    /// THE ITEM FRAME'S OWN SURVIVE POLL (javap HangingEntity.tick the
    /// 26.2 jar — the 100-tick clock the survives verdict rides, the
    /// support's own loss the pop walk serves: the break voice plays,
    /// the frame item + the contained item the pair spawns, the analog
    /// row the map drops) AND THE COMPARATOR'S OWN LIVE CHAIN (the
    /// store lane's broadcast the +2 schedule banks, the tick walk the
    /// output cache the through-wall read re-banks — the comparator
    /// behind the wall the framed item's fill speaks).
    #[test]
    fn the_item_frame_the_survive_poll_and_the_comparator_chain_wear() {
        let mut app = App::new(3);
        app.enter_world();
        let w0 = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let y = app.spawn_y + 6;
        // THE WALL + THE COMPARATOR (the comparator's back cell = the
        // wall, the second cell = the frame cell — the through-wall
        // read the classic setup keeps)
        let mut w = app.offline_world.as_mut().unwrap();
        w.set(w0, y, w0, crate::world::STONE); // the wall
        w.set(w0 + 1, y, w0, crate::world::AIR); // the frame cell the air keeps
        w.set(w0 - 1, y, w0, crate::world::COMPARATOR); // behind the wall
        w.set_variant(w0 - 1, y, w0, crate::world::comparator_variant(1, false, false, 0));
        drop(w);
        // facing 1: the back = +x — the wall the back cell wears, the
        // second cell the wall's east side the frame answers
        let hit = crate::world::ClipHit {
            pos: (w0, y, w0),
            face: 5, // EAST — the frame cell the wall's east side
            location: (w0 as f64 + 1.0, y as f64 + 0.5, w0 as f64 + 0.5),
            inside: false,
        };
        assert_eq!(app.on_right_press_item_frame_arm(&hit), Some(true));
        let fid = app.item_frames[0].id;
        assert_eq!(app.item_frames[0].cell, (w0 + 1, y, w0), "the cell the east side keeps");
        // THE STORE (the analog row the 1 banks — the broadcast the
        // +2 schedule the comparator's re-read arms)
        let ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
        app.inventory.items[app.selected_hotbar] = (ingot, 1);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(app.on_right_press_item_frame_interact(fid), "the store the press consumes");
        // THE CHAIN (the +2 tick the comparator the refresh banks —
        // the output cache the through-wall read the 1 speaks)
        app.update();
        app.update();
        let variant = app.offline_world.as_ref().unwrap().variant(w0 - 1, y, w0);
        assert_eq!(
            crate::world::comparator_output_of(variant),
            1,
            "the comparator the framed item's own fill the output speaks"
        );
        // THE SURVIVE POLL (the support dug the pop the 100-tick clock
        // serves — the break voice + the frame item's own drop the
        // pair wears; the DIG'S OWN BROADCAST the break walk's
        // attach_wire_voice ride serves — the through-wall arm the
        // missing conductor kills, the comparator's re-read the dig's
        // own consult arms BEFORE the pop lands)
        let before = app.drops.len();
        app.offline_world
            .as_mut()
            .unwrap()
            .set(w0, y, w0, crate::world::AIR);
        app.attach_wire_voice(w0, y, w0);
        for _ in 0..100 {
            app.update();
        }
        assert!(app.item_frames.is_empty(), "the dug support the poll pops");
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("entity.item_frame.break ")),
            "the break voice the pop walk speaks"
        );
        assert!(
            app.drops[before..]
                .iter()
                .any(|d| mc_registry::item_name(d.item) == Some("item_frame")),
            "the frame item the pop's own drop speaks"
        );
        assert!(
            app.drops[before..]
                .iter()
                .any(|d| mc_registry::item_name(d.item) == Some("iron_ingot")),
            "the contained item the pop's own drop speaks"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().frame_analog.get(&(w0 + 1, y, w0)),
            None,
            "the pop the analog row the map drops"
        );
        // THE COMPARATOR'S OWN TAIL (the dig's own broadcast the
        // re-read arms — the through-wall arm the missing conductor
        // kills, the output cache the 0 the refresh banks)
        let variant = app.offline_world.as_ref().unwrap().variant(w0 - 1, y, w0);
        assert_eq!(
            crate::world::comparator_output_of(variant),
            0,
            "the dug wall the comparator's own output the refresh banks"
        );
    }


    #[test]
    fn the_boat_use_walk_refuses_the_eye_inside_bodies() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/oak_boat"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let pool_y = app.spawn_y + 6;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, pool_y, c, crate::world::WATER);
        let eye_y = (pool_y + 4) as f64;
        // the wire hull parked around the eye: its 1.375 x 0.562 box
        // contains the eye position (the boat family the
        // AbstractBoat.isPickable law picks)
        app.net_boats.push(crate::world_view::WireBoat {
            id: 77,
            x: c as f64 + 0.5,
            y: eye_y - 0.3,
            z: c as f64 + 0.5,
            yaw: 0.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        });
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE ENTITY LANE'S OWN OUTRANK (the start-inside d²=0 the
        // pick speaks): the interact the press consumes — the WIRE
        // arm's send the verdict carries, no local hull spawned, the
        // local ride prediction the wire body never runs
        assert!(
            app.on_right_press(0, 0),
            "the eye inside the hull — the interact the start-inside law consumes"
        );
        assert!(app.offline_boats.is_empty(), "no hull spawned");
        assert!(
            app.offline_ride.is_none(),
            "the wire's own lane the client never predicts"
        );
        // THE MOB WALK: the same gate the living bodies speak (the
        // LivingEntity.isPickable law's own family) — the mob arm's
        // Pass the ITEM lane falls through to, the eye gate the boat's
        // use walk keeps (the PASS the press never consumes)
        app.net_boats.clear();
        app.mobs.push(crate::mob::Zombie::new(
            88,
            c as f64 + 0.5,
            eye_y - 0.5,
            c as f64 + 0.5,
            7,
        ));
        assert!(!app.on_right_press(0, 0), "the eye inside the mob — the PASS");
        assert!(app.offline_boats.is_empty(), "no hull spawned");
    }


    /// THE BOAT'S OWN USE WALK — the NO-COLLISION GATE (the
    /// level.noCollision walk's own verdict): the spawned box the
    /// fluids never block, the solids the FAIL speaks — the press
    /// consumes (the vanilla's own FAIL), the water stays hull-less.
    #[test]
    fn the_boat_use_walk_speaks_the_no_collision_law() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/oak_boat"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let pool_y = app.spawn_y + 6;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, pool_y, c, crate::world::WATER);
        // the stone INSIDE the spawned box's span (the box's x cells
        // c-1..c+1 at the surface level the hull's own body rides)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c + 1, pool_y + 1, c, crate::world::STONE);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, (pool_y + 4) as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the FAIL consumes the press");
        assert!(app.offline_boats.is_empty(), "the FAIL never spawns");
    }


    /// THE BOAT'S OWN USE WALK — the NETWORKED arc (the vanilla's own
    /// isClientSide guard): the client predicts NOTHING — the UseItem
    /// echo rides the command channel (the server's own use() the
    /// spawn rides, the Add Entity the hull lands through), the local
    /// hull lists stay empty, the store's stack the server's own echo
    /// updates.
    #[test]
    fn the_boat_use_walk_echoes_the_networked_lane() {
        use crate::world_view::ContainerCommand;
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        let mut net = air_net_world(2, 0);
        let c = net.size_blocks() / 2;
        let pool_y = net.size_blocks() / 4;
        net.set(c, pool_y, c, crate::world::WATER);
        app.net_world = Some(net);
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.live_held_updated(Some((wire("oak_boat"), 5)));
        app.net_camera = Some(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, (pool_y + 4) as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the boat use consumed the press");
        match click_rx.try_recv().expect("the use-item echo rode the lane") {
            ContainerCommand::UseItem { yaw, pitch, .. } => {
                // the wire's angles in the vanilla degrees (the
                // conversion's own fold — the float words the tolerance
                // keeps)
                assert!((yaw + 180.0).abs() < 1e-3, "yaw {yaw}");
                assert!((pitch - 90.0).abs() < 1e-3, "pitch {pitch}");
            }
            other => panic!("expected a UseItem echo, got {other:?}"),
        }
        // THE CLIENT PREDICTS NOTHING: the wire's own Add Entity is the
        // authority — no local hull on either lane
        assert!(app.net_boats.is_empty(), "no local hull on the wire lane");
        assert!(app.offline_boats.is_empty(), "no offline hull on the wire lane");
    }


    /// THE OFFLINE HULL'S OWN FLOAT — the AbstractBoat.tick's
    /// authoritative lane the singleplayer keeps: the placed hull's
    /// first ticks speak the splash-down's own dip (the InAir fall the
    /// gravity arms, the arm the water's surface catches at the d6
    /// fold), then the settle the fixed point reaches (the 0.65
    /// submersion fraction the lift's equilibrium speaks). The wire's
    /// own hulls the walk never touches.
    #[test]
    fn the_offline_hull_settles_the_water_line() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/oak_boat"));
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        // the pool high in the air band: every row the friction slab's
        // own ring reaches stays air (the terrain's own tops never
        // reach spawn_y + 7 — the InAir verdict the first tick keeps,
        // the splash-down the arm's InAir gate demands)
        let pool_y = app.spawn_y + 8;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, pool_y, c, crate::world::WATER);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, (pool_y + 4) as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the placed hull at the surface");
        let hull_y0 = app.offline_boats[0].y;
        assert_eq!(hull_y0, (pool_y + 1) as f64, "the surface hit's own spawn");
        // THE FIRST TICKS: the InAir fall the gravity arms (the placed
        // hull sits exactly ON the plane the base band's row misses),
        // then the splash-down the water's own surface catches
        app.tick_offline_hulls();
        let after_fall = app.offline_boats[0].y;
        assert_eq!(
            after_fall,
            hull_y0 - 0.04,
            "the InAir fall's own word"
        );
        app.tick_offline_hulls();
        assert_eq!(
            app.offline_boats[0].y,
            (pool_y + 1) as f64 - crate::hull::HULL_HEIGHT + crate::hull::SPLASH_DOWN,
            "the splash-down's own d6 fold"
        );
        assert_eq!(
            app.offline_boat_phys[&1].status,
            crate::hull::HullStatus::InWater,
            "the arm's own status swap"
        );
        // THE SETTLE: the fixed point the equilibrium speaks — the
        // water level's own plane minus the 0.65 submersion fraction,
        // the delta's own zero the rest keeps
        for _ in 0..400 {
            app.tick_offline_hulls();
        }
        let y_star = (pool_y + 1) as f64 - 0.65 * crate::hull::HULL_HEIGHT;
        let settled = app.offline_boats[0].y;
        assert!(
            (settled - y_star).abs() < 1.0e-4,
            "the settled y {settled} vs the fixed point {y_star}"
        );
        assert!(
            app.offline_boat_phys[&1].delta_y.abs() < 1.0e-4,
            "the rest's own silence"
        );
        // THE WIRE'S OWN AUTHORITY: a snapshot hull the walk never
        // moves (the offline lane's own gate keeps)
        app.net_boats.push(crate::world_view::WireBoat {
            id: 501,
            x: c as f64 + 0.5,
            y: 4.0,
            z: c as f64 + 0.5,
            yaw: 0.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        });
        app.tick_offline_hulls();
        assert_eq!(app.net_boats[0].y, 4.0, "the wire's own word stands");
    }


    /// THE OFFLINE MOUNT — the AbstractBoat.interact's own arc the
    /// right-press lane speaks: the ride belief arms (the single
    /// rider's seat the 0.0 base keeps, the ride height the height/3
    /// fold speaks), the body's yaw the hull's own word snaps, the
    /// mount's overlay message the 60-tick life carries, and the
    /// ALREADY-RIDING gate the startRiding's own false keeps (the
    /// second ask the first belief stands — the wire's hold the same
    /// word keeps).
    #[test]
    fn the_offline_mount_arms_the_ride_belief() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_boats.push(crate::world_view::WireBoat {
            id: 7,
            x: 10.0,
            y: 64.0,
            z: -3.0,
            yaw: 90.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        });
        app.body = Some(crate::body::PlayerBody::new(10.5, 65.0, -2.0, 0.0, 0.0));
        assert!(app.mount_offline_hull(7), "the startRiding's own true");
        let r = app.offline_ride.as_ref().expect("the ride belief armed");
        assert_eq!(r.vehicle_id, 7, "the hull's own id");
        assert_eq!(r.seat_x, 0.0, "the single rider's own seat");
        assert_eq!(
            r.ride_height,
            crate::world_view::BOAT_DIMENSIONS_HEIGHT / 3.0,
            "the ride height the height/3 fold speaks (the f32's own word —
            the snapshot's 0.1875 the wire's stale walk keeps)"
        );
        assert_eq!(
            app.body.as_ref().unwrap().yaw,
            90.0,
            "the fresh mount's own yaw snap"
        );
        assert_eq!(
            app.overlay_message.as_deref(),
            Some("Press Left Shift to Dismount"),
            "the mount's own overlay word"
        );
        // THE ALREADY-RIDING GATE (the startRiding's own false): a
        // second ask the first belief stands — the overlay the mount's
        // own life keeps (the re-arm never re-speaks)
        app.overlay_message_time = 5;
        assert!(!app.mount_offline_hull(7), "the second ask the false keeps");
        assert_eq!(
            app.offline_ride.as_ref().unwrap().vehicle_id,
            7,
            "the first belief stands"
        );
        assert_eq!(app.overlay_message_time, 5, "the re-arm's own silence");
        // THE UNKNOWN ID the honest false keeps (no hull the belief
        // never names)
        app.offline_ride = None;
        assert!(!app.mount_offline_hull(999), "the unknown id's own false");
        // THE WIRE'S OWN BELIEF the same gate stands (the two lanes
        // never ride together)
        app.net_ride = Some(crate::world_view::WireRide {
            vehicle_id: 9,
            feet_x: 0.0,
            feet_y: 0.0,
            feet_z: 0.0,
            vehicle_yaw: 0.0,
            seat_x: 0.0,
            ride_height: 0.1875,
        });
        assert!(!app.mount_offline_hull(7), "the wire's own hold stands");
        assert!(app.offline_ride.is_none(), "the wire's lane the belief keeps");
    }


    /// THE OFFLINE CARRY — the positionRider law the local hull's own
    /// pose keeps: the hull's authoritative tick moves FIRST (the
    /// vehicle-moves-then-passenger order the wire lane speaks — the
    /// call site's own arm), the body's feet the hull's FRESH pose +
    /// the attachment ride (the pre-move word the stale walk never
    /// speaks), the prev trio the render's trail anchors, the body's
    /// physics sleep (no impulses, no gravity — the rideTick's own
    /// silence), and the SHIFT EDGE's drop hands the body back to
    /// itself (the physics the next tick's walk resumes).
    #[test]
    fn the_offline_carry_follows_the_local_hull() {
        let mut app = App::new(3);
        app.enter_world();
        // THE ATTACHMENT'S OWN WORD (the rideHeight fold the mount
        // speaks — the f32/3 the honest recompute keeps, the 0.1875
        // the wire's SNAPSHOT word never names here)
        let rh = (crate::world_view::BOAT_DIMENSIONS_HEIGHT / 3.0) as f64;
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let hull_y0 = (app.spawn_y + 8) as f64;
        app.offline_boats.push(crate::world_view::WireBoat {
            id: 7,
            x: c as f64 + 0.5,
            y: hull_y0,
            z: c as f64 + 0.5,
            yaw: 0.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        });
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            hull_y0 + rh,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        app.offline_ride = Some(crate::world_view::OfflineRide {
            vehicle_id: 7,
            seat_x: 0.0,
            ride_height: rh as f32,
        });
        // THE FIRST UPDATE: the hull's own fall the gravity arms (the
        // spawn column's air band the verdict keeps — the -0.04 the
        // InAir word speaks), then the CARRY the fresh pose rides
        app.update();
        let hull_y1 = app.offline_boats[0].y;
        assert_eq!(hull_y1, hull_y0 - 0.04, "the hull's own fall first");
        let b = app.body.as_ref().unwrap();
        assert!(
            (b.y - (hull_y0 - 0.04 + rh)).abs() < 1.0e-9,
            "the carried feet the hull's FRESH pose keeps (b.y {} vs want {})",
            b.y,
            hull_y0 - 0.04 + rh
        );
        assert_eq!(b.x, c as f64 + 0.5, "the seat's cos the zero yaw keeps");
        assert_eq!(b.z, c as f64 + 0.5, "the seat's sin the zero yaw silences");
        assert_eq!(
            (b.prev_x, b.prev_y, b.prev_z),
            (c as f64 + 0.5, hull_y0 + rh, c as f64 + 0.5),
            "the trail the render's partial lerp anchors"
        );
        assert_eq!(b.vx, 0.0, "the body's own sleep");
        assert_eq!(b.vy, 0.0, "the gravity the ride never arms");
        assert!(!b.on_ground, "the ground never claims the rider");
        // THE SHIFT EDGE (the dismount's own ask — the
        // onMovementInput walk the offline lane speaks locally): the
        // ride drops, the body the carried pose keeps
        app.on_movement_input(0.0, 0.0, false, true, false);
        assert!(app.offline_ride.is_none(), "the edge's own drop");
        let carried_y = app.body.as_ref().unwrap().y;
        // THE PHYSICS RESUME (the rideTick's silence ends): the
        // FIRST tick's own stillness the move-before-gravity order
        // speaks (the ride's sleep handed a zero velocity — the
        // gravity the fold applies AFTER the collide), the SECOND
        // tick's walk the fall speaks
        app.update();
        let b1 = app.body.as_ref().unwrap();
        assert_eq!(
            b1.y, carried_y,
            "the first tick's own stillness the zero velocity keeps"
        );
        app.update();
        let b2 = app.body.as_ref().unwrap();
        assert!(
            b2.y < carried_y,
            "the body's own gravity the drop resumes ({} -> {})",
            carried_y,
            b2.y
        );
    }


    /// THE HULL PICK'S OWN MOUNT ARC — the right-press lane's own
    /// walk: the hull the crosshair holds (the box the pick clips, the
    /// block clip's own distance term the hull-behind-the-wall refusal
    /// speaks) the OFFLINE MOUNT arms (the press the CONSUME verdict
    /// consumes), the riding hull the second press the startRiding's
    /// own false keeps (the entity PASS the BLOCK case never reaches —
    /// the empty hand the item lane's silence keeps).
    #[test]
    fn the_right_press_mounts_the_nearest_hull() {
        let mut app = App::new(3);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let sy = app.spawn_y;
        app.offline_boats.push(crate::world_view::WireBoat {
            id: 7,
            x: c as f64 + 0.5,
            y: (sy + 2) as f64,
            z: c as f64 + 0.5,
            yaw: 90.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        });
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            (sy + 3) as f64 + 0.5,
            c as f64 + 0.5,
            0.0,
            90.0,
        ));
        // THE MOUNT ARC: the hull top's hit the reach keeps (the eye
        // — the feet + the standing eye's 1.62 — rides 2.56 above the
        // box top, the 3.0 the strict law speaks; the terrain top's
        // plane the block term keeps 4.1 away — the hull the nearer
        // word speaks), the press the CONSUME verdict consumes. THE
        // BODY'S OWN DEGREES (the vanilla pitch the turn walk speaks —
        // the 90 the straight-down aim keeps)
        assert!(app.on_right_press(0, 0), "the mount the press consumes");
        assert_eq!(
            app.offline_ride.as_ref().expect("the ride armed").vehicle_id,
            7,
            "the hull's own id the press arms"
        );
        assert_eq!(
            app.body.as_ref().unwrap().yaw,
            90.0,
            "the fresh mount's own yaw snap"
        );
        // THE SECOND PRESS (the startRiding's own false): the entity
        // PASS the BLOCK case never reaches — the empty hand the item
        // lane's silence keeps, the press the honest unconsumed keeps,
        // the ride the first belief stands
        assert!(
            !app.on_right_press(0, 0),
            "the riding hull's second ask the PASS keeps"
        );
        assert_eq!(
            app.offline_ride.as_ref().unwrap().vehicle_id,
            7,
            "the ride the false keeps"
        );
    }


    /// THE HULL PICK'S OWN WALL TERM — the block clip's own distance
    /// the pick's tail speaks: the hull behind the wall the miss keeps
    /// (the block the nearer word speaks), the press the block lane's
    /// own verdict rides (the empty hand the honest PASS keeps), the
    /// ride belief the silence never arms.
    #[test]
    fn the_wall_between_the_pick_refuses_the_hull() {
        let mut app = App::new(3);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let sy = app.spawn_y;
        app.offline_boats.push(crate::world_view::WireBoat {
            id: 7,
            x: c as f64 + 0.5,
            y: (sy + 2) as f64,
            z: c as f64 + 0.5,
            yaw: 90.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        });
        // THE WALL (the solid cell between the eye and the hull top —
        // the block the nearer hit the compare speaks)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, sy + 3, c, crate::world::STONE);
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            (sy + 3) as f64 + 0.5,
            c as f64 + 0.5,
            0.0,
            90.0,
        ));
        assert!(
            !app.on_right_press(0, 0),
            "the wall the hull pick refuses (the empty hand the PASS keeps)"
        );
        assert!(
            app.offline_ride.is_none(),
            "the refused pick the belief never arms"
        );
        assert!(
            app.overlay_message.is_none(),
            "the mount's word the refusal never speaks"
        );
    }


    /// THE WHIRLPOOL'S OWN CARRY — the column's current on the offline
    /// hull (the boat's own bubble law the integrated server's authority
    /// stands in for): the spawn pose the mouth never names, the dip the
    /// second tick's box overlap arms (the 60 the clock keeps, the tail
    /// the −1 counts), the rock the armed word ramps, the 60th riding
    /// tick the zero crossing fires (the eject the DOWN arm speaks, the
    /// dive the −0.7 word drives — the exact word the pure tail test
    /// pins), and the next tick's overlap the fresh 60 re-arms.
    #[test]
    fn the_whirlpool_column_carries_the_offline_hull() {
        let mut app = App::new(3);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let pool_y = app.spawn_y + 8;
        stamp_offline_column(&mut app, c, pool_y, 1);
        app.offline_held_updated(Some("item/oak_boat"));
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, (pool_y + 4) as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the placed hull at the surface");
        assert!(app.mount_offline_hull(1), "the rider aboard");
        // THE ARM'S OWN BEAT (the spawn pose the static scan's air band
        // keeps — the box's bottom edge the FIRST tick's fall crosses:
        // the swept pass the crossing speaks, the mouth the 60 arms,
        // the tail the first count walks — the vanilla post-move walk
        // the same word keeps)
        app.tick_offline_hulls();
        assert_eq!(
            app.offline_bubbles[&1].bubble_time, 59,
            "the swept crossing's first count"
        );
        let rock = app.bubble_state[&1];
        assert!(
            (rock.multiplier - 0.05).abs() < 1.0e-6,
            "the rock the armed word ramps: {}",
            rock.multiplier
        );
        // THE RIDE'S OWN COUNT (58 more ticks the count walks — the
        // eject the countdown holds off)
        for _ in 0..58 {
            app.tick_offline_hulls();
            assert!(app.offline_ride.is_some(), "the ride the countdown holds");
        }
        assert_eq!(
            app.offline_bubbles[&1].bubble_time, 1,
            "the last silent count"
        );
        app.tick_offline_hulls(); // THE FIRE
        assert!(
            app.offline_ride.is_none(),
            "the eject the DOWN arm speaks"
        );
        let phys = &app.offline_boat_phys[&1];
        assert!(phys.delta_y < 0.0, "the dive the launch drives: {}", phys.delta_y);
        // THE MOUTH'S OWN RE-ARM (the next tick's overlap the fresh 60
        // arms — the cycle the riding keeps)
        app.tick_offline_hulls();
        assert_eq!(app.offline_bubbles[&1].bubble_time, 59, "the re-armed clock");
    }


    /// THE UPWARD COLUMN'S OWN LAUNCH — the fire the y REPLACES (the
    /// empty hull's 0.6, the ridden hull's 2.7 the player predicate
    /// speaks), the eject the UP arm never speaks, the next tick's arc
    /// the launch drives, and the 1/100 splash the mouth's overlap
    /// speaks (the voice the gate's hit carries — the fixed seed's
    /// stream the drive replays deterministically).
    #[test]
    fn the_upward_column_launches_the_offline_hull() {
        let mut app = App::new(3);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let pool_y = app.spawn_y + 8;
        stamp_offline_column(&mut app, c, pool_y, 2);
        app.offline_held_updated(Some("item/oak_boat"));
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, (pool_y + 4) as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the placed hull at the surface");
        app.tick_offline_hulls(); // the fall — the swept crossing the mouth arms
        for _ in 0..58 {
            app.tick_offline_hulls();
        }
        assert_eq!(app.offline_bubbles[&1].bubble_time, 1, "the last silent count");
        app.tick_offline_hulls(); // THE FIRE
        assert_eq!(
            app.offline_boat_phys[&1].delta_y, 0.6,
            "the REPLACE word the empty launch keeps"
        );
        // THE RISE (the next tick's move the launch's own word walks —
        // the InWater lift's 0.75 damping the arc still climbs through)
        let y_pre = app.offline_boats[0].y;
        app.tick_offline_hulls();
        let dy = app.offline_boats[0].y - y_pre;
        assert!(dy > 0.3, "the arc the launch drives: {dy}");
        // THE 1/100 SPLASH (the mouth's overlap the long drive keeps —
        // the GENERIC_SPLASH voice the gate's hit speaks)
        for _ in 0..600 {
            app.tick_offline_hulls();
        }
        let splashes = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.generic.splash @"))
            .count();
        assert!(splashes >= 1, "the splash voice the gate speaks: {splashes}");

        // THE RIDER'S OWN 2.7 (the predicate's true the ridden launch
        // speaks — the fire the rider's own word carries, the eject the
        // UP arm never speaks)
        let mut app2 = App::new(3);
        app2.enter_world();
        let c2 = app2.offline_world.as_ref().unwrap().size_blocks() / 2;
        let pool_y2 = app2.spawn_y + 8;
        stamp_offline_column(&mut app2, c2, pool_y2, 2);
        app2.offline_held_updated(Some("item/oak_boat"));
        app2.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c2 as f32 + 0.5, (pool_y2 + 4) as f32, c2 as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app2.on_right_press(0, 0), "the placed hull at the surface");
        assert!(app2.mount_offline_hull(1), "the rider aboard");
        app2.tick_offline_hulls(); // the fall — the swept crossing the mouth arms
        for _ in 0..58 {
            app2.tick_offline_hulls();
        }
        app2.tick_offline_hulls(); // THE FIRE
        assert_eq!(
            app2.offline_boat_phys[&1].delta_y, 2.7,
            "the REPLACE word the ridden launch keeps"
        );
        assert!(
            app2.offline_ride.is_some(),
            "the rider the UP launch carries"
        );
    }


    /// THE PER-VISIT ROLL SHAPE (the offline lane's own count — the
    /// vanilla visit law the batch simplification retired): the 2×2
    /// column's four mouths the static pass rolls PER CELL (the wire
    /// lane's two-pass loop the twin shape speaks), the settled rest
    /// the sweep's empty walk keeps (the zero displacement the
    /// degenerate silence returns), the reference stream the same
    /// four draws the position proves, and the four-rolls-per-tick
    /// drive the 1/100 the splash voice speaks.
    #[test]
    fn the_offline_mouths_roll_the_per_visit_shape() {
        let mut app = App::new(3);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let pool_y = app.spawn_y + 8;
        // THE 2×2 STAMP (the four mouths the box's own span overlaps —
        // the variant layer the four cells carry, the above AIR the
        // mouth's word keeps)
        {
            let w = app.offline_world.as_mut().unwrap();
            for dx in 0..2 {
                for dz in 0..2 {
                    w.set(c + dx, pool_y, c + dz, crate::world::WATER);
                }
            }
            w.variants = Some(vec![vec![0u8; 16 * 48 * 16]; WORLD_CHUNKS * WORLD_CHUNKS]);
            for dx in 0..2 {
                for dz in 0..2 {
                    let (wx, wz) = (c + dx, c + dz);
                    let vi = ((wx % 16) + (wz % 16) * 16) as usize
                        + (pool_y as usize) * (16 * 16);
                    w.variants.as_mut().unwrap()
                        [((wx / 16) + (wz / 16) * WORLD_CHUNKS as i32) as usize][vi] = 1;
                }
            }
        }
        let settled_y = (pool_y + 1) as f64 - 0.65 * crate::hull::HULL_HEIGHT;
        app.offline_boats.push(crate::world_view::WireBoat {
            id: 1,
            x: c as f64 + 1.0,
            y: settled_y,
            z: c as f64 + 1.0,
            yaw: 0.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        });
        // THE SETTLE (the first tick the float's own line the y
        // converges to, the second the bob the buoyancy keeps — the
        // tiny oscillation the displacement speaks, the sweep the
        // footprint's own walk returns: the FOUR mouths both passes
        // visit, the eight rolls the tick speaks)
        app.tick_offline_hulls();
        let floated_y = app.offline_boats[0].y;
        app.tick_offline_hulls();
        assert!(
            (app.offline_boats[0].y - floated_y).abs() < 0.05,
            "the float's own bob the settle keeps: {}",
            app.offline_boats[0].y - floated_y
        );
        assert_eq!(app.offline_boats[0].x, c as f64 + 1.0, "the rest the x keeps");
        assert_eq!(app.offline_boats[0].z, c as f64 + 1.0, "the rest the z keeps");
        // THE PROBE (the four mouths' per-visit stream — the clone the
        // reference replays, the EIGHT rolls the two passes speak —
        // four the static scan visits, four the swept footprint's own
        // walk yields; the hit the arm's own three draws the replay
        // mirrors, the position the alignment proves)
        let mut reference = app.mob_rng.clone();
        for _ in 0..8 {
            if reference.next_int(100) == 0 {
                let _ = reference.next_f32();
                let _ = reference.next_f32();
                let _ = reference.next_f32();
            }
        }
        app.tick_offline_hulls();
        assert_eq!(
            app.mob_rng.next_int(100),
            reference.next_int(100),
            "the eight per-visit rolls' stream position"
        );
        // THE SPLASH THE DRIVE SPEAKS (the four rolls per tick the
        // 1/100 the long drive hits — the batch shape's one roll the
        // count retired)
        for _ in 0..400 {
            app.tick_offline_hulls();
        }
        let splashes = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.generic.splash @"))
            .count();
        assert!(
            splashes >= 1,
            "the four-mouth drive the splash speaks: {splashes}"
        );
    }


    /// THE WIRE MOUTH'S OWN SPLASH — the client arm the remote body's
    /// baseTick speaks (the fields the ServerLevel gate keeps
    /// server-owned, the splash the client's OWN word is): the mouth's
    /// 1/100 the long drive's fixed stream hits, the voice the
    /// GENERIC_SPLASH event carries, the droplet the particle lane
    /// births.
    #[test]
    fn the_wire_mouth_splashes_the_client_arm() {
        let settled_y = (10.0f64 + 1.0) - 0.65 * crate::hull::HULL_HEIGHT;
        let mut app = wire_column_app(settled_y, Some(1));
        for _ in 0..600 {
            app.tick_paddles();
        }
        let splashes = wire_splash_count(&app);
        assert!(
            splashes >= 1,
            "the wire mouth's splash the gate speaks: {splashes}"
        );
        let droplets = app
            .particles
            .iter()
            .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Splash { .. }))
            .count();
        assert!(droplets >= 1, "the droplet the arm births: {droplets}");
    }


    /// THE TWO PASSES' OWN STREAM (the vanilla boat's double
    /// applyEffectsFromBlocks the wire hull speaks — both calls the
    /// fallback Movement(oldPosition, position) build, the fresh set
    /// each pass keeps, the 1/100 the per-visit shape rolls): the
    /// settled mouth's single tick the TWO draws consume (2 passes ×
    /// the one mouth the fixture overlaps), the reference stream the
    /// same two draws the position keeps.
    #[test]
    fn the_wire_mouth_rolls_the_two_passes() {
        let settled_y = (10.0f64 + 1.0) - 0.65 * crate::hull::HULL_HEIGHT;
        let mut app = wire_column_app(settled_y, Some(1));
        app.tick_paddles();
        let mut reference = crate::drops::Lcg::new(0x5EED_10CA);
        let _ = reference.next_int(100); // the first pass's roll
        let _ = reference.next_int(100); // the second pass's roll
        assert_eq!(
            app.mob_rng.next_int(100),
            reference.next_int(100),
            "the two passes' own stream position"
        );
    }


    /// THE WIRE LANE'S OWN SWEPT WALK (the lerp's own displacement the
    /// client's baseTick reads — javap AbstractBoat.tick 119-123: the
    /// InterpolationHandler.interpolate() the tick speaks BETWEEN the
    /// super walk and the passes, the non-authoritative branch the
    /// move() never records, BOTH applyEffectsFromBlocks calls the
    /// fallback Movement(oldPosition, position) build — the lerp's real
    /// displacement the DDA's own branch answers, the resting hull the
    /// degenerate length the static box keeps): the hull easing OFF the
    /// column's mouth the START corner's own visit keeps — the mouth
    /// the static box at the landing pose never overlaps — the 1/100
    /// the two passes' per-visit shape rolls, the stream the reference's
    /// own position proves.
    #[test]
    fn the_wire_lerp_sweeps_the_leaving_mouth() {
        let settled_y = (10.0f64 + 1.0) - 0.65 * crate::hull::HULL_HEIGHT;
        let mut app = wire_column_app(settled_y, Some(1));
        // THE EASE'S OWN ARM (the live_boats_updated walk's shape): the
        // hull's prev AT the column's mouth cell (x 8.5), the sync's
        // word two blocks east (x 10.5), the 3 steps the vanilla
        // lerpSteps speaks
        app.net_boat_lerp.insert(
            700,
            crate::hull::HullLerp {
                prev: (8.5, settled_y, 8.5),
                target: (10.5, settled_y, 8.5),
                steps: 3,
            },
        );
        let mut reference = app.mob_rng.clone();
        // TICK 1: the ease 8.5 -> 9.1667 (the l.prev the pre-tick
        // capture keeps), the swept walk the segment's x layers {8,9}
        // yields — the start corner's (8,10,8) mouth the step-0 visit
        // speaks, the landing box (cell 9) the mouth never overlaps —
        // TWO passes × the one mouth = the TWO draws (the hits the
        // arm's own three draws the replay mirrors)
        for _ in 0..2 {
            if reference.next_int(100) == 0 {
                let _ = reference.next_f32();
                let _ = reference.next_f32();
                let _ = reference.next_f32();
            }
        }
        app.tick_net_boats();
        app.tick_paddles();
        assert_eq!(
            app.mob_rng.next_int(100),
            reference.next_int(100),
            "the leaving mouth's own two draws the swept walk speaks"
        );
        // TICK 2: the ease 9.1667 -> 9.8333 — the start box the cell 8's
        // edge still straddles (8.854 in [8,9)) the mouth the corner
        // set keeps AGAIN (the step-0 visit's own word), two more draws
        for _ in 0..2 {
            if reference.next_int(100) == 0 {
                let _ = reference.next_f32();
                let _ = reference.next_f32();
                let _ = reference.next_f32();
            }
        }
        app.tick_net_boats();
        app.tick_paddles();
        assert_eq!(
            app.mob_rng.next_int(100),
            reference.next_int(100),
            "the straddled departure's own two draws"
        );
        // TICK 3: the ease 9.8333 -> 10.5 — the box the column's cell
        // the departure completes (the start box [9.52,10.146] the cell
        // 9 keeps), the swept walk the zero mouths the silence keeps —
        // the stream the position the silence holds
        app.tick_net_boats();
        app.tick_paddles();
        assert_eq!(
            app.mob_rng.next_int(100),
            reference.next_int(100),
            "the cleared departure the stream's silence keeps"
        );
        // THE EASED POSE'S OWN REST (the exhausted walk the steps keep:
        // the l.prev the pose's own word equals, the segment the
        // degenerate length the static box's branch answers — the
        // settled lane's shape the walk returns to)
        app.tick_net_boats();
        app.tick_paddles();
        assert_eq!(
            app.mob_rng.next_int(100),
            reference.next_int(100),
            "the rest the static branch's silence keeps"
        );
    }


    /// THE UNDER GATE'S OWN SILENCE (the `!isUnderWater` band the
    /// client's own fluid read speaks): the submerged hull's mouth
    /// overlap the 1/100 never draws — the long drive's stream the
    /// silence keeps at zero.
    #[test]
    fn the_under_gate_silences_the_wire_splash() {
        // the box top 10.4625 under the rim's 8/9 surface the under
        // verdict speaks, the box the mouth cell (row 10) overlaps
        let under_y = 9.9f64;
        let mut app = wire_column_app(under_y, Some(1));
        assert!(
            boat_under_water(app.net_world.as_ref().unwrap(), &app.net_boats[0]),
            "the fixture's own under verdict"
        );
        for _ in 0..600 {
            app.tick_paddles();
        }
        assert_eq!(
            wire_splash_count(&app),
            0,
            "the under gate the splash silences"
        );
    }


    /// THE PLAIN POOL'S OWN SILENCE (the variant gate the scan keeps):
    /// the unstamped water cell no column the variant read answers,
    /// the mouths the zero the splash's draw never rides.
    #[test]
    fn the_plain_pool_never_splashes_the_wire_lane() {
        let settled_y = (10.0f64 + 1.0) - 0.65 * crate::hull::HULL_HEIGHT;
        let mut app = wire_column_app(settled_y, None);
        for _ in 0..600 {
            app.tick_paddles();
        }
        assert_eq!(
            wire_splash_count(&app),
            0,
            "the plain pool the variant gate silences"
        );
    }


    /// THE RIDDEN HULL'S OWN STEER — javap controlBoat the offline ride
    /// speaks: the W the 0.04 thrust feeds, the A the −1 fold turns,
    /// the single-turn's 0.005 the band arms, the paddle state the
    /// swapped law sets, the float's tail the decayed words keeps, and
    /// the rider the fresh pose + the drag the positionRider carries.
    /// The placed bystander the `!isVehicle` gate keeps silent.
    #[test]
    fn the_ridden_hull_steers_the_input_word() {
        let mut app = App::new(3);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let pool_y = app.spawn_y + 6;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, pool_y, c, crate::world::WATER);
        let rh = (crate::world_view::BOAT_DIMENSIONS_HEIGHT / 3.0) as f64;
        let settled_y = (pool_y + 1) as f64 - 0.65 * crate::hull::HULL_HEIGHT;
        app.offline_boats.push(crate::world_view::WireBoat {
            id: 7,
            x: c as f64 + 0.5,
            y: settled_y,
            z: c as f64 + 0.5,
            yaw: 0.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        });
        // THE PLACED BYSTANDER (the `!isVehicle` gate the walk keeps —
        // no ride the control never arms)
        app.offline_boats.push(crate::world_view::WireBoat {
            id: 8,
            x: c as f64 + 0.5,
            y: (app.spawn_y + 8) as f64,
            z: c as f64 + 0.5,
            yaw: 0.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        });
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            settled_y + rh,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        assert!(app.mount_offline_hull(7), "the ride armed");
        // THE W RUN (the forward's own 0.04: the decay the zero delta
        // keeps, the thrust the exact word lands, the move the z
        // carries, the rider the fresh pose follows)
        app.on_movement_input(1.0, 0.0, false, false, false);
        app.update();
        assert_eq!(
            app.offline_boats[0].z,
            c as f64 + 0.5 + 0.04f32 as f64,
            "the thrust's first tick the z carries (the f32's own widen)"
        );
        assert_eq!(
            app.offline_boats[0].yaw, 0.0,
            "the straight run the yaw keeps"
        );
        assert!(
            app.offline_boats[0].paddle_left && app.offline_boats[0].paddle_right,
            "the W the both paddles strokes"
        );
        assert_eq!(
            app.body.as_ref().unwrap().z,
            c as f64 + 0.5 + 0.04f32 as f64,
            "the rider the hull's fresh pose carries"
        );
        // THE A TURN (the −1 the fold keeps over the decayed word, the
        // single-turn's 0.005 the band arms — the W's release the
        // decayed thrust keeps, the drag the rider's yaw carries, the
        // swapped paddle the A strokes)
        app.on_movement_input(0.0, 1.0, false, false, false);
        app.update();
        let hull = &app.offline_boats[0];
        assert_eq!(hull.yaw, -1.0, "the left's own −1 the yaw carries");
        assert!(
            !hull.paddle_left && hull.paddle_right,
            "the A the right paddle strokes"
        );
        let f5 = 0.9f32 as f64;
        // the pose the tick 1's own full thrust already carries + the
        // tick 2's decayed word + the single-turn's own cos product
        let want_dz = c as f64 + 0.5 + 0.04f32 as f64
            + 0.04f32 as f64 * f5
            + ((crate::world_model::narrow_cos(-1.0 * crate::hull::HULL_RAD) as f32
                * 0.005f32) as f64);
        assert_eq!(
            hull.z, want_dz,
            "the decayed thrust the turn's own cos carries"
        );
        let want_dx = c as f64 + 0.5
            + ((crate::world_model::narrow_sin(1.0 * crate::hull::HULL_RAD) as f32
                * 0.005f32) as f64);
        assert_eq!(hull.x, want_dx, "the drift the turn's own sin carries");
        assert_eq!(
            app.body.as_ref().unwrap().yaw, -1.0,
            "the drag the rider's yaw carries"
        );
        // THE BYSTANDER (the `!isVehicle` gate the walk keeps: no ride
        // the control never arms — the paddles the silence keeps, the
        // delta the zeros stay, the fold the rotation never turns)
        assert!(
            !app.offline_boats[1].paddle_left && !app.offline_boats[1].paddle_right,
            "the bystander the control never arms"
        );
        assert_eq!(
            app.offline_boat_phys[&8].delta_x + app.offline_boat_phys[&8].delta_z,
            0.0,
            "the bystander the thrust never feeds"
        );
        assert_eq!(
            app.offline_boat_phys[&8].delta_rotation, 0.0,
            "the bystander the fold never turns"
        );
    }


    /// THE DIG LANE's OFFLINE ARC — the left press arms the destroy
    /// (startDestroyBlock), the hold mines on the frame==tick model
    /// (continueDestroyBlock's own beat), the completion breaks the cell
    /// and the mesh re-speaks (the placement's own rhythm), the release
    /// mid-dig aborts, and the air press arms the missTime anti-spam
    /// window the vanilla's survival speaks.
    #[test]
    fn the_dig_lane_mines_the_offline_world() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1; // the top SOLID cell the ray hits
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        let kind_before = app.offline_world.as_ref().unwrap().get(c, surface, c);
        assert_ne!(kind_before, crate::world::AIR, "the surface cell is solid");
        let mesh_before = app.world_quads.as_ref().unwrap().clone();
        // THE PRESS: the dig arms, the swing beats (the vanilla
        // startAttack's own tail)
        app.on_press(0, 0);
        assert!(app.dig.is_destroying, "the press armed the destroy");
        assert_eq!(app.dig.pos, (c, surface, c), "the armed cell");
        assert!(app.dig_held, "the hold flag rides the edge");
        assert!(app.swing.swinging, "the press swung the arm");
        // THE HOLD: the ticks mine (the frame==tick model), the crack
        // stage climbs, the completion breaks the cell
        let mut ticks = 0;
        while app.offline_world.as_ref().unwrap().get(c, surface, c) != crate::world::AIR {
            app.update();
            ticks += 1;
            assert!(ticks < 400, "the dig completed in time");
        }
        assert!(ticks > 1, "the dig took real beats ({ticks})");
        assert!(!app.dig.is_destroying, "the completion disarmed");
        assert_eq!(app.dig.delay, 5, "the completion's cooldown armed");
        assert_ne!(
            *app.world_quads.as_ref().unwrap(),
            mesh_before,
            "the mesh re-spoke with the broken cell (the quad COUNT may tie on a sloped cell — the faces move)"
        );
        // THE SOUND ARC (sound.rs): the hit beats rode the %4 law (the dig
        // ticks 0, 4, 8, ...), the completion spoke the 2001 break — the
        // folds' own numbers ride the entries (THE MIGRATION'S OWN LAW:
        // the cache-absent boot the silent lane keeps — the beat walk
        // below speaks where the cache exists)
        if sounds_loaded() {
            use mc_registry::generated::block_sounds as bs;
            let name = crate::world::NAME_OF[kind_before as usize];
            let idx = bs::sound_index(name).expect("the surface block in the sound table");
            let (hit_ev, break_ev) = (bs::SOUND_HIT[idx], bs::SOUND_BREAK[idx]);
            let hits = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with(&format!("{hit_ev} @")))
                .count();
            assert_eq!(hits, (ticks + 3) / 4, "the beat count for {ticks} dig ticks");
            let hit_entry = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with(&format!("{hit_ev} @")))
                .expect("the first beat rode the FIRST continue tick (dig tick 0)");
            assert!(
                hit_entry.contains(" v0.250000 p0.500000"),
                "the hit fold (v+1)/8, p*0.5: {hit_entry}"
            );
            let break_entry = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with(&format!("{break_ev} @")))
                .expect("the completion's break voice");
            assert!(
                break_entry.contains(" v1.000000 p0.800000"),
                "the break fold (v+1)/2, p*0.8: {break_entry}"
            );
        }
        // THE COOLDOWN + THE COLUMN CONTINUATION (the vanilla dig-down
        // rhythm): the eye drops to reach the column's next cell — the
        // hold ticks the delay away (each cooldown tick still beats the
        // swing), then the retarget arms the next cell through
        // startDestroyBlock (continueDestroyBlock's own tail call)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 2.5, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(app.dig.delay, 0, "the cooldown ticked away on the hold");
        assert!(!app.dig.is_destroying, "the dig stays disarmed through the cooldown");
        app.update();
        assert!(
            app.dig.is_destroying && app.dig.pos == (c, surface - 1, c),
            "the column's next cell armed through the retarget"
        );
        // THE RELEASE MID-DIG: two beats into the new cell, the falling
        // edge aborts (the crack clears, the cell survives)
        app.update();
        app.update();
        app.on_left_release();
        assert!(!app.dig.is_destroying, "the release aborted");
        assert_eq!(app.dig.progress, 0.0, "the crack progress cleared");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface - 1, c),
            crate::world::DIRT,
            "the aborted cell survives the abort"
        );
        // THE AIR MISS: aimed at the sky, the press arms the missTime
        // window (survival's hasMissTime) and the dig never arms
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: std::f32::consts::FRAC_PI_2,
        });
        app.on_press(0, 0);
        assert!(!app.dig.is_destroying, "the air press never armed");
        assert_eq!(app.miss_time, 10, "the anti-spam window armed");
    }


    /// THE BLOCK-SOUND ARC's PLACE VOICE: the right-press placement
    /// speaks the placed block's place sound at BlockItem.place's own
    /// fold — (v+1)/2 volume, p*0.8 pitch — one entry per press, the
    /// dirt's GRAVEL SoundType row naming the event.
    #[test]
    fn the_block_placement_speaks_the_place_voice() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/dirt"));
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::AIR,
            "the placement cell starts empty"
        );
        let before = app.sounds.events.len();
        assert!(app.on_right_press(0, 0), "the placement consumed the press");
        // THE VOICE (the assert computes the expected from the ACTUAL
        // loaded state — the cache-absent boot the silent lane keeps)
        if sounds_loaded() {
            assert_eq!(app.sounds.events.len(), before + 1, "exactly one voice");
            let entry = &app.sounds.events[before];
            assert!(
                entry.starts_with("block.gravel.place @")
                    && entry.contains(" v1.000000 p0.800000"),
                "the place fold on the dirt's GRAVEL row: {entry}"
            );
        } else {
            assert_eq!(
                app.sounds.events.len(),
                before,
                "the empty table the silent lane keeps"
            );
        }
        // THE MISS PLACES NOTHING: the sky press consumes nothing and
        // adds no voice
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: std::f32::consts::FRAC_PI_2,
        });
        let before = app.sounds.events.len();
        assert!(!app.on_right_press(0, 0), "the sky miss never places");
        assert_eq!(app.sounds.events.len(), before, "the miss is silent");
    }


    /// THE DROP LANE'S REWARD ARC: the dirt break spawns its item entity
    /// by the popResource law (the spread, the 10-tick delay), the body
    /// standing in the hole picks it up through the aiStep scan — the
    /// inventory add, the take voice, the hotbar's projection, the
    /// entity's honest discard.
    #[test]
    fn the_drop_lane_rewards_the_dig() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let kind_before = app.offline_world.as_ref().unwrap().get(c, surface, c);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.drops.is_empty(), "the lane starts empty");
        // THE DIG: the hold mines the surface cell to air
        app.on_press(0, 0);
        let mut ticks = 0;
        while app.offline_world.as_ref().unwrap().get(c, surface, c) != crate::world::AIR {
            app.update();
            ticks += 1;
            assert!(ticks < 400, "the dig completed in time");
        }
        // THE DROP: exactly one item entity (the hand-lane row's own
        // single entry), the popResource laws on the fields
        assert_eq!(app.drops.len(), 1, "the break spawned its drop");
        let drop_item = app.drops[0].item;
        // the item id resolves through the generated table's own row
        let name = crate::world::NAME_OF[kind_before as usize];
        let pools = mc_registry::generated::block_drops::drops(name)
            .expect("the broken block's loot row exists");
        let expected = mc_registry::item_id_by_name(pools[0].entries[0].item).unwrap();
        assert_eq!(drop_item, expected, "the drop rides the table's own row");
        assert_eq!(app.drops[0].count, 1);
        // the spawn update's own drop tick already drained one (the dig
        // completion and the entity tick share that update)
        assert_eq!(app.drops[0].pickup_delay, 9, "the popResource delay, one tick spoken");
        assert_eq!(app.drops[0].id, 1, "the offline id counter");
        assert!(
            (app.drops[0].x - (c as f64 + 0.5)).abs() <= 0.25
                && (app.drops[0].z - (c as f64 + 0.5)).abs() <= 0.25,
            "the ±0.25 spread around the block center"
        );
        // THE HOLD ENDS: the release clears the dig (no re-arm while the
        // test's body materializes in the hole)
        app.on_left_release();
        // THE BODY in the hole: the feet on the broken cell's floor — the
        // scan's inflate reaches the resting item
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            surface as f64,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        // THE TICKS: the item's delay drains, the scan speaks the pickup
        for _ in 0..20 {
            app.update();
        }
        assert!(app.drops.is_empty(), "the entity discarded after the take");
        assert_eq!(
            app.inventory.items[0],
            (drop_item, 1),
            "the hotbar's first slot holds the take"
        );
        assert_eq!(app.hotbar_slots[0], (drop_item, 1), "the HUD's projection");
        // THE TAKE'S VOICE (the assert computes the expected from the
        // ACTUAL loaded state — the cache-absent boot the silent lane keeps)
        if sounds_loaded() {
            let pickup_entry = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.item.pickup @"))
                .expect("the take voice spoke");
            assert!(
                pickup_entry.contains(" v0.200000 p"),
                "the pickup fold's own volume: {pickup_entry}"
            );
        }
    }


    /// THE OFFLINE PICKUP'S OWN POP (javap 26.2
    /// `ClientPacketListener.handleContainerSetSlot` pcs 97-148 — the
    /// integrated server's own SetSlot words the singleplayer client
    /// answers: the changed hotbar slot's fresh stack the grow/empty
    /// law pops, `setPopTime(5)` the verdict wears, the
    /// `inventoryTick` decay the game ticks ride): the take the
    /// before/after row diff reads, the grown slot the pop arms, the
    /// untouched slots the pops they had keep, and the merge into a
    /// partial stack the same law re-arms.
    #[test]
    fn the_offline_pickup_the_hotbar_pops() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let body = app.body.as_ref().unwrap();
        let (bx, by, bz) = (body.x, body.y, body.z);
        // THE DROP AT THE BODY (the entity-loot spawn's own shape —
        // the pickup_delay the test pre-drains, the scan the next
        // tick speaks)
        let mut rng = crate::drops::Lcg::new(0x5EED_10CA);
        app.drops.push(crate::drops::ItemDrop::spawn_entity_loot(
            1,
            dirt,
            3,
            bx,
            by + 0.5,
            bz,
            &mut rng,
        ));
        app.drops[0].pickup_delay = 0;
        // THE FIRST PICKUP: the empty hotbar slot the take lands in —
        // the grow the pop arms (the empty arm the sentinel compare
        // carries), the whole stack the single slot absorbs
        app.update();
        assert!(app.drops.is_empty(), "the take the whole stack spoke");
        assert_eq!(app.inventory.items[0], (dirt, 3), "the take the slot holds");
        assert_eq!(app.hotbar_slots[0], (dirt, 3), "the HUD's projection");
        assert_eq!(app.hotbar_pop[0], 5, "the fresh pick pops");
        assert_eq!(app.hotbar_pop[1], 0, "the silent slot stays silent");
        // THE DECAY: the next game tick the pop rides down (the
        // update()'s own inventoryTick walk)
        app.update();
        assert_eq!(app.hotbar_pop[0], 4, "the tick's own decay");
        // THE SECOND PICKUP (the merge into the partial stack — the
        // count the take grows, the same law re-arms): the tick's
        // head the decay speaks BEFORE the pickup arms, the 5 the
        // fresh verdict wears
        app.drops.push(crate::drops::ItemDrop::spawn_entity_loot(
            2,
            dirt,
            2,
            bx,
            by + 0.5,
            bz,
            &mut rng,
        ));
        app.drops[0].pickup_delay = 0;
        app.update();
        assert_eq!(app.inventory.items[0], (dirt, 5), "the merge the slot grows");
        assert_eq!(app.hotbar_pop[0], 5, "the grow re-pops");
        // THE SETTLE: the five ticks the pop rides, the plain draw the
        // f <= 0 gate keeps
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(app.hotbar_pop[0], 0, "the five ticks the pop rides");
    }


    /// THE USE-LANE'S OWN POP-KILL (javap 26.2
    /// ClientPacketListener.handleContainerSetSlot pcs 97-148 — the
    /// singleplayer consume the integrated server's SetSlot answers:
    /// the shrink the old-empty-or-grow compare refuses, the fresh
    /// stack's popTime-0 the in-flight flash kills): the placement the
    /// consumed slot's flash kills, the untouched slot's flash the
    /// decay keeps, the killed slot the silence wears, and the
    /// helper's own lands (the held consume, the arrow's hotbar arm)
    /// the same verdict speaks — the offhand arm the hotbar's pops
    /// never touches.
    #[test]
    fn the_use_consume_the_inflight_pop_kills() {
        let mut app = App::new(3);
        app.enter_world();
        let dirt_id = mc_registry::item_id_by_name("dirt").unwrap();
        // THE STACK SEEDED, THE FLASHES ARMED (the pickup lane's own
        // verdict the state wears — the take the pop spoke, the
        // neighbor's the same lane rides)
        app.inventory.items[0] = (dirt_id, 3);
        app.hotbar_slots = app.inventory.hotbar();
        app.hotbar_pop[0] = 5;
        app.hotbar_pop[2] = 5;
        // THE AIM (the place test's own shape — the downward look the
        // placement cell speaks)
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE PLACE: the survival shrink the integrated server's
        // SetSlot answers — the consumed slot the pop-0 verdict wears,
        // the untouched slot's flash the plain value keeps
        assert!(app.on_right_press(0, 0), "the placement spoke");
        assert_eq!(app.inventory.items[0], (dirt_id, 2), "the shrink rode");
        assert_eq!(app.hotbar_pop[0], 0, "the consumed slot the pop kills");
        assert_eq!(app.hotbar_pop[2], 5, "the untouched flash rides");
        // THE DECAY: the next tick the untouched flash rides down, the
        // killed slot stays silent (the zero the decay walk keeps)
        app.update();
        assert_eq!(app.hotbar_pop[0], 0, "the killed slot stays silent");
        assert_eq!(app.hotbar_pop[2], 4, "the untouched flash the decay rides");
        // THE HELPER'S OWN ARM: the held consume the same verdict
        // speaks (the selected slot's shrink the flash kills)
        app.selected_hotbar = 1;
        app.inventory.items[1] = (dirt_id, 4);
        app.hotbar_pop[1] = 5;
        app.consume_held_item();
        assert_eq!(app.inventory.items[1], (dirt_id, 3), "the held consume shrank");
        assert_eq!(app.hotbar_pop[1], 0, "the held consume the pop kills");
        // THE ARROW'S OWN ARMS: the hotbar arm the kill wears, the
        // offhand arm the hotbar's pops never touches (the
        // isHotbarSlot gate the law keeps)
        let arrow_id = mc_registry::item_id_by_name("arrow").unwrap();
        app.inventory.items[3] = (arrow_id, 8);
        app.inventory.offhand[0] = (arrow_id, 4);
        app.hotbar_pop[3] = 5;
        app.hotbar_pop[4] = 5;
        app.consume_arrow_at(3);
        assert_eq!(app.inventory.items[3], (arrow_id, 7), "the arrow shrank");
        assert_eq!(app.hotbar_pop[3], 0, "the arrow's hotbar arm the pop kills");
        app.consume_arrow_at(crate::drops::OFFHAND_CONTAINER);
        assert_eq!(app.inventory.offhand[0], (arrow_id, 3), "the offhand arrow shrank");
        assert_eq!(app.hotbar_pop[4], 5, "the offhand arm the hotbar never touches");
    }


    /// THE SURVIVAL CONSUME: the placement rides the selected stack's
    /// count down (BlockItem.place's own shrink), the HUD's projection
    /// follows, and the empty slot clears (the held lane falls back).
    #[test]
    fn the_place_lane_consumes_the_selected_stack() {
        let mut app = App::new(3);
        app.enter_world();
        let dirt_id = mc_registry::item_id_by_name("dirt").unwrap();
        // the selected slot carries 3 dirt — the pickups' own lane
        app.inventory.items[0] = (dirt_id, 3);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::AIR,
            "the placement cell starts empty"
        );
        assert!(app.on_right_press(0, 0), "the placement consumed the press");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::DIRT,
            "the voxel landed"
        );
        assert_eq!(
            app.inventory.items[0],
            (dirt_id, 2),
            "the count rode down one"
        );
        assert_eq!(app.hotbar_slots[0], (dirt_id, 2), "the projection followed");
        // THE SECOND PLACE: the crosshair now rides the placed dirt's own
        // UP face — the next cell lands on top, the count rides to 1
        assert!(app.on_right_press(0, 0), "the second place spoke");
        assert_eq!(app.inventory.items[0], (dirt_id, 1), "the count rode to 1");
        // THE REFUSAL: the next cell (the column's third) intersects the
        // camera's own standing box — the vanilla's own guard, the count
        // keeps its 1
        assert!(!app.on_right_press(0, 0), "the placement into the player refuses");
        assert_eq!(app.inventory.items[0], (dirt_id, 1), "the refused place consumes nothing");
        // THE CAMERA LIFTS: the same column's next cell no longer
        // intersects — the last stack places, the slot clears
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 6.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the third place spoke");
        assert_eq!(app.inventory.items[0], (0, 0), "the empty slot cleared");
        assert!(!app.on_right_press(0, 0), "the empty selection never places");
    }


    /// THE FURNACE'S OWN PLACEMENT FACING (the dev-24 GUI lane's own
    /// block the world's placement walk finally aims — javap 26.2
    /// AbstractFurnaceBlock.getStateForPlacement the FACING the
    /// placement state's own setValue wears): the south look the
    /// north byte 1 answers, the north look the south 3, the fresh
    /// cell the lazy variant layer banks, the non-furnace the byte
    /// never earns.
    #[test]
    fn the_furnace_the_placement_facing_the_look_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let furnace_id = mc_registry::item_id_by_name("furnace").unwrap();
        app.inventory.items[0] = (furnace_id, 4);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        // THE LAYER'S OWN ABSENCE (the offline scene the lazy
        // allocation serves — the furnace's own byte the first
        // stamp banks)
        assert!(
            app.offline_world.as_ref().unwrap().variants.is_none(),
            "the offline scene carries no variant layer yet"
        );
        // THE SOUTH LOOK (cam.yaw = PI — the vanilla yaw 0 the
        // wire_angles conversion speaks): the getDirection SOUTH
        // verdict, the opposite the front's NORTH byte 1 wears
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(
            app.on_right_press(0, 0),
            "the furnace placement consumed the press"
        );
        {
            let w = app.offline_world.as_ref().unwrap();
            assert_eq!(w.get(c, top, c), crate::world::FURNACE, "the furnace landed");
            assert_eq!(
                w.variant(c, top, c),
                1,
                "the south look faces the front north"
            );
            assert!(
                w.variants.is_some(),
                "the furnace's own byte the lazy layer banks"
            );
            assert_eq!(
                w.variant(c + 1, top, c),
                0,
                "the neighbor the no-opinion answer keeps"
            );
        }
        // THE NORTH LOOK (cam.yaw 0 — the vanilla -180): the stack's
        // next cell the SOUTH byte 3 wears — the camera the +5 lift
        // the clip's own 4.5 range keeps (the furnace's top face the
        // 4.0 read answers; the +6 the range never speaks). THE SNEAK
        // the block-use arm's own gate wears (the door the offline
        // sim's round added answers the un-sneaked press — the
        // vanilla shift the stacking walk's own answer keeps, the
        // use arm the sneak skips and the item lanes place).
        app.shift_down = true;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 5.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the second furnace placement spoke");
        {
            let w = app.offline_world.as_ref().unwrap();
            assert_eq!(
                w.get(c, top + 1, c),
                crate::world::FURNACE,
                "the second furnace landed"
            );
            assert_eq!(
                w.variant(c, top + 1, c),
                3,
                "the north look faces the front south"
            );
        }
        // THE NON-FURNACE STAMP'S OWN REFUSAL (the dirt the byte
        // never earns — the no-opinion answer the fresh cell keeps,
        // the layer the allocation the furnace stamps already paid)
        let dirt_id = mc_registry::item_id_by_name("dirt").unwrap();
        app.inventory.items[0] = (dirt_id, 2);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(app.on_right_press(0, 0), "the dirt placement spoke");
        {
            let w = app.offline_world.as_ref().unwrap();
            assert_eq!(w.get(c, top + 2, c), crate::world::DIRT, "the dirt landed");
            assert_eq!(
                w.variant(c, top + 2, c),
                0,
                "the dirt cell the facing byte never earns"
            );
        }
    }


    /// THE FURNACE'S OWN DOOR + THE SIM'S OWN ARC (the offline
    /// right-press the use lane's block-use arm opens, the sim's
    /// tick the block entity walks, the LIT FLIP the world's placed
    /// kind wears — the emission-13 light the flood re-speaks, the
    /// unlit kind the exhaustion restores; the clicks the local
    /// cursor wears, the close the block entity keeps).
    #[test]
    fn the_furnace_the_door_the_sim_the_world_glows() {
        let mut app = App::new(3);
        app.enter_world();
        let furnace_id = mc_registry::item_id_by_name("furnace").unwrap();
        let coal = mc_registry::item_id_by_name("coal").unwrap();
        let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
        let iron_ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
        // THE LOADOUT (the hotbar head the furnace rides, the main
        // grid the coal and the raw iron carry)
        app.inventory.items[0] = (furnace_id, 4);
        app.inventory.items[1] = (coal, 3);
        app.inventory.items[9] = (raw_iron, 5);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the furnace placement consumed");
        assert_eq!(app.offline_world.as_ref().unwrap().get(c, top, c), crate::world::FURNACE);
        // THE DOOR (the second press the block-use arm answers — the
        // held furnace item the placement lane never reaches; the
        // furnace's top face the 3.0 read the clip answers)
        assert!(app.on_right_press(0, 0), "the door consumed the press");
        assert_eq!(app.screen, ScreenKind::Furnace, "the door opened the screen");
        assert_eq!(app.furnace_pos, Some((c, top, c)), "the cell the sim records");
        assert!(!app.furnace_is_live, "the offline stand-in the door keeps");
        assert_eq!(app.furnace_slots[1], (0, 0), "the cells the fresh zeros wear");
        // THE STANDARD ROWS' OWN PROJECTION (the hotbar head the
        // menu 30 speaks — the leftover furnaces the placement's own
        // shrink wore, the coal the menu 31 carries, the main grid
        // head the menu 3)
        assert_eq!(app.furnace_slots[30], (furnace_id, 3), "the hotbar head rode");
        assert_eq!(app.furnace_slots[31], (coal, 3), "the coal rode menu 31");
        assert_eq!(app.furnace_slots[3], (raw_iron, 5), "the main head rode");
        // THE CLICKS (the geometry the test lays — the coal the fuel
        // slot takes, the raw iron the input wears). THE RELEASE LAW
        // (the quickcraft re-key the base walk wears — the carried
        // press ARMS, the place the RELEASE's deferred place serves,
        // the mouse feed the drive needs)
        app.furnace_panel = (50, 50);
        app.furnace_slot_rects = vec![
            (100, 100, 0),  // input
            (140, 100, 1),  // fuel
            (180, 100, 2),  // result
            (220, 100, 31), // hotbar's coal (menu 31 = items[1])
            (260, 100, 3),  // main head (raw iron)
        ];
        app.on_press(220, 100); // pickup the coal (the empty cursor the click lane keeps)
        assert_eq!(app.furnace_carried, (coal, 3), "the coal the cursor carries");
        app.on_press(140, 100); // place on the fuel cell
        app.on_mouse_move(140, 100);
        app.on_left_release();
        assert_eq!(app.furnace_slots[1], (coal, 3), "the fuel cell speaks");
        assert_eq!(app.furnace_carried, (0, 0), "the cursor the place empties");
        app.on_press(260, 100); // pickup the raw iron
        app.on_press(100, 100); // place on the input cell
        app.on_mouse_move(100, 100);
        app.on_left_release();
        assert_eq!(app.furnace_slots[0], (raw_iron, 5), "the input cell speaks");
        // THE RESULT'S OWN mayPlace (the carried the refusal keeps —
        // the fuel cell's own coal the cursor rides out)
        app.on_press(180, 100); // take the (empty) result — no-op
        assert_eq!(app.furnace_carried, (0, 0), "the empty take no-op");
        app.on_press(140, 100); // pickup the coal from the fuel cell
        assert_eq!(app.furnace_carried, (coal, 3), "the coal the cursor carries");
        app.furnace_slots[2] = (iron_ingot, 2);
        app.on_press(180, 100); // place on the result — REFUSED
        app.on_mouse_move(180, 100);
        app.on_left_release();
        assert_eq!(app.furnace_slots[2], (iron_ingot, 2), "the result the place refuses");
        assert_eq!(app.furnace_carried, (coal, 3), "the carried the refusal keeps");
        app.on_press(140, 100); // the coal rides home to the fuel cell
        app.on_mouse_move(140, 100);
        app.on_left_release();
        assert_eq!(app.furnace_slots[1], (coal, 3), "the coal the fuel keeps");
        assert_eq!(app.furnace_carried, (0, 0), "the cursor the place empties");
        // THE MERGE (the same item the input takes — a second raw
        // iron stack the projection rides)
        app.inventory.items[9] = (raw_iron, 4);
        app.furnace_slots[3] = (raw_iron, 4);
        app.on_press(260, 100); // pickup the second stack
        app.on_press(100, 100); // the input cell takes the carried (merge)
        app.on_mouse_move(100, 100);
        app.on_left_release();
        assert_eq!(app.furnace_slots[0], (raw_iron, 9), "the merge the same item wears");
        assert_eq!(app.furnace_carried, (0, 0), "the cursor the merge empties");
        // THE FIRST TICK (the ignite the sim speaks — the words bank,
        // the world's kind the LIT flip wears, the light the flood
        // spreads)
        let (chunks, _sx, _sz) = {
            let w = app.offline_world.as_ref().unwrap();
            (w.chunks_per_side, 0, 0)
        };
        let light_before = app
            .offline_world
            .as_ref()
            .unwrap()
            .light
            .as_ref()
            .map(|l| l.cell(chunks, c, top + 1, c))
            .unwrap();
        app.update();
        assert_eq!(app.furnace_data[0], 1600, "the coal's own burn banks");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::FURNACE_LIT,
            "the world's kind the flip wears"
        );
        let light_after = app
            .offline_world
            .as_ref()
            .unwrap()
            .light
            .as_ref()
            .map(|l| l.cell(chunks, c, top + 1, c))
            .unwrap();
        assert!(
            light_after.1.unwrap_or(0) > light_before.1.unwrap_or(0),
            "the emission-13 the flood re-spoke ({light_before:?} -> {light_after:?})"
        );
        assert_eq!(app.furnace_slots[1], (coal, 2), "the coal the consume loses");
        // THE SMELT'S OWN COMPLETION (the mayPlace fixture the empty
        // wears first — the fresh result the arc grows; 200 ticks the
        // arc walks, the 1st tick already spoke, 199 the rest carry)
        app.furnace_slots[2] = (0, 0);
        for _ in 0..199 {
            app.update();
        }
        assert_eq!(app.furnace_slots[2], (iron_ingot, 1), "the ingot the burn lands");
        assert_eq!(app.furnace_slots[0], (raw_iron, 8), "the input the shrink loses");
        // THE EXHAUSTION'S OWN RESTORE (the fuel the empty wears —
        // the flip back the unlit kind the world keeps)
        app.furnace_slots[1] = (0, 0);
        app.furnace_data[0] = 2;
        app.update();
        assert_eq!(app.furnace_data[0], 1, "the lit tick the walk spends");
        app.update();
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::FURNACE,
            "the unlit kind the exhaustion restores"
        );
        // THE CLOSE (the block entity the world keeps — the words and
        // the cells the retention banks)
        app.on_inventory_key(); // the E key's own close
        assert_eq!(app.screen, ScreenKind::World, "the close returned");
        assert_eq!(app.furnace_pos, Some((c, top, c)), "the cell the close keeps");
        assert_eq!(app.furnace_slots[2], (iron_ingot, 1), "the result the close keeps");
        // THE REOPEN (the same furnace the words and cells ride in —
        // the burn's own memory never died)
        assert!(app.on_right_press(0, 0), "the door consumed again");
        assert_eq!(app.screen, ScreenKind::Furnace, "the door opened again");
        assert_eq!(app.furnace_slots[2], (iron_ingot, 1), "the ingot the reopen keeps");
        assert_eq!(app.furnace_slots[0], (raw_iron, 8), "the input the reopen keeps");
        assert_eq!(app.furnace_data[3], 200, "the words the reopen keeps");
        // THE RESULT'S OWN TAKE (the EMPTY cursor the take speaks —
        // the carried the mayPlace refusal gates, the take-alone
        // verdict the empty cursor answers)
        app.on_press(180, 100); // take the ingot
        assert_eq!(app.furnace_carried, (iron_ingot, 1), "the ingot the cursor takes");
        assert_eq!(app.furnace_slots[2], (0, 0), "the result the take empties");
        // THE CARRIED'S OWN RETURN (the close the add-back walk
        // serves — the ingot the inventory takes home)
        app.on_inventory_key(); // the close's add-back
        assert_eq!(app.screen, ScreenKind::World, "the close returned again");
        assert_eq!(app.furnace_carried, (0, 0), "the cursor the close empties");
        let inv_had = app
            .inventory
            .items
            .iter()
            .filter(|&&(id, n)| id == iron_ingot && n > 0)
            .count();
        assert!(inv_had > 0, "the ingot the add-back walked home");
    }


    /// THE OFFLINE FURNACE'S OWN SHIFT-CLICK (the quick-move lane the
    /// predictor wears — the smeltable the INPUT takes, the fuel the
    /// FUEL, the write-through the standard rows' projection keeps,
    /// the shift-fed furnace the sim ignites: javap 26.2
    /// AbstractFurnaceMenu.quickMoveStack).
    #[test]
    fn the_furnace_the_shift_click_the_write_through_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let furnace_id = mc_registry::item_id_by_name("furnace").unwrap();
        let coal = mc_registry::item_id_by_name("coal").unwrap();
        let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
        app.inventory.items[0] = (furnace_id, 1);
        app.inventory.items[1] = (coal, 12);
        app.inventory.items[9] = (raw_iron, 5);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the furnace placed");
        assert!(app.on_right_press(0, 0), "the door opened");
        assert_eq!(app.screen, ScreenKind::Furnace);
        app.furnace_panel = (0, 0);
        app.furnace_slot_rects = vec![
            (100, 100, 0),  // input
            (140, 100, 1),  // fuel
            (220, 100, 31), // the hotbar's coal (menu 31 = items[1])
            (260, 100, 3),  // the main's ore (menu 3 = items[9])
        ];
        // THE SHIFT-CLICK ON THE COAL (the fuel arm the routing
        // speaks — the FUEL cell takes, the write-through the
        // projection empties, the cursor never rides)
        app.on_shift_down(true);
        app.on_press(220, 100);
        assert_eq!(app.furnace_slots[1], (coal, 12), "the fuel cell the shift took");
        assert_eq!(app.inventory.items[1], (0, 0), "the hotbar cell the write-through emptied");
        assert_eq!(app.furnace_carried, (0, 0), "the cursor the quick-move never rides");
        // THE SHIFT-CLICK ON THE ORE (the smeltable arm — the INPUT
        // cell takes, the main row's projection follows)
        app.on_press(260, 100);
        assert_eq!(app.furnace_slots[0], (raw_iron, 5), "the input cell the shift took");
        assert_eq!(app.inventory.items[9], (0, 0), "the main cell the write-through emptied");
        // THE SHIFT-FED FURNACE BURNS (the next update the sim walks —
        // the ignite the shift-click's own placement earned)
        app.shift_down = false;
        app.update();
        assert_eq!(app.furnace_data[0], 1600, "the coal's own burn banks");
        assert_eq!(app.furnace_slots[1], (coal, 11), "the coal the consume loses");
        assert_eq!(app.furnace_data[2], 1, "the same tick's cook speaks");
        // THE SHIFT-CLICK FROM THE RESULT (the reverse walk the
        // inventory takes — the hotbar's tail the one-empty pass
        // fills)
        app.furnace_data[2] = 199;
        app.furnace_slots[2] = (0, 0);
        for _ in 0..200 {
            app.update();
        }
        assert_eq!(app.furnace_slots[2], (mc_registry::item_id_by_name("iron_ingot").unwrap(), 1), "the ingot the arc landed");
        app.shift_down = true;
        let rects = app.furnace_slot_rects.clone();
        let (_, _, result_slot) = rects[0];
        assert_eq!(result_slot, 0);
        // the result cell's rect the fresh geometry needs — the test
        // lays it directly
        app.furnace_slot_rects = vec![(100, 100, 2), (300, 100, 38)];
        app.on_press(108, 108);
        assert_eq!(app.furnace_slots[2], (0, 0), "the result the shift emptied");
        assert_eq!(app.furnace_slots[38], (mc_registry::item_id_by_name("iron_ingot").unwrap(), 1), "the hotbar's tail took it");
        assert_eq!(app.inventory.items[8], (mc_registry::item_id_by_name("iron_ingot").unwrap(), 1), "the write-through rode the hotbar's tail");
        app.on_shift_down(false);
    }


    /// THE FURNACE'S OWN ANIMATE-TICK LANE (the crackle + the ember
    /// pair the level sampler speaks — javap 26.2 Minecraft.tick's
    /// "animateTick" section + ClientLevel.animateTick/doAnimateTick +
    /// FurnaceBlock.animateTick): the lit furnace the sampler hears,
    /// the 0.1 gate the seeded stream rolls, the positioned voice the
    /// fixed 1.0/1.0 fold speaks, the smoke-then-flame pair the
    /// front-face offsets place.
    #[test]
    fn the_furnace_the_crackle_the_embers_the_sampler_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let furnace_id = mc_registry::item_id_by_name("furnace").unwrap();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        // THE HEADLESS CAMERA (the body none stands — the camera's own
        // cell the sampler center proxies, the placement lane's own
        // convention)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        app.inventory.items[0] = (furnace_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(app.on_right_press(0, 0), "the placement consumed");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::FURNACE,
            "the furnace the placement lands"
        );
        // THE FACING THE PLACEMENT STAMPED (the ember's front face the
        // axis term reads — the byte the set_variant walk banked)
        let facing = app.offline_world.as_ref().unwrap().variant(c, top, c);
        // THE LIT STAMP (the kind the sim's flip would wear — the
        // animate walk the LIT cell alone answers)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, top, c, crate::world::FURNACE_LIT);
        // THE WALK (the ticks the sampler spends — ~0.025 visits/tick
        // the single furnace the two ranges' cells speak, the seeded
        // streams the visits answer deterministically)
        for _ in 0..600 {
            app.update();
        }
        // THE CRACKLES (the gate's 10% the visits roll — the event
        // the log speaks, the fixed 1.0/1.0 fold the distance-zero ear
        // (no body) the v1.000000 keeps, the weighted file pick the
        // engine's own LCG banks)
        let crackles: Vec<&String> = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("block.furnace.fire_crackle "))
            .collect();
        assert!(
            !crackles.is_empty(),
            "the lit furnace the sampler hears: {} crackles over 600 ticks",
            crackles.len()
        );
        for line in &crackles {
            assert!(
                line.contains(" v1.000000 p1.000000"),
                "the fixed fold the volume/pitch pair keeps: {line}"
            );
        }
        // THE UNLIT WORLD'S OWN SILENCE (the gate the unlit kind never
        // passes — a fresh scene's unlit furnace the sampler hears
        // NOTHING from)
        let mut cold = App::new(3);
        cold.enter_world();
        cold.inventory.items[0] = (furnace_id, 1);
        cold.hotbar_slots = cold.inventory.hotbar();
        let cold_top = cold.spawn_y;
        let cold_c = cold.offline_world.as_ref().unwrap().size_blocks() / 2;
        cold.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(
                cold_c as f32 + 0.5,
                cold_top as f32 + 4.0,
                cold_c as f32 + 0.5,
            ),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(cold.on_right_press(0, 0), "the cold placement consumed");
        assert_eq!(
            cold.offline_world.as_ref().unwrap().get(cold_c, cold_top, cold_c),
            crate::world::FURNACE,
            "the unlit kind the placement wears"
        );
        for _ in 0..600 {
            cold.update();
        }
        assert!(
            cold.sounds
                .events
                .iter()
                .all(|e| !e.starts_with("block.furnace.fire_crackle ")),
            "the unlit furnace the walk never hears"
        );
        assert!(
            cold.particles
                .iter()
                .all(|p| !matches!(p.kind, crate::particles::ParticleKind::Flame)),
            "the unlit furnace no flame speaks"
        );
        // THE EMBERS (the smoke-then-flame pair the visits speak — the
        // front-face anchor the axis term names, the lateral draw the
        // ±0.3 band the d4 fold keeps, the height the [0, 0.375) band
        // the d6 fold keeps; the drift the first ticks' tiny walk
        // wears the bands absorb)
        let (adx, adz) = match facing {
            2 => (0.52, 0.0),
            4 => (-0.52, 0.0),
            3 => (0.0, 0.52),
            _ => (0.0, -0.52),
        };
        let ax = c as f64 + 0.5 + adx;
        let az = c as f64 + 0.5 + adz;
        let near_front = |p: &crate::particles::Particle| {
            (p.x - ax).abs() <= 0.45
                && (p.z - az).abs() <= 0.45
                && p.y >= top as f64
                && p.y <= top as f64 + 0.45
        };
        let flames: Vec<&crate::particles::Particle> = app
            .particles
            .iter()
            .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Flame))
            .collect();
        let smokes: Vec<&crate::particles::Particle> = app
            .particles
            .iter()
            .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Smoke))
            .collect();
        assert!(
            flames.iter().any(|p| near_front(p)),
            "the flame the front face wears (facing {facing}, anchor ({ax}, {az})): {:?}",
            flames.iter().map(|p| (p.x, p.y, p.z)).take(6).collect::<Vec<_>>()
        );
        assert!(
            smokes.iter().any(|p| near_front(p)),
            "the smoke the front face wears (facing {facing}): {} smokes alive",
            smokes.len()
        );
    }


    /// THE BLAST FURNACE'S OWN DOOR + THE HUNDRED-TICK ARC (the dev-26
    /// round — the placement the family predicate stamps, the door the
    /// orientable gate answers, the sim's BLASTING lane the kind
    /// records, the LIT FLIP the world's BLAST_FURNACE_LIT kind wears,
    /// the 100-tick arc the codec default keeps, the close the block
    /// entity keeps).
    #[test]
    fn the_blast_furnace_the_door_the_hundred_arc_the_world_glows() {
        let mut app = App::new(3);
        app.enter_world();
        let blast_id = mc_registry::item_id_by_name("blast_furnace").unwrap();
        let coal = mc_registry::item_id_by_name("coal").unwrap();
        let iron_ore = mc_registry::item_id_by_name("iron_ore").unwrap();
        let iron_ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
        app.inventory.items[0] = (blast_id, 2);
        app.inventory.items[1] = (coal, 3);
        app.inventory.items[9] = (iron_ore, 5);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the blast furnace placed");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::BLAST_FURNACE,
            "the blast furnace the placement lands"
        );
        // THE FACING THE PLACEMENT STAMPED (the orientable trio's own
        // byte the AbstractFurnaceBlock base walk keeps — the
        // down-look's own decode the north byte wears: the vertical
        // look the horizontal flip never flips, the y-rotation the
        // byte vocabulary keeps)
        assert_eq!(
            app.offline_world.as_ref().unwrap().variant(c, top, c),
            1,
            "the down-look's placement the north byte wears"
        );
        // THE DOOR (the second press the orientable gate answers)
        assert!(app.on_right_press(0, 0), "the door consumed the press");
        assert_eq!(app.screen, ScreenKind::Furnace, "the door opened the screen");
        assert_eq!(app.furnace_pos, Some((c, top, c)), "the cell the sim records");
        assert_eq!(
            app.furnace_sim.kind,
            crate::furnace_sim::FurnaceKind::BlastFurnace,
            "the kind's own lane the door records"
        );
        // THE CLICKS (the coal the fuel takes, the ore the input
        // wears). THE RELEASE LAW (the quickcraft re-key the base
        // walk wears — the carried press ARMS, the place the
        // RELEASE's deferred place serves, the mouse feed the drive
        // needs)
        app.furnace_panel = (50, 50);
        app.furnace_slot_rects = vec![
            (100, 100, 0),  // input
            (140, 100, 1),  // fuel
            (220, 100, 31), // hotbar's coal (menu 31 = items[1])
            (260, 100, 3),  // main head (ore)
        ];
        app.on_press(220, 100);
        app.on_press(140, 100);
        app.on_mouse_move(140, 100);
        app.on_left_release();
        assert_eq!(app.furnace_slots[1], (coal, 3), "the fuel cell speaks");
        app.on_press(260, 100);
        app.on_press(100, 100);
        app.on_mouse_move(100, 100);
        app.on_left_release();
        assert_eq!(app.furnace_slots[0], (iron_ore, 5), "the input cell speaks");
        // THE FIRST TICK (the ignite the HALF coal banks — 1600/2 =
        // 800 — the world's kind the BLAST flip wears, the light the
        // flood re-speaks)
        let (chunks, _sx, _sz) = {
            let w = app.offline_world.as_ref().unwrap();
            (w.chunks_per_side, 0, 0)
        };
        let light_before = app
            .offline_world
            .as_ref()
            .unwrap()
            .light
            .as_ref()
            .map(|l| l.cell(chunks, c, top + 1, c))
            .unwrap();
        app.update();
        assert_eq!(app.furnace_data[0], 800, "the coal the half banks");
        assert_eq!(app.furnace_data[3], 100, "the blast's own 100 the reset speaks");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::BLAST_FURNACE_LIT,
            "the world's kind the BLAST flip wears"
        );
        let light_after = app
            .offline_world
            .as_ref()
            .unwrap()
            .light
            .as_ref()
            .map(|l| l.cell(chunks, c, top + 1, c))
            .unwrap();
        assert!(
            light_after.1.unwrap_or(0) > light_before.1.unwrap_or(0),
            "the emission-13 the flood re-spoke ({light_before:?} -> {light_after:?})"
        );
        // THE HUNDRED-TICK ARC (the first tick spoke, 99 the rest
        // carry — the ingot the burn lands, the input the shrink loses)
        for _ in 0..99 {
            app.update();
        }
        assert_eq!(app.furnace_slots[2], (iron_ingot, 1), "the ingot the burn lands");
        assert_eq!(app.furnace_slots[0], (iron_ore, 4), "the input the shrink loses");
        assert_eq!(
            app.furnace_data[3],
            100,
            "the re-fetch the same 100 keeps"
        );
        // THE POTATO REFUSAL (the food row the BLASTING table never
        // carries — the input swap the frozen walk wears, the fuel
        // the burn keeps)
        app.furnace_slots[0] = (
            mc_registry::item_id_by_name("potato").unwrap(),
            2,
        );
        let lit_before = app.furnace_data[0];
        app.update();
        assert_eq!(app.furnace_slots[2], (iron_ingot, 1), "the result never grows");
        assert_eq!(
            app.furnace_data[0],
            lit_before - 1,
            "the lit state the burn keeps"
        );
        // THE CLOSE (the block entity the world keeps)
        app.on_inventory_key();
        assert_eq!(app.screen, ScreenKind::World, "the close returned");
        assert_eq!(app.furnace_pos, Some((c, top, c)), "the cell the close keeps");
        // THE REOPEN (the kind's own lane the family re-answers)
        assert!(app.on_right_press(0, 0), "the door consumed again");
        assert_eq!(
            app.furnace_sim.kind,
            crate::furnace_sim::FurnaceKind::BlastFurnace,
            "the kind the reopen re-answers"
        );
        assert_eq!(app.furnace_slots[2], (iron_ingot, 1), "the ingot the reopen keeps");
    }


    /// THE FAST FAMILY'S OWN ANIMATE EVENTS (the dev-26 round — the
    /// lit BLAST furnace the sampler hears the BLASTFURNACE event +
    /// the SMOKE-ONLY pair the flame-less sibling speaks [no flame
    /// particle the blast cell ever spawns], the lit SMOKER the
    /// SMOKER_SMOKE event the ONE-draw walk answers + the smoke at
    /// the FIXED +1.1 anchor the simple tail keeps).
    #[test]
    fn the_blast_the_smoker_the_animate_events_the_smoke_only_wears() {
        // THE BLAST FURNACE'S OWN SCENE
        let mut app = App::new(3);
        app.enter_world();
        let blast_id = mc_registry::item_id_by_name("blast_furnace").unwrap();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        app.inventory.items[0] = (blast_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(app.on_right_press(0, 0), "the blast placed");
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, top, c, crate::world::BLAST_FURNACE_LIT);
        for _ in 0..600 {
            app.update();
        }
        let blast_crackles: Vec<&String> = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("block.blastfurnace.fire_crackle "))
            .collect();
        assert!(
            !blast_crackles.is_empty(),
            "the lit blast furnace the sampler hears: {} crackles",
            blast_crackles.len()
        );
        for line in &blast_crackles {
            assert!(
                line.contains(" v1.000000 p1.000000"),
                "the fixed fold the volume/pitch pair keeps: {line}"
            );
        }
        assert!(
            app.sounds
                .events
                .iter()
                .all(|e| !e.starts_with("block.furnace.fire_crackle ")),
            "the furnace's own event the blast cell never speaks"
        );
        assert!(
            app.sounds
                .events
                .iter()
                .all(|e| !e.starts_with("block.smoker.smoke ")),
            "the smoker's own event the blast cell never speaks"
        );
        // THE SMOKE-ONLY TAIL (the flame particle the blast's own
        // animateTick NEVER spawns — the one addParticle the bytecode
        // keeps — the smokes the front-face band the furnace's law
        // keeps, the taller 9/16 height the [0, 0.5625) band answers)
        assert!(
            app.particles
                .iter()
                .all(|p| !matches!(p.kind, crate::particles::ParticleKind::Flame)),
            "the blast furnace no flame speaks"
        );
        assert!(
            app.particles
                .iter()
                .any(|p| matches!(p.kind, crate::particles::ParticleKind::Smoke)
                    && p.y >= top as f64
                    && p.y <= top as f64 + 9.0 / 16.0),
            "the smoke the taller band the blast's own literal keeps"
        );
        // THE SMOKER'S OWN SCENE (the ONE-draw walk the fixed anchor
        // answers — the smoke ABOVE the block the +1.1 speaks)
        app.particles.clear();
        app.sounds.events.clear();
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, top, c, crate::world::SMOKER_LIT);
        for _ in 0..600 {
            app.update();
        }
        let smoker_events: Vec<&String> = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("block.smoker.smoke "))
            .collect();
        assert!(
            !smoker_events.is_empty(),
            "the lit smoker the sampler hears: {} events",
            smoker_events.len()
        );
        for line in &smoker_events {
            assert!(
                line.contains(" v1.000000 p1.000000"),
                "the fixed fold the volume/pitch pair keeps: {line}"
            );
        }
        assert!(
            app.particles
                .iter()
                .any(|p| matches!(p.kind, crate::particles::ParticleKind::Smoke)
                    && (p.y - (top as f64 + 1.1)).abs() < 0.35
                    && (p.x - (c as f64 + 0.5)).abs() < 0.35
                    && (p.z - (c as f64 + 0.5)).abs() < 0.35),
            "the smoke the FIXED +1.1 anchor the simple tail speaks: {:?}",
            app
                .particles
                .iter()
                .map(|p| (p.x, p.y, p.z))
                .take(6)
                .collect::<Vec<_>>()
        );
    }


    /// THE SMOKER'S OWN DOOR + THE FOOD LANE (the dev-26 round — the
    /// SMOKING recipes the sim's own table answers, the potato the
    /// baked potato the 100-tick arc wears, the LIT flip the world's
    /// SMOKER_LIT kind wears).
    #[test]
    fn the_smoker_the_door_the_food_lane_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let smoker_id = mc_registry::item_id_by_name("smoker").unwrap();
        let coal = mc_registry::item_id_by_name("coal").unwrap();
        let potato = mc_registry::item_id_by_name("potato").unwrap();
        let baked = mc_registry::item_id_by_name("baked_potato").unwrap();
        app.inventory.items[0] = (smoker_id, 1);
        app.inventory.items[1] = (coal, 2);
        app.inventory.items[9] = (potato, 4);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the smoker placed");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::SMOKER,
            "the smoker the placement lands"
        );
        assert!(app.on_right_press(0, 0), "the door consumed the press");
        assert_eq!(app.screen, ScreenKind::Furnace);
        assert_eq!(
            app.furnace_sim.kind,
            crate::furnace_sim::FurnaceKind::Smoker,
            "the kind's own lane the door records"
        );
        // THE RELEASE LAW (the quickcraft re-key the base walk wears
        // — the carried press ARMS, the place the RELEASE's deferred
        // place serves, the mouse feed the drive needs)
        app.furnace_panel = (50, 50);
        app.furnace_slot_rects = vec![
            (100, 100, 0),  // input
            (140, 100, 1),  // fuel
            (220, 100, 31), // hotbar's coal
            (260, 100, 3),  // main head (potato)
            (300, 100, 4),  // main row's ore (menu 4)
        ];
        app.on_press(220, 100);
        app.on_press(140, 100);
        app.on_mouse_move(140, 100);
        app.on_left_release();
        app.on_press(260, 100);
        app.on_press(100, 100);
        app.on_mouse_move(100, 100);
        app.on_left_release();
        assert_eq!(app.furnace_slots[0], (potato, 4), "the input speaks");
        // THE IRON ORE REFUSAL (the ore row the SMOKING table never
        // carries — the quick-move the MAIN arm answers: the ore the
        // hotbar rows take, the input the potato keeps)
        let iron_ore = mc_registry::item_id_by_name("iron_ore").unwrap();
        app.on_shift_down(true);
        app.furnace_slots[4] = (iron_ore, 3); // the believed row the test lays
        app.on_press(300, 100); // the shift-click the kind's OWN table answers
        assert_eq!(
            app.furnace_slots[0],
            (potato, 4),
            "the ore the SMOKING table refuses — the input the potato keeps"
        );
        assert_eq!(
            app.furnace_slots[4],
            (0, 0),
            "the ore's row the quick-move empties"
        );
        assert_eq!(
            app.furnace_slots[30],
            (iron_ore, 3),
            "the ore the hotbar's head row takes (the main arm's walk)"
        );
        app.on_shift_down(false);
        // THE ARC (the ignite the half coal banks, the 100 ticks the
        // baked potato lands, the world the SMOKER_LIT kind wears)
        app.update();
        assert_eq!(app.furnace_data[0], 800, "the coal the half banks");
        assert_eq!(app.furnace_data[3], 100, "the smoker's own 100 speaks");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::SMOKER_LIT,
            "the world's kind the SMOKER flip wears"
        );
        for _ in 0..99 {
            app.update();
        }
        assert_eq!(app.furnace_slots[2], (baked, 1), "the baked potato lands");
        assert_eq!(app.furnace_slots[0], (potato, 3), "the potato the shrink loses");
    }


    /// THE FALL DAMAGE ARC (the hurt round's own headline): the body
    /// spawned ten blocks above the surface falls, the landing folds
    /// damage 7 through calculateFallDamage, the pipeline speaks the
    /// fall voice (the FIXED 1.0/1.0 lane), the hurt voice (the
    /// (r-r)*0.2+1.0 fold), the health drops 20 -> 13, the flash arms
    /// (hurtTime 10) and the cooldown re-arms (invulnerableTime 20).
    #[test]
    fn the_fall_damage_hurts_the_player() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        // the body ten blocks above the spawn surface (the feet on the
        // top solid cell's surface, ten up)
        let spawn_feet = app.spawn_y;
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            (spawn_feet + 10) as f64,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        assert_eq!(app.health, 20.0, "the untouched player starts full");
        // the ticks until the landing speaks
        let mut landed = false;
        for _ in 0..60 {
            app.update();
            if app.health != 20.0 {
                landed = true;
                break;
            }
        }
        assert!(landed, "the landing spoke inside the ticks");
        // THE CLAMP'S OWN ARITHMETIC: the 10-block descent banks
        // 10.0 minus the rising clamp's 0.1435 (the
        // checkFallDistanceAccumulation law) — the landing folds 6
        // (floor(9.8565 + 1e-6 - 3.0)), the health rides to 14
        assert_eq!(app.health, 14.0, "the clamp-loss verdict, the bytecode's own");
        assert_eq!(app.hurt_time, 10, "the flash armed at its full window");
        assert_eq!(app.hurt_duration, 10);
        assert_eq!(app.invulnerable_time, 20, "the cooldown re-armed");
        assert!(!app.dead, "six damage never kills");
        // THE VOICES: the fall's own fixed lane, then the hurt's pitched
        // one (causeFallDamage's own order) — the assert computes the
        // expected from the ACTUAL loaded state (the cache-absent boot
        // the silent lane keeps)
        if sounds_loaded() {
            let fall = app
                .sounds
                .events
                .iter()
                .position(|e| e.contains("_fall @"))
                .expect("the fall voice spoke");
            assert!(
                app.sounds.events[fall].contains(" v1.000000 p1.000000"),
                "the fall's fixed fold: {}",
                app.sounds.events[fall]
            );
            let hurt = app
                .sounds
                .events
                .iter()
                .position(|e| e.starts_with("entity.player.hurt @"))
                .expect("the hurt voice spoke");
            assert!(hurt > fall, "the hurt voice follows the fall's own");
        }
        // THE COOLDOWN'S ECHO: a second landing inside the window refuses
        // the smaller repeat (the lastHurt comparison) — the health stays
        app.hurt(5.0);
        assert_eq!(
            app.health, 14.0,
            "the 5-damage repeat inside the window refuses (5 <= lastHurt 6)"
        );
    }


    /// THE HURT COOLDOWN LAW (javap hurtServer's own comparison chain):
    /// the fresh hurt re-arms the 20-tick window; a bigger hurt inside
    /// the 10-tick sub-window lands for the DIFFERENCE only; a smaller
    /// one refuses; a hurt after the window re-lands whole.
    #[test]
    fn the_hurt_cooldown_walks_the_javap_chain() {
        let mut app = App::new(3);
        app.enter_world();
        // the regen lane silenced for the walk's own arithmetic (the
        // saturation empty — the slow lane's 80-tick beat never fires
        // inside the 25-tick walk)
        app.food_data.set_saturation(0.0);
        // the fresh hurt: the whole amount lands
        app.hurt(2.0);
        assert_eq!(app.health, 18.0);
        assert_eq!(app.invulnerable_time, 20);
        assert_eq!(app.hurt_time, 10);
        assert_eq!(app.last_hurt, 2.0);
        // five ticks: the window still holds (15 > 10)
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(app.invulnerable_time, 15);
        // the BIGGER hurt lands the difference
        app.hurt(3.0);
        assert_eq!(app.health, 17.0, "3 - lastHurt 2 = 1 damage");
        assert_eq!(app.invulnerable_time, 15, "the window never re-arms");
        assert_eq!(app.hurt_time, 5, "the flash never re-arms either");
        // the SMALLER hurt refuses outright
        app.hurt(2.0);
        assert_eq!(app.health, 17.0, "2 <= lastHurt 3 refused");
        // the window drains: the fresh hurt re-lands whole
        for _ in 0..20 {
            app.update();
        }
        assert_eq!(app.invulnerable_time, 0);
        app.hurt(2.0);
        assert_eq!(app.health, 15.0, "the fresh hurt lands whole");
        assert_eq!(app.invulnerable_time, 20, "re-armed");
        assert_eq!(app.hurt_time, 10, "the flash re-armed");
    }


    /// THE DEATH ARC AND THE RESPAWN (the vanilla's own beat pair): the
    /// killing hurt speaks the death voice, arms the death screen (the
    /// HUD sleeps, the gradient owns the world), zeroes the input; the
    /// Respawn resets the vitals and re-anchors the body to the spawn
    /// pose with the fall state cleared.
    #[test]
    fn the_death_arc_respawns_at_the_spawn() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        let spawn = app.spawn_pose.expect("the spawn pose recorded");
        // THE KILLING HURT
        app.hurt(20.0);
        assert_eq!(app.health, 0.0, "the health clamped at zero");
        assert!(app.dead, "the death screen armed");
        // THE DEATH VOICE (the assert computes the expected from the
        // ACTUAL loaded state — the cache-absent boot the silent lane keeps)
        if sounds_loaded() {
            app.sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.player.death @"))
                .expect("the death voice spoke");
        }
        // further hurts refuse (the isDeadOrDying gate)
        app.hurt(5.0);
        assert_eq!(app.health, 0.0);
        // THE INPUT GATE: the dead body's impulses zero through update
        if let Some(b) = app.body.as_mut() {
            b.forward = 1.0;
            b.jumping = true;
        }
        app.update();
        if let Some(b) = app.body.as_ref() {
            assert_eq!(b.forward, 0.0, "the corpse's impulses zero");
            assert!(!b.jumping, "the corpse never jumps");
        }
        // THE ATTACK GATE: the press never arms a dig while dead
        app.on_attack();
        assert!(!app.dig.is_destroying, "the dead hand never mines");
        // THE RESPAWN: the vitals reset, the body re-anchors to the spawn
        // pose (the vanilla respawn's own teleport), the fall clears
        if let Some(b) = app.body.as_mut() {
            b.x += 3.0;
            b.fall_distance = 4.0;
        }
        app.respawn();
        assert!(!app.dead, "the death screen closed");
        assert_eq!(app.health, 20.0, "the vitals back to max");
        assert_eq!(app.invulnerable_time, 0);
        let b = app.body.as_ref().expect("the body stands");
        assert_eq!(b.x, spawn.0, "the feet re-anchored to the spawn pose");
        assert_eq!(b.y, spawn.1);
        assert_eq!(b.z, spawn.2);
        assert_eq!(b.fall_distance, 0.0, "the fall state cleared");
    }


    /// THE WIRE'S VITALS LANE (the Set Health / Hurt Animation / Combat
    /// Kill projections through live_health_updated): the store's truth
    /// lands verbatim, the death message arms the death gate, the
    /// respawn wire rides the command channel.
    #[test]
    fn the_wire_vitals_land_and_the_respawn_echoes() {
        let mut app = App::new(3);
        app.enter_world();
        // the vitals snapshot lands verbatim
        app.live_health_updated(crate::world_view::HealthView {
            health: 6.5,
            food: 18,
            saturation: 5.0,
            hurt_time: 10,
            hurt_yaw: -45.5,
            death_message: None,
            effects: Vec::new(),
        mining_efficiency: 0.0,
        block_break_speed: 1.0,
        submerged_mining_speed: 0.2,
        });
        assert_eq!(app.health, 6.5, "the half heart's own float");
        assert_eq!(app.food_data.food_level(), 18);
        assert_eq!(app.food_data.saturation(), 5.0);
        assert_eq!(app.hurt_time, 10, "the flash window rides");
        assert_eq!(app.hurt_yaw, -45.5, "the tilt yaw rides");
        assert!(!app.dead);
        // the death message arms the gate (the wire's own death)
        app.live_health_updated(crate::world_view::HealthView {
            health: 0.0,
            food: 0,
            saturation: 0.0,
            hurt_time: 0,
            hurt_yaw: 0.0,
            death_message: Some("Player fell from a high place".to_string()),
            effects: Vec::new(),
        mining_efficiency: 0.0,
        block_break_speed: 1.0,
        submerged_mining_speed: 0.2,
        });
        assert!(app.dead, "the message armed the death gate");
        assert_eq!(
            app.death_message.as_deref(),
            Some("Player fell from a high place"),
            "the cause line's own text"
        );
        // the respawn echo rides the command channel when a session
        // exists (the offline lane's channel is none — the no-op)
        let (tx, rx) = std::sync::mpsc::channel();
        app.live_clicks = Some(tx);
        app.respawn();
        match rx.try_recv().expect("the respawn echoed") {
            crate::world_view::ContainerCommand::Respawn => {}
            other => panic!("the wrong command rode the lane: {other:?}"),
        }
        assert!(!app.dead);
    }


    /// THE BREEDING WALK'S OWN TICK LANE (the canMate pair the scan
    /// answers end to end): the two in-love sheep the feed verdicts
    /// armed — the court the goal's own 60-tick walk the completed
    /// count stands in for (the approach the feed-and-leave lane's
    /// own legs walk off-camera) — the pairing fires on the app's own
    /// update, the child the spawn walk leaves (the -24000 the baby's
    /// own word), the parents the finalize walk cools (the 6000 the
    /// breed delay speaks), the HEARTS the event 18 broadcasts ride
    /// the particle store, the XP orb the MOB_DROPS roll owes (the
    /// zero roll the honest absence keeps).
    #[test]
    fn the_breeding_pair_spawns_the_child_through_the_tick_lane() {
        let mut app = App::new(5);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y;
        for (dx, id) in [(0i32, 1i32), (2, 2)] {
            let mut sheep = crate::mob::Zombie::new(
                id,
                c as f64 + 0.5 + dx as f64,
                surface as f64,
                c as f64 + 0.5,
                262 + id as u64,
            );
            sheep.kind = crate::mob::MobKind::Sheep;
            sheep.on_ground = true;
            sheep.in_love = 600; // the setInLove verdict's own word
            // THE COURT the goal's own walk completed (the loveTime 60
            // the BreedGoal's own gate reads — the pair the approach
            // walk courted, the meet the spawn's own tick answers)
            sheep.breed_love_time = crate::mob::BREED_COURT_TICKS;
            app.mobs.push(sheep);
        }
        let mobs_before = app.mobs.len();
        app.update();
        assert_eq!(
            app.mobs.len(),
            mobs_before + 1,
            "the child the pairing spawns"
        );
        let child = app
            .mobs
            .iter()
            .find(|m| m.is_baby())
            .expect("the baby the birth arms");
        assert_eq!(
            child.age, crate::mob::BABY_START_AGE,
            "getBabyStartAge the child's own clock"
        );
        assert_eq!(child.kind, crate::mob::MobKind::Sheep);
        // the parents the finalize walk cools (the 6000 both)
        let parents: Vec<&crate::mob::Zombie> = app
            .mobs
            .iter()
            .filter(|m| m.age == crate::mob::BREED_COOLDOWN_TICKS)
            .collect();
        assert_eq!(parents.len(), 2, "the cooled pair the walk speaks");
        assert!(parents.iter().all(|m| m.in_love == 0), "resetLove both");
        // the hearts the event 18 broadcast spawns ride the store
        assert!(
            app.particles.iter().any(|p| p.kind == crate::particles::ParticleKind::Heart),
            "the burst the broadcast owes"
        );
    }


    /// THE DAY BEAT'S OWN CREATURE ARM (the spawner's else arm the
    /// bright-outside word arms — the pack the beat lands, the monster
    /// lane the night word keeps, the procedural world the animals now
    /// graze).
    #[test]
    fn the_day_beat_walks_the_creatures() {
        let mut app = App::new(9);
        app.enter_world();
        // THE FLAT GRASS PLATFORM (the world's own voxels the test
        // sets — the walk's every spot the platform keeps)
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        {
            let w = app.offline_world.as_mut().unwrap();
            for x in 0..size {
                for z in 0..size {
                    for y in 0..8 {
                        w.set(x, y, z, crate::world::STONE);
                    }
                    w.set(x, 8, z, crate::world::GRASS);
                    for y in 9..size {
                        w.set(x, y, z, crate::world::AIR);
                    }
                }
            }
        }
        app.body = Some(crate::body::PlayerBody::new(40.0, 9.0, 40.0, 0.0, 0.0));
        // THE DAY BAND (the gate's own word — the darken's first
        // bright tick the beat's creature arm keeps)
        let day_tick = (0..24_000i64)
            .find(|t| crate::mob::is_bright_outside(*t))
            .expect("a day band the sky keeps");
        app.clock_total = day_tick as u64 + app.frame;
        // THE BEAT (the spawn_clock the 40th update the % SPAWN_CYCLE
        // arm — the beats the walk repeats, the cap the lane stops)
        for _ in 0..400 {
            app.update();
        }
        // THE PACK THE BEAT LANDED (the friendly kinds the walk
        // speaks, the feet the platform's top rides — the ring the
        // SPAWN-time law the mob.rs unit pins, the AI's own stroll the
        // landed spot the vanilla drift walks)
        let creatures: Vec<&crate::mob::Zombie> =
            app.mobs.iter().filter(|m| !m.kind.is_monster()).collect();
        assert!(
            !creatures.is_empty(),
            "the day beat the creatures land"
        );
        for m in &creatures {
            assert_ne!(m.kind, crate::mob::MobKind::Zombie, "the friendly lane");
            assert_eq!(m.y, 9.0, "the feet the platform's top keeps");
        }
    }


    /// THE MORNING BEAT'S OWN CREATURE ARM (autopush-425 — the
    /// discriminating flip the bright-outside word wears): the tick
    /// 1_000 the sun's sin the OLD gate's MONSTER arm rode (sin(angle)
    /// < 0 spans midnight->noon) yet the darken law the BRIGHT word
    /// keeps — the morning the creature lane deserves the beat now
    /// arms (the old gate the monster arm rode, the stage-3 light
    /// mask the roster never filled).
    #[test]
    fn the_morning_beat_walks_the_creatures() {
        let mut app = App::new(9);
        app.enter_world();
        // THE FLAT GRASS PLATFORM (the day beat's own voxels the test
        // sets — the walk's every spot the platform keeps)
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        {
            let w = app.offline_world.as_mut().unwrap();
            for x in 0..size {
                for z in 0..size {
                    for y in 0..8 {
                        w.set(x, y, z, crate::world::STONE);
                    }
                    w.set(x, 8, z, crate::world::GRASS);
                    for y in 9..size {
                        w.set(x, y, z, crate::world::AIR);
                    }
                }
            }
        }
        app.body = Some(crate::body::PlayerBody::new(40.0, 9.0, 40.0, 0.0, 0.0));
        // THE MORNING TICK (the bright-outside word keeps — the old
        // gate's sin the MONSTER arm the same tick rode)
        assert!(
            crate::mob::is_bright_outside(1_000),
            "the morning the bright word keeps"
        );
        assert!(
            crate::sky::sample(1_000).sun_angle.sin() < 0.0,
            "the old gate's sin the morning the monster arm rode"
        );
        app.clock_total = 1_000 + app.frame;
        // THE BEAT (the day beat's own 400-update walk — the creature
        // arm the whole window the bright word keeps)
        for _ in 0..400 {
            app.update();
        }
        let creatures: Vec<&crate::mob::Zombie> =
            app.mobs.iter().filter(|m| !m.kind.is_monster()).collect();
        assert!(
            !creatures.is_empty(),
            "the morning beat the creatures land"
        );
        for m in &creatures {
            assert_ne!(m.kind, crate::mob::MobKind::Zombie, "the friendly lane");
            assert_eq!(m.y, 9.0, "the feet the platform's top keeps");
        }
    }


    /// THE FALLING BLOCK'S OWN ROUND TRIP (dev-35 — the schedule the
    /// dig's neighbor change stands in, the 2-tick delay the
    /// getDelayAfterPlace law keeps, the conversion the fall factory
    /// wears, the landing the placement conjunct serves: the block
    /// re-forms on the floor the canBeReplaced(into) && !isFree(below)
    /// pair answers)
    #[test]
    fn the_falling_block_the_schedule_the_fall_the_landing_wears() {
        let mut app = App::new(9);
        let size = falling_platform(&mut app);
        let c = size / 2;
        // THE SAND IN THE AIR (the below cell the honest free keeps)
        app.offline_world.as_mut().unwrap().set(c, 10, c, crate::world::SAND);
        app.schedule_falling_check(c, 10, c);
        // THE 2-TICK DELAY (the first update the pending window keeps)
        app.update();
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, 10, c),
            crate::world::SAND,
            "the first update the pending window keeps"
        );
        // THE CONVERSION (the due tick the fall factory wears)
        for _ in 0..3 {
            app.update();
        }
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, 10, c),
            crate::world::AIR,
            "the conversion the cell's own air wears"
        );
        assert_eq!(
            app.falling_blocks.len(),
            1,
            "the fall factory the entity keeps"
        );
        assert!(
            app.falling_blocks[0].id >= 1,
            "the id lane the entity counter banks"
        );
        assert_eq!(
            app.falling_blocks[0].block,
            crate::world::SAND,
            "the kind the carried state keeps"
        );
        // THE LANDING (the floor the placement conjunct answers)
        for _ in 0..60 {
            app.update();
        }
        assert!(
            app.falling_blocks.is_empty(),
            "the landed body the list retires"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, 9, c),
            crate::world::SAND,
            "the landing the block re-forms"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, 8, c),
            crate::world::GRASS,
            "the floor the support keeps"
        );
    }


    /// THE FALLING'S OWN SUPPORT LAW (the isFree(below) the scheduled
    /// tick reads — the floor-held sand the conversion refuses, the
    /// schedule the honest consumed entry keeps)
    #[test]
    fn the_falling_block_the_support_the_conversion_refuses() {
        let mut app = App::new(9);
        let size = falling_platform(&mut app);
        let c = size / 2;
        // THE SAND ON THE FLOOR (the grass below the not-free hold keeps)
        app.offline_world.as_mut().unwrap().set(c, 9, c, crate::world::SAND);
        app.schedule_falling_check(c, 9, c);
        for _ in 0..8 {
            app.update();
        }
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, 9, c),
            crate::world::SAND,
            "the floor the sand holds"
        );
        assert!(
            app.falling_blocks.is_empty(),
            "the held block the entity never becomes"
        );
        assert!(
            app.falling_checks.is_empty(),
            "the due entry the honest consume keeps"
        );
    }


    /// THE FALLING COLUMN'S OWN CHAIN (the conversion's above-cell
    /// schedule the shape-update law re-arms — the column the cascade
    /// wears, the re-stack the landing conjunct serves: the staggered
    /// falls the bottom-up order the same physics keeps)
    #[test]
    fn the_falling_column_the_chain_the_collapse_wears() {
        let mut app = App::new(9);
        let size = falling_platform(&mut app);
        let c = size / 2;
        // THE COLUMN (the three sand the grass floor holds)
        for y in 9..12 {
            app.offline_world.as_mut().unwrap().set(c, y, c, crate::world::SAND);
        }
        // THE DIG'S OWN STAND-IN (the floor cell's removal the support
        // lost wears — the schedule the neighbor change arms)
        app.offline_world.as_mut().unwrap().set(c, 8, c, crate::world::AIR);
        app.schedule_falling_check(c, 9, c);
        // THE CASCADE (the each-own-schedule the chain wears — the
        // bottom the fall's 7-tick reach never crosses inside the
        // window: the three bodies the same physics keeps)
        for _ in 0..8 {
            app.update();
        }
        assert_eq!(
            app.falling_blocks.len(),
            3,
            "the cascade the three bodies keeps"
        );
        // THE RE-STACK (the stone at 7 the floor the entities land on)
        for _ in 0..120 {
            app.update();
        }
        assert!(
            app.falling_blocks.is_empty(),
            "the landed bodies the list retires"
        );
        for y in 8..11 {
            assert_eq!(
                app.offline_world.as_ref().unwrap().get(c, y, c),
                crate::world::SAND,
                "the re-stack the y{y} cell wears"
            );
        }
    }


    /// THE FALLING'S OWN TORCH LAW (the torch the falling body passes
    /// THROUGH — the Empty collision the entity never rests on — the
    /// landing cell the torch's own kind keeps: the canBeReplaced
    /// refusal the item drop branch serves, the torch the survival
    /// keeps, the block the item the asItem() law speaks — the loot
    /// table's flint roll the gravel walk never reads). The STATIC
    /// sand the torch holds the isFree refusal keeps (the scheduled
    /// tick the below's not-free verdict reads — the vanilla quirk the
    /// higher column the own free-below fall wears): the test the
    /// FALLING body's own path arms (the sand the three air cells
    /// above the torch the conversion's free-below law admits).
    #[test]
    fn the_falling_block_the_torch_the_item_drop_wears() {
        let mut app = App::new(9);
        let size = falling_platform(&mut app);
        let c = size / 2;
        // THE TORCH UNDER THE FALL PATH (the floor-held torch the
        // famous pair wears, the sand the air column above it the
        // conversion's isFree law admits)
        app.offline_world.as_mut().unwrap().set(c, 9, c, crate::world::TORCH);
        app.offline_world.as_mut().unwrap().set(c, 12, c, crate::world::SAND);
        // THE BODY'S OWN RETREAT (the drop's spawn cell the falling
        // path's own floor keeps — the player's pickup arc the far
        // pose never rides)
        app.body = Some(crate::body::PlayerBody::new(5.0, 9.0, 5.0, 0.0, 0.0));
        app.schedule_falling_check(c, 12, c);
        for _ in 0..80 {
            app.update();
        }
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(w.get(c, 9, c), crate::world::TORCH, "the torch the survival keeps");
        assert_eq!(w.get(c, 12, c), crate::world::AIR, "the sand the cell left");
        assert_eq!(w.get(c, 8, c), crate::world::GRASS, "the floor untouched");
        assert!(
            app.falling_blocks.is_empty(),
            "the broken body the list retires"
        );
        // THE ITEM DROP (the spawnAtLocation walk — the single stack
        // the entity's own pose rides)
        assert_eq!(app.drops.len(), 1, "the drop branch the item wears");
        assert_eq!(
            app.drops[0].item,
            crate::mob::falling_item_id(crate::world::SAND).unwrap(),
            "the sand item the wire order keeps"
        );
        assert_eq!(app.drops[0].count, 1, "the single stack the law spawns");
    }


    /// THE POWDER'S OWN UPDATE-SHAPE ARM (javap ConcretePowderBlock.
    /// updateShape over the 26.2 jar — the `if (touchesLiquid(level,
    /// pos)) return concrete.defaultBlockState()` early return the
    /// scheduled walk serves): the water ABOVE the scheduled powder the
    /// CONCRETE swap speaks (the fall the solidified block never
    /// becomes — the falling family the is_falling_block law refuses),
    /// THE DOWN GATE the water-below-only case the conversion refuses
    /// (the vanilla's own famous asymmetry — the below water the fall
    /// walk serves instead: the water the free-below verdict the body
    /// spawns).
    #[test]
    fn the_powder_the_schedule_the_solidify_the_fall_refuses() {
        let mut app = App::new(9);
        let size = falling_platform(&mut app);
        let c = size / 2;
        // THE WATER ABOVE: the scheduled powder the touchesLiquid walk
        // answers — the CONCRETE the cell wears, no body, no fall
        app.offline_world.as_mut().unwrap().set(c, 10, c, crate::world::CONCRETE_POWDER);
        app.offline_world.as_mut().unwrap().set(c, 11, c, crate::world::WATER);
        app.schedule_falling_check(c, 10, c);
        for _ in 0..8 {
            app.update();
        }
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, 10, c),
            crate::world::CONCRETE,
            "the water above the conversion speaks"
        );
        assert!(
            app.falling_blocks.is_empty(),
            "the solidified block the fall never becomes"
        );
        // THE WATER BELOW ONLY: the DOWN gate the scheduled walk's own
        // conversion refuses (the vanilla's own famous asymmetry — the
        // below water the SOLIDIFY never speaks) — the fall walk the
        // body spawns, and the BODY'S OWN mid-water verdict the pool
        // cell the concrete wears (the scheduled walk the cell the
        // powder kept — the AIR the fall factory left, the CONCRETE the
        // body's own serve speaks)
        let c2 = c + 2;
        app.offline_world.as_mut().unwrap().set(c2, 12, c2, crate::world::CONCRETE_POWDER);
        app.offline_world.as_mut().unwrap().set(c2, 11, c2, crate::world::WATER);
        app.schedule_falling_check(c2, 12, c2);
        for _ in 0..8 {
            app.update();
        }
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c2, 12, c2),
            crate::world::AIR,
            "the gate's refusal the fall speaks (the scheduled walk the powder never swapped)"
        );
        assert_eq!(
            w.get(c2, 11, c2),
            crate::world::CONCRETE,
            "the body's own mid-water verdict the pool cell wears"
        );
        assert!(
            app.falling_blocks.is_empty(),
            "the converted body the list retires"
        );
    }


    /// THE POWDER'S OWN LANDING SERVE (javap FallingBlockEntity.tick pc
    /// 241-529 over the 26.2 jar — the onGround||converts arm the
    /// mid-water verdict fires, the placement conjunct's break clause
    /// `isFree(below) && (!isConcrete || !converts)` the convert
    /// verdict kills, the setBlock(3) + the Fallable.onLand walk the
    /// shouldSolidify verdict the CONCRETE state speaks): the powder
    /// the pool's own cell the conversion wears — the CONCRETE the
    /// water cell re-forms, the item drop the branch never speaks, the
    /// schedule the landed cell's own dispatch re-arms.
    #[test]
    fn the_powder_the_landing_the_concrete_the_water_wears() {
        let mut app = App::new(9);
        let size = falling_platform(&mut app);
        let c = size / 2;
        // THE POOL CELL above the grass floor: the powder scheduled
        // three cells above (the dry air the fall keeps — the DOWN
        // gate the scheduled walk's own scan refuses, the body the
        // fall spawns)
        app.offline_world.as_mut().unwrap().set(c, 9, c, crate::world::WATER);
        app.offline_world.as_mut().unwrap().set(c, 12, c, crate::world::CONCRETE_POWDER);
        app.schedule_falling_check(c, 12, c);
        for _ in 0..60 {
            app.update();
        }
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, 9, c),
            crate::world::CONCRETE,
            "the water cell the concrete re-forms"
        );
        assert_eq!(
            w.get(c, 12, c),
            crate::world::AIR,
            "the origin cell the fall factory left"
        );
        assert!(
            app.falling_blocks.is_empty(),
            "the converted body the list retires"
        );
        assert_eq!(
            app.drops.len(),
            0,
            "the break clause the drop branch never speaks"
        );
    }



    /// THE POWDER'S OWN PLACEMENT VERDICT (javap BlockItem.place over
    /// the 26.2 jar — the getPlacementState walk the placement chain
    /// keeps before the canPlace gates: the shouldSolidify read the
    /// clicked cell's own state answers): the powder item INTO the
    /// water cell the CONCRETE places (the canSolidify arm — the
    /// replaceable clicked cell the placement target keeps), the dry
    /// cell the POWDER keeps, the cell BESIDE the pool the
    /// touchesLiquid arm the same conversion wears.
    #[test]
    fn the_powder_the_placement_the_solidify_arm_wears() {
        let mut app = App::new(3);
        let size = falling_platform(&mut app);
        let c = size / 2;
        // THE CAMERA'S OWN AUTHORITY (the body the platform staged the
        // world_camera's highest word speaks — the aim the offline
        // camera drives: the body the arena drops, the deterministic
        // scenes' own shape the press reads)
        app.body = None;
        app.offline_held_updated(Some("item/white_concrete_powder"));
        // THE POOL: the two water cells atop the grass floor (the into
        // aim converts the first, the beside aim reads the second)
        app.offline_world.as_mut().unwrap().set(c + 1, 9, c + 1, crate::world::WATER);
        app.offline_world.as_mut().unwrap().set(c + 1, 9, c + 2, crate::world::WATER);
        // THE INTO-WATER AIM: the straight-down ray the water cell the
        // Fluid-NONE clip passes through, the grass below the clicked
        // cell speaks, the UP-face offset the water cell is — the
        // canSolidify(the water state) the CONCRETE answers
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 1.5, 13.0, c as f32 + 1.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(
            app.on_right_press(0, 0),
            "the placement consumed the press"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c + 1, 9, c + 1),
            crate::world::CONCRETE,
            "the water cell the concrete places"
        );
        // THE DRY AIM: the plain grass column the offset cell the air
        // keeps — the powder the placement wears (the solidify verdict
        // the dry cell refuses)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 5.5, 13.0, c as f32 + 5.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(
            app.on_right_press(0, 0),
            "the dry placement consumed the press"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c + 5, 9, c + 5),
            crate::world::CONCRETE_POWDER,
            "the dry cell the powder keeps"
        );
        // THE BESIDE-WATER AIM: the offset cell the pool's own
        // neighbor wears — the touchesLiquid arm the CONCRETE speaks
        // (the empty target the state read refuses, the second pool
        // cell the side water the scan arms)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 2.5, 13.0, c as f32 + 2.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(
            app.on_right_press(0, 0),
            "the beside placement consumed the press"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c + 2, 9, c + 2),
            crate::world::CONCRETE,
            "the touching cell the conversion wears"
        );
    }


    /// THE NIGHT BEAT'S OWN PEACEFUL GATE (javap the 26.2 jar the
    /// `SpawnPlacements.checkSpawnRules` walk speaks): the PEACEFUL
    /// world the MONSTER spawn the gate refuses — 400 updates' worth
    /// of night beats the roster NEVER fills (the ring's own randoms
    /// the gate precedes, the honest silence the lane keeps); the
    /// NORMAL world the same beats the monster lane answers (the
    /// vanilla's own spawn the dial's other arms wear).
    #[test]
    fn the_night_beat_wears_the_peaceful_gate() {
        // THE PEACEFUL ARM (the dial's own refusal)
        let mut app = App::new(9);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        {
            let w = app.offline_world.as_mut().unwrap();
            for x in 0..size {
                for z in 0..size {
                    for y in 0..8 {
                        w.set(x, y, z, crate::world::STONE);
                    }
                    w.set(x, 8, z, crate::world::GRASS);
                    for y in 9..size {
                        w.set(x, y, z, crate::world::AIR);
                    }
                }
            }
        }
        app.offline_world.as_mut().unwrap().difficulty =
            crate::difficulty::Difficulty::Peaceful;
        app.body = Some(crate::body::PlayerBody::new(40.0, 9.0, 40.0, 0.0, 0.0));
        // THE NIGHT BAND (the gate's own word — the darken's first
        // dark tick the beat's monster arm keeps)
        let night_tick = (0..24_000i64)
            .find(|t| !crate::mob::is_bright_outside(*t))
            .expect("a night band the sky keeps");
        app.clock_total = night_tick as u64 + app.frame;
        for _ in 0..400 {
            app.update();
        }
        assert!(
            app.mobs.is_empty(),
            "the PEACEFUL world the monster spawn the gate refuses ({} landed)",
            app.mobs.len()
        );
        // THE NORMAL ARM (the same drive the default dial rides — the
        // beats the monster lane answers, the spawn the gate only the
        // peaceful arm refuses)
        let mut app = App::new(9);
        app.enter_world();
        {
            let w = app.offline_world.as_mut().unwrap();
            for x in 0..size {
                for z in 0..size {
                    for y in 0..8 {
                        w.set(x, y, z, crate::world::STONE);
                    }
                    w.set(x, 8, z, crate::world::GRASS);
                    for y in 9..size {
                        w.set(x, y, z, crate::world::AIR);
                    }
                }
            }
        }
        app.body = Some(crate::body::PlayerBody::new(40.0, 9.0, 40.0, 0.0, 0.0));
        // THE TRACK'S OWN MIDNIGHT (the darken's plateau the gate's
        // stage 3 subtracts — 18_500 the skyDarken 11 the beat's
        // monster arm the bright-outside word keeps)
        app.clock_total = 18_500 + app.frame;
        // the beats the gate's own texture absorbs (the midnight
        // surface the raw 4 the uniform's upper half admits — the
        // 60-beat band the spawn the vanilla texture lands)
        for _ in 0..2400 {
            app.update();
        }
        assert!(
            !app.mobs.is_empty(),
            "the NORMAL world the night beat the monster lane answers"
        );
    }


    /// THE PEACEFUL'S OWN DISCARD (javap the 26.2 jar the
    /// `Mob.checkDespawn` walk speaks): the PEACEFUL world the MONSTER
    /// roster DISCARDS every tick — the plain removal the walk speaks
    /// (no death clock, no poof cloud, no loot, no voice), the
    /// CREATURE roster the same walk keeps (the animals the peaceful
    /// world wears); the NORMAL world the same roster the walk never
    /// touches.
    #[test]
    fn the_peaceful_discard_clears_the_monster_roster() {
        let mut app = App::new(9);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let seed_pair = [
            (crate::mob::MobKind::Zombie, 1),
            (crate::mob::MobKind::Skeleton, 2),
            (crate::mob::MobKind::Cow, 3),
            (crate::mob::MobKind::Sheep, 4),
        ];
        for (kind, id) in seed_pair {
            let mut m = crate::mob::Zombie::new(
                id,
                c as f64 + 0.5,
                app.spawn_y as f64,
                c as f64 + 0.5,
                id as u64 * 7,
            );
            m.kind = kind;
            app.mobs.push(m);
        }
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            app.spawn_y as f64,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        // THE NORMAL ARM (the default dial — the roster the walk
        // never touches)
        app.update();
        assert_eq!(
            app.mobs.len(),
            4,
            "the NORMAL world the roster the walk keeps"
        );
        // THE PEACEFUL ARM (the dial's turn — the MONSTER kinds the
        // discard answers, the CREATURE kinds the same walk keeps)
        app.offline_world.as_mut().unwrap().difficulty =
            crate::difficulty::Difficulty::Peaceful;
        app.update();
        assert!(
            app.mobs
                .iter()
                .all(|m| m.kind.mob_category() != crate::mob::MobCategory::Monster),
            "the PEACEFUL walk the MONSTER kinds discards"
        );
        assert_eq!(app.mobs.len(), 2, "the CREATURE roster the walk keeps");
        assert!(
            app.mobs
                .iter()
                .any(|m| m.kind == crate::mob::MobKind::Cow)
                && app
                    .mobs
                    .iter()
                    .any(|m| m.kind == crate::mob::MobKind::Sheep),
            "the cow and the sheep the peaceful world wears"
        );
    }


    /// THE AMBIENT GROAN'S OWN KIND (the voice table the KIND's own
    /// row keeps — the cow's moo the zombie's row never named, the
    /// lane the creature census now arms).
    #[test]
    fn the_ambient_groan_speaks_the_kind_voice() {
        let mut app = App::new(11);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let mut cow = crate::mob::Zombie::new(
            7,
            c as f64 + 0.5,
            app.spawn_y as f64,
            c as f64 + 0.5,
            3,
        );
        cow.kind = crate::mob::MobKind::Cow;
        cow.on_ground = true;
        cow.ambient_time = 2000; // the draw the gate's own bound exceeds
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            app.spawn_y as f64,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        app.mobs.push(cow);
        app.update();
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("entity.cow.ambient ")),
            "the cow's own row the groan speaks"
        );
    }


    /// THE FRESH WORLD'S OWN ENTITY SWAP (the rebuild's own sweep the
    /// ClientLevel's map answers — the old session's entities the new
    /// terrain never inherits; the second entry the same world's own
    /// cargo the guard keeps).
    #[test]
    fn the_world_rebuild_sweeps_the_stale_entities() {
        let mut app = App::new(13);
        app.enter_world();
        // THE OLD SESSION'S OWN CARGO (the entities the stale world
        // carried: the mobs the wire's lane, the ride belief, the
        // drops the orbs the particles, the dig the pick)
        let mut z = crate::mob::Zombie::new(1, 40.5, 9.0, 40.5, 7);
        z.kind = crate::mob::MobKind::Pig;
        app.mobs.push(z);
        app.net_mobs.push(crate::mob::Zombie::new(2, 41.5, 9.0, 41.5, 8));
        app.net_ride = Some(crate::world_view::WireRide {
            vehicle_id: 9,
            feet_x: 40.0,
            feet_y: 9.0,
            feet_z: 40.0,
            vehicle_yaw: 0.0,
            seat_x: 0.0,
            ride_height: 0.9,
        });
        let stick_id =
            mc_registry::item_id_by_name("stick").expect("the stick the table draws");
        app.drops.push(crate::drops::ItemDrop::spawn_entity_loot(
            1,
            stick_id,
            1,
            40.0,
            9.0,
            40.0,
            &mut app.mob_rng,
        ));
        app.orbs.push(crate::xp::ExperienceOrb::spawn(1, 40.0, 9.0, 40.0, 3));
        app.particles.push(crate::particles::heart(
            1.0, 2.0, 3.0, 0.02, -0.03, 0.04, 0.5, 0.0,
        ));
        app.dig.is_destroying = true;
        app.dig.pos = (1, 2, 3);
        app.frame_pick = Some(crate::world::ClipHit {
            pos: (1, 2, 3),
            face: 1,
            location: (1.0, 2.0, 3.0),
            inside: false,
        });
        // THE REBUILD (the wire flag the swap's own guard arms — the
        // sweep the fresh terrain's own honesty speaks)
        app.world_is_network = true;
        app.enter_world();
        assert!(app.mobs.is_empty(), "the mobs the level drop keeps");
        assert!(app.net_mobs.is_empty(), "the wire bodies the swap drops");
        assert!(app.net_ride.is_none(), "the gone hull the ride belief drops");
        assert!(app.drops.is_empty(), "the items the old world drops");
        assert!(app.orbs.is_empty(), "the orbs the old world drops");
        assert!(app.particles.is_empty(), "the weather the old world drops");
        assert!(!app.dig.is_destroying, "the dig the old voxels reset");
        assert!(app.frame_pick.is_none(), "the pick the old terrain drops");
        // THE SECOND ENTRY KEEPS THE LIVING WORLD (the chest flow's own
        // return — the guard the rebuild never re-arms, the entities
        // the same world's own session carries)
        let mut z2 = crate::mob::Zombie::new(3, 40.5, 9.0, 40.5, 9);
        z2.kind = crate::mob::MobKind::Cow;
        app.mobs.push(z2);
        app.enter_world();
        assert_eq!(
            app.mobs.len(),
            1,
            "the same world the entities keep"
        );
    }


    /// THE FOLLOWPARENTGOAL'S OWN WALK (the tick lane the family
    /// answers end to end): the baby the caller's pre-pass scan arms
    /// (the nearest adult the inflate ring keeps), the ticks walk the
    /// straight-line stand-in — the distance the family's own walk
    /// speaks, the close ring the 3.0 the disarm keeps.
    #[test]
    fn the_babys_follow_walks_the_family_through_the_tick_lane() {
        let mut app = App::new(5);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y;
        // the adult the parent speaks — 5 blocks east of the baby
        let mut adult = crate::mob::Zombie::new(
            1,
            c as f64 + 0.5 + 5.0,
            surface as f64,
            c as f64 + 0.5,
            262,
        );
        adult.kind = crate::mob::MobKind::Sheep;
        adult.on_ground = true;
        let mut baby = crate::mob::Zombie::new(
            2,
            c as f64 + 0.5,
            surface as f64,
            c as f64 + 0.5,
            263,
        );
        baby.kind = crate::mob::MobKind::Sheep;
        baby.age = crate::mob::BABY_START_AGE;
        baby.on_ground = true;
        app.mobs.push(adult);
        app.mobs.push(baby);
        let d0 = 25.0f64; // the 5.0 the squared ring speaks
        for _ in 0..20 {
            app.update();
        }
        let baby_now = app
            .mobs
            .iter()
            .find(|m| m.is_baby())
            .expect("the baby the birth arms");
        let adult_now = app
            .mobs
            .iter()
            .find(|m| m.age >= 0)
            .expect("the adult the scan armed");
        let dx = adult_now.x - baby_now.x;
        let dz = adult_now.z - baby_now.z;
        let d1 = dx * dx + dz * dz;
        assert!(
            d1 < d0 - 1.0,
            "the family's own walk the distance speaks ({d1} vs {d0})"
        );
    }


    /// THE BREEDGOAL'S OWN WALK (the tick lane the pair answers end to
    /// end): the in-love pair the caller's pre-pass scan arms (the
    /// nearest mate the inflate ring keeps), the ticks walk the
    /// straight-line stand-in — the court's own clock counts, the
    /// approach the distance closes, the meet the close ring answers,
    /// the child the spawn walk leaves, the parents the finalize walk
    /// cools.
    #[test]
    fn the_breeders_walk_the_pair_through_the_tick_lane() {
        let mut app = App::new(5);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y;
        // the pair the feed-and-leave walk leaves — 5 blocks apart,
        // both in love, the court the real lane's own walk owes (the
        // player the body-less lane never passes — the goal-else chain
        // the no-target rest keeps)
        for (i, dx) in [0.0f64, 5.0].into_iter().enumerate() {
            let mut sheep = crate::mob::Zombie::new(
                1 + i as i32,
                c as f64 + 0.5 + dx,
                surface as f64,
                c as f64 + 0.5,
                262 + i as u64,
            );
            sheep.kind = crate::mob::MobKind::Sheep;
            sheep.on_ground = true;
            sheep.in_love = 600;
            app.mobs.push(sheep);
        }
        // the court's own 60 the walk runs through (the approach the
        // pair's own legs close well inside the ring) + the margin the
        // meet's own tick rides
        for _ in 0..90 {
            app.update();
        }
        assert_eq!(
            app.mobs.len(),
            3,
            "the child the walked court answers (the third the spawn leaves)"
        );
        let baby = app
            .mobs
            .iter()
            .find(|m| m.is_baby())
            .expect("the baby the birth arms");
        assert_eq!(
            baby.kind,
            crate::mob::MobKind::Sheep,
            "the child the same kind names"
        );
        let cooled = app
            .mobs
            .iter()
            .filter(|m| !m.is_baby())
            .all(|m| m.in_love == 0);
        assert!(cooled, "the walked pair the finalize walk cools");
    }


    /// THE BLOCK-SOUND ARC'S FOOTSTEPS: the walking body's steps ride
    /// Entity.move's own cadence — moveDist grows by the horizontal
    /// movement × 0.6, a step fires per whole moveDist unit over a solid
    /// block under the feet — and the step voice speaks the block under
    /// at Entity.playStepSound's own fold (v*0.15, the pitch untouched).
    #[test]
    fn the_walking_body_speaks_the_footstep_cadence() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.on_movement_input(1.0, 0.0, false, false, false);
        for _ in 0..40 {
            app.update();
        }
        let b = app.body.as_ref().unwrap();
        assert!(
            b.move_dist > 2.0,
            "the walk accumulated the vanilla law ({})",
            b.move_dist
        );
        let steps: Vec<&String> = app
            .sounds
            .events
            .iter()
            .filter(|e| e.contains(".step @"))
            .collect();
        // the walk crosses the lakebed's own slope (the honest seed-3
        // terrain): the airborne crossings stay silent — the vanilla
        // isAir gate — so the count never EXCEEDS the whole units.
        // THE MIGRATION'S OWN LAW (the assert computes the expected from
        // the ACTUAL loaded state): the cache-absent boot the empty table
        // the silent steps keep — the cadence walk below speaks where the
        // cache exists
        if snd::events().is_empty() {
            assert!(
                steps.is_empty(),
                "the empty table the silent steps keep: {:?}",
                steps
            );
            return;
        }
        assert!(
            !steps.is_empty() && steps.len() <= b.move_dist as i32 as usize,
            "the steps rode the cadence ({} steps for moveDist {})",
            steps.len(),
            b.move_dist
        );
        for s in &steps {
            assert!(
                s.contains(" v0.150000 p1.000000"),
                "the step fold on every entry: {s}"
            );
        }
        // the events the steps named are real sound events (THE MIGRATION'S
        // OWN LAW: the assert computes the expected from the ACTUAL loaded
        // state — the cache-absent boot the silent steps keep)
        use crate::sound_table as snd;
        let loaded = !snd::events().is_empty();
        for s in &steps {
            let ev = s.split(" @").next().unwrap();
            assert_eq!(
                snd::event(ev).is_some(),
                loaded,
                "the step event exists where the table speaks: {ev}"
            );
        }
        // the body rested on solid ground the whole walk (the lakebed
        // spawn's own honest floor) — no floating step possible
        assert!(b.on_ground, "the walk stayed on the terrain");
    }


    /// THE DIG LANE's NETWORKED ARC — the destroy state arms and mines
    /// over the net world, the START/ABORT echoes ride the command
    /// channel (the worker's serverbound Player Action packets), the
    /// SWING rides its own command, and the LOCAL voxels stay untouched
    /// (the server's own chunk refresh is the authority on this lane).
    #[test]
    fn the_dig_lane_echoes_the_networked_world() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        // the net world with a stone floor under the spawn column
        let mut net = air_net_world(2, 0);
        let c = net.size_blocks() / 2;
        let floor_y = net.size_blocks() / 4;
        for x in 0..net.size_blocks() {
            for z in 0..net.size_blocks() {
                net.set(x, floor_y, z, crate::world::STONE);
            }
        }
        app.net_world = Some(net);
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.net_camera = Some(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, floor_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE PRESS: the START + the swing ride the lane
        app.on_press(0, 0);
        assert!(app.dig.is_destroying, "the net lane armed the destroy");
        match click_rx.try_recv().expect("the START rode the lane") {
            ContainerCommand::DigAction { action, pos, face } => {
                assert_eq!(action, crate::dig::ACTION_START, "the START ordinal");
                assert_eq!(pos, (c, floor_y, c), "the hit voxel");
                assert_eq!(face, 1, "UP — the floor's entry face");
            }
            other => panic!("expected a DigAction START, got {other:?}"),
        }
        match click_rx.try_recv().expect("the swing rode the lane") {
            ContainerCommand::Swing => {}
            other => panic!("expected a Swing, got {other:?}"),
        }
        // THE HOLD: the progress ticks (the hand-on-stone 100 divisor),
        // the local voxels stay the server's own
        for _ in 0..3 {
            app.update();
        }
        assert!(app.dig.progress > 0.0, "the net lane's progress ticks");
        // the mining beats' swings queued on the lane; drain them (the
        // vanilla's own per-beat echo)
        let mut beats = 0;
        while let Ok(cmd) = click_rx.try_recv() {
            match cmd {
                ContainerCommand::Swing => beats += 1,
                other => panic!("unexpected lane traffic mid-dig: {other:?}"),
            }
        }
        assert!(beats >= 1, "the mining beats swung");
        assert_eq!(
            app.net_world.as_ref().unwrap().get(c, floor_y, c),
            crate::world::STONE,
            "no local prediction on the networked lane"
        );
        // THE RELEASE: the ABORT rides DOWN (the vanilla's own constant)
        app.on_left_release();
        match click_rx.try_recv().expect("the ABORT rode the lane") {
            ContainerCommand::DigAction { action, pos, face } => {
                assert_eq!(action, crate::dig::ACTION_ABORT, "the ABORT ordinal");
                assert_eq!(pos, (c, floor_y, c), "the held cell");
                assert_eq!(face, 0, "DOWN — the release's own direction");
            }
            other => panic!("expected a DigAction ABORT, got {other:?}"),
        }
        assert!(!app.dig.is_destroying);
    }


    /// The number keys' whole arc: the HUD's selection moves with the
    /// key, the LIVE session's belief rides the command channel (the
    /// SelectHotbar command the worker translates to the wire AND the
    /// store's own move), the out-of-domain and the repeat drop, and
    /// the offline stand-in keeps the move cosmetic.
    #[test]
    fn the_number_key_moves_the_selection_and_echoes() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);

        // the key moves the HUD's selection and rides the command
        app.on_number_key(4);
        assert_eq!(app.selected_hotbar, 4, "the HUD's own belief moved");
        match click_rx.try_recv().expect("the selection rode the lane") {
            ContainerCommand::SelectHotbar { selection } => {
                assert_eq!(selection, 4, "the SELECTION-relative slot");
            }
            other => panic!("expected a selection, got {other:?}"),
        }

        // the repeat naming the held slot is the honest silence
        app.on_number_key(4);
        assert!(click_rx.try_recv().is_err(), "the repeat sends nothing");

        // the out-of-domain drop (the gate: 0..8, the vanilla
        // Inventory.selected's own range)
        app.on_number_key(9);
        assert_eq!(app.selected_hotbar, 4, "the gate held");
        assert!(click_rx.try_recv().is_err(), "no command either");

        // the chest closes back to the world (the container-less
        // update) — the session LIVES: the key still echoes (the
        // vanilla client's selection is the session's own, not the
        // chest screen's)
        app.live_view_updated(air_net_world(2, 1), None, None, None);
        assert_eq!(app.screen, ScreenKind::World, "the live chest closed");
        app.on_number_key(2);
        assert_eq!(app.selected_hotbar, 2, "the world-screen key moved too");
        match click_rx.try_recv().expect("the world-screen key echoes") {
            ContainerCommand::SelectHotbar { selection } => assert_eq!(selection, 2),
            other => panic!("expected a selection, got {other:?}"),
        }

        // the OFFLINE stand-in: a fresh app in the procedural world —
        // no session, no lane: the selection still moves (the HUD's own
        // lane), the wire stays silent (the live channel above never
        // hears from it)
        let mut offline = App::new(3);
        offline.enter_world();
        offline.on_number_key(2);
        assert_eq!(offline.selected_hotbar, 2, "the offline HUD moves");
        assert!(click_rx.try_recv().is_err(), "the offline wire is silent");
    }


    /// THE SCROLL WHEEL'S HOTBAR CYCLE (the MouseHandler.onScroll's
    /// own walk): the down-scroll moves toward the higher numbers, the
    /// up-scroll back, the wrap the floor-mod keeps (0 scrolls up to
    /// 8), and the held model follows the selection — the offline
    /// walk's own lane, the store's counts untouched, NO swing, NO
    /// drop (the selection the scroll never names).
    #[test]
    fn the_scroll_wheel_cycles_the_hotbar() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.inventory.items[1] = (62, 5);
        app.hotbar_slots = app.inventory.hotbar(); // the staging's own sync
        // the DOWN notch: the folded -2.0 emits the step, the selection
        // moves toward the higher numbers
        app.on_hotbar_scroll(0.0, -1.0);
        assert_eq!(app.selected_hotbar, 1, "the down-scroll's next slot");
        assert_eq!(app.inventory.items[1], (62, 5), "the store untouched");
        // the UP notch: back
        app.on_hotbar_scroll(0.0, 1.0);
        assert_eq!(app.selected_hotbar, 0, "the up-scroll's previous slot");
        // the WRAP: 0 scrolls up to 8 (the floor-mod's own answer)
        app.on_hotbar_scroll(0.0, 1.0);
        assert_eq!(app.selected_hotbar, 8, "the wrap the floor-mod keeps");
        // and down wraps back to 0
        app.on_hotbar_scroll(0.0, -1.0);
        assert_eq!(app.selected_hotbar, 0, "the wrap speaks both ways");
        // the held model follows the selection (the item lands in the
        // crosshair's hand)
        app.on_hotbar_scroll(0.0, -1.0); // -> 1
        assert_eq!(app.held_id(), Some(62), "the held model follows");
        // the HUD hotbar row untouched by the selection (the counts are
        // the store's projection, the selection the HUD's own box)
        assert_eq!(app.hotbar_slots[1], (62, 5), "the row whole");
        assert!(app.drops.is_empty(), "no drop the scroll never names");
        assert!(!app.swing.swinging, "no swing the scroll never names");
    }

    /// THE DOMOBSPAWNING RULE'S OWN FREEZE LAW (the RULE_DOMOBSPAWNING
    /// row the natural cycle's own gate reads): the false the whole
    /// beat's attempts the walk never earns — the day's creature arm
    /// the gate refuses (the armed walk the day-beat test speaks),
    /// the world's own ticks the frame keeps.
    #[test]
    fn the_do_mob_spawning_rule_freezes_the_spawn_beat() {
        let mut app = App::new(9);
        app.enter_world();
        // THE FLAT GRASS PLATFORM (the day beat's own voxels the test
        // sets — the walk's every spot the platform keeps)
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        {
            let w = app.offline_world.as_mut().unwrap();
            for x in 0..size {
                for z in 0..size {
                    for y in 0..8 {
                        w.set(x, y, z, crate::world::STONE);
                    }
                    w.set(x, 8, z, crate::world::GRASS);
                    for y in 9..size {
                        w.set(x, y, z, crate::world::AIR);
                    }
                }
            }
        }
        app.body = Some(crate::body::PlayerBody::new(40.0, 9.0, 40.0, 0.0, 0.0));
        // THE DISARM (the rule the set walk flips — the chat literal's
        // own arm): the natural attempts the beat never earns
        app.chat_open = true;
        app.chat_input = "/gamerule doMobSpawning false".into();
        app.on_chat_submit();
        assert!(!app.do_mob_spawning, "the set the rule disarms");
        // THE DAY BAND (the gate's own word — the bright tick the beat's
        // creature arm keeps)
        let day_tick = (0..24_000i64)
            .find(|t| crate::mob::is_bright_outside(*t))
            .expect("a day band the sky keeps");
        app.clock_total = day_tick as u64 + app.frame;
        // THE BEAT (the spawn_clock the 40th update the % SPAWN_CYCLE
        // arm — the gate the attempts refuse, the ring the randoms
        // never spends)
        for _ in 0..400 {
            app.update();
        }
        assert!(
            app.mobs.is_empty(),
            "the disarmed beat the spawns refuses"
        );
        // THE WORLD'S OWN BEAT (the frame the rule never gates — the
        // doDaylightCycle law's own shape the spawn clock shares)
        assert_eq!(app.frame, 400, "the world's own ticks the rule keeps");
    }

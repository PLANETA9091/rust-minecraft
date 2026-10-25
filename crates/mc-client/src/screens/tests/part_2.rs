use super::*;

/// THE REMOVAL TAIL THE REPLY VOICE THE DARKNESS WEARS (javap the
/// 26.2 jar SculkShriekerBlockEntity.preRemoveSideEffects pc 0-41 +
/// tryRespond pc 0-42 + SpawnUtil.trySpawnMob + Warden
/// applyDarknessAround — the dig-removed SHRIEKING cell the walk
/// answers): the all-air world the can-summon lvl-4 cell the FAILED
/// summon wears (the 40 draws the 20 attempts spend, the reply
/// voice the listening_angry quartet speaks, the THREE offsets the
/// voice's own draws bank, the darkness record the both-arms tail
/// keeps), the stone floor the SUCCESSFUL summon wears (the
/// feet-center the wardens list banks, NO reply voice the success
/// never plays, the TWO draws the first attempt keeps), the gates
/// the silent removals speak (the never-shrieked window, the
/// zero-level cell, the PEACEFUL acmp), the verdict's own folds the
/// records answer (the EXACT-40 distance the strict refusal keeps,
/// the creative mode the survival gate refuses).
#[test]
fn the_removal_tail_the_reply_voice_the_darkness_wears() {
    // === WALK A: the FAILED summon — the all-air world the reply
    // voice + the darkness record keep (the 40 + 3 draws the walk
    // spends — the mirror the stream position pins)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 35.0, 0.0, 0.0, 0.0));
    app.place_sculk_shrieker(2, 35, 0, true);
    let f = app.frame;
    app.sculk_shrikers[0].free_at = f + 90;
    app.sculk_shrikers[0].warning_level = 4;
    let probe = app.level_ambient_rng.clone();
    app.remove_sculk_shrieker(2, 35, 0);
    assert!(
        app.sculk_shrikers.is_empty(),
        "the dug cell the vec removal retires"
    );
    assert!(
        app.wardens.is_empty(),
        "the failed summon the warden list keeps empty"
    );
    assert!(
        !app.game_events
            .iter()
            .any(|l| l.starts_with("warden_summon @")),
        "the failed summon the summon record never banks"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.warden.listening_angry ")),
        "the lvl-4 reply the listening_angry voice speaks"
    );
    assert_eq!(
        app.sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.warden."))
            .count(),
        1,
        "the reply the ONLY warden voice the tail plays"
    );
    assert_eq!(
        app.game_events
            .iter()
            .find(|l| l.starts_with("warden_darkness @"))
            .map(String::as_str),
        Some(format!("warden_darkness @{f} x2.500000 y35.500000 z0.500000 t260").as_str()),
        "the darkness record the both-arms tail banks — the +0.5 center the atCenterOf fold wears"
    );
    // THE DRAW LAW: the 40 summon draws + the THREE reply offsets —
    // the mirror the stream position pins
    let mut mirror = probe.clone();
    for _ in 0..(2 * crate::sculk::WARDEN_SUMMON_ATTEMPTS as usize) {
        mirror.next_int_inclusive(
            -crate::sculk::WARDEN_SUMMON_RANGE,
            crate::sculk::WARDEN_SUMMON_RANGE,
        );
    }
    for _ in 0..3 {
        mirror.next_int_inclusive(-10, 10);
    }
    assert_eq!(
        app.level_ambient_rng.next_f32(),
        mirror.next_f32(),
        "the failed walk the 43 draws spend — the bytecode's own order pins"
    );

    // === WALK B: the SUCCESSFUL summon — the stone floor the
    // warden lands (the feet-center the list banks, NO reply the
    // success never plays, the TWO draws the first attempt keeps)
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(14.0, 35.0, 14.0, 0.0, 0.0));
    app.place_sculk_shrieker(16, 35, 0, true);
    let f = app.frame;
    app.sculk_shrikers[0].free_at = f + 90;
    app.sculk_shrikers[0].warning_level = 4;
    let mut probe = app.level_ambient_rng.clone();
    let expect_dx = probe.next_int_inclusive(
        -crate::sculk::WARDEN_SUMMON_RANGE,
        crate::sculk::WARDEN_SUMMON_RANGE,
    );
    let expect_dz = probe.next_int_inclusive(
        -crate::sculk::WARDEN_SUMMON_RANGE,
        crate::sculk::WARDEN_SUMMON_RANGE,
    );
    app.remove_sculk_shrieker(16, 35, 0);
    assert_eq!(
        app.wardens.len(),
        1,
        "the spawn the ONE warden the list banks"
    );
    assert_eq!(
        (app.wardens[0].x, app.wardens[0].y, app.wardens[0].z),
        (16.0 + expect_dx as f64 + 0.5, 35.0, expect_dz as f64 + 0.5),
        "the spawn cell the feet-center wears — the first attempt's own draws name it"
    );
    assert_eq!(
        app.wardens[0].health,
        crate::sculk::WARDEN_MOB_MAX_HEALTH,
        "the fresh entity the MAX_HEALTH 500.0 keeps"
    );
    assert_eq!(
        app.wardens[0].active_anger(),
        0,
        "the fresh entity the zero anger the Calm level wears"
    );
    assert!(
        app.wardens[0].ambient_time < 0,
        "the fresh clock the -80 floor the house's ambient lane wears"
    );
    assert!(
        app.game_events.iter().any(|l| l.starts_with(&format!(
            "warden_summon @{f} x{:.6} y35.000000 z",
            16.0 + expect_dx as f64 + 0.5
        ))),
        "the summon record the spawn's own cell banks"
    );
    // THE SPAWN'S OWN AMBIENT (javap `SpawnUtil.trySpawnMob`
    // pc 172-184 — the addFreshEntity tail the playAmbientSound
    // call serves): the success the ONE warden voice speaks — the
    // entity's own ambient the Calm level answers, the reply
    // quartet the failed summon alone plays
    assert_eq!(
            app.sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.warden."))
                .count(),
            1,
            "the success the finalize's AGITATED voice ALONE speaks — the reply voice the failure keeps"
        );
    assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("entity.warden.agitated ")),
            "the finalizeSpawn's own arm the WARDEN_AGITATED voice serves — the EMERGING pose the spawn ambient the gate REFUSES"
        );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("warden_darkness @")),
        "the darkness the both-arms tail wears — the summon verdict the tail never skips"
    );
    assert_eq!(
        app.level_ambient_rng.next_f32(),
        probe.next_f32(),
        "the success the TWO draws spend"
    );

    // === WALK C: THE GATES THE SILENT REMOVALS SPEAK — the
    // never-shrieked window the cell alone retires, the zero-level
    // cell the tryRespond head refuses, the PEACEFUL world the
    // canRespond acmp refuses
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 35.0, 0.0, 0.0, 0.0));
    app.place_sculk_shrieker(2, 35, 0, true);
    app.sculk_shrikers[0].warning_level = 4;
    let mut probe = app.level_ambient_rng.clone();
    app.remove_sculk_shrieker(2, 35, 0);
    assert!(
        app.sculk_shrikers.is_empty(),
        "the never-shrieked window the cell alone retires"
    );
    assert!(
        !app.game_events
            .iter()
            .any(|l| l.starts_with("warden_darkness @") || l.starts_with("warden_summon @")),
        "the never-shrieked window the silent removal keeps"
    );
    assert_eq!(
        app.level_ambient_rng.next_f32(),
        probe.next_f32(),
        "the SHRIEKING gate the walk never reaches — no draw the removal spends"
    );
    // THE ZERO-LEVEL CELL: the shrieking window the warnLevel > 0
    // pair the head refuses
    app.place_sculk_shrieker(2, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    let mut probe = app.level_ambient_rng.clone();
    app.remove_sculk_shrieker(2, 35, 0);
    assert!(
        !app.game_events
            .iter()
            .any(|l| l.starts_with("warden_darkness @") || l.starts_with("warden_summon @")),
        "the zero level the tryRespond head refuses"
    );
    assert_eq!(
        app.level_ambient_rng.next_f32(),
        probe.next_f32(),
        "the head the draws never reaches"
    );
    // THE PEACEFUL ACMP: the can-summon lvl-4 cell the gate refuses
    let mut peaceful = air_net_world(2, 0);
    peaceful.difficulty = crate::difficulty::Difficulty::Peaceful;
    app.net_world = Some(peaceful);
    app.place_sculk_shrieker(2, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 4;
    let mut probe = app.level_ambient_rng.clone();
    app.remove_sculk_shrieker(2, 35, 0);
    assert!(
        !app.game_events
            .iter()
            .any(|l| l.starts_with("warden_darkness @") || l.starts_with("warden_summon @")),
        "the PEACEFUL acmp the canRespond refusal wears"
    );
    assert_eq!(
        app.level_ambient_rng.next_f32(),
        probe.next_f32(),
        "the gate the summon walk never arms"
    );
    // THE EXACT-40 DISTANCE: the strict closerThan the refusal
    // keeps (the d2 >= 1600 the FALSE wears) — the reply voice the
    // failed summon plays, the darkness record the verdict refuses
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(2.5, 75.5, 0.5, 0.0, 0.0));
    app.place_sculk_shrieker(2, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 1;
    app.remove_sculk_shrieker(2, 35, 0);
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.warden.nearby_close ")),
        "the lvl-1 reply the nearby_close voice speaks"
    );
    assert!(
        !app.game_events
            .iter()
            .any(|l| l.starts_with("warden_darkness @")),
        "the exact-40 the strict fold refuses — d2 >= 1600 the FALSE wears"
    );
    // THE CREATIVE GATE: the survival head the verdict refuses
    app.body = Some(crate::body::PlayerBody::new(0.0, 35.0, 0.0, 0.0, 0.0));
    app.game_mode = GameMode::Creative;
    app.place_sculk_shrieker(2, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 2;
    app.remove_sculk_shrieker(2, 35, 0);
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.warden.nearby_closer ")),
        "the lvl-2 reply the nearby_closer voice speaks"
    );
    assert!(
        !app.game_events
            .iter()
            .any(|l| l.starts_with("warden_darkness @")),
        "the creative mode the survival gate refuses"
    );
}

/// THE WARDEN'S OWN TICK THE FRAME WALK THE AMBIENT SPEAKS (javap
/// the 26.2 jar `SpawnUtil.trySpawnMob` pc 172-184 + `Mob.baseTick`
/// pc 17-55 + `LivingEntity.makeSound` — the summon the entity
/// banks, the spawn ambient the tail plays, the update() walk the
/// baseTick law ticks, the lockstep mirror the same fire tick
/// answers, the v4.0 the Warden.getSoundVolume keeps, the two-draw
/// pitch law the entity's own stream speaks).
#[test]
fn the_warden_tick_the_frame_walk_the_ambient_speaks() {
    // === THE SUMMON (the walk-B's own setup — the stone floor the
    // warden lands) ===
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(14.0, 35.0, 14.0, 0.0, 0.0));
    app.place_sculk_shrieker(16, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 4;
    app.remove_sculk_shrieker(16, 35, 0);
    assert_eq!(app.wardens.len(), 1, "the spawn the ONE warden banks");
    let warden_id = app.wardens[0].id;

    // THE SPAWN VOICE (javap `Warden.finalizeSpawn` pc 47-54): the
    // ONE warden event the sounds lane banks — the AGITATED word
    // the TRIGGERED arm speaks (the EMERGING pose the spawn
    // ambient the gate REFUSES — the getAmbientSound the pose
    // null keeps), the v5.0 the FIXED pair keeps. THE VOICE
    // STREAM THE DRAW NEVER SPENDS — the FIXED pitch the
    // explicit-arg walk carries, the two-draw law the ambient
    // voices spend the finalize NEVER rides.
    let agitated_count = |app: &App| {
        app.sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.warden.agitated "))
            .count()
    };
    assert_eq!(
        agitated_count(&app),
        1,
        "the finalize's AGITATED voice the summon speaks"
    );
    assert_eq!(
        app.sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.warden.ambient "))
            .count(),
        0,
        "the EMERGING pose the spawn ambient the gate refuses"
    );
    // the FIXED pitch the line carries — the fconst_1 the
    // playSound's own pitch arg wears
    let agitated_line = app
        .sounds
        .events
        .iter()
        .find(|e| e.starts_with("entity.warden.agitated "))
        .expect("the agitated voice rides");
    assert!(
        agitated_line.contains("p1.000000"),
        "the FIXED pitch the explicit-arg walk speaks: {agitated_line}"
    );
    // the volume the attenuate fold carries — the 5.0f the ldc
    // wears, the gain the distance law folds (the AL_MAX_GAIN
    // ceiling the close lane clamps)
    let ear = app.body.as_ref().map(|b| (b.x, b.y + b.eye_height(), b.z));
    let (wx, wy, wz) = (app.wardens[0].x, app.wardens[0].y, app.wardens[0].z);
    let agitated_dist = ear_distance(ear, wx, wy, wz);
    let agitated_gain = crate::sound::attenuate(
        crate::sculk::WARDEN_EMERGE_VOLUME,
        agitated_dist,
        crate::sound::voice_max_distance(crate::sculk::WARDEN_EMERGE_VOLUME),
    );
    assert!(
        agitated_line.contains(&format!("v{agitated_gain:.6}")),
        "the v5.0 the distance law folds — the line the gain carries: {agitated_line}"
    );
    assert_eq!(
        app.wardens[0].ambient_time,
        -crate::mob::AMBIENT_INTERVAL_TICKS,
        "the fresh clock the summon voice never touches"
    );
    // THE EMERGE WINDOW THE TEST SKIPS (the finalizeSpawn's own arm
    // the summoned body wears — the isDiggingOrEmerging() gate the
    // ambient the window REFUSES; the steady-state body the
    // schedule law consults)
    app.wardens[0].emerging = false;
    app.wardens[0].emerge_ticks = 0;

    // THE SCHEDULE (the twin's own walk pins — the fire tick the
    // same seed answers, the law's own probability the drive
    // resolves deterministically; the fresh stream the summon's
    // FIXED voice NEVER spends — the pre-draws the old ambient
    // premise wore are gone)
    let mut twin = crate::sculk::WardenMob::new(warden_id, wx, wy, wz);
    let schedule = {
        let mut probe = crate::sculk::WardenMob::new(warden_id, wx, wy, wz);
        let mut t = 0usize;
        for tick in 1..=1_000_000usize {
            if probe.tick() {
                t = tick;
                break;
            }
            // the client branch the frame walk serves — the
            // heartbeat draws the entity's own stream spends, the
            // mirror the same walk keeps (javap `Warden.tick` the
            // server walk the client branch the SAME tick serves)
            probe.tick_client();
        }
        assert!(t > 0, "the climb fires — the mirror the law keeps");
        t
    };
    assert!(
        schedule > crate::mob::AMBIENT_INTERVAL_TICKS as usize,
        "the fresh clock the 80-tick hold the schedule keeps"
    );

    // THE LOCKSTEP WALK: the update() the frame walk serves — the
    // warden the baseTick law ticks, the twin the same walk keeps
    // (the one-draw gate the same ticks spend), the fire the
    // schedule's OWN update speaks (the earlier updates the silence
    // keeps), the fired voice's pitch the post-fire pair the twin
    // draws first
    let base = {
        // the ambient count the lockstep base keeps
        app.sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.warden.ambient "))
            .count()
    };
    for i in 1..=schedule {
        let twin_fired = twin.tick();
        // the client branch the app's own walk serves — the
        // heartbeat draws the twin's stream spends the same ticks
        // the lockstep keeps (the due fold the same anger answers)
        twin.tick_client();
        app.update();
        let now = {
            app.sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.warden.ambient "))
                .count()
        };
        let expect = if i == schedule { base + 1 } else { base };
        assert_eq!(
            now, expect,
            "the schedule the lockstep keeps — the fire the same tick the twin wears"
        );
        assert_eq!(
            twin_fired,
            i == schedule,
            "the twin the same fire tick answers — the same seed the same walk wears"
        );
    }
    let qa = twin.rng.next_f32();
    let qb = twin.rng.next_f32();
    let fired_pitch = (qa - qb) * 0.2 + 1.0;
    let fired_line = app
        .sounds
        .events
        .iter()
        .rev()
        .find(|e| e.starts_with("entity.warden.ambient "))
        .expect("the fired voice rides");
    assert!(
        fired_line.contains(&format!("p{fired_pitch:.6}")),
        "the fired voice the two-draw law speaks — the twin the same pair wears: {fired_line}"
    );
    // the clock the -80 reset wears — the next chance the interval
    // holds again
    assert!(
        app.wardens[0].ambient_time < 0,
        "the fire the resetAmbientSoundTime floor wears"
    );
}

/// THE PULSE THE CADENCE THE DARKNESS WEARS (javap
/// `Warden.customServerAiStep` pc 33-56 — the
/// `(tickCount + getId()) % 120 == 0` cadence the walk serves, the
/// `applyDarknessAround(level, position(), this, 20)` call the
/// ENTITY's own feet position wears, the DARKNESS 260 the effects
/// store banks, the STRICT closerThan the exact-touch refuses).
#[test]
fn the_pulse_the_cadence_the_darkness_wears() {
    let _render_lane = render_lane();
    // === THE SUMMON (the walk-B's own setup — the stone floor the
    // warden lands) ===
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(14.0, 35.0, 14.0, 0.0, 0.0));
    app.place_sculk_shrieker(16, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 4;
    app.remove_sculk_shrieker(16, 35, 0);
    assert_eq!(app.wardens.len(), 1, "the spawn the ONE warden banks");
    // the warden the KNOWN feet moves — the entity's own position()
    // the center wears (the exact Vec3 the pulse call keeps)
    app.wardens[0].x = 10.5;
    app.wardens[0].y = 35.0;
    app.wardens[0].z = 10.5;
    // THE SNIFF LANE THE TEST ISOLATES (the standing SNIFF_COOLDOWN
    // memory the trigger's absent gate refuses — the value the
    // vanilla 100..200 range the isolation stretches, the law the
    // number never serves; the drain the 1/tick walk the test's
    // window never exhausts)
    app.wardens[0].sniff_cooldown = i32::MAX;
    // the SHRIEK TAIL's own application the summon banks (the
    // tryRespond walk the radius-40 verdict wears) — the fresh store
    // the pulse legs the summon's standing never taints
    app.effects = crate::effects::EffectsStore::new();
    let warden_id = app.wardens[0].id;
    // THE CADENCE (javap pc 33-45): the first tick_count N >= 1 the
    // (N + id) % 120 == 0 answers — the POST-increment count the
    // tick()'s own head banks
    let interval = crate::sculk::WARDEN_PULSE_DARKNESS_INTERVAL;
    let pulse_tick = interval - warden_id % interval;
    assert!(crate::sculk::warden_pulse_due(pulse_tick, warden_id));
    // the pre-pulse ticks the silence keeps
    let before = app.game_events.len();
    for _ in 0..pulse_tick {
        app.update();
    }
    // the verdict: the body (14, 35, 14) vs the center
    // (10.5, 35, 10.5) — d2 = 24.5 < 400.0 the pass wears
    let pulse_line = app.game_events[before..]
        .iter()
        .find(|l| l.starts_with("warden_darkness_pulse @"))
        .expect("the pulse record banks — the cadence the walk serves");
    assert!(
            pulse_line.contains("x10.500000 y35.000000 z10.500000"),
            "the entity's own feet position the center wears — the +0.5 the atCenterOf fold the shriek's BLOCK walk speaks, the pulse never: {pulse_line}"
        );
    assert!(
        pulse_line.contains(&format!("t{}", crate::sculk::WARDEN_DARKNESS_TICKS)),
        "the 260 the instance wears: {pulse_line}"
    );
    // the effect the store banks — the drain the post-pulse tick
    // wears (the tick_client the -1 the same update serves)
    let inst = app
        .effects
        .get(crate::effects::MOB_EFFECT_DARKNESS)
        .expect("the darkness rides");
    assert_eq!(
        inst.duration,
        crate::sculk::WARDEN_DARKNESS_TICKS - 1,
        "the drain the post-pulse tick wears"
    );
    assert_eq!(inst.amplifier, 0, "the zero amplifier the instance keeps");

    // THE STRICT TOUCH the distance fold refuses: the player the
    // exact +20.0 x moves, the NEXT pulse the silence keeps — the
    // standing the refresh never lifts
    app.body = Some(crate::body::PlayerBody::new(30.5, 35.0, 10.5, 0.0, 0.0));
    let before2 = app.game_events.len();
    for _ in 0..interval {
        app.update();
    }
    assert!(
        !app.game_events[before2..]
            .iter()
            .any(|l| l.starts_with("warden_darkness_pulse @")),
        "the exact-touch the STRICT fold refuses — the dcmpg the equality wears"
    );
    let inst2 = app
        .effects
        .get(crate::effects::MOB_EFFECT_DARKNESS)
        .expect("the standing keeps");
    assert_eq!(
        inst2.duration,
        crate::sculk::WARDEN_DARKNESS_TICKS - 1 - interval,
        "the standing the plain drain wears — the refresh the distance refusal never lifts"
    );
}

/// THE PULSE THE REFRESH THE WINDOW KEEPS (javap MobEffectUtil
/// lambda$addEffectToPlayersAround$0 pc 37-82 — the standing
/// instance the `endsWithin(200 - 1)` pair consults: the LONGER
/// walk the skip wears, the SHORT the refresh the 260 banks).
#[test]
fn the_pulse_the_refresh_the_window_keeps() {
    // === THE SUMMON (the walk-B's own setup) ===
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(10.5, 35.0, 10.5, 0.0, 0.0));
    app.place_sculk_shrieker(16, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 4;
    app.remove_sculk_shrieker(16, 35, 0);
    assert_eq!(app.wardens.len(), 1, "the spawn the ONE warden banks");
    app.wardens[0].x = 10.5;
    app.wardens[0].y = 35.0;
    app.wardens[0].z = 10.5;
    // the SHRIEK TAIL's own application the summon banks — the fresh
    // store the planted D the standing's own 260 never taints
    app.effects = crate::effects::EffectsStore::new();
    let warden_id = app.wardens[0].id;
    let interval = crate::sculk::WARDEN_PULSE_DARKNESS_INTERVAL;
    let pulse_tick = interval - warden_id % interval;

    // CASE A — the standing LONGER the window keeps: the planted D
    // the pulse reads 210 (> 199) wears — the refresh the verdict
    // refuses, the plain drain the standing keeps
    let d_long = 210 + pulse_tick - 1;
    app.effects.add_effect(
        crate::effects::MOB_EFFECT_DARKNESS,
        0,
        d_long,
        crate::effects::DEFAULT_EFFECT_FLAGS,
    );
    let before = app.game_events.len();
    for _ in 0..pulse_tick {
        // THE FIGHT LANE STILLED (the same-cell warden the summon's
        // own roar the swing arms — the 18l window the pin keeps:
        // the pulse law's own scene the combat-free body wears)
        app.wardens[0].attack_cooldown = crate::sculk::WARDEN_MELEE_COOLDOWN_TICKS;
        app.update();
    }
    assert!(
        !app.game_events[before..]
            .iter()
            .any(|l| l.starts_with("warden_darkness_pulse @")),
        "the longer standing the refresh refuses — the record the verdict never banks"
    );
    let inst = app
        .effects
        .get(crate::effects::MOB_EFFECT_DARKNESS)
        .expect("the standing keeps");
    assert_eq!(
        inst.duration, 209,
        "the 210 the pulse reads, the drain the window keeps — the 260 the refresh never banks"
    );

    // CASE B — the standing SHORT the window the refresh wears: the
    // fresh store the 150 the NEXT pulse reads plants, the refresh
    // the 260 the store banks
    app.effects = crate::effects::EffectsStore::new();
    // the next pulse the 120-tick gap walks — the planted D the
    // 119 drains the pulse read wears (the drain the pulse's own
    // update serves AFTER the verdict)
    let d_short = 150 + interval - 1;
    app.effects.add_effect(
        crate::effects::MOB_EFFECT_DARKNESS,
        0,
        d_short,
        crate::effects::DEFAULT_EFFECT_FLAGS,
    );
    let before2 = app.game_events.len();
    for _ in 0..interval {
        // THE FIGHT LANE STILLED (the same pin the case wears)
        app.wardens[0].attack_cooldown = crate::sculk::WARDEN_MELEE_COOLDOWN_TICKS;
        app.update();
    }
    let pulse2 = app.game_events[before2..]
        .iter()
        .find(|l| l.starts_with("warden_darkness_pulse @"))
        .expect("the refresh the record banks — the window the verdict opens");
    assert!(
        pulse2.contains(&format!("t{}", crate::sculk::WARDEN_DARKNESS_TICKS)),
        "the 260 the refresh wears: {pulse2}"
    );
    let inst2 = app
        .effects
        .get(crate::effects::MOB_EFFECT_DARKNESS)
        .expect("the refreshed rides");
    assert_eq!(
        inst2.duration,
        crate::sculk::WARDEN_DARKNESS_TICKS - 1,
        "the 260 the refresh banks, the drain the post-pulse tick wears"
    );
}

/// THE PULSE THE GATES THE REFUSAL KEEPS (javap pc 0-10 the
/// isSurvival head + the ALIVE body the customServerAiStep lane
/// keeps — the creative the head refuses, the dead body the lane
/// never serves, the effect the store never banks).
#[test]
fn the_pulse_the_gates_the_refusal_keeps() {
    // === THE SUMMON (the walk-B's own setup) ===
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(10.5, 35.0, 10.5, 0.0, 0.0));
    app.place_sculk_shrieker(16, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 4;
    app.remove_sculk_shrieker(16, 35, 0);
    assert_eq!(app.wardens.len(), 1, "the spawn the ONE warden banks");
    app.wardens[0].x = 10.5;
    app.wardens[0].y = 35.0;
    app.wardens[0].z = 10.5;
    // the SHRIEK TAIL's own application the summon banks — the fresh
    // store the gates' legs the summon's standing never taints
    app.effects = crate::effects::EffectsStore::new();
    let warden_id = app.wardens[0].id;
    let interval = crate::sculk::WARDEN_PULSE_DARKNESS_INTERVAL;
    let pulse_tick = interval - warden_id % interval;

    // THE CREATIVE the survival head refuses (pc 0-10 — the
    // gameMode's own isSurvival verdict the FALSE wears)
    app.game_mode = GameMode::Creative;
    for _ in 0..pulse_tick {
        app.update();
    }
    assert!(
        !app.game_events
            .iter()
            .any(|l| l.starts_with("warden_darkness_pulse @")),
        "the creative the head refuses — the record the verdict never banks"
    );
    assert!(
        app.effects
            .get(crate::effects::MOB_EFFECT_DARKNESS)
            .is_none(),
        "the effect the creative never banks"
    );

    // THE DEAD BODY the lane never serves: the survival the re-arm
    // wears, the health 0 the tick() the early-false keeps — the
    // pulse the alive gate refuses
    app.game_mode = GameMode::Survival;
    app.wardens[0].health = 0.0;
    let before = app.game_events.len();
    for _ in 0..interval {
        app.update();
    }
    assert!(
        !app.game_events[before..]
            .iter()
            .any(|l| l.starts_with("warden_darkness_pulse @")),
        "the dead body the pulse never serves"
    );
    assert!(
        app.effects
            .get(crate::effects::MOB_EFFECT_DARKNESS)
            .is_none(),
        "the effect the dead lane never banks"
    );
}

/// THE HEARTBEAT THE CADENCE THE RECORD BANKS (javap `Warden.tick`
/// pc 51-116 — the client branch the alive body serves, the
/// `tickCount % getHeartBeatDelay() == 0` fold the due tick keeps,
/// the `playLocalSound(x, y, z, WARDEN_HEARTBEAT, source, 5.0f,
/// pitch, false)` the voice the literal volume wears, the heart
/// clock the bipush-10 fills).
#[test]
fn the_heartbeat_the_cadence_the_record_banks() {
    // === THE SUMMON (the walk-B's own setup — the stone floor the
    // warden lands) ===
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(14.0, 35.0, 14.0, 0.0, 0.0));
    app.place_sculk_shrieker(16, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 4;
    app.remove_sculk_shrieker(16, 35, 0);
    assert_eq!(app.wardens.len(), 1, "the spawn the ONE warden banks");
    app.wardens[0].x = 10.5;
    app.wardens[0].y = 35.0;
    app.wardens[0].z = 10.5;
    // the SHRIEK TAIL's own application the summon banks — the fresh
    // store the beat's legs the summon's standing never taints
    app.effects = crate::effects::EffectsStore::new();

    // THE CALM WINDOW the delay fold answers (javap getHeartBeatDelay
    // pc 14-29 — the zero anger the clamp floor keeps, `40 - 0` the
    // 40-tick window wears)
    assert_eq!(
        crate::sculk::warden_heartbeat_delay(0),
        40,
        "the fresh Calm the 40-tick window wears"
    );

    // THE PRE-DUE TICKS the silence keeps (ticks 1..39 — the fold
    // the remainder refuses)
    let before = app.game_events.len();
    for _ in 0..39 {
        app.update();
    }
    assert!(
        !app.game_events[before..]
            .iter()
            .any(|l| l.starts_with("warden_heartbeat @")),
        "the pre-due ticks the silence keeps — the fold the remainder refuses"
    );

    // THE 40TH TICK the beat banks — the record the entity's own
    // feet the position wears, the literal volume the v5.0 tail
    // keeps, the pitch the two-draw the 1.0-neighborhood answers
    app.update();
    let beat_line = app.game_events[before..]
        .iter()
        .find(|l| l.starts_with("warden_heartbeat @"))
        .expect("the beat record banks — the due tick the cadence serves");
    assert!(
            beat_line.contains("x10.500000 y35.000000 z10.500000"),
            "the entity's own feet the record wears — the getX/getY/getZ folds the call keeps: {beat_line}"
        );
    assert!(
            beat_line.contains("v5.0 "),
            "the LITERAL volume the record wears — the ldc 5.0f the getSoundVolume 4.0 never serves: {beat_line}"
        );
    let pitch: f32 = beat_line
        .split_whitespace()
        .last()
        .and_then(|t| t.strip_prefix('p'))
        .and_then(|v| v.parse().ok())
        .expect("the p-tail the two-draw pitch wears");
    assert!(
        (0.79..=1.21).contains(&pitch),
        "the two-draw adult law the 1.0-neighborhood wears: {pitch}"
    );

    // THE HEART CLOCK the beat fills — the O snapshot the fire tick
    // banks, the drain the same walk serves (javap pc 73-76 the
    // fill, pc 144-166 the snapshot+drain pair)
    assert_eq!(
        app.wardens[0].heart_animation_o,
        crate::sculk::WARDEN_HEARTBEAT_ANIMATION,
        "the O snapshot the fire tick banks"
    );
    assert_eq!(
        app.wardens[0].heart_animation,
        crate::sculk::WARDEN_HEARTBEAT_ANIMATION - 1,
        "the drain the same walk serves"
    );

    // THE QUIET WINDOW the next 39 ticks keep — the fold the
    // remainder refuses until the 80th
    let before2 = app.game_events.len();
    for _ in 0..39 {
        app.update();
    }
    assert!(
        !app.game_events[before2..]
            .iter()
            .any(|l| l.starts_with("warden_heartbeat @")),
        "the quiet window the fold keeps"
    );
    app.update();
    assert!(
        app.game_events[before2..]
            .iter()
            .any(|l| l.starts_with("warden_heartbeat @")),
        "the 80th tick the next beat banks — the cadence the window repeats"
    );
}

/// THE SHIVER THE HEARING BANKS THE DECAY THE CLOCKS DRAIN (javap
/// `Warden$VibrationUser.onReceiveVibration` pc 30-37 — the
/// broadcastEntityEvent(this, 61) the walk speaks, the
/// handleEntityEvent pc 26-35 the tendril clock fills; javap
/// `Warden.tick` pc 119-166 — the O-snapshot + drain pair both
/// clocks the client branch serves, the >0 gate the zero keeps).
#[test]
fn the_shiver_the_hearing_banks_the_decay_drains() {
    let mut w = crate::sculk::WardenMob::new(11, 5.5, 40.0, 5.5);

    // THE FRESH CLOCKS the JVM zero-init keeps
    assert_eq!(w.tendril_animation, 0, "the fresh tendril the zero keeps");
    assert_eq!(w.heart_animation, 0, "the fresh heart the zero keeps");

    // THE HEARING the shiver banks — the byte-61 event the
    // vibration receive speaks (the pitch the two-draw the click
    // voice spends, the 1.0-neighborhood the adult law wears)
    let hear = w
        .hear_vibration(
            Some(crate::sculk::WARDEN_PLAYER_SUSPECT),
            None,
            None,
            true,
            true,
            true,
            (16, 34, 0),
        )
        .expect("the hearing answers");
    assert!(
        (0.79..=1.21).contains(&hear.click_pitch),
        "the click pitch the two-draw law wears: {}",
        hear.click_pitch
    );
    assert_eq!(
        w.tendril_animation,
        crate::sculk::WARDEN_TENDRILS_SHIVER_ANIMATION,
        "the shiver the byte-61 banks — the bipush-10 the clock fills"
    );

    // THE FIRST CLIENT TICK the shiver drains — the O snapshot 10
    // the lerp's own source banks, the cur the 9 answers (the
    // fresh anger the 40-window the first tick never beats — the
    // POST-increment count 1 the remainder keeps)
    w.tick();
    assert!(
        w.tick_client().is_none(),
        "the count 1 the 40-window refuses — the beat never banks early"
    );
    assert_eq!(
        w.tendril_animation_o,
        crate::sculk::WARDEN_TENDRILS_SHIVER_ANIMATION,
        "the O snapshot the first walk banks"
    );
    assert_eq!(
        w.tendril_animation,
        crate::sculk::WARDEN_TENDRILS_SHIVER_ANIMATION - 1,
        "the drain the first walk serves"
    );

    // THE NINE MORE WALKS the clock drains to the boundary — the
    // O-lag the pair wears (the 10th walk the O 1 the cur 0 keeps,
    // the 11th the both-zero the gate answers)
    for _ in 0..9 {
        w.tick();
        w.tick_client();
    }
    assert_eq!(w.tendril_animation_o, 1, "the O-lag the boundary wears");
    assert_eq!(w.tendril_animation, 0, "the drain the zero reaches");
    w.tick();
    w.tick_client();
    assert_eq!(w.tendril_animation_o, 0, "the O the zero follows");
    assert_eq!(w.tendril_animation, 0, "the >0 gate the drain stops");

    // THE HEART CLOCK the hearing's anger the window wears — the
    // +35 the hearing raised (the delay fold the 27 answers:
    // 35/80 * 30 = 13.125, the floor 13, `40 - 13` the window
    // wears), the count now 11 — the ANGER DECAY the 20th count
    // the window re-banks (35 -> 34, the floor 12, the 28 the
    // window wears), the NEXT MULTIPLE of 28 the beat answers
    assert_eq!(
        crate::sculk::warden_heartbeat_delay(w.active_anger()),
        27,
        "the hearing's +35 the 27-tick window wears"
    );
    for _ in 0..16 {
        w.tick();
        w.tick_client();
    }
    assert_eq!(
        w.tick_count, 27,
        "the 27th count the beat never banks — the decay the window re-banked"
    );
    // the count now 27 — the NEXT walk the count 28 banks the beat
    w.tick();
    let beat = w.tick_client().expect("the count 28 the beat banks");
    assert_eq!(
        w.tick_count, 28,
        "the multiple of 28 the beat answers — the decay the window re-banks"
    );
    assert!(
        (0.79..=1.21).contains(&beat.pitch),
        "the beat pitch the two-draw law wears: {}",
        beat.pitch
    );
    assert_eq!(
        w.heart_animation_o,
        crate::sculk::WARDEN_HEARTBEAT_ANIMATION,
        "the heart O the fire walk banks"
    );
    assert_eq!(
        w.heart_animation,
        crate::sculk::WARDEN_HEARTBEAT_ANIMATION - 1,
        "the heart drain the fire walk serves"
    );
}

/// THE ANGER THE WINDOW SHORTENS (javap getHeartBeatDelay pc 0-30 —
/// the delay the anger fold wears: the +100 hurt the ANGRY ceiling
/// the clamp keeps, the `40 - 30` the 10-tick floor answers, the
/// cadence the next multiple the shortened window wears).
#[test]
fn the_heartbeat_the_anger_the_window_shortens() {
    let mut w = crate::sculk::WardenMob::new(3, 5.5, 40.0, 5.5);

    // THE CALM WALK the first beat at 40 (the fresh anger the
    // 40-tick window wears, the 39 prior walks the silence keeps)
    let mut first_beat = 0;
    for _ in 0..40 {
        w.tick();
        if w.tick_client().is_some() {
            first_beat = w.tick_count;
        }
    }
    assert_eq!(first_beat, 40, "the calm window the first beat at 40");

    // THE HURT the +100 anger raises — the delay the ceiling fold
    // answers (100/80 the clamp 1.0 keeps, `40 - floor(30)` the
    // 10-tick floor wears; the anger the decay the -1-per-20 the
    // window never crosses the 80 floor within the test walk)
    w.hurt_from_player(7.0, true);
    assert_eq!(
        crate::sculk::warden_heartbeat_delay(w.active_anger()),
        10,
        "the ANGRY anger the 10-tick floor wears"
    );

    // THE SHORTENED WINDOW the next beat 10 ticks later — the fold
    // the multiple 50 answers (NOT the calm 80)
    let mut second_beat = 0;
    for _ in 0..20 {
        w.tick();
        if w.tick_client().is_some() {
            second_beat = w.tick_count;
            break;
        }
    }
    assert_eq!(
        second_beat, 50,
        "the shortened window the 50th count answers — the anger the heart quickens"
    );

    // THE DELAY FOLD'S OWN ROWS (javap pc 0-30 — the clamp the
    // rows keep: the zero the 40 wears, the 40 the half the 25
    // answers, the 80 the 10 wears, the 150 the ceiling the clamp
    // the 10 keeps — the anger the window never SLOWER wears)
    assert_eq!(crate::sculk::warden_heartbeat_delay(0), 40);
    assert_eq!(crate::sculk::warden_heartbeat_delay(40), 25);
    assert_eq!(crate::sculk::warden_heartbeat_delay(80), 10);
    assert_eq!(
        crate::sculk::warden_heartbeat_delay(150),
        10,
        "the max anger the clamp the floor keeps"
    );
}

/// THE DISTURBANCE THE CALM TAIL BANKS (javap
/// `WardenAi.setDisturbanceLocation` pc 0-106 + the
/// onReceiveVibration tail pc 172-229 — the containing fold the
/// pos wears, the 100l expiry the memory keeps, the Calm arm the
/// ANGRY level refuses, the drain the tick walks serve).
#[test]
fn the_disturbance_the_calm_tail_banks() {
    // === THE SUMMON (the glide-hears test's own setup — the stone
    // floor the warden lands) ===
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(14.0, 35.0, 14.0, 0.0, 0.0));
    app.place_sculk_shrieker(16, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 4;
    app.remove_sculk_shrieker(16, 35, 0);
    assert_eq!(app.wardens.len(), 1, "the spawn the ONE warden banks");
    app.wardens[0].x = 14.5;
    app.wardens[0].y = 35.0;
    app.wardens[0].z = 12.5;
    // THE EMERGE WINDOW THE TEST SKIPS (the finalizeSpawn's own
    // arm the summoned body wears — the isDiggingOrEmerging() gate
    // the hearing the window REFUSES; the steady-state body the
    // test's own lanes consult)
    app.wardens[0].emerging = false;
    app.wardens[0].emerge_ticks = 0;
    // the fresh store the empty-active pass keeps — the eyes the
    // first vibration opens
    assert!(
        app.wardens[0].disturbance.is_none(),
        "the fresh memory the None keeps"
    );

    // === THE GLIDE EVENT (the same lane the glide-hears test
    // drives — the pos the PRE-travel field wears) ===
    {
        let b = app.body.as_mut().expect("the body rides");
        b.elytra_glide_event_due = true;
        b.elytra_glide_event_pos = (14.0, 35.0, 12.0);
    }
    app.update();
    assert_eq!(app.warden_hears.len(), 1, "the pending the dispatch banks");
    let at = app.warden_hears[0].at_frame;
    // THE ARRIVAL the tail banks — the containing fold the pos
    // wears (floor(14.0, 35.0, 12.0) the (14, 35, 12) block keeps)
    while app.frame < at {
        app.update();
    }
    let line = app
        .game_events
        .iter()
        .find(|l| l.starts_with("warden_disturbance @"))
        .expect("the arrival the disturbance record banks");
    assert!(
        line.contains("x14 y35 z12"),
        "the containing fold the pos wears — the block the vibration rode: {line}"
    );
    assert!(
        line.contains("e100"),
        "the 100l expiry the record wears: {line}"
    );
    // the memory the entity banks — the bank tick the drain never
    // precedes (the tick walk the drain serves BEFORE the hears
    // walk the same frame banks)
    let d = app.wardens[0]
        .disturbance
        .expect("the memory the tail banks");
    assert_eq!(d.pos, (14, 35, 12), "the containing fold the memory wears");
    assert_eq!(
        d.remaining,
        crate::sculk::WARDEN_DISTURBANCE_EXPIRY,
        "the bank tick the drain never precedes"
    );
    // THE EXPIRY the 100 updates the memory drops — the drain the
    // zero the None answers
    for _ in 0..crate::sculk::WARDEN_DISTURBANCE_EXPIRY {
        app.update();
    }
    assert!(
        app.wardens[0].disturbance.is_none(),
        "the expiry the memory drops — the 100 ticks the walk drains"
    );
}

/// THE ANGRY TAIL THE REFUSAL KEEPS (javap onReceiveVibration
/// pc 176-182 — the isAngry head the tail never enters; javap
/// setDisturbanceLocation pc 14-21 — the ANGRY-at gate the direct
/// call refuses; the trio pc 197-217 — the mismatch the
/// `if_acmpne 229` refuses, the same-suspect pass the identity
/// keeps).
#[test]
fn the_disturbance_the_angry_the_refusal_keeps() {
    // THE ANGRY HEAD: the +100 hurt the ANGRY level banks — the
    // hearing the tail never enters (the ANGRY-at gate the same
    // verdict the direct call wears)
    let mut w = crate::sculk::WardenMob::new(3, 0.0, 40.0, 0.0);
    w.hurt_from_player(7.0, true);
    assert!(
        crate::sculk::warden_anger_level(w.active_anger()).is_angry(),
        "the +100 the ANGRY verdict banks"
    );
    let hear = w
        .hear_vibration(
            Some(crate::sculk::WARDEN_PLAYER_SUSPECT),
            None,
            None,
            true,
            true,
            true,
            (16, 34, 0),
        )
        .expect("the living warden the hearing speaks");
    assert!(
        hear.disturbance.is_none(),
        "the ANGRY head the tail refuses — the memory the walk never banks"
    );
    w.set_disturbance_location((16, 34, 0));
    assert!(
        w.disturbance.is_none(),
        "the ANGRY-at gate the direct call refuses"
    );

    // THE MISMATCH the trio refuses: the top suspect 7 the
    // vibration entity 1 — the identity the if_acmpne wears (the
    // NON-player flag the store's player-priority sort never
    // arms — the tie the stable order keeps: 7 the head wears)
    let mut m = crate::sculk::WardenMob::new(4, 0.0, 40.0, 0.0);
    m.anger.increase(7, 35);
    let hear2 = m
        .hear_vibration(Some(1), None, None, false, true, true, (16, 34, 0))
        .expect("the hearing speaks");
    assert!(
        hear2.disturbance.is_none(),
        "the mismatch the trio refuses — the top suspect the identity wears"
    );

    // THE SAME-SUSPECT pass the trio keeps: the vibration entity 7
    // the top suspect the identity answers
    m.vibration_cooldown = 0;
    let hear3 = m
        .hear_vibration(Some(7), None, None, false, true, true, (16, 34, 0))
        .expect("the hearing speaks");
    assert_eq!(
        hear3.disturbance.expect("the same-suspect pass banks").pos,
        (16, 34, 0),
        "the vibration pos the memory wears"
    );
    assert_eq!(
        m.disturbance.expect("the memory banks").remaining,
        crate::sculk::WARDEN_DISTURBANCE_EXPIRY,
        "the 100l expiry the fresh bank wears"
    );
}

/// THE OWNER PASS THE POS RE-BANKS (javap onReceiveVibration
/// pc 98-115 — the RECENT arm the canTarget consult the owner's
/// blockPosition re-banks, the disturbance the re-banked pos
/// wears; the fresh warden the RECENT arm the +35 wears, the
/// else arm the +10 keeps).
#[test]
fn the_disturbance_the_owner_pass_the_pos_re_banks() {
    // THE RECENT ARM: the memory armed, the owner the within-30
    // wears — the pos the OWNER's blockPosition re-banks (the
    // floor(2.0, 40.0, 2.0) the (2, 40, 2) block keeps, the
    // vibration pos (16, 34, 0) the re-bank never wears)
    let mut w = crate::sculk::WardenMob::new(6, 0.0, 40.0, 0.0);
    w.recent_projectile = crate::sculk::WARDEN_RECENT_PROJECTILE_TICKS;
    let hear = w
        .hear_vibration(
            None,
            Some(crate::sculk::WARDEN_PLAYER_SUSPECT),
            Some((2.0, 40.0, 2.0)),
            false,
            true,
            true,
            (16, 34, 0),
        )
        .expect("the hearing speaks");
    assert_eq!(
        w.active_anger(),
        crate::sculk::WARDEN_DEFAULT_ANGER,
        "the RECENT arm the +35 wears — the within-30 fold the d2 = 8 answers"
    );
    let d = hear
        .disturbance
        .expect("the owner pass the disturbance banks");
    assert_eq!(
        d.pos,
        (2, 40, 2),
        "the owner's blockPosition the pos re-banks — the floor law the containing wears"
    );

    // THE FRESH WARDEN the else arm the +10 wears — the pos the
    // vibration pos wears (the re-bank the no-RECENT arm never
    // reaches)
    let mut f = crate::sculk::WardenMob::new(7, 0.0, 40.0, 0.0);
    let hear2 = f
        .hear_vibration(
            None,
            Some(crate::sculk::WARDEN_PLAYER_SUSPECT),
            Some((2.0, 40.0, 2.0)),
            false,
            true,
            true,
            (16, 34, 0),
        )
        .expect("the hearing speaks");
    assert_eq!(
        f.active_anger(),
        crate::sculk::WARDEN_PROJECTILE_ANGER,
        "the no-RECENT memory the else arm's +10 wears"
    );
    assert_eq!(
        hear2.disturbance.expect("the owner pass banks").pos,
        (16, 34, 0),
        "the vibration pos the memory wears — the re-bank the else arm never reaches"
    );
}

/// THE TOUCH THE CONTACT THE ANGER CLIMBS (javap `Warden.doPush`
/// pc 1-51 + the `LivingEntity.pushEntities` caller — the bb query
/// the intersect walk answers, the TOUCH_COOLDOWN 20l the contact
/// banks, the increaseAngerAt(entity) the (35, true) fold wears,
/// the setDisturbanceLocation the touched entity's own block pos
/// banks, the standing cooldown the next touch refuses, the drain
/// the 20th update the walk re-arms).
#[test]
fn the_touch_the_contact_the_anger_climbs() {
    // === THE SETUP (the disturbance test's own floor — the warden
    // the zombie's body touches, the body the None keeps — the
    // player arm the scan never serves) ===
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.wardens
        .push(crate::sculk::WardenMob::new(101, 12.5, 35.0, 12.5));
    // THE SNIFF LANE THE TEST ISOLATES (the standing SNIFF_COOLDOWN
    // memory the trigger's absent gate refuses — the value the
    // vanilla 100..200 range the isolation stretches, the law the
    // number never serves; the drain the 1/tick walk the test's
    // window never exhausts)
    app.wardens[0].sniff_cooldown = i32::MAX;
    app.mobs
        .push(crate::mob::Zombie::new(1, 12.5, 35.0, 12.5, 7));
    assert!(
        app.body.is_none(),
        "the body the None keeps — the player arm the scan never serves"
    );

    // === THE FIRST CONTACT (the pushEntities walk the frame
    // serves — the boxes the intersect answers, the doPush the
    // touch speaks: the +35 the store climbs, the 20l the memory
    // banks, the disturbance the zombie's own block pos wears) ===
    app.update();
    let touches: Vec<&String> = app
        .game_events
        .iter()
        .filter(|l| l.starts_with("warden_touch @"))
        .collect();
    assert_eq!(
        touches.len(),
        1,
        "the contact the ONE touch banks: {:?}",
        touches
    );
    assert!(
        touches[0].contains("suspect1 +35 -> 35"),
        "the ONE-arg fold the +35 the POST-increase verdict banks: {}",
        touches[0]
    );
    let dist_line = app
        .game_events
        .iter()
        .find(|l| l.starts_with("warden_disturbance @"))
        .expect("the touch the disturbance record banks");
    assert!(
        dist_line.contains("x12 y35 z12 e100"),
        "the touched entity's own block pos the containing fold wears: {dist_line}"
    );
    assert_eq!(
        app.wardens[0].touch_cooldown,
        crate::sculk::WARDEN_TOUCH_COOLDOWN_TICKS,
        "the setMemoryWithExpiry 20l the contact banks"
    );
    assert_eq!(
        app.wardens[0].active_anger(),
        35,
        "the increaseAngerAt(entity) the anger climbs"
    );
    assert_eq!(
        app.wardens[0].disturbance.expect("the memory banks").pos,
        (12, 35, 12),
        "the zombie's own feet the floor law answers"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.warden.listening ")),
        "the Calm listening the 35-anger verdict plays"
    );

    // === THE IMMEDIATE SECOND FRAME (the standing cooldown the
    // hasMemoryValue gate refuses — the anger the refusal keeps,
    // the tick-2 the decay never walks) ===
    app.update();
    let touches: Vec<&String> = app
        .game_events
        .iter()
        .filter(|l| l.starts_with("warden_touch @"))
        .collect();
    assert_eq!(
        touches.len(),
        1,
        "the standing memory the anger arm refuses"
    );
    assert_eq!(
        app.wardens[0].active_anger(),
        35,
        "the refused touch the anger never climbs — the tick-2 the decay never walks"
    );

    // === THE DRAIN THE WALK RE-ARMS (the 20l expiry the updates
    // drain, the 20th-tick decay the anger wears, the 21st update
    // the touch re-fires — the 34 + 35 the store banks) ===
    for _ in 0..19 {
        let z = &mut app.mobs[0];
        z.x = 12.5;
        z.y = 35.0;
        z.z = 12.5;
        app.update();
    }
    let touches: Vec<&String> = app
        .game_events
        .iter()
        .filter(|l| l.starts_with("warden_touch @"))
        .collect();
    assert_eq!(
        touches.len(),
        2,
        "the drained memory the walk re-arms: {:?}",
        touches
    );
    assert!(
        touches[1].contains("suspect1 +35 -> 69"),
        "the decayed 34 the re-fire +35 banks: {}",
        touches[1]
    );
    assert_eq!(
        app.wardens[0].touch_cooldown,
        crate::sculk::WARDEN_TOUCH_COOLDOWN_TICKS,
        "the re-fire the 20l re-banks"
    );
}

/// THE TOUCH THE GATES THE REFUSALS KEEP (javap doPush pc 7-17 —
/// the standing cooldown the immediate second call refuses; javap
/// increaseAngerAt pc 9-12 — the NOT-targetable suspect the
/// canTarget gate refuses while the cooldown set pc 20-33 AND the
/// disturbance tail pc 41-46 the walk STILL serves; the pushable
/// scan's alive verdicts the dead pair keeps).
#[test]
fn the_touch_the_gates_the_refusals_keep() {
    // === THE NOT-TARGETABLE WALK (the creative/spectator suspect
    // the canTarget refusal wears — the cooldown + the disturbance
    // the refusal NEVER lifts) ===
    let mut w = crate::sculk::WardenMob::new(11, 2.5, 40.0, 2.5);
    let touch = w.do_push(
        crate::sculk::WARDEN_PLAYER_SUSPECT,
        (2.4, 40.0, 2.4),
        true,
        false,
    );
    assert_eq!(
        w.touch_cooldown,
        crate::sculk::WARDEN_TOUCH_COOLDOWN_TICKS,
        "the pc 20-33 set the increase NEVER lifts"
    );
    assert_eq!(
        w.active_anger(),
        0,
        "the canTarget gate the anger refuses — the +35 the refusal keeps"
    );
    assert!(
        touch.unwrap().listening.is_none(),
        "the refused increase the listening never arms"
    );
    assert_eq!(
            w.disturbance.expect("the tail banks").pos,
            (2, 40, 2),
            "the pc 41-46 the disturbance the walk STILL serves — the (2, 40, 2) block the floor law answers"
        );

    // === THE STANDING COOLDOWN (the immediate second call the
    // pc 7-17 gate refuses — the walk's None the arm skips) ===
    assert!(
        w.do_push(
            crate::sculk::WARDEN_PLAYER_SUSPECT,
            (2.4, 40.0, 2.4),
            true,
            true
        )
        .is_none(),
        "the standing TOUCH_COOLDOWN the anger arm refuses"
    );
    assert_eq!(
        w.active_anger(),
        0,
        "the refused call the anger never climbs"
    );

    // === THE DRAIN (the 20 updates the memory expires — the walk
    // re-fires the targetable suspect the +35 serves) ===
    for _ in 0..crate::sculk::WARDEN_TOUCH_COOLDOWN_TICKS {
        w.tick();
    }
    assert_eq!(w.touch_cooldown, 0, "the expiry the drain answers");
    let touch = w.do_push(
        crate::sculk::WARDEN_PLAYER_SUSPECT,
        (2.4, 40.0, 2.4),
        true,
        true,
    );
    assert!(
        touch.unwrap().listening.is_some(),
        "the re-fired walk the targetable +35 the listening arms"
    );
    assert_eq!(
        w.active_anger(),
        35,
        "the re-fire the anger climbs — the decayed store the +35 wears"
    );
}

/// THE PUSHABLE SCAN THE ALIVE PAIR KEEPS (javap
/// `LivingEntity.pushEntities` — the bb query the intersect walk
/// answers, the far body the box test refuses, the dead warden
/// the alive gate keeps, the dead zombie the pushable verdict
/// keeps out).
#[test]
fn the_pushable_scan_the_alive_pair_keeps() {
    // === THE FOUR WARDENS (the live+contact / the live+far / the
    // dead warden+live contact / the live+dead contact — the ONE
    // update the walk serves all) ===
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // A: the live warden the live zombie touches
    app.wardens
        .push(crate::sculk::WardenMob::new(101, 12.5, 35.0, 12.5));
    // B: the live warden no body near
    app.wardens
        .push(crate::sculk::WardenMob::new(102, 20.5, 35.0, 20.5));
    // C: the DEAD warden the live zombie touches
    let mut dead = crate::sculk::WardenMob::new(103, 24.5, 35.0, 24.5);
    dead.health = 0.0;
    app.wardens.push(dead);
    // D: the live warden the DEAD zombie touches
    app.wardens
        .push(crate::sculk::WardenMob::new(104, 28.5, 35.0, 28.5));
    app.mobs
        .push(crate::mob::Zombie::new(1, 12.5, 35.0, 12.5, 7));
    app.mobs
        .push(crate::mob::Zombie::new(2, 24.5, 35.0, 24.5, 8));
    let mut corpse = crate::mob::Zombie::new(3, 28.5, 35.0, 28.5, 9);
    corpse.health = 0.0;
    app.mobs.push(corpse);

    app.update();
    let touches: Vec<&String> = app
        .game_events
        .iter()
        .filter(|l| l.starts_with("warden_touch @"))
        .collect();
    assert_eq!(
        touches.len(),
        1,
        "the live contact the ONLY touch banks: {:?}",
        touches
    );
    assert!(
        touches[0].contains("suspect1 +35 -> 35"),
        "the A warden the zombie suspect banks: {}",
        touches[0]
    );
    // A: the touched state banks
    assert_eq!(app.wardens[0].active_anger(), 35, "the A the anger climbs");
    assert_eq!(
        app.wardens[0].touch_cooldown,
        crate::sculk::WARDEN_TOUCH_COOLDOWN_TICKS,
        "the A the 20l banks"
    );
    // B: the far body the box test refuses — the fresh state keeps
    assert_eq!(
        app.wardens[1].active_anger(),
        0,
        "the B the box never intersects — the anger the fresh 0 keeps"
    );
    assert_eq!(
        app.wardens[1].touch_cooldown, 0,
        "the B the memory never banks"
    );
    assert!(
        app.wardens[1].disturbance.is_none(),
        "the B the eyes never open"
    );
    // C: the dead warden the alive gate keeps
    assert_eq!(
        app.wardens[2].active_anger(),
        0,
        "the C the dead body the scan keeps out — the anger never climbs"
    );
    assert_eq!(
        app.wardens[2].touch_cooldown, 0,
        "the C the memory never banks"
    );
    // D: the dead zombie the pushable verdict keeps out
    assert_eq!(
        app.wardens[3].active_anger(),
        0,
        "the D the dead zombie the pushable list never joins — the anger never climbs"
    );
    assert_eq!(
        app.wardens[3].touch_cooldown, 0,
        "the D the memory never banks"
    );
    assert!(
        app.wardens[3].disturbance.is_none(),
        "the D the eyes never open"
    );
}

/// THE CRAMMING THE GATES THE DAMAGE WEARS (javap
/// `LivingEntity.pushEntities` pc 28-156 — the gates the order the
/// pc walk wears: the 0/negative gamerule the arm refuses, the few
/// entities the `size() > max - 1` STRICT fold never arms, the
/// 1-in-4 draw the ENTITY'S OWN stream spends — the refusals the
/// draw NEVER refund, the count gate the draw the spent wear
/// keeps, the RAW 6.0f the health folds, the null attacker the
/// anger NEVER banks).
#[test]
fn the_cramming_the_gates_the_damage_wears() {
    let mut w = crate::sculk::WardenMob::new(900, 0.0, 0.0, 0.0);
    // === THE DISABLED RULE (pc 57-58): the `ifle 157` the 0 the
    // arm NEVER enters — the rng the gate never draws ===
    let fresh = w.rng.clone();
    assert!(
        w.cramming_hurt(24, 24, 0).is_none(),
        "the 0 gamerule the cramming NEVER arms"
    );
    assert_eq!(w.rng, fresh, "the disabled rule the stream never spends");
    // === THE FEW ENTITIES (pc 61-70): the size 23 the
    // `23 > 23` STRICT fold refuses — the draw the gate never
    // reaches ===
    assert!(
        w.cramming_hurt(23, 23, crate::sculk::MAX_ENTITY_CRAMMING_DEFAULT)
            .is_none(),
        "the few entities the branch never arms"
    );
    assert_eq!(w.rng, fresh, "the size gate the refusal the stream keeps");
    // === THE SIZE GATE THE DRAW SPENDS (pc 73-83): the size 24
    // the draw rides — the unlucky seed the 1-in-4 refuses, the
    // stream the SPENT wear keeps (the refusal the NEVER refunds)
    let mut seed = 1u64;
    loop {
        let mut p = crate::drops::Lcg::new(seed);
        if p.next_int(4) != 0 {
            break;
        }
        seed += 1;
    }
    w.rng = crate::drops::Lcg::new(seed);
    let pre = w.rng.clone();
    assert!(
        w.cramming_hurt(24, 24, crate::sculk::MAX_ENTITY_CRAMMING_DEFAULT)
            .is_none(),
        "the 1-in-4 draw the unlucky walk refuses"
    );
    assert_ne!(
        w.rng, pre,
        "the draw the stream spends — the refusal NEVER refunds"
    );
    // === THE COUNT GATE THE DRAW STILL SPENDS (pc 86-138): the
    // passengers the crowd the count refuses — the draw the pc
    // order the BEFORE wears (the pc 73-83 the pc 133-138
    // precedes) ===
    let mut seed = 1u64;
    loop {
        let mut p = crate::drops::Lcg::new(seed);
        if p.next_int(4) == 0 {
            break;
        }
        seed += 1;
    }
    w.rng = crate::drops::Lcg::new(seed);
    let pre = w.rng.clone();
    assert!(
        w.cramming_hurt(24, 23, crate::sculk::MAX_ENTITY_CRAMMING_DEFAULT)
            .is_none(),
        "the passengers the count gate refuses"
    );
    assert_ne!(w.rng, pre, "the count gate the draw the spent wear keeps");
    // === THE DAMAGE (pc 141-156): the lucky seed the crowd the
    // hurt wears — the RAW 6.0f the 500-body folds, the hurt voice
    // the pipeline answers, the pitch the two-draw the voice
    // spends, THE ANGER the null attacker NEVER banks ===
    w.rng = crate::drops::Lcg::new(seed);
    let cram = w
        .cramming_hurt(24, 24, crate::sculk::MAX_ENTITY_CRAMMING_DEFAULT)
        .expect("the crowd the hurt wears");
    assert_eq!(
        w.health, 494.0,
        "the RAW 6.0f the health folds — the NO-ARMOR rows keep"
    );
    assert_eq!(
        cram.health_after, 494.0,
        "the POST-damage health the verdict banks"
    );
    assert!(!cram.died, "the wounded the false keeps");
    assert_eq!(
        cram.word, "entity.warden.hurt",
        "the hurt voice the pipeline answers"
    );
    assert!(cram.pitch > 0.0, "the two-draw pitch the voice wears");
    assert_eq!(
        w.active_anger(),
        0,
        "the null attacker the anger NEVER climbs"
    );
    assert!(
        w.disturbance.is_none(),
        "the disturbance the null pos NEVER banks"
    );
    assert_eq!(w.touch_cooldown, 0, "the cramming the touch never banks");
    // === THE DEATH (the same pipeline the 5-body wears): the
    // 5.0 - 6.0 the isDeadOrDying the TRUE answers, the WARDEN_DEATH
    // the word keeps ===
    w.health = 5.0;
    w.rng = crate::drops::Lcg::new(seed);
    let cram = w
        .cramming_hurt(24, 24, crate::sculk::MAX_ENTITY_CRAMMING_DEFAULT)
        .expect("the wounded the hurt wears");
    assert!(cram.died, "the 5.0 - 6.0 the death wears");
    assert_eq!(
        cram.word, "entity.warden.death",
        "the death voice the pipeline answers"
    );
    assert_eq!(cram.health_after, -1.0, "the negative the raw fold wears");
    // === THE DEAD HEAD (the isDeadOrDying the pipeline keeps):
    // the hurt refuses — the draw the pc order the spent wear keeps
    w.health = 0.0;
    w.rng = crate::drops::Lcg::new(seed);
    let pre = w.rng.clone();
    assert!(
        w.cramming_hurt(24, 24, crate::sculk::MAX_ENTITY_CRAMMING_DEFAULT)
            .is_none(),
        "the isDeadOrDying head the hurt refuses"
    );
    assert_ne!(w.rng, pre, "the dead head the draw the spent wear keeps");
}

/// THE CRAMMING THE PUSH WALK THE ORDER SERVES (the frame walk's
/// own restructure the 24-zombie crowd wears: the query pc 0-12
/// the list banks, the empty check pc 13-22 keeps, the cramming
/// arm pc 28-156 the touches PRECEDE — the record the pc order
/// banks, the hurt voice the pipeline plays, the touch loop
/// pc 157-191 the ONE touch the standing cooldown keeps, the +35
/// the anger the touch alone banks).
#[test]
fn the_cramming_the_push_walk_the_order_serves() {
    // === THE SETUP (the touch test's own floor — the warden the
    // 24-zombie crowd the box meets: the size 24 the `> 23` STRICT
    // fold the branch arms) ===
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.wardens
        .push(crate::sculk::WardenMob::new(101, 12.5, 35.0, 12.5));
    for i in 0..24 {
        app.mobs.push(crate::mob::Zombie::new(
            1000 + i as i32,
            12.5,
            35.0,
            12.5,
            7,
        ));
    }
    // THE LUCKY SEED (the per-tick stream the ambient draw rides
    // FIRST — the second draw the cramming's own 1-in-4 spends):
    // the probe the TWO draws consults
    let mut seed = 1u64;
    loop {
        let mut p = crate::drops::Lcg::new(seed);
        p.next();
        if p.next_int(4) == 0 {
            break;
        }
        seed += 1;
    }
    app.wardens[0].rng = crate::drops::Lcg::new(seed);
    // === THE FRAME (the push walk the cramming arm the touches
    // PRECEDE — the pc order the records wear) ===
    for z in &mut app.mobs {
        z.x = 12.5;
        z.y = 35.0;
        z.z = 12.5;
    }
    app.update();
    // THE CRAMMING RECORD THE TOUCHES PRECEDE (the pc 141-156 the
    // pc 157-191 precedes — the game_events order the walk wears)
    let cram_ix = app
        .game_events
        .iter()
        .position(|l| l.starts_with("warden_cramming @"))
        .expect("the crowd the hurt record banks");
    let touch_ix = app
        .game_events
        .iter()
        .position(|l| l.starts_with("warden_touch @"))
        .expect("the touch the contact banks");
    assert!(
        cram_ix < touch_ix,
        "the cramming arm the doPush loop PRECEDES: {:?}",
        &app.game_events[cram_ix..=touch_ix]
    );
    let cram_line = &app.game_events[cram_ix];
    assert!(
        cram_line.contains("d6 -> 494"),
        "the RAW 6.0f the 500-body folds: {}",
        cram_line
    );
    assert_eq!(
        app.wardens[0].health, 494.0,
        "the health the raw fold wears"
    );
    // THE ANGER THE TOUCH ALONE BANKS (the cramming the null
    // attacker the store NEVER touches — the +35 the doPush speaks)
    assert_eq!(
        app.wardens[0].active_anger(),
        35,
        "the touch the +35 banks — the cramming the anger NEVER climbs"
    );
    // THE CROWD THE ONE TOUCH (the TOUCH_COOLDOWN gate the loop's
    // own refusals keep — the 24 bodies the ONE anger arm wears)
    assert_eq!(
        app.game_events
            .iter()
            .filter(|l| l.starts_with("warden_touch @"))
            .count(),
        1,
        "the standing cooldown the crowd the ONE touch banks"
    );
    // THE HURT VOICE (the pipeline the WARDEN_HURT plays — the
    // getSoundVolume 4.0 the lane speaks)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.warden.hurt ")),
        "the cramming hurt the voice plays: {:?}",
        app.sounds.events
    );
}

/// THE CRAMMING THE FEW THE LANE KEEPS (the 23-zombie crowd the
/// size gate refuses — the `23 > 23` STRICT fold the FALSE keeps,
/// the draw the gate never spends, the health the fresh body
/// keeps, the hurt voice never plays — AND the touch lane the
/// restructure never breaks: the same list the doPush loop the
/// 555 law serves).
#[test]
fn the_cramming_the_few_the_lane_keeps() {
    // === THE SETUP (the 23-zombie crowd — ONE below the arm) ===
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.wardens
        .push(crate::sculk::WardenMob::new(101, 12.5, 35.0, 12.5));
    for i in 0..23 {
        app.mobs.push(crate::mob::Zombie::new(
            1000 + i as i32,
            12.5,
            35.0,
            12.5,
            7,
        ));
    }
    // the lucky seed anyway — the boundary the refusal keeps (the
    // draw the size gate never spends)
    let mut seed = 1u64;
    loop {
        let mut p = crate::drops::Lcg::new(seed);
        p.next();
        if p.next_int(4) == 0 {
            break;
        }
        seed += 1;
    }
    app.wardens[0].rng = crate::drops::Lcg::new(seed);
    for z in &mut app.mobs {
        z.x = 12.5;
        z.y = 35.0;
        z.z = 12.5;
    }
    app.update();
    // THE FEW THE BRANCH NEVER ARMS: no cramming record, the
    // health the fresh body keeps, the hurt voice never plays
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("warden_cramming @")),
        "the 23 the size gate refuses: {:?}",
        app.game_events
            .iter()
            .filter(|l| l.contains("cramming"))
            .collect::<Vec<_>>()
    );
    assert_eq!(
        app.wardens[0].health,
        crate::sculk::WARDEN_MOB_MAX_HEALTH,
        "the refusal the health keeps"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .all(|e| !e.starts_with("entity.warden.hurt ")),
        "the hurt voice the refusal never plays"
    );
    // THE TOUCH LANE THE RESTRUCTURE NEVER BREAKS (the same list
    // the doPush loop serves — the 555 law the restructure keeps)
    assert_eq!(
        app.game_events
            .iter()
            .filter(|l| l.starts_with("warden_touch @"))
            .count(),
        1,
        "the contact the ONE touch banks"
    );
    assert_eq!(
        app.wardens[0].active_anger(),
        35,
        "the touch the anger climbs"
    );
}

/// THE WARDEN'S EARS THE GLIDE THE LISTENING PLAYS (javap
/// `Warden$VibrationUser.onReceiveVibration` + the dispatch gates —
/// the glide event the pending hear banks, the arrival tick the
/// onReceiveVibration speaks, the tendril clicks + the anger + the
/// LISTENING voice the walk serves, the records the vocabulary
/// banks, the armed cooldown the second dispatch refuses).
#[test]
fn the_warden_ears_the_glide_the_listening_plays() {
    // === THE SUMMON (the walk-B's own setup — the stone floor the
    // warden lands) ===
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(14.0, 35.0, 14.0, 0.0, 0.0));
    app.place_sculk_shrieker(16, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 4;
    app.remove_sculk_shrieker(16, 35, 0);
    assert_eq!(app.wardens.len(), 1, "the spawn the ONE warden banks");
    // the warden the player's earshot moves — the (0.5, 0, -1.5)
    // offset the in-range verdict keeps
    app.wardens[0].x = 14.5;
    app.wardens[0].y = 35.0;
    app.wardens[0].z = 12.5;
    // THE SNIFF LANE THE TEST ISOLATES (the standing SNIFF_COOLDOWN
    // memory the trigger's absent gate refuses — the value the
    // vanilla 100..200 range the isolation stretches, the law the
    // number never serves; the drain the 1/tick walk the test's
    // window never exhausts)
    app.wardens[0].sniff_cooldown = i32::MAX;
    // THE EMERGE WINDOW THE TEST SKIPS (the finalizeSpawn's own
    // arm the summoned body wears — the isDiggingOrEmerging() gate
    // the hearing the window REFUSES; the steady-state body the
    // test's own lanes consult)
    app.wardens[0].emerging = false;
    app.wardens[0].emerge_ticks = 0;
    // the survival default the canTarget source verdict keeps
    assert!(
        app.game_mode.is_survival(),
        "the fresh App the survival default wears"
    );

    // === THE GLIDE EVENT THE LANE DRIVES (the every-10 gate the
    // body's own walk serves — the due flag the stamp carries, the
    // pos the PRE-travel field wears) ===
    {
        let b = app.body.as_mut().expect("the body rides");
        b.elytra_glide_event_due = true;
        b.elytra_glide_event_pos = (14.0, 35.0, 12.0);
    }
    let before = app.game_events.len();
    app.update();
    assert!(
        app.game_events[before..]
            .iter()
            .any(|l| l.starts_with("elytra_glide @")),
        "the glide record the sculk lane banks"
    );
    assert_eq!(
            app.warden_hears.len(),
            1,
            "the glide the warden's pending hear banks — the radius-16 + the occlusion + the source gates pass"
        );
    let pending = app.warden_hears[0];
    // the listener the EYE point the EntityPositionSource carries —
    // the distance the feet+2.465 offset folds
    let eye = f64::from(crate::sculk::WARDEN_EYE_HEIGHT);
    let (dx, dy, dz) = (14.0 - 14.5, 35.0 - (35.0 + eye), 12.0 - 12.5);
    let dist = (dx * dx + dy * dy + dz * dz).sqrt();
    assert_eq!(
        pending.at_frame,
        app.frame + crate::mth::floor(dist) as u64,
        "the arrival the floor fold banks"
    );
    assert_eq!(
        pending.vibration_entity,
        Some(crate::sculk::WARDEN_PLAYER_SUSPECT),
        "the vibrating entity the local suspect wears"
    );
    assert!(
        !app.game_events
            .iter()
            .any(|l| l.starts_with("warden_tendrils_clicking")),
        "the pre-arrival ticks the hearing never speaks"
    );
    assert_eq!(
        app.wardens[0].active_anger(),
        0,
        "the pre-arrival anger the fresh 0 keeps"
    );

    // === THE ARRIVAL (the onReceiveVibration walk the arrival
    // tick serves — the cooldown 40 arms, the tendril clicks the
    // v5.0 voice speaks, the anger +35 the default arm wears, the
    // CALM listening the 35-anger verdict answers) ===
    while app.frame < pending.at_frame {
        app.update();
    }
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("warden_tendrils_clicking @")),
        "the arrival the tendril-click record banks"
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("warden_anger @") && l.contains("suspect0 +35 -> 35")),
        "the anger record the +35 -> 35 verdict banks"
    );
    assert_eq!(
        app.wardens[0].active_anger(),
        35,
        "the default arm the anger climbs — the Calm level the 35 verdict wears"
    );
    assert_eq!(
        app.wardens[0].vibration_cooldown,
        crate::sculk::WARDEN_VIBRATION_COOLDOWN,
        "the arrival the 40-tick memory arms"
    );
    let listen_line = app
        .sounds
        .events
        .iter()
        .find(|e| e.starts_with("entity.warden.listening "));
    assert!(
        listen_line.is_some(),
        "the listening voice the arrival plays — the events lane banks: {:?}",
        app.sounds
            .events
            .iter()
            .filter(|e| e.contains("warden"))
            .collect::<Vec<_>>()
    );
    assert!(
        listen_line.unwrap().contains("v1.000000"),
        "the v10.0 the AL_MAX_GAIN close ceiling clamps: {:?}",
        listen_line
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.warden.tendril_clicks ")),
        "the tendril-click voice the always arm speaks"
    );

    // === THE SECOND GLIDE THE COOLDOWN REFUSES (the 40 the
    // arrival armed, the drain the 40-tick window keeps — the
    // dispatch the hasMemoryValue gate refuses) ===
    {
        let b = app.body.as_mut().expect("the body rides");
        b.elytra_glide_event_due = true;
        b.elytra_glide_event_pos = (14.0, 35.0, 12.0);
    }
    app.update();
    assert!(
        app.warden_hears.is_empty(),
        "the armed cooldown the dispatch refuses — the pending hear never banks"
    );
    assert_eq!(
        app.wardens[0].active_anger(),
        35,
        "the refused dispatch the anger never climbs"
    );
}

/// THE STEP THE SENSOR THE HEARING ANSWERS (javap
/// `vibrationAndSoundEffectsFromBlock` pc 69-90 — the walking
/// step's own `gameEvent(GameEvent.STEP, this.position(),
/// Context.of(this, state))` the dispatch serves: the step record
/// the lane banks (the r16 the register(name)'s default, the f1
/// the frequency map's own row), the registered sensor the hearing
/// answers, the arrival the activation schedule arms).
#[test]
fn the_step_the_sensor_the_hearing_answers() {
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(14.0, 35.0, 14.0, 0.0, 0.0));
    app.place_sculk_sensor(12, 35, 12);
    assert!(
        app.game_events.iter().all(|l| !l.starts_with("step @")),
        "the standing player the step never fires"
    );
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("sculk_hearing @")),
        "the standing player the hearing never speaks"
    );
    // THE WALK (the forward key the per-tick input drives — the
    // accumulator the *0.6 fold climbs, the 1.0 crossing the first
    // step fires, the nextStep reset the interval holds)
    for _ in 0..20 {
        app.on_movement_input(1.0, 0.0, false, false, false);
        app.update();
    }
    let step_count = app
        .game_events
        .iter()
        .filter(|l| l.starts_with("step @"))
        .count();
    assert!(
        step_count >= 1,
        "the walking player the step fires: {:?}",
        app.game_events
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("step @") && l.ends_with("r16 f1")),
        "the step the frequency-1 row wears"
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("sculk_hearing @")),
        "the step the registered sensor hears"
    );
    // the sensor's own re-arm the hearing banks (the free-at the
    // arrival + the ACTIVE 30 + the COOLDOWN 10 fold serves — the
    // clicking the arrival sweep may already have cleared, the
    // re-arm window the INACTIVE phase wears)
    assert!(
        app.sculk_sensors[0].free_at > 0,
        "the hearing the re-arm window arms"
    );
    // the STEP VOICE the sound arm speaks (the walkingStepSound
    // walk the playSound law rides — the block's step at the
    // SoundType's *0.15 fold)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.contains(".step ") || e.contains(".step@")),
        "the step voice the block's own row speaks: {:?}",
        app.sounds.events.iter().take(6).collect::<Vec<_>>()
    );
}

/// THE STEP THE WARDEN HEARS THE ANGER CLIMBS (javap
/// `onReceiveVibration` pc 0-229 — the walking suspect the
/// owner-null arm serves: the +35 the default wear, the listening
/// voice the Calm word answers, the armed cooldown 40 the next
/// steps refuse, the drain the ladder re-arms — the AGITATED
/// listening_angry the 70-anger verdict wears).
#[test]
fn the_step_the_warden_hears_the_anger_climbs() {
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    for x in 0..32 {
        for z in 0..32 {
            world.set(x, 34, z, crate::world::STONE);
        }
    }
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(14.0, 35.0, 14.0, 0.0, 0.0));
    app.place_sculk_shrieker(16, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 4;
    app.remove_sculk_shrieker(16, 35, 0);
    assert_eq!(app.wardens.len(), 1, "the spawn the ONE warden banks");
    // the warden the walk-path's earshot moves — the (2.5, 0, 0)
    // offset the in-range verdict keeps
    app.wardens[0].x = 16.5;
    app.wardens[0].y = 35.0;
    app.wardens[0].z = 14.0;
    // THE SNIFF LANE THE TEST ISOLATES (the standing SNIFF_COOLDOWN
    // memory the trigger's absent gate refuses — the value the
    // vanilla 100..200 range the isolation stretches, the law the
    // number never serves; the drain the 1/tick walk the test's
    // window never exhausts)
    app.wardens[0].sniff_cooldown = i32::MAX;
    // THE TOUCH LANE THE TEST ISOLATES (the standing TOUCH_COOLDOWN
    // memory the doPush's own gate refuses — the investigate
    // jitter's draws the LEVEL stream carries shifted when the
    // animate-tick sampler landed (the same stream the six
    // draws-per-sample ride — javap ClientLevel.doAnimateTick's
    // `getfield random` pair), the warden's walk target the
    // shifted jitter banks now crosses the retreating body's box
    // mid-walk: the vanilla touch lane's own +35 the isolation
    // keeps OUT of the hearing lane's pins, the law the drain the
    // window never exhausts serves)
    app.wardens[0].touch_cooldown = i32::MAX;
    // THE EMERGE WINDOW THE TEST SKIPS (the finalizeSpawn's own
    // arm the summoned body wears — the isDiggingOrEmerging() gate
    // the hearing the window REFUSES; the steady-state body the
    // test's own lanes consult)
    app.wardens[0].emerging = false;
    app.wardens[0].emerge_ticks = 0;
    // THE FIRST STEP THE HEARING BANKS (the walk the pending hear
    // pushes — the vibrating entity the local suspect wears)
    for _ in 0..12 {
        app.on_movement_input(1.0, 0.0, false, false, false);
        app.update();
    }
    assert!(
        !app.warden_hears.is_empty(),
        "the walking player the warden's pending hear banks"
    );
    let pending = app.warden_hears[0];
    assert_eq!(
        pending.vibration_entity,
        Some(crate::sculk::WARDEN_PLAYER_SUSPECT),
        "the vibrating entity the local suspect wears"
    );
    // THE ARRIVAL THE ANGER CLIMBS (the owner-null arm the +35
    // serves, the CALM listening the 35-anger verdict answers)
    while app.frame < pending.at_frame {
        app.update();
    }
    assert_eq!(
        app.wardens[0].active_anger(),
        35,
        "the first heard step the anger climbs"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.warden.listening ")),
        "the listening voice the arrival plays"
    );
    // THE WALK THE COOLDOWN OUTLASTS (the 40 the arrival armed —
    // the first ~3 steps the dispatch refuses; the decay the
    // 20-tick cadence wears — the anger the ladder NEVER crosses,
    // the Calm level the refused steps keep; the walk TURNS — the
    // earshot the oscillation keeps)
    for _ in 0..25 {
        app.on_movement_input(-1.0, 0.0, false, false, false);
        app.update();
    }
    assert!(
        app.warden_hears.is_empty(),
        "the armed cooldown the steps refuse"
    );
    assert!(
        app.wardens[0].active_anger() < 40,
        "the refused steps the ladder never crosses — the Calm level keeps"
    );
    // THE DRAIN THE LADDER RE-ARMS (the memory expires mid-walk —
    // the next step the hearing banks, the AGITATED verdict the
    // 70-anger answers, the listening_angry the pair wears)
    for _ in 0..60 {
        app.on_movement_input(1.0, 0.0, false, false, false);
        app.update();
    }
    // the second heard step the anger crosses (the pending hear
    // the walk's own drain already served — the arrival the walk
    // outlived — the AGITATED verdict the post-increase anger
    // answers, the +35 the default arm wears over the decayed
    // plateau)
    assert!(
        app.wardens[0].active_anger() >= 40,
        "the second heard step the ladder crosses — the Agitated level wears"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.warden.listening_angry ")),
        "the AGITATED verdict the listening_angry wears"
    );
}

/// THE FATIGUE THE DIG LANE THE APP WEARS (javap
/// `Player.getDestroySpeed` pcs 52-125 — the hasEffect(MINING_FATIGUE)
/// branch the tableswitch keeps): the branded body the per-tick
/// progress the ×0.3 speed wears (the live block row the
/// expectation reads, the completion the stretch the 2000-tick cap
/// keeps), the /effect give the store the app's own dig walk reads.
#[test]
fn the_fatigue_the_dig_lane_the_app_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.effects.add_effect(
        crate::effects::MOB_EFFECT_MINING_FATIGUE,
        0,
        600,
        crate::effects::DEFAULT_EFFECT_FLAGS,
    );
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1; // the top SOLID cell the ray hits
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    app.on_press(0, 0);
    assert!(app.dig.is_destroying, "the press armed the destroy");
    // THE FIRST MINING TICK (the fatigue the ×0.3 the per-tick
    // progress wears — the live block row + the dry grounded body
    // the exact f32 the mirror keeps)
    app.update();
    let kind = app.offline_world.as_ref().unwrap().get(c, surface, c);
    let (destroy_speed, requires) = crate::dig::DigState::block_dig_row(kind);
    let expected = crate::dig::DigState::destroy_progress(
        crate::dig::DigState::player_dig_speed(true, false, None, Some(0)),
        destroy_speed,
        !requires,
    );
    assert!(
        (app.dig.progress - expected).abs() < 1e-9,
        "the first tick the fatigued progress wears: {} vs {}",
        app.dig.progress,
        expected
    );
    // THE COMPLETION (the ×3.33 stretch the honest cap keeps — the
    // block the same lane the bare body's own <400 pace wore)
    let mut ticks = 1;
    while app.offline_world.as_ref().unwrap().get(c, surface, c) != crate::world::AIR {
        app.update();
        ticks += 1;
        assert!(ticks < 2000, "the dig completed in time");
    }
}

/// THE DIG THE SENSOR THE DESTROY HEARS (javap
/// `ServerPlayerGameMode.destroyBlock` pc 104-116 →
/// `Block.playerWillDestroy` pc 40-51 — the completion the
/// `gameEvent(BLOCK_DESTROY, pos, Context.of(player, state))` the
/// dug cell's center speaks, the f12 the frequency table's own row
/// wears): the held dig the block_destroy record banks, the
/// earshot sensor the sculk_hearing answers, the re-arm the
/// window arms — the sensors' hearing the dig lane joins.
#[test]
fn the_dig_the_sensor_the_destroy_hears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1; // the top SOLID cell the ray hits
                                   // the earshot sensor the air cell above the surface banks —
                                   // the (2, 2) offset the radius-16 gate passes, the rays the
                                   // wool-free terrain never refuse
    app.place_sculk_sensor(c + 2, surface + 2, c);
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("block_destroy @")),
        "the undug world the destroy never speaks"
    );
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    app.on_press(0, 0);
    assert!(app.dig.is_destroying, "the press armed the destroy");
    // THE HOLD: the ticks mine, the completion the event posts
    let mut ticks = 0;
    while app.offline_world.as_ref().unwrap().get(c, surface, c) != crate::world::AIR {
        app.update();
        ticks += 1;
        assert!(ticks < 400, "the dig completed in time");
    }
    // THE RECORD THE CENTER SPEAKS (the atCenterOf fold the +0.5
    // pair answers, the f12 the block_destroy row wears)
    let center = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        surface as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_destroy @")
                && l.contains(&center)
                && l.ends_with("r16 f12")),
        "the dig the block_destroy record at the cell center wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("sculk_hearing @")),
        "the earshot sensor the destroy hears"
    );
    assert!(
        app.sculk_sensors[0].free_at > 0,
        "the hearing the re-arm window arms"
    );
}

/// THE PLACE THE SENSOR THE PLACEMENT HEARS (javap
/// `BlockItem.place` pc 243-257 — the placement the
/// `gameEvent(BLOCK_PLACE, pos, Context.of(player, state))` the
/// placed cell's center speaks after the place sound, the f13 the
/// frequency table's own row wears): the held dirt the
/// block_place record banks, the earshot sensor the sculk_hearing
/// answers, the miss the press never speaks.
#[test]
fn the_place_the_sensor_the_placement_hears() {
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/dirt"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y; // the standing cell the straight-down ray fills
    app.place_sculk_sensor(c + 2, top + 2, c);
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("block_place @")),
        "the unplaced world the place never speaks"
    );
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the placement consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::DIRT,
        "the held dirt landed in the adjacent cell"
    );
    // THE RECORD THE CENTER SPEAKS (the placed cell's own +0.5
    // pair, the f13 the block_place row wears)
    let center = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        top as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_place @")
                && l.contains(&center)
                && l.ends_with("r16 f13")),
        "the placement the block_place record at the placed cell wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("sculk_hearing @")),
        "the earshot sensor the placement hears"
    );
    // THE MISS THE SILENCE KEEPS (the reach refusal the press never
    // consumes — the second placement cell the air the record
    // never banks)
    let records_before = app.game_events.len();
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 40.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(!app.on_right_press(0, 0), "the miss does not consume");
    assert_eq!(
        app.game_events.len(),
        records_before,
        "the miss the place record never banks"
    );
}

/// THE TILL THE SENSOR THE CHANGE HEARS (javap
/// `HoeItem.lambda$changeIntoState$0` pc 9-34 over the 26.2 jar —
/// the till consumer's setBlock pc 9-11 the BLOCK_CHANGE gameEvent
/// pc 15-34 FOLLOWS, the shovel's flatten the same tail
/// ShovelItem.useOn pc 180-202 wears — the converted cell's center
/// the +0.5 pair speaks, the f11 the block_change row keeps): the
/// hoe the grass tills, the record the tilled center banks, the
/// earshot sensor the sculk_hearing answers — the shovel the same
/// walk the DIRT_PATH swap wears.
#[test]
fn the_till_the_sensor_the_change_hears() {
    // === THE SETUP (the place test's own floor — the hoe the
    // surface's grass cell tills) ===
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/wooden_hoe"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1; // the top SOLID cell the ray hits
    app.place_sculk_sensor(c + 2, surface + 2, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, surface, c, crate::world::GRASS);
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("block_change @")),
        "the untilled world the change never speaks"
    );
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the till consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface, c),
        crate::world::FARMLAND,
        "the held hoe tilled the grass cell"
    );
    // THE RECORD THE CENTER SPEAKS (the atCenterOf fold the +0.5
    // pair answers, the f11 the block_change row wears)
    let center = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        surface as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_change @")
                && l.contains(&center)
                && l.ends_with("r16 f11")),
        "the till the block_change record at the cell center wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("sculk_hearing @")),
        "the earshot sensor the till hears"
    );
    // THE FLATTEN THE SAME WALK SPEAKS (the shovel the second map
    // arm — the DIRT_PATH the kind swap wears, the record the same
    // row keeps; the isolated events the clear keeps)
    app.offline_held_updated(Some("item/wooden_shovel"));
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, surface, c, crate::world::GRASS);
    app.game_events.clear();
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the flatten consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface, c),
        crate::world::DIRT_PATH,
        "the held shovel flattened the grass cell"
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_change @")
                && l.contains(&center)
                && l.ends_with("r16 f11")),
        "the flatten the block_change record the same row wears: {:?}",
        app.game_events
    );
}

/// THE DOUSE THE IGNITE THE CHANGE HEARS (javap ShovelItem.useOn
/// pc 95-202 — the campfire douse the flatten consult's ELSE arm
/// the same BLOCK_CHANGE tail wears; FlintAndSteelItem.useOn
/// pc 24-134 — the canLight branch the setBlock pc 95-100 the
/// gameEvent pc 101-108 follows): the shovel the lit campfire
/// douses, the flint the candle lights — the LIT swaps the
/// sensors hear, the voices the arms' own rows play.
#[test]
fn the_douse_the_ignite_the_change_hears() {
    // === THE SETUP (the lit campfire the shovel douses) ===
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/wooden_shovel"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    app.place_sculk_sensor(c + 2, surface + 2, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, surface, c, crate::world::CAMPFIRE_LIT);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the douse consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface, c),
        crate::world::CAMPFIRE,
        "the held shovel doused the lit campfire"
    );
    let center = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        surface as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_change @")
                && l.contains(&center)
                && l.ends_with("r16 f11")),
        "the douse the block_change record at the cell center wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.fire.extinguish ")),
        "the douse voice the levelEvent row plays"
    );
    // === THE IGNITE (the flint the candle lights — the LIT swap
    // the same row wears) ===
    app.offline_held_updated(Some("item/flint_and_steel"));
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, surface, c, crate::world::CANDLE);
    app.game_events.clear();
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the ignite consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface, c),
        crate::world::CANDLE_LIT,
        "the held flint lit the candle"
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_change @")
                && l.contains(&center)
                && l.ends_with("r16 f11")),
        "the ignite the block_change record the same row wears: {:?}",
        app.game_events
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.flintandsteel.use ")),
        "the ignite voice the FLINTANDSTEEL_USE row plays"
    );
}

/// THE CAKE PAIR THE CHANGE HEARS (javap CandleCakeBlock.useItemOn
/// pc 83-103 — the candle-on-cake swap the setBlockAndUpdate the
/// gameEvent pc 95-103 follows; AbstractCandleBlock.extinguish
/// pc 0-62 — the douse the setLit walk the BLOCK_CHANGE pc 56-62
/// tail wears): the held candle the plain cake takes, the empty
/// hand the lit candle-cake snuffs — the two swaps the sensors
/// hear, the eats the lane never speaks (the EAT gameEvent the
/// future round banks).
#[test]
fn the_cake_pair_the_change_hears() {
    // === THE SETUP (the candle the plain cake takes) ===
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/candle"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    app.place_sculk_sensor(c + 2, surface + 2, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, surface, c, crate::world::CAKE);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the swap consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface, c),
        crate::world::CANDLE_CAKE,
        "the held candle the plain cake took"
    );
    let center = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        surface as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_change @")
                && l.contains(&center)
                && l.ends_with("r16 f11")),
        "the swap the block_change record at the cake cell wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    // === THE DOUSE (the empty hand the lit candle-cake snuffs —
    // the upper-half hit the straight-down ray answers) ===
    app.offline_held_updated(None);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, surface, c, crate::world::CANDLE_CAKE_LIT);
    app.game_events.clear();
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the douse consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface, c),
        crate::world::CANDLE_CAKE,
        "the empty hand the lit candle-cake snuffed"
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_change @")
                && l.contains(&center)
                && l.ends_with("r16 f11")),
        "the douse the block_change record the same row wears: {:?}",
        app.game_events
    );
    assert!(
        app.game_events.iter().all(|l| !l.contains("f13")),
        "the eats the BLOCK_PLACE pair never speaks (the f11 rows alone)"
    );
}

/// THE FLINT FIRE THE PLACE HEARS (javap FlintAndSteelItem.useOn
/// pc 207-214 over the 26.2 jar — the placement branch's setBlock
/// pc 196-203 the `gameEvent(player, GameEvent.BLOCK_PLACE, pos)`
/// walk pc 207-214 follows — THE CLICKED POS the local-4 read
/// keeps: the `astore 6` relative cell the sound (pc 161) and the
/// set (pc 199) speak, the gameEvent's `aload 4` the clicked cell
/// the quirk the vanilla's own walk wears, the f13 the block_place
/// row keeps): the flint the stone top face sparks, the record the
/// CLICKED center banks, the fire cell the record never wears —
/// the non-air refusal the post never speaks.
#[test]
fn the_flint_fire_the_place_hears() {
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/flint_and_steel"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE STONE'S OWN STAGE (the placement test's own walk — the
    // hit cell the solid ground keeps, the target the honest air
    // wears)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::STONE);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top, c, crate::world::AIR);
    app.place_sculk_sensor(c + 2, top + 2, c);
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("block_place @")),
        "the unsparked world the place never speaks"
    );
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the spark consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::FIRE,
        "the fire cell the above-stone target wears"
    );
    // THE RECORD THE CLICKED CENTER SPEAKS (the local-4 quirk the
    // +0.5 pair answers — the f13 the block_place row wears)
    let clicked = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        (top - 1) as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_place @")
                && l.contains(&clicked)
                && l.ends_with("r16 f13")),
        "the spark the block_place record the CLICKED center wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    // THE FIRE CELL THE RECORD NEVER WEARS (the gameEvent's aload-4
    // the clicked slot reads — the `astore 6` fire cell the sound
    // and the set speak, the post the placement center never
    // banks)
    let fire_cell = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        top as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events.iter().all(|l| !l.contains(&fire_cell)),
        "the fire cell the record never wears (the local-4 quirk): {:?}",
        app.game_events
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("sculk_hearing @")),
        "the earshot sensor the spark hears"
    );
    // THE REFUSAL THE POST NEVER SPEAKS (the non-air target the
    // canBePlacedAt walk refuses — the second press the same
    // column aims, the fire cell the occupied target wears, the
    // FAIL the record never banks)
    let records_before = app.game_events.len();
    assert!(
        !app.on_right_press(0, 0),
        "the non-air target the press never consumes"
    );
    assert_eq!(
        app.game_events.len(),
        records_before,
        "the refusal the block_place record never banks"
    );
}

/// THE CHARGE FIRE THE PLACE HEARS (javap FireChargeItem.useOn
/// pc 127-136 over the 26.2 jar — the placement branch's
/// setBlockAndUpdate pc 116-123 the `gameEvent(player,
/// GameEvent.BLOCK_PLACE, pos)` walk pc 127-136 follows — THE
/// PLACEMENT CELL the reassigned local-3 read keeps: the pc 89-97
/// `astore_3` the relative cell the clickedPos slot REBANKS, the
/// flint's clicked-pos quirk the sibling NEVER wears, the f13 the
/// block_place row keeps): the charge the stone top face sparks,
/// the record the FIRE cell banks — the lit branch the same item
/// wears the f11 the BLOCK_CHANGE row keeps.
#[test]
fn the_charge_fire_the_place_hears() {
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/fire_charge"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::STONE);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top, c, crate::world::AIR);
    app.place_sculk_sensor(c + 2, top + 2, c);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the spark consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::FIRE,
        "the fire cell the above-stone target wears"
    );
    // THE RECORD THE FIRE CELL SPEAKS (the reassigned local-3 the
    // +0.5 pair answers — the f13 the block_place row wears)
    let fire_cell = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        top as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_place @")
                && l.contains(&fire_cell)
                && l.ends_with("r16 f13")),
        "the spark the block_place record the FIRE cell wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    // THE CLICKED CELL THE RECORD NEVER WEARS (the reassignment
    // the slot REBANKS — the stone cell the sound and the set
    // never speak the placement row)
    let clicked = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        (top - 1) as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events.iter().all(|l| !l.contains(&clicked)),
        "the clicked cell the placement record never wears: {:?}",
        app.game_events
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("sculk_hearing @")),
        "the earshot sensor the spark hears"
    );
    // THE LIT BRANCH THE SIBLING ROW KEEPS (the candle the same
    // item lights — the BLOCK_CHANGE the clicked center the f11
    // row wears, the placement row the consult never speaks)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::CANDLE);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top, c, crate::world::AIR);
    app.game_events.clear();
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the ignite consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top - 1, c),
        crate::world::CANDLE_LIT,
        "the held charge lit the candle"
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_change @")
                && l.contains(&clicked)
                && l.ends_with("r16 f11")),
        "the ignite the block_change record the clicked center wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("block_place @")),
        "the lit branch the placement row never speaks"
    );
}

/// THE SEEDS PLANT THE PLACE HEARS (javap BlockItem.place
/// pc 243-257 over the 26.2 jar — the plant arm's playSound
/// pc 208-240 the `gameEvent(GameEvent.BLOCK_PLACE, pos,
/// Context.of(player, state))` walk pc 243-257 follows, the
/// consume pc 260-265 the tail rides — the planted cell's center
/// the +0.5 pair speaks, the f13 the block_place row keeps): the
/// seeds the farmland floor plant, the record the planted cell
/// banks, the earshot sensor the sculk_hearing answers — the dirt
/// floor the SUPPORTS_CROPS refusal the post never speaks.
#[test]
fn the_seeds_plant_the_place_hears() {
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/wheat_seeds"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::FARMLAND);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top, c, crate::world::AIR);
    app.place_sculk_sensor(c + 2, top + 2, c);
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("block_place @")),
        "the unplanted world the place never speaks"
    );
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the plant consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::WHEAT,
        "the seeds the above-farmland cell planted"
    );
    // THE RECORD THE PLANTED CELL SPEAKS (the gameEvent's placed
    // pos the +0.5 pair answers — the f13 the block_place row
    // wears)
    let planted = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        top as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_place @")
                && l.contains(&planted)
                && l.ends_with("r16 f13")),
        "the plant the block_place record the planted cell wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("sculk_hearing @")),
        "the earshot sensor the plant hears"
    );
    // THE FLOOR REFUSAL THE POST NEVER SPEAKS (the plain dirt the
    // canSurvive floor refuses — the SUPPORTS_CROPS tag the sole
    // farmland row keeps, the FAIL the record never banks)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top, c, crate::world::AIR);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::DIRT);
    app.game_events.clear();
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the dirt floor the press never consumes"
    );
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("block_place @")),
        "the floor refusal the block_place record never banks: {:?}",
        app.game_events
    );
}

/// THE CAKE BITE THE SENSOR HEARS (javap CakeBlock.eat pc 44-50
/// over the 26.2 jar — the FoodData.eat pc 19-26 and the BITES
/// read pc 29-42 the `gameEvent(player, GameEvent.EAT, pos)` walk
/// pc 44-50 FOLLOWS, the bites<6 branch pc 55-88 the tail serves
/// — the cake cell's center the +0.5 pair speaks, the f8 the eat
/// row keeps): the hungry body the plain cake bites, the record
/// the cake center banks, the earshot sensor the sculk_hearing
/// answers — the ladder's first stage the destroy never speaks.
#[test]
fn the_cake_bite_the_sensor_hears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
            }
        }
        w.set(c, top, c, crate::world::CAKE);
    }
    app.place_sculk_sensor(c + 2, top + 2, c);
    assert!(
        app.game_events.iter().all(|l| !l.starts_with("eat @")),
        "the unbitten world the eat never speaks"
    );
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE HUNGRY BODY THE GATE ANSWERS (the bite test's own bar —
    // the 6 start the +2 feed the clamp never gates)
    app.food_data.set_food_level(6);
    app.food_data.set_saturation(0.0);
    assert!(app.on_right_press(0, 0), "the bite consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::CAKE_SLICE1,
        "the bites 0->1 branch the kind swap wears"
    );
    assert_eq!(app.food_data.food_level(), 8, "the +2 feed the bar wears");
    // THE RECORD THE CAKE CENTER SPEAKS (the gameEvent's pos the
    // +0.5 pair answers — the f8 the eat row wears)
    let center = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        top as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("eat @") && l.contains(&center) && l.ends_with("r16 f8")),
        "the bite the eat record the cake center wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("sculk_hearing @")),
        "the earshot sensor the bite hears"
    );
    // THE LADDER'S FIRST STAGE THE DESTROY NEVER SPEAKS (the
    // bites<6 branch the setBlock alone wears — the BLOCK_DESTROY
    // gameEvent the max-bite walk a later bite banks)
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("block_destroy @")),
        "the first bite the destroy never speaks: {:?}",
        app.game_events
    );
}

/// THE MAX BITE THE DESTROY WEARS (javap CakeBlock.eat pc 91-106
/// over the 26.2 jar — the seventh bite the bites >= 6 branch
/// serves: the removeBlock pc 91-99 the AIR returns, the
/// `gameEvent(player, GameEvent.BLOCK_DESTROY, pos)` walk
/// pc 100-106 FOLLOWS — the f12 the block_destroy row keeps, the
/// EAT post pc 44-50 the record PRECEDES the vanilla's own pc
/// order keeps): the ladder the six slices the eat row wears, the
/// seventh bite the destroy the sensors hear.
#[test]
fn the_cake_max_bite_the_destroy_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
            }
        }
        w.set(c, top, c, crate::world::CAKE);
    }
    app.place_sculk_sensor(c + 2, top + 2, c);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE HUNGRY BODY THE SEVEN BITES THE BAR CARRIES (the 18 the
    // seventh bite's canEat still answers — the ladder test's own
    // law)
    app.food_data.set_food_level(6);
    app.food_data.set_saturation(0.0);
    let ladder = [
        crate::world::CAKE_SLICE1,
        crate::world::CAKE_SLICE2,
        crate::world::CAKE_SLICE3,
        crate::world::CAKE_SLICE4,
        crate::world::CAKE_SLICE5,
        crate::world::CAKE_SLICE6,
    ];
    for (bite, expected) in ladder.iter().enumerate() {
        assert!(
            app.on_right_press(0, 0),
            "bite {bite}: the eat lane the press consumes"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            *expected,
            "bite {bite}: the stage the kind swap wears"
        );
    }
    // THE LADDER THE SIX EAT ROWS THE DESTROY NEVER SPEAKS (the
    // bites<6 branch the setBlock alone wears)
    let eats_after_ladder = app
        .game_events
        .iter()
        .filter(|l| l.starts_with("eat @"))
        .count();
    assert_eq!(eats_after_ladder, 6, "the six bites the six rows wear");
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("block_destroy @")),
        "the ladder the destroy never speaks"
    );
    // THE SEVENTH BITE THE AIR RETURNS (the removeBlock walk the
    // max-bite branch serves — the EAT row AND the destroy row
    // the same bite banks)
    assert!(app.on_right_press(0, 0), "the seventh bite consumes");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::AIR,
        "the cake the cell clears"
    );
    let center = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        top as f64 + 0.5,
        c as f64 + 0.5
    );
    let eats_after_max = app
        .game_events
        .iter()
        .filter(|l| l.starts_with("eat @"))
        .count();
    assert_eq!(eats_after_max, 7, "the seventh bite the eat row wears");
    let last_eat = app
        .game_events
        .iter()
        .rposition(|l| l.starts_with("eat @"))
        .expect("the eat rows the ladder banks");
    let destroy_pos = app
        .game_events
        .iter()
        .position(|l| l.starts_with("block_destroy @"))
        .expect("the max bite the destroy banks");
    assert!(
        destroy_pos > last_eat,
        "the destroy the EAT post FOLLOWS the vanilla's own pc order: {:?}",
        app.game_events
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_destroy @")
                && l.contains(&center)
                && l.ends_with("r16 f12")),
        "the max bite the block_destroy record the cake center wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("sculk_hearing @")),
        "the earshot sensor the ladder hears"
    );
}

/// THE CANDLE CAKE EAT THE SENSOR HEARS (javap CandleCakeBlock.
/// useWithoutItem pc 0-33 over the 26.2 jar — the douse consult's
/// refusal the CakeBlock.eat static the FRESH-CAKE state serves:
/// the same pc 44-50 EAT post the sibling wears, the bites 0->1
/// branch the CAKE_SLICE1 kind answers — the fresh stage the
/// destroy NEVER speaks, the f8 the eat row keeps): the empty
/// hand the lit-free cake eats, the record the cake center banks,
/// the candle drop the tail rides.
#[test]
fn the_candle_cake_eat_the_sensor_hears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        w.set(c, top, c, crate::world::CANDLE_CAKE);
    }
    app.place_sculk_sensor(c + 2, top + 2, c);
    assert!(
        app.game_events.iter().all(|l| !l.starts_with("eat @")),
        "the unbitten world the eat never speaks"
    );
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    app.food_data.set_food_level(6);
    app.food_data.set_saturation(0.0);
    assert!(app.on_right_press(0, 0), "the eat lane the press consumes");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::CAKE_SLICE1,
        "the fresh-CAKE state the bites 0->1 ladder wears"
    );
    assert_eq!(app.drops.len(), 1, "the candle drop the tail spawns");
    // THE RECORD THE CAKE CENTER SPEAKS (the same pc 44-50 walk
    // the static serves — the f8 the eat row wears)
    let center = format!(
        "x{:.6} y{:.6} z{:.6}",
        c as f64 + 0.5,
        top as f64 + 0.5,
        c as f64 + 0.5
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("eat @") && l.contains(&center) && l.ends_with("r16 f8")),
        "the candle cake's eat the record the cake center wears: {:?}",
        app.game_events.iter().rev().take(8).collect::<Vec<_>>()
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("sculk_hearing @")),
        "the earshot sensor the eat hears"
    );
    // THE FRESH STAGE THE DESTROY NEVER SPEAKS (the bites 0->1
    // branch the setBlock alone wears — the BLOCK_DESTROY the
    // max-bite walk a later bite banks)
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("block_destroy @")),
        "the fresh stage the destroy never speaks: {:?}",
        app.game_events
    );
}

/// THE DIG THE WARDEN THE PENDING BANKS (the shared dispatch's own
/// warden lane — the block_destroy event the dig completion posts
/// the warden's ears answer, the pending hear the arrival banks,
/// the +35 the anger ladder climbs, the Calm listening the
/// first-heard verdict wears): the digging suspect the warden
/// hears — the dig-near-the-warden stealth gameplay the client
/// speaks.
#[test]
fn the_dig_the_warden_the_pending_banks() {
    // === THE SUMMON (the walk-B's own setup — the terrain the
    // warden lands) ===
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    app.place_sculk_shrieker(c + 6, surface + 1, c + 6, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 4;
    app.remove_sculk_shrieker(c + 6, surface + 1, c + 6);
    assert_eq!(app.wardens.len(), 1, "the spawn the ONE warden banks");
    // the warden the dig-path's earshot moves — the (3, 0) offset
    // the radius-16 gate passes
    app.wardens[0].x = c as f64 + 3.5;
    app.wardens[0].y = surface as f64;
    app.wardens[0].z = c as f64 + 0.5;
    // THE EMERGE WINDOW THE TEST SKIPS (the finalizeSpawn's own
    // arm the summoned body wears — the isDiggingOrEmerging() gate
    // the hearing the window REFUSES; the steady-state body the
    // test's own lanes consult)
    app.wardens[0].emerging = false;
    app.wardens[0].emerge_ticks = 0;
    // THE DIG THE EVENT POSTS (the completion the shared dispatch
    // serves — the warden's pending hear the arrival banks)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    app.on_press(0, 0);
    let mut ticks = 0;
    while app.offline_world.as_ref().unwrap().get(c, surface, c) != crate::world::AIR {
        app.update();
        ticks += 1;
        assert!(ticks < 400, "the dig completed in time");
    }
    assert!(
        !app.warden_hears.is_empty(),
        "the digging player the warden's pending hear banks"
    );
    let pending = app.warden_hears[0];
    assert_eq!(
        pending.vibration_entity,
        Some(crate::sculk::WARDEN_PLAYER_SUSPECT),
        "the vibrating entity the local suspect wears"
    );
    // THE ARRIVAL THE ANGER CLIMBS (the owner-null arm the +35
    // serves, the Calm listening the first-heard verdict answers)
    while app.frame < pending.at_frame {
        app.update();
    }
    assert_eq!(
        app.wardens[0].active_anger(),
        35,
        "the first heard dig the anger climbs"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.warden.listening ")),
        "the listening voice the arrival plays"
    );
}

/// THE DARKNESS EFFECT THE STORE THE PULSE WEARS (javap the 26.2
/// jar Warden.applyDarknessAround + the effect-store's own laws +
/// LightmapRenderStateExtractor pc 304-344 — the removal tail the
/// 260-tick DARKNESS instance arms, the blend the 22-step fade
/// walks, the pulse the cosine fold breathes, the re-bake grain the
/// mesh's own re-walk wears, the expiry the client copy's own KEEP
/// the wire's Remove retires, the survival gate the creative mode
/// refuses).
#[test]
fn the_darkness_effect_the_store_the_pulse_wears() {
    // === THE REMOVAL TAIL ARMS THE STORE (the 547 walk-A's own
    // setup — the all-air net world the failed summon wears, the
    // darkness verdict the survival player's own fold answers)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 35.0, 0.0, 0.0, 0.0));
    app.place_sculk_shrieker(2, 35, 0, true);
    let f = app.frame;
    app.sculk_shrikers[0].free_at = f + 90;
    app.sculk_shrikers[0].warning_level = 4;
    app.remove_sculk_shrieker(2, 35, 0);
    {
        let inst = app
            .effects
            .get(crate::effects::MOB_EFFECT_DARKNESS)
            .expect("the effect rides");
        assert_eq!(
            inst.duration,
            crate::sculk::WARDEN_DARKNESS_TICKS,
            "the 260-tick instance the tail's own application banks"
        );
        assert_eq!(
            inst.amplifier, 0,
            "the amplifier the MobEffectInstance ctor keeps"
        );
        assert_eq!(
            inst.blend_factor, 0.0,
            "the fresh pair the fade-in walks from"
        );
    }
    assert!(
        app.darkness_bake.is_none(),
        "the pre-render floor the grain never arms — the walk the render serves"
    );
    // THE RENDER SEAT (the windowed loop's own split — the update
    // ticks, the render walks the lightmap state; the assets root
    // the tests' own helper discovers — the ancestor walk the
    // family's shared shape wears, the None the honest-skip keeps,
    // the jar-level root the GuiAssets root the bare family's own
    // seats answer)
    let Some(root) = assets_root_opt() else {
        return;
    };
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 180);
    // === THE PULSE (the update ticks the body + the store, the
    // render's own walk the scale + the grain wear): the mirror the
    // same f32 accumulation speaks, the world's own field the
    // render's write carries
    let step = 1.0_f32 / crate::effects::DARKNESS_BLEND_TICKS as f32;
    let mut mirror = 0.0_f32;
    for k in 1..=6u32 {
        app.update();
        app.render(&mut fb, &mut assets);
        mirror += (1.0_f32 - mirror).clamp(-step, step);
        let blend = app
            .effects
            .get_blend_factor(crate::effects::MOB_EFFECT_DARKNESS, app.partial_tick);
        assert_eq!(blend, mirror, "tick {k} the fade-in's own step wears");
        let scale = ((app.body.as_ref().unwrap().tick_count as f32 - app.partial_tick)
            * std::f32::consts::PI
            * 0.025)
            .cos()
            .max(0.0)
            * 0.45
            * blend;
        let world = app.net_world.as_ref().unwrap();
        assert_eq!(
            world.darkness_scale, scale,
            "tick {k} the world's own field the render's write banks"
        );
        if scale > 0.0 {
            assert_eq!(
                app.darkness_bake,
                Some((scale * 128.0).round() as u32),
                "tick {k} the grain the re-bake's own walk arms"
            );
        }
    }
    // === THE EXPIRY (the 260-tick window drains — the client
    // copy's own law the drained instance KEEPS: the wire's Remove
    // is the truth the store mirrors, the decrement the tail's own
    // walk banks — the blend the fade-out's completed 0.0 keeps,
    // the floor the render's own walk restores: the grain None the
    // field's 0.0 wears — the final render the re-bake's own
    // return speaks)
    for _ in 0..254 {
        app.update();
        // THE WALK'S OWN PIN (the all-air construct the fall never
        // grounds — the void's dose the living body's own expiry
        // walk refuses: the y the seat's 35.0 keeps, the fall the
        // zero velocity stills — the client copy's decrement the
        // living health gate the vanilla server's grounded word
        // wears)
        if let Some(b) = app.body.as_mut() {
            b.y = 35.0;
            b.vy = 0.0;
        }
    }
    let drained = app
        .effects
        .get(crate::effects::MOB_EFFECT_DARKNESS)
        .expect("the drained window the client copy's own law keeps");
    assert_eq!(
        drained.duration, 0,
        "the decrement the net lane's own tail banks"
    );
    app.update();
    app.render(&mut fb, &mut assets);
    assert_eq!(
        app.darkness_bake, None,
        "the expired effect the floor's grain restores"
    );
    assert_eq!(
        app.net_world.as_ref().unwrap().darkness_scale,
        0.0,
        "the floor the field's own identity keeps"
    );
    // THE WIRE'S OWN REMOVE (the sync's own law the snapshot's
    // dropped ids wear — the wire-less session the walk's own sync
    // call speaks, the drained corpse the empty snapshot retires)
    app.effects.sync_from_wire(&[]);
    assert!(
        app.effects
            .get(crate::effects::MOB_EFFECT_DARKNESS)
            .is_none(),
        "the wire's Remove the mirror's own walk lands"
    );
    // === THE CREATIVE GATE (the survival head the verdict refuses —
    // the store the removal never arms)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 35.0, 0.0, 0.0, 0.0));
    app.game_mode = GameMode::Creative;
    app.place_sculk_shrieker(2, 35, 0, true);
    app.sculk_shrikers[0].free_at = app.frame + 90;
    app.sculk_shrikers[0].warning_level = 4;
    app.remove_sculk_shrieker(2, 35, 0);
    assert!(
        app.effects
            .get(crate::effects::MOB_EFFECT_DARKNESS)
            .is_none(),
        "the creative mode the store never arms"
    );
}

/// THE CATALYST THE DEATH THE BLOOM ANSWERS (javap the
/// CatalystListener's gates the death the plain listener answers,
/// the addCursors walk the charges bank, the bloom the pulse arm
/// serves): the app walk — the fresh cell the window never arms,
/// the death the in-range catalyst answers (the exact cursor +
/// bloom lines the reference folds pin — the UP half-block the
/// containing fold floors, the +0.5 center the bloom pos wears),
/// the OUT-OF-RANGE twin the same death never wakes, the wisps the
/// 28-draw walk spawns (the zero velocity the 0.0-speed pair
/// keeps, the 12..44 life band the divisor law speaks), the voice
/// the pitch band wears, the SECOND death the pulsing catalyst
/// answers (NO data-busy window the plain listener keeps — the
/// zero-xp death the cursorless bloom wears, the pc 79 `ifle 112`
/// the bloom's own continuation speaks), the PULSE tail the
/// scheduleTick 8 window's own tick clears (NO voice, NO record
/// the clear walk speaks).
#[test]
fn the_catalyst_the_death_the_bloom_answers() {
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 40.0, 0.0, 0.0, 0.0));
    app.place_sculk_catalyst(0, 32, 0);
    // the OUT-OF-RANGE twin (the 20-block walk the r8 ear refuses)
    app.place_sculk_catalyst(20, 32, 0);
    // the fresh cell the window never arms
    assert!(app.sculk_catalysts[0].pulse_off_at.is_none());
    assert!(app.sculk_catalysts[0].cursors.is_empty());
    app.update();
    let walk_frame = app.frame;
    // THE DEATH (the 5-xp death the caller supplies — the event
    // pos the +0.5 centers keep, the range the 4-distSqr keeps)
    app.entity_die_at(2.5, 32.5, 0.5, 5);
    // THE CURSOR RECORD (the UP half-block the containing fold
    // floors: containing(2.5, 33.0, 0.5) = (2, 33, 0))
    let cursors = app
        .game_events
        .iter()
        .find(|l| l.starts_with("sculk_cursors"))
        .expect("the addCursors walk the record banks");
    assert_eq!(
        *cursors,
        format!("sculk_cursors @{walk_frame} x2 y33 z0 n1 c5"),
        "the one-cursor walk the exact line pins — the n the loop's count, the c the death's xp"
    );
    // THE CURSORS THE CELL BANKS (the spreader list's own growth)
    assert_eq!(app.sculk_catalysts[0].cursors.len(), 1);
    assert_eq!(app.sculk_catalysts[0].cursors[0].pos, (2, 33, 0));
    assert_eq!(app.sculk_catalysts[0].cursors[0].charge, 5);
    // THE BLOOM RECORD (the +0.5 center the atCenterOf fold wears)
    let bloom = app
        .game_events
        .iter()
        .find(|l| l.starts_with("sculk_bloom"))
        .expect("the bloom the record banks");
    assert_eq!(
        *bloom,
        format!("sculk_bloom @{walk_frame} x0.500000 y32.500000 z0.500000"),
        "the bloom the exact line pins"
    );
    // THE PULSE WINDOW (the scheduleTick 8 the walk banks)
    assert_eq!(
        app.sculk_catalysts[0].pulse_off_at,
        Some(walk_frame + 8),
        "the 8-tick window the bloom arms"
    );
    // THE WISPS (the two particles the count speaks — the zero
    // velocity the 0.0-speed pair keeps, the base the +0.5/+1.15
    // folds wear, the life the 12..44 band keeps)
    let wisps: Vec<&crate::particles::Particle> = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::SculkSoul))
        .collect();
    assert_eq!(wisps.len(), 2, "the two wisps the count banks");
    for w in &wisps {
        assert_eq!((w.vx, w.vy, w.vz), (0.0, 0.0, 0.0));
        assert!((12..=44).contains(&w.lifetime));
        assert!((w.y - 33.15f64).abs() <= 0.05, "the y the 0.0 dist keeps");
    }
    // THE OUT-OF-RANGE TWIN (the 20-block walk the ear refuses)
    assert!(app.sculk_catalysts[1].pulse_off_at.is_none());
    assert!(app.sculk_catalysts[1].cursors.is_empty());
    assert_eq!(
        app.game_events
            .iter()
            .filter(|l| l.starts_with("sculk_bloom"))
            .count(),
        1,
        "the twin the bloom never wears"
    );
    // THE VOICE (the pitch draw the band keeps)
    if sounds_loaded() {
        let voice = app
            .sounds
            .events
            .iter()
            .find(|e| e.starts_with(&format!("block.sculk_catalyst.bloom @{walk_frame} ")))
            .expect("the bloom voice the v2.0 walk speaks");
        let pitch: f32 = voice
            .split(" p")
            .nth(1)
            .and_then(|t| t[..6].parse().ok())
            .unwrap_or(0.0);
        assert!(
            (0.6..1.0).contains(&pitch),
            "the 0.6 + r * 0.4 band the voice keeps: {voice}"
        );
    }
    // THE SECOND DEATH (the pulsing catalyst answers — NO
    // data-busy window the plain listener keeps; the ZERO-xp death
    // the cursorless bloom wears)
    app.entity_die_at(2.5, 32.5, 0.5, 0);
    assert!(
        app.game_events
            .iter()
            .all(|l| !l.starts_with("sculk_cursors") || l.ends_with("n1 c5")),
        "the zero-xp death the cursor walk never spends"
    );
    assert_eq!(
        app.sculk_catalysts[0].cursors.len(),
        1,
        "the list the zero-xp walk never grows"
    );
    assert_eq!(
        app.game_events
            .iter()
            .filter(|l| l.starts_with("sculk_bloom"))
            .count(),
        2,
        "the second bloom the continuation banks"
    );
    assert_eq!(
        app.sculk_catalysts[0].pulse_off_at,
        Some(walk_frame + 8),
        "the pulse re-arms the same window"
    );
    // THE PULSE TAIL (the scheduleTick 8 the block's own tick
    // clears — the window's own end the walk serves)
    for _ in 0..8 {
        app.update();
    }
    assert!(
        app.sculk_catalysts[0].pulse_off_at.is_none(),
        "the 8-tick tail the pulse clears"
    );
}

/// THE SPREADER DRAIN THE TICKS WEAR (javap serverTick pc 0-17 —
/// the `updateCursors(level, pos, random, true)` the block
/// entity's own tick serves, the DEFAULT charge law the decay
/// arms): the app walk — the death the cursor banks, the FIRST
/// drain the charge keeps (the decay countdown folds), the SECOND
/// the spread countdown paces, the THIRD the data-0 pop the dead
/// cursor speaks (the voice the fixed 1.0/1.0 pair the pop arm
/// wears, the cube burst the full-block verdict splits, the RNG
/// draws the wire walk's own law banks), the list the drain
/// empties.
#[test]
fn the_spreader_drain_the_ticks_the_pop_wears() {
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 40.0, 0.0, 0.0, 0.0));
    app.place_sculk_catalyst(0, 32, 0);
    app.update();
    app.entity_die_at(2.5, 32.5, 0.5, 5);
    assert_eq!(app.sculk_catalysts[0].cursors.len(), 1);
    // TICK 1 (the drain the fresh cursor keeps — the charge walk
    // the decayDelay 1 reads): the pop the drain never wears
    app.update();
    assert_eq!(
        app.sculk_catalysts[0].cursors.len(),
        1,
        "the first drain the cursor keeps"
    );
    assert_eq!(app.sculk_catalysts[0].cursors[0].decay_delay, 0);
    assert_eq!(app.sculk_catalysts[0].cursors[0].update_delay, 1);
    // TICK 2 (the spread countdown the charge walk paces)
    app.update();
    assert_eq!(app.sculk_catalysts[0].cursors[0].update_delay, 0);
    assert_eq!(app.sculk_catalysts[0].cursors[0].charge, 5);
    // TICK 3 (the elapsed decay the ZERO answer wears — the pop
    // the client walk serves)
    app.update();
    assert!(
        app.sculk_catalysts[0].cursors.is_empty(),
        "the dead cursor the drain drops"
    );
    // THE POP ARM (the data-0 word the pop walk splits — the fixed
    // voice + the cube burst the full-block verdict splits)
    if sounds_loaded() {
        let voice = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("block.sculk.charge @"))
            .count();
        assert_eq!(voice, 1, "the pop arm's own voice the data-0 word wears");
    }
    // THE POP BURST (the SculkChargePop particles the arm spawns —
    // the air cell the lane's own full-cube approximation keeps
    // (the collision_full_block name law the model's own read
    // answers — the same verdict the wire arm's tests bank), the
    // 40-burst the full split wears)
    let pops = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::SculkChargePop))
        .count();
    assert_eq!(pops, 40, "the lane's own verdict the 40-burst wears");
}

/// THE DRAIN'S OWN VEIN SPREAD (javap `ChargeCursor.update` pc
/// 33-130 — the onlyGrowers arm the serverTick's `iconst_1`
/// speaks, the `attemptSpreadVein` default the spreadAll walk
/// serves, the `SCULK_BLOCK_SPREAD` v1.0/p1.0 voice the fixed
/// fconst_1 pair wears): the app walk the LEDGER keeps — the
/// stone substrate the wraps answer (the vein the spread's own
/// product the ledger banks, the grid's u16 vocabulary the face
/// words never carries), the `sculk_spread` record the +0.5
/// center wears, the voice the fixed pair speaks, the countdown
/// tick the arm skips — and the THIRD tick the VEIN LAW the 544
/// round wears (the sculk_vein host the stone substrate the
/// SCULK_REPLACEABLE placement converts, the strip deaths the
/// chain answers, the source row the skip spares).
#[test]
fn the_drain_the_vein_the_spread_wears() {
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    // THE STONE FLOOR CELL (the substrate the wraps crawl around —
    // the death cell's own below the vein's first faces wear)
    world.set(2, 32, 0, crate::world::STONE);
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 40.0, 0.0, 0.0, 0.0));
    app.place_sculk_catalyst(0, 32, 0);
    app.update();
    let walk_frame = app.frame;
    app.entity_die_at(2.5, 32.5, 0.5, 5);
    assert_eq!(app.sculk_catalysts[0].cursors.len(), 1);
    // THE FRESH LEDGER (the death the store never touches)
    assert!(app.sculk_vein_faces.is_empty());
    // TICK 1 (the full update the spread arm serves FIRST — the
    // placements the ledger banks, the voice the record wears)
    app.update();
    // THE LEDGER (the four side wraps the stone's sides + the down
    // self-face the walk's own order banks — the same walk the
    // sculk unit test's world pins)
    assert_eq!(
        app.sculk_vein_faces,
        vec![
            (2, 32, -1, 8),
            (2, 32, 1, 4),
            (1, 32, 0, 32),
            (3, 32, 0, 16),
            (2, 33, 0, 1),
        ],
        "the wraps + the down self-face the ledger banks"
    );
    // THE RECORD (the +0.5 center the atCenterOf fold wears, the
    // count the five placements wears)
    let record = app
        .game_events
        .iter()
        .find(|l| l.starts_with("sculk_spread"))
        .expect("the spread>0 verdict the record banks");
    assert_eq!(
        *record,
        format!(
            "sculk_spread @{} x2.500000 y33.500000 z0.500000 n5",
            walk_frame + 1
        ),
        "the spread record the walk's own frame wears"
    );
    // THE VOICE (javap pc 112-126 — the fixed pitch the fconst_1
    // wears, the gain the attenuate fold's own distance law speaks)
    if sounds_loaded() {
        let voice = app
            .sounds
            .events
            .iter()
            .find(|e| e.starts_with("block.sculk.spread @"))
            .expect("the spread voice the fixed pair wears");
        assert!(
            voice.contains(" p1.000000"),
            "the fixed pitch the spread voice carries"
        );
    }
    // THE COUNTDOWN TICK (the arm the early return skips — no
    // spread the ledger never grows)
    app.update();
    assert_eq!(
        app.sculk_vein_faces.len(),
        5,
        "the countdown tick the spread skips"
    );
    // TICK 3 (THE VEIN LAW the sculk_vein host wears — the 544
    // round's own change: the cursor the tick-1 movement the
    // substrate-side wrap banks (the east-face row (3,32,0,16) the
    // seed's shuffle serves), the carried west face the STONE
    // target wears — the stone the SCULK_REPLACEABLE tag's
    // base_stone_overworld family answers — the placement the
    // store banks, the SPREAD ARM's own wrap the strip death
    // clears, the neighbor-vein chain the sculk-touching rows
    // strips, the SOURCE row the opposite-direction skip spares)
    app.update();
    assert_eq!(
        app.sculk_block_placements,
        vec![(2, 32, 0)],
        "the stone substrate the sculk placement banks"
    );
    assert_eq!(
        app.sculk_vein_faces,
        vec![(3, 32, 0, 16)],
        "the chain the sculk-touching rows strips, the source row the skip spares"
    );
    assert_eq!(
        app.sculk_catalysts[0].cursors[0].charge, 4,
        "the placed walk the charge-1 wears"
    );
    // THE PLACE RECORD (the +0.5 center the atCenterOf fold wears —
    // the TARGET cell the placement banks)
    let place_record = app
        .game_events
        .iter()
        .find(|l| l.starts_with("sculk_place"))
        .expect("the placement the record banks");
    assert_eq!(
        *place_record,
        format!(
            "sculk_place @{} x2.500000 y32.500000 z0.500000",
            walk_frame + 3
        ),
        "the place record the walk's own frame wears"
    );
}

/// THE DRAIN'S OWN VEIN LAW (javap `SculkVeinBlock.attemptUseCharge`
/// pc 0-61 + the private `attemptPlaceSculk` pc 0-250 — the
/// sculk_vein HOST the charge walk's own override wears): the
/// SCULK_REPLACEABLE substrate the sculk block replaces (the
/// `setBlock(target, SCULK, 3)` the flag-3 walk banks — the store
/// the grid's u16 palette the sculk row never carries), the
/// `sculk_place` record the +0.5 center wears, the SAME
/// block.sculk.spread voice the fixed pair speaks, the charge-1
/// the placed walk banks (the halving draw the placed walk never
/// spends), the neighbor-vein chain the strip deaths wear (the
/// sculk-touching faces the AND-NOT clears — the substrate-side
/// wraps the death answers, the source vein the opposite-direction
/// skip spares), the regrow the all-air attach the zero-count
/// silence keeps.
#[test]
fn the_drain_the_vein_law_the_place_wears() {
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    // THE DIRT SUBSTRATE (the SCULK_REPLACEABLE cell the vein law's
    // placement wears — the tag the 52-row flat expansion answers)
    world.set(2, 32, 0, crate::world::DIRT);
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 40.0, 0.0, 0.0, 0.0));
    app.place_sculk_catalyst(0, 32, 0);
    app.update();
    let walk_frame = app.frame;
    app.entity_die_at(2.5, 32.5, 0.5, 5);
    assert_eq!(app.sculk_catalysts[0].cursors.len(), 1);
    // TICK 1 (the host AIR the DEFAULT charge law keeps — the vein
    // law the sculk_vein host alone wears): the spread the five
    // rows banks, the store the placement never touches
    app.update();
    assert_eq!(
        app.sculk_vein_faces,
        vec![
            (2, 32, -1, 8),
            (2, 32, 1, 4),
            (1, 32, 0, 32),
            (3, 32, 0, 16),
            (2, 33, 0, 1),
        ],
        "the wraps + the down self-face the ledger banks"
    );
    assert!(
        app.sculk_block_placements.is_empty(),
        "the air host the placement never wears"
    );
    // TICK 2 (the countdown the spread paces — the stores keep)
    app.update();
    assert_eq!(app.sculk_vein_faces.len(), 5);
    assert!(app.sculk_block_placements.is_empty());
    // TICK 3 (the vein law the sculk_vein host wears — the cursor
    // the tick-1 movement the substrate-side wrap banks, the ONLY
    // carried face the dirt target wears, the placement the store
    // banks): the charge-1 the placed walk keeps, the four
    // sculk-touching wraps the strip death clears, the SOURCE row
    // the opposite-direction skip spares
    app.update();
    assert_eq!(
        app.sculk_block_placements,
        vec![(2, 32, 0)],
        "the dirt substrate the sculk placement banks"
    );
    // THE LEDGER (the host row ALONE the skip spares — the row the
    // moved cursor's own cell keeps, the other four the strip
    // death wears)
    let host = &app.sculk_catalysts[0].cursors[0];
    assert_eq!(
        app.sculk_vein_faces.len(),
        1,
        "the sculk-touching faces the strip death clears"
    );
    assert_eq!(
        (
            app.sculk_vein_faces[0].0,
            app.sculk_vein_faces[0].1,
            app.sculk_vein_faces[0].2
        ),
        host.pos,
        "the spared row the source vein's own cell keeps"
    );
    assert_eq!(host.charge, 4, "the placed walk the charge-1 wears");
    // THE PLACE RECORD (the +0.5 center the atCenterOf fold wears —
    // the TARGET cell the placement banks)
    let record = app
        .game_events
        .iter()
        .find(|l| l.starts_with("sculk_place"))
        .expect("the placement the record banks");
    assert_eq!(
        *record,
        format!(
            "sculk_place @{} x2.500000 y32.500000 z0.500000",
            walk_frame + 3
        ),
        "the place record the walk's own frame wears"
    );
    // THE VOICES (the SAME block.sculk.spread the fixed pair wears —
    // the tick-1 spread + the tick-3 place the TWO deterministic
    // events bank, the tick-3 spread arm's own placement the
    // movement winner's seed may add; the PLACE voice the TARGET
    // cell's own coordinates answer)
    if sounds_loaded() {
        let voices = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("block.sculk.spread @"))
            .count();
        assert!(voices >= 2, "the spread + the place the voices bank");
        let place_voice = app
            .sounds
            .events
            .iter()
            .find(|e| e.starts_with(&format!("block.sculk.spread @{}", walk_frame + 3)))
            .expect("the place voice the place tick wears");
        assert!(
            place_voice.contains(" p1.000000"),
            "the fixed pitch the place voice carries"
        );
    }
}

/// THE DRAIN'S OWN SCULK LAW (javap `SculkBlock.attemptUseCharge`
/// pc 0-191 — the SCULK BLOCK HOST the charge walk's own override
/// wears): the death INSIDE the sculk cell the UP-half cursor the
/// sculk host banks, the pre-seeded placement store the host read
/// answers, the noGrowthRadius STRICT bound the far cursor the
/// GROWTH arm wears (the canPlaceGrowth the above air + the sensor
/// cap keeps), the `nextInt(growthSpawnCost) < charge` draw the
/// placement wears, the `getRandomGrowthState` draw the
/// sensor/shrieker kind banks — the grown cell the store + the
/// ACTIVE registration both bank, the place sound the CURSOR cell
/// speaks, the charge the cost the ALWAYS-subtract wears.
#[test]
fn the_drain_the_sculk_law_the_growth_wears() {
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 40.0, 0.0, 0.0, 0.0));
    app.place_sculk_catalyst(0, 32, 0);
    app.update();
    let walk_frame = app.frame;
    // THE PRE-SEEDED SCULK CELL (the vein law's own product the
    // store banks — the cursor the death inside the cell the sculk
    // host wears: the host read the store's own sculk name answers)
    app.sculk_block_placements.push((6, 32, 0));
    app.entity_die_at(6.5, 32.0, 0.5, 15);
    assert_eq!(app.sculk_catalysts[0].cursors.len(), 1);
    assert_eq!(
        app.sculk_catalysts[0].cursors[0].pos,
        (6, 32, 0),
        "the death inside the sculk cell the cursor banks"
    );
    // THE DRAIN'S OWN GATE (javap attemptUseCharge's head — the
    // `nextInt(CHARGE_DECAY_RATE) != 0` draw the charge walk's own
    // skip keeps): the animate-tick sampler's own six draws per
    // sample the LEVEL stream carries shifted the drain's position
    // in the stream (javap ClientLevel.doAnimateTick's
    // `getfield random` pair — the bodyful app's tick the sampler
    // spends), the first passing tick the walk's own determinism
    // banks — the loop the gate's 1-in-10 rolls (the countdown
    // ticks the zero-draw early return keeps, the full walks the
    // gate alone rolls), the growth the window never misses
    app.update();
    let mut growth_lag = 1usize;
    while app.sculk_growth_cells.is_empty() {
        growth_lag += 1;
        assert!(growth_lag <= 40, "the gate the window passes");
        app.update();
    }
    let growth_frame = walk_frame + growth_lag as u64;
    assert_eq!(
        app.sculk_growth_cells,
        vec![(6, 33, 0, false)],
        "the ABOVE cell the sensor write banks"
    );
    // THE REGISTRATION (the grown cell the ACTIVE sensor the
    // app's own placement walk banks — the hearing lane the cell
    // rides the next tick)
    assert_eq!(
        app.sculk_sensors.iter().map(|s| s.pos).collect::<Vec<_>>(),
        vec![(6, 33, 0)],
        "the grown sensor the registration banks"
    );
    assert!(
        app.sculk_shrikers.is_empty(),
        "the 10-in-11 draw the shrieker never wears"
    );
    // THE RECORD (the GROWN cell's pos + the kind the +0.5 center
    // wears)
    let record = app
        .game_events
        .iter()
        .find(|l| l.starts_with("sculk_growth"))
        .expect("the growth the record banks");
    assert_eq!(
        *record,
        format!(
            "sculk_growth @{} x6.500000 y33.500000 z0.500000 sensor",
            growth_frame
        ),
        "the growth record the walk's own frame wears"
    );
    // THE CURSOR (the charge the cost drops, the facings the sculk
    // cell's EMPTY set wears, the movement the air world the cursor
    // keeps)
    let cursor = &app.sculk_catalysts[0].cursors[0];
    assert_eq!(
        (
            cursor.pos,
            cursor.charge,
            cursor.update_delay,
            cursor.decay_delay,
            cursor.facings
        ),
        ((6, 32, 0), 5, 1, 0, Some(0)),
        "the (pos, 15-10, spread-delay, decay, pack-0) tuple the law keeps"
    );
    // THE PLACE SOUND (javap pc 160-177 — the getPlaceSound key the
    // CURSOR cell speaks, the fixed p1.000000 pair wears)
    if sounds_loaded() {
        let voice = app
            .sounds
            .events
            .iter()
            .find(|e| e.starts_with(&format!("block.sculk_sensor.place @{}", growth_frame)))
            .expect("the growth voice the sensor key wears");
        assert!(
            voice.contains(" p1.000000"),
            "the fixed pitch the growth voice carries"
        );
    }
}

/// THE DRAIN'S OWN SHUFFLE (javap ChargeCursor.getValidMovementPos
/// → getRandomizedNonCornerNeighbourOffsets → Util.shuffledCopy —
/// the Fisher-Yates walk the 18-element table serves, SEVENTEEN
/// nextInt draws every full update spends, the `level.getRandom()`
/// stream the app's own level ambient rides): the BODYLESS app the
/// drain the tick's ONLY spender keeps (the ambient walk the body
/// gate skips, the mood tick the world gate refuses), the replay
/// the bounds order pins — the full update the 17-draw stream
/// advance wears, the countdown and the pop ticks the zero-draw
/// early returns keep, the air world the cursor's pos the movement
/// never completes.
#[test]
fn the_drain_the_shuffle_the_walk_spends() {
    let mut app = App::new(3);
    // THE WORLD SCREEN the tick sweep's own gate keeps (the
    // ScreenKind::World && !paused pair the walk rides), NO body
    // and NO world otherwise — the tick's own spenders the gates
    // skip, the drain the stream's only rider keeps
    app.screen = ScreenKind::World;
    app.place_sculk_catalyst(0, 32, 0);
    app.entity_die_at(2.5, 32.5, 0.5, 5);
    assert_eq!(app.sculk_catalysts[0].cursors.len(), 1);
    // TICK 1 (the full update the fresh cursor keeps): the shuffle
    // the SEVENTEEN draws spends — the replay the bounds walk
    // pins (nextInt(18) down to nextInt(2), the raw draws the
    // same stream the clone answers)
    let before = app.level_ambient_rng.clone();
    app.update();
    let mut replay = before.clone();
    for i in (2..=18u64).rev() {
        replay.next_int(i);
    }
    assert_eq!(
        app.level_ambient_rng, replay,
        "the tick's only spender the 17-draw shuffle keeps"
    );
    assert_eq!(app.sculk_catalysts[0].cursors.len(), 1);
    assert_eq!(
        app.sculk_catalysts[0].cursors[0].pos,
        (2, 33, 0),
        "the air walk the cursor keeps — no vein the gates answer"
    );
    assert_eq!(app.sculk_catalysts[0].cursors[0].decay_delay, 0);
    assert_eq!(app.sculk_catalysts[0].cursors[0].update_delay, 1);
    // TICK 2 (the countdown the spread paces): the walk the early
    // return keeps — the stream the tick NEVER advances
    let mid = app.level_ambient_rng.clone();
    app.update();
    assert_eq!(
        app.level_ambient_rng, mid,
        "the countdown tick the walk never spends"
    );
    assert_eq!(app.sculk_catalysts[0].cursors[0].update_delay, 0);
    // TICK 3 (the elapsed decay the ZERO answer wears): the pop
    // the early return keeps — the stream the SHUFFLE never
    // touches (the pop burst's own draws the wire arm spends, the
    // stream the same walk advances)
    app.update();
    assert!(
        app.sculk_catalysts[0].cursors.is_empty(),
        "the dead cursor the drain drops"
    );
    assert!(
        app.level_ambient_rng != mid,
        "the pop burst's own draws the stream advances"
    );
    // THE 40-BURST the pop arm spawns (the same verdict the 541
    // pin banks)
    let pops = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::SculkChargePop))
        .count();
    assert_eq!(pops, 40, "the pop arm the burst wears");
}

/// THE ELYTRA BREAK'S OWN TAIL (javap onEquippedItemBroken → the
/// entityEvent 47 → the client's breakItem walk): the due flag the
/// drain reads, the chest slot the two lanes' own source clears,
/// the break voice the breakItem law lands, the FIVE item pieces
/// the spawnItemParticles(stack, 5) call speaks.
#[test]
fn the_elytra_break_the_tail_the_lane_wears() {
    let wire = |name: &str| {
        mc_registry::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == name)
            .expect("the item rides the wire") as i32
    };
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.inventory.armor[1] = (wire("elytra"), 1);
    {
        let b = app.body.as_mut().expect("the offline body spawns");
        b.y = 40.0;
        b.on_ground = false;
        // the 432 crossing the hurt walk's own answer — the direct
        // set stands for the isBroken arm the earlier body test
        // pins (the flag survives the tick, the drain the same
        // update reads)
        b.elytra_broke_due = true;
    }
    let pieces_before = app.pieces.len();
    app.update();
    assert_eq!(
        app.inventory.armor[1],
        (0, 0),
        "the slot the break drain clears"
    );
    assert!(
        !app.body.as_ref().expect("the body rides").elytra_broke_due,
        "the drain clears the flag"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.item.break @")),
        "the break voice the drain lands"
    );
    assert_eq!(
        app.pieces.len() - pieces_before,
        5,
        "the 5 item particles the spawn speaks"
    );
}

/// THE CHEST SLOT'S OWN LANE (the regression the last round's
/// mis-indexed chest read owed): the elytra the shift-click
/// quick-move EQUIPS lands the armor row's LOOP row 1 (the ctor
/// walk's SLOT_IDS order HEAD, CHEST, LEGS, FEET — the click/
/// draw/mayPlace walks the same index speaks, the vanilla
/// Inventory.armor list's own FEET-first order the client's rows
/// never ride), and the SAME worn stack the chest read the tick
/// walk keeps — the airborne jump chord the glide STARTS from the
/// real equip path (the hand-placed stands the earlier tests
/// wear, this walk the UI's own answer pins).
#[test]
fn the_quick_moved_elytra_the_glide_wears() {
    let elytra = mc_registry::item_id_by_name("elytra").expect("the elytra rides the registry");
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE EQUIP (store row 9's elytra the shift-click quick-move
    // equips — the armor arm ends the walk, the CHEST loop row 1
    // takes it)
    app.inventory.items[9] = (elytra, 1);
    app.shift_down = true;
    app.on_press(248, 189);
    app.shift_down = false;
    assert_eq!(
        app.inventory.armor[1],
        (elytra, 1),
        "the chest took the quick-moved elytra"
    );
    assert_eq!(app.inventory.items[9], (0, 0), "the source emptied");
    // THE SCREEN'S OWN CLOSE (the E-key route — the inventory
    // screen the body's tick never walks, the World screen the
    // tick gate reopens)
    app.on_inventory_key();
    assert_eq!(app.screen, ScreenKind::World, "the inventory closed");
    // THE GLIDE (the same worn stack the chest read speaks — the
    // airborne double-jump chord the glide starts)
    {
        let b = app.body.as_mut().expect("the offline body spawns");
        b.y = 40.0;
        b.on_ground = false;
    }
    app.on_movement_input(0.0, 0.0, true, false, false);
    app.update();
    assert!(
        app.body.as_ref().expect("the body rides").fall_flying,
        "the really-worn elytra the glide starts"
    );
}

/// THE TORN ELYTRA'S ICON VERDICT (the `minecraft:broken`
/// condition the armor row's draw consults — javap `Broken.get` =
/// `nextDamageWillBreak`): the fresh stack the resting icon wears,
/// the 431 stack the broken icon wears, the non-elytra chest the
/// verdict never moves, the wrong row never consults, the network
/// lane's bare tuples the resting arm keeps.
#[test]
fn the_torn_elytra_icon_the_verdict_wears() {
    let elytra = mc_registry::item_id_by_name("elytra").expect("the elytra rides the registry");
    let iron_chest =
        mc_registry::item_id_by_name("iron_chestplate").expect("the chestplate rides the registry");
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE FRESH STACK (damage 0 — the resting arm)
    app.inventory.armor[1] = (elytra, 1);
    assert!(
        !app.armor_slot_icon_broken(1, elytra),
        "the fresh elytra the resting icon wears"
    );
    // THE TORN STACK (431 — the verdict the on_true branch wears)
    {
        let b = app.body.as_mut().expect("the offline body spawns");
        b.elytra_damage = 431;
    }
    assert!(
        app.armor_slot_icon_broken(1, elytra),
        "the torn elytra the broken icon wears"
    );
    // THE NON-ELYTRA CHEST (the chestplate's icon the verdict
    // never moves — the condition the elytra's json alone roots)
    app.inventory.armor[1] = (iron_chest, 1);
    assert!(
        !app.armor_slot_icon_broken(1, iron_chest),
        "the chestplate the verdict never consults"
    );
    // THE WRONG ROW (the helmet row's k the law never wears)
    app.inventory.armor[1] = (elytra, 1);
    assert!(
        !app.armor_slot_icon_broken(0, elytra),
        "the head row the verdict never wears"
    );
    // THE NETWORK LANE (the live stacks' bare (id, count) tuples —
    // the damage component they never carry, the resting arm)
    app.world_is_network = true;
    assert!(
        !app.armor_slot_icon_broken(1, elytra),
        "the live lane the resting arm keeps"
    );
}

#[test]
fn the_f4_wire_apply_the_packet_alone_speaks() {
    // THE WIRE APPLY'S OWN WALK (javap switchToHoveredGameMode
    // pcs 49-60 — the send ALONE the live sink carries): the
    // networked apply the GameType id speaks, the LOCAL mode the
    // server's belief keeps (no prediction the walk predicts),
    // the same-mode arm the acmp keeps silent, the offline
    // control the local apply wears
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    assert_eq!(
        app.game_mode,
        GameMode::Survival,
        "the net world spawns survival"
    );
    // THE CHANGE APPLY (the hover banked to the CREATIVE icon —
    // the wire id 1 the packet carries)
    app.on_open_game_mode_switcher();
    app.gm_hovered = GameMode::Creative.icon_index();
    app.on_debug_modifier_released();
    assert_eq!(app.screen, ScreenKind::World, "the release closes");
    match rx.try_recv().expect("the wire apply the packet carried") {
        crate::world_view::ContainerCommand::ChangeGameMode { mode_id } => {
            assert_eq!(mode_id, 1, "the GameType CREATIVE id the clinit banks");
        }
        _ => panic!("the game-mode packet's own shape"),
    }
    assert_eq!(
        app.game_mode,
        GameMode::Survival,
        "the local mode the server owns"
    );
    assert!(
        app.previous_game_mode.is_none(),
        "the bank the wire never fills"
    );
    // THE SAME-MODE SILENCE (pcs 19-21's acmp — the hovered
    // current the send never earns)
    app.on_open_game_mode_switcher();
    app.gm_hovered = GameMode::Survival.icon_index();
    app.on_debug_modifier_released();
    assert!(
        rx.try_recv().is_err(),
        "the same-mode apply the wire keeps silent"
    );
    // THE OFFLINE CONTROL (the local apply the offline branch
    // keeps — the previous bank fills, the wire the packet never
    // speaks)
    app.world_is_network = false;
    app.on_open_game_mode_switcher();
    app.gm_hovered = GameMode::Creative.icon_index();
    app.on_debug_modifier_released();
    assert_eq!(
        app.game_mode,
        GameMode::Creative,
        "the offline apply the mode banks"
    );
    assert_eq!(
        app.previous_game_mode,
        Some(GameMode::Survival),
        "the bank the change earns"
    );
    assert!(rx.try_recv().is_err(), "the offline apply no packet speaks");
}

/// THE ABILITIES WIRE'S OWN LANES (the restored loc6 flag the
/// ServerboundPlayerAbilitiesPacket feeds — the audited
/// `LocalPlayer.onUpdateAbilities` walk): the pure seam returns
/// the changed verdict the three arms earn (the double-tap
/// toggle, the spectator sync, the survival gate's silence), the
/// real App's toggle tick the live sink speaks, the stable ticks
/// the wire keeps silent, and the landing retire the grounded
/// body speaks (pcs 1098-1133 — the spectator's own the ground
/// never claims).
#[test]
fn the_flight_loc6_the_abilities_wire_speaks() {
    // THE PURE SEAM (the loc6 flag the walk returns)
    let mut b = crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0);
    b.on_ground = true;
    let mut trigger = 0;
    let mut last = false;
    // the survival body the mayfly gate never arms — no change
    assert!(!App::fly_law_tick(
        &mut b,
        &mut trigger,
        &mut last,
        false,
        false,
        false,
        false,
    ));
    assert!(!b.flying, "the gate refuses the toggle");
    // the ARM (the first press — the window arms, the flight does
    // NOT start, the changed flag the silence keeps)
    assert!(!App::fly_law_tick(
        &mut b,
        &mut trigger,
        &mut last,
        true,
        true,
        false,
        false,
    ));
    assert_eq!(trigger, FLY_WINDOW_TICKS, "the window arms");
    assert!(!b.flying);
    // the release: the edge the lane never re-speaks
    assert!(!App::fly_law_tick(
        &mut b,
        &mut trigger,
        &mut last,
        false,
        true,
        false,
        false,
    ));
    // the TOGGLE (the second press inside the window — the changed
    // verdict the flight's own bank earns)
    assert!(App::fly_law_tick(
        &mut b,
        &mut trigger,
        &mut last,
        true,
        true,
        false,
        false,
    ));
    assert!(b.flying, "the toggle banked");
    assert_eq!(trigger, 0, "the window spent");
    // the STABLE FLYING tick: no change the false keeps
    assert!(!App::fly_law_tick(
        &mut b,
        &mut trigger,
        &mut last,
        false,
        true,
        false,
        false,
    ));
    assert!(b.flying);
    // THE SPECTATOR SYNC (the grounded body the sync banks — the
    // changed verdict the force speaks; the stable sync never
    // re-speaks)
    let mut b2 = crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0);
    let mut trigger2 = 0;
    let mut last2 = false;
    assert!(App::fly_law_tick(
        &mut b2,
        &mut trigger2,
        &mut last2,
        false,
        true,
        true,
        false,
    ));
    assert!(b2.flying, "the sync banks the flight");
    assert!(!App::fly_law_tick(
        &mut b2,
        &mut trigger2,
        &mut last2,
        false,
        true,
        true,
        false,
    ));
    // THE REAL APP'S TOGGLE TICK (the live sink the wire speaks —
    // the net world the body's own walk rides; the body the test
    // banks directly — the offline spawner the net world refuses)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    app.game_mode = GameMode::Creative;
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    for _ in 0..2 {
        app.update(); // the settle — the stable ticks the wire keeps silent
    }
    app.on_movement_input(0.0, 0.0, true, false, false);
    app.update(); // THE ARM (the window banks, no packet)
    app.on_movement_input(0.0, 0.0, false, false, false);
    app.update(); // the release
    app.on_movement_input(0.0, 0.0, true, false, false);
    app.update(); // THE TOGGLE — the abilities wire speaks
    assert!(
        app.body.as_ref().expect("the body rides").flying,
        "the flight the toggle banked"
    );
    let mut abilities_seen = false;
    while let Ok(cmd) = rx.try_recv() {
        if let crate::world_view::ContainerCommand::PlayerAbilities { flying } = cmd {
            assert!(flying, "the toggled-ON state the packet banks");
            abilities_seen = true;
        }
    }
    assert!(abilities_seen, "the changed tick the abilities wire spoke");
    // the STABLE ticks: the flying body the wire keeps silent
    for _ in 0..3 {
        app.on_movement_input(0.0, 0.0, false, false, false);
        app.update();
    }
    while let Ok(cmd) = rx.try_recv() {
        assert!(
            !matches!(
                cmd,
                crate::world_view::ContainerCommand::PlayerAbilities { .. }
            ),
            "the stable tick no abilities packet speaks"
        );
    }
}

/// THE LANDING RETIRE'S OWN WIRE (javap LocalPlayer.aiStep
/// pcs 1098-1133 the tail the update walk wears): the grounded
/// flying body the retire flattens AND the abilities wire speaks
/// (the flying-false byte the changed tick banks), the
/// spectator's own the ground never claims (no retire — the
/// sync's own ON packet the only word the lane keeps), the
/// offline ground the real terrain speaks.
#[test]
fn the_landing_retire_the_abilities_wire_speaks() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    for _ in 0..3 {
        app.update(); // the settle — the resting body the ground keeps
    }
    // THE GROUNDED FLIGHT (the pre-drained bank — the body the
    // retire lane reads): flying + on_ground the survival walk
    // keeps
    if let Some(b) = app.body.as_mut() {
        b.flying = true;
    }
    app.update();
    assert!(
        !app.body.as_ref().expect("the body rides").flying,
        "the retire flattens the flight"
    );
    let mut abilities_seen = false;
    while let Ok(cmd) = rx.try_recv() {
        if let crate::world_view::ContainerCommand::PlayerAbilities { flying } = cmd {
            assert!(!flying, "the retired state the packet banks");
            abilities_seen = true;
        }
    }
    assert!(abilities_seen, "the retire the abilities wire spoke");
    // THE SPECTATOR CONTROL (the gate's own refusal — the flight
    // the terrain never claims; the SYNC's own ON packet the
    // retire's silence accompanies — the flying-false byte the
    // lane never speaks)
    app.game_mode = GameMode::Spectator;
    app.update();
    assert!(
        app.body.as_ref().expect("the body rides").flying,
        "the spectator flight the sync banks"
    );
    while let Ok(cmd) = rx.try_recv() {
        if let crate::world_view::ContainerCommand::PlayerAbilities { flying } = cmd {
            assert!(flying, "the sync's own ON word the only packet speaks");
        }
    }
}

/// THE GAME-MODE ECHO'S OWN BANK (javap 26.2
/// `MultiPlayerGameMode.setLocalMode(GameType)` the walk the
/// installer wears): the changed word the previous bank earns,
/// the mode bank the F4 switcher's reopen hover feeds, the
/// abilities re-grant the three arms speak (the survival revoke,
/// the spectator's instant flight, the creative's untouched
/// flight), the invulnerable word the instant bank keeps, the
/// unknown id the ZERO fallback answers, the unchanged word the
/// idempotent acmp keeps silent, and the offline world the lane
/// never reaches.
#[test]
fn the_game_mode_echo_the_set_local_mode_speaks() {
    // THE OFFLINE GATE (the lane the wire world alone reaches)
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Survival;
    app.live_game_mode_updated(Some(1));
    assert_eq!(
        app.game_mode,
        GameMode::Survival,
        "the offline world the echo never banks"
    );
    // THE WIRE BANK (the net world the installer serves)
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    app.game_mode = GameMode::Survival;
    if let Some(b) = app.body.as_mut() {
        b.flying = false;
    }
    // THE CREATIVE ECHO (the id 1 the from_wire_id fold answers):
    // the previous bank fills, the mode banks, the flying field
    // UNTOUCHED (the vanilla CREATIVE arm's own silence), the
    // invulnerable word the instant bank keeps
    if let Some(b) = app.body.as_mut() {
        b.flying = true; // the mid-flight switch the flight preserves
    }
    app.live_game_mode_updated(Some(1));
    assert_eq!(app.game_mode, GameMode::Creative, "the mode the echo banks");
    assert_eq!(
        app.previous_game_mode,
        Some(GameMode::Survival),
        "the previous bank the changed word earns"
    );
    assert!(
        app.body.as_ref().expect("the body rides").flying,
        "the creative arm the flight preserves"
    );
    assert!(
        app.body.as_ref().expect("the body rides").invulnerable,
        "the invulnerable word the creative grant banks"
    );
    // THE SPECTATOR ECHO (the id 3): the instant flight the grant
    // banks (the updatePlayerAbilities walk's own `flying = 1`)
    if let Some(b) = app.body.as_mut() {
        b.flying = false;
    }
    app.live_game_mode_updated(Some(3));
    assert_eq!(app.game_mode, GameMode::Spectator, "the spectator banks");
    assert!(
        app.body.as_ref().expect("the body rides").flying,
        "the spectator arm the flight banks instantly"
    );
    // THE SURVIVAL ECHO (the id 0): the zero walk the flight folds,
    // the invulnerable grant retires
    app.live_game_mode_updated(Some(0));
    assert_eq!(app.game_mode, GameMode::Survival, "the survival banks");
    assert!(
        !app.body.as_ref().expect("the body rides").flying,
        "the zero walk the flight folds"
    );
    assert!(
        !app.body.as_ref().expect("the body rides").invulnerable,
        "the survival arm the grant retires"
    );
    // THE UNCHANGED ECHO (the idempotent re-ship): the acmp the
    // previous bank keeps silent (the server's own
    // changeGameModeForPlayer never re-sends it — the walk the
    // vanilla keeps anyway)
    app.live_game_mode_updated(Some(0));
    assert_eq!(
        app.previous_game_mode,
        Some(GameMode::Spectator),
        "the unchanged word the bank never re-fills"
    );
    assert_eq!(app.game_mode, GameMode::Survival, "the mode the acmp keeps");
    // THE UNKNOWN ID (the ZERO fallback — the
    // ByIdMap.continuous values()[0] the byId walk keeps): the
    // SURVIVAL default answers
    app.game_mode = GameMode::Creative;
    app.live_game_mode_updated(Some(200));
    assert_eq!(
        app.game_mode,
        GameMode::Survival,
        "the unknown id the fallback answers"
    );
    // THE ADVENTURE ECHO (the id 2): the restricted grant the
    // zero walk answers (the survival family's own arms)
    app.live_game_mode_updated(Some(2));
    assert_eq!(app.game_mode, GameMode::Adventure, "the adventure banks");
    assert!(
        !app.body.as_ref().expect("the body rides").invulnerable,
        "the adventure arm the grant never banks"
    );
}

/// THE ABILITIES ECHO'S OWN BANK (javap 26.2
/// `ClientPacketListener.handlePlayerAbilities` — the audited
/// six-putfield walk the installer wears): the offline gate the
/// local belief keeps, the absent word the join snapshot's
/// silence keeps, the FLYING putfield the core re-bank speaks
/// (both directions — the server's word the prediction confirms
/// and the word the prediction revokes), the invulnerable and
/// flying-speed putfields the Body-owned trio completes (the
/// wheel's adjusted speed the server's word re-pins), and the
/// idempotent re-bank the unchanged word's same-value putfields
/// keep.
#[test]
fn the_abilities_echo_the_flying_state_re_banks() {
    // THE OFFLINE GATE (the lane the wire world alone reaches —
    // the offline world the local belief owns, no server speaks)
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    let word = mc_protocol::play::PlayerAbilitiesWord {
        invulnerable: true,
        flying: true,
        mayfly: true,
        instabuild: false,
        flying_speed: 0.07,
        walking_speed: 0.1,
    };
    app.live_abilities_updated(Some(word.clone()));
    let b = app.body.as_ref().expect("the body rides");
    assert!(!b.flying, "the offline world the echo never banks");
    assert!(!b.invulnerable, "the offline gate the trio keeps silent");
    // THE WIRE BANK (the net world the installer serves)
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    // THE ABSENT WORD (the join snapshot's own silence — the None
    // the body's belief never touches)
    app.live_abilities_updated(None);
    let b = app.body.as_ref().expect("the body rides");
    assert!(!b.flying, "the absent word the body keeps");
    // THE FLYING PUTFIELD (the core re-bank — the server's word
    // the prediction confirms: the grounded body the flight
    // banks, the invulnerable word and the speed the trio
    // carries)
    app.live_abilities_updated(Some(word.clone()));
    let b = app.body.as_ref().expect("the body rides");
    assert!(b.flying, "the server's word the flight banks");
    assert!(b.invulnerable, "the invulnerable putfield banks");
    assert!(
        (b.flying_speed - 0.07).abs() < 1e-9,
        "the flying-speed putfield the word re-pins"
    );
    // THE REVERSE PUTFIELD (the server's word the prediction
    // revokes — the grounded word the flight folds, the
    // invulnerable grant the word retires)
    let ground = mc_protocol::play::PlayerAbilitiesWord {
        invulnerable: false,
        flying: false,
        mayfly: false,
        instabuild: false,
        flying_speed: 0.05,
        walking_speed: 0.1,
    };
    app.live_abilities_updated(Some(ground.clone()));
    let b = app.body.as_ref().expect("the body rides");
    assert!(!b.flying, "the grounded word the flight folds");
    assert!(!b.invulnerable, "the retired grant the word wears");
    assert!(
        (b.flying_speed - 0.05).abs() < 1e-9,
        "the default speed the word restores"
    );
    // THE IDEMPOTENT RE-BANK (the unchanged word's same-value
    // putfields — the handler's own unconditional walk the
    // re-ship keeps silent in state)
    app.live_abilities_updated(Some(ground));
    let b = app.body.as_ref().expect("the body rides");
    assert!(!b.flying && !b.invulnerable, "the re-bank the state keeps");
}

/// THE ABILITIES BANK'S OWN CONSUMERS (the vanilla ONE
/// `Abilities` object the walk family reads — the mayfly gate
/// `LocalPlayer.aiStep`'s own read, the instabuild lanes the dig
/// family's own): the ABSENT bank the mode folds stand in (the
/// offline world's authority, the pre-echo window's wait), the
/// BANKED word the folds replace both directions — a survival
/// player the server's grant arms, a creative player the
/// revoked word refuses (the server's own belief the client's
/// consumers truly mirror).
#[test]
fn the_abilities_bank_the_consumers_read() {
    // THE ABSENT BANK (the offline world the folds answer — the
    // fresh app's survival default the zero walk mirrors)
    let mut app = App::new(3);
    app.enter_world();
    assert!(app.abilities_word.is_none(), "the fresh bank sleeps");
    assert!(
        !app.abilities_mayfly(),
        "the survival fold the mayfly refuses"
    );
    assert!(
        !app.abilities_instabuild(),
        "the survival fold the instabuild refuses"
    );
    // the creative mode the folds grant (the offline world the
    // client's own authority speaks)
    app.game_mode = GameMode::Creative;
    assert!(
        app.abilities_mayfly(),
        "the creative fold the mayfly grants"
    );
    assert!(
        app.abilities_instabuild(),
        "the creative fold the instabuild grants"
    );
    // THE WIRE WORLD'S PRE-ECHO WINDOW (the join snapshot's own
    // silence — the folds the wait keeps)
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    app.game_mode = GameMode::Survival;
    assert!(app.abilities_word.is_none(), "the pre-echo bank sleeps");
    assert!(
        !app.abilities_mayfly() && !app.abilities_instabuild(),
        "the folds the pre-echo window answers"
    );
    // THE BANKED WORD (the server's own grant — a SURVIVAL player
    // the mayfly/instabuild word arms: the vanilla server's
    // plugin-driven grant the honest wire belief banks)
    let granted = mc_protocol::play::PlayerAbilitiesWord {
        invulnerable: false,
        flying: false,
        mayfly: true,
        instabuild: true,
        flying_speed: 0.05,
        walking_speed: 0.1,
    };
    app.live_abilities_updated(Some(granted));
    assert!(
        app.abilities_mayfly(),
        "the survival player the server's mayfly word arms"
    );
    assert!(
        app.abilities_instabuild(),
        "the survival player the server's instabuild word arms"
    );
    // THE REVOKED WORD (the CREATIVE player the server's own word
    // refuses — the local fold never overrides the wire belief)
    app.game_mode = GameMode::Creative;
    let revoked = mc_protocol::play::PlayerAbilitiesWord {
        invulnerable: false,
        flying: false,
        mayfly: false,
        instabuild: false,
        flying_speed: 0.05,
        walking_speed: 0.1,
    };
    app.live_abilities_updated(Some(revoked));
    assert!(
        !app.abilities_mayfly(),
        "the creative player the revoked word refuses"
    );
    assert!(
        !app.abilities_instabuild(),
        "the creative player the revoked word refuses"
    );
}

/// THE FLY GATE'S OWN BANKED WORD (the vanilla aiStep's
/// `abilities.mayfly` read the call site wears): the wire
/// world's SURVIVAL mode the server's mayfly word arms — the
/// real double-tap the flight toggles and the abilities wire
/// speaks (the changed tick's own packet the loc6 flag earns),
/// the PRE-ECHO window the same survival double-tap refuses (the
/// fold the absent bank keeps — the two arms the one gate
/// speaks).
#[test]
fn the_fly_gate_the_banked_word_arms() {
    // THE PRE-ECHO REFUSAL (the survival mode the fold answers —
    // no bank, no flight)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    assert!(app.game_mode == GameMode::Survival, "the survival default");
    app.on_movement_input(0.0, 0.0, true, false, false);
    app.update(); // THE ARM (the window would bank — the gate refuses first)
    app.on_movement_input(0.0, 0.0, false, false, false);
    app.update(); // the release
    app.on_movement_input(0.0, 0.0, true, false, false);
    app.update(); // THE WOULD-BE TOGGLE (the gate the fold refuses)
    assert!(
        !app.body.as_ref().expect("the body rides").flying,
        "the pre-echo survival the fold refuses"
    );
    // THE BANKED WORD (the server's mayfly grant — the SAME
    // survival double-tap the flight toggles, the wire the
    // changed tick speaks)
    let word = mc_protocol::play::PlayerAbilitiesWord {
        invulnerable: false,
        flying: false,
        mayfly: true,
        instabuild: false,
        flying_speed: 0.05,
        walking_speed: 0.1,
    };
    app.live_abilities_updated(Some(word));
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_movement_input(0.0, 0.0, false, false, false);
    app.update(); // the settle (the stable tick the wire keeps silent)
    app.on_movement_input(0.0, 0.0, true, false, false);
    app.update(); // THE ARM
    app.on_movement_input(0.0, 0.0, false, false, false);
    app.update(); // the release
    app.on_movement_input(0.0, 0.0, true, false, false);
    app.update(); // THE TOGGLE — the banked mayfly the flight arms
    assert!(
        app.body.as_ref().expect("the body rides").flying,
        "the banked word the double-tap arms"
    );
    let mut abilities_seen = false;
    while let Ok(cmd) = rx.try_recv() {
        if let crate::world_view::ContainerCommand::PlayerAbilities { flying } = cmd {
            assert!(flying, "the toggled-ON state the packet banks");
            abilities_seen = true;
        }
    }
    assert!(abilities_seen, "the changed tick the abilities wire spoke");
}

/// THE DIG LANE'S OWN BANKED WORD (the vanilla
/// `MultiPlayerGameMode.destroyBlock`'s `abilities.instabuild`
/// read the press wears): the wire world's SURVIVAL mode the
/// server's instabuild word the CREATIVE PRESS's own signatures
/// earns (the crack lane never arms, the destroyDelay the
/// creative cadence banks, the START send the wire speaks), the
/// PRE-ECHO window the same press the crack lane arms (the fold
/// the absent bank keeps), and the CREATIVE player the revoked
/// word refuses (the server's own belief the instant carve owns
/// — the local edit the wire world the server's own refresh
/// banks, `apply_locally` the false keeps).
#[test]
fn the_dig_lane_the_banked_instabuild_carves() {
    // THE WIRE WORLD (the air scene one solid cell the test sets —
    // the body's own eye the aim owns)
    let mut app = App::new(3);
    let mut world = air_net_world(2, 0);
    world.set(0, 32, 0, crate::world::STONE);
    app.net_world = Some(world);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(
        0.5, 34.5, 0.5, 0.0,
        90.0, // THE DOWN LOOK (the vanilla degrees — positive the pitch down speaks)
    ));
    assert!(
        app.game_mode == GameMode::Survival,
        "the survival default the fold keeps"
    );
    assert!(
        !app.dig.apply_locally,
        "the wire world the server's own authority keeps"
    );
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    // THE PRE-ECHO PRESS (the fold the absent bank keeps — the
    // crack lane arms, the START send the wire speaks)
    app.on_press(0, 0);
    assert!(
        app.dig.is_destroying,
        "the survival press the crack lane arms"
    );
    let mut starts = 0;
    while let Ok(cmd) = rx.try_recv() {
        if let crate::world_view::ContainerCommand::DigAction { action, .. } = cmd {
            assert_eq!(action, crate::dig::ACTION_START, "the start speaks");
            starts += 1;
        }
    }
    assert_eq!(starts, 1, "the survival press the one START speaks");
    // THE BANKED WORD (the server's instabuild grant — the SAME
    // survival press the CREATIVE signatures wears: the crack
    // lane never arms, the destroyDelay the creative cadence
    // banks, the START send the wire speaks)
    let word = mc_protocol::play::PlayerAbilitiesWord {
        invulnerable: false,
        flying: false,
        mayfly: false,
        instabuild: true,
        flying_speed: 0.05,
        walking_speed: 0.1,
    };
    app.live_abilities_updated(Some(word));
    app.dig = crate::dig::DigState::default();
    app.dig.apply_locally = false;
    app.on_press(0, 0);
    assert!(
        !app.dig.is_destroying,
        "the instabuild press the crack lane never arms"
    );
    assert_eq!(app.dig.delay, 5, "the audited destroyDelay arm");
    let mut starts = 0;
    while let Ok(cmd) = rx.try_recv() {
        if let crate::world_view::ContainerCommand::DigAction { action, .. } = cmd {
            assert_eq!(action, crate::dig::ACTION_START, "the start speaks");
            starts += 1;
        }
    }
    assert_eq!(starts, 1, "the instabuild press the one START speaks");
    // THE REVOKED WORD (the CREATIVE mode the server's own word
    // refuses — the press the crack lane arms again)
    app.game_mode = GameMode::Creative;
    let revoked = mc_protocol::play::PlayerAbilitiesWord {
        invulnerable: false,
        flying: false,
        mayfly: false,
        instabuild: false,
        flying_speed: 0.05,
        walking_speed: 0.1,
    };
    app.live_abilities_updated(Some(revoked));
    app.dig = crate::dig::DigState::default();
    app.dig.apply_locally = false;
    app.on_press(0, 0);
    assert!(
        app.dig.is_destroying,
        "the creative press the revoked word refuses"
    );
}

/// THE MODE ECHO'S OWN RE-GRANT RE-BANK (the vanilla ONE
/// `Abilities` object the `GameType.updatePlayerAbilities` walk
/// writes — the setLocalMode's unconditional pcs 21-35 the
/// installer wears): the banked word the game-mode echo re-pins
/// (the CREATIVE arm the mayfly/instabuild/invulnerable trio the
/// FLYING field untouched keeps, the SPECTATOR arm the instant
/// flight banks, the SURVIVAL arm the zero walk the four fields
/// folds), the server's own echo the whole word re-pins after
/// (the newest echo the replace walk speaks), and the absent
/// bank the silence keeps (the pre-echo window the re-grant
/// never banks).
#[test]
fn the_mode_echo_the_abilities_word_re_banks() {
    // THE BANKED SURVIVAL WORD (the server's own echo — the
    // grounded survival shape the word wears)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let ground = mc_protocol::play::PlayerAbilitiesWord {
        invulnerable: false,
        flying: false,
        mayfly: false,
        instabuild: false,
        flying_speed: 0.05,
        walking_speed: 0.1,
    };
    app.live_abilities_updated(Some(ground.clone()));
    // THE ABSENT BANK (the fresh wire app the mode echo never
    // banks — the pre-echo window the re-grant keeps silent)
    let mut fresh = App::new(3);
    fresh.net_world = Some(air_net_world(2, 0));
    fresh.enter_network_world(None, Skybox::Overworld, None, None);
    fresh.live_game_mode_updated(Some(1));
    assert!(
        fresh.abilities_word.is_none(),
        "the absent bank the re-grant keeps silent"
    );
    // THE CREATIVE ECHO (the id 1 — the trio the FLYING field
    // untouched keeps, the consumers the re-granted bits read)
    app.live_game_mode_updated(Some(1));
    let w = app.abilities_word.as_ref().expect("the bank re-granted");
    assert!(w.mayfly && w.instabuild && w.invulnerable, "the trio banks");
    assert!(!w.flying, "the creative arm the flying field untouched");
    assert!(
        app.abilities_mayfly() && app.abilities_instabuild(),
        "the consumers the re-granted bits read"
    );
    // THE SPECTATOR ECHO (the id 3 — the instant flight the word
    // banks, the instabuild the acmp lane refuses)
    app.live_game_mode_updated(Some(3));
    let w = app.abilities_word.as_ref().expect("the bank re-granted");
    assert!(w.flying && w.mayfly, "the flight the word banks");
    assert!(!w.instabuild, "the spectator the instabuild refuses");
    assert!(w.invulnerable, "the spectator the invulnerable wears");
    // THE SURVIVAL ECHO (the id 0 — the zero walk the four fields
    // folds, the consumers the revoked bits read)
    app.live_game_mode_updated(Some(0));
    let w = app.abilities_word.as_ref().expect("the bank re-granted");
    assert!(
        !w.flying && !w.mayfly && !w.instabuild && !w.invulnerable,
        "the zero walk banks"
    );
    assert!(
        !app.abilities_mayfly() && !app.abilities_instabuild(),
        "the consumers the revoked bits read"
    );
    // THE SERVER'S OWN ECHO (the whole word the replace walk
    // re-pins verbatim — the re-grant the server's word confirms
    // or replaces)
    app.live_abilities_updated(Some(ground.clone()));
    let w = app.abilities_word.as_ref().expect("the echo re-banks");
    assert_eq!(*w, ground, "the newest echo the word re-pins");
}

/// THE OFFLINE APPLY'S OWN RE-GRANT (the vanilla server-side
/// `GameType.updatePlayerAbilities` walk the local authority
/// owns — the singleplayer's integrated server the same walk
/// runs, the SEAM the mode echo's wire lane shares): the F4
/// apply the FULL walk speaks — the CREATIVE arm the
/// invulnerable word the hurt pipeline's head banks, the
/// SPECTATOR arm the instant flight banks (the apply's own
/// walk — no tick's sync the flight waits for), the CREATIVE
/// arm the flying field UNTOUCHED keeps (the mid-flight
/// creative switch the flight preserves), the SURVIVAL/ADVENTURE
/// arm the zero walk the fields folds, and the offline bank the
/// None keeps (the mode folds the consumers answer).
#[test]
fn the_offline_apply_the_re_grant_walk_completes() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    for _ in 0..3 {
        app.update(); // the settle — the grounded body the collide banks
    }
    assert!(
        !app.body.as_ref().expect("the body rides").invulnerable,
        "the survival body the invulnerable keeps silent"
    );
    // THE CREATIVE APPLY (the trio's invulnerable word the hurt
    // pipeline's head banks — the flying field the grounded body
    // untouched keeps)
    app.on_open_game_mode_switcher();
    while GameMode::ICON_ORDER[app.gm_hovered] != GameMode::Creative {
        app.on_game_mode_switcher_cycle();
    }
    app.on_debug_modifier_released();
    assert_eq!(app.game_mode, GameMode::Creative, "the apply lands");
    let b = app.body.as_ref().expect("the body rides");
    assert!(b.invulnerable, "the creative apply the grant banks");
    assert!(!b.flying, "the creative arm the grounded body untouched");
    assert!(
        app.abilities_word.is_none(),
        "the offline bank the None keeps"
    );
    assert!(
        app.abilities_mayfly() && app.abilities_instabuild(),
        "the folds the consumers answer"
    );
    // THE SPECTATOR APPLY (the instant flight the apply's own
    // walk banks — the sync arm the next tick the re-bank never
    // double-speaks)
    app.on_open_game_mode_switcher();
    while GameMode::ICON_ORDER[app.gm_hovered] != GameMode::Spectator {
        app.on_game_mode_switcher_cycle();
    }
    app.on_debug_modifier_released();
    assert_eq!(app.game_mode, GameMode::Spectator, "the apply lands");
    let b = app.body.as_ref().expect("the body rides");
    assert!(b.flying, "the spectator apply the flight banks instantly");
    assert!(b.invulnerable, "the spectator apply the invulnerable keeps");
    // THE MID-FLIGHT CREATIVE SWITCH (the CREATIVE arm the
    // flying field UNTOUCHED keeps — the flight the mode switch
    // preserves)
    app.on_open_game_mode_switcher();
    while GameMode::ICON_ORDER[app.gm_hovered] != GameMode::Creative {
        app.on_game_mode_switcher_cycle();
    }
    app.on_debug_modifier_released();
    assert_eq!(app.game_mode, GameMode::Creative, "the apply lands");
    assert!(
        app.body.as_ref().expect("the body rides").flying,
        "the creative arm the flight preserves"
    );
    // THE SURVIVAL APPLY (the zero walk the fields folds — the
    // ground the revoke asks, the invulnerable grant retires)
    app.on_open_game_mode_switcher();
    while GameMode::ICON_ORDER[app.gm_hovered] != GameMode::Survival {
        app.on_game_mode_switcher_cycle();
    }
    app.on_debug_modifier_released();
    assert_eq!(app.game_mode, GameMode::Survival, "the apply lands");
    let b = app.body.as_ref().expect("the body rides");
    assert!(!b.flying, "the zero walk the flight folds");
    assert!(!b.invulnerable, "the zero walk the grant retires");
    assert_eq!(
        app.previous_game_mode,
        Some(GameMode::Creative),
        "the previous bank the flick earns"
    );
    assert!(
        !app.abilities_mayfly() && !app.abilities_instabuild(),
        "the folds the revoked bits answer"
    );
}

/// THE ENTITY HITBOXES' OWN GIZMOS (javap 26.2 the
/// `EntityHitboxDebugRenderer.showHitboxes` walk the F3+B entry
/// serves): the white stroke cuboid the interpolation delta
/// moves onto the rendered pose, the white point the lerped
/// position wears, the LIVING arms the red eye strip + the blue
/// look arrow speak (the shaft + the four head lines the
/// shortest-arc basis builds), the NON-living body the
/// plain-entity lanes keep (the box + the point alone), and the
/// exact antiparallel the degenerate basis lane wears (no panic,
/// the shaft the −X speaks).
#[test]
fn the_hitbox_gizmos_the_audited_walk_speaks() {
    // THE LIVING BODY (the zombie's own dims, the stable tick the
    // zero delta wears, the yaw 0 the +Z view speaks)
    let shape = HitboxEntityShape {
        x: 10.0,
        y: 64.0,
        z: 12.0,
        rendered_x: 10.0,
        rendered_y: 64.0,
        rendered_z: 12.0,
        width: 0.6,
        height: 1.95,
        eye_height: 1.74,
        living: true,
        view: [0.0, 0.0, 1.0],
    };
    let (lines, points) = hitbox_gizmos(&shape);
    assert_eq!(lines.len(), 29, "the box 12 + the strip 12 + the arrow 5");
    assert_eq!(points.len(), 1, "the one point the lerped pos wears");
    // THE WHITE BOX (the 12 edges the bb's own corners speak — the
    // bottom edge the (0,1) pair wears)
    let edge = lines
        .iter()
        .find(|l| {
            (l.a.x - 9.7).abs() < 1e-4
                && (l.a.y - 64.0).abs() < 1e-4
                && (l.a.z - 11.7).abs() < 1e-4
                && (l.b.x - 10.3).abs() < 1e-4
                && (l.b.z - 11.7).abs() < 1e-4
        })
        .expect("the bottom edge the white box wears");
    assert!(
        edge.rgb[0] > 0.99 && edge.rgb[1] > 0.99 && edge.rgb[2] > 0.99,
        "the white stroke the box wears"
    );
    assert!(!edge.always_on_top, "the depth-tested lane the style keeps");
    // THE RED EYE STRIP (the eye level ±0.01 the flat box wears)
    let strip = lines
        .iter()
        .find(|l| (l.a.y - (64.0 + 1.74 - 0.01)).abs() < 1e-4)
        .expect("the strip's bottom the eye level wears");
    assert!(
        strip.rgb[0] > 0.99 && strip.rgb[1] < 0.01 && strip.rgb[2] < 0.01,
        "the red stroke the strip wears"
    );
    // THE BLUE ARROW (the shaft the +Z view speaks — the eye's own
    // level the 2.0 reach rides)
    let shaft = lines
        .iter()
        .find(|l| {
            (l.a.x - 10.0).abs() < 1e-4
                && (l.a.y - 65.74).abs() < 1e-4
                && (l.a.z - 12.0).abs() < 1e-4
                && (l.b.x - 10.0).abs() < 1e-4
                && (l.b.z - 14.0).abs() < 1e-4
        })
        .expect("the shaft the +Z view wears");
    assert!(
        shaft.rgb[2] > 0.99 && shaft.rgb[0] < 0.01 && shaft.rgb[1] < 0.01,
        "the blue stroke the arrow wears"
    );
    // THE HEAD LINES (the four the shortest-arc basis builds — the
    // yaw-only basis the sideways head wears)
    assert!(
        lines.iter().any(|l| {
            (l.b.x - 9.8).abs() < 1e-4
                && (l.b.y - 65.74).abs() < 1e-4
                && (l.b.z - 13.8).abs() < 1e-4
        }),
        "the sideways head the basis speaks"
    );
    // THE POINT (the white dot the rendered pos wears)
    assert!(
        (points[0].pos.x - 10.0).abs() < 1e-4
            && (points[0].pos.y - 64.0).abs() < 1e-4
            && (points[0].pos.z - 12.0).abs() < 1e-4,
        "the lerped position the point wears"
    );
    // THE NON-LIVING BODY (the box + the point alone — the strip
    // and the arrow the instanceof gate never reaches)
    let plain = HitboxEntityShape {
        x: 10.0,
        y: 64.0,
        z: 12.0,
        rendered_x: 10.0,
        rendered_y: 64.0,
        rendered_z: 12.0,
        width: 1.375,
        height: 0.562,
        eye_height: 0.0,
        living: false,
        view: [0.0, 0.0, 0.0],
    };
    let (plain_lines, plain_points) = hitbox_gizmos(&plain);
    assert_eq!(plain_lines.len(), 12, "the box alone the hull wears");
    assert_eq!(plain_points.len(), 1, "the point the hull keeps");
    // THE DELTA (the rendered pose the box moves onto — the
    // interpolation the +0.5 walk speaks)
    let moved = HitboxEntityShape {
        rendered_x: 10.5,
        ..shape
    };
    let (moved_lines, _) = hitbox_gizmos(&moved);
    assert!(
        moved_lines
            .iter()
            .any(|l| { (l.a.x - (10.0 - 0.3 + 0.5)).abs() < 1e-4 && (l.a.y - 64.0).abs() < 1e-4 }),
        "the interpolation delta the box moves"
    );
    // THE ANTIPARALLEL (the exact −X dir the degenerate basis lane
    // wears — no panic, the shaft the −X speaks)
    let anti = HitboxEntityShape {
        view: [-1.0, 0.0, 0.0],
        ..shape
    };
    let (anti_lines, _) = hitbox_gizmos(&anti);
    assert!(
        anti_lines.iter().any(|l| {
            (l.b.x - 8.0).abs() < 1e-4
                && (l.b.y - 65.74).abs() < 1e-4
                && (l.b.z - 12.0).abs() < 1e-4
        }),
        "the −X shaft the degenerate lane speaks"
    );
}

/// THE F3+B CHORD'S OWN WALK (the real App's entity set the
/// `entitiesForRendering` lane serves): the entry flip the toggle
/// speaks, the MOB the full living walk wears (the box + the
/// strip + the arrow + the point), the DROP the plain-entity
/// lanes keep (the box + the point), and the CAMERA ENTITY the
/// first-person skip keeps silent (the body's own box the walk
/// never draws).
#[test]
fn the_f3_b_chord_the_entity_set_speaks() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE TOGGLE (the entry flip the F3+B lane speaks — the
    // modifier-spent flag the bare F3 release gate reads)
    assert!(!app.hitboxes, "the fresh client the entry keeps silent");
    app.on_hitboxes_key();
    assert!(app.hitboxes, "the chord the entry enables");
    assert!(app.debug_modifier_used, "the modifier the chord spends");
    // the body's own pose (the camera entity the first-person
    // verdict skips)
    let (bx, by, bz) = {
        let b = app.body.as_ref().expect("the body rides");
        (b.x, b.y, b.z)
    };
    // THE MOB (the zombie 2 blocks east the walk serves — the
    // stable tick the zero delta wears)
    app.mobs
        .push(crate::mob::Zombie::new(7, bx + 2.0, by, bz, 3));
    // THE DROP (the item entity the plain lanes keep — the spawn
    // point the None prev wears)
    let mut rng = crate::drops::Lcg::new(11);
    app.drops.push(crate::drops::ItemDrop::spawn_entity_loot(
        9,
        62,
        1,
        bx + 4.0,
        by,
        bz,
        &mut rng,
    ));
    let (lines, points) = app.entity_hitbox_gizmos();
    assert_eq!(points.len(), 2, "the mob + the drop the points wear");
    assert_eq!(
        lines.len(),
        29 + 12,
        "the living walk + the plain walk the lines wear"
    );
    // THE MOB'S OWN BOX (the white edges the mob's bb corners
    // speak)
    let mob_x = (bx + 2.0) as f32;
    assert!(
        lines
            .iter()
            .any(|l| { (l.a.x - (mob_x - 0.3)).abs() < 1e-6 && (l.a.y - by as f32).abs() < 1e-6 }),
        "the mob's box the walk serves"
    );
    // THE CAMERA ENTITY'S OWN SILENCE (the body's pose no line
    // touches — the first-person skip the walk always answers)
    assert!(
        !lines.iter().any(|l| {
            (l.a.x - (bx as f32 - 0.3)).abs() < 1e-6 && (l.a.y - by as f32).abs() < 1e-6
        }),
        "the camera entity the skip keeps silent"
    );
}

/// THE FLIGHT'S OWN APP LANES (the audited 26.2 walk the update
/// loop wears whole): the F4 apply the mayFly revoke speaks (the
/// vanilla server's own onUpdateAbilities law — the mid-flight
/// mode switch the ground asks), the spectator apply the sync
/// forces (the next tick's abilities walk), and the landing
/// retire the touched ground speaks (LocalPlayer.aiStep's tail —
/// pcs 1098-1133 — the flight the terrain retires, the
/// spectator's own the ground never claims).
#[test]
fn the_flight_app_lanes_the_gamemode_and_ground_speak() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    assert!(app.body.is_some(), "the world the body carries");
    // THE SETTLE (the spawn rhythm — the ground flag the second
    // tick's own collide carries, the resting body the flag keeps)
    for _ in 0..3 {
        app.update();
    }
    // THE CREATIVE GRANT (the switcher's own walk — the mayFly
    // the mode speaks) + THE MID-FLIGHT SURVIVAL SWITCH (the F4
    // flick the two-mode feel): the apply the flying state folds —
    // the mayFly the grant lost (the vanilla server's own
    // onUpdateAbilities law)
    app.on_open_game_mode_switcher();
    while GameMode::ICON_ORDER[app.gm_hovered] != GameMode::Creative {
        app.on_game_mode_switcher_cycle();
    }
    app.on_debug_modifier_released();
    assert_eq!(app.game_mode, GameMode::Creative, "the grant lands");
    if let Some(b) = app.body.as_mut() {
        b.flying = true;
    }
    app.on_open_game_mode_switcher();
    while GameMode::ICON_ORDER[app.gm_hovered] != GameMode::Survival {
        app.on_game_mode_switcher_cycle();
    }
    app.on_debug_modifier_released();
    assert_eq!(app.game_mode, GameMode::Survival, "the flick lands");
    assert!(
        !app.body.as_ref().expect("the body rides").flying,
        "the revoke the apply speaks"
    );
    // THE SPECTATOR SYNC THROUGH THE REAL TICK (the mode's own
    // abilities walk — the flight no press asks)
    app.game_mode = GameMode::Spectator;
    if let Some(b) = app.body.as_mut() {
        b.flying = false;
    }
    app.update();
    assert!(
        app.body.as_ref().expect("the body rides").flying,
        "the sync the tick speaks"
    );
    // THE LANDING RETIRE (LocalPlayer.aiStep's tail — pcs
    // 1098-1133): the creative body's flight the touched ground
    // retires
    app.game_mode = GameMode::Creative;
    if let Some(b) = app.body.as_mut() {
        b.flying = true;
    }
    app.update();
    assert!(
        !app.body.as_ref().expect("the body rides").flying,
        "the ground the flight retires"
    );
    // THE SPECTATOR'S GROUND THE RETIRE NEVER CLAIMS (the
    // !isSpectator gate — the forced sync the terrain keeps)
    app.game_mode = GameMode::Spectator;
    if let Some(b) = app.body.as_mut() {
        b.flying = true;
    }
    app.update();
    assert!(
        app.body.as_ref().expect("the body rides").flying,
        "the spectator's flight the ground never claims"
    );
}

#[test]
fn the_switcher_hover_walk_the_moved_mouse_steals() {
    // THE extractRenderState LAW (the firstMouseX/Y bank the first
    // drawn frame arms and the cycle press resets, the unmoved
    // mouse the cycled hover keeps, the moved mouse the slot rect
    // steals)
    let mut app = App::new(3);
    app.enter_world();
    app.on_open_game_mode_switcher();
    assert_eq!(app.gm_hovered, 1);
    // THE ARM FRAME (the setFirstMousePos walk — the first drawn
    // frame's mouse the bank stores, the hover the same frame keeps)
    let l = gamemode_switcher_layout(854, 480);
    app.mouse_pos = Some((l.slots[2].0 + 5, l.slots[2].1 + 5));
    app.gamemode_switcher_hover(&l);
    assert_eq!(
        app.gm_first_mouse,
        Some((l.slots[2].0 + 5, l.slots[2].1 + 5)),
        "the bank the arm frame fills"
    );
    assert_eq!(app.gm_hovered, 1, "the arm frame the hover keeps");
    // THE MOVED MOUSE'S OWN STEAL (the unmoved law broken — the
    // hovered slot the isHoveredOrFocused test takes)
    app.mouse_pos = Some((l.slots[2].0 + 6, l.slots[2].1 + 5));
    app.gamemode_switcher_hover(&l);
    assert_eq!(app.gm_hovered, 2, "the moved mouse steals");
    // THE OUTSIDE MISS (the slot rects the between-space never
    // matches — the cycled hover the miss keeps)
    app.mouse_pos = Some((l.slots[2].0 - 1, l.slots[2].1 + 5));
    app.gamemode_switcher_hover(&l);
    assert_eq!(app.gm_hovered, 2, "the outside mouse the hover keeps");
    // THE CYCLE RESET (the keyPressed arm — the bank the cycle
    // clears, the next frame the re-arm speaks, the unmoved mouse
    // the cycled hover keeps)
    app.on_game_mode_switcher_cycle();
    assert_eq!(app.gm_hovered, 3, "the cycle advances");
    assert_eq!(app.gm_first_mouse, None, "the bank the cycle resets");
    app.gamemode_switcher_hover(&l);
    assert_eq!(app.gm_hovered, 3, "the re-arm frame the hover keeps");
    // THE NO-MOUSE ARM (the headless scenes' absent mouse — the
    // silent walk the cycled hover keeps)
    app.mouse_pos = None;
    app.gamemode_switcher_hover(&l);
    assert_eq!(app.gm_hovered, 3, "the no-mouse arm sleeps");
}

#[test]
fn the_switcher_panel_the_pixel_law_speaks() {
    // THE PANEL'S OWN PIXEL DECOMPOSITION (the audited texture's
    // exact colors — the opaque head band the 33 fill, the grey
    // frame the 85 rails, the black outline the chamfered corners
    // keep, the translucent tail the 102-alpha body wears)
    let mut fb = crate::raster::Framebuf::new(256, 128);
    let mut g = Gui::new(&mut fb, 1);
    draw_gamemode_switcher_panel(&mut g, 0, 0);
    let px = |x: i32, y: i32| -> [u8; 3] {
        let o = ((y * 256 + x) * 3) as usize;
        [fb.color[o], fb.color[o + 1], fb.color[o + 2]]
    };
    // the black outline (the y0 row inset 2 — the chamfer the
    // corner transparency speaks)
    assert_eq!(px(1, 0), [0, 0, 0], "the chamfer's transparency");
    assert_eq!(px(2, 0), [0, 0, 0], "the y0 row starts");
    assert_eq!(px(121, 0), [0, 0, 0], "the y0 row ends");
    assert_eq!(px(122, 0), [0, 0, 0], "the chamfer keeps silent");
    assert_eq!(px(0, 2), [0, 0, 0], "the y2 rail lands");
    assert_eq!(px(124, 3), [0, 0, 0], "the three-step top-right");
    // the grey frame (85 the inner rail)
    assert_eq!(px(1, 2), [85, 85, 85], "the grey frame");
    assert_eq!(px(122, 19), [85, 85, 85], "the y19 grey row");
    // the opaque head band (33 the slots' backdrop)
    assert_eq!(px(60, 10), [33, 33, 33], "the head fill");
    // the translucent tail (the 102-alpha body over the empty
    // framebuffer — 33 * 0.4 the alpha fold speaks, rounded)
    let tail = px(60, 60);
    let expected = (33.0f32 * 0.4).round() as u8;
    assert_eq!(
        tail,
        [expected, expected, expected],
        "the tail's alpha fold"
    );
    // the empty framebuffer outside (the 125x75 box the rest skips)
    assert_eq!(px(0, 74), [0, 0, 0], "the y74 bottom row starts x3");
    assert_eq!(px(0, 0), [0, 0, 0], "the outside corner sleeps");
}

#[test]
fn the_switcher_screen_the_render_round_speaks() {
    // THE RENDER ROUND (the world beneath dimmed, the panel, the
    // texts, the slot sprites, the icons — the screen draw the
    // dispatch arm serves; the assets the cached jar keeps)
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("gui/sprites/gamemode_switcher/slot.png").exists());
    let Some(root) = root else { return };
    let mut app = App::new(3);
    app.enter_world();
    app.on_open_game_mode_switcher();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    app.render(&mut fb, &mut assets);
    // the gui space the scale-1 fb keeps (640x360)
    let l = gamemode_switcher_layout(640, 360);
    let px = |x: i32, y: i32, c: &[u8]| -> [u8; 3] {
        let o = ((y * 640 + x) * 3) as usize;
        [c[o], c[o + 1], c[o + 2]]
    };
    let color = fb.color.clone();
    // THE OPAQUE HEAD BAND (the 33 fill the world's dim the fill
    // masks whole)
    assert_eq!(
        px(l.panel.0 + 60, l.panel.1 + 10, &color),
        [33, 33, 33],
        "the head band the world masks"
    );
    // THE NAME LINE (the hovered mode's white text above the panel)
    let name_lit = (0..120)
        .filter(|&dx| {
            let (x, y) = (l.panel.0 + 60 - 60 + dx, l.name_y + 2);
            let p = px(x, y, &color);
            p[0] > 200 && p[1] > 200 && p[2] > 200
        })
        .count();
    assert!(name_lit > 0, "the name line renders white ({name_lit})");
    // THE HINT LINE (the aqua F4 word the select_next walk draws)
    let hint_lit = (0..120)
        .filter(|&dx| {
            let (x, y) = (l.panel.0 + 62 - 60 + dx, l.hint_y + 2);
            let p = px(x, y, &color);
            p[1] > 180 && p[2] > 180 && p[0] < 160
        })
        .count();
    assert!(hint_lit > 0, "the aqua hint word renders ({hint_lit})");
    // THE SLOT SPRITES (the slot.png texture the cached jar blits —
    // the lit pixels the 26x26 box keeps)
    for (i, &(sx, sy)) in l.slots.iter().enumerate() {
        let lit = (0..26)
            .map(|dy| {
                (0..26)
                    .filter(|&dx| {
                        let p = px(sx + dx, sy + dy, &color);
                        p[0] as u32 + p[1] as u32 + p[2] as u32 > 30
                    })
                    .count()
            })
            .sum::<usize>();
        assert!(lit > 300, "the slot {i} texture renders ({lit} px)");
    }
    // THE ICON (the grass block the slot 0 carries — the lit pixels
    // the +5 inset speaks)
    let icon_lit = (0..16)
        .map(|dy| {
            (0..16)
                .filter(|&dx| {
                    let p = px(l.slots[0].0 + 5 + dx, l.slots[0].1 + 5 + dy, &color);
                    p[0] as u32 + p[1] as u32 + p[2] as u32 > 60
                })
                .count()
        })
        .sum::<usize>();
    assert!(
        icon_lit > 60,
        "the grass block icon renders ({icon_lit} px)"
    );
}

#[test]
fn the_f1_key_hides_the_whole_hud_layer() {
    // THE TOGGLE'S OWN LAW (Gui.handleKeybinds's keyToggleGui lane —
    // the `Hud.toggle()` flip the bytecode 9-16 keeps:
    // `isHidden = !isHidden`, the fresh App the false the field
    // init keeps)
    let mut app = App::new(3);
    app.enter_world();
    assert!(!app.hud_hidden);
    app.on_toggle_gui_key();
    assert!(app.hud_hidden);
    app.on_toggle_gui_key();
    assert!(!app.hud_hidden);

    // THE PIXELS' OWN VERDICT (the extractRenderState's hidden
    // groups the bare world the frame keeps — the hotbar chrome,
    // the crosshair, the XP bar's dark background the hidden frame
    // all skip): the shown render paints, the hidden render the
    // bare 3D world answers, the HUD regions the diff count speaks
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("gui/container/inventory.png").exists());
    let Some(root) = root else { return };
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE DAY CLOCK'S OWN PLATEAU (the frame == tick model the sky
    // lane rides — the clock_total the test pins into the flat day
    // plateau 133..11867: the shown/hidden pair the SAME frame
    // renders, the sky's dawn ramp the pinned clock never crosses,
    // the diff the HUD layer ALONE answers)
    app.clock_total = 6000 + app.frame;
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    app.render(&mut fb, &mut assets);
    let shown = fb.color.clone();
    // 640x360 the scale-1 gui keeps (auto_scale's 240-floor the
    // 360 height refuses past 1) — the gui space the fb space is
    let (hx, hy) = hotbar_rect(640, 360);
    let diff_count = |a: &[u8], b: &[u8], x0: i32, y0: i32, w: i32, h: i32| -> usize {
        let mut n = 0usize;
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                let o = ((y as usize) * 640 + x as usize) * 3;
                if a[o..o + 3] != b[o..o + 3] {
                    n += 1;
                }
            }
        }
        n
    };
    // THE HIDDEN RENDER (the same App the F1 flip answers — the
    // whole HUD layer the early return skips, the bare 3D world
    // the frame keeps)
    app.on_toggle_gui_key();
    app.render(&mut fb, &mut assets);
    let hidden = fb.color.clone();
    // THE HOTBAR + XP ROW (the hotbar 182x22 chrome + the bar's
    // own 182x5 the hidden frame skips — the terrain the diff
    // speaks)
    let hud_bottom_diff = diff_count(&shown, &hidden, hx, hy - 8, 182, 30);
    assert!(
        hud_bottom_diff > 500,
        "the hidden HUD's bottom row the bare world answers: {hud_bottom_diff}"
    );
    // THE CROSSHAIR (the 15x15 the first-person frame paints, the
    // hidden frame the world's own pixels keeps — the sprite's own
    // 17-pixel alpha census the white cross answers, the idempotent
    // render the HUD layer alone the diff speaks)
    let cross_diff = diff_count(&shown, &hidden, 640 / 2 - 7, 360 / 2 - 7, 15, 15);
    assert!(
        cross_diff >= 15,
        "the crosshair the bare frame skips: {cross_diff}"
    );

    // THE DEBUG OVERLAY'S OWN GATE (DebugScreenOverlay
    // .extractRenderState's `hud.isHidden() && gui.screen() == null`
    // → return — the world screen the null-screen shape keeps): the
    // F3-open + hidden frame the hidden no-debug frame's own bytes
    // answers — the two renders byte-identical, the panel the
    // hidden gate never draws
    let mut fb_debug = crate::raster::Framebuf::new(640, 360);
    app.on_toggle_gui_key(); // the flag drops — the HUD + the open panel return
    app.debug_held = true;
    app.render(&mut fb_debug, &mut assets);
    let debug_shown = fb_debug.color.clone();
    let mut fb_hidden_debug = crate::raster::Framebuf::new(640, 360);
    app.on_toggle_gui_key(); // the flag raises over the open panel
    app.render(&mut fb_hidden_debug, &mut assets);
    assert_ne!(
        shown, debug_shown,
        "the F3-open frame the shown HUD's own bands + text wears"
    );
    assert_eq!(
        hidden, fb_hidden_debug.color,
        "the hidden frame the F3-open panel never draws — the bare world twice"
    );

    // THE PAUSE SCREEN'S OWN LAYER (the vanilla Screen the
    // isHidden flag never gates — the pause the dim + the title +
    // the buttons the hidden frame still wears)
    app.paused = true;
    let mut fb_pause = crate::raster::Framebuf::new(640, 360);
    app.render(&mut fb_pause, &mut assets);
    let pause_diff = diff_count(&hidden, &fb_pause.color, 0, 0, 640, 360);
    assert!(
        pause_diff > 10_000,
        "the pause screen the hidden HUD's frame still paints: {pause_diff}"
    );
}

/// THE DECORATIONS GATE'S OWN PIXELS (javap Hud
/// .extractHotbarAndDecorations pcs 33-50 — the canHurtPlayer
/// `ifeq` the extractPlayerHealth body skips, the pcs 118-164
/// `hasExperience() && experienceLevel > 0` the XP digits gate,
/// scripts/jar_work/hud_extract_262.txt + hudgates_262.txt): the
/// survival frame the hearts + XP rows paint, the creative flick
/// the rows never draw — the decoration regions the bare world's
/// own bytes answer (the F1-hidden frame's truth the same
/// App renders), the hotbar chrome the gate never gates (the pcs
/// 0-32 item-hotbar arm the creative frame keeps), the flick back
/// the rows resume — the survival/adventure lane the isSurvival
/// verdict wears.
#[test]
fn the_survival_gate_the_decorations_rows_sleep() {
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("gui/container/inventory.png").exists());
    let Some(root) = root else { return };
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    // 640x360 the scale-1 gui keeps — the gui space the fb space is
    let (hx, hy) = hearts_origin(640, 360);
    let (xx, xy) = (640 / 2 - 91, 360 - 29); // the XP bar's own slot
    let (bx, by) = hotbar_rect(640, 360);
    let diff_count = |a: &[u8], b: &[u8], x0: i32, y0: i32, w: i32, h: i32| -> usize {
        let mut n = 0usize;
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                let o = ((y as usize) * 640 + x as usize) * 3;
                if a[o..o + 3] != b[o..o + 3] {
                    n += 1;
                }
            }
        }
        n
    };
    // THE SURVIVAL FRAME: the hearts + the XP bar's dark background
    // the full-health body paints
    app.render(&mut fb, &mut assets);
    let survival = fb.color.clone();
    // THE CREATIVE FLICK: the gate the rows never draw
    app.game_mode = GameMode::Creative;
    app.render(&mut fb, &mut assets);
    let creative = fb.color.clone();
    // THE BARE FRAME (the F1-hidden render — the whole HUD layer
    // the early return skips, the bare 3D world the decorations'
    // own silence answers)
    app.on_toggle_gui_key();
    app.render(&mut fb, &mut assets);
    let bare = fb.color.clone();
    app.on_toggle_gui_key(); // the flag drops — the HUD returns
                             // THE HEARTS ROW the gate hides (182x9 the sprite row's own
                             // band): the survival frame the row paints over the bare
                             // world, the creative frame the row never draws — the region
                             // the bare bytes keep verbatim
    let surv_vs_crea = diff_count(&survival, &creative, hx, hy, 182, 9);
    assert!(
        surv_vs_crea > 500,
        "the survival hearts the creative flick never paints: {surv_vs_crea}"
    );
    let same = hx as usize..(hx + 182) as usize;
    for y in hy..hy + 9 {
        let yo = (y as usize) * 640 * 3;
        assert_eq!(
            creative[yo + same.start * 3..yo + same.end * 3],
            bare[yo + same.start * 3..yo + same.end * 3],
            "the creative hearts row the bare world's own bytes answer"
        );
    }
    // THE XP BAR the sibling verdict hides (182x5 the
    // contextual-bar background's own slot — level 0's progress
    // the fill never arms, the dark background the survival lane
    // paints)
    let xp_surv_vs_crea = diff_count(&survival, &creative, xx, xy, 182, 5);
    assert!(
        xp_surv_vs_crea > 400,
        "the survival XP background the creative flick never paints: {xp_surv_vs_crea}"
    );
    for y in xy..xy + 5 {
        let yo = (y as usize) * 640 * 3;
        assert_eq!(
            creative[yo + same.start * 3..yo + same.end * 3],
            bare[yo + same.start * 3..yo + same.end * 3],
            "the creative XP row the bare world's own bytes answer"
        );
    }
    // THE HOTBAR the gate never gates (the pcs 0-32 item-hotbar arm
    // the creative frame keeps — the chrome-only shape the
    // decorations' silence never touches)
    let hot_diff = diff_count(&creative, &bare, bx, by, 182, 22);
    assert!(
        hot_diff > 500,
        "the creative hotbar the gate never hides: {hot_diff}"
    );
    // THE FLICK BACK: the survival rows resume (the gate the
    // verdict's own true lane wears)
    app.game_mode = GameMode::Survival;
    app.render(&mut fb, &mut assets);
    let resumed = fb.color.clone();
    let resumed_diff = diff_count(&resumed, &creative, hx, hy, 182, 9);
    assert!(
        resumed_diff > 500,
        "the flick back the hearts row resumes: {resumed_diff}"
    );
}

#[test]
fn the_spectator_menu_item_walk_the_audited_slots_speak() {
    // THE getItem WALK (javap SpectatorMenu.getItem pcs 0-120 — the
    // verbatim fold the root page wears: the two teleport
    // categories the root's own list carries, the EMPTY_SLOT the
    // out-of-bounds arm keeps, the slot-7 scroll-right the `j <
    // items.size()` verdict arms, the slot-8 the CLOSE item)
    assert_eq!(
        spectator_menu_item(0, 0),
        SpectatorMenuItem::TeleportToPlayer,
        "the root's first category"
    );
    assert_eq!(
        spectator_menu_item(0, 1),
        SpectatorMenuItem::TeleportToTeam,
        "the root's second category"
    );
    for slot in 2..7 {
        assert_eq!(
            spectator_menu_item(0, slot),
            SpectatorMenuItem::EmptySlot,
            "the empty lane the out-of-bounds arm keeps: slot {slot}"
        );
    }
    assert_eq!(
        spectator_menu_item(0, 7),
        SpectatorMenuItem::ScrollRightDisabled,
        "the scroll-right the root page's 2-item verdict disables"
    );
    assert_eq!(
        spectator_menu_item(0, 8),
        SpectatorMenuItem::Close,
        "the close the slot-8 arm carries"
    );
    // THE NEGATIVE SLOT the EMPTY_SLOT's own arm answers — the
    // fresh menu's selectedSlot -1 the getSelectedItem walk reads
    assert_eq!(
        spectator_menu_item(0, -1),
        SpectatorMenuItem::EmptySlot,
        "the -1 the fresh menu's highlight keeps"
    );
    // THE PAGE-1 SHAPE (the walk the offline lane never rides —
    // the scroll-left the page>0 slot-0 arm answers, the page
    // shift the j = slot + page*6 fold carries)
    assert_eq!(
        spectator_menu_item(1, 0),
        SpectatorMenuItem::ScrollLeft,
        "the scroll-left the page>0 arm carries"
    );
    assert_eq!(
        spectator_menu_item(1, 8),
        SpectatorMenuItem::Close,
        "the close the every-page arm carries"
    );
    // THE OFFLINE VERDICTS (javap TeleportToPlayerMenuCategory
    // isEnabled pcs 0-17 + the lambda$new$0 filter — the listed
    // players the SPECTATOR mode excludes, the only listed player
    // the menu's own existence empties — the categories the empty
    // lists keep DISABLED, the close the iconst_1 answers)
    assert!(!SpectatorMenuItem::TeleportToPlayer.is_enabled());
    assert!(!SpectatorMenuItem::TeleportToTeam.is_enabled());
    assert!(SpectatorMenuItem::Close.is_enabled());
    assert!(!SpectatorMenuItem::ScrollRightDisabled.is_enabled());
    // THE NAME KEYS the en_us table answers (javap the items' own
    // Component.translatable constants)
    assert_eq!(SpectatorMenuItem::Close.name_key(), "spectatorMenu.close");
    assert_eq!(
        SpectatorMenuItem::TeleportToPlayer.name_key(),
        "spectatorMenu.teleport"
    );
    assert_eq!(
        SpectatorMenuItem::TeleportToTeam.name_key(),
        "spectatorMenu.team_teleport"
    );
    // THE ICON SPRITES the generated table carries (javap the
    // CLOSE_SPRITE/SCROLL/TELEPORT constants — the assets-cache
    // the files answer)
    assert_eq!(SpectatorMenuItem::Close.icon(), "spectator/close");
    assert_eq!(
        SpectatorMenuItem::TeleportToPlayer.icon(),
        "spectator/teleport_to_player"
    );
    assert_eq!(
        SpectatorMenuItem::TeleportToTeam.icon(),
        "spectator/teleport_to_team"
    );
    // THE ALPHA SEAM (javap getHotbarAlpha pcs 0-23 — the
    // clamp((3000 - elapsed) / 1000, 0, 1) fold)
    assert_eq!(hotbar_alpha_from(0), 1.0, "the fresh clock the full answer");
    assert_eq!(
        hotbar_alpha_from(2500),
        0.5,
        "the final second's half the fade rides"
    );
    assert_eq!(hotbar_alpha_from(3000), 0.0, "the spent clock the zero");
    assert_eq!(hotbar_alpha_from(9000), 0.0, "the stale clock the zero");
}

#[test]
fn the_spectator_swap_the_item_hotbar_sleeps() {
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("gui/container/inventory.png").exists());
    let Some(root) = root else { return };
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    let (bx, by) = hotbar_rect(640, 360);
    let diff_count = |a: &[u8], b: &[u8], x0: i32, y0: i32, w: i32, h: i32| -> usize {
        let mut n = 0usize;
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                let o = ((y as usize) * 640 + x as usize) * 3;
                if a[o..o + 3] != b[o..o + 3] {
                    n += 1;
                }
            }
        }
        n
    };
    // THE BARE FRAME (the F1-hidden render — the whole HUD layer
    // the early return skips)
    app.on_toggle_gui_key();
    app.render(&mut fb, &mut assets);
    let bare = fb.color.clone();
    app.on_toggle_gui_key(); // the HUD returns
                             // THE SURVIVAL CONTROL: the item hotbar the non-spectator arm
                             // paints (the extractItemHotbar lane the pc 27-30 walk keeps)
    app.render(&mut fb, &mut assets);
    let survival = fb.color.clone();
    assert!(
        diff_count(&survival, &bare, bx, by, 182, 22) > 500,
        "the survival hotbar the non-spectator arm paints"
    );
    // THE CREATIVE CONTROL: the item hotbar the swap never gates
    // (the mode check the SPECTATOR arm alone opens)
    app.game_mode = GameMode::Creative;
    app.render(&mut fb, &mut assets);
    let creative = fb.color.clone();
    assert!(
        diff_count(&creative, &bare, bx, by, 182, 22) > 500,
        "the creative hotbar the swap never gates"
    );
    // THE SPECTATOR FRAME: the item hotbar the swap skips — the
    // region the bare world's own bytes answer (the extractHotbar
    // walk the pc 0-7 menu-null silence keeps, the menu the keys
    // alone create)
    app.game_mode = GameMode::Spectator;
    app.render(&mut fb, &mut assets);
    let spectator = fb.color.clone();
    assert_eq!(
        diff_count(&spectator, &bare, bx, by, 182, 22),
        0,
        "the spectator item hotbar the swap skips — the region the bare bytes answer"
    );
}

#[test]
fn the_spectator_menu_chrome_the_widgets_draw() {
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("gui/container/inventory.png").exists());
    let Some(root) = root else { return };
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    let diff_count = |a: &[u8], b: &[u8], x0: i32, y0: i32, w: i32, h: i32| -> usize {
        let mut n = 0usize;
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                let o = ((y as usize) * 640 + x as usize) * 3;
                if a[o..o + 3] != b[o..o + 3] {
                    n += 1;
                }
            }
        }
        n
    };
    // THE BARE FRAME
    app.on_toggle_gui_key();
    app.render(&mut fb, &mut assets);
    let bare = fb.color.clone();
    app.on_toggle_gui_key();
    // THE SPECTATOR FLICK + THE MENU CREATE (the first press the
    // create keeps — the selectedSlot -1 the <init>'s own law)
    app.game_mode = GameMode::Spectator;
    app.on_number_key(0);
    assert!(app.spectator_menu.is_some(), "the first press creates");
    assert_eq!(
        app.spectator_menu.as_ref().unwrap().selected_slot,
        -1,
        "the fresh menu nothing highlights"
    );
    // THE SECOND PRESS the slot-0 highlight selects — the menu
    // alpha the fresh clock's 1.0 keeps, the slide the floor
    // (360 - 61) the 61.0f fold speaks
    app.on_number_key(0);
    assert_eq!(
        app.spectator_menu.as_ref().unwrap().selected_slot,
        0,
        "the second press the slot selects"
    );
    assert!(
        (app.spectator_hotbar_alpha() - 1.0).abs() < 1e-6,
        "the fresh clock the full-bright window wears"
    );
    app.render(&mut fb, &mut assets);
    let open = fb.color.clone();
    // THE CHROME: the widgets the extractPage paints (the hotbar
    // sprite 182x22 at cx-91, the slide's own y = floor(360 - 61) =
    // 299, the selection 24x23 at the slot's own x, the icons the
    // extractSlot carries, the key labels the enabled slots wear)
    let (mx, my) = (640 / 2 - 91, 360 - 61);
    assert!(
        diff_count(&open, &bare, mx, my - 1, 182, 24) > 500,
        "the menu chrome the extractPage paints"
    );
    // THE ACTION NAME (the extractAction walk — the selected
    // TeleportToPlayer the item's own name answers, the text the
    // centered row at h - 35 wears)
    assert!(
        diff_count(&open, &bare, 100, 360 - 36, 440, 10) > 0,
        "the action name the selected item's own key carries"
    );
    // THE ITEM HOTBAR the swap still skips (the menu the slide's
    // own y paints, the region the bare bytes keep)
    let (bx, by) = hotbar_rect(640, 360);
    assert_eq!(
        diff_count(&open, &bare, bx, by, 182, 22),
        0,
        "the item hotbar region the menu-open frame never paints"
    );
    // THE FADED CLOCK the exit speaks (the stale selection the
    // render's own walk closes — the extractHotbar's alpha<=0 arm
    // the menu.exit() answers)
    app.spectator_last_selection =
        Some(std::time::Instant::now() - std::time::Duration::from_millis(4000));
    app.render(&mut fb, &mut assets);
    let closed = fb.color.clone();
    assert!(
        app.spectator_menu.is_none(),
        "the faded clock the exit speaks"
    );
    assert!(app.spectator_last_selection.is_none(), "the clock the zero");
    assert_eq!(
        diff_count(&closed, &bare, mx, my - 1, 182, 24),
        0,
        "the closed menu the region the bare bytes answer"
    );
}

#[test]
fn the_spectator_inputs_the_menu_walks_speak() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE SURVIVAL CONTROL: the number keys the hotbar selection
    // wear, the menu the lane never creates
    app.on_number_key(0);
    assert_eq!(
        app.selected_hotbar, 0,
        "the survival key the selection moves"
    );
    assert!(app.spectator_menu.is_none(), "the survival lane no menu");
    // THE SPECTATOR CREATE + SELECT (the first press the create
    // keeps, the second the slot-0 highlight answers)
    app.game_mode = GameMode::Spectator;
    app.on_number_key(0);
    app.on_number_key(0);
    assert_eq!(
        app.spectator_menu.as_ref().unwrap().selected_slot,
        0,
        "the highlight the second press selects"
    );
    // THE DISABLED ACTIVATION the verdict refuses (the third press —
    // the selectSlot's `cur == slot && enabled` gate the 0.5-color
    // category fails, the move arm the same slot writes)
    app.on_number_key(0);
    assert!(
        app.spectator_menu.is_some(),
        "the disabled category the activation refuses — the menu stays"
    );
    assert_eq!(
        app.spectator_menu.as_ref().unwrap().selected_slot,
        0,
        "the highlight the refusal keeps"
    );
    // THE WHEEL'S SKIP WALK (javap onMouseScrolled pcs 0-82 — the
    // route the -direction the spectator arm feeds, the EMPTY and
    // disabled slots the `j += i` loop skips, the CLOSE the slot-8
    // landing selects): the up-scroll the +2 direction the negated
    // fold feeds — the walk from slot 0 the close lands on
    app.on_hotbar_scroll(0.0, -1.0);
    assert_eq!(
        app.spectator_menu.as_ref().unwrap().selected_slot,
        8,
        "the wheel the skip loop the close lands on"
    );
    // THE ACTION KEY THE CLOSE ACTIVATES (javap
    // onHotbarActionKeyPressed pcs 14-32 — the highlighted slot the
    // re-select answers, the CloseSpectatorItem's selectItem the
    // menu.exit() speaks)
    app.spectator_on_action_key();
    assert!(app.spectator_menu.is_none(), "the close the exit speaks");
    // THE WHEEL THE MENU-CLOSED LANE THE FLYING SPEED ADJUSTS (the
    // test below the speed fold wears) — the menu the wheel alone
    // never recreates
    assert!(
        app.spectator_menu.is_none(),
        "the closed lane the wheel keeps"
    );
}

#[test]
fn the_spectator_fade_the_speed_spins_the_survival_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE MENU-CLOSED WHEEL THE FLYING SPEED FOLDS (javap
    // MouseHandler.onScroll pcs 316-364 — the spectator arm's own
    // flying-speed lane: `flyingSpeed + y * 0.005` the step, the
    // clamp(0.0, 0.2) the pair speaks — the y the folded steps'
    // own 2x the sensitivity carries)
    app.game_mode = GameMode::Spectator;
    app.on_hotbar_scroll(0.0, 1.0);
    let b = app.body.as_ref().unwrap();
    assert!(
        (b.flying_speed - 0.06).abs() < 1e-6,
        "the two-step fold the +0.01 rides: {}",
        b.flying_speed
    );
    // THE CLAMP the lower arm answers
    app.on_hotbar_scroll(0.0, -100.0);
    let b = app.body.as_ref().unwrap();
    assert_eq!(b.flying_speed, 0.0, "the clamp the floor keeps");
    // THE CLAMP the upper arm answers
    app.on_hotbar_scroll(0.0, 100.0);
    let b = app.body.as_ref().unwrap();
    assert_eq!(b.flying_speed, 0.2, "the clamp the ceiling keeps");
    // THE SURVIVAL CONTROL: the wheel the hotbar cycle wears, the
    // speed the lane never touches
    app.game_mode = GameMode::Survival;
    let before = app.selected_hotbar;
    app.on_hotbar_scroll(0.0, 1.0);
    assert_ne!(
        app.selected_hotbar, before,
        "the survival wheel the selection cycles"
    );
    let b = app.body.as_ref().unwrap();
    assert_eq!(
        b.flying_speed, 0.2,
        "the survival wheel the speed never touches"
    );
}

#[test]
fn the_chat_decoration_the_lang_walk_speaks() {
    // THE FORMAT LAW (the en_us table's chat.type.text the `<%s> %s`
    // entry carries — the sequential %s walk the TranslatableContents
    // decompose keeps)
    assert_eq!(
        format_chat_decoration("chat.type.text", &["Steve", "gg"]),
        "<Steve> gg"
    );
    assert_eq!(
        format_chat_decoration("chat.type.emote", &["Steve", "waves"]),
        "* Steve waves"
    );
    // THE ARG SHORTFALL (the missing slot the empty string fills —
    // the format the extra %s never speaks)
    assert_eq!(
        format_chat_decoration("chat.type.text", &["Steve"]),
        "<Steve> "
    );
    // THE MISSING KEY (the getOrDefault law — the key itself the
    // lookup answers, the args the format never speaks)
    assert_eq!(
        format_chat_decoration("server.custom.key", &["a", "b"]),
        "server.custom.key"
    );
}

#[test]
fn the_positional_walk_the_arg_order_speaks() {
    // THE ORDER SWAP (the `%N$s` form the 1-based index answers —
    // the lang table's own rows the same shape carries:
    // inventory.hotbarSaved `%1$s + %2$s`, mount.onboard `%1$s`,
    // difficulty.lock.question `%1$s`)
    assert_eq!(
        format_positional("%2$s before %1$s", &["first", "second"]),
        "second before first",
        "the positional index the order answers"
    );
    // THE MIXED WALK (the decompose walk's own counter law — ONLY
    // the plain %s branch the counter consumes, the positional read
    // the counter never touches: the mixed template `%1$s %s` the
    // args [a, b] the a a pair answers, the plain %s the FIRST
    // non-positional slot fills)
    assert_eq!(
        format_positional("%1$s %s %1$s", &["a", "b"]),
        "a a a",
        "the positional the sequential counter never advances"
    );
    // THE LITERAL PERCENT (the `%%` escape the walk folds)
    assert_eq!(
        format_positional("100%% %s", &["done"]),
        "100% done",
        "the doubled percent the literal keeps"
    );
    // THE MALFORMED WALKS (the trailing %, the unknown escape, the
    // digit run the `$s` tail never wears — each the literal percent
    // the honest walk emits, the next chars themselves answer)
    assert_eq!(format_positional("50%", &[]), "50%");
    assert_eq!(format_positional("%x", &["a"]), "%x");
    assert_eq!(format_positional("%1x", &["a"]), "%1x");
    assert_eq!(format_positional("%1$y", &["a"]), "%1$y");
    // THE MISSING SLOTS (the out-of-range positional the empty
    // string fills, the `%0$s` the 1-based law refuses)
    assert_eq!(format_positional("[%9$s]", &["a"]), "[]");
    assert_eq!(format_positional("[%0$s]", &["a"]), "[]");
    // THE MULTI-DIGIT INDEX (the digit run the run answers)
    assert_eq!(
        format_positional(
            "%10$s wins",
            &["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"]
        ),
        "j wins",
        "the two-digit index the tenth arg answers"
    );
    // THE SIBLING RESET (the §-bearing positional arg the §r close
    // speaks — the template's literals the template's style wear)
    assert_eq!(
        format_positional("hey %1$s!", &["\u{a7}9Steve"]),
        "hey \u{a7}9Steve\u{a7}r!",
        "the code-bearing arg the reset keeps"
    );
    // THE LANG TABLE'S OWN ROW (the en_us inventory.hotbarSaved the
    // general walk answers — the X + 4 pair the overlay speaks)
    assert_eq!(
        format_positional(
            tr("inventory.hotbarSaved").unwrap_or("Item hotbar saved (restore with %1$s + %2$s)"),
            &["X", "4"],
        ),
        "Item hotbar saved (restore with X + 4)",
        "the table row the positional walk fills"
    );
}

#[test]
fn the_player_chat_line_the_decoration_walk_answers() {
    // THE PARAMETER WALK (the bound's parameters name each %s's
    // source: "sender" the name, "content" the text, "target" the
    // absent slot the empty string fills)
    let m = mc_protocol::play::PlayerChatMessage {
        sender: [7u8; 16],
        sender_name: "Steve".to_string(),
        target_name: None,
        content: "gg".to_string(),
        chat_key: "chat.type.text".to_string(),
        parameters: vec!["sender".to_string(), "content".to_string()],
    };
    assert_eq!(format_player_chat_line(&m), "<Steve> gg");
    // THE TARGET SLOT (the team lane's three-%s key the target fills)
    let m = mc_protocol::play::PlayerChatMessage {
        sender: [7u8; 16],
        sender_name: "Steve".to_string(),
        target_name: Some("Red Team".to_string()),
        content: "hi team".to_string(),
        chat_key: "chat.type.team.text".to_string(),
        parameters: vec![
            "target".to_string(),
            "sender".to_string(),
            "content".to_string(),
        ],
    };
    assert_eq!(format_player_chat_line(&m), "Red Team <Steve> hi team");
}

#[test]
fn the_live_submit_the_wire_lane_rides() {
    // THE LIVE WALK (the submit's wire half — the trimmed message
    // the command channel's SendChat carries, the LOCAL echo the
    // send skips: the server's own Player Chat echo the display
    // queue answers through the pump's dedicated channel)
    let mut app = App::new(3);
    app.enter_world();
    let (chat_tx, chat_rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(chat_tx);
    app.on_chat_key();
    for c in "hello world".chars() {
        app.on_chat_char(c);
    }
    assert!(app.chat_open);
    app.on_chat_submit();
    assert!(!app.chat_open, "the close the submit keeps");
    assert_eq!(app.chat_input, "", "the input the close clears");
    assert_eq!(
        app.recent_chat.last().map(String::as_str),
        Some("hello world"),
        "the history the submit keeps"
    );
    match chat_rx.try_recv().expect("the send rode the lane") {
        crate::world_view::ContainerCommand::SendChat { message } => {
            assert_eq!(message, "hello world");
        }
        other => panic!("the wrong command rode the lane: {other:?}"),
    }
    assert!(
        app.chat_messages.iter().all(|m| m.0 != "hello world"),
        "the LOCAL echo the wire send skips — the server's echo the display answers"
    );
}

#[test]
fn the_slash_submit_the_command_body_rides() {
    // THE COMMAND WALK (the ChatScreen's startsWith("/") +
    // substring(1) law — the BODY the command packet carries, the
    // slash never rides). The TYPING lane the suggestion requests
    // ride first (the ask-server walk the '/'-edits answer — the
    // drain the submit's own packet asserts past).
    let mut app = App::new(3);
    app.enter_world();
    let (chat_tx, chat_rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(chat_tx);
    app.on_chat_key();
    for c in "/time set day".chars() {
        app.on_chat_char(c);
    }
    for cmd in chat_rx.try_iter() {
        match cmd {
            crate::world_view::ContainerCommand::RequestCommandSuggestions { .. } => {}
            other => panic!("the typing lane rode the wrong packet: {other:?}"),
        }
    }
    app.on_chat_submit();
    match chat_rx.try_recv().expect("the command rode the lane") {
        crate::world_view::ContainerCommand::SendChatCommand { command } => {
            assert_eq!(command, "time set day", "the slash the strip keeps");
        }
        other => panic!("the wrong command rode the lane: {other:?}"),
    }
    assert!(
        app.recent_chat.last().map(String::as_str) == Some("/time set day"),
        "the history the typed word keeps"
    );
}

/// THE COMMAND KEY'S OWN OPEN WALK (the on_command_key lane the
/// slash key speaks — the ChatMethod.COMMAND prefix the fresh open
/// wears BEFORE any keystroke, the suggestions the '/'-prefixed
/// input the LOCAL parse answers, the narrowed type-ahead the same
/// walk re-answers, the close the fresh-open law keeps).
#[test]
fn the_command_key_the_prefix_open_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_tree());
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    // THE OPEN (the '/' prefix the input wears — no keystroke the
    // walk spends, the draft/history walks the fresh open answers)
    app.on_command_key();
    assert!(app.chat_open, "the command key the screen opens");
    assert_eq!(app.chat_input, "/", "the COMMAND prefix the input wears");
    assert!(
        app.chat_draft.is_empty(),
        "the scratch draft the clear keeps"
    );
    assert_eq!(
        app.chat_history_pos,
        app.recent_chat.len(),
        "the history pos the end re-arms"
    );
    // THE BARE SLASH (the root's own children the local parse
    // answers — the same rows the typed '/' walk speaks, the range
    // [1,0) the prefix-length law keeps)
    assert_eq!(
        app.chat_suggestions
            .iter()
            .map(|m| m.text.as_str())
            .collect::<Vec<_>>(),
        vec!["tick", "time", "tp"],
        "the bare slash the root's own children answer"
    );
    assert_eq!(app.chat_suggestion_range, (1, 0));
    assert_eq!(app.chat_suggestion_original, "/");
    // THE TYPE-AHEAD (the 't' the narrowed sort answers — the
    // prefix the typed tail rides)
    app.on_chat_char('t');
    assert_eq!(
        app.chat_suggestions
            .iter()
            .map(|m| m.text.as_str())
            .collect::<Vec<_>>(),
        vec!["tick", "time", "tp"]
    );
    assert_eq!(app.chat_input, "/t", "the char the prefix rides");
    // THE CLOSE (the fresh-open law the Escape walk keeps — the
    // suggestions the screen never keeps, the input the empty
    // value the next open wears)
    app.close_chat();
    assert!(!app.chat_open, "the close the screen folds");
    assert_eq!(app.chat_input, "", "the close the input clears");
    assert!(
        app.chat_suggestions.is_empty(),
        "the close the suggestions forgets"
    );
}

/// THE COMMAND KEY'S OWN GATES (the live-only + closed-char laws
/// the prefix open answers: the tree-less offline world the
/// suggestions keep silent, the closed chat the input callback's
/// '/' the gate drops — the prefix the command lane ALONE carries,
/// no doubling the fresh open wears).
#[test]
fn the_command_prefix_the_closed_char_and_offline_gates_keep() {
    let mut app = App::new(3);
    app.enter_world();
    // THE OFFLINE OPEN (no live clicks the gate reads — the input
    // the '/' wears, the suggestions the live-only law keeps
    // silent, no pending id the gate waits on)
    app.on_command_key();
    assert!(app.chat_open, "the offline world the screen still opens");
    assert_eq!(app.chat_input, "/");
    assert!(
        app.chat_suggestions.is_empty(),
        "the offline world the suggestions keep silent"
    );
    assert!(
        app.chat_suggestion_pending.is_none(),
        "the offline world no id waits on"
    );
    app.close_chat();
    // THE CLOSED-CHAR GATE (the input callback's '/' the closed
    // screen keeps silent — the queue's same-frame char the drain
    // answers, the chat the gate never opens)
    app.on_chat_char('/');
    assert!(!app.chat_open, "the char the gate never opens");
    assert_eq!(app.chat_input, "", "the silent gate the input never wears");
    // THE NO-DOUBLING WALK (the reopen the command lane speaks —
    // the prefix the lane ALONE carries: the dropped callback char
    // the input never doubles)
    app.on_command_key();
    assert_eq!(
        app.chat_input, "/",
        "the prefix the command lane alone carries"
    );
}

/// THE DRAFT BANK'S OWN WALK (javap ChatComponent.saveAsDraft +
/// ChatScreen.removed + createScreen — scripts/jar_work/
/// chatcomp_full_49.txt + chatscreen_49.txt the full-disasm banks):
/// the ESC close the non-blank input the saveAsDraft lane banks
/// (the method the startsWith("/") walk names), the reopen the
/// MESSAGE method's isDraftRestorable the `iconst_1` arm answers,
/// the restored draft the isDraft purity wears, the FIRST edit the
/// onEdited drop answers (the edited text the next close saves
/// back), the whitespace-only close the isBlank discard keeps.
#[test]
fn the_esc_close_the_draft_bank_and_restore_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    // THE TYPE + ESC CLOSE (the save lane — the !isDraft else-arm
    // the raw text banks, the MESSAGE method the bare word wears)
    app.on_chat_key();
    assert!(
        !app.chat_input_is_draft,
        "the fresh open the purity never wears"
    );
    for c in "hello world".chars() {
        app.on_chat_char(c);
    }
    app.close_chat();
    assert!(!app.chat_open);
    assert_eq!(
        app.chat_latest_draft,
        Some(("hello world".to_string(), false)),
        "the close the message draft banks"
    );
    // THE RESTORED OPEN (the createScreen restorable arm — the text
    // the input wears, the isDraft purity the true arg banks)
    app.on_chat_key();
    assert!(app.chat_open);
    assert_eq!(
        app.chat_input, "hello world",
        "the draft the reopen restores"
    );
    assert!(
        app.chat_input_is_draft,
        "the restored draft the purity wears"
    );
    // THE FIRST EDIT (the onEdited walk — the purity the keystroke
    // drops, the edited text the next close's save lane answers)
    app.on_chat_char('x');
    assert!(!app.chat_input_is_draft, "the edit the purity drops");
    app.close_chat();
    assert_eq!(
        app.chat_latest_draft,
        Some(("hello worldx".to_string(), false)),
        "the edited close the save lane banks"
    );
    app.on_chat_key();
    assert_eq!(app.chat_input, "hello worldx");
    // THE BLANK CLOSE (the isBlank law — the whitespace-only input
    // the discard arm keeps, the bank the fresh-open law clears)
    while !app.chat_input.is_empty() {
        app.on_chat_backspace();
    }
    assert!(
        !app.chat_input_is_draft,
        "the emptied box the purity never wears"
    );
    app.close_chat();
    assert_eq!(
        app.chat_latest_draft, None,
        "the blank close the bank discards"
    );
    app.on_chat_key();
    assert_eq!(app.chat_input, "", "the fresh open the empty box wears");
    assert!(!app.chat_input_is_draft);
}

/// THE COMMAND DRAFT'S OWN METHOD LANES (the createScreen fold the
/// ChatMethod's $1/$2 bodies answer — scripts/jar_work/
/// chatcomp_full_49.txt + the full-disasm isDraftRestorable walks:
/// the MESSAGE method ANY draft keeps, the COMMAND method the
/// `draft.chatMethod == COMMAND` equality speaks — the message
/// drafts the '/'-open never restores, the prefix arm the bare
/// '/' carries; the '/'-draft the T-open restores too, the text
/// the command method keeps).
#[test]
fn the_command_draft_the_method_lanes_keep() {
    let mut app = App::new(3);
    app.enter_world();
    // THE COMMAND DRAFT BANKS (the '/'-open's own prefix the body
    // rides — the typed chars the bare word wears, the close the
    // COMMAND method banks)
    app.on_command_key();
    for c in "time set day".chars() {
        app.on_chat_char(c);
    }
    app.close_chat();
    assert_eq!(
        app.chat_latest_draft,
        Some(("/time set day".to_string(), true)),
        "the slash close the command draft banks"
    );
    // THE SLASH REOPEN (the $2 equality arm — the command draft the
    // COMMAND lane restores, the purity the true arg wears)
    app.on_command_key();
    assert_eq!(
        app.chat_input, "/time set day",
        "the command draft restores"
    );
    assert!(
        app.chat_input_is_draft,
        "the restored open the purity wears"
    );
    // THE UNTOUCHED CLOSE (the skip-save arm — the banked record
    // the SAME text the purity gate keeps, the re-save the no-op)
    app.close_chat();
    assert_eq!(
        app.chat_latest_draft,
        Some(("/time set day".to_string(), true)),
        "the untouched close the record keeps"
    );
    // THE T-OPEN (the $1 arm — the MESSAGE method ANY draft keeps:
    // the '/'-draft the message lane restores, no extra prefix)
    app.on_chat_key();
    assert_eq!(
        app.chat_input, "/time set day",
        "the message lane the command draft restores"
    );
    assert!(app.chat_input_is_draft);
    // THE EDIT FLIPS THE METHOD (the onEdited drop the next close
    // answers — the slash-less edit... the slash RIDES: the text
    // still starts with '/', the COMMAND method the startsWith
    // walk keeps)
    app.on_chat_backspace();
    assert_eq!(app.chat_input, "/time set da");
    app.close_chat();
    assert_eq!(
        app.chat_latest_draft,
        Some(("/time set da".to_string(), true)),
        "the startsWith walk the command method keeps"
    );
    // THE MESSAGE DRAFT THE SLASH LANE NEVER RESTORES (the $2
    // equality arm — the message draft the prefix arm answers:
    // the bare '/' the fresh open wears, the purity the false).
    // The T-open the '/time set da' draft restores — the clear
    // walk the box empties (the edit the purity drops), the bare
    // words the fresh text wears)
    app.on_chat_key();
    while !app.chat_input.is_empty() {
        app.on_chat_backspace();
    }
    for c in "just words".chars() {
        app.on_chat_char(c);
    }
    app.close_chat();
    assert_eq!(
        app.chat_latest_draft,
        Some(("just words".to_string(), false)),
        "the bare close the message draft banks"
    );
    app.on_command_key();
    assert_eq!(
        app.chat_input, "/",
        "the message draft the slash lane never restores"
    );
    assert!(
        !app.chat_input_is_draft,
        "the prefix arm the purity never banks"
    );
    // THE MESSAGE DRAFT THE T LANE RESTORES (the $1 arm)
    app.on_chat_key();
    assert_eq!(app.chat_input, "just words");
    assert!(app.chat_input_is_draft);
}

/// THE SUBMIT'S OWN DONE DISCARD (javap removed()'s
/// shouldDiscardDraft walk — the DONE exit ALWAYS discards: the
/// sent message the bank never wears, the next open the fresh
/// empty box carries; the empty submit the same DONE lane keeps).
#[test]
fn the_submit_the_done_discard_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    // THE SENT MESSAGE (the DONE exit — the close's save the same
    // walk discards)
    app.on_chat_key();
    for c in "hi there".chars() {
        app.on_chat_char(c);
    }
    app.on_chat_submit();
    assert!(!app.chat_open);
    assert_eq!(
        app.chat_latest_draft, None,
        "the submit the DONE discard keeps"
    );
    app.on_chat_key();
    assert_eq!(app.chat_input, "", "the next open the fresh box wears");
    assert!(!app.chat_input_is_draft);
    // THE EMPTY SUBMIT (the normalize's empty the silent close
    // answers — the same DONE discard the blank law keeps)
    app.on_chat_submit();
    assert!(!app.chat_open);
    assert_eq!(app.chat_latest_draft, None);
}

/// THE UNTOUCHED DRAFT'S OWN ESC-CLEAR (javap ChatScreen.keyPressed's
/// isDraft arm — scripts/jar_work/chatscreen_49.txt: the FIRST ESC
/// the `input.setValue("")` walk answers, the screen the OPEN
/// stays, the purity the isDraft=0 drop wears; the SECOND ESC the
/// close lane answers (the blank the discard keeps, the bank the
/// fresh-open law clears); the typed words the edited close
/// re-banks — the real client's two-step dismiss the same walk
/// plays).
#[test]
fn the_untouched_draft_the_esc_clear_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_chat_key();
    for c in "banked".chars() {
        app.on_chat_char(c);
    }
    app.close_chat();
    // THE RESTORED OPEN + THE FIRST ESC (the isDraft arm — the
    // input the empty value wears, the screen the open stays, the
    // bank the untouched record keeps)
    app.on_chat_key();
    assert_eq!(app.chat_input, "banked");
    assert!(app.chat_input_is_draft);
    let consumed = app.on_chat_escape();
    assert!(consumed, "the chat ESC the consumption keeps");
    assert!(app.chat_open, "the first ESC the screen keeps");
    assert_eq!(app.chat_input, "", "the first ESC the box clears");
    assert!(!app.chat_input_is_draft, "the clear the purity drops");
    assert_eq!(
        app.chat_latest_draft,
        Some(("banked".to_string(), false)),
        "the first ESC the bank never touches"
    );
    // THE TYPED WORDS + THE SECOND ESC (the edited close the save
    // lane answers — the new text the bank re-wears)
    for c in "new".chars() {
        app.on_chat_char(c);
    }
    app.on_chat_escape();
    assert!(!app.chat_open, "the second ESC the close answers");
    assert_eq!(
        app.chat_latest_draft,
        Some(("new".to_string(), false)),
        "the edited close the save lane banks"
    );
    // THE REOPEN (the restored draft the same two-step plays)
    app.on_chat_key();
    assert_eq!(app.chat_input, "new");
    assert!(app.chat_input_is_draft);
    app.on_chat_escape();
    assert!(app.chat_open, "the restored draft the first ESC keeps");
    assert_eq!(app.chat_input, "");
    app.on_chat_escape();
    assert!(!app.chat_open, "the cleared box the second ESC closes");
    assert_eq!(
        app.chat_latest_draft, None,
        "the blank close the discard keeps"
    );
}

/// THE INTERRUPTED DEATH'S OWN DRAFT BANK (javap ChatScreen's exit
/// verdict — scripts/jar_work/chatscreen_49.txt the ctor's
/// `exitReason = INTERRUPTED` default wears: the death screen's
/// own setScreen the old ChatScreen's removed() answers with the
/// verdict still INTERRUPTED, the shouldDiscardDraft the FALSE
/// keeps — the typed text the saveAsDraft lane banks exactly like
/// the ESC-ON close, the respawn's T the restored draft wears).
#[test]
fn the_interrupted_death_the_draft_banks_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    // THE MID-TYPE DEATH (the chat the open stays, the words the
    // input rides, the fatal dose the death screen opens)
    app.on_chat_key();
    for c in "see you".chars() {
        app.on_chat_char(c);
    }
    assert!(app.chat_open);
    app.actually_hurt(100.0, HurtVoice::Plain, None, None);
    // THE INTERRUPTED WALK (the close the save lane banks — the
    // method the startsWith walk names, the chat the death overlay
    // the input never shares)
    assert!(app.dead, "the dose the death arms");
    assert!(
        !app.chat_open,
        "the death the chat screen folds (the setScreen removed walk)"
    );
    assert_eq!(
        app.chat_latest_draft,
        Some(("see you".to_string(), false)),
        "the INTERRUPTED verdict the draft banks"
    );
    assert_eq!(app.chat_input, "", "the removed walk the box folds");
    // THE RESPAWN'S OWN RESTORE (the next T the restorable open
    // answers — the banked draft the isDraft purity wears)
    app.respawn();
    app.on_chat_key();
    assert_eq!(app.chat_input, "see you", "the reopen the draft restores");
    assert!(
        app.chat_input_is_draft,
        "the restored draft the purity wears"
    );
}

/// THE INTERRUPTED BLANK'S OWN DISCARD (the removed() walk's
/// isBlank arm — the whitespace-only input the discardDraft lane
/// keeps, the bank the fresh-open law clears: the interrupted
/// close the blank draft NEVER banks).
#[test]
fn the_interrupted_the_blank_discards_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    // THE BANKED PAST (the earlier close the draft wears — the
    // blank interruption the record CLEARS, the isBlank arm)
    app.on_chat_key();
    for c in "old".chars() {
        app.on_chat_char(c);
    }
    app.close_chat();
    assert!(app.chat_latest_draft.is_some());
    // THE WHITESPACE OPEN + THE DEATH (the restored draft the
    // first ESC clears — the untouched-draft dismiss the box
    // empties, the typed spaces the BLANK input wear, the isBlank
    // arm the discard keeps)
    app.on_chat_key();
    assert!(app.chat_input_is_draft, "the restore the purity wears");
    app.on_chat_escape();
    assert!(app.chat_open, "the first ESC the screen keeps");
    for c in "   ".chars() {
        app.on_chat_char(c);
    }
    app.actually_hurt(100.0, HurtVoice::Plain, None, None);
    assert!(app.dead);
    assert!(!app.chat_open);
    assert_eq!(
        app.chat_latest_draft, None,
        "the isBlank walk the discard keeps"
    );
}

/// THE INTERRUPTED UNTOUCHED DRAFT'S OWN BANK-KEEP (the removed()
/// walk's !isDraft gate — the restored draft the death replaces
/// BEFORE any keystroke: the skip-save the SAME banked record
/// keeps, the purity the close consumed, the reopen the same text
/// the isDraft walk restores again).
#[test]
fn the_interrupted_the_untouched_draft_keeps_its_bank() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_chat_key();
    for c in "banked".chars() {
        app.on_chat_char(c);
    }
    app.close_chat();
    // THE RESTORED OPEN (the isDraft purity) + THE DEATH (the
    // untouched draft the skip-save keeps — the bank unchanged)
    app.on_chat_key();
    assert!(app.chat_input_is_draft);
    app.actually_hurt(100.0, HurtVoice::Plain, None, None);
    assert!(app.dead);
    assert!(!app.chat_open);
    assert_eq!(
        app.chat_latest_draft,
        Some(("banked".to_string(), false)),
        "the skip-save the banked record keeps"
    );
    // THE REOPEN (the same draft the isDraft walk restores)
    app.on_chat_key();
    assert_eq!(app.chat_input, "banked");
    assert!(app.chat_input_is_draft);
}

/// THE DEATH SCREEN'S OWN MODAL (the vanilla screen!=null verdict —
/// the DeathScreen the input OWNS: the ESC walk the on_escape's
/// death gate keeps paused-never (the Respawn/Title buttons the
/// only exits), the window's T// routing the same modal law wears
/// (the !app.dead gate the handleKeybinds lane keeps — the chat
/// over the death overlay never opens); the respawn the world
/// lanes restore).
#[test]
fn the_death_screen_the_modal_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    app.actually_hurt(100.0, HurtVoice::Plain, None, None);
    assert!(app.dead && !app.chat_open);
    // THE DEAD ESC (the on_escape death gate — the pause the modal
    // never arms, the screen the World keeps)
    app.on_escape();
    assert!(!app.paused, "the death screen the ESC never pauses");
    assert_eq!(app.screen, ScreenKind::World);
    // THE RESPAWN'S OWN RESTORE (the ESC the pause lane answers
    // again — the world the keys own)
    app.respawn();
    app.on_escape();
    assert!(app.paused, "the respawn the ESC pause restores");
    app.paused = false;
}

/// THE COMMAND DRAFT'S OWN INTERRUPTED BANK (the same removed()
/// walk the '/'-prefixed input the COMMAND method banks — the
/// startsWith walk the method names, the slash reopen the command
/// draft restores: the death mid-command the text never burns).
#[test]
fn the_interrupted_the_command_draft_banks_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_key();
    assert_eq!(app.chat_input, "/", "the prefix the command lane wears");
    for c in "tp 1 2 3".chars() {
        app.on_chat_char(c);
    }
    app.actually_hurt(100.0, HurtVoice::Plain, None, None);
    assert!(app.dead && !app.chat_open);
    assert_eq!(
        app.chat_latest_draft,
        Some(("/tp 1 2 3".to_string(), true)),
        "the startsWith walk the COMMAND draft banks"
    );
    // THE SLASH REOPEN (the $2 arm — the command draft the command
    // lane restores, the purity wears)
    app.respawn();
    app.on_command_key();
    assert_eq!(app.chat_input, "/tp 1 2 3");
    assert!(app.chat_input_is_draft);
}

/// THE SCREEN TAKEOVER'S OWN INTERRUPTED WALK (the vanilla
/// setScreen's removed() lane the open() hook serves — the
/// server-driven chest open the open chat REPLACES: the typed
/// text the draft bank the interruption saves, the container
/// screen the typing lane never shares, the close the world the
/// chat keeps folded).
#[test]
fn the_open_takeover_the_interrupted_walk_speaks() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_chat_key();
    for c in "still here".chars() {
        app.on_chat_char(c);
    }
    // THE TAKEOVER (the chest screen the chat replaces — the old
    // ChatScreen's removed() the INTERRUPTED default wears)
    app.open(ScreenKind::Chest);
    assert!(!app.chat_open, "the takeover the chat folds");
    assert_eq!(
        app.chat_latest_draft,
        Some(("still here".to_string(), false)),
        "the interrupted takeover the draft banks"
    );
    // THE REOPEN (the world back the draft restores)
    app.open(ScreenKind::World);
    app.on_chat_key();
    assert_eq!(app.chat_input, "still here");
    assert!(app.chat_input_is_draft);
}

/// THE COMBAT KILL'S OWN INTERRUPTED WALK (the wire lane —
/// handlePlayerCombatKill's setScreen(new DeathScreen) the
/// live_health_updated hook serves: the death message's arrival
/// the open chat's removed() answers, the INTERRUPTED default the
/// draft bank saves; the alive health packet the open chat the
/// same hook never touches — the close the death crossing alone
/// speaks).
#[test]
fn the_combat_kill_the_wire_walk_banks() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_chat_key();
    for c in "wire words".chars() {
        app.on_chat_char(c);
    }
    // THE ALIVE PACKET (the hook the open chat never touches — the
    // close the death crossing alone answers)
    app.live_health_updated(crate::world_view::HealthView {
        health: 6.0,
        food: 20,
        saturation: 0.0,
        hurt_time: 0,
        hurt_yaw: 0.0,
        death_message: None,
        effects: Vec::new(),
        mining_efficiency: 0.0,
        block_break_speed: 1.0,
        submerged_mining_speed: 0.2,
    });
    assert!(app.chat_open, "the alive packet the chat screen keeps");
    // THE KILL (the death message the setScreen speaks — the
    // removed() walk the draft banks)
    app.live_health_updated(crate::world_view::HealthView {
        health: 0.0,
        food: 20,
        saturation: 0.0,
        hurt_time: 0,
        hurt_yaw: 0.0,
        death_message: Some("fell from a high place".to_string()),
        effects: Vec::new(),
        mining_efficiency: 0.0,
        block_break_speed: 1.0,
        submerged_mining_speed: 0.2,
    });
    assert!(app.dead, "the kill the death arms");
    assert!(!app.chat_open, "the kill the chat screen folds");
    assert_eq!(
        app.chat_latest_draft,
        Some(("wire words".to_string(), false)),
        "the combat kill's interrupted close the draft banks"
    );
}

/// THE OPTION'S OWN OFF ARM (the vanilla saveChatDrafts
/// OptionInstance the shouldDiscardDraft's INTENTIONAL arm reads —
/// the OFF the ESC close the discardDraft walk speaks: the bank
/// the typed text REGARDLESS clears, the reopen the fresh box
/// wears; the ON default the save lane keeps — the same removed()
/// walk the verdicts fork).
#[test]
fn the_option_off_the_esc_close_discards_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    // THE OFF OPTION + THE TYPED ESC CLOSE (the INTENTIONAL
    // verdict's TRUE arm — the bank the discardDraft clears)
    app.chat_save_drafts = false;
    app.on_chat_key();
    for c in "forgotten".chars() {
        app.on_chat_char(c);
    }
    app.on_chat_escape();
    assert!(!app.chat_open, "the ESC close the screen folds");
    assert_eq!(
        app.chat_latest_draft, None,
        "the OFF verdict the bank discards"
    );
    // THE REOPEN (the fresh box the discard law wears — no restore)
    app.on_chat_key();
    assert_eq!(app.chat_input, "", "the reopen the fresh box wears");
    assert!(!app.chat_input_is_draft);
    // THE ON DEFAULT + THE SAME WALK (the save lane the option's
    // ON verdict answers — the SAME removed() walk the fork)
    app.close_chat();
    app.chat_save_drafts = true;
    app.on_chat_key();
    for c in "remembered".chars() {
        app.on_chat_char(c);
    }
    app.on_chat_escape();
    assert_eq!(
        app.chat_latest_draft,
        Some(("remembered".to_string(), false)),
        "the ON verdict the save lane banks"
    );
}

/// THE OPTION'S OWN INTERRUPTED IMMUNITY (the dev-53 law the OFF
/// option never touches — the shouldDiscardDraft's INTERRUPTED
/// arm the FALSE keeps REGARDLESS of the option: the death close
/// the draft banks even when the ESC close would discard; the
/// option the INTENTIONAL lane ALONE gates).
#[test]
fn the_option_off_the_interrupted_still_banks_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    app.chat_save_drafts = false;
    app.on_chat_key();
    for c in "survives".chars() {
        app.on_chat_char(c);
    }
    // THE DEATH (the INTERRUPTED verdict — the option the lane
    // never reads, the draft the replacement close banks)
    app.actually_hurt(100.0, HurtVoice::Plain, None, None);
    assert!(app.dead && !app.chat_open);
    assert_eq!(
        app.chat_latest_draft,
        Some(("survives".to_string(), false)),
        "the INTERRUPTED lane the option never gates"
    );
    // THE SUBMIT'S OWN DONE (the always-discard the option never
    // touches — the OFF arm the DONE verdict never reads; the
    // restored draft the first ESC clears, the fresh words the
    // submit sends)
    app.respawn();
    app.on_chat_key();
    assert_eq!(app.chat_input, "survives", "the bank the restore wears");
    app.on_chat_escape();
    for c in "sent".chars() {
        app.on_chat_char(c);
    }
    app.on_chat_submit();
    assert_eq!(
        app.chat_latest_draft, None,
        "the DONE verdict the always-discard keeps"
    );
}

/// THE F KEY'S OWN OFFHAND TRADE (javap handleKeybinds's
/// keySwapOffhand arm — scripts/jar_work/handlekeybinds_48.txt pcs
/// 375-414 — the OFFLINE lane: the selected hotbar stack and the
/// offhand the ONE trade answers, the hand's projection the held
/// reload walk answers, the empty trade the honest bare arm keeps).
#[test]
fn the_f_key_the_offhand_swap_trades() {
    let mut app = App::new(3);
    app.enter_world();
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let shield = mc_registry::item_id_by_name("shield").unwrap();
    app.inventory.items[0] = (sword, 1);
    app.inventory.offhand[0] = (shield, 1);
    assert_eq!(app.held_id(), Some(sword), "the hand the selection speaks");
    // THE TRADE (the selected stack the offhand keeps, the offhand
    // the hand wears — the held model the reload walk answers)
    app.on_swap_offhand_key();
    assert_eq!(
        app.inventory.items[0],
        (shield, 1),
        "the offhand the hand wears"
    );
    assert_eq!(
        app.inventory.offhand[0],
        (sword, 1),
        "the hand the offhand keeps"
    );
    assert_eq!(
        app.held_id(),
        Some(shield),
        "the hand's projection the trade follows"
    );
    // THE RE-SWAP (the one trade re-answers — the repeated press
    // the same walk plays)
    app.on_swap_offhand_key();
    assert_eq!(app.inventory.items[0], (sword, 1));
    assert_eq!(app.inventory.offhand[0], (shield, 1));
    // THE EMPTY HAND'S OWN TRADE (the empty selection the offhand
    // the hand WEARS — the shield the offhand slot keeps, the
    // empty the trade banks)
    app.selected_hotbar = 4;
    app.on_swap_offhand_key();
    assert_eq!(
        app.inventory.items[4],
        (shield, 1),
        "the offhand the empty hand wears"
    );
    assert_eq!(
        app.inventory.offhand[0],
        (0, 0),
        "the empty the trade banks"
    );
    assert_eq!(
        app.held_id(),
        Some(shield),
        "the shield the hand's projection wears"
    );
}

/// THE F KEY'S OWN GATES (the jar lane's own verdicts the walk
/// keeps: the container screens the handleKeybinds's screen==null
/// guard never passes, the spectator the isSpectator verdict
/// skips, the death the modal keeps — the inventory the gates
/// never trade).
#[test]
fn the_f_key_the_gates_keep() {
    let mut app = App::new(3);
    app.enter_world();
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let shield = mc_registry::item_id_by_name("shield").unwrap();
    app.inventory.items[0] = (sword, 1);
    app.inventory.offhand[0] = (shield, 1);
    // THE CONTAINER GATE (the inventory screen the key never reads)
    app.open_inventory();
    app.on_swap_offhand_key();
    assert_eq!(
        app.inventory.items[0],
        (sword, 1),
        "the container gate the trade keeps"
    );
    app.close_inventory(ScreenKind::World);
    // THE SPECTATOR GATE (the jar lane's own isSpectator verdict —
    // the swap the spectator mode never answers)
    app.game_mode = GameMode::Spectator;
    app.on_swap_offhand_key();
    assert_eq!(
        app.inventory.items[0],
        (sword, 1),
        "the spectator the swap skips"
    );
    app.game_mode = GameMode::Survival;
    // THE DEATH GATE (the DeathScreen modal the world keys keep)
    app.dead = true;
    app.on_swap_offhand_key();
    assert_eq!(
        app.inventory.items[0],
        (sword, 1),
        "the death the modal keeps"
    );
    app.dead = false;
    // THE SURVIVOR's OWN TRADE (the gates open — the one trade the
    // lane answers)
    app.on_swap_offhand_key();
    assert_eq!(
        app.inventory.items[0],
        (shield, 1),
        "the survivor the trade answers"
    );
    assert_eq!(app.inventory.offhand[0], (sword, 1));
}

/// THE LIVE F KEY'S OWN STORE AND WIRE (the live lane: the window-0
/// belief the same in-place contract keeps — the hotbar row 36..45
/// and the offhand 45 the ONE trade answers, the hand's reload the
/// id change speaks, the SwapOffhand command the worker's packet
/// send rides).
#[test]
fn the_live_f_key_the_store_and_wire_speak() {
    let mut app = App::new(3);
    app.enter_world();
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let shield = mc_registry::item_id_by_name("shield").unwrap();
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    // THE WIRE-0 BELIEF (the hotbar row's slot 0 the sync spoke,
    // the offhand 45 the sync spoke — the full store's shape)
    app.live_inv_slots[36] = (sword, 1);
    app.live_inv_slots[45] = (shield, 1);
    app.live_held_updated(Some((sword, 1)));
    assert_eq!(app.live_held, Some(sword), "the hand the selection wears");
    // THE TRADE (the belief the in-place contract swaps, the hand
    // the reload walk answers, the command the channel rides)
    app.on_swap_offhand_key();
    assert_eq!(
        app.live_inv_slots[36],
        (shield, 1),
        "the offhand the hand wears"
    );
    assert_eq!(
        app.live_inv_slots[45],
        (sword, 1),
        "the hand the offhand keeps"
    );
    assert_eq!(
        app.live_held,
        Some(shield),
        "the hand's reload the id change speaks"
    );
    match rx.try_recv().expect("the command rode the channel") {
        crate::world_view::ContainerCommand::SwapOffhand => {}
        other => panic!("the wrong command rode the channel: {other:?}"),
    }
}

/// THE OFFHAND LANE'S OWN RELOAD (the OFF_HAND call's client half —
/// the lazy refresh the first-person pass speaks: the id-change key
/// the same law `live_held` wears, the gameplay offhand slot the
/// offline lane reads, the F swap's own mutation the SAME frame's
/// projection answers; the `--offhand-item` stand-in the empty
/// gameplay slot backs).
#[test]
fn the_offhand_lane_the_reload_and_swap_speak() {
    let mut app = App::new(3);
    app.enter_world();
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let shield = mc_registry::item_id_by_name("shield").unwrap();
    // THE EMPTY OFFHAND (the bare LEFT arm the draw lane renders —
    // no payload, no reload key)
    app.offhand_held_refreshed();
    assert!(
        app.held_offhand.is_none(),
        "the empty offhand the bare arm wears"
    );
    assert_eq!(app.offhand_held_loaded, None);
    // THE SHIELD LANDS (the gameplay offhand slot the pickup walk
    // arms — the refresh the payload loads)
    app.inventory.offhand[0] = (shield, 1);
    app.offhand_held_refreshed();
    assert!(app.held_offhand.is_some(), "the shield the left lane bakes");
    assert_eq!(app.offhand_held_loaded, Some(shield));
    // THE CADENCE (the unchanged id the key keeps — no re-bake)
    app.offhand_held_refreshed();
    assert_eq!(app.offhand_held_loaded, Some(shield));
    // THE F SWAP (the shield the hand wears, the sword the offhand
    // keeps — the next refresh the new id bakes)
    app.inventory.items[0] = (sword, 1);
    app.on_swap_offhand_key();
    assert_eq!(app.inventory.offhand[0], (sword, 1));
    app.offhand_held_refreshed();
    assert_eq!(app.offhand_held_loaded, Some(sword));
    assert!(app.held_offhand.is_some());
    // THE STAND-IN (the --offhand-item vocabulary the empty gameplay
    // slot backs — the offline scenes' own lane)
    app.inventory.offhand[0] = (0, 0);
    app.offline_offhand_updated(Some("item/shield"));
    assert_eq!(app.offhand_stack(), (shield, 1));
    assert_eq!(app.offhand_held_loaded, Some(shield));
}

/// THE LIVE OFFHAND'S OWN BELIEF (the window-0 store's slot 45 the
/// offhand lane reads — the server's slot echo the belief updates,
/// the refresh the same id-change key keeps).
#[test]
fn the_live_offhand_the_store_belief_speaks() {
    let mut app = App::new(3);
    app.enter_world();
    let bow = mc_registry::item_id_by_name("bow").unwrap();
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.live_inv_slots[45] = (bow, 1);
    app.offhand_held_refreshed();
    assert!(
        app.held_offhand.is_some(),
        "the live offhand the left lane bakes"
    );
    assert_eq!(app.offhand_held_loaded, Some(bow));
}

/// THE OFF-HAND'S OWN RMB (the vanilla for-each-hand walk the second
/// turn speaks: the sword's PASS the offhand shield's CONSUME answers
/// — the blocking payload re-bakes through the LEFT lane, the use
/// the OFF hand rides, the release the resting shield returns).
#[test]
fn the_offhand_shield_the_rmb_raises_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    let shield = mc_registry::item_id_by_name("shield").unwrap();
    // the spawn band's air (the terrain's own tops never reach) —
    // the level look the block lane never clips
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    // the sword the stand-in route bakes (the payload the reload
    // walk loads — the gameplay inventory the press's own gate
    // reads through the same unified id)
    app.offline_held_updated(Some("iron_sword"));
    let main_rest = app.held_item.as_ref().expect("the sword loads");
    let main_rest_centroid = main_rest.centroid();
    app.inventory.offhand[0] = (shield, 1);
    app.offhand_held_refreshed();
    let resting = app.held_offhand.as_ref().expect("the resting shield loads");
    let rest_centroid = resting.centroid();
    // THE PRESS: the sword's PASS the offhand's turn answers
    assert!(app.on_right_press(0, 0), "the offhand the use arms");
    let active = app.using_item.as_ref().expect("the use rides");
    assert_eq!(active.hand, UseHand::Off, "the OFF hand the use wears");
    assert_eq!(active.item, shield, "the shield the use carries");
    let blocking = app.held_offhand.as_ref().expect("the blocking payload");
    let block_centroid = blocking.centroid();
    assert!(
        (block_centroid[0] - rest_centroid[0]).abs() > 0.01
            || (block_centroid[1] - rest_centroid[1]).abs() > 0.01,
        "the blocking stanza MOVES the offhand ({rest_centroid:?} -> {block_centroid:?})"
    );
    // THE MAIN HAND UNTOUCHED (the sword the resting lane keeps —
    // the main payload the offhand's use never re-bakes)
    let main_now = app
        .held_item
        .as_ref()
        .expect("the sword the main lane keeps");
    let main_now_centroid = main_now.centroid();
    assert!(
        (main_now_centroid[0] - main_rest_centroid[0]).abs() < 1e-4
            && (main_now_centroid[1] - main_rest_centroid[1]).abs() < 1e-4,
        "the main rest pose the offhand's use never moves"
    );
    // THE HOLD: the single-state law the ticks never churn (no
    // reload — the pose the hold keeps)
    for _ in 0..10 {
        app.update();
    }
    assert!(app.is_using(), "the hold the use keeps");
    // THE RELEASE: the resting shield returns
    app.on_right_release();
    assert!(!app.is_using(), "the release the use ends");
    let back = app.held_offhand.as_ref().expect("the rest returns");
    let back_centroid = back.centroid();
    assert!(
        (back_centroid[0] - rest_centroid[0]).abs() < 1e-4
            && (back_centroid[1] - rest_centroid[1]).abs() < 1e-4,
        "the rest pose the release returns"
    );
}

/// THE MAIN HAND FIRST (the vanilla hand-order law: the MAIN
/// shield's CONSUME the press returns — the off hand the walk never
/// reaches, one use at a time the using field's own law keeps).
#[test]
fn the_offhand_the_main_consumes_first_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    let shield = mc_registry::item_id_by_name("shield").unwrap();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    app.offline_held_updated(Some("shield"));
    app.inventory.offhand[0] = (shield, 1);
    app.offhand_held_refreshed();
    let main_rest = app.held_item.as_ref().expect("the main shield loads");
    let main_rest_centroid = main_rest.centroid();
    let off_rest = app.held_offhand.as_ref().expect("the off shield loads");
    let off_rest_centroid = off_rest.centroid();
    assert!(app.on_right_press(0, 0), "the main shield the use arms");
    let active = app.using_item.as_ref().expect("the use rides");
    assert_eq!(active.hand, UseHand::Main, "the MAIN turn the order speaks");
    let main_block = app.held_item.as_ref().expect("the main blocking");
    let main_block_centroid = main_block.centroid();
    assert!(
        (main_block_centroid[0] - main_rest_centroid[0]).abs() > 0.01
            || (main_block_centroid[1] - main_rest_centroid[1]).abs() > 0.01,
        "the main shield the blocking pose wears"
    );
    let off_now = app.held_offhand.as_ref().expect("the off keeps");
    let off_now_centroid = off_now.centroid();
    assert!(
        (off_now_centroid[0] - off_rest_centroid[0]).abs() < 1e-4
            && (off_now_centroid[1] - off_rest_centroid[1]).abs() < 1e-4,
        "the offhand the rest keeps — the MAIN turn the walk consumed"
    );
    app.on_right_release();
    assert!(!app.is_using());
}

/// THE MAIN HAND'S OWN FAIL THE OFFHAND ANSWERS (the vanilla
/// consumesAction law: the full food's canEat FAIL the walk keeps
/// walking — the OFF hand's shield the turn speaks, the eat lane
/// silent, the OFF_HAND ordinal the wire rides).
#[test]
fn the_offhand_the_main_fail_falls_through_keeps() {
    use crate::world_view::ContainerCommand;
    let wire = |name: &str| {
        mc_registry::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == name)
            .expect("the item rides the wire") as i32
    };
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(tx);
    // the FULL food bar the canEat gate refuses (needs_food false)
    app.live_held_updated(Some((wire("cooked_beef"), 1)));
    app.live_inv_slots[45] = (wire("shield"), 1);
    app.offhand_held_refreshed();
    assert!(app.on_right_press(0, 0), "the offhand the FAIL answers");
    assert!(app.eating.is_none(), "the eat the FAIL keeps silent");
    let active = app.using_item.as_ref().expect("the offhand use rides");
    assert_eq!(
        active.hand,
        UseHand::Off,
        "the OFF turn the fall-through arms"
    );
    match rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::UseItem { yaw, pitch, hand } => {
            assert_eq!(hand, 1, "the OFF_HAND ordinal the wire speaks");
            assert_eq!(yaw, -90.0, "the vanilla yRot");
            assert!((pitch - 4.583).abs() < 0.01, "the vanilla xRot");
        }
        other => panic!("expected a UseItem echo, got {other:?}"),
    }
}

/// THE EMPTY MAIN'S OWN HAND-ORDER (the vanilla walk the empty
/// stack skips: the MAIN useItem never runs, the OFF hand's shield
/// the use arms — the wire the OFF_HAND ordinal carries).
#[test]
fn the_live_offhand_the_wire_hand_speaks() {
    use crate::world_view::ContainerCommand;
    let wire = |name: &str| {
        mc_registry::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == name)
            .expect("the item rides the wire") as i32
    };
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(tx);
    // the empty main (no live_held, no offline stack) the MAIN turn
    // skips — the OFF hand's shield the first use arms
    app.live_inv_slots[45] = (wire("shield"), 1);
    app.offhand_held_refreshed();
    assert!(app.on_right_press(0, 0), "the offhand the empty main arms");
    let active = app.using_item.as_ref().expect("the use rides");
    assert_eq!(active.hand, UseHand::Off);
    match rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::UseItem { hand, .. } => {
            assert_eq!(hand, 1, "the OFF_HAND ordinal");
        }
        other => panic!("expected a UseItem echo, got {other:?}"),
    }
}

/// THE USE THE STACK OUTLIVES NOT (the vanilla releaseUsingItem on
/// the item switch — the offhand reload key the id change speaks,
/// the OFF use ends, the new stack the LEFT lane bakes).
#[test]
fn the_offhand_use_the_stack_outlives_not_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let shield = mc_registry::item_id_by_name("shield").unwrap();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.live_inv_slots[45] = (shield, 1);
    app.offhand_held_refreshed();
    assert!(app.on_right_press(0, 0), "the offhand the use arms");
    assert_eq!(app.using_item.as_ref().unwrap().hand, UseHand::Off);
    // THE STACK CHANGE: the store's slot the refresh walks — the
    // switch the OFF use ends (the hold cannot outlive its stack)
    app.live_inv_slots[45] = (sword, 1);
    app.offhand_held_refreshed();
    assert!(app.using_item.is_none(), "the switch the use ends");
    assert_eq!(
        app.offhand_held_loaded,
        Some(sword),
        "the sword the lane bakes"
    );
    // the release after the switch: the use already gone — no
    // stale restore, no panic (the honest no-op the release keeps)
    app.on_right_release();
    assert_eq!(
        app.offhand_held_loaded,
        Some(sword),
        "the sword the lane keeps"
    );
}

/// THE OFFHAND'S OWN EAT ARC (the vanilla use()'s consumable arm
/// the OFF hand's turn answers — the empty main the MAIN turn
/// skips, the offhand's apple the hungry body the hold arms: the
/// EatActive the OFF hand wears (the usedItemHand record), the
/// 32-tick hold the same clock walks, the completion the FoodData
/// fold speaks, and the CONSUME the USED stack's own slot answers
/// — the offhand's stack the shrink wears, the hotbar's stacks the
/// arc never touches).
#[test]
fn the_offhand_food_the_eat_arc_consumes_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    let apple = mc_registry::item_id_by_name("apple").unwrap();
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    app.inventory.offhand[0] = (apple, 3);
    // the NON-food main (the sword's use the PASS keeps — the main
    // turn the fall-through verdict speaks, the OFF turn answers)
    app.inventory.items[app.selected_hotbar] = (sword, 1);
    // the camera the sky reads (the block lane the miss keeps —
    // the dev-52 tests' own determinism the walk wears)
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    // the hungry body: the bar under 20 (the canEat's own gate)
    app.food_data.set_food_level(10);
    app.food_data.set_saturation(0.0);
    // THE PRESS (the main's sword the PASS verdict speaks — the
    // OFF turn the eat arms)
    assert!(app.on_right_press(0, 0), "the offhand food the hold arms");
    let eat = app.eating.as_ref().expect("the hold opened");
    assert_eq!(eat.hand, UseHand::Off, "the OFF hand the eat wears");
    assert_eq!(eat.item, apple, "the offhand's own stack the hold rides");
    assert!(app.using_item.is_none(), "one use at a time");
    // THE FULL ARC (the 32 ticks the completion folds)
    for _ in 0..32 {
        app.update();
    }
    assert!(app.eating.is_none(), "the hold completed");
    assert_eq!(
        app.food_data.food_level(),
        14,
        "the FoodData the fold feeds"
    );
    // THE CONSUME THE USED STACK'S SLOT ANSWERS (the offhand's
    // stack 3 -> 2, the main's sword the arc never touches)
    assert_eq!(app.inventory.offhand[0], (apple, 2), "the OFF consume");
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (sword, 1),
        "the main lane's stack untouched"
    );
}

/// THE OFFHAND'S OWN GATES (the canConsume verdict the OFF turn's
/// last-word law keeps — the full bar the plain food the press's
/// silence answers; the food in BOTH hands the hungry body the
/// MAIN turn consumes first — the hand-order the vanilla walk
/// speaks, the offhand's stack the main's eat never touches).
#[test]
fn the_offhand_food_the_gates_keep() {
    let mut app = App::new(3);
    app.enter_world();
    let bread = mc_registry::item_id_by_name("bread").unwrap();
    let apple = mc_registry::item_id_by_name("apple").unwrap();
    app.inventory.offhand[0] = (bread, 1);
    // the camera the sky reads (the block lane the miss keeps)
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    // THE FULL BAR (needs_food false, canAlwaysEat false — the
    // canConsume FAIL the OFF turn answers, the press silent)
    assert!(
        !app.on_right_press(0, 0),
        "the full bar the plain food refuses"
    );
    assert!(app.eating.is_none(), "the FAIL the lane keeps silent");
    // THE FOOD IN BOTH HANDS (the MAIN turn consumes first — the
    // main's apple the hold opens, the offhand's bread the walk
    // never reaches)
    app.inventory.items[app.selected_hotbar] = (apple, 1);
    app.food_data.set_food_level(10);
    assert!(app.on_right_press(0, 0), "the main's food consumes first");
    let eat = app.eating.as_ref().expect("the main hold opened");
    assert_eq!(eat.hand, UseHand::Main, "the MAIN hand the eat wears");
    assert_eq!(eat.item, apple, "the main's own stack the hold rides");
    assert_eq!(
        app.inventory.offhand[0],
        (bread, 1),
        "the offhand's stack untouched"
    );
}

/// THE OFFHAND EAT'S OWN WIRE (the live lane — the OFF_HAND
/// ordinal the use-item echo speaks: the empty main the MAIN turn
/// skips, the offhand's apple the echo's hand byte 1 carries, the
/// camera's angles the vanilla degrees wear).
#[test]
fn the_offhand_food_the_wire_hand_speaks() {
    use crate::world_view::ContainerCommand;
    let wire = |name: &str| {
        mc_registry::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == name)
            .expect("the item rides the wire") as i32
    };
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(tx);
    // the empty main + the offhand's apple + the hungry belief
    app.live_inv_slots[45] = (wire("apple"), 3);
    app.offhand_held_refreshed();
    app.food_data.set_food_level(10);
    assert!(app.on_right_press(0, 0), "the offhand food the hold arms");
    let eat = app.eating.as_ref().expect("the hold opened");
    assert_eq!(eat.hand, UseHand::Off);
    match rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::UseItem { hand, .. } => {
            assert_eq!(hand, 1, "the OFF_HAND ordinal the wire speaks");
        }
        other => panic!("expected a UseItem echo, got {other:?}"),
    }
}

/// THE SPEAR'S OWN STAB (the 26.2 startAttack's piercing branch —
/// the main-hand spear the left press the STAB Player Action
/// answers INSTEAD of the hit walk: the swing the MAIN_HAND call
/// carries, the ticker the onAttack's reset resets, the wire the
/// Stab command rides, and the DIG the branch never arms — the
/// block lane the hit walk serves the press never reaches).
#[test]
fn the_spear_left_press_the_stab_speaks_keeps() {
    use crate::world_view::ContainerCommand;
    let mut app = App::new(3);
    app.enter_world();
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // the camera the sky reads (the block lane the miss keeps —
    // the stab branch's own verdict the press alone answers)
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(tx);
    // THE SPEAR PRESS (the stab branch the hit walk replaces —
    // the ticker the iron spear's OWN full charge wears: the
    // Properties.spear modifier 1.0/0.95 the 19-tick delay keeps,
    // the cannotAttackWithItem gate the not-yet-full press
    // refuses)
    app.attack_ticker = 19;
    app.on_attack();
    assert!(app.swing.swinging, "the stab the swing carries");
    assert_eq!(
        app.attack_ticker, 0,
        "the onAttack's ticker reset the branch rides"
    );
    assert!(!app.dig.is_destroying, "the dig the stab branch never arms");
    match rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::Stab => {}
        other => panic!("expected a Stab echo, got {other:?}"),
    }
    // THE SWORD PRESS (the hit walk answers — the dig the miss
    // lane arms, no Stab the channel carries)
    app.inventory.items[app.selected_hotbar] = (sword, 1);
    app.hotbar_slots = app.inventory.hotbar();
    app.on_attack();
    assert!(
        !app.dig.is_destroying,
        "the sky-miss the dig never arms either"
    );
    // the miss lane's own Swing echo may ride — the STAB never does
    match rx.try_recv() {
        Ok(ContainerCommand::Stab) => {
            panic!("the sword's press the Stab never sends")
        }
        Ok(_) => {}
        Err(_) => {}
    }
}

/// THE SPEAR'S OWN DIG REFUSAL (the block-aimed press the stab
/// branch owns — the crosshair ON a block face the dig the stab
/// never arms: the 26.2 branch the hit walk replaces, the spear
/// the mining tool never wears).
#[test]
fn the_spear_the_block_aim_the_dig_never_arms() {
    let mut app = App::new(3);
    app.enter_world();
    let spear = mc_registry::item_id_by_name("diamond_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // the camera the ground reads (the block lane the clip arms —
    // a NORMAL stack the dig would arm here)
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (top + 4) as f32, c as f32 + 0.5),
        yaw: 180.0,
        pitch: 80.0,
    });
    // THE SWORD SANITY (the same aim the dig arms — the walk's own
    // normal verdict the counter-case pins)
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    app.inventory.items[app.selected_hotbar] = (sword, 1);
    app.hotbar_slots = app.inventory.hotbar();
    app.on_attack();
    assert!(app.dig.is_destroying, "the sword the dig arms");
    app.dig = crate::dig::DigState::default();
    // THE SPEAR PRESS (the stab branch the dig refuses — the
    // diamond spear's own full charge: the 1.0/1.05 modifier the
    // 21-tick delay keeps, the gate the not-yet-full press
    // refuses)
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    app.attack_ticker = 21;
    app.on_attack();
    assert!(
        !app.dig.is_destroying,
        "the spear the dig never arms — the stab owns the press"
    );
}

/// THE MINIMUM CHARGE'S OWN BOUNDARY (the 26.2 cannotAttackWithItem
/// gate the startAttack pcs 134-147 serves — the iron spear's own
/// 19-tick delay the `1.0/0.95` attribute keeps: the ticker 18 the
/// `18/19 < 1.0` compare REFUSES — the early return the swing/
/// sound/packet/missTime the press never wears, the ticker the
/// reset never touches — the ticker 19 the `19/19 == 1.0` compare
/// ADMITS, the stab branch the full meter serves).
#[test]
fn the_spear_the_charge_the_press_refuses() {
    use crate::world_view::ContainerCommand;
    let mut app = App::new(3);
    app.enter_world();
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // the camera the sky reads (the miss lane the un-gated press
    // would serve — the gate returns BEFORE it)
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(tx);
    // THE NOT-YET-FULL PRESS (ticker 18 — one tick the 19-tick
    // delay falls short: pc 146's own ireturn)
    app.attack_ticker = 18;
    app.on_attack();
    assert!(
        !app.swing.swinging,
        "the refused press the swing never wears"
    );
    assert_eq!(app.miss_time, 0, "the gate returns BEFORE the miss lane");
    assert_eq!(app.attack_ticker, 18, "the refusal the ticker never resets");
    assert!(
        rx.try_recv().is_err(),
        "the refused press no echo rides — no Stab, no Swing"
    );
    // THE FULL-CHARGE PRESS (ticker 19 — the exact `== 1.0` the
    // `<` compare admits: the stab the meter serves)
    app.attack_ticker = 19;
    app.on_attack();
    assert!(app.swing.swinging, "the admitted press the stab swings");
    assert_eq!(app.attack_ticker, 0, "the onAttack's own ticker reset");
    match rx.try_recv().expect("the stab echo rode the lane") {
        ContainerCommand::Stab => {}
        other => panic!("expected a Stab echo, got {other:?}"),
    }
}

/// THE OFFLINE STAB'S OWN CONTACT (javap `PiercingWeapon.attack` —
/// the offline world's own server stand-in the PIERCING branch now
/// serves): the body the ray names the `stabAttack` call hurts —
/// the raw `getAttributeValue(ATTACK_DAMAGE)` the damage wears
/// (the 2.0 base the iron spear's +2.0 bonus the attribute rides:
/// 4.0 the health delta speaks, the charge scale the melee walk
/// keeps the stab NEVER wears), the hit voice the contact's own
/// `makeHitSound` speaks (`item.spear.hit` — full volume, full
/// pitch, the attacker's own ear), the whoosh the `makeSound` call
/// carries AFTER the hit voice, and the zombie's own hurt voice
/// the hurt pipeline rides.
#[test]
fn the_offline_stab_the_contact_the_raw_attribute_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    let cam = app.world_camera();
    let f = cam.forward();
    let (ex, ey, ez) = (cam.pos.x as f64, cam.pos.y as f64, cam.pos.z as f64);
    // the body on the aim line (the box front face ~2.7 — inside
    // the 4.625 window, outside the 1.875 dead zone)
    let t = 3.0;
    app.mobs.push(crate::mob::Zombie::new(
        200,
        ex + f.x as f64 * t,
        ey - 1.0,
        ez + f.z as f64 * t,
        4,
    ));
    let pre = app.mobs[0].health;
    app.attack_ticker = 19; // the iron spear's own 19-tick delay
    app.on_attack();
    // THE RAW ATTRIBUTE (the 2.0 + 2.0 the material bonus keeps —
    // the 0.2+0.8·charge² scale the melee walk speaks the stab
    // never wears; the zombie's own 2.0 armor the dose absorbs —
    // the CombatRules fold the 3.936 survivor speaks, the bare 4.0
    // the unarmored body alone would wear)
    let drop = pre - app.mobs[0].health;
    assert!(
        (3.9..4.0).contains(&drop),
        "the iron stab the raw 4.0 the armor law absorbs ({drop})"
    );
    assert!(app.swing.swinging, "the stab the swing carries");
    assert_eq!(app.attack_ticker, 0, "the onAttack's own ticker reset");
    if !sounds_loaded() {
        return;
    }
    // THE HIT VOICE (the contact's own channel the makeHitSound
    // call carries — v1.0 p1.0 the attacker's own ear keeps)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.hit ") && e.contains(" v1.000000 p1.000000")),
        "the iron stab's hit voice: {:?}",
        app.sounds.events
    );
    // THE WHOOSH AFTER (the makeSound call the attack walk banks)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.attack ")),
        "the stab's own whoosh: {:?}",
        app.sounds.events
    );
    // THE ZOMBIE'S OWN HURT VOICE (the hurt pipeline the stabAttack
    // call rides)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.zombie.hurt ")),
        "the body's own hurt voice: {:?}",
        app.sounds.events
    );
    // THE ORDER (the hit voice the whoosh precedes — the vanilla
    // order the attack walk banks)
    let hit = app
        .sounds
        .events
        .iter()
        .position(|e| e.starts_with("item.spear.hit "))
        .expect("the hit voice rode the lane");
    let whoosh = app
        .sounds
        .events
        .iter()
        .position(|e| e.starts_with("item.spear.attack "))
        .expect("the whoosh rode the lane");
    assert!(hit < whoosh, "the hit voice the whoosh precedes");
}

/// THE OFFLINE STAB'S OWN DEAD ZONE (javap `AttackRange.isInRange`'s
/// min fold over the stab's own ray — the 2.0 - 0.125 = 1.875 band
/// the too-close body the collection refuses): the chest-bumping
/// body the contact never serves — no hurt, no hit voice — the
/// whoosh the unconditional press still wears (the swing the
/// vanilla speaks whether or not a body stood there).
#[test]
fn the_offline_stab_the_dead_zone_the_polearm_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    let cam = app.world_camera();
    let f = cam.forward();
    let (ex, ey, ez) = (cam.pos.x as f64, cam.pos.y as f64, cam.pos.z as f64);
    // the body close on the aim line (the box front face ~0.9 —
    // inside the 1.875 dead zone the polearm's ray starts beyond)
    let t = 1.2;
    app.mobs.push(crate::mob::Zombie::new(
        200,
        ex + f.x as f64 * t,
        ey - 1.0,
        ez + f.z as f64 * t,
        4,
    ));
    let pre = app.mobs[0].health;
    app.attack_ticker = 19;
    app.on_attack();
    assert_eq!(app.mobs[0].health, pre, "the dead zone the contact refuses");
    assert!(app.swing.swinging, "the swing the miss still wears");
    if !sounds_loaded() {
        return;
    }
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.hit ")),
        "the dead zone the hit voice never wears: {:?}",
        app.sounds.events
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.attack ")),
        "the whoosh the press still wears: {:?}",
        app.sounds.events
    );
}

/// THE OFFLINE STAB'S OWN COLLECTION (javap `PiercingWeapon.attack`'s
/// `getHitEntitiesAlong` walk — the Either's RIGHT arm the identity
/// lambda keeps): EVERY body along the ray the window serves the
/// `stabAttack` call — the polearm PIERCES through the line, both
/// bodies the one press hurts (the nearest-first order the
/// collection walk wears, the melee pick's single verdict the stab
/// never keeps).
#[test]
fn the_offline_stab_the_collection_the_line_pierces() {
    let mut app = App::new(3);
    app.enter_world();
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    let cam = app.world_camera();
    let f = cam.forward();
    let (ex, ey, ez) = (cam.pos.x as f64, cam.pos.y as f64, cam.pos.z as f64);
    // the two bodies the one line holds (the front faces ~2.2 and
    // ~3.7 — both inside the 4.625 window, both outside the dead
    // zone)
    for (id, t) in [(200, 2.5), (201, 4.0)] {
        app.mobs.push(crate::mob::Zombie::new(
            id,
            ex + f.x as f64 * t,
            ey - 1.0,
            ez + f.z as f64 * t,
            4,
        ));
    }
    let (pre0, pre1) = (app.mobs[0].health, app.mobs[1].health);
    app.attack_ticker = 19;
    app.on_attack();
    let drop0 = pre0 - app.mobs[0].health;
    let drop1 = pre1 - app.mobs[1].health;
    assert!(
        (3.9..4.0).contains(&drop0),
        "the near body the raw 4.0 the armor absorbs ({drop0})"
    );
    assert!(
        (3.9..4.0).contains(&drop1),
        "the far body the same dose the pierce speaks ({drop1})"
    );
}

/// THE OFFLINE STAB'S OWN WOOD ROW (the factory's WOOD branch the
/// rows name): the wooden tier's own hit voice
/// `item.spear_wood.hit` the contact speaks, the whoosh the
/// `item.spear_wood.attack` row wears, and the damage the bare
/// base keeps (the WOOD material's 0.0 bonus the attribute rides:
/// 2.0 the health delta speaks).
#[test]
fn the_offline_stab_the_wood_tier_the_row_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let wooden = mc_registry::item_id_by_name("wooden_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (wooden, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    let cam = app.world_camera();
    let f = cam.forward();
    let (ex, ey, ez) = (cam.pos.x as f64, cam.pos.y as f64, cam.pos.z as f64);
    let t = 3.0;
    app.mobs.push(crate::mob::Zombie::new(
        200,
        ex + f.x as f64 * t,
        ey - 1.0,
        ez + f.z as f64 * t,
        4,
    ));
    let pre = app.mobs[0].health;
    app.attack_ticker = 13; // the wooden spear's own 13-tick delay
    app.on_attack();
    // THE BARE BASE (the WOOD material's 0.0 bonus the 2.0 keeps —
    // the zombie's own 2.0 armor the dose absorbs: the 1.92
    // survivor the CombatRules fold speaks)
    let drop = pre - app.mobs[0].health;
    assert!(
        (1.8..2.0).contains(&drop),
        "the wood stab the bare 2.0 the armor law absorbs ({drop})"
    );
    if !sounds_loaded() {
        return;
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear_wood.hit ") && e.contains(" v1.000000 p1.000000")),
        "the wooden tier's own hit row: {:?}",
        app.sounds.events
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear_wood.attack ")),
        "the wooden tier's own whoosh: {:?}",
        app.sounds.events
    );
}

/// THE WIRE STAB'S OWN SERVER (the live lane — the contact walk the
/// server's own process keeps): the networked world's press the
/// Stab packet sends, the local walk the `world_is_network` gate
/// never runs — the wire bodies the server's own verdict names, the
/// hit voice the entity-event mailbox answers (the byte-2 echo the
/// autopush-473 lane carries), the whoosh the prediction still
/// speaks.
#[test]
fn the_wire_stab_the_contact_the_server_keeps() {
    use crate::world_view::ContainerCommand;
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.live_held = Some(spear);
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(tx);
    // the wire body ahead (the server's own verdict the contact names)
    app.live_mobs_updated(vec![wire_zombie(17, 0.0, 62.62, 3.0)]);
    app.attack_ticker = 19;
    app.on_attack();
    match rx.try_recv().expect("the stab echo rode the lane") {
        ContainerCommand::Stab => {}
        other => panic!("expected a Stab echo, got {other:?}"),
    }
    assert!(app.swing.swinging, "the stab the swing carries");
    if !sounds_loaded() {
        return;
    }
    // THE LOCAL WALK'S SILENCE (the contact verdict the server owns
    // — the hit voice the mailbox lane answers, not the press)
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.hit ")),
        "the wire press the local hit voice never wears: {:?}",
        app.sounds.events
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.attack ")),
        "the whoosh the prediction speaks: {:?}",
        app.sounds.events
    );
}

/// THE KINETIC WEAPON'S OWN USE (javap `Item.use` pcs 90-122 — the
/// right-press hold the KINETIC_WEAPON component opens): the use
/// the press arms, the component's own sound the `makeSound` call
/// speaks (SPEAR_USE/SPEAR_WOOD_USE — `item.spear.use` /
/// `item.spear_wood.use`, 1.0/1.0 the playSound-at-the-player law
/// keeps), and the release the honest silence keeps (no
/// releaseUsing the spear ever speaks — the hold just ends).
#[test]
fn the_spear_use_the_press_the_hold_arms() {
    use crate::world_view::ContainerCommand;
    let mut app = App::new(3);
    app.enter_world();
    let iron = mc_registry::item_id_by_name("iron_spear").unwrap();
    let wooden = mc_registry::item_id_by_name("wooden_spear").unwrap();
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    app.inventory.items[app.selected_hotbar] = (iron, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // THE IRON PRESS (the use the hold opens — the item lane the
    // eat/crossbow/bow gates the spear never wears)
    assert!(app.on_right_press(0, 0), "the spear the hold arms");
    let active = app.using_item.as_ref().expect("the use opened");
    assert_eq!(active.item, iron, "the held spear the use rides");
    assert_eq!(active.ticks, 0, "the clock the fresh hold keeps");
    // THE WOOD TIER (the factory's WOOD branch the row names)
    app.on_right_release();
    assert!(app.using_item.is_none(), "the release the hold ends");
    app.inventory.items[app.selected_hotbar] = (wooden, 1);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(app.on_right_press(0, 0), "the wooden spear the hold arms");
    // THE SWORD'S OWN REFUSAL (the KINETIC_WEAPON component the
    // spears alone carry — the sword the item_using_def lane's
    // own PASS keeps)
    app.on_right_release();
    app.inventory.items[app.selected_hotbar] = (sword, 1);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(!app.on_right_press(0, 0), "the sword the hold never arms");
    assert!(app.using_item.is_none(), "the sword the use never opens");
    if !sounds_loaded() {
        return;
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.use ") && e.contains(" v1.000000 p1.000000")),
        "the iron tier's own use voice: {:?}",
        app.sounds.events
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear_wood.use ") && e.contains(" v1.000000 p1.000000")),
        "the wooden tier's own use voice: {:?}",
        app.sounds.events
    );
    // THE WIRE ECHO'S ABSENCE (the offline world no live lane keeps)
    assert!(!app.sounds.events.is_empty(), "the sounds the test speaks");
    let _ = std::mem::take(&mut app.sounds.events);
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(tx);
    app.inventory.items[app.selected_hotbar] = (iron, 1);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(app.on_right_press(0, 0), "the live press the hold arms");
    match rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::UseItem { hand, .. } => {
            assert_eq!(hand, 0, "the MAIN hand ordinal the echo wears");
        }
        other => panic!("expected a UseItem echo, got {other:?}"),
    }
}

/// THE BRUSH'S OWN USE-ON WALK (javap `BrushItem.useOn` over the
/// real 26.2 jar — the 31-pc body the whole press wears): the BLOCK
/// hit the using window opens (`startUsingItem` the BLOCK gate
/// arms), the CONSUME the always-verdict speaks, the MISS the
/// `useOn`-less air walk the PASS keeps (the brush the `use` body
/// never carries — the jar's own method list the seven methods
/// bank), the look-away the entity-level cancel closes (NO packet
/// the entity walk sends — the vanilla rightClickStop's echo the
/// RMB edge alone carries), the release the wire's own RELEASE_USE
/// echo rides.
#[test]
fn the_brush_the_press_the_window_wears() {
    use crate::world_view::ContainerCommand;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // the hit cell the STONE row keeps (the client's cell vocabulary
    // the suspicious cells never banks — every cell the generic arm
    // the tick walk names)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, surface, c, crate::world::STONE);
    let brush = mc_registry::item_id_by_name("brush").unwrap();
    app.inventory.items[app.selected_hotbar] = (brush, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // THE LIVE LANE FIRST (the echo the block lane's own head
    // speaks — the useItemOn packet the window NEVER doubles)
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(tx);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the always-CONSUME the press speaks"
    );
    let active = app.using_item.as_ref().expect("the window opened");
    assert_eq!(active.item, brush, "the held brush the use rides");
    assert_eq!(active.ticks, 0, "the clock the fresh hold keeps");
    assert_eq!(active.state, 0, "the state the def-less item keeps");
    match rx.try_recv().expect("the useItemOn echo rode") {
        ContainerCommand::UseItemOn { pos, .. } => {
            assert_eq!(pos, (c, surface, c), "the hit cell the packet wears");
        }
        other => panic!("expected a UseItemOn echo, got {other:?}"),
    }
    assert!(
        rx.try_recv().is_err(),
        "the ONE echo the block lane spoke — the window never doubles"
    );
    // THE LOOK-AWAY CANCEL (javap onUseTick pcs 26-64): the miss the
    // view-vector clip re-walks, the entity-level release the
    // silence keeps — NO packet the cancel sends
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: std::f32::consts::FRAC_PI_2,
    });
    app.update();
    assert!(
        app.using_item.is_none(),
        "the look-away the entity-level cancel closes"
    );
    assert!(
        rx.try_recv().is_err(),
        "the cancel no packet sends — the entity walk the vanilla silence keeps"
    );
    // THE MISS ARM (the BLOCK case the walk never routes): the air
    // walk the PASS keeps — the press never consumed, the window
    // never opens (the `use` body the jar never banks)
    assert!(!app.on_right_press(0, 0), "the miss the PASS keeps");
    assert!(app.using_item.is_none(), "the miss the window never opens");
    // THE RELEASE EDGE (the vanilla rightClickStop's own walk — the
    // gameMode.releaseUsingItem the RELEASE_USE echo speaks): the
    // re-press the window re-opens, the RMB-up the echo rides
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the re-press the window re-opens");
    match rx.try_recv().expect("the block echo rode again") {
        ContainerCommand::UseItemOn { .. } => {}
        other => panic!("expected a UseItemOn echo, got {other:?}"),
    }
    app.on_right_release();
    assert!(app.using_item.is_none(), "the release the hold ends");
    match rx.try_recv().expect("the release echo rode") {
        ContainerCommand::ReleaseUse => {}
        other => panic!("expected a ReleaseUse echo, got {other:?}"),
    }
}

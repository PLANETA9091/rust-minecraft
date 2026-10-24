use super::*;

/// THE BRUSH TICK'S OWN CADENCE (javap `BrushItem.onUseTick` pcs
/// 65-92 + 177-224 + updateUsingItem's own tail): the t the
/// 1-based elapsed the client's own clock keeps, the `% 10 == 5`
/// gate the voice arms (t = 5, 15, ..., 195 — the TWENTY voices the
/// 200-tick window spends), the fixed 1.0/1.0 the LevelAccessor
/// default's fconst_1 pair keeps, and the `sipush 200` duration the
/// `--useItemRemaining <= 0` walk completes — the window the local
/// end closes, no packet the complete sends.
#[test]
fn the_brush_the_tick_the_voice_the_cadence_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, surface, c, crate::world::STONE);
    let brush = mc_registry::item_id_by_name("brush").unwrap();
    app.inventory.items[app.selected_hotbar] = (brush, 1);
    app.hotbar_slots = app.inventory.hotbar();
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the window the press opens");
    let _ = std::mem::take(&mut app.sounds.events);
    // t = 1..4: the cadence the gate keeps silent
    for _ in 0..4 {
        app.update();
    }
    if sounds_loaded() {
        assert!(
            !app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("item.brush.brushing.generic @")),
            "the pre-cadence ticks the silence keeps"
        );
    }
    // t = 5: the FIRST voice the cadence arms
    app.update();
    if sounds_loaded() {
        let voices = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("item.brush.brushing.generic @"))
            .count();
        assert_eq!(voices, 1, "the t=5 voice the cadence arms");
        let voice = app
            .sounds
            .events
            .iter()
            .find(|e| e.starts_with("item.brush.brushing.generic @"))
            .unwrap();
        let pitch: f32 = voice
            .split(" p")
            .nth(1)
            .and_then(|t| t[..6].parse().ok())
            .unwrap_or(0.0);
        let volume: f32 = voice
            .split(" v")
            .nth(1)
            .and_then(|t| t[..6].parse().ok())
            .unwrap_or(0.0);
        assert!(
            (pitch - 1.0).abs() < 1e-6 && (volume - 1.0).abs() < 1e-6,
            "the fixed fold the fconst_1 pair keeps: {voice}"
        );
    }
    // t = 6..10: the mid-decade ticks the silence keeps
    for _ in 0..5 {
        app.update();
    }
    if sounds_loaded() {
        assert_eq!(
            app.sounds
                .events
                .iter()
                .filter(|e| e.starts_with("item.brush.brushing.generic @"))
                .count(),
            1,
            "the t=10 tick the gate never arms"
        );
    }
    // t = 11..15: the SECOND voice the next decade arms
    for _ in 0..5 {
        app.update();
    }
    if sounds_loaded() {
        assert_eq!(
            app.sounds
                .events
                .iter()
                .filter(|e| e.starts_with("item.brush.brushing.generic @"))
                .count(),
            2,
            "the t=15 voice the cadence arms"
        );
    }
    // t = 16..200: the window runs out — the t=195 voice the last
    // the cadence speaks, the 200-tick complete the `sipush 200`
    // duration closes (20 voices the full window spends)
    for _ in 0..185 {
        app.update();
    }
    assert!(
        app.using_item.is_none(),
        "the 200-tick complete the window closes"
    );
    if sounds_loaded() {
        assert_eq!(
            app.sounds
                .events
                .iter()
                .filter(|e| e.starts_with("item.brush.brushing.generic @"))
                .count(),
            20,
            "the twenty voices the 5,15..195 cadence spends"
        );
    }
}

/// THE BRUSH DUST'S OWN BURST (javap `spawnDustParticles` +
/// `DustParticlesDelta.fromDirection` over the real 26.2 jar — the
/// CP walk the #206/#256/#48/#50/#52 constants resolved): the
/// flag-tick burst the side faces' fixed tangent rides (NORTH the
/// (1.0, -0.1) pair the case-3 arm speaks), the count the
/// `nextInt(7, 12)` band arms, the velocity the `delta * side *
/// 3.0 * nextDouble()` fold carries, the yd the dconst_0 keeps,
/// the spawn point the hit location + the NORTH z-epsilon wears,
/// and the terrain ctor's own draws the particle walks ride.
#[test]
fn the_brush_the_dust_the_tangent_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, surface, c, crate::world::STONE);
    // the trench the side ray's own layer clears (the till test's
    // own walk — the ray's path the camera-to-face walk keeps
    // solid-free, the pinned stone the FIRST cell the ray enters)
    for dz in -3..0 {
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c + dz, crate::world::AIR);
    }
    let brush = mc_registry::item_id_by_name("brush").unwrap();
    app.inventory.items[app.selected_hotbar] = (brush, 1);
    app.hotbar_slots = app.inventory.hotbar();
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 0.5, (c - 3) as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: 0.0,
    });
    assert!(app.on_right_press(0, 0), "the window the press opens");
    let _ = std::mem::take(&mut app.sounds.events);
    // t = 1..4: the pre-cadence ticks the burst never spends
    for _ in 0..4 {
        app.update();
    }
    let before = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Terrain { .. }))
        .count();
    // t = 5: the burst the flag tick speaks
    app.update();
    let burst: Vec<&crate::particles::Particle> = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Terrain { .. }))
        .skip(before)
        .collect();
    assert!(
        (7..12).contains(&burst.len()),
        "the nextInt(7, 12) band the count wears: {}",
        burst.len()
    );
    for p in &burst {
        // THE TERRAIN KIND (the hit cell's own tile — stone the
        // STONE row maps)
        let crate::particles::ParticleKind::Terrain { tile } = p.kind else {
            panic!("the terrain kind the burst banks");
        };
        assert_eq!(tile, particle_tile(crate::world::STONE), "the stone tile");
        // THE TANGENT'S OWN VELOCITY (the NORTH face the (1.0,
        // -0.1) pair wears, the side the RIGHT arm answers, the
        // 3.0 the #206 double multiplies): the spawn velocity
        // vx = 3u, vz = -0.3u, yd = 0 — the PARTICLE TICK the same
        // update walks (the velocity step + the 0.98 friction +
        // the 0.04 gravity fold), so the post-tick reads: vx in
        // [0, 3), vz in (-0.3, 0], the yd the gravity's own -0.04
        // x 0.98 fold moves
        assert!(
            p.vx >= 0.0 && p.vx < 3.0,
            "the +x slide the NORTH tangent wears: {}",
            p.vx
        );
        assert!(
            p.vz <= 0.0 && p.vz > -0.3,
            "the -z drift the case-3's -0.1 keeps: {}",
            p.vz
        );
        assert!(
            p.vy < 0.0 && p.vy > -0.05,
            "the dconst_0 yd the gravity fold moved: {}",
            p.vy
        );
        // THE SPAWN POINT + THE FIRST TICK'S OWN WALK (the hit
        // location the ray's own x/y keeps — the f32 forward's own
        // sine drift the 1e-6 band holds — z the face boundary the
        // NORTH epsilon + the -0.3u slide the first tick walks)
        assert!(
            p.x >= c as f64 + 0.5 - 1e-6 && p.x < c as f64 + 0.5 + 3.0,
            "the ray's own x the +x slide walked: {}",
            p.x
        );
        assert!(
            p.y <= surface as f64 + 0.5 && p.y > surface as f64 + 0.45,
            "the eye layer the gravity fold dipped: {}",
            p.y
        );
        assert!(
            p.z <= c as f64 && p.z > c as f64 - 0.35,
            "the NORTH face z the tangent slide walked: {}",
            p.z
        );
        // THE CTOR'S OWN FOLDS (the terrain particle's gravity
        // 1.0 / friction 0.98 the base ctor wears)
        assert_eq!(p.gravity, 1.0, "the terrain gravity");
        assert_eq!(p.friction, 0.98, "the terrain friction");
    }
    // t = 6..10: the mid-decade ticks the burst never spends
    let settled = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Terrain { .. }))
        .count();
    for _ in 0..5 {
        app.update();
    }
    let settled_now = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Terrain { .. }))
        .count();
    assert!(
        settled_now <= settled,
        "the mid-decade ticks the burst never spends (terrain particles only age)"
    );
}

/// THE USE POSE'S OWN LANE (javap `submitArmWithItem` pcs 583-603 +
/// 1287-1336 — the SPEAR use-animation branch the `$SwitchMap`
/// maps case 9): the pose the gates dress — the hold open, the
/// USED hand's lane only, the held spear the windows name — and
/// the press frame the identity the zero raise keeps, the ticks
/// the raise speaks, the release the silence returns, the hit
/// feedback's 0.375 jolt the tail translate dips one tick after
/// the bank (the same raise both poses keep — the delta the
/// feedback's own law isolates).
#[test]
fn the_use_pose_the_lane_the_gate_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let iron = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (iron, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // THE CLOSED HOLD: no use, no pose
    assert!(app.held_spear_use_pose(UseHand::Main).is_none());
    assert!(app.held_spear_use_pose(UseHand::Off).is_none());
    // THE PRESS: the pose arms — the press frame the identity keeps
    // (timeHeld = 0 - 1 + partial 1.0 = 0, the raise's zero terms)
    assert!(app.on_right_press(0, 0), "the spear the hold arms");
    let pose = app
        .held_spear_use_pose(UseHand::Main)
        .expect("the MAIN lane the pose rides");
    assert_eq!(
        pose.0,
        crate::math::Mat4::translation(0.0, 0.0, 0.0).0,
        "the press frame the identity"
    );
    // THE OTHER LANE: the main-lane use the OFF lane never dresses
    // (the getUsedItemHand() == hand conjunct)
    assert!(app.held_spear_use_pose(UseHand::Off).is_none());
    // THE TICKS: the raise fades in — ten updates the 0.833 the
    // iron row's own 12-tick delay still rides (timeHeld = 10, the
    // pose the identity never equals)
    for _ in 0..10 {
        app.update();
    }
    let raised = app
        .held_spear_use_pose(UseHand::Main)
        .expect("the pose rides the raised frame");
    assert_ne!(raised.0, pose.0, "the raise the ticks speak");
    // THE RELEASE: the hold ends, the pose the silence returns
    app.on_right_release();
    assert!(app.held_spear_use_pose(UseHand::Main).is_none());
    // THE HIT FEEDBACK'S OWN JOLT: the byte-2 bank the negative
    // stamp lifts, one tick later the 0.375 the tail translate
    // dips — the raise PAST its plateau (12 ticks, the iron row's
    // own delay the clamp's steady terms keep) both poses share,
    // the delta the feedback's own law isolates
    assert!(app.on_right_press(0, 0), "the hold re-arms");
    for _ in 0..12 {
        app.update(); // ticks = 12 — the raise clamped at its full
    }
    let no_hit = app
        .held_spear_use_pose(UseHand::Main)
        .expect("the pose rides");
    app.live_kinetic_hit_events_updated(vec![crate::world_view::WireKineticHitEvent {
        entity_id: 0,
    }]);
    app.update(); // the feedback tick = 1 + partial 1.0 = 2 → 0.375
    let hit = app
        .held_spear_use_pose(UseHand::Main)
        .expect("the pose rides the jolt");
    let o_no = no_hit.transform_point(crate::math::Vec3::new(0.0, 0.0, 0.0));
    let o_hit = hit.transform_point(crate::math::Vec3::new(0.0, 0.0, 0.0));
    // the jolt the INNERMOST translate speaks — the dipped point
    // the two pivots then swing: the X the rotations never move,
    // the y dips 0.1585, the z reaches 0.3399 further out (the
    // deltas the f64 walk of (0, −0.375, 0) through the raise's
    // own two rotAround folds keeps)
    assert!(
        (o_hit[0] - o_no[0]).abs() < 1e-6,
        "the jolt never moves X (got {} vs {})",
        o_hit[0],
        o_no[0]
    );
    assert!(
        ((o_hit[1] - o_no[1]).abs() - 0.158_481_85).abs() < 1e-4,
        "the dipped y the tilt lifts (got {} vs {})",
        o_hit[1],
        o_no[1]
    );
    assert!(
        ((o_hit[2] - o_no[2]).abs() - 0.339_865_42).abs() < 1e-4,
        "the dipped z the turn reaches (got {} vs {})",
        o_hit[2],
        o_no[2]
    );
}

/// THE USE-TICK STAB'S OWN DELAY (javap `KineticWeapon.damageEntities`
/// pcs 9-27 + 188-193): the first 10 ticks the hold the walk never
/// passes (no bank, no hurt), the tick 10 the bank the remember
/// call performs — the standing body's zero dot the conditions
/// refuse (the relative 0 the 4.6 gate never meets), the next tick
/// the recent-stab memory the re-evaluation never serves.
#[test]
fn the_spear_use_tick_the_delay_the_first_ticks_sleep() {
    let mut app = App::new(3);
    app.enter_world();
    let iron = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (iron, 1);
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
    app.mobs.push(crate::mob::Zombie::new(
        200,
        ex + f.x as f64 * 3.0,
        ey - 1.0,
        ez + f.z as f64 * 3.0,
        4,
    ));
    let pre = app.mobs[0].health;
    // THE DELAY'S OWN SLEEP (the first `delayTicks` ticks the walk
    // refuses — no bank the memory keeps, no hurt the body wears;
    // the OLD law's tick 10 the iron tier's own 12 the sleep
    // extends past)
    app.spear_use_tick_stab(5);
    assert!(
        app.stab_contact_memory.is_empty(),
        "the delay no bank keeps"
    );
    assert_eq!(app.mobs[0].health, pre, "the delay the body never hurts");
    app.spear_use_tick_stab(10);
    assert!(
        app.stab_contact_memory.is_empty(),
        "the iron tier's own 12 the tick 10 never passes"
    );
    // THE FIRST EVALUATION (tick 12): the bank rides — the
    // standing body's zero dot the conditions refuse (the
    // refused evaluation the clock still spends)
    app.spear_use_tick_stab(12);
    assert_eq!(
        app.stab_contact_memory.len(),
        1,
        "the bank the remember call keeps"
    );
    assert_eq!(app.mobs[0].health, pre, "the standing stab never hurts");
    // THE RECENT-STAB SKIP (tick 13 — `now - last < 10` the CONTACT
    // constant's own window the recently verdict keeps: no re-bank,
    // no evaluation)
    app.spear_use_tick_stab(13);
    assert_eq!(
        app.stab_contact_memory.len(),
        1,
        "the skip no re-bank keeps"
    );
    assert_eq!(app.mobs[0].health, pre, "the cooldown the body guards");
}

/// THE USE-TICK STAB'S OWN KINETIC DAMAGE (javap
/// `KineticWeapon.damageEntities` pcs 361-397 over the sprinting
/// body): the look·motion dot the `getKnownSpeed()·20` fold wears
/// (0.28/tick × 20 = 5.6 b/s — the sprint the 4.6 gate meets), the
/// damage the base + the `floor(relative·multiplier)` fold speaks
/// (2.0 + floor(5.6·0.95) = 7.0 — the RAW base the
/// `getAttributeBaseValue` reads, the material bonus the use walk
/// never sees), the zombie's own 2.0 armor the dose absorbs, the
/// byte-2 the contact broadcasts — the local hit voice the
/// feedback lane carries.
#[test]
fn the_spear_use_tick_the_sprint_thrust_the_kinetic_damage_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let iron = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (iron, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    // the body the camera's own authority wears (the eye the
    // camera reads — the lunge the body's own velocity carries)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        (app.spawn_y + 7) as f64 - 1.62,
        c as f64 + 0.5,
        180.0,
        0.0,
    ));
    let cam = app.world_camera();
    let f = cam.forward();
    let (ex, ey, ez) = (cam.pos.x as f64, cam.pos.y as f64, cam.pos.z as f64);
    app.mobs.push(crate::mob::Zombie::new(
        200,
        ex + f.x as f64 * 3.0,
        ey - 1.0,
        ez + f.z as f64 * 3.0,
        4,
    ));
    // THE SPRINT'S OWN LUNGE (the velocity the look carries —
    // 0.28 b/tick × 20 = 5.6 b/s the sprint walk rides)
    if let Some(b) = app.body.as_mut() {
        b.vx = 0.28 * f.x as f64;
        b.vz = 0.28 * f.z as f64;
    }
    let pre = app.mobs[0].health;
    app.frame = 30; // the throttle's bank the first window clears
    app.spear_use_tick_stab(12); // the iron tier's own 12 the walk passes
                                 // THE KINETIC FOLD (the 2.0 + floor(5.6·0.95) = 7.0 the armor
                                 // absorbs: the 6.888 survivor the CombatRules walk speaks)
    let drop = pre - app.mobs[0].health;
    assert!(
        (6.8..7.0).contains(&drop),
        "the sprint thrust the 7.0 the armor absorbs ({drop})"
    );
    assert!(
        !app.stab_contact_memory.is_empty(),
        "the bank the contact keeps"
    );
    if !sounds_loaded() {
        return;
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.hit ") && e.contains(" v1.000000 p1.000000")),
        "the byte-2's own hit voice: {:?}",
        app.sounds.events
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.zombie.hurt ")),
        "the body's own hurt voice: {:?}",
        app.sounds.events
    );
    // THE COOLDOWN'S OWN SILENCE (the next tick the recent-stab
    // memory the re-evaluation never serves — no second dose, the
    // throttle's own window the second voice would never pass)
    let voiced = app.sounds.events.len();
    app.spear_use_tick_stab(13);
    assert_eq!(
        app.mobs[0].health,
        pre - drop,
        "the cooldown the second dose refuses"
    );
    if sounds_loaded() {
        assert_eq!(
            app.sounds.events.len(),
            voiced,
            "the cooldown the second voice refuses"
        );
    }
}

/// THE MELEE'S OWN DOSE (javap `WardenAi.initFightActivity` +
/// `MeleeAttack.create(18)` + `Mob.doHurtTarget` over the offline
/// world — the warden the reach's own box law the swing serves,
/// the 30.0 the health wears, the 1.5 push the yaw speaks, the
/// impact voice the landing plays, the 18l the cooldown banks).
#[test]
fn the_warden_melee_the_dose_the_push_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    // THE AIR NEIGHBORHOOD (the stab lane's own sky the eye ray
    // never fears — both feet the same altitude the boxes
    // intersect)
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    // THE WARDEN THE +X NEIGHBOR (the boxes the 1.0 gap the
    // 0.45 + 0.828 + 0.3 reach intersects), THE YAW THE -X
    // FACING WEARS (the push toward the player the yaw answers)
    app.wardens
        .push(crate::sculk::WardenMob::new(201, px + 1.0, feet_y, pz));
    app.wardens[0].attack_target = Some(crate::sculk::WARDEN_PLAYER_SUSPECT);
    app.wardens[0].yaw_deg = 90.0;
    // THE SONIC BANK THE FRESH WINDOW KEEPS (javap `SonicBoom` —
    // the FIGHT list's own index-4 the melee prio-15 precedes:
    // the fresh warden the in-reach target the CHARGE arms, the
    // swing the standing window refuses — the melee dose the
    // post-boom gameplay the bank serves: the 40l the
    // doHurtTarget's own setCooldown wears)
    app.wardens[0].sonic_boom_cooldown = crate::sculk::WARDEN_ATTACK_SONIC_COOLDOWN_TICKS;
    let health_before = app.health;
    let (vx_before, vz_before) = {
        let b = app.body.as_ref().expect("the body rides");
        (b.vx, b.vz)
    };
    app.update();
    // THE DOSE (the 30.0 the ATTACK_DAMAGE attr wears — the
    // health the hurt pipeline wears, the raw dose the 20.0 pool
    // the death answers)
    assert!(
        app.health < health_before,
        "the melee dose the health wears: {} vs {}",
        app.health,
        health_before
    );
    // THE KNOCKBACK (the -X push the yaw wears — nx = -sin(90°)
    // = -1, the strength 1.5 the push carries; the +Z the cos
    // never wears)
    let (vx, vz) = {
        let b = app.body.as_ref().expect("the body rides");
        (b.vx, b.vz)
    };
    assert!(
        vx - vx_before < -0.5,
        "the 1.5 push the -X yaw wears: {} vs {}",
        vx,
        vx_before
    );
    assert!(
        (vz - vz_before).abs() < 1e-6,
        "the push the +Z the yaw never wears: {} vs {}",
        vz,
        vz_before
    );
    // THE IMPACT VOICE (the 10.0 the landing speaks)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.contains("entity.warden.attack_impact")),
        "the impact voice the landing plays: {:?}",
        app.sounds
            .events
            .iter()
            .filter(|e| e.contains("warden"))
            .collect::<Vec<_>>()
    );
    // THE SWING'S OWN BANKS (the 18l the cooldown banks, the
    // event-4 the start tick banks, the 40l the sonic banks)
    assert_eq!(
        app.wardens[0].attack_cooldown,
        crate::sculk::WARDEN_MELEE_COOLDOWN_TICKS,
        "the 18l the setWithExpiry banks"
    );
    assert!(
        app.wardens[0].attack_start.is_some(),
        "the event-4 the attackAnimationState.start banks"
    );
    assert_eq!(
        app.wardens[0].sonic_boom_cooldown,
        crate::sculk::WARDEN_ATTACK_SONIC_COOLDOWN_TICKS,
        "the 40l the doHurtTarget's own setCooldown banks"
    );
}

/// THE MOB-VICTIM'S OWN REFERENCE (javap `LivingEntity.hurtServer`
/// the resolve walk's tail + `HurtByTargetGoal` over the offline
/// world — the warden's bite the setLastHurtByMob(warden)
/// reference banks, the bitten mob's own HurtBy prio-1 goal the
/// next tick's walk arms: the warden's own views the refresh pass
/// serves, the live pos the snapshot follows — the plain-dose
/// shape the stale honest-skip wore, the reference the round
/// retires).
#[test]
fn the_warden_bite_the_mob_victim_the_reference_arms() {
    let mut app = App::new(3);
    app.enter_world();
    // THE AIR NEIGHBORHOOD (the melee dose test's own stage — the
    // boxes the 1.0 gap the reach intersects)
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    // THE ZOMBIE THE PLAYER'S SPOT KEEPS (the +X neighbor the
    // warden fights), THE WARDEN THE BITE'S OWN BODY BANKS — the
    // 500.0 pool the test-only health the repeat swings survive
    // (the 30.0 raw the 20.0 zombie pool the kill answers, the
    // reference the walk the LIVING victim pins). THE WORLD'S OWN
    // WALK THE FIGHT PRECEDES (the fresh body's first ticks the
    // pre-update runs — the tickCount the bite's own moment the
    // reacted-event timestamp the NON-ZERO word arms: the age-0
    // bite the timestamp the goal's stored zero the EQUAL pair
    // never answers)
    app.mobs
        .push(crate::mob::Zombie::new(77, px + 1.0, feet_y, pz, 4));
    app.mobs[0].on_ground = true;
    app.update();
    app.mobs[0].health = 500.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(201, px, feet_y, pz));
    app.wardens[0].attack_target = Some(77);
    app.wardens[0].yaw_deg = 90.0;
    // THE SONIC BANK THE FRESH WINDOW KEEPS (the melee dose test's
    // own law — the swing the standing window refuses)
    app.wardens[0].sonic_boom_cooldown = crate::sculk::WARDEN_ATTACK_SONIC_COOLDOWN_TICKS;
    app.update();
    // THE DOSE (the 29.52 the ARMOR LAW wears — the zombie's own 2
    // armor the CombatRules fold speaks: f5 = 2.0, f6 = clamp(2 -
    // 30/2, armor*0.2, 20) = 0.4, the survivor `30 * (1 - 0.4/25)`
    // the 0.984 factor answers — the 500.0 pool the survivor keeps)
    assert_eq!(
        app.mobs[0].health, 470.48,
        "the bite the armor-absorbed dose wears: {}",
        app.mobs[0].health
    );
    // THE REFERENCE (the setLastHurtByMob the warden's own view
    // banks — the kind the WARDEN word carries, the live pos the
    // snapshot wears)
    let reference = app.mobs[0]
        .hurt_by_reference()
        .expect("the bite the reference banks");
    assert_eq!(reference.id, 201, "the warden's id the reference names");
    assert_eq!(
        reference.kind,
        crate::mob::MobKind::Warden,
        "the kind the variant banks"
    );
    assert_eq!(
        (reference.x, reference.z),
        (px, pz),
        "the live pos the snapshot wears"
    );
    // THE NEXT TICK'S OWN WALK (the warden's own views the refresh
    // pass serves — the reference the live body keeps, the
    // retaliation the HurtBy's own start arms)
    app.update();
    assert!(
        app.mobs[0].hurt_by_reference().is_some(),
        "the warden's views the reference the refresh keeps"
    );
    assert!(
        app.mobs[0].retaliation_armed(),
        "the HurtBy goal the warden's view arms"
    );
}

/// THE SONIC DOSE'S OWN REFERENCE (javap `SonicBoom.lambda$tick$2`
/// pc 121-137 + `LivingEntity.hurtServer` the resolve walk's tail
/// over the offline world — the boom the setLastHurtByMob(warden)
/// reference banks the SAME law the melee bite wears: the 10.0 the
/// hurt_by_mob walk wears, the reference the warden's own view
/// banks, the bitten mob's own HurtBy prio-1 goal the next tick's
/// walk arms — the refresh pass's own warden views the live pos
/// the chase serves).
#[test]
fn the_sonic_boom_the_mob_victim_the_reference_arms() {
    let mut app = App::new(3);
    app.enter_world();
    // THE AIR NEIGHBORHOOD (the melee reference test's own stage —
    // the zombie the +X neighbor the beam the chest→eye walk
    // serves)
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    // THE ZOMBIE THE PLAYER'S SPOT KEEPS (the +X neighbor the beam
    // crosses), THE AGE THE DOSE'S TIMESTAMP ARMS (the fresh
    // body's first tick the pre-update runs — the reacted-event
    // timestamp the NON-ZERO word the goal's own compare answers:
    // the age-0 dose the timestamp the goal's stored zero the
    // EQUAL pair never arms)
    app.mobs
        .push(crate::mob::Zombie::new(77, px + 1.0, feet_y, pz, 4));
    app.mobs[0].on_ground = true;
    app.update();
    app.mobs[0].health = 500.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(201, px, feet_y, pz));
    app.wardens[0].attack_target = Some(77);
    // THE MID-WINDOW STATE (the rings test's own surgical setup —
    // the active the running state keeps, the 2 the window's own
    // tail banks, the 1 the delay's last tick covers — the update's
    // drain walk the both-absent tick the boom arms; the standing
    // window the melee swing the same tick refuses)
    app.wardens[0].sonic_active = true;
    app.wardens[0].attack_cooldown = 2;
    app.wardens[0].sonic_sound_delay = 1;
    app.wardens[0].sonic_sound_cooldown = 0;
    app.update();
    // THE DOSE (the 9.84 the ARMOR LAW wears — the zombie's own 2
    // armor the CombatRules fold speaks: f6 = clamp(2 - 10/2,
    // armor*0.2, 20) = 0.4, the survivor `10 * (1 - 0.4/25)` the
    // 0.984 factor answers — the 500.0 pool the survivor keeps)
    let f6 = (crate::mob::ZOMBIE_ARMOR as f32 - 10.0f32 / 2.0)
        .clamp((crate::mob::ZOMBIE_ARMOR * 0.2) as f32, 20.0);
    assert_eq!(
        app.mobs[0].health,
        500.0 - 10.0 * (1.0 - f6 / 25.0),
        "the boom the armor-absorbed dose wears: {}",
        app.mobs[0].health
    );
    // THE REFERENCE (the setLastHurtByMob the warden's own view
    // banks — the kind the WARDEN word carries, the live pos the
    // snapshot wears)
    let reference = app.mobs[0]
        .hurt_by_reference()
        .expect("the boom the reference banks");
    assert_eq!(reference.id, 201, "the warden's id the reference names");
    assert_eq!(
        reference.kind,
        crate::mob::MobKind::Warden,
        "the kind the variant banks"
    );
    assert_eq!(
        (reference.x, reference.z),
        (px, pz),
        "the live pos the snapshot wears"
    );
    // THE NEXT TICK'S OWN WALK (the warden's own views the refresh
    // pass serves — the reference the live body keeps, the
    // retaliation the HurtBy's own start arms)
    app.update();
    assert!(
        app.mobs[0].hurt_by_reference().is_some(),
        "the warden's views the reference the refresh keeps"
    );
    assert!(
        app.mobs[0].retaliation_armed(),
        "the HurtBy goal the boom's reference arms"
    );
}

/// THE CHASE'S OWN WALK (javap `SetWalkTargetFromAttackTargetIfTargetOutOfReach`
/// lambda$create$3 pc 59-75 + `MoveToTargetSink` + `MeleeAttack` over the
/// offline world — the out-of-reach MOB target the 1.2f chase serves, the
/// walk the travel fold answers, the arrival the melee ring the dose
/// wears: the walk target the mob's LIVE spot banks (the
/// `EntityTracker(target, 1)` tracker the 1.2f modifier carries), the
/// body the atan2 law faces, the walk animation the move distance fills,
/// the in-reach pair the WALK_TARGET erase serves, the bite the 29.52
/// the zombie's own armor law lands, the setLastHurtByMob the reference
/// banks, the HurtBy prio-1 goal the retaliation arms — the loop the
/// 576/577 rounds closed the chase's own legs the round verifies
/// end-to-end).
#[test]
fn the_chase_the_walk_the_arrival_the_dose_lands() {
    let mut app = App::new(3);
    app.enter_world();
    // THE FLAT FLOOR (the walk test's own world the app wears — the
    // stone 0..5 the air above; the light grid the fresh build never
    // relights — the burn roll's canSeeSky the zero channel refuses:
    // the zombie the sky never brands, the 500.0 pool the health law
    // the exact keeps)
    let mut flat = crate::world::World::new(WORLD_CHUNKS, app.seed);
    for x in 0..flat.size_blocks() {
        for z in 0..flat.size_blocks() {
            for y in 0..5 {
                flat.set(x, y as i32, z, crate::world::STONE);
            }
            for y in 5..crate::world::CHUNK_Y {
                flat.set(x, y as i32, z, crate::world::AIR);
            }
        }
    }
    app.offline_world = Some(flat);
    app.spawn_y = 5;
    // THE STAGE (the player the center keeps, the zombie 3.0 east, the
    // warden 2.0 west — the 5.0 gap the 1.578 reach edge REFUSES: the
    // chase the out-of-reach pair arms, the swing the far window
    // refuses)
    let (px, pz) = (8.5f64, 8.5f64);
    app.body = Some(crate::body::PlayerBody::new(px, 5.0, pz, 180.0, 0.0));
    app.mobs
        .push(crate::mob::Zombie::new(77, px + 3.0, 5.0, pz, 4));
    app.mobs[0].on_ground = true;
    app.update();
    // THE POOL THE REPEAT SWINGS SURVIVE (the melee reference test's
    // own law — the 500.0 the dose the armor law answers)
    app.mobs[0].health = 500.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(201, px - 2.0, 5.0, pz));
    app.wardens[0].attack_target = Some(77);
    // THE SONIC BANK THE FRESH WINDOW KEEPS (the melee dose test's own
    // law — the chase the pure lane keeps, the boom the standing
    // window refuses: the 40l the bank the walk the whole serves)
    app.wardens[0].sonic_boom_cooldown = crate::sculk::WARDEN_ATTACK_SONIC_COOLDOWN_TICKS;
    // THE MOB'S OWN SPOT THE CHASE READS (the tick order the warden
    // loop FIRST serves — the mob's fields the previous tick the read
    // keeps)
    let (zx, zy, zz) = (app.mobs[0].x, app.mobs[0].y, app.mobs[0].z);
    app.update();
    // THE WALK TARGET THE MOB'S LIVE SPOT BANKS (the out-of-reach pair
    // — the EntityTracker(target, 1) tracker the 1.2f modifier
    // carries)
    let t = app.wardens[0]
        .walk_target
        .expect("the chase the walk target banks");
    assert_eq!(
        t.speed_modifier,
        crate::sculk::WARDEN_FIGHT_SPEED_MODIFIER,
        "the 1.2f the chase wears"
    );
    assert_eq!(t.close_enough, 1, "the tracker's own arg keeps");
    assert_eq!(t.pos, (zx, zy, zz), "the mob's live spot the tracker banks");
    assert_eq!(
        t.block,
        (zx.floor() as i32, zy.floor() as i32, zz.floor() as i32),
        "the block the spot floors"
    );
    // THE BODY THE CHASE FACES (the atan2 law the +X walk answers) —
    // the WALK ANIMATION the move distance fills, the EASTWARD walk
    // the x serves
    assert!(
        (app.wardens[0].yaw_deg + 90.0).abs() < 1e-3,
        "the atan2 law the chase yaw wears: {}",
        app.wardens[0].yaw_deg
    );
    assert!(
        app.wardens[0].walk.speed > 0.0,
        "the walk speed the move distance fills"
    );
    assert!(
        app.wardens[0].x > px - 2.0,
        "the eastward walk the x serves: {}",
        app.wardens[0].x
    );
    // THE CHASE THE GAP CLOSES (the zombie's own player-chase the -X
    // walk serves, the warden the +X chase rides — the arrival the
    // melee ring the dose answers)
    let mut dose_seen = false;
    for _ in 0..200 {
        app.update();
        if app.mobs[0].health < 500.0 {
            dose_seen = true;
            break;
        }
    }
    assert!(dose_seen, "the arrival the dose lands");
    // THE DOSE (the 29.52 the ARMOR LAW wears — the zombie's own 2
    // armor the CombatRules fold speaks: f6 = clamp(2 - 30/2, 0.4, 20)
    // = 0.4, the survivor `30 * (1 - 0.4/25)` the 0.984 factor answers)
    assert_eq!(
        app.mobs[0].health, 470.48,
        "the bite the armor-absorbed dose wears: {}",
        app.mobs[0].health
    );
    // THE ERASE (lambda$create$3 pc 52-56 — the in-reach pair the
    // WALK_TARGET drop serves: the arrival the warden stands, the
    // swing the same tick the melee arm wears)
    assert!(
        app.wardens[0].walk_target.is_none(),
        "the in-reach pair the arrival erase serves"
    );
    // THE SWING'S OWN BANK (the 18l the cooldown banks)
    assert_eq!(
        app.wardens[0].attack_cooldown,
        crate::sculk::WARDEN_MELEE_COOLDOWN_TICKS,
        "the 18l the setWithExpiry banks"
    );
    // THE IMPACT VOICE (the 10.0 the landing speaks)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.contains("entity.warden.attack_impact")),
        "the impact voice the landing plays"
    );
    // THE REFERENCE (the setLastHurtByMob the warden's own view banks
    // — the id the 201 the kind the WARDEN word carries)
    let reference = app.mobs[0]
        .hurt_by_reference()
        .expect("the bite the reference banks");
    assert_eq!(reference.id, 201, "the warden's id the reference names");
    assert_eq!(
        reference.kind,
        crate::mob::MobKind::Warden,
        "the kind the variant banks"
    );
    // THE RETALIATION (the HurtBy prio-1 goal the next tick's walk
    // arms — the loop the counter-bite the warden's own hurtServer
    // answers)
    app.update();
    assert!(
        app.mobs[0].retaliation_armed(),
        "the HurtBy goal the chase's dose arms"
    );
}

/// THE ALERT'S OWN PACK WALK (javap `HurtByTargetGoal.alertOthers`
/// + `alertOther(m, getLastHurtByMob())` over the offline world —
/// the warden-source dose the victim's own start event arms, the
/// sibling pass the same-kind box serves, the setTarget(attacker)
/// write the ALERTED sibling's own retaliation lane rides: the
/// pack the WARDEN's own view chases (the hurt memory the write
/// never rides — the canUse's timestamp pair the EQUAL words keeps,
/// the cascade the vanilla law never arms), the chase the melee
/// ring the pack's own fangs answer (the +100 the sibling's key
/// banks, the suspect78 row the dose's own walk records), the
/// standing ATTACK_TARGET the hurt walk's empty-brain gate keeps).
#[test]
fn the_alert_arms_the_pack_the_warden_the_sibling_chases() {
    let mut app = App::new(3);
    app.enter_world();
    // THE FLAT FLOOR (the chase test's own world the app wears —
    // the stone 0..5 the air above, the ROOF 12..14 the sky column
    // breaks — the burn roll's canSeeSky the broken column refuses:
    // the zombie the sun never brands, the 500.0 pool the health
    // law the exact keeps)
    let mut flat = crate::world::World::new(WORLD_CHUNKS, app.seed);
    for x in 0..flat.size_blocks() {
        for z in 0..flat.size_blocks() {
            for y in 0..5 {
                flat.set(x, y as i32, z, crate::world::STONE);
            }
            for y in 5..12 {
                flat.set(x, y as i32, z, crate::world::AIR);
            }
            // THE ROOF THE BURN ROLL'S SKY GATE REFUSES (the eye
            // column the 12..14 band breaks: the sky channel the
            // canSeeSky word never reads 15)
            for y in 12..14 {
                flat.set(x, y as i32, z, crate::world::STONE);
            }
            for y in 14..crate::world::CHUNK_Y {
                flat.set(x, y as i32, z, crate::world::AIR);
            }
        }
    }
    app.offline_world = Some(flat);
    app.spawn_y = 5;
    // THE WALL THE SCAN'S OWN REFUSAL KEEPS (the player west, the
    // pack east — the eye rays the wall's 6..9 band blocks: the
    // prio-2 scan never arms — the alert's getTarget() != null
    // gate the fresh sibling the alert's own reach arms)
    {
        let world = app.offline_world.as_mut().unwrap();
        for y in 6..=9 {
            for z in 2..=18 {
                world.set(16, y, z, crate::world::STONE);
                world.set(17, y, z, crate::world::STONE);
            }
        }
    }
    // THE STAGE (the player west of the wall, the pack east: the
    // victim the warden's 3.0 west gap keeps, the sibling 3.0
    // south — the alert box the victim's own follow range the
    // sibling's spot answers)
    app.body = Some(crate::body::PlayerBody::new(10.5, 5.0, 10.5, 180.0, 0.0));
    app.mobs
        .push(crate::mob::Zombie::new(77, 20.5, 5.0, 10.5, 4));
    app.mobs
        .push(crate::mob::Zombie::new(78, 20.5, 5.0, 13.5, 5));
    app.mobs[0].on_ground = true;
    app.mobs[1].on_ground = true;
    app.update();
    // THE POOL THE REPEAT SWINGS SURVIVE (the chase test's own law)
    app.mobs[0].health = 500.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(201, 23.5, 5.0, 10.5));
    app.wardens[0].attack_target = Some(77);
    // THE DIG COOLDOWN THE PROVOKED LAW KEEPS (the counter-bite
    // test's own law — the provoked body the calm-dig refuses),
    // THE SONIC BANK THE FRESH WINDOW KEEPS (the chase the pure
    // lane keeps — the 40l the walk the whole covers)
    app.wardens[0].dig_cooldown = crate::sculk::WARDEN_DIG_COOLDOWN_TICKS;
    app.wardens[0].sonic_boom_cooldown = crate::sculk::WARDEN_ATTACK_SONIC_COOLDOWN_TICKS;
    // THE CHASE THE GAP CLOSES (the 578 test's own law — the
    // arrival the 29.52 the armor law lands)
    let mut dose_seen = false;
    for _ in 0..200 {
        app.update();
        if app.mobs[0].health < 500.0 {
            dose_seen = true;
            break;
        }
    }
    assert!(dose_seen, "the arrival the dose lands");
    assert_eq!(
        app.mobs[0].health, 470.48,
        "the bite the armor-absorbed dose wears: {}",
        app.mobs[0].health
    );
    // THE ALERT'S OWN TICK (the victim's own start the event fires
    // — the sibling pass the same update's tail serves)
    app.update();
    // THE SIBLING THE ALERT ARMS (the setTarget write the Some arm
    // serves — the warden's own view the pack's chase names)
    assert!(app.mobs[1].retaliation_armed(), "the pack the alert arms");
    let alerted = app.mobs[1]
        .alerted_reference()
        .expect("the setTarget write banks");
    assert_eq!(alerted.id, 201, "the warden's id the alert names");
    assert_eq!(
        alerted.kind,
        crate::mob::MobKind::Warden,
        "the kind the variant banks"
    );
    assert!(
        (alerted.x - app.wardens[0].x).abs() < 1.0,
        "the live pos the snapshot wears: {} vs {}",
        alerted.x,
        app.wardens[0].x
    );
    // THE NO-CASCADE (the sibling's own hurt memory the setTarget
    // write never rides — the canUse's timestamp pair the EQUAL
    // words keeps: the alert the target alone arms, the HurtBy
    // goal's own start the sibling's hurt-less body never speaks)
    assert!(
        app.mobs[1].hurt_by_reference().is_none(),
        "the alert the hurt memory never rides"
    );
    // THE VICTIM'S OWN REFERENCE (the 576 law the dose banks — the
    // warden's own view the victim's retaliation names)
    assert!(
        app.mobs[0].hurt_by_reference().is_some(),
        "the dose the victim's reference banks"
    );
    // THE CHASE (the sibling the warden's own spot walks — the
    // approach the alerted lane serves: the warden's own coast the
    // dose's wake keeps, the gap the sibling's own walk CLOSES)
    let wpos = (app.wardens[0].x, app.wardens[0].y, app.wardens[0].z);
    let dist0 = {
        let dx = wpos.0 - app.mobs[1].x;
        let dz = wpos.2 - app.mobs[1].z;
        (dx * dx + dz * dz).sqrt()
    };
    for _ in 0..12 {
        app.update();
    }
    let dist1 = {
        let dx = app.wardens[0].x - app.mobs[1].x;
        let dz = app.wardens[0].z - app.mobs[1].z;
        (dx * dx + dz * dz).sqrt()
    };
    assert!(
        dist1 < dist0,
        "the chase the warden's own spot closes: {} -> {}",
        dist0,
        dist1
    );
    // THE PACK'S OWN BITE (the +100 the sibling's key banks — the
    // suspect78 +100 row the dose's own walk records; the steps'
    // own +35 rows the hearing walk banks the bite's row never
    // speaks)
    let mut sib_bite = false;
    for _ in 0..200 {
        app.update();
        if app
            .game_events
            .iter()
            .any(|e| e.starts_with("warden_anger @") && e.contains("suspect78 +100"))
        {
            sib_bite = true;
            break;
        }
    }
    assert!(
        sib_bite,
        "the pack's own bite the anger row records: {:?}",
        app.game_events
            .iter()
            .filter(|e| e.contains("suspect78"))
            .collect::<Vec<_>>()
    );
    // THE WARDEN'S OWN HEALTH (the pack's fangs the raw no-armor
    // body wears — the 3.0 the zombie's bare fangs deal)
    assert!(
        app.wardens[0].health < crate::sculk::WARDEN_MOB_MAX_HEALTH,
        "the pack's bite the warden's pool wears: {}",
        app.wardens[0].health
    );
    // THE BRAIN'S OWN KEEP (the standing ATTACK_TARGET the hurt
    // walk's empty-brain gate never re-banks — the victim's id the
    // brain holds, the sibling's +100 the anger map alone banks)
    assert_eq!(
        app.wardens[0].attack_target,
        Some(77),
        "the standing target the brain keeps"
    );
}

/// THE COUNTER-BITE'S OWN WALK (javap `Warden.hurtServer` pc 0-103
/// over the offline world — the zombie's own bite the MOB-SOURCE
/// dose wears, the +100 the ON_HURT_ANGER_BOOST the ANGRY_MIN
/// rides, the EMPTY brain + the closerThan(5.0) direct bite the
/// setAttackTarget arm serves: the ATTACK_TARGET the mob id banks,
/// the 200l the SonicBoom.setCooldown banks, the KNOCKBACK the
/// resistance's own 1.0 the push never moves).
#[test]
fn the_counter_bite_the_warden_the_anger_the_target_banks() {
    let mut app = App::new(3);
    app.enter_world();
    // THE AIR NEIGHBORHOOD (the dose tests' own stage)
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    // THE WARDEN THE COUNTER-BITE'S OWN BODY KEEPS (the +X
    // neighbor the zombie's fangs reach), THE REFERENCE THE
    // PRIOR BITE BANKED (the hurt walk the dose banks — the
    // zombie's own walk the retaliation chase serves). THE
    // DIG COOLDOWN THE PROVOKED LAW KEEPS (javap `WardenAi
    // .setDigCooldown` — the provoked body the 1200-tick
    // window the increase walks bank, the calm-dig the gates
    // refuse: the fresh warden the calm walk the dig arms,
    // the bite the digging pose the anger walk refuses — the
    // setup the provoked state honest-arms)
    app.wardens
        .push(crate::sculk::WardenMob::new(201, px + 1.0, feet_y, pz));
    app.wardens[0].dig_cooldown = crate::sculk::WARDEN_DIG_COOLDOWN_TICKS;
    app.mobs
        .push(crate::mob::Zombie::new(77, px, feet_y, pz, 4));
    app.mobs[0].on_ground = true;
    // THE AGE THE SETTER'S OWN WORD ARMS (the setLastHurtByMob the
    // tickCount the dose's own moment banks — the fresh mob's age
    // the first tick advances, the reacted-event timestamp the
    // start's own compare the unequal pair answers: the age-0
    // write the goal's start the EQUAL words never arm)
    app.update();
    app.mobs[0].set_hurt_by_reference(crate::mob::warden_prey_view(&app.wardens[0]));
    // THE CHASE + THE BITE (the zombie's own melee walk the
    // warden's view serves — the 20-tick interval the first swing
    // the window answers; THE STEP VIBRATIONS the hearing lane's
    // own +35 the walk banks — the anger the vibration walk
    // arrives BEFORE the bite's own dose, the health the exit
    // word answers)
    let mut bit = false;
    for tick in 0..40 {
        app.update();
        if app.wardens[0].health < crate::sculk::WARDEN_MOB_MAX_HEALTH {
            bit = true;
            break;
        }
    }
    assert!(bit, "the counter-bite the dose serves");
    // THE ANGER (the +100 the mob-id key banks — the game_event
    // row the walk records; THE STEP VIBRATIONS the hearing
    // lane's own +35 the store PRE-banks at the zombie's key —
    // the walk the chase's own steps speak, the +100 the dose's
    // own walk ADDS, the highest the sum answers)
    assert!(
        app.wardens[0].active_anger()
            >= crate::sculk::WARDEN_ANGER_ANGRY_MIN + crate::sculk::WARDEN_ON_HURT_ANGER_BOOST,
        "the +100 the ANGRY_MIN + the boost the dose banks: {}",
        app.wardens[0].active_anger()
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("warden_anger @") && e.contains("suspect77")),
        "the mob-id suspect the record names: {:?}",
        app.game_events
            .iter()
            .filter(|e| e.contains("warden_anger"))
            .collect::<Vec<_>>()
    );
    // THE SETATTACKTARGET ARM (the EMPTY brain the walk consults —
    // the mob id the ATTACK_TARGET banks, the 200l the
    // SonicBoom.setCooldown banks)
    assert_eq!(
        app.wardens[0].attack_target,
        Some(77),
        "the mob id the ATTACK_TARGET banks"
    );
    assert_eq!(
        app.wardens[0].attack_cooldown,
        crate::sculk::WARDEN_SET_TARGET_SONIC_COOLDOWN_TICKS,
        "the 200l the setCooldown bank speaks"
    );
    // THE DOSE (the 3.0 the bare fangs deal — the raw no-armor
    // body wears). THE KNOCKBACK the resistance's own 1.0 the
    // walk NEVER serves — no push call the bite walk carries
    // (the `*(1.0 - 1.0)` zero the LivingEntity.knockback fold
    // keeps); the body's own investigate walk the position the
    // hearing lanes' own drift moves — the positional assert the
    // walk's own mobility the honest test never pins.
    assert_eq!(
        app.wardens[0].health,
        crate::sculk::WARDEN_MOB_MAX_HEALTH - 3.0,
        "the raw dose the health wears: {}",
        app.wardens[0].health
    );
    // THE VOICE (getHurtSound WARDEN_HURT — the v4.0 the
    // getSoundVolume fold wears)
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("entity.warden.hurt ")),
            "the warden's own hurt voice: {:?}",
            app.sounds.events
        );
    }
}

/// THE FIGHT'S OWN LOOK (javap `MeleeAttackGoal.tick` pc 13-25 +
/// `LookControl.tick` + `BodyRotationControl.clientTick` over the
/// offline world — the armed target the look-at banks, the ±30 the
/// first tick the clamp bounds, the O-pair the pre-tick snapshot
/// keeps, the swing the body the ±75 turn serves: the standing
/// fight the warden the TARGET the head and body both wear — the
/// stale yaw the old law never turned).
#[test]
fn the_fight_look_the_head_the_body_the_target_wears() {
    let mut app = App::new(3);
    app.enter_world();
    // THE AIR NEIGHBORHOOD (the dose test's own stage — the boxes
    // the 1.0 gap the reach intersects, the swing the same window
    // lands)
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    // THE WARDEN THE +X NEIGHBOR, THE STALE YAW (the fresh spawn's
    // 0.0 the +Z facing keeps — the +X target the look NEVER wore,
    // the old law the yaw the walk direction alone turned)
    app.wardens
        .push(crate::sculk::WardenMob::new(201, px + 1.0, feet_y, pz));
    app.wardens[0].attack_target = Some(crate::sculk::WARDEN_PLAYER_SUSPECT);
    // THE SONIC BANK THE FRESH WINDOW KEEPS (the melee look the
    // sonic-free window serves — the SINK the memory serves: the
    // fight's bank the eye tracker arms, the ±10/±40 the mob's own
    // words answer — the 26.2 FIGHT list the MeleeAttack BEHAVIOR
    // keeps, the MeleeAttackGoal's ±30/±30 the warden never wears)
    app.wardens[0].sonic_boom_cooldown = crate::sculk::WARDEN_ATTACK_SONIC_COOLDOWN_TICKS;
    app.update();
    // THE HEAD (the ±10 the first tick bounds — the wanted +90 the
    // -X wing answers: atan2(dz=0, dx=-1)·C − 90, the sink's own
    // getHeadRotSpeed the exact 10.0 the clamp wears)
    assert_eq!(
        app.wardens[0].head_yaw, 10.0,
        "the +90 wanted the ±10 sink clamp bounds the first tick"
    );
    // THE O-PAIR (the pre-tick snapshot the render lerp keeps —
    // the fresh spawn's 0.0 the O banks)
    assert_eq!(app.wardens[0].head_yaw_o, 0.0);
    // THE PITCH (the player's eye BELOW the warden's own: the
    // −0.845 dy the +40.26 wanted wears, the ±40 the getMaxHead
    // XRot the first tick bounds — the exact 40.0 the clamp wears)
    assert_eq!(
        app.wardens[0].head_pitch, 40.0,
        "the below eye the pitch the ±40 sink turn bounds"
    );
    // THE BODY (the standing fight the swing arm keeps SILENT —
    // the 10° first-tick swing the |head − lastStable| > 15 gate
    // never answers: the standing body the small swing never
    // turns — the sink's own pace the body's ±75 swing the later
    // stable walks keep)
    assert_eq!(
        app.wardens[0].yaw_deg, 0.0,
        "the 10° swing the 15° gate the body never turns"
    );
}

/// THE CALM LOOK'S OWN INTEGRATION (javap `SetWardenLookTarget` +
/// `LookAtTargetSink` over the offline world — the disturbance the
/// CORE's trigger banks, the sink's 3-arg the ±10 the first tick
/// wears, the small swing the body the standing body keeps: the
/// calm warden the DISTURBANCE the head turns — the stealth
/// gameplay the sound's own echo answers).
#[test]
fn the_calm_look_the_disturbance_the_head_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    // THE WARDEN THE +X NEIGHBOR, THE CALM BODY (the attack target
    // never armed — the CORE's own look the trigger serves)
    app.wardens
        .push(crate::sculk::WardenMob::new(201, px + 1.0, feet_y, pz));
    // THE DISTURBANCE (the ONE-BLOCK wing the vibration rode — the
    // Manhattan 1 the investigate's close-enough 2 REFUSES (the
    // walk the calm body never starts), the CORE's own look the
    // distance gate never carries: the bank the sink serves)
    app.wardens[0].disturbance = Some(crate::sculk::WardenDisturbance {
        pos: (px as i32 + 2, feet_y as i32, pz as i32),
        remaining: 100,
    });
    app.update();
    // THE BANK (the bottom-center the BlockPosTracker wears — the
    // disturbance block the or() fallback serves)
    assert_eq!(
        app.wardens[0].look_target,
        Some((
            px as i32 as f64 + 2.5,
            feet_y as i32 as f64,
            pz as i32 as f64 + 0.5
        )),
        "the disturbance the trigger banks the bottom-center"
    );
    // THE INVESTIGATE'S OWN REFUSAL (the closer-than walk the walk
    // target never arms — the warden the standing body keeps)
    assert!(
        app.wardens[0].walk_target.is_none(),
        "the Manhattan-1 the investigate trigger refuses"
    );
    // THE SINK'S OWN CLOCK (the first tick the window banks)
    assert_eq!(
        app.wardens[0].look_sink_ticks, 1,
        "the first tick the sink's window banks"
    );
    // THE HEAD (the ±10 the 3-arg the mob's own getHeadRotSpeed
    // serves — the wanted −90 the +X wing answers: the slower turn
    // the fight's ±30 never wears)
    assert_eq!(
        app.wardens[0].head_yaw, -10.0,
        "the −90 wanted the ±10 clamp bounds the first tick"
    );
    // THE BODY (the 10° swing the 15.0f threshold refuses — the
    // calm warden the body the standing body keeps)
    assert_eq!(
        app.wardens[0].yaw_deg, 0.0,
        "the ≤15 swing the body the standing body keeps"
    );
}

/// THE BITE'S OWN MOB DOSE (javap `Mob.doHurtTarget` pc 49-60 over
/// the offline world — the warden the MOB target the reach's own
/// box law bites: the 30.0 the health wears through the hurt
/// pipeline's armor walk, the 1.5 push the facing speaks through
/// the mob's own knockback walk, the impact voice the landing
/// plays, the 18l the cooldown banks — the player arm the None
/// keeps the same swing the pair serves).
#[test]
fn the_warden_bite_the_mob_the_dose_the_push_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    // THE CREATIVE STAND-IN (the stab test's own law — the zombie's
    // own player-scan the creative word refuses: the hunt walk the
    // stage's walk keeps silent, the bite the warden's own dose
    // alone speaks)
    app.game_mode = GameMode::Creative;
    // THE AIR NEIGHBORHOOD (the dose test's own stage the eye ray
    // never fears — the zombie the +X neighbor the boxes the same
    // 1.0 gap the player arm intersects)
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    app.mobs
        .push(crate::mob::Zombie::new(777, px + 1.0, feet_y, pz, 4));
    // THE HEALED STAND-IN (the 100.0 pool the one-shot bite
    // refuses — the dose the SURVIVOR the walk serves)
    app.mobs[0].health = 100.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(201, px, feet_y, pz));
    app.wardens[0].attack_target = Some(777);
    // THE YAW THE +X FACING WEARS (the zombie the target the
    // facing answers — the push AWAY the bite speaks)
    app.wardens[0].yaw_deg = -90.0;
    // THE SONIC BANK THE FRESH WINDOW KEEPS (the dose test's own
    // law — the index-4 the melee prio-15 precedes, the charge
    // the standing window arms, the swing the same tick the bank
    // serves)
    app.wardens[0].sonic_boom_cooldown = crate::sculk::WARDEN_ATTACK_SONIC_COOLDOWN_TICKS;
    let health_before = app.mobs[0].health;
    let (vx_before, vz_before) = (app.mobs[0].vx, app.mobs[0].vz);
    app.update();
    // THE DOSE (the 30.0 the armor walk wears — the CombatRules
    // fold the hurt_shared body keeps: the zombie's own 2.0 armor
    // the 30/2 the divisor answers, the 0.4 floor the clamp keeps)
    let f6 = (crate::mob::ZOMBIE_ARMOR as f32 - 30.0f32 / 2.0)
        .clamp((crate::mob::ZOMBIE_ARMOR * 0.2) as f32, 20.0);
    assert_eq!(
        app.mobs[0].health,
        health_before - 30.0 * (1.0 - f6 / 25.0),
        "the bite the armor walk wears"
    );
    // THE KNOCKBACK (the +X push the facing wears — the minus law
    // the mob's own walk keeps: the facing (sin, -cos) the
    // vx/2 - n fold the push answers; the +Z the cos never wears)
    assert!(
        app.mobs[0].vx - vx_before > 0.5,
        "the 1.5 push the +X facing wears: {} vs {}",
        app.mobs[0].vx,
        vx_before
    );
    assert!(
        (app.mobs[0].vz - vz_before).abs() < 1e-6,
        "the push the +Z the facing never wears: {} vs {}",
        app.mobs[0].vz,
        vz_before
    );
    // THE IMPACT VOICE (the 10.0 the landing speaks)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.contains("entity.warden.attack_impact")),
        "the impact voice the landing plays"
    );
    // THE SWING'S OWN BANKS (the 18l the cooldown banks — the
    // player arm the same swing the pair wears)
    assert_eq!(
        app.wardens[0].attack_cooldown,
        crate::sculk::WARDEN_MELEE_COOLDOWN_TICKS,
        "the 18l the setWithExpiry banks"
    );
    // THE PLAYER THE BITE NEVER WEARS (the mob dose the player's
    // own pool refuses — the hurt the mob victim alone speaks)
    assert_eq!(app.health, 20.0, "the player the bite never wears");
}

/// THE BITE'S OWN REACH GATE (the MeleeAttack's own box law — the
/// out-of-reach target the swing refuses: the intersect the
/// expanded box answers FALSE, the dose the refused swing never
/// lands, the cooldown the landing never banks).
#[test]
fn the_warden_bite_the_out_of_reach_the_swing_refuses() {
    let mut app = App::new(3);
    app.enter_world();
    // THE CREATIVE STAND-IN (the dose test's own law — the scan
    // the creative word refuses, the hunt walk the stage stills)
    app.game_mode = GameMode::Creative;
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    // THE +3.0 ZOMBIE (the gap the reach box answers FALSE — the
    // warden's 0.45 + 0.828 + 0.3 ring the 2.7 gap refuses)
    app.mobs
        .push(crate::mob::Zombie::new(778, px + 3.0, feet_y, pz, 4));
    app.mobs[0].health = 100.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(202, px, feet_y, pz));
    app.wardens[0].attack_target = Some(778);
    app.wardens[0].yaw_deg = -90.0;
    app.wardens[0].sonic_boom_cooldown = crate::sculk::WARDEN_ATTACK_SONIC_COOLDOWN_TICKS;
    app.update();
    // THE REFUSAL (the health the untouched pool keeps, the
    // cooldown the landing never banks — the zombie's own stroll
    // the idle brain owns, the velocity the refused swing never
    // consults)
    assert_eq!(
        app.mobs[0].health, 100.0,
        "the out-of-reach the dose refuses"
    );
    assert_eq!(
        app.wardens[0].attack_cooldown, 0,
        "the refused swing the cooldown never banks"
    );
}

/// THE BITE'S OWN KILL (the die() tail the lethal bite serves —
/// the death voice the makeSound speaks, the loot the
/// dropAllDeathLoot rolls at the entity's feet, the XP the closed
/// lastHurtByPlayerMemoryTime gate never awards, the corpse the
/// repeat bite refuses).
#[test]
fn the_warden_bite_the_kill_the_loot_the_corpse_refuses() {
    let mut app = App::new(3);
    app.enter_world();
    // THE CREATIVE STAND-IN (the dose test's own law — the scan
    // the creative word refuses, the bite the warden's own dose
    // alone speaks)
    app.game_mode = GameMode::Creative;
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    app.mobs
        .push(crate::mob::Zombie::new(779, px + 1.0, feet_y, pz, 4));
    // THE WOUNDED BODY (the 10.0 pool the 30.0 bite the death
    // walk serves)
    app.mobs[0].health = 10.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(203, px, feet_y, pz));
    app.wardens[0].attack_target = Some(779);
    app.wardens[0].yaw_deg = -90.0;
    app.wardens[0].sonic_boom_cooldown = crate::sculk::WARDEN_ATTACK_SONIC_COOLDOWN_TICKS;
    app.update();
    // THE DEATH (the health the lethal dose sinks, the corpse the
    // topple walk keeps)
    assert!(
        app.mobs[0].health <= 0.0,
        "the lethal bite the death walk serves"
    );
    // THE LOOT (the dropAllDeathLoot roll — the entity's feet the
    // spawnAtLocation law serves)
    assert!(
        !app.drops.is_empty(),
        "the kill the loot roll speaks: {}",
        app.drops.len()
    );
    // THE XP THE CLOSED GATE NEVER AWARDS (the warden's bite the
    // player memory never banks — the fire-kill's own precedent
    // the closed reward wears)
    assert!(
        app.orbs.is_empty(),
        "the non-player kill the reward never rolls: {}",
        app.orbs.len()
    );
    // THE CORPSE THE REPEAT BITE REFUSES (the swing the
    // cooldown-erased re-arm keeps, the dead body the dose
    // refuses — the loot the second roll never wears)
    let drops_after_kill = app.drops.len();
    app.wardens[0].attack_cooldown = 0;
    app.update();
    assert_eq!(
        app.drops.len(),
        drops_after_kill,
        "the corpse the repeat bite refuses"
    );
    // THE DEATH VOICE (the die() tail the makeSound speaks — the
    // audio assets the CI body keeps absent the honest skip)
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.contains("entity.zombie.death")),
            "the kill the death voice speaks: {:?}",
            app.sounds
                .events
                .iter()
                .filter(|e| e.contains("zombie"))
                .collect::<Vec<_>>()
        );
    }
}

/// THE SONIC BOOM'S OWN MOB DOSE (javap `SonicBoom.lambda$tick$2`
/// pc 121-137 over the offline world — the warden the MOB target
/// the mid-window boom serves: the 10.0 the health wears through
/// the hurt pipeline's armor walk, the beam push the plain add
/// speaks, the hurt voice the pipeline's makeSound plays, the 26l
/// the fired boom banks — the player's own pool the mob victim
/// never wears).
#[test]
fn the_sonic_boom_the_mob_the_dose_the_push_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    // THE CREATIVE STAND-IN (the bite test's own law — the zombie's
    // own player-scan the creative word refuses: the hunt walk the
    // stage's walk keeps silent, the boom the warden's own dose
    // alone speaks)
    app.game_mode = GameMode::Creative;
    // THE AIR NEIGHBORHOOD (the dose test's own stage the eye ray
    // never fears — the zombie the +X neighbor the beam the
    // chest→eye walk serves)
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    app.mobs
        .push(crate::mob::Zombie::new(777, px + 1.0, feet_y, pz, 4));
    // THE HEALED STAND-IN (the 100.0 pool the one-shot boom
    // refuses — the dose the SURVIVOR the walk serves)
    app.mobs[0].health = 100.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(204, px, feet_y, pz));
    app.wardens[0].attack_target = Some(777);
    // THE MID-WINDOW STATE (the rings test's own surgical setup —
    // the active the running state keeps, the 2 the window's own
    // tail banks, the 1 the delay's last tick covers — the update's
    // drain walk the both-absent tick the boom arms; the standing
    // window the melee swing the same tick refuses)
    app.wardens[0].sonic_active = true;
    app.wardens[0].attack_cooldown = 2;
    app.wardens[0].sonic_sound_delay = 1;
    app.wardens[0].sonic_sound_cooldown = 0;
    let health_before = app.mobs[0].health;
    let (vx_before, vz_before) = (app.mobs[0].vx, app.mobs[0].vz);
    app.update();
    // THE DOSE (the 10.0 the armor walk wears — the CombatRules
    // fold the hurt_shared body keeps: the zombie's own 2.0 armor
    // the 10/2 the divisor answers, the 0.4 floor the clamp keeps)
    let f6 = (crate::mob::ZOMBIE_ARMOR as f32 - 10.0f32 / 2.0)
        .clamp((crate::mob::ZOMBIE_ARMOR * 0.2) as f32, 20.0);
    assert_eq!(
        app.mobs[0].health,
        health_before - 10.0 * (1.0 - f6 / 25.0),
        "the boom the armor walk wears"
    );
    // THE PUSH (the beam the +X direction wears — the plain add
    // the Entity.push keeps: the 2.5 the horizontal the
    // KNOCKBACK_HORIZONTAL carries, the +Z the beam never wears)
    assert!(
        app.mobs[0].vx - vx_before > 0.5,
        "the 2.5 push the +X beam wears: {} vs {}",
        app.mobs[0].vx,
        vx_before
    );
    assert!(
        (app.mobs[0].vz - vz_before).abs() < 1e-6,
        "the push the +Z the beam never wears: {} vs {}",
        app.mobs[0].vz,
        vz_before
    );
    // THE HURT VOICE (the hurtServer's makeSound speaks — the
    // audio assets the CI body keeps absent the honest skip)
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.contains("entity.zombie.hurt")),
            "the dose the hurt voice speaks: {:?}",
            app.sounds
                .events
                .iter()
                .filter(|e| e.contains("zombie"))
                .collect::<Vec<_>>()
        );
    }
    // THE PLAYER THE BOOM NEVER WEARS (the mob victim the pair
    // keeps — the player's own pool the Some arm never consults)
    assert_eq!(app.health, 20.0, "the player the mob dose never wears");
    // THE BOOM'S OWN BANKS (the 26l the fired boom banks)
    assert_eq!(
        app.wardens[0].sonic_sound_cooldown,
        crate::sculk::WARDEN_SONIC_SOUND_COOLDOWN_TICKS,
        "the 26l the fired boom banks"
    );
}

/// THE SONIC BOOM'S OWN KILL (the die() tail the lethal boom
/// serves — the death voice the makeSound speaks, the loot the
/// dropAllDeathLoot rolls at the entity's feet, the XP the closed
/// lastHurtByPlayerMemoryTime gate never awards, the corpse the
/// repeat boom refuses).
#[test]
fn the_sonic_boom_the_kill_the_loot_the_corpse_refuses() {
    let mut app = App::new(3);
    app.enter_world();
    // THE CREATIVE STAND-IN (the dose test's own law — the scan
    // the creative word refuses, the boom the warden's own dose
    // alone speaks)
    app.game_mode = GameMode::Creative;
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    app.mobs
        .push(crate::mob::Zombie::new(779, px + 1.0, feet_y, pz, 4));
    // THE WOUNDED BODY (the 9.0 pool the 9.84 armor-folded dose
    // the death walk serves)
    app.mobs[0].health = 9.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(205, px, feet_y, pz));
    app.wardens[0].attack_target = Some(779);
    app.wardens[0].sonic_active = true;
    app.wardens[0].attack_cooldown = 2;
    app.wardens[0].sonic_sound_delay = 1;
    app.wardens[0].sonic_sound_cooldown = 0;
    app.update();
    // THE DEATH (the health the lethal dose sinks)
    assert!(
        app.mobs[0].health <= 0.0,
        "the lethal boom the death walk serves"
    );
    // THE LOOT (the dropAllDeathLoot roll — the entity's feet the
    // spawnAtLocation law serves)
    assert!(
        !app.drops.is_empty(),
        "the kill the loot roll speaks: {}",
        app.drops.len()
    );
    // THE XP THE CLOSED GATE NEVER AWARDS (the warden's boom the
    // player memory never banks — the bite kill's own precedent
    // the closed reward wears)
    assert!(
        app.orbs.is_empty(),
        "the non-player kill the reward never rolls: {}",
        app.orbs.len()
    );
    // THE CORPSE THE REPEAT BOOM REFUSES (the window the fresh
    // arm keeps, the dead body the dose refuses — the loot the
    // second roll never wears)
    let drops_after_kill = app.drops.len();
    app.wardens[0].sonic_active = true;
    app.wardens[0].attack_cooldown = 2;
    app.wardens[0].sonic_sound_delay = 1;
    app.wardens[0].sonic_sound_cooldown = 0;
    app.update();
    assert_eq!(
        app.drops.len(),
        drops_after_kill,
        "the corpse the repeat boom refuses"
    );
    // THE DEATH VOICE (the die() tail the makeSound speaks — the
    // audio assets the CI body keeps absent the honest skip)
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.contains("entity.zombie.death")),
            "the kill the death voice speaks: {:?}",
            app.sounds
                .events
                .iter()
                .filter(|e| e.contains("zombie"))
                .collect::<Vec<_>>()
        );
    }
}

/// THE SONIC BOOM'S OWN PLAYER DOSE (the None arm the pre-victim
/// rounds kept — the 10.0 the health wears, the beam push the
/// plain add speaks, the mob-less world the Some arm never
/// consults: the victim pair's refactor the walk guards).
#[test]
fn the_sonic_boom_the_player_the_dose_the_push_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    // THE SURVIVAL STAND-IN (the hurt pipeline's own gate — the
    // creative word the dose refuses)
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    // THE WARDEN THE +X FIVE (the rings test's own staging — the
    // XZ 5 the 15-range keeps, the feet dy 0 the 20-range answers)
    let wx = px + 5.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(206, wx, feet_y, pz));
    app.wardens[0].attack_target = Some(crate::sculk::WARDEN_PLAYER_SUSPECT);
    app.wardens[0].sonic_active = true;
    app.wardens[0].attack_cooldown = 2;
    app.wardens[0].sonic_sound_delay = 1;
    app.wardens[0].sonic_sound_cooldown = 0;
    let health_before = app.health;
    let (vx_before, vz_before) = {
        let b = app.body.as_ref().expect("the body rides");
        (b.vx, b.vz)
    };
    app.update();
    // THE DOSE (the 10.0 the bare pool wears — the armor the
    // default body never wears)
    assert!(
        app.health < health_before,
        "the sonic dose the health wears: {} vs {}",
        app.health,
        health_before
    );
    // THE PUSH (the beam the -X direction wears — the plain add
    // the Entity.push keeps: the chest→eye walk the -X the
    // warden's +X staging answers, the +Z the beam never wears)
    let (vx, vz) = {
        let b = app.body.as_ref().expect("the body rides");
        (b.vx, b.vz)
    };
    assert!(
        vx - vx_before < -0.5,
        "the 2.5 push the -X beam wears: {} vs {}",
        vx,
        vx_before
    );
    assert!(
        (vz - vz_before).abs() < 1e-6,
        "the push the +Z the beam never wears: {} vs {}",
        vz,
        vz_before
    );
}

/// THE SONIC BOOM'S OWN BEAM (javap `SonicBoom.lambda$tick$2`
/// pc 0-108 over the 26.2 jar — the mid-window state the
/// surgical setup banks, the both-absent tick the boom arms, the
/// `Mth.floor(len) + 7` walk the exact points plants, the grey
/// draw the base ctor's own walk spends, the 16-tick life the
/// rings wear).
#[test]
fn the_sonic_boom_the_beam_the_rings_plant() {
    let mut app = App::new(3);
    app.enter_world();
    // THE AIR NEIGHBORHOOD (the eye ray the sky never fears — the
    // warden the +X FIVE the closerThan range keeps)
    let feet_y = (app.spawn_y + 6) as f64 - 1.62;
    let (px, pz) = {
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c as f64 + 0.5, c as f64 + 0.5)
    };
    app.body = Some(crate::body::PlayerBody::new(px, feet_y, pz, 180.0, 0.0));
    // THE WARDEN THE +X FIVE (the XZ 5 the 15-range keeps, the
    // feet dy 0 the 20-range answers)
    let wx = px + 5.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(202, wx, feet_y, pz));
    app.wardens[0].attack_target = Some(crate::sculk::WARDEN_PLAYER_SUSPECT);
    // THE MID-WINDOW STATE (the surgical setup the one update the
    // boom arms wears: the active the running state keeps, the 2
    // the window's own tail banks, the 1 the delay's last tick
    // covers — the update's drain walk the both-absent tick the
    // boom arms)
    app.wardens[0].sonic_active = true;
    app.wardens[0].attack_cooldown = 2;
    app.wardens[0].sonic_sound_delay = 1;
    app.wardens[0].sonic_sound_cooldown = 0;
    app.update();
    // THE BOOM'S OWN BANKS (the 26l the fired voice banks, the
    // window the same walk keeps — the 1 the drain's tail wears)
    assert!(
        app.wardens[0].sonic_sound_cooldown == crate::sculk::WARDEN_SONIC_SOUND_COOLDOWN_TICKS,
        "the 26l the fired boom banks"
    );
    assert!(app.wardens[0].sonic_active, "the window the boom keeps");
    // THE BEAM'S OWN RINGS (the chest + the dir * j for j in
    // 1..floor(len) + 7 the exact points plants — the floor(5.0)
    // + 7 = 12 the count the walk serves: 11 rings j = 1..12)
    let chest_y = feet_y + crate::sculk::WARDEN_SONIC_CHEST_OFFSET_Y;
    let eye_y = feet_y + 1.62;
    let dx = px - wx;
    let dy = eye_y - chest_y;
    let len = (dx * dx + dy * dy).sqrt();
    let want_points = (len.floor() as i64) + 7;
    let rings: Vec<&crate::particles::Particle> = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::SonicBoom))
        .collect();
    assert_eq!(
        rings.len(),
        (want_points - 1) as usize,
        "the beam walk the exact count plants"
    );
    for (k, ring) in rings.iter().enumerate() {
        let j = (k + 1) as f64;
        assert_eq!(ring.lifetime, 16, "the fixed 16 the ctor keeps");
        assert_eq!(ring.age, 0, "the fresh ring the spawn banks");
        assert_eq!(ring.vx, 0.0);
        let want_x = wx + (dx / len) * j;
        let want_y = chest_y + (dy / len) * j;
        assert!(
            (ring.x - want_x).abs() < 1e-9 && (ring.y - want_y).abs() < 1e-9,
            "the exact point the walk plants: ring {k} at {} {} vs {} {}",
            ring.x,
            ring.y,
            want_x,
            want_y
        );
    }
    // THE GREY TINT (the 0.4..1.0 band the base ctor's draw wears)
    for ring in &rings {
        assert!(
            ring.col[0] >= 0.4 && ring.col[0] <= 1.0,
            "the grey the draw band keeps"
        );
    }
}

/// THE USE-TICK STAB'S OWN WALK REFUSAL (the 4.6 relative gate the
/// standing/walking body never meets): the 0.2 b/tick lunge (4.0
/// b/s) the relative the damage gate refuses — no hurt, no
/// byte-2, the BANK the remember call still performs (the refused
/// evaluation the clock still spends — the vanilla order the walk
/// banks).
#[test]
fn the_spear_use_tick_the_walk_the_condition_refuses() {
    let mut app = App::new(3);
    app.enter_world();
    let iron = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (iron, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        (app.spawn_y + 7) as f64 - 1.62,
        c as f64 + 0.5,
        180.0,
        0.0,
    ));
    let cam = app.world_camera();
    let f = cam.forward();
    let (ex, ey, ez) = (cam.pos.x as f64, cam.pos.y as f64, cam.pos.z as f64);
    app.mobs.push(crate::mob::Zombie::new(
        200,
        ex + f.x as f64 * 3.0,
        ey - 1.0,
        ez + f.z as f64 * 3.0,
        4,
    ));
    // THE WALK'S OWN SPEED (0.2 b/tick × 20 = 4.0 b/s — under the
    // 4.6 relative gate the damage never meets)
    if let Some(b) = app.body.as_mut() {
        b.vx = 0.2 * f.x as f64;
        b.vz = 0.2 * f.z as f64;
    }
    let pre = app.mobs[0].health;
    app.frame = 30;
    app.clock_total = 30;
    app.spear_use_tick_stab(12); // the iron tier's own 12 the walk passes
    assert_eq!(app.mobs[0].health, pre, "the walk the damage refuses");
    assert_eq!(
        app.stab_contact_memory.len(),
        1,
        "the refused evaluation the clock still spends"
    );
    if !sounds_loaded() {
        return;
    }
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.hit ")),
        "the refused walk the hit voice never wears: {:?}",
        app.sounds.events
    );
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.zombie.hurt ")),
        "the refused walk the hurt voice never wears: {:?}",
        app.sounds.events
    );
}

/// THE HURT LANE'S OWN WARDEN (javap `Warden.hurtServer` pc 0-103
/// over the offline world — the stab the warden's own box the
/// merged pick consults, the hurt the RAW damage the
/// armor-absence wears, the anger the +100 the ANGRY_MIN + the
/// boost the iadd folds, the LISTENING the hurt path NEVER arms,
/// the knockback the resistance's own 1.0 the law refuses, the
/// DIGGING the anger the ifne gate refuses).
#[test]
fn the_stab_the_warden_the_hurt_the_anger_climbs() {
    let mut app = App::new(3);
    app.enter_world();
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // the camera the sky reads (the stab's own ray the block clip
    // never fears)
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    // the warden the camera's own forward serves (3.0 ahead, the
    // feet the eye - 1.0 the box's own span covers)
    let cam = app.world_camera();
    let f = cam.forward();
    let wx = cam.pos.x as f64 + f.x as f64 * 3.0;
    let wy = cam.pos.y as f64 - 1.0;
    let wz = cam.pos.z as f64 + f.z as f64 * 3.0;
    app.wardens
        .push(crate::sculk::WardenMob::new(101, wx, wy, wz));
    // THE STAB (the raw attribute the dmg wears — PLAYER_ATTACK_DAMAGE
    // + the material's own bonus the modifier rides)
    let expected_dmg = (PLAYER_ATTACK_DAMAGE + material_bonus("iron_spear")) as f32;
    let contact = app.stab_walk("iron_spear");
    assert!(contact, "the warden the stab's collection answers");
    assert_eq!(
        app.wardens[0].health,
        500.0 - expected_dmg,
        "the RAW damage the armor-absence keeps (the no-armor attributes the hurt wears)"
    );
    // THE ANGER (+100 -> 100 — the ANGRY_MIN 80 + the boost 20 the
    // iadd folds, the record the POST-increase verdict banks)
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("warden_anger @") && e.contains("+100 -> 100")),
        "the hurt's own +100 the record banks: {:?}",
        app.game_events
    );
    assert_eq!(
        app.wardens[0].active_anger(),
        crate::sculk::WARDEN_ANGER_ANGRY_MIN + crate::sculk::WARDEN_ON_HURT_ANGER_BOOST,
        "the store the +100 the hurt wears"
    );
    // THE HURT VOICE (getHurtSound WARDEN_HURT — the v4.0 the
    // getSoundVolume fold wears)
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("entity.warden.hurt ")),
            "the warden's own hurt voice: {:?}",
            app.sounds.events
        );
    }
    // THE KNOCKBACK the resistance's own 1.0 the law refuses (the
    // position the push never moves)
    assert_eq!(app.wardens[0].x, wx, "the warden the push never moves");
    assert_eq!(app.wardens[0].z, wz, "the z the push never moves");
    // THE DIGGING WARDEN (javap pc 16-20 — the isDiggingOrEmerging
    // gate the ifne pair keeps: the second stab the anger never
    // climbs, the hurt STILL wears)
    app.wardens[0].digging_or_emerging = true;
    let contact = app.stab_walk("iron_spear");
    assert!(contact, "the digging warden the stab still reaches");
    assert_eq!(
        app.wardens[0].health,
        500.0 - 2.0 * expected_dmg,
        "the digging the hurt never refuses"
    );
    assert_eq!(
        app.wardens[0].active_anger(),
        crate::sculk::WARDEN_ANGER_ANGRY_MIN + crate::sculk::WARDEN_ON_HURT_ANGER_BOOST,
        "the digging the anger the gate refuses"
    );
    assert_eq!(
        app.game_events
            .iter()
            .filter(|e| e.starts_with("warden_anger @"))
            .count(),
        1,
        "the second stab the digging silence keeps (no second record)"
    );
}

/// THE STAB'S OWN HULL REACH (the autopush-122 round — javap
/// `PiercingWeapon.attack` pc 27-112 + `ProjectileUtil
/// .getHitEntitiesAlong` the 26.2 jar the vehicles the `isPickable`
/// verdict serves): the collection the offline hulls answer — the
/// too-close hull the 1.875 dead zone refuses, the raw attribute
/// the stabAttack damage passes (the charge scale the melee fold
/// speaks the stab never wears), the wobble decade the hurt trio
/// arms, the damage the amount·10 ramps, the 40.0 threshold the
/// destroy verdict answers (the drop the destroy walk spawns), the
/// knockback the LivingEntity gate never pushes — and the USE-TICK
/// kinetic walk the same hulls the memory keyed by the vehicle's
/// own id serves (the second dose the 10-tick window refuses).
#[test]
fn the_stab_the_hulls_the_reach_the_vehicle_verdict_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // the camera the sky reads (the stab's own ray the block clip
    // never fears)
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
    // THE TOO-CLOSE BOAT (the eye-ahead 1.0 the window's own
    // 1.875 min rides below) + THE CART 3.0 AHEAD (the iron
    // spear's own window the reach answers)
    let bx = ex + f.x as f64 * 1.0;
    let bz = ez + f.z as f64 * 1.0;
    app.offline_boats.push(crate::world_view::WireBoat {
        id: 9,
        x: bx,
        y: ey - 0.4,
        z: bz,
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
    let cx = ex + f.x as f64 * 3.0;
    let cz = ez + f.z as f64 * 3.0;
    app.offline_carts.push(crate::cart::OfflineCart {
        id: 41,
        kind: "minecart",
        cart: crate::cart::Minecart::new([cx, ey - 0.4, cz]),
        hurt_time: 0,
        hurt_dir: 1,
        damage: 0.0,
        fuse: crate::cart::TNT_FUSE_UNPRIMED,
    });
    // THE PICK: the cart the window answers, the too-close boat
    // the dead zone refuses
    let picks = app.stab_pick_all();
    assert_eq!(
        picks,
        vec![StabPick::Cart(0)],
        "the window the cart answers, the boat the 1.875 dead zone refuses"
    );
    // THE STAB (the raw attribute the dmg wears —
    // PLAYER_ATTACK_DAMAGE + the material's own bonus the
    // modifier rides — the charge scale the stab never wears)
    let expected_dmg = (PLAYER_ATTACK_DAMAGE + material_bonus("iron_spear")) as f32;
    let contact = app.stab_walk("iron_spear");
    assert!(contact, "the cart the stab's collection answers");
    assert_eq!(
        app.offline_boats.len(),
        1,
        "the dead-zone boat the stab never serves"
    );
    assert_eq!(
        app.offline_carts.len(),
        1,
        "the first stab the cart survives"
    );
    let cart = &app.offline_carts[0];
    assert!(
        (cart.damage - expected_dmg * 10.0).abs() < 1e-4,
        "the RAW amount the ·10 ramp wears ({})",
        cart.damage
    );
    assert_eq!(
        cart.hurt_time,
        crate::cart::CART_HURT_TICKS,
        "the decade armed"
    );
    assert_eq!(cart.hurt_dir, -1, "the flip the first hit speaks");
    // THE KNOCKBACK the vehicles never wear (the position the
    // push never moves — the LivingEntity gate the walk's own
    // answer keeps)
    assert_eq!(cart.cart.pos[0], cx, "the x the push never moves");
    assert_eq!(cart.cart.pos[2], cz, "the z the push never moves");
    // THE SECOND STAB: the accumulator the 40.0 threshold crosses —
    // the destroy the drop speaks (the plain cart's own getDropItem
    // the minecart item answers)
    let contact = app.stab_walk("iron_spear");
    assert!(contact, "the destroy the contact still answers");
    assert!(
        app.offline_carts.is_empty(),
        "the meter the 40.0 the destroy serves"
    );
    let minecart_item = mc_registry::item_id_by_name("minecart").unwrap();
    assert!(
        app.drops.iter().any(|d| d.item == minecart_item),
        "the cart's own drop: {:?}",
        app.drops
    );
    // THE USE-TICK'S OWN HULL DOSE (javap `KineticWeapon
    // .damageEntities` the same collection the memory keyed by the
    // vehicle's own id serves): the sprint thrust the look·motion
    // dot carries, the folded amount the raw base +
    // `floor(relative·multiplier)` speaks, the hurt the damage
    // condition alone gates — the 7.0 the ·10 ramp the destroy
    // serves. THE BODY the camera's own authority wears (the
    // sprint pattern — the cart the BODY's own forward serves)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        (app.spawn_y + 7) as f64 - 1.62,
        c as f64 + 0.5,
        180.0,
        0.0,
    ));
    let look = app.world_camera().forward();
    let ucx = look.x as f64 * 3.0 + c as f64 + 0.5;
    let ucz = look.z as f64 * 3.0 + c as f64 + 0.5;
    app.offline_carts.push(crate::cart::OfflineCart {
        id: 41,
        kind: "minecart",
        cart: crate::cart::Minecart::new([ucx, ey - 0.4, ucz]),
        hurt_time: 0,
        hurt_dir: 1,
        damage: 0.0,
        fuse: crate::cart::TNT_FUSE_UNPRIMED,
    });
    // THE SPRINT'S OWN LUNGE (the velocity the look carries —
    // 0.28 b/tick × 20 = 5.6 b/s the sprint walk rides — the
    // iron row's own 4.6 min the relative answers)
    if let Some(b) = app.body.as_mut() {
        b.vx = 0.28 * look.x as f64;
        b.vz = 0.28 * look.z as f64;
    }
    app.frame = 30; // the throttle's bank the first window clears
    app.spear_use_tick_stab(12); // the iron tier's own 12 the walk passes
    assert!(
        app.offline_carts.is_empty(),
        "the folded 7.0 the ·10 ramp the destroy serves"
    );
    assert!(
        app.drops.iter().any(|d| d.item == minecart_item),
        "the use-tick's own drop"
    );
    assert!(
        app.stab_contact_memory.iter().any(|(id, _)| *id == 41),
        "the bank the vehicle's own id keeps"
    );
    // THE COOLDOWN'S OWN SILENCE (the respawned id the 10-tick
    // window refuses — the re-evaluation the recent-stab memory
    // never serves, no second dose)
    app.offline_carts.push(crate::cart::OfflineCart {
        id: 41,
        kind: "minecart",
        cart: crate::cart::Minecart::new([ucx, ey - 0.4, ucz]),
        hurt_time: 0,
        hurt_dir: 1,
        damage: 0.0,
        fuse: crate::cart::TNT_FUSE_UNPRIMED,
    });
    app.spear_use_tick_stab(13);
    assert_eq!(
        app.offline_carts.len(),
        1,
        "the cooldown the second dose refuses"
    );
    assert_eq!(
        app.offline_carts[0].damage, 0.0,
        "the hurt the refused walk never arms"
    );
    assert_eq!(
        app.offline_carts[0].hurt_time, 0,
        "the wobble the refused walk never arms"
    );
}

/// THE CREATIVE'S OWN REFUSAL (javap `Warden.canTargetEntity` — the
/// creative player the FALSE the anger refuses: the hurt the
/// creative STILL wears, the store the stab never climbs, the
/// record the anger_raised flag never banks).
#[test]
fn the_stab_the_warden_the_creative_the_anger_refuses() {
    let mut app = App::new(3);
    app.enter_world();
    app.game_mode = GameMode::Creative;
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
    app.wardens.push(crate::sculk::WardenMob::new(
        102,
        cam.pos.x as f64 + f.x as f64 * 3.0,
        cam.pos.y as f64 - 1.0,
        cam.pos.z as f64 + f.z as f64 * 3.0,
    ));
    let expected_dmg = (PLAYER_ATTACK_DAMAGE + material_bonus("iron_spear")) as f32;
    let contact = app.stab_walk("iron_spear");
    assert!(contact, "the creative the stab still reaches");
    assert_eq!(
        app.wardens[0].health,
        500.0 - expected_dmg,
        "the hurt the creative STILL wears"
    );
    assert_eq!(
        app.wardens[0].active_anger(),
        0,
        "the canTargetEntity refusal the anger keeps"
    );
    assert!(
        !app.game_events
            .iter()
            .any(|e| e.starts_with("warden_anger @")),
        "the refused anger the record never banks"
    );
}

/// THE USE-TICK STAB'S OWN WARDEN (javap `KineticWeapon.damageEntities`
/// — the sprint thrust the warden's own stationary body the zero
/// victim-dot keeps, the FULL 7.0 the armor-absence wears (the
/// zombie's own 6.888 the CombatRules walk absorbs), the memory the
/// warden's own id banks, the cooldown the second dose refuses, the
/// +100 the anger the record banks).
#[test]
fn the_use_tick_stab_the_warden_the_raw_damage_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let iron = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (iron, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        (app.spawn_y + 7) as f64 - 1.62,
        c as f64 + 0.5,
        180.0,
        0.0,
    ));
    let cam = app.world_camera();
    let f = cam.forward();
    app.wardens.push(crate::sculk::WardenMob::new(
        103,
        cam.pos.x as f64 + f.x as f64 * 3.0,
        cam.pos.y as f64 - 1.0,
        cam.pos.z as f64 + f.z as f64 * 3.0,
    ));
    // THE SPRINT'S OWN LUNGE (the velocity the look carries — the
    // same 0.28 b/tick the sprint walk rides, the warden the
    // STATIONARY body the victim's own zero dot keeps)
    if let Some(b) = app.body.as_mut() {
        b.vx = 0.28 * f.x as f64;
        b.vz = 0.28 * f.z as f64;
    }
    app.frame = 30; // the throttle's bank the first window clears
    app.spear_use_tick_stab(12); // the iron tier's own 12 the walk passes
                                 // THE RAW FOLD (2.0 + floor(5.6·0.95) = 7.0 — the FULL the
                                 // armor-absence wears, the zombie's own absorbed 6.888 the
                                 // CombatRules walk speaks)
    assert_eq!(
        app.wardens[0].health, 493.0,
        "the FULL 7.0 the no-armor attributes keep"
    );
    assert!(
        app.stab_contact_memory.iter().any(|(id, _)| *id == 103),
        "the memory the warden's own id banks"
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("warden_anger @") && e.contains("+100 -> 100")),
        "the use-tick stab's own +100 the record banks: {:?}",
        app.game_events
    );
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("entity.warden.hurt ")),
            "the warden's own hurt voice: {:?}",
            app.sounds.events
        );
    }
    // THE COOLDOWN'S OWN SILENCE (the next tick the recent-stab
    // memory the re-evaluation never serves — no second dose, no
    // second +100)
    app.spear_use_tick_stab(13);
    assert_eq!(
        app.wardens[0].health, 493.0,
        "the cooldown the second dose refuses"
    );
    assert_eq!(
        app.game_events
            .iter()
            .filter(|e| e.starts_with("warden_anger @"))
            .count(),
        1,
        "the cooldown the second record refuses"
    );
}

/// THE USE CLOCK'S OWN ADVANCE (the hold the update loop ticks):
/// the press opens the hold, the updates the ticks advance — the
/// first 10 the stab walk's own delay sleeps (no damage the
/// standing body wears), the tick counter the vanilla use lane's
/// own clock keeps.
#[test]
fn the_spear_use_the_hold_the_clock_serves() {
    let mut app = App::new(3);
    app.enter_world();
    let iron = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (iron, 1);
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
    app.mobs.push(crate::mob::Zombie::new(
        200,
        ex + f.x as f64 * 3.0,
        ey - 1.0,
        ez + f.z as f64 * 3.0,
        4,
    ));
    let pre = app.mobs[0].health;
    assert!(app.on_right_press(0, 0), "the hold opens");
    for _ in 0..3 {
        app.update();
    }
    let active = app.using_item.as_ref().expect("the hold lives");
    assert_eq!(active.ticks, 3, "the clock the updates advance");
    assert_eq!(app.mobs[0].health, pre, "the delay the body guards");
}

/// THE WIRE USE'S OWN SERVER (the live lane — the use-tick walk
/// the server's own process keeps): the networked world's press
/// the UseItem echo sends, the use the hold opens, the local walk
/// the `world_is_network` gate never runs (the byte-2 echo the
/// entity-event mailbox answers — the autopush-473 lane).
#[test]
fn the_wire_use_the_local_walk_never_runs() {
    use crate::world_view::ContainerCommand;
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let iron = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.live_held = Some(iron);
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(tx);
    assert!(app.on_right_press(0, 0), "the live press the hold arms");
    let active = app.using_item.as_ref().expect("the hold opened");
    assert_eq!(active.item, iron, "the held spear the use rides");
    match rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::UseItem { hand, .. } => {
            assert_eq!(hand, 0, "the MAIN hand ordinal the echo wears");
        }
        other => panic!("expected a UseItem echo, got {other:?}"),
    }
    if !sounds_loaded() {
        return;
    }
    // THE USE VOICE (the press's own prediction the client speaks)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.use ")),
        "the use voice the press speaks: {:?}",
        app.sounds.events
    );
    // THE LOCAL WALK'S SILENCE (the contact verdict the server
    // owns — the hit voice the mailbox answers, not the hold)
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.hit ")),
        "the wire hold the local hit voice never wears: {:?}",
        app.sounds.events
    );
}

/// THE SEVEN TIERS' OWN DELAY LADDER (the Items clinit's spear
/// rows the Properties.spear modifier turns into attack speeds —
/// the `(1.0/speed)*20.0` law the delays 13/15/17/19/19/21/23
/// ticks keeps: the exact floats the CP walk resolved —
/// 0.65/0.75/0.85/0.95/0.95/1.05/1.15 the seconds the rows wear).
#[test]
fn the_spear_the_delay_the_tiers_wear() {
    let cases: &[(&str, f64)] = &[
        ("wooden_spear", 13.0),
        ("stone_spear", 15.0),
        ("copper_spear", 17.0),
        ("iron_spear", 19.0),
        ("golden_spear", 19.0),
        ("diamond_spear", 21.0),
        ("netherite_spear", 23.0),
    ];
    for (name, delay) in cases {
        let got = super::held_charge_delay(Some(name));
        assert!(
            (got - delay).abs() < 1e-9,
            "{name}: the delay {got} the row {delay} keeps"
        );
    }
    // the raw speed the modifier speaks (the iron row the
    // `1.0/0.95` attribute keeps)
    let speed = super::held_attack_speed(Some("iron_spear"));
    assert!((speed - 1.0 / 0.95).abs() < 1e-9);
}

/// THE MISS LANE'S OWN RESET (the 26.2 MISS case's tail — the
/// `resetAttackStrengthTicker()` pcs 366-370 the dump keeps: the
/// air-click the meter zeroes; the BLOCK lane the same law NEVER
/// speaks — the dig the ticker's count leaves standing).
#[test]
fn the_miss_press_the_meter_zeroes() {
    let mut app = App::new(3);
    app.enter_world();
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    app.inventory.items[app.selected_hotbar] = (sword, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // THE BLOCK PRESS FIRST (the dig arms — the ticker the block
    // lane never touches)
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 4) as f32, c as f32 + 0.5),
        yaw: 180.0,
        pitch: 80.0,
    });
    app.attack_ticker = 30;
    app.on_attack();
    assert!(app.dig.is_destroying, "the block press the dig arms");
    assert_eq!(app.attack_ticker, 30, "the block lane the ticker keeps");
    // THE SKY PRESS (the miss arm — the missTime AND the reset the
    // same case tail speaks)
    app.dig = crate::dig::DigState::default();
    app.miss_time = 0;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    app.on_attack();
    assert_eq!(app.miss_time, 10, "the miss the hasMissTime branch arms");
    assert_eq!(app.attack_ticker, 0, "the miss lane the reset speaks");
}

/// THE ATTACK RANGE COMPONENT'S OWN PICK EXTENSION (javap
/// `LocalPlayer.raycastHitResult` + `AttackRange.effectiveMaxRange`
/// — the spear's AttackRange(2.0, 4.5, 2.0, 6.5, 0.125, 0.5) the
/// only carrier the CP scan speaks): the body ON the aim line at
/// ~4.2 the bare hand's 3.0 filter refuses, the spear's own
/// 4.625 reach the pick serves — the crosshair's mob verdict the
/// held item's own window wears.
#[test]
fn the_spear_the_pick_reach_the_component_extends() {
    let mut app = App::new(3);
    app.enter_world();
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
    // the body on the aim line (the box front face ~3.9 from the
    // eye — outside the 3.0 default, inside the spear's 4.625)
    let t = 4.2;
    app.mobs.push(crate::mob::Zombie::new(
        200,
        ex + f.x as f64 * t,
        ey - 1.0,
        ez + f.z as f64 * t,
        4,
    ));
    // THE BARE HAND (the default window): the 3.0 reach refuses
    assert_eq!(
        app.pick_mob(&[]),
        None,
        "the bare hand the 3.0 default keeps"
    );
    // THE SPEAR (the AttackRange carrier): the reach the pick
    // extends
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    assert_eq!(
        app.pick_mob(&[]),
        Some(0),
        "the spear's 4.625 the pick extends"
    );
}

/// THE POLEARM'S OWN DEAD ZONE (javap `AttackRange.isInRange`'s
/// min fold + `ProjectileUtil.getHitEntitiesAlong`'s own ray START
/// at `eye + look.scale(effectiveMinRange)` — the 2.0 - 0.125 =
/// 1.875 window the too-close body the pick refuses: the spear a
/// POLEARM wears, the chest-bumping body the crosshair never
/// names).
#[test]
fn the_spear_the_pick_dead_zone_the_polearm_keeps() {
    let mut app = App::new(3);
    app.enter_world();
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
    // inside the 3.0 default, INSIDE the spear's 1.875 dead zone)
    let t = 1.2;
    app.mobs.push(crate::mob::Zombie::new(
        200,
        ex + f.x as f64 * t,
        ey - 1.0,
        ez + f.z as f64 * t,
        4,
    ));
    // THE BARE HAND: the close body the default serves
    assert_eq!(
        app.pick_mob(&[]),
        Some(0),
        "the bare hand the close body serves"
    );
    // THE SPEAR: the dead zone the pick refuses
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    assert_eq!(
        app.pick_mob(&[]),
        None,
        "the polearm's dead zone the pick refuses"
    );
}

/// THE CREATIVE REACH'S OWN SWITCH (javap
/// `AttackRange.effectiveMaxRange`'s own isCreative fold — the
/// maxCreativeReach 6.5 + 0.125 the instabuild attacker wears):
/// the body at ~6.0 the survival 4.625 refuses, the creative
/// 6.625 the pick serves.
#[test]
fn the_spear_the_pick_the_creative_reach_extends() {
    let mut app = App::new(3);
    app.enter_world();
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
    // the body far on the aim line (the box front face ~5.7 —
    // beyond the survival 4.625, inside the creative 6.625)
    let t = 6.0;
    app.mobs.push(crate::mob::Zombie::new(
        200,
        ex + f.x as f64 * t,
        ey - 1.0,
        ez + f.z as f64 * t,
        4,
    ));
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // THE SURVIVAL HOLD: the 4.625 the far body refuses
    assert_eq!(
        app.pick_mob(&[]),
        None,
        "the survival 4.625 the far body refuses"
    );
    // THE CREATIVE HOLD: the 6.625 the pick extends
    app.game_mode = GameMode::Creative;
    assert_eq!(
        app.pick_mob(&[]),
        Some(0),
        "the creative 6.625 the pick extends"
    );
    // THE WINDOW HELPER'S OWN TABLE (the exact bounds the fold
    // keeps — survival (1.875, 4.625), creative (1.875, 6.625),
    // the non-carrier (0.0, 3.0))
    assert_eq!(
        super::held_attack_pick_window(Some("iron_spear"), false),
        (1.875 * 1.875, 4.625 * 4.625)
    );
    assert_eq!(
        super::held_attack_pick_window(Some("iron_spear"), true),
        (1.875 * 1.875, 6.625 * 6.625)
    );
    assert_eq!(
        super::held_attack_pick_window(None, false),
        (0.0, 3.0 * 3.0)
    );
}

/// THE MOVEMENT'S OWN LEAD (javap `ProjectileUtil
/// .getHitEntitiesAlong` pcs 30-64 — the ray END the velocity
/// extends: `effectiveMaxRange + max(0, movement · look)`): the
/// body at ~4.7 the static spear's 4.625 refuses, the LUNGING
/// body (the 0.3/tick walk ahead the velocity speaks) the lead
/// 0.3 the pick serves, the RETREATING body the max(0, dot) gate
/// the lead never wears (the far body the retreat never serves).
#[test]
fn the_spear_the_pick_the_motion_leads() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    let cam0 = app.world_camera();
    let (ex, ey, ez) = (cam0.pos.x as f64, cam0.pos.y as f64, cam0.pos.z as f64);
    // the body the eye's own pose keeps (the feet the STANDING_EYE
    // 1.62 below the offline camera's eye — the body camera the
    // world_camera's highest authority wears, the same eye the
    // offline discipline kept bit for bit; the YAW the body's own
    // +PI radian fold restores (body degrees -> camera radians
    // +PI), the pitch the mirrored sign the setup speaks)
    let body_pose = (ex, ey - crate::body::STANDING_EYE, ez);
    let body = crate::body::PlayerBody::new(
        body_pose.0,
        body_pose.1,
        body_pose.2,
        (cam0.yaw - std::f32::consts::PI).to_degrees(),
        (-cam0.pitch).to_degrees(),
    );
    app.body = Some(body);
    // the camera the body rides now — re-read (the same pose the
    // body's own camera restores bit for bit)
    let cam = app.world_camera();
    let f = cam.forward();
    let view = (f.x as f64, f.y as f64, f.z as f64);
    let (ex, ey, ez) = (cam.pos.x as f64, cam.pos.y as f64, cam.pos.z as f64);
    // the body far on the aim line (the box front face ~4.7 —
    // beyond the static 4.625, inside the lunging 4.925)
    let t = 5.0;
    app.mobs.push(crate::mob::Zombie::new(
        200,
        ex + view.0 * t,
        ey - 1.0,
        ez + view.2 * t,
        4,
    ));
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    app.inventory.items[app.selected_hotbar] = (spear, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // THE STANDING HOLD: the static 4.625 the far body refuses
    assert_eq!(
        app.pick_mob(&[]),
        None,
        "the standing spear the static reach keeps"
    );
    // THE LUNGING HOLD (the 0.3/tick velocity along the look —
    // the lead the ray end extends: 4.625 + 0.3 = 4.925; the body
    // pose the same, the velocity the only variant)
    let body = app.body.as_mut().unwrap();
    body.vx = view.0 * 0.3;
    body.vy = view.1 * 0.3;
    body.vz = view.2 * 0.3;
    assert_eq!(
        app.pick_mob(&[]),
        Some(0),
        "the lunging spear the lead extends"
    );
    // THE RETREATING HOLD (the velocity AGAINST the look — the
    // max(0, dot) gate the lead zeroes: the static reach the far
    // body the same refusal keeps)
    let back = app.body.as_mut().unwrap();
    back.vx = -view.0 * 0.3;
    back.vy = -view.1 * 0.3;
    back.vz = -view.2 * 0.3;
    assert_eq!(app.pick_mob(&[]), None, "the retreat the lead never wears");
}

/// THE OFFHAND DRINK'S OWN REMAINDER (the milk bucket's arc — the
/// single stack BECOMES the remainder: the milk consumed, the
/// bucket the offhand's own slot keeps — the USE_REMAINDER's
/// return the USED hand's lane answers).
#[test]
fn the_offhand_drink_the_remainder_returns_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    let milk = mc_registry::item_id_by_name("milk_bucket").unwrap();
    let bucket = mc_registry::item_id_by_name("bucket").unwrap();
    let bread = mc_registry::item_id_by_name("bread").unwrap();
    app.inventory.offhand[0] = (milk, 1);
    app.inventory.items[app.selected_hotbar] = (bread, 2);
    // the camera the sky reads (the block lane the miss keeps)
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    // the FOOD-less consumable (the nutrition -1 the canConsume's
    // unconditional arm wears — the milk's own def the registry's
    // rows speak): the drink the hold opens regardless of the bar
    assert!(app.on_right_press(0, 0), "the milk the hold opens");
    let eat = app.eating.as_ref().expect("the hold opened");
    assert_eq!(eat.hand, UseHand::Off, "the OFF hand the drink wears");
    assert!(!eat.has_food(), "the milk the FOOD-less lane rides");
    let total = eat.total;
    for _ in 0..total {
        app.update();
    }
    assert!(app.eating.is_none(), "the drink completed");
    // THE REMAINDER (the single stack BECOMES the remainder — the
    // bucket the offhand's own slot keeps)
    assert_eq!(
        app.inventory.offhand[0],
        (bucket, 1),
        "the bucket the remainder keeps"
    );
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (bread, 2),
        "the main lane's stack untouched"
    );
}

/// THE LOCAL SUGGESTIONS' OWN WALK (the Brigadier parse the tree's
/// literals answer — the sorted prefix matches, the redirect the
/// alias hop follows, the argument slot the ask-server lane keeps,
/// the lowercase fold the case-insensitive filter wears).
#[test]
fn the_local_walk_the_tree_literals_speak() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_tree();
    // THE ROOT PREFIX (the '/' the parse strips — the partial 't'
    // the lowercase fold filters, the sort the merge law keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/t", &[], true, &[])
    else {
        panic!("/t the local lane answers");
    };
    assert_eq!((start, length), (1, 1));
    let names: Vec<&str> = matches.iter().map(|m| m.text.as_str()).collect();
    assert_eq!(names, vec!["tick", "time", "tp"], "the sorted literals");
    // THE BARE SLASH (the empty partial the whole root answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/", &[], true, &[])
    else {
        panic!("/ the local lane answers");
    };
    assert_eq!((start, length), (1, 0));
    let names: Vec<&str> = matches.iter().map(|m| m.text.as_str()).collect();
    assert_eq!(
            names,
            vec!["tick", "time", "tp"],
            "the ROOT'S OWN children only — the teleport alias the redirect's target rides, no child of the root"
        );
    // THE CASE FOLD (the uppercase partial the lowercase law answers)
    let Src::Local { matches, .. } = local_command_suggestions(&tree, "/TIME", &[], true, &[])
    else {
        panic!("/TIME the case-insensitive lane answers");
    };
    let names: Vec<&str> = matches.iter().map(|m| m.text.as_str()).collect();
    assert_eq!(names, vec!["time"]);
    // THE SUB-COMMAND (the completed 'time' the equality consumes,
    // the trailing space the empty partial speaks — the sorted
    // children the range [6,0) wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/time ", &[], true, &[])
    else {
        panic!("/time␠ the sub-literals answer");
    };
    assert_eq!((start, length), (6, 0));
    let names: Vec<&str> = matches.iter().map(|m| m.text.as_str()).collect();
    assert_eq!(names, vec!["query", "set"]);
    // THE NARROWED TAIL (the partial 's' the set alone answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/time s", &[], true, &[])
    else {
        panic!("/time␠s the narrowed lane answers");
    };
    assert_eq!((start, length), (6, 1));
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].text, "set");
    // THE REDIRECT'S OWN SLOT (the consumed 'tp' the alias hop
    // follows — the teleport node's argument children the ask
    // lane answers)
    assert!(matches!(
        local_command_suggestions(&tree, "/tp ", &[], true, &[]),
        Src::AskServer
    ));
    // THE ARGUMENT CHILD WHOLE (the teleport node's own name typed
    // whole: the trailing space the argument slot the ask lane
    // answers)
    assert!(matches!(
        local_command_suggestions(&tree, "/teleport ", &[], true, &[]),
        Src::AskServer
    ));
    // THE UNKNOWN LITERAL (the root's literal-only children the
    // empty list keeps — no round-trip the honest silence spends)
    let Src::Local { matches, .. } = local_command_suggestions(&tree, "/zz", &[], true, &[]) else {
        panic!("/zz the empty local list keeps");
    };
    assert!(matches.is_empty());
    // THE LEAF WITH NO CHILDREN (the tick node's own silence)
    let Src::Local { matches, .. } = local_command_suggestions(&tree, "/tick ", &[], true, &[])
    else {
        panic!("/tick␠ the empty local list keeps");
    };
    assert!(matches.is_empty());
    // THE NO-SLASH TEXT (the plain message the ask lane keeps)
    assert!(matches!(
        local_command_suggestions(&tree, "hello", &[], true, &[]),
        Src::AskServer
    ));
    // THE DOUBLE SPACE (the empty token the literal equality
    // refuses — the ask lane answers)
    assert!(matches!(
        local_command_suggestions(&tree, "/time  s", &[], true, &[]),
        Src::AskServer
    ));
}

/// THE LOCAL LIST'S OWN LANES (the refresh the tree answers: the
/// '/' edit the LOCAL list lands — no request the lane spends, no
/// pending id the gate waits on, the fresh defaults the fields
/// wear; the argument slot the ask-server request rides; the Tab
/// apply the narrowed local walk re-answers; the stale reply the
/// None-pending gate drops).
#[test]
fn the_local_list_the_refresh_and_lanes_speak() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_tree());
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    // THE SLASH (the local list lands — no request, no pending)
    app.on_chat_char('/');
    assert!(
        rx.try_recv().is_err(),
        "the local lane the request never spends"
    );
    assert_eq!(app.chat_suggestion_pending, None, "the gate no id waits on");
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
    // THE TYPE-AHEAD (the partial 't' the narrowed sort answers)
    app.on_chat_char('t');
    assert_eq!(
        app.chat_suggestions
            .iter()
            .map(|m| m.text.as_str())
            .collect::<Vec<_>>(),
        vec!["tick", "time", "tp"]
    );
    assert_eq!(app.chat_suggestion_range, (1, 1));
    // THE TAB APPLY (the selected 'tick' the replace range lands —
    // the input the completion wears, the narrowed list the local
    // walk re-answers)
    app.on_chat_tab_key();
    assert_eq!(app.chat_input, "/tick", "the apply the range keeps");
    assert!(
        app.chat_suggestion_tab_cycles,
        "the cycle flag the apply raises"
    );
    assert_eq!(
        app.chat_suggestions
            .iter()
            .map(|m| m.text.as_str())
            .collect::<Vec<_>>(),
        vec!["tick"],
        "the narrowed list the fresh walk answers"
    );
    // THE ARGUMENT SLOT (the '/tp ' the alias hop lands in — the
    // request the ask lane rides, the pending id the gate banks)
    for _ in 0..4 {
        app.on_chat_backspace(); // "/tick" → "/" (the '/' the pops keep)
    }
    app.on_chat_char('t');
    app.on_chat_char('p');
    app.on_chat_char(' ');
    assert_eq!(app.chat_input, "/tp ");
    match rx.try_recv().expect("the argument slot rides the request") {
        crate::world_view::ContainerCommand::RequestCommandSuggestions { id, command } => {
            assert_eq!(command, "/tp ", "the FULL input the request carries");
            assert_eq!(
                app.chat_suggestion_pending,
                Some(id),
                "the gate the id waits on"
            );
        }
        other => panic!("the wrong command rode the lane: {other:?}"),
    }
    assert!(
        !app.chat_suggestions.is_empty(),
        "the shown list the fresh reply's landing replaces — the async gap the old law keeps"
    );
    // THE LOCAL LIST'S OWN FRESHNESS (the literal tail the local
    // walk answers again — a reply the None-pending gate drops)
    app.on_chat_backspace(); // "/tp" — the redirect alias's own name
    let src_reply = mc_protocol::play::CommandSuggestionsReply {
        id: 9999,
        start: 0,
        length: 0,
        matches: vec![mc_protocol::play::CommandSuggestionsMatch {
            text: "server_row".to_string(),
            tooltip: None,
        }],
    };
    app.on_command_suggestions(src_reply);
    assert!(
        !app.chat_suggestions.iter().any(|m| m.text == "server_row"),
        "the stale reply the gate drops"
    );
}

/// THE ENTITY SLOTS' OWN TAB BELIEF (javap EntityArgument /
/// GameProfileArgument listSuggestions → EntitySelectorParser
/// .fillSuggestions — the names consumer the online roster
/// answers: the SharedSuggestionProvider.suggest fold the
/// matchesSubStr splitter walk filters, the Suggestions.merge sort
/// the case-insensitive law keeps, the EMPTY partial the
/// suggestNameOrSelector state the names AND the six selectors
/// both answer).
#[test]
fn the_entity_slots_the_tab_belief_answers() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    let names: Vec<String> = ["Alice", "bob", "Zed_1"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    // THE EMPTY PARTIAL (the suggestNameOrSelector state — the
    // selectors first ('@' < 'a' the compareToIgnoreCase fold
    // keeps), the names after, the range the whole token span
    // names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kick ", &names, true, &[])
    else {
        panic!("/kick the tab belief answers");
    };
    assert_eq!((start, length), (6, 0), "the empty partial's own span");
    let texts: Vec<&str> = matches.iter().map(|m| m.text.as_str()).collect();
    assert_eq!(
        texts,
        vec!["@a", "@e", "@n", "@p", "@r", "@s", "Alice", "bob", "Zed_1"],
        "the merged sort — the selectors before the names, the case fold the order keeps"
    );
    assert!(
        matches.iter().take(6).all(|m| m.tooltip.is_some()),
        "the selectors the vanilla tooltips ride"
    );
    assert!(
        matches.iter().skip(6).all(|m| m.tooltip.is_none()),
        "the names no tooltip carry"
    );
    // THE NAME FOLD (the prefix the lowercase law filters — the
    // matchesSubStr walk the splitter never inside the match)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kick ali", &names, true, &[])
    else {
        panic!("/kick ali the name fold answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["Alice"],
        "the case fold the prefix matches"
    );
    // THE SPLITTER WALK (javap matchesSubStr — the startsWith at i
    // the loop keeps, the anyOf("._/") the hop past the next
    // splitter answers: 'x' the tail after Steve_x's '_' the walk
    // matches)
    let steve = vec!["Steve_x".to_string()];
    let Src::Local { matches, .. } = local_command_suggestions(&tree, "/kick x", &steve, true, &[])
    else {
        panic!("/kick x the splitter walk answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["Steve_x"],
        "the '_' boundary the splitter walk crosses"
    );
    // THE NO-MATCH LAW (a prefix no splitter walk rescues — the
    // empty LOCAL the honest silence keeps, no round-trip)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kick zz", &names, true, &[])
    else {
        panic!("/kick zz the empty local answers");
    };
    assert!(matches.is_empty(), "the silence the local list keeps");
    // THE ROSTER'S OWN EMPTINESS (no names the six selectors the
    // empty partial still answers — the suggestNameOrSelector walk
    // the consumer's empty fold rides)
    let Src::Local { matches, .. } = local_command_suggestions(&tree, "/kick ", &[], true, &[])
    else {
        panic!("/kick the empty roster answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["@a", "@e", "@n", "@p", "@r", "@s"],
        "the selectors the empty roster still speaks"
    );
    // THE PROFILE SLOT (the GameProfileArgument lambda's own walk —
    // the getOnlinePlayerNames consumer the same lane rides)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kick bob", &names, true, &[])
    else {
        panic!("/kick bob answers");
    };
    assert_eq!(matches[0].text, "bob");
}

/// THE SELECTOR LANE'S OWN WALK (javap EntitySelectorParser — the
/// '@' partial the suggestSelector state answers: the six
/// `fillSelectorSuggestions` calls the offset's own span replaces,
/// the '@'-state the createOffset(getStart() - 1) walk keeps, the
/// parsed state the span AFTER the '@' names, the unknown-kind
/// read the setCursor(i) rewind un-does, the options walk ('[')
/// the honest ask keeps, the list the six UNFILTERED — the re-pick
/// UX '@a' typed still lists all six).
#[test]
fn the_selector_lane_the_six_speak() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE BARE '@' (the suggestSelector state — the whole token
    // span the suggestions' own text replaces)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @", &[], true, &[])
    else {
        panic!("/kill @ the selector lane answers");
    };
    assert_eq!(
        (start, length),
        (6, 1),
        "the '@' span the replace range keeps"
    );
    let texts: Vec<&str> = matches.iter().map(|m| m.text.as_str()).collect();
    assert_eq!(
        texts,
        vec!["@a", "@e", "@n", "@p", "@r", "@s"],
        "the six the merged sort orders"
    );
    assert_eq!(
        matches[0].tooltip.as_deref(),
        Some("All players"),
        "the lang table's own lookup the en_us text answers"
    );
    assert_eq!(matches[2].tooltip.as_deref(), Some("Nearest entity"));
    assert_eq!(matches[5].tooltip.as_deref(), Some("Current entity"));
    // THE PARSED STATE (a kind char consumed — the span AFTER the
    // '@', the six the re-pick UX still lists, no filter the walk
    // wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a", &[], true, &[])
    else {
        panic!("/kill @a the parsed state answers");
    };
    assert_eq!(
        (start, length),
        (7, 1),
        "the after-'@' span the range keeps"
    );
    assert_eq!(matches.len(), 6, "the six the no-filter law keeps");
    // THE UNKNOWN KIND (the 'x' read the rewind un-does — the '@'
    // state the whole token span answers again)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @x", &[], true, &[])
    else {
        panic!("/kill @x the rewind state answers");
    };
    assert_eq!(
        (start, length),
        (6, 2),
        "the whole token the range replaces"
    );
    assert_eq!(matches.len(), 6, "the six the rewind state still speaks");
    // THE OPTIONS WALK (the '[' the name lane opens — the option
    // names the local answer speaks, the value states the ask
    // keeps — the dedicated test walks them)
    assert!(
        matches!(
            local_command_suggestions(&tree, "/kill @a[", &[], true, &[]),
            Src::Local { .. }
        ),
        "the '[' entry the option-name lane answers"
    );
    // THE NO-SELECTOR BELIEF (the atom-permission lane the house
    // never reads — false the '@' the EMPTY local answers, no
    // round-trip the vanilla's own catch-swallowed parse keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @", &[], false, &[])
    else {
        panic!("/kill @ the gated walk answers");
    };
    assert_eq!((start, length), (6, 1));
    assert!(matches.is_empty(), "the selectors the gate never names");
}

/// THE OPTIONS' OWN NAME LANE (javap `EntitySelectorParser
/// .parseOptions` + `EntitySelectorOptions.suggestNames` — the '['
/// entry and every ',' continuation the suggestNames state
/// installs, the 21 `bootStrap` keys the `key + '='` texts wear,
/// the tooltip the `Option.description` translatable carries, the
/// lowercase-prefix fold the remaining_lc law keeps, the merge
/// sort the case-insensitive law orders; the '{'/quoted/'=' value
/// states the honest ask keeps — the house speaks no value
/// providers; the '!' prefix the no-match silence answers — the
/// 26.2 keys the '!' never starts-with).
#[test]
fn the_selector_options_the_name_lane_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE '[' ENTRY (the suggestNames state — the empty tail the
    // 21 keys answer, the '=' suffix each text wears, the range
    // the tail's own span names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[", &[], true, &[])
    else {
        panic!("/kill @a[ the option-name lane answers");
    };
    assert_eq!((start, length), (9, 0), "the empty tail's own span");
    assert_eq!(matches.len(), 21, "the bootStrap keys the lane answers");
    assert_eq!(
        matches[0].text, "advancements=",
        "the merge sort the case-insensitive law orders"
    );
    assert_eq!(
        matches[0].tooltip.as_deref(),
        Some("Players with advancements"),
        "the lang table's own lookup the en_us text answers"
    );
    // THE PREFIX FOLD (the remaining_lc law — 'so' the sort answers,
    // the '=' suffix the text carries)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[so", &[], true, &[])
    else {
        panic!("/kill @a[so the prefix fold answers");
    };
    assert_eq!((start, length), (9, 2), "the typed prefix's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["sort="],
        "the 'so' prefix the sort option answers"
    );
    assert_eq!(matches[0].tooltip.as_deref(), Some("Sort the entities"));
    // THE COMMA CONTINUATION (the fresh-name position the ',' state
    // installs — the empty tail the keys the canUse gates answer:
    // the sort element the state MARKED (the audited $25 — the
    // markParsed the hit path wears), the sort row the canParse
    // gate hides (the audited $27) — the 20 rows the merge sort
    // orders)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=nearest,", &[], true, &[])
    else {
        panic!("/kill @a[sort=nearest, the comma state answers");
    };
    assert_eq!((start, length), (22, 0), "the fresh tail's own span");
    assert_eq!(matches.len(), 20, "the parsed sort the own row hides");
    // THE VALUE STATES (the '=' tail the UNKNOWN-KEY lane the
    // audited order law wears — the get('so') null the
    // rollbackAndThrow answers, the '='-carrying remaining no key
    // prefixes, the EMPTY local the span the key start names —
    // see the_unknown_key_lane test; the '{' map value the honest
    // ask keeps — the house speaks no map-value providers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[so=", &[], true, &[])
    else {
        panic!("/kill @a[so= the unknown-key walk answers");
    };
    assert_eq!((start, length), (9, 3), "the span the key start names");
    assert!(matches.is_empty(), "the prefix '=' the get() null wears");
    // THE SNBT FORM THE '=' EXPECTATION REFUSES (the audited $46
    // — the compound the walk speaks: the objective read, the
    // ws, the '=' expectation at 48-51 — the ':' the throw
    // answers, the SUGGEST_NOTHING silence the EMPTY local
    // keeps — the '='-led compound the walk accepts, the ':'
    // form the parse never wore)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={foo:1}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{foo:1}} the compound walk answers");
    };
    assert_eq!((start, length), (16, 7), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the ':' form the '=' expectation silences"
    );
    // THE QUOTED VALUE (the name key's readString walk — the
    // UNTERMINATED quote the END-OF-QUOTE throw answers, the
    // EMPTY local the parse-throw-catch's silence keeps — see
    // the_quoted_value_lanes test)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[name=\"St", &[], true, &[])
    else {
        panic!("/kill @a[name=\"St the quoted lane answers");
    };
    assert!(
        matches.is_empty(),
        "the unterminated quote the silence keeps"
    );
    // THE '!' PREFIX (the 26.2 suggestNames the keys the '!'
    // never starts-with — the empty local the silence keeps)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[!x", &[], true, &[])
    else {
        panic!("/kill @a[!x the no-match lane answers");
    };
    assert!(matches.is_empty(), "the '!' prefix the silence keeps");
    // THE TAB APPLY (the selected 'sort=' the replace lands — the
    // input the '=' suffix wears)
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/kill @a[so".chars() {
        app.on_chat_char(c);
    }
    assert!(
        _rx.try_recv().is_err(),
        "the option-name lane no request spends"
    );
    app.on_chat_tab_key();
    assert_eq!(
        app.chat_input, "/kill @a[sort=",
        "the apply the '=' suffix wears"
    );
}

/// THE OPTIONS' OWN VALUE LANE (javap `EntitySelectorOptions
/// lambda$bootStrap$25/26/28/29` + `EntitySelectorParser
/// .parseOptions` — the sort walk the SharedSuggestionProvider
/// .suggest fold answers, the gamemode walk the `$29` arm law
/// keeps, the valid complete element the post-value ',' + ']'
/// punctuation answers, the unspoken keys the honest ask keeps,
/// no tooltip the plain suggest texts wear).
#[test]
fn the_selector_options_the_value_lane_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE SORT WALK (the empty element the four literals answer —
    // the merge sort the compareToIgnoreCase law orders, the span
    // the element's own tail names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=", &[], true, &[])
    else {
        panic!("/kill @a[sort= the value lane answers");
    };
    assert_eq!((start, length), (14, 0), "the after-'=' element span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["arbitrary", "furthest", "nearest", "random"],
        "the four sorts the merge sort orders"
    );
    assert!(
        matches.iter().all(|m| m.tooltip.is_none()),
        "the plain suggest no tooltip wears"
    );
    // THE PREFIX FOLD (the matchesSubStr law — 'ne' the nearest
    // answers, the span the typed element wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=ne", &[], true, &[])
    else {
        panic!("/kill @a[sort=ne the fold answers");
    };
    assert_eq!((start, length), (14, 2), "the typed element's span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["nearest"],
        "the 'ne' prefix the nearest answers"
    );
    // THE CASE-INSENSITIVE FOLD (the lowercase fold both sides
    // wear — 'NE' the nearest answers, the unknown value the
    // case-sensitive membership never completes)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[sort=NE", &[], true, &[])
    else {
        panic!("/kill @a[sort=NE the fold answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["nearest"],
        "the case fold the nearest answers"
    );
    // THE UNKNOWN ELEMENT (the throw the rollback keeps, the walk
    // the empty merge answers — no round-trip the silence spends)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=xyz", &[], true, &[])
    else {
        panic!("/kill @a[sort=xyz the empty lane answers");
    };
    assert_eq!((start, length), (14, 3));
    assert!(matches.is_empty(), "the unknown element the silence keeps");
    // THE VALID COMPLETE ELEMENT (the parseOptions continuation
    // the post-value state installs — the ',' separator + the ']'
    // close, the span the input's own end names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=nearest", &[], true, &[])
    else {
        panic!("/kill @a[sort=nearest the post-value state answers");
    };
    assert_eq!((start, length), (21, 0), "the parse cursor's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the punctuation the post-value state suggests"
    );
    // THE GAMEMODE WALK (the empty element BOTH arms wears — the
    // '!' forms the negative arm rides, the enum the GameType
    // values() banks, the merge sort the compareToIgnoreCase law
    // orders the '!' forms first)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[gamemode=", &[], true, &[])
    else {
        panic!("/kill @a[gamemode= the value lane answers");
    };
    assert_eq!((start, length), (18, 0), "the after-'=' element span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![
            "!adventure",
            "!creative",
            "!spectator",
            "!survival",
            "adventure",
            "creative",
            "spectator",
            "survival",
        ],
        "the both-arms walk the eight answers wear"
    );
    // THE TYPED '!' (the positive arm the strip clears — the
    // negative arm only, the span the '!' includes)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[gamemode=!", &[], true, &[])
    else {
        panic!("/kill @a[gamemode=! the negative arm answers");
    };
    assert_eq!((start, length), (18, 1), "the '!' the span includes");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["!adventure", "!creative", "!spectator", "!survival"],
        "the negative arm the four answers wear"
    );
    // THE '!'-PREFIXED FOLD (the substring the walk spends — 'cr'
    // the !creative answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[gamemode=!cr", &[], true, &[])
    else {
        panic!("/kill @a[gamemode=!cr the fold answers");
    };
    assert_eq!((start, length), (18, 3));
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["!creative"],
        "the '!cr' prefix the !creative answers"
    );
    // THE PLAIN FOLD (no '!' typed — the positive arm only)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[gamemode=cr", &[], true, &[])
    else {
        panic!("/kill @a[gamemode=cr the fold answers");
    };
    assert_eq!((start, length), (18, 2));
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["creative"],
        "the plain arm the creative answers"
    );
    // THE UNKNOWN GAMEMODE (the byName null the rollback keeps —
    // the empty merge the silence answers)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[gamemode=xyz", &[], true, &[])
    else {
        panic!("/kill @a[gamemode=xyz the empty lane answers");
    };
    assert!(matches.is_empty(), "the unknown mode the silence keeps");
    // THE VALID COMPLETE GAMEMODE (the post-value state the parse
    // continuation installs)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[gamemode=creative", &[], true, &[])
    else {
        panic!("/kill @a[gamemode=creative the post-value state answers");
    };
    assert_eq!((start, length), (26, 0));
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the punctuation the successful value wears"
    );
    // THE UNSPOKEN KEYS (the limit value walk the NUMERIC lane
    // the audited completion wears — the readInt law the "5"
    // completes, the punctuation the post-value state suggests —
    // see the_numeric_value_lanes test; the unknown key the
    // honest ask keeps)
    let Src::Local { start, matches, .. } =
        local_command_suggestions(&tree, "/kill @a[limit=5", &[], true, &[])
    else {
        panic!("/kill @a[limit=5 the numeric completion answers");
    };
    assert_eq!(start, 16, "the consumed value's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the limit walk the numeric completion wears"
    );
    // THE UNKNOWN KEY (the audited order law the unknown-key lane
    // wears — the get('so') null the rollback answers, the EMPTY
    // local the '='-carrying remaining the filter refuses — see
    // the_unknown_key_lane test)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[so=", &[], true, &[])
    else {
        panic!("/kill @a[so= the unknown-key walk answers");
    };
    assert_eq!((start, length), (9, 3), "the span the key start names");
    assert!(matches.is_empty(), "the unknown key the get() throws");
    // THE INTERNAL WHITESPACE (the stopped read the ws the
    // charset refuses — the walk RETURNS, the parse's own
    // post-modifier skipWhitespace (the audited 146-150) the
    // cursor refines, the suggestOptionsNextOrClose install (the
    // audited 153-160) the refined cursor wears — the re-entry
    // the '=' expectation throws, the state survives — the UNGATED
    // pair AT the refined cursor the retired ask wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=nearest x", &[], true, &[])
    else {
        panic!("/kill @a[sort=nearest x the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (22, 1),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the mid-tail junk the pair wears"
    );
    // THE COMMA AFTER THE VALUE (the key state the parseOptions
    // ',' consumes — the 21 keys re-answer, the value lane never)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[gamemode=creative,", &[], true, &[])
    else {
        panic!("/kill @a[gamemode=creative, the key state answers");
    };
    assert_eq!((start, length), (27, 0), "the fresh tail's own span");
    assert_eq!(
        matches.len(),
        20,
        "the ALL-21 MINUS the gated gamemode row — the consumed\
             gamemode=creative element the state SINGLE marks, the\
             canParseAny() false the registered bootStrap$31 predicate\
             returns, the gamemode row the canUse filter never answers"
    );
    // THE TAB APPLY (the selected '!creative' the replace lands —
    // the '!' the text carries, the span the input wears)
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/kill @a[gamemode=!cr".chars() {
        app.on_chat_char(c);
    }
    assert!(_rx.try_recv().is_err(), "the value lane no request spends");
    app.on_chat_tab_key();
    assert_eq!(
        app.chat_input, "/kill @a[gamemode=!creative",
        "the apply the '!'-text lands"
    );
}

/// THE STRING VALUE WALKS (javap `EntitySelectorOptions
/// lambda$bootStrap$0/$32/$42` — the name/team/tag modifiers the
/// disassembly banks: NO setSuggestions call the walks carry
/// (SUGGEST_NOTHING the value position keeps), the
/// readString/readUnquotedString read the ANY-unquoted-charset
/// tail consumes (the empty value included, the leading '!' the
/// shouldInvertValue consumes first) — the parse COMPLETES and
/// the post-value ',' + ']' punctuation suggests (the
/// suggestOptionsNextOrClose state); a char the unquoted set
/// refuses the value walk stops at — the parse's own ']'
/// expectation the ERROR lane wears, the parse-throw-catch the
/// EMPTY local answers (no round-trip the refusal spends); the
/// internal-whitespace tail the committed honest-ask gate keeps,
/// the numeric parses the readInt walks the next rounds' lanes
/// the honest ask keeps).
#[test]
fn the_selector_options_the_string_value_lanes_speak() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE NAME WALK (the empty value the readString walk banks —
    // the parse completes, the punctuation the post-value state
    // suggests, the span the parse cursor's own end names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=", &[], true, &[])
    else {
        panic!("/kill @a[name= the string lane answers");
    };
    assert_eq!((start, length), (14, 0), "the parse cursor's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the punctuation the post-value state suggests"
    );
    // THE TYPED VALUE (any unquoted-charset tail the walk
    // consumes — the parse completes the same)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve the parse completes");
    };
    assert_eq!((start, length), (19, 0), "the consumed value's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the value consumed the punctuation wears"
    );
    // THE INVERTED FORM (the '!' the shouldInvertValue consumes
    // first — the walk the same completion wears)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[name=!Steve", &[], true, &[])
    else {
        panic!("/kill @a[name=!Steve the invert walk answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the inverted value the same completion wears"
    );
    // THE EMPTY INVERTED VALUE (the '!' consumed, the empty read
    // the walk banks — the parse completes)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[name=!", &[], true, &[])
    else {
        panic!("/kill @a[name=! the empty inverted walk answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the empty inverted value the completion wears"
    );
    // THE TEAM WALK (the readUnquotedString read the $32 banks —
    // the same completion law)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[team=", &[], true, &[])
    else {
        panic!("/kill @a[team= the string lane answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the empty team value the completion wears"
    );
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[team=!red", &[], true, &[])
    else {
        panic!("/kill @a[team=!red the invert walk answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the inverted team the completion wears"
    );
    // THE TAG WALK (the $42 the plainest walk banks — no state
    // gate the modifier carries, the charset the '+'/'_' tails
    // wear)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[tag=", &[], true, &[])
    else {
        panic!("/kill @a[tag= the string lane answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the empty tag value the completion wears"
    );
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[tag=my_tag+1", &[], true, &[])
    else {
        panic!("/kill @a[tag=my_tag+1 the charset walk answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the '+'/'_' tails the unquoted charset wears"
    );
    // THE REFUSED CHAR (the '!' the value walk stops at AND
    // RETURNS — the parse's own ERROR_EXPECTED_END_OF_OPTIONS
    // throw the cursor the stop position names, the
    // suggestOptionsNextOrClose state the throw survives — the
    // UNGATED pair the ',' + ']' AT the junk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Ste!", &[], true, &[])
    else {
        panic!("/kill @a[name=Ste! the refused-char lane answers");
    };
    assert_eq!((start, length), (17, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the junk the post-value state wears"
    );
    // THE REFUSED CHAR AFTER THE INVERT (the '!' consumed first,
    // the second the walk stops at — the same junk lane the
    // post-invert cursor names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=!Ste!", &[], true, &[])
    else {
        panic!("/kill @a[name=!Ste! the refused-char lane answers");
    };
    assert_eq!((start, length), (18, 1), "the post-invert junk span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the post-invert junk the state wears"
    );
    // THE INTERNAL WHITESPACE (the readString walk the stopped
    // read the ws the unquoted set refuses wears — the walk
    // RETURNS, the parse's own skipWhitespace the cursor refines,
    // the pair install the refined cursor wears — the same law
    // the post-quote lane the '@a[name=\"a b\" x' span banks)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Ste v", &[], true, &[])
    else {
        panic!("/kill @a[name=Ste v the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (18, 1),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the internal-ws junk the pair wears"
    );
    // THE NUMERIC PARSES (the readInt/readDouble/Bounds walks
    // the numeric lanes now speak — the EMPTY value the
    // EXPECTED-NUMBER/ERROR-EMPTY throws answer, the EMPTY local
    // the parse-throw-catch's silence wears, the span the
    // element's own text names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=", &[], true, &[])
    else {
        panic!("/kill @a[limit= the numeric lane answers");
    };
    assert_eq!((start, length), (15, 0), "the empty value's own span");
    assert!(matches.is_empty(), "the empty readInt the silence keeps");
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[distance=", &[], true, &[])
    else {
        panic!("/kill @a[distance= the range lane answers");
    };
    assert!(matches.is_empty(), "the empty fromReader the silence keeps");
    // THE TAB APPLY (the selected ',' the zero-length insert
    // lands — the span the parse cursor's end wears)
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/kill @a[name=".chars() {
        app.on_chat_char(c);
    }
    assert!(_rx.try_recv().is_err(), "the string lane no request spends");
    app.on_chat_tab_key();
    assert_eq!(
        app.chat_input, "/kill @a[name=,",
        "the apply the ',' insert lands"
    );
}

/// THE QUOTED VALUE LANES (javap `EntitySelectorOptions
/// lambda$bootStrap$0` → `StringReader.readString` — the NAME
/// modifier's read the QUOTED lane carries; the bundled brigadier
/// 1.2.9 empirically probed: `"Steve x"` → `Steve x` cursor 9
/// (the internal whitespace the value walk consumes), `""` → ``,
/// `"a\"b"` → `a"b`, `"a\\b"` → `a\b`, `"Ste` → the
/// END-OF-QUOTE throw, `"a\zb"` → the INVALID-ESCAPE throw at the
/// escape, `'sq'` the single-quote form the same walk wears): a
/// TERMINATED quoted value the parse completes — the post-value
/// ',' + ']' punctuation the parse cursor's own end wears; the
/// UNTERMINATED/INVALID-ESCAPE/tail-after-close forms the throws
/// answer — the EMPTY local the parse-throw-catch's silence keeps
/// (no round-trip the throw spends); the '!' the shouldInvertValue
/// consumes BEFORE the quote (the inverted quoted form the same
/// law wears); the close-quote-not-last form the ']' expectation
/// dies at; the team/tag walks' readUnquotedString the quote
/// chars REFUSE (the '"' the unquoted charset never carries — the
/// EMPTY local the stopped walk wears); the quoted values the
/// internal-whitespace honest-ask gate BYPASS (the whitespace
/// inside the quotes the value walk never leaks).
#[test]
fn the_selector_options_the_quoted_value_lanes_speak() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE TERMINATED DOUBLE-QUOTE FORM (the internal space the
    // value walk consumes — the completion the punctuation wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"Steve x\"", &[], true, &[])
    else {
        panic!("/kill @a[name=\"Steve x\" the quoted walk answers");
    };
    assert_eq!((start, length), (23, 0), "the consumed value's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the terminated quote the punctuation wears"
    );
    // THE EMPTY QUOTED VALUE (the immediate close the walk banks)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[name=\"\"", &[], true, &[])
    else {
        panic!("/kill @a[name=\"\" the empty quoted walk answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the empty quoted value the completion wears"
    );
    // THE ESCAPED QUOTE (the backslash the terminator escapes —
    // the value a"b the walk builds)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[name=\"a\\\"b\"", &[], true, &[])
    else {
        panic!("/kill @a[name=\"a\\\"b\" the escape walk answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the escaped quote the completion wears"
    );
    // THE ESCAPED BACKSLASH (the pair the walk consumes)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[name=\"a\\\\b\"", &[], true, &[])
    else {
        panic!("/kill @a[name=\"a\\\\b\" the escape walk answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the escaped backslash the completion wears"
    );
    // THE SINGLE-QUOTE FORM (the readString the '\'' terminator
    // wears — the same completion)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[name='Steve x'", &[], true, &[])
    else {
        panic!("/kill @a[name='Steve x' the single-quote walk answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the single-quote form the completion wears"
    );
    // THE INVERTED QUOTED FORM (the '!' the shouldInvertValue
    // consumes first — the quote the value walk opens after)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[name=!\"Steve x\"", &[], true, &[])
    else {
        panic!("/kill @a[name=!\"Steve x\" the invert walk answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the inverted quoted value the completion wears"
    );
    // THE REFUSAL LANES (the unterminated quote, the invalid
    // escape, the dangling backslash — the EMPTY local the
    // throws' silence wears; the tail after the close the
    // tokenizer's own split law the ask wears — the post-value
    // junk the vanilla's ',' + ']' state the next rounds' lane
    // keeps)
    for (input, why) in [
        (
            "/kill @a[name=\"Ste",
            "the unterminated quote the END-OF-QUOTE throw answers",
        ),
        (
            "/kill @a[name=\"a\\zb\"",
            "the invalid escape the throw answers",
        ),
        (
            "/kill @a[name=\"a\\",
            "the dangling backslash the END-OF-QUOTE throw answers",
        ),
    ] {
        let Src::Local { matches, .. } = local_command_suggestions(&tree, input, &[], true, &[])
        else {
            panic!("{input} the quoted refusal answers");
        };
        assert!(matches.is_empty(), "{why}");
    }
    // THE POST-QUOTE TAIL THE REFINED JUNK WEARS (the bracket
    // guard the token whole delivers — the stopped read's cursor
    // the post-value skip the ws consumes, the 'x' the junk cursor
    // wears, the ',' + ']' pair AT the junk — the committed ask
    // the split premise wore retires)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"a b\" x", &[], true, &[])
    else {
        panic!("/kill @a[name=\"a b\" x the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (20, 1),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the post-quote junk the pair wears"
    );
    // THE TEAM/TAG WALKS THE QUOTES THE JUNK LANE WEARS (the
    // readUnquotedString walks the '"' the unquoted charset never
    // carries — the stopped read RETURNS EMPTY (the unquoted walk
    // never throws on the stop), the parse's own
    // ERROR_EXPECTED_END_OF_OPTIONS throw the cursor the quote
    // position names, the UNGATED pair the ',' + ']' at the junk
    // wears)
    for (key, span) in [("team", (14, 5)), ("tag", (13, 5))] {
        let Src::Local {
            start,
            length,
            matches,
        } = local_command_suggestions(&tree, &format!("/kill @a[{key}=\"red\""), &[], true, &[])
        else {
            panic!("/kill @a[{key}=\"red\" the unquoted refusal answers");
        };
        assert_eq!((start, length), span, "the span the quote cursor names");
        assert_eq!(
            matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
            vec![",", "]"],
            "the '{key}' walk the quote the junk lane wears"
        );
    }
    // THE NUMERIC KEY NEVER OPENS THE QUOTE (the readInt walk the
    // '"' the number charset refuses — the EXPECTED-NUMBER throw
    // the EMPTY local wears; the quoted form with the internal
    // whitespace the honest-ask gate keeps)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[limit=\"5\"", &[], true, &[])
    else {
        panic!("/kill @a[limit=\"5\" the numeric refusal answers");
    };
    assert!(
        matches.is_empty(),
        "the quoted numeric value the silence keeps"
    );
    // the quoted form WITH the internal whitespace the walk's OWN
    // death answers (the retired ask): the readInt the '"' the
    // number charset refuses — the EXPECTED-NUMBER throw the
    // walk spends BEFORE any ws the value carries (the ws inside
    // the quotes the numeric walk never sees), the EMPTY local
    // the same law the ws-free quoted form wears
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=\"5 5\"", &[], true, &[])
    else {
        panic!("/kill @a[limit=\"5 5\" the numeric refusal answers");
    };
    assert_eq!((start, length), (15, 5), "the value region's own span");
    assert!(matches.is_empty(), "the quoted numeric the silence keeps");
    // THE TAB APPLY (the completed quoted value the ',' insert
    // lands — no request the lane spends)
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/kill @a[name=\"Steve x\"".chars() {
        app.on_chat_char(c);
    }
    assert!(_rx.try_recv().is_err(), "the quoted lane no request spends");
    app.on_chat_tab_key();
    assert_eq!(
        app.chat_input, "/kill @a[name=\"Steve x\",",
        "the apply the ',' insert lands"
    );
}

/// THE QUOTED KEY LANE (javap `EntitySelectorParser.parseOptions`
/// — the KEY the `reader.readString()` walk reads, the quoted
/// grammar the value lane's readStringUntil probes bank;
/// `EntitySelectorOptions.get` — the exact-case lookup the
/// rollbackAndThrow keeps; the suggestNames state the '[' entry
/// installs — the unterminated reads the ALL-21 state wear (the
/// typed prefix the input-end throw wipes — the vanilla quirk the
/// read's consumption banks), the terminated reads the rollback
/// cursor the key start names — the quote-led remaining the
/// filter's silence wears; the known key the '=' expectation
/// consumes — the value walk the modifier speaks).
#[test]
fn the_selector_options_the_quoted_key_lane_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE KNOWN KEY THE QUOTES CARRY (the readString KEY walk —
    // the get() the exact-case lookup answers, the name modifier
    // the value walk rides, the completion the punctuation wears
    // at the input end)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"name\"=Steve", &[], true, &[])
    else {
        panic!("/kill @a[\"name\"=Steve the quoted key walk answers");
    };
    assert_eq!((start, length), (21, 0), "the consumed value's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the terminated key the value walk answers"
    );
    // THE SINGLE-QUOTE KEY (the '\'' terminator the same walk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a['name'=Steve", &[], true, &[])
    else {
        panic!("/kill @a['name'=Steve the single-quote key answers");
    };
    assert_eq!((start, length), (21, 0), "the single-quote form's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the single-quote key the value walk answers"
    );
    // THE WHITESPACE FORMS THE WALK'S OWN SKIPWHITESPACE LAWS THE
    // GUARD DELIVERS (the bracket guard the token whole speaks —
    // the key-'=' ws the 69-73 positions consume, the '='-value ws
    // the 125-129 positions — the parse completes, the pair the
    // input end wears; the committed ask the split premise the
    // routing wore retires — the walk's own skipWhitespace law the
    // parse's inside-selector walk keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"name\"  =Steve", &[], true, &[])
    else {
        panic!("/kill @a[\"name\"  =Steve the key-'=' ws law answers");
    };
    assert_eq!(
        (start, length),
        (23, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the key-'=' ws the parse consumes"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"name\"= Steve", &[], true, &[])
    else {
        panic!("/kill @a[\"name\"= Steve the post-'=' ws law answers");
    };
    assert_eq!(
        (start, length),
        (22, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the post-'=' ws the parse consumes"
    );
    // THE UNTERMINATED READS (the END-OF-QUOTE throw at the input
    // end — the suggestNames state survives, the remaining "" the
    // FULL table answers, the typed prefix the throw wipes — the
    // vanilla quirk)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"na", &[], true, &[])
    else {
        panic!("/kill @a[\"na the unterminated key answers");
    };
    assert_eq!(
        (start, length),
        (12, 0),
        "the input end the throw cursor wears"
    );
    assert_eq!(matches.len(), 21, "the ALL-21 state the throw keeps");
    assert_eq!(
        matches[0].text, "advancements=",
        "the merge sort the case-insensitive law orders"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"so", &[], true, &[])
    else {
        panic!("/kill @a[\"so the wiped-prefix quirk answers");
    };
    assert_eq!(
        (start, length),
        (12, 0),
        "the input end the throw cursor wears"
    );
    assert_eq!(matches.len(), 21, "the typed prefix the throw wipes");
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"", &[], true, &[])
    else {
        panic!("/kill @a[\" the bare open quote answers");
    };
    assert_eq!((start, length), (10, 0), "the bare quote's input end");
    assert_eq!(matches.len(), 21, "the bare quote the ALL-21 state wears");
    // THE DANGLING BACKSLASH (the escape state the input-end
    // END-OF-QUOTE throw keeps — the same ALL-21 state)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"a\\", &[], true, &[])
    else {
        panic!("/kill @a[\"a\\ the dangling backslash answers");
    };
    assert_eq!(
        (start, length),
        (12, 0),
        "the input end the escape state wears"
    );
    assert_eq!(matches.len(), 21, "the dangling backslash the ALL-21 keeps");
    // THE INVALID ESCAPE (the throw AT THE BACKSLASH — the
    // remaining the '\' leads, no key the filter answers — the
    // EMPTY local the span the escape position names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"a\\zb", &[], true, &[])
    else {
        panic!("/kill @a[\"a\\zb the invalid escape answers");
    };
    assert_eq!(
        (start, length),
        (11, 3),
        "the span the escape position names"
    );
    assert!(matches.is_empty(), "the '\\' remaining the filter refuses");
    // THE TERMINATED SILENCES (the '=' expectation's failure / the
    // unknown key the get() throws / the exact-case lookup the
    // casing refuses — BOTH the cursor the rollback names (the
    // key start), the quote-led remaining the filter refuses)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"name\"", &[], true, &[])
    else {
        panic!("/kill @a[\"name\" the '=' expectation answers");
    };
    assert_eq!((start, length), (9, 6), "the span the key start names");
    assert!(matches.is_empty(), "the '=' failure the silence keeps");
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"zzz\"=x", &[], true, &[])
    else {
        panic!("/kill @a[\"zzz\"=x the unknown key answers");
    };
    assert_eq!((start, length), (9, 7), "the span the rollback names");
    assert!(matches.is_empty(), "the unknown key the get() throws");
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"NAME\"=Steve", &[], true, &[])
    else {
        panic!("/kill @a[\"NAME\"=Steve the case walk answers");
    };
    assert_eq!(
        (start, length),
        (9, 12),
        "the span the exact-case rollback names"
    );
    assert!(
        matches.is_empty(),
        "the exact-case lookup the casing refuses"
    );
    // THE INCOMPLETE VALUE THE QUOTED KEY RIDES (the refused tail
    // the value walk stops at AND RETURNS — the junk lane the
    // stop cursor wears, the span the parse cursor to the input
    // end names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"name\"=Ste!", &[], true, &[])
    else {
        panic!("/kill @a[\"name\"=Ste! the refused tail answers");
    };
    assert_eq!((start, length), (19, 1), "the span the junk cursor wears");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the refused tail the junk lane wears"
    );
    // THE TAB APPLY (the completed quoted key the ',' insert
    // lands — no request the lane spends)
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/kill @a[\"name\"=Steve".chars() {
        app.on_chat_char(c);
    }
    assert!(
        _rx.try_recv().is_err(),
        "the quoted key lane no request spends"
    );
    app.on_chat_tab_key();
    assert_eq!(
        app.chat_input, "/kill @a[\"name\"=Steve,",
        "the apply the ',' insert lands"
    );
}

/// THE '='-CARRYING UNKNOWN KEYS (javap `EntitySelectorParser
/// .parseOptions` + `EntitySelectorOptions.get` — the
/// autopush-362 audit's ORDER LAW the bytecode re-confirms: the
/// KEY the get() resolves BEFORE the '=' expectation and the
/// SUGGEST_NOTHING install — the unknown key the
/// ERROR_UNKNOWN_OPTION rollbackAndThrow answers (the cursor the
/// KEY START names, the int arg the saved post-skipWhitespace
/// cursor wears), the '[' entry's suggestNames state the throw
/// survives, and the '='-carrying remaining no key prefixes (the
/// 21 keys the '=' never carries) — the EMPTY local the filter's
/// silence wears; the exact-case lookup the casing refuses, the
/// empty key the '='-charset read refuses, the invert-char keys
/// the read stops at — the same silence; the known keys the value
/// walk rides, the quote-led continuations the honest ask keeps).
#[test]
fn the_selector_options_the_unknown_key_lane_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE CANONICAL FLIP (the unquoted 'zzz=x' form — the get()
    // null the rollback answers, the span the KEY START names, the
    // '='-carrying remaining the filter refuses)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[zzz=x", &[], true, &[])
    else {
        panic!("/kill @a[zzz=x the unknown key answers");
    };
    assert_eq!((start, length), (9, 5), "the span the key start names");
    assert!(matches.is_empty(), "the unknown key the get() throws");
    // THE EXACT-CASE REFUSAL (the lookup the casing keeps — the
    // same null the uppercase form answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[NAME=Steve", &[], true, &[])
    else {
        panic!("/kill @a[NAME=Steve the exact-case walk answers");
    };
    assert_eq!((start, length), (9, 10), "the span the rollback names");
    assert!(matches.is_empty(), "the casing the lookup refuses");
    // THE INVERT-CHAR KEY (the unquoted read the '!' the charset
    // never carries stops at — the key '' the get() null answers,
    // the same key-start silence)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[!name=Steve", &[], true, &[])
    else {
        panic!("/kill @a[!name=Steve the invert-char walk answers");
    };
    assert_eq!((start, length), (9, 11), "the span the key start names");
    assert!(matches.is_empty(), "the stopped read the null answers");
    // THE EMPTY KEY (the '='-led tail the unquoted read NOTHING
    // consumes — the get("") null the same throw answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[=x", &[], true, &[])
    else {
        panic!("/kill @a[=x the empty-key walk answers");
    };
    assert_eq!((start, length), (9, 2), "the span the '=' position names");
    assert!(matches.is_empty(), "the empty key the null answers");
    // THE PREFIX '=' NEVER FUZZES (the exact-case lookup the 'lim'
    // key refuses — the value walk never rides the prefix)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[lim=5", &[], true, &[])
    else {
        panic!("/kill @a[lim=5 the prefix walk answers");
    };
    assert_eq!((start, length), (9, 5), "the span the key start names");
    assert!(matches.is_empty(), "the prefix key the lookup refuses");
    // THE CONTINUATION UNKNOWN (the ',' state the suggestNames
    // re-installs — the same throw the same span names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,zzz=x", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,zzz=x the continuation walk answers");
    };
    assert_eq!(
        (start, length),
        (20, 5),
        "the span the continuation key names"
    );
    assert!(
        matches.is_empty(),
        "the continuation unknown the silence keeps"
    );
    // THE KNOWN KEYS THE VALUE WALK RIDES (the flip never touches
    // — the first-element completion the input end wears, the
    // continuation sort list the value lane keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve the known-key walk answers");
    };
    assert_eq!((start, length), (19, 0), "the consumed value's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the known key the value walk answers"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,sort=", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,sort= the continuation ride answers");
    };
    assert_eq!((start, length), (25, 0), "the empty element's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["arbitrary", "furthest", "nearest", "random"],
        "the sort list the merge sort orders"
    );
    // THE EARLIER QUOTED VALUE THE TAIL NEVER HIDES (the parse
    // consumed the quoted value fine — the unquoted unknown key
    // after the ',' the same get() throw answers, the region's
    // quote the lane's own order law never reaches)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"S\",zzz=x", &[], true, &[])
    else {
        panic!("/kill @a[name=\"S\",zzz=x the post-quote walk answers");
    };
    assert_eq!((start, length), (18, 5), "the span the tail key names");
    assert!(
        matches.is_empty(),
        "the post-quote unknown the silence keeps"
    );
    // THE QUOTE-LED CONTINUATION THE KEY WALK ANSWERS (the
    // quote-aware split the flip wears — the ',' continuation the
    // suggestOptionsKey state re-installs, the unterminated read
    // the ALL-21 state at the input end keeps MINUS the gated
    // name row (the consumed name= element the state SINGLE
    // marks, the canParseAny() false the registered bootStrap$2
    // predicate returns), the typed prefix the throw wipes)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,\"zzz=x", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,\"zzz=x the quote-led continuation answers");
    };
    assert_eq!(
        (start, length),
        (26, 0),
        "the input end the throw cursor wears"
    );
    assert_eq!(matches.len(), 20, "the ALL-21 MINUS the gated name row");
    // THE TAB APPLY (the EMPTY local no request spends — the
    // input the silence never touches)
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/kill @a[zzz=x".chars() {
        app.on_chat_char(c);
    }
    assert!(
        _rx.try_recv().is_err(),
        "the unknown-key lane no request spends"
    );
    app.on_chat_tab_key();
    assert_eq!(
        app.chat_input, "/kill @a[zzz=x",
        "the empty list the input never touches"
    );
}

/// THE POST-VALUE JUNK LANE (javap `EntitySelectorParser
/// .parseOptions` — the audit_bsm13.py bytecode banks: the
/// suggestOptionsNextOrClose install (the offsets 153-160)
/// PRECEDES the canRead/','/']'/throw checks, the
/// ERROR_EXPECTED_END_OF_OPTIONS throw (the offsets 220-230) the
/// state survives, and the bsm#13 lambda is UNGATED — the bare
/// `suggest(',')` + `suggest(']')` pair, NO getRemaining gate —
/// so a value walk that RETURNS with the read stopped at a char
/// the charset refuses wears ',' + ']' AT the junk, the span the
/// parse cursor to the input end the fill's own law keeps; a
/// walk that THROWS (the failed parse, the closed lanes' pair,
/// the empty runs) the SUGGEST_NOTHING silence keeps — the
/// sort/gamemode/type modifiers' own providers the throw
/// survives, the committed folds speak; the junk the ',' applies
/// — the replace the range [cursor, input end) the Brigadier
/// StringRange law keeps).
#[test]
fn the_selector_options_the_post_value_junk_lane_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    let types = vec![
        "minecraft:pig".to_string(),
        "minecraft:zombie".to_string(),
        "minecraft:skeleton_horse".to_string(),
    ];
    // THE SORT JUNK (the readUnquotedString run 'nearest' the
    // switch matches — the walk RETURNS, the '!' the junk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=nearest!", &[], true, &[])
    else {
        panic!("/kill @a[sort=nearest! the sort junk answers");
    };
    assert_eq!((start, length), (21, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the sort junk the post-value state wears"
    );
    // THE SORT UNKNOWN (the switch's default the
    // rollbackAndThrow answers — the provider state the throw
    // keeps, the fold the silence wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=zz!", &[], true, &[])
    else {
        panic!("/kill @a[sort=zz! the sort unknown answers");
    };
    assert_eq!((start, length), (14, 3), "the element span the fold wears");
    assert!(matches.is_empty(), "the unknown sort the fold refuses");
    // THE GAMEMODE JUNK (the byName hit the walk returns — the
    // same junk lane)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[gamemode=creative!", &[], true, &[])
    else {
        panic!("/kill @a[gamemode=creative! the gamemode junk answers");
    };
    assert_eq!((start, length), (26, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the gamemode junk the state wears"
    );
    // THE GAMEMODE UNKNOWN (the byName miss the throw answers —
    // the provider's fold the silence keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[gamemode=zz!", &[], true, &[])
    else {
        panic!("/kill @a[gamemode=zz! the gamemode unknown answers");
    };
    assert_eq!((start, length), (18, 3), "the element span the fold wears");
    assert!(matches.is_empty(), "the unknown gamemode the fold refuses");
    // THE TYPE JUNK (the id run 'pig' the membership keeps — the
    // walk returns, the '!' the junk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=pig!", &[], true, &types)
    else {
        panic!("/kill @a[type=pig! the type junk answers");
    };
    assert_eq!((start, length), (17, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the type junk the state wears"
    );
    // THE TYPE UNKNOWN (the orElseThrow the membership miss
    // answers — the provider's fold the silence keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=zz!", &[], true, &types)
    else {
        panic!("/kill @a[type=zz! the type unknown answers");
    };
    assert_eq!((start, length), (14, 3), "the element span the fold wears");
    assert!(matches.is_empty(), "the unknown type the fold refuses");
    // THE TYPE TAG JUNK (the tag the parse never refutes — the
    // Identifier run the walk returns, the '!' the junk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(
        &tree,
        "/kill @a[type=#minecraft:skeletons!",
        &[],
        true,
        &types,
    )
    else {
        panic!("/kill @a[type=#minecraft:skeletons! the tag junk answers");
    };
    assert_eq!((start, length), (34, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the tag junk the state wears"
    );
    // THE BOUNDS JUNK (the single-value form the min run builds —
    // the pair keeps, the walk returns, the '!' the junk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[distance=5!", &[], true, &[])
    else {
        panic!("/kill @a[distance=5! the bounds junk answers");
    };
    assert_eq!((start, length), (19, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the bounds junk the state wears"
    );
    // THE BOUNDS RANGE JUNK (the max run '10' the pair keeps —
    // the '!' the junk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[distance=5..10!", &[], true, &[])
    else {
        panic!("/kill @a[distance=5..10! the range junk answers");
    };
    assert_eq!((start, length), (23, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the range junk the state wears"
    );
    // THE BOUNDS EMPTY MAX (the pair the closed lane keeps — the
    // walk THROWS, the SUGGEST_NOTHING silence the empty wears)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[distance=5..!", &[], true, &[])
    else {
        panic!("/kill @a[distance=5..! the empty-max refusal answers");
    };
    assert!(matches.is_empty(), "the empty max the pair throws");
    // THE ROTATION JUNK (the open lane's no-pair law the empty
    // max accepts — the walk returns, the '!' the junk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[x_rotation=45..!", &[], true, &[])
    else {
        panic!("/kill @a[x_rotation=45..! the rotation junk answers");
    };
    assert_eq!((start, length), (24, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the rotation junk the state wears"
    );
    // THE QUOTED TAIL JUNK (the readString walk RETURNS at the
    // close — the 'x' tail the junk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"Steve\"x", &[], true, &[])
    else {
        panic!("/kill @a[name=\"Steve\"x the quoted tail answers");
    };
    assert_eq!(
        (start, length),
        (21, 1),
        "the span the post-close cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the quoted tail the junk lane wears"
    );
    // THE QUOTED UNTERMINATED (the END-OF-QUOTE throw the walk's
    // own throw keeps — the SUGGEST_NOTHING silence)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[name=\"Steve", &[], true, &[])
    else {
        panic!("/kill @a[name=\"Steve the unterminated refusal answers");
    };
    assert!(matches.is_empty(), "the unterminated quote the throw keeps");
    // THE COORDINATE JUNK (the run '5' the parseDouble accepts —
    // the 'x' the junk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[x=5x", &[], true, &[])
    else {
        panic!("/kill @a[x=5x the coordinate junk answers");
    };
    assert_eq!((start, length), (12, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the coordinate junk the state wears"
    );
    // THE TAB APPLY (the ',' applies OVER the junk — the replace
    // the range [cursor, input end) the StringRange law keeps,
    // the junk the punctuation replaces — no request the lane
    // spends)
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/kill @a[sort=nearest!".chars() {
        app.on_chat_char(c);
    }
    assert!(_rx.try_recv().is_err(), "the junk lane no request spends");
    app.on_chat_tab_key();
    assert_eq!(
        app.chat_input, "/kill @a[sort=nearest,",
        "the apply the junk replace lands"
    );
}

/// THE QUOTE-AWARE CONTINUATION LANE (javap
/// `EntitySelectorParser.parseOptions` — the ',' the loop's own
/// check consumes as the element separator ONLY: the completed
/// key=value element it precedes, the in-quote ',' the
/// readStringUntil walk's own text carries (the naive rfind the
/// split retired — the in-quote comma the misanswer wore), the
/// fresh key read at a ',' the unquoted charset refuses (the
/// double-comma shape — the get("") null the throw keeps, the
/// tail the ',' leads), the second '=' the value read's own junk
/// poisons (the parse threw at it — the later ','s the separator
/// never earn); the suggestOptionsKey state the ',' consumption
/// re-installs — the QUOTE-LED continuation keys the readString
/// KEY walk the first element rides answers (the unterminated
/// reads the ALL-21 state at the input end, the terminated
/// unknown the rollback cursor the key start names, the invalid
/// escape the throw at the backslash keeps), and the fresh-name
/// state the consumed quoted values never hide (the region's
/// quotes the completed elements carry — the filter the tail
/// speaks).
#[test]
fn the_selector_options_the_quote_aware_continuation_lane_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE IN-QUOTE COMMA THE SPLIT RETIRES (the terminated quoted
    // value carrying ',' — the top-level separator the parse
    // consumes AFTER it, the value walk the completion wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"a,b\"", &[], true, &[])
    else {
        panic!("/kill @a[name=\"a,b\" the in-quote comma answers");
    };
    assert_eq!((start, length), (19, 0), "the consumed value's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the terminated quoted value the punctuation wears"
    );
    // THE CONTINUATION AFTER THE IN-QUOTE COMMA (the top-level
    // ',' the fresh-name state re-installs — the filter the tail
    // speaks; the naive rfind the IN-QUOTE comma the split wore)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"a,b\",so", &[], true, &[])
    else {
        panic!("/kill @a[name=\"a,b\",so the continuation answers");
    };
    assert_eq!((start, length), (20, 2), "the span the tail key names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["sort="],
        "the 'so' prefix the sort row wears"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"a,b\",na", &[], true, &[])
    else {
        panic!("/kill @a[name=\"a,b\",na the fresh-name walk answers");
    };
    assert_eq!((start, length), (20, 2), "the span the tail key names");
    assert!(
        matches.is_empty(),
        "the consumed name the canUse gate the name row hides — the SINGLE\
             state the canParseAny() false the registered predicate speaks\
             (the audited bootStrap$2 nameOption gate)"
    );
    // THE EMPTY TAIL (the ',' consumed, the loop the input end
    // exits — the suggestOptionsKey state the ALL-21 wears MINUS
    // the gated name row: the consumed name= element the state
    // SINGLE marks, the canParseAny() false the registered
    // bootStrap$2 predicate returns, the name row the canUse
    // filter never answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"a,b\",", &[], true, &[])
    else {
        panic!("/kill @a[name=\"a,b\", the empty tail answers");
    };
    assert_eq!((start, length), (20, 0), "the input end the state wears");
    assert_eq!(matches.len(), 20, "the ALL-21 MINUS the gated name row");
    assert_eq!(
        matches[0].text, "advancements=",
        "the merge sort the case-insensitive law orders"
    );
    // THE UNTERMINATED IN-QUOTE COMMA (no top-level ',' the quote
    // state hides — the first element the value walk rides, the
    // END-OF-QUOTE throw the SUGGEST_NOTHING silence keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"a,b", &[], true, &[])
    else {
        panic!("/kill @a[name=\"a,b the unterminated walk answers");
    };
    assert_eq!(
        (start, length),
        (14, 4),
        "the element span the silence wears"
    );
    assert!(matches.is_empty(), "the thrown read the silence keeps");
    // THE NAIVE-SPLIT MISANSWER RETIRED (the rfind the IN-QUOTE
    // comma the 'sort=x' tail wore — the sort lane the value walk
    // NEVER reaches: the name read's own END-OF-QUOTE throw the
    // parse answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"a,sort=x", &[], true, &[])
    else {
        panic!("/kill @a[name=\"a,sort=x the honest walk answers");
    };
    assert_eq!((start, length), (14, 9), "the name element span wears");
    assert!(matches.is_empty(), "the sort lane the quote state hides");
    // THE QUOTE-LED CONTINUATION KEYS (the ',' state the
    // suggestOptionsKey re-install — the readString KEY walk the
    // first element rides answers EVERY element)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,\"so", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,\"so the continuation key answers");
    };
    assert_eq!(
        (start, length),
        (23, 0),
        "the input end the throw cursor wears"
    );
    assert_eq!(
        matches.len(),
        20,
        "the ALL-21 MINUS the gated name row (the state the consumed\
             name= element SINGLE marks, the canParseAny gate hides)"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,'na", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,'na the single-quote continuation answers");
    };
    assert_eq!((start, length), (23, 0), "the input end the state wears");
    assert_eq!(
        matches.len(),
        20,
        "the single-quote form the same gate keeps"
    );
    // THE TERMINATED CONTINUATION KEY THE STATE GATE REFUSES (the
    // known key the '=' expectation consumes — the SECOND name=
    // element the SINGLE state's canParseElement gate THROWS (the
    // audited $0 order — the readString at 13-20, the gate at
    // 27-33 AFTER it), the ERROR_INAPPLICABLE_OPTION the parse
    // answers, NO provider the name modifier installs — the EMPTY
    // local the SUGGEST_NOTHING silence keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,\"name\"=Steve", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,\"name\"=Steve the terminated key answers");
    };
    assert_eq!(
        (start, length),
        (27, 5),
        "the synth value region the silence wears"
    );
    assert!(
        matches.is_empty(),
        "the second name= element the SINGLE gate throws"
    );
    // THE TERMINATED UNKNOWN CONTINUATION KEY (the get() throw
    // the rollback cursor the KEY START names — the quote-led
    // remaining the filter's silence wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,\"zzz\"=x", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,\"zzz\"=x the unknown key answers");
    };
    assert_eq!((start, length), (20, 7), "the span the key start names");
    assert!(
        matches.is_empty(),
        "the quote-led remaining the filter refuses"
    );
    // THE INVALID ESCAPE CONTINUATION KEY (the throw AT THE
    // BACKSLASH — the remaining the '\' leads, the EMPTY local
    // the span the escape position names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,\"a\\zb", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,\"a\\zb the invalid escape answers");
    };
    assert_eq!(
        (start, length),
        (22, 3),
        "the span the escape position names"
    );
    assert!(matches.is_empty(), "the '\\' remaining the filter refuses");
    // THE SECOND '=' THE SEGMENT POISONS (the value read's own
    // junk — the parse's ']' expectation threw at it, the later
    // ','s the separator NEVER earn — the tail the whole element
    // keeps, the junk lane the ',' + ']' AT the second '=' wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"x\"=y,so", &[], true, &[])
    else {
        panic!("/kill @a[name=\"x\"=y,so the poisoned segment answers");
    };
    assert_eq!((start, length), (17, 5), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the post-value state the throw survives"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=x=y,so", &[], true, &[])
    else {
        panic!("/kill @a[name=x=y,so the unquoted poison answers");
    };
    assert_eq!((start, length), (15, 5), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the unquoted form the same junk wears"
    );
    // THE DOUBLE COMMA (the fresh key read at the second ',' the
    // unquoted charset refuses — the get("") null the throw
    // keeps, the tail the ',' LEADS, the filter's silence the
    // span the second comma names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=x,,", &[], true, &[])
    else {
        panic!("/kill @a[name=x,, the double comma answers");
    };
    assert_eq!((start, length), (16, 1), "the span the second comma names");
    assert!(
        matches.is_empty(),
        "the ','-led remaining the filter refuses"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=x,,so", &[], true, &[])
    else {
        panic!("/kill @a[name=x,,so the double comma tail answers");
    };
    assert_eq!((start, length), (16, 3), "the tail the ',' leads wears");
    assert!(matches.is_empty(), "the ',so' remaining the filter refuses");
    // THE KEY-LESS SEGMENT (the fresh key 'y' the read consumes,
    // the get("y") null the rollback the KEY START names — the
    // remaining the key start to the input end the silence wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=x,y,so", &[], true, &[])
    else {
        panic!("/kill @a[name=x,y,so the key-less segment answers");
    };
    assert_eq!((start, length), (16, 4), "the span the key start names");
    assert!(
        matches.is_empty(),
        "the 'y,so' remaining the filter refuses"
    );
    // THE CONSUMED QUOTED VALUE THE FRESH-NAME STATE NEVER HIDES
    // (the region-level ask retired — the parse consumed the
    // quoted read, the ',' continuation the filter speaks)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"Steve x\",so", &[], true, &[])
    else {
        panic!("/kill @a[name=\"Steve x\",so the post-quote walk answers");
    };
    assert_eq!((start, length), (24, 2), "the span the tail key names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["sort="],
        "the 'so' prefix the sort row wears"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"Steve x\",na", &[], true, &[])
    else {
        panic!("/kill @a[name=\"Steve x\",na the fresh-name walk answers");
    };
    assert_eq!((start, length), (24, 2), "the span the tail key names");
    assert!(
        matches.is_empty(),
        "the consumed name= element the state SINGLE marks — the canUse\
             gate the name row hides (the audited bootStrap$2 canParseAny\
             predicate the suggestNames filter speaks)"
    );
    // THE TAB APPLY (the fresh-name local the canUse gate the row
    // never answers — the apply NOTHING the gate hides, the input
    // the silence keeps)
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/kill @a[name=\"a,b\",na".chars() {
        app.on_chat_char(c);
    }
    assert!(
        _rx.try_recv().is_err(),
        "the quote-aware lane no request spends"
    );
    app.on_chat_tab_key();
    assert_eq!(
        app.chat_input, "/kill @a[name=\"a,b\",na",
        "the gated name row the apply never lands — the input the\
             silence keeps"
    );
}

/// THE INVERTABLE SET OPTION STATE'S OWN GATES (javap 26.2
/// `InvertableSetOptionState` + the audited modifier lambdas —
/// the parser's four fields the constructor walk banks PERSIST
/// across the ',' elements of one parse, the SECOND element the
/// accumulated state's gates wear): the SINGLE state the second
/// positive AND negative element throws (the
/// ERROR_INAPPLICABLE_OPTION the rollbackAndThrow answers — the
/// EMPTY silence the string keys' SUGGEST_NOTHING install keeps),
/// the MULTIPLE state the negative/tag arms keeps (the parse
/// completes, the punctuation wears), the repeated tag the
/// tags-set refusal wears; the gamemode/type providers the
/// setSuggestions install PRECEDES the gate the throw survives
/// (the committed folds the Refused arm speaks); the canUse
/// gates the registered `(parser)Z` predicates speak ($2 name /
/// $31 gamemode / $34 team / $41 type) the fresh-key rows
/// filter; the earlier-element deaths the parse walk answers —
/// the unknown key / the missing '=' / the byName miss the EMPTY
/// silence keeps, the walk's own junk the survived NextOrClose
/// pair at the junk wears.
#[test]
fn the_selector_options_the_invertable_set_state_gates_speak() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    let roster = ["minecraft:cow".to_string(), "minecraft:pig".to_string()];
    // THE SECOND NAME ELEMENT THE SINGLE GATE THROWS (the first
    // `name=Steve` the state SINGLE marks, the second element's
    // canParseElement(false) the NONE==NONE compare refuses — the
    // audited $0 order the gate AFTER the read rides, NO provider
    // the name modifier installs — the EMPTY local the value
    // region's own span wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,name=", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,name= the state gate answers");
    };
    assert_eq!(
        (start, length),
        (25, 0),
        "the empty value region the silence wears"
    );
    assert!(
        matches.is_empty(),
        "the second name= the SINGLE gate throws"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,name=Alex", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,name=Alex the state gate answers");
    };
    assert_eq!(
        (start, length),
        (25, 4),
        "the consumed value region the silence wears"
    );
    assert!(
        matches.is_empty(),
        "the second name=Alex the SINGLE gate throws"
    );
    // THE NEGATIVE ELEMENT THE SINGLE GATE THROWS TOO (the
    // canParseNegativeElement the state!=SINGLE compare keeps —
    // the SINGLE state the '!' arm refuses)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,name=!Al", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,name=!Al the state gate answers");
    };
    assert_eq!(
        (start, length),
        (25, 3),
        "the inverted value region the silence wears"
    );
    assert!(
        matches.is_empty(),
        "the second name=! the SINGLE gate throws"
    );
    // THE MULTIPLE STATE THE NEGATIVE ARMS KEEPS (the first
    // `name=!a` the state MULTIPLE marks, the second `name=!b`
    // the canParseNegativeElement the MULTIPLE!=SINGLE compare
    // accepts — the parse completes, the punctuation the
    // post-value state wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=!a,name=!b", &[], true, &[])
    else {
        panic!("/kill @a[name=!a,name=!b the multiple gate answers");
    };
    assert_eq!(
        (start, length),
        (24, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the second negative the MULTIPLE gate keeps"
    );
    // THE POSITIVE ELEMENT THE MULTIPLE GATE THROWS (the
    // canParsePositiveElement the NONE==NONE compare the MULTIPLE
    // state never passes)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=!a,name=", &[], true, &[])
    else {
        panic!("/kill @a[name=!a,name= the multiple gate answers");
    };
    assert_eq!(
        (start, length),
        (22, 0),
        "the empty value region the silence wears"
    );
    assert!(
        matches.is_empty(),
        "the second positive the MULTIPLE gate throws"
    );
    // THE TEAM LANES THE SAME GATES WEAR (the audited $32 — the
    // check after the read the name walk's own carries)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[team=a,team=", &[], true, &[])
    else {
        panic!("/kill @a[team=a,team= the state gate answers");
    };
    assert_eq!(
        (start, length),
        (21, 0),
        "the empty value region the silence wears"
    );
    assert!(
        matches.is_empty(),
        "the second team= the SINGLE gate throws"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[team=!a,team=!b", &[], true, &[])
    else {
        panic!("/kill @a[team=!a,team=!b the multiple gate answers");
    };
    assert_eq!(
        (start, length),
        (24, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the second negative team the MULTIPLE gate keeps"
    );
    // THE TYPE LANES THE GATES WEAR (the audited $35 — the gate
    // BEFORE the read the plain lane keeps, the id the roster
    // membership answers): the second positive the SINGLE gate
    // throws — the provider the setSuggestions install PRECEDES
    // the gate the throw survives, the fold rows the remaining
    // wears — and the canUse gate the fresh-key rows filter (the
    // 't' prefix the type row the SINGLE state hides, the tag row
    // the same lane keeps; the MULTIPLE state both rows answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=cow,type=", &[], true, &roster)
    else {
        panic!("/kill @a[type=cow,type= the state gate answers");
    };
    assert_eq!(
        (start, length),
        (23, 0),
        "the empty value region the fold wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![
            "!minecraft:cow",
            "!minecraft:pig",
            "minecraft:cow",
            "minecraft:pig"
        ],
        "the provider the gate's throw survives (the BOTH-arms fold)"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=cow,t", &[], true, &roster)
    else {
        panic!("/kill @a[type=cow,t the canUse gate answers");
    };
    assert_eq!((start, length), (18, 1), "the tail key span wears");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["tag=", "team="],
        "the type row the SINGLE state's canParseAny hides — the\
             't' prefix the tag/team rows the same lane keeps"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=!cow,t", &[], true, &roster)
    else {
        panic!("/kill @a[type=!cow,t the canUse gate answers");
    };
    assert_eq!((start, length), (19, 1), "the tail key span wears");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["tag=", "team=", "type="],
        "the MULTIPLE state all three rows answers"
    );
    // THE TAG LANES THE TAGS SET SPEAKS (the audited $35 tag lane
    // — the canParseAnyTag gate BEFORE the read, the canParseTag
    // repeat refusal AFTER: the distinct tags the MULTIPLE state
    // keeps, the parse completes; the repeated tag the refusal
    // the provider's silence wears — the fold rows the '#'-led
    // remaining never answers over the tag-table-less caller)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=#a,type=#b", &[], true, &[])
    else {
        panic!("/kill @a[type=#a,type=#b the tag lane answers");
    };
    assert_eq!(
        (start, length),
        (24, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the distinct tags the MULTIPLE state keeps"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=#a,type=#a", &[], true, &[])
    else {
        panic!("/kill @a[type=#a,type=#a the tag lane answers");
    };
    assert_eq!(
        (start, length),
        (22, 2),
        "the repeated tag the silence wears"
    );
    assert!(
        matches.is_empty(),
        "the repeated tag the tags-set refusal keeps"
    );
    // THE GAMEMODE PROVIDER THE GATE'S THROW SURVIVES (the
    // setSuggestions install at the audited 5-12 PRECEDES the
    // canParseElement check at 28-33 — the fold rows the
    // remaining wears: the empty element BOTH arms, the typed
    // 'cre' the plain arm filters)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(
        &tree,
        "/kill @a[gamemode=survival,gamemode=",
        &[],
        true,
        &[],
    )
    else {
        panic!("/kill @a[gamemode=survival,gamemode= the state gate answers");
    };
    assert_eq!(
        (start, length),
        (36, 0),
        "the empty value region the fold wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![
            "!adventure",
            "!creative",
            "!spectator",
            "!survival",
            "adventure",
            "creative",
            "spectator",
            "survival"
        ],
        "the provider the gate's throw survives (the BOTH-arms fold)"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(
        &tree,
        "/kill @a[gamemode=survival,gamemode=cre",
        &[],
        true,
        &[],
    )
    else {
        panic!("/kill @a[gamemode=survival,gamemode=cre the state gate answers");
    };
    assert_eq!(
        (start, length),
        (36, 3),
        "the typed remaining the fold wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["creative"],
        "the plain arm the 'cre' prefix filters"
    );
    // THE CANUSE GATE THE NAME ROW HIDES (the 'n' prefix the name
    // row alone answers — the SINGLE state's canParseAny() false
    // the registered bootStrap$2 predicate returns, the EMPTY
    // local the filter's silence keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,n", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,n the canUse gate answers");
    };
    assert_eq!((start, length), (20, 1), "the tail key span wears");
    assert_eq!(
            matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
            vec!["nbt="],
            "the name row the canUse gate hides — the 'n' prefix the nbt\n             row the same lane keeps"
        );
    // THE NUMERIC KEY THE SET-ONCE STATE SPEAKS (the limit walk
    // the modifier never gates — the SECOND element the parse
    // ACCEPTS (the audited $23 — NO canParse the body carries) —
    // but the ROW the state hides: the audited $24 canParse the
    // first element's mark the row wears — the 20 rows the
    // Invertable family's own law shadows)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5,", &[], true, &[])
    else {
        panic!("/kill @a[limit=5, the untouched lane answers");
    };
    assert_eq!((start, length), (17, 0), "the input end the state wears");
    assert_eq!(matches.len(), 20, "the parsed limit the own row hides");
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=cow,limit=5", &[], true, &roster)
    else {
        panic!("/kill @a[type=cow,limit=5 the mixed lanes answer");
    };
    assert_eq!(
        (start, length),
        (25, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the limit walk the state gates never touch"
    );
    // THE EARLIER-ELEMENT DEATHS THE PARSE WALK ANSWERS (the
    // elements the earned separators leave behind the parse
    // walks before the tail's own lane: the unknown key the
    // get() throw answers, the byName miss the walk's own throw
    // keeps — the EMPTY local the SUGGEST_NOTHING silence wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[zzz=x,name=St", &[], true, &[])
    else {
        panic!("/kill @a[zzz=x,name=St the earlier death answers");
    };
    assert_eq!((start, length), (9, 5), "the dead element's own span wears");
    assert!(
        matches.is_empty(),
        "the earlier unknown key the throw silences"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[gamemode=zzz,name=St", &[], true, &[])
    else {
        panic!("/kill @a[gamemode=zzz,name=St the earlier death answers");
    };
    assert_eq!(
        (start, length),
        (9, 12),
        "the dead element's own span wears"
    );
    assert!(
        matches.is_empty(),
        "the earlier byName miss the throw silences"
    );
    // THE EARLIER JUNK THE SURVIVED STATE WEARS (the walk RETURNS
    // at the refused char — the suggestOptionsNextOrClose install
    // the offsets 153-160 the throw survives — the ',' + ']' pair
    // AT the junk the UNGATED lambda answers, the span the fill's
    // own cursor law keeps to the input end)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Ste!e,so", &[], true, &[])
    else {
        panic!("/kill @a[name=Ste!e,so the earlier junk answers");
    };
    assert_eq!(
        (start, length),
        (17, 5),
        "the junk span the input end names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the survived state the punctuation at the junk wears"
    );
}

/// THE SET-ONCE SINGLE-PARSE GATES (javap 26.2 — the
/// `SetOnceOptionState` classfile + the parser's
/// limitedOption/sortedOption/scoresOption/advancementsOption
/// fields + the modifier/canUse lambdas the audit disassembles):
/// the SECOND limit=/sort= element the parse ACCEPTS (the audited
/// $23/$25 — NO canParse, NO ERROR_INAPPLICABLE_OPTION the
/// modifier bodies carry — the mark idempotent, setMaxResults/
/// setOrder overwrite — the name=/team= Invertable law the SetOnce
/// family NEVER wears), the ROWS the registered canUse predicates
/// hide instead ($24 limit / $27 sort / $48 scores / $54
/// advancements — the first-parse mark the own row wears, the
/// limit+sort rows the isCurrentEntity '@s' law ALSO hides — the
/// parseSelector 's' branch the offsets 243-245 bank the ONLY
/// currentEntity putfield), the LIMIT walk the `< 1` check the
/// ERROR_LIMIT_TOO_SMALL throw answers BEFORE setMaxResults (the
/// audited 16-29 — the junked read the same law rides, the value
/// the STOPPED read carries), the markParsed the hit path wears
/// (the Complete AND the Junk — the throw the mark never sees),
/// the '{' map values the honest ask keeps (the scores/
/// advancements state the map-value lane never reaches).
#[test]
fn the_selector_options_the_set_once_single_parse_gates_speak() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE SECOND LIMIT ELEMENT THE PARSE ACCEPTS (the headline —
    // the Invertable SINGLE gate the name= law wears NEVER fires:
    // the modifier runs setMaxResults+markParsed, the parse
    // completes, the punctuation the post-value state wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5,limit=5", &[], true, &[])
    else {
        panic!("/kill @a[limit=5,limit=5 the parse accepts");
    };
    assert_eq!(
        (start, length),
        (24, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the second limit= the parse accepts"
    );
    // THE SECOND SORT ELEMENT THE PARSE ACCEPTS TOO (the audited
    // $25 — the setOrder overwrite the mark rides)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5,sort=nearest", &[], true, &[])
    else {
        panic!("/kill @a[limit=5,sort=nearest the parse accepts");
    };
    assert_eq!(
        (start, length),
        (29, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the sort= element after the limit= the parse accepts"
    );
    // THE LIMIT ROW THE FIRST ELEMENT'S MARK HIDES (the audited
    // $24 canParse — the 'l' tail the level= row ALONE answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5,l", &[], true, &[])
    else {
        panic!("/kill @a[limit=5,l the row gates answer");
    };
    assert_eq!((start, length), (17, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["level="],
        "the parsed limit the own row hides"
    );
    // THE SORT ROW THE FIRST ELEMENT'S MARK HIDES (the audited
    // $27 — the 'so' tail NO key the silence wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=nearest,so", &[], true, &[])
    else {
        panic!("/kill @a[sort=nearest,so the row gates answer");
    };
    assert_eq!((start, length), (22, 2), "the fresh tail's own span");
    assert!(matches.is_empty(), "the parsed sort the own row hides");
    // THE SCORES ROW THE 's' TAIL KEEPS (the audited $48 — no
    // isCurrentEntity the lambda carries, the scores state the
    // sort element never touches)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=nearest,s", &[], true, &[])
    else {
        panic!("/kill @a[sort=nearest,s the row gates answer");
    };
    assert_eq!((start, length), (22, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["scores="],
        "the unparsed scores the own row keeps"
    );
    // THE '@s' LAW THE LIMIT+SORT ROWS HIDE (the audited $24/$27
    // isCurrentEntity FIRST — the currentEntity field the 's'
    // branch sets, the rows the single-target law hides BEFORE
    // the state ever speaks)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @s[l", &[], true, &[])
    else {
        panic!("/kill @s[l the row gates answer");
    };
    assert_eq!((start, length), (9, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["level="],
        "the '@s' selector the limit row hides"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @s[so", &[], true, &[])
    else {
        panic!("/kill @s[so the row gates answer");
    };
    assert_eq!((start, length), (9, 2), "the fresh tail's own span");
    assert!(matches.is_empty(), "the '@s' selector the sort row hides");
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @s[s", &[], true, &[])
    else {
        panic!("/kill @s[s the row gates answer");
    };
    assert_eq!((start, length), (9, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["scores="],
        "the '@s' selector the scores row keeps"
    );
    // THE '@a' SELECTOR THE SAME TAILS THE LIMIT ROW KEEPS (the
    // 'p'/'r'/'a' branches the currentEntity field never touch)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[l", &[], true, &[])
    else {
        panic!("/kill @a[l the row gates answer");
    };
    assert_eq!((start, length), (9, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["level=", "limit="],
        "the '@a' selector the limit row keeps (the merge sort orders)"
    );
    // THE '@s' SELECTOR THE LIMIT VALUE WALK THE PARSE STILL
    // SPEAKS (the modifiers the canUse gates NEVER carry — the
    // row the law hides, the parse itself accepts)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @s[limit=5", &[], true, &[])
    else {
        panic!("/kill @s[limit=5 the value walk answers");
    };
    assert_eq!(
        (start, length),
        (16, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the '@s' limit= the parse accepts"
    );
    // THE TOO-SMALL LAW THE `< 1` CHECK ANSWERS (the audited 16-29
    // — the rollbackAndThrow BEFORE setMaxResults — the
    // SUGGEST_NOTHING silence the throw keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=0", &[], true, &[])
    else {
        panic!("/kill @a[limit=0 the too-small law answers");
    };
    assert_eq!((start, length), (15, 1), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the zero limit the TOO-SMALL throw silences"
    );
    // THE JUNKED READ THE SAME `< 1` LAW RIDES (the modifier runs
    // the check on the value the STOPPED read returns — the `0x`
    // the throw answers BEFORE the junk lane the `5x` wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=0x", &[], true, &[])
    else {
        panic!("/kill @a[limit=0x the too-small law answers");
    };
    assert_eq!((start, length), (15, 2), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the zero-with-junk the TOO-SMALL throw silences"
    );
    // THE TOO-SMALL THROW THE STATE NEVER MARKS (the earlier
    // `limit=0` element the parse throws AT — the next element
    // the parse never reaches, the EMPTY local the silence wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=0,l", &[], true, &[])
    else {
        panic!("/kill @a[limit=0,l the earlier death answers");
    };
    assert_eq!((start, length), (9, 7), "the dead element's own span wears");
    assert!(
        matches.is_empty(),
        "the too-small element the parse silences"
    );
    // THE CROSS-FAMILY ROWS (the Invertable name gate the name=
    // element wears AND the SetOnce limit row the 'l' tail keeps
    // — the eight gated keys the ONE canUse fold filters)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve,l", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve,l the row gates answer");
    };
    assert_eq!((start, length), (20, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["level=", "limit="],
        "the name row hides, the limit row the 'l' tail keeps"
    );
    // THE QUOTED-KEY LANE THE '@s' GATES WEAR (the ALL-21 state
    // the unterminated read wears — the limit+sort rows the
    // isCurrentEntity law hides, the 19 rows the merge sort
    // orders; the '@a' selector the FULL 21 keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @s[\"l", &[], true, &[])
    else {
        panic!("/kill @s[\"l the quoted-key lane answers");
    };
    assert_eq!(
        (start, length),
        (11, 0),
        "the input end the ALL-21 state wears"
    );
    assert_eq!(matches.len(), 19, "the '@s' rows the two gates hide");
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"l", &[], true, &[])
    else {
        panic!("/kill @a[\"l the quoted-key lane answers");
    };
    assert_eq!(
        (start, length),
        (11, 0),
        "the input end the ALL-21 state wears"
    );
    assert_eq!(matches.len(), 21, "the '@a' rows the FULL table keeps");
    // THE MAP VALUES THE LAWS SPLIT (the scores compound the walk
    // speaks — the SNBT ':' form the '=' expectation the earlier
    // element THROWS at, the DEAD element the EMPTY silence
    // wears; the advancements value the unspoken walk the ask
    // keeps — the region-level flag the entry check answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={foo:1},l", &[], true, &[])
    else {
        panic!("/kill @a[scores={{foo:1}},l the earlier death answers");
    };
    assert_eq!(
        (start, length),
        (9, 14),
        "the dead element's own span wears"
    );
    assert!(
        matches.is_empty(),
        "the ':' element the '=' expectation silences"
    );
    assert_eq!(
        local_command_suggestions(&tree, "/kill @a[advancements=", &[], true, &[]),
        Src::AskServer,
        "the advancements value the unspoken walk the ask keeps"
    );
}

/// THE SCORES COMPOUND WALK (javap 26.2 `EntitySelectorOptions
/// lambda$bootStrap$46` — the walk the region-level ask retires):
/// the '{'-led compound the walk consumes (the empty compound the
/// head exit wears, the '='-led key=range pairs the loop banks,
/// the IN-BRACE ',' the compound's own loop consumes — the
/// top-level separator the depth never earns —, the trailing ','
/// the head's '}' re-check keeps, the Ints ranges the bounds walk
/// speaks), the chars after the '}' the parse's own junk lane
/// wears, the unterminated compound the expect-'}' at the input
/// end silences, the SNBT ':' form the '=' expectation silences,
/// the markParsed the successful close wears (the scores row the
/// continuation hides, the '@s' limit law the same rows keep),
/// the nbt/advancements/predicate compounds the honest ask keeps.
#[test]
fn the_selector_options_the_scores_compound_walk_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE EMPTY COMPOUND THE HEAD EXIT WEARS (the canRead/peek '}'
    // law — the isEmpty the addPredicate skips, the markParsed the
    // close wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{}} the compound walk answers");
    };
    assert_eq!(
        (start, length),
        (18, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the empty compound the parse completes"
    );
    // THE '='-LED PAIR THE LOOP BANKS (the objective read, the '='
    // expectation, the Ints.fromReader range)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}} the compound walk answers");
    };
    assert_eq!(
        (start, length),
        (21, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the key=range pair the parse completes"
    );
    // THE IN-BRACE ',' THE COMPOUND'S OWN LOOP CONSUMES (the
    // audited 94-95 skip + the 98→19 re-loop — the top-level
    // separator the depth NEVER earns, the element the compound
    // owns whole)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1,b=2}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1,b=2}} the compound walk answers");
    };
    assert_eq!(
        (start, length),
        (25, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the in-brace comma the compound consumes"
    );
    // THE TRAILING ',' THE HEAD'S '}' RE-CHECK KEEPS (the consumed
    // separator the head re-runs — the '}' the exit wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1,}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1,}} the compound walk answers");
    };
    assert_eq!(
        (start, length),
        (22, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the trailing comma the parse accepts"
    );
    // THE INTS RANGE FORM THE BOUNDS WALK SPEAKS (the fromReader
    // '..' separator the level lane's own law rides)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1..5}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1..5}} the compound walk answers");
    };
    assert_eq!(
        (start, length),
        (24, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the range form the parse completes"
    );
    // THE CHARS AFTER THE '}' THE JUNK LANE WEARS (the compound
    // consumed, the parse's own death at the leftover — the
    // UNGATED pair AT the junk)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1}!", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}}! the junk lane answers");
    };
    assert_eq!((start, length), (21, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the leftover char the junk lane wears"
    );
    // THE UNTERMINATED COMPOUND THE EXPECT-'}' AT THE INPUT END
    // SILENCES (the canRead FALSE exit the same expect throws —
    // the mark the throw never sees)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1 the unterminated law answers");
    };
    assert_eq!((start, length), (16, 4), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the unterminated compound the silence keeps"
    );
    // THE '=' EXPECTATION FAILURES THE COMPOUND WEARS (the SNBT
    // ':' form the committed flip re-pins beside the missing '='
    // and the '{'-less value)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[scores={a}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a}} the '=' expectation answers");
    };
    assert!(matches.is_empty(), "the missing '=' the throw silences");
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[scores={a=}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=}} the fromReader law answers");
    };
    assert!(
        matches.is_empty(),
        "the empty range the ERROR-EMPTY silences"
    );
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[scores=x", &[], true, &[])
    else {
        panic!("/kill @a[scores=x the expect-'{{' law answers");
    };
    assert!(matches.is_empty(), "the '{{'-less value the throw silences");
    // THE MARK THE SUCCESSFUL CLOSE WEARS (the earlier element
    // marks, the scores row the continuation hides — the 's' tail
    // the sort= row ALONE answers, the empty tail the 20 rows
    // wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1,b=2},s", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1,b=2}},s the row gates answer");
    };
    assert_eq!((start, length), (26, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["sort="],
        "the parsed scores the own row hides"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1},", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}}, the row gates answer");
    };
    assert_eq!((start, length), (22, 0), "the fresh tail's own span");
    assert_eq!(matches.len(), 20, "the parsed scores the own row hides");
    // THE 'l' TAIL THE OTHER ROWS KEEPS (the level= and the
    // unparsed limit= the 'l' prefix answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1},l", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}},l the row gates answer");
    };
    assert_eq!((start, length), (22, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["level=", "limit="],
        "the scores row hides, the other rows the 'l' tail keeps"
    );
    // THE '@s' LAW THE SAME ROWS WEAR (the mark hides the scores
    // row, the isCurrentEntity law hides the limit row — the
    // level= row ALONE answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @s[scores={a=1},l", &[], true, &[])
    else {
        panic!("/kill @s[scores={{a=1}},l the row gates answer");
    };
    assert_eq!((start, length), (22, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["level="],
        "the '@s' rows the two gates hide"
    );
    // THE OTHER MAP KEYS THE HONEST ASK KEEPS (the nbt compound
    // the flag asks — the scores element beside it the ask the
    // region owns, the lone nbt element the same law wears)
    assert_eq!(
        local_command_suggestions(&tree, "/kill @a[scores={a=1},nbt={b:1b}", &[], true, &[]),
        Src::AskServer,
        "the nbt compound the honest ask keeps"
    );
    assert_eq!(
        local_command_suggestions(&tree, "/kill @a[nbt={},l", &[], true, &[]),
        Src::AskServer,
        "the lone nbt element the honest ask keeps"
    );
    // THE INTERNAL-WS COMPOUND THE WALK'S OWN WS LAWS SPEAK (the
    // audited 15-16/35-36/44-45/54-55/74-75 loop positions the
    // in-brace ws banks — the element check the compound's own
    // ground the exemption retires, the walk the '{'-led form
    // answers): the '='-flanked ws the key-run/ws laws consume,
    // the parse completes, the punctuation the input end wears
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a = 1}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a = 1}} the ws-form walk answers");
    };
    assert_eq!(
        (start, length),
        (23, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the '='-flanked ws the loop consumes"
    );
    // THE PRE-HEAD WS THE LOOP'S OWN 15-16 POSITIONS SKIP (the '{'
    // the expect consumed, the ws before the key run)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={ a=1}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{ a=1}} the pre-head ws law answers");
    };
    assert_eq!(
        (start, length),
        (22, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the pre-head ws the loop skips"
    );
    // THE 54-55 WS THE '=' CONSUMPTION SKIPS (the ws between '='
    // and the range the fromReader never sees)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a= 1}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a= 1}} the post-'=' ws law answers");
    };
    assert_eq!(
        (start, length),
        (22, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the post-'=' ws the loop skips"
    );
    // THE 74-75 WS THE POST-RANGE POSITION SKIPS (the ws before
    // the close the loop head's peek '}' exits at)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1 }", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1 }} the post-range ws law answers");
    };
    assert_eq!(
        (start, length),
        (22, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the post-range ws the loop head exits past"
    );
    // THE WS BEFORE THE IN-BRACE ',' THE 74-75 POSITIONS SKIP (the
    // consumed separator the head re-runs — the SECOND pair the
    // loop walks)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1 ,b=2}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1 ,b=2}} the in-brace ws law answers");
    };
    assert_eq!(
        (start, length),
        (26, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the ws-led separator the loop consumes"
    );
    // THE '=' EXPECTATION THE WS-THEN-NON-'=' REFUSES (the 44-45
    // ws the loop skips, the 'b' the expect-'=' throws at — the
    // SUGGEST_NOTHING silence the modifier's own death keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a b=1}", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a b=1}} the '=' expectation answers");
    };
    assert_eq!((start, length), (16, 7), "the value region's own span");
    assert!(matches.is_empty(), "the missing '=' the throw silences");
    // THE POST-CLOSE JUNK THE REFINED VERDICT NAMES (the in-brace
    // ws the compound consumed, the 'x' after the '}' the parse's
    // own death wears — the UNGATED pair AT the junk)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1 ,b=2}x", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1 ,b=2}}x the junk lane answers");
    };
    assert_eq!((start, length), (26, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the post-close junk the pair wears"
    );
    // THE POST-CLOSE WS THE PARSE'S OWN SKIP NAMES (the audited
    // 146-150 the compound walk's refinement rides — the junk
    // cursor the ws the skip consumes, the 'x' the parse's own
    // death wears, the ',' + ']' pair AT the junk — the committed
    // ask the split premise wore retires)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1 ,b=2} x", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1 ,b=2}} x the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (27, 1),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the post-close ws the junk lane wears"
    );
    // THE UNTERMINATED WS-FORM THE EXPECT-'}' AT THE INPUT END
    // SILENCES (the in-brace ws the loop banks, the canRead FALSE
    // exit the same expect throws — the mark the throw never sees)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a =1", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a =1 the unterminated law answers");
    };
    assert_eq!((start, length), (16, 5), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the unterminated ws-form the silence keeps"
    );
    // THE WS-FORM COMPOUND THE PARSE MARKS (the successful close
    // the markParsed wears — the scores row the continuation
    // hides, the 's' tail the sort= row ALONE answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a =1},s", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a =1}},s the row gates answer");
    };
    assert_eq!((start, length), (23, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["sort="],
        "the ws-form close the mark wears"
    );
    // THE EARLIER WS-FORM COMPOUND THE PARSE WALKS (the element
    // the parse consumes — the in-brace ws the audited positions
    // the loop skips, the mark the close wears, the 'l' tail the
    // level= + limit= rows the gates keep)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a =1},l", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a =1}},l the earlier ws-form answers");
    };
    assert_eq!((start, length), (23, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["level=", "limit="],
        "the ws-form close the mark wears, the other rows the 'l' tail keeps"
    );
    // THE POST-CLOSE WS THE EARNED SEPARATOR RIDES (the bracket
    // guard the token whole delivers — the ',' the ARMED segment
    // earns past the skip the parse rides, the element the parse
    // consumed the mark wears, the 'x' the walk's own junk lane
    // names — the ',' + ']' pair AT the junk —, the committed ask
    // the split premise wore retires)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1} x,l", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}} x,l the earned separator answers");
    };
    assert_eq!(
        (start, length),
        (22, 3),
        "the span the junk cursor to the input end keeps"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the walk's own junk the pair wears"
    );
}

/// THE CLOSE BRACKET THE PARSE CONSUMES (javap `EntitySelectorParser
/// .parseOptions` + `parseSelector` — the suggestOptionsNextOrClose
/// install precedes the canRead/']' checks, the ']' exit the
/// SUGGEST_NOTHING putfield wears (the audited 251-255), the
/// parseSelector ']' expectation the close answers and the parse
/// COMPLETES — the successful parse the suggestions none keeps:
/// the fully-typed selector NOTHING shows). The value walks' junk
/// verdicts the ']' stop the close law refines — the ',' + ']' pair
/// the junk lane suggests the honest answer never was — the EMPTY
/// local the silence wears, the TAIL lane and the EARLIER-ELEMENT
/// walk the same law keeps; the junk the ']' never leads the pair
/// lane wears whole (the 'x'-led junk the span to the input end
/// carries — the close the parse consumed AFTER the junk char the
/// fill's own cursor law keeps).
#[test]
fn the_selector_options_the_close_bracket_silence_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE NUMERIC WALK'S ']' STOP THE CLOSE LAW SILENCES (the readInt
    // the stopped read returns, the mark the modifier wears, the ']'
    // exit the SUGGEST_NOTHING putfield answers, the parseSelector
    // ']' expectation the close consumes — the parse completes)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5]", &[], true, &[])
    else {
        panic!("/kill @a[limit=5] the close law answers");
    };
    assert_eq!((start, length), (15, 2), "the value region's own span");
    assert!(matches.is_empty(), "the typed close the silence wears");
    // THE SCORES COMPOUND'S ']' STOP THE SAME LAW WEARS (the
    // post-'}' junk verdict the refined ws law banks, the ']' the
    // close law silences)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1}]", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}}] the close law answers");
    };
    assert_eq!((start, length), (16, 6), "the value region's own span");
    assert!(matches.is_empty(), "the compound's close the silence wears");
    // THE PROVIDER WALKS' ']' STOPS THE SAME SILENCE KEEPS (the
    // gamemode/byName hit and the sort/switch hit the mark wear,
    // the close the throw the state survives never suggests)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[gamemode=creative]", &[], true, &[])
    else {
        panic!("/kill @a[gamemode=creative] the close law answers");
    };
    assert_eq!((start, length), (18, 9), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the provider walk's close the silence wears"
    );
    // THE STRING WALK'S ']' STOP THE SAME LAW WEARS (the unquoted
    // run the ']' the charset refuses — the stopped read RETURNS,
    // the modifier completes, the close the parse consumes)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=Steve]", &[], true, &[])
    else {
        panic!("/kill @a[name=Steve] the close law answers");
    };
    assert_eq!((start, length), (14, 6), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the string walk's close the silence wears"
    );
    // THE QUOTED VALUE'S ']' TAIL THE SAME LAW WEARS (the
    // readStringUntil the close quote consumes, the tail the junk
    // verdict names, the ']' the close law silences)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"Steve x\"]", &[], true, &[])
    else {
        panic!("/kill @a[name=\"Steve x\"] the close law answers");
    };
    assert_eq!((start, length), (14, 10), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the quoted value's close the silence wears"
    );
    // THE SORT WALK'S ']' STOP THE SAME LAW WEARS (the case-switch
    // hit the setOrder+mark wear, the ']' the close answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=nearest]", &[], true, &[])
    else {
        panic!("/kill @a[sort=nearest] the close law answers");
    };
    assert_eq!((start, length), (14, 8), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the sort walk's close the silence wears"
    );
    // THE QUOTED-KEY FORM THE SYNTH THE SAME LAW KEEPS (the
    // readString KEY walk the readString law speaks, the synth the
    // value lane rides, the ']' the close silences)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[\"name\"=Steve]", &[], true, &[])
    else {
        panic!("/kill @a[\"name\"=Steve] the close law answers");
    };
    assert_eq!((start, length), (16, 6), "the synth's value span");
    assert!(matches.is_empty(), "the quoted-key close the silence wears");
    // THE EARLIER ELEMENT'S ']' THE PARSE CONSUMES THE SELECTOR
    // WHOLE (the close the options lane the ',' never reaches —
    // the parse the successful deathless walk keeps — the EMPTY
    // silence the earlier-element law wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5],l", &[], true, &[])
    else {
        panic!("/kill @a[limit=5],l the earlier close law answers");
    };
    assert_eq!((start, length), (9, 8), "the element's own span");
    assert!(matches.is_empty(), "the closed selector the silence wears");
    // THE JUNK THE ']' NEVER LEADS THE PAIR LANE WEARS WHOLE (the
    // 'x' the stopped read names, the throw the survived state
    // carries — the ',' + ']' AT the junk, the span the junk to
    // the input end the fill's own cursor law keeps — the close
    // the parse consumed AFTER the junk char)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5x]", &[], true, &[])
    else {
        panic!("/kill @a[limit=5x] the junk lane answers");
    };
    assert_eq!((start, length), (16, 2), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the 'x'-led junk the pair lane wears"
    );
}

/// THE BRACKET REGION THE TOKENIZER THE PARSE CONSUMES WHOLE
/// (javap `EntitySelectorParser.parseOptions` — the parse's own
/// skipWhitespace the audited loop positions the in-bracket
/// whitespace banks: the '[' entry the loop's head skip rides,
/// the key read the ws the 69-73 positions consume, the value
/// walk the 125-129 positions, the post-modifier walk the
/// 146-150 positions before the canRead/']' checks — the
/// argument tokenization the StringReader's parse never applies
/// inside, the space-split the token law the selector WHOLE
/// keeps). The forms the guard the walk's own lanes delivers:
/// the key/value ws the value walk's own laws speak, the
/// post-value ws the parse's own skip names (the refined junk
/// cursor the pair wears, the ']' the close silence, the ws to
/// the input end the NextOrClose pair wears), the post-close-ws
/// ',' the ARMED segment earns (the fresh-key lane the mark's
/// own row hides), and the two depth guards ride INDEPENDENTLY
/// (the ']' never closes a '{' region — the mismatched pair the
/// compound's own throw answers).
#[test]
fn the_selector_options_the_bracket_region_guard_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE KEY/VALUE WS THE WALK'S OWN AUDITED POSITIONS CONSUME
    // (the key read the 69-73 ws skip rides, the '=' the parse
    // expects, the value the 125-129 positions precede — the
    // selector the guard ONE token speaks whole): the CLOSED form
    // the close law silences (the fully-typed selector nothing
    // shows), the OPEN form the pair the input end wears
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit = 5]", &[], true, &[])
    else {
        panic!("/kill @a[limit = 5] the guard law answers");
    };
    assert_eq!((start, length), (17, 2), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the fully-typed close the silence wears"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit = 5", &[], true, &[])
    else {
        panic!("/kill @a[limit = 5 the open-form law answers");
    };
    assert_eq!(
        (start, length),
        (18, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the key/value ws the parse consumes"
    );
    // THE '[' ENTRY'S OWN WS THE LOOP HEAD SKIPS (the fresh tail
    // the post-skip cursor the value lane speaks — the lead ws
    // the key trim and the walk's own audited positions consume):
    // the CLOSED form the close law silences, the OPEN form the
    // pair the input end wears
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[ limit=5]", &[], true, &[])
    else {
        panic!("/kill @a[ limit=5] the entry ws law answers");
    };
    assert_eq!((start, length), (16, 2), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the fully-typed close the silence wears"
    );
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[ limit=5", &[], true, &[])
    else {
        panic!("/kill @a[ limit=5 the open-form law answers");
    };
    assert_eq!(
        (start, length),
        (17, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the entry ws the loop head skips"
    );
    // THE POST-CLOSE WS THE PARSE'S OWN SKIP NAMES (the compound
    // consumed, the 146-150 skip the ws consumes, the 'x' the
    // parse's own death wears — the ',' + ']' pair AT the junk)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1} x", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}} x the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (22, 1),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the post-close junk the pair wears"
    );
    // THE POST-CLOSE WS TO THE INPUT END THE NEXTORCLOSE STATE
    // WEARS (the skip the ws consumes, the canRead FALSE exit the
    // parse returns — the ',' + ']' pair at the input end)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1} ", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}} + ws the NextOrClose law answers");
    };
    assert_eq!(
        (start, length),
        (22, 0),
        "the input end the completion wears"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the post-close ws the pair at the input end wears"
    );
    // THE POST-CLOSE WS ']' THE CLOSE LAW SILENCES (the skip the
    // ws consumes, the ']' the parseSelector expectation consumes
    // — the parse completes, the fully-typed selector nothing
    // shows)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1} ]", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}} ] the close law answers");
    };
    assert_eq!((start, length), (16, 7), "the value region's own span");
    assert!(matches.is_empty(), "the past-ws close the silence wears");
    // THE POST-CLOSE WS ',' THE ARMED SEGMENT EARNS (the skip the
    // ws consumes, the ',' the parse consumes, the element the
    // parse marked — the fresh-key lane the empty tail the 20
    // rows the SetOnce law wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1} ,", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}} , the fresh-key lane answers");
    };
    assert_eq!((start, length), (23, 0), "the fresh tail's own span");
    assert_eq!(matches.len(), 20, "the parsed scores the own row hides");
    // THE WS-LED CONTINUATION THE FRESH LANE SPEAKS (the 'l' tail
    // the level= + limit= rows the gates keep — the mark the
    // scores row alone hides)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1} ,l", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}} ,l the fresh-key lane answers");
    };
    assert_eq!((start, length), (23, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["level=", "limit="],
        "the ws-led ',' the fresh-key lane wears"
    );
    // THE '@s' LAW THE SAME LANE WEARS (the isCurrentEntity gate
    // the limit row hides — the level= row alone)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @s[scores={a=1} ,l", &[], true, &[])
    else {
        panic!("/kill @s[scores={{a=1}} ,l the '@s' law answers");
    };
    assert_eq!((start, length), (23, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["level="],
        "the '@s' rows the two gates hide"
    );
    // THE MARK THE WS-LED 's' TAIL WEARS (the parsed scores the
    // own row hides — the sort= row alone)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1} ,s", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1}} ,s the mark law answers");
    };
    assert_eq!((start, length), (23, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["sort="],
        "the ws-led close the mark wears"
    );
    // THE EARLIER ELEMENT'S POST-VALUE WS THE REFINED JUNK WEARS
    // (the earned ',' the ARMED segment earns, the stopped read's
    // cursor the post-value skip the ws consumes, the 'x' the
    // parse's own death wears — the ',' + ']' pair AT the junk)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5 x,l", &[], true, &[])
    else {
        panic!("/kill @a[limit=5 x,l the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (17, 3),
        "the span the junk cursor to the input end keeps"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the stopped read's ws the parse skip consumes"
    );
    // THE CONTINUATION'S OWN WS THE LOOP HEAD SKIPS (the fresh
    // tail's lead ws the post-skip cursor the rows fold — the
    // 'name' partial the name= row alone)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5, name", &[], true, &[])
    else {
        panic!("/kill @a[limit=5, name the head-skip law answers");
    };
    assert_eq!(
        (start, length),
        (18, 4),
        "the post-skip cursor the span names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["name="],
        "the ws-led fresh tail the prefix fold filters"
    );
    // THE WS-ONLY TAIL THE SAME ROWS WEARS (the head skip the ws
    // consumes, the fresh-key lane the empty partial the 20 rows
    // the SetOnce limit mark wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5, ", &[], true, &[])
    else {
        panic!("/kill @a[limit=5, + ws the fresh-key lane answers");
    };
    assert_eq!(
        (start, length),
        (18, 0),
        "the post-skip cursor the span names"
    );
    assert_eq!(matches.len(), 20, "the parsed limit the own row hides");
    // THE QUOTED VALUE'S ',' THE ARMED SEGMENT EARNS (the
    // readString walk the quoted value consumes whole, the ws the
    // post-value skip eats, the fresh 's' tail the rows the
    // InvertableSetOptionState mark keeps — the name row the
    // 's' tail never matches)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[name=\"Steve x\" ,s", &[], true, &[])
    else {
        panic!("/kill @a[name=\"Steve x\" ,s the quoted continuation answers");
    };
    assert_eq!((start, length), (25, 1), "the fresh tail's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["scores=", "sort="],
        "the quoted element the parse consumed, the fresh rows keep"
    );
    // THE ']' NEVER CLOSES A '{' REGION (the two depth guards ride
    // INDEPENDENTLY — the mismatched pair the compound's own '='
    // expectation throw answers, the space the brace guard the
    // split never spends — the EMPTY silence the walk's own death
    // keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores={a=1] x", &[], true, &[])
    else {
        panic!("/kill @a[scores={{a=1] x the independence law answers");
    };
    assert_eq!((start, length), (16, 7), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the mismatched pair the walk's death silences"
    );
    // THE OTHER KEYS' INTERNAL-WS FORMS THE WALKS SPEAK (the
    // retired ask the walk's own verdict answers — the stopped
    // read the refinement the refined cursor wears, the pair the
    // UNGATED state at the junk keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5 x", &[], true, &[])
    else {
        panic!("/kill @a[limit=5 x the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (17, 1),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the internal-ws junk the pair wears"
    );
}

/// THE POST-VALUE WS THE WALKS SPEAK (the retired honest-ask
/// gate — the internal-ws forms the tail the blanket AskServer
/// the pre-refinement rounds wore retires): the value walks'
/// charsets STOP at the ws char the read refuses — the stopped
/// read the walk RETURNS (the Junk verdict the throw never
/// spends on a ws stop), the parse's OWN post-modifier
/// skipWhitespace (the audited 146-150) the cursor the ws past
/// the skip refines, the suggestOptionsNextOrClose install (the
/// audited 153-160) the refined cursor wears, and the loop's
/// re-entry the '=' expectation throw answers — the state the
/// throw SURVIVES (the install precedes the next body's own
/// SUGGEST_NOTHING spend) — the UNGATED ',' + ']' pair AT the
/// refined cursor every lane the walk speaks wears (the same
/// law the earlier-element walk, the post-quote refinement and
/// the post-'}' refinement bank); the WALK'S OWN DEATH the
/// throw the walk spends BEFORE the ws (the too-small limit,
/// the unknown switch value) the SUGGEST_NOTHING silence (or
/// the provider's own fold) keeps, and the UNSPOKEN walks (the
/// nbt/advancements/predicate compounds) the honest ask the
/// None arm keeps.
#[test]
fn the_selector_options_the_post_value_ws_walks_speak() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    let types = vec![
        "minecraft:pig".to_string(),
        "minecraft:zombie".to_string(),
        "minecraft:skeleton_horse".to_string(),
    ];
    // THE GAMEMODE LANE (the byName hit the stopped read the ws
    // the byName charset refuses wears — the mark the hit path
    // keeps, the pair the refined cursor the 'x' wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[gamemode=creative x", &[], true, &[])
    else {
        panic!("/kill @a[gamemode=creative x the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (27, 1),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the gamemode ws junk the pair wears"
    );
    // THE TYPE LANE (the id run the membership keeps — the same
    // refinement the 'x' the pair wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=pig x", &[], true, &types)
    else {
        panic!("/kill @a[type=pig x the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (18, 1),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the type ws junk the pair wears"
    );
    // THE TEAM LANE (the readUnquotedString the stopped read the
    // ws wears — the state gate the read AFTER the walk rides,
    // the mark the hit keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[team=red x", &[], true, &[])
    else {
        panic!("/kill @a[team=red x the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (18, 1),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the team ws junk the pair wears"
    );
    // THE TAG LANE (the plainest walk — the ws the charset
    // refuses, the pair the refined cursor AT the input end the
    // length-0 span wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[tag=a b", &[], true, &[])
    else {
        panic!("/kill @a[tag=a b the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (15, 1),
        "the input end the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the tag ws junk the pair wears"
    );
    // THE COORDINATE LANE (the readDouble the stopped read the ws
    // wears — the pair the refined cursor the '2' wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[x=1 2", &[], true, &[])
    else {
        panic!("/kill @a[x=1 2 the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (13, 1),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the coordinate ws junk the pair wears"
    );
    // THE DISTANCE LANE (the bounds walk the single-value branch
    // the junk the stop position names — the closed walk the
    // square form carries, the junk the walk's own return the
    // post-value lane wears — the pair the refined cursor the
    // '6' wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[distance=5 6", &[], true, &[])
    else {
        panic!("/kill @a[distance=5 6 the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (20, 1),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the distance ws junk the pair wears"
    );
    // THE ROTATION LANE (the OPEN walk the isPresent pair never
    // carries — the same single-value junk the walk's own return
    // wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[x_rotation=10 20", &[], true, &[])
    else {
        panic!("/kill @a[x_rotation=10 20 the refined junk answers");
    };
    assert_eq!(
        (start, length),
        (23, 2),
        "the span the refined cursor names"
    );
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the rotation ws junk the pair wears"
    );
    // THE WALK'S OWN DEATH THE WS NEVER REACHES (the too-small
    // limit the ERROR_LIMIT_TOO_SMALL throw the walk spends on
    // the value the stopped read returns — the ws after it the
    // throw never sees — the SUGGEST_NOTHING silence the EMPTY
    // local wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=0 x", &[], true, &[])
    else {
        panic!("/kill @a[limit=0 x the too-small silence answers");
    };
    assert_eq!((start, length), (15, 3), "the value region's own span");
    assert!(matches.is_empty(), "the too-small limit the silence keeps");
    // THE SORT SWITCH'S OWN THROW (the case-sensitive switch the
    // unknown value the rollbackAndThrow answers — the provider
    // the throw survives, the fold the remaining 'nea x' the
    // splitter law never matches — the EMPTY local the silence
    // wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[sort=nea x", &[], true, &[])
    else {
        panic!("/kill @a[sort=nea x the switch silence answers");
    };
    assert_eq!((start, length), (14, 5), "the value region's own span");
    assert!(
        matches.is_empty(),
        "the unknown sort value the silence keeps"
    );
    // THE UNSPOKEN WALKS THE HONEST ASK KEEPS (the
    // nbt/advancements compounds the None arm answers — the ws
    // the forms carry the lane never speaks)
    assert_eq!(
        local_command_suggestions(&tree, "/kill @a[nbt={a:1b} x", &[], true, &[]),
        Src::AskServer,
        "the nbt compound the honest ask keeps"
    );
    assert_eq!(
        local_command_suggestions(&tree, "/kill @a[advancements={a=true} x", &[], true, &[]),
        Src::AskServer,
        "the advancements compound the honest ask keeps"
    );
}

/// THE NUMERIC VALUE WALKS (javap `EntitySelectorOptions
/// lambda$bootStrap$3/$5/$7-17/$19/$21/$23` + `MinMaxBounds$Bounds
/// .fromReader` — the distance/level/coordinate/rotation/limit
/// modifiers install NO suggestions provider, SUGGEST_NOTHING the
/// value position keeps, and the parse's own number grammar
/// answers the completion: a fully-consumed valid value the
/// post-value ',' + ']' punctuation wears, a failed/stopped parse
/// the EMPTY local keeps (no round-trip the throw spends). The
/// LIMIT walk the readInt law keeps (`"5"` the parse completes,
/// the `< 1` values the ERROR_LIMIT_TOO_SMALL throw the audited
/// $23 answers — `"-5"` the silence wears —, `"5.5"` the
/// INVALID-INT throw answers, `"5x"` the
/// ']' expectation dies at, the overflow the parse's own range
/// law wears); the COORDINATE walks the readDouble law (the
/// trailing-dot `"5."` the parse completes — the charset law the
/// '.' carries has NO peek-ahead); the distance/level walks the
/// Bounds.fromReader grammar (the '.'-law the
/// isAllowedInputChar walk speaks: a `.` consumed iff the next
/// char is `>` `.` — the `..` separator the run never swallows,
/// the trailing `.` the run stops before and the vanilla's own
/// end-of-input StringIndexOutOfBounds the suggestions future
/// swallows — the EMPTY local the same silence answers) + the
/// isPresent PAIR (either range side absent → the
/// rollbackAndThrow) + the areSwapped check (min > max → the
/// cursor reset + rethrow); the ROTATION walks the same grammar
/// WITHOUT the pair (the `FloatDegrees` wrapper — the empty side
/// and the swapped bounds the walk accepts, `"5.."` the open
/// range completes); the '!' the numeric walks never invert (the
/// read walks carry no shouldInvertValue — `limit=!5` the
/// EXPECTED-NUMBER throw answers); the scores/nbt/advancements/
/// predicate keys the honest ask keeps).
#[test]
fn the_selector_options_the_numeric_value_lanes_speak() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE LIMIT WALK (the readInt law — the TYPED value the
    // parse completes, the punctuation the post-value state
    // suggests, the span the parse cursor's own end names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5", &[], true, &[])
    else {
        panic!("/kill @a[limit=5 the readInt walk answers");
    };
    assert_eq!((start, length), (16, 0), "the consumed value's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the int consumed the punctuation wears"
    );
    // THE NEGATIVE INT THE LIMIT-TOO-SMALL THROW ANSWERS (the
    // audited $23 — the `< 1` check the rollbackAndThrow carries
    // BEFORE setMaxResults, the '-' the number charset parses —
    // the SUGGEST_NOTHING silence the throw keeps)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[limit=-5", &[], true, &[])
    else {
        panic!("/kill @a[limit=-5 the readInt walk answers");
    };
    assert!(
        matches.is_empty(),
        "the negative int the TOO-SMALL throw silences"
    );
    // THE INVALID INT FORMS (the "5.5" the parseInt throw
    // answers, the overflow the parse's own range law wears, the
    // bare "-" the INVALID-INT throw answers, the '+' the charset
    // refuses — the walk's own throws the SUGGEST_NOTHING silence
    // keeps)
    for (input, why) in [
        (
            "/kill @a[limit=5.5",
            "the fraction the parseInt throw answers",
        ),
        (
            "/kill @a[limit=99999999999",
            "the overflow the parse's range law wears",
        ),
        (
            "/kill @a[limit=-",
            "the bare sign the INVALID-INT throw answers",
        ),
        ("/kill @a[limit=+5", "the '+' the number charset refuses"),
    ] {
        let Src::Local { matches, .. } = local_command_suggestions(&tree, input, &[], true, &[])
        else {
            panic!("{input} the numeric refusal answers");
        };
        assert!(matches.is_empty(), "{why}");
    }
    // THE LEFTOVER CHAR THE JUNK LANE WEARS (the "5x" the run the
    // charset stops inside AND the parse accepts — the walk
    // RETURNS, the parse's own ERROR_EXPECTED_END_OF_OPTIONS
    // throw the cursor the stop position names, the
    // suggestOptionsNextOrClose state the throw survives — the
    // UNGATED pair the ',' + ']' AT the junk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[limit=5x", &[], true, &[])
    else {
        panic!("/kill @a[limit=5x the junk lane answers");
    };
    assert_eq!((start, length), (16, 1), "the span the junk cursor names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the leftover char the junk lane wears"
    );
    // THE COORDINATE WALKS (the readDouble law — the
    // x/y/z/dx/dy/dz keys the same completion wear; the
    // trailing-dot form the charset law ACCEPTS — the
    // parseDouble walk the "5." completes)
    for key in ["x", "y", "z", "dx", "dy", "dz"] {
        let Src::Local { matches, .. } =
            local_command_suggestions(&tree, &format!("/kill @a[{key}=5."), &[], true, &[])
        else {
            panic!("/kill @a[{key}=5. the readDouble walk answers");
        };
        assert_eq!(
            matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
            vec![",", "]"],
            "the trailing-dot double the completion wears"
        );
        let Src::Local { matches, .. } =
            local_command_suggestions(&tree, &format!("/kill @a[{key}=-.5"), &[], true, &[])
        else {
            panic!("/kill @a[{key}=-.5 the readDouble walk answers");
        };
        assert_eq!(
            matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
            vec![",", "]"],
            "the negative fraction the completion wears"
        );
    }
    // THE INVALID DOUBLE FORMS (the "5..10" the parseDouble
    // throw answers — the coords carry NO range grammar)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[x=5..10", &[], true, &[])
    else {
        panic!("/kill @a[x=5..10 the readDouble refusal answers");
    };
    assert!(matches.is_empty(), "the range tail the coord walk refuses");
    // THE DISTANCE WALK (the Bounds.fromReader grammar — the
    // single value the (min, min) pair builds, the range the
    // both-present law keeps, the swap the areSwapped check
    // refuses)
    for (input, why) in [
        (
            "/kill @a[distance=5",
            "the single value the (min, min) pair builds",
        ),
        (
            "/kill @a[distance=5..10",
            "the range the both-present law keeps",
        ),
        (
            "/kill @a[distance=5.5..10.5",
            "the dotted sides the '.'-law run consumes",
        ),
        (
            "/kill @a[distance=5..5",
            "the equal bounds the swap law keeps",
        ),
    ] {
        let Src::Local { matches, .. } = local_command_suggestions(&tree, input, &[], true, &[])
        else {
            panic!("{input} the Bounds walk answers");
        };
        assert_eq!(
            matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
            vec![",", "]"],
            "{why}"
        );
    }
    // THE INVALID BOUNDS FORMS (the open range the isPresent
    // pair refuses — the closed walk the rollbackAndThrow
    // answers; the swapped bounds the cursor reset + rethrow
    // wears; the trailing dot the run stops before — the
    // vanilla's own end-of-input crash the future swallows)
    for (input, why) in [
        (
            "/kill @a[distance=5..",
            "the open max the isPresent pair refuses",
        ),
        (
            "/kill @a[distance=..5",
            "the open min the isPresent pair refuses",
        ),
        (
            "/kill @a[distance=10..5",
            "the swapped bounds the areSwapped check refuses",
        ),
        (
            "/kill @a[distance=..",
            "the both-empty form the ERROR-EMPTY throw answers",
        ),
        (
            "/kill @a[distance=abc",
            "the unparseable form the both-empty throw answers",
        ),
        (
            "/kill @a[distance=5..x",
            "the unparseable max the isPresent pair refuses",
        ),
        (
            "/kill @a[distance=5.",
            "the trailing dot the run stops before — the crash lane the silence keeps",
        ),
        (
            "/kill @a[level=5.5",
            "the fraction the parseInt throw answers",
        ),
        (
            "/kill @a[level=5..",
            "the open max the level walk's pair refuses",
        ),
    ] {
        let Src::Local { matches, .. } = local_command_suggestions(&tree, input, &[], true, &[])
        else {
            panic!("{input} the Bounds refusal answers");
        };
        assert!(matches.is_empty(), "{why}");
    }
    // THE ROTATION WALKS (the FloatDegrees wrapper — NO
    // isPresent pair the walk carries: the open range and the
    // swapped bounds the parse accepts, the empty-min form
    // completes)
    for key in ["x_rotation", "y_rotation"] {
        for (input, why) in [
            (format!("/kill @a[{key}=45"), "the single value completes"),
            (
                format!("/kill @a[{key}=45..90"),
                "the range the no-pair law keeps",
            ),
            (
                format!("/kill @a[{key}=45.."),
                "the open max the no-pair law accepts",
            ),
            (
                format!("/kill @a[{key}=..90"),
                "the open min the no-pair law accepts",
            ),
            (
                format!("/kill @a[{key}=90..45"),
                "the swapped bounds the no-areSwapped law accepts",
            ),
        ] {
            let Src::Local { matches, .. } =
                local_command_suggestions(&tree, &input, &[], true, &[])
            else {
                panic!("{input} the rotation walk answers");
            };
            assert_eq!(
                matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
                vec![",", "]"],
                "{why}"
            );
        }
        // the both-empty form the shared both-empty check refuses
        let Src::Local { matches, .. } =
            local_command_suggestions(&tree, &format!("/kill @a[{key}=.."), &[], true, &[])
        else {
            panic!("/kill @a[{key}=.. the both-empty refusal answers");
        };
        assert!(matches.is_empty(), "the empty range the throw keeps");
        // the leftover char the junk lane wears (the open lane's
        // no-pair law the absent max accepts — the walk RETURNS,
        // the ERROR_EXPECTED_END_OF_OPTIONS throw the cursor the
        // 'x' position names, the ',' + ']' the state wears)
        let Src::Local {
            start,
            length,
            matches,
        } = local_command_suggestions(&tree, &format!("/kill @a[{key}=45..x"), &[], true, &[])
        else {
            panic!("/kill @a[{key}=45..x the junk lane answers");
        };
        assert_eq!((start, length), (24, 1), "the span the junk cursor wears");
        assert_eq!(
            matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
            vec![",", "]"],
            "the unparseable max the junk lane wears"
        );
    }
    // THE NO-INVERT LAW (the numeric walks carry NO
    // shouldInvertValue — the '!' the read walks refuse)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[limit=!5", &[], true, &[])
    else {
        panic!("/kill @a[limit=!5 the no-invert refusal answers");
    };
    assert!(matches.is_empty(), "the '!' the read walk refuses");
    // THE EMPTY SCORES VALUE THE EXPECT-'{' THROW ANSWERS (the
    // audited $46 — the compound walk the empty value the same
    // throw silences — the flip the spoken walk wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[scores=", &[], true, &[])
    else {
        panic!("/kill @a[scores= the expect-'{{' law answers");
    };
    assert_eq!(
        (start, length),
        (16, 0),
        "the empty value region's own span"
    );
    assert!(
        matches.is_empty(),
        "the empty compound value the throw silences"
    );
    assert_eq!(
        local_command_suggestions(&tree, "/kill @a[nbt=", &[], true, &[]),
        Src::AskServer,
        "the nbt compound the honest ask keeps"
    );
    // THE TAB APPLY (the consumed value the ',' zero-length
    // insert lands — the span the parse cursor's end wears, no
    // request the lane spends)
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/kill @a[limit=5".chars() {
        app.on_chat_char(c);
    }
    assert!(
        _rx.try_recv().is_err(),
        "the numeric lane no request spends"
    );
    app.on_chat_tab_key();
    assert_eq!(
        app.chat_input, "/kill @a[limit=5,",
        "the apply the ',' insert lands"
    );
}

/// THE TYPE VALUE WALK (javap `EntitySelectorOptions
/// lambda$bootStrap$35/36` + `SharedSuggestionProvider
/// .suggestResource` — the two arms the fresh NONE state fires:
/// the negative the '!' prefix rides (the commonPrefix gate the
/// non-'!' remaining silences), the positive the bare ids wear
/// (the filterResources walk — the ':' the remaining carries the
/// FULL id match keeps, the bare remaining the NAMESPACE-OR-PATH
/// match answers), the tag arms the house's empty tag table never
/// fires; the '#' element the parse never refutes (the
/// canParseTag set-membership law — any identifier lands), the
/// id element the registry membership keeps (the defaulted id
/// the no-colon form wears), the invalid-shape id the parse's
/// own throw answers (the rollback the case-folded remaining
/// rides — 'PIG' the throw spends, the suggestions the lowercase
/// fold speaks); the merge sort the compareToIgnoreCase law
/// orders the '!' forms first).
#[test]
fn the_selector_options_the_type_lane_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    let types = vec![
        "minecraft:pig".to_string(),
        "minecraft:zombie".to_string(),
        "minecraft:skeleton_horse".to_string(),
    ];
    // THE EMPTY ELEMENT (the isEmpty forEach arm — ALL resources
    // under BOTH prefixes, the merge sort the '!' forms first)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=", &[], true, &types)
    else {
        panic!("/kill @a[type= the both-arms answer");
    };
    assert_eq!((start, length), (14, 0), "the after-'=' element span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![
            "!minecraft:pig",
            "!minecraft:skeleton_horse",
            "!minecraft:zombie",
            "minecraft:pig",
            "minecraft:skeleton_horse",
            "minecraft:zombie",
        ],
        "the both arms the forEach law answers"
    );
    assert!(
        matches.iter().all(|m| m.tooltip.is_none()),
        "the plain suggest no tooltip wears"
    );
    // THE PATH PREFIX (the filterResources walk — the bare
    // remaining the namespace-OR-path match answers, 'z' the
    // zombie's path names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=z", &[], true, &types)
    else {
        panic!("/kill @a[type=z the path match answers");
    };
    assert_eq!((start, length), (14, 1), "the typed element's span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["minecraft:zombie"],
        "the 'z' prefix the zombie answers"
    );
    // THE NAMESPACE PREFIX (the 'min' the minecraft namespace
    // matches — EVERY id the arm answers, the vanilla's own
    // namespace law)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[type=min", &[], true, &types)
    else {
        panic!("/kill @a[type=min the namespace match answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![
            "minecraft:pig",
            "minecraft:skeleton_horse",
            "minecraft:zombie",
        ],
        "the namespace prefix every id answers"
    );
    // THE FULL-ID LANE (the ':' the remaining carries — the FULL
    // id match only, the namespace/probe the splitter walk keeps)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[type=minecraft:z", &[], true, &types)
    else {
        panic!("/kill @a[type=minecraft:z the full-id match answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["minecraft:zombie"],
        "the ':' remaining the full-id law keeps"
    );
    // THE NEGATIVE ARM (the '!' the commonPrefix gate keeps — the
    // bare '!' the forEach arm answers, the '!' the arm alone wears)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=!", &[], true, &types)
    else {
        panic!("/kill @a[type=! the negative arm answers");
    };
    assert_eq!((start, length), (14, 1), "the '!' element's own span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![
            "!minecraft:pig",
            "!minecraft:skeleton_horse",
            "!minecraft:zombie",
        ],
        "the bare '!' the negative arm alone answers"
    );
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[type=!z", &[], true, &types)
    else {
        panic!("/kill @a[type=!z the negative fold answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["!minecraft:zombie"],
        "the '!z' fold the negative zombie answers"
    );
    // THE CASE FOLD (the suggestResource walk the remaining
    // lowercases FIRST — 'ZO' the zombie answers)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[type=ZO", &[], true, &types)
    else {
        panic!("/kill @a[type=ZO the case fold answers");
    };
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["minecraft:zombie"],
        "the lowercase fold both sides wear"
    );
    // THE INVALID-SHAPE ID (the parse's own throw the rollback
    // keeps — 'PIG' the Identifier validation refuses, the
    // case-folded remaining the walk still answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=PIG", &[], true, &types)
    else {
        panic!("/kill @a[type=PIG the rolled-back walk answers");
    };
    assert_eq!((start, length), (14, 3), "the element span the throw wears");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["minecraft:pig"],
        "the case-folded remaining the suggestions speak"
    );
    // THE UNKNOWN ID (the getOptional orElseThrow the rollback
    // keeps — the walk the empty merge answers)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[type=zz", &[], true, &types)
    else {
        panic!("/kill @a[type=zz the empty lane answers");
    };
    assert!(matches.is_empty(), "the unknown id the silence keeps");
    // THE COLD START (the registry never shipped — the empty table
    // the honest empty list keeps)
    let Src::Local { matches, .. } =
        local_command_suggestions(&tree, "/kill @a[type=z", &[], true, &[])
    else {
        panic!("/kill @a[type=z the cold start answers");
    };
    assert!(matches.is_empty(), "the empty roster the empty list wears");
    // THE VALID COMPLETE ID (the post-value state the ',' + ']'
    // punctuation suggests — the parse the membership lands)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=pig", &[], true, &types)
    else {
        panic!("/kill @a[type=pig the post-value state answers");
    };
    assert_eq!((start, length), (17, 0), "the parse cursor's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the punctuation the successful value wears"
    );
    // THE INVERTED COMPLETE ID (the '!' the shouldInvertValue
    // consumes — the membership the negative element keeps)
    let (start, matches) = if let Src::Local { start, matches, .. } =
        local_command_suggestions(&tree, "/kill @a[type=!pig", &[], true, &types)
    {
        (start, matches)
    } else {
        panic!("/kill @a[type=!pig the post-value state answers");
    };
    assert_eq!(start, 18, "the consumed '!' the parse end names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the inverted element the punctuation wears"
    );
    // THE TAG ELEMENT (the '#' the isTag walk spends — the parse
    // the canParseTag set-membership never refutes, the tag id
    // the registry's tag table the parse never asks)
    let (start, matches) = if let Src::Local { start, matches, .. } =
        local_command_suggestions(&tree, "/kill @a[type=#skeletons", &[], true, &types)
    {
        (start, matches)
    } else {
        panic!("/kill @a[type=#skeletons the post-value state answers");
    };
    assert_eq!(start, 24, "the parsed tag the parse end names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the tag element the punctuation wears"
    );
    // THE BARE '#' (the tag id the read throws on — the empty id
    // the identifier validation refuses; the tag arms the empty
    // table never fire, the walk the silence keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions(&tree, "/kill @a[type=#", &[], true, &types)
    else {
        panic!("/kill @a[type=# the empty lane answers");
    };
    assert_eq!((start, length), (14, 1), "the '#' the element span wears");
    assert!(matches.is_empty(), "the bare '#' the silence keeps");
    // THE TAB APPLY (the selected id the replace lands — the full
    // 'minecraft:'-prefixed text the suggestion carries, the
    // roster the App's field banks)
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    app.live_entity_types_updated(vec![
        "minecraft:pig".to_string(),
        "minecraft:zombie".to_string(),
    ]);
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/kill @a[type=zo".chars() {
        app.on_chat_char(c);
    }
    assert!(_rx.try_recv().is_err(), "the type lane no request spends");
    app.on_chat_tab_key();
    assert_eq!(
        app.chat_input, "/kill @a[type=minecraft:zombie",
        "the apply the full id lands"
    );
    // THE JOIN'S OWN SHIP (the roster the milestone lands — the
    // Loaded variant's table the App's field banks, the chat's
    // type= walk it answers from now on)
    let mut app2 = App::new(3);
    app2.open(ScreenKind::Servers);
    app2.selected_server = 1;
    app2.join_selected_server();
    let id = app2.login_requests[0].id;
    app2.login_finished(
        id,
        Ok(LoginResult::Loaded {
            username: crate::screens::PLAYER_NAME.to_string(),
            entity_id: 1,
            view_distance: 8,
            chunks: 1,
            world: air_net_world(2, 0),
            camera: None,
            spawn_biome: None,
            slim_arm: false,
            skybox: Skybox::Overworld,
            fog: None,
            cloud_fog_end: None,
            entity_types: vec!["minecraft:zombie".to_string()],
            entity_type_tags: vec![(
                "minecraft:undead".to_string(),
                vec!["minecraft:zombie".to_string()],
            )],
        }),
    );
    assert_eq!(
        app2.entity_types,
        vec!["minecraft:zombie".to_string()],
        "the join milestone the roster ships"
    );
    assert_eq!(
        app2.entity_type_tags,
        vec![(
            "minecraft:undead".to_string(),
            vec!["minecraft:zombie".to_string()],
        )],
        "the join milestone the tag table ships"
    );
}

#[test]
fn the_selector_options_the_type_tag_lane_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    let types = vec![
        "minecraft:pig".to_string(),
        "minecraft:zombie".to_string(),
        "minecraft:skeleton_horse".to_string(),
    ];
    // THE SYNCED TAG TABLE (the Update Tags packet's entity_type
    // payload the join resolved — the tag name -> member NAMES
    // pairs the '#'-suggestion arms the bootStrap$36 walk banks)
    let tags = vec![
        (
            "minecraft:skeletons".to_string(),
            vec![
                "minecraft:skeleton".to_string(),
                "minecraft:stray".to_string(),
            ],
        ),
        (
            "minecraft:undead".to_string(),
            vec![
                "minecraft:zombie".to_string(),
                "minecraft:skeleton_horse".to_string(),
            ],
        ),
    ];
    // THE EMPTY ELEMENT (the forEach arm ALL four groups answers —
    // the bytecode's own order '!' ids / bare ids / '#' tags /
    // '!#' tags the merge sort the final order wears: '!#' the
    // '!'-then-'#' texts first, the bare ids last)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions_with_tags(&tree, "/kill @a[type=", &[], true, &types, &tags)
    else {
        panic!("/kill @a[type= the four-group answer");
    };
    assert_eq!((start, length), (14, 0), "the after-'=' element span");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![
            "!#minecraft:skeletons",
            "!#minecraft:undead",
            "!minecraft:pig",
            "!minecraft:skeleton_horse",
            "!minecraft:zombie",
            "#minecraft:skeletons",
            "#minecraft:undead",
            "minecraft:pig",
            "minecraft:skeleton_horse",
            "minecraft:zombie",
        ],
        "the both id arms + both tag arms the forEach law answers"
    );
    assert!(
        matches.iter().all(|m| m.tooltip.is_none()),
        "the plain suggest no tooltip wears"
    );
    // THE BARE '#' (the commonPrefix('#', '#') gate opens — the
    // EMPTY rest the forEach arm answers, EVERY tag the '#' text
    // wears; the id arms the '#' remaining never matches, the
    // '!#' arm the commonPrefix('#','!#') emptiness keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions_with_tags(&tree, "/kill @a[type=#", &[], true, &types, &tags)
    else {
        panic!("/kill @a[type=# the tag arms answer");
    };
    assert_eq!((start, length), (14, 1), "the '#' the element span wears");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["#minecraft:skeletons", "#minecraft:undead"],
        "the bare '#' every tag the '#' arm answers"
    );
    // THE BARE '!#' (the two-char prefix the commonPrefix keeps —
    // the '#'-arm the silence wears, the '!#' arm EVERY tag)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions_with_tags(&tree, "/kill @a[type=!#", &[], true, &types, &tags)
    else {
        panic!("/kill @a[type=!# the inverted tag arm answers");
    };
    assert_eq!((start, length), (14, 2), "the '!#' the element span wears");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["!#minecraft:skeletons", "!#minecraft:undead"],
        "the bare '!#' every tag the inverted arm answers"
    );
    // THE TYPED VALID TAIL (the tag lane the Identifier.read
    // answers — 's' the lexically-valid id the parse COMPLETES,
    // the tag's EXISTENCE the parse never asks, the post-value
    // state the punctuation wears — the filtered tag arms the
    // parse's own THROW only answers, see the invalid tail below)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions_with_tags(&tree, "/kill @a[type=#s", &[], true, &types, &tags)
    else {
        panic!("/kill @a[type=#s the post-value state answers");
    };
    assert_eq!(start, 16, "the parsed tag the parse end names");
    assert_eq!(length, 0, "the parse cursor's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the valid tag tail the punctuation wears"
    );
    // THE FILTERED '#' (the parse's own throw the rollback keeps —
    // 'SK' the Identifier validation refuses (the uppercase char
    // the path charset never allows), the rolled-back walk the
    // suggestResource answers: the remaining lowercases FIRST,
    // the 'sk' tail the skeletons' path names, the undead's
    // silence the no-match keeps)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions_with_tags(&tree, "/kill @a[type=#SK", &[], true, &types, &tags)
    else {
        panic!("/kill @a[type=#SK the rolled-back walk answers");
    };
    assert_eq!((start, length), (14, 3), "the element span the throw wears");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["#minecraft:skeletons"],
        "the folded tail the skeletons tag answers"
    );
    // THE FILTERED '!#' (the same throw law the inverted arm
    // wears — 'U' the read refuses, the '!#' arm's own tail the
    // undead's path names)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions_with_tags(&tree, "/kill @a[type=!#U", &[], true, &types, &tags)
    else {
        panic!("/kill @a[type=!#U the rolled-back walk answers");
    };
    assert_eq!((start, length), (14, 3), "the '!#U' span the throw wears");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec!["!#minecraft:undead"],
        "the '!#u' tail the undead tag answers"
    );
    // THE NAMESPACE PREFIX (the parse 'min' the valid id
    // completes — the post-value state the punctuation wears,
    // the namespace-prefix suggestions the THROW lane's own
    // answer stay, see the 'SK' rollback above)
    let Src::Local {
        start,
        length: _,
        matches,
    } = local_command_suggestions_with_tags(&tree, "/kill @a[type=#min", &[], true, &types, &tags)
    else {
        panic!("/kill @a[type=#min the post-value state answers");
    };
    assert_eq!(start, 18, "the parsed tag the parse end names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the valid tag tail the punctuation wears"
    );
    // THE UNKNOWN TAG (the tag's EXISTENCE the parse never asks —
    // 'zz' the lexically-valid id the parse completes, the
    // post-value state the punctuation wears, the never-parsed
    // tag the never-refuted law keeps)
    let Src::Local {
        start,
        length: _,
        matches,
    } = local_command_suggestions_with_tags(&tree, "/kill @a[type=#zz", &[], true, &types, &tags)
    else {
        panic!("/kill @a[type=#zz the post-value state answers");
    };
    assert_eq!(start, 17, "the parsed tag the parse end names");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the unknown tag the parse still completes"
    );
    // THE COLD START (the tag table the snapshot never shipped —
    // the empty vec the '#' arms the silence keeps, the ID arms
    // the roster still speaks)
    let Src::Local { matches, .. } =
        local_command_suggestions_with_tags(&tree, "/kill @a[type=#", &[], true, &types, &[])
    else {
        panic!("/kill @a[type=# the cold start answers");
    };
    assert!(matches.is_empty(), "the empty table the '#' arms silent");
    // THE COMPLETE TAG ELEMENT (the parse the canParseTag law
    // never refutes — the tag's EXISTENCE the parse never asks,
    // the non-empty table the punctuation still answers)
    let Src::Local {
        start,
        length,
        matches,
    } = local_command_suggestions_with_tags(
        &tree,
        "/kill @a[type=#skeletons",
        &[],
        true,
        &types,
        &tags,
    )
    else {
        panic!("/kill @a[type=#skeletons the post-value state answers");
    };
    assert_eq!(start, 24, "the parsed tag the parse end names");
    assert_eq!(length, 0, "the parse cursor's own end");
    assert_eq!(
        matches.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
        vec![",", "]"],
        "the tag element the punctuation wears"
    );
    // THE TAB APPLY (the selected tag the replace lands — the full
    // '#minecraft:'-prefixed text the suggestion carries, the tag
    // table the App's field banks, the refresh's own pass the
    // '#' lane answers, no request spends)
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    app.live_entity_types_updated(types.clone());
    app.live_entity_type_tags_updated(tags.clone());
    assert_eq!(app.entity_type_tags, tags, "the one-shot the table banks");
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/kill @a[type=#".chars() {
        app.on_chat_char(c);
    }
    assert!(_rx.try_recv().is_err(), "the '#' lane no request spends");
    app.on_chat_tab_key();
    assert_eq!(
        app.chat_input, "/kill @a[type=#minecraft:skeletons",
        "the apply the full tag lands"
    );
}

/// THE PARSERS' OWN SPLIT LAW (brigadier
/// ArgumentCommandNode.listSuggestions — the node's custom
/// provider (the 0x10 id) the ask-server lane answers FIRST, the
/// argument type's own listSuggestions the local lane answers:
/// entity / game-profile the house's tab belief speaks,
/// score-holder the 26.2 class no listSuggestions carries — the
/// honest ask, every other parser the same).
#[test]
fn the_argument_parsers_the_split_law_speaks() {
    use crate::screens::ChatSuggestionSource as Src;
    let tree = test_provider_tree();
    // THE SCORE-HOLDER SLOT (no local provider, no custom id — the
    // 26.2 ScoreHolderArgument no listSuggestions carries, the ask
    // lane the honest keep)
    assert_eq!(
        local_command_suggestions(&tree, "/score ", &["Alice".to_string()], true, &[]),
        Src::AskServer,
        "the score-holder slot the ask lane keeps"
    );
    // THE CUSTOM PROVIDER (the entity slot WITH the 0x10 id — the
    // ask lane wins, the local tab belief the custom provider
    // never replaces)
    assert_eq!(
        local_command_suggestions(&tree, "/give ", &["Alice".to_string()], true, &[]),
        Src::AskServer,
        "the custom provider the ask lane answers"
    );
    // THE NO-TREE WORLD (the command tree never landed — the ask
    // lane the old walk keeps verbatim)
    assert_eq!(
        local_command_suggestions(
            &mc_protocol::play::CommandTree::default(),
            "/kick ",
            &["Alice".to_string()],
            true,
            &[],
        ),
        Src::AskServer,
        "the tree-less world the ask lane keeps"
    );
}

/// THE REFRESH'S OWN ENTITY LANE (the App's own walk — the tab
/// belief the player_list field carries, the '/kick ' edit the
/// LOCAL list lands, no request the lane spends, no pending id the
/// gate waits on).
#[test]
fn the_refresh_the_entity_slot_lands_local() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_command_tree(test_provider_tree());
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    // THE ROSTER'S OWN ARRIVAL (the live refresh the tab lane
    // ships — the names the App's own belief banks)
    app.live_player_list_updated(crate::world_view::WirePlayerList {
        header: String::new(),
        footer: String::new(),
        rows: vec![crate::world_view::WirePlayerRow {
            name: "Alice".to_string(),
            game_mode: 0,
            latency: 0,
            list_order: 0,
            score: None,
        }],
        names: vec!["Alice".to_string(), "bob".to_string()],
    });
    for c in "/kick ".chars() {
        app.on_chat_char(c);
    }
    assert!(
        rx.try_recv().is_err(),
        "the entity slot the LOCAL lane answers — no request the walk spends"
    );
    assert_eq!(app.chat_suggestion_pending, None, "the gate no id waits on");
    assert_eq!(
        app.chat_suggestions
            .iter()
            .map(|m| m.text.as_str())
            .collect::<Vec<_>>(),
        vec!["@a", "@e", "@n", "@p", "@r", "@s", "Alice", "bob"],
        "the merged sort the merged list keeps"
    );
    assert_eq!(app.chat_suggestion_range, (6, 0));
    // THE TAB APPLY (the selected '@a' the replace lands — the
    // input the selector wears, the narrowed state the fresh walk
    // re-answers)
    app.on_chat_tab_key();
    assert_eq!(app.chat_input, "/kick @a", "the apply the range keeps");
    assert_eq!(
        app.chat_suggestions.len(),
        6,
        "the parsed state the six still speaks"
    );
    // THE SERVER'S OWN WORD (the '/give ' slot the ask lane
    // answers — the pops walk back to the bare slash, the fresh
    // typing the custom provider's slot reaches)
    for _ in 0..7 {
        app.on_chat_backspace(); // "/kick @a" → "/" (the pops the walk keeps)
    }
    for c in "give ".chars() {
        app.on_chat_char(c);
    }
    assert_eq!(app.chat_input, "/give ");
    let got = rx
        .try_recv()
        .expect("the give slot's custom provider rides the request");
    assert!(matches!(
        got,
        crate::world_view::ContainerCommand::RequestCommandSuggestions { .. }
    ));
}

/// THE TREE'S OWN LANDING (the handleCommands walk — the field the
/// packet replaces, the wholesale replace the re-send answers).
#[test]
fn the_commands_tree_the_landing_speaks() {
    let mut app = App::new(3);
    assert!(
        app.command_tree.is_none(),
        "the offline world no tree carries"
    );
    app.on_command_tree(test_tree());
    let tree = app
        .command_tree
        .as_ref()
        .expect("the tree the landing banks");
    assert_eq!(tree.root_index, 0);
    assert_eq!(tree.nodes.len(), 8);
    // the re-send the replace law answers (a /reload the server's
    // own re-sync speaks)
    let mut re_sent = test_tree();
    re_sent.nodes[1].name = "time2".to_string();
    app.on_command_tree(re_sent);
    assert_eq!(
        app.command_tree.as_ref().unwrap().nodes[1].name,
        "time2",
        "the wholesale replace the landing keeps"
    );
}

/// THE SUGGESTION TYPING'S OWN WALK (javap ChatScreen.onEdited's
/// responder lane — the '/'-typed edits the ask-server request
/// rides on EVERY keystroke: the transaction id the counter
/// increments, the FULL input the '/' rides, the non-command text
/// the silent lane keeps, the OFFLINE world the no-connection walk
/// never asks).
#[test]
fn the_suggestion_typing_the_ask_server_rides() {
    let mut app = App::new(3);
    app.enter_world();
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    // THE NON-COMMAND TEXT (the no-slash edits the silent lane keeps)
    for c in "hello".chars() {
        app.on_chat_char(c);
    }
    assert!(
        rx.try_recv().is_err(),
        "the plain text the no-request lane keeps"
    );
    assert!(app.chat_suggestions.is_empty());
    // THE CLEAR (the pops the typed word answers — the fresh input
    // the '/'-first walk wears)
    for _ in 0..5 {
        app.on_chat_backspace();
    }
    assert_eq!(app.chat_input, "");
    // THE SLASH (the request the first '/' keystroke rides)
    app.on_chat_char('/');
    match rx.try_recv().expect("the '/' rides the request") {
        crate::world_view::ContainerCommand::RequestCommandSuggestions { id, command } => {
            assert_eq!(id, 1, "the counter's own increment");
            assert_eq!(command, "/", "the FULL input the '/' rides");
        }
        other => panic!("the wrong command rode the lane: {other:?}"),
    }
    // EVERY EDIT ASKS (the responder the keystroke walk fires)
    app.on_chat_char('t');
    match rx.try_recv().expect("the 't' edit rides") {
        crate::world_view::ContainerCommand::RequestCommandSuggestions { id, command } => {
            assert_eq!(id, 2, "the next edit the next id speaks");
            assert_eq!(command, "/t");
        }
        other => panic!("the wrong command rode the lane: {other:?}"),
    }
    // THE CLEAR (the backspace past the '/' the non-command gate
    // answers — the empty input the active law refuses)
    app.on_chat_backspace();
    assert!(rx.try_recv().is_ok(), "the '/t'→'/' edit still asks");
    app.on_chat_backspace();
    assert!(rx.try_recv().is_err(), "the empty input never asks");
    assert!(
        app.chat_suggestions.is_empty(),
        "the clear the list forgets"
    );
    assert_eq!(
        app.chat_suggestion_pending, None,
        "the pending the gate drops"
    );
}

/// THE SUGGESTION REPLY'S OWN LANDING (the pending id the equality
/// gate keeps — the STALE replies the dropped lane answers, the
/// matching id the fresh list wears: the rows, the range, the
/// original capture, the row-0 selection).
#[test]
fn the_suggestion_reply_the_pending_gate_speaks() {
    let mut app = App::new(3);
    app.enter_world();
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/tp".chars() {
        app.on_chat_char(c);
    }
    // the typing walk's own request: pending = Some(1)
    // THE STALE REPLY (the older id the gate drops)
    app.on_command_suggestions(mc_protocol::play::CommandSuggestionsReply {
        id: 0,
        start: 1,
        length: 2,
        matches: vec![mc_protocol::play::CommandSuggestionsMatch {
            text: "old".to_string(),
            tooltip: None,
        }],
    });
    assert!(
        app.chat_suggestions.is_empty(),
        "the stale id the list refuses"
    );
    // THE MATCHING REPLY (the fresh list lands)
    app.on_command_suggestions(mc_protocol::play::CommandSuggestionsReply {
        id: app.chat_suggestion_pending.unwrap(),
        start: 1,
        length: 2,
        matches: vec![
            mc_protocol::play::CommandSuggestionsMatch {
                text: "teleport".to_string(),
                tooltip: None,
            },
            mc_protocol::play::CommandSuggestionsMatch {
                text: "time".to_string(),
                tooltip: None,
            },
        ],
    });
    assert_eq!(
        app.chat_suggestions.len(),
        2,
        "the matching id the list wears"
    );
    assert_eq!(app.chat_suggestion_range, (1, 2));
    assert_eq!(
        app.chat_suggestion_original, "/tp",
        "the original the list captures"
    );
    assert_eq!(
        app.chat_suggestion_sel, 0,
        "the row-0 the fresh list speaks"
    );
    assert_eq!(
        app.chat_suggestion_pending, None,
        "the landing the pending clears"
    );
}

/// THE TAB + ARROWS' OWN WALK (javap SuggestionsList.keyPressed —
/// the FIRST Tab the useSuggestion applies (the range the
/// original's own [start, start+length) speaks), the tabCycles the
/// next Tab's cycle answers, the arrows the SHOWN list the cycle
/// keeps and the HIDDEN list the history recall wears, the wrap
/// the select's own clamps speak).
#[test]
fn the_suggestion_tab_and_arrows_the_selection_laws_speak() {
    let mut app = App::new(3);
    app.enter_world();
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/tp".chars() {
        app.on_chat_char(c);
    }
    app.on_command_suggestions(mc_protocol::play::CommandSuggestionsReply {
        id: app.chat_suggestion_pending.unwrap(),
        start: 1,
        length: 2,
        matches: vec![
            mc_protocol::play::CommandSuggestionsMatch {
                text: "teleport".to_string(),
                tooltip: None,
            },
            mc_protocol::play::CommandSuggestionsMatch {
                text: "time".to_string(),
                tooltip: None,
            },
            mc_protocol::play::CommandSuggestionsMatch {
                text: "trigger".to_string(),
                tooltip: None,
            },
        ],
    });
    // THE GHOST (the calculateSuggestionSuffix walk — the completed
    // '/teleport' minus the typed '/tp' common prefix)
    assert_eq!(app.chat_ghost_suffix(), "eleport");
    // THE FIRST TAB (the apply — the replace range the completed
    // input speaks, the tabCycles flag the next Tab's cycle keeps)
    app.on_chat_tab_key();
    assert_eq!(app.chat_input, "/teleport", "the range the apply replaces");
    assert!(app.chat_suggestion_tab_cycles, "the flag the use raises");
    // THE SECOND TAB (the cycle — the selection the +1 walk moves)
    app.on_chat_tab_key();
    assert_eq!(app.chat_suggestion_sel, 1, "the cycle the selection moves");
    // THE ARROWS WHILE SHOWN (the cycle the Up/Down answers, the
    // history the walk never reaches)
    app.on_chat_arrow(true); // Up from 1 → 0
    assert_eq!(app.chat_suggestion_sel, 0);
    app.on_chat_arrow(true); // Up from 0 → the WRAP to the last row
    assert_eq!(app.chat_suggestion_sel, 2, "the wrap the clamp speaks");
    app.on_chat_arrow(false); // Down from 2 → the WRAP to row 0
    assert_eq!(app.chat_suggestion_sel, 0);
    // THE OFFLINE WALK (no live lane — the '/' never asks, the
    // arrows the plain history laws keep)
    let mut offline = App::new(3);
    offline.enter_world();
    offline.on_chat_key();
    for c in "hello".chars() {
        offline.on_chat_char(c);
    }
    offline.on_chat_submit();
    offline.on_chat_key();
    offline.on_chat_arrow(true);
    assert_eq!(
        offline.chat_input, "hello",
        "the history the hidden list recalls"
    );
    assert!(
        offline.chat_suggestions.is_empty(),
        "the offline the list never lands"
    );
}

/// THE SUGGESTIONS' OWN MOUSE LANES (javap SuggestionsList's
/// extractRenderState hover + mouseClicked + mouseScrolled walks —
/// the drawn rect the hit lanes read, the moved gate the resting
/// cursor's honesty keeps, the click the apply tail speaks, the
/// scroll the offset window's clamp holds).
#[test]
fn the_suggestion_mouse_lanes_the_rect_laws_speak() {
    let mut app = App::new(3);
    app.enter_world();
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/tp".chars() {
        app.on_chat_char(c);
    }
    app.on_command_suggestions(mc_protocol::play::CommandSuggestionsReply {
        id: app.chat_suggestion_pending.unwrap(),
        start: 1,
        length: 2,
        matches: vec![
            mc_protocol::play::CommandSuggestionsMatch {
                text: "teleport".to_string(),
                tooltip: None,
            },
            mc_protocol::play::CommandSuggestionsMatch {
                text: "time".to_string(),
                tooltip: Some("§aTeleports or sets the time".to_string()),
            },
            mc_protocol::play::CommandSuggestionsMatch {
                text: "trigger".to_string(),
                tooltip: None,
            },
        ],
    });
    // THE RECT THE DRAW OWNS (the hidden list the None keeps — the
    // render the hit-box cache fills)
    assert!(
        app.chat_suggestion_rect.is_none(),
        "the rect the render owns"
    );
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let mut assets = GuiAssets::open(assets_root());
    app.render(&mut fb, &mut assets);
    let (rx, ry, rw, rh) = app.chat_suggestion_rect.expect("the drawn list's rect");
    // the constructor's Rect2i margins the bounds speak (the x - 1
    // / rows * 12 + 1 the ring pixels cover; the width the widest
    // row + 1 the fill spans)
    assert_eq!(rx, 2, "the input's 4 - 1 the x law keeps");
    assert_eq!(rh, 3 * 12 + 1, "the rows * 12 + 1 the height keeps");
    assert!(rw > 3, "the widest row + 1 the width carries");
    // THE HOVER'S OWN SELECT (the row 1 band the moved frame answers)
    app.on_mouse_move(rx + 4, ry + 12 + 5);
    assert_eq!(app.chat_suggestion_sel, 1, "the hover selects the row");
    // THE MOVED GATE (the keyboard the resting cursor never fights)
    app.on_chat_arrow(true); // Up — the keyboard moves sel to 0
    assert_eq!(app.chat_suggestion_sel, 0);
    app.on_mouse_move(rx + 4, ry + 12 + 5); // the SAME pos — no move
    assert_eq!(
        app.chat_suggestion_sel, 0,
        "the resting cursor never re-selects"
    );
    // THE DEAD BOUNDARY (the strict bands the rel % 12 line keeps)
    app.on_mouse_move(rx + 4, ry + 12); // the row boundary — dead
    assert_eq!(
        app.chat_suggestion_sel, 0,
        "the boundary row the band never names"
    );
    // THE X BAND (the outside the walk refuses)
    app.on_mouse_move(rx + rw + 2, ry + 12 + 5);
    assert_eq!(app.chat_suggestion_sel, 0, "the x outside the band refuses");
    // THE CLICK'S OWN GATES (the outside the false keeps, the
    // contained row the apply tail answers)
    assert!(
        !app.on_chat_click(rx + 4, ry - 5),
        "the outside the false keeps"
    );
    assert!(
        app.on_chat_click(rx + 4, ry + 12 + 5),
        "the contained walk answers 1"
    );
    assert_eq!(app.chat_input, "/time", "the selected row the apply wears");
    assert!(app.chat_suggestion_tab_cycles, "the flag the use raises");
    // THE BOTTOM RING (the h + 1 the constructor keeps — the tail
    // pixel the out-of-list gate the apply skips, the true keeps)
    let sel_before = app.chat_suggestion_sel;
    let input_before = app.chat_input.clone();
    assert!(
        app.on_chat_click(rx + 4, ry + rh - 1),
        "the ring the rect contains"
    );
    assert_eq!(
        app.chat_input, input_before,
        "the tail pixel the gate skips"
    );
    assert_eq!(app.chat_suggestion_sel, sel_before);
}

/// THE SUGGESTIONS' OWN SCROLL WINDOW (javap mouseScrolled — the
/// clamp fold the 15-row list speaks: the down the later rows
/// show, the up the earlier, the max the size - limit keeps, the
/// sub-notch the whole-expression trunc never spends).
#[test]
fn the_suggestion_scroll_the_window_walk_speaks() {
    let mut app = App::new(3);
    app.enter_world();
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.on_chat_key();
    for c in "/tp".chars() {
        app.on_chat_char(c);
    }
    let row = |n: usize| mc_protocol::play::CommandSuggestionsMatch {
        text: format!("teleport_to_target_{n}"),
        tooltip: None,
    };
    app.on_command_suggestions(mc_protocol::play::CommandSuggestionsReply {
        id: app.chat_suggestion_pending.unwrap(),
        start: 1,
        length: 2,
        matches: (0..15).map(row).collect(),
    });
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let mut assets = GuiAssets::open(assets_root());
    app.render(&mut fb, &mut assets);
    let (rx, ry, _rw, _rh) = app.chat_suggestion_rect.expect("the drawn list's rect");
    assert_eq!(app.chat_suggestion_offset, 0);
    // THE OUTSIDE (the contains refusal — the false keeps)
    assert!(
        !app.on_chat_scroll(0.0, -1.0, rx + 4, ry - 4),
        "the above refuses"
    );
    // THE DOWN SCROLL (the dsub fold — offset - (-1) the later rows show)
    assert!(
        app.on_chat_scroll(0.0, -1.0, rx + 4, ry + 5),
        "the contained answers 1"
    );
    assert_eq!(
        app.chat_suggestion_offset, 1,
        "the down the later rows show"
    );
    // THE UP SCROLL
    app.on_chat_scroll(0.0, 1.0, rx + 4, ry + 5);
    assert_eq!(
        app.chat_suggestion_offset, 0,
        "the up the earlier rows show"
    );
    // THE MAX CLAMP (the size - limit the fold keeps — 15 - 10 = 5)
    for _ in 0..8 {
        app.on_chat_scroll(0.0, -1.0, rx + 4, ry + 5);
    }
    assert_eq!(app.chat_suggestion_offset, 5, "the max the clamp keeps");
    // THE SUB-NOTCH (the whole-expression trunc — 5 - (-0.5) = 5.5
    // → 5: the remainder the offset never spends)
    app.on_chat_scroll(0.0, -0.5, rx + 4, ry + 5);
    assert_eq!(
        app.chat_suggestion_offset, 5,
        "the sub-notch the remainder keeps"
    );
}

/// THE TOOLTIP'S OWN PAINT (the extractRenderState tail — the
/// hovered band the gate speaks, the current row's tooltip the box
/// carries: the 0xF0100010 fill the gradient ring paints, the
/// text the §-codes the shared draw consumes, the tooltip-less row
/// the box never paints).
#[test]
fn the_suggestion_tooltip_the_hover_band_paints() {
    let build = |tip: Option<&str>| {
        let mut app = App::new(3);
        app.enter_world();
        let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
        app.attach_live_clicks(tx);
        app.on_chat_key();
        for c in "/tp".chars() {
            app.on_chat_char(c);
        }
        app.on_command_suggestions(mc_protocol::play::CommandSuggestionsReply {
            id: app.chat_suggestion_pending.unwrap(),
            start: 1,
            length: 2,
            matches: vec![
                mc_protocol::play::CommandSuggestionsMatch {
                    text: "teleport".to_string(),
                    tooltip: None,
                },
                mc_protocol::play::CommandSuggestionsMatch {
                    text: "time".to_string(),
                    tooltip: tip.map(|s| s.to_string()),
                },
            ],
        });
        app
    };
    let mut app = build(Some("§aTeleports or sets"));
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let mut assets = GuiAssets::open(assets_root());
    app.render(&mut fb, &mut assets);
    let (rx, ry, _rw, _rh) = app.chat_suggestion_rect.expect("the drawn list's rect");
    // THE HOVER (the move the row 1 selects — the tooltip's own row)
    app.on_mouse_move(rx + 4, ry + 12 + 5);
    assert_eq!(app.chat_suggestion_sel, 1, "the hover the tip row selects");
    app.render(&mut fb, &mut assets);
    // THE BOX (the positioner's +12/-12 the no-flip walk keeps —
    // the box the gradient ring paints over the list's ground)
    let (bx, by) = (rx + 4 + 12 - 1, ry + 12 + 5 - 12 - 1);
    let px = |fb: &crate::raster::Framebuf, x: i32, y: i32| {
        let i = (y as usize * 320 + x as usize) * 3;
        [fb.color[i], fb.color[i + 1], fb.color[i + 2]]
    };
    // THE RING (the 0x505000FF start strip — the blue the red
    // dominates, the same law the blue_wins probe keeps)
    let ring = px(&fb, bx, by);
    assert!(
        ring[2] > ring[0],
        "the start strip the blue speaks: {ring:?}"
    );
    // THE FILL (the 0xF0100010 — the dark red-tinted ground the
    // alpha carries over the list's black: the red the raw list's
    // pure-black ground speaks)
    let fill = px(&fb, bx + 4, by + 4);
    assert!(
        fill[0] > 0 || fill[2] > 0,
        "the fill the tinted ground speaks: {fill:?}"
    );
    // THE TOOLTIP-LESS WALK (the None tip the box never paints —
    // the same walk the fill/ring pixels keep silent)
    let mut plain = build(None);
    let mut fb2 = crate::raster::Framebuf::new(320, 180);
    plain.render(&mut fb2, &mut assets);
    plain.on_mouse_move(rx + 4, ry + 12 + 5);
    plain.render(&mut fb2, &mut assets);
    // the honest check: the box's region DIFFERS between the tip
    // and the plain frames (the rows beneath the same pixels paint
    // — the box the diff's own law carries)
    let mut diff = 0usize;
    for dy in 0..15i32 {
        for dx in 0..40i32 {
            if px(&fb, bx + dx, by + dy) != px(&fb2, bx + dx, by + dy) {
                diff += 1;
            }
        }
    }
    assert!(diff > 0, "the box the frame diff speaks: {diff}");
}

#[test]
fn the_offline_submit_the_echo_keeps() {
    // THE OFFLINE GUARD (no live lane — the local echo the
    // display queue answers, the same law the chat round kept)
    let mut app = App::new(3);
    app.enter_world();
    app.on_chat_key();
    for c in "just me".chars() {
        app.on_chat_char(c);
    }
    app.on_chat_submit();
    assert_eq!(
        app.chat_messages.last().map(|m| m.0.as_str()),
        Some("just me"),
        "the offline echo the display answers"
    );
}

#[test]
fn the_t_key_opens_the_chat_lane() {
    // THE PURE LAWS' OWN VERDICTS (the AlphaCalculator.timeBased
    // lambda the bytecode verbatim speaks: age 0 → 1.0, the 180th
    // tick the full keeps, the 190th the 0.25 fold wears
    // ((1-0.95)*10 = 0.5 → 0.25), the 200th the zero answers; the
    // page the focused 180/9 = 20 / unfocused 100/9 = 11 keep; the
    // y-fold the newest line at guiHeight - 41 speaks)
    assert_eq!(chat_line_alpha(0), 1.0);
    assert_eq!(chat_line_alpha(180), 1.0);
    assert!((chat_line_alpha(190) - 0.25).abs() < 1e-6);
    assert_eq!(chat_line_alpha(200), 0.0);
    assert_eq!(chat_line_alpha(300), 0.0);
    assert_eq!(chat_lines_per_page(true), 20);
    assert_eq!(chat_lines_per_page(false), 11);
    assert_eq!(chat_line_y(360, 0, 0.0), 360 - 41);
    assert_eq!(chat_line_y(360, 1, 0.0), 360 - 50);

    // THE STATE WALK (the handleKeybinds keyChat lane the T edge
    // opens, the EditBox the chars the 256 cap feeds, the
    // backspace the pop keeps, the handleChatInput the trim +
    // empty-close law wears, the non-empty the addRecentChat +
    // the local echo pushes, the consecutive duplicate the skip
    // keeps)
    let mut app = App::new(3);
    app.enter_world();
    assert!(!app.chat_open);
    app.on_chat_key();
    assert!(app.chat_open);
    for c in "hello".chars() {
        app.on_chat_char(c);
    }
    assert_eq!(app.chat_input, "hello");
    app.on_chat_backspace();
    assert_eq!(app.chat_input, "hell");
    app.on_chat_history_key(true);
    assert_eq!(app.chat_input, "hell", "the empty recent the walk keeps");
    for _ in 0..4 {
        app.on_chat_backspace();
    }
    assert_eq!(app.chat_input, "", "the pops the empty input answers");
    app.on_chat_submit();
    assert!(!app.chat_open, "the empty-trim close the silent law keeps");
    assert!(
        app.chat_messages.is_empty(),
        "the empty submit no echo rides"
    );
    assert!(
        app.recent_chat.is_empty(),
        "the empty submit no history keeps"
    );

    app.on_chat_key();
    app.chat_input.clear();
    for c in "  hello world  ".chars() {
        app.on_chat_char(c);
    }
    app.on_chat_submit();
    assert_eq!(
        app.recent_chat,
        ["hello world"],
        "the trim the normalize law keeps"
    );
    assert_eq!(app.chat_messages.len(), 1, "the local echo the push speaks");
    assert_eq!(
        app.chat_messages[0].1, app.frame as u32,
        "the addedTime the frame law keeps"
    );
    app.on_chat_key();
    for c in "hello world".chars() {
        app.on_chat_char(c);
    }
    app.on_chat_submit();
    assert_eq!(
        app.recent_chat.len(),
        1,
        "the consecutive dup the skip keeps"
    );

    // THE HISTORY RECALL (the moveInHistory walk — the Up the
    // OLDER walks, the draft the leaving keeps, the Down the
    // newer returns, the free pos the draft restores)
    app.on_chat_key();
    app.chat_input.clear();
    for c in "draft word".chars() {
        app.on_chat_char(c);
    }
    app.on_chat_history_key(true);
    assert_eq!(
        app.chat_input, "hello world",
        "the Up the older entry recalls"
    );
    app.on_chat_history_key(false);
    assert_eq!(app.chat_input, "draft word", "the Down the draft restores");
    app.close_chat();

    // THE WRAP'S OWN LAW (the refreshTrimmedMessages walk — the
    // short the one line keeps, the long the 320px width splits)
    assert_eq!(wrap_chat_text("hi there", 320).len(), 1);
    let long = "the quick brown fox jumps over the lazy dog again and again until the line wraps past the chat width the game keeps";
    let wrapped = wrap_chat_text(long, 320);
    assert!(wrapped.len() > 1, "the long message the wrap splits");
    for seg in &wrapped {
        assert!(
            crate::gui::font_text_width(seg) <= 320,
            "the wrapped line the width cap keeps: {} ({}px)",
            seg,
            crate::gui::font_text_width(seg)
        );
    }

    // THE PIXELS' OWN VERDICT (the cache-root walk the inventory
    // test keeps): the fresh message the bottom-left band + text
    // the bare world answers, the 200-tick age the fade zeroes,
    // the open chat the input band paints — even over the F1
    // hidden HUD the Screen the isHidden flag never gates
    fn diff_count(a: &[u8], b: &[u8], x0: i32, y0: i32, w: i32, h: i32) -> usize {
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
    }
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
    // THE DAY CLOCK'S OWN PLATEAU (the frame == tick model the sky
    // lane rides — the base the test pins into the flat day plateau
    // 133..11867: the bare frame and the 201-tick aged frame the
    // SAME flat sky sample, the byte-equality the fade's zero
    // answers)
    app.clock_total = 6000 + app.frame;
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    app.render(&mut fb, &mut assets);
    let bare = fb.color.clone();
    // THE UNFOCUSED HISTORY (the fresh message the fade's full
    // alpha wears — the newest line's band at (2, y-1) the
    // chat_line_y law keeps)
    app.push_chat_message("chat hello".to_string());
    app.render(&mut fb, &mut assets);
    let y = chat_line_y(360, 0, 0.0);
    let hist_diff = diff_count(&bare, &fb.color, 2, y - 1, 200, 10);
    assert!(
        hist_diff > 40,
        "the chat line the band + text paints: {hist_diff}"
    );
    // THE AGE THE LIVING WORLD'S LANES COINCIDE (the fade's zero
    // the line skips — the band the bare world's own bytes return).
    // The frame == tick model the sky/celestial lane rides moves the
    // aged frame's sky the band never contains — the band-level
    // equality the chat line's own fade answers. THE ANIMATION'S OWN
    // SHIFT: the world's living lanes (the water/lava/fire strips the
    // draw-time remap repaints) the 1216-tick shift never repaints —
    // the multiple every strip's own period wears (water 64, lava
    // 76, fire 32) — while the age the fade's 200-tick window
    // outlives, the plateau the same flat sky sample keeps (6000 +
    // 1216 = 7216 < 11867).
    app.frame += 1216;
    // THE CLOCK RIDES THE SHIFT (the old base+frame fold the pair
    // moved together — the sky band the 7216 sample answers)
    app.clock_total += 1216;
    app.render(&mut fb, &mut assets);
    let aged_band = diff_count(&bare, &fb.color, 2, y - 1, 200, 10);
    assert_eq!(
        aged_band, 0,
        "the aged line the fade never draws — the bare band returns"
    );
    app.frame -= 1216;
    app.clock_total -= 1216;
    // THE FOCUSED INPUT (the ChatScreen's own layer — the band the
    // hidden HUD's frame still paints, the F1 law the Screen never
    // gates)
    app.hud_hidden = true;
    app.render(&mut fb, &mut assets);
    let hidden_bare = fb.color.clone();
    app.on_chat_key();
    for c in "typing".chars() {
        app.on_chat_char(c);
    }
    app.render(&mut fb, &mut assets);
    let band_diff = diff_count(&hidden_bare, &fb.color, 2, 360 - 14, 636, 12);
    assert!(
        band_diff > 400,
        "the input band the open chat paints: {band_diff}"
    );
}

/// THE CHAT COLORS' OWN VERDICTS (the component walk the wire's
/// `color` field speaks, the wrap the ACTIVE §-code re-opens, the
/// decoration the §r tail the template's own style wears, the
/// draw the §r reset the entry-base law answers — the pixels the
/// colored line paints).
#[test]
fn the_chat_colors_the_walk_the_lines_re_speak() {
    // THE WRAP'S OWN CARRY (the StringSplitter's own law — every
    // wrapped line the active code re-opens, the color the line
    // break never strips)
    let colored = "\u{a7}c".to_string() + &"the quick brown fox jumps over the lazy dog".repeat(3);
    let wrapped = wrap_chat_text(&colored, 320);
    assert!(wrapped.len() > 1, "the colored line the wrap splits");
    for (i, seg) in wrapped.iter().enumerate() {
        assert!(
            crate::gui::font_text_width(seg) <= 320,
            "the wrapped line the width cap keeps: {seg}"
        );
        if i > 0 {
            assert!(
                seg.starts_with("\u{a7}c"),
                "the wrapped line the active code re-opens: {seg}"
            );
        }
    }
    // THE UNCOLORED PASSTHROUGH (the plain line no code wears —
    // the wrap the source's own bytes keeps)
    let long = "the quick brown fox jumps over the lazy dog again and again until the line wraps past the chat width the game keeps";
    for seg in wrap_chat_text(long, 320) {
        assert!(
            !seg.contains('\u{a7}'),
            "the plain line no code wears: {seg}"
        );
    }
    // THE MID-STRING CODE (the lines BEFORE the code stay bare,
    // the lines AFTER the code re-open it — the state the split
    // position carries)
    let mid = format!("{}\u{a7}9 {}", "word ".repeat(30), "tail ".repeat(30));
    let wrapped = wrap_chat_text(mid.trim_end(), 320);
    let mut seen_code = false;
    for seg in &wrapped {
        if seg.contains("\u{a7}9") {
            seen_code = true;
        } else if seen_code {
            assert!(
                seg.starts_with("\u{a7}9"),
                "the post-code line the state re-opens: {seg}"
            );
        } else {
            assert!(
                !seg.contains('\u{a7}'),
                "the pre-code line stays bare: {seg}"
            );
        }
    }
    assert!(seen_code, "the mid-string code the fixture wears");
    // THE DECORATION'S OWN RESET (the arg the §-codes carries the
    // §r tail wears — the template's literals the TEMPLATE's
    // style speaks, never the argument's)
    assert_eq!(
        format_chat_decoration("chat.type.text", &["\u{a7}9Steve", "gg"]),
        "<\u{a7}9Steve\u{a7}r> gg",
        "the coded arg the \u{a7}r tail wears"
    );
    assert_eq!(
        format_chat_decoration("chat.type.text", &["Steve", "gg"]),
        "<Steve> gg",
        "the code-less arg no tail wears"
    );

    // THE PIXELS' OWN VERDICT (the harness the chat lane keeps):
    // the §c line the RED the white line's own bytes answers, the
    // §r tail the BASE returns (the reset the draw's own law
    // speaks), the reset's head the red run's own bytes keeps
    fn diff_count(a: &[u8], b: &[u8], x0: i32, y0: i32, w: i32, h: i32) -> usize {
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
    }
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
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    // THE COLORED LINE (the newest line's own region the fresh
    // push's full alpha wears)
    app.push_chat_message("\u{a7}cAAAAAA".to_string());
    app.render(&mut fb, &mut assets);
    let red_frame = fb.color.clone();
    app.chat_messages[0].0 = "AAAAAA".to_string();
    app.render(&mut fb, &mut assets);
    let white_frame = fb.color.clone();
    let y = chat_line_y(360, 0, 0.0);
    let color_diff = diff_count(&red_frame, &white_frame, 2, y - 1, 210, 10);
    assert!(color_diff > 40, "the code the color paints: {color_diff}");
    // THE RESET'S OWN TAIL (the §r the base returns — the tail
    // the red line and the reset line differ, the HEAD the red
    // run's own bytes keeps)
    app.chat_messages[0].0 = "\u{a7}cAAA\u{a7}rAAA".to_string();
    app.render(&mut fb, &mut assets);
    let reset_frame = fb.color.clone();
    let head_diff = diff_count(&red_frame, &reset_frame, 2, y - 1, 20, 10);
    assert_eq!(head_diff, 0, "the reset's head the red run keeps");
    let tail_diff = diff_count(&red_frame, &reset_frame, 20, y - 1, 190, 10);
    assert!(tail_diff > 20, "the \u{a7}r the base returns: {tail_diff}");
    // THE SHADOW'S OWN DIM (the §-code color the 0.25 fold
    // darkens — the vanilla shadow's own law the bright-code bug
    // the pre-walk draw wore; the shadow pixel the glyph's own
    // (+1,+1) copy speaks)
    let px = |fb: &crate::raster::Framebuf, x: usize, y: usize| -> [f32; 3] {
        let o = (y * 640 + x) * 3;
        [
            fb.color[o] as f32 / 255.0,
            fb.color[o + 1] as f32 / 255.0,
            fb.color[o + 2] as f32 / 255.0,
        ]
    };
    let mut base_fb = crate::raster::Framebuf::new(640, 360);
    let mut base_no = crate::raster::Framebuf::new(640, 360);
    let mut coded_fb = crate::raster::Framebuf::new(640, 360);
    let mut coded_no = crate::raster::Framebuf::new(640, 360);
    {
        let mut g = Gui::new(&mut base_fb, 1);
        g.text_faded(&mut assets, 10, 10, "A", [1.0, 1.0, 1.0], 1.0, true);
    }
    {
        let mut g = Gui::new(&mut base_no, 1);
        g.text_faded(&mut assets, 10, 10, "A", [1.0, 1.0, 1.0], 1.0, false);
    }
    {
        let mut g = Gui::new(&mut coded_fb, 1);
        g.text_faded(&mut assets, 10, 10, "\u{a7}cA", [1.0, 1.0, 1.0], 1.0, true);
    }
    {
        let mut g = Gui::new(&mut coded_no, 1);
        g.text_faded(&mut assets, 10, 10, "\u{a7}cA", [1.0, 1.0, 1.0], 1.0, false);
    }
    // THE CODED SHADOW (the shadow-only pixels the dim + the code
    // color carry — red-tinted dark, never the full-bright the
    // pre-dim draw wore)
    let mut coded_shadow_pixels = 0usize;
    for y in 0..360usize {
        for x in 0..640usize {
            let shadowed = px(&coded_fb, x, y);
            let main = px(&coded_no, x, y);
            if shadowed[0] > 0.02 && main[0] <= 0.02 {
                coded_shadow_pixels += 1;
                assert!(
                    shadowed[0] < 0.6,
                    "the shadow the 0.25 dim keeps: {shadowed:?}"
                );
                assert!(
                    shadowed[0] > shadowed[1] + 0.03,
                    "the code color the shadow carries: {shadowed:?}"
                );
            }
        }
    }
    assert!(
        coded_shadow_pixels > 3,
        "the shadow pixels the (+1,+1) copy paints: {coded_shadow_pixels}"
    );
    // THE BASE'S OWN SHADOW (the white text the gray shadow wears
    // — the r==g the no-code shadow speaks, the same dim the
    // reference verdict keeps)
    let mut base_shadow_pixels = 0usize;
    for y in 0..360usize {
        for x in 0..640usize {
            let shadowed = px(&base_fb, x, y);
            let main = px(&base_no, x, y);
            if shadowed[0] > 0.02 && main[0] <= 0.02 {
                base_shadow_pixels += 1;
                assert!(
                    (shadowed[0] - shadowed[1]).abs() < 0.02,
                    "the base shadow the gray speaks: {shadowed:?}"
                );
            }
        }
    }
    assert!(
        base_shadow_pixels > 3,
        "the base shadow pixels the same copy paints: {base_shadow_pixels}"
    );
}

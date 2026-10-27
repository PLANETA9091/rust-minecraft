use super::*;

// THE WORLD EDITS' OWN ROUND TRIP (the offline save's own law — the
// file the session's journal banks, the reload the rows the fresh
// terrain replays, the seed gate the foreign file refuses, the
// cadence tick the dirty world's own flush wears)
#[test]
fn the_world_edits_journal_round_trips_the_session() {
    let dir = std::env::temp_dir().join(format!("mc2rs_world_edits_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("world_edits.nbt");
    let mut app = App::new(3);
    app.offline_world_file = Some(path.clone());
    app.enter_world();
    app.offline_world.as_mut().unwrap().set(10, 5, 10, 41);
    app.offline_world
        .as_mut()
        .unwrap()
        .set_variant(10, 5, 10, 3);
    app.offline_world_file_save();
    // THE FILE the rows wear (the save the marks bank — the dirty
    // verdict the next cadence the clean verdict speaks)
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(w.saved_edits_mark, 1, "the flush the mark banks");
    let bytes = std::fs::read(&path).expect("the flush the file writes");
    let (_, root) =
        mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the file the parse answers");
    let (seed, edits, variants) =
        crate::screens::world_edits_from_nbt(&root).expect("the rows the parse wears");
    assert_eq!(seed, 3);
    assert_eq!(edits, vec![(10, 5, 10, 41u16)]);
    assert_eq!(variants, vec![(10, 5, 10, 3u8)]);
    // THE RELOAD: the fresh world the rows wear (the vanilla law — the
    // restart the session's builds outlive; the journal the file's own
    // rows the baseline keeps, the session's edits append)
    let mut app2 = App::new(3);
    app2.offline_world_file = Some(path.clone());
    app2.enter_world();
    let w2 = app2.offline_world.as_ref().unwrap();
    assert_eq!(w2.get(10, 5, 10), 41, "the placed block the reload wears");
    assert_eq!(w2.variant(10, 5, 10), 3, "the state byte the reload wears");
    assert!(w2.journaling, "the armed gate the session continues");
    assert_eq!(w2.edit_journal, edits, "the file the baseline keeps");
    // THE SEED GATE: the foreign file the fresh terrain refuses
    let mut app3 = App::new(99);
    app3.offline_world_file = Some(path.clone());
    app3.enter_world();
    let w3 = app3.offline_world.as_ref().unwrap();
    assert!(
        w3.edit_journal.is_empty(),
        "the foreign seed the apply never answers"
    );
    assert!(w3.journaling, "the fresh session the gate arms");
    // THE CADENCE: the dirty world the tick's own flush banks (the
    // throttle the hold-mine's per-frame write never pays)
    let _ = std::fs::remove_file(&path);
    app2.offline_world.as_mut().unwrap().set(11, 5, 10, 41);
    app2.update();
    assert!(path.exists(), "the dirty tick the flush banks");
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
}

// THE SAVED PLAYER'S OWN ROUND TRIP (the level.dat player tag's own
// law — the pose the reload re-anchors, the pockets the inventory
// restores, the selected slot the HUD re-wears, the RESPAWN point the
// world spawn keeps)
#[test]
fn the_saved_player_row_restores_the_pose_and_the_pockets() {
    let dir = std::env::temp_dir().join(format!("mc2rs_saved_player_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("world_edits.nbt");
    let mut app = App::new(3);
    app.offline_world_file = Some(path.clone());
    app.enter_world();
    app.spawn_offline_body();
    // THE STAGED SESSION: the body walks, the pockets fill, the
    // selection moves (the state the file's player row banks)
    {
        let b = app.body.as_mut().unwrap();
        b.x = 12.5;
        b.y = 8.0;
        b.z = 20.5;
        b.yaw = 91.0;
        b.pitch = -13.5;
    }
    app.inventory.items[0] = (4, 17);
    app.inventory.items[1] = (1, 3);
    app.inventory.armor[0] = (748, 1);
    app.inventory.offhand[0] = (903, 5);
    app.selected_hotbar = 2;
    app.offline_world_file_save();
    // THE RELOAD: the fresh boot the row wears (the pose the spawn
    // re-anchors, the pockets the gameplay inventory re-wears)
    let mut app2 = App::new(3);
    app2.offline_world_file = Some(path.clone());
    app2.enter_world();
    assert_eq!(
        app2.inventory.items[0],
        (4, 17),
        "the pockets the reload wears"
    );
    assert_eq!(app2.inventory.items[1], (1, 3));
    assert_eq!(app2.inventory.armor[0], (748, 1));
    assert_eq!(app2.inventory.offhand[0], (903, 5));
    assert_eq!(app2.selected_hotbar, 2, "the selection the reload wears");
    assert!(app2.saved_player.is_some(), "the row the load banks");
    app2.spawn_offline_body();
    let b2 = app2.body.as_ref().unwrap();
    assert_eq!(b2.x, 12.5, "the saved pose the spawn wears");
    assert_eq!(b2.y, 8.0);
    assert_eq!(b2.z, 20.5);
    assert_eq!(b2.yaw, 91.0);
    assert_eq!(b2.pitch, -13.5);
    // THE RESPAWN POINT the world spawn stays (the death lane's own
    // law — the saved pose the FIRST spawn alone wears)
    let sp = app2.spawn_pose.expect("the spawn the surface scan banks");
    assert_ne!(sp.0, 12.5, "the respawn the world spawn keeps");
    // THE FRESH PLAYER: the absent row the surface spawn answers
    let dir3 = std::env::temp_dir().join(format!("mc2rs_fresh_player_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir3);
    let mut app3 = App::new(3);
    app3.offline_world_file = Some(dir3.join("world_edits.nbt"));
    app3.enter_world();
    app3.spawn_offline_body();
    let b3 = app3.body.as_ref().unwrap();
    assert_eq!(
        b3.x,
        app3.world_size as f64 / 2.0,
        "the fresh player the world spawn wears"
    );
    assert!(app3.saved_player.is_none(), "the fresh row the None keeps");
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
    let _ = std::fs::remove_dir(&dir3);
}

// THE VITALS' OWN SAVE ROW (the player tag's Health/food/Xp rows the
// same file wears — the hurt bar, the drained food quartet, the
// earned xp trio the reload re-wears; the ELDER file's missing rows
// the vanilla spawn state answers)
#[test]
fn the_saved_player_row_wears_the_vitals() {
    let dir = std::env::temp_dir().join(format!("mc2rs_saved_vitals_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("world_edits.nbt");
    let mut app = App::new(3);
    app.offline_world_file = Some(path.clone());
    app.enter_world();
    app.spawn_offline_body();
    // THE STAGED SESSION: the hurt bar, the drained food quartet (the
    // from_saved constructor the staging dogfoods), the earned xp trio
    app.health = 13.5;
    app.food_data = crate::food::FoodData::from_saved(9, 1.5, 2.0, 7);
    app.xp = crate::xp::ExperienceStat {
        level: 3,
        progress: 0.25,
        total: 42,
    };
    app.offline_world_file_save();
    // THE FILE the vitals rows wear (the vanilla tag's own field
    // names the parse re-reads)
    let bytes = std::fs::read(&path).expect("the flush the file writes");
    let (_, root) =
        mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the file the parse answers");
    let sp = crate::screens::player_state_from_nbt(&root).expect("the row the parse wears");
    assert_eq!(sp.health, 13.5, "the hurt bar the file banks");
    assert_eq!(sp.food_level, 9, "the drained bar the file banks");
    assert_eq!(sp.saturation, 1.5, "the drained saturation the file banks");
    assert_eq!(sp.exhaustion, 2.0, "the banked exhaustion the file wears");
    assert_eq!(
        sp.tick_timer, 7,
        "the shared beat's own count the file wears"
    );
    assert_eq!(sp.xp_level, 3, "the level the file banks");
    assert_eq!(sp.xp_progress, 0.25, "the progress the file banks");
    assert_eq!(sp.xp_total, 42, "the lifetime points the file banks");
    // THE RELOAD: the fresh boot the vitals wear (the HUD the same
    // bars re-draws, the food sim the saved quartet resumes)
    let mut app2 = App::new(3);
    app2.offline_world_file = Some(path.clone());
    app2.enter_world();
    assert_eq!(app2.health, 13.5, "the bar the reload wears");
    assert_eq!(app2.food_data.food_level(), 9, "the bar the reload wears");
    assert_eq!(
        app2.food_data.saturation(),
        1.5,
        "the saturation the reload wears"
    );
    assert_eq!(
        app2.food_data.exhaustion(),
        2.0,
        "the bank the reload wears"
    );
    assert_eq!(app2.food_data.tick_timer(), 7, "the timer the reload wears");
    assert_eq!(app2.xp.level, 3, "the level the reload wears");
    assert_eq!(app2.xp.progress, 0.25, "the progress the reload wears");
    assert_eq!(app2.xp.total, 42, "the points the reload wears");
    // THE ELDER FILE: the missing vitals rows the spawn state answers
    // (the pre-vitals tag the fresh constructor's own quartet/trio
    // wears — the pose + the pockets the same parse still banks)
    let elder = mc_protocol::nbt::Nbt::Compound(vec![(
        "player".to_string(),
        mc_protocol::nbt::Nbt::Compound(vec![
            (
                "Pos".to_string(),
                mc_protocol::nbt::Nbt::List(vec![
                    mc_protocol::nbt::Nbt::Double(1.0),
                    mc_protocol::nbt::Nbt::Double(2.0),
                    mc_protocol::nbt::Nbt::Double(3.0),
                ]),
            ),
            (
                "Rotation".to_string(),
                mc_protocol::nbt::Nbt::List(vec![
                    mc_protocol::nbt::Nbt::Float(45.0),
                    mc_protocol::nbt::Nbt::Float(-5.0),
                ]),
            ),
            ("SelectedHotbar".to_string(), mc_protocol::nbt::Nbt::Int(1)),
            ("Items".to_string(), mc_protocol::nbt::Nbt::List(vec![])),
            ("Armor".to_string(), mc_protocol::nbt::Nbt::List(vec![])),
            ("Offhand".to_string(), mc_protocol::nbt::Nbt::List(vec![])),
        ]),
    )]);
    let sp2 = crate::screens::player_state_from_nbt(&elder).expect("the elder row the parse wears");
    assert_eq!(sp2.x, 1.0, "the pose the elder parse keeps");
    assert_eq!(
        sp2.selected_hotbar, 1,
        "the selection the elder parse keeps"
    );
    assert_eq!(
        sp2.health,
        crate::screens::MAX_HEALTH,
        "the elder file the spawn bar wears"
    );
    assert_eq!(
        sp2.food_level,
        crate::food::MAX_FOOD,
        "the elder file the spawn bar wears"
    );
    assert_eq!(
        sp2.saturation,
        crate::food::START_SATURATION,
        "the elder file the spawn saturation wears"
    );
    assert_eq!(sp2.exhaustion, 0.0, "the elder file the fresh bank wears");
    assert_eq!(sp2.tick_timer, 0, "the elder file the fresh beat wears");
    assert_eq!(sp2.xp_level, 0, "the elder file the fresh level wears");
    assert_eq!(
        sp2.xp_progress, 0.0,
        "the elder file the fresh progress wears"
    );
    assert_eq!(sp2.xp_total, 0, "the elder file the fresh points wear");
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
}

/// THE TORCH'S OWN BEAT THE DRAIN'S LAMP SEEDS WEAR (the autopush-116
/// round — javap RedStoneWireBlock.updatePowerStrength the 26.2 jar
/// the changed cells' own setBlock flag-3 the Level.updateNeighborsAt
/// six the neighborChanged broadcast speaks, the RedstoneLampBlock the
/// lamp among the flips' own neighborhoods the consult answers): the
/// lit torch the wire's own 15 the run's tail lamp the press path's
/// own consult lights, the lever press the NOT-gate the +2 beat flips,
/// the OFF flip the wire's drop the SAME drain the tail lamp's STALE
/// lit kind retires (the press path's own [redstone_lamp_flips]
/// helper the drain's seeds wear — the lamp the lever's twelve-cell
/// scope never reaches), the second press the relight the lamp's lit
/// kind the same consult re-arms.
#[test]
fn the_drain_lamp_seeds_the_tail_lamp_the_not_gate_cycle_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=8 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
                w.set(c + dx, top + 2, c + dz, crate::world::AIR);
            }
        }
        // THE NOT-GATE'S OWN SEATS (the lever the pedestal's west
        // neighbor the ray reads, the torch the pedestal's top the
        // LIT default wears, the wire the torch's east neighbor the
        // support stone carries, the tail lamp the wire's east end
        // the same support row wears — the lamp BESIDE the tail
        // rung the lever's own scope never reaches)
        w.set(c + 2, top, c, crate::world::LEVER);
        w.set_variant(c + 2, top, c, 1);
        w.set(c + 3, top, c, crate::world::STONE);
        w.set(c + 3, top + 1, c, crate::world::REDSTONE_TORCH);
        w.set(c + 4, top, c, crate::world::STONE);
        w.set(c + 4, top + 1, c, crate::world::REDSTONE_WIRE);
        w.set(c + 5, top, c, crate::world::STONE);
        w.set(c + 5, top + 1, c, crate::world::REDSTONE_LAMP);
    }
    // THE LIT TORCH'S OWN ARM (the attach walk's own seam the
    // placed-wire beat speaks — the component re-read the torch's
    // 15 the wire wears, the press path's own lamp consult the
    // tail lamp the ON rung's 15 lights)
    app.attach_wire_voice(c + 4, top + 1, c);
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 4, top + 1, c),
            crate::world::REDSTONE_WIRE_ON,
            "the lit torch the wire's own 15 wears"
        );
        assert_eq!(
            w.get(c + 5, top + 1, c),
            crate::world::REDSTONE_LAMP_LIT,
            "the press path's own consult the tail lamp lights"
        );
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE ON PRESS (the NOT-gate's own consult the lever's scope
    // arms — the tail lamp the STALE lit kind keeps until the drain)
    assert!(app.on_right_press(0, 0), "the lever's ON press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 2, top, c),
            crate::world::LEVER_ON,
            "the lever's own kind the ON bit wears"
        );
        assert_eq!(
            w.get(c + 5, top + 1, c),
            crate::world::REDSTONE_LAMP_LIT,
            "the press beat the lamp's stale kind keeps — the +2 consult the flip alone answers"
        );
    }
    // THE DRAIN'S OWN BEAT (the +2 consult the tick walk serves —
    // the torch the extinguish arm flips OFF, the wire re-read the
    // component's drop the same walk answers, THE NEW LAMP SEEDS
    // the tail lamp the wire's drop the stale kind RETIRES)
    for _ in 0..3 {
        app.update();
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 3, top + 1, c),
            crate::world::REDSTONE_TORCH_OFF,
            "the NOT-gate's own verdict the torch's LIT bit drops"
        );
        assert_eq!(
            w.get(c + 4, top + 1, c),
            crate::world::REDSTONE_WIRE,
            "the dead source the wire's plain kind wears"
        );
        assert_eq!(
            w.get(c + 5, top + 1, c),
            crate::world::REDSTONE_LAMP,
            "the drain's own lamp seeds the tail lamp the stale kind retires"
        );
    }
    // THE SECOND PRESS (the lever's death the consult re-arms —
    // the relight the +2 beat serves, the wire's 15 the re-read
    // re-wears, the tail lamp the same consult the lit kind re-arms)
    assert!(app.on_right_press(0, 0), "the lever's OFF press consumes");
    for _ in 0..3 {
        app.update();
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 3, top + 1, c),
            crate::world::REDSTONE_TORCH,
            "the dead support the torch's LIT bit re-arms"
        );
        assert_eq!(
            w.get(c + 4, top + 1, c),
            crate::world::REDSTONE_WIRE_ON,
            "the relit torch the wire's own 15 re-wears"
        );
        assert_eq!(
            w.get(c + 5, top + 1, c),
            crate::world::REDSTONE_LAMP_LIT,
            "the drain's own lamp seeds the tail lamp the lit kind re-arms"
        );
    }
}

/// THE DRAIN'S OWN DOOR SEATS (this round — javap
/// RedStoneWireBlock.updatePowerStrength the 26.2 jar the changed
/// cells' own setBlock flag-3 walk the Level.updateNeighborsAt six
/// the neighborChanged broadcast speaks, the DoorBlock the door
/// among the flips' own neighborhoods the consult answers): the
/// flipped torch cells + the wire flips' own cells the
/// [redstone_door_lane_flips] scan the drain's own beat consults —
/// the door at the NOT-gate run's tail the lever's twelve-cell
/// scope never reached the drain's own seeds re-read (the ON press
/// the stale OPEN kind keeps, the drain the torch OFF + the wire
/// drop + the door RETIRES the same beat, the second press the
/// relight the door the lit consult re-arms — the full NOT-gate
/// cycle the tail door wears).
#[test]
fn the_drain_door_seats_the_tail_door_the_not_gate_cycle_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=8 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
                w.set(c + dx, top + 2, c + dz, crate::world::AIR);
            }
        }
        // THE NOT-GATE'S OWN SEATS (the lever the pedestal's west
        // neighbor the ray reads, the torch the pedestal's top the
        // LIT default wears, the wire the torch's east neighbor the
        // support stone carries, the tail DOOR the wire's east end
        // the same ground row wears — the door BESIDE the tail
        // rung the lever's own scope never reaches)
        w.set(c + 2, top, c, crate::world::LEVER);
        w.set_variant(c + 2, top, c, 1);
        w.set(c + 3, top, c, crate::world::STONE);
        w.set(c + 3, top + 1, c, crate::world::REDSTONE_TORCH);
        w.set(c + 4, top, c, crate::world::STONE);
        w.set(c + 4, top + 1, c, crate::world::REDSTONE_WIRE);
        w.set(c + 5, top, c, crate::world::OAK_DOOR);
        w.set(c + 5, top + 1, c, crate::world::OAK_DOOR_UPPER);
    }
    // THE LIT TORCH'S OWN ARM (the attach walk's own seam the
    // placed-wire beat speaks — the component re-read the torch's
    // 15 the wire wears, the press path's own door consult the
    // tail door the ON rung's 15 opens)
    app.attach_wire_voice(c + 4, top + 1, c);
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 4, top + 1, c),
            crate::world::REDSTONE_WIRE_ON,
            "the lit torch the wire's own 15 wears"
        );
        assert_eq!(
            w.get(c + 5, top, c),
            crate::world::OAK_DOOR_OPEN,
            "the press path's own door seeds the tail door opens"
        );
        assert_eq!(
            w.get(c + 5, top + 1, c),
            crate::world::OAK_DOOR_UPPER_OPEN,
            "the mate the scan's own sync wears"
        );
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE ON PRESS (the NOT-gate's own consult the lever's scope
    // arms — the tail door the STALE open kind keeps until the
    // drain)
    assert!(app.on_right_press(0, 0), "the lever's ON press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 2, top, c),
            crate::world::LEVER_ON,
            "the lever's own kind the ON bit wears"
        );
        assert_eq!(
            w.get(c + 5, top, c),
            crate::world::OAK_DOOR_OPEN,
            "the press beat the door's stale kind keeps — the +2 consult the flip alone answers"
        );
    }
    // THE DRAIN'S OWN BEAT (the +2 consult the tick walk serves —
    // the torch the extinguish arm flips OFF, the wire re-read the
    // component's drop the same walk answers, THE NEW DOOR SEATS
    // the tail door the wire's drop the stale kind RETIRES)
    for _ in 0..3 {
        app.update();
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 3, top + 1, c),
            crate::world::REDSTONE_TORCH_OFF,
            "the NOT-gate's own verdict the torch's LIT bit drops"
        );
        assert_eq!(
            w.get(c + 4, top + 1, c),
            crate::world::REDSTONE_WIRE,
            "the dead source the wire's plain kind wears"
        );
        assert_eq!(
            w.get(c + 5, top, c),
            crate::world::OAK_DOOR,
            "the drain's own door seats the tail door the stale kind retires"
        );
        assert_eq!(
            w.get(c + 5, top + 1, c),
            crate::world::OAK_DOOR_UPPER,
            "the mate the closed kind the same sync wears"
        );
    }
    // THE SECOND PRESS (the lever's death the consult re-arms —
    // the relight the +2 beat serves, the wire's 15 the re-read
    // re-wears, the tail door the same consult the open kind
    // re-arms)
    assert!(app.on_right_press(0, 0), "the lever's OFF press consumes");
    for _ in 0..3 {
        app.update();
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 3, top + 1, c),
            crate::world::REDSTONE_TORCH,
            "the dead support the torch's LIT bit re-arms"
        );
        assert_eq!(
            w.get(c + 4, top + 1, c),
            crate::world::REDSTONE_WIRE_ON,
            "the relit torch the wire's own 15 re-wears"
        );
        assert_eq!(
            w.get(c + 5, top, c),
            crate::world::OAK_DOOR_OPEN,
            "the drain's own door seats the tail door the open kind re-arms"
        );
        assert_eq!(
            w.get(c + 5, top + 1, c),
            crate::world::OAK_DOOR_UPPER_OPEN,
            "the mate the open kind the same sync wears"
        );
    }
}

#[test]
fn the_wire_run_the_door_and_trapdoor_tails_the_scan_wears() {
    // THE DOOR LANE'S OWN WIRE SEEDS (this round — javap
    // RedStoneWireBlock.updatePowerStrength the changed cells' own
    // setBlock flag-3 the Level.updateNeighborsAt six the
    // neighborChanged broadcast speaks: the door/trapdoor/gate among
    // the flips' own neighborhoods the consult answers — the
    // lever's twelve-cell scope the wire run's own tails never
    // reached before): the ON press the east run's decay the rungs
    // wear (the tail cell the P13 kind the fixpoint answers), the
    // east door the lower's own signal the OPEN bit cycles (the
    // mate the scan's own sync wears), the west trapdoor the tail's
    // 15 the same beat opens, the OFF press the closed kinds the
    // same seeds re-answer.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -4..=6 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
                w.set(c + dx, top + 2, c + dz, crate::world::AIR);
                w.set(c + dx, top + 3, c + dz, crate::world::AIR);
            }
        }
        // THE TWO TAILS (the lever's own cell the ground answers,
        // the east run the door's lower the run's tail wears, the
        // west cell the trapdoor the floor carries — both consumers
        // the lever's own scope never reaches)
        w.set(c, top, c, crate::world::LEVER);
        w.set_variant(c, top, c, 1);
        w.set(c + 1, top, c, crate::world::REDSTONE_WIRE);
        w.set(c + 2, top, c, crate::world::REDSTONE_WIRE);
        w.set(c + 3, top, c, crate::world::REDSTONE_WIRE);
        w.set(c + 4, top, c, crate::world::OAK_DOOR);
        w.set(c + 4, top + 1, c, crate::world::OAK_DOOR_UPPER);
        w.set(c - 1, top, c, crate::world::REDSTONE_WIRE);
        w.set(c - 2, top, c, crate::world::OAK_TRAPDOOR);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE ON PRESS (the tails the flips' own seeds the scan
    // consults — the door and the trapdoor the signal's own verdict
    // opens)
    assert!(app.on_right_press(0, 0), "the lever's ON press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 4, top, c),
            crate::world::OAK_DOOR_OPEN,
            "the east door's lower the tail's P13 signal opens"
        );
        assert_eq!(
            w.get(c + 4, top + 1, c),
            crate::world::OAK_DOOR_UPPER_OPEN,
            "the mate the scan's own sync wears"
        );
        assert_eq!(
            w.get(c - 2, top, c),
            crate::world::OAK_TRAPDOOR_OPEN,
            "the west trapdoor the tail's 15 the same beat opens"
        );
    }
    // THE OFF PRESS (the signal's death the same seeds the verdicts
    // re-answer — the closed kinds the quiet keep wears)
    assert!(app.on_right_press(0, 0), "the lever's OFF press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 4, top, c),
            crate::world::OAK_DOOR,
            "the dead run the door's lower the closed kind wears"
        );
        assert_eq!(
            w.get(c + 4, top + 1, c),
            crate::world::OAK_DOOR_UPPER,
            "the mate the closed kind the same sync wears"
        );
        assert_eq!(
            w.get(c - 2, top, c),
            crate::world::OAK_TRAPDOOR,
            "the dead tail the trapdoor's closed kind wears"
        );
    }
}

#[test]
fn the_lever_ceiling_lane_the_under_hit_wears() {
    // THE CEILING LANE'S OWN UNDER HIT (this round — javap the
    // base getStateForPlacement pcs 34-58 the Y-axis arm's UP
    // verdict the CEILING face answers, the raw look the FACING
    // wears, the canSurvive the connected DOWN's UP opposite the
    // above cell reads: the up look the stone slab's underside the
    // ray hits, the lever the cell below lands, the two-field
    // byte the high nibble the ceiling face banks)
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
                w.set(c + dx, top + 3, c + dz, crate::world::STONE);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
                w.set(c + dx, top + 2, c + dz, crate::world::AIR);
            }
        }
    }
    app.offline_held_updated(Some("item/lever"));
    // THE UP LOOK'S OWN UNDER HIT (the camera under the slab the
    // pitch +1/2 the ray carries — the hit face the DOWN ordinal
    // the underside read wears, the placement cell the hit + the
    // normal the below cell speaks)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.5, c as f32 + 0.5),
        yaw: 0.0,
        pitch: std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the underside hit the ceiling lane lands"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top + 2, c),
            crate::world::LEVER,
            "the OFF kind the below-slab cell lands"
        );
        assert_eq!(
            w.variant(c, top + 2, c),
            crate::world::lever_variant(crate::world::LEVER_FACE_CEILING, 1),
            "the ceiling face the raw look's north byte wears"
        );
    }
    // THE TOGGLE'S OWN BEAT (the same aim the lever cell the ray
    // answers — the flip the kind mate wears, the variant byte
    // the ceiling face the same stamp keeps)
    assert!(app.on_right_press(0, 0), "the ON toggle consumes");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top + 2, c),
        crate::world::LEVER_ON,
        "the ON kind the +1 mate wears"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().variant(c, top + 2, c),
        crate::world::lever_variant(crate::world::LEVER_FACE_CEILING, 1),
        "the variant byte the flip keeps"
    );
}

#[test]
fn the_button_presses_pulses_and_the_wire_speaks() {
    // THE BUTTON'S OWN PRESS + PULSE WALK (the autopush-660 round
    // — javap ButtonBlock.useWithoutItem + press + tick the 26.2
    // jar): the placement the wall lane lands, the press the ON
    // kind the click-on voice + the wire voice wears (the IRON
    // door the wire ALONE swings — the hand gate never consults),
    // the re-press the CONSUME verdict banks, the 20-tick window
    // the release beat serves (the OFF kind the click-off voice +
    // the block_deactivate vibration wears, the door the silence
    // closes).
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
        // THE IRON DOOR'S OWN PAIR (the closed kinds the cell pair
        // wears — the button's own south neighbor the wire scan
        // reads, the mate's own sync the upper the same beat
        // wears; the ray's own east lane the door never blocks)
        w.set(c + 1, top, c + 1, crate::world::IRON_DOOR);
        w.set(c + 1, top + 1, c + 1, crate::world::IRON_DOOR_UPPER);
    }
    app.offline_held_updated(Some("item/stone_button"));
    // THE WALL LANE'S OWN LANDING (the east look the stone block's
    // west face the ray reads — the FACING the west byte the
    // normal banks, the two-field byte the high nibble wears)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c + 2, top, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 - 1.5, top as f32 + 0.5, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    assert!(
        app.on_right_press(0, 0),
        "the wall hit the button's wall lane lands"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::BUTTON,
            "the OFF kind the wall lane lands"
        );
        assert_eq!(
            w.variant(c + 1, top, c),
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 4),
            "the west-facing wall byte the stamp wears"
        );
    }
    // THE PRESS'S OWN BEAT (the same aim the button cell the ray
    // answers — the ON kind the click-on voice + the wire voice
    // wears: the ON button's 15 the iron door swings)
    assert!(app.on_right_press(0, 0), "the button's press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::BUTTON_ON,
            "the ON kind the press wears"
        );
        assert_eq!(
            w.get(c + 1, top, c + 1),
            crate::world::IRON_DOOR_OPEN,
            "the ON button's wire the door swings"
        );
        assert_eq!(
            w.get(c + 1, top + 1, c + 1),
            crate::world::IRON_DOOR_UPPER_OPEN,
            "the mate's own sync the upper wears"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.stone_button.click_on") && e.contains("p1.000000")),
        "the click-on voice the 1.0 pitch wears"
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("block_activate")),
        "the vibration post the sculk lane hears"
    );
    // THE CONSUME VERDICT (the re-press the ON kind the pass never
    // flips — the interaction the success return banks)
    assert!(
        app.on_right_press(0, 0),
        "the powered re-press the CONSUME wears"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 1, top, c),
        crate::world::BUTTON_ON,
        "the re-press the ON keeps"
    );
    // THE PULSE'S OWN END (the 20-tick window the release beat
    // serves — the OFF kind the click-off voice + the door the
    // silence closes)
    for _ in 0..22 {
        app.update();
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::BUTTON,
            "the OFF kind the release wears"
        );
        assert_eq!(
            w.get(c + 1, top, c + 1),
            crate::world::IRON_DOOR,
            "the door the silence closes"
        );
        assert_eq!(
            w.get(c + 1, top + 1, c + 1),
            crate::world::IRON_DOOR_UPPER,
            "the mate the silence keeps"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.stone_button.click_off") && e.contains("p1.000000")),
        "the click-off voice the walk speaks"
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("block_deactivate")),
        "the deactivate post the sculk lane hears"
    );
}

#[test]
fn the_button_pulse_the_rail_verdict_the_swap_wears() {
    // THE BUTTON'S OWN RAIL VERDICT (the dev-100 round — javap
    // press pc 21-27 the updateNeighbours walk the two seeds the
    // flag-3 broadcast speaks, ButtonBlock.tick the unpress the
    // same walk re-fires): the pressed pulse the powered rail's
    // POWERED bit the swap arms, the release beat the bit clears,
    // the shape byte the verdicts the nibble alone answer.
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
        // THE RAIL'S OWN SEAT (the button's south horizontal
        // neighbor the walk reads — the ray's east lane the rail
        // never blocks)
        w.set(c + 1, top, c + 1, crate::world::POWERED_RAIL);
        w.set_variant(c + 1, top, c + 1, 0);
        // THE WALL'S OWN STONE (the button's west face the ray
        // reads)
        w.set(c + 2, top, c, crate::world::STONE);
    }
    app.offline_held_updated(Some("item/stone_button"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 - 1.5, top as f32 + 0.5, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    assert!(
        app.on_right_press(0, 0),
        "the wall hit the button's wall lane lands"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 1, top, c),
        crate::world::BUTTON,
        "the OFF kind the wall lane lands"
    );
    // THE PRESS'S OWN BEAT (the same aim the button cell the ray
    // answers): the ON kind the pulse wears, the rail's bit the
    // swap arms
    assert!(app.on_right_press(0, 0), "the button's press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::BUTTON_ON,
            "the ON kind the press wears"
        );
        assert_eq!(
            w.variant(c + 1, top, c + 1),
            crate::world::RAIL_POWERED_BIT,
            "the pressed pulse the powered rail's bit arms"
        );
    }
    // THE PULSE'S OWN END (the 20-tick window the release beat
    // serves): the OFF kind the release wears, the rail's bit the
    // swap clears
    for _ in 0..22 {
        app.update();
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::BUTTON,
            "the OFF kind the release wears"
        );
        assert_eq!(
            w.variant(c + 1, top, c + 1),
            0,
            "the pulse's death the rail's bit clears"
        );
    }
}

#[test]
fn the_button_press_the_activator_arms_the_detector_refuses() {
    // THE TWINS THROUGH THE BUTTON SEAM (the dev-100 round — the
    // same two-seed broadcast the dev-97 law the press re-fires):
    // the pressed pulse the activator's bit the same walk arms,
    // the detector's own cart law the button's signal never arms,
    // the release beat the armed bit clears.
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
        // THE TWINS THE BUTTON'S Z-PAIR FLANKS (the ray's east
        // lane neither blocks)
        w.set(c + 1, top, c + 1, crate::world::ACTIVATOR_RAIL);
        w.set_variant(c + 1, top, c + 1, 0);
        w.set(c + 1, top, c - 1, crate::world::DETECTOR_RAIL);
        w.set_variant(c + 1, top, c - 1, 0);
        w.set(c + 2, top, c, crate::world::STONE);
    }
    app.offline_held_updated(Some("item/stone_button"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 - 1.5, top as f32 + 0.5, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    assert!(
        app.on_right_press(0, 0),
        "the wall hit the button's wall lane lands"
    );
    assert!(app.on_right_press(0, 0), "the button's press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::BUTTON_ON,
            "the ON kind the press wears"
        );
        assert_eq!(
            w.variant(c + 1, top, c + 1),
            crate::world::RAIL_POWERED_BIT,
            "the pressed pulse the activator's bit arms"
        );
        assert_eq!(
            w.variant(c + 1, top, c - 1),
            0,
            "the detector's own cart law the button's signal never arms"
        );
    }
    for _ in 0..22 {
        app.update();
    }
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .variant(c + 1, top, c + 1),
        0,
        "the pulse's death the activator's bit clears"
    );
}

#[test]
fn the_plate_press_the_rail_verdict_the_swap_wears() {
    // THE PLATE'S OWN RAIL VERDICT (the dev-101 round — javap
    // checkPressed pc 58-61 the updateNeighbours walk the two
    // seeds the press's flag-2 setBlock speaks, the vacated beat
    // the same walk re-fires): the pressed plate the powered
    // rail's POWERED bit the swap arms, the release beat the bit
    // clears, the shape byte the verdicts the nibble alone
    // answer.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (px, py, pz) = (c - 1, top, c);
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        // THE PLATE'S OWN SEAT (the floor the support keeps)
        w.set(px, py, pz, crate::world::OAK_PRESSURE_PLATE);
        // THE RAIL'S OWN SEAT (the plate's east horizontal
        // neighbor the walk reads)
        w.set(c, py, c, crate::world::POWERED_RAIL);
        w.set_variant(c, py, c, 0);
    }
    // THE PRESS'S OWN BEAT (the player's feet bbox the touch band
    // the OFF plate scans): the ON kind the press wears, the
    // rail's bit the swap arms
    app.body = Some(crate::body::PlayerBody::new(
        px as f64 + 0.5,
        py as f64,
        pz as f64 + 0.5,
        0.0,
        0.0,
    ));
    app.update();
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(px, py, pz),
            crate::world::OAK_PRESSURE_PLATE_ON,
            "the ON kind the press wears"
        );
        assert_eq!(
            w.variant(c, py, c),
            crate::world::RAIL_POWERED_BIT,
            "the pressed plate the powered rail's bit arms"
        );
    }
    // THE HOLD'S OWN BEAT (the occupied scan the due walk re-arms
    // — the bit the window the stand keeps)
    let start_frame = app.frame;
    while app.frame < start_frame + 25 {
        app.update();
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(px, py, pz),
            crate::world::OAK_PRESSURE_PLATE_ON,
            "the occupied scan the ON keeps"
        );
        assert_eq!(
            w.variant(c, py, c),
            crate::world::RAIL_POWERED_BIT,
            "the hold the armed bit keeps"
        );
    }
    // THE STEP-OFF'S OWN RELEASE (the vacated beat the OFF set +
    // the two seeds the same walk re-fires): the OFF kind the
    // release wears, the rail's bit the swap clears
    app.body = Some(crate::body::PlayerBody::new(
        (px + 2) as f64 + 0.5,
        py as f64,
        pz as f64 + 0.5,
        0.0,
        0.0,
    ));
    let start_frame = app.frame;
    while app.frame < start_frame + 25 {
        app.update();
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(px, py, pz),
            crate::world::OAK_PRESSURE_PLATE,
            "the vacated beat the OFF release wears"
        );
        assert_eq!(
            w.variant(c, py, c),
            0,
            "the release beat the rail's bit clears"
        );
    }
}

#[test]
fn the_plate_press_the_activator_arms_the_detector_refuses() {
    // THE TWINS THROUGH THE PLATE SEAM (the dev-101 round — the
    // same two-seed broadcast the dev-97 law the press re-fires):
    // the pressed plate the activator's bit the same walk arms,
    // the detector's own cart law the plate's signal never arms,
    // the release beat the armed bit clears.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (px, py, pz) = (c - 1, top, c);
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        // THE TWINS THE PLATE'S Z-PAIR FLANKS (the plate's north
        // and south horizontal neighbors the walk reads — the
        // updateNeighborsAt seed's own six the vanilla walk
        // speaks, the diagonal the scan never reads)
        w.set(px, py, pz, crate::world::OAK_PRESSURE_PLATE);
        w.set(px, py, pz - 1, crate::world::ACTIVATOR_RAIL);
        w.set_variant(px, py, pz - 1, 0);
        w.set(px, py, pz + 1, crate::world::DETECTOR_RAIL);
        w.set_variant(px, py, pz + 1, 0);
    }
    // THE PRESS'S OWN BEAT (the player the LIVING class the
    // EVERYTHING row counts): the activator's bit the same walk
    // arms, the detector's own cart law the refusal keeps
    app.body = Some(crate::body::PlayerBody::new(
        px as f64 + 0.5,
        py as f64,
        pz as f64 + 0.5,
        0.0,
        0.0,
    ));
    app.update();
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(px, py, pz),
            crate::world::OAK_PRESSURE_PLATE_ON,
            "the ON kind the press wears"
        );
        assert_eq!(
            w.variant(px, py, pz - 1),
            crate::world::RAIL_POWERED_BIT,
            "the pressed plate the activator's bit arms"
        );
        assert_eq!(
            w.variant(px, py, pz + 1),
            0,
            "the detector's own cart law the plate's signal never arms"
        );
    }
    // THE STEP-OFF'S OWN RELEASE: the armed bit the same walk
    // clears
    app.body = Some(crate::body::PlayerBody::new(
        (px + 2) as f64 + 0.5,
        py as f64,
        pz as f64 + 0.5,
        0.0,
        0.0,
    ));
    let start_frame = app.frame;
    while app.frame < start_frame + 25 {
        app.update();
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.variant(px, py, pz - 1),
            0,
            "the release beat the activator's bit clears"
        );
        assert_eq!(
            w.variant(px, py, pz + 1),
            0,
            "the detector the silence the release keeps"
        );
    }
}

#[test]
fn the_oak_button_kind_laws_the_family_and_the_windows_wear() {
    // THE WOODEN BUTTON'S OWN LAWS (the dev-47 round — javap
    // BlockSetType's static init + the Blocks factory rows the
    // 26.2 jar keeps): the OAK pair the palette's 263/264 seats
    // wear (the END_PORTAL's 262 the displaced seat the growth
    // never moves), the family fn the FOUR kinds the same base
    // serves, the wooden verdict the OAK pair the String ctor's
    // iconst_1,1,1 triple arms (the STONE pair the iconst_0 the
    // refusal keeps), the windows the OAK row's 30 the stone set's
    // 20 the lambda wears, the voices the stone/wooden rows the
    // getSound walk answers, the NAME_OF rows the ONE registry
    // entry both kinds the same name speaks.
    use mc_registry::generated::atlas;
    assert_eq!(crate::world::OAK_BUTTON, 262);
    assert_eq!(crate::world::OAK_BUTTON_ON, 263);
    assert_eq!(
        crate::world::END_PORTAL,
        510,
        "the displaced seat the weighted plates' own tail keeps"
    );
    for kind in [
        crate::world::BUTTON,
        crate::world::BUTTON_ON,
        crate::world::OAK_BUTTON,
        crate::world::OAK_BUTTON_ON,
    ] {
        assert!(
            crate::world::is_button(kind),
            "kind {kind} the family keeps"
        );
    }
    assert!(
        !crate::world::is_button(crate::world::LEVER)
            && !crate::world::is_button(crate::world::OAK_DOOR),
        "the siblings the family never wears"
    );
    assert!(
        crate::world::button_is_wooden(crate::world::OAK_BUTTON)
            && crate::world::button_is_wooden(crate::world::OAK_BUTTON_ON),
        "the oak pair the wooden verdict wears"
    );
    assert!(
        !crate::world::button_is_wooden(crate::world::BUTTON)
            && !crate::world::button_is_wooden(crate::world::BUTTON_ON),
        "the stone pair the refusal keeps"
    );
    assert!(
        crate::world::button_powered(crate::world::OAK_BUTTON_ON)
            && !crate::world::button_powered(crate::world::OAK_BUTTON),
        "the oak pair's ON bit the verdict wears"
    );
    assert_eq!(
        crate::world::button_toggled(crate::world::OAK_BUTTON),
        Some(crate::world::OAK_BUTTON_ON),
        "the OFF rung's mate the ladder wears"
    );
    assert_eq!(
        crate::world::button_toggled(crate::world::OAK_BUTTON_ON),
        Some(crate::world::OAK_BUTTON),
        "the ON rung's mate the unpress wears"
    );
    assert_eq!(
        crate::world::button_pulse_ticks(crate::world::OAK_BUTTON),
        crate::world::WOODEN_BUTTON_PULSE_TICKS,
        "the oak row's 30 the window banks"
    );
    assert_eq!(
        crate::world::button_pulse_ticks(crate::world::BUTTON),
        crate::world::BUTTON_PULSE_TICKS,
        "the stone set's 20 the lambda keeps"
    );
    assert_eq!(
        crate::world::button_click_voice(crate::world::OAK_BUTTON, true),
        "block.wooden_button.click_on",
    );
    assert_eq!(
        crate::world::button_click_voice(crate::world::OAK_BUTTON, false),
        "block.wooden_button.click_off",
    );
    assert_eq!(
        crate::world::button_click_voice(crate::world::BUTTON, true),
        "block.stone_button.click_on",
    );
    assert_eq!(
        crate::world::button_click_voice(crate::world::BUTTON_ON, false),
        "block.stone_button.click_off",
    );
    // THE NAMES' OWN ROWS (the ONE "oak_button" entry both kinds
    // the same registry law wears — the pick lane grants BOTH)
    assert_eq!(
        crate::world::NAME_OF[crate::world::OAK_BUTTON as usize],
        "oak_button"
    );
    assert_eq!(
        crate::world::NAME_OF[crate::world::OAK_BUTTON_ON as usize],
        "oak_button"
    );
    // THE PALETTE'S OWN TAIL (the generated rows the regen banks —
    // the KIND_NAMES the 316 kinds speak, the oak pair the mid seats
    // wear, the FACE_TILES row the ONE oak-planks tile the
    // #texture ref banks)
    assert_eq!(
        atlas::N_KINDS,
        779,
        "the palette the weighted plates' own rows bank"
    );
    assert_eq!(atlas::KIND_NAMES[262], "oak_button");
    assert_eq!(atlas::KIND_NAMES[263], "oak_button_pressed");
    assert_eq!(atlas::REGISTRY_NAMES[262], "oak_button");
    assert_eq!(atlas::REGISTRY_NAMES[263], "oak_button");
    let tile = atlas::FACE_TILES[263][2];
    assert_ne!(
        tile,
        atlas::TILE_NONE,
        "the oak pair's tile the #texture ref banks"
    );
    assert_eq!(
        tile,
        atlas::FACE_TILES[263][0],
        "the ONE tile all six slots wear"
    );
    // THE PLATE'S OWN BOX (javap makeShapes pc 0-45 + the
    // lambda$makeShapes$0 join the 26.2 jar — the UNPRESSED the
    // 2-px slice the support edge hugs, the PRESSED the 1-px the
    // thinner plate serves; the seats the rotateAttachFace walk's
    // own groups answer)
    let off = crate::world::button_plate_box(
        crate::world::lever_variant(crate::world::LEVER_FACE_FLOOR, 1),
        false,
    );
    assert_eq!(
        off,
        [0.3125, 0.0, 0.375, 0.6875, 0.125, 0.625],
        "the floor plate the model base's own slice wears"
    );
    let on = crate::world::button_plate_box(
        crate::world::lever_variant(crate::world::LEVER_FACE_FLOOR, 1),
        true,
    );
    assert_eq!(
        on[4] - on[1],
        1.0 / 16.0,
        "the pressed slice the 1-px keeps"
    );
    let wall_n = crate::world::button_plate_box(
        crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 1),
        false,
    );
    assert_eq!(
        (wall_n[2], wall_n[5]),
        (14.0 / 16.0, 1.0),
        "the north-facing wall plate the south edge hugs"
    );
    let wall_e = crate::world::button_plate_box(
        crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 2),
        false,
    );
    assert_eq!(
        (wall_e[0], wall_e[3]),
        (0.0, 2.0 / 16.0),
        "the east-facing wall plate the west edge hugs"
    );
    let wall_s = crate::world::button_plate_box(
        crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 3),
        false,
    );
    assert_eq!(
        (wall_s[2], wall_s[5]),
        (0.0, 2.0 / 16.0),
        "the south-facing wall plate the north edge hugs"
    );
    let wall_w = crate::world::button_plate_box(
        crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 4),
        false,
    );
    assert_eq!(
        (wall_w[0], wall_w[3]),
        (14.0 / 16.0, 1.0),
        "the west-facing wall plate the east edge hugs"
    );
    let ceil = crate::world::button_plate_box(
        crate::world::lever_variant(crate::world::LEVER_FACE_CEILING, 1),
        false,
    );
    assert_eq!(
        (ceil[1], ceil[4]),
        (14.0 / 16.0, 1.0),
        "the ceiling plate the top slice wears"
    );
    // THE ARROW'S OWN BBOX (javap EntityTypes the ARROW row's
    // sized(0.5f, 0.5f) pair — the position the bbox base centers)
    let ab = crate::world::arrow_scan_box(10.0, 5.0, 20.0);
    assert_eq!(
        ab,
        [9.75, 5.0, 19.75, 10.25, 5.5, 20.5],
        "the 0.5x0.5 box the base-centered law keeps"
    );
}

#[test]
fn the_button_ladder_the_full_family_and_the_voice_sets_wear() {
    // THE LADDER'S OWN SEATS (javap the 26.2 jar the clinit's own
    // indy→bootstrap→lambda walk banks — the twelve rungs the
    // lambda$static$217..226 + $323/$324 rows ALL `bipush 30` the
    // wooden window speaks, the POLISHED_BLACKSTONE rung the
    // lambda$static$338 row the BlockSetType.STONE + `bipush 20`
    // the stone-family law keeps): the CONTIGUOUS tail the
    // 264..287 seats wear (the OFF kinds the even, the ON the odd
    // the ±1 mate speaks), the END_PORTAL the beyond-table
    // seat the displacement law keeps (the plate ladder's own tail
    // the seat the 316 banks), the family fn the FOURTEEN
    // pairs the same base serves, the atlas tail the 316 kinds the
    // ladder rows bank, the per-rung FACE_TILES row the ONE planks
    // tile all six slots wear (the polished rung the
    // polished_blackstone master), the wood order the door
    // family's own ladder wears.
    use crate::world as w;
    use mc_registry::generated::atlas;
    let pairs = [
        (w::SPRUCE_BUTTON, w::SPRUCE_BUTTON_ON, "spruce_button", true),
        (w::BIRCH_BUTTON, w::BIRCH_BUTTON_ON, "birch_button", true),
        (w::JUNGLE_BUTTON, w::JUNGLE_BUTTON_ON, "jungle_button", true),
        (w::ACACIA_BUTTON, w::ACACIA_BUTTON_ON, "acacia_button", true),
        (
            w::DARK_OAK_BUTTON,
            w::DARK_OAK_BUTTON_ON,
            "dark_oak_button",
            true,
        ),
        (
            w::MANGROVE_BUTTON,
            w::MANGROVE_BUTTON_ON,
            "mangrove_button",
            true,
        ),
        (w::CHERRY_BUTTON, w::CHERRY_BUTTON_ON, "cherry_button", true),
        (
            w::PALE_OAK_BUTTON,
            w::PALE_OAK_BUTTON_ON,
            "pale_oak_button",
            true,
        ),
        (w::BAMBOO_BUTTON, w::BAMBOO_BUTTON_ON, "bamboo_button", true),
        (
            w::CRIMSON_BUTTON,
            w::CRIMSON_BUTTON_ON,
            "crimson_button",
            true,
        ),
        (w::WARPED_BUTTON, w::WARPED_BUTTON_ON, "warped_button", true),
        (
            w::POLISHED_BLACKSTONE_BUTTON,
            w::POLISHED_BLACKSTONE_BUTTON_ON,
            "polished_blackstone_button",
            false,
        ),
    ];
    let mut seat = 264u16;
    for (off, on, name, wooden) in pairs {
        assert_eq!(off, seat, "the OFF kind the even seat wears");
        assert_eq!(on, seat + 1, "the ON kind the +1 mate wears");
        assert!(
            w::is_button(off) && w::is_button(on),
            "the {name} rung the family keeps"
        );
        assert_eq!(
            w::button_is_wooden(off),
            wooden,
            "the set type's own triple the {name} verdict arms"
        );
        assert_eq!(
            w::button_pulse_ticks(off),
            if wooden { 30 } else { 20 },
            "the factory's own ctor arg the {name} window banks"
        );
        assert!(
            !w::button_powered(off),
            "the OFF kind the resting bit wears"
        );
        assert!(w::button_powered(on), "the ON kind the powered bit keeps");
        assert_eq!(
            w::button_toggled(off),
            Some(on),
            "the {name} mate the ±1 law speaks"
        );
        assert_eq!(w::button_toggled(on), Some(off));
        // THE NAMES' OWN ROWS (the ONE "{name}" entry both kinds the
        // same registry law speaks — the pick lane grants BOTH)
        assert_eq!(w::NAME_OF[off as usize], name);
        assert_eq!(w::NAME_OF[on as usize], name);
        // THE ATLAS'S OWN ROWS (the generated rows the regen banks —
        // the KIND_NAMES the ladder rows wear, the REGISTRY_NAMES
        // the ONE registry name both kinds speak, the FACE_TILES
        // row the ONE planks tile all six slots wear)
        assert_eq!(atlas::KIND_NAMES[off as usize], name);
        assert_eq!(atlas::KIND_NAMES[on as usize], format!("{name}_pressed"));
        assert_eq!(atlas::REGISTRY_NAMES[off as usize], name);
        assert_eq!(atlas::REGISTRY_NAMES[on as usize], name);
        let tile = atlas::FACE_TILES[off as usize][2];
        assert_ne!(tile, atlas::TILE_NONE, "the {name} tile the ref banks");
        assert_eq!(
            tile,
            atlas::FACE_TILES[off as usize][0],
            "the ONE tile all six slots wear"
        );
        assert_eq!(
            atlas::FACE_TILES[on as usize][2],
            tile,
            "the ON kind the same tile keeps"
        );
        // THE ITEM FORM THE REGISTRATION CARRIES (the generated
        // items.rs the jar's BlockItemIds rows the twelve rungs
        // carry — the placement lane the NAME_OF reverse the same
        // walk serves)
        assert_eq!(
            w::World::placeable_block(name),
            Some(off),
            "the {name} item the placement lane serves"
        );
        seat += 2;
    }
    assert_eq!(
        seat, 288,
        "the button ladder's own tail the plates' rungs succeed"
    );
    assert_eq!(
        crate::world::END_PORTAL,
        510,
        "the displaced seat the weighted plates' own tail keeps"
    );
    assert_eq!(
        atlas::N_KINDS,
        779,
        "the palette the weighted plates' own rows bank"
    );
    // THE VOICES' OWN FIVE SETS (javap BlockSetType's static init —
    // the getSound walk the buttonClickOn/Off pair serves): the
    // String-ctor woods the WOODEN defaults speak, the CHERRY full
    // ctor the CHERRY_WOOD pair, the CRIMSON/WARPED rows the
    // NETHER_WOOD pairs, the BAMBOO row the BAMBOO_WOOD pair, the
    // STONE set the stone AND polished rungs the shared rows wear
    assert_eq!(
        w::button_click_voice(w::SPRUCE_BUTTON, true),
        "block.wooden_button.click_on"
    );
    assert_eq!(
        w::button_click_voice(w::CHERRY_BUTTON, true),
        "block.cherry_wood_button.click_on"
    );
    assert_eq!(
        w::button_click_voice(w::CHERRY_BUTTON, false),
        "block.cherry_wood_button.click_off"
    );
    assert_eq!(
        w::button_click_voice(w::BAMBOO_BUTTON_ON, true),
        "block.bamboo_wood_button.click_on"
    );
    assert_eq!(
        w::button_click_voice(w::CRIMSON_BUTTON, false),
        "block.nether_wood_button.click_off"
    );
    assert_eq!(
        w::button_click_voice(w::WARPED_BUTTON_ON, true),
        "block.nether_wood_button.click_on"
    );
    assert_eq!(
        w::button_click_voice(w::POLISHED_BLACKSTONE_BUTTON, true),
        "block.stone_button.click_on"
    );
    assert_eq!(
        w::button_click_voice(w::POLISHED_BLACKSTONE_BUTTON_ON, false),
        "block.stone_button.click_off"
    );
    // THE ARROWS' OWN FAMILY VERDICTS (the entityInside gate the
    // canButtonBeActivatedByArrows read arms): the wooden rungs the
    // press keeps, the polished rung the STONE set's refusal the
    // shared set type speaks
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (bx, by, bz) = (c + 1, top, c);
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                world.set(c + dx, top - 1, c + dz, crate::world::STONE);
                world.set(c + dx, top, c + dz, crate::world::AIR);
                world.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        // THE EAST WALL (the west-facing button the support east
        // hugs — the plate the cell's east edge the scan tests)
        world.set(bx + 1, by, bz, crate::world::STONE);
        world.set(bx, by, bz, crate::world::CRIMSON_BUTTON);
        world.set_variant(
            bx,
            by,
            bz,
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 4),
        );
        // THE POLISHED SIBLING (the same wall the stone-family rung
        // keeps — the arrow the shared set type's refusal never
        // lets press)
        world.set(bx, by, bz - 1, crate::world::POLISHED_BLACKSTONE_BUTTON);
        world.set_variant(
            bx,
            by,
            bz - 1,
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 4),
        );
    }
    let (ax, ay, az) = (bx as f64 + 1.0, by as f64 + 0.4, bz as f64 + 0.5);
    assert!(
        crate::world::arrow_presses_button(
            app.offline_world.as_ref().unwrap(),
            ax,
            ay,
            az,
            bx,
            by,
            bz,
        ),
        "the crimson rung the wooden press arms"
    );
    assert!(
        !crate::world::arrow_presses_button(
            app.offline_world.as_ref().unwrap(),
            ax,
            ay,
            az,
            bx,
            by,
            bz - 1,
        ),
        "the polished rung the STONE set's refusal keeps"
    );
}

#[test]
fn the_plate_ladder_the_full_family_and_the_voice_sets_wear() {
    // THE LADDER'S OWN SEATS (javap the 26.2 jar Blocks clinit the
    // twelve plate factories the indy constants wear — the twelve
    // rungs the bsm[183..191]+$491/$492/$521 rows ALL the ONE
    // two-arg ctor shape serve (`new PressurePlateBlock(BlockSet
    // Type.X, props)`), the BlockSetType records the sensitivity
    // AND the voices carry — the wooden rows the String ctor's
    // EVERYTHING default + the full-ctor rows the iconst_1,1,1
    // triple speak, the STONE AND POLISHED_BLACKSTONE rows the
    // `iconst_1,1,0` + MOBS pair the arrows' refusal bank): the
    // CONTIGUOUS tail the 292..315 seats wear (the OFF kinds the
    // even, the ON the odd the ±1 mate speaks), the END_PORTAL the
    // 316 beyond-table seat the displacement law keeps, the family
    // fn the FOURTEEN pairs the same base serves, the atlas tail
    // the 316 kinds the ladder rows bank, the per-rung FACE_TILES
    // row the ONE planks/blackstone tile all six slots wear, the
    // wood order the door family's own ladder wears, the release
    // clock the `bipush 20` BOTH sets share.
    use crate::world as w;
    use mc_registry::generated::atlas;
    let pairs = [
        (
            w::SPRUCE_PRESSURE_PLATE,
            w::SPRUCE_PRESSURE_PLATE_ON,
            "spruce_pressure_plate",
            true,
        ),
        (
            w::BIRCH_PRESSURE_PLATE,
            w::BIRCH_PRESSURE_PLATE_ON,
            "birch_pressure_plate",
            true,
        ),
        (
            w::JUNGLE_PRESSURE_PLATE,
            w::JUNGLE_PRESSURE_PLATE_ON,
            "jungle_pressure_plate",
            true,
        ),
        (
            w::ACACIA_PRESSURE_PLATE,
            w::ACACIA_PRESSURE_PLATE_ON,
            "acacia_pressure_plate",
            true,
        ),
        (
            w::DARK_OAK_PRESSURE_PLATE,
            w::DARK_OAK_PRESSURE_PLATE_ON,
            "dark_oak_pressure_plate",
            true,
        ),
        (
            w::MANGROVE_PRESSURE_PLATE,
            w::MANGROVE_PRESSURE_PLATE_ON,
            "mangrove_pressure_plate",
            true,
        ),
        (
            w::CHERRY_PRESSURE_PLATE,
            w::CHERRY_PRESSURE_PLATE_ON,
            "cherry_pressure_plate",
            true,
        ),
        (
            w::PALE_OAK_PRESSURE_PLATE,
            w::PALE_OAK_PRESSURE_PLATE_ON,
            "pale_oak_pressure_plate",
            true,
        ),
        (
            w::BAMBOO_PRESSURE_PLATE,
            w::BAMBOO_PRESSURE_PLATE_ON,
            "bamboo_pressure_plate",
            true,
        ),
        (
            w::CRIMSON_PRESSURE_PLATE,
            w::CRIMSON_PRESSURE_PLATE_ON,
            "crimson_pressure_plate",
            true,
        ),
        (
            w::WARPED_PRESSURE_PLATE,
            w::WARPED_PRESSURE_PLATE_ON,
            "warped_pressure_plate",
            true,
        ),
        (
            w::POLISHED_BLACKSTONE_PRESSURE_PLATE,
            w::POLISHED_BLACKSTONE_PRESSURE_PLATE_ON,
            "polished_blackstone_pressure_plate",
            false,
        ),
    ];
    let mut seat = 292u16;
    for (off, on, name, wooden) in pairs {
        assert_eq!(off, seat, "the OFF kind the even seat wears");
        assert_eq!(on, seat + 1, "the ON kind the +1 mate wears");
        assert!(
            w::is_pressure_plate(off) && w::is_pressure_plate(on),
            "the {name} rung the family keeps"
        );
        assert_eq!(
            w::plate_is_wooden(off),
            wooden,
            "the set type's own sensitivity the {name} verdict arms"
        );
        assert_eq!(
            w::plate_is_wooden(on),
            wooden,
            "the ON kind the same set type keeps"
        );
        assert!(!w::plate_powered(off), "the OFF kind the resting bit wears");
        assert!(w::plate_powered(on), "the ON kind the powered bit keeps");
        assert_eq!(
            w::plate_toggled(off),
            Some(on),
            "the {name} mate the ±1 law speaks"
        );
        assert_eq!(w::plate_toggled(on), Some(off));
        // THE NAMES' OWN ROWS (the ONE "{name}" entry both kinds the
        // same registry law speaks — the pick lane grants BOTH)
        assert_eq!(w::NAME_OF[off as usize], name);
        assert_eq!(w::NAME_OF[on as usize], name);
        // THE ATLAS'S OWN ROWS (the generated rows the regen banks —
        // the KIND_NAMES the ladder rows wear, the REGISTRY_NAMES
        // the ONE registry name both kinds speak, the FACE_TILES
        // row the ONE planks/blackstone tile all six slots wear)
        assert_eq!(atlas::KIND_NAMES[off as usize], name);
        assert_eq!(atlas::KIND_NAMES[on as usize], format!("{name}_down"));
        assert_eq!(atlas::REGISTRY_NAMES[off as usize], name);
        assert_eq!(atlas::REGISTRY_NAMES[on as usize], name);
        let tile = atlas::FACE_TILES[off as usize][2];
        assert_ne!(tile, atlas::TILE_NONE, "the {name} tile the ref banks");
        assert_eq!(
            tile,
            atlas::FACE_TILES[off as usize][0],
            "the ONE tile all six slots wear"
        );
        assert_eq!(
            atlas::FACE_TILES[on as usize][2],
            tile,
            "the ON kind the same tile keeps"
        );
        // THE ITEM FORM THE REGISTRATION CARRIES (the generated
        // items.rs the jar's BlockItemIds rows the twelve rungs
        // carry — the placement lane the NAME_OF reverse the same
        // walk serves)
        assert_eq!(
            w::World::placeable_block(name),
            Some(off),
            "the {name} item the placement lane serves"
        );
        seat += 2;
    }
    assert_eq!(
        seat, 316,
        "the plate ladder's own tail the weighted rungs succeed"
    );
    assert_eq!(
        crate::world::END_PORTAL,
        510,
        "the displaced seat the weighted plates' own tail keeps"
    );
    assert_eq!(
        atlas::N_KINDS,
        779,
        "the palette the weighted plates' own rows bank"
    );
    // THE VOICES' OWN FIVE SETS (javap the BlockSetType clinit the
    // pressurePlateClickOn/Off fields the record rows bank): the
    // String-ctor woods the WOODEN defaults speak, the CHERRY full
    // ctor the CHERRY_WOOD pair pc 362-365, the CRIMSON/WARPED rows
    // the NETHER_WOOD pairs pc 459-462/508-511, the BAMBOO row the
    // BAMBOO_WOOD pair pc 573-576, the STONE set the stone AND
    // polished rungs the shared rows wear
    assert_eq!(
        w::plate_click_voice(w::SPRUCE_PRESSURE_PLATE, true),
        "block.wooden_pressure_plate.click_on"
    );
    assert_eq!(
        w::plate_click_voice(w::BIRCH_PRESSURE_PLATE_ON, false),
        "block.wooden_pressure_plate.click_off"
    );
    assert_eq!(
        w::plate_click_voice(w::CHERRY_PRESSURE_PLATE, true),
        "block.cherry_wood_pressure_plate.click_on"
    );
    assert_eq!(
        w::plate_click_voice(w::CHERRY_PRESSURE_PLATE_ON, false),
        "block.cherry_wood_pressure_plate.click_off"
    );
    assert_eq!(
        w::plate_click_voice(w::BAMBOO_PRESSURE_PLATE_ON, true),
        "block.bamboo_wood_pressure_plate.click_on"
    );
    assert_eq!(
        w::plate_click_voice(w::CRIMSON_PRESSURE_PLATE, false),
        "block.nether_wood_pressure_plate.click_off"
    );
    assert_eq!(
        w::plate_click_voice(w::WARPED_PRESSURE_PLATE_ON, true),
        "block.nether_wood_pressure_plate.click_on"
    );
    assert_eq!(
        w::plate_click_voice(w::POLISHED_BLACKSTONE_PRESSURE_PLATE, true),
        "block.stone_pressure_plate.click_on"
    );
    assert_eq!(
        w::plate_click_voice(w::POLISHED_BLACKSTONE_PRESSURE_PLATE_ON, false),
        "block.stone_pressure_plate.click_off"
    );
    // THE PARAMS' OWN ROWS (the cache's sounds.json the five
    // wooden-family rows the wooden pair's 0.8/0.7 pitches verbatim
    // bank, the stone rows the 0.6/0.5 pair the wooden verdict the
    // flag serves)
    assert_eq!(
        w::plate_voice_params(w::CHERRY_PRESSURE_PLATE, true),
        (0.3, 0.8)
    );
    assert_eq!(
        w::plate_voice_params(w::BAMBOO_PRESSURE_PLATE_ON, false),
        (0.3, 0.7)
    );
    assert_eq!(
        w::plate_voice_params(w::POLISHED_BLACKSTONE_PRESSURE_PLATE, true),
        (0.3, 0.6)
    );
    assert_eq!(
        w::plate_voice_params(w::POLISHED_BLACKSTONE_PRESSURE_PLATE_ON, false),
        (0.3, 0.5)
    );
    // THE SENSITIVITY'S OWN FAMILY VERDICTS (the getSignalStrength
    // walk the lookupswitch the classes serve — the EVERYTHING row
    // the Entity.class count arms, the MOBS row the LivingEntity
    // .class count wears): the crimson rung the arrow's own bbox
    // the touch band presses, the polished rung the MOBS row's
    // refusal the same bbox never counts
    let entity: [f64; 6] = [9.6, 64.0, 9.6, 10.1, 64.5, 10.1];
    assert!(
        w::plate_entity_presses(true, false, &entity, 10, 64, 10),
        "the crimson rung the EVERYTHING row the arrow counts"
    );
    assert!(
        !w::plate_entity_presses(false, false, &entity, 10, 64, 10),
        "the polished rung the MOBS row's refusal keeps"
    );
    assert!(
        w::plate_entity_presses(false, true, &entity, 10, 64, 10),
        "the living entity both rows count"
    );
}

#[test]
fn the_weighted_plates_the_count_law_and_the_metal_voices_wear() {
    // THE WEIGHTED PLATES' OWN LAWS (the dev-49 round — javap
    // `WeightedPressurePlateBlock` the 26.2 jar the probe
    // scripts/jar_work/weighted_plate_49.py banks): the pair-pair
    // the 316..319 tail wears, the END_PORTAL the beyond-table
    // seat the displacement keeps (the 320 the `id < N_KINDS`
    // guards the bigger tail banks), the family fn the weighted
    // rungs the gold/iron verdict arms (the wooden verdict the
    // same rungs never wear — the OWN subclass the sensitivity
    // rows never share), the Entity.class count the EVERY entity
    // arms (the MOBS row the stone set wears the weighted rows
    // never speak), the release clock the `bipush 10` OVERRIDE
    // the half-window banks (the 20-tick BOTH-sets law the
    // PressurePlateBlock rows keep), the count law the
    // getSignalStrength walk's own math speaks (the
    // `Math.min(count, maxWeight)` fold + the `Mth.ceil(min /
    // maxWeight * 15.0f)` rung — the LIGHT row the `bipush 15`
    // ctor arg banks, the HEAVY row the `sipush 150`), the metal
    // voices the block.metal_pressure_plate rows BOTH rungs share
    // (the 0.75/0.65 pitches the 0.3 volume the sounds.json
    // carries), the names/atlas rows the ONE registry name both
    // kinds speak, the per-rung FACE_TILES row the gold_block/
    // iron_block masters bank, the placement the items the pick
    // lane grants, the pass-through row the collision's Empty the
    // noCollision factories speak.
    use crate::world as w;
    use mc_registry::generated::atlas;
    assert_eq!(w::LIGHT_WEIGHTED_PRESSURE_PLATE, 316);
    assert_eq!(w::LIGHT_WEIGHTED_PRESSURE_PLATE_ON, 317);
    assert_eq!(w::HEAVY_WEIGHTED_PRESSURE_PLATE, 318);
    assert_eq!(w::HEAVY_WEIGHTED_PRESSURE_PLATE_ON, 319);
    assert_eq!(w::END_PORTAL, 510);
    assert_eq!(atlas::N_KINDS, 779, "the palette the weighted rows bank");
    for (off, on, name) in [
        (
            w::LIGHT_WEIGHTED_PRESSURE_PLATE,
            w::LIGHT_WEIGHTED_PRESSURE_PLATE_ON,
            "light_weighted_pressure_plate",
        ),
        (
            w::HEAVY_WEIGHTED_PRESSURE_PLATE,
            w::HEAVY_WEIGHTED_PRESSURE_PLATE_ON,
            "heavy_weighted_pressure_plate",
        ),
    ] {
        assert!(
            w::is_pressure_plate(off) && w::is_pressure_plate(on),
            "the {name} rung the family keeps"
        );
        assert!(
            w::plate_is_weighted(off) && w::plate_is_weighted(on),
            "the weighted verdict the gold/iron rungs arm"
        );
        assert!(
            !w::plate_is_wooden(off) && !w::plate_is_wooden(on),
            "the wooden verdict the weighted rungs never wear"
        );
        assert!(
            w::plate_counts_any_entity(off) && w::plate_counts_any_entity(on),
            "the Entity.class rows the weighted count wears"
        );
        assert!(!w::plate_powered(off), "the OFF kind the resting bit wears");
        assert!(w::plate_powered(on), "the ON kind the powered bit keeps");
        assert_eq!(
            w::plate_toggled(off),
            Some(on),
            "the ±1 mate the weighted pair wears"
        );
        assert_eq!(w::plate_toggled(on), Some(off));
        assert_eq!(
            w::plate_toggled(w::LEVER),
            None,
            "the lever the family gate refuses"
        );
        assert_eq!(w::NAME_OF[off as usize], name);
        assert_eq!(w::NAME_OF[on as usize], name);
        assert_eq!(atlas::KIND_NAMES[off as usize], name);
        assert_eq!(atlas::KIND_NAMES[on as usize], format!("{name}_down"));
        assert_eq!(atlas::REGISTRY_NAMES[off as usize], name);
        assert_eq!(atlas::REGISTRY_NAMES[on as usize], name);
        let tile = atlas::FACE_TILES[off as usize][2];
        assert_ne!(
            tile,
            atlas::TILE_NONE,
            "the {name} tile the metal master banks"
        );
        assert_eq!(
            atlas::FACE_TILES[on as usize][2],
            tile,
            "the ON kind the same tile keeps"
        );
        assert_eq!(
            w::World::placeable_block(name),
            Some(off),
            "the {name} item the placement lane serves"
        );
    }
    assert_ne!(
        atlas::FACE_TILES[w::LIGHT_WEIGHTED_PRESSURE_PLATE as usize][2],
        atlas::FACE_TILES[w::HEAVY_WEIGHTED_PRESSURE_PLATE as usize][2],
        "the gold/iron tiles the two masters wear"
    );
    // THE RELEASE WINDOW'S OWN OVERRIDE (javap the two getPressed
    // Time walks — the Base's `bipush 20` pc 0-2 the ladder rows
    // wear, the weighted body's `bipush 10` pc 0-2 the OVERRIDE
    // the gold/iron rows speak)
    assert_eq!(
        w::PLATE_PRESSED_TIME,
        20,
        "the Base's own window the ladder rows keep"
    );
    assert_eq!(
        w::plate_release_ticks(w::LIGHT_WEIGHTED_PRESSURE_PLATE),
        10,
        "the weighted rungs the half-window wear"
    );
    assert_eq!(
        w::plate_release_ticks(w::HEAVY_WEIGHTED_PRESSURE_PLATE_ON),
        10,
        "the ON kind the same window reads"
    );
    assert_eq!(
        w::plate_release_ticks(w::OAK_PRESSURE_PLATE),
        20,
        "the ladder rungs the Base's window keep"
    );
    // THE COUNT LAW'S OWN MATH (javap getSignalStrength pc 0-52 —
    // the `Math.min(count, maxWeight)` fold the `istore_3` banks,
    // the zero gate pc 21-24 the 0 the empty scan returns, the
    // `Mth.ceil((float)min / (float)maxWeight * ldc 15.0f)` pc
    // 25-50 the graded rung answers)
    assert_eq!(
        w::weighted_plate_signal(0, 15),
        0,
        "the empty scan the zero wears"
    );
    assert_eq!(
        w::weighted_plate_signal(1, 15),
        1,
        "the LIGHT row one entity one rung"
    );
    assert_eq!(
        w::weighted_plate_signal(7, 15),
        7,
        "the LIGHT row the count speaks"
    );
    assert_eq!(
        w::weighted_plate_signal(15, 15),
        15,
        "the LIGHT row fifteen full"
    );
    assert_eq!(
        w::weighted_plate_signal(200, 15),
        15,
        "the min fold the cap keeps"
    );
    assert_eq!(
        w::weighted_plate_signal(1, 150),
        1,
        "the HEAVY row one entity one rung"
    );
    assert_eq!(
        w::weighted_plate_signal(10, 150),
        1,
        "the HEAVY row ten entities one rung"
    );
    assert_eq!(
        w::weighted_plate_signal(11, 150),
        2,
        "the HEAVY row the ceil answers"
    );
    assert_eq!(
        w::weighted_plate_signal(150, 150),
        15,
        "the HEAVY row one-fifty full"
    );
    // THE METAL VOICES' OWN ROWS (javap the GOLD/IRON set types'
    // pressurePlateClickOn/Off fields the record rows bank — the
    // sounds.json the 0.75/0.65 pitches the 0.3 volume wears)
    assert_eq!(
        w::plate_click_voice(w::LIGHT_WEIGHTED_PRESSURE_PLATE, true),
        "block.metal_pressure_plate.click_on"
    );
    assert_eq!(
        w::plate_click_voice(w::LIGHT_WEIGHTED_PRESSURE_PLATE_ON, false),
        "block.metal_pressure_plate.click_off"
    );
    assert_eq!(
        w::plate_click_voice(w::HEAVY_WEIGHTED_PRESSURE_PLATE, true),
        "block.metal_pressure_plate.click_on"
    );
    assert_eq!(
        w::plate_click_voice(w::HEAVY_WEIGHTED_PRESSURE_PLATE_ON, false),
        "block.metal_pressure_plate.click_off"
    );
    assert_eq!(
        w::plate_voice_params(w::LIGHT_WEIGHTED_PRESSURE_PLATE, true),
        (0.3, 0.75)
    );
    assert_eq!(
        w::plate_voice_params(w::LIGHT_WEIGHTED_PRESSURE_PLATE_ON, false),
        (0.3, 0.65)
    );
    assert_eq!(
        w::plate_voice_params(w::HEAVY_WEIGHTED_PRESSURE_PLATE, true),
        (0.3, 0.75)
    );
    assert_eq!(
        w::plate_voice_params(w::HEAVY_WEIGHTED_PRESSURE_PLATE_ON, false),
        (0.3, 0.65)
    );
    // THE PASS-THROUGH'S OWN ROW (javap the two factories'
    // Properties chains — the noCollision() call the weighted
    // windows pc 15721/15761 speak, the ladder rows the same
    // call wear)
    assert_eq!(
        w::collision_shape(w::LIGHT_WEIGHTED_PRESSURE_PLATE),
        w::CollisionShape::Empty,
        "the weighted cell the body walks through"
    );
    assert_eq!(
        w::collision_shape(w::HEAVY_WEIGHTED_PRESSURE_PLATE_ON),
        w::CollisionShape::Empty,
        "the ON kind the same row keeps"
    );
    // THE ENTITY.CLASS ROWS' OWN ARMS (the fn-level verdict the
    // press walk composes — the arrow the non-living entity the
    // weighted row counts, the stone row the MOBS refusal keeps)
    let entity: [f64; 6] = [9.6, 64.0, 9.6, 10.1, 64.5, 10.1];
    assert!(
        w::plate_entity_presses(
            w::plate_counts_any_entity(w::LIGHT_WEIGHTED_PRESSURE_PLATE),
            false,
            &entity,
            10,
            64,
            10
        ),
        "the weighted row the arrow's own bbox counts"
    );
    assert!(
        !w::plate_entity_presses(
            w::plate_counts_any_entity(w::STONE_PRESSURE_PLATE),
            false,
            &entity,
            10,
            64,
            10
        ),
        "the stone row the arrow refuses"
    );
    // THE APP'S OWN LANE (javap entityInside pc 0-31 — the
    // player's feet bbox the touch band the OFF weighted plate
    // scans, the ON set + the metal click-on voice + the
    // vibration the apply beat serves, the half-window the
    // vacated beat the release serves)
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (px, py, pz) = (c + 2, top, c);
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                world.set(px + dx, py - 1, pz + dz, crate::world::STONE);
                world.set(px + dx, py, pz + dz, crate::world::AIR);
                world.set(px + dx, py + 1, pz + dz, crate::world::AIR);
            }
        }
        world.set(px, py, pz, w::LIGHT_WEIGHTED_PRESSURE_PLATE);
    }
    app.body = Some(crate::body::PlayerBody::new(
        px as f64 + 0.5,
        py as f64,
        pz as f64 + 0.5,
        0.0,
        0.0,
    ));
    app.update();
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(px, py, pz),
        w::LIGHT_WEIGHTED_PRESSURE_PLATE_ON,
        "the weighted plate the player's own feet presses"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.metal_pressure_plate.click_on")),
        "the metal click-on voice the row's own params wear"
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("block_activate")),
        "the activate vibration the sculk lane hears"
    );
    // THE STEP-OFF'S OWN RELEASE (the vacated beat the OFF set +
    // the metal click-off voice + the deactivate vibration the
    // HALF-window serves — the 10 ticks the weighted rows wear)
    app.body = Some(crate::body::PlayerBody::new(
        px as f64 + 3.5,
        py as f64,
        pz as f64 + 0.5,
        0.0,
        0.0,
    ));
    let start_frame = app.frame;
    while app.frame < start_frame + 10 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(px, py, pz),
        w::LIGHT_WEIGHTED_PRESSURE_PLATE,
        "the vacated beat the half-window release wears"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.metal_pressure_plate.click_off")),
        "the metal click-off voice the release beat speaks"
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("block_deactivate")),
        "the deactivate vibration the release beat wears"
    );
}

#[test]
fn the_redstone_torch_the_not_gate_consult_and_the_wire_feed_wear() {
    // THE TORCH'S OWN APP LANE (the autopush-666 round — the
    // attach broadcast's NOT-gate consult the offline world wears:
    // the lit torch the run lights, the powered floor the flip the
    // dark kind speaks, the quiet floor the lit kind restores —
    // javap RedstoneTorchBlock.neighborChanged pc 0-44 + the tick
    // walk the 26.2 jar the probe scripts/jar_work/
    // redstone_torch_666.py banks). THE 2-TICK DELAY the dev-50
    // round serves — the consult the +2 entry the queue banks, the
    // flip the drain's own beat the update walk answers (the
    // immediate verdict the schedule arm retires).
    use crate::world as w;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c, top - 1, c, w::STONE);
        world.set(c, top, c, w::REDSTONE_TORCH);
        world.set(c + 1, top, c, w::REDSTONE_WIRE);
        world.set(c + 2, top, c, w::REDSTONE_WIRE);
    }
    // THE PLACE BROADCAST'S OWN FEED (the attach walk the wire's
    // recompute the lit torch's arms serve — the run the torch's
    // light carries)
    app.attach_wire_voice(c, top, c);
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 1, top, c),
        w::REDSTONE_WIRE_ON,
        "the lit torch the run lights"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 2, top, c),
        w::REDSTONE_WIRE_P14,
        "the far cell the decay's own rung banks"
    );
    // THE NOT-GATE'S OWN FLIP (the lever ON beside the floor cell
    // the floor's own read arms — the consult the OFF kind the
    // same beat speaks, the wire recompute the dark keep serves)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c - 1, top - 1, c, w::LEVER_ON);
    }
    app.attach_wire_voice(c - 1, top - 1, c);
    // THE SCHEDULED FLIP'S OWN LANDING (the dev-50 round — the
    // consult the +2 entry the queue banks, the lit torch the
    // schedule the SAME beat keeps [the immediate verdict the
    // tick walk retires], the drain the two updates the crossing
    // answers — the tick walk the setBlock(LIT=false) walk the
    // wire's own re-read the dark keep serves)
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::REDSTONE_TORCH,
        "the pending +2 the lit kind the schedule keeps"
    );
    for _ in 0..2 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::REDSTONE_TORCH_OFF,
        "the powered floor the NOT gate flips"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 1, top, c),
        w::REDSTONE_WIRE,
        "the dark run the flip keeps"
    );
    // THE QUIET FLOOR'S OWN RESTORE (the lever OFF the floor's read
    // silences — the consult the lit kind the same beat restores,
    // the wire recompute the light re-banks)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c - 1, top - 1, c, w::LEVER);
    }
    app.attach_wire_voice(c - 1, top - 1, c);
    // THE SCHEDULED RESTORE'S OWN LANDING (the +2 entry the queue
    // banks, the drain the two updates the crossing answers — the
    // relight arm the lit kind the quiet floor restores)
    for _ in 0..2 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::REDSTONE_TORCH,
        "the quiet floor the lit kind restores"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 1, top, c),
        w::REDSTONE_WIRE_ON,
        "the light the restore re-banks"
    );
}

#[test]
fn the_redstone_wall_torch_the_consult_and_the_scan_wear() {
    // THE WALL TORCH'S OWN APP LANE (the autopush-667 round — the
    // attach broadcast's NOT-gate consult the attached cell the
    // variant decode wears: the wall torch the wall BEHIND it the
    // powered lever's own beat the dark kind speaks, the quiet
    // wall the lit kind restores — javap RedstoneWallTorchBlock
    // .hasNeighborSignal pc 0-27 + the tick walk the 26.2 jar the
    // probe scripts/jar_work/redstone_wall_torch_667.py banks).
    // THE FACING ARM'S OWN FEED (the getSignal walk's
    // direction == FACING verdict the wire arms wear — the wire in
    // the torch's own facing arm the dark keep wears, the side
    // arm the lit kind's own 15 banks)
    use crate::world as w;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let world = app.offline_world.as_mut().unwrap();
        // THE WALL THE TORCH ATTACHES TO (west — the facing the
        // AWAY-from-the-wall law wears: the torch at (c) faces
        // EAST, the attached STONE at (c-1)), the lever OFF the
        // wall's own neighbor cell (the support notification the
        // consult's own scope wears — the lever's own attached
        // block the updateNeighboursAt walk serves), the two wires
        // the SIDE arms' own verdict reads (the south and the north
        // wires the lit kind's own 15 lights, the attached wall the
        // FACING arm the self-feed refuses)
        world.set(c - 1, top, c, w::STONE);
        world.set(c, top, c, w::REDSTONE_WALL_TORCH);
        world.set_variant(c, top, c, w::WALL_TORCH_FACING_EAST);
        world.set(c, top, c + 1, w::REDSTONE_WIRE);
        world.set(c, top, c - 1, w::REDSTONE_WIRE);
        world.set(c - 2, top, c, w::LEVER);
    }
    // THE PLACE BROADCAST'S OWN VERDICT (the quiet wall the lit
    // kind keeps — the attached cell the lever OFF the unpowered
    // read answers, the wires the side arms' own verdict speaks)
    app.attach_wire_voice(c, top, c);
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::REDSTONE_WALL_TORCH,
        "the quiet wall the lit kind keeps"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c + 1),
        w::REDSTONE_WIRE_ON,
        "the south wire the side arm lights"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c - 1),
        w::REDSTONE_WIRE_ON,
        "the north wire the side arm lights"
    );
    // THE NOT-GATE'S OWN FLIP (the lever ON beside the wall the
    // wall's own read arms — the lever's own attached block the
    // wall the notification chain serves, the wall torch's seat
    // the consult's own scope reaches, the wall pair's own OFF
    // kind the same beat speaks, the side wire the dark keep
    // serves)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c - 2, top, c, w::LEVER_ON);
    }
    app.attach_wire_voice(c - 2, top, c);
    // THE SCHEDULED FLIP'S OWN LANDING (the dev-50 round — the
    // consult the +2 entry the queue banks, the drain the two
    // updates the crossing answers — the tick walk's attached-cell
    // read the OFF kind the powered wall speaks)
    for _ in 0..2 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::REDSTONE_WALL_TORCH_OFF,
        "the powered wall the NOT gate flips"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c + 1),
        w::REDSTONE_WIRE,
        "the dark run the flip keeps"
    );
    // THE QUIET WALL'S OWN RESTORE (the lever OFF the wall's read
    // silences — the consult the lit kind the same beat restores,
    // the wire recompute the light re-banks)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c - 2, top, c, w::LEVER);
    }
    app.attach_wire_voice(c - 2, top, c);
    // THE SCHEDULED RESTORE'S OWN LANDING (the +2 entry the queue
    // banks, the drain the two updates the crossing answers — the
    // relight arm the lit kind the quiet wall restores)
    for _ in 0..2 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::REDSTONE_WALL_TORCH,
        "the quiet wall the lit kind restores"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c + 1),
        w::REDSTONE_WIRE_ON,
        "the light the restore re-banks"
    );
}

#[test]
fn the_redstone_torch_the_burnout_the_voices_and_the_cooldown_wear() {
    // THE BURNOUT'S OWN APP LANE (the dev-50 round — javap
    // RedstoneTorchBlock.tick pc 118-149 + isToggledTooFrequently
    // pc 0-105 the 26.2 jar the probe scripts/jar_work/
    // redstone_torch_666.out banks: the 7 backfilled rows the
    // window keeps, the 8th extinguish the consult's +2 entry the
    // drain's own beat answers, the BURNOUT verdict the
    // levelEvent 1502 walk the five puffs + the burnout voice
    // fires, the scheduleTick(pos, this, 160) cooldown retry the
    // queue banks, the pending cooldown the re-consult's +2 the
    // dedup refuses — the relight the quiet floor the gate
    // refuses while the window's rows the burnout keeps)
    use crate::world as w;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c, top - 1, c, w::STONE);
        world.set(c, top, c, w::REDSTONE_TORCH);
        // THE 7 BACKFILLED ROWS (the window the 8th verdict reads —
        // the same pos the per-pos count arms)
        for when in 0..7u64 {
            world.torch_toggles.push((when, c, top, c));
        }
    }
    app.particles.clear();
    // THE POWERED FLOOR'S OWN CONSULT (the lever ON the +2 entry
    // the queue banks — the lit torch the schedule the SAME beat
    // keeps [the immediate verdict the tick walk retires])
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c - 1, top - 1, c, w::LEVER_ON);
    }
    app.attach_wire_voice(c - 1, top - 1, c);
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::REDSTONE_TORCH,
        "the pending +2 the lit kind the schedule keeps"
    );
    // THE BURNOUT'S OWN LANDING (the drain the two updates the
    // crossing answers — the 8th extinguish the verdict speaks)
    for _ in 0..2 {
        app.update();
    }
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(c, top, c),
            w::REDSTONE_TORCH_OFF,
            "the 8th extinguish the OFF kind speaks"
        );
        assert_eq!(
            world.torch_toggles.len(),
            8,
            "the 7 + the 8th the window banks"
        );
        assert!(
            !world.torch_ticks.is_empty(),
            "the cooldown retry the queue banks"
        );
    }
    // THE FIVE PUFFS' OWN ROW (the levelEvent 1502 walk the
    // provider banks — the shifted corner bands the SMOKE kind
    // keeps)
    assert_eq!(
        app.particles.len(),
        5,
        "the iconst_5 the burnout loop keeps"
    );
    for p in &app.particles {
        assert!(
            matches!(p.kind, crate::particles::ParticleKind::Smoke),
            "the SMOKE type the provider answers"
        );
    }
    // THE VOICE'S OWN ROW (the block.redstone_torch.burnout the
    // pitched walk speaks — the events the table's own rows bank)
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("block.redstone_torch.burnout ")),
            "the burnout voice the levelEvent row speaks"
        );
    }
    // THE COOLDOWN'S OWN SILENCE (the pending 160 the re-consult's
    // +2 the dedup refuses — the lever's own toggle the consult
    // the queue never re-arms, the relight the quiet floor the
    // gate keeps)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c - 1, top - 1, c, w::LEVER);
    }
    app.attach_wire_voice(c - 1, top - 1, c);
    for _ in 0..2 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::REDSTONE_TORCH_OFF,
        "the cooldown the OFF kind the relight gate keeps"
    );
    // THE WINDOW'S OWN EXPIRY (the 160-tick retry the trim the
    // stale rows clears — the relight the quiet floor the lit
    // kind restores)
    for _ in 0..165 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::REDSTONE_TORCH,
        "the cooled retry the lit kind restores"
    );
}

#[test]
fn the_not_gate_the_cascade_the_wire_flood_and_the_pair_wear() {
    // THE CASCADE'S OWN APP LANE (the dev-51 round — javap
    // RedstoneTorchBlock.neighborChanged pc 0-44 the 26.2 jar the
    // probe scripts/jar_work/neighbor_updater_51.txt banks: the
    // lever's own attach the wire flood the component's ON kinds
    // the sets speak, the wire's own flips the neighbor torches
    // the consult re-arms (the schedule_torch_consults walk the
    // +2 entries the queue banks), the drain the pair's own beat
    // the extinguish arms answers, the flip's own broadcast the
    // pair's second consult the same walk re-arms — the two
    // torches the wire's own verdict the SAME flood the schedule
    // wears, the drains the two beats the order the queue's own
    // sort keeps)
    use crate::world as w;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let world = app.offline_world.as_mut().unwrap();
        // THE CHAIN'S OWN SEATS (the two torches the wire run's
        // own consult the supports wear — the wire run the E-W
        // axis the dead-end's own fill arms speak, the head cell
        // the B's support the run's own tail the verdict reads,
        // the head's west face the A's support's east neighbor the
        // same fill arms. THE AUTOBPUSH-119 ROUND THE AXIS THE
        // POINTING LAW BANKS: the Z-axis run the A's support the
        // pass-through refusal keeps — the occupied N-S axis the
        // fill never arms — the X-axis run the pointing law the
        // both supports' verdicts wear)
        world.set(c, top, c, w::REDSTONE_TORCH);
        world.set(c + 1, top, c, w::REDSTONE_TORCH);
        world.set(c + 1, top - 1, c, w::REDSTONE_WIRE);
        world.set(c + 2, top - 1, c, w::REDSTONE_WIRE);
        world.set(c + 3, top - 1, c, w::REDSTONE_WIRE);
        world.set(c + 4, top - 1, c, w::LEVER_ON);
    }
    // THE LEVER'S OWN ATTACH (the wire flood the component's graded
    // kinds the sets speak — the source-adjacent cell the ON kind
    // wears, the run the decay's own rungs the -1 arm answers, the
    // cascade the B's re-consult the +2 entry the queue banks — the
    // lit B the wire's own verdict the LIT == hasNeighborSignal
    // verdict arms)
    app.attach_wire_voice(c + 4, top - 1, c);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(c + 1, top - 1, c),
            w::REDSTONE_WIRE_P13,
            "the wire flood the graded rungs the sets speak"
        );
        assert_eq!(
            world.get(c + 1, top, c),
            w::REDSTONE_TORCH,
            "the schedule the lit kind the pending keeps"
        );
        assert_eq!(
            world.torch_ticks.len(),
            1,
            "the B's own +2 entry the cascade banks"
        );
    }
    // THE PAIR'S OWN FIRST BEAT (the drain the B's flip the
    // extinguish arm serves, the flip's own broadcast the A's
    // re-consult the same walk re-arms — the wire the A's
    // support's east neighbor the verdict reads)
    for _ in 0..2 {
        app.update();
    }
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(c + 1, top, c),
            w::REDSTONE_TORCH_OFF,
            "the lit B the wire's verdict the OFF kind speaks"
        );
        assert_eq!(
            world.get(c, top, c),
            w::REDSTONE_TORCH,
            "the A's own flip the +2 beat the pending keeps"
        );
        assert_eq!(
            world.torch_ticks.len(),
            1,
            "the A's own +2 entry the flip's broadcast banks"
        );
    }
    // THE PAIR'S OWN SECOND BEAT (the drain the A's flip the
    // extinguish arm serves, the flip's own broadcast the OFF
    // pair the state-correct keep wears — the queue the quiet
    // walk never fills)
    for _ in 0..2 {
        app.update();
    }
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(c, top, c),
            w::REDSTONE_TORCH_OFF,
            "the lit A the wire's verdict the OFF kind speaks"
        );
        assert!(
            world.torch_ticks.is_empty(),
            "the OFF pair the state-correct keep the queue never fills"
        );
    }
}

#[test]
fn the_redstone_lamp_the_consumer_lane_the_wire_and_the_lever_wear() {
    // THE LAMP'S OWN APP LANE (the autopush-672 round — the
    // lever's own attach the wire flood the consumer arm the same
    // walk serves): the wire run the lever's verdict the ON kinds
    // the sets speak, the lamp the run's own tail the LIT kind the
    // verdict walk banks (the wire flips' own cells the seed set
    // wears — the lamp the flood's own reach the tail answers),
    // the lever's retirement the dead run the dark kind the same
    // walk restores.
    use crate::world as w;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c, top - 1, c, w::LEVER_ON);
        world.set(c - 1, top - 1, c, w::REDSTONE_WIRE);
        world.set(c - 2, top - 1, c, w::REDSTONE_WIRE);
        world.set(c - 3, top - 1, c, w::REDSTONE_LAMP);
    }
    // THE LEVER'S OWN ATTACH (the wire flood the ON kinds the sets
    // speak, the lamp the run's own tail the lit kind banks)
    app.attach_wire_voice(c, top - 1, c);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(c - 1, top - 1, c),
            w::REDSTONE_WIRE_ON,
            "the wire flood the ON kinds the sets speak"
        );
        assert_eq!(
            world.get(c - 3, top - 1, c),
            w::REDSTONE_LAMP_LIT,
            "the powered run the lit kind the consumer arm banks"
        );
    }
    // THE LEVER'S OWN RETIREMENT (the OFF lever the dead run the
    // dark kind the same walk restores)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c, top - 1, c, w::LEVER);
    }
    app.attach_wire_voice(c, top - 1, c);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(c - 1, top - 1, c),
            w::REDSTONE_WIRE,
            "the dead run the OFF kinds the sets speak"
        );
        assert_eq!(
            world.get(c - 3, top - 1, c),
            w::REDSTONE_LAMP,
            "the dead run the dark kind the consumer arm restores"
        );
    }
}

/// THE TRAPPED CHEST'S OWN REDSTONE LANE (the autopush-293 round —
/// javap TrappedChestBlock the 26.2 jar the verdict banks: isSignalSource
/// iconst_1 the source family arms, ownSignal pc 0-11 the
/// `Mth.clamp(ChestBlockEntity.getOpenCount(level, pos), 0, 15)`
/// viewer-count the weak walk reads [the base getSignal fold the six
/// arms the no-facing gate wears], getDirectSignal pc 0-18 the
/// dir==UP compare the asker ABOVE the chest the strong read wears
/// [the OTHER five dirs the iconst_0 the refusal keeps]). THE
/// CLIENT'S OWN BYTE (the [`w::CHEST_OPEN_BIT`] the container walk
/// stamps — the open the viewer's 1 the armed verdict keeps, the
/// close the count the 0 the dead read restores, the STALE CELL the
/// re-open's release the leak refuses, the PLAIN/copper/ender kinds
/// the ownSignal override the family never wears — the signal the
/// trapped kind alone speaks).
#[test]
fn the_trapped_chest_own_redstone_lane_wears() {
    use crate::world as w;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (cx, cy, cz) = (c, top - 1, c);
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(cx, cy, cz, w::TRAPPED_CHEST);
        // THE STRONG ARM'S OWN CONDUCTOR (the full-cube the cell
        // ABOVE the chest the strong read carries — the
        // can_support_rail verdict the gate keeps)
        world.set(cx, cy + 1, cz, w::STONE);
    }
    // THE CLOSED CHEST'S OWN DEAD READ (the count the 0 the clamp
    // banks — the weak six-arm scan the closed byte refuses, the
    // strong lane the same count the dead read keeps)
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            !crate::world::redstone_signal_at(world, cx + 1, cy, cz),
            "the closed chest the weak scan refuses"
        );
        assert!(
            !crate::world::strong_signal_conductors_near(world, cx + 1, cy + 1, cz),
            "the closed chest the strong lane refuses"
        );
    }
    // THE OPEN'S OWN ARMED READ (the container walk the byte stamps
    // — the viewer's 1 the clamp's armed verdict keeps: the six
    // neighbors the weak scan arms [the no-facing gate the fold
    // wears], the conductor above the strong lane the same count
    // banks)
    app.open_chest_for((cx, cy, cz));
    assert_eq!(app.screen, ScreenKind::Chest);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            w::chest_open_bit_of(world.variant(cx, cy, cz)),
            "the open walk the byte stamps"
        );
        assert!(
            crate::world::redstone_signal_at(world, cx + 1, cy, cz),
            "the east neighbor the weak scan arms"
        );
        assert!(
            crate::world::redstone_signal_at(world, cx - 1, cy, cz),
            "the west neighbor the no-facing gate arms"
        );
        assert!(
            crate::world::redstone_signal_at(world, cx, cy, cz + 1),
            "the south neighbor the same fold arms"
        );
        assert!(
            crate::world::redstone_signal_at(world, cx, cy, cz - 1),
            "the north neighbor the same fold arms"
        );
        assert!(
            crate::world::redstone_signal_at(world, cx, cy - 1, cz),
            "the below neighbor the same fold arms"
        );
        assert!(
            crate::world::strong_signal_conductors_near(world, cx + 1, cy + 1, cz),
            "the conductor above the chest the strong lane arms"
        );
    }
    // THE FAMILY'S OWN HONESTY + THE STALE CELL'S OWN RELEASE (the
    // plain chest the byte the family wears, the signal the
    // ownSignal override the plain kind never wears — the trapped
    // cell's byte the re-open's release the leak refuses)
    let (px, py, pz) = (cx + 3, cy, cz);
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(px, py, pz, w::CHEST);
    }
    app.open_chest_for((px, py, pz));
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            w::chest_open_bit_of(world.variant(px, py, pz)),
            "the byte the chest family wears"
        );
        assert!(
            !crate::world::redstone_signal_at(world, px + 1, py, pz),
            "the plain chest the signal never speaks"
        );
        assert!(
            !w::chest_open_bit_of(world.variant(cx, cy, cz)),
            "the stale cell the re-open's release clears"
        );
        assert!(
            !crate::world::redstone_signal_at(world, cx + 1, cy, cz),
            "the released chest the weak scan refuses"
        );
    }
    // THE ENDER'S OWN SILENCE (the shared open the byte wears, the
    // signal the family law the ender never speaks)
    let (ex, ey, ez) = (px + 3, py, pz);
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(ex, ey, ez, w::ENDER_CHEST);
    }
    app.open_chest_for((ex, ey, ez));
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            !crate::world::redstone_signal_at(world, ex + 1, ey, ez),
            "the ender chest the signal never speaks"
        );
    }
    // THE RELEASE'S OWN DEAD READ (the re-opened trapped chest the
    // close walk releases — the weak + the strong lanes the dead
    // read the release restores)
    app.open_chest_for((cx, cy, cz));
    app.close_chest_container(ScreenKind::World);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            !w::chest_open_bit_of(world.variant(cx, cy, cz)),
            "the close walk the byte releases"
        );
        assert!(
            !crate::world::redstone_signal_at(world, cx + 1, cy, cz),
            "the closed chest the weak scan refuses again"
        );
        assert!(
            !crate::world::strong_signal_conductors_near(world, cx + 1, cy + 1, cz),
            "the closed chest the strong lane refuses again"
        );
    }
}

/// THE DOUBLE CHEST'S OWN SIGNAL PAIR (the autopush-296 round — javap
/// 26.2 CompoundContainer.startOpen pc 0-20 the `container1.startOpen`
/// + `container2.startOpen` pair the BOTH-halves forward speaks —
/// stopOpen pc 0-20 the same shape the release serves, each half the
/// openersCounter.incrementOpeners the openerCountChanged block event
/// answers): the ONE open the BOTH cells the [`CHEST_OPEN_BIT`] stamps
/// — the far half's own neighbors the weak scan the pair's second
/// source arms [the opened half alone the pre-pair law the dead far
/// read kept], the conductor above the far half the strong lane the
/// same pair banks, the close walk the BOTH halves the one release
/// clears, the reverse half's open the SAME pair stamps [the connected
/// direction the walk reverses], the SINGLE type the neighbor the pair
/// never stamps, the ender the pair never banks [the shared inventory
/// the every cell binds].
#[test]
fn the_double_chest_own_signal_pair_wears() {
    use crate::world as w;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (cx, cy, cz) = (c, top - 1, c);
    // THE PAIR'S OWN FACING (the EAST-facing pair the quarter walk
    // banks: the LEFT half the cw quarter the south cell reads, the
    // RIGHT half the ccw quarter the north cell answers — the SAME
    // facing both halves wear)
    let face = 1u8;
    let (a_var, b_var) = (
        face | (w::CHEST_TYPE_LEFT << 2),
        face | (w::CHEST_TYPE_RIGHT << 2),
    );
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(cx, cy, cz, w::TRAPPED_CHEST);
        world.set_variant(cx, cy, cz, a_var);
        world.set(cx, cy, cz + 1, w::TRAPPED_CHEST);
        world.set_variant(cx, cy, cz + 1, b_var);
        // THE STRONG ARM'S OWN CONDUCTOR (the cell above the FAR half
        // — the pair's second source the strong read carries)
        world.set(cx, cy + 1, cz + 1, w::STONE);
    }
    // THE CLOSED PAIR'S OWN DEAD READ (the count the 0 the clamp
    // banks — the both halves the closed byte wears, the east seats
    // the weak scan refuses, the far strong lane the same verdict
    // keeps)
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            !crate::world::redstone_signal_at(world, cx + 1, cy, cz),
            "the closed near half the weak scan refuses"
        );
        assert!(
            !crate::world::redstone_signal_at(world, cx + 1, cy, cz + 1),
            "the closed far half the weak scan refuses"
        );
        assert!(
            !crate::world::strong_signal_conductors_near(world, cx + 1, cy + 1, cz + 1),
            "the closed far half the strong lane refuses"
        );
    }
    // THE OPEN'S OWN PAIR STAMP (the ONE open the BOTH halves the
    // CompoundContainer forward stamps — the far half's own weak
    // seat the pre-pair law never armed)
    app.open_chest_for((cx, cy, cz));
    assert_eq!(app.screen, ScreenKind::Chest);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            w::chest_open_bit_of(world.variant(cx, cy, cz)),
            "the opened half the byte stamps"
        );
        assert!(
            w::chest_open_bit_of(world.variant(cx, cy, cz + 1)),
            "the partner half the SAME open stamps"
        );
        assert!(
            crate::world::redstone_signal_at(world, cx + 1, cy, cz),
            "the opened half the weak scan arms"
        );
        assert!(
            crate::world::redstone_signal_at(world, cx + 1, cy, cz + 1),
            "the far half the pair's own weak seat arms"
        );
        assert!(
            crate::world::strong_signal_conductors_near(world, cx + 1, cy + 1, cz + 1),
            "the far half's conductor the strong lane arms"
        );
    }
    // THE PAIR'S OWN RELEASE (the one close the BOTH halves the
    // stopOpen forward clears — the dead reads the release restores)
    app.close_chest_container(ScreenKind::World);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            !w::chest_open_bit_of(world.variant(cx, cy, cz)),
            "the close walk the opened half releases"
        );
        assert!(
            !w::chest_open_bit_of(world.variant(cx, cy, cz + 1)),
            "the close walk the partner half releases"
        );
        assert!(
            !crate::world::redstone_signal_at(world, cx + 1, cy, cz),
            "the released near half the weak scan refuses"
        );
        assert!(
            !crate::world::redstone_signal_at(world, cx + 1, cy, cz + 1),
            "the released far half the weak scan refuses"
        );
        assert!(
            !crate::world::strong_signal_conductors_near(world, cx + 1, cy + 1, cz + 1),
            "the released far half the strong lane refuses"
        );
    }
    // THE REVERSE HALF'S OWN OPEN (the RIGHT half the ccw read the
    // north cell pairs — the SAME pair stamps, the close the same
    // release serves)
    app.open_chest_for((cx, cy, cz + 1));
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            w::chest_open_bit_of(world.variant(cx, cy, cz)),
            "the reverse open the near half stamps"
        );
        assert!(
            w::chest_open_bit_of(world.variant(cx, cy, cz + 1)),
            "the reverse open the far half stamps"
        );
        assert!(
            crate::world::redstone_signal_at(world, cx + 1, cy, cz),
            "the reverse pair the near weak seat arms"
        );
    }
    app.close_chest_container(ScreenKind::World);
    // THE SINGLE'S OWN REFUSAL (the SINGLE type the connected walk
    // never speaks — the adjacent same-kind cell the neighbor the
    // pair never stamps)
    let (sx, sy, sz) = (cx + 5, cy, cz);
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(sx, sy, sz, w::TRAPPED_CHEST);
        world.set_variant(sx, sy, sz, 0u8);
        world.set(sx, sy, sz + 1, w::TRAPPED_CHEST);
        world.set_variant(sx, sy, sz + 1, 0u8);
    }
    app.open_chest_for((sx, sy, sz));
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            w::chest_open_bit_of(world.variant(sx, sy, sz)),
            "the single open its own cell stamps"
        );
        assert!(
            !w::chest_open_bit_of(world.variant(sx, sy, sz + 1)),
            "the SINGLE type the neighbor never stamps"
        );
        assert!(
            crate::world::redstone_signal_at(world, sx + 1, sy, sz),
            "the single's own weak seat arms"
        );
    }
    app.close_chest_container(ScreenKind::World);
    // THE ENDER'S OWN SILENCE (the shared inventory the every cell
    // binds — the pair the typed variant never banks)
    let (ex, ey, ez) = (sx + 5, sy, sz);
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(ex, ey, ez, w::ENDER_CHEST);
        world.set_variant(ex, ey, ez, a_var);
        world.set(ex, ey, ez + 1, w::ENDER_CHEST);
        world.set_variant(ex, ey, ez + 1, b_var);
    }
    app.open_chest_for((ex, ey, ez));
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            w::chest_open_bit_of(world.variant(ex, ey, ez)),
            "the ender open its own cell stamps"
        );
        assert!(
            !w::chest_open_bit_of(world.variant(ex, ey, ez + 1)),
            "the ender the pair never banks"
        );
    }
}

/// THE TRAPPED CHEST'S OWN NEIGHBOUR RIDE (the dev-188 round — javap
/// 26.2 the evidence chain the ride answers:
/// ChestBlockEntity.startOpen/stopOpen the openersCounter
/// incrementOpeners/decrementOpeners walk serves, the
/// ContainerOpenersCounter the count change the openerCountChanged
/// hook answers, the ChestBlockEntity$1 the signalOpenCount override
/// wears, the signalOpenCount pc 0-16 the
/// `level.blockEvent(pos, block, 1, count)` walk the re-speak banks
/// — the viewer-count source the neighbors the block event re-arms:
/// the wire run the ONE-signal the first rung the ON kind the flood
/// banks [the ownSignal clamp's 1 the strength speaks, the second
/// rung the -1 decay the dark keep], the close the dead run the
/// same walk restores, the door the verdict walk the open chest
/// the hidden-door build the vanilla player banks — the consumer
/// family the chest open/close the SAME ride the button
/// press/release lane wears).
#[test]
fn the_trapped_chest_own_neighbour_ride_wears() {
    use crate::world as w;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (cx, cy, cz) = (c, top - 1, c);
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(cx, cy, cz, w::TRAPPED_CHEST);
        world.set(cx + 1, cy, cz, w::REDSTONE_WIRE);
        world.set(cx + 2, cy, cz, w::REDSTONE_WIRE);
        world.set(cx + 3, cy, cz, w::REDSTONE_LAMP);
    }
    // THE CLOSED CHEST'S OWN DEAD RUN (the count the 0 the wire the
    // OFF kinds the verdict keeps — the lamp the dark kind banks)
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(world.get(cx + 1, cy, cz), w::REDSTONE_WIRE);
        assert_eq!(world.get(cx + 3, cy, cz), w::REDSTONE_LAMP);
    }
    // THE OPEN'S OWN ARMED RIDE (the container walk the byte stamps
    // + the flood the ride speaks — the wire run the ONE-signal the
    // first rung the ON kind wears [the ownSignal viewer-count the
    // clamp's 1 the strength speaks], the second rung the -1 decay
    // the 0 the dark keep, the far lamp the run's death the dark
    // kind keeps: the chest the ride the lever's own attach the
    // same lane serves)
    app.open_chest_for((cx, cy, cz));
    assert_eq!(app.screen, ScreenKind::Chest);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(cx + 1, cy, cz),
            w::REDSTONE_WIRE_P1,
            "the open chest the run's first rung the P1 row wears"
        );
        assert_eq!(
            world.get(cx + 2, cy, cz),
            w::REDSTONE_WIRE,
            "the second rung the -1 decay the dark keep"
        );
        assert_eq!(
            world.get(cx + 3, cy, cz),
            w::REDSTONE_LAMP,
            "the far lamp the run's death the dark kind keeps"
        );
    }
    // THE CLOSE'S OWN DEAD RIDE (the release walk the byte clears +
    // the flood the ride speaks — the dead run the OFF kinds the
    // sets wear, the lamp the dark kind the consumer arm restores)
    app.close_chest_container(ScreenKind::World);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(cx + 1, cy, cz),
            w::REDSTONE_WIRE,
            "the closed chest the dead run the OFF kinds the sets speak"
        );
        assert_eq!(
            world.get(cx + 3, cy, cz),
            w::REDSTONE_LAMP,
            "the dead run the dark kind the consumer arm restores"
        );
    }
    // THE DOOR'S OWN RIDE (the open chest the door the verdict walk
    // opens — the trapped chest the hidden-door build the vanilla
    // player banks, the close the same ride the door closes)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(cx, cy, cz + 1, w::OAK_DOOR);
    }
    app.open_chest_for((cx, cy, cz));
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(cx, cy, cz + 1),
            w::OAK_DOOR_OPEN,
            "the open chest the door the verdict walk opens"
        );
    }
    app.close_chest_container(ScreenKind::World);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(cx, cy, cz + 1),
            w::OAK_DOOR,
            "the closed chest the same ride the door closes"
        );
    }
}

#[test]
fn the_oak_trapdoor_the_placement_the_toggle_and_the_consult_wear() {
    // THE TRAPDOOR'S OWN APP LANE (the autopush-668 round — the
    // placement's stamp the side-hit/vertical-hit arms the
    // getStateForPlacement walk serves, the redstone preset the
    // armed floor the OPEN kind lands, the hand toggle the cycle
    // law the ±1 seat swings, the wire consult the OPEN-sync
    // verdict the neighborChanged walk keeps — javap
    // TrapDoorBlock the 26.2 jar the probe scripts/jar_work/
    // trapdoor_668.txt banks)
    use crate::world as w;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=3 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, w::STONE);
                for dy in 0..3 {
                    w.set(c + dx, top + dy, c + dz, w::AIR);
                }
            }
        }
        // THE SIDE-HIT'S OWN WALL (the STONE at (c+2, top+1, c) —
        // the eye-level east look the west face the ray answers)
        w.set(c + 2, top + 1, c, w::STONE);
    }
    // THE TOP-FACE'S OWN PLACEMENT (the down look the floor's UP
    // face the ray answers — the placement cell the camera's own
    // floating body the obstruction never refuses [the feet box
    // the camera's own altitude the y-span never meets]): the
    // FACING the look-opposite furnace law wears (yaw 0.0 the
    // north look the SOUTH byte 3 answers), the HALF the top
    // face's BOTTOM arm keeps
    app.offline_held_updated(Some("item/oak_trapdoor"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the floor placement consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            w::OAK_TRAPDOOR,
            "the closed kind the item's own kind lands"
        );
        assert_eq!(
            w.variant(c, top, c),
            w::trapdoor_variant(w::WALL_TORCH_FACING_SOUTH, false),
            "the south-facing bottom slab the vertical arm stamps"
        );
    }
    // THE SIDE-HIT'S OWN PLACEMENT (the east look the wall's west
    // face the ray answers — the FACING the clicked face itself
    // the pc 88-97 walk keeps, the HALF the eye-level click the
    // 0.5 boundary's TOP arm answers)
    app.offline_held_updated(Some("item/oak_trapdoor"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.62, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    assert!(app.on_right_press(0, 0), "the side placement consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top + 1, c),
            w::OAK_TRAPDOOR,
            "the wall-adjacent cell the side hit claims"
        );
        assert_eq!(
            w.variant(c + 1, top + 1, c),
            w::trapdoor_variant(w::WALL_TORCH_FACING_WEST, true),
            "the west-facing top half the side-hit arm stamps"
        );
    }
    // THE REDSTONE PRESET'S OWN ARM (the lever ON beside the
    // placement cell the signal walk answers — the OPEN kind the
    // ±1 seat the placement lands — javap getStateForPlacement
    // pc 142-183 the hasNeighborSignal walk banks)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c + 1, top, c + 1, w::LEVER_ON);
    }
    app.offline_held_updated(Some("item/oak_trapdoor"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.62, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    assert!(app.on_right_press(0, 0), "the armed placement consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top + 1, c),
            w::OAK_TRAPDOOR_OPEN,
            "the live signal the OPEN kind the preset lands"
        );
        assert_eq!(
            w.variant(c + 1, top + 1, c),
            w::trapdoor_variant(w::WALL_TORCH_FACING_WEST, true),
            "the armed kind the variant byte rides"
        );
    }
    // THE HAND'S OWN TOGGLE (the down look the placed trapdoor's
    // cell the ray answers — the cycle walk the ±1 mate swings,
    // the wooden_trapdoor voices the set type's own pair carries,
    // the gameEvents the sculk lane hears — javap toggle pc 0-75
    // the 26.2 jar the BLOCK_OPEN/BLOCK_CLOSE posts speak)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the flip consumed the press");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            w::OAK_TRAPDOOR_OPEN,
            "the clicked closed kind the cycle opens"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.wooden_trapdoor.open")),
        "the open voice the flip plays"
    );
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_open @")),
        "the block_open event the flip posts"
    );
    // THE SECOND PRESS THE SWING BACK (the cycle's own tail — the
    // closed kind the voice's close event answers)
    assert!(app.on_right_press(0, 0), "the second flip consumed");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            w::OAK_TRAPDOOR,
            "the open kind the second cycle closes"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.wooden_trapdoor.close")),
        "the close voice the second flip plays"
    );
    // THE CONSULT'S OWN ARM (the lever ON beside the closed
    // trapdoor the attach broadcast arms — the OPEN-sync verdict
    // the neighborChanged walk serves, the voices the consult's
    // own loop plays; the lever OFF the dead signal the closed
    // kind the same beat restores)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c + 1, w::LEVER_ON);
    }
    app.attach_wire_voice(c, top, c + 1);
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::OAK_TRAPDOOR_OPEN,
        "the armed floor the consult opens"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.wooden_trapdoor.open")),
        "the consult's open voice the flip plays"
    );
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c + 1, w::LEVER);
    }
    app.attach_wire_voice(c, top, c + 1);
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::OAK_TRAPDOOR,
        "the dead signal the consult closes"
    );
}

#[test]
fn the_iron_copper_trapdoors_the_hand_refusal_and_the_consult_wear() {
    // THE METAL RUNGS' OWN APP LANE (the autopush-670 round — the
    // iron pair the hand gate's refusal keeps [the press the PASS
    // never consumes, the cycle never swings, the voices never
    // fire], the copper ladder the hand's PASS the cycle swings,
    // the consult's OPEN-sync the family verdict the iron rung
    // arms — javap the BlockSetType clinit the 26.2 jar the probe
    // scripts/jar_work/iron_copper_trapdoors_670.txt banks: the
    // IRON row the iconst_0 the refusal speaks, the COPPER row the
    // iconst_1 the copper ladder's PASS keeps)
    use crate::world as w;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=3 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, w::STONE);
                for dy in 0..3 {
                    w.set(c + dx, top + dy, c + dz, w::AIR);
                }
            }
        }
    }
    // THE IRON PAIR'S OWN PLACEMENT (the down look the floor's UP
    // face the ray answers — the closed kind the item's own kind
    // lands)
    app.offline_held_updated(Some("item/iron_trapdoor"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the iron floor placement consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::IRON_TRAPDOOR,
        "the closed kind the item's own kind lands"
    );
    // THE HAND'S OWN REFUSAL (the canOpenByHand gate the IRON set
    // type's iconst_0 keeps — the press the PASS never consumes,
    // the cycle never swings, the voices never fire)
    let before = app.sounds.events.len();
    assert!(!app.on_right_press(0, 0), "the iron hand toggle refuses");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::IRON_TRAPDOOR,
        "the closed kind the refusal keeps"
    );
    assert_eq!(
        app.sounds.events.len(),
        before,
        "the refusal the silence keeps"
    );
    // THE CONSULT'S OWN IRON ARM (the lever ON beside the iron
    // trapdoor the attach broadcast arms — the family verdict the
    // OPEN-sync walk serves, the iron voice the set type's pair
    // speaks; the lever OFF the dead signal the closed kind the
    // same beat restores)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c + 1, w::LEVER_ON);
    }
    app.attach_wire_voice(c, top, c + 1);
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::IRON_TRAPDOOR_OPEN,
        "the armed floor the consult opens the iron rung"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.iron_trapdoor.open")),
        "the iron open voice the consult plays"
    );
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c + 1, w::LEVER);
    }
    app.attach_wire_voice(c, top, c + 1);
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        w::IRON_TRAPDOOR,
        "the dead signal the consult closes"
    );
    // THE COPPER RUNG'S OWN HAND PASS (the waxed kind the COPPER
    // set type's iconst_1 the cycle swings — the copper_trapdoor
    // voices the set type's own pair carries)
    app.offline_held_updated(Some("item/waxed_copper_trapdoor"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 1.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the waxed copper floor placement consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 1, top, c),
        w::WAXED_COPPER_TRAPDOOR,
        "the waxed closed kind the item's own kind lands"
    );
    assert!(
        app.on_right_press(0, 0),
        "the waxed copper hand toggle passes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 1, top, c),
        w::WAXED_COPPER_TRAPDOOR_OPEN,
        "the clicked closed kind the cycle opens"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.copper_trapdoor.open")),
        "the copper open voice the flip plays"
    );
}

#[test]
fn the_pressure_plates_the_sensor_the_sensitivity_and_the_release_wear() {
    // THE PLATES' OWN LAWS (the dev-48 round — javap the 26.2 jar
    // the BasePressurePlateBlock + PressurePlateBlock walks the
    // seats/the sensor/the sensitivity/the release bank): the pair
    // seats the 288..291 tail wears, the END_PORTAL the beyond-table
    // seat the displacement keeps (the autopush-663 round the plate
    // ladder's own tail the seat the 316 banks), the family fn the
    // four kinds serves, the wooden verdict the OAK String-ctor's
    // EVERYTHING default speaks (the STONE full-ctor's MOBS row the
    // arrows' refusal keeps), the release clock the `bipush 20`
    // BOTH sets wear, the names/atlas rows the ONE registry name
    // both kinds speak, the per-rung FACE_TILES row the planks/
    // stone masters bank, the placement the items the pick lane
    // grants.
    use crate::world as w;
    use mc_registry::generated::atlas;
    assert_eq!(w::OAK_PRESSURE_PLATE, 288);
    assert_eq!(w::OAK_PRESSURE_PLATE_ON, 289);
    assert_eq!(w::STONE_PRESSURE_PLATE, 290);
    assert_eq!(w::STONE_PRESSURE_PLATE_ON, 291);
    assert_eq!(w::END_PORTAL, 510);
    assert_eq!(
        atlas::N_KINDS,
        779,
        "the palette the weighted plates' own rows bank"
    );
    for (off, on, name, wooden) in [
        (
            w::OAK_PRESSURE_PLATE,
            w::OAK_PRESSURE_PLATE_ON,
            "oak_pressure_plate",
            true,
        ),
        (
            w::STONE_PRESSURE_PLATE,
            w::STONE_PRESSURE_PLATE_ON,
            "stone_pressure_plate",
            false,
        ),
    ] {
        assert!(
            w::is_pressure_plate(off) && w::is_pressure_plate(on),
            "the {name} rung the family keeps"
        );
        assert_eq!(
            w::plate_is_wooden(off),
            wooden,
            "the set type's own sensitivity the {name} verdict arms"
        );
        assert!(!w::plate_powered(off), "the OFF kind the resting bit wears");
        assert!(w::plate_powered(on), "the ON kind the powered bit keeps");
        assert_eq!(
            w::plate_toggled(off),
            Some(on),
            "the {name} mate the ±1 law speaks"
        );
        assert_eq!(w::plate_toggled(on), Some(off));
        assert_eq!(
            w::plate_toggled(w::LEVER),
            None,
            "the lever the family gate refuses"
        );
        assert_eq!(w::NAME_OF[off as usize], name);
        assert_eq!(w::NAME_OF[on as usize], name);
        assert_eq!(atlas::KIND_NAMES[off as usize], name);
        assert_eq!(atlas::KIND_NAMES[on as usize], format!("{name}_down"));
        assert_eq!(atlas::REGISTRY_NAMES[off as usize], name);
        assert_eq!(atlas::REGISTRY_NAMES[on as usize], name);
        let tile = atlas::FACE_TILES[off as usize][2];
        assert_ne!(tile, atlas::TILE_NONE, "the {name} tile the ref banks");
        assert_eq!(
            atlas::FACE_TILES[on as usize][2],
            tile,
            "the ON kind the same tile keeps"
        );
        assert_eq!(
            w::World::placeable_block(name),
            Some(off),
            "the {name} item the placement lane serves"
        );
    }
    assert_eq!(
        w::PLATE_PRESSED_TIME,
        20,
        "the release clock BOTH sets wear"
    );
    // THE TOUCH BOX'S OWN LAW (javap the Base clinit — the TOUCH_AABB
    // the `column(14.0, 0.0, 4.0).toAabbs().getFirst()` walk banks:
    // the 14/16-wide, the 4/16-tall sensor slab the cell's floor
    // wears)
    let tb = w::plate_touch_box(3, 5, 7);
    assert_eq!(
        tb,
        [
            3.0 + 1.0 / 16.0,
            5.0,
            7.0 + 1.0 / 16.0,
            3.0 + 15.0 / 16.0,
            5.0 + 4.0 / 16.0,
            7.0 + 15.0 / 16.0,
        ],
        "the sensor slab the clinit's own constants keep"
    );
    // THE SENSITIVITY'S OWN VERDICT (the getSignalStrength walk's
    // lookupswitch the classes serve — the EVERYTHING row the
    // Entity.class count arms, the MOBS row the LivingEntity.class
    // count wears)
    assert!(
        w::plate_accepts(true, false),
        "the wooden plate the arrow counts"
    );
    assert!(
        !w::plate_accepts(false, false),
        "the stone plate the arrow refuses"
    );
    assert!(
        w::plate_accepts(true, true) && w::plate_accepts(false, true),
        "the living both sets count"
    );
    // THE VOICES' OWN FOUR ROWS (the sounds.json rows' own
    // pitch/volume the rendered output keeps)
    assert_eq!(
        w::plate_click_voice(w::OAK_PRESSURE_PLATE, true),
        "block.wooden_pressure_plate.click_on"
    );
    assert_eq!(
        w::plate_click_voice(w::OAK_PRESSURE_PLATE_ON, false),
        "block.wooden_pressure_plate.click_off"
    );
    assert_eq!(
        w::plate_click_voice(w::STONE_PRESSURE_PLATE, true),
        "block.stone_pressure_plate.click_on"
    );
    assert_eq!(
        w::plate_click_voice(w::STONE_PRESSURE_PLATE_ON, false),
        "block.stone_pressure_plate.click_off"
    );
    assert_eq!(
        w::plate_voice_params(w::OAK_PRESSURE_PLATE, true),
        (0.3, 0.8)
    );
    assert_eq!(
        w::plate_voice_params(w::OAK_PRESSURE_PLATE_ON, false),
        (0.3, 0.7)
    );
    assert_eq!(
        w::plate_voice_params(w::STONE_PRESSURE_PLATE, true),
        (0.3, 0.6)
    );
    assert_eq!(
        w::plate_voice_params(w::STONE_PRESSURE_PLATE_ON, false),
        (0.3, 0.5)
    );
    // THE APP'S OWN PRESS LANE (javap entityInside pc 0-31 — the
    // player's feet bbox the touch band the OFF plate scans, the ON
    // set + the voices + the vibration the apply beat serves)
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (px, py, pz) = (c + 2, top, c);
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                world.set(px + dx, py - 1, pz + dz, crate::world::STONE);
                world.set(px + dx, py, pz + dz, crate::world::AIR);
                world.set(px + dx, py + 1, pz + dz, crate::world::AIR);
            }
        }
        world.set(px, py, pz, w::OAK_PRESSURE_PLATE);
        world.set(px + 1, py, pz, w::STONE_PRESSURE_PLATE);
    }
    // the survival gate's own arms (the below cell's support the
    // straight-DOWN law reads)
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            w::plate_can_survive(world, px, py, pz),
            "the floor-backed plate the support keeps"
        );
        assert!(
            !w::plate_can_survive(world, px, py + 3, pz),
            "the floating plate the refusal keeps"
        );
    }
    // THE WOODEN PLATE'S OWN PRESS (the player the LIVING class the
    // EVERYTHING row counts)
    app.body = Some(crate::body::PlayerBody::new(
        px as f64 + 0.5,
        py as f64,
        pz as f64 + 0.5,
        0.0,
        0.0,
    ));
    app.update();
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(px, py, pz),
        w::OAK_PRESSURE_PLATE_ON,
        "the wooden plate the player's own feet presses"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.wooden_pressure_plate.click_on")),
        "the click-on voice the row's own params wear"
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("block_activate")),
        "the activate vibration the sculk lane hears"
    );
    // THE HOLD'S OWN BEAT (the occupied scan the due walk re-arms —
    // the ON the 25 frames the player's own stand keeps)
    let start_frame = app.frame;
    while app.frame < start_frame + 25 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(px, py, pz),
        w::OAK_PRESSURE_PLATE_ON,
        "the occupied scan the ON keeps"
    );
    // THE STEP-OFF'S OWN RELEASE (the vacated beat the OFF set + the
    // click-off voice + the deactivate vibration serves)
    app.body = Some(crate::body::PlayerBody::new(
        px as f64 + 3.5,
        py as f64,
        pz as f64 + 0.5,
        0.0,
        0.0,
    ));
    let start_frame = app.frame;
    while app.frame < start_frame + 25 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(px, py, pz),
        w::OAK_PRESSURE_PLATE,
        "the vacated beat the OFF release wears"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.wooden_pressure_plate.click_off")),
        "the click-off voice the release beat speaks"
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("block_deactivate")),
        "the deactivate vibration the release beat wears"
    );
    // THE ARROWS' OWN SENSITIVITY (the wooden plate's Entity.class
    // scan the arrow presses, the stone plate's MOBS row the same
    // arrow refuses)
    app.arrows
        .push(crate::mob::Arrow::from_spawn(&crate::mob::ArrowShot {
            id: 9001,
            x: px as f64 + 0.5,
            y: py as f64 + 0.1,
            z: pz as f64 + 0.5,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            y_rot: 0.0,
            x_rot: 0.0,
            crit: false,
            pitch: 1.0,
            seed: 7,
            owner: None,
        }));
    app.update();
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(px, py, pz),
        w::OAK_PRESSURE_PLATE_ON,
        "the arrow the wooden plate's Entity scan presses"
    );
    app.arrows.clear();
    let start_frame = app.frame;
    while app.frame < start_frame + 25 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(px, py, pz),
        w::OAK_PRESSURE_PLATE,
        "the empty scan the wooden plate releases"
    );
    app.arrows
        .push(crate::mob::Arrow::from_spawn(&crate::mob::ArrowShot {
            id: 9002,
            x: (px + 1) as f64 + 0.5,
            y: py as f64 + 0.1,
            z: pz as f64 + 0.5,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            y_rot: 0.0,
            x_rot: 0.0,
            crit: false,
            pitch: 1.0,
            seed: 9,
            owner: None,
        }));
    app.update();
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(px + 1, py, pz),
        w::STONE_PRESSURE_PLATE,
        "the stone plate the arrow the MOBS row refuses"
    );
    // THE SUPPORT'S OWN POP (the dev-46 walk the plate family
    // serves — the floor's dig the straight-DOWN gate the plate
    // pops, the kind the pop's own return carries)
    let popped = {
        let world = app.offline_world.as_mut().unwrap();
        world.set(px + 1, py - 1, pz, crate::world::AIR);
        world.attach_support_updates((px + 1, py - 1, pz))
    };
    assert_eq!(popped.len(), 1, "the support's loss the plate pops");
    assert_eq!(popped[0].0, (px + 1, py, pz));
    assert_eq!(
        popped[0].1,
        w::STONE_PRESSURE_PLATE,
        "the popped kind the OFF plate wears"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(px + 1, py, pz),
        crate::world::AIR,
        "the popped cell the AIR keeps"
    );
    // THE WIRE'S OWN SOURCE (the ON plate the hasNeighborSignal
    // walk's own six-neighbor scan arms — the doors the same walk
    // the placement's preset serves)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(px + 1, py - 1, pz, crate::world::STONE);
        world.set(px + 1, py, pz, w::OAK_PRESSURE_PLATE_ON);
    }
    assert!(
        crate::world::redstone_signal_at(app.offline_world.as_ref().unwrap(), px + 1, py + 1, pz,),
        "the ON plate the wire's own scan arms"
    );
}

#[test]
fn the_arrow_press_the_verdicts_and_the_refusals_wear() {
    // THE ARROWS' OWN VERDICTS (javap entityInside pc 0-41 +
    // checkPressed pc 0-167 the 26.2 jar): the wooden OFF cell the
    // plate-hit the press verdict arms, the cell-but-off-plate
    // miss the false keeps, the STONE set the canButtonBeActivated
    // ByArrows refusal the scan gate never lets press, the ON
    // cell the POWERED gate the early return serves, the lever
    // and the non-button cells the family gate refuses.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (bx, by, bz) = (c + 1, top, c);
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        // THE EAST WALL (the west-facing button the support east
        // hugs — the plate the cell's east edge the scan tests)
        w.set(bx + 1, by, bz, crate::world::STONE);
        w.set(bx, by, bz, crate::world::OAK_BUTTON);
        w.set_variant(
            bx,
            by,
            bz,
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 4),
        );
        // THE STONE SIBLING (the same wall the stone pair keeps —
        // the arrow the refusal the scan gate banks)
        w.set(bx, by, bz - 1, crate::world::BUTTON);
        w.set_variant(
            bx,
            by,
            bz - 1,
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 4),
        );
    }
    // THE PLATE-HIT VERDICT (the arrow AT the east face the clip
    // law freezes — the bbox [bx+0.75, bx+1.25] the 2-px plate
    // [bx+0.875, bx+1.0] the inclusive-overlap law answers)
    let (ax, ay, az) = (bx as f64 + 1.0, by as f64 + 0.4, bz as f64 + 0.5);
    assert!(
        crate::world::arrow_presses_button(
            app.offline_world.as_ref().unwrap(),
            ax,
            ay,
            az,
            bx,
            by,
            bz,
        ),
        "the plate hit the wooden press arms"
    );
    // THE PRESSED STATE'S OWN SCAN (the 1-px plate the stuck
    // arrow's bbox still answers — the re-arm the due beat serves)
    assert!(
        crate::world::arrow_overlaps_plate(
            crate::world::arrow_scan_box(ax, ay, az),
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 4),
            true,
            bx,
            by,
            bz,
        ),
        "the 1-px plate the stuck arrow's bbox answers"
    );
    // THE CELL-BUT-OFF-PLATE MISS (the arrow IN the button's cell
    // the plate's cross-axis the scan refuses — the vanilla's own
    // verdict the no-flip keeps)
    assert!(
        !crate::world::arrow_presses_button(
            app.offline_world.as_ref().unwrap(),
            bx as f64 + 0.2,
            by as f64 + 0.9,
            bz as f64 + 0.5,
            bx,
            by,
            bz,
        ),
        "the off-plate cell the miss keeps"
    );
    // THE STONE SET'S OWN REFUSAL (the same plate hit the stone
    // sibling wears — the canButtonBeActivatedByArrows gate the
    // scan never lets pass)
    assert!(
        !crate::world::arrow_presses_button(
            app.offline_world.as_ref().unwrap(),
            ax,
            ay,
            az,
            bx,
            by,
            bz - 1,
        ),
        "the stone button the arrow refusal keeps"
    );
    // THE POWERED GATE'S OWN EARLY RETURN (the ON cell the scan's
    // own verdict the re-press the pass keeps)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(bx, by, bz, crate::world::OAK_BUTTON_ON);
    }
    assert!(
        !crate::world::arrow_presses_button(
            app.offline_world.as_ref().unwrap(),
            ax,
            ay,
            az,
            bx,
            by,
            bz,
        ),
        "the ON cell the powered gate refuses"
    );
    // THE FAMILY GATE'S OWN REFUSALS (the lever and the plain
    // cells the is_button gate never lets the scan speak)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(bx, by, bz, crate::world::LEVER);
    }
    assert!(
        !crate::world::arrow_presses_button(
            app.offline_world.as_ref().unwrap(),
            ax,
            ay,
            az,
            bx,
            by,
            bz,
        ),
        "the lever cell the family gate refuses"
    );
    assert!(
        !crate::world::arrow_presses_button(
            app.offline_world.as_ref().unwrap(),
            ax,
            ay,
            az,
            bx,
            by + 2,
            bz,
        ),
        "the plain cell the family gate refuses"
    );
}

#[test]
fn the_arrow_press_the_pulse_and_the_rearm_wear() {
    // THE PULSE'S OWN WALK (javap checkPressed pc 79-167 the 26.2
    // jar — the flip + the updateNeighbours + the playSound + the
    // gameEvent + the scheduleTick re-arm the offline queue's own
    // beats wear): the stuck arrow the wooden button the inside
    // walk presses (the 30-tick window the schedule banks), the
    // due beat the re-arm consult the ON keeps (no flip, no
    // click-off, the same window re-scheduled), the despawned
    // arrow the empty scan the OFF release speaks (the click-off
    // voice + the deactivate post the walk wears).
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (bx, by, bz) = (c + 1, top, c);
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        w.set(bx + 1, by, bz, crate::world::STONE);
        w.set(bx, by, bz, crate::world::OAK_BUTTON);
        w.set_variant(
            bx,
            by,
            bz,
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 4),
        );
    }
    // THE STUCK ARROW'S OWN POSE (the east face the clip law
    // freezes — the in_ground flag the flight walk's stuck branch
    // keeps, the zero velocity the gravity the stuck lane never
    // answers)
    app.arrows
        .push(crate::mob::Arrow::from_spawn(&crate::mob::ArrowShot {
            id: 7001,
            x: bx as f64 + 1.0,
            y: by as f64 + 0.4,
            z: bz as f64 + 0.5,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            y_rot: 90.0,
            x_rot: 0.0,
            crit: false,
            pitch: 1.0,
            seed: 3,
            owner: None,
        }));
    if let Some(a) = app.arrows.first_mut() {
        a.in_ground = true;
        a.life = 10;
    }
    // THE PRESS'S OWN BEAT (the inside walk the plate scan the
    // ON kind the apply serves — the click-on voice + the wire
    // voice + the 30-tick window the press arm wears)
    app.update();
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(bx, by, bz),
        crate::world::OAK_BUTTON_ON,
        "the stuck arrow the wooden press arms"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.wooden_button.click_on")),
        "the wooden click-on voice the press speaks"
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("block_activate")),
        "the activate post the press wears"
    );
    let seat = app
        .falling_checks
        .iter()
        .find(|s| s.0 == bx && s.1 == by && s.2 == bz)
        .map(|s| s.3);
    assert!(seat.is_some(), "the 30-tick window the schedule banks");
    // THE RE-ARM'S OWN BEAT (the due tick the stuck arrow the
    // scan answers — the re-schedule the same window wears, the
    // NO-flip verdict the byte-identical state keeps, no click-off
    // voice the silence keeps)
    for _ in 0..31 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(bx, by, bz),
        crate::world::OAK_BUTTON_ON,
        "the stuck arrow the re-arm the ON keeps"
    );
    let seat2 = app
        .falling_checks
        .iter()
        .find(|s| s.0 == bx && s.1 == by && s.2 == bz)
        .map(|s| s.3);
    assert!(
        seat2.is_some_and(|due| due > app.frame.saturating_sub(1)),
        "the re-arm the same window re-schedules"
    );
    let click_offs = app
        .sounds
        .events
        .iter()
        .filter(|e| e.starts_with("block.wooden_button.click_off"))
        .count();
    assert_eq!(click_offs, 0, "the stuck arrow no release speaks");
    // THE DESPAWN'S OWN SILENCE (the arrow gone the next due beat
    // the empty scan answers — the OFF kind the click-off voice +
    // the deactivate post wear)
    app.arrows.clear();
    for _ in 0..32 {
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(bx, by, bz),
        crate::world::OAK_BUTTON,
        "the empty scan the OFF kind wears"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.wooden_button.click_off")),
        "the wooden click-off voice the release speaks"
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("block_deactivate")),
        "the deactivate post the release wears"
    );
}

#[test]
fn the_fire_burnout_the_pop_lanes_speak_wear() {
    // THE BURN'S OWN POP WALK (javap FireBlock.checkBurnOut the
    // removeBlock flag-3 broadcast the 26.2 jar keeps — the attach
    // pair's own verdict the burn arms serve): the leaves the fire
    // ate the floor lever the support's loss pops, the popped
    // cells the walk the dig lane's helper shares speaks — the
    // lever item the dropResources roll grants, the block_destroy
    // vibration the sensors hear, the AIR cell the remesh speaks.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (fx, fy, fz) = (c, top, c);
    let (bx, by, bz) = (c + 1, top, c);
    let (lx, ly, lz) = (c + 1, top + 1, c);
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
                w.set(c + dx, top + 2, c + dz, crate::world::AIR);
            }
        }
        w.set(bx, by, bz, crate::world::LEAVES);
        w.set(lx, ly, lz, crate::world::LEVER);
        w.set_variant(
            lx,
            ly,
            lz,
            crate::world::lever_variant(crate::world::LEVER_FACE_FLOOR, 1),
        );
        w.set(fx, fy, fz, crate::world::FIRE);
        w.fire_age.insert((fx, fy, fz), 0);
        // THE ONPLACE LAW'S OWN FIRST SCHEDULE (the placement arm's
        // own shape — the age-0 entry + the 30..=39 due)
        let mut rng = crate::drops::Lcg::new(0xF19E_119);
        w.schedule_fire_tick(
            app.frame + crate::world::fire_tick_delay(&mut rng),
            fx,
            fy,
            fz,
        );
    }
    // THE BURN'S OWN LOOP (the leaves the 60-odds roll the fire
    // eats — the pop the walk's own verdict the frames serve)
    let mut saw_pop_frame = false;
    for _ in 0..400 {
        app.update();
        if app
            .offline_world
            .as_ref()
            .map(|w| w.get(lx, ly, lz))
            .unwrap_or(crate::world::LEVER)
            == crate::world::AIR
        {
            saw_pop_frame = true;
            break;
        }
    }
    assert!(saw_pop_frame, "the lever the burn's pop the walk wears");
    let lever_item = mc_registry::item_id_by_name("lever").unwrap();
    assert!(
        app.drops.iter().any(|d| d.item == lever_item),
        "the lever item the dropResources roll grants"
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("block_destroy")),
        "the block_destroy post the sensors hear"
    );
}

#[test]
fn the_lever_gates_and_the_door_preset_wear() {
    // THE GATES' OWN WALK (the dev-45 round the FLOOR lane banked,
    // this round the WALL lane the side hit serves — the wall hit
    // the lever's wall lane lands, the floating lever the
    // canSurvive verdict refuses, the POWERED preset the
    // getStateForPlacement's own flag arms: the door placed beside
    // an ON lever lands SWUNG, the silent ground the closed kinds
    // the item's own kinds wear)
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
    }
    app.offline_held_updated(Some("item/lever"));
    // THE WALL LANE'S OWN LANDING (this round — javap the base
    // getStateForPlacement pcs 82-112 the horizontal arm the hit
    // face's normal answers: the east look the stone pillar's
    // west face the ray reads, the placement cell the hit + the
    // normal speaks, the FACING the WEST byte the getOpposite()
    // verdict banks, the variant the two-field stamp the high
    // nibble wears, the survival the stone supporter the east
    // cell answers)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c + 2, top, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 - 1.5, top as f32 + 0.5, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    assert!(
        app.on_right_press(0, 0),
        "the wall hit the lever's wall lane lands"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::LEVER,
            "the OFF kind the wall lane lands"
        );
        assert_eq!(
            w.variant(c + 1, top, c),
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 4),
            "the west-facing wall byte the stamp wears"
        );
    }
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c + 1, top, c, crate::world::AIR);
    }
    // THE FLOOR'S OWN PLACEMENT + THE DOOR'S WIRE PRESET (the ON
    // lever the stone floor banks, the oak door placed beside the
    // door's own cell the signal walk reads — the open kinds BOTH
    // halves wear, the facing byte the stamp keeps)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the floor lever the stone banks");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::LEVER,
        "the OFF kind the item's own kind lands"
    );
    // THE ON BEAT (the same aim the lever cell the ray answers —
    // the toggle arm the ON kind the +1 mate wears)
    assert!(app.on_right_press(0, 0), "the ON toggle consumes");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::LEVER_ON,
        "the ON kind the +1 mate wears"
    );
    // THE DOOR'S WIRE PRESET (the held walk the oak item speaks —
    // the west look the lever's own east cell the placement
    // claims; the placement's own cell the signal walk reads, the
    // armed verdict the OPEN kinds BOTH halves wear, the facing
    // byte the yaw stamp banks)
    app.offline_held_updated(Some("item/oak_door"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 1.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the door's wire preset the placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c + 1),
            crate::world::OAK_DOOR_OPEN,
            "the armed cell the open kind wears"
        );
        assert_eq!(
            w.get(c, top + 1, c + 1),
            crate::world::OAK_DOOR_UPPER_OPEN,
            "the open upper the same +1 seat banks"
        );
    }
    // THE TOGGLE'S OWN CLOSE (the lever OFF the scan re-reads —
    // the preset's own kinds the walk's verdict the closed kinds
    // the flag's false arm wears — the same wire law the toggle
    // test pins, the preset the door's own start keeps)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the OFF toggle consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c + 1),
            crate::world::OAK_DOOR,
            "the dead signal the door closes"
        );
        assert_eq!(
            w.get(c, top + 1, c + 1),
            crate::world::OAK_DOOR_UPPER,
            "the upper the same beat closes"
        );
    }
}

#[test]
fn the_door_the_flip_the_both_halves_and_the_voice_wear() {
    // THE DOOR'S OWN FLIP WALK (javap DoorBlock.useWithoutItem the
    // 26.2 jar — the `state.cycle(OPEN)` pc 14-21 the clicked
    // half's own swap speaks, the OTHER half's OPEN the
    // updateShape Y-arm's own return syncs, the playSound
    // pc 34-54 the doorOpen/doorClose events the BlockSetType's
    // own pair carries, the gameEvent pc 55-78 the
    // BLOCK_OPEN/BLOCK_CLOSE posts the sculk lane hears; the
    // shift gate the door never wears — the vanilla's own use
    // walk the isShiftKeyDown read never speaks)
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
        w.set(c, top, c, crate::world::OAK_DOOR);
        w.set(c, top + 1, c, crate::world::OAK_DOOR_UPPER);
        w.set_variant(c, top, c, 1);
        w.set_variant(c, top + 1, c, 1);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE EMPTY HAND'S OWN FLIP (the door's use the held stack
    // never demands — the vanilla's own useWithoutItem walk the
    // EMPTY main hand answers)
    assert!(app.on_right_press(0, 0), "the flip consumed the press");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::OAK_DOOR_OPEN,
            "the clicked lower's own cycle the open kind wears"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::OAK_DOOR_UPPER_OPEN,
            "the other half's sync the updateShape Y-arm answers"
        );
    }
    // THE VOICE'S OWN LOG (the BlockSetType's doorOpen event the
    // runtime sounds.json carries — the wooden open the file pick
    // banks)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.wooden_door.open")),
        "the open voice the flip plays"
    );
    // THE GAME EVENT'S OWN POST (the BLOCK_OPEN row the vibration
    // frequency table banks — the door cell's own center speaks)
    assert!(
        app.game_events
            .iter()
            .any(|l| l.starts_with("block_open @")),
        "the block_open event the flip posts"
    );
    // THE SECOND PRESS THE SWING BACK (the cycle's own tail — the
    // closed pair the voice's close event answers)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the second flip consumed");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::OAK_DOOR,
            "the open lower the second cycle closes"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::OAK_DOOR_UPPER,
            "the open upper the sync closes"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.wooden_door.close")),
        "the close voice the second flip plays"
    );
}

#[test]
fn the_door_the_dig_the_mate_and_the_lower_drop_wear() {
    // THE DOOR'S OWN MATE LAW (javap DoorBlock the 26.2 jar — the
    // broken half the OTHER half's removal wears: the UPPER's dig
    // the DoublePlantBlock.preventDropFromBottomPart walk speaks,
    // the LOWER's dig the updateShape Y-arm's own AIR the silent
    // keep wears; the loot's own conditional pool the LOWER's
    // break alone rolls — the jar's oak_door.json the
    // block_state_property half=lower condition speaks, the UPPER's
    // own break the condition refuses)
    let door_item =
        mc_registry::item_id_by_name("oak_door").expect("the door item the registry banks");
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
        w.set(c, top, c, crate::world::OAK_DOOR);
        w.set(c, top + 1, c, crate::world::OAK_DOOR_UPPER);
    }
    app.offline_camera_updated(crate::camera::Camera {
        // THE LOWER'S OWN AIM (the straight-down ray the UPPER hits
        // first — the eye at the lower's own band the WEST look the
        // lower's east face answers: the upper the ray never
        // touches)
        pos: crate::math::Vec3::new(c as f32 + 3.5, top as f32 + 0.5, c as f32 + 0.5),
        yaw: -std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    // THE LOWER'S DIG (the aim the lower cell the level ray hits,
    // the ticks mine the hardness 3.0 the bare hand walks)
    app.on_press(0, 0);
    let mut ticks = 0;
    while app.offline_world.as_ref().unwrap().get(c, top, c) != crate::world::AIR {
        app.update();
        ticks += 1;
        assert!(ticks < 400, "the lower's dig completed in time");
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::AIR,
            "the upper half the mate's own law clears"
        );
    }
    assert!(
        app.drops
            .iter()
            .any(|d| d.item == door_item && d.count == 1),
        "the conditional pool the LOWER's break rolls"
    );
    // THE UPPER'S DIG (the fresh pair the aim's upper cell hits —
    // the mate's silent removal the walk serves, the drop the
    // condition refuses)
    let drops_before = app.drops.len();
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c + 2, top, c + 2, crate::world::OAK_DOOR);
        w.set(c + 2, top + 1, c + 2, crate::world::OAK_DOOR_UPPER);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, top as f32 + 5.0, c as f32 + 2.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    app.on_press(0, 0);
    let mut ticks2 = 0;
    while app
        .offline_world
        .as_ref()
        .unwrap()
        .get(c + 2, top + 1, c + 2)
        != crate::world::AIR
    {
        app.update();
        ticks2 += 1;
        assert!(ticks2 < 400, "the upper's dig completed in time");
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 2, top, c + 2),
        crate::world::AIR,
        "the lower half the silent mate removal wears"
    );
    assert_eq!(
        app.drops.len(),
        drops_before,
        "the UPPER's break the conditional pool refuses"
    );
}

#[test]
fn the_door_family_the_wood_walk_wears() {
    // THE WOOD LADDER'S OWN WALK (the dev-43 round — the eleven
    // wooden quartets the oak lane's own interaction generalizes:
    // the placement the placeable lookup + the door_upper_mate
    // the spruce pair banks, the flip the BOTH halves' kind swaps
    // the quartet arithmetic wears, the voice the per-wood set
    // type's own pair speaks [the cherry full ctor's
    // CHERRY_WOOD_DOOR_OPEN the block.cherry_wood_door.open
    // event carries, the crimson NETHER_WOOD pair the
    // block.nether_wood_door.open event carries — the wooden
    // family the single-arg ctor's WOODEN pair keeps], the dig's
    // conditional pool the {wood}_door item the shared NAME_OF
    // base row rolls)
    let spruce_item = mc_registry::item_id_by_name("spruce_door")
        .expect("the spruce door item the registry banks");
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE SPRUCE PLACEMENT (the item's own place the two halves
    // wears: the closed lower the clicked cell banks, the UPPER
    // the setPlacedBy walk's above cell keeps, the facing byte
    // BOTH cells wear)
    app.offline_held_updated(Some("item/spruce_door"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the spruce placement consumed");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::SPRUCE_DOOR,
            "the spruce closed lower the item's own place wears"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::SPRUCE_DOOR_UPPER,
            "the spruce upper the setPlacedBy walk banks"
        );
        assert_eq!(
            w.variant(c, top, c),
            1,
            "the spruce lower's facing byte the yaw-0 north wears"
        );
    }
    // THE SPRUCE FLIP (the empty hand's use the quartet arithmetic
    // the both halves swap, the voice the WOODEN pair the
    // single-arg ctor's delegation keeps)
    app.offline_held_updated(None);
    assert!(app.on_right_press(0, 0), "the spruce flip consumed");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::SPRUCE_DOOR_OPEN,
            "the spruce lower's own cycle the open kind wears"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::SPRUCE_DOOR_UPPER_OPEN,
            "the spruce other half's sync the Y-arm answers"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.wooden_door.open")),
        "the spruce voice the WOODEN pair keeps"
    );
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.cherry_wood_door")),
        "the wooden family the cherry voice never wears"
    );
    // THE SPRUCE LOWER'S DIG (the conditional pool the
    // {wood}_door item rolls — the shared NAME_OF base row the
    // per-wood drop keys, the upper the mate law clears)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 3.5, top as f32 + 0.5, c as f32 + 0.5),
        yaw: -std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    app.on_press(0, 0);
    let mut ticks = 0;
    while app.offline_world.as_ref().unwrap().get(c, top, c) != crate::world::AIR {
        app.update();
        ticks += 1;
        assert!(ticks < 400, "the spruce lower's dig completed in time");
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::AIR,
            "the spruce upper the mate's own law clears"
        );
    }
    assert!(
        app.drops
            .iter()
            .any(|d| d.item == spruce_item && d.count == 1),
        "the spruce pool the LOWER's break rolls"
    );
    // THE PER-WOOD VOICES' OWN STEMS (the cherry pair the full
    // ctor's CHERRY_WOOD sounds bank, the crimson pair the
    // NETHER_WOOD sounds keep — the flip walk the same swap the
    // voices' own events wear)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c + 2, top, c + 2, crate::world::CHERRY_DOOR);
        w.set(c + 2, top + 1, c + 2, crate::world::CHERRY_DOOR_UPPER);
        w.set(c - 2, top, c - 2, crate::world::CRIMSON_DOOR);
        w.set(c - 2, top + 1, c - 2, crate::world::CRIMSON_DOOR_UPPER);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, top as f32 + 4.0, c as f32 + 2.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the cherry flip consumed");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 2, top, c + 2),
            crate::world::CHERRY_DOOR_OPEN,
            "the cherry lower's own cycle the open kind wears"
        );
        assert_eq!(
            w.get(c + 2, top + 1, c + 2),
            crate::world::CHERRY_DOOR_UPPER_OPEN,
            "the cherry upper's sync the Y-arm answers"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.cherry_wood_door.open")),
        "the cherry voice the CHERRY_WOOD pair speaks"
    );
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 - 1.5, top as f32 + 4.0, c as f32 - 1.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the crimson flip consumed");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c - 2, top, c - 2),
            crate::world::CRIMSON_DOOR_OPEN,
            "the crimson lower's own cycle the open kind wears"
        );
        assert_eq!(
            w.get(c - 2, top + 1, c - 2),
            crate::world::CRIMSON_DOOR_UPPER_OPEN,
            "the crimson upper's sync the Y-arm answers"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.nether_wood_door.open")),
        "the crimson voice the NETHER_WOOD pair speaks"
    );
}

#[test]
fn the_door_the_iron_refusal_and_the_copper_voice_wear() {
    // THE IRON DOOR'S OWN REFUSAL (javap DoorBlock.useWithoutItem
    // pc 0-13 the 26.2 jar — the canOpenByHand() read the PASS the
    // BlockSetType.IRON's iconst_0 field keeps: the hand press the
    // flip never arms, both halves the kinds the refusal keeps,
    // the voice the walk never plays; the wire/redstone echo the
    // wired lane the kind swaps serves). THE COPPER LADDER'S OWN
    // PASS (the iconst_1 the set type carries — the hand press
    // the both-halves swap the block.copper_door.open voice
    // speaks)
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -3..=3 {
            for dz in -3..=3 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        w.set(c, top, c, crate::world::IRON_DOOR);
        w.set(c, top + 1, c, crate::world::IRON_DOOR_UPPER);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let consumed = app.on_right_press(0, 0);
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::IRON_DOOR,
            "the iron lower the refusal keeps ({consumed})"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::IRON_DOOR_UPPER,
            "the iron upper the refusal keeps"
        );
    }
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.iron_door")),
        "the iron voice the refusal never plays"
    );
    assert!(
        !app.game_events
            .iter()
            .any(|l| l.starts_with("block_open @")),
        "the iron press the vibration never posts"
    );
    // THE COPPER DOOR'S OWN FLIP (the iconst_1 the set type
    // carries — the both halves the quartet arithmetic swaps, the
    // block.copper_door.open voice the COPPER pair plays)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c + 2, top, c + 2, crate::world::COPPER_DOOR);
        w.set(c + 2, top + 1, c + 2, crate::world::COPPER_DOOR_UPPER);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, top as f32 + 4.0, c as f32 + 2.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the copper flip consumed");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 2, top, c + 2),
            crate::world::COPPER_DOOR_OPEN,
            "the copper lower's own cycle the open kind wears"
        );
        assert_eq!(
            w.get(c + 2, top + 1, c + 2),
            crate::world::COPPER_DOOR_UPPER_OPEN,
            "the copper upper's sync the Y-arm answers"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.copper_door.open")),
        "the copper voice the COPPER pair speaks"
    );
}

#[test]
fn the_empty_hand_the_cake_eats() {
    // THE EAT LANE'S OWN WALK (javap CakeBlock.useWithoutItem +
    // the eat static over the 26.2 jar — the EMPTY main hand the
    // TRY_WITH_EMPTY_HAND flow alone reaches, the canEat(false)
    // gate the needsFood verdict speaks, the FoodData.eat(2,
    // 0.1F) the bite's own feed, the bites<6 branch the flag-3
    // setBlock the kind swap wears, the max-bite branch the
    // removeBlock the AIR returns): the seven bites the ladder
    // walks, the food bar the +2 the bite feeds, the mesh the
    // swap re-speaks
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
        w.set(c, top, c, crate::world::CAKE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE HUNGRY BODY THE GATE ANSWERS (the default bar the
    // MAX_FOOD the new() starts — the walk the set_food_level
    // lowering speaks; the 6 start level the SEVEN bites the bar
    // carries — the +2 feed the clamp never gates, the 18 the
    // seventh bite's canEat still answers)
    app.food_data.set_food_level(6);
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
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            *expected,
            "bite {bite}: the stage the kind swap wears"
        );
        assert_eq!(
            app.food_data.food_level(),
            6 + (bite as i32 + 1) * 2,
            "bite {bite}: the +2 feed the bar wears"
        );
    }
    // THE MAX BITE THE AIR RETURNS (the removeBlock walk the
    // seventh bite's own branch serves — the eaten cake the cell
    // clears)
    assert!(app.on_right_press(0, 0), "the seventh bite consumes");
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(c, top, c),
        crate::world::AIR,
        "the cake the cell clears"
    );
    assert_eq!(
        app.food_data.food_level(),
        20,
        "the seventh feed the bar wears"
    );
}

#[test]
fn the_full_bar_the_cake_refuses() {
    // THE CANEAT GATE'S OWN PASS (javap Player.canEat(false) —
    // the needsFood verdict the full bar refuses, the PASS the
    // item lanes' fall-through keeps): the press the false
    // verdict carries, the cake the same stage keeps, the bar
    // the same level keeps
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
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the full bar the PASS verdict carries"
    );
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(c, top, c),
        crate::world::CAKE,
        "the cake the press never touches"
    );
    assert_eq!(
        app.food_data.food_level(),
        20,
        "the full bar the bite never feeds"
    );
}

#[test]
fn the_candle_the_cake_wears() {
    // THE CANDLE-ON-CAKE ARM'S OWN WALK (javap CakeBlock.
    // useItemOn pc 0-122 over the 26.2 jar — the CANDLES tag +
    // BITES==0 + instanceof CandleBlock consult the held candle
    // answers: the playSound(CAKE_ADD_CANDLE, BLOCKS, 1.0F,
    // 1.0F) the swap's first fruit speaks, the consume(1) the
    // stack answers, the setBlockAndUpdate(pos, byCandle(candle))
    // the CANDLE_CAKE kind the flag-3 broadcast wears, the
    // SUCCESS the press consumes)
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
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let candle_item = mc_registry::item_id_by_name("candle").unwrap();
    let sel = app.selected_hotbar;
    app.inventory.items[sel] = (candle_item, 3);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(app.on_right_press(0, 0), "the consult the press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::CANDLE_CAKE,
            "the byCandle walk the swap wears"
        );
    }
    assert_eq!(
        app.inventory.items[sel],
        (candle_item, 2),
        "the consume(1) the stack answers"
    );
    // THE VOICE THE SWAP PLAYS (the playSound the log's own
    // event row speaks)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.cake.add_candle")),
        "the CAKE_ADD_CANDLE voice the log wears"
    );
}

#[test]
fn the_bitten_cake_the_candle_refuses() {
    // THE BITES==0 GATE'S OWN REFUSAL (javap useItemOn pc 16-29 —
    // the `getValue(BITES) != 0` branch the TRY_WITH_EMPTY_HAND
    // returns: the bitten stage the candle consult never wears,
    // the held stack the placement lane's own walk follows — the
    // candle the placeable lookup the above cell places): the
    // cake cell the bitten stage keeps, the swap never speaks
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
        w.set(c, top, c, crate::world::CAKE_SLICE2);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let candle_item = mc_registry::item_id_by_name("candle").unwrap();
    let sel = app.selected_hotbar;
    app.inventory.items[sel] = (candle_item, 3);
    app.hotbar_slots = app.inventory.hotbar();
    // the placement lane the candle item the above cell places —
    // the press CONSUMES (the placement's own SUCCESS the
    // can_support_torch coarse fold the cake row answers — the
    // vanilla's own sturdy-face law the per-cell predicate the
    // documented fold keeps), the cake the bitten stage keeps,
    // the swap never speaks
    assert!(app.on_right_press(0, 0), "the placement the candle places");
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(c, top, c),
        crate::world::CAKE_SLICE2,
        "the bitten stage the swap never touches"
    );
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.cake.add_candle")),
        "the voice the refused consult never plays"
    );
}

/// THE TORCH'S OWN PLACEMENT WALK (the dev-32 round — javap
/// BaseTorchBlock.canSurvive the 26.2 jar the placement's own
/// canPlace gate refuses through): the held torch the ground's
/// above cell places (the normal-offset cell the non-replaceable
/// clicked law keeps, the below cell the canSupportCenter
/// delegate the survival gate reads), the floating torch the
/// refusal keeps (the below air the PASS verdict carries, nothing
/// consumed), the dig lane the generated rows rolls (the
/// instabreak row the 0.0 seconds answers, the torch drop the
/// self-entry speaks).
#[test]
fn the_torch_placement_wears_the_survival_laws() {
    // THE GROUNDED PLACEMENT: the held torch aiming straight down
    // — the hit the surface cell keeps (the non-replaceable the
    // normal-offset cell rides), the torch the above cell lands
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/torch"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the torch on the ground the placement consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::TORCH,
        "the placed torch the ground's above cell wears"
    );
    // THE FLOATING REFUSAL: a hovering stone at the carved air —
    // the side aim the west face hits, the offset cell the stone's
    // beside keeps, the below-air read the canSurvive gate refuses
    // (the press the PASS carries — the torch never floats)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/torch"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE CARVED AIR the determinism keeps: the walk lane's own
    // cells the carve empties (the terrain's own shape the center
    // column the tests never trust)
    let w = app.offline_world.as_mut().unwrap();
    for dx in -1..=3 {
        for dz in -1..=1 {
            for dy in -3..=3 {
                w.set(c + dx, top + dy, c + dz, crate::world::AIR);
            }
        }
    }
    w.set(c + 2, top + 1, c, crate::world::STONE);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.5, c as f32 + 0.5),
        yaw: 270.0,
        pitch: 0.0,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the floating torch the survival refusal keeps"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 1, top + 1, c),
        crate::world::AIR,
        "the offset cell the refusal never fills"
    );
    // THE DIG LANE'S OWN GENERATED ROWS (the instabreak row the
    // Properties chain banks, the torch drop the self-entry
    // speaks — the broken torch the item pops)
    let (secs, _) =
        mc_registry::generated::block_dig::dig_row("torch").expect("the torch's own dig row");
    assert_eq!(secs, 0.0, "the instabreak the noCollision family wears");
    let pools =
        mc_registry::generated::block_drops::drops("torch").expect("the torch's own loot row");
    assert_eq!(pools.len(), 1, "the single pool the drop speaks");
    assert_eq!(pools[0].entries[0].item, "torch", "the torch drops itself");
    // THE PICK LANE'S OWN GRANT (the item id the name walk answers)
    assert!(mc_registry::item_id_by_name("torch").is_some());
}

#[test]
fn the_wall_head_placement_wears_the_twin_laws() {
    // THE WALL HEADS' OWN PLACEMENT LAWS (the dev-83 round — javap
    // StandingAndWallBlockItem.getPlacementState + WallSkullBlock.
    // getStateForPlacement the 26.2 jar the probe
    // scripts/skull_probe_83.py banks): the side hit the WALL twin
    // the item's secondary block wears [the scan arm the
    // non-replaceable support finds, the FACING the opposite fold
    // stamps], the top hit the FLOOR kind the item's primary keeps
    // [the SEGMENT stamp the rotation segment wears].
    // THE WALL CASE (the carved air the determinism keeps — the
    // stone the east wall the west face hit answers): the skull
    // the wall twin's kind the offset cell wears, the facing the
    // west seat the support's own opposite banks
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/skeleton_skull"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=3 {
            for dz in -1..=1 {
                for dy in -3..=3 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        w.set(c + 2, top + 1, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.5, c as f32 + 0.5),
        // THE EAST LOOK (the forward walk sin(yaw) the +x arm
        // speaks — the pi/2 yaw the east ray aims, the stone's
        // west face the hit answers)
        yaw: std::f32::consts::FRAC_PI_2,
        // THE UP LOOK (the dev-85 round — the pitch gate the side
        // hit the orderedByNearest b8 fold wears: the vanilla
        // sinP<0 the UP-first arm the wall scan banks, the
        // client's own convention the UP look the POSITIVE pitch
        // wears — the +0.2 slope the ray the west face the y
        // top+1.804 the face span keeps, the level/down look the
        // FLOOR arm the floating skull the pitch-gate test speaks)
        pitch: 0.2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the skull on the wall the twin placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top + 1, c),
            crate::world::SKELETON_WALL_SKULL,
            "the side hit the WALL twin the offset cell wears"
        );
        assert_eq!(
            w.variant(c + 1, top + 1, c),
            crate::world::WALL_TORCH_FACING_WEST,
            "the facing the support's own opposite the scan stamps"
        );
    }
    // THE FLOOR CASE (the same stone the top face the down-look
    // hits): the skull the FLOOR kind the above cell wears — the
    // item's primary the SEGMENT stamp the rotation segment keeps
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/skeleton_skull"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=3 {
            for dz in -1..=1 {
                for dy in -3..=4 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        w.set(c + 2, top + 1, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        // THE DIAGONAL DOWN-LOOK (the eye the body's own column
        // never enters the placement cell's conservative box —
        // the pi/2 yaw the east arm aims, the -45 pitch the ray
        // the stone's top face lands: the eye at the carved
        // cell's own column, the body's box x 40.2..40.8 the
        // placed cell x 42..43 the honest absence keeps)
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: -std::f32::consts::FRAC_PI_4,
    });
    assert!(
        app.on_right_press(0, 0),
        "the skull on the stone top the floor placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 2, top + 2, c),
            crate::world::SKELETON_SKULL,
            "the top hit the FLOOR kind the above cell wears"
        );
        assert!(
            w.variant(c + 2, top + 2, c) <= 15,
            "the rotation segment the variant byte wears"
        );
    }
}

#[test]
fn the_ceiling_click_wears_the_wall_first_law() {
    // THE CEILING CLICK'S OWN WALL-FIRST LAW (the dev-84 round —
    // probe scripts/skull_probe_84.py the StandingAndWallBlockItem
    // .getPlacementState 145B walk the 26.2 jar banks): the bottom
    // face hit the UP arm the wall state serves [the dirs scan the
    // pitch<0 look the UP-first order wears — the horizontal dirs
    // the loop's skip the wall attempt the look-ordered support
    // scan answers], the all-fail walk the FLOOR fallback the
    // floating skull keeps [the canSurvive default the true the
    // no-override absence speaks — the SkullBlock census the
    // canSurvive row never banks, the SEGMENT stamp the rotation
    // walk wears].
    // THE WALL-FIRST CASE (the ceiling stone the bottom face the
    // up-ray lands, the support stone the south neighbor the scan
    // finds): the skull the WALL twin the ceiling cell wears, the
    // facing the north seat the support's own opposite banks
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/skeleton_skull"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=3 {
            for dz in -1..=2 {
                for dy in -3..=4 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        // THE CEILING STONE (the bottom face y top+2 the up-ray
        // the tan(0.75) slope lands) + THE SUPPORT (the south
        // neighbor the first non-replaceable horizontal the scan
        // finds)
        w.set(c + 2, top + 2, c, crate::world::STONE);
        w.set(c + 2, top + 1, c + 1, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 0.5, c as f32 + 0.5),
        // THE EAST UP-LOOK (the yaw pi/2 the +x arm aims, the
        // pitch the 0.75 slope the bottom-face crossing wears:
        // the delta y 1.5 the delta x 2.0 the x c+2.5 the
        // ceiling's underside the DOWN face lands — the target
        // cell c+2/top+1 the last air walk keeps, the body's own
        // column the x c+0.2..c+0.8 the placed cell never enters)
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.75f32.atan(),
    });
    assert!(
        app.on_right_press(0, 0),
        "the ceiling click the wall twin placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 2, top + 1, c),
            crate::world::SKELETON_WALL_SKULL,
            "the ceiling click the WALL twin the wall-first law wears"
        );
        assert_eq!(
            w.variant(c + 2, top + 1, c),
            crate::world::WALL_TORCH_FACING_NORTH,
            "the facing the support's own opposite the scan stamps"
        );
    }
    // THE FLOOR FALLBACK CASE (the same ceiling the support the
    // carved absence keeps): the scan the None verdict the all-fail
    // walk speaks, the skull the FLOOR kind the floating seat wears
    // — the canSurvive default the true the no-override law banks,
    // the SEGMENT stamp the rotation walk keeps
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/skeleton_skull"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=3 {
            for dz in -1..=2 {
                for dy in -3..=4 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        w.set(c + 2, top + 2, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 0.5, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.75f32.atan(),
    });
    assert!(
        app.on_right_press(0, 0),
        "the ceiling click the floating floor placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 2, top + 1, c),
            crate::world::SKELETON_SKULL,
            "the all-fail walk the FLOOR kind the ceiling cell wears"
        );
        assert!(
            w.variant(c + 2, top + 1, c) <= 15,
            "the rotation segment the floating skull wears"
        );
    }
}

#[test]
fn the_side_hit_wears_the_pitch_gate_law() {
    // THE SIDE HIT'S OWN PITCH GATE (the dev-85 round — the
    // orderedByNearest b8 fold the probe scripts/skull_probe_84.py
    // banks: the vanilla sinP<0 the UP-first order the wall scan
    // banks, the level/down look the DOWN-first arm the floor
    // state the scan's first survive answers — the canSurvive
    // default the true the floating skull the SEGMENT stamp keeps,
    // the UP arm the scan never reaches; the client's own
    // convention the UP look the POSITIVE pitch wears, the
    // cam_pitch>0.0 the wall arm alone re-arms).
    // THE LEVEL LOOK (the pitch 0.0 the sinP=0 the DOWN-first
    // boundary the fold wears) + THE DOWN LOOK (the pitch -0.2
    // the vanilla down-look the same arm banks): the side click
    // the FLOOR kind the offset cell wears — the wall twin the
    // kind never seats, the segment the yaw stamp keeps
    for (pitch, label) in [(0.0f32, "the level look"), (-0.2f32, "the down look")] {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/skeleton_skull"));
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        {
            let w = app.offline_world.as_mut().unwrap();
            for dx in -1..=3 {
                for dz in -1..=1 {
                    for dy in -3..=3 {
                        w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                    }
                }
            }
            w.set(c + 2, top + 1, c, crate::world::STONE);
        }
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.5, c as f32 + 0.5),
            yaw: std::f32::consts::FRAC_PI_2,
            pitch,
        });
        assert!(
            app.on_right_press(0, 0),
            "{label} the floating floor placement consumes"
        );
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top + 1, c),
            crate::world::SKELETON_SKULL,
            "{label} the DOWN-first arm the FLOOR kind the offset cell wears"
        );
        assert!(
            w.variant(c + 1, top + 1, c) <= 15,
            "{label} the rotation segment the floating skull wears"
        );
    }
    // THE UP LOOK (the pitch +0.2 the tan slope the west face the
    // y top+1.804 the face span keeps — the eye the body's own
    // column the placement cell never enters): the side click the
    // WALL twin the UP-first arm wears, the facing the west seat
    // the support's own opposite stamps
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/skeleton_skull"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=3 {
            for dz in -1..=1 {
                for dy in -3..=3 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        w.set(c + 2, top + 1, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.5, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the up look the wall twin placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top + 1, c),
            crate::world::SKELETON_WALL_SKULL,
            "the up look the UP-first arm the WALL twin wears"
        );
        assert_eq!(
            w.variant(c + 1, top + 1, c),
            crate::world::WALL_TORCH_FACING_WEST,
            "the facing the support's own opposite the scan stamps"
        );
    }
}

/// THE PICK'S OWN OUTLINE SHAPE (the dev-87 law): the rail the
/// band verdict wears (the SHAPE_FLAT the clip refines), the
/// every-other-kind the full cube keeps, the air the honest
/// default keeps. THE ASCENT'S OWN TOP (the dev-94 round): the
/// ascent bytes the 8-px box the same verdict speaks.
#[test]
fn the_hit_outline_wears_the_band_only_for_the_rails() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c, crate::world::RAIL);
        w.set(c + 2, top, c, crate::world::STONE);
        w.set(c + 4, top, c, crate::world::RAIL);
        w.set_variant(c + 4, top, c, 2); // ASCENDING_EAST
    }
    let band_top = app.hit_outline_band_top((c, top, c));
    assert_eq!(
        band_top,
        crate::world::RAIL_FLAT_TOP as f32,
        "the flat rail the 2-px wireframe arms"
    );
    let ascent_top = app.hit_outline_band_top((c + 4, top, c));
    assert_eq!(
        ascent_top,
        crate::world::RAIL_ASC_TOP as f32,
        "the ascent bytes the 8-px box the verdict serves"
    );
    assert_eq!(
        app.hit_outline_band_top((c + 2, top, c)),
        0.0,
        "the stone the full cube keeps"
    );
    assert_eq!(
        app.hit_outline_band_top((c + 5, top, c)),
        0.0,
        "the air the full cube keeps"
    );
}

/// THE PLACEMENT'S OWN INSTANT CORNER (the dev-95 round — javap
/// RailState.place the 26.2 jar the initial connection derivation
/// the on_right_press walk the dev-89 handshake serves): the rail
/// the perpendicular pair's own cell the placement lands in the
/// CORNER shape the same press wears — the curve arm the dev-92
/// verdict banks (the north+east pair the NORTH_EAST byte 9
/// answers, the curve-capable gate the plain RAIL alone takes),
/// the perpendicular arms the axis law they already held keep
/// (the E-W byte 1 the east arm's own west connection answers,
/// the N-S byte 0 the north arm's own south connection speaks),
/// the look-stamp the byte the handshake's verdict overrides (the
/// raw look the E-W quarter the corner arm never keeps).
#[test]
fn the_rail_placement_wears_the_instant_corner() {
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/rail"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=2 {
            for dz in -2..=2 {
                for dy in -3..=3 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        // the three supports: the corner cell's own floor, the
        // east arm's, the north arm's
        w.set(c, top - 1, c, crate::world::STONE);
        w.set(c + 1, top - 1, c, crate::world::STONE);
        w.set(c, top - 1, c - 1, crate::world::STONE);
    }
    // THE EAST ARM: the east-look floor click the E-W quarter
    // stamps (the lone rail the look-stamp keeps — the dev-89 law)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 1.5, top as f32 + 3.0, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the east arm the placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::RAIL,
            "the east arm placed"
        );
        assert_eq!(
            w.variant(c + 1, top, c),
            1,
            "the lone arm the look-stamp keeps"
        );
    }
    // THE NORTH ARM: the same down click the north cell serves (the
    // N-S default the Z-axis look stamps) — the two arms the
    // perpendicular pair the corner cell's future speaks
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 3.0, c as f32 - 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the north arm the placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c - 1),
            crate::world::RAIL,
            "the north arm placed"
        );
        assert_eq!(
            w.variant(c, top, c - 1),
            0,
            "the lone arm the look-stamp keeps"
        );
    }
    // THE INSTANT CORNER: the perpendicular pair's own cell the
    // same press lands in — the look-stamp the raw E-W look wears,
    // the curve arm the verdict overrides (the north+east pair the
    // NORTH_EAST byte 9 the vanilla's own initial derivation
    // serves), the arms the axis bytes they held keep
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 3.0, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the corner cell the placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(w.get(c, top, c), crate::world::RAIL, "the corner placed");
        assert_eq!(
            w.variant(c, top, c),
            9,
            "the perpendicular pair the instant corner wears"
        );
        assert_eq!(
            w.variant(c + 1, top, c),
            1,
            "the east arm the west connection the E-W byte keeps"
        );
        assert_eq!(
            w.variant(c, top, c - 1),
            0,
            "the north arm the south connection the N-S byte keeps"
        );
    }
}

/// THE PLACEMENT'S OWN ASCENT SEAT (the dev-95 round — javap
/// RailState.place the slope arms the on_right_press walk the
/// dev-93 handshake serves): the rail the slope's low end the
/// placement lands in the ASCENDING byte the same press wears —
/// the up-rail the step holds the low arm's canSurvive consult
/// reads (the Full floor the ride keeps), the high seat the same
/// byte the one slope the two cells share keeps.
#[test]
fn the_rail_placement_wears_the_ascent_seat() {
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/rail"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=2 {
            for dz in -2..=2 {
                for dy in -3..=3 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        // the floor the low seat rides, the step the slope climbs,
        // the up-rail the step's top holds (the look-stamp
        // convention the direct stamp the tests' own set wears)
        w.set(c, top - 1, c, crate::world::STONE);
        w.set(c + 1, top, c, crate::world::STONE);
        w.set(c + 1, top + 1, c, crate::world::RAIL);
        w.set_variant(c + 1, top + 1, c, 2);
    }
    // THE DOWN CLICK: the floor's top face the ray names, the low
    // seat the placement lands in — the ascent arm the up-rail +
    // Full step the byte 2 speaks
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 3.0, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the low seat the placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(w.get(c, top, c), crate::world::RAIL, "the low seat placed");
        assert_eq!(
            w.variant(c, top, c),
            2,
            "the up-rail the step holds the ascent byte speaks"
        );
        assert_eq!(
            w.variant(c + 1, top + 1, c),
            2,
            "the high seat the same slope byte keeps"
        );
    }
}

/// THE RAIL PLACEMENT'S OWN CONNECTION HANDSHAKE (the dev-89 round—
/// the on_right_press walk the sweep call serves): the second rail
/// the single connection the axis along it wears, the first rail
/// the same law flips (the E-W look-stamped pair the N-S cross the
/// handshake speaks), the vanilla's own neighbor updates the beat
/// answers.
#[test]
fn the_rail_placement_wears_the_connection_handshake() {
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/rail"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=1 {
            for dz in -2..=2 {
                for dy in -3..=3 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        w.set(c, top - 1, c, crate::world::STONE);
        w.set(c, top - 1, c - 1, crate::world::STONE);
    }
    // THE FIRST RAIL: the east-look floor click the X-axis quarter
    // stamps (the E-W byte 1 the dev-86 law wears)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 3.0, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the first rail the placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::RAIL,
            "the first rail placed"
        );
        assert_eq!(
            w.variant(c, top, c),
            1,
            "the lone rail the look-stamp keeps"
        );
    }
    // THE SECOND RAIL: the north cell the same east look stamps (the
    // E-W byte the placement wears), the sweep the single-connection
    // law overrides (the south connection the N-S axis answers), the
    // first rail the handshake flips to the same cross
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 3.0, c as f32 - 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the second rail the placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c - 1),
            crate::world::RAIL,
            "the second rail placed"
        );
        assert_eq!(
            w.variant(c, top, c - 1),
            0,
            "the placed rail the south connection aligns"
        );
        assert_eq!(
            w.variant(c, top, c),
            0,
            "the first rail the handshake flips to N-S"
        );
    }
}

#[test]
fn the_rail_dig_wears_the_support_pop_and_the_re_align() {
    // THE SUPPORT POP'S OWN DIG BEAT (the dev-90 round — javap
    // BaseRailBlock.canSurvive the neighborChanged walk the
    // destroyBlock chain serves): the dug support the rail above
    // pops the SAME completion wears, the pop's own broadcast the
    // perpendicular survivor re-stamps (the single connection the
    // axis along it wears), the voice + the drop lane the pop list
    // the [`Self::speak_attach_pops`] beat carries
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        // the lane the ray and the eye ride
        for dx in -3..=2 {
            for dz in -3..=2 {
                for dy in -2..=2 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        // the supports: the dug one under the pop's rail, the
        // survivor's own pair the L wears
        w.set(c, top - 1, c, crate::world::STONE);
        w.set(c + 1, top - 1, c, crate::world::STONE);
        w.set(c + 1, top - 1, c - 1, crate::world::STONE);
        // the rails: R1 the support's loss pops, R2 the
        // perpendicular pair rides, R4 the north arm the single
        // connection aligns
        w.set(c, top, c, crate::world::RAIL);
        w.set_variant(c, top, c, 1);
        w.set(c + 1, top, c, crate::world::RAIL);
        w.set_variant(c + 1, top, c, 1);
        w.set(c + 1, top, c - 1, crate::world::RAIL);
        w.set_variant(c + 1, top, c - 1, 0);
    }
    // THE SIDE LOOK: the support's own layer the ray rides (the y
    // the rail's band never crosses — the west face the hit
    // names, the rail above the ray never consults)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 - 2.5, top as f32 - 0.5, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    app.on_press(0, 0);
    assert!(app.dig.is_destroying, "the press armed the support's dig");
    assert_eq!(app.dig.pos, (c, top - 1, c), "the stone the ray names");
    // THE HOLD: the ticks mine the stone, the completion the pop
    // the SAME beat serves
    let mut ticks = 0;
    while app.offline_world.as_ref().unwrap().get(c, top - 1, c) != crate::world::AIR {
        app.update();
        ticks += 1;
        assert!(ticks < 400, "the dig completed in time");
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::AIR,
            "the rail the support's loss pops — the same beat"
        );
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::RAIL,
            "the survivor rides"
        );
        assert_eq!(
            w.variant(c + 1, top, c),
            0,
            "the perpendicular survivor the single north re-stamps"
        );
        assert_eq!(
            w.variant(c + 1, top, c - 1),
            0,
            "the north arm the N-S byte keeps"
        );
    }
}

/// THE PLACEMENT'S OWN SUPPORT CHAIN (the dev-91 round — javap
/// BlockItem.placeBlock pc 9 the setBlock flag-11 broadcast the
/// UPDATE_NEIGHBORS bit keeps, the rail above the changed cell the
/// neighborChanged survival consult reads): the torch into the
/// floating rail's support cell the pop speaks [the thin below the
/// canSurvive refusal the destroyBlock walk serves, the 2001 voice
/// + the drop roll the speak walk carries], the stone into the
/// same cell the survive verdict keeps (the Full support the rail
/// rides). The offset look the ray the rail's own band never
/// consults wears — the pass-under the clip law keeps, the hit the
/// floor's top face names, the offset cell the rail's support cell
/// equals.
#[test]
fn the_rail_placement_wears_the_support_chain() {
    // THE POP ARM: the torch the support cell lands, the floating
    // rail the consult pops the SAME beat wears
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/torch"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        // the lane the ray and the eye ride
        for dx in -3..=2 {
            for dz in -2..=2 {
                for dy in -3..=3 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        // the floor the click lands on + the floating rail the
        // support cell below keeps (the air the canSurvive never
        // banks — the look-stamp the tests' own set wears)
        w.set(c, top - 2, c, crate::world::STONE);
        w.set(c, top, c, crate::world::RAIL);
    }
    // THE OFFSET LOOK: the ray the band underpasses (the +x/-y
    // slope the floor's top face the far half names — the reach
    // the 3.3 the click keeps, the rail's own layer the ray
    // enters below the band's y — the hit the support cell's
    // floor names)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 - 1.0, top as f32 + 2.0, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: -1.15,
    });
    assert!(app.on_right_press(0, 0), "the torch the placement consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top - 1, c),
            crate::world::TORCH,
            "the torch the support cell wears"
        );
        assert_eq!(
            w.get(c, top, c),
            crate::world::AIR,
            "the rail the thin support's verdict pops"
        );
    }
    // THE KEEP ARM: the stone the same cell wears, the Full
    // support the consult keeps — the rail rides
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/stone"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -3..=2 {
            for dz in -2..=2 {
                for dy in -3..=3 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        w.set(c, top - 2, c, crate::world::STONE);
        w.set(c, top, c, crate::world::RAIL);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 - 1.0, top as f32 + 2.0, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: -1.15,
    });
    assert!(app.on_right_press(0, 0), "the stone the placement consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top - 1, c),
            crate::world::STONE,
            "the stone the support cell wears"
        );
        assert_eq!(
            w.get(c, top, c),
            crate::world::RAIL,
            "the rail the Full support keeps"
        );
    }
}

#[test]
fn the_rail_placement_wears_the_axis_and_rigid_laws() {
    // THE RAILS' OWN PLACEMENT LAWS (the dev-86 round — javap
    // BaseRailBlock the 26.2 jar the probe scripts/rail_probe_86.py
    // banks): the floor click the offset cell the RAIL kind wears
    // [the getHorizontalDirection RAW look the axis variant
    // stamps], the canSurvive walk the rigid-below verdict reads
    // [the floating rail the refusal keeps — the press the PASS
    // carries, nothing consumed].
    // THE EAST LOOK'S OWN E-W QUARTER (the yaw pi/2 the +x arm
    // aims, the down ray the stone's top face lands): the rail
    // the offset cell wears, the variant the EAST_WEST byte banks
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/rail"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=3 {
            for dz in -1..=1 {
                for dy in -3..=3 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        w.set(c, top - 1, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 3.0, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the rail on the stone the placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::RAIL,
            "the floor click the offset cell wears"
        );
        assert_eq!(
            w.variant(c, top, c),
            1,
            "the east look the EAST_WEST quarter wears"
        );
    }
    // THE NORTH LOOK'S OWN N-S DEFAULT (the yaw 0 the -z arm aims):
    // the same floor the NORTH_SOUTH byte banks
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/rail"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=3 {
            for dz in -1..=1 {
                for dy in -3..=3 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        w.set(c, top - 1, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 3.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the rail on the stone the placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::RAIL,
            "the floor click the offset cell wears"
        );
        assert_eq!(
            w.variant(c, top, c),
            0,
            "the north look the NORTH_SOUTH default wears"
        );
    }
    // THE FLOATING REFUSAL (the side hit the placement cell the
    // carved absence below keeps): the canSurvive walk the
    // rigid-below verdict refuses — the press the PASS carries,
    // the offset cell the refusal never fills
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/rail"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=3 {
            for dz in -1..=1 {
                for dy in -3..=3 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        w.set(c + 2, top + 1, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.5, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the floating rail the survival refusal keeps"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 1, top + 1, c),
        crate::world::AIR,
        "the offset cell the refusal never fills"
    );
    // THE SUPPORTED SIDE HIT (the below cell the stone carries):
    // the gate passes the side-click lane the rail lands, the
    // east look the same quarter stamps
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/rail"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=3 {
            for dz in -1..=1 {
                for dy in -3..=3 {
                    w.set(c + dx, top + dy, c + dz, crate::world::AIR);
                }
            }
        }
        w.set(c + 2, top + 1, c, crate::world::STONE);
        w.set(c + 1, top, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.5, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    assert!(
        app.on_right_press(0, 0),
        "the supported rail the side placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top + 1, c),
            crate::world::RAIL,
            "the side hit the offset cell wears"
        );
        assert_eq!(
            w.variant(c + 1, top + 1, c),
            1,
            "the east look the same quarter wears"
        );
    }
}

#[test]
fn the_double_plant_placement_wears_the_two_cell_laws() {
    // THE DOUBLE PLANT'S OWN PLACEMENT LAWS (javap DoublePlantBlock
    // over the 26.2 jar — the autopush-413 round): the held
    // tall_grass the grass cell PLACES THE PAIR (the LOWER the
    // clicked cell wears, the UPPER the setPlacedBy walk's
    // `pos.above()` cell keeps), the stone-below refusal the
    // canSurvive gate speaks, the OCCUPIED-above refusal the
    // getStateForPlacement's `above.canBeReplaced(context)` walk
    // keeps, the placement-INTO the pair's replaceable rows wear
    // (the clicked half gives way, the ORPHAN the mate's own clear
    // removes), the dig lane's PAIR LAW the broken half answers
    // (the mate the setBlock(35) walk clears, the LOOT the broken
    // cell alone rolls).
    // THE PAIR'S OWN PLACEMENT (the vertical walk — the grass
    // surface the below-cell read answers, the air above the
    // upper-cell gate keeps). The column the ray rides stays AIR
    // (the natural decor the controlled walk keeps out)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/tall_grass"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for y in top..=top + 4 {
            w.set(c, y, c, crate::world::AIR);
        }
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the tall_grass on grass the placement consumes"
    );
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(c, top, c),
        crate::world::TALL_GRASS,
        "the LOWER half the clicked cell wears"
    );
    assert_eq!(
        w.get(c, top + 1, c),
        crate::world::TALL_GRASS_UPPER,
        "the UPPER half the setPlacedBy walk wears"
    );

    // THE OCCUPIED-ABOVE REFUSAL (the getStateForPlacement walk —
    // the non-replaceable above cell the null refusal keeps): the
    // leaves the pair's upper cell blocks, the horizontal ray the
    // placement cell names from the SIDE (the vertical ray the
    // leaves themselves would hit)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/tall_grass"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c + 1, top, c, crate::world::STONE); // the ray's own wall
        w.set(c, top + 1, c, crate::world::LEAVES); // the blocked upper
        for dx in -4..=0 {
            w.set(c + dx, top, c, crate::world::AIR); // the ray's row
        }
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new((c - 4) as f32 + 0.5, top as f32 + 0.5, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2, // facing +X (east)
        pitch: 0.0,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the occupied above cell the upper gate refuses"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::AIR,
        "the refusal the PASS keeps — nothing placed"
    );

    // THE PLACEMENT-INTO'S OWN ORPHAN CLEAR (the updateShape walk —
    // the replaced half the mate's removal answers): the standing
    // pair the stone's placement breaks — the ray from above the
    // UPPER half names (the replaceable clicked cell the placement
    // target keeps), the stone the upper's own cell takes, the
    // LOWER the orphan's own clear removes
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/stone"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c, crate::world::TALL_GRASS);
        w.set(c, top + 1, c, crate::world::TALL_GRASS_UPPER);
        for y in top + 2..=top + 4 {
            w.set(c, y, c, crate::world::AIR);
        }
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the placement into the pair's upper the press consumes"
    );
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(c, top + 1, c),
        crate::world::STONE,
        "the stone the clicked upper's cell wears"
    );
    assert_eq!(
        w.get(c, top, c),
        crate::world::AIR,
        "the orphaned lower the mate's clear removes"
    );

    // THE DIG'S OWN PAIR LAW (the preventDropFromBottomPart walk —
    // the UPPER's dig the below cell's removal speaks, the LOOT the
    // broken cell alone rolls): the held hand digs the pair's
    // upper, both cells take the air, the drop (when the 1/8 roll
    // speaks) the wheat_seeds item wears
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c, crate::world::TALL_GRASS);
        w.set(c, top + 1, c, crate::world::TALL_GRASS_UPPER);
        for y in top + 2..=top + 6 {
            w.set(c, y, c, crate::world::AIR);
        }
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 6.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    app.on_press(0, 0);
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(c, top + 1, c),
        crate::world::AIR,
        "the dug upper the dig lane clears"
    );
    assert_eq!(
        w.get(c, top, c),
        crate::world::AIR,
        "the lower half the pair law's own removal keeps"
    );
    // the loot's own row (the generated table the dig lane rolls):
    // the tall_grass/large_fern rows the pair's identical pools wear
    for name in ["tall_grass", "large_fern"] {
        let pools = mc_registry::generated::block_drops::drops(name)
            .expect("the double plant's own loot row");
        assert_eq!(pools.len(), 1, "the single pool the drop speaks");
        assert_eq!(pools[0].entries.len(), 1, "the else arm's single entry");
        assert_eq!(pools[0].entries[0].item, "wheat_seeds", "the seeds drop");
        assert_eq!(pools[0].entries[0].chance, 0.125, "the 1/8 chance");
    }
    assert!(
        app.drops.is_empty() || app.drops.len() == 1,
        "the broken cell's own roll the only drop speaks"
    );
    for d in &app.drops {
        assert_eq!(
            d.item,
            mc_registry::item_id_by_name("wheat_seeds").unwrap(),
            "the seeds the hand lane rolls"
        );
    }

    // THE LOWER'S OWN DIG (the horizontal ray the lower cell's side
    // face names — the upper the ray never crosses): the pair law
    // the ABOVE mate's silent removal wears (the updateShape walk)
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c, crate::world::TALL_GRASS);
        w.set(c, top + 1, c, crate::world::TALL_GRASS_UPPER);
        // the ray's own row (the eye cell + the approach — the pair's
        // lower cell the range never touches)
        for dx in -4..=-1 {
            w.set(c + dx, top, c, crate::world::AIR);
        }
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new((c - 4) as f32 + 0.5, top as f32 + 0.5, c as f32 + 0.5),
        yaw: std::f32::consts::FRAC_PI_2, // facing +X (east)
        pitch: 0.0,
    });
    app.on_press(0, 0);
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(c, top, c),
        crate::world::AIR,
        "the dug lower the dig lane clears"
    );
    assert_eq!(
        w.get(c, top + 1, c),
        crate::world::AIR,
        "the upper half the pair law's own removal keeps"
    );
    // (the upper's dig the 2001 voice at the mate wears — the
    // preventDropFromBottomPart levelEvent; the lower's dig the
    // silent updateShape keeps — the two arms the vanilla's own
    // asymmetry speaks)
}

#[test]
fn the_right_press_shears_the_offline_sheep() {
    // THE TABLE'S FIRST VERDICT (the vanilla startUseItem's ENTITY
    // case — the offline prediction `player.interactOn` returns):
    // the sheep the pick lands, the shears the held stack speaks —
    // the Shear the shearing/sheep/red table rolls, the press the
    // SUCCESS_SERVER merge consumes (the voice the playSound law
    // rides, the loot the copyWithCount(1) singles spawn at the
    // body's feet, the swing the SwingSource.SERVER starts); the
    // sheared bit closes the gate — the second press the item
    // lane's own PASS refuses (the shears no use def carry).
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/shears"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut sheep = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    sheep.kind = crate::mob::MobKind::Sheep;
    sheep.wool_color = 14;
    app.mobs.push(sheep);
    assert_eq!(
        app.pick_mob(&app.mobs),
        Some(0),
        "the offline sheep the verdict speaks"
    );
    assert!(
        app.on_right_press(0, 0),
        "the Shear verdict consumed the press"
    );
    // THE WOOL'S OWN TRUTH: the bit set, the singles spawned (the
    // App's own mob_rng first draw — the seed 0x5EED_10CA's % 3
    // speaks 1), each one red_wool count 1 at the body's feet
    assert!(app.mobs[0].sheared, "setSheared(true) the press speaks");
    let red = crate::mob::wire_id("red_wool").expect("red_wool the table draws");
    assert_eq!(app.drops.len(), 1, "the mob_rng's own first roll");
    assert_eq!(app.drops[0].item, red, "the red table's own item");
    assert_eq!(app.drops[0].count, 1, "the copyWithCount(1) law");
    assert!(
        (app.drops[0].x - (c as f64 + 0.5)).abs() < 0.5,
        "the spawnAtLocation law's own x"
    );
    // THE VOICE: the playSound(SHEEP_SHEAR, PLAYERS, 1.0, 1.0)
    // the shear's own walk opens with
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.sheep.shear ")),
        "the shear's own voice the lane carries"
    );
    // THE SWING: the SwingSource.SERVER the verdict carries
    assert!(app.swing.swinging, "the press swung the arm");
    // THE CLOSED GATE: the second press falls to the item lane —
    // the shears the use def never speak, the PASS refuses
    assert!(
        !app.on_right_press(0, 0),
        "the sheared sheep the gate closed"
    );
}

/// THE ABSTRACTCOW'S OWN MILK ARM (javap mobInteract 13-55 — the
/// 26.2 bytecode the subclass's own override keeps, BEFORE the
/// Animal super's food walk): the grown-up cow the pick lands, the
/// empty bucket the held stack speaks — the Milk verdict the
/// InteractionResult.SUCCESS consumes (the voice the
/// playSound(COW_MILK, 1.0F, 1.0F) law rides, the swing the
/// SUCCESS verdict starts, the createFilledResult the survival
/// walk answers — the count-1 stack the `consume(1)` empties, the
/// filled instance the setItemInHand REPLACES).
#[test]
fn the_right_press_milks_the_offline_cow() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut cow = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    cow.kind = crate::mob::MobKind::Cow;
    app.mobs.push(cow);
    assert_eq!(
        app.pick_mob(&app.mobs),
        Some(0),
        "the offline cow the verdict speaks"
    );
    // THE HELD STACK (the inventory's selected slot the held_id
    // walk speaks first — the count-1 bucket the hand carries)
    let bucket_id = mc_registry::item_id_by_name("bucket").expect("bucket the table draws");
    let milk_id = mc_registry::item_id_by_name("milk_bucket").expect("milk_bucket the table draws");
    app.inventory.items[app.selected_hotbar] = (bucket_id, 1);
    assert!(
        app.on_right_press(0, 0),
        "the Milk verdict consumed the press"
    );
    // THE VOICE: the playSound(COW_MILK, 1.0F, 1.0F) the walk
    // opens with
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.cow.milk ")),
        "the milk's own voice the lane carries"
    );
    // THE FILLED RESULT: the count-1 stack the consume(1) empties,
    // the filled instance the setItemInHand REPLACES
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (milk_id, 1),
        "the filled instance the hand keeps"
    );
    // THE SWING: the InteractionResult.SUCCESS the verdict carries
    assert!(app.swing.swinging, "the press swung the arm");
}

/// THE TALLER STACK'S OWN FILL (the createFilledResult survival
/// walk's tail — the `consume(1)` the shrink speaks, the shrunken
/// stack the hand keeps, the filled instance the `Inventory.add`
/// walk lands — a free slot takes it, the drop the overflow never
/// speaks while a home exists).
#[test]
fn the_cow_milk_keeps_the_taller_stack() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut cow = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    cow.kind = crate::mob::MobKind::Cow;
    app.mobs.push(cow);
    let bucket_id = mc_registry::item_id_by_name("bucket").expect("bucket the table draws");
    let milk_id = mc_registry::item_id_by_name("milk_bucket").expect("milk_bucket the table draws");
    app.inventory.items[app.selected_hotbar] = (bucket_id, 5);
    assert!(app.on_right_press(0, 0), "the Milk verdict consumed");
    // THE SHRINK: the shrunken stack the hand keeps, the filled
    // instance the free slot takes (the getFreeSlot walk — the
    // first empty the items order keeps), NO drop the home speaks
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (bucket_id, 4),
        "the consume(1) the shrunken stack keeps"
    );
    let free = app
        .inventory
        .items
        .iter()
        .position(|&s| s == (milk_id, 1))
        .expect("the filled instance the inventory takes");
    assert_ne!(free, app.selected_hotbar, "the fill the OTHER slot keeps");
    assert!(
        app.drops.is_empty(),
        "the overflow the home's existence silences"
    );
}

/// THE BABY'S OWN REFUSAL (the `!isBaby()` band the gate keeps —
/// the bytecode 16-20 the `ifne -> 56` speaks): the baby cow the
/// bucket's press the super's chain falls to — the food arm the
/// bucket never answers, the milk the walk never speaks, the
/// stack the walk never touches.
#[test]
fn the_baby_cow_refuses_the_milk_arm() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut cow = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    cow.kind = crate::mob::MobKind::Cow;
    cow.age = crate::mob::BABY_START_AGE + 1600;
    app.mobs.push(cow);
    let bucket_id = mc_registry::item_id_by_name("bucket").expect("bucket the table draws");
    app.inventory.items[app.selected_hotbar] = (bucket_id, 1);
    // THE PRESS: the verdict the super's PASS speaks — the walk
    // falls to the item lane (the empty bucket no use def carries,
    // the press the lane's own verdict answers whatever it says)
    let _ = app.on_right_press(0, 0);
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.cow.milk ")),
        "the baby the milk voice silences"
    );
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (bucket_id, 1),
        "the stack the refused walk never touches"
    );
    assert!(!app.swing.swinging, "no verdict the swing never starts");
}

/// THE OVERFLOW'S OWN DROP (the `!Inventory.add` arm the
/// createFilledResult tail speaks — `player.drop(filledStack,
/// false)` the entity at the player's own feet): the full
/// inventory the fill no home answers, the leftover the drop walk
/// spawns, the shrunken stack the hand keeps.
#[test]
fn the_full_inventory_drops_the_cow_milk() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut cow = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    cow.kind = crate::mob::MobKind::Cow;
    app.mobs.push(cow);
    let bucket_id = mc_registry::item_id_by_name("bucket").expect("bucket the table draws");
    let milk_id = mc_registry::item_id_by_name("milk_bucket").expect("milk_bucket the table draws");
    // THE FULL INVENTORY (every slot the filler's full stack keeps
    // — no home the fill's add walk ever finds)
    let filler_id = mc_registry::item_id_by_name("wheat").expect("wheat the table draws");
    for slot in app.inventory.items.iter_mut() {
        *slot = (filler_id, 64);
    }
    app.inventory.items[app.selected_hotbar] = (bucket_id, 2);
    assert!(app.on_right_press(0, 0), "the Milk verdict consumed");
    // THE SHRINK + THE DROP: the shrunken stack the hand keeps,
    // the fill no home found — the entity the drop walk spawns at
    // the player's own feet
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (bucket_id, 1),
        "the consume(1) the shrunken stack keeps"
    );
    assert_eq!(app.drops.len(), 1, "the overflow the drop speaks");
    assert_eq!(app.drops[0].item, milk_id, "the filled instance drops");
    assert_eq!(app.drops[0].count, 1, "the single fill the drop carries");
}

/// THE PIG'S OWN SADDLE PRESS (javap Pig.mobInteract 73-100 — the
/// post-super arm the 26.2 bytecode keeps): the saddle the held
/// stack speaks, the unsaddled hull the gate opens — the Saddle
/// verdict the InteractionResult.SUCCESS consumes (the equip voice
/// the onEquipItem walk rides — entity.pig.saddle the sounds.json
/// keeps, the swing the SUCCESS verdict starts, the
/// setItemSlot(SADDLE, stack.split(1)) the taller stack the tail
/// keeps — the one the split takes).
#[test]
fn the_right_press_saddles_the_pig() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut pig = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    pig.kind = crate::mob::MobKind::Pig;
    app.mobs.push(pig);
    assert_eq!(
        app.pick_mob(&app.mobs),
        Some(0),
        "the offline pig the verdict speaks"
    );
    // THE HELD STACK (the taller saddle the split(1) the tail keeps
    // — the one the walk takes)
    let saddle_id = mc_registry::item_id_by_name("saddle").expect("saddle the table draws");
    app.inventory.items[app.selected_hotbar] = (saddle_id, 2);
    assert!(
        app.on_right_press(0, 0),
        "the Saddle verdict consumed the press"
    );
    // THE EQUIP'S OWN WORD: the bit the setItemSlot arms, the voice
    // the onEquipItem walk plays, the tail the split keeps
    assert!(app.mobs[0].saddled, "the equipped SADDLE the bit arms");
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.pig.saddle ")),
        "the equip's own voice the lane carries"
    );
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (saddle_id, 1),
        "the split(1) the stack's tail keeps"
    );
    assert!(app.swing.swinging, "the press swung the arm");
}

/// THE SADDLED PIG'S OWN RIDE (javap Pig.mobInteract 14-54 — the
/// subclass's own first arm the 26.2 bytecode keeps): the empty
/// hand the isFood gate lets through, the isSaddled bit the
/// saddle arm armed — the Ride verdict the InteractionResult
/// .SUCCESS consumes (the startRiding the server arm the OfflineRide
/// arms — the PASSENGER attachment the fallback lambda speaks:
/// the FULL height the PIG_HEIGHT 0.9 carries, no boat's height/3;
/// the positionRider the body's feet the pig's own pose carries,
/// the yaw the rider's own look keeps), the shift edge the
/// dismount's own drop speaks.
#[test]
fn the_saddled_pig_takes_the_empty_hand_rider() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut pig = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    pig.kind = crate::mob::MobKind::Pig;
    app.mobs.push(pig);
    // THE SADDLE ARM (the count-1 stack the split the whole speaks
    // — the hand the (0,0) empty keeps, the bit the walk arms)
    let saddle_id = mc_registry::item_id_by_name("saddle").expect("saddle the table draws");
    app.inventory.items[app.selected_hotbar] = (saddle_id, 1);
    assert!(app.on_right_press(0, 0), "the saddle arm consumed");
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (0, 0),
        "the last saddle the split empties"
    );
    // THE RIDE ARM (the empty hand the held_id's None the sentinel
    // rides — the isFood false the gate opens)
    assert!(app.on_right_press(0, 0), "the Ride verdict consumed");
    let r = app.offline_ride.as_ref().expect("the ride belief armed");
    assert_eq!(r.vehicle_id, 7, "the pig's own id");
    assert_eq!(
        r.ride_height,
        crate::mob::PIG_HEIGHT as f32,
        "the FULL height the fallback lambda speaks"
    );
    assert_eq!(
        app.overlay_message.as_deref(),
        Some("Press Left Shift to Dismount"),
        "the mount's own overlay word"
    );
    // THE CARRY (the positionRider law the local mob's own pose
    // keeps): the body's feet the pig's pose + the FULL height, the
    // yaw the rider's own look keeps (the pig's deltaRotation the
    // zero the no-steering walk speaks), the body's physics sleep
    let px = c as f64 + 0.5;
    app.body = Some(crate::body::PlayerBody::new(
        px,
        top as f64 + crate::mob::PIG_HEIGHT,
        px,
        42.0,
        0.0,
    ));
    app.update();
    let b = app.body.as_ref().unwrap();
    // THE ATTACHMENT'S OWN F32 WORD (the ride_height the OfflineRide
    // keeps — the f64 the cast re-widens: the 0.899999976… the
    // f32's own word speaks, the exact fold the carry walks)
    let want_y = top as f64 + crate::mob::PIG_HEIGHT as f32 as f64;
    assert!(
        (b.y - want_y).abs() < 1.0e-9,
        "the carried feet the FULL height the attachment keeps (b.y {} vs want {})",
        b.y,
        want_y
    );
    assert_eq!(b.x, px, "the pig's own x the carry keeps");
    assert_eq!(b.z, px, "the pig's own z the carry keeps");
    assert_eq!(b.yaw, 42.0, "the rider's own look the pig never drags");
    assert_eq!(b.vx, 0.0, "the body's own sleep");
    // THE SHIFT EDGE (the dismount's own ask): the ride drops, the
    // body the carried pose keeps
    app.on_movement_input(0.0, 0.0, false, true, false);
    assert!(app.offline_ride.is_none(), "the edge's own drop");
}

/// THE FOOD HAND'S OWN GATE (javap Pig.mobInteract 0-11 — the
/// isFood ifne -> 55 the FOOD hand the super speaks FIRST: the
/// carrot the FEEDING lane answers, the ride the gate never arms
/// — the saddled hull the food hand the same law keeps).
#[test]
fn the_food_hand_feeds_the_saddled_pig() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut pig = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    pig.kind = crate::mob::MobKind::Pig;
    pig.saddled = true;
    app.mobs.push(pig);
    let carrot_id = mc_registry::item_id_by_name("carrot").expect("carrot the table draws");
    app.inventory.items[app.selected_hotbar] = (carrot_id, 3);
    assert!(
        app.on_right_press(0, 0),
        "the Feed verdict consumed the press"
    );
    // THE FEED'S OWN TRUTH: the love clock the 600 arms, the eat
    // voice the playEatingSound override speaks (the PIG only), the
    // ride the gate never arms, the stack the usePlayerItem's own
    // walk the food arm speaks — the consume the caller's lane
    // keeps
    assert_eq!(app.mobs[0].in_love, 600, "setInLove's own 600");
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.pig.eat ")),
        "the pig's own eat voice the override speaks"
    );
    assert!(
        app.offline_ride.is_none(),
        "the food hand the ride never arms"
    );
}

/// THE SECONDARY-USE STAND-DOWN (javap Pig.mobInteract 28-32 —
/// `player.isSecondaryUseActive()` ifne -> 55 the sneak the ride
/// arm refuses): the shift-held press the walk falls to the
/// super's miss — the PASS the item lane the walk returns, the
/// ride the belief never arms.
#[test]
fn the_shift_hand_mounts_nothing() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut pig = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    pig.kind = crate::mob::MobKind::Pig;
    pig.saddled = true;
    app.mobs.push(pig);
    app.shift_down = true;
    assert!(
        !app.on_right_press(0, 0),
        "the sneak's stand-down the PASS keeps"
    );
    assert!(
        app.offline_ride.is_none(),
        "the ride the stand-down never arms"
    );
}

/// THE REINS' OWN WALK (javap Pig.getControllingPassenger +
/// tickRidden + travelRidden — the 26.2 bytecode the controlled pig
/// keeps): the carrot-on-a-stick the held stack speaks, the
/// TICKRIDDEN SNAP the pig's yaw the rider's own look carries, the
/// RIDDEN TRAVEL the Vec3(0,0,1) CONSTANT the input keeps (the pig
/// the FULL forward walks — the WASD the speed never steers, the
/// LOOK the direction IS) at the getRiddenSpeed 0.25*0.225 word,
/// the drag the ground law applies, the CARRY the pig's fresh pose
/// the rider's feet keeps, the WALK ANIMATION the legs swing.
#[test]
fn the_reins_walk_the_pig_forward() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut pig = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    pig.kind = crate::mob::MobKind::Pig;
    // THE SETTLED STAND (the ground the pig's own ticks keep — the
    // on_ground the drag's friction word reads: the BLOCK 0.546 the
    // ground law speaks, the AIR 0.8281 the never-ticked fall
    // keeps)
    pig.on_ground = true;
    app.mobs.push(pig);
    // THE SADDLE-AND-MOUNT ARC (the count-1 saddle the split
    // empties, the empty hand the ride the mount arms)
    let saddle_id = mc_registry::item_id_by_name("saddle").expect("saddle the table draws");
    app.inventory.items[app.selected_hotbar] = (saddle_id, 1);
    assert!(app.on_right_press(0, 0), "the saddle arm consumed");
    assert!(app.on_right_press(0, 0), "the ride arm consumed");
    // THE REINS (the carrot-on-a-stick the held stack speaks — the
    // controlling pass the gate opens)
    let reins_id = mc_registry::item_id_by_name("carrot_on_a_stick")
        .expect("carrot_on_a_stick the table draws");
    app.inventory.items[app.selected_hotbar] = (reins_id, 1);
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        top as f64 + crate::mob::PIG_HEIGHT as f32 as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    // THE FIRST TICK'S OWN WORD (the impulse the exact fold keeps:
    // a = 0.25 * 0.225 = 0.05625, the yaw 0 the z the cos carries,
    // the MOVE the pre-drag velocity walks, the drag the carried
    // word scales after)
    let a = crate::mob::PIG_MOVEMENT_SPEED * crate::mob::PIG_RIDDEN_SPEED_MULTIPLIER;
    let want_z = c as f64 + 0.5 + a;
    app.update();
    let pig = &app.mobs[0];
    assert_eq!(pig.yaw, 0.0, "the tickRidden snap the rider's look keeps");
    assert!(
        (pig.z - want_z).abs() < 1.0e-9,
        "the first tick's own move the pre-drag impulse walks (pig.z {} vs want {})",
        pig.z,
        want_z
    );
    assert!(
        (pig.vz - a * (crate::body::BLOCK_FRICTION * crate::body::GROUND_DRAG) as f64).abs()
            < 1.0e-9,
        "the drag the carried velocity scales (the f32 word the friction fold keeps)"
    );
    // THE CARRY (the positionRider the pig's FRESH pose keeps — the
    // vehicle-moves-then-passenger order the block's own shape
    // speaks) + THE WALK ANIMATION (the legs the dist speaks)
    let b = app.body.as_ref().unwrap();
    assert!(
        (b.z - pig.z).abs() < 1.0e-9,
        "the rider the pig's fresh pose carries"
    );
    assert!(
        (b.y - (top as f64 + crate::mob::PIG_HEIGHT as f32 as f64)).abs() < 1.0e-9,
        "the FULL height the attachment keeps"
    );
    assert!(
        pig.walk.speed > 0.0,
        "the walk animation the dist arms (the legs the swing speaks)"
    );
    // THE LOOK'S OWN STEER (the yaw 90 the x the −sin carries — the
    // pig the rider's own look turns, the WASD the speed never
    // steers)
    let b = app.body.as_mut().unwrap();
    b.yaw = 90.0;
    let want_x = app.mobs[0].x - a;
    app.update();
    let pig = &app.mobs[0];
    assert_eq!(pig.yaw, 90.0, "the pig the rider's own look turns");
    assert!(
        (pig.x - want_x).abs() < 1.0e-9,
        "the −sin fold the x carries (the look the direction IS)"
    );
}

/// THE GATE'S OWN MISS (javap Pig.getControllingPassenger — the
/// carrot-on-a-stick the held stack the gate REQUIRES: the bare
/// hand the controlling pass never names, the tickRidden the walk
/// never runs — the pig's own stand the pose keeps, the boost the
/// drain pauses).
#[test]
fn the_bare_hands_stand_the_pig() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut pig = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    pig.kind = crate::mob::MobKind::Pig;
    pig.saddled = true;
    pig.steering_boosting = true;
    pig.boost_time = 3;
    pig.boost_time_total = 140;
    app.mobs.push(pig);
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        top as f64 + crate::mob::PIG_HEIGHT as f32 as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    // THE BARE HAND (no stack the held lane speaks — the gate's
    // miss): the ride carries (the pig's pose the rider follows),
    // the WALK the pig's own stand keeps
    app.update();
    let pig = &app.mobs[0];
    assert_eq!(pig.x, c as f64 + 0.5, "the pig's own x the stand keeps");
    assert_eq!(pig.z, c as f64 + 0.5, "the pig's own z the stand keeps");
    assert_eq!(pig.vx + pig.vz, 0.0, "the impulse the gate never arms");
    // THE BOOST'S OWN PAUSE (the tickRidden the walk never ran —
    // the drain the un-controlled pig keeps)
    assert!(
        pig.steering_boosting && pig.boost_time == 3,
        "the boost the un-controlled pig pauses"
    );
}

/// THE CARROT'S OWN BOOST PRESS (the 26.2's data-driven item walk
/// the ride speaks): the ridden pig the vehicle answers — the
/// ItemBasedSteering.boost the window arms (the nextInt(841)+140
/// the Lcg roll keeps), the CONSUME the swing speaks; the re-press
/// the boosting bit refuses. The ISVEHICLE'S OWN REFUSE the mob
/// arm's fall-through keeps (the ridden hull the ride arm never
/// takes — the press the item lane's use walk answers).
#[test]
fn the_carrot_press_boosts_the_ridden_pig() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut pig = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    pig.kind = crate::mob::MobKind::Pig;
    pig.saddled = true;
    pig.on_ground = true;
    app.mobs.push(pig);
    // THE FLAT WALKWAY (the +Z corridor the boosted travel walks —
    // the floor the feet keep, the air the body crosses: the
    // terrain's own hills the walkway's own words replace)
    for dz in 0..8 {
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, top - 1, c + dz, crate::world::STONE);
        for dy in 0..3 {
            app.offline_world
                .as_mut()
                .unwrap()
                .set(c, top + dy, c + dz, crate::world::AIR);
        }
    }
    let reins_id = mc_registry::item_id_by_name("carrot_on_a_stick")
        .expect("carrot_on_a_stick the table draws");
    app.inventory.items[app.selected_hotbar] = (reins_id, 1);
    // THE BODY (the ride branch's own seat — the body block the
    // update's walk enters, the carry the rider's pose keeps; the
    // PITCH the body's own look word speaks — the world_camera the
    // body's pose rides once the body stands, the pick's ray the
    // DOWN look keeps)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        top as f64 + crate::mob::PIG_HEIGHT as f32 as f64,
        c as f64 + 0.5,
        0.0,
        90.0,
    ));
    // THE RIDE (the empty-hand... the carrot IS the held stack —
    // the ride arm the non-food held answers, the mount the belief
    // arms)
    assert!(app.on_right_press(0, 0), "the ride arm consumed");
    assert!(app.offline_ride.is_some(), "the ride belief armed");
    // THE BOOST PRESS (the ridden hull the mob arm's Ride verdict
    // the isVehicle refuse stands down — the item lane's use walk
    // the boost arms)
    assert!(
        app.on_right_press(0, 0),
        "the carrot's own use consumed the press"
    );
    let pig = &app.mobs[0];
    assert!(pig.steering_boosting, "the boost the item's use arms");
    assert!(
        (140..=980).contains(&pig.boost_time_total),
        "the nextInt(841)+140 the window keeps, got {}",
        pig.boost_time_total
    );
    assert!(app.swing.swinging, "the CONSUME the swing speaks");
    // THE RE-PRESS (the boosting bit the ItemBasedSteering.boost's
    // own false keeps — the press the PASS the walk returns)
    assert!(
        !app.on_right_press(0, 0),
        "the boosting bit the re-press refuses"
    );
    assert!(app.mobs[0].steering_boosting, "the first window stands");
    // THE BOOSTED TRAVEL (the factor the sin ramp speaks — the
    // accel the un-boosted word EXCEEDS: the window's ticks the
    // sin the positive keeps, the pig the faster walk rides)
    let a0 = crate::mob::PIG_MOVEMENT_SPEED * crate::mob::PIG_RIDDEN_SPEED_MULTIPLIER;
    let z0 = app.mobs[0].z;
    for _ in 0..5 {
        app.update();
    }
    let gained = app.mobs[0].z - z0;
    assert!(
        gained > 5.0 * a0,
        "the boost the un-boosted walk exceeds (gained {gained} vs {})",
        5.0 * a0
    );
}

/// THE RIDDEN PIG'S OWN FIRE DEATH (the ride branch's own event
/// consumption the loop's ridden-id skip demands — the vehicle's
/// events the ride branch speaks or they never play): the burning
/// dose the health kills, the DEATH VOICE the pig's own row plays,
/// the LOOT the death walk rolls at the pig — the cooked porkchop
/// the burning body's smelt fold speaks, the saddle the custom
/// drop rides — and the ride belief the death animation the
/// removal's own clock keeps (the eject the 20 the walk owns).
#[test]
fn the_ridden_pig_burns_its_own_loot() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut pig = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    pig.kind = crate::mob::MobKind::Pig;
    pig.saddled = true;
    pig.on_ground = true;
    pig.health = 0.5; // the one dose the fire kills on
    app.mobs.push(pig);
    let carried_y = top as f64 + crate::mob::PIG_HEIGHT as f32 as f64;
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        carried_y,
        c as f64 + 0.5,
        0.0,
        90.0,
    ));
    assert!(app.mount_offline_pig(7), "the ride armed");
    // THE BURN'S OWN DOSE (fire_ticks = 20 the interval's own
    // crossing — the hurt the health crosses, the death the event
    // speaks)
    app.mobs[0].fire_ticks = 20;
    app.update();
    let dead = app
        .mobs
        .iter()
        .find(|m| m.id == 7)
        .expect("the corpse the death animation keeps")
        .health
        <= 0.0;
    assert!(dead, "the dose the fire killed");
    // THE RIDE'S OWN DEATH VOICE (the kind's row the ride branch's
    // consumption plays — the pig's own death the zombie's groan
    // never names)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.pig.death ")),
        "the pig's own row the ride's death speaks"
    );
    // THE LOOT'S OWN WALK (the cooked meat the smelt fold speaks,
    // the saddle the custom drop rides, the raw the burning death
    // never names)
    let cooked = crate::mob::wire_id("cooked_porkchop").expect("the cooked rides the wire");
    let raw = crate::mob::wire_id("porkchop").expect("the raw rides the wire");
    let saddle = crate::mob::wire_id("saddle").expect("the saddle rides the wire");
    assert!(
        app.drops
            .iter()
            .any(|d| d.item == cooked && (1..=3).contains(&d.count)),
        "the cooked porkchop the burning death drops"
    );
    assert!(
        app.drops.iter().any(|d| d.item == saddle && d.count == 1),
        "the saddle the custom drop pops"
    );
    assert!(
        app.drops.iter().all(|d| d.item != raw),
        "the raw meat the burning body never drops"
    );
    // THE BELIEF THE DEATH ANIMATION KEEPS (the eject the removal's
    // own clock speaks — the rider the dying pose the animation
    // rides)
    assert!(app.offline_ride.is_some(), "the ride the animation holds");
}

/// THE HEN'S OWN EGG THE WORLD KEEPS (the gift table's zero
/// crossing the tick arms — the CHICKEN_EGG voice the lay spot
/// plays, the egg item the feet the spawnAtLocation law drops,
/// the count-1 entry the temperate alternatives gate picks, the
/// refill the same walk arms — the hen the 6000..12000 the next
/// lay waits).
#[test]
fn the_chicken_lays_the_egg_at_the_feet() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut hen = crate::mob::Zombie::new(9, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    hen.kind = crate::mob::MobKind::Chicken;
    hen.on_ground = true;
    hen.egg_time = 1; // the lay the crossing the next tick arms
    app.mobs.push(hen);
    let before = app.drops.len();
    app.update();
    let egg_id = crate::mob::wire_id("egg").expect("the egg rides the wire");
    assert!(
        app.drops[before..]
            .iter()
            .any(|d| d.item == egg_id && d.count == 1),
        "the egg the lay drops at the feet"
    );
    assert!(
        (6000..=12000).contains(&app.mobs[0].egg_time),
        "the refill the tail arms: {}",
        app.mobs[0].egg_time
    );
}

/// THE VEHICLE'S OWN EJECT (Entity.remove(RemovalReason.KILLED)'s
/// own ejectPassengers — the death clock's removal the 20 the walk
/// speaks): the gone mob the ride belief drops, the body the last
/// carried pose keeps, and the standing walk the next tick's own
/// gravity resumes (the rider the pig's death spot lands).
#[test]
fn the_vehicle_death_ejects_the_rider() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut pig = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    pig.kind = crate::mob::MobKind::Pig;
    pig.saddled = true;
    pig.on_ground = true;
    pig.health = 0.5;
    app.mobs.push(pig);
    let carried_y = top as f64 + crate::mob::PIG_HEIGHT as f32 as f64;
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        carried_y,
        c as f64 + 0.5,
        0.0,
        90.0,
    ));
    assert!(app.mount_offline_pig(7), "the ride armed");
    app.mobs[0].fire_ticks = 20;
    app.update();
    // THE DEATH CLOCK'S OWN WALK (the animation the 20 ticks ride —
    // the corpse the ride branch's own tick the clock walks)
    for _ in 0..25 {
        app.update();
    }
    assert!(
        app.offline_ride.is_none(),
        "the gone mob the ride belief drops"
    );
    assert!(
        !app.mobs.iter().any(|m| m.id == 7),
        "the corpse the removal walk clears"
    );
    // THE STANDING WALK'S OWN RESUME (the gravity the carried pose
    // pulls — the rider the pig's death spot lands, the ride
    // branch's zero-vy hold the standing tick never keeps)
    for _ in 0..3 {
        app.update();
    }
    let b = app.body.as_ref().expect("the body stands");
    assert!(
        b.y < carried_y - 0.2,
        "the gravity the eject's spot resumes (y {} vs carried {carried_y})",
        b.y
    );
}

/// THE PLAYER'S OWN DEATH DISMOUNT (the ServerPlayer's removal the
/// vanilla death speaks — the passenger entity discarded, the fresh
/// respawn the seat never inherits): the death crossing the ride
/// beliefs drops — both lanes the same word keeps — and the respawn
/// the spawn pose walks (the respawned body the pig's back never
/// re-mounts).
#[test]
fn the_player_death_drops_the_rides() {
    let mut app = App::new(5);
    app.enter_world();
    app.offline_ride = Some(crate::world_view::OfflineRide {
        vehicle_id: 7,
        seat_x: 0.0,
        ride_height: 0.9,
    });
    app.net_ride = Some(crate::world_view::WireRide {
        vehicle_id: 9,
        feet_x: 40.0,
        feet_y: 9.0,
        feet_z: 40.0,
        vehicle_yaw: 0.0,
        seat_x: 0.0,
        ride_height: 0.9,
    });
    // THE FATAL DOSE (the health the 0.0 crossing speaks)
    app.actually_hurt(100.0, HurtVoice::Plain, None, None);
    assert!(app.dead, "the dose the player kills");
    assert!(
        app.offline_ride.is_none(),
        "the offline hull the death discards"
    );
    assert!(app.net_ride.is_none(), "the wire hull the death discards");
    // THE RESPAWN'S OWN WALK (the vitals the reset speaks, the seat
    // the fresh body never inherits)
    app.respawn();
    assert!(!app.dead, "the respawn the vitals reset");
    assert!(app.offline_ride.is_none() && app.net_ride.is_none());
}

/// THE DEATH'S OWN XP (javap LivingEntity.dropExperience →
/// Player.getExperienceReward — the orbs the death spot wears,
/// the level-capped half the reward law speaks, the SPECTATOR the
/// zero the isSpectator gate answers, the fresh respawn the trio's
/// zero the new body's constructor rows speak).
#[test]
fn the_death_drops_the_xp_orbs_and_the_respawn_zeroes_the_trio() {
    let mut app = App::new(5);
    app.enter_world();
    app.spawn_offline_body();
    let (dx, dy, dz) = {
        let b = app.body.as_ref().expect("the body stands");
        (b.x, b.y, b.z)
    };
    // THE STAGED TRIO (the vitals test's own row): level 3, total
    // 42 — the reward `min(floor(42 * 0.5F), 7 * 3)` = 21 speaks
    app.xp = crate::xp::ExperienceStat {
        level: 3,
        progress: 0.25,
        total: 42,
    };
    // THE FATAL DOSE (the orbs the death spot wears — the
    // `awardWithDirection` split the flat 21 into [17, 3, 1])
    app.actually_hurt(100.0, HurtVoice::Plain, None, None);
    assert!(app.dead, "the dose the player kills");
    assert_eq!(app.orbs.len(), 3, "the split the bands speak");
    for o in &app.orbs {
        assert_eq!((o.x, o.y, o.z), (dx, dy, dz), "the body's own spot");
    }
    let values: Vec<i32> = app.orbs.iter().map(|o| o.value).collect();
    assert_eq!(values, vec![17, 3, 1], "the split the bands speak");
    assert_eq!(
        values.iter().sum::<i32>(),
        21,
        "the level-capped half the reward speaks"
    );
    // THE RESPAWN'S OWN ZERO (the fresh entity the vanilla respawn
    // re-creates — the bar the empty rows wear)
    app.respawn();
    assert_eq!(app.xp.level, 0, "the level the respawn zeroes");
    assert_eq!(app.xp.progress, 0.0, "the progress the respawn zeroes");
    assert_eq!(app.xp.total, 0, "the points the respawn zeroes");
    // THE PARTIAL'S OWN FALL (the level-0 body the sub-1 arm wears:
    // the reward `ceil(progress * needed * 0.5F)` the partial bar
    // answers — the zero-zero death the 0 both arms agree on)
    let mut app2 = App::new(5);
    app2.enter_world();
    app2.spawn_offline_body();
    let need = crate::xp::xp_needed_for_next_level(0);
    app2.xp = crate::xp::ExperienceStat {
        level: 0,
        progress: 0.5,
        total: 0,
    };
    app2.actually_hurt(100.0, HurtVoice::Plain, None, None);
    let sum2: i32 = app2.orbs.iter().map(|o| o.value).sum();
    assert_eq!(
        sum2,
        ((0.5 * need as f32 * 0.5).ceil()) as i32,
        "the partial the sub-1 arm answers"
    );
    assert!(sum2 > 0, "the partial the drop wears");
    // THE ZERO-ZERO DEATH (the fresh body the 0 both arms agree on
    // — the orb-less crossing the vanilla silence keeps)
    let mut app3 = App::new(5);
    app3.enter_world();
    app3.spawn_offline_body();
    app3.actually_hurt(100.0, HurtVoice::Plain, None, None);
    assert!(app3.dead, "the dose the player kills");
    assert!(app3.orbs.is_empty(), "the zero the reward speaks");
    // THE SPECTATOR'S OWN GATE (the isSpectator arm the reward
    // wears — the staged trio the zero the gate answers)
    let mut app4 = App::new(5);
    app4.enter_world();
    app4.spawn_offline_body();
    app4.game_mode = GameMode::Spectator;
    app4.xp = crate::xp::ExperienceStat {
        level: 3,
        progress: 0.25,
        total: 42,
    };
    app4.actually_hurt(f32::MAX, HurtVoice::GenericKill, None, None);
    assert!(app4.dead, "the genericKill the invulnerable gate bypasses");
    assert!(
        app4.orbs.is_empty(),
        "the spectator the zero the gate keeps"
    );
}

/// THE DEATH'S OWN INVENTORY DROP (javap Inventory.dropAll the
/// dropCustomDeathLoot consumer — the compartments' every
/// non-empty stack the THROW-RANDOMLY ItemEntity scatters at the
/// eye-minus-0.3 spot, the 40-tick pickup delay the setPickUpDelay
/// call wears, the container the walk empties, the fresh respawn
/// the empty rows re-answers).
#[test]
fn the_death_scatters_the_pockets_and_the_container_empties() {
    let mut app = App::new(5);
    app.enter_world();
    app.spawn_offline_body();
    let (dx, dz) = {
        let b = app.body.as_ref().expect("the body stands");
        (b.x, b.z)
    };
    // THE STAGED POCKETS: the hotbar trio, one armor stack, the
    // offhand — the three compartments the walk wears
    app.inventory.items[0] = (1, 64);
    app.inventory.items[5] = (748, 1);
    app.inventory.items[35] = (903, 5);
    app.inventory.armor[2] = (310, 1);
    app.inventory.offhand[0] = (861, 1);
    // THE FATAL DOSE (the scatters the death spot wears)
    app.actually_hurt(100.0, HurtVoice::Plain, None, None);
    assert!(app.dead, "the dose the player kills");
    assert_eq!(app.drops.len(), 5, "the stacks the scatters speak");
    let stacks: Vec<(i32, i32)> = app.drops.iter().map(|d| (d.item, d.count)).collect();
    assert_eq!(
        stacks,
        vec![(1, 64), (748, 1), (903, 5), (310, 1), (861, 1)],
        "the compartments' order the walk wears"
    );
    for d in &app.drops {
        assert_eq!(d.pickup_delay, 40, "the death's own grace");
        assert_eq!(d.vy, 0.2, "the throw's own lift");
        assert_eq!(d.x, dx, "the x the body wears");
        assert_eq!(d.z, dz, "the z the body wears");
        let spread = d.vx * d.vx + d.vz * d.vz;
        assert!(
            spread <= 0.25,
            "the f the 0.5 cap the magnitude's square speaks"
        );
    }
    // THE CONTAINER THE DEATH EMPTIED (dropAll's slot clears —
    // the respawn's fresh entity the empty pockets re-answers)
    assert_eq!(
        app.inventory,
        crate::drops::Inventory::default(),
        "the rows the walk empties"
    );
    app.respawn();
    assert!(!app.dead, "the respawn the vitals reset");
    assert_eq!(
        app.inventory,
        crate::drops::Inventory::default(),
        "the fresh entity the empty pockets wears"
    );
    // THE ZERO-POCKET DEATH (the empty compartments the silence
    // keeps — the drop-less crossing the xp orb the only spawn)
    let mut app3 = App::new(5);
    app3.enter_world();
    app3.spawn_offline_body();
    app3.actually_hurt(100.0, HurtVoice::Plain, None, None);
    assert!(app3.dead, "the dose the player kills");
    assert!(app3.drops.is_empty(), "the empty rows the silence keeps");
}

/// THE KEEPINVENTORY RULE'S OWN SPARE (javap GameRules.RULE_KEEPIVENTORY
/// the death lanes' gates read — the reward the zero keeps, the
/// scatters never walk, the respawn's fresh entity the trio +
/// pockets the rule re-copies: the death screen the arms the
/// rule's own death still answers).
#[test]
fn the_keep_inventory_rule_spares_the_death_rows() {
    let mut app = App::new(5);
    app.enter_world();
    app.spawn_offline_body();
    app.keep_inventory = true;
    // THE STAGED ROWS (the pockets the trio the rule spares)
    app.inventory.items[0] = (1, 64);
    app.inventory.armor[2] = (310, 1);
    app.xp = crate::xp::ExperienceStat {
        level: 3,
        progress: 0.25,
        total: 42,
    };
    // THE FATAL DOSE (the death the screen arms, the rows the
    // rule spares)
    app.actually_hurt(100.0, HurtVoice::Plain, None, None);
    assert!(app.dead, "the dose the player kills");
    assert!(app.orbs.is_empty(), "the reward the rule zeroes");
    assert!(app.drops.is_empty(), "the scatters the rule refuses");
    assert_eq!(
        app.inventory.items[0],
        (1, 64),
        "the pockets the rule spares"
    );
    assert_eq!(
        app.inventory.armor[2],
        (310, 1),
        "the armor the rule spares"
    );
    assert_eq!(app.xp.total, 42, "the trio the rule spares");
    // THE RESPAWN (the fresh entity the rows the rule re-copies —
    // the bar the spent nothing the death never wore)
    app.respawn();
    assert!(!app.dead, "the respawn the screen closes");
    assert_eq!(
        app.inventory.items[0],
        (1, 64),
        "the pockets the respawn re-wears"
    );
    assert_eq!(app.xp.total, 42, "the trio the respawn re-wears");
}

/// THE GAMERULE'S OWN ROUND TRIP (vanilla level.dat's GameRules
/// compound the String "true"/"false" values keep — the save the
/// rule's own row banks, the reload the death lanes' gate re-arms,
/// the flip the file's own latest truth re-wears, the elder file's
/// missing row the false default the boot wears).
#[test]
fn the_keep_inventory_rule_round_trips_the_save() {
    let dir = std::env::temp_dir().join(format!("mc2rs_keep_inv_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("world_edits.nbt");
    // THE ARMED SESSION'S SAVE (the row the rule's own truth banks)
    let mut app = App::new(3);
    app.offline_world_file = Some(path.clone());
    app.enter_world();
    app.keep_inventory = true;
    app.offline_world_file_save();
    // THE FILE'S OWN ROW (the String "true" the vanilla gamerule
    // shape wears)
    let bytes = std::fs::read(&path).expect("the flush the file writes");
    let (_, root) =
        mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the file the parse answers");
    if let mc_protocol::nbt::Nbt::Compound(rows) = &root {
        let row = rows
            .iter()
            .find(|(k, _)| k == "keepInventory")
            .expect("the rule's own row the save banks");
        assert_eq!(
            row.1,
            mc_protocol::nbt::Nbt::String("true".to_string()),
            "the String 'true' the vanilla gamerule shape wears"
        );
    }
    // THE RELOAD (the armed rule the fresh session re-banks — the
    // death lanes' gate the restart keeps)
    let mut app2 = App::new(3);
    app2.offline_world_file = Some(path.clone());
    app2.enter_world();
    assert!(app2.keep_inventory, "the armed rule the reload re-banks");
    // THE SECOND SAVE (the disarmed row the flip banks — the file
    // the session's own latest truth wears)
    app2.keep_inventory = false;
    app2.offline_world_file_save();
    let mut app3 = App::new(3);
    app3.offline_world_file = Some(path.clone());
    app3.enter_world();
    assert!(
        !app3.keep_inventory,
        "the disarmed rule the reload re-banks"
    );
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
}

/// THE FIRE LANES' OWN KIND ROWS (the loop's own consumption the
/// kind fix speaks): the burning cow's death the cow's own row
/// plays, the burning sheep's hurt the sheep's own row — the
/// zombie's hardcoded groan the animal lanes never play again.
#[test]
fn the_kind_voices_the_fire_rows() {
    let mut app = App::new(11);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        top as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    let mut cow = crate::mob::Zombie::new(7, c as f64 + 2.5, top as f64, c as f64 + 0.5, 3);
    cow.kind = crate::mob::MobKind::Cow;
    cow.on_ground = true;
    cow.health = 0.5; // the one dose the fire kills on
    cow.fire_ticks = 20;
    app.mobs.push(cow);
    let mut sheep = crate::mob::Zombie::new(8, c as f64 - 2.5, top as f64, c as f64 + 0.5, 4);
    sheep.kind = crate::mob::MobKind::Sheep;
    sheep.on_ground = true;
    sheep.fire_ticks = 20; // the hurt the dose speaks (the health holds)
    app.mobs.push(sheep);
    app.update();
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.cow.death ")),
        "the cow's own row the fire death speaks"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.sheep.hurt ")),
        "the sheep's own row the fire hurt speaks"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .all(|e| !e.starts_with("entity.zombie.")),
        "the zombie's row the animal lanes never play"
    );
}

/// THE FIST'S OWN KIND ROW (the attack lane's direct kill the kind
/// fix speaks): the punched cow's death the cow's own row plays —
/// the zombie's groan the animal's kill never names — and the
/// loot the cow's own table rolls (the beef the flesh never is).
#[test]
fn the_fist_kills_the_cow_its_own_row() {
    let mut app = App::new(13);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        top as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    let mut cow = crate::mob::Zombie::new(7, c as f64 + 3.0, top as f64 + 1.62, c as f64 + 0.5, 3);
    cow.kind = crate::mob::MobKind::Cow;
    cow.health = 0.2; // the one punch the charge kills on
    app.mobs.push(cow);
    // THE CROSSHAIR'S OWN AIM (the mid-height the ray walks)
    {
        let (tx, ty, tz) = {
            let m = &app.mobs[0];
            (m.x, m.y + crate::mob::ZOMBIE_HEIGHT * 0.5, m.z)
        };
        let b = app.body.as_mut().unwrap();
        let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
        let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
        let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
        b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees() as f32;
        b.pitch = -(dy / len3).asin().to_degrees() as f32;
    }
    assert_eq!(app.pick_mob(&app.mobs), Some(0), "the cow the reach keeps");
    app.attack_mob(0);
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.cow.death ")),
        "the cow's own row the punch speaks"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .all(|e| !e.starts_with("entity.zombie.")),
        "the zombie's groan the cow's kill never plays"
    );
    let beef = crate::mob::wire_id("beef").expect("the beef rides the wire");
    let flesh = crate::mob::wire_id("rotten_flesh").expect("the flesh rides the wire");
    assert!(
        app.drops.iter().any(|d| d.item == beef),
        "the beef the cow's own table rolls"
    );
    assert!(
        app.drops.iter().all(|d| d.item != flesh),
        "the flesh the cow's death never drops"
    );
}

/// THE FIST'S OWN KIND XP (the attack lane's direct kill the
/// kind's own base row speaks): the punched cow's kill spawns the
/// orbs the Animal override rolls — the 1..3 band the zombie's
/// flat 5 the animal kill never names — the award the touch's
/// grant never pre-empts (the xp.total the kill keeps at zero).
#[test]
fn the_fist_kills_the_cow_its_own_xp() {
    for seed in [13u64, 14, 15] {
        let mut app = App::new(seed);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            top as f64,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        let mut cow =
            crate::mob::Zombie::new(7, c as f64 + 3.0, top as f64 + 1.62, c as f64 + 0.5, 31);
        cow.kind = crate::mob::MobKind::Cow;
        cow.health = 0.2; // the one punch the charge kills on
        app.mobs.push(cow);
        {
            let (tx, ty, tz) = {
                let m = &app.mobs[0];
                (m.x, m.y + crate::mob::ZOMBIE_HEIGHT * 0.5, m.z)
            };
            let b = app.body.as_mut().unwrap();
            let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
            let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
            let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
            b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees() as f32;
            b.pitch = -(dy / len3).asin().to_degrees() as f32;
        }
        assert_eq!(app.pick_mob(&app.mobs), Some(0), "the cow the reach keeps");
        app.attack_mob(0);
        assert!(
            !app.orbs.is_empty(),
            "the kill's own award spawns the orbs (seed {seed})"
        );
        let total: i32 = app.orbs.iter().map(|o| o.value).sum();
        assert!(
            (1..=3).contains(&total),
            "the cow's own band the kill rolls: {total} (seed {seed})"
        );
        assert_eq!(
            app.xp.total, 0,
            "the grant rides the touch, not the kill (seed {seed})"
        );
    }
}

/// THE SWEEP'S OWN KIND XP (doSweepAttack's neighbor walk the
/// outcomes lane speaks): the swept cow's death rolls the same
/// base row — the orbs the Animal override answers — while the
/// surviving zombie target the kill's gate never awards (the
/// closed `lastHurtByPlayerMemoryTime` lane the live body keeps).
#[test]
fn the_sweep_kills_the_cow_its_own_xp() {
    let mut app = App::new(3);
    app.enter_world();
    crate::headless::seed_scene(
        &mut app,
        &crate::headless::SceneOpts {
            body: true,
            zombie: true,
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    clear_the_aim_corridor(&mut app);
    let idx = 0;
    // the stage's own geometry: the target exactly 2.0 out along
    // the facing (full health — the swept lane's own survivor),
    // the cow beside it 0.7 lateral (inside the inflated box +
    // the 9.0 ring, the 0.2 health the swept total kills)
    let (nx, ny, nz) = {
        let b = app.body.as_ref().unwrap();
        let vyaw = b.yaw as f64 * std::f64::consts::PI / 180.0;
        let fx = -vyaw.sin();
        let fz = vyaw.cos();
        let m = &mut app.mobs[idx];
        m.x = b.x + fx * 2.0;
        m.z = b.z + fz * 2.0;
        (m.x + fz * 0.7, m.y, m.z - fx * 0.7)
    };
    let mut cow = crate::mob::Zombie::new(2, nx, ny, nz, 311);
    cow.kind = crate::mob::MobKind::Cow;
    cow.health = 0.2;
    app.mobs.push(cow);
    // aim at the zombie's mid height (AFTER the staging sits)
    {
        let (tx, ty, tz) = {
            let m = &app.mobs[idx];
            (m.x, m.y + crate::mob::ZOMBIE_HEIGHT * 0.5, m.z)
        };
        let b = app.body.as_mut().unwrap();
        let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
        let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
        let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
        b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees() as f32;
        b.pitch = -(dy / len3).asin().to_degrees() as f32;
    }
    // the sword + the full charge + the grounded swing
    let sword = crate::mob::wire_id("iron_sword").expect("the sword rides the wire");
    app.inventory.items[app.selected_hotbar] = (sword, 1);
    app.attack_ticker = 20;
    app.on_attack();
    // the cow's death the sweep's own award answers
    assert!(
        app.mobs[1].health <= 0.0,
        "the swept total the 0.2 cow kills"
    );
    assert!(
        !app.orbs.is_empty(),
        "the swept kill's own award spawns the orbs"
    );
    let total: i32 = app.orbs.iter().map(|o| o.value).sum();
    assert!(
        (1..=3).contains(&total),
        "the cow's own band the swept kill rolls: {total}"
    );
    // the zombie target survived — the kill's gate the live body
    // never opens (no orbs the survivor's site speaks)
    assert!(app.mobs[0].health > 0.0, "the target the sword wounds");
    assert_eq!(
        app.mobs[0].id, app.mobs[idx].id,
        "the target the lane keeps"
    );
}

#[test]
fn the_right_press_feeds_the_offline_sheep() {
    // THE FOOD ARM'S OWN PRESS (the Animal.mobInteract walk the
    // offline prediction speaks): the wheat the held stack talks,
    // the ADULT arm the canFallInLove gate opens — the setInLove
    // the 600 arms, the SUCCESS_SERVER the press consumes (the
    // swing the SwingSource.SERVER starts, the EAT VOICE the
    // playEatingSound base keeps SILENT — the Sheep the override
    // never speaks, the jar's own law); the second press the
    // in-love adult keeps — the CONSUME the fallback speaks (the
    // press consumed, SwingSource.NONE no swing restarts)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/wheat"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut sheep = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    sheep.kind = crate::mob::MobKind::Sheep;
    app.mobs.push(sheep);
    assert_eq!(
        app.pick_mob(&app.mobs),
        Some(0),
        "the offline sheep the pick speaks"
    );
    assert!(
        app.on_right_press(0, 0),
        "the Feed verdict consumed the press"
    );
    // THE LOVE'S OWN TRUTH: the clock the 600 arms, the swing the
    // SERVER word starts, the loot lane NO drop speaks (the food
    // the wool's table never is)
    assert_eq!(app.mobs[0].in_love, 600, "setInLove's own 600");
    assert!(app.swing.swinging, "the press swung the arm");
    assert!(app.drops.is_empty(), "the feed the loot lane never speaks");
    // THE SILENCE: the Sheep the base's own empty playEatingSound
    // keeps — no voice event the lane carries
    assert!(
        !app.sounds.events.iter().any(|e| e.contains(".eat")),
        "the sheep's feed the silent law keeps"
    );
    // the swing's own clock walks to rest (the 6-tick duration)
    for _ in 0..8 {
        app.swing.tick();
    }
    assert!(!app.swing.swinging, "the swing the clock completes");
    // THE CONSUME FALLBACK: the in-love adult the canFallInLove
    // gate refuses — the press consumed, NO swing restarts, the
    // clock the fallback never changes
    assert!(
        app.on_right_press(0, 0),
        "the CONSUME verdict consumed the press"
    );
    assert!(!app.swing.swinging, "the CONSUME no swing starts");
    assert_eq!(
        app.mobs[0].in_love, 600,
        "the fallback the clock never changes (no ticks walked)"
    );
}

#[test]
fn the_golden_dandelion_press_locks_the_baby() {
    // THE AGEABLEMOB.MOBINTERACT'S OWN ARM the 26.2 full disasm
    // speaks — the REAL on_right_press the ENTITY case lands: the
    // canUseGoldenDandelion gate the held stack, the baby, the
    // drained timer reads, the AgeLock verdict the toggle+reset
    // walk speaks, the use voice the post-toggle pick plays, the
    // swing the SUCCESS starts
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/golden_dandelion"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut sheep = crate::mob::Zombie::new(9, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    sheep.kind = crate::mob::MobKind::Sheep;
    sheep.age = crate::mob::BABY_START_AGE + 1600;
    app.mobs.push(sheep);
    assert!(
        app.on_right_press(0, 0),
        "the AgeLock verdict consumed the press"
    );
    // THE VOICE: the playSound the static's own tail speaks — the
    // locked arm the USE event names
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.golden_dandelion.use ")),
        "the locked arm the use voice speaks"
    );
    assert!(app.swing.swinging, "the press swung the arm");
    // THE LOCK: the toggle arms, the re-baby the -24000 walks, the
    // window the 40 marks
    assert!(app.mobs[0].age_locked, "the lock the toggle arms");
    assert_eq!(
        app.mobs[0].age,
        crate::mob::BABY_START_AGE,
        "the re-baby the fresh walk arms"
    );
    assert_eq!(
        app.mobs[0].age_lock_particle_timer,
        crate::mob::AGE_LOCK_PARTICLE_WINDOW,
        "the window the toggle marks"
    );
    // THE UNLOCK PRESS: the drained window the re-use gate requires
    // — the settle walks the window empty, the second press the
    // unuse voice speaks. The camera re-pins the body the idle
    // walk may have moved (the stroll's own drift the 40 ticks
    // carry) — the aim the pick's own range needs.
    for _ in 0..40 {
        app.update();
    }
    {
        let m = &app.mobs[0];
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(m.x as f32, (m.y + 4.0) as f32, m.z as f32),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
    }
    assert!(
        app.on_right_press(0, 0),
        "the drained window the re-use arms"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.golden_dandelion.unuse ")),
        "the unlock arm the unuse voice speaks"
    );
    assert!(!app.mobs[0].age_locked, "the unlock the toggle walks back");
}

#[test]
fn the_feed_plays_the_pig_eat_voice() {
    // THE PLAYEATINGSOUND OVERRIDE the jar keeps — the PIG only:
    // the eat voice the sounds.json's own event speaks
    // (entity.pig.eat), the carrot the PIG_FOOD tag's own first
    // value talks, the SUCCESS_SERVER the press consumes
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/carrot"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut pig = crate::mob::Zombie::new(9, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    pig.kind = crate::mob::MobKind::Pig;
    app.mobs.push(pig);
    assert!(
        app.on_right_press(0, 0),
        "the Feed verdict consumed the press"
    );
    // THE VOICE: the playEatingSound the Pig's own override speaks
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.pig.eat ")),
        "the pig's eat voice the override carries"
    );
    assert!(app.swing.swinging, "the press swung the arm");
    assert_eq!(app.mobs[0].in_love, 600, "setInLove's own 600");
}

#[test]
fn the_grazes_own_2001_walks_the_burst_and_the_voice() {
    // THE EATBLOCKGOAL'S OWN LANDING the caller's 2001 walk now
    // speaks (javap `LevelEventHandler` case 2001 the dig lane's
    // completion speaks — the sound rides FIRST, then
    // `addDestroyBlockEffect`): the below arm's setBlock(DIRT) the
    // break voice + the 4x4x4 debris burst the PRE-swap state's
    // own particle tile carries — the ear the camera keeps, the
    // spawn the eaten cell's own fractions fold
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE PAD the bite reads: the below cell the GRASS arm's own
    // law keeps (the staging's own surface word)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::GRASS);
    let mut sheep = crate::mob::Zombie::new(9, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    sheep.kind = crate::mob::MobKind::Sheep;
    app.mobs.push(sheep);
    // THE STAGE: the goal's start walk the timer arms (the 40
    // window the canUse roll's stand-in keeps), the 36 updates the
    // consume's `== 4` landing walks
    app.mobs[0].eat_animation_tick = crate::mob::EAT_ANIMATION_TICKS;
    for _ in 0..36 {
        app.update();
    }
    // THE SWAP: the grass block the dirt speaks
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top - 1, c),
        crate::world::DIRT,
        "the below arm's own swap landed"
    );
    // THE BURST: the 64 debris the full cube's 4x4x4 grid keeps,
    // the grass's own particle tile (dirt — the model's particle
    // the table speaks), the spawn the eaten cell's own law folds
    let debris: Vec<_> = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Terrain { tile: 1 }))
        .collect();
    assert_eq!(debris.len(), 64, "the full cube's own 4x4x4 burst");
    let (fx, fy, fz) = (
        app.mobs[0].x.floor() as i32,
        app.mobs[0].y.floor() as i32,
        app.mobs[0].z.floor() as i32,
    );
    assert_eq!(
        (fx, fy - 1, fz),
        (c, top - 1, c),
        "the frozen feet the eaten cell keeps"
    );
    for p in &debris {
        assert!(
            (fx as f64..fx as f64 + 1.0).contains(&p.x)
                && ((fy - 1) as f64..(fy) as f64).contains(&p.y)
                && (fz as f64..fz as f64 + 1.0).contains(&p.z),
            "the debris rides the eaten cell: {} {} {}",
            p.x,
            p.y,
            p.z
        );
    }
    // THE VOICE: the break fold (v+1)/2, p*0.8 the grass block's
    // own row speaks — the gain the ear's own distance folds (the
    // expected the caller's own law computes)
    let entry = app
        .sounds
        .events
        .iter()
        .find(|e| e.starts_with("block.grass.break @"))
        .expect("the 2001 event's own break voice");
    let ear = app.body.as_ref().map(|b| (b.x, b.y + b.eye_height(), b.z));
    let dist = ear_distance(ear, c as f64 + 0.5, (top - 1) as f64 + 0.5, c as f64 + 0.5);
    let gain = crate::sound::attenuate(1.0, dist, crate::sound::voice_max_distance(1.0));
    assert!(
        entry.contains(&format!(" v{gain:.6} p0.800000")),
        "the fold + the distance law: {entry}"
    );
}

#[test]
fn the_bite_remeshes_the_eaten_cell_this_tick() {
    // THE BITE'S OWN RE-SPEAK (the eat-swaps lane's own freshness —
    // the deferred staleness the prior rounds named): the eaten
    // cell the mesh wears THIS tick — the batched shape the crops
    // arm speaks (the batch the single re-speak answers), the
    // next-edit staleness the gap closes. THE VERDICT: the quads
    // the current voxels' own re-mesh answers verbatim — the grass
    // the dirt's own tiles wear, the grid the relight refreshed
    // bakes.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE PAD the bite reads: the below cell the GRASS arm's own
    // law keeps (the staging's own surface word)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::GRASS);
    let mut sheep = crate::mob::Zombie::new(9, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    sheep.kind = crate::mob::MobKind::Sheep;
    app.mobs.push(sheep);
    // THE STAGE: the goal's start walk the timer arms (the same
    // window the graze's 2001 walk wears)
    app.mobs[0].eat_animation_tick = crate::mob::EAT_ANIMATION_TICKS;
    for _ in 0..36 {
        app.update();
    }
    // THE SWAP: the grass block the dirt speaks
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top - 1, c),
        crate::world::DIRT,
        "the below arm's own swap landed"
    );
    // THE FRESH MESH: the quads the current voxels' own re-mesh
    // answers — the bite the re-speak the swap tick already wore
    // (the stale lane the next edit never needed)
    let w = app.offline_world.as_ref().unwrap();
    let fresh = crate::mesher::mesh_world(w, None);
    assert_eq!(
        app.world_quads.as_ref().unwrap(),
        &fresh,
        "the bite the mesh the swap tick wears"
    );
}

#[test]
fn the_blast_remeshes_the_crater_this_tick() {
    // THE BLAST'S OWN RE-SPEAK (the explosion lane's own freshness —
    // the same deferred staleness the bite lane closes): the crater
    // the mesh wears THIS tick — the silent clears the fresh grid
    // the quads re-bake, the next-edit staleness the gap closes.
    // THE VERDICT: the quads the crater's own re-mesh answers
    // verbatim the boom tick keeps.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE PAD the blast reads: the stone cells the staged surface
    // wears — the boom the pad's own cells clear (the spot the
    // body's spawn lane never walks, the knockback the distance
    // the six cells refuses)
    let (px, pz) = (c + 6, c);
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=1 {
            for dz in -1..=1 {
                w.set(px + dx, top - 1, pz + dz, crate::world::STONE);
            }
        }
    }
    app.remesh_offline(); // the staged pad the baseline mesh wears
    let mut t = crate::mob::PrimedTnt::new(9, px as f64 + 0.5, top as f64, pz as f64 + 0.5, 7);
    t.fuse = 1; // the crossing's own test seed — one tick to the boom
    app.primed_tnts.push(t);
    app.update();
    assert!(app.primed_tnts.is_empty(), "the discard the boom speaks");
    // THE CRATER: the pad's center cell the blast clears (the
    // radius the surface pad reads)
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(px, top - 1, pz),
        crate::world::AIR,
        "the blast the pad's center clears"
    );
    // THE FRESH MESH: the quads the crater's own re-mesh answers —
    // the boom the re-speak the blast tick already wore
    let w = app.offline_world.as_ref().unwrap();
    let fresh = crate::mesher::mesh_world(w, None);
    assert_eq!(
        app.world_quads.as_ref().unwrap(),
        &fresh,
        "the crater the mesh the boom tick wears"
    );
}

#[test]
fn the_blast_the_bamboo_the_cascade_the_wears() {
    // THE BLAST'S OWN BAMBOO WALK (javap `BlockBehaviour.
    // onExplosionHit` pc 179-191 the 26.2 jar speaks — every
    // non-air exploded cell the flag-3 AIR setBlock tail answers,
    // the neighbor broadcast the bamboo's own updateShape arms):
    // the boom the tower's base clears, the surviving stalks the
    // scheduleTick lag banks, the one-per-tick crackle the
    // drain's own consumer answers — the bottom-up pop the
    // vanilla's own cadence keeps.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE TOWER the blast reads: the 16-tall column the max-height
    // law grows (the stalks the surface cell upward wear, the
    // grass floor the supports_bamboo row keeps)
    let (px, pz) = (c + 6, c);
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(px, top - 1, pz, crate::world::GRASS);
        for dy in 0..16 {
            w.set(px, top + dy, pz, crate::world::BAMBOO);
        }
    }
    app.remesh_offline();
    // THE TNT the tower's base owns (the thin plant the body's
    // collide never blocks — the fuse=1 test seed the crossing
    // arms): the blast's own center cell the base stalk wears
    let mut t = crate::mob::PrimedTnt::new(9, px as f64 + 0.5, top as f64, pz as f64 + 0.5, 7);
    t.fuse = 1;
    app.primed_tnts.push(t);
    app.update();
    assert!(app.primed_tnts.is_empty(), "the discard the boom speaks");
    {
        let w = app.offline_world.as_ref().unwrap();
        // THE CRATER THE BASE CARVES: the center cell the ray's own
        // t=0 start clears (the base stalk the blast's own cell
        // wears), the floor the blast's reach answers
        assert_eq!(
            w.get(px, top, pz),
            crate::world::AIR,
            "the base the blast clears"
        );
        // THE TOP THE REACH NEVER CLEARS: the stalks the ray's
        // 5.2-cap refuses (the distance the 16-tall tower banks —
        // 4.0 power's max draw 5.2 the cap speaks)
        assert_eq!(
            w.get(px, top + 15, pz),
            crate::world::BAMBOO,
            "the top the reach keeps"
        );
        // THE ONE LIVE ENTRY (the Set's own dedup the cleared
        // cells' walks keep): the lowest survivor the only
        // unsupported stalk the 6-neighbor broadcasts speak, the
        // scheduleTick lag the due frame+1 banks
        assert_eq!(
            w.bamboo_ticks.len(),
            1,
            "the ONE live entry the dedup keeps"
        );
        let lowest = (0..16)
            .find(|dy| w.get(px, top + dy, pz) == crate::world::BAMBOO)
            .map(|dy| top + dy)
            .expect("the survivors the reach banks");
        assert!(
            lowest > top,
            "the base the blast cleared, the survivor the prefix law keeps"
        );
        assert_eq!(
            w.bamboo_ticks[0],
            (app.frame as u64 + 1, px, lowest, pz),
            "the lowest survivor the lag arms"
        );
    }
    // THE ONE-PER-TICK CASCADE (the drain's own consumer the tick
    // walk answers): the next update the lowest survivor pops, the
    // higher stalks the lag keeps, the pop's own re-broadcast the
    // next level arms — the bottom-up crackle the vanilla's own
    // cadence speaks
    app.update();
    {
        let w = app.offline_world.as_ref().unwrap();
        let lowest = (0..16)
            .find(|dy| w.get(px, top + dy, pz) == crate::world::BAMBOO)
            .map(|dy| top + dy)
            .expect("the cascade the first pop still leaves");
        assert!(
            lowest > top + 1,
            "the first pop the lowest survivor cleared, the next the lag keeps"
        );
    }
    // THE WHOLE TOWER FALLS: the re-broadcast chain the drain's
    // own rhythm answers — the survivors the one-per-tick pops
    // clear (16 stalks the 16-tick ceiling the cadence caps)
    for _ in 0..20 {
        app.update();
    }
    let w = app.offline_world.as_ref().unwrap();
    for dy in 0..16 {
        assert_eq!(
            w.get(px, top + dy, pz),
            crate::world::AIR,
            "the fall the column clears"
        );
    }
    assert!(w.bamboo_ticks.is_empty(), "the queue the cascade drains");
}

#[test]
fn the_blast_the_stocked_stores_the_spill_the_entries_die() {
    // THE BLAST'S OWN SPILL (javap `BlockBehaviour.onExplosionHit`
    // pc 49-99 + pc 160-174 the 26.2 jar the blastflags_704 probe
    // banks — the loot walk the BLOCK_ENTITY param carries, the
    // stocked slots the getDrops walk releases; the
    // preRemoveSideEffects dropContents the blast path NEVER
    // calls): the boom the stocked chest's and the stocked
    // furnace's cells clear, the stores' stacks the item entities
    // the crater wears, the entries (and the words) the walk dies
    // with — the RESTART the spill never resurrects (the file the
    // death ride banks).
    let mut app = App::new(3);
    app.enter_world();
    let wire_id_of = |name: &str| {
        mc_registry::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == name)
            .expect("the item rides the wire") as i32
    };
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let (px, pz) = (c + 6, c);
    // THE LANE the blast reads: the cells the boom's own ray
    // crosses the honest air keeps (the pad tests' own stamping —
    // the generated terrain's variance the verdict never rides)
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -1..=1i32 {
            for dz in -1..=3i32 {
                w.set(px + dx, surface, pz + dz, crate::world::AIR);
            }
        }
        w.set(px, surface, pz, crate::world::CHEST);
        w.set(px, surface, pz + 2, crate::world::FURNACE);
    }
    // THE STOCKS (the full 27 the chest store wears, the trio +
    // words the furnace store wears — the dig/open lanes' own
    // formats)
    app.container_stores.insert(
        (px, surface, pz),
        vec![
            (0, 0),
            (wire_id_of("diamond"), 12),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ],
    );
    app.furnace_stores.insert(
        (px, surface, pz + 2),
        vec![(wire_id_of("raw_iron"), 5), (wire_id_of("coal"), 2), (0, 0)],
    );
    app.furnace_word_stores
        .insert((px, surface, pz + 2), [800, 1600, 100, 200]);
    // THE BOOM (the fuse=1 test seed — one tick the crossing arms;
    // the TNT the chest's own cell tops — the diamond the blast
    // verdict reads)
    let mut t =
        crate::mob::PrimedTnt::new(9, px as f64 + 0.5, (surface + 1) as f64, pz as f64 + 0.5, 7);
    t.fuse = 1;
    app.primed_tnts.push(t);
    app.update();
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(px, surface, pz),
            crate::world::AIR,
            "the chest the blast clears"
        );
        assert_eq!(
            w.get(px, surface, pz + 2),
            crate::world::AIR,
            "the furnace the blast clears"
        );
    }
    // THE ENTRIES THE WALK DIES WITH (the stores the keys drop,
    // the words the progress never resurrects)
    assert!(
        !app.container_stores.contains_key(&(px, surface, pz)),
        "the chest's entry the spill releases"
    );
    assert!(
        !app.furnace_stores.contains_key(&(px, surface, pz + 2)),
        "the furnace's trio the spill releases"
    );
    assert!(
        !app.furnace_word_stores.contains_key(&(px, surface, pz + 2)),
        "the furnace's words the death ride banks"
    );
    // THE STOCKED STACKS THE FLOOR WEARS (the spill's own chunks
    // the drops vec carries — the diamond 12 and the raw_iron 5 +
    // the coal 2 the stocked stores held)
    // THE CHUNK LAW'S OWN SPLIT (javap ContainerHelper.dropAllItems
    // the `min(nextInt(21) + 10, stackMax)` drain speaks — the 12
    // the 10..30 rolls the 10+2 pair the draw wears): the stacks
    // the chunks SUM to the stocked counts — the totals the spill
    // released, the chunks the roll's own split keeps
    let total = |app: &App, item: i32| -> i32 {
        app.drops
            .iter()
            .filter(|d| d.item == item)
            .map(|d| d.count)
            .sum()
    };
    assert_eq!(
        total(&app, wire_id_of("diamond")),
        12,
        "the chest's diamond the spill wears"
    );
    assert_eq!(
        total(&app, wire_id_of("raw_iron")),
        5,
        "the furnace's input the spill wears"
    );
    assert_eq!(
        total(&app, wire_id_of("coal")),
        2,
        "the furnace's fuel the spill wears"
    );
}

#[test]
fn the_blast_the_chest_half_the_survivor_the_single_wears() {
    // THE BLAST'S OWN PAIR UPDATE (the blastflags_704 probe's own
    // verdict — javap `BlockBehaviour.onExplosionHit` pc 187
    // `iconst_3` the 26.2 jar speaks: the flag-3 setBlock the
    // UPDATE_NEIGHBORS keeps, the neighbor ChestBlock.updateShape
    // walk the demote arms): the boom the LEFT half clears, the
    // survivor the SINGLE wears (the facing the byte keeps), the
    // survivor's own stock the world keeps (the spill the broken
    // half's own store alone releases — the near half's own blast
    // resistance the aimed rays the partner's reach refuses), the
    // reopen the single shape the 27 rows wears.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let (px, pz) = (c + 6, c);
    let right = (px, surface, pz);
    let left = (px - 1, surface, pz);
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -3..=1i32 {
            for dz in -1..=1i32 {
                w.set(px + dx, surface, pz + dz, crate::world::AIR);
            }
        }
        w.set(right.0, right.1, right.2, crate::world::CHEST);
        w.set_variant(
            right.0,
            right.1,
            right.2,
            crate::world::chest_variant(
                crate::world::CHEST_FACING_NORTH,
                crate::world::CHEST_TYPE_RIGHT,
            ),
        );
        w.set(left.0, left.1, left.2, crate::world::CHEST);
        w.set_variant(
            left.0,
            left.1,
            left.2,
            crate::world::chest_variant(
                crate::world::CHEST_FACING_NORTH,
                crate::world::CHEST_TYPE_LEFT,
            ),
        );
    }
    // THE STOCKS (the survivor's own row the RIGHT store banks,
    // the broken half's own stack the LEFT store banks)
    let mut left_store = vec![(0, 0); 27];
    left_store[1] = (925, 4); // the LEFT half's own row
    app.container_stores.insert(left, left_store);
    let mut right_store = vec![(0, 0); 27];
    right_store[3] = (926, 12); // the RIGHT half's own row
    app.container_stores.insert(right, right_store);
    // THE BOOM (the fuse=1 test seed — the TNT the ground ONE gap
    // cell west of the LEFT half: the near half the blast's own
    // aimed rays clear [the budget the 2.38 cost the weakest 2.8
    // draw keeps], the aimed rays the near chest's OWN 2.5
    // resistance the march attenuation speaks — (2.5+0.3)*0.3 —
    // the partner's reach refuses: the max 4.6 budget the full
    // 1.0-wide crossing the 3-4 samples the 2.52+ attenuation
    // wears DIES INSIDE the near chest, the far half the ray
    // never samples — the vanilla shield law the ray walk keeps)
    let mut t = crate::mob::PrimedTnt::new(
        9,
        (left.0 - 2) as f64 + 0.5,
        surface as f64,
        left.2 as f64 + 0.5,
        7,
    );
    t.fuse = 1;
    app.primed_tnts.push(t);
    app.update();
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(left.0, left.1, left.2),
            crate::world::AIR,
            "the broken half the blast clears"
        );
        assert_eq!(
            w.get(right.0, right.1, right.2),
            crate::world::CHEST,
            "the survivor the blast keeps"
        );
        // THE DEMOTE (the updateShape arm the flag-3 chain serves):
        // the survivor the SINGLE wears, the facing the byte keeps
        assert_eq!(
            crate::world::chest_variant_type(w.variant(right.0, right.1, right.2)),
            crate::world::CHEST_TYPE_SINGLE,
            "the survivor the SINGLE wears"
        );
        assert_eq!(
            crate::world::chest_variant_facing(w.variant(right.0, right.1, right.2)),
            crate::world::CHEST_FACING_NORTH,
            "the facing the byte keeps"
        );
    }
    // THE SURVIVOR'S OWN STOCK THE WORLD KEEPS (the vanilla law —
    // the spill the broken half's own store alone releases)
    assert!(
        !app.container_stores.contains_key(&left),
        "the broken half's entry the spill releases"
    );
    assert_eq!(
        app.container_stores.get(&right).map(|s| s[3]),
        Some((926, 12)),
        "the survivor's own store the world keeps"
    );
    // THE FRESH MESH (the variant bytes the demote changed — the
    // re-speak the boom tick already wore, the pad test's own
    // verdict the pair change wears)
    let w = app.offline_world.as_ref().unwrap();
    let fresh = crate::mesher::mesh_world(w, None);
    assert_eq!(
        app.world_quads.as_ref().unwrap(),
        &fresh,
        "the survivor's demote the mesh the boom tick wears"
    );
    // THE REOPEN (the single shape the 27 rows wears — the partner
    // key the demote takes)
    app.open_chest_for(right);
    assert_eq!(
        app.chest_slots.len(),
        27,
        "the survivor the single shape reopens"
    );
    assert!(
        app.container_open_cell_b.is_none(),
        "the partner key the demote takes"
    );
    assert_eq!(
        app.chest_slots[3],
        (926, 12),
        "the survivor's stock the reopen wears"
    );
}

// THE DAY BAKE'S OWN PAIR LAW (the helper the re-bake check reads —
// the SAME expressions the mesher's fold speaks, quantized to the
// re-walk's grain): the clockless world the full-day step and the
// white track keep, the noon plateau the same pair, the night
// plateau the 0.24 factor's 31st step and the #7a7aff moonlight
// bytes, the dusk ramp the interpolated pair.
#[test]
fn the_day_bake_inputs_wear_the_timeline_tracks() {
    let mut w = crate::world::World::new(1, 0);
    // THE CLOCKLESS BAKE: the fresh world's None the full-day pair
    // keeps — the exact pair the enter_world seed banks
    assert_eq!(day_bake_inputs(&w), (128, [255, 255, 255]));
    // THE NOON PLATEAU: factor 1.0, the white track — the day
    // goldens' own pair (the staged tick the day plateau keeps)
    w.day_tick = Some(6_030);
    assert_eq!(day_bake_inputs(&w), (128, [255, 255, 255]));
    // THE NIGHT PLATEAU: the 0.24 factor's 31st step, the moonlight
    // bytes (#7a7aff — the plateau the golden night tick wears)
    w.day_tick = Some(18_000);
    assert_eq!(day_bake_inputs(&w), (31, [122, 122, 255]));
    // THE DUSK RAMP: the interpolated factor's step and the rounded
    // moonlight-walk bytes (the 12205 tick the byte-sampler law pins)
    w.day_tick = Some(12_205);
    let (step, tint) = day_bake_inputs(&w);
    assert_eq!(tint, [189, 189, 255], "the ramp's byte pair");
    assert_eq!(
        step,
        (crate::timeline::sky_light_factor(12_205) * 128.0).round() as u32,
        "the factor's own 1/128 step"
    );
}

// THE OFFLINE CLOCK'S OWN WIRE + THE NIGHT RE-BAKE (the render
// walk's law): the app's clock the offline world's day_tick answers,
// the day plateau's pair the seed keeps — the mesh never re-walks —
// and the worldnight tick's step the re-bake answers (a fresh
// allocation wearing the moonlight pair, the terrain the 0.24 fold
// dims).
#[test]
fn the_offline_clock_rebakes_the_night_terrain() {
    let mut app = App::new(3);
    app.enter_world();
    let day_ptr = app.world_quads.as_ref().map(|q| q.as_ptr());
    assert_eq!(
        app.day_bake,
        Some((128, [255, 255, 255])),
        "the seed the clockless fold's pair banks"
    );
    // THE NOON RENDER: the day plateau's pair the seed already
    // wears — the SAME allocation the plateau's no-op keeps
    app.frame = 30;
    app.clock_total = 6_000 + 30;
    let mut fb = crate::raster::Framebuf::new(64, 64);
    let mut assets = GuiAssets::open(assets_root());
    app.render_world_3d(&mut fb, &mut assets);
    assert_eq!(app.day_bake, Some((128, [255, 255, 255])));
    assert_eq!(
        app.world_quads.as_ref().map(|q| q.as_ptr()),
        day_ptr,
        "the plateau never re-walks"
    );
    // THE CLOCK'S WIRE: the render walk's tick the offline world
    // wears (the app's own clock — the integrated-server fiction)
    assert_eq!(app.offline_world.as_ref().unwrap().day_tick, Some(6_030));
    // THE NIGHT RENDER: the worldnight tick's step the re-bake
    // answers — a fresh allocation, the moonlight pair banked
    // (the frame's 30 the staged render keeps — 14_010 + 30 the
    // old fold's total the owned clock banks directly)
    app.clock_total = 14_040;
    let mut fb2 = crate::raster::Framebuf::new(64, 64);
    app.render_world_3d(&mut fb2, &mut assets);
    assert_eq!(app.day_bake, Some((31, [122, 122, 255])));
    assert_ne!(
        app.world_quads.as_ref().map(|q| q.as_ptr()),
        day_ptr,
        "the step the mesh re-walks"
    );
    assert_eq!(app.offline_world.as_ref().unwrap().day_tick, Some(14_040));
}

// THE MESHER'S OWN NIGHT FOLD (the day_tick the mesh consumes): the
// same voxels re-meshed at the night tick wear the dimmer colors —
// the moonlight fold the sky term's 0.24 factor speaks, the block
// term the torch's own light keeps.
#[test]
fn the_mesh_bakes_the_clocks_night_fold() {
    let mut w = crate::world::World::new(2, 42); // the REAL terrain
    w.relight();
    let day = crate::mesher::mesh_world(&w, None);
    w.day_tick = Some(18_000);
    let night = crate::mesher::mesh_world(&w, None);
    assert_eq!(day.len(), night.len(), "the same voxels the same quads");
    // THE FOLD'S OWN MONOTONE: every vertex the night fold dims or
    // keeps (the sky term's 0.24 factor and the moonlight tint the
    // white track never exceeds; the block term the night keeps)
    let mut dimmed = 0usize;
    for (d, n) in day.iter().zip(night.iter()) {
        for (dc, nc) in d.colors.iter().zip(n.colors.iter()) {
            for ch in 0..3 {
                assert!(
                    nc[ch] <= dc[ch] + 1e-6,
                    "the night fold never brightens: {nc:?} vs {dc:?}"
                );
            }
        }
        if n.colors[0] != d.colors[0] {
            dimmed += 1;
        }
    }
    // THE OUTDOOR'S OWN DARK: the terrain's sunlit faces the 0.24
    // fold dims — the sky-lit quads the night never keeps
    assert!(dimmed > 0, "the moonlight fold the terrain wears");
}

// THE NET CLOCK'S OWN GUARD (the render walk's wiring the wire lane
// never touches): the networked world's day_tick the server's Set
// Time word keeps, the parked offline world's clock the net screen
// never wires.
#[test]
fn the_net_clocks_wire_the_server_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    app.world_is_network = true;
    let mut net = crate::world::World::new(1, 0);
    net.day_tick = Some(12_345); // the server's own Set Time word
    app.net_world = Some(net);
    app.frame = 30;
    app.clock_total = 14_040;
    let mut fb = crate::raster::Framebuf::new(64, 64);
    let mut assets = GuiAssets::open(assets_root());
    app.render_world_3d(&mut fb, &mut assets);
    // THE SERVER'S WORD: the wire world's clock the Set Time value
    // keeps — the app's own clock the wire lane never overwrites
    assert_eq!(app.net_world.as_ref().unwrap().day_tick, Some(12_345));
    // THE PARKED WORLD'S OWN SILENCE: the offline clock the guard
    // never wires (the net screen the offline lane never speaks)
    assert_eq!(app.offline_world.as_ref().unwrap().day_tick, None);
}

#[test]
fn the_feed_press_spawns_the_event18_burst() {
    // THE BROADCAST'S OWN BURST (the setInLove walk the press's
    // verdict carries — the (byte)18 the handleEntityEvent arm
    // answers with the SEVEN hearts, each at the body's own
    // registration box the getRandomX/getRandomY law spreads); the
    // CONSUME fallback the burst never speaks
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/wheat"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut sheep = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    sheep.kind = crate::mob::MobKind::Sheep;
    app.mobs.push(sheep);
    assert!(
        app.on_right_press(0, 0),
        "the Feed verdict consumed the press"
    );
    // THE SEVEN: the kind the heart lane speaks, the flat 16, the
    // box spread, the gaussian*0.02 scale
    assert_eq!(
        app.particles.len(),
        7,
        "the handleEntityEvent arm's own count"
    );
    let bw = crate::mob::SHEEP_WIDTH / 2.0;
    let bh = crate::mob::SHEEP_HEIGHT;
    let cx = c as f64 + 0.5;
    let cz = c as f64 + 0.5;
    for p in &app.particles {
        assert!(
            matches!(p.kind, crate::particles::ParticleKind::Heart),
            "the love's own kind"
        );
        assert_eq!(p.lifetime, 16, "the flat 16 the 26.2 speaks");
        assert!(
            (p.x - cx).abs() <= bw + 1e-9,
            "the ±half-width spread: {}",
            p.x
        );
        assert!(
            (p.z - cz).abs() <= bw + 1e-9,
            "the ±half-width spread: {}",
            p.z
        );
        assert!(
            p.y >= top as f64 + 0.5 - 1e-9 && p.y <= top as f64 + bh + 0.5 + 1e-9,
            "the height fraction + 0.5: {}",
            p.y
        );
        assert!(
            p.vx.abs() < 0.3 && p.vy.abs() < 0.3 && p.vz.abs() < 0.3,
            "the gaussian * 0.02 scale"
        );
    }
    // THE CONSUME FALLBACK: the second press the in-love adult
    // keeps — no burst the fallback speaks
    let before = app.particles.len();
    assert!(
        app.on_right_press(0, 0),
        "the CONSUME verdict consumed the press"
    );
    assert_eq!(app.particles.len(), before, "the fallback no burst speaks");
}

#[test]
fn the_love_clock_walks_the_ambient_hearts() {
    // THE AISTEP'S OWN DRIP (javap `Animal.aiStep`: the tick's
    // decrement the love clock walks, the ONE heart the
    // post-decrement gate spawns while in love)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/wheat"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut sheep = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    sheep.kind = crate::mob::MobKind::Sheep;
    app.mobs.push(sheep);
    assert!(
        app.on_right_press(0, 0),
        "the Feed verdict consumed the press"
    );
    assert_eq!(app.particles.len(), 7, "the burst the press speaks");
    for _ in 0..3 {
        app.update();
    }
    // the burst + the ONE-per-tick drip the three updates walked
    assert_eq!(
        app.particles.len(),
        7 + 3,
        "the aiStep gate one heart per tick"
    );
    assert!(app.mobs[0].in_love > 0, "the clock the 600 arms walks");
    // every heart the young one the 16-tick life keeps
    assert!(
        app.particles
            .iter()
            .all(|p| matches!(p.kind, crate::particles::ParticleKind::Heart) && p.age < p.lifetime),
        "the hearts the lifetime walk keeps"
    );
}

/// THE TAME MACHINE THE APP FRAME SPEAKS (the press the
/// Wolf.mobInteract walk rides end to end): the bone press the
/// verdict consumes — the stack the consume(1) shrinks, the
/// SEVEN the verdict's own burst wears (the success the HEART
/// arm, the failure the SMOKE arm — the kind the tame bit's own
/// word pairs), the SERVER swing the verdict starts; the tamed
/// body's own press the SIT toggle serves (no burst the toggle
/// never speaks, the tick's goal fold the pose IS).
#[test]
fn the_tame_machine_the_app_frame_speaks() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut wolf = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    wolf.kind = crate::mob::MobKind::Wolf;
    app.mobs.push(wolf);
    // THE BONE HAND (the held stack the tame arm's own gate reads)
    let bone_id = mc_registry::item_id_by_name("bone").expect("bone the table draws");
    app.inventory.items[app.selected_hotbar] = (bone_id, 5);
    app.offline_held_updated(Some("item/bone"));
    // THE PRESS (the verdict the roll speaks — either arm the
    // frame's own walk serves)
    assert!(app.on_right_press(0, 0), "the tame walk consumed the press");
    // THE STACK'S OWN SHRINK (the consume(1) the taller stack
    // speaks — the offline lane's own inventory walk)
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (bone_id, 4),
        "the bone the press spends"
    );
    // THE BURST (SEVEN the spawnTamingParticles count keeps — the
    // kind the tame bit's own word pairs: the success the HEART,
    // the failure the SMOKE, exactly one the verdict wears)
    assert_eq!(app.particles.len(), 7, "the taming burst the count keeps");
    let tamed = app.mobs[0].tamed;
    let hearts = app
        .particles
        .iter()
        .all(|p| matches!(p.kind, crate::particles::ParticleKind::Heart));
    let smoke = app
        .particles
        .iter()
        .all(|p| matches!(p.kind, crate::particles::ParticleKind::Smoke));
    assert!(hearts != smoke, "exactly one burst kind the verdict speaks");
    assert_eq!(
        hearts, tamed,
        "the success the HEART wears, the failure the SMOKE"
    );
    assert!(app.swing.swinging, "the SUCCESS_SERVER the swing starts");
    // THE TAMED BODY'S OWN PRESS (the tame walk's own sit order
    // the toggle re-flips — the SUCCESS the setOrderedToSit(true)
    // armed, the second press the STAND-DOWN wears, the third the
    // order re-arms; the pose the goal's fold the order IS)
    app.particles.clear();
    // THE RE-CENTER (the stroll the update may arm — the crosshair
    // the body's own box the press re-picks)
    app.mobs[0].x = c as f64 + 0.5;
    app.mobs[0].z = c as f64 + 0.5;
    if tamed {
        assert!(
            app.mobs[0].ordered_to_sit,
            "the tame walk's own tail the order armed"
        );
        assert!(
            app.on_right_press(0, 0),
            "the stand-down consumed the press"
        );
        assert!(
            app.particles.is_empty(),
            "the toggle the burst never speaks"
        );
        assert!(!app.mobs[0].ordered_to_sit, "the re-press the order drops");
        app.update();
        assert!(
            !app.mobs[0].in_sitting_pose,
            "the stood-down body the pose drops"
        );
        // THE THIRD PRESS (the order the flip re-arms — the pose
        // the next tick's fold wears; the body the crosshair's
        // own column re-centers)
        app.mobs[0].x = c as f64 + 0.5;
        app.mobs[0].z = c as f64 + 0.5;
        assert!(app.on_right_press(0, 0), "the re-arm consumed the press");
        assert!(app.mobs[0].ordered_to_sit, "the order the press re-arms");
        app.update();
        assert!(app.mobs[0].in_sitting_pose, "the goal's fold the pose IS");
    } else {
        // the untamed body the SECOND bone the same walk re-rolls —
        // the verdict whatever the stream speaks, the SEVEN the
        // burst count keeps
        let before = app.particles.len();
        assert!(
            app.on_right_press(0, 0),
            "the second bone consumed the press"
        );
        assert_eq!(
            app.particles.len() - before,
            7,
            "the re-rolled burst the count keeps"
        );
    }
}

#[test]
fn the_offline_arm_rides_when_the_chain_speaks_it() {
    // THE ONE-LIST WALK's own split: the offline body NEARER the
    // wire's — the chain index lands the offline side, the local
    // prediction rides (the health the client's own to mutate),
    // the Attack packet the press never speaks (the Swing the only
    // beat the lane carries).
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let n = app.world_size as f64;
    let mut body =
        crate::body::PlayerBody::new(n / 2.0 + 0.5, app.spawn_y as f64, n / 2.0 + 0.5, -90.0, 0.0);
    body.on_ground = true;
    app.body = Some(body);
    let (bx, bz) = {
        let b = app.body.as_ref().unwrap();
        (b.x, b.z)
    };
    // the wire body at 1.8 (the crosshair stage's own t), the
    // offline body NEARER at 1.5 — the chain's own order rides
    app.live_mobs_updated(vec![wire_mob_named(
        100,
        bx + 1.8,
        app.spawn_y as f64,
        bz,
        "minecraft:zombie",
    )]);
    app.mobs.push(crate::mob::Zombie::new(
        7,
        bx + 1.5,
        app.spawn_y as f64,
        bz,
        3,
    ));
    let wire_health_before = app.net_mobs[0].health;
    let offline_health_before = app.mobs[0].health;
    let (tx, rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    app.on_attack();
    // THE OFFLINE ARM: the local prediction the health drop speaks;
    // the wire body the untouched mirror
    assert!(
        app.mobs[0].health < offline_health_before,
        "the offline body the local prediction owns"
    );
    assert_eq!(
        app.net_mobs[0].health, wire_health_before,
        "the wire body the offline prediction never touches"
    );
    // THE LANE: the Swing the only command — the Attack packet the
    // offline arm never mints
    assert_eq!(
        rx.try_recv().unwrap(),
        crate::world_view::ContainerCommand::Swing,
        "the offline arm the Swing alone speaks"
    );
    assert!(
        rx.try_recv().is_err(),
        "no Attack packet the offline press sends"
    );
}

/// THE DODAYLIGHTCYCLE RULE'S OWN FREEZE LAW (the RULE_DAYLIGHT
/// row the clock's advance gate reads): the false the per-tick day
/// step the walk never earns — the world's own ticks the frame
/// keeps (the pause law's own shape), the manual `/time set` the
/// frozen walk still re-anchors, the re-armed rule the advance
/// resumes.
#[test]
fn the_do_daylight_cycle_rule_freezes_the_day_clock() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE BASELINE WALK (the default TRUE the 1-tick step earns)
    app.chat_open = true;
    app.chat_input = "/time set 6000".into();
    app.on_chat_submit();
    assert_eq!(
        app.day_time_now(),
        6000,
        "the manual write the walk pre-arms"
    );
    for _ in 0..10 {
        app.update();
    }
    assert_eq!(app.day_time_now(), 6010, "the default walk the step earns");
    // THE DISARM (the rule the set walk flips — the chat literal's
    // own arm): the day the freeze never earns, the frame the
    // world's own beat still speaks
    app.chat_open = true;
    app.chat_input = "/gamerule doDaylightCycle false".into();
    app.on_chat_submit();
    assert!(!app.do_daylight_cycle, "the set the rule disarms");
    for _ in 0..10 {
        app.update();
    }
    assert_eq!(app.day_time_now(), 6010, "the frozen day the rule keeps");
    assert_eq!(app.frame, 20, "the world's own ticks the rule never gates");
    // THE MANUAL WRITE UNDER THE FREEZE (the frozen walk the set
    // arm still re-anchors — the rule the advance gates, the
    // manual lane the write keeps; the freeze the write never
    // thaws)
    app.chat_open = true;
    app.chat_input = "/time set 7000".into();
    app.on_chat_submit();
    assert_eq!(
        app.day_time_now(),
        7000,
        "the manual write the freeze never gates"
    );
    for _ in 0..10 {
        app.update();
    }
    assert_eq!(
        app.day_time_now(),
        7000,
        "the frozen day the write never thaws"
    );
    // THE RE-ARM (the rule the set walk flips back — the advance
    // the walk resumes)
    app.chat_open = true;
    app.chat_input = "/gamerule doDaylightCycle true".into();
    app.on_chat_submit();
    assert!(app.do_daylight_cycle, "the set the rule re-arms");
    for _ in 0..10 {
        app.update();
    }
    assert_eq!(
        app.day_time_now(),
        7010,
        "the re-armed walk the step resumes"
    );
}

/// THE DODAYLIGHTCYCLE RULE'S OWN ROUND TRIP (vanilla level.dat's
/// GameRules compound the String "true"/"false" values keep — the
/// save the day clock's own row banks, the reload the advance gate
/// re-arms, the flip the file's own latest truth re-wears, the
/// elder file's missing row the TRUE default the boot wears).
#[test]
fn the_do_daylight_cycle_rule_round_trips_the_save() {
    let dir = std::env::temp_dir().join(format!("mc2rs_daylight_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("world_edits.nbt");
    // THE DISARMED SESSION'S SAVE (the row the rule's own truth banks)
    let mut app = App::new(3);
    app.offline_world_file = Some(path.clone());
    app.enter_world();
    app.do_daylight_cycle = false;
    app.offline_world_file_save();
    // THE FILE'S OWN ROW (the String "false" the vanilla gamerule
    // shape wears)
    let bytes = std::fs::read(&path).expect("the flush the file writes");
    let (_, root) =
        mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the file the parse answers");
    if let mc_protocol::nbt::Nbt::Compound(rows) = &root {
        let row = rows
            .iter()
            .find(|(k, _)| k == "doDaylightCycle")
            .expect("the rule's own row the save banks");
        assert_eq!(
            row.1,
            mc_protocol::nbt::Nbt::String("false".to_string()),
            "the String 'false' the vanilla gamerule shape wears"
        );
    }
    // THE RELOAD (the disarmed gate the fresh session re-banks — the
    // day clock the restart keeps frozen)
    let mut app2 = App::new(3);
    app2.offline_world_file = Some(path.clone());
    app2.enter_world();
    assert!(
        !app2.do_daylight_cycle,
        "the disarmed rule the reload re-banks"
    );
    // THE SECOND SAVE (the armed row the flip banks — the file the
    // session's own latest truth wears)
    app2.do_daylight_cycle = true;
    app2.offline_world_file_save();
    let mut app3 = App::new(3);
    app3.offline_world_file = Some(path.clone());
    app3.enter_world();
    assert!(app3.do_daylight_cycle, "the armed rule the reload re-banks");
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
}

/// THE DOMOBSPAWNING RULE'S OWN ROUND TRIP (vanilla level.dat's
/// GameRules compound the String "true"/"false" values keep — the
/// save the spawner's own row banks, the reload the natural gate
/// re-arms, the flip the file's own latest truth re-wears, the
/// elder file's missing row the TRUE default the boot wears).
#[test]
fn the_do_mob_spawning_rule_round_trips_the_save() {
    let dir = std::env::temp_dir().join(format!("mc2rs_mobspawn_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("world_edits.nbt");
    // THE DISARMED SESSION'S SAVE (the row the rule's own truth banks)
    let mut app = App::new(3);
    app.offline_world_file = Some(path.clone());
    app.enter_world();
    app.do_mob_spawning = false;
    app.offline_world_file_save();
    // THE FILE'S OWN ROW (the String "false" the vanilla gamerule
    // shape wears)
    let bytes = std::fs::read(&path).expect("the flush the file writes");
    let (_, root) =
        mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the file the parse answers");
    if let mc_protocol::nbt::Nbt::Compound(rows) = &root {
        let row = rows
            .iter()
            .find(|(k, _)| k == "doMobSpawning")
            .expect("the rule's own row the save banks");
        assert_eq!(
            row.1,
            mc_protocol::nbt::Nbt::String("false".to_string()),
            "the String 'false' the vanilla gamerule shape wears"
        );
    }
    // THE RELOAD (the disarmed gate the fresh session re-banks — the
    // natural attempts the restart keeps refusing)
    let mut app2 = App::new(3);
    app2.offline_world_file = Some(path.clone());
    app2.enter_world();
    assert!(
        !app2.do_mob_spawning,
        "the disarmed rule the reload re-banks"
    );
    // THE SECOND SAVE (the armed row the flip banks — the file the
    // session's own latest truth wears)
    app2.do_mob_spawning = true;
    app2.offline_world_file_save();
    let mut app3 = App::new(3);
    app3.offline_world_file = Some(path.clone());
    app3.enter_world();
    assert!(app3.do_mob_spawning, "the armed rule the reload re-banks");
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
}

/// THE CHEST OPEN'S OWN FILL BANK (the autopush-294 round — the
/// container lane the open walk the load's fill banks: javap 26.2
/// the formula the bank wears — ChestBlock.getAnalogOutputSignal the
/// getContainer(combine) + getRedstoneSignalFromContainer pair
/// answers, the AbstractContainerMenu.getRedstoneSignalFromContainer
/// pc 0-75 the `f / getContainerSize()` FULL-slot divide +
/// `Mth.lerpDiscrete(f, 0, 15)` the `from + floor(delta * (range-1))
/// + (delta > 0 ? 1 : 0)` fold speaks [the ONE full stack the 1/27
/// the fill's 1 banks, the 27 the f=1 the fill's 15 speaks, the
/// empty the 0 the same override keeps] — the RESTART the map's
/// empty truth the open arm re-banks [the container_fill the
/// world's own field the journal never replays], the comparator the
/// fresh fill the back read answers, the ENDER the jar's own
/// silence the sync's lane refuses).
#[test]
fn the_chest_open_own_fill_bank_wears() {
    use crate::world as w;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (kx, ky, kz) = (c, top - 1, c); // the comparator (facing EAST)
    let (cx, cy, cz) = (c + 1, top - 1, c); // the chest (the back cell)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(kx, ky, kz, w::COMPARATOR);
        world.set_variant(kx, ky, kz, w::comparator_variant(1, false, false, 0));
        world.set(cx, cy, cz, w::CHEST);
    }
    // THE FIRST OPEN'S OWN EMPTY BANK (the fresh store the or_insert
    // 27 empties the fill's 0 the override keeps — the comparator
    // the through-wall probe the container answer never rides)
    app.open_chest_for((cx, cy, cz));
    assert_eq!(app.screen, ScreenKind::Chest);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.container_fill.get(&(cx, cy, cz)),
            Some(&0),
            "the fresh open the empty store the 0 banks"
        );
    }
    app.close_chest_container(ScreenKind::World);
    // THE STORE'S OWN SEED + THE RESTART'S OWN WIPE (the one full
    // stone stack the f=1/27 the fill's 1 speaks; the wipe the
    // honest restart simulation — the container_fill the field the
    // journal never replays, the ghost-0 the reopen the bank restores)
    {
        let mut store = vec![(0, 0); 27];
        store[0] = (1, 64);
        app.container_stores.insert((cx, cy, cz), store);
    }
    app.offline_world.as_mut().unwrap().container_fill.clear();
    // THE REOPEN'S OWN BANK (the open arm the load's fill re-banks —
    // the comparator the fresh fill the back read answers, the
    // refresh the powered verdict the nibble wears)
    app.open_chest_for((cx, cy, cz));
    assert_eq!(app.screen, ScreenKind::Chest);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.container_fill.get(&(cx, cy, cz)),
            Some(&1),
            "the reopen the stored stack the fill's 1 banks"
        );
        assert_eq!(
            crate::world::comparator_output_signal(world, kx, ky, kz),
            1,
            "the comparator the fresh fill the back read answers"
        );
        let fresh = crate::world::comparator_refresh_variant(world, kx, ky, kz);
        assert!(fresh.is_some(), "the fill the verdict the refresh arms");
        let fresh = fresh.unwrap();
        assert_eq!(
            crate::world::comparator_output_of(fresh),
            1,
            "the nibble the output the fill wears"
        );
        assert!(
            crate::world::comparator_powered_of(fresh),
            "the nonzero input the powered bit speaks"
        );
    }
    app.close_chest_container(ScreenKind::World);
    // THE FULL STORE'S OWN 15 (the 27 full stacks the f=1 the max
    // speaks — the lerpDiscrete the delta 1 the floor's 14 + 1 the
    // 15 banks)
    {
        let full = vec![(1, 64); 27];
        app.container_stores.insert((cx, cy, cz), full);
    }
    app.open_chest_for((cx, cy, cz));
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.container_fill.get(&(cx, cy, cz)),
            Some(&15),
            "the full store the f=1 the fill's 15 banks"
        );
        assert_eq!(
            crate::world::comparator_output_signal(world, kx, ky, kz),
            15,
            "the comparator the max fill the output wears"
        );
    }
    app.close_chest_container(ScreenKind::World);
    // THE ENDER'S OWN SILENCE (the shared open the sync's lane the
    // refusal keeps — the stale entry the map the absence restores)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(cx, cy, cz, w::ENDER_CHEST);
        world.container_fill.insert((cx, cy, cz), 9);
    }
    app.open_chest_for((cx, cy, cz));
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            !world.container_fill.contains_key(&(cx, cy, cz)),
            "the ender the silence the stale entry the map drops"
        );
    }
}

/// THE BOOT'S OWN FILL RE-DERIVE (the dev-189 round — the vanilla chunk
/// load's own law the BE restore the signal's live compute wears: the
/// restart the stocked chest's comparator the ghost-0 the fresh
/// world's empty map the once-spent parse rides the gap wore, the
/// enter walk the restored stores the SAME open-arm law every cell
/// re-banks — the single the own grid, the double the union's mate
/// both halves the one total, the furnace the trio, the dispenser the
/// nine, the ender the silence, the foreign cell the air-arm drops)
#[test]
fn the_boot_own_fill_rederive_wears() {
    use crate::world as w;
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let dir = std::env::temp_dir().join(format!("mc2rs_boot_fill_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let world_path = dir.join("world_edits.nbt");
    let stores_path = dir.join("container_stores.nbt");
    // THE FIRST SESSION (the blocks the journal banks, the stocks the
    // containers file wears — the two saves the restart's own disk
    // rows speak)
    let mut app = App::new(3);
    app.offline_world_file = Some(world_path.clone());
    app.container_stores_file = Some(stores_path.clone());
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (kx, ky, kz) = (c, top - 1, c); // the comparator (facing EAST)
    let (cx, cy, cz) = (c + 1, top - 1, c); // the single chest (the back cell)
    let (fx, fy, fz) = (c + 2, top - 1, c); // the furnace
    let (px, py, pz) = (c + 3, top - 1, c); // the dispenser
    let (ex, ey, ez) = (c + 4, top - 1, c); // the ender cell
    let (sx, sy, sz) = (c + 5, top - 1, c); // the plain stone (the foreign key)
                                            // THE DOUBLE'S OWN PAIR (the EAST-facing pair the +z walk banks —
                                            // the LEFT half the south cell answers)
    let (dx, dy, dz) = (c, top - 1, c + 2);
    let (dx2, dy2, dz2) = (c, top - 1, c + 3);
    let face = 1u8;
    let (a_var, b_var) = (
        face | (w::CHEST_TYPE_LEFT << 2),
        face | (w::CHEST_TYPE_RIGHT << 2),
    );
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(kx, ky, kz, w::COMPARATOR);
        world.set_variant(kx, ky, kz, w::comparator_variant(1, false, false, 0));
        world.set(cx, cy, cz, w::CHEST);
        world.set(fx, fy, fz, w::FURNACE);
        world.set(px, py, pz, w::DISPENSER);
        world.set(ex, ey, ez, w::ENDER_CHEST);
        world.set(sx, sy, sz, w::STONE);
        world.set(dx, dy, dz, w::CHEST);
        world.set_variant(dx, dy, dz, a_var);
        world.set(dx2, dy2, dz2, w::CHEST);
        world.set_variant(dx2, dy2, dz2, b_var);
    }
    // THE STORES' OWN STAGE (the session's stocks the maps wear — the
    // one full stack the f=1/27 the fill's 1 speaks, the trio the 1/3
    // the 5, the nine the f=1/9 the kind's own heal the 2 wears)
    let mut chest_store = vec![(0, 0); 27];
    chest_store[0] = (stone, 64);
    app.container_stores.insert((cx, cy, cz), chest_store);
    let mut trio = vec![(0, 0); 3];
    trio[0] = (stone, 64);
    app.furnace_stores.insert((fx, fy, fz), trio);
    let mut nine = vec![(0, 0); 9];
    nine[0] = (stone, 64);
    app.container_stores.insert((px, py, pz), nine);
    // the ender + the foreign keys the honest absence rows stage
    app.container_stores.insert((ex, ey, ez), vec![(0, 0); 27]);
    app.container_stores.insert((sx, sy, sz), vec![(0, 0); 27]);
    let mut half_a = vec![(0, 0); 27];
    half_a[0] = (stone, 64);
    app.container_stores.insert((dx, dy, dz), half_a);
    app.container_stores
        .insert((dx2, dy2, dz2), vec![(0, 0); 27]);
    app.offline_world_file_save();
    app.container_stores_file_save();
    // THE RESTART'S OWN FIRST TICK (the update walk the lazy parse the
    // world-less ride speaks — the stores the file extends, the fill
    // rides the no-op the offline world the None the sync's early
    // return keeps: the once-flag the spend the gap's own shape wears)
    let mut app2 = App::new(3);
    app2.offline_world_file = Some(world_path.clone());
    app2.container_stores_file = Some(stores_path.clone());
    app2.container_stores_load_once();
    assert!(
        app2.container_stores.contains_key(&(cx, cy, cz)),
        "the parse the stocks extends"
    );
    assert!(
        app2.container_stores_loaded,
        "the spent flag the first tick wears"
    );
    // THE ENTER WALK (the fresh world the journal replay wears — the
    // re-derive the stores the file restored the SAME open-arm law
    // every cell the fill re-banks)
    app2.enter_world();
    {
        let world = app2.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(cx, cy, cz),
            w::CHEST,
            "the chest the replay wears"
        );
        assert_eq!(
            world.get(dx, dy, dz),
            w::CHEST,
            "the double's half the replay wears"
        );
        assert_eq!(
            world.container_fill.get(&(cx, cy, cz)),
            Some(&1),
            "the stocked chest the fill's 1 re-banks"
        );
        assert_eq!(
            crate::world::comparator_output_signal(world, kx, ky, kz),
            1,
            "the comparator the back read the boot answers"
        );
        assert_eq!(
            world.container_fill.get(&(fx, fy, fz)),
            Some(&5),
            "the stocked trio the fill's 5 banks"
        );
        assert_eq!(
            world.container_fill.get(&(px, py, pz)),
            Some(&2),
            "the nine the kind's own heal the restart wears — the f=1/9 the floor(1.55)+1 the honest 2 banks (the parse's 27-pad the denominator the dev-190 round retires)"
        );
        // THE DOUBLE'S OWN UNION (the ONE stock the BOTH halves the
        // same fill banks — the mate the walk's own ride wears)
        assert_eq!(
            world.container_fill.get(&(dx, dy, dz)),
            Some(&1),
            "the double's stocked half the fill banks"
        );
        assert_eq!(
            world.container_fill.get(&(dx2, dy2, dz2)),
            Some(&1),
            "the double's empty mate the SAME fill banks"
        );
        // THE ENDER'S OWN SILENCE + THE FOREIGN CELL'S OWN DROP (the
        // keys the walk the verdicts the honest absences keep)
        assert!(
            !world.container_fill.contains_key(&(ex, ey, ez)),
            "the ender key the silence keeps"
        );
        assert!(
            !world.container_fill.contains_key(&(sx, sy, sz)),
            "the foreign key the air-arm drops"
        );
    }
    let _ = std::fs::remove_file(&world_path);
    let _ = std::fs::remove_file(&stores_path);
    let _ = std::fs::remove_dir(&dir);
}

/// THE STORES' OWN PER-KIND CAP (the dev-190 round — the parse's own
/// 27-cap the kind's own size retires: the vanilla
/// `ContainerHelper.loadAllItems` walk the load the BE's own
/// `getContainerSize()` governs — the list beyond the size the
/// Math.min DROP keeps, the DispenserBlockEntity/DropperBlockEntity
/// the `NonNullList.withSize(9, EMPTY)` the nine slots speak, the
/// ChestBlockEntity's 27 the grid keeps. THE POLLUTED SHAPE the old
/// kind-blind file wore: the dispenser row the 27-pad the parse
/// padded, the fill formula's denominator the 27 the restart wore
/// [the f=1/27 the 1 the honest f=1/9 the 2 never banks], the GUI's
/// open the 27-row the canvas never speaks). THE HEAL the both
/// orders the row truncates: the world-less parse the superset the
/// tolerant keep rides, the enter walk the blocks the kind's own
/// cap answers — the next save the file's own widths the heal wears.
#[test]
fn the_stores_own_per_kind_cap_wears() {
    use crate::world as w;
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let dir = std::env::temp_dir().join(format!("mc2rs_kind_cap_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let world_path = dir.join("world_edits.nbt");
    let stores_path = dir.join("container_stores.nbt");
    // THE FIRST SESSION (the blocks the journal banks, the rows the
    // containers file wears — the dispenser row the POLLUTED 27-pad
    // the old kind-blind save spoke, the dropper the honest nine)
    let mut app = App::new(3);
    app.offline_world_file = Some(world_path.clone());
    app.container_stores_file = Some(stores_path.clone());
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (px, py, pz) = (c, top - 1, c); // the dispenser
    let (rx, ry, rz) = (c + 1, top - 1, c); // the dropper
    let (cx, cy, cz) = (c + 2, top - 1, c); // the chest
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(px, py, pz, w::DISPENSER);
        world.set(rx, ry, rz, w::DROPPER);
        world.set(cx, cy, cz, w::CHEST);
    }
    // the polluted 27-wide dispenser row (the slots 9..26 the old
    // parse's pads wear — the stocked slot 0 the same stack rides)
    let mut padded = vec![(0, 0); 27];
    padded[0] = (stone, 64);
    app.container_stores.insert((px, py, pz), padded);
    // the honest nine the fresh open's own shape keeps
    let mut nine = vec![(0, 0); 9];
    nine[0] = (stone, 64);
    app.container_stores.insert((rx, ry, rz), nine);
    // the chest grid the 27 the menu law wears
    let mut grid = vec![(0, 0); 27];
    grid[0] = (stone, 64);
    app.container_stores.insert((cx, cy, cz), grid);
    app.offline_world_file_save();
    app.container_stores_file_save();
    // THE RESTART'S OWN PARSE (the world-less ride the superset keeps
    // — the 27-cap the every row pads, the kind the boot walk never
    // reads, the heal the world's absence the honest no-op wears)
    let mut app2 = App::new(3);
    app2.offline_world_file = Some(world_path.clone());
    app2.container_stores_file = Some(stores_path.clone());
    app2.container_stores_load_once();
    assert_eq!(
        app2.container_stores.get(&(px, py, pz)).map(Vec::len),
        Some(27),
        "the world-less parse the 27-superset keeps"
    );
    // THE ENTER WALK'S OWN HEAL (the blocks the kind's own cap answer
    // — the dispenser family's nine the pad truncates, the chest grid
    // the 27 keeps, the dropper's honest row the no-op wears)
    app2.enter_world();
    assert_eq!(
        app2.container_stores.get(&(px, py, pz)).map(Vec::len),
        Some(9),
        "the dispenser row the kind's own nine truncates"
    );
    assert_eq!(
        app2.container_stores.get(&(rx, ry, rz)).map(Vec::len),
        Some(9),
        "the dropper's honest row the heal never doubles"
    );
    assert_eq!(
        app2.container_stores.get(&(cx, cy, cz)).map(Vec::len),
        Some(27),
        "the chest grid the 27 the menu law keeps"
    );
    {
        let world = app2.offline_world.as_ref().unwrap();
        assert_eq!(
            world.container_fill.get(&(px, py, pz)),
            Some(&2),
            "the dispenser's fill the f=1/9 the honest 2 banks"
        );
        assert_eq!(
            world.container_fill.get(&(rx, ry, rz)),
            Some(&2),
            "the dropper's fill the same nine law banks"
        );
        assert_eq!(
            world.container_fill.get(&(cx, cy, cz)),
            Some(&1),
            "the chest's fill the f=1/27 the 1 keeps"
        );
    }
    // THE GUI'S OWN CANVAS (the open the healed row clones — the
    // dispenser screen the menu's own nine speaks, the 27-pad the
    // canvas never wears)
    app2.open_dispenser_for((px, py, pz));
    assert_eq!(app2.screen, ScreenKind::Dispenser);
    assert_eq!(
        app2.dispenser_slots.len(),
        9,
        "the canvas the menu's own nine the open clones"
    );
    assert_eq!(app2.dispenser_slots[0], (stone, 64), "the stock rides");
    app2.close_dispenser(ScreenKind::World);
    // THE FILE'S OWN HEAL (the close flush the honest widths the disk
    // wears — the third session the parse the kind's own shape keeps)
    let mut app3 = App::new(3);
    app3.offline_world_file = Some(world_path.clone());
    app3.container_stores_file = Some(stores_path.clone());
    app3.container_stores_load_once();
    app3.enter_world();
    assert_eq!(
        app3.container_stores.get(&(px, py, pz)).map(Vec::len),
        Some(9),
        "the disk row the healed width keeps"
    );
    let _ = std::fs::remove_file(&world_path);
    let _ = std::fs::remove_file(&stores_path);
    let _ = std::fs::remove_dir(&dir);
}

/// THE TRANSFER'S OWN LIVE FILL RIDE (the autopush-295 round — javap
/// 26.2 the per-mutation law the chain proves: Slot.set pc 0-18 the
/// `container.setItem` + `setChanged` pair the GUI click speaks,
/// BaseContainerBlockEntity.setItem pc 0-23 the `getItems().set` +
/// `limitSize` + `setChanged` walk the chest BE inherits [the
/// ChestBlockEntity method table the override never banks],
/// BlockEntity.setChanged(Level, BlockPos, BlockState) pc 0-21 the
/// `!isAir` gate the `updateNeighbourForOutputSignal(pos, block)`
/// walk serves, Level.updateNeighbourForOutputSignal pc 0-130 the
/// HORIZONTAL plane the direct comparator the neighborChanged
/// re-read answers, the one-conductor seat the second probe the
/// same walk serves): the MID-OPEN transfer the comparator the LIVE
/// fill re-reads — the grid place AND the take the ride the beat
/// serves, the quick-move the CHEST-side delta the same lane wears,
/// the drag's painted seats the release beat answers, the
/// inventory-side seat the store untouched the silence keeps, the
/// close flush the ONLY lane the stale law never keeps.
#[test]
fn the_chest_transfer_own_live_fill_ride_wears() {
    use crate::world as w;
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    let (kx, ky, kz) = (c, top - 1, c); // the comparator (facing EAST)
    let (cx, cy, cz) = (c + 1, top - 1, c); // the chest (the back cell)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(kx, ky, kz, w::COMPARATOR);
        world.set_variant(kx, ky, kz, w::comparator_variant(1, false, false, 0));
        world.set(cx, cy, cz, w::CHEST);
    }
    // THE OPEN'S OWN EMPTY BANK (the fresh store the fill's 0 — the
    // mid-open beats the comparator the LIVE law answers)
    app.open_chest_for((cx, cy, cz));
    assert_eq!(app.screen, ScreenKind::Chest);
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .container_fill
            .get(&(cx, cy, cz)),
        Some(&0),
        "the fresh open the empty store the 0 banks"
    );
    // THE INVENTORY-SIDE PICKUP THE FILL NEVER BANKS (the hotbar row
    // the stack rides the cursor — the chest's store UNTOUCHED, the
    // comparator the stale 0 keeps: the vanilla setItem the chest BE
    // alone speaks)
    app.inventory.items[0] = (stone, 64);
    app.chest_mirror_sync_from_inventory();
    app.chest_slot_rects = vec![(100, 100, 54)];
    app.mouse_pos = Some((108, 108));
    app.click_chest_slot(100, 100, 0);
    assert_eq!(app.chest_carried, (stone, 64), "the take rides");
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .container_fill
            .get(&(cx, cy, cz)),
        Some(&0),
        "the inventory-side seat the fill never banks"
    );
    // THE GRID SEAT'S OWN PLACE (the stack lands the chest — the
    // MID-OPEN beat the ride wears: the fill's 1 the comparator the
    // back read answers, the close flush the ONLY lane the stale law
    // never keeps)
    app.chest_slot_rects = vec![(160, 100, 0)];
    app.mouse_pos = Some((168, 108));
    app.click_chest_slot(160, 100, 0);
    assert_eq!(app.chest_slots[0], (stone, 64), "the stack landed");
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.container_fill.get(&(cx, cy, cz)),
            Some(&1),
            "the mid-open place the fill's 1 the ride banks"
        );
        assert_eq!(
            crate::world::comparator_output_signal(world, kx, ky, kz),
            1,
            "the comparator the live fill the back read answers"
        );
    }
    // THE GRID SEAT'S OWN TAKE (the same lane the whole stack rides
    // back — the fill the dead 0 the mid-open beat restores)
    app.chest_slot_rects = vec![(160, 100, 0)];
    app.mouse_pos = Some((168, 108));
    app.click_chest_slot(160, 100, 0);
    assert_eq!(app.chest_slots[0], (0, 0), "the stack rode back");
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .container_fill
            .get(&(cx, cy, cz)),
        Some(&0),
        "the mid-open take the fill the dead 0 restores"
    );
    // THE QUICK-MOVE'S OWN HOME RIDE (the shift-click the inventory
    // stack the chest hands — the CHEST-side delta the ride answers,
    // the cursor the lane never touches the carried keeps)
    app.inventory.items[1] = (stone, 64);
    app.chest_mirror_sync_from_inventory();
    app.shift_down = true;
    app.chest_slot_rects = vec![(100, 100, 55)];
    app.mouse_pos = Some((108, 108));
    app.click_chest_slot(100, 100, 0);
    assert_eq!(
        app.chest_carried,
        (stone, 64),
        "the quick-move the cursor never touches"
    );
    let seat = app
        .chest_slots
        .iter()
        .position(|&p| p == (stone, 64))
        .expect("the quick-move the grid seat lands");
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.container_fill.get(&(cx, cy, cz)),
            Some(&1),
            "the quick-move's chest-side delta the ride banks"
        );
        assert_eq!(
            crate::world::comparator_output_signal(world, kx, ky, kz),
            1,
            "the comparator the quick-move beat the live read wears"
        );
    }
    // THE QUICK-MOVE'S OWN OUT RIDE (the shift-click the grid seat
    // the inventory hands back — the fill the dead 0 the beat
    // restores)
    app.chest_slot_rects = vec![(160, 100, seat as i16)];
    app.mouse_pos = Some((168, 108));
    app.click_chest_slot(160, 100, 0);
    assert_eq!(
        app.chest_slots[seat],
        (0, 0),
        "the quick-move out the grid empties"
    );
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .container_fill
            .get(&(cx, cy, cz)),
        Some(&0),
        "the quick-move out the fill the dead 0 restores"
    );
    app.shift_down = false;
    // THE DRAG'S OWN PAINTED SEATS (the quickcraft the carried
    // spreads — the press the arm speaks, the motion the seats
    // paint, the release the floor split distributes, the
    // chest_touched gate the ride the release beat serves)
    app.chest_slot_rects = vec![(160, 100, 1), (200, 100, 2)];
    app.chest_panel = chest_panel_origin(640, 360);
    app.mouse_pos = Some((300, 300)); // the resting cursor the first motion the moved gate answers
    app.on_press(168, 108);
    assert!(app.qc_armed, "the carried press the arm speaks");
    app.on_mouse_move(168, 108);
    app.on_mouse_move(208, 108);
    assert_eq!(app.qc_slots, vec![1, 2], "the two seats the motion paints");
    app.on_left_release();
    assert_eq!(app.chest_slots[1], (stone, 32), "the first seat's half");
    assert_eq!(app.chest_slots[2], (stone, 32), "the second seat's half");
    assert_eq!(app.chest_carried, (0, 0), "the spread the cursor empties");
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .container_fill
            .get(&(cx, cy, cz)),
        Some(&1),
        "the drag's release beat the fill the ride re-banks"
    );
    // THE CLOSE'S OWN FLUSH (the dragged halves the map rides home —
    // the disk bank the same walk keeps)
    app.on_escape();
    assert_eq!(
        app.container_stores[&(cx, cy, cz)][1],
        (stone, 32),
        "the first half the flush banks"
    );
    assert_eq!(
        app.container_stores[&(cx, cy, cz)][2],
        (stone, 32),
        "the second half the flush banks"
    );
}

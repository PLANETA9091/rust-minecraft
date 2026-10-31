use super::*;








/// THE SNOWBALL'S OWN BREAK (javap `Snowball.handleEntityEvent`
/// case 3 the 26.2 jar — the `bipush 8` loop the plain case
/// speaks, the ZERO-velocity shards the jitter walk never wears,
/// the NO-chick break the onHit override never rolls): the wall
/// the clip ray banks, the eight SnowballItem shards the burst
/// wears, the mobs the count the gate never adds.
#[test]
fn the_snowball_the_break_the_eight_shards_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let (px, pz) = (16i32, 16i32);
    let gy = 30i32;
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -4..=4 {
            for dz in -4..=4 {
                world.set(px + dx, gy - 1, pz + dz, crate::world::STONE);
            }
        }
        // THE WALL THE SHOT'S OWN TARGET (the south column the
        // flight's ray crosses)
        world.set(px, gy + 1, pz + 3, crate::world::STONE);
    }
    let mobs_before = app.mobs.len();
    app.eggs.push(crate::mob::EggProjectile {
        id: 777,
        x: f64::from(px) + 0.5,
        y: f64::from(gy) + 1.5,
        z: f64::from(pz) + 0.5,
        vx: 0.0,
        vy: 0.0,
        vz: 0.5,
        yaw: 0.0,
        pitch: 0.0,
        dead: false,
        kind: crate::mob::ThrownProjectileKind::Snowball,
    });
    for _ in 0..12 {
        if app.eggs.is_empty() {
            break;
        }
        app.update();
    }
    assert!(app.eggs.is_empty(), "the break the discard serves");
    let snow_shards = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::SnowballItem))
        .count();
    let egg_shards = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::EggItem))
        .count();
    assert_eq!(
        snow_shards, 8,
        "the bipush-8 loop the case speaks ({snow_shards})"
    );
    assert_eq!(egg_shards, 0, "the egg's own shards the kind never wears");
    assert_eq!(
        app.mobs.len(),
        mobs_before,
        "the NO-chick break the onHit override keeps"
    );
    // THE SHARD'S OWN REST (the ZERO wire velocity the case-3
    // call wears — the internal drift the only speed the shard
    // carries)
    let shard = app
        .particles
        .iter()
        .find(|p| matches!(p.kind, crate::particles::ParticleKind::SnowballItem))
        .expect("the burst the shards keep");
    assert!(
        shard.vx.abs() < 0.05 && shard.vy.abs() < 0.15 && shard.vz.abs() < 0.05,
        "the zero-velocity call the internal drift keeps ({},{},{})",
        shard.vx,
        shard.vy,
        shard.vz
    );
}
/// THE OFFLINE CHEST DRAG'S OWN PAINT AND SPLIT (autopush-146's
/// own lanes — the quickcraft walk the offline sim the
/// dispenser/table/furnace's own gate shape wears): the carried
/// press the drag arms (no place, no packet), the motion the
/// paint speaks (the grid seat the press's own motion paints, the
/// hotbar mirror row the paint adds), the release the even split
/// the local stores answer — the grid cell the 27-hold keeps, the
/// hotbar row the write-through speaks, the mirror the sync
/// re-banks, the HUD row the sync follows, the exact split the
/// cursor empties — and the right drag the one-each split serves.
#[test]
fn the_offline_chest_drag_the_paint_and_the_split_wear() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    app.open_chest_for(cell);
    assert_eq!(app.screen, ScreenKind::Chest);
    assert!(!app.chest_is_live, "the offline sim the door keeps");
    let panel = inventory_panel_origin(640, 360);
    app.chest_panel = panel;
    app.chest_slot_rects = chest_rects_for(panel);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the split reads
    }
    app.chest_mirror_sync_from_inventory();
    // the cursor 8 dirt; the drag over the grid cell 4 + the
    // hotbar mirror row 0 (menu 54 — the grid's 27 the n speaks)
    app.chest_carried = (dirt, 8);
    let (ax, ay) = chest_container_slot(panel, 4);
    app.on_press(ax + 9, ay + 9);
    assert!(app.qc_armed, "the offline carried press the drag arms");
    assert_eq!(app.chest_slots[4], (0, 0), "no place the press speaks");
    app.on_mouse_move(ax + 9, ay + 9);
    let (bx, by) = chest_mirror_slot(panel, 27);
    app.on_mouse_move(bx + 9, by + 9);
    assert_eq!(app.qc_slots, vec![4, 54], "the two seats the motion paints");
    app.on_left_release();
    // THE SPLIT: the floor(8/2) halves the local stores speak
    assert_eq!(app.chest_slots[4], (dirt, 4), "the grid cell's half");
    assert_eq!(
        app.inventory.items[0],
        (dirt, 4),
        "the hotbar row the write-through speaks"
    );
    assert_eq!(
        app.chest_mirror[27],
        (dirt, 4),
        "the mirror row the sync re-banks"
    );
    assert_eq!(
        app.hotbar_slots[0],
        (dirt, 4),
        "the HUD row the sync follows"
    );
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    // THE RIGHT DRAG'S OWN ONE-EACH (the type-1 the right press
    // arms): the two seats the one each speaks, the 2-stack the
    // seats empty
    app.chest_carried = (dirt, 2);
    let (cx, cy) = chest_container_slot(panel, 5);
    app.on_right_press(cx + 9, cy + 9);
    assert!(app.qc_armed, "the right press the drag arms");
    app.on_mouse_move(cx + 9, cy + 9);
    let (dx, dy) = chest_container_slot(panel, 6);
    app.on_mouse_move(dx + 9, dy + 9);
    assert_eq!(app.qc_slots, vec![5, 6], "the two seats the motion paints");
    app.on_right_release();
    assert_eq!(app.chest_slots[5], (dirt, 1), "the one-each's own one");
    assert_eq!(app.chest_slots[6], (dirt, 1), "the one-each's own one");
    assert_eq!(app.chest_carried, (0, 0), "the 2-stack the two seats empty");
}

/// THE OFFLINE CHEST DRAG'S OWN GATES AND THE DEFERRED PLACE
/// (autopush-146's own lanes — the drag the offline sim wears):
/// the different item's seat the paint refuses (the empty-painted
/// release the deferred place's PICKUP speaks — the swap the
/// offline click lane serves), the between-slots press the arm
/// skips (the release the un-armed walk never enters), the armed
/// drag the right press silences (the in-flight gate), the wrong
/// button's release the abort speaks, the between release the
/// offline no-op keeps, the OUTSIDE release the -999 throw the
/// local drop speaks (the whole carried the primary button drops).
#[test]
fn the_offline_chest_drag_the_gates_and_the_deferred_place_wear() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    app.open_chest_for(cell);
    let panel = inventory_panel_origin(640, 360);
    app.chest_panel = panel;
    app.chest_slot_rects = chest_rects_for(panel);
    // THE DIFFERENT ITEM'S SEAT REFUSES the paint — the empty-
    // painted release the deferred place's PICKUP speaks (the swap
    // the offline lanes serve)
    app.chest_slots[4] = (stone, 5);
    app.chest_carried = (dirt, 8);
    let (gx, gy) = chest_container_slot(panel, 4);
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the carried press arms on ANY seat");
    app.on_mouse_move(gx + 9, gy + 9);
    assert!(
        app.qc_slots.is_empty(),
        "the stone seat the different carried refuses"
    );
    app.on_left_release();
    assert_eq!(
        app.chest_carried,
        (stone, 5),
        "the deferred place the swap speaks"
    );
    assert_eq!(app.chest_slots[4], (dirt, 8), "the stone the hand took");
    // THE BETWEEN-SLOTS PRESS THE ARM SKIPS (the k == -1 silence —
    // the release the un-armed walk never enters)
    app.chest_carried = (dirt, 4);
    app.on_press(panel.0 + 170, panel.1 + 20);
    assert!(!app.qc_armed, "the between press the arm never speaks");
    app.on_left_release();
    assert_eq!(
        app.chest_carried,
        (dirt, 4),
        "the un-armed release the silence keeps"
    );
    // THE ARMED DRAG THE RIGHT PRESS SILENCES (the in-flight gate —
    // no split-place rides, the mismatch release the abort speaks)
    app.chest_slots[4] = (0, 0);
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the drag armed");
    app.on_right_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the right press the in-flight gate keeps");
    assert_eq!(app.chest_slots[4], (0, 0), "no split-place rode the press");
    app.on_right_release();
    assert!(!app.qc_armed, "the wrong button's release the drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    // THE EMPTY-PAINT RELEASE BETWEEN THE SLOTS: the offline walk
    // the -1 no-op keeps (no throw, no place — the cursor keeps)
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the drag armed again");
    app.on_mouse_move(panel.0 + 170, panel.1 + 20); // the paint skips (no seat)
    assert!(app.qc_slots.is_empty(), "the between motion paints nothing");
    app.on_left_release();
    assert_eq!(
        app.chest_carried,
        (dirt, 4),
        "the between release the no-op keeps"
    );
    // THE OUTSIDE RELEASE THE THROW SPEAKS (the -999 PICKUP the
    // local throw wears — the whole carried the primary drops: the
    // cursor the throw empties, the headless menus the body-less
    // silence the drop walk's own guard keeps)
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the drag armed");
    app.on_mouse_move(panel.0 + 200, panel.1 + 20); // outside — no seat
    assert!(app.qc_slots.is_empty(), "the outside motion paints nothing");
    app.on_left_release();
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the whole carried the throw drops"
    );
    assert!(
        app.drops.is_empty(),
        "the headless body the drop walk's own guard keeps silent"
    );
}

/// THE SHULKER'S OWN PLACEMENT CHAIN (the autopush-148 round minted
/// the walk, the autopush-304 round the 26.2 jar's own law re-pins —
/// javap ShulkerBoxBlock.getStateForPlacement the live disasm the
/// `setValue(FACING, context.getClickedFace())` walk answers): the
/// held shulker_box item the offline press places, the SHULKER_BOX
/// kind the NAME_OF row 778 speaks, the FLOOR LOOK the clicked face's
/// own UP byte stamps [the floor block's top face the hit's own
/// ordinal wears — the dispenser family's look-opposite walk the
/// CLICKED-FACE law retires], the consume(1) the stack rides, the
/// dropper's front read the placed box the next beat answers.
#[test]
fn the_offline_shulker_the_placement_the_floor_look_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let shulker_id = mc_registry::item_id_by_name("shulker_box").expect("the wire table");
    app.inventory.items[0] = (shulker_id, 4);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE FLOOR LOOK (the furnace placement test's own camera —
    // the pitch -PI/2 the top face the 4.0 read answers)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the shulker placement consumed the press"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::SHULKER_BOX,
            "the shulker the kind mint the placement lands"
        );
        assert_eq!(
            w.variant(c, top, c),
            crate::world::SHULKER_FACING_UP,
            "the floor look the clicked face's own UP byte the stamp wears"
        );
    }
    assert_eq!(
        app.inventory.items[0],
        (shulker_id, 3),
        "the consume(1) the stack rides"
    );
}

/// THE SHULKER'S OWN WALL CLICK (the autopush-304 round — the
/// CLICKED-FACE law the distinguishing case wears): the steep
/// down-north look the wall block's SOUTH face the ray enters [the
/// floor's top face the ray never reaches — the wall the volume
/// occludes], the hit's own SOUTH ordinal the FACING stamps — the
/// dispenser family's pitch-quarter law the UP byte would wear [the
/// two six-direction walks the two bytes speak], the placement cell
/// the wall's south side the empty cell answers, the lid the wall
/// the away direction opens.
#[test]
fn the_offline_shulker_the_wall_click_the_clicked_face_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let shulker_id = mc_registry::item_id_by_name("shulker_box").expect("the wire table");
    app.inventory.items[0] = (shulker_id, 4);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE WALL'S OWN SEAT (the cell two north of the player the
    // eye-level row wears — the placement cell the wall's south side
    // the empty cell keeps, the player's own cell the arm never
    // targets)
    let (wx, wy, wz) = (c, top + 1, c - 2);
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(wx, wy, wz, crate::world::STONE);
    }
    // THE STEEP DOWN-NORTH LOOK (the vanilla yaw 180 the north look
    // answers, the pitch -55 degrees the south-face entry the ray
    // serves [the drop 1.5*tan55 the y band the wall's own span
    // wears] — the floor's top face the hit the wall's volume
    // preempts)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 3.5, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -55.0f32.to_radians(),
    });
    assert!(
        app.on_right_press(0, 0),
        "the shulker placement consumed the press"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top + 1, c - 1),
            crate::world::SHULKER_BOX,
            "the shulker the wall's south cell the placement lands"
        );
        assert_eq!(
            w.variant(c, top + 1, c - 1),
            crate::world::SHULKER_FACING_SOUTH,
            "the clicked face's own SOUTH byte the stamp wears"
        );
    }
}

/// THE SHULKER'S OWN DOOR THE WINDOW BANKS (dev-150's own lane —
/// the right-press on the placed box the OFFLINE world answers):
/// the window the chest family's own ScreenKind::Chest shape opens,
/// the shulker's own kind row the title lane banks, the cell's own
/// 27-slot store the SAME map the dropper's front-read inserts
/// (the pre-banked stack the door's open loads), the carried the
/// fresh open keeps, the DOUBLE the lone box never speaks, the
/// sneaking player the arm SKIPS (the empty-hand press the PASS
/// verdict keeps), the reopen the store's own law keeps.
#[test]
fn the_offline_shulker_the_door_the_window_the_27_slots_banks() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::SHULKER_BOX);
    // THE EMPTY HAND (the drag test's own law — the sneaking press
    // the item lanes' place fallback the fresh loadout never arms)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    // THE SAME MAP THE FRONT-READ BANKS (the autopush-147 row — the
    // pre-inserted stack the door's open loads, the dropper's shot
    // the SAME entry consumes)
    app.container_stores_load_once();
    app.container_stores.insert(cell, {
        let mut v = vec![(0i32, 0i32); 27];
        v[13] = (dirt, 5);
        v
    });
    // THE FLOOR LOOK (the placement test's own camera — the pitch
    // -PI/2 the box cell the 2.0 read answers, the clip the placed
    // kind the own cell hit speaks)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 4.0, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the shulker door the press consumed"
    );
    assert_eq!(app.screen, ScreenKind::Chest, "the window the door opens");
    assert!(!app.chest_is_live, "the offline stand-in the door keeps");
    assert!(
        app.chest_window_shulker,
        "the shulker's own kind row the title lane banks"
    );
    assert_eq!(
        app.container_open_cell,
        Some(cell),
        "the cell the flush keys"
    );
    assert_eq!(
        app.container_open_cell_b, None,
        "the DOUBLE the lone box never speaks"
    );
    assert_eq!(app.chest_slots.len(), 27, "the lone box the 27 banks");
    assert_eq!(
        app.chest_slots[13],
        (dirt, 5),
        "the store's stack the open loads"
    );
    assert_eq!(app.chest_carried, (0, 0), "the fresh open the cursor keeps");
    // THE SHIFT GATE (the crafting-table's own law — the sneaking
    // player the door never opens, the empty-hand press the PASS
    // verdict keeps)
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::World, "the close the world keeps");
    app.shift_down = true;
    assert!(
        !app.on_right_press(0, 0),
        "the sneaking press the door refuses"
    );
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the shift gate the world keeps"
    );
    app.shift_down = false;
    // THE REOPEN THE STORE'S OWN LAW KEEPS (the second door the
    // stored stack loads — the chest family's own entry walk)
    assert!(
        app.on_right_press(0, 0),
        "the second press the door answers"
    );
    assert_eq!(
        app.screen,
        ScreenKind::Chest,
        "the window the second door opens"
    );
    assert_eq!(app.chest_slots[13], (dirt, 5), "the reopen the stack rides");
}

/// THE HOPPER'S OWN DOOR THE WINDOW THE FIVE SLOTS BANK (the
/// autopush-305 round — the autopush-304 NEXT list's own first
/// candidate the offline walk answers: javap 26.2 HopperBlock the
/// menu walk the HopperBlockEntity.createMenu the HopperMenu the
/// five-seat row opens, the container.hopper default name the
/// DEFAULT_NAME row banks, the BE's own withSize(5) the store width
/// the [`crate::world::container_kind_slot_count`] law wears): the
/// placement the facing stamp lands, the right-press on the placed
/// hopper the OFFLINE world answers — the window the hopper's own
/// ScreenKind::Chest shape opens (the 176x133 canvas flag the draw
/// reads), the cell's own five-slot store the SAME map the chest
/// family's doors insert (the pre-banked stack the door's open
/// loads), the carried the fresh open keeps, the vanilla OPEN-counter
/// voice the hopper kind the silence arm keeps, the sneaking player
/// the arm SKIPS, the reopen the store's own law keeps.
#[test]
fn the_offline_hopper_the_door_the_window_the_five_slots_banks() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::HOPPER);
    // THE EMPTY HAND (the drag test's own law — the sneaking press
    // the item lanes' place fallback the fresh loadout never arms)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    // THE SAME MAP THE DOORS BANK (the pre-inserted stack the door's
    // open loads — the five-slot width the BE's own withSize(5) the
    // fresh store keeps)
    app.container_stores_load_once();
    app.container_stores.insert(cell, {
        let mut v = vec![(0i32, 0i32); 5];
        v[2] = (dirt, 7);
        v
    });
    // THE FLOOR LOOK (the placement test's own camera — the pitch
    // -PI/2 the hopper cell the 2.0 read answers, the clip the placed
    // kind the own cell hit speaks)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 4.0, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the hopper door the press consumed"
    );
    assert_eq!(app.screen, ScreenKind::Chest, "the window the door opens");
    assert!(!app.chest_is_live, "the offline stand-in the door keeps");
    assert!(
        app.chest_window_hopper,
        "the hopper's own canvas flag the title lane banks"
    );
    assert!(
        !app.chest_window_shulker,
        "the shulker's row the door never wears"
    );
    assert_eq!(
        app.container_open_cell,
        Some(cell),
        "the cell the flush keys"
    );
    assert_eq!(
        app.container_open_cell_b, None,
        "the DOUBLE the lone hopper never speaks"
    );
    assert_eq!(app.chest_slots.len(), 5, "the lone hopper the five banks");
    assert_eq!(
        app.chest_slots[2],
        (dirt, 7),
        "the store's stack the open loads"
    );
    assert_eq!(app.chest_carried, (0, 0), "the fresh open the cursor keeps");
    // THE SHIFT GATE (the crafting-table's own law — the sneaking
    // player the door never opens, the empty-hand press the PASS
    // verdict keeps)
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::World, "the close the world keeps");
    app.shift_down = true;
    assert!(
        !app.on_right_press(0, 0),
        "the sneaking press the door refuses"
    );
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the shift gate the world keeps"
    );
    app.shift_down = false;
    // THE REOPEN THE STORE'S OWN LAW KEEPS (the second door the
    // stored stack loads — the chest family's own entry walk)
    assert!(
        app.on_right_press(0, 0),
        "the second press the door answers"
    );
    assert_eq!(
        app.screen,
        ScreenKind::Chest,
        "the window the second door opens"
    );
    assert_eq!(app.chest_slots[2], (dirt, 7), "the reopen the stack rides");
    // THE CLOSE'S OWN FLUSH (the edited store the map rides home — the
    // vanilla stopOpen tail the container walk serves, the five-slot
    // vec the whole bank keeps)
    app.chest_slots[1] = (dirt, 2);
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::World, "the second close lands");
    assert_eq!(
        app.container_stores.get(&cell).map(|s| s.as_slice()),
        Some(&[(0, 0), (dirt, 2), (dirt, 7), (0, 0), (0, 0)][..]),
        "the flush the five-slot store banks"
    );
}

/// THE HOPPER'S OWN TRANSFER THE PUSH THE COOLDOWN WEARS (the
/// autopush-307 round — javap 26.2 the live disasm the
/// scripts/jar_work/hopper_transfer_307.txt walk banks): the REAL
/// placement walk the hopper the chest's top face lands [the floor
/// click the DOWN facing stamps, the placement seed the queue the
/// next frame arms], the stocked row the beat the chest pushes, the
/// setChanged tail the analog + the disk bank ride, the cooldown the
/// `setCooldown(8)` law the next push banks — the eight-tick gap
/// the second item the same beat serves.
#[test]
fn the_offline_hopper_the_push_the_chest_the_cooldown_wears() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let hopper_id = mc_registry::item_id_by_name("hopper").expect("the wire table");
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE OFF-COLUMN SEAT (the player's own body the spawn column
    // keeps — the chest the two-east cell the BB intersection the
    // placement's canPlace gate never wears)
    let chest_cell = (c + 2, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(chest_cell.0, chest_cell.1, chest_cell.2, crate::world::CHEST);
    app.container_stores_load_once();
    app.container_stores.insert(chest_cell, vec![(0i32, 0i32); 27]);
    // THE REAL PLACEMENT WALK (the hopper item the hotbar's first
    // slot wears, the ANGLED look the chest's south face the ray
    // enters — the straight-down look the player's own BB the
    // canPlace gate refuses [the vanilla's own guard the body the
    // camera-following walk wears], the wall click the hopper the
    // chest's east cell lands, the NORTH facing the clicked face's
    // opposite stamps [the funnel the chest the mouth serves], the
    // tick seed the queue arms)
    app.inventory.items[0] = (hopper_id, 8);
    app.hotbar_slots = app.inventory.hotbar();
    // THE SNEAKING PLACE (the vanilla BlockItem.useOn walk the shift
    // gate wears — the chest's own use arm the sneaking press the
    // place fallback serves)
    app.shift_down = true;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, surface as f32 + 3.5, c as f32 + 2.5),
        yaw: 0.0,
        pitch: -55.0f32.to_radians(),
    });
    assert!(
        app.on_right_press(0, 0),
        "the hopper placement consumed the press"
    );
    app.shift_down = false;
    let hopper_cell = (c + 2, surface + 1, c + 1);
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(hopper_cell.0, hopper_cell.1, hopper_cell.2),
            crate::world::HOPPER,
            "the hopper the chest's east cell the placement lands"
        );
        assert_eq!(
            w.variant(hopper_cell.0, hopper_cell.1, hopper_cell.2),
            crate::world::DISPENSER_FACING_NORTH,
            "the clicked face's opposite the NORTH mouth stamps"
        );
        assert!(
            !w.hopper_ticks.is_empty(),
            "the placement seed the queue arms"
        );
    }
    // THE STOCKED ROW (the pre-bank the door test's own idiom wears —
    // the fresh five the two dirt the slot 0 keeps)
    app.container_stores.insert(
        hopper_cell,
        vec![(dirt, 2), (0, 0), (0, 0), (0, 0), (0, 0)],
    );
    // THE FIRST BEAT (the update the queue drains — the push the one
    // dirt the chest the seats, the cooldown the success banks)
    app.update();
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[0]),
        Some((dirt, 1)),
        "the hopper row the decrement wears"
    );
    assert_eq!(
        app.container_stores.get(&chest_cell).map(|s| s[0]),
        Some((dirt, 1)),
        "the chest the first item seats"
    );
    // THE COOLDOWN'S OWN GATE (the next tick the push never wears —
    // the setCooldown(8) law the intermediate beats refuse)
    app.update();
    app.update();
    assert_eq!(
        app.container_stores.get(&chest_cell).map(|s| s[0]),
        Some((dirt, 1)),
        "the cooldown the second push refuses"
    );
    // THE EIGHTH TICK'S OWN SERVE (the six more updates the +8 cadence
    // completes — the second dirt the same beat lands)
    for _ in 0..6 {
        app.update();
    }
    assert_eq!(
        app.container_stores.get(&chest_cell).map(|s| s[0]),
        Some((dirt, 2)),
        "the cooldown spent the second item serves"
    );
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[0]),
        Some((0, 0)),
        "the emptied slot the honest (0, 0) row banks"
    );
}

/// THE HOPPER'S OWN SUCK THE CHEST AND THE ENTITY WEARS (the
/// autopush-307 round — javap suckInItems the 26.2 jar the live
/// disasm answers): the chest ABOVE the hopper the container arm
/// serves [the DOWN face the flat slot order the one item per tick
/// speaks], the item entity the column arm the WHOLE stack the
/// insert serves [the empty leftover the discard keeps], the full
/// block above the entity lane refuses (the DOES_NOT_BLOCK gate the
/// collision verdict wears).
#[test]
fn the_offline_hopper_the_suck_the_chest_and_the_entity_wears() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let hopper_id = mc_registry::item_id_by_name("hopper").expect("the wire table");
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE REAL PLACEMENT WALK FIRST (the open ground cell the floor
    // look the ray enters — the chest the ABOVE seat the set walk
    // banks AFTER the placement lands: the pre-placed chest the ray
    // the top face would preempt)
    app.inventory.items[0] = (hopper_id, 8);
    app.hotbar_slots = app.inventory.hotbar();
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 4.0, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the hopper placement consumed the press"
    );
    let hopper_cell = (c, surface + 1, c);
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(hopper_cell.0, hopper_cell.1, hopper_cell.2),
            crate::world::HOPPER,
            "the hopper the ground cell the placement lands"
        );
    }
    // THE CHEST'S OWN SEAT ABOVE (the pre-banked kind the set walk
    // lands — the suck lane's source the same map keeps)
    let chest_cell = (c, surface + 2, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(chest_cell.0, chest_cell.1, chest_cell.2, crate::world::CHEST);
    app.container_stores_load_once();
    app.container_stores.insert(
        chest_cell,
        vec![(dirt, 2), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    );
    // THE FIRST BEAT (the suck the one dirt the chest the above row
    // the hopper the seats — the cooldown the success banks)
    app.update();
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[0]),
        Some((dirt, 1)),
        "the hopper the first dirt seats"
    );
    assert_eq!(
        app.container_stores.get(&chest_cell).map(|s| s[0]),
        Some((dirt, 1)),
        "the chest row the decrement wears"
    );
    // THE COOLDOWN'S OWN GATE (the seven intermediate beats the
    // setCooldown(8) law the second suck refuses)
    for _ in 0..7 {
        app.update();
    }
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[0]),
        Some((dirt, 1)),
        "the cooldown the second suck refuses"
    );
    // THE EIGHTH TICK'S OWN SERVE (the second dirt the same beat
    // lands — the chest row the (0, 0) honest empty wears)
    app.update();
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[0]),
        Some((dirt, 2)),
        "the cooldown spent the second dirt serves"
    );
    assert_eq!(
        app.container_stores.get(&chest_cell).map(|s| s[0]),
        Some((0, 0)),
        "the emptied chest slot the honest row banks"
    );
    // THE ENTITY ARM (javap getItemsAtAndAbove + addItem(Container,
    // ItemEntity) — the drop the hopper's own cell the suck column
    // answers, the WHOLE stack the insert serves, the discard the
    // empty leftover keeps). THE CHEST the above cell the container
    // arm the entity lane NEVER serves — the fresh cell the no-
    // container-above walk the entity lane arms.
    let entity_hopper_cell = (c + 4, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(entity_hopper_cell.0, entity_hopper_cell.1, entity_hopper_cell.2, crate::world::HOPPER);
    app.offline_world
        .as_mut()
        .unwrap()
        .set_variant(entity_hopper_cell.0, entity_hopper_cell.1, entity_hopper_cell.2, 0);
    app.offline_world
        .as_mut()
        .unwrap()
        .schedule_hopper_tick(app.frame + 1, entity_hopper_cell.0, entity_hopper_cell.1, entity_hopper_cell.2);
    let id = app.next_drop_id;
    app.next_drop_id += 1;
    let mut rng = crate::drops::Lcg::new(7);
    let drop = crate::drops::ItemDrop::spawn(id, dirt, 3, entity_hopper_cell.0, entity_hopper_cell.1, entity_hopper_cell.2, &mut rng);
    let drop_id = drop.id;
    app.drops.push(drop);
    app.update();
    assert_eq!(
        app.container_stores.get(&entity_hopper_cell).map(|s| s[0]),
        Some((dirt, 3)),
        "the entity's whole stack the hopper row seats"
    );
    assert!(
        !app.drops.iter().any(|d| d.id == drop_id),
        "the empty leftover the entity discards"
    );
}

/// THE HOPPER'S OWN FURNACE FEED THE SIDE LANE THE COOLDOWN WEARS
/// (the autopush-308 round — javap ejectItems pc 13-20 the 26.2 jar
/// the live disasm answers): the REAL sneak-place walk the furnace's
/// south face the ray enters, the NORTH facing the clicked face's
/// opposite stamps [the eject the facing.getOpposite() the furnace's
/// SIDES face the FUEL lane answers], the coal the fuel slot the
/// beats seat, the dirt the fuel gate refuses [the isFuel verdict the
/// probe restores], the no-input furnace the unlit kind the honest
/// keep.
#[test]
fn the_offline_hopper_the_furnace_side_the_fuel_lane_the_cooldown_wears() {
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let hopper_id = mc_registry::item_id_by_name("hopper").expect("the wire table");
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE FURNACE'S OWN SEAT (the push test's own chest cell the
    // furnace wears — the south face the ray the side lane enters)
    let furnace_cell = (c + 2, surface + 1, c);
    app.offline_world.as_mut().unwrap().set(
        furnace_cell.0,
        furnace_cell.1,
        furnace_cell.2,
        crate::world::FURNACE,
    );
    app.container_stores_load_once();
    // THE REAL PLACEMENT WALK (the hopper item the hotbar's first slot
    // wears, the ANGLED look the furnace's south face the ray enters,
    // the sneak press the place arm serves, the NORTH facing the
    // clicked face's opposite stamps, the tick seed the queue arms)
    app.inventory.items[0] = (hopper_id, 8);
    app.hotbar_slots = app.inventory.hotbar();
    app.shift_down = true;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, surface as f32 + 3.5, c as f32 + 2.5),
        yaw: 0.0,
        pitch: -55.0f32.to_radians(),
    });
    assert!(
        app.on_right_press(0, 0),
        "the hopper placement consumed the press"
    );
    app.shift_down = false;
    let hopper_cell = (c + 2, surface + 1, c + 1);
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(hopper_cell.0, hopper_cell.1, hopper_cell.2),
            crate::world::HOPPER,
            "the hopper the furnace's south cell the placement lands"
        );
        assert_eq!(
            w.variant(hopper_cell.0, hopper_cell.1, hopper_cell.2),
            crate::world::DISPENSER_FACING_NORTH,
            "the clicked face's opposite the NORTH mouth stamps"
        );
        assert!(
            !w.hopper_ticks.is_empty(),
            "the placement seed the queue arms"
        );
    }
    // THE STOCKED ROW (the two coal the slot 0 keeps, the two dirt the
    // slot 1 rides — the fuel lane's refuse verdict the later beat
    // wears)
    app.container_stores.insert(
        hopper_cell,
        vec![(coal, 2), (dirt, 2), (0, 0), (0, 0), (0, 0)],
    );
    // THE FIRST BEAT (the update the queue drains — the push the one
    // coal the furnace's FUEL slot seats, the cooldown the success
    // banks)
    app.update();
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[0]),
        Some((coal, 1)),
        "the hopper row the coal decrement wears"
    );
    assert_eq!(
        app.furnace_stores
            .get(&furnace_cell)
            .map(|s| s[crate::world::FURNACE_SLOT_FUEL]),
        Some((coal, 1)),
        "the furnace's fuel slot the first coal seats"
    );
    // THE COOLDOWN'S OWN GATE (the setCooldown(8) law the
    // intermediate beats refuse)
    app.update();
    app.update();
    assert_eq!(
        app.furnace_stores
            .get(&furnace_cell)
            .map(|s| s[crate::world::FURNACE_SLOT_FUEL]),
        Some((coal, 1)),
        "the cooldown the second push refuses"
    );
    // THE EIGHTH TICK'S OWN SERVE (the six more updates the +8 cadence
    // completes — the second coal the same beat lands)
    for _ in 0..6 {
        app.update();
    }
    assert_eq!(
        app.furnace_stores
            .get(&furnace_cell)
            .map(|s| s[crate::world::FURNACE_SLOT_FUEL]),
        Some((coal, 2)),
        "the cooldown spent the second coal serves"
    );
    // THE FUEL GATE'S OWN REFUSE (the coal row spent the dirt probe
    // the fuel lane refuses — the canPlaceItem the isFuel verdict the
    // restore keeps, the hopper row the honest dirt keeps)
    for _ in 0..9 {
        app.update();
    }
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[1]),
        Some((dirt, 2)),
        "the dirt the fuel lane refuses"
    );
    assert_eq!(
        app.furnace_stores
            .get(&furnace_cell)
            .map(|s| s[crate::world::FURNACE_SLOT_FUEL]),
        Some((coal, 2)),
        "the fuel slot the two coal keeps"
    );
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[0]),
        Some((0, 0)),
        "the spent coal slot the honest (0, 0) row banks"
    );
    // THE UNLIT'S OWN KEEP (the no-input furnace the no-recipe walk
    // the ignite never wears — the world kind the unlit pair the
    // honest keep)
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(furnace_cell.0, furnace_cell.1, furnace_cell.2),
        crate::world::FURNACE,
        "the no-input furnace the unlit kind keeps"
    );
}

/// THE HOPPER'S OWN FURNACE PIPELINE THE ORE THE SMELT THE INGOT
/// WEARS (the autopush-308 round — the full ore-to-ingot lane the
/// three pieces the walk serves): the REAL floor-look placement the
/// under hopper lands [the DOWN mouth the ground stamps], the
/// furnace the above seat the set walk banks, the feeder hopper the
/// top seat the DOWN mouth the furnace's UP face the INPUT lane
/// arms, the ore the input slot the beats seat, the furnace world
/// walk the coal the burn the smelt the 200-tick total speaks [the
/// lit pair the world kind flips], the ingot the result slot the
/// under hopper's [2, 1] order the DOWN face takes — the vanilla
/// automation chain the client the real beats plays.
#[test]
fn the_offline_hopper_the_furnace_pipeline_the_ore_the_smelt_the_ingot_wears() {
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let iron_ore = mc_registry::item_id_by_name("iron_ore").unwrap();
    let iron_ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
    let hopper_id = mc_registry::item_id_by_name("hopper").expect("the wire table");
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE REAL PLACEMENT WALK (the open ground cell the floor look the
    // ray enters — the under hopper the DOWN mouth stamps, the tick
    // seed the queue arms)
    app.inventory.items[0] = (hopper_id, 8);
    app.hotbar_slots = app.inventory.hotbar();
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 4.0, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the hopper placement consumed the press"
    );
    let under_cell = (c, surface + 1, c);
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(under_cell.0, under_cell.1, under_cell.2),
            crate::world::HOPPER,
            "the under hopper the ground cell the placement lands"
        );
        assert_eq!(
            w.variant(under_cell.0, under_cell.1, under_cell.2),
            crate::world::DISPENSER_FACING_DOWN,
            "the vertical look the DOWN mouth stamps"
        );
    }
    // THE FURNACE'S OWN SEAT ABOVE (the pre-banked kind the set walk
    // lands — the under hopper's suck source the same walk the chest
    // test keeps)
    let furnace_cell = (c, surface + 2, c);
    app.offline_world.as_mut().unwrap().set(
        furnace_cell.0,
        furnace_cell.1,
        furnace_cell.2,
        crate::world::FURNACE,
    );
    // THE FEEDER'S OWN SEAT (the hopper the furnace's top cell the
    // DOWN mouth the furnace's UP face the INPUT lane arms — the
    // placement seed the queue wears)
    let feeder_cell = (c, surface + 3, c);
    app.offline_world.as_mut().unwrap().set(
        feeder_cell.0,
        feeder_cell.1,
        feeder_cell.2,
        crate::world::HOPPER,
    );
    app.offline_world.as_mut().unwrap().set_variant(
        feeder_cell.0,
        feeder_cell.1,
        feeder_cell.2,
        crate::world::DISPENSER_FACING_DOWN,
    );
    app.offline_world.as_mut().unwrap().schedule_hopper_tick(
        app.frame + 1,
        feeder_cell.0,
        feeder_cell.1,
        feeder_cell.2,
    );
    app.container_stores_load_once();
    // THE STOCKED ROWS (the feeder the one ore rides, the furnace the
    // eight coal the fuel slot keeps — the under hopper the empty
    // five wears)
    app.container_stores.insert(
        feeder_cell,
        vec![(iron_ore, 1), (0, 0), (0, 0), (0, 0), (0, 0)],
    );
    app.furnace_stores
        .insert(furnace_cell, vec![(0, 0), (coal, 8), (0, 0)]);
    // THE FIRST BEAT (the feeder the ore the INPUT slot seats — the
    // facing.getOpposite() the UP face the [0] order answers)
    app.update();
    assert_eq!(
        app.container_stores.get(&feeder_cell).map(|s| s[0]),
        Some((0, 0)),
        "the feeder row the ore departs"
    );
    assert_eq!(
        app.furnace_stores
            .get(&furnace_cell)
            .map(|s| s[crate::world::FURNACE_SLOT_INPUT]),
        Some((iron_ore, 1)),
        "the furnace's input slot the ore seats"
    );
    // THE SMELT'S OWN WALK (the furnace world tick the coal the burn
    // the lit pair flips — the 200-tick total the vanilla codec
    // default speaks, the cooking progress the beats count, the
    // result slot the ingot seats)
    for _ in 0..260 {
        app.update();
    }
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(furnace_cell.0, furnace_cell.1, furnace_cell.2),
        crate::world::FURNACE_LIT,
        "the burning furnace the lit pair the world kind flips"
    );
    let trio = app
        .furnace_stores
        .get(&furnace_cell)
        .cloned()
        .expect("the furnace trio the walk banks");
    assert_eq!(
        trio[crate::world::FURNACE_SLOT_INPUT],
        (0, 0),
        "the smelted ore the input slot departs"
    );
    assert_eq!(
        trio[crate::world::FURNACE_SLOT_FUEL],
        (coal, 7),
        "the burnt coal the fuel slot the consume keeps"
    );
    // THE UNDER HOPPER'S OWN TAKE (the [2, 1] order the DOWN face the
    // can_take gate serves — the result slot the ingot the hopper
    // seats, the spent result the empty keeps)
    assert_eq!(
        app.container_stores.get(&under_cell).map(|s| s[0]),
        Some((iron_ingot, 1)),
        "the under hopper the ingot the result lane takes"
    );
    assert_eq!(
        trio[crate::world::FURNACE_SLOT_RESULT],
        (0, 0),
        "the taken ingot the result slot the empty keeps"
    );
}

/// THE HOPPER'S OWN BREWING SIDE THE FUEL LANE THE COOLDOWN WEARS
/// (the autopush-309 round — the stand's own SIDES lane the real
/// walk serves): the REAL sneak-place walk the hopper the stand's
/// south face the ray enters — the NORTH facing the SIDES lane
/// [0, 1, 2, 4] stamps, the blaze powder the fuel slot the beats
/// seat, the cooldown the eight-tick cadence banks, the dirt the
/// bottle trio the fuel lane the refuse keeps [the restore the
/// hopper row wears] — the vanilla automation chain the real beats
/// plays.
#[test]
fn the_offline_hopper_the_brewing_side_the_fuel_lane_the_cooldown_wears() {
    let blaze_powder = mc_registry::item_id_by_name("blaze_powder").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let hopper_id = mc_registry::item_id_by_name("hopper").expect("the wire table");
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE BREWING STAND'S OWN SEAT (the push test's own furnace cell
    // the stand wears — the south face the ray the side lane enters)
    let stand_cell = (c + 2, surface + 1, c);
    app.offline_world.as_mut().unwrap().set(
        stand_cell.0,
        stand_cell.1,
        stand_cell.2,
        crate::world::BREWING_STAND,
    );
    app.container_stores_load_once();
    // THE REAL PLACEMENT WALK (the hopper item the hotbar's first slot
    // wears, the ANGLED look the stand's south face the ray enters,
    // the sneak press the place arm serves, the NORTH facing the
    // clicked face's opposite stamps, the tick seed the queue arms)
    app.inventory.items[0] = (hopper_id, 8);
    app.hotbar_slots = app.inventory.hotbar();
    app.shift_down = true;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, surface as f32 + 3.5, c as f32 + 2.5),
        yaw: 0.0,
        pitch: -55.0f32.to_radians(),
    });
    assert!(
        app.on_right_press(0, 0),
        "the hopper placement consumed the press"
    );
    app.shift_down = false;
    let hopper_cell = (c + 2, surface + 1, c + 1);
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(hopper_cell.0, hopper_cell.1, hopper_cell.2),
            crate::world::HOPPER,
            "the hopper the stand's south cell the placement lands"
        );
        assert_eq!(
            w.variant(hopper_cell.0, hopper_cell.1, hopper_cell.2),
            crate::world::DISPENSER_FACING_NORTH,
            "the clicked face's opposite the NORTH mouth stamps"
        );
        assert!(
            !w.hopper_ticks.is_empty(),
            "the placement seed the queue arms"
        );
    }
    // THE STOCKED ROW (the two blaze powder the slot 0 keeps, the two
    // dirt the slot 1 rides — the fuel lane's refuse verdict the later
    // beat wears)
    app.container_stores.insert(
        hopper_cell,
        vec![(blaze_powder, 2), (dirt, 2), (0, 0), (0, 0), (0, 0)],
    );
    // THE FIRST BEAT (the update the queue drains — the push the one
    // blaze powder the stand's FUEL slot seats, the SIDES order the
    // bottle trio the refuse speaks, the cooldown the success banks)
    app.update();
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[0]),
        Some((blaze_powder, 1)),
        "the hopper row the blaze decrement wears"
    );
    assert_eq!(
        app.brewing_stores
            .get(&stand_cell)
            .map(|s| s[crate::world::BREWING_SLOT_FUEL]),
        Some((blaze_powder, 1)),
        "the stand's fuel slot the first blaze seats"
    );
    // THE COOLDOWN'S OWN GATE (the setCooldown(8) law the
    // intermediate beats refuse) + THE WALK'S OWN REFILL (the
    // dev-338 round — the vanilla serverTick the dry stand the NEXT
    // beat wears: the fuel word the zero reads, the powder the
    // refill arm consumes, the twenty uses the tank banks — the
    // frozen-stand expectation the walk's own vanilla law retires)
    app.update();
    app.update();
    assert_eq!(
        app.brewing_stores
            .get(&stand_cell)
            .map(|s| s[crate::world::BREWING_SLOT_FUEL]),
        Some((0, 0)),
        "the refill arm the first blaze converts"
    );
    assert_eq!(
        app.brewing_word_stores
            .get(&stand_cell)
            .map(|w| w[crate::brewing_sim::DATA_FUEL_USES]),
        Some(20),
        "the twenty uses the tank banks"
    );
    // THE EIGHTH TICK'S OWN SERVE (the six more updates the +8 cadence
    // completes — the second blaze the same beat lands, the FULL tank
    // the walk's refill arm never touches [the fuel > 0 gate the
    // powder keeps])
    for _ in 0..6 {
        app.update();
    }
    assert_eq!(
        app.brewing_stores
            .get(&stand_cell)
            .map(|s| s[crate::world::BREWING_SLOT_FUEL]),
        Some((blaze_powder, 1)),
        "the cooldown spent the second blaze serves"
    );
    // THE FUEL GATE'S OWN REFUSE (the blaze row spent the dirt probe
    // the bottle trio the fuel lane refuses — the canPlaceItem the
    // BREWING_FUEL tag the lone member the verdict wears, the hopper
    // row the honest dirt keeps)
    for _ in 0..9 {
        app.update();
    }
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[1]),
        Some((dirt, 2)),
        "the dirt the SIDES lane refuses"
    );
    assert_eq!(
        app.brewing_stores
            .get(&stand_cell)
            .map(|s| s[crate::world::BREWING_SLOT_FUEL]),
        Some((blaze_powder, 1)),
        "the fuel slot the full tank the second blaze keeps"
    );
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[0]),
        Some((0, 0)),
        "the spent blaze slot the honest (0, 0) row banks"
    );
}

/// THE HOPPER'S OWN BREWING BELOW THE POTION LANE THE DOWN ORDER
/// WEARS (the autopush-309 round — the stand's own DOWN face the real
/// walk serves): the REAL floor-look placement the under hopper lands
/// [the DOWN mouth the ground stamps], the brewing stand the above
/// seat the set walk banks, the potion the bottle slot the beats
/// take [the SLOTS_FOR_DOWN the [0, 1, 2, 3] order speaks], the
/// spent ingredient the canTakeItemThroughFace the GLASS_BOTTLE the
/// only take the refuse keeps — the vanilla automation chain the
/// real beats plays.
#[test]
fn the_offline_hopper_the_brewing_below_the_potion_lane_the_down_order_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let nether_wart = mc_registry::item_id_by_name("nether_wart").unwrap();
    let hopper_id = mc_registry::item_id_by_name("hopper").expect("the wire table");
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE REAL PLACEMENT WALK (the open ground cell the floor look the
    // ray enters — the under hopper the DOWN mouth stamps, the tick
    // seed the queue arms)
    app.inventory.items[0] = (hopper_id, 8);
    app.hotbar_slots = app.inventory.hotbar();
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 4.0, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the hopper placement consumed the press"
    );
    let under_cell = (c, surface + 1, c);
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(under_cell.0, under_cell.1, under_cell.2),
            crate::world::HOPPER,
            "the under hopper the ground cell the placement lands"
        );
        assert_eq!(
            w.variant(under_cell.0, under_cell.1, under_cell.2),
            crate::world::DISPENSER_FACING_DOWN,
            "the vertical look the DOWN mouth stamps"
        );
    }
    // THE BREWING STAND'S OWN SEAT ABOVE (the pre-banked kind the set
    // walk lands — the under hopper's suck source the same walk the
    // furnace test keeps)
    let stand_cell = (c, surface + 2, c);
    app.offline_world.as_mut().unwrap().set(
        stand_cell.0,
        stand_cell.1,
        stand_cell.2,
        crate::world::BREWING_STAND,
    );
    app.container_stores_load_once();
    // THE STOCKED FIVE (the potion the bottle slot keeps, the wart
    // the ingredient seat rides — the DOWN order the bottle the first
    // probe speaks, the spent wart the take gate the later beat
    // refuses)
    app.brewing_stores.insert(
        stand_cell,
        vec![(potion, 1), (0, 0), (0, 0), (nether_wart, 2), (0, 0)],
    );
    // THE FIRST BEAT (the update the queue drains — the suck the one
    // potion the bottle lane takes, the cooldown the success banks)
    app.update();
    assert_eq!(
        app.container_stores.get(&under_cell).map(|s| s[0]),
        Some((potion, 1)),
        "the under hopper the potion the bottle lane takes"
    );
    assert_eq!(
        app.brewing_stores
            .get(&stand_cell)
            .map(|s| s[crate::world::BREWING_SLOT_BOTTLE_0]),
        Some((0, 0)),
        "the bottle slot the empty keeps"
    );
    // THE COOLDOWN'S OWN GATE (the intermediate beats the suck refuse)
    app.update();
    app.update();
    assert_eq!(
        app.container_stores.get(&under_cell).map(|s| s[0]),
        Some((potion, 1)),
        "the cooldown the second suck refuses"
    );
    // THE SPENT INGREDIENT'S OWN REFUSE (the eight-tick cadence spent
    // the suck the DOWN order the walk exhausts — the bottle trio the
    // empty the probe walks, the ingredient slot the wart the
    // canTakeItemThroughFace the GLASS_BOTTLE the only take refuses,
    // the hopper row the honest keep wears)
    for _ in 0..7 {
        app.update();
    }
    assert_eq!(
        app.container_stores.get(&under_cell).map(|s| s[0]),
        Some((potion, 1)),
        "the spent wart the ingredient take refuses"
    );
    assert_eq!(
        app.brewing_stores
            .get(&stand_cell)
            .map(|s| s[crate::world::BREWING_SLOT_INGREDIENT]),
        Some((nether_wart, 2)),
        "the refused take the ingredient row keeps"
    );
    assert_eq!(
        app.container_stores.get(&under_cell).map(|s| s[1]),
        Some((0, 0)),
        "the hopper's second slot the honest empty keeps"
    );
}

/// THE CHORUS EAT'S OWN TELEPORT WALK (the autopush-310 round —
/// javap TeleportRandomlyConsumeEffect.apply the 26.2 jar the 16
/// attempts the three-draw rows wear, the randomTeleport the
/// walk-down the safe verdict the arrival banks, the completion the
/// FoodData fold the teleport voice the 128-puff departure cloud the
/// body snap serves — the REAL eat arc the press the hold the
/// completion the vanilla's own beats play).
#[test]
fn the_offline_eat_the_chorus_the_teleport_lane_the_sound_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    let chorus = mc_registry::item_id_by_name("chorus_fruit").unwrap();
    app.inventory.items[app.selected_hotbar] = (chorus, 4);
    app.hotbar_slots = app.inventory.hotbar();
    // THE FULL BAR (javap FoodProperties.canAlwaysEat the chorus's
    // own true the consume gate arms — the vanilla law the plain
    // food's full-bar refusal the chorus never wears)
    app.food_data.set_food_level(20);
    app.food_data.set_saturation(5.0);
    // THE LANDING ZONE (the flat plate the walk-down's own seat —
    // the ±4 targets the plate the walk-down lands, the column the
    // four cells the clear keeps)
    let body0 = {
        let b = app.body.as_ref().unwrap();
        (b.x, b.y, b.z)
    };
    let (px, pz) = (body0.0.floor() as i32, body0.2.floor() as i32);
    let gy = body0.1.floor() as i32 - 1;
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -6..=6 {
            for dz in -6..=6 {
                world.set(px + dx, gy, pz + dz, crate::world::STONE);
                for dy in 1..=5 {
                    world.set(px + dx, gy + dy, pz + dz, crate::world::AIR);
                }
            }
        }
    }
    // THE PRESS (the full bar the always-eat the hold arms)
    assert!(
        app.on_right_press(0, 0),
        "the chorus the always-eat the hold arms"
    );
    assert!(app.eating.is_some(), "the eat opened");
    // THE FULL ARC (the 32 ticks the completion folds)
    for _ in 0..32 {
        app.update();
    }
    assert!(app.eating.is_none(), "the hold completed");
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (chorus, 3),
        "the survival consume the stack shrinks"
    );
    // THE TELEPORT (javap apply the success the body the arrival
    // wears — the ±4 band the diameter folds, the moved verdict the
    // walk-down's own seat answers)
    let body = app.body.as_ref().unwrap();
    let dx = body.x - body0.0;
    let dz = body.z - body0.2;
    assert!(
        dx.abs() <= 4.5 && dz.abs() <= 4.5,
        "the ±4 band the diameter wears (dx={dx}, dz={dz})"
    );
    assert!(
        dx.abs() > 0.001 || dz.abs() > 0.001,
        "the teleport the body moved (dx={dx}, dz={dz})"
    );
    // THE WALK-DOWN'S OWN SEAT (the body the plate the settle wears —
    // the below the feet the solid the landed verdict keeps)
    for _ in 0..4 {
        app.update();
    }
    let body = app.body.as_ref().unwrap();
    {
        let world = app.offline_world.as_ref().unwrap();
        assert!(
            crate::body::solid(
                world,
                body.x.floor() as i32,
                body.y.floor() as i32 - 1,
                body.z.floor() as i32
            ),
            "the walk-down the below the feet the solid keeps (y={})",
            body.y
        );
    }
    // THE TELEPORT VOICE (javap CHORUS_FRUIT_TELEPORT the PLAYERS
    // source the completion's own event the log carries)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.chorus_fruit.teleport ")),
        "the teleport voice the log carries"
    );
    // THE DEPARTURE CLOUD (javap LivingEntity.handleEntityEvent case
    // 46 the sipush 128 loop — the Portal puffs the old pose's own
    // anchor the event order keeps)
    let puffs = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Portal { .. }))
        .count();
    assert_eq!(puffs, 128, "the 128-puff the event-46 loop banks");
    // THE FOOD FOLD (the chorus's own nutrition the absolute fold —
    // the bar the cap the chorus never breaks)
    assert_eq!(app.food_data.food_level(), 20, "the bar the cap keeps");
}

/// THE END GATEWAY'S OWN ITEM LANE (the autopush-310 round — javap
/// EndGatewayBlock.entityInside the cooldown gate the trigger pair
/// the 26.2 jar banks, the arrival the getPortalDestination's
/// atBottomCenterOf exit the Vec3.ZERO velocity the transition
/// serves, the second item the cooldown the wait keeps, the drain
/// the lane re-arms — the REAL item entities the lane moves).
#[test]
fn the_offline_gateway_the_item_lane_the_cooldown_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    // THE AIR PLATFORM (the stone plate the sites the test builds —
    // the terrain the interference never wears)
    let py = app.spawn_y + 10;
    let (gx, gz) = (20i32, 20i32);
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -10..=10 {
            for dz in -10..=10 {
                world.set(gx + dx, py, gz + dz, crate::world::STONE);
            }
        }
        // THE GATEWAY'S OWN SEAT (the cell the kind wears — the
        // no-item law the placement lane never carries, the store the
        // test stages; the float the two-above keeps — the fallen
        // item the platform's own top the OUTSIDE cell rests, the
        // re-trigger the steal never wears)
        world.set(gx, py + 3, gz, crate::world::END_GATEWAY);
    }
    let gate_cell = (gx, py + 3, gz);
    let exit_cell = (gx + 8, py + 1, gz);
    app.gateway_stores.insert(
        gate_cell,
        crate::screens::GatewayStore {
            exit: exit_cell,
            exact: true,
            cooldown: 0,
            age: crate::world::GATEWAY_SPAWN_TICKS,
        },
    );
    // THE FIRST ITEM (javap entityInside the cell the item's own read
    // answers — the spawn the gateway's own center the fall-through
    // walk the same tick the lane serves)
    let mut rng = crate::drops::Lcg::new(11);
    app.drops.push(crate::drops::ItemDrop::spawn_entity_loot(
        9001,
        dirt,
        1,
        f64::from(gate_cell.0) + 0.5,
        f64::from(gate_cell.1) + 0.4,
        f64::from(gate_cell.2) + 0.5,
        &mut rng,
    ));
    app.update();
    // THE ARRIVAL (javap getPortalPosition the atBottomCenterOf the
    // exit cell the bottom-center vector speaks, the transition's own
    // ZERO the velocity banks, the triggerCooldown the 40 the enter
    // tick writes)
    let first = app.drops.iter().find(|d| d.id == 9001).unwrap();
    assert_eq!(first.x, f64::from(exit_cell.0) + 0.5, "the exit the x");
    assert_eq!(first.y, f64::from(exit_cell.1), "the exit the y");
    assert_eq!(first.z, f64::from(exit_cell.2) + 0.5, "the exit the z");
    assert_eq!(
        (first.vx, first.vy, first.vz),
        (0.0, 0.0, 0.0),
        "the ZERO the velocity"
    );
    assert_eq!(
        app.gateway_stores.get(&gate_cell).unwrap().cooldown,
        crate::world::GATEWAY_COOLDOWN_TICKS,
        "the cooldown the 40 the enter tick banks"
    );
    // THE SECOND ITEM'S OWN WAIT (javap the isCoolingDown fold — the
    // cooldown the tick the decay walks, the item the gateway the
    // fall-through keeps, the teleport the lane never serves)
    app.drops.push(crate::drops::ItemDrop::spawn_entity_loot(
        9002,
        dirt,
        1,
        f64::from(gate_cell.0) + 0.5,
        f64::from(gate_cell.1) + 0.4,
        f64::from(gate_cell.2) + 0.5,
        &mut rng,
    ));
    for _ in 0..6 {
        app.update();
    }
    let second = app.drops.iter().find(|d| d.id == 9002).unwrap();
    assert!(
        second.x != f64::from(exit_cell.0) + 0.5,
        "the second item the cooldown the teleport refuses (y={}, the exit the x never wears)",
        second.y
    );
    assert_eq!(
        app.gateway_stores.get(&gate_cell).unwrap().cooldown,
        crate::world::GATEWAY_COOLDOWN_TICKS - 6,
        "the decay the -1 the tick wears"
    );
    // THE DRAIN'S OWN RE-ARM (javap portalTick the -1-per-tick the
    // walk drains — the third item the lane the re-arm serves)
    for _ in 0..(crate::world::GATEWAY_COOLDOWN_TICKS - 6) {
        app.update();
    }
    assert_eq!(
        app.gateway_stores.get(&gate_cell).unwrap().cooldown,
        0,
        "the drain the cooldown empties"
    );
    app.drops.push(crate::drops::ItemDrop::spawn_entity_loot(
        9003,
        dirt,
        1,
        f64::from(gate_cell.0) + 0.5,
        f64::from(gate_cell.1) + 0.4,
        f64::from(gate_cell.2) + 0.5,
        &mut rng,
    ));
    app.update();
    let third = app.drops.iter().find(|d| d.id == 9003).unwrap();
    assert_eq!(third.x, f64::from(exit_cell.0) + 0.5, "the re-arm the x");
    assert_eq!(third.y, f64::from(exit_cell.1), "the re-arm the y");
}

/// THE GATEWAY'S OWN HOPPER CHAIN (the autopush-310 round — the
/// vanilla automation the real beats play: the item the gateway
/// teleports, the exit the hopper's own top the arrival lands, the
/// suck walk the entity the store banks — the REAL placement walk
/// the under lane the 309 record's own pattern keeps).
#[test]
fn the_offline_gateway_the_hopper_chain_the_exit_lands_the_suck_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let hopper_id = mc_registry::item_id_by_name("hopper").unwrap();
    // THE AIR PLATFORM (the sites the test builds)
    let py = app.spawn_y + 10;
    let (gx, gz) = (20i32, 20i32);
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -10..=10 {
            for dz in -10..=10 {
                world.set(gx + dx, py, gz + dz, crate::world::STONE);
            }
        }
        world.set(gx, py + 1, gz, crate::world::END_GATEWAY);
    }
    // THE REAL PLACEMENT WALK (the floor look the ray enters — the
    // hopper the platform cell the DOWN mouth stamps, the tick seed
    // the queue arms)
    app.inventory.items[0] = (hopper_id, 4);
    app.hotbar_slots = app.inventory.hotbar();
    let hopper_cell = (gx + 8, py + 1, gz);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(
            hopper_cell.0 as f32 + 0.5,
            (py + 5) as f32,
            hopper_cell.2 as f32 + 0.5,
        ),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the hopper placement consumed the press"
    );
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(hopper_cell.0, hopper_cell.1, hopper_cell.2),
            crate::world::HOPPER,
            "the hopper the platform cell the placement lands"
        );
    }
    // THE EXIT'S OWN SEAT (the air above the hopper — the arrival the
    // item the hopper's own top lands)
    let exit_cell = (hopper_cell.0, hopper_cell.1 + 1, hopper_cell.2);
    app.gateway_stores.insert(
        (gx, py + 1, gz),
        crate::screens::GatewayStore {
            exit: exit_cell,
            exact: true,
            cooldown: 0,
            age: crate::world::GATEWAY_SPAWN_TICKS,
        },
    );
    // THE ITEM'S OWN WALK (the spawn the gateway's own center, the
    // teleport the arrival speaks, the fall the hopper's top lands,
    // the suck walk the store banks)
    let mut rng = crate::drops::Lcg::new(13);
    app.drops.push(crate::drops::ItemDrop::spawn_entity_loot(
        9101,
        dirt,
        1,
        f64::from(gx) + 0.5,
        f64::from(py + 1) + 0.4,
        f64::from(gz) + 0.5,
        &mut rng,
    ));
    for _ in 0..24 {
        app.update();
    }
    // THE STORE'S OWN BANK (javap suckInItems the item-entity arm —
    // the whole stack the insert serves, the hopper's first slot the
    // dirt seats)
    assert_eq!(
        app.container_stores.get(&hopper_cell).map(|s| s[0]),
        Some((dirt, 1)),
        "the hopper the dirt the suck banks"
    );
    // THE DROPS' OWN DISCARD (the empty leftover the removal serves —
    // the item entity the list keeps silent)
    assert!(
        !app.drops.iter().any(|d| d.id == 9101),
        "the taken item the discard keeps"
    );
}

/// THE GATEWAY'S OWN BEAM STAR LANE (the autopush-316 round — the
/// REAL walk the starry surface + the spawn beam + the purple
/// re-fire the live clocks wear: the mesher's Y-face arm the UP +
/// DOWN sentinel quads bakes, the age clock the spawning ladder
/// climbs, the 2400-tick re-trigger the purple beam re-arms, the
/// item lane the cooldown arms, the starfield puffs the sampler
/// plants).
#[test]
fn the_offline_gateway_the_beam_the_starfield_the_clocks_wear() {
    let mut app = App::new(3);
    app.enter_world();
    // THE AIR PLATFORM (the stone plate the sites the test builds —
    // the terrain the interference never wears)
    let py = app.spawn_y + 10;
    let (gx, gz) = (24i32, 24i32);
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -8..=8 {
            for dz in -8..=8 {
                world.set(gx + dx, py, gz + dz, crate::world::STONE);
            }
        }
        // THE GATEWAY'S OWN SEAT (the fresh BE the walk stages — the
        // no-item law the placement lane never carries, the age-0
        // store the spawning beam's ladder arms)
        world.set(gx, py + 3, gz, crate::world::END_GATEWAY);
    }
    let gate_cell = (gx, py + 3, gz);
    app.gateway_stores.insert(
        gate_cell,
        crate::screens::GatewayStore {
            exit: (0, 0, 0),
            exact: false,
            cooldown: 0,
            age: 0,
        },
    );
    // THE CAMERA'S OWN SEAT (the animate block's body_cell the camera
    // proxy keeps — the furnace lanes' own staged look, the gateway
    // the down-look's own cell the sampler reaches)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(gx as f32 + 0.5, (py + 7) as f32, gz as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE CUBE'S OWN BAKE (javap the facesToShow walk the mesher's
    // static-law arm mirrors — the 317 audit the gateway's own
    // verdict re-pins: the open-air seat the SIX faces the static
    // law renders [the four horizontals the air neighbors the
    // sliver render wears — the 316 axis gate the PORTAL's own law
    // the swap retires], the sentinel tile the render pass
    // partitions on)
    {
        let world = app.offline_world.as_ref().unwrap();
        let mesh = crate::mesher::mesh_world(world, None);
        let gateway_quads: Vec<_> = mesh
            .iter()
            .filter(|q| q.tile == crate::mesher::TILE_END_GATEWAY)
            .collect();
        assert_eq!(
            gateway_quads.len(),
            6,
            "the open-air seat the six faces the static law renders"
        );
        for q in &gateway_quads {
            // THE FACES' OWN SEATS (the UP the wy+1 plane, the DOWN the
            // wy plane, the four sides the x/z cell planes — the EPS
            // inset the coplanar fight answers)
            let ys: Vec<f32> = q.pts.iter().map(|p| p[1]).collect();
            let up = ys
                .iter()
                .all(|&y| (y - (gate_cell.1 + 1) as f32).abs() < 0.01);
            let down = ys.iter().all(|&y| (y - gate_cell.1 as f32).abs() < 0.01);
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let west = xs.iter().all(|&x| (x - gate_cell.0 as f32).abs() < 0.01);
            let east = xs
                .iter()
                .all(|&x| (x - (gate_cell.0 + 1) as f32).abs() < 0.01);
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            let north = zs.iter().all(|&z| (z - gate_cell.2 as f32).abs() < 0.01);
            let south = zs
                .iter()
                .all(|&z| (z - (gate_cell.2 + 1) as f32).abs() < 0.01);
            assert!(
                up || down || west || east || north || south,
                "the face the six cell planes one bakes ({ys:?})"
            );
        }
        // THE REAL CULL WALK (the stone cap the setBlock stamps — the
        // full occluder above the shared face the static law culls,
        // the re-mesh the five faces the live world answers, the
        // slab's directional cross-section the variant byte splits:
        // the top slab the empty DOWN band the UP face restores, the
        // bottom slab the full band the cull re-arms)
        let mut world = app.offline_world.as_mut().unwrap();
        world.set(
            gate_cell.0,
            gate_cell.1 + 1,
            gate_cell.2,
            crate::world::STONE,
        );
        let mesh = crate::mesher::mesh_world(world, None);
        let capped: Vec<_> = mesh
            .iter()
            .filter(|q| q.tile == crate::mesher::TILE_END_GATEWAY)
            .collect();
        assert_eq!(
            capped.len(),
            5,
            "the stone cap the shared face the static law culls"
        );
        assert!(
            capped.iter().all(|q| !q
                .pts
                .iter()
                .all(|p| (p[1] - (gate_cell.1 + 1) as f32).abs() < 0.01)),
            "the capped UP plane the quad list retires"
        );
        world.set(
            gate_cell.0,
            gate_cell.1 + 1,
            gate_cell.2,
            crate::world::RESIN_BRICK_SLAB,
        );
        world.set_variant(
            gate_cell.0,
            gate_cell.1 + 1,
            gate_cell.2,
            crate::world::SLAB_TYPE_TOP,
        );
        let mesh = crate::mesher::mesh_world(world, None);
        assert_eq!(
            mesh.iter()
                .filter(|q| q.tile == crate::mesher::TILE_END_GATEWAY)
                .count(),
            6,
            "the top slab the empty DOWN band the UP face restores"
        );
        world.set_variant(
            gate_cell.0,
            gate_cell.1 + 1,
            gate_cell.2,
            crate::world::SLAB_TYPE_BOTTOM,
        );
        let mesh = crate::mesher::mesh_world(world, None);
        assert_eq!(
            mesh.iter()
                .filter(|q| q.tile == crate::mesher::TILE_END_GATEWAY)
                .count(),
            5,
            "the bottom slab the full DOWN band the cull re-arms"
        );
    }
    // THE AGE CLOCK'S OWN FIRST TICK (javap portalTick the age++ the
    // every-tick wear — the real update the store's clock banks)
    app.update();
    assert_eq!(
        app.gateway_stores.get(&gate_cell).unwrap().age,
        1,
        "the age the tick banks"
    );
    // THE SPAWN BEAM'S OWN LADDER (javap extractRenderState the
    // spawning arm — the age the walk banks the percent the sin
    // ladder climbs, the render-side fold the live clocks replay:
    // the mid-climb the MAGENTA beam the world's own max answers)
    for _ in 0..99 {
        app.update();
    }
    {
        let st = app.gateway_stores.get(&gate_cell).copied().unwrap();
        assert_eq!(st.age, 100, "the climb the age banks");
        assert!(
            crate::world::gateway_is_spawning(st.age),
            "the window the spawning gate keeps"
        );
        let scale = crate::world::gateway_beam_scale(true, st.age, st.cooldown, 0.0);
        let height =
            crate::world::gateway_beam_height(scale, true, crate::world::CHUNK_Y as i32 - 1);
        assert_eq!(
            (scale, height),
            (1.0, crate::world::CHUNK_Y as i32 - 1),
            "the half-climb the peak wears — the MAGENTA beam the world max climbs"
        );
        assert_eq!(
            crate::world::gateway_beam_color(true),
            crate::world::GATEWAY_BEAM_MAGENTA,
            "the spawning arm the MAGENTA row keeps"
        );
    }
    // THE WINDOW'S OWN RETIRE (javap the `age < 200` gate — the walk
    // past the window the scale the zero fold answers, the beam the
    // quiet gateway keeps)
    for _ in 0..100 {
        app.update();
    }
    {
        let st = app.gateway_stores.get(&gate_cell).copied().unwrap();
        assert_eq!(st.age, 200);
        assert!(
            !crate::world::gateway_is_spawning(st.age),
            "the window the spawn gate retires"
        );
        // THE HEIGHT-0 GATE (javap the floor fold the tiny rung the
        // zero answer keeps — the drained store the beam retires; the
        // table's 32767 row the +9.6e-5 sliver the floor retires)
        let scale = crate::world::gateway_beam_scale(false, st.age, st.cooldown, 0.0);
        let height =
            crate::world::gateway_beam_height(scale, false, crate::world::CHUNK_Y as i32 - 1);
        assert_eq!(
            height, 0,
            "the drained store the height-0 gate the beam retires ({scale})"
        );
    }
    // THE ITEM LANE'S OWN COOLDOWN (javap entityInside the
    // triggerCooldown the enter tick writes — the REAL drop the 40
    // the store banks, the purple beam the cooling arm serves)
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut rng = crate::drops::Lcg::new(31);
    app.drops.push(crate::drops::ItemDrop::spawn_entity_loot(
        9201,
        dirt,
        1,
        f64::from(gate_cell.0) + 0.5,
        f64::from(gate_cell.1) + 0.4,
        f64::from(gate_cell.2) + 0.5,
        &mut rng,
    ));
    app.update();
    {
        let st = app.gateway_stores.get(&gate_cell).copied().unwrap();
        assert_eq!(
            st.cooldown,
            crate::world::GATEWAY_COOLDOWN_TICKS,
            "the enter tick the cooldown writes"
        );
    }
    // THE PURPLE BEAM'S OWN LADDER (javap the cooling arm — the drain
    // the percent climbs, the mid-drain rung the peak the 50.0 base
    // answers)
    for _ in 0..20 {
        app.update();
    }
    {
        let st = app.gateway_stores.get(&gate_cell).copied().unwrap();
        assert_eq!(
            st.cooldown,
            crate::world::GATEWAY_COOLDOWN_TICKS - 20,
            "the drain the -1 the tick wears"
        );
        let scale = crate::world::gateway_beam_scale(false, st.age, st.cooldown, 0.0);
        let height =
            crate::world::gateway_beam_height(scale, false, crate::world::CHUNK_Y as i32 - 1);
        assert_eq!(
            (scale, height),
            (1.0, 50),
            "the mid-drain the peak wears — the purple beam the 50.0 ceiling keeps"
        );
        assert_eq!(
            crate::world::gateway_beam_color(false),
            crate::world::GATEWAY_BEAM_PURPLE,
            "the cooling arm the PURPLE row wears"
        );
    }
    // THE DRAIN'S OWN SILENCE (the 20 remaining ticks the decay walks —
    // the beam the drained store retires)
    for _ in 0..20 {
        app.update();
    }
    assert_eq!(
        app.gateway_stores.get(&gate_cell).unwrap().cooldown,
        0,
        "the drain the cooldown empties"
    );
    // THE 2400-TICK RE-TRIGGER (javap portalTick the `age % 2400 ==
    // 0` verdict the triggerCooldown walk serves — the staged age the
    // one tick the purple beam re-arms)
    {
        let st = app.gateway_stores.get_mut(&gate_cell).unwrap();
        st.age = 2399;
    }
    app.update();
    {
        let st = app.gateway_stores.get(&gate_cell).copied().unwrap();
        assert_eq!(
            st.cooldown,
            crate::world::GATEWAY_COOLDOWN_TICKS,
            "the re-trigger the 2400-tick fold arms"
        );
        assert_eq!(st.age, 2400, "the re-trigger the age the fold wears");
    }
    // THE STARFIELD'S OWN PUFFS (javap EndGatewayBlock.animateTick the
    // 26.2 jar — the sampler the gateway cell the 2-puff walk hits,
    // the Portal particles the seeded streams the deterministic
    // visits answer, the furnace lanes' own 600-tick idiom; the
    // lifetime the 40-49 ticks the expiry wears — the PEAK the probe
    // banks, the end-of-walk list the expired rows never keeps)
    let mut gateway_puff_peak = 0usize;
    for _ in 0..600 {
        app.update();
        let alive = app
            .particles
            .iter()
            .filter(|p| match p.kind {
                crate::particles::ParticleKind::Portal { sx, sy, sz, .. } => {
                    // THE ANCHOR'S OWN SEAT (the ctor's own Start fields
                    // the swirl never rewrites — the spawn pos the
                    // cell's own space keeps)
                    (sx.floor() as i32, sy.floor() as i32, sz.floor() as i32) == gate_cell
                }
                _ => false,
            })
            .count();
        gateway_puff_peak = gateway_puff_peak.max(alive);
    }

    assert!(
        gateway_puff_peak > 0,
        "the gateway the sampler hears: the peak {} portal puffs over 600 ticks",
        gateway_puff_peak
    );
}

/// THE BED'S OWN PLACE WALK (the autopush-311 round — javap
/// BedBlock.getStateForPlacement the 26.2 jar the RAW look the
/// FACING answers, the setPlacedBy the pos.relative(FACING) head
/// cell the PART=HEAD stamp wears — the REAL placement walk the
/// held bed item the right-press the both cells the variant bytes
/// stamps).
#[test]
fn the_offline_bed_the_place_walk_the_two_halves_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let red_bed = mc_registry::item_id_by_name("red_bed").unwrap();
    app.inventory.items[0] = (red_bed, 2);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE PAD (the two cells the ground the walk flattens — the foot
    // the clicked cell the head the south neighbour keeps)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c, surface + 1, c, crate::world::AIR);
        world.set(c, surface + 1, c + 1, crate::world::AIR);
        world.set(c, surface, c, crate::world::STONE);
        world.set(c, surface, c + 1, crate::world::STONE);
    }
    // THE CAMERA (the floor look the ray enters, the yaw the SOUTH
    // facing the head's own side answers — the cam_yaw PI the
    // vanilla 0 the raw look speaks)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE PRESS (the head cell's give-way verdict the placement arms)
    assert!(
        app.on_right_press(0, 0),
        "the bed placement consumed the press"
    );
    let foot_cell = (c, surface + 1, c);
    let head_cell = (c, surface + 1, c + 1);
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(foot_cell.0, foot_cell.1, foot_cell.2),
            crate::world::BED_RED,
            "the foot the clicked cell the placement lands"
        );
        assert_eq!(
            world.get(head_cell.0, head_cell.1, head_cell.2),
            crate::world::BED_RED,
            "the head the look's own far side the setPlacedBy lands"
        );
        let fv = world.variant(foot_cell.0, foot_cell.1, foot_cell.2);
        assert_eq!(
            crate::world::bed_variant_part(fv),
            crate::world::BED_PART_FOOT,
            "the foot the part bit keeps"
        );
        assert_eq!(
            crate::world::bed_variant_facing(fv),
            crate::world::BED_FACING_SOUTH,
            "the raw look the south byte stamps"
        );
        let hv = world.variant(head_cell.0, head_cell.1, head_cell.2);
        assert_eq!(
            crate::world::bed_variant_part(hv),
            crate::world::BED_PART_HEAD,
            "the head the third bit stamps"
        );
        assert_eq!(
            crate::world::bed_variant_facing(hv),
            crate::world::BED_FACING_SOUTH,
            "the same facing the both cells wear"
        );
    }
    assert_eq!(
        app.inventory.items[0],
        (red_bed, 1),
        "the survival consume the stack shrinks"
    );
}

/// THE BED'S OWN OCCUPIED-HEAD REFUSAL (javap BedBlock
/// .getStateForPlacement the 26.2 jar the canBeReplaced fold the
/// `aconst_null` refusal keeps — the occupied head cell the
/// placement never arms, the press the PASS the item lane never
/// consumes).
#[test]
fn the_offline_bed_the_occupied_head_the_refusal_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let red_bed = mc_registry::item_id_by_name("red_bed").unwrap();
    app.inventory.items[0] = (red_bed, 2);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE PAD (the head cell the STONE the verdict refuses)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c, surface + 1, c, crate::world::AIR);
        world.set(c, surface + 1, c + 1, crate::world::STONE);
        world.set(c, surface, c, crate::world::STONE);
        world.set(c, surface, c + 1, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the occupied head the placement refuses"
    );
    {
        let world = app.offline_world.as_ref().unwrap();
        assert_eq!(
            world.get(c, surface + 1, c),
            crate::world::AIR,
            "the foot cell the refusal never stamps"
        );
        assert_eq!(
            world.get(c, surface + 1, c + 1),
            crate::world::STONE,
            "the occupied head the refusal keeps"
        );
    }
    assert_eq!(
        app.inventory.items[0],
        (red_bed, 2),
        "the refusal the stack keeps"
    );
}

/// THE BED'S OWN HEAD-ONLY DROP (javap the jar's loot —
/// data/minecraft/loot_table/blocks/red_bed.json the
/// `block_state_property` condition the `part=head` row speaks —
/// the HEAD's break alone rolls the bed item, the FOOT's own break
/// the condition refuses; the updateShape the AIR return the mate
/// the silent keep — the REAL dig walk the both halves the vanilla's
/// own asymmetry plays).
#[test]
fn the_offline_bed_the_break_the_head_only_drop_wears() {
    let red_bed = mc_registry::item_id_by_name("red_bed").unwrap();
    // PHASE ONE: the FOOT's break — the mate vanishes, no item speaks
    {
        let mut app = App::new(3);
        app.enter_world();
        app.inventory.items[0] = (red_bed, 1);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        {
            let world = app.offline_world.as_mut().unwrap();
            world.set(c, surface + 1, c, crate::world::AIR);
            world.set(c, surface + 1, c + 1, crate::world::AIR);
            world.set(c, surface, c, crate::world::STONE);
            world.set(c, surface, c + 1, crate::world::STONE);
        }
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the bed the placement lands");
        // THE FOOT'S OWN DIG (the SAME camera the foot cell the ray
        // enters — the placement's own click the dig's own target
        // wears, the press arms the destroy, the ticks the cell
        // breaks)
        app.on_press(0, 0);
        let mut ticks = 0;
        while app.offline_world.as_ref().unwrap().get(c, surface + 1, c) != crate::world::AIR {
            app.update();
            ticks += 1;
            assert!(ticks < 400, "the foot's dig completed in time");
        }
        // THE MATE'S OWN VANISH (javap updateShape the AIR return —
        // the head the silent keep wears)
        assert_eq!(
            app.offline_world
                .as_ref()
                .unwrap()
                .get(c, surface + 1, c + 1),
            crate::world::AIR,
            "the head the mate the updateShape the silent keep wears"
        );
        // THE CONDITION'S OWN REFUSE (the part=head row the foot's
        // break never rolls — the drops list the honest empty keeps)
        assert!(
            !app.drops.iter().any(|d| d.item == red_bed),
            "the foot's break the condition refuses (no item)"
        );
    }
    // PHASE TWO: the HEAD's break — the item the condition rolls
    {
        let mut app = App::new(3);
        app.enter_world();
        app.inventory.items[0] = (red_bed, 1);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        {
            let world = app.offline_world.as_mut().unwrap();
            world.set(c, surface + 1, c, crate::world::AIR);
            world.set(c, surface + 1, c + 1, crate::world::AIR);
            world.set(c, surface, c, crate::world::STONE);
            world.set(c, surface, c + 1, crate::world::STONE);
        }
        // THE PLACEMENT'S OWN CAMERA (the foot cell the ray enters —
        // the foot the clicked cell the head the south neighbour the
        // setPlacedBy lands)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the bed the placement lands");
        // THE HEAD'S OWN DIG (the camera the head cell the ray
        // enters — the moved pose the head's own cell the dig's own
        // target wears)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 1.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        app.on_press(0, 0);
        let mut ticks = 0;
        while app
            .offline_world
            .as_ref()
            .unwrap()
            .get(c, surface + 1, c + 1)
            != crate::world::AIR
        {
            app.update();
            ticks += 1;
            assert!(ticks < 400, "the head's dig completed in time");
        }
        // THE MATE'S OWN VANISH (the foot the updateShape the AIR
        // return the silent keep wears)
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface + 1, c),
            crate::world::AIR,
            "the foot the mate the updateShape the silent keep wears"
        );
        // THE CONDITION'S OWN ROLL (the part=head row the head's break
        // rolls — the ONE bed item the popResource law spawns)
        let bed_drops = app.drops.iter().filter(|d| d.item == red_bed).count();
        assert_eq!(bed_drops, 1, "the head's break the condition rolls");
    }
}

/// THE BED'S OWN DAY PRESS (the autopush-312 round — javap 26.2
/// ServerPlayer.startSleepInBed the live disasm the walk banks —
/// scripts/jar_work/bed_sleep_312.txt the dumps serve: the
/// setRespawnPosition arm the canSetSpawn rule the BEFORE canSleep
/// order wears — the day press the bank still serves, the
/// SPAWN_SET_MESSAGE the chat speaks, the !canSleep the asProblem
/// left the useWithoutItem's ifLeft the displayClientMessage overlay
/// serves — the BedRule WHEN_DARK the no_sleep row the verdict
/// speaks; the REAL right-press walk the placed bed the day clock
/// the press arms).
#[test]
fn the_offline_bed_the_day_press_the_spawn_bank_the_refusal_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let red_bed = mc_registry::item_id_by_name("red_bed").unwrap();
    app.inventory.items[0] = (red_bed, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE PAD (the 311 placement walk's own shape — the head's above
    // cells the obstruction verdict never refuses)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c, surface + 1, c, crate::world::AIR);
        world.set(c, surface + 1, c + 1, crate::world::AIR);
        world.set(c, surface + 2, c, crate::world::AIR);
        world.set(c, surface + 2, c + 1, crate::world::AIR);
        world.set(c, surface, c, crate::world::STONE);
        world.set(c, surface, c + 1, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the bed the placement lands");
    let head_cell = (c, surface + 1, c + 1);
    // THE PLAYER'S OWN SEAT (the test lane's body the headless walk
    // never creates — the reach verdict the vanilla startSleepInBed
    // the player's own pos reads, the CAMERA the body the highest
    // authority wears: the spawn cell the body banks, the -90 pitch
    // the bed's own top the ray enters)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        app.spawn_y as f64,
        c as f64 + 0.5,
        180.0,
        90.0,
    ));
    // THE DAY CLOCK (the day marker's own 1000 the bright outside
    // keeps — the WHEN_DARK verdict the refusal arms)
    app.clock_total = 1_000;
    // THE PRESS (the bed arm the sleep door the verdict chain serves)
    assert!(
        app.on_right_press(0, 0),
        "the day press the SUCCESS the use consumes"
    );
    // THE SPAWN POINT'S OWN SET (pcs 127-161 — the bank the BEFORE
    // canSleep order wears, the anchor the setPosToBed triple banks)
    let (ax, ay, az) = crate::world::bed_sleep_anchor(head_cell.0, head_cell.1, head_cell.2);
    assert_eq!(
        app.respawn_point.map(|(x, y, z, _, _)| (x, y, z)),
        Some((ax, ay, az)),
        "the day press the spawn bank the head anchor wears"
    );
    // THE MESSAGE'S OWN CHAT (javap setRespawnPosition pc 19-23 — the
    // SPAWN_SET_MESSAGE the sendSystemMessage walk speaks)
    assert!(
        app.chat_messages
            .iter()
            .any(|(m, _)| m == "Respawn point set"),
        "the spawn set the chat speaks"
    );
    // THE REFUSAL'S OWN OVERLAY (pc 164 the !canSleep the asProblem
    // left the ifLeft the displayClientMessage overlay serves — the
    // no_sleep row the BedRule WHEN_DARK keeps)
    assert_eq!(
        app.overlay_message.as_deref(),
        Some("You can sleep only at night or during thunderstorms"),
        "the day press the no_sleep the overlay wears"
    );
    // THE SLEEP'S OWN REFUSE (the awake body the sleep never stamps)
    assert!(
        app.sleeping_bed.is_none(),
        "the day press the sleep refuses"
    );
    assert_eq!(app.sleep_counter, 0, "the refusal the counter keeps");
    assert!(
        !app.sleep_screen_open,
        "the refusal the sleep screen never opens"
    );
    let b = app.body.as_ref().unwrap();
    assert!(!b.sleeping, "the refusal the pose the standing keeps");
}

/// THE BED'S OWN NIGHT SLEEP (the autopush-312 round — javap 26.2
/// ServerLevel.tick pcs 85-204 the live disasm the walk banks —
/// scripts/jar_work/bed_sleep_312.txt the dumps serve: the deep-sleep
/// counter the 100 ticks the areEnoughDeepSleeping verdict arms, the
/// morning's own jump the moveToTimeMarker(WAKE_UP_FROM_SLEEP) the
/// 26.2 timeline's own `wake_up_from_sleep: 0` row serves, the
/// weather's own clear the resetWeatherCycle the isRaining gate
/// arms, the wakeUpAllPlayers the stopSleepInBed(true,false) the
/// tail serves — the REAL sleep walk the placed bed the night clock
/// the press the 100 updates the skip plays).
#[test]
fn the_offline_bed_the_night_press_the_sleep_the_skip_the_morning_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let red_bed = mc_registry::item_id_by_name("red_bed").unwrap();
    app.inventory.items[0] = (red_bed, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c, surface + 1, c, crate::world::AIR);
        world.set(c, surface + 1, c + 1, crate::world::AIR);
        world.set(c, surface + 2, c, crate::world::AIR);
        world.set(c, surface + 2, c + 1, crate::world::AIR);
        world.set(c, surface, c, crate::world::STONE);
        world.set(c, surface, c + 1, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the bed the placement lands");
    let head_cell = (c, surface + 1, c + 1);
    // THE PLAYER'S OWN SEAT (the test lane's body the headless walk
    // never creates — the reach verdict the vanilla startSleepInBed
    // the player's own pos reads, the CAMERA the body the highest
    // authority wears: the spawn cell the body banks, the -90 pitch
    // the bed's own top the ray enters)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        app.spawn_y as f64,
        c as f64 + 0.5,
        180.0,
        90.0,
    ));
    // THE NIGHT CLOCK (the midnight's own 18000 the dark outside the
    // verdict keeps) + THE RAIN (the resetWeatherCycle the isRaining
    // gate the skip's clear arm proves)
    app.clock_total = 18_000;
    app.set_offline_weather(1.0, 0.0);
    // THE PRESS (the sleep door the dark verdict the sleep stamps)
    assert!(
        app.on_right_press(0, 0),
        "the night press the SUCCESS the use consumes"
    );
    assert_eq!(
        app.sleeping_bed,
        Some(head_cell),
        "the sleep the head cell the bank keeps"
    );
    assert!(app.sleep_screen_open, "the sleep screen the walk opens");
    let (ax, ay, az) = crate::world::bed_sleep_anchor(head_cell.0, head_cell.1, head_cell.2);
    {
        let b = app.body.as_ref().unwrap();
        assert!(b.sleeping, "the pose the sleeping flag wears");
        assert_eq!(
            (b.x, b.y, b.z),
            (ax, ay, az),
            "the anchor the setPosToBed lands"
        );
        assert_eq!(
            (b.vx, b.vy, b.vz),
            (0.0, 0.0, 0.0),
            "the delta the ZERO stamps"
        );
    }
    assert!(
        app.chat_messages
            .iter()
            .any(|(m, _)| m == "Respawn point set"),
        "the spawn set the night press the chat speaks"
    );
    // THE DEEP-SLEEP'S OWN WALK (the counter the per-tick increment —
    // the 100 the verdict arms, the mid-sleep ticks the frozen body
    // the anchor keeps)
    for i in 0..99u32 {
        app.update();
        assert_eq!(
            app.sleeping_bed,
            Some(head_cell),
            "the tick {i} the sleep the walk keeps"
        );
        let b = app.body.as_ref().unwrap();
        assert_eq!(
            (b.x, b.y, b.z),
            (ax, ay, az),
            "the tick {i} the freeze the anchor keeps"
        );
    }
    assert_eq!(
        app.sleep_counter, 99,
        "the 99th tick the counter the mid-ease wears"
    );
    // THE 100TH TICK'S OWN SKIP (the deep-sleep verdict the
    // areEnoughDeepSleeping arms — the update the counter the 100 the
    // walk spends, the morning's own jump the same tick wears)
    app.update();
    // THE MORNING'S OWN JUMP (pcs 149-170 — the clock the NEXT tick-0
    // the WAKE_UP_FROM_SLEEP marker wears: the press tick the clock
    // the 18000 + the 100 deep-sleep ticks the 18100 walked, the jump
    // the 24000 the dawn answers)
    assert_eq!(
        app.day_time_now(),
        24_000,
        "the skip the dawn the clock wears"
    );
    // THE WEATHER'S OWN CLEAR (pcs 194-204 — the isRaining gate the
    // resetWeatherCycle the rain the zero set wears)
    assert_eq!(
        app.offline_world.as_ref().unwrap().rain_level,
        0.0,
        "the skip the rain the clear serves"
    );
    // THE WAKE'S OWN TAIL (pcs 171-174 — the wakeUpAllPlayers the
    // stopSleepInBed(true,false) the counter the zero banks)
    assert!(app.sleeping_bed.is_none(), "the skip the wake serves");
    assert_eq!(app.sleep_counter, 0, "the wake the counter resets");
    assert!(!app.sleep_screen_open, "the wake the sleep screen closes");
    let b = app.body.as_ref().unwrap();
    assert!(!b.sleeping, "the wake the pose the standing keeps");
}

/// THE BED'S OWN TOO-FAR REFUSAL (javap 26.2 ServerPlayer
/// .startSleepInBed pc 95 the bedInRange the TOO_FAR_AWAY the 3/2/3
/// band the band's outside wears — the reach verdict the BEFORE
/// spawn-set order the disasm speaks: the far press the bank never
/// serves, the REAL right-press walk the body the 5-block south the
/// verdict plays).
#[test]
fn the_offline_bed_the_too_far_the_refusal_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let red_bed = mc_registry::item_id_by_name("red_bed").unwrap();
    app.inventory.items[0] = (red_bed, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c, surface + 1, c, crate::world::AIR);
        world.set(c, surface + 1, c + 1, crate::world::AIR);
        world.set(c, surface + 2, c, crate::world::AIR);
        world.set(c, surface + 2, c + 1, crate::world::AIR);
        world.set(c, surface, c, crate::world::STONE);
        world.set(c, surface, c + 1, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the bed the placement lands");
    // THE PLAYER'S OWN SEAT (the test lane's body the headless walk
    // never creates — the reach verdict the vanilla startSleepInBed
    // the player's own pos reads, the CAMERA the body the highest
    // authority wears: the spawn cell the body banks, the -90 pitch
    // the bed's own top the ray enters)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        app.spawn_y as f64,
        c as f64 + 0.5,
        180.0,
        90.0,
    ));
    // THE FAR BODY (the 3-block lift the dy band's outside wears —
    // the 3.0 the |dy| <= 2.0 band refuses, the eye the 4.62 the bed
    // top the 4.06 the clip range keeps: the vanilla's own 3..4.5
    // window the reach-vs-range pair speaks)
    {
        let b = app.body.as_mut().unwrap();
        b.y += 3.0;
    }
    app.clock_total = 18_000;
    assert!(
        app.on_right_press(0, 0),
        "the far press the SUCCESS the use consumes"
    );
    // THE REFUSAL'S OWN OVERLAY (pc 104-110 — the TOO_FAR_AWAY the
    // asProblem the ifLeft the displayClientMessage overlay serves)
    assert_eq!(
        app.overlay_message.as_deref(),
        Some("You may not rest now; the bed is too far away"),
        "the far press the too_far the overlay wears"
    );
    // THE BANK'S OWN REFUSE (the reach verdict the BEFORE spawn-set
    // order the disasm speaks — the far press the bank never serves)
    assert!(
        app.respawn_point.is_none(),
        "the far press the bank refuses"
    );
    assert!(
        app.sleeping_bed.is_none(),
        "the far press the sleep refuses"
    );
}

/// THE BED'S OWN ESC WAKE (the autopush-312 round — javap 26.2
/// InBedChatScreen the shouldCloseOnEsc default the onClose walk the
/// sendWakeUp ride serves — the ServerboundPlayerCommandPacket
/// (STOP_SLEEPING) the wire speaks, the offline walk the
/// wake_from_bed lane answers: the STANDING pose the pos keeps [the
/// bed anchor the vanilla stopSleeping's own setPos(current) walk],
/// the sleep screen the close wears, the REAL sleep walk the ESC the
/// wake plays).
#[test]
fn the_offline_bed_the_esc_the_leave_the_wake_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let red_bed = mc_registry::item_id_by_name("red_bed").unwrap();
    app.inventory.items[0] = (red_bed, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c, surface + 1, c, crate::world::AIR);
        world.set(c, surface + 1, c + 1, crate::world::AIR);
        world.set(c, surface + 2, c, crate::world::AIR);
        world.set(c, surface + 2, c + 1, crate::world::AIR);
        world.set(c, surface, c, crate::world::STONE);
        world.set(c, surface, c + 1, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the bed the placement lands");
    app.clock_total = 18_000;
    // THE PLAYER'S OWN SEAT (the test lane's body the headless walk
    // never creates — the reach verdict the vanilla startSleepInBed
    // the player's own pos reads, the CAMERA the body the highest
    // authority wears: the spawn cell the body banks, the +90 pitch
    // the bed's own top the ray enters)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        app.spawn_y as f64,
        c as f64 + 0.5,
        180.0,
        90.0,
    ));
    app.clock_total = 18_000;
    assert!(app.on_right_press(0, 0), "the night press the sleep stamps");
    let (ax, ay, az) = crate::world::bed_sleep_anchor(c, surface + 1, c + 1);
    // THE DEEP-SLEEP'S OWN PART WALK (the 40 ticks the counter the
    // mid-ease wears — the ESC the wake the mid-sleep serves)
    for _ in 0..40 {
        app.update();
    }
    assert_eq!(app.sleep_counter, 40, "the mid-sleep the counter walks");
    // THE ESC'S OWN WAKE (the InBedChatScreen's onClose the
    // sendWakeUp walk the offline lane answers)
    app.on_escape();
    assert!(app.sleeping_bed.is_none(), "the ESC the wake serves");
    assert_eq!(app.sleep_counter, 0, "the wake the counter the zero banks");
    assert!(!app.sleep_screen_open, "the wake the sleep screen closes");
    // THE PAUSE'S OWN REFUSE (the ESC the sleep screen consumed — the
    // wake the pause never arms)
    assert!(!app.paused, "the ESC the sleep screen the pause refuses");
    let b = app.body.as_ref().unwrap();
    assert!(!b.sleeping, "the wake the pose the standing keeps");
    // THE POS'S OWN STAY (javap Avatar.stopSleeping — the setPos
    // (current position) walk the bed anchor the body keeps)
    assert_eq!(
        (b.x, b.y, b.z),
        (ax, ay, az),
        "the wake the pos the anchor keeps"
    );
    // THE CLOCK'S OWN WALK (the ESC wake the clock the local advance
    // keeps — the skip never armed, the night the 18000 + the ticks
    // the walk speaks)
    assert_eq!(
        app.day_time_now(),
        18_040,
        "the ESC wake the clock the night keeps"
    );
}

/// THE BED'S OWN RESPAWN RETURN (the autopush-313 round — javap 26.2
/// ServerPlayer.findRespawnPositionAndUseSpawnBlock pcs 23-94 the
/// static findRespawnAndUseSpawnBlock the scan law re-seats —
/// scripts/jar_work/bed_respawn{,2,3}_313.txt the dumps serve: the
/// banked RespawnData the bed arm the around-bed scan answers, the
/// RespawnPosAngle (position, yaw, pitch) record the walk wears —
/// the REAL walk the placed bed the sleep the skip the far death the
/// respawn the scan's own stand-up spot the body re-seats).
#[test]
fn the_offline_bed_the_respawn_return_the_scan_the_reseat_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let red_bed = mc_registry::item_id_by_name("red_bed").unwrap();
    app.inventory.items[0] = (red_bed, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE PAD (the scan's own ring the ground the walk flattens — the
    // bed's two cells + the surround rows the strict pass walks: the
    // stone row the floor the fold lands on, the two air rows the
    // bbox the overlap never finds)
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -1i32..=1 {
            for dz in -1i32..=2 {
                world.set(c + dx, surface, c + dz, crate::world::STONE);
                world.set(c + dx, surface + 1, c + dz, crate::world::AIR);
                world.set(c + dx, surface + 2, c + dz, crate::world::AIR);
            }
        }
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the bed the placement lands");
    let head_cell = (c, surface + 1, c + 1);
    // THE PLAYER'S OWN SEAT (the 312 walk's own pattern — the body the
    // spawn cell the reach verdict reads, the yaw 180 the RespawnData
    // the banked angle wears)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        app.spawn_y as f64,
        c as f64 + 0.5,
        180.0,
        90.0,
    ));
    // THE NIGHT PRESS (the sleep door the dark verdict the sleep
    // stamps — the spawn bank the BEFORE canSleep order wears)
    app.clock_total = 18_000;
    assert!(app.on_right_press(0, 0), "the night press the sleep stamps");
    assert_eq!(app.sleeping_bed, Some(head_cell));
    let (ax, ay, az) = crate::world::bed_sleep_anchor(head_cell.0, head_cell.1, head_cell.2);
    assert_eq!(
        app.respawn_point.map(|(x, y, z, _, _)| (x, y, z)),
        Some((ax, ay, az)),
        "the press the RespawnData the head anchor banks"
    );
    // THE DEEP-SLEEP'S OWN SKIP (the 100 ticks the morning jump wears
    // — the wake the walk's own loop completes)
    for _ in 0..100 {
        app.update();
    }
    assert!(app.sleeping_bed.is_none(), "the skip the wake serves");
    assert_eq!(app.day_time_now(), 24_000, "the skip the dawn wears");
    // THE WALK'S OWN DEPARTURE (the far spot the body the honest move
    // wears — the respawn the teleport-back the distance never keeps)
    {
        let b = app.body.as_mut().unwrap();
        b.x = c as f64 + 6.5;
        b.y = app.spawn_y as f64;
        b.z = c as f64 + 0.5;
        b.vx = 0.0;
        b.vy = 0.0;
        b.vz = 0.0;
    }
    // THE DEATH (the /kill the honest death walk the part-1 lane wears)
    app.chat_open = true;
    app.chat_input = "/kill".into();
    app.on_chat_submit();
    assert!(app.dead, "the kill the death arms");
    // THE RESPAWN (the DeathScreen's own button walk — the static
    // findRespawnAndUseSpawnBlock the scan law re-seats: the bed arm
    // the intact bed the scan answers — the yaw-180 the table sine
    // the EAST side pick serves, the row0 spot the head's east
    // neighbour the strict pass lands)
    app.respawn();
    assert!(!app.dead, "the respawn the death clears");
    let b = app.body.as_ref().unwrap();
    assert_eq!(
        (b.x, b.y, b.z),
        (c as f64 + 1.5, (surface + 1) as f64, c as f64 + 1.5,),
        "the scan's row0 spot the body re-seats (the east neighbour the ground floor wears)"
    );
    assert_eq!(
        (b.yaw, b.pitch),
        (180.0, 90.0),
        "the RespawnPosAngle's own pair the banked angles wear"
    );
    // THE QUIET LANE (the bed intact — the not_valid message never
    // speaks, the missingRespawnBlock arm never enters)
    assert!(
        !app.chat_messages
            .iter()
            .any(|(m, _)| m.contains("no home bed")),
        "the intact bed the chat quiet keeps"
    );
    assert_eq!(app.health, MAX_HEALTH, "the respawn the vitals reset");
}

/// THE BED'S OWN BROKEN-BED RETURN (the autopush-313 round — the
/// generic arm the passable pair the 0.1 lift rides: the bed broken
/// the banked pos the air cells answer, the same spot the respawn
/// keeps [the vanilla feel the broken bed the respawn wears] — the
/// REAL walk the sleep the skip the dig the death the respawn the
/// generic arm's own spot wears).
#[test]
fn the_offline_bed_the_broken_bed_the_generic_arm_the_spot_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let red_bed = mc_registry::item_id_by_name("red_bed").unwrap();
    app.inventory.items[0] = (red_bed, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -1i32..=1 {
            for dz in -1i32..=2 {
                world.set(c + dx, surface, c + dz, crate::world::STONE);
                world.set(c + dx, surface + 1, c + dz, crate::world::AIR);
                world.set(c + dx, surface + 2, c + dz, crate::world::AIR);
            }
        }
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the bed the placement lands");
    let head_cell = (c, surface + 1, c + 1);
    let foot_cell = (c, surface + 1, c);
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        app.spawn_y as f64,
        c as f64 + 0.5,
        180.0,
        90.0,
    ));
    app.clock_total = 18_000;
    assert!(app.on_right_press(0, 0), "the night press the sleep stamps");
    assert_eq!(
        app.respawn_point.map(|(x, y, z, _, _)| (x, y, z)),
        Some(crate::world::bed_sleep_anchor(
            head_cell.0,
            head_cell.1,
            head_cell.2
        ))
    );
    for _ in 0..100 {
        app.update();
    }
    assert!(app.sleeping_bed.is_none(), "the skip the wake serves");
    // THE BED'S OWN BREAK (the REAL dig walk the 311 lane wears — the
    // placement's own camera the foot cell the ray enters, the press
    // arms the destroy, the ticks the cell breaks, the mate the
    // updateShape the silent keep wears)
    app.on_press(0, 0);
    let mut ticks = 0;
    while app
        .offline_world
        .as_ref()
        .unwrap()
        .get(foot_cell.0, foot_cell.1, foot_cell.2)
        != crate::world::AIR
    {
        app.update();
        ticks += 1;
        assert!(ticks < 400, "the foot's dig completed in time");
    }
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .get(head_cell.0, head_cell.1, head_cell.2),
        crate::world::AIR,
        "the head the mate the updateShape the silent keep wears"
    );
    // THE DEATH + THE RESPAWN (the generic arm the passable pair the
    // 0.1 lift rides — the banked pos the air cells the same spot
    // answers)
    app.chat_open = true;
    app.chat_input = "/kill".into();
    app.on_chat_submit();
    assert!(app.dead, "the kill the death arms");
    app.respawn();
    assert!(!app.dead, "the respawn the death clears");
    let b = app.body.as_ref().unwrap();
    assert_eq!(
        (b.x, b.y, b.z),
        (
            head_cell.0 as f64 + 0.5,
            head_cell.1 as f64 + 0.1,
            head_cell.2 as f64 + 0.5,
        ),
        "the generic arm the 0.1-lift spot the body re-seats"
    );
    assert_eq!(
        (b.yaw, b.pitch),
        (180.0, 90.0),
        "the RespawnPosAngle's own pair the banked angles wear"
    );
    // THE QUIET LANE (the generic arm answered — the not_valid message
    // never speaks)
    assert!(
        !app.chat_messages
            .iter()
            .any(|(m, _)| m.contains("no home bed")),
        "the generic arm the chat quiet keeps"
    );
}

/// THE BED'S OWN BUILT-OVER FALLBACK (the autopush-313 round — the
/// EMPTY verdict the missingRespawnBlock arm the shared spawn + the
/// spawn.not_valid message serves: the bed broken AND the banked spot
/// the stone occupies — the generic arm's passable pair refuses, the
/// world spawn the fallback wears, the chat the obstruction speaks —
/// the REAL walk the sleep the skip the dig the occupy the death the
/// respawn the fallback plays).
#[test]
fn the_offline_bed_the_built_over_the_fallback_the_message_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let red_bed = mc_registry::item_id_by_name("red_bed").unwrap();
    app.inventory.items[0] = (red_bed, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -1i32..=1 {
            for dz in -1i32..=2 {
                world.set(c + dx, surface, c + dz, crate::world::STONE);
                world.set(c + dx, surface + 1, c + dz, crate::world::AIR);
                world.set(c + dx, surface + 2, c + dz, crate::world::AIR);
            }
        }
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 5) as f32, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the bed the placement lands");
    let head_cell = (c, surface + 1, c + 1);
    let foot_cell = (c, surface + 1, c);
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        app.spawn_y as f64,
        c as f64 + 0.5,
        180.0,
        90.0,
    ));
    // THE SPAWN LANE'S OWN BANK (the spawn walk's own law — the pose
    // the body materialized at is the pose the respawn restores: the
    // manual seat the materialization wears, the spawn_pose the bank
    // keeps — the offline lane's spawn_offline_body the same triple
    // banks)
    app.spawn_pose = app.body.as_ref().map(|b| (b.x, b.y, b.z, b.yaw, b.pitch));
    app.clock_total = 18_000;
    assert!(app.on_right_press(0, 0), "the night press the sleep stamps");
    for _ in 0..100 {
        app.update();
    }
    assert!(app.sleeping_bed.is_none(), "the skip the wake serves");
    // THE BED'S OWN BREAK (the REAL dig — the foot the camera enters,
    // the mate the silent keep wears)
    app.on_press(0, 0);
    let mut ticks = 0;
    while app
        .offline_world
        .as_ref()
        .unwrap()
        .get(foot_cell.0, foot_cell.1, foot_cell.2)
        != crate::world::AIR
    {
        app.update();
        ticks += 1;
        assert!(ticks < 400, "the foot's dig completed in time");
    }
    // THE BUILDER'S OWN OCCUPY (the head cell the stone takes — the
    // generic arm's passable pair the refuse wears)
    app.offline_world.as_mut().unwrap().set(
        head_cell.0,
        head_cell.1,
        head_cell.2,
        crate::world::STONE,
    );
    // THE DEATH + THE RESPAWN (the EMPTY verdict the missingRespawnBlock
    // arm — the shared spawn the fallback, the not_valid the chat
    // speaks)
    app.chat_open = true;
    app.chat_input = "/kill".into();
    app.on_chat_submit();
    assert!(app.dead, "the kill the death arms");
    app.respawn();
    assert!(!app.dead, "the respawn the death clears");
    // THE SHARED SPAWN'S OWN RE-SEAT (the createDefault/missing
    // fallback the spawn pose the body wears)
    let (sx, sy, sz, _, _) = app.spawn_pose.expect("the offline spawn pose banks");
    let b = app.body.as_ref().unwrap();
    assert_eq!(
        (b.x, b.y, b.z),
        (sx, sy, sz),
        "the fallback the world spawn the body re-seats"
    );
    // THE OBSTRUCTION'S OWN MESSAGE (javap the missingRespawnBlock
    // flag — the spawn.not_valid row the chat speaks)
    assert!(
        app.chat_messages
            .iter()
            .any(|(m, _)| m.contains("no home bed")),
        "the built-over spot the not_valid the chat speaks"
    );
}

// THE ANCHOR'S OWN CHARGE-EXPLODE WALK (the autopush-314 round — javap
// 26.2 RespawnAnchorBlock the live disasm the walk banks — scripts/
// jar_work/anchor{,3}_314.txt the dumps serve): the REAL walk the
// held anchor the placement lane the kind stamps, the glowstone the
// charge arm the four presses the ladder climbs, THE CONSUME QUIRK
// the full anchor the fuel the press still eats, the empty hand the
// verdict walk the canSetSpawn-false arm the pending boom queues —
// the update the same-tick blast the cells clear the voice + the
// flash the boom serves.
#[test]
fn the_offline_anchor_the_charge_the_press_the_explode_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let anchor_item = mc_registry::item_id_by_name("respawn_anchor").unwrap();
    let glow_item = mc_registry::item_id_by_name("glowstone").unwrap();
    app.inventory.items[0] = (anchor_item, 2);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE PAD (the blast's own stage — the stone row the cells the
    // ray walk clears, the air rows the ring the scan reads)
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -8..=8 {
            for dz in -8..=8 {
                world.set(c + dx, surface, c + dz, crate::world::STONE);
                world.set(c + dx, surface + 1, c + dz, crate::world::AIR);
                world.set(c + dx, surface + 2, c + dz, crate::world::AIR);
            }
        }
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 4) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE PLACEMENT (the real right-press the held anchor the block
    // lane lands — the kind the NAME_OF row the palette answers)
    assert!(app.on_right_press(0, 0), "the anchor the placement lands");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface + 1, c),
        crate::world::RESPAWN_ANCHOR,
        "the placed cell the anchor kind wears"
    );
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .variant(c, surface + 1, c),
        0,
        "the fresh anchor the charge-0 state wears"
    );
    // THE EMPTY-HAND PRESS AT CHARGE 0 (useWithoutItem pcs 0-19 — the
    // PASS the item lanes follow: the press not consumed, the cell
    // the same charge wears)
    app.selected_hotbar = 2; // the empty slot
    app.inventory.items[2] = (0, 0);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(
        !app.on_right_press(0, 0),
        "the empty anchor the press the PASS keeps (the item lanes follow)"
    );
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .variant(c, surface + 1, c),
        0,
        "the PASS the charge never moves"
    );
    // THE CHARGE ARM (useItemOn pcs 0-33 — the held glowstone the
    // walk speaks: the setValue(+1) the ladder climbs, the voice the
    // charge lane banks, THE CONSUME the stack the press eats)
    app.selected_hotbar = 1;
    app.inventory.items[1] = (glow_item, 6);
    app.hotbar_slots = app.inventory.hotbar();
    for expected in 1..=4 {
        assert!(
            app.on_right_press(0, 0),
            "the charge press #{expected} the SUCCESS consumes"
        );
        assert_eq!(
            app.offline_world
                .as_ref()
                .unwrap()
                .variant(c, surface + 1, c),
            expected,
            "the charge ladder the {expected} stamp wears"
        );
        assert_eq!(
            app.inventory.items[1].1,
            6 - i32::from(expected),
            "the fuel stack the consume the count keeps"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.contains("block.respawn_anchor.charge")),
        "the charge voice the ladder's presses bank"
    );
    // THE FULL ANCHOR'S OWN PRESS (the canBeCharged gate the charge<4
    // verdict refuses — THE CONSUME QUIRK the stack the press STILL
    // eats [pc 23-27 the un-gated consume], the SUCCESS the press
    // consumes)
    assert!(
        app.on_right_press(0, 0),
        "the full anchor the press the SUCCESS keeps"
    );
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .variant(c, surface + 1, c),
        4,
        "the full anchor the charge the gate refuses"
    );
    assert_eq!(
        app.inventory.items[1].1, 1,
        "the quirk the fuel the press still eats"
    );
    // THE OVERWORLD PRESS (useWithoutItem the canSetSpawn-false arm —
    // the explode walk the removeBlock the same-tick removal keeps,
    // the dampening scan the dry pad the flag refuses, the pending
    // boom the update the blast serves)
    app.selected_hotbar = 2; // the empty hand
    app.hotbar_slots = app.inventory.hotbar();
    assert!(
        app.on_right_press(0, 0),
        "the overworld press the explode arm consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface + 1, c),
        crate::world::AIR,
        "the removeBlock the cell the same-tick AIR wears"
    );
    assert_eq!(
        app.pending_anchor_booms.len(),
        1,
        "the pending boom the queue keeps"
    );
    assert!(
        !app.pending_anchor_booms[0].4,
        "the dry pad the dampening flag refuses"
    );
    // THE BLAST'S OWN TICK (the queue the same-tick blast serves —
    // the ray scan the cells clear, the voice the explode lane
    // banks, the flash the particle seat wears)
    app.update();
    assert!(
        app.pending_anchor_booms.is_empty(),
        "the tick the queue consumes"
    );
    assert!(
        app.offline_world.as_ref().unwrap().get(c, surface, c) != crate::world::STONE,
        "the blast the pad cell the ray walk clears"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.contains("entity.generic.explode")),
        "the boom the explode voice banks"
    );
    assert!(
        app.particles.iter().any(|p| matches!(
            p.kind,
            crate::particles::ParticleKind::ExplosionSeed
                | crate::particles::ParticleKind::Explosion
        )),
        "the flash the particle seat wears"
    );
}

// THE ANCHOR'S OWN NETHER WALK (the autopush-314 round — javap 26.2
// useWithoutItem pcs 40-159 the spawn-set arm + ServerPlayer
// .findRespawnAndUseSpawnBlock pcs 44-161 the static's anchor arm the
// respawn re-seats — scripts/jar_work/anchor{,4}_314.txt the dumps
// serve): the REAL walk the nether dimension the canSetSpawn gate the
// works verdict reads, the charged anchor the press the RespawnData
// (pos, 0.0f, 0.0f) the fconst pair banks [the SPAWN_SET_MESSAGE the
// changed-config chat speaks], the far walk the /kill the respawn —
// the 25-row scan the north neighbour the body re-seats, the charge-1
// setBlock the consume the variant byte answers.
#[test]
fn the_offline_anchor_the_nether_the_spawn_set_the_respawn_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let anchor_item = mc_registry::item_id_by_name("respawn_anchor").unwrap();
    let glow_item = mc_registry::item_id_by_name("glowstone").unwrap();
    app.inventory.items[0] = (anchor_item, 1);
    app.inventory.items[1] = (glow_item, 2);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -4..=4 {
            for dz in -4..=4 {
                world.set(c + dx, surface, c + dz, crate::world::STONE);
                world.set(c + dx, surface + 1, c + dz, crate::world::AIR);
                world.set(c + dx, surface + 2, c + dz, crate::world::AIR);
            }
        }
    }
    // THE NETHER'S OWN GATE (the canSetSpawn gate the dimension
    // attribute the works verdict reads — the nether the TRUE row the
    // offline law keeps)
    app.world_dimension = Dimension::Nether;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 4) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE PLACEMENT (the real right-press the kind lands)
    assert!(app.on_right_press(0, 0), "the anchor the placement lands");
    // THE CHARGE (the real glowstone press the ladder's first rung
    // wears)
    app.selected_hotbar = 1;
    app.hotbar_slots = app.inventory.hotbar();
    assert!(
        app.on_right_press(0, 0),
        "the charge press the SUCCESS consumes"
    );
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .variant(c, surface + 1, c),
        1,
        "the charge the first rung wears"
    );
    // THE SPAWN SET (the empty hand the verdict walk the works arm
    // serves — the RespawnData the fconst pair the zero angles banks,
    // the changed-config chat the SPAWN_SET_MESSAGE speaks)
    app.selected_hotbar = 2;
    app.inventory.items[2] = (0, 0);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(app.on_right_press(0, 0), "the spawn-set press consumes");
    assert_eq!(
        app.respawn_point,
        Some((f64::from(c), f64::from(surface + 1), f64::from(c), 0.0, 0.0)),
        "the RespawnData (pos, 0.0f, 0.0f) the fconst pair banks"
    );
    assert!(
        app.chat_messages
            .iter()
            .any(|(m, _)| m.contains("Respawn point set")),
        "the changed-config chat the SPAWN_SET_MESSAGE speaks"
    );
    assert!(
        app.pending_anchor_booms.is_empty(),
        "the works arm the explode never queues"
    );
    // THE WALK'S OWN DEPARTURE + THE DEATH (the far spot the body the
    // honest move wears, the /kill the death arms)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 6.5,
        app.spawn_y as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    app.chat_open = true;
    app.chat_input = "/kill".into();
    app.on_chat_submit();
    assert!(app.dead, "the kill the death arms");
    // THE RESPAWN (the static's anchor arm — the 25-row scan the
    // north neighbour the body re-seats, the charge-1 setBlock the
    // consume the variant byte answers)
    app.respawn();
    assert!(!app.dead, "the respawn the death clears");
    let b = app.body.as_ref().unwrap();
    assert_eq!(
        (b.x, b.y, b.z),
        (c as f64 + 0.5, (surface + 1) as f64, (c - 1) as f64 + 0.5),
        "the scan's row0 spot the body re-seats (the north neighbour the ground floor wears)"
    );
    assert_eq!(
        (b.yaw, b.pitch),
        (0.0, 0.0),
        "the RespawnPosAngle's own pair the banked zero angles wear"
    );
    assert_eq!(
        app.offline_world
            .as_ref()
            .unwrap()
            .variant(c, surface + 1, c),
        0,
        "the consume the charge-1 setBlock the variant byte answers"
    );
}

// THE ANCHOR'S OWN MESH-LADDER WALK (the autopush-315 round — the
// blockstate charges=0..4 the five cube_bottom_top models the jar's
// own JSON speaks, scripts/jar_work/anchor_light_315.txt the dumps
// serve): the REAL walk the held anchor the placement lane the kind
// stamps, the mesher the charge-0 look the side0 + top_off masters
// bake, the glowstone the charge arm the four presses the ladder
// climbs — the side1..4 masters the re-mesh bakes, the glow top the
// charged rows wear, the relight the glow ladder the cell's own
// flood paints (0/3/7/11/15 the lambda$static$332 law the scaled
// charge answers).
#[test]
fn the_offline_anchor_the_mesh_the_charge_ladder_the_glow_wears() {
    use mc_registry::generated::atlas;
    let mut app = App::new(3);
    app.enter_world();
    let anchor_item = mc_registry::item_id_by_name("respawn_anchor").unwrap();
    let glow_item = mc_registry::item_id_by_name("glowstone").unwrap();
    app.inventory.items[0] = (anchor_item, 2);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE FLAT PAD (the look's own stage — the stone row the cell the
    // camera seats, the air rows the scan reads)
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -3..=3 {
            for dz in -3..=3 {
                world.set(c + dx, surface, c + dz, crate::world::STONE);
                for dy in 1..=3 {
                    world.set(c + dx, surface + dy, c + dz, crate::world::AIR);
                }
            }
        }
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (surface + 4) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE PLACEMENT (the real right-press the held anchor the block
    // lane lands — the kind the NAME_OF row the palette answers)
    assert!(app.on_right_press(0, 0), "the anchor the placement lands");
    let ay = surface + 1;
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, ay, c),
        crate::world::RESPAWN_ANCHOR,
        "the placed cell the anchor kind wears"
    );
    // THE BAND'S OWN QUADS (the cell the mesher bakes — the tile the
    // quad's own seat answers: the mesh_chunk walk the part_8 walk's
    // own controlled world wears)
    let band_tiles = |app: &App, ay: i32| -> Vec<u16> {
        let world = app.offline_world.as_ref().unwrap();
        let quads = crate::mesher::mesh_chunk(
            world,
            (c / crate::world::CHUNK_X as i32) as usize,
            (c / crate::world::CHUNK_Z as i32) as usize,
            None,
        );
        quads
            .iter()
            .filter(|q| {
                q.pts.iter().all(|p| {
                    p[0] >= c as f32 - 1e-4
                        && p[0] <= c as f32 + 1.0 + 1e-4
                        && p[1] >= ay as f32 - 1e-4
                        && p[1] <= ay as f32 + 1.0 + 1e-4
                        && p[2] >= c as f32 - 1e-4
                        && p[2] <= c as f32 + 1.0 + 1e-4
                })
            })
            .map(|q| q.tile)
            .collect()
    };
    // THE TOP SEAT'S OWN TILE (the UP face the y = ay+1 plane's four
    // corners wear — the charged glow / the dark off the row picks)
    let top_tile = |app: &App, ay: i32| -> u16 {
        let world = app.offline_world.as_ref().unwrap();
        let quads = crate::mesher::mesh_chunk(
            world,
            (c / crate::world::CHUNK_X as i32) as usize,
            (c / crate::world::CHUNK_Z as i32) as usize,
            None,
        );
        quads
            .iter()
            .find(|q| {
                q.pts.iter().all(|p| {
                    (p[1] - (ay as f32 + 1.0)).abs() < 1e-4
                        && p[0] >= c as f32 - 1e-4
                        && p[0] <= c as f32 + 1.0 + 1e-4
                        && p[2] >= c as f32 - 1e-4
                        && p[2] <= c as f32 + 1.0 + 1e-4
                })
            })
            .map(|q| q.tile)
            .unwrap_or(0)
    };
    // THE CELL'S OWN BLOCK LIGHT (the grid the relight paints — the
    // emission the cell's own seat wears)
    let block_light = |app: &App, ay: i32| -> Option<u8> {
        let world = app.offline_world.as_ref().unwrap();
        world
            .light
            .as_ref()
            .and_then(|g| g.cell(world.chunks_per_side, c, ay, c).1)
    };
    // THE CHARGE-0 LOOK (the charges=0 model the mesher bakes — the
    // four sides the side0 master, the top the dark top_off wears,
    // the charged masters the ladder never bakes)
    {
        let tiles = band_tiles(&app, ay);
        assert_eq!(
            tiles
                .iter()
                .filter(|t| **t == atlas::TILE_RESPAWN_ANCHOR_SIDE0)
                .count(),
            4,
            "the charge-0 sides the side0 master wears"
        );
        assert_eq!(
            top_tile(&app, ay),
            atlas::TILE_RESPAWN_ANCHOR_TOP_OFF,
            "the dark top the uncharged row wears"
        );
        for t in [
            atlas::TILE_RESPAWN_ANCHOR_SIDE1,
            atlas::TILE_RESPAWN_ANCHOR_SIDE2,
            atlas::TILE_RESPAWN_ANCHOR_SIDE3,
            atlas::TILE_RESPAWN_ANCHOR_SIDE4,
            atlas::TILE_RESPAWN_ANCHOR_TOP,
        ] {
            assert!(
                !tiles.contains(&t),
                "the uncharged look the charged masters never bake (tile {t})"
            );
        }
        assert_eq!(
            block_light(&app, ay),
            Some(0),
            "the dark base the uncharged cell wears"
        );
    }
    // THE CHARGE ARM (useItemOn pcs 0-33 — the held glowstone the
    // walk speaks: the setValue(+1) the ladder climbs, the re-mesh
    // the side{charge} masters bake, the glow top the charged rows
    // wear, the relight the scaled charge the flood paints)
    app.selected_hotbar = 1;
    app.inventory.items[1] = (glow_item, 6);
    app.hotbar_slots = app.inventory.hotbar();
    for expected in 1..=4u8 {
        assert!(
            app.on_right_press(0, 0),
            "the charge press #{expected} the SUCCESS consumes"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().variant(c, ay, c),
            expected,
            "the charge ladder the {expected} stamp wears"
        );
        // THE MESH LADDER (the re-mesh the press's own walk answers —
        // the side{expected} master the four sides wear, the PRIOR
        // rung the climb retires, the glow top the charged rows wear)
        let tiles = band_tiles(&app, ay);
        let rung = |n: u8| -> u16 {
            match n {
                1 => atlas::TILE_RESPAWN_ANCHOR_SIDE1,
                2 => atlas::TILE_RESPAWN_ANCHOR_SIDE2,
                3 => atlas::TILE_RESPAWN_ANCHOR_SIDE3,
                4 => atlas::TILE_RESPAWN_ANCHOR_SIDE4,
                _ => atlas::TILE_RESPAWN_ANCHOR_SIDE0,
            }
        };
        assert_eq!(
            tiles.iter().filter(|t| **t == rung(expected)).count(),
            4,
            "the charge-{expected} sides the side{expected} master wears"
        );
        for n in 0..=4u8 {
            if n != expected {
                assert!(
                    !tiles.contains(&rung(n)),
                    "the charge-{expected} look the rung-{n} master never bakes"
                );
            }
        }
        assert_eq!(
            top_tile(&app, ay),
            atlas::TILE_RESPAWN_ANCHOR_TOP,
            "the glow top the charged {expected} wears"
        );
        // THE GLOW LADDER (lambda$static$332 the getScaledChargeLevel
        // (state, 15) call banks — the cell's own block light the
        // scaled charge answers: 3/7/11/15 the presses climb)
        assert_eq!(
            block_light(&app, ay),
            Some(u8::try_from(crate::world::anchor_scaled_charge(expected, 15)).unwrap()),
            "the charge-{expected} glow the scaled charge wears"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.contains("block.respawn_anchor.charge")),
        "the charge voice the ladder's presses bank"
    );
}

/// THE BEACON'S OWN BEAM (the autopush-320 round — the
/// BeaconRenderer's submit the sections law the render pass wears:
/// the pyramid's levels the visibility gate answers, the full-column
/// ladder the steady state the budgeted walk banks, the per-section
/// segments the y cursor the 2048 tail wears, the store the bank
/// lane the containers file wears, the stale cell the live world's
/// own gate refuses).
#[test]
fn the_offline_beacon_the_beam_the_pyramid_the_sections_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let py = app.spawn_y + 10;
    let (bx, bz) = (24i32, 24i32);
    let by = py + 2;
    // THE AIR PLATFORM (the stone plate the sites the test builds —
    // the terrain the interference never wears)
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in -8..=8 {
            for dz in -8..=8 {
                world.set(bx + dx, py - 1, bz + dz, crate::world::STONE);
            }
        }
        // THE PYRAMID'S OWN TIERS (the tag's five base kinds the
        // seats wear — the tier-1 the (2·1+1)^2 square at by-1, the
        // tier-2 the (2·2+1)^2 square at by-2, the iron the ladder's
        // own first rung the 319 seat banks)
        for dx in -1..=1 {
            for dz in -1..=1 {
                world.set(bx + dx, by - 1, bz + dz, crate::world::IRON_BLOCK);
            }
        }
        for dx in -2..=2 {
            for dz in -2..=2 {
                world.set(bx + dx, by - 2, bz + dz, crate::world::IRON_BLOCK);
            }
        }
        // THE BEACON'S OWN SEAT (the 804 tail seat the kind lane
        // wears)
        world.set(bx, by, bz, crate::world::BEACON);
    }
    // THE STORE'S OWN BANK (the containers lane the cells the disk
    // banks — the boot parse the same set the load wears)
    app.beacon_stores.insert((bx, by, bz));

    // THE LEVELS' OWN LADDER (javap updateBase the tier scan the
    // 318 law wears — the complete pair the levels-2 answers)
    let world = app.offline_world.as_ref().unwrap();
    let levels =
        crate::world::beacon_levels(world, bx, by, bz, |k| crate::world::is_beacon_base(k));
    assert_eq!(levels, 2, "the complete pair the levels-2 wears");

    // THE LADDER'S OWN WALK (the full column the render's steady
    // state wears — the beacon's own cell the WHITE root seeds, the
    // air the transparent-grow arm the column grows)
    let max_y = crate::world::CHUNK_Y as i32 - 1;
    let sections = crate::world::beacon_beam_sections_budgeted(
        world,
        bx,
        by,
        bz,
        max_y,
        |k| crate::world::is_beacon(k),
        (max_y - by + 1).max(1) as usize,
    );
    assert_eq!(sections.len(), 1, "the WHITE root the single section wears");
    assert_eq!(sections[0].color, 0xF9_FFFE, "the WHITE the root wears");
    assert_eq!(
        sections[0].height,
        max_y - by + 1,
        "the air the root the column grows"
    );

    // THE VISIBILITY GATE (javap getBeamSections the levels==0 gate
    // the empty answer keeps — the leveled beacon the sections serve)
    let shown = crate::world::beacon_get_beam_sections(levels, &sections);
    assert_eq!(shown.len(), 1, "the leveled beacon the sections serve");

    // THE SEGMENTS' OWN WALK (the single section the 2048 tail the
    // render height wears — the y cursor the BE-local 0 the beacon's
    // own y wears, the two prisms the 8 quads the pair carries)
    let segs = crate::world::beacon_beam_segments((bx, by, bz), &shown, 1.0, 20.0);
    assert_eq!(segs.len(), 8, "the two prisms the 8 quads wear");
    for (color, q) in &segs {
        assert_eq!(*color, 0xF9_FFFE, "the WHITE the pair carries");
        let ys: Vec<f32> = q.pts.iter().map(|p| p[1]).collect();
        assert!(
            ys.iter().all(|&y| (y - f64::from(by) as f32).abs() < 1e-4
                || (y - (f64::from(by) as f32 + 2048.0)).abs() < 1e-4),
            "the segment the by..by+2048 span wears ({ys:?})"
        );
    }

    // THE RED FILTER'S OWN LADDER (the stained glass the fresh
    // section wears — the two segments the y cursor the real heights
    // the accumulate keeps, the 2048 tail the LAST section wears)
    {
        let mut world = app.offline_world.as_mut().unwrap();
        world.set(bx, by + 1, bz, crate::world::RED_STAINED_GLASS);
    }
    let world = app.offline_world.as_ref().unwrap();
    let sections = crate::world::beacon_beam_sections_budgeted(
        world,
        bx,
        by,
        bz,
        max_y,
        |k| crate::world::is_beacon(k),
        (max_y - by + 1).max(1) as usize,
    );
    assert_eq!(sections.len(), 2, "the red the fresh section mints");
    assert_eq!(sections[0].color, 0xF9_FFFE, "the WHITE the root keeps");
    assert_eq!(sections[0].height, 1, "the root the height-1 seed keeps");
    assert_eq!(
        sections[1].color, 0xB0_2E_26,
        "the red the fresh section wears"
    );
    assert_eq!(
        sections[1].height,
        max_y - by,
        "the red the column grows — the air the transparent arm serves"
    );
    let segs = crate::world::beacon_beam_segments((bx, by, bz), &sections, 1.0, 20.0);
    assert_eq!(segs.len(), 16, "the two sections the 16 quads wear");
    for (color, q) in segs.iter().take(8) {
        assert_eq!(*color, 0xF9_FFFE, "the first segment the WHITE wears");
        let ys: Vec<f32> = q.pts.iter().map(|p| p[1]).collect();
        assert!(
            ys.iter().all(|&y| (y - f64::from(by) as f32).abs() < 1e-4
                || (y - (f64::from(by) as f32 + 1.0)).abs() < 1e-4),
            "the first segment the by..by+1 span wears ({ys:?})"
        );
    }
    for (color, q) in segs.iter().skip(8) {
        assert_eq!(*color, 0xB0_2E_26, "the second segment the red wears");
        let ys: Vec<f32> = q.pts.iter().map(|p| p[1]).collect();
        assert!(
            ys.iter()
                .all(|&y| (y - (f64::from(by) as f32 + 1.0)).abs() < 1e-4
                    || (y - (f64::from(by) as f32 + 1.0 + 2048.0)).abs() < 1e-4),
            "the second segment the 2048 tail wears ({ys:?})"
        );
    }

    // THE RADIUS'S OWN SCOPE (the camera near the beacon the
    // horizontal the 96 divisor the 1.0 floor keeps)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(bx as f32 + 0.5, (by + 6) as f32, bz as f32 + 30.5),
        yaw: std::f32::consts::PI,
        pitch: 0.0,
    });
    let cam = app.world_camera();
    let dx = f64::from(cam.pos.x) - (f64::from(bx) + 0.5);
    let dz = f64::from(cam.pos.z) - (f64::from(bz) + 0.5);
    let horizontal = ((dx * dx + dz * dz).sqrt()) as f32;
    assert!(
        (horizontal - 30.0).abs() < 0.5,
        "the staged seat the horizontal wears ({horizontal})"
    );
    assert_eq!(
        crate::world::beacon_beam_radius_scale(horizontal, false),
        1.0,
        "the near camera the 1.0 floor keeps"
    );

    // THE STALE CELL'S OWN GATE (the dug-out beacon the live world
    // refuses — the seat gate the pass wears, the walk the air cell
    // the clear+stop arm serves)
    {
        let mut world = app.offline_world.as_mut().unwrap();
        world.set(bx, by, bz, crate::world::AIR);
    }
    let world = app.offline_world.as_ref().unwrap();
    assert_ne!(
        world.get(bx, by, bz),
        crate::world::BEACON,
        "the dug-out beacon the seat gate refuses"
    );
    let sections = crate::world::beacon_beam_sections_budgeted(
        world,
        bx,
        by,
        bz,
        max_y,
        |k| crate::world::is_beacon(k),
        (max_y - by + 1).max(1) as usize,
    );
    assert!(
        sections.is_empty(),
        "the air cell the walk the clear+stop arm serves"
    );

    // THE BANK'S OWN ROUND TRIP (the containers lane the cells the
    // disk banks — the parse the same set the boot wears, the
    // missing list the honest empties keep; the powers the pairs
    // the 322 rows wear)
    let banked = crate::screens::beacon_bank_to_nbt(&app.beacon_stores, &app.beacon_powers);
    let root = mc_protocol::nbt::Nbt::Compound(vec![("beacons".to_string(), banked)]);
    let (parsed, parsed_powers) = crate::screens::beacon_cells_from_nbt(&root);
    assert_eq!(
        parsed, app.beacon_stores,
        "the round trip the same set wears"
    );
    assert_eq!(
        parsed_powers, app.beacon_powers,
        "the powers the trip keeps"
    );
    let (empty, empty_powers) =
        crate::screens::beacon_cells_from_nbt(&mc_protocol::nbt::Nbt::Compound(vec![]));
    assert!(empty.is_empty(), "the missing list the honest empties keep");
    assert!(empty_powers.is_empty(), "the missing list the powers keep");
}

/// THE BEACON'S OWN PLACEMENT WALK (the autopush-321 round — the REAL
/// walk the hands serve: the held beacon's right-press the store
/// feeds [the placement seat the INSERT speaks], the creative press
/// the dug cell retires [the instabreak arm the REMOVE wears], the
/// blast the pre-clear scan the cratered beacon serves [the same
/// REMOVE the tick lane answers]). The drive the part_10 placement
/// wear keeps: the straight-down look the surface click speaks, the
/// bare world the body-less obstruction law keeps honest.
#[test]
fn the_offline_beacon_the_placement_walk_the_store_feeds_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE HELD BEACON'S OWN SEAT (the offline stand-in the held lane
    // wears — the item/beacon the placeable walk answers, the 804
    // kind the NAME_OF row banks)
    app.offline_held_updated(Some("item/beacon"));
    // THE AIM'S OWN DOWN-LOOK (the floor click the straight-down ray
    // speaks — the surface cell the placement target wears)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, (top + 4) as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE PLACEMENT'S OWN SEAT (the right-press the block lane
    // speaks — the placed beacon the store feeds)
    assert!(app.on_right_press(0, 0), "the beacon's placement consumed");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::BEACON,
        "the placed cell the beacon wears"
    );
    assert!(
        app.beacon_stores.contains(&(c, top, c)),
        "the placement seat the store feeds"
    );
    // THE CREATIVE PRESS'S OWN RETIREMENT (the instabreak arm the
    // same store walk wears — the dug cell the bank retires)
    app.game_mode = GameMode::Creative;
    app.on_attack();
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::AIR,
        "the creative press the cell breaks"
    );
    assert!(
        !app.beacon_stores.contains(&(c, top, c)),
        "the instabreak arm the store retires"
    );
    // THE RE-SEAT'S OWN FEED (the second placement the same seat
    // wears — the store the fresh cell banks again)
    assert!(app.on_right_press(0, 0), "the re-seat consumed");
    assert!(
        app.beacon_stores.contains(&(c, top, c)),
        "the re-seat the store feeds again"
    );
    // THE BLAST'S OWN RETIREMENT (the pre-clear scan the cratered
    // beacon serves — the primed shot the same tick the walk
    // answers, the crater the beacon cell clears)
    let mut t = crate::mob::PrimedTnt::new(9, c as f64 + 0.5, top as f64 + 2.0, c as f64 + 0.5, 7);
    t.fuse = 2;
    app.primed_tnts.push(t);
    for _ in 0..12 {
        if app.offline_world.as_ref().unwrap().get(c, top, c) == crate::world::AIR
            && !app.beacon_stores.contains(&(c, top, c))
        {
            break;
        }
        app.update();
    }
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::AIR,
        "the blast the beacon cell clears"
    );
    assert!(
        !app.beacon_stores.contains(&(c, top, c)),
        "the blast arm the store retires"
    );
}

/// THE BEACON'S OWN GUI DOOR (the autopush-322 round — the REAL walk
/// the hands serve: the right-press on the placed beacon the screen
/// opens [the banked powers' sync the open walk wears], the power
/// seats the press laws answer [the isSelected noop the re-press
/// keeps], the payment slot the mayPlace + max-1 laws keep [the
/// click walk the mirror write-through wears], the confirm the
/// updateEffects walk serves [the invalid pair the validator
/// refuses, the valid pair the bank feeds + the ONE payment the
/// remove(1) consumes + the close the tail speaks], the cancel the
/// payment drop wears, the 80-tick beat the effects re-apply
/// serves, the reopen the banked sync speaks). The drive the
/// part_9 slot lanes keep: the rect geometry the render's own
/// seats bank, the press dispatch the real walks serve.
#[test]
fn the_offline_beacon_the_gui_door_the_powers_the_payment_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE TIER-1 PYRAMID'S OWN BASE (the iron ring the levels-1
    // verdict answers — the 3x3 the tier-1 scan wears)
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in (c - 1)..=(c + 1) {
            for dz in (c - 1)..=(c + 1) {
                world.set(dx, top - 1, dz, crate::world::IRON_BLOCK);
            }
        }
        world.set(c, top, c, crate::world::BEACON);
    }
    app.beacon_stores.insert((c, top, c));
    // THE COIN'S OWN SEAT (the diamond pair the main row carries —
    // the mirror the open walk projects)
    let diamond = mc_registry::item_id_by_name("diamond").expect("the diamond id");
    app.inventory.items[9] = (diamond, 2);
    app.inventory.items[10] = (diamond, 1);
    app.inventory.items[11] = (1, 4); // the stone row the refusal wears
    app.hotbar_slots = app.inventory.hotbar();
    // THE BODY'S OWN SEAT (the feet pos the range walk reads — the
    // two blocks east the levels-1 range-20 answers; the BODY the
    // camera's highest authority wears, the aim the body's own
    // angles speak)
    {
        // THE AIM'S OWN PAIR (the eye at +1.62 the forward law reads:
        // `forward = (sin(yaw)cp, sinP, -cos(yaw)cp)` — the beacon
        // center the ray the top face enters)
        let ex = c as f64 + 2.5;
        let ey = top as f64 + 1.0 + 1.62;
        let ez = c as f64 + 2.5;
        let dx = (c as f64 + 0.5) - ex;
        let dy = (top as f64 + 0.5) - ey;
        let dz = (c as f64 + 0.5) - ez;
        let len = (dx * dx + dy * dy + dz * dz).sqrt();
        // THE BODY'S OWN ANGLE PAIR (the camera's own walk the body
        // wears — `yaw.to_radians() + PI` + the `-pitch.to_radians()`
        // pair the body.camera speaks, the DEGREES the fields keep,
        // the DOWN-LOOK the positive pitch the vanilla convention
        // banks)
        let yaw_deg = dx.atan2(-dz).to_degrees() - 180.0;
        let pitch_deg = -(dy / len).asin().to_degrees();
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 2.5,
            top as f64 + 1.0,
            c as f64 + 2.5,
            yaw_deg as f32,
            pitch_deg as f32,
        ));
    }
    // THE DOOR'S OWN OPEN (the body-aimed look the hit speaks — the
    // beacon cell the block lane answers)
    assert!(app.on_right_press(0, 0), "the beacon's press consumed");
    assert_eq!(app.screen, ScreenKind::Beacon, "the door the screen opens");
    assert_eq!(app.beacon_open_cell, Some((c, top, c)));
    assert_eq!(app.beacon_primary, None, "the fresh open the zeros keep");
    assert_eq!(
        app.beacon_mirror[0],
        (diamond, 2),
        "the mirror the inventory projects"
    );
    // THE GEOMETRY THE RENDER BANKS (the honest stand-in the panel
    // centers — the payment slot + the mirror row the two seats
    // keep)
    let panel = ((640 - 230) / 2, (360 - 219) / 2);
    app.beacon_panel = panel;
    let pay = (panel.0 + 136 + 1 + 8, panel.1 + 110 + 1 + 8);
    let row0 = (panel.0 + 36 + 1 + 8, panel.1 + 137 + 1 + 8);
    app.beacon_slot_rects = vec![(pay.0 - 8, pay.1 - 8, 0), (row0.0 - 8, row0.1 - 8, 1)];
    let speed = crate::effects::MOB_EFFECT_SPEED;
    let regen = crate::effects::MOB_EFFECT_REGENERATION;
    // THE POWER SEAT'S OWN PRESS (the isSelected gate the re-press
    // noop keeps — the primary the effect banks)
    app.beacon_power_press(speed, true);
    assert_eq!(app.beacon_primary, Some(speed), "the press the field banks");
    app.beacon_power_press(speed, true);
    assert_eq!(
        app.beacon_primary,
        Some(speed),
        "the re-press the noop keeps"
    );
    // THE TIER-4 PICK'S OWN WALK (the secondary the field banks —
    // the cheap pair the validator the CONFIRM refuses)
    app.beacon_power_press(regen, false);
    assert_eq!(
        app.beacon_secondary,
        Some(regen),
        "the pick the field banks"
    );
    // THE PAYMENT'S OWN RIDE (the click walk the row serves: the
    // pick-up the carried speaks, the write-through the inventory
    // wears, the place the max-1 cap keeps)
    app.on_press(row0.0, row0.1);
    assert_eq!(
        app.beacon_carried,
        (diamond, 2),
        "the pick-up the carried speaks"
    );
    assert_eq!(app.beacon_mirror[0], (0, 0), "the row the pick empties");
    assert_eq!(
        app.inventory.items[9],
        (0, 0),
        "the write-through the inventory wears"
    );
    // THE DRAG'S OWN RIDE (the 323 round the carried press the arm
    // speaks, the pay seat the paint keeps, the release the max-1
    // distribution serves — the same cap verdict the click walk wore)
    app.on_press(pay.0, pay.1);
    app.on_mouse_move(pay.0 + 1, pay.1);
    app.on_left_release();
    assert_eq!(
        app.beacon_payment,
        (diamond, 1),
        "the max-1 cap the place wears"
    );
    assert_eq!(
        app.beacon_carried,
        (diamond, 1),
        "the remainder the cursor keeps"
    );
    // THE MAYPLACE'S OWN REFUSAL (the non-payment carried the slot
    // never takes — the stone row the SWAP walk carries: the cursor
    // diamond the stone's row takes, the stone the cursor wears)
    let stone_seat = (panel.0 + 36 + 2 * 18 + 1 + 8, panel.1 + 137 + 1 + 8);
    app.beacon_slot_rects
        .push((stone_seat.0 - 8, stone_seat.1 - 8, 3));
    // THE DRAG'S OWN SWAP (the carried press the arm wears, the
    // stone seat the paint refusal keeps, the release the deferred
    // PICKUP the swap walk serves)
    app.on_press(stone_seat.0, stone_seat.1);
    app.on_mouse_move(stone_seat.0 + 1, stone_seat.1);
    app.on_left_release();
    assert_eq!(app.beacon_carried, (1, 4), "the swap the stone carries");
    app.on_press(pay.0, pay.1);
    app.on_mouse_move(pay.0 + 1, pay.1);
    app.on_left_release();
    assert_eq!(
        app.beacon_payment,
        (diamond, 1),
        "the refusal the slot keeps"
    );
    assert_eq!(app.beacon_carried, (1, 4), "the refusal the carried keeps");
    // THE INVALID CONFIRM'S OWN VERDICT (the validator the levels-1
    // pyramid wears — the REGEN secondary the pair refuses, the
    // payment the ground drop wears, the close the tail speaks)
    app.beacon_confirm_press();
    assert_eq!(app.screen, ScreenKind::World, "the close the tail speaks");
    assert!(
        app.beacon_powers.get(&(c, top, c)).is_none(),
        "the refused pair the bank never banks"
    );
    assert_eq!(app.beacon_payment, (0, 0), "the close the payment strips");
    assert!(
        app.drops.iter().any(|d| d.item == diamond),
        "the removed walk the coin drops"
    );
    // THE RE-OPEN'S OWN SYNC (the door the same seat speaks — the
    // fresh open the zeros keep [the bank never banked])
    assert!(app.on_right_press(0, 0), "the re-open consumed");
    assert_eq!(
        app.screen,
        ScreenKind::Beacon,
        "the re-open the screen wears"
    );
    app.beacon_panel = panel;
    // THE COIN'S OWN SEAT (the close walk's first-empty law the row
    // the mirror scan finds — the carried home the seat the real
    // inventory order wears)
    let (coin_rect, coin_slot) = beacon_coin_seat(&app, panel, diamond);
    app.beacon_slot_rects = vec![
        (pay.0 - 8, pay.1 - 8, 0),
        (coin_rect.0, coin_rect.1, coin_slot),
    ];
    // THE VALID CONFIRM'S OWN WALK (the primary alone the levels-1
    // answers — the coin the slot rides, the confirm the bank feeds)
    app.on_press(coin_rect.0 + 8, coin_rect.1 + 8);
    assert_eq!(
        app.beacon_carried,
        (diamond, 1),
        "the fresh mirror the remainder carries"
    );
    app.on_press(pay.0, pay.1);
    // THE DRAG'S OWN VALID RIDE (the carried press the arm wears —
    // the pay seat the paint keeps, the release the max-1 the
    // distribution serves)
    app.on_mouse_move(pay.0 + 1, pay.1);
    app.on_left_release();
    assert_eq!(app.beacon_payment, (diamond, 1), "the coin the slot takes");
    app.beacon_power_press(speed, true);
    assert_eq!(
        app.beacon_secondary, None,
        "the fresh open the secondary clears"
    );
    app.beacon_confirm_press();
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the confirm the close speaks"
    );
    assert_eq!(
        app.beacon_powers.get(&(c, top, c)),
        Some(&(Some(speed), None)),
        "the bank the powers feed"
    );
    assert_eq!(
        app.beacon_payment,
        (0, 0),
        "the ONE payment the remove(1) consumes"
    );
    // THE REOPEN'S OWN BANKED SYNC (the BeaconScreen$1 dataChanged
    // walk — the menu's data slots the screen fields mirror)
    assert!(app.on_right_press(0, 0), "the reopen consumed");
    assert_eq!(app.screen, ScreenKind::Beacon);
    assert_eq!(
        app.beacon_primary,
        Some(speed),
        "the banked sync the reopen wears"
    );
    // THE CANCEL'S OWN DROP (the fresh coin the seat rides — the
    // close the removed walk the strip + the drop serves)
    app.beacon_panel = panel;
    let (coin_rect, coin_slot) = beacon_coin_seat(&app, panel, diamond);
    app.beacon_slot_rects = vec![
        (pay.0 - 8, pay.1 - 8, 0),
        (coin_rect.0, coin_rect.1, coin_slot),
    ];
    app.on_press(coin_rect.0 + 8, coin_rect.1 + 8);
    // THE DRAG'S OWN FRESH RIDE (the carried press the arm wears —
    // the pay seat the paint keeps, the release the max-1 the
    // distribution serves)
    app.on_press(pay.0, pay.1);
    app.on_mouse_move(pay.0 + 1, pay.1);
    app.on_left_release();
    assert_eq!(
        app.beacon_payment,
        (diamond, 1),
        "the fresh coin the slot takes"
    );
    let drops_before = app.drops.len();
    // THE SEAT RECTS THE RENDER BANKS (the honest stand-in the headless
    // walk keeps — the (164,107)/(190,107) pair the init walk speaks)
    app.beacon_confirm_rect = (panel.0 + 164, panel.1 + 107);
    app.beacon_cancel_rect = (panel.0 + 190, panel.1 + 107);
    app.on_press(panel.0 + 190 + 11, panel.1 + 107 + 11);
    assert_eq!(app.screen, ScreenKind::World, "the cancel the close speaks");
    assert_eq!(app.beacon_payment, (0, 0), "the cancel the slot strips");
    assert!(app.drops.len() > drops_before, "the cancel the coin drops");
    // THE 80-TICK BEAT'S OWN DOSE (the banked powers the effects
    // serve — the clock the gate the beat wears, the range the
    // near-seated body answers)
    app.close_beacon(ScreenKind::World);
    app.clock_total = 79;
    app.update();
    assert!(app.effects.has_effect(speed), "the beat the primary serves");
    // THE STALE ROW'S OWN RETIRE (the dug-out beacon the walk never
    // re-serves — the cell the kind gate retires)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top, c, crate::world::AIR);
    app.clock_total = 159;
    app.update();
    assert!(
        !app.beacon_powers.contains_key(&(c, top, c)),
        "the stale cell the walk retires"
    );
}

/// THE BEACON'S OWN LAWS THE PURE LAYER BANKS (the autopush-322
/// round — the tier ladder the required-levels walk answers, the
/// validator the five verdicts keeps, the duration/amplifier/range
/// the applyEffects formulas speak, the encode the data-slot
/// semantics wears, the beat the 80-tick gate answers).
#[test]
fn the_beacon_the_tier_laws_the_validator_the_formulas_wear() {
    let speed = crate::effects::MOB_EFFECT_SPEED;
    let haste = crate::effects::MOB_EFFECT_HASTE;
    let resistance = crate::effects::MOB_EFFECT_RESISTANCE;
    let jump = crate::effects::MOB_EFFECT_JUMP_BOOST;
    let strength = crate::effects::MOB_EFFECT_STRENGTH;
    let regen = crate::effects::MOB_EFFECT_REGENERATION;
    // THE TIERS' OWN LADDER (the clinit's four rows the walk banks)
    assert_eq!(
        crate::world::BEACON_EFFECT_TIERS,
        [
            [speed, haste].as_slice(),
            [resistance, jump].as_slice(),
            [strength].as_slice(),
            [regen].as_slice(),
        ]
    );
    // THE REQUIREMENTS' OWN WALK (the tier index + 1 the scan speaks)
    assert_eq!(crate::world::beacon_required_levels_for(None), 0);
    assert_eq!(crate::world::beacon_required_levels_for(Some(speed)), 1);
    assert_eq!(crate::world::beacon_required_levels_for(Some(haste)), 1);
    assert_eq!(
        crate::world::beacon_required_levels_for(Some(resistance)),
        2
    );
    assert_eq!(crate::world::beacon_required_levels_for(Some(jump)), 2);
    assert_eq!(crate::world::beacon_required_levels_for(Some(strength)), 3);
    assert_eq!(crate::world::beacon_required_levels_for(Some(regen)), 4);
    assert_eq!(
        crate::world::beacon_required_levels_for(Some(999)),
        i32::MAX,
        "the unlisted effect the MAX keeps"
    );
    // THE VALIDATOR'S OWN VERDICTS (the five branches the walk keeps)
    assert!(crate::world::beacon_validate_effects(Some(speed), None, 1));
    assert!(
        !crate::world::beacon_validate_effects(Some(speed), Some(regen), 1),
        "the cheap secondary the levels-4 gate refuses"
    );
    assert!(
        !crate::world::beacon_validate_effects(Some(strength), None, 2),
        "the tier-3 primary the levels-2 gate refuses"
    );
    assert!(
        !crate::world::beacon_validate_effects(Some(regen), None, 4),
        "the tier-4 effect the primary seat refuses"
    );
    assert!(
        !crate::world::beacon_validate_effects(Some(speed), Some(strength), 4),
        "the mismatched cheap pair the same-effect gate refuses"
    );
    assert!(
        crate::world::beacon_validate_effects(Some(speed), Some(speed), 4),
        "the same-effect upgrade the tier-4 answers"
    );
    assert!(
        crate::world::beacon_validate_effects(Some(speed), Some(regen), 4),
        "the regen secondary the levels-4 answers"
    );
    // THE FORMULAS' OWN WALK (the applyEffects pair the bytecode keeps)
    assert_eq!(crate::world::beacon_effect_duration(1), (9 + 2) * 20);
    assert_eq!(crate::world::beacon_effect_duration(4), (9 + 8) * 20);
    assert_eq!(
        crate::world::beacon_effect_amplifier(4, Some(speed), Some(speed)),
        1
    );
    assert_eq!(
        crate::world::beacon_effect_amplifier(4, Some(speed), Some(regen)),
        0
    );
    assert_eq!(
        crate::world::beacon_effect_amplifier(3, Some(speed), Some(speed)),
        0
    );
    assert_eq!(crate::world::beacon_effect_range(1), 20.0);
    assert_eq!(crate::world::beacon_effect_range(4), 50.0);
    // THE ENCODE'S OWN WALK (the data-slot semantics the +1 wears)
    assert_eq!(crate::world::beacon_encode_effect(None), 0);
    assert_eq!(crate::world::beacon_encode_effect(Some(speed)), speed + 1);
    // THE BEAT'S OWN GATE (the 80-tick remainder the walk keeps)
    assert!(crate::world::beacon_effects_due(0));
    assert!(crate::world::beacon_effects_due(80));
    assert!(!crate::world::beacon_effects_due(79));
    // THE ROWS' OWN WALK (the applyEffects instances the passes keep)
    assert_eq!(
        crate::world::beacon_apply_effect_instances(1, Some(speed), None),
        vec![(speed, (9 + 2) * 20, 0)]
    );
    assert_eq!(
        crate::world::beacon_apply_effect_instances(4, Some(speed), Some(regen)),
        vec![(speed, (9 + 8) * 20, 0), (regen, (9 + 8) * 20, 0),],
        "the tier-4 mismatched pair the secondary pass serves"
    );
    assert_eq!(
        crate::world::beacon_apply_effect_instances(4, Some(speed), Some(speed)),
        vec![(speed, (9 + 8) * 20, 1)],
        "the same-effect pick the level-II primary serves"
    );
    assert!(
        crate::world::beacon_apply_effect_instances(2, None, Some(speed)).is_empty(),
        "the null primary the empty walk keeps"
    );
    // THE AABB'S OWN TOUCH (the inflate + the expandTowards pair the
    // intersect answers)
    let beacon = (10, 20, 10);
    assert!(crate::world::beacon_aabb_touches(
        (12.5, 21.0, 12.5),
        beacon,
        20.0,
        48
    ));
    assert!(
        !crate::world::beacon_aabb_touches((50.5, 21.0, 12.5), beacon, 20.0, 48),
        "the far player the box refuses"
    );
    assert!(
        !crate::world::beacon_aabb_touches((12.5, 5.0, 12.5), beacon, 5.0, 48),
        "the below-range skirt the box refuses"
    );
    // THE PAYMENT'S OWN VERDICT (the tag the five members keep)
    for name in [
        "netherite_ingot",
        "emerald",
        "diamond",
        "gold_ingot",
        "iron_ingot",
    ] {
        let id = mc_registry::item_id_by_name(name).expect(name);
        assert!(
            crate::world::is_beacon_payment_item(id),
            "the {name} the tag answers"
        );
    }
    assert!(
        !crate::world::is_beacon_payment_item(1),
        "the stone row the tag refuses"
    );
    // THE DISK ROW'S OWN WALK (the name strings the save keeps, the
    // tolerant parse the round trip answers)
    let mut stores = std::collections::BTreeSet::new();
    stores.insert((3, 4, 5));
    let mut powers = std::collections::HashMap::new();
    powers.insert((3, 4, 5), (Some(speed), Some(regen)));
    let nbt = crate::screens::beacon_bank_to_nbt(&stores, &powers);
    let (cells, parsed) = crate::screens::beacon_cells_from_nbt(&mc_protocol::nbt::Nbt::Compound(
        vec![("beacons".to_string(), nbt)],
    ));
    assert!(cells.contains(&(3, 4, 5)));
    assert_eq!(parsed.get(&(3, 4, 5)), Some(&(Some(speed), Some(regen))));
    // the powers-less row the bare triple keeps (the 320 rows the
    // parse never breaks)
    let mut bare = std::collections::BTreeSet::new();
    bare.insert((6, 7, 8));
    let nbt = crate::screens::beacon_bank_to_nbt(&bare, &std::collections::HashMap::new());
    let (cells, parsed) = crate::screens::beacon_cells_from_nbt(&mc_protocol::nbt::Nbt::Compound(
        vec![("beacons".to_string(), nbt)],
    ));
    assert!(cells.contains(&(6, 7, 8)));
    assert!(parsed.is_empty(), "the bare row the powers keep");
}

/// THE COIN SEAT'S OWN SCAN (the tests' helper — the diamond row the
/// mirror keeps the real geometry the render's own law speaks: the
/// 18-pitch grid the +58 hotbar row wears, the +1 inset the Slot's
/// hit shape keeps).
fn beacon_coin_seat(app: &App, panel: (i32, i32), diamond: i32) -> ((i32, i32), i16) {
    let idx = app
        .beacon_mirror
        .iter()
        .position(|&p| p.0 == diamond && p.1 > 0)
        .expect("the coin row the mirror keeps");
    let row = (idx / 9) as i32;
    let col = (idx % 9) as i32;
    let sx = panel.0 + 36 + col * 18 + 1;
    let sy = panel.1 + 137 + row * 18 + if row == 3 { 4 } else { 0 } + 1;
    ((sx, sy), (idx + 1) as i16)
}

/// THE OFFLINE BEACON DRAG'S OWN PAINT AND SPLIT (the autopush-323
/// round — the quickcraft walk the offline window the family's own
/// gate shape wears): the carried press the drag arms (no place, no
/// packet), the motion the paint speaks (the press seat the first
/// motion paints, the payment coin the tag verdict admits), the
/// recalc's ghost the floor split keeps, the release the
/// distribution serves — the PAYMENT seat the ICONST_1 cap speaks
/// (the ONE coin the max place wears), the mirror rows the floor
/// tenth wears, the write-through the inventory answers, the HUD row
/// the sync follows, the remainder the cursor keeps.
#[test]
fn the_offline_beacon_quickcraft_drag_the_paint_the_payment_cap_wear() {
    let diamond = mc_registry::item_id_by_name("diamond").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE TIER-1 PYRAMID'S OWN BASE (the iron ring the realistic
    // open wears — the drag test the door alone reads)
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in (c - 1)..=(c + 1) {
            for dz in (c - 1)..=(c + 1) {
                world.set(dx, top - 1, dz, crate::world::IRON_BLOCK);
            }
        }
        world.set(c, top, c, crate::world::BEACON);
    }
    app.beacon_stores.insert((c, top, c));
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the split reads
    }
    app.hotbar_slots = app.inventory.hotbar();
    app.open_beacon_for((c, top, c));
    assert_eq!(app.screen, ScreenKind::Beacon);
    assert!(
        !app.world_is_network,
        "the offline door the drag lane wears"
    );
    // THE GEOMETRY THE RENDER BANKS (the 322 round's own stand-in —
    // the payment seat + the two mirror row seats the rects keep)
    let panel = ((640 - 230) / 2, (360 - 219) / 2);
    app.beacon_panel = panel;
    let pay = (panel.0 + 136 + 1 + 8, panel.1 + 110 + 1 + 8);
    let row0 = (panel.0 + 36 + 1 + 8, panel.1 + 137 + 1 + 8);
    let row1 = (row0.0 + 18, row0.1);
    app.beacon_slot_rects = vec![
        (pay.0 - 8, pay.1 - 8, 0),
        (row0.0 - 8, row0.1 - 8, 1),
        (row1.0 - 8, row1.1 - 8, 2),
    ];
    // THE ARM: the carried press the state sets — NO place (the
    // mirror rows stay EMPTY), no painted seat yet (the arm the
    // silent start keeps)
    app.beacon_carried = (diamond, 5);
    app.on_press(row0.0, row0.1);
    assert!(app.qc_armed, "the carried press the arm speaks");
    assert_eq!(app.qc_type, 0, "the left button's own type");
    assert_eq!(app.qc_button, 0, "the left button remembered");
    assert!(app.qc_slots.is_empty(), "the press paints nothing");
    assert_eq!(app.beacon_mirror[0], (0, 0), "the press NO place speaks");
    // THE PAINT: the press seat the first motion paints (the +1
    // jiggle the same rect the moved gate answers), the payment
    // coin the tag verdict admits — the diamond the PaymentSlot's
    // own mayPlace passes
    app.on_mouse_move(row0.0 + 1, row0.1);
    assert_eq!(app.qc_slots, vec![1], "the press seat the motion paints");
    assert_eq!(
        app.qc_remainder, 0,
        "the one-seat projection the count empties"
    );
    app.on_mouse_move(row1.0 + 1, row1.1);
    assert_eq!(
        app.qc_slots,
        vec![1, 2],
        "the second seat the motion paints"
    );
    assert_eq!(app.qc_remainder, 1, "the floor(5/2) the ghost keeps");
    app.on_mouse_move(pay.0, pay.1);
    assert_eq!(
        app.qc_slots,
        vec![1, 2, 0],
        "the payment coin the tag paints"
    );
    assert_eq!(app.qc_remainder, 2, "the floor(5/3) the ghost keeps");
    // THE RELEASE: the distribution — the PAYMENT seat the ICONST_1
    // cap speaks (the floor(5/3) the min(1) folds), the mirror rows
    // the floor tenth wears, the write-through the inventory
    // answers, the remainder the cursor keeps
    app.on_left_release();
    assert_eq!(
        app.beacon_payment,
        (diamond, 1),
        "the payment seat's own max-1 cap"
    );
    assert_eq!(app.beacon_mirror[0], (diamond, 1), "the first row's tenth");
    assert_eq!(app.beacon_mirror[1], (diamond, 1), "the second row's tenth");
    assert_eq!(
        app.inventory.items[9],
        (diamond, 1),
        "the write-through the inventory wears"
    );
    assert_eq!(
        app.inventory.items[10],
        (diamond, 1),
        "the second write-through speaks"
    );
    assert_eq!(
        app.beacon_carried,
        (diamond, 2),
        "the remainder rides the cursor"
    );
    assert!(!app.qc_armed, "the release the arm always clears");
    assert!(app.qc_slots.is_empty(), "the painted set the tail clears");
    assert_eq!(
        app.hotbar_slots,
        app.inventory.hotbar(),
        "the HUD row the sync follows"
    );
}

/// THE BEACON DRAG'S OWN GATES AND THE ONE-EACH (the shouldAdd gates
/// the paint answers — the payment-only drag the ICONST_1 ghost the
/// recalc names, the non-payment carried the pay seat the tag
/// refusal speaks, the between-slots press the arm skips, the right
/// drag the type-1 one-each serves, the outside press the arm wears
/// — the release the -999 drop the whole carried speaks).
#[test]
fn the_offline_beacon_quickcraft_the_gates_the_one_each_the_outside_wear() {
    let diamond = mc_registry::item_id_by_name("diamond").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in (c - 1)..=(c + 1) {
            for dz in (c - 1)..=(c + 1) {
                world.set(dx, top - 1, dz, crate::world::IRON_BLOCK);
            }
        }
        world.set(c, top, c, crate::world::BEACON);
    }
    app.beacon_stores.insert((c, top, c));
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.hotbar_slots = app.inventory.hotbar();
    app.open_beacon_for((c, top, c));
    let panel = ((640 - 230) / 2, (360 - 219) / 2);
    app.beacon_panel = panel;
    let pay = (panel.0 + 136 + 1 + 8, panel.1 + 110 + 1 + 8);
    let row0 = (panel.0 + 36 + 1 + 8, panel.1 + 137 + 1 + 8);
    let row1 = (row0.0 + 18, row0.1);
    app.beacon_slot_rects = vec![
        (pay.0 - 8, pay.1 - 8, 0),
        (row0.0 - 8, row0.1 - 8, 1),
        (row1.0 - 8, row1.1 - 8, 2),
    ];
    // THE PAYMENT-ONLY GHOST: the single painted seat the ICONST_1
    // cap the recalc names — the floor(5/1) the min(1) folds, the
    // release the ONE coin places, the four the cursor keeps
    app.beacon_carried = (diamond, 5);
    app.on_press(pay.0, pay.1);
    assert!(app.qc_armed, "the pay seat's press the arm speaks");
    app.on_mouse_move(pay.0 + 1, pay.1);
    assert_eq!(app.qc_slots, vec![0], "the payment seat alone paints");
    assert_eq!(app.qc_remainder, 4, "the ICONST_1 cap the ghost keeps");
    app.on_left_release();
    assert_eq!(
        app.beacon_payment,
        (diamond, 1),
        "the one coin the cap places"
    );
    assert_eq!(
        app.beacon_carried,
        (diamond, 4),
        "the four the cursor keeps"
    );
    assert_eq!(
        app.beacon_mirror[0],
        (0, 0),
        "the mirror rows the drag never wears"
    );
    // THE NON-PAYMENT'S OWN REFUSAL: the dirt carried the pay seat
    // never paints (the tag verdict the paint re-keys), the mirror
    // row the plain TRUE paints — the release the row fills
    app.beacon_payment = (0, 0);
    app.beacon_carried = (dirt, 3);
    app.on_press(pay.0, pay.1);
    assert!(app.qc_armed, "the carried press the arm speaks");
    app.on_mouse_move(pay.0 + 1, pay.1);
    assert!(
        app.qc_slots.is_empty(),
        "the non-payment the pay seat refuses"
    );
    app.on_mouse_move(row0.0, row0.1);
    assert_eq!(app.qc_slots, vec![1], "the plain row the dirt paints");
    app.on_left_release();
    assert_eq!(
        app.beacon_mirror[0],
        (dirt, 3),
        "the row the whole split wears"
    );
    assert_eq!(
        app.beacon_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    // THE BETWEEN-SLOTS PRESS: the k == -1 the arm skips (the press
    // between the seats the drag never starts)
    app.beacon_carried = (diamond, 5);
    app.on_press(panel.0 + 225, panel.1 + 60);
    assert!(!app.qc_armed, "the between press the arm skips");
    assert_eq!(
        app.beacon_carried,
        (diamond, 5),
        "the skipped press the carried keeps"
    );
    // THE RIGHT ONE-EACH: the type-1 drag the three seats paint —
    // the release the ONE per seat speaks (the pay seat the cap the
    // same ONE wears), the pair the cursor keeps
    app.beacon_mirror[0] = (0, 0);
    app.beacon_mirror[1] = (0, 0);
    app.inventory.items[9] = (0, 0);
    app.inventory.items[10] = (0, 0);
    app.hotbar_slots = app.inventory.hotbar();
    app.on_right_press(row0.0, row0.1);
    assert!(app.qc_armed, "the right press the arm speaks");
    assert_eq!(app.qc_type, 1, "the right button's own type");
    app.on_mouse_move(row0.0 + 1, row0.1);
    app.on_mouse_move(row1.0 + 1, row1.1);
    app.on_mouse_move(pay.0, pay.1);
    assert_eq!(
        app.qc_slots,
        vec![1, 2, 0],
        "the three seats the motion paints"
    );
    assert_eq!(app.qc_remainder, 2, "the one-each the ghost keeps");
    app.on_right_release();
    assert_eq!(
        app.beacon_payment,
        (diamond, 1),
        "the right drag's ONE the cap wears"
    );
    assert_eq!(
        app.beacon_mirror[0],
        (diamond, 1),
        "the one-each the row wears"
    );
    assert_eq!(
        app.beacon_mirror[1],
        (diamond, 1),
        "the second one-each speaks"
    );
    assert_eq!(
        app.beacon_carried,
        (diamond, 2),
        "the pair the cursor keeps"
    );
    // THE OUTSIDE DROP: the -999 press the arm wears (the outside
    // rect the strict compare keeps), the motion paints nothing —
    // the release the -999 PICKUP the whole carried drops (the BODY
    // the eye the throw's aimed lane reads — the headless menus the
    // silent guard keeps)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 2.5,
        top as f64 + 1.0,
        c as f64 + 2.5,
        0.0,
        0.0,
    ));
    app.on_press(panel.0 - 10, panel.1 - 10);
    assert!(app.qc_armed, "the outside press the arm wears");
    app.on_mouse_move(panel.0 - 12, panel.1 - 12);
    assert!(app.qc_slots.is_empty(), "the outside motion paints nothing");
    app.on_left_release();
    assert_eq!(
        app.beacon_carried,
        (0, 0),
        "the whole carried the drop wears"
    );
    assert!(
        app.drops.iter().any(|d| d.item == diamond && d.count == 2),
        "the drop the entity speaks (the pair the whole carried wears)"
    );
}

/// THE BEACON'S OWN HOVER-TOOLTIP SEAT (the 324 round — the
/// extractTooltip pass the beacon the AbstractContainerScreen's own
/// lane INHERITS, the probe scripts/tooltip_324.py the override
/// absence banks): the hovered seat's own stack the tooltip rows
/// speak — the payment coin the seat's own id the rows name, the
/// mirror row the store's stack answers, the borders the isHovering
/// walk's own -1/+1 growth keeps, the misses the None verdict
/// answers. The REAL walk the offline door wears.
#[test]
fn the_offline_beacon_tooltip_the_hover_the_name_rows_wear() {
    let gold = mc_registry::item_id_by_name("gold_ingot").unwrap();
    let diamond = mc_registry::item_id_by_name("diamond").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE TIER-1 PYRAMID'S OWN BASE (the iron ring the realistic
    // open wears — the tooltip test the door alone reads)
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in (c - 1)..=(c + 1) {
            for dz in (c - 1)..=(c + 1) {
                world.set(dx, top - 1, dz, crate::world::IRON_BLOCK);
            }
        }
        world.set(c, top, c, crate::world::BEACON);
    }
    app.beacon_stores.insert((c, top, c));
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the mirror reads
    }
    app.hotbar_slots = app.inventory.hotbar();
    app.open_beacon_for((c, top, c));
    assert_eq!(
        app.screen,
        ScreenKind::Beacon,
        "the offline door the walk wears"
    );
    // THE GEOMETRY THE RENDER BANKS (the 323 round's own stand-in —
    // the payment seat + the two mirror row seats the rects keep)
    let panel = ((640 - 230) / 2, (360 - 219) / 2);
    app.beacon_panel = panel;
    let pay = (panel.0 + 136 + 1 + 8, panel.1 + 110 + 1 + 8);
    let row0 = (panel.0 + 36 + 1 + 8, panel.1 + 137 + 1 + 8);
    let row1 = (row0.0 + 18, row0.1);
    app.beacon_slot_rects = vec![
        (pay.0 - 8, pay.1 - 8, 0),
        (row0.0 - 8, row0.1 - 8, 1),
        (row1.0 - 8, row1.1 - 8, 2),
    ];
    // THE PAYMENT SEAT'S OWN ROWS (the coin the seat holds — the
    // hover the center the hasItem pass answers, the shared lane
    // the display name speaks)
    app.beacon_payment = (gold, 1);
    let (id, rows) = app
        .beacon_item_tooltip_hover(pay.0, pay.1)
        .expect("the payment hover the rows speak");
    assert_eq!(id, gold, "the seat's own stack the id answers");
    assert_eq!(rows.len(), 1, "the plain flag the name row alone wears");
    let reg = mc_registry::item_name(gold).unwrap();
    assert_eq!(
        rows[0].0,
        item_display_name(reg),
        "the name the shared lane speaks"
    );
    // THE MIRROR ROW'S OWN ANSWER (the diamond the store holds —
    // the same gates the row serves)
    app.beacon_mirror[0] = (diamond, 7);
    let (id, rows) = app
        .beacon_item_tooltip_hover(row0.0, row0.1)
        .expect("the mirror hover the rows speak");
    assert_eq!(id, diamond, "the row's own stack the id answers");
    assert_eq!(rows.len(), 1, "the name row the plain flag keeps");
    // THE ISHOVERING BORDERS (the -1/+1 growth the jar walk pins —
    // the 18x18 outer cell the edge pixels answer, the +18 miss the
    // gap wears)
    assert!(
        app.beacon_item_tooltip_hover(pay.0 - 8 - 1, pay.1)
            .is_some(),
        "the left border the -1 admits"
    );
    assert!(
        app.beacon_item_tooltip_hover(pay.0 - 8 + 16, pay.1)
            .is_some(),
        "the inner tail the < x + 17 admits"
    );
    assert!(
        app.beacon_item_tooltip_hover(pay.0 - 8 + 17, pay.1)
            .is_none(),
        "the x + 17 the band refuses"
    );
    assert!(
        app.beacon_item_tooltip_hover(pay.0 - 8 - 2, pay.1)
            .is_none(),
        "the x - 2 the band refuses"
    );
    assert!(
        app.beacon_item_tooltip_hover(pay.0, pay.1 - 8 - 1)
            .is_some(),
        "the top border the -1 admits"
    );
    assert!(
        app.beacon_item_tooltip_hover(pay.0, pay.1 - 8 - 2)
            .is_none(),
        "the y - 2 the band refuses"
    );
    // THE EMPTY SPACE (the no-seat walk the None answers)
    assert!(
        app.beacon_item_tooltip_hover(pay.0 + 60, pay.1 + 60)
            .is_none(),
        "the panel's blank face the None speaks"
    );
    // THE WRONG SCREEN (the world view the beacon walk never serves)
    app.close_beacon(ScreenKind::World);
    assert!(
        app.beacon_item_tooltip_hover(pay.0, pay.1).is_none(),
        "the closed door the walk refuses"
    );
}

/// THE BEACON'S OWN TOOLTIP GATES (the extractTooltip walk's own
/// walls — the carried non-empty the image-less stack the
/// showTooltipWithItemInHand FALSE answers, the empty seats the
/// hasItem gate refuses, the count-0 sentinel the wire's own law
/// keeps): the rows the walls never wear. The REAL walk the
/// offline door wears.
#[test]
fn the_offline_beacon_tooltip_the_carried_wall_and_the_empty_gates_wear() {
    let gold = mc_registry::item_id_by_name("gold_ingot").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in (c - 1)..=(c + 1) {
            for dz in (c - 1)..=(c + 1) {
                world.set(dx, top - 1, dz, crate::world::IRON_BLOCK);
            }
        }
        world.set(c, top, c, crate::world::BEACON);
    }
    app.beacon_stores.insert((c, top, c));
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.hotbar_slots = app.inventory.hotbar();
    app.open_beacon_for((c, top, c));
    let panel = ((640 - 230) / 2, (360 - 219) / 2);
    app.beacon_panel = panel;
    let pay = (panel.0 + 136 + 1 + 8, panel.1 + 110 + 1 + 8);
    let row0 = (panel.0 + 36 + 1 + 8, panel.1 + 137 + 1 + 8);
    app.beacon_slot_rects = vec![(pay.0 - 8, pay.1 - 8, 0), (row0.0 - 8, row0.1 - 8, 1)];
    // THE SANITY (the coin the seat holds — the empty-carried walk
    // the rows speak)
    app.beacon_payment = (gold, 3);
    assert!(
        app.beacon_item_tooltip_hover(pay.0, pay.1).is_some(),
        "the empty carried the tooltip serves"
    );
    // THE CARRIED WALL (the stack the cursor holds — the image-less
    // walk the FALSE verdict keeps, the tooltip the wall refuses)
    app.beacon_carried = (gold, 2);
    assert!(
        app.beacon_item_tooltip_hover(pay.0, pay.1).is_none(),
        "the carried stack the wall speaks"
    );
    app.beacon_carried = (0, 0); // the release the wall retires
                                 // THE EMPTY SEAT (the hasItem gate the untouched payment row
                                 // refuses — the mirror the zero sentinel keeps)
    assert!(
        app.beacon_item_tooltip_hover(row0.0, row0.1).is_none(),
        "the empty mirror row the None speaks"
    );
    // THE COUNT-0 SENTINEL (the wire's own emptiness the hasItem
    // law keeps — the stack the zero count wears)
    app.beacon_payment = (gold, 0);
    assert!(
        app.beacon_item_tooltip_hover(pay.0, pay.1).is_none(),
        "the count-0 the hasItem gate refuses"
    );
    // THE RECOVERED SEAT (the coin the count restores — the rows
    // the walk serves again)
    app.beacon_payment = (gold, 1);
    assert!(
        app.beacon_item_tooltip_hover(pay.0, pay.1).is_some(),
        "the restored coin the rows serve"
    );
}

/// THE BEACON'S OWN HIGHLIGHT SEAT (the 325 round — the
/// extractSlotHighlightBack/Front walks the probe
/// scripts/highlight_325.py banks): the hovered seat the two-layer
/// walk serves — the EMPTY seat the highlight answers (the
/// hasItem gate the law never wears), the carried stack the wall
/// NEVER speaks (the tooltip's own pc 27-46 the highlight law
/// retires), the borders the isHovering band the -1/+17 pair
/// keeps, the wrong screen the None answers. The REAL walk the
/// offline door wears.
#[test]
fn the_offline_beacon_highlight_the_seat_the_bands_the_no_gates_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE TIER-1 PYRAMID'S OWN BASE (the iron ring the realistic
    // open wears — the highlight test the door alone reads)
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in (c - 1)..=(c + 1) {
            for dz in (c - 1)..=(c + 1) {
                world.set(dx, top - 1, dz, crate::world::IRON_BLOCK);
            }
        }
        world.set(c, top, c, crate::world::BEACON);
    }
    app.beacon_stores.insert((c, top, c));
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the empty seats keep
    }
    app.hotbar_slots = app.inventory.hotbar();
    app.open_beacon_for((c, top, c));
    assert_eq!(
        app.screen,
        ScreenKind::Beacon,
        "the offline door the walk wears"
    );
    // THE GEOMETRY THE RENDER BANKS (the 323 round's own stand-in —
    // the payment seat + the two mirror row seats the rects keep)
    let panel = ((640 - 230) / 2, (360 - 219) / 2);
    app.beacon_panel = panel;
    let pay = (panel.0 + 136 + 1 + 8, panel.1 + 110 + 1 + 8);
    let row0 = (panel.0 + 36 + 1 + 8, panel.1 + 137 + 1 + 8);
    let row1 = (row0.0 + 18, row0.1);
    app.beacon_slot_rects = vec![
        (pay.0 - 8, pay.1 - 8, 0),
        (row0.0 - 8, row0.1 - 8, 1),
        (row1.0 - 8, row1.1 - 8, 2),
    ];
    // THE EMPTY SEAT'S OWN ANSWER (the isHighlightable gate the
    // ONLY wall speaks — the payment + the mirror rows the zero
    // sentinels keep, the highlight the seats STILL wear)
    assert_eq!(
        app.beacon_highlight_seat(pay.0, pay.1),
        Some(0),
        "the empty payment the highlight answers"
    );
    assert_eq!(
        app.beacon_highlight_seat(row0.0, row0.1),
        Some(1),
        "the empty mirror row the highlight answers"
    );
    // THE CARRIED STACK'S OWN NO-WALL (the tooltip's carried wall
    // the highlight law retires — the held stack the highlight
    // STILL paints)
    app.beacon_carried = (7, 3);
    assert_eq!(
        app.beacon_highlight_seat(pay.0, pay.1),
        Some(0),
        "the carried stack the highlight never refuses"
    );
    app.beacon_carried = (0, 0);
    // THE ISHOVERING BORDERS (the -1/+1 growth the 324 round's own
    // law keeps — the 18x18 outer cell the edge pixels answer)
    assert_eq!(
        app.beacon_highlight_seat(pay.0 - 8 - 1, pay.1),
        Some(0),
        "the left border the -1 admits"
    );
    assert_eq!(
        app.beacon_highlight_seat(pay.0 - 8 + 16, pay.1),
        Some(0),
        "the inner tail the < x + 17 admits"
    );
    assert_eq!(
        app.beacon_highlight_seat(pay.0 - 8 + 17, pay.1),
        None,
        "the x + 17 the band refuses"
    );
    assert_eq!(
        app.beacon_highlight_seat(pay.0, pay.1 - 8 - 2),
        None,
        "the y - 2 the band refuses"
    );
    // THE BLANK FACE (the no-seat walk the None answers)
    assert_eq!(
        app.beacon_highlight_seat(pay.0 + 60, pay.1 + 60),
        None,
        "the panel's blank face the None speaks"
    );
    // THE WRONG SCREEN (the world view the highlight never wears)
    app.close_beacon(ScreenKind::World);
    assert_eq!(
        app.beacon_highlight_seat(pay.0, pay.1),
        None,
        "the closed door the walk refuses"
    );
}

/// THE BEACON'S OWN HIGHLIGHT PAINT (the REAL render walk — the
/// 320x180 headless canvas the beacon screen renders, the two
/// layers the exact alpha math answers): the empty payment cell
/// the back alpha-96 + the front alpha-32 pair tints (the hasItem
/// gate the paint law never wears), the transparent border ring
/// the 24x24 sprite's own 4px margin keeps (the cell-only tint
/// the geometry pins), the no-mouse pass the paint never serves.
#[test]
fn the_offline_beacon_highlight_the_paint_the_two_layers_the_cell_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let world = app.offline_world.as_mut().unwrap();
        for dx in (c - 1)..=(c + 1) {
            for dz in (c - 1)..=(c + 1) {
                world.set(dx, top - 1, dz, crate::world::IRON_BLOCK);
            }
        }
        world.set(c, top, c, crate::world::BEACON);
    }
    app.beacon_stores.insert((c, top, c));
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the empty seats the paint law reads
    }
    app.hotbar_slots = app.inventory.hotbar();
    app.open_beacon_for((c, top, c));
    // THE REAL RENDER PAIR (the 320x180 the scale-1 canvas keeps —
    // the same frame the scramble seed answers, the mouse the only
    // delta wears)
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let blend = |dst: u8, alpha: f32| -> u8 {
        let d = dst as f32 / 255.0;
        let v = d * (1.0 - alpha) + 1.0 * alpha;
        ((v.clamp(0.0, 1.0)) * 255.0 + 0.5) as u8
    };
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    let mut assets = assets_root();
    {
        let mut a = GuiAssets::open(&assets);
        app.mouse_pos = None;
        app.render(&mut fb_a, &mut a);
    }
    // THE REAL GEOMETRY THE RENDER BANKS (the rects the render walk
    // itself computes — the 320x180 panel the payment cell answers)
    let panel = ((320 - 230) / 2, (180 - 219) / 2);
    let pay = (panel.0 + 136 + 1, panel.1 + 110 + 1);
    let cx = pay.0 + 8;
    let cy = pay.1 + 8;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets);
        app.mouse_pos = Some((cx, cy));
        app.render(&mut fb_b, &mut a);
    }
    // THE CELL'S OWN TINT (the empty seat the highlight paints —
    // the exact two-layer math the sprite alphas answer: the back
    // 96/255 then the front 32/255 the white the px walk blends)
    let a0 = pixel(&fb_a, cx, cy);
    let expected = blend(blend(a0, 96.0 / 255.0), 32.0 / 255.0);
    assert_eq!(
        pixel(&fb_b, cx, cy),
        expected,
        "the two-layer tint the exact math wears (back 96 + front 32)"
    );
    assert_ne!(
        a0, expected,
        "the tint the paint speaks (the highlight the canvas moves)"
    );
    // THE BORDER RING (the 1px inside the hit band the sprite's own
    // 4px transparent margin keeps — the cell-only tint the
    // geometry pins)
    assert_eq!(
        pixel(&fb_a, pay.0 - 1, cy),
        pixel(&fb_b, pay.0 - 1, cy),
        "the border ring the sprite's margin keeps (no paint)"
    );
    // THE FAR FACE (the panel's own distance the paint never wears)
    assert_eq!(
        pixel(&fb_a, panel.0 + 20, panel.1 + 180),
        pixel(&fb_b, panel.0 + 20, panel.1 + 180),
        "the far face the same verdict keeps"
    );
}

/// THE INVENTORY'S OWN HIGHLIGHT SEAT (the 326 round — the shared
/// AbstractContainerScreen law the survival screen re-wears, the
/// probe scripts/inventory_highlight_326.py the subclass absence
/// banks): the hovered seat's inner origin the inventory_hover
/// walk's own verdict answers — the craft/armor/offhand/store rows
/// the +1 inset keep, the NO-GATE law the 325 round banks (the
/// carried stack the wall never speaks, the empty seat the
/// highlight answers), the wrong screen the None answers. The REAL
/// walk the offline door wears.
#[test]
fn the_offline_inventory_highlight_the_seat_the_no_gates_wear() {
    let mut app = App::new(3);
    app.enter_world();
    app.open_inventory();
    assert_eq!(
        app.screen,
        ScreenKind::Inventory,
        "the survival door the walk wears"
    );
    // THE PANEL THE RENDER BANKS (the 320x180 the centered canvas
    // keeps — the logic walk the same origin fns the render wears)
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    app.inv_panel = panel;
    // THE STORE ROW'S OWN ORIGIN (the +1 inset the Slot.x keeps)
    let (sx, sy) = inventory_slot_origin(panel, 0);
    let origin = (sx + 1, sy + 1);
    assert_eq!(
        app.inventory_highlight_origin(sx + 1 + 8, sy + 1 + 8),
        Some(origin),
        "the store row the origin answers"
    );
    // THE RESULT SEAT'S OWN ANSWER (the craft result the same walk
    // serves)
    let (rx, ry) = craft_slot_origin(panel, 4);
    assert_eq!(
        app.inventory_highlight_origin(rx + 1 + 8, ry + 1 + 8),
        Some((rx + 1, ry + 1)),
        "the result seat the origin answers"
    );
    // THE ARMOR ROW'S OWN ANSWER (the helmet cell the same walk
    // serves)
    let (ax, ay) = armor_slot_origin(panel, 0);
    assert_eq!(
        app.inventory_highlight_origin(ax + 1 + 8, ay + 1 + 8),
        Some((ax + 1, ay + 1)),
        "the armor seat the origin answers"
    );
    // THE OFFHAND'S OWN ANSWER (the shield cell the same walk
    // serves)
    let (ox, oy) = offhand_slot_origin(panel);
    assert_eq!(
        app.inventory_highlight_origin(ox + 1 + 8, oy + 1 + 8),
        Some((ox + 1, oy + 1)),
        "the offhand seat the origin answers"
    );
    // THE NO-GATE LAW (the 325 round's own verdict the survival
    // screen re-wears — the carried stack the wall NEVER speaks,
    // the empty seats the plain answer keeps)
    app.inv_carried = (7, 3);
    assert_eq!(
        app.inventory_highlight_origin(sx + 1 + 8, sy + 1 + 8),
        Some(origin),
        "the carried stack the highlight never refuses"
    );
    app.inv_carried = (0, 0);
    // THE BLANK FACE (the no-seat walk the None answers)
    assert_eq!(
        app.inventory_highlight_origin(panel.0 - 20, panel.1 - 20),
        None,
        "the outside face the None speaks"
    );
    // THE WRONG SCREEN (the world view the highlight never wears)
    app.close_inventory(ScreenKind::World);
    assert_eq!(
        app.inventory_highlight_origin(sx + 1 + 8, sy + 1 + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE INVENTORY'S OWN HIGHLIGHT PAINT (the REAL render walk — the
/// 320x180 headless canvas the survival screen renders, the
/// same-frame pair the mouse the only delta wears): the empty
/// store cell the back-96 + the front-32 the EXACT alpha math
/// answers (the two-layer order the composite pins), the
/// transparent border ring the 24x24 sprite's own 4px margin
/// keeps, the no-mouse pass the paint never serves.
#[test]
fn the_offline_inventory_highlight_the_paint_the_cell_wear() {
    let mut app = App::new(3);
    app.enter_world();
    app.open_inventory();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the empty seats the paint law reads
    }
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let blend = |dst: u8, alpha: f32| -> u8 {
        let d = dst as f32 / 255.0;
        let v = d * (1.0 - alpha) + 1.0 * alpha;
        ((v.clamp(0.0, 1.0)) * 255.0 + 0.5) as u8
    };
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_a, &mut a);
    }
    // THE REAL GEOMETRY THE RENDER BANKS (the panel the centered
    // canvas keeps — the store row 0 the hotbar's own cell answers)
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    let (sx, sy) = inventory_slot_origin(panel, 0);
    let (ox, oy) = (sx + 1, sy + 1);
    let cx = ox + 8;
    let cy = oy + 8;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((cx, cy));
        app.render(&mut fb_b, &mut a);
    }
    // THE CELL'S OWN TINT (the empty seat the highlight paints —
    // the exact two-layer math the sprite alphas answer)
    let a0 = pixel(&fb_a, cx, cy);
    let expected = blend(blend(a0, 96.0 / 255.0), 32.0 / 255.0);
    assert_eq!(
        pixel(&fb_b, cx, cy),
        expected,
        "the two-layer tint the exact math wears (back 96 + front 32)"
    );
    assert_ne!(
        a0, expected,
        "the tint the paint speaks (the highlight the canvas moves)"
    );
    // THE BORDER RING (the 1px inside the hover walk's band the
    // sprite's own 4px transparent margin keeps — the cell-only
    // tint the geometry pins)
    assert_eq!(
        pixel(&fb_a, ox - 1, cy),
        pixel(&fb_b, ox - 1, cy),
        "the border ring the sprite's margin keeps (no paint)"
    );
    // THE FAR FACE (the panel's own distance the paint never wears)
    assert_eq!(
        pixel(&fb_a, panel.0 + 20, panel.1 + 20),
        pixel(&fb_b, panel.0 + 20, panel.1 + 20),
        "the far face the same verdict keeps"
    );
}

/// THE CRAFTING TABLE'S OWN HIGHLIGHT SEAT (the 327 round — the
/// same shared AbstractContainerScreen law the beacon + inventory
/// rounds bank): the hovered seat's inner origin the table_hover_menu
/// walk's own verdict answers — the grid cells / the result seat /
/// the store rows the +1 inset keep, the NO-GATE law the 325 round
/// banks (the carried stack the wall never speaks, the empty seat
/// the highlight answers), the wrong screen the None answers. The
/// REAL walk the offline door wears.
#[test]
fn the_offline_table_highlight_the_seat_the_no_gates_wear() {
    let mut app = App::new(3);
    app.enter_world();
    app.open_crafting_table();
    assert_eq!(
        app.screen,
        ScreenKind::Crafting,
        "the table door the walk wears"
    );
    // THE PANEL THE RENDER BANKS (the 320x180 the centered canvas
    // keeps — the logic walk the same origin fns the render wears)
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    app.table_panel = panel;
    // THE STORE ROW'S OWN ORIGIN (the +1 inset the Slot.x keeps)
    let (sx, sy) = inventory_slot_origin(panel, 0);
    let origin = (sx + 1, sy + 1);
    assert_eq!(
        app.table_highlight_origin(sx + 1 + 8, sy + 1 + 8),
        Some(origin),
        "the store row the origin answers"
    );
    // THE MAIN GRID'S OWN ROWS (the menu 10 the store 9 the inverse
    // law keys, the menu 36 the store 35 the tail answers)
    let (mx9, my9) = inventory_slot_origin(panel, 9);
    assert_eq!(
        app.table_highlight_origin(mx9 + 1 + 8, my9 + 1 + 8),
        Some((mx9 + 1, my9 + 1)),
        "the main grid row the origin answers"
    );
    let (mx35, my35) = inventory_slot_origin(panel, 35);
    assert_eq!(
        app.table_highlight_origin(mx35 + 1 + 8, my35 + 1 + 8),
        Some((mx35 + 1, my35 + 1)),
        "the main grid tail the origin answers"
    );
    // THE HOTBAR ROW'S OWN ANSWER (the menu 45 the store 8 the
    // inverse walk keys)
    let (hx, hy) = inventory_slot_origin(panel, 8);
    assert_eq!(
        app.table_highlight_origin(hx + 1 + 8, hy + 1 + 8),
        Some((hx + 1, hy + 1)),
        "the hotbar row the origin answers"
    );
    // THE GRID CELL'S OWN ANSWER (the 3x3 cell the same walk serves)
    let (gx, gy) = table_craft_origin(panel, 4);
    assert_eq!(
        app.table_highlight_origin(gx + 1 + 8, gy + 1 + 8),
        Some((gx + 1, gy + 1)),
        "the grid cell the origin answers"
    );
    // THE RESULT SEAT'S OWN ANSWER (the addResultSlot(124, 35) seat
    // the same walk serves)
    let (rx, ry) = table_craft_origin(panel, 9);
    assert_eq!(
        app.table_highlight_origin(rx + 1 + 8, ry + 1 + 8),
        Some((rx + 1, ry + 1)),
        "the result seat the origin answers"
    );
    // THE NO-GATE LAW (the 325 round's own verdict the table screen
    // re-wears — the carried stack the wall NEVER speaks, the empty
    // seats the plain answer keeps)
    app.inv_carried = (7, 3);
    assert_eq!(
        app.table_highlight_origin(sx + 1 + 8, sy + 1 + 8),
        Some(origin),
        "the carried stack the highlight never refuses"
    );
    app.inv_carried = (0, 0);
    // THE BLANK FACE (the no-seat walk the None answers)
    assert_eq!(
        app.table_highlight_origin(panel.0 - 20, panel.1 - 20),
        None,
        "the outside face the None speaks"
    );
    // THE WRONG SCREEN (the world view the highlight never wears)
    app.close_crafting_table(ScreenKind::World);
    assert_eq!(
        app.table_highlight_origin(sx + 1 + 8, sy + 1 + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE CRAFTING TABLE'S OWN HIGHLIGHT PAINT (the REAL render walk —
/// the 320x180 headless canvas the table screen renders, the
/// same-frame pair the mouse the only delta wears): the empty store
/// cell the back-96 + the front-32 the EXACT alpha math answers (the
/// two-layer order the composite pins), the transparent border ring
/// the 24x24 sprite's own 4px margin keeps, the no-mouse pass the
/// paint never serves.
#[test]
fn the_offline_table_highlight_the_paint_the_cell_wear() {
    let mut app = App::new(3);
    app.enter_world();
    app.open_crafting_table();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the empty seats the paint law reads
    }
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let blend = |dst: u8, alpha: f32| -> u8 {
        let d = dst as f32 / 255.0;
        let v = d * (1.0 - alpha) + 1.0 * alpha;
        ((v.clamp(0.0, 1.0)) * 255.0 + 0.5) as u8
    };
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_a, &mut a);
    }
    // THE REAL GEOMETRY THE RENDER BANKS (the panel the centered
    // canvas keeps — the store row 0 the hotbar's own cell answers)
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    let (sx, sy) = inventory_slot_origin(panel, 0);
    let (ox, oy) = (sx + 1, sy + 1);
    let cx = ox + 8;
    let cy = oy + 8;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((cx, cy));
        app.render(&mut fb_b, &mut a);
    }
    // THE CELL'S OWN TINT (the empty seat the highlight paints —
    // the exact two-layer math the sprite alphas answer)
    let a0 = pixel(&fb_a, cx, cy);
    let expected = blend(blend(a0, 96.0 / 255.0), 32.0 / 255.0);
    assert_eq!(
        pixel(&fb_b, cx, cy),
        expected,
        "the two-layer tint the exact math wears (back 96 + front 32)"
    );
    assert_ne!(
        a0, expected,
        "the tint the paint speaks (the highlight the canvas moves)"
    );
    // THE BORDER RING (the 1px inside the hover walk's band the
    // sprite's own 4px transparent margin keeps — the cell-only
    // tint the geometry pins)
    assert_eq!(
        pixel(&fb_a, ox - 1, cy),
        pixel(&fb_b, ox - 1, cy),
        "the border ring the sprite's margin keeps (no paint)"
    );
    // THE FAR FACE (the panel's own distance the paint never wears)
    assert_eq!(
        pixel(&fb_a, panel.0 + 20, panel.1 + 20),
        pixel(&fb_b, panel.0 + 20, panel.1 + 20),
        "the far face the same verdict keeps"
    );
}

/// THE FURNACE'S OWN HIGHLIGHT SEAT (the 328 round — the same shared
/// AbstractContainerScreen law the beacon + inventory + table rounds
/// bank): the hovered seat's inner origin the furnace_hover_seat
/// walk's own verdict answers — the three furnace cells + the store
/// rows the render's own rects speak, the NO-GATE law the 325 round
/// banks (the carried stack the wall never speaks, the empty cell
/// the highlight answers), the wrong screen the None answers. The
/// REAL walk the offline door wears — the headless frame the REAL
/// rects the render lays.
#[test]
fn the_offline_furnace_highlight_the_seat_the_no_gates_wear() {
    let mut app = App::new(3);
    app.enter_world();
    app.open_furnace((0, 0, 0));
    assert_eq!(
        app.screen,
        ScreenKind::Furnace,
        "the furnace door the walk wears"
    );
    // THE REAL RECTS THE RENDER BANKS (one headless frame the mouse
    // never tracks — the render's own walk the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    assert_eq!(
        app.furnace_panel, panel,
        "the centered panel the canvas keeps"
    );
    // THE SEAT LOOKUP (the menu's own numbering the rects speak —
    // the three furnace cells then the standard rows)
    let seat = |slot: i16| -> (i32, i32) {
        app.furnace_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE INPUT CELL'S OWN ANSWER (the menu 0 the +56/+17 seat the
    // texture speaks)
    let (ix, iy) = seat(0);
    assert_eq!(
        app.furnace_highlight_origin(ix + 8, iy + 8),
        Some((ix, iy)),
        "the input cell the origin answers"
    );
    // THE FUEL CELL'S OWN ANSWER (the menu 1 the +56/+53 seat)
    let (fx, fy) = seat(1);
    assert_eq!(
        app.furnace_highlight_origin(fx + 8, fy + 8),
        Some((fx, fy)),
        "the fuel cell the origin answers"
    );
    // THE RESULT CELL'S OWN ANSWER (the menu 2 the +116/+35 seat)
    let (rx, ry) = seat(2);
    assert_eq!(
        app.furnace_highlight_origin(rx + 8, ry + 8),
        Some((rx, ry)),
        "the result cell the origin answers"
    );
    // THE STORE ROWS' OWN ANSWERS (the menu 3 the main grid's first
    // cell the render speaks, the menu 38 the hotbar tail the SAME
    // walk the clicks share)
    let (sx, sy) = seat(3);
    assert_eq!(
        app.furnace_highlight_origin(sx + 8, sy + 8),
        Some((sx, sy)),
        "the store row the origin answers"
    );
    let (hx, hy) = seat(38);
    assert_eq!(
        app.furnace_highlight_origin(hx + 8, hy + 8),
        Some((hx, hy)),
        "the hotbar tail the origin answers"
    );
    // THE NO-GATE LAW (the 325 round's own verdict the furnace
    // screen re-wears — the carried stack the wall NEVER speaks, the
    // EMPTY cells the plain answer keeps)
    app.furnace_carried = (7, 3);
    assert_eq!(
        app.furnace_highlight_origin(ix + 8, iy + 8),
        Some((ix, iy)),
        "the carried stack the highlight never refuses"
    );
    app.furnace_carried = (0, 0);
    // THE CELL'S OWN EDGES (the strict 16x16 the walk answers — the
    // shared lane the clicks ride the SAME band keeps)
    assert_eq!(
        app.furnace_highlight_origin(ix + 16, iy + 8),
        None,
        "the east edge the walk refuses"
    );
    assert_eq!(
        app.furnace_highlight_origin(ix - 1, iy + 8),
        None,
        "the west edge the walk refuses"
    );
    // THE BLANK FACE (the no-seat walk the None answers)
    assert_eq!(
        app.furnace_highlight_origin(panel.0 - 20, panel.1 - 20),
        None,
        "the outside face the None speaks"
    );
    // THE WRONG SCREEN (the world view the highlight never wears)
    app.close_furnace(ScreenKind::World);
    assert_eq!(
        app.furnace_highlight_origin(ix + 8, iy + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE FURNACE'S OWN HIGHLIGHT PAINT (the REAL render walk — the
/// 320x180 headless canvas the furnace screen renders, the
/// same-frame pair the mouse the only delta wears): the empty input
/// cell the back-96 + the front-32 the EXACT alpha math answers (the
/// two-layer order the composite pins), the transparent border ring
/// the 24x24 sprite's own 4px margin keeps, the no-mouse pass the
/// paint never serves.
#[test]
fn the_offline_furnace_highlight_the_paint_the_cell_wear() {
    let mut app = App::new(3);
    app.enter_world();
    app.open_furnace((0, 0, 0));
    for it in app.furnace_slots.iter_mut() {
        *it = (0, 0); // the empty seats the paint law reads
    }
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let blend = |dst: u8, alpha: f32| -> u8 {
        let d = dst as f32 / 255.0;
        let v = d * (1.0 - alpha) + 1.0 * alpha;
        ((v.clamp(0.0, 1.0)) * 255.0 + 0.5) as u8
    };
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_a, &mut a);
    }
    // THE REAL GEOMETRY THE RENDER BANKS (the menu 0 the +56/+17
    // seat the texture speaks — the +1 inset the rects wear)
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    let (ox, oy) = (panel.0 + 56 + 1, panel.1 + 17 + 1);
    let cx = ox + 8;
    let cy = oy + 8;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((cx, cy));
        app.render(&mut fb_b, &mut a);
    }
    // THE CELL'S OWN TINT (the empty seat the highlight paints —
    // the exact two-layer math the sprite alphas answer)
    let a0 = pixel(&fb_a, cx, cy);
    let expected = blend(blend(a0, 96.0 / 255.0), 32.0 / 255.0);
    assert_eq!(
        pixel(&fb_b, cx, cy),
        expected,
        "the two-layer tint the exact math wears (back 96 + front 32)"
    );
    assert_ne!(
        a0, expected,
        "the tint the paint speaks (the highlight the canvas moves)"
    );
    // THE BORDER RING (the 1px inside the hover walk's band the
    // sprite's own 4px transparent margin keeps — the cell-only
    // tint the geometry pins)
    assert_eq!(
        pixel(&fb_a, ox - 1, cy),
        pixel(&fb_b, ox - 1, cy),
        "the border ring the sprite's margin keeps (no paint)"
    );
    // THE FAR FACE (the panel's own distance the paint never wears)
    assert_eq!(
        pixel(&fb_a, panel.0 + 20, panel.1 + 20),
        pixel(&fb_b, panel.0 + 20, panel.1 + 20),
        "the far face the same verdict keeps"
    );
}

/// THE CHEST'S OWN HIGHLIGHT SEAT (the 329 round — the same shared
/// AbstractContainerScreen law the beacon + inventory + table +
/// furnace rounds bank): the hovered seat's inner origin the
/// chest_hover_seat walk's own verdict answers — the grid cells +
/// the mirror rows the render's own rects speak, the NO-GATE law the
/// 325 round banks (the carried stack the wall never speaks, the
/// empty cell the highlight answers), the wrong screen the None
/// answers. The REAL walk the offline door wears — the headless
/// frame the REAL rects the render lays.
#[test]
fn the_offline_chest_highlight_the_seat_the_no_gates_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    app.open_chest_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Chest,
        "the chest door the walk wears"
    );
    assert!(!app.chest_is_live, "the offline sim the door keeps");
    // THE REAL RECTS THE RENDER BANKS (one headless frame the mouse
    // never tracks — the render's own walk the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    assert_eq!(
        app.chest_panel, panel,
        "the centered panel the canvas keeps"
    );
    // THE SEAT LOOKUP (the menu's own numbering the rects speak —
    // the grid's 0..27 then the mirror's 27..63)
    let seat = |slot: i16| -> (i32, i32) {
        app.chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE GRID'S OWN ANSWERS (the first cell the (8, 18) seat the
    // generic_54 layout speaks, the row's tail the 8th column keys)
    let (gx, gy) = seat(0);
    assert_eq!(
        app.chest_highlight_origin(gx + 8, gy + 8),
        Some((gx, gy)),
        "the grid head the origin answers"
    );
    let (gx8, gy8) = seat(8);
    assert_eq!(
        app.chest_highlight_origin(gx8 + 8, gy8 + 8),
        Some((gx8, gy8)),
        "the grid row tail the origin answers"
    );
    // THE GRID'S LAST CELL (the 26 the third row's tail speaks)
    let (gx26, gy26) = seat(26);
    assert_eq!(
        app.chest_highlight_origin(gx26 + 8, gy26 + 8),
        Some((gx26, gy26)),
        "the grid tail the origin answers"
    );
    // THE MIRROR ROWS' OWN ANSWERS (the menu 27 the main mirror's
    // first cell, the menu 62 the hotbar tail the SAME walk the
    // clicks share)
    let (mx, my) = seat(27);
    assert_eq!(
        app.chest_highlight_origin(mx + 8, my + 8),
        Some((mx, my)),
        "the mirror head the origin answers"
    );
    let (hx, hy) = seat(62);
    assert_eq!(
        app.chest_highlight_origin(hx + 8, hy + 8),
        Some((hx, hy)),
        "the hotbar tail the origin answers"
    );
    // THE NO-GATE LAW (the 325 round's own verdict the chest screen
    // re-wears — the carried stack the wall NEVER speaks, the EMPTY
    // cells the plain answer keeps)
    app.chest_carried = (7, 3);
    assert_eq!(
        app.chest_highlight_origin(gx + 8, gy + 8),
        Some((gx, gy)),
        "the carried stack the highlight never refuses"
    );
    app.chest_carried = (0, 0);
    // THE CELL'S OWN EDGES (the strict 16x16 the walk answers — the
    // shared lane the clicks ride the SAME band keeps)
    assert_eq!(
        app.chest_highlight_origin(gx + 16, gy + 8),
        None,
        "the east edge the walk refuses"
    );
    assert_eq!(
        app.chest_highlight_origin(gx - 1, gy + 8),
        None,
        "the west edge the walk refuses"
    );
    // THE BLANK FACE (the no-seat walk the None answers)
    assert_eq!(
        app.chest_highlight_origin(panel.0 - 20, panel.1 - 20),
        None,
        "the outside face the None speaks"
    );
    // THE WRONG SCREEN (the world view the highlight never wears)
    app.close_chest_container(ScreenKind::World);
    assert_eq!(
        app.chest_highlight_origin(gx + 8, gy + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE CHEST'S OWN HIGHLIGHT PAINT (the REAL render walk — the
/// 320x180 headless canvas the chest screen renders, the same-frame
/// pair the mouse the only delta wears): the empty grid head the
/// back-96 + the front-32 the EXACT alpha math answers (the
/// two-layer order the composite pins), the transparent border ring
/// the 24x24 sprite's own 4px margin keeps, the no-mouse pass the
/// paint never serves.
#[test]
fn the_offline_chest_highlight_the_paint_the_cell_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    app.open_chest_for(cell);
    for it in app.chest_slots.iter_mut() {
        *it = (0, 0); // the empty seats the paint law reads
    }
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let blend = |dst: u8, alpha: f32| -> u8 {
        let d = dst as f32 / 255.0;
        let v = d * (1.0 - alpha) + 1.0 * alpha;
        ((v.clamp(0.0, 1.0)) * 255.0 + 0.5) as u8
    };
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_a, &mut a);
    }
    // THE REAL GEOMETRY THE RENDER BANKS (the grid head the (8, 18)
    // seat the generic_54 layout speaks — the +1 inset the rects wear)
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    let (ox, oy) = (panel.0 + 8 + 1, panel.1 + 18 + 1);
    let cx = ox + 8;
    let cy = oy + 8;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((cx, cy));
        app.render(&mut fb_b, &mut a);
    }
    // THE CELL'S OWN TINT (the empty seat the highlight paints —
    // the exact two-layer math the sprite alphas answer)
    let a0 = pixel(&fb_a, cx, cy);
    let expected = blend(blend(a0, 96.0 / 255.0), 32.0 / 255.0);
    assert_eq!(
        pixel(&fb_b, cx, cy),
        expected,
        "the two-layer tint the exact math wears (back 96 + front 32)"
    );
    assert_ne!(
        a0, expected,
        "the tint the paint speaks (the highlight the canvas moves)"
    );
    // THE BORDER RING (the 1px inside the hover walk's band the
    // sprite's own 4px transparent margin keeps — the cell-only
    // tint the geometry pins)
    assert_eq!(
        pixel(&fb_a, ox - 1, cy),
        pixel(&fb_b, ox - 1, cy),
        "the border ring the sprite's margin keeps (no paint)"
    );
    // THE FAR FACE (the title row's own distance the paint never
    // wears — the static label band the same verdict keeps)
    assert_eq!(
        pixel(&fb_a, panel.0 + 150, panel.1 + 6),
        pixel(&fb_b, panel.0 + 150, panel.1 + 6),
        "the far face the same verdict keeps"
    );
}

/// THE HOPPER'S OWN HIGHLIGHT SEAT (the 330 round — the same shared
/// AbstractContainerScreen law the beacon + inventory + table +
/// furnace + chest rounds bank, the SHARED SEAT the chest's own fn
/// answers [the Chest gate the hopper screen wears, the rects the
/// hopper's own render laid the walk reads]): the five-seat row +
/// the mirror rows the origin answers, the NO-GATE law the 325
/// round banks (the carried stack the wall never speaks, the empty
/// cell the highlight answers), the wrong screen the None answers.
/// The REAL walk the offline door wears — the headless frame the
/// REAL rects the render lays.
#[test]
fn the_offline_hopper_highlight_the_seat_the_no_gates_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::HOPPER);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the empty hand the door's own lane keeps
    }
    app.open_hopper_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Chest,
        "the hopper door the Chest shape wears"
    );
    assert!(app.chest_window_hopper, "the hopper canvas the flag banks");
    // THE REAL RECTS THE RENDER BANKS (one headless frame the mouse
    // never tracks — the render's own walk the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = ((320 - 176) / 2, (180 - 133) / 2);
    assert_eq!(
        app.chest_panel, panel,
        "the centered panel the canvas keeps"
    );
    // THE SEAT LOOKUP (the menu's own numbering the rects speak —
    // the five-seat row 0..5 then the mirror's 5..41)
    let seat = |slot: i16| -> (i32, i32) {
        app.chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE FIVE-SEAT ROW'S OWN ANSWERS (the head the (44, 20) seat the
    // ctor's Slot loop speaks, the tail the 4th column keys)
    let (ix, iy) = seat(0);
    assert_eq!(
        app.chest_highlight_origin(ix + 8, iy + 8),
        Some((ix, iy)),
        "the seat head the origin answers"
    );
    let (tx, ty) = seat(4);
    assert_eq!(
        app.chest_highlight_origin(tx + 8, ty + 8),
        Some((tx, ty)),
        "the seat tail the origin answers"
    );
    // THE MIRROR ROWS' OWN ANSWERS (the menu 5 the main mirror's
    // first cell, the menu 40 the hotbar tail the SAME walk the
    // clicks share)
    let (mx, my) = seat(5);
    assert_eq!(
        app.chest_highlight_origin(mx + 8, my + 8),
        Some((mx, my)),
        "the mirror head the origin answers"
    );
    let (hx, hy) = seat(40);
    assert_eq!(
        app.chest_highlight_origin(hx + 8, hy + 8),
        Some((hx, hy)),
        "the hotbar tail the origin answers"
    );
    // THE NO-GATE LAW (the 325 round's own verdict the hopper screen
    // re-wears — the carried stack the wall NEVER speaks, the EMPTY
    // cells the plain answer keeps)
    app.chest_carried = (7, 3);
    assert_eq!(
        app.chest_highlight_origin(ix + 8, iy + 8),
        Some((ix, iy)),
        "the carried stack the highlight never refuses"
    );
    app.chest_carried = (0, 0);
    // THE CELL'S OWN EDGES (the strict 16x16 the walk answers — the
    // shared lane the clicks ride the SAME band keeps)
    assert_eq!(
        app.chest_highlight_origin(ix + 16, iy + 8),
        None,
        "the east edge the walk refuses"
    );
    assert_eq!(
        app.chest_highlight_origin(ix - 1, iy + 8),
        None,
        "the west edge the walk refuses"
    );
    // THE BLANK FACE (the no-seat walk the None answers)
    assert_eq!(
        app.chest_highlight_origin(panel.0 - 20, panel.1 - 20),
        None,
        "the outside face the None speaks"
    );
    // THE WRONG SCREEN (the world view the highlight never wears)
    app.close_chest_container(ScreenKind::World);
    assert_eq!(
        app.chest_highlight_origin(ix + 8, iy + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE HOPPER'S OWN HIGHLIGHT PAINT (the REAL render walk — the
/// 320x180 headless canvas the hopper screen renders, the same-frame
/// pair the mouse the only delta wears): the empty seat head the
/// back-96 + the front-32 the EXACT alpha math answers (the
/// two-layer order the composite pins), the transparent border ring
/// the 24x24 sprite's own 4px margin keeps, the no-mouse pass the
/// paint never serves.
#[test]
fn the_offline_hopper_highlight_the_paint_the_cell_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::HOPPER);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_hopper_for(cell);
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let blend = |dst: u8, alpha: f32| -> u8 {
        let d = dst as f32 / 255.0;
        let v = d * (1.0 - alpha) + 1.0 * alpha;
        ((v.clamp(0.0, 1.0)) * 255.0 + 0.5) as u8
    };
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_a, &mut a);
    }
    // THE REAL GEOMETRY THE RENDER BANKS (the seat head the (44, 20)
    // seat the ctor's Slot loop speaks — the +1 inset the rects wear)
    let panel = ((320 - 176) / 2, (180 - 133) / 2);
    let (ox, oy) = (panel.0 + 44 + 1, panel.1 + 20 + 1);
    let cx = ox + 8;
    let cy = oy + 8;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((cx, cy));
        app.render(&mut fb_b, &mut a);
    }
    // THE CELL'S OWN TINT (the empty seat the highlight paints —
    // the exact two-layer math the sprite alphas answer)
    let a0 = pixel(&fb_a, cx, cy);
    let expected = blend(blend(a0, 96.0 / 255.0), 32.0 / 255.0);
    assert_eq!(
        pixel(&fb_b, cx, cy),
        expected,
        "the two-layer tint the exact math wears (back 96 + front 32)"
    );
    assert_ne!(
        a0, expected,
        "the tint the paint speaks (the highlight the canvas moves)"
    );
    // THE BORDER RING (the 1px inside the hover walk's band the
    // sprite's own 4px transparent margin keeps — the cell-only
    // tint the geometry pins)
    assert_eq!(
        pixel(&fb_a, ox - 1, cy),
        pixel(&fb_b, ox - 1, cy),
        "the border ring the sprite's margin keeps (no paint)"
    );
    // THE FAR FACE (the title row's own distance the paint never
    // wears — the static label band the same verdict keeps)
    assert_eq!(
        pixel(&fb_a, panel.0 + 150, panel.1 + 6),
        pixel(&fb_b, panel.0 + 150, panel.1 + 6),
        "the far face the same verdict keeps"
    );
}

/// THE DISPENSER'S OWN HIGHLIGHT SEAT (the 331 round — the same shared
/// AbstractContainerScreen law the beacon + inventory + table +
/// furnace + chest + hopper rounds bank, the SHARED SEAT the
/// dispenser's own fn answers [the Dispenser gate the walk keys, the
/// rects the dispenser's own render laid the walk reads]): the 3x3
/// grid + the mirror rows the origin answers, the NO-GATE law the
/// 325 round banks (the carried stack the wall never speaks, the
/// empty cell the highlight answers), the wrong screen the None
/// answers. The REAL walk the offline door wears — the headless
/// frame the REAL rects the render lays.
#[test]
fn the_offline_dispenser_highlight_the_seat_the_no_gates_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the empty hand the door's own lane keeps
    }
    app.open_dispenser_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Dispenser,
        "the dispenser door the kind wears"
    );
    // THE REAL RECTS THE RENDER BANKS (one headless frame the mouse
    // never tracks — the render's own walk the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    assert_eq!(
        app.dispenser_panel, panel,
        "the centered panel the canvas keeps"
    );
    // THE SEAT LOOKUP (the menu's own numbering the rects speak —
    // the 3x3 grid 0..9 then the mirror's 9..45)
    let seat = |slot: i16| -> (i32, i32) {
        app.dispenser_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE GRID'S OWN ANSWERS (the head the (62, 17) seat the ctor's
    // Slot loop speaks, the tail the 3x3's ninth cell keys)
    let (gx, gy) = seat(0);
    assert_eq!(
        app.dispenser_highlight_origin(gx + 8, gy + 8),
        Some((gx, gy)),
        "the grid head the origin answers"
    );
    let (tx, ty) = seat(8);
    assert_eq!(
        app.dispenser_highlight_origin(tx + 8, ty + 8),
        Some((tx, ty)),
        "the grid tail the origin answers"
    );
    // THE MIRROR ROWS' OWN ANSWERS (the menu 9 the main mirror's
    // first cell, the menu 44 the hotbar tail the SAME walk the
    // clicks share)
    let (mx, my) = seat(9);
    assert_eq!(
        app.dispenser_highlight_origin(mx + 8, my + 8),
        Some((mx, my)),
        "the mirror head the origin answers"
    );
    let (hx, hy) = seat(44);
    assert_eq!(
        app.dispenser_highlight_origin(hx + 8, hy + 8),
        Some((hx, hy)),
        "the hotbar tail the origin answers"
    );
    // THE NO-GATE LAW (the 325 round's own verdict the dispenser
    // screen re-wears — the carried stack the wall NEVER speaks, the
    // EMPTY cells the plain answer keeps)
    app.dispenser_carried = (7, 3);
    assert_eq!(
        app.dispenser_highlight_origin(gx + 8, gy + 8),
        Some((gx, gy)),
        "the carried stack the highlight never refuses"
    );
    app.dispenser_carried = (0, 0);
    // THE CELL'S OWN EDGES (the strict 16x16 the walk answers — the
    // shared lane the clicks ride the SAME band keeps)
    assert_eq!(
        app.dispenser_highlight_origin(gx + 16, gy + 8),
        None,
        "the east edge the walk refuses"
    );
    assert_eq!(
        app.dispenser_highlight_origin(gx - 1, gy + 8),
        None,
        "the west edge the walk refuses"
    );
    // THE BLANK FACE (the no-seat walk the None answers)
    assert_eq!(
        app.dispenser_highlight_origin(panel.0 - 20, panel.1 - 20),
        None,
        "the outside face the None speaks"
    );
    // THE WRONG SCREEN (the world view the highlight never wears)
    app.close_dispenser(ScreenKind::World);
    assert_eq!(
        app.dispenser_highlight_origin(gx + 8, gy + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE DISPENSER'S OWN HIGHLIGHT PAINT (the REAL render walk — the
/// 320x180 headless canvas the dispenser screen renders, the
/// same-frame pair the mouse the only delta wears): the empty grid
/// head the back-96 + the front-32 the EXACT alpha math answers (the
/// two-layer order the composite pins), the transparent border ring
/// the 24x24 sprite's own 4px margin keeps, the no-mouse pass the
/// paint never serves.
#[test]
fn the_offline_dispenser_highlight_the_paint_the_cell_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_dispenser_for(cell);
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let blend = |dst: u8, alpha: f32| -> u8 {
        let d = dst as f32 / 255.0;
        let v = d * (1.0 - alpha) + 1.0 * alpha;
        ((v.clamp(0.0, 1.0)) * 255.0 + 0.5) as u8
    };
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_a, &mut a);
    }
    // THE REAL GEOMETRY THE RENDER BANKS (the grid head the (62, 17)
    // seat the ctor's Slot loop speaks — the +1 inset the rects wear)
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    let (ox, oy) = (panel.0 + 62 + 1, panel.1 + 17 + 1);
    let cx = ox + 8;
    let cy = oy + 8;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((cx, cy));
        app.render(&mut fb_b, &mut a);
    }
    // THE CELL'S OWN TINT (the empty seat the highlight paints —
    // the exact two-layer math the sprite alphas answer)
    let a0 = pixel(&fb_a, cx, cy);
    let expected = blend(blend(a0, 96.0 / 255.0), 32.0 / 255.0);
    assert_eq!(
        pixel(&fb_b, cx, cy),
        expected,
        "the two-layer tint the exact math wears (back 96 + front 32)"
    );
    assert_ne!(
        a0, expected,
        "the tint the paint speaks (the highlight the canvas moves)"
    );
    // THE BORDER RING (the 1px inside the hover walk's band the
    // sprite's own 4px transparent margin keeps — the cell-only
    // tint the geometry pins)
    assert_eq!(
        pixel(&fb_a, ox - 1, cy),
        pixel(&fb_b, ox - 1, cy),
        "the border ring the sprite's margin keeps (no paint)"
    );
    // THE FAR FACE (the centered title's own reach the paint never
    // wears — the static band the point keeps, the centered row the
    // +150 face the text never spans)
    assert_eq!(
        pixel(&fb_a, panel.0 + 150, panel.1 + 6),
        pixel(&fb_b, panel.0 + 150, panel.1 + 6),
        "the far face the same verdict keeps"
    );
}

/// THE BREWING STAND'S OWN DOOR SEAT (the autopush-332 round — the 26.2
/// ctor's own literal seats the bytecode probe banks): the five-seat
/// walk the origin answers, the DIPPED middle bottle the +7 pitch
/// the 26.2 re-layout pins, the NO-GATE law the family banks, the
/// wrong screen the None answers. The REAL walk the offline door
/// wears — the headless frame the REAL rects the render lays.
#[test]
fn the_offline_brewing_the_door_the_five_seats_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::BREWING_STAND);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the empty hand the door's own lane keeps
    }
    app.open_brewing_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Brewing,
        "the brewing door the kind wears"
    );
    assert_eq!(app.brewing_slots.len(), 5, "the fresh five the open banks");
    assert_eq!(
        app.brewing_mirror.len(),
        36,
        "the mirror rows the sync keeps"
    );
    // THE REAL RECTS THE RENDER BANKS (one headless frame the mouse
    // never tracks — the render's own walk the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    assert_eq!(
        app.brewing_panel, panel,
        "the centered panel the canvas keeps"
    );
    let seat = |slot: i16| -> (i32, i32) {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE 26.2 RE-LAYOUT'S OWN PITCH (the ctor's literal adds the
    // probe spoke: the bottles 23px apart, the MIDDLE bottle the +7
    // dip wears, the fuel the far-left seat keeps)
    let (b0x, b0y) = seat(0);
    let (b1x, b1y) = seat(1);
    let (b2x, b2y) = seat(2);
    assert_eq!(
        (b1x - b0x, b1y - b0y),
        (23, 7),
        "the dipped middle bottle the 26.2 pitch banks"
    );
    assert_eq!(
        (b2x - b1x, b2y - b1y),
        (23, -7),
        "the third bottle the rise answers"
    );
    let (g3x, g3y) = seat(3);
    let (g4x, g4y) = seat(4);
    assert_eq!(
        (g3x - b1x, g3y - (b1y - 7)),
        (0, -34),
        "the ingredient the top-center seat keeps"
    );
    assert_eq!(
        (g4x - b0x, g4y - b0y),
        (-39, -34),
        "the fuel the far-left seat keeps"
    );
    // THE FIVE-SEAT'S OWN ANSWERS (each stand cell + the mirror rows
    // the origin answers — the ONE walk the clicks share)
    let seat_origin = |slot: i16| -> (i32, i32) {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    assert_eq!(
        app.brewing_highlight_origin(seat_origin(0).0 + 8, seat_origin(0).1 + 8),
        Some(seat_origin(0)),
        "the bottle head the origin answers"
    );
    assert_eq!(
        app.brewing_highlight_origin(seat_origin(1).0 + 8, seat_origin(1).1 + 8),
        Some(seat_origin(1)),
        "the dipped bottle the origin answers"
    );
    assert_eq!(
        app.brewing_highlight_origin(seat_origin(2).0 + 8, seat_origin(2).1 + 8),
        Some(seat_origin(2)),
        "the bottle tail the origin answers"
    );
    assert_eq!(
        app.brewing_highlight_origin(seat_origin(3).0 + 8, seat_origin(3).1 + 8),
        Some(seat_origin(3)),
        "the ingredient the origin answers"
    );
    assert_eq!(
        app.brewing_highlight_origin(seat_origin(4).0 + 8, seat_origin(4).1 + 8),
        Some(seat_origin(4)),
        "the fuel the origin answers"
    );
    let (mx, my) = seat(5);
    assert_eq!(
        app.brewing_highlight_origin(mx + 8, my + 8),
        Some((mx, my)),
        "the mirror head the origin answers"
    );
    let (hx, hy) = seat(40);
    assert_eq!(
        app.brewing_highlight_origin(hx + 8, hy + 8),
        Some((hx, hy)),
        "the hotbar tail the origin answers"
    );
    // THE NO-GATE LAW (the family's own verdict the brewing screen
    // re-wears — the carried stack the wall NEVER speaks, the EMPTY
    // cells the plain answer keeps)
    app.brewing_carried = (7, 3);
    assert_eq!(
        app.brewing_highlight_origin(b0x + 8, b0y + 8),
        Some((b0x, b0y)),
        "the carried stack the highlight never refuses"
    );
    app.brewing_carried = (0, 0);
    // THE CELL'S OWN EDGES (the strict 16x16 the walk answers)
    assert_eq!(
        app.brewing_highlight_origin(b0x + 16, b0y + 8),
        None,
        "the east edge the walk refuses"
    );
    assert_eq!(
        app.brewing_highlight_origin(b0x - 1, b0y + 8),
        None,
        "the west edge the walk refuses"
    );
    // THE BLANK FACE (the no-seat walk the None answers)
    assert_eq!(
        app.brewing_highlight_origin(panel.0 - 20, panel.1 - 20),
        None,
        "the outside face the None speaks"
    );
    // THE WRONG SCREEN (the world view the highlight never wears)
    app.close_brewing(ScreenKind::World);
    assert_eq!(
        app.brewing_highlight_origin(b0x + 8, b0y + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE BREWING STAND'S OWN HIGHLIGHT PAINT (the REAL render walk —
/// the 320x180 headless canvas the brewing screen renders, the
/// same-frame pair the mouse the only delta wears): the empty fuel
/// seat the back-96 + the front-32 the EXACT alpha math answers (the
/// two-layer order the composite pins), the transparent border ring
/// the 24x24 sprite's own 4px margin keeps, the no-mouse pass the
/// paint never serves.
#[test]
fn the_offline_brewing_the_paint_the_cell_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::BREWING_STAND);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_brewing_for(cell);
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let blend = |dst: u8, alpha: f32| -> u8 {
        let d = dst as f32 / 255.0;
        let v = d * (1.0 - alpha) + 1.0 * alpha;
        ((v.clamp(0.0, 1.0)) * 255.0 + 0.5) as u8
    };
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_a, &mut a);
    }
    // THE REAL GEOMETRY THE RENDER BANKS (the fuel seat the (17, 17)
    // seat the 26.2 ctor's own literal speaks — the +1 inset the
    // rects wear)
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    let (ox, oy) = (panel.0 + 17 + 1, panel.1 + 17 + 1);
    let cx = ox + 8;
    let cy = oy + 8;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((cx, cy));
        app.render(&mut fb_b, &mut a);
    }
    // THE CELL'S OWN TINT (the empty seat the highlight paints —
    // the exact two-layer math the sprite alphas answer)
    let a0 = pixel(&fb_a, cx, cy);
    let expected = blend(blend(a0, 96.0 / 255.0), 32.0 / 255.0);
    assert_eq!(
        pixel(&fb_b, cx, cy),
        expected,
        "the two-layer tint the exact math wears (back 96 + front 32)"
    );
    assert_ne!(
        a0, expected,
        "the tint the paint speaks (the highlight the canvas moves)"
    );
    // THE BORDER RING (the 1px inside the hover walk's band the
    // sprite's own 4px transparent margin keeps — the cell-only
    // tint the geometry pins)
    assert_eq!(
        pixel(&fb_a, ox - 1, cy),
        pixel(&fb_b, ox - 1, cy),
        "the border ring the sprite's margin keeps (no paint)"
    );
    // THE FAR FACE (the centered title's own reach the paint never
    // wears — the static band the point keeps, the centered row the
    // +150 face the text never spans)
    assert_eq!(
        pixel(&fb_a, panel.0 + 150, panel.1 + 6),
        pixel(&fb_b, panel.0 + 150, panel.1 + 6),
        "the far face the same verdict keeps"
    );
}

/// THE BREWING STAND'S OWN CLICK ROUND (the offline sim's own walk —
/// the pickup the five banks, the place-back the cursor returns, the
/// quick-move the shift lane speaks, the close flush the edited five
/// the store banks): the REAL click walk the offline door wears.
#[test]
fn the_offline_brewing_the_click_the_round_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::BREWING_STAND);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.inventory.items[0] = (7, 10); // the hotbar head the real stack keeps
    app.open_brewing_for(cell);
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    // THE SEAT COORDS (the scoped lookup the mutable walk keeps — the
    // rects the render laid the click lanes read)
    let (h32x, h32y) = {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 32)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    let (fx, fy) = {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 4)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE PICKUP LANE (the hotbar head's own click the cursor banks —
    // the player's own row the REAL inventory wears: the menu 32 the
    // items 0 answer, the stack the cursor carries)
    app.click_brewing_slot(h32x + 8, h32y + 8, 0);
    assert_eq!(app.brewing_carried, (7, 10), "the pickup the cursor banks");
    assert_eq!(
        app.inventory.items[0],
        (0, 0),
        "the source row the empty wears"
    );
    assert_eq!(
        app.brewing_slots[4],
        (0, 0),
        "the fuel cell the untouched empty wears"
    );
    // THE PLACE-BACK (the fuel seat's own click the five banks — the
    // carried the empty wears, the mirror rows the sync re-speaks)
    app.click_brewing_slot(fx + 8, fy + 8, 0);
    assert_eq!(
        app.brewing_carried,
        (0, 0),
        "the place-back the cursor clears"
    );
    assert_eq!(
        app.brewing_slots[4],
        (7, 10),
        "the fuel cell the stack banks"
    );
    // THE QUICK-MOVE LANE (the shift-click the mirror main cell the
    // five banks — the whole stack the OTHER section moves)
    app.inventory.items[9] = (8, 3);
    app.brewing_mirror_sync_from_inventory();
    let (sx5, sy5) = {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 5)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    app.shift_down = true;
    app.click_brewing_slot(sx5 + 8, sy5 + 8, 0);
    app.shift_down = false;
    assert_eq!(
        app.brewing_slots[0],
        (8, 3),
        "the quick-move the five banks"
    );
    assert_eq!(
        app.inventory.items[9],
        (0, 0),
        "the source row the empty wears"
    );
    // THE CLOSE FLUSH (the edited five the store banks — the reopen
    // the stored stacks load)
    app.close_brewing(ScreenKind::World);
    assert_eq!(app.screen, ScreenKind::World, "the close the world wears");
    let store = app.brewing_stores.get(&cell).unwrap();
    assert_eq!(store[0], (8, 3), "the flushed five the edit banks");
    assert_eq!(store[4], (7, 10), "the flushed five the fuel banks");
    // THE REOPEN (the stored stacks the projection loads — the
    // persistence the same map speaks)
    app.open_brewing_for(cell);
    assert_eq!(app.brewing_slots[0], (8, 3), "the reopen the edit loads");
    assert_eq!(app.brewing_slots[4], (7, 10), "the reopen the fuel loads");
}

/// THE ENCHANTING TABLE'S OWN DOOR SEAT (the autopush-333 round — the
/// 26.2 ctor's own literal seats the bytecode probe banks): the
/// transient-pair walk the origin answers, the item/lapis pair the
/// (15, 47)/(35, 47) seats the ctor's bipush stream pins, the NO-GATE
/// law the family banks, the wrong screen the None answers. The REAL
/// walk the offline door wears — the headless frame the REAL rects
/// the render lays.
#[test]
fn the_offline_enchanting_the_door_the_seats_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::ENCHANTING_TABLE);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the empty hand the door's own lane keeps
    }
    app.open_enchanting_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Enchanting,
        "the enchanting door the kind wears"
    );
    assert_eq!(app.enchant_slots.len(), 2, "the fresh pair the open lays");
    assert_eq!(
        app.enchant_mirror.len(),
        36,
        "the mirror rows the sync keeps"
    );
    // THE REAL RECTS THE RENDER BANKS (one headless frame the mouse
    // never tracks — the render's own walk the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    assert_eq!(
        app.enchant_panel, panel,
        "the centered panel the canvas keeps"
    );
    let seat = |slot: i16| -> (i32, i32) {
        app.enchant_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE 26.2 CTOR'S OWN PITCH (the bipush stream the probe spoke:
    // the item (15, 47), the lapis (35, 47) — the +20 x-pitch the
    // same row wears)
    let (i0x, i0y) = seat(0);
    let (l1x, l1y) = seat(1);
    assert_eq!(
        (l1x - i0x, l1y - i0y),
        (20, 0),
        "the item/lapis pair the ctor pitch banks"
    );
    // THE PAIR'S OWN ANSWERS (each seat + the mirror head + the hotbar
    // tail the origin answers — the ONE walk the clicks share)
    assert_eq!(
        app.enchant_highlight_origin(i0x + 8, i0y + 8),
        Some((i0x, i0y)),
        "the item seat the origin answers"
    );
    assert_eq!(
        app.enchant_highlight_origin(l1x + 8, l1y + 8),
        Some((l1x, l1y)),
        "the lapis seat the origin answers"
    );
    let (mx, my) = seat(2);
    assert_eq!(
        app.enchant_highlight_origin(mx + 8, my + 8),
        Some((mx, my)),
        "the mirror head the origin answers"
    );
    let (hx, hy) = seat(37);
    assert_eq!(
        app.enchant_highlight_origin(hx + 8, hy + 8),
        Some((hx, hy)),
        "the hotbar tail the origin answers"
    );
    // THE NO-GATE LAW (the family's own verdict the enchanting screen
    // re-wears — the carried stack the wall NEVER speaks, the EMPTY
    // seats the plain answer keeps)
    app.enchant_carried = (7, 3);
    assert_eq!(
        app.enchant_highlight_origin(i0x + 8, i0y + 8),
        Some((i0x, i0y)),
        "the carried stack the highlight never refuses"
    );
    app.enchant_carried = (0, 0);
    // THE CELL'S OWN EDGES (the strict 16x16 the walk answers)
    assert_eq!(
        app.enchant_highlight_origin(i0x + 16, i0y + 8),
        None,
        "the east edge the walk refuses"
    );
    assert_eq!(
        app.enchant_highlight_origin(i0x - 1, i0y + 8),
        None,
        "the west edge the walk refuses"
    );
    // THE BLANK FACE (the no-seat walk the None answers)
    assert_eq!(
        app.enchant_highlight_origin(panel.0 - 20, panel.1 - 20),
        None,
        "the outside face the None speaks"
    );
    // THE WRONG SCREEN (the world view the highlight never wears)
    app.close_enchanting(ScreenKind::World);
    assert_eq!(
        app.enchant_highlight_origin(i0x + 8, i0y + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE ENCHANTING TABLE'S OWN HIGHLIGHT PAINT (the REAL render walk —
/// the 320x180 headless canvas the enchanting screen renders, the
/// same-frame pair the mouse the only delta wears): the empty lapis
/// seat the back-96 + the front-32 the EXACT alpha math answers (the
/// two-layer order the composite pins), the transparent border ring
/// the 24x24 sprite's own 4px margin keeps, the no-mouse pass the
/// paint never serves.
#[test]
fn the_offline_enchanting_the_paint_the_cell_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::ENCHANTING_TABLE);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_enchanting_for(cell);
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let blend = |dst: u8, alpha: f32| -> u8 {
        let d = dst as f32 / 255.0;
        let v = d * (1.0 - alpha) + 1.0 * alpha;
        ((v.clamp(0.0, 1.0)) * 255.0 + 0.5) as u8
    };
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_a, &mut a);
    }
    // THE REAL GEOMETRY THE RENDER BANKS (the lapis seat the (35, 47)
    // seat the 26.2 ctor's own literal speaks — the +1 inset the
    // rects wear)
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    let (ox, oy) = (panel.0 + 35 + 1, panel.1 + 47 + 1);
    let cx = ox + 8;
    let cy = oy + 8;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((cx, cy));
        app.render(&mut fb_b, &mut a);
    }
    // THE CELL'S OWN TINT (the empty seat the highlight paints —
    // the exact two-layer math the sprite alphas answer)
    let a0 = pixel(&fb_a, cx, cy);
    let expected = blend(blend(a0, 96.0 / 255.0), 32.0 / 255.0);
    assert_eq!(
        pixel(&fb_b, cx, cy),
        expected,
        "the two-layer tint the exact math wears (back 96 + front 32)"
    );
    assert_ne!(
        a0, expected,
        "the tint the paint speaks (the highlight the canvas moves)"
    );
    // THE BORDER RING (the 1px outside the hover walk's band the
    // sprite's own 4px transparent margin keeps — the cell-only
    // tint the geometry pins)
    assert_eq!(
        pixel(&fb_a, ox - 1, cy),
        pixel(&fb_b, ox - 1, cy),
        "the border ring the sprite's margin keeps (no paint)"
    );
    // THE FAR FACE (the centered title's own reach the paint never
    // wears — the static band the point keeps, the centered row the
    // +150 face the text never spans)
    assert_eq!(
        pixel(&fb_a, panel.0 + 150, panel.1 + 6),
        pixel(&fb_b, panel.0 + 150, panel.1 + 6),
        "the far face the same verdict keeps"
    );
}

/// THE ENCHANTING TABLE'S OWN CLICK ROUND (the offline sim's own walk —
/// the pickup the mirror banks, the place the lapis seat keeps, the
/// quick-move the shift lane speaks, the close the TRANSIENT pair the
/// ride-home walk returns [NO store the vanilla menu never banks],
/// the reopen the FRESH pair lays): the REAL click walk the offline
/// door wears.
#[test]
fn the_offline_enchanting_the_click_the_round_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::ENCHANTING_TABLE);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.inventory.items[9] = (7, 10); // the main grid head the real stack keeps
    app.open_enchanting_for(cell);
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    // THE SEAT COORDS (the scoped lookup the mutable walk keeps — the
    // rects the render laid the click lanes read)
    let (m2x, m2y) = {
        app.enchant_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 2)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    let (l1x, l1y) = {
        app.enchant_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 1)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE PICKUP LANE (the mirror head's own click the cursor banks —
    // the player's own row the REAL inventory wears: the menu 2 the
    // items 9 answer, the stack the cursor carries)
    app.click_enchanting_slot(m2x + 8, m2y + 8, 0);
    assert_eq!(app.enchant_carried, (7, 10), "the pickup the cursor banks");
    assert_eq!(
        app.inventory.items[9],
        (0, 0),
        "the source row the empty wears"
    );
    assert_eq!(
        app.enchant_slots[1],
        (0, 0),
        "the lapis seat the untouched empty wears"
    );
    // THE PLACE LANE (the lapis seat's own click the pair banks — the
    // carried the empty wears, the mirror rows the sync re-speaks)
    app.click_enchanting_slot(l1x + 8, l1y + 8, 0);
    assert_eq!(app.enchant_carried, (0, 0), "the place the cursor clears");
    assert_eq!(
        app.enchant_slots[1],
        (7, 10),
        "the lapis seat the stack banks"
    );
    // THE QUICK-MOVE LANE (the shift-click the mirror main cell the
    // pair banks — the whole stack the OTHER section moves)
    app.inventory.items[9] = (8, 3);
    app.enchant_mirror_sync_from_inventory();
    app.shift_down = true;
    app.click_enchanting_slot(m2x + 8, m2y + 8, 0);
    app.shift_down = false;
    assert_eq!(
        app.enchant_slots[0],
        (8, 3),
        "the quick-move the pair banks"
    );
    assert_eq!(
        app.inventory.items[9],
        (0, 0),
        "the source row the empty wears"
    );
    // THE CLOSE RIDE-HOME (the TRANSIENT pair the walk returns — the
    // NO store the vanilla menu never banks, the held stacks the
    // inventory re-keeps, the seats the empty wear)
    app.close_enchanting(ScreenKind::World);
    assert_eq!(app.screen, ScreenKind::World, "the close the world wears");
    let total: i32 = app
        .inventory
        .items
        .iter()
        .filter(|it| it.0 == 7)
        .map(|it| it.1)
        .sum();
    assert_eq!(total, 10, "the lapis seat's stack the ride home keeps");
    let total8: i32 = app
        .inventory
        .items
        .iter()
        .filter(|it| it.0 == 8)
        .map(|it| it.1)
        .sum();
    assert_eq!(total8, 3, "the item seat's stack the ride home keeps");
    assert!(
        app.enchant_slots.iter().all(|it| it.1 == 0),
        "the transient seats the empty wear"
    );
    // THE REOPEN (the FRESH pair the open lays — the transient law the
    // chest's persist law never wears)
    app.open_enchanting_for(cell);
    assert_eq!(
        app.enchant_slots.iter().all(|it| it.1 == 0),
        true,
        "the fresh pair the transient law lays"
    );
}

#[test]
fn the_enchanting_table_the_power_scan_the_ring_wears() {
    // THE SPELL WALK'S OWN RING (the dev-195 round — the
    // EnchantingTableBlock clinit's betweenClosedStream(-2,0,-2, 2,1,2)
    // box the lambda$static$0 abs==2 filter keeps, the
    // isValidBookShelf pair the provider/transmitter tags the
    // midpoint (x/2, y, z/2) gap answers)
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let t = (c, surface, c);
    // THE FLAT RING'S OWN STAGE (the terrain the region's own AIR
    // clear silences — the scan the constructed ring alone reads)
    let w = app.offline_world.as_mut().unwrap();
    for y in surface - 1..=surface + 2 {
        for x in c - 3..=c + 3 {
            for z in c - 3..=c + 3 {
                w.set(x, y, z, crate::world::AIR);
            }
        }
    }
    w.set(t.0, t.1, t.2, crate::world::ENCHANTING_TABLE);
    // THE EMPTY RING (the fresh world the zero keeps)
    assert_eq!(
        app.enchant_power_scan(t),
        0,
        "the empty ring the zero power keeps"
    );
    // THE SINGLE SHELF (the (2,0,0) offset the provider arm counts —
    // the (1,0,0) midpoint the air gap the transmitter serves)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(t.0 + 2, t.1, t.2, crate::world::BOOKSHELF);
    assert_eq!(
        app.enchant_power_scan(t),
        1,
        "the single shelf the gap serves"
    );
    // THE SOLID GAP'S OWN REFUSAL (the transmitter arm the stone cell
    // the provider's own count refuses — the classic air-gap law)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(t.0 + 1, t.1, t.2, crate::world::STONE);
    assert_eq!(
        app.enchant_power_scan(t),
        0,
        "the solid gap the transmitter refuses"
    );
    app.offline_world
        .as_mut()
        .unwrap()
        .set(t.0 + 1, t.1, t.2, crate::world::AIR);
    // THE PLANT'S OWN ARM (the #replaceable tag the offline plant
    // family answers — the grass cell the gap the count keeps)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(t.0 + 1, t.1, t.2, crate::world::SHORT_GRASS);
    assert_eq!(
        app.enchant_power_scan(t),
        1,
        "the plant gap the replaceable arm keeps"
    );
    app.offline_world
        .as_mut()
        .unwrap()
        .set(t.0 + 1, t.1, t.2, crate::world::AIR);
    // THE SECOND LAYER (the (2,1,0) offset the y=1 layer speaks — the
    // stack the two shelves the count answers)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(t.0 + 2, t.1 + 1, t.2, crate::world::BOOKSHELF);
    assert_eq!(
        app.enchant_power_scan(t),
        2,
        "the stacked shelves the two layers speak"
    );
    // THE INNER 3x3'S OWN EXCLUSION (the lambda$static$0 filter the
    // |x|==2 || |z|==2 ring alone scans as PROVIDERS — the inner
    // shelf the count never adds [the 2 stays 2-as-providers], yet
    // its cell the (1,0,0) MIDPOINT GAP the (2,0,0) shelf wears
    // blocks — the vanilla law the count 2 -> 1 the gap-blocking
    // speaks, the provider set the inner cell the +1 never speaks)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(t.0 + 1, t.1, t.2, crate::world::BOOKSHELF);
    assert_eq!(
        app.enchant_power_scan(t),
        1,
        "the inner shelf the gap it blocks, the provider count it never adds"
    );
    // THE CLEAR'S OWN RESTORE (the inner shelf the AIR returns — the
    // stacked pair the 2 the fresh gap serves)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(t.0 + 1, t.1, t.2, crate::world::AIR);
    assert_eq!(
        app.enchant_power_scan(t),
        2,
        "the cleared gap the stacked pair restores"
    );
    // THE CORNER'S OWN MIDPOINT (the (2,0,2) offset the (1,0,1) gap
    // the integer division halves — the diagonal shelf the corner
    // arm counts)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(t.0 + 2, t.1, t.2 + 2, crate::world::BOOKSHELF);
    assert_eq!(
        app.enchant_power_scan(t),
        3,
        "the corner shelf the diagonal gap serves"
    );
}

#[test]
fn the_enchanting_table_the_cost_roll_the_formula_wears() {
    // THE SPELL WALK'S OWN ROLL (the dev-195 round — javap the 26.2
    // EnchantmentHelper.getEnchantmentCost pc 19-89 the exact walk:
    // the 15 cap the bipush speaks, the base the
    // nextInt(8) + 1 + (power >> 1) + nextInt(power + 1) roll keeps,
    // the slot arms the three splits bank, the clamp the cost < i+1
    // zero the lambda's own branch wears)
    let roll_row = |power: i32| -> [i32; 3] {
        let mut rng = crate::noise::VanillaRandom::new(0);
        let mut costs = [0i32; 3];
        for (i, c) in costs.iter_mut().enumerate() {
            let mut v = App::enchant_cost_roll(i as i32, power, &mut rng);
            if v < i as i32 + 1 {
                v = 0;
            }
            *c = v;
        }
        costs
    };
    // THE SEED-0 LAWS (the python LCG twin the hand walk banks — the
    // ONE continuous rng the vanilla lambda's own walk keeps: the
    // power-0 bases 6/2/6 the rows [2, 2, 6] speak, the power-15
    // bases 25/4/26 the cap the rows answer)
    assert_eq!(
        roll_row(0),
        [2, 2, 6],
        "the empty ring the weak enchants wear"
    );
    assert_eq!(
        roll_row(15),
        [8, 13, 30],
        "the full ring the strong enchants wear"
    );
    // THE CAP'S OWN RING (the power 32 the 15 cap the scan's own
    // overflow never speaks — the roll the capped power alone reads)
    assert_eq!(
        roll_row(32),
        roll_row(15),
        "the overflow power the cap folds"
    );
    // THE REFRESH'S OWN GATES (the empty item slot the zero costs the
    // slotsChanged's own isEmpty gate keeps — the powered world the
    // gate alone answers)
    let mut app = App::new(3);
    app.enter_world();
    app.enchant_slots = vec![(0, 0), (0, 0)];
    app.enchanting_table_pos = Some((0, 0, 0));
    app.enchant_costs_refresh();
    assert_eq!(
        app.enchanting_costs,
        [0, 0, 0],
        "the empty item the zero costs keep"
    );
    assert_eq!(app.enchanting_power, 0, "the gate the power zero keeps");
    // THE ITEM'S OWN ROLL (the non-empty stack the presence arm the
    // three rolls the fresh open lays)
    app.enchant_slots[0] = (7, 1);
    app.enchant_costs_refresh();
    assert_eq!(
        app.enchanting_costs,
        [2, 2, 6],
        "the item seat the seed-0 roll wears"
    );
}

#[test]
fn the_enchanting_table_the_rows_the_frame_and_number_wear() {
    // THE SPELL WALK'S OWN ROWS (the dev-195 round — the
    // extractBackground walk the offline canvas wears: the cost==0
    // gate the DISABLED frame alone answers, the enabled row the
    // frame + the level sprite + the right-aligned green number the
    // (166 - width, 23 + 19*i) seat banks)
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    let w = app.offline_world.as_mut().unwrap();
    for y in surface..=surface + 2 {
        for x in c - 3..=c + 3 {
            for z in c - 3..=c + 3 {
                w.set(x, y, z, crate::world::AIR);
            }
        }
    }
    w.set(c, surface, c, crate::world::ENCHANTING_TABLE);
    w.set(c + 2, surface, c, crate::world::BOOKSHELF);
    // THE POWER-0 PAIR'S OWN DISABLED LANE (the empty item the zero
    // costs keep — the three rows the disabled frame alone draws)
    app.open_enchanting_for(cell);
    assert_eq!(app.enchanting_costs, [0, 0, 0], "the empty pair the gate");
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    let rgb = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> [u8; 3] {
        let o = ((y as usize) * fb.w + x as usize) * 3;
        [fb.color[o], fb.color[o + 1], fb.color[o + 2]]
    };
    let near = |p: [u8; 3], r: u8, g: u8, b: u8| -> bool {
        (p[0] as i32 - r as i32).abs() <= 4
            && (p[1] as i32 - g as i32).abs() <= 4
            && (p[2] as i32 - b as i32).abs() <= 4
    };
    // THE DISABLED FRAME'S OWN CORNER (the (0,0) pixel the decode
    // pins — the (33,29,23) master the row's border speaks)
    let f0 = rgb(&fb, panel.0 + 60, panel.1 + 14);
    assert!(
        near(f0, 33, 29, 23),
        "the disabled frame the row-0 corner wears, got {f0:?}"
    );
    // THE LEVEL SEAT'S OWN ABSENCE (the cost==0 gate the level sprite
    // never draws — the (0,0) transparent cell the bg through speaks)
    let lv = rgb(&fb, panel.0 + 61, panel.1 + 15);
    assert!(
        !near(lv, 245, 255, 143),
        "the level seat the cost-0 gate keeps empty, got {lv:?}"
    );
    // THE ITEM'S OWN ROLL (the place the refresh the rolls lay — the
    // costs the seed-0 law wears; the survival xp 0 the rows the
    // UNAFFORDABLE lane keeps — the disabled frames + the dark
    // numbers the vanilla's own level gate speaks)
    app.enchant_slots[0] = (7, 1);
    app.enchant_costs_refresh();
    assert_eq!(
        app.enchanting_costs,
        [2, 2, 6],
        "the one-shelf roll the seed-0 law wears"
    );
    let mut fb = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    // THE LEVEL-SHORT LANE (the vanilla's own gate the xp-0 survival
    // keeps: the disabled frame + the disabled level sprite + the
    // dark number the row speaks)
    let f1 = rgb(&fb, panel.0 + 60, panel.1 + 14);
    assert!(
        near(f1, 33, 29, 23),
        "the xp-short row the disabled frame wears, got {f1:?}"
    );
    let lv_short = rgb(&fb, panel.0 + 61 + 12, panel.1 + 15 + 3);
    assert!(
        near(lv_short, 71, 53, 47),
        "the disabled level sprite the glyph seat wears, got {lv_short:?}"
    );
    let dark_seat = |fb: &crate::raster::Framebuf, row_y: i32, digit: &str| -> bool {
        let nx = panel.0 + 166 - crate::gui::font_text_width(digit) as i32;
        for dy in 0..8 {
            for dx in -1..8 {
                let p = rgb(fb, nx + dx, row_y + dy);
                if near(p, 64, 127, 16) {
                    return true;
                }
            }
        }
        false
    };
    assert!(
        dark_seat(&fb, panel.1 + 23, "2"),
        "the xp-short number the dark seat wears"
    );
    // THE LEVEL'S OWN LIFT (the xp 30 + the lapis stack the rows the
    // affordable lane wears — the vanilla's own double gate the
    // lapis >= i+1 AND the xp >= cost keeps, the frames the enabled
    // master + the level sprite + the green number speak)
    app.xp.level = 30;
    app.enchant_slots[1] = (7, 3);
    let mut fb = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    // THE ENABLED FRAME'S OWN CORNER (the (224,202,159) master the
    // affordable row's border speaks)
    let f1 = rgb(&fb, panel.0 + 60, panel.1 + 14);
    assert!(
        near(f1, 224, 202, 159),
        "the affordable row the enabled frame wears, got {f1:?}"
    );
    // THE LEVEL SPRITE'S OWN GLYPH (the (6,3) pixel the decode pins —
    // the level_1 master the row's sprite seat speaks)
    let lv = rgb(&fb, panel.0 + 61 + 6, panel.1 + 15 + 3);
    assert!(
        near(lv, 245, 255, 143),
        "the level sprite the glyph seat wears, got {lv:?}"
    );
    // THE NUMBER'S OWN SEAT (the right-aligned (166 - width,
    // 23 + 19*i) the font walk banks — the green #80ff20 the
    // affordable color speaks; the glyph window the inked-cell scan
    // the exact top-left column the font's own advance never pins)
    let green_seat = |fb: &crate::raster::Framebuf, row_y: i32, digit: &str| -> bool {
        let nx = panel.0 + 166 - crate::gui::font_text_width(digit) as i32;
        for dy in 0..8 {
            for dx in -1..8 {
                let p = rgb(fb, nx + dx, row_y + dy);
                if near(p, 128, 255, 32) {
                    return true;
                }
            }
        }
        false
    };
    assert!(
        green_seat(&fb, panel.1 + 23, "2"),
        "the affordable number the green seat wears"
    );
}

#[test]
fn the_enchanting_table_the_labels_the_sga_glyphs_wear() {
    // THE SPELL WALK'S OWN GIBBERISH LABEL (the dev-336 round — the
    // EnchantmentNames.getRandomName walk the SGA alt-font lane
    // renders: the seed-0 rolls the per-frame re-seed keeps, the
    // (80, 16 + 19*i) seat the 86-minus-number-width trim banks, the
    // colors the branches speak)
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    let w = app.offline_world.as_mut().unwrap();
    for y in surface..=surface + 2 {
        for x in c - 3..=c + 3 {
            for z in c - 3..=c + 3 {
                w.set(x, y, z, crate::world::AIR);
            }
        }
    }
    w.set(c, surface, c, crate::world::ENCHANTING_TABLE);
    w.set(c + 2, surface, c, crate::world::BOOKSHELF);
    // THE DOOR'S OWN OPEN (the empty pair the cost-0 gate keeps — the
    // labels the silence the gate speaks)
    app.open_enchanting_for(cell);
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    let rgb = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> [u8; 3] {
        let o = ((y as usize) * fb.w + x as usize) * 3;
        [fb.color[o], fb.color[o + 1], fb.color[o + 2]]
    };
    let has_rgb = |fb: &crate::raster::Framebuf, x0: i32, y0: i32, target: [u8; 3]| -> bool {
        for dy in 0..9 {
            for dx in 0..60 {
                let p = rgb(fb, x0 + dx, y0 + dy);
                if (p[0] as i32 - target[0] as i32).abs() <= 6
                    && (p[1] as i32 - target[1] as i32).abs() <= 6
                    && (p[2] as i32 - target[2] as i32).abs() <= 6
                {
                    return true;
                }
            }
        }
        false
    };
    // THE COST-0 GATE'S OWN SILENCE (the empty pair the labels never
    // draws — the label seat the bg through speaks)
    assert!(
        !has_rgb(&fb, panel.0 + 80, panel.1 + 15, [0x68, 0x5e, 0x4a]),
        "the cost-0 row the label seat keeps silent"
    );
    assert!(
        !has_rgb(&fb, panel.0 + 80, panel.1 + 15, [0x34, 0x2f, 0x25]),
        "the cost-0 row the dim ink never wears"
    );
    // THE XP-SHORT LANE'S OWN DIM (the unaffordable rows the
    // (color & 0xfefefe) >> 1 halve the classic law wears — the
    // #342f25 the label ink speaks)
    app.enchant_slots[0] = (7, 1);
    app.enchant_costs_refresh();
    let mut fb = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    assert!(
        has_rgb(&fb, panel.0 + 80, panel.1 + 15, [0x34, 0x2f, 0x25]),
        "the xp-short row the dim label ink wears"
    );
    // THE AFFORDABLE LANE'S OWN COLOR (the brown-gray #685e4a the
    // -9937334 the enabled rows speak — the hovered white the mouse
    // the headless law never rides)
    app.xp.level = 30;
    app.enchant_slots[1] = (7, 3);
    let mut fb = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    assert!(
        has_rgb(&fb, panel.0 + 80, panel.1 + 15, [0x68, 0x5e, 0x4a]),
        "the affordable row the label ink wears"
    );
    // THE ROLL'S OWN DETERMINISM (the seed-0 strings the python LCG
    // twin banks — the ONE continuous stream the three rows consume)
    let mut rng = crate::noise::VanillaRandom::new(0);
    assert_eq!(
        App::enchant_label_roll(&mut rng),
        "scrolls humanoid fire earth",
        "the row-0 roll the seed-0 law wears"
    );
    assert_eq!(
        App::enchant_label_roll(&mut rng),
        "elder niktu bless",
        "the row-1 roll the stream keeps"
    );
    assert_eq!(
        App::enchant_label_roll(&mut rng),
        "cold spirit imbue curse",
        "the row-2 roll the stream keeps"
    );
}

#[test]
fn the_enchanting_table_the_click_enchant_walk_wears() {
    // THE CLICK-ENCHANT WALK (the autopush-337 round — the 26.2
    // EnchantmentMenu.clickMenuButton the exact guards the spend the
    // seed draw the re-roll the USE voice wears; the enchant
    // APPLICATION the item-stack's own data-model round keeps)
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    let w = app.offline_world.as_mut().unwrap();
    for y in surface..=surface + 2 {
        for x in c - 3..=c + 3 {
            for z in c - 3..=c + 3 {
                w.set(x, y, z, crate::world::AIR);
            }
        }
    }
    w.set(c, surface, c, crate::world::ENCHANTING_TABLE);
    w.set(c + 2, surface, c, crate::world::BOOKSHELF);
    app.open_enchanting_for(cell);
    app.enchant_slots[0] = (7, 1);
    app.enchant_slots[1] = (7, 3);
    app.xp.level = 30;
    app.enchant_costs_refresh();
    // THE POWER-1 ROWS' OWN LIVENESS (the slot-0 roll the
    // (base / 3).max(1) the NEVER-dead answer keeps — the clamp the
    // v < 1 never fires, the row-0 the always-enabled law wears)
    assert!(
        app.enchanting_costs[0] >= 1,
        "the row-0 cost the floor law keeps"
    );
    // THE ID GATE'S OWN FALSE (the out-of-range id the
    // costs.length arm keeps)
    assert!(
        !app.click_enchant_option(3),
        "the id-3 press the out-of-range false wears"
    );
    // THE XP GUARD'S OWN FALSE (the level-short the max-of-two law
    // keeps — the level below the cost the same false wears)
    let c0 = app.enchanting_costs[0];
    app.xp.level = c0 - 1;
    assert!(
        !app.click_enchant_option(0),
        "the xp-short click the false answer wears"
    );
    // THE LAPIS GUARD'S OWN FALSE (the empty slot the
    // infinite-materials arm the survival's false keeps)
    app.xp.level = 30;
    app.enchant_slots[1] = (0, 0);
    assert!(
        !app.click_enchant_option(0),
        "the lapis-empty click the false answer wears"
    );
    // THE ITEM GUARD'S OWN FALSE (the empty item the pc 83-96 pair
    // keeps — the cost row the liveness the item presence gates)
    app.enchant_slots[0] = (0, 0);
    app.enchant_slots[1] = (7, 3);
    assert!(
        !app.click_enchant_option(0),
        "the item-empty click the false answer wears"
    );
    // THE HAPPY WALK (the guards the pass the spend the exact order
    // wears — the xp + the seed draw the onEnchantmentPerformed
    // speaks, the lapis the consume shrink keeps, the re-roll the
    // slotsChanged answers; the dev-196 round the ENCHANTABLE item
    // the walk wears — the 337 round's own andesite the empty gate
    // the honest stop now answers, the diamond sword the list the
    // live walk serves)
    let sword = mc_registry::item_id_by_name("diamond_sword").unwrap();
    app.enchant_slots[0] = (sword, 1);
    let seed_before = app.enchant_seed;
    assert_eq!(seed_before, 0, "the boot seed the ctor's own 0 keeps");
    assert!(
        app.click_enchant_option(0),
        "the happy click the true answer wears"
    );
    assert_eq!(
        app.enchant_slots[1],
        (7, 2),
        "the lapis the consume-1 shrink wears"
    );
    assert_eq!(
        app.xp.level, 29,
        "the xp the level-1 spend the onEnchantmentPerformed law keeps"
    );
    // THE ENCHANT APPLICATION (the dev-196 round — the instances the
    // store's own rows wear: the seed-0 row-0 twin the
    // [sharpness 1] list answers, the registry index the
    // alphabetical order keeps)
    assert_eq!(
        app.enchant_comps[0],
        vec![(33, 1)],
        "the seat-0 store the sharpness-I instance wears"
    );
    // THE SEED DRAW (the player-random stand-in the full next(32)
    // draw answers — the enchantmentSeed the bump the bytecode
    // banks)
    let mut seed_rng = crate::noise::VanillaRandom::new(0);
    let s1 = seed_rng.next_unbounded();
    assert_eq!(
        app.enchant_seed, s1,
        "the seed the first player-random draw wears"
    );
    // THE COSTS RE-ROLL (the slotsChanged the fresh seed the
    // setSeed walk answers — the three rolls the new stream consumes)
    let power = app.enchanting_power;
    let mut roll = crate::noise::VanillaRandom::new(s1 as i64);
    for (j, got) in app.enchanting_costs.iter().enumerate() {
        let mut want = App::enchant_cost_roll(j as i32, power, &mut roll);
        if want < j as i32 + 1 {
            want = 0;
        }
        assert_eq!(
            *got, want,
            "the row-{j} re-roll the fresh-seed stream wears"
        );
    }
    // THE USE VOICE (the lambda tail the playSound walk the events
    // log answers — the registry name the sounds.json row keeps, the
    // legacy block.enchantment_table.use the table wears)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.contains("block.enchantment_table.use")),
        "the use voice the enchant click's tail wears"
    );
    // THE LAPIS-EXHAUST LAW (the count-1 pair the consume the
    // setItem(1, EMPTY) the empty-pair answer wears — the row-0 cost
    // the floor law the second click the liveness keeps; the
    // enchanted sword the fresh seed the fresh list serves)
    app.enchant_slots[1] = (7, 1);
    assert!(
        app.click_enchant_option(0),
        "the exhaust click the true answer wears"
    );
    assert_eq!(
        app.enchant_slots[1],
        (0, 0),
        "the spent slot the empty-pair law wears"
    );
    // THE FRESH LIST (the seed-1 stream the pick answers — the
    // sharpness-I instance the s1 row-0 twin wears)
    assert_eq!(
        app.enchant_comps[0],
        vec![(33, 1)],
        "the fresh-seed store the sharpness-I instance wears"
    );
    let s2 = seed_rng.next_unbounded();
    assert_eq!(
        app.enchant_seed, s2,
        "the seed the second draw the stream keeps"
    );
    // THE ROW WIDGET'S OWN PRECEDENCE (the on_press the option rows
    // the slot walk BEFORE consumes — the rendered panel the hit box
    // the row-0 rect answers)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = app.enchant_panel;
    assert_ne!(panel, (0, 0), "the render the panel origin lays");
    // the lapis the press walk the guard the supply answers (the
    // exhaust walk the empty pair the fresh stack the seat lays)
    app.enchant_slots[1] = (7, 3);
    let level_before = app.xp.level;
    let seed_before_press = app.enchant_seed;
    app.on_press(panel.0 + 60 + 5, panel.1 + 14 + 5);
    assert_eq!(
        app.xp.level,
        level_before - 1,
        "the row-0 press the click walk the xp spend wears"
    );
    assert_ne!(
        app.enchant_seed, seed_before_press,
        "the row-0 press the seed bump wears"
    );
    // THE SLOT-RECT'S OWN NON-OVERLAP (the rows x 60..168 the slots
    // x 15..51 the gutter keeps — the item seat the press the row
    // walk never steals)
    let item_seat = app.enchant_slot_rects[0];
    assert!(
        item_seat.0 + 16 <= panel.0 + 60,
        "the item seat the row-0 rect the x-gutter keeps"
    );
}

/// THE ITEM-STACK'S OWN ENCHANTMENT ROUND (the dev-196 round — the
/// registry the jar's own 43 definitions the alphabetical order
/// keeps, the walks the javap evidence banks:
/// scripts/jar_work/enchant_select_196.txt the stream speaks): the
/// spot rows the exact numbers answer, the table pool the 36
/// members wear, the compatibility the exclusive sets speak, the
/// families the resolved trees answer, the enchantability the
/// material rows keep.
#[test]
fn the_item_stack_the_enchantment_registry_wears() {
    use mc_registry::generated::enchantments as ench;
    // THE COUNT (the 43 definitions the jar's own data lane serves)
    assert_eq!(ench::ENCHANTMENTS.len(), 43, "the 43 rows the registry keeps");
    assert_eq!(ench::TABLE_MEMBERS.len(), 36, "the 36 table members the non_treasure tag keeps");
    // THE SPOT ROWS (the jar's own numbers the json files bank)
    let sharp = ench::enchantment_index("sharpness").unwrap();
    assert_eq!(sharp, 33, "the alphabetical order the registry id keeps");
    assert_eq!(ench::ENCHANTMENTS[sharp].weight, 10, "the weight the json row keeps");
    assert_eq!(ench::ENCHANTMENTS[sharp].max_level, 5, "the max level the json row keeps");
    assert_eq!(ench::ENCHANTMENTS[sharp].min_cost, (1, 11), "the min curve the json row keeps");
    assert_eq!(ench::ENCHANTMENTS[sharp].max_cost, (21, 11), "the max curve the json row keeps");
    assert_eq!(ench::ENCHANTMENTS[sharp].primary, Some("melee_weapon"), "the primary family the json row keeps");
    let prot = ench::enchantment_index("protection").unwrap();
    assert_eq!(prot, 28, "the protection id the alphabetical order keeps");
    assert_eq!(ench::ENCHANTMENTS[prot].min_cost, (1, 11), "the protection min curve keeps");
    assert_eq!(ench::ENCHANTMENTS[prot].max_cost, (12, 11), "the protection max curve keeps");
    assert_eq!(ench::ENCHANTMENTS[prot].exclusive, Some("armor"), "the exclusive tag keeps");
    let silk = ench::enchantment_index("silk_touch").unwrap();
    assert_eq!(ench::ENCHANTMENTS[silk].weight, 1, "the very-rare weight the json row keeps");
    assert_eq!(ench::ENCHANTMENTS[silk].min_cost, (15, 0), "the silk min curve keeps");
    assert_eq!(ench::ENCHANTMENTS[silk].max_cost, (65, 0), "the silk max curve keeps");
    // THE COST CURVE (the Enchantment$Cost.calculate law — base +
    // per * (l - 1))
    assert_eq!(ench::EnchDef::cost_at((1, 11), 1), 1, "the level-1 cost the base keeps");
    assert_eq!(ench::EnchDef::cost_at((1, 11), 4), 34, "the level-4 cost the per-level fold keeps");
    // THE TABLE POOL (the non_treasure tag's own resolution — the 7
    // treasure-only rows the pool never seats)
    for treasure in ["binding_curse", "frost_walker", "mending", "soul_speed", "swift_sneak", "vanishing_curse", "wind_burst"] {
        assert!(
            !ench::is_table_member(treasure),
            "the {treasure} treasure row the pool never seats"
        );
    }
    assert!(ench::is_table_member("sharpness"), "the sharpness row the pool seats");
    // THE COMPATIBILITY (the areCompatible law — the exclusive set
    // the containment speaks)
    let blast = ench::enchantment_index("blast_protection").unwrap();
    assert!(!ench::are_compatible(prot, blast), "the armor exclusivity the false wears");
    assert!(!ench::are_compatible(blast, prot), "the symmetric check the false wears");
    assert!(ench::are_compatible(prot, sharp), "the cross-family pair the true wears");
    let sharp2 = ench::enchantment_index("sharpness").unwrap();
    assert!(!ench::are_compatible(sharp2, sharp2), "the self-pair the false wears");
    // THE FAMILIES (the resolved tag trees the plain names answer)
    assert!(ench::item_in_family("diamond_sword", "sharp_weapon"), "the sword the sharp family keeps");
    assert!(ench::item_in_family("diamond_sword", "melee_weapon"), "the sword the melee family keeps");
    assert!(ench::item_in_family("diamond_axe", "sharp_weapon"), "the axe the sharp family keeps");
    assert!(!ench::item_in_family("diamond_axe", "melee_weapon"), "the axe the melee family refuses");
    assert!(ench::item_in_family("bow", "durability"), "the bow the durability family keeps");
    assert!(ench::item_in_family("mace", "weapon"), "the mace the weapon family keeps");
    assert!(ench::item_in_family("wooden_spear", "lunge"), "the spear the lunge family keeps");
    assert!(!ench::item_in_family("dirt", "durability"), "the dirt no family seats");
    // THE ENCHANTABILITY (the Enchantable component values — the
    // ToolMaterial/ArmorMaterial clinit rows jar-proven)
    assert_eq!(ench::item_enchantability("wooden_sword"), 15, "the wood row keeps");
    assert_eq!(ench::item_enchantability("stone_pickaxe"), 5, "the stone row keeps");
    assert_eq!(ench::item_enchantability("copper_sword"), 13, "the copper row keeps");
    assert_eq!(ench::item_enchantability("iron_sword"), 14, "the iron row keeps");
    assert_eq!(ench::item_enchantability("diamond_sword"), 10, "the diamond row keeps");
    assert_eq!(ench::item_enchantability("golden_sword"), 22, "the gold row keeps");
    assert_eq!(ench::item_enchantability("netherite_sword"), 15, "the netherite row keeps");
    assert_eq!(ench::item_enchantability("golden_helmet"), 25, "the gold armor row keeps");
    assert_eq!(ench::item_enchantability("turtle_helmet"), 9, "the turtle row keeps");
    assert_eq!(ench::item_enchantability("leather_boots"), 15, "the leather row keeps");
    assert_eq!(ench::item_enchantability("book"), 1, "the book row keeps");
    assert_eq!(ench::item_enchantability("mace"), 15, "the mace row keeps");
    assert_eq!(ench::item_enchantability("dirt"), 0, "the missing row the 0 keeps");
    // THE PRIMARY LAW (the isPrimaryItem pair — the supported gate
    // the primary set refines)
    assert!(ench::enchantment_accepts_item("sharpness", "diamond_sword"), "the primary sword accepts");
    // THE AXE'S OWN REFUSAL (the data-driven law — the sharpness
    // primary = melee_weapon = swords + spears, the axe the table
    // never seats [the anvil/book lane the axe's own road keeps])
    assert!(!ench::enchantment_accepts_item("sharpness", "diamond_axe"), "the axe the primary refuses");
    let thorns = ench::enchantment_index("thorns").unwrap();
    assert!(ench::enchantment_accepts_item("thorns", "diamond_chestplate"), "the chest the thorns primary keeps");
    assert!(!ench::enchantment_accepts_item("thorns", "diamond_helmet"), "the helmet the thorns primary refuses");
    assert!(!ench::enchantment_accepts_item("fire_aspect", "diamond_axe"), "the fire_aspect the axe refuses");
}

/// THE SELECTION WALK (the dev-196 round — the
/// selectEnchantment walk the python twin pins: the level roll the
/// spread the available scan the weighted pick the halving loop
/// the exact stream wears; the seed-0 rows the deterministic
/// lists answer).
#[test]
fn the_enchanting_table_the_selection_walk_wears() {
    let sword = mc_registry::item_id_by_name("diamond_sword").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let book = mc_registry::item_id_by_name("book").unwrap();
    let helmet = mc_registry::item_id_by_name("golden_helmet").unwrap();
    // THE SEED-0 ROWS (the twin's costs the same stream keeps —
    // the twin scripts/jar_work/enchant_twin_196.txt banks)
    let mut rng = crate::noise::VanillaRandom::new(0);
    assert_eq!(
        App::select_enchantments(sword, 2, &mut rng),
        vec![(33, 1)],
        "the cost-2 list the sharpness-I twin wears"
    );
    let mut rng = crate::noise::VanillaRandom::new(0);
    assert_eq!(
        App::select_enchantments(sword, 3, &mut rng),
        vec![(35, 1), (40, 1)],
        "the cost-3 list the smite+unbreaking twin wears"
    );
    let mut rng = crate::noise::VanillaRandom::new(0);
    assert_eq!(
        App::select_enchantments(sword, 6, &mut rng),
        vec![(35, 1), (40, 1)],
        "the cost-6 list the smite+unbreaking twin wears"
    );
    // THE SEED-0 BIG-COST ROWS (the direct walk the wide windows
    // answer — the multi-instance lists the halving loop lays)
    let mut rng = crate::noise::VanillaRandom::new(0);
    assert_eq!(
        App::select_enchantments(sword, 13, &mut rng),
        vec![(40, 2), (1, 2)],
        "the cost-13 list the unbreaking+bane twin wears"
    );
    let mut rng = crate::noise::VanillaRandom::new(0);
    assert_eq!(
        App::select_enchantments(sword, 30, &mut rng),
        vec![(40, 3), (1, 4), (17, 2), (37, 3)],
        "the cost-30 list the four-instance twin wears"
    );
    // THE ARMOR WALK (the golden helmet the armor pool the
    // projectile-protection primary wears)
    let mut rng = crate::noise::VanillaRandom::new(0);
    assert_eq!(
        App::select_enchantments(helmet, 8, &mut rng),
        vec![(27, 3), (40, 2)],
        "the helmet list the projectile-protection twin wears"
    );
    // THE NON-ENCHANTABLE GATE (the Enchantable null the empty
    // list wears — the dirt the honest empty answer keeps)
    let mut rng = crate::noise::VanillaRandom::new(0);
    assert!(
        App::select_enchantments(dirt, 30, &mut rng).is_empty(),
        "the dirt walk the empty list wears"
    );
    // THE BOOK WALK (the isBook bypass the whole pool serves — the
    // cost-2 the power-I instance, the cost-8 the unbreaking)
    let mut rng = crate::noise::VanillaRandom::new(0);
    assert_eq!(
        App::select_enchantments(book, 2, &mut rng),
        vec![(26, 1), (8, 1)],
        "the cost-2 book the power+efficiency twin wears"
    );
    let mut rng = crate::noise::VanillaRandom::new(0);
    assert_eq!(
        App::select_enchantments(book, 8, &mut rng),
        vec![(40, 1), (17, 1)],
        "the cost-8 book the unbreaking+knockback twin wears"
    );
    // THE PICK LIST (the menu's own seed + ROW law the
    // getEnchantmentList walk wears — the rows the DISTINCT streams
    // answer: the seed-0 app the row-0 the [sharpness, unbreaking]
    // list, the row-1 the smite-II trio, the row-2 the
    // unbreaking-III pair the twin banks)
    let mut app = App::new(3);
    app.enter_world();
    app.enchant_seed = 0;
    let cost_row = app.enchant_pick_list(0, sword, 8);
    assert_eq!(
        cost_row,
        vec![(33, 1), (40, 1)],
        "the row-0 pick the seed+0 stream wears"
    );
    let cost_row1 = app.enchant_pick_list(1, sword, 13);
    assert_eq!(
        cost_row1,
        vec![(35, 2), (17, 1), (10, 1)],
        "the row-1 pick the seed+1 stream wears"
    );
    let cost_row2 = app.enchant_pick_list(2, sword, 30);
    assert_eq!(
        cost_row2,
        vec![(40, 3), (35, 4)],
        "the row-2 pick the seed+2 stream wears"
    );
}

/// THE EMPTY GATE (the dev-196 round — the lambda pc 23-30 walk the
/// getEnchantmentList-empty the NOTHING-spent return wears: the
/// non-enchantable item the guards pass the list never forms, the
/// xp and the lapis and the seed the untouched keep — the 337
/// round's own unconditional spend the honest stop answers).
#[test]
fn the_enchanting_table_the_empty_gate_the_honest_stop_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    let w = app.offline_world.as_mut().unwrap();
    for y in surface..=surface + 2 {
        for x in c - 3..=c + 3 {
            for z in c - 3..=c + 3 {
                w.set(x, y, z, crate::world::AIR);
            }
        }
    }
    w.set(c, surface, c, crate::world::ENCHANTING_TABLE);
    w.set(c + 2, surface, c, crate::world::BOOKSHELF);
    app.open_enchanting_for(cell);
    let andesite = 7i32;
    let lapis = mc_registry::item_id_by_name("lapis_lazuli").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    // THE ANDESITE WALK (the 337 test's own item the gate now
    // refuses — the guards pass, the list never forms, NOTHING
    // spends)
    app.enchant_slots[0] = (andesite, 1);
    app.enchant_slots[1] = (lapis, 3);
    app.xp.level = 30;
    app.enchant_costs_refresh();
    assert!(
        app.enchanting_costs.iter().any(|&cost| cost > 0),
        "the cost rows the live walk lays"
    );
    assert!(
        !app.click_enchant_option(0),
        "the non-enchantable click the false wears"
    );
    assert_eq!(
        app.enchant_slots[1],
        (lapis, 3),
        "the lapis the untouched keep"
    );
    assert_eq!(app.xp.level, 30, "the xp the untouched keep");
    assert_eq!(app.enchant_seed, 0, "the seed the untouched keep");
    assert!(app.enchant_comps[0].is_empty(), "the store the empty keep");
    // THE DIRT WALK (the same honest stop the plain block wears)
    app.enchant_slots[0] = (dirt, 1);
    app.enchant_costs_refresh();
    assert!(
        !app.click_enchant_option(1),
        "the dirt click the false wears"
    );
    assert_eq!(
        app.enchant_slots[1],
        (lapis, 3),
        "the lapis the untouched keep"
    );
    assert_eq!(app.xp.level, 30, "the xp the untouched keep");
}

/// THE ENCHANT APPLICATION (the dev-196 round — the happy walk the
/// real instances the store wears: the power-15 ring the twin
/// costs lay, the click the [sharpness I, unbreaking I] list the
/// seat-0 store answers, the lapis and the xp the spend keeps, the
/// sword the unchanged item wears).
#[test]
fn the_enchanting_table_the_enchant_application_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    let w = app.offline_world.as_mut().unwrap();
    for y in surface..=surface + 2 {
        for x in c - 3..=c + 3 {
            for z in c - 3..=c + 3 {
                w.set(x, y, z, crate::world::AIR);
            }
        }
    }
    w.set(c, surface, c, crate::world::ENCHANTING_TABLE);
    // THE 15-SHELF RING (the scanned rows the y = surface+1 band
    // keeps — the ring cells the dx/dz = ±2 the scan iterates, the
    // power the 15 count answers)
    let mut placed = 0;
    for dx in -2i32..=2 {
        for dz in -2i32..=2 {
            if dx.abs() != 2 && dz.abs() != 2 {
                continue;
            }
            if placed >= 15 {
                break;
            }
            w.set(c + dx, surface + 1, c + dz, crate::world::BOOKSHELF);
            placed += 1;
        }
    }
    app.open_enchanting_for(cell);
    // THE OPEN'S OWN ZERO (the empty seats the slotsChanged's own
    // isEmpty gate keeps — the power the fresh-open zero wears, the
    // seat setup the refresh the real scan answers)
    assert_eq!(
        app.enchanting_power, 0,
        "the empty seats the zero power keeps"
    );
    let sword = mc_registry::item_id_by_name("diamond_sword").unwrap();
    let lapis = mc_registry::item_id_by_name("lapis_lazuli").unwrap();
    app.enchant_slots[0] = (sword, 1);
    app.enchant_slots[1] = (lapis, 3);
    app.xp.level = 30;
    app.enchant_costs_refresh();
    assert_eq!(
        app.enchanting_power, 15,
        "the 15-shelf ring the power scan keeps"
    );
    // THE TWIN COSTS (the seed-0 power-15 stream the [8, 13, 30]
    // rows answer)
    assert_eq!(
        app.enchanting_costs,
        [8, 13, 30],
        "the power-15 rows the twin costs wear"
    );
    // THE HAPPY CLICK (the row-0 the [sharpness I, unbreaking I]
    // list the store wears)
    assert!(
        app.click_enchant_option(0),
        "the power-15 click the true answer wears"
    );
    assert_eq!(
        app.enchant_comps[0],
        vec![(33, 1), (40, 1)],
        "the store the two-instance list wears"
    );
    assert_eq!(
        app.enchant_slots[1],
        (lapis, 2),
        "the lapis the consume-1 shrink wears"
    );
    assert_eq!(app.xp.level, 29, "the xp the level-1 spend wears");
    assert_eq!(
        app.enchant_slots[0],
        (sword, 1),
        "the sword the unchanged item wears"
    );
    // THE ROW-1 WALK (the fresh seed the re-roll + the fresh pick
    // serve — the costs the s1 stream [5, 10, 30] lays, the pick the
    // s1+1 stream the [sharpness II, knockback I] list wears)
    let seed_after_first = app.enchant_seed;
    app.enchant_slots[1] = (lapis, 3);
    app.xp.level = 30;
    assert!(
        app.click_enchant_option(1),
        "the row-1 click the true answer wears"
    );
    assert_eq!(
        app.enchant_comps[0],
        vec![(33, 2), (17, 1)],
        "the row-1 store the re-roll chain list wears"
    );
    assert_ne!(
        app.enchant_seed, seed_after_first,
        "the seed the second bump wears"
    );
    // THE ROW-2 WALK (the s2 stream the re-roll + the pick the
    // [sharpness IV, unbreaking III] list wears)
    app.enchant_slots[1] = (lapis, 3);
    app.xp.level = 30;
    assert!(
        app.click_enchant_option(2),
        "the row-2 click the true answer wears"
    );
    assert_eq!(
        app.enchant_comps[0],
        vec![(33, 4), (40, 3)],
        "the row-2 store the big-cost list wears"
    );
}

/// THE BOOK TRANSMUTE (the lambda pc 41-68 walk — the
/// transmuteCopy(ENCHANTED_BOOK) the count keeps, the seat-0 item
/// the enchanted-book id wears, the store the single instance the
/// book trim keeps).
#[test]
fn the_enchanting_table_the_book_transmute_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    let w = app.offline_world.as_mut().unwrap();
    for y in surface..=surface + 2 {
        for x in c - 3..=c + 3 {
            for z in c - 3..=c + 3 {
                w.set(x, y, z, crate::world::AIR);
            }
        }
    }
    w.set(c, surface, c, crate::world::ENCHANTING_TABLE);
    let mut placed = 0;
    for dx in -2i32..=2 {
        for dz in -2i32..=2 {
            if dx.abs() != 2 && dz.abs() != 2 {
                continue;
            }
            if placed >= 15 {
                break;
            }
            w.set(c + dx, surface + 1, c + dz, crate::world::BOOKSHELF);
            placed += 1;
        }
    }
    app.open_enchanting_for(cell);
    let book = mc_registry::item_id_by_name("book").unwrap();
    let enchanted_book = mc_registry::item_id_by_name("enchanted_book").unwrap();
    let lapis = mc_registry::item_id_by_name("lapis_lazuli").unwrap();
    app.enchant_slots[0] = (book, 1);
    app.enchant_slots[1] = (lapis, 3);
    app.xp.level = 30;
    app.enchant_costs_refresh();
    assert!(
        app.click_enchant_option(0),
        "the book click the true answer wears"
    );
    assert_eq!(
        app.enchant_slots[0],
        (enchanted_book, 1),
        "the seat-0 item the enchanted-book transmute wears"
    );
    assert_eq!(
        app.enchant_comps[0],
        vec![(40, 1)],
        "the store the single-instance trim wears"
    );
}

/// THE STORE'S OWN RIDE (the dev-196 round — the enchanted stack
/// the seats the metadata follows: the pickup the store hands to
/// the carried, the merge refusal the SWAP answers, the close the
/// ride-home the inventory store answers, the reopen the mirror
/// load wears).
#[test]
fn the_item_stack_the_store_rides_and_refuses_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    let w = app.offline_world.as_mut().unwrap();
    for y in surface..=surface + 2 {
        for x in c - 3..=c + 3 {
            for z in c - 3..=c + 3 {
                w.set(x, y, z, crate::world::AIR);
            }
        }
    }
    w.set(c, surface, c, crate::world::ENCHANTING_TABLE);
    let mut placed = 0;
    for dx in -2i32..=2 {
        for dz in -2i32..=2 {
            if dx.abs() != 2 && dz.abs() != 2 {
                continue;
            }
            if placed >= 15 {
                break;
            }
            w.set(c + dx, surface + 1, c + dz, crate::world::BOOKSHELF);
            placed += 1;
        }
    }
    app.open_enchanting_for(cell);
    let sword = mc_registry::item_id_by_name("diamond_sword").unwrap();
    let lapis = mc_registry::item_id_by_name("lapis_lazuli").unwrap();
    app.enchant_slots[0] = (sword, 1);
    app.enchant_slots[1] = (lapis, 3);
    app.xp.level = 30;
    app.enchant_costs_refresh();
    assert!(app.click_enchant_option(0), "the enchant the true wears");
    let store = app.enchant_comps[0].clone();
    assert_eq!(
        store,
        vec![(33, 1), (40, 1)],
        "the store the twin list wears"
    );
    // THE PICKUP (the seat click the store hands to the carried —
    // the empty hand the whole stack takes; the render the rects
    // the layout answers)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root());
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    assert!(
        !app.enchant_slot_rects.is_empty(),
        "the render the rects lay"
    );
    let (sx, sy, seat0) = app.enchant_slot_rects[0];
    assert_eq!(seat0, 0, "the first rect the item seat keeps");
    app.click_enchanting_slot(sx + 8, sy + 8, 0);
    assert_eq!(
        app.enchant_carried,
        (sword, 1),
        "the pickup the whole stack takes"
    );
    assert!(
        app.enchant_comps[38] == store && app.enchant_comps[0].is_empty(),
        "the store the carried rides"
    );
    // THE PLACE-BACK (the second click the enchanted stack the seat
    // returns — the carried's store the place-all arm rides home)
    app.click_enchanting_slot(sx + 8, sy + 8, 0);
    assert_eq!(
        app.enchant_slots[0],
        (sword, 1),
        "the place-back the whole stack wears"
    );
    assert!(
        app.enchant_comps[0] == store && app.enchant_comps[38].is_empty(),
        "the store the seat rides home"
    );
    // THE MERGE REFUSAL (the plain sword the carried the enchanted
    // seat the SWAP answers — the components the stack identity
    // carries, the full-slot same-id pair the merge never rides)
    app.enchant_carried = (sword, 1);
    app.enchant_comps[38].clear();
    app.click_enchanting_slot(sx + 8, sy + 8, 0);
    assert_eq!(
        app.enchant_carried,
        (sword, 1),
        "the carried the enchanted stack wears"
    );
    assert!(
        app.enchant_comps[38] == store && app.enchant_comps[0].is_empty(),
        "the refusal the swap keeps the store with the carried"
    );
    assert_eq!(
        app.enchant_slots[0],
        (sword, 1),
        "the seat the plain stack wears"
    );
    // THE RIDE-HOME (the close the enchanted stack the first-empty
    // placement the store answers — the merge lane never rides)
    app.close_enchanting(ScreenKind::World);
    assert_eq!(app.screen, ScreenKind::World, "the close the world wears");
    assert_eq!(
        app.inventory.items[0],
        (sword, 1),
        "the first-empty slot the enchanted stack wears"
    );
    assert_eq!(
        app.inventory_enchants[0], store,
        "the inventory store the ride-home keeps"
    );
    // THE REOPEN (the mirror load the store answers — the menu seat
    // 29 the hotbar row 0 keys)
    app.open_enchanting_for(cell);
    assert_eq!(
        app.enchant_comps[29], store,
        "the reopen the mirror load wears"
    );
}

/// THE SPELL WALK'S OWN CLUES (the dev-197 round — the slotsChanged
/// lambda's SECOND pass the pc 159-273 walk the clue rows the
/// random pick answers: the `get(nextInt(size))` instance the
/// python twin pins, the -1 sentinel the empty seats + the zeroed
/// rows keep, the seed+row stream the pick's own draw answers).
#[test]
fn the_enchanting_table_the_clue_picks_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    let w = app.offline_world.as_mut().unwrap();
    for y in surface..=surface + 2 {
        for x in c - 3..=c + 3 {
            for z in c - 3..=c + 3 {
                w.set(x, y, z, crate::world::AIR);
            }
        }
    }
    w.set(c, surface, c, crate::world::ENCHANTING_TABLE);
    let mut placed = 0;
    for dx in -2i32..=2 {
        for dz in -2i32..=2 {
            if dx.abs() != 2 && dz.abs() != 2 {
                continue;
            }
            if placed >= 15 {
                break;
            }
            w.set(c + dx, surface + 1, c + dz, crate::world::BOOKSHELF);
            placed += 1;
        }
    }
    app.open_enchanting_for(cell);
    // THE EMPTY SEATS' OWN SENTINELS (the reset walk the all-minus-
    // one rows keeps — the fresh open the -1 rows wear)
    assert!(
        app.enchant_clues.iter().all(|&clue| clue == -1),
        "the empty seats the -1 clues keep"
    );
    assert!(
        app.enchant_level_clues.iter().all(|&lv| lv == -1),
        "the empty seats the -1 levels keep"
    );
    // THE POWER-15 FILL (the seed-0 rows the twin clues answer —
    // the pick the random instance the tail draw wears: the row-0
    // the unbreaking-I instance, the row-1 the knockback-I, the
    // row-2 the smite-IV the twin banks)
    let sword = mc_registry::item_id_by_name("diamond_sword").unwrap();
    let lapis = mc_registry::item_id_by_name("lapis_lazuli").unwrap();
    app.enchant_slots[0] = (sword, 1);
    app.enchant_slots[1] = (lapis, 3);
    app.xp.level = 30;
    app.enchant_costs_refresh();
    assert_eq!(
        app.enchanting_costs,
        [8, 13, 30],
        "the power-15 rows the twin costs wear"
    );
    assert_eq!(
        app.enchant_clues[0], 40,
        "the row-0 clue the unbreaking id wears"
    );
    assert_eq!(app.enchant_level_clues[0], 1, "the row-0 level the I wears");
    assert_eq!(
        app.enchant_clues[1], 17,
        "the row-1 clue the knockback id wears"
    );
    assert_eq!(app.enchant_level_clues[1], 1, "the row-1 level the I wears");
    assert_eq!(
        app.enchant_clues[2], 35,
        "the row-2 clue the smite id wears"
    );
    assert_eq!(
        app.enchant_level_clues[2], 4,
        "the row-2 level the IV wears"
    );
    // THE BOOK'S OWN CLUE (the trimmed list the single instance the
    // pick answers — the unbreaking-I clue the trim's tail wears)
    let book = mc_registry::item_id_by_name("book").unwrap();
    app.enchant_slots[0] = (book, 1);
    app.enchant_costs_refresh();
    assert_eq!(
        app.enchant_clues[0], 40,
        "the book clue the unbreaking id wears"
    );
    assert_eq!(app.enchant_level_clues[0], 1, "the book level the I wears");
    // THE ROLL CHAIN (the s1 stream the re-roll + the picks answer —
    // the row-2 the smite-V instance the five-cost window wears)
    app.enchant_slots[0] = (sword, 1);
    app.xp.level = 30;
    assert!(app.click_enchant_option(0), "the click the true wears");
    assert_eq!(
        app.enchant_clues[0], 40,
        "the s1 row-0 clue the unbreaking id wears"
    );
    assert_eq!(
        app.enchant_level_clues[0], 1,
        "the s1 row-0 level the I wears"
    );
    assert_eq!(
        app.enchant_clues[1], 33,
        "the s1 row-1 clue the sharpness id wears"
    );
    assert_eq!(
        app.enchant_level_clues[1], 2,
        "the s1 row-1 level the II wears"
    );
    assert_eq!(
        app.enchant_clues[2], 35,
        "the s1 row-2 clue the smite id wears"
    );
    assert_eq!(
        app.enchant_level_clues[2], 5,
        "the s1 row-2 level the V wears"
    );
}

/// THE CLUE TOOLTIP'S OWN RENDER (the dev-197 round — the
/// EnchantmentScreen.render pc 48-439 walk the hovering row the
/// tooltip paints: the isHovering(60, 14 + 19*i, 108, 17) box the
/// hit answers, the clue + the cost gates speak, the box the
/// mouse-follow origin wears; the headless no-mouse pass the pure
/// render keeps).
#[test]
fn the_enchanting_table_the_clue_tooltip_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    let w = app.offline_world.as_mut().unwrap();
    for y in surface..=surface + 2 {
        for x in c - 3..=c + 3 {
            for z in c - 3..=c + 3 {
                w.set(x, y, z, crate::world::AIR);
            }
        }
    }
    w.set(c, surface, c, crate::world::ENCHANTING_TABLE);
    let mut placed = 0;
    for dx in -2i32..=2 {
        for dz in -2i32..=2 {
            if dx.abs() != 2 && dz.abs() != 2 {
                continue;
            }
            if placed >= 15 {
                break;
            }
            w.set(c + dx, surface + 1, c + dz, crate::world::BOOKSHELF);
            placed += 1;
        }
    }
    app.open_enchanting_for(cell);
    let sword = mc_registry::item_id_by_name("diamond_sword").unwrap();
    let lapis = mc_registry::item_id_by_name("lapis_lazuli").unwrap();
    app.enchant_slots[0] = (sword, 1);
    app.enchant_slots[1] = (lapis, 3);
    app.xp.level = 30;
    app.enchant_costs_refresh();
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let render = |app: &mut App| -> crate::raster::Framebuf {
        let mut fb = crate::raster::Framebuf::new(320, 180);
        let mut a = GuiAssets::open(&assets_root());
        app.render(&mut fb, &mut a);
        fb
    };
    // THE PURE RENDER (the no-mouse pass the baseline keeps)
    app.mouse_pos = None;
    let baseline = render(&mut app);
    // THE ROW-0 HOVER (the mouse the row's interior the hit answers
    // — the frame the tooltip the box paints)
    let panel = app.enchant_panel;
    let (mx, my) = (panel.0 + 60 + 50, panel.1 + 14 + 8);
    app.mouse_pos = Some((mx, my));
    let hovered = render(&mut app);
    // THE BOX'S OWN PRESENCE (the origin walk the +12/-12 seat keeps
    // — the interior margin pixel the background ink answers, the
    // baseline the same seat the pure world wears)
    let (tx, ty) = (mx + 12, my - 12);
    assert_ne!(
        pixel(&hovered, tx + 1, ty + 1),
        pixel(&baseline, tx + 1, ty + 1),
        "the tooltip box the interior the paint wears"
    );
    // THE OFF-PANEL PASS (the outside mouse the pure render keeps —
    // the no-seat hover the highlight and the tooltip the skips wear)
    app.mouse_pos = Some((2, 2));
    let offpanel = render(&mut app);
    assert_eq!(
        pixel(&offpanel, 0, 0),
        pixel(&baseline, 0, 0),
        "the off-panel mouse the pure render keeps"
    );
    assert_ne!(
        pixel(&hovered, tx + 1, ty + 1),
        pixel(&offpanel, tx + 1, ty + 1),
        "the tooltip render the distinct frames wear"
    );
    // THE XP-SHORT PASS (the level the cost below the RED
    // requirement line the three-row box answers — the box the
    // shorter height wears)
    app.xp.level = 1;
    app.mouse_pos = Some((mx, my));
    let xpshort = render(&mut app);
    assert_ne!(
        pixel(&xpshort, tx + 1, ty + 1),
        pixel(&baseline, tx + 1, ty + 1),
        "the xp-short box the paint wears"
    );
    // THE CREATIVE PASS (the infinite-materials arm the clue line
    // ALONE answers — the one-row box the paint keeps)
    app.game_mode = GameMode::Creative;
    let creative = render(&mut app);
    assert_ne!(
        pixel(&creative, tx + 1, ty + 1),
        pixel(&baseline, tx + 1, ty + 1),
        "the creative box the paint wears"
    );
}

/// THE ITEM STACK'S OWN ENCHANTMENT TOOLTIP LINES (the dev-198 round —
/// the `ItemEnchantments.addToTooltip` walk the 26.2 jar serves,
/// scripts/jar_work/glint198/item_ench_tooltip.txt the javap speaks):
/// the FIRST pass the `#minecraft:enchantment/tooltip_order` tag rows
/// the tag order speaks [pc 8-83 — the curse rows the tag's own head
/// keeps], the SECOND pass the untagged rows the map order answers,
/// the NO empty spacer the 26.2 bytecode banks [the CommonComponents
/// reference the class never carries], the ink the getFullname's own
/// style arms keep [the curse RED the non-curse GRAY], the numeral
/// the `level != 1 || maxLevel != 1` gate keeps, the ADVANCED id row
/// the walk lays AFTER the enchantment rows [pc 602-632 the order
/// banks], the `getInt(holder) > 0` gate the zero-level pair the skip
/// wears.
#[test]
fn the_item_stack_the_enchantment_tooltip_lines_wear() {
    let sword = mc_registry::item_id_by_name("diamond_sword").unwrap();
    use mc_registry::generated::enchantments as ench_mod;
    let sharp = ench_mod::enchantment_index("sharpness").unwrap();
    let prot = ench_mod::enchantment_index("protection").unwrap();
    let silk = ench_mod::enchantment_index("silk_touch").unwrap();
    let bind = ench_mod::enchantment_index("binding_curse").unwrap();
    // THE TWO PASSES' OWN ORDER (the store order [protection,
    // sharpness, curse] the TAG order [curse 0, sharpness 7,
    // protection 21] re-speaks — the curse line FIRST the tag's own
    // head keeps)
    let store = vec![(prot as i32, 4), (sharp as i32, 5), (bind as i32, 1)];
    let rows = App::item_tooltip_rows_enchanted(sword, false, &store);
    assert_eq!(
        rows.len(),
        4,
        "the name + the three lines the NO-spacer law keeps"
    );
    assert!(
        rows[0].1 == [1.0, 1.0, 1.0],
        "the name row the WHITE ink wears"
    );
    assert_eq!(rows[1].0, "Curse of Binding", "the tag's own head speaks");
    assert!(
        rows[1].1 == [1.0, 0.3333, 0.3333],
        "the curse line the RED ink wears"
    );
    assert_eq!(
        rows[2].0, "Sharpness V",
        "the sharpness line the tag order speaks"
    );
    assert!(
        rows[2].1 == [0.6667, 0.6667, 0.6667],
        "the non-curse line the GRAY ink wears"
    );
    assert_eq!(
        rows[3].0, "Protection IV",
        "the protection line the tail keeps"
    );
    // THE NUMERAL GATE (the `level != 1 || maxLevel != 1` walk — the
    // level-1-max-1 row the BARE name wears, the level-1-max-5 row
    // the numeral keeps)
    let rows = App::item_tooltip_rows_enchanted(sword, false, &[(silk as i32, 1)]);
    assert_eq!(rows.len(), 2, "the name + the one line the count keeps");
    assert_eq!(
        rows[1].0, "Silk Touch",
        "the level-1-max-1 row the bare name wears"
    );
    let rows = App::item_tooltip_rows_enchanted(sword, false, &[(sharp as i32, 1)]);
    assert_eq!(
        rows[1].0, "Sharpness I",
        "the level-1-max-5 row the numeral keeps"
    );
    // THE ZERO-LEVEL GATE (the `getInt(holder) > 0` pair — the
    // zero-level store row the skip wears)
    let rows = App::item_tooltip_rows_enchanted(sword, false, &[(sharp as i32, 0)]);
    assert_eq!(rows.len(), 1, "the zero-level pair the gate never seats");
    // THE ADVANCED ID ROW AFTER (the pc 602-632 arm the walk's own
    // order keeps — the enchantment rows PRECEDE the DARK_GRAY id)
    let rows = App::item_tooltip_rows_enchanted(sword, true, &[(sharp as i32, 3)]);
    assert_eq!(
        rows.len(),
        3,
        "the name + the line + the id the count keeps"
    );
    assert_eq!(rows[1].0, "Sharpness III", "the line the id PRECEDES");
    assert_eq!(
        rows[2].0, "minecraft:diamond_sword",
        "the id row the tail wears"
    );
    // THE UNTOUCHED LANE (the plain rows the empty store the same
    // shape keeps — the pre-round walk the byte-identical rest wears)
    let rows = App::item_tooltip_rows(sword, false);
    assert_eq!(rows.len(), 1, "the plain rows the empty store keeps");
}

/// THE ENCHANTED STACK'S OWN GLINT (the dev-198 round — the foil pass
/// the 26.2 jar's own law keeps, scripts/jar_work/glint198/ the
/// evidence banks): the SCROLL PHASES the setupGlintTexturing walk
/// answers, the FOOTPRINT the depth-EQUAL law keeps [the transparent
/// texels the glint never tints], the ADDITIVE the (SRC_COLOR, ONE)
/// blend the channels never darkens [the out = src.rgb² + dst the
/// factor pair speaks], the DETERMINISM the same-millis render the
/// byte-identical answer keeps, the SCROLL the phase shift the
/// different-millis frame the distinct pixels wear.
#[test]
fn the_item_stack_the_enchanted_glint_wears() {
    // THE PHASES (the pc 26-45 walk — the millis-0 the zero phases,
    // the 13750 ms the full X cycle [t = 13750*8 = 110000 → f3 = 0,
    // f4 = 20000/30000] the wrap answer keeps)
    assert_eq!(
        glint_scroll_phases(0),
        (0.0, 0.0),
        "the millis-0 the zero phases"
    );
    let (f3, f4) = glint_scroll_phases(13750);
    assert!((f3 - 0.0).abs() < 1e-6, "the X phase the full cycle wraps");
    assert!(
        (f4 - 20000.0 / 30000.0).abs() < 1e-6,
        "the Y phase the remainder keeps"
    );
    let (f3, f4) = glint_scroll_phases(6875);
    assert!(
        (f3 - 0.5).abs() < 1e-6 && (f4 - (55000.0 % 30000.0) / 30000.0).abs() < 1e-6,
        "the half-cycle phase the exact fold answers"
    );
    // THE RENDER (the inventory screen's own enchanted seat — the
    // store the foil pass the hotbar row's icon wears)
    let mut app = App::new(3);
    app.enter_world();
    let sword = mc_registry::item_id_by_name("diamond_sword").unwrap();
    use mc_registry::generated::enchantments as ench_mod;
    let sharp = ench_mod::enchantment_index("sharpness").unwrap() as i32;
    app.inventory.items[0] = (sword, 1);
    app.inventory_enchants[0] = vec![(sharp, 5)];
    app.open(ScreenKind::Inventory);
    let render = |app: &mut App| -> crate::raster::Framebuf {
        let mut fb = crate::raster::Framebuf::new(320, 180);
        let mut a = GuiAssets::open(&assets_root());
        app.render(&mut fb, &mut a);
        fb
    };
    // THE WARM RENDER (the panel's own layout the render lays — the
    // inv_panel the (0,0) floor the pre-render read never seats)
    app.frame_millis = 0;
    let _warm = render(&mut app);
    // THE ICON'S OWN SEAT (the hotbar row's slot 0 the inventory
    // panel geometry speaks — the origin helper the same cell both
    // lanes answer)
    let panel = app.inv_panel;
    let (sx, sy) = inventory_slot_origin(panel, 0);
    let ix = sx + 1;
    let iy = sy + 1;
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> [u8; 3] {
        let o = ((y as usize) * fb.w + x as usize) * 3;
        [fb.color[o], fb.color[o + 1], fb.color[o + 2]]
    };
    // THE PLAIN BASELINE (the store-less stack the same seat wears —
    // the same walk the cleared store the plain icon answers)
    app.inventory_enchants[0].clear();
    let plain_fb = render(&mut app);
    app.inventory_enchants[0] = vec![(sharp, 5)];
    app.frame_millis = 0;
    let g0 = render(&mut app);
    app.frame_millis = 20000;
    let g20 = render(&mut app);
    // THE ADDITIVE LAW (the enchanted pixels the channels never
    // darken — the out = src.rgb² + dst fold the per-channel verdict
    // keeps, the at-least-one the strict brightening speaks)
    let mut strict = 0;
    for py in 0..16i32 {
        for px in 0..16i32 {
            let (a, b) = (
                pixel(&plain_fb, ix + px, iy + py),
                pixel(&g0, ix + px, iy + py),
            );
            for k in 0..3 {
                assert!(
                    b[k] >= a[k],
                    "the additive law the channel never darkens at ({px},{py})[{k}]"
                );
            }
            if a != b {
                strict += 1;
            }
        }
    }
    assert!(
        strict > 0,
        "the glint the at-least-one pixel the tint speaks"
    );
    // THE FOOTPRINT LAW (the icon's own transparent pixels the glint
    // never tints — the sprite's alpha-0 cells the two millis frames
    // the identical answer keep)
    let (rgba, iw, ih) = mc_registry::item_icon(sword).expect("the icon the table keeps");
    let mut transparent = 0;
    for py in 0..16i32 {
        for px in 0..16i32 {
            let sxp = (px as u32).min(u32::from(iw) - 1);
            let syp = (py as u32).min(u32::from(ih) - 1);
            let alpha = rgba[((syp * u32::from(iw) + sxp) * 4 + 3) as usize];
            if alpha == 0 {
                transparent += 1;
                assert_eq!(
                    pixel(&g0, ix + px, iy + py),
                    pixel(&g20, ix + px, iy + py),
                    "the transparent pixel the footprint law keeps at ({px},{py})"
                );
                assert_eq!(
                    pixel(&g0, ix + px, iy + py),
                    pixel(&plain_fb, ix + px, iy + py),
                    "the transparent pixel the baseline the identical answer keeps at ({px},{py})"
                );
            }
        }
    }
    assert!(
        transparent > 0,
        "the sword sprite the transparent cells the footprint speaks"
    );
    // THE DETERMINISM + THE SCROLL (the same millis the byte-identical
    // render, the phase shift the distinct frame wears)
    app.frame_millis = 0;
    let g0b = render(&mut app);
    assert_eq!(
        fb_bytes(&g0),
        fb_bytes(&g0b),
        "the same millis the byte-identical render keeps"
    );
    let mut diff = 0;
    for py in 0..16i32 {
        for px in 0..16i32 {
            if pixel(&g0, ix + px, iy + py) != pixel(&g20, ix + px, iy + py) {
                diff += 1;
            }
        }
    }
    assert!(
        diff > 0,
        "the phase shift the scroll the distinct pixels wear"
    );
}

/// THE BREWING STAND'S OWN SCREEN TICK (the dev-338 round — javap 26.2
/// BrewingStandBlockEntity.serverTick the opened stand the live arm
/// serves: the blaze powder the fuel twenty refills, the 400-tick
/// count the start arms, the finish the ingredient spends — the
/// POTION_CONTENTS wall the mix never transforms, the vanilla's own
/// component-less verdict the (id, count) model wears). THE RESUME
/// the close/reopen pair the words bank keeps.
#[test]
fn the_offline_brewing_the_tick_walk_the_screen_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world.as_mut().unwrap().set(
        cell.0,
        cell.1,
        cell.2,
        crate::world::BREWING_STAND,
    );
    let gunpowder = mc_registry::item_id_by_name("gunpowder").unwrap();
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let powder = mc_registry::item_id_by_name("blaze_powder").unwrap();
    app.brewing_stores.insert(
        cell,
        vec![(potion, 1), (0, 0), (0, 0), (gunpowder, 2), (powder, 1)],
    );
    app.open_brewing_for(cell);
    assert_eq!(app.screen, ScreenKind::Brewing);
    assert_eq!(
        app.brewing_data,
        [0, 0, 0],
        "the fresh stand the zero words keep"
    );
    // THE FIRST TICK the refill + the start pair (the order the
    // bytecode keeps: the refill arm BEFORE the brewable gate reads).
    app.update();
    assert_eq!(
        app.brewing_data[crate::brewing_sim::DATA_BREW_TIME],
        400,
        "the sipush 400 the start arms"
    );
    assert_eq!(
        app.brewing_data[crate::brewing_sim::DATA_FUEL_USES],
        19,
        "the refill's twenty the start's own spend answers"
    );
    assert_eq!(
        app.brewing_data[crate::brewing_sim::INGREDIENT_WORD],
        gunpowder,
        "the start word banks"
    );
    assert_eq!(app.brewing_slots[4], (0, 0), "the powder the refill spent");
    // THE COUNT the middle walk wears (the 399 ticks the countdown
    // speaks, the finish the zero tick fires — the voice the
    // levelEvent 1035 the client answers).
    for _ in 0..399 {
        app.update();
    }
    assert_eq!(app.brewing_data[0], 1, "the last-before tick the one keeps");
    app.update();
    assert_eq!(app.brewing_data[0], 0, "the finish the zero wears");
    assert_eq!(
        app.brewing_slots[3],
        (gunpowder, 1),
        "the doBrew the ingredient spends"
    );
    assert_eq!(
        app.brewing_slots[0],
        (potion, 1),
        "the POTION_CONTENTS wall the bottle never transforms"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.brewing_stand.brew ")),
        "the brew voice the finish fires"
    );
    // THE CLOSE the words the bank flushes, the REOPEN the resume
    // wears (the retention law the words never clobbers — the tank's
    // 18 the bank keeps).
    app.close_brewing(ScreenKind::World);
    assert_eq!(
        app.brewing_word_stores
            .get(&cell)
            .map(|w| w[crate::brewing_sim::DATA_FUEL_USES]),
        Some(19),
        "the close flush the tank banks"
    );
    app.open_brewing_for(cell);
    assert_eq!(
        app.brewing_data[crate::brewing_sim::DATA_FUEL_USES],
        19,
        "the reopen the tank resumes"
    );
}

/// THE BREWING STANDS' OWN WORLD WALK (the dev-338 round — the vanilla
/// law the block entity's serverTick EVERY stocked stand speaks: the
/// closed cell the per-cell bank the tick wears, the finish the same
/// voice fires, the MINED cell the frozen walk keeps).
#[test]
fn the_offline_brewing_the_world_walk_the_banks_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world.as_mut().unwrap().set(
        cell.0,
        cell.1,
        cell.2,
        crate::world::BREWING_STAND,
    );
    let gunpowder = mc_registry::item_id_by_name("gunpowder").unwrap();
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let powder = mc_registry::item_id_by_name("blaze_powder").unwrap();
    app.brewing_stores.insert(
        cell,
        vec![(potion, 1), (0, 0), (0, 0), (gunpowder, 1), (powder, 1)],
    );
    // THE CLOSED CELL the world walk ticks (the open never spoke —
    // the brewing_pos the None keeps).
    app.update();
    assert_eq!(
        app.brewing_word_stores
            .get(&cell)
            .map(|w| w[crate::brewing_sim::DATA_BREW_TIME]),
        Some(400),
        "the closed stand the start arms"
    );
    // THE FINISH the same 400-tick walk answers.
    for _ in 0..400 {
        app.update();
    }
    assert_eq!(
        app.brewing_stores.get(&cell).map(|f| f[3]),
        Some((0, 0)),
        "the finish the ingredient spends"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.brewing_stand.brew ")),
        "the world walk the voice wears"
    );
    // THE MINED CELL the frozen walk keeps (the block the family
    // lost — the tick the walk never serves).
    let mined = (c + 3, surface + 1, c);
    app.brewing_stores.insert(
        mined,
        vec![(potion, 1), (0, 0), (0, 0), (gunpowder, 1), (powder, 1)],
    );
    app.brewing_word_stores.insert(mined, [0, 0, 0]);
    app.update();
    assert_eq!(
        app.brewing_word_stores.get(&mined).map(|w| w[0]),
        Some(0),
        "the mined stand the walk never serves"
    );
}

/// THE BREWING WORDS' OWN DISK BANK (the dev-338 round — the furnace
/// words' own law the brewing lane re-wears: the BTree order the
/// deterministic bytes keep, the short list the zero pad fills, the
/// over-long list the 3-word cap truncates, the missing list the
/// honest empties keep).
#[test]
fn the_brewing_stand_the_words_bank_the_disk_wears() {
    let mut bank = std::collections::HashMap::new();
    bank.insert((3, 40, -7), [400i32, 18, 978]);
    bank.insert((-1, 64, 2), [0, 20, 0]);
    let nbt = mc_protocol::nbt::Nbt::Compound(vec![(
        "brewing_words".to_string(),
        brewing_words_bank_to_nbt(&bank),
    )]);
    let parsed = brewing_words_from_nbt(&nbt);
    assert_eq!(parsed.len(), 2, "the two cells the round trip keeps");
    assert_eq!(parsed.get(&(3, 40, -7)), Some(&[400, 18, 978]));
    assert_eq!(parsed.get(&(-1, 64, 2)), Some(&[0, 20, 0]));
    // THE SHORT LIST the zero pad fills (the missing ingredient word
    // the honest zero keeps).
    let short = mc_protocol::nbt::Nbt::Compound(vec![(
        "brewing_words".to_string(),
        mc_protocol::nbt::Nbt::List(vec![mc_protocol::nbt::Nbt::Compound(vec![
            ("x".to_string(), mc_protocol::nbt::Nbt::Int(1)),
            ("y".to_string(), mc_protocol::nbt::Nbt::Int(2)),
            ("z".to_string(), mc_protocol::nbt::Nbt::Int(3)),
            (
                "Words".to_string(),
                mc_protocol::nbt::Nbt::List(vec![mc_protocol::nbt::Nbt::Int(123)]),
            ),
        ])]),
    )]);
    let parsed = brewing_words_from_nbt(&short);
    assert_eq!(parsed.get(&(1, 2, 3)), Some(&[123, 0, 0]));
    // THE OVER-LONG LIST the cap truncates (the corrupt width the
    // 3-word law keeps).
    let long = mc_protocol::nbt::Nbt::Compound(vec![(
        "brewing_words".to_string(),
        mc_protocol::nbt::Nbt::List(vec![mc_protocol::nbt::Nbt::Compound(vec![
            ("x".to_string(), mc_protocol::nbt::Nbt::Int(4)),
            ("y".to_string(), mc_protocol::nbt::Nbt::Int(5)),
            ("z".to_string(), mc_protocol::nbt::Nbt::Int(6)),
            (
                "Words".to_string(),
                mc_protocol::nbt::Nbt::List(vec![
                    mc_protocol::nbt::Nbt::Int(1),
                    mc_protocol::nbt::Nbt::Int(2),
                    mc_protocol::nbt::Nbt::Int(3),
                    mc_protocol::nbt::Nbt::Int(4),
                ]),
            ),
        ])]),
    )]);
    let parsed = brewing_words_from_nbt(&long);
    assert_eq!(parsed.get(&(4, 5, 6)), Some(&[1, 2, 3]));
    // THE MISSING LIST the honest empties keep.
    let none = mc_protocol::nbt::Nbt::Compound(vec![]);
    assert!(brewing_words_from_nbt(&none).is_empty());
}

/// THE BREWING SCREEN'S OWN PROGRESS SPRITES (the dev-338 round —
/// javap 26.2 BrewingStandScreen.extractBackground the exact draws
/// the words wear: the fuel bar the (60, 44) seat the k width keeps,
/// the arrow the (97, 16) seat the l height grows, the bubbles the
/// (63, 14 + 29 - m) seat the m rise keeps; the IDLE words the zero
/// words the draws never speak — the byte-identical rest the golden
/// law keeps).
#[test]
fn the_offline_brewing_the_render_the_progress_sprites_wear() {
    fn px(fb: &crate::raster::Framebuf, x: usize, y: usize) -> [u8; 3] {
        let o = (y * fb.w + x) * 3;
        [fb.color[o], fb.color[o + 1], fb.color[o + 2]]
    }
    // the idle bytes' own reader (the cloned baseline the compare
    // wears — the fb-free access the byte slice keeps)
    fn px_at(bytes: &[u8], w: usize, x: usize, y: usize) -> [u8; 3] {
        let o = (y * w + x) * 3;
        [bytes[o], bytes[o + 1], bytes[o + 2]]
    }
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| {
        p.join("gui/sprites/container/brewing_stand/fuel_length.png")
            .exists()
    });
    let Some(root) = root else { return };
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world.as_mut().unwrap().set(
        cell.0,
        cell.1,
        cell.2,
        crate::world::BREWING_STAND,
    );
    app.open_brewing_for(cell);
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let mut a = GuiAssets::open(&root);
    app.mouse_pos = None;
    app.render(&mut fb, &mut a);
    let idle = fb.color.clone();
    let panel = app.brewing_panel;
    // THE STAGED WORDS: the half-counted brew + the half-spent tank
    // (ticks 200 the arrow 14 + the bubbles 24 wears, fuel 10 the
    // 9-wide bar keeps).
    app.brewing_data = [200, 10, 0];
    let mut fb = crate::raster::Framebuf::new(320, 180);
    app.render(&mut fb, &mut a);
    // THE FUEL BAR the drawn width speaks (the k=9 bar the (60, 44)
    // seat wears — the idle render the region never painted).
    let mut bar_diff = 0;
    for dx in 0..9i32 {
        for dy in 0..4i32 {
            if px(
                &fb,
                (panel.0 + 60 + dx) as usize,
                (panel.1 + 44 + dy) as usize,
            ) != px_at(
                &idle,
                fb.w as usize,
                (panel.0 + 60 + dx) as usize,
                (panel.1 + 44 + dy) as usize,
            ) {
                bar_diff += 1;
            }
        }
    }
    assert!(bar_diff > 0, "the fuel bar the region paints ({bar_diff})");
    // THE PAST-BAR COLUMNS the idle keep (the no-stretch the crop
    // law wears — the k..18 tail the draw never reaches).
    let past_same = px(&fb, (panel.0 + 60 + 17) as usize, (panel.1 + 45) as usize)
        == px_at(
            &idle,
            fb.w as usize,
            (panel.0 + 60 + 17) as usize,
            (panel.1 + 45) as usize,
        );
    assert!(past_same, "the past-bar tail the idle keeps");
    // THE ARROW the fill speaks (the 14-tall fill the (97, 16) seat
    // wears) + THE BUBBLES the rise (the 24-tall column the (63,
    // 14 + 29 - 24) seat wears).
    let mut arrow_diff = 0;
    for dy in 0..14i32 {
        for dx in 0..9i32 {
            if px(
                &fb,
                (panel.0 + 97 + dx) as usize,
                (panel.1 + 16 + dy) as usize,
            ) != px_at(
                &idle,
                fb.w as usize,
                (panel.0 + 97 + dx) as usize,
                (panel.1 + 16 + dy) as usize,
            ) {
                arrow_diff += 1;
            }
        }
    }
    assert!(arrow_diff > 0, "the arrow the fill paints ({arrow_diff})");
    let mut bubble_diff = 0;
    for dy in 0..24i32 {
        for dx in 0..12i32 {
            if px(
                &fb,
                (panel.0 + 63 + dx) as usize,
                (panel.1 + 14 + 29 - 24 + dy) as usize,
            ) != px_at(
                &idle,
                fb.w as usize,
                (panel.0 + 63 + dx) as usize,
                (panel.1 + 14 + 29 - 24 + dy) as usize,
            ) {
                bubble_diff += 1;
            }
        }
    }
    assert!(
        bubble_diff > 0,
        "the bubbles the rise paints ({bubble_diff})"
    );
}

/// THE POTION KINDS' OWN SCREEN WALK (the autopush-339 round — the
/// POTION_CONTENTS kinds the seat bank rides: the kinded water
/// bottle the nether-wart brew the AWKWARD walk transforms, the
/// click pickup/place the kind pairs keep, the close/reopen the
/// bank resumes, the contents-less stack the wall the mix never
/// transforms).
#[test]
fn the_offline_brewing_the_kinds_the_mix_transform_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world.as_mut().unwrap().set(
        cell.0,
        cell.1,
        cell.2,
        crate::world::BREWING_STAND,
    );
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let wart = mc_registry::item_id_by_name("nether_wart").unwrap();
    let powder = mc_registry::item_id_by_name("blaze_powder").unwrap();
    let water = crate::brewing_sim::POTION_WATER;
    let awkward = crate::brewing_sim::POTION_AWKWARD;
    app.brewing_stores.insert(
        cell,
        vec![(potion, 1), (0, 0), (0, 0), (wart, 2), (powder, 1)],
    );
    app.brewing_kind_stores.insert(
        cell,
        [
            water,
            crate::brewing_sim::POTION_KIND_NONE,
            crate::brewing_sim::POTION_KIND_NONE,
            crate::brewing_sim::POTION_KIND_NONE,
            crate::brewing_sim::POTION_KIND_NONE,
        ],
    );
    app.open_brewing_for(cell);
    assert_eq!(
        app.brewing_kinds[0], water,
        "the open the banked kind loads"
    );
    // THE FIRST TICK the refill + the start pair.
    app.update();
    assert_eq!(
        app.brewing_data[crate::brewing_sim::DATA_BREW_TIME],
        400,
        "the kinded stand the start arms"
    );
    // THE FULL ARC the 400 ticks the finish speaks.
    for _ in 0..400 {
        app.update();
    }
    assert_eq!(
        app.brewing_slots[0],
        (potion, 1),
        "the potion row the item keeps"
    );
    assert_eq!(
        app.brewing_kinds[0], awkward,
        "the water bottle the awkward walk transforms"
    );
    assert_eq!(
        app.brewing_data[crate::brewing_sim::INGREDIENT_WORD],
        wart,
        "the start word the wart banks"
    );
    assert_eq!(
        app.brewing_slots[3],
        (wart, 1),
        "the doBrew the wart spends"
    );
    // THE CLOSE the kinds the bank flushes, the REOPEN the resume
    // wears.
    app.close_brewing(ScreenKind::World);
    assert_eq!(
        app.brewing_kind_stores.get(&cell).map(|k| k[0]),
        Some(awkward),
        "the close flush the awkward kind banks"
    );
    app.open_brewing_for(cell);
    assert_eq!(app.brewing_kinds[0], awkward, "the reopen the kind resumes");
    // THE KIND'S OWN RIDE the clicks wear: the pickup the kind the
    // carried banks (the seat the NONE wears), the place the carried
    // kind the seat lays (the same seat the resume answers).
    app.brewing_slot_rects = vec![(0, 0, 0), (20, 0, 1), (40, 0, 2), (60, 0, 3), (80, 0, 4)];
    app.brewing_panel = (0, 0);
    app.shift_down = false;
    app.click_brewing_slot(0, 0, 0);
    assert_eq!(
        app.brewing_carried,
        (potion, 1),
        "the pickup the stack rides"
    );
    assert_eq!(
        app.brewing_carried_kind, awkward,
        "the pickup the kind rides"
    );
    assert_eq!(
        app.brewing_kinds[0],
        crate::brewing_sim::POTION_KIND_NONE,
        "the emptied seat the NONE wears"
    );
    app.click_brewing_slot(0, 0, 0);
    assert_eq!(app.brewing_carried, (0, 0), "the place the carried lays");
    assert_eq!(
        app.brewing_carried_kind,
        crate::brewing_sim::POTION_KIND_NONE,
        "the placed carried the NONE wears"
    );
    assert_eq!(
        app.brewing_kinds[0], awkward,
        "the place the kind resumes the seat"
    );
    // THE NO-ROW WALL the brew never rewrites (the awkward + wart
    // pair the potionMixes table never banks — the brewable gate
    // the false keeps, the count the abort arm zeroes, the kinds
    // the rewrite never speaks).
    app.brewing_data[crate::brewing_sim::DATA_BREW_TIME] = 1;
    app.update();
    assert_eq!(
        app.brewing_kinds[0], awkward,
        "the row-less stand the kind never rewrites"
    );
    assert_eq!(
        app.brewing_data[crate::brewing_sim::DATA_BREW_TIME],
        0,
        "the unbrewable stand the count aborts"
    );
}

/// THE BREWING KINDS' OWN DISK BANK (the words' own law the 339
/// round re-wears — the round trip the five-cell widths keep, the
/// short list the NONE pad fills [never a kind 0 the water the
/// absence invents], the over-long list the cap truncates, the
/// missing list the honest empties keep).
#[test]
fn the_brewing_stand_the_kinds_bank_the_disk_wears() {
    let water = crate::brewing_sim::POTION_WATER;
    let awkward = crate::brewing_sim::POTION_AWKWARD;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut bank = std::collections::HashMap::new();
    bank.insert((3, 40, -7), [water, awkward, none, none, none]);
    bank.insert((-1, 64, 2), [none; 5]);
    let nbt = mc_protocol::nbt::Nbt::Compound(vec![(
        "brewing_kinds".to_string(),
        brewing_kinds_bank_to_nbt(&bank),
    )]);
    let parsed = brewing_kinds_from_nbt(&nbt);
    assert_eq!(parsed.len(), 2, "the two cells the round trip keeps");
    assert_eq!(
        parsed.get(&(3, 40, -7)),
        Some(&[water, awkward, none, none, none])
    );
    assert_eq!(parsed.get(&(-1, 64, 2)), Some(&[none; 5]));
    // THE SHORT LIST the NONE pad fills (the missing seats the kind
    // absence keeps — never a kind 0 the water potion invents).
    let short = mc_protocol::nbt::Nbt::Compound(vec![(
        "brewing_kinds".to_string(),
        mc_protocol::nbt::Nbt::List(vec![mc_protocol::nbt::Nbt::Compound(vec![
            ("x".to_string(), mc_protocol::nbt::Nbt::Int(1)),
            ("y".to_string(), mc_protocol::nbt::Nbt::Int(2)),
            ("z".to_string(), mc_protocol::nbt::Nbt::Int(3)),
            (
                "Kinds".to_string(),
                mc_protocol::nbt::Nbt::List(vec![mc_protocol::nbt::Nbt::Int(3)]),
            ),
        ])]),
    )]);
    let parsed = brewing_kinds_from_nbt(&short);
    assert_eq!(
        parsed.get(&(1, 2, 3)),
        Some(&[awkward, none, none, none, none]),
        "the short list the NONE pad wears"
    );
    // THE OVER-LONG LIST the cap truncates (the corrupt width the
    // 5-kind law keeps).
    let long = mc_protocol::nbt::Nbt::Compound(vec![(
        "brewing_kinds".to_string(),
        mc_protocol::nbt::Nbt::List(vec![mc_protocol::nbt::Nbt::Compound(vec![
            ("x".to_string(), mc_protocol::nbt::Nbt::Int(4)),
            ("y".to_string(), mc_protocol::nbt::Nbt::Int(5)),
            ("z".to_string(), mc_protocol::nbt::Nbt::Int(6)),
            (
                "Kinds".to_string(),
                mc_protocol::nbt::Nbt::List(vec![
                    mc_protocol::nbt::Nbt::Int(1),
                    mc_protocol::nbt::Nbt::Int(2),
                    mc_protocol::nbt::Nbt::Int(3),
                    mc_protocol::nbt::Nbt::Int(4),
                    mc_protocol::nbt::Nbt::Int(5),
                    mc_protocol::nbt::Nbt::Int(6),
                ]),
            ),
        ])]),
    )]);
    let parsed = brewing_kinds_from_nbt(&long);
    assert_eq!(
        parsed.get(&(4, 5, 6)),
        Some(&[1, 2, 3, 4, 5]),
        "the cap the five keeps"
    );
    // THE MISSING LIST the honest empties keep.
    let none_root = mc_protocol::nbt::Nbt::Compound(vec![]);
    assert!(brewing_kinds_from_nbt(&none_root).is_empty());
}

fn fb_bytes(fb: &crate::raster::Framebuf) -> &[u8] {
    &fb.color
}

/// THE WATER-BOTTLE'S OWN FILL (the dev-199 round — javap 26.2
/// BottleItem.use pcs 152-281 the live disasm the walk banks —
/// scripts/jar_work/bottle_item_199.txt the evidence serves): the
/// SOURCE_ONLY POV clip the water source the fill verdict arms, the
/// `getFluidState(pos).is(FluidTags.WATER)` read the pool cell
/// answers, the BOTTLE_FILL voice the player's ear the NEUTRAL
/// 1.0/1.0 pair wears, the `PotionContents.createItemStack(Items.
/// POTION, Potions.WATER)` the WATER kind mints, the
/// createFilledResult walk the emptied stack the filled result
/// REPLACES. The source the fill NEVER drains (the vanilla's own
/// law — the bucket the only drain the pickup lane speaks).
#[test]
fn the_offline_bottle_the_fill_the_water_kind_mints() {
    let mut app = App::new(3);
    app.enter_world();
    let slot = app.selected_hotbar;
    app.inventory.items[slot] = (crate::world::ITEM_GLASS_BOTTLE, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // the hit cell the fluid source keeps (the pool cell the DOWN
    // look answers — the stone floor the same column carries)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, surface, c, crate::world::WATER);
        w.set(c, surface - 1, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE FILL: the SUCCESS verdict the press consumes
    assert!(app.on_right_press(0, 0), "the fill the SUCCESS consumes");
    // THE SLOT'S OWN REPLACE (the emptied stack the filled answers)
    assert_eq!(
        app.inventory.items[slot],
        (crate::world::ITEM_POTION, 1),
        "the bottle the water potion wears"
    );
    // THE KIND'S OWN MINT (the WATER kind the seat banks — the
    // registry's row 0 the kind wears)
    assert_eq!(
        app.items_potion_kinds[slot],
        crate::brewing_sim::POTION_WATER,
        "the WATER kind the seat banks"
    );
    // THE SOURCE'S OWN STAY (the vanilla fill the pool never drains
    // — the bucket the only drain the pickup lane speaks)
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface, c),
        crate::world::WATER,
        "the source the fill never drains"
    );
    // THE VOICE (pc 224-246 — the BOTTLE_FILL the player's ear seats)
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("item.bottle.fill @")),
            "the BOTTLE_FILL voice the fill rode"
        );
    }
}

/// THE FILL'S OWN REFUSALS — javap BottleItem.use the PASS arms the
/// walk keeps: the LAVA source the `is(FluidTags.WATER)` read the
/// tag's own refusal wears (pc 218-221 the ifeq the PASS speaks),
/// the air miss the SOURCE_ONLY clip the None the PASS keeps — the
/// press never consumes, the stack the same bottle wears, the voice
/// the walk never speaks.
#[test]
fn the_offline_bottle_the_refusals_the_pass_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    let slot = app.selected_hotbar;
    app.inventory.items[slot] = (crate::world::ITEM_GLASS_BOTTLE, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    // THE LAVA POOL (the source the tag's refusal answers)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, surface, c, crate::world::LAVA);
        w.set(c, surface - 1, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the lava the PASS the press never consumes"
    );
    assert_eq!(
        app.inventory.items[slot],
        (crate::world::ITEM_GLASS_BOTTLE, 1),
        "the bottle the refusal keeps"
    );
    assert_eq!(
        app.items_potion_kinds[slot],
        crate::brewing_sim::POTION_KIND_NONE,
        "the seat the refusal never banks"
    );
    // THE AIR MISS (the open-sky look the SOURCE_ONLY clip the None
    // answers — the water pool behind the back the ray never sees)
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -6..=6 {
            for dz in -6..=6 {
                w.set(c + dx, surface, c + dz, crate::world::AIR);
            }
        }
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 8.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the miss the PASS the press never consumes"
    );
    assert_eq!(
        app.inventory.items[slot],
        (crate::world::ITEM_GLASS_BOTTLE, 1),
        "the bottle the miss keeps"
    );
}

/// THE SURVIVING COUNT'S OWN MINT — javap ItemUtils.createFilledResult
/// pc 52-69 (scripts/jar_work/item_utils_199.txt the evidence serves):
/// the `consume(1)` the shrink the stack keeps, the filled the
/// `Inventory.add` the free-slot scan answers, the minted row's kind
/// seat the WATER banks. The potion the max-stack-1 law the merge
/// never fits (the remaining-space scan the None verdict keeps).
#[test]
fn the_offline_bottle_the_count3_the_free_row_mints() {
    let mut app = App::new(3);
    app.enter_world();
    let slot = app.selected_hotbar;
    app.inventory.items[slot] = (crate::world::ITEM_GLASS_BOTTLE, 3);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, surface, c, crate::world::WATER);
        w.set(c, surface - 1, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the fill the SUCCESS consumes");
    // THE SHRINK (the surviving count the slot keeps)
    assert_eq!(
        app.inventory.items[slot],
        (crate::world::ITEM_GLASS_BOTTLE, 2),
        "the shrink the bottles wear"
    );
    // THE MINT (the first free row the potion answers — the max-1
    // law the merge never fits)
    let minted = (0..36)
        .find(|&i| app.inventory.items[i] == (crate::world::ITEM_POTION, 1))
        .expect("the minted row the free-slot scan finds");
    assert_ne!(minted, slot, "the mint the free row wears");
    assert_eq!(
        app.items_potion_kinds[minted],
        crate::brewing_sim::POTION_WATER,
        "the WATER kind the minted seat banks"
    );
    assert_eq!(
        app.items_potion_kinds[slot],
        crate::brewing_sim::POTION_KIND_NONE,
        "the bottle row the NONE keeps"
    );
}

/// THE CREATIVE ARM'S OWN KEEP — javap createFilledResult pc 6-36:
/// the `hasInfiniteMaterials` verdict the bottle NOT consumed, the
/// potion the `inventory.contains` gate the add rides (the potion
/// row the scan finds the add never speaks, the no-row the free-slot
/// mint answers). The second press the contains gate the same
/// verdict keeps.
#[test]
fn the_offline_bottle_the_creative_the_bottle_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    let slot = app.selected_hotbar;
    app.inventory.items[slot] = (crate::world::ITEM_GLASS_BOTTLE, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, surface, c, crate::world::WATER);
        w.set(c, surface - 1, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the creative fill consumes");
    // THE KEEP (the bottle the slot wears)
    assert_eq!(
        app.inventory.items[slot],
        (crate::world::ITEM_GLASS_BOTTLE, 1),
        "the creative the bottle never consumes"
    );
    // THE MINT (the free row the potion answers, the seat banks)
    let minted = (0..36)
        .find(|&i| app.inventory.items[i] == (crate::world::ITEM_POTION, 1))
        .expect("the minted row the add answers");
    assert_eq!(
        app.items_potion_kinds[minted],
        crate::brewing_sim::POTION_WATER,
        "the WATER kind the minted seat banks"
    );
    // THE SECOND PRESS (the contains gate the potion row finds — the
    // add never speaks, the mint never doubles)
    assert!(app.on_right_press(0, 0), "the creative fill consumes");
    let potions = app
        .inventory
        .items
        .iter()
        .filter(|(id, _)| *id == crate::world::ITEM_POTION)
        .count();
    assert_eq!(potions, 1, "the contains gate the mint keeps single");
}



/// THE DROPPED KIND'S OWN SEAT (the dev-199 round — the Q-drop the
/// emptied row the bank clears; the dropped entity the (id, count)
/// pair the kind never carries — the same honest note the thrown
/// carried speaks).
#[test]
fn the_offline_bottle_the_drop_the_seat_clears() {
    let mut app = App::new(3);
    app.enter_world();
    let slot = app.selected_hotbar;
    app.inventory.items[slot] = (crate::world::ITEM_POTION, 1);
    app.items_potion_kinds[slot] = crate::brewing_sim::POTION_WATER;
    app.on_drop_key(true);
    assert_eq!(
        app.inventory.items[slot],
        (0, 0),
        "the whole drop the row empties"
    );
    assert_eq!(
        app.items_potion_kinds[slot],
        crate::brewing_sim::POTION_KIND_NONE,
        "the emptied row the bank clears"
    );
}


/// THE FILL LOOP'S OWN REAL WALK (the dev-199 round's own purpose —
/// the kinds' own entry lane the brewing loop the real flow wears):
/// the pool fill the WATER kind mints, the stand's open the seat
/// pickup the kind the carried rides, the place the seat lays, the
/// nether wart the AWKWARD transform the 400-tick arc answers — the
/// SAME flow the real Minecraft's first brew speaks. The close the
/// picked-up brew the ride home the row's seat banks.
#[test]
fn the_offline_bottle_the_fill_the_brew_loop_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let slot = app.selected_hotbar;
    app.inventory.items[slot] = (crate::world::ITEM_GLASS_BOTTLE, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let stand_cell = (c + 2, surface, c);
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, surface, c, crate::world::WATER);
        w.set(c, surface - 1, c, crate::world::STONE);
        w.set(
            stand_cell.0,
            stand_cell.1,
            stand_cell.2,
            crate::world::BREWING_STAND,
        );
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE FILL (the mint the kind banks)
    assert!(app.on_right_press(0, 0), "the fill consumes");
    assert_eq!(
        app.items_potion_kinds[slot],
        crate::brewing_sim::POTION_WATER,
        "the filled seat the WATER banks"
    );
    // THE STAND'S OWN DOOR (the open the fresh five the walk serves)
    app.open_brewing_for(stand_cell);
    // THE SYNTHETIC RECTS (the five + the mirror rows the same
    // numbering the render lays — the hotbar row the menu 32+ speaks)
    app.brewing_slot_rects = [(0, 0, 0i16), (20, 0, 1), (40, 0, 2), (60, 0, 3), (80, 0, 4)]
        .into_iter()
        .chain((0..36).map(|i| ((i % 9) * 20, 100 + (i / 9) * 20, (5 + i) as i16)))
        .collect();
    app.brewing_panel = (0, 0);
    app.shift_down = false;
    // THE PICKUP (the hotbar seat menu 32+slot = items[slot] — the
    // mirror rows the main 27 first speak, the hotbar the 27+index
    // seat wears; the kind the carried rides)
    let mirror = 27 + slot as usize;
    let seat_mx = ((mirror % 9) * 20) as i32;
    let seat_my = (100 + (mirror / 9) * 20) as i32;
    app.click_brewing_slot(seat_mx, seat_my, 0);
    assert_eq!(
        app.brewing_carried,
        (crate::world::ITEM_POTION, 1),
        "the pickup the filled stack rides"
    );
    assert_eq!(
        app.brewing_carried_kind,
        crate::brewing_sim::POTION_WATER,
        "the pickup the WATER kind rides"
    );
    assert_eq!(
        app.items_potion_kinds[slot],
        crate::brewing_sim::POTION_KIND_NONE,
        "the emptied row the NONE wears"
    );
    // THE PLACE (the bottle seat the carried lays — the kind the
    // seat banks)
    app.click_brewing_slot(0, 0, 0);
    assert_eq!(
        app.brewing_slots[0],
        (crate::world::ITEM_POTION, 1),
        "the place the bottle seat wears"
    );
    assert_eq!(
        app.brewing_kinds[0],
        crate::brewing_sim::POTION_WATER,
        "the place the kind seat banks"
    );
    // THE RECIPE (the wart + the powder the seats take)
    let wart = mc_registry::item_id_by_name("nether_wart").unwrap();
    let powder = mc_registry::item_id_by_name("blaze_powder").unwrap();
    app.brewing_slots[3] = (wart, 2);
    app.brewing_slots[4] = (powder, 1);
    // THE FULL ARC (the 400 ticks the AWKWARD transform answers)
    app.update();
    for _ in 0..400 {
        app.update();
    }
    assert_eq!(
        app.brewing_kinds[0],
        crate::brewing_sim::POTION_AWKWARD,
        "the water bottle the awkward walk transforms"
    );
    // THE RIDE HOME (the pick-up + the close the brewed kind the row
    // banks — the loop's own half the fill round wears)
    app.brewing_slot_rects = [(0, 0, 0i16), (20, 0, 1), (40, 0, 2), (60, 0, 3), (80, 0, 4)]
        .into_iter()
        .chain((0..36).map(|i| ((i % 9) * 20, 100 + (i / 9) * 20, (5 + i) as i16)))
        .collect();
    app.click_brewing_slot(0, 0, 0);
    assert_eq!(
        app.brewing_carried_kind,
        crate::brewing_sim::POTION_AWKWARD,
        "the pickup the brewed kind rides"
    );
    app.close_brewing(crate::screens::ScreenKind::World);
    assert_eq!(
        app.items_potion_kinds[slot],
        crate::brewing_sim::POTION_AWKWARD,
        "the ride home the row's seat banks"
    );
    assert_eq!(
        app.brewing_carried,
        (0, 0),
        "the close the carried home rides"
    );
}
use super::*;

/// THE ROWS' OWN QUICK-MOVE HAND-OVER (the dev-199 round — the kind
/// the whole move BOTH directions wears): the filled hotbar row the
/// shift-click the five seat rides (the kind the seat banks), the
/// kinded five seat the shift-click the row's bank answers — the
/// source seat the cleared bank hands over, the destination the
/// gained seat lays.
#[test]
fn the_offline_brewing_the_quickmove_the_kind_hands_over() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let stand_cell = (c, surface, c);
    app.offline_world.as_mut().unwrap().set(
        stand_cell.0,
        stand_cell.1,
        stand_cell.2,
        crate::world::BREWING_STAND,
    );
    let slot = app.selected_hotbar;
    // THE BANKED ROW (the fill walk the mint banks — the direct lane
    // the same seat the tests speak)
    app.inventory.items[slot] = (crate::world::ITEM_POTION, 1);
    app.items_potion_kinds[slot] = crate::brewing_sim::POTION_WATER;
    app.open_brewing_for(stand_cell);
    app.brewing_slot_rects = [(0, 0, 0i16), (20, 0, 1), (40, 0, 2), (60, 0, 3), (80, 0, 4)]
        .into_iter()
        .chain((0..36).map(|i| ((i % 9) * 20, 100 + (i / 9) * 20, (5 + i) as i16)))
        .collect();
    app.brewing_panel = (0, 0);
    // THE ROW → FIVE HAND-OVER (the shift-click the potion the seat
    // rides, the kind the seat banks, the row's bank the NONE wears)
    app.shift_down = true;
    let mirror = 27 + slot as usize;
    let seat_mx = ((mirror % 9) * 20) as i32;
    let seat_my = (100 + (mirror / 9) * 20) as i32;
    app.click_brewing_slot(seat_mx, seat_my, 0);
    assert_eq!(
        app.brewing_slots[0],
        (crate::world::ITEM_POTION, 1),
        "the quick-move the potion the seat rides"
    );
    assert_eq!(
        app.brewing_kinds[0],
        crate::brewing_sim::POTION_WATER,
        "the quick-move the kind the seat banks"
    );
    assert_eq!(
        app.items_potion_kinds[slot],
        crate::brewing_sim::POTION_KIND_NONE,
        "the source row the cleared bank wears"
    );
    // THE FIVE → ROW HAND-OVER (the shift-click back — the kind the
    // row's bank answers; the mirror scan the FIRST empty the main
    // grid's own head wears [items 9..36 the rows 5..32 speak] — the
    // minted row the ride banks)
    app.click_brewing_slot(0, 0, 0);
    let home = (0..36)
        .find(|&i| app.inventory.items[i] == (crate::world::ITEM_POTION, 1))
        .expect("the ride home the row the scan finds");
    assert_eq!(
        app.items_potion_kinds[home],
        crate::brewing_sim::POTION_WATER,
        "the ride home the kind the row banks"
    );
    assert_eq!(
        app.brewing_kinds[0],
        crate::brewing_sim::POTION_KIND_NONE,
        "the emptied seat the NONE wears"
    );
    assert_eq!(
        app.items_potion_kinds[slot],
        crate::brewing_sim::POTION_KIND_NONE,
        "the original row the absence keeps"
    );
}
use super::*;

/// THE ROWS' OWN DISK BANK (the dev-199 round — the "PotionKinds"
/// list the player tag stores flat, the round trip the restart
/// resumes, the elder file the missing row the NONE parse keeps, the
/// short list the pad fills, the over-long the 36-cap truncates).
#[test]
fn the_player_rows_the_potion_kinds_the_disk_wears() {
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let water = crate::brewing_sim::POTION_WATER;
    let mut sp = SavedPlayer {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        yaw: 0.0,
        pitch: 0.0,
        inventory: crate::drops::Inventory::default(),
        potion_kinds: [none; 37],
        selected_hotbar: 0,
        health: 20.0,
        food_level: 20,
        saturation: 5.0,
        exhaustion: 0.0,
        tick_timer: 0,
        xp_level: 0,
        xp_progress: 0.0,
        xp_total: 0,
    };
    sp.inventory.items[3] = (crate::world::ITEM_POTION, 1);
    sp.potion_kinds[3] = water;
    sp.potion_kinds[10] = crate::brewing_sim::POTION_AWKWARD;
    // THE FILE'S OWN SHAPE (the "player" compound the root wraps —
    // the tag row the same law the save speaks)
    let root =
        mc_protocol::nbt::Nbt::Compound(vec![("player".to_string(), player_row_to_nbt(&sp))]);
    let parsed = player_state_from_nbt(&root).expect("the row the parse answers");
    assert_eq!(
        parsed.potion_kinds[3], water,
        "the WATER seat the ride wears"
    );
    assert_eq!(
        parsed.potion_kinds[10],
        crate::brewing_sim::POTION_AWKWARD,
        "the AWKWARD seat the ride wears"
    );
    assert_eq!(
        parsed.potion_kinds[0], none,
        "the empty seat the NONE keeps"
    );
    assert_eq!(
        parsed.inventory.items[3],
        (crate::world::ITEM_POTION, 1),
        "the potion row the same tag wears"
    );
    // THE ELDER FILE (the missing list the NONE parse keeps)
    let elder_root = {
        let inner = player_row_to_nbt(&sp);
        let stripped: Vec<_> = match inner {
            mc_protocol::nbt::Nbt::Compound(fields) => fields
                .into_iter()
                .filter(|(k, _)| k != "PotionKinds")
                .collect(),
            _ => unreachable!(),
        };
        mc_protocol::nbt::Nbt::Compound(vec![(
            "player".to_string(),
            mc_protocol::nbt::Nbt::Compound(stripped),
        )])
    };
    let parsed = player_state_from_nbt(&elder_root).expect("the elder row the parse answers");
    assert_eq!(
        parsed.potion_kinds, [none; 37],
        "the elder file the NONE parse keeps"
    );
    // THE SHORT LIST the pad fills + THE OVER-LONG the 37-cap truncates
    let patched = {
        let inner = player_row_to_nbt(&sp);
        let mut player_fields = match inner {
            mc_protocol::nbt::Nbt::Compound(fields) => fields,
            _ => unreachable!(),
        };
        for (k, v) in player_fields.iter_mut() {
            if k == "PotionKinds" {
                *v = mc_protocol::nbt::Nbt::List(
                    (0..40).map(|i| mc_protocol::nbt::Nbt::Int(i)).collect(),
                );
            }
        }
        mc_protocol::nbt::Nbt::Compound(vec![(
            "player".to_string(),
            mc_protocol::nbt::Nbt::Compound(player_fields),
        )])
    };
    let parsed = player_state_from_nbt(&patched).expect("the patched row the parse answers");
    assert_eq!(parsed.potion_kinds[35], 35, "the cap the 36 keeps");
    assert_eq!(
        parsed.potion_kinds[36], 36,
        "the cap the 37 wears — the offhand's own seat the row lands"
    );
    assert_eq!(
        parsed.potion_kinds[3], 3,
        "the over-long list the rows wear"
    );
}

/// THE POTION KIND'S OWN TINT TABLES (the autopush-340 round — the
/// GENERATED tables the jar's own bytecode banks: the MobEffects
/// bootstrap the 40 `(name, color)` rows the ctor's ldc int wears,
/// the Potions bootstrap the 46 base names the strip law answers
/// [the long_/strong_ registry rows the BASE name the ctor arg wears
/// — pc 95/133 both "night_vision"], the effect rows the `(Holder, I)`
/// the amp-0 default and the `(Holder, II)` the explicit second int
/// wear). THE BLEND the `getColorOr` walk serves — the
/// `(amplifier + 1)`-weighted channels the divisor answers, the
/// empty rows the 0x385DC6 default keep, the single-effect rows the
/// effect's own color return REGARDLESS of the amplifier [the k·c/k
/// fold the weight division speaks].
#[test]
fn the_potion_kinds_the_tint_tables_the_jar_wears() {
    use crate::brewing_sim as bs;
    // THE TABLE SHAPE (the 40 colors, the 46 rows, the strip law)
    assert_eq!(bs::MOB_EFFECT_COLORS.len(), 40);
    assert_eq!(bs::POTION_KIND_BASE_NAMES.len(), 46);
    assert_eq!(bs::POTION_KIND_EFFECTS.len(), 46);
    for (i, reg) in bs::POTION_KIND_NAMES.iter().enumerate() {
        let strip = reg
            .strip_prefix("long_")
            .or_else(|| reg.strip_prefix("strong_"))
            .unwrap_or(reg);
        assert_eq!(
            bs::POTION_KIND_BASE_NAMES[i],
            strip,
            "the row {i} the registry {reg} the strip keeps"
        );
    }
    // THE COLOR SPOTS (the jar's own ldc ints — 3402751 the speed
    // green, 12779366 the night_vision lime, 16253176-... the instant
    // health's red the HealOrHarmMobEffect ctor wears)
    let color = |name: &str| {
        bs::MOB_EFFECT_COLORS
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, c)| *c)
            .unwrap()
    };
    assert_eq!(color("speed"), 0x33EBFF);
    assert_eq!(color("night_vision"), 0xC2FF66);
    assert_eq!(color("instant_health"), 0xF82423);
    assert_eq!(color("slowness"), 0x8BAFE0);
    assert_eq!(color("resistance"), 0x9146F0);
    // THE EFFECT ROWS (the jar's own pairs — turtle_master the TWO
    // effects the iconst_2 array wears [slowness IV + resistance III],
    // the strong row the amps 5/3 the same walk wears, the healing
    // rows the instant_health the amp 0/1 pair keeps)
    assert!(bs::POTION_KIND_EFFECTS[bs::POTION_WATER as usize].is_empty());
    assert!(bs::POTION_KIND_EFFECTS[bs::POTION_MUNDANE as usize].is_empty());
    assert!(bs::POTION_KIND_EFFECTS[bs::POTION_THICK as usize].is_empty());
    assert!(bs::POTION_KIND_EFFECTS[bs::POTION_AWKWARD as usize].is_empty());
    assert_eq!(
        bs::POTION_KIND_EFFECTS[bs::POTION_TURTLE_MASTER as usize],
        &[("slowness", 3), ("resistance", 2)]
    );
    assert_eq!(
        bs::POTION_KIND_EFFECTS[bs::POTION_STRONG_TURTLE_MASTER as usize],
        &[("slowness", 5), ("resistance", 3)]
    );
    assert_eq!(
        bs::POTION_KIND_EFFECTS[bs::POTION_STRONG_HEALING as usize],
        &[("instant_health", 1)]
    );
    // THE BLEND PINS (the walk's own integer answers — the empty rows
    // the 0x385DC6 default keep, the single-effect rows the effect's
    // own color return [the k·c/k fold], the turtle master the
    // weighted pair the divisor blends: slowness k4 + resistance k3,
    // r = (4·0x8B + 3·0x91)/7 = 0x8D, g = (4·0xAF + 3·0x46)/7 = 0x82,
    // b = (4·0xE0 + 3·0xF0)/7 = 0xE6)
    let def = bs::POTION_DEFAULT_COLOR;
    assert_eq!(def, 0x385DC6);
    assert_eq!(bs::potion_kind_liquid_color(bs::POTION_WATER), def);
    assert_eq!(bs::potion_kind_liquid_color(bs::POTION_MUNDANE), def);
    assert_eq!(bs::potion_kind_liquid_color(bs::POTION_THICK), def);
    assert_eq!(bs::potion_kind_liquid_color(bs::POTION_AWKWARD), def);
    assert_eq!(
        bs::potion_kind_liquid_color(-2),
        def,
        "the unknown the default keeps"
    );
    assert_eq!(
        bs::potion_kind_liquid_color(bs::POTION_NIGHT_VISION),
        0xC2FF66
    );
    assert_eq!(
        bs::potion_kind_liquid_color(bs::POTION_LONG_NIGHT_VISION),
        0xC2FF66,
        "the long row the SAME base name the same color wears"
    );
    assert_eq!(
        bs::potion_kind_liquid_color(bs::POTION_STRONG_HEALING),
        0xF82423,
        "the amp-1 row the k·c/k fold the same color answers"
    );
    assert_eq!(
        bs::potion_kind_liquid_color(bs::POTION_TURTLE_MASTER),
        0x8D82E6
    );
    assert_eq!(
        bs::potion_kind_liquid_color(bs::POTION_STRONG_TURTLE_MASTER),
        0x8D85E6
    );
    assert_eq!(bs::potion_kind_liquid_color(bs::POTION_LUCK), 0x59C106);
    assert_eq!(
        bs::potion_kind_liquid_color(bs::POTION_STRONG_HARMING),
        0xA9656A
    );
    // THE BASE NAME TAIL (the strip law the accessor answers, the
    // unknown the "empty" lane the orElse walk keeps)
    assert_eq!(bs::potion_kind_base_name(bs::POTION_WATER), "water");
    assert_eq!(
        bs::potion_kind_base_name(bs::POTION_LONG_NIGHT_VISION),
        "night_vision"
    );
    assert_eq!(bs::potion_kind_base_name(bs::POTION_LUCK), "luck");
    assert_eq!(bs::potion_kind_base_name(-1), "empty");
}

/// THE POTION KIND'S OWN DISPLAY NAME (the `getName` walk the lang
/// lane serves — the base-name strip the `item.minecraft.potion.
/// effect.<base>` key pairs [the 26.2 table's own 26 keys the strip
/// explains], the kindless/plain rows the item's own key keeps). THE
/// LANES: the held flash's kinded name re-arms the change arm [the
/// kind swap the SAME id the name still changes], the tooltip's
/// kinded rows the name row swaps the enchant/advanced rows keep.
#[test]
fn the_potion_kinds_the_display_names_the_strip_wears() {
    let app = App::new(3);
    let potion_id = mc_registry::item_id_by_name("potion").expect("the potion id");
    // THE KINDED NAMES (the lang rows the strip pairs)
    assert_eq!(
        app.held_item_display_name_kind(potion_id, crate::brewing_sim::POTION_WATER),
        "Water Bottle"
    );
    assert_eq!(
        app.held_item_display_name_kind(potion_id, crate::brewing_sim::POTION_LONG_NIGHT_VISION),
        "Potion of Night Vision",
        "the long row the BASE name the same display wears"
    );
    assert_eq!(
        app.held_item_display_name_kind(potion_id, crate::brewing_sim::POTION_STRONG_HEALING),
        "Potion of Healing"
    );
    assert_eq!(
        app.held_item_display_name_kind(potion_id, crate::brewing_sim::POTION_LUCK),
        "Potion of Luck"
    );
    assert_eq!(
        app.held_item_display_name_kind(potion_id, crate::brewing_sim::POTION_WIND_CHARGED),
        "Potion of Wind Charging"
    );
    assert_eq!(
        app.held_item_display_name_kind(potion_id, crate::brewing_sim::POTION_AWKWARD),
        "Awkward Potion"
    );
    // THE KINDLESS ROWS (the NONE the plain item name keeps, the
    // non-potion id the item's own lane keeps)
    assert_eq!(
        app.held_item_display_name_kind(potion_id, crate::brewing_sim::POTION_KIND_NONE),
        "Potion"
    );
    let stone_id = mc_registry::item_id_by_name("stone").unwrap_or(1);
    assert_eq!(
        app.held_item_display_name_kind(stone_id, crate::brewing_sim::POTION_NIGHT_VISION),
        "Stone",
        "the non-potion id the kind never speaks"
    );
    // THE TOOLTIP ROWS (the kinded seat the name row swaps — the
    // plain rows the parent walk keeps; the effect rows the 341 round
    // the walk appends, the advanced id row the tail the AFTER lane
    // wears)
    let plain = App::item_tooltip_rows(potion_id, false);
    assert_eq!(plain.len(), 1);
    assert_eq!(plain[0].0, "Potion");
    let kinded = App::item_tooltip_rows_kind(
        potion_id,
        crate::brewing_sim::POTION_LONG_NIGHT_VISION,
        false,
        &[],
    );
    assert_eq!(
        kinded.len(),
        2,
        "the name row the effect row the walk appends"
    );
    assert_eq!(kinded[0].0, "Potion of Night Vision");
    assert_eq!(kinded[0].1, WHITE);
    assert_eq!(
        kinded[1].0, "Night Vision (08:00)",
        "the long row the 9600-tick fold the %02d walk wears"
    );
    let kinded_advanced =
        App::item_tooltip_rows_kind(potion_id, crate::brewing_sim::POTION_WATER, true, &[]);
    assert_eq!(kinded_advanced.len(), 3);
    assert_eq!(kinded_advanced[0].0, "Water Bottle");
    assert_eq!(
        kinded_advanced[1].0, "No Effects",
        "the empty rows the NO_EFFECT row the walk wears"
    );
    assert_eq!(kinded_advanced[2].0, "minecraft:potion");
    // THE ENCHANT ROWS KEEP (the potion rows the glint override the
    // component never carries — the append the parent walk wears; the
    // name row the kind swaps, the effect rows the ench rows follow)
    let enched = App::item_tooltip_rows_kind(
        potion_id,
        crate::brewing_sim::POTION_WATER,
        false,
        &[(0, 1)],
    );
    assert_eq!(
        enched.len(),
        3,
        "the ench rows the walk appends, the effect row follows"
    );
    assert_eq!(enched[0].0, "Water Bottle");
    assert_eq!(enched[2].0, "No Effects");
}

/// THE POTION KIND'S OWN TINT WIRING (the autopush-340 round — the
/// `minecraft:potion` tint type the json def wears, the kindless NONE
/// the json's own default answers, the kinded rows the blend serves;
/// THE HOTBAR TICK the kind lane re-arms — the kind swap the SAME id
/// the name still changes, the change arm the vanilla law keeps).
#[test]
fn the_potion_kinds_the_icon_tint_the_wiring_wears() {
    // THE JSON LANES (the item_models def the real rows speak)
    let potion_json = "{\"default\":-13083194,\"type\":\"minecraft:potion\"}";
    let none = icon_tint_kind(&[potion_json], crate::brewing_sim::POTION_KIND_NONE);
    assert_eq!(none, crate::tint::unpack(0x385DC6));
    let nv = icon_tint_kind(&[potion_json], crate::brewing_sim::POTION_NIGHT_VISION);
    assert_eq!(nv, crate::tint::unpack(0xC2FF66));
    // THE KINDLESS WRAPPER the same default keeps (the world lanes'
    // call shape)
    assert_eq!(icon_tint(&[potion_json]), none);
    // THE OTHER TYPES UNAFFECTED (the constant row the parent walk
    // keeps, the potion arm the FIRST recognized type returns)
    let mixed = icon_tint_kind(
        &[
            "{\"value\":16711680,\"type\":\"minecraft:constant\"}",
            potion_json,
        ],
        crate::brewing_sim::POTION_NIGHT_VISION,
    );
    assert_eq!(mixed, crate::tint::unpack(0xFF0000));
    // THE REAL DEF (the registry's own potion model the tint json the
    // overlay layer wears — the wiring the 3D path serves)
    let potion_id = mc_registry::item_id_by_name("potion").expect("the potion id");
    let layers = mc_registry::display::item_gui_quads_broken(potion_id, false)
        .expect("the potion quads")
        .expect("the bake");
    assert!(
        layers
            .iter()
            .any(|(_, _, tints)| tints.iter().any(|t| t.contains("minecraft:potion"))),
        "the potion model the tint json the layer wears"
    );
    // THE HELD NAME TICK (the kind lane the change arm serves — the
    // SAME potion id the kind swap re-arms)
    let mut app = App::new(3);
    app.hotbar_slots[0] = (potion_id, 1);
    app.items_potion_kinds[0] = crate::brewing_sim::POTION_WATER;
    app.selected_hotbar = 0;
    app.tick_held_name();
    assert_eq!(
        app.held_name_last.as_ref().unwrap().1,
        "Water Bottle",
        "the water kind the fill lane's own name wears"
    );
    app.items_potion_kinds[0] = crate::brewing_sim::POTION_LONG_NIGHT_VISION;
    app.tick_held_name();
    assert_eq!(
        app.held_name_last.as_ref().unwrap().1,
        "Potion of Night Vision",
        "the kind swap the SAME id the name still changes"
    );
    // THE ID GATE (the non-potion row the stale seat never speaks)
    let stone_id = mc_registry::item_id_by_name("stone").unwrap_or(1);
    app.hotbar_slots[1] = (stone_id, 1);
    app.items_potion_kinds[1] = crate::brewing_sim::POTION_WATER;
    app.selected_hotbar = 1;
    app.tick_held_name();
    assert_eq!(
        app.held_name_last.as_ref().unwrap().1,
        "Stone",
        "the stale seat the id gate the NONE keeps"
    );
}

/// THE POTION KIND'S OWN EFFECT LINES (the autopush-341 round — the
/// addPotionTooltip walk the tooltip the rows wear: the description
/// the effect.minecraft.<name> tr the potency numeral pairs, the
/// duration the formatTickDuration %02d fold, the ink the category's
/// tooltip formatting, the empty rows the NO_EFFECT row).
#[test]
fn the_potion_kinds_the_effect_lines_the_jar_wears() {
    // THE TABLES' OWN SHAPES (the durations the rows the effects table
    // pairs, the categories the 40 rows the bootstrap wears)
    assert_eq!(crate::brewing_sim::POTION_KIND_DURATIONS.len(), 46);
    for k in 0..46usize {
        assert_eq!(
            crate::brewing_sim::POTION_KIND_DURATIONS[k].len(),
            crate::brewing_sim::POTION_KIND_EFFECTS[k].len(),
            "row {k} the durations the pairs' own length answers"
        );
    }
    assert_eq!(crate::brewing_sim::MOB_EFFECT_CATEGORIES.len(), 40);
    // THE CATEGORY SPOTS (the MobEffects ctor's first arg the parse wears)
    let cat = |n: &str| {
        crate::brewing_sim::MOB_EFFECT_CATEGORIES
            .iter()
            .find(|(e, _)| *e == n)
            .unwrap()
            .1
    };
    assert_eq!(cat("speed"), 0);
    assert_eq!(cat("slowness"), 1);
    assert_eq!(cat("night_vision"), 0);
    assert_eq!(cat("instant_damage"), 1);
    // THE DURATION SPOTS (the clinit's int pushes the parse wears)
    let k = crate::brewing_sim::POTION_NIGHT_VISION as usize;
    assert_eq!(crate::brewing_sim::POTION_KIND_DURATIONS[k], &[3600]);
    let k = crate::brewing_sim::POTION_LONG_NIGHT_VISION as usize;
    assert_eq!(crate::brewing_sim::POTION_KIND_DURATIONS[k], &[9600]);
    let k = crate::brewing_sim::POTION_TURTLE_MASTER as usize;
    assert_eq!(crate::brewing_sim::POTION_KIND_DURATIONS[k], &[400, 400]);
    let k = crate::brewing_sim::POTION_STRONG_HEALING as usize;
    assert_eq!(
        crate::brewing_sim::POTION_KIND_DURATIONS[k],
        &[1],
        "the instant row the duration-1 wears"
    );
    let k = crate::brewing_sim::POTION_LUCK as usize;
    assert_eq!(crate::brewing_sim::POTION_KIND_DURATIONS[k], &[6000]);
    // THE EFFECT ROWS' OWN WALK (the description + numeral + duration
    // + ink the addPotionTooltip law answers)
    let blue = [0.3333, 0.3333, 1.0];
    let red = [1.0, 0.3333, 0.3333];
    let rows = App::potion_kind_effect_rows(crate::brewing_sim::POTION_NIGHT_VISION);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].0, "Night Vision (03:00)");
    assert_eq!(rows[0].1, blue, "the BENEFICIAL the BLUE ink wears");
    let rows = App::potion_kind_effect_rows(crate::brewing_sim::POTION_STRONG_SWIFTNESS);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].0, "Speed II (01:30)",
        "the amp-1 the potency numeral the 1800-tick fold pairs (the effect row the registry name the Speed lang key answers — the Swiftness the potion NAME the other lane keeps)"
    );
    let rows = App::potion_kind_effect_rows(crate::brewing_sim::POTION_STRONG_HEALING);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].0, "Instant Health II",
        "the instant row the endsWithin(20) gate the duration never speaks"
    );
    let rows = App::potion_kind_effect_rows(crate::brewing_sim::POTION_HARMING);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].0, "Instant Damage");
    assert_eq!(rows[0].1, red, "the HARMFUL the RED ink wears");
    let rows = App::potion_kind_effect_rows(crate::brewing_sim::POTION_TURTLE_MASTER);
    assert_eq!(rows.len(), 2, "the turtle rows the TWO effects wear");
    assert_eq!(rows[0].0, "Slowness IV (00:20)");
    assert_eq!(rows[0].1, red);
    assert_eq!(rows[1].0, "Resistance III (00:20)");
    assert_eq!(rows[1].1, blue);
    let rows = App::potion_kind_effect_rows(crate::brewing_sim::POTION_LUCK);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].0, "Luck (05:00)");
    let rows = App::potion_kind_effect_rows(crate::brewing_sim::POTION_WATER);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].0, "No Effects",
        "the empty rows the NO_EFFECT row wears"
    );
    assert_eq!(rows[0].1, GRAY, "the NO_EFFECT the GRAY ink wears");
    // THE BOUNDS (the unknown kind the empty vec keeps)
    assert!(App::potion_kind_effect_rows(-1).is_empty());
    assert!(App::potion_kind_effect_rows(999).is_empty());
    // THE TOOLTIP'S OWN ORDER (the name the effect rows the advanced
    // id row the AFTER lane wears — the pop the tail the re-lay; the
    // attribute block the dev-200 round the effect rows AFTER serves
    // — the turtle's slowness the pair the collect wears, the
    // whenDrank block the walk's own tail keeps)
    let potion_id = mc_registry::item_id_by_name("potion").expect("the potion id");
    let rows = App::item_tooltip_rows_kind(
        potion_id,
        crate::brewing_sim::POTION_TURTLE_MASTER,
        true,
        &[],
    );
    assert_eq!(
        rows.len(),
        7,
        "the name the two effect rows the attribute block the id row the walk wears"
    );
    assert_eq!(rows[0].0, "Potion of the Turtle Master");
    assert_eq!(rows[1].0, "Slowness IV (00:20)");
    assert_eq!(rows[2].0, "Resistance III (00:20)");
    assert_eq!(
        rows[3].0, "",
        "the blank line the CommonComponents.EMPTY the block head wears"
    );
    assert_eq!(rows[4].0, "When Applied:", "the whenDrank header speaks");
    assert_eq!(
        rows[5].0, "-60% Speed",
        "the turtle slowness the amp-3 x4 fold pairs"
    );
    assert_eq!(
        rows[6].0, "minecraft:potion",
        "the id row the attribute block AFTER the walk wears"
    );
}

/// THE POTION KIND'S OWN ATTRIBUTE ROWS (the dev-200 round — the
/// addPotionTooltip walk's own tail the pairs the collect wears: the
/// createModifiers the amount*(amp+1) law, the whenDrank block the
/// blank line the header the DARK_PURPLE keeps, the plus/take rows
/// the x100 scale the DecimalFormat("#.##") fold, the ink the BLUE
/// the RED the operation's own sign answers).
#[test]
fn the_potion_kinds_the_attribute_rows_the_jar_wears() {
    // THE TABLES' OWN SHAPES (the 12 registrations the bootstrap
    // wears, the 8 names the walk's own rows the display serves)
    assert_eq!(
        crate::brewing_sim::MOB_EFFECT_ATTRIBUTE_MODIFIERS.len(),
        12,
        "the 12 addAttributeModifier rows the clinit pairs"
    );
    assert_eq!(crate::brewing_sim::ATTRIBUTE_NAMES.len(), 8);
    // THE REGISTRATION SPOTS (the (attr, op, amount) triples the
    // ldc2_w doubles the float-promoted values wear, the dconst_1
    // rows the 1.0 the pc pairing answers)
    let mods = |n: &str| {
        crate::brewing_sim::MOB_EFFECT_ATTRIBUTE_MODIFIERS
            .iter()
            .find(|(e, _)| *e == n)
            .map(|(_, m)| *m)
            .unwrap_or(&[])
    };
    assert_eq!(
        mods("speed"),
        &[("movement_speed", 2, 0.20000000298023224)],
        "the +20% movement-speed lane the MULTIPLIED_TOTAL op-2 wears"
    );
    assert_eq!(
        mods("slowness"),
        &[("movement_speed", 2, -0.15000000596046448)]
    );
    assert_eq!(mods("haste"), &[("attack_speed", 2, 0.10000000149011612)]);
    assert_eq!(
        mods("mining_fatigue"),
        &[("attack_speed", 2, -0.10000000149011612)]
    );
    assert_eq!(mods("strength"), &[("attack_damage", 0, 3.0)]);
    assert_eq!(mods("jump_boost"), &[("safe_fall_distance", 0, 1.0)]);
    assert_eq!(
        mods("invisibility"),
        &[("waypoint_transmit_range", 2, -1.0)],
        "the waypoint hide the -1.0 the MULTIPLIED_TOTAL wears"
    );
    assert_eq!(mods("weakness"), &[("attack_damage", 0, -4.0)]);
    assert_eq!(mods("health_boost"), &[("max_health", 0, 4.0)]);
    assert_eq!(mods("absorption"), &[("max_absorption", 0, 4.0)]);
    assert_eq!(mods("luck"), &[("luck", 0, 1.0)]);
    assert_eq!(mods("unluck"), &[("luck", 0, -1.0)]);
    // THE MODIFIER-FREE EFFECTS (the night_vision the regeneration
    // rows the walk's own silence keeps — the find the miss the
    // empty slice answers)
    assert!(mods("night_vision").is_empty());
    assert!(mods("resistance").is_empty());
    assert!(mods("fire_resistance").is_empty());
    // THE NAME SPOTS (the description-id tails the lang rows pair)
    let name = |n: &str| {
        crate::brewing_sim::ATTRIBUTE_NAMES
            .iter()
            .find(|(s, _)| *s == n)
            .unwrap()
            .1
    };
    assert_eq!(name("movement_speed"), "Speed");
    assert_eq!(name("attack_speed"), "Attack Speed");
    assert_eq!(name("attack_damage"), "Attack Damage");
    assert_eq!(name("safe_fall_distance"), "Safe Fall Distance");
    assert_eq!(name("waypoint_transmit_range"), "Waypoint Transmit Range");
    assert_eq!(name("max_health"), "Max Health");
    assert_eq!(name("max_absorption"), "Max Absorption");
    assert_eq!(name("luck"), "Luck");
    // THE FORMAT LAW (the DecimalFormat("#.##") fold — the trailing
    // zeros the pattern never serves, the sub-unity rows the 0 keeps)
    assert_eq!(App::format_attr_amount(20.000000298023224), "20");
    assert_eq!(App::format_attr_amount(15.000000596046448), "15");
    assert_eq!(App::format_attr_amount(60.00000238418579), "60");
    assert_eq!(App::format_attr_amount(90.00000357627869), "90");
    assert_eq!(App::format_attr_amount(100.0), "100");
    assert_eq!(App::format_attr_amount(3.0), "3");
    assert_eq!(App::format_attr_amount(4.0), "4");
    assert_eq!(App::format_attr_amount(1.0), "1");
    assert_eq!(App::format_attr_amount(0.2), "0.2");
    assert_eq!(App::format_attr_amount(0.25), "0.25");
    // THE WALK'S OWN ROWS (the base x (amp+1) the Template.create
    // law scales, the x100 the MULTIPLIED lane folds, the sign the
    // plus/take arms the ink answers)
    let blue = [0.3333, 0.3333, 1.0];
    let red = [1.0, 0.3333, 0.3333];
    let purple = [0.6667, 0.0, 0.6667];
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_SWIFTNESS);
    assert_eq!(
        rows.len(),
        3,
        "the blank the header the one row the pair collect wears"
    );
    assert_eq!(rows[0].0, "", "the blank line the block head wears");
    assert_eq!(rows[0].1, WHITE);
    assert_eq!(rows[1].0, "When Applied:");
    assert_eq!(rows[1].1, purple, "the DARK_PURPLE header ink wears");
    assert_eq!(
        rows[2].0, "+20% Speed",
        "the base 0.2 the x100 the %% fold pairs"
    );
    assert_eq!(rows[2].1, blue);
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_STRONG_SWIFTNESS);
    assert_eq!(
        rows[2].0, "+40% Speed",
        "the amp-1 the x2 fold the walk wears"
    );
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_SLOWNESS);
    assert_eq!(
        rows[2].0, "-15% Speed",
        "the negative the take arm the abs fold pairs"
    );
    assert_eq!(rows[2].1, red, "the take rows the RED ink wears");
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_STRONG_SLOWNESS);
    assert_eq!(rows[2].0, "-60% Speed", "the amp-3 the x4 fold pairs");
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_LEAPING);
    assert_eq!(
        rows[2].0, "+1 Safe Fall Distance",
        "the ADD_VALUE the raw lane keeps"
    );
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_STRONG_LEAPING);
    assert_eq!(
        rows[2].0, "+2 Safe Fall Distance",
        "the amp-1 the x2 the ADD_VALUE pairs"
    );
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_STRENGTH);
    assert_eq!(
        rows[2].0, "+3 Attack Damage",
        "the plus.0 the no-percent shape wears"
    );
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_STRONG_STRENGTH);
    assert_eq!(
        rows[2].0, "+6 Attack Damage",
        "the amp-1 the strength x2 pairs"
    );
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_WEAKNESS);
    assert_eq!(
        rows[2].0, "-4 Attack Damage",
        "the weakness the take.0 pairs"
    );
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_TURTLE_MASTER);
    assert_eq!(
        rows.len(),
        3,
        "the head pair the ONE row the resistance's own silence keeps"
    );
    assert_eq!(
        rows[2].0, "-60% Speed",
        "the slowness amp-3 the x4 fold the resistance never speaks"
    );
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_STRONG_TURTLE_MASTER);
    assert_eq!(rows[2].0, "-90% Speed", "the amp-5 the x6 fold pairs");
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_LUCK);
    assert_eq!(rows[2].0, "+1 Luck", "the dconst_1 base the 1.0 pairs");
    let rows = App::potion_kind_attribute_rows(crate::brewing_sim::POTION_INVISIBILITY);
    assert_eq!(
        rows[2].0, "-100% Waypoint Transmit Range",
        "the -1.0 x100 the waypoint row wears"
    );
    // THE MODIFIER-FREE KINDS (the night_vision the water rows the
    // empty vec the pairs' own gate keeps)
    assert!(App::potion_kind_attribute_rows(crate::brewing_sim::POTION_NIGHT_VISION).is_empty());
    assert!(App::potion_kind_attribute_rows(crate::brewing_sim::POTION_WATER).is_empty());
    // THE BOUNDS (the unknown kind the empty vec keeps)
    assert!(App::potion_kind_attribute_rows(-1).is_empty());
    assert!(App::potion_kind_attribute_rows(999).is_empty());
    // THE TRANSLATABLE'S OWN DECOMPOSE (the %s the arg the %% the
    // literal percent the escape keeps)
    assert_eq!(App::tr_fill("+%s%% %s", &["20", "Speed"]), "+20% Speed");
    assert_eq!(
        App::tr_fill("+%s %s", &["3", "Attack Damage"]),
        "+3 Attack Damage"
    );
    assert_eq!(App::tr_fill("no args here", &[]), "no args here");
    // THE KIND TABLE'S OWN REACH (the health_boost the absorption
    // rows the registry registers — the 46 potion kinds the table
    // serves NEVER carry them [the vanilla potions the walk banks —
    // the golden apple's own lanes the other seats keep], the kind
    // lane the modifier rows the speed/slowness/leaping/strength/
    // weakness/turtle/luck/invisibility seats wear, the haste the
    // mining_fatigue the unluck rows no potion kind carries)
    let carried: Vec<&str> = (0..46i32)
        .filter_map(|k| {
            let rows = App::potion_kind_attribute_rows(k);
            if rows.is_empty() {
                None
            } else {
                Some(crate::brewing_sim::POTION_KIND_BASE_NAMES[k as usize])
            }
        })
        .collect();
    assert_eq!(
        carried,
        vec![
            "invisibility",
            "invisibility",
            "leaping",
            "leaping",
            "leaping",
            "swiftness",
            "swiftness",
            "swiftness",
            "slowness",
            "slowness",
            "slowness",
            "turtle_master",
            "turtle_master",
            "turtle_master",
            "strength",
            "strength",
            "strength",
            "weakness",
            "weakness",
            "luck",
        ],
        "the 20 kind seats the pairs the collect wears — the haste/mining_fatigue/unluck/health_boost/absorption rows no potion kind carries"
    );
}

/// THE KIND BANK'S OWN WIRE SEAT (the autopush-343 round — the
/// window-0 projection's kinds the bank the tooltip lanes consult
/// re-bank): the main grid the identity the seats wear, the hotbar
/// row the menu ids 36..45 the bank's 0..8 wear, the kindless tail
/// the NONE keeps — and the OFFLINE world the wire never feeds (the
/// lane's own precondition the gate keeps, the local truth the bank
/// retains).
#[test]
fn the_live_potion_kinds_remap_the_bank_the_store_wears() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the projection's kinds: menu id 9 the strong swiftness seat
    // (the main grid's identity), menu id 40 the long night vision
    // row (the hotbar's own seat the bank's index 4 wears), the rest
    // the NONE the kindless stacks keep
    let mut kinds = [crate::brewing_sim::POTION_KIND_NONE; crate::world_view::INVENTORY_MENU_SLOTS];
    kinds[9] = crate::brewing_sim::POTION_STRONG_SWIFTNESS;
    kinds[40] = crate::brewing_sim::POTION_LONG_NIGHT_VISION;
    app.live_potion_kinds_updated(kinds);
    assert_eq!(
        app.items_potion_kinds[9],
        crate::brewing_sim::POTION_STRONG_SWIFTNESS,
        "the main grid the identity the seats wear"
    );
    assert_eq!(
        app.items_potion_kinds[4],
        crate::brewing_sim::POTION_LONG_NIGHT_VISION,
        "the hotbar row the menu id 40 the bank's seat 4 wears"
    );
    assert_eq!(
        app.items_potion_kinds[0],
        crate::brewing_sim::POTION_KIND_NONE,
        "the kindless tail the NONE keeps"
    );
    // THE OFFLINE GATE (the lane's own precondition): the offline
    // world the wire never feeds — the bank the local truth keeps
    let mut offline = App::new(3);
    offline.enter_world();
    let banked = offline.items_potion_kinds;
    offline.live_potion_kinds_updated(kinds);
    assert_eq!(
        offline.items_potion_kinds, banked,
        "the offline world the lane's gate refuses"
    );
}

/// THE CARRIED CURSOR'S OWN KIND SEAT (the autopush-344 round — the
/// Set Content packet's carriedItem the patch the cursor's own
/// extraction the carried render consults): the wire's kind the
/// wholesale overwrite banks (the re-bank the newest word wins), the
/// kindless cursor the NONE keeps — and the OFFLINE world the wire
/// never feeds (the lane's own precondition the gate keeps, the local
/// truth the bank retains).
#[test]
fn the_live_carried_potion_kind_bank_the_lane_wears() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the wire's kind: the carried potion the strong swiftness seat
    // answers (the bootstrap row the projection's own test wears)
    app.live_carried_potion_kind_updated(crate::brewing_sim::POTION_STRONG_SWIFTNESS);
    assert_eq!(
        app.live_carried_potion_kind,
        crate::brewing_sim::POTION_STRONG_SWIFTNESS,
        "the carried potion the wire's kind banks"
    );
    // the kindless cursor: the plain stack the wire re-speaks — the
    // wholesale overwrite the NONE re-banks
    app.live_carried_potion_kind_updated(crate::brewing_sim::POTION_KIND_NONE);
    assert_eq!(
        app.live_carried_potion_kind,
        crate::brewing_sim::POTION_KIND_NONE,
        "the kindless cursor the overwrite re-banks"
    );
    // THE OFFLINE GATE (the lane's own precondition): the offline
    // world the wire never feeds — the bank the local truth keeps
    let mut offline = App::new(3);
    offline.enter_world();
    offline.live_carried_potion_kind_updated(crate::brewing_sim::POTION_STRONG_SWIFTNESS);
    assert_eq!(
        offline.live_carried_potion_kind,
        crate::brewing_sim::POTION_KIND_NONE,
        "the offline world the lane's gate refuses"
    );
}

/// THE BELIEVED KIND'S OWN RIDE (the autopush-345 round — the brewing
/// ride's own law the PICKUP walk re-wears): the four arms the PICKUP
/// table speaks the kinds move — the TAKE the seat's kind the cursor
/// wears [the whole take the seat clears], the PLACE the cursor's
/// kind the seat wears, the SWAP the pairs exchange, the MERGE the
/// seat's own identity keeps [the no-op the full seat answers] — and
/// the WALL seats (the `None` the craft rows wear — the offhand the
/// 346 round the bank's 36 re-seats) the cursor's clear
/// alone answers. The seat map the bank's own numbering re-keys (the
/// main grid the identity, the hotbar row the bank's 0..8, the walls
/// the None keeps).
#[test]
fn the_pickup_ride_the_kinds_the_arms_wear() {
    // THE SEAT MAP (the 343 remap the ride re-keys)
    assert_eq!(kind_bank_seat(9), Some(9), "the main grid the identity");
    assert_eq!(kind_bank_seat(40), Some(4), "the hotbar row the bank's 4");
    assert_eq!(
        kind_bank_seat(45),
        Some(36),
        "the offhand the bank's 36 wears (the 346 re-seat)"
    );
    assert_eq!(kind_bank_seat(3), None, "the craft region the wall keeps");
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    let potion = mc_registry::item_id_by_name("potion").expect("the potion the table keeps");
    let kind_a = crate::brewing_sim::POTION_STRONG_SWIFTNESS;
    let kind_b = crate::brewing_sim::POTION_LONG_NIGHT_VISION;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    // THE TAKE (button 0, empty cursor over a kinded seat): the kind
    // the cursor wears, the seat the NONE clears, the packet the
    // pickup input rides
    let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    slots[9] = (potion, 1);
    let mut carried = (0, 0);
    let mut hotbar = [(0, 0); 9];
    let mut seat_kind = kind_a;
    let mut carried_kind = none;
    wire_inventory_pickup(
        &tx,
        &mut slots,
        &mut carried,
        &mut hotbar,
        9,
        0,
        1,
        Some(&mut seat_kind),
        &mut carried_kind,
    );
    assert_eq!(slots[9], (0, 0), "the take the seat empties");
    assert_eq!(carried, (potion, 1), "the take the cursor wears the stack");
    assert_eq!(seat_kind, none, "the whole take the seat's kind clears");
    assert_eq!(carried_kind, kind_a, "the take the kind the cursor wears");
    match rx.try_recv() {
        Ok(crate::world_view::ContainerCommand::Click(c)) => {
            assert_eq!(c.slot, 9, "the take the menu id rides");
            assert_eq!(
                c.input,
                crate::world_view::ContainerInput::Pickup,
                "the take the PICKUP input speaks"
            );
        }
        _ => panic!("the take the packet rides"),
    }
    // THE PLACE (the kinded cursor over the empty seat): the kind the
    // seat wears, the cursor the NONE clears
    wire_inventory_pickup(
        &tx,
        &mut slots,
        &mut carried,
        &mut hotbar,
        9,
        0,
        2,
        Some(&mut seat_kind),
        &mut carried_kind,
    );
    assert_eq!(slots[9], (potion, 1), "the place the seat wears the stack");
    assert_eq!(carried, (0, 0), "the place the cursor empties");
    assert_eq!(seat_kind, kind_a, "the place the kind the seat wears");
    assert_eq!(carried_kind, none, "the place the cursor's kind clears");
    // THE SWAP (two DIFFERENT kinded bottles exchange — the potion
    // and the splash potion the max-stack-1 law never merges): the
    // pairs the kinds exchange
    let splash =
        mc_registry::item_id_by_name("splash_potion").expect("the splash potion the table keeps");
    slots[10] = (splash, 1);
    let mut seat_b = kind_b;
    let mut carried_a = kind_a;
    let mut carried = (potion, 1);
    wire_inventory_pickup(
        &tx,
        &mut slots,
        &mut carried,
        &mut hotbar,
        10,
        0,
        3,
        Some(&mut seat_b),
        &mut carried_a,
    );
    assert_eq!(slots[10], (potion, 1), "the swap the seat wears the potion");
    assert_eq!(carried, (splash, 1), "the swap the cursor wears the splash");
    assert_eq!(seat_b, kind_a, "the swap the cursor's kind lands");
    assert_eq!(carried_a, kind_b, "the swap the seat's kind rides");
    // THE MERGE (the same-id non-potion with room): the seat's own
    // identity keeps, the cursor's remainder its own kind keeps
    let dirt = mc_registry::item_id_by_name("dirt").expect("the dirt the table keeps");
    let mut slots2 = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    slots2[11] = (dirt, 10);
    let mut carried2 = (dirt, 20);
    let mut seat_none = none;
    let mut carried_none = none;
    wire_inventory_pickup(
        &tx,
        &mut slots2,
        &mut carried2,
        &mut hotbar,
        11,
        0,
        4,
        Some(&mut seat_none),
        &mut carried_none,
    );
    assert_eq!(slots2[11], (dirt, 30), "the merge the seat grows");
    assert_eq!(carried2, (0, 0), "the merge the cursor empties");
    assert_eq!(seat_none, none, "the kindless merge the seat keeps");
    assert_eq!(carried_none, none, "the kindless merge the cursor keeps");
    // THE WALL (menu 45, the `None` seat): the place the cursor's
    // clear alone answers, the take the honest NONE the wall keeps
    let mut slots3 = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    let mut carried3 = (potion, 1);
    let mut carried_wall = kind_a;
    wire_inventory_pickup(
        &tx,
        &mut slots3,
        &mut carried3,
        &mut hotbar,
        45,
        0,
        5,
        None,
        &mut carried_wall,
    );
    assert_eq!(slots3[45], (potion, 1), "the wall the place the seat wears");
    assert_eq!(carried3, (0, 0), "the wall the place the cursor empties");
    assert_eq!(
        carried_wall, none,
        "the wall the place the kind leaves with the stack"
    );
}

/// THE THROW'S OWN KIND RIDE (the autopush-345 round — the drop the
/// kind the cursor wore): the whole throw the kind the entity never
/// carries the cursor's bank clears, the split the remainder's own
/// kind keeps, the `None` the kindless windows the wall keeps silent.
#[test]
fn the_throw_ride_the_cursor_kind_the_drop_wears() {
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let kind_a = crate::brewing_sim::POTION_STRONG_SWIFTNESS;
    // THE WHOLE THROW (button 0): the kind the bank clears
    let mut carried = (745, 1);
    let mut carried_kind = kind_a;
    wire_throw_outside(&tx, &mut carried, 0, 0, 1, Some(&mut carried_kind));
    assert_eq!(carried, (0, 0), "the whole throw the cursor empties");
    assert_eq!(carried_kind, none, "the whole throw the kind clears");
    // THE SPLIT (button 1, count 2): the remainder's own kind keeps
    let mut carried2 = (745, 2);
    let mut carried_kind2 = kind_a;
    wire_throw_outside(&tx, &mut carried2, 1, 0, 2, Some(&mut carried_kind2));
    assert_eq!(carried2, (745, 1), "the split the remainder keeps");
    assert_eq!(
        carried_kind2, kind_a,
        "the split the remainder's own kind keeps"
    );
    // THE KINDLESS WINDOW (the `None` the chest/table lanes wear):
    // the walk the wall keeps silent
    let mut carried3 = (745, 1);
    wire_throw_outside(&tx, &mut carried3, 0, 5, 3, None);
    assert_eq!(carried3, (0, 0), "the kindless throw the cursor empties");
}
/// THE OFFHAND'S OWN KIND SEAT (the autopush-346 round — the wire's
/// kinds[45] the bank the offhand pickup/tooltip/icon lanes consult
/// re-seats): the seat map the 45 the bank's 36 answers, the wire's
/// wholesale overwrite the 37TH seat re-banks, the PICKUP ride the
/// offhand's arms the kinds move (the take the kind the cursor wears
/// + the seat clears, the place the cursor's kind the seat lands, the
/// swap the pairs exchange) — and the OFFLINE world the wire never
/// feeds (the lane's own precondition the gate keeps, the local truth
/// the bank retains).
#[test]
fn the_offhand_seat_the_kinds_the_bank_wears() {
    // THE SEAT MAP (the 346 round the 45 re-keys — the craft region
    // the wall keeps)
    assert_eq!(
        kind_bank_seat(45),
        Some(36),
        "the offhand the bank's 36 wears"
    );
    assert_eq!(kind_bank_seat(4), None, "the craft region the wall keeps");
    // THE WIRE SEAT (the projection's kinds[45] the bank re-seats —
    // the wholesale overwrite the 37TH seat re-banks)
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let mut kinds = [crate::brewing_sim::POTION_KIND_NONE; crate::world_view::INVENTORY_MENU_SLOTS];
    kinds[45] = crate::brewing_sim::POTION_STRONG_SWIFTNESS;
    kinds[9] = crate::brewing_sim::POTION_LONG_NIGHT_VISION;
    app.live_potion_kinds_updated(kinds);
    assert_eq!(
        app.items_potion_kinds[36],
        crate::brewing_sim::POTION_STRONG_SWIFTNESS,
        "the offhand's kind the bank's 36 wears"
    );
    assert_eq!(
        app.items_potion_kinds[9],
        crate::brewing_sim::POTION_LONG_NIGHT_VISION,
        "the main grid the identity the seats keep"
    );
    // THE OFFLINE GATE (the lane's own precondition): the offline
    // world the wire never feeds — the bank the local truth keeps
    let mut offline = App::new(3);
    offline.enter_world();
    let banked = offline.items_potion_kinds;
    offline.live_potion_kinds_updated(kinds);
    assert_eq!(
        offline.items_potion_kinds, banked,
        "the offline world the lane's gate refuses"
    );
    // THE OFFHAND'S OWN RIDE (menu 45 the banked seat the arms answer)
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    let potion = mc_registry::item_id_by_name("potion").expect("the potion the table keeps");
    let kind_a = crate::brewing_sim::POTION_STRONG_SWIFTNESS;
    let kind_b = crate::brewing_sim::POTION_LONG_NIGHT_VISION;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    let mut carried = (0, 0);
    let mut hotbar = [(0, 0); 9];
    let mut offhand_kind = kind_a;
    let mut carried_kind = none;
    // THE TAKE (empty cursor over the kinded offhand): the kind the
    // cursor wears, the seat the NONE clears
    slots[45] = (potion, 1);
    wire_inventory_pickup(
        &tx,
        &mut slots,
        &mut carried,
        &mut hotbar,
        45,
        0,
        1,
        Some(&mut offhand_kind),
        &mut carried_kind,
    );
    assert_eq!(slots[45], (0, 0), "the take the seat empties");
    assert_eq!(carried, (potion, 1), "the take the cursor wears the stack");
    assert_eq!(offhand_kind, none, "the whole take the seat's kind clears");
    assert_eq!(carried_kind, kind_a, "the take the kind the cursor wears");
    // THE PLACE (the kinded cursor over the empty offhand): the kind
    // the seat wears, the cursor the NONE clears
    wire_inventory_pickup(
        &tx,
        &mut slots,
        &mut carried,
        &mut hotbar,
        45,
        0,
        2,
        Some(&mut offhand_kind),
        &mut carried_kind,
    );
    assert_eq!(slots[45], (potion, 1), "the place the seat wears the stack");
    assert_eq!(carried, (0, 0), "the place the cursor empties");
    assert_eq!(offhand_kind, kind_a, "the place the kind the seat wears");
    assert_eq!(carried_kind, none, "the place the cursor's kind clears");
    // THE SWAP (the splash exchange — the max-stack-1 law the merge
    // never walks): the pairs the kinds exchange
    let splash =
        mc_registry::item_id_by_name("splash_potion").expect("the splash potion the table keeps");
    slots[45] = (splash, 1);
    let mut carried2 = (potion, 1);
    let mut carried_kind2 = kind_a;
    let mut offhand_kind2 = kind_b;
    wire_inventory_pickup(
        &tx,
        &mut slots,
        &mut carried2,
        &mut hotbar,
        45,
        0,
        3,
        Some(&mut offhand_kind2),
        &mut carried_kind2,
    );
    assert_eq!(slots[45], (potion, 1), "the swap the seat wears the potion");
    assert_eq!(
        carried2,
        (splash, 1),
        "the swap the cursor wears the splash"
    );
    assert_eq!(offhand_kind2, kind_a, "the swap the cursor's kind lands");
    assert_eq!(carried_kind2, kind_b, "the swap the seat's kind rides");
}

/// THE SWAP'S OWN KIND RIDE (the dev-201 round — the F key's ONE
/// trade the bank's seats the same walk exchanges): the stack the
/// kind follows — the offline bank's hotbar seat and the offhand's
/// 36 the pairs exchange, the live bank the same trade the
/// prediction's own truth wears (the sub-echo window the wholesale
/// refresh re-banks, the 343/345 law the click lanes keep).
#[test]
fn the_swap_ride_the_kinds_the_seats_exchange() {
    let dirt = mc_registry::item_id_by_name("dirt").expect("the dirt the table keeps");
    let potion = mc_registry::item_id_by_name("potion").expect("the potion the table keeps");
    let kind_a = crate::brewing_sim::POTION_STRONG_SWIFTNESS;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    // THE OFFLINE TRADE (the gameplay inventory the truth wears)
    let mut app = App::new(3);
    app.enter_world();
    app.inventory.items[0] = (dirt, 10);
    app.inventory.offhand[0] = (potion, 1);
    app.items_potion_kinds[0] = none;
    app.items_potion_kinds[OFFHAND_KIND_SEAT] = kind_a;
    app.on_swap_offhand_key();
    assert_eq!(
        app.inventory.items[0],
        (potion, 1),
        "the offhand the hand wears"
    );
    assert_eq!(
        app.inventory.offhand[0],
        (dirt, 10),
        "the hand the offhand keeps"
    );
    assert_eq!(
        app.items_potion_kinds[0], kind_a,
        "the potion's kind the hotbar seat rides"
    );
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], none,
        "the offhand seat the plain's NONE wears"
    );
    // THE ROUND TRIP (the re-swap the pairs restore)
    app.on_swap_offhand_key();
    assert_eq!(app.items_potion_kinds[0], none, "the hotbar seat restores");
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], kind_a,
        "the offhand seat restores"
    );
    // THE LIVE TRADE (the belief + the bank the prediction's own
    // truth wears — the wholesale refresh the next re-bank answers)
    let mut live = App::new(3);
    live.open(ScreenKind::Connect);
    live.net_world = Some(air_net_world(2, 0));
    live.enter_network_world(None, Skybox::Overworld, None, None);
    let (tx, _rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    live.attach_live_clicks(tx);
    live.live_inv_slots[36] = (dirt, 10);
    live.live_inv_slots[45] = (potion, 1);
    live.items_potion_kinds[0] = none;
    live.items_potion_kinds[OFFHAND_KIND_SEAT] = kind_a;
    live.on_swap_offhand_key();
    assert_eq!(
        live.live_inv_slots[36],
        (potion, 1),
        "the belief the trade wears"
    );
    assert_eq!(
        live.live_inv_slots[45],
        (dirt, 10),
        "the belief the offhand keeps"
    );
    assert_eq!(
        live.items_potion_kinds[0], kind_a,
        "the live bank the hotbar seat rides"
    );
    assert_eq!(
        live.items_potion_kinds[OFFHAND_KIND_SEAT], none,
        "the live bank the offhand seat clears"
    );
}

/// THE Q-DROP'S OWN SEAT CLEAR (the dev-201 round — the emptied
/// store/offhand rows the bank the dev-199 walk's own law the
/// inventory screen's own Q re-answers): the offline lane the
/// emptied seat the bank clears, the remainder the banked kind
/// keeps [the <= 0 gate the clear's own law wears]; the wire lane
/// the belief's emptied seat the mapped bank seat the caller-side
/// ride clears — the craft region the wall the None keeps, the
/// packet the THROW input rides.
#[test]
fn the_qdrop_ride_the_seats_the_bank_clears() {
    let dirt = mc_registry::item_id_by_name("dirt").expect("the dirt the table keeps");
    let potion = mc_registry::item_id_by_name("potion").expect("the potion the table keeps");
    let kind_a = crate::brewing_sim::POTION_STRONG_SWIFTNESS;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    // THE OFFLINE STORE ROW (the last item the seat empties — the
    // potion the max-stack-1 law the one off the empty answers)
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    app.inventory.items[5] = (potion, 1);
    app.items_potion_kinds[5] = kind_a;
    app.mouse_pos = Some((336, 246)); // store 5's own rect
    app.on_inventory_drop_key(false);
    assert_eq!(
        app.inventory.items[5],
        (potion, 0),
        "the one off the seat empties"
    );
    assert_eq!(
        app.items_potion_kinds[5], none,
        "the emptied seat the bank clears"
    );
    // THE PLAIN REMAINDER (the partial the seat keeps — the banked
    // kind the <= 0 gate the clear refuses, the whole take the clear
    // answers)
    app.inventory.items[6] = (dirt, 7);
    app.items_potion_kinds[6] = kind_a;
    app.mouse_pos = Some((354, 246)); // store 6's own rect
    app.on_inventory_drop_key(false);
    assert_eq!(
        app.inventory.items[6],
        (dirt, 6),
        "the one off the seat keeps"
    );
    assert_eq!(
        app.items_potion_kinds[6], kind_a,
        "the remainder the banked kind keeps"
    );
    app.on_inventory_drop_key(true);
    assert_eq!(app.inventory.items[6], (0, 0), "the whole take empties");
    assert_eq!(
        app.items_potion_kinds[6], none,
        "the emptied seat the bank clears"
    );
    // THE OFFHAND'S OWN SEAT (the shield cell the Q throws — the
    // emptied offhand the bank's 36 clears)
    app.inventory.offhand[0] = (potion, 1);
    app.items_potion_kinds[OFFHAND_KIND_SEAT] = kind_a;
    app.mouse_pos = Some((317, 167)); // the offhand's own rect
    app.on_inventory_drop_key(false);
    assert_eq!(
        app.inventory.offhand[0],
        (potion, 0),
        "the one off the offhand empties"
    );
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], none,
        "the emptied offhand the bank clears"
    );
    // THE WIRE LANE (the belief's emptied seat the mapped bank seat
    // the caller-side ride clears — the packet the THROW input rides)
    let mut live = App::new(3);
    live.open(ScreenKind::Connect);
    live.net_world = Some(air_net_world(2, 0));
    live.enter_network_world(None, Skybox::Overworld, None, None);
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    live.attach_live_clicks(tx);
    live.open(ScreenKind::Inventory);
    live.inv_panel = inventory_panel_origin(640, 360);
    live.live_inv_slots[45] = (potion, 1);
    live.items_potion_kinds[OFFHAND_KIND_SEAT] = kind_a;
    live.mouse_pos = Some((317, 167));
    live.on_inventory_drop_key(false);
    assert_eq!(
        live.live_inv_slots[45],
        (0, 0),
        "the belief the throw empties"
    );
    assert_eq!(
        live.items_potion_kinds[OFFHAND_KIND_SEAT], none,
        "the wire lane the bank clears"
    );
    match rx.try_recv().expect("the THROW packet rode") {
        crate::world_view::ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 45, "the offhand's own menu id");
            assert_eq!(
                c.input,
                crate::world_view::ContainerInput::Throw,
                "the THROW input the packet wears"
            );
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE CRAFT REGION'S WALL (the hover the map's None keeps — the
    // banked seat the clear never answers)
    live.live_inv_slots[1] = (potion, 1);
    live.items_potion_kinds[0] = kind_a;
    let (cx, cy) = craft_slot_origin(live.inv_panel, 0);
    live.mouse_pos = Some((cx + 9, cy + 9));
    live.on_inventory_drop_key(false);
    assert_eq!(
        live.live_inv_slots[1],
        (0, 0),
        "the craft cell the throw empties"
    );
    assert_eq!(
        live.items_potion_kinds[0], kind_a,
        "the craft region the wall the bank keeps"
    );
}

/// THE WORLD Q-DROP'S OWN WIRE RIDE (the autopush-347 round — the
/// dev-201 refusal's own answer the handleKeybinds keyDrop lane
/// re-walks): the wire session's Q the LocalPlayer.drop walk speaks —
/// the belief's selected seat the removeFromSelected law wears (the
/// ONE off the remainder keeps, the whole take the seat empties),
/// the HUD row's belief the every-mutation law syncs, the emptied
/// seat the bank clears [the dev-199 walk's own law the partial the
/// banked kind keeps], the MAIN_HAND swing the success arm fires,
/// the 3-arg ctor's packet the sequence-0 lane rides — the drop
/// entity the server's own Add Entity lane answers (the client-side
/// spawn the prediction never duplicates), the SINK-LESS session the
/// honest refusal (the belief the walk never touches without its own
/// packet).
#[test]
fn the_world_qdrop_ride_the_wire_the_prediction_wears() {
    let potion = mc_registry::item_id_by_name("potion").expect("the potion the table keeps");
    let kind_a = crate::brewing_sim::POTION_STRONG_SWIFTNESS;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    // THE WIRE LANE (the belief + the bank the prediction's own truth
    // wears — the wholesale refresh the next re-bank answers)
    let mut live = App::new(3);
    live.open(ScreenKind::Connect);
    live.net_world = Some(air_net_world(2, 0));
    live.enter_network_world(None, Skybox::Overworld, None, None);
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    live.attach_live_clicks(tx);
    live.live_inv_slots[36] = (potion, 3);
    live.items_potion_kinds[0] = kind_a;
    // THE PLAIN Q (the ONE off the seat keeps — the drop the belief
    // speaks, the entity the server's own lane answers)
    live.on_drop_key(false);
    assert_eq!(
        live.live_inv_slots[36],
        (potion, 2),
        "the one off the belief wears"
    );
    assert_eq!(
        live.items_potion_kinds[0], kind_a,
        "the remainder the banked kind keeps"
    );
    assert_eq!(
        live.hotbar_slots[0],
        (potion, 2),
        "the HUD row's belief syncs"
    );
    assert!(live.swing.swinging, "the success arm the MAIN_HAND fires");
    match rx.try_recv().expect("the DROP packet rode") {
        crate::world_view::ContainerCommand::DropKey { whole_stack } => {
            assert!(!whole_stack, "the plain Q the ONE speaks");
        }
        other => panic!("expected the drop key, got {other:?}"),
    }
    // THE WHOLE TAKE (the Ctrl the seat empties — the bank the clear
    // answers, the held the unload wears)
    live.on_drop_key(true);
    assert_eq!(live.live_inv_slots[36], (0, 0), "the whole take empties");
    assert_eq!(
        live.items_potion_kinds[0], none,
        "the emptied seat the bank clears"
    );
    assert_eq!(live.hotbar_slots[0], (0, 0), "the HUD row the clear wears");
    assert_eq!(live.live_held, None, "the emptied hand the unload wears");
    match rx.try_recv().expect("the DROP-ALL packet rode") {
        crate::world_view::ContainerCommand::DropKey { whole_stack } => {
            assert!(whole_stack, "the Ctrl the WHOLE speaks");
        }
        other => panic!("expected the drop key, got {other:?}"),
    }
    // THE EMPTY SEAT (the silence — the removeFromSelected EMPTY law
    // the drop-false verdict keeps)
    live.on_drop_key(false);
    assert!(rx.try_recv().is_err(), "the empty seat the silence keeps");
    // THE SINK-LESS SESSION (the honest refusal — the belief the walk
    // never touches without its own packet)
    let mut bare = App::new(3);
    bare.open(ScreenKind::Connect);
    bare.net_world = Some(air_net_world(2, 0));
    bare.enter_network_world(None, Skybox::Overworld, None, None);
    bare.live_inv_slots[37] = (potion, 5);
    bare.selected_hotbar = 1;
    bare.on_drop_key(false);
    assert_eq!(
        bare.live_inv_slots[37],
        (potion, 5),
        "the sink-less belief the walk keeps whole"
    );
}

/// THE PICKER'S OWN WRAPPED KIND LANE (the autopush-348 round — the
/// wrapped seats the bank the survival rows read the same rides wear):
/// the wrapped offhand's tooltip the bank's [`OFFHAND_KIND_SEAT`] the
/// kinded name answers (the 346 law's own seat the picker's mirror
/// re-wears — the hovered shield cell the display name the effect
/// rows ride), the wrapped main row the same bank the seat's own
/// index reads, the ARMOR rows the kind never carries the plain name
/// the wall keeps, the PALETTE rows the no-component vanilla law the
/// kindless NONE keeps.
#[test]
fn the_picker_wrap_kind_the_bank_the_seats_rewear() {
    let potion = mc_registry::item_id_by_name("potion").expect("the potion the table keeps");
    let kind_a = crate::brewing_sim::POTION_STRONG_SWIFTNESS;
    let mut app = App::new(262);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    assert_eq!(app.screen, ScreenKind::Creative, "the creative arm opens");
    // THE WRAPPED TAB (the INVENTORY tab the wrapped seats serve —
    // the default seat the building blocks keep)
    let inv_tab = (0..CREATIVE_TABS.len())
        .find(|&i| CREATIVE_TABS[i].kind == CreativeTabKind::Inventory)
        .expect("the inventory tab the strip keeps");
    app.select_creative_tab(inv_tab);
    app.creative_panel = (100, 50);
    // THE WRAPPED OFFHAND (the bank's 36 the kinded name the tooltip
    // wears — the 346 law the same seat the picker's mirror re-wears)
    app.inventory.offhand[0] = (potion, 1);
    app.items_potion_kinds[OFFHAND_KIND_SEAT] = kind_a;
    let expected = App::potion_kind_display_name(kind_a);
    assert!(
        !expected.contains("potion.effect"),
        "the lang row the tr resolves (got {expected})"
    );
    let (id, rows) = app
        .creative_item_tooltip_hover(100 + 35 + 8, 50 + 20 + 8)
        .expect("the offhand seat the gate answers");
    assert_eq!(id, potion, "the offhand's own stack the gate banks");
    assert_eq!(
        rows[0],
        (expected.clone(), [1.0, 1.0, 1.0]),
        "the kinded name the wrapped offhand wears"
    );
    assert!(rows.len() > 1, "the effect rows the kind lane wears");
    // THE WRAPPED MAIN ROW (the seat's own index the same bank reads)
    app.inventory.items[9] = (potion, 1);
    app.items_potion_kinds[9] = kind_a;
    let (id, rows) = app
        .creative_item_tooltip_hover(100 + 9 + 8, 50 + 54 + 8)
        .expect("the wrapped main seat the gate answers");
    assert_eq!(id, potion, "the main row's own stack the gate banks");
    assert_eq!(
        rows[0],
        (expected.clone(), [1.0, 1.0, 1.0]),
        "the kinded name the wrapped main wears"
    );
    // THE ARMOR WALL (the kind never carries — the plain name the
    // row keeps whatever the bank's armor seats wore)
    app.inventory.armor[0] = (potion, 1);
    app.items_potion_kinds[5] = kind_a;
    let (id, rows) = app
        .creative_item_tooltip_hover(100 + 54 + 8, 50 + 6 + 8)
        .expect("the armor seat the gate answers");
    assert_eq!(id, potion, "the armor row's own stack the gate banks");
    assert_ne!(
        rows[0].0, expected,
        "the armor row the kind never carries — the plain name the wall keeps"
    );
    // THE PALETTE WALL (the no-component vanilla law the kindless
    // NONE keeps — the minted row the plain name speaks; the cell the
    // seat test's own law sets — the first page the wire order's
    // potion row never wears)
    app.select_creative_tab(0);
    app.creative_palette[2] = (potion, 1);
    let (id, rows) = app
        .creative_item_tooltip_hover(100 + 9 + 2 * 18 + 8, 50 + 18 + 8)
        .expect("the palette seat the gate answers");
    assert_eq!(id, potion, "the palette's own potion row");
    assert_ne!(
        rows[0].0, expected,
        "the palette row the no-component law keeps kindless"
    );
}

/// THE PICKER'S OWN STALE SEAT CLEAR (the autopush-349 round — the
/// creative Q's THROW arms + the bin's whole-clear the bank the
/// dev-201 walk's own law the inventory screen's own Q re-wears): the
/// emptied wrap row the bank clears, the partial the banked kind
/// keeps [the <= 0 gate], the emptied shield cell the bank's
/// [`OFFHAND_KIND_SEAT`] clears, the bottom store row the same law
/// the seat's own index answers, the bin's whole-clear the store
/// rows' 36 seats + the offhand seat the NONE wears — the stale kind
/// the later potion never wears.
#[test]
fn the_picker_qthrow_the_stale_seats_the_bank_clears() {
    let potion = mc_registry::item_id_by_name("potion").expect("the potion the table keeps");
    let kind_a = crate::brewing_sim::POTION_STRONG_SWIFTNESS;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(262);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    assert_eq!(app.screen, ScreenKind::Creative, "the creative arm opens");
    let inv_tab = (0..CREATIVE_TABS.len())
        .find(|&i| CREATIVE_TABS[i].kind == CreativeTabKind::Inventory)
        .expect("the inventory tab the strip keeps");
    app.select_creative_tab(inv_tab);
    app.creative_panel = (100, 50);
    // THE WRAP MAIN ROW (the whole take the seat empties + the bank
    // clears; the partial the banked kind keeps)
    app.inventory.items[12] = (potion, 2);
    app.items_potion_kinds[12] = kind_a;
    let (wx, wy) = (100 + 9 + 3 * 18 + 8, 50 + 54 + 8); // main-12's own rect
    app.mouse_pos = Some((wx, wy));
    app.on_creative_drop_key(false);
    assert_eq!(
        app.inventory.items[12],
        (potion, 1),
        "the one off the seat keeps"
    );
    assert_eq!(
        app.items_potion_kinds[12], kind_a,
        "the partial the banked kind keeps"
    );
    app.on_creative_drop_key(true);
    assert_eq!(app.inventory.items[12], (0, 0), "the whole take empties");
    assert_eq!(
        app.items_potion_kinds[12], none,
        "the emptied seat the bank clears"
    );
    // THE WRAP OFFHAND (the emptied shield cell the bank's 36 clears)
    app.inventory.offhand[0] = (potion, 1);
    app.items_potion_kinds[OFFHAND_KIND_SEAT] = kind_a;
    app.mouse_pos = Some((100 + 35 + 8, 50 + 20 + 8)); // the offhand's own rect
    app.on_creative_drop_key(true);
    assert_eq!(app.inventory.offhand[0], (0, 0), "the throw empties");
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], none,
        "the emptied offhand the bank clears"
    );
    // THE BOTTOM STORE ROW (the non-Inventory tab the same law the
    // seat's own index answers)
    app.select_creative_tab(0);
    app.inventory.items[4] = (potion, 1);
    app.items_potion_kinds[4] = kind_a;
    app.mouse_pos = Some((100 + 9 + 4 * 18 + 8, 50 + 112 + 8)); // the bottom row's rect
    app.on_creative_drop_key(true);
    assert_eq!(app.inventory.items[4], (0, 0), "the throw empties");
    assert_eq!(
        app.items_potion_kinds[4], none,
        "the emptied bottom row the bank clears"
    );
    // THE BIN'S WHOLE-CLEAR (the shift press the store rows' 36 seats
    // + the offhand seat the NONE wears — the stale kinds the whole
    // clear never leaves)
    app.select_creative_tab(inv_tab);
    app.items_potion_kinds[7] = kind_a;
    app.items_potion_kinds[20] = kind_a;
    app.items_potion_kinds[OFFHAND_KIND_SEAT] = kind_a;
    app.mouse_pos = Some((100 + 173 + 8, 50 + 112 + 8)); // the bin's own rect
    app.shift_down = true;
    app.click_creative(100 + 173 + 8, 50 + 112 + 8, 0);
    app.shift_down = false;
    assert_eq!(
        app.items_potion_kinds[7], none,
        "the hotbar seat the clear wears"
    );
    assert_eq!(
        app.items_potion_kinds[20], none,
        "the main seat the clear wears"
    );
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], none,
        "the offhand seat the clear wears"
    );
}

/// THE ANVIL'S OWN DOOR SEAT (the dev-202 round — the 26.2
/// createInputSlotDefinitions stream the bytecode dump banks —
/// scripts/jar_work/anvil_menu_202.txt the seats speak): the
/// transient-trio walk the origin answers, the input/result trio the
/// (27, 47)/(76, 47)/(134, 47) seats the ctor's stream pins, the
/// mirror rows the standard numbering keys, the NO-GATE law the
/// family banks. The REAL walk the offline door wears — the headless
/// frame the REAL rects the render lays.
#[test]
fn the_offline_anvil_the_door_the_trio_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::ANVIL);
    // THE DOOR'S OWN KIND GATE (the is_anvil verdict the mint's own
    // row answers)
    assert!(
        crate::world::is_anvil(crate::world::ANVIL),
        "the anvil kind the helper answers"
    );
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the empty hand the door's own lane keeps
    }
    app.open_anvil_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Anvil,
        "the anvil door the kind wears"
    );
    assert_eq!(app.anvil_slots, [(0, 0); 2], "the fresh trio the open lays");
    assert_eq!(app.anvil_result, (0, 0), "the result the fresh zero keeps");
    assert_eq!(app.anvil_mirror.len(), 36, "the mirror rows the sync keeps");
    assert!(
        app.anvil_name.is_empty() && app.anvil_cost == 0,
        "the field's fresh word the open lays"
    );
    // THE MIRROR'S OWN PROJECTION (the inventory IS — the main rows
    // the items 9..35 speak, the hotbar row the items 0..8)
    app.inventory.items[9] = (15, 2);
    app.inventory.items[3] = (7, 5);
    app.anvil_mirror_sync_from_inventory();
    assert_eq!(
        app.anvil_mirror[0],
        (15, 2),
        "the main head the items 9 answers"
    );
    assert_eq!(
        app.anvil_mirror[30],
        (7, 5),
        "the hotbar row the items 3 answer"
    );
    app.inventory.items[9] = (0, 0);
    app.inventory.items[3] = (0, 0);
    // THE REAL RECTS THE RENDER BANKS (one headless frame — the
    // render's own walk the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = ((320 - 176) / 2, (180 - 166) / 2);
    assert_eq!(
        app.anvil_panel, panel,
        "the centered panel the canvas keeps"
    );
    let seat = |slot: i16| -> (i32, i32) {
        app.anvil_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE 26.2 STREAM'S OWN SEATS (the createInputSlotDefinitions
    // banks: input1 (27, 47), input2 (76, 47) — the +49 x-pitch the
    // same row wears, the result (134, 47) the +58 keeps)
    let (i0x, i0y) = seat(0);
    let (i1x, i1y) = seat(1);
    let (rx, ry) = seat(2);
    assert_eq!(
        (i1x - i0x, i1y - i0y),
        (49, 0),
        "the input pair the ctor pitch banks"
    );
    assert_eq!(
        (rx - i0x, ry - i0y),
        (107, 0),
        "the result seat the ctor pitch banks"
    );
    // THE SEATS' OWN ANSWERS (each seat + the mirror head + the hotbar
    // tail the origin answers — the ONE walk the clicks share)
    assert_eq!(
        app.anvil_highlight_origin(i0x + 8, i0y + 8),
        Some((i0x, i0y)),
        "the input1 seat the origin answers"
    );
    assert_eq!(
        app.anvil_highlight_origin(rx + 8, ry + 8),
        Some((rx, ry)),
        "the result seat the origin answers"
    );
    let (mx, my) = seat(3);
    assert_eq!(
        app.anvil_highlight_origin(mx + 8, my + 8),
        Some((mx, my)),
        "the mirror head the origin answers"
    );
    let (hx, hy) = seat(38);
    assert_eq!(
        app.anvil_highlight_origin(hx + 8, hy + 8),
        Some((hx, hy)),
        "the hotbar tail the origin answers"
    );
    // THE NO-GATE LAW (the family's own verdict — the carried stack
    // the wall NEVER speaks)
    app.anvil_carried = (7, 3);
    assert_eq!(
        app.anvil_highlight_origin(i0x + 8, i0y + 8),
        Some((i0x, i0y)),
        "the carried stack the highlight never refuses"
    );
    app.anvil_carried = (0, 0);
    // THE CELL'S OWN EDGES (the strict 16x16 the walk answers)
    assert_eq!(
        app.anvil_highlight_origin(i0x + 16, i0y + 8),
        None,
        "the east edge the walk refuses"
    );
    assert_eq!(
        app.anvil_highlight_origin(i0x - 1, i0y + 8),
        None,
        "the west edge the walk refuses"
    );
    // THE WRONG SCREEN (the world view the highlight never wears)
    app.close_anvil(ScreenKind::World);
    assert_eq!(
        app.anvil_highlight_origin(i0x + 8, i0y + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE ANVIL'S OWN WALK (the dev-202 round — the createResult law the
/// fixed-alignment dump banks — scripts/jar_work/anvil_walk_202.txt
/// the honest arm serves): the rename arm the +1 the custom-set /
/// blank-remove laws wear, the equal word the no-op keeps, the
/// no-action walk the EMPTY/0 verdict answers, the incompatible pair
/// the same verdict keeps, the combine arm the anvil_cost levels sum,
/// the PURE-RENAME 39 cap + the 40 TOO-EXPENSIVE gate the result
/// empties, the REPAIR-COST bump the take's doubled law rides, the
/// TAKE the inputs consume + the xp charges + the onlyRenaming arm
/// the second item keeps.
#[test]
fn the_anvil_walk_the_rename_the_costs_wear() {
    let mut app = App::new(3);
    app.enter_world();
    // THE IRON SWORD'S OWN ID (the registry order the names speak)
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    // THE RENAME WALK (the plain stack + the typed word — the result
    // the +1 cost wears, the custom name the seat lane answers)
    app.anvil_slots[0] = (sword, 1);
    app.on_anvil_name_changed("Doom Bringer");
    assert_eq!(app.anvil_cost, 1, "the rename the +1 cost wears");
    assert_eq!(app.anvil_result, (sword, 1), "the copy the result keeps");
    assert_eq!(
        app.anvil_names[2].as_deref(),
        Some("Doom Bringer"),
        "the typed word the result seat banks"
    );
    assert!(app.anvil_only_renaming, "the pure rename the flag wears");
    // THE EQUAL WORD'S OWN NO-OP (the field's word the hover-name
    // equal the reset arm wears — the empty string the plain stack's
    // own law keeps: the no cost, the EMPTY result)
    app.on_anvil_name_changed("Doom Bringer");
    assert_eq!(
        app.anvil_cost, 1,
        "the equal word the walk never re-charges"
    );
    app.anvil_name.clear();
    app.anvil_create_result();
    assert_eq!(app.anvil_cost, 0, "the blank word the no-action keeps");
    assert_eq!(
        app.anvil_result,
        (0, 0),
        "the no-action walk the EMPTY wears"
    );
    // THE CUSTOM-NAME REMOVE ARM (the named seat + the blank word —
    // the +1 the flag wears, the result the plain copy answers; the
    // lane's own edit the slotsChanged recompute answers)
    app.anvil_names[0] = Some("Doom Bringer".to_string());
    app.anvil_name.clear();
    app.anvil_create_result();
    assert_eq!(app.anvil_cost, 1, "the remove arm the +1 wears");
    assert_eq!(
        app.anvil_names[2], None,
        "the blank word the remove arm answers"
    );
    app.anvil_names[0] = None;
    // THE INCOMPATIBLE PAIR (the sword + the dirt — the undamaged
    // stacks the material arm's EMPTY verdict the same answer wears)
    app.anvil_slots[1] = (dirt, 1);
    app.anvil_create_result();
    assert_eq!(
        app.anvil_result,
        (0, 0),
        "the incompatible pair the EMPTY wears"
    );
    assert_eq!(app.anvil_cost, 0, "the incompatible pair the 0 cost keeps");
    // THE COMBINE ARM (the enchanted book + the sword — the stored
    // pair the merge laws answer, the cost the anvilCost*level sum
    // wears; sharpness' own row the anvil_cost 1 speaks, the level 3
    // the new_level 3 the +3 cost banks)
    let sharp = mc_registry::generated::enchantments::enchantment_index("sharpness").unwrap();
    let book = mc_registry::item_id_by_name("enchanted_book").unwrap();
    app.anvil_slots[1] = (book, 1);
    app.anvil_enchants[1] = vec![(sharp as i32, 3)];
    app.anvil_create_result();
    assert_eq!(app.anvil_result, (sword, 1), "the combine the copy keeps");
    assert_eq!(
        app.anvil_cost, 3,
        "the combine the anvilCost*level sum wears"
    );
    assert_eq!(
        app.anvil_enchants[2],
        vec![(sharp as i32, 3)],
        "the stored pair the result store wears"
    );
    assert!(
        !app.anvil_only_renaming,
        "the combine the pure-rename flag never wears"
    );
    // THE TAKE WALK (the result the carried hands, the input 1 the
    // ALWAYS-clear arm empties, the xp the cost charges, the repair
    // bump the fresh-entry law doubles)
    app.xp.level = 30;
    app.anvil_take_result();
    assert_eq!(
        app.anvil_carried,
        (sword, 1),
        "the result the carried hands"
    );
    assert_eq!(
        app.anvil_names[39].as_deref(),
        None,
        "the unnamed carry the None keeps"
    );
    assert_eq!(
        app.anvil_enchants[39],
        vec![(sharp as i32, 3)],
        "the store the carried hands"
    );
    assert_eq!(app.anvil_slots, [(0, 0); 2], "the inputs the take consumes");
    assert_eq!(app.xp.level, 27, "the xp the cost charges");
    assert_eq!(app.anvil_cost, 0, "the cost the zero reset wears");
    // THE RIDE-HOME (the enchanted carry the merge lane never rides —
    // the first-empty placement the lanes answer)
    app.close_anvil(ScreenKind::World);
    let store_idx = app
        .inventory
        .items
        .iter()
        .position(|&s| s == (sword, 1))
        .expect("the carried stack the first-empty seat wears");
    assert_eq!(
        app.inventory_enchants[store_idx],
        vec![(sharp as i32, 3)],
        "the store the ride-home banks"
    );
    // THE PURE-RENAME 39 CAP (the repair-cost-heavy rename the cap arm
    // answers — the 41 the 39 re-write wears, the result the SHOWS
    // keeps: the may-pickup verdict the xp gate alone refuses)
    app.anvil_slots[0] = (sword, 1);
    app.anvil_slots[1] = (0, 0);
    app.anvil_repair_costs[0] = 40;
    app.anvil_repair_costs[1] = 0;
    app.anvil_name.clear();
    app.on_anvil_name_changed("Renamed");
    assert_eq!(app.anvil_cost, 39, "the cap the pure rename re-writes");
    assert!(app.anvil_only_renaming, "the cap arm the flag wears");
    assert_eq!(
        app.anvil_result,
        (sword, 1),
        "the capped walk the result shows"
    );
    // THE 40 GATE (the combine's own l + j the cap never wears — the
    // >= 40 cost the survival arm the EMPTY result keeps, the
    // no-rename walk the j the combine alone speaks)
    app.anvil_slots[0] = (sword, 1);
    app.anvil_slots[1] = (book, 1);
    app.anvil_enchants[1] = vec![(sharp as i32, 3)];
    app.anvil_repair_costs[0] = 40;
    app.anvil_repair_costs[1] = 0;
    app.anvil_name.clear();
    app.anvil_create_result();
    assert_eq!(
        app.anvil_result,
        (0, 0),
        "the too-expensive walk the EMPTY wears"
    );
    assert_eq!(app.anvil_cost, 43, "the assembly the l + j word keeps");
}

/// THE ANVIL'S OWN WIRE LANE (the dev-202 round — the menu type 8 the
/// OpenScreen word routes, the honest drop the round retires): the
/// window's slots the projection feeds, the title the decode lane
/// owns, the identity the clicks echo against, the prediction-free
/// echo the click emits, the rename the packet command carries.
#[test]
fn the_anvil_wire_route_the_menu_type8_wear() {
    use crate::world_view::{ContainerCommand, LiveContainer};
    let mut app = App::new(3);
    app.enter_world();
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    // THE ROUTE'S OWN FEED (the live_container_updated lane — the
    // window-2 the menu type 8 the projection answers)
    app.live_clicks = Some(std::sync::mpsc::channel().0);
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.live_clicks = Some(tx);
    let mut nbt = Vec::new();
    mc_protocol::nbt::write_network_nbt(
        &mc_protocol::nbt::Nbt::Compound(vec![(
            "text".to_string(),
            mc_protocol::nbt::Nbt::String("Villager's Anvil".to_string()),
        )]),
        &mut nbt,
    );
    app.live_container_updated(Some(LiveContainer {
        id: 2,
        menu_type_id: crate::world_model::MENU_TYPE_ANVIL,
        slots: vec![(sword, 1), (0, 0), (sword, 1)],
        bundle_rows: Vec::new(),
        carried: (0, 0),
        state_id: 7,
        data_slots: Vec::new(),
        title_nbt: nbt,
    }));
    assert_eq!(app.screen, ScreenKind::Anvil, "the route the anvil opens");
    assert!(app.anvil_is_live, "the live flag the route wears");
    assert_eq!(
        app.anvil_live_window,
        Some((2, 7)),
        "the window identity the clicks echo against"
    );
    assert_eq!(
        app.anvil_slots[0],
        (sword, 1),
        "the input seat the feed answers"
    );
    assert_eq!(
        app.anvil_result,
        (sword, 1),
        "the result seat the feed answers"
    );
    assert_eq!(
        app.anvil_mirror.len(),
        36,
        "the mirror rows the padded shape keeps"
    );
    assert_eq!(
        app.live_anvil_title.as_deref(),
        Some("Villager's Anvil"),
        "the window name the decode lane owns"
    );
    // THE PREDICTION-FREE ECHO (the plain click the empty delta list
    // rides — the server's own corrections the container lane re-banks)
    app.screen = ScreenKind::Anvil;
    app.anvil_slot_rects = vec![(0, 0, 0)];
    app.anvil_panel = (-176, -166); // the whole screen the seat alone covers
    app.shift_down = false;
    app.click_anvil_slot(4, 4, 0);
    match rx.try_recv().unwrap() {
        ContainerCommand::Click(click) => {
            assert_eq!(click.window_id, 2, "the window id the echo wears");
            assert_eq!(click.state_id, 7, "the state id the echo wears");
            assert_eq!(click.slot, 0, "the input seat the echo speaks");
            assert_eq!(click.button, 0, "the left press the echo carries");
            assert!(
                matches!(click.input, crate::world_view::ContainerInput::Pickup),
                "the pickup mode the plain press wears"
            );
            assert!(click.changed.is_empty(), "the prediction-free lane rides");
        }
        other => panic!("the click the echo speaks, not {other:?}"),
    }
    // THE RENAME'S OWN PACKET (the live window the responder the
    // command carries — the setItemName verdict TRUE the send answers)
    app.anvil_slots[0] = (sword, 1);
    app.on_anvil_name_changed("Renamed");
    match rx.try_recv().unwrap() {
        ContainerCommand::RenameItem { name } => {
            assert_eq!(name, "Renamed", "the typed word the command carries");
        }
        other => panic!("the rename the command speaks, not {other:?}"),
    }
    // THE HONEST DROP RETIRED (the route the chest lane never swallows)
    app.live_container_updated(None);
    assert_eq!(
        app.screen,
        ScreenKind::Anvil,
        "the live close the screen keeps"
    );
}

/// THE ANVIL'S OWN TOOLTIP SEAT (the 351 round — the extractTooltip
/// pass the anvil's own canvas wears): the hovered seat's rows the
/// shared lane speaks, the custom-name lane the first row re-keys
/// (the §o italic the styled hover name wears — the trio + the
/// mirror the store's own seats answer), the seat's own ENCHANT
/// store the lines ride, the carried wall + the empty seat + the
/// closed door the None verdicts keep. The REAL walk the offline
/// door wears — the headless frame the REAL rects the render lays.
#[test]
fn the_anvil_tooltip_the_seats_the_italic_wears() {
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::ANVIL);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the mirror reads
    }
    app.open_anvil_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Anvil,
        "the anvil door the kind wears"
    );
    // THE REAL RECTS THE RENDER BANKS (the door test's own harness —
    // the headless mouse-less frame the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let seat = |slot: i16| -> (i32, i32) {
        app.anvil_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE RENAMED INPUT'S OWN ROW (the §o italic the styled hover
    // name wears — the seat lane the first row re-keys)
    app.anvil_slots[0] = (sword, 1);
    app.anvil_names[0] = Some("Doom Bringer".to_string());
    let (i0x, i0y) = seat(0);
    let rows = app
        .anvil_item_tooltip_hover(i0x + 8, i0y + 8)
        .expect("the renamed input the rows speak");
    assert_eq!(
        rows[0].0, "\u{a7}oDoom Bringer",
        "the custom name the italic prefix wears"
    );
    // THE PLAIN STACK'S OWN NAME (the lane's None the shared lane
    // keeps — the hover name the display walk answers)
    app.anvil_names[0] = None;
    let rows = app
        .anvil_item_tooltip_hover(i0x + 8, i0y + 8)
        .expect("the plain input the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(sword).unwrap()),
        "the plain name the shared lane speaks"
    );
    // THE SEAT'S OWN ENCHANT STORE RIDES (the dev-202 walk the lines
    // wear — the sharpness III the name row answers)
    use mc_registry::generated::enchantments as ench_mod;
    let sharp = ench_mod::enchantment_index("sharpness").unwrap();
    app.anvil_enchants[0] = vec![(sharp as i32, 3)];
    let rows = app
        .anvil_item_tooltip_hover(i0x + 8, i0y + 8)
        .expect("the enchanted input the rows speak");
    assert_eq!(rows.len(), 2, "the name + the line the store wears");
    assert_eq!(rows[1].0, "Sharpness III", "the store line the walk speaks");
    app.anvil_enchants[0].clear();
    // THE RESULT SEAT'S OWN LANE (the createResult's product the
    // walk banks — the copy name the result seat answers)
    app.anvil_result = (sword, 1);
    app.anvil_names[2] = Some("Forged Blade".to_string());
    let (rx, ry) = seat(2);
    let rows = app
        .anvil_item_tooltip_hover(rx + 8, ry + 8)
        .expect("the result seat the rows speak");
    assert_eq!(
        rows[0].0, "\u{a7}oForged Blade",
        "the result's copy name the italic wears"
    );
    app.anvil_result = (0, 0);
    app.anvil_names[2] = None;
    // THE MIRROR'S OWN POSITIONAL RIDE (the dev-202 map the store
    // row answers — the mirror head the inventory's item 9 the lane
    // keeps)
    app.anvil_mirror[0] = (dirt, 5);
    app.items_custom_names[9] = Some("Named Dirt".to_string());
    let (mx, my) = seat(3);
    let rows = app
        .anvil_item_tooltip_hover(mx + 8, my + 8)
        .expect("the named mirror row the rows speak");
    assert_eq!(
        rows[0].0, "\u{a7}oNamed Dirt",
        "the positional map the store row wears"
    );
    app.items_custom_names[9] = None;
    let rows = app
        .anvil_item_tooltip_hover(mx + 8, my + 8)
        .expect("the plain mirror row the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(dirt).unwrap()),
        "the cleared lane the plain name keeps"
    );
    // THE CARRIED WALL (the pc 27-46 verdict the anvil's own stack
    // answers)
    app.anvil_carried = (1, 1);
    assert_eq!(
        app.anvil_item_tooltip_hover(i0x + 8, i0y + 8),
        None,
        "the carried stack the wall refuses"
    );
    app.anvil_carried = (0, 0);
    // THE EMPTY SEAT (the hasItem gate the silence keeps)
    let (e1x, e1y) = seat(1);
    assert_eq!(
        app.anvil_item_tooltip_hover(e1x + 8, e1y + 8),
        None,
        "the empty seat the gate refuses"
    );
    // THE WRONG SCREEN (the closed door the walk refuses)
    app.close_anvil(ScreenKind::World);
    assert_eq!(
        app.anvil_item_tooltip_hover(i0x + 8, i0y + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE INVENTORY TOOLTIP'S OWN LANE RIDE (the 351 round — the dev-202
/// store the hovered seat's first row re-keys): the renamed store row
/// the §o italic the styled hover name wears, the offhand's 36 the
/// same lane answers, the armor row the store never carries the plain
/// name keeps, the carried wall + the empty seat the None verdicts
/// keep.
#[test]
fn the_inventory_tooltip_the_custom_lane_wears() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let shield = mc_registry::item_id_by_name("shield").unwrap();
    let boots = mc_registry::item_id_by_name("iron_boots").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE RENAMED STORE ROW (the §o italic the lane's own answer)
    app.inventory.items[10] = (dirt, 1);
    app.items_custom_names[10] = Some("Named Block".to_string());
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 10);
    let (id, rows) = app
        .item_tooltip_hover(ox + 8, oy + 8)
        .expect("the renamed row the rows speak");
    assert_eq!(id, dirt, "the seat's own stack the id answers");
    assert_eq!(
        rows[0].0, "\u{a7}oNamed Block",
        "the custom name the italic prefix wears"
    );
    // THE CLEARED LANE (the plain name the shared lane keeps)
    app.items_custom_names[10] = None;
    let (_, rows) = app
        .item_tooltip_hover(ox + 8, oy + 8)
        .expect("the plain row the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(dirt).unwrap()),
        "the plain name the lane's None keeps"
    );
    // THE OFFHAND'S OWN SEAT (the bank's 36 the hovered shield cell
    // answers — the 346 seat the lane keeps)
    app.inventory.offhand[0] = (shield, 1);
    app.items_custom_names[36] = Some("Warded Shield".to_string());
    let (ox2, oy2) = offhand_slot_origin(app.inv_panel);
    let (_, rows) = app
        .item_tooltip_hover(ox2 + 8, oy2 + 8)
        .expect("the offhand seat the rows speak");
    assert_eq!(
        rows[0].0, "\u{a7}oWarded Shield",
        "the offhand's 36 the lane answers"
    );
    app.items_custom_names[36] = None;
    // THE ARMOR ROW'S OWN WALL (the store the lane never carries —
    // the plain name the honest silence keeps)
    app.inventory.armor[3] = (boots, 1);
    let (ax, ay) = armor_slot_origin(app.inv_panel, 3);
    let (_, rows) = app
        .item_tooltip_hover(ax + 8, ay + 8)
        .expect("the armor row the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(boots).unwrap()),
        "the armor row the plain name keeps"
    );
    // THE CARRIED WALL (the image-less false answer the pc 27-46
    // keeps)
    app.inv_carried = (dirt, 1);
    assert_eq!(
        app.item_tooltip_hover(ox + 8, oy + 8),
        None,
        "the carried stack the wall refuses"
    );
    app.inv_carried = (0, 0);
    // THE EMPTY SEAT (the hasItem gate the silence keeps — the row
    // the test's own walk empties first)
    app.inventory.items[10] = (0, 0);
    assert_eq!(
        app.item_tooltip_hover(ox + 8, oy + 8),
        None,
        "the empty row the gate refuses"
    );
}

/// THE HELD NAME'S OWN LANE RIDE (the 351 round — the pcs 44-64
/// italic branch the dev-202 store the honest absence retires): the
/// renamed hotbar seat the §o word the popup stores, the change arm
/// the re-arm the timer answers, the quiet tick the decay keeps, the
/// cleared lane the plain name the change re-arms.
#[test]
fn the_held_name_the_custom_lane_the_italic_wears() {
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let mut row = [(0, 0); 9];
    row[0] = (sword, 1);
    app.live_hotbar_updated(row);
    // THE RENAMED SEAT'S OWN POP (the lane's Some the §o word the
    // tick stores — the styled hover name the legacy prefix carries)
    app.items_custom_names[0] = Some("Doom Bringer".to_string());
    app.update();
    assert_eq!(
        app.held_name_timer, HELD_NAME_POP_TICKS,
        "the change the pop arms"
    );
    assert_eq!(
        app.held_name_last,
        Some((sword, "\u{a7}oDoom Bringer".to_string())),
        "the custom name the italic word wears"
    );
    // THE SAME-NAME QUIET TICK (the equal word the decay keeps)
    app.update();
    assert_eq!(
        app.held_name_timer,
        HELD_NAME_POP_TICKS - 1,
        "the quiet tick the decay rides"
    );
    // THE CLEARED LANE (the plain name the change re-arms — the
    // lane's None the shared lane keeps)
    app.items_custom_names[0] = None;
    app.update();
    assert_eq!(
        app.held_name_timer, HELD_NAME_POP_TICKS,
        "the plain word the change re-arms"
    );
    assert_eq!(
        app.held_name_last,
        Some((sword, app.held_item_display_name(sword))),
        "the plain name the cleared lane keeps"
    );
}

/// THE ENCHANTING SCREEN'S OWN TOOLTIP SEAT (the 352 round — the 351
/// law the enchanting's own canvas re-wears): the hovered seat's rows
/// the shared lane speaks, the seat's own ENCHANT store the
/// enchant_comps numbering rides (the sharpness III the line wears),
/// the mirror rows the custom-name lane positionally re-keys (the
/// transient pair the plain name the honest lane keeps), the carried
/// wall + the empty seat + the closed door the None verdicts keep.
/// The REAL walk the offline door wears — the headless frame the
/// REAL rects the render lays.
#[test]
fn the_enchant_tooltip_the_seats_the_stores_wear() {
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let lapis = mc_registry::item_id_by_name("lapis_lazuli").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::ENCHANTING_TABLE);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the mirror reads
    }
    app.open_enchanting_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Enchanting,
        "the table door the kind wears"
    );
    // THE REAL RECTS THE RENDER BANKS (the door test's own harness —
    // the headless mouse-less frame the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let seat = |slot: i16| -> (i32, i32) {
        app.enchant_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE ENCHANTED ITEM SEAT'S OWN ROW (the store the lines ride —
    // the sharpness III the name row answers)
    use mc_registry::generated::enchantments as ench_mod;
    let sharp = ench_mod::enchantment_index("sharpness").unwrap();
    app.enchant_slots[0] = (sword, 1);
    app.enchant_comps[0] = vec![(sharp as i32, 3)];
    let (i0x, i0y) = seat(0);
    let rows = app
        .enchant_item_tooltip_hover(i0x + 8, i0y + 8)
        .expect("the enchanted item the rows speak");
    assert_eq!(rows.len(), 2, "the name + the line the store wears");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(sword).unwrap()),
        "the pair seat the plain name keeps"
    );
    assert_eq!(rows[1].0, "Sharpness III", "the store line the walk speaks");
    // THE LAPIS SEAT'S OWN PLAIN ROW (the transient pair the store
    // never names — the shared lane the display name answers)
    app.enchant_slots[1] = (lapis, 3);
    let (i1x, i1y) = seat(1);
    let rows = app
        .enchant_item_tooltip_hover(i1x + 8, i1y + 8)
        .expect("the lapis seat the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(lapis).unwrap()),
        "the plain lapis the shared lane speaks"
    );
    // THE MIRROR'S OWN POSITIONAL RIDE (the dev-202 map the store row
    // answers — the mirror head the inventory's item 9 the lane keeps)
    app.enchant_mirror[0] = (dirt, 5);
    app.items_custom_names[9] = Some("Named Dirt".to_string());
    let (mx, my) = seat(2);
    let rows = app
        .enchant_item_tooltip_hover(mx + 8, my + 8)
        .expect("the named mirror row the rows speak");
    assert_eq!(
        rows[0].0, "\u{a7}oNamed Dirt",
        "the positional map the store row wears"
    );
    app.items_custom_names[9] = None;
    let rows = app
        .enchant_item_tooltip_hover(mx + 8, my + 8)
        .expect("the plain mirror row the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(dirt).unwrap()),
        "the cleared lane the plain name keeps"
    );
    // THE CARRIED WALL (the pc 27-46 verdict the enchanting's own
    // stack answers)
    app.enchant_carried = (1, 1);
    assert_eq!(
        app.enchant_item_tooltip_hover(i0x + 8, i0y + 8),
        None,
        "the carried stack the wall refuses"
    );
    app.enchant_carried = (0, 0);
    // THE EMPTY SEAT (the hasItem gate the silence keeps — the lapis
    // cleared the store the plain walk refuses)
    app.enchant_slots[1] = (0, 0);
    assert_eq!(
        app.enchant_item_tooltip_hover(i1x + 8, i1y + 8),
        None,
        "the empty seat the gate refuses"
    );
    // THE WRONG SCREEN (the closed door the walk refuses)
    app.close_enchanting(ScreenKind::World);
    assert_eq!(
        app.enchant_item_tooltip_hover(i0x + 8, i0y + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE CHEST'S OWN TOOLTIP SEAT (the dev-203 round — the 351/352 law
/// the chest's own canvas wears): the container cell the plain row
/// the per-cell absence keeps, the mirror rows the store lane
/// positionally re-keys (the §o italic the named store row wears —
/// the offline write-through the REAL inventory keeps), the carried
/// wall + the empty seat + the closed door the None verdicts keep.
/// The REAL walk the offline door wears — the headless frame the
/// REAL rects the render lays.
#[test]
fn the_chest_tooltip_the_mirror_stores_wear() {
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the mirror reads
    }
    app.open_chest_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Chest,
        "the chest door the kind wears"
    );
    // THE REAL RECTS THE RENDER BANKS (the door test's own harness —
    // the headless mouse-less frame the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let seat = |slot: i16| -> (i32, i32) {
        app.chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE CONTAINER CELL'S OWN PLAIN ROW (the per-cell store the
    // name lane never carries — the shared lane the display name
    // answers)
    app.chest_slots[0] = (sword, 1);
    let (c0x, c0y) = seat(0);
    let rows = app
        .chest_item_tooltip_hover(c0x + 8, c0y + 8)
        .expect("the held cell the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(sword).unwrap()),
        "the cell the plain name keeps"
    );
    // THE MIRROR'S OWN POSITIONAL RIDE (the §o italic the named
    // store row wears — the mirror head the inventory's item 9 the
    // lane keeps)
    app.chest_mirror[0] = (dirt, 5);
    app.items_custom_names[9] = Some("Named Dirt".to_string());
    let (mx, my) = seat(27);
    let rows = app
        .chest_item_tooltip_hover(mx + 8, my + 8)
        .expect("the named mirror row the rows speak");
    assert_eq!(
        rows[0].0, "\u{a7}oNamed Dirt",
        "the positional map the store row wears"
    );
    app.items_custom_names[9] = None;
    let rows = app
        .chest_item_tooltip_hover(mx + 8, my + 8)
        .expect("the plain mirror row the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(dirt).unwrap()),
        "the cleared lane the plain name keeps"
    );
    // THE CARRIED WALL (the offline cursor the same field answers)
    app.chest_carried = (1, 1);
    assert_eq!(
        app.chest_item_tooltip_hover(mx + 8, my + 8),
        None,
        "the carried stack the wall refuses"
    );
    app.chest_carried = (0, 0);
    // THE EMPTY CELL (the hasItem gate the silence keeps)
    app.chest_slots[1] = (0, 0);
    let (c1x, c1y) = seat(1);
    assert_eq!(
        app.chest_item_tooltip_hover(c1x + 8, c1y + 8),
        None,
        "the empty cell the gate refuses"
    );
    // THE WRONG SCREEN (the closed door the walk refuses)
    app.close_chest_container(ScreenKind::World);
    assert_eq!(
        app.chest_item_tooltip_hover(c0x + 8, c0y + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE FURNACE'S OWN TOOLTIP SEAT (the dev-203 round — the 351/352
/// law the furnace's own canvas wears): the trio the plain rows the
/// per-cell absence keeps, the mirror rows the store lane
/// positionally re-keys (the §o italic + the ENCHANT line the store
/// rides — the offline write-through the REAL inventory keeps), the
/// carried wall the render's own split reads, the empty seat + the
/// closed door the None verdicts keep. The REAL walk the offline
/// door wears — the headless frame the REAL rects the render lays.
#[test]
fn the_furnace_tooltip_the_mirror_stores_wear() {
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::FURNACE);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the mirror reads
    }
    app.open_furnace(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Furnace,
        "the furnace door the kind wears"
    );
    // THE REAL RECTS THE RENDER BANKS (the door test's own harness —
    // the headless mouse-less frame the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let seat = |slot: i16| -> (i32, i32) {
        app.furnace_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE TRIO'S OWN PLAIN ROW (the input seat the per-cell store the
    // name lane never carries — the shared lane the display name
    // answers)
    app.furnace_slots[0] = (sword, 1);
    let (i0x, i0y) = seat(0);
    let rows = app
        .furnace_item_tooltip_hover(i0x + 8, i0y + 8)
        .expect("the held input the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(sword).unwrap()),
        "the trio seat the plain name keeps"
    );
    // THE MIRROR'S OWN POSITIONAL RIDE (the §o italic + the ENCHANT
    // line the store rides — the mirror head the inventory's item 9
    // the lane keeps)
    use mc_registry::generated::enchantments as ench_mod;
    let sharp = ench_mod::enchantment_index("sharpness").unwrap();
    app.furnace_slots[3] = (dirt, 5);
    app.items_custom_names[9] = Some("Named Dirt".to_string());
    app.inventory_enchants[9] = vec![(sharp as i32, 3)];
    let (mx, my) = seat(3);
    let rows = app
        .furnace_item_tooltip_hover(mx + 8, my + 8)
        .expect("the named mirror row the rows speak");
    assert_eq!(rows.len(), 2, "the name + the line the store wears");
    assert_eq!(
        rows[0].0, "\u{a7}oNamed Dirt",
        "the positional map the store row wears"
    );
    assert_eq!(rows[1].0, "Sharpness III", "the store line the walk speaks");
    app.items_custom_names[9] = None;
    app.inventory_enchants[9].clear();
    let rows = app
        .furnace_item_tooltip_hover(mx + 8, my + 8)
        .expect("the plain mirror row the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(dirt).unwrap()),
        "the cleared lane the plain name keeps"
    );
    // THE CARRIED WALL (the offline cursor the same field answers)
    app.furnace_carried = (1, 1);
    assert_eq!(
        app.furnace_item_tooltip_hover(mx + 8, my + 8),
        None,
        "the carried stack the wall refuses"
    );
    app.furnace_carried = (0, 0);
    // THE EMPTY SEAT (the hasItem gate the silence keeps — the fuel
    // cell the fresh zeros wear)
    let (f1x, f1y) = seat(1);
    assert_eq!(
        app.furnace_item_tooltip_hover(f1x + 8, f1y + 8),
        None,
        "the empty seat the gate refuses"
    );
    // THE WRONG SCREEN (the closed door the walk refuses)
    app.close_furnace(ScreenKind::World);
    assert_eq!(
        app.furnace_item_tooltip_hover(i0x + 8, i0y + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE DISPENSER'S OWN TOOLTIP SEAT (the dev-203 round — the 351/352
/// law the dispenser's own canvas wears): the grid cell the plain
/// row the per-cell absence keeps, the mirror rows the store lane
/// positionally re-keys (the §o italic the named store row wears —
/// the offline write-through the REAL inventory keeps), the carried
/// wall + the empty seat + the closed door the None verdicts keep.
/// The REAL walk the offline door wears — the headless frame the
/// REAL rects the render lays.
#[test]
fn the_dispenser_tooltip_the_mirror_stores_wear() {
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the mirror reads
    }
    app.open_dispenser_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Dispenser,
        "the dispenser door the kind wears"
    );
    // THE REAL RECTS THE RENDER BANKS (the door test's own harness —
    // the headless mouse-less frame the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let seat = |slot: i16| -> (i32, i32) {
        app.dispenser_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE GRID CELL'S OWN PLAIN ROW (the per-cell store the name lane
    // never carries — the shared lane the display name answers)
    app.dispenser_slots[0] = (sword, 1);
    let (g0x, g0y) = seat(0);
    let rows = app
        .dispenser_item_tooltip_hover(g0x + 8, g0y + 8)
        .expect("the held cell the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(sword).unwrap()),
        "the grid cell the plain name keeps"
    );
    // THE MIRROR'S OWN POSITIONAL RIDE (the §o italic the named store
    // row wears — the mirror head the inventory's item 9 the lane
    // keeps)
    app.dispenser_mirror[0] = (dirt, 5);
    app.items_custom_names[9] = Some("Named Dirt".to_string());
    let (mx, my) = seat(9);
    let rows = app
        .dispenser_item_tooltip_hover(mx + 8, my + 8)
        .expect("the named mirror row the rows speak");
    assert_eq!(
        rows[0].0, "\u{a7}oNamed Dirt",
        "the positional map the store row wears"
    );
    app.items_custom_names[9] = None;
    let rows = app
        .dispenser_item_tooltip_hover(mx + 8, my + 8)
        .expect("the plain mirror row the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(dirt).unwrap()),
        "the cleared lane the plain name keeps"
    );
    // THE CARRIED WALL (the offline cursor the same field answers)
    app.dispenser_carried = (1, 1);
    assert_eq!(
        app.dispenser_item_tooltip_hover(mx + 8, my + 8),
        None,
        "the carried stack the wall refuses"
    );
    app.dispenser_carried = (0, 0);
    // THE EMPTY CELL (the hasItem gate the silence keeps)
    app.dispenser_slots[1] = (0, 0);
    let (g1x, g1y) = seat(1);
    assert_eq!(
        app.dispenser_item_tooltip_hover(g1x + 8, g1y + 8),
        None,
        "the empty cell the gate refuses"
    );
    // THE WRONG SCREEN (the closed door the walk refuses)
    app.close_dispenser(ScreenKind::World);
    assert_eq!(
        app.dispenser_item_tooltip_hover(g0x + 8, g0y + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE HOPPER'S OWN TOOLTIP SEAT (the autopush-353 round — the
/// 351/352 law the hopper's own canvas wears, the dev-203 door the
/// five-seat flag keeps): the five-seat row the plain rows the
/// per-cell absence keeps, the mirror rows the store lane
/// positionally re-keys (the §o italic the named store row wears —
/// the offline write-through the REAL inventory keeps), the carried
/// wall + the empty seat + the closed door the None verdicts keep.
/// The REAL walk the offline door wears — the headless frame the
/// REAL rects the render lays.
#[test]
fn the_hopper_tooltip_the_five_seats_wear() {
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::HOPPER);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the mirror reads
    }
    app.open_hopper_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Chest,
        "the hopper door the chest kind wears"
    );
    assert!(
        app.chest_window_hopper,
        "the five-seat flag the hopper door keeps"
    );
    // THE REAL RECTS THE RENDER BANKS (the door test's own harness —
    // the headless mouse-less frame the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let seat = |slot: i16| -> (i32, i32) {
        app.chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE FIVE-SEAT ROW'S OWN PLAIN RIDE (the per-cell store the name
    // lane never carries — the shared lane the display name answers)
    app.chest_slots[0] = (sword, 1);
    let (h0x, h0y) = seat(0);
    let rows = app
        .hopper_item_tooltip_hover(h0x + 8, h0y + 8)
        .expect("the held seat the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(sword).unwrap()),
        "the five-seat cell the plain name keeps"
    );
    // THE MIRROR'S OWN POSITIONAL RIDE (the §o italic the named store
    // row wears — the mirror head the inventory's item 9 the lane
    // keeps)
    app.chest_mirror[0] = (dirt, 5);
    app.items_custom_names[9] = Some("Named Dirt".to_string());
    let (mx, my) = seat(5);
    let rows = app
        .hopper_item_tooltip_hover(mx + 8, my + 8)
        .expect("the named mirror row the rows speak");
    assert_eq!(
        rows[0].0, "\u{a7}oNamed Dirt",
        "the positional map the store row wears"
    );
    app.items_custom_names[9] = None;
    let rows = app
        .hopper_item_tooltip_hover(mx + 8, my + 8)
        .expect("the plain mirror row the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(dirt).unwrap()),
        "the cleared lane the plain name keeps"
    );
    // THE CARRIED WALL (the offline cursor the same field answers)
    app.chest_carried = (1, 1);
    assert_eq!(
        app.hopper_item_tooltip_hover(mx + 8, my + 8),
        None,
        "the carried stack the wall refuses"
    );
    app.chest_carried = (0, 0);
    // THE EMPTY SEAT (the hasItem gate the silence keeps)
    app.chest_slots[1] = (0, 0);
    let (h1x, h1y) = seat(1);
    assert_eq!(
        app.hopper_item_tooltip_hover(h1x + 8, h1y + 8),
        None,
        "the empty seat the gate refuses"
    );
    // THE WRONG SCREEN (the closed door the walk refuses)
    app.close_chest_container(ScreenKind::World);
    assert_eq!(
        app.hopper_item_tooltip_hover(h0x + 8, h0y + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE BREWING STAND'S OWN TOOLTIP SEAT (the autopush-354 round — the
/// 351/352 law the brewing's own canvas wears, the render's own kind
/// lane the icons bank): the kinded bottle seat the kind store rides
/// (the display name the kind lane re-keys, the effect rows the ride
/// speaks), the kind-less bottle the plain name keeps, the ingredient
/// cell the plain ride speaks, the mirror rows the positional name +
/// kind lanes re-key (the §o italic the named store row wears — the
/// offline write-through the REAL inventory keeps), the carried wall
/// + the empty seat + the closed door the None verdicts keep. The
/// REAL walk the offline door wears — the headless frame the REAL
/// rects the render lays.
#[test]
fn the_brewing_tooltip_the_seats_the_kinds_wear() {
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::BREWING_STAND);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the mirror reads
    }
    app.open_brewing_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Brewing,
        "the brewing door the kind wears"
    );
    // THE REAL RECTS THE RENDER BANKS (the door test's own harness —
    // the headless mouse-less frame the cells lay)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let seat = |slot: i16| -> (i32, i32) {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // THE KINDED BOTTLE'S OWN RIDE (the stand's own KIND store the
    // icons wear — the night-vision seat the potion rows the shared
    // lane speaks, the display name the kind lane re-keys)
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    app.brewing_slots[0] = (potion, 1);
    app.brewing_kinds[0] = kind;
    let (b0x, b0y) = seat(0);
    let rows = app
        .brewing_item_tooltip_hover(b0x + 8, b0y + 8)
        .expect("the kinded bottle the rows speak");
    assert_eq!(
        rows[0].0,
        App::potion_kind_display_name(kind),
        "the kind store the display name wears"
    );
    assert!(rows.len() > 1, "the effect rows the kind lane rides");
    // THE KIND-LESS BOTTLE'S OWN PLAIN WALL (the NONE seat the plain
    // name keeps — the stale kind the lane never wears)
    app.brewing_kinds[0] = crate::brewing_sim::POTION_KIND_NONE;
    let rows = app
        .brewing_item_tooltip_hover(b0x + 8, b0y + 8)
        .expect("the kind-less bottle the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(potion).unwrap()),
        "the NONE wall the plain name keeps"
    );
    // THE INGREDIENT SEAT'S OWN PLAIN RIDE (the non-potion cell the
    // kind store never wears — the plain name the shared lane speaks)
    app.brewing_slots[3] = (sword, 1);
    let (ix, iy) = seat(3);
    let rows = app
        .brewing_item_tooltip_hover(ix + 8, iy + 8)
        .expect("the ingredient seat the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(sword).unwrap()),
        "the ingredient cell the plain name keeps"
    );
    // THE MIRROR'S OWN POSITIONAL RIDE (the §o italic the named store
    // row wears — the mirror head the inventory's item 9 the lane
    // keeps)
    app.brewing_mirror[0] = (dirt, 5);
    app.items_custom_names[9] = Some("Named Dirt".to_string());
    let (mx, my) = seat(5);
    let rows = app
        .brewing_item_tooltip_hover(mx + 8, my + 8)
        .expect("the named mirror row the rows speak");
    assert_eq!(
        rows[0].0, "\u{a7}oNamed Dirt",
        "the positional map the store row wears"
    );
    app.items_custom_names[9] = None;
    let rows = app
        .brewing_item_tooltip_hover(mx + 8, my + 8)
        .expect("the plain mirror row the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(dirt).unwrap()),
        "the cleared lane the plain name keeps"
    );
    // THE MIRROR'S OWN KIND RIDE (the inventory's own kind lane the
    // menu map reads — the kinded stack the row the icons wear, the
    // +4 shift the main rows speak)
    app.brewing_mirror[1] = (potion, 1);
    app.items_potion_kinds[10] = kind;
    let (mx1, my1) = seat(6);
    let rows = app
        .brewing_item_tooltip_hover(mx1 + 8, my1 + 8)
        .expect("the kinded mirror row the rows speak");
    assert_eq!(
        rows[0].0,
        App::potion_kind_display_name(kind),
        "the inventory kind lane the mirror row wears"
    );
    app.items_potion_kinds[10] = crate::brewing_sim::POTION_KIND_NONE;
    // THE CARRIED WALL (the offline cursor the same field answers)
    app.brewing_carried = (1, 1);
    assert_eq!(
        app.brewing_item_tooltip_hover(mx + 8, my + 8),
        None,
        "the carried stack the wall refuses"
    );
    app.brewing_carried = (0, 0);
    // THE EMPTY SEAT (the hasItem gate the silence keeps)
    app.brewing_slots[1] = (0, 0);
    let (b1x, b1y) = seat(1);
    assert_eq!(
        app.brewing_item_tooltip_hover(b1x + 8, b1y + 8),
        None,
        "the empty seat the gate refuses"
    );
    // THE WRONG SCREEN (the closed door the walk refuses)
    app.close_brewing(ScreenKind::World);
    assert_eq!(
        app.brewing_item_tooltip_hover(b0x + 8, b0y + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE CRAFTING TABLE'S OWN TOOLTIP SEAT (the autopush-355 round —
/// the 351/352/203/353/354 law the table's own canvas wears, the
/// window-1 menu numbering the ONE hover walk the clicks share): the
/// craft cell the plain ride speaks, the result seat the sim's
/// recompute the rows wear, the store rows the positional name +
/// kind lanes re-key (the §o italic the named store row wears — the
/// main grid the menu-1 map the items 9..35 speaks, the hotbar the
/// -37 shift the items 0..8 keeps), the carried wall + the empty
/// seat + the closed door the None verdicts keep. The REAL walk the
/// offline door wears — the headless frame the REAL panel the
/// render lays.
#[test]
fn the_table_tooltip_the_seats_the_store_wear() {
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the walk reads
    }
    app.open_crafting_table();
    assert_eq!(
        app.screen,
        ScreenKind::Crafting,
        "the table door the walk wears"
    );
    // THE REAL PANEL THE RENDER BANKS (the door test's own harness —
    // the headless mouse-less frame the panel lays)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = app.table_panel;
    let craft_seat = |kind: usize| -> (i32, i32) {
        let (x, y) = table_craft_origin(panel, kind);
        (x + 1, y + 1)
    };
    let store_seat = |i: usize| -> (i32, i32) {
        let (x, y) = inventory_slot_origin(panel, i);
        (x + 1, y + 1)
    };
    // THE CRAFT CELL'S OWN PLAIN RIDE (the transient store the name
    // lane never carries — the shared lane the display name answers)
    app.table_grid[0] = (sword, 1);
    let (c0x, c0y) = craft_seat(0);
    let rows = app
        .table_item_tooltip_hover(c0x + 8, c0y + 8)
        .expect("the craft cell the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(sword).unwrap()),
        "the craft cell the plain name keeps"
    );
    // THE RESULT SEAT'S OWN PLAIN RIDE (the sim's recompute the rows
    // wear — the transient seat the store lane never maps)
    app.table_result = (dirt, 4);
    let (rx, ry) = craft_seat(9);
    let rows = app
        .table_item_tooltip_hover(rx + 8, ry + 8)
        .expect("the result seat the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(dirt).unwrap()),
        "the result seat the plain name keeps"
    );
    // THE MAIN GRID'S OWN POSITIONAL RIDE (the §o italic the named
    // store row wears — the menu-1 map the items 9..35 the lane
    // keeps)
    app.inventory.items[9] = (dirt, 5);
    app.items_custom_names[9] = Some("Named Dirt".to_string());
    let (mx, my) = store_seat(9);
    let rows = app
        .table_item_tooltip_hover(mx + 8, my + 8)
        .expect("the named main row the rows speak");
    assert_eq!(
        rows[0].0, "\u{a7}oNamed Dirt",
        "the positional map the store row wears"
    );
    app.items_custom_names[9] = None;
    let rows = app
        .table_item_tooltip_hover(mx + 8, my + 8)
        .expect("the plain main row the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(dirt).unwrap()),
        "the cleared lane the plain name keeps"
    );
    // THE HOTBAR'S OWN POSITIONAL RIDE (the -37 shift the items 0..8
    // the lane keeps — the named hotbar row the §o italic wears)
    app.inventory.items[0] = (sword, 1);
    app.items_custom_names[0] = Some("Named Sword".to_string());
    let (hx, hy) = store_seat(0);
    let rows = app
        .table_item_tooltip_hover(hx + 8, hy + 8)
        .expect("the named hotbar row the rows speak");
    assert_eq!(
        rows[0].0, "\u{a7}oNamed Sword",
        "the hotbar shift the store row wears"
    );
    app.items_custom_names[0] = None;
    let rows = app
        .table_item_tooltip_hover(hx + 8, hy + 8)
        .expect("the plain hotbar row the rows speak");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(sword).unwrap()),
        "the cleared lane the plain name keeps"
    );
    // THE KIND'S OWN RIDE (the 354 brewing law the store rows
    // re-wear — the kinded stack the row the potion rows the shared
    // lane speaks, the display name the kind lane re-keys)
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    app.inventory.items[10] = (potion, 1);
    app.items_potion_kinds[10] = kind;
    let (kx, ky) = store_seat(10);
    let rows = app
        .table_item_tooltip_hover(kx + 8, ky + 8)
        .expect("the kinded store row the rows speak");
    assert_eq!(
        rows[0].0,
        App::potion_kind_display_name(kind),
        "the inventory kind lane the store row wears"
    );
    assert!(rows.len() > 1, "the effect rows the kind lane rides");
    app.items_potion_kinds[10] = crate::brewing_sim::POTION_KIND_NONE;
    // THE CARRIED WALL (the offline cursor the same field answers)
    app.inv_carried = (1, 1);
    assert_eq!(
        app.table_item_tooltip_hover(mx + 8, my + 8),
        None,
        "the carried stack the wall refuses"
    );
    app.inv_carried = (0, 0);
    // THE EMPTY SEAT (the hasItem gate the silence keeps)
    app.table_grid[1] = (0, 0);
    let (c1x, c1y) = craft_seat(1);
    assert_eq!(
        app.table_item_tooltip_hover(c1x + 8, c1y + 8),
        None,
        "the empty seat the gate refuses"
    );
    // THE WRONG SCREEN (the closed door the walk refuses)
    app.close_crafting_table(ScreenKind::World);
    assert_eq!(
        app.table_item_tooltip_hover(c0x + 8, c0y + 8),
        None,
        "the closed door the walk refuses"
    );
}

/// THE MIRROR KIND RIDE'S FAMILY RETROFIT (the autopush-356 round —
/// the 355 law the chest/furnace/dispenser/hopper walks re-wear: the
/// kinded stack the mirror row the display name + the effect rows
/// the shared lane speaks, the container cell the NONE wall keeps —
/// the kind the per-cell stores never carry, the kind seat the cell
/// never reads). The REAL walks the offline doors wear — the
/// headless frames the REAL rects the renders lay.
#[test]
fn the_mirror_kind_the_family_wear() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    let surface_cell = |app: &mut App| -> (i32, i32, i32) {
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        (c, app.spawn_y - 1 + 1, c)
    };
    // THE CHEST'S OWN RIDE (the mirror head the inventory's item 9
    // the lane keeps)
    {
        let mut app = App::new(3);
        let cell = surface_cell(&mut app);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::CHEST);
        for it in app.inventory.items.iter_mut() {
            *it = (0, 0);
        }
        app.open_chest_for(cell);
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
        let seat = |slot: i16| -> (i32, i32) {
            app.chest_slot_rects
                .iter()
                .find(|&&(_, _, s)| s == slot)
                .map(|&(x, y, _)| (x, y))
                .unwrap()
        };
        app.chest_mirror[0] = (potion, 1);
        app.items_potion_kinds[9] = kind;
        let (mx, my) = seat(27);
        let rows = app
            .chest_item_tooltip_hover(mx + 8, my + 8)
            .expect("the chest kinded mirror row the rows speak");
        assert_eq!(
            rows[0].0,
            App::potion_kind_display_name(kind),
            "the chest mirror row the kind lane wears"
        );
        assert!(rows.len() > 1, "the effect rows the chest mirror rides");
        // THE CONTAINER CELL'S OWN NONE WALL (the kind seat the cell
        // never reads — the per-cell store the kind never carries)
        app.chest_slots[0] = (potion, 1);
        let (c0x, c0y) = seat(0);
        let rows = app
            .chest_item_tooltip_hover(c0x + 8, c0y + 8)
            .expect("the chest cell the rows speak");
        assert_eq!(
            rows[0].0,
            item_display_name(mc_registry::item_name(potion).unwrap()),
            "the chest cell the NONE wall keeps"
        );
        app.items_potion_kinds[9] = none;
    }
    // THE FURNACE'S OWN RIDE (the mirror head the seat 3 the lane
    // keeps — the menu map the items 9..35 the +6 shift speaks)
    {
        let mut app = App::new(3);
        let cell = surface_cell(&mut app);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::FURNACE);
        for it in app.inventory.items.iter_mut() {
            *it = (0, 0);
        }
        app.open_furnace(cell);
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
        let seat = |slot: i16| -> (i32, i32) {
            app.furnace_slot_rects
                .iter()
                .find(|&&(_, _, s)| s == slot)
                .map(|&(x, y, _)| (x, y))
                .unwrap()
        };
        app.furnace_slots[3] = (potion, 1);
        app.items_potion_kinds[9] = kind;
        let (mx, my) = seat(3);
        let rows = app
            .furnace_item_tooltip_hover(mx + 8, my + 8)
            .expect("the furnace kinded mirror row the rows speak");
        assert_eq!(
            rows[0].0,
            App::potion_kind_display_name(kind),
            "the furnace mirror row the kind lane wears"
        );
        assert!(rows.len() > 1, "the effect rows the furnace mirror rides");
        // THE TRIO'S OWN NONE WALL (the input seat the kind never
        // reads — the furnace store the kind never carries)
        app.furnace_slots[0] = (potion, 1);
        let (i0x, i0y) = seat(0);
        let rows = app
            .furnace_item_tooltip_hover(i0x + 8, i0y + 8)
            .expect("the furnace trio the rows speak");
        assert_eq!(
            rows[0].0,
            item_display_name(mc_registry::item_name(potion).unwrap()),
            "the furnace trio the NONE wall keeps"
        );
        app.items_potion_kinds[9] = none;
    }
    // THE DISPENSER'S OWN RIDE (the mirror head the seat 9 the lane
    // keeps — the same numbering the identity keeps)
    {
        let mut app = App::new(3);
        let cell = surface_cell(&mut app);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
        for it in app.inventory.items.iter_mut() {
            *it = (0, 0);
        }
        app.open_dispenser_for(cell);
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
        let seat = |slot: i16| -> (i32, i32) {
            app.dispenser_slot_rects
                .iter()
                .find(|&&(_, _, s)| s == slot)
                .map(|&(x, y, _)| (x, y))
                .unwrap()
        };
        app.dispenser_mirror[0] = (potion, 1);
        app.items_potion_kinds[9] = kind;
        let (mx, my) = seat(9);
        let rows = app
            .dispenser_item_tooltip_hover(mx + 8, my + 8)
            .expect("the dispenser kinded mirror row the rows speak");
        assert_eq!(
            rows[0].0,
            App::potion_kind_display_name(kind),
            "the dispenser mirror row the kind lane wears"
        );
        assert!(rows.len() > 1, "the effect rows the dispenser mirror rides");
        // THE GRID'S OWN NONE WALL (the cell the kind never reads —
        // the dispenser store the kind never carries)
        app.dispenser_slots[0] = (potion, 1);
        let (g0x, g0y) = seat(0);
        let rows = app
            .dispenser_item_tooltip_hover(g0x + 8, g0y + 8)
            .expect("the dispenser grid the rows speak");
        assert_eq!(
            rows[0].0,
            item_display_name(mc_registry::item_name(potion).unwrap()),
            "the dispenser grid the NONE wall keeps"
        );
        app.items_potion_kinds[9] = none;
    }
    // THE HOPPER'S OWN RIDE (the mirror head the seat 5 the lane
    // keeps — the s-5 positional the chest map speaks)
    {
        let mut app = App::new(3);
        let cell = surface_cell(&mut app);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::HOPPER);
        for it in app.inventory.items.iter_mut() {
            *it = (0, 0);
        }
        app.open_hopper_for(cell);
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
        let seat = |slot: i16| -> (i32, i32) {
            app.chest_slot_rects
                .iter()
                .find(|&&(_, _, s)| s == slot)
                .map(|&(x, y, _)| (x, y))
                .unwrap()
        };
        app.chest_mirror[0] = (potion, 1);
        app.items_potion_kinds[9] = kind;
        let (mx, my) = seat(5);
        let rows = app
            .hopper_item_tooltip_hover(mx + 8, my + 8)
            .expect("the hopper kinded mirror row the rows speak");
        assert_eq!(
            rows[0].0,
            App::potion_kind_display_name(kind),
            "the hopper mirror row the kind lane wears"
        );
        assert!(rows.len() > 1, "the effect rows the hopper mirror rides");
        // THE FIVE-SEAT ROW'S OWN NONE WALL (the cell the kind never
        // reads — the hopper store the kind never carries)
        app.chest_slots[0] = (potion, 1);
        let (h0x, h0y) = seat(0);
        let rows = app
            .hopper_item_tooltip_hover(h0x + 8, h0y + 8)
            .expect("the hopper seat the rows speak");
        assert_eq!(
            rows[0].0,
            item_display_name(mc_registry::item_name(potion).unwrap()),
            "the hopper seat the NONE wall keeps"
        );
        app.items_potion_kinds[9] = none;
    }
}

/// THE SAVED HOTBAR TAB'S OWN RIDE (the autopush-357 round — javap
/// selectTab's Type.HOTBAR arm the honest silence retires): the tab
/// the strip selects, the 81 seats the nine pages wear (the empty
/// page the paper marker the diagonal seat keeps, the full page the
/// row the stacks wear), the save chord the page banks the rebuild
/// re-walks, the paper's lock the click refuses (the CREATIVE_SLOT_
/// LOCK law the cursor keeps), the page stack the copy gives, the
/// paper's tooltip the hotbarInfo text the keys name.
#[test]
fn the_creative_hotbar_tab_the_pages_wear() {
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    let paper = mc_registry::item_id_by_name("paper").unwrap();
    let mut app = App::new(262);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    // THE SAVE CHORD'S OWN RIDE FIRST (the screen-null gate the chords
    // keep — the page the row banks BEFORE the picker opens)
    app.inventory.items[0] = (sword, 1);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(
        app.on_hotbar_save_load_key(2, true, false),
        "the save chord the page banks"
    );
    assert_eq!(
        app.hotbar_manager[2][0],
        (sword, 1),
        "the page the manager banks"
    );
    app.open_inventory();
    assert_eq!(app.screen, ScreenKind::Creative, "the creative arm opens");
    let hb_tab = (0..CREATIVE_TABS.len())
        .find(|&i| CREATIVE_TABS[i].kind == CreativeTabKind::SavedHotbar)
        .expect("the saved hotbar tab the strip keeps");
    app.select_creative_tab(hb_tab);
    assert_eq!(
        app.creative_tab, hb_tab,
        "the tab the strip selects (the early return the walk retires)"
    );
    assert_eq!(
        app.creative_items.len(),
        81,
        "the nine pages the list wears"
    );
    // THE NINE PAGES' OWN SEATS (the full page the row the stacks
    // wear, the empty pages the papers the diagonal seats keep, the
    // off-diagonal seats the emptiness speaks)
    for i in 0..81usize {
        let (id, count) = app.creative_items[i];
        if i / 9 == 2 {
            let expected = if i % 9 == 0 { (sword, 1) } else { (0, 0) };
            assert_eq!(
                (id, count),
                expected,
                "the saved page the row wears (seat {i})"
            );
        } else if i % 10 == 0 {
            assert_eq!(
                (id, count),
                (paper, 1),
                "the empty page the paper wears (seat {i})"
            );
        } else {
            assert_eq!(
                (id, count),
                (0, 0),
                "the off-diagonal seat the emptiness keeps (seat {i})"
            );
        }
    }
    assert!(
        app.creative_can_scroll(),
        "the 81 seats the scroll law answers"
    );
    // THE PAPER'S OWN TOOLTIP (the hotbarInfo text the keys name —
    // the save activator the chords' own naming speaks, the digit the
    // page's own 1-based number wears)
    app.creative_panel = (100, 50);
    let (id, rows) = app
        .creative_item_tooltip_hover(100 + 9 + 8, 50 + 18 + 8)
        .expect("the paper seat the tooltip answers");
    assert_eq!(id, paper, "the paper seat the gate banks");
    assert_eq!(
        rows[0],
        ("Save hotbar with C + 1".to_string(), [1.0, 1.0, 1.0]),
        "the hotbarInfo text the keys name"
    );
    // THE FULL PAGE'S OWN TOOLTIP (the stack the plain name speaks —
    // the seat the paper walk never names)
    let (id, rows) = app
        .creative_item_tooltip_hover(100 + 9 + 0 * 18 + 8, 50 + 18 + 2 * 18 + 8)
        .expect("the page stack seat the tooltip answers");
    assert_eq!(id, sword, "the page stack the gate banks");
    assert_eq!(
        rows[0].0,
        item_display_name(mc_registry::item_name(sword).unwrap()),
        "the page stack the plain name keeps"
    );
    // THE PAPER'S OWN LOCK (the CREATIVE_SLOT_LOCK the click refuses —
    // the cursor the emptiness keeps)
    app.click_creative_palette(0, 0);
    assert_eq!(app.inv_carried, (0, 0), "the paper lock the click refuses");
    // THE OFF-DIAGONAL EMPTY SEAT (the silence — the take the empty
    // hover never answers)
    app.click_creative_palette(1, 0);
    assert_eq!(app.inv_carried, (0, 0), "the empty seat the silence keeps");
    // THE PAGE STACK'S OWN COPY (the empty carried the take answers —
    // the count-1 law the palette feeds)
    app.click_creative_palette(2 * 9, 0);
    assert_eq!(app.inv_carried, (sword, 1), "the page stack the copy gives");
    // THE SCROLLED PAPER'S OWN SEAT (the offset the absolute index
    // names — the bottom pages the paper walk the scroll law reads)
    app.inv_carried = (0, 0);
    app.creative_scroll = 1.0;
    app.creative_scroll_to(1.0);
    let bottom_cell = 4 * 9 + 8; // the viewport's last row, the last col
    assert_eq!(
        app.creative_hotbar_paper_page(bottom_cell),
        Some(8),
        "the scrolled seat the absolute index names"
    );
}

/// THE SURVIVAL STORE'S OWN CURSOR WORD (the dev-204 round — the
/// offline `inv_carried` the kind seat the brewing stand's own
/// `brewing_carried_kind` word re-wears): the pickup the seat's kind
/// the word banks, the place the word the seat lays, the model's own
/// swap the pairs exchange, the throw the word the entity never
/// carries clears, the close the ride-home row the seat banks, the
/// fresh-open pair the stale word retires, the offhand's 36TH seat
/// the same law the ride answers, the stale non-potion seat the
/// normalized read cleans.
#[test]
fn the_survival_store_the_cursor_word_rides() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let kind2 = 5;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE PICKUP ARM (the seat's kind the word banks, the emptied seat
    // the NONE word wears)
    app.inventory.items[9] = (potion, 1);
    app.items_potion_kinds[9] = kind;
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 9);
    app.click_inventory_slot(ox + 8, oy + 8, 0);
    assert_eq!(app.inv_carried, (potion, 1), "the pickup the carried banks");
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the kinded pickup the word banks"
    );
    assert_eq!(
        app.inventory.items[9],
        (0, 0),
        "the seat the pickup empties"
    );
    assert_eq!(
        app.items_potion_kinds[9], none,
        "the emptied seat the NONE word wears"
    );
    // THE PLACE ARM (the word the new seat lays)
    let (px, py) = inventory_slot_origin(app.inv_panel, 12);
    app.click_inventory_slot(px + 8, py + 8, 0);
    assert_eq!(app.inv_carried, (0, 0), "the place the carried empties");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the placed carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[12],
        (potion, 1),
        "the new seat the stack lays"
    );
    assert_eq!(
        app.items_potion_kinds[12], kind,
        "the new seat the kind lays"
    );
    // THE RE-PICKUP (the seat's kind the word re-banks — the carried
    // the swap's own fuel keeps)
    let (qx, qy) = inventory_slot_origin(app.inv_panel, 12);
    app.click_inventory_slot(qx + 8, qy + 8, 0);
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the re-pickup the word banks"
    );
    // THE MODEL'S OWN SWAP (the kinded carried the dirt seat exchanges
    // — the kind the pairs ride, the word the dirt's NONE keeps)
    app.inventory.items[13] = (dirt, 1);
    let (sx, sy) = inventory_slot_origin(app.inv_panel, 13);
    app.click_inventory_slot(sx + 8, sy + 8, 0);
    assert_eq!(
        app.inventory.items[13],
        (potion, 1),
        "the swap the seat the potion lays"
    );
    assert_eq!(
        app.inv_carried,
        (dirt, 1),
        "the swap the carried the dirt keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the swapped carried the dirt's NONE word wears"
    );
    assert_eq!(
        app.items_potion_kinds[13], kind,
        "the swapped seat the kind keeps"
    );
    // THE THROW'S OWN WORD (the kinded carried the whole throw clears)
    app.inventory.items[13] = (0, 0);
    app.items_potion_kinds[13] = none;
    app.inventory.items[15] = (potion, 1);
    app.items_potion_kinds[15] = kind;
    let (tx, ty) = inventory_slot_origin(app.inv_panel, 15);
    app.click_inventory_slot(tx + 8, ty + 8, 0);
    assert_eq!(
        app.inv_carried,
        (potion, 1),
        "the dirt swap the carried rides"
    );
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the re-pickup the word banks"
    );
    app.throw_carried_outside(0);
    assert_eq!(app.inv_carried, (0, 0), "the throw the carried drops");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the thrown kind the word clears"
    );
    // THE RIDE HOME (the close the receiving empty row the seat banks)
    app.inventory.items[15] = (potion, 1);
    app.items_potion_kinds[15] = kind;
    let (rx, ry) = inventory_slot_origin(app.inv_panel, 15);
    app.click_inventory_slot(rx + 8, ry + 8, 0);
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the ride's pickup the word banks"
    );
    app.close_inventory(ScreenKind::World);
    assert_eq!(app.inv_carried, (0, 0), "the close the carried returns");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the returned carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[0],
        (potion, 1),
        "the ride home the row lays"
    );
    assert_eq!(
        app.items_potion_kinds[0], kind,
        "the receiving row the kind banks"
    );
    // THE FRESH-OPEN PAIR (the stale word the open retires)
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = kind;
    app.open_inventory();
    assert_eq!(app.inv_carried, (0, 0), "the fresh open the carried clears");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the fresh open the word retires"
    );
    // THE OFFHAND'S OWN RIDE (the 36TH seat the same law answers)
    app.inventory.offhand[0] = (potion, 1);
    app.items_potion_kinds[OFFHAND_KIND_SEAT] = kind2;
    let (ox2, oy2) = offhand_slot_origin(app.inv_panel);
    app.click_inventory_slot(ox2 + 8, oy2 + 8, 0);
    assert_eq!(
        app.inv_carried,
        (potion, 1),
        "the offhand pickup the carried banks"
    );
    assert_eq!(
        app.inv_carried_potion_kind, kind2,
        "the offhand's kind the word banks"
    );
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], none,
        "the emptied offhand seat the NONE word wears"
    );
    let (hx, hy) = offhand_slot_origin(app.inv_panel);
    app.click_inventory_slot(hx + 8, hy + 8, 0);
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the placed back the word clears"
    );
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], kind2,
        "the offhand seat the kind lays"
    );
    // THE STALE SEAT'S OWN CLEANSE (the normalized read the non-potion
    // rows never keeps)
    app.inventory.items[20] = (dirt, 5);
    app.items_potion_kinds[20] = kind;
    let (dx, dy) = inventory_slot_origin(app.inv_panel, 20);
    app.click_inventory_slot(dx + 8, dy + 8, 0);
    assert_eq!(
        app.inv_carried,
        (dirt, 5),
        "the dirt pickup the carried banks"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the non-potion pickup the NONE word wears"
    );
    assert_eq!(
        app.items_potion_kinds[20], none,
        "the stale seat the read cleans"
    );
    // THE MERGE'S OWN WORD (the emptied carried the NONE wears, the
    // seat its own kind keeps)
    app.inventory.items[21] = (dirt, 10);
    let (mx2, my2) = inventory_slot_origin(app.inv_panel, 21);
    app.click_inventory_slot(mx2 + 8, my2 + 8, 0);
    assert_eq!(app.inv_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the merged carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[21],
        (dirt, 15),
        "the merge the seat grows"
    );
}

/// THE TABLE'S OWN STORE RIDE (the autopush-358 round — the dev-204
/// law the click_table_slot walk's own store rows re-wear, the SAME
/// four arms the survival screen speaks: the pickup the seat's kind
/// the word banks, the place the word the seat lays, the model's own
/// swap the pairs exchange, the merge the emptied carried the NONE
/// word wears, the stale seat the normalized read the non-potion
/// rows never keeps; the hotbar tail the same law answers, the
/// table's own canvas the offline ghost the word's own tint wears —
/// the wire lane the plain draw the honest wall keeps).
#[test]
fn the_table_store_the_cursor_word_rides() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let kind2 = 5;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    // THE FRESH-OPEN PAIR (the stale word the table door retires)
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = kind;
    app.open_crafting_table();
    assert_eq!(app.inv_carried, (0, 0), "the fresh open the carried clears");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the fresh open the word retires"
    );
    assert_eq!(
        app.screen,
        ScreenKind::Crafting,
        "the table door the walk wears"
    );
    // THE REAL PANEL THE RENDER BANKS (the 355 harness the door wears
    // — the headless mouse-less frame the panel lays)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = app.table_panel;
    let store_seat = |i: usize| -> (i32, i32) {
        let (x, y) = inventory_slot_origin(panel, i);
        (x + 8, y + 8)
    };
    // THE PICKUP ARM (the seat's kind the word banks, the emptied seat
    // the NONE word wears)
    app.inventory.items[9] = (potion, 1);
    app.items_potion_kinds[9] = kind;
    let (ox, oy) = store_seat(9);
    app.click_table_slot(ox, oy, 0);
    assert_eq!(app.inv_carried, (potion, 1), "the pickup the carried banks");
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the kinded pickup the word banks"
    );
    assert_eq!(
        app.inventory.items[9],
        (0, 0),
        "the seat the pickup empties"
    );
    assert_eq!(
        app.items_potion_kinds[9], none,
        "the emptied seat the NONE word wears"
    );
    // THE PLACE ARM (the word the new seat lays)
    let (px, py) = store_seat(12);
    app.click_table_slot(px, py, 0);
    assert_eq!(app.inv_carried, (0, 0), "the place the carried empties");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the placed carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[12],
        (potion, 1),
        "the new seat the stack lays"
    );
    assert_eq!(
        app.items_potion_kinds[12], kind,
        "the new seat the kind lays"
    );
    // THE RE-PICKUP (the seat's kind the word re-banks)
    let (qx, qy) = store_seat(12);
    app.click_table_slot(qx, qy, 0);
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the re-pickup the word banks"
    );
    // THE MODEL'S OWN SWAP (the kinded carried the dirt seat exchanges
    // — the kind the pairs ride, the word the dirt's NONE keeps)
    app.inventory.items[13] = (dirt, 1);
    let (sx, sy) = store_seat(13);
    app.click_table_slot(sx, sy, 0);
    assert_eq!(
        app.inventory.items[13],
        (potion, 1),
        "the swap the seat the potion lays"
    );
    assert_eq!(
        app.inv_carried,
        (dirt, 1),
        "the swap the carried the dirt keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the swapped carried the dirt's NONE word wears"
    );
    assert_eq!(
        app.items_potion_kinds[13], kind,
        "the swapped seat the kind keeps"
    );
    // THE REVERSE EXCHANGE (the dirt carried the kinded seat swaps —
    // the pairs re-bank, the word the potion's own kind rides home)
    app.inventory.items[15] = (potion, 1);
    app.items_potion_kinds[15] = kind;
    let (rx, ry) = store_seat(15);
    app.click_table_slot(rx, ry, 0);
    assert_eq!(
        app.inv_carried,
        (potion, 1),
        "the reverse swap the carried the potion keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the reverse swap the word re-banks"
    );
    assert_eq!(
        app.inventory.items[15],
        (dirt, 1),
        "the reverse swap the seat the dirt lays"
    );
    assert_eq!(
        app.items_potion_kinds[15], none,
        "the dirt seat the NONE word wears"
    );
    // THE THROW'S OWN WORD (the kinded carried the whole throw clears
    // — the table's own outside lane the shared walk re-wears)
    app.throw_carried_outside(0);
    assert_eq!(app.inv_carried, (0, 0), "the throw the carried drops");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the thrown kind the word clears"
    );
    // THE STALE SEAT'S OWN CLEANSE (the normalized read the non-potion
    // rows never keeps)
    app.inventory.items[20] = (dirt, 5);
    app.items_potion_kinds[20] = kind;
    let (dx, dy) = store_seat(20);
    app.click_table_slot(dx, dy, 0);
    assert_eq!(
        app.inv_carried,
        (dirt, 5),
        "the dirt pickup the carried banks"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the non-potion pickup the NONE word wears"
    );
    assert_eq!(
        app.items_potion_kinds[20], none,
        "the stale seat the read cleans"
    );
    // THE MERGE'S OWN WORD (the emptied carried the NONE wears, the
    // seat its own kind keeps)
    app.inventory.items[21] = (dirt, 10);
    let (mx2, my2) = store_seat(21);
    app.click_table_slot(mx2, my2, 0);
    assert_eq!(app.inv_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the merged carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[21],
        (dirt, 15),
        "the merge the seat grows"
    );
    // THE HOTBAR TAIL'S OWN RIDE (the store 0..8 rows the same law
    // answers — the menu 37+i numbering the walk's own map keeps)
    app.inventory.items[5] = (potion, 1);
    app.items_potion_kinds[5] = kind2;
    let (hx, hy) = store_seat(5);
    app.click_table_slot(hx, hy, 0);
    assert_eq!(
        app.inv_carried_potion_kind, kind2,
        "the hotbar pickup the word banks"
    );
    assert_eq!(
        app.items_potion_kinds[5], none,
        "the hotbar seat the NONE word wears"
    );
    app.click_table_slot(hx, hy, 0);
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the hotbar place the word clears"
    );
    assert_eq!(
        app.items_potion_kinds[5], kind2,
        "the hotbar seat the kind lays"
    );
    // THE GHOST'S OWN TINT (the offline arm the word reads — the same
    // carried the word swap the liquid re-colors, the wire lane the
    // plain draw the honest wall keeps; the headless pair the only
    // delta the ghost's own 16x16 wears)
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = kind;
    let (gx, gy) = store_seat(30); // the empty seat the ghost overpaints
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((gx, gy));
        app.render(&mut fb_a, &mut a);
    }
    app.inv_carried_potion_kind = none;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((gx, gy));
        app.render(&mut fb_b, &mut a);
    }
    let mut diff = 0usize;
    for dy in 0..16i32 {
        for dx in 0..16i32 {
            if pixel(&fb_a, gx - 8 + dx, gy - 8 + dy) != pixel(&fb_b, gx - 8 + dx, gy - 8 + dy) {
                diff += 1;
            }
        }
    }
    assert!(diff > 0, "the kinded ghost the tint lane re-colors");
    assert_eq!(
        pixel(&fb_a, panel.0 + 20, panel.1 + 60),
        pixel(&fb_b, panel.0 + 20, panel.1 + 60),
        "the far face the tint never wears"
    );
}

/// THE CREATIVE WRAP'S OWN RIDE (the autopush-359 round — the 358
/// NEXT list's own first candidate the dev-204 law the creative
/// screen's own cursor walks re-wear: the wrapped store rows the
/// four arms the survival screen speaks, the picker's bottom row the
/// same ride answers, the bin's destroy the word drops, the palette
/// eats the emptied carried the same silence wears [the take the
/// kindless vanilla law keeps], the wrapped + the canvas ghosts the
/// rides' word the liquid lane colors).
#[test]
fn the_creative_wrap_the_cursor_word_rides() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let kind2 = 5;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(262);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    assert_eq!(app.screen, ScreenKind::Creative, "the creative arm opens");
    let inv_tab = (0..CREATIVE_TABS.len())
        .find(|&i| CREATIVE_TABS[i].kind == CreativeTabKind::Inventory)
        .expect("the inventory tab the strip keeps");
    app.select_creative_tab(inv_tab);
    app.creative_panel = (100, 50);
    let seat = |i: usize| -> (i32, i32) {
        let (sx, sy) = if i < 9 {
            (9 + i as i32 * 18, 112)
        } else {
            (
                9 + ((i - 9) % 9) as i32 * 18,
                54 + ((i - 9) / 9) as i32 * 18,
            )
        };
        (100 + sx + 8, 50 + sy + 8)
    };
    // THE PICKUP ARM (the wrapped main row the word banks)
    app.inventory.items[9] = (potion, 1);
    app.items_potion_kinds[9] = kind;
    let (ox, oy) = seat(9);
    app.click_creative(ox, oy, 0);
    assert_eq!(app.inv_carried, (potion, 1), "the pickup the carried banks");
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the kinded pickup the word banks"
    );
    assert_eq!(
        app.inventory.items[9],
        (0, 0),
        "the seat the pickup empties"
    );
    assert_eq!(
        app.items_potion_kinds[9], none,
        "the emptied seat the NONE word wears"
    );
    // THE PLACE ARM (the word the new seat lays)
    let (px, py) = seat(12);
    app.click_creative(px, py, 0);
    assert_eq!(app.inv_carried, (0, 0), "the place the carried empties");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the placed carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[12],
        (potion, 1),
        "the new seat the stack lays"
    );
    assert_eq!(
        app.items_potion_kinds[12], kind,
        "the new seat the kind lays"
    );
    // THE RE-PICKUP (the seat's kind the word re-banks)
    let (qx, qy) = seat(12);
    app.click_creative(qx, qy, 0);
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the re-pickup the word banks"
    );
    // THE MODEL'S OWN SWAP (the kinded carried the dirt seat exchanges)
    app.inventory.items[13] = (dirt, 1);
    let (sx, sy) = seat(13);
    app.click_creative(sx, sy, 0);
    assert_eq!(
        app.inventory.items[13],
        (potion, 1),
        "the swap the seat the potion lays"
    );
    assert_eq!(
        app.inv_carried,
        (dirt, 1),
        "the swap the carried the dirt keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the swapped carried the dirt's NONE word wears"
    );
    assert_eq!(
        app.items_potion_kinds[13], kind,
        "the swapped seat the kind keeps"
    );
    // THE WRAPPED HOTBAR ROW'S OWN RIDE (the store 0..8 rows the same
    // law answers — the y-112 row the walk's own map keeps)
    app.inventory.items[5] = (potion, 1);
    app.items_potion_kinds[5] = kind2;
    let (hx, hy) = seat(5);
    app.click_creative(hx, hy, 0);
    assert_eq!(
        app.inv_carried_potion_kind, kind2,
        "the hotbar pickup the word banks"
    );
    assert_eq!(
        app.items_potion_kinds[5], none,
        "the hotbar seat the NONE word wears"
    );
    app.click_creative(hx, hy, 0);
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the hotbar place the word clears"
    );
    assert_eq!(
        app.items_potion_kinds[5], kind2,
        "the hotbar seat the kind lays"
    );
    // THE BIN'S OWN DESTROY (the carried the word drops — the throw's
    // own silence the destroy slot wears)
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = kind;
    app.click_creative(100 + 173 + 8, 50 + 112 + 8, 0);
    assert_eq!(app.inv_carried, (0, 0), "the bin the carried destroys");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the destroyed carried the NONE word wears"
    );
    // THE PALETTE TAKE'S OWN KINDLESS LAW (the picker's copy the word
    // never banks — the no-component vanilla law the ghost's plain
    // draw keeps)
    app.select_creative_tab(0);
    app.creative_palette[2] = (potion, 1);
    let palette_seat = |cell: usize| -> (i32, i32) {
        (
            100 + 9 + (cell % 9) as i32 * 18 + 8,
            50 + 18 + (cell / 9) as i32 * 18 + 8,
        )
    };
    let (tx, ty) = palette_seat(2);
    app.click_creative(tx, ty, 0);
    assert_eq!(app.inv_carried, (potion, 1), "the take the copy banks");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the palette take the kindless law keeps"
    );
    // THE SAME-ITEM DIAL'S OWN SHRINK (the emptied carried the word
    // the same silence wears)
    app.inv_carried_potion_kind = kind;
    app.click_creative(tx, ty, 1); // the shrink(1) — the one bottle eats
    assert_eq!(
        app.inv_carried,
        (potion, 0),
        "the shrink the carried spends"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the shrunk-out carried the NONE word wears"
    );
    // THE PALETTE EAT'S OWN WORD (the different-item hover the whole
    // carried destroys — the word the destroy drops)
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = kind;
    app.creative_palette[3] = (dirt, 1);
    let (ex, ey) = palette_seat(3);
    app.click_creative(ex, ey, 0);
    assert_eq!(app.inv_carried, (0, 0), "the eat the carried destroys");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the eaten carried the NONE word wears"
    );
    // THE PICKER'S OWN BOTTOM ROW (the store walk the same ride
    // answers — the hotbar seat the wrapped row's twin wears)
    app.inventory.items[3] = (potion, 1);
    app.items_potion_kinds[3] = kind2;
    app.click_creative_store(3, 0);
    assert_eq!(
        app.inv_carried,
        (potion, 1),
        "the store pickup the carried banks"
    );
    assert_eq!(
        app.inv_carried_potion_kind, kind2,
        "the store pickup the word banks"
    );
    assert_eq!(
        app.items_potion_kinds[3], none,
        "the store seat the NONE word wears"
    );
    app.click_creative_store(3, 0);
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the store place the word clears"
    );
    assert_eq!(
        app.items_potion_kinds[3], kind2,
        "the store seat the kind lays"
    );
    // THE GHOST'S OWN TINT (the wrapped canvas the word reads — the
    // headless pair the only delta the ghost's own 16x16 wears)
    app.select_creative_tab(inv_tab);
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let panel = app.creative_panel;
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = kind;
    // the empty wrapped seat the ghost overpaints (main row 30 — the
    // walk the store 30 the walk never touched)
    let (gx, gy) = (
        panel.0 + 9 + ((30 - 9) % 9) as i32 * 18 + 8,
        panel.1 + 54 + ((30 - 9) / 9) as i32 * 18 + 8,
    );
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((gx, gy));
        app.render(&mut fb_a, &mut a);
    }
    app.inv_carried_potion_kind = none;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((gx, gy));
        app.render(&mut fb_b, &mut a);
    }
    let mut diff = 0usize;
    for dy in 0..16i32 {
        for dx in 0..16i32 {
            if pixel(&fb_a, gx - 8 + dx, gy - 8 + dy) != pixel(&fb_b, gx - 8 + dx, gy - 8 + dy) {
                diff += 1;
            }
        }
    }
    assert!(diff > 0, "the kinded ghost the tint lane re-colors");
    assert_eq!(
        pixel(&fb_a, panel.0 + 4, panel.1 + 132),
        pixel(&fb_b, panel.0 + 4, panel.1 + 132),
        "the far face the tint never wears"
    );
}

/// THE BREWING SWAP ARM'S OWN RETROFIT (the autopush-360 round — the
/// 359 NEXT list's own candidate the 339 round's dead predicate pair
/// [seat_gone && seat_new] the contradiction the arm never spoke: the
/// real swap the left-press the different-item predict answers, the
/// dev-204 law the survival store's walk wears — the (id, count)
/// pairs the different-item exchange keeps, the kind the pairs
/// exchange; the same-id pair the predict's own no-op the kindless
/// model's honest wall retires).
#[test]
fn the_brewing_swap_the_pairs_exchange_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind_a = crate::brewing_sim::POTION_NIGHT_VISION;
    let kind_c = crate::brewing_sim::POTION_STRONG_SWIFTNESS;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::BREWING_STAND);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the walk reads
    }
    app.open_brewing_for(cell);
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    // THE FIVE-SEAT'S OWN EXCHANGE (the kinded bottle seat the dirt
    // carried swaps — the word the potion's own kind rides home)
    app.brewing_slots[0] = (potion, 1);
    app.brewing_kinds[0] = kind_a;
    app.brewing_carried = (dirt, 1);
    app.brewing_carried_kind = none;
    let (sx, sy) = {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 0)
            .map(|&(x, y, _)| (x + 8, y + 8))
            .unwrap()
    };
    app.click_brewing_slot(sx, sy, 0);
    assert_eq!(
        app.brewing_slots[0],
        (dirt, 1),
        "the swap the seat the dirt lays"
    );
    assert_eq!(
        app.brewing_kinds[0], none,
        "the dirt seat the NONE word wears"
    );
    assert_eq!(
        app.brewing_carried,
        (potion, 1),
        "the swap the carried the potion keeps"
    );
    assert_eq!(
        app.brewing_carried_kind, kind_a,
        "the swapped carried the seat's own kind banks"
    );
    // THE ROW'S OWN EXCHANGE (the kinded store row the dirt carried
    // swaps — the same predicate the row seat answers)
    app.brewing_carried = (dirt, 1);
    app.brewing_carried_kind = none;
    app.inventory.items[14] = (potion, 1); // the menu 10 the items 14 keeps
    app.items_potion_kinds[14] = kind_c;
    let (rx, ry) = {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 10)
            .map(|&(x, y, _)| (x + 8, y + 8))
            .unwrap()
    };
    app.click_brewing_slot(rx, ry, 0);
    assert_eq!(
        app.inventory.items[14],
        (dirt, 1),
        "the row swap the seat the dirt lays"
    );
    assert_eq!(
        app.items_potion_kinds[14], none,
        "the dirt row the NONE word wears"
    );
    assert_eq!(
        app.brewing_carried_kind, kind_c,
        "the row's own kind the cursor banks"
    );
    // THE REVERSE EXCHANGE (the kinded carried the dirt row swaps —
    // the word the carried's own kind lays)
    app.inventory.items[15] = (dirt, 1); // the menu 11 the items 15 keeps
    app.items_potion_kinds[15] = none;
    let (rx2, ry2) = {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 11)
            .map(|&(x, y, _)| (x + 8, y + 8))
            .unwrap()
    };
    app.click_brewing_slot(rx2, ry2, 0);
    assert_eq!(
        app.inventory.items[15],
        (potion, 1),
        "the reverse swap the seat the potion lays"
    );
    assert_eq!(
        app.items_potion_kinds[15], kind_c,
        "the carried's own kind the seat banks"
    );
    assert_eq!(
        app.brewing_carried,
        (dirt, 1),
        "the reverse swap the carried the dirt keeps"
    );
    assert_eq!(
        app.brewing_carried_kind, none,
        "the dirt carried the NONE word wears"
    );
    // THE SAME-ID KIND PAIR'S OWN SWAP (the 377 round — the (id,
    // count) model the kind never carries the wall the component law
    // retires: the same-id DIFFERENT-KIND pair the vanilla
    // isSameItemSameComponents the different items answers — the
    // stacks the identical pairs keep, the kinds the exchange)
    app.brewing_slots[1] = (potion, 1);
    app.brewing_kinds[1] = kind_a;
    app.brewing_carried = (potion, 1);
    app.brewing_carried_kind = kind_c;
    let (sx2, sy2) = {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 1)
            .map(|&(x, y, _)| (x + 8, y + 8))
            .unwrap()
    };
    app.click_brewing_slot(sx2, sy2, 0);
    assert_eq!(
        app.brewing_slots[1],
        (potion, 1),
        "the same-id pair the stacks keep"
    );
    assert_eq!(
        app.brewing_kinds[1], kind_c,
        "the carried's own kind the seat banks"
    );
    assert_eq!(
        app.brewing_carried_kind, kind_a,
        "the seat's own kind the carried banks"
    );
    // THE PICKUP ARM'S OWN REWEAR (the plain pickup the same walk
    // keeps — the retrofit the other arms never moves)
    app.brewing_carried = (0, 0);
    app.brewing_carried_kind = none;
    app.inventory.items[16] = (potion, 1); // the menu 12 the items 16 keeps
    app.items_potion_kinds[16] = kind_a;
    let (px, py) = {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 12)
            .map(|&(x, y, _)| (x + 8, y + 8))
            .unwrap()
    };
    app.click_brewing_slot(px, py, 0);
    assert_eq!(
        app.brewing_carried,
        (potion, 1),
        "the pickup the carried banks"
    );
    assert_eq!(
        app.brewing_carried_kind, kind_a,
        "the pickup the word banks"
    );
    assert_eq!(
        app.items_potion_kinds[16], none,
        "the emptied row the NONE word wears"
    );
    // THE MERGE ARM'S OWN REWEAR (the same-item carried the seat
    // grows — the emptied carried the NONE word wears)
    app.brewing_carried = (dirt, 5);
    app.brewing_carried_kind = none;
    app.inventory.items[17] = (dirt, 10); // the menu 13 the items 17 keeps
    let (mx2, my2) = {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 13)
            .map(|&(x, y, _)| (x + 8, y + 8))
            .unwrap()
    };
    app.click_brewing_slot(mx2, my2, 0);
    assert_eq!(
        app.inventory.items[17],
        (dirt, 15),
        "the merge the seat grows"
    );
    assert_eq!(app.brewing_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.brewing_carried_kind, none,
        "the merged carried the NONE word wears"
    );
}

/// THE CHEST SEAT'S OWN SYNTHETIC RECT (the part-1 harness's own
/// shape the scoped helper re-wears — the mutable borrow the call's
/// end returns, the click walks the pair the conflict never keeps:
/// the seat-closure borrow lesson the 360 record taught).
fn chest_seat(app: &mut App, menu: i16) -> (i32, i32) {
    app.chest_slot_rects = vec![(100, 100, menu)];
    app.mouse_pos = Some((108, 108));
    (108, 108)
}

/// THE CHEST'S OWN CARRIED WORD (the autopush-361 round — the dev-204
/// law the chest's own walk re-wears: the SAME four arms the table's
/// store rows speak, the chest rows the kindless model the honest
/// NONE wall keeps [the (id, count) store the word never carried],
/// the throw the emptied carried the word drops, the close the ride
/// home the receiving seat banks, the ghost the offline word the
/// liquid lane colors — the live lane the plain draw the wire's own
/// wall wears).
#[test]
fn the_chest_carried_the_word_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let kind2 = 5;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(361);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    // THE FRESH-OPEN PAIR (the stale word the chest door retires)
    app.chest_carried = (potion, 1);
    app.chest_carried_potion_kind = kind;
    app.open_chest_for(cell);
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the fresh open the carried clears"
    );
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the fresh open the word retires"
    );
    assert_eq!(
        app.screen,
        ScreenKind::Chest,
        "the chest door the walk wears"
    );
    // THE CLICK SEATS (the synthetic rects the part-1 harness wears —
    // the single chest's 27 rows the menu 0..27 keeps, the main grid
    // 27..54 the items 9..36, the hotbar 54..63 the items 0..9)
    // THE PICKUP ARM (the seat's kind the word banks, the emptied
    // seat the NONE word wears)
    app.inventory.items[18] = (potion, 1); // the menu 36 the main grid wears
    app.items_potion_kinds[18] = kind;
    let (ox, oy) = chest_seat(&mut app, 36);
    app.click_chest_slot(ox, oy, 0);
    assert_eq!(
        app.chest_carried,
        (potion, 1),
        "the pickup the carried banks"
    );
    assert_eq!(
        app.chest_carried_potion_kind, kind,
        "the kinded pickup the word banks"
    );
    assert_eq!(
        app.inventory.items[18],
        (0, 0),
        "the seat the pickup empties"
    );
    assert_eq!(
        app.items_potion_kinds[18], none,
        "the emptied seat the NONE word wears"
    );
    // THE PLACE ARM (the word the new seat lays)
    let (px, py) = chest_seat(&mut app, 40); // the items 22 the main grid wears
    app.click_chest_slot(px, py, 0);
    assert_eq!(app.chest_carried, (0, 0), "the place the carried empties");
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the placed carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[22],
        (potion, 1),
        "the new seat the stack lays"
    );
    assert_eq!(
        app.items_potion_kinds[22], kind,
        "the new seat the kind lays"
    );
    // THE RE-PICKUP (the seat's kind the word re-banks)
    let (qx, qy) = chest_seat(&mut app, 40);
    app.click_chest_slot(qx, qy, 0);
    assert_eq!(
        app.chest_carried_potion_kind, kind,
        "the re-pickup the word banks"
    );
    // THE MODEL'S OWN SWAP (the kinded carried the dirt seat exchanges
    // — the kind the pairs ride, the word the dirt's NONE keeps)
    app.inventory.items[23] = (dirt, 1); // the menu 41 the main grid wears
    let (sx, sy) = chest_seat(&mut app, 41);
    app.click_chest_slot(sx, sy, 0);
    assert_eq!(
        app.inventory.items[23],
        (potion, 1),
        "the swap the seat the potion lays"
    );
    assert_eq!(
        app.chest_carried,
        (dirt, 1),
        "the swap the carried the dirt keeps"
    );
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the swapped carried the dirt's NONE word wears"
    );
    assert_eq!(
        app.items_potion_kinds[23], kind,
        "the swapped seat the kind keeps"
    );
    // THE REVERSE EXCHANGE (the dirt carried the kinded seat swaps —
    // the pairs re-bank, the word the potion's own kind rides home)
    app.inventory.items[25] = (potion, 1); // the menu 43 the main grid wears
    app.items_potion_kinds[25] = kind;
    let (rx, ry) = chest_seat(&mut app, 43);
    app.click_chest_slot(rx, ry, 0);
    assert_eq!(
        app.chest_carried,
        (potion, 1),
        "the reverse swap the carried the potion keeps"
    );
    assert_eq!(
        app.chest_carried_potion_kind, kind,
        "the reverse swap the word re-banks"
    );
    assert_eq!(
        app.inventory.items[25],
        (dirt, 1),
        "the reverse swap the seat the dirt lays"
    );
    assert_eq!(
        app.items_potion_kinds[25], none,
        "the dirt seat the NONE word wears"
    );
    // THE CHEST ROW'S OWN BANK (the 363 round — the place the row's
    // word the bank keeps, the pickup the banked word the carried
    // lends)
    let (cx, cy) = chest_seat(&mut app, 3); // the chest's own grid the menu 3 wears
    app.click_chest_slot(cx, cy, 0);
    assert_eq!(
        app.chest_slots[3],
        (potion, 1),
        "the chest row the stack lays"
    );
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the placed carried the NONE word wears"
    );
    assert_eq!(
        app.chest_row_potion_kinds[3], kind,
        "the chest row the word banks"
    );
    let (cx2, cy2) = chest_seat(&mut app, 3);
    app.click_chest_slot(cx2, cy2, 0);
    assert_eq!(
        app.chest_carried,
        (potion, 1),
        "the chest pickup the carried banks"
    );
    assert_eq!(
        app.chest_carried_potion_kind, kind,
        "the chest row the banked word lends"
    );
    // THE CARRIED THE CHEST ROW HOLDS BACK (the same-id pair the
    // predict's own no-op keeps — the 360 record's wall the re-arm
    // the empty cursor wears)
    let (cx3, cy3) = chest_seat(&mut app, 5);
    app.click_chest_slot(cx3, cy3, 0);
    assert_eq!(
        app.chest_slots[5],
        (potion, 1),
        "the chest row the stack holds"
    );
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the held back the cursor empties"
    );
    // THE THROW'S OWN WORD (the kinded carried the whole throw clears)
    app.inventory.items[26] = (potion, 1); // the menu 44 the main grid wears
    app.items_potion_kinds[26] = kind2;
    let (tx, ty) = chest_seat(&mut app, 44);
    app.click_chest_slot(tx, ty, 0);
    assert_eq!(
        app.chest_carried_potion_kind, kind2,
        "the re-armed carried the word banks"
    );
    app.throw_chest_carried_outside(0);
    assert_eq!(app.chest_carried, (0, 0), "the throw the carried drops");
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the thrown kind the word clears"
    );
    // THE STALE SEAT'S OWN CLEANSE (the normalized read the non-potion
    // rows never keeps)
    app.inventory.items[30] = (dirt, 5); // the menu 48 the main grid wears
    app.items_potion_kinds[30] = kind;
    let (dx, dy) = chest_seat(&mut app, 48);
    app.click_chest_slot(dx, dy, 0);
    assert_eq!(
        app.chest_carried,
        (dirt, 5),
        "the dirt pickup the carried banks"
    );
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the non-potion pickup the NONE word wears"
    );
    assert_eq!(
        app.items_potion_kinds[30], none,
        "the stale seat the read cleans"
    );
    // THE MERGE'S OWN WORD (the emptied carried the NONE wears, the
    // seat its own kind keeps)
    app.inventory.items[31] = (dirt, 10); // the menu 49 the main grid wears
    let (mx2, my2) = chest_seat(&mut app, 49);
    app.click_chest_slot(mx2, my2, 0);
    assert_eq!(app.chest_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the merged carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[31],
        (dirt, 15),
        "the merge the seat grows"
    );
    // THE GHOST'S OWN TINT (the offline arm the word reads — the same
    // carried the word swap the liquid re-colors, the live lane the
    // plain draw the honest wall keeps; the headless pair the only
    // delta the ghost's own 16x16 wears)
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let assets_root = assets_root();
    app.chest_carried = (potion, 1);
    app.chest_carried_potion_kind = kind;
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((240, 60)); // the ghost the panel's right wears
        app.render(&mut fb_a, &mut a);
    }
    app.chest_carried_potion_kind = none;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((240, 60));
        app.render(&mut fb_b, &mut a);
    }
    let mut diff = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_a, 240 - 8 + gx, 60 - 8 + gy) != pixel(&fb_b, 240 - 8 + gx, 60 - 8 + gy) {
                diff += 1;
            }
        }
    }
    assert!(diff > 0, "the kinded ghost the tint lane re-colors");
    assert_eq!(
        pixel(&fb_a, 20, 160),
        pixel(&fb_b, 20, 160),
        "the far face the tint never wears"
    );
    // THE RIDE HOME'S OWN WORD (the close the receiving seat banks —
    // the free-slot scan the add's own law the landing re-speaks, the
    // cursor's own emptiness the silence wears)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    for k in app.items_potion_kinds.iter_mut() {
        *k = none;
    }
    app.chest_carried = (potion, 1);
    app.chest_carried_potion_kind = kind2;
    app.on_escape();
    assert_eq!(
        app.inventory.items[0],
        (potion, 1),
        "the ride home the seat banks"
    );
    assert_eq!(
        app.items_potion_kinds[0], kind2,
        "the ride home the word banks"
    );
    assert_eq!(app.chest_carried, (0, 0), "the close the cursor clears");
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the close the word retires"
    );
}

fn furnace_seat(app: &mut App, menu: i16) -> (i32, i32) {
    app.furnace_slot_rects = vec![(100, 100, menu)];
    app.mouse_pos = Some((108, 108));
    (108, 108)
}

fn dispenser_seat(app: &mut App, menu: i16) -> (i32, i32) {
    app.dispenser_slot_rects = vec![(100, 100, menu)];
    app.mouse_pos = Some((108, 108));
    (108, 108)
}

/// THE FURNACE CARRIED'S OWN KIND WORD (the autopush-362 round — the
/// dev-204 law the furnace's own walk re-wears: the SAME four arms the
/// chest's walk speaks [the pickup the seat's kind the word banks, the
/// place the word the seat lays, the model's own swap the pairs
/// exchange, the merge the emptied carried the NONE word wears], the
/// furnace's own three the kindless store the honest NONE wall keeps
/// [the (id, count) store the word never carried, the pickup the
/// silence the stale seat never lends, the place into the input the
/// word retires, the result cell the mayPlace gate the word keeps],
/// the throw the emptied carried the word drops, the stale seat the
/// normalized read cleans, the ghost the offline word the liquid lane
/// colors — the live lane the plain draw the honest wall keeps, the
/// close the ride home the receiving seat banks).
#[test]
fn the_furnace_carried_the_word_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let kind2 = 5;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(362);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::FURNACE);
    // THE FRESH-OPEN PAIR (the stale word the furnace door retires)
    app.furnace_carried = (potion, 1);
    app.furnace_carried_potion_kind = kind;
    app.open_furnace(cell);
    assert_eq!(
        app.furnace_carried,
        (0, 0),
        "the fresh open the carried clears"
    );
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the fresh open the word retires"
    );
    assert_eq!(
        app.screen,
        ScreenKind::Furnace,
        "the furnace door the walk wears"
    );
    // THE PICKUP ARM (the seat's kind the word banks — the menu 12
    // the main grid's items 18 the write-through speaks)
    app.inventory.items[18] = (potion, 1);
    app.items_potion_kinds[18] = kind;
    let (ox, oy) = furnace_seat(&mut app, 12);
    app.click_furnace_slot(ox, oy, 0);
    assert_eq!(
        app.furnace_carried,
        (potion, 1),
        "the pickup the carried banks"
    );
    assert_eq!(
        app.furnace_carried_potion_kind, kind,
        "the kinded pickup the word banks"
    );
    assert_eq!(
        app.inventory.items[18],
        (0, 0),
        "the seat the pickup empties"
    );
    assert_eq!(
        app.items_potion_kinds[18], none,
        "the emptied seat the NONE word wears"
    );
    assert_eq!(
        app.furnace_slots[12],
        (0, 0),
        "the projection the pickup empties"
    );
    // THE PLACE ARM (the word the new seat lays)
    let (px, py) = furnace_seat(&mut app, 16); // the items 22 the main grid wears
    app.click_furnace_slot(px, py, 0);
    assert_eq!(app.furnace_carried, (0, 0), "the place the carried empties");
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the placed carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[22],
        (potion, 1),
        "the new seat the stack lays"
    );
    assert_eq!(
        app.items_potion_kinds[22], kind,
        "the new seat the kind lays"
    );
    // THE RE-PICKUP (the seat's kind the word re-banks)
    let (qx, qy) = furnace_seat(&mut app, 16);
    app.click_furnace_slot(qx, qy, 0);
    assert_eq!(
        app.furnace_carried_potion_kind, kind,
        "the re-pickup the word banks"
    );
    // THE MODEL'S OWN SWAP (the kinded carried the dirt seat exchanges
    // — the kind the pairs ride, the word the dirt's NONE keeps)
    app.inventory.items[23] = (dirt, 1); // the menu 17 the main grid wears
    let (sx, sy) = furnace_seat(&mut app, 17);
    app.click_furnace_slot(sx, sy, 0);
    assert_eq!(
        app.inventory.items[23],
        (potion, 1),
        "the swap the seat the potion lays"
    );
    assert_eq!(
        app.furnace_carried,
        (dirt, 1),
        "the swap the carried the dirt keeps"
    );
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the swapped carried the dirt's NONE word wears"
    );
    assert_eq!(
        app.items_potion_kinds[23], kind,
        "the swapped seat the kind keeps"
    );
    // THE REVERSE EXCHANGE (the dirt carried the kinded seat swaps —
    // the pairs re-bank, the word the potion's own kind rides home)
    app.inventory.items[25] = (potion, 1); // the menu 19 the main grid wears
    app.items_potion_kinds[25] = kind;
    let (rx, ry) = furnace_seat(&mut app, 19);
    app.click_furnace_slot(rx, ry, 0);
    assert_eq!(
        app.furnace_carried,
        (potion, 1),
        "the reverse swap the carried the potion keeps"
    );
    assert_eq!(
        app.furnace_carried_potion_kind, kind,
        "the reverse swap the word re-banks"
    );
    assert_eq!(
        app.inventory.items[25],
        (dirt, 1),
        "the reverse swap the seat the dirt lays"
    );
    assert_eq!(
        app.items_potion_kinds[25], none,
        "the dirt seat the NONE word wears"
    );
    // THE FURNACE SEAT'S OWN HONEST WALL (the input cell the word
    // retires, the re-pickup the silence lends — the kindless store
    // the word never carried)
    let (cx, cy) = furnace_seat(&mut app, 0);
    app.click_furnace_slot(cx, cy, 0);
    assert_eq!(
        app.furnace_slots[0],
        (potion, 1),
        "the furnace seat the stack lays"
    );
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the kindless seat the word retires"
    );
    let (cx2, cy2) = furnace_seat(&mut app, 0);
    app.click_furnace_slot(cx2, cy2, 0);
    assert_eq!(
        app.furnace_carried,
        (potion, 1),
        "the furnace pickup the carried banks"
    );
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the kindless seat the NONE word lends"
    );
    // THE RESULT CELL'S OWN PLACE GATE (the mayPlace the walk returns —
    // the carried the gate keeps, the word the same carried wears)
    app.furnace_carried_potion_kind = kind2;
    let (gx, gy) = furnace_seat(&mut app, 2);
    app.click_furnace_slot(gx, gy, 0);
    assert_eq!(
        app.furnace_carried,
        (potion, 1),
        "the result gate the carried holds"
    );
    assert_eq!(
        app.furnace_carried_potion_kind, kind2,
        "the result gate the word keeps"
    );
    // THE THROW'S OWN WORD (the kinded carried the whole throw clears)
    app.throw_furnace_carried_outside(0);
    assert_eq!(app.furnace_carried, (0, 0), "the throw the carried drops");
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the thrown kind the word clears"
    );
    // THE STALE SEAT'S OWN CLEANSE (the normalized read the non-potion
    // rows never keeps)
    app.inventory.items[30] = (dirt, 5); // the menu 24 the main grid wears
    app.items_potion_kinds[30] = kind;
    let (dx, dy) = furnace_seat(&mut app, 24);
    app.click_furnace_slot(dx, dy, 0);
    assert_eq!(
        app.furnace_carried,
        (dirt, 5),
        "the dirt pickup the carried banks"
    );
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the non-potion pickup the NONE word wears"
    );
    assert_eq!(
        app.items_potion_kinds[30], none,
        "the stale seat the read cleans"
    );
    // THE MERGE'S OWN WORD (the emptied carried the NONE wears, the
    // seat its own kind keeps)
    app.inventory.items[31] = (dirt, 10); // the menu 25 the main grid wears
    let (mx2, my2) = furnace_seat(&mut app, 25);
    app.click_furnace_slot(mx2, my2, 0);
    assert_eq!(app.furnace_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the merged carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[31],
        (dirt, 15),
        "the merge the seat grows"
    );
    // THE GHOST'S OWN TINT (the offline arm the word reads — the same
    // carried the word swap the liquid re-colors, the live lane the
    // plain draw the honest wall keeps; the headless pair the only
    // delta the ghost's own 16x16 wears)
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let assets_root = assets_root();
    app.furnace_carried = (potion, 1);
    app.furnace_carried_potion_kind = kind;
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((240, 60)); // the ghost the panel's right wears
        app.render(&mut fb_a, &mut a);
    }
    app.furnace_carried_potion_kind = none;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((240, 60));
        app.render(&mut fb_b, &mut a);
    }
    let mut diff = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_a, 240 - 8 + gx, 60 - 8 + gy) != pixel(&fb_b, 240 - 8 + gx, 60 - 8 + gy) {
                diff += 1;
            }
        }
    }
    assert!(diff > 0, "the kinded ghost the tint lane re-colors");
    assert_eq!(
        pixel(&fb_a, 20, 160),
        pixel(&fb_b, 20, 160),
        "the far face the tint never wears"
    );
    // THE RIDE HOME'S OWN WORD (the close the receiving seat banks —
    // the free-slot scan the add's own law the landing re-speaks, the
    // cursor's own emptiness the silence wears)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    for k in app.items_potion_kinds.iter_mut() {
        *k = none;
    }
    app.furnace_carried = (potion, 1);
    app.furnace_carried_potion_kind = kind2;
    app.on_escape();
    assert_eq!(
        app.inventory.items[0],
        (potion, 1),
        "the ride home the seat banks"
    );
    assert_eq!(
        app.items_potion_kinds[0], kind2,
        "the ride home the word banks"
    );
    assert_eq!(app.furnace_carried, (0, 0), "the close the cursor clears");
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the close the word retires"
    );
}

/// THE DISPENSER CARRIED'S OWN KIND WORD (the autopush-362 round — the
/// dev-204 law the dispenser's own walk re-wears: the SAME four arms
/// the chest's walk speaks, the nine the kindless store the honest
/// NONE wall keeps [the (id, count) store the word never carried, the
/// pickup the silence the stale seat never lends, the place into the
/// grid the word retires], the throw the emptied carried the word
/// drops, the stale seat the normalized read cleans, the ghost the
/// offline word the liquid lane colors — the live lane the plain draw
/// the honest wall keeps, the close the ride home the receiving seat
/// banks).
#[test]
fn the_dispenser_carried_the_word_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let kind2 = 5;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(362);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
    // THE FRESH-OPEN PAIR (the stale word the dispenser door retires)
    app.dispenser_carried = (potion, 1);
    app.dispenser_carried_potion_kind = kind;
    app.open_dispenser_for(cell);
    assert_eq!(
        app.dispenser_carried,
        (0, 0),
        "the fresh open the carried clears"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the fresh open the word retires"
    );
    assert_eq!(
        app.screen,
        ScreenKind::Dispenser,
        "the dispenser door the walk wears"
    );
    // THE PICKUP ARM (the seat's kind the word banks — the menu 18
    // the main grid's items 18 the same number wears)
    app.inventory.items[18] = (potion, 1);
    app.items_potion_kinds[18] = kind;
    let (ox, oy) = dispenser_seat(&mut app, 18);
    app.click_dispenser_slot(ox, oy, 0);
    assert_eq!(
        app.dispenser_carried,
        (potion, 1),
        "the pickup the carried banks"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, kind,
        "the kinded pickup the word banks"
    );
    assert_eq!(
        app.inventory.items[18],
        (0, 0),
        "the seat the pickup empties"
    );
    assert_eq!(
        app.items_potion_kinds[18], none,
        "the emptied seat the NONE word wears"
    );
    // THE PLACE ARM (the word the new seat lays)
    let (px, py) = dispenser_seat(&mut app, 22); // the items 22 the main grid wears
    app.click_dispenser_slot(px, py, 0);
    assert_eq!(
        app.dispenser_carried,
        (0, 0),
        "the place the carried empties"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the placed carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[22],
        (potion, 1),
        "the new seat the stack lays"
    );
    assert_eq!(
        app.items_potion_kinds[22], kind,
        "the new seat the kind lays"
    );
    // THE RE-PICKUP (the seat's kind the word re-banks)
    let (qx, qy) = dispenser_seat(&mut app, 22);
    app.click_dispenser_slot(qx, qy, 0);
    assert_eq!(
        app.dispenser_carried_potion_kind, kind,
        "the re-pickup the word banks"
    );
    // THE MODEL'S OWN SWAP (the kinded carried the dirt seat exchanges
    // — the kind the pairs ride, the word the dirt's NONE keeps)
    app.inventory.items[23] = (dirt, 1); // the menu 23 the main grid wears
    let (sx, sy) = dispenser_seat(&mut app, 23);
    app.click_dispenser_slot(sx, sy, 0);
    assert_eq!(
        app.inventory.items[23],
        (potion, 1),
        "the swap the seat the potion lays"
    );
    assert_eq!(
        app.dispenser_carried,
        (dirt, 1),
        "the swap the carried the dirt keeps"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the swapped carried the dirt's NONE word wears"
    );
    assert_eq!(
        app.items_potion_kinds[23], kind,
        "the swapped seat the kind keeps"
    );
    // THE REVERSE EXCHANGE (the dirt carried the kinded seat swaps —
    // the pairs re-bank, the word the potion's own kind rides home)
    app.inventory.items[25] = (potion, 1); // the menu 25 the main grid wears
    app.items_potion_kinds[25] = kind;
    let (rx, ry) = dispenser_seat(&mut app, 25);
    app.click_dispenser_slot(rx, ry, 0);
    assert_eq!(
        app.dispenser_carried,
        (potion, 1),
        "the reverse swap the carried the potion keeps"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, kind,
        "the reverse swap the word re-banks"
    );
    assert_eq!(
        app.inventory.items[25],
        (dirt, 1),
        "the reverse swap the seat the dirt lays"
    );
    assert_eq!(
        app.items_potion_kinds[25], none,
        "the dirt seat the NONE word wears"
    );
    // THE NINE'S OWN HONEST WALL (the grid cell the word retires, the
    // re-pickup the silence lends — the kindless store the word never
    // carried)
    let (cx, cy) = dispenser_seat(&mut app, 0);
    app.click_dispenser_slot(cx, cy, 0);
    assert_eq!(
        app.dispenser_slots[0],
        (potion, 1),
        "the dispenser seat the stack lays"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the kindless seat the word retires"
    );
    let (cx2, cy2) = dispenser_seat(&mut app, 0);
    app.click_dispenser_slot(cx2, cy2, 0);
    assert_eq!(
        app.dispenser_carried,
        (potion, 1),
        "the dispenser pickup the carried banks"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the kindless seat the NONE word lends"
    );
    // THE THROW'S OWN WORD (the kinded carried the whole throw clears)
    app.throw_dispenser_carried_outside(0);
    assert_eq!(app.dispenser_carried, (0, 0), "the throw the carried drops");
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the thrown kind the word clears"
    );
    // THE STALE SEAT'S OWN CLEANSE (the normalized read the non-potion
    // rows never keeps)
    app.inventory.items[30] = (dirt, 5); // the menu 30 the main grid wears
    app.items_potion_kinds[30] = kind;
    let (dx, dy) = dispenser_seat(&mut app, 30);
    app.click_dispenser_slot(dx, dy, 0);
    assert_eq!(
        app.dispenser_carried,
        (dirt, 5),
        "the dirt pickup the carried banks"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the non-potion pickup the NONE word wears"
    );
    assert_eq!(
        app.items_potion_kinds[30], none,
        "the stale seat the read cleans"
    );
    // THE MERGE'S OWN WORD (the emptied carried the NONE wears, the
    // seat its own kind keeps)
    app.inventory.items[31] = (dirt, 10); // the menu 31 the main grid wears
    let (mx2, my2) = dispenser_seat(&mut app, 31);
    app.click_dispenser_slot(mx2, my2, 0);
    assert_eq!(
        app.dispenser_carried,
        (0, 0),
        "the merge the carried empties"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the merged carried the NONE word wears"
    );
    assert_eq!(
        app.inventory.items[31],
        (dirt, 15),
        "the merge the seat grows"
    );
    // THE GHOST'S OWN TINT (the offline arm the word reads — the same
    // carried the word swap the liquid re-colors, the live lane the
    // plain draw the honest wall keeps; the headless pair the only
    // delta the ghost's own 16x16 wears)
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let assets_root = assets_root();
    app.dispenser_carried = (potion, 1);
    app.dispenser_carried_potion_kind = kind;
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((240, 60)); // the ghost the panel's right wears
        app.render(&mut fb_a, &mut a);
    }
    app.dispenser_carried_potion_kind = none;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = Some((240, 60));
        app.render(&mut fb_b, &mut a);
    }
    let mut diff = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_a, 240 - 8 + gx, 60 - 8 + gy) != pixel(&fb_b, 240 - 8 + gx, 60 - 8 + gy) {
                diff += 1;
            }
        }
    }
    assert!(diff > 0, "the kinded ghost the tint lane re-colors");
    assert_eq!(
        pixel(&fb_a, 20, 160),
        pixel(&fb_b, 20, 160),
        "the far face the tint never wears"
    );
    // THE RIDE HOME'S OWN WORD (the close the receiving seat banks —
    // the free-slot scan the add's own law the landing re-speaks, the
    // cursor's own emptiness the silence wears)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    for k in app.items_potion_kinds.iter_mut() {
        *k = none;
    }
    app.dispenser_carried = (potion, 1);
    app.dispenser_carried_potion_kind = kind2;
    app.on_escape();
    assert_eq!(
        app.inventory.items[0],
        (potion, 1),
        "the ride home the seat banks"
    );
    assert_eq!(
        app.items_potion_kinds[0], kind2,
        "the ride home the word banks"
    );
    assert_eq!(app.dispenser_carried, (0, 0), "the close the cursor clears");
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the close the word retires"
    );
}

/// THE CHEST ROWS' OWN KIND BANK (the autopush-363 round — the dev-204
/// law the stored rows the world's own parallel map answers: the place
/// the row's word the bank keeps, the reopen the same words load [the
/// stale silence the kindless store wore the bank retires], the pickup
/// the banked word the carried lends, the model's own swap the pairs
/// through the row exchange, the quick-move the source's word the
/// destinations bank [the drained source the NONE retires], the render
/// the row's word the liquid lane colors, the flush the words the
/// per-cell entries ride home to).
#[test]
fn the_chest_rows_the_word_bank_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let kind2 = 5;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(363);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    app.open_chest_for(cell);
    assert_eq!(
        app.screen,
        ScreenKind::Chest,
        "the chest door the walk wears"
    );
    assert!(
        app.chest_row_potion_kinds.iter().all(|&k| k == none),
        "the fresh open the rows the NONE bank wears"
    );
    // THE PLACE'S OWN BANK (the kinded carried the row's word keeps —
    // the carried the NONE word wears)
    app.inventory.items[18] = (potion, 1); // the menu 36 the main grid wears
    app.items_potion_kinds[18] = kind;
    let (ox, oy) = chest_seat(&mut app, 36);
    app.click_chest_slot(ox, oy, 0);
    let (px, py) = chest_seat(&mut app, 3); // the chest's own grid the menu 3 wears
    app.click_chest_slot(px, py, 0);
    assert_eq!(
        app.chest_slots[3],
        (potion, 1),
        "the chest row the stack lays"
    );
    assert_eq!(
        app.chest_row_potion_kinds[3], kind,
        "the placed row the word banks"
    );
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the placed carried the NONE word wears"
    );
    // THE RENDER'S OWN ROW TINT (the row's word the liquid lane colors
    // — the headless pair the only delta the row's own icon wears, the
    // far face the tint never wears)
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let assets_root = assets_root();
    let mut fb_a = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.render(&mut fb_a, &mut a);
    }
    app.chest_row_potion_kinds[3] = none;
    let mut fb_b = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.render(&mut fb_b, &mut a);
    }
    app.chest_row_potion_kinds[3] = kind; // the pair's own state restores
    let mut diff = 0usize;
    for y in 0..180i32 {
        for x in 0..320i32 {
            if pixel(&fb_a, x, y) != pixel(&fb_b, x, y) {
                diff += 1;
            }
        }
    }
    assert!(diff > 0, "the kinded row the tint lane re-colors");
    assert_eq!(
        pixel(&fb_a, 20, 160),
        pixel(&fb_b, 20, 160),
        "the far face the tint never wears"
    );
    // THE REOPEN'S OWN BANK (the close the words the map entry rides
    // home to, the open the same words load — the stale silence the
    // kindless store wore the bank retires)
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::World, "the close the world returns");
    app.open_chest_for(cell);
    assert_eq!(
        app.chest_row_potion_kinds[3], kind,
        "the reopen the banked word loads"
    );
    // THE PICKUP'S OWN RIDE (the banked word the carried lends, the
    // row the NONE bank wears)
    let (qx, qy) = chest_seat(&mut app, 3);
    app.click_chest_slot(qx, qy, 0);
    assert_eq!(
        app.chest_carried,
        (potion, 1),
        "the pickup the carried banks"
    );
    assert_eq!(
        app.chest_carried_potion_kind, kind,
        "the banked row the word lends"
    );
    assert_eq!(
        app.chest_row_potion_kinds[3], none,
        "the emptied row the NONE bank wears"
    );
    // THE MODEL'S OWN SWAP (the dirt carried the kinded row exchanges
    // — the pairs through the row the words exchange)
    let (sx, sy) = chest_seat(&mut app, 5);
    app.click_chest_slot(sx, sy, 0); // the re-place the row 5 banks
    assert_eq!(
        app.chest_row_potion_kinds[5], kind,
        "the re-placed row the word banks"
    );
    app.inventory.items[30] = (dirt, 5); // the menu 48 the main grid wears
    let (dx, dy) = chest_seat(&mut app, 48);
    app.click_chest_slot(dx, dy, 0);
    let (wx, wy) = chest_seat(&mut app, 5);
    app.click_chest_slot(wx, wy, 0);
    assert_eq!(
        app.chest_slots[5],
        (dirt, 5),
        "the swap the row the dirt lays"
    );
    assert_eq!(
        app.chest_row_potion_kinds[5], none,
        "the dirt row the NONE bank wears"
    );
    assert_eq!(
        app.chest_carried,
        (potion, 1),
        "the swap the carried the potion keeps"
    );
    assert_eq!(
        app.chest_carried_potion_kind, kind,
        "the swapped carried the word banks"
    );
    // THE QUICK-MOVE'S OWN RIDE (the inventory source the row's word
    // the bank lands — the drained source the NONE retires)
    app.throw_chest_carried_outside(0); // the carried the throw drops
    app.inventory.items[20] = (potion, 1); // the menu 38 the main grid wears
    app.items_potion_kinds[20] = kind2;
    app.shift_down = true;
    let (mx2, my2) = chest_seat(&mut app, 38);
    app.click_chest_slot(mx2, my2, 0);
    assert_eq!(
        app.chest_slots[0],
        (potion, 1),
        "the quick-move the row the stack lands"
    );
    assert_eq!(
        app.chest_row_potion_kinds[0], kind2,
        "the quick-move the row's word banks"
    );
    assert_eq!(
        app.inventory.items[20],
        (0, 0),
        "the drained source the stack retires"
    );
    assert_eq!(
        app.items_potion_kinds[20], none,
        "the drained source the NONE word wears"
    );
    // THE RIDE BACK (the row source the player row's word banks — the
    // hotbar row the rev order lands)
    let (bx, by) = chest_seat(&mut app, 0);
    app.click_chest_slot(bx, by, 0);
    assert_eq!(app.chest_slots[0], (0, 0), "the ride back the row empties");
    assert_eq!(
        app.inventory.items[8],
        (potion, 1),
        "the ride back the hotbar row banks"
    );
    assert_eq!(
        app.items_potion_kinds[8], kind2,
        "the ride back the word banks"
    );
    assert_eq!(
        app.chest_row_potion_kinds[0], none,
        "the emptied row the NONE bank wears"
    );
    app.shift_down = false;
}

/// THE CHEST ROWS' OWN KIND BANK'S DISK RIDE (the autopush-364 round —
/// the 363 bank the file's own row wears: the close the words the
/// "chest_row_kinds" list the flush banks, the fresh app the
/// once-flag's parse the same words load [the restart the
/// session-only silence the disk retires], the ender jar's own row
/// the shared store's parallel banks, the pre-364 file the tolerant
/// parse the honest NONE rows keep — never a guessed kind the absence
/// invents).
#[test]
fn the_chest_kinds_the_disk_ride_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let kind2 = 5;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let dir = std::env::temp_dir().join(format!("mc2rs-rowkinds-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("the temp dir");
    let file = dir.join("rowkinds-round.nbt");
    let _ = std::fs::remove_file(&file);
    let mut app = App::new(364);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.container_stores_file = Some(file.clone());
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    // THE WORD'S OWN RIDE OUT (the place the row's word banks, the
    // close the flush the file write wears)
    app.open_chest_for(cell);
    app.inventory.items[18] = (potion, 1); // the menu 36 the main grid wears
    app.items_potion_kinds[18] = kind;
    let (ox, oy) = chest_seat(&mut app, 36);
    app.click_chest_slot(ox, oy, 0);
    let (px, py) = chest_seat(&mut app, 3);
    app.click_chest_slot(px, py, 0);
    assert_eq!(
        app.chest_row_potion_kinds[3], kind,
        "the placed row the word banks"
    );
    app.on_escape();
    assert!(file.exists(), "the close the file write banks");
    // THE ENDER JAR'S OWN WORD (the shared row the same session the
    // disk banks — the every-cell binding the file's own row speaks)
    let ecell = (c, surface + 1, c + 5);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(ecell.0, ecell.1, ecell.2, crate::world::ENDER_CHEST);
    app.open_chest_for(ecell);
    app.inventory.items[21] = (potion, 1); // the menu 39 the main grid wears
    app.items_potion_kinds[21] = kind2;
    let (ex, ey) = chest_seat(&mut app, 39);
    app.click_chest_slot(ex, ey, 0);
    let (ex2, ey2) = chest_seat(&mut app, 7);
    app.click_chest_slot(ex2, ey2, 0);
    assert_eq!(
        app.chest_row_potion_kinds[7], kind2,
        "the ender row the word banks"
    );
    app.on_escape();
    // THE FILE'S OWN SHAPE (the two rows the root wears — the
    // entries' own triple + the Kinds list the Int row speaks)
    let bytes = std::fs::read(&file).expect("the banked file reads");
    let (_, root) =
        mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the parse serves");
    let kinds_list = root
        .get("chest_row_kinds")
        .and_then(|v| v.as_list())
        .expect("the kinds list");
    assert_eq!(kinds_list.len(), 1, "the one opened cell rides");
    let row = kinds_list[0]
        .get("Kinds")
        .and_then(|v| v.as_list())
        .expect("the Kinds list");
    assert_eq!(row.len(), 27, "the 27-row the disk banks");
    assert_eq!(row[3].as_i32(), Some(kind), "the banked word rides");
    let ender_row = root
        .get("ender_row_kinds")
        .and_then(|v| v.as_list())
        .expect("the ender kinds row");
    assert_eq!(ender_row.len(), 27, "the jar's own row the disk banks");
    assert_eq!(ender_row[7].as_i32(), Some(kind2), "the jar's word rides");
    // THE RESTART'S OWN LOAD (the fresh app the once-flag's parse the
    // words bank — the session-only silence the disk retires: the
    // chest cell the banked word the open composes, the ender the
    // every-cell binding the same jar loads)
    let mut fresh = App::new(364);
    fresh.enter_world();
    fresh.container_stores_file = Some(file.clone());
    {
        let w = fresh.offline_world.as_mut().unwrap();
        w.set(cell.0, cell.1, cell.2, crate::world::CHEST);
        w.set(ecell.0, ecell.1, ecell.2, crate::world::ENDER_CHEST);
    }
    fresh.open_chest_for(cell);
    assert_eq!(
        fresh.chest_slots[3],
        (potion, 1),
        "the banked stack the restart wears"
    );
    assert_eq!(
        fresh.chest_row_potion_kinds[3], kind,
        "the banked word the restart loads"
    );
    fresh.on_escape();
    fresh.open_chest_for(ecell);
    assert_eq!(
        fresh.chest_slots[7],
        (potion, 1),
        "the jar's stack the restart wears"
    );
    assert_eq!(
        fresh.chest_row_potion_kinds[7], kind2,
        "the jar's word the every cell loads"
    );
    fresh.on_escape();
    // THE OLD FILE'S OWN SILENCE (the pre-364 file the tolerant parse
    // the honest NONE rows keep — the missing rows the absence banks,
    // never a guessed kind the absence invents)
    let old_file = dir.join("rowkinds-old.nbt");
    let _ = std::fs::remove_file(&old_file);
    let mut old_stores = std::collections::HashMap::new();
    let mut old_slots = vec![(0i32, 0i32); 27];
    old_slots[3] = (potion, 1);
    old_stores.insert(cell, old_slots);
    let old_root = mc_protocol::nbt::Nbt::Compound(vec![
        (
            "DataVersion".to_string(),
            mc_protocol::nbt::Nbt::Int(CONTAINERS_DATA_VERSION),
        ),
        (
            "entries".to_string(),
            container_bank_to_nbt(&old_stores, 27),
        ),
    ]);
    let mut old_bytes = Vec::new();
    mc_protocol::nbt::write_file_nbt("", &old_root, &mut old_bytes)
        .expect("the old file's write serves");
    std::fs::write(&old_file, &old_bytes).expect("the old file banks");
    let mut heir = App::new(364);
    heir.enter_world();
    heir.container_stores_file = Some(old_file.clone());
    heir.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    heir.open_chest_for(cell);
    assert_eq!(
        heir.chest_slots[3],
        (potion, 1),
        "the old file's stack the restart wears"
    );
    assert_eq!(
        heir.chest_row_potion_kinds[3], none,
        "the old file the honest NONE rows keep"
    );
    let _ = std::fs::remove_file(&file);
    let _ = std::fs::remove_file(&old_file);
}

/// THE CHEST MIRROR'S OWN KIND LANE (the autopush-365 round — the 356
/// tooltip law the mirror's render lane re-wears: the player rows the
/// REAL inventory the kind bank answers, the mirror map the seat
/// re-keys [the main row the items 9..36 the +9 speaks, the hotbar row
/// the items 0..9 the -27 seat keeps], the kinded row the liquid
/// overlay colors, the non-potion id the stale word never paints, the
/// wordless row the NONE draw the same pixels the plain icon wears —
/// the golden scenes' wordless rows the bytes never move).
#[test]
fn the_chest_mirror_the_kind_lane_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(365);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    // THE OPEN (the mirror rows the real inventory the sync wears)
    app.inventory.items[9] = (potion, 1); // the mirror row 0 the main grid wears
    app.items_potion_kinds[9] = kind;
    app.inventory.items[0] = (potion, 1); // the mirror row 27 the hotbar wears
    app.items_potion_kinds[0] = kind;
    app.open_chest_for(cell);
    assert_eq!(
        app.chest_mirror[0],
        (potion, 1),
        "the main row the mirror syncs"
    );
    assert_eq!(
        app.chest_mirror[27],
        (potion, 1),
        "the hotbar row the mirror syncs"
    );
    // THE GEOMETRY (the render the panel banks — the mirror cell the
    // +1 inset the icons wear)
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let assets_root = assets_root();
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.render(&mut fb_probe, &mut a);
    }
    let (px, py) = app.chest_panel;
    let (mx, my) = chest_mirror_slot((px, py), 0);
    let (hx, hy) = chest_mirror_slot((px, py), 27);
    // THE MIRROR'S OWN TINT (the kinded words the liquid overlay
    // colors, the wordless rows the NONE draw the same pixels keep —
    // the only delta the two mirror cells wear)
    let mut shot = |word: i32| -> crate::raster::Framebuf {
        app.items_potion_kinds[9] = word;
        app.items_potion_kinds[0] = word;
        let mut fb = crate::raster::Framebuf::new(320, 180);
        {
            let mut a = GuiAssets::open(&assets_root);
            app.render(&mut fb, &mut a);
        }
        fb
    };
    let fb_a = shot(kind);
    let fb_b = shot(none);
    let mut diff = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_a, mx + 1 + gx, my + 1 + gy) != pixel(&fb_b, mx + 1 + gx, my + 1 + gy) {
                diff += 1;
            }
        }
    }
    assert!(diff > 0, "the kinded mirror row the tint lane re-colors");
    let mut hot_diff = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_a, hx + 1 + gx, hy + 1 + gy) != pixel(&fb_b, hx + 1 + gx, hy + 1 + gy) {
                hot_diff += 1;
            }
        }
    }
    assert!(
        hot_diff > 0,
        "the hotbar mirror row the tint lane re-colors"
    );
    // THE ID GATE'S OWN WALL (the stale word the non-potion row never
    // paints — the dirt seat the same pixels the plain draw keeps)
    app.inventory.items[9] = (dirt, 1);
    app.items_potion_kinds[9] = kind; // the stale word the id gate retires
    app.chest_mirror_sync_from_inventory();
    let mut fb_c = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.render(&mut fb_c, &mut a);
    }
    app.items_potion_kinds[9] = none;
    app.chest_mirror_sync_from_inventory();
    let mut fb_d = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.render(&mut fb_d, &mut a);
    }
    let mut gate = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_c, mx + 1 + gx, my + 1 + gy) != pixel(&fb_d, mx + 1 + gx, my + 1 + gy) {
                gate += 1;
            }
        }
    }
    assert_eq!(gate, 0, "the stale word the dirt row never paints");
    // THE FAR FACE (the tint the panel's own chrome never wears)
    assert_eq!(
        pixel(&fb_a, 20, 160),
        pixel(&fb_b, 20, 160),
        "the far face the tint never wears"
    );
}

/// THE HOPPER'S OWN KIND LANE (the autopush-366 round — the 340/365
/// laws the five-seat canvas re-wears: the container seats the row
/// bank's word the liquid lane colors [the open door the
/// chest_row_potion_kinds loads, the click walk the words banks], the
/// mirror rows the real inventory the 365 map re-keys, the id gate
/// the stale word never paints, the wordless rows the NONE draw the
/// same pixels the plain icon wears — the golden scenes' wordless
/// rows the bytes never move; the tooltip's own five-seat ride the
/// same bank the hover rows re-colors).
#[test]
fn the_hopper_the_kind_lane_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(366);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::HOPPER);
    // THE DOOR (the five-seat store the offline open inserts — the
    // HopperMenu's own withSize(5) the width answers)
    app.open_hopper_for(cell);
    assert_eq!(app.screen, ScreenKind::Chest, "the hopper door opens");
    assert!(app.chest_window_hopper, "the hopper flag the canvas wears");
    assert_eq!(app.chest_slots.len(), 5, "the five-seat row opens");
    assert!(
        app.chest_row_potion_kinds.iter().all(|&k| k == none),
        "the fresh open the rows the NONE bank wears"
    );
    // THE SEAT'S OWN BANK + THE MIRROR'S OWN WORD (the kinded pair the
    // lanes answer)
    app.chest_slots[0] = (potion, 1);
    app.chest_row_potion_kinds[0] = kind;
    app.inventory.items[9] = (potion, 1); // the mirror row 0 the main grid wears
    app.items_potion_kinds[9] = kind;
    app.chest_mirror_sync_from_inventory();
    // THE GEOMETRY (the render the rects bank — the +1 inset the icons wear)
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let assets_root = assets_root();
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.render(&mut fb_probe, &mut a);
    }
    let (px, py) = app.chest_panel;
    let (sx, sy) = hopper_container_slot((px, py), 0);
    let (mx, my) = hopper_mirror_slot((px, py), 0);
    // THE KIND LANE'S OWN PAIR (the kinded words the liquid overlay
    // colors, the wordless words the NONE draw keeps — the deltas the
    // two cells wear; the mut-closure lesson the app-borrowing helper
    // the seat id's own arm the between-calls mutation never trips)
    let mut shot = |seat_id: i32, seat_word: i32, mirror_word: i32| -> crate::raster::Framebuf {
        app.chest_slots[0] = (seat_id, 1);
        app.chest_row_potion_kinds[0] = seat_word;
        app.items_potion_kinds[9] = mirror_word;
        let mut fb = crate::raster::Framebuf::new(320, 180);
        {
            let mut a = GuiAssets::open(&assets_root);
            app.render(&mut fb, &mut a);
        }
        fb
    };
    let fb_a = shot(potion, kind, kind);
    let fb_b = shot(potion, none, none);
    let mut seat_diff = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_a, sx + 1 + gx, sy + 1 + gy) != pixel(&fb_b, sx + 1 + gx, sy + 1 + gy) {
                seat_diff += 1;
            }
        }
    }
    assert!(seat_diff > 0, "the kinded seat the tint lane re-colors");
    let mut mirror_diff = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_a, mx + 1 + gx, my + 1 + gy) != pixel(&fb_b, mx + 1 + gx, my + 1 + gy) {
                mirror_diff += 1;
            }
        }
    }
    assert!(
        mirror_diff > 0,
        "the kinded mirror row the tint lane re-colors"
    );
    // THE ID GATE'S OWN WALL (the stale word the non-potion seat never
    // paints — the dirt pair the same pixels keeps; the seat's own id
    // the pair's arm swaps)
    let fb_c = shot(dirt, kind, none);
    let fb_d = shot(dirt, none, none);
    let mut gate = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_c, sx + 1 + gx, sy + 1 + gy) != pixel(&fb_d, sx + 1 + gx, sy + 1 + gy) {
                gate += 1;
            }
        }
    }
    assert_eq!(gate, 0, "the stale word the dirt seat never paints");
    // THE FAR FACE (the tint the panel's own chrome never wears)
    assert_eq!(
        pixel(&fb_a, 20, 160),
        pixel(&fb_b, 20, 160),
        "the far face the tint never wears"
    );
    // THE TOOLTIP'S OWN FIVE-SEAT RIDE (the 356 law the hopper's own
    // hover re-wears — the seat's banked word the name row re-colors,
    // the effect rows the walk appends, the wordless seat the plain
    // name keeps; the carried wall the None verdict the fresh cursor
    // never trips)
    app.chest_slots[0] = (potion, 1);
    app.chest_row_potion_kinds[0] = kind;
    let rows_a = app
        .hopper_item_tooltip_hover(sx + 8, sy + 8)
        .expect("the hovered seat the rows speak");
    assert_eq!(
        rows_a[0].0, "Potion of Night Vision",
        "the banked word the name row wears"
    );
    assert!(rows_a.len() >= 2, "the effect row the kinded walk appends");
    app.chest_row_potion_kinds[0] = none;
    let rows_b = app
        .hopper_item_tooltip_hover(sx + 8, sy + 8)
        .expect("the hovered seat the rows speak");
    assert_eq!(
        rows_b[0].0, "Potion",
        "the wordless seat the plain name wears"
    );
}

/// THE TABLE'S OWN STORE TINT (the dev-205 round — the 340 law the
/// table's own store rows re-wear: the rows the REAL inventory the
/// kind bank answers, the id gate the stale seat never paints, the
/// wordless row the NONE draw the same pixels the plain icon wears —
/// the golden scenes' wordless rows the bytes never move; the hotbar
/// tail the same law answers).
#[test]
fn the_table_store_the_tint_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    // THE DOOR (the table's own canvas the offline walk opens)
    app.open_crafting_table();
    assert_eq!(
        app.screen,
        ScreenKind::Crafting,
        "the table door the walk wears"
    );
    // THE REAL PANEL THE RENDER BANKS (the 355 harness the door wears
    // — the headless mouse-less frame the panel lays)
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_probe, &mut a);
    }
    let panel = app.table_panel;
    let (mx, my) = inventory_slot_origin(panel, 12);
    let (hx, hy) = inventory_slot_origin(panel, 3);
    // THE KIND LANE'S OWN PAIR (the kinded words the liquid overlay
    // colors, the wordless words the NONE draw keeps — the mut-closure
    // lesson the app-borrowing helper the seat ids the arms wear)
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let mut shot =
        |main_id: i32, main_word: i32, hot_id: i32, hot_word: i32| -> crate::raster::Framebuf {
            app.inventory.items[12] = (main_id, 1);
            app.items_potion_kinds[12] = main_word;
            app.inventory.items[3] = (hot_id, 1);
            app.items_potion_kinds[3] = hot_word;
            let mut fb = crate::raster::Framebuf::new(320, 180);
            {
                let mut a = GuiAssets::open(&assets_root);
                app.mouse_pos = None;
                app.render(&mut fb, &mut a);
            }
            fb
        };
    let fb_a = shot(potion, kind, potion, kind);
    let fb_b = shot(potion, none, potion, none);
    let mut main_diff = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_a, mx + 1 + gx, my + 1 + gy) != pixel(&fb_b, mx + 1 + gx, my + 1 + gy) {
                main_diff += 1;
            }
        }
    }
    assert!(
        main_diff > 0,
        "the kinded store row the tint lane re-colors"
    );
    let mut hot_diff = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_a, hx + 1 + gx, hy + 1 + gy) != pixel(&fb_b, hx + 1 + gx, hy + 1 + gy) {
                hot_diff += 1;
            }
        }
    }
    assert!(hot_diff > 0, "the hotbar row the tint lane re-colors");
    // THE ID GATE'S OWN WALL (the stale word the non-potion row never
    // paints — the dirt pair the same pixels keeps)
    let fb_c = shot(dirt, kind, dirt, kind);
    let fb_d = shot(dirt, none, dirt, none);
    let mut gate = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_c, mx + 1 + gx, my + 1 + gy) != pixel(&fb_d, mx + 1 + gx, my + 1 + gy) {
                gate += 1;
            }
        }
    }
    assert_eq!(gate, 0, "the stale word the dirt row never paints");
    // THE FAR FACE (the tint the panel's own chrome never wears)
    assert_eq!(
        pixel(&fb_a, 20, 160),
        pixel(&fb_b, 20, 160),
        "the far face the tint never wears"
    );
}

/// THE TABLE GRID'S OWN KIND BANK (the autopush-367 round — the
/// dev-204 law the 3x3 cells re-wear: the place the word the cell
/// lays, the pickup the cell's kind the word banks, the model's own
/// swap the pairs exchange, the normalized read the stale word the
/// non-potion cells never keeps; the render the grid's own bank the
/// liquid lane colors, the close the words the receiving seats ride
/// home to [the carried's own return + the clearContainer's own grid
/// drain], the reopen the fresh NONE row the door banks).
#[test]
fn the_table_grid_the_word_bank_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let kind2 = 5;
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    // THE DOOR (the fresh open the NONE row banks)
    app.open_crafting_table();
    assert_eq!(app.screen, ScreenKind::Crafting, "the table door opens");
    assert!(
        app.table_grid_potion_kinds.iter().all(|&k| k == none),
        "the fresh open the grid the NONE bank wears"
    );
    // THE REAL PANEL THE RENDER BANKS (the 355 harness the door wears)
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_probe, &mut a);
    }
    let panel = app.table_panel;
    let cell_seat = |cell: usize| -> (i32, i32) {
        let (x, y) = table_craft_origin(panel, cell);
        (x + 8, y + 8)
    };
    // THE PLACE ARM (the kinded carried the cell's word lays — the
    // carried the NONE word wears)
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = kind;
    let (cx, cy) = cell_seat(0);
    app.click_table_cell(0, 0);
    assert_eq!(app.table_grid[0], (potion, 1), "the place the cell lays");
    assert_eq!(
        app.table_grid_potion_kinds[0], kind,
        "the placed cell the word banks"
    );
    assert_eq!(app.inv_carried, (0, 0), "the place the carried empties");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the placed carried the NONE word wears"
    );
    // THE RENDER'S OWN GRID TINT (the cell's word the liquid lane
    // colors — the headless pair the only delta the cell's own icon
    // wears, the far face the tint never wears)
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let mut shot = |word: i32| -> crate::raster::Framebuf {
        app.table_grid_potion_kinds[0] = word;
        let mut fb = crate::raster::Framebuf::new(320, 180);
        {
            let mut a = GuiAssets::open(&assets_root);
            app.mouse_pos = None;
            app.render(&mut fb, &mut a);
        }
        fb
    };
    let fb_a = shot(kind);
    let fb_b = shot(none);
    let mut diff = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_a, cx - 7 + gx, cy - 7 + gy) != pixel(&fb_b, cx - 7 + gx, cy - 7 + gy) {
                diff += 1;
            }
        }
    }
    assert!(diff > 0, "the kinded cell the tint lane re-colors");
    assert_eq!(
        pixel(&fb_a, 20, 160),
        pixel(&fb_b, 20, 160),
        "the far face the tint never wears"
    );
    app.table_grid_potion_kinds[0] = kind; // the pair's own state restores
                                           // THE PICKUP ARM (the cell's kind the word banks, the emptied cell
                                           // the NONE word wears)
    app.click_table_cell(0, 0);
    assert_eq!(app.inv_carried, (potion, 1), "the pickup the carried banks");
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the kinded pickup the word banks"
    );
    assert_eq!(app.table_grid[0], (0, 0), "the pickup the cell empties");
    assert_eq!(
        app.table_grid_potion_kinds[0], none,
        "the emptied cell the NONE word wears"
    );
    // THE SWAP ARM (the dirt seat the potion pairs exchange — the kind
    // the pairs ride)
    app.table_grid[1] = (dirt, 1);
    app.table_grid_potion_kinds[1] = none;
    app.click_table_cell(1, 0);
    assert_eq!(app.table_grid[1], (potion, 1), "the swap the cell takes");
    assert_eq!(
        app.table_grid_potion_kinds[1], kind,
        "the swapped cell the word banks"
    );
    assert_eq!(app.inv_carried, (dirt, 1), "the swap the carried takes");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the swapped carried the NONE word wears"
    );
    // THE STALE WORD'S OWN WALL (the dirt seat the stale kind never
    // lends — the normalized read the non-potion cells the honest NONE
    // keeps: the empty-hand pickup the NONE word wears)
    app.throw_carried_outside(0); // the carried's own reset the stale arm wears
    app.table_grid[2] = (dirt, 1);
    app.table_grid_potion_kinds[2] = kind; // the stale word the bank keeps
    app.click_table_cell(2, 0);
    assert_eq!(app.inv_carried, (dirt, 1), "the pickup the carried banks");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the stale word the dirt cell never lends"
    );
    app.throw_carried_outside(0); // the carried's own reset the close wears
                                  // THE SWAP'S OWN POTION HOME (the cell 1 the same pickup the word
                                  // lends — the throw the ride retires, the close the cell 3's own
                                  // potion alone drains)
    app.click_table_cell(1, 0);
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the swap's own potion the word lends"
    );
    app.throw_carried_outside(0);
    // THE CLOSE'S OWN GRID RIDE (the kinded cell the first-empty seat
    // the word banks — the drain the grid empties, the bank the fresh
    // NONE row wears)
    app.table_grid[3] = (potion, 1);
    app.table_grid_potion_kinds[3] = kind;
    app.close_crafting_table(ScreenKind::World);
    assert_eq!(app.screen, ScreenKind::World, "the close the world returns");
    assert_eq!(app.table_grid[3], (0, 0), "the close the grid drains");
    assert!(
        app.table_grid_potion_kinds.iter().all(|&k| k == none),
        "the close the bank the fresh NONE row wears"
    );
    let landing = app
        .inventory
        .items
        .iter()
        .position(|it| it.0 == potion && it.1 > 0)
        .expect("the potion the inventory lands");
    assert_eq!(
        app.items_potion_kinds[landing], kind,
        "the landed potion the word banks"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the close the carried word retires"
    );
    // THE CLOSE'S OWN CARRIED RIDE (the kinded carried the receiving
    // free row the word banks — the 361 law the table's own close
    // re-wears; the two rides the kinded pair the seats speak)
    app.open_crafting_table();
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = kind2;
    app.close_crafting_table(ScreenKind::World);
    let landed_kinds: Vec<i32> = (0..36)
        .filter(|&i| app.inventory.items[i] == (potion, 1))
        .map(|i| app.items_potion_kinds[i])
        .collect();
    assert_eq!(landed_kinds.len(), 2, "the two rides the pair lands");
    assert!(landed_kinds.contains(&kind), "the grid ride the word banks");
    assert!(
        landed_kinds.contains(&kind2),
        "the carried ride the word banks"
    );
}

/// THE INVENTORY CRAFT CELLS' OWN KIND BANK (the autopush-368 round —
/// the 367 law the 2x2 cells re-wear: the fresh door the constructor's
/// own NONE row banks, the place the word the cell lays, the render's
/// own tint the kinded cell re-colors, the pickup the word rides home,
/// the swap the pairs exchange, the stale silence the non-potion cell
/// never lends — the PERSISTENCE'S OWN LAW the close the grid keeps,
/// the transient table the drain wore, the persistent menu the words
/// the same ride never drains, the reopen the kinded cell re-wears).
#[test]
fn the_inventory_craft_cells_the_word_bank_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    // THE DOOR (the constructor's own NONE row the fresh open wears)
    app.open_inventory();
    assert_eq!(
        app.screen,
        ScreenKind::Inventory,
        "the inventory door opens"
    );
    assert!(
        app.craft_grid_potion_kinds.iter().all(|&k| k == none),
        "the fresh door the cells the NONE bank wears"
    );
    // THE REAL PANEL THE RENDER BANKS (the headless render the panel
    // origin lays — the craft seats the same origin the click lane
    // hits)
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_probe, &mut a);
    }
    let panel = app.inv_panel;
    let cell_seat = |cell: usize| -> (i32, i32) {
        let (x, y) = craft_slot_origin(panel, cell);
        (x + 8, y + 8)
    };
    // THE PLACE ARM (the kinded carried the cell's word lays — the
    // carried the NONE word wears)
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = kind;
    let (cx, cy) = cell_seat(0);
    app.click_craft_cell(0, 0);
    assert_eq!(app.craft_grid[0], (potion, 1), "the place the cell lays");
    assert_eq!(
        app.craft_grid_potion_kinds[0], kind,
        "the placed cell the word banks"
    );
    assert_eq!(app.inv_carried, (0, 0), "the place the carried empties");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the placed carried the NONE word wears"
    );
    // THE RENDER'S OWN TINT (the cell's word the liquid lane colors —
    // the headless pair the only delta the cell's own icon wears, the
    // far face the tint never wears)
    let pixel = |fb: &crate::raster::Framebuf, x: i32, y: i32| -> u8 {
        fb.color[((y as usize) * fb.w + x as usize) * 3]
    };
    let mut shot = |word: i32| -> crate::raster::Framebuf {
        app.craft_grid_potion_kinds[0] = word;
        let mut fb = crate::raster::Framebuf::new(320, 180);
        {
            let mut a = GuiAssets::open(&assets_root);
            app.mouse_pos = None;
            app.render(&mut fb, &mut a);
        }
        fb
    };
    let fb_a = shot(kind);
    let fb_b = shot(none);
    let mut diff = 0usize;
    for gy in 0..16i32 {
        for gx in 0..16i32 {
            if pixel(&fb_a, cx - 7 + gx, cy - 7 + gy) != pixel(&fb_b, cx - 7 + gx, cy - 7 + gy) {
                diff += 1;
            }
        }
    }
    assert!(diff > 0, "the kinded cell the tint lane re-colors");
    assert_eq!(
        pixel(&fb_a, 20, 160),
        pixel(&fb_b, 20, 160),
        "the far face the tint never wears"
    );
    app.craft_grid_potion_kinds[0] = kind; // the pair's own state restores
                                           // THE PICKUP ARM (the cell's kind the word banks, the emptied
                                           // cell the NONE word wears)
    app.click_craft_cell(0, 0);
    assert_eq!(app.inv_carried, (potion, 1), "the pickup the carried banks");
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the kinded pickup the word banks"
    );
    assert_eq!(app.craft_grid[0], (0, 0), "the pickup the cell empties");
    assert_eq!(
        app.craft_grid_potion_kinds[0], none,
        "the emptied cell the NONE word wears"
    );
    // THE SWAP ARM (the dirt seat the potion pairs exchange — the kind
    // the pairs ride)
    app.craft_grid[1] = (dirt, 1);
    app.craft_grid_potion_kinds[1] = none;
    app.click_craft_cell(1, 0);
    assert_eq!(app.craft_grid[1], (potion, 1), "the swap the cell takes");
    assert_eq!(
        app.craft_grid_potion_kinds[1], kind,
        "the swapped cell the word banks"
    );
    assert_eq!(app.inv_carried, (dirt, 1), "the swap the carried takes");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the swapped carried the NONE word wears"
    );
    app.throw_carried_outside(0); // the carried's own reset the stale arm wears
                                  // THE STALE WORD'S OWN WALL (the dirt seat the stale kind never
                                  // lends — the normalized read the non-potion cells the honest
                                  // NONE keeps: the empty-hand pickup the NONE word wears)
    app.craft_grid[2] = (dirt, 1);
    app.craft_grid_potion_kinds[2] = kind; // the stale word the bank keeps
    app.click_craft_cell(2, 0);
    assert_eq!(app.inv_carried, (dirt, 1), "the pickup the carried banks");
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the stale word the dirt cell never lends"
    );
    app.throw_carried_outside(0); // the carried's own reset the persistence wears
                                  // THE PERSISTENCE'S OWN LAW (the close the grid keeps, the words
                                  // the same ride never drains — the transient table the drain
                                  // wore, the persistent menu the kinded cell the reopen re-wears)
    app.close_inventory(ScreenKind::World);
    assert_eq!(app.screen, ScreenKind::World, "the close the world returns");
    app.open_inventory();
    assert_eq!(
        app.screen,
        ScreenKind::Inventory,
        "the reopen the door wears"
    );
    assert_eq!(
        app.craft_grid[1],
        (potion, 1),
        "the persistent cell the close keeps"
    );
    assert_eq!(
        app.craft_grid_potion_kinds[1], kind,
        "the persistent cell the word the close keeps"
    );
    assert_eq!(app.craft_grid[2], (0, 0), "the picked cell the empty keeps");
}

/// THE CRAFT CELLS' OWN TOOLTIP RIDE (the autopush-369 round — the
/// 340 law the inventory's own hover re-wears: the 368 bank the
/// hovered craft cell's word the name row answers, the effect rows
/// the kinded walk appends, the id gate the stale seat never lends,
/// the wordless cell the plain name keeps, the carried wall the
/// tooltip refuses).
#[test]
fn the_inventory_craft_cells_the_tooltip_word_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_inventory();
    assert_eq!(
        app.screen,
        ScreenKind::Inventory,
        "the inventory door opens"
    );
    // THE REAL PANEL THE HOVER SEATS (the headless render the panel
    // origin lays — the craft seats the same origin the click lane
    // hits)
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_probe, &mut a);
    }
    let panel = app.inv_panel;
    let cell_seat = |cell: usize| -> (i32, i32) {
        let (x, y) = craft_slot_origin(panel, cell);
        (x + 8, y + 8)
    };
    // THE KINDED CELL'S OWN NAME ROW (the 368 bank the hovered cell's
    // word the display name answers — the effect rows the kinded walk
    // appends)
    app.craft_grid[0] = (potion, 1);
    app.craft_grid_potion_kinds[0] = kind;
    app.inv_carried = (0, 0);
    let (cx, cy) = cell_seat(0);
    let (_, rows_a) = app
        .item_tooltip_hover(cx, cy)
        .expect("the hovered craft cell the rows speak");
    assert_eq!(
        rows_a[0].0, "Potion of Night Vision",
        "the banked word the name row wears"
    );
    assert!(rows_a.len() >= 2, "the effect row the kinded walk appends");
    // THE WORDLESS CELL'S OWN PLAIN NAME (the NONE word the plain row
    // keeps — the same pixels the plain lane wears)
    app.craft_grid_potion_kinds[0] = none;
    let (_, rows_b) = app
        .item_tooltip_hover(cx, cy)
        .expect("the hovered craft cell the rows speak");
    assert_eq!(
        rows_b[0].0, "Potion",
        "the wordless cell the plain name wears"
    );
    // THE STALE WORD'S OWN WALL (the dirt cell the stale kind never
    // lends — the normalized read the non-potion cells the honest
    // plain name keeps)
    app.craft_grid[0] = (dirt, 1);
    app.craft_grid_potion_kinds[0] = kind;
    let (_, rows_c) = app
        .item_tooltip_hover(cx, cy)
        .expect("the hovered craft cell the rows speak");
    assert_eq!(
        rows_c[0].0, "Dirt",
        "the stale word the dirt cell never lends"
    );
    // THE CARRIED WALL (the carried stack the tooltip refuses — the
    // image-less false answer the gate never serves)
    app.craft_grid[0] = (potion, 1);
    app.craft_grid_potion_kinds[0] = kind;
    app.inv_carried = (dirt, 1);
    assert!(
        app.item_tooltip_hover(cx, cy).is_none(),
        "the carried wall the tooltip refuses"
    );
    app.inv_carried = (0, 0);
    // THE EMPTY CELL'S OWN SILENCE (the count gate the hasItem false
    // the tooltip refuses)
    let (ex, ey) = cell_seat(3);
    assert!(
        app.item_tooltip_hover(ex, ey).is_none(),
        "the empty cell the tooltip refuses"
    );
}

/// THE HOPPER'S OWN TRANSFER RIDE (the autopush-370 round — the 369
/// NEXT list's own third candidate the shift quick-move the hopper's
/// five seats the chest's own lane re-wears: the kinded pair the
/// both directions ride, the drained source the NONE retires, the
/// stale word the kindless walk never lends, the scan order the
/// vanilla's own shape wears [the grid source the hotbar-first
/// reverse ride, the player row the seat-0 forward ride]).
#[test]
fn the_hopper_the_transfer_ride_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(370);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::HOPPER);
    app.open_hopper_for(cell);
    assert_eq!(app.screen, ScreenKind::Chest, "the hopper door opens");
    // THE GEOMETRY (the render the rects bank — the seat + the mirror
    // rows the same panel origin lays)
    let assets_root = assets_root();
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.render(&mut fb_probe, &mut a);
    }
    let (px, py) = app.chest_panel;
    let (sx, sy) = hopper_container_slot((px, py), 0);
    let (mx, my) = hopper_mirror_slot((px, py), 0);
    let (s1x, s1y) = hopper_container_slot((px, py), 1);
    // RIDE A (the hopper seat the player rows ride — the hotbar-first
    // reverse scan the empty rows wear, the drained source the NONE
    // retires, the landing row the word banks)
    app.chest_slots[0] = (potion, 1);
    app.chest_row_potion_kinds[0] = kind;
    app.shift_down = true;
    app.click_chest_slot_offline(sx + 8, sy + 8, 0);
    assert_eq!(
        app.chest_slots[0],
        (0, 0),
        "the drained source the empty keeps"
    );
    assert_eq!(
        app.chest_row_potion_kinds[0], none,
        "the drained source the NONE retires"
    );
    assert_eq!(
        app.inventory.items[8],
        (potion, 1),
        "the hotbar-first scan the tail row lands"
    );
    assert_eq!(
        app.items_potion_kinds[8], kind,
        "the landing row the word banks"
    );
    app.shift_down = false;
    app.inventory.items[8] = (0, 0); // the ride's own surface the round clears
    app.items_potion_kinds[8] = none;
    // RIDE B (the player row the hopper seat rides back — the mirror
    // row's own word the seat banks, the drained row the NONE wears)
    app.inventory.items[9] = (potion, 1);
    app.items_potion_kinds[9] = kind;
    app.shift_down = true;
    app.click_chest_slot_offline(mx + 8, my + 8, 0);
    assert_eq!(
        app.inventory.items[9],
        (0, 0),
        "the drained row the empty keeps"
    );
    assert_eq!(
        app.items_potion_kinds[9], none,
        "the drained row the NONE retires"
    );
    assert_eq!(
        app.chest_slots[0],
        (potion, 1),
        "the forward scan the seat-0 lands"
    );
    assert_eq!(
        app.chest_row_potion_kinds[0], kind,
        "the landing seat the word banks"
    );
    app.shift_down = false;
    // RIDE C (the stale silence — the dirt seat the stale word never
    // lends: the kindless ride the honest NONE keeps, the drained
    // seat the stale word the drain retires)
    app.chest_slots[1] = (dirt, 1);
    app.chest_row_potion_kinds[1] = kind; // the stale word the bank keeps
    app.shift_down = true;
    app.click_chest_slot_offline(s1x + 8, s1y + 8, 0);
    assert_eq!(
        app.inventory.items[8],
        (dirt, 1),
        "the kindless ride the row lands"
    );
    assert_eq!(
        app.items_potion_kinds[8], none,
        "the stale word the ride never lends"
    );
    assert_eq!(
        app.chest_row_potion_kinds[1], none,
        "the drained seat the word retires"
    );
    app.shift_down = false;
}

/// THE CRAFT CELLS' OWN BANK DRAG (the dev-206 round — the 370 NEXT
/// list's own third candidate the 368 bank the drag's own placement
/// banks: the empty seat the cursor's word wears, the occupied seat
/// the merge the stack's own word keeps, the remainder the cursor's
/// word keeps, the type-1's own one-each ride the same bank wears).
#[test]
fn the_inventory_craft_cells_the_drag_word_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(206);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_inventory();
    assert_eq!(
        app.screen,
        ScreenKind::Inventory,
        "the inventory door opens"
    );
    // THE REAL PANEL THE DRAG SEATS (the headless render the panel
    // origin lays — the craft seats the same origin the click lane
    // hits)
    let assets_root = assets_root();
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_probe, &mut a);
    }
    let panel = app.inv_panel;
    let cell_seat = |cell: usize| -> (i32, i32) {
        let (x, y) = craft_slot_origin(panel, cell);
        (x + 8, y + 8)
    };
    let (cx0, cy0) = cell_seat(0);
    let (cx3, cy3) = cell_seat(3);
    // THE EMPTY SEATS' OWN WORD (the type-0 drag the cursor's word
    // wears — the two painted cells the kinded pair bank, the
    // remainder the cursor keeps)
    app.inv_carried = (potion, 3);
    app.inv_carried_potion_kind = kind;
    app.qc_arm_inventory(cx0, cy0, 0);
    app.qc_paint(cx0, cy0);
    app.qc_paint(cx3, cy3);
    app.qc_release(0);
    assert_eq!(app.craft_grid[0], (potion, 1), "the drag the cell lays");
    assert_eq!(
        app.craft_grid_potion_kinds[0], kind,
        "the placed cell the word banks"
    );
    assert_eq!(app.craft_grid[3], (potion, 1), "the second seat lays");
    assert_eq!(
        app.craft_grid_potion_kinds[3], kind,
        "the second seat the word banks"
    );
    assert_eq!(
        app.inv_carried,
        (potion, 1),
        "the remainder the cursor keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, kind,
        "the remainder the word keeps"
    );
    // THE OCCUPIED SEAT'S OWN WORD (the merge the stack's own
    // components keep — the kindless seat the cursor's word never
    // lends, the max-1 cap the net the carried never loses)
    app.craft_grid_potion_kinds[0] = none; // the kindless seat the scenario lays
    app.qc_arm_inventory(cx0, cy0, 0);
    app.qc_paint(cx0, cy0);
    app.qc_release(0);
    assert_eq!(app.craft_grid[0], (potion, 1), "the merge the count keeps");
    assert_eq!(
        app.craft_grid_potion_kinds[0], none,
        "the seat's own word the merge keeps"
    );
    assert_eq!(
        app.inv_carried,
        (potion, 1),
        "the max-1 merge the carried keeps"
    );
    // THE TYPE-1'S OWN RIDE (the right drag the one-each answer — the
    // empty cell the cursor's word the same lane banks)
    app.craft_grid[3] = (0, 0); // the empty seat the type-1 lays
    app.craft_grid_potion_kinds[3] = none;
    app.inv_carried = (potion, 3);
    app.inv_carried_potion_kind = kind;
    app.qc_arm_inventory(cx0, cy0, 1);
    app.qc_paint(cx0, cy0);
    app.qc_paint(cx3, cy3);
    app.qc_release(1);
    assert_eq!(
        app.craft_grid[0],
        (potion, 1),
        "the max-1 merge the count keeps"
    );
    assert_eq!(app.craft_grid[3], (potion, 1), "the type-1 the seat lays");
    assert_eq!(
        app.craft_grid_potion_kinds[3], kind,
        "the type-1 seat the word banks"
    );
    assert_eq!(
        app.craft_grid_potion_kinds[0], none,
        "the occupied seat the own word keeps"
    );
}

/// THE TABLE GRID'S OWN BANK DRAG (the autopush-371 round — the
/// dev-206 law the 3x3 cells re-wear: the 367 bank the drag's own
/// placement banks, the empty cell the cursor's word wears, the
/// occupied cell the merge the stack's own word keeps, the type-1's
/// own one-each ride the same bank wears).
#[test]
fn the_table_grid_the_drag_word_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let kind = 4; // the night_vision row the POTION_KIND_NAMES banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(371);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_crafting_table();
    assert_eq!(app.screen, ScreenKind::Crafting, "the table door opens");
    // THE REAL PANEL THE DRAG SEATS (the headless render the panel
    // origin lays — the table cells the same origin the click lane
    // hits)
    let assets_root = assets_root();
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_probe, &mut a);
    }
    let panel = app.table_panel;
    let cell_seat = |cell: usize| -> (i32, i32) {
        let (x, y) = table_craft_origin(panel, cell);
        (x + 8, y + 8)
    };
    let (cx0, cy0) = cell_seat(0);
    let (cx8, cy8) = cell_seat(8);
    // THE EMPTY CELLS' OWN WORD (the type-0 drag the cursor's word
    // wears — the two painted cells the kinded pair bank, the
    // remainder the cursor keeps)
    app.inv_carried = (potion, 3);
    app.inv_carried_potion_kind = kind;
    app.qc_arm_table(cx0, cy0, 0);
    app.qc_paint_table(cx0, cy0);
    app.qc_paint_table(cx8, cy8);
    app.qc_release(0);
    assert_eq!(app.table_grid[0], (potion, 1), "the drag the cell lays");
    assert_eq!(
        app.table_grid_potion_kinds[0], kind,
        "the placed cell the word banks"
    );
    assert_eq!(app.table_grid[8], (potion, 1), "the second cell lays");
    assert_eq!(
        app.table_grid_potion_kinds[8], kind,
        "the second cell the word banks"
    );
    assert_eq!(
        app.inv_carried,
        (potion, 1),
        "the remainder the cursor keeps"
    );
    // THE OCCUPIED CELL'S OWN WORD (the merge the stack's own
    // components keep — the kindless cell the cursor's word never
    // lends, the max-1 cap the net the carried never loses)
    app.table_grid_potion_kinds[0] = none;
    app.qc_arm_table(cx0, cy0, 0);
    app.qc_paint_table(cx0, cy0);
    app.qc_release(0);
    assert_eq!(app.table_grid[0], (potion, 1), "the merge the count keeps");
    assert_eq!(
        app.table_grid_potion_kinds[0], none,
        "the cell's own word the merge keeps"
    );
    // THE TYPE-1'S OWN RIDE (the right drag the one-each answer —
    // the emptied cell the cursor's word the same lane banks)
    app.table_grid[8] = (0, 0);
    app.table_grid_potion_kinds[8] = none;
    app.inv_carried = (potion, 3);
    app.inv_carried_potion_kind = kind;
    app.qc_arm_table(cx0, cy0, 1);
    app.qc_paint_table(cx0, cy0);
    app.qc_paint_table(cx8, cy8);
    app.qc_release(1);
    assert_eq!(
        app.table_grid[0],
        (potion, 1),
        "the max-1 merge the count keeps"
    );
    assert_eq!(app.table_grid[8], (potion, 1), "the type-1 the cell lays");
    assert_eq!(
        app.table_grid_potion_kinds[8], kind,
        "the type-1 cell the word banks"
    );
    assert_eq!(
        app.table_grid_potion_kinds[0], none,
        "the occupied cell the own word keeps"
    );
}

/// THE SAME-ID KIND PAIR'S OWN MODEL RIDE (the autopush-372 round —
/// the vanilla `isSameItemSameComponents` the plain (id, count) model
/// never saw: the same-id DIFFERENT-KIND potion pair the chest's own
/// pickup the vanilla SWAP answers, the pairs the numerically
/// identical stacks keep, the KINDS the exchange the honest verdict
/// wears, the same-kind pair the max-1 gate the honest silence keeps,
/// the wordless pair the component difference the same swap wears,
/// the right press the never-swap law the silence keeps, the
/// non-potion ids the plain id law the merge keeps).
#[test]
fn the_same_id_kind_pair_the_model_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    // THE MODEL'S OWN ANSWER (the pure walk the lane rides — the
    // non-potion pair the plain law the merge keeps, the different
    // ids the vanilla swap the base walk wears)
    let (s, c) = crate::world_view::predict_pickup_kind((dirt, 3), (dirt, 2), none, none);
    assert_eq!(s, (dirt, 5), "the non-potion pair the merge keeps");
    assert_eq!(c, (0, 0), "the merge the carried empties");
    let (s, c) = crate::world_view::predict_pickup_kind((dirt, 3), (stone, 2), none, none);
    assert_eq!(s, (stone, 2), "the different ids the swap wears");
    assert_eq!(c, (dirt, 3), "the swap the carried exchanges");
    let mut app = App::new(372);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    app.open_chest_for(cell);
    assert_eq!(app.screen, ScreenKind::Chest, "the chest door opens");
    assert!(!app.chest_is_live, "the offline sim the door keeps");
    // THE REAL RECTS THE RENDER BANKS (the headless frame the seat
    // lookups the same walk the clicks share)
    let assets_root = assets_root();
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_probe, &mut a);
    }
    let seat_of = |rects: &[(i32, i32, i16)], slot: i16| -> (i32, i32) {
        rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    let (gx, gy) = seat_of(&app.chest_slot_rects, 0);
    let (gx1, gy1) = seat_of(&app.chest_slot_rects, 1);
    // ARM A — THE DIFFERENT-KIND PAIR'S OWN SWAP (the vanilla law the
    // plain id model never wore — the pairs the identical stacks
    // keep, the KINDS the exchange the honest verdict wears)
    app.chest_slots[0] = (potion, 1);
    app.chest_row_potion_kinds[0] = night;
    app.chest_carried = (potion, 1);
    app.chest_carried_potion_kind = swift;
    app.click_chest_slot_offline(gx + 8, gy + 8, 0);
    assert_eq!(app.chest_slots[0], (potion, 1), "the seat the stack keeps");
    assert_eq!(
        app.chest_carried,
        (potion, 1),
        "the carried the stack keeps"
    );
    assert_eq!(
        app.chest_row_potion_kinds[0], swift,
        "the carried's own word the seat wears"
    );
    assert_eq!(
        app.chest_carried_potion_kind, night,
        "the seat's own word the carried wears"
    );
    // ARM B — THE SAME-KIND PAIR'S OWN SILENCE (the same components
    // the same item speaks, the max-1 gate the merge closes, the
    // kinds the own words keep)
    app.chest_row_potion_kinds[0] = night;
    app.chest_carried_potion_kind = night;
    app.click_chest_slot_offline(gx + 8, gy + 8, 0);
    assert_eq!(
        app.chest_row_potion_kinds[0], night,
        "the same-kind pair the seat word keeps"
    );
    assert_eq!(
        app.chest_carried_potion_kind, night,
        "the same-kind pair the carried word keeps"
    );
    // ARM C — THE WORDLESS PAIR'S OWN LAW (the contents-less stack
    // the kinded potion the different components the swap answers —
    // the NONE word the kinded word the seats exchange)
    app.chest_row_potion_kinds[0] = none;
    app.chest_carried_potion_kind = swift;
    app.click_chest_slot_offline(gx + 8, gy + 8, 0);
    assert_eq!(
        app.chest_row_potion_kinds[0], swift,
        "the kinded word the seat wears"
    );
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the wordless stack the carried wears"
    );
    // ARM D — THE RIGHT PRESS'S OWN SILENCE (the never-swap law the
    // component pair the same no-op keeps — the kinds the own words
    // keep, the button gate the kind exchange never lends)
    app.chest_row_potion_kinds[0] = night;
    app.chest_carried_potion_kind = swift;
    app.click_chest_slot_offline(gx + 8, gy + 8, 1);
    assert_eq!(
        app.chest_row_potion_kinds[0], night,
        "the right press the seat word keeps"
    );
    assert_eq!(
        app.chest_carried_potion_kind, swift,
        "the right press the carried word keeps"
    );
    // ARM E — THE NON-POTION PAIR'S OWN PLAIN LAW (the dirt rows the
    // merge the plain id law wears, the kinds the NONE words keep,
    // the emptied carried the honest NONE retires)
    app.chest_slots[1] = (dirt, 3);
    app.chest_row_potion_kinds[1] = none;
    app.chest_carried = (dirt, 2);
    app.chest_carried_potion_kind = none;
    app.click_chest_slot_offline(gx1 + 8, gy1 + 8, 0);
    assert_eq!(
        app.chest_slots[1],
        (dirt, 5),
        "the dirt pair the merge wears"
    );
    assert_eq!(app.chest_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.chest_row_potion_kinds[1], none,
        "the dirt row the word keeps"
    );
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the emptied carried the NONE wears"
    );
}

/// THE INVENTORY STORE ROWS' OWN MODEL RIDE (the autopush-373 round —
/// the 372 NEXT list's own family walk the store rows re-wear: the
/// vanilla `isSameItemSameComponents` law the store rows' plain (id,
/// count) model never saw: the same-id DIFFERENT-KIND potion pair the
/// store row's own pickup the vanilla SWAP answers, the same-kind
/// pair the max-1 gate the honest silence keeps, the wordless pair
/// the component difference the same swap wears, the right press the
/// never-swap law the silence keeps, the non-potion ids the plain id
/// law the merge keeps).
#[test]
fn the_store_rows_the_model_ride_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(373);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_inventory();
    assert_eq!(
        app.screen,
        ScreenKind::Inventory,
        "the inventory door opens"
    );
    // THE REAL PANEL THE RENDER BANKS (the headless frame the seat
    // lookups the same walk the clicks share)
    let assets_root = assets_root();
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_probe, &mut a);
    }
    let panel = app.inv_panel;
    let seat_of = |store: usize| -> (i32, i32) {
        let (x, y) = inventory_slot_origin(panel, store);
        (x + 8, y + 8)
    };
    // ARM A — THE DIFFERENT-KIND PAIR'S OWN SWAP (the vanilla law the
    // plain id model never wore — the pairs the identical stacks
    // keep, the KINDS the exchange the honest verdict wears)
    app.inventory.items[9] = (potion, 1);
    app.items_potion_kinds[9] = night;
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = swift;
    let (sx, sy) = seat_of(9);
    app.click_inventory_slot(sx, sy, 0);
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the seat the stack keeps"
    );
    assert_eq!(app.inv_carried, (potion, 1), "the carried the stack keeps");
    assert_eq!(
        app.items_potion_kinds[9], swift,
        "the carried's own word the seat wears"
    );
    assert_eq!(
        app.inv_carried_potion_kind, night,
        "the seat's own word the carried wears"
    );
    // ARM B — THE SAME-KIND PAIR'S OWN SILENCE (the same components
    // the same item speaks, the max-1 gate the merge closes, the
    // kinds the own words keep)
    app.items_potion_kinds[9] = night;
    app.inv_carried_potion_kind = night;
    app.click_inventory_slot(sx, sy, 0);
    assert_eq!(
        app.items_potion_kinds[9], night,
        "the same-kind pair the seat word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, night,
        "the same-kind pair the carried word keeps"
    );
    // ARM C — THE WORDLESS PAIR'S OWN LAW (the contents-less stack
    // the kinded potion the different components the swap answers —
    // the NONE word the kinded word the seats exchange)
    app.items_potion_kinds[9] = none;
    app.inv_carried_potion_kind = swift;
    app.click_inventory_slot(sx, sy, 0);
    assert_eq!(
        app.items_potion_kinds[9], swift,
        "the kinded word the seat wears"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the wordless stack the carried wears"
    );
    // ARM D — THE RIGHT PRESS'S OWN SILENCE (the never-swap law the
    // component pair the same no-op keeps — the kinds the own words
    // keep, the button gate the kind exchange never lends)
    app.items_potion_kinds[9] = night;
    app.inv_carried_potion_kind = swift;
    app.click_inventory_slot(sx, sy, 1);
    assert_eq!(
        app.items_potion_kinds[9], night,
        "the right press the seat word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, swift,
        "the right press the carried word keeps"
    );
    // ARM E — THE NON-POTION PAIR'S OWN PLAIN LAW (the dirt rows the
    // merge the plain id law wears, the kinds the NONE words keep,
    // the emptied carried the honest NONE retires)
    app.inventory.items[10] = (dirt, 3);
    app.items_potion_kinds[10] = none;
    app.inv_carried = (dirt, 2);
    app.inv_carried_potion_kind = none;
    let (sx1, sy1) = seat_of(10);
    app.click_inventory_slot(sx1, sy1, 0);
    assert_eq!(
        app.inventory.items[10],
        (dirt, 5),
        "the dirt pair the merge wears"
    );
    assert_eq!(app.inv_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.items_potion_kinds[10], none,
        "the dirt row the word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the emptied carried the NONE wears"
    );
}

/// THE TABLE STORE ROWS' OWN MODEL RIDE (the autopush-374 round —
/// the 373 NEXT list's own first candidate the store rows' law the
/// table's own store walk re-wears: the vanilla
/// `isSameItemSameComponents` law the plain (id, count) model never
/// saw — the same-id DIFFERENT-KIND potion pair the table's own
/// store row the vanilla SWAP answers, the same-kind pair the max-1
/// gate the honest silence keeps, the wordless pair the component
/// difference the same swap wears, the right press the never-swap
/// law the silence keeps, the non-potion ids the plain id law the
/// merge keeps).
#[test]
fn the_table_store_the_model_ride_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(374);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_crafting_table();
    assert_eq!(app.screen, ScreenKind::Crafting, "the table door opens");
    // THE REAL PANEL THE RENDER BANKS (the headless frame the seat
    // lookups the same walk the clicks share)
    let assets_root = assets_root();
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_probe, &mut a);
    }
    let panel = app.table_panel;
    let seat_of = |store: usize| -> (i32, i32) {
        let (x, y) = inventory_slot_origin(panel, store);
        (x + 8, y + 8)
    };
    // ARM A — THE DIFFERENT-KIND PAIR'S OWN SWAP (the vanilla law the
    // plain id model never wore — the pairs the identical stacks
    // keep, the KINDS the exchange the honest verdict wears)
    app.inventory.items[9] = (potion, 1);
    app.items_potion_kinds[9] = night;
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = swift;
    let (sx, sy) = seat_of(9);
    app.click_table_slot(sx, sy, 0);
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the seat the stack keeps"
    );
    assert_eq!(app.inv_carried, (potion, 1), "the carried the stack keeps");
    assert_eq!(
        app.items_potion_kinds[9], swift,
        "the carried's own word the seat wears"
    );
    assert_eq!(
        app.inv_carried_potion_kind, night,
        "the seat's own word the carried wears"
    );
    // ARM B — THE SAME-KIND PAIR'S OWN SILENCE (the same components
    // the same item speaks, the max-1 gate the merge closes, the
    // kinds the own words keep)
    app.items_potion_kinds[9] = night;
    app.inv_carried_potion_kind = night;
    app.click_table_slot(sx, sy, 0);
    assert_eq!(
        app.items_potion_kinds[9], night,
        "the same-kind pair the seat word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, night,
        "the same-kind pair the carried word keeps"
    );
    // ARM C — THE WORDLESS PAIR'S OWN LAW (the contents-less stack
    // the kinded potion the different components the swap answers —
    // the NONE word the kinded word the seats exchange)
    app.items_potion_kinds[9] = none;
    app.inv_carried_potion_kind = swift;
    app.click_table_slot(sx, sy, 0);
    assert_eq!(
        app.items_potion_kinds[9], swift,
        "the kinded word the seat wears"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the wordless stack the carried wears"
    );
    // ARM D — THE RIGHT PRESS'S OWN SILENCE (the never-swap law the
    // component pair the same no-op keeps — the kinds the own words
    // keep, the button gate the kind exchange never lends)
    app.items_potion_kinds[9] = night;
    app.inv_carried_potion_kind = swift;
    app.click_table_slot(sx, sy, 1);
    assert_eq!(
        app.items_potion_kinds[9], night,
        "the right press the seat word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, swift,
        "the right press the carried word keeps"
    );
    // ARM E — THE NON-POTION PAIR'S OWN PLAIN LAW (the dirt rows the
    // merge the plain id law wears, the kinds the NONE words keep,
    // the emptied carried the honest NONE retires)
    app.inventory.items[10] = (dirt, 3);
    app.items_potion_kinds[10] = none;
    app.inv_carried = (dirt, 2);
    app.inv_carried_potion_kind = none;
    let (sx1, sy1) = seat_of(10);
    app.click_table_slot(sx1, sy1, 0);
    assert_eq!(
        app.inventory.items[10],
        (dirt, 5),
        "the dirt pair the merge wears"
    );
    assert_eq!(app.inv_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.items_potion_kinds[10], none,
        "the dirt row the word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the emptied carried the NONE wears"
    );
}

/// THE OFFHAND SEAT'S OWN MODEL RIDE (the autopush-375 round — the
/// 374 NEXT list's own first candidate the 36TH seat re-wears: the
/// vanilla `isSameItemSameComponents` law the offhand's plain (id,
/// count) model never saw — the same-id DIFFERENT-KIND potion pair
/// the offhand's own pickup the vanilla SWAP answers, the same-kind
/// pair the max-1 gate the honest silence keeps, the wordless pair
/// the component difference the same swap wears, the right press the
/// never-swap law the silence keeps, the non-potion ids the plain id
/// law the merge keeps).
#[test]
fn the_offhand_seat_the_model_ride_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(375);
    app.enter_world();
    app.spawn_offline_body();
    // ARM A — THE DIFFERENT-KIND PAIR'S OWN SWAP (the vanilla law the
    // plain id model never wore — the pairs the identical stacks
    // keep, the KINDS the exchange the honest verdict wears)
    app.inventory.offhand[0] = (potion, 1);
    app.items_potion_kinds[OFFHAND_KIND_SEAT] = night;
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = swift;
    app.click_offhand_slot(0);
    assert_eq!(
        app.inventory.offhand[0],
        (potion, 1),
        "the seat the stack keeps"
    );
    assert_eq!(app.inv_carried, (potion, 1), "the carried the stack keeps");
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], swift,
        "the carried's own word the seat wears"
    );
    assert_eq!(
        app.inv_carried_potion_kind, night,
        "the seat's own word the carried wears"
    );
    // ARM B — THE SAME-KIND PAIR'S OWN SILENCE (the same components
    // the same item speaks, the max-1 gate the merge closes, the
    // kinds the own words keep)
    app.items_potion_kinds[OFFHAND_KIND_SEAT] = night;
    app.inv_carried_potion_kind = night;
    app.click_offhand_slot(0);
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], night,
        "the same-kind pair the seat word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, night,
        "the same-kind pair the carried word keeps"
    );
    // ARM C — THE WORDLESS PAIR'S OWN LAW (the contents-less stack
    // the kinded potion the different components the swap answers —
    // the NONE word the kinded word the seats exchange)
    app.items_potion_kinds[OFFHAND_KIND_SEAT] = none;
    app.inv_carried_potion_kind = swift;
    app.click_offhand_slot(0);
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], swift,
        "the kinded word the seat wears"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the wordless stack the carried wears"
    );
    // ARM D — THE RIGHT PRESS'S OWN SILENCE (the never-swap law the
    // component pair the same no-op keeps — the kinds the own words
    // keep, the button gate the kind exchange never lends)
    app.items_potion_kinds[OFFHAND_KIND_SEAT] = night;
    app.inv_carried_potion_kind = swift;
    app.click_offhand_slot(1);
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], night,
        "the right press the seat word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, swift,
        "the right press the carried word keeps"
    );
    // ARM E — THE NON-POTION PAIR'S OWN PLAIN LAW (the dirt rows the
    // merge the plain id law wears, the kinds the NONE words keep,
    // the emptied carried the honest NONE retires)
    app.inventory.offhand[0] = (dirt, 3);
    app.items_potion_kinds[OFFHAND_KIND_SEAT] = none;
    app.inv_carried = (dirt, 2);
    app.inv_carried_potion_kind = none;
    app.click_offhand_slot(0);
    assert_eq!(
        app.inventory.offhand[0],
        (dirt, 5),
        "the dirt pair the merge wears"
    );
    assert_eq!(app.inv_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.items_potion_kinds[OFFHAND_KIND_SEAT], none,
        "the dirt seat the word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the emptied carried the NONE wears"
    );
}

/// THE CRAFT CELLS' OWN MODEL RIDE (the dev-207 round — the 375 NEXT
/// list's own second candidate the 368 cells re-wear: the vanilla
/// `isSameItemSameComponents` law the craft cells' plain (id, count)
/// model never saw — the same-id DIFFERENT-KIND potion pair the
/// cell's own pickup the vanilla SWAP answers, the same-kind pair
/// the max-1 gate the honest silence keeps, the wordless pair the
/// component difference the same swap wears, the right press the
/// never-swap law the silence keeps, the non-potion ids the plain id
/// law the merge keeps).
#[test]
fn the_craft_cells_the_model_ride_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(207);
    app.enter_world();
    app.spawn_offline_body();
    // ARM A — THE DIFFERENT-KIND PAIR'S OWN SWAP (the vanilla law the
    // plain id model never wore — the pairs the identical stacks
    // keep, the KINDS the exchange the honest verdict wears)
    app.craft_grid[0] = (potion, 1);
    app.craft_grid_potion_kinds[0] = night;
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = swift;
    app.click_craft_cell(0, 0);
    assert_eq!(app.craft_grid[0], (potion, 1), "the cell the stack keeps");
    assert_eq!(app.inv_carried, (potion, 1), "the carried the stack keeps");
    assert_eq!(
        app.craft_grid_potion_kinds[0], swift,
        "the carried's own word the cell wears"
    );
    assert_eq!(
        app.inv_carried_potion_kind, night,
        "the cell's own word the carried wears"
    );
    // ARM B — THE SAME-KIND PAIR'S OWN SILENCE (the same components
    // the same item speaks, the max-1 gate the merge closes, the
    // kinds the own words keep)
    app.craft_grid_potion_kinds[0] = night;
    app.inv_carried_potion_kind = night;
    app.click_craft_cell(0, 0);
    assert_eq!(
        app.craft_grid_potion_kinds[0], night,
        "the same-kind pair the cell word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, night,
        "the same-kind pair the carried word keeps"
    );
    // ARM C — THE WORDLESS PAIR'S OWN LAW (the contents-less stack
    // the kinded potion the different components the swap answers —
    // the NONE word the kinded word the seats exchange)
    app.craft_grid_potion_kinds[0] = none;
    app.inv_carried_potion_kind = swift;
    app.click_craft_cell(0, 0);
    assert_eq!(
        app.craft_grid_potion_kinds[0], swift,
        "the kinded word the cell wears"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the wordless stack the carried wears"
    );
    // ARM D — THE RIGHT PRESS'S OWN SILENCE (the never-swap law the
    // component pair the same no-op keeps — the kinds the own words
    // keep, the button gate the kind exchange never lends)
    app.craft_grid_potion_kinds[0] = night;
    app.inv_carried_potion_kind = swift;
    app.click_craft_cell(0, 1);
    assert_eq!(
        app.craft_grid_potion_kinds[0], night,
        "the right press the cell word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, swift,
        "the right press the carried word keeps"
    );
    // ARM E — THE NON-POTION PAIR'S OWN PLAIN LAW (the dirt cells the
    // merge the plain id law wears, the kinds the NONE words keep,
    // the emptied carried the honest NONE retires)
    app.craft_grid[1] = (dirt, 3);
    app.craft_grid_potion_kinds[1] = none;
    app.inv_carried = (dirt, 2);
    app.inv_carried_potion_kind = none;
    app.click_craft_cell(1, 0);
    assert_eq!(
        app.craft_grid[1],
        (dirt, 5),
        "the dirt pair the merge wears"
    );
    assert_eq!(app.inv_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.craft_grid_potion_kinds[1], none,
        "the dirt cell the word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the emptied carried the NONE wears"
    );
}

/// THE TABLE CELLS' OWN MODEL RIDE (the 376 round — the dev-207 NEXT
/// list's own first candidate the 367 cells re-wear: the vanilla
/// `isSameItemSameComponents` law the table screen's 3x3 cells' plain
/// (id, count) model never saw — the same-id DIFFERENT-KIND potion
/// pair the cell's own pickup the vanilla SWAP answers, the same-kind
/// pair the max-1 gate the honest silence keeps, the wordless pair
/// the component difference the same swap wears, the right press the
/// never-swap law the silence keeps, the non-potion ids the plain id
/// law the merge keeps).
#[test]
fn the_table_cells_the_model_ride_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(376);
    app.enter_world();
    app.spawn_offline_body();
    // ARM A — THE DIFFERENT-KIND PAIR'S OWN SWAP (the vanilla law the
    // plain id model never wore — the pairs the identical stacks
    // keep, the KINDS the exchange the honest verdict wears)
    app.table_grid[0] = (potion, 1);
    app.table_grid_potion_kinds[0] = night;
    app.inv_carried = (potion, 1);
    app.inv_carried_potion_kind = swift;
    app.click_table_cell(0, 0);
    assert_eq!(app.table_grid[0], (potion, 1), "the cell the stack keeps");
    assert_eq!(app.inv_carried, (potion, 1), "the carried the stack keeps");
    assert_eq!(
        app.table_grid_potion_kinds[0], swift,
        "the carried's own word the cell wears"
    );
    assert_eq!(
        app.inv_carried_potion_kind, night,
        "the cell's own word the carried wears"
    );
    // ARM B — THE SAME-KIND PAIR'S OWN SILENCE (the same components
    // the same item speaks, the max-1 gate the merge closes, the
    // kinds the own words keep)
    app.table_grid_potion_kinds[0] = night;
    app.inv_carried_potion_kind = night;
    app.click_table_cell(0, 0);
    assert_eq!(
        app.table_grid_potion_kinds[0], night,
        "the same-kind pair the cell word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, night,
        "the same-kind pair the carried word keeps"
    );
    // ARM C — THE WORDLESS PAIR'S OWN LAW (the contents-less stack
    // the kinded potion the different components the swap answers —
    // the NONE word the kinded word the seats exchange)
    app.table_grid_potion_kinds[0] = none;
    app.inv_carried_potion_kind = swift;
    app.click_table_cell(0, 0);
    assert_eq!(
        app.table_grid_potion_kinds[0], swift,
        "the kinded word the cell wears"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the wordless stack the carried wears"
    );
    // ARM D — THE RIGHT PRESS'S OWN SILENCE (the never-swap law the
    // component pair the same no-op keeps — the kinds the own words
    // keep, the button gate the kind exchange never lends)
    app.table_grid_potion_kinds[0] = night;
    app.inv_carried_potion_kind = swift;
    app.click_table_cell(0, 1);
    assert_eq!(
        app.table_grid_potion_kinds[0], night,
        "the right press the cell word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, swift,
        "the right press the carried word keeps"
    );
    // ARM E — THE NON-POTION PAIR'S OWN PLAIN LAW (the dirt cells the
    // merge the plain id law wears, the kinds the NONE words keep,
    // the emptied carried the honest NONE retires)
    app.table_grid[1] = (dirt, 3);
    app.table_grid_potion_kinds[1] = none;
    app.inv_carried = (dirt, 2);
    app.inv_carried_potion_kind = none;
    app.click_table_cell(1, 0);
    assert_eq!(
        app.table_grid[1],
        (dirt, 5),
        "the dirt pair the merge wears"
    );
    assert_eq!(app.inv_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.table_grid_potion_kinds[1], none,
        "the dirt cell the word keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the emptied carried the NONE wears"
    );
}

/// THE BREWING STAND'S OWN MODEL RIDE (the 377 round — the 376 NEXT
/// list's own first candidate the 339/dev-199 seats re-wear: the
/// vanilla `isSameItemSameComponents` law the brewing stand's plain
/// (id, count) model never saw — the same-id DIFFERENT-KIND potion
/// pair the seat's own pickup the vanilla SWAP answers, the same-kind
/// pair the max-1 gate the honest silence keeps, the wordless pair
/// the component difference the same swap wears, the right press the
/// never-swap law the silence keeps, the non-potion ids the plain id
/// law the merge keeps).
#[test]
fn the_brewing_seats_the_model_ride_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(377);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::BREWING_STAND);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_brewing_for(cell);
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    // THE SEAT COORD (the scoped lookup the mutable walk keeps — the
    // rects the render laid the click lane reads, the bottle seat 0
    // the five's own kind bank wears)
    let (bx, by) = {
        app.brewing_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 0)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // ARM A — THE DIFFERENT-KIND PAIR'S OWN SWAP (the vanilla law the
    // plain id model never wore — the pairs the identical stacks
    // keep, the KINDS the exchange the honest verdict wears)
    app.brewing_slots[0] = (potion, 1);
    app.brewing_kinds[0] = night;
    app.brewing_carried = (potion, 1);
    app.brewing_carried_kind = swift;
    app.click_brewing_slot(bx + 8, by + 8, 0);
    assert_eq!(
        app.brewing_slots[0],
        (potion, 1),
        "the seat the stack keeps"
    );
    assert_eq!(
        app.brewing_carried,
        (potion, 1),
        "the carried the stack keeps"
    );
    assert_eq!(
        app.brewing_kinds[0], swift,
        "the carried's own word the seat wears"
    );
    assert_eq!(
        app.brewing_carried_kind, night,
        "the seat's own word the carried wears"
    );
    // ARM B — THE SAME-KIND PAIR'S OWN SILENCE (the same components
    // the same item speaks, the max-1 gate the merge closes, the
    // kinds the own words keep)
    app.brewing_kinds[0] = night;
    app.brewing_carried_kind = night;
    app.click_brewing_slot(bx + 8, by + 8, 0);
    assert_eq!(
        app.brewing_kinds[0], night,
        "the same-kind pair the seat word keeps"
    );
    assert_eq!(
        app.brewing_carried_kind, night,
        "the same-kind pair the carried word keeps"
    );
    // ARM C — THE WORDLESS PAIR'S OWN LAW (the contents-less stack
    // the kinded potion the different components the swap answers —
    // the NONE word the kinded word the seats exchange)
    app.brewing_kinds[0] = none;
    app.brewing_carried_kind = swift;
    app.click_brewing_slot(bx + 8, by + 8, 0);
    assert_eq!(
        app.brewing_kinds[0], swift,
        "the kinded word the seat wears"
    );
    assert_eq!(
        app.brewing_carried_kind, none,
        "the wordless stack the carried wears"
    );
    // ARM D — THE RIGHT PRESS'S OWN SILENCE (the never-swap law the
    // component pair the same no-op keeps — the kinds the own words
    // keep, the button gate the kind exchange never lends)
    app.brewing_kinds[0] = night;
    app.brewing_carried_kind = swift;
    app.click_brewing_slot(bx + 8, by + 8, 1);
    assert_eq!(
        app.brewing_kinds[0], night,
        "the right press the seat word keeps"
    );
    assert_eq!(
        app.brewing_carried_kind, swift,
        "the right press the carried word keeps"
    );
    // ARM E — THE NON-POTION PAIR'S OWN PLAIN LAW (the dirt seats the
    // merge the plain id law wears, the kinds the NONE words keep,
    // the emptied carried the honest NONE retires)
    app.brewing_slots[0] = (dirt, 3);
    app.brewing_kinds[0] = none;
    app.brewing_carried = (dirt, 2);
    app.brewing_carried_kind = none;
    app.click_brewing_slot(bx + 8, by + 8, 0);
    assert_eq!(
        app.brewing_slots[0],
        (dirt, 5),
        "the dirt pair the merge wears"
    );
    assert_eq!(app.brewing_carried, (0, 0), "the merge the carried empties");
    assert_eq!(app.brewing_kinds[0], none, "the dirt seat the word keeps");
    assert_eq!(
        app.brewing_carried_kind, none,
        "the emptied carried the NONE wears"
    );
}

/// THE DISPENSER'S OWN MODEL RIDE (the 378 round — the 377 NEXT list's
/// own first candidate the dev-204 seats re-wear: the vanilla
/// `isSameItemSameComponents` law the dispenser's plain (id, count)
/// model never saw — the same-id DIFFERENT-KIND potion pair the
/// seat's own pickup the vanilla SWAP answers, the merge gate the
/// no-op wore, the nine the kindless store the honest wall keeps,
/// the mirror rows the kinded words the exchange banks).
#[test]
fn the_dispenser_seats_the_model_ride_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(378);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_dispenser_for(cell);
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    // THE SEAT COORDS (the scoped lookup the mutable walk keeps — the
    // grid head 0 the nine's kindless store wears, the mirror head 9
    // the REAL inventory's row the walk reads)
    let (gx, gy) = {
        app.dispenser_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 0)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    let (mx, my) = {
        app.dispenser_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 9)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // ARM A — THE WORDLESS SEAT'S OWN SWAP (the vanilla law the plain
    // id model never wore — the wordless potion the kinded carried
    // the different components answers, the stacks the identical
    // pairs keep, the carried word the kindless store retires)
    app.dispenser_slots[0] = (potion, 1);
    app.dispenser_carried = (potion, 1);
    app.dispenser_carried_potion_kind = swift;
    app.click_dispenser_slot(gx + 8, gy + 8, 0);
    assert_eq!(
        app.dispenser_slots[0],
        (potion, 1),
        "the seat the stack keeps"
    );
    assert_eq!(
        app.dispenser_carried,
        (potion, 1),
        "the carried the stack keeps"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the kindless store the carried word retires"
    );
    // ARM B — THE SAME-WORDLESS PAIR'S OWN SILENCE (the same
    // components the same item speaks, the max-1 gate the merge
    // closes, the kinds the NONE words keep)
    app.dispenser_carried_potion_kind = none;
    app.click_dispenser_slot(gx + 8, gy + 8, 0);
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the same-wordless pair the carried word keeps"
    );
    // ARM C — THE MIRROR ROW'S OWN EXCHANGE (the kinded row the
    // kinded carried the different components the swap answers — the
    // kinds the pairs exchange, the row the carried's word banks)
    app.inventory.items[9] = (potion, 1);
    app.items_potion_kinds[9] = night;
    app.dispenser_carried = (potion, 1);
    app.dispenser_carried_potion_kind = swift;
    app.click_dispenser_slot(mx + 8, my + 8, 0);
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the row the stack keeps"
    );
    assert_eq!(
        app.items_potion_kinds[9], swift,
        "the carried's own kind the row banks"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, night,
        "the row's own kind the carried banks"
    );
    // ARM D — THE RIGHT PRESS'S OWN SILENCE (the never-swap law the
    // component pair the same no-op keeps — the kinds the own words
    // keep, the button gate the kind exchange never lends)
    app.dispenser_slots[0] = (potion, 1);
    app.dispenser_carried = (potion, 1);
    app.dispenser_carried_potion_kind = swift;
    app.click_dispenser_slot(gx + 8, gy + 8, 1);
    assert_eq!(
        app.dispenser_carried_potion_kind, swift,
        "the right press the carried word keeps"
    );
    // ARM E — THE NON-POTION PAIR'S OWN PLAIN LAW (the dirt seats the
    // merge the plain id law wears, the kinds the NONE words keep,
    // the emptied carried the honest NONE retires)
    app.dispenser_slots[0] = (dirt, 3);
    app.dispenser_carried = (dirt, 2);
    app.dispenser_carried_potion_kind = none;
    app.click_dispenser_slot(gx + 8, gy + 8, 0);
    assert_eq!(
        app.dispenser_slots[0],
        (dirt, 5),
        "the dirt pair the merge wears"
    );
    assert_eq!(
        app.dispenser_carried,
        (0, 0),
        "the merge the carried empties"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the emptied carried the NONE wears"
    );
}

/// THE FURNACE'S OWN MODEL RIDE (the dev-208 round — the 378 NEXT
/// list's own walk the plain id law re-wears: the vanilla
/// `isSameItemSameComponents` law the furnace's plain (id, count)
/// model never saw — the same-id DIFFERENT-KIND potion pair the
/// seat's own pickup the vanilla SWAP answers, the merge gate the
/// no-op wore, the three the kindless store the honest wall keeps,
/// the store rows the kinded words the exchange banks).
#[test]
fn the_furnace_seats_the_model_ride_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(208);
    app.enter_world();
    app.open_furnace((0, 0, 0));
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    // THE SEAT COORDS (the scoped lookup the mutable walk keeps — the
    // input cell 0 the kindless three wears, the main row head 3 the
    // store rows' write-through answers)
    let (ix, iy) = {
        app.furnace_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 0)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    let (rx, ry) = {
        app.furnace_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 3)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    let (dx, dy) = {
        app.furnace_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 4)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // ARM A — THE WORDLESS SEAT'S OWN SWAP (the vanilla law the plain
    // id model never wore — the wordless potion the kinded carried
    // the different components answers, the stacks the identical
    // pairs keep, the carried word the kindless store retires)
    app.furnace_slots[0] = (potion, 1);
    app.furnace_carried = (potion, 1);
    app.furnace_carried_potion_kind = swift;
    app.click_furnace_slot(ix + 8, iy + 8, 0);
    assert_eq!(
        app.furnace_slots[0],
        (potion, 1),
        "the seat the stack keeps"
    );
    assert_eq!(
        app.furnace_carried,
        (potion, 1),
        "the carried the stack keeps"
    );
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the kindless store the carried word retires"
    );
    // ARM B — THE SAME-WORDLESS PAIR'S OWN SILENCE (the same
    // components the same item speaks, the max-1 gate the merge
    // closes, the kind the NONE word keeps)
    app.furnace_carried_potion_kind = none;
    app.click_furnace_slot(ix + 8, iy + 8, 0);
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the same-wordless pair the carried word keeps"
    );
    // ARM C — THE RIGHT PRESS'S OWN SILENCE (the never-swap law the
    // component pair the same no-op keeps — the kind the own word
    // keeps, the button gate the kind retire never lends)
    app.furnace_carried_potion_kind = swift;
    app.click_furnace_slot(ix + 8, iy + 8, 1);
    assert_eq!(
        app.furnace_carried_potion_kind, swift,
        "the right press the carried word keeps"
    );
    // ARM D — THE STORE ROW'S OWN EXCHANGE (the kinded row the kinded
    // carried the different components the swap answers — the kinds
    // the pairs exchange, the row the carried's word banks)
    app.inventory.items[9] = (potion, 1);
    app.items_potion_kinds[9] = night;
    app.furnace_carried = (potion, 1);
    app.furnace_carried_potion_kind = swift;
    app.click_furnace_slot(rx + 8, ry + 8, 0);
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the row the stack keeps"
    );
    assert_eq!(
        app.items_potion_kinds[9], swift,
        "the carried's own kind the row banks"
    );
    assert_eq!(
        app.furnace_carried_potion_kind, night,
        "the row's own kind the carried banks"
    );
    // ARM E — THE NON-POTION PAIR'S OWN PLAIN LAW (the dirt rows the
    // merge the plain id law wears, the kinds the NONE words keep,
    // the emptied carried the honest NONE retires)
    app.inventory.items[10] = (dirt, 3);
    app.items_potion_kinds[10] = none;
    app.furnace_carried = (dirt, 2);
    app.furnace_carried_potion_kind = none;
    app.click_furnace_slot(dx + 8, dy + 8, 0);
    assert_eq!(
        app.inventory.items[10],
        (dirt, 5),
        "the dirt pair the merge wears"
    );
    assert_eq!(app.furnace_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the emptied carried the NONE wears"
    );
}

/// THE ENCHANTING SEATS' OWN KIND BANK (the 379 round — the dev-208
/// NEXT list's own walk the plain id law re-wears: the vanilla
/// `isSameItemSameComponents` law the enchant's plain (id, count)
/// model never saw — the same-id DIFFERENT-KIND potion pair the
/// seat's own pickup the vanilla SWAP answers, the merge gate the
/// no-op wore, the pair seats the kindless store the honest wall
/// keeps, the store rows the kinded words the exchange banks).
#[test]
fn the_enchant_seats_the_kind_bank_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(379);
    app.enter_world();
    app.open_enchanting_for((0, 0, 0));
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    // THE SEAT COORDS (the scoped lookup the mutable walk keeps — the
    // item seat 0 the kindless pair wears, the main rows 2 and 3 the
    // store rows' write-through answers)
    let (ix, iy) = {
        app.enchant_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 0)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    let (rx, ry) = {
        app.enchant_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 2)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    let (dx, dy) = {
        app.enchant_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 3)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    // ARM A — THE WORDLESS SEAT'S OWN SWAP (the vanilla law the plain
    // id model never wore — the wordless potion the kinded carried
    // the different components answers, the stacks the identical
    // pairs keep, the carried word the kindless pair store retires)
    app.enchant_slots[0] = (potion, 1);
    app.enchant_carried = (potion, 1);
    app.enchant_carried_potion_kind = swift;
    app.click_enchanting_slot(ix + 8, iy + 8, 0);
    assert_eq!(
        app.enchant_slots[0],
        (potion, 1),
        "the seat the stack keeps"
    );
    assert_eq!(
        app.enchant_carried,
        (potion, 1),
        "the carried the stack keeps"
    );
    assert_eq!(
        app.enchant_carried_potion_kind, none,
        "the kindless pair store the carried word retires"
    );
    // ARM B — THE SAME-WORDLESS PAIR'S OWN SILENCE (the same
    // components the same item speaks, the max-1 gate the merge
    // closes, the kind the NONE word keeps)
    app.enchant_carried_potion_kind = none;
    app.click_enchanting_slot(ix + 8, iy + 8, 0);
    assert_eq!(
        app.enchant_carried_potion_kind, none,
        "the same-wordless pair the carried word keeps"
    );
    // ARM C — THE RIGHT PRESS'S OWN SILENCE (the never-swap law the
    // component pair the same no-op keeps — the kind the own word
    // keeps, the button gate the kind retire never lends)
    app.enchant_carried_potion_kind = swift;
    app.click_enchanting_slot(ix + 8, iy + 8, 1);
    assert_eq!(
        app.enchant_carried_potion_kind, swift,
        "the right press the carried word keeps"
    );
    // ARM D — THE STORE ROW'S OWN EXCHANGE (the kinded row the kinded
    // carried the different components the swap answers — the kinds
    // the pairs exchange, the row the carried's word banks)
    app.inventory.items[9] = (potion, 1);
    app.items_potion_kinds[9] = night;
    app.enchant_carried = (potion, 1);
    app.enchant_carried_potion_kind = swift;
    app.click_enchanting_slot(rx + 8, ry + 8, 0);
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the row the stack keeps"
    );
    assert_eq!(
        app.items_potion_kinds[9], swift,
        "the carried's own kind the row banks"
    );
    assert_eq!(
        app.enchant_carried_potion_kind, night,
        "the row's own kind the carried banks"
    );
    // ARM E — THE NON-POTION PAIR'S OWN PLAIN LAW (the dirt rows the
    // merge the plain id law wears, the kinds the NONE words keep,
    // the emptied carried the honest NONE retires)
    app.inventory.items[10] = (dirt, 3);
    app.items_potion_kinds[10] = none;
    app.enchant_carried = (dirt, 2);
    app.enchant_carried_potion_kind = none;
    app.click_enchanting_slot(dx + 8, dy + 8, 0);
    assert_eq!(
        app.inventory.items[10],
        (dirt, 5),
        "the dirt pair the merge wears"
    );
    assert_eq!(app.enchant_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.enchant_carried_potion_kind, none,
        "the emptied carried the NONE wears"
    );
}

/// THE DRAG'S OWN KIND RETIRE (autopush-380 — the nine distribute
/// lanes the emptied carried the NONE word wears): the chest offline
/// lane's own law the family's tails re-wear — the drag's place math
/// the carried empties, the stale kind word the tail retires, the
/// partial carry the remainder the same stack the word keeps.
#[test]
fn the_drag_tails_the_kind_word_retires_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(380);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    // ARM A — THE GENERIC LANE'S OWN RETIRE (the store rows 9 and 36
    // the write-through wears, the exact split the carried empties,
    // the stale word the NONE retires)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the split reads
    }
    app.open_inventory();
    app.inv_carried = (potion, 2);
    app.inv_carried_potion_kind = night;
    app.qc_slots = vec![9, 36];
    app.qc_type = 0;
    app.qc_distribute_offline();
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the store row's own one"
    );
    assert_eq!(
        app.inventory.items[0],
        (potion, 1),
        "the hotbar row's own one"
    );
    assert_eq!(
        app.inv_carried,
        (0, 0),
        "the exact split the carried empties"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the emptied carried the word retires"
    );
    // ARM B — THE PARTIAL CARRY'S OWN KEEP (the remainder the same
    // stack the word keeps — the retire the emptied tail alone wears;
    // the fresh seats the max-1 potion the merge never wears — the
    // floor split the odd count the one item keeps)
    app.inv_carried = (potion, 3);
    app.inv_carried_potion_kind = night;
    app.qc_slots = vec![10, 37];
    app.qc_type = 0;
    app.qc_distribute_offline();
    assert_eq!(
        app.inventory.items[10],
        (potion, 1),
        "the second row's own one"
    );
    assert_eq!(
        app.inventory.items[1],
        (potion, 1),
        "the hotbar row's own one"
    );
    assert_eq!(
        app.inv_carried,
        (potion, 1),
        "the floor split the remainder keeps"
    );
    assert_eq!(
        app.inv_carried_potion_kind, night,
        "the remainder the same stack the word keeps"
    );
    // ARM C — THE TABLE LANE'S OWN RETIRE (the store rows 10 and 37
    // the inverse folds wear, the exact split the carried empties)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_crafting_table();
    app.inv_carried = (potion, 2);
    app.inv_carried_potion_kind = night;
    app.qc_slots = vec![10, 37];
    app.qc_type = 0;
    app.qc_distribute_table_offline();
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the main row's own one"
    );
    assert_eq!(
        app.inventory.items[0],
        (potion, 1),
        "the hotbar row's own one"
    );
    assert_eq!(
        app.inv_carried,
        (0, 0),
        "the exact split the carried empties"
    );
    assert_eq!(
        app.inv_carried_potion_kind, none,
        "the emptied carried the word retires"
    );
    // ARM D — THE DISPENSER LANE'S OWN RETIRE (the grid seat 0 the
    // nine's kindless store wears, the mirror row the write-through
    // speaks, the stale word the tail retires)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
    app.open_dispenser_for(cell);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the re-entry's own restore the deterministic zero re-wears
    }
    for s in app.dispenser_slots.iter_mut() {
        *s = (0, 0);
    }
    app.dispenser_mirror_sync_from_inventory();
    app.dispenser_carried = (potion, 2);
    app.dispenser_carried_potion_kind = night;
    app.qc_slots = vec![0, 9];
    app.qc_type = 0;
    app.qc_distribute_dispenser_offline();
    assert_eq!(
        app.dispenser_slots[0],
        (potion, 1),
        "the grid seat's own one"
    );
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the mirror row the write-through speaks"
    );
    assert_eq!(
        app.dispenser_carried,
        (0, 0),
        "the exact split the carried empties"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the emptied carried the word retires"
    );
    // ARM E — THE BREWING LANE'S OWN RETIRE (the stand's five the
    // grid head wears, the mirror row the write-through speaks)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::BREWING_STAND);
    app.open_brewing_for(cell);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the re-entry's own restore the deterministic zero re-wears
    }
    for s in app.brewing_slots.iter_mut() {
        *s = (0, 0);
    }
    app.brewing_mirror_sync_from_inventory();
    app.brewing_carried = (potion, 2);
    app.brewing_carried_kind = night;
    app.qc_slots = vec![0, 5];
    app.qc_type = 0;
    app.qc_distribute_brewing_offline();
    assert_eq!(
        app.brewing_slots[0],
        (potion, 1),
        "the stand seat's own one"
    );
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the mirror row the write-through speaks"
    );
    assert_eq!(
        app.brewing_carried,
        (0, 0),
        "the exact split the carried empties"
    );
    assert_eq!(
        app.brewing_carried_kind, none,
        "the emptied carried the word retires"
    );
    // ARM F — THE ENCHANT LANE'S OWN RETIRE (the pair's item seat the
    // transient store wears, the mirror row the write-through speaks)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_enchanting_for(cell);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the re-entry's own restore the deterministic zero re-wears
    }
    for s in app.enchant_slots.iter_mut() {
        *s = (0, 0);
    }
    app.enchant_mirror_sync_from_inventory();
    app.enchant_carried = (potion, 2);
    app.enchant_carried_potion_kind = night;
    app.qc_slots = vec![0, 2];
    app.qc_type = 0;
    app.qc_distribute_enchanting_offline();
    assert_eq!(app.enchant_slots[0], (potion, 1), "the pair seat's own one");
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the mirror row the write-through speaks"
    );
    assert_eq!(
        app.enchant_carried,
        (0, 0),
        "the exact split the carried empties"
    );
    assert_eq!(
        app.enchant_carried_potion_kind, none,
        "the emptied carried the word retires"
    );
    // ARM G — THE FURNACE LANE'S OWN RETIRE (the store rows 3 and 30
    // the flat belief wears, the write-through the both cells speak)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.open_furnace(cell);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the re-entry's own restore the deterministic zero re-wears
    }
    for s in app.furnace_slots.iter_mut() {
        *s = (0, 0); // the open's own projection the deterministic zeros re-wear
    }
    app.furnace_carried = (potion, 2);
    app.furnace_carried_potion_kind = night;
    app.qc_slots = vec![3, 30];
    app.qc_type = 0;
    app.qc_distribute_furnace_offline();
    assert_eq!(app.furnace_slots[3], (potion, 1), "the store row's own one");
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the write-through the main row speaks"
    );
    assert_eq!(
        app.furnace_carried,
        (0, 0),
        "the exact split the carried empties"
    );
    assert_eq!(
        app.furnace_carried_potion_kind, none,
        "the emptied carried the word retires"
    );
    // ARM H — THE WIRE CHEST LANE'S OWN RETIRE (the live window the
    // packets the sink rides, the free fn the belief answers, the
    // caller's own tail the word retires)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    app.open_chest_for(cell);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the re-entry's own restore the deterministic zero re-wears
    }
    for s in app.chest_slots.iter_mut() {
        *s = (0, 0);
    }
    app.chest_mirror_sync_from_inventory();
    app.live_clicks = Some(std::sync::mpsc::channel().0);
    app.chest_is_live = true;
    app.chest_live_window = Some((5, 11));
    app.chest_carried = (potion, 2);
    app.chest_carried_potion_kind = night;
    app.qc_slots = vec![4, 54];
    app.qc_type = 0;
    app.qc_distribute_chest_wire();
    assert_eq!(app.chest_slots[4], (potion, 1), "the grid cell's own one");
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the exact split the carried empties"
    );
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the emptied carried the word retires"
    );
    // ARM I — THE WIRE DISPENSER LANE'S OWN RETIRE (the live window
    // the packets ride, the fresh stand the re-open's own law keeps,
    // the caller's own tail the word retires)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
    app.open_dispenser_for(cell);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the re-entry's own restore the deterministic zero re-wears
    }
    for s in app.dispenser_slots.iter_mut() {
        *s = (0, 0);
    }
    app.dispenser_mirror_sync_from_inventory();
    app.live_clicks = Some(std::sync::mpsc::channel().0);
    app.dispenser_is_live = true;
    app.dispenser_live_window = Some((6, 12));
    app.dispenser_carried = (potion, 2);
    app.dispenser_carried_potion_kind = night;
    app.qc_slots = vec![0, 9];
    app.qc_type = 0;
    app.qc_distribute_dispenser_wire();
    assert_eq!(
        app.dispenser_carried,
        (0, 0),
        "the exact split the carried empties"
    );
    assert_eq!(
        app.dispenser_carried_potion_kind, none,
        "the emptied carried the word retires"
    );
    // ARM J — THE WIRE GENERIC LANE'S OWN RETIRE (the live belief the
    // free fn answers, the caller's own tail the word retires)
    app.live_clicks = Some(std::sync::mpsc::channel().0);
    app.live_inv_carried = (potion, 2);
    app.live_carried_potion_kind = night;
    app.qc_slots = vec![9, 36];
    app.qc_type = 0;
    app.qc_distribute_wire();
    assert_eq!(
        app.live_inv_carried,
        (0, 0),
        "the exact split the carried empties"
    );
    assert_eq!(
        app.live_carried_potion_kind, none,
        "the emptied carried the word retires"
    );
}

/// THE QUICK-MOVE'S OWN KIND RIDE (autopush-381 — the four shift
/// lanes the moved stack the source's word rides): the drag retire's
/// own law the shift tails re-wear — the drained source the NONE
/// retires, the filled empty cell the word banks, the full section
/// the silence keeps, the live belief the same tail wears.
#[test]
fn the_quick_move_the_kind_word_rides_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(381);
    app.enter_world();
    app.shift_down = true;
    let panel = (100, 40);
    app.inv_panel = panel;
    // ARM A — THE SURVIVAL'S HOTBAR→MAIN RIDE (the occupied cell the
    // merge skips, the empty fill the word banks, the drained source
    // the NONE retires)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    for k in app.items_potion_kinds.iter_mut() {
        *k = none;
    }
    app.inventory.items[0] = (potion, 1);
    app.items_potion_kinds[0] = night;
    app.inventory.items[9] = (dirt, 64);
    let (sx, sy) = inventory_slot_origin(panel, 0);
    app.click_inventory_slot(sx + 8, sy + 8, 0);
    assert_eq!(
        app.inventory.items[10],
        (potion, 1),
        "the fill the empty cell wears"
    );
    assert_eq!(
        app.items_potion_kinds[10], night,
        "the moved stack the word banks"
    );
    assert_eq!(app.inventory.items[0], (0, 0), "the source the move drains");
    assert_eq!(
        app.items_potion_kinds[0], none,
        "the drained source the word retires"
    );
    // ARM B — THE MAIN→HOTBAR RIDE (the hotbar's head the fill takes,
    // the word the same ride banks)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    for k in app.items_potion_kinds.iter_mut() {
        *k = none;
    }
    app.inventory.items[20] = (potion, 1);
    app.items_potion_kinds[20] = swift;
    let (bx, by) = inventory_slot_origin(panel, 20);
    app.click_inventory_slot(bx + 8, by + 8, 0);
    assert_eq!(
        app.inventory.items[0],
        (potion, 1),
        "the hotbar head the fill takes"
    );
    assert_eq!(
        app.items_potion_kinds[0], swift,
        "the word the hotbar row banks"
    );
    assert_eq!(
        app.items_potion_kinds[20], none,
        "the drained source the word retires"
    );
    assert_eq!(
        app.inventory.items[20],
        (0, 0),
        "the source the move drains"
    );
    // ARM C — THE FULL SECTION'S OWN SILENCE (the hotbar the max-1
    // potions fill, the move the no-room walk refuses, the stack the
    // source keeps, the word the same row wears)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    for k in app.items_potion_kinds.iter_mut() {
        *k = none;
    }
    for i in 0..9 {
        app.inventory.items[i] = (potion, 1);
    }
    app.inventory.items[30] = (potion, 1);
    app.items_potion_kinds[30] = night;
    let (cx, cy) = inventory_slot_origin(panel, 30);
    app.click_inventory_slot(cx + 8, cy + 8, 0);
    assert_eq!(
        app.inventory.items[30],
        (potion, 1),
        "the no-room walk the stack keeps"
    );
    assert_eq!(
        app.items_potion_kinds[30], night,
        "the silence the source's word wears"
    );
    // ARM D — THE TABLE STORE ROW'S OWN RIDE (the table screen's
    // store rows the same shift lane wears, the hotbar head the fill
    // takes, the word the same banks ride)
    app.table_panel = (60, 30);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    for k in app.items_potion_kinds.iter_mut() {
        *k = none;
    }
    app.inventory.items[9] = (potion, 1);
    app.items_potion_kinds[9] = night;
    let (tx, ty) = inventory_slot_origin((60, 30), 9);
    app.click_table_slot(tx + 8, ty + 8, 0);
    assert_eq!(
        app.inventory.items[0],
        (potion, 1),
        "the hotbar head the fill takes"
    );
    assert_eq!(
        app.items_potion_kinds[0], night,
        "the word the hotbar row banks"
    );
    assert_eq!(
        app.items_potion_kinds[9], none,
        "the drained source the word retires"
    );
    assert_eq!(app.inventory.items[9], (0, 0), "the source the move drains");
    // ARM E — THE CREATIVE WRAP'S OWN RIDE (the wrap's hotbar seat
    // the same shift arm wears, the main head the fill takes)
    app.creative_panel = (200, 20);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    for k in app.items_potion_kinds.iter_mut() {
        *k = none;
    }
    app.inventory.items[0] = (potion, 1);
    app.items_potion_kinds[0] = night;
    app.click_creative_wrap(200 + 9 + 8, 20 + 112 + 8, 0);
    assert_eq!(
        app.inventory.items[9],
        (potion, 1),
        "the main head the fill takes"
    );
    assert_eq!(
        app.items_potion_kinds[9], night,
        "the word the main row banks"
    );
    assert_eq!(
        app.items_potion_kinds[0], none,
        "the drained source the word retires"
    );
    assert_eq!(app.inventory.items[0], (0, 0), "the source the move drains");
    // ARM F — THE LIVE INVENTORY'S OWN RIDE (the live belief the same
    // tail wears, the mirror cell the normalized read gates, the
    // packet the changed map rides)
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    for k in app.items_potion_kinds.iter_mut() {
        *k = none;
    }
    for s in app.live_inv_slots.iter_mut() {
        *s = (0, 0);
    }
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.live_clicks = Some(tx);
    app.live_inv_state_id = 9;
    app.live_inv_slots[36] = (potion, 1); // store_menu_id(0) — the hotbar head
    app.items_potion_kinds[0] = night;
    let (lx, ly) = inventory_slot_origin(panel, 0);
    app.click_wire_inventory_slot(lx + 8, ly + 8, 0);
    assert_eq!(
        app.live_inv_slots[9],
        (potion, 1),
        "the main head the fill takes"
    );
    assert_eq!(app.live_inv_slots[36], (0, 0), "the source the move drains");
    assert_eq!(
        app.items_potion_kinds[9], night,
        "the word the main row banks"
    );
    assert_eq!(
        app.items_potion_kinds[0], none,
        "the drained source the word retires"
    );
    match rx.try_recv() {
        Ok(crate::world_view::ContainerCommand::Click(c)) => {
            assert_eq!(c.slot, 36, "the hotbar head the menu id rides");
            assert_eq!(
                c.input,
                crate::world_view::ContainerInput::QuickMove,
                "the quick-move input speaks"
            );
        }
        _ => panic!("the packet the changed map rides"),
    }
}

/// THE DRAG'S OWN COLLECT VERDICT (the autopush-382 round — the
/// shouldAdd's canItemQuickReplace the component half the plain id
/// gate never wore): the chest paint lane the same-id kind pair the
/// seat's own word refuses — the different kind the paint the seat
/// never joins, the release the PICKUP the swap answers, the
/// same-kind pair the paint the distribution's merge serves, the
/// wordless pair the plain law keeps, the kindless seat the carried's
/// word the mismatch refuses, the right drag the refused seat the
/// never-swap silence keeps, the dirt pair the plain merge the kind
/// tail retires.
#[test]
fn the_drag_collect_the_kind_pair_refuses_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(382);
    app.enter_world();
    app.spawn_offline_body();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    for k in app.items_potion_kinds.iter_mut() {
        *k = none;
    }
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::CHEST);
    app.open_chest_for(cell);
    assert_eq!(app.screen, ScreenKind::Chest, "the chest door opens");
    // THE REAL RECTS THE RENDER BANKS (the headless frame the seat
    // lookups the same walk the clicks share)
    let assets_root = assets_root();
    let mut fb_probe = crate::raster::Framebuf::new(320, 180);
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb_probe, &mut a);
    }
    let seat_of = |rects: &[(i32, i32, i16)], slot: i16| -> (i32, i32) {
        rects
            .iter()
            .find(|&&(_, _, s)| s == slot)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    let (gx, gy) = seat_of(&app.chest_slot_rects, 0);
    // ARM A — THE DIFFERENT-KIND PAIR'S OWN REFUSAL (the paint the
    // seat never joins, the release the PICKUP the swap answers —
    // the stacks the exchange the honest verdict wears)
    app.chest_slots[0] = (potion, 1);
    app.chest_row_potion_kinds[0] = night;
    app.chest_carried = (potion, 3);
    app.chest_carried_potion_kind = swift;
    app.qc_arm_chest(gx + 8, gy + 8, 0);
    app.qc_paint_chest(gx + 8, gy + 8);
    assert!(
        !app.qc_slots.contains(&0),
        "the different kind the paint refuses"
    );
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(0);
    assert_eq!(app.chest_slots[0], (potion, 3), "the swap the seat wears");
    assert_eq!(
        app.chest_row_potion_kinds[0], swift,
        "the carried's own word the seat wears"
    );
    assert_eq!(
        app.chest_carried,
        (potion, 1),
        "the seat's own stack the carried wears"
    );
    assert_eq!(
        app.chest_carried_potion_kind, night,
        "the seat's own word the carried wears"
    );
    // ARM B — THE SAME-KIND PAIR'S OWN MERGE (the paint the seat
    // joins, the distribution the max-1 cap the silence serves — the
    // kinds the own words keep)
    app.chest_slots[0] = (potion, 1);
    app.chest_row_potion_kinds[0] = night;
    app.chest_carried = (potion, 3);
    app.chest_carried_potion_kind = night;
    app.qc_arm_chest(gx + 8, gy + 8, 0);
    app.qc_paint_chest(gx + 8, gy + 8);
    assert!(app.qc_slots.contains(&0), "the same kind the paint joins");
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(0);
    assert_eq!(
        app.chest_slots[0],
        (potion, 1),
        "the max-1 cap the seat keeps"
    );
    assert_eq!(
        app.chest_row_potion_kinds[0], night,
        "the same-kind pair the seat word keeps"
    );
    assert_eq!(
        app.chest_carried,
        (potion, 3),
        "the max-1 cap the carried keeps"
    );
    assert_eq!(
        app.chest_carried_potion_kind, night,
        "the same-kind pair the carried word keeps"
    );
    // ARM C — THE WORDLESS PAIR'S OWN PLAIN LAW (the NONE words the
    // same item speaks, the paint the seat joins, the max-1 silence
    // the same numbers keep)
    app.chest_slots[0] = (potion, 1);
    app.chest_row_potion_kinds[0] = none;
    app.chest_carried = (potion, 3);
    app.chest_carried_potion_kind = none;
    app.qc_arm_chest(gx + 8, gy + 8, 0);
    app.qc_paint_chest(gx + 8, gy + 8);
    assert!(
        app.qc_slots.contains(&0),
        "the wordless pair the paint joins"
    );
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(0);
    assert_eq!(
        app.chest_slots[0],
        (potion, 1),
        "the max-1 cap the seat keeps"
    );
    assert_eq!(
        app.chest_row_potion_kinds[0], none,
        "the wordless pair the seat word keeps"
    );
    // ARM D — THE KINDLESS SEAT'S OWN REFUSAL (the carried's word the
    // wordless seat the mismatch refuses, the release the PICKUP the
    // swap answers — the carried's word the seat wears, the seat's
    // NONE the carried retires)
    app.chest_slots[0] = (potion, 1);
    app.chest_row_potion_kinds[0] = none;
    app.chest_carried = (potion, 3);
    app.chest_carried_potion_kind = night;
    app.qc_arm_chest(gx + 8, gy + 8, 0);
    app.qc_paint_chest(gx + 8, gy + 8);
    assert!(
        !app.qc_slots.contains(&0),
        "the kindless seat the paint refuses"
    );
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(0);
    assert_eq!(app.chest_slots[0], (potion, 3), "the swap the seat wears");
    assert_eq!(
        app.chest_row_potion_kinds[0], night,
        "the carried's own word the seat wears"
    );
    assert_eq!(
        app.chest_carried,
        (potion, 1),
        "the seat's own stack the carried wears"
    );
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the seat's own NONE the carried retires"
    );
    // ARM E — THE RIGHT DRAG'S OWN NEVER-SWAP (the refused seat the
    // right release the place-ONE lane serves — the mismatched pair
    // the silence keeps, the stacks the own words wear)
    app.chest_slots[0] = (potion, 1);
    app.chest_row_potion_kinds[0] = night;
    app.chest_carried = (potion, 2);
    app.chest_carried_potion_kind = swift;
    app.qc_arm_chest(gx + 8, gy + 8, 1);
    app.qc_paint_chest(gx + 8, gy + 8);
    assert!(
        !app.qc_slots.contains(&0),
        "the right drag the paint refuses"
    );
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(1);
    assert_eq!(
        app.chest_slots[0],
        (potion, 1),
        "the never-swap law the seat keeps"
    );
    assert_eq!(
        app.chest_carried,
        (potion, 2),
        "the never-swap law the carried keeps"
    );
    assert_eq!(
        app.chest_row_potion_kinds[0], night,
        "the seat's own word the silence keeps"
    );
    assert_eq!(
        app.chest_carried_potion_kind, swift,
        "the carried's own word the silence keeps"
    );
    // ARM F — THE DIRT PAIR'S OWN PLAIN MERGE (the non-potion ids the
    // NONE words the paint joins, the merge the whole carried lays,
    // the emptied carried the kind tail retires)
    app.chest_slots[0] = (dirt, 10);
    app.chest_row_potion_kinds[0] = none;
    app.chest_carried = (dirt, 5);
    app.chest_carried_potion_kind = none;
    app.qc_arm_chest(gx + 8, gy + 8, 0);
    app.qc_paint_chest(gx + 8, gy + 8);
    assert!(app.qc_slots.contains(&0), "the dirt pair the paint joins");
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(0);
    assert_eq!(app.chest_slots[0], (dirt, 15), "the merge the seat lays");
    assert_eq!(app.chest_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the emptied carried the kind tail retires"
    );
}

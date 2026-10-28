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
    // intermediate beats refuse)
    app.update();
    app.update();
    assert_eq!(
        app.brewing_stores
            .get(&stand_cell)
            .map(|s| s[crate::world::BREWING_SLOT_FUEL]),
        Some((blaze_powder, 1)),
        "the cooldown the second push refuses"
    );
    // THE EIGHTH TICK'S OWN SERVE (the six more updates the +8 cadence
    // completes — the second blaze the same beat lands)
    for _ in 0..6 {
        app.update();
    }
    assert_eq!(
        app.brewing_stores
            .get(&stand_cell)
            .map(|s| s[crate::world::BREWING_SLOT_FUEL]),
        Some((blaze_powder, 2)),
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
        Some((blaze_powder, 2)),
        "the fuel slot the two blaze keeps"
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

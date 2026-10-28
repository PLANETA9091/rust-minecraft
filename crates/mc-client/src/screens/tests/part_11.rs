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

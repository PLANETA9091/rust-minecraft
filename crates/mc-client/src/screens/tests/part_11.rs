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
        assert_eq!(snow_shards, 8, "the bipush-8 loop the case speaks ({snow_shards})");
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
            app.inventory.items[0], (dirt, 4),
            "the hotbar row the write-through speaks"
        );
        assert_eq!(app.chest_mirror[27], (dirt, 4), "the mirror row the sync re-banks");
        assert_eq!(app.hotbar_slots[0], (dirt, 4), "the HUD row the sync follows");
        assert_eq!(app.chest_carried, (0, 0), "the exact split the cursor empties");
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
        assert!(app.qc_slots.is_empty(), "the stone seat the different carried refuses");
        app.on_left_release();
        assert_eq!(app.chest_carried, (stone, 5), "the deferred place the swap speaks");
        assert_eq!(app.chest_slots[4], (dirt, 8), "the stone the hand took");
        // THE BETWEEN-SLOTS PRESS THE ARM SKIPS (the k == -1 silence —
        // the release the un-armed walk never enters)
        app.chest_carried = (dirt, 4);
        app.on_press(panel.0 + 170, panel.1 + 20);
        assert!(!app.qc_armed, "the between press the arm never speaks");
        app.on_left_release();
        assert_eq!(app.chest_carried, (dirt, 4), "the un-armed release the silence keeps");
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
        assert_eq!(app.chest_carried, (dirt, 4), "the between release the no-op keeps");
        // THE OUTSIDE RELEASE THE THROW SPEAKS (the -999 PICKUP the
        // local throw wears — the whole carried the primary drops: the
        // cursor the throw empties, the headless menus the body-less
        // silence the drop walk's own guard keeps)
        app.on_press(gx + 9, gy + 9);
        assert!(app.qc_armed, "the drag armed");
        app.on_mouse_move(panel.0 + 200, panel.1 + 20); // outside — no seat
        assert!(app.qc_slots.is_empty(), "the outside motion paints nothing");
        app.on_left_release();
        assert_eq!(app.chest_carried, (0, 0), "the whole carried the throw drops");
        assert!(
            app.drops.is_empty(),
            "the headless body the drop walk's own guard keeps silent"
        );
    }



    /// THE SHULKER'S OWN PLACEMENT CHAIN (the autopush-148 round —
    /// the kind mint v0.3.876 banked the walk the generic
    /// placeable_block lane already answers): the held shulker_box
    /// item the offline press places, the SHULKER_BOX kind the
    /// NAME_OF row 778 speaks, the FLOOR LOOK the UP byte the
    /// dispenser family's own getStateForPlacement law stamps
    /// (ShulkerBoxBlock the same setValue(FACING,
    /// nearestLooking.getOpposite()) walk the six-direction byte
    /// wears), the consume(1) the stack rides, the dropper's front
    /// read the placed box the next beat answers.
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
                crate::world::DISPENSER_FACING_UP,
                "the floor look the UP byte the shulker stamp wears"
            );
        }
        assert_eq!(
            app.inventory.items[0],
            (shulker_id, 3),
            "the consume(1) the stack rides"
        );
    }

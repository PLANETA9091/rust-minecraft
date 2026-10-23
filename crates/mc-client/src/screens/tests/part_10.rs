use super::*;


    /// THE REMAINDER'S OWN STORE ARM (javap 26.2 ResultSlot.onTake
    /// pcs 230-255 — the third way the taller milk stack wears): the
    /// cell keeps its own milk (the decrement's honest tail), the
    /// bucket the player's store the Inventory.add walk lands, the
    /// HUD row the every-mutation law syncs, and the FULL store the
    /// player.drop the body drops speak (the drops vec the spawn
    /// names).
    #[test]
    fn the_cake_multimilk_the_store_adds_the_buckets() {
        let id = |name: &str| mc_registry::item_id_by_name(name).unwrap();
        let (milk, sugar, egg, wheat, cake) = (
            id("milk_bucket"),
            id("sugar"),
            id("egg"),
            id("wheat"),
            id("cake"),
        );
        let bucket = id("bucket");
        let stone = id("stone");
        let zero = (0, 0);
        // THE TALLER STACKS (2 milk per cell — the third arm the
        // decremented milk's own tail answers)
        let grid: [(i32, i32); 9] = [
            (milk, 2), (milk, 2), (milk, 2),
            (sugar, 1), (egg, 1), (sugar, 1),
            (wheat, 1), (wheat, 1), (wheat, 1),
        ];
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_crafting_table();
        app.table_grid = grid;
        app.refresh_table_result();
        app.click_table_result(0);
        assert_eq!(app.inv_carried, (cake, 1), "the whole result taken");
        assert_eq!(app.table_grid[0], (milk, 1), "the cell keeps its own milk");
        assert_eq!(app.table_grid[1], (milk, 1), "the second cell keeps");
        assert_eq!(app.table_grid[2], (milk, 1), "the third cell keeps");
        assert_eq!(
            app.table_result, (0, 0),
            "the milk-and-empty grid the recipe refuses"
        );
        // THE STORE'S OWN LANDING (the add walk's merge-first law —
        // the SELECTED hotbar slot's own stack the remaining-space
        // scan fills first, three calls one stack)
        assert_eq!(app.inventory.items[0], (bucket, 3), "the three buckets merge");
        // THE HUD ROW (the every-mutation law syncs)
        assert_eq!(app.hotbar_slots[0], (bucket, 3), "the row the add syncs");
        // THE DROP ARM (the FULL store — the add's FALSE the
        // player.drop the body drops speak): no slot lands, three
        // bucket drops the spawn names
        let mut full = App::new(3);
        full.enter_world();
        full.spawn_offline_body();
        full.open_crafting_table();
        full.inventory.items = [(stone, 64); 36];
        full.table_grid = grid;
        full.refresh_table_result();
        full.click_table_result(0);
        assert_eq!(full.inv_carried, (cake, 1), "the whole result taken");
        assert!(
            full.inventory.items.iter().all(|s| s.0 == stone),
            "the full store the bucket never lands in"
        );
        let dropped = full
            .drops
            .iter()
            .filter(|d| d.item == bucket && d.count == 1)
            .count();
        assert_eq!(dropped, 3, "the three buckets the body drops speak");
        let _ = zero;
    }


    /// THE CAKE'S OWN WIRE WALK (the window-1 belief the onTake
    /// answers — the remainders arm the packet's changed map names):
    /// the emptied milk cells the BUCKET pairs wear (the repeated
    /// slot the map's own put replaces — one entry per slot, the
    /// final pair), the spent cells the emptied pairs speak, the
    /// recomputed verdict the result's own entry names, the taller
    /// stack's buckets the BELIEF's own rows the add walk lands (the
    /// selected hotbar head first), the HUD row the sync follows.
    #[test]
    fn the_wire_table_cake_the_remainders_rid() {
        use crate::world_view::{ContainerCommand, ContainerInput, LiveContainer};
        let id = |name: &str| mc_registry::item_id_by_name(name).unwrap();
        let (milk, sugar, egg, wheat, cake) = (
            id("milk_bucket"),
            id("sugar"),
            id("egg"),
            id("wheat"),
            id("cake"),
        );
        let bucket = id("bucket");
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        // THE SINGLES GRID (1 milk per cell — the setItem arm the
        // cells' own pairs wear, no add the store keeps)
        let table = LiveContainer {
            id: 9,
            menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
            slots: {
                let mut s = vec![(0i32, 0i32); 46];
                s[0] = (cake, 1); // the belief's own preview
                s[1] = (milk, 1);
                s[2] = (milk, 1);
                s[3] = (milk, 1);
                s[4] = (sugar, 1);
                s[5] = (egg, 1);
                s[6] = (sugar, 1);
                s[7] = (wheat, 1);
                s[8] = (wheat, 1);
                s[9] = (wheat, 1);
                s
            },
            carried: (0, 0),
            state_id: 3,
            bundle_rows: Vec::new(),
            data_slots: Vec::new(),
            title_nbt: Vec::new(),
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
        app.table_panel = inventory_panel_origin(640, 360);
        let (px, py) = inventory_panel_origin(640, 360);
        app.on_press(px + 124 + 1 + 8, py + 35 + 1 + 8);
        match click_rx.try_recv().expect("the result click rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 9, "the window's own id");
                assert_eq!(c.slot, 0, "the result slot's own take");
                assert_eq!(c.input, ContainerInput::Pickup);
                assert_eq!(c.carried, (cake, 1), "the whole result taken");
                assert_eq!(
                    c.changed,
                    vec![
                        (1, (bucket, 1)),
                        (2, (bucket, 1)),
                        (3, (bucket, 1)),
                        (4, (0, 0)),
                        (5, (0, 0)),
                        (6, (0, 0)),
                        (7, (0, 0)),
                        (8, (0, 0)),
                        (9, (0, 0)),
                        (0, (0, 0)),
                    ],
                    "the buckets' pairs the map's own put replaces, the \
                     spent cells the emptied pairs speak, the verdict the \
                     result's entry names"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        assert_eq!(app.live_table_slots[1], (bucket, 1), "the belief's cell wears");
        assert_eq!(app.live_table_slots[0], (0, 0), "the verdict empty");
        // THE TALLER STACKS (2 milk per cell — the third arm the
        // BELIEF's own rows the add walk lands): the cells keep their
        // milk, the buckets the selected hotbar head takes, the HUD
        // row the sync follows
        let table2 = LiveContainer {
            id: 11,
            menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
            slots: {
                let mut s = vec![(0i32, 0i32); 46];
                s[0] = (cake, 1);
                s[1] = (milk, 2);
                s[2] = (milk, 2);
                s[3] = (milk, 2);
                s[4] = (sugar, 1);
                s[5] = (egg, 1);
                s[6] = (sugar, 1);
                s[7] = (wheat, 1);
                s[8] = (wheat, 1);
                s[9] = (wheat, 1);
                s
            },
            carried: (0, 0),
            state_id: 5,
            bundle_rows: Vec::new(),
            data_slots: Vec::new(),
            title_nbt: Vec::new(),
        };
        app.live_view_updated(air_net_world(2, 2), None, None, Some(table2));
        app.on_press(px + 124 + 1 + 8, py + 35 + 1 + 8);
        match click_rx.try_recv().expect("the second take rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.carried, (cake, 1), "the whole result taken");
                assert_eq!(
                    c.changed,
                    vec![
                        (1, (milk, 1)),
                        (37, (bucket, 3)),
                        (2, (milk, 1)),
                        (3, (milk, 1)),
                        (4, (0, 0)),
                        (5, (0, 0)),
                        (6, (0, 0)),
                        (7, (0, 0)),
                        (8, (0, 0)),
                        (9, (0, 0)),
                        (0, (0, 0)),
                    ],
                    "the cells keep their milk, the buckets the selected row's \
                     own stack merges (the CraftingMenu's own hotbar head 37 \
                     the scan keys — the repeated slot the map's own put \
                     replaces, the FINAL pair one entry), the verdict the \
                     result's entry names"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        assert_eq!(app.live_table_slots[37], (bucket, 3), "the row's own merge");
        assert_eq!(app.live_table_slots[1], (milk, 1), "the cell keeps its milk");
        assert_eq!(app.hotbar_slots[0], (bucket, 3), "the HUD row the sync follows");
    }


    /// THE TABLE SCREEN'S OWN LANES (the open's fresh grid, the cell
    /// clicks' place/take walks, the result's recompute, the close's
    /// clearContainer ride-home): the transient menu the vanilla use
    /// speaks per open, the contents the alive player's
    /// placeItemBackInInventory answers.
    #[test]
    fn the_table_screen_clicks_speak_the_grid_lanes() {
        let id = |name: &str| mc_registry::item_id_by_name(name).unwrap();
        let (planks, stick) = (id("oak_planks"), id("stick"));
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        // THE OPEN: the fresh grid, the Crafting screen
        app.open_crafting_table();
        assert_eq!(app.screen, ScreenKind::Crafting, "the menu opens");
        assert!(
            app.table_grid.iter().all(|c| c.1 == 0),
            "the transient menu's own fresh grid"
        );
        app.table_panel = inventory_panel_origin(640, 360);
        let (px, py) = inventory_panel_origin(640, 360);
        // THE CELL CLICKS place the carried whole (the predict_pickup
        // walk the store slots speak) and the recompute follows — the
        // sticks recipe ([P / P] the left column keeps). THE
        // QUICKCRAFT KEYING (the 26.2 mouseClicked's own law): the
        // carried press ARMS the drag (no place on press), and the
        // empty-paint release the deferred place speaks — the same
        // PICKUP walk the press used to ride, the release's own lane
        // answers.
        app.inv_carried = (planks, 6);
        app.on_mouse_move(px + 30 + 1 + 8, py + 17 + 1 + 8);
        app.on_press(px + 30 + 1 + 8, py + 17 + 1 + 8); // cell 0
        app.on_left_release();
        assert_eq!(app.table_grid[0], (planks, 6), "the whole stack places");
        assert_eq!(app.inv_carried, (0, 0), "the hand emptied");
        app.inv_carried = (planks, 6);
        app.on_mouse_move(px + 30 + 1 + 8, py + 17 + 18 + 1 + 8);
        app.on_press(px + 30 + 1 + 8, py + 17 + 18 + 1 + 8); // cell 3 (row 1)
        app.on_left_release();
        assert_eq!(app.table_grid[3], (planks, 6), "the second cell places");
        assert_eq!(app.table_result, (stick, 4), "the sticks recipe speaks");
        // THE RESULT TAKE (the mayPlace-false slot's own walk): the
        // carried receives, the cells one off, the recipe RE-SPEAKS
        let (rx, ry) = table_craft_origin(app.table_panel, 9);
        app.on_press(rx + 1 + 8, ry + 1 + 8);
        assert_eq!(app.inv_carried, (stick, 4), "the whole result taken");
        assert_eq!(app.table_grid[0], (planks, 5), "the cell one off");
        assert_eq!(app.table_result, (stick, 4), "the recipe re-speaks");
        // THE SECOND CRAFT the merge arm serves (the same-item carried
        // the carried's space caps — 4 + 4 = 8). The carried cursor
        // the drag arms — the release the deferred place serves (the
        // result's own take the merge arm answers)
        app.on_mouse_move(rx + 1 + 8, ry + 1 + 8);
        app.on_press(rx + 1 + 8, ry + 1 + 8);
        app.on_left_release();
        assert_eq!(app.inv_carried, (stick, 8), "the merge arm answers");
        assert_eq!(app.table_grid[0], (planks, 4), "the cells keep spending");
        // THE CLOSE (the clearContainer walk): the grid's 8 planks and
        // the carried's 8 sticks ride home, the store keeps them
        app.close_crafting_table();
        assert_eq!(app.screen, ScreenKind::World, "the menu closes");
        assert_eq!(app.inv_carried, (0, 0), "the carried rode home");
        assert!(
            app.table_grid.iter().all(|c| c.1 == 0),
            "the grid emptied (the removeItemNoUpdate walk)"
        );
        let planks_in_store: i32 = app
            .inventory
            .items
            .iter()
            .filter(|(i, _)| *i == planks)
            .map(|(_, c)| c)
            .sum();
        let sticks_in_store: i32 = app
            .inventory
            .items
            .iter()
            .filter(|(i, _)| *i == stick)
            .map(|(_, c)| c)
            .sum();
        assert_eq!(planks_in_store, 8, "the grid contents rode home");
        assert_eq!(sticks_in_store, 8, "the carried rode home");
        assert_eq!(app.table_result, (0, 0), "the result slot's own reset");
        // THE E KEY'S OWN LANE (the toggle the container screen
        // answers): the reopened table the key closes back
        app.open_crafting_table();
        app.on_inventory_key();
        assert_eq!(app.screen, ScreenKind::World, "the E key closes the table");
        // THE ESCAPE'S OWN LANE mirrors (the open + the on_escape walk)
        app.open_crafting_table();
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::World, "the Escape closes the table");
    }


    /// THE TABLE'S OWN USE LANE (javap 26.2 GameType.useItemOn — the
    /// block.use dispatch the item.useOn walk precedes): the
    /// non-sneaking right-press on the placed table the CONSUME
    /// verdict opens the 3x3 screen, the SNEAKING press the gate
    /// skips — the item lanes fall through and the held block PLACES.
    #[test]
    fn the_table_screen_opens_on_the_use_and_the_sneak_bypasses() {
        let mut app = App::new(3);
        app.enter_world();
        // NO BODY (the spawn's own box the placement's entity law
        // refuses — the placement cell the straight-down ray names IS
        // the standing cell): the bare world the vanilla's own
        // no-intersection guard keeps honest
        app.offline_held_updated(Some("item/dirt"));
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        // THE SNEAK BYPASS FIRST (the isShiftKeyDown gate the vanilla's
        // own arm keeps): the shift-down press the table-use walk
        // never arms — the held dirt the placement lane places on the
        // surface
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        app.shift_down = true;
        assert!(app.on_right_press(0, 0), "the sneak placement consumed");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::DIRT,
            "the dirt placed (the bypass spoke)"
        );
        assert_eq!(app.screen, ScreenKind::World, "no screen opened on the sneak");
        app.shift_down = false;
        // THE USE (the non-sneaking press on the table cell): the
        // interactive arm opens the 3x3 screen and CONSUMES the press
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, top, c, crate::world::CRAFTING_TABLE);
        assert!(app.on_right_press(0, 0), "the table's use consumes");
        assert_eq!(app.screen, ScreenKind::Crafting, "the 3x3 screen opened");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::CRAFTING_TABLE,
            "the world cell untouched"
        );
        // THE ESCAPE back — the world screen stands
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::World);
    }


    /// THE SHIFT-CLICK BATCH (the doClick QUICK_MOVE arm over the
    /// result slot — the quickMoveStack loop the while-same-item guard
    /// keeps): the whole log stack crafts in ONE press, the store the
    /// moveItemStackTo(stack, 9, 45, TRUE) walk fills — the merge pass
    /// over the whole range FIRST (the hotbar END-first the menu's own
    /// slot order speaks), the first-empty pass after — and the batch
    /// NEVER touches the carried stack (the quickMoveStack walk the
    /// carried ignores). The empty result the silence keeps, the wire
    /// session the lane's own early return.
    #[test]
    fn the_shift_click_crafts_the_whole_log_into_the_store() {
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // the EMPTY result — the shift's silence (no store move, no drop)
        app.shift_down = true;
        app.on_press(394, 133);
        assert!(app.drops.is_empty(), "the empty result never spoke");
        assert_eq!(app.inventory.items[8], (0, 0), "the store whole");
        // the stage: five logs in the cell, a partial plank stack in
        // the hotbar END (the merge-first order's own witness), a
        // carried stack the batch must never touch
        app.craft_grid[0] = (log, 5);
        app.refresh_craft_result();
        app.inventory.items[8] = (planks, 62);
        app.inv_carried = (log, 1);
        app.hotbar_slots = app.inventory.hotbar();
        // THE BATCH — one shift-click crafts the whole cell (the held
        // log the drag arm arms — the release's own deferred place the
        // shift's QUICK_MOVE lane serves: the paint the result seat's
        // mayPlace refusal never fills, the empty-painted release the
        // place walks)
        app.on_press(394, 133);
        app.on_mouse_move(394, 133);
        app.on_left_release();
        // craft 1: the merge pass took 2 into store 8 (62 -> 64), the
        // first-empty pass landed the rest in store 7; crafts 2..5
        // merged into store 7 (the hotbar-END-first law)
        assert_eq!(app.inventory.items[8], (planks, 64), "the merge first");
        assert_eq!(app.inventory.items[7], (planks, 18), "the rest merged there");
        for s in (0..7).chain(9..36) {
            assert_eq!(app.inventory.items[s], (0, 0), "slot {s} untouched");
        }
        assert_eq!(app.craft_grid[0], (0, 0), "the cell spent");
        assert_eq!(app.craft_result, (0, 0), "the spent grid's EMPTY verdict");
        assert_eq!(app.inv_carried, (log, 1), "the carried never rode");
        assert!(app.drops.is_empty(), "nothing the store could not hold");
        assert_eq!(app.hotbar_slots[7], (planks, 18), "the HUD synced");
        assert_eq!(app.hotbar_slots[8], (planks, 64), "the HUD synced");
        // THE WIRE — the lane's own early return keeps the shift silent
        app.world_is_network = true;
        app.on_press(394, 133);
        assert_eq!(app.inventory.items[7], (planks, 18), "the wire refusal holds");
    }


    /// THE LEFTOVER DROP (the quickMoveStack index-0 tail — the 378-385
    /// bytecode): the stack part the store never holds rides
    /// `player.drop(stack, false)` — the AIMED throw the shared lane
    /// keeps — and the batch breaks on the nothing-moved EMPTY the
    /// NEXT iteration speaks (the recipe still matching, the store
    /// full, the loop the EMPTY verdict ends).
    #[test]
    fn the_shift_click_drops_the_leftover_when_the_store_cannot_hold() {
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // the store FULL of planks but ONE room in the hotbar end
        for s in 0..36 {
            app.inventory.items[s] = (planks, 64);
        }
        app.inventory.items[8] = (planks, 63);
        app.craft_grid[0] = (log, 2); // two crafts speak
        app.refresh_craft_result();
        // THE BATCH: craft 1 merges ONE into store 8 (63 -> 64), the
        // leftover 3 rides the aimed drop; craft 2 the nothing-moved
        // EMPTY breaks — the result still speaks, the cell holds one
        app.shift_down = true;
        app.on_press(394, 133);
        assert_eq!(app.inventory.items[8], (planks, 64), "the one room filled");
        assert_eq!(app.drops.len(), 1, "the leftover dropped");
        let d = app.drops.last().unwrap();
        assert_eq!(d.item, planks, "the result's own item");
        assert_eq!(d.count, 3, "the store's leftover (4 - 1)");
        assert_eq!(d.pickup_delay, 40, "the aimed throw's own delay");
        assert_eq!(app.craft_grid[0], (log, 1), "ONE craft's decrement");
        assert_eq!(app.craft_result, (planks, 4), "the recipe still speaks");
        assert_eq!(app.hotbar_slots[8], (planks, 64), "the HUD synced");
    }


    /// THE INVENTORY'S OWN Q (the AbstractContainerScreen.keyPressed's
    /// keyDrop arm — the plain form): the HOVERED slot's ONE item flies
    /// the AIMED look throw, the slot decrements, NO swing (the
    /// keyPressed chain the success arm never names), the HUD hotbar
    /// row syncs, the container stays open.
    #[test]
    fn the_inventory_q_throws_one_from_the_hovered_slot() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // store 5 = the hotbar row's own rect (the slot origin walk):
        // panel (232, 97) -> slot 5 origin (330, 239), the cursor inside
        app.inventory.items[5] = (62, 7);
        app.mouse_pos = Some((336, 246));
        let drops_before = app.drops.len();
        app.on_inventory_drop_key(false);
        // safeTake(1, MAX): exactly ONE off the hovered slot
        assert_eq!(app.inventory.items[5], (62, 6), "one item gone");
        assert_eq!(app.drops.len(), drops_before + 1, "one drop spawned");
        let d = app.drops.last().unwrap();
        assert_eq!(d.item, 62);
        assert_eq!(d.count, 1, "ONE item");
        assert_eq!(d.pickup_delay, 40, "the throw's own delay");
        let b = app.body.as_ref().unwrap();
        assert_eq!(d.x, b.x, "the body's x");
        assert_eq!(d.z, b.z, "the body's z");
        assert_eq!(
            d.y,
            b.y + b.eye_height() - 0.30000001192092896,
            "the eye minus the 0.30000001192092896 lift"
        );
        // the AIMED look throw's own shape (the createItemStackToDrop
        // folds — the 0.3 forward, the 0.1 lift within the jitter band)
        let speed = (d.vx * d.vx + d.vz * d.vz).sqrt();
        assert!(
            (speed - 0.3).abs() <= 0.03,
            "the horizontal speed the 0.3 fold speaks (speed={speed})"
        );
        assert!(
            d.vy > 0.05 && d.vy < 0.15,
            "the 0.1 base lift within the ±0.05 jitter band (vy={vy})",
            vy = d.vy
        );
        // the keyPressed chain names NO swing — the handleKeybinds
        // success arm the container walk never rides
        assert!(!app.swing.swinging, "no swing on the container's Q");
        // the HUD hotbar row synced (store 5 IS the hotbar row)
        assert_eq!(app.hotbar_slots[5], (62, 6), "the HUD synced");
        // the container stays open
        assert_eq!(app.screen, ScreenKind::Inventory, "the screen stays");
    }


    /// THE INVENTORY'S CTRL+Q (the keyPressed hasControlDown ? 1 : 0
    /// button int — the doClick's `count = button == 0 ? 1 :
    /// slot.getItem().getCount()` arm): the WHOLE hovered stack flies,
    /// the slot empties — and a MAIN-ROW slot the hotbar never
    /// contains answers too (the geometry walk beyond the hotbar row).
    #[test]
    fn the_inventory_ctrl_q_throws_the_whole_hovered_stack() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // store 12 = the main row's own rect: i = 3 -> panel-relative
        // (62, 84) -> absolute (294, 181), the cursor inside
        app.inventory.items[12] = (63, 12);
        app.mouse_pos = Some((300, 190));
        let drops_before = app.drops.len();
        app.on_inventory_drop_key(true);
        assert_eq!(app.inventory.items[12], (0, 0), "the slot emptied");
        assert_eq!(app.drops.len(), drops_before + 1, "one drop spawned");
        let d = app.drops.last().unwrap();
        assert_eq!(d.item, 63);
        assert_eq!(d.count, 12, "the WHOLE stack");
        assert_eq!(d.pickup_delay, 40);
        assert!(!app.swing.swinging, "no swing");
        // the main row is NOT the hotbar — the HUD row untouched
        assert_eq!(app.hotbar_slots[app.selected_hotbar], (0, 0), "the hotbar untouched");
        // the doClick's button==1 repeat loop the one-shot whole-take
        // answers on plain stacks — safeTake(count, MAX) empties in one
        // call: a second Ctrl+Q on the now-empty slot is silence
        app.on_inventory_drop_key(true);
        assert_eq!(app.drops.len(), drops_before + 1, "still one drop");
    }


    /// THE INVENTORY Q'S OWN GATES (the keyPressed chain + the doClick
    /// THROW arm's own pair): the EMPTY hovered slot silence (the
    /// hasItem() gate), the CARRIED cursor silence (the
    /// getCarried().isEmpty() gate — the held stack the Q never
    /// throws), the hoveredSlot-null silence (the mouse off the slots —
    /// the gate the keyPressed bytecode 39-53 speaks), the world-screen
    /// cross silence (this lane IS the container's own), and the wire
    /// session refusal (the store the server owns).
    #[test]
    fn the_inventory_q_refuses_the_empty_and_the_carried_and_the_wire() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        app.inventory.items[5] = (62, 7);
        let drops_before = app.drops.len();
        // the hoveredSlot NULL (the cursor between the slots — the
        // mannequin's own region): the keyPressed gate returns false
        app.mouse_pos = Some((332, 157));
        app.on_inventory_drop_key(false);
        assert_eq!(app.drops.len(), drops_before, "the null hover is silence");
        // the world screen CROSS — this lane the world Q's chain never
        // doubles (the screen gate holds)
        app.screen = ScreenKind::World;
        app.mouse_pos = Some((336, 246));
        app.on_inventory_drop_key(false);
        assert_eq!(app.drops.len(), drops_before, "the world screen cross is silence");
        app.open_inventory(); // back on the container screen
        // the EMPTY hovered slot (the hasItem() gate — slot 0 the store
        // never carried: origin (240, 239), the cursor inside)
        app.mouse_pos = Some((246, 246));
        app.on_inventory_drop_key(false);
        assert_eq!(app.drops.len(), drops_before, "the empty hover is silence");
        // the CARRIED cursor (the getCarried().isEmpty() gate — the
        // doClick's own first test; the cursor back over the FULL slot 5
        // so the silence IS the carried gate's own, not the empty slot's)
        app.mouse_pos = Some((336, 246));
        app.inv_carried = (62, 5);
        app.on_inventory_drop_key(false);
        assert_eq!(app.drops.len(), drops_before, "the carried cursor is silence");
        assert_eq!(app.inventory.items[5], (62, 7), "the store whole");
        assert_eq!(app.inv_carried, (62, 5), "the carry untouched");
        // the Ctrl form keeps every gate
        app.on_inventory_drop_key(true);
        assert_eq!(app.drops.len(), drops_before, "still silence");
        // the WIRE session — the store the server owns
        app.inv_carried = (0, 0);
        app.mouse_pos = Some((336, 246));
        app.world_is_network = true;
        app.on_inventory_drop_key(false);
        assert_eq!(app.drops.len(), drops_before, "the wire refusal holds");
        assert_eq!(app.inventory.items[5], (62, 7), "the wire store whole");
    }


    /// THE SURVIVAL SCREEN'S OWN DIGIT SWAP (the checkHotbarKeyPressed
    /// walk the InventoryMenu's seats answer — the doClick SWAP table's
    /// own arms): CASE B the hovered stack rides the empty button row
    /// AS-IS (the store cell EMPTIES, the HUD row synced), the
    /// both-empty press the consumed silence keeps, the hoveredSlot
    /// null the false verdict wears.
    #[test]
    fn the_inventory_digits_the_swap_table_wears() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // CASE B: the hovered store stack rides the EMPTY hotbar row 0
        app.inventory.items[10] = (62, 7);
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 10);
        app.mouse_pos = Some((ox + 8, oy + 8));
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.inventory.items[10], (0, 0), "the store empties");
        assert_eq!(app.inventory.items[0], (62, 7), "the row rides AS-IS");
        assert_eq!(app.hotbar_slots[0], (62, 7), "the HUD synced");
        assert_eq!(app.screen, ScreenKind::Inventory, "the screen stays");
        assert_eq!(app.inv_carried, (0, 0), "the cursor stays empty");
        // the BOTH-EMPTY press (pcs 1130-1146): the consumed silence —
        // the row 0's stack rode away, the hover re-armed on store 11
        // (both empty)
        let (ox2, oy2) = inventory_slot_origin(app.inv_panel, 11);
        app.mouse_pos = Some((ox2 + 8, oy2 + 8));
        assert!(app.on_inventory_swap_key(1), "the silence CONSUMES");
        assert_eq!(app.inventory.items[1], (0, 0), "the row whole");
        assert_eq!(app.inventory.items[11], (0, 0), "the cell whole");
        // the hoveredSlot NULL (the cursor off the slots): the false
        // verdict the world chain answers
        app.mouse_pos = Some((332, 157));
        assert!(!app.on_inventory_swap_key(2), "the null hover refuses");
    }


    /// THE CASE C'S OWN SPLIT LAW (the getMaxStackSize walk — min(64,
    /// the item's own)): the row 20 eggs over the EMPTY hovered cell —
    /// the cell takes the max 16, the row the remainder 4 keeps; the
    /// count <= max arm the row EMPTIES, the cell the whole stack
    /// wears.
    #[test]
    fn the_inventory_digit_splits_the_max_overflow() {
        let egg = crate::mob::wire_id("egg").expect("the egg's own id");
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // the SPLIT arm: the row 20 eggs (the test's own state — the
        // placement law never carries 20 eggs), the hover the empty
        // store 12
        app.inventory.items[0] = (egg, 20);
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 12);
        app.mouse_pos = Some((ox + 8, oy + 8));
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.inventory.items[12], (egg, 16), "the cell the max takes");
        assert_eq!(app.inventory.items[0], (egg, 4), "the row the remainder keeps");
        // the WHOLE arm: the row 12 stone over the empty store 13
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        app.inventory.items[0] = (stone, 12);
        let (ox2, oy2) = inventory_slot_origin(app.inv_panel, 13);
        app.mouse_pos = Some((ox2 + 8, oy2 + 8));
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.inventory.items[13], (stone, 12), "the cell the whole wears");
        assert_eq!(app.inventory.items[0], (0, 0), "the row empties");
    }


    /// THE CASE D'S OWN TRADE (the count <= max arm — the straight
    /// swap the pcs 1358-1379 walk speaks): the two stacks trade —
    /// the row the hovered stack takes, the cell the row's stack
    /// wears; the hotbar row's own hover the REARRANGE arm answers
    /// (store 5 hovered, the digit 2 — the items' 5 and 2 trade).
    #[test]
    fn the_inventory_digit_the_straight_trade_wears() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        // the main-row trade: the row 5 stone, the hover 3 dirt
        app.inventory.items[1] = (stone, 5);
        app.inventory.items[20] = (dirt, 3);
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 20);
        app.mouse_pos = Some((ox + 8, oy + 8));
        assert!(app.on_inventory_swap_key(1), "the digit consumes");
        assert_eq!(app.inventory.items[1], (dirt, 3), "the row the hovered takes");
        assert_eq!(app.inventory.items[20], (stone, 5), "the cell the row's wears");
        // THE HOTBAR REARRANGE (the store's own row the hover reads —
        // menu 36..44 the walk's store index folds): store 5 hovered,
        // the digit 2 — items[5] and items[2] trade
        app.inventory.items[5] = (62, 1);
        app.inventory.items[2] = (63, 2);
        let (ox2, oy2) = inventory_slot_origin(app.inv_panel, 5);
        app.mouse_pos = Some((ox2 + 8, oy2 + 8));
        assert!(app.on_inventory_swap_key(2), "the digit consumes");
        assert_eq!(app.inventory.items[2], (62, 1), "the row the hover's takes");
        assert_eq!(app.inventory.items[5], (63, 2), "the cell the row's wears");
        assert_eq!(app.hotbar_slots[2], (62, 1), "the HUD synced");
    }


    /// THE CASE D'S OWN SPLIT + ADD (the count > max arm — the pcs
    /// 1315-1355 walk): the cell takes max, the row the remainder
    /// keeps, the ORIGINAL hovered stack the Inventory.add rides —
    /// the add's own merge-then-first-empty law answers, the FALSE
    /// the player.drop speaks (the leftover the body's own lane
    /// spawns), the TRUE the silent landing wears.
    #[test]
    fn the_inventory_digit_the_split_adds_back_or_drops() {
        let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // THE DROP ARM: the row 3 helmets (the over-max state the
        // test's own — the count the item's own max 1 exceeds), the
        // hover 30 stone, the store FULL of stone — the split the cell
        // takes 1 helmet, the row the remainder 2 keeps; the 30 stone
        // the add's scan refuses (the stacks the 64-cap fills, the
        // free slot absent) — the drop speaks
        for it in app.inventory.items.iter_mut() {
            *it = (stone, 64);
        }
        app.inventory.items[0] = (helmet, 3);
        app.inventory.items[12] = (stone, 30);
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 12);
        app.mouse_pos = Some((ox + 8, oy + 8));
        let drops_before = app.drops.len();
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.inventory.items[12], (helmet, 1), "the cell the max takes");
        assert_eq!(app.inventory.items[0], (helmet, 2), "the row the remainder keeps");
        assert_eq!(app.drops.len(), drops_before + 1, "the add's FALSE the drop speaks");
        let d = app.drops.last().unwrap();
        assert_eq!(d.item, stone);
        assert_eq!(d.count, 30, "the ORIGINAL hovered stack the drop wears");
        // THE LANDING ARM: the store's slot 30 freed — the 30 stone the
        // first-empty takes whole (the merge finds no stone space — the
        // 64-stacks full, the helmet cells different-id), no drop
        app.inventory.items[0] = (helmet, 3);
        app.inventory.items[12] = (stone, 30);
        app.inventory.items[30] = (0, 0);
        let drops_before = app.drops.len();
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.inventory.items[12], (helmet, 1), "the cell the max takes");
        assert_eq!(app.inventory.items[0], (helmet, 2), "the row the remainder keeps");
        assert_eq!(app.drops.len(), drops_before, "the add's TRUE the silence keeps");
        assert_eq!(app.inventory.items[30], (stone, 30), "the stones the first-empty lands");
    }


    /// THE ARMOR SEAT'S OWN GATES (the ArmorSlot's mayPlace the
    /// equippable_loop_k verdict answers — the creative round's law
    /// the survival seat shares): the matching piece places (the max-1
    /// split the multi-count wears), the mismatch the consumed
    /// silence keeps (the CASE C refusal AND the CASE D both-stay).
    #[test]
    fn the_inventory_digit_the_armor_gates_keep() {
        let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // CASE C the MATCHING piece: the row 2 helmets (the test's own
        // state), the armor cell 0 EMPTY — the cell takes ONE, the row
        // keeps 1
        app.inventory.items[0] = (helmet, 2);
        let (ax, ay) = armor_slot_origin(app.inv_panel, 0);
        app.mouse_pos = Some((ax + 8, ay + 8));
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.inventory.armor[0], (helmet, 1), "the piece places");
        assert_eq!(app.inventory.items[0], (helmet, 1), "the row keeps the rest");
        // CASE C the MISMATCH: the row stone over the EMPTY armor cell
        // 1 — the mayPlace refusal, both stay
        app.inventory.items[0] = (stone, 5);
        let (ax2, ay2) = armor_slot_origin(app.inv_panel, 1);
        app.mouse_pos = Some((ax2 + 8, ay2 + 8));
        assert!(app.on_inventory_swap_key(0), "the refusal CONSUMES");
        assert_eq!(app.inventory.armor[1], (0, 0), "the cell stays empty");
        assert_eq!(app.inventory.items[0], (stone, 5), "the row whole");
        // CASE D the MISMATCH: the row stone over the WORN helmet —
        // both stay
        let (ax3, ay3) = armor_slot_origin(app.inv_panel, 0);
        app.mouse_pos = Some((ax3 + 8, ay3 + 8));
        assert!(app.on_inventory_swap_key(0), "the refusal CONSUMES");
        assert_eq!(app.inventory.armor[0], (helmet, 1), "the worn whole");
        assert_eq!(app.inventory.items[0], (stone, 5), "the row whole");
        // CASE D the STRAIGHT TRADE: the row 1 helmet over the worn
        // helmet — the worn rides the row, the cell the row's wears
        app.inventory.items[0] = (helmet, 1);
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.inventory.armor[0], (helmet, 1), "the cell the row's wears");
        assert_eq!(app.inventory.items[0], (helmet, 1), "the worn the row rides");
    }


    /// THE F KEY'S OWN OFFHAND SWAP (checkHotbarKeyPressed's FIRST arm
    /// — the button 40 the InventoryMenu$1's plain law answers, the
    /// fresh offhand_slot_262.txt dump the mayPlace override the
    /// 26.2 walk never wears): CASE B the hovered stack rides the
    /// EMPTY offhand, the CASE D trade the two stacks swap.
    #[test]
    fn the_inventory_f_swaps_the_offhand() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // CASE B: the hovered store stack rides the EMPTY offhand
        app.inventory.items[14] = (62, 6);
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 14);
        app.mouse_pos = Some((ox + 8, oy + 8));
        assert!(app.on_inventory_swap_key(40), "the F consumes");
        assert_eq!(app.inventory.offhand[0], (62, 6), "the offhand rides");
        assert_eq!(app.inventory.items[14], (0, 0), "the store empties");
        // CASE D: the offhand and the hovered slot trade
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        app.inventory.items[14] = (dirt, 9);
        let (ox2, oy2) = inventory_slot_origin(app.inv_panel, 14);
        app.mouse_pos = Some((ox2 + 8, oy2 + 8));
        assert!(app.on_inventory_swap_key(40), "the F consumes");
        assert_eq!(app.inventory.offhand[0], (dirt, 9), "the offhand the hover's takes");
        assert_eq!(app.inventory.items[14], (62, 6), "the store the offhand's wears");
    }


    /// THE RESULT SEAT'S OWN TAKE (CASE B — the row empty, the
    /// result's mayPickup the recompute's own match verdict speaks):
    /// the 1-log planks the row rides, the craft_on_take the
    /// ingredients' decrement + the recompute answers; the
    /// mayPlace-false seat the CASE C/D arms the silence keeps (the
    /// row's stack the result seat never places, the CASE D both
    /// stay).
    #[test]
    fn the_inventory_digit_the_result_take_consumes() {
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // the grid's 1 log → the result 4 planks (the shapeless single)
        app.craft_grid[0] = (log, 1);
        app.refresh_craft_result();
        assert_eq!(app.craft_result, (planks, 4), "the recipe speaks");
        let (rx, ry) = craft_slot_origin(app.inv_panel, 4);
        app.mouse_pos = Some((rx + 8, ry + 8));
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.inventory.items[0], (planks, 4), "the row the result rides");
        assert_eq!(app.craft_result, (0, 0), "the result empties");
        assert_eq!(app.craft_grid[0], (0, 0), "the log consumed");
        // the CASE D refusal: the result refilled, the row filled —
        // the mayPlace-false seat BOTH stay
        app.craft_grid[0] = (log, 1);
        app.refresh_craft_result();
        assert!(app.on_inventory_swap_key(0), "the refusal CONSUMES");
        assert_eq!(app.craft_result, (planks, 4), "the result whole");
        assert_eq!(app.inventory.items[0], (planks, 4), "the row whole");
        assert_eq!(app.craft_grid[0], (log, 1), "the grid whole");
        // the CASE C refusal: the result cleared, the row filled — the
        // mayPlace refusal the place arm never rides
        app.craft_grid[0] = (0, 0);
        app.refresh_craft_result();
        assert!(app.on_inventory_swap_key(0), "the refusal CONSUMES");
        assert_eq!(app.craft_result, (0, 0), "the result stays empty");
        assert_eq!(app.inventory.items[0], (planks, 4), "the row whole");
    }


    /// THE WALK'S OWN GATE CHAIN (the checkHotbarKeyPressed pcs 0-17
    /// verdicts): the CARRIED cursor the false return keeps, the WIRE
    /// session the store-server-owned refusal wears, the WORLD screen
    /// cross the false keeps, the NO-PANEL silence (the render never
    /// spoke) the false keeps.
    #[test]
    fn the_inventory_digit_the_gates_refuse() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        app.inventory.items[10] = (62, 7);
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 10);
        // the CARRIED gate (pc 0-10)
        app.mouse_pos = Some((ox + 8, oy + 8));
        app.inv_carried = (62, 5);
        assert!(!app.on_inventory_swap_key(0), "the carried refuses");
        assert_eq!(app.inventory.items[10], (62, 7), "the store whole");
        // the WIRE gate (the store the server owns)
        app.inv_carried = (0, 0);
        app.world_is_network = true;
        assert!(!app.on_inventory_swap_key(0), "the wire refuses");
        assert_eq!(app.inventory.items[10], (62, 7), "the wire store whole");
        app.world_is_network = false;
        // the WORLD screen cross — the walk the survival screen alone
        // reads
        app.screen = ScreenKind::World;
        assert!(!app.on_inventory_swap_key(0), "the world cross refuses");
        app.open_inventory();
        // the NO-PANEL gate (the hoveredSlot null arm — the render
        // never spoke)
        app.inv_panel = (0, 0);
        assert!(!app.on_inventory_swap_key(0), "the no-panel refuses");
        assert_eq!(app.inventory.items[10], (62, 7), "the store whole");
    }


    /// THE RIGHT PRESS'S OWN TABLE (the doClick PICKUP button-1 branch
    /// — the press->release drive the type-1 arm law wears: the
    /// carried press the drag arms, the release's own deferred place
    /// the place-ONE serves): the ceiling-half take (5 -> 3 taken, 2
    /// left — the EMPTY cursor the immediate secondary keeps), the
    /// place-one, and the same-item move-one.
    #[test]
    fn the_right_press_places_one() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        app.inventory.items[0] = (62, 5);
        // the ceiling-half take (the empty cursor the press lane keeps)
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 0);
        app.on_right_press(ox + 8, oy + 8);
        assert_eq!(app.inventory.items[0], (62, 2), "2 left");
        assert_eq!(app.inv_carried, (62, 3), "3 taken");
        // the place-one into the empty slot 9 (the press the drag arms,
        // the release the place-ONE serves — the pre-press motion the
        // release position feeds)
        let (px, py) = inventory_slot_origin(app.inv_panel, 9);
        app.on_mouse_move(px + 8, py + 8);
        app.on_right_press(px + 8, py + 8);
        app.on_right_release();
        assert_eq!(app.inventory.items[9], (62, 1), "one placed");
        assert_eq!(app.inv_carried, (62, 2), "two ride");
        // the same-item move-one on the next press->release
        app.on_right_press(px + 8, py + 8);
        app.on_right_release();
        assert_eq!(app.inventory.items[9], (62, 2), "one more moved");
        assert_eq!(app.inv_carried, (62, 1), "one rides");
    }


    /// THE QUICK-MOVE'S FORWARD SECTIONS (the 26.2 quickMoveStack law —
    /// the inventory menu's own order, the chest's fillFromEnd the other
    /// menus speak): the shift-click moves the whole stack to the OTHER
    /// section, the same-item merges first, the FIRST empty slot takes
    /// the remainder (store 0 forward for the hotbar, store 9 forward
    /// for the main rows).
    #[test]
    fn the_quick_move_walks_the_forward_sections() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // the main rows: cobblestone x70 at store 9 (30 over the stack
        // ceiling), the hotbar: cobblestone x40 at store 3 (24 room)
        app.inventory.items[9] = (62, 70);
        app.inventory.items[3] = (62, 40);
        // THE SHIFT-CLICK on the main stack: the hotbar's 40 merges to 64
        // first (pass 1), the first EMPTY hotbar slot (store 0) takes the
        // whole 46 remainder (pass 2 — the empty slot takes the stack up
        // to its ceiling, forward)
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 9);
        app.shift_down = true;
        app.on_press(ox + 8, oy + 8);
        app.shift_down = false;
        assert_eq!(app.inventory.items[3], (62, 64), "the merge ceiling");
        assert_eq!(app.inventory.items[0], (62, 46), "the FIRST empty took the rest");
        assert_eq!(app.inventory.items[9], (0, 0), "the source emptied");
        // THE REVERSE: the hotbar stack quick-moves to the main rows —
        // the same-item merge at store 9's neighbors first, the first
        // empty main slot after
        app.inventory.items[0] = (62, 6);
        app.inventory.items[12] = (62, 60);
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 0);
        app.shift_down = true;
        app.on_press(ox + 8, oy + 8);
        app.shift_down = false;
        assert_eq!(app.inventory.items[12], (62, 64), "the main merge ceiling");
        assert_eq!(app.inventory.items[9], (62, 2), "the FIRST main empty (slot 9) took the rest");
        assert_eq!(app.inventory.items[0], (0, 0), "the hotbar source emptied");
    }


    /// THE CLOSE'S ADD-BACK WALK (the vanilla close's own tail): a
    /// carried stack returns to the store (the same-item merges first,
    /// the first empty slot takes the remainder), the full store's
    /// remainder DROPS at the body's feet (the Player.drop law).
    #[test]
    fn the_close_returns_the_carried_to_the_store() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        // the carried stack with a same-item home: merges into it
        app.inventory.items[5] = (921, 10); // apple
        app.inv_carried = (921, 7);
        app.close_inventory();
        assert_eq!(app.inventory.items[5], (921, 17), "the merge back");
        assert_eq!(app.inv_carried, (0, 0));
        assert_eq!(app.screen, ScreenKind::World, "the screen back");
        // the carried with NO same-item home: the first empty slot
        app.open_inventory();
        app.inv_carried = (926, 4); // diamond
        app.close_inventory();
        assert_eq!(app.inventory.items[0], (926, 4), "the first empty took it");
        // the FULL store: the remainder drops at the body's feet
        app.open_inventory();
        for it in app.inventory.items.iter_mut() {
            *it = (62, 64);
        }
        app.inv_carried = (62, 9);
        let (bx, by, bz) = {
            let b = app.body.as_ref().unwrap();
            (b.x, b.y, b.z)
        };
        app.close_inventory();
        assert_eq!(app.inv_carried, (0, 0), "the cursor emptied");
        let dropped = app
            .drops
            .iter()
            .find(|d| d.item == 62 && d.count == 9)
            .expect("the remainder dropped");
        assert!((dropped.x - bx).abs() < 1.0 && (dropped.z - bz).abs() < 1.0);
        let _ = by;
    }


    /// THE MANNEQUIN'S OWN LAW (the 26.2 bytecode the round pinned): the
    /// neutral pose when no mouse speaks, the atan turn law on the mouse
    /// (the rect-center delta over 40, atan, *20 — the bodyRot 180 + the
    /// lean), and the VP chain's placement (the feet 28.875*S below the
    /// rect center, the model's center 1.875*S above — the slightly-low
    /// vanilla placement inside the 49x70 rect).
    #[test]
    fn the_mannequin_law_pins_the_vanilla_chain() {
        // THE TURN LAW: the neutral pose (the headless stand-in)
        assert_eq!(mannequin_turn(None, 232, 97), (180.0, 0.0));
        // the mouse 40 px left of the rect center (the i32 mouse the
        // real input speaks — the expected folds the ACTUAL 40.5 delta
        // the center's half-pixel keeps): atan(d/40)*20 = the lean the
        // vanilla radians-as-degrees soup speaks
        let (x1, y1, x2, y2) = MANNEQUIN_RECT;
        let cx = (232 + x1 + 232 + x2) as f32 / 2.0;
        let cy = (97 + y1 + 97 + y2) as f32 / 2.0;
        let mx = cx as i32 - 40;
        let d = (cx - mx as f32) / 40.0;
        let (yaw, pitch) = mannequin_turn(Some((mx, cy as i32)), 232, 97);
        assert!((yaw - (180.0 + 20.0 * d.atan())).abs() < 1e-4);
        assert!(pitch.abs() < 1e-4, "the vertical delta zero");
        // THE VP: the feet and the model's center project through the
        // chain the raster's own NDC mapping consumes
        let vp = mannequin_vp(232, 97, 1.0, 640.0, 360.0, 0.0);
        let project = |p: [f32; 3]| -> (f32, f32) {
            let c = vp.transform_point(crate::math::Vec3::new(p[0], p[1], p[2]));
            let ndc_x = c[0] / c[3];
            let ndc_y = c[1] / c[3];
            (
                (ndc_x * 0.5 + 0.5) * 640.0,
                (1.0 - (ndc_y * 0.5 + 0.5)) * 360.0,
            )
        };
        let (fx, fy) = project([0.0, 0.0, 0.0]);
        let (mx, my) = project([0.0, 0.9, 0.0]);
        assert!((fx - cx).abs() < 0.01, "the feet centered: {fx} vs {cx}");
        assert!(
            (fy - (cy + 28.875)).abs() < 0.01,
            "the feet 28.875 below: {fy}"
        );
        assert!(
            (my - (cy + 1.875)).abs() < 0.01,
            "the center 1.875 below (the 0.0625 lift's own quirk): {my}"
        );
        // both inside the mannequin rect (the 49x70 window the PiP owns)
        let (rx0, ry0, rx1, ry1) = (232 + x1, 97 + y1, 232 + x2, 97 + y2);
        assert!(fx >= rx0 as f32 && fx < rx1 as f32);
        assert!(fy >= ry0 as f32 && fy < ry1 as f32);
        assert!(mx >= rx0 as f32 && mx < rx1 as f32);
        assert!(my >= ry0 as f32 && my < ry1 as f32);
    }


    /// THE SCREEN'S OWN PIXELS: the background texture blits (the panel
    /// face the bevel's own gray speaks), the store's icons and counts
    /// land in the slots (the pair against the empty store differs), and
    /// the mannequin's body pixels fill the hole (the model-loaded gate
    /// the sha1 walk keeps — the no-cache boot skips the pass honestly).
    #[test]
    fn the_inventory_screen_renders_the_store() {
        // the test's own pixel reader (the fb's public color vec)
        fn px(fb: &crate::raster::Framebuf, x: usize, y: usize) -> [u8; 3] {
            let o = (y * fb.w + x) * 3;
            [fb.color[o], fb.color[o + 1], fb.color[o + 2]]
        }
        // the cache root the loaders' prefix walk keeps (the CWD the
        // cargo test process speaks varies — the absent cache the
        // honest skip answers, the font tests' real() gate the shape)
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        // the EMPTY store first (the baseline the pair speaks against)
        let mut fb = crate::raster::Framebuf::new(640, 360);
        let mut assets = GuiAssets::open(&root);
        app.render(&mut fb, &mut assets);
        let empty = fb.color.clone();
        // THE STAGED STORE: the same loadout the headless staging speaks
        app.inventory.items[0] = (62, 27);
        app.inventory.items[1] = (63, 16);
        app.hotbar_slots = app.inventory.hotbar();
        let mut fb = crate::raster::Framebuf::new(640, 360);
        app.render(&mut fb, &mut assets);
        // the panel face pixel (the texture's own 198 gray at panel
        // (50, 81) — between the hole and the rows, no chrome)
        let face = px(&fb, 232 + 50, 97 + 81);
        assert_eq!(face, [198, 198, 198], "the background blit speaks");
        // the slot pair differs (the icons painted over the chrome)
        let (sx, sy) = inventory_slot_origin((232, 97), 0);
        let mut diff = 0usize;
        for dy in 0..16 {
            for dx in 0..16 {
                if px(&fb, sx as usize + 1 + dx, sy as usize + 1 + dy)
                    != [
                        empty[((sy as usize + 1 + dy) * fb.w + sx as usize + 1 + dx) * 3],
                        empty[((sy as usize + 1 + dy) * fb.w + sx as usize + 1 + dx) * 3 + 1],
                        empty[((sy as usize + 1 + dy) * fb.w + sx as usize + 1 + dx) * 3 + 2],
                    ]
                {
                    diff += 1;
                }
            }
        }
        assert!(diff > 40, "the icon painted: {diff} pixels differ");
        // the mannequin's body pixels inside the hole (the model gate —
        // the same root the loader walks)
        if crate::avatar::AvatarModel::load(&root).is_some() {
            let mut diff = 0usize;
            for y in 110..165 {
                for x in 262..303 {
                    if px(&fb, x, y) != [0, 0, 0] {
                        diff += 1;
                    }
                }
            }
            assert!(diff > 200, "the mannequin painted: {diff} pixels");
        }
    }


    /// THE MANNEQUIN'S ARMOR (the HumanoidArmorLayer walk the store's
    /// own rows keep): the EMPTY store renders pixel-identical twice
    /// (the no-armor path adds ZERO quads — the golden law's own gate),
    /// the iron kit paints the head/chest/legs (the equipment textures
    /// the sha1 table verified), the elytra renders NOTHING (the
    /// wings-layer asset the humanoid slots never texture), and the
    /// leather chestplate rides its dyeable composite (the overlay
    /// over the tinted base the client info JSON speaks).
    #[test]
    fn the_mannequin_wears_the_store_armor() {
        // the runtime root the loaders' prefix walk keeps (the absent
        // cache the honest skip answers — the mannequin tests' own law)
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        let Some(_) = crate::avatar::AvatarModel::load(&root) else {
            return; // the skin asset gate — the armor rides the same honesty
        };
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        let mut assets = GuiAssets::open(&root);
        let bare = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let bare_twice = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color
        };
        assert_eq!(bare, bare_twice, "the empty-armor path adds zero quads");
        let px = |color: &[u8], x: usize, y: usize| -> [u8; 3] {
            let o = (y * 640 + x) * 3;
            [color[o], color[o + 1], color[o + 2]]
        };
        // the mannequin's own window (the hole the PiP owns — the armor
        // slots' icons never overlap it)
        let diff_in_window = |a: &[u8], b: &[u8]| -> usize {
            let mut diff = 0usize;
            for y in 110..165 {
                for x in 262..303 {
                    if px(a, x, y) != px(b, x, y) {
                        diff += 1;
                    }
                }
            }
            diff
        };
        // THE IRON KIT (the SLOT_IDS order: HEAD, CHEST, LEGS, FEET)
        let id = |name: &str| mc_registry::item_id_by_name(name).expect("the registry speaks");
        app.inventory.armor = [
            (id("iron_helmet"), 1),
            (id("iron_chestplate"), 1),
            (id("iron_leggings"), 1),
            (id("iron_boots"), 1),
        ];
        let iron = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let iron_diff = diff_in_window(&bare, &iron);
        assert!(iron_diff > 120, "the iron kit painted: {iron_diff} pixels");
        // each body region the kit covers changed (the helmet's crown
        // band, the chestplate's torso band, the legs' own band)
        let band_diff = |a: &[u8], b: &[u8], y0: usize, y1: usize| -> usize {
            let mut diff = 0usize;
            for y in y0..y1 {
                for x in 262..303 {
                    if px(a, x, y) != px(b, x, y) {
                        diff += 1;
                    }
                }
            }
            diff
        };
        assert!(band_diff(&bare, &iron, 108, 122) > 5, "the helmet painted");
        assert!(band_diff(&bare, &iron, 126, 146) > 5, "the chestplate painted");
        assert!(band_diff(&bare, &iron, 148, 165) > 5, "the legs painted");
        // the cache holds the TWO layer entries the iron kit names (the
        // humanoid dir the head/chest/feet share, the leggings dir the
        // legs' own layer rides) — both sha1-verified loads
        use mc_registry::generated::equipment;
        assert_eq!(app.armor_textures.len(), 2);
        assert!(matches!(
            app.armor_textures.get(&format!("{}/iron", equipment::HUMANOID_DIR)),
            Some(Some(_))
        ));
        assert!(matches!(
            app.armor_textures.get(&format!("{}/iron", equipment::LEGGINGS_DIR)),
            Some(Some(_))
        ));
        // THE ELYTRA'S HONEST MISS: the CHEST slot's piece the
        // equipment asset never names (the wings layer the humanoid
        // slots never texture) renders nothing — the mannequin back to
        // the bare skin (the shouldRender gate's empty-asset verdict;
        // the piece itself rides an item icon — the flat slot path)
        app.inventory.armor = [(0, 0), (id("elytra"), 1), (0, 0), (0, 0)];
        let elytra = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let elytra_diff = diff_in_window(&bare, &elytra);
        assert_eq!(elytra_diff, 0, "the elytra rides no humanoid layer");
        // THE LEATHER COMPOSITE: the dyeable pair loads (the base + the
        // untinted overlay), the chest band paints the tinted brown
        app.inventory.armor = [(0, 0), (id("leather_chestplate"), 1), (0, 0), (0, 0)];
        let leather = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        assert!(matches!(
            app.armor_textures.get(&format!("{}/leather", equipment::HUMANOID_DIR)),
            Some(Some(_))
        ));
        assert!(matches!(
            app.armor_textures
                .get(&format!("{}/leather_overlay", equipment::HUMANOID_DIR)),
            Some(Some(_))
        ));
        let mut leather_diff = 0usize;
        for y in 120..155 {
            for x in 262..303 {
                if px(&bare, x, y) != px(&leather, x, y) {
                    leather_diff += 1;
                }
            }
        }
        assert!(leather_diff > 60, "the leather chest painted: {leather_diff}");
    }


    /// THE MANNEQUIN'S HEAD-BLOCK (the CustomHeadLayer item branch the
    /// carved pumpkin rides — the head stack the armor walk refuses
    /// still paints the mannequin): the pumpkin's block pixels cover
    /// the head (the bare-skin diff speaks), the equipment loads stay
    /// EMPTY (the asset fold's own verdict — the head-block lane is no
    /// armor lane), and the skull family the `wornHeadType` branch
    /// keeps renders NOTHING (the special-models' follow-up — the
    /// honest silence the elytra's own miss speaks).
    #[test]
    fn the_mannequin_wears_the_carved_pumpkin() {
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        let Some(_) = crate::avatar::AvatarModel::load(&root) else {
            return; // the skin asset gate — the head-block rides the same honesty
        };
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        let mut assets = GuiAssets::open(&root);
        let bare = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let px = |color: &[u8], x: usize, y: usize| -> [u8; 3] {
            let o = (y * 640 + x) * 3;
            [color[o], color[o + 1], color[o + 2]]
        };
        let diff_in_window = |a: &[u8], b: &[u8]| -> usize {
            let mut diff = 0usize;
            for y in 96..168 {
                for x in 262..303 {
                    if px(a, x, y) != px(b, x, y) {
                        diff += 1;
                    }
                }
            }
            diff
        };
        // THE PUMPKIN PAINTS (the 0.625 block over the head — bigger
        // than the helmet the armor lane paints, the overhang ABOVE the
        // crown the 2.0635 top speaks)
        let id = |name: &str| mc_registry::item_id_by_name(name).expect("the registry speaks");
        app.inventory.armor = [(id("carved_pumpkin"), 1), (0, 0), (0, 0), (0, 0)];
        let pumpkin = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let pumpkin_diff = diff_in_window(&bare, &pumpkin);
        // 98 at the raw-order fix (v0.3.245): the CORRECT paint stays
        // inside the head block (the carved face replaces the skin's
        // own, the dark carved texels matching the bare render's dark
        // hair/eyes where they overlap) — the old bowtie garble scored
        // higher only because its sheared survivor spilled over the
        // gui background. The regression floor: no paint scores ~0.
        assert!(pumpkin_diff > 60, "the pumpkin painted: {pumpkin_diff}");
        // the equipment loads stay EMPTY (the asset fold refuses the
        // pumpkin — the head-block lane speaks no armor texture)
        assert!(app.armor_textures.is_empty());
    }


    /// THE MANNEQUIN'S WORN SKULL (the CustomHeadLayer wornHeadType
    /// branch the skeleton skull rides — the model straight through the
    /// head's own chain, the 1.1875 scale the submitSkull walk keeps):
    /// the skull paints the mannequin (the entity diffuse the skin's
    /// own law shades it), the equipment loads stay EMPTY (the asset
    /// fold refuses the whole family), and the piglin's ears + the
    /// dragon's jaw ride their part-tree poses (the quad counts the
    /// dump speaks).
    #[test]
    fn the_mannequin_wears_the_skeleton_skull() {
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        let Some(_) = crate::avatar::AvatarModel::load(&root) else { return };
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        let mut assets = GuiAssets::open(&root);
        let bare = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let id = |name: &str| mc_registry::item_id_by_name(name).expect("the registry speaks");
        app.inventory.armor = [(id("skeleton_skull"), 1), (0, 0), (0, 0), (0, 0)];
        let skull = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let px = |color: &[u8], x: usize, y: usize| -> [u8; 3] {
            let o = (y * 640 + x) * 3;
            [color[o], color[o + 1], color[o + 2]]
        };
        let mut skull_diff = 0usize;
        for y in 96..168 {
            for x in 262..303 {
                if px(&bare, x, y) != px(&skull, x, y) {
                    skull_diff += 1;
                }
            }
        }
        assert!(skull_diff > 40, "the worn skull painted: {skull_diff}");
        // the asset fold's own verdict — the family speaks no armor texture
        assert!(app.armor_textures.is_empty());
    }


    #[test]
    fn the_skull_icon_bakes_in_its_slot() {
        // THE /16 LAW (the vanilla Cube constructor's own division):
        // the dump-backed special polygons bake in block units — the
        // skeleton skull's icon paints INSIDE the head slot's 16x16
        // (the former raw-texel bake spread a ~180px smear across the
        // screen — the regressions this test buries)
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        let Some(_) = crate::avatar::AvatarModel::load(&root) else { return };
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        let mut assets = GuiAssets::open(&root);
        let bare = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let id = mc_registry::item_id_by_name("skeleton_skull").expect("the registry speaks");
        app.inventory.armor = [(id, 1), (0, 0), (0, 0), (0, 0)];
        let skull = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let px = |c: &[u8], x: usize, y: usize| -> [u8; 3] {
            let o = (y * 640 + x) * 3;
            [c[o], c[o + 1], c[o + 2]]
        };
        // the head slot's own region (panel 232,97 + (8,8), the +1 inset
        // the icon draw keeps — a 2px margin around the 16x16), PLUS the
        // mannequin's own window (the worn skull the wornHeadType lane
        // paints — the diff law's second home)
        let in_slot = |x: usize, y: usize| x >= 239 && x < 258 && y >= 104 && y < 123;
        let in_window = |x: usize, y: usize| x >= 262 && x < 303 && y >= 96 && y < 168;
        let mut slot_px = 0usize;
        let mut strays = Vec::new();
        for y in 0..360 {
            for x in 0..640 {
                if px(&bare, x, y) != px(&skull, x, y) {
                    if in_slot(x, y) {
                        slot_px += 1;
                    } else if !in_window(x, y) {
                        if strays.len() < 8 {
                            strays.push((x, y));
                        }
                    }
                }
            }
        }
        assert!(slot_px > 100, "the skull icon painted the slot: {slot_px}");
        assert!(strays.is_empty(), "the icon stayed in its slot: {strays:?}");
    }


    /// THE MANNEQUIN'S HELD ITEM (the ItemInHandLayer
    /// submitArmWithItem walk the selected hotbar stack keeps — the
    /// main hand the extraction fills): the selected slot's stone
    /// paints the mannequin's hand (the bare-skin diff speaks), the
    /// empty store renders pixel-identical twice (the no-item path
    /// adds ZERO quads — the golden law's own gate), a DIFFERENT
    /// selected slot keeps the mannequin bare (the selection the main
    /// hand follows), and the equipment loads stay EMPTY (the held
    /// lane is no armor lane).
    #[test]
    fn the_mannequin_holds_the_selected_stack() {
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        let Some(_) = crate::avatar::AvatarModel::load(&root) else { return };
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        let mut assets = GuiAssets::open(&root);
        let bare = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let bare_twice = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color
        };
        assert_eq!(bare, bare_twice, "the empty-hand path adds zero quads");
        let px = |c: &[u8], x: usize, y: usize| -> [u8; 3] {
            let o = (y * 640 + x) * 3;
            [c[o], c[o + 1], c[o + 2]]
        };
        // the mannequin's own neighborhood (the window the PiP owns +
        // the held item's spill past its chrome — the hand's own
        // reach), the pair the held lane's paint speaks through
        let diff_in_window = |a: &[u8], b: &[u8]| -> usize {
            let mut diff = 0usize;
            for y in 96..168 {
                for x in 250..320 {
                    if px(a, x, y) != px(b, x, y) {
                        diff += 1;
                    }
                }
            }
            diff
        };
        // THE SELECTED STACK (the stone in the main hand — slot 0 the
        // default selected speaks)
        let id = mc_registry::item_id_by_name("stone").expect("the registry speaks");
        app.inventory.items[app.selected_hotbar] = (id, 27);
        let held = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let held_diff = diff_in_window(&bare, &held);
        assert!(held_diff > 20, "the held stone painted: {held_diff} pixels");
        // no strays outside the mannequin's neighborhood (the slot
        // icon the slot pass paints rides its own region — the slot 0
        // icon the store's own draw keeps; the rest is the held lane's)
        let (sx, sy) = inventory_slot_origin((232, 97), 0);
        let mut strays = Vec::new();
        for y in 0..360 {
            for x in 0..640 {
                if px(&bare, x, y) != px(&held, x, y) {
                    let in_window = x >= 250 && x < 320 && y >= 96 && y < 168;
                    let in_slot0 = x >= sx as usize && x < sx as usize + 18
                        && y >= sy as usize && y < sy as usize + 18;
                    if !in_window && !in_slot0 {
                        if strays.len() < 8 {
                            strays.push((x, y));
                        }
                    }
                }
            }
        }
        assert!(strays.is_empty(), "the paint stayed home: {strays:?}");
        // the equipment loads stay EMPTY (the held lane is no armor lane)
        assert!(app.armor_textures.is_empty(), "no armor loads");
        // A DIFFERENT SELECTION (slot 3 empty): the main hand's stack
        // the selection names — the mannequin back to bare
        app.selected_hotbar = 3;
        let other = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        assert_eq!(
            diff_in_window(&bare, &other),
            0,
            "the empty selected slot keeps the hand bare"
        );
        // THE SELECTED SLOT'S OWN STACK (the sword at slot 3 — the
        // selection moves the item's paint)
        let sword = mc_registry::item_id_by_name("iron_sword").expect("the registry speaks");
        app.inventory.items[app.selected_hotbar] = (sword, 1);
        let swd = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let swd_diff = diff_in_window(&bare, &swd);
        assert!(swd_diff > 5, "the held sword painted: {swd_diff} pixels");
    }


    /// THE OFFHAND'S OWN LANE (ItemInHandLayer.submit's SECOND
    /// submitArmWithItem — the leftHandItemState/Stack pair the
    /// extraction fills, the mannequin's OFFHAND slot 45 the store
    /// keeps): the offhand stack paints the mannequin's LEFT side the
    /// leftArm chain rides, the empty offhand adds zero quads, the
    /// paint stays home (the mannequin's window + the offhand slot's
    /// own icon the slot pass draws), the equipment loads stay empty.
    #[test]
    fn the_mannequin_holds_the_offhand_stack() {
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        let Some(_) = crate::avatar::AvatarModel::load(&root) else { return };
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        let mut assets = GuiAssets::open(&root);
        let bare = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let bare_twice = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color
        };
        assert_eq!(bare, bare_twice, "the empty-offhand path adds zero quads");
        let px = |c: &[u8], x: usize, y: usize| -> [u8; 3] {
            let o = (y * 640 + x) * 3;
            [c[o], c[o + 1], c[o + 2]]
        };
        let diff_in_window = |a: &[u8], b: &[u8]| -> usize {
            let mut diff = 0usize;
            for y in 96..168 {
                for x in 250..320 {
                    if px(a, x, y) != px(b, x, y) {
                        diff += 1;
                    }
                }
            }
            diff
        };
        // THE OFFHAND STACK (the stone rides slot 45 — the main hand
        // stays empty so the paint is the offhand lane's alone)
        let id = mc_registry::item_id_by_name("stone").expect("the registry speaks");
        app.inventory.offhand[0] = (id, 5);
        let off = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let off_diff = diff_in_window(&bare, &off);
        assert!(off_diff > 20, "the offhand stone painted: {off_diff} pixels");
        // no strays outside the mannequin's neighborhood (the offhand
        // slot's own icon the slot pass paints rides its region — the
        // rest is the offhand lane's)
        let (ox, oy) = offhand_slot_origin((232, 97));
        let mut strays = Vec::new();
        for y in 0..360 {
            for x in 0..640 {
                if px(&bare, x, y) != px(&off, x, y) {
                    let in_window = x >= 250 && x < 320 && y >= 96 && y < 168;
                    let in_slot = x >= ox as usize && x < ox as usize + 18
                        && y >= oy as usize && y < oy as usize + 18;
                    if !in_window && !in_slot {
                        if strays.len() < 8 {
                            strays.push((x, y));
                        }
                    }
                }
            }
        }
        assert!(strays.is_empty(), "the paint stayed home: {strays:?}");
        // the equipment loads stay EMPTY (the offhand lane is no armor
        // lane — the main hand's own verdict this lane shares)
        assert!(app.armor_textures.is_empty(), "no armor loads");
        // THE CLEARED OFFHAND (slot 45 back to empty): the mannequin
        // back to bare — the lane's gate the count's zero keeps
        app.inventory.offhand[0] = (0, 0);
        let cleared = {
            let mut fb = crate::raster::Framebuf::new(640, 360);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        assert_eq!(
            diff_in_window(&bare, &cleared),
            0,
            "the empty offhand keeps the hand bare"
        );
    }


    /// THE WINDOW-0 WIRE PICKUP (the vanilla AbstractContainerScreen's
    /// own mouseClicked over the player's OWN menu — the containerId 0
    /// the InventoryMenu's ctor pins): the left press on a main-grid
    /// cell the live store's belief feeds — the whole stack rides the
    /// cursor, the changed slot + the post-click cursor the packet
    /// speaks, the revision the feed's own state_id rides — and the
    /// OFFLINE store the wire world never touches (the two lanes
    /// never mix).
    #[test]
    fn the_wire_inventory_click_speaks_the_window0_pickup() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        // the window-0 sync: the main grid's cell (store 12 = the
        // menu's own slot id 12 — the store >= 9 the identity keeps)
        // carries 32 dirt, the revision 7 the packets spoke last
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[12] = (dirt, 32);
        app.live_inventory_updated(slots, Vec::new(), 7, (0, 0));
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 12);
        app.on_press(ox + 8, oy + 8);
        assert_eq!(
            app.live_inv_slots[12],
            (0, 0),
            "the believed slot the pickup emptied"
        );
        assert_eq!(app.live_inv_carried, (dirt, 32), "the cursor's own take");
        assert_eq!(
            app.inventory.items[12], (0, 0),
            "the offline store the wire world never touches"
        );
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 0, "the player's OWN menu the ctor pins");
                assert_eq!(c.state_id, 7, "the revision the feed spoke");
                assert_eq!(c.slot, 12, "the menu's own slot id the store keeps");
                assert_eq!(c.button, 0);
                assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
                assert_eq!(c.changed, vec![(12, (0, 0))]);
                assert_eq!(c.carried, (dirt, 32));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE HOTBAR CELL'S OWN USE_ROW ID (the store index's menu law —
    /// the hotbar row the menu ids 36..44 the USE_ROW constants keep,
    /// NOT the store's own 0..8): the right press places ONE from the
    /// carried stack (the split-table's own law), the HUD row's
    /// belief follows the menu ids the sync replaces wholesale.
    #[test]
    fn the_wire_inventory_hotbar_click_follows_the_use_row_ids() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        // the hotbar cell store 3 = the menu's own slot id 39 (the
        // 36 + store the USE_ROW law keys), the cursor carries 10
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[39] = (0, 0);
        app.live_inventory_updated(slots, Vec::new(), 9, (dirt, 10));
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 3);
        // the press->release drive (the type-1 arm law the carried
        // press wears — the release's own deferred place the ONE
        // speaks; the pre-press motion the release position feeds)
        app.on_mouse_move(ox + 8, oy + 8);
        app.on_right_press(ox + 8, oy + 8);
        app.on_right_release();
        assert_eq!(
            app.live_inv_slots[39],
            (dirt, 1),
            "the ONE item the right press placed"
        );
        assert_eq!(app.live_inv_carried, (dirt, 9), "the cursor's own split");
        assert_eq!(
            app.hotbar_slots[3],
            (dirt, 1),
            "the HUD row's belief the menu id keys"
        );
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 39, "the USE_ROW's own menu id");
                assert_eq!(c.button, 1);
                assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
                assert_eq!(c.changed, vec![(39, (dirt, 1))]);
                assert_eq!(c.carried, (dirt, 9));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE QUICK-MOVE'S MENU NUMBERING (the shift-click the
    /// quickMoveStack walk speaks): the main grid's source rides the
    /// HOTBAR's merge order (the store 0..8 the hotbar stores), the
    /// changed indices map BACK to the menu ids (the store 4 -> 40,
    /// the store 0 -> 36 — the USE_ROW law the pickup test keeps),
    /// the HUD row the belief's own law follows, the armor column
    /// untouched.
    #[test]
    fn the_wire_inventory_quick_move_rides_the_menu_numbering() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        app.on_shift_down(true);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        // the main grid's source (store 14 = menu 14) carries 60, the
        // hotbar cells the merge order walks: store 4 (menu 40) holds
        // 10 (room 54), store 0 (menu 36) the first empty — the walk
        // feeds 64 + 6, the source empties
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[14] = (dirt, 60);
        slots[40] = (dirt, 10);
        app.live_inventory_updated(slots, Vec::new(), 3, (0, 0));
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 14);
        app.on_press(ox + 8, oy + 8);
        assert_eq!(app.live_inv_slots[14], (0, 0), "the source's own empty");
        assert_eq!(app.live_inv_slots[40], (dirt, 64), "the merge's own cap");
        assert_eq!(app.live_inv_slots[36], (dirt, 6), "the remainder's own slot");
        assert_eq!(app.hotbar_slots[4], (dirt, 64), "the HUD belief rides");
        assert_eq!(app.hotbar_slots[0], (dirt, 6));
        assert!(
            app.live_inv_slots[5..9].iter().all(|&s| s == (0, 0)),
            "the armor column the walk never touched"
        );
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 0);
                assert_eq!(c.slot, 14);
                assert_eq!(
                    c.input,
                    crate::world_view::ContainerInput::QuickMove,
                    "the shift the hasShiftDown query speaks"
                );
                assert_eq!(
                    c.changed,
                    vec![(14, (0, 0)), (40, (dirt, 64)), (36, (dirt, 6))],
                    "the menu-keyed deltas the walk mapped"
                );
                assert_eq!(c.carried, (0, 0), "the quick-move the cursor never rides");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        app.on_shift_down(false);
    }


    /// THE SWAP KEY'S OWN WIRE (the digit press the live session's
    /// ServerboundContainerClickPacket speaks — the doClick SWAP table
    /// over the BELIEF): CASE B the hovered store stack rides the
    /// empty hotbar row AS-IS (the cell EMPTIES, the HUD row's belief
    /// synced, the packet the walk's own mutations names, the cursor
    /// the SWAP arm never touches), the OFFLINE store the wire world
    /// never touches.
    #[test]
    fn the_wire_swap_key_rides_the_swap_table() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        // the window-0 sync: store 12 (menu 12) carries 32 dirt, the
        // hotbar row 0 (menu 36) empty, the revision 7 the packets
        // spoke last
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[12] = (dirt, 32);
        app.live_inventory_updated(slots, Vec::new(), 7, (0, 0));
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 12);
        app.mouse_pos = Some((ox + 8, oy + 8));
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.live_inv_slots[12], (0, 0), "the cell emptied");
        assert_eq!(app.live_inv_slots[36], (dirt, 32), "the row rides AS-IS");
        assert_eq!(app.hotbar_slots[0], (dirt, 32), "the HUD belief synced");
        assert_eq!(
            app.inventory.items[12], (0, 0),
            "the offline store the wire world never touches"
        );
        assert_eq!(app.screen, ScreenKind::Inventory, "the screen stays");
        assert_eq!(app.live_inv_carried, (0, 0), "the cursor the SWAP never touches");
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 0, "the player's OWN menu the ctor pins");
                assert_eq!(c.state_id, 7, "the revision the feed spoke");
                assert_eq!(c.slot, 12, "the hovered seat's own menu id");
                assert_eq!(c.button, 0, "the digit the button byte keys");
                assert_eq!(
                    c.input,
                    crate::world_view::ContainerInput::Swap,
                    "the SWAP input the keyPressed walk speaks"
                );
                assert_eq!(
                    c.changed,
                    vec![(12, (0, 0)), (36, (dirt, 32))],
                    "the walk's own mutations the changed map names"
                );
                assert_eq!(c.carried, (0, 0));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE SWAP WIRE'S OWN GATES AND THE ALWAYS PACKET (the
    /// checkHotbarKeyPressed walk's own shape over the belief): the
    /// both-empty press the consumed silence keeps AND the packet STILL
    /// rides (the slotClicked call the keyPressed walk keeps
    /// unconditional — the changed map the menu's own untouched state
    /// keeps empty), the carried cursor the false verdict keeps BEFORE
    /// the packet lane (the getCarried().isEmpty() gate the world chain
    /// answers), the null hover and the no-panel the same refusals
    /// speak, and the F key the OFFHAND row (button 40 the menu 45's
    /// own seat) the same CASE B answers.
    #[test]
    fn the_wire_swap_key_the_silences_and_the_always_packet() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[20] = (stone, 5);
        app.live_inventory_updated(slots, Vec::new(), 2, (0, 0));
        // THE BOTH-EMPTY PRESS (pcs 1130-1146): store 13 and the row 1
        // both empty — the consumed silence, the belief whole, the
        // packet rides EMPTY
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 13);
        app.mouse_pos = Some((ox + 8, oy + 8));
        assert!(app.on_inventory_swap_key(1), "the silence CONSUMES");
        assert_eq!(app.live_inv_slots[13], (0, 0), "the cell whole");
        assert_eq!(app.live_inv_slots[37], (0, 0), "the row whole");
        match click_rx.try_recv().expect("the always packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.input, crate::world_view::ContainerInput::Swap);
                assert!(c.changed.is_empty(), "the untouched state keeps no entry");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE CARRIED CURSOR (the getCarried().isEmpty() gate): the
        // false verdict BEFORE the packet lane — the world chain answers
        app.live_inv_carried = (stone, 5);
        let (sx, sy) = inventory_slot_origin(app.inv_panel, 20);
        assert!(!app.on_inventory_swap_key(2), "the carried cursor refuses");
        assert_eq!(app.live_inv_slots[20], (stone, 5), "the cell whole");
        assert!(
            click_rx.try_recv().is_err(),
            "the gate's refusal never rides the lane"
        );
        app.live_inv_carried = (0, 0);
        // THE NULL HOVER (the hoveredSlot null arm): the cursor between
        // the slots
        app.mouse_pos = Some((332, 157));
        assert!(!app.on_inventory_swap_key(2), "the null hover refuses");
        assert!(click_rx.try_recv().is_err(), "still silence");
        // THE NO-PANEL (the render never spoke): the gate keeps
        let panel = app.inv_panel;
        app.inv_panel = (0, 0);
        assert!(!app.on_inventory_swap_key(2), "the no-panel refuses");
        app.inv_panel = panel;
        assert!(click_rx.try_recv().is_err(), "still silence");
        // THE F KEY'S OFFHAND ROW (the button 40 the menu 45's seat):
        // the hovered stack rides the EMPTY offhand AS-IS — CASE B the
        // same walk answers
        app.mouse_pos = Some((sx + 8, sy + 8));
        assert!(app.on_inventory_swap_key(40), "the offhand chord consumes");
        assert_eq!(app.live_inv_slots[20], (0, 0), "the cell emptied");
        assert_eq!(app.live_inv_slots[45], (stone, 5), "the offhand took");
        match click_rx.try_recv().expect("the offhand packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 20);
                assert_eq!(c.button, 40, "the offhand key the button byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Swap);
                assert_eq!(c.changed, vec![(20, (0, 0)), (45, (stone, 5))]);
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE DROP KEY'S OWN WIRE (the Q the LIVE session speaks — the
    /// doClick THROW arm over the belief): the plain Q the ONE item
    /// takes (the count law the button 0 keys, the remainder the cell
    /// keeps), the packet the hovered seat's own menu id names, the
    /// changed map the walk's ACTUAL mutation carries, the offline
    /// store the wire world never touches, the cursor pair the THROW
    /// arm never touches, the screen stays.
    #[test]
    fn the_wire_drop_key_the_one_off_the_packet_wears() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        // the window-0 sync: store 12 (menu 12) carries 32 dirt, the
        // revision 7 the packets spoke last, the cursor empty
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[12] = (dirt, 32);
        app.live_inventory_updated(slots, Vec::new(), 7, (0, 0));
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 12);
        app.mouse_pos = Some((ox + 8, oy + 8));
        app.on_inventory_drop_key(false);
        assert_eq!(app.live_inv_slots[12], (dirt, 31), "the one off rode");
        assert_eq!(
            app.inventory.items[12], (0, 0),
            "the offline store the wire world never touches"
        );
        assert_eq!(app.screen, ScreenKind::Inventory, "the screen stays");
        assert_eq!(app.live_inv_carried, (0, 0), "the cursor never touched");
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 0, "the player's OWN menu the ctor pins");
                assert_eq!(c.state_id, 7, "the revision the feed spoke");
                assert_eq!(c.slot, 12, "the hovered seat's own menu id");
                assert_eq!(c.button, 0, "the plain Q the button byte keys");
                assert_eq!(
                    c.input,
                    crate::world_view::ContainerInput::Throw,
                    "the THROW input the keyPressed walk speaks"
                );
                assert_eq!(
                    c.changed,
                    vec![(12, (dirt, 31))],
                    "the walk's own mutation the changed map names"
                );
                assert_eq!(c.carried, (0, 0), "the cursor pair rides unchanged");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE DROP KEY'S OWN WIRE, THE CTRL ARM AND THE HUD ROW (the
    /// whole-stack law the button 1 keys): the hotbar row's stack the
    /// Ctrl+Q empties WHOLE (the safeTake(count, MAX) walk), the HUD
    /// row's belief the every-mutation law syncs, the armor seat the
    /// same walk answers (the menu 5's own pair the packet names).
    #[test]
    fn the_wire_drop_key_the_ctrl_the_hud_and_the_armor_wear() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[40] = (stone, 7); // the hotbar row 4 (store 4 → menu 40)
        slots[5] = (helmet, 1); // the armor seat k=0 → menu 5
        app.live_inventory_updated(slots, Vec::new(), 3, (0, 0));
        // THE CTRL+Q OVER THE HOTBAR ROW: the whole stack gone, the
        // HUD row's belief synced
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 4);
        app.mouse_pos = Some((ox + 8, oy + 8));
        app.on_inventory_drop_key(true);
        assert_eq!(app.live_inv_slots[40], (0, 0), "the whole stack gone");
        assert_eq!(app.hotbar_slots[4], (0, 0), "the HUD belief synced");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 40, "the hotbar row's own menu id");
                assert_eq!(c.button, 1, "the Ctrl the button byte keys");
                assert_eq!(c.changed, vec![(40, (0, 0))]);
                assert_eq!(c.carried, (0, 0));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE ARMOR SEAT (the menu 5): the plain Q the ONE off — the
        // piece's own count 1 the whole-take arm empties, the HUD row
        // the armor seat never wears
        let (ax, ay) = armor_slot_origin(app.inv_panel, 0);
        app.mouse_pos = Some((ax + 8, ay + 8));
        app.on_inventory_drop_key(false);
        assert_eq!(app.live_inv_slots[5], (0, 0), "the piece gone");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 5, "the armor seat's own menu id");
                assert_eq!(c.button, 0);
                assert_eq!(c.changed, vec![(5, (0, 0))]);
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE DROP KEY'S OWN WIRE, THE GATES AND THE ALWAYS PACKET (the
    /// keyPressed chain's own shape over the belief): the EMPTY hovered
    /// slot the silence keeps (the hasItem gate lives in the keyPressed
    /// walk — the slotClicked call never fires, NO packet), the CARRIED
    /// cursor the doClick arm's own refusal the ALWAYS-PACKET law names
    /// (the empty changed map the menu's untouched state keeps, the
    /// belief whole), the null hover and the no-panel the same
    /// refusals speak.
    #[test]
    fn the_wire_drop_key_the_gates_and_the_always_packet_wear() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[20] = (stone, 5);
        app.live_inventory_updated(slots, Vec::new(), 2, (0, 0));
        // THE EMPTY HOVERED SLOT (the hasItem gate lives in the
        // keyPressed chain): store 13 empty — the silence, NO packet
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 13);
        app.mouse_pos = Some((ox + 8, oy + 8));
        app.on_inventory_drop_key(false);
        assert_eq!(app.live_inv_slots[13], (0, 0), "the cell whole");
        assert!(
            click_rx.try_recv().is_err(),
            "the hasItem gate the packet lane never passes"
        );
        // THE CARRIED CURSOR (the doClick THROW's own isEmpty gate):
        // the packet STILL rides (the slotClicked call the keyPressed
        // walk keeps once the hovered+hasItem gates passed), the EMPTY
        // changed map the untouched state keeps, the belief whole
        let (sx, sy) = inventory_slot_origin(app.inv_panel, 20);
        app.mouse_pos = Some((sx + 8, sy + 8));
        app.live_inv_carried = (stone, 5);
        app.on_inventory_drop_key(true);
        assert_eq!(app.live_inv_slots[20], (stone, 5), "the belief whole");
        match click_rx.try_recv().expect("the always packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 20, "the hovered seat's own menu id");
                assert_eq!(c.button, 1, "the Ctrl the byte keeps");
                assert_eq!(
                    c.input,
                    crate::world_view::ContainerInput::Throw
                );
                assert!(
                    c.changed.is_empty(),
                    "the refused arm the untouched state keeps"
                );
                assert_eq!(
                    c.carried,
                    (stone, 5),
                    "the cursor pair rides unchanged"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        app.live_inv_carried = (0, 0);
        // THE NULL HOVER (the hoveredSlot null arm): the cursor between
        // the slots
        app.mouse_pos = Some((332, 157));
        app.on_inventory_drop_key(false);
        assert!(click_rx.try_recv().is_err(), "still silence");
        // THE NO-PANEL (the render never spoke): the gate keeps
        let panel = app.inv_panel;
        app.inv_panel = (0, 0);
        app.on_inventory_drop_key(false);
        assert!(click_rx.try_recv().is_err(), "still silence");
        app.inv_panel = panel;
    }


    /// THE TABLE'S OWN Q, THE OFFLINE WALK (the CraftingScreen's own
    /// keyDrop arm — the plain Slots the cells + the store rows are):
    /// the plain Q the ONE off (the recompute the cell's setChanged
    /// fires — the pattern's verdict the result re-speaks), the Ctrl+Q
    /// the whole stack (the broken pattern the recompute empties), the
    /// aimed drop the body throws, the carried cursor and the empty
    /// hover and the RESULT seat the silences keep.
    #[test]
    fn the_table_drop_key_the_cell_the_store_the_silences_wear() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open(ScreenKind::Crafting);
        app.table_panel = inventory_panel_origin(640, 360);
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let stick = mc_registry::item_id_by_name("stick").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        // THE STICKS RECIPE (the column the cells 1 + 4 keep — the
        // result (stick, 4) the verdict speaks)
        app.table_grid[1] = (planks, 2);
        app.table_grid[4] = (planks, 2);
        app.refresh_table_result();
        assert_eq!(app.table_result, (stick, 4), "the recipe armed");
        let drops_before = app.drops.len();
        // THE PLAIN Q ON CELL 1: the ONE off — the pattern HOLDS, the
        // recompute the verdict re-speaks, the aimed drop one plank
        let (cx, cy) = table_craft_origin(app.table_panel, 1);
        app.mouse_pos = Some((cx + 8, cy + 8));
        app.on_table_drop_key(false);
        assert_eq!(app.table_grid[1], (planks, 1), "the one off rode");
        assert_eq!(app.table_result, (stick, 4), "the pattern holds, the verdict re-spoke");
        assert_eq!(app.drops.len(), drops_before + 1, "the aimed drop rode");
        assert_eq!(app.screen, ScreenKind::Crafting, "the screen stays");
        assert_eq!(app.inv_carried, (0, 0), "the cursor never touched");
        // THE CTRL+Q ON CELL 4: the whole stack gone — the pattern
        // BROKEN, the recompute the result empties
        let (cx2, cy2) = table_craft_origin(app.table_panel, 4);
        app.mouse_pos = Some((cx2 + 8, cy2 + 8));
        app.on_table_drop_key(true);
        assert_eq!(app.table_grid[4], (0, 0), "the whole stack gone");
        assert_eq!(app.table_result, (0, 0), "the broken pattern the recompute empties");
        assert_eq!(app.drops.len(), drops_before + 2, "the second drop rode");
        // THE STORE ROW (the hotbar row 4 — the HUD sync the
        // every-mutation law keeps)
        app.inventory.items[4] = (stone, 7);
        let (ox, oy) = inventory_slot_origin(app.table_panel, 4);
        app.mouse_pos = Some((ox + 8, oy + 8));
        app.on_table_drop_key(true);
        assert_eq!(app.inventory.items[4], (0, 0), "the row emptied");
        assert_eq!(app.hotbar_slots[4], (0, 0), "the HUD synced");
        assert_eq!(app.drops.len(), drops_before + 3, "the third drop rode");
        // THE CARRIED CURSOR (the doClick's own isEmpty gate): the
        // cell whole, no drop
        app.inv_carried = (stone, 5);
        let (cx3, cy3) = table_craft_origin(app.table_panel, 1);
        app.mouse_pos = Some((cx3 + 8, cy3 + 8));
        app.on_table_drop_key(false);
        assert_eq!(app.table_grid[1], (planks, 1), "the cell whole");
        assert_eq!(app.drops.len(), drops_before + 3, "still three");
        app.inv_carried = (0, 0);
        // THE EMPTY HOVER (the null hoveredSlot): the cursor between
        // the slots
        app.mouse_pos = Some((332, 157));
        app.on_table_drop_key(false);
        assert_eq!(app.drops.len(), drops_before + 3, "still silence");
        // THE RESULT SEAT (the lane's own scope — the ResultSlot's
        // safeTake chain the later jar round banks): the silence, the
        // verdict whole
        app.table_result = (stick, 4);
        let (rx, ry) = table_craft_origin(app.table_panel, 9);
        app.mouse_pos = Some((rx + 8, ry + 8));
        app.on_table_drop_key(false);
        app.on_table_drop_key(true);
        assert_eq!(app.table_result, (stick, 4), "the result seat whole");
        assert_eq!(app.drops.len(), drops_before + 3, "the result seat is silence");
    }


    /// THE TABLE'S OWN Q, THE WIRE WALK (the window-1 belief the same
    /// doClick THROW arm answers): the cell's menu id the packet names
    /// (the window id the OpenScreen assigned), the store row the
    /// [`Self::table_store_menu_id`] law keys (the hotbar row 3 the
    /// menu 40 speaks), the HUD row's belief the every-mutation law
    /// syncs over the crafting menu's own 37..46 range, the offline
    /// store the wire world never touches.
    #[test]
    fn the_wire_table_drop_key_the_cells_the_store_the_hud_wear() {
        use crate::world_view::{ContainerCommand, LiveContainer};
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        let table = LiveContainer {
            id: 9,
            menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
            slots: {
                let mut s = vec![(0i32, 0i32); 46];
                s[3] = (planks, 8); // the craft cell 2 → menu 3
                s[40] = (stone, 7); // the hotbar row 3 → menu 40
                s
            },
            carried: (0, 0),
            state_id: 3,
            bundle_rows: Vec::new(),
            data_slots: Vec::new(),
            title_nbt: Vec::new(),
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
        assert_eq!(app.screen, ScreenKind::Crafting, "the window shows");
        app.table_panel = inventory_panel_origin(640, 360);
        // THE PLAIN Q ON CELL 2 (menu 3): the ONE off, the packet the
        // window's own identity names
        let (cx, cy) = table_craft_origin(app.table_panel, 2);
        app.mouse_pos = Some((cx + 8, cy + 8));
        app.on_table_drop_key(false);
        assert_eq!(app.live_table_slots[3], (planks, 7), "the one off rode");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 9, "the window's own id");
                assert_eq!(c.state_id, 3, "the revision rides");
                assert_eq!(c.slot, 3, "the cell's own menu id");
                assert_eq!(c.button, 0, "the plain Q the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Throw);
                assert_eq!(c.changed, vec![(3, (planks, 7))], "the mutation named");
                assert_eq!(c.carried, (0, 0), "the cursor pair rides unchanged");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE CTRL+Q ON STORE 3 (menu 40): the whole stack gone, the
        // HUD row's belief synced
        let (ox, oy) = inventory_slot_origin(app.table_panel, 3);
        app.mouse_pos = Some((ox + 8, oy + 8));
        app.on_table_drop_key(true);
        assert_eq!(app.live_table_slots[40], (0, 0), "the whole stack gone");
        assert_eq!(app.hotbar_slots[3], (0, 0), "the HUD belief synced");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 40, "the hotbar row's own menu id");
                assert_eq!(c.button, 1, "the Ctrl the byte keys");
                assert_eq!(c.changed, vec![(40, (0, 0))]);
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        assert_eq!(
            app.inventory.items[3], (0, 0),
            "the offline store the wire world never touches"
        );
    }


    /// THE TABLE'S OWN Q, THE WIRE GATES (the keyPressed chain's own
    /// shape over the window-1 belief): the EMPTY hovered cell the
    /// silence keeps (NO packet — the hasItem gate lives in the
    /// keyPressed walk), the carried cursor the doClick arm's own
    /// refusal the ALWAYS-PACKET law names (the EMPTY changed map, the
    /// belief whole), the RESULT seat and the no-window the same
    /// silences speak.
    #[test]
    fn the_wire_table_drop_key_the_gates_and_the_silences_wear() {
        use crate::world_view::{ContainerCommand, LiveContainer};
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let table = LiveContainer {
            id: 9,
            menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
            slots: {
                let mut s = vec![(0i32, 0i32); 46];
                s[3] = (planks, 8); // the craft cell 2 → menu 3
                s
            },
            carried: (0, 0),
            state_id: 3,
            bundle_rows: Vec::new(),
            data_slots: Vec::new(),
            title_nbt: Vec::new(),
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
        app.table_panel = inventory_panel_origin(640, 360);
        // THE EMPTY HOVERED CELL (the hasItem gate — cell 5 empty):
        // silence, NO packet
        let (ex, ey) = table_craft_origin(app.table_panel, 5);
        app.mouse_pos = Some((ex + 8, ey + 8));
        app.on_table_drop_key(false);
        assert!(
            click_rx.try_recv().is_err(),
            "the hasItem gate the packet lane never passes"
        );
        // THE CARRIED CURSOR (the doClick THROW's own isEmpty gate):
        // the packet STILL rides, the EMPTY changed map the untouched
        // state keeps, the belief whole
        let (cx, cy) = table_craft_origin(app.table_panel, 2);
        app.mouse_pos = Some((cx + 8, cy + 8));
        app.live_table_carried = (planks, 5);
        app.on_table_drop_key(true);
        assert_eq!(app.live_table_slots[3], (planks, 8), "the belief whole");
        match click_rx.try_recv().expect("the always packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 3, "the hovered seat's own menu id");
                assert_eq!(c.button, 1, "the Ctrl the byte keeps");
                assert_eq!(c.input, crate::world_view::ContainerInput::Throw);
                assert!(
                    c.changed.is_empty(),
                    "the refused arm the untouched state keeps"
                );
                assert_eq!(c.carried, (planks, 5), "the cursor pair unchanged");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        app.live_table_carried = (0, 0);
        // THE RESULT SEAT (the lane's own scope): the silence
        let (rx, ry) = table_craft_origin(app.table_panel, 9);
        app.mouse_pos = Some((rx + 8, ry + 8));
        app.on_table_drop_key(false);
        assert!(click_rx.try_recv().is_err(), "the result seat is silence");
        // THE NO WINDOW (the belief never armed): the gate keeps
        app.table_live_window = None;
        app.mouse_pos = Some((cx + 8, cy + 8));
        app.on_table_drop_key(false);
        assert!(click_rx.try_recv().is_err(), "still silence");
    }


    /// THE CHEST'S OWN Q (the GenericContainerScreen's keyDrop arm —
    /// the live window the lane alone wears): the grid's stack the
    /// plain-Q ONE off rides (the packet the window's identity names),
    /// the mirror's hotbar row the Ctrl+Q empties WHOLE (the HUD
    /// row's belief synced — the menu 54..62 the store indices 27..35
    /// speak), the cursor pair never touched, the screen stays.
    #[test]
    fn the_chest_drop_key_the_grid_the_mirror_the_hud_wear() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open(ScreenKind::Chest);
        app.chest_live_window = Some((5, 7));
        let panel = chest_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        app.chest_slots = vec![(0, 0); 27];
        app.chest_mirror = vec![(0, 0); 36];
        app.chest_slots[4] = (dirt, 32);
        app.chest_mirror[27] = (stone, 7); // the hotbar row 0 → menu 54
        // the rects the render lays: the grid's 0..27, the mirror's
        // 27..63 — the +1 inset the store's own lane speaks
        app.chest_slot_rects = (0..27)
            .map(|i| {
                let (sx, sy) = chest_container_slot(panel, i);
                (sx + 1, sy + 1, i as i16)
            })
            .chain((0..36).map(|i| {
                let (sx, sy) = chest_mirror_slot(panel, i);
                (sx + 1, sy + 1, (27 + i) as i16)
            }))
            .collect();
        // THE PLAIN Q ON GRID CELL 4: the ONE off, the packet the
        // window's own identity names
        let (gx, gy) = chest_container_slot(panel, 4);
        app.mouse_pos = Some((gx + 9, gy + 9));
        app.on_chest_drop_key(false);
        assert_eq!(app.chest_slots[4], (dirt, 31), "the one off rode");
        assert_eq!(app.screen, ScreenKind::Chest, "the screen stays");
        assert_eq!(app.chest_carried, (0, 0), "the cursor never touched");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 5, "the window's own id");
                assert_eq!(c.state_id, 7, "the revision rides");
                assert_eq!(c.slot, 4, "the grid cell's own menu id");
                assert_eq!(c.button, 0, "the plain Q the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Throw);
                assert_eq!(c.changed, vec![(4, (dirt, 31))], "the mutation named");
                assert_eq!(c.carried, (0, 0), "the cursor pair rides unchanged");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE CTRL+Q ON THE MIRROR'S HOTBAR ROW 0 (menu 54): the whole
        // stack gone, the HUD row's belief synced
        let (mx2, my2) = chest_mirror_slot(panel, 27);
        app.mouse_pos = Some((mx2 + 9, my2 + 9));
        app.on_chest_drop_key(true);
        assert_eq!(app.chest_mirror[27], (0, 0), "the whole stack gone");
        assert_eq!(app.hotbar_slots[0], (0, 0), "the HUD belief synced");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 54, "the mirror hotbar row's own menu id");
                assert_eq!(c.button, 1, "the Ctrl the byte keys");
                assert_eq!(c.changed, vec![(54, (0, 0))]);
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE CHEST'S OWN Q, THE GATES (the keyPressed chain's own shape
    /// over the generic_9x3's belief): the EMPTY hovered slot the
    /// silence keeps (NO packet), the carried cursor the doClick arm's
    /// own refusal the ALWAYS-PACKET law names (the EMPTY changed map,
    /// the belief whole), the between-slots press and the offline
    /// stand-in (no window) the same silences speak.
    #[test]
    fn the_chest_drop_key_the_gates_and_the_always_packet_wear() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open(ScreenKind::Chest);
        let panel = chest_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        app.chest_slots = vec![(0, 0); 27];
        app.chest_mirror = vec![(0, 0); 36];
        app.chest_slots[4] = (dirt, 32);
        app.chest_slot_rects = (0..27)
            .map(|i| {
                let (sx, sy) = chest_container_slot(panel, i);
                (sx + 1, sy + 1, i as i16)
            })
            .chain((0..36).map(|i| {
                let (sx, sy) = chest_mirror_slot(panel, i);
                (sx + 1, sy + 1, (27 + i) as i16)
            }))
            .collect();
        // THE OFFLINE STAND-IN (no window armed): the silence — the
        // click lane's own gate the chest speaks
        let (gx, gy) = chest_container_slot(panel, 4);
        app.mouse_pos = Some((gx + 9, gy + 9));
        app.on_chest_drop_key(false);
        assert!(click_rx.try_recv().is_err(), "the offline stand-in is silence");
        // THE WINDOW ARMS
        app.chest_live_window = Some((5, 7));
        // THE EMPTY HOVERED SLOT (the hasItem gate): grid cell 5 empty —
        // silence, NO packet
        let (ex, ey) = chest_container_slot(panel, 5);
        app.mouse_pos = Some((ex + 9, ey + 9));
        app.on_chest_drop_key(false);
        assert!(
            click_rx.try_recv().is_err(),
            "the hasItem gate the packet lane never passes"
        );
        // THE CARRIED CURSOR (the doClick THROW's own isEmpty gate):
        // the packet STILL rides, the EMPTY changed map the untouched
        // state keeps, the belief whole
        app.mouse_pos = Some((gx + 9, gy + 9));
        app.chest_carried = (dirt, 5);
        app.on_chest_drop_key(true);
        assert_eq!(app.chest_slots[4], (dirt, 32), "the belief whole");
        match click_rx.try_recv().expect("the always packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 4, "the hovered seat's own menu id");
                assert_eq!(c.button, 1, "the Ctrl the byte keeps");
                assert_eq!(c.input, crate::world_view::ContainerInput::Throw);
                assert!(
                    c.changed.is_empty(),
                    "the refused arm the untouched state keeps"
                );
                assert_eq!(c.carried, (dirt, 5), "the cursor pair unchanged");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        app.chest_carried = (0, 0);
        // THE BETWEEN-SLOTS PRESS (the null hoveredSlot): the cursor
        // over the panel's spine — the rects never answer
        app.mouse_pos = Some((panel.0 + 170, panel.1 + 20));
        app.on_chest_drop_key(false);
        assert!(click_rx.try_recv().is_err(), "still silence");
    }


    /// THE CONTAINER SCREENS' OWN CLONE, THE COPY (the mouseClicked
    /// walk's flag arm over the window-0 belief — the doClick's own
    /// table pcs 1385-1446 answers): the safeClone law the FULL max
    /// the cursor takes (the diamond 64, the egg's own 16), the
    /// hovered seat the copy NEVER touches, the wire lane the packet
    /// the EMPTY changed map names — the slot-mutation silence the
    /// cursor-alone walk speaks.
    #[test]
    fn the_container_clone_arm_the_copy_and_the_wire_wear() {
        use crate::world_view::ContainerCommand;
        let egg = crate::mob::wire_id("egg").expect("the egg's own id");
        // THE OFFLINE LANE (the window-0's own store — the local walk
        // the sim's cursor answers)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        app.game_mode = crate::screens::GameMode::Creative;
        let diamond = mc_registry::item_id_by_name("diamond").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        app.inventory.items[12] = (diamond, 3);
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 12);
        app.on_container_clone_click(ox + 8, oy + 8);
        assert_eq!(app.inv_carried, (diamond, 64), "the FULL max the cursor takes");
        assert_eq!(app.inventory.items[12], (diamond, 3), "the seat the copy never touches");
        // THE EGG'S OWN MAX (the copyWithCount the item's own 16 takes
        // — the flat 64 the split law's own answer never wears)
        app.inv_carried = (0, 0);
        app.inventory.items[13] = (egg, 2);
        let (ox2, oy2) = inventory_slot_origin(app.inv_panel, 13);
        app.on_container_clone_click(ox2 + 8, oy2 + 8);
        assert_eq!(app.inv_carried, (egg, 16), "the egg's own max");
        assert_eq!(app.inventory.items[13], (egg, 2), "the seat whole");
        // THE WIRE LANE (the window-0 belief — the packet the live
        // session speaks, the EMPTY changed map the cursor-alone walk
        // names)
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open(ScreenKind::Inventory);
        app.inv_panel = inventory_panel_origin(640, 360);
        app.game_mode = crate::screens::GameMode::Creative;
        app.live_inv_slots[10] = (stone, 5);
        app.live_inv_state_id = 4;
        let (wx, wy) = inventory_slot_origin(app.inv_panel, 10);
        app.on_container_clone_click(wx + 8, wy + 8);
        assert_eq!(app.live_inv_carried, (stone, 64), "the belief's cursor rides");
        assert_eq!(app.live_inv_slots[10], (stone, 5), "the seat whole");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 0, "the window-0's own id");
                assert_eq!(c.state_id, 4, "the revision rides");
                assert_eq!(c.slot, 10, "the hovered seat's own menu id");
                assert_eq!(c.button, 2, "the middle press the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Clone);
                assert!(
                    c.changed.is_empty(),
                    "the slot-mutation silence the EMPTY map keeps"
                );
                assert_eq!(c.carried, (stone, 64), "the safeClone pair rides");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE CONTAINER SCREENS' OWN CLONE, THE FOUR MENUS (the route the
    /// screen match feeds): the furnace's offline sim the seats' own
    /// stores read (the fuel cell the sim array answers, the standard
    /// row the write-through projection IS), the wire lanes the packet
    /// the window's own identity names — the chest's grid seat, the
    /// table's craft cell — the cursor-alone law every lane wears.
    #[test]
    fn the_container_clone_arm_the_furnace_the_chest_the_table_wear() {
        use crate::world_view::{ContainerCommand, LiveContainer};
        // THE FURNACE'S OFFLINE SIM (the fresh stand-in's own lane)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open(ScreenKind::Furnace);
        app.furnace_panel = inventory_panel_origin(640, 360);
        app.game_mode = crate::screens::GameMode::Creative;
        let coal = mc_registry::item_id_by_name("coal").unwrap();
        let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        app.furnace_slots = vec![(raw_iron, 7), (coal, 5), (0, 0)];
        app.furnace_slot_rects = vec![
            (100, 100, 0),
            (100, 130, 1),
            (200, 100, 2),
            (300, 300, 5), // the standard row menu 5
        ];
        // THE FUEL CELL'S CLONE (the sim array's own read)
        app.on_container_clone_click(108, 138);
        assert_eq!(app.furnace_carried, (coal, 64), "the fuel's own max");
        assert_eq!(app.furnace_slots[1], (coal, 5), "the cell whole");
        app.furnace_carried = (0, 0);
        // THE STANDARD ROW'S WRITE-THROUGH READ (the projection the
        // inventory IS — the row's stack the inventory's cell answers)
        app.inventory.items[11] = (stone, 9);
        app.on_container_clone_click(308, 308);
        assert_eq!(app.furnace_carried, (stone, 64), "the row's own read");
        assert_eq!(app.inventory.items[11], (stone, 9), "the row whole");
        // THE RESULT SEAT'S EMPTINESS (the hasItem gate): the sim's
        // result (0, 0) — the silence the cursor keeps
        app.furnace_carried = (0, 0);
        app.on_container_clone_click(208, 108);
        assert_eq!(app.furnace_carried, (0, 0), "the empty seat the silence keeps");
        // THE FURNACE'S WIRE LANE (the 39-slot belief the packet names)
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open(ScreenKind::Furnace);
        app.game_mode = crate::screens::GameMode::Creative;
        app.furnace_is_live = true;
        app.furnace_live_window = Some((6, 8));
        app.furnace_slot_rects = vec![(100, 130, 1)];
        app.furnace_slots = vec![(0, 0), (coal, 5), (0, 0)];
        app.on_container_clone_click(108, 138);
        assert_eq!(app.live_furnace_carried, (coal, 64), "the belief's cursor rides");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 6, "the window's own id");
                assert_eq!(c.state_id, 8, "the revision rides");
                assert_eq!(c.slot, 1, "the fuel seat's own menu id");
                assert_eq!(c.button, 2, "the middle press the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Clone);
                assert!(c.changed.is_empty(), "the EMPTY map");
                assert_eq!(c.carried, (coal, 64), "the safeClone pair");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE CHEST'S WIRE LANE (the grid seat the packet names)
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open(ScreenKind::Chest);
        app.game_mode = crate::screens::GameMode::Creative;
        app.chest_is_live = true;
        app.chest_live_window = Some((5, 7));
        let panel = chest_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        app.chest_slots = vec![(0, 0); 27];
        app.chest_mirror = vec![(0, 0); 36];
        app.chest_slots[4] = (dirt, 32);
        app.chest_slot_rects = (0..27)
            .map(|i| {
                let (sx, sy) = chest_container_slot(panel, i);
                (sx + 1, sy + 1, i as i16)
            })
            .chain((0..36).map(|i| {
                let (sx, sy) = chest_mirror_slot(panel, i);
                (sx + 1, sy + 1, (27 + i) as i16)
            }))
            .collect();
        let (gx, gy) = chest_container_slot(panel, 4);
        app.on_container_clone_click(gx + 9, gy + 9);
        assert_eq!(app.chest_carried, (dirt, 64), "the cursor rides");
        assert_eq!(app.chest_slots[4], (dirt, 32), "the seat whole");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 5, "the window's own id");
                assert_eq!(c.state_id, 7, "the revision rides");
                assert_eq!(c.slot, 4, "the grid cell's own menu id");
                assert_eq!(c.button, 2, "the middle press the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Clone);
                assert!(c.changed.is_empty(), "the EMPTY map");
                assert_eq!(c.carried, (dirt, 64), "the safeClone pair");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE TABLE'S WIRE LANE (the craft cell the window-1 belief
        // answers — the live view the screen and the window arm)
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let table = LiveContainer {
            id: 9,
            menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
            slots: {
                let mut s = vec![(0i32, 0i32); 46];
                s[1] = (log, 4); // the craft cell 0 → menu 1
                s
            },
            carried: (0, 0),
            state_id: 3,
            bundle_rows: Vec::new(),
            data_slots: Vec::new(),
            title_nbt: Vec::new(),
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
        assert_eq!(app.screen, ScreenKind::Crafting, "the window shows");
        app.table_panel = inventory_panel_origin(640, 360);
        app.game_mode = crate::screens::GameMode::Creative;
        let (cx, cy) = table_craft_origin(app.table_panel, 0);
        app.on_container_clone_click(cx + 8, cy + 8);
        assert_eq!(app.live_table_carried, (log, 64), "the cursor rides");
        assert_eq!(app.live_table_slots[1], (log, 4), "the cell whole");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 9, "the window's own id");
                assert_eq!(c.state_id, 3, "the revision rides");
                assert_eq!(c.slot, 1, "the craft cell's own menu id");
                assert_eq!(c.button, 2, "the middle press the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Clone);
                assert!(c.changed.is_empty(), "the EMPTY map");
                assert_eq!(c.carried, (log, 64), "the safeClone pair");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE CONTAINER SCREENS' OWN CLONE, THE GATES (the mouseClicked
    /// walk's own chain): the flag-less middle press the WHOLE walk
    /// silences (pcs 92-104 — the survival mode no seat walk, no
    /// packet), the carried cursor the QuickCraft start the client
    /// keeps stopped (the silence the cursor pair wears), the empty
    /// seat and the between-slots press the hasItem / null-hover gates
    /// keep, the offline chest stand-in and the wire carried gate the
    /// packet lane never passes.
    #[test]
    fn the_container_clone_arm_the_gates_and_the_silences_wear() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        app.inventory.items[12] = (dirt, 3);
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 12);
        // THE FLAG-LESS MIDDLE PRESS (pcs 92-104): the survival mode
        // the whole walk skips — no cursor, no mutation
        app.on_container_clone_click(ox + 8, oy + 8);
        assert_eq!(app.inv_carried, (0, 0), "the flag-less press the silence keeps");
        assert_eq!(app.inventory.items[12], (dirt, 3), "the seat whole");
        // THE CARRIED CURSOR (the QuickCraft start the client keeps
        // stopped): the walk the silence wears
        app.game_mode = crate::screens::GameMode::Creative;
        app.inv_carried = (dirt, 5);
        app.on_container_clone_click(ox + 8, oy + 8);
        assert_eq!(app.inv_carried, (dirt, 5), "the cursor whole");
        assert_eq!(app.inventory.items[12], (dirt, 3), "the seat whole");
        // THE EMPTY SEAT (the hasItem gate): store 14 empty — silence
        app.inv_carried = (0, 0);
        let (ex, ey) = inventory_slot_origin(app.inv_panel, 14);
        app.on_container_clone_click(ex + 8, ey + 8);
        assert_eq!(app.inv_carried, (0, 0), "the empty seat the silence keeps");
        // THE BETWEEN-SLOTS PRESS (the null hover): the panel's spine
        app.on_container_clone_click(app.inv_panel.0 + 88, app.inv_panel.1 + 3);
        assert_eq!(app.inv_carried, (0, 0), "still silence");
        // THE CHEST'S OFFLINE STAND-IN (no window armed): no packet
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open(ScreenKind::Chest);
        app.game_mode = crate::screens::GameMode::Creative;
        let panel = chest_panel_origin(640, 360);
        app.chest_slots = vec![(0, 0); 27];
        app.chest_mirror = vec![(0, 0); 36];
        app.chest_slots[4] = (dirt, 32);
        app.chest_slot_rects = (0..27)
            .map(|i| {
                let (sx, sy) = chest_container_slot(panel, i);
                (sx + 1, sy + 1, i as i16)
            })
            .collect();
        let (gx, gy) = chest_container_slot(panel, 4);
        app.on_container_clone_click(gx + 9, gy + 9);
        assert!(
            click_rx.try_recv().is_err(),
            "the offline stand-in is silence"
        );
        // THE CARRIED GATE (the wire lane): the cursor non-empty — no
        // packet
        app.chest_is_live = true;
        app.chest_live_window = Some((5, 7));
        app.chest_carried = (dirt, 1);
        app.on_container_clone_click(gx + 9, gy + 9);
        assert!(
            click_rx.try_recv().is_err(),
            "the carried gate the packet lane never passes"
        );
        assert_eq!(app.chest_carried, (dirt, 1), "the cursor whole");
    }


    /// THE CHEST'S OWN SWAP, THE TABLE (the doClick SWAP walk over the
    /// generic_9x3's belief): CASE B the hovered grid stack rides the
    /// empty hotbar row AS-IS (the cell empties, the HUD row synced,
    /// the packet the window's own identity + the walk's own
    /// mutations names), the CASE D straight trade the mirror rows
    /// wears (the row the hovered stack takes, the cell the row's
    /// stack wears — the changed vec the mutation order speaks).
    #[test]
    fn the_chest_swap_key_the_table_the_hud_wear() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open(ScreenKind::Chest);
        app.chest_live_window = Some((5, 7));
        let panel = chest_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        app.chest_slots = vec![(0, 0); 27];
        app.chest_mirror = vec![(0, 0); 36];
        app.chest_slots[4] = (dirt, 32);
        app.chest_slot_rects = (0..27)
            .map(|i| {
                let (sx, sy) = chest_container_slot(panel, i);
                (sx + 1, sy + 1, i as i16)
            })
            .chain((0..36).map(|i| {
                let (sx, sy) = chest_mirror_slot(panel, i);
                (sx + 1, sy + 1, (27 + i) as i16)
            }))
            .collect();
        // CASE B: THE GRID STACK OVER THE EMPTY HOTBAR ROW 2 (menu 56)
        let (gx, gy) = chest_container_slot(panel, 4);
        app.mouse_pos = Some((gx + 9, gy + 9));
        assert!(app.on_chest_swap_key(2), "the digit consumes");
        assert_eq!(app.chest_slots[4], (0, 0), "the cell emptied");
        assert_eq!(app.chest_mirror[29], (dirt, 32), "the row rides AS-IS");
        assert_eq!(app.hotbar_slots[2], (dirt, 32), "the HUD synced");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 5, "the window's own id");
                assert_eq!(c.state_id, 7, "the revision rides");
                assert_eq!(c.slot, 4, "the hovered seat's own menu id");
                assert_eq!(c.button, 2, "the digit the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Swap);
                assert_eq!(
                    c.changed,
                    vec![(4, (0, 0)), (56, (dirt, 32))],
                    "the mutations named"
                );
                assert_eq!(c.carried, (0, 0), "the cursor pair rides unchanged");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE CASE D STRAIGHT TRADE: THE MIRROR MAIN ROW (menu 30)
        // OVER THE HOTBAR ROW 0 (menu 54)
        app.chest_mirror[3] = (stone, 5);
        app.chest_mirror[27] = (planks, 7);
        let (mx2, my2) = chest_mirror_slot(panel, 3);
        app.mouse_pos = Some((mx2 + 9, my2 + 9));
        assert!(app.on_chest_swap_key(0));
        assert_eq!(app.chest_mirror[3], (planks, 7), "the cell wears the row's stack");
        assert_eq!(app.chest_mirror[27], (stone, 5), "the row takes the hovered's");
        assert_eq!(app.hotbar_slots[0], (stone, 5), "the HUD synced");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 30, "the mirror row's own menu id");
                assert_eq!(
                    c.changed,
                    vec![(54, (stone, 5)), (30, (planks, 7))],
                    "the trade's mutations named"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE CHEST'S OWN SWAP, THE SPLIT LAW (the max the SPLIT wears):
    /// CASE C the row's 20 eggs over the empty cell — the cell takes
    /// the max 16, the row the remainder 4 keeps; CASE D the row 20
    /// eggs over the hovered 30 dirt — the cell takes 16, the row
    /// keeps 4, the ORIGINAL hovered stack the add rides (the chest
    /// belief's own scan: the selected row refused — the eggs' id —
    /// the merge scan refuses, the FIRST-EMPTY HOTBAR ROW lands the
    /// 30 whole — the leftover none).
    #[test]
    fn the_chest_swap_key_the_split_law_and_the_add_wear() {
        use crate::world_view::ContainerCommand;
        let egg = crate::mob::wire_id("egg").expect("the egg's own id");
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open(ScreenKind::Chest);
        app.chest_live_window = Some((5, 7));
        let panel = chest_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        app.chest_slots = vec![(0, 0); 27];
        app.chest_mirror = vec![(0, 0); 36];
        app.chest_mirror[30] = (egg, 20); // the hotbar row 3 — menu 57
        app.chest_slot_rects = (0..27)
            .map(|i| {
                let (sx, sy) = chest_container_slot(panel, i);
                (sx + 1, sy + 1, i as i16)
            })
            .chain((0..36).map(|i| {
                let (sx, sy) = chest_mirror_slot(panel, i);
                (sx + 1, sy + 1, (27 + i) as i16)
            }))
            .collect();
        // CASE C: THE EMPTY CELL TAKES THE MAX 16
        let (gx, gy) = chest_container_slot(panel, 9);
        app.mouse_pos = Some((gx + 9, gy + 9));
        assert!(app.on_chest_swap_key(3));
        assert_eq!(app.chest_slots[9], (egg, 16), "the cell takes the max");
        assert_eq!(app.chest_mirror[30], (egg, 4), "the row the remainder keeps");
        assert_eq!(app.hotbar_slots[3], (egg, 4), "the HUD synced");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 9, "the hovered cell's own menu id");
                assert_eq!(
                    c.changed,
                    vec![(9, (egg, 16)), (57, (egg, 4))],
                    "the split's mutations named"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // CASE D SPLIT: THE ROW 20 EGGS OVER THE HOVERED 30 DIRT — THE
        // ADD RIDES (the dirt the FIRST-EMPTY HOTBAR ROW lands — menu 55)
        app.chest_mirror[27] = (egg, 20);
        app.chest_slots[12] = (dirt, 30);
        let (dx, dy) = chest_container_slot(panel, 12);
        app.mouse_pos = Some((dx + 9, dy + 9));
        assert!(app.on_chest_swap_key(0));
        assert_eq!(app.chest_slots[12], (egg, 16), "the cell takes the max");
        assert_eq!(app.chest_mirror[27], (egg, 4), "the row the remainder keeps");
        assert_eq!(app.chest_mirror[28], (dirt, 30), "the add the first-empty row lands");
        assert_eq!(app.hotbar_slots[1], (dirt, 30), "the HUD synced");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(
                    c.changed,
                    vec![(12, (egg, 16)), (54, (egg, 4)), (55, (dirt, 30))],
                    "the split + the add's mutations named"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE CHEST'S OWN SWAP, THE GATES (the checkHotbarKeyPressed
    /// chain's own shape): the BOTH-EMPTY press the consumed silence
    /// keeps AND the packet rides (the empty changed map the
    /// untouched state names), the SAME-SEAT press (the hovered slot
    /// IS the button's own row) the identical pair sets back — no
    /// mutation the diff ever names, the carried cursor the false
    /// BEFORE the packet lane keeps, the offline stand-in and the
    /// between-slots press the refusals keep.
    #[test]
    fn the_chest_swap_key_the_gates_and_the_always_packet_wear() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open(ScreenKind::Chest);
        let panel = chest_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        app.chest_slots = vec![(0, 0); 27];
        app.chest_mirror = vec![(0, 0); 36];
        app.chest_slots[4] = (dirt, 32);
        app.chest_slot_rects = (0..27)
            .map(|i| {
                let (sx, sy) = chest_container_slot(panel, i);
                (sx + 1, sy + 1, i as i16)
            })
            .chain((0..36).map(|i| {
                let (sx, sy) = chest_mirror_slot(panel, i);
                (sx + 1, sy + 1, (27 + i) as i16)
            }))
            .collect();
        let (gx, gy) = chest_container_slot(panel, 4);
        // THE OFFLINE STAND-IN (no window armed): the silence
        app.mouse_pos = Some((gx + 9, gy + 9));
        assert!(!app.on_chest_swap_key(0), "no window — the false wears");
        assert!(click_rx.try_recv().is_err(), "the offline stand-in is silence");
        // THE WINDOW ARMS
        app.chest_live_window = Some((5, 7));
        // THE BOTH-EMPTY PRESS: the digit CONSUMES and the packet
        // rides with the EMPTY changed map
        let (ex, ey) = chest_container_slot(panel, 20);
        app.mouse_pos = Some((ex + 9, ey + 9));
        assert!(app.on_chest_swap_key(5), "the silence still consumes");
        match click_rx.try_recv().expect("the always packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 20, "the hovered seat's own menu id");
                assert_eq!(c.button, 5, "the digit the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Swap);
                assert!(c.changed.is_empty(), "the untouched state keeps");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE SAME-SEAT PRESS (the hotbar row 0 IS menu 54): the
        // identical pair sets back — no mutation, the belief whole
        app.chest_mirror[27] = (stone, 7);
        let (mx2, my2) = chest_mirror_slot(panel, 27);
        app.mouse_pos = Some((mx2 + 9, my2 + 9));
        assert!(app.on_chest_swap_key(0), "the same seat still consumes");
        assert_eq!(app.chest_mirror[27], (stone, 7), "the belief whole");
        assert_eq!(app.hotbar_slots[0], (stone, 7), "the HUD whole");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 54, "the same seat's own menu id");
                assert!(c.changed.is_empty(), "no mutation the diff names");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE CARRIED CURSOR (the getCarried().isEmpty() gate): the
        // false BEFORE the packet lane — NO packet
        app.mouse_pos = Some((gx + 9, gy + 9));
        app.chest_carried = (dirt, 5);
        assert!(!app.on_chest_swap_key(0), "the carried gate refuses");
        assert!(click_rx.try_recv().is_err(), "no packet the gate passes");
        app.chest_carried = (0, 0);
        // THE BETWEEN-SLOTS PRESS (the null hoveredSlot): the refusal
        app.mouse_pos = Some((panel.0 + 170, panel.1 + 20));
        assert!(!app.on_chest_swap_key(0), "the null hover refuses");
        assert!(click_rx.try_recv().is_err(), "still silence");
    }


    /// THE CONTAINER SCREENS' OWN F ARM (the checkHotbarKeyPressed's
    /// own FIRST arm — the keySwapOffhand match the
    /// `slotClicked(hovered, hovered.index, 40, SWAP)` walk rides; the
    /// doClick SWAP table's `inventory.getItem(40)` the player
    /// Inventory's OFFHAND answers — the outside-menu seat the changed
    /// map can't name): CASE B the chest grid's stack rides the
    /// offhand AS-IS (the cell EMPTIES, the changed map the hovered
    /// seat ALONE names); the CASE D straight trades the chest mirror
    /// row and the furnace main store row over the offhand wear.
    #[test]
    fn the_container_f_arm_the_offhand_row_and_the_changed_map_wear() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open(ScreenKind::Chest);
        app.chest_live_window = Some((5, 7));
        let panel = chest_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        app.chest_slots = vec![(0, 0); 27];
        app.chest_mirror = vec![(0, 0); 36];
        app.chest_slots[4] = (dirt, 32);
        app.chest_slot_rects = (0..27)
            .map(|i| {
                let (sx, sy) = chest_container_slot(panel, i);
                (sx + 1, sy + 1, i as i16)
            })
            .chain((0..36).map(|i| {
                let (sx, sy) = chest_mirror_slot(panel, i);
                (sx + 1, sy + 1, (27 + i) as i16)
            }))
            .collect();
        assert_eq!(app.live_inv_slots[45], (0, 0), "the offhand starts empty");
        // CASE B: THE GRID STACK OVER THE EMPTY OFFHAND (button 40)
        let (gx, gy) = chest_container_slot(panel, 4);
        app.mouse_pos = Some((gx + 9, gy + 9));
        assert!(app.on_chest_swap_key(40), "the F arm consumes");
        assert_eq!(app.chest_slots[4], (0, 0), "the cell emptied");
        assert_eq!(app.live_inv_slots[45], (dirt, 32), "the offhand rides AS-IS");
        assert_eq!(app.hotbar_slots[2], (0, 0), "the HUD rows untouched");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 5, "the window's own id");
                assert_eq!(c.state_id, 7, "the revision rides");
                assert_eq!(c.slot, 4, "the hovered seat's own menu id");
                assert_eq!(c.button, 40, "the offhand key the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Swap);
                assert_eq!(
                    c.changed,
                    vec![(4, (0, 0))],
                    "the outside-menu seat the changed map can't name"
                );
                assert_eq!(c.carried, (0, 0), "the cursor pair rides unchanged");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE CASE D STRAIGHT TRADE: THE MIRROR MAIN ROW (menu 30)
        // OVER THE OFFHAND'S PLANKS
        app.chest_mirror[3] = (stone, 5);
        app.live_inv_slots[45] = (planks, 7);
        let (mx2, my2) = chest_mirror_slot(panel, 3);
        app.mouse_pos = Some((mx2 + 9, my2 + 9));
        assert!(app.on_chest_swap_key(40));
        assert_eq!(app.chest_mirror[3], (planks, 7), "the cell wears the offhand's");
        assert_eq!(app.live_inv_slots[45], (stone, 5), "the offhand takes the hovered's");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 30, "the mirror row's own menu id");
                assert_eq!(
                    c.changed,
                    vec![(30, (planks, 7))],
                    "the hovered seat alone the map names"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE FURNACE'S OWN F ARM: THE MAIN STORE 4 (menu 7) OVER THE
        // OFFHAND'S STONE
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let furnace = crate::world_view::LiveContainer {
            id: 7,
            menu_type_id: crate::world_model::MENU_TYPE_FURNACE,
            slots: {
                let mut s = vec![(0i32, 0i32); 39];
                s[7] = (stone, 5); // the main store 4 — menu 7
                s
            },
            carried: (0, 0),
            state_id: 2,
            bundle_rows: Vec::new(),
            data_slots: Vec::new(),
            title_nbt: Vec::new(),
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(furnace));
        assert_eq!(app.screen, ScreenKind::Furnace, "the window shows");
        app.live_inv_slots[45] = (planks, 7);
        app.furnace_slot_rects = vec![(100, 100, 7)];
        app.mouse_pos = Some((108, 108));
        assert!(app.on_furnace_swap_key(40), "the F arm consumes");
        assert_eq!(app.furnace_slots[7], (planks, 7), "the cell wears the offhand's");
        assert_eq!(app.live_inv_slots[45], (stone, 5), "the offhand takes the hovered's");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 7, "the main store's own menu id");
                assert_eq!(c.button, 40, "the offhand key the byte keys");
                assert_eq!(
                    c.changed,
                    vec![(7, (planks, 7))],
                    "the hovered seat alone the map names"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE CONTAINER SCREENS' OWN F ARM, THE VERDICTS AND THE SPLIT
    /// (the seats' own mayPlace pair the SWAP table keys): the
    /// furnace INPUT seat the SMELTABLE verdict answers (the 70
    /// raw_iron the SPLIT law the max 64 wears — the input takes the
    /// max, the offhand the remainder 6 keeps), the FUEL seat the
    /// isFuel verdict (the coal places, the offhand empties), the
    /// RESULT seat's mayPlace iconst_0 the place arms refuse (the
    /// packet rides, the belief whole); the TABLE's result seat the
    /// CASE B ride the onTake walk wears (the ingredient decrement +
    /// the recompute refill, the offhand the result takes).
    #[test]
    fn the_container_f_arm_the_verdicts_and_the_split_wear() {
        use crate::world_view::{ContainerCommand, LiveContainer};
        let coal = mc_registry::item_id_by_name("coal").unwrap();
        let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let furnace = LiveContainer {
            id: 7,
            menu_type_id: crate::world_model::MENU_TYPE_FURNACE,
            slots: {
                let mut s = vec![(0i32, 0i32); 39];
                s[2] = (mc_registry::item_id_by_name("raw_iron").unwrap(), 2); // the result seat
                s
            },
            carried: (0, 0),
            state_id: 2,
            bundle_rows: Vec::new(),
            data_slots: Vec::new(),
            title_nbt: Vec::new(),
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(furnace));
        app.furnace_slot_rects = vec![(100, 100, 0), (140, 100, 1), (180, 100, 2)];
        // THE INPUT SEAT'S SPLIT (CASE C): THE OFFHAND'S 70 RAW_IRON —
        // THE SMELTABLE VERDICT PASSES, THE INPUT TAKES THE MAX 64
        app.live_inv_slots[45] = (raw_iron, 70);
        app.mouse_pos = Some((108, 108));
        assert!(app.on_furnace_swap_key(40));
        assert_eq!(app.furnace_slots[0], (raw_iron, 64), "the input takes the max");
        assert_eq!(app.live_inv_slots[45], (raw_iron, 6), "the offhand the remainder keeps");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(
                    c.changed,
                    vec![(0, (raw_iron, 64))],
                    "the offhand's remainder no entry names"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE FUEL SEAT (CASE C): THE OFFHAND'S COAL PLACES WHOLE
        app.live_inv_slots[45] = (coal, 12);
        app.mouse_pos = Some((148, 108));
        assert!(app.on_furnace_swap_key(40));
        assert_eq!(app.furnace_slots[1], (coal, 12), "the fuel takes the whole");
        assert_eq!(app.live_inv_slots[45], (0, 0), "the offhand empties");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.changed, vec![(1, (coal, 12))], "the fuel's landing named");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE RESULT SEAT (CASE C): THE mayPlace iconst_0 REFUSES —
        // THE PACKET RIDES, THE BELIEF WHOLE
        app.live_inv_slots[45] = (coal, 3);
        app.mouse_pos = Some((188, 108));
        assert!(app.on_furnace_swap_key(40), "the refusal still consumes");
        assert_eq!(app.furnace_slots[2].1, 2, "the result whole");
        assert_eq!(app.live_inv_slots[45], (coal, 3), "the offhand whole");
        match click_rx.try_recv().expect("the always packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 2, "the result seat's own menu id");
                assert!(c.changed.is_empty(), "the refusal no mutation names");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE TABLE'S RESULT SEAT (CASE B): THE RESULT RIDES THE
        // OFFHAND, THE ONTAKE WALK THE RECOMPUTE WEARS (the 2-log
        // cell the one log keeps, the recipe the refill re-speaks)
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let table = LiveContainer {
            id: 9,
            menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
            slots: {
                let mut s = vec![(0i32, 0i32); 46];
                s[0] = (planks, 4); // the result — the one-log recipe's own match
                s[1] = (log, 2); // the craft cell
                s
            },
            carried: (0, 0),
            state_id: 3,
            bundle_rows: Vec::new(),
            data_slots: Vec::new(),
            title_nbt: Vec::new(),
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
        assert_eq!(app.screen, ScreenKind::Crafting, "the window shows");
        app.table_panel = inventory_panel_origin(640, 360);
        let panel = inventory_panel_origin(640, 360);
        let (rx, ry) = table_craft_origin(panel, 9);
        app.mouse_pos = Some((rx + 9, ry + 9));
        assert!(app.on_table_swap_key(40), "the F arm consumes");
        assert_eq!(app.live_inv_slots[45], (planks, 4), "the result rides the offhand");
        assert_eq!(app.live_table_slots[1], (log, 1), "the ingredient the one off keeps");
        assert_eq!(
            app.live_table_slots[0],
            (planks, 4),
            "the recompute the result refills"
        );
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 9, "the window's own id");
                assert_eq!(c.slot, 0, "the result seat's own menu id");
                assert_eq!(
                    c.changed,
                    vec![(0, (planks, 4)), (1, (log, 1))],
                    "the ride + the decrement + the refill named (the offhand no entry)"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE CONTAINER SCREENS' OWN F ARM, THE GATES AND THE ALWAYS
    /// PACKET (the checkHotbarKeyPressed walk's own gates the chain
    /// keeps): the offline stand-in the false keeps (the world lane
    /// the open screen's silence answers), the both-empty press the
    /// consumed silence and the EMPTY packet ride, the carried cursor
    /// the false BEFORE the packet lane keeps, the null hover the
    /// refusal wears.
    #[test]
    fn the_container_f_arm_the_gates_and_the_always_packet_wear() {
        use crate::world_view::ContainerCommand;
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open(ScreenKind::Chest);
        let panel = chest_panel_origin(640, 360);
        app.chest_slots = vec![(0, 0); 27];
        app.chest_mirror = vec![(0, 0); 36];
        app.chest_slot_rects = (0..27)
            .map(|i| {
                let (sx, sy) = chest_container_slot(panel, i);
                (sx + 1, sy + 1, i as i16)
            })
            .chain((0..36).map(|i| {
                let (sx, sy) = chest_mirror_slot(panel, i);
                (sx + 1, sy + 1, (27 + i) as i16)
            }))
            .collect();
        // THE OFFLINE STAND-IN (no window armed): the false — and the
        // world lane the open screen's gate keeps (the offhand whole)
        let (gx, gy) = chest_container_slot(panel, 4);
        app.mouse_pos = Some((gx + 9, gy + 9));
        assert!(!app.on_chest_swap_key(40), "no window — the false wears");
        app.on_swap_offhand_key();
        assert_eq!(app.live_inv_slots[45], (0, 0), "the world lane the screen keeps");
        assert!(click_rx.try_recv().is_err(), "the offline stand-in is silence");
        // THE WINDOW ARMS
        app.chest_live_window = Some((5, 7));
        // THE BOTH-EMPTY PRESS: the F arm CONSUMES and the packet
        // rides with the EMPTY changed map
        let (ex, ey) = chest_container_slot(panel, 20);
        app.mouse_pos = Some((ex + 9, ey + 9));
        assert!(app.on_chest_swap_key(40), "the silence still consumes");
        match click_rx.try_recv().expect("the always packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 20, "the hovered seat's own menu id");
                assert_eq!(c.button, 40, "the offhand key the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Swap);
                assert!(c.changed.is_empty(), "the untouched state keeps");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE CARRIED CURSOR (the getCarried().isEmpty() gate): the
        // false BEFORE the packet lane — NO packet
        app.mouse_pos = Some((gx + 9, gy + 9));
        app.chest_carried = (dirt, 5);
        assert!(!app.on_chest_swap_key(40), "the carried gate refuses");
        assert!(click_rx.try_recv().is_err(), "no packet the gate passes");
        app.chest_carried = (0, 0);
        // THE BETWEEN-SLOTS PRESS (the null hoveredSlot): the refusal
        app.mouse_pos = Some((panel.0 + 170, panel.1 + 20));
        assert!(!app.on_chest_swap_key(40), "the null hover refuses");
        assert!(click_rx.try_recv().is_err(), "still silence");
    }


    /// THE TABLE'S OWN SWAP, THE TABLE THE HUD WEARS (the doClick SWAP
    /// table over the window-1 belief): CASE B the craft cell's stack
    /// rides the button row AS-IS (the cell EMPTIES, the HUD row the
    /// every-mutation law syncs); the CASE D straight trade the main
    /// store row over the hotbar row wears (the row the hovered's
    /// takes, the cell the row's stack wears).
    #[test]
    fn the_table_swap_key_the_table_the_hud_wear() {
        use crate::world_view::{ContainerCommand, LiveContainer};
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        let table = LiveContainer {
            id: 9,
            menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
            slots: {
                let mut s = vec![(0i32, 0i32); 46];
                s[2] = (dirt, 32); // the craft cell 1 → menu 2
                s[10] = (stone, 5); // the main store 9 → menu 10
                s[37] = (planks, 7); // the hotbar row 0 → menu 37
                s
            },
            carried: (0, 0),
            state_id: 3,
            bundle_rows: Vec::new(),
            data_slots: Vec::new(),
            title_nbt: Vec::new(),
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
        assert_eq!(app.screen, ScreenKind::Crafting, "the window shows");
        app.table_panel = inventory_panel_origin(640, 360);
        // CASE B: THE CRAFT CELL'S STACK OVER THE EMPTY HOTBAR ROW 2
        // (menu 39)
        let (cx, cy) = table_craft_origin(app.table_panel, 1);
        app.mouse_pos = Some((cx + 8, cy + 8));
        assert!(app.on_table_swap_key(2), "the digit consumes");
        assert_eq!(app.live_table_slots[2], (0, 0), "the cell emptied");
        assert_eq!(app.live_table_slots[39], (dirt, 32), "the row rides AS-IS");
        assert_eq!(app.hotbar_slots[2], (dirt, 32), "the HUD synced");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 9, "the window's own id");
                assert_eq!(c.state_id, 3, "the revision rides");
                assert_eq!(c.slot, 2, "the hovered cell's own menu id");
                assert_eq!(c.button, 2, "the digit the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Swap);
                assert_eq!(
                    c.changed,
                    vec![(2, (0, 0)), (39, (dirt, 32))],
                    "the mutations named"
                );
                assert_eq!(c.carried, (0, 0), "the cursor pair rides unchanged");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE CASE D STRAIGHT TRADE: THE MAIN STORE 9 (menu 10) OVER
        // THE HOTBAR ROW 0 (menu 37)
        let (ox, oy) = inventory_slot_origin(app.table_panel, 9);
        app.mouse_pos = Some((ox + 8, oy + 8));
        assert!(app.on_table_swap_key(0));
        assert_eq!(app.live_table_slots[10], (planks, 7), "the cell wears the row's stack");
        assert_eq!(app.live_table_slots[37], (stone, 5), "the row takes the hovered's");
        assert_eq!(app.hotbar_slots[0], (stone, 5), "the HUD synced");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 10, "the main store's own menu id");
                assert_eq!(
                    c.changed,
                    vec![(37, (stone, 5)), (10, (planks, 7))],
                    "the trade's mutations named"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE TABLE'S OWN SWAP, THE SPLIT LAW AND THE RESULT'S ON-TAKE
    /// (the seat's own mayPickup/mayPlace verdicts the table keys):
    /// CASE C the row's 20 eggs over the empty MAIN STORE row — the
    /// cell takes the max 16, the row the remainder 4 keeps (the
    /// store rows the plain Slots answer, the recipe grid the press
    /// never feeds); the RESULT seat's CASE B the mayPickup matches
    /// verdict answers (the result rides the row AS-IS, the onTake's
    /// decrement the ingredient wears, the recompute the result
    /// REFILLS — the belief the recipe alive keeps); the result's
    /// mayPlace-false verdict the CASE D press refuses (the packet
    /// rides, the belief whole).
    #[test]
    fn the_table_swap_key_the_split_law_and_the_result_on_take_wear() {
        use crate::world_view::{ContainerCommand, LiveContainer};
        let egg = crate::mob::wire_id("egg").expect("the egg's own id");
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let table = LiveContainer {
            id: 9,
            menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
            slots: {
                let mut s = vec![(0i32, 0i32); 46];
                s[0] = (planks, 4); // the result seat — the recipe's own verdict
                s[1] = (log, 2); // the craft cell 0 — TWO logs the recompute keeps alive
                s[40] = (egg, 20); // the hotbar row 3 → menu 40
                s
            },
            carried: (0, 0),
            state_id: 3,
            bundle_rows: Vec::new(),
            data_slots: Vec::new(),
            title_nbt: Vec::new(),
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
        app.table_panel = inventory_panel_origin(640, 360);
        // CASE C: THE EMPTY MAIN STORE ROW TAKES THE MAX 16 (the store
        // row the press never feeds the recipe grid — the craft cells
        // the log alone keeps)
        let (ox, oy) = inventory_slot_origin(app.table_panel, 9);
        app.mouse_pos = Some((ox + 8, oy + 8));
        assert!(app.on_table_swap_key(3));
        assert_eq!(app.live_table_slots[10], (egg, 16), "the cell takes the max");
        assert_eq!(app.live_table_slots[40], (egg, 4), "the row the remainder keeps");
        assert_eq!(app.hotbar_slots[3], (egg, 4), "the HUD synced");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 10, "the main store's own menu id");
                assert_eq!(
                    c.changed,
                    vec![(10, (egg, 16)), (40, (egg, 4))],
                    "the split's mutations named"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE RESULT SEAT'S CASE B: THE RESULT RIDES THE ROW AS-IS,
        // THE ON-TAKE THE INGREDIENT DECREMENTS, THE RECOMPUTE THE
        // RESULT REFILLS (the one log the recipe keeps alive)
        let (rx, ry) = table_craft_origin(app.table_panel, 9);
        app.mouse_pos = Some((rx + 8, ry + 8));
        assert!(app.on_table_swap_key(0), "the result press consumes");
        assert_eq!(app.live_table_slots[37], (planks, 4), "the row rides AS-IS");
        assert_eq!(app.live_table_slots[1], (log, 1), "the ingredient the one off wears");
        assert_eq!(app.live_table_slots[0], (planks, 4), "the recompute the result refills");
        assert_eq!(app.hotbar_slots[0], (planks, 4), "the HUD synced");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 0, "the result seat's own menu id");
                assert_eq!(
                    c.changed,
                    vec![(0, (planks, 4)), (37, (planks, 4)), (1, (log, 1))],
                    "the ride + the refill's mutations named"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE RESULT SEAT'S CASE D REFUSAL (the mayPlace iconst_0 the
        // place arms refuse): the packet rides, the belief whole
        app.live_table_slots[38] = (stone, 7); // the hotbar row 1 — menu 38
        app.mouse_pos = Some((rx + 8, ry + 8));
        assert!(app.on_table_swap_key(1), "the refusal still consumes");
        assert_eq!(app.live_table_slots[0], (planks, 4), "the result whole");
        assert_eq!(app.live_table_slots[38], (stone, 7), "the row whole");
        match click_rx.try_recv().expect("the always packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 0, "the result seat's own menu id");
                assert_eq!(c.button, 1, "the digit the byte keys");
                assert!(c.changed.is_empty(), "the refusal no mutation names");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE TABLE'S OWN SWAP, THE GATES (the checkHotbarKeyPressed
    /// chain's own shape over the window-1 belief): the BOTH-EMPTY
    /// press the consumed silence keeps AND the packet rides (the
    /// empty changed map the untouched state names), the SAME-SEAT
    /// press (the hovered hotbar row IS the button's own row) the
    /// identical pair sets back — no mutation the diff ever names,
    /// the carried cursor the false BEFORE the packet lane keeps, the
    /// offline stand-in and the between-slots press the refusals
    /// keep.
    #[test]
    fn the_table_swap_key_the_gates_and_the_always_packet_wear() {
        use crate::world_view::{ContainerCommand, LiveContainer};
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        // THE OFFLINE STAND-IN (no window armed): the silence
        app.open(ScreenKind::Crafting);
        app.table_panel = inventory_panel_origin(640, 360);
        let (cx, cy) = table_craft_origin(app.table_panel, 8);
        app.mouse_pos = Some((cx + 8, cy + 8));
        assert!(!app.on_table_swap_key(0), "no window — the false wears");
        assert!(click_rx.try_recv().is_err(), "the offline stand-in is silence");
        // THE WINDOW ARMS
        let table = LiveContainer {
            id: 9,
            menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
            slots: {
                let mut s = vec![(0i32, 0i32); 46];
                s[41] = (stone, 7); // the hotbar row 4 → menu 41
                s
            },
            carried: (0, 0),
            state_id: 3,
            bundle_rows: Vec::new(),
            data_slots: Vec::new(),
            title_nbt: Vec::new(),
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
        app.table_panel = inventory_panel_origin(640, 360);
        // THE BOTH-EMPTY PRESS: the digit CONSUMES and the packet
        // rides with the EMPTY changed map
        app.mouse_pos = Some((cx + 8, cy + 8));
        assert!(app.on_table_swap_key(0), "the silence still consumes");
        match click_rx.try_recv().expect("the always packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 9, "the hovered cell's own menu id");
                assert_eq!(c.button, 0, "the digit the byte keys");
                assert_eq!(c.input, crate::world_view::ContainerInput::Swap);
                assert!(c.changed.is_empty(), "the untouched state keeps");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE SAME-SEAT PRESS (the hotbar row 4 IS menu 41): the
        // identical pair sets back — no mutation, the belief whole
        let (sx, sy) = inventory_slot_origin(app.table_panel, 4);
        app.mouse_pos = Some((sx + 8, sy + 8));
        assert!(app.on_table_swap_key(4), "the same seat still consumes");
        assert_eq!(app.live_table_slots[41], (stone, 7), "the belief whole");
        assert_eq!(app.hotbar_slots[4], (stone, 7), "the HUD whole");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 41, "the same seat's own menu id");
                assert!(c.changed.is_empty(), "no mutation the diff names");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE CARRIED CURSOR (the getCarried().isEmpty() gate): the
        // false BEFORE the packet lane — NO packet
        app.live_table_carried = (dirt, 5);
        app.mouse_pos = Some((cx + 8, cy + 8));
        assert!(!app.on_table_swap_key(0), "the carried gate refuses");
        assert!(click_rx.try_recv().is_err(), "no packet the gate passes");
        app.live_table_carried = (0, 0);
        // THE BETWEEN-SLOTS PRESS (the null hoveredSlot): the refusal
        app.mouse_pos = Some((app.table_panel.0 + 170, app.table_panel.1 + 20));
        assert!(!app.on_table_swap_key(0), "the null hover refuses");
        assert!(click_rx.try_recv().is_err(), "still silence");
    }


    /// THE SWAP WIRE'S OWN CASE C (the SPLIT law the max wears): the
    /// row 20 eggs over the empty hovered cell — the cell takes the
    /// max 16, the row the remainder 4 keeps; the count <= max arm the
    /// row EMPTIES, the cell the whole wears. The RESULT seat's
    /// mayPlace-false verdict the place arms refuse (the packet rides,
    /// the belief whole — the consumed silence the lane speaks).
    #[test]
    fn the_wire_swap_key_splits_the_max_and_refuses_the_result() {
        use crate::world_view::ContainerCommand;
        let egg = crate::mob::wire_id("egg").expect("the egg's own id");
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // THE SPLIT ARM: the row 0 (menu 36) 20 eggs, the hover the
        // empty store 15 (menu 15), the digit 0
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[36] = (egg, 20);
        app.live_inventory_updated(slots, Vec::new(), 5, (0, 0));
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 15);
        app.mouse_pos = Some((ox + 8, oy + 8));
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.live_inv_slots[15], (egg, 16), "the cell the max takes");
        assert_eq!(app.live_inv_slots[36], (egg, 4), "the row the remainder keeps");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.changed, vec![(15, (egg, 16)), (36, (egg, 4))]);
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE WHOLE ARM: the row 12 stone, the hover the empty store 16
        app.live_inv_slots[36] = (stone, 12);
        let (ox2, oy2) = inventory_slot_origin(app.inv_panel, 16);
        app.mouse_pos = Some((ox2 + 8, oy2 + 8));
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.live_inv_slots[16], (stone, 12), "the cell the whole wears");
        assert_eq!(app.live_inv_slots[36], (0, 0), "the row empties");
        assert_eq!(app.hotbar_slots[0], (0, 0), "the HUD belief synced");
        click_rx.try_recv().expect("the packet rode");
        // THE RESULT SEAT'S REFUSAL (the mayPlace-false slot 0 — the
        // place arms refuse): the believed result 8 planks, the row 4
        // dirt, the digit 0 — the consumed silence, the belief whole,
        // the packet STILL rides
        app.live_inv_slots[0] = (planks, 8);
        app.live_inv_slots[36] = (stone, 4);
        let (rx, ry) = craft_slot_origin(app.inv_panel, 4);
        app.mouse_pos = Some((rx + 8, ry + 8));
        assert!(app.on_inventory_swap_key(0), "the refusal consumes");
        assert_eq!(app.live_inv_slots[0], (planks, 8), "the result whole");
        assert_eq!(app.live_inv_slots[36], (stone, 4), "the row whole");
        match click_rx.try_recv().expect("the always packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 0, "the result's own menu id");
                assert!(c.changed.is_empty(), "the refusal keeps no entry");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE SWAP WIRE'S OWN CASE D (both filled — the mayPlace gate
    /// answers): the count <= max arm the STRAIGHT TRADE wears (the two
    /// stacks trade), the count > max arm the split(max) + the ORIGINAL
    /// hovered stack the BELIEF's add rides (the add's merge-then-
    /// first-empty law over the window-0 rows — the row's own remaining
    /// space the ITEM'S OWN MAX caps, the overflow the fresh slots
    /// wear, the FULL store the stall speaks —
    /// the leftover the belief's honest tail, the server's drop lane
    /// the correction speaks).
    #[test]
    fn the_wire_swap_key_trades_and_splits_with_the_add() {
        use crate::world_view::ContainerCommand;
        let egg = crate::mob::wire_id("egg").expect("the egg's own id");
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
        let cobble = mc_registry::item_id_by_name("cobblestone").unwrap();
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // THE STRAIGHT TRADE: the row 0 (menu 36) 12 stone, the hover
        // store 14 (menu 14) 32 dirt, the digit 0 — the stacks trade
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[36] = (stone, 12);
        slots[14] = (dirt, 32);
        app.live_inventory_updated(slots, Vec::new(), 6, (0, 0));
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 14);
        app.mouse_pos = Some((ox + 8, oy + 8));
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.live_inv_slots[14], (stone, 12), "the cell the row's stack wears");
        assert_eq!(app.live_inv_slots[36], (dirt, 32), "the row the hover's stack takes");
        click_rx.try_recv().expect("the packet rode");
        // THE SPLIT + THE ADD'S MERGE: the row 20 eggs, the hover 30
        // eggs — the cell takes 16, the row keeps 4, the add's scan
        // finds the row's own remaining space the ITEM'S OWN MAX caps
        // (4 + 12 = 16), the overflow the fresh slots wear (16 + 2) —
        // the row's entry the map's own put replaces
        app.live_inv_slots[14] = (0, 0);
        app.live_inv_slots[36] = (egg, 20);
        app.live_inv_slots[15] = (egg, 30);
        let (ox2, oy2) = inventory_slot_origin(app.inv_panel, 15);
        app.mouse_pos = Some((ox2 + 8, oy2 + 8));
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.live_inv_slots[15], (egg, 16), "the cell the max takes");
        assert_eq!(
            app.live_inv_slots[36],
            (egg, 16),
            "the add's merge the row's remaining space fills"
        );
        assert_eq!(
            app.live_inv_slots[37], (egg, 16),
            "the overflow the first fresh slot wears"
        );
        assert_eq!(
            app.live_inv_slots[38], (egg, 2),
            "the tail the second fresh slot keeps"
        );
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(
                    c.changed,
                    vec![
                        (15, (egg, 16)),
                        (36, (egg, 16)),
                        (37, (egg, 16)),
                        (38, (egg, 2)),
                    ],
                    "the repeated slot the map's own put replaces"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE SPLIT + THE ADD'S STALL: the row 3 helmets (the max 1),
        // the hover 30 stone, EVERY row full — the cell takes 1, the
        // row keeps 2, the add's scan finds no home (no merge, no free
        // slot), the leftover the belief drops (the server's drop lane
        // the correction speaks)
        app.live_inv_slots[15] = (0, 0);
        app.live_inv_slots[36] = (helmet, 3);
        for m in 9..45 {
            if m != 36 {
                app.live_inv_slots[m] = (cobble, 64);
            }
        }
        app.live_inv_slots[16] = (stone, 30);
        let (ox3, oy3) = inventory_slot_origin(app.inv_panel, 16);
        app.mouse_pos = Some((ox3 + 8, oy3 + 8));
        assert!(app.on_inventory_swap_key(0), "the digit consumes");
        assert_eq!(app.live_inv_slots[16], (helmet, 1), "the cell the max takes");
        assert_eq!(app.live_inv_slots[36], (helmet, 2), "the row the remainder keeps");
        assert_eq!(app.live_inv_slots[17], (cobble, 64), "the store whole");
        match click_rx.try_recv().expect("the packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(
                    c.changed,
                    vec![(16, (helmet, 1)), (36, (helmet, 2))],
                    "the stall's own tail the changed map never names"
                );
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE DOUBLE-CLICK'S OWN SPREAD (the offline world — the full
    /// flow the vanilla law keeps): the FIRST press the stack grabs
    /// (the plain PICKUP walk), the SECOND press the EMPTIED seat
    /// re-arms the flag and NEVER walks (the carried non-empty the
    /// vanilla QUICK_CRAFT arm answers — the place the release's own
    /// spread serves), the RELEASE the PICKUP_ALL the spread speaks:
    /// the two passes the menu order walks (the partials drain first,
    /// the fulls after), the craft cells the craftSlots container law
    /// never names, the carried the item's own max caps, the HUD row
    /// the every-mutation law syncs.
    #[test]
    fn the_inventory_double_click_the_spread_wears() {
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let cobble = mc_registry::item_id_by_name("cobblestone").unwrap();
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // the store: main-row model 20 holds 32 dirt (the grab), the
        // hotbar row 0 (menu 36) 8 dirt (a partial), the main model 12
        // (menu 12) a FULL 64, the craft cell 0 five dirt (the craft
        // slots' container the spread never pulls from), a cobble stack
        // the different-item gate skips
        app.inventory.items[20] = (dirt, 32);
        app.inventory.items[0] = (dirt, 8);
        app.inventory.items[12] = (dirt, 64);
        app.inventory.items[30] = (cobble, 17);
        app.craft_grid[0] = (dirt, 5);
        app.hotbar_slots = app.inventory.hotbar();
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 20);
        app.mouse_pos = Some((ox + 8, oy + 8));
        // THE FIRST PRESS (the plain PICKUP): the 32 stack the cursor
        // grabs, the seat empties, the last-click seat the press stores
        app.on_press_doubled(false);
        app.on_press(ox + 8, oy + 8);
        assert_eq!(app.inv_carried, (dirt, 32), "the grab");
        assert_eq!(app.inventory.items[20], (0, 0), "the seat empties");
        assert_eq!(app.inv_last_click_slot, Some(20), "the seat remembered");
        assert!(!app.inv_doubleclick, "the first press the flag never arms");
        // THE SECOND PRESS (the doubled tag the window fed, the SAME
        // seat): the flag arms, the held stack the walk NEVER places
        // (the vanilla QUICK_CRAFT arm — the carried stays in the cursor)
        app.on_press_doubled(true);
        app.on_press(ox + 8, oy + 8);
        assert!(app.inv_doubleclick, "the same seat the doubled press arms");
        assert_eq!(app.inv_carried, (dirt, 32), "the held stack the walk keeps");
        assert_eq!(app.inventory.items[20], (0, 0), "the seat stays empty");
        // THE RELEASE (the mouseReleased walk's PICKUP_ALL arm): the
        // spread — pass 1 the partials drain (menu 36's 8 first — the
        // main grid's empties stay silent, the full 64 skipped, the
        // cobble refused, the craft cell refused), pass 2 the full
        // 64's 24 the carried's room takes
        app.on_left_release();
        assert_eq!(app.inv_carried, (dirt, 64), "the spread fills the cursor");
        assert_eq!(app.inventory.items[0], (0, 0), "the hotbar row drained");
        assert_eq!(app.inventory.items[12], (dirt, 40), "the full stack's remainder");
        assert_eq!(app.inventory.items[30], (cobble, 17), "the different item whole");
        assert_eq!(app.craft_grid[0], (dirt, 5), "the craft cell the container law keeps");
        assert_eq!(app.hotbar_slots[0], (0, 0), "the HUD row synced");
        assert!(!app.inv_doubleclick, "the arm's own clear");
        // THE SECOND SPREAD's SILENCE: the release the flag never
        // re-arms (the plain release the gate keeps)
        app.on_left_release();
        assert_eq!(app.inv_carried, (dirt, 64), "the cursor whole");
    }


    /// THE PURE WALK'S OWN PASSES (the doClick PICKUP_ALL arm's shape
    /// over the flat window-0 array): the FIRST pass the partial
    /// stacks drain in the menu order, the FULL stacks skip, the
    /// SECOND pass the fulls answer; the carried stack the ITEM'S OWN
    /// max caps (the eggs' 16 the 64-carrying store the gate keeps);
    /// the backward walk (button != 0) the offhand answers first —
    /// the direction law the byte keys; the no-op gates (the hovered
    /// has an item, the carried empty) the `None` speak.
    #[test]
    fn the_pickup_all_the_two_pass_the_direction_the_max_wear() {
        let egg = mc_registry::item_id_by_name("egg").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        // the menu: main grid 9 holds 5 eggs, 10 a FULL 16, the hotbar
        // 36 three eggs, the offhand 45 two, a stone stack the
        // different-item gate skips
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[9] = (egg, 5);
        slots[10] = (egg, 16);
        slots[36] = (egg, 3);
        slots[45] = (egg, 2);
        slots[12] = (stone, 7);
        // THE FORWARD WALK (button 0), the carried 6: pass 1 the
        // partials — menu 9 takes 5, menu 36 takes 3, the offhand's 2
        // fills the 16 — the full 16 at menu 10 skipped; pass 2 silent
        // (the cursor full)
        let (carried, changed) =
            crate::world_view::predict_pickup_all(&slots, (egg, 6), 20, 0)
                .expect("the spread runs");
        assert_eq!(carried, (egg, 16), "the item's own max caps");
        assert_eq!(
            changed,
            vec![(9, (0, 0)), (36, (0, 0)), (45, (0, 0))],
            "the partials drain in the menu order"
        );
        assert_eq!(slots[10], (egg, 16), "the full stack the first pass skips");
        assert_eq!(slots[12], (stone, 7), "the different item whole");
        // THE PARTIAL CAP: the carried 10 — pass 1 menu 9 fills to 15,
        // menu 36's room 1 takes one (the remainder 2 stays), the
        // offhand never walks (the cursor full)
        let (carried, changed) =
            crate::world_view::predict_pickup_all(&slots, (egg, 10), 20, 0)
                .expect("the spread runs");
        assert_eq!(carried, (egg, 16), "the cap");
        assert_eq!(
            changed,
            vec![(9, (0, 0)), (36, (egg, 2))],
            "the partial take the remainder keeps"
        );
        // THE BACKWARD WALK (button != 0): the offhand first, then the
        // hotbar, the main grid after — the direction law the byte keys
        let (carried, changed) =
            crate::world_view::predict_pickup_all(&slots, (egg, 6), 20, 1)
                .expect("the spread runs");
        assert_eq!(carried, (egg, 16), "the cap");
        assert_eq!(
            changed,
            vec![(45, (0, 0)), (36, (0, 0)), (9, (0, 0))],
            "the backward order the offhand answers first"
        );
        // THE FULLS' OWN PASS: the carried 14 over the FULL-only menu —
        // pass 1 skips the 16s, pass 2 menu 10's room 2 takes
        let mut fulls = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        fulls[10] = (egg, 16);
        let (carried, changed) =
            crate::world_view::predict_pickup_all(&fulls, (egg, 14), 20, 0)
                .expect("the spread runs");
        assert_eq!(carried, (egg, 16), "the cap");
        assert_eq!(changed, vec![(10, (egg, 14))], "the second pass the full answers");
        // THE NO-OP GATES: the hovered HAS an item — None; the carried
        // empty — None; the out-of-range id — None
        assert!(
            crate::world_view::predict_pickup_all(&slots, (egg, 6), 9, 0).is_none(),
            "the hovered's own item the arm refuses"
        );
        assert!(
            crate::world_view::predict_pickup_all(&slots, (0, 0), 20, 0).is_none(),
            "the empty cursor the arm never walks"
        );
        assert!(
            crate::world_view::predict_pickup_all(&slots, (egg, 6), 99, 0).is_none(),
            "the out-of-range id the same silence keeps"
        );
        // THE CRAFT REGION'S OWN REFUSAL (the craftSlots container law):
        // the craft cells + the result the walk never pulls from, the
        // hovered RESULT seat empty the spread still answers (the
        // craft cells' dirt untouched)
        let mut crafty = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        crafty[1] = (egg, 4);
        crafty[2] = (egg, 4);
        let (carried, changed) =
            crate::world_view::predict_pickup_all(&crafty, (egg, 6), 0, 0)
                .expect("the empty result seat the spread answers");
        assert_eq!(carried, (egg, 6), "the cursor whole");
        assert!(changed.is_empty(), "the craft cells the walk never names");
    }


    /// THE RELEASE'S OWN GATES (the mouseReleased walk's arm): the
    /// result + the craft cells' canTakeItemForPickAll refusal the
    /// flag NEVER clears (pc 93's own jump lands past the arm), the
    /// hovered-has-item no-op the arm's own clear STILL speaks (the
    /// slotClicked call unconditional), the null hover the flag keeps.
    #[test]
    fn the_inventory_release_the_gates_the_flag_laws_wear() {
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        app.inventory.items[20] = (dirt, 32);
        // THE RESULT SEAT (menu 0): the armed flag the refusal keeps —
        // no mutation, the flag stays armed
        app.inv_doubleclick = true;
        app.inv_last_click_slot = Some(0);
        app.inv_carried = (dirt, 32);
        let (rx, ry) = craft_slot_origin(app.inv_panel, 4);
        app.mouse_pos = Some((rx + 8, ry + 8));
        app.on_left_release();
        assert!(app.inv_doubleclick, "the refused gate the flag never touches");
        assert_eq!(app.inventory.items[20], (dirt, 32), "the store whole");
        // THE CRAFT CELL (menu 1): the same refusal
        let (cx, cy) = craft_slot_origin(app.inv_panel, 0);
        app.mouse_pos = Some((cx + 8, cy + 8));
        app.on_left_release();
        assert!(app.inv_doubleclick, "the craft cell the same law keeps");
        // THE HOVERED-HAS-ITEM NO-OP: the arm FIRES (the flag clears —
        // the slotClicked call unconditional) but the store never moves
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 20);
        app.mouse_pos = Some((ox + 8, oy + 8));
        app.on_left_release();
        assert!(!app.inv_doubleclick, "the arm's own clear the no-op keeps");
        assert_eq!(app.inventory.items[20], (dirt, 32), "the store whole");
        assert_eq!(app.inv_carried, (dirt, 32), "the cursor whole");
        // THE NULL HOVER: the flag stays (the hoveredSlot null arm)
        app.inv_doubleclick = true;
        app.mouse_pos = Some((332, 157));
        app.on_left_release();
        assert!(app.inv_doubleclick, "the null hover the flag keeps");
    }


    /// THE WIRE SPREAD'S OWN PACKET (the live session — the belief the
    /// same spread answers, the `ServerboundContainerClickPacket` the
    /// command lane speaks): the input the PICKUP_ALL names, the
    /// changed map the walk's ACTUAL mutations carry (the menu ids the
    /// window-0 numbering keys), the carried the spread's outcome
    /// rides, the HUD row the sync follows, and the OFFLINE store the
    /// wire world never touches.
    #[test]
    fn the_wire_pickup_all_the_spread_the_packet_wears() {
        use crate::world_view::ContainerCommand;
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        // the window-0 sync: menu 12 holds 32 dirt (the grab), the
        // hotbar row 0 (menu 36) 8 dirt (a partial), the revision 7
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[12] = (dirt, 32);
        slots[36] = (dirt, 8);
        app.live_inventory_updated(slots, Vec::new(), 7, (0, 0));
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 12);
        app.mouse_pos = Some((ox + 8, oy + 8));
        // THE FIRST PRESS (the wire PICKUP): the 32 stack the cursor
        // grabs, the changed map the seat's own empty names
        app.on_press_doubled(false);
        app.on_press(ox + 8, oy + 8);
        assert_eq!(app.live_inv_carried, (dirt, 32), "the grab");
        assert_eq!(app.live_inv_slots[12], (0, 0), "the seat empties");
        click_rx.try_recv().expect("the pickup's packet rode");
        // THE SECOND PRESS (the doubled tag): the flag arms, no walk
        app.on_press_doubled(true);
        app.on_press(ox + 8, oy + 8);
        assert!(app.inv_doubleclick, "the flag arms");
        assert_eq!(app.live_inv_carried, (dirt, 32), "the held stack stays");
        // THE RELEASE: the spread the belief walks, the packet rides
        app.on_left_release();
        assert_eq!(app.live_inv_carried, (dirt, 40), "the spread's take");
        assert_eq!(app.live_inv_slots[36], (0, 0), "the hotbar cell drained");
        assert_eq!(app.hotbar_slots[0], (0, 0), "the HUD row synced");
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 0, "the player's OWN menu the ctor pins");
                assert_eq!(c.state_id, 7, "the revision the feed spoke");
                assert_eq!(c.slot, 12, "the hovered seat's own menu id");
                assert_eq!(c.button, 0, "the left release the byte keys");
                assert_eq!(
                    c.input,
                    crate::world_view::ContainerInput::PickupAll,
                    "the PICKUP_ALL input the release arm speaks"
                );
                assert_eq!(
                    c.changed,
                    vec![(36, (0, 0))],
                    "the walk's own mutations the changed map names"
                );
                assert_eq!(c.carried, (dirt, 40), "the spread's outcome rides");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        assert_eq!(
            app.inventory.items[12], (0, 0),
            "the offline store the wire world never touches"
        );
    }


    /// THE WIRE RELEASE'S OWN ALWAYS-PACKET (the no-op gates the swap
    /// round's law keeps): the hovered-has-item arm the packet STILL
    /// rides (the empty changed map the untouched belief names, the
    /// carried whole), and the result seat's canTakeItemForPickAll
    /// refusal the packet NEVER rides (the gate the arm BEFORE the
    /// lane keeps).
    #[test]
    fn the_wire_pickup_all_the_noop_and_the_refusal_speak() {
        use crate::world_view::ContainerCommand;
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[12] = (dirt, 32);
        app.live_inventory_updated(slots, Vec::new(), 3, (0, 0));
        // THE HOVERED-HAS-ITEM NO-OP: the armed release over menu 12's
        // own stack — the packet rides, the belief whole
        app.inv_doubleclick = true;
        app.live_inv_carried = (dirt, 32);
        let (ox, oy) = inventory_slot_origin(app.inv_panel, 12);
        app.mouse_pos = Some((ox + 8, oy + 8));
        app.on_left_release();
        assert!(!app.inv_doubleclick, "the arm's own clear");
        assert_eq!(app.live_inv_slots[12], (dirt, 32), "the belief whole");
        assert_eq!(app.live_inv_carried, (dirt, 32), "the cursor whole");
        match click_rx.try_recv().expect("the always packet rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.input, crate::world_view::ContainerInput::PickupAll);
                assert!(c.changed.is_empty(), "the untouched state keeps no entry");
                assert_eq!(c.carried, (dirt, 32));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE RESULT SEAT (menu 0): the refusal the packet never rides
        app.inv_doubleclick = true;
        let (rx, ry) = craft_slot_origin(app.inv_panel, 4);
        app.mouse_pos = Some((rx + 8, ry + 8));
        app.on_left_release();
        assert!(app.inv_doubleclick, "the refused gate the flag keeps");
        assert!(
            click_rx.try_recv().is_err(),
            "the gate's refusal never rides the lane"
        );
    }


    /// THE ARMOR COLUMN'S OWN LANES (the piece slot's click the
    /// menu ids 5..8 key): the shift-click moves the piece to the
    /// store rows (`predict_quick_move_to_store` — the store 9 the
    /// forward range's first empty), the plain press picks the piece
    /// up (the empty cursor's whole take the pickup table speaks).
    #[test]
    fn the_wire_inventory_armor_lanes_speak_their_menu_ids() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let chest = mc_registry::item_id_by_name("iron_chestplate").unwrap();
        // THE SHIFT LANE: the chestplate at the armor slot menu 6
        // (k=1) moves to the store's first row (store 9 = menu 9)
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[6] = (chest, 1);
        app.live_inventory_updated(slots, Vec::new(), 4, (0, 0));
        app.on_shift_down(true);
        let (ax, ay) = armor_slot_origin(app.inv_panel, 1);
        app.on_press(ax + 8, ay + 8);
        assert_eq!(app.live_inv_slots[6], (0, 0), "the piece's own move");
        assert_eq!(app.live_inv_slots[9], (chest, 1), "the store's own take");
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 6, "the armor's own menu id");
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickMove);
                assert_eq!(c.changed, vec![(9, (chest, 1))]);
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE PLAIN PICKUP: the fresh piece rides again (the sync's
        // own word), the empty cursor takes it whole
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[6] = (chest, 1);
        app.live_inventory_updated(slots, Vec::new(), 5, (0, 0));
        app.on_shift_down(false);
        app.on_press(ax + 8, ay + 8);
        assert_eq!(app.live_inv_slots[6], (0, 0));
        assert_eq!(app.live_inv_carried, (chest, 1), "the piece the cursor holds");
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 6);
                assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
                assert_eq!(c.changed, vec![(6, (0, 0))]);
                assert_eq!(c.carried, (chest, 1));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE THROW LANE (the outside press the doClick THROW arm
    /// speaks — the slot -999 the wire's own sentinel): the LEFT
    /// press drops the whole carried stack, the RIGHT splits one, the
    /// drop entity the SERVER spawns (the wire's own Add Entity lane
    /// — the offline stand-in's local spawn never rides), the EMPTY
    /// cursor's press stays silent (the THROW arm's own isEmpty
    /// guard).
    #[test]
    fn the_wire_inventory_throw_outside_speaks_the_999_lane() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        // the LEFT press outside the panel: the drag arm arms, the
        // release's own -999 PICKUP the drop speaks (the whole stack
        // rides)
        let slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        app.live_inventory_updated(slots, Vec::new(), 11, (dirt, 5));
        app.on_press(app.inv_panel.0 - 12, app.inv_panel.1 + 90);
        app.on_mouse_move(app.inv_panel.0 - 12, app.inv_panel.1 + 90);
        app.on_left_release();
        assert_eq!(app.live_inv_carried, (0, 0), "the whole stack's own drop");
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, -999, "the drop's own sentinel");
                assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
                assert_eq!(c.changed, Vec::new(), "no slot the drop touched");
                assert_eq!(c.carried, (0, 0));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE RIGHT press->release: the ONE item the split speaks (the
        // press the drag arms, the release's own deferred throw the
        // ONE serves — the motion the left lane fed the position keeps)
        let slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        app.live_inventory_updated(slots, Vec::new(), 12, (dirt, 5));
        app.on_right_press(app.inv_panel.0 - 12, app.inv_panel.1 + 90);
        app.on_right_release();
        assert_eq!(app.live_inv_carried, (dirt, 4), "the one the split kept");
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, -999);
                assert_eq!(c.button, 1);
                assert_eq!(c.carried, (dirt, 4));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE EMPTY CURSOR: the press stays silent (no packet)
        let slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        app.live_inventory_updated(slots, Vec::new(), 13, (0, 0));
        app.on_press(app.inv_panel.0 - 12, app.inv_panel.1 + 90);
        assert!(
            click_rx.try_recv().is_err(),
            "the empty cursor the THROW arm's guard keeps silent"
        );
    }


    /// THE RESULT SLOT'S OWN TAKE (the window-0 wire lane's onTake —
    /// the plain press the result rides): the LEFT press the WHOLE
    /// result takes (the cursor (planks, 4) holds), the believed cell
    /// the onTake's per-cell removeItem empties, the recompute's own
    /// EMPTY verdict the result keeps; the RIGHT press the
    /// half-rounded-up arm speaks; the changed slots the walk's
    /// actual mutations name (the cell then the verdict, the order
    /// the onTake walk keeps).
    #[test]
    fn the_wire_inventory_result_take_speaks_the_on_take_consume() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        // the recipe the belief speaks: one log in the cell menu 1,
        // the result (planks, 4) the server's sync spoke
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[1] = (log, 1);
        slots[0] = (planks, 4);
        app.live_inventory_updated(slots, Vec::new(), 7, (0, 0));
        let (rx, ry) = craft_slot_origin(app.inv_panel, 4);
        // THE LEFT PRESS: the whole result rides the cursor, the
        // onTake the cell consumes, the recompute's EMPTY verdict lands
        app.on_press(rx + 8, ry + 8);
        assert_eq!(
            app.live_inv_carried, (planks, 4),
            "the whole result the cursor takes"
        );
        assert_eq!(app.live_inv_slots[1], (0, 0), "the cell the onTake consumed");
        assert_eq!(
            app.live_inv_slots[0], (0, 0),
            "the recompute's own EMPTY verdict"
        );
        assert_eq!(
            app.inventory.items[12], (0, 0),
            "the offline store the wire world never touches"
        );
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 0, "the player's OWN menu the ctor pins");
                assert_eq!(c.state_id, 7, "the revision the feed spoke");
                assert_eq!(c.slot, 0, "the result slot's own menu id");
                assert_eq!(c.button, 0);
                assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
                assert_eq!(
                    c.changed,
                    vec![(1, (0, 0)), (0, (0, 0))],
                    "the onTake's own deltas — the cell then the verdict"
                );
                assert_eq!(c.carried, (planks, 4));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE RIGHT PRESS: the half-rounded-up arm the secondary speaks
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[1] = (log, 1);
        slots[0] = (planks, 4);
        app.live_inventory_updated(slots, Vec::new(), 8, (0, 0));
        app.on_right_press(rx + 8, ry + 8);
        assert_eq!(
            app.live_inv_carried, (planks, 2),
            "the half-rounded-up the result count speaks"
        );
        assert_eq!(app.live_inv_slots[1], (0, 0));
        assert_eq!(app.live_inv_slots[0], (0, 0));
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.button, 1);
                assert_eq!(c.changed, vec![(1, (0, 0)), (0, (0, 0))]);
                assert_eq!(c.carried, (planks, 2));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE MERGE ARM'S OWN WALK (the carried-same-item take — the
    /// carried's own space caps the take, the grow the consumer
    /// keeps): the onTake's cell decrement the belief rides, the
    /// recipe's REPEAT the recompute speaks (the re-spoken verdict
    /// the diff law never names — the changed vec the cell alone
    /// carries), the FULL carried the space-0 refusal keeps (no
    /// mutation, the packet rides with the empty changed).
    #[test]
    fn the_wire_inventory_result_merge_rides_the_space_cap() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        // the cursor holds 2 planks, the cell 3 logs, the result 4
        // planks — the space 62 caps the take at the result's own 4
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[1] = (log, 3);
        slots[0] = (planks, 4);
        app.live_inventory_updated(slots, Vec::new(), 9, (planks, 2));
        let (rx, ry) = craft_slot_origin(app.inv_panel, 4);
        app.on_press(rx + 8, ry + 8);
        app.on_mouse_move(rx + 8, ry + 8);
        app.on_left_release();
        assert_eq!(
            app.live_inv_carried, (planks, 6),
            "the merge the carried's own space keeps"
        );
        assert_eq!(
            app.live_inv_slots[1], (log, 2),
            "the one log the onTake consumed"
        );
        assert_eq!(
            app.live_inv_slots[0], (planks, 4),
            "the recipe's REPEAT the recompute re-spoke"
        );
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 0);
                assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
                assert_eq!(
                    c.changed,
                    vec![(1, (log, 2))],
                    "the re-spoken verdict the diff law never names"
                );
                assert_eq!(c.carried, (planks, 6));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // THE FULL CARRIED: the space 0 refuses — the belief stays,
        // the packet rides with the empty changed (the walk's own law)
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[1] = (log, 3);
        slots[0] = (planks, 4);
        app.live_inventory_updated(slots, Vec::new(), 10, (planks, 64));
        app.on_press(rx + 8, ry + 8);
        app.on_mouse_move(rx + 8, ry + 8);
        app.on_left_release();
        assert_eq!(
            app.live_inv_carried, (planks, 64),
            "the full carried the space-0 refusal keeps"
        );
        assert_eq!(app.live_inv_slots[1], (log, 3), "the cell untouched");
        assert_eq!(app.live_inv_slots[0], (planks, 4), "the result untouched");
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.changed, Vec::new(), "no slot the refusal touched");
                assert_eq!(c.carried, (planks, 64));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE CELL PLACE'S OWN PREVIEW + THE MAYPLACE REFUSALS (the
    /// result slot's own mayPlace-false law): the carried log the
    /// cell place rides (the uniform pickup table), the mutation's
    /// slotsChanged ask the recipe verdict answers INSTANTLY (the
    /// (planks, 4) preview the client computes — the changed vec the
    /// cell then the verdict names); the empty result the carried
    /// stack never takes (the place arm's refusal), the different
    /// carried the non-empty result never swaps (the
    /// isSameItemSameComponents silence) — both refusals the belief
    /// untouched keeps, the packets the walk's law still sends.
    #[test]
    fn the_wire_inventory_cell_place_previews_and_the_result_refuses() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        // (a) THE PLACE + THE PREVIEW: the carried 5 logs the cell
        // menu 1 takes, the recompute the (planks, 4) verdict speaks
        app.live_inventory_updated(
            [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS],
            Vec::new(),
            11,
            (log, 5),
        );
        let (cx, cy) = craft_slot_origin(app.inv_panel, 0);
        app.on_press(cx + 8, cy + 8);
        app.on_mouse_move(cx + 8, cy + 8);
        app.on_left_release();
        assert_eq!(
            app.live_inv_slots[1], (log, 5),
            "the cell the carried stack fills"
        );
        assert_eq!(app.live_inv_carried, (0, 0), "the cursor's own empty");
        assert_eq!(
            app.live_inv_slots[0], (planks, 4),
            "the instant preview the recompute spoke"
        );
        // THE PACKET TRIPLE (quickCraftToSlots pcs 0-90 — the START,
        // the ADD per painted seat, the END the distribution names)
        match click_rx.try_recv().expect("the START rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, -999, "the drag's own sentinel");
                assert_eq!(c.button, 0, "the mask(0, type) byte");
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new(), "the transition names nothing");
                assert_eq!(c.carried, (log, 5));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 1, "the painted seat's own menu id");
                assert_eq!(c.button, 1, "the mask(1, type) byte");
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new(), "the collect's own silence");
                assert_eq!(c.carried, (log, 5));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        match click_rx.try_recv().expect("the END rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, -999, "the END's own sentinel");
                assert_eq!(c.button, 2, "the mask(2, type) byte");
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(
                    c.changed,
                    vec![(1, (log, 5)), (0, (planks, 4))],
                    "the distribution's own deltas"
                );
                assert_eq!(c.carried, (0, 0), "the remainder the cursor keeps");
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // (b) THE PLACE REFUSAL: the empty result the carried dirt
        // never takes — no mutation, the packet rides with the empty
        // changed (the mayPlace-false slot's own law)
        let slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        app.live_inventory_updated(slots, Vec::new(), 12, (dirt, 3));
        let (rx, ry) = craft_slot_origin(app.inv_panel, 4);
        app.on_press(rx + 8, ry + 8);
        app.on_mouse_move(rx + 8, ry + 8);
        app.on_left_release();
        assert_eq!(
            app.live_inv_carried, (dirt, 3),
            "the carried stack the refusal keeps"
        );
        assert_eq!(app.live_inv_slots[0], (0, 0), "the result stays empty");
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 0);
                assert_eq!(c.changed, Vec::new(), "no slot the refusal touched");
                assert_eq!(c.carried, (dirt, 3));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
        // (c) THE SWAP REFUSAL: the different carried the non-empty
        // result never swaps — the result the belief keeps
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[0] = (planks, 4);
        app.live_inventory_updated(slots, Vec::new(), 13, (dirt, 2));
        app.on_press(rx + 8, ry + 8);
        app.on_mouse_move(rx + 8, ry + 8);
        app.on_left_release();
        assert_eq!(app.live_inv_carried, (dirt, 2), "the cursor untouched");
        assert_eq!(
            app.live_inv_slots[0], (planks, 4),
            "the result the silence keeps"
        );
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.changed, Vec::new(), "no slot the silence touched");
                assert_eq!(c.carried, (dirt, 2));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE BATCH'S OWN BELIEF WALK (the shift-press on the result —
    /// the quickMoveStack loop the offline craft_result_quick_move
    /// models): two logs the grid holds, the (planks, 4) verdict TWO
    /// iterations speak — each the store rows take (the merge the
    /// hotbar cell menu 36 keeps, the reverse range the walk speaks),
    /// each the onTake the cell consumes, the spent ingredients the
    /// EMPTY verdict breaks; the changed vec one entry per slot, the
    /// final pair the map law keeps (the store cell the merge filled
    /// twice its LAST pair rides, the re-spoken verdict the diff law
    /// never names); the HUD row the merge follows; the cursor and
    /// the offline store the quickMove never touch.
    #[test]
    fn the_wire_inventory_result_quick_move_batches_the_belief() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        app.on_shift_down(true);
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        // the recipe the belief speaks: two logs in the cell menu 1,
        // the result (planks, 4), the hotbar cell menu 36 the merge
        // target (planks, 2) — the batch the two iterations speaks
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[1] = (log, 2);
        slots[0] = (planks, 4);
        slots[36] = (planks, 2);
        app.live_inventory_updated(slots, Vec::new(), 14, (0, 0));
        let (rx, ry) = craft_slot_origin(app.inv_panel, 4);
        app.on_press(rx + 8, ry + 8);
        assert_eq!(
            app.live_inv_slots[36], (planks, 10),
            "the merge the two iterations filled"
        );
        assert_eq!(
            app.live_inv_slots[1], (0, 0),
            "the onTake the two logs consumed"
        );
        assert_eq!(
            app.live_inv_slots[0], (0, 0),
            "the spent grid the EMPTY verdict breaks"
        );
        assert_eq!(
            app.hotbar_slots[0], (planks, 10),
            "the HUD row the merge follows"
        );
        assert_eq!(
            app.live_inv_carried, (0, 0),
            "the cursor the quickMove never rides"
        );
        assert_eq!(
            app.inventory.items[12], (0, 0),
            "the offline store the wire world never touches"
        );
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 0, "the player's OWN menu the ctor pins");
                assert_eq!(c.state_id, 14, "the revision the feed spoke");
                assert_eq!(c.slot, 0, "the result slot's own menu id");
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickMove);
                assert_eq!(
                    c.changed,
                    vec![(36, (planks, 10)), (1, (0, 0)), (0, (0, 0))],
                    "the batch's own deltas — the store, the cell, the verdict, \
                     the final pairs the map law keeps"
                );
                assert_eq!(c.carried, (0, 0));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE FULL STORE'S OWN REFUSAL (the nothing-moved EMPTY the loop
    /// break keeps): every store row a foreign item holds, the merge
    /// pass and the first-empty pass both refuse — the belief
    /// untouched, the packet still rides with the empty changed (the
    /// doClick's own unconditional send the house law keeps).
    #[test]
    fn the_wire_inventory_result_quick_move_refuses_the_full_store() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        app.on_shift_down(true);
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        // the store's whole range (menu 9..44) the foreign stack fills
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        for s in 9..45 {
            slots[s] = (stone, 64);
        }
        slots[1] = (log, 3);
        slots[0] = (planks, 4);
        app.live_inventory_updated(slots, Vec::new(), 15, (0, 0));
        let (rx, ry) = craft_slot_origin(app.inv_panel, 4);
        app.on_press(rx + 8, ry + 8);
        assert_eq!(app.live_inv_slots[1], (log, 3), "the cell untouched");
        assert_eq!(
            app.live_inv_slots[0], (planks, 4),
            "the result the nothing-moved break keeps"
        );
        assert_eq!(
            app.live_inv_slots[9], (stone, 64),
            "the store the refused walk never touched"
        );
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 0);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickMove);
                assert_eq!(c.changed, Vec::new(), "no slot the refusal touched");
                assert_eq!(c.carried, (0, 0));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE LEFTOVER'S OWN LANE (the index-0 tail the 378-385 bytecode
    /// speaks): the store's one planks cell space 2 keeps, the merge
    /// takes 2, the stack part the store never held the SERVER drops
    /// (the wire's own Add Entity lane — the belief never names it),
    /// and the recipe's repeat the loop keeps answering — the second
    /// iteration the full store breaks (the nothing-moved EMPTY), the
    /// re-spoken verdict the belief keeps (the sync's truth replaces).
    #[test]
    fn the_wire_inventory_result_quick_move_drops_the_leftover_lane() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        app.on_shift_down(true);
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let stone = mc_registry::item_id_by_name("stone").unwrap();
        // the store's whole range the foreign stack fills BUT the
        // hotbar cell menu 36 keeps two spaces for the merge
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        for s in 9..45 {
            if s != 36 {
                slots[s] = (stone, 64);
            }
        }
        slots[36] = (planks, 62);
        slots[1] = (log, 2);
        slots[0] = (planks, 4);
        app.live_inventory_updated(slots, Vec::new(), 16, (0, 0));
        let (rx, ry) = craft_slot_origin(app.inv_panel, 4);
        app.on_press(rx + 8, ry + 8);
        assert_eq!(
            app.live_inv_slots[36], (planks, 64),
            "the merge the two spaces filled"
        );
        assert_eq!(
            app.live_inv_slots[1], (log, 1),
            "the one log the first onTake consumed"
        );
        assert_eq!(
            app.live_inv_slots[0], (planks, 4),
            "the recipe's REPEAT the belief keeps — the second iteration \
             the full store broke, the sync's truth replaces"
        );
        match click_rx.try_recv().expect("the packet rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 0);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickMove);
                assert_eq!(
                    c.changed,
                    vec![(36, (planks, 64)), (1, (log, 1))],
                    "the leftover the changed vec never names — the server's \
                     drop the wire's own lane spawns"
                );
                assert_eq!(c.carried, (0, 0));
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }


    /// THE BUNDLE TOOLTIP'S OWN GATE (the renderTooltip walk the
    /// hovered row answers): the bundle slot the contents row feeds
    /// the view speaks (the items the row owns, the selected index
    /// the scroll moved), the plain stack the walk refuses, the
    /// bundle WITHOUT the row the map's own miss keeps, the EMPTY
    /// bundle the show-count law's 0 wall keeps.
    #[test]
    fn the_bundle_tooltip_gate_speaks_the_hovered_row() {
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let bundle = mc_registry::item_id_by_name("bundle").unwrap();
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let dirt = mc_registry::item_id_by_name("dirt").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        // the wire store's belief: the bundle at menu 9 (store 9), a
        // plain dirt stack at menu 10, the contents row the sync fed
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[9] = (bundle, 1);
        slots[10] = (dirt, 32);
        app.live_inventory_updated(slots, Vec::new(), 17, (0, 0));
        app.bundle_contents.insert(
            9,
            BundleContents {
                items: vec![(log, 2), (planks, 5), (dirt, 1)],
                selected_item_index: 1,
            },
        );
        // THE BUNDLE'S HOVER: the view the row speaks
        let (sx, sy) = inventory_slot_origin(app.inv_panel, 9);
        let view = app
            .bundle_tooltip_view(sx + 8, sy + 8)
            .expect("the bundle the gate answers");
        assert_eq!(
            view.items,
            vec![(log, 2), (planks, 5), (dirt, 1)],
            "the row's own items the view keeps"
        );
        assert_eq!(view.selected, 1, "the scroll's moved index rides");
        assert_eq!(view.shown, 3, "the show-count the three items speak");
        assert_eq!(view.rows, 1, "the single row the 3-of-4 grid keeps");
        assert_eq!(view.hidden, 0, "nothing the shown count hides");
        // THE PLAIN STACK: the walk refuses
        let (sx, sy) = inventory_slot_origin(app.inv_panel, 10);
        assert!(
            app.bundle_tooltip_view(sx + 8, sy + 8).is_none(),
            "the plain stack the bundles tag refuses"
        );
        // THE ROWLESS BUNDLE: the map's own miss
        app.bundle_contents.remove(&9);
        let (sx, sy) = inventory_slot_origin(app.inv_panel, 9);
        assert!(
            app.bundle_tooltip_view(sx + 8, sy + 8).is_none(),
            "no row the map keeps — no tooltip"
        );
        // THE EMPTY BUNDLE (the 26.2 extractImage dispatch — the
        // contents' isEmpty the EMPTY branch speaks): the view the
        // zero total carries, the description's own walk the render
        // rides
        app.bundle_contents.insert(9, BundleContents::new(Vec::new()));
        let view = app
            .bundle_tooltip_view(sx + 8, sy + 8)
            .expect("the empty bundle the EMPTY branch answers");
        assert!(
            view.items.is_empty(),
            "the empty bundle's own row the view keeps"
        );
        assert_eq!(
            view.shown, 0,
            "the show-count law's own 0 the empty row keeps"
        );
        assert_eq!(
            view.total, 0,
            "the zero total the Fraction.ZERO bar rides"
        );
        // THE OVERFLOWING BUNDLE (the 26.2 grid the 12-cell budget
        // keeps): 13 stacks the shown count the lone-tail cut caps to
        // 8, the rows the 12-cell budget speaks (3), and the HIDDEN
        // count the surplus cell speaks the skipped stacks' SUM keeps
        app.bundle_contents.insert(
            9,
            BundleContents {
                items: vec![(log, 2); 13],
                selected_item_index: -1,
            },
        );
        let (sx, sy) = inventory_slot_origin(app.inv_panel, 9);
        let view = app
            .bundle_tooltip_view(sx + 8, sy + 8)
            .expect("the overflowing bundle the gate answers");
        assert_eq!(
            view.shown, 8,
            "the lone tail the cap-pad walk cuts (the getNumberOfItemsToShow law)"
        );
        assert_eq!(
            view.rows, 3,
            "the 12-cell budget the slotCount law keeps (the surplus + the blanks the tail speaks)"
        );
        assert_eq!(
            view.hidden, 10,
            "the five skipped stacks' counts the stream-sum law keeps"
        );
    }


    /// THE SHAPE'S OWN PINS (the 26.2 ClientBundleTooltip walk —
    /// slotCount/gridSizeY/getNumberOfItemsToShow the bytecode
    /// keeps): the grid the 12-cell budget speaks, the rows the
    /// positive ceil over FOUR, the shown count the show-count law
    /// caps — the lone tail item never starts a row past the cap,
    /// the full bundle's 11 the three rows keep, a 13-stack bundle
    /// the 12-cell budget holds (the surplus + the blanks the tail
    /// speaks).
    #[test]
    fn the_bundle_tooltip_shape_rides_the_rows_of_four() {
        assert_eq!(
            App::bundle_tooltip_shape(0),
            (0, 0, 0),
            "the empty bundle's wall"
        );
        assert_eq!(App::bundle_tooltip_shape(1), (1, 1, 1), "the one item's cell");
        assert_eq!(App::bundle_tooltip_shape(5), (5, 2, 5), "the 4+1 grid");
        assert_eq!(
            App::bundle_tooltip_shape(12),
            (12, 3, 12),
            "the three full rows"
        );
        assert_eq!(
            App::bundle_tooltip_shape(13),
            (8, 3, 12),
            "the lone tail the cap cuts, the 12-cell budget the grid keeps"
        );
        assert_eq!(
            App::bundle_tooltip_shape(64),
            (11, 3, 12),
            "the full bundle's 11 + the surplus cell"
        );
    }


    /// THE EMPTY BUNDLE'S OWN WALK (the 26.2 extractEmptyBundleTooltip
    /// the extractImage dispatch speaks): the description the split
    /// law wraps at 96, the 9px pitch the height law keeps, the +24
    /// tail the bar region folds, and the ZERO bar the shared
    /// extractProgressbar walk rides (the fill the 0 keeps, the
    /// "Empty" the strict word speaks).
    #[test]
    fn the_bundle_tooltip_empty_walk_rides_the_description() {
        // THE WRAP (font.split at 96 — the house's own greedy walk the
        // metrics keep): the vanilla description's own lines
        let mut fb = crate::raster::Framebuf::new(64, 64);
        let g = Gui::new(&mut fb, 2);
        let lines = wrap_lines(&g, BUNDLE_EMPTY_DESCRIPTION, 96);
        assert_eq!(
            lines.join(" "),
            BUNDLE_EMPTY_DESCRIPTION,
            "the words the wrap never loses"
        );
        assert!(
            lines.len() >= 2,
            "the 31-char description the 96-width law wraps"
        );
        for line in &lines {
            assert!(
                g.text_width(line) <= 96,
                "each line the 96-width law keeps"
            );
        }
        // THE HEIGHT (getEmptyBundleBackgroundHeight — the split's
        // count the 9px pitch speaks, the +24 tail the bar region
        // keeps: 3 margin + 4 gap + 13 bar + 4 spare)
        assert_eq!(
            App::bundle_tooltip_empty_panel_height(lines.len()),
            lines.len() as i32 * 9 + 24,
            "the panel the height law keeps"
        );
        assert_eq!(
            App::bundle_tooltip_empty_panel_height(2),
            42,
            "the two-line walk's own 42 (the 18 + the 24 tail)"
        );
        // THE ZERO BAR (the shared extractProgressbar walk the empty
        // tooltip rides — the Fraction.ZERO the bytecode speaks): the
        // fill the 0 keeps, the word the strict ZERO speaks
        assert_eq!(App::bundle_progressbar_fill(0), 0, "the zero fill");
        assert_eq!(
            App::bundle_progressbar_fill_text(0),
            Some("Empty"),
            "the Empty word the ZERO fraction speaks"
        );
    }


    /// THE SELECTED ITEM'S OWN NAME TOOLTIP (the 26.2
    /// extractSelectedItemTooltip walk the FULL branch's own tail
    /// keeps): the display name the registry's own vocabulary speaks,
    /// the origin the centering + the flips keep, the view's own
    /// selection the walk rides (the items the gate test pins, the
    /// selected index the scroll's walk moves).
    #[test]
    fn the_bundle_name_tooltip_rides_the_selection() {
        // THE DISPLAY NAME (the getStyledHoverName stand-in — the
        // underscores the spaces speak, the words the capitals ride)
        assert_eq!(item_display_name("oak_log"), "Oak Log", "the two-word name");
        assert_eq!(item_display_name("dirt"), "Dirt", "the single word");
        assert_eq!(
            item_display_name("oak_planks"),
            "Oak Planks",
            "the planks' own two words"
        );
        // THE ORIGIN (the virtual cursor the centering keeps — the
        // name's left edge at the panel's own center minus the half
        // name, the -27 the call's -15 and the entry's -12 speak; the
        // box the house's 3px margins keep — the text +6 wide, the
        // 9px line +6 tall)
        let (px, py) = App::bundle_name_tooltip_origin(100, 200, 20, 640, 360);
        assert_eq!(px, 138, "the centering the +12/-12 cancel keeps");
        assert_eq!(py, 173, "the -27 the -15 and the entry keep");
        // THE FLIP (the right overflow the 24-flip the max-4 clamp
        // keeps — the name box the screen's own edge refuses; the
        // bottom the +3 the guard keeps — the y the flip never moves)
        let (px, py) = App::bundle_name_tooltip_origin(600, 200, 60, 640, 360);
        assert_eq!(px, 528, "the 24-flip the overflow speaks (618 - 24 - 66)");
        assert_eq!(py, 173, "the y the flip never moves");
        // THE BOTTOM CLAMP (the +3 the screen's own edge keeps — the
        // guard the bytecode's own branch holds: the box + gap the
        // screen's bottom refuses)
        let (_, py) = App::bundle_name_tooltip_origin(100, 60, 20, 640, 36);
        assert_eq!(py, 36 - 18, "the clamp the +3 keeps (15 + 3 the box + gap)");
    }


    /// THE PROGRESS BAR'S OWN WEIGHT (the 26.2 extractProgressbar
    /// walk the grid's tail keeps): the view the row's SUM carries
    /// (the weight Fraction the per-item 1/maxStackSize law speaks —
    /// the house's default-64 the sum collapses), the fill the
    /// truncate-at-94 law keeps (Mth.mulAndTruncate(weight, 94) the
    /// clamp holds), the text the STRICT ZERO/ONE compares speak
    /// ("Empty"/"Full" the en_us lang keeps, the between the null),
    /// and the FULL texture the f >= ONE law turns red (0xff5555 the
    /// sprite speaks, the 0x5555ff the partial keeps).
    #[test]
    fn the_bundle_tooltip_progressbar_rides_the_weight() {
        // THE LAWS (the pure pins the bytecode keeps)
        assert_eq!(
            App::bundle_progressbar_fill(0),
            0,
            "the empty row the fill refuses"
        );
        assert_eq!(
            App::bundle_progressbar_fill(32),
            47,
            "the half row the 94-scale truncate keeps (32*94/64)"
        );
        assert_eq!(
            App::bundle_progressbar_fill(64),
            94,
            "the full row the 94-wide fill speaks"
        );
        assert_eq!(
            App::bundle_progressbar_fill(100),
            94,
            "the over-full row the clamp holds"
        );
        assert_eq!(
            App::bundle_progressbar_fill_text(0),
            Some("Empty"),
            "the ZERO fraction the BUNDLE_EMPTY_TEXT speaks"
        );
        assert_eq!(
            App::bundle_progressbar_fill_text(64),
            Some("Full"),
            "the ONE fraction the BUNDLE_FULL_TEXT speaks"
        );
        assert_eq!(
            App::bundle_progressbar_fill_text(32),
            None,
            "the between the null keeps (the strict == the branches hold)"
        );
        assert_eq!(
            App::bundle_progressbar_fill_text(100),
            None,
            "the over-full the strict == refuses the text"
        );
        // THE VIEW (the row's own sum the weight carries): a
        // half-filled bundle the 32 the bar's numerator keeps
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.open_inventory();
        app.inv_panel = inventory_panel_origin(640, 360);
        let bundle = mc_registry::item_id_by_name("bundle").unwrap();
        let log = mc_registry::item_id_by_name("oak_log").unwrap();
        let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots[9] = (bundle, 1);
        app.live_inventory_updated(slots, Vec::new(), 17, (0, 0));
        app.bundle_contents.insert(
            9,
            BundleContents {
                items: vec![(log, 16), (planks, 16)],
                selected_item_index: -1,
            },
        );
        let (sx, sy) = inventory_slot_origin(app.inv_panel, 9);
        let view = app
            .bundle_tooltip_view(sx + 8, sy + 8)
            .expect("the bundle the gate answers");
        assert_eq!(
            view.total, 32,
            "the row's stacks the weight sum keeps (the count/64 law the default stack speaks)"
        );
        assert_eq!(
            App::bundle_progressbar_fill(view.total),
            47,
            "the view's own weight the half fill rides"
        );
        // THE FULL BUNDLE: the 64 the red texture + the Full text
        // the ONE fraction speaks
        app.bundle_contents.insert(
            9,
            BundleContents {
                items: vec![(log, 30), (planks, 34)],
                selected_item_index: -1,
            },
        );
        let (sx, sy) = inventory_slot_origin(app.inv_panel, 9);
        let view = app
            .bundle_tooltip_view(sx + 8, sy + 8)
            .expect("the full bundle the gate answers");
        assert_eq!(view.total, 64, "the full row the weight sums");
        assert!(
            view.total >= 64,
            "the f >= ONE law the FULL texture keeps (0xff5555)"
        );
        assert_eq!(
            App::bundle_progressbar_fill_text(view.total),
            Some("Full"),
            "the ONE fraction the bar's own word speaks"
        );
    }


    #[test]
    fn the_sidebar_layout_rides_the_geometry() {
        // two rows on a 427x244 screen: the width law keeps the rows
        // (name + sep + score) vs the title; the center law rides
        // `h/2 + rows*9/3` and the right edge is `w - 3 + 2`
        let rows = [(20, 8), (30, 12)];
        let (left, top, bottom, right, width) = sidebar_layout(427, 244, 40, 6, &rows);
        assert_eq!(width, 30 + 6 + 12, "the widest row wins (sep rides when >0)");
        assert_eq!(right, 426, "the -3 +2 edge law");
        assert_eq!(left, 427 - width - 3, "the left edge the width feeds");
        assert_eq!(bottom, 244 / 2 + 18 / 3, "the h/2 + rows/3 center");
        assert_eq!(top, bottom - 18, "the rows' own height below");
        // a score width of ZERO contributes nothing (the jar's own
        // conditional); a wide title outgrows the rows
        let (l2, _, _, _, w2) = sidebar_layout(427, 244, 200, 6, &[(20, 0)]);
        assert_eq!(w2, 200, "the title wins and the zero score adds naught");
        assert_eq!(l2, 427 - 200 - 3);
        // the zero-row panel: top == bottom (the title-only band)
        let (_, t3, b3, _, _) = sidebar_layout(427, 244, 40, 6, &[]);
        assert_eq!(t3, b3, "no rows, no body");
    }


    #[test]
    fn the_tab_layout_walks_the_grid() {
        // THE COLUMN WALK (the `for(k=1; rows>20; k++) rows=(i+k-1)/k`
        // law): 25 entries need TWO columns (25 > 20 → k=2, ceil(25/2)
        // = 13 ≤ 20); 400 entries ride 20 columns of 20
        let (columns, rows, col_width, grid_x, grid_w) = tab_layout(427, 20, 6, 25);
        assert_eq!((columns, rows), (2, 13), "the smallest k whose rows sink to 20");
        // the content budget: ping 9 + name 20 + score 6 + pad 13 = 48;
        // the grid = min(2*48, w-50)/2 per column + the 5px gutter
        assert_eq!(col_width, (2 * 48).min(427 - 50) / 2, "the capped budget");
        assert_eq!(grid_w, col_width * 2 + 5, "the gutter rides the grid");
        assert_eq!(grid_x, 427 / 2 - grid_w / 2, "the centered origin");
        // the 20x20 ceiling: 400 entries exactly fill the grid
        let (c4, r4, _, _, _) = tab_layout(427, 20, 6, 400);
        assert_eq!((c4, r4), (20, 20), "the walk's own ceiling");
        // 81 entries: k=2 → 41 > 20, k=3 → 27 > 20, k=4 → 21 > 20,
        // k=5 → 17 ≤ 20 (the walk's five-step climb)
        let (c5, r5, _, _, _) = tab_layout(427, 20, 6, 81);
        assert_eq!((c5, r5), (5, 17));
        // the tiny list: ONE column (the loop never climbs), the
        // content budget the whole width share
        let (c1, r1, w1, _, _) = tab_layout(427, 20, 0, 3);
        assert_eq!((c1, r1), (1, 3));
        assert_eq!(w1, 9 + 20 + 0 + 13, "the uncapped budget");
        // the EMPTY list: one column, zero rows — the body band the
        // vanilla extract draws regardless (the 22px content width)
        let (_, r0, w0, _, _) = tab_layout(427, 0, 0, 0);
        assert_eq!((r0, w0), (0, 22), "the empty tab still speaks its width");
    }


    #[test]
    fn the_ride_lane_carries_the_body_and_snaps_the_mount() {
        use crate::world_view::{ContainerCommand, WireRide};
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        // the body stands (the teleport re-anchor materialized it)
        let pose = crate::camera::Camera {
            pos: Vec3::new(1.0, 2.0, 3.0),
            yaw: 0.5,
            pitch: -0.1,
        };
        app.live_view_updated(air_net_world(2, 1), Some(pose), None, None);
        assert!(app.body.is_some(), "the network body materialized");

        // THE INPUT WIRE (the vanilla LocalPlayer.tick's own law): the
        // CHANGED record speaks one packet — the seven booleans the
        // raw keys fold, the sprint the RAW key (never the food
        // gate's verdict)
        app.on_movement_input(1.0, 0.0, false, false, false);
        match click_rx.try_recv().expect("the input change rode the lane") {
            ContainerCommand::PlayerInput {
                forward, sprint, ..
            } => {
                assert!(forward, "W held");
                assert!(!sprint, "the raw Ctrl key state, not the body's gate");
            }
            other => panic!("the wrong command rode the lane: {other:?}"),
        }
        // the SAME record never re-speaks (the lastSentInput re-baseline)
        app.on_movement_input(1.0, 0.0, false, false, false);
        assert!(click_rx.try_recv().is_err(), "no change, no packet");
        // the SHIFT press IS a change (the dismount's own ask)
        app.on_movement_input(1.0, 0.0, false, true, false);
        match click_rx.try_recv().expect("the shift rode the lane") {
            ContainerCommand::PlayerInput { shift, .. } => assert!(shift),
            other => panic!("the wrong command rode the lane: {other:?}"),
        }

        // THE MOUNT SNAP (the fresh-mount law): the ride that appears
        // under the player turns the body's yaw to the vehicle's own
        let ride = WireRide {
            vehicle_id: 7,
            feet_x: 5.0,
            feet_y: 64.2,
            feet_z: -3.0,
            vehicle_yaw: 270.0,
            seat_x: 0.0,
            ride_height: 0.1875,
        };
        app.live_ride_updated(Some(ride));
        assert_eq!(app.net_ride, Some(ride));
        assert_eq!(
            app.body.as_ref().unwrap().yaw,
            270.0,
            "the yRotOF/setYRot/setYHeadRot fold speaks once"
        );

        // THE CARRY (the rideTick's own law): the fixed tick sleeps the
        // body's physics — the carried feet the projection keeps set,
        // the impulses zeroed (the boat steers, not the body), the
        // ground never claims the rider, and the move wire keeps
        // flowing with the carried position (the vanilla
        // isControlledCamera gate keeps sending while riding)
        let b = app.body.as_mut().unwrap();
        b.forward = 1.0; // the W impulse the ride must silence
        b.on_ground = true; // the ground's stale claim
        b.vx = 0.3; // the drift the ride must zero
        // (the borrow ends here — the no-op drop(&mut) the old form
        // kept the NLL loan already speaks)
        app.update();
        let b = app.body.as_ref().unwrap();
        assert_eq!(
            (b.x, b.y, b.z),
            (5.0, 64.2, -3.0),
            "the carried feet the projection keeps"
        );
        assert_eq!(
            (b.forward, b.vx, b.on_ground),
            (0.0, 0.0, false),
            "the physics asleep, the ground's claim dropped"
        );

        // THE DISMOUNT (the ride's drop): the walk owns the player
        // again — the next tick's physics run (the carried feet stop
        // being assigned, the impulses the keys speak flow again)
        app.live_ride_updated(None);
        assert_eq!(app.net_ride, None);
        app.update();
        let b = app.body.as_ref().unwrap();
        assert!(
            b.y != 64.2 || b.vy != 0.0,
            "the body's own physics resumed after the ride dropped"
        );
    }


    #[test]
    fn the_boat_lane_lands_verbatim_and_gates_the_offline_world() {
        use crate::world_view::WireBoat;
        let mut app = App::new(3);
        // THE OFFLINE GATE: the lane never lands (the offline world's
        // lanes the client's own authority keeps)
        app.live_boats_updated(vec![WireBoat {
            id: 7,
            x: 1.0,
            y: 2.0,
            z: 3.0,
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
        }]);
        assert!(
            app.net_boats.is_empty(),
            "the offline world keeps the hulls out"
        );

        // THE NETWORK WORLD: the hulls land verbatim (the snapshot's
        // own word — both shapes one list speaks)
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let boats = vec![
            WireBoat {
                id: 7,
                x: 10.0,
                y: 64.0,
                z: -20.0,
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
            },
            WireBoat {
                id: 9,
                x: 0.0,
                y: 62.5,
                z: 5.0,
                yaw: 0.0,
                raft: true,
                chest: true,
                wood: "bamboo".to_string(),
                paddle_left: false,
                paddle_right: false,
                bubble_time: 0,
                hurt_time: 0,
                hurt_dir: 1,
                damage: 0.0,
            },
        ];
        app.live_boats_updated(boats.clone());
        assert_eq!(app.net_boats, boats, "the hulls land verbatim");
    }


    #[test]
    fn the_wire_hull_lerps_the_paddled_word() {
        use crate::world_view::WireBoat;
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let hull = |x: f64| WireBoat {
            id: 9,
            x,
            y: 64.0,
            z: -20.0,
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
        };
        // THE FIRST LANDING VERBATIM (the fresh id's pose IS the word —
        // the first sync's own honesty, no walk it keeps)
        app.live_boats_updated(vec![hull(10.0)]);
        assert_eq!(app.net_boats[0].x, 10.0, "the first landing verbatim");
        assert_eq!(app.net_boat_lerp[&9].steps, 0, "no walk the first word keeps");
        // THE PADDLED WORD ARMS (the fresh sync the target speaks — the
        // stored pose the eased stand keeps until the tick, never a
        // teleport to the raw word)
        app.live_boats_updated(vec![hull(13.0)]);
        assert_eq!(
            app.net_boats[0].x, 10.0,
            "the eased stand holds until the tick"
        );
        assert_eq!(app.net_boat_lerp[&9].target, (13.0, 64.0, -20.0));
        assert_eq!(app.net_boat_lerp[&9].steps, 3, "the vanilla lerpSteps");
        // THE THREE-TICK WALK (the vanilla lerp's own fold — /3, /2,
        // /1: the third step lands the word exactly, the prev trio the
        // render's partial lerp anchors at the pose the tick eased
        // FROM)
        app.update();
        assert_eq!(app.net_boats[0].x, 11.0, "the first third");
        assert_eq!(
            app.net_boat_lerp[&9].prev,
            (10.0, 64.0, -20.0),
            "the render's own anchor"
        );
        app.update();
        assert!((app.net_boats[0].x - 12.0).abs() < 1e-9, "the half of the rest");
        app.update();
        assert_eq!(app.net_boats[0].x, 13.0, "the last step lands the word");
        assert_eq!(app.net_boat_lerp[&9].steps, 0);
        // THE EXHAUSTED REST (no sync, no motion — the next refresh's
        // arm re-opens)
        app.update();
        assert_eq!(app.net_boats[0].x, 13.0, "the walk rests at its word");
    }


    #[test]
    fn the_ride_carry_follows_the_eased_vehicle() {
        use crate::world_view::WireBoat;
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.body = Some(crate::body::PlayerBody::new(9.4, 64.187, -20.0, 0.0, 0.0));
        let hull = |x: f64, yaw: f32| WireBoat {
            id: 9,
            x,
            y: 64.0,
            z: -20.0,
            yaw,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        };
        // THE MOUNT (the projection's own word — the seat the snapshot
        // yaw rotated: the back seat's −0.6 the multi-rider list
        // speaks, the y the ride height carries)
        app.net_ride = Some(crate::world_view::WireRide {
            vehicle_id: 9,
            feet_x: 9.4,
            feet_y: 64.187,
            feet_z: -20.0,
            vehicle_yaw: 0.0,
            seat_x: -0.6,
            ride_height: 0.1875,
        });
        app.live_boats_updated(vec![hull(10.0, 0.0)]);
        // THE PADDLED WORD (the hull eases toward +3 — the rider the
        // EASED pose carries, the positionRider law the vanilla
        // rideTick speaks, never the raw target's jump)
        app.live_boats_updated(vec![hull(13.0, 0.0)]);
        app.update();
        let b = app.body.as_ref().unwrap();
        // THE F32 SEAT'S OWN ROUNDING (the vanilla fconst the seat's
        // local word keeps — -0.6f32 the f64 sum's own drift speaks,
        // the tolerance the honest fold asserts)
        assert!(
            (b.x - 10.4).abs() < 1.0e-7,
            "the carried feet the eased vehicle keeps (the hull's first third)"
        );
        assert!(
            (b.y - 64.1875).abs() < 1.0e-9,
            "the ride height the vehicle's own y adds"
        );
        assert_eq!(b.z, -20.0, "the seat's sin the zero yaw silences");
        assert_eq!(
            (b.prev_x, b.prev_y, b.prev_z),
            (9.4, 64.187, -20.0),
            "the trail the render's partial lerp anchors"
        );
        // THE TURN'S OWN RE-ROTATE (the positionRider law the CURRENT
        // yaw speaks — the seat's local x the yRot(-yaw·π/180) fold
        // rotates, the rider the orbit the hull's center keeps: the
        // 90° word's cos ~0 / sin −1 the az = −lx·sin fold rides)
        app.live_boats_updated(vec![hull(13.0, 90.0)]);
        app.update();
        let b = app.body.as_ref().unwrap();
        assert!(
            (b.x - 11.666666666666668).abs() < 1e-9,
            "the eased x the carry keeps (the second arm's first third)"
        );
        assert!(
            (b.z - (-20.6)).abs() < 1.0e-7,
            "the seat the turn's re-rotate orbits"
        );
        assert_eq!(b.y, 64.1875, "the ride height the vehicle's own y adds");
    }


    #[test]
    fn the_vehicle_gone_the_carry_keeps_the_stale_word() {
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.body = Some(crate::body::PlayerBody::new(0.0, 70.0, 0.0, 0.0, 0.0));
        // the ride whose vehicle the boat lane never names (the
        // despawn the broadcast hasn't dropped yet): the projection's
        // own snapshot feet the stale walk keeps — a non-zero seat the
        // re-rotate would move, the silence the fallback pins
        app.net_ride = Some(crate::world_view::WireRide {
            vehicle_id: 42,
            feet_x: 5.0,
            feet_y: 64.2,
            feet_z: -3.0,
            vehicle_yaw: 270.0,
            seat_x: -0.6,
            ride_height: 0.1875,
        });
        app.update();
        let b = app.body.as_ref().unwrap();
        assert_eq!(
            (b.x, b.y, b.z),
            (5.0, 64.2, -3.0),
            "the snapshot's own feet the vehicle's silence keeps"
        );
    }


    #[test]
    fn the_paddle_clock_walks_the_vanilla_tick() {
        use crate::world_view::{ContainerCommand, WireBoat, WireRide};
        use crate::screens::{rowing_time, PADDLE_TICK_STEP};
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let boat = |id: i32, left: bool, right: bool| WireBoat {
            id,
            x: 10.0,
            y: 64.0,
            z: -20.0,
            yaw: 0.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: left,
            paddle_right: right,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        };

        // THE REMOTE CLOCK (the projection's gated booleans the walk
        // feeds): a rowing hull advances π/8 per tick, a SILENT hull's
        // clock the 0.0 reset keeps
        app.live_boats_updated(vec![boat(5, true, false), boat(6, false, false)]);
        app.update();
        assert_eq!(
            app.paddle_positions.get(&5).copied(),
            Some([PADDLE_TICK_STEP, 0.0]),
            "the left paddle rows, the right rests"
        );
        assert_eq!(
            app.paddle_positions.get(&6).copied(),
            Some([0.0, 0.0]),
            "the silent hull's clock the reset keeps"
        );
        app.update();
        assert_eq!(
            app.paddle_positions.get(&5).copied(),
            Some([PADDLE_TICK_STEP * 2.0, 0.0]),
            "the phase climbs one step a tick"
        );
        // THE SILENCE RESETS (the rower lifts — the next tick's walk
        // speaks 0.0, never a frozen swing)
        app.live_boats_updated(vec![boat(5, false, false)]);
        app.update();
        assert_eq!(
            app.paddle_positions.get(&5).copied(),
            Some([0.0, 0.0]),
            "the silence the vanilla loop resets to"
        );

        // THE RIDDEN HULL'S OVERRIDE (the controlBoat law the input
        // record feeds — the wire echo never gates the local player's
        // own prediction): W rows BOTH paddles, A rows the RIGHT one
        // only (the turn's own stroke), and ONE PaddleBoat packet
        // rides UNCONDITIONALLY every tick (the vanilla's own send)
        app.last_sent_input = mc_protocol::InputFlags {
            forward: true,
            ..Default::default()
        };
        app.net_ride = Some(WireRide {
            vehicle_id: 9,
            feet_x: 10.0,
            feet_y: 64.187,
            feet_z: -20.0,
            vehicle_yaw: 0.0,
            seat_x: 0.0,
            ride_height: 0.1875,
        });
        app.live_boats_updated(vec![boat(9, false, false)]);
        app.update();
        assert_eq!(
            app.paddle_positions.get(&9).copied(),
            Some([PADDLE_TICK_STEP, PADDLE_TICK_STEP]),
            "the W key rows both paddles — the prediction, not the echo"
        );
        match click_rx
            .try_recv()
            .expect("the ride's own tick speaks the paddle wire")
        {
            ContainerCommand::PaddleBoat { left, right } => {
                assert!(left && right, "the input-derived booleans ride");
            }
            other => panic!("the wrong command rode the lane: {other:?}"),
        }
        app.last_sent_input = mc_protocol::InputFlags {
            left: true,
            ..Default::default()
        };
        app.update();
        match click_rx
            .try_recv()
            .expect("the second tick's unconditional send")
        {
            ContainerCommand::PaddleBoat { left, right } => {
                assert!(!left && right, "the A turn the RIGHT paddle strokes");
            }
            other => panic!("the wrong command rode the lane: {other:?}"),
        }
        assert_eq!(
            app.paddle_positions.get(&9).copied(),
            Some([0.0, PADDLE_TICK_STEP * 2.0]),
            "the left paddle rests, the right climbs"
        );
        // THE DISMOUNT (the ride's drop): the wire falls silent, the
        // clocks back to the projection's gated booleans
        app.net_ride = None;
        app.update();
        assert!(
            click_rx.try_recv().is_err(),
            "no ride, no paddle wire"
        );

        // THE DESPAWN PRUNE (the gone ids' ghost clocks never stay)
        app.live_boats_updated(vec![boat(11, true, true)]);
        assert!(
            !app.paddle_positions.contains_key(&9),
            "the refreshed list's prune takes the gone hull's clock"
        );

        // THE ROWING TIME (the getRowingTime law): a moving paddle's
        // phase the clock's one-tick-ago word through the partial
        // lerp, a silent paddle's 0.0 verbatim
        let pos = PADDLE_TICK_STEP * 3.0;
        assert_eq!(rowing_time(0.0, false, pos), 0.0, "the silence the 0.0 keeps");
        assert!(
            (rowing_time(0.0, true, pos) - (pos - PADDLE_TICK_STEP)).abs() < 1e-6,
            "the partial 0 folds to one tick behind the clock"
        );
        assert!(
            (rowing_time(1.0, true, pos) - pos).abs() < 1e-6,
            "the partial 1 folds to the clock's own word"
        );
        assert!(
            (rowing_time(0.25, true, pos) - (pos - PADDLE_TICK_STEP * 0.75)).abs() < 1e-6,
            "the quarter's own fraction"
        );
    }


    #[test]
    fn the_paddle_window_fires_the_vanilla_crossing() {
        use crate::screens::{paddle_stroke_fires, PADDLE_TICK_STEP};
        // THE SIXTEEN-STEP CYCLE (the π/8 step the clock walks): the
        // window speaks ONLY the π/8 and the π/4 steps — the two
        // firing steps the bytecode's own <= / >= compares keep, the
        // rest of the cycle the probes skip
        let fires: Vec<usize> = (0..16)
            .filter(|k| paddle_stroke_fires(PADDLE_TICK_STEP * *k as f32))
            .collect();
        assert_eq!(fires, vec![1, 2], "the two firing steps of the cycle");
        // THE FRESH CLOCK never speaks: the (pos + π/8) probe the
        // first step keeps under the quarter band
        assert!(!paddle_stroke_fires(0.0), "the fresh clock stays silent");
    }


    #[test]
    fn the_boat_rider_yaw_walks_the_position_rider_law() {
        use crate::screens::{boat_rider_yaw_after, wrap_degrees};
        // THE WRAP (Mth.wrapDegrees(F)F's own arms): the frem fold, the
        // >= 180 arm subtracts (the famous 180.0F itself the fold's
        // -180.0F keeps), the < -180 arm adds, the boundary the arm
        // never speaks
        assert_eq!(wrap_degrees(0.0), 0.0);
        assert_eq!(wrap_degrees(380.0), 20.0);
        assert_eq!(wrap_degrees(-380.0), -20.0);
        assert_eq!(wrap_degrees(180.0), -180.0, "the first arm's own fold");
        assert_eq!(wrap_degrees(-180.0), -180.0, "the second arm never fires at the boundary");
        assert_eq!(wrap_degrees(540.0), -180.0);
        // THE DRAG (setYRot(getYRot() + deltaRotation)): the aligned
        // rider the hull's turn carries, the relative angle the drag
        // keeps
        let boat = 10.0;
        assert!(
            (boat_rider_yaw_after(10.0, boat, 5.0) - 15.0).abs() < 1e-6,
            "the aligned rider the hull's turn carries"
        );
        assert!(
            (boat_rider_yaw_after(100.0, boat, 5.0) - 105.0).abs() < 1e-6,
            "the relative 90 the drag preserves — the free-look rides the hull"
        );
        // THE CLAMP (clampRotation's ±105° band): the beyond-band rider
        // the (g - f) correction snaps back to the hull ± 105
        assert!(
            (boat_rider_yaw_after(180.0, boat, 0.0) - 115.0).abs() < 1e-6,
            "the +170 excess the band pulls to +105"
        );
        assert!(
            (boat_rider_yaw_after(-160.0, boat, 0.0) + 95.0).abs() < 1e-6,
            "the -170 excess the band pulls to -105"
        );
        // THE BOUNDARIES hold verbatim
        assert!(
            (boat_rider_yaw_after(115.0, boat, 0.0) - 115.0).abs() < 1e-6,
            "the +105 boundary holds"
        );
        assert!(
            (boat_rider_yaw_after(-95.0, boat, 0.0) + 95.0).abs() < 1e-6,
            "the -105 boundary holds"
        );
        // THE WRAP INSIDE THE CLAMP: the raw 200° difference the wrap
        // folds the short way round (-160), the band pulls to -105 —
        // the +55 correction the (g - f) walk lands
        assert!(
            (boat_rider_yaw_after(210.0, boat, 0.0) - 265.0).abs() < 1e-6,
            "the short-way fold the band's own pull keeps"
        );
    }


    #[test]
    fn the_ride_yaw_walks_the_wire_turn() {
        use crate::world_view::WireRide;
        let mut app = App::new(3);
        // THE BODY (the rider the mount carries — the windowed walk's
        // own shape, the headless scenes never own one)
        app.body = Some(crate::body::PlayerBody::new(10.0, 64.0, -20.0, 0.0, 0.0));
        let ride = |id: i32, yaw: f32| WireRide {
            vehicle_id: id,
            feet_x: 10.0,
            feet_y: 64.187,
            feet_z: -20.0,
            vehicle_yaw: yaw,
            seat_x: 0.0,
            ride_height: 0.1875,
        };
        // THE FRESH MOUNT ARMS (the first tick never drags — the
        // deltaRotation 0.0 the vanilla field starts at)
        app.net_ride = Some(ride(9, 0.0));
        app.tick_ride_yaw();
        assert_eq!(
            app.ride_boat_yaw_prev,
            Some((9, 0.0)),
            "the tracker the hull's own yaw arms"
        );
        assert_eq!(app.body.as_ref().expect("body").yaw, 0.0, "the arm tick no drag");
        // THE TURN DRAGS (the wire speaks 5° the next tick — the
        // rider's yaw the hull's turn carries)
        app.net_ride = Some(ride(9, 5.0));
        app.tick_ride_yaw();
        assert!(
            (app.body.as_ref().expect("body").yaw - 5.0).abs() < 1e-6,
            "the 5° turn the rider follows"
        );
        // THE FREE-LOOK SURVIVES (the mouse's +60 the body carries; the
        // next identical-yaw tick never moves it — the delta 0, the
        // relative angle the drag preserves)
        app.body.as_mut().expect("body").yaw = 65.0;
        app.net_ride = Some(ride(9, 5.0));
        app.tick_ride_yaw();
        assert!(
            (app.body.as_ref().expect("body").yaw - 65.0).abs() < 1e-6,
            "the free-look the zero delta keeps"
        );
        // THE CLAMP BITES (the mouse swings to the relative +170 — the
        // next tick's walk the (g - f) correction snaps back to the
        // hull + 105)
        app.body.as_mut().expect("body").yaw = 175.0;
        app.net_ride = Some(ride(9, 5.0));
        app.tick_ride_yaw();
        assert!(
            (app.body.as_ref().expect("body").yaw - 110.0).abs() < 1e-6,
            "the +170 excess the band snaps to +105 (110 - boat 5)"
        );
        // THE VEHICLE CHANGE RE-ARMS (a NEW hull's id the tracker takes
        // fresh — the old hull's word never speaks a delta)
        app.net_ride = Some(ride(12, 200.0));
        app.tick_ride_yaw();
        assert_eq!(
            app.ride_boat_yaw_prev,
            Some((12, 200.0)),
            "the new hull's own arm"
        );
        assert!(
            (app.body.as_ref().expect("body").yaw - 110.0).abs() < 1e-6,
            "the arm tick no drag"
        );
        // THE NEXT TICK DRAGS from the new hull's own word: 110 + 5 =
        // 115, the relative (115 - 205 = -90) inside the band stays
        app.net_ride = Some(ride(12, 205.0));
        app.tick_ride_yaw();
        assert!(
            (app.body.as_ref().expect("body").yaw - 115.0).abs() < 1e-6,
            "the new hull's turn the rider follows"
        );
        // THE DISMOUNT CLEARS (the ride's silence the tracker's None
        // keeps — a later mount arms fresh)
        app.net_ride = None;
        app.tick_ride_yaw();
        assert_eq!(app.ride_boat_yaw_prev, None, "the dismount the tracker clears");
    }


    #[test]
    fn the_overlay_message_alpha_walks_the_extract_fold() {
        use crate::screens::overlay_message_alpha;
        // THE 255 CLAMP (the first 40 ticks the full white: the time's
        // own 60 the set takes, the *255/20 fold overflows the byte
        // until the time drops to 20)
        assert_eq!(overlay_message_alpha(60, 1.0), 255, "the fresh set's clamp");
        assert_eq!(overlay_message_alpha(20, 0.0), 255, "the boundary the exact fold keeps");
        // THE FADE (the last 20 ticks the linear walk: the *255/20 the
        // truncation speaks)
        assert_eq!(overlay_message_alpha(10, 0.0), 127, "the half-life's truncation");
        assert_eq!(overlay_message_alpha(5, 0.5), 57, "the partial's own fraction");
        // THE GATES (the <= 0 skip the caller keeps)
        assert_eq!(overlay_message_alpha(1, 1.0), 0, "the last tick's own zero");
        assert_eq!(overlay_message_alpha(0, 0.0), 0, "the dead clock");
    }


    #[test]
    fn the_overlay_message_lives_the_set_overlay_law() {
        let mut app = App::new(3);
        assert_eq!(app.overlay_message, None, "the fresh app's silence");
        // THE SET (Hud.setOverlayMessage's own store: the string, the
        // 60 the time takes)
        app.set_overlay_message("Press Left Shift to Dismount");
        assert_eq!(
            app.overlay_message.as_deref(),
            Some("Press Left Shift to Dismount"),
            "the string the store keeps"
        );
        assert_eq!(app.overlay_message_time, 60, "the vanilla's own life");
        // THE DECAY (Hud.tick's own arm: -1 per tick while it lives)
        for _ in 0..60 {
            app.update();
        }
        assert_eq!(app.overlay_message_time, 0, "the 60 ticks the life spends");
        assert_eq!(
            app.overlay_message.as_deref(),
            Some("Press Left Shift to Dismount"),
            "the STRING never clears — the time's own gate the render reads"
        );
        // THE FLOOR (the > 0 gate the negative never keeps)
        for _ in 0..10 {
            app.update();
        }
        assert_eq!(app.overlay_message_time, 0, "the floor holds");
    }


    #[test]
    fn the_mount_speaks_the_press_shift_message() {
        let mut app = App::new(3);
        app.world_is_network = true;
        app.body = Some(crate::body::PlayerBody::new(10.0, 64.0, -20.0, 0.0, 0.0));
        let ride = |id: i32, yaw: f32| crate::world_view::WireRide {
            vehicle_id: id,
            feet_x: 10.0,
            feet_y: 64.187,
            feet_z: -20.0,
            vehicle_yaw: yaw,
            seat_x: 0.0,
            ride_height: 0.1875,
        };
        // THE MESSAGE'S OWN TEXT (the translatable the lang folds:
        // mount.onboard + the shift binding's display)
        assert_eq!(
            format_positional(MOUNT_ONBOARD_FORMAT, &[SHIFT_KEY_DISPLAY]),
            "Press Left Shift to Dismount",
            "the en_us the jar speaks"
        );
        // THE NO-RIDE REFRESH never speaks (the refresh's own None the
        // walk sees every beat — the mount arm the Some gate keeps)
        app.live_ride_updated(None);
        assert_eq!(app.overlay_message, None, "no mount, no message");
        // THE FRESH MOUNT SPEAKS (the packet handler's own arm: the
        // !hadIndirect gate the fresh passenger passes)
        app.live_ride_updated(Some(ride(9, 0.0)));
        assert_eq!(
            app.overlay_message.as_deref(),
            Some("Press Left Shift to Dismount"),
            "the mount's own words"
        );
        assert_eq!(app.overlay_message_time, 60, "the fresh 60");
        // THE RE-SPEAK never re-sets (the refresh re-ships the same
        // ride — the hadIndirect shape the second packet keeps; the
        // decayed time the walk respects)
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(app.overlay_message_time, 55, "the decay between refreshes");
        app.live_ride_updated(Some(ride(9, 0.0)));
        assert_eq!(app.overlay_message_time, 55, "the re-speak no re-set");
        // THE DISMOUNT clears NOTHING (the packet handler's own
        // honesty: no clear call — the 60-tick timeout the fade owns)
        app.live_ride_updated(None);
        assert_eq!(
            app.overlay_message.as_deref(),
            Some("Press Left Shift to Dismount"),
            "the message outlives the ride"
        );
        assert_eq!(app.overlay_message_time, 55);
        // THE TIMEOUT lands (the remaining ticks the fade spends)
        for _ in 0..55 {
            app.update();
        }
        assert_eq!(app.overlay_message_time, 0, "the time out");
        // THE REMOUNT speaks fresh (a new ride the None the walk crossed)
        app.live_ride_updated(Some(ride(12, 90.0)));
        assert_eq!(app.overlay_message_time, 60, "the remount's fresh life");
    }


    #[test]
    fn the_overlay_message_lands_the_vanilla_spot() {
        use crate::screens::overlay_message_origin;
        // THE GEOMETRY (extractOverlayMessage's translate +
        // textWithBackdrop's own args): the centered x at gui_w/2, the
        // y the h - 68 - 4 fold keeps
        let s = "Press Left Shift to Dismount";
        let tw = crate::gui::font_text_width(s);
        assert_eq!(overlay_message_origin(320, 180, tw), (160 - tw / 2, 108));
        assert_eq!(overlay_message_origin(640, 360, 40), (300, 288));
        // THE DRAW (the backdrop's own rect the pixels pin — the empty
        // assets the text lane keeps silent, the white ground the
        // 0.75/0.25 blend speaks): the 2px pads + the 13px band edges
        let mut fb = crate::raster::Framebuf::new(320, 180);
        fb.clear([1.0, 1.0, 1.0], [1.0, 1.0, 1.0]);
        let (tx, ty) = overlay_message_origin(320, 180, tw);
        {
            let mut g = Gui {
                fb: &mut fb,
                scale: 1,
                scramble: 0,
            };
            let mut assets = crate::gui::GuiAssets::open(std::env::temp_dir());
            let mut app = App::new(3);
            app.set_overlay_message(s);
            app.draw_overlay_message(&mut g, &mut assets, 320, 180);
        }
        let px_at = |x: i32, y: i32| -> u8 {
            let o = (y as usize * 320 + x as usize) * 3;
            fb.color[o]
        };
        // THE BACKDROP's inside: the 0.25 black over white the blend
        // lands at 191 (the 0.75/0.25 mix the px law speaks)
        assert_eq!(px_at(tx - 2, ty - 2), 191, "the top-left pad's own corner");
        assert_eq!(px_at(tx + tw + 1, ty - 2), 191, "the right pad's own edge");
        assert_eq!(px_at(tx, ty + 10), 191, "the 13px band's own bottom row");
        // THE OUTSIDE stays the ground (the pads the rect's own bounds keep)
        assert_eq!(px_at(tx - 3, ty - 2), 255, "the left of the pad untouched");
        assert_eq!(px_at(tx, ty + 11), 255, "below the band untouched");
        assert_eq!(px_at(tx - 2, ty - 3), 255, "above the band untouched");
    }


    #[test]
    fn the_paddle_side_offset_walks_the_view_vector() {
        use crate::screens::paddle_side_offset;
        // yaw 0 = facing south (+Z): the left paddle rides the east
        // (+X) flank, the right the west — the forward the (-sin, cos)
        // fold keeps, the side law the zD/xD reads speak
        assert_eq!(paddle_side_offset(0.0, 0), (1.0, 0.0), "the left flank");
        assert_eq!(paddle_side_offset(0.0, 1), (-1.0, 0.0), "the right flank");
        // yaw 90 = facing west (-X): the left paddle rides the south
        // (+Z) flank, the right the north (the f32 pi/2's own cos the
        // epsilon keeps)
        let (dx, dz) = paddle_side_offset(90.0, 0);
        assert!(dx.abs() < 1e-6 && (dz - 1.0).abs() < 1e-6, "the left of a west-facing hull: ({dx}, {dz})");
        let (dx, dz) = paddle_side_offset(90.0, 1);
        assert!(dx.abs() < 1e-6 && (dz + 1.0).abs() < 1e-6, "the right of a west-facing hull: ({dx}, {dz})");
    }


    #[test]
    fn the_paddle_medium_walks_the_status_law() {
        use crate::screens::{paddle_medium, PADDLE_LAND_VOICE, PADDLE_WATER_VOICE};
        let mut world = air_net_world(2, 0);
        world.set(5, 30, 5, crate::world::WATER);
        world.set(7, 31, 7, crate::world::GRASS);
        world.set(9, 30, 9, crate::world::LAVA);
        // the hull's own cell water — the checkInWater bottom-layer
        // sample the water voice keeps
        assert_eq!(
            paddle_medium(&world, 5.4, 30.3, 5.6),
            Some(PADDLE_WATER_VOICE),
            "the water cell the water voice speaks"
        );
        // the hull's cell air, the block below solid — the ground
        // friction the land voice keeps
        assert_eq!(
            paddle_medium(&world, 7.4, 32.0, 7.6),
            Some(PADDLE_LAND_VOICE),
            "the solid ground the land voice speaks"
        );
        // the hull's cell air over the water surface — the vanilla
        // IN_AIR the water's null ground friction keeps silent
        assert_eq!(
            paddle_medium(&world, 5.4, 31.3, 5.6),
            None,
            "the water top stays silent"
        );
        // the lava cell — the IN_AIR arm the fluid keeps silent
        assert_eq!(
            paddle_medium(&world, 9.4, 30.3, 9.6),
            None,
            "the lava hull stays silent"
        );
        // mid-air — nothing below, the null the silence keeps
        assert_eq!(
            paddle_medium(&world, 20.4, 40.3, 20.6),
            None,
            "the air hull stays silent"
        );
    }


    #[test]
    fn the_paddle_voices_speak_the_stroke_window() {
        use crate::world_view::WireBoat;
        let mut app = App::new(3);
        let mut world = air_net_world(2, 0);
        world.set(5, 30, 5, crate::world::WATER);
        world.set(5, 33, 5, crate::world::GRASS);
        app.net_world = Some(world);
        app.enter_network_world(None, Skybox::Overworld, None, None);
        // THE LISTENER: none — the staging's ear sits at the source (the
        // distance-zero arm the no-fold reads), every voice's gain the
        // 1.0 the full-volume law speaks; the spots' own offsets the
        // paddle_side_offset pins carry
        let boat = |id: i32, x: f64, y: f64, z: f64| WireBoat {
            id,
            x,
            y,
            z,
            yaw: 0.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: true,
            paddle_right: false,
            bubble_time: 0,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        };
        app.live_boats_updated(vec![
            boat(5, 5.4, 30.3, 5.6),  // the water hull
            boat(7, 5.4, 34.0, 5.6),  // the land hull (grass below)
            boat(9, 20.4, 40.3, 20.6), // the air hull
        ]);
        // THE FOUR TICKS: the fresh clock silent, the π/8 and the π/4
        // steps fire (two voices per rowing hull), the π*3/8 step and
        // beyond silent
        for _ in 0..4 {
            app.update();
        }
        if !sounds_loaded() {
            assert!(
                app.sounds.events.is_empty(),
                "the empty table the silent voice lane keeps"
            );
            return;
        }
        let water: Vec<&String> = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.boat.paddle_water "))
            .collect();
        assert_eq!(
            water.len(),
            2,
            "the two firing steps spoke: {:?}",
            app.sounds.events
        );
        assert!(
            water.iter().all(|e| e.contains(" v1.000000 p")),
            "the at-ear voices full volume: {:?}",
            water
        );
        let land: Vec<&String> = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.boat.paddle_land "))
            .collect();
        assert_eq!(
            land.len(),
            2,
            "the land hull's own strokes: {:?}",
            land
        );
        assert!(
            land.iter().all(|e| e.contains(" v1.000000 p")),
            "the at-ear land voices full: {:?}",
            land
        );
        // THE PITCH (the boat's own RandomSource the `0.8F + 0.4F *
        // random.nextFloat()` fold draws — the engine's LCG stands
        // in): every voice inside the band
        for e in water.iter().chain(land.iter()) {
            // the token-scoped walk the param parser speaks — the s
            // tail the panned lanes may wear the pitch token never
            // swallows
            let p: f32 = e
                .split(" p")
                .nth(1)
                .and_then(|s| s.split(' ').next())
                .expect("the pitch tail")
                .parse()
                .expect("the pitch float");
            assert!(
                (0.8..1.2).contains(&p),
                "the pitch band the vanilla fold keeps: {e}"
            );
        }
    }


    #[test]
    fn the_offline_strokes_speak_the_water_voice() {
        use crate::world_view::WireBoat;
        use crate::screens::PADDLE_TICK_STEP;
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
        app.offline_boats.push(WireBoat {
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
        // THE PLACED BYSTANDER (no ride the control never arms — the
        // silence the `!isVehicle` gate keeps, the clock the zero stays)
        app.offline_boats.push(WireBoat {
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
        // THE W RUN (four ticks): the fresh clock silent, the π/8 and
        // the π/4 steps fire — TWO voices per stroking paddle, FOUR
        // with both rowing
        app.last_sent_input = mc_protocol::InputFlags {
            forward: true,
            ..Default::default()
        };
        for _ in 0..4 {
            app.update();
        }
        if !sounds_loaded() {
            assert!(
                app.sounds.events.is_empty(),
                "the empty table the silent voice lane keeps"
            );
            return;
        }
        let water: Vec<&String> = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.boat.paddle_water "))
            .collect();
        assert_eq!(
            water.len(),
            4,
            "the two firing steps spoke per rowing paddle: {:?}",
            app.sounds.events
        );
        // THE PITCH (the `0.8F + 0.4F * random.nextFloat()` fold — the
        // engine's LCG stands in, every voice inside the band)
        for e in &water {
            // the token-scoped walk the param parser speaks — the s
            // tail the panned lanes may wear the pitch token never
            // swallows
            let p: f32 = e
                .split(" p")
                .nth(1)
                .and_then(|s| s.split(' ').next())
                .expect("the pitch tail")
                .parse()
                .expect("the pitch float");
            assert!(
                (0.8..1.2).contains(&p),
                "the pitch band the vanilla fold keeps: {e}"
            );
        }
        // THE CLOCKS (four strokes climbed, both sides) — the
        // bystander's own the zero stays
        assert_eq!(
            app.paddle_positions.get(&7).copied(),
            Some([PADDLE_TICK_STEP * 4.0, PADDLE_TICK_STEP * 4.0]),
            "the W the both clocks climbed"
        );
        assert_eq!(
            app.paddle_positions.get(&8).copied(),
            Some([0.0, 0.0]),
            "the bystander the control never arms"
        );
    }


    #[test]
    fn the_offline_clock_walks_the_stroke_law() {
        use crate::world_view::WireBoat;
        use crate::screens::PADDLE_TICK_STEP;
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
        app.offline_boats.push(WireBoat {
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
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            settled_y + rh,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        assert!(app.mount_offline_hull(7), "the ride armed");
        // THE W TICKS (the fresh clock's first tick silent, the phase
        // climbs one step a tick — both sides row)
        app.last_sent_input = mc_protocol::InputFlags {
            forward: true,
            ..Default::default()
        };
        app.update();
        assert_eq!(
            app.paddle_positions.get(&7).copied(),
            Some([PADDLE_TICK_STEP, PADDLE_TICK_STEP]),
            "the W the both paddles stroke, the first step climbed"
        );
        app.update();
        assert_eq!(
            app.paddle_positions.get(&7).copied(),
            Some([PADDLE_TICK_STEP * 2.0, PADDLE_TICK_STEP * 2.0]),
            "the phase climbs one step a tick"
        );
        // THE A TURN (the swapped law the offline control speaks — the
        // left paddle rests to 0.0, the right keeps climbing)
        app.last_sent_input = mc_protocol::InputFlags {
            left: true,
            ..Default::default()
        };
        app.update();
        assert_eq!(
            app.paddle_positions.get(&7).copied(),
            Some([0.0, PADDLE_TICK_STEP * 3.0]),
            "the A the right paddle strokes, the left the reset keeps"
        );
        // THE S KEY (the backward never arms a paddle — the vanilla
        // law the both clocks reset to zero, never a frozen swing)
        app.last_sent_input = mc_protocol::InputFlags {
            backward: true,
            ..Default::default()
        };
        app.update();
        assert_eq!(
            app.paddle_positions.get(&7).copied(),
            Some([0.0, 0.0]),
            "the silence the vanilla loop resets to"
        );
    }


    #[test]
    fn the_offline_medium_walks_the_local_window() {
        use crate::world_view::WireBoat;
        // THE LAND ARM (the local terrain's own word — the hull's cell
        // air, the solid below the land voice speaks): the grass
        // platform the center column keeps (the spawn column's own air
        // above the surface the walk guarantees)
        let mut app = App::new(3);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let gy = app.spawn_y + 4;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, gy, c, crate::world::GRASS);
        let rh = (crate::world_view::BOAT_DIMENSIONS_HEIGHT / 3.0) as f64;
        let rest_y = (gy + 1) as f64;
        app.offline_boats.push(WireBoat {
            id: 7,
            x: c as f64 + 0.5,
            y: rest_y,
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
            rest_y + rh,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        assert!(app.mount_offline_hull(7), "the ride armed");
        app.last_sent_input = mc_protocol::InputFlags {
            forward: true,
            ..Default::default()
        };
        // THE TWO TICKS (the fresh clock silent, the π/8 step fires —
        // both paddles one voice apiece)
        for _ in 0..2 {
            app.update();
        }
        if !sounds_loaded() {
            assert!(
                app.sounds.events.is_empty(),
                "the empty table the silent voice lane keeps"
            );
        } else {
            let land: Vec<&String> = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.boat.paddle_land "))
                .collect();
            assert_eq!(
                land.len(),
                2,
                "the solid ground the land voice speaks: {:?}",
                app.sounds.events
            );
            assert!(
                app.sounds
                    .events
                    .iter()
                    .all(|e| !e.starts_with("entity.boat.paddle_water ")),
                "the solid ground the water voice never speaks: {:?}",
                app.sounds.events
            );
        }
        // THE AIR ARM (the open sky the medium's silence keeps — the
        // window the clock arms, the voice the null medium withholds)
        let mut app2 = App::new(3);
        app2.enter_world();
        let c2 = app2.offline_world.as_ref().unwrap().size_blocks() / 2;
        app2.offline_boats.push(WireBoat {
            id: 7,
            x: c2 as f64 + 0.5,
            y: (app2.spawn_y + 8) as f64,
            z: c2 as f64 + 0.5,
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
        app2.body = Some(crate::body::PlayerBody::new(
            c2 as f64 + 0.5,
            app2.spawn_y as f64 + 8.0 + rh,
            c2 as f64 + 0.5,
            0.0,
            0.0,
        ));
        assert!(app2.mount_offline_hull(7), "the ride armed");
        app2.last_sent_input = mc_protocol::InputFlags {
            forward: true,
            ..Default::default()
        };
        for _ in 0..2 {
            app2.update();
        }
        assert!(
            app2.sounds.events.is_empty(),
            "the open air the stroke's voice never lands: {:?}",
            app2.sounds.events
        );
    }


    #[test]
    fn the_fresh_world_the_fresh_clocks_keep() {
        use crate::world_view::WireBoat;
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
        app.offline_boats.push(WireBoat {
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
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            settled_y + rh,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        assert!(app.mount_offline_hull(7), "the ride armed");
        app.last_sent_input = mc_protocol::InputFlags {
            forward: true,
            ..Default::default()
        };
        app.update();
        app.update();
        assert!(
            app.paddle_positions.contains_key(&7),
            "the strokes the clocks armed"
        );
        // THE NETWORKED WORLD'S OWN ENTER (the wire's lane owns this
        // water now — the stale offline clocks the prune takes, the
        // vanilla fresh-entity zero the new world's hulls keep)
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        assert!(
            app.paddle_positions.is_empty(),
            "the stale clocks the fresh world's prune takes: {:?}",
            app.paddle_positions
        );
        // AND BACK (the offline terrain's re-entry the same word keeps)
        app.enter_world();
        assert!(
            app.paddle_positions.is_empty(),
            "the re-entry the same prune keeps"
        );
    }


    #[test]
    fn the_bubble_rock_walks_the_tick_bubble_column_law() {
        use crate::world_view::WireBoat;
        let mut app = App::new(3);
        let world = air_net_world(2, 0);
        app.net_world = Some(world);
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let boat = |id: i32, bubble_time: i32| WireBoat {
            id,
            x: 5.4,
            y: 40.3,
            z: 5.6,
            yaw: 0.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time,
            hurt_time: 0,
            hurt_dir: 1,
            damage: 0.0,
        };
        // THE ARMED HULL (the column's word 60) and the silent one (0
        // — the decay lane the no-column silence keeps)
        app.live_boats_updated(vec![boat(5, 60), boat(7, 0)]);
        // THE FOUR TICKS: the armed multiplier ramps 0.05 a tick (the
        // 0..1 clamp far), the angle the `10.0f * sin(0.5*tick) *
        // multiplier` fold speaks; the silent multiplier decays -0.1 a
        // tick INTO the 0 floor — the angle stays 0 exactly
        for _ in 0..4 {
            app.update();
        }
        let armed = app.bubble_state.get(&5).copied().expect("the armed state");
        assert_eq!(
            armed.multiplier, 0.20000000298023224,
            "the four 0.05 ramps the f32 words keep"
        );
        assert_eq!(armed.tick, 4, "the client's own age counter walked");
        assert_eq!(
            armed.angle, 1.9949899911880493,
            "the tick-3 word: 10.0f * sin(1.5) as f32 * 0.2"
        );
        assert_eq!(
            armed.angle_o, 1.2622064352035522,
            "the tick-2 word the lerp's from keeps"
        );
        let silent = app.bubble_state.get(&7).copied().expect("the silent state");
        assert_eq!(silent.multiplier, 0.0, "the decay clamped at the floor");
        assert_eq!(silent.angle, 0.0, "the floor's zero wobble");
        // THE DESPAWN'S OWN PRUNE (the gone ids' rock states never stay)
        app.live_boats_updated(vec![boat(7, 0)]);
        assert!(!app.bubble_state.contains_key(&5), "the gone hull pruned");
        assert!(app.bubble_state.contains_key(&7), "the survivor keeps its word");
    }


    #[test]
    fn the_hurt_wobble_walks_the_tick_decay_law() {
        use crate::screens::HurtWobble;
        use crate::world_view::WireBoat;
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let boat = |hurt_time: i32, hurt_dir: i32, damage: f32| WireBoat {
            id: 5,
            x: 10.0,
            y: 64.0,
            z: -20.0,
            yaw: 0.0,
            raft: false,
            chest: false,
            wood: "oak".to_string(),
            paddle_left: false,
            paddle_right: false,
            bubble_time: 0,
            hurt_time,
            hurt_dir,
            damage,
        };
        // THE NEVER-HIT HULL (the define's own defaults the projection
        // keeps): the walk births the state, the decay's gate the
        // silence keeps — one tick, nothing moved
        app.live_boats_updated(vec![boat(0, 1, 0.0)]);
        app.update();
        let rest = app.hurt_state.get(&5).copied().expect("the state births");
        assert_eq!(rest.time, 0, "the silence the 0 keeps");
        assert_eq!(rest.damage, 0.0, "the no-damage rest");
        // THE HIT'S OWN ADOPT (the server's hurt() words — time 10,
        // the first hit's negate, the damage the amount's ×10 ramps):
        // the seen-tracker's fresh broadcast adopts wholesale, then
        // THE SAME TICK'S OWN DECAY walks (the vanilla client's tick
        // order: the accessor's words the wire set, then the tick's
        // −1/−1.0 the client's own copy keeps)
        app.live_boats_updated(vec![boat(10, -1, 90.0)]);
        app.update();
        let hit = app.hurt_state.get(&5).copied().expect("the hit's state");
        assert_eq!(hit.time, 9, "the arm the adopt speaks, the decay the tick keeps");
        assert_eq!(hit.dir, -1, "the first hit's flip the negate keeps");
        assert_eq!(hit.damage, 89.0, "the −1.0 ramp the tick falls");
        assert_eq!(
            (hit.seen_time, hit.seen_dir, hit.seen_damage),
            (10, -1, 90.0),
            "the seen-tracker's reference the wire's word keeps"
        );
        // THE DECAY'S OWN WALK (two more ticks — 9→7, 89.0→87.0)
        app.update();
        app.update();
        let dec = app.hurt_state.get(&5).copied().expect("the decayed state");
        assert_eq!(dec.time, 7, "the two −1 steps the ticks keep");
        assert_eq!(dec.damage, 87.0, "the two −1.0 steps the ticks keep");
        // THE SILENT HULL'S OWN REST (the wire's word back at the
        // define's defaults — the server's decay caught up, the fresh
        // broadcast the copy takes wholesale: the gate keeps nothing)
        app.live_boats_updated(vec![boat(0, 1, 0.0)]);
        app.update();
        let rest = app.hurt_state.get(&5).copied().expect("the re-adopted state");
        assert_eq!(rest.time, 0, "the fresh word's silence the adopt speaks");
        assert_eq!(rest.damage, 0.0, "the fresh word's rest the adopt speaks");
        // THE DESPAWN'S OWN PRUNE (the gone ids' hurt states never stay)
        app.live_boats_updated(vec![boat(0, 1, 0.0); 1]);
        // a second hull's id absent — the prune rides the same retain
        app.hurt_state.insert(99, HurtWobble::default());
        app.live_boats_updated(vec![boat(0, 1, 0.0)]);
        assert!(!app.hurt_state.contains_key(&99), "the ghost's hurt state pruned");
        assert!(app.hurt_state.contains_key(&5), "the survivor keeps its word");
    }


    #[test]
    fn the_hurt_angle_folds_the_table_sin_chain() {
        use crate::screens::boat_hurt_angle;
        // THE GATE'S OWN SILENCE (the never-hit hull: hurtTime_f = −0.25
        // the ifle the closed gate keeps — 0.0 the rest pose speaks)
        assert_eq!(boat_hurt_angle(0, 1, 0.0, 0.25), 0.0, "the closed gate");
        // THE EXTRACT'S OWN LERP (the hit's word at the render's
        // partial 0.25: hurtTime_f = 10 − 0.25 = 9.75, damageTime_f =
        // max(0, 90 − 0.25) = 89.75) and THE SUBMIT'S OWN FOLD (the
        // f32 chain sin·time·damage/10.0·dir — the sin the 65536-step
        // table's own cell at 9.75): the exact words the bytecode
        // keeps — the f32 sin's own rounding the ideal-math assert
        // would miss
        assert_eq!(
            boat_hurt_angle(10, 1, 90.0, 0.25),
            -27.958423614501953,
            "the dir +1 word the table's cell speaks"
        );
        assert_eq!(
            boat_hurt_angle(10, -1, 90.0, 0.25),
            27.958423614501953,
            "the negate the dir's own flip speaks"
        );
        // THE PARTIAL-ZERO WORD (the headless scenes' own render tick:
        // the cell at 10 ticks, the damage at its full 90.0)
        assert_eq!(
            boat_hurt_angle(10, 1, 90.0, 0.0),
            -48.95623016357422,
            "the full-damage word the cell at 10 speaks"
        );
        // THE CLAMP'S OWN FLOOR (the damage already decayed past the
        // partial: max(0, −0.25) and max(0, −5.25) the extract's own
        // Math.max keeps — both the 0.0 silence the time's arm alone
        // never rescues)
        assert_eq!(boat_hurt_angle(2, 1, 0.0, 0.25), 0.0, "the floored damage");
        assert_eq!(
            boat_hurt_angle(2, 1, -5.0, 0.25),
            0.0,
            "the deeper negative the same floor keeps"
        );
        // THE TIME'S OWN DECAY RESHAPES THE WOBBLE (the cell at 5
        // ticks its own word speaks — the table's quantization the
        // nearby ticks' shared cells the same word may keep)
        let mid = boat_hurt_angle(5, 1, 45.0, 0.0);
        assert!(
            (mid - (-21.57634162902832)).abs() < 1e-6,
            "the tick-5 word: {}",
            mid
        );
    }


    #[test]
    fn the_boat_rocks_only_above_the_surface() {
        use crate::world_view::WireBoat;
        let mut world = air_net_world(2, 0);
        // a 3x3 water patch at y 30 (the fluid's own surface at 30 + 8/9)
        for x in 4..7 {
            for z in 4..7 {
                world.set(x, 30, z, crate::world::WATER);
            }
        }
        let hull = |x: f64, y: f64, z: f64| WireBoat {
            id: 5,
            x,
            y,
            z,
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
        };
        // THE FLOATING HULL (the feet at the fluid's own top — the rim
        // cell above the patch the surface test never reaches): dry
        assert!(!boat_under_water(&world, &hull(5.4, 30.888, 5.6)));
        // THE SUNK HULL (the rim band inside the water — the surface
        // rises above maxY + 0.001): under
        assert!(boat_under_water(&world, &hull(5.4, 29.8, 5.6)));
        // THE DRY HULL (the same height over air — no water at the
        // rim's own cells): never under
        assert!(!boat_under_water(&world, &hull(12.4, 29.8, 12.6)));
    }


    #[test]
    fn the_inside_scan_walks_the_box_cells() {
        let mut world = crate::world::World::new(1, 0);
        world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 4]);
        // THE WHIRLPOOL'S OWN ANSWER (variant 1, drag=true): the box
        // over the column's cells finds it
        bubble_cell(&mut world, 5, 11, 5, 1);
        // the player inside the column: feet 10.0 -> the y band 10.4..
        // 11.4 covers the cells 10 AND 11 (the torso's own band)
        assert_eq!(bubble_inside_scan(&world, 5.0, 10.0, 5.0), Some(true));
        // THE UPWARD'S OWN ANSWER (variant 2, drag=false)
        let mut world = crate::world::World::new(1, 0);
        world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 4]);
        bubble_cell(&mut world, 5, 11, 5, 2);
        assert_eq!(bubble_inside_scan(&world, 5.0, 10.0, 5.0), Some(false));
        // THE FINDFIRST'S OWN ORDER (the betweenClosed walk: x the
        // innermost, y the middle, z the outer — the smallest
        // (z, y, x) wins): the player straddles the x/z boundary
        // (4.71 -> the x cells 4..5, the z cells 4..5), the upward at
        // (4,11,4) the (z, y, x) minimum speaks BEFORE the whirlpool at
        // (5,11,5) the walk reaches later
        let mut world = crate::world::World::new(1, 0);
        world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 4]);
        bubble_cell(&mut world, 5, 11, 5, 1);
        bubble_cell(&mut world, 4, 11, 4, 2);
        assert_eq!(
            bubble_inside_scan(&world, 4.71, 10.0, 4.71),
            Some(false),
            "the (z,y,x) minimum the findFirst keeps"
        );
        // the z flip: the whirlpool at the smaller z wins the same walk
        let mut world = crate::world::World::new(1, 0);
        world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 4]);
        bubble_cell(&mut world, 5, 11, 4, 1);
        bubble_cell(&mut world, 4, 11, 5, 2);
        assert_eq!(
            bubble_inside_scan(&world, 4.71, 10.0, 4.71),
            Some(true),
            "the smaller z the outer arm reaches first"
        );
        // THE PLAIN WATER'S OWN SKIP (variant 0 — the y cell BELOW the
        // bubble never ends the walk): the plain water at y 10, the
        // bubble at y 11 — both in the torso band, the walk finds the
        // bubble through the water
        let mut world = crate::world::World::new(1, 0);
        world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 4]);
        world.set(5, 10, 5, crate::world::WATER);
        bubble_cell(&mut world, 5, 11, 5, 1);
        assert_eq!(
            bubble_inside_scan(&world, 5.0, 10.0, 5.0),
            Some(true),
            "the plain water skipped, the bubble above found"
        );
        // THE STRICT EDGE (the deflate's 1e-6 shave): the box max
        // landing exactly on the integer (x = 4.7 -> 4.7 + 0.3 = 5.0)
        // EXCLUDES the boundary cell — the bubble at x 5 alone never
        // enters the walk's answer
        let mut world = crate::world::World::new(1, 0);
        world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 4]);
        bubble_cell(&mut world, 5, 11, 5, 1);
        assert_eq!(
            bubble_inside_scan(&world, 4.7, 10.0, 5.0),
            None,
            "the exact boundary cell the strict edge excludes"
        );
        // THE OUT-OF-WINDOW'S OWN ANSWER (the grid's AIR): the bubble
        // beyond the 2x2 window never found
        let mut world = crate::world::World::new(1, 0);
        world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 4]);
        bubble_cell(&mut world, 40, 11, 40, 1);
        assert_eq!(bubble_inside_scan(&world, 5.0, 10.0, 5.0), None);
        // THE NO-LAYER WORLD (the procedural scene's variants None):
        // water everywhere, no opinion — silent
        let mut world = crate::world::World::new(1, 0);
        world.set(5, 11, 5, crate::world::WATER);
        assert_eq!(bubble_inside_scan(&world, 5.0, 10.0, 5.0), None);
    }


    #[test]
    fn the_inside_voice_walks_the_handler_state() {
        use crate::screens::BubbleInsideVoice;
        // THE HANDLER'S OWN BIRTH (firstTick armed): the spawn-inside-
        // a-column moment the FIRST tick keeps silent, the latch arms
        let mut h = BubbleInsideVoice::default();
        assert!((h.was_in_column, h.first_tick) == (false, true), "the birth");
        assert_eq!(h.tick(Some(true)), None, "the first tick's silence");
        assert!(h.was_in_column, "the latch armed");
        // THE HOLD (the in-column ticks never repeat the voice)
        assert_eq!(h.tick(Some(true)), None, "the hold never repeats");
        assert_eq!(h.tick(Some(true)), None, "still held");
        // THE EXIT'S OWN RESET (the absence the latch clears)
        assert_eq!(h.tick(None), None);
        assert!(!h.was_in_column, "the latch cleared");
        // THE RE-ENTRY'S OWN VOICE (the entry edge speaks the column's
        // drag bit — the upward's false the Some carries)
        assert_eq!(h.tick(Some(false)), Some(false), "the entry edge speaks");
        assert!(h.was_in_column, "the latch armed again");
        // THE SECOND ENTRY'S OWN VOICE (the exit-re-enter pair speaks
        // every time — the whirlpool's true)
        assert_eq!(h.tick(None), None);
        assert_eq!(h.tick(Some(true)), Some(true), "the whirlpool's entry");
        // the firstTick clear is UNCONDITIONAL (the every-tick arm the
        // bytecode's own tail keeps — even the absent ticks clear it)
        assert!(!h.first_tick, "the birth gate spent");
    }


    #[test]
    fn the_rain_splash_the_tick_walk_speaks() {
        // THE CADENCE + THE VARIANT GATE (javap ClientLevel.
        // tickWeatherEffects, the 26.2 tail): the splash clock counts up
        // while raining, the nextInt(3) < old compare fires (the
        // guaranteed fire by old == 3 — the ramp the post-increment
        // keeps), the fired tick resets the clock and plays the column's
        // variant: the exposed camera the full weather.rain word, the
        // under-roof camera the muffled weather.rain.above one, each the
        // playLocalSound pair verbatim (0.1F/0.5F above, 0.2F/1.0F plain
        // — the ear-less scenes the 0.0 distance answers, the unfaded
        // gain the log records). The dry world the reset + the silence
        // keeps.
        let mut app = App::new(1);
        app.ensure_world();
        let world = app.offline_world.as_mut().unwrap();
        world.rain_level = 1.0;
        // the controlled column: the whole column cleared, the stone
        // floor at y=9 -> the heightmap's first-air-above word is y=10
        for y in 0..crate::world::CHUNK_Y as i32 {
            world.set(8, y, 8, crate::world::AIR);
        }
        world.set(8, 9, 8, crate::world::STONE);
        // THE EXPOSED CAMERA: the eye in the surface air (the floor at
        // 11.62 -> 11) — the surface (10) never clears camera floor + 1,
        // the PLAIN arm the two compares keep
        app.offline_camera = Some(crate::camera::Camera {
            pos: Vec3::new(
                8.5,
                10.0 + crate::world_view::PLAYER_EYE_HEIGHT,
                8.5,
            ),
            yaw: 0.0,
            pitch: 0.0,
        });
        for t in 0..40u64 {
            app.tick_weather_splash(100 + t as i64);
        }
        let dry = &app.sounds.events;
        assert!(!dry.is_empty(), "the ramp guarantees a fire within 3 ticks");
        assert!(
            dry.iter().all(|e| e.starts_with("weather.rain ")),
            "the exposed camera speaks the plain word: {dry:?}"
        );
        assert!(
            dry.iter().all(|e| e.contains(" v0.200000 p1.000000")),
            "the playLocalSound pair verbatim: {dry:?}"
        );
        assert!(app.rain_sound_time <= 3, "the reset the fired arm keeps");

        // THE UNDER-ROOF ARM: the stone cap puts the surface above the
        // camera floor + 1 — the muffled ABOVE word the compares name
        app.sounds.events.clear();
        app.rain_sound_time = 0;
        {
            let world = app.offline_world.as_mut().unwrap();
            for y in 14..crate::world::CHUNK_Y as i32 {
                world.set(8, y, 8, crate::world::STONE);
            }
        }
        for t in 0..40u64 {
            app.tick_weather_splash(200 + t as i64);
        }
        let roofed = &app.sounds.events;
        assert!(!roofed.is_empty(), "the roofed column still splashes");
        assert!(
            roofed.iter().all(|e| e.starts_with("weather.rain.above ")),
            "the under-roof camera speaks the muffled word: {roofed:?}"
        );
        assert!(
            roofed.iter().all(|e| e.contains(" v0.100000 p0.500000")),
            "the above pair verbatim: {roofed:?}"
        );

        // THE DRY SILENCE: the reset the early return keeps, the log
        // the silence keeps
        app.offline_world.as_mut().unwrap().rain_level = 0.0;
        app.sounds.events.clear();
        for t in 0..40u64 {
            app.tick_weather_splash(300 + t as i64);
        }
        assert!(app.sounds.events.is_empty(), "the dry world never splashes");
        assert_eq!(app.rain_sound_time, 0, "the dry tick resets the clock");
    }


    #[test]
    fn the_profiler_toggle_the_audited_law_speaks() {
        let mut app = App::new(3);
        app.enter_world();
        // THE FRESH CLIENT (the renderProfilerChart all-false the
        // ctor keeps)
        assert!(!app.profiler_chart, "the fresh client the pie keeps silent");
        assert_eq!(app.profiler_tree_path, "root", "the ctor's own word");
        assert!(app.profiler_results.is_none(), "the no-results lane");
        // THE RAISE (the `!(isOverlayVisible && flag)` flip — the
        // closed state the raise answers, the setOverlayVisible ride
        // the debug_held word speaks, the modifier the chord spends)
        app.on_profiler_chart_key();
        assert!(app.profiler_chart, "the chord the pie raises");
        assert!(app.debug_held, "the raise the overlay forces visible");
        assert!(app.debug_modifier_used, "the modifier the chord spends");
        // THE OTHER FLAGS THE TOGGLE NEVER TOUCHES (the audited walk
        // the chart family's own pair keeps)
        assert!(!app.fps_charts, "the fps flag the walk never raises");
        assert!(!app.network_charts, "the network flag the walk never raises");
        // THE LOWER (the overlay-visible state the flip answers)
        app.on_profiler_chart_key();
        assert!(!app.profiler_chart, "the re-press the pie lowers");
    }


    #[test]
    fn the_profiler_digit_the_drill_walk_speaks() {
        let mut app = App::new(3);
        app.enter_world();
        // THE CLOSED CHART (the digit the hotbar lane keeps — the
        // false return the consumption never wears)
        assert!(!app.on_profiler_chart_digit(1), "the closed chart never consumes");
        // THE OPEN CHART, NO RESULTS (the consumed word the early
        // return keeps — the panic the None gate avoids)
        app.on_profiler_chart_key();
        assert!(app.on_profiler_chart_digit(1), "the open chart consumes");
        assert_eq!(app.profiler_tree_path, "root", "the no-results walk never moves");
        // THE RESULTS LANDING (the hand-built frame the walk reads)
        app.live_profiler_updated(crate::profiler::ProfilerResults::new(vec![
            ("root".to_string(), 1000, 1),
            ("root\u{1e}a".to_string(), 600, 1),
            ("root\u{1e}b".to_string(), 300, 1),
        ]));
        // THE DRILL-DOWN (the digit 1 the row-1 name answers — the
        // sorted list's head the `a` row wears, the separator the
        // concat chains)
        assert!(app.on_profiler_chart_digit(1));
        assert_eq!(app.profiler_tree_path, "root\u{1e}a", "the digit 1 the row-1 keeps");
        // THE OUT-OF-RANGE REFUSAL (the childless level the range
        // gate keeps — the path the refusal never moves)
        assert!(app.on_profiler_chart_digit(5));
        assert_eq!(app.profiler_tree_path, "root\u{1e}a", "the out-of-range the path keeps");
        // THE DRILL-UP (the digit 0 the parent slice answers — the
        // separator the truncate cuts)
        assert!(app.on_profiler_chart_digit(0));
        assert_eq!(app.profiler_tree_path, "root", "the digit 0 the parent keeps");
        // THE ROOT-LEVEL UP (the no-separator path the same word
        // wears — the walk the no-move answer keeps)
        assert!(app.on_profiler_chart_digit(0));
        assert_eq!(app.profiler_tree_path, "root", "the root-level up the path keeps");
        // THE UNSPECIFIED REFUSAL (the [?] row the drill never
        // enters — the under-covered root the row names)
        app.profiler_tree_path = "root".to_string();
        app.live_profiler_updated(crate::profiler::ProfilerResults::new(vec![
            ("root".to_string(), 1000, 1),
            ("root\u{1e}a".to_string(), 600, 1),
        ]));
        assert!(app.on_profiler_chart_digit(2));
        assert_eq!(app.profiler_tree_path, "root", "the [?] row the drill refuses");
    }


    #[test]
    fn the_profiler_words_the_jar_format_keeps() {
        // THE PERCENTAGE (the DecimalFormat("##0.00") HALF_UP law —
        // the two decimals the whole values keep, the 0.125 the
        // half-up "0.13" the Rust half-even never wears)
        assert_eq!(percentage_word(100.0), "100.00");
        assert_eq!(percentage_word(60.0), "60.00");
        assert_eq!(percentage_word(12.5), "12.50");
        assert_eq!(percentage_word(0.125), "0.13");
        assert_eq!(percentage_word(0.0), "0.00");
        // THE SIDE MULTIPLY (the ARGB.multiply floor law — the
        // half-gray the channels' own /255 keeps)
        assert_eq!(
            argb_multiply(0xFF80FF40u32 as i32, -8355712),
            0xFF408020u32 as i32
        );
        assert_eq!(
            argb_multiply(0xFFFFFFFFu32 as i32, -8355712),
            0xFF808080u32 as i32
        );
    }


    #[test]
    fn the_pie_runs_the_sector_law_speaks() {
        // THE QUARTER SLICE (the [0, 25) span the down-right quadrant
        // answers — the t = atan2(dx/rx, dy/ry) fold the vertex law
        // keeps: t=0 the +Y rim, t=π/2 the +X rim)
        let slices = vec![ProfilerPieSlice {
            start_pct: 0.0,
            span_pct: 25.0,
            color: -1,
        }];
        let runs = profiler_pie_top_runs(500, 192, &slices);
        assert!(!runs.is_empty(), "the sector the runs wear");
        assert!(
            runs.iter().all(|(x, y, _, _)| *x >= 500 && *y >= 192),
            "the down-right quadrant the [0,25) slice keeps: {:?}",
            &runs[..runs.len().min(4)]
        );
        assert!(
            runs.iter().all(|(x, _, _, _)| *x < 500 + 106),
            "the ellipse x the 105-radius bounds"
        );
        assert!(
            runs.iter().all(|(_, y, _, _)| *y < 192 + 54),
            "the ellipse y the 52.5-radius bounds"
        );
        // THE MIRRORED SLICE (the [75, 100) span the down-LEFT
        // quadrant answers — the vertex law the left half of the
        // sweep wears: t ∈ [3π/2, 2π) the sin-negative rim keeps)
        let slices = vec![ProfilerPieSlice {
            start_pct: 75.0,
            span_pct: 25.0,
            color: -1,
        }];
        let runs = profiler_pie_top_runs(500, 192, &slices);
        assert!(
            runs.iter().all(|(x, y, _, _)| *x < 500 && *y >= 192),
            "the down-left quadrant the [75,100) slice keeps"
        );
        // THE CONSECUTIVE PAIR (the two slices the boundary the
        // half-open test never double-covers — the total the pie's
        // own area answers)
        let slices = vec![
            ProfilerPieSlice {
                start_pct: 0.0,
                span_pct: 25.0,
                color: -1,
            },
            ProfilerPieSlice {
                start_pct: 25.0,
                span_pct: 25.0,
                color: -2,
            },
        ];
        let runs = profiler_pie_top_runs(500, 192, &slices);
        let pixels: usize = runs.iter().map(|(_, _, w, _)| *w as usize).sum();
        let quarter = runs
            .iter()
            .filter(|(_, _, _, si)| *si == 0)
            .map(|(_, _, w, _)| *w as usize)
            .sum::<usize>();
        let second = pixels - quarter;
        assert!(quarter > 1000, "the first slice the real area wears");
        // the [25,50) slice = the UP-RIGHT quadrant (t ∈ [π/2, π) —
        // the sin-positive, cos-negative rim the sweep keeps)
        assert!(
            runs.iter()
                .filter(|(_, _, _, si)| *si == 1)
                .all(|(x, _, _, _)| *x >= 500),
            "the up-right quadrant the second slice keeps"
        );
        assert!(second > 1000, "the second slice the real area wears");
    }


    #[test]
    fn the_pie_sides_the_lower_rim_law_speaks() {
        // THE HALF SLICE (the [0, 50) span — the lower rim the
        // `(yA+yB)/2 >= 0` gate draws (the t ∈ (0, π/2] rim the
        // cos-positive half keeps), the upper rim the skip keeps)
        let slices = vec![ProfilerPieSlice {
            start_pct: 0.0,
            span_pct: 50.0,
            color: -1,
        }];
        let fills = profiler_pie_side_fills(500, 192, &slices);
        assert!(!fills.is_empty(), "the lower rim the bands wear");
        assert!(
            fills.iter().all(|(_, y, h, _)| *y >= 192 && *h == 10),
            "the +10 depth the lower rim keeps: {:?}",
            &fills[..fills.len().min(4)]
        );
        // THE SECOND HALF SLICE (the [50, 100) span — the upper rim
        // the gate keeps silent, the rim's own lower arc (the
        // t-near-2π cos-positive tail) the same law draws — the
        // bands the lower rim alone wears either way)
        let slices = vec![ProfilerPieSlice {
            start_pct: 50.0,
            span_pct: 50.0,
            color: -1,
        }];
        let fills = profiler_pie_side_fills(500, 192, &slices);
        assert!(
            fills.iter().all(|(_, y, h, _)| *y >= 192 && *h == 10),
            "the lower rim alone the gate draws: {fills:?}"
        );
        // THE ZERO SLICE (the 0.00% row the no-segment answer keeps)
        let slices = vec![ProfilerPieSlice {
            start_pct: 0.0,
            span_pct: 0.0,
            color: -1,
        }];
        assert!(profiler_pie_side_fills(500, 192, &slices).is_empty());
    }


    #[test]
    fn the_lightmap_toggle_the_mutual_exclusion_law_speaks() {
        let mut app = App::new(3);
        app.enter_world();
        // THE FRESH CLIENT (the renderLightmapTexture all-false the
        // ctor keeps)
        assert!(!app.lightmap_view, "the fresh client the view keeps silent");
        // THE RAISE WITH THE FPS CHARTS UP (the audited putfield pair
        // — the lightmap raise the fps AND network flags zeroes)
        app.fps_charts = true;
        app.network_charts = true;
        app.on_lightmap_view_key();
        assert!(app.lightmap_view, "the chord the view raises");
        assert!(app.debug_held, "the raise the overlay forces visible");
        assert!(!app.fps_charts, "the raise the fps flag zeroes");
        assert!(!app.network_charts, "the raise the network flag zeroes");
        assert!(app.debug_modifier_used, "the modifier the chord spends");
        // THE LOWER (the flags the lowered walk never touches)
        app.on_lightmap_view_key();
        assert!(!app.lightmap_view, "the re-press the view lowers");
    }


    #[test]
    fn the_lightmap_row_the_help_word_speaks() {
        // THE HIDDEN WORD (the fresh client's row the hidden suffix
        // keeps)
        let rows = debug_bottom_lines(false, false, false, false);
        assert!(
            rows.iter().any(|r| r.contains("Lightmap") && r.contains("hidden")),
            "the lightmap row the hidden word wears: {rows:?}"
        );
        // THE VISIBLE WORD (the raised flag's own lane the row re-speaks)
        let rows = debug_bottom_lines(false, false, false, true);
        assert!(
            rows.iter().any(|r| r.contains("Lightmap") && r.contains("visible")),
            "the lightmap row the visible word wears: {rows:?}"
        );
    }


    #[test]
    fn the_lightmap_grid_the_shader_law_speaks() {
        // THE DAY UNIFORMS (the no-opinion world the defaults wear:
        // the full-day sky factor, the white track, the black ambient,
        // the default block tint the None carries)
        let u = LightmapUniforms {
            darkness_scale: 0.0,
            sky_factor: 1.0,
            sky_tint: [1.0, 1.0, 1.0],
            ambient: [0.0, 0.0, 0.0],
            block_tint: None,
            night_vision: None,
            night_vision_factor: 0.0,
        };
        let grid = lightmap_grid(&u);
        // THE DARK CORNER (the block-0 / sky-0 cell the black floor
        // wears — both brightness ramps the zero level answers)
        assert_eq!(grid[0], [0.0, 0.0, 0.0], "the (block0, sky0) cell");
        // THE BRIGHT CORNER (the block-15 / sky-15 cell the clamped
        // white wears — both terms the full level answers)
        assert_eq!(grid[15 * 16 + 15], [1.0, 1.0, 1.0], "the (block15, sky15) cell");
        // THE SKY-ONLY CELL (the block-0 / sky-1 cell the sky term's
        // own brightness wears — the ramp the get_brightness walk
        // keeps, the tint the white track passes)
        let expect = crate::mesher::light_brightness(1.0);
        assert_eq!(grid[1 * 16], [expect, expect, expect], "the (block0, sky1) cell");
        // THE NIGHT SCALE (the moonlight factor the sky term scales —
        // the block term the factor never touches)
        let night = LightmapUniforms {
            sky_factor: 0.26666668,
            ..u
        };
        let grid = lightmap_grid(&night);
        let expect = crate::mesher::light_brightness(15.0) * 0.26666668;
        assert_eq!(grid[15 * 16], [expect, expect, expect], "the night (block0, sky15) cell");
        assert_eq!(
            grid[15],
            crate::mesher::lightmap_combine(
                Some(0.0),
                Some(15.0),
                0.26666668,
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                None,
                None,
                0.0,
                0.0
            ),
            "the night (block15, sky0) cell the block term alone wears"
        );
    }


    #[test]
    fn the_lightmap_cell_the_blit_flip_speaks() {
        // THE V-FLIP (the blit's UV (0,1)-(1,0) walk — the top-left
        // pixel the sky-15 cell shows, the bottom-right the sky-0 row
        // wears, the block axis the left→right run keeps)
        assert_eq!(lightmap_screen_cell(0, 0), (0, 15), "the top-left pixel");
        assert_eq!(lightmap_screen_cell(63, 0), (15, 15), "the top-right pixel");
        assert_eq!(lightmap_screen_cell(0, 63), (0, 0), "the bottom-left pixel");
        assert_eq!(lightmap_screen_cell(63, 63), (15, 0), "the bottom-right pixel");
        assert_eq!(lightmap_screen_cell(4, 0), (1, 15), "the second cell the 4px stride keeps");
    }


    /// THE SWING ANIMATION COMPONENT'S OWN TABLE (javap `Item$Properties
    /// .spear` pcs 221-240 + the SwingAnimation ctor): the seven spear
    /// tiers the STAB type with the `(int)(seconds·20)` duration wear —
    /// 13/15/17/19/19/21/23 (the f32 multiply the exact integers the
    /// nearest-f32 rounding lands, the f2i the truncation speaks) —
    /// every other stack the DEFAULT `(WHACK, 6)` the getOrDefault keeps
    /// (the empty hand, the sword, the unknown id — the fist never
    /// leaves the whack arc).
    #[test]
    fn the_swing_anim_component_the_seven_tiers_wear() {
        let at = |name: Option<&str>| held_swing_anim(name);
        let stab = |dur: i32| crate::hand::SwingAnim {
            kind: crate::hand::SwingKind::Stab,
            duration: dur,
        };
        assert_eq!(at(Some("wooden_spear")), stab(13));
        assert_eq!(at(Some("stone_spear")), stab(15));
        assert_eq!(at(Some("copper_spear")), stab(17));
        assert_eq!(at(Some("iron_spear")), stab(19));
        assert_eq!(at(Some("golden_spear")), stab(19));
        assert_eq!(at(Some("diamond_spear")), stab(21));
        assert_eq!(at(Some("netherite_spear")), stab(23));
        // the DEFAULT component: the empty hand, the non-spear, the unknown
        assert_eq!(at(None), crate::hand::SwingAnim::default());
        assert_eq!(at(Some("iron_sword")), crate::hand::SwingAnim::default());
        assert_eq!(
            at(Some("not_an_item")),
            crate::hand::SwingAnim::default(),
            "the unknown the DEFAULT keeps"
        );
        assert_eq!(
            crate::hand::SwingAnim::default(),
            crate::hand::SwingAnim {
                kind: crate::hand::SwingKind::Whack,
                duration: 6
            }
        );
    }


    /// THE LIVE DURATION READ (javap `updateSwingTime` pcs 0-4 over
    /// `getCurrentSwingDuration`): the App's own tick refreshes the
    /// swing clock's duration from the HELD stack's component — the
    /// iron spear the 19-tick clock wears (the attackAnim the /19 step
    /// speaks), the sword switch the DEFAULT 6 answers — and the
    /// re-trigger gate the LIVE half keeps (the spear's 9, the fist's
    /// 3).
    #[test]
    fn the_swing_clock_the_held_component_refreshes() {
        let mut app = App::new(3);
        app.enter_world();
        let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
        app.inventory.items[app.selected_hotbar] = (spear, 1);
        app.update();
        assert_eq!(app.swing.duration, 19, "the spear's component the clock wears");
        // the swing rides the component's own rhythm: the trigger, the
        // first tick lands 0, the /19 progress step
        app.swing.start();
        app.update();
        assert_eq!(app.swing.swing_time, 0, "the trigger the next tick lands");
        assert!(
            (app.swing.attack_anim - 0.0).abs() < 1e-6,
            "the progress at the landing"
        );
        app.update();
        assert_eq!(app.swing.swing_time, 1);
        assert!(
            (app.swing.attack_anim - 1.0 / 19.0).abs() < 1e-6,
            "the /19 fold the component's duration keeps"
        );
        // the switch: the sword's DEFAULT component the next tick reads
        let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
        app.inventory.items[app.selected_hotbar] = (sword, 1);
        app.update();
        assert_eq!(app.swing.duration, 6, "the DEFAULT the sword wears");
        assert!(app.swing.swing_time < 6, "the clock the new duration rides");
        // the empty hand: the fist's own 6
        app.inventory.items[app.selected_hotbar] = (0, 0);
        app.update();
        assert_eq!(app.swing.duration, 6, "the empty stack the DEFAULT keeps");
    }


    /// THE PLAYER'S OWN ARMOR LANE (the HumanoidArmorLayer walk the
    /// third-person render wires — javap the 26.2 jar byte-level):
    /// the pure resolution the row law speaks (the dirs the
    /// usesInnerModel verdict answers, the leather flag the
    /// composite's own lane arms, the empty/non-armor/elytra rows the
    /// honest None keeps), the offline list the head-first order the
    /// submit's field reads wear, the live store's window slots 5..8
    /// the ctor's 39-k containers speak.
    #[test]
    fn the_player_armor_lane_the_rows_resolve() {
        let id = |name: &str| {
            mc_registry::item_id_by_name(name)
                .unwrap_or_else(|| panic!("{name} the registry carries"))
        };
        let hum = mc_registry::generated::equipment::HUMANOID_DIR;
        let leg = mc_registry::generated::equipment::LEGGINGS_DIR;

        // THE PURE ROW (the chestplate the humanoid dir keeps)
        let (slot, dir, name, leather) =
            App::player_armor_texture_row(crate::avatar::ARMOR_SLOT_CHEST, (id("iron_chestplate"), 1))
                .expect("the chestplate rides the lane");
        assert_eq!(slot, crate::avatar::ARMOR_SLOT_CHEST);
        assert_eq!(dir, hum, "the chestplate the humanoid dir keeps");
        assert_eq!(name, "iron");
        assert!(!leather, "the iron the composite never rides");

        // THE LEGS DIR (usesInnerModel's own verdict — the leggings
        // the inner model set textures, the golden prefix the asset
        // fold keeps)
        let (_, dir, name, _) =
            App::player_armor_texture_row(crate::avatar::ARMOR_SLOT_LEGS, (id("golden_leggings"), 1))
                .expect("the leggings ride the lane");
        assert_eq!(dir, leg, "the leggings the inner dir reads");
        assert_eq!(name, "gold");

        // THE LEATHER FLAG (the dyeable composite's own lane)
        let (_, _, name, leather) =
            App::player_armor_texture_row(crate::avatar::ARMOR_SLOT_CHEST, (id("leather_chestplate"), 1))
                .expect("the leather rides the lane");
        assert_eq!(name, "leather");
        assert!(leather, "the leather the composite arms");

        // THE REFUSALS (the honest gates the shouldRender verdict
        // speaks: the empty slot, the wings-only json, the pumpkin)
        assert_eq!(
            App::player_armor_texture_row(crate::avatar::ARMOR_SLOT_HEAD, (0, 0)),
            None,
            "the empty slot the None keeps"
        );
        assert_eq!(
            App::player_armor_texture_row(crate::avatar::ARMOR_SLOT_CHEST, (id("elytra"), 1)),
            None,
            "the elytra the wings-only json keeps bare"
        );
        assert_eq!(
            App::player_armor_texture_row(crate::avatar::ARMOR_SLOT_HEAD, (id("carved_pumpkin"), 1)),
            None,
            "the pumpkin the CustomHead lane owns"
        );

        // THE OFFLINE LANE (the armor list head-first — the rows the
        // submit's CHEST, LEGS, FEET, HEAD call order wears)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.inventory.armor[0] = (id("iron_helmet"), 1);
        app.inventory.armor[1] = (id("iron_chestplate"), 1);
        app.inventory.armor[2] = (id("golden_leggings"), 1);
        app.inventory.armor[3] = (id("diamond_boots"), 1);
        let want: Vec<(usize, &'static str, &'static str, bool)> = vec![
            (crate::avatar::ARMOR_SLOT_CHEST, hum, "iron", false),
            (crate::avatar::ARMOR_SLOT_LEGS, leg, "gold", false),
            (crate::avatar::ARMOR_SLOT_FEET, hum, "diamond", false),
            (crate::avatar::ARMOR_SLOT_HEAD, hum, "iron", false),
        ];
        assert_eq!(app.player_armor_lane_rows(), want, "the offline rows the walk order keeps");

        // THE BARE HEAD'S SKIP (the empty row the lane drops)
        app.inventory.armor[0] = (0, 0);
        assert_eq!(app.player_armor_lane_rows().len(), 3, "the bare head the lane drops");

        // THE NETWORK LANE (the window slots 5 head, 6 chest, 7 legs,
        // 8 feet — the ctor's 39-k containers the menu slots answer;
        // the bare legs row the lane drops, the leather the flag arms)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.world_is_network = true;
        app.live_inv_slots[5] = (id("iron_helmet"), 1);
        app.live_inv_slots[6] = (id("leather_chestplate"), 1);
        app.live_inv_slots[7] = (0, 0);
        app.live_inv_slots[8] = (id("iron_boots"), 1);
        let want: Vec<(usize, &'static str, &'static str, bool)> = vec![
            (crate::avatar::ARMOR_SLOT_CHEST, hum, "leather", true),
            (crate::avatar::ARMOR_SLOT_FEET, hum, "iron", false),
            (crate::avatar::ARMOR_SLOT_HEAD, hum, "iron", false),
        ];
        assert_eq!(app.player_armor_lane_rows(), want, "the live rows the window slots read");
    }


    /// THE PLAYER'S ARMOR THE FRAME PAINTS (the third-person render's
    /// own round — the pre-filled assets the walked root loads, the
    /// bare body the baseline speaks against, the iron kit the
    /// inflated cubes paint over): the bare frames bit-identical
    /// twice, the dressed frame the armor's pixels wear.
    #[test]
    fn the_player_armor_the_frame_paints() {
        let _render_lane = render_lane();
        // the runtime root the loaders' prefix walk keeps (the absent
        // cache the honest skip answers — the mannequin tests' own law)
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        // the skin + armor gates the same honesty keeps (a failed
        // sha1 verify the lane never draws)
        let Some(model) = crate::avatar::AvatarModel::load(root) else {
            return;
        };
        let hum = mc_registry::generated::equipment::HUMANOID_DIR;
        let leg = mc_registry::generated::equipment::LEGGINGS_DIR;
        let iron_base = mc_registry::generated::equipment::find("iron")
            .and_then(|e| crate::avatar::ArmorTex::load(root, hum, e));
        let iron_legs = mc_registry::generated::equipment::find("iron")
            .and_then(|e| crate::avatar::ArmorTex::load(root, leg, e));
        let (Some(iron_base), Some(iron_legs)) = (iron_base, iron_legs) else {
            return; // the equipment pngs the runtime cache keeps absent
        };
        let id = |name: &str| {
            mc_registry::item_id_by_name(name)
                .unwrap_or_else(|| panic!("{name} the registry carries"))
        };
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.perspective = crate::camera::CameraType::ThirdPersonBack;
        // THE PRE-FILLED ASSETS (the walked root the loaders' honesty
        // keeps — the lazy pass the cache's presence never re-loads,
        // the hardcoded CWD the test process never consults)
        app.avatar = Some(model);
        app.armor_textures.insert(format!("{hum}/iron"), Some(iron_base));
        app.armor_textures.insert(format!("{leg}/iron"), Some(iron_legs));
        let mut assets = GuiAssets::open(root);
        let mut render_bare = || {
            let mut fb = crate::raster::Framebuf::new(320, 200);
            app.render(&mut fb, &mut assets);
            fb.color.clone()
        };
        let bare = render_bare();
        let bare_twice = render_bare();
        assert_eq!(bare, bare_twice, "the bare frames bit-identical twice");
        // THE KIT (the four rows the offline list head-first carries —
        // the helmet, the chestplate, the leggings, the boots)
        app.inventory.armor[0] = (id("iron_helmet"), 1);
        app.inventory.armor[1] = (id("iron_chestplate"), 1);
        app.inventory.armor[2] = (id("iron_leggings"), 1);
        app.inventory.armor[3] = (id("iron_boots"), 1);
        let mut fb = crate::raster::Framebuf::new(320, 200);
        app.render(&mut fb, &mut assets);
        let mut diff = 0usize;
        for (b, d) in bare.chunks_exact(3).zip(fb.color.chunks_exact(3)) {
            if b != d {
                diff += 1;
            }
        }
        assert!(
            diff > 100,
            "the iron kit the frame paints: {diff} pixels differ"
        );
    }


    #[test]
    fn the_third_person_head_block_the_frame_paints() {
        // the runtime root the loaders' prefix walk keeps (the absent
        // cache the honest skip answers — the armor frame test's own
        // law)
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        let Some(model) = crate::avatar::AvatarModel::load(root) else {
            return;
        };
        let id = |name: &str| {
            mc_registry::item_id_by_name(name)
                .unwrap_or_else(|| panic!("{name} the registry carries"))
        };
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.perspective = crate::camera::CameraType::ThirdPersonBack;
        app.avatar = Some(model);
        let bare = third_person_frame(&mut app);
        let bare_twice = third_person_frame(&mut app);
        assert_eq!(bare, bare_twice, "the bare frames bit-identical twice");
        // THE PUMPKIN (the offline HEAD row's own stack): the item
        // branch the third-person frame paints — the block's faces
        // the head chain rides (the centered 0.625 law the mannequin
        // round's geometry test pins, the atlas fetch the sampler
        // keeps)
        app.inventory.armor[0] = (id("carved_pumpkin"), 1);
        let worn = third_person_frame(&mut app);
        let diff = diff_pixels(&bare, &worn);
        assert!(
            diff > 200,
            "the carved pumpkin the third-person frame paints: {diff} pixels differ"
        );
    }


    #[test]
    fn the_third_person_skull_the_frame_paints() {
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        let Some(model) = crate::avatar::AvatarModel::load(root) else {
            return;
        };
        let id = |name: &str| {
            mc_registry::item_id_by_name(name)
                .unwrap_or_else(|| panic!("{name} the registry carries"))
        };
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.perspective = crate::camera::CameraType::ThirdPersonBack;
        app.avatar = Some(model);
        let bare = third_person_frame(&mut app);
        // THE SKULL (the wornHeadType branch's own stack): the special
        // bake the third-person frame paints — the 1.1875 chain the
        // submitSkull walk keeps, the entity textures the atlas serves
        app.inventory.armor[0] = (id("skeleton_skull"), 1);
        let worn = third_person_frame(&mut app);
        let worn_twice = third_person_frame(&mut app);
        assert_eq!(worn, worn_twice, "the skull frames bit-identical twice");
        let diff = diff_pixels(&bare, &worn);
        assert!(
            diff > 200,
            "the skeleton skull the third-person frame paints: {diff} pixels differ"
        );
    }


    #[test]
    fn the_third_person_head_lane_the_live_and_verdict_keep() {
        let _render_lane = render_lane();
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        let Some(model) = crate::avatar::AvatarModel::load(root) else {
            return;
        };
        let id = |name: &str| {
            mc_registry::item_id_by_name(name)
                .unwrap_or_else(|| panic!("{name} the registry carries"))
        };
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.perspective = crate::camera::CameraType::ThirdPersonBack;
        app.avatar = Some(model);
        let bare = third_person_frame(&mut app);
        // THE HELMET'S NO-DOUBLE-RENDER (the shouldRender TRUE arm's
        // headItem.clear() the extraction speaks): the head lane's
        // NEITHER verdict the pumpkin/skull branches refuse — the
        // helmet the ARMOR lane's one paint rides (the dev-152 round's
        // runtime-root walk the armor textures BOTH lanes load — the
        // old 0-diff pin the degenerate no-asset lane's own artifact
        // wore, the loaded world the paint speaks)
        app.inventory.armor[0] = (id("iron_helmet"), 1);
        let helmeted = third_person_frame(&mut app);
        let helmet_diff = diff_pixels(&bare, &helmeted);
        assert!(
            helmet_diff > 100,
            "the helmet the armor lane paints once ({helmet_diff} pixels differ)"
        );
        // THE LIVE LANE (the network store's window slot 5 the 39-k
        // order speaks): the server's own equipment the head chain
        // wears — the offline row the walk ignores in the network mode
        app.inventory.armor[0] = (0, 0);
        app.world_is_network = true;
        app.live_inv_slots[5] = (id("carved_pumpkin"), 1);
        let live = third_person_frame(&mut app);
        let diff = diff_pixels(&bare, &live);
        assert!(
            diff > 200,
            "the live window slot 5 the third-person frame paints: {diff} pixels differ"
        );
    }


    #[test]
    fn the_skeleton_armor_the_prepass_collects() {
        // the pre-pass's own law (the kind gate's drop this round
        // speaks): an armored SKELETON's pairs the collection loads —
        // the bare skeleton the map never touches
        let mut app = App::new(3);
        armored_skeleton_stage(&mut app);
        let key = format!(
            "{}/iron",
            mc_registry::generated::equipment::HUMANOID_DIR
        );
        let _ = third_person_frame(&mut app);
        assert!(
            !app.armor_textures.contains_key(&key),
            "the bare skeleton the armor map never fills"
        );
        // THE DRESS (the wire words the skeleton's rows carry — the
        // boots the FEET row, the helmet the HEAD row): the render
        // round the pairs collect (the value the test's CWD the
        // honest failed-read keeps — the collection the law pins)
        app.mobs[0].armor[0] = Some("iron_boots");
        app.mobs[0].armor[3] = Some("iron_helmet");
        let _ = third_person_frame(&mut app);
        assert!(
            app.armor_textures.contains_key(&key),
            "the armored skeleton's pairs the pre-pass collects"
        );
    }


    #[test]
    fn the_skeleton_armor_the_frame_paints() {
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join("gui/container/inventory.png").exists());
        let Some(root) = root else { return };
        let Some(model) = crate::avatar::AvatarModel::load(root) else {
            return;
        };
        let Some(skeleton) = crate::avatar::AvatarModel::load_skeleton(root) else {
            return;
        };
        let hum = mc_registry::generated::equipment::HUMANOID_DIR;
        let Some(iron) = mc_registry::generated::equipment::find("iron")
            .and_then(|e| crate::avatar::ArmorTex::load(root, hum, e))
        else {
            return; // the equipment png the runtime cache keeps absent
        };
        let mut app = App::new(3);
        armored_skeleton_stage(&mut app);
        // THE PRE-FILLED ASSETS (the walked root the loaders' honesty
        // keeps — the hardcoded CWD the test process never consults)
        app.avatar = Some(model);
        app.skeleton_avatar = Some(skeleton);
        app.armor_textures.insert(format!("{hum}/iron"), Some(iron));
        let bare = third_person_frame(&mut app);
        let bare_twice = third_person_frame(&mut app);
        assert_eq!(bare, bare_twice, "the bare frames bit-identical twice");
        // THE DRESS (the boots the FEET row, the helmet the HEAD row):
        // the skeleton's armor the frame paints — the iron kit the
        // archer's own body wears over the bare skin
        app.mobs[0].armor[0] = Some("iron_boots");
        app.mobs[0].armor[3] = Some("iron_helmet");
        let dressed = third_person_frame(&mut app);
        let dressed_twice = third_person_frame(&mut app);
        assert_eq!(
            dressed, dressed_twice,
            "the dressed frames bit-identical twice"
        );
        let diff = diff_pixels(&bare, &dressed);
        assert!(
            diff > 2000,
            "the skeleton's armor the frame paints: {diff} pixels differ"
        );
    }

    /// THE WARDEN'S OWN RENDER (the WardenRenderer's own lane — the
    /// wardens list the warden body the base skin draws): the
    /// sha1-verified 128x128 load the frame answers, the standing
    /// body the with/without diff lights (the idle sway the age
    /// clock wears), the dead body the health gate refuses (the
    /// corpse the frame never wears). THE TENDRIL SHIVER's own swing
    /// the avatar module's quad-level test pins — the sparse dotted
    /// tendril texels the north-facing faces keep, the south-facing
    /// stage camera the frame never lights them the honest cutout
    /// law speaks.
    #[test]
    fn the_warden_the_body_the_frame_wears() {
        // THE WALKED ROOT (the 3-path fallback the test process speaks
        // — the hardcoded CWD the lazy loads consult, the walked root
        // the PRE-FILLED field keeps — the skeleton armor's own law)
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join(mc_registry::generated::skin::WARDEN_RUNTIME_PATH).exists());
        let Some(root) = root else { return };
        let Some(warden_model) = crate::avatar::WardenModel::load(root) else {
            return; // the sha1 gate — the honest skip
        };
        let mut app = App::new(3);
        armored_skeleton_stage(&mut app);
        app.warden_model = Some(warden_model);
        // THE WARDEN (the stage's own ground keeps — the body's look
        // three ahead, the static no-tick frame the skeleton's stage
        // shares): the SHIVER clock the O pair fills (the lerp the
        // getter serves — BOTH halves the 10 the partial sweep keeps,
        // the any-partial answer the 1.0 swing wears)
        let mut warden = crate::sculk::WardenMob::new(501, app.mobs[0].x, app.mobs[0].y, app.mobs[0].z);
        warden.tendril_animation_o = crate::sculk::WARDEN_TENDRILS_SHIVER_ANIMATION;
        warden.tendril_animation = crate::sculk::WARDEN_TENDRILS_SHIVER_ANIMATION;
        app.mobs.clear();
        app.wardens.push(warden);
        // THE WITH/WITHOUT (the same frame the warden the only delta
        // wears — the skeleton armor's own diff law speaks)
        let with = third_person_frame(&mut app);
        let wardens = std::mem::take(&mut app.wardens);
        let without = third_person_frame(&mut app);
        let diff = with
            .iter()
            .zip(without.iter())
            .filter(|(a, b)| a != b)
            .count();
        assert!(
            diff > 512,
            "the warden's body the frame lights ({diff} bytes differ)"
        );
        // THE DEAD GATE (the health-0 body the frame never wears —
        // the corpse the removal lane keeps): the dead re-render the
        // without frame the EXACT bytes answer
        let mut dead = wardens.clone();
        dead[0].health = 0.0;
        app.wardens = dead;
        let dead_frame = third_person_frame(&mut app);
        assert_eq!(
            dead_frame, without,
            "the dead warden the frame never wears"
        );
    }




    /// THE WARDEN'S EMISSIVE GLOW (the LivingEntityEmissiveLayer
    /// passes the render arm serves): the sha1-verified layer loads
    /// the frame answers, the BIOLUMINESCENT glow the with/without
    /// diff lights (the constant-1.0 alpha the head/limb glow rects
    /// blend, the stage camera's far look the warden's lit side
    /// faces), the HEART glow the heartbeat clock fills (the flipped
    /// stage camera the chest glow's own unwrap side turns — the full
    /// clock the chest dots light, the drained clock the
    /// `<= 1.0E-5` gate the texture's own absence answers), the
    /// LEQUAL tie the coplanar overlay the body's own depth answers.
    #[test]
    fn the_warden_glow_the_layers_the_frame_wears() {
        let _render_lane = render_lane();
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join(mc_registry::generated::skin::WARDEN_RUNTIME_PATH).exists());
        let Some(root) = root else { return };
        let Some(warden_model) = crate::avatar::WardenModel::load(root) else {
            return;
        };
        let Some(heart_model) =
            crate::avatar::WardenModel::load_layer(root, crate::avatar::WardenLayer::Heart)
        else {
            return;
        };
        let Some(biolum_model) = crate::avatar::WardenModel::load_layer(
            root,
            crate::avatar::WardenLayer::Bioluminescent,
        ) else {
            return;
        };
        let mut app = App::new(3);
        armored_skeleton_stage(&mut app);
        app.warden_model = Some(warden_model);
        app.warden_heart_model = Some(heart_model);
        app.warden_biolum_model = Some(biolum_model);
        // THE WARDEN (the stage's own side seat — the body's z the TWO
        // cameras share: the yaw-0 camera the 4-ahead 2-right view
        // speaks, the flipped heart camera the SAME seat mirrors — the
        // warden the player silhouette clears both ways, the dev-152
        // round's own law: the render_avatar's runtime-root walk the
        // player body DRAWS now, the behind-the-body seat the body's
        // own shadow the degenerate lane never wore)
        let (bx, by, bz) = {
            let b = app.body.as_ref().unwrap();
            (b.x, b.y, b.z)
        };
        let warden = crate::sculk::WardenMob::new(502, bx + 2.0, by, bz);
        app.mobs.clear();
        app.wardens.push(warden);
        // THE BIOLUM AXIS (the constant-1.0 layer the frame lights):
        // the jar_root law the lazy pass the REFILL arms — the take()
        // the load's own success DEFEATS (the refilled frame the full
        // stack wears again, the 0-diff the refill's artifact the
        // dev-152 record's '383 cargo / 0 repo' split the honest lane
        // now answers — the repo lane the spots loads the SUCCESS wore,
        // the cargo lane the jar_root walk the same world sees). THE
        // TEXTURE SWAP the axis isolates: the biolum lane the quads
        // the SAME state wears, the texels the spots1 texture speaks —
        // the two textures the glow palette differ, the contribution
        // the diff bytes keep (the refill the swap never rides: both
        // frames the Some the lane draws).
        let Some(spots1_tex) = crate::avatar::WardenModel::load_layer(
            root,
            crate::avatar::WardenLayer::PulsatingSpots1,
        ) else {
            return;
        };
        let plain0 = third_person_frame(&mut app);
        let saved = app.warden_biolum_model.take();
        app.warden_biolum_model = Some(spots1_tex);
        let swapped = third_person_frame(&mut app);
        app.warden_biolum_model = saved;
        let biolum_diff = plain0
            .iter()
            .zip(swapped.iter())
            .filter(|(a, b)| a != b)
            .count();
        assert!(
            biolum_diff > 100,
            "the biolum glow the frame lights ({biolum_diff} bytes differ)"
        );
        // THE HEART AXIS (the flipped stage camera the chest glow's
        // own unwrap side turns): the full heartbeat clock the chest
        // dots light, the drained clock the gate skips
        {
            let body = app.body.as_mut().unwrap();
            body.yaw = 180.0;
        }
        app.wardens[0].heart_animation_o = crate::sculk::WARDEN_HEARTBEAT_ANIMATION;
        app.wardens[0].heart_animation = crate::sculk::WARDEN_HEARTBEAT_ANIMATION;
        let full180 = third_person_frame(&mut app);
        app.wardens[0].heart_animation_o = 0;
        app.wardens[0].heart_animation = 0;
        let drained180 = third_person_frame(&mut app);
        let heart_diff = full180
            .iter()
            .zip(drained180.iter())
            .filter(|(a, b)| a != b)
            .count();
        assert!(
            heart_diff > 300,
            "the heart glow the frame lights ({heart_diff} bytes differ)"
        );
        // THE GATE'S OWN HONESTY (the drained clock the layer skips —
        // the same frame the texture's ABSENCE answers)
        let saved_heart = app.warden_heart_model.take();
        let taken180 = third_person_frame(&mut app);
        app.warden_heart_model = saved_heart;
        assert_eq!(
            drained180, taken180,
            "the drained clock the gate's skip the absence answers"
        );
    }



    /// THE DISPENSER'S OWN THROWN SHOT (javap the 26.2 walk the
    /// autopush-140 lane banks — the snowball the shot lane wears):
    /// the lever pulse the beat answers, the SNOWBALL the kind byte
    /// the eggs vec flies, the position the `dispense_projectile_position`
    /// fold keeps, the velocity the 1.1 power's band carries, the
    /// slot the shrink law spends, the 1002 the launch voice speaks.
    #[test]
    fn the_dispenser_the_snowball_shot_the_thrown_lane_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let snow_id = mc_registry::item_id_by_name("snowball").expect("the wire table");
        // THE SKY PAD'S OWN STAGE (the egg throw test's own shape —
        // the terrain's tops the pad's level never reaches)
        let (px, pz) = (16i32, 16i32);
        let gy = 30i32;
        {
            let world = app.offline_world.as_mut().unwrap();
            for dx in -4..=4 {
                for dz in -4..=4 {
                    world.set(px + dx, gy - 1, pz + dz, crate::world::STONE);
                }
            }
            // THE DISPENSER'S OWN SEAT (the facing the SOUTH byte
            // stamps, the front cell the pad's own air keeps)
            world.set(px, gy, pz, crate::world::DISPENSER);
            world.set_variant(
                px,
                gy,
                pz,
                crate::world::dispenser_variant(crate::world::DISPENSER_FACING_SOUTH, false),
            );
        }
        // THE STORE'S OWN ROW (the snowball the first slot holds, the
        // nine the DispenserBlockEntity's own size keeps)
        app.container_stores.insert(
            (px, gy, pz),
            vec![(snow_id, 2), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
        );
        // THE PULSE'S OWN TAIL (the consult arm's own schedule shape —
        // the due the +4 cooldown keeps, the drain the frame's own
        // crossing answers)
        {
            let world = app.offline_world.as_mut().unwrap();
            world.schedule_dispenser_tick(
                app.frame + crate::world::DISPENSER_COOLDOWN,
                px,
                gy,
                pz,
            );
        }
        for _ in 0..6 {
            app.update();
        }
        assert_eq!(app.eggs.len(), 1, "the snowball the shot lane flies");
        let e = &app.eggs[0];
        assert_eq!(
            e.kind,
            crate::mob::ThrownProjectileKind::Snowball,
            "the kind byte the item's own asProjectile speaks"
        );
        // THE SPAWN'S OWN FOLD (the `0.7 + Vec3(0.0, 0.1, 0.0)` lambda
        // the south step scales — the flight ticks the drained window
        // wears, the spread's tiny drift the tolerance keeps)
        assert!(
            (e.x - (f64::from(px) + 0.5)).abs() < 0.5,
            "the x the center keeps ({})",
            e.x
        );
        assert!(
            e.y > f64::from(gy) + 0.4 && e.y < f64::from(gy) + 0.8,
            "the y the 0.1 nudge the fall fold keeps ({})",
            e.y
        );
        assert!(
            e.z > f64::from(pz) + 1.0 && e.z < f64::from(pz) + 5.0,
            "the z the 0.7 offset the flight keeps ({})",
            e.z
        );
        // THE VELOCITY'S OWN BAND (the 1.1 power the triangle spread's
        // ±0.1034 pair keeps, the drag the flight ticks wear)
        assert!(
            e.vz > 1.0 && e.vz < 1.25,
            "the south step the 1.1 scale speaks ({})",
            e.vz
        );
        assert!(e.vx.abs() < 0.2 && e.vy.abs() < 0.2, "the flanks the spread keeps");
        // THE SLOT'S OWN SPEND (the count-1 law the write-back wears)
        assert_eq!(
            app.container_stores[&(px, gy, pz)][0],
            (snow_id, 1),
            "the shrink the setItem law answers"
        );
        // THE LAUNCH'S OWN VOICE (the 1002 the playSound walk answers)
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("block.dispenser.launch @")),
                "the 1002 the launch voice rode"
            );
        }
    }


    /// THE DISPENSER'S OWN EGG SHOT (the sibling kind the same lane
    /// wears — the kind byte the `asProjectile` factory's own item
    /// speaks, the eggs vec the ThrownEgg's own flight answers).
    #[test]
    fn the_dispenser_the_egg_shot_the_kind_byte_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let egg_id = mc_registry::item_id_by_name("egg").expect("the wire table");
        let (px, pz) = (16i32, 16i32);
        let gy = 30i32;
        {
            let world = app.offline_world.as_mut().unwrap();
            for dx in -4..=4 {
                for dz in -4..=4 {
                    world.set(px + dx, gy - 1, pz + dz, crate::world::STONE);
                }
            }
            world.set(px, gy, pz, crate::world::DISPENSER);
            world.set_variant(
                px,
                gy,
                pz,
                crate::world::dispenser_variant(crate::world::DISPENSER_FACING_SOUTH, false),
            );
        }
        app.container_stores.insert(
            (px, gy, pz),
            vec![(egg_id, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
        );
        {
            let world = app.offline_world.as_mut().unwrap();
            world.schedule_dispenser_tick(
                app.frame + crate::world::DISPENSER_COOLDOWN,
                px,
                gy,
                pz,
            );
        }
        for _ in 0..6 {
            app.update();
        }
        assert_eq!(app.eggs.len(), 1, "the egg the shot lane flies");
        assert_eq!(
            app.eggs[0].kind,
            crate::mob::ThrownProjectileKind::Egg,
            "the kind byte the egg's own factory speaks"
        );
        assert_eq!(
            app.container_stores[&(px, gy, pz)][0],
            (0, 0),
            "the last egg the emptied slot's honest row keeps"
        );
        assert!(
            app.drops.is_empty(),
            "the shot lane the DEFAULT eject never wears"
        );
    }


    /// THE DEFAULT'S OWN KEEP (the stone the spawnItem walk serves —
    /// the eject the ItemDrop wears, the thrown family the shot lane
    /// never sees, the 1000 the dispense voice speaks).
    #[test]
    fn the_dispenser_the_stone_eject_the_default_keeps() {
        let mut app = App::new(3);
        app.enter_world();
        let stone_id = mc_registry::item_id_by_name("stone").expect("the wire table");
        let (px, pz) = (16i32, 16i32);
        let gy = 30i32;
        {
            let world = app.offline_world.as_mut().unwrap();
            for dx in -4..=4 {
                for dz in -4..=4 {
                    world.set(px + dx, gy - 1, pz + dz, crate::world::STONE);
                }
            }
            world.set(px, gy, pz, crate::world::DISPENSER);
            world.set_variant(
                px,
                gy,
                pz,
                crate::world::dispenser_variant(crate::world::DISPENSER_FACING_SOUTH, false),
            );
        }
        app.container_stores.insert(
            (px, gy, pz),
            vec![(stone_id, 3), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
        );
        {
            let world = app.offline_world.as_mut().unwrap();
            world.schedule_dispenser_tick(
                app.frame + crate::world::DISPENSER_COOLDOWN,
                px,
                gy,
                pz,
            );
        }
        for _ in 0..6 {
            app.update();
        }
        assert!(app.eggs.is_empty(), "the stone the thrown lane never rides");
        assert_eq!(app.drops.len(), 1, "the eject the ItemDrop wears");
        assert_eq!(
            app.container_stores[&(px, gy, pz)][0],
            (stone_id, 2),
            "the shrink the count-1 law keeps"
        );
    }

    /// THE SHULKER FRONT'S OWN INSERT (the autopush-147 round — the
    /// getContainerAt walk the DyeableBlockEntity container the one
    /// predicate gains): the offline dropper the south-facing beat
    /// wears, the front cell's shulker box the walk answers, the
    /// 27-slot store the first beat mints, the addItem copy the
    /// `copyWithCount(1)` tail wears, the source slot the shrink
    /// keeps, the NO-SOUND arm the container lane keeps (the drops
    /// vec the eject never sees).
    #[test]
    fn the_droppers_front_shulker_the_insert_wears() {
        let dirt = mc_registry::item_id_by_name("dirt").expect("the wire table");
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
            world.set(px, gy, pz, crate::world::DROPPER);
            world.set_variant(
                px,
                gy,
                pz,
                crate::world::dispenser_variant(crate::world::DISPENSER_FACING_SOUTH, false),
            );
            world.set(px, gy, pz + 1, crate::world::SHULKER_BOX);
        }
        let mut nine = vec![(0i32, 0i32); 9];
        nine[3] = (dirt, 5);
        app.container_stores.insert((px, gy, pz), nine);
        {
            let world = app.offline_world.as_mut().unwrap();
            world.schedule_dispenser_tick(
                app.frame + crate::world::DISPENSER_COOLDOWN,
                px,
                gy,
                pz,
            );
        }
        for _ in 0..6 {
            app.update();
        }
        let sink = app
            .container_stores
            .get(&(px, gy, pz + 1))
            .expect("the shulker's own 27 the first beat mints");
        assert_eq!(sink.len(), 27, "the chest-sized store the shulker keeps");
        assert_eq!(sink[0], (dirt, 1), "the inserted copy the slot 0 wears");
        assert_eq!(
            app.container_stores[&(px, gy, pz)][3],
            (dirt, 4),
            "the source slot the shrink keeps"
        );
        assert!(
            app.drops.is_empty(),
            "the container arm the eject never wears"
        );
    }

    /// THE SHULKER FRONT'S OWN FULL KEEP (the addItem's false the
    /// vanilla tail wears): the 27 full seats the insert refuses, the
    /// stack the count the walk keeps (the setItem write-back the
    /// insert-refused row the intact count answers), the eject the
    /// container-present arm never sees (the drop the lane never
    /// mints), the sink's own rows the refusal the untouched keeps.
    #[test]
    fn the_droppers_front_shulker_the_full_keep_wears() {
        let dirt = mc_registry::item_id_by_name("dirt").expect("the wire table");
        let stone = mc_registry::item_id_by_name("stone").expect("the wire table");
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
            world.set(px, gy, pz, crate::world::DROPPER);
            world.set_variant(
                px,
                gy,
                pz,
                crate::world::dispenser_variant(crate::world::DISPENSER_FACING_SOUTH, false),
            );
            world.set(px, gy, pz + 1, crate::world::SHULKER_BOX);
        }
        app.container_stores.insert((px, gy, pz + 1), vec![(stone, 64); 27]);
        let mut nine = vec![(0i32, 0i32); 9];
        nine[3] = (dirt, 5);
        app.container_stores.insert((px, gy, pz), nine);
        {
            let world = app.offline_world.as_mut().unwrap();
            world.schedule_dispenser_tick(
                app.frame + crate::world::DISPENSER_COOLDOWN,
                px,
                gy,
                pz,
            );
        }
        for _ in 0..6 {
            app.update();
        }
        assert_eq!(
            app.container_stores[&(px, gy, pz)][3],
            (dirt, 5),
            "the refused insert the stack keeps"
        );
        assert_eq!(
            app.container_stores[&(px, gy, pz + 1)][0],
            (stone, 64),
            "the sink's own rows the refusal never moves"
        );
        assert!(
            app.drops.is_empty(),
            "the container-present arm the eject never wears"
        );
    }

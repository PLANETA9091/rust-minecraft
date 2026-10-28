use super::*;

/// THE ACCUMULATOR'S OWN LAW (ScrollWheelHandler.onMouseScroll code
/// 148): the sub-step deltas accumulate across frames (the folded
/// 0.6 + 0.6 = 1.2 -> ONE step, the 0.2 remainder stays), the
/// direction flip RESETS the stale remainder, and the X tilt rides
/// the inverted arm the call site keeps (`-vector2i.x`).
#[test]
fn the_scroll_accumulator_keeps_the_remainder_and_resets() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // the sub-step trackpad frame: 0.6 folded < 1 — no step
    app.on_hotbar_scroll(0.0, 0.6);
    assert_eq!(app.selected_hotbar, 0, "the sub-step stays silent");
    assert!(
        (app.scroll_acc_y - 0.6).abs() < 1e-9,
        "the accumulator holds"
    );
    // the crossing: +0.6 more -> 1.2 -> ONE step, the 0.2 remainder
    app.on_hotbar_scroll(0.0, 0.6);
    assert_eq!(app.selected_hotbar, 8, "the crossing fires (up-scroll)");
    assert!((app.scroll_acc_y - 0.2).abs() < 1e-9, "the remainder kept");
    // the DIRECTION FLIP resets the stale positive remainder — the
    // first negative frame stays sub-step
    app.on_hotbar_scroll(0.0, -0.6);
    assert_eq!(app.selected_hotbar, 8, "the flipped frame stays silent");
    assert!(
        (app.scroll_acc_y + 0.6).abs() < 1e-9,
        "the reset + the fold"
    );
    // the crossing the other way
    app.on_hotbar_scroll(0.0, -0.6);
    assert_eq!(app.selected_hotbar, 0, "the down-scroll crossing fires");
    // the X TILT (the call site's inverted arm): the right tilt
    // moves toward the higher numbers
    app.on_hotbar_scroll(1.0, 0.0);
    assert_eq!(app.selected_hotbar, 1, "the tilt's own direction");
}

/// THE SCROLL'S LIVE ECHO (the SelectHotbar command the number-key
/// lane shares): the LIVE session's belief rides the command
/// channel, the same walk the keys answer.
#[test]
fn the_scroll_wheel_echoes_the_live_session() {
    use crate::world_view::ContainerCommand;
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.on_hotbar_scroll(0.0, -1.0);
    assert_eq!(app.selected_hotbar, 1, "the live belief moved");
    match click_rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::SelectHotbar { selection } => assert_eq!(selection, 1),
        other => panic!("expected a selection, got {other:?}"),
    }
}

/// THE CONTAINER'S OWN mouseScrolled — THE GATE CHAIN (the 26.2
/// AbstractContainerScreen.mouseScrolled walk): the hovered-slot
/// verdict the SAME 16x16 rects the click lane keeps — null (the
/// between-slots point), empty (the cleared store row), and the
/// filled non-bundle stack (no action matches) all return FALSE,
/// the hotbar the screen arm never touches (the selection and the
/// MouseHandler fold the walk leaves where they were — the
/// scroll_acc the screen event never advances).
#[test]
fn the_container_scroll_walks_the_gates() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    app.inventory.items[0] = (62, 27); // the cobblestone the hotbar keeps
    app.hotbar_slots = app.inventory.hotbar();
    // the hovered verdict the rects speak
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 0);
    assert_eq!(
        app.hovered_container_stack(ox + 8, oy + 8),
        Some((62, 27)),
        "the slot's stack the hover keeps"
    );
    assert_eq!(
        app.hovered_container_stack(app.inv_panel.0 + 90, app.inv_panel.1 + 10),
        None,
        "the between-slots point the hover refuses"
    );
    // THE WALK: every gate's own false — no slot, empty slot, the
    // non-bundle stack no action matches
    assert!(
        !app.on_container_scroll(0.0, -1.0, app.inv_panel.0 + 90, app.inv_panel.1 + 10),
        "the hoveredSlot-null arm"
    );
    assert!(
        !app.on_container_scroll(0.0, -1.0, ox + 8, oy + 8),
        "the filled cobble matches no action"
    );
    let (ox9, oy9) = inventory_slot_origin(app.inv_panel, 9);
    assert!(
        !app.on_container_scroll(0.0, -1.0, ox9 + 8, oy9 + 8),
        "the empty slot's hasItem gate"
    );
    // THE SCREEN-CONSUMES LAW: the hotbar untouched through the
    // whole walk — the selection, the row, the fold's accumulator
    assert_eq!(app.selected_hotbar, 0, "the selection never moved");
    assert_eq!(app.hotbar_slots[0], (62, 27), "the row untouched");
    assert_eq!(app.scroll_acc_x, 0.0, "the fold never rode");
    assert_eq!(app.scroll_acc_y, 0.0, "the fold never rode");
    // the walk only the container screens answer — the world's
    // wheel the hotbar arm owns (the screen arm the routing keeps)
    app.open(ScreenKind::World);
    assert_eq!(app.hovered_container_stack(ox + 8, oy + 8), None);
}

/// THE BUNDLE LANE'S OWN WALL (the BundleMouseActions.onMouseScrolled
/// opening gate — `getNumberOfItemsToShow(stack) == 0 -> return
/// false` at the bytecode 0-13): the hovered BUNDLE matches the
/// BUNDLES tag (the 17 names the verdict rides), and the
/// bundle_contents component the client's stacks never carry reads
/// the EMPTY default's 0 — the false the fold's gate precedes (the
/// accumulator untouched), the toggleSelectedBundleItem tail the
/// contents round owns.
#[test]
fn the_bundle_lane_matches_and_stops_at_the_contents_wall() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    let bundle = mc_registry::item_id_by_name("bundle").expect("the bundle the wire keeps");
    // THE TAG VERDICT (ItemTags.BUNDLES): the plain bundle + the
    // dyed variants the 17-name law keeps, the neighbors it refuses
    assert!(App::is_bundle_item(bundle), "the plain bundle rides");
    for name in ["black_bundle", "light_blue_bundle", "red_bundle"] {
        let id = mc_registry::item_id_by_name(name).expect(name);
        assert!(App::is_bundle_item(id), "{name} rides the tag");
    }
    assert!(
        !App::is_bundle_item(62),
        "the cobblestone no action matches"
    );
    // the hovered bundle stack
    app.inventory.items[3] = (bundle, 1);
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 3);
    assert_eq!(
        app.hovered_container_stack(ox + 8, oy + 8),
        Some((bundle, 1)),
        "the bundle the hover keeps"
    );
    // THE WALK: the match rides, the wall stops it — the contents
    // component the model never carries, the 0 the gate reads
    assert!(
        !app.on_container_scroll(0.0, -1.0, ox + 8, oy + 8),
        "the empty contents the false keeps"
    );
    // the fold's gate order: repeated scrolls touch NOTHING — no
    // accumulator, no selection, no row move
    app.on_container_scroll(0.0, -1.0, ox + 8, oy + 8);
    assert_eq!(app.scroll_acc_x, 0.0, "the fold never rode");
    assert_eq!(app.scroll_acc_y, 0.0, "the fold never rode");
    assert_eq!(app.selected_hotbar, 0, "the selection never moved");
    assert_eq!(app.inventory.items[3], (bundle, 1), "the stack stayed");
}

/// THE CHEST SCREEN'S OWN SCROLL WALK (the same mouseScrolled the
/// generic_9x3 container rides): the grid + mirror rows the hover's
/// rect walk keeps — the filled non-bundle stack the false keeps,
/// the between-slots point the null arm holds, the bundle the same
/// contents wall stops. The offline stand-in's rects the render
/// lays (the test speaks them directly — the render-then-hit
/// pattern).
#[test]
fn the_chest_screen_rides_the_same_scroll_walk() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open(ScreenKind::Chest);
    let panel = chest_panel_origin(640, 360);
    app.chest_slots = vec![(0, 0); 27];
    app.chest_mirror = vec![(0, 0); 36];
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
    // the grid's stack: the oak the offline story keeps (63)
    app.chest_slots[5] = (63, 12);
    let (gx, gy) = chest_container_slot(panel, 5);
    assert_eq!(
        app.hovered_container_stack(gx + 8, gy + 8),
        Some((63, 12)),
        "the grid cell the hover keeps"
    );
    // the mirror row's stack (the player's own store the container
    // mirrors — the bundle rides here too)
    let bundle = mc_registry::item_id_by_name("bundle").unwrap();
    app.chest_mirror[9] = (bundle, 1);
    let (mx, my) = chest_mirror_slot(panel, 9);
    assert_eq!(
        app.hovered_container_stack(mx + 8, my + 8),
        Some((bundle, 1)),
        "the mirror row the hover keeps"
    );
    // THE WALK: the oak no action matches, the bundle the wall
    // stops, the between-slots point the null arm holds
    assert!(
        !app.on_container_scroll(0.0, -1.0, gx + 8, gy + 8),
        "the oak's honest false"
    );
    assert!(
        !app.on_container_scroll(0.0, -1.0, mx + 8, my + 8),
        "the bundle's contents wall"
    );
    assert!(
        !app.on_container_scroll(0.0, -1.0, panel.0 + 88, panel.1 + 2),
        "the between-slots null"
    );
    // the hotbar the container walk never touches
    assert_eq!(app.selected_hotbar, 0, "the selection never moved");
    assert_eq!(app.scroll_acc_x, 0.0, "the fold never rode");
}

/// THE BUNDLE CONTENTS' OWN WALK — THE PURE LAWS (the 26.2 pair
/// the toggle tail rides): `BundleContents.getNumberOfItemsToShow`'s
/// cap-to-whole-rows table (the `size > 12 ? 11 : 12` cap, the
/// `4 - size % 4` pad, the `min` tail — the EMPTY default's 0 the
/// wall reads, the 13th item's lone row the cap refuses, the full
/// bundle's 11) and `getNextScrollWheelSelection`'s exact loop (the
/// `prev - signum` walk, the `max(-1)` clamp the UNSELECTED state
/// rides — the -1 a rem_euclid would land `size - 2` from, the
/// wrap into `[0, n)` both directions keep).
#[test]
fn the_bundle_show_count_and_the_selection_law_speak_the_walk() {
    // THE SHOW-COUNT TABLE (the bytecode's own pins)
    for (size, want) in [
        (0, 0),   // the EMPTY default — the wall
        (1, 1),   // cap 12, rem 1, pad 3 -> min(1, 9)
        (2, 2),   // cap 12, rem 2, pad 2 -> min(2, 10)
        (3, 3),   // cap 12, rem 3, pad 1 -> min(3, 11)
        (4, 4),   // the whole row
        (5, 5),   // min(5, 9)
        (8, 8),   // the two rows
        (10, 10), // min(10, 10)
        (11, 11), // min(11, 11)
        (12, 12), // the cap's whole keep
        (13, 8),  // cap 11, rem 1, pad 3 -> min(13, 8)
        (14, 9),  // cap 11, rem 2, pad 2 -> min(14, 9)
        (16, 11), // cap 11, rem 0 -> min(16, 11)
        (20, 11), // the cap's own hold
        (64, 11), // the FULL bundle's 11
    ] {
        assert_eq!(
            App::bundle_number_of_items_to_show(size),
            want,
            "size {size} the show-count law keeps"
        );
    }
    // THE SELECTION LAW: the unselected -1 the clamp feeds
    assert_eq!(
        App::get_next_scroll_wheel_selection(1.0, -1, 5),
        4,
        "the -1 the +1 scroll wraps to the LAST (the clamp's own walk)"
    );
    assert_eq!(
        App::get_next_scroll_wheel_selection(-1.0, -1, 5),
        0,
        "the -1 the -1 scroll walks to the FIRST"
    );
    // the wrap the [0, n) domain keeps, both directions
    assert_eq!(
        App::get_next_scroll_wheel_selection(1.0, 0, 5),
        4,
        "the 0 the +1 scroll wraps past the front"
    );
    assert_eq!(
        App::get_next_scroll_wheel_selection(-1.0, 4, 5),
        0,
        "the last the -1 scroll wraps past the back"
    );
    // the mid-domain steps the signum rides — ONE index per event
    assert_eq!(App::get_next_scroll_wheel_selection(-1.0, 1, 5), 2);
    assert_eq!(App::get_next_scroll_wheel_selection(1.0, 3, 5), 2);
}

/// THE BUNDLE CONTENTS' OWN WALK — THE LIVE TAIL (the 26.2
/// `BundleMouseActions.onMouseScrolled` beyond the wall the old
/// model's EMPTY default kept): the contents row the map speaks
/// opens the gate, the action's OWN accumulator folds (the Y the
/// 2.0 the hotbar law pins — the sub-step remainder stays, the
/// flip resets), the selection walks the wrap law, and the MOVED
/// selection lands the LOCAL set + the SelectBundleItem echo the
/// worker's session speaks. The WALLS the walk keeps honest: no
/// map row (the getOrDefault default) and an EMPTY row read 0 —
/// the false the fold never rides. The OFFLINE guard the tail
/// keeps: no live lane, the verdict true, NOTHING moves (the
/// bytecode 0-7 the null connection keeps).
#[test]
fn the_bundle_contents_walk_moves_the_selection_and_echoes_the_live_session() {
    use crate::world_view::ContainerCommand;
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    let bundle = mc_registry::item_id_by_name("bundle").expect("the bundle the wire keeps");
    // the window-0 sync feeds BOTH lanes (the projection the hover
    // speaks, the contents row the walk's gate keeps) — the main
    // grid's first cell (store index 9, the menu's own slot id 9)
    // carries the bundle, the walk's own five-entry contents ride
    let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    slots[9] = (bundle, 1);
    app.live_inventory_updated(
        slots,
        vec![(9, vec![(62, 16), (63, 12), (4, 1), (5, 7), (17, 3)])],
        0,
        (0, 0),
    );
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 9);
    // THE FIRST SCROLL (the up-notch): the fold's -2 steps the 2.0
    // scale speaks, the direction -2, the signum -1 — the
    // UNSELECTED -1 walks to 0 (the clamp's own arm), the local set
    // + the echo ride
    assert!(
        app.on_container_scroll(0.0, -1.0, ox + 8, oy + 8),
        "the open gate the contents row keeps"
    );
    assert_eq!(
        app.bundle_contents[&9].selected_item_index, 0,
        "the selection the walk moved"
    );
    match click_rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::SelectBundleItem {
            slot_id,
            selected_item_index,
        } => {
            assert_eq!(slot_id, 9, "the menu's own slot id");
            assert_eq!(selected_item_index, 0, "the selection the wrap landed");
        }
        other => panic!("expected a bundle selection, got {other:?}"),
    }
    // the accumulator's remainder law: the whole steps emitted, the
    // fractional ride gone (the -2 the fold handed out whole)
    assert_eq!(
        app.bundle_scroll_acc_y, 0.0,
        "the remainder the handout kept"
    );
    // THE SECOND SCROLL (the down-notch): from 0, the +1 signum —
    // the max(-1) clamp feeds the -1, the wrap lands the LAST
    assert!(app.on_container_scroll(0.0, 1.0, ox + 8, oy + 8));
    assert_eq!(
        app.bundle_contents[&9].selected_item_index, 4,
        "the wrap the clamp's law keeps"
    );
    match click_rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::SelectBundleItem {
            slot_id,
            selected_item_index,
        } => {
            assert_eq!(slot_id, 9);
            assert_eq!(selected_item_index, 4);
        }
        other => panic!("expected a bundle selection, got {other:?}"),
    }
    // THE SUB-STEP FRAME (the trackpad's own 0.3): the fold's 0.6
    // never crosses, the direction 0 — the verdict TRUE, nothing
    // moves, the remainder stays for the next frame
    assert!(app.on_container_scroll(0.0, 0.3, ox + 8, oy + 8));
    assert_eq!(
        app.bundle_contents[&9].selected_item_index, 4,
        "the sub-step silent"
    );
    assert!(
        (app.bundle_scroll_acc_y - 0.6).abs() < 1e-9,
        "the remainder kept"
    );
    // THE FLIP RESET (the direction change discards the stale
    // remainder): the -0.4 frame folds -0.8 — the no-step arm, the
    // accumulator rides negative, nothing moves
    assert!(app.on_container_scroll(0.0, -0.4, ox + 8, oy + 8));
    assert_eq!(
        app.bundle_contents[&9].selected_item_index, 4,
        "the flipped frame silent"
    );
    assert!(
        (app.bundle_scroll_acc_y + 0.8).abs() < 1e-9,
        "the reset + the fold"
    );
    // THE CROSSING (the -0.4 more): -0.8 + -0.8 = -1.6 -> ONE step
    // (the d2i trunc toward zero), the -0.6 remainder rides — from
    // 4, the -1 signum walks forward, the wrap lands the FIRST
    assert!(app.on_container_scroll(0.0, -0.4, ox + 8, oy + 8));
    assert_eq!(
        app.bundle_contents[&9].selected_item_index, 0,
        "the crossing fired (the forward wrap)"
    );
    assert!(
        (app.bundle_scroll_acc_y + 0.6).abs() < 1e-9,
        "the remainder kept"
    );
    match click_rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::SelectBundleItem {
            slot_id,
            selected_item_index,
        } => {
            assert_eq!(slot_id, 9);
            assert_eq!(selected_item_index, 0);
        }
        other => panic!("expected a bundle selection, got {other:?}"),
    }
    // THE BOUNDS TAIL (the bytecode 10-15): a selection past the
    // show count refuses BOTH — no set, no packet
    let before = app.bundle_contents[&9].selected_item_index;
    app.toggle_selected_bundle_item(9, 5, 5);
    assert_eq!(
        app.bundle_contents[&9].selected_item_index, before,
        "the bounds the tail keeps"
    );
    assert!(click_rx.try_recv().is_err(), "no packet the bounds spoke");
    // THE WALLS: the bundle WITHOUT a map row (the getOrDefault
    // default the old model rode) and the EMPTY row read 0 — the
    // false the fold never rides (the crossing's -0.6 remainder
    // the untouched handler keeps — the wall precedes the fold)
    app.bundle_contents.remove(&9);
    assert!(
        !app.on_container_scroll(0.0, -1.0, ox + 8, oy + 8),
        "the no-row default's wall"
    );
    assert!(
        (app.bundle_scroll_acc_y + 0.6).abs() < 1e-9,
        "the wall the fold never rode"
    );
    app.bundle_contents
        .insert(9, BundleContents::new(Vec::new()));
    assert!(
        !app.on_container_scroll(0.0, -1.0, ox + 8, oy + 8),
        "the EMPTY contents' wall"
    );
    assert!(
        (app.bundle_scroll_acc_y + 0.6).abs() < 1e-9,
        "the wall the fold never rode"
    );
    // THE OFFLINE GUARD (the bytecode 0-7): the tail without the
    // live lane — the walk's verdict TRUE, nothing moves
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    app.inventory.items[9] = (bundle, 1);
    app.bundle_contents.insert(
        9,
        BundleContents::new(vec![(62, 16), (63, 12), (4, 1), (5, 7), (17, 3)]),
    );
    assert!(
        app.on_container_scroll(0.0, -1.0, ox + 8, oy + 8),
        "the gate the contents row opens offline too"
    );
    assert_eq!(
        app.bundle_contents[&9].selected_item_index,
        BundleContents::NO_SELECTED_INDEX,
        "the tail the null connection bails"
    );
    // the sub-step offline frames ride the same accumulator: the
    // 2.0 fold crossed (the -1.0 notch), the remainder the same
    assert!(
        (app.bundle_scroll_acc_y).abs() < 1e-9,
        "the -2 handed whole"
    );
}

/// THE CHEST'S OWN BUNDLE RIDE (the same onMouseScrolled the
/// generic_9x3's hovered slot walks): the mirror row's bundle the
/// map's MENU id keys (the chest's own numbering — the mirror row
/// 9 the 27+9=36 the rects keep), the wrap the same law lands, the
/// echo the same lane rides — and the SWITCH's clear the
/// lifecycle keeps (the window-scoped rows die with the screen).
#[test]
fn the_chest_bundle_rides_the_same_contents_walk() {
    use crate::world_view::ContainerCommand;
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Chest);
    // the LIVE window's identity (the sync's own lane): the chest
    // the wire opened — the ownership the switch's clear speaks
    app.chest_live_window = Some((5, 7));
    let panel = chest_panel_origin(640, 360);
    app.chest_slots = vec![(0, 0); 27];
    app.chest_mirror = vec![(0, 0); 36];
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
    let bundle = mc_registry::item_id_by_name("bundle").unwrap();
    // the mirror row 9 the chest's menu id 36 speaks, the contents
    // row the same id keys
    app.chest_mirror[9] = (bundle, 1);
    app.bundle_contents.insert(
        36,
        BundleContents::new(vec![(62, 16), (63, 12), (4, 1), (5, 7), (17, 3)]),
    );
    let (mx, my) = chest_mirror_slot(panel, 9);
    // THE WALK: the up-notch from the unselected -1 — the clamp
    // feeds the wrap, the LAST the selection lands
    assert!(
        app.on_container_scroll(0.0, 1.0, mx + 8, my + 8),
        "the mirror row's bundle the gate opens"
    );
    assert_eq!(
        app.bundle_contents[&36].selected_item_index, 4,
        "the wrap the clamp's law keeps"
    );
    match click_rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::SelectBundleItem {
            slot_id,
            selected_item_index,
        } => {
            assert_eq!(slot_id, 36, "the mirror row's own menu id");
            assert_eq!(selected_item_index, 4);
        }
        other => panic!("expected a bundle selection, got {other:?}"),
    }
    // THE SWITCH'S CLEAR (the lifecycle the open() keeps): the
    // window-scoped rows the next container re-learns — no stale
    // row rides a later screen's bundle
    app.open(ScreenKind::Inventory);
    assert!(
        app.bundle_contents.is_empty(),
        "the switch the lifecycle keeps"
    );
    assert_eq!(
        app.bundle_scroll_acc_x, 0.0,
        "the accumulator the switch resets"
    );
    assert_eq!(
        app.bundle_scroll_acc_y, 0.0,
        "the accumulator the switch resets"
    );
}

/// THE SET-SLOT SYNC'S OWN FEED (the live_container_updated lane —
/// the map's feeder round): the chest sync's bundle rows land the
/// map wholesale (the projection the store's snapshot speaks),
/// identical contents preserve the selection the client believes
/// (the wire never carries the index — the wrap's -1 default any
/// CHANGED component resets), and the window's drop clears the
/// rows the lifecycle keeps.
#[test]
fn the_chest_sync_feeds_the_bundle_rows() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the chest opens with a loaded bundle in the grid's slot 4 —
    // the menu id the rows key, the walk's own five-entry contents
    let chest = crate::world_view::LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: vec![(0, 0); 63],
        bundle_rows: vec![(4, vec![(62, 16), (63, 12), (4, 1), (5, 7), (17, 3)])],
        carried: (0, 0),
        state_id: 7,
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
    assert_eq!(app.screen, ScreenKind::Chest, "the window shows");
    assert_eq!(
        app.bundle_contents[&4].items.len(),
        5,
        "the row the sync fed"
    );
    assert_eq!(
        app.bundle_contents[&4].selected_item_index,
        BundleContents::NO_SELECTED_INDEX,
        "the wire's own -1 default the fresh row keeps"
    );
    // the client's own selection rides the toggle walk's belief
    app.bundle_contents.get_mut(&4).unwrap().selected_item_index = 2;
    // the SAME sync again (the projection refreshes every window):
    // identical contents preserve the belief — the selection the
    // wire never carries survives the refresh
    let chest = crate::world_view::LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: vec![(0, 0); 63],
        bundle_rows: vec![(4, vec![(62, 16), (63, 12), (4, 1), (5, 7), (17, 3)])],
        carried: (0, 0),
        state_id: 8,
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
    assert_eq!(
        app.bundle_contents[&4].selected_item_index, 2,
        "the identical row the belief survives"
    );
    // the CHANGED contents (the server's own update): the -1 the
    // wrap's default resets
    let chest = crate::world_view::LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: vec![(0, 0); 63],
        bundle_rows: vec![(4, vec![(62, 16)])],
        carried: (0, 0),
        state_id: 9,
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
    assert_eq!(app.bundle_contents[&4].items.len(), 1, "the changed row");
    assert_eq!(
        app.bundle_contents[&4].selected_item_index,
        BundleContents::NO_SELECTED_INDEX,
        "the changed component the reset speaks"
    );
    // the ROWLESS sync: no bundle-bearing slot — the map's
    // wholesale replacement the snapshot semantics keeps
    let chest = crate::world_view::LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: vec![(0, 0); 63],
        bundle_rows: Vec::new(),
        carried: (0, 0),
        state_id: 10,
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
    assert!(app.bundle_contents.is_empty(), "the rowless sync");
    // the WINDOW DROPS (the close): the rows die with the window,
    // the accumulator the same lifecycle resets
    app.live_view_updated(air_net_world(2, 1), None, None, None);
    assert!(
        app.bundle_contents.is_empty(),
        "the drop the lifecycle keeps"
    );
    assert_eq!(
        app.bundle_scroll_acc_x, 0.0,
        "the accumulator the drop resets"
    );
    assert_eq!(
        app.bundle_scroll_acc_y, 0.0,
        "the accumulator the drop resets"
    );
}

/// THE WINDOW-0 SYNC'S OWN FEED (the live_inventory_updated lane —
/// the player's OWN inventory window the store projection speaks):
/// the slots land verbatim (the index IS the menu id), the bundle
/// rows feed the map the chest lane's exact law keeps (the
/// wholesale replacement, the identical row's belief survival, the
/// CHANGED row's -1 reset, the rowless sync's emptiness), the
/// OFFLINE world the lane never rides, the LIVE CHEST owns the map
/// while its window stands (the ownership gate the numeric-domain
/// share keeps honest), and the E TOGGLE the rows survive (the
/// same window's component data the vanilla screen-open never
/// touches) while a chest-open re-learns from its own sync.
#[test]
fn the_window0_sync_feeds_the_inventory_bundles() {
    let mut app = App::new(3);
    let bundle = mc_registry::item_id_by_name("bundle").expect("the bundle the wire keeps");
    // the window-0 shape: the store's own slots keyed by the menu
    // id — the loaded bundle rides the HOTBAR's cell 4 (menu id
    // 40), the plain stack the main grid's first cell (menu id 9)
    let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    slots[9] = (745, 7);
    slots[40] = (bundle, 1);
    // THE OFFLINE REFUSAL: the live lane's own precondition — the
    // offline store stays the truth, the projection never lands
    app.live_inventory_updated(slots, vec![(40, vec![(62, 16)])], 0, (0, 0));
    assert_eq!(app.live_inv_slots[40], (0, 0), "the offline lane refused");
    assert!(
        app.bundle_contents.is_empty(),
        "no rows the offline lane feeds"
    );
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // THE FEED: the rows land keyed by the window's own menu ids
    app.live_inventory_updated(
        slots,
        vec![(40, vec![(62, 16), (63, 12), (4, 1), (5, 7), (17, 3)])],
        0,
        (0, 0),
    );
    assert_eq!(
        app.live_inv_slots[40],
        (bundle, 1),
        "the slot the projection keys"
    );
    assert_eq!(app.live_inv_slots[9], (745, 7), "the plain stack rides too");
    assert_eq!(
        app.bundle_contents[&40].items.len(),
        5,
        "the row the window-0 sync fed"
    );
    assert_eq!(
        app.bundle_contents[&40].selected_item_index,
        BundleContents::NO_SELECTED_INDEX,
        "the wire's own -1 default the fresh row keeps"
    );
    // the client's own selection rides the toggle walk's belief
    app.bundle_contents
        .get_mut(&40)
        .unwrap()
        .selected_item_index = 1;
    // the SAME sync again: identical contents preserve the belief
    app.live_inventory_updated(
        slots,
        vec![(40, vec![(62, 16), (63, 12), (4, 1), (5, 7), (17, 3)])],
        0,
        (0, 0),
    );
    assert_eq!(
        app.bundle_contents[&40].selected_item_index, 1,
        "the identical row the belief survives"
    );
    // the CHANGED contents: the -1 the wrap's default resets
    app.live_inventory_updated(slots, vec![(40, vec![(62, 16)])], 0, (0, 0));
    assert_eq!(app.bundle_contents[&40].items.len(), 1, "the changed row");
    assert_eq!(
        app.bundle_contents[&40].selected_item_index,
        BundleContents::NO_SELECTED_INDEX,
        "the changed component the reset speaks"
    );
    // the ROWLESS sync: no bundle-bearing slot — the map empties
    app.live_inventory_updated(slots, Vec::new(), 0, (0, 0));
    assert!(app.bundle_contents.is_empty(), "the rowless sync");
    // re-feed: the rows the E toggle's keep-law needs
    app.live_inventory_updated(slots, vec![(40, vec![(62, 16), (63, 12)])], 0, (0, 0));
    app.bundle_contents
        .get_mut(&40)
        .unwrap()
        .selected_item_index = 1;
    // THE E TOGGLE: the player's OWN inventory opens — the rows
    // survive (the component data the vanilla screen-open never
    // touches), the selection the toggle's belief keeps
    app.open_inventory();
    assert_eq!(app.screen, ScreenKind::Inventory, "the E key shows");
    assert_eq!(
        app.bundle_contents[&40].selected_item_index, 1,
        "the rows the own-window open keeps"
    );
    // back to the world and E again — the belief still stands
    app.open(ScreenKind::World);
    app.open_inventory();
    assert_eq!(
        app.bundle_contents[&40].selected_item_index, 1,
        "the E cycle the rows survive"
    );
    // the world screen back (the chest's window the world's own
    // lane opens) — THE OWNERSHIP GATE: the chest's sync lands the
    // map, the inventory feed the foreign window's ownership
    // silences (the numeric domain the two windows share, a stale
    // row never rides)
    app.open(ScreenKind::World);
    let chest = crate::world_view::LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: vec![(0, 0); 63],
        bundle_rows: vec![(9, vec![(5, 7)])],
        carried: (0, 0),
        state_id: 7,
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
    assert_eq!(app.screen, ScreenKind::Chest, "the window shows");
    assert_eq!(app.bundle_contents[&9].items.len(), 1, "the chest row owns");
    app.live_inventory_updated(slots, vec![(40, vec![(62, 16)])], 0, (0, 0));
    assert_eq!(
        app.bundle_contents[&9].items.len(),
        1,
        "the chest row the inventory feed never displaced"
    );
    assert!(
        app.bundle_contents.get(&40).is_none(),
        "the inventory row the ownership gate refused"
    );
    // the chest drops: the rows die with the window, the next
    // inventory sync the map re-learns
    app.live_view_updated(air_net_world(2, 1), None, None, None);
    assert!(
        app.bundle_contents.is_empty(),
        "the drop the lifecycle keeps"
    );
    app.live_inventory_updated(slots, vec![(40, vec![(62, 16)])], 0, (0, 0));
    assert!(app.bundle_contents.contains_key(&40), "the feed re-lands");
    // the CHEST-OPEN clear: a foreign window's open re-learns from
    // its own sync — the inventory rows it never rides
    app.open(ScreenKind::Chest);
    assert!(
        app.bundle_contents.is_empty(),
        "the foreign open the clear keeps"
    );
}

/// THE WIRE INVENTORY SCREEN'S OWN WALK (the vanilla scroll chain
/// over the player's OWN store): the window-0 projection the hover
/// speaks (the hotbar cell and the main-grid cell both keyed by
/// the menu id), the bundle the player carries in their own
/// inventory the wheel walks — the fold, the selection, the
/// SelectBundleItem echo the menu id carries. The OFFLINE store
/// the walk never touches (the live lane's own truth).
#[test]
fn the_wire_inventory_screen_walks_the_players_own_bundle() {
    use crate::world_view::ContainerCommand;
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let bundle = mc_registry::item_id_by_name("bundle").expect("the bundle the wire keeps");
    // the store's own shape: the loaded bundle the HOTBAR's cell 4
    // (menu id 40) and a second bundle the main grid's cell 12
    // (menu id 12) — the menu ids the window-0 numbering keys
    let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    slots[40] = (bundle, 1);
    slots[12] = (bundle, 1);
    slots[9] = (745, 7); // the plain stack the no-action arm keeps
    let contents = vec![(62, 16), (63, 12), (4, 1), (5, 7), (17, 3)];
    app.live_inventory_updated(
        slots,
        vec![(40, contents.clone()), (12, contents.clone())],
        0,
        (0, 0),
    );
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE HOTBAR CELL (store index 4, menu id 40): the hover the
    // window-0 projection speaks — the live lane's own truth, the
    // offline store untouched
    let (hx, hy) = inventory_slot_origin(app.inv_panel, 4);
    assert_eq!(
        app.hovered_container_stack(hx + 8, hy + 8),
        Some((bundle, 1)),
        "the wire's own stack the hover keeps"
    );
    assert_eq!(
        app.inventory.items[4],
        (0, 0),
        "the offline store the walk never touched"
    );
    // THE WALK: the scroll moves the selection, the echo the menu
    // id 40 carries — the packet the server's own bundle speaks
    assert!(
        app.on_container_scroll(0.0, -1.0, hx + 8, hy + 8),
        "the open gate the live row keeps"
    );
    assert_eq!(
        app.bundle_contents[&40].selected_item_index, 0,
        "the selection the walk moved"
    );
    match click_rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::SelectBundleItem {
            slot_id,
            selected_item_index,
        } => {
            assert_eq!(slot_id, 40, "the hotbar cell's own menu id");
            assert_eq!(selected_item_index, 0);
        }
        other => panic!("expected a bundle selection, got {other:?}"),
    }
    // THE MAIN-GRID CELL (menu id 12): the same walk over the
    // store's own rows — the id the second bundle's echo carries
    let (gx, gy) = inventory_slot_origin(app.inv_panel, 12);
    assert!(app.on_container_scroll(0.0, 1.0, gx + 8, gy + 8));
    assert_eq!(
        app.bundle_contents[&12].selected_item_index, 4,
        "the wrap the clamp's law keeps"
    );
    match click_rx.try_recv().expect("the echo rode the lane") {
        ContainerCommand::SelectBundleItem {
            slot_id,
            selected_item_index,
        } => {
            assert_eq!(slot_id, 12, "the main grid's own menu id");
            assert_eq!(selected_item_index, 4);
        }
        other => panic!("expected a bundle selection, got {other:?}"),
    }
    // the plain stack the same hover walk keeps honest — no
    // action matches, no packet rides
    let (px, py) = inventory_slot_origin(app.inv_panel, 9);
    assert!(
        !app.on_container_scroll(0.0, -1.0, px + 8, py + 8),
        "the plain stack's honest false"
    );
    assert!(
        click_rx.try_recv().is_err(),
        "no packet the plain stack spoke"
    );
}

/// THE PICK'S WIRE HALF (the gameMode.handlePickItemFromBlock walk
/// — the packet send ALONE): the LIVE session's middle-click rides
/// the PickBlock command with the clip's hit voxel and the Ctrl
/// flag verbatim, the store the server's to move — NO local
/// prediction (the offline walk the pick's own lane keeps is the
/// OFFLINE world's answer), the voxels whole, the selection and
/// the hotbar row untouched (the Set Held Slot echo the verdict's
/// own ride back is the only mover). The MISS the gates keep
/// silent ON the wire too — the sky aim sends NOTHING.
#[test]
fn the_wire_pick_sends_the_block_packet() {
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
    // the camera: the eye 3.0 above the floor plane, straight down
    app.net_camera = Some(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, floor_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // the plain middle-click: the packet's includeData byte LOW
    app.on_pick_block(false);
    match click_rx.try_recv().expect("the pick rode the lane") {
        ContainerCommand::PickBlock { pos, include_data } => {
            assert_eq!(pos, (c, floor_y, c), "the hit voxel");
            assert!(!include_data, "the plain form's flag");
        }
        other => panic!("expected a PickBlock echo, got {other:?}"),
    }
    // the CTRL form: the flag rides the packet's includeData slot
    app.on_pick_block(true);
    match click_rx.try_recv().expect("the ctrl pick rode the lane") {
        ContainerCommand::PickBlock { pos, include_data } => {
            assert_eq!(pos, (c, floor_y, c), "the same hit voxel");
            assert!(include_data, "the ctrl form's flag");
        }
        other => panic!("expected a PickBlock echo, got {other:?}"),
    }
    // the local voxels are untouched — the server's own verdict
    // (the tryPickItem walk the integrated server's bytecode
    // speaks) owns the store on this lane
    assert_eq!(
        app.net_world.as_ref().unwrap().get(c, floor_y, c),
        crate::world::STONE,
        "no local prediction on the networked lane"
    );
    // the MISS: the sky aim sends NOTHING (the second command
    // stays consumed — the channel's emptiness the silence is)
    app.net_camera.as_mut().unwrap().pitch = std::f32::consts::FRAC_PI_2;
    app.on_pick_block(false);
    assert!(
        click_rx.try_recv().is_err(),
        "the MISS never speaks on the wire"
    );
    assert_eq!(
        app.net_world.as_ref().unwrap().get(c, floor_y, c),
        crate::world::STONE,
        "the store whole throughout"
    );
}

#[test]
fn escape_closes_the_chest_back_to_the_world() {
    // the LIVE chest: Escape is the player's own close — back to the
    // world (the serverbound echo rides the send lane, pinned by the
    // escape test above; the flag drops so the next container-less
    // update is a no-op)
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let chest = crate::world_view::LiveContainer {
        id: 2,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: vec![(926, 1)],
        carried: (0, 0),
        state_id: 3,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
    assert_eq!(app.screen, ScreenKind::Chest);
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::World, "the container closed");
    assert!(!app.chest_is_live);
    // the following container-less update does not re-open anything
    app.live_view_updated(air_net_world(2, 2), None, None, None);
    assert_eq!(app.screen, ScreenKind::World);

    // the OFFLINE chest closes the same way (the deterministic scene
    // is a screen too — Escape is its close, the world is under it)
    app.open_chest();
    assert_eq!(app.screen, ScreenKind::Chest);
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the offline chest closed too"
    );
}

#[test]
fn live_session_lost_kicks_to_the_disconnect_screen() {
    // IN the network world: a session death is the vanilla kick
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(
        Some("minecraft:swamp".into()),
        Skybox::Overworld,
        None,
        None,
    );
    let run = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    app.attach_live_run(1, run);
    app.live_session_lost("Server closed".to_string());
    assert_eq!(app.screen, ScreenKind::Disconnect);
    assert_eq!(app.disconnect_reason, "Server closed");
    assert_eq!(
        app.disconnect_title,
        tr("disconnect.lost").unwrap_or("Connection lost"),
        "the vanilla lost-connection title"
    );
    assert!(!app.world_is_network, "the level unloaded with the session");
    assert!(app.world_quads.is_none(), "the mesh dropped with the level");
    assert!(app.net_camera.is_none(), "the pose dropped with the level");
    assert!(
        app.net_biome.is_none(),
        "the biome situation dropped with the level"
    );
}

#[test]
fn the_world_skybox_pins_at_level_entry_and_rides_the_dimension() {
    // the vanilla SkyRenderState.skybox discipline: the extract pins
    // the dimension's skybox at level entry, the sky pipeline gates
    // on it, and re-entering the PROCEDURAL world re-pins Overworld
    let mut app = App::new(3);
    assert_eq!(app.world_skybox, Skybox::Overworld, "the App default");
    assert_eq!(app.world_fog, None, "no fog pair: the window treatment");
    assert_eq!(
        app.world_cloud_fog_end, None,
        "no stated cloud-fog end: the 2048.0 default's shape"
    );
    // a nether join ships Skybox::None: the sky pipeline renders no
    // sky state at all there (the extract returns before any exists)
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    // the nether's real 10/96 pins with the join (the fogged walls)
    app.enter_network_world(None, Skybox::None, Some((10.0, 96.0)), Some(512.0));
    assert!(app.world_is_network);
    assert_eq!(app.world_skybox, Skybox::None, "the nether pin");
    assert_eq!(app.world_fog, Some((10.0, 96.0)), "the stated pair pins");
    assert_eq!(
        app.world_cloud_fog_end,
        Some(512.0),
        "the stated cloud-fog end pins with the join"
    );
    // a procedural re-entry re-pins the overworld sky AND drops the
    // fog pair (the extract re-runs on every level entry — a network
    // nether never haunts the singleplayer world's sky or fog)
    app.enter_world();
    assert!(!app.world_is_network);
    assert_eq!(app.world_skybox, Skybox::Overworld, "the procedural re-pin");
    assert_eq!(app.world_fog, None, "the pair drops with the level");
    assert_eq!(
        app.world_cloud_fog_end, None,
        "the cloud-fog end drops with the level"
    );
    // the resolution discipline itself lives in the model: the
    // synced registry's own skybox, the codec default, or the
    // built-in table (WorldModel::skybox — pinned by the
    // world_model tests, the built-in verdicts included)
}

#[test]
fn live_run_flags_close_on_cancel_supersede_and_quit() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    // two joins dispatched (the frame thread attaches a flag each)
    let run_a = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let run_b = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    app.attach_live_run(1, std::sync::Arc::clone(&run_a));
    app.attach_live_run(2, std::sync::Arc::clone(&run_b));
    // join A's success arrives while the Connect screen shows: A is
    // THE session, B (superseded) closes
    app.login_requests.push(LoginRequest {
        id: 1,
        entry: 0,
        host: "h".into(),
        port: 1,
    });
    app.login_requests.push(LoginRequest {
        id: 2,
        entry: 0,
        host: "h".into(),
        port: 2,
    });
    app.login_finished(
        1,
        Ok(LoginResult::Loaded {
            username: crate::screens::PLAYER_NAME.to_string(),
            entity_id: 1,
            view_distance: 8,
            chunks: 1,
            world: air_net_world(2, 0),
            camera: None,
            spawn_biome: None,
            slim_arm: true,
            skybox: Skybox::Overworld,
            fog: None,
            cloud_fog_end: None,
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
        }),
    );
    assert!(app.world_is_network, "A adopted");
    // the profile-metadata consumer: the join's skin model IS the
    // first-person arm's selection (the wide --slim-arm stand-in is
    // overridden by a real login's textures metadata)
    assert!(app.slim_arm, "the slim profile rides the join onto the app");
    assert!(
        run_a.load(std::sync::atomic::Ordering::Relaxed),
        "the adopted session keeps running"
    );
    assert!(
        !run_b.load(std::sync::atomic::Ordering::Relaxed),
        "the superseded session closed"
    );
    assert_eq!(app.live_runs.len(), 1, "only the adopted flag remains");

    // Save and Quit closes the adopted session too
    app.paused = true;
    app.render_title_buttons_for_test();
    let quit = app
        .buttons
        .iter()
        .find(|b| b.id == "saveandquit")
        .expect("saveandquit")
        .clone();
    app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
    assert!(
        !run_a.load(std::sync::atomic::Ordering::Relaxed),
        "the quit closed the network session"
    );

    // a cancelled join's late success closes ITS session (C), and
    // nothing else is disturbed
    let run_c = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    app.attach_live_run(3, std::sync::Arc::clone(&run_c));
    app.open(ScreenKind::Connect);
    app.on_escape(); // the user cancelled while C was in flight
    assert_eq!(app.screen, ScreenKind::Servers);
    app.login_requests.push(LoginRequest {
        id: 3,
        entry: 0,
        host: "h".into(),
        port: 3,
    });
    app.login_finished(
        3,
        Ok(LoginResult::Loaded {
            username: crate::screens::PLAYER_NAME.to_string(),
            entity_id: 3,
            view_distance: 8,
            chunks: 1,
            world: air_net_world(2, 3),
            camera: None,
            spawn_biome: None,
            slim_arm: false,
            skybox: Skybox::Overworld,
            fog: None,
            cloud_fog_end: None,
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
        }),
    );
    assert!(
        !run_c.load(std::sync::atomic::Ordering::Relaxed),
        "the cancelled join's session closed"
    );
    assert_eq!(
        app.screen,
        ScreenKind::Servers,
        "the late success is still dropped"
    );
    assert_eq!(
        app.net_world
            .as_ref()
            .expect("dormant from the earlier join")
            .seed,
        0,
        "nothing adopted from a cancelled join (the old world stays dormant)"
    );
}

/// THE FIRST-PERSON BODY: the offline spawn, the look curve through
/// the App's own feed, the walk tick on the procedural voxels, and
/// the pause freeze (the vanilla pause's own law).
#[test]
fn the_body_spawns_walks_and_freezes_with_the_pause() {
    let mut app = App::new(3);
    app.enter_world();
    assert!(
        app.body.is_none(),
        "the headless discipline: no body until fed"
    );
    app.spawn_offline_body();
    let body = app.body.as_ref().expect("the offline body spawned");
    // the feet ON the procedural surface at the spawn column's
    // center, facing the flight camera's east
    let n = app.world_size as f64;
    assert_eq!(body.x, n / 2.0);
    assert_eq!(body.z, n / 2.0);
    assert!(
        body.y <= app.spawn_y as f64,
        "the feet at or below the recorded spawn height (the lake law)"
    );
    // THE LOOK: the App feed turns the body through the curve
    let yaw_before = app.body.as_ref().unwrap().yaw;
    app.on_look_delta(10.0, 0.0);
    let turned = app.body.as_ref().unwrap().yaw - yaw_before;
    assert!(
        (turned - 1.5).abs() < 1e-4,
        "10 px at the default sensitivity = 1.5 degrees, got {}",
        turned
    );
    // THE WALK: the forward key feeds, the update ticks the travel,
    // the camera moves south-ish (yaw east -> the forward is +X)
    app.on_movement_input(1.0, 0.0, false, false, false);
    let x_before = app.body.as_ref().unwrap().x;
    for _ in 0..12 {
        app.update();
    }
    let b = app.body.as_ref().unwrap();
    assert!(
        b.x > x_before,
        "the east-facing walk moved +X ({x_before} -> {})",
        b.x
    );
    assert!(b.on_ground, "the walk stayed on the terrain");
    // THE PAUSE: the freeze (no tick, no move)
    app.paused = true;
    let (x, y, z) = (b.x, b.y, b.z);
    app.on_movement_input(1.0, 0.0, true, false, false);
    for _ in 0..10 {
        app.update();
    }
    let b = app.body.as_ref().unwrap();
    assert_eq!(b.x, x, "the paused body never moves (x)");
    assert_eq!(b.y, y, "the paused body never moves (y)");
    assert_eq!(b.z, z, "the paused body never moves (z)");
}

/// THE NETWORKED BODY: the join's teleport pose materializes the
/// body; the plain refreshes (the same pose re-shipped) never
/// re-anchor it; a CHANGED pose (the server's correction) does.
#[test]
fn the_body_reanchors_on_the_teleport_and_holds_the_prediction() {
    let mut app = App::new(3);
    app.net_world = Some(crate::world::World {
        darkness_scale: 0.0,
        chunks_per_side: 2,
        seed: 0,
        difficulty: crate::difficulty::Difficulty::Normal,
        difficulty_locked: false,
        inhabited: None,
        // THE FIRE'S OWN QUEUE + LADDER the test stage never arms —
        // the empty store the offline walk alone fills
        fire_ticks: Vec::new(),
        fire_age: std::collections::HashMap::new(),
        portal_axis: std::collections::HashMap::new(), // the portal's own axis store: empty
        container_fill: std::collections::HashMap::new(), // the comparator lane: no fill
        frame_analog: std::collections::HashMap::new(), // the frame lane: no analog
        has_skylight: true,                            // the portal's own axis store: empty
        campfire_ticks: Vec::new(),                    // the campfire's own queue: empty
        torch_ticks: Vec::new(),                       // the torch's own queue: empty
        detector_ticks: Vec::new(),                    // the detector's own queue: empty
        comparator_ticks: Vec::new(),
        dispenser_ticks: Vec::new(),
        hopper_ticks: Vec::new(),
        repeater_ticks: Vec::new(),
        observer_ticks: Vec::new(),
        piston_ticks: Vec::new(),
        moving_states: Vec::new(),
        moving_ticks: Vec::new(),
        torch_toggles: Vec::new(), // the RECENT_TOGGLES window: empty
        bamboo_ticks: Vec::new(),  // the bamboo's own queue: empty
        chunks: vec![
            vec![
                crate::world::AIR;
                crate::world::CHUNK_X * crate::world::CHUNK_Y * crate::world::CHUNK_Z
            ];
            4
        ],
        light: None,
        day_tick: None,
        biomes: None,
        variants: None,
        ambient_light: None,
        block_light_tint: None,
        sky_light_factor: None,
        sky_light_color: None,
        night_vision_color: None,
        night_vision_factor: 0.0,
        rain_level: 0.0,
        thunder_level: 0.0,
        // THE OFFLINE SAVE'S OWN LANES the staged world never wears —
        // the gate the gen-time default keeps off, the journal the
        // session's rows never names
        journaling: false,
        edit_journal: Vec::new(),
        variant_journal: Vec::new(),
        saved_edits_mark: 0,
        saved_variants_mark: 0,
        chest_lids: std::collections::HashMap::new(),
        lid_gen: 0,
        lid_partial: 1.0,
    });
    app.net_world
        .as_mut()
        .unwrap()
        .set(8, 0, 8, crate::world::STONE);
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert!(app.body.is_none(), "no body until the first pose ships");
    let cam = crate::camera::Camera {
        pos: Vec3::new(8.5, 1.0 + crate::world_view::PLAYER_EYE_HEIGHT, 8.5),
        yaw: 0.0,
        pitch: 0.0,
    };
    app.live_view_updated(app.net_world.clone().unwrap(), Some(cam), None, None);
    let b = app
        .body
        .as_ref()
        .expect("the teleport pose materialized the body");
    assert_eq!(b.x, 8.5);
    assert!(
        (b.y - 1.0).abs() < 1e-4,
        "the feet anchor below the 1.62 eye (the f32 camera cast)"
    );
    assert_eq!(b.z, 8.5);
    // the prediction: walk forward (the camera yaw 0 = the vanilla
    // -180 = facing -Z); the same-pose refresh holds it
    app.on_movement_input(1.0, 0.0, false, false, false);
    for _ in 0..10 {
        app.update();
    }
    let walked_z = app.body.as_ref().unwrap().z;
    assert!(
        walked_z < 8.5,
        "the body walked its own prediction (z {})",
        walked_z
    );
    app.live_view_updated(app.net_world.clone().unwrap(), Some(cam), None, None);
    assert_eq!(
        app.body.as_ref().unwrap().z,
        walked_z,
        "the plain refresh never re-anchors (the server does not echo plain moves)"
    );
    // the CHANGED pose: the server's correction lands
    let corrected = crate::camera::Camera {
        pos: Vec3::new(20.5, 2.0 + crate::world_view::PLAYER_EYE_HEIGHT, 20.5),
        yaw: 1.0,
        pitch: 0.0,
    };
    app.live_view_updated(app.net_world.clone().unwrap(), Some(corrected), None, None);
    let b = app.body.as_ref().unwrap();
    assert_eq!(b.x, 20.5, "the correction re-baselines the body");
    assert!(
        (b.y - 2.0).abs() < 1e-4,
        "the corrected feet anchor (the f32 cast): {}",
        b.y
    );
}

/// THE FOOD TICK'S REGEN ARC (the stand-in server's own FoodData):
/// a hurt body on a FULL bar + saturation heals through the fast
/// lane's 10-tick beat, pays the exhaustion, and the fed body's
/// hurt-clock lane (the else tail) resets the timer.
#[test]
fn the_food_tick_heals_the_hurt_body_off_the_saturation() {
    let mut app = App::new(3);
    app.enter_world();
    // the hurt body on the full bar: the fast lane's own setup
    app.health = 14.0;
    app.food_data.set_saturation(6.0);
    for _ in 0..9 {
        app.update();
        assert_eq!(
            app.health, 14.0,
            "the beat walks before the 10th tick speaks"
        );
    }
    app.update();
    assert_eq!(app.health, 15.0, "min(6, 6) / 6 healed the body");
    assert_eq!(
        app.food_data.exhaustion(),
        6.0,
        "the heal's own 6.0 exhaustion cost"
    );
    // the exhaustion over the fold's threshold: the fold drains the
    // saturation 1.0 at the next tick's head (the banked 6.0 > 4.0)
    app.update();
    assert_eq!(app.food_data.saturation(), 5.0, "the fold's own drain");
    assert_eq!(app.food_data.exhaustion(), 2.0, "the 4.0 banked");
    // the second beat (ticks 12..20): the SHARED timer continues —
    // the heal rides the DRAINED saturation (min(5, 6) / 6)
    for _ in 0..8 {
        app.update();
    }
    app.update();
    assert!(
        (app.health - (15.0 + 5.0 / 6.0)).abs() < 1e-4,
        "the second beat's 5/6 heal: {}",
        app.health
    );
    assert_eq!(
        app.food_data.exhaustion(),
        7.0,
        "the 2.0 bank + the 5.0 cost"
    );
}

/// THE FOOD TICK'S STARVE ARC: an EMPTY bar's 80-tick beat bites
/// 1.0 through the hurt pipeline (the cooldown chain, the flash,
/// the voice), the NORMAL difficulty stops at the last heart.
#[test]
fn the_food_tick_starves_the_empty_bar_on_its_own_beat() {
    let mut app = App::new(3);
    app.enter_world();
    // the starving setup: the empty bar, the empty saturation
    app.health = 20.0;
    app.food_data.set_food_level(0);
    app.food_data.set_saturation(0.0);
    // the exhaustion fold can't drop the bar further; the starve
    // lane's timer walks 80 ticks
    for _ in 0..79 {
        app.update();
        assert!(!app.dead);
    }
    app.update();
    assert_eq!(app.health, 19.0, "the starve bite's own 1.0");
    // the clocks armed THIS tick (the decrement rides the next
    // update's head — the food tick runs after the clock lane)
    assert_eq!(app.hurt_time, 10, "the flash armed");
    assert_eq!(app.invulnerable_time, 20, "the cooldown armed");
    let hurt_entry = app
        .sounds
        .events
        .iter()
        .filter(|e| e.contains("hurt"))
        .count();
    assert!(!sounds_loaded() || hurt_entry >= 1, "the hurt voice spoke");
    // the cooldown window: the next bite 80 ticks later lands only
    // when the window's own 20 folded — the beats' honest cadence
    for _ in 0..80 {
        app.update();
    }
    assert_eq!(app.health, 18.0, "the second bite");
}

/// THE MINE'S OWN COST: every break's reward lane banks the 0.005
/// the Block.playerDestroy tail speaks — the dig's completion folds
/// into the exhaustion bank.
#[test]
fn the_dig_banks_the_mine_exhaustion() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let banked_before = app.food_data.exhaustion();
    app.on_press(0, 0);
    let mut ticks = 0;
    while app.offline_world.as_ref().unwrap().get(c, surface, c) != crate::world::AIR {
        app.update();
        ticks += 1;
        assert!(ticks < 400, "the dig completed in time");
    }
    app.on_left_release();
    assert!(
        app.food_data.exhaustion() > banked_before,
        "the break banked the mine's own 0.005"
    );
}

/// THE GAMEMODE FOLDS' OWN AUDIT (26.2 GameType
/// `isBlockPlacingRestricted`'s acmp pair + GameType
/// `updatePlayerAbilities`'s putfield walk): the restricted pair
/// the dig lane's own head rides, the instabuild grant CREATIVE
/// alone wears — the spectator's mayfly the instabuild walk never
/// grants.
#[test]
fn the_gamemode_folds_the_audited_grants_speak() {
    assert!(!GameMode::Survival.is_block_placing_restricted());
    assert!(!GameMode::Creative.is_block_placing_restricted());
    assert!(GameMode::Adventure.is_block_placing_restricted());
    assert!(GameMode::Spectator.is_block_placing_restricted());
    assert!(GameMode::Creative.instabuild());
    assert!(
        !GameMode::Spectator.instabuild(),
        "the mayfly the spectator wears, the instabuild the acmp walk never grants"
    );
    assert!(!GameMode::Survival.instabuild());
    assert!(!GameMode::Adventure.instabuild());
}

/// THE INVULNERABLE GRANT (26.2 GameType.updatePlayerAbilities's
/// audited putfield walk, scripts/jar_work/mpgamemode_262.txt +
/// the invuln_262.txt / hurtserver_262.txt / basetick_262.txt
/// evidence this round banks): the four modes' own verdicts — the
/// creative and the spectator the flag banks, the survival and
/// the adventure the zero walk keeps.
#[test]
fn the_gamemode_invulnerable_folds_the_audited_grants_speak() {
    assert!(!GameMode::Survival.invulnerable());
    assert!(GameMode::Creative.invulnerable());
    assert!(!GameMode::Adventure.invulnerable());
    assert!(GameMode::Spectator.invulnerable());
}

/// THE IS-SURVIVAL FOLD (javap 26.2 GameType.isSurvival's acmp
/// pair, scripts/jar_work/gametype_262.txt — SURVIVAL | ADVENTURE
/// the true the pair speaks, creative/spectator the false): the
/// verdict the HUD decorations' own gates wear twice —
/// MultiPlayerGameMode.canHurtPlayer pcs 0-7 (the
/// extractHotbarAndDecorations pcs 33-50 ifeq the whole
/// extractPlayerHealth body skips) and
/// MultiPlayerGameMode.hasExperience pcs 0-7 (the XP contextual
/// bar the same verdict selects, scripts/jar_work/hudgates_262.txt)
/// — the audited equality the two callers' identical bytecode
/// keeps.
#[test]
fn the_gamemode_survival_folds_the_audited_arms_speak() {
    assert!(GameMode::Survival.is_survival());
    assert!(GameMode::Adventure.is_survival());
    assert!(!GameMode::Creative.is_survival());
    assert!(!GameMode::Spectator.is_survival());
}

/// THE INVULNERABLE HEAD (javap LivingEntity.hurtServer pcs 0-10,
/// scripts/jar_work/hurtserver_262.txt — the isInvulnerableTo
/// verdict the isInvulnerableToBase walk answers, scripts/
/// jar_work/invuln_262.txt): the pipeline's opening refusal the
/// flag alone wears — every source the client speaks (the plain
/// hurt the mob blow and the starve ride, the burning family the
/// lava and the onFire cadence speak, the drowning source) the
/// head refuses BEFORE the cooldown, the flash, the health, the
/// voice the hurt never lands to earn. The survival control the
/// same plain hurt the landing law speaks (the bite, the
/// re-armed window).
#[test]
fn the_invulnerable_head_the_pipeline_refuses() {
    let mut app = App::new(3);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    app.food_data.set_saturation(0.0);
    let before = app.health;
    // the plain hurt (the mob blow, the arrow, the starve's own lane)
    assert!(!app.hurt(5.0), "the head refuses the plain source");
    // the burning family (the lava dose, the onFire cadence)
    assert!(
        !app.hurt_burning(crate::body::LAVA_DAMAGE),
        "the head refuses the burning family"
    );
    // the drowning source
    assert!(
        !app.hurt_drowning(crate::body::DROWN_DAMAGE),
        "the head refuses the drown"
    );
    // the health the refusals never spend, the arms the landing never earns
    assert_eq!(app.health, before, "the bite the refusal never spends");
    assert_eq!(app.invulnerable_time, 0, "the window never armed");
    assert_eq!(app.hurt_time, 0, "the flash never armed");
    assert_eq!(app.last_hurt, 0.0, "the comparison bank never armed");
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.hurt")),
        "no hurt voice the refused pipeline speaks: {:?}",
        app.sounds.events
    );
    // THE SURVIVAL CONTROL: the same plain hurt the landing law wears
    app.game_mode = GameMode::Survival;
    assert!(app.hurt(5.0), "the survival landing speaks");
    assert_eq!(app.health, before - 5.0, "the bite the landing spends");
    assert_eq!(app.invulnerable_time, 20, "the window re-armed");
}

/// THE INVULNERABLE SOURCES' OWN LANES (the real App update loop):
/// the creative body stands IN the lava — the dose the head
/// refuses (the health the bites never spend, the cooldown never
/// arms, the GENERIC_BURN the refused hurt never earns), the fire
/// visual the fluid walk keeps (the vanilla ignite rides the fluid
/// entry, NOT the refused lavaHurt — javap lavaHurt's own walk
/// carries no ignite, scripts/jar_work/lavahurt_262.txt); the
/// starving creative the empty bar's bite the head refuses.
#[test]
fn the_invulnerable_lanes_the_doses_never_speak() {
    let mut app = App::new(3);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    let size = app.world_size;
    let c = size / 2;
    let surface = app.spawn_y;
    // the lava pool AT the feet cell (the fluid walk the touch speaks)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, surface as i32, c, crate::world::LAVA);
    }
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        surface as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    {
        let b = app.body.as_mut().unwrap();
        b.vy = 0.0;
        b.on_ground = true;
    }
    let before = app.health;
    for _ in 0..40 {
        app.update();
    }
    assert_eq!(app.health, before, "the lava dose the head refuses");
    assert!(
        app.body.as_ref().unwrap().remaining_fire_ticks > 0,
        "the fire visual rides — the ignite the fluid walk keeps"
    );
    assert_eq!(app.invulnerable_time, 0, "the cooldown never armed");
    assert!(
        !app.sounds.events.iter().any(|e| e.contains("burn")),
        "the GENERIC_BURN the refused lava hurt never earns: {:?}",
        app.sounds.events
    );
    // THE STARVE LANE: the empty bar's 80-tick beat the head refuses
    app.food_data.set_food_level(0);
    app.food_data.set_saturation(0.0);
    for _ in 0..90 {
        app.update();
    }
    assert_eq!(
        app.health, before,
        "the starve bite the invulnerable head refuses"
    );
}

/// THE INVULNERABLE AIR CLOCK (javap LivingEntity.baseTick pcs
/// 258-280, scripts/jar_work/basetick_262.txt — the player's
/// decrease arm `(!bl2 || !abilities.invulnerable)` the flag alone
/// answers): the submerged creative body the drain never speaks —
/// the pre-drained clock the gated arm REFILLS four a tick
/// underwater (the decrease gate false the vanilla else-if rides),
/// the dose flag the drain never earns; the survival control the
/// same column the one-a-tick drain wears.
#[test]
fn the_invulnerable_air_clock_the_underwater_never_drains() {
    let mut app = App::new(3);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    let size = app.world_size;
    let c = size / 2;
    let surface = app.spawn_y;
    // the 3-deep water column AT the body's own cells (the drown
    // test's own shape — the eye one cell under the surface)
    {
        let w = app.offline_world.as_mut().unwrap();
        for y in surface..surface + 3 {
            w.set(c, y as i32, c, crate::world::WATER);
        }
    }
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        surface as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    {
        let b = app.body.as_mut().unwrap();
        b.vy = 0.0;
        b.on_ground = true;
        b.air_supply = crate::body::TOTAL_AIR_SUPPLY - 10;
    }
    for _ in 0..5 {
        app.update();
    }
    {
        let b = app.body.as_ref().unwrap();
        assert!(
            b.eye_in_water,
            "the eye the column keeps (the gate's own query)"
        );
        assert_eq!(
            b.air_supply,
            crate::body::TOTAL_AIR_SUPPLY,
            "the underwater refill the gated arm speaks — the drain the flag never wears"
        );
        assert!(!b.drown_hurt_due, "the dose flag the drain never earns");
    }
    assert_eq!(app.health, 20.0, "the dose the pipeline never sees");
    // THE SURVIVAL CONTROL: the same column the drain wears
    app.game_mode = GameMode::Survival;
    {
        let b = app.body.as_mut().unwrap();
        b.air_supply = crate::body::TOTAL_AIR_SUPPLY;
    }
    for _ in 0..30 {
        app.update();
    }
    let b = app.body.as_ref().unwrap();
    assert_eq!(
        b.air_supply,
        crate::body::TOTAL_AIR_SUPPLY - 30,
        "the one-a-tick drain the grant's absence wears"
    );
    assert!(!b.drown_hurt_due, "the clock above the -20 fold");
}

/// THE INVULNERABLE FALL (the wrapper arm the pipeline head keeps):
/// the creative ten-block landing the fall voice the refusal never
/// earns, the health the bite never spends, the window never arms;
/// the survival control the same descent the landing law speaks
/// (the fall voice, the bite, the re-armed window).
#[test]
fn the_invulnerable_fall_the_landing_never_spends() {
    let mut app = App::new(3);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let spawn_feet = app.spawn_y;
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        (spawn_feet + 10) as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    let before = app.health;
    let mut landed = false;
    for _ in 0..60 {
        app.update();
        if app.body.as_ref().unwrap().on_ground {
            landed = true;
            break;
        }
    }
    assert!(landed, "the body landed inside the ticks");
    assert_eq!(app.health, before, "the bite the refusal never spends");
    assert_eq!(app.invulnerable_time, 0, "the window never armed");
    assert!(
        !app.sounds.events.iter().any(|e| e.contains("_fall @")),
        "the fall voice the refused landing never earns: {:?}",
        app.sounds.events
    );
    // THE SURVIVAL CONTROL: the same descent the landing law wears
    app.game_mode = GameMode::Survival;
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        (spawn_feet + 10) as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    let mut landed = false;
    for _ in 0..60 {
        app.update();
        if app.body.as_ref().unwrap().on_ground {
            landed = true;
            break;
        }
    }
    assert!(landed, "the control landed inside the ticks");
    assert!(app.health < before, "the bite the survival landing spends");
    assert_eq!(app.invulnerable_time, 20, "the window re-armed");
    if sounds_loaded() {
        assert!(
            app.sounds.events.iter().any(|e| e.contains("_fall @")),
            "the fall voice the survival landing earns"
        );
    }
}

/// THE F4 FLICK THE GRANT FEEDS INSTANTLY (the per-frame feed the
/// noPhysics head's own shape wears): the survival body the lava
/// doses land on, the flick to creative the NEXT beat refuses (the
/// health the bites stop spending), the flick back the survival
/// doses resume — the grant the mode bank's own law wears.
#[test]
fn the_f4_flick_the_grant_feeds_instantly() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.world_size;
    let c = size / 2;
    let surface = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, surface as i32, c, crate::world::LAVA);
    }
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        surface as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    {
        let b = app.body.as_mut().unwrap();
        b.vy = 0.0;
        b.on_ground = true;
    }
    // the survival doses land
    for _ in 0..30 {
        app.update();
    }
    let drained = app.health;
    assert!(drained < 20.0, "the survival doses landed: {drained}");
    // THE FLICK: the grant feeds, the doses refuse — the regen the
    // emptied bar keeps silent (the full bar's 10-tick beat the
    // health climbs), the starve bite the 80-tick walk never
    // reaches inside the 30 (and the invulnerable head would
    // refuse it anyway)
    app.food_data.set_food_level(0);
    app.food_data.set_saturation(0.0);
    app.game_mode = GameMode::Creative;
    let mid = app.health;
    for _ in 0..30 {
        app.update();
    }
    assert_eq!(app.health, mid, "the doses the grant refuses");
    // THE FLICK BACK: the survival doses resume
    app.game_mode = GameMode::Survival;
    for _ in 0..30 {
        app.update();
    }
    assert!(app.health < mid, "the survival doses resumed");
}

/// THE CREATIVE DIG (26.2 MultiPlayerGameMode's audited instabuild
/// lanes, scripts/jar_work/mpgamemode_262.txt): the PRESS breaks
/// the aimed cell the SAME tick (the lambda$0 prediction, the
/// destroyDelay arm, isDestroying the lane never arms), the HOLD
/// carves at the 6-tick cadence (the delay head the five decrement
/// ticks speak), the REWARD the instabuild arm refuses (no drops,
/// no mine exhaustion — the ServerPlayerGameMode.destroyBlock's own
/// split), and the 2001 voice the press's own break speaks.
#[test]
fn the_creative_press_the_offline_world_carves() {
    let mut app = App::new(3);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let kind_before = app.offline_world.as_ref().unwrap().get(c, surface, c);
    assert_ne!(kind_before, crate::world::AIR, "the surface cell is solid");
    let below_before = app.offline_world.as_ref().unwrap().get(c, surface - 1, c);
    assert_ne!(
        below_before,
        crate::world::AIR,
        "the column's next cell is solid"
    );
    // THE PRESS: the cell vanishes the SAME tick — the crack lane
    // never arms, the delay arms, the swing rides
    app.on_press(0, 0);
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface, c),
        crate::world::AIR,
        "the instant break"
    );
    assert!(!app.dig.is_destroying, "the crack lane never arms");
    assert_eq!(app.dig.delay, 5, "the audited destroyDelay arm");
    assert!(app.swing.swinging, "the press swung the arm");
    assert!(app.drops.is_empty(), "the instabuild arm refuses the drops");
    // THE 2001 VOICE (the press's own destroyBlock — the helper the
    // two dig lanes share)
    if sounds_loaded() {
        use mc_registry::generated::block_sounds as bs;
        let name = crate::world::NAME_OF[kind_before as usize];
        let idx = bs::sound_index(name).expect("the surface block in the sound table");
        let entry = app
            .sounds
            .events
            .iter()
            .find(|e| e.starts_with(&format!("{} @", bs::SOUND_BREAK[idx])))
            .expect("the press's break voice");
        assert!(
            entry.contains(" v1.000000 p0.800000"),
            "the break fold (v+1)/2, p*0.8: {entry}"
        );
    }
    // THE CARVE: the eye drops into the hole (the column's next
    // cell), five decrement ticks keep it standing, the sixth
    // breaks it (the audited 6-tick cadence the continue's own
    // delay head speaks)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 2.5, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    for t in 0..5 {
        app.update();
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface - 1, c),
            below_before,
            "the decrement tick {t} never breaks"
        );
    }
    app.update();
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface - 1, c),
        crate::world::AIR,
        "the sixth tick's own break"
    );
    assert_eq!(app.dig.delay, 5, "the delay re-armed");
    assert!(!app.dig.is_destroying, "the hold lane the state never arms");
    assert!(
        app.drops.is_empty(),
        "the carve's reward the mode never pays"
    );
}

/// THE SPECTATOR PRESS (26.2 `Player.blockActionRestricted`'s own
/// acmp lane): the press speaks NOTHING for the dig — no break, no
/// arm, no send — the swing's own tail rides, and the hold never
/// dusts (the TRUE beat the refused lanes never speak).
#[test]
fn the_spectator_press_the_world_keeps_the_block() {
    let mut app = App::new(3);
    app.enter_world();
    app.game_mode = GameMode::Spectator;
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let kind_before = app.offline_world.as_ref().unwrap().get(c, surface, c);
    assert_ne!(kind_before, crate::world::AIR, "the surface cell is solid");
    let particles_before = app.particles.len();
    // THE PRESS: the restricted head the dig never arms — the
    // swing's own tail rides
    app.on_press(0, 0);
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface, c),
        kind_before,
        "the block stands"
    );
    assert!(!app.dig.is_destroying, "the dig never arms");
    assert_eq!(app.dig.delay, 0, "the delay never arms");
    assert!(app.swing.swinging, "the swing's tail rides the refusal");
    assert!(app.dig_held, "the button state rides the edge");
    // THE HOLD: the refused lanes never beat, never dust, never pay
    app.update();
    app.update();
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, surface, c),
        kind_before,
        "the hold never breaks"
    );
    assert_eq!(
        app.particles.len(),
        particles_before,
        "the refused hold never dusts"
    );
    assert!(app.drops.is_empty(), "the reward the refusal never reaches");
    // THE RELEASE: the isDestroying check keeps silent
    app.on_left_release();
    assert!(!app.dig.is_destroying);
}

/// THE SPECTATOR WALK (26.2 Entity.noPhysics's own law — the
/// Player.tick head writes the flag fresh every frame
/// `noPhysics = isSpectator()`, the Entity.move branch the raw
/// walk wears): the spectator body phases through the stone wall
/// the collide body rests against — the noPhysics feed the
/// spectator bank wears, the survival walk the collide walk
/// keeps.
#[test]
fn the_spectator_walk_the_terrain_never_stops() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Spectator;
    // THE WALL: a stone column on the spawn's east walk line (the
    // vanilla yaw 270 = +X the body's impulse fold wears)
    let (sx, sy, sz) = {
        let b = app.body.as_ref().unwrap();
        (b.x, b.y, b.z)
    };
    app.body.as_mut().unwrap().yaw = 270.0;
    let wall_x = sx.floor() as i32 + 3;
    let wall_z = sz.floor() as i32;
    let feet = sy.floor() as i32;
    for dy in 0..3 {
        app.offline_world
            .as_mut()
            .unwrap()
            .set(wall_x, feet + dy, wall_z, crate::world::STONE);
    }
    assert_ne!(
        app.offline_world
            .as_ref()
            .unwrap()
            .get(wall_x, feet, wall_z),
        crate::world::AIR,
        "the wall stands"
    );
    // THE SPECTATOR WALK: the noPhysics feed rides the tick, the
    // raw walk phases through the wall
    app.on_movement_input(1.0, 0.0, false, false, false);
    for _ in 0..120 {
        app.update();
    }
    let b = app.body.as_ref().unwrap();
    assert!(b.no_physics, "the feed the spectator bank wears");
    assert!(
        b.x > wall_x as f64 + 1.0,
        "the spectator phased through at {:.3}",
        b.x
    );
    assert!(!b.horizontal_collision, "the flag the branch zeroes");
    // THE SURVIVAL CONTROL: the same walk the wall stops — the
    // collide walk the survival body keeps
    let mut app2 = App::new(3);
    app2.enter_world();
    app2.spawn_offline_body();
    app2.body.as_mut().unwrap().yaw = 270.0;
    let (s2x, s2y, s2z) = {
        let b = app2.body.as_ref().unwrap();
        (b.x, b.y, b.z)
    };
    let wall2_x = s2x.floor() as i32 + 3;
    for dy in 0..3 {
        app2.offline_world.as_mut().unwrap().set(
            wall2_x,
            s2y.floor() as i32 + dy,
            s2z.floor() as i32,
            crate::world::STONE,
        );
    }
    app2.on_movement_input(1.0, 0.0, false, false, false);
    for _ in 0..120 {
        app2.update();
    }
    let b2 = app2.body.as_ref().unwrap();
    assert!(!b2.no_physics, "the survival feed the flag never wears");
    assert!(
        b2.x < wall2_x as f64,
        "the collide body never crossed at {:.3}",
        b2.x
    );
}

/// THE DOUBLE-TAP W'S OWN WINDOW (26.2 LocalPlayer.aiStep's
/// audited sprintTriggerTime law, the law's own state machine the
/// [`App::sprint_law_tick`] wears): the FIRST W press ARMS the
/// `options.sprintWindow()` default 7 (the Options <init>
/// IntRange(0, 10) keeps), the SECOND press INSIDE the window
/// STARTS the sprint (the sticky state the Ctrl release keeps),
/// the single tap NEVER sprints (the window spends and expires —
/// the press, the release, the silence), the Shift and the S key
/// KILL the armed window, and the expired re-press re-arms.
#[test]
fn the_double_tap_w_sprint_window_speaks() {
    // THE LAW'S OWN HARNESS (the standalone state machine — the
    // body the fresh constructor stages, the food the fed bar
    // keeps, the raw keys the parameters carry)
    let law = |b: &mut crate::body::PlayerBody,
               trigger: &mut i32,
               last: &mut bool,
               sprint_key: bool,
               using: bool| {
        App::sprint_law_tick(b, trigger, last, sprint_key, true, using, 7);
    };
    let mut b = crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0);
    let mut trigger = 0;
    let mut last = false;
    // THE FIRST PRESS (the W down tick): the window ARMS — the
    // sprint does NOT start (the vanilla's own single-press
    // silence)
    b.forward = 1.0;
    law(&mut b, &mut trigger, &mut last, false, false);
    assert_eq!(trigger, SPRINT_WINDOW_TICKS, "the window arms");
    assert!(!b.sprinting, "the first press walks");
    // THE HOLD (the window spends one per tick — the held key the
    // edge never re-speaks)
    law(&mut b, &mut trigger, &mut last, false, false);
    assert_eq!(trigger, SPRINT_WINDOW_TICKS - 1, "the window spends");
    assert!(!b.sprinting);
    // THE SECOND PRESS INSIDE THE WINDOW (release first — the
    // edge the fresh down tick re-speaks): the SPRINT STARTS
    b.forward = 0.0;
    law(&mut b, &mut trigger, &mut last, false, false);
    b.forward = 1.0;
    law(&mut b, &mut trigger, &mut last, false, false);
    assert!(b.sprinting, "the double tap sprints");
    // THE STICKY STATE (the Ctrl released, the W held): the sprint
    // CONTINUES — no lane holds it down, the stop lane's forward
    // impulse keeps it alive
    for _ in 0..5 {
        law(&mut b, &mut trigger, &mut last, false, false);
        assert!(b.sprinting, "the sticky sprint rides");
    }
    // THE STOP (the W released): the forward impulse dies, the
    // sprint retires
    b.forward = 0.0;
    law(&mut b, &mut trigger, &mut last, false, false);
    assert!(!b.sprinting, "the release retires");
    // THE SINGLE TAP NEVER SPRINTS (press, release, the window
    // spends past zero — the re-press EXPIRED re-arms, the sprint
    // stays silent)
    b.forward = 1.0;
    law(&mut b, &mut trigger, &mut last, false, false);
    assert!(!b.sprinting, "the fresh press arms anew");
    b.forward = 0.0;
    for _ in 0..SPRINT_WINDOW_TICKS {
        law(&mut b, &mut trigger, &mut last, false, false);
    }
    assert_eq!(trigger, 0, "the window expires");
    b.forward = 1.0;
    law(&mut b, &mut trigger, &mut last, false, false);
    assert!(!b.sprinting, "the expired press walks");
    assert_eq!(trigger, SPRINT_WINDOW_TICKS, "the expired re-arms");
    // THE KILL LANES (the armed window the sneak and the S key
    // each zero — the re-press inside the window the sprint wears,
    // the kill the state and the window both retire)
    b.forward = 0.0;
    law(&mut b, &mut trigger, &mut last, false, false); // release
    b.forward = 1.0;
    law(&mut b, &mut trigger, &mut last, false, false); // re-press: sprint
    b.forward = 0.0;
    b.want_crouch = true; // THE SHIFT KILL
    law(&mut b, &mut trigger, &mut last, false, false);
    assert_eq!(trigger, 0, "the sneak kills the window");
    assert!(!b.sprinting, "the sneak retires the state too");
    b.want_crouch = false;
    b.forward = 1.0;
    law(&mut b, &mut trigger, &mut last, false, false); // fresh press arms: 7
    b.forward = -1.0; // THE BACKWARD KILL (the S key)
    law(&mut b, &mut trigger, &mut last, false, false);
    assert_eq!(trigger, 0, "the backward kills the window");
    // THE CTRL LANE (the raw key the sprint speaks — the sticky
    // start the release keeps, the same law the window shares)
    b.forward = 1.0;
    law(&mut b, &mut trigger, &mut last, true, false);
    assert!(b.sprinting, "the Ctrl lane sprints");
    law(&mut b, &mut trigger, &mut last, false, false);
    assert!(b.sprinting, "the Ctrl release keeps the sprint");
    // THE COLLISION STOP (shouldStopRunSprinting's own arm)
    b.horizontal_collision = true;
    law(&mut b, &mut trigger, &mut last, false, false);
    assert!(!b.sprinting, "the wall retires the sprint");
    b.horizontal_collision = false;
    // THE ITEM-USE SLOWDOWN (the slow && !passenger kill + the
    // canStart gate the using lane wears)
    law(&mut b, &mut trigger, &mut last, true, true);
    assert!(!b.sprinting, "the using body walks");
}

/// THE DOUBLE-TAP SPACE'S OWN WINDOW (26.2 LocalPlayer.aiStep's
/// audited jumpTriggerTime law, the law's own state machine the
/// [`App::fly_law_tick`] wears): the FIRST space press ARMS the 7
/// window (pc 567's bipush), the SECOND press INSIDE it TOGGLES
/// the flight (the toggle-on the ground hop rides — pc 628's own
/// jumpFromGround) and SPENDS the window to zero (pc 640), the
/// spent window the fresh press re-arms, the survival body the
/// mayfly gate never arms, the spectator's flight the sync forces
/// without any press, and the riding gate the toggle refuses.
#[test]
fn the_double_tap_space_flight_window_speaks() {
    // THE LAW'S OWN HARNESS (the call site's own shape — the law
    // first, the Player.aiStep head decrement the super chain
    // speaks after: pcs 0-14 the same tick's tail spends)
    let law = |b: &mut crate::body::PlayerBody,
               trigger: &mut i32,
               last: &mut bool,
               jump: bool,
               mayfly: bool,
               spectator: bool,
               riding: bool| {
        App::fly_law_tick(b, trigger, last, jump, mayfly, spectator, riding);
        if *trigger > 0 {
            *trigger -= 1;
        }
    };
    let mut b = crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0);
    b.on_ground = true;
    let mut trigger = 0;
    let mut last = false;
    // THE FIRST PRESS (the space down tick): the window ARMS — the
    // flight does NOT start (the vanilla's own single-press
    // silence)
    law(&mut b, &mut trigger, &mut last, true, true, false, false);
    assert_eq!(trigger, FLY_WINDOW_TICKS - 1, "the window arms");
    assert!(!b.flying, "the first press walks");
    // THE HOLD (the held key the edge never re-speaks, the window
    // the decrement spends)
    law(&mut b, &mut trigger, &mut last, true, true, false, false);
    assert_eq!(trigger, FLY_WINDOW_TICKS - 2, "the window spends");
    assert!(!b.flying);
    // THE SECOND PRESS INSIDE THE WINDOW (release first — the
    // fresh down tick the edge re-speaks): the FLIGHT TOGGLES ON,
    // the launch hop rides (the on-ground body), the window
    // SPENDS
    law(&mut b, &mut trigger, &mut last, false, true, false, false);
    law(&mut b, &mut trigger, &mut last, true, true, false, false);
    assert!(b.flying, "the double tap flies");
    assert_eq!(trigger, 0, "the toggle spends the window");
    assert!(
        (b.vy - crate::body::JUMP_POWER as f64).abs() < 1e-9,
        "the launch hop rides, vy={}",
        b.vy
    );
    assert!(
        (b.exhaustion_event - crate::food::EXHAUSTION_JUMP).abs() < 1e-6,
        "the hop's own cost the plain jump banks"
    );
    // THE STICKY FLIGHT (the keys release, the flight the restore
    // law keeps — the toggle lane's edge the silence never re-arms)
    for _ in 0..3 {
        law(&mut b, &mut trigger, &mut last, false, true, false, false);
        assert!(b.flying, "the sticky flight rides");
    }
    // THE THIRD IMMEDIATE PRESS: the spent window RE-ARMS (no
    // toggle the fresh arm speaks)
    law(&mut b, &mut trigger, &mut last, true, true, false, false);
    assert_eq!(trigger, FLY_WINDOW_TICKS - 1, "the spent window re-arms");
    assert!(b.flying, "the flight keeps");
    // THE TOGGLE OFF (the window armed, the next press inside it
    // speaks the OFF): no hop the retirement never earns
    let (vy_before, exhaust_before) = (b.vy, b.exhaustion_event);
    law(&mut b, &mut trigger, &mut last, false, true, false, false);
    law(&mut b, &mut trigger, &mut last, true, true, false, false);
    assert!(!b.flying, "the second double tap retires");
    assert!((b.vy - vy_before).abs() < 1e-9, "the toggle-off never hops");
    assert!(
        (b.exhaustion_event - exhaust_before).abs() < 1e-6,
        "the toggle-off never banks the jump cost"
    );
    // THE SURVIVAL REFUSAL (the mayfly gate — the vanilla server's
    // own abilities grant): the double tap never arms, never flies
    let mut sb = crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0);
    sb.on_ground = true;
    let mut st = 0;
    let mut sl = false;
    law(&mut sb, &mut st, &mut sl, true, false, false, false);
    assert_eq!(st, 0, "the survival never arms");
    law(&mut sb, &mut st, &mut sl, false, false, false, false);
    law(&mut sb, &mut st, &mut sl, true, false, false, false);
    assert!(!sb.flying, "the survival never flies");
    // THE SPECTATOR SYNC (pcs 508-532): the flight the mode forces
    // — no press the sync asks
    let mut pb = crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0);
    let mut pt = 0;
    let mut pl = false;
    law(&mut pb, &mut pt, &mut pl, false, true, true, false);
    assert!(pb.flying, "the spectator's flight the sync forces");
    // THE RIDING REFUSAL (the unjumpable vehicle's own gate — the
    // armed window the toggle never speaks while the ride carries)
    let mut rb = crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0);
    rb.on_ground = true;
    let mut rt = 0;
    let mut rl = false;
    law(&mut rb, &mut rt, &mut rl, true, true, false, false);
    law(&mut rb, &mut rt, &mut rl, false, true, false, false);
    law(&mut rb, &mut rt, &mut rl, true, true, false, true);
    assert_eq!(
        rt,
        FLY_WINDOW_TICKS - 3,
        "the window the decrements keep spending"
    );
    assert!(!rb.flying, "the ride the toggle refuses");
}

/// THE SPRINT'S FOOD GATE (the hasEnoughFoodToDoExhaustiveManoeuvres
/// law): the bar over 6 sprints, the starving bar walks.
#[test]
fn the_sprint_gate_refuses_the_starving_body() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE LAW'S OWN TICK (the sprinting field the per-tick state
    // machine writes — the on_movement_input feed only banks the
    // raw keys, the update() walk the law's verdict carries)
    app.on_movement_input(1.0, 0.0, false, false, true);
    app.update();
    assert!(app.body.as_ref().unwrap().sprinting, "the fed body sprints");
    // the bar drops under the gate (food 6 — `> 6` refuses): the
    // STOP lane the impossible sprint retires the same tick
    app.food_data.set_food_level(6);
    app.on_movement_input(1.0, 0.0, false, false, true);
    app.update();
    assert!(
        !app.body.as_ref().unwrap().sprinting,
        "the starving body walks"
    );
}

/// THE FOOD ROW'S SHIMMY GATE (javap Hud.extractFood): the
/// saturation at zero shakes on the `tickCount % (food * 3 + 1)`
/// beat — the saturation fed keeps the row still. The sprites'
/// own half/full law rides the row's walk.
#[test]
fn the_food_row_gate_speaks_the_shimmy_beat() {
    let mut app = App::new(3);
    app.enter_world();
    // the beat's own arithmetic: the full bar shakes at the 61st
    // (and every 61st), the 7-bar at the 22nd, the empty bar at
    // every tick (the modulus 1)
    for (food, frame, fires) in [
        (20u32, 61u64, true),
        (20, 100, false),
        (7, 22, true),
        (0, 1, true),
    ] {
        let beat = food as u64 * 3 + 1;
        assert_eq!(frame % beat == 0, fires);
    }
    // the default vitals (saturation 5.0) keep the goldens' row
    // still — the shake's own gate never fires
    assert!(
        app.food_data.saturation() > 0.0,
        "the spawn saturation shakes nothing"
    );
    // the drained saturation arms the gate
    app.food_data.set_saturation(0.0);
    app.food_data.set_food_level(20);
    app.frame = 61;
    let fires = app.frame % (app.food_data.food_level() as u64 * 3 + 1) == 0;
    assert!(fires, "the 61st tick shakes the full bar's row");
}

/// THE RESPAWN'S FOOD RESET: the fresh FoodData the constructor
/// speaks — the full bar, the 5.0 saturation, the zero bank, the
/// zero timer.
#[test]
fn the_respawn_resets_the_food_lane() {
    let mut app = App::new(3);
    app.enter_world();
    app.food_data.set_food_level(4);
    app.food_data.set_saturation(0.0);
    app.food_data.add_exhaustion(10.0);
    app.respawn();
    assert_eq!(app.food_data.food_level(), 20);
    assert_eq!(app.food_data.saturation(), 5.0);
    assert_eq!(app.food_data.exhaustion(), 0.0);
    assert!(app.food_data.has_enough_food(), "the fresh bar sprints");
}

/// THE EAT ARC — the full 32-tick hold over an apple: the six chew
/// bursts (5 pieces + a voice each), the completion's 16-piece
/// finisher, the gulp, the FoodData.eat fold (the ABSOLUTE 2.4
/// saturation), THE BURP, and the survival's consume. The item
/// pieces spawned and ticked (the BreakingItemParticle lane).
#[test]
fn the_eat_arc_feeds_the_hungry_body() {
    let mut app = App::new(3);
    app.enter_world();
    let apple_id = mc_registry::item_id_by_name("apple").unwrap();
    app.inventory.items[app.selected_hotbar] = (apple_id, 3);
    app.hotbar_slots = app.inventory.hotbar();
    // the hungry body: the bar under 20 (the canEat's own gate)
    app.food_data.set_food_level(10);
    app.food_data.set_saturation(0.0);
    let pieces_before = app.pieces.len();
    // the press opens the hold
    assert!(app.on_right_press(0, 0), "the hungry body starts the eat");
    let eat = app.eating.as_ref().expect("the hold opened");
    assert_eq!(eat.total, 32, "the apple's 1.6s");
    assert_eq!(eat.item, apple_id);
    // walk the 32 ticks: the bursts at the cadence's own stride,
    // the completion at the 32nd
    for _ in 0..32 {
        app.update();
    }
    assert!(app.eating.is_none(), "the hold completed");
    // THE FOOD DATA ATE: the bar 10 + 4, the saturation 0 + 2.4
    // (the 26.2 record's ABSOLUTE field)
    assert_eq!(app.food_data.food_level(), 14);
    assert!((app.food_data.saturation() - 2.4).abs() < 1e-4);
    // THE SURVIVAL CONSUME: the stack 3 -> 2
    assert_eq!(app.inventory.items[app.selected_hotbar], (apple_id, 2));
    // THE VOICES: six chews + the finisher + the gulp (all
    // entity.generic.eat) + the burp (the assert computes the expected
    // from the ACTUAL loaded state — the cache-absent boot the silent
    // lane keeps)
    if sounds_loaded() {
        let eat_events = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.generic.eat "))
            .count();
        assert_eq!(
            eat_events,
            8,
            "6 chews + the finisher + the gulp: {}",
            app.sounds
                .events
                .iter()
                .filter(|e| e.contains("eat"))
                .count()
        );
        let burp_events = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.player.burp "))
            .count();
        assert_eq!(burp_events, 1, "the burp's own voice");
    }
    // THE PIECES SPAWNED: 6 chews x 5 + the finisher's 16 = 46 over
    // the arc; some already expired (the lifetime's own law) — the
    // spawner's own floor: the earliest pieces at least RAN
    assert!(
        app.pieces.len() + pieces_before <= 46,
        "the spawner spoke its 46"
    );
}

/// THE FULL BAR'S REFUSAL: the canEat gate — the plain apple on a
/// full bar refuses (the press returns false, the lane stays
/// shut); the golden apple's canAlwaysEat speaks anyway.
#[test]
fn the_can_eat_gate_refuses_the_full_bar() {
    let mut app = App::new(3);
    app.enter_world();
    let apple_id = mc_registry::item_id_by_name("apple").unwrap();
    let gapple_id = mc_registry::item_id_by_name("golden_apple").unwrap();
    // the FULL bar: the plain apple refused
    app.inventory.items[app.selected_hotbar] = (apple_id, 1);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(!app.on_right_press(0, 0), "the full bar refuses the apple");
    assert!(app.eating.is_none(), "the lane stayed shut");
    // the golden apple's canAlwaysEat: the full bar eats it
    app.inventory.items[app.selected_hotbar] = (gapple_id, 1);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(app.on_right_press(0, 0), "the golden apple always");
    let eat = app.eating.as_ref().expect("the hold opened");
    assert_eq!(eat.total, 32);
    app.on_right_release();
    assert!(app.eating.is_none(), "the release cancelled");
}

/// THE GOLDEN APPLE'S STATUS EFFECTS LAND (javap
/// `ApplyStatusEffectsConsumeEffect.apply` through the eat arc's
/// payload lane): the completion applies the Regeneration II
/// (100t) + the Absorption (2400t) — the rows verbatim — and the
/// absorption's `onEffectStarted` lifts the value to 8.0.
#[test]
fn the_golden_apple_lands_its_effects() {
    let mut app = App::new(3);
    app.enter_world();
    let gapple_id = mc_registry::item_id_by_name("golden_apple").unwrap();
    app.inventory.items[app.selected_hotbar] = (gapple_id, 1);
    app.hotbar_slots = app.inventory.hotbar();
    app.food_data.set_food_level(20); // the canAlwaysEat lane
    assert!(app.on_right_press(0, 0), "the golden apple always eats");
    for _ in 0..32 {
        app.update();
    }
    assert!(app.eating.is_none(), "the hold completed");
    // the store speaks both effects at the payload's own numbers —
    // the durations already walked their FIRST tick (the eat's
    // completion lands mid-update, the body tick's tick_offline
    // runs the same update: the beat fired at duration 100, the
    // tick-down followed)
    let regen = app
        .effects
        .get(crate::effects::MOB_EFFECT_REGENERATION)
        .expect("the regen rides");
    assert_eq!(regen.amplifier, 1, "Regeneration II");
    assert_eq!(regen.duration, 99);
    let absorption = app
        .effects
        .get(crate::effects::MOB_EFFECT_ABSORPTION)
        .expect("the absorption rides");
    assert_eq!(absorption.amplifier, 0);
    assert_eq!(absorption.duration, 2399);
    // the onEffectStarted's lift: 4 * (1 + 0) = 4.0 — two white
    // hearts the vanilla golden apple speaks
    assert!((app.effects.absorption() - 4.0).abs() < 1e-5);
}

/// THE ABSORPTION'S TWO DRAINS (javap `actuallyHurt`'s 26.2
/// verdict): a hit inside the absorption never reaches the health
/// (the fully-absorbed return); a hit beyond it takes the
/// remainder off the health AND drains the absorption AGAIN — the
/// second `setAbsorptionAmount` the bytecode speaks.
#[test]
fn the_absorption_drains_and_the_health_lives() {
    let mut app = App::new(3);
    app.enter_world();
    app.effects.add_effect(
        crate::effects::MOB_EFFECT_ABSORPTION,
        1,
        2400,
        crate::effects::DEFAULT_EFFECT_FLAGS,
    );
    assert!((app.effects.absorption() - 8.0).abs() < 1e-5);
    // the small hit: fully absorbed, the health untouched
    app.hurt(4.0);
    assert_eq!(app.health, MAX_HEALTH, "the white hearts took it");
    assert!((app.effects.absorption() - 4.0).abs() < 1e-5);
    // the cooldown's own law rides (the 10-tick window would only
    // land the difference — the test resets the pipeline for the
    // clean second hit)
    app.invulnerable_time = 0;
    app.last_hurt = 0.0;
    // the big hit: 10 total against 4 absorption — the health share
    // is 6, the drain one takes the 4, the drain two takes the 6
    // again (the absorption floors at the clamp's own zero)
    app.hurt(10.0);
    assert!(
        (app.health - (MAX_HEALTH - 6.0)).abs() < 1e-5,
        "the remainder hit the health"
    );
    assert_eq!(app.effects.absorption(), 0.0, "the double drain's floor");
}

/// THE REGENERATION'S BEAT HEALS (the offline stand-in server's
/// `tickServer` arc): the golden apple's 100t Regeneration II
/// heals 1.0 at the 25-tick period's four beats — the FoodData's
/// own lanes silent (the bar under the regen gates).
#[test]
fn the_regen_beat_heals_through_the_world_tick() {
    let mut app = App::new(3);
    app.enter_world();
    let gapple_id = mc_registry::item_id_by_name("golden_apple").unwrap();
    app.inventory.items[app.selected_hotbar] = (gapple_id, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // the damaged body, the bar under the food regen's gates
    app.food_data.set_food_level(14);
    app.food_data.set_saturation(0.0);
    app.health = 14.0;
    assert!(app.on_right_press(0, 0));
    for _ in 0..32 {
        app.update();
    }
    // the eat landed: the FIRST regen beat fired inside the
    // completion tick itself (the store's duration 100 beat, then
    // the tick-down) — the body climbed to 15.0
    let after_eat = app.health;
    assert_eq!(after_eat, 15.0, "the first beat healed in the walk");
    // the regen's remaining three beats over the 100-tick span
    for _ in 0..100 {
        app.update();
    }
    // the beats at durations 75/50/25 healed +3.0 (the food's slow
    // lane may add its own 80-tick beat — the ceiling the clamp
    // holds); the claim: the regen's beats RAN
    assert!(
        app.health >= after_eat + 3.0,
        "the regen's beats healed: {after_eat} -> {}",
        app.health
    );
    assert!(app.health <= MAX_HEALTH, "the Entity.heal clamp held");
}

/// THE WIRE'S EFFECTS MIRROR (the networked lane): the Update Mob
/// Effect packets' filtered truth lands through the vitals view —
/// the store's add/update laws ride, the absorption's lift rides,
/// the icons' data speaks.
#[test]
fn the_wire_effects_mirror_lands() {
    let mut app = App::new(3);
    app.enter_world();
    app.live_health_updated(crate::world_view::HealthView {
        health: 20.0,
        food: 20,
        saturation: 5.0,
        hurt_time: 0,
        hurt_yaw: 0.0,
        death_message: None,
        effects: vec![crate::world_model::MobEffectWireState {
            effect_id: crate::effects::MOB_EFFECT_ABSORPTION,
            amplifier: 0,
            duration: 2400,
            flags: crate::effects::DEFAULT_EFFECT_FLAGS,
        }],
        mining_efficiency: 0.0,
        block_break_speed: 1.0,
        submerged_mining_speed: 0.2,
    });
    assert!(app
        .effects
        .has_effect(crate::effects::MOB_EFFECT_ABSORPTION));
    assert!((app.effects.absorption() - 4.0).abs() < 1e-5);
    // the update re-speaks: the amplifier's upgrade rides
    app.live_health_updated(crate::world_view::HealthView {
        health: 20.0,
        food: 20,
        saturation: 5.0,
        hurt_time: 0,
        hurt_yaw: 0.0,
        death_message: None,
        effects: vec![crate::world_model::MobEffectWireState {
            effect_id: crate::effects::MOB_EFFECT_ABSORPTION,
            amplifier: 2,
            duration: 2400,
            flags: crate::effects::DEFAULT_EFFECT_FLAGS,
        }],
        mining_efficiency: 0.0,
        block_break_speed: 1.0,
        submerged_mining_speed: 0.2,
    });
    assert_eq!(
        app.effects
            .get(crate::effects::MOB_EFFECT_ABSORPTION)
            .unwrap()
            .amplifier,
        2,
        "the wire's upgrade landed"
    );
    assert!((app.effects.absorption() - 12.0).abs() < 1e-5);
    // the silence removes (the Remove packet's own verdict)
    app.live_health_updated(crate::world_view::HealthView {
        health: 20.0,
        food: 20,
        saturation: 5.0,
        hurt_time: 0,
        hurt_yaw: 0.0,
        death_message: None,
        effects: Vec::new(),
        mining_efficiency: 0.0,
        block_break_speed: 1.0,
        submerged_mining_speed: 0.2,
    });
    assert!(app.effects.is_empty());
}

/// THE ATTRIBUTE ROW'S OWN MIRROR (the networked lane): the
/// vitals view's mining_efficiency lift lands on the App's own
/// field — the dig walk's addend the value feeds (the E5 row the
/// 26.0 the fold computed speaks, the absent row the 0.0
/// identity keeps), the respawn's own reset the fresh body's
/// silence serves.
#[test]
fn the_mining_efficiency_row_the_mirror_lands() {
    let mut app = App::new(3);
    app.enter_world();
    assert_eq!(app.mining_efficiency, 0.0, "the absent row the 0.0 keeps");
    assert!(
        (app.block_break_speed - 1.0).abs() < 1e-9,
        "the absent row the 1.0 default keeps"
    );
    assert!(
        (app.submerged_mining_speed - 0.2).abs() < 1e-9,
        "the absent row the 0.2 default keeps"
    );
    app.live_health_updated(crate::world_view::HealthView {
        health: 20.0,
        food: 20,
        saturation: 5.0,
        hurt_time: 0,
        hurt_yaw: 0.0,
        death_message: None,
        effects: Vec::new(),
        mining_efficiency: 26.0,
        block_break_speed: 2.0,
        submerged_mining_speed: 0.1,
    });
    assert_eq!(
        app.mining_efficiency, 26.0,
        "the E5 row the vitals view's lift wears"
    );
    assert!(
        (app.block_break_speed - 2.0).abs() < 1e-9,
        "the 2.0 row the multiply lane's mirror wears"
    );
    assert!(
        (app.submerged_mining_speed - 0.1).abs() < 1e-9,
        "the 0.1 row the water arm's mirror wears"
    );
    // the re-speak REPLACES the value (a lesser enchant the
    // downgraded row wears, the stale addend never lingers; the
    // multiply rows the same wholesale truth speaks — the rows'
    // absence the defaults' return wears)
    app.live_health_updated(crate::world_view::HealthView {
        health: 20.0,
        food: 20,
        saturation: 5.0,
        hurt_time: 0,
        hurt_yaw: 0.0,
        death_message: None,
        effects: Vec::new(),
        mining_efficiency: 2.0,
        block_break_speed: 1.0,
        submerged_mining_speed: 0.2,
    });
    assert_eq!(app.mining_efficiency, 2.0, "the E1 row the replace wears");
    assert!(
        (app.block_break_speed - 1.0).abs() < 1e-9,
        "the multiply row the wholesale truth restores"
    );
    assert!(
        (app.submerged_mining_speed - 0.2).abs() < 1e-9,
        "the water row the wholesale truth restores"
    );
    // THE RESPAWN'S OWN RESET (the fresh body carries no stat
    // opinions — the server's spawn stream re-speaks them)
    app.respawn();
    assert_eq!(
        app.mining_efficiency, 0.0,
        "the respawn's reset the 0.0 wears"
    );
    assert!(
        (app.block_break_speed - 1.0).abs() < 1e-9,
        "the respawn's reset the multiply default wears"
    );
    assert!(
        (app.submerged_mining_speed - 0.2).abs() < 1e-9,
        "the respawn's reset the water default wears"
    );
}

/// THE RESPAWN CLEARS THE STORE (the death's removal — the fresh
/// body carries no effects, the absorption rides the dead body's
/// history).
#[test]
fn the_respawn_clears_the_effects() {
    let mut app = App::new(3);
    app.enter_world();
    app.effects.add_effect(
        crate::effects::MOB_EFFECT_REGENERATION,
        0,
        100,
        crate::effects::DEFAULT_EFFECT_FLAGS,
    );
    app.effects.set_absorption(8.0);
    app.dead = true;
    app.respawn();
    assert!(app.effects.is_empty(), "the death's removal");
    assert_eq!(app.effects.absorption(), 0.0);
}

/// THE MILK CLEARS THE STATUS EFFECTS (the ClearAllStatusEffects
/// payload's own lane — the FOOD-LESS consumable's arc: no roll,
/// no listener, the store empties and the bucket returns).
#[test]
fn the_milk_clears_the_status_effects() {
    let mut app = App::new(3);
    app.enter_world();
    let milk_id = mc_registry::item_id_by_name("milk_bucket").unwrap();
    app.effects.add_effect(
        crate::effects::MOB_EFFECT_REGENERATION,
        1,
        100,
        crate::effects::DEFAULT_EFFECT_FLAGS,
    );
    app.effects.set_absorption(8.0);
    app.inventory.items[app.selected_hotbar] = (milk_id, 1);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(app.on_right_press(0, 0), "the milk drinks");
    for _ in 0..32 {
        app.update();
    }
    assert!(app.eating.is_none(), "the drink completed");
    assert!(app.effects.is_empty(), "the ClearAll's own verdict");
    // the 26.2's own honesty: the removal hook's absence — the
    // absorption VALUE persists until damage drains it (the
    // AbsorptionMobEffect carries no removeAttributeModifiers)
    assert_eq!(app.effects.absorption(), 8.0, "the value persists");
}

/// THE MILK DRINK: no FOOD component (the potion's own split) —
/// the full bar consumes unconditionally, the completion folds NO
/// FoodData (no listener, no burp), and the USE_REMAINDER returns
/// the bucket.
#[test]
fn the_milk_drink_returns_the_bucket() {
    let mut app = App::new(3);
    app.enter_world();
    let milk_id = mc_registry::item_id_by_name("milk_bucket").unwrap();
    let bucket_id = mc_registry::item_id_by_name("bucket").unwrap();
    app.inventory.items[app.selected_hotbar] = (milk_id, 2);
    app.hotbar_slots = app.inventory.hotbar();
    app.food_data.set_food_level(20); // the FULL bar — milk doesn't care
    assert!(
        app.on_right_press(0, 0),
        "the consumable without food consumes"
    );
    for _ in 0..32 {
        app.update();
    }
    assert!(app.eating.is_none(), "the drink completed");
    // the FoodData untouched (no FOOD listener)
    assert_eq!(app.food_data.food_level(), 20);
    // the stack 2 -> 1 and the BUCKET returned to the slot's place
    // (the add lane's first-empty walks the selected first — the
    // 2-stack left the slot occupied, the bucket rides the next
    // free slot)
    assert_eq!(app.inventory.items[app.selected_hotbar].0, milk_id);
    assert_eq!(app.inventory.items[app.selected_hotbar].1, 1);
    let bucket_slots = app
        .inventory
        .items
        .iter()
        .filter(|s| s.0 == bucket_id && s.1 >= 1)
        .count();
    assert_eq!(bucket_slots, 1, "the remainder's own return");
    // no burp without the FOOD listener
    let burp_events = app
        .sounds
        .events
        .iter()
        .filter(|e| e.starts_with("entity.player.burp "))
        .count();
    assert_eq!(burp_events, 0, "the potion's split: no listener, no burp");
}

/// THE CANCELLED HOLD: the release mid-hold ends the eat silently
/// — no completion, no voices, no consume, the stack untouched.
#[test]
fn the_release_cancels_the_hold_silently() {
    let mut app = App::new(3);
    app.enter_world();
    let apple_id = mc_registry::item_id_by_name("apple").unwrap();
    app.inventory.items[app.selected_hotbar] = (apple_id, 2);
    app.hotbar_slots = app.inventory.hotbar();
    app.food_data.set_food_level(10);
    assert!(app.on_right_press(0, 0));
    for _ in 0..10 {
        app.update();
    }
    let pieces_mid = app.pieces.len();
    assert!(pieces_mid > 0, "the chews already spoke their pieces");
    app.on_right_release();
    assert!(app.eating.is_none(), "the release cancelled");
    // the walk continues: no completion fires (the lane's gone)
    for _ in 0..32 {
        app.update();
    }
    assert_eq!(app.food_data.food_level(), 10, "the FoodData never ate");
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (apple_id, 2),
        "the stack untouched"
    );
    let eat_events = app
        .sounds
        .events
        .iter()
        .filter(|e| e.starts_with("entity.generic.eat "))
        .count();
    assert!(
        eat_events <= 2,
        "the cancelled hold's chews only: {eat_events}"
    );
}

/// THE PAUSE FREEZES THE HOLD: the vanilla pause's own law — the
/// eat clock sleeps, no burst, no completion while paused.
#[test]
fn the_pause_freezes_the_eat_clock() {
    let mut app = App::new(3);
    app.enter_world();
    let apple_id = mc_registry::item_id_by_name("apple").unwrap();
    app.inventory.items[app.selected_hotbar] = (apple_id, 1);
    app.hotbar_slots = app.inventory.hotbar();
    app.food_data.set_food_level(10);
    assert!(app.on_right_press(0, 0));
    app.paused = true;
    for _ in 0..40 {
        app.update();
    }
    assert!(app.eating.is_some(), "the hold survived the pause");
    assert_eq!(app.eating.as_ref().unwrap().ticks, 0, "the clock slept");
    app.paused = false;
    for _ in 0..32 {
        app.update();
    }
    assert!(app.eating.is_none(), "the hold resumed and completed");
}

/// THE MOB LANE'S WIRING (the offline tick's own authority): the
/// spawner's 40-tick beat under a forced night lands a monster in
/// the ring (the light gate's own texture the beats the band
/// absorbs — the midnight surface the raw 4 the uniform admits,
/// the vanilla isDarkEnoughToSpawn the spawn tick answers), the
/// crosshair's pick + the fist's charge law land
/// through the attack lane, and the death clock's 20-tick walk
/// discards the corpse through the same update loop.
#[test]
fn the_mob_lane_spawns_attacks_and_removes_through_the_offline_tick() {
    let mut app = App::new(4);
    app.enter_world();
    app.spawn_offline_body();
    assert!(app.body.is_some(), "the offline body stands");
    // THE NIGHT GATE: the base lands deep night (the sky lane's own
    // celestial track — the sun below the horizon plane)
    app.clock_total = 18_000;
    let mut spawned_at = None;
    for t in 0..4800u64 {
        app.update();
        if !app.mobs.is_empty() {
            spawned_at = Some(t);
            break;
        }
    }
    assert!(
        spawned_at.is_some(),
        "the night spawner never landed a monster"
    );
    assert!(
        app.mobs.iter().all(|m| m.health == m.kind.max_health()),
        "the spawned monster spawns with the own attribute"
    );
    // the combat phase's clean stage: the wandering crowd leaves
    app.mobs.clear();
    assert!(app.health > 0.0, "the player survived the night");

    // THE COMBAT PICK: a controlled zombie 2.5 blocks east, on the
    // surface (the same law the spawner's own scan speaks)
    let (bx, bz) = {
        let b = app.body.as_ref().unwrap();
        (b.x, b.z)
    };
    let mut z = crate::mob::Zombie::new(999, 0.0, 0.0, 0.0, 11);
    let world = app.offline_world.as_ref().unwrap().size_blocks();
    let (sx, sz) = ((bx + 2.5).floor() as i32, bz.floor() as i32);
    let mut sy = 0;
    let wref = app.offline_world.as_ref().unwrap();
    for y in (0..world).rev() {
        if crate::body::solid(wref, sx, y, sz) {
            sy = y + 1;
            break;
        }
    }
    z.x = bx + 2.5;
    z.y = sy as f64;
    z.z = bz as f64;
    app.mobs.push(z);
    let idx = app.mobs.len() - 1;
    // aim the camera at the mob's center (the pick law's own ray —
    // the camera yaw/pitch the body.camera fold speaks)
    {
        let (mx, my, mz) = {
            let m = &app.mobs[idx];
            (m.x, m.y + 1.0, m.z)
        };
        let b = app.body.as_mut().unwrap();
        let (dx, dy, dz) = (mx - b.x, my - (b.y + b.eye_height()), mz - b.z);
        let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
        let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
        let cam_yaw = (dx / horiz).atan2(-dz / horiz);
        let cam_pitch = (dy / len3).asin();
        b.yaw = (cam_yaw - std::f64::consts::PI).to_degrees() as f32;
        b.pitch = -cam_pitch.to_degrees() as f32;
    }
    app.on_attack();
    assert!(
        app.mobs[idx].health < crate::mob::ZOMBIE_MAX_HEALTH,
        "the fist landed through the pick (health {})",
        app.mobs[idx].health
    );
    assert_eq!(
        app.mobs[idx].hurt_time,
        crate::mob::HURT_DURATION_TICKS,
        "the 10-tick flash rides"
    );
    // THE CHARGE LAW (baseDamageScaleFactor's 0.2 + 0.8c^2): the
    // ticker reset to 0 — an immediate second hit speaks the weak
    // charge (the 0.1 charge's own 0.416 damage)
    let health_after_full = app.mobs[idx].health;
    app.on_attack();
    let weak_drop = health_after_full - app.mobs[idx].health;
    assert!(
        weak_drop > 0.0 && weak_drop < 0.6,
        "the uncharged fist barely hurts ({weak_drop})"
    );

    // THE DEATH LOOT (dropAllDeathLoot's own wiring): the sliver
    // zombie dies to ONE full-charge fist — the loot spawns at
    // its feet (the spawnAtLocation law), the pickup delay the
    // default 10, every drop speaking the table's own items
    let (flesh, iron, carrot, potato) = (
        crate::mob::wire_id("rotten_flesh").unwrap(),
        crate::mob::wire_id("iron_ingot").unwrap(),
        crate::mob::wire_id("carrot").unwrap(),
        crate::mob::wire_id("potato").unwrap(),
    );
    let (mx, my, mz) = {
        let m = &app.mobs[idx];
        (m.x, m.y, m.z)
    };
    app.mobs[idx].health = 0.5;
    // the full charge on the clock (the ticker the Player.tick
    // speaks — the test pulls it straight to the delay's end)
    app.attack_ticker = 20;
    // re-aim (the zombie may have wandered off the crosshair)
    {
        let (tx, ty, tz) = {
            let m = &app.mobs[idx];
            (m.x, m.y + 1.0, m.z)
        };
        let b = app.body.as_mut().unwrap();
        let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
        let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
        let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
        b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees() as f32;
        b.pitch = -(dy / len3).asin().to_degrees() as f32;
    }
    app.on_attack();
    assert!(app.mobs[idx].health <= 0.0, "the killing blow landed");
    let loot: Vec<&crate::drops::ItemDrop> = app
        .drops
        .iter()
        .filter(|d| d.x == mx && d.y == my && d.z == mz && d.pickup_delay == 10)
        .collect();
    for d in &loot {
        assert!(
            (d.item == flesh && (1..=2).contains(&d.count))
                || ((d.item == iron || d.item == carrot || d.item == potato) && d.count == 1),
            "the drop speaks the table (item {}, count {})",
            d.item,
            d.count
        );
    }
    // the pools speak at most the flesh + the rare (the table's
    // own two-lane shape)
    assert!(loot.len() <= 2, "the pools speak at most two drops");
    // THE KILL'S OWN XP (javap LivingEntity.dropExperience → the
    // award's own split): the fist kill's flat 5 spawns the
    // PHYSICAL orbs — the [3, 1, 1] walk the getExperienceValue
    // bands speak — at the mob's feet, the grant rides the touch
    // (the orbs fly the follow law's pull, the takeXpDelay gate
    // one orb per window)
    assert_eq!(
        app.orbs.iter().map(|o| o.value).collect::<Vec<_>>(),
        vec![3, 1, 1],
        "the fist kill's own award split"
    );
    assert_eq!(app.xp.total, 0, "the grant rides the touch, not the kill");
    // the orbs' flight: the ticks pull them in and the touch grants
    // one per 2-tick window. The award's own spawn cell may sit on
    // a ledge the pull cannot cross (the vanilla's own stuck-orb
    // truth) — the test repositions the orbs beside the body (the
    // same cell the touch's plain bbox speaks), the flight law's
    // own unit tests carry the pull's numbers
    {
        let (bx, by, bz) = {
            let b = app.body.as_ref().unwrap();
            (b.x, b.y, b.z)
        };
        for (i, o) in app.orbs.iter_mut().enumerate() {
            o.x = bx + 0.1 + i as f64 * 0.05;
            o.y = by + 0.3;
            o.z = bz + 0.1;
            o.vx = 0.0;
            o.vy = 0.0;
            o.vz = 0.0;
        }
    }
    for _ in 0..90 {
        app.update();
    }
    assert!(
        app.orbs.is_empty(),
        "the orbs spoke the touch: {} left",
        app.orbs.len()
    );
    assert_eq!(
        app.xp.total,
        crate::mob::ZOMBIE_XP_REWARD,
        "the fist kill's own reward"
    );
    assert_eq!(app.xp.level, 0);
    assert!(app.xp.progress > 0.0, "the fill's fraction rides");

    // THE REMOVAL: the death clock's 20-tick walk discards the corpse
    // through the same update loop (the retain law) — the FRESH mob
    // the walk's own witness wears (the spawner's own beats the
    // gate's texture the night answers — the vec the walk never
    // indexes blind, the first corpse's clock the orbs' flight
    // already spent)
    let mut z2 = crate::mob::Zombie::new(998, 0.0, 0.0, 0.0, 12);
    {
        let b = app.body.as_ref().unwrap();
        let (bx2, bz2) = (b.x, b.z);
        let wref = app.offline_world.as_ref().unwrap();
        let size2 = wref.size_blocks();
        let (sx2, sz2) = ((bx2 + 2.5).floor() as i32, bz2.floor() as i32);
        let mut sy2 = 0;
        for y in (0..size2).rev() {
            if crate::body::solid(wref, sx2, y, sz2) {
                sy2 = y + 1;
                break;
            }
        }
        z2.x = bx2 + 2.5;
        z2.y = sy2 as f64;
        z2.z = bz2 as f64;
    }
    app.mobs.push(z2);
    let idx2 = app.mobs.len() - 1;
    app.mobs[idx2].hurt(1000.0, true);
    assert!(app.mobs[idx2].health <= 0.0, "the finishing hit kills");
    for _ in 0..25 {
        app.update();
    }
    assert!(
        !app.mobs.iter().any(|m| m.id == 998),
        "the corpse left the world"
    );
}

/// THE SWORD'S OWN ATTRIBUTE ROWS (the ToolMaterial clinit the
/// tiers keep, the createSwordAttributes the 3.0 + bonus addend
/// speaks — the seven rows the registry's own wire order carries,
/// the non-sword the zero addend the fist's floor keeps).
#[test]
fn the_sword_rows_the_addends_keep() {
    assert_eq!(
        super::held_attack_addend(Some("wooden_sword")),
        3.0,
        "the wood tier's own 0.0 bonus"
    );
    assert_eq!(
        super::held_attack_addend(Some("golden_sword")),
        3.0,
        "the gold tier the wood's 0.0 keeps"
    );
    assert_eq!(
        super::held_attack_addend(Some("stone_sword")),
        4.0,
        "the stone tier's 1.0 bonus"
    );
    assert_eq!(
        super::held_attack_addend(Some("copper_sword")),
        4.0,
        "the copper tier the stone's 1.0 keeps"
    );
    assert_eq!(
        super::held_attack_addend(Some("iron_sword")),
        5.0,
        "the iron tier's 2.0 bonus"
    );
    assert_eq!(
        super::held_attack_addend(Some("diamond_sword")),
        6.0,
        "the diamond tier's 3.0 bonus"
    );
    assert_eq!(
        super::held_attack_addend(Some("netherite_sword")),
        7.0,
        "the netherite tier's 4.0 bonus"
    );
    // THE NON-SWORD THE FLOOR KEEPS (the fist, the food, the
    // bucket, the None — the zero addend the bare hand rides)
    assert_eq!(super::held_attack_addend(None), 0.0);
    assert_eq!(super::held_attack_addend(Some("wheat")), 0.0);
    assert_eq!(super::held_attack_addend(Some("bucket")), 0.0);
    // THE SPEED'S OWN ROWS (the sword the -2.4 addend the 1.6
    // rides, the bare hand the 4.0 keeps)
    assert_eq!(super::held_attack_speed(Some("iron_sword")), 1.6);
    assert_eq!(super::held_attack_speed(Some("netherite_sword")), 1.6);
    assert_eq!(super::held_attack_speed(None), 4.0);
    // THE CHARGE'S OWN DELAY (the (1/speed)·20 law: the fist the
    // 5.0 keeps, the sword the 12.5 rides)
    assert_eq!(super::held_charge_delay(None), 5.0);
    assert_eq!(super::held_charge_delay(Some("iron_sword")), 12.5);
}

/// THE TOOL'S OWN ROWS (the Items clinit's lambda bodies the
/// axe/shovel/hoe keep, the pickaxe() direct rows the same walk
/// extracts — the addend the dmg_arg + bonus the law keeps, the
/// speed the 4.0 + spd_arg the modifier rides, the hoes the
/// CANCEL law the weak tools speak).
#[test]
fn the_tool_rows_the_addends_keep() {
    // THE AXES (the addend the dmg_arg + bonus: wooden/golden 6.0,
    // stone/copper 8.0, iron 8.0, diamond 8.0, netherite 9.0)
    assert_eq!(super::held_attack_addend(Some("wooden_axe")), 6.0);
    assert_eq!(super::held_attack_addend(Some("golden_axe")), 6.0);
    assert_eq!(super::held_attack_addend(Some("stone_axe")), 8.0);
    assert_eq!(super::held_attack_addend(Some("copper_axe")), 8.0);
    assert_eq!(super::held_attack_addend(Some("iron_axe")), 8.0);
    assert_eq!(super::held_attack_addend(Some("diamond_axe")), 8.0);
    assert_eq!(super::held_attack_addend(Some("netherite_axe")), 9.0);
    // THE SHOVELS (1.5 + bonus the seven rows share)
    assert_eq!(super::held_attack_addend(Some("wooden_shovel")), 1.5);
    assert_eq!(super::held_attack_addend(Some("golden_shovel")), 1.5);
    assert_eq!(super::held_attack_addend(Some("stone_shovel")), 2.5);
    assert_eq!(super::held_attack_addend(Some("iron_shovel")), 3.5);
    assert_eq!(super::held_attack_addend(Some("diamond_shovel")), 4.5);
    assert_eq!(super::held_attack_addend(Some("netherite_shovel")), 5.5);
    // THE PICKAXES (1.0 + bonus the direct rows keep)
    assert_eq!(super::held_attack_addend(Some("wooden_pickaxe")), 1.0);
    assert_eq!(super::held_attack_addend(Some("stone_pickaxe")), 2.0);
    assert_eq!(super::held_attack_addend(Some("iron_pickaxe")), 3.0);
    assert_eq!(super::held_attack_addend(Some("netherite_pickaxe")), 5.0);
    // THE HOES' OWN CANCEL (the dmg arg the bonus the zero addend
    // every hoe speaks — the weak tool the vanilla law keeps)
    assert_eq!(super::held_attack_addend(Some("wooden_hoe")), 0.0);
    assert_eq!(super::held_attack_addend(Some("stone_hoe")), 0.0);
    assert_eq!(super::held_attack_addend(Some("iron_hoe")), 0.0);
    assert_eq!(super::held_attack_addend(Some("netherite_hoe")), 0.0);
    // THE SPEEDS (the 4.0 + spd_arg the modifier rides — the axes
    // the 0.8/0.9/1.0 ladder, the shovels the 1.0, the pickaxes
    // the 1.2, the hoes the 1..4 fast-but-weak ladder; the
    // (4.0 + arg) shape the f64's own rounding the mirror pairs)
    assert_eq!(super::held_attack_speed(Some("wooden_axe")), 4.0 + (-3.2));
    assert_eq!(super::held_attack_speed(Some("iron_axe")), 4.0 + (-3.1));
    assert_eq!(
        super::held_attack_speed(Some("netherite_axe")),
        4.0 + (-3.0)
    );
    assert_eq!(super::held_attack_speed(Some("iron_shovel")), 4.0 + (-3.0));
    assert_eq!(super::held_attack_speed(Some("iron_pickaxe")), 4.0 + (-2.8));
    assert_eq!(super::held_attack_speed(Some("wooden_hoe")), 4.0 + (-3.0));
    assert_eq!(super::held_attack_speed(Some("stone_hoe")), 4.0 + (-2.0));
    assert_eq!(super::held_attack_speed(Some("iron_hoe")), 4.0 + (-1.0));
    assert_eq!(super::held_attack_speed(Some("diamond_hoe")), 4.0 + 0.0);
    // THE CHARGE'S OWN DELAYS (the wooden axe the 25-tick rhythm
    // the 0.8 speed speaks, the pickaxe the ~16.67 the 1.2 keeps)
    assert_eq!(
        super::held_charge_delay(Some("wooden_axe")),
        (1.0 / (4.0 + (-3.2))) * 20.0
    );
    assert_eq!(
        super::held_charge_delay(Some("iron_pickaxe")),
        (1.0 / (4.0 + (-2.8))) * 20.0
    );
}

/// THE AXE HIT'S OWN PAYOFF (the attack lane the tool row the
/// iron axe rides: the raw (base + 8.0) the full charge speaks —
/// the ticker the 22.22-delay the 30 ticks the clamp's 1.0
/// keeps — the armor fold the zombie's own 2.0 the sword's walk
/// mirrors, the axe the sword's 7.0 raw the +3 the tool ladder
/// speaks).
#[test]
fn the_axe_hit_the_addend_rides() {
    let mut app = App::new(4);
    app.enter_world();
    app.spawn_offline_body();
    let (bx, bz) = {
        let b = app.body.as_ref().unwrap();
        (b.x, b.z)
    };
    let world = app.offline_world.as_ref().unwrap().size_blocks();
    let (sx, sz) = ((bx + 2.5).floor() as i32, bz.floor() as i32);
    let mut sy = 0;
    let wref = app.offline_world.as_ref().unwrap();
    for y in (0..world).rev() {
        if crate::body::solid(wref, sx, y, sz) {
            sy = y + 1;
            break;
        }
    }
    let mut z = crate::mob::Zombie::new(4243, bx + 2.5, sy as f64, bz as f64, 5);
    z.health = crate::mob::ZOMBIE_MAX_HEALTH;
    app.mobs.push(z);
    let idx = app.mobs.len() - 1;
    {
        let (mx, my, mz) = {
            let m = &app.mobs[idx];
            (m.x, m.y + 1.0, m.z)
        };
        let b = app.body.as_mut().unwrap();
        let (dx, dy, dz) = (mx - b.x, my - (b.y + b.eye_height()), mz - b.z);
        let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
        let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
        let cam_yaw = (dx / horiz).atan2(-dz / horiz);
        let cam_pitch = (dy / len3).asin();
        b.yaw = (cam_yaw - std::f64::consts::PI).to_degrees() as f32;
        b.pitch = -cam_pitch.to_degrees() as f32;
    }
    // THE IRON AXE'S OWN HAND (the live lane's id the held read
    // keeps)
    app.live_held = Some(crate::mob::wire_id("iron_axe").unwrap());
    // THE FULL CHARGE (the axe's own 22.22-delay the 30 ticks the
    // clamp's 1.0 keeps)
    app.attack_ticker = 30;
    app.on_attack();
    // THE RAW + THE ARMOR'S OWN FOLD (the raw (2.0 + 8.0) the full
    // charge's 1.0 scale speaks — the fold the sword's own walk
    // mirrors: the f6 the 0.4 floor the subtraction falls to, the
    // (1 - f6/25) the health subtraction rides — the 9.84 the
    // fold's own answer speaks)
    let raw = (super::PLAYER_ATTACK_DAMAGE + super::held_attack_addend(Some("iron_axe"))) as f32;
    let f6 = (crate::mob::ZOMBIE_ARMOR as f32 - raw / 2.0)
        .clamp(crate::mob::ZOMBIE_ARMOR as f32 * 0.2, 20.0);
    let dealt = crate::mob::ZOMBIE_MAX_HEALTH - app.mobs[idx].health;
    assert!(
        (dealt - raw * (1.0 - f6 / 25.0)).abs() < 1e-4,
        "the iron axe's full charge the (2.0 + 8.0) raw the armor fold speaks"
    );
    assert!(
        (dealt - 9.84).abs() < 1e-3,
        "the fold's own value the axe's raw keeps ({dealt})"
    );
}

/// THE SWORD HIT'S OWN PAYOFF (the attack lane the mainhand's
/// attribute read keeps: the iron sword the (base + 5.0) the full
/// charge speaks, the bare fist the base's own floor rides, the
/// immediate re-hit the 12.5-delay's 0.04 charge the weak law
/// keeps — the sword's weak band the fist's own 0.4 never names).
#[test]
fn the_sword_hit_the_addend_rides() {
    let mut app = App::new(4);
    app.enter_world();
    app.spawn_offline_body();
    // THE CONTROLLED TARGET (the same walk the mob lane's test
    // speaks — the 2.5-east zombie the surface scan lands)
    let (bx, bz) = {
        let b = app.body.as_ref().unwrap();
        (b.x, b.z)
    };
    let world = app.offline_world.as_ref().unwrap().size_blocks();
    let (sx, sz) = ((bx + 2.5).floor() as i32, bz.floor() as i32);
    let mut sy = 0;
    let wref = app.offline_world.as_ref().unwrap();
    for y in (0..world).rev() {
        if crate::body::solid(wref, sx, y, sz) {
            sy = y + 1;
            break;
        }
    }
    let mut z = crate::mob::Zombie::new(4242, bx + 2.5, sy as f64, bz as f64, 5);
    z.health = crate::mob::ZOMBIE_MAX_HEALTH;
    app.mobs.push(z);
    let idx = app.mobs.len() - 1;
    {
        let (mx, my, mz) = {
            let m = &app.mobs[idx];
            (m.x, m.y + 1.0, m.z)
        };
        let b = app.body.as_mut().unwrap();
        let (dx, dy, dz) = (mx - b.x, my - (b.y + b.eye_height()), mz - b.z);
        let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
        let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
        let cam_yaw = (dx / horiz).atan2(-dz / horiz);
        let cam_pitch = (dy / len3).asin();
        b.yaw = (cam_yaw - std::f64::consts::PI).to_degrees() as f32;
        b.pitch = -cam_pitch.to_degrees() as f32;
    }
    // THE IRON SWORD'S OWN HAND (the live lane's id the held read
    // keeps — the wire order the name resolves)
    app.live_held = Some(crate::mob::wire_id("iron_sword").unwrap());
    // THE FULL CHARGE (the ticker the 12.5-delay sword rides —
    // 20 ticks the clamp's 1.0 keeps)
    app.attack_ticker = 20;
    app.on_attack();
    // THE RAW + THE ARMOR'S OWN FOLD (the raw (2.0 + 5.0) the
    // full charge's 1.0 scale speaks — the zombie's own 2.0 armor
    // the CombatRules.getDamageAfterAbsorb walk keeps: the f6 the
    // (armor - raw/2) the 0.4 floor the subtraction falls to, the
    // (1 - f6/25) the health subtraction rides — the 6.888 the
    // fold's own answer speaks)
    let raw = (super::PLAYER_ATTACK_DAMAGE + super::held_attack_addend(Some("iron_sword"))) as f32;
    let f6 = (crate::mob::ZOMBIE_ARMOR as f32 - raw / 2.0)
        .clamp(crate::mob::ZOMBIE_ARMOR as f32 * 0.2, 20.0);
    let dealt = crate::mob::ZOMBIE_MAX_HEALTH - app.mobs[idx].health;
    // the health-subtraction roundtrip the f32's own last-ULP the
    // fold's direct product never pairs — the 1e-4 band the
    // mirror's own honesty keeps
    assert!(
        (dealt - raw * (1.0 - f6 / 25.0)).abs() < 1e-4,
        "the iron sword's full charge the (2.0 + 5.0) raw the armor fold speaks"
    );
    // THE RAW'S OWN PAYOFF (the fold's own answer the fist's 2.0
    // raw never rides — the dealt the ~6.888 the fist's 1.968
    // never names)
    assert!(
        (dealt - 6.888).abs() < 1e-3,
        "the fold's own value the sword's raw keeps ({dealt})"
    );
    // THE SWORD'S OWN CLOCK (the onAttack reset the re-hit the
    // 0.5/12.5 = 0.04 charge speaks — the raw 7.0 x 0.20128 the
    // armor fold rides, the weak band the fist's own 0.4 never
    // names)
    app.on_attack();
    let raw_weak = (super::PLAYER_ATTACK_DAMAGE + super::held_attack_addend(Some("iron_sword")))
        as f32
        * (0.2 + 0.8 * (0.5f32 / 12.5) * (0.5f32 / 12.5));
    let f6w = (crate::mob::ZOMBIE_ARMOR as f32 - raw_weak / 2.0)
        .clamp(crate::mob::ZOMBIE_ARMOR as f32 * 0.2, 20.0);
    let weak = crate::mob::ZOMBIE_MAX_HEALTH - (raw * (1.0 - f6 / 25.0)) - app.mobs[idx].health;
    assert!(
        weak > 1.0 && weak < 2.0,
        "the sword's weak charge the 12.5 delay keeps ({weak}; raw {raw_weak} folded {})",
        raw_weak * (1.0 - f6w / 25.0)
    );
}

/// THE WATER SCREEN's GATE LAWS (javap `ScreenEffectRenderer.
/// submitWater`'s own pair): the dive scene blends the 16x16 blue
/// over EVERY pixel (the [-1,1]² quad at z=-0.5 covers any
/// frustum), the dry eye, the detached camera and the body-less
/// scene all keep the framebuffer bit-for-bit.
#[test]
fn the_water_overlay_gates_and_blends() {
    // the tile rides synthetic texels (the loader's file lane is
    // the ignored test's own) — the blue the vanilla texture speaks
    let blue_tile = || {
        let mut rgba = vec![0u8; 16 * 16 * 4];
        for p in rgba.chunks_exact_mut(4) {
            p.copy_from_slice(&[31, 85, 255, 135]);
        }
        crate::avatar::UnderwaterScreen { rgba }
    };
    let ctx_of = |cam: &crate::camera::Camera| RenderCtx {
        view: cam.view(),
        proj: crate::math::Mat4::perspective(70.0f32.to_radians(), 1.0, 0.05, 512.0),
        fog_color: [0.0, 0.0, 0.0],
        fog_near: 100.0,
        fog_far: 200.0,
        fire_anim_tick: 0, // the dive preview's still
    };

    // THE DIVE: the seeded pool + the first-person eye under the
    // surface — the overlay answers through the real gate
    let mut app = App::new(3);
    app.enter_world();
    crate::headless::seed_scene(
        &mut app,
        &crate::headless::SceneOpts {
            body: true,
            water_pool: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    app.underwater_screen = Some(blue_tile());
    let cam = app.world_camera();
    let mut fb = crate::raster::Framebuf::new(64, 64);
    fb.color.fill(255);
    app.render_water_overlay(&mut fb, &ctx_of(&cam));
    // EVERY pixel blended: the white falls toward the texel by the
    // texel alpha x the vertex alpha (135/255 x 25/255 ≈ 0.052) —
    // the red drops the most, the blue the least (the blue tile)
    let mut blue_wins = 0usize;
    for px in fb.color.chunks_exact(3) {
        assert_ne!(px[0], 255, "every pixel blended, none untouched");
        if px[2] > px[0] {
            blue_wins += 1;
        }
    }
    assert!(
        blue_wins > fb.color.len() / 3 / 2,
        "the blue dominates the blended screen: {blue_wins}"
    );

    // THE DRY EYE: the same scene without the pool keeps the
    // framebuffer untouched
    let mut dry = App::new(3);
    dry.enter_world();
    crate::headless::seed_scene(
        &mut dry,
        &crate::headless::SceneOpts {
            body: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    dry.underwater_screen = Some(blue_tile());
    let cam = dry.world_camera();
    let mut fb = crate::raster::Framebuf::new(64, 64);
    fb.color.fill(255);
    dry.render_water_overlay(&mut fb, &ctx_of(&cam));
    assert!(
        fb.color.chunks(3).all(|px| px == [255, 255, 255]),
        "the dry eye draws nothing"
    );

    // THE DETACHED CAMERA: the water eye behind the third person —
    // the submit's first-person gate sleeps the whole block
    let mut detached = App::new(3);
    detached.enter_world();
    crate::headless::seed_scene(
        &mut detached,
        &crate::headless::SceneOpts {
            body: true,
            water_pool: true,
            perspective: Some(crate::camera::CameraType::ThirdPersonBack),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    detached.underwater_screen = Some(blue_tile());
    let cam = detached.world_camera();
    let mut fb = crate::raster::Framebuf::new(64, 64);
    fb.color.fill(255);
    detached.render_water_overlay(&mut fb, &ctx_of(&cam));
    assert!(
        fb.color.chunks(3).all(|px| px == [255, 255, 255]),
        "the detached view draws no screen effect"
    );

    // THE BODY-LESS SCENE: no player, no overlay (the submit's own
    // player field dies before the gates)
    let mut nobody = App::new(3);
    nobody.enter_world();
    crate::headless::seed_scene(
        &mut nobody,
        &crate::headless::SceneOpts {
            water_pool: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    nobody.underwater_screen = Some(blue_tile());
    let cam = nobody.world_camera();
    let mut fb = crate::raster::Framebuf::new(64, 64);
    fb.color.fill(255);
    nobody.render_water_overlay(&mut fb, &ctx_of(&cam));
    assert!(
        fb.color.chunks(3).all(|px| px == [255, 255, 255]),
        "the player-less scene draws no overlay"
    );
}

// THE WATER VISION CLOCK (javap LocalPlayer.tick, the 26.2 jar's
// offsets 737-812): the dive speaks +1/tick, the air drains
// -10/tick, the clamps [0, 600] the bytecode's own Mth.clamp
// speaks.
#[test]
fn the_water_vision_clock_rides_the_eye() {
    // THE DIVE: the pool's eye climbs one tick per game tick
    let mut app = App::new(3);
    app.enter_world();
    crate::headless::seed_scene(
        &mut app,
        &crate::headless::SceneOpts {
            body: true,
            water_pool: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    assert_eq!(app.water_vision_time, 0, "the fresh spawn's clock rests");
    app.update();
    assert_eq!(app.water_vision_time, 1, "the dive speaks +1/tick");
    app.water_vision_time = 599;
    app.update();
    assert_eq!(
        app.water_vision_time, 600,
        "the clamp [0, 600] the Mth.clamp speaks"
    );
    app.update();
    assert_eq!(app.water_vision_time, 600, "the clock pins at 600");

    // THE SURFACE: the dry eye drains ten ticks a game tick
    let mut dry = App::new(3);
    dry.enter_world();
    crate::headless::seed_scene(
        &mut dry,
        &crate::headless::SceneOpts {
            body: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    dry.water_vision_time = 100;
    dry.update();
    assert_eq!(dry.water_vision_time, 90, "the air speaks -10/tick");
    dry.water_vision_time = 3;
    dry.update();
    assert_eq!(dry.water_vision_time, 0, "the clamp floors the drain");
    dry.update();
    assert_eq!(dry.water_vision_time, 0, "the dead clock stays dead");
}

// THE LAVA FOG PAIR (javap LavaFogEnvironment.setupFog +
// getBaseColor): the bare arm's 0.25/1.0, the fire-resistance
// arm's 0.0/5.0 (the store's own truth), the fixed color
// getBaseColor speaks (-6743808 = 0xFF991900), and the dry
// camera's honest None.
#[test]
fn the_lava_fog_pair_rides_the_camera_cell() {
    let mut app = App::new(3);
    app.enter_world();
    crate::headless::seed_scene(
        &mut app,
        &crate::headless::SceneOpts {
            body: true,
            lava_dive: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    let ember = [153.0 / 255.0, 25.0 / 255.0, 0.0];
    let (start, end, color) = app.lava_fog().expect("the dive arms the lava murk");
    assert_eq!(start, 0.25, "the bare arm's fconst_0.25 start");
    assert_eq!(end, 1.0, "the bare arm's fconst_1 end");
    assert_eq!(color, ember, "the murk is getBaseColor's 0xFF991900");
    // THE FIRE-RESISTANCE ARM: the potion's five blocks of vision
    assert!(
        app.effects.add_effect(
            crate::effects::MOB_EFFECT_FIRE_RESISTANCE,
            0,
            200,
            crate::effects::DEFAULT_EFFECT_FLAGS
        ),
        "the store takes the effect"
    );
    let (start, end, _) = app.lava_fog().unwrap();
    assert_eq!(start, 0.0, "the resistance arm's start");
    assert_eq!(end, 5.0, "the resistance arm's end");

    // THE DRY CAMERA: no murk at all — the atmospheric treatment
    // stays
    let mut dry = App::new(3);
    dry.enter_world();
    crate::headless::seed_scene(
        &mut dry,
        &crate::headless::SceneOpts {
            body: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    assert!(dry.lava_fog().is_none(), "the dry camera keeps the sky");
}

// THE HEAD-IN-BLOCK WALK (javap getViewBlockingState + the submit
// chain's own first arm): the bury scene's eight samples all read
// the solid, the walk returns the kind; the dry scene's samples
// read air; the fluids and the leaves never block (the
// isSuffocating default the registration speaks); the particle
// table pins the models' own entries to the generated tile names.
#[test]
fn the_block_screen_walk_reads_the_eye_cell() {
    let mut app = App::new(3);
    app.enter_world();
    crate::headless::seed_scene(
        &mut app,
        &crate::headless::SceneOpts {
            body: true,
            bury: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    let kind = app.block_screen().expect("the buried eye reads the solid");
    assert_eq!(
        kind,
        crate::world::DIRT,
        "the bury stages the eye cell's dirt"
    );
    assert!(
        is_view_blocking(kind),
        "the dirt passes the isSuffocating default"
    );
    assert_eq!(particle_tile(kind), 1, "the dirt particle is the dirt tile");
    assert_eq!(
        mc_registry::generated::atlas::tile_name(1),
        "dirt",
        "the tile index rides the generated names"
    );

    // THE DRY CAMERA: no burial — the walk reads air, the arm
    // sleeps (the honest unseen the old comment spoke)
    let mut dry = App::new(3);
    dry.enter_world();
    crate::headless::seed_scene(
        &mut dry,
        &crate::headless::SceneOpts {
            body: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    assert!(
        dry.block_screen().is_none(),
        "the dry eye keeps the world visible"
    );
}

// THE VIEW-BLOCKING + PARTICLE TABLES (the registration default's
// own palette mapping): the full-cube solids block, the fluids and
// the leaf kinds never do; the particle entries the vanilla models
// speak — grass_block's particle is dirt, the log's is its side,
// the deepslate's is its side.
#[test]
fn the_view_blocking_law_maps_the_palette() {
    // THE SOLIDS: the suffocation default's own set
    for kind in [
        crate::world::STONE,
        crate::world::DIRT,
        crate::world::GRASS,
        crate::world::LOG,
        crate::world::SAND,
        crate::world::BEDROCK,
        crate::world::NETHERRACK,
    ] {
        assert!(is_view_blocking(kind), "the solid kind {kind} blocks");
    }
    // THE NON-BLOCKERS: the fluids, the leaves, the flat cover, THE
    // WEB (the dev-22 round — the noCollision pc 4247 the suffocation
    // conjunction refuses)
    for kind in [
        crate::world::AIR,
        crate::world::WATER,
        crate::world::LAVA,
        crate::world::LEAVES,
        crate::world::PALE_LEAVES,
        crate::world::LEAF_LITTER,
        crate::world::COBWEB,
    ] {
        assert!(!is_view_blocking(kind), "the kind {kind} never blocks");
    }
    // THE WOOL'S OWN SOLID (the dev-22 round — the full-cube default
    // the isSuffocating conjunction keeps)
    assert!(
        is_view_blocking(crate::world::WHITE_WOOL),
        "the wool the full-cube suffocation keeps"
    );
    // THE PARTICLE ENTRIES: the models' own choice
    assert_eq!(
        mc_registry::generated::atlas::tile_name(particle_tile(crate::world::GRASS)),
        "dirt",
        "grass_block's particle is dirt"
    );
    assert_eq!(
        mc_registry::generated::atlas::tile_name(particle_tile(crate::world::LOG)),
        "oak_log",
        "the log's particle is its side"
    );
    assert_eq!(
        mc_registry::generated::atlas::tile_name(particle_tile(crate::world::DEEPSLATE)),
        "deepslate",
        "the deepslate's particle is its side"
    );
    assert_eq!(
        mc_registry::generated::atlas::tile_name(particle_tile(crate::world::NETHERRACK)),
        "netherrack",
        "the netherrack's particle is its body"
    );
    // THE NON-BLOCKERS never ask: TILE_NONE keeps the table total
    for kind in [
        crate::world::WATER,
        crate::world::LAVA,
        crate::world::LEAVES,
        crate::world::PALE_LEAVES,
        crate::world::LEAF_LITTER,
    ] {
        assert_eq!(
            particle_tile(kind),
            mc_registry::generated::atlas::TILE_NONE,
            "the kind {kind} owns no particle screen"
        );
    }
}

// THE TOOL WAKE'S OWN PALETTE ROWS (the dev-22 round): the cobweb +
// white_wool kinds the atlas tiles bank (the cobweb the cross arm's
// tile, the wool the full-cube generic walk's), the collision the
// noCollision vs full-cube rows speak, the placeable lookup the
// item ids' reverse walk serves (the cobweb the BlockItemIds.COBWEB
// item form HAS, the wool the collection's own member), the cross
// mesh the diagonal quads the web's own lane wears.
#[test]
fn the_web_and_wool_the_palette_rows_wear() {
    use mc_registry::generated::atlas;
    let web = crate::world::COBWEB;
    let wool = crate::world::WHITE_WOOL;
    // THE ATLAS'S OWN ROWS (the tiles the regenerated walk banks —
    // the cobweb tile the cross arm resolves, the wool the
    // cube_all's `all` ref answers; both untinted — the 78 the
    // soul sand's tail row the autopush-598 wake banks)
    assert_eq!(
        atlas::N_KINDS,
        797,
        "the palette the weighted plates' own tail banks"
    );
    assert_eq!(atlas::KIND_NAMES[web as usize], "cobweb");
    assert_eq!(atlas::KIND_NAMES[wool as usize], "white_wool");
    assert_eq!(atlas::REGISTRY_NAMES[web as usize], "cobweb");
    assert_eq!(atlas::REGISTRY_NAMES[wool as usize], "white_wool");
    let web_tile = atlas::FACE_TILES[web as usize][0];
    let wool_tile = atlas::FACE_TILES[wool as usize][0];
    assert_ne!(web_tile, atlas::TILE_NONE, "the web the atlas tile banks");
    assert_ne!(wool_tile, atlas::TILE_NONE, "the wool the atlas tile banks");
    assert_eq!(
        atlas::tile_name(web_tile),
        "cobweb",
        "the web tile the cross master names"
    );
    assert_eq!(
        atlas::tile_name(wool_tile),
        "white_wool",
        "the wool tile the cube master names"
    );
    for face in 0..6 {
        assert_eq!(
            atlas::FACE_TILES[wool as usize][face],
            wool_tile,
            "the wool face {face} the cube_all's same tile keeps"
        );
        assert_eq!(
            atlas::TINTS[wool as usize][face],
            0,
            "the wool face {face} untinted"
        );
        assert_eq!(
            atlas::TINTS[web as usize][face],
            0,
            "the web face {face} untinted"
        );
    }
    // THE COLLISION (the web the noCollision Empty row, the wool the
    // full-cube default)
    assert_eq!(
        crate::world::collision_shape(web),
        crate::world::CollisionShape::Empty,
        "the web the pass-through shape keeps"
    );
    assert_eq!(
        crate::world::collision_shape(wool),
        crate::world::CollisionShape::Full,
        "the wool the full-cube shape keeps"
    );
    // THE PLACEABLE LOOKUP (the item ids' reverse walk — the web the
    // BlockItemIds.COBWEB form places, the wool the collection
    // member places)
    assert_eq!(
        crate::world::World::placeable_block("cobweb"),
        Some(web),
        "the web the item form places"
    );
    assert_eq!(
        crate::world::World::placeable_block("white_wool"),
        Some(wool),
        "the wool the item form places"
    );
    // THE MESH QUADS the mesher's own test module pins (the empty
    // stage the helpers keep — the web the cross planes, the wool
    // the six-face cube)
}

/// THE SOUL SAND'S OWN PALETTE ROW (the autopush-598 round — javap
/// Blocks clinit pc 10639-10722 the 26.2 jar): the speedFactor(0.4f)
/// row's first palette-native consumer — the atlas the cube_all
/// generic walk resolves (blockstates soul_sand.json "" ->
/// block/soul_sand), the dig row the strength(0.5f) pc 10662 the
/// no-demand arm speaks (NO requiresCorrectToolForDrops the
/// bare-hand harvest the verdict wears), the drops table the
/// self-entry banks, the collision the Column(14) SHAPE keeps, the
/// placeable lookup the item form answers, the view-blocking walk
/// the opaque full footprint the conjunction admits.
#[test]
fn the_soul_sand_the_palette_row_wears() {
    use mc_registry::generated::atlas;
    let sand = crate::world::SOUL_SAND;
    assert_eq!(sand, 77, "the tail slot the palette wake banks");
    assert_eq!(
        atlas::N_KINDS,
        797,
        "the palette the weighted plates' own tail banks"
    );
    assert_eq!(atlas::KIND_NAMES[sand as usize], "soul_sand");
    assert_eq!(atlas::REGISTRY_NAMES[sand as usize], "soul_sand");
    assert_eq!(
        crate::world::NAME_OF[sand as usize],
        "soul_sand",
        "the name row the dig/drop/sound lanes read"
    );
    // THE ATLAS'S OWN ROW (the tile the cube_all's `all` ref banks —
    // every face the same master, untinted)
    let sand_tile = atlas::FACE_TILES[sand as usize][0];
    assert_ne!(sand_tile, atlas::TILE_NONE, "the sand the atlas tile banks");
    assert_eq!(atlas::tile_name(sand_tile), "soul_sand");
    for face in 0..6 {
        assert_eq!(atlas::FACE_TILES[sand as usize][face], sand_tile);
        assert_eq!(
            atlas::TINTS[sand as usize][face],
            0,
            "the sand face {face} untinted"
        );
    }
    // THE COLLISION (the Block.column(16.0, 0.0, 14.0) SHAPE the
    // 14/16 band keeps) + THE DIG ROW (the strength(0.5f) the
    // no-demand arm speaks — the bare hand the verdict harvests)
    assert_eq!(
        crate::world::collision_shape(sand),
        crate::world::CollisionShape::Column(14),
        "the sand the 14/16 column keeps"
    );
    assert_eq!(
        mc_registry::generated::block_dig::dig_row("soul_sand"),
        Some((0.5, false)),
        "the dig row the clinit strength speaks"
    );
    // THE PLACEABLE LOOKUP + THE VIEW-BLOCKING WALK (the item form
    // places, the opaque footprint the suffocation conjunction
    // admits — the wool's own verdict the same walk wears)
    assert_eq!(
        crate::world::World::placeable_block("soul_sand"),
        Some(sand),
        "the sand the item form places"
    );
    assert!(
        crate::screens::is_view_blocking(sand),
        "the sand the view blocks"
    );
    // THE SPEED FACTOR'S OWN TABLE ROW (the generated walk the
    // body's move tail reads — the honey pair the same table banks)
    assert_eq!(
        mc_registry::generated::block_factors::speed_factor("soul_sand"),
        0.4,
        "the speed factor the clinit row banks"
    );
    assert_eq!(
        mc_registry::generated::block_factors::jump_factor("soul_sand"),
        1.0,
        "the jump factor the no-row default keeps"
    );
    assert_eq!(
        mc_registry::generated::block_factors::speed_factor("honey_block"),
        0.4,
        "the honey speed the same table banks"
    );
    assert_eq!(
        mc_registry::generated::block_factors::jump_factor("honey_block"),
        0.5,
        "the honey jump the 0.5 row banks"
    );
    assert_eq!(
        mc_registry::generated::block_factors::speed_factor("stone"),
        1.0,
        "the unmatched block the 1.0 default rides"
    );
}

/// THE OBSIDIAN'S OWN PALETTE ROW (the autopush-606 round — javap
/// Blocks clinit pc 6491-6524 the 26.2 jar): the frame-scan lane's
/// own building block the item form places — the dig row (50.0
/// hardness + requiresCorrectToolForDrops — the strength(50.0f,
/// 1200.0f) pc 6512-6518 + requiresCorrectToolForDrops() pc 6509
/// the clinit arm speaks), the drops table (the self-entry the
/// generated DROPS_OBSIDIAN row banks), the sounds' STONE family
/// (the NO SoundType call the clinit arm carries — the default the
/// registration keeps), the full-cube collision the BlockBehaviour
/// default the collision's Full arm answers, the pickaxe class the
/// mineable table banks (the correct-tool verdict the class match
/// answers — the bare hand the requires-tool gate refuses), the
/// fire tables the silence keeps (the NO setFlammable row the
/// obsidian's own registration never speaks).
#[test]
fn the_obsidian_the_palette_row_the_dig_row_wears() {
    use mc_registry::generated::atlas;
    let obs = crate::world::OBSIDIAN;
    assert_eq!(obs, 86, "the tail slot the palette wake banks");
    assert_eq!(
        atlas::KIND_NAMES[obs as usize],
        "obsidian",
        "the kind name the palette row banks"
    );
    assert_eq!(
        atlas::REGISTRY_NAMES[obs as usize],
        "obsidian",
        "the registry name the mapper lane reads"
    );
    assert_eq!(
        crate::world::NAME_OF[obs as usize],
        "obsidian",
        "the name row the dig/drop/sound lanes read"
    );
    // THE ATLAS'S OWN ROW (the tile the cube_all's `all` ref banks —
    // every face the same master, untinted)
    let obs_tile = atlas::FACE_TILES[obs as usize][0];
    assert_ne!(
        obs_tile,
        atlas::TILE_NONE,
        "the obsidian the atlas tile banks"
    );
    assert_eq!(atlas::tile_name(obs_tile), "obsidian");
    for face in 0..6 {
        assert_eq!(atlas::FACE_TILES[obs as usize][face], obs_tile);
        assert_eq!(
            atlas::TINTS[obs as usize][face],
            0,
            "the obsidian face {face} untinted"
        );
    }
    // THE FULL-CUBE COLLISION (the BlockBehaviour default the
    // collision's Full arm answers) + THE DIG ROW (the strength
    // (50.0f, 1200.0f) pc 6512-6518 + requiresCorrectToolForDrops
    // pc 6509 the clinit arm speaks — the generated table's own
    // dump banks (50.0, true))
    assert_eq!(
        crate::world::collision_shape(obs),
        crate::world::CollisionShape::Full,
        "the obsidian the full-cube default keeps"
    );
    assert_eq!(
        mc_registry::generated::block_dig::dig_row("obsidian"),
        Some((50.0, true)),
        "the dig row the clinit strength speaks"
    );
    // THE MINEABLE CLASS'S OWN VERDICT (the pickaxe the class
    // match answers, the bare hand the requires-tool gate refuses)
    let pickaxe = mc_registry::generated::item_order::ITEM_WIRE_ORDER
        .iter()
        .position(|n| *n == "iron_pickaxe")
        .expect("the pickaxe the wire order banks") as i32;
    assert!(
        crate::dig::DigState::correct_tool(obs, Some(pickaxe)),
        "the pickaxe the correct-tool verdict answers"
    );
    assert!(
        !crate::dig::DigState::correct_tool(obs, None),
        "the bare hand the requires-tool gate refuses"
    );
    // THE PLACEABLE LOOKUP + THE VIEW-BLOCKING WALK (the item form
    // places, the opaque footprint the suffocation conjunction
    // admits)
    assert_eq!(
        crate::world::World::placeable_block("obsidian"),
        Some(obs),
        "the obsidian the item form places"
    );
    assert!(
        crate::screens::is_view_blocking(obs),
        "the obsidian the view blocks"
    );
    // THE DROPS TABLE'S OWN SELF-ENTRY (the generated DROPS_OBSIDIAN
    // row the loot lane reads)
    assert_eq!(
        mc_registry::generated::block_drops::drops("obsidian").map(|pools| pools.len()),
        Some(1),
        "the self-entry the drops table banks"
    );
}

/// THE FLINT THE FRAME THE GATE THE FILL WEARS (javap
/// FlintAndSteelItem.useOn pc 135-265 + BaseFireBlock.onPlace pc
/// 0-50 over the 26.2 jar — the fire the placement arm sets the
/// onPlace tail's scan the interior fills): the obsidian frame the
/// player builds the placement lane serves, the flint the floor's
/// top face sparks, the fire cell the scan's own start the EMPTY
/// predicate admits, the fill the interior the NETHER_PORTAL kind
/// wears (the FIRE CELL the interior member the fill OVERWRITES),
/// the relight the emission-11 the light grid bakes, the frame
/// cells the fill never touches; the frame-less spark the fire's
/// own landing keeps (the failed scan the FALSE the vanilla's own
/// pc 50 return the portal-less arm speaks).
#[test]
fn the_flint_the_frame_the_gate_the_fill_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/flint_and_steel"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        // THE CLEAR BOX (the generated terrain the frame's own
        // neighborhood never names — the scan the cleared air
        // alone reads)
        for x in (c - 4)..=(c + 4) {
            for y in (top - 2)..=(top + 5) {
                for z in (c - 3)..=(c + 3) {
                    w.set(x, y, z, crate::world::AIR);
                }
            }
        }
        // THE OBSIDIAN FLOOR (the frame's own bottom row the
        // click's target the top face wears) + THE RING: the side
        // columns + the top row the interior rows the 2x3 wears
        for x in (c - 2)..=(c + 2) {
            w.set(x, top - 1, c, crate::world::OBSIDIAN); // the floor row
            w.set(x, top + 3, c, crate::world::OBSIDIAN); // the top row
        }
        for y in top..=(top + 2) {
            w.set(c - 2, y, c, crate::world::OBSIDIAN); // the west column
            w.set(c + 2, y, c, crate::world::OBSIDIAN); // the east column
        }
    }
    // THE INTERIOR'S OWN EYE (the camera the frame's own interior
    // wears — the straight-down ray the floor's top face the clip
    // answers, the frame's own top row the ray never crosses)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 2.5, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the spark consumed the press");
    // THE FILL'S OWN VERDICT: the interior the portal kind wears —
    // the fire cell the scan's own start the interior member the
    // fill OVERWRITES
    for y in top..=(top + 2) {
        for x in (c - 1)..=(c + 1) {
            assert_eq!(
                app.offline_world.as_ref().unwrap().get(x, y, c),
                crate::world::NETHER_PORTAL,
                "the interior cell ({x},{y}) the portal kind wears"
            );
        }
    }
    // THE FRAME'S OWN INTEGRITY: the ring cells the fill never
    // touches
    for x in (c - 2)..=(c + 2) {
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(x, top - 1, c),
            crate::world::OBSIDIAN,
            "the floor cell ({x}) the fill never touches"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(x, top + 3, c),
            crate::world::OBSIDIAN,
            "the top cell ({x}) the fill never touches"
        );
    }
    // THE FRAME-LESS CONTROL'S OWN SPARK: the failed scan the
    // fire's own landing keeps — the plain floor the flint the
    // fire the canBePlacedAt walk serves, the portal the scan
    // never fills
    let mut app2 = App::new(3);
    app2.enter_world();
    app2.offline_held_updated(Some("item/flint_and_steel"));
    let size2 = app2.offline_world.as_ref().unwrap().size_blocks();
    let c2 = size2 / 2;
    let top2 = app2.spawn_y;
    {
        let w = app2.offline_world.as_mut().unwrap();
        for x in (c2 - 3)..=(c2 + 3) {
            for y in (top2 - 2)..=(top2 + 4) {
                for z in (c2 - 3)..=(c2 + 3) {
                    w.set(x, y, z, crate::world::AIR);
                }
            }
            w.set(x, top2 - 1, c2, crate::world::OBSIDIAN); // the plain floor
        }
    }
    app2.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c2 as f32 + 0.5, top2 as f32 + 4.0, c2 as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app2.on_right_press(0, 0), "the frame-less spark consumed");
    assert_eq!(
        app2.offline_world.as_ref().unwrap().get(c2, top2, c2),
        crate::world::FIRE,
        "the frame-less spark the fire's own landing keeps"
    );
    assert!(
        app2.offline_world.as_ref().unwrap().get(c2 + 1, top2, c2) != crate::world::NETHER_PORTAL,
        "the frame-less spark the portal never fills"
    );
}

/// THE HONEY'S OWN PALETTE ROW (the autopush-599 round — javap
/// Blocks clinit pc 26860-26900 the 26.2 jar): the speedFactor(0.4f)
/// + jumpFactor(0.5f) rows' first palette-native consumer — the
/// atlas the two-element walk resolves (the #down master the kind
/// tile banks, the #up/#side skins the static bank appends), the
/// dig row the NO-strength default keeps (the instabreak 0.0 the
/// bootstrapped registry speaks), the collision the full-cube
/// default keeps, the placeable lookup the item form answers, the
/// view-blocking walk the noOcclusion shape refuses, the fire
/// tables the silence keeps.
#[test]
fn the_honey_the_palette_row_wears() {
    use mc_registry::generated::atlas;
    let honey = crate::world::HONEY_BLOCK;
    assert_eq!(honey, 78, "the tail slot the palette wake banks");
    assert_eq!(
        atlas::N_KINDS,
        797,
        "the palette the weighted plates' own tail banks"
    );
    assert_eq!(atlas::KIND_NAMES[honey as usize], "honey_block");
    assert_eq!(atlas::REGISTRY_NAMES[honey as usize], "honey_block");
    assert_eq!(
        crate::world::NAME_OF[honey as usize],
        "honey_block",
        "the name row the dig/drop/sound lanes read"
    );
    // THE ATLAS'S OWN ROWS (the #down master the kind tile banks,
    // the two skins the static bank appends — the named consts the
    // mesher's per-quad verdict reads)
    assert_eq!(atlas::TILE_HONEY_BLOCK_BOTTOM, 78, "the kind's own tile");
    assert_eq!(
        atlas::tile_name(atlas::TILE_HONEY_BLOCK_TOP),
        "honey_block_top"
    );
    assert_eq!(
        atlas::tile_name(atlas::TILE_HONEY_BLOCK_SIDE),
        "honey_block_side"
    );
    // THE COLLISION (the dev-23 round — javap HoneyBlock
    // getCollisionShape the SHAPE field the clinit's
    // `Block.column(14.0d, 0.0d, 15.0d)` pcs 11-21 the 26.2 jar
    // keeps: the InsetColumn(15, 1) pair the 15/16 band and the
    // 1/16 side inset banks — the full-cube row the 599 round
    // kept the jar's own constant retires) + THE DIG ROW (the
    // instabreak 0.0 the no-demand arm speaks — the bare fist the
    // instant break arms)
    assert_eq!(
        crate::world::collision_shape(honey),
        crate::world::CollisionShape::InsetColumn(15, 1),
        "the honey the 14/16 inset column keeps"
    );
    assert_eq!(
        mc_registry::generated::block_dig::dig_row("honey_block"),
        Some((0.0, false)),
        "the dig row the no-strength default speaks"
    );
    // THE PLACEABLE LOOKUP + THE VIEW-BLOCKING WALK (the item form
    // places, the noOcclusion shape the suffocation conjunction
    // refuses — the web's own verdict the same walk wears)
    assert_eq!(
        crate::world::World::placeable_block("honey_block"),
        Some(honey),
        "the honey the item form places"
    );
    assert!(
        !crate::screens::is_view_blocking(honey),
        "the honey the view never screens"
    );
    // THE FACTOR LANES' OWN TABLE ROWS (the generated walk the
    // body's move tail + jump fold read — the palette wake the
    // kind name feeds)
    assert_eq!(
        mc_registry::generated::block_factors::speed_factor(crate::world::NAME_OF[honey as usize]),
        0.4,
        "the speed factor the clinit row banks"
    );
    assert_eq!(
        mc_registry::generated::block_factors::jump_factor(crate::world::NAME_OF[honey as usize]),
        0.5,
        "the jump factor the 0.5 row banks"
    );
    // THE FIRE TABLES' OWN SILENCE (the setFlammable walk the honey
    // row never wears — the parity law the canBurn verdict agrees)
    assert_eq!(crate::world::fire_ignite_odds(honey), 0);
    assert_eq!(crate::world::fire_burn_odds(honey), 0);
    assert_eq!(crate::world::fire_can_burn(honey), false);
}

// THE WATER FOG PAIR (javap WaterFogEnvironment.setupFog +
// EnvironmentAttributes <clinit>): the start -8.0, the end 96.0 x
// max(0.25, getWaterVision()) — the ramp's own 24.0 / 57.6 / 76.8 /
// 96.0 stations — the color the WATER_FOG_COLOR default 0x050533.
#[test]
fn the_water_fog_pair_rides_the_vision_ramp() {
    let mut app = App::new(3);
    app.enter_world();
    crate::headless::seed_scene(
        &mut app,
        &crate::headless::SceneOpts {
            body: true,
            water_pool: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    let murk = [5.0 / 255.0, 5.0 / 255.0, 51.0 / 255.0];
    // the fresh dive: vision 0, the 0.25 floor speaks 24.0
    app.water_vision_time = 0;
    let (start, end, color) = app.water_fog().expect("the pool arms the murk");
    assert_eq!(start, -8.0, "the start the bytecode default speaks");
    assert!(
        (end - 24.0).abs() < 1e-3,
        "the fresh dive's end is the 0.25 floor: {end}"
    );
    assert_eq!(color, murk, "the murk is the attribute's 0x050533");
    // the ramp's first station: wvt 100 -> vision 0.6 -> 57.6
    app.water_vision_time = 100;
    let (_, end, _) = app.water_fog().unwrap();
    assert!(
        (end - 96.0 * 0.6).abs() < 1e-2,
        "the 100-tick station's end: {end}"
    );
    // the mid ramp: wvt 350 -> h 1.0 + k 0.5 -> vision 0.8 -> 76.8
    app.water_vision_time = 350;
    let (_, end, _) = app.water_fog().unwrap();
    assert!((end - 96.0 * 0.8).abs() < 1e-2, "the mid ramp's end: {end}");
    // the ramp's full open: wvt 600 -> vision 1.0 -> 96.0
    app.water_vision_time = 600;
    let (_, end, _) = app.water_fog().unwrap();
    assert!((end - 96.0).abs() < 1e-2, "the soaked eye's end: {end}");

    // THE DRY CAMERA: no murk at all — the atmospheric treatment
    // stays (the fog type rides the camera-position query)
    let mut dry = App::new(3);
    dry.enter_world();
    crate::headless::seed_scene(
        &mut dry,
        &crate::headless::SceneOpts {
            body: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    dry.water_vision_time = 600;
    assert!(dry.water_fog().is_none(), "the dry camera keeps the sky");
}

// THE MURK'S OWN FRAME (the render path): the dive's world frame
// clears to the murk exactly (the skyEnd law — the sky bands fold
// into the fog color, the celestials sleep inside it) and the
// soaked eye's open fog speaks a different frame than the fresh
// dive's — the ramp rides the render.
#[test]
fn the_water_fog_paints_the_murk_over_the_sky() {
    let _render_lane = render_lane();
    let mut app = App::new(3);
    app.enter_world();
    crate::headless::seed_scene(
        &mut app,
        &crate::headless::SceneOpts {
            body: true,
            water_pool: true,
            perspective: Some(crate::camera::CameraType::FirstPerson),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    // THE FRESH DIVE (the honest world the overlay texture wears —
    // the 16x16 translucent blues the alpha 135/146 the fog blend
    // carries, the pure attribute color [5,5,51] the fullscreen
    // cover the sky band never keeps — the pre-jar_root lane the
    // load's failure the fog-only frame wore, the degenerate pass
    // the record retires)
    app.water_vision_time = 0;
    let mut fb = crate::raster::Framebuf::new(64, 64);
    fb.color.fill(0);
    let mut assets = GuiAssets::open(assets_root());
    app.render_world_3d(&mut fb, &mut assets);
    // THE MURK'S OWN PAINT (the honest overlay's signature): the
    // dive frame the texture's blend family wears — the two
    // dominant murk-over-scene blends the deterministic scene
    // paints, the floors the overlay's absence the 0-pixel red
    // answers
    let blend_a = [162u8, 111, 93];
    let blend_b = [148u8, 97, 77];
    let a_pixels = fb.color.chunks_exact(3).filter(|px| *px == blend_a).count();
    let b_pixels = fb.color.chunks_exact(3).filter(|px| *px == blend_b).count();
    assert!(
        a_pixels > 150 && b_pixels > 60,
        "the murk overlay the dive frame paints (a={a_pixels} b={b_pixels})"
    );
    // THE RAMP RIDES THE RENDER: the soaked eye (wvt 600, end 96)
    // sees a different frame than the fresh dive (end 24) — the
    // fog pair's own law speaks through the pixels
    app.water_vision_time = 600;
    let mut open = crate::raster::Framebuf::new(64, 64);
    open.color.fill(0);
    let mut open_assets = GuiAssets::open(assets_root());
    app.render_world_3d(&mut open, &mut open_assets);
    let moved = open
        .color
        .chunks_exact(3)
        .zip(fb.color.chunks_exact(3))
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        moved > 64 * 64 / 10,
        "the fog end's open speaks through the frame: {moved} pixels moved"
    );
}

#[test]
fn the_crosshair_verdict_speaks_the_entity_reach_law() {
    // THE 26.2 ENTITY ARM (LocalPlayer.pick's own tail): the
    // winning hit rides `filterHitResult(hit, eye, entityRange)` —
    // the clip point stays `closerThan(eye, 3.0)` STRICT (the
    // entity_interaction_range 3.0 the Attributes clinit's ctor
    // args speak), else the MISS the null crosshairPickEntity
    // speaks. The 2.5 body the attack tests stage picks; the 3.5
    // body sits INSIDE the 4.5 search sphere but OUTSIDE the 3.0
    // arm — the verdict the null speaks.
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
    let aim_at_first = |app: &mut App| {
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
    };
    // INSIDE the reach: the 2.5 body the verdict keeps
    {
        let bx = app.body.as_ref().unwrap().x;
        app.mobs[0].x = bx + 2.5;
    }
    aim_at_first(&mut app);
    assert!(
        app.pick_mob(&[]).is_some(),
        "the 2.5 body inside the entity arm's reach"
    );
    // BEYOND the 3.0 arm, inside the 4.5 search sphere: the
    // closerThan refuses — the verdict the null speaks
    {
        let bx = app.body.as_ref().unwrap().x;
        app.mobs[0].x = bx + 3.5;
    }
    aim_at_first(&mut app);
    assert!(
        app.pick_mob(&[]).is_none(),
        "the 3.5 body the closerThan(3.0) refuses"
    );
}

#[test]
fn the_wire_lane_rides_the_crosshair_verdict() {
    // THE ONE VERDICT the 26.2 law speaks: the per-frame pick
    // walks the SAME chain the render walks — the wire's remote
    // bodies the offline lane never owned. The named-but-unflagged
    // zombie (the accessor-3 Boolean silent — the byte the
    // named-AND-visible form rides, the --net-crosshair stage's
    // own word) ON the facing line: the chain index the wire side
    // carries when the offline lane sleeps.
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the body (the camera's own authority — the seed_scene
    // placement: the world-center column, the east facing)
    let n = app.world_size as f64;
    let mut body =
        crate::body::PlayerBody::new(n / 2.0 + 0.5, app.spawn_y as f64, n / 2.0 + 0.5, -90.0, 0.0);
    body.on_ground = true;
    app.body = Some(body);
    // the named-but-unflagged zombie ON the facing line (the eye's
    // own ray the pick walks — yaw -90 faces +x, the level ray at
    // feet + 1.62 crosses the 0..1.95 box)
    let (bx, bz) = {
        let b = app.body.as_ref().unwrap();
        (b.x, b.z)
    };
    let mut w = wire_mob_named(100, bx + 1.8, app.spawn_y as f64, bz, "minecraft:zombie");
    w.custom_name = Some("Lurker".to_string());
    app.live_mobs_updated(vec![w]);
    // the staged word: named, the byte silent
    assert_eq!(app.net_mobs[0].custom_name.as_deref(), Some("Lurker"));
    assert!(
        !app.net_mobs[0].name_visible,
        "the byte the tag's first arm reads stays silent"
    );
    // the verdict: the chain index 0 — the wire side's own (the
    // offline lane empty), the tag's second arm the gate rides
    assert_eq!(
        app.pick_mob(&app.net_mobs),
        Some(0),
        "the wire mob the crosshair verdict speaks"
    );
    // THE OFF-LINE BODY: the ray the line rides misses the box
    // laterally (the 0.9 side, the half-width 0.3 the box keeps)
    // — the verdict the null speaks
    app.net_mobs[0].z = bz + 0.9;
    assert!(
        app.pick_mob(&app.net_mobs).is_none(),
        "the lateral miss the one-list walk speaks"
    );
}

#[test]
fn the_wire_attack_speaks_the_server_id() {
    // THE 26.2'S OWN gameMode.attack WALK (the dump's order: ensure,
    // SEND, attack, RESET): the crosshair's verdict lands the wire
    // body, the dedicated Attack packet the SERVER'S OWN id carries
    // rides the command lane, the remote body's HURT the client
    // never runs (the health the server's to keep — the Hurt echo
    // the hurt lane renders), the ATTACKER'S OWN PREDICTION the
    // client law walks (the crit arm + the voice branch — the
    // staged ground hit the strong voice speaks, the crit gate
    // refuses), the RESET the resetAttackStrengthTicker step
    // speaks, the SWING the shared tail rides.
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
    // the wire zombie ON the facing line (the eye's own ray the
    // pick walks), the SERVER'S OWN id 100 the offline counter
    // never mints
    app.live_mobs_updated(vec![wire_mob_named(
        100,
        bx + 1.8,
        app.spawn_y as f64,
        bz,
        "minecraft:zombie",
    )]);
    let health_before = app.net_mobs[0].health;
    assert_eq!(
        app.pick_mob(&app.net_mobs),
        Some(0),
        "the wire body the verdict speaks (the offline lane sleeps)"
    );
    // the command lane the live session owns — the test's own
    // recorder
    let (tx, rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    app.attack_ticker = 10; // the charged clock the reset re-arms
    app.on_attack();
    // THE SEND: the record's one VarInt carries the server's own
    // id, the packet the swing beat FOLLOWS (the dump's order)
    assert_eq!(
        rx.try_recv().unwrap(),
        crate::world_view::ContainerCommand::Attack { entity_id: 100 },
        "the server's own id the Attack packet carries"
    );
    assert_eq!(
        rx.try_recv().unwrap(),
        crate::world_view::ContainerCommand::Swing,
        "the swing beat the shared tail rides"
    );
    assert!(rx.try_recv().is_err(), "nothing else the press speaks");
    // THE RESET the ticker's law speaks; THE HURT PREDICTION the
    // wire body never runs — the health the server's own
    assert_eq!(app.attack_ticker, 0, "the reset the walk speaks");
    assert_eq!(
        app.net_mobs[0].health, health_before,
        "the wire body's health the client never predicts"
    );
    // THE ECHO'S OWN VOICE (attackVisualEffects's branch the client
    // walk keeps — the staged ground hit the full charge the STRONG
    // pick names, the weak's absence the crit gate's refusal keeps,
    // the sweep's no-sword staging the branch never arms)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.attack.strong ")),
        "the full charge the strong voice speaks"
    );
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.attack.weak ")),
        "the non-crit ground hit the weak never speaks"
    );
    assert!(
        app.crit_emitters.is_empty(),
        "the staged ground hit the crit gate refuses"
    );
}

#[test]
fn the_wire_attack_the_sword_clock_the_meter_wears() {
    // THE WIRE ARM'S OWN CHARGE CLOCK (the getAttackStrengthScale
    // fold the offline arm wears — the delay the held item's own
    // attack speed drives: the iron sword's -2.4 modifier the 1.6
    // speed the 12.5-tick delay keeps — the /5.0 fist clock the
    // wire lane hardcoded the sword's windows a half-meter early
    // spoke: ticker 10 the fist clock 2.1 FULL called while the
    // sword's own meter 10.5/12.5 = 0.84 still rides — the weak
    // voice the strong spoke, the sweep arc the not-yet meter
    // flung). THE HALF-METER the weak voice answers, THE FULL
    // SWORD METER (ticker 12: 12.5/12.5 == 1.0) the strong the
    // exact boundary speaks.
    use crate::world_view::ContainerCommand;
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
    app.live_mobs_updated(vec![wire_mob_named(
        100,
        bx + 1.8,
        app.spawn_y as f64,
        bz,
        "minecraft:zombie",
    )]);
    // the iron sword the main hand holds — the wire-arm charge the
    // item's OWN clock reads (the wire name the same table the
    // held_id resolves)
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    app.inventory.items[app.selected_hotbar] = (sword, 1);
    app.hotbar_slots = app.inventory.hotbar();
    let (tx, rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    // THE HALF-METER PRESS (ticker 10 — 10.5/12.5 = 0.84 the ≤0.9
    // gate the weak voice speaks; the fist clock 2.1 the strong
    // would have named, the sweep arc flung)
    app.attack_ticker = 10;
    app.on_attack();
    assert_eq!(
        rx.try_recv().unwrap(),
        ContainerCommand::Attack { entity_id: 100 },
        "the wire press the packet rides regardless"
    );
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.attack.strong ")),
        "the sword's half-meter the strong never speaks"
    );
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.attack.sweep")),
        "the not-yet meter the sweep never arms"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.attack.weak ")),
        "the half-meter the weak voice speaks"
    );
    // THE FULL SWORD METER (ticker 12 — (12+0.5)/12.5 == 1.0 the
    // >0.9 gate arms — the full-charge sword ON THE GROUND the
    // doSweepAttack lane owns: the sweep voice the arc flings, the
    // strong the !crit && !sweep gate never speaks; the fist clock
    // the ticker 5 named, 7 ticks early)
    app.sounds.events.clear();
    app.attack_ticker = 12;
    app.on_attack();
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.attack.sweep")),
        "the sword's full meter the sweep flings"
    );
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.attack.strong ")),
        "the sweep lane the strong never speaks"
    );
    assert_eq!(app.attack_ticker, 0, "the reset the walk rides last");
}

#[test]
fn the_wire_attack_predicts_the_crit_emitter() {
    // THE CRIT ARM'S OWN PREDICTION (the 26.2 Player.attack's
    // `this.crit(target)` the repo's javap places BEFORE the hurt —
    // ParticleEngine.createTrackingEmitter the client-only lane the
    // remote target walks): the falling attacker the crit gate arms,
    // the emitter the SERVER'S OWN id tracks, the attack.crit voice
    // the branch speaks, the HURT the client never runs (the health
    // the server's own, the damage stars the server's sendParticles
    // walk keeps)
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let n = app.world_size as f64;
    let mut body =
        crate::body::PlayerBody::new(n / 2.0 + 0.5, app.spawn_y as f64, n / 2.0 + 0.5, -90.0, 0.0);
    // THE FALLING ATTACKER (the crit gate's own facts: the fall
    // distance, the ground's absence, the eye's dry air, the
    // sprint's absence)
    body.on_ground = false;
    body.fall_distance = 3.0;
    app.body = Some(body);
    let (bx, bz) = {
        let b = app.body.as_ref().unwrap();
        (b.x, b.z)
    };
    app.live_mobs_updated(vec![wire_mob_named(
        100,
        bx + 1.8,
        app.spawn_y as f64,
        bz,
        "minecraft:zombie",
    )]);
    let health_before = app.net_mobs[0].health;
    assert_eq!(app.pick_mob(&app.net_mobs), Some(0), "the wire verdict");
    let (tx, _rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    app.attack_ticker = 10; // the full charge the crit gate reads
    app.on_attack();
    // THE EMITTER (the server's own id the tracking walk reads, the
    // ctor's frozen box read the wire body's own bbox keeps)
    assert_eq!(app.crit_emitters.len(), 1, "the crit arm's emitter");
    assert_eq!(app.crit_emitters[0].target_id, 100, "the server's id");
    {
        let b = app.net_mobs[0].box_at();
        let (cx, cz) = ((b.0 + b.3) * 0.5, (b.2 + b.5) * 0.5);
        assert!(
            (app.crit_emitters[0].x - cx).abs() < 1e-9,
            "the ctor's center-x read: {} vs {cx}",
            app.crit_emitters[0].x
        );
        assert!(
            (app.crit_emitters[0].z - cz).abs() < 1e-9,
            "the ctor's center-z read: {} vs {cz}",
            app.crit_emitters[0].z
        );
    }
    // THE TRACKING WALK (the net list the fallback reads — the
    // moved body the emitter's refresh follows, the frozen fold the
    // old walk kept the fallback replaced)
    app.net_mobs[0].x += 0.5;
    app.update();
    assert!(
        (app.crit_emitters[0].x - app.net_mobs[0].x).abs() < 1e-9,
        "the emitter follows the wire body: {} vs {}",
        app.crit_emitters[0].x,
        app.net_mobs[0].x
    );
    // THE VOICE + THE HURT'S ABSENCE (the crit voice the branch
    // speaks, the health the server's own, the strong's absence the
    // crit branch's own pick keeps)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.attack.crit ")),
        "the crit voice the branch speaks"
    );
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.attack.strong ")),
        "the crit branch the strong never speaks"
    );
    assert_eq!(
        app.net_mobs[0].health, health_before,
        "the hurt the client never runs"
    );
    // the damage stars the server's sendParticles walk keeps — the
    // client's lane the emitter's own crit stars alone fill (the
    // Damage kind the provider's walk speaks, the emitter's Crit
    // kind the stars the tracking walk spawns)
    assert!(
        !app.particles
            .iter()
            .any(|p| matches!(p.kind, crate::particles::ParticleKind::Damage)),
        "the damage stars the server's sendParticles walk keeps"
    );
}

#[test]
fn the_wire_attack_echoes_the_sweep() {
    // THE SWEEP'S OWN PREDICTION (doSweepAttack's client walk the
    // sword's staging arms — the voice the offset-0 speaks, the arc
    // the count-0 sendParticles leaves at the body's own mid height
    // one forward block; the AoE's hurts the server's own verdict
    // keeps — the wire bodies' health the client never touches)
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
    app.live_mobs_updated(vec![wire_mob_named(
        100,
        bx + 1.8,
        app.spawn_y as f64,
        bz,
        "minecraft:zombie",
    )]);
    let health_before = app.net_mobs[0].health;
    assert_eq!(app.pick_mob(&app.net_mobs), Some(0), "the wire verdict");
    app.offline_held_updated(Some("item/wooden_sword"));
    let (tx, _rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    // the full charge the sword's OWN clock reads (the wooden
    // sword's 1.6 speed the 12.5-tick delay keeps — ticker 12:
    // 12.5/12.5 == 1.0 the >0.9 gate arms; the /5.0 fist clock the
    // old ticker 10 called full while the sword's meter 0.84 rode)
    app.attack_ticker = 12;
    app.on_attack();
    // THE SWEEP VOICE + THE ARC (the branch's own pair — the
    // strong's absence the sweep arm's None keeps, the crit the
    // ground staging refuses)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.attack.sweep ")),
        "the sweep voice the offset-0 speaks"
    );
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.player.attack.strong ")),
        "the sweep arm the strong never speaks"
    );
    assert!(
        app.particles
            .iter()
            .any(|p| matches!(p.kind, crate::particles::ParticleKind::Sweep)),
        "the arc the count-0 sendParticles leaves"
    );
    assert!(
        app.crit_emitters.is_empty(),
        "the ground staging the crit gate refuses"
    );
    assert_eq!(
        app.net_mobs[0].health, health_before,
        "the AoE hurts the server's own verdict keeps"
    );
}

#[test]
fn the_wire_body_answers_the_right_press() {
    // THE VANILLA startUseItem'S OWN WALK (the hitResult type ==
    // ENTITY case the switch speaks FIRST): the crosshair's verdict
    // lands the wire body, the flat Interact record the LpVec3
    // cursor carries rides the command lane — the cursor the hit
    // location minus the body's origin, the shift the
    // isShiftKeyDown speaks. The local prediction the wire body
    // never runs — the interaction the server's to resolve (the
    // SWING the result's shouldSwing speaks stays silent until the
    // echo rides), and the BLOCK case the switch never reaches.
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
    // the wire zombie ON the facing line (the eye's own ray the
    // pick walks), the SERVER'S OWN id 100 the offline counter
    // never mints
    app.live_mobs_updated(vec![wire_mob_named(
        100,
        bx + 1.8,
        app.spawn_y as f64,
        bz,
        "minecraft:zombie",
    )]);
    assert_eq!(
        app.pick_mob(&app.net_mobs),
        Some(0),
        "the wire body the verdict speaks (the offline lane sleeps)"
    );
    let (tx, rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    // THE PRESS: the entity lane consumes — the BLOCK case the
    // switch never routes on an entity hit
    assert!(
        app.on_right_press(0, 0),
        "the entity hit consumed the press"
    );
    match rx.try_recv().expect("the interact") {
        crate::world_view::ContainerCommand::Interact {
            entity_id,
            cursor,
            shift,
        } => {
            assert_eq!(entity_id, 100, "the server's own id the record carries");
            // THE CURSOR: the hit location minus the body's origin —
            // the ray along the facing line clips the box's near
            // face (the 1.8 standoff minus the half-width 0.3), the
            // eye's height the box's y span contains rides
            assert!(
                (cursor.0 - (-0.3)).abs() < 1e-9,
                "the near face the clip speaks, got {cursor:?}"
            );
            assert!(
                (cursor.1 - 1.62).abs() < 1e-4,
                "the eye's height the box contains, got {cursor:?}"
            );
            assert!(
                cursor.2.abs() < 1e-6,
                "the facing line the z keeps (the f32 trig's own residual), got {cursor:?}"
            );
            assert!(!shift, "the unpressed shift the record's own field keeps");
        }
        other => panic!("expected an interact, got {other:?}"),
    }
    assert!(
        rx.try_recv().is_err(),
        "nothing else the press speaks — no swing, the verdict the server's own"
    );
    // THE SHIFT: the sneak key the record's fourth field rides
    app.on_shift_down(true);
    assert!(app.on_right_press(0, 0), "the press consumed again");
    match rx.try_recv().expect("the shifted interact") {
        crate::world_view::ContainerCommand::Interact { shift, .. } => {
            assert!(shift, "the pressed shift the isShiftKeyDown speaks");
        }
        other => panic!("expected an interact, got {other:?}"),
    }
}

#[test]
fn the_offline_entity_pass_skips_the_block_lane() {
    // THE ENTITY CASE'S OWN FALL-THROUGH (the vanilla startUseItem's
    // walk): the offline body the pick speaks PASSes — the
    // ZOMBIE's own mob.interact table empty (the sheep's shear the
    // table's first entry — its own test speaks) — and the BLOCK
    // case the switch never reaches on an entity hit: the held
    // dirt the ray aims at a face BEHIND the body never places
    // (the vanilla never places through the body the crosshair
    // picked), the press the item lane's own PASS refuses.
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
    // the offline body IN the cell the straight-down ray walks (the
    // box the ray enters at the top face — 2.05 below the eye, the
    // block's own hit 4.0 the strict compare loses)
    app.mobs.push(crate::mob::Zombie::new(
        7,
        c as f64 + 0.5,
        top as f64,
        c as f64 + 0.5,
        3,
    ));
    assert_eq!(
        app.pick_mob(&app.mobs),
        Some(0),
        "the offline body the verdict speaks (the wire lane sleeps)"
    );
    let (tx, rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    assert!(
        !app.on_right_press(0, 0),
        "the entity PASS + the dirt's own PASS refuse the press"
    );
    assert!(
        rx.try_recv().is_err(),
        "no UseItemOn the entity hit ever routes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::AIR,
        "the placement cell the body's own verdict guards"
    );
    // THE BODY STEPS ASIDE: the pick misses, the BLOCK case speaks —
    // the same press the block lane's own placement answers
    app.mobs.clear();
    assert!(app.on_right_press(0, 0), "the block hit consumed the press");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::DIRT,
        "the held dirt landed the block case speaks"
    );
}

#[test]
fn the_tnt_prime_arm_wears_the_ignite_law() {
    // THE BLOCK-USE'S OWN WALK (javap TntBlock.useItemOn +
    // TntBlock.prime the 26.2 jar speaks): the held flint the item
    // gate arms, the tnt cell the prime walk wears — the block the
    // setBlock(AIR) clears, the PrimedTnt at (x+0.5, y, z+0.5) the
    // addFreshEntity speaks, the TNT_PRIMED voice the 1.0F/1.0F
    // pair plays, the press the CONSUME verdict carries.
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/flint_and_steel"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE SURFACE'S OWN SWAP (the ray the straight-down walk aims —
    // the surface cell the tnt block wears, the top face the clip
    // speaks)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::TNT);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let (tx, rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    assert!(
        app.on_right_press(0, 0),
        "the prime walk the press consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top - 1, c),
        crate::world::AIR,
        "the setBlock(AIR) the prime tail keeps"
    );
    assert_eq!(
        app.primed_tnts.len(),
        1,
        "the primed entity the spawn wears"
    );
    let t = &app.primed_tnts[0];
    assert_eq!(
        (t.x, t.y, t.z),
        (c as f64 + 0.5, top as f64 - 1.0, c as f64 + 0.5),
        "the (x+0.5, y, z+0.5) spot the prime law speaks"
    );
    assert_eq!(
        t.fuse,
        crate::mob::TNT_FUSE,
        "the 80-tick fuse the ctor wears"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.tnt.primed @")),
        "the TNT_PRIMED voice the prime walk plays"
    );
    assert!(
        rx.try_recv().is_ok(),
        "the UseItemOn echo the block press rides"
    );
    // THE NON-TNT CELL'S OWN PASS (the surface the flint's BLOCK-use
    // gate passes — the TNT arm the non-tnt cell refuses, the
    // PLACEMENT arm the item's own useOn walk the pass reaches: the
    // natural surface the sturdy below keeps, the fire the above
    // cell wears, the primed machine the walk never spawns)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/flint_and_steel"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE TARGET'S OWN CLEAR (the spawn flora the procedural lane
    // may plant — the deterministic target cell the honest air
    // keeps, the hit cell the solid ground the clip's own column
    // serves)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::STONE);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top, c, crate::world::AIR);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the flint's placement the press consumes"
    );
    assert!(
        app.primed_tnts.is_empty(),
        "no primed entity the non-tnt cell speaks"
    );
    // THE FIRE'S OWN CLEAR (the third block's own placement the
    // cell re-wears — the is_replaceable set the FIRE row never
    // speaks, the honest clear the stale flame the lane refuses)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top, c, crate::world::AIR);
    // THE TNT ITEM'S OWN PLACEMENT (the non-igniter held the item
    // gate refuses — the placement lane the tnt block item
    // answers, the same cell the fresh block wears)
    app.offline_held_updated(Some("item/tnt"));
    assert!(
        app.on_right_press(0, 0),
        "the placement the tnt block item consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::TNT,
        "the placed tnt the placement lane wears"
    );
    assert!(
        app.primed_tnts.is_empty(),
        "the placement the fuse machine never spawns"
    );
}

#[test]
fn the_flint_placement_the_world_lands() {
    // THE ITEM'S OWN USEON WALK (javap FlintAndSteelItem.useOn the
    // 26.2 jar speaks): the non-TNT cell the BLOCK-use pass rides,
    // the placement cell the entry face's normal keeps, the
    // canBePlacedAt gate the isAir + canSurvive arms wears, the
    // sound-then-set order the verdict serves, the FIRE the
    // above-stone cell wears.
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/flint_and_steel"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE STONE'S OWN STAGE (the rocket lane's own walk — the hit
    // cell the solid ground keeps, the target the honest air wears)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::STONE);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top, c, crate::world::AIR);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let (tx, rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    // THE DURABILITY'S OWN FOLD (the (id, count) stack the
    // hurtAndBreak walk never touches — the count the honest
    // no-op keeps)
    let flint_id = mc_registry::item_id_by_name("flint_and_steel").expect("the wire rides");
    let sel = app.selected_hotbar;
    app.inventory.items[sel] = (flint_id, 5);
    app.hotbar_slots = app.inventory.hotbar();
    // THE PLACEMENT: the SUCCESS verdict the press consumes
    assert!(
        app.on_right_press(0, 0),
        "the useOn SUCCESS the press consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::FIRE,
        "the setBlock(FIRE) the above-stone cell wears"
    );
    assert_eq!(
        app.inventory.items[sel].1, 5,
        "the hurtAndBreak the offline stack never counts"
    );
    assert!(
        app.world_quads.is_some(),
        "the mesh re-speak the world set bakes"
    );
    assert!(
        rx.try_recv().is_ok(),
        "the UseItemOn echo the block press rides"
    );
    // THE VOICE'S OWN ROW (the playSound the verdict's first fruit
    // speaks — the FLINTANDSTEEL_USE row the real table carries)
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("item.flintandsteel.use @")),
            "the FLINTANDSTEEL_USE voice the placement plays"
        );
    }
    // THE ISAIR ARM'S OWN REFUSAL (the second press the same column
    // aims — the flame cell the clip's Empty-collision row passes,
    // the stone top face the hit again serves, the non-air target
    // the FAIL keeps: the press the item lanes' PASS refuses)
    app.sounds.events.clear();
    assert!(
        !app.on_right_press(0, 0),
        "the non-air target the press never consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::FIRE,
        "the fire cell the refused press never clears"
    );
    // THE CANSURVIVE ARM'S OWN REFUSAL (the farmland the 15/16
    // column wears — the sturdy arm refuses, the stone-sided
    // neighborhood the burnable scan refuses: the FAIL the press
    // never consumes)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/flint_and_steel"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top - 1, c, crate::world::FARMLAND);
        // the six-way scan's honest emptiness (the spawn flora the
        // neighborhood may carry — the deterministic refusal the
        // cleared cells keep)
        w.set(c, top, c, crate::world::AIR);
        w.set(c, top + 1, c, crate::world::AIR);
        w.set(c, top, c - 1, crate::world::AIR);
        w.set(c, top, c + 1, crate::world::AIR);
        w.set(c - 1, top, c, crate::world::STONE);
        w.set(c + 1, top, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the farmland's no-odds cell the press never consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::AIR,
        "the refused fire the cell never wears"
    );
    // THE LOCATION ARM'S OWN ANSWER (the east log the burnable
    // odds speak — the farmland the sturdy arm refuses, the
    // six-way scan the log's ignite row answers, the fire lands)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c + 1, top, c, crate::world::LOG);
    assert!(
        app.on_right_press(0, 0),
        "the log's odds the placement consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::FIRE,
        "the location arm's fire the cell wears"
    );
}

/// THE FIRE DRAIN'S OWN WALK THE APP LANDS — the update loop's
/// scheduled-tick lane the drain site speaks: the placement's own
/// onPlace law the queue arms (the age-0 entry + the 30..=39 due),
/// the due tick the drain crosses (the walk the world's own law
/// answers), the burnt TNT the prime walk spawns (the PrimedTnt
/// the ctor's own center keeps, the fuse the entity's own clock
/// starts) and the TNT_PRIMED voice the BLOCKS source plays — the
/// chain the fire the player sets the offline world finally
/// carries.
#[test]
fn the_fire_drain_the_walk_the_app_lands() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE STAGE'S OWN DETERMINISM: the stone floor, the TNT
    // neighbor the burn-out lane reads, the open box the terrain's
    // own flora never perturbs
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        w.set(c + 1, top, c, crate::world::TNT); // the east neighbor the roll eats
    }
    // THE ONPLACE LAW'S OWN ARM (the placement arm's own shape):
    // the fire lands, the age-0 entry the store banks, the queue
    // the 30..=39 due wears
    let frame0 = app.frame;
    let due = {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c, crate::world::FIRE);
        w.fire_age.insert((c, top, c), 0);
        let d = frame0 + crate::world::fire_tick_delay(&mut app.fire_rng);
        w.schedule_fire_tick(d, c, top, c);
        assert!(
            d > frame0 + 29 && d < frame0 + 40,
            "the delay window the onPlace draw banks"
        );
        d
    };
    assert!(
        due > app.frame,
        "the due the future the first walk never crosses"
    );
    // THE DUE TICK'S OWN CROSSING: the updates the walk the drain
    // speaks (the re-arm the queue keeps — the flame the TNT's own
    // burnable odds holds, the isValid scan the neighbor answers)
    for _ in 0..45 {
        app.update();
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::FIRE,
            "the young flame the walk's own verdict keeps"
        );
        assert!(
            w.fire_ticks
                .iter()
                .any(|&(_, x, y, z)| (x, y, z) == (c, top, c)),
            "the re-arm the walk's own opening speaks"
        );
    }
    // THE PRIME'S OWN LANDING: the 100-odds roll the east neighbor
    // wears (1/3 per fire tick) — the drive the spawn answers, the
    // break the fuse's own crossing beats (the 80-tick clock the
    // boom never reaches)
    let mut primed = false;
    for _ in 0..900 {
        app.update();
        if !app.primed_tnts.is_empty() {
            primed = true;
            break;
        }
    }
    assert!(primed, "the burnt TNT the drain's own spawn wears");
    let t = &app.primed_tnts[0];
    assert!(
        t.fuse <= 80,
        "the fuse the ctor's own clock starts (the tick the same update wears)"
    );
    assert!(
        (t.x - (c as f64 + 1.5)).abs() < 0.5 && (t.z - (c as f64 + 0.5)).abs() < 0.5,
        "the prime's own center the ctor keeps (the same cell the entity lane's own
             ticks the accumulator ran may drift — the exact cell the world walk pins)"
    );
    assert!(
        t.y >= top as f64 - 0.1 && t.y < top as f64 + 0.3,
        "the ignite's own lift the first ticks wear"
    );
    // THE VOICE'S OWN ROW (the TNT_PRIMED row the BLOCKS source
    // serves — the 1.0F/1.0F pair the volume and the pitch keep)
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("entity.tnt.primed @")),
            "the TNT_PRIMED voice the drain's own spawn plays"
        );
    }
}

#[test]
fn the_fire_charge_placement_the_world_lands() {
    // THE ITEM'S OWN USEON WALK (javap FireChargeItem.useOn the
    // 26.2 jar speaks): the non-TNT cell the BLOCK-use pass rides,
    // the placement cell the entry face's normal keeps, the
    // canBePlacedAt gate the isAir + canSurvive arms wears, the
    // sound-then-set order the private playSound serves, the
    // shrink(1) the count walk keeps, the FIRE the above-stone
    // cell wears.
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/fire_charge"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE STONE'S OWN STAGE (the flint arm's own walk — the hit
    // cell the solid ground keeps, the target the honest air wears)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::STONE);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top, c, crate::world::AIR);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let (tx, rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    // THE CONSUMPTION'S OWN SEED (the (id, count) pair the shrink
    // walk answers)
    let charge_id = mc_registry::item_id_by_name("fire_charge").expect("the wire rides");
    let sel = app.selected_hotbar;
    app.inventory.items[sel] = (charge_id, 3);
    app.hotbar_slots = app.inventory.hotbar();
    // THE PLACEMENT: the SUCCESS verdict the press consumes
    assert!(
        app.on_right_press(0, 0),
        "the useOn SUCCESS the press consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::FIRE,
        "the setBlockAndUpdate(FIRE) the above-stone cell wears"
    );
    assert_eq!(
        app.inventory.items[sel],
        (charge_id, 2),
        "the shrink(1) the count walk keeps"
    );
    assert!(
        app.world_quads.is_some(),
        "the mesh re-speak the world set bakes"
    );
    assert!(
        rx.try_recv().is_ok(),
        "the UseItemOn echo the block press rides"
    );
    // THE VOICE'S OWN ROW (the private playSound the verdict's
    // first fruit speaks — the FIRECHARGE_USE row the real table
    // carries, the pitch the (r - r) * 0.2F + 1.0F fold the
    // rng-less replay keeps — the constant 1.0 the log's p field
    // wears)
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("item.firecharge.use @") && e.contains("p1.000000")),
            "the FIRECHARGE_USE voice the placement plays — the pitch the fold keeps"
        );
    }
    // THE ISAIR ARM'S OWN REFUSAL (the second press the same column
    // aims — the flame cell the clip's Empty-collision row passes,
    // the stone top face the hit again serves, the non-air target
    // the FAIL keeps: the press the item lanes' PASS refuses, the
    // count the shrink never touches)
    app.sounds.events.clear();
    assert!(
        !app.on_right_press(0, 0),
        "the non-air target the press never consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::FIRE,
        "the fire cell the refused press never clears"
    );
    assert_eq!(
        app.inventory.items[sel],
        (charge_id, 2),
        "the FAIL the shrink never touches"
    );
    // THE CANSURVIVE ARM'S OWN REFUSAL (the farmland the 15/16
    // column wears — the sturdy arm refuses, the stone-sided
    // neighborhood the burnable scan refuses: the FAIL the press
    // never consumes)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/fire_charge"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top - 1, c, crate::world::FARMLAND);
        // the six-way scan's honest emptiness (the spawn flora the
        // neighborhood may carry — the deterministic refusal the
        // cleared cells keep)
        w.set(c, top, c, crate::world::AIR);
        w.set(c, top + 1, c, crate::world::AIR);
        w.set(c, top, c - 1, crate::world::AIR);
        w.set(c, top, c + 1, crate::world::AIR);
        w.set(c - 1, top, c, crate::world::STONE);
        w.set(c + 1, top, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let charge_id = mc_registry::item_id_by_name("fire_charge").expect("the wire rides");
    let sel = app.selected_hotbar;
    app.inventory.items[sel] = (charge_id, 2);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(
        !app.on_right_press(0, 0),
        "the farmland's no-odds cell the press never consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::AIR,
        "the refused fire the cell never wears"
    );
    assert_eq!(
        app.inventory.items[sel],
        (charge_id, 2),
        "the refusal the count never touches"
    );
    // THE LOCATION ARM'S OWN ANSWER (the west log the burnable
    // odds speak — the farmland the sturdy arm refuses, the
    // six-way scan the log's ignite row answers, the fire lands)
    // + THE ZERO'S OWN SLOT (the count-1 stack the last shrink
    // empties — the (0, 0) pair the empty slot wears, the hotbar
    // the refresh re-speaks)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c - 1, top, c, crate::world::LOG);
    app.inventory.items[sel] = (charge_id, 1);
    app.hotbar_slots = app.inventory.hotbar();
    assert!(
        app.on_right_press(0, 0),
        "the log's odds the placement consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::FIRE,
        "the location arm's fire the cell wears"
    );
    assert_eq!(
        app.inventory.items[sel],
        (0, 0),
        "the last shrink the empty slot wears"
    );
}

#[test]
fn the_primed_tnt_tick_wears_the_boom_law() {
    // THE FUSE MACHINE'S OWN TICK (javap PrimedTnt.tick the 26.2
    // jar speaks — the `fuse <= 0` crossing the discard + explode
    // tail serves): the boom the same event consumer the creeper's
    // blast wears — the cells the silent law clears, the
    // entity.generic.explode voice the handleExplosion walk plays,
    // the entity the retain removes.
    let mut app = App::new(3);
    app.enter_world();
    let mut t = crate::mob::PrimedTnt::new(9, 8.5, 6.0, 8.5, 7);
    t.fuse = 1; // the crossing's own test seed — one tick to the boom
    app.primed_tnts.push(t);
    assert_eq!(app.primed_tnts[0].fuse, 1, "the test's own one-tick fuse");
    app.update();
    assert!(app.primed_tnts.is_empty(), "the discard the boom speaks");
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.generic.explode @")),
        "the boom voice the consumer speaks"
    );
}

#[test]
fn the_flower_placement_wears_the_can_survive_law() {
    // THE SUPPORT TAG'S OWN WALK (javap VegetationBlock.canSurvive +
    // mayPlaceOn the 26.2 jar speaks): the held dandelion the
    // grass cell places (the below cell the SUPPORTS_VEGETATION
    // subset answers), the stone cell the canSurvive refusal keeps
    // (the press the PASS verdict carries), the mined bloom the
    // item the loot row drops.
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/dandelion"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE GRASS SURFACE'S OWN SWAP (the hit cell the surface keeps —
    // the below the canSurvive gate answers)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the dandelion on grass the placement consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::DANDELION,
        "the placed bloom the placement lane wears"
    );
    // THE STONE'S OWN REFUSAL (the hit cell the stone surface keeps —
    // the below-cell read the SUPPORTS_VEGETATION subset refuses)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/dandelion"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::STONE);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the dandelion on stone the canSurvive refusal keeps"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::AIR,
        "the placement cell the refusal guards"
    );
    // THE MINE'S OWN DROP (the loot row the dig lane rolls — the
    // bloom the item the survives gate answers)
    let pools = mc_registry::generated::block_drops::drops("dandelion")
        .expect("the dandelion's own loot row");
    assert_eq!(pools.len(), 1, "the single pool the drop speaks");
    assert_eq!(pools[0].entries.len(), 1, "the self entry the pool keeps");
    assert_eq!(pools[0].entries[0].item, "dandelion", "the self drop");
    assert_eq!(pools[0].entries[0].chance, 1.0, "the definite pass");
    // THE PLACEABLE LOOKUP (the BlockItem the generic lane names)
    assert_eq!(
        crate::world::World::placeable_block("dandelion"),
        Some(crate::world::DANDELION),
        "the flower the placeable lookup names"
    );
}

#[test]
fn the_bamboo_the_item_the_sapling_the_stalk_wears() {
    // THE PLACEMENT LAW'S OWN E2E (javap BambooStalkBlock.
    // getStateForPlacement over the 26.2 jar — the three-way the
    // walk serves): the plain ground the SAPLING arm's own product
    // places (pc 126-184 the BAMBOO_SAPLING.defaultBlockState()
    // return the vanilla's own quirk pins — the bamboo item on
    // grass the SAPLING block places, the stalk the item's own
    // kind never is), the sapling-below the STALK arm serves
    // (pc 46-73 the AGE=0 default the arm keeps), the water target
    // the pc 0-20 fluid refusal keeps (the replaceable law would
    // admit, the bamboo's own law refuses).
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/bamboo"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE POOL'S OWN SETUP (the water cell the ray passes through —
    // the clip's own law the fluid ignores, the floor beneath the
    // hit the pool's own surface never decides; the column the
    // generated terrain never carries — the clear the assert's own
    // guard keeps)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c + 2, top - 1, c, crate::world::STONE);
        w.set(c + 2, top, c, crate::world::WATER);
        w.set(c + 2, top + 1, c, crate::world::AIR);
    }
    // PRESS 1: the plain ground the SAPLING arm serves (the
    // generated surface's own top the SUPPORTS_BAMBOO subset
    // answers — the grass/dirt rows the tag banks)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the bamboo on the plain ground the placement consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::BAMBOO_SAPLING,
        "the placed block the SAPLING arm's own product wears (the vanilla's own quirk)"
    );
    // PRESS 2: the sapling-below the STALK arm serves (the ray the
    // sapling's own cell hits — the clip predicate the non-air
    // cell answers, the clicked non-replaceable the normal-offset
    // target keeps, the below read the STALK arm's own verdict)
    app.offline_held_updated(Some("item/bamboo"));
    assert!(
        app.on_right_press(0, 0),
        "the bamboo on the sapling the placement consumes"
    );
    assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top + 1, c),
            crate::world::BAMBOO,
            "the placed block the STALK arm's own kind wears (the age the kind-only vocabulary never stores)"
        );
    // THE WATER'S OWN REFUSAL (pc 0-20 the target cell's fluid the
    // non-empty refusal keeps — the press the PASS verdict carries,
    // the water cell the water kind keeps)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the bamboo into the water the fluid refusal keeps"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 2, top, c),
        crate::world::WATER,
        "the water cell the refusal guards"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c + 2, top + 1, c),
        crate::world::AIR,
        "the pool's above the placement never fills"
    );
}

#[test]
fn the_tuft_placement_wears_the_replaceable_laws() {
    // THE REPLACEABLE PLANT'S OWN WALK (javap BlockPlaceContext +
    // VegetationBlock.canSurvive the 26.2 jar speaks — the
    // autopush-412 round): the held short_grass the grass cell
    // places (the below cell the SUPPORTS_VEGETATION subset
    // answers), the replaceable CLICKED cell the placement target
    // keeps (the `replaceClicked` read the `getClickedPos` walk
    // serves — the tuft gives way, the held block takes ITS cell,
    // the offset pos the non-replaceable keeps), the stone-below
    // refusal the canSurvive gate speaks, the loot rows the dig
    // lane rolls (the short_grass 1/8 wheat_seeds chance the
    // hand-lane keeps, the bush the shears-only gate never opens —
    // no drops).
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/stone"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE TUFT'S OWN CLICK-TARGET WALK (the replaceable clicked cell
    // the placement target keeps): a short_grass at the aim cell —
    // the placement rides the stone INTO the tuft's cell, the
    // normal-offset cell the law never touches (the vanilla
    // BlockPlaceContext.getClickedPos walk)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top, c, crate::world::SHORT_GRASS);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the placement into the tuft the press consumes"
    );
    // the STONE kind's own id (the NAME_OF walk the placeable
    // lookup speaks — the palette's second kind)
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::STONE,
        "the stone the tuft's own cell wears"
    );
    // THE TUFT'S OWN PLACEMENT ARM (the held short_grass the grass
    // below-cell places — the SUPPORTS_VEGETATION subset answers)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/short_grass"));
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
        "the short_grass on grass the placement consumes"
    );
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::SHORT_GRASS,
        "the placed tuft the placement lane wears"
    );
    // THE STONE'S OWN REFUSAL (the hit cell the stone surface keeps —
    // the below-cell read the canSurvive gate refuses)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/short_grass"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top - 1, c, crate::world::STONE);
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the short_grass on stone the canSurvive refusal keeps"
    );
    // THE LOOT'S OWN ROWS (the generated tables the dig lane rolls):
    // the short_grass 1/8 wheat_seeds chance the alternatives' else
    // arm keeps (the shears the hand lane never speaks), the bush
    // the shears/silk gate the no-drop verdict keeps
    let pools = mc_registry::generated::block_drops::drops("short_grass")
        .expect("the short_grass's own loot row");
    assert_eq!(pools.len(), 1, "the single pool the drop speaks");
    assert_eq!(pools[0].entries.len(), 1, "the else arm's single entry");
    assert_eq!(pools[0].entries[0].item, "wheat_seeds", "the seeds drop");
    assert_eq!(pools[0].entries[0].chance, 0.125, "the 1/8 chance");
    assert!(
        mc_registry::generated::block_drops::drops("bush").is_none(),
        "the bush's shears-only loot the hand lane never opens"
    );
    // THE PLACEABLE LOOKUP (the BlockItem the generic lane names)
    assert_eq!(
        crate::world::World::placeable_block("short_grass"),
        Some(crate::world::SHORT_GRASS),
        "the tuft the placeable lookup names"
    );
    assert_eq!(
        crate::world::World::placeable_block("bush"),
        Some(crate::world::BUSH),
        "the bush the placeable lookup names"
    );
}

#[test]
fn the_placement_the_burnable_swap_the_fire_douses() {
    // THE PLACEMENT'S OWN CHAIN (javap BlockItem.placeBlock pc 9
    // `bipush 11` — the setBlock the UPDATE_NEIGHBORS bit the
    // broadcast keeps): the tuft the placement's own swap the
    // burnable loss wears — the fire cell the canBurn scan the
    // tuft held the SAME beat douses (the flag-11 broadcast the
    // updateShape walk the vanilla serves; the scheduled tick's
    // 30..=39 lag the vanilla never waits)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/stone"));
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE STAGE: the tuft the aim cell keeps, the fire its ONLY
    // burnable neighbor holds (the below cell the air the sturdy
    // arm refuses), the far fire the sturdy floor the chain's
    // reach never crosses
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -2..=2 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        w.set(c, top - 1, c + 1, crate::world::AIR); // the fire's own below the air
        w.set(c, top, c, crate::world::SHORT_GRASS); // the tuft the scan holds
        w.set(c, top, c + 1, crate::world::FIRE);
        w.fire_age.insert((c, top, c + 1), 0);
        w.set(c - 8, top, c - 8, crate::world::FIRE); // the far fire on the stone floor
        w.fire_age.insert((c - 8, top, c - 8), 0);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the placement into the tuft the press consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::STONE,
            "the stone the tuft's own cell wears"
        );
        assert_eq!(
            w.get(c, top, c + 1),
            crate::world::AIR,
            "the fire the burnable's swap the chain douses — the same beat"
        );
        assert!(
            !w.fire_age.contains_key(&(c, top, c + 1)),
            "the dead cell's age entry the chain drops"
        );
        assert_eq!(
            w.get(c - 8, top, c - 8),
            crate::world::FIRE,
            "the far fire the chain's reach never crosses"
        );
    }
}

#[test]
fn the_flint_consult_the_candle_lights() {
    // THE CANLIGHT CONSULT'S OWN LIT BRANCH (javap
    // FlintAndSteelItem.useOn pc 24-45 over the 26.2 jar — the
    // CandleBlock.canLight arm the consult answers FIRST, the
    // branch pc 48-134 the SUCCESS verdict carries): the clicked
    // candle the LIT swap the kind-only vocabulary the CANDLE_LIT
    // half-kind word answers — the sound the branch's first fruit
    // speaks, the set the flag-11 broadcast the chain call wears,
    // the placement cell the branch never touches (the fire the
    // placement walk would set beside the candle the consult's
    // consume keeps absent)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/flint_and_steel"));
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
        w.set(c, top, c, crate::world::CANDLE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the lit branch the press consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::CANDLE_LIT,
            "the lit kind the setValue walk's own swap wears"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::AIR,
            "the placement cell the branch never touches — no fire beside"
        );
    }
}

#[test]
fn the_fire_charge_consult_the_same_beat() {
    // THE CHARGE'S OWN CONSULT (javap FireChargeItem.useOn pc 20-41
    // over the 26.2 jar — the SAME three-way canLight disjunction
    // the flint's consult speaks, the branch pc 44-86 the
    // setBlockAndUpdate flag-3 arm keeps): the clicked candle the
    // SAME lit swap the charge's walk wears
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/fire_charge"));
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
        w.set(c, top, c, crate::world::CANDLE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the charge's lit branch the press consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::CANDLE_LIT,
            "the lit kind the setBlockAndUpdate walk wears"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::AIR,
            "the placement cell the branch never touches"
        );
    }
}

#[test]
fn the_lit_candle_the_consult_refuses() {
    // THE !LIT VERDICT'S OWN REFUSAL (javap CandleBlock.canLight —
    // the tag consult the LIT row's own state refuses, the
    // fall-through the placement walk answers): the lit candle the
    // flint's consult NEVER relights, the placement's own
    // canBePlacedAt gate the survival law refuses (the candle cell
    // the below read the Empty collision keeps, the neighbors the
    // canBurn scan refuses) — the press the PASS verdict carries,
    // the world the same state keeps
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/flint_and_steel"));
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
        w.set(c, top, c, crate::world::CANDLE_LIT);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the refused consult the PASS verdict carries"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::CANDLE_LIT,
            "the lit kind the press never touches"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::AIR,
            "no fire the refused placement keeps"
        );
    }
}

#[test]
fn the_canlight_consult_the_campfire_lights() {
    // THE CAMPFIRE ROW'S OWN ANSWER (the dev-6 round — javap
    // CampfireBlock.canLight: the CAMPFIRES tag + `!LIT` verdict
    // the unlit kind answers, the branch the SAME walk the
    // candle row wears): the clicked campfire the LIT swap the
    // can_light_product mapping carries — the sound the branch's
    // first fruit speaks, the set the flag-11 broadcast the chain
    // call wears, the placement cell the branch never touches
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/flint_and_steel"));
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
        w.set(c, top, c, crate::world::CAMPFIRE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the lit branch the press consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::CAMPFIRE_LIT,
            "the lit kind the setValue walk's own swap wears"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::AIR,
            "the placement cell the branch never touches — no fire beside"
        );
    }
}

#[test]
fn the_charge_consult_the_soul_campfire_lights() {
    // THE SOUL ROW'S OWN ANSWER (the same consult — the
    // FireChargeItem arm the CAMPFIRES family's soul kind reads):
    // the clicked soul campfire the SOUL_CAMPFIRE_LIT swap the
    // flag-3 broadcast the charge's own walk wears
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/fire_charge"));
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
        w.set(c, top, c, crate::world::SOUL_CAMPFIRE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the lit branch the press consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::SOUL_CAMPFIRE_LIT,
            "the soul lit kind the setValue walk's own swap wears"
        );
    }
}

#[test]
fn the_canlight_consult_the_candle_cake_lights() {
    // THE THIRD ROW'S OWN ANSWER (the autopush-523 round — javap
    // CandleCakeBlock.canLight over the 26.2 jar: the CANDLE_CAKES
    // tag + the `hasProperty(LIT) && !getValue(LIT)` predicate the
    // lambda$canLight$0 walk speaks — the unlit kind the consult
    // answers, the branch the SAME walk the candle/campfire rows
    // wear): the clicked candle cake the LIT swap the
    // can_light_product mapping carries — the sound the branch's
    // first fruit speaks, the set the flag-11 broadcast the chain
    // call wears, the placement cell the branch never touches
    // (no fire beside the cake the consult's consume keeps)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/flint_and_steel"));
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
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the lit branch the press consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::CANDLE_CAKE_LIT,
            "the lit kind the setValue walk's own swap wears"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::AIR,
            "the placement cell the branch never touches — no fire beside"
        );
    }
}

#[test]
fn the_lit_candle_cake_the_consult_refuses() {
    // THE !LIT VERDICT'S OWN REFUSAL (javap CandleCakeBlock.
    // canLight — the predicate the LIT row's own state refuses,
    // the fall-through the placement walk answers): the lit cake
    // the flint's consult NEVER relights, the placement's own
    // canBePlacedAt gate the survival law refuses (the cake cell
    // the Column(8) collision the below read answers, the
    // neighbors the canBurn scan refuses) — the press the PASS
    // verdict carries, the world the same state keeps
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/flint_and_steel"));
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
        w.set(c, top, c, crate::world::CANDLE_CAKE_LIT);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the refused consult the PASS verdict carries"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::CANDLE_CAKE_LIT,
            "the lit kind the press never touches"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::AIR,
            "no fire the refused placement keeps"
        );
    }
}

#[test]
fn the_empty_hand_the_lit_candle_cake_douses() {
    // THE DOUSE ARM'S OWN WALK (javap CandleCakeBlock.useItemOn
    // pc 24-67 + AbstractCandleBlock.extinguish pc 0-62 over the
    // 26.2 jar): the EMPTY main hand the stack.isEmpty() read
    // answers, the upper-half hit the candleHit read's 0.5
    // verdict speaks (the top-face press the cell's own top the
    // y-frac 1.0 wears), the LIT kind the verdict answers: the
    // CANDLE_CAKE kind the setLit walk's swap wears, the
    // flag-11 chain call the fire_shape_updates walk serves, the
    // CANDLE_EXTINGUISH voice the extinguish walk plays AFTER
    // the set (the bytecode's own order the cake-add arm's
    // voice-first beat departs), the SUCCESS the press consumes
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
        w.set(c, top, c, crate::world::CANDLE_CAKE_LIT);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the douse the press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::CANDLE_CAKE,
            "the setLit walk the unlit kind wears"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.candle.extinguish")),
        "the CANDLE_EXTINGUISH voice the log wears"
    );
    // THE SMOKE PUFF'S OWN WALK (the lambda$extinguish$0's
    // addParticle the client's own lane speaks — the wick top the
    // position the exact spawn keeps, the speed args the ctor
    // chain's own velocity fold rides)
    assert_eq!(
        app.particles.len(),
        1,
        "the wick's own puff the douse spawns"
    );
    let puff = &app.particles[0];
    assert_eq!(puff.kind, crate::particles::ParticleKind::Smoke);
    assert_eq!(puff.x, c as f64 + 0.5, "the wick top's own x");
    assert_eq!(puff.y, top as f64 + 1.0, "the wick top's own y");
    assert_eq!(puff.z, c as f64 + 0.5, "the wick top's own z");
    // the speed args (0, 0.1, 0) the ctor's spread-fold rides —
    // the normalized spread's own band the vy neighborhood keeps
    assert!(
        puff.vy > 0.05 && puff.vy < 0.15,
        "the rising drift the speed arg's own band keeps: {}",
        puff.vy
    );
}

#[test]
fn the_lower_half_press_the_douse_refuses() {
    // THE CANDLEHIT READ'S OWN PRECISION (javap pc 24-29 — the
    // 0.5 the dcmpl verdict keeps): the side press the slab
    // band's own y-frac 0.25 the upper-half verdict refuses —
    // the douse never speaks, the TRY_WITH_EMPTY_HAND verdict
    // the super walk's own default returns (the useWithoutItem
    // lane the CakeBlock.eat + the dropResources candle drop a
    // future round wears), the press the fall-through keeps,
    // the lit kind the world keeps, no voice the log never
    // banks
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -4..=4 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        w.set(c, top, c, crate::world::CANDLE_CAKE_LIT);
    }
    // THE SIDE PRESS (the -x look the yaw -pi/2 the forward walk
    // speaks): the ray the slab band's 0.25 row enters — the
    // y-frac the 0.5 floor the verdict refuses
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 3.5, top as f32 + 0.25, c as f32 + 0.5),
        yaw: -std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the refused douse the press never consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::CANDLE_CAKE_LIT,
            "the lit kind the press keeps"
        );
    }
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.candle.extinguish")),
        "no extinguish voice the refused verdict keeps"
    );
}

#[test]
fn the_unlit_candle_cake_the_douse_refuses() {
    // THE LIT VERDICT'S OWN REFUSAL (javap pc 39-52 — the
    // Boolean.booleanValue false the branch never takes): the
    // unlit kind the douse never speaks — the press the
    // fall-through keeps (the eat lane the candle-cake kinds a
    // future round wears), the unlit kind the world keeps
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
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        !app.on_right_press(0, 0),
        "the refused douse the press never consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::CANDLE_CAKE,
            "the unlit kind the press keeps"
        );
    }
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.candle.extinguish")),
        "no extinguish voice the refused verdict keeps"
    );
    assert!(
        app.particles.is_empty(),
        "the refused douse the puff never spawns"
    );
}

#[test]
fn the_empty_hand_the_candle_cake_eats() {
    // THE EAT FALL-THROUGH'S OWN WALK (javap CandleCakeBlock.
    // useWithoutItem pc 0-33 over the 26.2 jar): the douse
    // consult's refusal the useWithoutItem lane serves — the
    // CakeBlock.eat static the FRESH-CAKE state wears (the canEat
    // gate the hungry body answers, the FoodData.eat(2, 0.1F) the
    // bite's own feed, the bites 0->1 branch the CAKE_SLICE1 kind
    // the flag-3 setBlock wears), the consumesAction verdict the
    // dropResources tail serves (the candle item the loot table's
    // own row the item entity the popResource law spawns)
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
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.drops.is_empty(), "the lane starts empty");
    app.food_data.set_food_level(6);
    assert!(app.on_right_press(0, 0), "the eat lane the press consumes");
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(c, top, c),
        crate::world::CAKE_SLICE1,
        "the fresh-CAKE state the bites 0->1 ladder wears"
    );
    assert_eq!(app.food_data.food_level(), 8, "the +2 feed the bar wears");
    // THE DROP'S OWN TAIL (the candle item the loot table's own
    // row the item entity the popResource law spawns)
    assert_eq!(app.drops.len(), 1, "the candle drop the tail spawns");
    let name = crate::world::NAME_OF[crate::world::CANDLE_CAKE as usize];
    let pools = mc_registry::generated::block_drops::drops(name)
        .expect("the candle cake's loot row exists");
    let expected = mc_registry::item_id_by_name(pools[0].entries[0].item).unwrap();
    assert_eq!(
        app.drops[0].item, expected,
        "the drop rides the table's own row"
    );
    assert_eq!(app.drops[0].count, 1);
}

#[test]
fn the_lit_candle_cake_the_lower_half_eats() {
    // THE FALL-THROUGH'S OWN PRECISION (javap pc 24-29 the
    // candleHit read the 0.5 verdict keeps, the useWithoutItem
    // lane the refused douse alone serves): the side press the
    // slab band's 0.25 y-frac the douse verdict refuses — the
    // hungry body the eat walk answers: the CAKE_SLICE1 kind the
    // fresh-CAKE state wears, the +2 feed the bar carries, the
    // candle drop the consumesAction tail spawns, the
    // extinguish voice the refused douse never banks
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -4..=4 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        w.set(c, top, c, crate::world::CANDLE_CAKE_LIT);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 3.5, top as f32 + 0.25, c as f32 + 0.5),
        yaw: -std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    app.food_data.set_food_level(6);
    assert!(
        app.on_right_press(0, 0),
        "the eat walk the refused douse serves"
    );
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(c, top, c),
        crate::world::CAKE_SLICE1,
        "the fresh-CAKE state the bites 0->1 ladder wears"
    );
    assert_eq!(app.food_data.food_level(), 8, "the +2 feed the bar wears");
    assert_eq!(app.drops.len(), 1, "the candle drop the tail spawns");
    assert!(
        !app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.candle.extinguish")),
        "no extinguish voice the eat walk banks"
    );
}

#[test]
fn the_hungry_body_the_douse_the_eat_precedes() {
    // THE DOUSE ARM'S OWN PRECEDENCE (javap the useItemOn walk the
    // useWithoutItem lane the dispatch's own order precedes): the
    // upper-half press the candleHit verdict answers FIRST — the
    // extinguish walk the hungry body never denies, the eat lane
    // the press the douse's SUCCESS alone withholds: the unlit
    // kind the setLit swap wears, the bar the bite never feeds,
    // the candle drop the eat's tail never spawns
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
        w.set(c, top, c, crate::world::CANDLE_CAKE_LIT);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    app.food_data.set_food_level(6);
    assert!(app.on_right_press(0, 0), "the douse the press consumes");
    let w = app.offline_world.as_ref().unwrap();
    assert_eq!(
        w.get(c, top, c),
        crate::world::CANDLE_CAKE,
        "the setLit walk the unlit kind wears"
    );
    assert_eq!(
        app.food_data.food_level(),
        6,
        "the douse the bar never feeds"
    );
    assert!(
        app.drops.is_empty(),
        "the eat tail the douse's SUCCESS withholds"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.candle.extinguish")),
        "the CANDLE_EXTINGUISH voice the log wears"
    );
}

#[test]
fn the_campfire_the_ambient_walk_the_signal_puffs_wear() {
    // THE PARTICLE TICK'S OWN WALK (javap CampfireBlockEntity.
    // particleTick over the 26.2 jar — the 11% gate the 2..3 puff
    // loop the SIGNAL flag the hay-below verdict rides): the
    // stream replay the fixed seed pins bit-for-bit — the gate
    // draw, the loop's re-rolled bounds, the bodies' 11-draw
    // chains, the SIGNAL provider's force arm the helper banks.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top - 1, c, crate::world::HAY_BLOCK);
        w.set(c, top, c, crate::world::CAMPFIRE_LIT);
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        assert!(
            w.is_signal_fire(c, top, c),
            "the hay below the signal verdict answers"
        );
        assert_eq!(
            w.campfire_ticks,
            vec![(0, c, top, c)],
            "the lit transition the queue entry banks"
        );
    }
    // THE SIGNAL REPLAY (seed 0xbeef: the gate draw 0.0847 the 11%
    // share passes, the third check draw 1 the third body grants —
    // THREE puffs, the lifetimes 280 + [3, 42, 39] the SIGNAL
    // ladder wears)
    app.frame = 100;
    app.level_ambient_rng = crate::drops::Lcg::new(0xbeef);
    app.tick_campfire_ambient();
    let signal_puffs: Vec<&crate::particles::Particle> = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::CampfireSmoke { .. }))
        .collect();
    assert_eq!(
        signal_puffs.len(),
        3,
        "the seed's own replay the third puff grants"
    );
    let pinned_lifetimes: Vec<u32> = signal_puffs.iter().map(|p| p.lifetime).collect();
    assert_eq!(
        pinned_lifetimes,
        vec![283, 322, 319],
        "the SIGNAL ladder 280 + [3, 42, 39]"
    );
    for p in &signal_puffs {
        match p.kind {
            crate::particles::ParticleKind::CampfireSmoke { alpha, .. } => {
                assert_eq!(alpha, 0.95, "the SIGNAL provider's own alpha");
            }
            _ => unreachable!(),
        }
        // the makeParticles COSY half's own position bands (the
        // /3.0 folds the cell center keeps, the y the
        // nextDouble+nextDouble band rides)
        assert!(
            (p.x - (c as f64 + 0.5)).abs() <= 1.0 / 3.0,
            "the x band the /3.0 fold keeps"
        );
        assert!(
            (p.z - (c as f64 + 0.5)).abs() <= 1.0 / 3.0,
            "the z band the /3.0 fold keeps"
        );
        assert!(
            (top as f64..top as f64 + 2.0).contains(&p.y),
            "the y band the double roll rides"
        );
        // the vy the makeParticles arg carries + the ctor's /500
        // bump (0.07 + [0, 0.002))
        assert!(
            (0.07..0.072).contains(&p.vy),
            "the vy the 0.07 arg the bump keeps"
        );
    }
    // THE RE-ARM'S OWN CADENCE: the due+1 entry the walk banks,
    // the one-live-entry law the duplicate refuses
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.campfire_ticks,
            vec![(101, c, top, c)],
            "the due+1 the re-arm banks"
        );
    }
    // the not-yet-due entry the earlier walk keeps (the every-tick
    // walk the future entry leaves)
    app.frame = 100;
    app.tick_campfire_ambient();
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.campfire_ticks,
            vec![(101, c, top, c)],
            "the future entry the drain keeps"
        );
    }
    // THE COSY ARM'S OWN REPLAY (the hay removal the verdict
    // re-answers — the live read the property cache's own law
    // keeps; seed 0xbeef: the gate 0.0847 passes, the third check
    // draw 1 the third body grants, the lifetimes 80 + [3, 42, 39]
    // the COSY ladder wears)
    app.particles.clear();
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top - 1, c, crate::world::AIR);
    }
    app.frame = 200;
    app.level_ambient_rng = crate::drops::Lcg::new(0xbeef);
    app.tick_campfire_ambient();
    {
        let w = app.offline_world.as_ref().unwrap();
        assert!(
            !w.is_signal_fire(c, top, c),
            "the hay removal the verdict refuses"
        );
    }
    let cosy_puffs: Vec<&crate::particles::Particle> = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::CampfireSmoke { .. }))
        .collect();
    assert_eq!(
        cosy_puffs.len(),
        3,
        "the seed's own replay the third puff grants"
    );
    let cosy_lifetimes: Vec<u32> = cosy_puffs.iter().map(|p| p.lifetime).collect();
    assert_eq!(
        cosy_lifetimes,
        vec![83, 122, 119],
        "the COSY ladder 80 + [3, 42, 39]"
    );
    for p in &cosy_puffs {
        match p.kind {
            crate::particles::ParticleKind::CampfireSmoke { alpha, .. } => {
                assert_eq!(alpha, 0.9, "the COSY provider's own alpha");
            }
            _ => unreachable!(),
        }
        assert!(
            (80..=129).contains(&p.lifetime),
            "the COSY band the ladder keeps"
        );
    }
    // the re-arm the next cadence tick banks
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.campfire_ticks,
            vec![(201, c, top, c)],
            "the due+1 the re-arm banks"
        );
    }
}

#[test]
fn the_campfire_the_crackle_the_animate_gate_wears() {
    // THE CRACKLE'S OWN ANIMATE LANE (javap CampfireBlock.animateTick
    // over the 26.2 jar — the block display tick's own walk the BE
    // ticker's smoke walk precedes, the vanilla phase order the
    // level tick speaks): the crackle arm pc 17-26 the
    // `nextInt(10) == 0` gate every tick rolls, pc 29-92 the
    // positioned voice the BLOCKS source plays — the volume the
    // `0.5f + nextFloat()` fold pc 63-73, the pitch the
    // `nextFloat() * 0.7f + 0.6f` fold pc 74-88, the draw order the
    // gate the two folds keeps (the folds the gate hit alone
    // spends). THE SEED LAW the replay pins: the smoke arm's draws
    // the stream first spends, the crackle's the tail rides — the
    // gate-hit walk seed 45 the exact folds the log wears (the
    // raw 0.5+f the 0.702055 the distance-zero gain keeps, the
    // pitch the 1.093107 fold), the seed 20 the volume the 1.218
    // fold the attenuate's own min() the 1.0 cap speaks, the seed
    // 2 the gate draw 6 the silence keeps, the seed 0xbeef the
    // smoke pins the crackle gate 7 the tail never disturbs.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c, crate::world::CAMPFIRE_LIT);
    }
    // THE GATE'S OWN HIT (seed 45: the smoke gate draw the 11%
    // share refuses — zero puffs — the crackle gate draw 0 the
    // voice the tail speaks, the bit-for-bit folds the log wears)
    app.frame = 100;
    app.level_ambient_rng = crate::drops::Lcg::new(45);
    app.tick_campfire_ambient();
    assert!(
        app.particles
            .iter()
            .all(|p| !matches!(p.kind, crate::particles::ParticleKind::CampfireSmoke { .. })),
        "the smoke gate the 11% share refuses"
    );
    let crackle_lines: Vec<&String> = app
        .sounds
        .events
        .iter()
        .filter(|e| e.starts_with("block.campfire.crackle @"))
        .collect();
    assert_eq!(
        crackle_lines.len(),
        1,
        "the gate hit the ONE voice speaks: {:?}",
        app.sounds.events
    );
    assert!(
        crackle_lines[0].ends_with(" v0.702055 p1.093107"),
        "the bit-for-bit folds the volume/pitch log wears: {}",
        crackle_lines[0]
    );
    // THE CAP'S OWN LAW (seed 20: the volume fold 0.5+f the 1.218
    // the gain the min() the 1.0 ceiling speaks — the positioned
    // voice's own fold the distance-zero linear keeps)
    app.frame = 200;
    app.level_ambient_rng = crate::drops::Lcg::new(20);
    app.tick_campfire_ambient();
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.campfire.crackle @200 ")
                && e.ends_with(" v1.000000 p0.623867")),
        "the volume cap the gain fold wears: {:?}",
        app.sounds.events.last()
    );
    // THE GATE'S OWN MISS (seed 2: the crackle gate draw 6 the
    // silence the no-draw walk keeps — no third voice the count
    // law pins, the smoke the same refusal wears)
    app.frame = 300;
    app.level_ambient_rng = crate::drops::Lcg::new(2);
    app.tick_campfire_ambient();
    assert_eq!(
        app.sounds
            .events
            .iter()
            .filter(|e| e.starts_with("block.campfire.crackle @"))
            .count(),
        2,
        "the miss the count keeps"
    );
    // THE SOUL CAMPFIRE'S OWN ARM (the LIT gate pc 0-16 the only
    // verdict the bytecode speaks — the soul kind the same crackle
    // the same folds plays)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c, crate::world::SOUL_CAMPFIRE_LIT);
    }
    app.frame = 400;
    app.level_ambient_rng = crate::drops::Lcg::new(45);
    app.tick_campfire_ambient();
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.campfire.crackle @400 ")
                && e.ends_with(" v0.702055 p1.093107")),
        "the soul kind the same law wears"
    );
}

#[test]
fn the_campfire_the_lava_arm_the_spawn_particles_gate_wears() {
    // THE LAVA-PARTICLE'S OWN ANIMATE TAIL (javap
    // CampfireBlock.animateTick pc 93-192 over the 26.2 jar): the
    // `spawnParticles` field the CONSTRUCTOR arg the regular kind
    // alone lights (the (true, 1) lambda$static$310 — the soul the
    // (false, 2) lambda$static$311 the field never speaks), pc
    // 100-108 the `nextInt(5) == 0` gate, pc 111-126 the loop the
    // `i < nextInt(1) + 1` bound keeps — the body ONCE, the
    // condition twice. THE SEED LAW the replay pins: seed 3 the
    // smoke gate 0.662262 the 11% share refuses, the crackle gate
    // 8 the silence keeps, the lava gate 0 the arm fires — the
    // 11-draw walk the stream tail pins; seed 2 the lava gate 3
    // the miss keeps (three draws the tick spends); the soul kind
    // the same seed the short-circuit the gate draw never rolls
    // (two draws the tick spends).
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c, crate::world::CAMPFIRE_LIT);
    }
    // THE GATE'S OWN HIT (seed 3: the one-iteration loop the ONE
    // blob the cell center wears — the position the +0.5 folds
    // speak, the speeds the call site spent the provider dropped,
    // the ctor's four draws the lava_drip helper replays)
    app.frame = 100;
    app.level_ambient_rng = crate::drops::Lcg::new(3);
    app.tick_campfire_ambient();
    let blobs: Vec<&crate::particles::Particle> = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Lava))
        .collect();
    assert_eq!(
        blobs.len(),
        1,
        "the loop the one-iteration law keeps: {}",
        app.particles.len()
    );
    // the pinned fields the fresh walk's own helper replay answers
    let mut r = crate::drops::Lcg::new(3);
    let _smoke_gate = r.next_f32();
    let _crackle_gate = r.next_int(10);
    let _lava_gate = r.next_int(5);
    let _cond_entry = r.next_int(1);
    let _xs = r.next_f32() / 2.0f32;
    let _zs = r.next_f32() / 2.0f32;
    let r_base = r.next_f32();
    let r_yd = r.next_f32();
    let r_size = r.next_f32();
    let r_life = r.next_f32();
    let expected = crate::particles::lava_drip(
        c as f64 + 0.5,
        top as f64 + 0.5,
        c as f64 + 0.5,
        r_base,
        r_yd,
        r_size,
        r_life,
    );
    let b = blobs[0];
    assert_eq!(
        (b.x, b.y, b.z),
        (expected.x, expected.y, expected.z),
        "the cell-center spawn the +0.5 folds wear"
    );
    assert_eq!(b.vy, expected.vy, "the yd fold r*0.4+0.05");
    assert_eq!(b.quad_size, expected.quad_size, "the size-mul fold");
    assert_eq!(b.lifetime, expected.lifetime, "the 16.0/(r*0.8+0.2) fold");
    assert_eq!(b.gravity, 0.75, "the lava blob's own gravity");
    assert_eq!(b.friction, 0.999, "the lava blob's own friction");
    // THE STREAM TAIL the total-draw count pins (11 raw draws: the
    // three gates + the two condition spends + the two speed
    // folds + the ctor's four — the probe the 12th draw answers)
    let mut probe = crate::drops::Lcg::new(3);
    for _ in 0..11 {
        probe.next();
    }
    assert_eq!(
        app.level_ambient_rng.next_f32(),
        probe.next_f32(),
        "the 11-draw walk the tail pins"
    );
    // THE GATE'S OWN MISS (seed 2: the lava gate draw 3 the
    // silence keeps — no blob, the three-gate walk the tail pins)
    app.frame = 200;
    app.level_ambient_rng = crate::drops::Lcg::new(2);
    app.tick_campfire_ambient();
    let blobs_after: usize = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Lava))
        .count();
    assert_eq!(blobs_after, 1, "the miss the count keeps (no second blob)");
    let mut probe2 = crate::drops::Lcg::new(2);
    for _ in 0..3 {
        probe2.next();
    }
    assert_eq!(
        app.level_ambient_rng.next_f32(),
        probe2.next_f32(),
        "the miss the three-draw walk pins"
    );
    // THE SOUL KIND'S OWN SHORT-CIRCUIT (the spawnParticles field
    // false — the gate draw the stream never spends, the two-gate
    // walk the tail pins)
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c, top, c, crate::world::SOUL_CAMPFIRE_LIT);
    }
    app.frame = 300;
    app.level_ambient_rng = crate::drops::Lcg::new(3);
    app.tick_campfire_ambient();
    let blobs_soul: usize = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Lava))
        .count();
    assert_eq!(blobs_soul, 1, "the soul field the arm never lights");
    let mut probe3 = crate::drops::Lcg::new(3);
    for _ in 0..2 {
        probe3.next();
    }
    assert_eq!(
        app.level_ambient_rng.next_f32(),
        probe3.next_f32(),
        "the soul's two-draw walk the tail pins"
    );
}

#[test]
fn the_shovel_the_lit_campfire_douses() {
    // THE DOUSE ARM'S OWN WALK (javap ShovelItem.useOn pc 95-230
    // over the 26.2 jar — the `instanceof CampfireBlock` + LIT
    // verdict the lit kind answers: the levelEvent(1009) the
    // FIRE_EXTINGUISH row speaks, the `setValue(LIT, false)` the
    // unlit kind the douse's own product carries, the flag-11 set
    // the chain call wears, the SUCCESS pc 227-230 the press
    // consumes). THE DOWN-FACE GATE pc 17-24 the shared gate the
    // flatten arm's own face test keeps (the acmpeq the underside
    // PASS wears — the clicked underside neither flattens nor
    // douses)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/diamond_shovel"));
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
        w.set(c, top, c, crate::world::CAMPFIRE_LIT);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the douse arm the press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::CAMPFIRE,
            "the unlit kind the setValue walk's own product wears"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.fire.extinguish @")),
        "the levelEvent(1009) the FIRE_EXTINGUISH row the douse speaks: {:?}",
        app.sounds.events.last()
    );
    // THE DOWSE'S OWN SMOKE COLUMN (the 20-call loop the
    // makeParticles walk serves — the COSY half the banked kind
    // spawns the big_smoke frame the ascending list picks, the y
    // the +nextDouble+nextDouble band keeps, the vy the 0.07 arg
    // + the /500 bump carries; the smoke half the y +0.4 the
    // exact row keeps, the x/z the ±0.25 jitter band the /4.0
    // folds speak, the 0.005 rise the spread band carries)
    assert_eq!(
        app.particles.len(),
        40,
        "the 20-call loop the TWO puffs the call spawns"
    );
    for p in &app.particles {
        match &p.kind {
            crate::particles::ParticleKind::CampfireSmoke { frame, alpha } => {
                // THE COSY HALF'S OWN LAWS (the banked kind wears)
                assert!(*frame < 12, "the frame the ascending list keeps: {}", frame);
                assert!(
                    (*alpha - 0.9).abs() < 1e-6,
                    "the Cosy setAlpha the spawn keeps: {}",
                    alpha
                );
                assert!(
                    (80..=129).contains(&p.lifetime),
                    "the nextInt(50)+80 band the cosy arm keeps: {}",
                    p.lifetime
                );
                assert!(
                    p.quad_size >= 0.3 && p.quad_size <= 0.6,
                    "the scale(3.0f) the base quad triples: {}",
                    p.quad_size
                );
                assert_eq!(
                    p.col,
                    [1.0, 1.0, 1.0],
                    "the WHITE tint the ctor defaults keep"
                );
                assert!(
                    p.vy >= 0.07 && p.vy < 0.082,
                    "the 0.07 arg the /500 bump carries: {}",
                    p.vy
                );
                assert!(
                    p.y >= top as f64 && p.y <= top as f64 + 2.0,
                    "the +nextDouble+nextDouble band the y row keeps: {}",
                    p.y
                );
                assert!(
                    p.x >= c as f64 + 0.1666 && p.x <= c as f64 + 0.8334,
                    "the x jitter the /3.0 band keeps: {}",
                    p.x
                );
                assert!(
                    p.z >= c as f64 + 0.1666 && p.z <= c as f64 + 0.8334,
                    "the z jitter the /3.0 band keeps: {}",
                    p.z
                );
            }
            crate::particles::ParticleKind::Smoke => {
                assert_eq!(p.y, top as f64 + 0.4, "the +0.4 constant the y row wears");
                assert!(
                    p.x >= c as f64 + 0.25 && p.x <= c as f64 + 0.75,
                    "the x jitter the /4.0 band keeps: {}",
                    p.x
                );
                assert!(
                    p.z >= c as f64 + 0.25 && p.z <= c as f64 + 0.75,
                    "the z jitter the /4.0 band keeps: {}",
                    p.z
                );
                assert!(
                    p.vy > -0.03 && p.vy < 0.06,
                    "the 0.005 rise the spread band keeps: {}",
                    p.vy
                );
            }
            other => panic!("the TWO kinds the column speaks: {:?}", other),
        }
    }
    // THE PAIR ORDER (the makeParticles walk the cosy half FIRST
    // serves, the isForce smoke the same call appends — the
    // interleaved pairs the bytecode's own sequence keeps)
    for pair in app.particles.chunks(2) {
        assert!(
            matches!(
                pair[0].kind,
                crate::particles::ParticleKind::CampfireSmoke { .. }
            ),
            "the cosy puff the pair's head wears"
        );
        assert!(
            matches!(pair[1].kind, crate::particles::ParticleKind::Smoke),
            "the smoke puff the pair's tail wears"
        );
    }
}

#[test]
fn the_douse_the_wick_smoke_the_ctor_law_wears() {
    // THE SMOKE'S OWN CTOR CHAIN (javap BaseAshSmokeParticle over
    // the 26.2 jar — the shoot_smoke_puff helper's own fold the
    // douse's addParticle rides): the grey tint the r*0.3 draw
    // carries, the 0.75 size fold the base quad's own band keeps,
    // the 8..40 lifetime the smoke_lifetime law speaks, the
    // gravity −0.1 the rise rides, the 0.96 friction the ctor
    // sets — the wick-top puff the douse's own walk spawns
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
        w.set(c, top, c, crate::world::CANDLE_CAKE_LIT);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the douse the press consumes");
    assert_eq!(
        app.particles.len(),
        1,
        "the wick's own puff the douse spawns"
    );
    let p = &app.particles[0];
    assert_eq!(p.kind, crate::particles::ParticleKind::Smoke);
    // THE TINT'S OWN GREY (the r*0.3 draw the [0, 0.3) band keeps,
    // the r=g=b trio the grey lane speaks)
    assert!(
        p.col[0] < 0.3 && p.col[0] == p.col[1] && p.col[1] == p.col[2],
        "the grey tint the r*0.3 draw keeps: {:?}",
        p.col
    );
    // THE SIZE'S OWN FOLD (the base 0.1..0.2 the ×0.75 the band
    // 0.075..0.15 keeps)
    assert!(
        p.quad_size >= 0.0749 && p.quad_size <= 0.1501,
        "the 0.75 fold the size band keeps: {}",
        p.quad_size
    );
    // THE LIFETIME'S OWN LAW (the 8..40 ticks the 8/(r*0.8+0.2)
    // walk speaks)
    assert!(
        (8..=40).contains(&p.lifetime),
        "the 8..40 band the lifetime law keeps: {}",
        p.lifetime
    );
    assert_eq!(p.gravity, -0.1, "the rise the negative gravity rides");
    assert_eq!(p.friction, 0.96, "the ctor's own friction");
}

#[test]
fn the_dig_the_mate_clear_the_fire_chain_wears() {
    // THE MATE'S OWN CHAIN (javap the autopush-413 walk — the
    // flag-35 setBlock the UPDATE_NEIGHBORS bit keeps, the
    // DoublePlantBlock.updateShape the Y-arm the orphan's own AIR
    // serves): the UPPER's dig the LOWER's clear the same beat
    // speaks, the fire cell the canBurn scan the mate held the
    // SAME beat douses
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
        w.set(c, top, c, crate::world::TALL_GRASS); // the LOWER half the dig reads
        w.set(c, top + 1, c, crate::world::TALL_GRASS_UPPER); // the UPPER the aim hits
        w.set(c, top - 1, c + 1, crate::world::AIR); // the fire's own below the air
        w.set(c, top, c + 1, crate::world::FIRE); // the fire the LOWER the scan holds
        w.fire_age.insert((c, top, c + 1), 0);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 5.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE INSTABREAK PRESS: the dig arms the aim cell (the UPPER —
    // the instabreak row the START completes), the mate's own clear
    // the effects walk serves, the chain the fire the SAME beat
    // douses
    app.on_attack();
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_ne!(
            w.get(c, top + 1, c),
            crate::world::TALL_GRASS_UPPER,
            "the dig the UPPER consumes"
        );
        assert_ne!(
            w.get(c, top, c),
            crate::world::TALL_GRASS,
            "the mate's own clear the walk speaks"
        );
        assert_eq!(
            w.get(c, top, c + 1),
            crate::world::AIR,
            "the fire the mate's loss the chain douses — the same beat"
        );
        assert!(
            !w.fire_age.contains_key(&(c, top, c + 1)),
            "the dead cell's age entry the chain drops"
        );
    }
}

#[test]
fn the_door_the_place_the_two_halves_wear() {
    // THE DOOR'S OWN TWO HALVES (javap DoorBlock the 26.2 jar —
    // the setPlacedBy walk pc 0-21 the `pos.above()` cell's
    // `setValue(HALF, DoubleBlockHalf.UPPER)` setBlockAndUpdate
    // speaks, the facing byte BOTH cells wear [the
    // getStateForPlacement walk's own state the setPlacedBy
    // pass-through keeps — the NO-opposite look the
    // door_facing_byte stamps]; the yaw 0 the north byte 1 the
    // FACING_BY_2D's own row answers)
    let mut app = App::new(3);
    app.enter_world();
    app.offline_held_updated(Some("item/oak_door"));
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
        "the door placement consumed the press"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::OAK_DOOR,
            "the closed lower the item's own place wears"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::OAK_DOOR_UPPER,
            "the upper half the setPlacedBy walk banks"
        );
        assert_eq!(
            w.variant(c, top, c),
            1,
            "the lower's facing byte the yaw-0 north wears"
        );
        assert_eq!(
            w.variant(c, top + 1, c),
            1,
            "the upper's facing byte the same state keeps"
        );
    }
    // THE CONSUME (BlockItem.place's own shrink — the held stack
    // the count rides down)
    let sel = app.selected_hotbar;
    assert_eq!(
        app.inventory.items[sel].0, 0,
        "the single stack the consume empties"
    );
    // THE GATE'S OWN REFUSAL (javap getStateForPlacement pc 21-33
    // — the occupied above cell the `aconst_null` refusal keeps:
    // the stone roof the placement never arms, the PASS the item
    // lanes follow)
    app.offline_held_updated(Some("item/oak_door"));
    {
        let w = app.offline_world.as_mut().unwrap();
        w.set(c + 2, top, c, crate::world::AIR);
        w.set(c + 2, top + 1, c, crate::world::STONE);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let press = app.on_right_press(0, 0);
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_ne!(
            w.get(c + 2, top, c),
            crate::world::OAK_DOOR,
            "the roofed cell the placement refuses ({press})"
        );
    }
}

#[test]
fn the_door_the_hinge_census_the_right_quartet_wears() {
    // THE DOOR'S OWN HINGE CENSUS (javap DoorBlock.getHinge the
    // 26.2 jar — the getHinge(BlockPlaceContext) walk the
    // getStateForPlacement's own HINGE setValue serves: the
    // LOWER-door presence arms the double-door seam's own verdict
    // speaks — the LEFT side cell's door the RIGHT hinge banks,
    // the seam door the pair's own mirrored swing the vanilla's
    // double-door feel wears; the empty world the score-0 click
    // walk serves, the center hit the 0.5 boundary the LEFT
    // keeps). THE RIGHT QUARTET'S OWN FLIP (the mate walk the ±2
    // arithmetic the same-hinge quartet serves — the open bit the
    // cycle wears, the hinge bit the flip never touches)
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE FIRST DOOR THE EMPTY CENSUS LANDS (the no-wall no-door
    // world the click walk answers — the LEFT quartet the
    // registerDefaultState pin's own default wears)
    app.offline_held_updated(Some("item/oak_door"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the first door the placement consumes"
    );
    // THE SEAM'S OWN NEIGHBOR (the adjacent east cell the second
    // door claims — the placed cell's own ccw side read the first
    // door's closed lower answers, the RIGHT verdict the ±84 mate
    // the RIGHT quartet banks)
    app.offline_held_updated(Some("item/oak_door"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 1.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(
        app.on_right_press(0, 0),
        "the seam door the placement consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::OAK_DOOR_RIGHT,
            "the seam's closed lower the RIGHT quartet wears"
        );
        assert_eq!(
            w.get(c + 1, top + 1, c),
            crate::world::OAK_DOOR_UPPER_RIGHT,
            "the seam's upper the same-hinge mate banks"
        );
        assert_eq!(
            w.variant(c + 1, top, c),
            1,
            "the right lower's facing byte the yaw-0 north wears"
        );
        assert_eq!(
            w.variant(c + 1, top + 1, c),
            1,
            "the right upper's facing byte the same state keeps"
        );
    }
    // THE RIGHT QUARTET'S OWN FLIP (the empty hand's use the same
    // walk the LEFT quartet speaks — the both halves the one beat
    // swaps, the voice the set type's own pair wears)
    assert!(
        app.on_right_press(0, 0),
        "the right-hinged door the flip consumes"
    );
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::OAK_DOOR_OPEN_RIGHT,
            "the seam's open lower the right quartet's own open kind wears"
        );
        assert_eq!(
            w.get(c + 1, top + 1, c),
            crate::world::OAK_DOOR_UPPER_OPEN_RIGHT,
            "the seam's open upper the synced mate wears"
        );
    }
}

#[test]
fn the_lever_places_toggles_and_the_click_voice_wears() {
    // THE LEVER'S OWN PLACEMENT + TOGGLE WALK (the dev-45 round —
    // javap the base getStateForPlacement pcs 0-139 the FLOOR
    // face's UP hit answers, the LeverBlock.useWithoutItem pc 0-55
    // the cycle the toggle speaks, the playSound pc 0-43 the
    // LEVER_CLICK 0.3 volume carries, the pitch the NEW state's
    // own bit the 0.6/0.5 fold answers): the OFF kind the item
    // lands, the raw-look north byte the yaw-0 stamp wears, the
    // ON flip the +1 mate the second press speaks, the two
    // pitches the two voices wear.
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_held_updated(Some("item/lever"));
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the lever's placement consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c, top, c),
            crate::world::LEVER,
            "the OFF kind the item's own kind wears"
        );
        assert_eq!(
            w.variant(c, top, c),
            1,
            "the raw look's north byte the yaw-0 stamp wears"
        );
    }
    // THE TOGGLE'S OWN BEAT (the same aim the lever cell the ray
    // answers — the clip's kind law the lever cell the hit speaks)
    assert!(app.on_right_press(0, 0), "the ON toggle consumes");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::LEVER_ON,
        "the ON kind the +1 mate wears"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.lever.click") && e.contains("p0.600000")),
        "the ON click's 0.6 pitch the walk speaks"
    );
    assert!(app.on_right_press(0, 0), "the OFF toggle consumes");
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(c, top, c),
        crate::world::LEVER,
        "the OFF kind the -1 mate wears"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.lever.click") && e.contains("p0.500000")),
        "the OFF click's 0.5 pitch the walk speaks"
    );
}

#[test]
fn the_lever_wire_the_iron_door_speaks() {
    // THE WIRE'S OWN VOICE (the dev-45 round — javap
    // LeverBlock.updateNeighbours pc 9-65 the lever pos + the
    // support pos broadcasts the scan serves, the
    // DoorBlock.neighborChanged walk the POWERED vs the flag
    // compare the setOpen law speaks: the IRON door the hand gate
    // refuses the wire answers — the lever's ON the both halves
    // swing, the click's OFF the door closes, the door voices the
    // set's own events wear, the vibration posts the sculk lane
    // hears)
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
        // THE IRON DOOR'S OWN PAIR + THE OFF LEVER EAST (the door's
        // lower/upper the closed kinds wear, the lever's own cell
        // the ground answers — the survival law the stone floor
        // keeps)
        w.set(c, top, c, crate::world::IRON_DOOR);
        w.set(c, top + 1, c, crate::world::IRON_DOOR_UPPER);
        w.set(c + 1, top, c, crate::world::LEVER);
        w.set_variant(c + 1, top, c, 1);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 1.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the lever's ON press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::LEVER_ON,
            "the lever's own kind the ON bit wears"
        );
        assert_eq!(
            w.get(c, top, c),
            crate::world::IRON_DOOR_OPEN,
            "the wire the iron door's lower swings"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::IRON_DOOR_UPPER_OPEN,
            "the wire the upper half syncs"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.lever.click")),
        "the lever's own click the walk plays"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.iron_door.open")),
        "the iron door's wire voice the set plays"
    );
    // THE OFF PRESS (the signal's death the door closes — the
    // second broadcast the scan re-reads, the verdict's silence
    // the closed kinds the flag's false arm wears)
    assert!(app.on_right_press(0, 0), "the lever's OFF press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::LEVER,
            "the lever's own OFF the -1 mate wears"
        );
        assert_eq!(
            w.get(c, top, c),
            crate::world::IRON_DOOR,
            "the dead signal the door closes"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::IRON_DOOR_UPPER,
            "the upper the same beat closes"
        );
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.iron_door.close")),
        "the close voice the second flip plays"
    );
    assert!(
        app.game_events
            .iter()
            .any(|e| e.starts_with("block_open") || e.starts_with("block_close")),
        "the vibration posts the sculk lane hears"
    );
}

#[test]
fn the_lever_toggle_the_rail_verdict_the_swap_wears() {
    // THE LEVER'S OWN RAIL VERDICT (the dev-99 round — javap
    // LeverBlock.updateNeighbours the setBlock flag-3 the two
    // seeds the lever pos + the support pos the broadcast speaks,
    // the PoweredRailBlock.neighborChanged -> updateState chain
    // the rails among the seeds' neighborhoods the re-read
    // serves): the live lever the powered rail's POWERED bit the
    // swap arms, the dead lever the bit clears, the detector's
    // own cart law the signal never arms, the plain rail the bit
    // the swap never wears, the shape bytes the verdicts the
    // nibble alone answer.
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
        // THE THREE RAILS + THE OFF LEVER (the lever's own cell the
        // ground answers — the rails the lever's own four horizontal
        // neighbors the x-pair the z-pair flank)
        w.set(c + 1, top, c, crate::world::POWERED_RAIL);
        w.set_variant(c + 1, top, c, 0);
        w.set(c + 3, top, c, crate::world::DETECTOR_RAIL);
        w.set_variant(c + 3, top, c, 0);
        w.set(c + 2, top, c - 1, crate::world::RAIL);
        w.set_variant(c + 2, top, c - 1, 0);
        w.set(c + 2, top, c, crate::world::LEVER);
        w.set_variant(c + 2, top, c, 1);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE ON PRESS (the live lever the bit the swap arms)
    assert!(app.on_right_press(0, 0), "the lever's ON press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 2, top, c),
            crate::world::LEVER_ON,
            "the lever's own kind the ON bit wears"
        );
        assert_eq!(
            w.variant(c + 1, top, c),
            crate::world::RAIL_POWERED_BIT,
            "the live lever the powered rail's bit arms"
        );
        assert_eq!(
            w.variant(c + 3, top, c),
            0,
            "the detector's own cart law the lever's signal never arms"
        );
        assert_eq!(
            w.variant(c + 2, top, c - 1),
            0,
            "the plain rail the bit the swap never wears"
        );
    }
    // THE OFF PRESS (the signal's death the bit the swap clears)
    assert!(app.on_right_press(0, 0), "the lever's OFF press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 2, top, c),
            crate::world::LEVER,
            "the lever's own OFF the -1 mate wears"
        );
        assert_eq!(
            w.variant(c + 1, top, c),
            0,
            "the dead lever the powered rail's bit clears"
        );
    }
}

#[test]
fn the_lever_toggle_the_activator_twin_the_same_seam_wears() {
    // THE ACTIVATOR'S OWN TWIN THROUGH THE SEAM (the dev-99 round —
    // the same two-seed broadcast the dev-97 swap law the toggle
    // re-fires): the live lever the activator's bit the same walk
    // arms, the dead lever the bit clears, the powered kind the
    // same press its own bit keeps, the shape nibble the pair's
    // own bytes answer.
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
        w.set(c + 1, top, c, crate::world::POWERED_RAIL);
        w.set_variant(c + 1, top, c, 0);
        w.set(c + 3, top, c, crate::world::ACTIVATOR_RAIL);
        w.set_variant(c + 3, top, c, 0);
        w.set(c + 2, top, c, crate::world::LEVER);
        w.set_variant(c + 2, top, c, 1);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the lever's ON press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.variant(c + 3, top, c),
            crate::world::RAIL_POWERED_BIT,
            "the live lever the activator's bit arms"
        );
        assert_eq!(
            w.variant(c + 1, top, c),
            crate::world::RAIL_POWERED_BIT,
            "the powered kind the same press its own bit keeps"
        );
    }
    assert!(app.on_right_press(0, 0), "the lever's OFF press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.variant(c + 3, top, c),
            0,
            "the dead lever the activator's bit clears"
        );
        assert_eq!(
            w.variant(c + 1, top, c),
            0,
            "the powered kind the dead lever's bit clears"
        );
    }
}

#[test]
fn the_support_loss_the_pop_the_voices_and_the_wire_wear() {
    // THE SUPPORT-LOSS WALK'S OWN LOOP (this round — javap the
    // base updateShape pc 0-28 the gate + the canSurvive verdict
    // the AIR the pop serves, NeighborUpdater.executeShapeUpdate
    // pc 50-60 the updateOrDestroy chain the Level.destroyBlock
    // walk completes, LeverBlock.affectNeighborsAfterRemoval pc
    // 0-28 the powered source's wire re-read the doors' silence
    // speaks): the pedestal's own dig the ON lever pops, the item
    // the dropResources lane spawns, the doors the signal verdict
    // the false the setOpen law closes, the voices the walks wear
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    {
        let w = app.offline_world.as_mut().unwrap();
        for dx in -3..=4 {
            for dz in -2..=2 {
                w.set(c + dx, top - 1, c + dz, crate::world::STONE);
                w.set(c + dx, top, c + dz, crate::world::AIR);
                w.set(c + dx, top + 1, c + dz, crate::world::AIR);
            }
        }
        // THE WALL LEVER'S OWN STATE (the east-facing lever's wall
        // east the variant byte banks, the ON kind the wire holds —
        // the iron door the open kinds BOTH halves wear, the door
        // the lever's own WEST cell the scan reads)
        w.set(c + 2, top, c, crate::world::STONE);
        w.set(c + 1, top, c, crate::world::LEVER_ON);
        w.set_variant(
            c + 1,
            top,
            c,
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 4),
        );
        w.set(c, top, c, crate::world::IRON_DOOR_OPEN);
        w.set(c, top + 1, c, crate::world::IRON_DOOR_UPPER_OPEN);
    }
    // THE EAST LOOK'S OWN DIG (the ray the support pillar's EAST
    // face reads — the lever's own cell the path never crosses,
    // the dig the support stone mines)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 4.5, top as f32 + 0.5, c as f32 + 0.5),
        yaw: -std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    app.on_press(0, 0);
    assert!(
        app.dig.is_destroying,
        "the press armed the support's own dig"
    );
    assert_eq!(app.dig.pos, (c + 2, top, c), "the support cell armed");
    let mut ticks = 0;
    while app.offline_world.as_ref().unwrap().get(c + 2, top, c) != crate::world::AIR {
        app.update();
        ticks += 1;
        assert!(ticks < 400, "the dig completed in time");
    }
    {
        let w = app.offline_world.as_ref().unwrap();
        // THE POP'S OWN VERDICT (the updateShape arm's AIR return
        // the walk the support's loss speaks)
        assert_eq!(
            w.get(c + 1, top, c),
            crate::world::AIR,
            "the lever the support's loss pops"
        );
        // THE WIRE'S OWN RE-READ (the ON source GONE the doors'
        // signal verdict the false the setOpen law closes — the
        // both halves the same beat the sync wears)
        assert_eq!(
            w.get(c, top, c),
            crate::world::IRON_DOOR,
            "the door the wire's silence closes"
        );
        assert_eq!(
            w.get(c, top + 1, c),
            crate::world::IRON_DOOR_UPPER,
            "the upper half the sync closes"
        );
    }
    // THE POP'S OWN REWARD (javap Level.destroyBlock pc 48-81 —
    // the dropResources the pop's own lane wears: the lever item
    // the entity speaks, the bare hand's stone the dig's own roll
    // refuses — the ONE drop the pop alone owns)
    assert_eq!(app.drops.len(), 1, "the pop's own drop spawned");
    let expected = mc_registry::item_id_by_name("lever").unwrap();
    assert_eq!(
        app.drops[0].item, expected,
        "the lever item the pop's roll wears"
    );
    assert_eq!(app.drops[0].count, 1);
    // THE VOICES' OWN ARCS (the 2001 events: the close voice the
    // wire re-read plays, the block_destroy posts the stone's own
    // + the pop's own the two walk both speak)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.iron_door.close")),
        "the close voice the wire's own re-read plays"
    );
    assert_eq!(
        app.game_events
            .iter()
            .filter(|e| e.starts_with("block_destroy"))
            .count(),
        2,
        "the stone's own + the pop's own the vibration posts wear"
    );
}

#[test]
fn the_wire_run_the_tail_rail_the_power_wears() {
    // THE WIRE'S OWN RAIL WAKE (this round — javap RedStoneWireBlock
    // .updatePowerStrength the changed cells' own setBlock flag-3 the
    // Level.updateNeighborsAt the six the neighborChanged law speaks,
    // the PoweredRailBlock/ActivatorRailBlock updateState chain the
    // rails among the flips' own neighborhoods the re-read answers):
    // the lever the wire run's own decay the rungs wear (the source
    // cell the 15 speaks, the -1 per step the fixpoint answers), the
    // run's tail rail the POWERED bit the verdict arms — the rail
    // FOUR cells past the lever the two-seed sweep the updateNeighbours
    // walk wears never reaches, the wire's own wake the ONLY arm the
    // verdict answers, the OFF press the bit the same wake clears.
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
            }
        }
        // THE ROW (the lever's own cell the ground answers, the wire
        // run the x-row wears, the tail rail the run's east end the
        // floor stone carries)
        w.set(c + 2, top, c, crate::world::LEVER);
        w.set_variant(c + 2, top, c, 1);
        w.set(c + 3, top, c, crate::world::REDSTONE_WIRE);
        w.set(c + 4, top, c, crate::world::REDSTONE_WIRE);
        w.set(c + 5, top, c, crate::world::REDSTONE_WIRE);
        w.set(c + 6, top, c, crate::world::POWERED_RAIL);
        w.set_variant(c + 6, top, c, 0);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE ON PRESS (the wire run the decay the rungs wear, the tail
    // rail the wire's own wake arms)
    assert!(app.on_right_press(0, 0), "the lever's ON press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 2, top, c),
            crate::world::LEVER_ON,
            "the lever's own kind the ON bit wears"
        );
        assert_eq!(
            w.get(c + 3, top, c),
            crate::world::REDSTONE_WIRE_ON,
            "the source cell the 15 wears"
        );
        assert_eq!(
            w.get(c + 4, top, c),
            crate::world::REDSTONE_WIRE_P14,
            "the first step the -1 decay answers"
        );
        assert_eq!(
            w.get(c + 5, top, c),
            crate::world::REDSTONE_WIRE_P13,
            "the second step the rung ladder wears"
        );
        assert_eq!(
                w.variant(c + 6, top, c),
                crate::world::RAIL_POWERED_BIT,
                "the tail rail the wire's own wake arms — the lever's two-seed sweep the walk never reaches"
            );
    }
    // THE OFF PRESS (the signal's death the rungs the plain kind
    // wear, the tail rail the bit the same wake clears)
    assert!(app.on_right_press(0, 0), "the lever's OFF press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 5, top, c),
            crate::world::REDSTONE_WIRE,
            "the dead run the plain kind wears"
        );
        assert_eq!(
            w.variant(c + 6, top, c),
            0,
            "the dead run the tail rail's bit clears"
        );
    }
}

#[test]
fn the_torch_not_gate_the_wire_re_read_the_tail_rail_the_wake_wears() {
    // THE TORCH'S OWN BEAT THE DRAIN SERVES (this round — the drain
    // walk's own wire flips the rail wake the attach walk's own seam
    // banks): the lit torch the wire's own 15 the re-read arms and
    // the tail rail the wake's own verdict seats, the lever press
    // the NOT-gate the +2 beat flips (the pedestal the lever's own
    // signal reads), the OFF flip the wire's drop the SAME drain
    // re-reads and the rail's STALE bit the wake clears, the second
    // press the relight the 15 the wire re-wears and the rail the
    // wake re-arms.
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
        // support stone carries, the tail rail the wire's east end
        // the same support row wears)
        w.set(c + 2, top, c, crate::world::LEVER);
        w.set_variant(c + 2, top, c, 1);
        w.set(c + 3, top, c, crate::world::STONE);
        w.set(c + 3, top + 1, c, crate::world::REDSTONE_TORCH);
        w.set(c + 4, top, c, crate::world::STONE);
        w.set(c + 4, top + 1, c, crate::world::REDSTONE_WIRE);
        w.set(c + 5, top, c, crate::world::STONE);
        w.set(c + 5, top + 1, c, crate::world::POWERED_RAIL);
        w.set_variant(c + 5, top + 1, c, 0);
    }
    // THE LIT TORCH'S OWN ARM (the attach walk's own seam the
    // placed-wire beat speaks — the component re-read the torch's
    // 15 the wire wears, the tail rail the wake's own verdict arms)
    app.attach_wire_voice(c + 4, top + 1, c);
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 4, top + 1, c),
            crate::world::REDSTONE_WIRE_ON,
            "the lit torch the wire's own 15 wears"
        );
        assert_eq!(
            w.variant(c + 5, top + 1, c),
            crate::world::RAIL_POWERED_BIT,
            "the tail rail the wire's own wake arms"
        );
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 2.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE ON PRESS (the NOT-gate's own consult the lever's scope
    // arms — the pedestal the torch's own below the verdict reads,
    // the rail's bit the STALE armed state keeps until the drain)
    assert!(app.on_right_press(0, 0), "the lever's ON press consumes");
    {
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(
            w.get(c + 2, top, c),
            crate::world::LEVER_ON,
            "the lever's own kind the ON bit wears"
        );
        assert_eq!(
            w.variant(c + 5, top + 1, c),
            crate::world::RAIL_POWERED_BIT,
            "the press beat the rail's stale bit keeps — the +2 consult the flip alone answers"
        );
    }
    // THE DRAIN'S OWN BEAT (the +2 consult the tick walk serves —
    // the torch the extinguish arm flips OFF, the wire re-read the
    // component's drop the same walk answers, the tail rail the
    // wake's own verdict the stale bit CLEARS)
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
            w.variant(c + 5, top + 1, c),
            0,
            "the drain's own wake the stale bit clears — the attach seam's own law the beat wears"
        );
    }
    // THE SECOND PRESS (the lever's death the consult re-arms — the
    // relight the +2 beat serves, the wire's 15 the re-read re-wears,
    // the tail rail the wake the bit re-arms)
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
            w.variant(c + 5, top + 1, c),
            crate::world::RAIL_POWERED_BIT,
            "the drain's own wake the rail's bit re-arms"
        );
    }
}

/// THE MOBGRIEFING RULE'S OWN GRIEF LAW (the RULE_MOBGRIEFING row
/// the mob blast's own interaction reads): the false the mob-sourced
/// terrain the blast never clears — the creeper's toBlow the empty
/// set the NONE interaction keeps, the SOUND the discard the rule
/// never gates (the entity doses the blast's own walk still serves),
/// the TNT's own KEEP interaction the primed tick's own lane keeps.
#[test]
fn the_mob_griefing_rule_spares_the_creeper_terrain() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE FLAT STONE PAD (the blast's own cells the walk banks — the
    // every-cell STONE the after-verdict the honest difference keeps)
    {
        let w = app.offline_world.as_mut().unwrap();
        for x in (c - 4)..(c + 6) {
            for z in (c - 4)..(c + 6) {
                for y in (top - 3)..top {
                    w.set(x, y, z, crate::world::STONE);
                }
                w.set(x, top, z, crate::world::STONE);
                for y in (top + 1)..size {
                    w.set(x, y, z, crate::world::AIR);
                }
            }
        }
    }
    app.spawn_offline_body();
    // THE DISARM (the rule the set walk flips — the chat literal's
    // own arm)
    app.chat_open = true;
    app.chat_input = "/gamerule mobGriefing false".into();
    app.on_chat_submit();
    assert!(!app.mob_griefing, "the set the rule disarms");
    // THE CREEPER'S OWN RING (the body the 2-block seat wears — the
    // dist-squared 4.0 the START ring 9.0 the goal arms, the LOS
    // the same-level seats keep)
    let mut creep = crate::mob::Zombie::new(7, c as f64 + 2.5, top as f64, c as f64 + 0.5, 3);
    creep.kind = crate::mob::MobKind::Creeper;
    creep.on_ground = true;
    app.mobs.push(creep);
    // THE SWELL WALK (the goal the dir arms, the 30-tick fuse the
    // maxSwell crossing serves — the boom the walk's own tail speaks)
    for _ in 0..40 {
        app.update();
    }
    // THE TERRAIN THE RULE SPARES (the NONE interaction's empty
    // toBlow — the pad's every cell the blast never clears)
    let w = app.offline_world.as_ref().unwrap();
    for x in (c - 4)..(c + 6) {
        for z in (c - 4)..(c + 6) {
            assert_eq!(
                w.get(x, top, z),
                crate::world::STONE,
                "the pad's cell the spared terrain keeps"
            );
        }
    }
    // THE BLAST THE RULE NEVER GATES (the discard the swell machine's
    // own tail speaks, the boom voice the same consumer plays — the
    // sound the mobGriefing-off world still hears)
    assert!(
        app.mobs.iter().all(|m| m.id != 7),
        "the exploded creeper the discard removes"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.generic.explode @")),
        "the boom voice the rule never gates"
    );
}

/// THE TURN KEY'S OWN LANE (the pointer-grab alternative the
/// window-edge recon lands — minifb 0.27 has no cursor grab/warp
/// the API surface speaks, the Pass stream the desktop cursor
/// tracks until the SCREEN edge's pin the mouse lane's own turn
/// stop wears): the held arrow the frame's dt the rate scales —
/// the World screen's own gate the mouse lane's shape keeps (the
/// pause and the open screen the lane never reads), the degrees
/// the body's turn_keys the same clamp law wears.
#[test]
fn the_turn_keys_steer_where_the_cursor_pinned() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE RATE: half a second at the 180 deg/s the 90 degrees
    // speaks — the quarter revolution the held Right wears
    let yaw_before = app.body.as_ref().unwrap().yaw;
    app.on_turn_key(1.0, 0.0, 0.5);
    let turned = app.body.as_ref().unwrap().yaw - yaw_before;
    assert!(
        (turned - 90.0).abs() < 1e-4,
        "0.5s at the 180 deg/s = 90 degrees, got {}",
        turned
    );
    // THE PITCH CLAMP: the held Down the ±90 the turn's own law
    // wears (the same PITCH_LIMIT the mouse curve keeps)
    app.on_turn_key(0.0, 1.0, 100.0);
    assert_eq!(
        app.body.as_ref().unwrap().pitch,
        crate::body::PITCH_LIMIT,
        "the held Down the clamp keeps"
    );
    // THE PAUSE FREEZE: the keys the frozen lane never reads
    app.paused = true;
    let yaw = app.body.as_ref().unwrap().yaw;
    app.on_turn_key(1.0, 0.0, 1.0);
    assert_eq!(
        app.body.as_ref().unwrap().yaw,
        yaw,
        "the pause the turn freezes"
    );
    // THE SCREEN GATE: the open screen the in-world lane never is
    // (the menus the cursor shows, the turn the mouse lane's own
    // gate keeps silent)
    app.paused = false;
    app.screen = crate::screens::ScreenKind::Title;
    let yaw = app.body.as_ref().unwrap().yaw;
    app.on_turn_key(1.0, 0.0, 1.0);
    assert_eq!(
        app.body.as_ref().unwrap().yaw,
        yaw,
        "the screen the turn gates"
    );
}

#!/usr/bin/env python3
"""autopush-417 helper — the round's own test the part_11 tail appends
(the two shulker tails the shared closing asserts wear — the append the
EOF the anchor keeps). Idempotent: the marker the second run refuses."""
import sys

PATH = "crates/mc-client/src/screens/tests/part_11.rs"
MARK = "fn the_enchant_gather_the_wire_half_wears"

TEST = '''

/// THE ENCHANT GATHER'S OWN WIRE HALF (the 417 round — the menu type 13
/// the OpenScreen word routes, the honest drop the round retires): the
/// route the window opens [the seats the pair speaks, the rows the
/// mirror feeds, the carried the authoritative cursor lands, the
/// identity the gather echoes against], the gather's wire walk the
/// release serves [the belief the prediction answers, the packet the
/// command lane speaks, the seat the pull rides, the different row the
/// walk never names, the HUD row the sync follows], the honest gates
/// the silence keeps [the pickup press + the drag arm + the spell
/// click the wire round never banks], the wire close the echo speaks,
/// the server's own drop the window retires.
#[test]
fn the_enchant_gather_the_wire_half_wears() {
    use crate::world_view::{ContainerCommand, ContainerInput, LiveContainer};
    let lapis = mc_registry::item_id_by_name("lapis_lazuli").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();

    // APP ARM A — THE ROUTE'S OWN FEED (the menu type 13 the window
    // opens — the seats the pair speaks, the rows the mirror feeds,
    // the identity the clicks echo against)
    let mut app = App::new(3);
    app.enter_world();
    app.world_is_network = true;
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.live_clicks = Some(tx);
    // the menu's own numbering: the item seat 0, the lapis seat 1, the
    // rows 2..28 the main grid, 29..37 the hotbar (menu 4 the main
    // row's item 11 answers, menu 30 the hotbar row 1)
    let mut slots = vec![(0i32, 0i32); 38];
    slots[1] = (lapis, 3);
    slots[4] = (lapis, 20);
    slots[5] = (dirt, 30);
    slots[30] = (dirt, 7);
    app.live_container_updated(Some(LiveContainer {
        id: 4,
        menu_type_id: crate::world_model::MENU_TYPE_ENCHANTMENT,
        slots,
        bundle_rows: Vec::new(),
        carried: (0, 0),
        state_id: 9,
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    }));
    assert_eq!(
        app.screen,
        ScreenKind::Enchanting,
        "the route the enchant window opens"
    );
    assert!(app.enchant_is_live, "the live flag the route wears");
    assert_eq!(
        app.enchant_live_window,
        Some((4, 9)),
        "the window identity the gather echoes against"
    );
    assert_eq!(
        app.enchant_slots,
        vec![(0, 0), (lapis, 3)],
        "the seats the pair speaks"
    );
    assert_eq!(
        app.enchant_mirror[2],
        (lapis, 20),
        "the wire row the mirror banks"
    );
    assert_eq!(
        app.enchant_mirror[3],
        (dirt, 30),
        "the different row the mirror banks"
    );

    // APP ARM B — THE GATHER'S OWN WIRE WALK (the release the echo
    // speaks: the belief the prediction answers, the seat the pull
    // rides, the different row the walk never names, the packet the
    // command lane carries)
    let mut fb = crate::raster::Framebuf::new(320, 180);
    let assets_root = assets_root();
    {
        let mut a = GuiAssets::open(&assets_root);
        app.mouse_pos = None;
        app.render(&mut fb, &mut a);
    }
    let (rx4, ry4) = {
        app.enchant_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 4)
            .map(|&(x, y, _)| (x, y))
            .unwrap()
    };
    app.enchant_carried = (lapis, 5);
    app.enchant_doubleclick = true;
    app.mouse_pos = Some((rx4 + 8, ry4 + 8));
    app.on_left_release();
    assert_eq!(
        app.enchant_carried,
        (lapis, 28),
        "the seat + the row the pull grows"
    );
    assert_eq!(
        app.enchant_slots[1],
        (0, 0),
        "the lapis seat the gather drains"
    );
    assert_eq!(
        app.enchant_mirror[2],
        (0, 0),
        "the wire row the pull drains"
    );
    assert_eq!(
        app.enchant_mirror[3],
        (dirt, 30),
        "the different row the walk never names"
    );
    assert_eq!(
        app.hotbar_slots[1],
        (dirt, 7),
        "the HUD row the mirror sync feeds"
    );
    match rx.try_recv().unwrap() {
        ContainerCommand::Click(click) => {
            assert_eq!(click.window_id, 4, "the window id the echo wears");
            assert_eq!(click.state_id, 9, "the state id the echo wears");
            assert_eq!(click.slot, 4, "the hovered row the echo speaks");
            assert_eq!(click.button, 0, "the left press the echo carries");
            assert!(
                matches!(click.input, ContainerInput::PickupAll),
                "the gather mode the echo wears"
            );
            assert_eq!(
                click.changed,
                vec![(1, (0, 0)), (4, (0, 0))],
                "the changed map the walk's own mutations name"
            );
            assert_eq!(click.carried, (lapis, 28), "the cursor the echo carries");
        }
        other => panic!("the gather the echo speaks, not {other:?}"),
    }
    assert!(
        !app.enchant_doubleclick,
        "the arm's own clear the release speaks"
    );

    // APP ARM C — THE HONEST GATES (the pickup press + the drag arm +
    // the spell click the wire round never banks — the silence the
    // local fiction never wears)
    app.mouse_pos = Some((rx4 + 8, ry4 + 8));
    app.press_doubled = false;
    app.enchant_last_click_slot = None;
    app.on_press(rx4 + 8, ry4 + 8); // the empty carried the pickup dispatch keeps
    assert!(
        rx.try_recv().is_err(),
        "the gated pickup press the silence keeps"
    );
    assert_eq!(
        app.enchant_mirror[2],
        (0, 0),
        "the drained row the gated press never re-fills"
    );
    let held_before = app.qc_armed;
    app.enchant_carried = (dirt, 10);
    app.qc_arm_enchanting(rx4 + 8, ry4 + 8, 0);
    assert_eq!(
        app.qc_armed,
        held_before,
        "the gated drag arm the silence keeps"
    );
    app.enchant_carried = (0, 0);
    assert!(
        !app.click_enchant_option(0),
        "the gated spell click the silence speaks"
    );
    assert!(
        rx.try_recv().is_err(),
        "the gated lanes no command speak"
    );

    // APP ARM D — THE WIRE CLOSE'S OWN ECHO (the client-initiated
    // close the packet speaks — the belief resets with the window, no
    // ride-home walk the local fiction never wears)
    app.enchant_carried = (dirt, 2);
    app.close_enchanting(ScreenKind::World);
    match rx.try_recv().unwrap() {
        ContainerCommand::Close { window_id } => {
            assert_eq!(window_id, 4, "the close the window id carries");
        }
        other => panic!("the close the command speaks, not {other:?}"),
    }
    assert!(!app.enchant_is_live, "the close the live flag retires");
    assert!(
        app.enchant_live_window.is_none(),
        "the close the window drops"
    );
    assert_eq!(
        app.enchant_carried,
        (0, 0),
        "the close the carried the server's own law resets"
    );
    assert_eq!(app.screen, ScreenKind::World, "the close the world answers");

    // APP ARM E — THE SERVER'S OWN DROP (the store's own None the
    // window retires — the screen back to the world)
    let mut slots2 = vec![(0i32, 0i32); 38];
    slots2[0] = (lapis, 1);
    app.live_container_updated(Some(LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_ENCHANTMENT,
        slots: slots2,
        bundle_rows: Vec::new(),
        carried: (0, 0),
        state_id: 3,
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    }));
    assert!(app.enchant_is_live, "the reopen the live flag wears");
    assert_eq!(app.enchant_live_window, Some((5, 3)), "the fresh identity");
    app.live_container_updated(None);
    assert!(
        !app.enchant_is_live,
        "the server's own drop the live flag retires"
    );
    assert!(app.enchant_live_window.is_none(), "the drop the window dies");
    assert_eq!(app.screen, ScreenKind::World, "the drop the world answers");
}
'''

src = open(PATH, encoding="utf-8").read()
assert MARK not in src, "the round's own test the file already wears"
if not src.endswith("\n"):
    src += "\n"
open(PATH, "w", encoding="utf-8").write(src + TEST)
print("appended", MARK)

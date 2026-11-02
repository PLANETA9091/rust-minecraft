#!/usr/bin/env python3
"""autopush-425 — THE TRIO'S OWN GENERATOR (the 424 round's own
mk-lane the round re-wears): the test the part_11's EOF the append
banks, the directed filter the deferred place's own lane the head
wears, the fmt regions the release walk re-keys (the five-seat's own
deferred place the region the cursor threads — the throw lane's
regions the retire drops). NO sed, NO grep: the pure-Python
transforms the -byte trap refuses. Usage: python3 scripts/mk_425.py"""

TEST = '''
/// THE BREWING STAND'S OWN DEFERRED PLACE (the 425 autopush round —
/// the empty release the five-seat canvas owes: the press the drag
/// arm holds, the release the deferred place serves — the seat the
/// click walk's own PICKUP speaks, the outside the -999 THROW the
/// 339 walk serves, the between the -1 no-op keeps, the live window
/// the arm never enters). The arms wear: the place-back's own whole
/// stack, the right's own place-ONE, the outside's own whole drop,
/// the between's own no-op, the live window's own silence.
#[test]
fn the_brewing_stand_the_deferred_place_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();

    // THE OFFLINE SIM'S OWN ARM (the fresh stand-in the drag the
    // local lanes ride — no window, no packet, the world the drop
    // entity's own lane keeps)
    let mut app = App::new(3);
    app.enter_world();
    app.world_is_network = false;
    app.screen = ScreenKind::Brewing;
    app.brewing_slot_rects = vec![
        (100 + 56 + 1, 100 + 51 + 1, 0i16),
        (100 + 79 + 1, 100 + 58 + 1, 1i16),
        (100 + 102 + 1, 100 + 51 + 1, 2i16),
        (100 + 79 + 1, 100 + 17 + 1, 3i16),
        (100 + 17 + 1, 100 + 17 + 1, 4i16),
    ];
    app.brewing_panel = (100, 100);
    app.brewing_slots = vec![(0, 0); 5]; // the five seats the fresh stand-in banks

    // ARM A — THE PLACE-BACK'S OWN WHOLE STACK (the press the arm
    // holds, the release the click walk's own PICKUP serves — the
    // carried the bottle seat takes, the cursor the walk clears)
    app.brewing_carried = (potion, 5);
    app.qc_armed = true;
    app.qc_button = 0;
    app.qc_slots.clear();
    app.qc_remainder = 0;
    app.mouse_pos = Some((100 + 56 + 8, 100 + 51 + 8));
    app.qc_release(0);
    assert_eq!(
        app.brewing_slots[0],
        (potion, 5),
        "the bottle seat the place-back takes"
    );
    assert_eq!(
        app.brewing_carried,
        (0, 0),
        "the whole stack the cursor hands over"
    );
    assert!(!app.qc_armed, "the release the arm's own tail clears");

    // ARM B — THE RIGHT'S OWN PLACE-ONE (the split press the armed
    // byte wears — the ONE item the EMPTY seat takes, the remainder
    // the cursor keeps)
    app.brewing_carried = (potion, 5);
    app.qc_armed = true;
    app.qc_button = 1;
    app.qc_slots.clear();
    app.qc_remainder = 0;
    app.mouse_pos = Some((100 + 79 + 8, 100 + 58 + 8));
    app.qc_release(1);
    assert_eq!(
        app.brewing_slots[1],
        (potion, 1),
        "the ONE item the seat takes"
    );
    assert_eq!(
        app.brewing_carried,
        (potion, 4),
        "the remainder the cursor keeps"
    );

    // ARM C — THE OUTSIDE'S OWN WHOLE DROP (the -999 THROW the 339
    // walk serves — the carried the world's own entity takes, the
    // kind the cursor's bank clears)
    app.brewing_carried = (potion, 5);
    app.brewing_carried_kind = 1;
    app.qc_armed = true;
    app.qc_button = 0;
    app.qc_slots.clear();
    app.qc_remainder = 0;
    app.mouse_pos = Some((100 - 10, 100 + 40));
    app.qc_release(0);
    assert_eq!(
        app.brewing_carried,
        (0, 0),
        "the whole drop the cursor clears"
    );
    assert_eq!(
        app.brewing_carried_kind,
        crate::brewing_sim::POTION_KIND_NONE,
        "the drop the kind's own bank clears"
    );

    // ARM D — THE BETWEEN'S OWN NO-OP (the -1 PICKUP the walk keeps —
    // the carried the release never moves, the arm the tail clears)
    app.brewing_carried = (dirt, 5);
    app.qc_armed = true;
    app.qc_button = 0;
    app.qc_slots.clear();
    app.qc_remainder = 0;
    app.mouse_pos = Some((100 + 140, 100 + 35));
    app.qc_release(0);
    assert_eq!(
        app.brewing_carried,
        (dirt, 5),
        "the between the cursor never moves"
    );
    assert!(!app.qc_armed, "the tail the arm clears either way");

    // ARM E — THE LIVE WINDOW'S OWN SILENCE (the outside release the
    // arm never throws — the 424 press lane the drop already served,
    // the offline cursor the release never touches, no packet the
    // release mints)
    app.world_is_network = true;
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.live_clicks = Some(tx);
    app.live_container_updated(Some(crate::world_view::LiveContainer {
        id: 9,
        menu_type_id: crate::world_model::MENU_TYPE_BREWING_STAND,
        slots: vec![(0i32, 0i32); 41],
        bundle_rows: Vec::new(),
        carried: (0, 0),
        state_id: 9,
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    }));
    assert!(app.brewing_is_live, "the live flag the route wears");
    app.brewing_carried = (dirt, 5);
    app.qc_armed = true;
    app.qc_button = 0;
    app.qc_slots.clear();
    app.qc_remainder = 0;
    app.mouse_pos = Some((100 - 10, 100 + 40));
    app.qc_release(0);
    assert_eq!(
        app.brewing_carried,
        (dirt, 5),
        "the live release the throw never wears"
    );
    assert!(
        rx.try_recv().is_err(),
        "the live release no packet mints — the 424 press the drop owns"
    );
}
'''

TEST_NAME = "the_brewing_stand_the_deferred_place_wears"
PART11 = "crates/mc-client/src/screens/tests/part_11.rs"

# --- the test the part_11's EOF appends ---
src = open(PART11, encoding="utf-8").read()
assert TEST_NAME not in src, "the part_11 the round's own test already wears"
assert src.endswith("}\n"), "the part_11's own tail the brace keeps"
open(PART11, "w", encoding="utf-8").write(src + TEST)
check = open(PART11, encoding="utf-8").read()
assert check.startswith(src), "the part_11's own head the append never moves"
assert check.endswith(TEST), "the part_11's own tail the test wears"
print("part_11.rs: the deferred place test appended")

# --- directed_425.py: the 424 text, the rename + the head-insert ---
src = open("scripts/directed_424.py", encoding="utf-8").read()
src = src.replace("autopush-424", "autopush-425")
src = src.replace("scripts/directed_424.py", "scripts/directed_425.py")
src = src.replace(
    'FILTERS = [\n    "the_brewing_stand_the_wire_throw_wears",',
    'FILTERS = [\n    "the_brewing_stand_the_deferred_place_wears",\n'
    '    "the_brewing_stand_the_wire_throw_wears",',
)
assert "the_brewing_stand_the_deferred_place_wears" in src, "the insert the head wears"
open("scripts/directed_425.py", "w", encoding="utf-8").write(src)
print("directed_425.py: written")

# --- fmt_probe_425.py: the 424 shape, the regions re-keyed ---
probe = open("scripts/fmt_probe_424.py", encoding="utf-8").read()
probe = probe.replace("autopush-424", "autopush-425")
probe = probe.replace("fmt_probe_424.py", "fmt_probe_425.py")
old_region = (
    'SCREENS = [\n'
    "    # the brewing live lane's own throw (the table's own wire\n"
    "    # shape the -999 sentinel wears — the 424 round's own edit)\n"
    '    (\n'
    '        "// THE LIVE LANE\'S OWN THROW (the table\'s own wire shape",\n'
    '        "if self.brewing_is_live {",\n'
    '    ),\n'
    ']'
)
new_region = (
    'SCREENS = [\n'
    "    # the five-seat's own deferred place (the empty release the\n"
    "    # brewing canvas owes — the 425 round's own edit)\n"
    '    (\n'
    '        "// THE FIVE-SEAT\'S OWN DEFERRED PLACE",\n'
    '        "if self.screen == ScreenKind::Dispenser {",\n'
    '    ),\n'
    ']'
)
assert old_region in probe, "the throw region the probe's own bytes wear"
probe = probe.replace(old_region, new_region)
assert new_region in probe, "the deferred place region the re-key wears"
open("scripts/fmt_probe_425.py", "w", encoding="utf-8").write(probe)
print("fmt_probe_425.py: written")

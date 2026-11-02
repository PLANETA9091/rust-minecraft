#!/usr/bin/env python3
"""autopush-424 — THE TRIO'S OWN GENERATOR (the dev-215 round's own
mk-lane the round re-wears): the test the part_11's EOF the append
banks, the directed filter the wire throw's own lane the head wears,
the fmt regions the render's own read re-keys (the throw lane the
region the cursor threads — the hopper title's regions the retire
drops). NO sed, NO grep: the pure-Python transforms the -byte trap
refuses. Usage: python3 scripts/mk_424.py"""

TEST = '''
/// THE BREWING STAND'S OWN WIRE THROW (the 424 autopush round — the
/// outside press the live window the THROW input speaks: the -999
/// sentinel the doClick THROW arm the table's own wire shape
/// re-wears — the carried stack the packet rides, the whole stack
/// the LEFT press drops, the one item the RIGHT splits, the belief
/// clears or splits, the drop entity the SERVER spawns). The arms
/// wear: the left's own whole drop, the right's own split, the
/// empty cursor's own silence, the between-slots lane's own
/// silence, the no-window lane's own silence, the no-session
/// lane's own silence, the seats the throw never mints.
#[test]
fn the_brewing_stand_the_wire_throw_wears() {
    use crate::world_view::{ContainerCommand, ContainerInput, LiveContainer};
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();

    // THE ROUTE'S OWN FEED (the menu type the window opens — the
    // identity the throw echoes against)
    let mut app = App::new(3);
    app.enter_world();
    app.world_is_network = true;
    let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.live_clicks = Some(tx);
    app.live_container_updated(Some(LiveContainer {
        id: 7,
        menu_type_id: crate::world_model::MENU_TYPE_BREWING_STAND,
        slots: vec![(0i32, 0i32); 41],
        bundle_rows: Vec::new(),
        carried: (0, 0),
        state_id: 9,
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    }));
    assert!(app.brewing_is_live, "the live flag the route wears");
    assert_eq!(
        app.brewing_live_window,
        Some((7, 9)),
        "the identity the throw echoes against"
    );
    // THE GEOMETRY THE RENDER LAID (the rects + the panel the walk
    // gates keep — the strict rect compare the outside press reads)
    app.brewing_slot_rects = vec![
        (100 + 56 + 1, 100 + 51 + 1, 0i16),
        (100 + 79 + 1, 100 + 58 + 1, 1i16),
        (100 + 102 + 1, 100 + 51 + 1, 2i16),
        (100 + 79 + 1, 100 + 17 + 1, 3i16),
        (100 + 17 + 1, 100 + 17 + 1, 4i16),
    ];
    app.brewing_panel = (100, 100);

    // ARM A — THE LEFT'S OWN WHOLE DROP (the carried stack the
    // packet rides — the belief clears, the -999 the slot wears)
    app.live_brewing_carried = (dirt, 5);
    app.click_brewing_slot(100 - 10, 100 + 40, 0);
    match rx.try_recv().unwrap() {
        ContainerCommand::Click(click) => {
            assert_eq!(click.window_id, 7, "the window id the throw wears");
            assert_eq!(click.state_id, 9, "the revision the echo wears");
            assert_eq!(click.slot, -999, "the outside press the sentinel wears");
            assert_eq!(click.button, 0, "the left press the byte wears");
            assert_eq!(
                click.input,
                ContainerInput::Throw,
                "the throw input the outside speaks"
            );
            assert!(click.changed.is_empty(), "the throw the deltas never mints");
            assert_eq!(click.carried, (0, 0), "the whole drop the packet carries");
        }
        other => panic!("the outside press the throw speaks, not {other:?}"),
    }
    assert_eq!(
        app.live_brewing_carried,
        (0, 0),
        "the whole drop the belief clears"
    );

    // ARM B — THE RIGHT'S OWN SPLIT (the one item the RIGHT drops —
    // the remainder the belief keeps, the packet the split carries)
    app.live_brewing_carried = (dirt, 5);
    app.click_brewing_slot(100 + 180, 100 + 40, 1);
    match rx.try_recv().unwrap() {
        ContainerCommand::Click(click) => {
            assert_eq!(click.slot, -999, "the outside press the sentinel wears");
            assert_eq!(click.button, 1, "the right press the byte wears");
            assert_eq!(click.carried, (dirt, 4), "the split the packet carries");
        }
        other => panic!("the right press the split speaks, not {other:?}"),
    }
    assert_eq!(
        app.live_brewing_carried,
        (dirt, 4),
        "the remainder the belief keeps"
    );

    // ARM C — THE EMPTY CURSOR'S OWN SILENCE (the doClick THROW
    // arm's own isEmpty guard — no packet the press mints)
    app.live_brewing_carried = (0, 0);
    app.click_brewing_slot(100 - 10, 100 + 40, 0);
    assert!(
        rx.try_recv().is_err(),
        "the empty cursor the press stays silent"
    );

    // ARM D — THE BETWEEN-SLOTS LANE'S OWN SILENCE (the press inside
    // the panel the seats never hit — the rect compare the throw
    // refuses, no packet, the cursor the press never moves)
    app.live_brewing_carried = (dirt, 5);
    app.click_brewing_slot(100 + 140, 100 + 35, 0);
    assert!(
        rx.try_recv().is_err(),
        "the between-slots press no packet mints"
    );
    assert_eq!(
        app.live_brewing_carried,
        (dirt, 5),
        "the between-slots press the cursor never moves"
    );

    // ARM E — THE NO-WINDOW LANE'S OWN SILENCE (the identity the
    // retire took — no packet the press mints, the cursor stays)
    app.brewing_live_window = None;
    app.click_brewing_slot(100 - 10, 100 + 40, 0);
    assert!(
        rx.try_recv().is_err(),
        "the no-window press no packet mints"
    );
    assert_eq!(
        app.live_brewing_carried,
        (dirt, 5),
        "the no-window press the cursor never moves"
    );
    app.brewing_live_window = Some((7, 9));

    // ARM F — THE NO-SESSION LANE'S OWN SILENCE (the command the
    // frame thread never serves — the lane the belief never moves)
    app.live_clicks = None;
    app.click_brewing_slot(100 - 10, 100 + 40, 0);
    assert_eq!(
        app.live_brewing_carried,
        (dirt, 5),
        "the no-session press the belief never moves"
    );

    // THE SEATS THE THROW NEVER MINTS (the whole walk the cursor's
    // own lane rides — the five the wholesale bank keeps)
    assert_eq!(
        app.brewing_slots[0],
        (0, 0),
        "the seats the throw never mints"
    );
}
'''

TEST_NAME = "the_brewing_stand_the_wire_throw_wears"
PART11 = "crates/mc-client/src/screens/tests/part_11.rs"

# --- the test the part_11's EOF appends ---
src = open(PART11, encoding="utf-8").read()
assert TEST_NAME not in src, "the part_11 the round's own test already wears"
assert src.endswith("}\n"), "the part_11's own tail the brace keeps"
open(PART11, "w", encoding="utf-8").write(src + TEST)
check = open(PART11, encoding="utf-8").read()
assert check.startswith(src), "the part_11's own head the append never moves"
assert check.endswith(TEST), "the part_11's own tail the test wears"
print("part_11.rs: the wire throw test appended")

# --- directed_424.py: the dev-215 text, the rename + the head-insert ---
src = open("scripts/directed_dev215.py", encoding="utf-8").read()
src = src.replace("dev-215", "autopush-424")
src = src.replace("scripts/directed_dev215.py", "scripts/directed_424.py")
src = src.replace(
    'FILTERS = [\n    "the_hopper_the_wire_title_wears",',
    'FILTERS = [\n    "the_brewing_stand_the_wire_throw_wears",\n'
    '    "the_hopper_the_wire_title_wears",',
)
assert "the_brewing_stand_the_wire_throw_wears" in src, "the insert the head wears"
open("scripts/directed_424.py", "w", encoding="utf-8").write(src)
print("directed_424.py: written")

# --- fmt_probe_424.py: the dev-215 shape, the regions re-keyed ---
probe = open("scripts/fmt_probe_dev215.py", encoding="utf-8").read()
probe = probe.replace("dev-215", "autopush-424")
probe = probe.replace("fmt_probe_dev215.py", "fmt_probe_424.py")
old_region = (
    'SCREENS = [\n'
    "    # the hopper's own title read (the live window's own name the\n"
    "    # chest's own law wears — the None the lang row the offline\n"
    '    # canvas keeps)\n'
    '    (\n'
    '        "THE LIVE WINDOW\'S OWN NAME (the chest\'s own law the",\n'
    '        "if let Some(t) = tr(\\"container.inventory\\")",\n'
    '    ),\n'
    ']'
)
new_region = (
    'SCREENS = [\n'
    "    # the brewing live lane's own throw (the table's own wire\n"
    "    # shape the -999 sentinel wears — the 424 round's own edit)\n"
    '    (\n'
    '        "// THE LIVE LANE\'S OWN THROW (the table\'s own wire shape",\n'
    '        "if self.brewing_is_live {",\n'
    '    ),\n'
    ']'
)
assert old_region in probe, "the hopper region the probe's own bytes wear"
probe = probe.replace(old_region, new_region)
assert new_region in probe, "the throw region the re-key wears"
open("scripts/fmt_probe_424.py", "w", encoding="utf-8").write(probe)
print("fmt_probe_424.py: written")

#!/usr/bin/env python3
"""autopush-383 — the part_11 test insert: THE DRAG'S OWN PLACE
VERDICT (the chest offline fold's own walk, six arms)."""
import sys

PATH = "crates/mc-client/src/screens/tests/part_11.rs"
src = open(PATH, encoding="utf-8").read()

TEST = '''
/// THE DRAG'S OWN PLACE VERDICT (the autopush-383 round — the nine
/// distribute folds' own per-seat gate the same-id kind pair the
/// paint's refusal the release never re-keys): the chest offline fold
/// the forced set the fold's own reads re-key — the mismatched pair
/// the fold's own word refuses the seat the share keeps, the
/// same-kind pair the merge the kind tail retires, the wordless pair
/// the plain law keeps, the empty seat the placed stack's own word
/// the stale word the bank replaces, the right drag the one-each the
/// refusal re-keys, the dirt pair the plain merge the words keep.
#[test]
fn the_drag_place_the_kind_pair_refuses_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(383);
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
    // ARM A — THE FOLD'S OWN REFUSAL (the forced set the fold's own
    // reads re-key: the mismatched pair the distribution's seat gate
    // refuses — the honest seat the merge serves, the refused seat
    // the share the carried keeps)
    app.chest_slots[0] = (potion, 1);
    app.chest_row_potion_kinds[0] = night;
    app.chest_slots[1] = (potion, 1);
    app.chest_row_potion_kinds[1] = swift;
    app.chest_carried = (potion, 10);
    app.chest_carried_potion_kind = night;
    app.qc_arm_chest(gx + 8, gy + 8, 0);
    app.qc_slots = vec![0, 1];
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(0);
    assert_eq!(app.chest_slots[0], (potion, 6), "the share the seat lays");
    assert_eq!(
        app.chest_row_potion_kinds[0], night,
        "the honest seat the word keeps"
    );
    assert_eq!(
        app.chest_slots[1],
        (potion, 1),
        "the refused seat the stack keeps"
    );
    assert_eq!(
        app.chest_row_potion_kinds[1], swift,
        "the refused seat the word keeps"
    );
    assert_eq!(
        app.chest_carried,
        (potion, 5),
        "the one seat the carried's share keeps"
    );
    assert_eq!(
        app.chest_carried_potion_kind, night,
        "the carried's own word the walk keeps"
    );
    // ARM B — THE SAME-KIND PAIR'S OWN MERGE (the fold the seat
    // serves, the whole carried lays, the kind tail the emptiness
    // retires)
    app.chest_slots[0] = (potion, 2);
    app.chest_row_potion_kinds[0] = swift;
    app.chest_carried = (potion, 7);
    app.chest_carried_potion_kind = swift;
    app.qc_arm_chest(gx + 8, gy + 8, 0);
    app.qc_slots = vec![0];
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(0);
    assert_eq!(app.chest_slots[0], (potion, 9), "the merge the seat lays");
    assert_eq!(
        app.chest_row_potion_kinds[0], swift,
        "the same-kind pair the seat word keeps"
    );
    assert_eq!(app.chest_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the emptied carried the kind tail retires"
    );
    // ARM C — THE WORDLESS PAIR'S OWN PLAIN LAW (the NONE words the
    // same item speaks, the fold the merge serves, the words the
    // silence keeps)
    app.chest_slots[0] = (potion, 2);
    app.chest_row_potion_kinds[0] = none;
    app.chest_carried = (potion, 7);
    app.chest_carried_potion_kind = none;
    app.qc_arm_chest(gx + 8, gy + 8, 0);
    app.qc_slots = vec![0];
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(0);
    assert_eq!(
        app.chest_slots[0],
        (potion, 9),
        "the wordless pair the merge lays"
    );
    assert_eq!(
        app.chest_row_potion_kinds[0], none,
        "the wordless pair the seat word keeps"
    );
    // ARM D — THE EMPTY SEAT'S OWN WORD BANK (the stale word an
    // earlier stack left the placed stack's own word replaces — the
    // drag's own place the click lane's own bank law re-wears)
    app.chest_slots[3] = (0, 0);
    app.chest_row_potion_kinds[3] = swift;
    app.chest_carried = (potion, 10);
    app.chest_carried_potion_kind = night;
    app.qc_arm_chest(gx + 8, gy + 8, 0);
    app.qc_slots = vec![3];
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(0);
    assert_eq!(app.chest_slots[3], (potion, 10), "the place the seat lays");
    assert_eq!(
        app.chest_row_potion_kinds[3], night,
        "the placed stack's own word the bank wears"
    );
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the whole carried the seat lays"
    );
    assert_eq!(
        app.chest_carried_potion_kind, none,
        "the emptied carried the kind tail retires"
    );
    // ARM E — THE RIGHT DRAG'S OWN ONE-EACH (the type-1 lane the
    // refusal re-keys: the honest seat the ONE wears, the refused
    // seat the stack the word keeps, the carried the one share keeps)
    app.chest_slots[0] = (potion, 1);
    app.chest_row_potion_kinds[0] = night;
    app.chest_slots[1] = (potion, 1);
    app.chest_row_potion_kinds[1] = swift;
    app.chest_carried = (potion, 4);
    app.chest_carried_potion_kind = night;
    app.qc_arm_chest(gx + 8, gy + 8, 1);
    app.qc_slots = vec![0, 1];
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(1);
    assert_eq!(
        app.chest_slots[0],
        (potion, 2),
        "the one-each the honest seat wears"
    );
    assert_eq!(
        app.chest_row_potion_kinds[0], night,
        "the honest seat the word keeps"
    );
    assert_eq!(
        app.chest_slots[1],
        (potion, 1),
        "the refused seat the stack keeps"
    );
    assert_eq!(
        app.chest_row_potion_kinds[1], swift,
        "the refused seat the word keeps"
    );
    assert_eq!(
        app.chest_carried,
        (potion, 3),
        "the one seat the carried's share keeps"
    );
    assert_eq!(
        app.chest_carried_potion_kind, night,
        "the carried's own word the walk keeps"
    );
    // ARM F — THE DIRT PAIR'S OWN PLAIN MERGE (the non-potion ids the
    // NONE words the plain law keeps, the merge the whole carried
    // lays)
    app.chest_slots[0] = (dirt, 10);
    app.chest_row_potion_kinds[0] = none;
    app.chest_carried = (dirt, 5);
    app.chest_carried_potion_kind = none;
    app.qc_arm_chest(gx + 8, gy + 8, 0);
    app.qc_slots = vec![0];
    app.mouse_pos = Some((gx + 8, gy + 8));
    app.qc_release(0);
    assert_eq!(app.chest_slots[0], (dirt, 15), "the merge the seat lays");
    assert_eq!(app.chest_carried, (0, 0), "the merge the carried empties");
    assert_eq!(
        app.chest_row_potion_kinds[0], none,
        "the dirt row the NONE word keeps"
    );
}'''

marker = (
    '    assert_eq!(\n'
    '        app.chest_carried_potion_kind, none,\n'
    '        "the emptied carried the kind tail retires"\n'
    '    );\n'
    '}'
)
n = src.count(marker)
assert n == 1, f"tail marker count {n} != 1"
src = src.replace(marker, marker + "\n" + TEST)
open(PATH, "w", encoding="utf-8").write(src)
print("[done] test appended — part_11.rs")

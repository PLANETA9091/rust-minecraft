#!/usr/bin/env python3
"""autopush-391 — the part_11 tail's own append (the 390 record's own
lane the round re-wears): the tail the closing brace names, the new
test the block appends, the count the +1 speaks. Usage:
python3 scripts/test_391.py"""
import sys

PATH = "crates/mc-client/src/screens/tests/part_11.rs"

TEST = '''
/// THE WIRE GENERIC'S OWN PLACE WORD BANK (the autopush-391 round —
/// the 390 table's own law the generic's wire tail re-wears, the
/// 382 folds' own reads the write follows): the wire window-0's
/// drag distribute the empty seat the placed stack's own word
/// banks (the mapped row the kind_bank_seat's own mapping answers
/// — the main rows the identity keeps, the hotbar the -36 shift
/// speaks, the offhand the 36 seat wears), the occupied row the
/// merge the original's word keeps, the craft/armor wall the None
/// the write never wears, the pre-retire word the honest order
/// reads (the bank the pre-drag word wears, the drag's own word
/// retire the AFTER the emptied carried the NONE wears).
#[test]
fn the_wire_generic_place_the_kind_word_banks_wears() {
    let potion = mc_registry::item_id_by_name("potion").unwrap();
    let night = 4; // the night_vision row the POTION_KIND_NAMES banks
    let swift = 13; // the swiftness row the same table banks
    let none = crate::brewing_sim::POTION_KIND_NONE;
    let mut app = App::new(391);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    app.world_is_network = true;
    app.live_clicks = Some(std::sync::mpsc::channel().0);
    app.live_inv_state_id = 3;
    assert_eq!(app.screen, ScreenKind::Inventory, "the door opens");
    for k in app.items_potion_kinds.iter_mut() {
        *k = none;
    }
    for s in app.live_inv_slots.iter_mut() {
        *s = (0, 0);
    }
    // ARM A — THE MAIN ROW'S OWN BANK (the empty seat the placed
    // stack's own word banks — the cap-1 place the row the word
    // wears, the remainder the cursor the word keeps)
    app.live_inv_carried = (potion, 5);
    app.live_carried_potion_kind = swift;
    let (ax, ay) = inventory_slot_origin(app.inv_panel, 20);
    app.qc_arm_inventory(ax + 8, ay + 8, 0);
    app.qc_paint(ax + 8, ay + 8);
    assert!(
        app.qc_slots.contains(&20),
        "the empty seat the paint joins"
    );
    app.qc_release(0);
    assert_eq!(
        app.live_inv_slots[20],
        (potion, 1),
        "the cap-1 place the seat wears"
    );
    assert_eq!(
        app.items_potion_kinds[20], swift,
        "the placed stack's own word the row banks"
    );
    assert_eq!(
        app.live_inv_carried,
        (potion, 4),
        "the remainder keeps"
    );
    assert_eq!(
        app.live_carried_potion_kind, swift,
        "the remainder the same stack the word keeps"
    );
    // ARM B — THE HOTBAR SEAT'S OWN BANK (the -36 shift the mapped
    // row answers — the emptied carried the pre-retire word the
    // honest order banks, the tail the NONE the word retires)
    app.live_inv_carried = (potion, 1);
    app.live_carried_potion_kind = night;
    let (bx, by) = inventory_slot_origin(app.inv_panel, 4);
    app.qc_arm_inventory(bx + 8, by + 8, 0);
    app.qc_paint(bx + 8, by + 8);
    assert!(
        app.qc_slots.contains(&40),
        "the hotbar seat the paint joins"
    );
    app.qc_release(0);
    assert_eq!(
        app.live_inv_slots[40],
        (potion, 1),
        "the hotbar seat the place wears"
    );
    assert_eq!(
        app.items_potion_kinds[4], night,
        "the -36 shift the mapped row banks"
    );
    assert_eq!(app.live_inv_carried, (0, 0), "the place empties");
    assert_eq!(
        app.live_carried_potion_kind, none,
        "the emptied carried the word retires"
    );
    // ARM C — THE SPLIT'S OWN BOTH BANKS (the two empty rows the
    // floor split serves, the both mapped words the bank wears,
    // the emptied carried the NONE retires)
    app.live_inv_carried = (potion, 2);
    app.live_carried_potion_kind = night;
    let (cx, cy) = inventory_slot_origin(app.inv_panel, 22);
    let (dx, dy) = inventory_slot_origin(app.inv_panel, 23);
    app.qc_arm_inventory(cx + 8, cy + 8, 0);
    app.qc_paint(cx + 8, cy + 8);
    app.qc_paint(dx + 8, dy + 8);
    assert!(
        app.qc_slots.contains(&22),
        "the first row the paint joins"
    );
    assert!(
        app.qc_slots.contains(&23),
        "the second row the paint joins"
    );
    app.qc_release(0);
    assert_eq!(
        app.live_inv_slots[22],
        (potion, 1),
        "the split's first the seat wears"
    );
    assert_eq!(
        app.live_inv_slots[23],
        (potion, 1),
        "the split's second the seat wears"
    );
    assert_eq!(
        app.items_potion_kinds[22], night,
        "the split's first the word banks"
    );
    assert_eq!(
        app.items_potion_kinds[23], night,
        "the split's second the word banks"
    );
    assert_eq!(app.live_inv_carried, (0, 0), "the split empties");
    assert_eq!(
        app.live_carried_potion_kind, none,
        "the emptied carried the word retires"
    );
    // ARM D — THE CRAFT WALL'S OWN NONE (the craft cell the plain
    // place serves, the mapped rows the kindless wall never touches
    // — the sentinel row the wrong-row write never keys)
    app.live_inv_carried = (potion, 1);
    app.live_carried_potion_kind = night;
    app.items_potion_kinds[2] = swift; // the sentinel row the wall guards
    let (ex, ey) = craft_slot_origin(app.inv_panel, 1);
    app.qc_arm_inventory(ex + 8, ey + 8, 0);
    app.qc_paint(ex + 8, ey + 8);
    assert!(
        app.qc_slots.contains(&2),
        "the craft cell the paint joins"
    );
    app.qc_release(0);
    assert_eq!(
        app.live_inv_slots[2],
        (potion, 1),
        "the craft cell the place serves"
    );
    assert_eq!(
        app.items_potion_kinds[2], swift,
        "the kindless wall the sentinel row keeps"
    );
    assert_eq!(app.live_inv_carried, (0, 0), "the place empties");
    assert_eq!(
        app.live_carried_potion_kind, none,
        "the plain carried the word keeps"
    );
    // ARM E — THE OFFHAND SEAT'S OWN BANK (the 45 menu the 36 seat
    // the bank's own mapping answers — the empty seat the placed
    // stack's own word banks)
    app.live_inv_carried = (potion, 1);
    app.live_carried_potion_kind = swift;
    let (fx, fy) = offhand_slot_origin(app.inv_panel);
    app.qc_arm_inventory(fx + 8, fy + 8, 0);
    app.qc_paint(fx + 8, fy + 8);
    assert!(
        app.qc_slots.contains(&45),
        "the offhand seat the paint joins"
    );
    app.qc_release(0);
    assert_eq!(
        app.live_inv_slots[45],
        (potion, 1),
        "the offhand seat the place wears"
    );
    assert_eq!(
        app.items_potion_kinds[36], swift,
        "the offhand seat the word banks"
    );
    // ARM F — THE REFUSED PAIR'S OWN BANK NEVER (the painted kinded
    // pair the distribute's own gate refuses — the no place the
    // bank never wears, the row's own word the refusal keeps)
    app.live_inv_slots[21] = (potion, 1);
    app.items_potion_kinds[21] = night;
    app.live_inv_carried = (potion, 3);
    app.live_carried_potion_kind = swift;
    app.qc_armed = true;
    app.qc_button = 0;
    app.qc_type = 0;
    app.qc_slots = vec![21];
    app.qc_release(0);
    assert_eq!(
        app.live_inv_slots[21],
        (potion, 1),
        "the kinded pair the seat keeps"
    );
    assert_eq!(
        app.items_potion_kinds[21], night,
        "the refused place the row's word keeps"
    );
    assert_eq!(
        app.live_inv_carried,
        (potion, 3),
        "the refused place the cursor keeps"
    );
    assert_eq!(
        app.live_carried_potion_kind, swift,
        "the refused place the word keeps"
    );
}
'''


def main() -> None:
    with open(PATH, "r", encoding="utf-8") as f:
        lines = f.read().split("\n")
    while lines and lines[-1] == "":
        lines.pop()
    if lines[-1] != "}":
        sys.exit(f"UNEXPECTED TAIL: {lines[-1]!r}")
    joined = "\n".join(lines)
    if "the_wire_generic_place_the_kind_word_banks_wears" in joined:
        sys.exit("TEST ALREADY PRESENT — the append the idempotence keeps")
    body = TEST.split("\n")
    out = lines + body
    with open(PATH, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    with open(PATH, "r", encoding="utf-8") as f:
        check = f.read().split("\n")
    n = sum(1 for line in check if line.startswith("fn the_"))
    tail_fn = [line for line in check if line.startswith("fn the_")][-1]
    print(f"lines now {len(check)}; the_ fns {n}; last {tail_fn.strip()}")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""autopush-392 — THE FMT REGION WALK (the 372..391 record's own
law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the anvil's own kind bank (the seat store decl +
                  ctor + the read/write helpers + the fresh lay + the
                  ride-home word + the close death + the create/copy
                  kind + the empty arms + the take hand + the throw
                  retire + the tooltip read + the click hoist + the
                  four-arm ride + the paint gate + the distribute
                  bank + the wire landing).
  part_11.rs    — the round's own test tail (the anvil kind bank's
                  eight arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the seat store's own decl + the ctor's fresh lay
    ("pub anvil_seat_potion_kinds: [i32; 40],", "pub anvil_is_live: bool,"),
    (
        "anvil_seat_potion_kinds: [crate::brewing_sim::POTION_KIND_NONE; 40],",
        "anvil_is_live: false,",
    ),
    # the read/write helpers
    ("/// THE ANVIL'S OWN SEAT KIND READ", "/// THE ANVIL'S OWN OFFLINE OPEN"),
    # the open's own fresh lay
    ("// THE KIND SEAT'S OWN FRESH LAY", "self.anvil_mirror_sync_from_inventory();"),
    # the close walk's result death
    ("// THE RESULT'S OWN DEATH", "// THE FIELD'S OWN WORDS"),
    # the ride-home's own word (the head + the first-empty + the tail)
    ("// the seat's own word the ride wears", "let kind = self.anvil_seat_potion_kind(seat);"),
    (
        "self.inventory_enchants[idx] = self.anvil_enchants[seat].clone();",
        "self.items_potion_kinds[idx] = kind;",
    ),
    ("self.anvil_enchants[seat].clear();", "/// THE COST WALK"),
    # the create's empty gate + the copy kind + the arms + the tail
    ("// pc 31-64 — the EMPTY gate", "let item2 = self.anvil_slots[1];"),
    (
        "// the copy's own kind (the input 0's own word the copy",
        "let copy_kind = self.anvil_seat_potion_kind(0);",
    ),
    ("// pc 143-335", "if has_stored {"),
    ("// pc 726-755 — the all-incompatible walk", "// pc 756-845 — the RENAME walk"),
    ("// pc 923-950 — the TOO-EXPENSIVE GATE", "// pc 950-1062"),
    ("self.anvil_names[2] = copy_name;", "self.anvil_result = item1; // the copy's own (id, count)"),
    # the take's own hand
    ("fn anvil_take_result(&mut self) {", "// the fresh compute (the broadcastChanges walk)"),
    # the tooltip's own read
    ("the potion kind the seat store's own word answers", "fn anvil_item_tooltip_hover"),
    ("let ench = self.anvil_seat_enchants(s);", "if let Some(name) = self.anvil_seat_name(s) {"),
    # the throw's own retire
    (
        "// the carried's own word the emptied cursor retires",
        "/// THE ANVIL'S OWN CLICK LANE",
    ),
    # the click lane's kind reads + the four-arm ride
    (
        "// THE KIND'S OWN READS (the hoist the component law pre-keys —",
        "let (new_slot, new_carried) = if button == 0 {",
    ),
    ("// THE KIND'S OWN FOUR-ARM RIDE", "self.anvil_seat_potion_kinds[39] = new_carried_kind;"),
    # the paint's own component half
    (
        "// THE COMPONENT HALF (the 382 chest's own law the anvil's own",
        "/// THE ANVIL DRAG'S OWN DISTRIBUTION",
    ),
    # the distribute's own bank
    ("fn qc_distribute_anvil_offline(&mut self) {", "self.anvil_create_result();"),
    # the wire landing's own NONE wall
    (
        "// the wire's own word (the live lane the honest NONE",
        "if c.menu_type_id != crate::world_model::MENU_TYPE_CHEST",
    ),
]

PART11 = [
    # the round's own test tail
    ("/// THE ANVIL'S OWN KIND BANK (the autopush-392 round —", None),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/screens/tests/part_11.rs": PART11,
}


def find(text: str, marker: str, start: int = 0) -> int:
    i = text.find(marker, start)
    if i < 0:
        raise SystemExit(f"MARKER NOT FOUND: {marker!r}")
    return i


def region_end(text: str, marker: str, start_at: int) -> int:
    if marker is None:
        return len(text)
    return find(text, marker, start_at)


def main() -> int:
    ok = True
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        proc = subprocess.run(
            [RUSTFMT, "--emit", "stdout", path],
            capture_output=True, text=True)
        if proc.returncode != 0:
            print(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
            ok = False
            continue
        formatted = proc.stdout
        sm = difflib.SequenceMatcher(None, original.splitlines(), formatted.splitlines())
        hunks = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == "equal":
                continue
            hunks.append((i1, i2, tag, j1, j2))
        cursor = 0  # the sequential threading — the shared texts the order keys
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker, cursor)
            e = region_end(original, end_marker, s)
            cursor = e
            s_line = original[:s].count("\n")
            e_line = original[:e].count("\n")
            bad = [h for h in hunks if h[0] < e_line and h[1] > s_line]
            label = start_marker.strip()[:56].replace("\n", " | ")
            if bad:
                ok = False
                print(f"{path}: REGION NOT FMT-CLEAN [{label}] hunks={bad}")
            else:
                print(f"{path}: region fmt-clean [{label}]")
    if ok:
        print("ALL-REGIONS-CLEAN")
    else:
        print("REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""autopush-384 — THE FMT REGION WALK (the 372..383 record's own law
the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the five wire distribute fns' own kind hoists (the
                  doc + signature + component gate: the generic, chest,
                  dispenser, table, furnace lanes) + the five callers'
                  own kind reads (the closures + the hoisted words).
  part_11.rs    — the round's own test tail (the wire folds).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

HALF = "            // THE COMPONENT HALF (the autopush-384 round — the wire\n            // fold's own kind hoist the same-id kind pair the seat\n            // refuses): "

SCREENS = [
    # the chest caller's own kind reads (the closure + the hoisted words)
    ("let chest_kinds = &self.chest_row_potion_kinds;",
     "        if self.chest_carried.1 <= 0 {"),
    # the dispenser caller's own kind reads
    ("let items_kinds = &self.items_potion_kinds;\n        let carried_kind = self.dispenser_carried_potion_kind;",
     "        if self.dispenser_carried.1 <= 0 {"),
    # the generic caller's own kind reads
    ("let items_kinds = &self.items_potion_kinds;\n        let carried_kind = self.live_carried_potion_kind;",
     "        if self.live_inv_carried.1 <= 0 {"),
    # the table caller's own NONE wall
    ("let carried_kind = crate::brewing_sim::POTION_KIND_NONE;\n        let seat_kinds = |_menu: usize| crate::brewing_sim::POTION_KIND_NONE;\n        wire_table_quickcraft_distribute(",
     "    /// THE FURNACE'S OFFLINE DISTRIBUTION (the doClick QUICK_CRAFT"),
    # the furnace caller's own NONE wall
    ("let carried_kind = crate::brewing_sim::POTION_KIND_NONE;\n        let seat_kinds = |_menu: usize| crate::brewing_sim::POTION_KIND_NONE;\n        wire_furnace_quickcraft_distribute(",
     "    /// The LEFT press's rising edge (the vanilla MouseHandler's own"),
    # the generic wire fn's own hoist (doc + signature)
    ("/// hotbar menus' sync follows. THE WIRE FOLD'S OWN KIND HOIST (the",
     "    let start_button = ((qc_type & 3) << 2) as i8; // getQuickcraftMask(0, type)"),
    # the generic wire fn's own component gate
    (HALF + "the caller's own kind reads the closure wears,\n            // the potion pair the gate answers (the kindless pairs the\n            // NONE words the plain law keeps)",
     "            if !quickcraft_seat_may_place(menu, carried.0) {"),
    # the chest wire fn's own hoist (doc + signature)
    ("/// chest swap walk banks). THE WIRE FOLD'S OWN KIND HOIST (the",
     "    let start_button = ((qc_type & 3) << 2) as i8; // getQuickcraftMask(0, type)"),
    # the chest wire fn's own component gate
    (HALF + "the caller's own kind reads the closure wears,\n            // the potion pair the gate answers (the kindless pairs the\n            // NONE words the plain law keeps)",
     "            if qc_type != 2 && carried.1 < painted as i32 {"),
    # the dispenser wire fn's own hoist (doc + signature)
    ("/// the chest swap walk banks). THE WIRE FOLD'S OWN KIND HOIST (the",
     "    let start_button = ((qc_type & 3) << 2) as i8; // getQuickcraftMask(0, type)"),
    # the dispenser wire fn's own component gate
    (HALF + "the caller's own kind reads the closure wears,\n            // the potion pair the gate answers (the kindless pairs the\n            // NONE words the plain law keeps)",
     "            if qc_type != 2 && carried.1 < painted as i32 {"),
    # the table wire fn's own hoist (doc + signature)
    ("/// menus 37..45 the sync follows. THE WIRE FOLD'S OWN KIND HOIST (the",
     "    let start_button = ((qc_type & 3) << 2) as i8; // getQuickcraftMask(0, type)"),
    # the table wire fn's own component gate
    (HALF + "the live table lane's words the NONE wall\n            // keeps (the carried word the sync never banks), the gate\n            // the plain verdict wears",
     "            if !quickcraft_table_seat_may_place(menu) {"),
    # the furnace wire fn's own hoist (doc + signature)
    ("/// HUD row the hotbar menus 30..38 the sync follows. THE WIRE FOLD'S",
     "    let start_button = ((qc_type & 3) << 2) as i8; // getQuickcraftMask(0, type)"),
    # the furnace wire fn's own component gate
    (HALF + "the live furnace lane's words the NONE wall\n            // keeps (the carried word never banks), the gate the\n            // plain verdict wears",
     "            if !quickcraft_furnace_seat_may_place(menu, carried.0, kind) {"),
]

PART11 = [
    # the round's own test tail
    ("/// THE WIRE FOLDS' OWN KIND HOIST (autopush-384 — the five wire",
     None),
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
        cursor = 0  # the sequential threading — the same-text gates the order keys
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

#!/usr/bin/env python3
"""autopush-383 — THE FMT REGION WALK (the 372..382 record's own law
the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the seven offline folds' component gates + the
                  place word banks (the chest, dispenser, brewing,
                  enchant, table, furnace, generic lanes).
  part_11.rs    — the round's own test tail (the drag place).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

HALF = (
    "            // THE COMPONENT HALF (the autopush-383 round — the same-id\n"
    "            // kind pair the seat's own word the place refuses): "
)

SCREENS = [
    # the chest gate (the container rows + the store rows)
    (HALF + "the\n            // container rows",
     "        // the mayPlace verdict: the chest menu's slots ALL plain"),
    # the chest word bank
    ("        // THE PLACE'S OWN WORD BANK (the autopush-383 round — the\n"
     "            // drag's own word ride the chest rows re-wear",
     "        self.chest_carried = if k > 0"),
    # the dispenser gate (the grid nine + the mirror rows)
    (HALF + "the\n            // grid nine the",
     "        // the mayPlace verdict: the dispenser menu's slots ALL"),
    # the dispenser word bank
    ("        // THE PLACE'S OWN WORD BANK (the autopush-383 round — the\n"
     "            // empty mirror row",
     "        self.dispenser_carried = if k > 0"),
    # the brewing gate (the stand seats + the mirror rows)
    (HALF + "the\n            // stand seats the",
     "            if self.qc_type != 2 && carried.1 < painted as i32 {"),
    # the brewing word bank
    ("        // THE PLACE'S OWN WORD BANK (the autopush-383 round — the\n"
     "            // empty seat the placed stack's own word banks [the stand",
     "        self.brewing_carried = if k > 0"),
    # the enchant gate (the kindless pair)
    (HALF + "the\n            // pair seats the",
     "            if self.qc_type != 2 && carried.1 < painted as i32 {"),
    # the table gate (the grid cells + the store rows)
    (HALF + "the\n            // grid cells the",
     "            if !quickcraft_table_seat_may_place(menu) {"),
    # the table store word bank
    ("        // THE STORE'S OWN PLACE BANK (the autopush-383 round — the\n"
     "            // drag's own word ride the store rows re-wear: the empty\n"
     "            // row the placed stack's own word banks, the occupied row\n"
     "            // the merge the original's word keeps)\n"
     "            if seat_count == 0 && menu >= 10 {",
     "        self.inv_carried = if k > 0 { (carried.0, k) } else { (0, 0) }; // pc 520-530 — the remainder the cursor keeps"),
    # the furnace gate (the trio + the store rows)
    (HALF + "the\n            // trio the",
     "            if !quickcraft_furnace_seat_may_place(menu, carried.0, kind) {"),
    # the furnace store word bank
    ("        // THE PLACE'S OWN WORD BANK (the autopush-383 round — the\n"
     "            // empty store row",
     "        self.furnace_carried = if k > 0"),
    # the generic gate (the store rows + the craft cells)
    (HALF + "the\n            // store rows the bank's own word reads",
     "            if !self.inventory_seat_may_place(hover, carried.0) {"),
    # the generic store/offhand word bank
    ("        // THE STORE'S OWN PLACE BANK (the autopush-383 round — the\n"
     "            // drag's own word ride the store rows re-wear: the empty\n"
     "            // row the placed stack's own word banks, the occupied row\n"
     "            // the merge the original's word keeps, the craft cells the",
     "        self.inv_carried = if k > 0 { (carried.0, k) } else { (0, 0) }; // pc 520-530 — the remainder the cursor keeps (the empty the (0,0) law speaks)"),
]

PART11 = [
    # the round's own test tail
    ("/// THE DRAG'S OWN PLACE VERDICT (the autopush-383 round — the nine",
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
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker)
            e = region_end(original, end_marker, s)
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

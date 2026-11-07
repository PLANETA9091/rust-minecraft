#!/usr/bin/env python3
"""autopush-492 — THE FMT REGION WALK (the 485..491 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions (screens.rs, file order):
  the craft cells' own data seats (the fields doc + the ctor rows),
  the on-take emptied-cell resets, the click's own grid ride,
  the render loop's own glint seat.

part_11.rs — the round's own tests (the three new craft-cell tests
the tail wears, EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the fields doc (the table cells' own wear + ench seats)
    (
        "    /// THE TABLE CELLS' OWN WEAR SEATS (the autopush-492 round — the",
        "    /// The table grid's computed result slot (the CraftingMenu's slot",
    ),
    # the ctor rows
    (
        "            table_grid: [(0, 0); 9],",
        "            table_result: (0, 0),",
    ),
    # the close's own data ledger + the ride home + the bank's own clear
    (
        "        // THE CELLS' OWN DATA LEDGER (the autopush-492 round — the 491",
        "        self.hotbar_slots = self.inventory.hotbar();",
    ),
    # the on-take emptied-cell resets
    (
        "                        // THE EMPTIED CELL'S OWN RESET (the autopush-492",
        "                    apply_remainder_cell(cell, *rem)",
    ),
    # the click's own grid ride
    (
        "        // THE DATA'S OWN GRID RIDE (the autopush-492 round — the 229",
        "        self.table_grid[cell] = new_cell;",
    ),
    # the render loop's own glint seat
    (
        "            // THE TABLE CELLS' OWN GLINT (the autopush-492 round — the",
        "            if count > 1 {",
    ),
]

PART11 = [
    # the round's own tests (the three new table-cell tests the tail wears)
    (
        "/// THE TABLE CELLS' OWN DATA SEATS (the autopush-492 round",
        None,
    ),
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
        sm = difflib.SequenceMatcher(
            None, original.splitlines(), formatted.splitlines())
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

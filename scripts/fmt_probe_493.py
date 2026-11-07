#!/usr/bin/env python3
"""autopush-493 — THE FMT REGION WALK (the 485..492 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the table cell's own shift arm, the craft cell's own
  shift arm, the table store-row fill arm.
  world_view.rs — the grid fill walk (the 510 census's own doc).
  part_11.rs — the round's own tests (the three new grid-shift tests
  the tail wears, EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the table cell's own doc + the shift arm
    (
        "    /// The table cell's click (the doClick PICKUP/QUICK_MOVE walk's",
        "        let believed = self.table_grid[cell];",
    ),
    # the table store-row fill arm (the 510 census's own first lane)
    (
        "            // THE GRID'S OWN FILL ARM (the 510 census — CraftingMenu.",
        "            let changed = crate::world_view::predict_quick_move_inventory(",
    ),
    # the craft cell's own doc + the shift arm
    (
        "    /// The craft cell's click (the doClick PICKUP/QUICK_MOVE walk's",
        "        let believed = self.craft_grid[cell];",
    ),
]

WORLD = [
    # the grid fill walk (the new predictor)
    (
        "/// THE GRID'S OWN FILL WALK (the 510 census — CraftingMenu.",
        "/// THE FURNACE'S OWN QUICK-MOVE (javap 26.2",
    ),
]

PART11 = [
    # the round's own tests (the three new grid-shift tests the tail wears)
    (
        "/// THE GRID'S OWN SHIFT RIDE (the autopush-493 round",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/world_view.rs": WORLD,
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

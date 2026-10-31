#!/usr/bin/env python3
"""autopush-382 — THE FMT REGION WALK (the 372..381 record's own law
the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the free fn (the collect verdict's own body); the
                  seven paint gates' component halves (the generic +
                  the chest + the dispenser + the brewing + the
                  enchant + the table + the furnace lanes).
  part_11.rs    — the round's own test tail (the drag collect).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

HALF = "        // THE COMPONENT HALF (the autopush-382 round — the same-id\n        // kind pair the seat's own word refuses): "

SCREENS = [
    # the free fn's own body
    ("/// THE DRAG'S OWN COLLECT VERDICT (the autopush-382 round — the\n/// shouldAdd's canItemQuickReplace",
     "/// THE CHEST DRAG'S OWN WIRE DISTRIBUTION"),
    # the generic window-0 gate (the store rows + the craft cells)
    (HALF + "the store rows the",
     "        if !self.inventory_seat_may_place(hover, carried.0) {"),
    # the chest gate (the container rows + the store rows)
    (HALF + "the container rows\n",
     "        // the mayPlace verdict: the chest menu's slots ALL plain"),
    # the dispenser gate (the grid nine + the mirror rows)
    (HALF + "the grid nine the",
     "        // the mayPlace verdict: the dispenser menu's slots ALL plain"),
    # the brewing gate (the stand seats + the mirror rows)
    (HALF + "the stand seats the",
     "    /// THE ENCHANTING DRAG'S OWN ARM"),
    # the enchant gate (the kindless pair)
    (HALF + "the pair seats the",
     "    /// THE TABLE'S OWN SEAT CLASSIFICATION"),
    # the table gate (the grid cells + the store rows, the wire wall)
    (HALF + "the grid cells the",
     "        if !quickcraft_table_seat_may_place(menu) {"),
    # the furnace gate (the trio + the store rows, the wire wall)
    (HALF + "the trio the",
     "        if !quickcraft_furnace_seat_may_place(menu,"),
]

PART11 = [
    # the round's own test tail
    ("/// THE DRAG'S OWN COLLECT VERDICT (the autopush-382 round — the\n/// shouldAdd's canItemQuickReplace",
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

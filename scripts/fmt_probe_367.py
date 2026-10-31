#!/usr/bin/env python3
"""autopush-367 — THE FMT REGION WALK (the 323/204/358/360/361/362/363/
364/365/366 record's own law the round wears): the whole-file rustfmt
the ORIGINAL text the formatted stdout the diff answers — the
MY-REGIONS verdict the marker ranges keep (no hunk may touch the
round's own edits).

Regions:
  screens.rs  — the grid kind bank's own field + doc, the click cell's
                own four-arm ride, the close's own ride homes (the
                carried return + the clearContainer drain), the render
                grid's own tint.
  part_11.rs  — the round's own test tail (the grid bank walk).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the bank field + its doc
    ("    /// THE TABLE GRID'S OWN KIND BANK (the autopush-367 round — the",
     "    /// The table grid's computed result slot (the CraftingMenu's slot"),
    # the click cell's own four-arm ride
    ("    fn click_table_cell(&mut self, cell: usize, button: i8) {",
     "    /// The TABLE RESULT slot's click (the CraftingMenu's slot 0 — the"),
    # the close's own ride homes
    ("        // the carried's own return (the super.removed walk)",
     "        self.table_result = (0, 0);"),
    # the render grid's own tint
    ("            let (sx, sy) = table_craft_origin(panel, cell);\n            // THE GRID'S OWN TINT (the autopush-367 round — the 340 law",
     "        {\n            let (id, count) = if live {\n                self.live_table_slots[0]"),
]

PART11 = [
    # the round's own test tail
    ("/// THE TABLE GRID'S OWN KIND BANK (the autopush-367 round — the",
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
            label = start_marker.strip()[:56]
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

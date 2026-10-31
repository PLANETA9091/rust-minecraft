#!/usr/bin/env python3
"""autopush-368 — THE FMT REGION WALK (the 323/204/358/360/361/362/363/
364/365/366/367 record's own law the round wears): the whole-file
rustfmt the ORIGINAL text the formatted stdout the diff answers — the
MY-REGIONS verdict the marker ranges keep (no hunk may touch the
round's own edits).

Regions:
  screens.rs  — the craft cells' own bank field + doc, the Bin clear's
                own four seats, the click cell's own four-arm ride, the
                render cells' own tint.
  part_11.rs  — the round's own test tail (the cells' bank walk).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the bank field + its doc
    ("    /// THE INVENTORY CRAFT CELLS' OWN KIND BANK (the autopush-368 round —",
     "    /// The craft grid's computed result slot (the InventoryMenu's slot"),
    # the Bin clear's own four seats
    ("                    for cell in self.craft_grid.iter_mut() {",
     "                    for k in self.items_potion_kinds.iter_mut().take(36) {"),
    # the click cell's own four-arm ride
    ("    fn click_craft_cell(&mut self, cell: usize, button: i8) {",
     "    /// The armor column's click (the doClick PICKUP arm over an"),
    # the render cells' own tint
    ("            let (sx, sy) = craft_slot_origin(panel, cell);\n            // THE CRAFT CELLS' OWN TINT (the autopush-368 round — the 340",
     "        {\n            let (id, count) = if self.world_is_network {\n                self.live_inv_slots[0]"),
]

PART11 = [
    # the round's own test tail
    ("/// THE INVENTORY CRAFT CELLS' OWN KIND BANK (the autopush-368 round —",
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

#!/usr/bin/env python3
"""autopush-361 — THE FMT REGION WALK (the 323/204/358/360 record's own
law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits), the
pre-existing static noise the HEAD law confirms the non-mine hunks
the honest inventory banks.

Regions:
  screens.rs  — the chest walk's four-arm block, the offline held-stack
                ghost pair (the plain chest's new block + the hopper's
                tinted tail), the ride-home bank, the field doc.
  part_11.rs  — the round's own test tail (the chest seat helper + the
                chest carried word walk).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the chest walk's own cursor ride (the four-arm block)
    ("        // THE KIND'S OWN CURSOR RIDE (the dev-204 round's law the\n        // chest's own walk re-wears",
     "        if menu < n {\n            self.chest_slots[menu] = new_slot;"),
    # the plain chest's own offline held stack (the new ghost + tint)
    ("        // THE OFFLINE HELD STACK (the sim's own local cursor the same",
     "        // THE ITEM TOOLTIP (the extractTooltip pass — the dev-203 round"),
    # the hopper's own offline ghost tail (the tint gate re-wear)
    ("        // THE HELD STACK follows the cursor (the vanilla carried\n        // render — the offline sim's local cursor the chest's own gate\n        // keeps, the headless scenes track no mouse; THE KIND'S OWN\n        // CARRIED TINT the 358 law the chest's own ghost re-wears",
     "        // THE ITEM TOOLTIP (the extractTooltip pass — the autopush-353"),
    # the ride home's own word bank
    ("        // THE WORD'S OWN RIDE HOME (the close_inventory law the chest's",
     "        self.hotbar_slots = self.inventory.hotbar();\n    }"),
    # the carried word's own field doc
    ("    /// THE CHEST CARRIED'S OWN WORD (the autopush-361 round",
     "    /// THE BARREL LID'S OWN CELL (the dev-65 round"),
]
PART11 = [
    ("THE CHEST SEAT'S OWN SYNTHETIC RECT (the part-1 harness's own", None),
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

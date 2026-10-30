#!/usr/bin/env python3
"""autopush-356 — THE FMT REGION WALK (the 323 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits), the pre-existing
static noise the HEAD law confirms the non-mine hunks the honest
inventory banks.

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the four walk bodies (the kind rides the 356 round wears — the
    # furnace's own region the chest's the hopper's the dispenser's
    # the same marker pair banks)
    ("    /// THE FURNACE'S OWN TOOLTIP SEAT (the dev-203 round — the",
     "    /// THE CHEST'S OWN HOVER SEAT (the 329 round —"),
    ("    /// THE CHEST'S OWN TOOLTIP SEAT (the dev-203 round — the 351/352",
     "    /// THE HOPPER'S OWN TOOLTIP SEAT (the autopush-353 round — the"),
    ("    /// THE HOPPER'S OWN TOOLTIP SEAT (the autopush-353 round — the",
     "    /// THE DISPENSER'S OWN HOVER SEAT (the 331 round —"),
    ("    /// THE DISPENSER'S OWN TOOLTIP SEAT (the dev-203 round — the",
     "    /// THE BREWING STAND'S OWN HOVER SEAT (the autopush-332 round — the"),
]
PART11 = [
    ("THE MIRROR KIND RIDE'S FAMILY RETROFIT (the autopush-356 round", None),
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

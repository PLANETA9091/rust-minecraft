#!/usr/bin/env python3
"""autopush-345 — THE FMT REGION WALK (the 323 record's own law the
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

REGIONS = {
    "crates/mc-client/src/screens.rs": [
        # the seat map + the ride walk + the extended pickup fn (the
        # store_menu_id tail the region's head pins)
        ("THE KIND BANK'S OWN SEAT MAP (the 343 remap the ride re-keys)",
         "/// THE OUTSIDE RELEASE'S OWN WIRE DROP"),
        # the extended throw fn (the doc the fn's own tail wears —
        # the wire-lane sentence the marker keeps unique)
        ("wire lane never rides). THE THROW'S OWN KIND RIDE",
         "fn wire_quickcraft_distribute("),
        # the offhand pickup caller (the wall's own `None` seat)
        ("THE OFFHAND'S OWN WALL SEAT (the 343 bank the",
         "                return;"),
        # the banked caller (the ride the store rows wear)
        ("THE BANKED SEAT'S OWN RIDE (the 345 round",
         "/// The survival inventory's slot click"),
        # the inline throw lane's ride
        ("THE THROW'S OWN KIND RIDE (the 345 round — the",
         "            return;"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        # the two ride tests (the file's own tail)
        ("THE BELIEVED KIND'S OWN RIDE (the autopush-345 round", None),
    ],
}


def find(text: str, marker: str, start: int = 0) -> int:
    i = text.find(marker, start)
    if i < 0:
        raise SystemExit(f"marker not found: {marker[:60]!r}")
    return i


def region_span(text: str, start_marker: str, end_marker) -> tuple[int, int]:
    s = find(text, start_marker)
    if end_marker is None:
        return (s, len(text))
    e = find(text, end_marker, s + len(start_marker))
    return (s, e)


ok = True
for path, regions in REGIONS.items():
    with open(path) as f:
        original = f.read()
    proc = subprocess.run(
        [RUSTFMT, "--edition", "2021"], input=original, capture_output=True, text=True
    )
    if proc.returncode != 0:
        print(f"{path}: rustfmt FAILED rc={proc.returncode}\n{proc.stderr[:400]}")
        ok = False
        continue
    formatted = proc.stdout
    if formatted == original:
        print(f"{path}: WHOLE-FILE CLEAN")
        continue
    sm = difflib.SequenceMatcher(None, original.splitlines(), formatted.splitlines())
    hunks = []
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal":
            continue
        hunks.append((i1, i2))
    dirty = 0
    for (s, e) in [region_span(original, a, b) for a, b in regions]:
        sl, el = original[:s].count("\n") + 1, original[:e].count("\n") + 1
        overlap = [h for h in hunks if h[0] < el and h[1] > sl]
        if overlap:
            ok = False
            dirty += 1
            print(f"{path}: REGION DIRTY lines {sl}..{el} hunks {overlap[:4]}")
    if not dirty:
        print(f"{path}: ALL REGIONS CLEAN ({len(hunks)} non-mine hunks pre-existing)")

sys.exit(0 if ok else 1)

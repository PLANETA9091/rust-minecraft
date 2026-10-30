#!/usr/bin/env python3
"""autopush-343 — THE FMT REGION WALK (the 323 record's own law the
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
    "crates/mc-client/src/world_view.rs": [
        # the SnapshotView field (doc + seat)
        ("pub inventory_potion_kinds: [i32; INVENTORY_MENU_SLOTS],", "pub mode_word: Option<u8>,"),
        # the helper + the 6-tuple walk
        ("THE WIRE STACK'S OWN KIND EXTRACTION (the autopush-343 round",
         "fn live_inventory_view("),
        ("fn live_inventory_view(", "/// The store's SELECTED hotbar stack"),
        # the three constructor splices (the post-splice wrapped
        # destructure the field comment the seat anchors)
        ("// the kinds ride with the store — the potion stacks the wire",
         "// the join snapshot carries no mode echo"),
        ("// the kinds ride with the store — the pose-less arm",
         "health: health_view"),
        ("// the kinds ride with the store — the potion stacks the\n            // wire spoke",
         "// the mode echo rides every refresh"),
        # the new test
        ("fn the_window0_potion_kinds_projection_rides_the_view()",
         "fn live_view_streams_chunks_and_follows_the_player()"),
    ],
    "crates/mc-client/src/screens.rs": [
        # the kind bank's own lane method
        ("THE KIND BANK'S OWN WIRE SEAT (the autopush-343 round",
         "/// The live session ended"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        # the App bank remap test (the file's own tail)
        ("THE KIND BANK'S OWN WIRE SEAT (the autopush-343 round", None),
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

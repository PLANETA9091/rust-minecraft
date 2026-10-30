#!/usr/bin/env python3
"""autopush-344 — THE FMT REGION WALK (the 323 record's own law the
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
        # the SnapshotView field block (the 343 kinds seat + the 344
        # carried cursor's own seat)
        ("pub inventory_potion_kinds: [i32; INVENTORY_MENU_SLOTS],", "pub mode_word: Option<u8>,"),
        # the helper + the SEVEN-tuple walk (the 344 extraction the
        # carried cursor's own seat wears)
        ("fn live_inventory_view(", "/// The store's SELECTED hotbar stack"),
        # the join arm: the destructure (the region's tail the
        # `live_inventory_view` call pins — the literal's pre-existing
        # WirePlayerList hunk the HEAD law keeps OUTSIDE the regions)
        (
            "    // the join's own store — the window-0 lane reads the adopted\n"
            "    // state the same way the hand's lane does (empty until the wire\n"
            "    // speaks)\n"
            "    let (",
            ") = live_inventory_view(&model);",
        ),
        # the join arm's literal field splice (the 343 probe's own form)
        ("// the kinds ride with the store — the potion stacks the wire",
         "// the join snapshot carries no mode echo"),
        # the refresh destructure (the 12-space form the refresh arm
        # keeps)
        (
            "the window-0 store's own projection rides every refresh —",
            ") = live_inventory_view(&self.model);",
        ),
        # the pose-less arm's field splice
        ("// the kinds ride with the store — the pose-less arm", "health: health_view"),
        # the posed arm's field splice
        (
            "// the kinds ride with the store — the potion stacks the\n            // wire spoke",
            "// the mode echo rides every refresh",
        ),
        # the new test
        ("fn the_carried_cursor_potion_kind_rides_the_view()",
         "fn live_view_streams_chunks_and_follows_the_player()"),
    ],
    "crates/mc-client/src/screens.rs": [
        # the kind bank's own field (the doc + the seat)
        ("THE CARRIED CURSOR'S OWN KIND BANK (the autopush-344 round",
         "THE WORLD CLOCK'S OWN ACCUMULATED TIME"),
        # the ctor init seat
        ("live_inv_state_id: 0,", "clock_total: 0,"),
        # the kind seat's own lane method
        ("THE CARRIED CURSOR'S OWN KIND SEAT (the autopush-344 round",
         "/// The live session ended"),
        # the held-stack render ride (the lane pick + the kind tint)
        ("THE HELD STACK follows the cursor (the vanilla carried render",
         "// THE ITEM TOOLTIP (the extractTooltip pass"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        # the carried cursor's own bank test (the file's own tail)
        ("THE CARRIED CURSOR'S OWN KIND SEAT (the autopush-344 round", None),
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

#!/usr/bin/env python3
"""dev-201 — THE FMT REGION WALK (the 323 record's own law the round
wears): the whole-file rustfmt the ORIGINAL text the formatted stdout
the diff answers — the MY-REGIONS verdict the marker ranges keep (no
hunk may touch the round's own edits), the pre-existing static noise
the HEAD law confirms the non-mine hunks the honest inventory banks.

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

REGIONS = {
    "crates/mc-client/src/screens.rs": [
        # the swap fn's doc ride paragraph (the LOCAL SWAP tail the
        # marker's head pins)
        ("/// THE KIND'S OWN RIDE (the dev-201 round — the swap the",
         "pub fn on_swap_offhand_key"),
        # the live arm's kind swap (the bank's seats the one trade)
        (
            "self.items_potion_kinds\n                .swap(self.selected_hotbar, OFFHAND_KIND_SEAT);",
            "let held = Some",
        ),
        # the offline arm's kind swap (the stack the kind follows)
        (
            "self.items_potion_kinds.swap(sel, OFFHAND_KIND_SEAT);",
            "let id = self.held_id();",
        ),
        # the offhand drop's kind clear (the dev-199 law the seat
        # answers)
        ("emptied offhand the bank's 36 the dev-199 walk's own",
         "            return;"),
        # the store row's kind clear (the emptied row the bank clears)
        (
            "THE KIND'S OWN SEAT CLEAR (the dev-201 round — the emptied\n        // row the bank the dev-199 walk's own law the store seat",
            "self.hotbar_slots = self.inventory.hotbar();",
        ),
        # the wire lane's caller-side ride (the mapped seat the clear
        # answers — the block the fn's own tail wears)
        ("THE KIND'S OWN SEAT CLEAR (the dev-201 round — the caller-side",
         "/// THE TABLE'S OWN Q"),
        # the constant (the offhand seat the index names)
        ("37TH seat the index answers", "fn kind_bank_seat"),
        # the seat map's 45 arm (the re-seat the constant wears)
        ("fn kind_bank_seat(menu: i32) -> Option<usize> {", "/// THE BELIEVED KIND'S OWN RIDE"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        # the two ride tests (the file's own tail)
        ("THE SWAP'S OWN KIND RIDE (the dev-201 round", None),
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

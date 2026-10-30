#!/usr/bin/env python3
"""autopush-346 — THE FMT REGION WALK (the 323 record's own law the
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
        # the App field's doc + the 37-wide array (the fill-walk doc's
        # tail the marker's head pins)
        ("The 37TH SEAT\n    /// (the autopush-346 round",
         "pub items_potion_kinds: [i32; 37],"),
        # the SavedPlayer field's doc + the 37-wide array
        ("pub potion_kinds: [i32; 37],", "pub selected_hotbar: i32,"),
        # the parse walk's cap + the take(37) ride
        ("37-cap truncates — the 37TH SEAT", "\"PotionKinds\" => {"),
        # the App::new init line (the bank's 37 the init wears)
        (
            "items_potion_kinds: [crate::brewing_sim::POTION_KIND_NONE; 37],",
            "enchant_clues: [-1; 3],",
        ),
        # the offhand pickup caller (the seat's own Some ride)
        ("THE OFFHAND'S OWN KIND SEAT (the autopush-346",
         "                return;"),
        # the offhand icon render's tint ride (the world HUD lane)
        ("THE KIND'S OWN TINT RIDE (the autopush-346 round —",
         "if count > 1 {"),
        # the tooltip's offhand arm (the name ride's own seat)
        ("THE OFFHAND'S OWN SEAT (the autopush-346 round) the",
         "        };\n        Some(("),
        # the live lane's doc + the 37TH seat copy
        ("THE OFFHAND'S OWN SEAT (the autopush-346 round —\n    /// the 37TH seat)",
         "self.items_potion_kinds[36] = kinds[45];"),
        # the seat map's doc + the 45 arm
        ("THE OFFHAND 45\n/// THE BANK'S 36", "fn kind_bank_seat(menu: i32)"),
        ("fn kind_bank_seat(menu: i32) -> Option<usize> {", "/// THE BELIEVED KIND'S OWN RIDE"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        # the new seat test (the file's own tail)
        ("THE OFFHAND'S OWN KIND SEAT (the autopush-346 round — the wire's", None),
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

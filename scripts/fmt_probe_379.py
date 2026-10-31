#!/usr/bin/env python3
"""autopush-379 — THE FMT REGION WALK (the 372..378/dev-207/dev-208
record's own law the round wears): the whole-file rustfmt the ORIGINAL
text the formatted stdout the diff answers — the MY-REGIONS verdict the
marker ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the enchant's own field + the fresh-open's door +
                  the close's ride-home (the carried_kind hoist the
                  first-empty write keys); the throw's own retire; the
                  click lane's hoisted reads + the kind-aware call +
                  the re-keyed swap arm (the row_potion_kind read the
                  verdict block keys).
  part_11.rs    — the round's own test tail (the enchant kind bank).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the field + the init + the fresh-open's door (the carried word
    # the open retires)
    ("    /// THE CARRIED STACK'S OWN KIND WORD (the 379 round — the",
     "    /// The enchanting table's slot hit boxes the LAST render laid out:"),
    # the close's own ride-home (the carried_kind hoist the
    # first-empty write keys, the retire the zeroing speaks)
    ("        // THE CARRIED'S OWN HOME RIDE (the vanilla removed()'s\n        // placeItemBackInInventory walk — the close_brewing shape;",
     "        // THE PAIR'S OWN HOME RIDE (the removed()'s clearContainer walk —"),
    # the throw's own kind retire
    ("        if self.enchant_carried.1 <= 0 {\n            // the throw's own word (the emptied carried the NONE wears —",
     "    /// THE ENCHANTING TABLE'S OWN CLICK LANE (the offline sim's own walk"),
    # the click lane's hoisted reads + the kind-aware call + the
    # verdict block (the row_potion_kind read keys)
    ("        let old_car = self.enchant_carried;\n        // THE KIND'S OWN READS (the hoist the component law pre-keys —",
     "        self.enchant_mirror_sync_from_inventory();\n        // THE SLOTSCHANGED'S OWN WALK (the pickup/place the pair may"),
]

PART11 = [
    # the round's own test tail
    ("/// THE ENCHANTING SEATS' OWN KIND BANK (the 379 round —",
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

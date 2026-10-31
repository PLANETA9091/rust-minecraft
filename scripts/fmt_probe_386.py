#!/usr/bin/env python3
"""autopush-386 — THE FMT REGION WALK (the 372..autopush-385 record's own
law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the wire furnace paint's own kind gate (the 385 bank
                  the `!world_is_network` wall lifts, the component half
                  the BOTH lanes wears) + the wire furnace distribute's
                  own honest words (the carried word the field banks,
                  the mapped seat words the items bank lends, the tail's
                  own word retire).
  part_11.rs    — the round's own test tail (the wire furnace paint's
                  kind gate six arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the wire furnace paint's own component half (the gate the 385
    # bank lifts — the shared comment texts the 385-bank variant the
    # marker keys, the paint's own tail the cursor lands)
    ("answer); the WIRE lane the paint's own gate the 385 bank",
     "self.qc_slots.push(menu);\n        self.qc_recalc_remainder(); // pc 53-54 — the new seat the recalc speaks"),
    # the wire furnace distribute's own honest words + the tail retire
    ("the autopush-386 round the honest words ride): the 385",
     "if self.live_furnace_carried.1 <= 0 {\n            self.furnace_carried_potion_kind = crate::brewing_sim::POTION_KIND_NONE;\n        }\n    }"),
]

PART11 = [
    # the round's own test tail
    ("/// THE WIRE FURNACE PAINT'S OWN KIND GATE (the autopush-386 round — the",
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
        cursor = 0  # the sequential threading — the shared texts the order keys
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker, cursor)
            e = region_end(original, end_marker, s)
            cursor = e
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

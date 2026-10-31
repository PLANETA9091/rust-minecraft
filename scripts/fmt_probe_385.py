#!/usr/bin/env python3
"""autopush-385 — THE FMT REGION WALK (the 372..dev-211 record's own
law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the close's own word retire (the wire window's
                  death the banked word spends) + the wire furnace's
                  own kind walk (the reads' hoist, the predict's kind
                  call, the four arms' verdict, the mapped bank's
                  write, the carried's word ride).
  part_11.rs    — the round's own test tail (the wire furnace).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the close's own word retire (the cursor order keys — the close
    # precedes the click walk in the file, the region the walk's
    # cursor skips past)
    ("THE WINDOW'S OWN DEATH THE WORD RETIRES (the chest's own",
     "self.furnace_carried_potion_kind = crate::brewing_sim::POTION_KIND_NONE;\n        }"),
    # the wire furnace's own kind walk (the reads' hoist the match
    # keys — the qc paint's own menu variable the shared helper never
    # matches, the trio the None arm the same helper wears)
    ("items bank lends — the believed id's own gate the stale row\n        // never keys, the menu map the +6/-30 shifts answer)",
     "self.live_furnace_carried = new_carried;\n        let _ = sink.send(crate::world_view::ContainerCommand::Click("),
]

PART11 = [
    # the round's own test tail
    ("/// THE WIRE FURNACE'S OWN KIND BANK (the autopush-385 round — the",
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

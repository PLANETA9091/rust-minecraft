#!/usr/bin/env python3
"""autopush-412 — THE FMT REGION WALK (the 372..autopush-411 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  world.rs      — the insert walk's own index lend (the wrapper fn +
                  the (idx, arm) pair fn the landing law wears).
  screens.rs    — the front bank's own row + the landing's own ride
                  (the split arm the word banks, the merge arm the
                  grow law keeps, the refusal the cell never touches).
  part_11.rs    — the round's own test (the insert ride's arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

WORLD = [
    # the wrapper fn + the index-lend fn (the same merge law the
    # landing pair speaks)
    (
        "pub fn dispenser_insert_into(slots: &mut [(i32, i32)], id: i32) -> bool {",
        "/// THE MOVING PISTON'S OWN VARIANT BYTE",
    ),
]

SCREENS = [
    # the front bank's own row + the landing's own ride (the split
    # arm the word banks, the merge arm the grow law keeps)
    (
        "                    // THE FRONT BANK'S OWN ROW",
        "                } else if crate::world::is_orientable_furnace(front_kind) {",
    ),
]

PART11 = [
    # the round's own test (the insert ride's arms)
    (
        "/// THE DROPPER'S OWN INSERT RIDE",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/world.rs": WORLD,
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

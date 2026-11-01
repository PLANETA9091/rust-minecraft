#!/usr/bin/env python3
"""autopush-415 — THE FMT REGION WALK (the 372..autopush-414 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  world.rs      — the shulker box item's own kind predicate.
  screens.rs    — the spill's own mark + the beat's own place arm +
                  the eject gate's own ride.
  part_11.rs    — the round's own test (the spill's arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

WORLD = [
    # the shulker box item's own kind predicate (the item-keyed gate)
    (
        "/// THE SHULKER BOX ITEM'S OWN KIND",
        "/// THE HOPPER'S OWN KIND SEAT",
    ),
]

SCREENS = [
    # the spill's own mark (the entity gate's own flag the chain sets)
    (
        "            // THE SPILL'S OWN MARK",
        "            let (dispensed, voice) = if kind == crate::world::DROPPER {",
    ),
    # the beat's own place arm (the item-keyed law the walk wears)
    (
        "                // THE SHULKER BOX ITEM'S OWN SPILL (javap",
        "                } else if crate::world::is_chest(front_kind)",
    ),
    # the eject gate's own ride (the mark the spawn never wears)
    (
        "            // THE EJECT'S OWN ENTITY (the spawned drop the arrow the",
        "                let (px, py, pz) = crate::world::dispense_position",
    ),
]

PART11 = [
    # the round's own test (the spill's arms)
    (
        "/// THE DROPPER'S OWN SHULKER SPILL",
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

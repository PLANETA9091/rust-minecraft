#!/usr/bin/env python3
"""autopush-405 — THE FMT REGION WALK (the 372..autopush-404 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  world_view.rs — the slot-max hint's own predictors (the pickup
                  kind max + the right max the round wears).
  screens.rs    — the plain arms' own max-one re-key (the item
                  seat the menu-0 hint answers).
  part_11.rs    — the round's own test (the max-one's six arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

WORLDVIEW = [
    # the slot-max hint's own predictors (the round's own walk)
    (
        "/// THE SLOT-MAX HINT'S OWN PICKUP (the item seat's own ceiling the",
        "/// The vanilla `AbstractContainerMenu.doClick` PICKUP_ALL prediction",
    ),
]

SCREENS = [
    # the plain arms' own max-one re-key (the round's own lane)
    (
        "            } else if menu == 0 {",
        "        if menu < 2 {",
    ),
]

PART11 = [
    # the round's own test (the max-one's six arms)
    (
        "/// THE ENCHANT'S ITEM SEAT'S OWN MAX-ONE (the jar's EnchantmentMenu$2",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/world_view.rs": WORLDVIEW,
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

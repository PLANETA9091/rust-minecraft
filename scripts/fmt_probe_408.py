#!/usr/bin/env python3
"""autopush-408 — THE FMT REGION WALK (the 372..autopush-407 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  world_view.rs — the grid gather's own predictor (the two-pass take
                  the generic slice wears).
  screens.rs    — the double-click field pair + the release routing
                  arm + the release/offline walk pair + the close
                  reset + the press lane's own arm.
  part_11.rs    — the round's own test (the gather's arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

WORLDVIEW = [
    # the grid gather's own predictor (the round's own walk)
    (
        "/// THE GRID GATHER'S OWN WALK",
        "/// The vanilla `AbstractContainerMenu.doClick` PICKUP prediction for the",
    ),
]

SCREENS = [
    # the double-click field pair (the enchanting's own copies)
    (
        "THE ENCHANT SCREEN'S OWN DOUBLE-CLICK FLAG",
        "THE CONTAINER REMOVED RIDE'S OWN FLAG",
    ),
    (
        "THE ENCHANT SCREEN'S OWN LAST-CLICK SEAT",
        "THE QUICKCRAFT DRAG'S OWN ARM (the `AbstractContainerScreen.",
    ),
    # the release routing arm (the spread the enchant screen answers)
    (
        "THE ENCHANT GATHER'S OWN ARM (the 408 round",
        "THE QUICKCRAFT DRAG'S OWN RELEASE",
    ),
    # the release + offline walk pair
    (
        "THE ENCHANT GATHER'S OWN RELEASE WALK",
        "THE WIRE SPREAD'S OWN WALK",
    ),
    # the close's own reset
    (
        "THE DOUBLE-CLICK PAIR'S OWN RESET",
        "THE MIRROR ROWS' OWN STORE RIDE FIRST",
    ),
    # the press lane's own arm (the flag compute + the skip + the seat)
    (
        "THE DOUBLE-CLICK FLAG (the inventory's own shape",
        "        if self.screen == ScreenKind::Anvil {",
    ),
]

PART11 = [
    # the round's own test (the gather's arms)
    (
        "/// THE ENCHANT SEATS' OWN GATHER",
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

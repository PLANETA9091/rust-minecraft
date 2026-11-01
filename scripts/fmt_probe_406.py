#!/usr/bin/env python3
"""autopush-406 — THE FMT REGION WALK (the 372..autopush-405 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  world_view.rs — the max predictor's own count gate (the pickup
                  kind max the swap arms the round wears).
  screens.rs    — the plain arms' own gate re-key (the value branch
                  + the store ride the refusal keeps).
  part_11.rs    — the round's own test (the count gate's five arms).

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
    # the plain arms' own gate re-key (the value branch the walk keeps)
    (
        "THE COMPONENT IDENTITY LAW (the dev-196 round",
        "        if menu < 2 {",
    ),
    # the store ride's own refusal re-key (the gate the ride keeps)
    (
        "THE STORE'S OWN RIDE (the dev-196 round",
        "THE KIND'S OWN CURSOR RIDE",
    ),
]

PART11 = [
    # the round's own test (the count gate's five arms)
    (
        "/// THE SWAP ARM'S OWN COUNT GATE",
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

#!/usr/bin/env python3
"""autopush-487 — THE FMT REGION WALK (the 485/486 record's own law
the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  mth.rs      — the hue fold (the hsv_to_argb/hsv_to_rgb the durbar
                census banks).
  drops.rs    — the stack bar laws (the width/color/visible/opaque
                the strip's own reads bank).
  screens.rs  — the row + the ctor seat + the pickup ride + the HUD
                strip (the round's own four edits).
  part_11.rs  — the round's own tests (the two new damage-bar tests
                the tail wears, EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

MTH = [
    # the hue fold (the hsv pair the round banks)
    (
        "/// THE HUE'S OWN FOLD",
        "#[cfg(test)]",
    ),
]

DROPS = [
    # the stack bar laws (the four fns the strip reads)
    (
        "/// THE DAMAGE BAR'S OWN STACK LAWS",
        "/// The aiStep scan",
    ),
]

SCREENS = [
    # the row's own field (the parallel bank the ride feeds)
    (
        "/// THE DAMAGE BAR'S OWN ROW",
        "/// THE ANVIL'S OWN TRANSIENT TRIO",
    ),
    # the ctor's own seat (the default 0.0 row the pristine keeps)
    (
        "items_damage_frac: [0.0; 37],",
        "enchant_clues: [-1; 3],",
    ),
    # the pickup ride (the grow/reset arms the wear banks)
    (
        "// THE DAMAGE BAR'S OWN RIDE",
        "// THE PICKUP'S OWN POP",
    ),
    # the HUD strip (the itemBar lane the draw wears)
    (
        "// THE DAMAGE BAR'S OWN STRIP",
        "if count > 1 {",
    ),
]

PART11 = [
    # the round's own tests (the tail the EOF keeps)
    (
        "/// THE DAMAGE BAR'S OWN LAWS",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/mth.rs": MTH,
    "crates/mc-client/src/drops.rs": DROPS,
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
        sm = difflib.SequenceMatcher(
            None, original.splitlines(), formatted.splitlines())
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

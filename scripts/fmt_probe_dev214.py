#!/usr/bin/env python3
"""dev-214 — THE FMT REGION WALK (the 372..autopush-417 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the stand's own swap key fn (the prediction-free
               packet lane the checkHotbarKeyPressed gates wear — the
               dev-214 round's own edit).
  window.rs — the two inserts (the offhand 40 + the digits the swap
              chain the brewing arm joins).
  part_11.rs — the round's own test (the swap key's arms).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the stand's own swap key fn (the prediction-free packet lane
    # the checkHotbarKeyPressed gates wear)
    (
        "/// THE BREWING STAND'S OWN SWAP KEY (javap 26.2",
        "/// THE BREWING STAND'S OWN CLICK LANE (the offline sim's own walk",
    ),
]

PART11 = [
    # the round's own test (the wire title's arms)
    (
        "/// THE BREWING STAND'S OWN WIRE TITLE",
        None,
    ),
]

WINDOW = [
    # the offhand 40 insert (the swap chain the brewing arm joins)
    (
        "if !app.on_inventory_swap_key(40)",
        "app.on_swap_offhand_key();",
    ),
    # the digits insert (the same chain the digit arm joins)
    (
        "if !app.on_inventory_swap_key(i)",
        "&& !app.on_enchant_swap_key(i)",
    ),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/window.rs": WINDOW,
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

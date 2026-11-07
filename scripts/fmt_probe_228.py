#!/usr/bin/env python3
"""dev-228 — THE FMT REGION WALK (the 490 record's own law the round
wears): the whole-file rustfmt the ORIGINAL text the formatted stdout
the diff answers — the MY-REGIONS verdict the marker ranges keep (no
hunk may touch the round's own edits).

Regions:
  screens.rs  — the five glint seats (the chest mirror ride, the
                hopper lane, the table lane, the creative wrapped
                rows, the creative wrapped hotbar).
  part_11.rs  — the round's own helpers + the two new tests (the tail
                the EOF keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the chest mirror's own glint ride
    (
        "THE MIRROR'S OWN GLINT RIDE",
        "if *count > 1 {",
    ),
    # the hopper's own glint lane
    (
        "THE HOPPER'S OWN GLINT LANE",
        "if count > 1 {",
    ),
    # the table's own glint lane
    (
        "THE TABLE'S OWN GLINT LANE",
        "if count > 1 {",
    ),
    # the creative wrapped rows' own glint
    (
        "THE WRAPPED ROWS' OWN GLINT",
        "if count > 1 {",
    ),
    # the creative wrapped hotbar's own glint
    (
        "THE WRAPPED HOTBAR'S OWN GLINT",
        "if count > 1 {",
    ),
]

PART11 = [
    # the round's own helpers + tests (the tail the EOF keeps)
    (
        "fn the_menus_glint_count(app: &mut App) -> usize {",
        None,
    ),
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

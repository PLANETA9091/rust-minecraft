#!/usr/bin/env python3
"""autopush-481 — THE FMT REGION WALK (the 372..autopush-480 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs  — the knot bank fn pair (the block_pos row the 504
                census speaks), the load-once parse walk, the save
                row, the bind's save ride, the tick discard's ride,
                the attack kill's ride.
  part_11.rs  — the round's own tests (the bank roundtrip + the
                restart walk the tail wears, EOF the end the round
                keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the knot bank fn pair (the block_pos row the census speaks —
    # the impl App the next seat keeps)
    (
        "/// THE KNOT BANK'S OWN WALK",
        "impl App {",
    ),
    # the load-once parse walk (the fence knots' boot bank the round
    # mints — the save walk's doc the next seat keeps)
    (
        "                // THE KNOT BANK'S OWN PARSE",
        "    /// THE STORES' OWN SAVE WALK",
    ),
    # the save row (the fence knots the file's next list wears — the
    # bed spawn's row the next seat keeps)
    (
        "            // THE KNOT BANK'S OWN LIST",
        "            // THE BED SPAWN'S OWN ROW",
    ),
    # the bind's save ride (the spawn the file banks — the discard
    # arm the next seat keeps)
    (
        "        if flag {\n            // THE PLACEMENT'S OWN VOICE",
        "        // THE FRESH-KNOT'S OWN DISCARD",
    ),
    # the tick discard's ride (the poll's removal the file banks —
    # the punch doc the next seat keeps)
    (
        "        // THE KNOT'S OWN SAVE RIDE (the autopush-481 round — the\n        // discard the containers file banks",
        "    /// THE KNOT'S OWN PUNCH BREAK",
    ),
    # the attack kill's ride (the kill the file banks — the
    # frame-analog doc the sibling seat keeps)
    (
        "        // THE KNOT'S OWN VOICE (javap `dropItem` the playSound word —",
        "    /// THE FRAME'S OWN ANALOG BROADCAST",
    ),
]

PART11 = [
    # the round's own tests (the bank roundtrip + the restart walk
    # the tail wears — the EOF the end the round keeps)
    (
        "/// THE KNOT BANK'S OWN ROUNDTRIP",
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

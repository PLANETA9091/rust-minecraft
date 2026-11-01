#!/usr/bin/env python3
"""autopush-393 — THE FMT REGION WALK (the 372..dev-213 record's own
law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the quick-move's own kind ride (the offline lane's
                  pre-reads + ride + the sink's own clone + the live
                  lane's pre-reads + ride).
  part_11.rs    — the round's own test tail (the quick-move kind
                  ride's six arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the offline lane's own pre-reads + kind ride
    (
        "// THE KIND'S OWN PRE-READS (the 384 law the furnace's",
        "self.hotbar_slots = self.inventory.hotbar();\n                return;",
    ),
    # the sink's own clone
    (
        "// THE SINK'S OWN CLONE (the owned sender the ride's own",
        "let Some((_, _, slot)) = self.furnace_hover_seat(mx, my) else {",
    ),
    # the live lane's own pre-reads + kind ride
    (
        "// THE KIND'S OWN PRE-READS (the 384 law the live lane's",
        "let _ = sink.send(crate::world_view::ContainerCommand::Click(",
    ),
]

PART11 = [
    # the round's own test tail
    ("/// THE FURNACE QUICK-MOVE'S OWN KIND RIDE (the 393 round —", None),
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

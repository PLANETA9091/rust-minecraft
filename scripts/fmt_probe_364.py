#!/usr/bin/env python3
"""autopush-364 — THE FMT REGION WALK (the 323/204/358/360/361/362/363
record's own law the round wears): the whole-file rustfmt the ORIGINAL
text the formatted stdout the diff answers — the MY-REGIONS verdict the
marker ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs  — the row-kind bank's own disk lane (the four bank/parse
                helpers), the save root's two rows, the load parse's
                two extends.
  part_11.rs  — the round's own test tail (the disk ride walk).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the row-kind bank's own disk helpers
    ("/// THE CHEST ROWS' OWN KIND BANK'S DISK LANE (the autopush-364 round —",
     "/// THE FRAME BANK'S OWN WALK (the autopush-136 round"),
    # the save root's own two rows
    ("            // THE CHEST ROWS' OWN KIND BANK'S ROW (the autopush-364",
     "        let mut bytes = Vec::new();"),
    # the load parse's own two extends
    ("                // THE CHEST ROWS' OWN KIND PARSE (the autopush-364",
     "                // THE BED SPAWN'S OWN PARSE (the autopush-312 round"),
]

PART11 = [
    # the round's own test tail
    ("/// THE CHEST ROWS' OWN KIND BANK'S DISK RIDE (the autopush-364 round —",
     None),
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
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker)
            e = region_end(original, end_marker, s)
            s_line = original[:s].count("\n")
            e_line = original[:e].count("\n")
            bad = [h for h in hunks if h[0] < e_line and h[1] > s_line]
            label = start_marker.strip()[:56]
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

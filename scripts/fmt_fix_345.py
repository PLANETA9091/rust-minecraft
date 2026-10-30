#!/usr/bin/env python3
"""autopush-345 — THE FMT FIX WALK (the probe's dirty regions the
splice re-lays): the whole-file rustfmt the diff opcodes answer — the
MY-REGIONS hunks ONLY the original text the formatted lines replace
(the pre-existing static noise the HEAD law keeps, the non-mine hunks
the splice never touches).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

REGIONS = {
    "crates/mc-client/src/screens.rs": [],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        # the ride tests (the rustfmt re-lay the swap comment's
        # trailing-column law wears)
        ("THE BELIEVED KIND'S OWN RIDE (the autopush-345 round", None),
    ],
}


def find(text: str, marker: str, start: int = 0) -> int:
    i = text.find(marker, start)
    if i < 0:
        raise SystemExit(f"marker not found: {marker[:60]!r}")
    return i


for path, regions in REGIONS.items():
    with open(path) as f:
        original = f.read()
    proc = subprocess.run(
        [RUSTFMT, "--edition", "2021"], input=original, capture_output=True, text=True
    )
    if proc.returncode != 0:
        print(f"{path}: rustfmt FAILED rc={proc.returncode}")
        sys.exit(1)
    formatted = proc.stdout
    sm = difflib.SequenceMatcher(None, original.splitlines(), formatted.splitlines())
    mine = []
    for (s_marker, e_marker) in regions:
        s = find(original, s_marker)
        e = (
            len(original)
            if e_marker is None
            else find(original, e_marker, s + len(s_marker))
        )
        sl, el = original[:s].count("\n") + 1, original[:e].count("\n") + 1
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == "equal":
                continue
            if i1 + 1 < el and i2 > sl:  # 1-based line overlap
                mine.append((i1, i2, j1, j2))
    if not mine:
        print(f"{path}: nothing to splice")
        continue
    # apply from the bottom so line numbers stay valid
    lines_original = original.splitlines(keepends=True)
    lines_formatted = formatted.splitlines(keepends=True)
    for i1, i2, j1, j2 in sorted(mine, reverse=True):
        lines_original[i1:i2] = lines_formatted[j1:j2]
    with open(path, "w") as f:
        f.write("".join(lines_original))
    print(f"{path}: spliced {len(mine)} dirty region hunks")

print("done")

#!/usr/bin/env python3
"""autopush-324 — THE FMT SPLICE WALK (the 323 record's own law the
round wears): the rustfmt text the MINE hunks the splice serves — the
pre-existing static the HEAD law keeps (the non-mine hunks the file
never wears). The round's own ranges the marker search resolves at
runtime: the two REAL tooltip tests the appended block keeps.

Usage: python3 scripts/fmt_splice_324.py
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

MARKERS = {
    "crates/mc-client/src/screens/tests/part_11.rs": [
        # the two REAL tooltip tests (the appended block the round banks)
        (
            "/// THE BEACON'S OWN HOVER-TOOLTIP SEAT (the 324 round",
            "the restored coin the rows serve",
        ),
    ],
}


def resolve_ranges(path, markers):
    lines = open(path, encoding="utf-8").read().split("\n")
    out = []
    for start_marker, end_marker in markers:
        lo = next(i for i, l in enumerate(lines) if start_marker in l)
        hi = next(i for i, l in enumerate(lines[lo:], lo) if end_marker in l)
        out.append((lo, hi + 1))
    return out


def main():
    fail = 0
    for path, markers in MARKERS.items():
        ranges = resolve_ranges(path, markers)
        print(f"RANGE {path}: {ranges}")
        orig = open(path, encoding="utf-8").read().split("\n")
        r = subprocess.run(
            [RUSTFMT, "--edition", "2021", "--emit", "stdout", path],
            capture_output=True,
            text=True,
        )
        if r.returncode != 0:
            print(f"[FAIL] rustfmt: {r.stderr[:300]}")
            return 1
        fmt = r.stdout.split("\n")
        sm = difflib.SequenceMatcher(None, orig, fmt, autojunk=False)
        out = list(orig)
        rides = 0
        for tag, i1, i2, j1, j2 in reversed(sm.get_opcodes()):
            if tag == "equal":
                continue
            if not any(lo <= i1 and i2 <= hi for lo, hi in ranges):
                continue
            out[i1:i2] = fmt[j1:j2]
            rides += 1
        if rides:
            open(path, "w", encoding="utf-8").write("\n".join(out))
        print(f"SPLICE {path}: {rides} hunk(s) rode")
    return fail


sys.exit(main())

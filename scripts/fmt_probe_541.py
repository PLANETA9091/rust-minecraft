#!/usr/bin/env python3
"""autopush-541 — THE FMT REGION WALK (the 535/536/537/538/539/540
record's own law the round wears): the whole-file rustfmt the ORIGINAL
text the formatted stdout the diff answers — the MY-REGIONS verdict the
marker ranges keep (no hunk may touch the round's own edits).

Regions: the round's own census completion record the walk wears. One
shape: the FN-WALK region (the census doc's first line the start marker
keeps, the EOF the tail test's fallback wears — the indented-fn pattern
the body the interior helper fns never match, the census tests the
straight-line harness the region the whole pair wears).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

TESTS_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
TESTS_REGIONS = [
    # THE ROUND'S OWN CENSUS RECORD (the player + container lanes the
    # tail pair wears — the EOF the fallback the whole pair bounds)
    (
        "/// THE TAKE-KEY CENSUS'S OWN COMPLETION RECORD (the autopush-541",
        None,
    ),
]

FN_LINE = re.compile(r"^    (pub )?fn ")


def region_bounds(lines, start, end, fn_line):
    si = lines.index(start)
    if end is None:
        ei = None
        for j in range(si + 1, len(lines)):
            if fn_line.match(lines[j]):
                ei = j
                break
        if ei is None:
            ei = len(lines)  # the EOF fallback the tail test wears
    else:
        ei = lines.index(end, si + 1)
    return (si, ei)


def walk(path, regions, fn_line=FN_LINE) -> bool:
    ok = True
    original = open(path, encoding="utf-8").read()
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path], capture_output=True, text=True
    )
    if proc.returncode != 0:
        raise SystemExit(f"RUSTFMT FAILED\n{proc.stderr[:400]}")
    mine_lines = original.split("\n")
    theirs_lines = proc.stdout.split("\n")
    for (start, end) in regions:
        ms, me = region_bounds(mine_lines, start, end, fn_line)
        ts, te = region_bounds(theirs_lines, start, end, fn_line)
        mine = "\n".join(mine_lines[ms:me])
        theirs = "\n".join(theirs_lines[ts:te])
        label = mine_lines[ms].strip()[:58]
        if mine == theirs:
            print(f"CLEAN  {label}")
        else:
            ok = False
            print(f"DIRTY  {label}")
            for line in difflib.unified_diff(
                mine.splitlines(), theirs.splitlines(), "mine", "rustfmt",
                lineterm="", n=1,
            ):
                print(line)
    return ok


def main() -> int:
    ok = walk(TESTS_PATH, TESTS_REGIONS)
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

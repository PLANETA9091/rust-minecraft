#!/usr/bin/env python3
"""autopush-542 — THE FMT REGION WALK (the 535/536/537/538/539/540/541
record's own law the round wears): the whole-file rustfmt the ORIGINAL
text the formatted stdout the diff answers — the MY-REGIONS verdict the
marker ranges keep (no hunk may touch the round's own edits).

Regions: the round's own world-merge component leg the lanes wear.
Two shapes: the FN-WALK region (the doc's first line the start marker
keeps, the NEXT doc's own line the end answers) and the EOF fallback
(the tail test the whole census wears — the indented-fn pattern the
interior the body never matches).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

DROPS_PATH = "crates/mc-client/src/drops.rs"
DROPS_REGIONS = [
    # THE WORLD MERGE'S OWN COMPONENT LEG (the mergeable doc the
    # walk's own region wears — the neighbour merge's own doc the end
    # answers)
    (
        "/// `isMergable`'s own law (javap): the stack fits one more of its kind",
        "/// The neighbour merge (javap mergeWithNeighbours): the closest",
    ),
]

TESTS_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
TESTS_REGIONS = [
    # THE ROUND'S OWN CENSUS (the tail test the EOF fallback wears)
    (
        "/// THE WORLD MERGE'S OWN COMPONENT LEG (the autopush-542 round — the",
        None,
    ),
]

FN_LINE = re.compile(r"^(pub(\(crate\))? )?fn ")
TEST_FN_LINE = re.compile(r"^    (pub )?fn ")


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
    ok = walk(DROPS_PATH, DROPS_REGIONS)
    ok = walk(TESTS_PATH, TESTS_REGIONS, TEST_FN_LINE) and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

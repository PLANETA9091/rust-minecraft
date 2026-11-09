#!/usr/bin/env python3
"""autopush-544 — THE FMT REGION WALK (the 535..543 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges keep
(no hunk may touch the round's own edits).

Regions: the round's own world kind tint the lanes wear. Two shapes:
the TIGHT BLOCK (the unique statement lines the round's own edits
bound) and the EOF-fallback tail test (the start marker the round's
own census wears, no indented fn after — the whole tail the region
keeps).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS_PATH = "crates/mc-client/src/screens.rs"
SCREENS_REGIONS = [
    # THE DROP'S OWN KIND WORD (the drops lane the GROUND walk's tint)
    (
        "                // THE DROP'S OWN KIND WORD (the autopush-544 ride — the",
        "                sampled.push(crate::raster::SampledQuad {",
    ),
    # THE FRAME'S OWN KIND WORD (the contained item the FIXED tint)
    (
        "                            // THE FRAME'S OWN KIND WORD (the autopush-544",
        "                            sampled.push(crate::raster::SampledQuad {",
    ),
]

TESTS_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
TESTS_REGIONS = [
    # THE ROUND'S OWN TINT CENSUS (the tail test the EOF fallback wears)
    (
        "/// THE WORLD'S OWN KIND TINT (the autopush-544 round — the NEXT's own",
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
    ok = walk(SCREENS_PATH, SCREENS_REGIONS)
    ok = walk(TESTS_PATH, TESTS_REGIONS) and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

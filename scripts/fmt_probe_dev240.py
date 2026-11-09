#!/usr/bin/env python3
"""dev-240 — THE FMT REGION WALK (the 535..547 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges keep
(no hunk may touch the round's own edits).

Regions: the round's own spawn finalize the lanes wear. Shapes: the
TIGHT BLOCK (the unique statement lines the round's own edits bound),
the EOF-fallback tail test (the start marker the round's own census
wears, no indented fn after — the whole tail the region keeps). The
mob.rs arms the line-width audit answers (the short match arms rustfmt
keeps verbatim, the 100-width law the audit pins).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

MOB_PATH = "crates/mc-client/src/mob.rs"
MOB_REGIONS = [
    # THE FINALIZE'S OWN PURE BLOCK (the by-id law the JEB doc bounds)
    (
        "/// java `Math.floorMod` semantics the negative half speaks",
        "/// THE JEB_ RAINBOW'S OWN WORD",
    ),
    # THE TABLE'S OWN HORSE ROW (the weighted rows the creature count
    # doc bounds)
    (
        "/// THE BIOME'S OWN WEIGHTED ROWS (the plains",
        "/// THE CREATURE LANE'S OWN COUNT",
    ),
    # THE PACK DRAW + THE HERD SLOT (the biome pick the 3-tries loop
    # bounds)
    (
        "    // THE BIOME'S OWN PICK (getRandomSpawnMobAt's weighted fold)",
        "    // THE 3 TRIES (loc12's own loop",
    ),
    # THE MEMBER'S OWN FINALIZE ARM (the sheep color arm the group
    # data arm bounds)
    (
        "    // THE HORSE'S OWN FINALIZE (Horse.finalizeSpawn the walk",
        "            // THE GROUP DATA'S OWN ARM",
    ),
    # THE ROUND'S OWN CENSUS (the tail test the EOF fallback wears)
    (
        "/// THE HORSE'S OWN SPAWN FINALIZE THE CREATURE WALK WEARS",
        None,
    ),
    # THE MIRROR'S OWN CHAIN (the stream test the death-loot doc
    # bounds)
    (
        "    /// THE BABY ROLL'S OWN STREAM (the mirror-replay the same seed",
        "    /// THE DEATH LOOT's own distribution",
    ),
    # THE WEIGHTS' OWN BANDS (the chicken doc the drops doc bounds)
    (
        "    /// THE PLAINS WEIGHTS THE CHICKEN BAND CARRIES",
        "    /// THE CHICKEN DROPS ITS OWN ROWS",
    ),
]

WIDTH_LAW = 100
FN_LINE = re.compile(r"^    fn |^    pub fn |^fn |^pub fn ")


def region_bounds(lines, start, end, fn_line) -> tuple:
    si = None
    for i, line in enumerate(lines):
        if start in line:
            si = i
            break
    if si is None:
        raise SystemExit(f"START MARKER NOT FOUND: {start[:60]}")
    if end is None:
        # the EOF fallback the tail test wears — the next indented fn
        # after the start the region closes, the whole tail the
        # no-fn-after keeps
        for j in range(si + 1, len(lines)):
            if fn_line.match(lines[j]):
                return (si, j)
        return (si, len(lines))
    ei = None
    for j in range(si + 1, len(lines)):
        if end in lines[j]:
            ei = j
            break
    if ei is None:
        ei = len(lines)  # the EOF fallback the tail test wears
    return (si, ei)


def width_audit(path, needle) -> bool:
    ok = True
    for i, line in enumerate(open(path, encoding="utf-8"), 1):
        if needle in line and len(line.rstrip("\n")) > WIDTH_LAW:
            ok = False
            print(f"WIDTH  {path}:{i} ({len(line.rstrip())} cols) {line.strip()[:60]}")
    return ok


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
    ok = walk(MOB_PATH, MOB_REGIONS)
    # THE MOB ARMS' OWN WIDTH AUDIT (the short match arms rustfmt
    # keeps verbatim — the Horse + the finalize words the 100-width
    # law pins)
    ok = width_audit(MOB_PATH, "Horse") and ok
    ok = width_audit(MOB_PATH, "horse_markings") and ok
    ok = width_audit(MOB_PATH, "horse_finalize") and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

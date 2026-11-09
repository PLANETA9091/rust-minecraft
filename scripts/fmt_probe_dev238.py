#!/usr/bin/env python3
"""dev-238 — THE FMT REGION WALK (the 535..545 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges keep
(no hunk may touch the round's own edits).

Regions: the round's own horse mesh the lanes wear. Shapes: the TIGHT
BLOCK (the unique statement lines the round's own edits bound), the
start-only tail (the fn line the region closes) and the EOF-fallback
tail test (the start marker the round's own census wears, no indented
fn after — the whole tail the region keeps). The mob.rs arms the
line-width audit answers (the short match arms rustfmt keeps verbatim,
the 100-width law the audit pins).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

AVATAR_PATH = "crates/mc-client/src/avatar.rs"
AVATAR_REGIONS = [
    # THE HORSE'S OWN BLOCK (the consts + the model + the parts the
    # round's own section wears — the Chicken header the tail bounds)
    (
        "// THE HORSE (the equine mesh the autopush-545 unseen lane wore — the",
        "// THE CHICKEN (the fifth drawn body — the avian's own chain)",
    ),
]

SCREENS_PATH = "crates/mc-client/src/screens.rs"
SCREENS_REGIONS = [
    # THE HORSE RENDER'S OWN ARM (the unseen lane the mesh walk
    # retires — the Sheep arm the tail bounds)
    (
        "                crate::mob::MobKind::Horse => {",
        "                crate::mob::MobKind::Sheep => {",
    ),
    # THE LAZY LOAD'S OWN LANE (the coat set the jar root reads)
    (
        "        // THE HORSE'S OWN LAZY LOAD (the seven-coat set the same cache",
        "        // THE VARIANT SKINS' OWN LAZY WALK (the cold/warm pairs the",
    ),
    # THE FIELD'S OWN SEAT (the bake the llama field's tail wears)
    (
        "    pub horse_model: Option<crate::avatar::HorseModel>,",
        "    /// THE FOURTH MOB's own WOOL (the SheepFurModel inflate-mesh over",
    ),
]

TESTS_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
TESTS_REGIONS = [
    # THE ROUND'S OWN MESH CENSUS (the tail test the EOF fallback wears)
    (
        "/// THE HORSE'S OWN MESH (javap 26.2 `AbstractEquineModel.createBodyMesh`",
        None,
    ),
]

MOB_PATH = "crates/mc-client/src/mob.rs"
FN_LINE = re.compile(r"^    (pub )?fn ")
WIDTH_LAW = 100


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
        label = mine_lines[ms].strip().split("\n")[0][:58]
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
    ok = walk(AVATAR_PATH, AVATAR_REGIONS)
    ok = walk(SCREENS_PATH, SCREENS_REGIONS) and ok
    ok = walk(TESTS_PATH, TESTS_REGIONS) and ok
    # THE MOB ARMS' OWN WIDTH AUDIT (the short match arms rustfmt
    # keeps verbatim — the Horse words the 100-width law pins)
    ok = width_audit(MOB_PATH, "Horse") and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

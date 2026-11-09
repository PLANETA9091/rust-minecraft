#!/usr/bin/env python3
"""dev-241 — THE FMT REGION WALK (the 535..548 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges keep
(no hunk may touch the round's own edits).

Regions: the round's own wire variant int the lanes wear. Shapes: the
TIGHT BLOCK (the unique statement lines the round's own edits bound),
the EOF-fallback tail test (the body fn the round's own walk wears, no
fn after — the whole tail the region keeps). The touched files the
width audit answers (the 100-width law the audit pins) and the count
law (the identical insert lines the helper literals repeat, the
per-file count the walk pins).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

WIDTH_LAW = 100
FN_LINE = re.compile(r"^    fn |^    pub fn |^fn |^pub fn ")

MOB_PATH = "crates/mc-client/src/mob.rs"
MOB_REGIONS = [
    # THE VARIANT-INT'S OWN WIRE INDEX (the class-tree doc the pig's
    # word bounds)
    (
        "/// THE HORSE'S OWN VARIANT-INT WIRE INDEX",
        "/// THE PIG'S OWN VARIANT WORD",
    ),
]

WV_PATH = "crates/mc-client/src/world_view.rs"
WV_REGIONS = [
    # THE WIREMOB'S OWN FIELDS (the horse pair the equipment doc
    # bounds)
    (
        "/// THE HORSE VARIANT'S OWN WORD (the DATA_ID_TYPE_VARIANT int the",
        "/// THE EQUIPMENT WORD (the Set Equipment store's MAINHAND slot",
    ),
    # THE REFRESH'S OWN READS (the horse pair the equipment read
    # bounds)
    (
        "                    // THE HORSE VARIANT'S OWN WORD (the packed int the",
        "                    // THE EQUIPMENT WORD (the store's MAINHAND slot 0",
    ),
    # THE PROJECTION'S OWN TEST (the doc the spider's walk bounds)
    (
        "/// THE HORSE'S OWN VARIANT INT (the DATA_ID_TYPE_VARIANT packed",
        "/// THE SPIDER'S OWN CLIMB WORD",
    ),
]

SCREENS_PATH = "crates/mc-client/src/screens.rs"
SCREENS_REGIONS = [
    # THE FRESH SPAWN'S OWN ARM (the horse merge the equipment ride
    # bounds)
    (
        "                // THE HORSE'S OWN VARIANT INT rides the spawn",
        "                // THE EQUIPMENT WORD rides the spawn",
    ),
    # THE POSE WALK'S OWN ARM (the horse merge the name ride bounds)
    (
        "            // THE HORSE'S OWN VARIANT INT rides with the pose",
        "            // THE NAME WORD rides with the pose",
    ),
]

HEADLESS_PATH = "crates/mc-client/src/headless.rs"
HEADLESS_REGIONS = [
    # THE FOUR LITERALS' OWN PAIR (the identical insert lines the
    # first site bounds — the count law the whole-file grep pins)
    (
        "                            horse_variant: 0,",
        "                            held_weapon: None,",
    ),
]

TESTS_MOD_PATH = "crates/mc-client/src/screens/tests/mod.rs"
TESTS_MOD_REGIONS = [
    # THE HELPERS' OWN PAIR (the identical insert lines the first
    # helper bounds — the count law the whole-file grep pins)
    (
        "        horse_variant: 0,",
        "        held_weapon: None,",
    ),
]

PART11_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
PART11_REGIONS = [
    # THE WIRE TEST'S OWN DOC (the doc the round's fn bounds)
    (
        "/// THE HORSE'S OWN WIRE VARIANT (javap 26.2 `Horse` the",
        None,
    ),
    # THE WIRE TEST'S OWN BODY (the fn the EOF fallback keeps — the
    # walk the tail test wears)
    (
        "fn the_horse_the_wire_variant_the_state_wears",
        None,
    ),
]


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


def count_audit(path, needle, want) -> bool:
    got = sum(1 for line in open(path, encoding="utf-8") if needle in line)
    if got != want:
        print(f"COUNT  {path}: '{needle}' {got} sites, the law wants {want}")
        return False
    print(f"COUNT  {path}: '{needle}' {got} sites the law wears")
    return True


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
    ok = walk(WV_PATH, WV_REGIONS) and ok
    ok = walk(SCREENS_PATH, SCREENS_REGIONS) and ok
    ok = walk(HEADLESS_PATH, HEADLESS_REGIONS) and ok
    ok = walk(TESTS_MOD_PATH, TESTS_MOD_REGIONS) and ok
    ok = walk(PART11_PATH, PART11_REGIONS) and ok
    # THE COUNT LAW (the identical insert lines the literal sites
    # repeat)
    ok = count_audit(HEADLESS_PATH, "horse_variant: 0,", 4) and ok
    ok = count_audit(TESTS_MOD_PATH, "horse_variant: 0,", 2) and ok
    ok = count_audit(MOB_PATH, "pub const HORSE_DATA_TYPE_VARIANT_ID", 1) and ok
    ok = count_audit(WV_PATH, "HORSE_DATA_TYPE_VARIANT_ID", 6) and ok
    # THE WIDTH LAW (the round's own words the 100 cols pin)
    for path in (
        MOB_PATH,
        WV_PATH,
        SCREENS_PATH,
        HEADLESS_PATH,
        TESTS_MOD_PATH,
        PART11_PATH,
    ):
        ok = width_audit(path, "horse_variant") and ok
        ok = width_audit(path, "horse_markings") and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

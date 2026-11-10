#!/usr/bin/env python3
"""dev-246 — THE FMT REGION WALK (the 535..553 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges keep
(no hunk may touch the round's own edits).

Regions: the round's own mount quickcraft drag the lanes wear. Shapes:
the TIGHT BLOCK (the unique statement lines the round's own edits
bound), the EOF-fallback tail test (the quickcraft test fn the round's
own walk wears, no fn after — the whole tail the region keeps). The
touched files the width audit answers (the 100-width law the audit
pins) and the count law (the identical insert lines the helper literals
repeat, the per-file count the walk pins).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

WIDTH_LAW = 100
FN_LINE = re.compile(r"^    fn |^    pub fn |^fn |^pub fn ")

SCREENS_PATH = "crates/mc-client/src/screens.rs"
SCREENS_REGIONS = [
    # THE LANE CURSOR'S OWN ARM (the horse carried the qc carried
    # chain the Crafting arm bounds)
    (
        "            // THE MOUNT'S OWN LANE CURSOR (the 553 NEXT's own drag the",
        "        if self.screen == ScreenKind::Crafting {",
    ),
    # THE SEAT STACK'S OWN NUMBERING (the horse pair the furnace arm
    # bounds)
    (
        "            // THE MOUNT'S OWN NUMBERING (the 553 NEXT's own drag the",
        "        if self.screen == ScreenKind::Furnace {",
    ),
    # THE COLLECT'S OWN WALK (the paint the chest wire doc bounds)
    (
        "    /// THE MOUNT DRAG'S OWN COLLECT (mouseDragged pcs 23-58",
        "    /// THE CHEST DRAG'S OWN WIRE DISTRIBUTION",
    ),
    # THE HOVER'S OWN DISPATCH (the paint call the keyboard doc bounds)
    (
        "                // THE MOUNT'S OWN HOVER (the 553 NEXT's own first lane",
        "    /// The keyboard lane's shift state",
    ),
    # THE RELEASE'S OWN LANES (the wire/offline split the painted tail
    # bounds)
    (
        "                // THE MOUNT'S OWN LANES (the 553 NEXT's own first lane",
        "            self.qc_armed = false; // pc 452-454",
    ),
    # THE OFFLINE FOLD'S OWN WALK (the distribution the wire doc
    # bounds)
    (
        "    /// THE MOUNT'S OWN OFFLINE DISTRIBUTION (the doClick QUICK_CRAFT",
        "    /// THE MOUNT'S WIRE DISTRIBUTION",
    ),
    # THE WIRE FOLD'S OWN CALLER (the write-back the furnace doc
    # bounds)
    (
        "    /// THE MOUNT'S WIRE DISTRIBUTION (the offline walk's twin over the",
        "    /// THE FURNACE'S WIRE DISTRIBUTION",
    ),
    # THE FREE FN'S OWN PACKETS (the mount wire walk the brewing doc
    # bounds)
    (
        "/// THE MOUNT DRAG'S OWN WIRE DISTRIBUTION (the same doClick QUICK_CRAFT",
        "/// THE BREWING DRAG'S OWN WIRE DISTRIBUTION",
    ),
]

PART11_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
PART11_REGIONS = [
    # THE QUICKCRAFT TEST'S OWN DOC + BODY (the doc the EOF fallback
    # keeps)
    (
        "/// THE HORSE'S OWN QUICKCRAFT DRAG (the 554 round",
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
    ok = walk(SCREENS_PATH, SCREENS_REGIONS)
    ok = walk(PART11_PATH, PART11_REGIONS) and ok
    # THE COUNT LAW (the identical insert lines the literal sites
    # repeat)
    ok = count_audit(SCREENS_PATH, "fn qc_paint_horse", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn qc_distribute_horse_offline", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn qc_distribute_horse_wire", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn wire_horse_quickcraft_distribute", 1) and ok
    ok = count_audit(PART11_PATH, "fn the_horse_the_quickcraft_the_drag_wears", 1) and ok
    # THE WIDTH LAW (the round's own words the 100 cols pin)
    for path in (SCREENS_PATH, PART11_PATH):
        ok = width_audit(path, "qc_paint_horse") and ok
        ok = width_audit(path, "qc_distribute_horse") and ok
        ok = width_audit(path, "wire_horse_quickcraft_distribute") and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

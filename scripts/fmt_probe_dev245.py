#!/usr/bin/env python3
"""dev-245 — THE FMT REGION WALK (the 535..552 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges keep
(no hunk may touch the round's own edits).

Regions: the round's own mount quick-move the lanes wear. Shapes:
the TIGHT BLOCK (the unique statement lines the round's own edits
bound), the EOF-fallback tail test (the Q test fn the round's own walk
wears, no fn after — the whole tail the region keeps). The touched
files the width audit answers (the 100-width law the audit pins) and
the count law (the identical insert lines the helper literals repeat,
the per-file count the walk pins).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

WIDTH_LAW = 100
FN_LINE = re.compile(r"^    fn |^    pub fn |^fn |^pub fn ")

WORLD_VIEW_PATH = "crates/mc-client/src/world_view.rs"
WORLD_VIEW_REGIONS = [
    # THE MOUNT TRIO'S OWN BANK (the armor family const + the mayPlace
    # law + the quick-move predictor the bundle-rows doc bounds)
    (
        "/// THE HORSE ARMOR'S OWN FAMILY (the BODY seat's own mayPlace row",
        "/// The component patch's OWN bundle rows",
    ),
]

SCREENS_PATH = "crates/mc-client/src/screens.rs"
SCREENS_REGIONS = [
    # THE CONST TAIL'S OWN LAW (the armor family the round MOVED to
    # the wire domain — the WHITE const the sprint window doc bounds)
    (
        "pub const WHITE: [f32; 3] = [1.0, 1.0, 1.0];",
        "/// THE SPRINT WINDOW'S OWN SPAN",
    ),
    # THE OFFLINE SHIFT'S OWN LANE (the quick-move walk the pickup
    # pair bounds)
    (
        "            // THE QUICK-MOVE LANE (the shift-click — the vanilla",
        "            let (believed, seat) = self.horse_menu_stack(slot);",
    ),
    # THE WIRE SHIFT'S OWN LANE (the comment the pickup pair bounds —
    # the if line the family shape the other branches share the
    # rustfmt law never wears)
    (
        "            // THE QUICK-MOVE LANE (the wire half",
        "        let (believed, seat) = self.horse_menu_stack(slot);",
    ),
    # THE FLAT + APPLY'S OWN PAIR (the projection the mayPlace doc
    # bounds)
    (
        "    /// THE HORSE MENU'S OWN FLAT PROJECTION",
        "    /// THE HORSE CARRIED'S OWN THROW",
    ),
]

PART11_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
PART11_REGIONS = [
    # THE QUICK-MOVE TEST'S OWN DOC + BODY (the doc the EOF fallback
    # keeps)
    (
        "/// THE MOUNT WINDOW'S OWN QUICK-MOVE (the 552 NEXT's own first lane",
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
    ok = walk(WORLD_VIEW_PATH, WORLD_VIEW_REGIONS)
    ok = walk(SCREENS_PATH, SCREENS_REGIONS) and ok
    ok = walk(PART11_PATH, PART11_REGIONS) and ok
    # THE COUNT LAW (the identical insert lines the literal sites
    # repeat)
    ok = count_audit(WORLD_VIEW_PATH, "pub fn predict_horse_quick_move", 1) and ok
    ok = count_audit(WORLD_VIEW_PATH, "pub fn horse_seat_may_place", 1) and ok
    ok = count_audit(WORLD_VIEW_PATH, "pub const HORSE_ARMOR_NAMES", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn horse_menu_flat", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn horse_quick_move_apply", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn horse_seat_may_place", 1) and ok
    ok = count_audit(SCREENS_PATH, "const HORSE_ARMOR_NAMES", 0) and ok
    ok = count_audit(PART11_PATH, "fn the_horse_the_quick_move_the_lanes_wear", 1) and ok
    # THE WIDTH LAW (the round's own words the 100 cols pin)
    for path in (WORLD_VIEW_PATH, SCREENS_PATH, PART11_PATH):
        ok = width_audit(path, "predict_horse_quick_move") and ok
        ok = width_audit(path, "horse_menu_flat") and ok
        ok = width_audit(path, "horse_quick_move_apply") and ok
        ok = width_audit(path, "horse_seat_may_place") and ok
        ok = width_audit(path, "HORSE_ARMOR_NAMES") and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""dev-244 — THE FMT REGION WALK (the 535..551 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges keep
(no hunk may touch the round's own edits).

Regions: the round's own mount click pair the lanes wear. Shapes:
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

SCREENS_PATH = "crates/mc-client/src/screens.rs"
SCREENS_REGIONS = [
    # THE ARMOR FAMILY'S OWN CONST (the doc the sprint window doc
    # bounds)
    (
        "/// THE HORSE ARMOR'S OWN FAMILY (the BODY seat's own mayPlace row",
        "/// THE SPRINT WINDOW'S OWN SPAN",
    ),
    # THE FIELDS' OWN PAIR (the flag + the seat the chest panel doc
    # bounds)
    (
        "    /// THE HORSE SCREEN'S OWN DOUBLE-CLICK FLAG",
        "    /// THE CHEST PANEL'S OWN ORIGIN",
    ),
    # THE INIT'S OWN PAIR (the fresh falses the horse panel bounds)
    (
        "            horse_doubleclick: false,",
        "            horse_panel: (0, 0),",
    ),
    # THE OPEN'S OWN RESET (the fresh pair the close doc bounds)
    (
        "    pub fn open_horse(&mut self) {",
        "    /// THE HORSE'S OWN CLOSE (the vanilla Escape/E on the horse's",
    ),
    # THE CLICK PAIR'S OWN WALK (the doc the inventory-open doc bounds)
    (
        "    /// THE MOUNT WINDOW'S OWN CLICK PAIR (the 551 NEXT's own first",
        "    /// Open the survival inventory over the standing world",
    ),
    # THE Q WALK'S OWN GATE (the wire gate the rects gate bounds)
    (
        "        if self.horse_is_live {",
        "        if self.horse_slot_rects.is_empty() || self.horse_panel == (0, 0) {",
    ),
    # THE RELEASE GATE'S OWN ARM (the left gate the spread comment
    # bounds — the FIRST Horse row the file's own order keeps)
    (
        "            || self.screen == ScreenKind::Horse",
        "        if self.screen != ScreenKind::World || self.paused {",
    ),
    # THE QC ARM'S OWN WALK (the doc the merchant paint doc bounds)
    (
        "    /// THE MOUNT DRAG'S OWN ARM (the merchant's own shape",
        "    /// THE MERCHANT DRAG'S OWN COLLECT",
    ),
    # THE DEFERRED PLACE'S OWN BRANCH (the merchant release tail the
    # inventory fallthrough bounds)
    (
        "                self.click_merchant_slot(mx, my, self.qc_button as i8);",
        "        let panel = self.inv_panel;",
    ),
    # THE RIGHT PRESS'S OWN BRANCH (the merchant right tail the
    # creative bounds)
    (
        "                self.click_merchant_slot(mx, my, 1);",
        "        if self.screen == ScreenKind::Creative {",
    ),
    # THE THROW'S OWN WALK (the doc the chest throw doc bounds)
    (
        "    /// THE HORSE CARRIED'S OWN THROW (the 26.2 mouseClicked's own",
        "    fn on_horse_drop_key_wire(&mut self, whole_stack: bool) {",
    ),
    # THE Q WIRE'S OWN WALK (the doc the inventory-open doc bounds —
    # the same tail the click pair region wears)
    (
        "    /// THE HORSE'S OWN Q WALK (javap 26.2 — the SAME keyPressed",
        "    /// Open the survival inventory over the standing world",
    ),
]

PART11_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
PART11_REGIONS = [
    # THE WIRE TEST'S OWN DOC + BODY (the doc the offline test bounds)
    (
        "/// THE MOUNT WINDOW'S OWN CLICK PAIR (the 551 NEXT's own first lane",
        "/// THE MOUNT WINDOW'S OWN OFFLINE CLICK PAIR",
    ),
    # THE OFFLINE TEST'S OWN BODY (the doc the Q test bounds)
    (
        "/// THE MOUNT WINDOW'S OWN OFFLINE CLICK PAIR",
        "/// THE MOUNT WINDOW'S OWN Q WALK",
    ),
    # THE Q TEST'S OWN BODY (the doc the EOF fallback keeps)
    (
        "/// THE MOUNT WINDOW'S OWN Q WALK",
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
    ok = count_audit(SCREENS_PATH, "pub horse_doubleclick: bool", 1) and ok
    ok = count_audit(SCREENS_PATH, "pub horse_last_click_slot", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn click_horse_slot", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn qc_arm_horse", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn on_horse_drop_key_wire", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn throw_horse_carried_outside", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn horse_seat_may_place", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn horse_menu_stack(", 1) and ok
    ok = count_audit(SCREENS_PATH, "const HORSE_ARMOR_NAMES", 1) and ok
    ok = count_audit(SCREENS_PATH, "self.horse_doubleclick = self.horse_last_click_slot.is_some()", 1) and ok
    ok = count_audit(PART11_PATH, "fn the_horse_the_click_pair_the_wire_the_seats_speak", 1) and ok
    ok = count_audit(PART11_PATH, "fn the_horse_the_click_pair_the_offline_the_rows_wear", 1) and ok
    ok = count_audit(PART11_PATH, "fn the_horse_the_q_walk_the_wire_the_throw_speaks", 1) and ok
    # THE WIDTH LAW (the round's own words the 100 cols pin)
    for path in (SCREENS_PATH, PART11_PATH):
        ok = width_audit(path, "horse_seat_may_place") and ok
        ok = width_audit(path, "horse_menu_stack") and ok
        ok = width_audit(path, "click_horse_slot") and ok
        ok = width_audit(path, "qc_arm_horse") and ok
        ok = width_audit(path, "on_horse_drop_key_wire") and ok
        ok = width_audit(path, "throw_horse_carried_outside") and ok
        ok = width_audit(path, "HORSE_ARMOR_NAMES") and ok
        ok = width_audit(path, "horse_doubleclick") and ok
        ok = width_audit(path, "horse_last_click_slot") and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

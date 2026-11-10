#!/usr/bin/env python3
"""dev-243 — THE FMT REGION WALK (the 535..550 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges keep
(no hunk may touch the round's own edits).

Regions: the round's own mount GUI window the lanes wear. Shapes:
the TIGHT BLOCK (the unique statement lines the round's own edits
bound), the EOF-fallback tail test (the body fn the round's own walk
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

PLAY_PATH = "crates/mc-protocol/src/play.rs"
PLAY_REGIONS = [
    # THE RECORD + THE WIRE WALK (the struct + the parser the
    # Container Close doc bounds; the use-block row the reflow the
    # aliasing own territory wears — the count law the row pins)
    (
        "/// Mount Screen Open: a MOUNT's inventory window OPENS",
        "/// Container Close: the open container window CLOSES",
    ),
    # THE EVENT'S OWN VARIANT (the indented doc the ContainerClosed
    # variant bounds)
    (
        "    /// Mount Screen Open: a MOUNT's inventory window OPENS",
        "    /// Container Close: the open container window CLOSES",
    ),
    # THE DISPATCH'S OWN ARM (the MOUNT event the MERCHANT arm bounds)
    (
        "            clientbound::MOUNT_SCREEN_OPEN => {",
        "            clientbound::MERCHANT_OFFERS => {",
    ),
    # THE PIN'S OWN TEST (the indented walk doc the Set Data doc
    # bounds)
    (
        "    /// Mount Screen Open's wire walk (see [`MountScreenOpen`]",
        "    /// Set Container Data's wire walk",
    ),
]

WORLD_MODEL_PATH = "crates/mc-client/src/world_model.rs"
WORLD_MODEL_REGIONS = [
    # THE ROUTING WORD'S OWN CONST (the doc the OpenContainerState doc
    # bounds)
    (
        "/// THE MOUNT WINDOW'S OWN ROUTING WORD",
        "/// An OPEN container window — the",
    ),
    # THE CONSUMER'S OWN ARM (the MOUNT event the ContainerClosed arm
    # bounds)
    (
        "            PlayEvent::MountScreenOpen(m) => {",
        "            PlayEvent::ContainerClosed(_) => {",
    ),
    # THE STORE WALK'S OWN TEST (the fn the Set Data doc bounds)
    (
        "    fn the_mount_screen_open_births_the_window_store",
        "    /// The Set Data lane's own store walk",
    ),
]

SCREENS_PATH = "crates/mc-client/src/screens.rs"
SCREENS_REGIONS = [
    # THE LIVE WORDS' OWN FIELDS (the flag + the identity the chest
    # panel doc bounds)
    (
        "    /// THE HORSE WINDOW'S OWN LIVE FLAG",
        "    /// THE CHEST PANEL'S OWN ORIGIN",
    ),
    # THE INIT'S OWN PAIR (the fresh falses the horse panel bounds)
    (
        "            horse_is_live: false,",
        "            horse_panel: (0, 0),",
    ),
    # THE VERDICT'S OWN ARM (the OpenInventory caller the Feed arm
    # bounds)
    (
        "                    if let crate::mob::MobInteract::OpenInventory = verdict {",
        "                    if let crate::mob::MobInteract::Feed { love, grow } = verdict {",
    ),
    # THE ESCAPE'S OWN LANE (the Horse arm the Inventory arm bounds)
    (
        "        } else if self.screen == ScreenKind::Horse {",
        "        } else if self.screen == ScreenKind::Inventory {",
    ),
    # THE CLOSE'S OWN RETIRE (the live echo the carried ride bounds)
    (
        "    pub fn close_horse(&mut self, back: ScreenKind) {",
        "        if self.horse_carried.1 > 0 {",
    ),
    # THE WIRE ROUTE'S OWN BRANCH (the MOUNT window the enchant
    # branch bounds)
    (
        "                if c.menu_type_id == crate::world_model::MENU_TYPE_MOUNT {",
        "                if c.menu_type_id == crate::world_model::MENU_TYPE_ENCHANTMENT {",
    ),
]

PART11_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
PART11_REGIONS = [
    # THE WIRE TEST'S OWN DOC (the doc the round's fn bounds)
    (
        "/// THE HORSE'S OWN GUI WINDOW (the 550 NEXT's own first lane",
        None,
    ),
    # THE WIRE TEST'S OWN BODY (the fn the verdict test bounds)
    (
        "fn the_horse_the_gui_window_the_wire_wears",
        None,
    ),
    # THE VERDICT TEST'S OWN DOC (the doc the round's fn bounds)
    (
        "/// THE HORSE'S OWN GUI VERDICT (the mobInteract OpenInventory",
        None,
    ),
    # THE VERDICT TEST'S OWN BODY (the fn the EOF fallback keeps — the
    # walk the tail test wears)
    (
        "fn the_horse_the_gui_verdict_the_screen_opens",
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
    ok = walk(PLAY_PATH, PLAY_REGIONS)
    ok = walk(WORLD_MODEL_PATH, WORLD_MODEL_REGIONS) and ok
    ok = walk(SCREENS_PATH, SCREENS_REGIONS) and ok
    ok = walk(PART11_PATH, PART11_REGIONS) and ok
    # THE COUNT LAW (the identical insert lines the literal sites
    # repeat)
    ok = count_audit(PLAY_PATH, "CB_MOUNT_SCREEN_OPEN as MOUNT_SCREEN_OPEN,", 1) and ok
    ok = count_audit(PLAY_PATH, "pub struct MountScreenOpen", 1) and ok
    ok = count_audit(PLAY_PATH, "fn parse_mount_screen_open", 1) and ok
    ok = count_audit(PLAY_PATH, "PlayEvent::MountScreenOpen", 1) and ok
    ok = count_audit(WORLD_MODEL_PATH, "pub const MENU_TYPE_MOUNT", 1) and ok
    ok = count_audit(WORLD_MODEL_PATH, "MENU_TYPE_MOUNT", 5) and ok
    ok = count_audit(WORLD_MODEL_PATH, "PlayEvent::MountScreenOpen", 2) and ok
    ok = count_audit(SCREENS_PATH, "pub horse_is_live: bool", 1) and ok
    ok = count_audit(SCREENS_PATH, "pub horse_live_window", 1) and ok
    ok = count_audit(SCREENS_PATH, "horse_is_live = true", 1) and ok
    ok = count_audit(SCREENS_PATH, "horse_is_live = false", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn close_horse", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn open_horse", 1) and ok
    ok = count_audit(PART11_PATH, "fn the_horse_the_gui_window_the_wire_wears", 1) and ok
    ok = count_audit(PART11_PATH, "fn the_horse_the_gui_verdict_the_screen_opens", 1) and ok
    # THE WIDTH LAW (the round's own words the 100 cols pin)
    for path in (PLAY_PATH, WORLD_MODEL_PATH, SCREENS_PATH, PART11_PATH):
        ok = width_audit(path, "MOUNT_SCREEN_OPEN") and ok
        ok = width_audit(path, "MENU_TYPE_MOUNT") and ok
        ok = width_audit(path, "MountScreenOpen") and ok
        ok = width_audit(path, "mount_screen_open") and ok
        ok = width_audit(path, "horse_is_live") and ok
        ok = width_audit(path, "horse_live_window") and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

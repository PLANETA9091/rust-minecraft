#!/usr/bin/env python3
"""autopush-543 — THE FMT REGION WALK (the 535/536/537/538/539/540/541/
542 record's own law the round wears): the whole-file rustfmt the
ORIGINAL text the formatted stdout the diff answers — the MY-REGIONS
verdict the marker ranges keep (no hunk may touch the round's own
edits).

Regions: the round's own item-frame kind ride the lanes wear. Two
shapes: the FN-WALK region (the fn's signature the start marker keeps,
the NEXT fn-signature or doc line the end answers) and the TIGHT BLOCK
(the unique statement lines the round's own edits bound).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS_PATH = "crates/mc-client/src/screens.rs"
SCREENS_REGIONS = [
    # THE FRAME'S OWN KIND WORD (the struct field the doc wears)
    (
        "    /// THE FRAME'S OWN KIND WORD (the autopush-543 round — the held",
        "    pub glow: bool,",
    ),
    # THE FACTORY'S OWN FIELD (the placement ctor the NONE init wears)
    (
        "        self.item_frames.push(ItemFrameEnt {",
        "        // THE ANALOG ROW'S OWN BANK (the getItemFrame scan the map's",
    ),
    # THE SWAP'S OWN WORD RIDE (the sneak lane the pair exchange wears)
    (
        "        // THE SNEAK SLOT LANE (the getSlot(0) access the filled frame",
        "        // THE STORE LANE (the EMPTY frame + the held stack — pcs",
    ),
    # THE STORE'S OWN KIND BANK (the placement's row word the frame banks)
    (
        "        // THE STORE LANE (the EMPTY frame + the held stack — pcs",
        "        // THE ROTATE LANE (the filled frame + the empty hand — pcs",
    ),
    # THE PUNCH'S OWN DESTRUCTURE (the kind the tuple rides)
    (
        "        let (filled, glow, cell, contained, kind) = {",
        "        let creative = self.game_mode == GameMode::Creative;",
    ),
    # THE POP'S OWN KIND RIDE (the contained spawn the word re-banks)
    (
        "                    let mut d = crate::drops::ItemDrop::spawn_entity_loot(",
        "                    self.drops.push(d);",
    ),
    # THE BANK'S OWN ROW (the whole bank fn the walk wears)
    (
        "pub fn item_frames_bank_to_nbt(frames: &[ItemFrameEnt]) -> mc_protocol::nbt::Nbt {",
        "pub fn item_frames_from_nbt(value: &mc_protocol::nbt::Nbt) -> Vec<ItemFrameEnt> {",
    ),
    # THE PARSE'S OWN WORD (the whole parse fn the walk wears)
    (
        "pub fn item_frames_from_nbt(value: &mc_protocol::nbt::Nbt) -> Vec<ItemFrameEnt> {",
        "pub fn leash_knots_bank_to_nbt(knots: &[LeashKnotEnt]) -> mc_protocol::nbt::Nbt {",
    ),
]

PART6_PATH = "crates/mc-client/src/screens/tests/part_6.rs"
PART6_REGIONS = [
    # THE ROUNDTRIP'S OWN KIND ROWS (the whole test the walk wears)
    (
        "fn the_item_frames_bank_parse_roundtrip_wears() {",
        "/// THE FRAME PARSE'S OWN TOLERANCE (the same walk the store parse",
    ),
    # THE TOLERANCE'S OWN NONE WALL (the absent kind the assert wears)
    (
        '    assert_eq!(parsed[1].rotation, 1, "the 9 the %8 wrap banks");',
        "    assert_eq!(parsed[1].facing, 3);",
    ),
]

TESTS_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
TESTS_REGIONS = [
    # THE ROUND'S OWN CENSUS (the tail test the EOF fallback wears)
    (
        "/// THE ITEM-FRAME'S OWN KIND RIDE (the autopush-543 round — the NEXT's",
        None,
    ),
]

FN_LINE = re.compile(r"^    (pub )?fn ")
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
    ok = walk(SCREENS_PATH, SCREENS_REGIONS)
    ok = walk(PART6_PATH, PART6_REGIONS) and ok
    ok = walk(TESTS_PATH, TESTS_REGIONS, TEST_FN_LINE) and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

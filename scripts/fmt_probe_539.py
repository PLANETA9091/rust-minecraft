#!/usr/bin/env python3
"""autopush-539 — THE FMT REGION WALK (the 535/536/537/538 record's own
law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions: the round's own qc collect gates the lanes wear. Two shapes:
the FN-WALK region (the fn's signature the start marker keeps, the NEXT
fn-signature line the end answers — the EOF the last test's fallback
wears) and the TIGHT BLOCK (the unique statement lines the round's own
edits bound).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS_PATH = "crates/mc-client/src/screens.rs"
SCREENS_REGIONS = [
    # THE PAINT BEACON'S OWN KIND HALF (the component block the tag
    # verdict wears)
    (
        "        // THE COMPONENT HALF (the 539 round — the family's own gate the",
        "            return; // the component half — the same-id kind pair refuses",
    ),
    # THE PAINT MERCHANT'S OWN KIND HALF (the menu >= 3 read the
    # 8-space paint wears)
    (
        "        let seat_kind = if menu >= 3 && crate::world::brewing_is_potion_kind(seat_id) {",
        "            return; // the component half — the same-id kind pair refuses",
    ),
    # THE BEACON FOLD'S OWN KIND HALF (the 12-space walk re-wears)
    (
        "            let seat_kind = if menu != 0 && crate::world::brewing_is_potion_kind(seat_id) {",
        "                continue; // the component half — the same-id kind pair refuses",
    ),
    # THE MERCHANT FOLD'S OWN KIND HALF (the 12-space menu >= 3 read)
    (
        "            let seat_kind = if menu >= 3 && crate::world::brewing_is_potion_kind(seat_id) {",
        "                continue; // the component half — the same-id kind pair refuses",
    ),
    # THE BEACON FOLD'S OWN PLACE BANK (the chest law the inv_idx
    # seat banks)
    (
        "                // own law the beacon's offline fold re-wears: the empty",
        "                    self.items_potion_kinds[inv_idx] = self.beacon_carried_potion_kind;",
    ),
    # THE MERCHANT FOLD'S OWN PLACE BANK (the menu map the row banks)
    (
        "                // own law the merchant's offline fold re-wears: the",
        "                            self.items_potion_kinds[row] = self.merchant_carried_potion_kind;",
    ),
    # THE WIRE CALLER'S OWN WALK (the whole fn the closure + tail bank
    # + retire wears)
    (
        "    fn qc_distribute_merchant_wire(&mut self) {",
        "    /// THE FURNACE'S WIRE DISTRIBUTION (the offline walk's twin over",
    ),
    # THE WIRE FREE FN'S OWN SHAPE (the signature the kind words wear,
    # the component half the fold re-wears)
    (
        "fn wire_merchant_quickcraft_distribute(",
        "fn wire_brewing_quickcraft_distribute(",
    ),
]

TESTS_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
TESTS_REGIONS = [
    # THE ROUND'S OWN TEST (the file's tail the EOF fallback wears)
    (
        "/// THE QC COLLECT GATES'S OWN BEACON+MERCHANT LANES (the autopush-539",
        None,
    ),
]

FN_LINE = re.compile(r"^    (pub )?fn ")
TEST_FN_LINE = re.compile(r"^fn |^#\[test\]")


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
    ok = walk(TESTS_PATH, TESTS_REGIONS, TEST_FN_LINE) and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

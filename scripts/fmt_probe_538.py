#!/usr/bin/env python3
"""autopush-538 — THE FMT REGION WALK (the 535/536/537 record's own
law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions: the round's own carried-kind walk the lanes wear. Two
shapes: the FN-WALK region (the fn's signature the start marker
keeps, the NEXT fn-signature line the end answers) and the TIGHT
PAIR (the unique statement/doc lines the round's own edits bound —
the live_container_updated lane the pre-existing rustfmt dirt the
region never sweeps).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS_PATH = "crates/mc-client/src/screens.rs"
# (start_marker, end_marker_or_None) — None = the next fn-signature line.
SCREENS_REGIONS = [
    ("    pub beacon_carried: (i32, i32),", "    pub beacon_mirror: Vec<(i32, i32)>,"),
    ("    pub merchant_carried: (i32, i32),",
     "    /// THE ACTIVE OFFER'S OWN BANK (javap 26.2 MerchantContainer —"),
    ("            beacon_carried: (0, 0),", "            beacon_mirror: Vec::new(),"),
    ("            merchant_carried: (0, 0),", "            merchant_active_offer: None,"),
    ("    pub fn open_beacon_for(&mut self, pos: (i32, i32, i32)) {",
     "    /// THE BEACON'S OWN CLOSE (javap 26.2 BeaconMenu.removed pc 0-45"),
    ("    pub fn close_beacon(&mut self, back: ScreenKind) {",
     "    /// THE MERCHANT'S OWN CLOSE (javap of `MerchantMenu.removed` —"),
    ("    pub fn close_merchant(&mut self, back: ScreenKind) {",
     "    fn throw_merchant_carried_outside(&mut self, button: i8) {"),
    ("    fn throw_merchant_carried_outside(&mut self, button: i8) {",
     "    /// THE MERCHANT'S OWN SLOT WALK (javap 26.2 AbstractContainerMenu."),
    ("    fn throw_beacon_carried_outside(&mut self, button: i8) {",
     "    /// THE POWER SEAT'S OWN PRESS (javap 26.2 BeaconScreen$"),
    ("    fn click_merchant_slot(&mut self, mx: i32, my: i32, button: i8) {",
     "    /// THE MERCHANT GATHER'S OWN REFUSAL (javap 26.2 MerchantMenu —"),
    ("    fn merchant_result_take_click(&mut self, button: i8) {",
     "    /// THE RESULT SOURCE'S OWN QUICK-MOVE (javap 26.2 MerchantMenu."),
    ("    fn beacon_screen_click(&mut self, mx: i32, my: i32, button: i8) {",
     "    fn throw_beacon_carried_outside(&mut self, button: i8) {"),
    ("        self.beacon_carried = if k > 0 { (carried.0, k) } else { (0, 0) };",
     "    /// THE MERCHANT'S OWN OFFLINE DISTRIBUTION (the doClick"),
    ("    fn qc_distribute_merchant_offline(&mut self) {",
     "    /// THE MERCHANT'S WIRE DISTRIBUTION (the offline walk's twin over"),
    ("    pub fn on_right_press(&mut self, mx: i32, my: i32) -> bool {", None),
    ("                    self.beacon_carried = c.carried;",
     "                    self.beacon_live_window = Some((c.id, c.state_id));"),
    ("                    self.merchant_carried = c.carried;",
     "                    self.merchant_title = decode_container_title(&c.title_nbt);"),
    ("                self.beacon_carried = (0, 0);",
     "                self.beacon_payment = (0, 0);"),
]

TESTS_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
TESTS_REGIONS = [
    ("fn the_merchant_beacon_carried_the_words_the_walks_bank() {",
     "/// THE KINDLESS CARRIED SEATS'S OWN VERDICTS (the autopush-538 round"),
    ("fn the_kindless_carried_seats_the_verdicts_keep() {",
     "/// THE BEACON WIRE SWAP'S OWN KIND VERDICTS (the autopush-527 round —"),
]

FN_LINE = re.compile(r"^    (pub )?fn ")


def region_bounds(lines, start, end):
    si = lines.index(start)
    if end is None:
        ei = None
        for j in range(si + 1, len(lines)):
            if FN_LINE.match(lines[j]):
                ei = j
                break
        if ei is None:
            raise SystemExit(f"NO END FN after {start!r}")
    else:
        ei = lines.index(end, si + 1)
    return (si, ei)


def walk(path, regions) -> bool:
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
        ms, me = region_bounds(mine_lines, start, end)
        ts, te = region_bounds(theirs_lines, start, end)
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

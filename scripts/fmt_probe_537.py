#!/usr/bin/env python3
"""autopush-537 — THE FMT REGION WALK (the 535/536 record's own law
the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions: the round's own nine carried-throw lanes (each fn's
signature the start marker keeps, the NEXT fn's signature the end
answers — the scan-forward walk the region bounds).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS_PATH = "crates/mc-client/src/screens.rs"
THROW_FNS = [
    "    fn throw_anvil_carried_outside(&mut self, button: i8) {",
    "    fn throw_merchant_carried_outside(&mut self, button: i8) {",
    "    fn throw_beacon_carried_outside(&mut self, button: i8) {",
    "    fn throw_carried_outside(&mut self, button: i8) {",
    "    fn throw_furnace_carried_outside(&mut self, button: i8) {",
    "    fn throw_chest_carried_outside(&mut self, button: i8) {",
    "    fn throw_brewing_carried_outside(&mut self, button: i8) {",
    "    fn throw_enchanting_carried_outside(&mut self, button: i8) {",
    "    fn throw_dispenser_carried_outside(&mut self, button: i8) {",
]

FN_LINE = re.compile(r"^    (pub )?fn ")


def regions_for(lines, markers):
    """start = marker line index; end = next fn-signature line after."""
    out = []
    for m in markers:
        si = lines.index(m)
        ei = None
        for j in range(si + 1, len(lines)):
            if FN_LINE.match(lines[j]):
                ei = j
                break
        if ei is None:
            raise SystemExit(f"NO END FN after {m!r}")
        out.append((si, ei))
    return out


def main() -> int:
    ok = True
    path = SCREENS_PATH
    original = open(path, encoding="utf-8").read()
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path], capture_output=True, text=True
    )
    if proc.returncode != 0:
        raise SystemExit(f"RUSTFMT FAILED\n{proc.stderr[:400]}")
    formatted = proc.stdout
    mine_regions = regions_for(original.split("\n"), THROW_FNS)
    theirs_regions = regions_for(formatted.split("\n"), THROW_FNS)
    for (ms, me), (ts, te) in zip(mine_regions, theirs_regions):
        mine = "\n".join(original.split("\n")[ms:me])
        theirs = "\n".join(formatted.split("\n")[ts:te])
        label = original.split("\n")[ms].strip()[:58]
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
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

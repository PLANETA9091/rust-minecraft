#!/usr/bin/env python3
"""autopush-322 — THE FMT SPLICE WALK (the 320 record's own law the
round wears): the rustfmt text the MINE hunks the splice serves — the
pre-existing static the HEAD law keeps (the non-mine hunks the file
never wears). The hunk filter the orig-line ranges keep: only the
ranges the round's own edits bank ride the splice.

Usage: python3 scripts/fmt_splice_322.py
"""
import difflib
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

# the MINE line ranges per file (the round's own edits the hunks must
# fall inside — the pre-existing static the ranges never cover)
MINE = {
    "crates/mc-client/src/screens.rs": [
        (11759, 11870),   # the bank/parse walk
        (12230, 12280),   # the ctor init rows
        (17090, 17140),   # the tick lane call
        (27230, 27770),   # the open/close/click/confirm/quick-move/reapply block
        (31790, 32060),   # the render fn
        (54900, 54950),   # the door arm
        (7080, 7130),     # the ScreenKind variant
        (35900, 35930),   # the press dispatch arm
        (36150, 36190),   # the right press arm
        (23985, 24005),   # the E-key close arm
        (49320, 49345),   # the removed close arm
        (62590, 62640),   # the portal pair
        (66930, 66960),   # the render match arm
        (41280, 41300),   # the load row ride
        (41385, 41400),   # the save row ride
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        (3700, 3730),     # the 320 round-trip update
        (3730, 4220),     # the two REAL tests + the helper
    ],
    "crates/mc-client/src/world.rs": [
        (4725, 4950),     # the law layer
    ],
    "crates/mc-client/src/gui.rs": [
        (315, 350),       # the beacon sprite lane
    ],
}


def mine_hunk(path, i1, i2):
    for lo, hi in MINE.get(path, []):
        if lo <= i1 and i2 <= hi:
            return True
        # the hunk INSIDE the range even when the edges touch
        if i1 >= lo and i1 < hi:
            return True
    return False


def main():
    for path, ranges in MINE.items():
        orig = open(path).read().split("\n")
        r = subprocess.run([RUSTFMT, "--emit", "stdout", path],
                           capture_output=True, text=True)
        fmt = r.stdout.split("\n")
        sm = difflib.SequenceMatcher(None, orig, fmt, autojunk=False)
        out = list(orig)
        rides = 0
        for tag, i1, i2, j1, j2 in reversed(sm.get_opcodes()):
            if tag == "equal":
                continue
            if not mine_hunk(path, i1, i2):
                continue
            out[i1:i2] = fmt[j1:j2]
            rides += 1
        if rides:
            open(path, "w").write("\n".join(out))
        print(f"SPLICE {path}: {rides} hunk(s) rode")
    return 0


if __name__ == "__main__":
    sys_exit = main()

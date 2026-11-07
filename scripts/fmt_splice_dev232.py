#!/usr/bin/env python3
"""dev-232 — THE EOF SPLICE (the 497 record's own law the round
wears): the round's own test the tail region the rustfmt stdout the
EOF splice wears — the region the probe the DIRTY verdict spoke, the
formatted bytes the file's tail replaces, the rest the original keeps.
"""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
PATH = "crates/mc-client/src/screens/tests/part_11.rs"
START = "/// THE CLONE'S OWN FURNACE RIDE (the 497 NEXT's own first lane"


def main() -> int:
    original = open(PATH, encoding="utf-8").read()
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", PATH],
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        print(f"RUSTFMT FAILED\n{proc.stderr[:400]}")
        return 1
    formatted = proc.stdout
    s = original.find(START)
    if s < 0:
        print("START MARKER NOT FOUND in original")
        return 1
    fs = formatted.find(START)
    if fs < 0:
        print("START MARKER NOT FOUND in formatted")
        return 1
    spliced = original[:s] + formatted[fs:]
    open(PATH, "w", encoding="utf-8").write(spliced)
    print("SPLICED (the tail the formatted bytes wear)")
    return 0


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""autopush-453 — THE FMT REGION WALK (the 372..dev-221 record's own law
the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the inventory close's own window-0 echo block
                  the live lane wears (no rustfmt
                  hunk may touch it).
  part_11.rs    — the round's own test (the window-0 the echo
                  wears, EOF the end the round keeps).
"""
import difflib
import os

import subprocess
import sys


RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the inventory close's own window-0 echo (the false gate the
    # offline truth keeps — the vanilla law the sibling walks re-wear
    # marker keeps)
    (
        "THE WINDOW-0'S OWN ECHO (the vanilla LocalPlayer.closeContainer",
        "if self.inv_carried.1 > 0 {",
    ),
]

PART11 = [
    # the quickcraft kin's own echo drain the switch serves (the lane
    # the Close{0} the open(Chest) inventory close rides first)
    (
        "THE INVENTORY'S OWN ECHO the switch speaks (the 453 round's own",
        "let panel = chest_panel_origin(640, 360);",
    ),
    # the round's own test (the window-0 the echo wears — the
    # EOF the end the round keeps)
    (
        "THE INVENTORY CLOSE'S OWN ECHO GATE (the 221 round's own census",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/screens/tests/part_11.rs": PART11,
}


def find(text: str, marker: str, start: int = 0) -> int:
    i = text.find(marker, start)
    if i < 0:
        raise SystemExit(f"MARKER NOT FOUND: {marker!r}")
    return i


def region_end(text: str, marker: str, start_at: int) -> int:
    if marker is None:
        return len(text)
    return find(text, marker, start_at)


def formatted_of(path: str) -> str:
    """the formatted text the stdout answers — the filename header this
    rustfmt wears the strip removes (the header the j-side only rides,
    the region hunk math the i-side keeps)"""
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise SystemExit(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
    out = proc.stdout
    lines = out.split("\n")
    # the header this rustfmt wears — the CANONICAL path the first line
    # speaks + the blank the second keeps (the relative arg the abs
    # prefix never matches — the endswith-colon shape the rule reads)
    if len(lines) >= 2 and lines[0].endswith(":") and lines[1] == "":
        out = "\n".join(lines[2:])
    return out


def main() -> int:
    ok = True
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        formatted = formatted_of(path)
        sm = difflib.SequenceMatcher(
            None, original.splitlines(), formatted.splitlines())
        hunks = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == "equal":
                continue
            hunks.append((i1, i2, tag, j1, j2))
        cursor = 0  # the sequential threading — the shared texts the order keys
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker, cursor)
            e = region_end(original, end_marker, s)
            cursor = e
            s_line = original[:s].count("\n")
            e_line = original[:e].count("\n")
            bad = [h for h in hunks if h[0] < e_line and h[1] > s_line]
            label = start_marker.strip()[:56].replace("\n", " | ")
            if bad:
                ok = False
                print(f"{path}: REGION NOT FMT-CLEAN [{label}] hunks={bad}")
            else:
                print(f"{path}: region fmt-clean [{label}]")
    if ok:
        print("ALL-REGIONS-CLEAN")
    else:
        print("REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

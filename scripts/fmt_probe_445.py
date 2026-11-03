#!/usr/bin/env python3
"""autopush-445 — THE FMT REGION WALK (the 372..autopush-417 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text
the formatted stdout the diff answers — the MY-REGIONS verdict the
marker ranges keep (no hunk may touch the round's own edits).

Regions:
  part_11.rs    — the round's own test (the chest gather the release
                  arm wears, EOF the end the round keeps).
  screens.rs    — the round's own regions the trio + the press walk
                  + the field pair wear (no rustfmt hunk may touch
                  the gather trio, the flag compute, the resets).
"""
import difflib
import os

import subprocess
import sys


RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the field pair the round banks (the flag + the seat the docs
    # speak — the quickcraft doc the tail markers)
    (
        "THE CHEST SCREEN'S OWN DOUBLE-CLICK FLAG",
        "THE QUICKCRAFT DRAG'S OWN ARM (the `AbstractContainerScreen.",
    ),
    # the gather trio the round banks (the release arm + the offline
    # walk + the wire half — the wire spread's own doc the tail
    # markers)
    (
        "THE CHEST GATHER'S OWN RELEASE ARM",
        "THE WIRE SPREAD'S OWN WALK",
    ),
    # the open reset the round banks
    (
        "THE FRESH SESSION'S OWN RESET",
        "chest_window_shulker = false;",
    ),
    # the close reset the round banks
    (
        "THE GATHER PAIR'S OWN DEATH",
        "chest_carried_potion_kind = crate::brewing_sim::POTION_KIND_NONE;",
    ),
    # the press walk's own flag compute (the double-click flag the
    # dispatch BEFORE keeps — the furnace arm the tail markers)
    (
        "THE DOUBLE-CLICK FLAG (the inventory's own shape",
        "if self.screen == ScreenKind::Furnace {",
    ),
    # the route reset the round banks (the fresh state's own law the
    # window retires — the authoritative carried the tail markers)
    (
        "THE FRESH STATE'S OWN RESET (the open walk's own law\n                // the enchant route wears",
        "chest_carried = c.carried;",
    ),
    # the window-death reset the round banks (the None arm the pair
    # retires — the carried-kind word the tail markers)
    (
        "the gather pair the window's death retires",
        "chest_carried_potion_kind = crate::brewing_sim::POTION_KIND_NONE; // the word the window's death retires",
    ),
]

PART11 = [
    # the round's own test (the chest gather the release arm wears —
    # the EOF the end the round keeps)
    (
        "THE CHEST SEATS' OWN GATHER",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens/tests/part_11.rs": PART11,
    "crates/mc-client/src/screens.rs": SCREENS,
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

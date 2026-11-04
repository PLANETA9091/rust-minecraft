#!/usr/bin/env python3
"""autopush-451 — THE FMT REGION WALK (the 372..autopush-450 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  part_11.rs    — the round's own test (the anvil gather the release
                  arm wears, EOF the end the round keeps).
  screens.rs    — the round's own regions the field pair + the release
                  chain arm + the gather trio + the open/close resets +
                  the press walk wear (no rustfmt hunk may touch them).
"""
import difflib
import os

import subprocess
import sys


RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the field pair the round banks (the flag + the seat the docs
    # speak — the rects field the tail marker)
    (
        "THE ANVIL SCREEN'S OWN DOUBLE-CLICK FLAG",
        "pub anvil_slot_rects",
    ),
    # the release chain's own arm the round banks (the spread the
    # anvil branch serves — the else tail the chain keeps)
    (
        "} else if self.screen == ScreenKind::Anvil {\n            // THE ANVIL GATHER'S OWN ARM",
        "self.on_anvil_release_pickup_all()",
    ),
    # the gather trio the round banks (the release arm + the offline
    # walk + the wire walk — the wire spread's own doc the tail
    # markers)
    (
        "THE ANVIL GATHER'S OWN RELEASE ARM",
        "THE WIRE SPREAD'S OWN WALK",
    ),
    # the open reset the round banks (the anvil's own open the pair
    # retires — the transient trio the tail marker)
    (
        "THE STALE GATHER PAIR'S OWN RETIRE (the vanilla fresh-screen",
        "THE TRANSIENT TRIO (the ItemCombinerMenu's own SimpleContainer(2)",
    ),
    # the close reset the round banks (the anvil's own close the pair
    # death wears — the title's own retire the tail marker)
    (
        "THE GATHER PAIR'S OWN DEATH (the removed() walk the 450",
        "self.live_anvil_title = None;",
    ),
    # the press walk's own flag compute (the double-click flag the
    # gate serves — the chest arm the tail markers)
    (
        "if self.screen == ScreenKind::Anvil {\n            // THE DOUBLE-CLICK FLAG (the enchanting's own shape",
        "if self.screen == ScreenKind::Chest {\n            // THE DOUBLE-CLICK FLAG (the inventory's own shape",
    ),
]

PART11 = [
    # the round's own test (the anvil gather the release arm wears —
    # the EOF the end the round keeps)
    (
        "THE ANVIL SEATS THE GATHER WEARS",
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

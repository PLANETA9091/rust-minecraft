#!/usr/bin/env python3
"""autopush-428 — THE FMT REGION WALK (the 372..autopush-417 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the drag arm's own un-refusal (the chest's own gate
               the live lane re-wears — the 428 round's own edit),
               the wire lanes (the offline walk's twin the free fn
               keeps), the release dispatch's own split, the deferred
               place's own comment walk, the free fn the packet triple
               mints.
  part_11.rs — the round's own echo test (the triple + the refusals),
               the 426 test's own comment walk (the stale phrases the
               honest words retire).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the drag arm's own un-refusal (the chest's own gate the live
    # lane re-wears — the 428 round's own edit)
    (
        "/// THE ENCHANTING DRAG'S OWN ARM",
        "/// THE ENCHANTING DRAG'S OWN COLLECT",
    ),
    # the wire lanes (the offline walk's twin the free fn keeps — the
    # 428 round's own fn + the offline doc's own refresh)
    (
        "/// THE ENCHANT DRAG'S OWN WIRE LANES",
        "fn qc_distribute_enchanting_offline(&mut self)",
    ),
    # the release dispatch's own split (the wire the window belief
    # rides, the offline sim the local write-through answer)
    (
        "THE TRANSIENT PAIR'S OWN LANES (the click walk's own",
        "self.qc_distribute_enchanting_offline();",
    ),
    # the deferred place's own comment walk (the stale phrases the
    # honest words retire)
    (
        "// THE TRANSIENT PAIR'S OWN DEFERRED PLACE",
        "if self.screen == ScreenKind::Anvil {",
    ),
    # the free fn the packet triple mints (the chest's own lane the
    # shape mirrors)
    (
        "/// THE ENCHANT DRAG'S OWN WIRE DISTRIBUTION",
        "/// THE HOVERED SEAT'S OWN MENU id",
    ),
]

PART11 = [
    # the 426 test's own comment walk (the stale phrases the honest
    # words retire — the doc, the ARM E block, the tail assert)
    (
        "/// THE ENCHANTING TABLE'S OWN DEFERRED PLACE",
        "/// THE ANVIL'S OWN DEFERRED PLACE",
    ),
    # the round's own echo test (the triple + the refusals)
    (
        "/// THE ENCHANTING'S OWN QUICKCRAFT ECHO",
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


def main() -> int:
    ok = True
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        proc = subprocess.run(
            [RUSTFMT, "--emit", "stdout", path],
            capture_output=True, text=True)
        if proc.returncode != 0:
            print(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
            ok = False
            continue
        formatted = proc.stdout
        sm = difflib.SequenceMatcher(None, original.splitlines(), formatted.splitlines())
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

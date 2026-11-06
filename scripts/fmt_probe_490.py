#!/usr/bin/env python3
"""autopush-490 — THE FMT REGION WALK (the 489 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  drops.rs    — the gated scan + the candidate gate (the
                getSlotWithRemainingSpace delegate the gate answers) +
                the gated walk (the add delegate + the add_gate loop)
                + the gated touch (the wrapper + the gate body).
  screens.rs  — the merge-law gate wire (the ench_gate closure the
                pickup walk wears, the player_touch_gate call).
  part_11.rs  — the 489 test's own third arm the vanilla law
                corrects + the round's own tests (the tail the EOF
                keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

DROPS = [
    # the gated scan + the candidate gate (the delegate the plain
    # scan re-wears, the gate the components speak)
    (
        "self.get_slot_with_remaining_space_gate(selected, id, &|_| true)",
        "/// `getFreeSlot` (javap)",
    ),
    # the gated walk (the add delegate + the add_gate loop)
    (
        "THE MERGE LAW'S OWN GATED WALK",
        "/// The hotbar row's projection",
    ),
    # the gated touch (the wrapper + the gate body the caller wears)
    (
        "player_touch_gate(drop, inventory, selected, creative, &|_| true)",
        "/// THE DAMAGE BAR'S OWN STACK LAWS",
    ),
]

SCREENS = [
    # the merge-law gate wire (the ench_gate closure + the gated call)
    (
        "let mut moved = false;",
        "crate::drops::Touch::Picked { .. } => {",
    ),
]

PART11 = [
    # the 489 third arm the round corrects + the two new tests (the
    # tail the EOF keeps)
    (
        "// THE REFUSE-TO-MERGE'S OWN LAW",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/drops.rs": DROPS,
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

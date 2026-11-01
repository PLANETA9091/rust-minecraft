#!/usr/bin/env python3
"""autopush-411 — THE FMT REGION WALK (the 372..autopush-410 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  drops.rs      — the kind word's own field (the struct doc + the four
                  ctor sites the NONE default wears).
  screens.rs    — the shot beat's own kind ride (the seat word's read
                  + the drain's own retire + the spawn lane's lend) +
                  the pickup walk's own landing ride.
  part_11.rs    — the round's own test (the shot ride's arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

DROPS = [
    # the struct field (the kind word's own declaration)
    (
        "    /// The potion's own kind word",
        "    pub x: f64,",
    ),
    # the entity-loot ctor (the NONE default wears)
    (
        "    pub fn spawn_entity_loot",
        "    /// THE DEATH DROP'S OWN SPAWN",
    ),
    # the death ctor (the NONE default wears)
    (
        "    /// THE DEATH DROP'S OWN SPAWN",
        "    /// THE CONTAINER SPILL'S OWN SPAWN",
    ),
    # the spill ctor (the NONE default wears)
    (
        "            // THE SPILL'S OWN THROW",
        "                pickup_delay: POP_PICKUP_DELAY,",
    ),
]

SCREENS = [
    # the pickup walk's own landing ride (the fill arm the word banks)
    (
        "                            // THE WORD BEFORE THE TAKE",
        "                    if touched {",
    ),
    # the seat word's own read (the 410 normalized law the shot re-wears)
    (
        "            let (item, count) = slots[slot_i];\n            // THE SEAT'S OWN WORD",
        '            let arrow_id = mc_registry::item_id_by_name("arrow").unwrap_or(-1);',
    ),
    # the drain's own retire (the bank cell the emptied seat answers)
    (
        "            // THE DRAIN'S OWN RETIRE",
        "            // THE EJECT'S OWN ENTITY",
    ),
    # the spawn lane's own kind lend (the drop the seat's word wears)
    (
        "                // THE SPAWN LANE'S OWN KIND LEND",
        "                self.drops.push(drop);",
    ),
]

PART11 = [
    # the round's own test (the shot ride's arms)
    (
        "/// THE DISPENSER SHOT'S OWN KIND RIDE",
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

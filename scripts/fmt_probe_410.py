#!/usr/bin/env python3
"""autopush-410 — THE FMT REGION WALK (the 372..autopush-409 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the grid nine's own kind bank (the field doc + the
                  open load + the close flush + the normalized read +
                  the quick-move's own kind lane + the pickup's own
                  kind lane).
  part_11.rs    — the round's own test (the bank's arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the field doc (the bank's own declaration)
    (
        "THE GRID NINE'S OWN KIND BANK (the autopush-410 round",
        "The dispenser's slot hit boxes",
    ),
    # the open load (the row bank's own 363 law the nine wears)
    (
        "        // THE ROW BANK'S OWN LOAD (the chest's own 363 law",
        "        self.dispenser_open_cell = Some(pos);",
    ),
    # the normalized read (the id wall the word never lends)
    (
        "    /// THE ROW BANK'S OWN NORMALIZED READ",
        "    /// THE DISPENSER'S OWN CLOSE",
    ),
    # the close flush (the ride home the map entry banks)
    (
        "            // THE ROW BANK'S OWN RIDE HOME (the chest's own 363 law",
        "            self.container_stores_file_save();",
    ),
    # the quick-move's own kind lane (the ride + the tail the bank answers)
    (
        "            // THE KIND'S OWN PRE-READ (the source row's word the tail",
        "            self.dispenser_mirror_sync_from_inventory();",
    ),
    # the pickup's own kind lane (the read + the ride the bank answers)
    (
        "        // THE KIND'S OWN READS (the hoist the component law pre-keys —\n        // the banks' own normalized verdicts the pickup pair's answer",
        "        let old_carried_kind = self.dispenser_carried_potion_kind;",
    ),
    (
        "            if menu < 9 {\n                // THE GRID'S OWN BANK RIDE",
        "            self.dispenser_carried_potion_kind = new_carried_kind;",
    ),
]

PART11 = [
    # the round's own test (the bank's arms)
    (
        "/// THE DISPENSER SEATS' OWN KIND BANK",
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

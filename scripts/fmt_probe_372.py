#!/usr/bin/env python3
"""autopush-372 — THE FMT REGION WALK (the 323/204/358/360/361/362/363/
364/365/366/367/368/369/370/371 record's own law the round wears): the
whole-file rustfmt the ORIGINAL text the formatted stdout the diff
answers — the MY-REGIONS verdict the marker ranges keep (no hunk may
touch the round's own edits).

Regions:
  world_view.rs — the component law's own verdict + the kind-aware
                  pickup pair (the SAME-ID KIND PAIR'S OWN MODEL RIDE).
  screens.rs    — the chest lane's own hoisted reads + the kind-aware
                  call + the re-keyed swap arm.
  part_11.rs    — the round's own test tail (the model ride walk).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

WORLD_VIEW = [
    # the component law's own verdict + the kind-aware pickup pair
    ("/// THE SAME-ID KIND PAIR'S OWN COMPONENT LAW (the autopush-372 round —",
     "/// The vanilla `AbstractContainerMenu.doClick` PICKUP_ALL prediction"),
]

SCREENS = [
    # the chest lane's own hoisted reads + the kind-aware call + the
    # re-keyed swap arm (the block the bank writes close)
    ("        // THE KIND'S OWN READS (the hoist the component law pre-keys —",
     "        self.chest_carried = new_carried;"),
]

PART11 = [
    # the round's own test tail
    ("/// THE SAME-ID KIND PAIR'S OWN MODEL RIDE (the autopush-372 round —",
     None),
]

REGIONS = {
    "crates/mc-client/src/world_view.rs": WORLD_VIEW,
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
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker)
            e = region_end(original, end_marker, s)
            s_line = original[:s].count("\n")
            e_line = original[:e].count("\n")
            bad = [h for h in hunks if h[0] < e_line and h[1] > s_line]
            label = start_marker.strip()[:56]
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

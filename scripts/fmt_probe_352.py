#!/usr/bin/env python3
"""autopush-352 — THE FMT REGION WALK (the 323 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits), the pre-existing
static noise the HEAD law confirms the non-mine hunks the honest
inventory banks.

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

# the screens.rs regions (the doc-head anchors the round's own edits
# keep — each block the 352 round's own doc words wear)
SCREENS = [
    # the enchant hover-tooltip seat (the highlight block the tail keeps)
    ("    /// THE ENCHANTING SCREEN'S OWN TOOLTIP SEAT (the 352 round", "    /// THE F4 SWITCHER'S OWN RENDER"),
    # the render call site (the item tooltip the bundle lane keeps)
    ("        // THE ITEM TOOLTIP (the extractTooltip pass — the 352 round", "        self.render_bundle_tooltip(g, a);"),
]
PART11 = [
    ("THE ENCHANTING SCREEN'S OWN TOOLTIP SEAT (the 352 round", None),
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
            if tag != "equal":
                hunks.append((i1, i2))
        for head_marker, tail_marker in regions:
            start = find(original, head_marker)
            end = region_end(original, tail_marker, start)
            start_line = original.count("\n", 0, start)
            end_line = original.count("\n", 0, end)
            overlap = [
                (a, b) for (a, b) in hunks
                if a < end_line and b > start_line
            ]
            if overlap:
                ok = False
                print(f"{path}: REGION DIRTY [{start_line},{end_line}) "
                      f"head={head_marker[:60]!r} hunks={overlap}")
            else:
                print(f"{path}: region clean [{start_line},{end_line}) "
                      f"head={head_marker[:48]!r}")
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

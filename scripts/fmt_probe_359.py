#!/usr/bin/env python3
"""autopush-359 — THE FMT REGION WALK (the 323/204/358 record's own
law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits), the
pre-existing static noise the HEAD law confirms the non-mine hunks
the honest inventory banks.

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the bin destroy + the wrapped pickup pair (the word drops the
    # four arms the broadcast diff doc keeps)
    ("                    // THE CARRIED DESTROYS (pc 238-251 —",
     "        // THE BROADCAST'S OWN DIFF (the live session's walk"),
    # the palette eat arms (the shrink the take doc keeps)
    ("                // the shrink(1) the palette eats — the emptied carried",
     "        // the empty carried: the take (the hovered stack's own count"),
    # the store ride (the kind block the live word comment keeps)
    ("        // THE KIND'S OWN CURSOR RIDE (the dev-204 round's law the\n        // picker's own bottom row re-wear",
     "        // THE LIVE WORD (the listener law the swap lane speaks"),
    # the wrapped ghost tint (the carried follows the tab hover keeps)
    ("            // the carried follows (the shared law — the tail walk the",
     "                if let Some(idx) = self.creative_tab_at(mx, my) {"),
    # the canvas ghost tint (the held stack the tab label keeps)
    ("        // THE HELD STACK follows the cursor (the vanilla carried render;",
     "        // THE TAB'S OWN HOVER LABEL (the checkTabHovering walk"),
]
PART11 = [
    ("THE CREATIVE WRAP'S OWN RIDE (the autopush-359 round", None),
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

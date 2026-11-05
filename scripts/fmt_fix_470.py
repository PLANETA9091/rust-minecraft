#!/usr/bin/env python3
"""fmt_fix_470 — the surgical region formatter (the probe's own law the
fixer serves): the whole-file rustfmt output the REGION-restricted
hunks the original text applies — the round's own regions the fix
wears, the OTHER regions the drift never touches.

Regions:
  screens.rs    — the round's own trio (the debug search box the
                  copy/cut/paste seats wear).
  window.rs     — the chord wiring's own edit (the 67/86/88 arms
                  wear).
  part_11.rs    — the round's own test (the clipboard-trio walk
                  wears, EOF the end the round keeps).
Usage: python3 scripts/fmt_fix_470.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the round's own trio (the copy/cut/paste seats wear — the
    # debug-box wording the marker disambiguates)
    (
        "THE SEARCH BOX'S OWN COPY SEAT (the 470 census's isCopy arm",
        "THE DIRECT CONNECTION'S OWN ENTER RIDE (javap 26.2",
    ),
]

WINDOW = [
    # the chord wiring's own edit (the 67/86/88 arms wear — the
    # shift-seat comment the anchor keeps)
    (
        "THE SEARCH BOX'S OWN CLIPBOARD CHORD (the 470 census's",
        "THE SELECTION'S OWN SHIFT SEAT (the hasShiftDown query",
    ),
]

PART11 = [
    # the round's own test (the clipboard-trio walk wears — EOF the end keeps)
    (
        "THE DEBUG SEARCH BOX'S OWN CLIPBOARD TRIO (the 470 census's",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/window.rs": WINDOW,
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
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise SystemExit(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
    out = proc.stdout
    lines = out.split("\n")
    if len(lines) >= 2 and lines[0].endswith(":") and lines[1] == "":
        out = "\n".join(lines[2:])
    return out


def main() -> int:
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        orig_lines = original.split("\n")
        formatted = formatted_of(path)
        sm = difflib.SequenceMatcher(None, orig_lines, formatted.split("\n"))
        spans = []
        cursor = 0
        for (start_marker, end_marker) in regions:
            s_ = find(original, start_marker, cursor)
            e = region_end(original, end_marker, s_)
            cursor = e
            s_line = original[:s_].count("\n")
            e_line = original[:e].count("\n")
            for tag, i1, i2, j1, j2 in sm.get_opcodes():
                if tag == "equal":
                    continue
                if i1 < e_line and i2 > s_line:
                    spans.append((i1, i2, j1, j2))
        if not spans:
            print(f"[CLEAN] {path}")
            continue
        # the reverse walk the line numbers the shifts keep
        out_lines = list(orig_lines)
        for i1, i2, j1, j2 in reversed(spans):
            out_lines[i1:i2] = formatted.split("\n")[j1:j2]
        open(path, "w", encoding="utf-8").write("\n".join(out_lines))
        print(f"[FIXED] {path} :: {len(spans)} hunks")
    return 0


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""fmt_fix_471 — the surgical region formatter (the probe's own law the
fixer serves): the whole-file rustfmt output the REGION-restricted
hunks the original text applies — the round's own regions the fix
wears, the OTHER regions the drift never touches.

Regions: the same families the fmt_probe_471 walk wears (the five
boxes' copy/cut/paste seats + the six part_11 tests' own re-bank).
Usage: python3 scripts/fmt_fix_471.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    (
        "THE SEARCH BOX'S OWN COPY SEAT (the 471 re-wear",
        "THE SEARCH BOX'S OWN CLICK SEAT (the EditBox.mouseClicked walk",
    ),
    (
        "THE ANVIL FIELD'S OWN COPY SEAT (the 471 re-wear",
        "THE ANVIL FIELD'S OWN X-BAND SEAT (the private seat compute",
    ),
    (
        "THE DIRECT FIELD'S OWN COPY SEAT (the 471 re-wear",
        "THE ENTRY FORM'S OWN SHIFT RIDE (the two-arg walk the FOCUSED",
    ),
    (
        "THE PAIR'S OWN COPY SEAT (the 471 re-wear",
        "THE ENTRY FORM'S OWN BOX CLICK (the EditBox.mouseClicked walk",
    ),
    (
        "THE CHAT FIELD'S OWN COPY SEAT (the 471 re-wear",
        "THE SELECTION'S OWN WORD SEAT (the vanilla double-click walk",
    ),
]

PART11 = [
    (
        "fn the_anvil_field_the_clipboard_walk_wears() {",
        "fn the_chat_field_the_clipboard_sibling_wears() {",
    ),
    (
        "fn the_chat_field_the_clipboard_sibling_wears() {",
        "fn the_direct_field_the_clipboard_sibling_wears() {",
    ),
    (
        "fn the_direct_field_the_clipboard_sibling_wears() {",
        "fn the_editserver_pair_the_clipboard_siblings_wear() {",
    ),
    (
        "fn the_editserver_pair_the_clipboard_siblings_wear() {",
        "fn the_creative_search_the_clipboard_sibling_wears() {",
    ),
    (
        "fn the_creative_search_the_clipboard_sibling_wears() {",
        "fn the_debug_search_the_clipboard_trio_wears() {",
    ),
    (
        "fn the_os_clipboard_the_bridge_the_store_law_wears() {",
        "fn the_chest_seats_the_gather_wears() {",
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
        fmt_lines = formatted.split("\n")
        sm = difflib.SequenceMatcher(None, orig_lines, fmt_lines)
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
            out_lines[i1:i2] = fmt_lines[j1:j2]
        open(path, "w", encoding="utf-8").write("\n".join(out_lines))
        print(f"[FIXED] {path} :: {len(spans)} hunks")
    return 0


if __name__ == "__main__":
    sys.exit(main())

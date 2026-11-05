#!/usr/bin/env python3
"""autopush-469 — THE FMT REGION WALK (the 470 record's own law the
round re-wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs  — the five families' own re-wear (the creative/anvil/
                direct/pair/chat boxes the copy/cut/paste seats wear,
                the select-all sibling the range law banks).
  part_11.rs  — the round's own tests (the five sibling walks + the
                bridge test's own range-law seat the select-all ride
                wears).
Usage: python3 scripts/fmt_probe_471.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the creative family (the copy/cut/paste trio the 471 re-wear
    # wears — the click seat the region's EOF side keeps)
    (
        "THE SEARCH BOX'S OWN COPY SEAT (the 471 re-wear",
        "THE SEARCH BOX'S OWN CLICK SEAT (the EditBox.mouseClicked walk",
    ),
    # the anvil family (the field's own trio the range law wears —
    # the x-band private seat the tail keeps)
    (
        "THE ANVIL FIELD'S OWN COPY SEAT (the 471 re-wear",
        "THE ANVIL FIELD'S OWN X-BAND SEAT (the private seat compute",
    ),
    # the direct family (the field's own trio — the shift ride the
    # tail keeps)
    (
        "THE DIRECT FIELD'S OWN COPY SEAT (the 471 re-wear",
        "THE ENTRY FORM'S OWN SHIFT RIDE (the two-arg walk the FOCUSED",
    ),
    # the editserver pair (the pair's own trio — the box click the
    # tail keeps)
    (
        "THE PAIR'S OWN COPY SEAT (the 471 re-wear",
        "THE ENTRY FORM'S OWN BOX CLICK (the EditBox.mouseClicked walk",
    ),
    # the chat family (the field's own trio — the word seat the tail
    # keeps)
    (
        "THE CHAT FIELD'S OWN COPY SEAT (the 471 re-wear",
        "THE SELECTION'S OWN WORD SEAT (the vanilla double-click walk",
    ),
]

PART11 = [
    # the anvil walk's own re-bank (the chat sibling the next keeps)
    (
        "fn the_anvil_field_the_clipboard_walk_wears() {",
        "fn the_chat_field_the_clipboard_sibling_wears() {",
    ),
    # the chat sibling (the direct sibling the next keeps)
    (
        "fn the_chat_field_the_clipboard_sibling_wears() {",
        "fn the_direct_field_the_clipboard_sibling_wears() {",
    ),
    # the direct sibling (the editserver pair the next keeps)
    (
        "fn the_direct_field_the_clipboard_sibling_wears() {",
        "fn the_editserver_pair_the_clipboard_siblings_wear() {",
    ),
    # the editserver pair (the creative sibling the next keeps)
    (
        "fn the_editserver_pair_the_clipboard_siblings_wear() {",
        "fn the_creative_search_the_clipboard_sibling_wears() {",
    ),
    # the creative sibling (the 468 debug trio the next keeps)
    (
        "fn the_creative_search_the_clipboard_sibling_wears() {",
        "fn the_debug_search_the_clipboard_trio_wears() {",
    ),
    # the bridge test's own range-law seat (the chest gather the EOF
    # side keeps)
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
            inside = [
                (i1, i2, tag, j1, j2)
                for (i1, i2, tag, j1, j2) in hunks
                if i1 < e_line and i2 > s_line
            ]
            if inside:
                ok = False
                print(f"[MY-REGIONS DIRTY] {path} :: {start_marker[:60]!r}")
                for h in inside:
                    print(f"   hunk {h}")
            else:
                print(f"[MY-REGIONS CLEAN] {path} :: {start_marker[:60]!r}")
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())

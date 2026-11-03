#!/usr/bin/env python3
"""autopush-436 — THE FMT REGION WALK (the 372..autopush-417 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the field's own decls (the drag arm the tail joins),
               the char/backspace/delete/ctrl/click lanes (the click
               seat the box trio the round mints), the canvas's
               selection band + the caret line.
  window.rs  — the click chain's anvil arm (the picker the tail
               joins), the anvil drag walk, the release disarm.
  part_11.rs — the caret walk + the ctrl hop + the delete ride +
               the click seat tests (the round's own tail wears).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the field's own decls (the drag arm the tail joins)
    (
        "pub anvil_name: String,",
        "pub anvil_cost: i32,",
    ),
    # the char/backspace/delete/ctrl/click lanes (the click seat the
    # box trio the round mints)
    (
        "/// THE ANVIL FIELD'S OWN CHARS",
        "/// 26.2 BeaconBlock.useWithoutItem",
    ),
    # the canvas's selection band (the highlight walk the word block
    # wears)
    (
        "// THE FIELD'S OWN WORD",
        "// THE CARET'S OWN LINE",
    ),
    # the canvas's caret line (the insert-indicator walk the prefix
    # width serves)
    (
        "// THE CARET'S OWN LINE",
        "// THE TITLE + THE INVENTORY LABEL",
    ),
]

WINDOW = [
    # the click chain's anvil arm (the picker comment the head wears,
    # the chain's own else the tail answers)
    (
        "// THE PICKER'S OWN BOX CONSUME",
        "app.on_click(gmx, gmy);",
    ),
    # the anvil field's own drag walk (the creative lane the head
    # wears, the scroll lane the tail answers)
    (
        "// THE ANVIL FIELD'S OWN DRAG WALK",
        "// THE PICKER'S OWN SCROLL DRAG",
    ),
    # the release disarm (the anvil flag the tail joins)
    (
        "app.creative_search_drag_armed = false;",
        "was_left_down = left_down;",
    ),
]

PART11 = [
    # the caret walk + the ctrl hop + the delete ride + the click
    # seat tests (the round's own tail wears)
    (
        "/// THE ANVIL FIELD'S OWN CARET WALK",
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

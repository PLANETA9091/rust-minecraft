#!/usr/bin/env python3
"""autopush-461 — THE FMT REGION WALK (the 460 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs  — the scroll fields + the release disarm + the wheel and
                drag methods + the sprite bank's free fns + the
                render_merchant rewrite + the two drag arms (the left
                and the right press the bar band arms).
  window.rs   — the drag dispatch + the wheel arm.
  part_11.rs  — the round's own test (the sprite walk wears, EOF the
                end the round keeps).
Usage: python3 scripts/fmt_probe_462.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the scroll fields (the state block the fields join)
    (
        "THE SCROLLER'S OWN OFFSET (javap 26.2 MerchantScreen.scrollOff",
        "pub merchant_scroll_drag: bool,",
    ),
    # the release disarm (the mouseReleased lane the flag clears)
    (
        "THE MERCHANT BAR'S OWN DISARM (javap 26.2 mouseReleased",
        "THE DOUBLE-CLICK'S OWN RELEASE",
    ),
    # the wheel and drag methods (the input lanes the offsets serve)
    (
        "THE MERCHANT'S OWN WHEEL (javap 26.2 mouseScrolled",
        "THE SEARCH BOX'S OWN WINDOW SYNC",
    ),
    # the render_merchant rewrite (the sprite round the canvas wears)
    (
        "THE MERCHANT'S OWN ROUND (javap 26.2 MerchantScreen",
        "THE BEACON'S OWN HOVER-TOOLTIP SEAT",
    ),
    # the left press's drag arm (the bar band the flag arms)
    (
        "THE SCROLLER'S OWN DRAG ARM (javap mouseClicked head —",
        "THE WIDGET DISPATCH'S OWN PRECEDENCE",
    ),
    # the right press's drag arm (the button-blind walk the arm wears)
    (
        "THE SCROLLER'S OWN DRAG ARM (javap mouseClicked head — the",
        "THE IN-FLIGHT DRAG'S OWN GATE",
    ),
    # the sprite bank's free fns (the consts + the six pure walks)
    (
        "THE MERCHANT'S OWN SPRITE BANK (javap 26.2 MerchantScreen <clinit>",
        "/// THE PICKER'S PANEL ORIGIN",
    ),
]

WINDOW = [
    # the drag dispatch (the held frames the walk feeds)
    (
        "THE MERCHANT BAR'S OWN SCROLL DRAG (the mouseDragged walk's",
        "THE SENSITIVITY SLIDER'S OWN DRAG",
    ),
    # the wheel arm (the delta the offset fn serves)
    (
        "THE MERCHANT'S OWN SCROLL (the mouseScrolled walk's",
        "ScreenKind::KeyBinds {",
    ),
]

PART11 = [
    # the round's own test (the sprite walk wears — EOF the end keeps)
    (
        "THE MERCHANT ROWS' OWN SPRITES (autopush-461",
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

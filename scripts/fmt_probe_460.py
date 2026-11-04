#!/usr/bin/env python3
"""autopush-459 — THE FMT REGION WALK (the 458 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs  — the three new merchant fns (the sell re-speak + the
                fill arm + the try-move walk) + the widget press the
                client-local halves the round banks.
  world_view.rs — the four pure try-move helpers (the payment fill +
                the satisfiedBy matcher + the recipe search + the
                sell item compute) the round banks.
  part_11.rs  — the round's own test (the trade's own try-move walk
                wears, EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the three new fns (the sell re-speak + the fill arm + the walk)
    (
        "THE MERCHANT'S OWN SELL RE-SPEAK (javap 26.2 MerchantContainer",
        "/// THE BEACON'S OWN SCREEN CLICK (the autopush-322 round",
    ),
    # the widget press (the client-local halves the press serves)
    (
        "THE MERCHANT'S OWN WIDGET PRESS (javap of the TradeOfferButton",
        "/// THE MERCHANT'S OWN ROUND (javap 26.2 MerchantScreen",
    ),
]

WORLD_VIEW = [
    # the four pure helpers (the try-move's own walk the module banks)
    (
        "THE MERCHANT'S OWN PAYMENT FILL (javap 26.2 MerchantMenu.",
        "/// The quickMoveStack walk over the player's own store (javap 26.2",
    ),
]

PART11 = [
    # the round's own test (the try-move walk wears — EOF the end the
    # round keeps)
    (
        "THE TRADE'S OWN TRY-MOVE (javap 26.2 MerchantMenu.",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/world_view.rs": WORLD_VIEW,
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

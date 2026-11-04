#!/usr/bin/env python3
"""autopush-460 — THE FMT REGION WALK (the 459 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs  — the active-offer field + the close reset + the click
                walk's take dispatch + the sell re-speak's own bank +
                the three take fns (the take click + the quick-move
                source + the trade's own apply).
  world_view.rs — the offer view's xp word + the projection's own
                ride + the three pure take helpers (the sell match +
                the offer take + the take pairs).
  part_11.rs  — the walk-family's own re-pins (the result arms) +
                the try-move test's xp words + the round's own test
                (the result seat's own take wears, EOF the end the
                round keeps) + the wire test's xp word.
Usage: python3 scripts/fmt_probe_461.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the active-offer field (the state block the field joins)
    (
        "THE ACTIVE OFFER'S OWN BANK (javap 26.2 MerchantContainer",
        "THE WINDOW'S OWN TITLE (the OpenScreen word's decode",
    ),
    # the close reset (the bank's own death the field joins)
    (
        "THE MERCHANT'S OWN CLOSE (javap of `MerchantMenu.removed`",
        "THE MERCHANT'S OWN HOVER SEAT (the chest family's own shape",
    ),
    # the click walk's take dispatch + the sell re-speak's own bank +
    # the three take fns (the contiguous merchant block the round owns)
    (
        "THE MERCHANT'S OWN SLOT WALK (javap 26.2 AbstractContainerMenu.",
        "/// THE BEACON'S OWN SCREEN CLICK (the autopush-322 round",
    ),
]

WORLD_VIEW = [
    # the offer view's xp word (the struct the field joins)
    (
        "pub struct MerchantOfferView {",
        "/// The merchant window's projected bank",
    ),
    # the three pure take helpers (the sell match + the offer take +
    # the take pairs the round banks)
    (
        "THE ACTIVE OFFER'S OWN SEARCH (javap 26.2 MerchantContainer.",
        "/// The quickMoveStack walk over the player's own store (javap 26.2",
    ),
    # the projection's own ride (the xp the view wears)
    (
        ".map(|o| MerchantOfferView {",
        "level: oc.merchant.level,",
    ),
]

PART11 = [
    # the wire test's xp word (the site the field joins)
    (
        "offers: vec![crate::world_view::MerchantOfferView {",
        "level: 2,",
    ),
    # the walk-family's own re-pin (the result-seat take arms)
    (
        "THE RESULT SEAT'S OWN TAKE (the 460 round's own law",
        "THE STANDARD ROW'S OWN WRITE-THROUGH",
    ),
    # the walk-family's own re-pin (the result source's quick-move arm)
    (
        "the RESULT source the rows the quick-move serves",
        "THE GATHER'S OWN REFUSAL",
    ),
    # the try-move test's xp words (the sites the field joins)
    (
        "THE TRADE'S OWN TRY-MOVE (javap 26.2 MerchantMenu.",
        "THE RESULT SEAT'S OWN TAKE (javap 26.2 MerchantResultSlot",
    ),
    # the round's own test (the take walk wears — EOF the end keeps)
    (
        "THE RESULT SEAT'S OWN TAKE (javap 26.2 MerchantResultSlot",
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

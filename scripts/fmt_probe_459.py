#!/usr/bin/env python3
"""autopush-458 — THE FMT REGION WALK (the dev-222 record's own law
the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the merchant fields + the ctor init + the walk
                  block + the qc arm/paint + the seat-stack branch +
                  the distribution branch + the offline/wire lanes +
                  the deferred place + the press blocks + the paint
                  dispatch + the free fn the round banks.
  part_11.rs    — the round's own test (the merchant slots walk
                  family wear, EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the merchant fields (the state the walk wears)
    (
        "THE MERCHANT'S OWN DOUBLE-CLICK FLAG",
        "/// THE PIG/COW VARIANTS' OWN SKINS",
    ),
    # the ctor init (the fresh fields the None keeps)
    (
        "merchant_slots: Vec::new(),",
        "merchant_carried: (0, 0),",
    ),
    # the gather dispatch arm (the FALSE override the release wears)
    (
        "} else if self.screen == ScreenKind::Merchant {\n            // THE MERCHANT GATHER'S OWN REFUSAL",
        "} else {\n            false\n        };",
    ),
    # the release screen list (the Merchant the qc release wears)
    (
        "|| self.screen == ScreenKind::Merchant\n        {",
        "// the BEACON screen the same base walk wears",
    ),
    # the walk block (the hover/throw/click/gather the round banks)
    (
        "THE MERCHANT'S OWN HOVER SEAT",
        "/// THE BEACON'S OWN SCREEN CLICK (the autopush-322 round",
    ),
    # the qc arm + paint (the drag arms the family shape wears)
    (
        "THE MERCHANT DRAG'S OWN ARM",
        "/// THE CHEST DRAG'S OWN WIRE DISTRIBUTION",
    ),
    # the seat-stack branch (the flat mirror the paint read wears)
    (
        "if self.screen == ScreenKind::Merchant {\n            // THE MERCHANT'S OWN NUMBERING",
        "if self.screen == ScreenKind::Crafting {",
    ),
    # the distribution branch (the lanes the release keys)
    (
        "} else if self.screen == ScreenKind::Merchant {\n                // THE MERCHANT'S OWN LANES",
        "} else if self.world_is_network {",
    ),
    # the deferred place (the release POSITION the vanilla law reads)
    (
        "THE MERCHANT'S OWN DEFERRED PLACE",
        "let panel = self.inv_panel;",
    ),
    # the offline + wire lanes (the place math the fold keeps)
    (
        "THE MERCHANT'S OWN OFFLINE DISTRIBUTION",
        "/// THE FURNACE'S WIRE DISTRIBUTION",
    ),
    # the left press block (the deferral the round replaces)
    (
        "law the buttons serve FIRST — the SEVEN offer seats",
        "if self.screen == ScreenKind::Inventory {",
    ),
    # the right press block (the secondary dispatch the round banks)
    (
        "if self.screen == ScreenKind::Merchant {\n            // THE IN-FLIGHT DRAG'S OWN GATE",
        "if self.screen == ScreenKind::Creative {",
    ),
    # the paint dispatch arm (the drag's own collect the frame wears)
    (
        "} else if self.screen == ScreenKind::Merchant {\n                // THE MERCHANT'S OWN HOVER",
        "/// The keyboard lane's shift state",
    ),
    # the free fn (the wire distribution the packets mint)
    (
        "THE MERCHANT DRAG'S OWN WIRE DISTRIBUTION",
        "fn wire_brewing_quickcraft_distribute(",
    ),
]

PART11 = [
    # the round's own test (the merchant slots walk family wears —
    # EOF the end the round keeps)
    (
        "THE MERCHANT'S OWN SLOT WALK (javap 26.2 AbstractContainerMenu.",
        None,
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

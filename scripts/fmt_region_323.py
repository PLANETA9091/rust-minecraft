#!/usr/bin/env python3
"""autopush-323 — THE FMT REGION WALK (the 322 record's own law the
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

REGIONS = {
    "crates/mc-client/src/screens.rs": [
        # the left-press Beacon branch (the widget precedence + the arm)
        ("THE WIDGET DISPATCH'S OWN PRECEDENCE (the super.mouseClicked",
         "if self.screen == ScreenKind::Inventory {"),
        # the right-press Beacon branch (the gate + the precedence + the arm)
        ("THE IN-FLIGHT DRAG'S OWN GATE (mouseClicked pc 168-172",
         "if self.screen == ScreenKind::Creative {"),
        # the release gates (the left release dispatch + the right pair)
        ("|| self.screen == ScreenKind::Beacon\n        {\n            // the BEACON screen the same base walk wears (the\n            // autopush-323 round \u2014 the beacon's own lanes the",
         "if self.screen != ScreenKind::World || self.paused {"),
        ("THE DRAG'S OWN MISMATCH ABORT (mouseReleased pcs 246-284",
         "// THE EAT'S CANCEL (javap releaseUsing's own shape for the"),
        # the carried-cursor getter lane
        ("THE BEACON'S OWN LANE CURSOR (the autopush-323 round",
         "if self.world_is_network {\n            self.live_inv_carried"),
        # the free fn seat verdict
        ("THE BEACON DRAG'S OWN SEAT VERDICT (the autopush-323 round",
         "/// THE CHEST DRAG'S OWN WIRE DISTRIBUTION (the same doClick QUICK_CRAFT"),
        # the arm + paint fns
        ("THE BEACON DRAG'S OWN ARM (the autopush-323 round \u2014 the same",
         "/// THE CHEST DRAG'S OWN WIRE DISTRIBUTION"),
        ("THE BEACON DRAG'S OWN COLLECT (the same mouseDragged pcs 0-58",
         "/// THE CHEST DRAG'S OWN WIRE DISTRIBUTION"),
        # the recalc per-seat cap
        ("THE PER-SEAT CAP (the recalc's own pc 102-114 walk the",
         "remainder -= place - seat_count;"),
        # the seat-stack numbering lane
        ("THE BEACON'S OWN NUMBERING (the autopush-323 round \u2014 the",
         "if self.screen == ScreenKind::Crafting {"),
        # the distribute dispatch lane
        ("THE BEACON'S OWN LANE (the autopush-323 round \u2014 the",
         "} else if self.world_is_network {"),
        # the deferred place branch
        ("THE BEACON'S OWN DEFERRED PLACE (the autopush-323 round",
         "let panel = self.inv_panel;"),
        # the offline distribution fn
        ("THE BEACON'S OFFLINE DISTRIBUTION (the autopush-323 round",
         "/// THE FURNACE'S WIRE DISTRIBUTION (the offline walk's twin"),
        # the paint wiring arm
        ("THE BEACON'S OWN HOVER (the autopush-323 round \u2014 the",
         "}\n        }\n    }"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        # the two REAL tests (the whole appended block)
        ("THE OFFLINE BEACON DRAG'S OWN PAINT AND SPLIT (the autopush-323",
         "\n/// THE BEACON DRAG'S OWN GATES AND THE ONE-EACH"),
        ("THE BEACON DRAG'S OWN GATES AND THE ONE-EACH (the shouldAdd gates",
         "\n    assert_eq!(\n        app.beacon_carried,\n        (0, 0),\n        \"the whole carried the drop wears\"\n    );"),
    ],
}


def hunks(a, b):
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    out = []
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag != "equal":
            out.append((i1, i2))
    return out


def main():
    fail = 0
    for path, regions in REGIONS.items():
        src = open(path, encoding="utf-8").read()
        lines = src.splitlines(keepends=True)
        p = subprocess.run(
            [RUSTFMT, "--edition", "2021", "--emit", "stdout"],
            input=src,
            capture_output=True,
            text=True,
        )
        if p.returncode != 0:
            print(f"[FAIL] {path}: rustfmt error\n{p.stderr[:400]}")
            fail = 1
            continue
        fmt_lines = p.stdout.splitlines(keepends=True)
        hs = hunks(lines, fmt_lines)
        if not hs:
            print(f"[CLEAN] {path}: whole file fmt-clean")
            continue
        # marker ranges
        ranges = []
        for start_marker, end_marker in regions:
            try:
                s = next(
                    i for i, l in enumerate(lines) if start_marker.split("\n")[0] in l
                )
                e = next(
                    i
                    for i, l in enumerate(lines[s:], s)
                    if i > s and end_marker.split("\n")[0] in l
                )
            except StopIteration:
                print(f"[WARN] {path}: marker not found: {start_marker[:50]!r}")
                continue
            ranges.append((s, e, start_marker[:44]))
        for h1, h2 in hs:
            hunk_txt = "".join(lines[h1 : min(h2, h1 + 3)]).strip()[:90]
            mine = any(s <= h1 < e + 2 for s, e, _ in ranges)
            flag = "MINE" if mine else "static"
            print(f"  [{flag}] {path}:{h1 + 1} {hunk_txt!r}")
            if mine:
                fail = 1
    sys.exit(fail)


main()

#!/usr/bin/env python3
"""autopush-319 — THE FMT REGION WALK (the 318 record's own law the
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
    "crates/mc-client/src/world.rs": [
        # the kind-consts seat (the six consts + the predicates +
        # the BASE ladder the anchor block's tail wears)
        ("/// THE PYRAMID'S OWN SIX SEATS", "pub fn is_beacon_base(id: u16) -> bool {"),
        # the new kind-seat test (the 318 test's tail the next test wears)
        ("    fn the_beacon_the_kinds_the_six_seats_the_law_wears() {",
         "    fn the_chorus_the_walk_down_and_the_safe_verdicts_wear() {"),
        # the NAME_OF tail rows (the respawn_anchor row the six wear)
        ('    "respawn_anchor",', "];"),
    ],
    "crates/mc-client/src/mesher.rs": [
        # the is_transparent beacon row
        ("THE BEACON'S OWN ROW (the autopush-319 round", "|| crate::world::is_wheat(kind)"),
        # the three-element arm
        ("THE BEACON'S OWN THREE-ELEMENT LANE (the autopush-319",
         "THE END GATEWAY'S OWN INVISIBLE LANE"),
        # the helpers
        ("fn beacon_model_quads()", "/// THE BED MODELS' OWN CACHED BAKES"),
        # the arm test
        ("    fn the_beacon_the_three_elements_the_18_quads_wear() {",
         "    fn the_bed_lane_the_two_cell_template_wears() {"),
    ],
    "crates/mc-registry/src/generated/atlas.rs": [
        # the tile statics + the named consts the pin appends
        ("THE BEACON'S OWN MASTERS (the autopush-319 round", "pub static TILES"),
        ("THE BEACON'S OWN NAMED SEATS (the pool tail's four",
         "pub const TILE_NETHERITE_BLOCK: u16 = 1066;"),
    ],
    "crates/mc-client/src/screens/tests/part_7.rs": [],
    "crates/mc-client/src/screens/tests/part_8.rs": [],
}


def fmt_lines(path):
    r = subprocess.run([RUSTFMT, "--emit", "stdout", path],
                       capture_output=True, text=True)
    if r.returncode != 0:
        print(f"FMT-ERR {path}: {r.stderr[:400]}")
        sys.exit(2)
    return r.stdout.split("\n")


def marker_ranges(lines, marks, path):
    out = []
    n = len(lines)
    for start, end in marks:
        s = None
        for i, l in enumerate(lines):
            if start in l:
                s = i
                break
        if s is None:
            print(f"MISSING START MARKER {path}:{start!r}")
            sys.exit(2)
        if end == "@@EOF@@":
            out.append((s, n))
            continue
        e = None
        for i in range(s + 1, n):
            if end in lines[i]:
                e = i
                break
        if e is None:
            print(f"MISSING END MARKER {path}:{end!r}")
            sys.exit(2)
        out.append((s, e + 1))
    return out


def main():
    bad = 0
    for path, marks in REGIONS.items():
        orig = open(path).read().split("\n")
        fmt = fmt_lines(path)
        sm = difflib.SequenceMatcher(None, orig, fmt, autojunk=False)
        regions = marker_ranges(orig, marks, path)
        hunks = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag != "equal":
                hunks.append((i1, i2))
        if not regions:
            # the no-marker files: the MY-EDIT verdict the diff's own
            # silence keeps (the sweep's numeric edits the formatted
            # text never reorders — the hunk inventory the honest
            # answer prints)
            dirty = hunks
            if dirty:
                print(f"NOTICE {path}: {len(dirty)} hunk(s) the fmt speaks")
                for h in dirty[:2]:
                    print(f"    hunk [{h[0]}:{h[1]}]")
            else:
                print(f"CLEAN {path} (no hunks)")
            continue
        for (s, e) in regions:
            overlap = [h for h in hunks if h[0] < e and h[1] > s]
            if overlap:
                bad += 1
                print(f"DIRTY {path} [{s}:{e}] hunks {overlap}")
                for h in overlap[:3]:
                    for ln in orig[max(0, h[0]):h[1]][:6]:
                        print(f"    | {ln}")
            else:
                print(f"CLEAN {path} [{s}:{e}]")
    print(f"== {'FAIL' if bad else 'ALL REGIONS CLEAN'} ({bad} dirty) ==")
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()

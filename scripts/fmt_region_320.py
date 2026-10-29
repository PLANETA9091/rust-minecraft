#!/usr/bin/env python3
"""autopush-320 — THE FMT REGION WALK (the 319 record's own law the
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
        # the budgeted ladder core (the wrapper + the budget the
        # render walk wears)
        ("pub fn beacon_beam_sections(", "/// THE PYRAMID'S OWN LEVELS SCAN"),
        # the beam law layer (the animation + the radius + the frac +
        # the factors + the prisms + the segments walk)
        ("THE BEAM'S OWN ANIMATION TIME", "/// THE CHORUS WALK'S OWN SAFE SEAT"),
        # the beam law test (the 318 test's tail the next test wears)
        ("    fn the_beacon_the_beam_the_segments_the_render_laws_wear() {",
         "    fn the_beacon_the_kinds_the_six_seats_the_law_wears() {"),
    ],
    "crates/mc-client/src/screens.rs": [
        # the two fields (the texture + the cell bank)
        ("THE BEACON BEAM'S OWN TEXTURE", "THE PIG/COW VARIANTS' OWN SKINS"),
        # the None inits (the gateway row the beacon pair wears)
        ("gateway_beam_tex: None,", "chicken_warm_model: None,"),
        # the bank fns (the walk + the parse)
        ("THE BEACON BANK'S OWN WALK", "THE BED SPAWN'S OWN BANK"),
        # the boot read (the extend law the gateway parse rides)
        ("self.beacon_stores.extend(beacon_cells_from_nbt(&root));",
         "THE PROGRESS WORDS' OWN PARSE"),
        # the save row (the beacons list the file wears)
        ("THE BEACON BANK'S OWN ROW", '"furnace_words".to_string(),'),
        # the call site (the gateway call the beacon pass rides)
        ("self.render_gateway_beams(fb, &ctx);", "THE DROP LANE's entities"),
        # the render pass fn
        ("fn render_beacon_beams(&mut self", "THE PAIR'S OWN LOADER"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        # the REAL walk test (the file tail the append wears)
        ("fn the_offline_beacon_the_beam_the_pyramid_the_sections_wear() {",
         "@@EOF@@"),
    ],
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
        dirty = [
            (i1, i2)
            for (i1, i2) in hunks
            if any(i1 < e and s < i2 for (s, e) in regions)
        ]
        if dirty:
            bad += 1
            print(f"DIRTY {path}: {len(dirty)} hunk(s) touch my regions")
            for h in dirty[:4]:
                print(f"    hunk [{h[0]}:{h[1]}]")
        else:
            noise = len(hunks)
            print(f"CLEAN {path} (my regions {len(regions)}, non-mine hunks {noise})")
    if bad:
        print(f"FMT REGION VERDICT: {bad} file(s) dirty")
        sys.exit(1)
    print("FMT REGION VERDICT: CLEAN")
    return 0


if __name__ == "__main__":
    sys.exit(main())

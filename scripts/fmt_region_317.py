#!/usr/bin/env python3
"""autopush-317 — THE FMT REGION WALK (the 316 record's own law the
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
        ("/// THE END PORTAL CUBE'S OWN FACE GATE (javap",
         "/// THE GATEWAY PUFF'S OWN WORDS"),
        ("    fn the_gateway_the_beam_the_starfield_the_laws_wear() {",
         "    fn the_chorus_the_walk_down_and_the_safe_verdicts_wear() {"),
    ],
    "crates/mc-client/src/mesher.rs": [
        ("/// THE OCCLUSION VOCABULARY (the mesh walk's own non-occluding",
         "    kind == AIR"),
        ("// THE CUBE'S OWN STATIC-LAW FACES (the autopush-317",
         "// THE BED'S OWN TWO-CELL LANE"),
        ("// THE END PORTAL'S OWN ARM (the dev-29 round",
         "// THE GLASS FAMILY'S OWN CULL ARM"),
    ],
    "crates/mc-client/src/screens.rs": [
        ("crate::world::END_GATEWAY => {",
         "_ => {}"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        ("// THE CUBE'S OWN BAKE (javap the facesToShow walk the mesher's",
         "// THE AGE CLOCK'S OWN FIRST TICK"),
    ],
    "crates/mc-client/src/screens/tests/part_9.rs": [
        ("/// THE END PORTAL'S OWN MESH (the dev-29 round",
         "// THE SHADER PASS'S OWN PAIR"),
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

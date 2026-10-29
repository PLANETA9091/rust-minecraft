#!/usr/bin/env python3
"""autopush-321 — THE FMT REGION WALK (the 320 record's own law the
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
        # the cell bank's own field doc (the 321 round's update wears)
        ("THE BEACON CELLS' OWN BANK", "THE PIG/COW VARIANTS' OWN SKINS"),
        # the tick-site store drop (the dig completion's own walk)
        ("THE BEACON'S OWN STORE DROP (the autopush-321",
         "if beat {"),
        # the instabreak store drop (the press site's own walk)
        ("// — the instabuild press the same store walk the",
         "// THE UNCONDITIONAL SWING"),
        # the blast pre-clear scan + the retirement (the crater's own walk)
        ("THE BEACON'S OWN PRE-CLEAR SCAN",
         "self.remesh_offline();"),
        # the placement seat (the block lane's own tail)
        ("THE BEACON'S OWN PLACEMENT SEAT",
         "let name = crate::world::NAME_OF[block as usize];"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        # the REAL walk test (the file tail the append wears)
        ("fn the_offline_beacon_the_placement_walk_the_store_feeds_wear() {",
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

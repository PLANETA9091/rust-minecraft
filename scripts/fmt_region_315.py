#!/usr/bin/env python3
"""autopush-315 — THE FMT REGION WALK (the 314 record's own law the
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

# (path, [(start_marker, end_marker_inclusive_start_of_next), ...])
# — the region = [start line .. the line BEFORE end_marker's first hit
# after start); the end marker may be @@EOF@@ (tail region).
REGIONS = {
    "crates/mc-client/src/world.rs": [
        ("/// THE ANCHOR'S OWN CHARGE-LADDER MESH FOLD",
         "/// THE CHARGE GATE'S OWN FOLD"),
        ("    fn the_anchor_the_mesh_ladder_the_tiles_wear() {",
         "    fn the_shulker_cell_the_lids_tick_the_same_ramp_wears() {"),
    ],
    "crates/mc-client/src/mesher.rs": [
        ("} else if crate::world::is_respawn_anchor(id) {",
         "} else if crate::world::is_dispenser_kind(id) {"),
    ],
    "crates/mc-client/src/light.rs": [
        ("fn emission(id: u16, variant: u8) -> u8 {",
         "@@EOF@@"),
    ],
    "crates/mc-client/src/screens.rs": [
        ("// THE GLOW'S OWN RELIGHT",
         "// THE VOICE (charge pcs 82-90"),
        ("// THE CONSUME'S OWN WALK (the charge-1 setBlock the",
         "if seat.is_none() {"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        ("// THE ANCHOR'S OWN MESH-LADDER WALK",
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


def marker_ranges(lines, marks):
    out = []
    for start, end in marks:
        si = next((i for i, ln in enumerate(lines) if start in ln), None)
        if si is None:
            print(f"MISS start {start[:50]!r}")
            sys.exit(2)
        if end == "@@EOF@@":
            ei = len(lines)
        else:
            ei = next((i for i, ln in enumerate(lines) if end in ln and i > si), None)
            if ei is None:
                print(f"MISS end {end[:50]!r}")
                sys.exit(2)
        out.append((si, ei, start))
    return out


def main():
    bad = 0
    for path, marks in REGIONS.items():
        orig = open(path).read().split("\n")
        fmt = fmt_lines(path)
        sm = difflib.SequenceMatcher(None, orig, fmt, autojunk=False)
        ranges = marker_ranges(orig, marks)
        touched = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == "equal":
                continue
            for si, ei, label in ranges:
                if i1 < ei and i2 > si:  # hunk overlaps the region
                    touched.append((label[:44], tag, i1 + 1, i2))
        if touched:
            bad += len(touched)
            for t in touched:
                print(f"DIRTY {path.rsplit('/',1)[-1]} :: {t}")
        else:
            names = ",".join(l[0][4:36].strip() for _, _, l in ranges)
            print(f"CLEAN {path.rsplit('/',1)[-1]} :: {names}")
    print(f"[fmt-region-315] {'CLEAN' if bad == 0 else f'{bad} DIRTY'}")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())

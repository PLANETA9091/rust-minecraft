#!/usr/bin/env python3
"""autopush-316 — THE FMT REGION WALK (the 315 record's own law the
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
        ("/// THE GATEWAY'S OWN BEAM STAR LANE (the autopush-316 round",
         "/// THE CHORUS WALK'S OWN SAFE SEAT"),
        ("    fn the_gateway_the_beam_the_starfield_the_laws_wear() {",
         "    fn the_chorus_the_walk_down_and_the_safe_verdicts_wear() {"),
    ],
    "crates/mc-client/src/mesher.rs": [
        ("pub const TILE_END_GATEWAY: u16 = 65533;",
         "/// THE HEAD'S OWN MODEL-FACE TABLE"),
        ("// THE CUBE'S OWN Y-FACES (the autopush-316 round",
         "// THE BED'S OWN TWO-CELL LANE"),
    ],
    "crates/mc-client/src/screens.rs": [
        ("/// THE END GATEWAY'S OWN STORE WORDS (the autopush-310 round",
         "/// THE BED SPAWN'S OWN BANK"),
        ("// THE GATEWAY CLOCK'S OWN DECAY (the autopush-316 round",
         "if !self.world_is_network && !self.drops.is_empty() {"),
        ("// THE GATEWAY'S OWN PUFFS (the autopush-316 round",
         "let animate = &mut self.ambient_rng;"),
        ("crate::world::END_GATEWAY => {",
         "_ => {}"),
        ("for puff in gateway_puffs {",
         "match self.screen {"),
        ("pub gateway_beam_tex: Option<(u32, u32, Vec<u8>)>,",
         "/// THE PIG/COW VARIANTS' OWN SKINS"),
        ("// THE GATEWAY'S OWN CUBE PASS (the autopush-316 round",
         "// THE DROP LANE's entities"),
        ("/// THE GATEWAY BEAM'S OWN RENDER (the autopush-316 round",
         "/// THE PAIR'S OWN LOADER"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        ("/// THE GATEWAY'S OWN BEAM STAR LANE (the autopush-316 round",
         "/// THE BED'S OWN PLACE WALK"),
    ],
    "crates/mc-protocol/src/nbt.rs": [
        ("/// Long view (the BE clocks like the gateway's own `Age` word",
         "/// Float view (the timeline track values"),
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
        out.append((s, e))
    return out


def main():
    bad = 0
    for path, marks in REGIONS.items():
        orig = open(path).read().split("\n")
        fmt = fmt_lines(path)
        ranges = marker_ranges(orig, marks)
        sm = difflib.SequenceMatcher(None, orig, fmt)
        hunks = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == "equal":
                continue
            hunks.append((i1, i2))
        clean = []
        dirty = []
        for (rs, re_) in ranges:
            hit = [(h1, h2) for (h1, h2) in hunks
                   if h1 < re_ and h2 > rs]
            if hit:
                dirty.append((rs, re_, hit))
                bad += 1
            else:
                clean.append((rs, re_))
        print(f"{path}: regions={len(ranges)} clean={len(clean)} dirty={len(dirty)}")
        for rs, re_, hit in dirty:
            print(f"  DIRTY [{rs}..{re_}) hunks={hit}")
            for h1, h2 in hit[:2]:
                for l in orig[max(0, h1 - 1):h2 + 1][:12]:
                    print(f"    | {l}")
    if bad:
        print(f"FMT REGION VERDICT: {bad} dirty region(s)")
        sys.exit(1)
    print("FMT REGION VERDICT: all CLEAN")


if __name__ == "__main__":
    main()

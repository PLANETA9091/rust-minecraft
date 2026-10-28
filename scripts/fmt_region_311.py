#!/usr/bin/env python3
"""autopush-311 — THE FMT REGION WALK (the 310 record's own law the
round wears: the parseable fragments the standalone rustfmt the CLEAN
rc=0 verdict banks — the whole-file rustfmt the pre-existing static
noise the HEAD walk confirms).

Exit 0 iff every fragment fmt-checks clean.
"""
import subprocess
import sys
import tempfile
import os

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

# (path, start_marker inclusive, end_marker EXCLUSIVE, wrapper)
FRAGMENTS = [
    # THE BED FACING'S OWN LAWS (the world.rs block the round wrote —
    # the look decode + the offsets)
    ("crates/mc-client/src/world.rs",
     "/// THE BED FACING'S OWN LOOK DECODE (javap BedBlock.getStateForPlacement",
     "/// THE END GATEWAY'S OWN KIND SEAT",
     "top"),
    # THE BED TEST (the world.rs unit)
    ("crates/mc-client/src/world.rs",
     "    #[test]\n    fn the_bed_place_walk_the_look_decode_the_offsets_wear() {",
     "    #[test]\n    fn the_shulker_cell_the_lids_tick_the_same_ramp_wears() {",
     "mod"),
    # THE PLACEMENT GATE (the screens.rs two-cell gate)
    ("crates/mc-client/src/screens.rs",
     "        // THE BED'S OWN TWO-CELL GATE (javap BedBlock.getStateForPlacement",
     "        // THE WALL TORCH'S OWN PLACEMENT CANDIDATE",
     "stmt8"),
    # THE TWO-HALVES SET (the screens.rs placement walk's own arm)
    ("crates/mc-client/src/screens.rs",
     "            // THE BED'S OWN TWO HALVES (javap BedBlock.setPlacedBy the",
     "            // THE DOOR'S OWN WIRE PRESET",
     "stmt12"),
    # THE MATE SCAN (the dig_break_effects' own walk)
    ("crates/mc-client/src/screens.rs",
     "        // THE BED'S OWN MATE LAW (javap BedBlock.updateShape +",
     "        // THE BLOCK_DESTROY EVENT'S OWN POST",
     "stmt8"),
    # THE HEAD-ONLY DROP (the spawn_drops_for's own arm)
    ("crates/mc-client/src/screens.rs",
     "        // THE BED'S OWN HEAD-ONLY DROP (javap the jar's loot —",
     "        for (item, count) in crate::drops::roll_drops(kind, held, &mut self.drop_rng) {",
     "stmt8"),
    # THE THREE PART_11 REAL WALKS
    ("crates/mc-client/src/screens/tests/part_11.rs",
     "/// THE BED'S OWN PLACE WALK (the autopush-311 round — javap",
     "@@EOF@@",
     "top-tail"),
]


def rustfmt_file(name, check):
    cmd = [RUSTFMT] + (["--check"] if check else []) + [name]
    return subprocess.run(cmd, capture_output=True, text=True)


def dedent(text, base):
    out = []
    for ln in text.split("\n"):
        out.append(ln[base:] if ln.startswith(" " * base) else ln.lstrip())
    while out and out[-1].strip() == "":
        out.pop()
    return "\n".join(out)


def reindent(text, pads):
    pre = "    " * pads
    return "\n".join((pre + ln) if ln.strip() else ln for ln in text.split("\n"))


def wrap(kind, frag):
    if kind == "top" or kind == "top-tail":
        return frag
    if kind == "mod":
        return "mod t {\n" + frag + "}\n"
    if kind == "stmt16":
        return "fn f() {\n" + reindent(dedent(frag, 16), 1) + "\n}\n"
    if kind == "stmt12":
        return "fn f() {\n" + reindent(dedent(frag, 12), 1) + "\n}\n"
    if kind == "stmt8":
        return "fn f() {\n" + reindent(dedent(frag, 8), 1) + "\n}\n"
    sys.exit(f"unknown wrapper {kind}")


def main():
    bad = 0
    for path, start, end, wrapper in FRAGMENTS:
        text = open(path).read()
        i = text.find(start)
        if i < 0:
            print(f"MISS  {path} :: start not found: {start[:50]!r}")
            bad += 1
            continue
        j = text.find(end, i + len(start)) if end != "@@EOF@@" else len(text)
        if end != "@@EOF@@" and j < 0:
            print(f"MISS  {path} :: end not found: {end[:50]!r}")
            bad += 1
            continue
        frag = text[i:j].rstrip() + "\n"
        frag = wrap(wrapper, frag)
        with tempfile.NamedTemporaryFile("w", suffix=".rs", delete=False) as f:
            f.write(frag)
            name = f.name
        r = rustfmt_file(name, check=True)
        label = f"{path.rsplit('/', 1)[-1]} :: {start[:44]!r}"
        if r.returncode == 0:
            print(f"CLEAN {label}")
        else:
            print(f"DIRTY {label}\n{r.stdout[:1200]}{r.stderr[:600]}")
            bad += 1
        os.unlink(name)
    print(f"[fmt-region] {'CLEAN' if bad == 0 else f'{bad} DIRTY'}")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())

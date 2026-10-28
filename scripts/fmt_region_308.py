#!/usr/bin/env python3
"""autopush-308 — THE FMT REGION WALK (the 307 record's own law the
round wears: the parseable fragments the standalone rustfmt the CLEAN
rc=0 verdict banks — the whole-file rustfmt the pre-existing static
noise the HEAD walk confirms).

The trailing blanks the extraction's own boundary the strip folds —
the in-region shape the rustfmt verdict reads. The dedent fragment
the format-and-compare the wrapper's own indent the artifact answers.

Exit 0 iff every fragment fmt-checks clean.
"""
import subprocess
import sys
import tempfile
import os

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

# (path, start_marker inclusive, end_marker EXCLUSIVE, wrapper)
FRAGMENTS = [
    # THE FURNACE FEED HELPERS (the world.rs block the round wrote)
    ("crates/mc-client/src/world.rs",
     "/// THE FURNACE SLOT'S OWN SEATS",
     "/// THE FAMILY'S OWN VARIANT ENCODE",
     "top"),
    # THE TWO HOPPER ARMS (the screens.rs push + suck furnace lanes)
    ("crates/mc-client/src/screens.rs",
     "        } else if crate::world::is_orientable_furnace(kind_at(attached)) {",
     "        // THE SUCK LAW (javap tryMoveItems pc 50-67 the EAGER `ior`",
     "dedent-fn"),
    ("crates/mc-client/src/screens.rs",
     "            } else if crate::world::is_orientable_furnace(kind_at(above)) {",
     "        // THE NEXT TICK'S OWN ARM",
     "dedent-fn"),
    # THE THREE WORLD TESTS (the furnace feed trio)
    ("crates/mc-client/src/world.rs",
     "    #[test]\n    fn the_hopper_furnace_feed_the_face_tables_wear() {",
     "    #[test]\n    fn the_shulker_cell_the_lids_tick_the_same_ramp_wears() {",
     "mod"),
    # THE TWO PART_11 REAL WALKS (the side lane + the pipeline)
    ("crates/mc-client/src/screens/tests/part_11.rs",
     "/// THE HOPPER'S OWN FURNACE FEED THE SIDE LANE",
     "@@EOF@@",
     "top-tail"),
]


def rustfmt_file(name, check):
    cmd = [RUSTFMT] + (["--check"] if check else []) + [name]
    return subprocess.run(cmd, capture_output=True, text=True)


def dedent_fn(text):
    lines = text.split("\n")
    out = []
    for ln in lines:
        if ln.startswith("        } else if") or ln.startswith("            } else if"):
            out.append(ln[8:])
        else:
            out.append(ln[12:] if ln.startswith("            ") else ln)
    return "\n".join(out)


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
        if wrapper == "dedent-fn":
            frag = dedent_fn(frag)
        elif wrapper == "mod":
            frag = "mod t {\n" + frag + "}\n"
        elif wrapper == "impl":
            frag = "struct S;\nimpl S {\n" + frag + "}\n"
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

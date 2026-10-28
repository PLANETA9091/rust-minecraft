#!/usr/bin/env python3
"""autopush-309 — THE FMT REGION WALK (the 308 record's own law the
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
    # THE BREWING LAWS (the world.rs block the round wrote — the kind
    # seat + the face tables + the gates + the move/push/suck arms)
    ("crates/mc-client/src/world.rs",
     "/// Kind 780 (the seven-hundred-eighty-first entry, the autopush-309",
     "/// THE FAMILY'S OWN VARIANT ENCODE",
     "top"),
    # THE THREE WORLD TESTS (the brewing trio)
    ("crates/mc-client/src/world.rs",
     "    #[test]\n    fn the_brewing_stand_the_kind_seat_and_the_face_tables_wear() {",
     "    #[test]\n    fn the_shulker_cell_the_lids_tick_the_same_ramp_wears() {",
     "mod"),
    # THE BREWING FIELD (the screens.rs bank declaration — the struct
    # wrapper the field item the parse wears)
    ("crates/mc-client/src/screens.rs",
     "    /// THE BREWING STANDS' OWN BANK (the autopush-309 round — the",
     "    /// THE OPEN FURNACE'S OWN CELL",
     "struct"),
    # THE BREWING PARSE (the screens.rs from-nbt fn)
    ("crates/mc-client/src/screens.rs",
     "/// THE BREWING BANK'S OWN PARSE (the autopush-309 round — the",
     "/// THE FURNACE WORDS' OWN BANK",
     "top"),
    # THE FILL BRANCH (the screens.rs comparator five — the if-head
    # synthesis the parse wears)
    ("crates/mc-client/src/screens.rs",
     "        if crate::world::is_brewing_stand(kind) {",
     "        if crate::world::is_dispenser_kind(kind) {",
     "if-branch"),
    # THE TWO HOPPER ARMS (the screens.rs push + suck brewing lanes —
    # the if-head synthesis the else-if shape the parse wears)
    ("crates/mc-client/src/screens.rs",
     "        } else if crate::world::is_brewing_stand(kind_at(attached)) {",
     "        // THE SUCK LAW (javap tryMoveItems pc 50-67 the EAGER `ior`",
     "if-branch"),
    ("crates/mc-client/src/screens.rs",
     "            } else if crate::world::is_brewing_stand(kind_at(above)) {",
     "            } else if crate::world::collision_shape(kind_at(above))",
     "if-branch"),
    # THE BREWING SPILL (the screens.rs removal law)
    ("crates/mc-client/src/screens.rs",
     "    /// THE BREWING STAND'S OWN SPILL (the autopush-309 round — the",
     "    /// THE TABLE'S OWN CLOSE",
     "impl"),
    # THE TWO PART_11 REAL WALKS (the side fuel lane + the below potion lane)
    ("crates/mc-client/src/screens/tests/part_11.rs",
     "/// THE HOPPER'S OWN BREWING SIDE THE FUEL LANE THE COOLDOWN WEARS",
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


def if_branch(text):
    """The else-if head's own synthesis (the `} else if COND {` head
    the `if COND {` form the parse wears — the head's own indent the
    strip base wears, the +4 the fn wrapper's own if depth re-folds,
    the exclusive-cut close the brace balance the wrap re-folds [the
    else-if arm's own `}` the NEXT head's line the chain wears — the
    cut the close never carries])."""
    lines = text.rstrip("\n").split("\n")
    base = len(lines[0]) - len(lines[0].lstrip())
    out = []
    for ln in lines:
        s = ln[base:] if ln.startswith(" " * base) else ln.lstrip()
        stripped = s.strip()
        if stripped.startswith("} else if "):
            s = "if " + stripped[len("} else if "):]
        elif stripped.startswith("if ") and not s.startswith(" "):
            s = stripped  # the base-cut head the no-op wear keeps
        if s.strip():
            s = "    " + s
        out.append(s)
    while out and out[-1].strip() == "":
        out.pop()
    body = "\n".join(out)
    opens = body.count("{")
    closes = body.count("}")
    for i in range(max(0, opens - closes)):
        body += "\n" + "    " * (i + 1) + "}"
    return "fn f() {\n" + body + "\n}\n"


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
        elif wrapper == "if-branch":
            frag = if_branch(frag)
        elif wrapper == "mod":
            frag = "mod t {\n" + frag + "}\n"
        elif wrapper == "struct":
            frag = "struct S {\n" + frag + "}\n"
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

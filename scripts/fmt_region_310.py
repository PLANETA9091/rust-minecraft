#!/usr/bin/env python3
"""autopush-310 — THE FMT REGION WALK (the 309 record's own law the
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
    # THE GATEWAY + CHORUS LAWS (the world.rs block the round wrote —
    # the kind seat + the constants + the exit resolution + the walk)
    ("crates/mc-client/src/world.rs",
     "/// THE END GATEWAY'S OWN KIND SEAT (the KIND_NAMES tail's row the",
     "/// THE BREWING SLOT'S OWN SEATS",
     "top"),
    # THE COLLISION ROW (the gateway's own Empty seat — the chain
    # fragment the let-expr wrapper parses)
    ("crates/mc-client/src/world.rs",
     "        // THE END GATEWAY'S OWN ROW",
     "        // THE SAPLING'S OWN ROW",
     "expr8"),
    # THE THREE WORLD TESTS (the gateway trio)
    ("crates/mc-client/src/world.rs",
     "    #[test]\n    fn the_end_gateway_the_kind_seat_and_the_empty_shape_wear() {",
     "    #[test]\n    fn the_shulker_cell_the_lids_tick_the_same_ramp_wears() {",
     "mod"),
    # THE GATEWAY STORE WORDS (the screens.rs struct + the bank pair)
    ("crates/mc-client/src/screens.rs",
     "/// THE END GATEWAY'S OWN STORE WORDS (the autopush-310 round —",
     "/// THE BREWING BANK'S OWN PARSE",
     "top"),
    # THE GATEWAY FIELD (the screens.rs bank declaration)
    ("crates/mc-client/src/screens.rs",
     "    /// THE END GATEWAYS' OWN BANK (the autopush-310 round — the same",
     "    /// THE OPEN FURNACE'S OWN CELL",
     "struct"),
    # THE DROP LOOP'S OWN ARM (the decay walk + the gateway lane —
    # the stmt block the fn wrapper parses)
    ("crates/mc-client/src/screens.rs",
     "                // THE GATEWAY CLOCK'S OWN DECAY (javap",
     "                crate::drops::merge_pass(&mut self.drops);",
     "stmt16"),
    # THE EAT ARM (the chorus teleport walk — the match arm the match
    # wrapper parses)
    ("crates/mc-client/src/screens.rs",
     "                        // THE CHORUS TELEPORT'S OWN WALK (the",
     "                        // the other kinds' client surfaces",
     "arm24"),
    # THE MESHER'S OWN LANE (the INVISIBLE early retire)
    ("crates/mc-client/src/mesher.rs",
     "                // THE END GATEWAY'S OWN INVISIBLE LANE (the autopush-310",
     "                // THE BED'S OWN TWO-CELL LANE",
     "stmt16"),
    # THE THREE PART_11 REAL WALKS (the chorus eat + the two gateway walks)
    ("crates/mc-client/src/screens/tests/part_11.rs",
     "/// THE CHORUS EAT'S OWN TELEPORT WALK (the autopush-310 round —",
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
    if kind == "struct":
        return "struct S {\n" + frag + "}\n"
    if kind == "stmt16":
        return "fn f() {\n" + reindent(dedent(frag, 16), 1) + "\n}\n"
    if kind == "arm24":
        return (
            "fn f() {\n    match () {\n"
            + reindent(dedent(frag, 24), 2)
            + "\n    }\n}\n"
        )
    if kind == "expr8":
        body = reindent(dedent(frag, 8), 1)
        body = body.rstrip() + ";"
        return "fn f() {\n    let _v = true\n" + body + "\n}\n"
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

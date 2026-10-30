#!/usr/bin/env python3
"""fmt probe for the autopush-342 round's own regions (the harness law —
the ENCLOSING UNIT extraction: the complete fns / structs / tests the
self-contained syntax keeps; the brace-less const the doc head + the
ONE semicolon line wears; the mod-nested units the dedent the probe
serves). Usage: python3 scripts/fmt_probe_342.py [-v]"""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = os.path.join(HERE, "_probe_342.rs")

verbose = "-v" in sys.argv


def lines_of(path):
    return open(path).read().split("\n")


def unit(lines, marker):
    """The complete unit: the contiguous /// doc head + the body to the
    balanced closing brace (or the ONE semicolon line the brace-less
    const wears) at the unit's own indent."""
    f = next(i for i, l in enumerate(lines) if marker in l)
    start = f
    while start > 0 and lines[start - 1].lstrip().startswith(("///", "//")):
        start -= 1
    indent = len(lines[f]) - len(lines[f].lstrip())
    if lines[f].rstrip().endswith(";"):
        return start, f + 1, indent
    end = f
    depth = 0
    seen_open = False
    while end < len(lines):
        ln = lines[end]
        if not seen_open and "{" in ln:
            seen_open = True
        depth += ln.count("{") - ln.count("}")
        if seen_open and depth == 0 and ln.rstrip().endswith("}"):
            return start, end + 1, indent
        end += 1
    raise SystemExit(f"unbalanced unit: {marker}")


def run_check(name, body_lines, indent):
    stripped = [ln[indent:] if len(ln) > indent else "" for ln in body_lines]
    with open(SCRATCH, "w") as f:
        f.write("#![allow(dead_code, unused_variables, unused_mut)]\n")
        f.write("\n".join(stripped) + "\n")
    r = subprocess.run(
        [RUSTFMT, "--edition", "2021", "--check", SCRATCH],
        capture_output=True, text=True,
    )
    ok = r.returncode == 0
    print(f"[{'CLEAN' if ok else 'DIRTY'}] {name}")
    if not ok and verbose:
        print(r.stdout[:5000])
    if not ok and r.stderr.strip() and not r.stdout.strip():
        print("  STDERR:", r.stderr[:300])
    return ok


ok = True
SRC = "crates/mc-protocol/src/play.rs"
L = lines_of(SRC)

for name, marker in [
    ("const POTION_CONTENTS", "pub const POTION_CONTENTS: i32 = 51;"),
    ("struct MobEffectInstanceData", "pub struct MobEffectInstanceData {"),
    ("struct PotionContents", "pub struct PotionContents {"),
    ("fn read_potion_contents", "fn read_potion_contents(c: &mut &[u8])"),
    ("fn read_mob_effect_instance", "fn read_mob_effect_instance(c: &mut &[u8]"),
    ("fn test the_potion_contents", "fn the_potion_contents_component_rides_the_optional_lanes("),
    ("enum ComponentValue", "pub enum ComponentValue {"),
]:
    s, e, indent = unit(L, marker)
    ok &= run_check(name, L[s:e], indent)

print("ALL-CLEAN" if ok else "DIRTY-FOUND")
sys.exit(0 if ok else 1)

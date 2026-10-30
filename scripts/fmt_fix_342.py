#!/usr/bin/env python3
"""fmt fix for the autopush-342 round's own regions (the splice law —
the rustfmt re-lay the dirty units wear, the region law the probe
pairs). Usage: python3 scripts/fmt_fix_342.py"""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = os.path.join(HERE, "_probe_342.rs")


def lines_of(path):
    return open(path).read().split("\n")


def unit(lines, marker):
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


def rustfmt_dedented(body_lines, indent):
    stripped = [ln[indent:] if len(ln) > indent else "" for ln in body_lines]
    with open(SCRATCH, "w") as f:
        f.write("\n".join(stripped) + "\n")
    r = subprocess.run([RUSTFMT, "--edition", "2021", SCRATCH], capture_output=True, text=True)
    if r.returncode != 0:
        raise SystemExit(f"rustfmt failed: {r.stderr[:500]}")
    return open(SCRATCH).read().rstrip("\n").split("\n")


SRC = "crates/mc-protocol/src/play.rs"
for marker in [
    "fn the_potion_contents_component_rides_the_optional_lanes(",
]:
    L = lines_of(SRC)
    s, e, indent = unit(L, marker)
    fixed = rustfmt_dedented(L[s:e], indent)
    reindented = [("    " + ln) if ln else ln for ln in fixed]
    L[s:e] = reindented
    open(SRC, "w").write("\n".join(L))
    print(f"spliced {marker} ({e - s} -> {len(reindented)} lines)")

print("FIX-APPLIED")

#!/usr/bin/env python3
"""fmt fix for the autopush-468 round's own regions (the splice law —
the rustfmt re-lay the touched units wear, the sibling hunks the
pre-existing drift the untouched code keeps, the wholesale fmt the 478
law refuses). Usage: python3 scripts/fmt_fix_468.py"""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = os.path.join(HERE, "_probe_468.rs")


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
    r = subprocess.run(
        [RUSTFMT, "--edition", "2021", SCRATCH], capture_output=True, text=True
    )
    if r.returncode != 0:
        raise SystemExit(f"rustfmt failed: {r.stderr[:500]}")
    return open(SCRATCH).read().rstrip("\n").split("\n")


JOBS = [
    (
        "crates/mc-client/src/mob.rs",
        "pub fn is_fence_block",
    ),
    (
        "crates/mc-client/src/screens.rs",
        "struct LeashKnotEnt",
    ),
    (
        "crates/mc-client/src/screens.rs",
        "fn pick_knot",
    ),
    (
        "crates/mc-client/src/screens.rs",
        "fn bind_player_mobs_to_fence",
    ),
    (
        "crates/mc-client/src/screens.rs",
        "fn on_right_press_knot_interact",
    ),
    (
        "crates/mc-client/src/screens.rs",
        "fn tick_knots",
    ),
    (
        "crates/mc-client/src/screens.rs",
        "fn render_knots",
    ),
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "fn the_lead_knot_the_vanilla_words_wear",
    ),
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "fn the_lead_knot_the_fence_tie_wears",
    ),
]

changed = 0
for path, marker in JOBS:
    lines = lines_of(path)
    s, e, ind = unit(lines, marker)
    body = lines[s:e]
    formatted = rustfmt_dedented(body, ind)
    reind = [(" " * ind + ln) if ln else ln for ln in formatted]
    # the doc-comment indentation the unit() walk keeps — the leading
    # /// lines the dedent stripped the same re-indent wears
    if reind != body:
        lines[s:e] = reind
        with open(path, "w") as f:
            f.write("\n".join(lines))
        print(f"{path}: {marker} — RESPLICED ({e - s} lines)")
        changed += 1
    else:
        print(f"{path}: {marker} — zero drift ({e - s} lines)")

if os.path.exists(SCRATCH):
    os.remove(SCRATCH)
print(f"changed={changed}")

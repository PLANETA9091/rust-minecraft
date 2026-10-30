#!/usr/bin/env python3
"""fmt fix for the autopush-340 round's dirty regions — the splice law:
extract the unit, run rustfmt on the scratch (the SAME shape the probe
checks), splice the formatted body back. The whole-file lane (brewing_sim)
rustfmt serves directly. Usage: python3 scripts/fmt_fix_340.py"""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = os.path.join(HERE, "_fix_340.rs")


def lines_of(path):
    return open(path).read().split("\n")


def fn_unit(lines, fn_marker):
    f = next(i for i, l in enumerate(lines) if fn_marker in l)
    start = f
    while start > 0 and lines[start - 1].lstrip().startswith("///"):
        start -= 1
    indent = len(lines[f]) - len(lines[f].lstrip())
    close = f"{'' if indent == 0 else ' ' * indent}}}"
    end = f
    depth = 0
    seen_open = False
    while end < len(lines):
        ln = lines[end]
        if not seen_open and "{" in ln:
            seen_open = True
        depth += ln.count("{") - ln.count("}")
        if seen_open and depth == 0 and ln.rstrip().endswith("}"):
            return start, end + 1
        end += 1
    raise SystemExit(f"unbalanced fn: {fn_marker}")


def splice(path, unit, wrapped):
    """Reformat the unit in the scratch (the probe's own shape) and
    splice the answer back at the unit's own span."""
    body = (["impl App {"] + unit + ["}"]) if wrapped else unit
    with open(SCRATCH, "w") as f:
        f.write("#![allow(dead_code, unused_variables, unused_mut)]\n")
        f.write("\n".join(body) + "\n")
    r = subprocess.run([RUSTFMT, "--edition", "2021", SCRATCH], check=True)
    out = open(SCRATCH).read().split("\n")
    # strip the scratch's own head + the wrapper
    if out and out[0].startswith("#![allow"):
        out = out[1:]
    if wrapped:
        while out and out[-1] == "":
            out.pop()
        assert out[0].strip() == "impl App {", out[0]
        assert out[-1].strip() == "}", out[-1]
        out = out[1:-1]
    # drop trailing blanks
    while out and out[-1] == "":
        out.pop()
    return out


# the whole-file lane first
r = subprocess.run([RUSTFMT, "--edition", "2021", "crates/mc-client/src/brewing_sim.rs"])
print("brewing_sim rustfmt rc=", r.returncode)

SRC = "crates/mc-client/src/screens.rs"
L = lines_of(SRC)
IMPL_FNS = [
    ("fn held_item_display_name_kind(&self", True),
    ("fn creative_item_tooltip_hover(", True),
]
FREE_FNS = [
    ("fn draw_item_icon_pop(", False),
    ("fn draw_item_icon_pop_kind(", False),
    ("fn draw_item_icon_broken(", False),
]
spans = []
for marker, wrapped in IMPL_FNS + FREE_FNS:
    s, e = fn_unit(L, marker)
    spans.append((s, e, marker, wrapped))
# apply from the BOTTOM so the earlier spans stay valid
for s, e, marker, wrapped in sorted(spans, reverse=True):
    unit = L[s:e]
    fixed = splice(SRC, unit, wrapped)
    L = L[:s] + fixed + L[e:]
    print(f"spliced {marker} ({len(unit)} -> {len(fixed)} lines)")
with open(SRC, "w") as f:
    f.write("\n".join(L))

T = "crates/mc-client/src/screens/tests/part_11.rs"
Tl = lines_of(T)
t_units = [
    "fn the_potion_kinds_the_tint_tables_the_jar_wears",
    "fn the_potion_kinds_the_display_names_the_strip_wears",
]
tspans = []
for u in t_units:
    s, e = fn_unit(Tl, u)
    tspans.append((s, e, u))
for s, e, u in sorted(tspans, reverse=True):
    unit = Tl[s:e]
    fixed = splice(T, unit, False)
    Tl = Tl[:s] + fixed + Tl[e:]
    print(f"spliced {u} ({len(unit)} -> {len(fixed)} lines)")
with open(T, "w") as f:
    f.write("\n".join(Tl))

os.remove(SCRATCH)
print("FMT-FIX-DONE")

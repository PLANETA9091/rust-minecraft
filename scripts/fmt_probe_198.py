#!/usr/bin/env python3
"""fmt probe for the 198 round's own regions (the 335 harness law —
the ENCLOSING UNIT extraction: the complete fns / free fns the
self-contained syntax keeps, the diff hunks the touched ranges filter).
Usage: python3 scripts/fmt_probe_198.py [-v]"""
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = os.path.join(HERE, "_probe_198.rs")

verbose = "-v" in sys.argv


def lines_of(path):
    return open(path).read().split("\n")


def fn_unit(lines, fn_marker):
    """The complete fn unit: the contiguous /// doc head + the body to
    the balanced closing brace at the fn's own indent."""
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


def run_check(name, body):
    with open(SCRATCH, "w") as f:
        f.write("#![allow(dead_code, unused_variables, unused_mut)]\n")
        f.write("\n".join(body) + "\n")
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


SRC = "crates/mc-client/src/screens.rs"
L = lines_of(SRC)

ok = True

# A: the tooltip rows family + the hover + the creative hover — the
# three complete impl fns the one impl unit shares
a_units = [
    "fn item_tooltip_rows(id: i32, advanced: bool)",
    "fn item_tooltip_rows_enchanted(",
    "fn enchant_fullname_row(ench: usize, level: i32)",
    "fn item_tooltip_hover(&self, mx: i32, my: i32)",
    "fn creative_item_tooltip_hover(",
]
a0 = None
a1 = None
for u in a_units:
    s, e = fn_unit(L, u)
    a0 = s if a0 is None else min(a0, s)
    a1 = e if a1 is None else max(a1, e)
ok &= run_check("A_tooltip_family (3 impl fns)", ["impl App {"] + L[a0:a1] + ["}"])

# D/E/F: the render fns the glint rides — the whole fn, the diff the
# touched hunks filter
for name, marker in [
    ("D_render_inventory_slots", "fn render_inventory_slots(&mut self"),
    ("E_render_enchanting", "fn render_enchanting(&mut self"),
    ("F_render_hud", "fn render_hud(&mut self"),
]:
    s, e = fn_unit(L, marker)
    ok &= run_check(name, ["impl App {"] + L[s:e] + ["}"])

# H: the free fns (the phases helper + the enchanted draw)
h_units = ["fn glint_scroll_phases(millis: u64)", "fn draw_item_icon_enchanted("]
h0 = None
h1 = None
for u in h_units:
    s, e = fn_unit(L, u)
    h0 = s if h0 is None else min(h0, s)
    h1 = e if h1 is None else max(h1, e)
ok &= run_check("H_glint_free_fns", L[h0:h1])

# part_11: the two new test fns
T = lines_of("crates/mc-client/src/screens/tests/part_11.rs")
t_units = [
    "fn the_item_stack_the_enchantment_tooltip_lines_wear",
    "fn the_item_stack_the_enchanted_glint_wears",
]
for u in t_units:
    s, e = fn_unit(T, u)
    s2 = s
    while T[s2 - 1].strip().startswith("#[test]") or T[s2 - 1].lstrip().startswith("///"):
        s2 -= 1
    ok &= run_check(f"part_11 {u[3:40]}", ["mod probe198 {", "    use super::*;"] + [
        ("    " + ln if ln.strip() else ln) for ln in T[s2:e]
    ] + ["}"])

# the generated registry file: the whole-file check
r = subprocess.run(
    [RUSTFMT, "--edition", "2021", "--check", "crates/mc-registry/src/generated/enchantments.rs"],
    capture_output=True, text=True,
)
print(f"[{'CLEAN' if r.returncode == 0 else 'DIRTY'}] enchantments.rs (generated)")
if r.returncode != 0:
    ok = False
    if verbose:
        print(r.stdout[:3000])

# window.rs: the banked line's region — the whole-file check's own
# pre-existing drift the repo keeps; check the fn unit only
W = lines_of("crates/mc-client/src/window.rs")
s, e = fn_unit(W, "pub fn run_windowed(")
ok &= run_check("window run_windowed", W[s:e])

if os.path.exists(SCRATCH):
    os.remove(SCRATCH)
sys.exit(0 if ok else 1)

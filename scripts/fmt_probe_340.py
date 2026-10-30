#!/usr/bin/env python3
"""fmt probe for the autopush-340 round's own regions (the harness law —
the ENCLOSING UNIT extraction: the complete fns / free fns the
self-contained syntax keeps). Usage: python3 scripts/fmt_probe_340.py [-v]"""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = os.path.join(HERE, "_probe_340.rs")

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


ok = True

# the brewing module's own tables: the WHOLE-FILE check (the generated
# tables the round appends, the blend fns the same lane wears)
r = subprocess.run(
    [RUSTFMT, "--edition", "2021", "--check", "crates/mc-client/src/brewing_sim.rs"],
    capture_output=True, text=True,
)
print(f"[{'CLEAN' if r.returncode == 0 else 'DIRTY'}] brewing_sim.rs (whole)")
if r.returncode != 0:
    ok = False
    if verbose:
        print(r.stdout[:3000])

SRC = "crates/mc-client/src/screens.rs"
L = lines_of(SRC)

# the impl fns the round's own name/tick lanes ride
for name, marker in [
    ("item_tooltip_rows_kind", "fn item_tooltip_rows_kind("),
    ("potion_kind_display_name", "fn potion_kind_display_name(kind: i32) -> String"),
    ("held_item_display_name", "fn held_item_display_name(&self"),
    ("held_item_display_name_kind", "fn held_item_display_name_kind(&self"),
    ("tick_held_name", "pub fn tick_held_name(&mut self"),
    ("item_tooltip_hover", "fn item_tooltip_hover(&self"),
    ("creative_item_tooltip_hover", "fn creative_item_tooltip_hover("),
]:
    s, e = fn_unit(L, marker)
    body = ["impl App {"] + L[s:e] + ["}"]
    ok &= run_check(name, body)

# the RENDER fns the tint ride edits (the HUD hotbar loop, the
# inventory store rows, the brewing cells + carried)
for name, marker in [
    ("render_hud", "fn render_hud(&mut self"),
    ("render_inventory_slots", "fn render_inventory_slots(&mut self"),
    ("render_brewing", "fn render_brewing(&mut self"),
]:
    s, e = fn_unit(L, marker)
    body = ["impl App {"] + L[s:e] + ["}"]
    ok &= run_check(name, body)

# the FREE fns the draw path rides (the tint pair, the kinded wrappers,
# the 3D core the kind param wears)
for name, marker in [
    ("icon_tint", "fn icon_tint(tints: &[&str])"),
    ("icon_tint_kind", "fn icon_tint_kind(tints: &[&str], kind: i32)"),
    ("draw_item_icon", "fn draw_item_icon(g: &mut Gui, item_id: i32, x: i32, y: i32)"),
    ("draw_item_icon_kind", "fn draw_item_icon_kind("),
    ("draw_item_icon_pop", "fn draw_item_icon_pop("),
    ("draw_item_icon_pop_kind", "fn draw_item_icon_pop_kind("),
    ("draw_item_icon_broken", "fn draw_item_icon_broken("),
    ("draw_item_icon_3d_broken", "fn draw_item_icon_3d_broken("),
]:
    s, e = fn_unit(L, marker)
    ok &= run_check(name, L[s:e])

# part_11: the three new tests
T = lines_of("crates/mc-client/src/screens/tests/part_11.rs")
t_units = [
    "fn the_potion_kinds_the_tint_tables_the_jar_wears",
    "fn the_potion_kinds_the_display_names_the_strip_wears",
    "fn the_potion_kinds_the_icon_tint_the_wiring_wears",
]
for u in t_units:
    s, e = fn_unit(T, u)
    ok &= run_check(u.replace("fn ", ""), T[s:e])

print("ALL-CLEAN" if ok else "DIRTY-FOUND")
sys.exit(0 if ok else 1)

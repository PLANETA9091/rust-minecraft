#!/usr/bin/env python3
"""fmt probe for the dev-195 round's own regions (the 335 harness the
same walk wears — the spell walk the seven regions span)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_probe_195.rs")

GUI = "crates/mc-client/src/gui.rs"
SCREENS = "crates/mc-client/src/screens.rs"
P11 = "crates/mc-client/src/screens/tests/part_11.rs"


def load_lines(path):
    return open(path).read().split("\n")


def find(lines, marker, start=0):
    for i in range(start, len(lines)):
        if marker in lines[i]:
            return i
    raise SystemExit(f"marker not found: {marker}")


def find_exact(lines, text, start=0):
    for i in range(start, len(lines)):
        if lines[i] == text:
            return i
    raise SystemExit(f"exact line not found: {text!r}")


def trim(body):
    while body and body[-1].strip() == "":
        body.pop()
    return body


def dedent(body, n):
    out = []
    for ln in body:
        out.append(ln[n:] if ln.startswith(" " * n) else ln)
    return out


def indent(body, n):
    return [" " * n + ln if ln.strip() else ln for ln in body]


gui = load_lines(GUI)
scr = load_lines(SCREENS)
p11 = load_lines(P11)

# region G: the gui.rs sprite lane (the statements the fn body level
# wears — the 8-space nest the 4-space strip keeps)
g0 = find(gui, "THE ENCHANTING ROWS' OWN LANE")
g1 = find(gui, 'if !name.starts_with("mob_effect/")', g0)
body_g = dedent(trim(gui[g0:g1]), 4)

# region A: the scan fn (the doc head the roll fn's doc follows — the
# impl fns the brace-balanced slice keeps)
a0 = find(scr, "THE SPELL WALK'S OWN POWER SCAN")
a1 = find(scr, "THE SPELL WALK'S OWN COST ROLL")
body_a = trim(scr[a0:a1])

# region B: the roll + the refresh fns (the doc head the open fn's
# doc follows)
b0 = find(scr, "THE SPELL WALK'S OWN COST ROLL")
b1 = find(scr, "the right-press on the placed enchanting table")
body_b = trim(scr[b0:b1])

# region C: the render rows (the comment head the title's comment
# follows — the 8-space nest the 4-space strip keeps)
c0 = find(scr, "THE SPELL WALK'S OWN COST ROWS (the dev-195")
c1 = find(scr, "THE CENTERED TITLE", c0)
body_c = dedent(trim(scr[c0:c1]), 4)

# region D: the open fn's seat + costs insert (the 8-space statements
# the fn wrapper wears)
d0 = find(scr, "THE SPELL WALK'S OWN SEAT + COSTS")
d1 = find(scr, "the player's own rows the REAL inventory projects", d0)
body_d = dedent(trim(scr[d0:d1]), 4)

# region E: the click fn (the signature the impl wrapper keeps — the
# brace-balanced slice the walk wears; the fn's own close the first
# exact 4-space '}' line the body's deeper nest never speaks; the
# 4-space body the impl's own item level keeps — NO dedent the
# wrapper's indent speaks)
e0 = find(scr, "fn click_enchanting_slot(&mut self")
e1 = find_exact(scr, "    }", e0)
body_e = trim(scr[e0 : e1 + 1])

# region F: the three test fns (the first #[test] attr to the file
# tail — the mod wrapper the +4 indent the item level wears)
f0 = find(p11, "#[test]", find(p11, "fn the_enchanting_table_the_power_scan_the_ring_wears") - 3)
body_f = indent(trim(p11[f0:]), 4)

harness = {
    "G_gui_lane": ["fn probe195() {"] + body_g + ["}"],
    "A_scan_fn": ["impl Probe195 {"] + body_a + ["}"],
    "B_roll_refresh_fns": ["impl Probe195 {"] + body_b + ["}"],
    "C_render_rows": ["fn probe195() {"] + body_c + ["}"],
    "D_open_insert": ["fn probe195() {"] + body_d + ["}"],
    "E_click_fn": ["impl Probe195 {"] + body_e + ["}"],
    "F_test_fns": ["mod probe195 {"] + body_f + ["}"],
}

fail = False
for key, body in harness.items():
    open(SCRATCH, "w").write("\n".join(body) + "\n")
    r = subprocess.run(
        [RUSTFMT, "--check", "--edition", "2021", SCRATCH],
        capture_output=True, text=True,
    )
    if r.returncode != 0:
        fail = True
        print(f"=== {key} DIFF ===")
        print(r.stdout[:4000])
        if not r.stdout:
            print("(stderr)", r.stderr[:1500])
    else:
        print(f"{key}: CLEAN")

os.unlink(SCRATCH)
sys.exit(1 if fail else 0)

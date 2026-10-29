#!/usr/bin/env python3
"""fmt probe for the autopush-336 round's own regions (the 195 harness
the same walk wears — the SGA label lane the four regions span)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_probe_336.rs")

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

# region A: the sga_widths free fn (the doc head the Sprite impl
# follows)
a0 = find(gui, "THE SGA GLYPH WIDTHS' OWN MEASURE")
a1 = find(gui, "impl Sprite {", a0)
body_a = trim(gui[a0:a1])

# region B: the sga_text_width + the draw fns (the impl items the
# 4-space level keeps)
b0 = find(gui, "THE SGA TEXT'S OWN WIDTH")
b1 = find(gui, "/// Draw text with `", b0)
body_b = trim(gui[b0:b1])

# region C: the label roll fn (the impl item the refresh fn's doc
# follows)
c0 = find(scr, "THE SPELL WALK'S OWN LABEL ROLL")
c1 = find(scr, "THE SPELL WALK'S OWN COSTS REFRESH")
body_c = trim(scr[c0:c1])

# region D: the render rows' seed + the label lane (the 8-space
# statements the fn wrapper wears — the seed comment the for-loop
# follows, the label lane the number draw precedes)
d0 = find(scr, "LABEL'S OWN SEED (the dev-336")
d1 = find(scr, "THE CENTERED TITLE", d0)
body_d = dedent(trim(scr[d0:d1]), 4)

# region E: the label test fn (the #[test] attr to the file tail —
# the mod wrapper the +4 indent wears)
e0 = find(p11, "#[test]", find(p11, "fn the_enchanting_table_the_labels_the_sga_glyphs_wear") - 3)
body_e = indent(trim(p11[e0:]), 4)

harness = {
    "A_sga_widths_fn": body_a,
    "B_sga_text_fns": ["impl Probe336 {"] + body_b + ["}"],
    "C_label_roll_fn": ["impl Probe336 {"] + body_c + ["}"],
    "D_render_label_lane": ["fn probe336() {"] + body_d + ["}"],
    "E_label_test_fn": ["mod probe336 {"] + body_e + ["}"],
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
        print(r.stdout[:3000])
        if not r.stdout:
            print("(stderr)", r.stderr[:1200])
    else:
        print(f"{key}: CLEAN")

os.unlink(SCRATCH)
sys.exit(1 if fail else 0)

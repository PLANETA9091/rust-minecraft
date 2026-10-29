#!/usr/bin/env python3
"""fmt probe for the autopush-337 round's own regions (the 336 harness
the same walk wears — the click-enchant walk the seven regions span)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_probe_337.rs")

NOISE = "crates/mc-client/src/noise.rs"
SOUND = "crates/mc-client/src/sound.rs"
SCREENS = "crates/mc-client/src/screens.rs"
P11 = "crates/mc-client/src/screens/tests/part_11.rs"


def load_lines(path):
    return open(path).read().split("\n")


def find(lines, marker, start=0):
    for i in range(start, len(lines)):
        if marker in lines[i]:
            return i
    raise SystemExit(f"marker not found: {marker}")


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


noise = load_lines(NOISE)
sound = load_lines(SOUND)
scr = load_lines(SCREENS)
p11 = load_lines(P11)

# region A: the unbounded draw fn (the doc head the consume doc
# follows)
a0 = find(noise, "the UNBOUNDED draw the RandomSource default")
a1 = find(noise, "`consumeCount(n)`", a0)
body_a = trim(noise[a0:a1])

# region B: the use-voice fn (the impl item the fall voice follows)
b0 = find(sound, "THE ENCHANT TABLE'S OWN USE VOICE")
b1 = find(sound, "The FALL VOICE", b0)
body_b = trim(sound[b0:b1])

# region C: the two App fields (the struct items the table-seat doc
# follows)
c0 = find(scr, "THE CLICK-ENCHANT WALK'S OWN SEED")
c1 = find(scr, "THE SPELL WALK'S OWN TABLE SEAT", c0)
body_c = trim(scr[c0:c1])

# region D: the refresh fn (the impl item the click fn follows)
d0 = find(scr, "THE SPELL WALK'S OWN COSTS REFRESH")
d1 = find(scr, "THE CLICK-ENCHANT WALK (javap", d0)
body_d = trim(scr[d0:d1])

# region E: the click walk + the voice fn (the impl items the open
# doc follows)
e0 = find(scr, "THE CLICK-ENCHANT WALK (javap")
e1 = find(scr, "the right-press on the placed enchanting table", e0)
body_e = trim(scr[e0:e1])

# region F: the on_press enchanting branch (the 8-space if the
# fn wrapper wears — the slice the branch's own closing keeps, the
# chest branch the next marker answers)
f0 = find(scr, "if self.screen == ScreenKind::Enchanting {", find(scr, "pub fn on_press"))
f1 = find(scr, "if self.screen == ScreenKind::Chest {", f0)
while "}" != scr[f1 - 1].strip():
    f1 -= 1
body_f = dedent(trim(scr[f0:f1]), 8)
body_f = [" " * 4 + ln if ln.strip() else ln for ln in body_f]

# region G: the click test fn (the #[test] attr to the file tail —
# the mod wrapper the +4 indent wears)
g0 = find(p11, "#[test]", find(p11, "fn the_enchanting_table_the_click_enchant_walk_wears") - 3)
body_g = indent(trim(p11[g0:]), 4)

harness = {
    "A_noise_unbounded_fn": ["impl Probe337 {"] + body_a + ["}"],
    "B_sound_use_voice_fn": ["impl Probe337 {"] + body_b + ["}"],
    "C_app_fields_block": ["struct Probe337 {"] + body_c + ["}"],
    "D_costs_refresh_fn": ["impl Probe337 {"] + body_d + ["}"],
    "E_click_walk_fns": ["impl Probe337 {"] + body_e + ["}"],
    "F_press_option_branch": ["fn probe337() {"] + body_f + ["}"],
    "G_click_test_fn": ["mod probe337 {"] + body_g + ["}"],
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

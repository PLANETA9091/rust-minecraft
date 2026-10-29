#!/usr/bin/env python3
"""fmt probe for the 338 round's own regions (the 198 harness law —
the ENCLOSING UNIT extraction: the complete fns / free fns the
self-contained syntax keeps). Usage: python3 scripts/fmt_probe_338.py [-v]"""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = os.path.join(HERE, "_probe_338.rs")

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

# the brewing sim module: the WHOLE-FILE check (the new module the
# generator drift never wore)
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

# the touched impl fns the brewing round rides
for name, marker, wrapper in [
    ("open_brewing_for", "fn open_brewing_for(&mut self", True),
    ("close_brewing", "fn close_brewing(&mut self", True),
    ("render_brewing", "fn render_brewing(&mut self", True),
    ("brew_finish_voice", "fn brew_finish_voice(&mut self", True),
    ("brewing_words_bank_to_nbt", "fn brewing_words_bank_to_nbt(", False),
    ("brewing_words_from_nbt", "fn brewing_words_from_nbt(", False),
    ("update", "pub fn update(&mut self) {", True),
]:
    s, e = fn_unit(L, marker)
    body = L[s:e]
    if wrapper:
        body = ["impl App {"] + body + ["}"]
    ok &= run_check(name, body)

# sound.rs: the brew voice fn
S = lines_of("crates/mc-client/src/sound.rs")
s, e = fn_unit(S, "fn play_brew_voice_at(&mut self")
ok &= run_check("play_brew_voice_at", ["impl SoundBank {"] + S[s:e] + ["}"])

# gui.rs: the sprite_any fn (the brewing lane the body rides)
G = lines_of("crates/mc-client/src/gui.rs")
s, e = fn_unit(G, "fn sprite_any(&mut self")
ok &= run_check("sprite_any", ["impl GuiAssets {"] + G[s:e] + ["}"])

# world.rs: the consts block (the two rows the round adds)
W = lines_of("crates/mc-client/src/world.rs")
f = next(i for i, l in enumerate(W) if "pub const ITEM_BLAZE_POWDER" in l)
start = f
while start > 0 and W[start - 1].lstrip().startswith("///"):
    start -= 1
e2 = next(i for i, l in enumerate(W) if "pub const ITEM_SPLASH_POTION" in l)
ok &= run_check("brewing_wire_consts", W[start:e2 + 1])

# part_11: the four new tests + the hopper test the walk re-pins
T = lines_of("crates/mc-client/src/screens/tests/part_11.rs")
t_units = [
    "fn the_offline_hopper_the_brewing_side_the_fuel_lane_the_cooldown_wears",
    "fn the_offline_brewing_the_tick_walk_the_screen_wears",
    "fn the_offline_brewing_the_world_walk_the_banks_wear",
    "fn the_brewing_stand_the_words_bank_the_disk_wears",
    "fn the_offline_brewing_the_render_the_progress_sprites_wear",
]
for u in t_units:
    s, e = fn_unit(T, u)
    s2 = s
    while T[s2 - 1].strip().startswith("#[test]") or T[s2 - 1].lstrip().startswith("///"):
        s2 -= 1
    ok &= run_check(f"part_11 {u[3:44]}", ["mod probe338 {", "    use super::*;"] + [
        ("    " + ln if ln.strip() else ln) for ln in T[s2:e]
    ] + ["}"])

os.remove(SCRATCH)
print("ALL-CLEAN" if ok else "DIRTY-FOUND")
sys.exit(0 if ok else 1)

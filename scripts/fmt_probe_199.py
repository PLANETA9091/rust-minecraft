#!/usr/bin/env python3
"""fmt probe for the dev-199 round's own regions (the 339 harness law —
the ENCLOSING UNIT extraction: the complete fns / free fns the
self-contained syntax keeps). Usage: python3 scripts/fmt_probe_199.py [-v]"""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = os.path.join(HERE, "_probe_199.rs")

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

# the sound module's own fill voice: the WHOLE-FILE check (the event
# fn the round rides, the 27-line insert the one lane wears)
r = subprocess.run(
    [RUSTFMT, "--edition", "2021", "--check", "crates/mc-client/src/sound.rs"],
    capture_output=True, text=True,
)
print(f"[{'CLEAN' if r.returncode == 0 else 'DIRTY'}] sound.rs (whole)")
if r.returncode != 0:
    ok = False
    if verbose:
        print(r.stdout[:3000])

SRC = "crates/mc-client/src/screens.rs"
L = lines_of(SRC)

# the touched impl fns the fill round rides
for name, marker in [
    ("row_potion_kind", "fn row_potion_kind(&self"),
    ("bottle_fill_lane", "fn on_right_press_bottle_fill_lane(&mut self"),
    ("main_item_lane_head", "fn on_right_press_main_item_lane(&mut self"),
    ("transform_bucket_slot", "fn transform_bucket_slot(&mut self"),
    ("click_brewing_slot", "fn click_brewing_slot(&mut self"),
    ("on_drop_key", "pub fn on_drop_key(&mut self"),
    ("close_brewing", "pub fn close_brewing(&mut self"),
    ("player_state_now", "fn player_state_now(&self"),
]:
    s, e = fn_unit(L, marker)
    body = L[s:e]
    # the impl wrapper: the App impl the method units wear — the
    # free fns (player_state_now lives in the impl too) the same
    wrapper = not marker.startswith("fn row_potion_kind") or True
    body = ["impl App {"] + body + ["}"]
    ok &= run_check(name, body)

# the SavedPlayer struct (the potion_kinds field the round adds)
s, e = fn_unit(L, "pub struct SavedPlayer")
ok &= run_check("SavedPlayer struct", L[s:e])

# the free fns the disk lane rides
for name, marker in [
    ("player_row_to_nbt", "fn player_row_to_nbt("),
    ("player_state_from_nbt", "pub fn player_state_from_nbt("),
]:
    s, e = fn_unit(L, marker)
    ok &= run_check(name, L[s:e])

# part_11: the eight new tests
T = lines_of("crates/mc-client/src/screens/tests/part_11.rs")
t_units = [
    "fn the_offline_bottle_the_fill_the_water_kind_mints",
    "fn the_offline_bottle_the_refusals_the_pass_keeps",
    "fn the_offline_bottle_the_count3_the_free_row_mints",
    "fn the_offline_bottle_the_creative_the_bottle_keeps",
    "fn the_offline_bottle_the_fill_the_brew_loop_wears",
    "fn the_offline_brewing_the_quickmove_the_kind_hands_over",
    "fn the_offline_bottle_the_drop_the_seat_clears",
    "fn the_player_rows_the_potion_kinds_the_disk_wears",
]
for u in t_units:
    s, e = fn_unit(T, u)
    s2 = s
    while T[s2 - 1].strip().startswith("#[test]") or T[s2 - 1].lstrip().startswith("///"):
        s2 -= 1
    ok &= run_check(f"part_11 {u[3:48]}", ["mod probe199 {", "    use super::*;"] + [
        ("    " + ln if ln.strip() else ln) for ln in T[s2:e]
    ] + ["}"])

os.remove(SCRATCH)
print("ALL-CLEAN" if ok else "DIRTY-FOUND")
sys.exit(0 if ok else 1)

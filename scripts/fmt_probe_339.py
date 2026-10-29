#!/usr/bin/env python3
"""fmt probe for the 339 round's own regions (the 338 harness law —
the ENCLOSING UNIT extraction: the complete fns / free fns the
self-contained syntax keeps). Usage: python3 scripts/fmt_probe_339.py [-v]"""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = os.path.join(HERE, "_probe_339.rs")

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

# the brewing sim module: the WHOLE-FILE check (the kind section the
# round rides, the walks the rewrites wear)
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

# the touched impl fns the kinds round rides
for name, marker, wrapper in [
    ("open_brewing_for", "fn open_brewing_for(&mut self", True),
    ("close_brewing", "fn close_brewing(&mut self", True),
    ("click_brewing_slot", "fn click_brewing_slot(&mut self", True),
    ("throw_brewing_carried_outside", "fn throw_brewing_carried_outside(&mut self", True),
    ("brewing_kinds_bank_to_nbt", "fn brewing_kinds_bank_to_nbt(", False),
    ("brewing_kinds_from_nbt", "fn brewing_kinds_from_nbt(", False),
    ("update", "pub fn update(&mut self) {", True),
]:
    s, e = fn_unit(L, marker)
    body = L[s:e]
    if wrapper:
        body = ["impl App {"] + body + ["}"]
    ok &= run_check(name, body)

# world.rs: the consts block (the rows the round adds)
W = lines_of("crates/mc-client/src/world.rs")
f = next(i for i, l in enumerate(W) if "pub const ITEM_STONE" in l)
start = f
while start > 0 and W[start - 1].lstrip().startswith("///"):
    start -= 1
e2 = next(i for i, l in enumerate(W) if "pub const ITEM_PHANTOM_MEMBRANE" in l)
ok &= run_check("brewing_wire_consts_339", W[start:e2 + 1])

# part_11: the two new tests
T = lines_of("crates/mc-client/src/screens/tests/part_11.rs")
t_units = [
    "fn the_offline_brewing_the_kinds_the_mix_transform_wears",
    "fn the_brewing_stand_the_kinds_bank_the_disk_wears",
]
for u in t_units:
    s, e = fn_unit(T, u)
    s2 = s
    while T[s2 - 1].strip().startswith("#[test]") or T[s2 - 1].lstrip().startswith("///"):
        s2 -= 1
    ok &= run_check(f"part_11 {u[3:46]}", ["mod probe339 {", "    use super::*;"] + [
        ("    " + ln if ln.strip() else ln) for ln in T[s2:e]
    ] + ["}"])

os.remove(SCRATCH)
print("ALL-CLEAN" if ok else "DIRTY-FOUND")
sys.exit(0 if ok else 1)

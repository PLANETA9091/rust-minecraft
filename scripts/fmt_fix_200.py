#!/usr/bin/env python3
"""fmt fix for the dev-200 round's own regions (the splice law — the
rustfmt re-lay the dirty units wear, the region law the probe pairs).
Usage: python3 scripts/fmt_fix_200.py"""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = os.path.join(HERE, "_probe_200.rs")


def lines_of(path):
    return open(path).read().split("\n")


def fn_unit(lines, fn_marker):
    f = next(i for i, l in enumerate(lines) if fn_marker in l)
    start = f
    while start > 0 and lines[start - 1].lstrip().startswith("///"):
        start -= 1
    indent = len(lines[f]) - len(lines[f].lstrip())
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


def rustfmt_lines(body, wrapped=False):
    with open(SCRATCH, "w") as f:
        f.write("\n".join(body) + "\n")
    r = subprocess.run(
        [RUSTFMT, "--edition", "2021", SCRATCH], capture_output=True, text=True
    )
    if r.returncode != 0:
        raise SystemExit(f"rustfmt failed: {r.stderr[:500]}")
    out = open(SCRATCH).read().rstrip("\n").split("\n")
    if wrapped:
        # the impl wrapper the unit check wears — the shell the splice
        # strips (the first and last lines the harness adds)
        out = out[1:-1]
    return out


# 1. the brewing tables: the whole-file re-lay
BREW = "crates/mc-client/src/brewing_sim.rs"
r = subprocess.run([RUSTFMT, "--edition", "2021", BREW], capture_output=True, text=True)
print(f"brewing_sim.rs rustfmt rc={r.returncode}")

# 2. the screens units: the splice the re-laid bodies wear
SRC = "crates/mc-client/src/screens.rs"
for marker in [
    "fn potion_kind_attribute_rows(kind: i32)",
]:
    L = lines_of(SRC)
    s, e = fn_unit(L, marker)
    body = ["impl App {"] + L[s:e] + ["}"]
    fixed = rustfmt_lines(body, wrapped=True)
    L[s:e] = fixed
    open(SRC, "w").write("\n".join(L))
    print(f"spliced {marker} ({e - s} -> {len(fixed)} lines)")

# 3. the part_11 tests: the free-fn splice
T = "crates/mc-client/src/screens/tests/part_11.rs"
for u in [
    "fn the_potion_kinds_the_effect_lines_the_jar_wears",
    "fn the_potion_kinds_the_attribute_rows_the_jar_wears",
]:
    L = lines_of(T)
    s, e = fn_unit(L, u)
    fixed = rustfmt_lines(L[s:e])
    L[s:e] = fixed
    open(T, "w").write("\n".join(L))
    print(f"spliced {u} ({e - s} -> {len(fixed)} lines)")

print("FIX-APPLIED")

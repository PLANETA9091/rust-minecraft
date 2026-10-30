#!/usr/bin/env python3
"""fmt fix for the autopush-341 round's own dirty regions: the
brewing_sim.rs WHOLE-FILE pass (the generated tables the round appends)
+ the screens.rs potion_kind_effect_rows unit splice (the region law —
the enclosing unit the rustfmt re-lays, the file's other lines the
pre-existing drift keeps untouched). Re-run fmt_probe_341.py after."""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SCRATCH = os.path.join(ROOT, "scripts", "_fix_341.rs")


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


# 1) brewing_sim.rs — the whole-file pass (the region law for the
# generated tables file)
r = subprocess.run(
    [RUSTFMT, "--edition", "2021", os.path.join(ROOT, "crates/mc-client/src/brewing_sim.rs")],
    capture_output=True, text=True,
)
print("brewing_sim.rs rustfmt rc=", r.returncode, r.stderr[:200] if r.returncode else "")

# 2) screens.rs — the potion_kind_effect_rows unit splice
SRC = os.path.join(ROOT, "crates/mc-client/src/screens.rs")
L = lines_of(SRC)
s, e = fn_unit(L, "fn potion_kind_effect_rows(kind: i32)")
body = ["impl App {"] + L[s:e] + ["}"]
with open(SCRATCH, "w") as f:
    f.write("#![allow(dead_code, unused_variables, unused_mut)]\n")
    f.write("\n".join(body) + "\n")
r = subprocess.run(
    [RUSTFMT, "--edition", "2021", SCRATCH], capture_output=True, text=True
)
print("unit rustfmt rc=", r.returncode, r.stderr[:200] if r.returncode else "")
fmt = open(SCRATCH).read().split("\n")
# the scratch head (#![allow...]) + the impl wrapper lines strip
fmt_body = fmt[2:]
while fmt_body and fmt_body[0].strip() == "":
    fmt_body.pop(0)
fmt_body = fmt_body[:-1]  # the trailing impl }
# the leading blank rustfmt may add after the wrapper
while fmt_body and fmt_body[0].strip() == "":
    fmt_body.pop(0)
new_lines = L[:s] + fmt_body + L[e:]
open(SRC, "w").write("\n".join(new_lines))
print(f"spliced potion_kind_effect_rows: lines {s}..{e} -> {len(fmt_body)} lines")

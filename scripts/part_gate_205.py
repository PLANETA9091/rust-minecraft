#!/usr/bin/env python3
"""THE TWELVE-WINDOW WALK (autopush-205): the part gate the codegen
peak's own answer — the mod.rs the one-part window the compile wears,
the sibling parts the gate the full-tree rustc peak the cgroup never
crosses. The walk the eleven part windows + the all-gated rest window
the counts bank, the mod.rs the git HEAD the byte-exact restore keeps.

Usage: python3 scripts/part_gate_205.py [part|rest]
"""
import subprocess
import sys

MOD = "crates/mc-client/src/screens/tests/mod.rs"
CARGO = subprocess.run(["bash", "-lc", "echo $HOME/.cargo/bin/cargo"],
                       capture_output=True, text=True).stdout.strip() or "cargo"

ORIGINAL = open(MOD).read() if __import__("os").path.exists(MOD) else None
PARTS = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]


def write_mod(only):
    lines = []
    for line in ORIGINAL.splitlines(keepends=True):
        s = line.strip()
        hit = None
        for n in PARTS:
            if s == f"mod part_{n};":
                hit = n
                break
        if hit is None:
            lines.append(line)
        elif only is not None and hit == only:
            lines.append(line)
        else:
            lines.append(f"// GATED {line}")
    open(MOD, "w").write("".join(lines))


def run(only):
    write_mod(only)
    label = f"part_{only}" if only else "rest"
    cmd = [CARGO, "test", "-p", "mc-client", "--", "--format=terse"]
    r = subprocess.run(cmd, capture_output=True, text=True, timeout=590)
    out = r.stdout + r.stderr
    tail = out.strip().split("\n")[-3:]
    print(f"[{label}] rc={r.returncode} :: {' | '.join(tail)}", flush=True)
    return r.returncode, out


def restore():
    if ORIGINAL is not None:
        open(MOD, "w").write(ORIGINAL)


if __name__ == "__main__":
    arg = sys.argv[1] if len(sys.argv) > 1 else "all"
    rc = 0
    try:
        if arg == "all":
            for n in PARTS:
                rc_n, _ = run(n)
                rc = rc or rc_n
            rc_r, _ = run(None)
            rc = rc or rc_r
        elif arg == "rest":
            _, _ = run(None)
        else:
            _, _ = run(int(arg))
    finally:
        restore()
    sys.exit(rc)

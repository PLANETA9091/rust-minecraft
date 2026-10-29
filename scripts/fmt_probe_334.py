#!/usr/bin/env python3
"""fmt probe for the 334 round's own mesher regions (the 333 harness
the same walk wears — the pedestal lane the three regions span)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_probe_334.rs")

SRC = "crates/mc-client/src/mesher.rs"

lines = open(SRC).read().split("\n")


def find(marker: str, start: int = 0) -> int:
    for i in range(start, len(lines)):
        if marker in lines[i]:
            return i
    raise SystemExit(f"marker not found: {marker}")


# region A: the fn pair (the bake + the named tile — the bed models
# doc the close banks)
a0 = find("THE ENCHANTING TABLE'S OWN CACHED BAKE (the autopush-334 round")
a1 = find("THE BED MODELS' OWN CACHED BAKES (the dev-194 round")
# region B: the dispatch branch (the pedestal-lane comment to the
# end-gateway doc — the self-contained if the harness wraps)
b0 = find("THE ENCHANTING TABLE'S OWN PEDESTAL LANE (the autopush-334")
b1 = find("THE END GATEWAY'S OWN INVISIBLE LANE (the autopush-310", b0 + 1)
# region C: the test fn (the #[test] attr to the NEXT #[test] after)
c0 = find("#[test]", find("fn the_enchanting_table_the_pedestal_the_6_quads_wear") - 3)
c1 = find("#[test]", c0 + 1)


def trim(body):
    while body and body[-1].strip() == "":
        body.pop()
    return body


body_a = trim(lines[a0:a1])
# B: the dispatch branch rides the mesher loop's own 16-space nest —
# the dedent the harness's fn-body level serves (the self-contained
# if the 12-space strip keeps)
body_b = []
for ln in trim(lines[b0:b1]):
    body_b.append(ln[12:] if ln.startswith(" " * 12) else ln)
body_c = trim(lines[c0:c1])

harness = {
    "A_fn_pair": body_a,
    "B_dispatch_branch": ["fn probe334() {"] + body_b + ["}"],
    "C_test_fn": ["mod probe334 {"] + body_c + ["}"],
}

fail = False
for key, body in harness.items():
    open(SCRATCH, "w").write("\n".join(body) + "\n")
    r = subprocess.run(
        [RUSTFMT, "--edition", "2021", "--check", SCRATCH],
        capture_output=True, text=True,
    )
    if r.returncode == 0:
        print(f"{key}: CLEAN")
    else:
        fail = True
        print(f"{key}: DIRTY rc={r.returncode}")
        print(r.stdout[:3000])
        print(r.stderr[:500])

os.remove(SCRATCH)
sys.exit(1 if fail else 0)

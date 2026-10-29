#!/usr/bin/env python3
"""fmt probe for the 335 round's own mesher regions (the 334 harness
the same walk wears — the book-float lane the three regions span)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_probe_335.rs")

SRC = "crates/mc-client/src/mesher.rs"

lines = open(SRC).read().split("\n")


def find(marker: str, start: int = 0) -> int:
    for i in range(start, len(lines)):
        if marker in lines[i]:
            return i
    raise SystemExit(f"marker not found: {marker}")


# region A: the BookQuad struct + the ENCHANT_BOOK_QUADS static (the
# doc-comment head the bed-models doc follows)
a0 = find("THE BOOK FLOAT'S OWN BAKED QUAD (the autopush-335 round")
a1 = find("THE BED MODELS' OWN CACHED BAKES (the dev-194 round")
# region B: the dispatch branch (the book-float comment to the
# continue — the self-contained for the harness wraps; the branch's
# own closing brace the slice EXCLUDES — the extra-brace law the fn
# wrapper's balance keeps)
b0 = find("THE BOOK FLOAT (the autopush-335 round — the")
b1 = find("continue;", b0) + 1
# region C: the test fn (the #[test] attr to the NEXT #[test] after)
c0 = find("#[test]", find("fn the_enchanting_table_the_book_float_the_16_quads_wear") - 3)
c1 = find("#[test]", c0 + 1)


def trim(body):
    while body and body[-1].strip() == "":
        body.pop()
    return body


body_a = trim(lines[a0:a1])
# B: the dispatch branch rides the enchanting branch's own 20-space
# nest — the dedent the harness's fn-body level serves (the
# self-contained for the 16-space strip keeps)
body_b = []
for ln in trim(lines[b0:b1]):
    body_b.append(ln[16:] if ln.startswith(" " * 16) else ln)
body_c = trim(lines[c0:c1])

harness = {
    "A_struct_static": body_a,
    "B_dispatch_branch": ["fn probe335() {"] + body_b + ["}"],
    "C_test_fn": ["mod probe335 {"] + body_c + ["}"],
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
    else:
        print(f"{key}: CLEAN")

os.unlink(SCRATCH)
sys.exit(1 if fail else 0)

#!/usr/bin/env python3
"""fmt fix: rustfmt-format the two dirty 335 regions (A static + C
test fn) and paste back into mesher.rs (the 334 walk the same lane
wears — the region law the probe's verdict banks)."""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_fix_335.rs")

SRC = "crates/mc-client/src/mesher.rs"

lines = open(SRC).read().split("\n")


def find(marker: str, start: int = 0) -> int:
    for i in range(start, len(lines)):
        if marker in lines[i]:
            return i
    raise SystemExit(f"marker not found: {marker}")


def trim(body):
    while body and body[-1].strip() == "":
        body.pop()
    return body


def reformat(body, wrapper):
    """rustfmt the body inside the wrapper, return inner lines."""
    text = "\n".join(wrapper(body))
    open(SCRATCH, "w").write(text + "\n")
    r = subprocess.run([RUSTFMT, "--edition", "2021", SCRATCH],
                       capture_output=True, text=True)
    if r.returncode != 0:
        raise SystemExit(f"rustfmt failed: {r.stderr[:2000]}")
    out = open(SCRATCH).read().split("\n")
    while out and out[-1].strip() == "":
        out.pop()
    # strip the wrapper's first/last lines
    inner = out[1:-1]
    os.unlink(SCRATCH)
    return inner


# region A: struct + static (top-level, no wrapper needed)
a0 = find("THE BOOK FLOAT'S OWN BAKED QUAD (the autopush-335 round")
a1 = find("THE BED MODELS' OWN CACHED BAKES (the dev-194 round")
body_a = trim(lines[a0:a1])

# region C: test fn (wrap in a mod)
c0 = find("#[test]", find("fn the_enchanting_table_the_book_float_the_16_quads_wear") - 3)
c1 = find("#[test]", c0 + 1)
body_c = trim(lines[c0:c1])

# B is clean — untouched.
new_a = reformat(body_a, lambda b: b)
new_c = reformat(body_c, lambda b: ["mod fix335 {"] + b + ["}"])

# paste back (bottom-up so indices hold)
lines[c0:c1] = new_c
lines[a0:a1] = new_a

open(SRC, "w").write("\n".join(lines))
print(f"A: {len(body_a)} -> {len(new_a)} lines; C: {len(body_c)} -> {len(new_c)} lines")

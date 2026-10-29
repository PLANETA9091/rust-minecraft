#!/usr/bin/env python3
"""fmt probe for the 327 round's own regions (v3 — the trim-then-wrap
order the v2's shared-list artifact retires: trim boundaries first, then
wrap the impl+fn harnesses)."""
import subprocess
import sys

SRC = "crates/mc-client/src/screens.rs"

text = open(SRC).read()
lines = text.split("\n")  # 0-based; line N (1-based) = lines[N-1]


def find(marker: str) -> int:
    for i, ln in enumerate(lines):
        if marker in ln:
            return i
    raise SystemExit(f"marker not found: {marker}")


# region A: the back-layer block (from the round marker to the grid loop)
a0 = find("THE HIGHLIGHT'S OWN BACK LAYER (the 327 round")
a1 = find("THE 3x3 GRID + THE RESULT SLOT")
# region B: the front-layer block (round marker to the fn's closing brace)
b0 = find("THE HIGHLIGHT'S OWN FRONT LAYER (the extractSlotHighlightFront")
b1 = find("THE CREATIVE PICKER'S OWN RENDER")
# region C: the table_highlight_origin fn (doc marker to the F4 doc)
c0 = find("THE CRAFTING TABLE'S OWN HIGHLIGHT SEAT (the 327 round")
c1 = find("THE F4 SWITCHER'S OWN RENDER")

# ---- trim the extraction boundaries FIRST (no shared-list aliasing) ----
# A: exactly the round's block, ends before the grid-loop comment
body_a = lines[a0:a1]
while body_a and body_a[-1].strip() == "":
    body_a.pop()
# B: drop the render fn's own closing brace (4-space) + blank tail —
# the if-let's own 8-space close the region keeps
body_b = lines[b0:b1]
while body_b and (body_b[-1].strip() == "" or body_b[-1] == "    }"):
    body_b.pop()
# C: the fn + doc, trailing blank before the F4 doc the cut rides
body_c = lines[c0:c1]
while body_c and body_c[-1].strip() == "":
    body_c.pop()

harness = {
    "A_back_layer": ["impl Probe327 {", "    fn probe327() {"] + body_a + ["    }", "}"],
    "B_front_layer": ["impl Probe327 {", "    fn probe327() {"] + body_b + ["    }", "}"],
    "C_fn": ["impl Probe327 {"] + body_c + ["}"],
}

fail = False
for key, body in harness.items():
    open("/tmp/probe_327.rs", "w").write("\n".join(body) + "\n")
    r = subprocess.run(
        ["rustfmt", "--edition", "2021", "--check", "/tmp/probe_327.rs"],
        capture_output=True, text=True,
    )
    if r.returncode == 0:
        print(f"{key}: CLEAN")
    else:
        fail = True
        print(f"{key}: DIRTY rc={r.returncode}")
        print(r.stdout[:3000])
        print(r.stderr[:500])

sys.exit(1 if fail else 0)

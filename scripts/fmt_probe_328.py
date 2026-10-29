#!/usr/bin/env python3
"""fmt probe for the 328 round's own regions (the 327 v3 harness the
trim-then-wrap order keeps — the furnace's own blocks the same walk
spans)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SRC = "crates/mc-client/src/screens.rs"

text = open(SRC).read()
lines = text.split("\n")  # 0-based; line N (1-based) = lines[N-1]


def find(marker: str) -> int:
    for i, ln in enumerate(lines):
        if marker in ln:
            return i
    raise SystemExit(f"marker not found: {marker}")


# region A: the back-layer block (from the round marker to the icons loop)
a0 = find("THE HIGHLIGHT'S OWN BACK LAYER (the 328 round")
a1 = find("for &(sx, sy, slot) in &self.furnace_slot_rects {")
# region B: the front-layer block (round marker to the fn's closing brace)
b0 = find("THE HIGHLIGHT'S OWN FRONT LAYER (the extractSlotHighlightFront")
b1 = find("THE DISPENSER'S OWN RENDER")
# region C: the furnace_hover_seat + furnace_highlight_origin fns (doc
# marker to the F4 doc)
c0 = find("THE FURNACE'S OWN HOVER SEAT (the 328 round")
c1 = find("THE F4 SWITCHER'S OWN RENDER")

# ---- trim the extraction boundaries FIRST (no shared-list aliasing) ----
# A: exactly the round's block, ends before the icons-loop comment
body_a = lines[a0:a1]
while body_a and body_a[-1].strip() == "":
    body_a.pop()
# B: drop the render fn's own closing brace (4-space) + blank tail —
# the if-let's own 8-space close the region keeps
body_b = lines[b0:b1]
while body_b and (body_b[-1].strip() == "" or body_b[-1] == "    }"):
    body_b.pop()
# C: the fns + docs, trailing blank before the F4 doc the cut rides
body_c = lines[c0:c1]
while body_c and body_c[-1].strip() == "":
    body_c.pop()

harness = {
    "A_back_layer": ["impl Probe328 {", "    fn probe328() {"] + body_a + ["    }", "}"],
    "B_front_layer": ["impl Probe328 {", "    fn probe328() {"] + body_b + ["    }", "}"],
    "C_fns": ["impl Probe328 {"] + body_c + ["}"],
}

fail = False
for key, body in harness.items():
    open("/tmp/probe_328.rs", "w").write("\n".join(body) + "\n")
    r = subprocess.run(
        [RUSTFMT, "--edition", "2021", "--check", "/tmp/probe_328.rs"],
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

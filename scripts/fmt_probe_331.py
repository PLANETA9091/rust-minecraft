#!/usr/bin/env python3
"""fmt probe for the 331 round's own regions (the 327 v3 harness the
trim-then-wrap order keeps — the dispenser's own blocks the same walk
spans; the 329/330 SHARED-MARKER lessons the anchors wear: the 331
round marker the back lane keeps, the front lane the FORWARD search
from the unique 331 anchor the dispenser's own row the first match
banks — the chest's earlier row the find_first ordering never rides)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_probe_331.rs")

SRC = "crates/mc-client/src/screens.rs"

lines = open(SRC).read().split("\n")


def find(marker: str, start: int = 0) -> int:
    for i in range(start, len(lines)):
        if marker in lines[i]:
            return i
    raise SystemExit(f"marker not found: {marker}")


# the unique 331 anchor (the back-layer round marker — ONE in the file)
a_back = find("THE HIGHLIGHT'S OWN BACK LAYER (the 331 round")
# region A: the dispenser's back-layer block (the 331 marker to the
# dispenser's OWN icons loop — the unique furnace-worded comment the
# end anchor keeps)
a0 = a_back
a1 = find("THE SLOT ICONS (the furnace's own draw", a_back + 1)
# region B: the dispenser's front-layer block (the FIRST shared front
# marker AFTER the 331 back anchor — the dispenser's own row the order
# speaks; the end the next render fn's own doc the close banks)
b0 = find("THE HIGHLIGHT'S OWN FRONT LAYER (the extractSlotHighlightFront", a_back + 1)
b1 = find("THE BEACON'S OWN ROUND (the autopush-322 round", b0 + 1)
# region C: the dispenser_hover_seat + dispenser_highlight_origin fns
# (the doc marker to the F4 doc — the 329 harness the same law keeps)
c0 = find("THE DISPENSER'S OWN HOVER SEAT (the 331 round")
c1 = find("THE F4 SWITCHER'S OWN RENDER")

# ---- trim the extraction boundaries FIRST (no shared-list aliasing) ----
body_a = lines[a0:a1]
while body_a and body_a[-1].strip() == "":
    body_a.pop()
# B: drop the render fn's own closing brace (4-space) + blank tail —
# the if-let's own 8-space close the region keeps (the 329 trim law)
body_b = lines[b0:b1]
while body_b and (body_b[-1].strip() == "" or body_b[-1] == "    }"):
    body_b.pop()
body_c = lines[c0:c1]
while body_c and body_c[-1].strip() == "":
    body_c.pop()

harness = {
    "A_dispenser_back_layer": ["impl Probe331 {", "    fn probe331() {"] + body_a + ["    }", "}"],
    "B_dispenser_front_layer": ["impl Probe331 {", "    fn probe331() {"] + body_b + ["    }", "}"],
    "C_fns": ["impl Probe331 {"] + body_c + ["}"],
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

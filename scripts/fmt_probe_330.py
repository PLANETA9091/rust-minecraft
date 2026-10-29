#!/usr/bin/env python3
"""fmt probe for the 330 round's own regions (the 327 v3 harness the
trim-then-wrap order keeps — the hopper's own blocks the same walk
spans; the 329 SHARED-MARKER lesson the anchors wear: the 330 round
marker the back lane keeps, the front lane the FORWARD search from
the unique 330 anchor the first match banks — the chest's earlier
row the find_first ordering never rides)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_probe_330.rs")

SRC = "crates/mc-client/src/screens.rs"

text = open(SRC).read()
lines = text.split("\n")  # 0-based; line N (1-based) = lines[N-1]


def find(marker: str, start: int = 0) -> int:
    for i in range(start, len(lines)):
        if marker in lines[i]:
            return i
    raise SystemExit(f"marker not found: {marker}")


# the unique 330 anchor (the back-layer round marker — ONE in the file)
a_back = find("THE HIGHLIGHT'S OWN BACK LAYER (the 330 round")
# region A: the hopper's back-layer block (the 330 marker to the hopper's
# OWN icons loop — the forward search the shared SLOT ICONS comment's
# first match AFTER the anchor keeps)
a0 = a_back
a1 = find("THE SLOT ICONS (the chest's own draw", a_back + 1)
# region B: the hopper's front-layer block (the FIRST shared front
# marker AFTER the 330 back anchor — the hopper's own row the order
# speaks; the end the render_hopper fn's own next-doc the close banks)
b0 = find("THE HIGHLIGHT'S OWN FRONT LAYER (the extractSlotHighlightFront", a_back + 1)
b1 = find("The survival inventory's BASE pass", b0 + 1)

# ---- trim the extraction boundaries FIRST (no shared-list aliasing) ----
body_a = lines[a0:a1]
while body_a and body_a[-1].strip() == "":
    body_a.pop()
# B: drop the render fn's own closing brace (4-space) + blank tail —
# the if-let's own 8-space close the region keeps (the 329 trim law)
body_b = lines[b0:b1]
while body_b and (body_b[-1].strip() == "" or body_b[-1] == "    }"):
    body_b.pop()

harness = {
    "A_hopper_back_layer": ["impl Probe330 {", "    fn probe330() {"] + body_a + ["    }", "}"],
    "B_hopper_front_layer": ["impl Probe330 {", "    fn probe330() {"] + body_b + ["    }", "}"],
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

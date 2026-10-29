#!/usr/bin/env python3
"""fmt probe for the 329 round's own regions (the 327 v3 harness the
trim-then-wrap order keeps — the chest's own blocks the same walk
spans; the front-layer marker the FIVE screens now share the LAST
match the chest lane keeps)."""
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


def find_last(marker: str) -> int:
    hit = -1
    for i, ln in enumerate(lines):
        if marker in ln:
            hit = i
    if hit < 0:
        raise SystemExit(f"marker not found: {marker}")
    return hit


# region A: the sprites-first + back-layer block (round marker to the
# grid icons loop — the chest's own reorder the block spans)
a0 = find("THE SLOT SPRITES FIRST (the renderBg decomposition's own")
a1 = find("for (i, (id, count)) in self.chest_slots.iter().enumerate() {")
# region B: the front-layer block (the chest's own — the render_chest
# lane the file's FIRST of the five shared markers keeps)
b0 = find("THE HIGHLIGHT'S OWN FRONT LAYER (the extractSlotHighlightFront")
b1 = find("THE HOPPER'S OWN RENDER (javap 26.2 HopperScreen")
# region C: the chest_hover_seat + chest_highlight_origin fns (doc
# marker to the F4 doc)
c0 = find("THE CHEST'S OWN HOVER SEAT (the 329 round")
c1 = find("THE F4 SWITCHER'S OWN RENDER")

# ---- trim the extraction boundaries FIRST (no shared-list aliasing) ----
body_a = lines[a0:a1]
while body_a and body_a[-1].strip() == "":
    body_a.pop()
# B: drop the render fn's own closing brace (4-space) + blank tail —
# the if-let's own 8-space close the region keeps
body_b = lines[b0:b1]
while body_b and (body_b[-1].strip() == "" or body_b[-1] == "    }"):
    body_b.pop()
body_c = lines[c0:c1]
while body_c and body_c[-1].strip() == "":
    body_c.pop()

harness = {
    "A_sprites_back": ["impl Probe329 {", "    fn probe329() {"] + body_a + ["    }", "}"],
    "B_front_layer": ["impl Probe329 {", "    fn probe329() {"] + body_b + ["    }", "}"],
    "C_fns": ["impl Probe329 {"] + body_c + ["}"],
}

fail = False
for key, body in harness.items():
    open("/tmp/probe_329.rs", "w").write("\n".join(body) + "\n")
    r = subprocess.run(
        [RUSTFMT, "--edition", "2021", "--check", "/tmp/probe_329.rs"],
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

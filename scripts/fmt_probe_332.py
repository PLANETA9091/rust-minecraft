#!/usr/bin/env python3
"""fmt probe for the 332 round's own regions (the 331 harness the same
walk wears — the brewing blocks the eight regions span; the 329/330/331
anchor lessons the search wears: the unique 332-round markers the
anchors keep, the front lane the FORWARD search from the brewing back
anchor the first match banks)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_probe_332.rs")

SRC = "crates/mc-client/src/screens.rs"

lines = open(SRC).read().split("\n")


def find(marker: str, start: int = 0) -> int:
    for i in range(start, len(lines)):
        if marker in lines[i]:
            return i
    raise SystemExit(f"marker not found: {marker}")


# the unique 332 anchor (the brewing back-layer round marker — ONE in
# the file)
a_back = find("THE HIGHLIGHT'S OWN BACK LAYER (the autopush-332 round")
# region A: the brewing back-layer block (the 332 marker to the
# brewing's OWN icons loop — the dispenser-worded comment the end
# anchor keeps)
a0 = a_back
a1 = find("THE SLOT ICONS (the dispenser's own draw", a_back + 1)
# region B: the brewing front-layer block (the FIRST shared front
# marker AFTER the 332 back anchor; the end the next render fn's own
# doc the close banks)
b0 = find("THE HIGHLIGHT'S OWN FRONT LAYER (the extractSlotHighlightFront", a_back + 1)
b1 = find("THE BEACON'S OWN ROUND (the autopush-322 round", b0 + 1)
# region C: the brewing_hover_seat + brewing_highlight_origin fns (the
# doc marker to the F4 doc — the 331 harness the same law keeps)
c0 = find("THE BREWING STAND'S OWN HOVER SEAT (the autopush-332 round")
c1 = find("THE F4 SWITCHER'S OWN RENDER")
# region D: the door + mirror sync + close fns (the brewing door doc
# to the beacon open doc — the whole block the insert banks)
d0 = find("THE BREWING STAND'S OWN DOOR (javap 26.2")
d1 = find("THE BEACON'S OWN OFFLINE OPEN (the autopush-322 round", d0 + 1)
# region E: the qc arm + paint fns (the brewing drag doc to the table
# classification doc)
e0 = find("THE BREWING DRAG'S OWN ARM")
e1 = find("THE TABLE'S OWN SEAT CLASSIFICATION", e0 + 1)
# region F: the distribute fn (the distribution doc to the ghost doc)
f0 = find("THE BREWING DRAG'S OWN DISTRIBUTION")
f1 = find("THE GHOST'S OWN RECALC", f0 + 1)
# region G: the outside throw + click lane fns (the throw doc to the
# dispenser throw doc)
g0 = find("THE BREWING STAND'S OWN OUTSIDE THROW")
g1 = find("THE DISPENSER'S OWN OUTSIDE THROW", g0 + 1)

# ---- trim the extraction boundaries FIRST (no shared-list aliasing) ----
def trim(body, extra_braces=False):
    while body and body[-1].strip() == "":
        body.pop()
    if extra_braces:
        while body and body[-1] == "    }":
            body.pop()
    return body


body_a = trim(lines[a0:a1])
body_b = trim(lines[b0:b1], extra_braces=True)
body_c = trim(lines[c0:c1])
body_d = trim(lines[d0:d1])
body_e = trim(lines[e0:e1])
body_f = trim(lines[f0:f1])
body_g = trim(lines[g0:g1])

harness = {
    "A_brewing_back_layer": ["impl Probe332 {", "    fn probe332() {"] + body_a + ["    }", "}"],
    "B_brewing_front_layer": ["impl Probe332 {", "    fn probe332() {"] + body_b + ["    }", "}"],
    "C_fns": ["impl Probe332 {"] + body_c + ["}"],
    "D_door_fns": ["impl Probe332 {"] + body_d + ["}"],
    "E_qc_fns": ["impl Probe332 {"] + body_e + ["}"],
    "F_distribute_fn": ["impl Probe332 {"] + body_f + ["}"],
    "G_click_fns": ["impl Probe332 {"] + body_g + ["}"],
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

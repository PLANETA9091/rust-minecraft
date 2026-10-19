#!/usr/bin/env python3
"""QA probe for the dev-76 furnace GUI golden scene.

Verifies the FurnaceMenu canvas's own regions on the staged render:
  1. the INPUT slot wears the iron_ore icon (the bright texels),
  2. the FUEL slot wears the coal icon (the dark texels),
  3. the RESULT slot stays the empty canvas,
  4. the flame cell wears the lit_progress sprite (the warm fire
     texels the mid-arc words [800, 1600, ..] drive at fh = 8),
  5. the arrow cell wears the burn_progress sprite (the bright
     progress texels the aw = 12 fold speaks).
The panel origin for 320x180: the 176x166 canvas centered -> (72, 7).
"""
import subprocess, sys
from PIL import Image

ROOT = "/home/z/my-project/rust-minecraft"
CLIENT = f"{ROOT}/target/release/mc-client"
OUT = f"{ROOT}/target/golden_verify/qa_furnace_gui.png"

subprocess.run(
    [CLIENT, "--headless", "--screen", "furnace", "--ui-frame", "30",
     "--seed", "20260214", "--width", "320", "--height", "180", "--out", OUT],
    cwd=ROOT, check=True, capture_output=True,
)
img = Image.open(OUT).convert("RGB")
px = img.load()
P = (72, 7)

def stats(x0, y0, x1, y1):
    warm = bright = dark = 0
    for y in range(y0, y1):
        for x in range(x0, x1):
            r, g, b = px[x, y]
            if r >= 120 and b <= 90 and r - b > 60:
                warm += 1
            if r > 220 and g > 200 and b > 150:
                bright += 1
            if r < 60 and g < 60 and b < 60:
                dark += 1
    return warm, bright, dark

checks = [
    ("flame cell (lit_progress)", stats(P[0]+56, P[1]+36, P[0]+70, P[1]+50),
     lambda w, b, d: w > 20, "warm fire texels"),
    ("arrow cell (burn_progress)", stats(P[0]+79, P[1]+34, P[0]+103, P[1]+50),
     lambda w, b, d: b > 20, "bright progress texels"),
    ("INPUT slot (iron_ore icon)", stats(P[0]+57, P[1]+18, P[0]+73, P[1]+34),
     lambda w, b, d: b > 20, "bright ore texels"),
    ("FUEL slot (coal icon)", stats(P[0]+57, P[1]+54, P[0]+73, P[1]+70),
     lambda w, b, d: d > 50, "dark coal texels"),
    ("RESULT slot (empty canvas)", stats(P[0]+117, P[1]+36, P[0]+133, P[1]+52),
     lambda w, b, d: w == 0 and b == 0 and d == 0, "the flat emptiness"),
]

ok = True
for label, (w, b, d), gate, why in checks:
    verdict = gate(w, b, d)
    ok &= verdict
    print(f"{'PASS' if verdict else 'FAIL'}  {label:32s} warm={w:3d} bright={b:3d} dark={d:3d}  ({why})")

print("QA VERDICT:", "PASS — the menu canvas the slots the words wear" if ok else "FAIL")
sys.exit(0 if ok else 1)

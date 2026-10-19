#!/usr/bin/env python3
"""QA probe for the dev-78 chest_pair golden scene (the placed pair's
own world pin).

Renders the staged scene at the QA resolution and verifies the mesher's
double arms' own pixel work:
  1. THE PAIR'S OWN WOOD MASS — the two joined halves the wood body
     wears in the pair region (the plateau edge the seat keeps),
  2. THE TWO LATCHES — the neutral-gray dots (94,94,94) the front
     tiles speak, ONE PER HALF, the seam the halves' own split keeps,
  3. THE JOINED LID — the top edge the SAME y at both halves' centers
     and at the seam column (the 15/16 lid the doubles wear — the
     single's 14/16 the halves never keep).
"""
import subprocess
import sys

from PIL import Image

ROOT = "/home/z/my-project/rust-minecraft"
CLIENT = f"{ROOT}/target/release/mc-client"

cmd = [
    CLIENT, "--headless", "--screen", "chest_pair",
    "--body", "--body-yaw", "0", "--body-pitch=-12",
    "--ui-frame", "30", "--seed", "20260214",
    "--width", "640", "--height", "360",
    "--out", f"{ROOT}/target/golden_verify/qa_pair.png",
]
subprocess.run(cmd, cwd=ROOT, check=True, capture_output=True)
img = Image.open(f"{ROOT}/target/golden_verify/qa_pair.png").convert("RGB")
px = img.load()
fails = []

def is_wood(p):
    r, g, b = p
    return 110 <= r <= 215 and 80 <= g <= 165 and b <= 120 and r - b >= 40 and r > g

def is_latch(p):
    r, g, b = p
    return 85 <= r <= 125 and abs(r - g) <= 6 and abs(g - b) <= 6

# 1. THE WOOD MASS (the pair region the calibration banks: x 210..360,
#    y 270..340 — the plateau edge the water strip fronts)
wood = sum(1 for y in range(270, 340) for x in range(210, 360) if is_wood(px[x, y]))
print(f"wood px = {wood}")
if wood < 2000:
    fails.append(f"the pair's own wood mass the region keeps ({wood} < 2000)")

# 2. THE TWO LATCHES (the neutral-gray dots the front faces speak —
#    one per half, the seam the split keeps; the scan the pair's own
#    span keeps — the x 210..340 the wood body wears, the east of it
#    the wall's own grays the region never reads)
latch_xs = sorted(
    set(x for y in range(280, 325) for x in range(210, 340) if is_latch(px[x, y]))
)
clusters, cur = [], []
for v in latch_xs:
    if cur and v - cur[-1] > 20:
        clusters.append(cur)
        cur = []
    cur.append(v)
if cur:
    clusters.append(cur)
widths = [(c[0], c[-1], c[-1] - c[0] + 1) for c in clusters]
print(f"latch clusters = {widths}")
if len(clusters) != 2:
    fails.append(f"the TWO latches the halves speak (got {len(clusters)} clusters)")
else:
    (a0, a1, _), (b0, b1, _) = widths
    if not (a0 < 282 < b0):
        fails.append("the seam the halves' own split keeps (the clusters straddle)")
    if not (30 <= b0 - a1 <= 90):
        fails.append(f"the seam gap the halves keep ({b0 - a1} px)")

# 3. THE JOINED LID (the top edge the SAME y at both halves' centers
#    and at the seam — the 15/16 the doubles wear)
def top_wood(x):
    for y in range(255, 340):
        if is_wood(px[x, y]):
            return y
    return None

tops = [top_wood(x) for x in (245, 282, 320)]
print(f"lid top y at left/seam/right = {tops}")
if any(t is None for t in tops):
    fails.append(f"the lid the three columns wear (None in {tops})")
elif max(tops) - min(tops) > 1:
    fails.append(f"the joined lid the SAME line wears (tops {tops})")

if fails:
    print("QA FAIL:")
    for f in fails:
        print("  -", f)
    sys.exit(1)
print("QA PASS: the pair's wood mass, the two latches, the joined lid all speak")

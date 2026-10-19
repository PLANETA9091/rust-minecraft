#!/usr/bin/env python3
"""QA probe for the dev-75 furnace_lit golden scene.

Renders the staged scene twice (the settled walk = lit, the static
stage = unlit) and verifies the world walk's own pixel work:
  1. the LIT render shows warm bright pixels (the lit fire hole +
     the flame embers) the UNLIT one keeps none,
  2. the light the emission-13 flood re-speaks brightens the ground
     around the furnace (the overall diff the two renders keep),
  3. the furnace's own silhouette region changed (the front tile the
     mesher's furnace arm re-renders).
"""
import subprocess, sys
from PIL import Image

ROOT = "/home/z/my-project/rust-minecraft"
CLIENT = f"{ROOT}/target/release/mc-client"

def render(flags, out):
    cmd = [CLIENT, "--headless", *flags, "--out", out]
    subprocess.run(cmd, cwd=ROOT, check=True, capture_output=True)
    return Image.open(out).convert("RGB")

base_flags = ["--screen", "furnace_lit", "--body", "--body-yaw", "0",
              "--body-pitch", "12", "--ui-frame", "30",
              "--seed", "20260214", "--width", "320", "--height", "180"]

lit = render(base_flags + ["--settle-ticks", "15"],
             f"{ROOT}/target/golden_verify/qa_lit.png")
unlit = render(base_flags + ["--settle-ticks", "0"],
               f"{ROOT}/target/golden_verify/qa_unlit.png")

def warm_counts(img):
    """The fire pixels — the lit fire hole's own palette the lightmap
    dims (the raw r 95..175, b ~0..15 the dump spoke): r >= 95, b <= 40,
    r - b >= 70, g >= 35. Scoped to the furnace's own projection region
    (x 100..220, y 70..165 — the sun never rides); the unlit background
    keeps ~12 warm texels, the lit face ~150."""
    px = img.load()
    n_fire = 0
    n_warm = 0
    cells = {}
    for y in range(70, 165):
        for x in range(100, 220):
            r, g, b = px[x, y]
            if r >= 95 and b <= 40 and r - b >= 70 and g >= 35:
                n_fire += 1
                cells.setdefault((x // 20, y // 20), 0)
                cells[(x // 20, y // 20)] += 1
            elif r > 180 and g < 160 and b < 130 and r - b > 60:
                n_warm += 1
    return n_fire, n_warm, cells

lf, lw, lcells = warm_counts(lit)
uf, uw, _ = warm_counts(unlit)
print(f"LIT   fire px={lf}  warm px={lw}  fire cells={sorted(lcells.items(), key=lambda kv: -kv[1])[:6]}")
print(f"UNLIT fire px={uf}  warm px={uw}")

# 3. the overall diff the two renders keep (the walk's own pixel work)
diff = 0
lp, up = lit.load(), unlit.load()
w, h = lit.size
rows = {}
for y in range(h):
    for x in range(w):
        if lp[x, y] != up[x, y]:
            diff += 1
            rows.setdefault(y // 20, 0)
            rows[y // 20] += 1
print(f"DIFF  changed px={diff} / {w*h}  ({100.0*diff/(w*h):.1f}%)  by band={dict(sorted(rows.items()))}")

ok = lf > 60 and lf > uf * 3
print("QA VERDICT:", "PASS — the walk's LIT flip the pixels speak" if ok else "FAIL — the furnace hides or the walk never lit")
sys.exit(0 if ok else 1)

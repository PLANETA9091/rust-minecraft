#!/usr/bin/env python3
"""THE SHEET REGION VERIFY: decode entity/chest/normal.png, print the
non-transparent content boxes per region band (the texOffs math check).
"""
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

for sheet in ("normal", "trapped", "ender", "copper"):
    data = z.read(f"assets/minecraft/textures/entity/chest/{sheet}.png")
    w, h, rgba = png_pixels(data)
    print(f"=== {sheet}: {w}x{h}")
    # content census per 1-px row: which x-ranges have alpha>0
    def alpha(x, y):
        return rgba[(y * w + x) * 4 + 3]
    # region boxes to verify (x0, y0, x1, y1, label)
    regions = [
        (14, 0, 28, 14, "lid.up"),
        (28, 0, 42, 14, "lid.down"),
        (0, 5, 14, 10, "lid.east"),
        (14, 5, 28, 10, "lid.north"),
        (28, 5, 42, 10, "lid.west"),
        (42, 5, 56, 10, "lid.south"),
        (14, 19, 28, 33, "body.up"),
        (28, 19, 42, 33, "body.down"),
        (0, 33, 14, 47, "body.east"),
        (14, 33, 28, 47, "body.north"),
        (28, 33, 42, 47, "body.west"),
        (42, 33, 56, 47, "body.south"),
        (1, 10, 3, 13, "lock.north"),
    ]
    for x0, y0, x1, y1, label in regions:
        n_opaque = sum(1 for y in range(y0, y1) for x in range(x0, x1) if alpha(x, y) > 0)
        n_total = (x1 - x0) * (y1 - y0)
        frac = n_opaque / n_total if n_total else 0
        # sample the first opaque pixel color
        col = None
        for y in range(y0, y1):
            for x in range(x0, x1):
                if alpha(x, y) > 0:
                    i = (y * w + x) * 4
                    col = tuple(rgba[i:i + 3])
                    break
            if col:
                break
        print(f"  {label}: {n_opaque}/{n_total} opaque ({frac:.2f}) first={col}")
    print()

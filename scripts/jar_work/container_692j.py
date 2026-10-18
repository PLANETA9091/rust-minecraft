#!/usr/bin/env python3
"""THE SHEET ASCII MAP: print the 64x64 sheet as quantized chars to find
every content region (the latch hunt).
"""
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

for sheet in ("normal", "ender"):
    data = z.read(f"assets/minecraft/textures/entity/chest/{sheet}.png")
    w, h, rgba = png_pixels(data)
    print(f"=== {sheet} ({w}x{h})")
    for y in range(0, h, 2):
        row = ""
        for x in range(0, w):
            i = (y * w + x) * 4
            a = rgba[i + 3]
            if a == 0:
                row += "."
            else:
                r, g, b = rgba[i], rgba[i + 1], rgba[i + 2]
                lum = (r + g + b) // 3
                if sheet == "normal":
                    row += "#" if lum > 40 else "+"
                else:
                    row += "#" if lum > 20 else "+"
        print(f"  {y:2d} {row}")
    print()

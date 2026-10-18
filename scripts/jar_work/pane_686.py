#!/usr/bin/env python3
"""THE PANE FAMILY'S OWN PROBE (the dev-58 round): the eighteen kinds
the palette banks (the glass pane + the sixteen stained cloths + the
iron bars the metal seam keeps) — javap IronBarsBlock /
StainedGlassPaneBlock / CrossCollisionBlock the 26.2 jar (pinned sha1
2dc72797) the connection law the connectsTo walk speaks, the ctor the
collision band banks, the skipRendering the same-color cull answers,
the blockstates the multipart when-rows carry, the models the
post/side/cap elements speak, the textures the alpha histograms
answer, the sounds the metal law banks, the dig rows the table
speaks.

Usage: python3 scripts/jar_work/pane_686.py <client.jar>
"""
import json
import sys

sys.path.insert(0, "scripts/jar_work")
# reuse the glass probe's helpers verbatim (parse_cp, walk_code, ...)
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

print("=== class headers")
for c in ("net.minecraft.world.level.block.IronBarsBlock",
          "net.minecraft.world.level.block.StainedGlassPaneBlock",
          "net.minecraft.world.level.block.CrossCollisionBlock"):
    print(f"--- {c}: {class_header(z, c)}")

print()
print("=== CrossCollisionBlock methods")
for m in ("<init>", "connectsTo", "updateShape", "getShape", "getCollisionShape",
          "makeShapes", "isSameInstance"):
    dump_method(z, "net.minecraft.world.level.block.CrossCollisionBlock", m, max_pc=90)

print()
print("=== IronBarsBlock methods")
for m in ("<init>", "skipRendering", "<clinit>"):
    dump_method(z, "net.minecraft.world.level.block.IronBarsBlock", m, max_pc=40)

print()
print("=== StainedGlassPaneBlock methods")
for m in ("<init>", "skipRendering"):
    dump_method(z, "net.minecraft.world.level.block.StainedGlassPaneBlock", m, max_pc=40)

print()
print("=== Blocks clinit — GLASS_PANE row + IRON_BARS row")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "Blocks.GLASS_PANE:", span=12, hits=1)
clinit_window(z, "net.minecraft.world.level.block.Blocks", "Blocks.IRON_BARS:", span=12, hits=1)

print()
print("=== blockstates")
for name in ("glass_pane", "white_stained_glass_pane", "iron_bars"):
    try:
        j = json.loads(z.read(f"assets/minecraft/blockstates/{name}.json"))
        print(f"--- blockstates/{name}.json: {json.dumps(j)[:900]}")
    except KeyError:
        print(f"--- {name}: ABSENT")

print()
print("=== models")
for m in ("template_glass_pane_post", "template_glass_pane_side",
          "glass_pane_post", "glass_pane_north", "white_stained_glass_pane_post",
          "white_stained_glass_pane_north",
          "iron_bars_post", "iron_bars_cap", "iron_bars_side"):
    try:
        j = json.loads(z.read(f"assets/minecraft/models/block/{m}.json"))
        print(f"--- models/block/{m}.json: {json.dumps(j)[:1100]}")
    except KeyError:
        print(f"--- models/block/{m}.json: ABSENT")

print()
print("=== texture alpha histograms")
for name in ("glass_pane_top", "white_stained_glass_pane_top",
             "red_stained_glass_pane_top", "iron_bars"):
    key = f"assets/minecraft/textures/block/{name}.png"
    try:
        w, h, rgba = png_pixels(z.read(key))
        print(f"--- {name}: {w}x{h} alpha {alpha_hist(rgba)}")
    except KeyError:
        print(f"--- {name}: ABSENT")

print()
print("=== tags")
for t in ("mineable/pickaxe", "mineable/pickaxe"):
    path = f"data/minecraft/tags/block/{t}.json"
    try:
        j = json.loads(z.read(path))
        vals = j.get("values", [])
        hits = [v for v in vals if "pane" in v or "bars" in v]
        print(f"--- tag {t}: {len(vals)} values; pane/bars rows: {hits}")
    except KeyError:
        print(f"--- tag {t}: ABSENT")
    break

print()
print("=== Blocks clinit — STAINED_GLASS_PANE collection")
lines = clinit_lines(z, "net.minecraft.world.level.block.Blocks")
for i, line in enumerate(lines):
    if "STAINED_GLASS_PANE" in line:
        lo = max(0, i - 10)
        print(f"--- window @ pc {lines[lo].split(':')[0]}")
        for l in lines[lo:i + 2]:
            print(l)
        break

print()
print("=== Items clinit — GLASS_PANE + IRON_BARS pairing rows")
clinit_window(z, "net.minecraft.world.item.Items", "Items.GLASS_PANE:", span=5, hits=1)
clinit_window(z, "net.minecraft.world.item.Items", "Items.IRON_BARS:", span=5, hits=1)

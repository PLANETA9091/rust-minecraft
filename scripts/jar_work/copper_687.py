#!/usr/bin/env python3
"""THE COPPER BARS FAMILY'S OWN PROBE (the dev-59 round): the eight
oxidization rungs the palette banks (copper_bars + exposed/weathered/
oxidized + the four waxed twins) — javap CopperBarsBlock the 26.2 jar
(pinned sha1 2dc72797) the class header the IronBarsBlock super the
instanceof arm's own verdict banks, the Blocks clinit the
WeatheringCopperCollection walk the eight register rows the exact order
speaks, the BARS tag the nine members the pair-cull arm the palette
arms, the blockstates the waxed kinds the BASE models re-wear (the
texture-sharing law the atlas tile count guards), the textures the
alpha histograms answer, the sounds.json the block.copper.* events
carry, the Items clinit the pairing rows bank.

Usage: python3 scripts/jar_work/copper_687.py <client.jar>
"""
import json
import sys

sys.path.insert(0, "scripts/jar_work")
# reuse the glass probe's helpers verbatim (parse_cp, walk_code,
# methods_of, png_pixels, alpha_hist, clinit_lines, clinit_window —
# the whole helper section up to the main body's zip open)
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

RUNGS = ["copper_bars", "exposed_copper_bars", "weathered_copper_bars",
         "oxidized_copper_bars", "waxed_copper_bars",
         "waxed_exposed_copper_bars", "waxed_weathered_copper_bars",
         "waxed_oxidized_copper_bars"]

print("=== class headers")
for c in ("net.minecraft.world.level.block.CopperBarsBlock",
          "net.minecraft.world.level.block.IronBarsBlock"):
    try:
        print(f"--- {c}: {class_header(z, c)}")
    except Exception as e:
        print(f"--- {c}: ERR {e}")

print()
print("=== CopperBarsBlock methods")
for m in ("<init>", "skipRendering", "attachsTo"):
    try:
        dump_method(z, "net.minecraft.world.level.block.CopperBarsBlock", m, max_pc=40)
    except Exception as e:
        print(f"--- {m}: ERR {e}")

print()
print("=== BARS tag membership")
try:
    tag = json.loads(z.read("data/minecraft/tags/block/bars.json"))
    print("tags/block/bars.json:", json.dumps(tag, indent=1))
except Exception as e:
    print("ERR", e)

print()
print("=== Blocks clinit: the COPPER_BARS..WAXED_OXIDIZED_COPPER_BARS rows")
lines = clinit_lines(z, "net.minecraft.world.level.block.Blocks")
idx = [i for i, l in enumerate(lines) if "COPPER_BARS" in l]
lo = max(0, idx[0] - 2 if idx else 0)
hi = min(len(lines), (idx[-1] + 2) if idx else 0)
for l in lines[lo:hi]:
    print(l)
print("  (IRON_BARS row pc for the ordering walk:)")
for i, l in enumerate(lines):
    if "IRON_BARS" in l:
        print(lines[i])
        break

print()
print("=== blockstates: the waxed kinds' model refs (the BASE re-wear law)")
for name in RUNGS:
    bs = json.loads(z.read(f"assets/minecraft/blockstates/{name}.json"))
    models = []
    for part in bs["multipart"]:
        apply = part["apply"]
        for a in (apply if isinstance(apply, list) else [apply]):
            models.append(a["model"].replace("minecraft:block/", ""))
    print(f"  {name}: {sorted(set(models))}")

print()
print("=== textures: alpha histograms of the four masters")
for stem in ("copper_bars", "exposed_copper_bars", "weathered_copper_bars",
             "oxidized_copper_bars", "copper_bars_cap" ):
    try:
        w, h, px = png_pixels(z.read(f"assets/minecraft/textures/block/{stem}.png"))
        hist = {}
        for p in px:
            hist[p[3]] = hist.get(p[3], 0) + 1
        print(f"  {stem}: {w}x{h} alpha {dict(sorted(hist.items()))}")
    except Exception as e:
        print(f"  {stem}: ERR {e}")

print()
print("=== sounds.json: the block.copper.* + block.iron_bars.* events")
snd = json.loads(z.read("assets/minecraft/sounds.json"))
for ev in sorted(snd):
    if ev.startswith("block.copper.") or ev.startswith("block.iron."):
        print(f"  {ev}")

print()
print("=== Items clinit: the copper bars pairing rows")
ilines = clinit_lines(z, "net.minecraft.world.item.Items")
for i, l in enumerate(ilines):
    if "Items.COPPER_BARS" in l and ("registerBlock" in l or "putstatic" in l or "getstatic" in l):
        for x in ilines[max(0, i - 1):i + 3]:
            print(x)
        break

print()
print("=== mineable/pickaxe membership (the dig law)")
mt = json.loads(z.read("data/minecraft/tags/blocks/mineable/pickaxe.json"))
vals = mt.get("values", [])
print(f"  pickaxe total {len(vals)}; copper bars rows:",
      [v for v in vals if "copper_bars" in v])

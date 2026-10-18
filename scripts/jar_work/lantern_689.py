#!/usr/bin/env python3
"""THE LANTERN FAMILY'S OWN PROBE (the dev-61 round): the ten kinds
the palette banks (lantern the iron seam + soul_lantern the nether
row + the eight copper rungs the weathering walk serves) — javap
LanternBlock the 26.2 jar (pinned sha1 2dc72797) the class header
the HANGING property banks, the SHAPES the standing/hanging boxes
speak, the getStateForPlacement the clicked-face walk answers, the
Blocks clinit the LANTERN + SOUL_LANTERN rows + the COPPER_LANTERN
collection the properties chains bank (the lightLevel lambdas the
emission constants carry), the blockstates the hanging variants
carry, the models the element boxes speak, the textures the alpha
histograms answer, the sounds.json the block.lantern.* events carry.

Usage: python3 scripts/jar_work/lantern_689.py <client.jar>
"""
import json
import sys

sys.path.insert(0, "scripts/jar_work")
# reuse the glass probe's helpers verbatim
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

KINDS = ["lantern", "soul_lantern", "copper_lantern", "exposed_copper_lantern",
         "weathered_copper_lantern", "oxidized_copper_lantern",
         "waxed_copper_lantern", "waxed_exposed_copper_lantern",
         "waxed_weathered_copper_lantern", "waxed_oxidized_copper_lantern"]

print("=== class headers")
for c in ("net.minecraft.world.level.block.LanternBlock",
          "net.minecraft.world.level.block.BaseTorchBlock",
          "net.minecraft.world.level.block.Block"):
    try:
        print(f"--- {c}: {class_header(z, c)}")
    except Exception as e:
        print(f"--- {c}: ERR {e}")

print()
print("=== LanternBlock fields + methods")
ms = methods_of(z, "net.minecraft.world.level.block.LanternBlock")
if ms is None:
    print("!! LanternBlock not found")
else:
    for sig in sorted(ms):
        print(f"  {sig}")

print()
print("=== LanternBlock methods of interest")
for m in ("<init>", "getShape", "getStateForPlacement", "getConnectedDirection", "<clinit>"):
    dump_method(z, "net.minecraft.world.level.block.LanternBlock", m, max_pc=80)

print()
print("=== Blocks clinit: LANTERN row")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "LANTERN;putstatic", hits=2)
print()
print("=== Blocks clinit: SOUL_LANTERN row")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "SOUL_LANTERN;putstatic", hits=1)

print()
print("=== the lantern lambdas' own bodies (lightLevel + properties)")
zf = z.read("net/minecraft/world/level/block/Blocks.class")
msB = methods_of(z, "net.minecraft.world.level.block.Blocks")
for sig, (cp, body) in sorted(msB.items()):
    if "lambda$static$" in sig and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        joined = " | ".join(lines)
        if "LanternBlock" in joined or "SoundType.LANTERN" in joined or "SOUL" in joined.upper():
            print(f"--- {sig}")
            for l in lines[:30]:
                print("   ", l)
            print()

print()
print("=== blockstates: the ten kinds' hanging variants")
for name in KINDS:
    try:
        bs = json.loads(z.read(f"assets/minecraft/blockstates/{name}.json"))
        vs = bs.get("variants", {})
        summary = {k: (v if isinstance(v, str) else
                       [a.get("model", "") + f" x={a.get('x', 0)} y={a.get('y', 0)}"
                        for a in (v if isinstance(v, list) else [v])])
                   for k, v in vs.items()}
        print(f"  {name}: {json.dumps(summary)}")
    except Exception as e:
        print(f"  {name}: ERR {e}")

print()
print("=== models: the template pair + the copper masters")
for m in ("block/template_lantern", "block/template_hanging_lantern",
          "block/lantern", "block/soul_lantern", "block/copper_lantern",
          "block/exposed_copper_lantern"):
    try:
        print(f"--- {m}.json")
        print(z.read(f"assets/minecraft/models/{m}.json").decode())
    except Exception as e:
        print(f"  ERR {e}")

print()
print("=== textures: alpha histograms")
for stem in ("lantern", "soul_lantern", "copper_lantern", "exposed_copper_lantern",
             "weathered_copper_lantern", "oxidized_copper_lantern"):
    try:
        w, h, rgba = png_pixels(z.read(f"assets/minecraft/textures/block/{stem}.png"))
        print(f"  {stem}: {w}x{h} alpha {alpha_hist(rgba)}")
    except Exception as e:
        print(f"  {stem}: ERR {e}")

print()
print("=== sounds.json: the block.lantern.* events")
try:
    snd = json.loads(z.read("assets/minecraft/sounds.json"))
    for ev in sorted(snd):
        if ev.startswith("block.lantern.") or ev.startswith("block.copper.") and "lantern" in ev:
            print(f"  {ev}")
except Exception as e:
    print("ERR", e)

print()
print("=== loot tables: the survival drops")
for name in KINDS:
    try:
        lt = json.loads(z.read(f"data/minecraft/loot_table/blocks/{name}.json"))
        pools = lt.get("pools", [])
        entries = []
        for pool in pools:
            for e in pool.get("entries", []):
                entries.append(e.get("name", "?"))
        print(f"  {name}: entries={entries}")
    except Exception as e:
        print(f"  {name}: ERR {e}")

print()
print("=== tags: mineable/pickaxe membership")
try:
    tag = json.loads(z.read("data/minecraft/tags/block/mineable/pickaxe.json"))
    members = [v.replace("minecraft:", "") for v in tag.get("values", [])]
    hits = [m for m in members if "lantern" in m]
    print(f"  pickaxe lantern rows: {hits}")
except Exception as e:
    print("ERR", e)

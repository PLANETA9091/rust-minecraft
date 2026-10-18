#!/usr/bin/env python3
"""THE CHAIN FAMILY'S OWN PROBE (the dev-60 round): the nine kinds the
palette banks (iron_chain the metal seam + the eight copper rungs the
weathering walk serves) — javap ChainBlock the 26.2 jar (pinned sha1
2dc72797) the class header the RotatedPillar super the axis vocabulary
banks, the SHAPES map the collision bands speaks, the getStateForPlacement
the clicked-face axis walk answers, the Blocks clinit the IRON_CHAIN row
pc 12360-12398 + the COPPER_CHAIN collection pc 12401-12427 the
properties chains bank, the blockstates the axis variants carry, the
waxed kinds the BASE models re-wear, the textures the alpha histograms
answer, the sounds.json the block.chain.* events carry.

Usage: python3 scripts/jar_work/chain_688.py <client.jar>
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

RUNGS = ["iron_chain", "copper_chain", "exposed_copper_chain",
         "weathered_copper_chain", "oxidized_copper_chain",
         "waxed_copper_chain", "waxed_exposed_copper_chain",
         "waxed_weathered_copper_chain", "waxed_oxidized_copper_chain"]

print("=== class headers")
for c in ("net.minecraft.world.level.block.ChainBlock",
          "net.minecraft.world.level.block.RotatedPillarBlock"):
    try:
        print(f"--- {c}: {class_header(z, c)}")
    except Exception as e:
        print(f"--- {c}: ERR {e}")

print()
print("=== ChainBlock methods")
for m in ("<init>", "getShape", "getStateForPlacement", "<clinit>"):
    try:
        dump_method(z, "net.minecraft.world.level.block.ChainBlock", m, max_pc=60)
    except Exception as e:
        print(f"--- {m}: ERR {e}")

print()
print("=== Blocks clinit: IRON_CHAIN row (pc 12360-12398) window")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "Blocks.IRON_CHAIN:", span=14, hits=1)

print()
print("=== the copper chain collection's own lambdas (SoundType + properties)")
zf = z.read("net/minecraft/world/level/block/Blocks.class")
ms = methods_of(z, "net.minecraft.world.level.block.Blocks")
for sig, (cp, body) in sorted(ms.items()):
    if "lambda$static$" in sig and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        joined = " | ".join(lines)
        if "SoundType.CHAIN" in joined:
            print(f"--- {sig}")
            for l in lines[:24]:
                print("   ", l)
            print()

print()
print("=== blockstates: the nine kinds' variant models")
for name in RUNGS:
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
print("=== textures: alpha histograms")
for stem in ("iron_chain", "copper_chain", "exposed_copper_chain",
             "weathered_copper_chain", "oxidized_copper_chain"):
    try:
        w, h, rgba = png_pixels(z.read(f"assets/minecraft/textures/block/{stem}.png"))
        print(f"  {stem}: {w}x{h} alpha {alpha_hist(rgba)}")
    except Exception as e:
        print(f"  {stem}: ERR {e}")

print()
print("=== sounds.json: the block.chain.* events")
try:
    snd = json.loads(z.read("assets/minecraft/sounds.json"))
    for ev in sorted(snd):
        if ev.startswith("block.chain."):
            print(f"  {ev}")
except Exception as e:
    print("ERR", e)

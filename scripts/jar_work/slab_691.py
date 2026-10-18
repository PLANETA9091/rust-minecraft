#!/usr/bin/env python3
"""THE SLAB FAMILY'S OWN PROBE (the dev-63 round): the kinds the palette
banks (the wooden ladder + the stone walk + the copper rungs the cut_copper
weathering ladder serves) — javap SlabBlock the 26.2 jar (pinned sha1
2dc72797) the TYPE vocabulary the class header banks, the getStateForPlacement
the clicked-half walk answers, the Blocks clinit the OAK_SLAB + STONE_SLAB
rows + the copper collection the properties chains bank, the blockstates the
type=bottom/top/double variant keys carry, the models the slab trio speak,
the loot tables the drops bank, the tags the membership answer.

Usage: python3 scripts/jar_work/slab_691.py <client.jar>
"""
import json
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

print("=== the full slab census from blockstates dir")
names = sorted(n for n in z.namelist()
               if n.startswith("assets/minecraft/blockstates/") and n.endswith("_slab.json"))
slabs = [n.split("/")[-1][:-len(".json")] for n in names]
print(f"  {len(slabs)} slabs:")
print("  ", slabs)

print()
print("=== class headers")
for c in ("net.minecraft.world.level.block.SlabBlock",
          "net.minecraft.world.level.block.WeatheringCopperSlabBlock",
          "net.minecraft.world.level.block.state.properties.SlabType",
          "net.minecraft.world.level.block.state.properties.BlockStateProperties"):
    try:
        print(f"--- {c}: {class_header(z, c)}")
    except Exception as e:
        print(f"--- {c}: ERR {e}")

print()
print("=== SlabBlock fields + methods")
ms = methods_of(z, "net.minecraft.world.level.block.SlabBlock")
if ms is None:
    print("!! SlabBlock not found")
else:
    for sig in sorted(ms):
        print(f"  {sig}")

print()
print("=== SlabBlock methods of interest")
for m in ("getShape", "getStateForPlacement", "updateShape", "use", "<clinit>",
          "getCollisionShape", "canPlace"):
    dump_method(z, "net.minecraft.world.level.block.SlabBlock", m, max_pc=300)

print()
print("=== SlabType enum constants")
try:
    lines = clinit_lines(z, "net.minecraft.world.level.block.state.properties.SlabType")
    for l in lines:
        print("   ", l)
except Exception as e:
    print("ERR", e)

print()
print("=== Blocks clinit: OAK_SLAB row")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "OAK_SLAB;putstatic", hits=1)
print()
print("=== Blocks clinit: STONE_SLAB row")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "STONE_SLAB;putstatic", hits=1)
print()
print("=== Blocks clinit: CUT_COPPER_SLAB row")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "CUT_COPPER_SLAB;putstatic", hits=1)
print()
print("=== Blocks clinit: COBBLESTONE_SLAB row")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "COBBLESTONE_SLAB;putstatic", hits=1)

print()
print("=== the slab lambdas' own bodies (properties)")
zf = z.read("net/minecraft/world/level/block/Blocks.class")
msB = methods_of(z, "net.minecraft.world.level.block.Blocks")
shown = 0
for sig, (cp, body) in sorted(msB.items()):
    if "lambda$static$" in sig and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        joined = " | ".join(lines)
        if "SlabBlock" in joined and shown < 6:
            print(f"--- {sig}")
            for l in lines[:24]:
                print("   ", l)
            print()
            shown += 1

print()
print("=== blockstates: variant-key census (sample)")
SAMPLE = ["oak_slab", "stone_slab", "cobblestone_slab", "cut_copper_slab",
          "waxed_cut_copper_slab", "deepslate_tile_slab", "cinnabar_slab",
          "resin_brick_slab", "smooth_quartz_slab", "sandstone_slab",
          "crimson_slab", "bamboo_mosaic_slab", "smooth_stone_slab"]
for name in SAMPLE:
    try:
        bs = json.loads(z.read(f"assets/minecraft/blockstates/{name}.json"))
        vs = bs.get("variants", {})
        print(f"--- {name}: {len(vs)} variants")
        for k in sorted(vs):
            v = vs[k]
            model = v if isinstance(v, str) else (v[0].get("model", "") if isinstance(v, list) and v else str(v))
            rot = ""
            if isinstance(v, list) and v:
                x, y = v[0].get("x", 0), v[0].get("y", 0)
                rot = f" x={x} y={y}"
            print(f"    {k} -> {model}{rot}")
    except Exception as e:
        print(f"  {name}: ERR {e}")

print()
print("=== models: the slab trio + masters")
for m in ("block/slab", "block/slab_top", "block/oak_slab", "block/stone_slab",
          "block/cut_copper_slab", "block/waxed_cut_copper_slab",
          "block/sandstone_slab", "block/smooth_stone_slab",
          "block/cobblestone_slab", "block/deepslate_tile_slab"):
    try:
        print(f"--- {m}.json")
        print(z.read(f"assets/minecraft/models/{m}.json").decode())
    except Exception as e:
        print(f"  ERR {e}")

print()
print("=== textures: slab own pngs? (should be none)")
own = [n for n in z.namelist() if n.startswith("assets/minecraft/textures/block/") and "slab" in n]
print("  slab textures:", own if own else "NONE — slabs borrow the parent's textures")

print()
print("=== loot tables: the drops")
for name in ["oak_slab", "stone_slab", "cut_copper_slab", "waxed_cut_copper_slab",
             "cobblestone_slab", "sandstone_slab", "deepslate_tile_slab",
             "cinnabar_slab", "resin_brick_slab", "smooth_stone_slab"]:
    try:
        lt = json.loads(z.read(f"data/minecraft/loot_table/blocks/{name}.json"))
        entries = []
        conds = []
        for pool in lt.get("pools", []):
            for e in pool.get("entries", []):
                entries.append(e.get("name", "?"))
                for c in e.get("conditions", []):
                    conds.append(c.get("condition", "?"))
        print(f"  {name}: entries={entries} conds={conds}")
    except Exception as e:
        print(f"  {name}: ERR {e}")

print()
print("=== tags: mineable axe/pickaxe/shovel membership")
for tool in ("axe", "pickaxe", "shovel"):
    try:
        tag = json.loads(z.read(f"data/minecraft/tags/block/mineable/{tool}.json"))
        members = [v.replace("minecraft:", "") for v in tag.get("values", [])]
        hits = [m for m in members if m.endswith("_slab")]
        print(f"  {tool}: {len(hits)} slabs")
        print("   ", hits)
    except Exception as e:
        print(f"  {tool}: ERR {e}")

print()
print("=== requiresCorrectToolForDrops: which slab rows carry it")
print("  (see the clinit windows above — the pickaxe rows the tool gate keeps)")

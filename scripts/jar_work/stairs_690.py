#!/usr/bin/env python3
"""THE STAIRS FAMILY'S OWN PROBE (the dev-62 round): the sixty-four kinds
the palette banks (the wooden ladder the oak..warped rows wear + the
stone walk the cobble..quartz rows keep + the copper rungs the cut_copper
weathering ladder serves) — javap StairsBlock the 26.2 jar (pinned sha1
2dc72797) the FACING/HALF/SHAPE vocabulary the class header banks, the
SHAPE_BY_STATE the voxel unions speak, the getStairsShape the neighbor
walk answers, the Blocks clinit the OAK_STAIRS + STONE_STAIRS rows +
the stone collection the properties chains bank, the blockstates the
facing/half/shape variant keys carry, the models the template trio
speak, the loot tables the self drops bank, the tags the axe/pickaxe
membership answer.

Usage: python3 scripts/jar_work/stairs_690.py <client.jar>
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

WOODEN = ["oak", "spruce", "birch", "jungle", "acacia", "dark_oak", "pale_oak",
          "mangrove", "cherry", "bamboo", "bamboo_mosaic", "crimson", "warped"]
COPPER = ["cut_copper", "exposed_cut_copper", "weathered_cut_copper",
          "oxidized_cut_copper", "waxed_cut_copper", "waxed_exposed_cut_copper",
          "waxed_weathered_cut_copper", "waxed_oxidized_cut_copper"]

print("=== class headers")
for c in ("net.minecraft.world.level.block.StairBlock",
          "net.minecraft.world.level.block.WeatheringCopperStairBlock",
          "net.minecraft.world.level.block.state.properties.StairsShape",
          "net.minecraft.world.level.block.Block"):
    try:
        print(f"--- {c}: {class_header(z, c)}")
    except Exception as e:
        print(f"--- {c}: ERR {e}")

print()
print("=== StairBlock fields + methods")
ms = methods_of(z, "net.minecraft.world.level.block.StairBlock")
if ms is None:
    print("!! StairBlock not found")
else:
    for sig in sorted(ms):
        print(f"  {sig}")

print()
print("=== StairBlock methods of interest")
for m in ("<init>", "getShape", "getStateForPlacement", "updateShape", "<clinit>", "<clinit>"):
    dump_method(z, "net.minecraft.world.level.block.StairBlock", m, max_pc=260)

print()
print("=== the static initializer shapes (clinit fields)")
try:
    lines = clinit_lines(z, "net.minecraft.world.level.block.StairBlock")
    for l in lines:
        print("   ", l)
except Exception as e:
    print("ERR", e)

print()
print("=== StairsShape enum constants")
try:
    lines = clinit_lines(z, "net.minecraft.world.level.block.state.properties.StairsShape")
    for l in lines:
        print("   ", l)
except Exception as e:
    print("ERR", e)

print()
print("=== Blocks clinit: OAK_STAIRS row")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "OAK_STAIRS;putstatic", hits=1)
print()
print("=== Blocks clinit: STONE_STAIRS row")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "STONE_STAIRS;putstatic", hits=1)
print()
print("=== Blocks clinit: CUT_COPPER_STAIRS row")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "CUT_COPPER_STAIRS;putstatic", hits=1)

print()
print("=== the stairs lambdas' own bodies (properties)")
zf = z.read("net/minecraft/world/level/block/Blocks.class")
msB = methods_of(z, "net.minecraft.world.level.block.Blocks")
shown = 0
for sig, (cp, body) in sorted(msB.items()):
    if "lambda$static$" in sig and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        joined = " | ".join(lines)
        if "StairBlock" in joined and shown < 4:
            print(f"--- {sig}")
            for l in lines[:20]:
                print("   ", l)
            print()
            shown += 1

print()
print("=== blockstates: variant-key census (sample of families)")
SAMPLE = ["oak_stairs", "stone_stairs", "cut_copper_stairs", "waxed_cut_copper_stairs",
          "deepslate_tile_stairs", "cinnabar_stairs", "resin_brick_stairs",
          "smooth_quartz_stairs", "sandstone_stairs", "crimson_stairs", "bamboo_mosaic_stairs"]
keysets = {}
for name in SAMPLE:
    try:
        bs = json.loads(z.read(f"assets/minecraft/blockstates/{name}.json"))
        vs = bs.get("variants", {})
        keys = sorted({tuple(sorted(k.split(","))) for k in vs}) if vs else []
        mk = {}
        for k, v in vs.items():
            model = v if isinstance(v, str) else (v[0].get("model", "") if isinstance(v, list) and v else str(v))
            mk[k] = model + (f" x={v[0].get('x',0)} y={v[0].get('y',0)}" if isinstance(v, list) and v else "")
        print(f"--- {name}: {len(vs)} variants")
        for k in sorted(mk):
            print(f"    {k} -> {mk[k]}")
    except Exception as e:
        print(f"  {name}: ERR {e}")

print()
print("=== models: the template trio + masters")
for m in ("block/template_stairs", "block/template_stairs_inner", "block/template_stairs_outer",
          "block/oak_stairs", "block/stone_stairs", "block/cut_copper_stairs",
          "block/waxed_cut_copper_stairs", "block/deepslate_tile_stairs",
          "block/sandstone_stairs", "block/smooth_sandstone_stairs"):
    try:
        print(f"--- {m}.json")
        print(z.read(f"assets/minecraft/models/{m}.json").decode())
    except Exception as e:
        print(f"  ERR {e}")

print()
print("=== textures: stairs own pngs? (should be none)")
own = [n for n in z.namelist() if n.startswith("assets/minecraft/textures/block/") and "stairs" in n]
print("  stairs textures:", own if own else "NONE — stairs borrow the parent's textures")

print()
print("=== loot tables: the self drops")
for name in ["oak_stairs", "stone_stairs", "cut_copper_stairs", "waxed_cut_copper_stairs"]:
    try:
        lt = json.loads(z.read(f"data/minecraft/loot_table/blocks/{name}.json"))
        entries = []
        for pool in lt.get("pools", []):
            for e in pool.get("entries", []):
                entries.append(e.get("name", "?"))
        print(f"  {name}: entries={entries}")
    except Exception as e:
        print(f"  {name}: ERR {e}")

print()
print("=== tags: mineable axe/pickaxe/shovel membership")
for tool in ("axe", "pickaxe", "shovel"):
    try:
        tag = json.loads(z.read(f"data/minecraft/tags/block/mineable/{tool}.json"))
        members = [v.replace("minecraft:", "") for v in tag.get("values", [])]
        hits = [m for m in members if m.endswith("_stairs")]
        print(f"  {tool}: {len(hits)} stairs")
        if tool != "pickaxe":
            print("   ", hits)
        else:
            print("   sample:", hits[:12])
    except Exception as e:
        print(f"  {tool}: ERR {e}")

print()
print("=== sounds: what voices the stairs rows reuse (wood/stone/copper)")
try:
    snd = json.loads(z.read("assets/minecraft/sounds.json"))
    for ev in sorted(snd):
        if ev in ("block.wood.break", "block.stone.break", "block.copper.break",
                  "block.deepslate.break", "block.nether_wood.break"):
            print(f"  {ev}")
except Exception as e:
    print("ERR", e)

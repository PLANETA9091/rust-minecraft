#!/usr/bin/env python3
"""THE STAIRS PROBE PART 2 (the dev-62 round): the neighbor-walk
bytecode (getStairsShape + canTakeShape + isStairs), the Blocks
lambdas the sixty-four property chains bank (sound/strength/tool/base),
the clinit rows the STAIRS constants reference, the rotation-table
identity across all 64 kinds, the WeatheringCopperStairBlock header,
the template trio's element boxes.

Usage: python3 scripts/jar_work/stairs_690b.py <client.jar>
"""
import json
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

print("=== the neighbor walk: getStairsShape + canTakeShape + isStairs")
for m in ("getStairsShape", "canTakeShape", "isStairs"):
    dump_method(z, "net.minecraft.world.level.block.StairBlock", m, max_pc=300)

print()
print("=== WeatheringCopperStairBlock methods")
ms = methods_of(z, "net.minecraft.world.level.block.WeatheringCopperStairBlock")
if ms:
    for sig in sorted(ms):
        print(f"  {sig}")

print()
print("=== Blocks clinit: the STAIRS rows (windows around each constant)")
clinit = clinit_lines(z, "net.minecraft.world.level.block.Blocks")
all_lines = list(clinit)
hits = {}
for i, l in enumerate(all_lines):
    if "STAIRS" in l and "getstatic" in l and "putstatic" not in l:
        # field ref like getstatic .../Blocks.OAK_STAIRS : ... — collect index
        name = l.split("Blocks.")[1].split(":")[0].strip() if "Blocks." in l else l
        hits.setdefault(name, i)
print(f"  constants referenced in clinit: {len(hits)}")
for name, idx in sorted(hits.items()):
    print(f"--- {name} @ line {idx}")
    for l in all_lines[max(0, idx - 6):idx + 6]:
        print("   ", l)

print()
print("=== Blocks lambdas: the stair property chains (sound/strength/base)")
zf = z.read("net/minecraft/world/level/block/Blocks.class")
msB = methods_of(z, "net.minecraft.world.level.block.Blocks")
stair_lambdas = {}
for sig, (cp, body) in sorted(msB.items()):
    if "lambda$static$" in sig and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        joined = " | ".join(lines)
        if "StairBlock" in joined:
            stair_lambdas[sig] = lines
            print(f"--- {sig}")
            for l in lines[:26]:
                print("   ", l)
            print()
print(f"  TOTAL stair lambdas: {len(stair_lambdas)}")

print()
print("=== indy wiring: which lambda each stairs row uses")
for i, l in enumerate(all_lines):
    if "STAIRS" in l and "indy" in l:
        pass
# walk indy instructions near STAIRS constants: the lambda$static$N appears in bootstrap args
# simpler: grep clinit lines for both STAIRS constant and lambda references
for name, idx in sorted(hits.items())[:6]:
    print(f"--- {name} context (±10)")
    for l in all_lines[max(0, idx - 10):idx + 10]:
        print("   ", l)

print()
print("=== rotation-table identity across all 64 blockstates")
names = [n.split("/")[-1][:-5] for n in z.namelist()
         if n.startswith("assets/minecraft/blockstates/") and n.endswith(".json")
         and n.split("/")[-1][:-5].endswith("_stairs")]
REF = None
odd = []
for name in sorted(names):
    bs = json.loads(z.read(f"assets/minecraft/blockstates/{name}.json"))
    vs = bs.get("variants", {})
    table = {}
    for k, v in vs.items():
        if isinstance(v, list):
            v = v[0]
        model = v.get("model", "")
        table[k] = (model.split("/")[-1].replace(name, "{KIND}"),
                    v.get("x", 0), v.get("y", 0))
    if REF is None:
        REF = (name, table)
        print(f"  reference: {name} ({len(table)} variants)")
    elif table != REF[1]:
        odd.append(name)
print(f"  kinds checked: {len(names)}; identical to reference: {len(names) - len(odd)}")
print(f"  ODD kinds: {odd if odd else 'NONE — the table is family-uniform'}")

print()
print("=== the template trio element boxes")
for m in ("block/template_stairs", "block/template_stairs_inner", "block/template_stairs_outer"):
    try:
        d = json.loads(z.read(f"assets/minecraft/models/{m}.json"))
        print(f"--- {m}:")
        for el in d.get("elements", []):
            print("   ", el.get("from"), el.get("to"), "faces:", sorted(el.get("faces", {}).keys()))
    except Exception as e:
        print(f"  ERR {e}")

print()
print("=== master stair models (kind -> textures + parent)")
for m in ("block/oak_stairs", "block/oak_stairs_inner", "block/oak_stairs_outer",
          "block/stone_stairs", "block/cut_copper_stairs", "block/waxed_cut_copper_stairs",
          "block/bamboo_mosaic_stairs", "block/crimson_stairs", "block/deepslate_tile_stairs",
          "block/smooth_sandstone_stairs", "block/cinnabar_stairs", "block/resin_brick_stairs"):
    try:
        d = json.loads(z.read(f"assets/minecraft/models/{m}.json"))
        print(f"  {m}: parent={d.get('parent', '-')} textures={d.get('textures', {})}")
    except Exception as e:
        print(f"  {m}: ERR {e}")

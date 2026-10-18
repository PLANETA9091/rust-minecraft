#!/usr/bin/env python3
"""THE CONTAINER PROBE'S THIRD PASS (dev-64): Block.boxZ decode, the Blocks
clinit rows, chest blockstate model refs, models/textures, loot, sounds, tags,
and the client's own right-click block-use path census.
"""
import json
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

print("=== Block.boxZ signature + body")
dump_method(z, "net.minecraft.world.level.block.Block", "boxZ", max_pc=160)
ms = methods_of(z, "net.minecraft.world.level.block.Block")
if ms:
    for sig in sorted(ms):
        if "box" in sig.split("(")[0]:
            print(f"  SIG: {sig}")

print()
print("=== chest/trapped/ender/barrel blockstate model refs")
for n in ("chest", "trapped_chest", "ender_chest", "barrel"):
    bs = json.loads(z.read(f"assets/minecraft/blockstates/{n}.json"))
    print(f"  {n}: {json.dumps(bs.get('variants', bs))[:600]}")

print()
print("=== models")
for m in ("chest", "barrel", "barrel_open"):
    for p in (f"assets/minecraft/models/block/{m}.json",
              f"assets/minecraft/models/block/{m}_open.json"):
        try:
            print(f"  {p}: {z.read(p).decode()[:500]}")
        except KeyError:
            print(f"  {p}: ABSENT")

print()
print("=== textures present")
for t in ("entity/chest/normal", "entity/chest/trapped", "entity/chest/ender_left",
          "entity/chest/ender_right", "entity/chest/normal_left", "entity/chest/normal_right",
          "block/barrel_side", "block/barrel_top", "block/barrel_bottom", "block/barrel_open"):
    p = f"assets/minecraft/textures/{t}.png"
    print(f"  {p}: {'PRESENT' if p in set(z.namelist()) else 'ABSENT'}")

print()
print("=== loot tables")
for n in ("chest", "trapped_chest", "ender_chest", "barrel"):
    p = f"data/minecraft/loot_table/blocks/{n}.json"
    try:
        print(f"  {n}: {z.read(p).decode()[:240]}")
    except KeyError:
        print(f"  {n}: ABSENT")

print()
print("=== sounds.json events")
sj = json.loads(z.read("assets/minecraft/sounds.json"))
for k in sorted(sj):
    if "chest" in k or "barrel" in k:
        print(f"  {k}: subtitle={sj[k].get('subtitle', '-')}")

print()
print("=== tags")
for t in ("mineable/axe", "needs_stone_tool", "needs_iron_tool"):
    p = f"data/minecraft/tags/block/{t}.json"
    try:
        lst = json.loads(z.read(p)).get("values", [])
        hits = [v for v in lst if "chest" in v or "barrel" in v]
        print(f"  {t}: {len(lst)} total; container hits: {hits}")
    except KeyError:
        print(f"  {t}: ABSENT")

print()
print("=== Blocks clinit rows around CHEST/TRAPPED/ENDER/BARREL")
code = method_code(z, "net.minecraft.world.level.block.Blocks", "<clinit>")
lines = walk_code(*struct.unpack_from(">I", code, 0), code[8:]) if False else None
# use the helper bank's row scan if present
try:
    src = clinit_source(z, "net.minecraft.world.level.block.Blocks")
except Exception as e:
    src = None
if src:
    keep = [ln for ln in src.splitlines() if any(k in ln for k in
            ("CHEST", "BARREL"))]
    for ln in keep:
        print("  ", ln.strip())
else:
    print("  (clinit_source helper absent)")

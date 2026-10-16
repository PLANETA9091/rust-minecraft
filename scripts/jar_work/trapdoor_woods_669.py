#!/usr/bin/env python3
"""THE TRAPDOOR WOODS' OWN PROBE (the autopush-669 round): the
BlockSetType clinit's trapdoorOpen/trapdoorClose SoundEvent rows the
per-wood voices bank — the WOODEN set type's pair the eight plain
woods speak, the CHERRY/BAMBOO/NETHER full-ctor pairs the cherry/
bamboo/crimson/warped rows wear. Plus the blockstates + models +
items rows the ladder's palette rows resolve (the eleven {wood}_
trapdoor.json maps the per-row arms read, the {wood}_trapdoor.png
masters the tile bank gains, the Items clinit rows the BlockItem
pairings speak).

Usage: python3 scripts/jar_work/trapdoor_woods_669.py <client.jar>
"""
import json
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"

# reuse the probe's classfile walker
src = open("scripts/jar_work/trapdoor_668.py").read()
head = src[: src.index("z = zipfile.ZipFile(JAR)")]
ns = {}
exec(head, ns)

z = zipfile.ZipFile(JAR)

print("=== BlockSetType fields (the sound event rows the clinit banks)")
d = z.read("net/minecraft/world/level/block/state/properties/BlockSetType.class")
cp, _ = ns["parse_cp"](d)
ms = ns["methods_of"](z, "net.minecraft.world.level.block.state.properties.BlockSetType")
for sig, (cp2, body) in ms.items():
    if body and sig.startswith("<clinit>"):
        clen = struct.unpack_from(">I", body, 4)[0]
        lines = ns["walk_code"](cp2, body[8 : 8 + clen])
        # print the ldc rows naming sound events + the putstatic fields
        for i, line in enumerate(lines):
            if "putstatic" in line or ("ldc" in line and ("trapdoor" in line or "wood" in line)):
                print(line)

print()
print("=== the eleven wooden trapdoor blockstates (the per-row arms read)")
woods = [
    "spruce", "birch", "jungle", "acacia", "dark_oak", "mangrove",
    "cherry", "pale_oak", "bamboo", "crimson", "warped",
]
for w in woods:
    try:
        bs = json.loads(z.read(f"assets/minecraft/blockstates/{w}_trapdoor.json"))
        keys = sorted(bs.get("variants", {}).keys())
        closed = bs["variants"].get("facing=east,half=bottom,open=false", {}).get("model")
        opened = bs["variants"].get("facing=north,half=bottom,open=true", {}).get("model")
        print(f"{w}_trapdoor: rows={len(keys)} closed={closed} open={opened}")
    except KeyError:
        print(f"{w}_trapdoor: ABSENT")

print()
print("=== the models' parent chains (the per-stem bakes the mesher reads)")
for w in woods:
    rows = []
    for suffix in ("bottom", "top", "open"):
        try:
            m = json.loads(z.read(f"assets/minecraft/models/block/{w}_trapdoor_{suffix}.json"))
            rows.append(f"{suffix}:{m.get('parent')}")
        except KeyError:
            rows.append(f"{suffix}:ABSENT")
    print(f"{w}_trapdoor: " + " ".join(rows))

print()
print("=== the textures' dims (the tile bank's masters)")
for w in woods:
    try:
        raw = z.read(f"assets/minecraft/textures/block/{w}_trapdoor.png")
        width, height = struct.unpack_from(">II", raw, 16)
        print(f"{w}_trapdoor: {width}x{height} colortype {raw[25]}")
    except KeyError:
        print(f"{w}_trapdoor: ABSENT")

print()
print("=== the Items clinit rows (the registerBlock pairings)")
d = z.read("net/minecraft/world/item/Items.class")
cp, _ = ns["parse_cp"](d)
ms = ns["methods_of"](z, "net.minecraft.world.item.Items")
for sig, (cp2, body) in ms.items():
    if body and sig.startswith("<clinit>"):
        clen = struct.unpack_from(">I", body, 4)[0]
        lines = ns["walk_code"](cp2, body[8 : 8 + clen])
        for w in woods:
            needle = f"{w.upper()}_TRAPDOOR"
            for i, line in enumerate(lines):
                if needle in line and "putstatic" in line and "BlockItemIds" not in line:
                    print(" ".join(lines[i : i + 4]))
                    break

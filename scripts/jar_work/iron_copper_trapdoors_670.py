#!/usr/bin/env python3
"""THE IRON/COPPER TRAPDOORS' OWN PROBE (the autopush-670 round): the
non-wooden set types the hand-refusal law banks — the IRON and COPPER
set types' canOpenByHand ctor rows the gate verdicts wear, the
blockstates + models + textures the ladder's palette rows resolve,
the sounds.json the iron/copper trapdoor voice pairs carry, the Items
clinit rows the registerBlock pairings speak.

Usage: python3 scripts/jar_work/iron_copper_trapdoors_670.py <client.jar>
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

STEMS = [
    "iron",
    "copper", "exposed_copper", "weathered_copper", "oxidized_copper",
    "waxed_copper", "waxed_exposed_copper", "waxed_weathered_copper",
    "waxed_oxidized_copper",
]

print("=== the iron/copper trapdoor blockstates (the per-row arms read)")
for s in STEMS:
    try:
        bs = json.loads(z.read(f"assets/minecraft/blockstates/{s}_trapdoor.json"))
        keys = sorted(bs.get("variants", {}).keys())
        closed = bs["variants"].get("facing=east,half=bottom,open=false", {}).get("model")
        opened = bs["variants"].get("facing=north,half=bottom,open=true", {}).get("model")
        print(f"{s}_trapdoor: rows={len(keys)} closed={closed} open={opened}")
    except KeyError:
        print(f"{s}_trapdoor: ABSENT")

print()
print("=== the models' parent chains (the per-stem bakes the mesher reads)")
for s in STEMS:
    rows = []
    for suffix in ("bottom", "top", "open"):
        try:
            m = json.loads(z.read(f"assets/minecraft/models/block/{s}_trapdoor_{suffix}.json"))
            rows.append(f"{suffix}:{m.get('parent')}")
        except KeyError:
            rows.append(f"{suffix}:ABSENT")
    print(f"{s}_trapdoor: " + " ".join(rows))

print()
print("=== the textures' dims (the tile bank's masters)")
for s in STEMS:
    try:
        raw = z.read(f"assets/minecraft/textures/block/{s}_trapdoor.png")
        width, height = struct.unpack_from(">II", raw, 16)
        print(f"{s}_trapdoor: {width}x{height} colortype {raw[25]}")
    except KeyError:
        print(f"{s}_trapdoor: ABSENT")

print()
print("=== BlockSetType clinit rows (the ctor split the IRON/COPPER rows wear)")
d = z.read("net/minecraft/world/level/block/state/properties/BlockSetType.class")
cp, _ = ns["parse_cp"](d)
ms = ns["methods_of"](z, "net.minecraft.world.level.block.state.properties.BlockSetType")
for sig, (cp2, body) in ms.items():
    if body and sig.startswith("<clinit>"):
        clen = struct.unpack_from(">I", body, 4)[0]
        lines = ns["walk_code"](cp2, body[8 : 8 + clen])
        for i, line in enumerate(lines):
            if "putstatic" in line and ("IRON" in line or "COPPER" in line or "WOODEN" in line or "NETHER" in line):
                lo = max(0, i - 30)
                print("  ...")
                for j in range(lo, min(len(lines), i + 2)):
                    print("  " + lines[j])
        # the canOpenByHand method the verdicts read
        for msig, (cp3, mbody) in ms.items():
            if msig.startswith("canOpenByHand") and mbody:
                mlen = struct.unpack_from(">I", mbody, 4)[0]
                print("canOpenByHand bytecode:")
                for line in ns["walk_code"](cp3, mbody[8 : 8 + mlen]):
                    print("  " + line)

print()
print("=== the ctor's own putfield order (the boolean seats the clinit args fill)")
for sig, (cp2, body) in ms.items():
    if body and sig.startswith("<init>"):
        clen = struct.unpack_from(">I", body, 4)[0]
        for line in ns["walk_code"](cp2, body[8 : 8 + clen]):
            if "putfield" in line:
                print("  " + line)

print()
print("=== the WOODEN/NETHER rows' booleans (the comparison law)")
for sig, (cp2, body) in ms.items():
    if body and sig.startswith("<clinit>"):
        clen = struct.unpack_from(">I", body, 4)[0]
        lines = ns["walk_code"](cp2, body[8 : 8 + clen])
        for i, line in enumerate(lines):
            if "putstatic" in line and ("WOODEN" in line or "NETHER_WOOD" in line):
                for j in range(max(0, i - 24), i + 1):
                    print("  " + lines[j])
                print("  ---")

print()
print("=== sounds.json the trapdoor voice rows (the iron/copper pairs)")
sounds = json.loads(z.read("assets/minecraft/sounds.json"))
for key in sorted(sounds.keys()):
    if "trapdoor" in key:
        print(f"{key}")

print()
print("=== the Items clinit rows (the registerBlock pairings)")
d = z.read("net/minecraft/world/item/Items.class")
cp, _ = ns["parse_cp"](d)
ms = ns["methods_of"](z, "net.minecraft.world.item.Items")
for sig, (cp2, body) in ms.items():
    if body and sig.startswith("<clinit>"):
        clen = struct.unpack_from(">I", body, 4)[0]
        lines = ns["walk_code"](cp2, body[8 : 8 + clen])
        for s in STEMS:
            needle = f"{s.upper()}_TRAPDOOR"
            for i, line in enumerate(lines):
                if needle in line and "putstatic" in line and "BlockItemIds" not in line:
                    print(" ".join(lines[i : i + 4]))
                    break
            else:
                print(f"{needle}: (no putstatic row found)")

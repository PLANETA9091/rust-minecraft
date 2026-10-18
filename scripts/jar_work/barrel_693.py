#!/usr/bin/env python3
"""THE BARREL PROBE (dev-65): the Blocks clinit row, the dig/sound rows,
the textures, the loot, the blockstate x/y map, the models.
"""
import json
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)
names = set(z.namelist())

def short(l):
    l = l.strip()
    for pat in ("net/minecraft/world/level/block/state/BlockBehaviour$Properties.",
                "net/minecraft/world/level/material/MapColor.",
                "net/minecraft/world/level/block/state/properties/NoteBlockInstrument.",
                "net/minecraft/world/level/block/SoundType.",
                "invokevirtual net/minecraft/world/level/block/state/BlockBehaviour$Properties.",
                "getstatic ", "ldc2_w ", "ldc_w ", "ldc ", "invokestatic ",
                "invokevirtual ", "putstatic ", "invokedynamic "):
        l = l.replace(pat, "")
    return l

print("=== Blocks clinit BARREL row")
ms = methods_of(z, "net.minecraft.world.level.block.Blocks")
cp, body = next(b for s, b in ms.items() if "<clinit>" in s)
clen = struct.unpack_from(">I", body, 4)[0]
text = [str(l) for l in walk_code(cp, body[8:8 + clen])]
for i, t in enumerate(text):
    if "Blocks.BARREL" in t:
        seg = " ~ ".join(short(x) for x in text[i-30:i+2])
        print(f"pc{i}: {seg[-1200:]}")

print()
print("=== textures")
for t in ("block/barrel_side", "block/barrel_top", "block/barrel_bottom",
          "block/barrel_top_open"):
    p = f"assets/minecraft/textures/{t}.png"
    print(f"  {t}: {'PRESENT' if p in names else 'ABSENT'}")

print()
print("=== barrel blockstate (full)")
bs = json.loads(z.read("assets/minecraft/blockstates/barrel.json"))
print(json.dumps(bs, indent=1)[:1200])

print()
print("=== barrel models")
for m in ("barrel", "barrel_open"):
    p = f"assets/minecraft/models/block/{m}.json"
    print(f"  {m}: {z.read(p).decode()[:340]}")

print()
print("=== loot")
print("  ", z.read("data/minecraft/loot_table/blocks/barrel.json").decode()[:200])

print()
print("=== mineable/axe tag barrel:")
for p in ("data/minecraft/tags/block/mineable/axe.json",
          "data/minecraft/tags/blocks/mineable/axe.json"):
    try:
        lst = json.loads(z.read(p)).get("values", [])
        print(f"  {p}: barrel={'minecraft:barrel' in lst}")
        break
    except KeyError:
        continue

print()
print("=== BarrelBlock methods")
mb = methods_of(z, "net.minecraft.world.level.block.BarrelBlock")
if mb:
    for sig in sorted(mb):
        print(f"  {sig[:130]}")

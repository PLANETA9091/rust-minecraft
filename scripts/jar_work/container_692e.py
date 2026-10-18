#!/usr/bin/env python3
"""THE CHEST FAMILY'S FIFTH PASS (dev-64): find ChestModel/ChestRenderer,
dump the Boxes (texOffs + box dims), the Blocks clinit rows via methods_of.
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

print("=== find chest model class")
cand = sorted(n for n in z.namelist() if "Chest" in n and n.endswith(".class"))
for c in cand[:20]:
    print("  ", c)

print()
print("=== Blocks clinit rows via methods_of")
ms = methods_of(z, "net.minecraft.world.level.block.Blocks")
clinit = None
for sig, (cp, body) in ms.items():
    if "<clinit>" in sig:
        clinit = (cp, body)
        break
cp, body = clinit
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8:8 + clen]
lines = walk_code(cp, code)
text = [str(l) for l in lines]
# print rows around each *_CHEST / BARREL putstatic region
targets = ("CHEST", "BARREL")
for i, t in enumerate(text):
    if any(f"Blocks.{k}" in t for k in ("CHEST", "TRAPPED_CHEST", "ENDER_CHEST")) or \
       ("COPPER_CHEST" in t and "putstatic" in t):
        seg = " ~ ".join(x.strip() for x in text[i:i+16])
        print(f"pc{i}: {seg[:700]}")
        print("-----")

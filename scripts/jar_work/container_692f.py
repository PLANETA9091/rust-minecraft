#!/usr/bin/env python3
"""THE CHEST FAMILY'S SIXTH PASS: the Blocks clinit rows BEFORE the
putstatic (the properties chains), the ChestModel boxes, the ChestRenderer.
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

ms = methods_of(z, "net.minecraft.world.level.block.Blocks")
cp, body = next(b for s, b in ms.items() if "<clinit>" in s)
clen = struct.unpack_from(">I", body, 4)[0]
text = [str(l) for l in walk_code(cp, body[8:8 + clen])]

for i, t in enumerate(text):
    if "Blocks.CHEST" in t or "Blocks.TRAPPED_CHEST" in t or "Blocks.ENDER_CHEST" in t \
       or "Blocks.COPPER_CHEST" in t:
        seg = " ~ ".join(x.strip() for x in text[i-22:i+2])
        print(f"pc{i}: {seg[-900:]}")
        print("-----")

print()
print("=== ChestModel (object/chest) methods")
CM = "net.minecraft.client.model.object.chest.ChestModel"
msm = methods_of(z, CM)
if msm:
    for sig in sorted(msm):
        print(f"  SIG: {sig}")
    for m in ("<init>", "createBodyLayer", "texOffs", "<clinit>"):
        dump_method(z, CM, m, max_pc=500)

print()
print("=== ChestRenderer render")
CR = "net.minecraft.client.renderer.blockentity.ChestRenderer"
dump_method(z, CR, "render", max_pc=500)

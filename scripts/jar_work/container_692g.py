#!/usr/bin/env python3
"""SEVENTH PASS: full property chains (wider windows), the layer definitions
(box dims), the ender lightLevel lambda, ChestRenderer methods list.
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

def short(l):
    l = l.strip()
    for pat in ("net/minecraft/world/level/block/state/BlockBehaviour$Properties.",
                "net/minecraft/world/level/block/state/BlockBehaviour$",
                "invokevirtual net/minecraft/world/level/block/state/BlockBehaviour$Properties.",
                "getstatic ", "ldc2_w ", "ldc_w ", "ldc ", "invokestatic ",
                "invokevirtual ", "putstatic ", "invokedynamic "):
        l = l.replace(pat, "")
    return l

for i, t in enumerate(text):
    if "Blocks.CHEST" in t or "Blocks.TRAPPED_CHEST" in t or "Blocks.ENDER_CHEST" in t \
       or "Blocks.COPPER_CHEST" in t:
        seg = " ~ ".join(short(x) for x in text[i-40:i+2])
        print(f"pc{i}: {seg[-1500:]}")
        print("=====")

print()
print("=== ender lightLevel lambda (the indy near 13996)")
# find the lambda method: scan Blocks methods for lambda names near chest rows
for sig in sorted(ms):
    if "lambda" in sig:
        print(f"  LAMBDA SIG: {sig[:150]}")

print()
CM = "net.minecraft.client.model.object.chest.ChestModel"
for m in ("createSingleBodyLayer", "createDoubleBodyLeftLayer", "createDoubleBodyRightLayer"):
    dump_method(z, CM, m, max_pc=600)

CR = "net.minecraft.client.renderer.blockentity.ChestRenderer"
mcr = methods_of(z, CR)
if mcr:
    for sig in sorted(mcr):
        print(f"  RENDERER SIG: {sig[:150]}")

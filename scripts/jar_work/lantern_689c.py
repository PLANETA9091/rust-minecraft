#!/usr/bin/env python3
"""THE LANTERN PROBE'S OWN THIRD WALK (the dev-61 round): the lightLevel
lambdas the indy bootstraps serve — the Blocks$1 holder the bootstrap
methods bank, every (BlockState)I lambda the constant bodies carry,
the WeatheringCopperCollection the lantern PREFIXES order speaks.

Usage: python3 scripts/jar_work/lantern_689c.py <client.jar>
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

print("=== Blocks$1 (the lightLevel bootstrap holder) — every method")
ms = methods_of(z, "net.minecraft.world.level.block.Blocks$1") or {}
for sig, (cp, body) in sorted(ms.items()):
    if not body:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    print(f"--- {sig}")
    for l in walk_code(cp, code)[:20]:
        print("   ", l)

print()
print("=== every Blocks lambda with an I return and a tiny body (the light walk)")
msB = methods_of(z, "net.minecraft.world.level.block.Blocks") or {}
for sig, (cp, body) in sorted(msB.items()):
    if "lambda$static$" not in sig or not body:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    lines = walk_code(cp, code)
    if len(lines) <= 4 and ("ireturn" in " | ".join(lines)):
        print(f"--- {sig}: {' | '.join(lines)}")

print()
print("=== WeatheringCopperCollection clinit (the lantern PREFIXES)")
msW = methods_of(z, "net.minecraft.world.level.block.WeatheringCopperCollection") or {}
for sig, (cp, body) in sorted(msW.items()):
    if sig.startswith("<clinit>") and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        for l in walk_code(cp, code):
            print("   ", l)

print()
print("=== the copper lantern indy chain: Blocks clinit pc 24709-24740 full window")
lines = clinit_lines(z, "net.minecraft.world.level.block.Blocks") or []
for i, line in enumerate(lines):
    if line.startswith("24709:") or line.startswith("24712:"):
        for l in lines[i:i + 14]:
            print("   ", l)
        break

print()
print("=== BlockBehaviour$Properties.lightLevel indy bootstraps — the lambda NAMES")
# the indy at pc 24631 (LANTERN lightLevel) references a bootstrap method;
# dump the indy name+bootstrap via the constant pool around it
d = z.read("net/minecraft/world/level/block/Blocks.class")
cp, p = parse_cp(d)
# find lambda methods whose body contains 'int 15' or 'int 10' as their only const
for sig, (c2, body) in sorted(msB.items()):
    if "lambda$static$" not in sig or not body:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    lines2 = walk_code(c2, code)
    joined = " | ".join(lines2)
    if "int 15" in joined or "int 10" in joined:
        if len(lines2) <= 6:
            print(f"--- {sig}: {joined}")

print()
print("=== LanternBlock.SHAPE_* constants cross-check (the collision walk)")
msL = methods_of(z, "net.minecraft.world.level.block.LanternBlock") or {}
for sig, (cp, body) in sorted(msL.items()):
    if sig.startswith("<clinit>") and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        for l in walk_code(cp, code):
            print("   ", l)

#!/usr/bin/env python3
"""THE CONTAINER PROBE'S SECOND PASS (dev-64): the chest SHAPE tables, the
getStateForPlacement tails, the ender walk, the Blocks clinit rows, the
blockstate model refs, the models/textures, the loot, the sounds, the tags.
"""
import json
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)
CB = "net.minecraft.world.level.block.ChestBlock"
EB = "net.minecraft.world.level.block.EnderChestBlock"

print("=== ChestBlock getStateForPlacement tail (pc 115+)")
dump_method(z, CB, "getStateForPlacement", max_pc=400)

print("=== ChestBlock <clinit> (SHAPES)")
dump_method(z, CB, "<clinit>", max_pc=300)

print("=== ChestBlock getShape")
dump_method(z, CB, "getShape", max_pc=300)

print("=== ChestBlock useWithoutItem")
dump_method(z, CB, "useWithoutItem", max_pc=220)

print("=== EnderChestBlock getStateForPlacement + use")
dump_method(z, EB, "getStateForPlacement", max_pc=200)
dump_method(z, EB, "useWithoutItem", max_pc=220)

print("=== EnderChestBlock methods")
ms = methods_of(z, EB)
if ms:
    for sig in sorted(ms):
        print(f"  {sig}")

print()
print("=== Blocks clinit chest/barrel rows")
try:
    out = blocks_clinit_rows(z, ["CHEST", "TRAPPED_CHEST", "ENDER_CHEST", "BARREL"])
except NameError:
    # manual scan: find the register rows around the constants
    out = None
    code = method_code(z, "net.minecraft.world.level.block.Blocks", "<clinit>")
    print(f"  (blocks_clinit_rows helper absent — raw clinit {len(code)} bytes)")
if out:
    for r in out:
        print(r)

#!/usr/bin/env python3
"""THE CONTAINER FAMILY'S OWN PROBE (the dev-64 round): the chest trio +
barrel kinds the palette banks — javap ChestBlock / TrappedChestBlock /
EnderChestBlock / BarrelBlock the 26.2 jar (pinned sha1 2dc72797) the
FACING/TYPE/OPEN vocabularies the classes bank, the Blocks clinit rows the
properties chains carry, the blockstates the variant keys speak, the models
the chest/barrel parents answer, the loot tables the drops bank, the sounds
the block.chest.*/barrel.* events wear.

Usage: python3 scripts/jar_work/container_692.py <client.jar>
"""
import json
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

print("=== the container census (blockstates)")
for n in ("chest", "trapped_chest", "ender_chest", "barrel",
          "chest_left", "chest_right", "trapped_chest_left", "trapped_chest_right"):
    p = f"assets/minecraft/blockstates/{n}.json"
    if p in set(z.namelist()):
        bs = json.loads(z.read(p))
        keys = list(bs.get("variants", {}).keys())
        print(f"  {n}: {len(keys)} variant keys -> {keys[:14]}")
    else:
        print(f"  {n}: ABSENT")

print()
print("=== class headers")
for c in ("net.minecraft.world.level.block.ChestBlock",
          "net.minecraft.world.level.block.TrappedChestBlock",
          "net.minecraft.world.level.block.EnderChestBlock",
          "net.minecraft.world.level.block.BarrelBlock",
          "net.minecraft.world.level.block.AbstractChestBlock",
          "net.minecraft.world.level.block.state.properties.ChestType",
          "net.minecraft.world.level.block.state.properties.WoodType"):
    try:
        print(f"--- {c}: {class_header(z, c)}")
    except Exception as e:
        print(f"--- {c}: ERR {e}")

print()
print("=== ChestBlock fields + methods (signatures only)")
ms = methods_of(z, "net.minecraft.world.level.block.ChestBlock")
if ms is None:
    print("!! ChestBlock not found")
else:
    for sig in sorted(ms):
        print(f"  {sig}")

print()
print("=== BarrelBlock methods of interest")
for m in ("getStateForPlacement", "updateShape", "useWithoutItem", "use", "<clinit>",
          "getShape", "getCollisionShape", "onRemove"):
    dump_method(z, "net.minecraft.world.level.block.BarrelBlock", m, max_pc=220)

print()
print("=== ChestBlock methods of interest")
for m in ("getStateForPlacement", "updateShape", "getShape", "<clinit>"):
    dump_method(z, "net.minecraft.world.level.block.ChestBlock", m, max_pc=260)

print()
print("=== EnderChestBlock methods of interest")
for m in ("getStateForPlacement", "useWithoutItem", "use"):
    dump_method(z, "net.minecraft.world.level.block.EnderChestBlock", m, max_pc=200)

print()
print("=== Blocks clinit rows (chest/trapped/ender/barrel)")
bc = "net.minecraft.world.level.block.Blocks"
src = javap_source(z, bc) if "javap_source" in dir() else None
# fall back to the clinit scan the helpers bank
try:
    code = method_code(z, bc, "<clinit>")
    text = disasm_text(code) if "disasm_text" in dir() else None
except Exception as e:
    print(f"  clinit scan ERR {e}")

for kw in ("CHEST", "BARREL", "ENDER_CHEST", "TRAPPED"):
    try:
        rows = clinit_rows(z, bc, kw) if "clinit_rows" in dir() else None
    except Exception as e:
        rows = None
    if rows:
        for r in rows[:4]:
            print(f"  [{kw}] {r}")
    else:
        print(f"  [{kw}] (clinit_rows helper absent — skip)")

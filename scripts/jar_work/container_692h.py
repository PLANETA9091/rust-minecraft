#!/usr/bin/env python3
"""EIGHTH PASS: the copper chest collection indies decode, the ender
lightLevel lambda, the chest sounds, the mineable tag, the partner walks.
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

# the copper chest collection lambdas: the FOUR indies at pc 29450/29455/29460/29465
print("=== copper chest collection lambda ctors")
ms = methods_of(z, "net.minecraft.world.level.block.Blocks")
for sig in sorted(ms):
    if "lambda$static$" in sig:
        num = int(sig.split("$static$")[1].split("(")[0])
        if 1580 <= num <= 1600 or 1570 <= num <= 1590:
            pass
# simpler: find lambdas returning Block/Properties near COPPER_CHEST numbers by content scan
import re
for sig, (cp, body) in sorted(ms.items(), key=lambda kv: int(kv[0].split("$static$")[1].split("(")[0]) if "lambda$static$" in kv[0] else 0):
    if "lambda$static$" not in sig:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    try:
        lines = [str(l) for l in walk_code(cp, code)]
    except Exception:
        continue
    joined = " ".join(lines)
    if "Chest" in joined or "chest" in joined:
        print(f"--- {sig}")
        for l in lines[:24]:
            print("   ", l.strip()[:160])

print()
print("=== ender light lambda: which lambda$static$N is (LBlockState;)I with bipush 7")
for sig, (cp, body) in sorted(ms.items(), key=lambda kv: int(kv[0].split("$static$")[1].split("(")[0]) if "lambda$static$" in kv[0] else 0):
    if "lambda$static$" not in sig:
        continue
    if sig.startswith("lambda$static$") is False:
        continue
    if "(Lnet/minecraft/world/level/block/state/BlockState;)I" not in sig:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    lines = [str(l) for l in walk_code(cp, body[8:8 + clen])]
    print(f"--- {sig}: {' ~ '.join(l.strip()[:80] for l in lines[:8])}")

print()
print("=== chest sounds events")
# sounds are in .ogg bank — the SOUND EVENTS live in the sounds json under assets? 26.2 uses references
sn = sorted(n for n in z.namelist() if "sound" in n.lower() and n.endswith(".json"))
for s in sn[:6]:
    print("  ", s)

print()
print("=== mineable/axe tag")
lst = None
for p in ("data/minecraft/tags/block/mineable/axe.json",
          "data/minecraft/tags/blocks/mineable/axe.json"):
    try:
        lst = __import__("json").loads(z.read(p)).get("values", [])
        break
    except KeyError:
        continue
if lst is not None:
    hits = [v for v in lst if "chest" in v or "barrel" in v]
    print(f"  {len(lst)} members; hits: {hits}")
else:
    print("  tag ABSENT")

print()
print("=== candidatePartnerFacing + getChestType + getConnectedDirection walks")
CB = "net.minecraft.world.level.block.ChestBlock"
for m in ("candidatePartnerFacing", "getChestType", "getConnectedDirection",
          "chestCanConnectTo", "isBlockedChestByBlock"):
    dump_method(z, CB, m, max_pc=260)

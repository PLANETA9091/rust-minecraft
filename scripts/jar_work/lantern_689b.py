#!/usr/bin/env python3
"""THE LANTERN PROBE'S OWN SECOND WALK (the dev-61 round): the clinit
rows the first walk's needle missed — the LANTERN + SOUL_LANTERN
registrations the putstatic windows speak, the copper collection's
own row the WeatheringCopper.Blocks walk banks, the lightLevel
lambdas the (BlockState)I bodies the emission constants carry, the
canSurvive the connected-direction verdict answers, the updateShape
the hanging break walk speaks, the sounds the sounds.njb/archive the
block.lantern events carry.

Usage: python3 scripts/jar_work/lantern_689b.py <client.jar>
"""
import json
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

CLINIT = clinit_lines(z, "net.minecraft.world.level.block.Blocks")
print("=== Blocks clinit: every LANTERN-bearing line with windows")
for i, line in enumerate(CLINIT):
    if "LANTERN" in line:
        lo = max(0, i - 6)
        hi = min(len(CLINIT), i + 4)
        print(f"--- window @ line {i}")
        for l in CLINIT[lo:hi]:
            print("   ", l)
        print()

print()
print("=== LanternBlock: canSurvive + updateShape + isPathfindable")
for m in ("canSurvive", "updateShape", "isPathfindable", "getFluidState"):
    dump_method(z, "net.minecraft.world.level.block.LanternBlock", m, max_pc=110)

print()
print("=== the lightLevel lambdas: (BlockState)I bodies near the lantern rows")
msB = methods_of(z, "net.minecraft.world.level.block.Blocks")
hits = []
for sig, (cp, body) in sorted(msB.items()):
    if "lambda$static$" in sig and body and sig.endswith("(Lnet/minecraft/world/level/block/state/BlockBehaviour$BlockState;)I"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        joined = " | ".join(lines)
        for want, name in (("bipush 10", "10"), ("bipush 15", "15"), ("iconst_5", "5")):
            if joined.strip() == f"0: aload_0 | 2: {want} | 4: ireturn":
                hits.append((sig, name))
print(f"  plain-constant light lambdas: {len(hits)}")
for sig, v in hits:
    print(f"   {sig} -> {v}")

print()
print("=== bootstrap lambdas the lightLevel indy calls (lantern rows)")
# the indy in lambda$static$308 (copper lantern) references a bootstrap
# method handle; find BlockBehaviour$Properties$1 or the Blocks$1 bootstraps
for cls in ("net.minecraft.world.level.block.Blocks$1",
            "net.minecraft.world.level.block.state.BlockBehaviour$Properties$1"):
    try:
        d = z.read(f"{cls.replace('.', '/')}.class")
        print(f"  {cls} exists ({len(d)} bytes)")
    except KeyError:
        print(f"  {cls} ABSENT")

print()
print("=== the lanterns tag + the copper collection registration window")
try:
    tag = json.loads(z.read("data/minecraft/tags/block/lanterns.json"))
    print("  block/lanterns:", json.dumps(tag))
except Exception as e:
    print("  ERR", e)

print()
print("=== sounds archive entries")
for n in z.namelist():
    if "sounds" in n and n.endswith(".json"):
        print("  ", n)
try:
    snd = json.loads(z.read("assets/minecraft/sounds.njb"))
    keys = sorted(snd) if isinstance(snd, dict) else []
    hits = [k for k in keys if "lantern" in k]
    print("  sounds.njb lantern events:", hits)
except Exception as e:
    print("  sounds.njb ERR", e)

print()
print("=== SoundType.LANTERN row (the pitch/volume constants the client's voices wear)")
try:
    d = dump_method(z, "net.minecraft.world.level.block.SoundType", "<clinit>", max_pc=0)
except Exception as e:
    print("  ERR", e)
msS = methods_of(z, "net.minecraft.world.level.block.SoundType")
if msS:
    for sig, (cp, body) in msS.items():
        if not sig.startswith("<clinit>") or not body:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        for i, line in enumerate(lines):
            if "SoundType" in line and "LANTERN" in line:
                lo = max(0, i - 3)
                hi = min(len(lines), i + 14)
                print(f"--- SoundType clinit window @ LANTERN (line {i})")
                for l in lines[lo:hi]:
                    print("   ", l)
                break

print()
print("=== weathering copper: the collection registration walk (LANTERN window)")
msW = methods_of(z, "net.minecraft.world.level.block.WeatheringCopper$WeatheringCopperCollection") or {}
for sig, (cp, body) in sorted(msW.items()):
    if sig.startswith("<clinit>") and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        for l in walk_code(cp, code):
            if "antern" in l:
                print("   ", l)

print()
print("=== lantern_hanging + copper models the waxed twins re-wear")
for m in ("block/lantern_hanging", "block/soul_lantern_hanging",
          "block/weathered_copper_lantern", "block/oxidized_copper_lantern"):
    try:
        j = json.loads(z.read(f"assets/minecraft/models/{m}.json"))
        print(f"--- {m}: parent={j.get('parent')} textures={j.get('textures')}")
    except Exception as e:
        print(f"  {m}: ERR {e}")

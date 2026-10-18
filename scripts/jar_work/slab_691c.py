#!/usr/bin/env python3
"""THE SLAB FAMILY'S OWN THIRD PROBE (the dev-63 round): the full clinit
rows the 61 slab constants wear (the property chains the strength + sound +
requiresCorrectToolForDrops + legacy-copy wiring speaks), the copper
collection's four indy lambdas the weathering ladder serves, the FIXED
model census the bottom/side/top texture refs bank.

Usage: python3 scripts/jar_work/slab_691c.py <client.jar>
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

print("=== Blocks clinit: the FULL row per slab constant")
clinit = clinit_lines(z, "net.minecraft.world.level.block.Blocks")
all_lines = list(clinit)
# find putstatic indices for every Blocks.* constant to delimit rows
putstatic_idx = []
for i, l in enumerate(all_lines):
    if "putstatic net/minecraft/world/level/block/Blocks." in l:
        name = l.split("Blocks.")[1].split(":")[0].strip()
        putstatic_idx.append((i, name))

slab_names = set()
for i, l in enumerate(all_lines):
    if "_SLAB" in l and "putstatic" in l:
        name = l.split("Blocks.")[1].split(":")[0].strip()
        slab_names.add(name)

rows = {}
for j, (i, name) in enumerate(putstatic_idx):
    if name in slab_names:
        start = putstatic_idx[j - 1][0] + 1 if j > 0 else 0
        rows[name] = all_lines[start:i + 1]

print(f"  slab rows: {len(rows)}")
for name in sorted(rows):
    print(f"--- {name} ({len(rows[name])} lines)")
    for l in rows[name]:
        print("   ", l)
    print()

print()
print("=== the copper collection: CUT_COPPER_SLAB row + the lambdas it wires")
for name in ("CUT_COPPER_SLAB",):
    for j, (i, nm) in enumerate(putstatic_idx):
        if nm == name:
            start = putstatic_idx[j - 1][0] + 1
            print(f"--- {name} row:")
            for l in all_lines[start:i + 1]:
                print("   ", l)
# the lambdas: find indy targets — dump every lambda whose body mentions
# WeatheringCopperSlabBlock OR the slab sound/strength trio near copper
zf = z.read("net/minecraft/world/level/block/Blocks.class")
msB = methods_of(z, "net.minecraft.world.level.block.Blocks")
print()
for sig, (cp, body) in sorted(msB.items()):
    if "lambda$static$" in sig and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        joined = " | ".join(lines)
        if ("WeatheringCopperSlabBlock" in joined or "WeatheringCopperStairBlock" in joined
                or ("SoundType.COPPER" in joined or "SoundType.WEATHERING_COPPER" in joined
                    or "SoundType.WAXED_COPPER" in joined)):
            print(f"--- {sig}")
            for l in lines[:30]:
                print("   ", l)
            print()

print()
print("=== WeatheringCopperSlabBlock methods")
ms = methods_of(z, "net.minecraft.world.level.block.WeatheringCopperSlabBlock")
if ms:
    for sig in sorted(ms):
        print(f"  {sig}")

print()
print("=== registerSlab method (the legacy slab derivation)")
dump_method(z, "net.minecraft.world.level.block.Blocks", "registerSlab", max_pc=120)

print()
print("=== FIXED model census (bottom/side/top refs per kind)")
names = sorted(n.split("/")[-1][:-5] for n in z.namelist()
               if n.startswith("assets/minecraft/blockstates/") and n.endswith(".json")
               and n.split("/")[-1][:-5].endswith("_slab"))
print(f"  {len(names)} kinds")
GROUPS = {}
for name in names:
    bs = json.loads(z.read(f"assets/minecraft/blockstates/{name}.json"))
    vs = bs.get("variants", {})
    bm = vs["type=bottom"]
    tm = vs["type=top"]
    dm = vs["type=double"]
    if isinstance(bm, list):
        bm = bm[0]
    if isinstance(tm, list):
        tm = tm[0]
    if isinstance(dm, list):
        dm = dm[0]
    bmodel = bm["model"].replace("minecraft:", "")
    tmodel = tm["model"].replace("minecraft:", "")
    dmodel = dm["model"].replace("minecraft:", "")
    def tex_of(m):
        d = json.loads(z.read(f"assets/minecraft/models/{m}.json"))
        return d.get("parent", "").replace("minecraft:", ""), dict(sorted(d.get("textures", {}).items()))
    bp, btex = tex_of(bmodel)
    tp, ttex = tex_of(tmodel)
    key = (bp, tp, json.dumps(btex, sort_keys=True))
    GROUPS.setdefault(key, []).append((name, bmodel, tmodel, dmodel, btex))

for (bp, tp, btexj), kinds in sorted(GROUPS.items()):
    print(f"--- parentB={bp} parentT={tp} tex={btexj}")
    for name, bmodel, tmodel, dmodel, btex in kinds:
        print(f"    {name}: bottom={bmodel} top={tmodel} double={dmodel}")

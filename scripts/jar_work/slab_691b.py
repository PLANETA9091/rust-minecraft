#!/usr/bin/env python3
"""THE SLAB FAMILY'S OWN SECOND PROBE (the dev-63 round): the Blocks clinit
rows the slab constants register (the register/registerLegacySlab wiring the
strength + sound + requiresCorrectToolForDrops chains carry), the
canBeReplaced the doubling walk speaks, the full 68-kind model census the
bottom/side/top + double refs bank.

Usage: python3 scripts/jar_work/slab_691b.py <client.jar>
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

print("=== SlabBlock.canBeReplaced (the doubling walk)")
dump_method(z, "net.minecraft.world.level.block.SlabBlock", "canBeReplaced", max_pc=200)

print()
print("=== Blocks clinit: all register method names mentioned (census)")
clinit = clinit_lines(z, "net.minecraft.world.level.block.Blocks")
regs = {}
for l in clinit:
    if "invokestatic net/minecraft/world/level/block/Blocks.register" in l:
        m = l.split("Blocks.")[1].split(":")[0].strip()
        regs[m] = regs.get(m, 0) + 1
for k, v in sorted(regs.items()):
    print(f"  {k}: {v}")

print()
print("=== Blocks clinit: the SLAB rows (windows around each putstatic)")
all_lines = list(clinit)
hits = {}
for i, l in enumerate(all_lines):
    if "_SLAB" in l and "putstatic" in l:
        name = l.split("Blocks.")[1].split(":")[0].strip() if "Blocks." in l else l
        hits[name] = i
print(f"  slab constants: {len(hits)}")
for name, idx in sorted(hits.items()):
    print(f"--- {name} @ line {idx}")
    for l in all_lines[max(0, idx - 7):idx + 1]:
        print("   ", l)
    print()

print()
print("=== the slab lambdas (property chains): every lambda the SlabBlock news")
zf = z.read("net/minecraft/world/level/block/Blocks.class")
msB = methods_of(z, "net.minecraft.world.level.block.Blocks")
slab_lambdas = {}
for sig, (cp, body) in sorted(msB.items()):
    if "lambda$static$" in sig and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        joined = " | ".join(lines)
        if "SlabBlock" in joined:
            slab_lambdas[sig] = lines
print(f"  TOTAL slab lambdas: {len(slab_lambdas)}")
for sig, lines in slab_lambdas.items():
    print(f"--- {sig}")
    for l in lines[:30]:
        print("   ", l)
    print()

print()
print("=== the FULL 68-kind model census (bottom model -> json)")
names = sorted(n.split("/")[-1][:-5] for n in z.namelist()
               if n.startswith("assets/minecraft/blockstates/") and n.endswith(".json")
               and n.split("/")[-1][:-5].endswith("_slab"))
census = {}
for name in names:
    bs = json.loads(z.read(f"assets/minecraft/blockstates/{name}.json"))
    vs = bs.get("variants", {})
    row = {}
    for k in ("type=bottom", "type=top", "type=double"):
        v = vs.get(k)
        if v is None:
            row[k] = None
        else:
            if isinstance(v, list):
                v = v[0]
            row[k] = v.get("model", "")
    # resolve the bottom model's textures
    bmodel = row["type=bottom"]
    tmodel = row["type=top"]
    dmodel = row["type=double"]
    def model_info(m):
        if m is None:
            return None
        try:
            d = json.loads(z.read(f"assets/minecraft/models/{m}.json"))
        except Exception as e:
            return f"ERR {e}"
        parent = d.get("parent", "")
        tex = d.get("textures", {})
        return (parent, dict(sorted(tex.items())))
    census[name] = (bmodel, tmodel, dmodel, model_info(bmodel), model_info(tmodel), model_info(dmodel))

# print a compact table
print(f"{'kind':34} | parentB | bottom/side/top | double-model | parentD")
for name in names:
    b, t, d, bi, ti, di = census[name]
    if isinstance(bi, tuple):
        parent_b, tex = bi
        texp = "|".join(f"{k}={v.split('/')[-1]}" for k, v in tex.items())
    else:
        parent_b, texp = "?", str(bi)
    if isinstance(di, tuple):
        parent_d = di[0]
    else:
        parent_d = str(di)
    print(f"{name:34} | {parent_b:22} | {texp:52} | {d:44} | {parent_d}")

print()
print("=== rotation/ref identity: bottom vs top model texture sets equal?")
diff = []
for name in names:
    b, t, d, bi, ti, di = census[name]
    if isinstance(bi, tuple) and isinstance(ti, tuple):
        if bi[1] != ti[1]:
            diff.append(name)
print(f"  kinds where top-model textures differ from bottom: {diff if diff else 'NONE'}")

print()
print("=== double-model census (group by)")
groups = {}
for name in names:
    b, t, d, bi, ti, di = census[name]
    groups.setdefault(d, []).append(name)
for dmodel, kinds in sorted(groups.items()):
    print(f"  double={dmodel}: {len(kinds)} kinds")
    print(f"    {kinds}")

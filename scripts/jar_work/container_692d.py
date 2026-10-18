#!/usr/bin/env python3
"""THE CHEST FAMILY'S FOURTH PASS (dev-64): the full census — entity sheets,
the copper chest blockstates, the Blocks clinit rows, the sounds, the dig
rows, the model regions decode.
"""
import json
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)
names = set(z.namelist())

print("=== entity/chest sheets census")
sheets = sorted(n for n in names if n.startswith("assets/minecraft/textures/entity/chest/"))
for s in sheets:
    data = z.read(s)
    if len(data) < 32:
        continue
    import struct as _st
    w, h = _st.unpack_from(">II", data, 16)
    print(f"  {s}: {w}x{h} ({len(data)} bytes)")

print()
print("=== copper chest blockstates")
for n in ("copper_chest", "exposed_copper_chest", "weathered_copper_chest",
          "oxidized_copper_chest", "waxed_copper_chest", "waxed_exposed_copper_chest",
          "waxed_weathered_copper_chest", "waxed_oxidized_copper_chest"):
    p = f"assets/minecraft/blockstates/{n}.json"
    if p in names:
        bs = json.loads(z.read(p))
        keys = sorted(bs.get("variants", {}).keys())
        print(f"  {n}: {len(keys)} keys; sample: {keys[:6]}")
    else:
        print(f"  {n}: ABSENT")

print()
print("=== copper chest models")
for m in ("copper_chest", "copper_chest_left", "copper_chest_right"):
    p = f"assets/minecraft/models/block/{m}.json"
    print(f"  {m}: {z.read(p).decode()[:300] if p in names else 'ABSENT'}")

print()
print("=== ChestModel texOffs/boxes")
dump_method(z, "net.minecraft.client.model.ChestModel", "<init>", max_pc=400)
dump_method(z, "net.minecraft.client.model.ChestModel", "createBodyLayer", max_pc=500)
ms = methods_of(z, "net.minecraft.client.model.ChestModel")
if ms:
    for sig in sorted(ms):
        print(f"  SIG: {sig}")

print()
print("=== ChestRenderer setup")
dump_method(z, "net.minecraft.client.renderer.blockentity.ChestRenderer", "render", max_pc=400)

print()
print("=== Blocks clinit rows (the 11)")
for kw in ("CHEST", "TRAPPED_CHEST", "ENDER_CHEST", "COPPER_CHEST"):
    try:
        idx = method_index(z, "net.minecraft.world.level.block.Blocks", "<clinit>", kw) \
            if "method_index" in dir() else None
    except Exception as e:
        idx = None
print("  (raw scan below)")
src = disassemble(z, "net.minecraft.world.level.block.Blocks", "<clinit>", 40000) \
    if "disassemble" in dir() else None
if src:
    lines = src.splitlines()
    for i, ln in enumerate(lines):
        if "CHEST" in ln and ("Properties" in "".join(lines[i:i+12]) or "strength" in "".join(lines[i:i+12])):
            print("  ", " | ".join(x.strip() for x in lines[i:i+14]))
            print("   ---")
else:
    # find register rows near CHEST constants by scanning the whole clinit
    code = method_code(z, "net.minecraft.world.level.block.Blocks", "<clinit>")
    import struct as _st2
    clen = _st2.unpack_from(">I", code, 4)[0]
    body = code[8:8+clen]
    lines = walk_code(*_st2.unpack_from(">I", code, 0), body)
    text = [str(l) for l in lines]
    for i, t in enumerate(text):
        if "LDC.W" in t.upper() and "CHEST" in "".join(text[max(0,i-3):i+3]).upper():
            pass
    # fallback: print rows around each CHEST putstatic
    for i, t in enumerate(text):
        if "CHEST" in t and ("putstatic" in t or "getstatic" in t):
            seg = " | ".join(x.strip() for x in text[i:i+10])
            print(f"  pc~{i}: {seg[:400]}")

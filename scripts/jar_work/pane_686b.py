#!/usr/bin/env python3
"""pane_686b — the follow-up probe: noside/bars templates, the FULL
iron_bars.json, the BARS tag, IronBarsBlock.updateShape + the
skipRendering tail, the CrossCollisionBlock fields."""
import sys
import json
import struct
import zipfile

sys.argv = ["x", ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"]
exec(open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0])
JAR = sys.argv[1]
z = zipfile.ZipFile(JAR)

print("=== full iron_bars.json")
print(z.read("assets/minecraft/blockstates/iron_bars.json").decode())

print("=== noside + bars template models")
for m in ("template_glass_pane_noside", "template_glass_pane_noside_alt",
          "glass_pane_noside", "glass_pane_noside_alt",
          "template_bars_post", "template_bars_cap", "template_bars_cap_alt",
          "template_bars_side", "template_bars_side_alt", "iron_bars_post_ends"):
    try:
        j = json.loads(z.read(f"assets/minecraft/models/block/{m}.json"))
        print(f"--- {m}.json: {json.dumps(j)}")
    except KeyError:
        print(f"--- {m}.json: ABSENT")

print()
print("=== BARS tag")
try:
    j = json.loads(z.read("data/minecraft/tags/block/bars.json"))
    print(json.dumps(j))
except KeyError:
    print("ABSENT")

print()
print("=== IronBarsBlock updateShape + skipRendering tail")
dump_method(z, "net.minecraft.world.level.block.IronBarsBlock", "updateShape", max_pc=140)
ms = methods_of(z, "net.minecraft.world.level.block.IronBarsBlock")
for sig, (cp, body) in ms.items():
    if sig.startswith("skipRendering") and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        ls = walk_code(cp, code)
        print(f"--- skipRendering FULL ({len(ls)} pcs)")
        for l in ls:
            print(l)

print()
print("=== CrossCollisionBlock fields")
d = z.read("net/minecraft/world/level/block/CrossCollisionBlock.class")
cp, p = parse_cp(d)
p2 = p + 6
ifc_n = struct.unpack_from(">H", d, p2)[0]; p2 += 2 + 2 * ifc_n
f_n = struct.unpack_from(">H", d, p2)[0]; p2 += 2
for _ in range(f_n):
    acc = struct.unpack_from(">H", d, p2)[0]
    ni = struct.unpack_from(">H", d, p2 + 2)[0]
    di = struct.unpack_from(">H", d, p2 + 4)[0]
    print(f"field {utf(cp, ni)} : {utf(cp, di)}")
    attrs = struct.unpack_from(">H", d, p2 + 6)[0]
    p2 += 8
    for _ in range(attrs):
        ln = struct.unpack_from(">I", d, p2 + 2)[0]
        p2 += 6 + ln

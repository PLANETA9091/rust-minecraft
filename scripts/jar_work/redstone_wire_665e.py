#!/usr/bin/env python3
"""THE WIRE'S FACTORY ROWS (the autopush-665 round): the Blocks clinit
REDSTONE_WIRE row (the Properties chain the noCollision/strength/sound
windows speak) + the Items clinit REDSTONE row (the BlockItem pairing).
The weighted_plate_49 harness keeps.
"""
import struct
import sys
import zipfile

src = open("scripts/jar_work/weighted_plate_49.py").read()
cut = src.index("def parse_cp")
prefix = src[cut:].split('if __name__ == "__main__":')[0]
exec(prefix)

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

d = z.read("net/minecraft/world/level/block/Blocks.class")
cp, cpend = parse_cp(d)
# find the putstatic for REDSTONE_WIRE field ref
target = None
for i, e in enumerate(cp):
    if e and e[0] == 9:  # getstatic/putstatic Fieldref
        nt = cp[e[2]]
        name = utf(cp, nt[1])
        if name == "REDSTONE_WIRE":
            target = i
            break
print("REDSTONE_WIRE fieldref cp index:", target)

# walk the clinit code, find putstatic target, print a window around it
for (mn, md, an, body) in methods_of(d, cp, cpend):
    if mn != "<clinit>":
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    steps = walk_code(cp, code)
    for k, (pc, mnem, detail) in enumerate(steps):
        if "putstatic" in mnem and "REDSTONE_WIRE" in detail:
            lo = max(0, k - 60)
            print(f"\n-- window around pc {pc} (steps {lo}..{k + 3}):")
            for (pc2, mn2, dt2) in steps[lo:k + 3]:
                print(f"  {pc2}: {mn2} {dt2}")
            break

# the Items row
d2 = z.read("net/minecraft/world/item/Items.class")
cp2, cpend2 = parse_cp(d2)
t2 = None
for i, e in enumerate(cp2):
    if e and e[0] == 9:
        nt = cp2[e[2]]
        if utf(cp2, nt[1]) == "REDSTONE":
            t2 = i
            break
print("\nItems.REDSTONE fieldref cp index:", t2)
for (mn, md, an, body) in methods_of(d2, cp2, cpend2):
    if mn != "<clinit>":
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    steps = walk_code(cp2, code)
    for k, (pc, mnem, detail) in enumerate(steps):
        if "putstatic" in mnem and detail.rstrip().endswith(".REDSTONE:") or ("putstatic" in mnem and ".REDSTONE:" in detail):
            lo = max(0, k - 30)
            print(f"\n-- Items window around pc {pc}:")
            for (pc2, mn2, dt2) in steps[lo:k + 3]:
                print(f"  {pc2}: {mn2} {dt2}")
            break

#!/usr/bin/env python3
"""Blocks clinit window probe: the skeleton_wall_skull registration row —
the Properties chain (lootTable override?)."""
import struct, zipfile

src = open("scripts/skull_probe_83.py").read()
ns = {}
exec(src.split("SKULL = ")[0], ns)
CF = ns["CF"]
load = ns["load"]
disasm = ns["disasm"]

cf = CF(load("net/minecraft/world/level/block/Blocks"))
code = None
for name, desc, c in cf.methods():
    if name == "<clinit>":
        code = c
lines = disasm(cf, code)
print("clinit lines:", len(lines))
# find putstatic SkeletonWallSkull windows
hits = [i for i, (pc, t) in enumerate(lines) if "SKELETON_WALL_SKULL" in t or "WALL_SKULL" in t]
print("wall-skull rows:", [(lines[i][0], lines[i][1]) for i in hits[:8]])
if hits:
    lo = max(0, hits[0] - 40)
    hi = min(len(lines), hits[0] + 12)
    for pc, t in lines[lo:hi]:
        print(f"  pc {pc:5d}: {t}")

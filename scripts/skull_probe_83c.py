#!/usr/bin/env python3
"""Dump Block.boxZ(DDD) + Block.column(DDD) bodies."""
import struct, zipfile, sys

JAR = "tools/client.jar"

def load(cls):
    with zipfile.ZipFile(JAR) as z:
        return z.read(cls + ".class")

# reuse the full decoder by exec-ing the probe module with __name__ guard trick:
# simpler — import the class definition from skull_probe_83 by copying minimal parts.
src = open("scripts/skull_probe_83.py").read()
head = src.split("SKULL = ")[0]
head = head.replace('JAR = "tools/client.jar"', 'JAR = "tools/client.jar"')
ns = {}
exec(head, ns)
CF = ns["CF"]
load = ns["load"]
disasm = ns["disasm"]

cf = CF(load("net/minecraft/world/level/block/Block"))
print("=== Block boxZ/column/box census ===")
for name, desc, code in cf.methods():
    if name in ("boxZ", "column", "box"):
        print(f"{name} {desc} code={0 if code is None else len(code)}")

def dump(name, desc):
    print(f"--- Block.{name}{desc} ---")
    for pc, txt in disasm(cf, code):
        print(f"  pc {pc:4d}: {txt}")

for name, desc, code in cf.methods():
    if name == "boxZ" and desc != "(DDD)Lnet/minecraft/world/phys/shapes/VoxelShape;":
        dump(name, desc)

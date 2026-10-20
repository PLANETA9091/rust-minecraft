#!/usr/bin/env python3
"""Rail probe 26.2 (dev-86): the BaseRailBlock/RailBlock laws — the
placement orientation (getStateForPlacement), the canSurvive below-check,
the collision/shape rows, the noCollision/instabreak registration, the
powered family's class census."""
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from skull_probe_83 import show, census, jar_bytes
import json

RAIL = "net/minecraft/world/level/block/RailBlock"
BASE = "net/minecraft/world/level/block/BaseRailBlock"
POW = "net/minecraft/world/level/block/DetectorRailBlock"
ACT = "net/minecraft/world/level/block/PoweredRailBlock"

print("===== RAIL BLOCK =====")
census(RAIL)
show(RAIL, ["<clinit>", "<init>", "getStateForPlacement", "canSurvive",
            "updateShape", "rotate", "mirror", "getShape", "getCollisionShape"],
     fields=True)

print("===== BASE RAIL BLOCK =====")
census(BASE)
show(BASE, ["<clinit>", "<init>", "canSurvive", "getShape", "getCollisionShape",
            "getStateForPlacement", "getRailDirection", "updateDir"],
     fields=True)

print("===== DETECTOR RAIL (census only) =====")
census(POW)
print("===== POWERED RAIL (census only) =====")
census(ACT)

print("===== BLOCKSTATE + MODELS =====")
for n in ["assets/minecraft/blockstates/rail.json",
          "assets/minecraft/blockstates/powered_rail.json",
          "assets/minecraft/blockstates/detector_rail.json",
          "assets/minecraft/blockstates/activator_rail.json"]:
    b = jar_bytes(n)
    print(n, "::", (b.decode() if b else "MISSING"))

print("===== LOOT TABLES =====")
for n in ["data/minecraft/loot_table/blocks/rail.json",
          "data/minecraft/loot_tables/blocks/rail.json"]:
    b = jar_bytes(n)
    if b:
        print(n, "::", b.decode()[:400])
        break
else:
    print("rail loot MISSING in both paths")

print("===== TEXTURE SIZES =====")
import struct
for n in ["assets/minecraft/textures/block/rail.png",
          "assets/minecraft/textures/block/powered_rail.png",
          "assets/minecraft/textures/block/detector_rail.png",
          "assets/minecraft/textures/block/activator_rail.png"]:
    b = jar_bytes(n)
    if b:
        w, h = struct.unpack(">II", b[16:24])
        print(n, w, "x", h)
    else:
        print(n, "MISSING")

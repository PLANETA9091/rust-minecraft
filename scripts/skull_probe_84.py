#!/usr/bin/env python3
"""Skull ceiling-click probe 26.2 (dev-84): the StandingAndWallBlockItem
getPlacementState scan law — what a DOWN-face (ceiling) click yields —,
the floor canSurvive, the item's canPlace, the place flow."""
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from skull_probe_83 import show, census

ITEM = "net/minecraft/world/item/StandingAndWallBlockItem"
SKULL = "net/minecraft/world/level/block/SkullBlock"
ABS = "net/minecraft/world/level/block/AbstractSkullBlock"

census(ITEM)
show(ITEM, ["<init>", "getPlacementState", "canPlace", "place",
            "updatePlacementContext", "getWallBlock", "setBlockEntityData"],
     fields=True)
show(SKULL, ["<clinit>", "<init>", "getStateForPlacement", "canSurvive",
             "createBlockStateDefinition"], fields=True)
show(ABS, ["<init>", "canSurvive", "getStateForPlacement"])

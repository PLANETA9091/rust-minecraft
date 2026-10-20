#!/usr/bin/env python3
"""dev-84 probe: Direction.orderedByNearest (the look-order law),
BlockPlaceContext.getNearestLookingDirections, BlockItem.place flow."""
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from skull_probe_83 import show, census

DIR = "net/minecraft/core/Direction"
CTX = "net/minecraft/world/item/context/BlockPlaceContext"
BI = "net/minecraft/world/item/BlockItem"

census(DIR)
show(DIR, ["orderedByNearest", "getNearest", "<clinit>"])
show(CTX, ["getNearestLookingDirections", "canPlace", "<init>"])
show(BI, ["place", "getPlacementState", "canPlace"])

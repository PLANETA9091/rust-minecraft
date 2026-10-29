#!/usr/bin/env python3
"""mix_rows_339.py — the autopush-339 round's own table generator.

Parses scripts/jar_work/brewing_mixes_309.txt (the addVanillaMixes
javap walk the 309 round banked) + the Builder.addStartMix law the
builder_339.txt speaks (addStartMix(item, potion) = the addMix pair
water->mundane + awkward->potion), resolves the potion registry ids
from potions_clinit_339.txt (the registration order) and the item
ids from the wire order, and emits the Rust POTION_MIXES table.
"""
import re

# --- the potion registry order (potions_clinit_339.txt, verified 46/46) ---
POTIONS = [
    "water", "mundane", "thick", "awkward", "night_vision", "long_night_vision",
    "invisibility", "long_invisibility", "leaping", "long_leaping", "strong_leaping",
    "fire_resistance", "long_fire_resistance", "swiftness", "long_swiftness",
    "strong_swiftness", "slowness", "long_slowness", "strong_slowness",
    "turtle_master", "long_turtle_master", "strong_turtle_master",
    "water_breathing", "long_water_breathing", "healing", "strong_healing",
    "harming", "strong_harming", "poison", "long_poison", "strong_poison",
    "regeneration", "long_regeneration", "strong_regeneration",
    "strength", "long_strength", "strong_strength", "weakness", "long_weakness",
    "luck", "slow_falling", "long_slow_falling", "wind_charged", "weaving",
    "oozing", "infested",
]
POT_ID = {n: i for i, n in enumerate(POTIONS)}

# --- the item ids (the wire order table the mc-registry banks) ---
ITEMS = {
    "glowstone_dust": 1085, "redstone": 745, "nether_wart": 1148,
    "breeze_rod": 1252, "slime_block": 752, "stone": 1, "cobweb": 228,
    "golden_carrot": 1262, "fermented_spider_eye": 1152, "magma_cream": 1154,
    "rabbit_foot": 1282, "turtle_helmet": 915, "sugar": 1113,
    "pufferfish": 1089, "glistering_melon_slice": 1158, "spider_eye": 1151,
    "ghast_tear": 1146, "phantom_membrane": 889, "blaze_powder": 1153,
    "gunpowder": 978, "dragon_breath": 1320,
}

# --- parse the addVanillaMixes walk in bytecode order ---
src = open("scripts/jar_work/brewing_mixes_309.txt").read()
lines = src.splitlines()

# collect the getstatic constants between pc boundaries per addMix/addStartMix call
events = []  # ('addMix', from_potion, item, to_potion) | ('startMix', item, potion)
window = []
for ln in lines:
    m = re.search(r'getstatic\s+#\d+\s+net/minecraft/world/item/alchemy/Potions\.([A-Z_]+)', ln)
    if m:
        window.append(("pot", m.group(1)))
        continue
    m = re.search(r'getstatic\s+#\d+\s+net/minecraft/world/item/Items\.([A-Z_]+)', ln)
    if m:
        window.append(("item", m.group(1)))
        continue
    if "addMix" in ln and "Builder.addMix" in ln:
        pots = [v for k, v in window if k == "pot"]
        items = [v for k, v in window if k == "item"]
        assert len(pots) == 2 and len(items) == 1, (pots, items, ln)
        events.append(("addMix", pots[0], items[0], pots[1]))
        window = []
    elif "addStartMix" in ln and "Builder.addStartMix" in ln:
        pots = [v for k, v in window if k == "pot"]
        items = [v for k, v in window if k == "item"]
        assert len(pots) == 1 and len(items) == 1, (pots, items, ln)
        events.append(("startMix", items[0], pots[0]))
        window = []
    elif "addContainerRecipe" in ln and "Builder.addContainerRecipe" in ln:
        window = []  # the container rows the client's own consts pin

assert len(events) == 52, f"expected 52 events, got {len(events)}"

def item_const(java_item):
    # java field name -> the snake_case item name the registry banks
    known = {
        "REDSTONE": "redstone", "GLOWSTONE_DUST": "glowstone_dust",
        "NETHER_WART": "nether_wart", "BREEZE_ROD": "breeze_rod",
        "SLIME_BLOCK": "slime_block", "STONE": "stone", "COBWEB": "cobweb",
        "GOLDEN_CARROT": "golden_carrot", "FERMENTED_SPIDER_EYE": "fermented_spider_eye",
        "MAGMA_CREAM": "magma_cream", "RABBIT_FOOT": "rabbit_foot",
        "TURTLE_HELMET": "turtle_helmet", "SUGAR": "sugar",
        "PUFFERFISH": "pufferfish", "GLISTERING_MELON_SLICE": "glistering_melon_slice",
        "SPIDER_EYE": "spider_eye", "GHAST_TEAR": "ghast_tear",
        "BLAZE_POWDER": "blaze_powder", "PHANTOM_MEMBRANE": "phantom_membrane",
    }
    name = known[java_item]
    return name, ITEMS[name]

rows = []  # (from_potion_name, item_name, item_id, to_potion_name)
for ev in events:
    if ev[0] == "addMix":
        _, fp, it, tp = ev
        iname, iid = item_const(it)
        rows.append((fp.lower(), iname, iid, tp.lower()))
    else:
        _, it, pot = ev
        iname, iid = item_const(it)
        # THE BUILDER'S OWN EXPANSION (addStartMix = the addMix pair:
        # water + item -> MUNDANE, awkward + item -> potion)
        rows.append(("water", iname, iid, "mundane"))
        rows.append(("awkward", iname, iid, pot.lower()))

print(f"// {len(rows)} rows (the addVanillaMixes order the builder's list keeps,")
print(f"// the addStartMix pair the expansion wears)")
out = []
for fp, iname, iid, tp in rows:
    out.append(f"    (POTION_{fp.upper()}, crate::world::ITEM_{iname.upper()}, POTION_{tp.upper()}),")
print("\n".join(out))
print("// kind name sanity:", len(POTIONS), "potions")

#!/usr/bin/env python3
"""dev-196 twin — the 26.2 enchantment selection walk the python bit-exact
lane keeps (the LCG the client's VanillaRandom carries), plus the COSTS
refresh roll lane (the dev-195 walk the seed answers). The walk order the
javap banks (scripts/jar_work/enchant_select_196.txt the evidence speaks):

  getEnchantmentList:   setSeed((long)(seed + row)); select; book trim
  selectEnchantment:    the level roll + the 0.15 spread + the
                        available scan (maxLevel..minLevel, the cost
                        window) + the weighted pick + the nextInt(50)
                        halving loop + the conflict filter
  getEnchantmentCost:   the dev-195 lane (the 15 cap, the base roll,
                        the three slot arms)
"""
M = (1 << 48) - 1
MULT = 0x5DEECE66D
ADD = 0xB


class R:
    def __init__(self, seed):
        self.s = (seed ^ MULT) & M

    def _next(self, bits):
        self.s = (self.s * MULT + ADD) & M
        return self.s >> (48 - bits)

    def next_int(self, bound):
        assert bound > 0
        if (bound & -bound) == bound:
            return (bound * self._next(31)) >> 31
        while True:
            r = self._next(31)
            v = r % bound
            if r - v + (bound - 1) >= 0:
                return v

    def next_unbounded(self):
        return self._next(32)

    def next_f32(self):
        return self._next(24) / float(1 << 24)


def f32(x):
    import struct
    return struct.unpack('f', struct.pack('f', x))[0]


# --- the 43 defs (the jar's exact numbers, alphabetical registry order) ---
# (name, weight, max_level, min_base, min_per, max_base, max_per, exclusive_tag, supported, primary)
DEFS = [
    ("aqua_affinity", 2, 1, 1, 0, 41, 0, None, "head_armor", None),
    ("bane_of_arthropods", 5, 5, 5, 8, 25, 8, "damage", "weapon", "melee_weapon"),
    ("binding_curse", 1, 1, 25, 0, 50, 0, None, "equippable", None),
    ("blast_protection", 2, 4, 5, 8, 13, 8, "armor", "armor", None),
    ("breach", 2, 4, 15, 9, 65, 9, "damage", "mace", None),
    ("channeling", 1, 1, 25, 0, 50, 0, None, "trident", None),
    ("density", 5, 5, 5, 8, 25, 8, "damage", "mace", None),
    ("depth_strider", 2, 3, 10, 10, 25, 10, "boots", "foot_armor", None),
    ("efficiency", 10, 5, 1, 10, 51, 10, None, "mining", None),
    ("feather_falling", 5, 4, 5, 6, 11, 6, None, "foot_armor", None),
    ("fire_aspect", 2, 2, 10, 20, 60, 20, None, "fire_aspect", "melee_weapon"),
    ("fire_protection", 5, 4, 10, 8, 18, 8, "armor", "armor", None),
    ("flame", 2, 1, 20, 0, 50, 0, None, "bow", None),
    ("fortune", 2, 3, 15, 9, 65, 9, "mining", "mining_loot", None),
    ("frost_walker", 2, 2, 10, 10, 25, 10, "boots", "foot_armor", None),
    ("impaling", 2, 5, 1, 8, 21, 8, "damage", "trident", None),
    ("infinity", 1, 1, 20, 0, 50, 0, "bow", "bow", None),
    ("knockback", 5, 2, 5, 20, 55, 20, None, "melee_weapon", None),
    ("looting", 2, 3, 15, 9, 65, 9, None, "melee_weapon", None),
    ("loyalty", 5, 3, 12, 7, 50, 0, "riptide", "trident", None),
    ("luck_of_the_sea", 2, 3, 15, 9, 65, 9, None, "fishing", None),
    ("lunge", 5, 3, 5, 8, 25, 8, None, "lunge", None),
    ("lure", 2, 3, 15, 9, 65, 9, None, "fishing", None),
    ("mending", 2, 1, 25, 25, 75, 25, None, "durability", None),
    ("multishot", 2, 1, 20, 0, 50, 0, "crossbow", "crossbow", None),
    ("piercing", 10, 4, 1, 10, 50, 0, "crossbow", "crossbow", None),
    ("power", 10, 5, 1, 10, 16, 10, None, "bow", None),
    ("projectile_protection", 5, 4, 3, 6, 9, 6, "armor", "armor", None),
    ("protection", 10, 4, 1, 11, 12, 11, "armor", "armor", None),
    ("punch", 2, 2, 12, 20, 37, 20, None, "bow", None),
    ("quick_charge", 5, 3, 12, 20, 50, 0, None, "crossbow", None),
    ("respiration", 2, 3, 10, 10, 40, 10, None, "head_armor", None),
    ("riptide", 2, 3, 17, 7, 50, 0, "riptide", "trident", None),
    ("sharpness", 10, 5, 1, 11, 21, 11, "damage", "sharp_weapon", "melee_weapon"),
    ("silk_touch", 1, 1, 15, 0, 65, 0, "mining", "mining_loot", None),
    ("smite", 5, 5, 5, 8, 25, 8, "damage", "weapon", "melee_weapon"),
    ("soul_speed", 1, 3, 10, 10, 25, 10, None, "foot_armor", None),
    ("sweeping_edge", 2, 3, 5, 9, 20, 9, None, "sweeping", None),
    ("swift_sneak", 1, 3, 25, 25, 75, 25, None, "leg_armor", None),
    ("thorns", 1, 3, 10, 20, 60, 20, None, "armor", "chest_armor"),
    ("unbreaking", 5, 3, 5, 8, 55, 8, None, "durability", None),
    ("vanishing_curse", 1, 1, 25, 0, 50, 0, None, "vanishing", None),
    ("wind_burst", 2, 3, 15, 9, 65, 9, None, "mace", None),
]
NAMES = [d[0] for d in DEFS]
IDX = {n: i for i, n in enumerate(NAMES)}

EXCLUSIVE_SETS = {
    "armor": ["protection", "blast_protection", "fire_protection", "projectile_protection"],
    "boots": ["frost_walker", "depth_strider"],
    "bow": ["infinity", "mending"],
    "crossbow": ["multishot", "piercing"],
    "damage": ["sharpness", "smite", "bane_of_arthropods", "impaling", "density", "breach"],
    "mining": ["fortune", "silk_touch"],
    "riptide": ["loyalty", "channeling"],
}

TABLE_MEMBERS = [d[0] for d in DEFS if d[0] not in
                 ("binding_curse", "frost_walker", "mending", "soul_speed",
                  "swift_sneak", "vanishing_curse", "wind_burst")]

CURSE_MEMBERS = ["binding_curse", "vanishing_curse"]

SWORDS = ["copper_sword", "diamond_sword", "golden_sword", "iron_sword",
          "netherite_sword", "stone_sword", "wooden_sword"]
AXES = ["copper_axe", "diamond_axe", "golden_axe", "iron_axe",
        "netherite_axe", "stone_axe", "wooden_axe"]
PICKAXES = ["copper_pickaxe", "diamond_pickaxe", "golden_pickaxe",
            "iron_pickaxe", "netherite_pickaxe", "stone_pickaxe", "wooden_pickaxe"]
SHOVELS = ["copper_shovel", "diamond_shovel", "golden_shovel", "iron_shovel",
           "netherite_shovel", "stone_shovel", "wooden_shovel"]
HOES = ["copper_hoe", "diamond_hoe", "golden_hoe", "iron_hoe",
        "netherite_hoe", "stone_hoe", "wooden_hoe"]
SPEARS = ["copper_spear", "diamond_spear", "golden_spear", "iron_spear",
          "netherite_spear", "stone_spear", "wooden_spear"]
SKULLS = ["creeper_head", "dragon_head", "piglin_head", "player_head",
          "skeleton_skull", "wither_skeleton_skull", "zombie_head"]
FOOT = ["chainmail_boots", "copper_boots", "diamond_boots", "golden_boots",
        "iron_boots", "leather_boots", "netherite_boots"]
LEG = ["chainmail_leggings", "copper_leggings", "diamond_leggings",
       "golden_leggings", "iron_leggings", "leather_leggings", "netherite_leggings"]
CHEST = ["chainmail_chestplate", "copper_chestplate", "diamond_chestplate",
         "golden_chestplate", "iron_chestplate", "leather_chestplate",
         "netherite_chestplate"]
HEAD = ["chainmail_helmet", "copper_helmet", "diamond_helmet",
        "golden_helmet", "iron_helmet", "leather_helmet", "netherite_helmet",
        "turtle_helmet"]
ARMOR = FOOT + LEG + CHEST + HEAD
MELEE = SWORDS + SPEARS
SHARP = MELEE + AXES
DURABILITY = (ARMOR + SWORDS + AXES + PICKAXES + SHOVELS + HOES + SPEARS +
              ["elytra", "shield", "bow", "crossbow", "trident",
               "flint_and_steel", "shears", "brush", "fishing_rod",
               "carrot_on_a_stick", "warped_fungus_on_a_stick", "mace"])
EQUIPPABLE = ARMOR + SKULLS + ["elytra", "carved_pumpkin"]
VANISHING = DURABILITY + SKULLS + ["compass", "carved_pumpkin"]
MINING = AXES + PICKAXES + SHOVELS + HOES + ["shears"]
MINING_LOOT = AXES + PICKAXES + SHOVELS + HOES
WEAPON = SHARP + ["mace"]

FAMILIES = {
    "armor": ARMOR, "bow": ["bow"], "chest_armor": CHEST, "crossbow": ["crossbow"],
    "durability": DURABILITY, "equippable": EQUIPPABLE,
    "fire_aspect": MELEE, "fishing": ["fishing_rod"], "foot_armor": FOOT,
    "head_armor": HEAD, "leg_armor": LEG, "lunge": SPEARS, "mace": ["mace"],
    "melee_weapon": MELEE, "mining": MINING, "mining_loot": MINING_LOOT,
    "sharp_weapon": SHARP, "sweeping": SWORDS, "trident": ["trident"],
    "vanishing": VANISHING, "weapon": WEAPON,
}

ENCHANTABILITY = {}
_tool_val = {"wooden": 15, "stone": 5, "copper": 13, "iron": 14,
             "diamond": 10, "golden": 22, "netherite": 15}
for mat, v in _tool_val.items():
    for kind in ["sword", "shovel", "pickaxe", "axe", "hoe", "spear"]:
        ENCHANTABILITY[f"{mat}_{kind}"] = v
_armor_val = {"leather": 15, "copper": 8, "chainmail": 12, "iron": 9,
              "golden": 25, "diamond": 10, "netherite": 15}
for mat, v in _armor_val.items():
    for kind in ["helmet", "chestplate", "leggings", "boots"]:
        ENCHANTABILITY[f"{mat}_{kind}"] = v
ENCHANTABILITY["turtle_helmet"] = 9
for n, v in [("book", 1), ("bow", 1), ("crossbow", 1), ("fishing_rod", 1),
             ("trident", 1), ("mace", 15), ("elytra", 1), ("shield", 1),
             ("flint_and_steel", 1), ("shears", 1), ("brush", 1),
             ("carrot_on_a_stick", 1), ("warped_fungus_on_a_stick", 1)]:
    ENCHANTABILITY[n] = v


def ench_of(item):
    return ENCHANTABILITY.get(item, 0)


def is_book(item):
    return item == "book"


def compatible(a, b):
    ia, ib = IDX[a], IDX[b]
    if ia == ib:
        return False
    ta = DEFS[ia][7]
    tb = DEFS[ib][7]
    if ta and b in EXCLUSIVE_SETS[ta]:
        return False
    if tb and a in EXCLUSIVE_SETS[tb]:
        return False
    return True


def cost_at(d, level, base, per):
    return base + per * (level - 1)


def get_enchantment_cost(rng, slot, power, enchantable):
    if enchantable is None:
        return 0
    p = min(power, 15)
    base = rng.next_int(8) + 1 + (p >> 1) + rng.next_int(p + 1)
    if slot == 0:
        return max(base // 3, 1)
    if slot == 1:
        return base * 2 // 3 + 1
    return max(base, p * 2)


def available(cost, item):
    # the lambda$0 law: isPrimaryItem(stack) || isBook
    # isPrimaryItem = isSupportedItem && (primary.isEmpty() || stack.is(primary))
    book = is_book(item)
    out = []
    for name in TABLE_MEMBERS:
        d = DEFS[IDX[name]]
        if book:
            ok = True  # the isBook short-circuit the lambda's second arm keeps
        else:
            ok = item in FAMILIES[d[8]] and (
                d[9] is None or item in FAMILIES[d[9]])
        if not ok:
            continue
        for l in range(d[2], 0, -1):
            lo = cost_at(d, l, d[3], d[4])
            hi = cost_at(d, l, d[5], d[6])
            if cost >= lo and cost <= hi:
                out.append((name, l))
                break
    return out


def weighted_pick(rng, avail):
    total = sum(DEFS[IDX[n]][1] for n, _ in avail)
    if total <= 0:
        return None
    i = rng.next_int(total)
    for n, l in avail:
        i -= DEFS[IDX[n]][1]
        if i < 0:
            return (n, l)
    return None


def select_enchantment(rng, item, level):
    e = ench_of(item)
    if e <= 0:
        return []
    level = level + 1 + rng.next_int(e // 4 + 1) + rng.next_int(e // 4 + 1)
    # the f32 STEP chain the Java arithmetic keeps (each op rounds to f32
    # — the f64-lane drift the last-bit flips the round gate rides)
    import math
    f1 = rng.next_f32()
    f2 = rng.next_f32()
    f = f32(f32(f32(f1 + f2) - 1.0) * 0.15)
    prod = f32(f32(level) * f)
    total = f32(f32(level) + prod)
    lvl = math.floor(total + 0.5)  # the Math.round(float) law
    lvl = max(1, min(lvl, 2147483647))
    avail = available(lvl, item)
    out = []
    if not avail:
        return out
    p = weighted_pick(rng, avail)
    if p:
        out.append(p)
    while rng.next_int(50) <= lvl:
        if out:
            last = out[-1][0]
            avail = [(n, l) for (n, l) in avail if compatible(n, last)]
        if not avail:
            break
        p = weighted_pick(rng, avail)
        if p:
            out.append(p)
        lvl = lvl // 2
    return out


def get_enchantment_list(seed, row, item, cost):
    rng = R(seed + row)
    lst = select_enchantment(rng, item, cost)
    if is_book(item) and len(lst) > 1:
        del lst[rng.next_int(len(lst))]
    return lst


def costs_refresh(seed, power, item_empty):
    rng = R(seed)
    if item_empty:
        return [0, 0, 0]
    out = []
    for i in range(3):
        v = get_enchantment_cost(rng, i, power, 1)
        if v < i + 1:
            v = 0
        out.append(v)
    return out


if __name__ == "__main__":
    # --- the costs lane (seed 0, power 1: the one bookshelf ring) ---
    costs = costs_refresh(0, 1, False)
    print("costs(seed=0, power=1):", costs)
    # the DIAMOND SWORD walk (seed 0, rows 0..2)
    for row in range(3):
        lst = get_enchantment_list(0, row, "diamond_sword", costs[row])
        print(f"list(seed=0,row={row},diamond_sword,cost={costs[row]}):",
              [(n, l) for n, l in lst])
    # the BOOK walk (the trim law)
    blist = get_enchantment_list(0, 0, "book", costs[0])
    print("book list row0:", blist)
    # the non-enchantable gate
    print("dirt list row0:", get_enchantment_list(0, 0, "dirt", costs[0]))
    # the big-power sword (power 15, the max ring)
    costs15 = costs_refresh(0, 15, False)
    print("costs(seed=0, power=15):", costs15)
    for row in range(3):
        lst = get_enchantment_list(0, row, "diamond_sword", costs15[row])
        print(f"list(seed=0,row={row},diamond_sword,cost={costs15[row]}):",
              [(n, l) for n, l in lst])
    # the golden armor walk (a supported-not-primary lane: thorns primary chest)
    print("golden_helmet list row0 (p15):",
          get_enchantment_list(0, 0, "golden_helmet", costs15[0]))

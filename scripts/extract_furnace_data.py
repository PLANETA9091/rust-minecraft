#!/usr/bin/env python3
"""Extract the furnace's own data from the client.jar into a generated
Rust table: crates/mc-registry/src/generated/furnace_data.rs.

Two walks, both pinned by javap against the same jar (26.2):

1. THE SMELTING RECIPES — the jar's data/minecraft/recipe/*.json rows
   whose "type" is "minecraft:smelting" (73 in 26.2). Every ingredient
   is a plain item string (no tag rows — the 26.2 data pack's own
   shape), every result is {"id": ..., "count"?}; the cookingtime
   field is ABSENT from every row, so the SmeltingRecipe clinit's own
   default speaks (javap SmeltingRecipe static{}: `sipush 200` the
   cookingMapCodec(I) default wears — 200 ticks, the
   AbstractFurnaceBlockEntity.getTotalCookTime's `sipush 200` fallback
   the no-recipe arm keeps).

2. THE FUEL VALUES — javap FuelValues.vanillaBurnTimes(Provider,
   FeatureFlagSet, I): the Builder.add walk over base 200
   (BURN_TIME_STANDARD), the tag rows the jar's own
   data/minecraft/tags/item/<name>.json answer (nested #refs resolved,
   cycle-guarded), and the tail `remove(NON_FLAMMABLE_WOOD)` the
   subtracted rows keep (the crimson/warped stems the #logs tag had
   carried — the removal the last add speaks).

The output is indexed by the ITEM WIRE ID (the ITEM_WIRE_ORDER index
space the containers speak — the furnace slot (id, count) pairs the
client holds); rows the wire order never names are skipped with a
note (the extractor's own audit line).

Usage: extract_furnace_data.py [client.jar]
       (default /tmp/mc-verify/client.jar)
"""
import json
import re
import sys
import zipfile
from pathlib import Path

JAR = Path(sys.argv[1] if len(sys.argv[1:] or []) > 0 and sys.argv[1] else "/tmp/mc-verify/client.jar")
OUT = Path(__file__).resolve().parent.parent / "crates/mc-registry/src/generated/furnace_data.rs"
ITEM_ORDER = Path(__file__).resolve().parent.parent / "crates/mc-registry/src/generated/item_order.rs"

# THE FUEL WALK'S OWN ROWS (javap FuelValues.vanillaBurnTimes — the
# Builder.add order the bytecode keeps; ("item", name, ticks) the
# direct rows, ("tag", name, ticks) the tag rows the jar answers).
# The base 200 the sipush speaks: 200*100, 200*8*10, 200*12, 200*8,
# 200*3/2 ... the literals folded here.
FUEL_ITEMS = [
    ("lava_bucket", 20000),        # 200 * 100
    ("coal_block", 16000),         # 200 * 8 * 10
    ("blaze_rod", 2400),           # 200 * 12
    ("coal", 1600),                # 200 * 8
    ("charcoal", 1600),            # 200 * 8
    ("bamboo_mosaic", 300),        # 200 * 3 / 2
    ("bamboo_mosaic_stairs", 300), # 200 * 3 / 2
    ("bamboo_mosaic_slab", 150),   # 200 * 3 / 4
    ("note_block", 300),
    ("bookshelf", 300),
    ("chiseled_bookshelf", 300),
    ("lectern", 300),
    ("jukebox", 300),
    ("chest", 300),
    ("trapped_chest", 300),
    ("crafting_table", 300),
    ("daylight_detector", 300),
    ("bow", 300),
    ("fishing_rod", 300),
    ("ladder", 300),
    ("wooden_shovel", 200),
    ("wooden_sword", 200),
    ("wooden_spear", 200),
    ("wooden_hoe", 200),
    ("wooden_axe", 200),
    ("wooden_pickaxe", 200),
    ("dried_kelp_block", 4000),    # 200 * 1 * 20
    ("crossbow", 300),             # 200 * 3 / 2
    ("bamboo", 50),                # 200 / 4
    ("dead_bush", 100),            # 200 / 2
    ("short_dry_grass", 100),
    ("tall_dry_grass", 100),
    ("scaffolding", 50),           # 200 / 4
    ("azalea", 100),
    ("flowering_azalea", 100),
    ("mangrove_roots", 300),
    ("leaf_litter", 100),
    ("stick", 100),                # 200 / 2
    ("bowl", 100),                 # 200 / 2
]
FUEL_TAGS = [
    ("logs", 300),                  # 200 * 3 / 2
    ("bamboo_blocks", 300),
    ("planks", 300),
    ("wooden_stairs", 300),
    ("wooden_slabs", 150),          # 200 * 3 / 4
    ("wooden_trapdoors", 300),
    ("wooden_pressure_plates", 300),
    ("wooden_shelves", 300),
    ("wooden_fences", 300),
    ("fence_gates", 300),
    ("banners", 300),
    ("signs", 200),                 # the base itself
    ("hanging_signs", 800),         # 200 * 4
    ("wooden_doors", 200),
    ("boats", 1200),                # 200 * 6
    ("wool", 100),                  # 200 / 2
    ("wooden_buttons", 100),
    ("saplings", 100),
    ("wool_carpets", 66),           # 200 * 1 / 3
]
# THE TAIL'S OWN REMOVE (javap pc 651-654 — the tag's members the map
# loses AFTER every add spoke).
FUEL_REMOVE_TAG = "non_flammable_wood"


def load_item_tags(zf: zipfile.ZipFile, name: str, cache: dict, stack: tuple) -> list[str]:
    """data/minecraft/tags/item/<name>.json -> flat values (refs resolved)."""
    if name in cache:
        return cache[name]
    if name in stack:
        raise RuntimeError(f"tag cycle: {' -> '.join(stack + (name,))}")
    path = f"data/minecraft/tags/item/{name}.json"
    try:
        raw = json.loads(zf.read(path))
    except KeyError:
        print(f"  note: item tag {name} absent from the jar — skipped")
        cache[name] = []
        return []
    out: list[str] = []
    for v in raw.get("values", []):
        if v.startswith("#"):
            out.extend(load_item_tags(zf, v[1:].removeprefix("minecraft:"), cache, stack + (name,)))
        else:
            out.append(v.removeprefix("minecraft:"))
    cache[name] = out
    return out


def load_wire_order() -> tuple[dict[str, int], int]:
    """item_order.rs -> (name -> wire id, the table's length)."""
    src = ITEM_ORDER.read_text()
    body = src.split("[", 1)[1].rsplit("]", 1)[0]
    names = re.findall(r'"([a-z_0-9]+)"', body)
    return {n: i for i, n in enumerate(names)}, len(names)


def main() -> None:
    wire, n_items = load_wire_order()
    with zipfile.ZipFile(JAR) as zf:
        tag_cache: dict = {}
        # ---- THE SMELTING RECIPES (the jar's data rows) ----
        recipes: dict[str, tuple[str, int]] = {}
        for path in sorted(zf.namelist()):
            if not path.startswith("data/minecraft/recipe/") or not path.endswith(".json"):
                continue
            row = json.loads(zf.read(path))
            if row.get("type") != "minecraft:smelting":
                continue
            result = row["result"]["id"].removeprefix("minecraft:")
            count = int(row["result"].get("count", 1))
            cook = int(row.get("cookingtime", 200))  # the clinit's own default
            if cook != 200:
                print(f"  note: {path} cookingtime {cook} rides its row")
            # THE INGREDIENT'S OWN SHAPES (the 26.2 data pack's rows):
            # the plain item string ("minecraft:raw_iron"), the tag
            # string ("#minecraft:logs_that_burn" — the charcoal/glass/
            # leaf_litter rows speak), the item-list row (the copper
            # gear's own nugget walk — 12 strings). Every shape the
            # walk flattens to the member items; the tag strings the
            # jar's item tags answer.
            raw = row["ingredient"]
            members: list[str] = []
            if isinstance(raw, str):
                members = [raw]
            elif isinstance(raw, list):
                members = [v for v in raw if isinstance(v, str)]
            else:
                print(f"  note: {path} carries an exotic ingredient — skipped")
                continue
            for v in members:
                if v.startswith("#"):
                    tag = v[1:].removeprefix("minecraft:")
                    flat = load_item_tags(zf, tag, tag_cache, ())
                else:
                    flat = [v.removeprefix("minecraft:")]
                for name in flat:
                    if name in recipes and recipes[name] != (result, count):
                        print(f"  note: duplicate smelting row for {name} — first kept")
                        continue
                    recipes[name] = (result, count)

        # ---- THE FUEL VALUES (the javap walk over the jar's tags) ----
        fuel: dict[str, int] = {}
        cache: dict = {}
        for name, ticks in FUEL_ITEMS:
            fuel[name] = ticks
        for tag, ticks in FUEL_TAGS:
            for member in load_item_tags(zf, tag, cache, ()):
                fuel.setdefault(member, ticks)
        for member in load_item_tags(zf, FUEL_REMOVE_TAG, cache, ()):
            if fuel.pop(member, None) is not None:
                pass  # the removal the tail speaks
            else:
                print(f"  note: remove-tag member {member} was never a fuel")

    # ---- THE WIRE-ID PROJECTION ----
    smelt_ids: dict[int, tuple[int, int]] = {}
    for name, (result, count) in recipes.items():
        i = wire.get(name)
        r = wire.get(result)
        if i is None or r is None:
            print(f"  note: smelting row {name} -> {result} outside the wire order — skipped")
            continue
        smelt_ids[i] = (r, count)
    fuel_ids: dict[int, int] = {}
    for name, ticks in fuel.items():
        i = wire.get(name)
        if i is None:
            print(f"  note: fuel {name} outside the wire order — skipped")
            continue
        fuel_ids[i] = ticks

    lines: list[str] = []
    a = lines.append
    a("// GENERATED from the client.jar by scripts/extract_furnace_data.py —")
    a("// the furnace's own data tables the offline smelting sim reads:")
    a("// the smelting rows (data/minecraft/recipe/*.json, type")
    a("// minecraft:smelting — the cookingtime default 200 the")
    a("// SmeltingRecipe clinit's `sipush 200` codec default speaks) and")
    a("// the fuel rows (javap FuelValues.vanillaBurnTimes — the base 200")
    a("// BURN_TIME_STANDARD walk, the tag rows the jar's item tags")
    a("// answer, the tail remove(NON_FLAMMABLE_WOOD) the stems lose).")
    a("// Indexed by the ITEM WIRE ID (the ITEM_WIRE_ORDER index space);")
    a("// 0 = no row.")
    a(f"pub const ITEM_COUNT: usize = {n_items};")
    a("")
    a("/// The smelting result's wire id per ingredient id (0 = no row).")
    a("pub const SMELT_RESULT: [i32; ITEM_COUNT] = [")
    row = []
    for i in range(n_items):
        r = smelt_ids.get(i, (0, 0))[0]
        row.append(str(r))
        if len(row) == 20:
            a("    " + ", ".join(row) + ",")
            row = []
    if row:
        a("    " + ", ".join(row) + ",")
    a("];")
    a("")
    a("/// The smelting result's stack count per ingredient id (0 = no row).")
    a("pub const SMELT_RESULT_COUNT: [i32; ITEM_COUNT] = [")
    row = []
    for i in range(n_items):
        c = smelt_ids.get(i, (0, 0))[1]
        row.append(str(c))
        if len(row) == 20:
            a("    " + ", ".join(row) + ",")
            row = []
    if row:
        a("    " + ", ".join(row) + ",")
    a("];")
    a("")
    a("/// The fuel's burn duration in ticks per item id (0 = not fuel).")
    a("pub const FUEL_TICKS: [i32; ITEM_COUNT] = [")
    row = []
    for i in range(n_items):
        row.append(str(fuel_ids.get(i, 0)))
        if len(row) == 20:
            a("    " + ", ".join(row) + ",")
            row = []
    if row:
        a("    " + ", ".join(row) + ",")
    a("];")
    a("")
    a("/// The smelting row (the None the no-recipe arm keeps).")
    a("pub fn smelt_result(id: i32) -> Option<(i32, i32)> {")
    a("    let i = usize::try_from(id).ok()?;")
    a("    let r = *SMELT_RESULT.get(i)?;")
    a("    if r == 0 {")
    a("        return None;")
    a("    }")
    a("    Some((r, SMELT_RESULT_COUNT[i]))")
    a("}")
    a("")
    a("/// The fuel's burn duration (the 0 the not-fuel arm keeps).")
    a("pub fn fuel_ticks(id: i32) -> i32 {")
    a("    usize::try_from(id).ok().and_then(|i| FUEL_TICKS.get(i).copied()).unwrap_or(0)")
    a("}")
    OUT.write_text("\n".join(lines) + "\n")
    print(f"wrote {OUT}: {n_items} wire ids, {len(smelt_ids)} smelting rows, {len(fuel_ids)} fuel rows")


if __name__ == "__main__":
    main()

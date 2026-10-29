#!/usr/bin/env python3
"""dev-196 — the enchantment registry generator.

Emits crates/mc-registry/src/generated/enchantments.rs from the SAME
data tables the python twin (scripts/ench_twin_196.py) carries — the
jar's own numbers (the 43 enchantment definitions, the 7 exclusive
sets, the 36-member table pool, the resolved enchantable families,
the enchantability values). ONE source of truth: the twin computes
the deterministic test expectations, this generator emits the Rust
tables the client walks read.
"""
import importlib.util
import json
import os
import zipfile

spec = importlib.util.spec_from_file_location(
    "twin", os.path.join(os.path.dirname(__file__), "ench_twin_196.py"))
twin = importlib.util.module_from_spec(spec)
spec.loader.exec_module(twin)

# THE TOOLTIP ORDER TAG (the dev-198 round — the jar's own
# `data/minecraft/tags/enchantment/tooltip_order.json` read LIVE, the
# single-source law the definitions keep): the ItemEnchantments.
# addToTooltip walk's first pass the tag order serves.
JAR = os.path.join(os.path.dirname(__file__), "..", ".mc2rs",
                   "classpath", "client.jar")
with zipfile.ZipFile(JAR) as z:
    TOOLTIP_ORDER = [
        n.split(":", 1)[1] for n in json.loads(
            z.read("data/minecraft/tags/enchantment/tooltip_order.json")
        )["values"]
    ]

OUT = os.path.join(os.path.dirname(__file__), "..", "crates", "mc-registry",
                   "src", "generated", "enchantments.rs")

HEAD = '''//! THE ENCHANTMENT REGISTRY (the dev-196 round — the jar's own
//! `data/minecraft/enchantment/*.json` definitions the 26.2 client.jar
//! serves, the tables the python twin `scripts/ench_twin_196.py` emits
//! — scripts/jar_work/enchant_select_196.txt the javap evidence
//! speaks). THE ORDER the `BuiltInRegistries.ENCHANTMENT` registry's
//! own — the paths' alphabetical sort the HolderSet.Named stream
//! preserves (the weighted pick's index walk the order rides, the
//! registry id the (ench, level) pairs carry).
//!
//! The COST curves ride the `Enchantment$Cost.calculate(l)` law —
//! `base + per_level_above_first * (l - 1)` the javap keeps. The
//! TABLE POOL the `#minecraft:in_enchanting_table` tag's resolution
//! wears (`= #minecraft:non_treasure` — the 7 treasure-only rows the
//! pool never seats). The FAMILIES the `data/minecraft/tags/item/
//! enchantable/*.json` trees resolved to plain item names (the
//! `#minecraft:` references the resolution expanded). The
//! ENCHANTABILITY the `Enchantable` component's per-item values (the
//! ToolMaterial/ArmorMaterial clinit walks jar-proven, the special
//! items the classic `Item.Properties` rows).

/// THE DEFINITION ROW (the `Enchantment$EnchantmentDefinition` record
/// the selection walk reads — the name the registry key's plain path
/// keeps, the weight the weighted pick rides, the cost curves the
/// level scan answers, the exclusive tag the compatibility law keys,
/// the supported/primary families the availability filter speaks).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnchDef {
    /// The registry key's plain name (the `minecraft:` prefix
    /// stripped — the description translate key's tail row).
    pub name: &'static str,
    /// The definition's `weight` (the weighted pick's whole law).
    pub weight: i32,
    /// The definition's `max_level` (the level scan's top walk).
    pub max_level: i32,
    /// The `min_cost` curve: (base, per_level_above_first).
    pub min_cost: (i32, i32),
    /// The `max_cost` curve: (base, per_level_above_first).
    pub max_cost: (i32, i32),
    /// The `exclusive_set` tag's family name (None = no exclusivity).
    pub exclusive: Option<&'static str>,
    /// The `supported_items` family name (the availability filter).
    pub supported: &'static str,
    /// The `primary_items` family name (None = the supported set
    /// alone the primary gate keeps).
    pub primary: Option<&'static str>,
}

impl EnchDef {
    /// THE `Cost.calculate(l)` LAW — `base + per * (l - 1)` the javap
    /// keeps (both curves the same fold ride).
    pub fn cost_at(curve: (i32, i32), level: i32) -> i32 {
        curve.0 + curve.1 * (level - 1)
    }
}
'''

FOOT = '''
/// THE EXCLUSIVE SET LAW (the `Enchantment.areCompatible` walk the
/// javap keeps — `a != b && !a.exclusiveSet.contains(b) &&
/// !b.exclusiveSet.contains(a)`; the tag the member names resolve
/// through [`EXCLUSIVE_SETS`]).
pub fn are_compatible(a: usize, b: usize) -> bool {
    if a == b {
        return false;
    }
    let (da, db) = (&ENCHANTMENTS[a], &ENCHANTMENTS[b]);
    if let Some(ta) = da.exclusive {
        if EXCLUSIVE_SETS.iter().any(|&(t, m)| t == ta && m.contains(&db.name)) {
            return false;
        }
    }
    if let Some(tb) = db.exclusive {
        if EXCLUSIVE_SETS.iter().any(|&(t, m)| t == tb && m.contains(&da.name)) {
            return false;
        }
    }
    true
}

/// THE REGISTRY ID's NAME (the index the (ench, level) pairs carry —
/// past-the-table ids the honest None answers).
pub fn enchantment_name(idx: usize) -> Option<&'static str> {
    ENCHANTMENTS.get(idx).map(|d| d.name)
}

/// THE NAME's REGISTRY ID (the alphabetical order the lookup's
/// linear walk answers — 43 rows the scan's cost never speaks).
pub fn enchantment_index(name: &str) -> Option<usize> {
    ENCHANTMENTS.iter().position(|d| d.name == name)
}

/// THE `#minecraft:in_enchanting_table` MEMBERSHIP (the pool the
/// `getEnchantmentList` stream serves — the non_treasure tag's 36
/// rows, the 7 treasure-only rows the pool never seats).
pub fn is_table_member(name: &str) -> bool {
    TABLE_MEMBERS.contains(&name)
}

/// THE ENCHANTABLE FAMILY'S MEMBERSHIP (the resolved tag tree the
/// plain item names answer).
pub fn item_in_family(item: &str, family: &str) -> bool {
    ENCHANTABLE_FAMILIES
        .iter()
        .find(|&&(f, _)| f == family)
        .map(|&(_, items)| items.contains(&item))
        .unwrap_or(false)
}

/// THE `Enchantable` COMPONENT VALUE (the item's own enchantability —
/// the tool/armor material clinit rows jar-proven, the specials the
/// classic `Item.Properties` rows keep, the MISSING rows the
/// component's absence the 0 answer speaks [the
/// `getEnchantmentCost` null gate the empty selection wears]).
pub fn item_enchantability(item: &str) -> i32 {
    ITEM_ENCHANTABILITY
        .iter()
        .find(|&&(n, _)| n == item)
        .map(|&(_, v)| v)
        .unwrap_or(0)
}

/// THE `isPrimaryItem` LAW (the availability filter's own pair —
/// `isSupportedItem && (primary.isEmpty() || stack.is(primary))` the
/// javap keeps; the BOOK bypass the lambda$0's second arm wears —
/// the caller's own `|| is_book` the walk answers).
pub fn enchantment_accepts_item(ench: &str, item: &str) -> bool {
    let Some(d) = ENCHANTMENTS.iter().find(|&&ref x| x.name == ench) else {
        return false;
    };
    if !item_in_family(item, d.supported) {
        return false;
    }
    match d.primary {
        None => true,
        Some(p) => item_in_family(item, p),
    }
}

/// THE TOOLTIP ORDER TAG (the dev-198 round — the jar's own
/// `data/minecraft/tags/enchantment/tooltip_order.json` members the
/// `ItemEnchantments.addToTooltip` walk banks: the FIRST pass the
/// tagged rows the tag order speaks [pc 30-83 the HolderSet iterator
/// the `getInt > 0` gate wears], the SECOND pass the untagged rows
/// the map's own order answers [pc 84-173]).
pub static TOOLTIP_ORDER: [&str; %d] = [
    %s,
];

/// THE ROW'S RANK (the tag order's own index — the untagged rows the
/// None answer keeps, the second pass the map order wears).
pub fn tooltip_rank(name: &str) -> Option<usize> {
    TOOLTIP_ORDER.iter().position(|&n| n == name)
}
''' % (len(TOOLTIP_ORDER), ",\n    ".join('"%s"' % n for n in TOOLTIP_ORDER))

lines = [HEAD, "/// THE 43 DEFINITIONS (the registry's alphabetical order — the\n/// json files' own names the sort keeps).\npub static ENCHANTMENTS: [EnchDef; %d] = [" % len(twin.DEFS)]
for d in twin.DEFS:
    name, w, ml, mb, mp, xb, xp, ex, sup, pri = d
    ex_s = f'Some("{ex}")' if ex else "None"
    pri_s = f'Some("{pri}")' if pri else "None"
    lines.append(f'    EnchDef {{ name: "{name}", weight: {w}, max_level: {ml}, '
                 f'min_cost: ({mb}, {mp}), max_cost: ({xb}, {xp}), '
                 f'exclusive: {ex_s}, supported: "{sup}", primary: {pri_s} }},')
lines.append("];")
lines.append("")
lines.append("/// THE EXCLUSIVE SETS (the 7 tags the jar's own\n"
             "/// `data/minecraft/tags/enchantment/exclusive_set/*.json` banks —\n"
             "/// the member names the compatibility law's containment speaks).\n"
             "pub static EXCLUSIVE_SETS: [(&str, &[&str]); %d] = [" % len(twin.EXCLUSIVE_SETS))
for tag, members in twin.EXCLUSIVE_SETS.items():
    ms = ", ".join(f'"{m}"' for m in members)
    lines.append(f'    ("{tag}", &[{ms}]),')
lines.append("];")
lines.append("")
lines.append("/// THE TABLE POOL (the `#minecraft:in_enchanting_table` tag's\n"
             "/// resolution — the `#minecraft:non_treasure` members the stream\n"
             "/// serves, the REGISTRY ORDER the iteration preserves).\n"
             "pub static TABLE_MEMBERS: [&str; %d] = [" % len(twin.TABLE_MEMBERS))
lines.append("    " + ", ".join(f'"{n}"' for n in twin.TABLE_MEMBERS) + ",")
lines.append("];")
lines.append("")
lines.append("/// THE CURSE TAG (the `#minecraft:curse` members the jar's own\n"
             "/// `data/minecraft/tags/enchantment/curse.json` banks — the RED\n"
             "/// ink the fullname's own walk answers, the anvil/book lanes the\n"
             "/// tooltip rows serve).\n"
             "pub static CURSE_MEMBERS: [&str; %d] = [" % len(twin.CURSE_MEMBERS))
lines.append("    " + ", ".join(f'"{n}"' for n in twin.CURSE_MEMBERS) + ",")
lines.append("];")
lines.append("")
lines.append("/// THE ENCHANTABLE FAMILIES (the 22 tags resolved — the `#minecraft:`\n"
             "/// references the expansion carried, the direct items the tail rows\n"
             "/// keep; the order the tag file names' alphabetical sort banks).\n"
             "pub static ENCHANTABLE_FAMILIES: [(&str, &[&str]); %d] = [" % len(twin.FAMILIES))
for fam in sorted(twin.FAMILIES):
    items = sorted(twin.FAMILIES[fam])
    lines.append(f'    ("{fam}", &[{", ".join(chr(34)+i+chr(34) for i in items)}]),')
lines.append("];")
lines.append("")
lines.append("/// THE ENCHANTABILITY TABLE (the `Enchantable` component's values —\n"
             "/// the tools the ToolMaterial rows speak, the armor the\n"
             "/// ArmorMaterials rows, the specials the classic `Item.Properties`\n"
             "/// keeps; the MISSING items the 0 the component-absence answers).\n"
             "pub static ITEM_ENCHANTABILITY: [(&str, i32); %d] = [" % len(twin.ENCHANTABILITY))
for item, v in sorted(twin.ENCHANTABILITY.items()):
    lines.append(f'    ("{item}", {v}),')
lines.append("];")
lines.append(FOOT)

with open(OUT, "w") as f:
    f.write("\n".join(lines) + "\n")
print("wrote", OUT, len(lines), "lines")

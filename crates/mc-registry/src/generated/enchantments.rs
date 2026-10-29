//! THE ENCHANTMENT REGISTRY (the dev-196 round — the jar's own
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

/// THE 43 DEFINITIONS (the registry's alphabetical order — the
/// json files' own names the sort keeps).
pub static ENCHANTMENTS: [EnchDef; 43] = [
    EnchDef { name: "aqua_affinity", weight: 2, max_level: 1, min_cost: (1, 0), max_cost: (41, 0), exclusive: None, supported: "head_armor", primary: None },
    EnchDef { name: "bane_of_arthropods", weight: 5, max_level: 5, min_cost: (5, 8), max_cost: (25, 8), exclusive: Some("damage"), supported: "weapon", primary: Some("melee_weapon") },
    EnchDef { name: "binding_curse", weight: 1, max_level: 1, min_cost: (25, 0), max_cost: (50, 0), exclusive: None, supported: "equippable", primary: None },
    EnchDef { name: "blast_protection", weight: 2, max_level: 4, min_cost: (5, 8), max_cost: (13, 8), exclusive: Some("armor"), supported: "armor", primary: None },
    EnchDef { name: "breach", weight: 2, max_level: 4, min_cost: (15, 9), max_cost: (65, 9), exclusive: Some("damage"), supported: "mace", primary: None },
    EnchDef { name: "channeling", weight: 1, max_level: 1, min_cost: (25, 0), max_cost: (50, 0), exclusive: None, supported: "trident", primary: None },
    EnchDef { name: "density", weight: 5, max_level: 5, min_cost: (5, 8), max_cost: (25, 8), exclusive: Some("damage"), supported: "mace", primary: None },
    EnchDef { name: "depth_strider", weight: 2, max_level: 3, min_cost: (10, 10), max_cost: (25, 10), exclusive: Some("boots"), supported: "foot_armor", primary: None },
    EnchDef { name: "efficiency", weight: 10, max_level: 5, min_cost: (1, 10), max_cost: (51, 10), exclusive: None, supported: "mining", primary: None },
    EnchDef { name: "feather_falling", weight: 5, max_level: 4, min_cost: (5, 6), max_cost: (11, 6), exclusive: None, supported: "foot_armor", primary: None },
    EnchDef { name: "fire_aspect", weight: 2, max_level: 2, min_cost: (10, 20), max_cost: (60, 20), exclusive: None, supported: "fire_aspect", primary: Some("melee_weapon") },
    EnchDef { name: "fire_protection", weight: 5, max_level: 4, min_cost: (10, 8), max_cost: (18, 8), exclusive: Some("armor"), supported: "armor", primary: None },
    EnchDef { name: "flame", weight: 2, max_level: 1, min_cost: (20, 0), max_cost: (50, 0), exclusive: None, supported: "bow", primary: None },
    EnchDef { name: "fortune", weight: 2, max_level: 3, min_cost: (15, 9), max_cost: (65, 9), exclusive: Some("mining"), supported: "mining_loot", primary: None },
    EnchDef { name: "frost_walker", weight: 2, max_level: 2, min_cost: (10, 10), max_cost: (25, 10), exclusive: Some("boots"), supported: "foot_armor", primary: None },
    EnchDef { name: "impaling", weight: 2, max_level: 5, min_cost: (1, 8), max_cost: (21, 8), exclusive: Some("damage"), supported: "trident", primary: None },
    EnchDef { name: "infinity", weight: 1, max_level: 1, min_cost: (20, 0), max_cost: (50, 0), exclusive: Some("bow"), supported: "bow", primary: None },
    EnchDef { name: "knockback", weight: 5, max_level: 2, min_cost: (5, 20), max_cost: (55, 20), exclusive: None, supported: "melee_weapon", primary: None },
    EnchDef { name: "looting", weight: 2, max_level: 3, min_cost: (15, 9), max_cost: (65, 9), exclusive: None, supported: "melee_weapon", primary: None },
    EnchDef { name: "loyalty", weight: 5, max_level: 3, min_cost: (12, 7), max_cost: (50, 0), exclusive: Some("riptide"), supported: "trident", primary: None },
    EnchDef { name: "luck_of_the_sea", weight: 2, max_level: 3, min_cost: (15, 9), max_cost: (65, 9), exclusive: None, supported: "fishing", primary: None },
    EnchDef { name: "lunge", weight: 5, max_level: 3, min_cost: (5, 8), max_cost: (25, 8), exclusive: None, supported: "lunge", primary: None },
    EnchDef { name: "lure", weight: 2, max_level: 3, min_cost: (15, 9), max_cost: (65, 9), exclusive: None, supported: "fishing", primary: None },
    EnchDef { name: "mending", weight: 2, max_level: 1, min_cost: (25, 25), max_cost: (75, 25), exclusive: None, supported: "durability", primary: None },
    EnchDef { name: "multishot", weight: 2, max_level: 1, min_cost: (20, 0), max_cost: (50, 0), exclusive: Some("crossbow"), supported: "crossbow", primary: None },
    EnchDef { name: "piercing", weight: 10, max_level: 4, min_cost: (1, 10), max_cost: (50, 0), exclusive: Some("crossbow"), supported: "crossbow", primary: None },
    EnchDef { name: "power", weight: 10, max_level: 5, min_cost: (1, 10), max_cost: (16, 10), exclusive: None, supported: "bow", primary: None },
    EnchDef { name: "projectile_protection", weight: 5, max_level: 4, min_cost: (3, 6), max_cost: (9, 6), exclusive: Some("armor"), supported: "armor", primary: None },
    EnchDef { name: "protection", weight: 10, max_level: 4, min_cost: (1, 11), max_cost: (12, 11), exclusive: Some("armor"), supported: "armor", primary: None },
    EnchDef { name: "punch", weight: 2, max_level: 2, min_cost: (12, 20), max_cost: (37, 20), exclusive: None, supported: "bow", primary: None },
    EnchDef { name: "quick_charge", weight: 5, max_level: 3, min_cost: (12, 20), max_cost: (50, 0), exclusive: None, supported: "crossbow", primary: None },
    EnchDef { name: "respiration", weight: 2, max_level: 3, min_cost: (10, 10), max_cost: (40, 10), exclusive: None, supported: "head_armor", primary: None },
    EnchDef { name: "riptide", weight: 2, max_level: 3, min_cost: (17, 7), max_cost: (50, 0), exclusive: Some("riptide"), supported: "trident", primary: None },
    EnchDef { name: "sharpness", weight: 10, max_level: 5, min_cost: (1, 11), max_cost: (21, 11), exclusive: Some("damage"), supported: "sharp_weapon", primary: Some("melee_weapon") },
    EnchDef { name: "silk_touch", weight: 1, max_level: 1, min_cost: (15, 0), max_cost: (65, 0), exclusive: Some("mining"), supported: "mining_loot", primary: None },
    EnchDef { name: "smite", weight: 5, max_level: 5, min_cost: (5, 8), max_cost: (25, 8), exclusive: Some("damage"), supported: "weapon", primary: Some("melee_weapon") },
    EnchDef { name: "soul_speed", weight: 1, max_level: 3, min_cost: (10, 10), max_cost: (25, 10), exclusive: None, supported: "foot_armor", primary: None },
    EnchDef { name: "sweeping_edge", weight: 2, max_level: 3, min_cost: (5, 9), max_cost: (20, 9), exclusive: None, supported: "sweeping", primary: None },
    EnchDef { name: "swift_sneak", weight: 1, max_level: 3, min_cost: (25, 25), max_cost: (75, 25), exclusive: None, supported: "leg_armor", primary: None },
    EnchDef { name: "thorns", weight: 1, max_level: 3, min_cost: (10, 20), max_cost: (60, 20), exclusive: None, supported: "armor", primary: Some("chest_armor") },
    EnchDef { name: "unbreaking", weight: 5, max_level: 3, min_cost: (5, 8), max_cost: (55, 8), exclusive: None, supported: "durability", primary: None },
    EnchDef { name: "vanishing_curse", weight: 1, max_level: 1, min_cost: (25, 0), max_cost: (50, 0), exclusive: None, supported: "vanishing", primary: None },
    EnchDef { name: "wind_burst", weight: 2, max_level: 3, min_cost: (15, 9), max_cost: (65, 9), exclusive: None, supported: "mace", primary: None },
];

/// THE EXCLUSIVE SETS (the 7 tags the jar's own
/// `data/minecraft/tags/enchantment/exclusive_set/*.json` banks —
/// the member names the compatibility law's containment speaks).
pub static EXCLUSIVE_SETS: [(&str, &[&str]); 7] = [
    ("armor", &["protection", "blast_protection", "fire_protection", "projectile_protection"]),
    ("boots", &["frost_walker", "depth_strider"]),
    ("bow", &["infinity", "mending"]),
    ("crossbow", &["multishot", "piercing"]),
    ("damage", &["sharpness", "smite", "bane_of_arthropods", "impaling", "density", "breach"]),
    ("mining", &["fortune", "silk_touch"]),
    ("riptide", &["loyalty", "channeling"]),
];

/// THE TABLE POOL (the `#minecraft:in_enchanting_table` tag's
/// resolution — the `#minecraft:non_treasure` members the stream
/// serves, the REGISTRY ORDER the iteration preserves).
pub static TABLE_MEMBERS: [&str; 36] = [
    "aqua_affinity", "bane_of_arthropods", "blast_protection", "breach", "channeling", "density", "depth_strider", "efficiency", "feather_falling", "fire_aspect", "fire_protection", "flame", "fortune", "impaling", "infinity", "knockback", "looting", "loyalty", "luck_of_the_sea", "lunge", "lure", "multishot", "piercing", "power", "projectile_protection", "protection", "punch", "quick_charge", "respiration", "riptide", "sharpness", "silk_touch", "smite", "sweeping_edge", "thorns", "unbreaking",
];

/// THE ENCHANTABLE FAMILIES (the 22 tags resolved — the `#minecraft:`
/// references the expansion carried, the direct items the tail rows
/// keep; the order the tag file names' alphabetical sort banks).
pub static ENCHANTABLE_FAMILIES: [(&str, &[&str]); 21] = [
    ("armor", &["chainmail_boots", "chainmail_chestplate", "chainmail_helmet", "chainmail_leggings", "copper_boots", "copper_chestplate", "copper_helmet", "copper_leggings", "diamond_boots", "diamond_chestplate", "diamond_helmet", "diamond_leggings", "golden_boots", "golden_chestplate", "golden_helmet", "golden_leggings", "iron_boots", "iron_chestplate", "iron_helmet", "iron_leggings", "leather_boots", "leather_chestplate", "leather_helmet", "leather_leggings", "netherite_boots", "netherite_chestplate", "netherite_helmet", "netherite_leggings", "turtle_helmet"]),
    ("bow", &["bow"]),
    ("chest_armor", &["chainmail_chestplate", "copper_chestplate", "diamond_chestplate", "golden_chestplate", "iron_chestplate", "leather_chestplate", "netherite_chestplate"]),
    ("crossbow", &["crossbow"]),
    ("durability", &["bow", "brush", "carrot_on_a_stick", "chainmail_boots", "chainmail_chestplate", "chainmail_helmet", "chainmail_leggings", "copper_axe", "copper_boots", "copper_chestplate", "copper_helmet", "copper_hoe", "copper_leggings", "copper_pickaxe", "copper_shovel", "copper_spear", "copper_sword", "crossbow", "diamond_axe", "diamond_boots", "diamond_chestplate", "diamond_helmet", "diamond_hoe", "diamond_leggings", "diamond_pickaxe", "diamond_shovel", "diamond_spear", "diamond_sword", "elytra", "fishing_rod", "flint_and_steel", "golden_axe", "golden_boots", "golden_chestplate", "golden_helmet", "golden_hoe", "golden_leggings", "golden_pickaxe", "golden_shovel", "golden_spear", "golden_sword", "iron_axe", "iron_boots", "iron_chestplate", "iron_helmet", "iron_hoe", "iron_leggings", "iron_pickaxe", "iron_shovel", "iron_spear", "iron_sword", "leather_boots", "leather_chestplate", "leather_helmet", "leather_leggings", "mace", "netherite_axe", "netherite_boots", "netherite_chestplate", "netherite_helmet", "netherite_hoe", "netherite_leggings", "netherite_pickaxe", "netherite_shovel", "netherite_spear", "netherite_sword", "shears", "shield", "stone_axe", "stone_hoe", "stone_pickaxe", "stone_shovel", "stone_spear", "stone_sword", "trident", "turtle_helmet", "warped_fungus_on_a_stick", "wooden_axe", "wooden_hoe", "wooden_pickaxe", "wooden_shovel", "wooden_spear", "wooden_sword"]),
    ("equippable", &["carved_pumpkin", "chainmail_boots", "chainmail_chestplate", "chainmail_helmet", "chainmail_leggings", "copper_boots", "copper_chestplate", "copper_helmet", "copper_leggings", "creeper_head", "diamond_boots", "diamond_chestplate", "diamond_helmet", "diamond_leggings", "dragon_head", "elytra", "golden_boots", "golden_chestplate", "golden_helmet", "golden_leggings", "iron_boots", "iron_chestplate", "iron_helmet", "iron_leggings", "leather_boots", "leather_chestplate", "leather_helmet", "leather_leggings", "netherite_boots", "netherite_chestplate", "netherite_helmet", "netherite_leggings", "piglin_head", "player_head", "skeleton_skull", "turtle_helmet", "wither_skeleton_skull", "zombie_head"]),
    ("fire_aspect", &["copper_spear", "copper_sword", "diamond_spear", "diamond_sword", "golden_spear", "golden_sword", "iron_spear", "iron_sword", "netherite_spear", "netherite_sword", "stone_spear", "stone_sword", "wooden_spear", "wooden_sword"]),
    ("fishing", &["fishing_rod"]),
    ("foot_armor", &["chainmail_boots", "copper_boots", "diamond_boots", "golden_boots", "iron_boots", "leather_boots", "netherite_boots"]),
    ("head_armor", &["chainmail_helmet", "copper_helmet", "diamond_helmet", "golden_helmet", "iron_helmet", "leather_helmet", "netherite_helmet", "turtle_helmet"]),
    ("leg_armor", &["chainmail_leggings", "copper_leggings", "diamond_leggings", "golden_leggings", "iron_leggings", "leather_leggings", "netherite_leggings"]),
    ("lunge", &["copper_spear", "diamond_spear", "golden_spear", "iron_spear", "netherite_spear", "stone_spear", "wooden_spear"]),
    ("mace", &["mace"]),
    ("melee_weapon", &["copper_spear", "copper_sword", "diamond_spear", "diamond_sword", "golden_spear", "golden_sword", "iron_spear", "iron_sword", "netherite_spear", "netherite_sword", "stone_spear", "stone_sword", "wooden_spear", "wooden_sword"]),
    ("mining", &["copper_axe", "copper_hoe", "copper_pickaxe", "copper_shovel", "diamond_axe", "diamond_hoe", "diamond_pickaxe", "diamond_shovel", "golden_axe", "golden_hoe", "golden_pickaxe", "golden_shovel", "iron_axe", "iron_hoe", "iron_pickaxe", "iron_shovel", "netherite_axe", "netherite_hoe", "netherite_pickaxe", "netherite_shovel", "shears", "stone_axe", "stone_hoe", "stone_pickaxe", "stone_shovel", "wooden_axe", "wooden_hoe", "wooden_pickaxe", "wooden_shovel"]),
    ("mining_loot", &["copper_axe", "copper_hoe", "copper_pickaxe", "copper_shovel", "diamond_axe", "diamond_hoe", "diamond_pickaxe", "diamond_shovel", "golden_axe", "golden_hoe", "golden_pickaxe", "golden_shovel", "iron_axe", "iron_hoe", "iron_pickaxe", "iron_shovel", "netherite_axe", "netherite_hoe", "netherite_pickaxe", "netherite_shovel", "stone_axe", "stone_hoe", "stone_pickaxe", "stone_shovel", "wooden_axe", "wooden_hoe", "wooden_pickaxe", "wooden_shovel"]),
    ("sharp_weapon", &["copper_axe", "copper_spear", "copper_sword", "diamond_axe", "diamond_spear", "diamond_sword", "golden_axe", "golden_spear", "golden_sword", "iron_axe", "iron_spear", "iron_sword", "netherite_axe", "netherite_spear", "netherite_sword", "stone_axe", "stone_spear", "stone_sword", "wooden_axe", "wooden_spear", "wooden_sword"]),
    ("sweeping", &["copper_sword", "diamond_sword", "golden_sword", "iron_sword", "netherite_sword", "stone_sword", "wooden_sword"]),
    ("trident", &["trident"]),
    ("vanishing", &["bow", "brush", "carrot_on_a_stick", "carved_pumpkin", "chainmail_boots", "chainmail_chestplate", "chainmail_helmet", "chainmail_leggings", "compass", "copper_axe", "copper_boots", "copper_chestplate", "copper_helmet", "copper_hoe", "copper_leggings", "copper_pickaxe", "copper_shovel", "copper_spear", "copper_sword", "creeper_head", "crossbow", "diamond_axe", "diamond_boots", "diamond_chestplate", "diamond_helmet", "diamond_hoe", "diamond_leggings", "diamond_pickaxe", "diamond_shovel", "diamond_spear", "diamond_sword", "dragon_head", "elytra", "fishing_rod", "flint_and_steel", "golden_axe", "golden_boots", "golden_chestplate", "golden_helmet", "golden_hoe", "golden_leggings", "golden_pickaxe", "golden_shovel", "golden_spear", "golden_sword", "iron_axe", "iron_boots", "iron_chestplate", "iron_helmet", "iron_hoe", "iron_leggings", "iron_pickaxe", "iron_shovel", "iron_spear", "iron_sword", "leather_boots", "leather_chestplate", "leather_helmet", "leather_leggings", "mace", "netherite_axe", "netherite_boots", "netherite_chestplate", "netherite_helmet", "netherite_hoe", "netherite_leggings", "netherite_pickaxe", "netherite_shovel", "netherite_spear", "netherite_sword", "piglin_head", "player_head", "shears", "shield", "skeleton_skull", "stone_axe", "stone_hoe", "stone_pickaxe", "stone_shovel", "stone_spear", "stone_sword", "trident", "turtle_helmet", "warped_fungus_on_a_stick", "wither_skeleton_skull", "wooden_axe", "wooden_hoe", "wooden_pickaxe", "wooden_shovel", "wooden_spear", "wooden_sword", "zombie_head"]),
    ("weapon", &["copper_axe", "copper_spear", "copper_sword", "diamond_axe", "diamond_spear", "diamond_sword", "golden_axe", "golden_spear", "golden_sword", "iron_axe", "iron_spear", "iron_sword", "mace", "netherite_axe", "netherite_spear", "netherite_sword", "stone_axe", "stone_spear", "stone_sword", "wooden_axe", "wooden_spear", "wooden_sword"]),
];

/// THE ENCHANTABILITY TABLE (the `Enchantable` component's values —
/// the tools the ToolMaterial rows speak, the armor the
/// ArmorMaterials rows, the specials the classic `Item.Properties`
/// keeps; the MISSING items the 0 the component-absence answers).
pub static ITEM_ENCHANTABILITY: [(&str, i32); 84] = [
    ("book", 1),
    ("bow", 1),
    ("brush", 1),
    ("carrot_on_a_stick", 1),
    ("chainmail_boots", 12),
    ("chainmail_chestplate", 12),
    ("chainmail_helmet", 12),
    ("chainmail_leggings", 12),
    ("copper_axe", 13),
    ("copper_boots", 8),
    ("copper_chestplate", 8),
    ("copper_helmet", 8),
    ("copper_hoe", 13),
    ("copper_leggings", 8),
    ("copper_pickaxe", 13),
    ("copper_shovel", 13),
    ("copper_spear", 13),
    ("copper_sword", 13),
    ("crossbow", 1),
    ("diamond_axe", 10),
    ("diamond_boots", 10),
    ("diamond_chestplate", 10),
    ("diamond_helmet", 10),
    ("diamond_hoe", 10),
    ("diamond_leggings", 10),
    ("diamond_pickaxe", 10),
    ("diamond_shovel", 10),
    ("diamond_spear", 10),
    ("diamond_sword", 10),
    ("elytra", 1),
    ("fishing_rod", 1),
    ("flint_and_steel", 1),
    ("golden_axe", 22),
    ("golden_boots", 25),
    ("golden_chestplate", 25),
    ("golden_helmet", 25),
    ("golden_hoe", 22),
    ("golden_leggings", 25),
    ("golden_pickaxe", 22),
    ("golden_shovel", 22),
    ("golden_spear", 22),
    ("golden_sword", 22),
    ("iron_axe", 14),
    ("iron_boots", 9),
    ("iron_chestplate", 9),
    ("iron_helmet", 9),
    ("iron_hoe", 14),
    ("iron_leggings", 9),
    ("iron_pickaxe", 14),
    ("iron_shovel", 14),
    ("iron_spear", 14),
    ("iron_sword", 14),
    ("leather_boots", 15),
    ("leather_chestplate", 15),
    ("leather_helmet", 15),
    ("leather_leggings", 15),
    ("mace", 15),
    ("netherite_axe", 15),
    ("netherite_boots", 15),
    ("netherite_chestplate", 15),
    ("netherite_helmet", 15),
    ("netherite_hoe", 15),
    ("netherite_leggings", 15),
    ("netherite_pickaxe", 15),
    ("netherite_shovel", 15),
    ("netherite_spear", 15),
    ("netherite_sword", 15),
    ("shears", 1),
    ("shield", 1),
    ("stone_axe", 5),
    ("stone_hoe", 5),
    ("stone_pickaxe", 5),
    ("stone_shovel", 5),
    ("stone_spear", 5),
    ("stone_sword", 5),
    ("trident", 1),
    ("turtle_helmet", 9),
    ("warped_fungus_on_a_stick", 1),
    ("wooden_axe", 15),
    ("wooden_hoe", 15),
    ("wooden_pickaxe", 15),
    ("wooden_shovel", 15),
    ("wooden_spear", 15),
    ("wooden_sword", 15),
];

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


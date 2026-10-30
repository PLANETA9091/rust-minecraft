//! THE BREWING STAND'S OWN OFFLINE CHEMISTRY SIM (javap 26.2
//! BrewingStandBlockEntity — the serverTick walk the block entity
//! speaks every game tick, the same order the bytecode keeps; the
//! doBrew/isBrewable/mix lanes the PotionBrewing walk serves). The
//! OFFLINE stand-in's own engine: the opened stand's block entity
//! the fresh-menu stand-in retires — the blaze powder the fuel
//! twenty refills, the 400-tick count the bubbles and the arrow
//! wear, the finish the ingredient spends. The wire lane never
//! rides (the live menu the server's own words drive).
//!
//! THE WORDS' OWN SHAPE: the bank's three words carry the BE's own
//! fields — 0 the `brewTime` int (the ContainerData slot 0 the
//! DATA_BREW_TIME constant banks), 1 the `fuel` int (the
//! ContainerData slot 1 the DATA_FUEL_USES constant banks), 2 the
//! BE's own `ingredient` Item field the count arm compares (the
//! brew START's own putfield the bytecode speaks — NOT a
//! ContainerData slot, the field the vanilla `stand.ingredient`
//! wears; the model banks it the words' third cell the closed
//! cells' memory keeps).
//!
//! THE KINDS' OWN LANE (the autopush-339 round — the
//! POTION_CONTENTS component the (id, count) pair the seat bank's
//! parallel kinds ride): javap PotionBrewing the 26.2 jar banks —
//! the mix() walk pc 26-34 the contents Optional the EMPTY fold
//! the early return wears (the contents-less stack the walk NEVER
//! transforms), the pc 35-118 the CONTAINER scan the first speaks
//! (the new item the `PotionContents.createItemStack(mix.to,
//! contents.get())` lays — the SAME kind rides), the pc 119-199
//! the potion scan the second (the `createItemStack(potion.
//! getItem(), mix.to)` the new kind lays — the item unchanged),
//! the no-row tail the same stack keeps. THE KIND the potion
//! REGISTRY row the Holder identity speaks — the Potions clinit
//! the PotionIds keys' order the registration wears (46 rows, the
//! scripts/jar_work/potions_clinit_339.txt + potion_ids_339.txt
//! the evidence serves), the addVanillaMixes list the builder's
//! own order keeps (scripts/jar_work/brewing_mixes_309.txt the
//! walk banks, the Builder.addStartMix the javap pair proves:
//! `addMix(WATER, item, MUNDANE)` + `addMix(AWKWARD, item, potion)`
//! the TWO rows the start mixes wear).

/// THE DATA WORDS' OWN SEATS (javap 26.2 BrewingStandBlockEntity —
/// the `DATA_BREW_TIME = 0` / `DATA_FUEL_USES = 1` /
/// `NUM_DATA_VALUES = 2` constants the clinit banks; the third word
/// the BE's own `ingredient` Item field the model banks).
pub const DATA_BREW_TIME: usize = 0;
pub const DATA_FUEL_USES: usize = 1;
pub const INGREDIENT_WORD: usize = 2;

/// THE BREW TOTAL'S OWN CONSTANT (javap serverTick pc 193-198 — the
/// `sipush 400` putfield the START arm lays; 26.2 keeps NO
/// totalBrewTime field — the total the constant alone speaks, the
/// arrow's 400.0f divisor the screen walk re-wears).
pub const BREW_TOTAL_TICKS: i32 = 400;

/// THE FUEL USES' OWN CONSTANT (javap `FUEL_USES = 20` the clinit
/// banks — the refill arm's `bipush 20` putfield the same number
/// wears; one blaze powder TWENTY brew starts buy).
pub const FUEL_USES: i32 = 20;

/// THE BUBBLES' OWN LENGTH TABLE (javap 26.2 BrewingStandScreen
/// static{} pc 32-71 — the `newarray int` 7 the clinit fills, the
/// BUBBLELENGTHS the bubbles walk indexes `[brewTime / 2 % 7]`).
pub const BUBBLE_LENGTHS: [i32; 7] = [29, 24, 20, 16, 11, 6, 0];

/// THE POTION KIND'S OWN EMPTY (the `PotionContents.EMPTY` the
/// contents-less stack wears — the Optional the mix walk pc 26-34
/// the early return answers; the kind the model NEVER carries the
/// seat bank's own sentinel speaks). NOT a registry id — the
/// water potion the kind 0 wears, the -1 the absence keeps.
pub const POTION_KIND_NONE: i32 = -1;

/// THE POTION REGISTRY'S OWN ROWS (javap 26.2 Potions.<clinit> —
/// the PotionIds ResourceKey fields the registration order IS the
/// registry id order the MappedRegistry's sequential assignment
/// speaks; the 46 rows scripts/jar_work/potion_ids_339.txt the
/// field order banks, the potions_clinit_339.txt the putstatic
/// sequence the same order wears).
pub const POTION_KIND_NAMES: [&str; 46] = [
    "water",
    "mundane",
    "thick",
    "awkward",
    "night_vision",
    "long_night_vision",
    "invisibility",
    "long_invisibility",
    "leaping",
    "long_leaping",
    "strong_leaping",
    "fire_resistance",
    "long_fire_resistance",
    "swiftness",
    "long_swiftness",
    "strong_swiftness",
    "slowness",
    "long_slowness",
    "strong_slowness",
    "turtle_master",
    "long_turtle_master",
    "strong_turtle_master",
    "water_breathing",
    "long_water_breathing",
    "healing",
    "strong_healing",
    "harming",
    "strong_harming",
    "poison",
    "long_poison",
    "strong_poison",
    "regeneration",
    "long_regeneration",
    "strong_regeneration",
    "strength",
    "long_strength",
    "strong_strength",
    "weakness",
    "long_weakness",
    "luck",
    "slow_falling",
    "long_slow_falling",
    "wind_charged",
    "weaving",
    "oozing",
    "infested",
];

/// THE KIND ID'S OWN LOOKUP (the names table the index answers —
/// the registry row the name speaks).
pub fn potion_kind_by_name(name: &str) -> Option<i32> {
    POTION_KIND_NAMES
        .iter()
        .position(|n| *n == name)
        .map(|p| p as i32)
}

/// THE REGISTRY'S OWN KIND SEATS (the POTION_KIND_NAMES the index
/// pins — the mix table's rows the const expressions these wear;
/// the luck potion the row 39 the mixes never name).
pub const POTION_WATER: i32 = 0;
pub const POTION_MUNDANE: i32 = 1;
pub const POTION_THICK: i32 = 2;
pub const POTION_AWKWARD: i32 = 3;
pub const POTION_NIGHT_VISION: i32 = 4;
pub const POTION_LONG_NIGHT_VISION: i32 = 5;
pub const POTION_INVISIBILITY: i32 = 6;
pub const POTION_LONG_INVISIBILITY: i32 = 7;
pub const POTION_LEAPING: i32 = 8;
pub const POTION_LONG_LEAPING: i32 = 9;
pub const POTION_STRONG_LEAPING: i32 = 10;
pub const POTION_FIRE_RESISTANCE: i32 = 11;
pub const POTION_LONG_FIRE_RESISTANCE: i32 = 12;
pub const POTION_SWIFTNESS: i32 = 13;
pub const POTION_LONG_SWIFTNESS: i32 = 14;
pub const POTION_STRONG_SWIFTNESS: i32 = 15;
pub const POTION_SLOWNESS: i32 = 16;
pub const POTION_LONG_SLOWNESS: i32 = 17;
pub const POTION_STRONG_SLOWNESS: i32 = 18;
pub const POTION_TURTLE_MASTER: i32 = 19;
pub const POTION_LONG_TURTLE_MASTER: i32 = 20;
pub const POTION_STRONG_TURTLE_MASTER: i32 = 21;
pub const POTION_WATER_BREATHING: i32 = 22;
pub const POTION_LONG_WATER_BREATHING: i32 = 23;
pub const POTION_HEALING: i32 = 24;
pub const POTION_STRONG_HEALING: i32 = 25;
pub const POTION_HARMING: i32 = 26;
pub const POTION_STRONG_HARMING: i32 = 27;
pub const POTION_POISON: i32 = 28;
pub const POTION_LONG_POISON: i32 = 29;
pub const POTION_STRONG_POISON: i32 = 30;
pub const POTION_REGENERATION: i32 = 31;
pub const POTION_LONG_REGENERATION: i32 = 32;
pub const POTION_STRONG_REGENERATION: i32 = 33;
pub const POTION_STRENGTH: i32 = 34;
pub const POTION_LONG_STRENGTH: i32 = 35;
pub const POTION_STRONG_STRENGTH: i32 = 36;
pub const POTION_WEAKNESS: i32 = 37;
pub const POTION_LONG_WEAKNESS: i32 = 38;
pub const POTION_LUCK: i32 = 39;
pub const POTION_SLOW_FALLING: i32 = 40;
pub const POTION_LONG_SLOW_FALLING: i32 = 41;
pub const POTION_WIND_CHARGED: i32 = 42;
pub const POTION_WEAVING: i32 = 43;
pub const POTION_OOZING: i32 = 44;
pub const POTION_INFESTED: i32 = 45;

/// THE POTION MIXES' OWN TABLE (javap addVanillaMixes the 309
/// round banks — the `Builder.addMix(from, ingredient, to)` rows
/// the builder's list order keeps, the `addStartMix(item, potion)`
/// the Builder bytecode pc 19-36 the TWO-row expansion wears [the
/// water + item the MUNDANE keeps, the awkward + item the potion
/// wears]; 63 rows scripts/mix_rows_339.py the evidence the same
/// order machine-parses, the first matching row the mix walk
/// speaks). `(from_kind, ingredient_item_id, to_kind)`.
pub const POTION_MIXES: [(i32, i32, i32); 63] = [
    (
        POTION_WATER,
        crate::world::ITEM_GLOWSTONE_DUST,
        POTION_THICK,
    ),
    (POTION_WATER, crate::world::ITEM_REDSTONE, POTION_MUNDANE),
    (POTION_WATER, crate::world::ITEM_NETHER_WART, POTION_AWKWARD),
    (POTION_WATER, crate::world::ITEM_BREEZE_ROD, POTION_MUNDANE),
    (
        POTION_AWKWARD,
        crate::world::ITEM_BREEZE_ROD,
        POTION_WIND_CHARGED,
    ),
    (POTION_WATER, crate::world::ITEM_SLIME_BLOCK, POTION_MUNDANE),
    (
        POTION_AWKWARD,
        crate::world::ITEM_SLIME_BLOCK,
        POTION_OOZING,
    ),
    (POTION_WATER, crate::world::ITEM_STONE, POTION_MUNDANE),
    (POTION_AWKWARD, crate::world::ITEM_STONE, POTION_INFESTED),
    (POTION_WATER, crate::world::ITEM_COBWEB, POTION_MUNDANE),
    (POTION_AWKWARD, crate::world::ITEM_COBWEB, POTION_WEAVING),
    (
        POTION_AWKWARD,
        crate::world::ITEM_GOLDEN_CARROT,
        POTION_NIGHT_VISION,
    ),
    (
        POTION_NIGHT_VISION,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_NIGHT_VISION,
    ),
    (
        POTION_NIGHT_VISION,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_INVISIBILITY,
    ),
    (
        POTION_LONG_NIGHT_VISION,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_LONG_INVISIBILITY,
    ),
    (
        POTION_INVISIBILITY,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_INVISIBILITY,
    ),
    (POTION_WATER, crate::world::ITEM_MAGMA_CREAM, POTION_MUNDANE),
    (
        POTION_AWKWARD,
        crate::world::ITEM_MAGMA_CREAM,
        POTION_FIRE_RESISTANCE,
    ),
    (
        POTION_FIRE_RESISTANCE,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_FIRE_RESISTANCE,
    ),
    (POTION_WATER, crate::world::ITEM_RABBIT_FOOT, POTION_MUNDANE),
    (
        POTION_AWKWARD,
        crate::world::ITEM_RABBIT_FOOT,
        POTION_LEAPING,
    ),
    (
        POTION_LEAPING,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_LEAPING,
    ),
    (
        POTION_LEAPING,
        crate::world::ITEM_GLOWSTONE_DUST,
        POTION_STRONG_LEAPING,
    ),
    (
        POTION_LEAPING,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_SLOWNESS,
    ),
    (
        POTION_LONG_LEAPING,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_LONG_SLOWNESS,
    ),
    (
        POTION_SLOWNESS,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_SLOWNESS,
    ),
    (
        POTION_SLOWNESS,
        crate::world::ITEM_GLOWSTONE_DUST,
        POTION_STRONG_SLOWNESS,
    ),
    (
        POTION_AWKWARD,
        crate::world::ITEM_TURTLE_HELMET,
        POTION_TURTLE_MASTER,
    ),
    (
        POTION_TURTLE_MASTER,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_TURTLE_MASTER,
    ),
    (
        POTION_TURTLE_MASTER,
        crate::world::ITEM_GLOWSTONE_DUST,
        POTION_STRONG_TURTLE_MASTER,
    ),
    (
        POTION_SWIFTNESS,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_SLOWNESS,
    ),
    (
        POTION_LONG_SWIFTNESS,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_LONG_SLOWNESS,
    ),
    (POTION_WATER, crate::world::ITEM_SUGAR, POTION_MUNDANE),
    (POTION_AWKWARD, crate::world::ITEM_SUGAR, POTION_SWIFTNESS),
    (
        POTION_SWIFTNESS,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_SWIFTNESS,
    ),
    (
        POTION_SWIFTNESS,
        crate::world::ITEM_GLOWSTONE_DUST,
        POTION_STRONG_SWIFTNESS,
    ),
    (
        POTION_AWKWARD,
        crate::world::ITEM_PUFFERFISH,
        POTION_WATER_BREATHING,
    ),
    (
        POTION_WATER_BREATHING,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_WATER_BREATHING,
    ),
    (
        POTION_WATER,
        crate::world::ITEM_GLISTERING_MELON_SLICE,
        POTION_MUNDANE,
    ),
    (
        POTION_AWKWARD,
        crate::world::ITEM_GLISTERING_MELON_SLICE,
        POTION_HEALING,
    ),
    (
        POTION_HEALING,
        crate::world::ITEM_GLOWSTONE_DUST,
        POTION_STRONG_HEALING,
    ),
    (
        POTION_HEALING,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_HARMING,
    ),
    (
        POTION_STRONG_HEALING,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_STRONG_HARMING,
    ),
    (
        POTION_HARMING,
        crate::world::ITEM_GLOWSTONE_DUST,
        POTION_STRONG_HARMING,
    ),
    (
        POTION_POISON,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_HARMING,
    ),
    (
        POTION_LONG_POISON,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_HARMING,
    ),
    (
        POTION_STRONG_POISON,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_STRONG_HARMING,
    ),
    (POTION_WATER, crate::world::ITEM_SPIDER_EYE, POTION_MUNDANE),
    (POTION_AWKWARD, crate::world::ITEM_SPIDER_EYE, POTION_POISON),
    (
        POTION_POISON,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_POISON,
    ),
    (
        POTION_POISON,
        crate::world::ITEM_GLOWSTONE_DUST,
        POTION_STRONG_POISON,
    ),
    (POTION_WATER, crate::world::ITEM_GHAST_TEAR, POTION_MUNDANE),
    (
        POTION_AWKWARD,
        crate::world::ITEM_GHAST_TEAR,
        POTION_REGENERATION,
    ),
    (
        POTION_REGENERATION,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_REGENERATION,
    ),
    (
        POTION_REGENERATION,
        crate::world::ITEM_GLOWSTONE_DUST,
        POTION_STRONG_REGENERATION,
    ),
    (
        POTION_WATER,
        crate::world::ITEM_BLAZE_POWDER,
        POTION_MUNDANE,
    ),
    (
        POTION_AWKWARD,
        crate::world::ITEM_BLAZE_POWDER,
        POTION_STRENGTH,
    ),
    (
        POTION_STRENGTH,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_STRENGTH,
    ),
    (
        POTION_STRENGTH,
        crate::world::ITEM_GLOWSTONE_DUST,
        POTION_STRONG_STRENGTH,
    ),
    (
        POTION_WATER,
        crate::world::ITEM_FERMENTED_SPIDER_EYE,
        POTION_WEAKNESS,
    ),
    (
        POTION_WEAKNESS,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_WEAKNESS,
    ),
    (
        POTION_AWKWARD,
        crate::world::ITEM_PHANTOM_MEMBRANE,
        POTION_SLOW_FALLING,
    ),
    (
        POTION_SLOW_FALLING,
        crate::world::ITEM_REDSTONE,
        POTION_LONG_SLOW_FALLING,
    ),
];

/// THE BREW TICK (javap BrewingStandBlockEntity.serverTick — the
/// walk in the bytecode's own order). `slots` the menu's five cells
/// (bottles 0..2, ingredient 3, fuel 4), `words` the bank's three
/// (brewTime, fuel, the start-ingredient id), `kinds` the seats'
/// POTION_CONTENTS kinds the parallel bank keeps (the bottle trio
/// the live cells wear — the ingredient/fuel seats the NONE keeps;
/// the [`POTION_KIND_NONE`] the contents-less stack speaks).
/// Returns whether the finish fired (the doBrew walk — the caller's
/// voice the levelEvent 1035 the client answers).
pub fn tick(slots: &mut [(i32, i32)], words: &mut [i32; 3], kinds: &mut [i32; 5]) -> bool {
    // THE FUEL REFILL ARM (pc 13-48): `fuel <= 0 &&
    // fuelStack.is(ItemTags.BREWING_FUEL)` — the blaze powder the
    // tag's lone member the canPlaceItem lane banks — the `fuel =
    // FUEL_USES` putfield the direct write keeps (NOT an add — the
    // bipush 20 the putfield wears), the `shrink(1)` the powder
    // spends, the setChanged the caller's own bank write answers.
    if words[DATA_FUEL_USES] <= 0 {
        if let Some(fuel) = slots.get_mut(4) {
            if fuel.1 > 0 && fuel.0 == crate::world::ITEM_BLAZE_POWDER {
                words[DATA_FUEL_USES] = FUEL_USES;
                fuel.1 -= 1;
                if fuel.1 <= 0 {
                    *fuel = (0, 0);
                }
            }
        }
    }
    // THE BREWABLE GATE (isBrewable walk — pc 49-60 the call speaks).
    let brewable = is_brewable(slots, kinds);
    // THE COUNTING READ (pc 62-74): `brewTime > 0` the flag keeps.
    let counting = words[DATA_BREW_TIME] > 0;
    // THE INGREDIENT STACK (pc 76-88): items.get(3) the walk reads.
    let ingredient = slots.get(3).copied().unwrap_or((0, 0));
    if counting {
        // THE COUNT ARM (pc 94-168): the decrement first, the finish
        // the zero equality speaks.
        words[DATA_BREW_TIME] -= 1;
        let done = words[DATA_BREW_TIME] == 0;
        if done && brewable {
            // pc 128-137 — the doBrew call the finish wears.
            do_brew(slots, kinds);
            return true;
        }
        // pc 140-159 — the keep/abort ladder: the brewable stand the
        // SAME-ingredient stack keeps (`ingredientStack.is(
        // stand.ingredient)` the item identity the start word
        // answers), the swapped/empty ingredient or the unbrewable
        // stand the `brewTime = 0` abort wears.
        let same = ingredient.1 > 0 && ingredient.0 == words[INGREDIENT_WORD];
        if !(brewable && same) {
            words[DATA_BREW_TIME] = 0;
        }
        return false;
    }
    // THE START ARM (pc 171-215): `brewable && fuel > 0` — the
    // `fuel--` spend, the `brewTime = 400` sipush, the
    // `ingredient = ingredientStack.getItem()` putfield the start
    // word banks.
    if brewable && words[DATA_FUEL_USES] > 0 {
        words[DATA_FUEL_USES] -= 1;
        words[DATA_BREW_TIME] = BREW_TOTAL_TICKS;
        words[INGREDIENT_WORD] = ingredient.0;
    }
    false
}

/// THE BREWABLE WALK (javap isBrewable — the private static the
/// serverTick pc 49-60 calls): the ingredient stack EMPTY the first
/// false wears, the not-an-ingredient stack the second keeps, the
/// three bottle seats the hasMix scan answers — the FIRST
/// mix-bearing bottle the true speaks, the all-empty/all-no-mix
/// trio the final false keeps. THE KINDS the bottle trio's own
/// bank the scan rides (the seat's kind the hasPotionMix walk
/// reads — the NONE the contents wall keeps).
pub fn is_brewable(slots: &[(i32, i32)], kinds: &[i32; 5]) -> bool {
    let Some(ingredient) = slots.get(3) else {
        return false;
    };
    if ingredient.1 <= 0 {
        return false;
    }
    if !crate::world::brewing_is_ingredient(ingredient.0) {
        return false;
    }
    for i in 0..3 {
        let Some(bottle) = slots.get(i) else {
            continue;
        };
        if bottle.1 <= 0 {
            continue; // pc 45-53 — the empty seat the scan skips
        }
        let kind = kinds.get(i).copied().unwrap_or(POTION_KIND_NONE);
        if has_mix(bottle.0, ingredient.0, kind) {
            return true;
        }
    }
    false
}

/// THE HAS-MIX WALK (javap PotionBrewing.hasMix — the `isContainer &&
/// (hasContainerMix || hasPotionMix)` pair the verdict speaks).
/// THE KIND the potion scan's own read wears (the seat's
/// POTION_CONTENTS the [`has_potion_mix`] walk answers — the NONE
/// the contents wall keeps).
pub fn has_mix(bottle_id: i32, ingredient_id: i32, kind: i32) -> bool {
    if !crate::world::brewing_is_potion_kind(bottle_id) {
        return false; // isContainer — the containers list the trio tests
    }
    has_container_mix(bottle_id, ingredient_id) || has_potion_mix(kind, ingredient_id)
}

/// THE CONTAINER MIX'S OWN ROWS (javap PotionBrewing.addVanillaMixes —
/// the addContainerRecipe pair the containerMixes lane wears: the
/// POTION + GUNPOWDER the SPLASH_POTION row, the SPLASH_POTION +
/// DRAGON_BREATH the LINGERING_POTION row; the from-item the
/// `ItemStack.is(Holder)` test the bottle's own id answers, the
/// ingredient the `Ingredient.test` the id equality speaks).
fn has_container_mix(bottle_id: i32, ingredient_id: i32) -> bool {
    (bottle_id == crate::world::ITEM_POTION && ingredient_id == crate::world::ITEM_GUNPOWDER)
        || (bottle_id == crate::world::ITEM_SPLASH_POTION
            && ingredient_id == crate::world::ITEM_DRAGON_BREATH)
}

/// THE POTION MIX'S OWN WALK (javap hasPotionMix pc 0-27 — the
/// `POTION_CONTENTS` Optional the EMPTY fold answers: the
/// contents-less stack the `optional.isEmpty()` early false keeps;
/// pc 26-95 the potionMixes scan the FIRST matching row the true
/// speaks — the `mix.from.is(potion)` holder identity the kind's
/// registry id answers, the ingredient test the item id speaks).
pub fn has_potion_mix(kind: i32, ingredient_id: i32) -> bool {
    if kind == POTION_KIND_NONE {
        return false; // the Optional empty fold the early false keeps
    }
    potion_mix_target(kind, ingredient_id).is_some()
}

/// THE FIRST MATCHING POTION ROW (javap hasPotionMix/mix pc
/// 119-199 — the potionMixes scan the first
/// `from.is(kind) && ingredient.test(item)` row the to-kind
/// speaks; the no-row tail the None keeps).
pub fn potion_mix_target(kind: i32, ingredient_id: i32) -> Option<i32> {
    POTION_MIXES
        .iter()
        .find(|&&(from, ing, _)| from == kind && ing == ingredient_id)
        .map(|&(_, _, to)| to)
}

/// THE DO-BREW WALK (javap doBrew — the finish the count arm's
/// zero-tick speaks): the three bottle seats the mix loop rewrites
/// (the stack AND the kind the pair the [`mix`] walk answers), the
/// ingredient the `shrink(1)` spends, the crafting remainder
/// the empty-slot arm the refill keeps. THE KINDS the bottle
/// trio's own cells the loop wears (the ingredient/fuel seats the
/// walk never touches).
fn do_brew(slots: &mut [(i32, i32)], kinds: &mut [i32; 5]) {
    let ingredient_id = slots.get(3).map(|s| s.0).unwrap_or(0);
    for i in 0..3 {
        let kind = kinds[i];
        if let Some(bottle) = slots.get_mut(i) {
            let (mixed, new_kind) = mix(ingredient_id, *bottle, kind);
            *bottle = mixed;
            kinds[i] = new_kind;
        }
    }
    if let Some(ingredient) = slots.get_mut(3) {
        // pc 52-56 — the shrink the ingredient spends.
        ingredient.1 -= 1;
        if ingredient.1 <= 0 {
            *ingredient = (0, 0);
        }
        // pc 57-108 — the crafting-remainder lane: the empty slot
        // the remainder REFILLS, the non-empty slot the world drop
        // wears (the model's drop lane the spill walk serves — no
        // brewing ingredient 26.2 banks a remainder, the lane the
        // honest never-spoken keeps).
    }
}

/// THE MIX WALK (javap PotionBrewing.mix — the exact order): the
/// empty bottle the first `areturn` keeps, the POTION_CONTENTS
/// Optional the EMPTY fold the second speaks (pc 26-34 — the
/// [`POTION_KIND_NONE`] the early return wears), the container
/// rows the FIRST scan answers (pc 35-118 — the new item the
/// `createItemStack(mix.to, contents.get())` lays, the SAME kind
/// rides), the potion rows the SECOND scan (pc 119-199 — the same
/// item the `createItemStack(potion.getItem(), mix.to)` keeps, the
/// NEW kind lays), the no-row tail the same pair keeps (pc
/// 200-201). Returns `(stack, kind)`.
pub fn mix(ingredient_id: i32, bottle: (i32, i32), kind: i32) -> ((i32, i32), i32) {
    if bottle.1 <= 0 {
        return (bottle, kind); // pc 0-8 — the empty bottle itself returns
    }
    if kind == POTION_KIND_NONE {
        // pc 26-34 — the POTION_CONTENTS Optional the EMPTY fold:
        // the contents-less stack the vanilla's own walk NEVER
        // transforms (the container scans too the early return
        // never reaches).
        return (bottle, kind);
    }
    // pc 35-118 — the containerMixes scan the FIRST speaks: the
    // bottle's own item the `potion.is(mix.from)` answers, the new
    // item the to-holder lays, the SAME potion kind the contents
    // get() rides.
    if let Some(to_item) = container_mix_target(bottle.0, ingredient_id) {
        return ((to_item, bottle.1), kind);
    }
    // pc 119-199 — the potionMixes scan the SECOND: the kind the
    // `mix.from.is(contents.get())` answers, the same item the
    // getItem() keeps, the new kind the to-holder lays.
    if let Some(to_kind) = potion_mix_target(kind, ingredient_id) {
        return (bottle, to_kind);
    }
    // pc 200-201 — the no-row tail the same pair keeps.
    (bottle, kind)
}

/// THE CONTAINER MIX'S OWN TARGET (javap mix pc 35-118 — the
/// containerMixes scan the first `potion.is(from) &&
/// ingredient.test(item)` row the to-item speaks).
fn container_mix_target(bottle_id: i32, ingredient_id: i32) -> Option<i32> {
    if bottle_id == crate::world::ITEM_POTION && ingredient_id == crate::world::ITEM_GUNPOWDER {
        Some(crate::world::ITEM_SPLASH_POTION)
    } else if bottle_id == crate::world::ITEM_SPLASH_POTION
        && ingredient_id == crate::world::ITEM_DRAGON_BREATH
    {
        Some(crate::world::ITEM_LINGERING_POTION)
    } else {
        None
    }
}

/// THE FUEL BAR'S OWN WIDTH (javap 26.2 BrewingStandScreen.
/// extractBackground pcs 77-96 — the `Mth.clamp((18 * fuel + 20 - 1)
/// / 20, 0, 18)` fold the ceil law the integer trick wears).
pub fn fuel_bar_width(fuel: i32) -> i32 {
    ((18 * fuel + 19) / 20).clamp(0, 18)
}

/// THE ARROW'S OWN HEIGHT (javap extractBackground pcs 148-160 — the
/// `(int)(28.0f * (1.0f - brewingTicks / 400.0f))` fold the fill
/// speaks: the fresh 400 the zero height wears, the counted-down
/// stand the growing fill keeps, the Java float divide the f32
/// lane answers).
pub fn arrow_height(brewing_ticks: i32) -> i32 {
    if brewing_ticks <= 0 {
        return 0;
    }
    let l = (28.0f32 * (1.0f32 - brewing_ticks as f32 / 400.0f32)) as i32;
    l.max(0)
}

/// THE BUBBLES' OWN HEIGHT (javap extractBackground pcs 197-208 — the
/// `BUBBLELENGTHS[brewingTicks / 2 % 7]` index the table answers,
/// the caller's `> 0` gate the zero cell skips).
pub fn bubble_height(brewing_ticks: i32) -> i32 {
    if brewing_ticks <= 0 {
        return 0;
    }
    BUBBLE_LENGTHS[(brewing_ticks / 2) as usize % 7]
}

/// THE SIM'S OWN PIN WALKS (the tables the javap walk serves — the
/// arcs the bytecode's own order keeps).
#[cfg(test)]
mod tests {
    use super::*;

    fn wire(name: &str) -> i32 {
        mc_registry::item_id_by_name(name).unwrap()
    }

    #[test]
    fn the_brewing_words_the_constants_wear() {
        assert_eq!(DATA_BREW_TIME, 0);
        assert_eq!(DATA_FUEL_USES, 1);
        assert_eq!(INGREDIENT_WORD, 2);
        assert_eq!(BREW_TOTAL_TICKS, 400, "the sipush 400 the start arm lays");
        assert_eq!(FUEL_USES, 20, "the FUEL_USES clinit the refill keeps");
        assert_eq!(
            BUBBLE_LENGTHS,
            [29, 24, 20, 16, 11, 6, 0],
            "the 26.2 clinit's own seven the screen walk indexes"
        );
    }

    #[test]
    fn the_brew_tick_the_fuel_refill_arm_wears() {
        let powder = wire("blaze_powder");
        let mut slots = vec![(0, 0); 5];
        slots[4] = (powder, 3);
        let mut words = [0i32; 3];
        let mut kinds = [POTION_KIND_NONE; 5];
        let finished = tick(&mut slots, &mut words, &mut kinds);
        assert!(!finished, "the refill alone the finish never speaks");
        assert_eq!(words[DATA_FUEL_USES], FUEL_USES, "the bipush 20 lands");
        assert_eq!(slots[4], (powder, 2), "the shrink the powder spends");
        assert_eq!(
            words[DATA_BREW_TIME], 0,
            "the count the refill never starts"
        );
        // THE SECOND ARM the full tank skips (pc 13 `ifgt 49` the
        // fuel > 0 gate keeps).
        let before = (slots[4], words[DATA_FUEL_USES]);
        tick(&mut slots, &mut words, &mut kinds);
        assert_eq!(slots[4], before.0);
        assert_eq!(words[DATA_FUEL_USES], before.1);
        // THE NON-FUEL ROW the gate refuses (the BREWING_FUEL tag the
        // blaze powder the lone member the canPlaceItem lane banks —
        // the coal the furnace fuel the brewing never burns).
        let coal = wire("coal");
        let mut slots2 = vec![(0, 0); 5];
        slots2[4] = (coal, 5);
        let mut words2 = [0i32; 3];
        let mut kinds2 = [POTION_KIND_NONE; 5];
        tick(&mut slots2, &mut words2, &mut kinds2);
        assert_eq!(words2[DATA_FUEL_USES], 0, "the coal the tank never fills");
        assert_eq!(slots2[4], (coal, 5), "the powder never spent");
    }

    #[test]
    fn the_brew_tick_the_start_and_count_arms_wear() {
        let powder = wire("blaze_powder");
        let gunpowder = wire("gunpowder");
        let potion = wire("potion");
        let mut slots = vec![(0, 0); 5];
        slots[0] = (potion, 1);
        slots[3] = (gunpowder, 2);
        slots[4] = (powder, 1);
        let mut words = [0i32; 3];
        let mut kinds = [POTION_KIND_NONE; 5];
        // THE REFILL + THE START the first tick pairs (the order the
        // bytecode keeps: the refill arm runs BEFORE the brewable
        // gate reads).
        assert!(!tick(&mut slots, &mut words, &mut kinds));
        assert_eq!(words[DATA_FUEL_USES], FUEL_USES - 1, "the start spends one");
        assert_eq!(
            words[DATA_BREW_TIME], BREW_TOTAL_TICKS,
            "the sipush 400 lays"
        );
        assert_eq!(words[INGREDIENT_WORD], gunpowder, "the start word banks");
        assert_eq!(
            slots[4],
            (0, 0),
            "the refill's own shrink the empty pair wears"
        );
        // THE COUNT the middle walk wears (the decrement the tick
        // keeps, the finish the zero tick speaks).
        for _ in 0..399 {
            assert!(!tick(&mut slots, &mut words, &mut kinds));
        }
        assert_eq!(words[DATA_BREW_TIME], 1);
        // THE FINISH: the ingredient the shrink spends, the bottle
        // the contents wall never transforms (the NONE kind the
        // POTION_CONTENTS Optional the empty fold wears), the event
        // the caller fires.
        assert!(
            tick(&mut slots, &mut words, &mut kinds),
            "the zero tick the finish speaks"
        );
        assert_eq!(words[DATA_BREW_TIME], 0);
        assert_eq!(
            slots[0],
            (potion, 1),
            "the POTION_CONTENTS wall the mix keeps"
        );
        assert_eq!(kinds[0], POTION_KIND_NONE, "the wall the kind keeps");
        assert_eq!(slots[3], (gunpowder, 1), "the doBrew the ingredient spends");
        // THE NEXT START: the fuel WORD the tank's own counter keeps
        // (19 uses the refill already banked — the powder's absence
        // the `fuel > 0` gate never reads, the brewable stand the
        // next tick the start arms again).
        assert!(!tick(&mut slots, &mut words, &mut kinds));
        assert_eq!(
            words[DATA_FUEL_USES],
            FUEL_USES - 2,
            "the word the spend wears"
        );
        assert_eq!(words[DATA_BREW_TIME], BREW_TOTAL_TICKS, "the tank re-arms");
        // THE EXHAUSTED TANK the start refuses (the word the zero
        // wears, the powder-less refill the arm never serves).
        words[DATA_FUEL_USES] = 0;
        words[DATA_BREW_TIME] = 0;
        assert!(!tick(&mut slots, &mut words, &mut kinds));
        assert_eq!(words[DATA_BREW_TIME], 0, "the empty tank the start refuses");
    }

    #[test]
    fn the_brew_tick_the_abort_and_keep_arms_wear() {
        let gunpowder = wire("gunpowder");
        let nether_wart = wire("nether_wart");
        let potion = wire("potion");
        let mut slots = vec![(0, 0); 5];
        slots[0] = (potion, 1);
        slots[3] = (gunpowder, 1);
        let mut words = [0i32; 3];
        let mut kinds = [POTION_KIND_NONE; 5];
        words[DATA_FUEL_USES] = FUEL_USES;
        assert!(!tick(&mut slots, &mut words, &mut kinds), "the start arms");
        assert_eq!(words[DATA_BREW_TIME], BREW_TOTAL_TICKS);
        // THE MID-BREW SWAP the abort arm wears (pc 140-159 — the
        // different-item stack the `brewTime = 0` keeps).
        slots[3] = (nether_wart, 1);
        assert!(!tick(&mut slots, &mut words, &mut kinds));
        assert_eq!(words[DATA_BREW_TIME], 0, "the swap the count aborts");
        // THE EMPTY INGREDIENT the same abort answers (the empty
        // stack the `.is` identity never matches).
        slots[3] = (gunpowder, 1);
        words[DATA_BREW_TIME] = 200;
        slots[3] = (0, 0);
        assert!(!tick(&mut slots, &mut words, &mut kinds));
        assert_eq!(words[DATA_BREW_TIME], 0);
        // THE SAME-INGREDIENT STAND the count keeps (the restart the
        // start arm never repeats — the counting walk the decrement
        // alone speaks).
        slots[3] = (gunpowder, 1);
        words[DATA_BREW_TIME] = 200;
        assert!(!tick(&mut slots, &mut words, &mut kinds));
        assert_eq!(words[DATA_BREW_TIME], 199, "the same item the count keeps");
    }

    #[test]
    fn the_brew_chemistry_the_walls_wear() {
        let potion = wire("potion");
        let splash = wire("splash_potion");
        let lingering = wire("lingering_potion");
        let gunpowder = wire("gunpowder");
        let dragon = wire("dragon_breath");
        let nether_wart = wire("nether_wart");
        let dirt = wire("dirt");
        // THE HAS-MIX PAIR the container rows answer (the
        // addContainerRecipe pair the from-item + ingredient test
        // wears) — the kind the scan rides, the NONE the potion wall
        // keeps.
        assert!(has_mix(potion, gunpowder, POTION_KIND_NONE));
        assert!(has_mix(splash, dragon, POTION_KIND_NONE));
        assert!(
            !has_mix(splash, gunpowder, POTION_KIND_NONE),
            "the backwards row never speaks"
        );
        assert!(
            !has_mix(lingering, gunpowder, POTION_KIND_NONE),
            "the lingering no container row"
        );
        assert!(
            !has_mix(potion, nether_wart, POTION_KIND_NONE),
            "the contents-less stack the potion wall keeps"
        );
        assert!(
            has_mix(potion, nether_wart, POTION_WATER),
            "the water->awkward row the kinded stack serves"
        );
        assert!(!has_mix(potion, dirt, POTION_WATER));
        assert!(
            !has_mix(dirt, gunpowder, POTION_WATER),
            "the non-container the gate refuses"
        );
        // THE MIX the contents wall never transforms (javap mix
        // pc 26-34 — the EMPTY fold the early return wears, the
        // container scans too the return never reaches).
        assert_eq!(
            mix(gunpowder, (potion, 3), POTION_KIND_NONE),
            ((potion, 3), POTION_KIND_NONE)
        );
        assert_eq!(
            mix(dragon, (splash, 1), POTION_KIND_NONE),
            ((splash, 1), POTION_KIND_NONE)
        );
        assert_eq!(
            mix(gunpowder, (0, 0), POTION_WATER),
            ((0, 0), POTION_WATER),
            "the empty bottle itself"
        );
        // THE BREWABLE the gate lanes wear: the valid container mix
        // brews, the contents-less nether-wart stand the wall
        // refuses, the kinded water bottle the row serves, the empty
        // ingredient the first false keeps.
        let mut slots = vec![(0, 0); 5];
        let kinds_none = [POTION_KIND_NONE; 5];
        let mut kinds_water = [POTION_KIND_NONE; 5];
        kinds_water[0] = POTION_WATER;
        assert!(
            !is_brewable(&slots, &kinds_none),
            "the empty stand never brews"
        );
        slots[0] = (potion, 1);
        assert!(
            !is_brewable(&slots, &kinds_none),
            "the empty ingredient the false"
        );
        slots[3] = (nether_wart, 1);
        assert!(
            !is_brewable(&slots, &kinds_none),
            "the contents-less stand the potion wall refuses"
        );
        assert!(
            is_brewable(&slots, &kinds_water),
            "the water kind the nether-wart row serves"
        );
        slots[3] = (gunpowder, 1);
        assert!(
            is_brewable(&slots, &kinds_none),
            "the container mix the stand serves"
        );
        slots[3] = (dirt, 4);
        assert!(
            !is_brewable(&slots, &kinds_water),
            "the dirt the isIngredient refuses"
        );
    }

    #[test]
    fn the_potion_kinds_the_registry_and_the_mix_walk_wear() {
        // THE REGISTRY the 46 rows the clinit order pins (the
        // registration sequence the Potions clinit speaks).
        assert_eq!(POTION_KIND_NAMES.len(), 46);
        assert_eq!(potion_kind_by_name("water"), Some(POTION_WATER));
        assert_eq!(potion_kind_by_name("mundane"), Some(POTION_MUNDANE));
        assert_eq!(potion_kind_by_name("thick"), Some(POTION_THICK));
        assert_eq!(potion_kind_by_name("awkward"), Some(POTION_AWKWARD));
        assert_eq!(
            potion_kind_by_name("wind_charged"),
            Some(POTION_WIND_CHARGED)
        );
        assert_eq!(potion_kind_by_name("infested"), Some(POTION_INFESTED));
        assert_eq!(potion_kind_by_name("luck"), Some(POTION_LUCK));
        assert_eq!(potion_kind_by_name("nether_wart"), None);
        // THE START-MIX PAIR the builder's own expansion wears (javap
        // Builder.addStartMix pc 19-36 — the water + item the
        // MUNDANE keeps, the awkward + item the potion lays).
        let breeze = wire("breeze_rod");
        assert_eq!(
            potion_mix_target(POTION_WATER, breeze),
            Some(POTION_MUNDANE),
            "the water + breeze rod the mundane row speaks"
        );
        assert_eq!(
            potion_mix_target(POTION_AWKWARD, breeze),
            Some(POTION_WIND_CHARGED),
            "the awkward + breeze rod the wind-charged row speaks"
        );
        // THE MIX WALK the potion rows transform (the same item the
        // getItem keeps, the new kind the to-holder lays).
        let potion = wire("potion");
        assert_eq!(
            mix(breeze, (potion, 1), POTION_AWKWARD),
            ((potion, 1), POTION_WIND_CHARGED),
            "the awkward + breeze rod the kind transforms"
        );
        // THE CONTAINER SCAN the FIRST speaks (pc 35-118 — the new
        // item lays, the SAME kind rides).
        let gunpowder = wire("gunpowder");
        let splash = wire("splash_potion");
        assert_eq!(
            mix(gunpowder, (potion, 1), POTION_NIGHT_VISION),
            ((splash, 1), POTION_NIGHT_VISION),
            "the splash row the kind rides"
        );
        // THE NO-ROW TAIL the same pair keeps (pc 200-201).
        let dirt = wire("dirt");
        assert_eq!(
            mix(dirt, (potion, 1), POTION_WATER),
            ((potion, 1), POTION_WATER),
            "the no-row tail the kind keeps"
        );
        // THE FIRST-MATCH ORDER the list scan wears (the poison +
        // fermented-spider-eye rows the HARMING trio answers, the
        // long rows the same target keep).
        let eye = wire("fermented_spider_eye");
        assert_eq!(potion_mix_target(POTION_POISON, eye), Some(POTION_HARMING));
        assert_eq!(
            potion_mix_target(POTION_LONG_POISON, eye),
            Some(POTION_HARMING)
        );
        assert_eq!(
            potion_mix_target(POTION_STRONG_POISON, eye),
            Some(POTION_STRONG_HARMING)
        );
        // THE NETHER WART the water->awkward base row (the 309
        // round's own law the potion lane re-wears).
        let wart = wire("nether_wart");
        assert_eq!(
            mix(wart, (potion, 1), POTION_WATER),
            ((potion, 1), POTION_AWKWARD),
            "the water bottle the awkward walk serves"
        );
        // THE REDSTONE the mundane row (the water + redstone the
        // second addMix wears).
        let redstone = wire("redstone");
        assert_eq!(
            potion_mix_target(POTION_WATER, redstone),
            Some(POTION_MUNDANE)
        );
        // THE KIND-CHAINED WALK: water -> awkward -> night vision ->
        // long night vision (the three brews the table's rows
        // answer).
        let carrot = wire("golden_carrot");
        let mut kind = POTION_WATER;
        let stack = (potion, 1);
        let (stack2, k2) = mix(wart, stack, kind);
        kind = k2;
        assert_eq!(kind, POTION_AWKWARD);
        let (_, k3) = mix(carrot, stack2, kind);
        kind = k3;
        assert_eq!(kind, POTION_NIGHT_VISION);
        let redstone = wire("redstone");
        let (_, k4) = mix(redstone, stack2, kind);
        assert_eq!(k4, POTION_LONG_NIGHT_VISION, "the long row the chain wears");
    }

    #[test]
    fn the_brew_screen_the_render_folds_wear() {
        // THE FUEL BAR (pcs 77-96 — the ceil law the integer trick
        // wears, the 18-wide cap the clamp keeps).
        assert_eq!(fuel_bar_width(0), 0);
        assert_eq!(fuel_bar_width(1), 1);
        assert_eq!(fuel_bar_width(10), 9);
        assert_eq!(fuel_bar_width(19), 18);
        assert_eq!(fuel_bar_width(20), 18);
        assert_eq!(fuel_bar_width(400), 18, "the clamp the over cap keeps");
        // THE ARROW (pcs 148-160 — the 28-scale fill the countdown
        // grows, the f32 divide the Java lane answers).
        assert_eq!(arrow_height(0), 0, "the idle the caller gates");
        assert_eq!(arrow_height(400), 0, "the fresh start the fill never shows");
        assert_eq!(arrow_height(200), 14, "the half the 14 fill wears");
        assert_eq!(arrow_height(100), 21, "the quarter the 21 fill wears");
        assert_eq!(
            arrow_height(1),
            27,
            "the last tick the near-full fill wears"
        );
        // THE BUBBLES (pcs 197-208 — the table the /2 %7 index
        // walks, the cycle the countdown wears).
        assert_eq!(bubble_height(0), 0);
        assert_eq!(bubble_height(1), 29, "the [0] row the first tick wears");
        assert_eq!(bubble_height(2), 24);
        assert_eq!(bubble_height(3), 24, "the /2 the pair shares");
        assert_eq!(bubble_height(11), 6, "the [5] row the sixth pair wears");
        assert_eq!(bubble_height(13), 0, "the [6] row the zero cell keeps");
        assert_eq!(bubble_height(15), 29, "the wrap the cycle restarts");
    }
}

// ===== autopush-340 THE POTION-KIND'S OWN TINT + NAME TABLES =====
// THE JAR LAW the 26.2 client.jar's own bytecode banks (scripts/jar_work/
// mob_effects_340.txt + potions_clinit_340.txt, both rc=0): the
// MobEffects bootstrap the register(name, new MobEffect(CATEGORY, COLOR))
// rows the ctor's ldc int wears, the Potions bootstrap the
// register(key, new Potion("base_name", new MobEffectInstance(EFFECT,
// duration[, amplifier])...)) rows the putstatic order IS the registry id
// order the 339 round's own law keeps. GENERATED by
// scripts/gen_potion_kinds_340.py — the tables the parser emits, the
// names the strip law answers.

/// THE MOB EFFECT'S OWN COLOR ROWS (javap 26.2 MobEffects.<clinit> — the
/// `new MobEffect(category, color)` / `new InstantMobEffect(category,
/// color)` ctor args the ldc int wears, the 40 rows the
/// bootstrap registers). The blend the getColorOptional walk reads.
pub const MOB_EFFECT_COLORS: &[(&str, u32)] = &[
    ("speed", 0x33EBFF),
    ("slowness", 0x8BAFE0),
    ("haste", 0xD9C043),
    ("mining_fatigue", 0x4A4217),
    ("strength", 0xFFC700),
    ("instant_health", 0xF82423),
    ("instant_damage", 0xA9656A),
    ("jump_boost", 0xFDFF84),
    ("nausea", 0x551D4A),
    ("regeneration", 0xCD5CAB),
    ("resistance", 0x9146F0),
    ("fire_resistance", 0xFF9900),
    ("water_breathing", 0x98DAC0),
    ("invisibility", 0xF6F6F6),
    ("blindness", 0x1F1F23),
    ("night_vision", 0xC2FF66),
    ("hunger", 0x587653),
    ("weakness", 0x484D48),
    ("poison", 0x87A363),
    ("wither", 0x736156),
    ("health_boost", 0xF87D23),
    ("absorption", 0x2552A5),
    ("saturation", 0xF82423),
    ("glowing", 0x94A061),
    ("levitation", 0xCEFFFF),
    ("luck", 0x59C106),
    ("unluck", 0xC0A44D),
    ("slow_falling", 0xF3CFB9),
    ("conduit_power", 0x1DC2D1),
    ("dolphins_grace", 0x88A3BE),
    ("bad_omen", 0x0B6138),
    ("hero_of_the_village", 0x44FF44),
    ("darkness", 0x292721),
    ("trial_omen", 0x16A6A6),
    ("raid_omen", 0xDE4058),
    ("wind_charged", 0xBDC9FF),
    ("weaving", 0x78695A),
    ("oozing", 0x99FFA3),
    ("infested", 0x8C9B8C),
    ("breath_of_the_nautilus", 0x00FFEE),
];

/// THE KIND'S OWN BASE NAME (javap 26.2 Potions.<clinit> — the
/// `Potion.name` field the ctor's FIRST arg wears; the long_/strong_
/// registry rows the BASE name the same arg answers — pc 95/133 both
/// "night_vision", the 26.2 lang table's own 26-key shape the strip law
/// explains). The display key `item.minecraft.potion.effect.<base>` the
/// getName walk's makeConcat pairs.
pub const POTION_KIND_BASE_NAMES: [&str; 46] = [
    "water",
    "mundane",
    "thick",
    "awkward",
    "night_vision",
    "night_vision",
    "invisibility",
    "invisibility",
    "leaping",
    "leaping",
    "leaping",
    "fire_resistance",
    "fire_resistance",
    "swiftness",
    "swiftness",
    "swiftness",
    "slowness",
    "slowness",
    "slowness",
    "turtle_master",
    "turtle_master",
    "turtle_master",
    "water_breathing",
    "water_breathing",
    "healing",
    "healing",
    "harming",
    "harming",
    "poison",
    "poison",
    "poison",
    "regeneration",
    "regeneration",
    "regeneration",
    "strength",
    "strength",
    "strength",
    "weakness",
    "weakness",
    "luck",
    "slow_falling",
    "slow_falling",
    "wind_charged",
    "weaving",
    "oozing",
    "infested",
];

/// THE KIND'S OWN EFFECT ROWS ((effect, amplifier) the ctor pairs — the
/// `(Holder, I)` ctor the amplifier-0 default wears, the `(Holder, II)`
/// the explicit second int; the water/mundane/thick/awkward rows the
/// EMPTY slice keeps). The visibility the short ctors' own default
/// answers (ambient false, visible true — every row the blend reads).
pub const POTION_KIND_EFFECTS: [&[(&str, i32)]; 46] = [
    &[],
    &[],
    &[],
    &[],
    &[("night_vision", 0)],
    &[("night_vision", 0)],
    &[("invisibility", 0)],
    &[("invisibility", 0)],
    &[("jump_boost", 0)],
    &[("jump_boost", 0)],
    &[("jump_boost", 1)],
    &[("fire_resistance", 0)],
    &[("fire_resistance", 0)],
    &[("speed", 0)],
    &[("speed", 0)],
    &[("speed", 1)],
    &[("slowness", 0)],
    &[("slowness", 0)],
    &[("slowness", 3)],
    &[("slowness", 3), ("resistance", 2)],
    &[("slowness", 3), ("resistance", 2)],
    &[("slowness", 5), ("resistance", 3)],
    &[("water_breathing", 0)],
    &[("water_breathing", 0)],
    &[("instant_health", 0)],
    &[("instant_health", 1)],
    &[("instant_damage", 0)],
    &[("instant_damage", 1)],
    &[("poison", 0)],
    &[("poison", 0)],
    &[("poison", 1)],
    &[("regeneration", 0)],
    &[("regeneration", 0)],
    &[("regeneration", 1)],
    &[("strength", 0)],
    &[("strength", 0)],
    &[("strength", 1)],
    &[("weakness", 0)],
    &[("weakness", 0)],
    &[("luck", 0)],
    &[("slow_falling", 0)],
    &[("slow_falling", 0)],
    &[("wind_charged", 0)],
    &[("weaving", 0)],
    &[("oozing", 0)],
    &[("infested", 0)],
];

/// THE BLEND'S OWN DEFAULT (javap 26.2 PotionContents.getColor pc 1 —
/// the `getColorOr(-13083194)` ldc the walk wears; -13083194 as u32 =
/// 0xFF385DC6, the RGB the water blue answers). The no-effect and the
/// unknown kinds the default keeps.
pub const POTION_DEFAULT_COLOR: u32 = 0x385DC6;

/// THE KIND'S OWN LIQUID COLOR (javap 26.2 PotionContents.getColorOr
/// pcs 0-35 — the customColor the present arm returns [the registry
/// potions the absent lane wears], the getAllEffects the
/// getColorOptional(Iterable) walk serves: the VISIBLE instances the
/// `(amplifier + 1)`-weighted ARGB channel sums the divisor blends
/// [pc 63-115], the zero-weight the OptionalInt.empty the default
/// arm answers [pc 120-128 + orElse(pc 31-32)]; the
/// `ARGB.color(r/w, g/w, b/w)` the channel walk wears). The client's
/// registry potions carry no custom colors — the kind's effects the
/// whole walk serves, the empty rows the default returns.
pub fn potion_kind_liquid_color(kind: i32) -> u32 {
    if kind < 0 || kind as usize >= POTION_KIND_NAMES.len() {
        return POTION_DEFAULT_COLOR;
    }
    let rows = POTION_KIND_EFFECTS[kind as usize];
    if rows.is_empty() {
        return POTION_DEFAULT_COLOR;
    }
    let (mut r, mut g, mut b, mut w) = (0i64, 0i64, 0i64, 0i64);
    for &(effect, amp) in rows {
        let Some(&(_, color)) = MOB_EFFECT_COLORS.iter().find(|(n, _)| *n == effect) else {
            continue; // the registry's own miss the skip keeps
        };
        let k = (amp + 1) as i64;
        r += k * ((color >> 16) & 0xFF) as i64;
        g += k * ((color >> 8) & 0xFF) as i64;
        b += k * (color & 0xFF) as i64;
        w += k;
    }
    if w == 0 {
        return POTION_DEFAULT_COLOR;
    }
    (((r / w) as u32) << 16) | (((g / w) as u32) << 8) | ((b / w) as u32)
}

/// THE KIND'S OWN DISPLAY KEY TAIL (the Potion.name field the base name
/// returns — the caller the `item.minecraft.potion.effect.` prefix the
/// lang key pairs). The unknown kind the "empty" tail answers [the
/// getName walk pc 13-18 the orElse("empty") the Uncraftable key wears].
pub fn potion_kind_base_name(kind: i32) -> &'static str {
    if kind < 0 || kind as usize >= POTION_KIND_BASE_NAMES.len() {
        return "empty";
    }
    POTION_KIND_BASE_NAMES[kind as usize]
}

// ===== end autopush-340 tables =====

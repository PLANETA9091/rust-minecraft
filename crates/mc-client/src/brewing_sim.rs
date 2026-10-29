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

/// THE BREW TICK (javap BrewingStandBlockEntity.serverTick — the
/// walk in the bytecode's own order). `slots` the menu's five cells
/// (bottles 0..2, ingredient 3, fuel 4), `words` the bank's three
/// (brewTime, fuel, the start-ingredient id). Returns whether the
/// finish fired (the doBrew walk — the caller's voice the
/// levelEvent 1035 the client answers).
pub fn tick(slots: &mut [(i32, i32)], words: &mut [i32; 3]) -> bool {
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
    let brewable = is_brewable(slots);
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
            do_brew(slots);
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
/// trio the final false keeps.
pub fn is_brewable(slots: &[(i32, i32)]) -> bool {
    let Some(ingredient) = slots.get(3) else {
        return false;
    };
    if ingredient.1 <= 0 {
        return false;
    }
    if !crate::world::brewing_is_ingredient(ingredient.0) {
        return false;
    }
    for bottle in slots.iter().take(3) {
        if bottle.1 <= 0 {
            continue; // pc 45-53 — the empty seat the scan skips
        }
        if has_mix(bottle.0, ingredient.0) {
            return true;
        }
    }
    false
}

/// THE HAS-MIX WALK (javap PotionBrewing.hasMix — the `isContainer &&
/// (hasContainerMix || hasPotionMix)` pair the verdict speaks).
pub fn has_mix(bottle_id: i32, ingredient_id: i32) -> bool {
    if !crate::world::brewing_is_potion_kind(bottle_id) {
        return false; // isContainer — the containers list the trio tests
    }
    has_container_mix(bottle_id, ingredient_id) || has_potion_mix()
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

/// THE POTION MIX'S OWN WALL (javap hasPotionMix pc 0-27 — the
/// `POTION_CONTENTS` Optional the EMPTY fold answers: the
/// contents-less stack the `optional.isEmpty()` early false keeps).
/// THE (id, count) model the component never carries — the lane the
/// honest false wears, the potion-kind data-model round the wall's
/// own answer keeps.
fn has_potion_mix() -> bool {
    false
}

/// THE DO-BREW WALK (javap doBrew — the finish the count arm's
/// zero-tick speaks): the three bottle seats the mix loop rewrites,
/// the ingredient the `shrink(1)` spends, the crafting remainder
/// the empty-slot arm the refill keeps. THE MIX'S OWN WALL (javap
/// mix pc 9-34): the bottle's POTION_CONTENTS the EMPTY fold the
/// early `areturn` answers — the contents-less stack the vanilla's
/// own walk NEVER transforms, the (id, count) model the same
/// verdict wears (the fuel and the ingredient the walk still
/// spends — the vanilla's own component-less behaviour the tests
/// pin).
fn do_brew(slots: &mut [(i32, i32)]) {
    let ingredient_id = slots.get(3).map(|s| s.0).unwrap_or(0);
    for i in 0..3 {
        if let Some(bottle) = slots.get_mut(i) {
            *bottle = mix(ingredient_id, *bottle);
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
/// Optional the EMPTY fold the second speaks, the container rows
/// the from-item + ingredient test answers, the potion rows the
/// contents wall never reaches.
pub fn mix(ingredient_id: i32, bottle: (i32, i32)) -> (i32, i32) {
    if bottle.1 <= 0 {
        return bottle; // pc 0-8 — the empty bottle itself returns
    }
    // pc 9-34 — the POTION_CONTENTS Optional: the (id, count) model
    // the component never carries, the EMPTY fold the same early
    // return the contents-less stack wears.
    let _ = ingredient_id;
    bottle
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
        let finished = tick(&mut slots, &mut words);
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
        tick(&mut slots, &mut words);
        assert_eq!(slots[4], before.0);
        assert_eq!(words[DATA_FUEL_USES], before.1);
        // THE NON-FUEL ROW the gate refuses (the BREWING_FUEL tag the
        // blaze powder the lone member the canPlaceItem lane banks —
        // the coal the furnace fuel the brewing never burns).
        let coal = wire("coal");
        let mut slots2 = vec![(0, 0); 5];
        slots2[4] = (coal, 5);
        let mut words2 = [0i32; 3];
        tick(&mut slots2, &mut words2);
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
        // THE REFILL + THE START the first tick pairs (the order the
        // bytecode keeps: the refill arm runs BEFORE the brewable
        // gate reads).
        assert!(!tick(&mut slots, &mut words));
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
            assert!(!tick(&mut slots, &mut words));
        }
        assert_eq!(words[DATA_BREW_TIME], 1);
        // THE FINISH: the ingredient the shrink spends, the bottle
        // the component wall never transforms, the event the caller
        // fires.
        assert!(
            tick(&mut slots, &mut words),
            "the zero tick the finish speaks"
        );
        assert_eq!(words[DATA_BREW_TIME], 0);
        assert_eq!(
            slots[0],
            (potion, 1),
            "the POTION_CONTENTS wall the mix keeps"
        );
        assert_eq!(slots[3], (gunpowder, 1), "the doBrew the ingredient spends");
        // THE NEXT START: the fuel WORD the tank's own counter keeps
        // (19 uses the refill already banked — the powder's absence
        // the `fuel > 0` gate never reads, the brewable stand the
        // next tick the start arms again).
        assert!(!tick(&mut slots, &mut words));
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
        assert!(!tick(&mut slots, &mut words));
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
        words[DATA_FUEL_USES] = FUEL_USES;
        assert!(!tick(&mut slots, &mut words), "the start arms");
        assert_eq!(words[DATA_BREW_TIME], BREW_TOTAL_TICKS);
        // THE MID-BREW SWAP the abort arm wears (pc 140-159 — the
        // different-item stack the `brewTime = 0` keeps).
        slots[3] = (nether_wart, 1);
        assert!(!tick(&mut slots, &mut words));
        assert_eq!(words[DATA_BREW_TIME], 0, "the swap the count aborts");
        // THE EMPTY INGREDIENT the same abort answers (the empty
        // stack the `.is` identity never matches).
        slots[3] = (gunpowder, 1);
        words[DATA_BREW_TIME] = 200;
        slots[3] = (0, 0);
        assert!(!tick(&mut slots, &mut words));
        assert_eq!(words[DATA_BREW_TIME], 0);
        // THE SAME-INGREDIENT STAND the count keeps (the restart the
        // start arm never repeats — the counting walk the decrement
        // alone speaks).
        slots[3] = (gunpowder, 1);
        words[DATA_BREW_TIME] = 200;
        assert!(!tick(&mut slots, &mut words));
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
        // wears).
        assert!(has_mix(potion, gunpowder));
        assert!(has_mix(splash, dragon));
        assert!(
            !has_mix(splash, gunpowder),
            "the backwards row never speaks"
        );
        assert!(
            !has_mix(lingering, gunpowder),
            "the lingering no container row"
        );
        assert!(
            !has_mix(potion, nether_wart),
            "the potion mix the wall keeps"
        );
        assert!(!has_mix(potion, dirt));
        assert!(
            !has_mix(dirt, gunpowder),
            "the non-container the gate refuses"
        );
        // THE MIX the contents wall never transforms (javap mix
        // pc 9-34 — the EMPTY fold the early return wears).
        assert_eq!(mix(gunpowder, (potion, 3)), (potion, 3));
        assert_eq!(mix(dragon, (splash, 1)), (splash, 1));
        assert_eq!(mix(gunpowder, (0, 0)), (0, 0), "the empty bottle itself");
        // THE BREWABLE the gate lanes wear: the valid container mix
        // brews, the nether-wart stand the potion-mix wall refuses,
        // the empty ingredient the first false keeps.
        let mut slots = vec![(0, 0); 5];
        assert!(!is_brewable(&slots), "the empty stand never brews");
        slots[0] = (potion, 1);
        assert!(!is_brewable(&slots), "the empty ingredient the false");
        slots[3] = (nether_wart, 1);
        assert!(
            !is_brewable(&slots),
            "the potion-mix wall the stand refuses"
        );
        slots[3] = (gunpowder, 1);
        assert!(is_brewable(&slots), "the container mix the stand serves");
        slots[3] = (dirt, 4);
        assert!(!is_brewable(&slots), "the dirt the isIngredient refuses");
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

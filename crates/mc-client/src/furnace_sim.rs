//! THE FURNACE'S OWN OFFLINE SMELTING SIM (javap 26.2
//! AbstractFurnaceBlockEntity — the serverTick walk the block entity
//! speaks every game tick, the same order the bytecode keeps). The
//! OFFLINE stand-in's own engine: the opened furnace's block entity
//! the fresh-menu stand-in retires — the coal burns, the ore smelts,
//! the result lands, the world's placed kind flips FURNACE /
//! FURNACE_LIT the lit-propagation walk wears. The wire lane never
//! rides (the live menu the server's own words drive).

/// THE DATA WORDS' OWN SHAPE (the `SimpleContainerData(4)` the
/// AbstractFurnaceMenu ctor speaks — the ContainerData get tableswitch
/// pins the order): 0 the lit ticks remaining, 1 the lit total, 2 the
/// cooking progress, 3 the cooking total.
pub const DATA_LIT_TIME: usize = 0;
pub const DATA_LIT_DURATION: usize = 1;
pub const DATA_COOKING_PROGRESS: usize = 2;
pub const DATA_COOKING_TOTAL_TIME: usize = 3;

/// THE BURN COOL SPEED (javap ConstantValue int 2 — the cooldown's
/// own -2 the unlit walk wears).
pub const BURN_COOL_SPEED: i32 = 2;

/// THE COOKING TIME'S OWN DEFAULT (javap getTotalCookTime pc 32
/// `sipush 200` the no-recipe fallback keeps; the SmeltingRecipe
/// clinit's codec default the same 200 wears — every 26.2 smelting
/// row omits the cookingtime field, so the recipe-time fetch and the
/// fallback speak ONE number).
pub const DEFAULT_COOKING_TOTAL_TIME: i32 = 200;

/// THE WIRE IDS THE SPECIAL WALKS WEAR (the item_order table's own
/// rows — the bucket family the consume/burn walks answer).
const BUCKET: i32 = 1040;
const WATER_BUCKET: i32 = 1041;
const LAVA_BUCKET: i32 = 1042;
const WET_SPONGE: i32 = 221;

/// THE SIM'S OWN MEMORY (the block entity's private field the setItem
/// walk serves: the input item the slot wore LAST tick — the identity
/// change the cookingTotalTime reset answers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FurnaceSim {
    /// The input slot's item id the LAST reset saw (0 = empty) — the
    /// change the cooking words reset answers (javap setItem pc 53-99:
    /// the slot-0 read, the NOT same-item arm, the
    /// `cookingTotalTime = getTotalCookTime(); cookingTimer = 0` walk
    /// the ServerLevel branch speaks).
    pub last_input_id: i32,
}

impl FurnaceSim {
    /// THE SETITEM WALK'S OWN RESET (javap setItem pc 53-99 — the
    /// input slot's item CHANGE the cooking words re-fetch speaks:
    /// the recipe's own cooking time, the no-recipe arm the sipush-200
    /// default keeps — both arms the same 200 in 26.2). The count
    /// changes NEVER reset (the isSameItemSameComponents test the item
    /// identity alone compares).
    pub fn on_input_set(&mut self, data: &mut [i32; 4], input_id: i32) {
        if input_id != self.last_input_id {
            // THE GETTOTALCOOKTIME WALK (javap getTotalCookTime — the
            // recipe's cookingTime() the Optional carries, the
            // orElse(200) the no-recipe arm wears; the generated
            // table's row the same default answers)
            data[DATA_COOKING_TOTAL_TIME] = DEFAULT_COOKING_TOTAL_TIME;
            data[DATA_COOKING_PROGRESS] = 0;
            self.last_input_id = input_id;
        }
    }

    /// THE TICK (javap AbstractFurnaceBlockEntity.serverTick — the
    /// walk in the bytecode's own order). `slots` the menu's first
    /// three cells (input 0, fuel 1, result 2), `data` the four
    /// progress words. Returns whether the LIT state flipped (the
    /// wasLit != isLit verdict pc 369-373 — the caller's world walk
    /// the setBlock LIT swap wears).
    pub fn tick(&mut self, slots: &mut [(i32, i32)], data: &mut [i32; 4]) -> bool {
        let has_input = slots.first().is_some_and(|s| s.1 > 0);
        // THE SETITEM'S OWN RESET FIRST (the click the slot mutation
        // spoke BEFORE the tick — the setItem walk at the tick's head
        // the same observable order keeps)
        let input_id = if has_input { slots[0].0 } else { 0 };
        self.on_input_set(data, input_id);
        // THE LIT WALK (pc 3-44): wasLit the > 0 read, the decrement
        // the lit tick spends, the now-lit the re-read answers
        let was_lit = data[DATA_LIT_TIME] > 0;
        let mut now_lit = false;
        if was_lit {
            data[DATA_LIT_TIME] -= 1;
            now_lit = data[DATA_LIT_TIME] > 0;
        }
        let has_fuel = slots.get(1).is_some_and(|s| s.1 > 0);
        // THE BRANCH LADDER (pc 102-119): lit → the recipe walk; not
        // lit with fuel AND input → the recipe walk too (the ignite
        // attempt rides); not lit without either → the cooldown walk.
        if now_lit || (has_fuel && has_input) {
            if has_input {
                // THE RECIPE CHECK (pc 122-152 — the quickCheck the
                // SingleRecipeInput walk serves)
                let input = slots[0];
                let Some(assembled) =
                    mc_registry::generated::furnace_data::smelt_result(input.0)
                else {
                    // the no-recipe arm (pc 154-156 `ifnull 333`):
                    // NOTHING rides — the timer the frozen walk keeps
                    return was_lit != now_lit;
                };
                // THE CANBURN CHECK (pc 199 `ifeq 328`): the failed
                // fit the timer zero wears
                if !can_burn(slots.get(2).copied().unwrap_or((0, 0)), assembled) {
                    data[DATA_COOKING_PROGRESS] = 0;
                    return was_lit != now_lit;
                }
                // THE IGNITE ARM (pc 202-249): the NOT-lit state the
                // fuel's own burn duration wears — the both-total
                // write banks, the >0 gate the consume speaks
                if !now_lit {
                    let burn = mc_registry::generated::furnace_data::fuel_ticks(slots[1].0);
                    data[DATA_LIT_TIME] = burn;
                    data[DATA_LIT_DURATION] = burn;
                    if burn > 0 {
                        consume_fuel(slots);
                        now_lit = true;
                    }
                }
                // THE COOK WALK (pc 251-316): the lit state alone
                // advances — the timer the +1 speaks, the total-time
                // equality the burn answers
                if now_lit {
                    data[DATA_COOKING_PROGRESS] += 1;
                    if data[DATA_COOKING_PROGRESS] == data[DATA_COOKING_TOTAL_TIME] {
                        data[DATA_COOKING_PROGRESS] = 0;
                        data[DATA_COOKING_TOTAL_TIME] = DEFAULT_COOKING_TOTAL_TIME;
                        burn_items(slots, input, assembled);
                    }
                } else {
                    // pc 320-325: the failed ignite the timer zeroes
                    // (the not-really-fuel row the 0 duration wears)
                    data[DATA_COOKING_PROGRESS] = 0;
                }
            } else {
                // pc 336-341: the lit furnace the empty input the
                // timer zero wears
                data[DATA_COOKING_PROGRESS] = 0;
            }
        } else {
            // THE COOLDOWN WALK (pc 344-369): the timer the -2 the
            // clamp keeps (the BURN_COOL_SPEED constant serves, the
            // 0..=total band the Mth.clamp answers)
            let t = data[DATA_COOKING_PROGRESS];
            if t > 0 {
                data[DATA_COOKING_PROGRESS] =
                    (t - BURN_COOL_SPEED).clamp(0, data[DATA_COOKING_TOTAL_TIME]);
            }
        }
        // THE LIT FLIP'S OWN VERDICT (pc 369-373): the wasLit != isLit
        // comparison the changed flag speaks
        was_lit != now_lit
    }
}

/// THE CANBURN WALK (javap canBurn pcs 0-61 — the empty result the
/// TRUE speaks, the different item the FALSE wears, the count sum the
/// max-stack test the 64 default answers).
fn can_burn(result: (i32, i32), assembled: (i32, i32)) -> bool {
    if result.1 <= 0 {
        return true;
    }
    if result.0 != assembled.0 {
        return false;
    }
    result.1 + assembled.1 <= crate::world_view::ITEM_MAX_STACK_DEFAULT
}

/// THE CONSUME FUEL WALK (javap consumeFuel pcs 0-42 — the shrink the
/// count speaks, the empty slot the crafting remainder takes: the
/// lava bucket the empty bucket the walk banks, the coal the nothing
/// wears).
fn consume_fuel(slots: &mut [(i32, i32)]) {
    let Some(fuel) = slots.get_mut(1) else { return };
    if fuel.1 <= 0 {
        return;
    }
    fuel.1 -= 1;
    if fuel.1 <= 0 {
        // THE REMAINDER'S OWN WALK (javap pc 17-38 — the
        // getCraftingRemainder the item answers): the lava bucket the
        // empty bucket speaks, every other item the EMPTY wears.
        *fuel = if fuel.0 == LAVA_BUCKET {
            (BUCKET, 1)
        } else {
            (0, 0)
        };
    }
}

/// THE BURN WALK (javap burn pcs 0-99 — the empty result the copy
/// takes, the same-item result the grow wears, the input the shrink
/// loses; the WET_SPONGE + BUCKET special the water bucket speaks).
fn burn_items(slots: &mut [(i32, i32)], input: (i32, i32), assembled: (i32, i32)) {
    match slots.get_mut(2) {
        Some(slot) if slot.1 <= 0 => *slot = assembled,
        Some(slot) => slot.1 += assembled.1,
        None => {}
    }
    if let Some(inp) = slots.first_mut() {
        if inp.1 > 0 {
            inp.1 -= 1;
            if inp.1 <= 0 {
                *inp = (0, 0);
            }
        }
    }
    // THE WET SPONGE'S OWN BUCKET SWAP (javap pc 37-93 — the input
    // the WET_SPONGE item answers, the fuel slot the BUCKET holds,
    // the water bucket the set wears)
    if input.0 == WET_SPONGE && slots.get(1).is_some_and(|s| s.1 > 0 && s.0 == BUCKET) {
        if let Some(fuel) = slots.get_mut(1) {
            *fuel = (WATER_BUCKET, 1);
        }
    }
}

/// THE SIM'S OWN PIN WALKS (the tables the extractor generated, the
/// tick the javap walk serves — the arcs the bytecode's own order
/// keeps).
#[cfg(test)]
mod tests {
    use super::*;

    fn wire(name: &str) -> i32 {
        mc_registry::item_id_by_name(name).unwrap()
    }

    #[test]
    fn the_smelting_rows_the_pairs_wear() {
        // THE 26.2 ROWS (the jar's data/minecraft/recipe/*.json — the
        // plain ingredients the extractor flattened, the cookingtime
        // the clinit's own 200 default wears everywhere)
        let (r, c) = mc_registry::generated::furnace_data::smelt_result(wire("raw_iron"))
            .unwrap();
        assert_eq!((r, c), (wire("iron_ingot"), 1), "the raw iron's own row");
        let (r, _) = mc_registry::generated::furnace_data::smelt_result(wire("iron_ore"))
            .unwrap();
        assert_eq!(r, wire("iron_ingot"), "the ore row the same ingot wears");
        let (r, _) = mc_registry::generated::furnace_data::smelt_result(
            wire("deepslate_iron_ore"),
        )
        .unwrap();
        assert_eq!(r, wire("iron_ingot"), "the deepslate variant the same ingot wears");
        let (r, _) = mc_registry::generated::furnace_data::smelt_result(wire("sand")).unwrap();
        assert_eq!(r, wire("glass"), "the sand's own glass row");
        let (r, _) =
            mc_registry::generated::furnace_data::smelt_result(wire("cobblestone")).unwrap();
        assert_eq!(r, wire("stone"), "the cobblestone's own stone row");
        let (r, _) = mc_registry::generated::furnace_data::smelt_result(wire("potato")).unwrap();
        assert_eq!(r, wire("baked_potato"), "the food row rides");
        let (r, _) =
            mc_registry::generated::furnace_data::smelt_result(wire("ancient_debris")).unwrap();
        assert_eq!(r, wire("netherite_scrap"), "the debris' own scrap row");
        let (r, _) =
            mc_registry::generated::furnace_data::smelt_result(wire("wet_sponge")).unwrap();
        assert_eq!(r, wire("sponge"), "the sponge row rides (the bucket special's own input)");
        let (r, _) = mc_registry::generated::furnace_data::smelt_result(wire("clay_ball")).unwrap();
        assert_eq!(r, wire("brick"), "the brick row rides");
        // THE TAG ROWS (the #minecraft:logs_that_burn the charcoal
        // recipe wears — the tag string the extractor resolved)
        let (r, _) = mc_registry::generated::furnace_data::smelt_result(wire("oak_log")).unwrap();
        assert_eq!(r, wire("charcoal"), "the log's own charcoal row");
        // THE LIST ROW (the copper gear's own nugget walk — the 12
        // strings the ingredient list carries)
        let (r, _) =
            mc_registry::generated::furnace_data::smelt_result(wire("copper_pickaxe")).unwrap();
        assert_eq!(r, wire("copper_nugget"), "the gear's own nugget row");
        // THE NO-RECIPE ARMS (the None the early return keeps)
        assert!(
            mc_registry::generated::furnace_data::smelt_result(wire("dirt")).is_none(),
            "the dirt no recipe speaks"
        );
        assert!(
            mc_registry::generated::furnace_data::smelt_result(wire("oak_planks")).is_none(),
            "the planks no recipe speaks"
        );
        assert!(
            mc_registry::generated::furnace_data::smelt_result(0).is_none(),
            "the air no recipe speaks"
        );
    }

    #[test]
    fn the_fuel_rows_the_ticks_wear() {
        // THE DIRECT ROWS (javap FuelValues.vanillaBurnTimes — the
        // base-200 walk the literals folded)
        let f = |name: &str| mc_registry::generated::furnace_data::fuel_ticks(wire(name));
        assert_eq!(f("coal"), 1600, "the 200*8 coal speaks");
        assert_eq!(f("charcoal"), 1600, "the 200*8 charcoal speaks");
        assert_eq!(f("coal_block"), 16000, "the 200*8*10 block speaks");
        assert_eq!(f("lava_bucket"), 20000, "the 200*100 bucket speaks");
        assert_eq!(f("blaze_rod"), 2400, "the 200*12 rod speaks");
        assert_eq!(f("dried_kelp_block"), 4000, "the 200*20 kelp speaks");
        assert_eq!(f("stick"), 100, "the 200/2 stick speaks");
        assert_eq!(f("bowl"), 100, "the 200/2 bowl speaks");
        assert_eq!(f("bamboo"), 50, "the 200/4 bamboo speaks");
        assert_eq!(f("scaffolding"), 50, "the 200/4 scaffold speaks");
        assert_eq!(f("wooden_pickaxe"), 200, "the tool's own 200 speaks");
        // THE TAG ROWS (the jar's item tags the walk consults)
        assert_eq!(f("oak_log"), 300, "the 200*3/2 log speaks");
        assert_eq!(f("oak_planks"), 300, "the 200*3/2 planks speak");
        assert_eq!(f("white_wool"), 100, "the 200/2 wool speaks");
        assert_eq!(f("oak_boat"), 1200, "the 200*6 boat speaks");
        assert_eq!(f("oak_slab"), 150, "the 200*3/4 slab speaks");
        assert_eq!(f("oak_sign"), 200, "the signs' own base speaks");
        assert_eq!(f("oak_hanging_sign"), 800, "the 200*4 hanging speaks");
        assert_eq!(f("dead_bush"), 100, "the bush's own 100 speaks");
        // THE REMOVAL (the tail's remove(NON_FLAMMABLE_WOOD) — the
        // stems the #logs tag carried the map loses)
        assert_eq!(f("crimson_stem"), 0, "the stem the removal never burns");
        assert_eq!(f("warped_stem"), 0, "the stem the removal never burns");
        // THE NON-FUEL ARMS (the 0 the map's misses wear)
        assert_eq!(f("dirt"), 0, "the dirt no fuel speaks");
        assert_eq!(f("stone"), 0, "the stone no fuel speaks");
        // THE OUT-OF-RANGE ARMS (the 0 the bounds read answers)
        assert_eq!(
            mc_registry::generated::furnace_data::fuel_ticks(-1),
            0,
            "the negative id the bounds answer keeps"
        );
        assert_eq!(
            mc_registry::generated::furnace_data::fuel_ticks(i32::MAX),
            0,
            "the huge id the bounds answer keeps"
        );
    }

    #[test]
    fn the_tick_the_ignite_the_smelt_the_cooldown_wear() {
        // THE COLD IGNITE (the not-lit + fuel + input walk — the burn
        // the words bank, the coal the consume loses, the lit flip
        // the TRUE speaks)
        let mut sim = FurnaceSim::default();
        let mut slots = [(wire("raw_iron"), 5), (wire("coal"), 3), (0, 0)];
        let mut data = [0; 4];
        assert!(sim.tick(&mut slots, &mut data), "the ignite flips");
        assert_eq!(data[DATA_LIT_TIME], 1600, "the coal's own burn banks");
        assert_eq!(data[DATA_LIT_DURATION], 1600, "the total banks too");
        assert_eq!(slots[1], (wire("coal"), 2), "the coal the consume loses");
        assert_eq!(data[DATA_COOKING_PROGRESS], 1, "the same tick's cook speaks");
        // THE WARM COOK (the timer climbs — the 200th the burn lands:
        // the result the copy takes, the input the shrink loses)
        for _ in 0..198 {
            sim.tick(&mut slots, &mut data);
        }
        assert_eq!(data[DATA_COOKING_PROGRESS], 199, "the climb speaks");
        assert!(!sim.tick(&mut slots, &mut data), "the warm tick no flip");
        assert_eq!(data[DATA_COOKING_PROGRESS], 0, "the completion zeroes");
        assert_eq!(slots[2], (wire("iron_ingot"), 1), "the ingot the burn lands");
        assert_eq!(slots[0], (wire("raw_iron"), 4), "the input the shrink loses");
        // THE CONTINUOUS BURN (the second arc the SAME coal wears —
        // no second ignite while lit; the decrement the climb keeps)
        for _ in 0..199 {
            sim.tick(&mut slots, &mut data);
        }
        assert_eq!(data[DATA_COOKING_PROGRESS], 199, "the second climb speaks");
        sim.tick(&mut slots, &mut data);
        assert_eq!(slots[2], (wire("iron_ingot"), 2), "the second ingot lands");
        assert_eq!(slots[0], (wire("raw_iron"), 3), "the second shrink loses");
        assert_eq!(data[DATA_LIT_TIME], 1201, "the first coal still burns (1600-399)");
        // THE EXHAUSTION FLIP (the fuel the empty wears — the last lit
        // tick the decrement spends, the flip the TRUE speaks; the
        // timer the mid-arc value the cooldown chews)
        slots[1] = (0, 0);
        data[DATA_COOKING_PROGRESS] = 150;
        data[DATA_LIT_TIME] = 2;
        assert!(!sim.tick(&mut slots, &mut data), "the lit tick no flip yet");
        assert_eq!(data[DATA_LIT_TIME], 1, "the last lit tick holds");
        assert!(sim.tick(&mut slots, &mut data), "the exhaustion flips");
        assert_eq!(data[DATA_LIT_TIME], 0, "the zero speaks");
        // THE COOLDOWN (the unlit + no fuel walk — the timer the -2
        // the clamp keeps, the floor the zero holds)
        let t = data[DATA_COOKING_PROGRESS];
        assert_eq!(t, 149, "the cook tick the flip wore (150+1-2)");
        assert!(t > 2, "the mid-arc timer the cooldown wears");
        sim.tick(&mut slots, &mut data);
        assert_eq!(data[DATA_COOKING_PROGRESS], t - 2, "the -2 spoke");
        for _ in 0..100 {
            sim.tick(&mut slots, &mut data);
        }
        assert_eq!(data[DATA_COOKING_PROGRESS], 0, "the floor holds");
        // THE REIGNITE (the fuel again — the fresh coal the walk
        // consumes, the flip the TRUE wears)
        slots[1] = (wire("coal"), 1);
        assert!(sim.tick(&mut slots, &mut data), "the reignite flips");
        assert_eq!(slots[1], (0, 0), "the last coal the consume empties");
        assert_eq!(data[DATA_LIT_TIME], 1600, "the fresh burn banks");
    }

    #[test]
    fn the_no_recipe_the_freeze_the_nonfuel_zero_wear() {
        // THE NO-RECIPE FREEZE (pc 154-156 — the not-lit + fuel +
        // input walk the recipe None the EARLY return wears: no
        // ignite, no consume, no timer — the setItem reset the tick
        // head already spoke: the total the 200 default wears, the
        // timer the zero)
        let mut sim = FurnaceSim::default();
        let mut slots = [(wire("dirt"), 4), (wire("coal"), 3), (0, 0)];
        let mut data = [0; 4];
        assert!(!sim.tick(&mut slots, &mut data), "the freeze no flip");
        assert_eq!(data, [0, 0, 0, 200], "the reset spoke, nothing else rode");
        assert_eq!(slots[1], (wire("coal"), 3), "the fuel the freeze keeps");
        // THE SECOND FREEZE TICK (the same input the memory holds —
        // no reset, the same frozen shape)
        sim.tick(&mut slots, &mut data);
        assert_eq!(data, [0, 0, 0, 200], "the freeze the words keep");
        // THE NON-FUEL ZERO (the ignite arm the 0 duration wears —
        // the now_lit false stays, the timer zeroes: pc 320-325)
        let mut slots = [(wire("raw_iron"), 4), (wire("dirt"), 3), (0, 0)];
        let mut data = [0, 0, 150, 200];
        assert!(!sim.tick(&mut slots, &mut data), "the failed ignite no flip");
        assert_eq!(data[DATA_LIT_TIME], 0, "the zero burn banks");
        assert_eq!(data[DATA_LIT_DURATION], 0, "the zero total banks");
        assert_eq!(slots[1], (wire("dirt"), 3), "the dirt the consume refuses");
        assert_eq!(data[DATA_COOKING_PROGRESS], 0, "the timer the zero speaks");
    }

    #[test]
    fn the_setitem_reset_the_input_change_wears() {
        // THE ITEM CHANGE (the recipe's own reset — the timer the
        // zero wears, the total the 200 speaks, the memory updates)
        let mut sim = FurnaceSim::default();
        let mut slots = [(wire("raw_iron"), 2), (wire("coal"), 1), (0, 0)];
        let mut data = [0, 0, 137, 200];
        sim.tick(&mut slots, &mut data);
        assert_eq!(data[DATA_COOKING_PROGRESS], 1, "the fresh input cooks");
        // THE COUNT-ONLY CHANGE NEVER RESETS (the isSameItemSame-
        // Components test the item identity alone compares)
        data[DATA_COOKING_PROGRESS] = 99;
        slots[0] = (wire("raw_iron"), 5);
        sim.tick(&mut slots, &mut data);
        assert_eq!(data[DATA_COOKING_PROGRESS], 100, "the count change the climb keeps");
        // THE ITEM CHANGE THE RESET WEARS (the mid-burn swap — the
        // iron's arc dies, the sand's arc starts)
        slots[0] = (wire("sand"), 8);
        sim.tick(&mut slots, &mut data);
        assert_eq!(data[DATA_COOKING_PROGRESS], 1, "the reset the fresh start wears");
        assert_eq!(data[DATA_COOKING_TOTAL_TIME], 200, "the total the recipe speaks");
        // THE EMPTY-THEN-REFILL THE SAME RESET WEARS (the slot the
        // empty reads — the id the 0 the memory takes, the refill the
        // change answers)
        slots[0] = (0, 0);
        sim.tick(&mut slots, &mut data);
        slots[0] = (wire("cobblestone"), 1);
        sim.tick(&mut slots, &mut data);
        assert_eq!(data[DATA_COOKING_PROGRESS], 1, "the refill's own start wears");
    }

    #[test]
    fn the_bucket_the_remainder_the_sponge_the_refusals_wear() {
        // THE LAVA BUCKET'S OWN REMAINDER (the consume's empty-slot
        // walk — the empty bucket the walk banks, the burn the 20000
        // ticks carries)
        let mut sim = FurnaceSim::default();
        let mut slots = [(wire("sand"), 2), (wire("lava_bucket"), 1), (0, 0)];
        let mut data = [0; 4];
        assert!(sim.tick(&mut slots, &mut data), "the ignite flips");
        assert_eq!(data[DATA_LIT_TIME], 20000, "the bucket's own burn banks");
        assert_eq!(slots[1], (wire("bucket"), 1), "the empty bucket remains");
        // THE WET SPONGE'S OWN SWAP (the burn walk's special — the
        // bucket the fuel holds, the water bucket the set wears; the
        // sim's memory the pre-warm speaks so the tick-head reset
        // never zeroes the edge timer)
        let mut sim = FurnaceSim::default();
        let mut slots = [(wire("wet_sponge"), 2), (wire("bucket"), 1), (0, 0)];
        let mut data = [1600, 1600, 199, 200]; // lit, the timer at the edge
        sim.last_input_id = wire("wet_sponge"); // the memory pre-warmed — no reset
        assert!(!sim.tick(&mut slots, &mut data), "the warm lit tick no flip");
        assert_eq!(slots[2], (wire("sponge"), 1), "the sponge the burn lands");
        assert_eq!(slots[1], (wire("water_bucket"), 1), "the water bucket the swap wears");
        assert_eq!(slots[0], (wire("wet_sponge"), 1), "the input the shrink loses");
        // THE CANBURN'S OWN REFUSAL (pc 199 the canBurn walk rides
        // BEFORE the ignite arm — the different item the result
        // holds: NO ignite, the timer the zero wears, the fuel the
        // refusal keeps)
        let mut sim = FurnaceSim::default();
        let mut slots = [(wire("raw_iron"), 2), (wire("coal"), 1), (wire("glass"), 4)];
        let mut data = [0; 4];
        assert!(!sim.tick(&mut slots, &mut data), "the refusal no flip");
        assert_eq!(data[DATA_LIT_TIME], 0, "the ignite the refusal never spoke");
        assert_eq!(slots[1], (wire("coal"), 1), "the fuel the refusal keeps");
        assert_eq!(data[DATA_COOKING_PROGRESS], 0, "the timer the refusal zeroes");
        // THE STACK LIMIT (the same item the 64 the law speaks — the
        // 63+1 the last smelt the boundary carries, the next arc the
        // 65 the refusal refuses)
        let mut sim = FurnaceSim::default();
        let mut slots = [(wire("raw_iron"), 2), (wire("coal"), 1), (wire("iron_ingot"), 63)];
        let mut data = [0; 4];
        sim.tick(&mut slots, &mut data);
        for _ in 0..199 {
            sim.tick(&mut slots, &mut data);
        }
        assert_eq!(slots[2], (wire("iron_ingot"), 64), "the 63+1 the last smelt banks");
        assert_eq!(slots[0], (wire("raw_iron"), 1), "the input the smelt loses");
        data[DATA_COOKING_PROGRESS] = 199;
        for _ in 0..200 {
            sim.tick(&mut slots, &mut data);
        }
        assert_eq!(slots[2], (wire("iron_ingot"), 64), "the full stack the burn refuses");
        assert_eq!(slots[0], (wire("raw_iron"), 1), "the input the refusal keeps");
        assert_eq!(data[DATA_COOKING_PROGRESS], 0, "the timer the refusal zeroes");
    }

}

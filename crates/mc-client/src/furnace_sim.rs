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

/// THE FAST FAMILY'S OWN COOKING DEFAULT (javap BlastingRecipe
/// static{} pc 5-7 `bipush 100` + SmokingRecipe static{} pc 5-7
/// `bipush 100` the cookingMapCodec(I) defaults wear — every 26.2
/// blasting/smoking row omits the cookingtime field too, so the
/// recipe-time fetch speaks 100 ticks — the furnace's HALF the fast
/// pair's own speed the vanilla names).
pub const FAST_COOKING_TOTAL_TIME: i32 = 100;

/// THE COOKING FAMILY'S OWN KIND (the AbstractFurnaceBlockEntity
/// subclass the three registrations speak — the RecipeType field the
/// ctors bank: FURNACE pc 0-12 SMELTING, BLAST_FURNACE pc 0-12
/// BLASTING, SMOKER pc 0-12 SMOKING; the menu type ids the MenuType
/// clinit's registration order keeps — FURNACE 14, BLAST_FURNACE 10,
/// SMOKER 22).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FurnaceKind {
    /// The FurnaceMenu's own slow lane (RecipeType.SMELTING, the
    /// codec default 200 the SmeltingRecipe clinit keeps, the fuel
    /// the FULL burn duration the base class's getBurnDuration
    /// speaks).
    #[default]
    Furnace,
    /// The BlastFurnaceMenu's own ore lane (RecipeType.BLASTING, the
    /// codec default 100 the BlastingRecipe clinit keeps, the fuel
    /// the HALF burn duration the getBurnDuration override's
    /// `iconst_2; idiv` walk speaks — javap BlastFurnaceBlockEntity
    /// pcs 0-8: the super call, the 2-divide, the return).
    BlastFurnace,
    /// The SmokerMenu's own food lane (RecipeType.SMOKING, the codec
    /// default 100 the SmokingRecipe clinit keeps, the fuel the HALF
    /// burn duration the same override speaks — javap
    /// SmokerBlockEntity pcs 0-8).
    Smoker,
}

impl FurnaceKind {
    /// THE PER-TYPE RECIPE TABLE (the AbstractFurnaceBlockEntity's
    /// own RecipeType field the subclass ctors bank — the
    /// quickCheck walk the type's own recipe map answers).
    pub fn recipe(self, input_id: i32) -> Option<(i32, i32)> {
        match self {
            FurnaceKind::Furnace => mc_registry::generated::furnace_data::smelt_result(input_id),
            FurnaceKind::BlastFurnace => {
                mc_registry::generated::furnace_data::blast_result(input_id)
            }
            FurnaceKind::Smoker => mc_registry::generated::furnace_data::smoke_result(input_id),
        }
    }

    /// THE GETTOTALCOOKTIME WALK (javap getTotalCookTime — the
    /// recipe's cookingTime() the Optional carries, the `sipush 200`
    /// the no-recipe fallback keeps for EVERY type; the recipe rows
    /// the per-type codec defaults answer — 200 smelting, 100
    /// blasting/smoking, the cookingtime field every 26.2 row omits).
    pub fn total_cook_time(self, input_id: i32) -> i32 {
        match self.recipe(input_id) {
            Some(_) if self != FurnaceKind::Furnace => FAST_COOKING_TOTAL_TIME,
            Some(_) => DEFAULT_COOKING_TOTAL_TIME,
            None => DEFAULT_COOKING_TOTAL_TIME,
        }
    }

    /// THE GETBURNDURATION WALK (javap AbstractFurnaceBlockEntity.
    /// getBurnDuration the base fuel-values fetch; the BLAST/SMOKER
    /// subclasses' override the `iconst_2; idiv` HALF speaks —
    /// BlastFurnaceBlockEntity pcs 0-8, SmokerBlockEntity pcs 0-8 —
    /// the coal the 1600/2 = 800 the fast pair's own half-duration
    /// keeps, the furnace the base itself wears). The integer
    /// division the idiv speaks (odd base the floor keeps).
    pub fn burn_ticks(self, base: i32) -> i32 {
        match self {
            FurnaceKind::Furnace => base,
            FurnaceKind::BlastFurnace | FurnaceKind::Smoker => base / 2,
        }
    }

    /// The kind's own lit kind (the LIT flip's world walk the sim's
    /// verdict wears — the setBlock LIT swap the caller answers).
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn lit_kind(self) -> u16 {
        match self {
            FurnaceKind::Furnace => crate::world::FURNACE_LIT,
            FurnaceKind::BlastFurnace => crate::world::BLAST_FURNACE_LIT,
            FurnaceKind::Smoker => crate::world::SMOKER_LIT,
        }
    }

    /// The kind's own unlit kind (the same pair's first half).
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn unlit_kind(self) -> u16 {
        match self {
            FurnaceKind::Furnace => crate::world::FURNACE,
            FurnaceKind::BlastFurnace => crate::world::BLAST_FURNACE,
            FurnaceKind::Smoker => crate::world::SMOKER,
        }
    }
}

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
    /// THE KIND'S OWN LANE (the dev-26 round — the
    /// AbstractFurnaceBlockEntity subclass the block kind answers: the
    /// furnace the SMELTING base lane keeps, the blast furnace /
    /// smoker the BLASTING/SMOKING overrides speak). The default the
    /// furnace keeps (the base class lane).
    pub kind: FurnaceKind,
}

impl FurnaceSim {
    /// THE SETITEM WALK'S OWN RESET (javap setItem pc 53-99 — the
    /// input slot's item CHANGE the cooking words re-fetch speaks:
    /// the recipe's own cooking time, the no-recipe arm the sipush-200
    /// default keeps — both arms the same 200 in 26.2 for the furnace;
    /// the blast/smoker the recipe arm's 100 the codec defaults keep).
    /// The count changes NEVER reset (the isSameItemSameComponents
    /// test the item identity alone compares).
    pub fn on_input_set(&mut self, data: &mut [i32; 4], input_id: i32) {
        if input_id != self.last_input_id {
            // THE GETTOTALCOOKTIME WALK (javap getTotalCookTime — the
            // recipe's cookingTime() the Optional carries, the
            // orElse(200) the no-recipe arm wears; the generated
            // table's row the same default answers)
            data[DATA_COOKING_TOTAL_TIME] = self.kind.total_cook_time(input_id);
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
                // SingleRecipeInput walk serves; the kind's own
                // RecipeType the subclass ctors bank the table answers)
                let input = slots[0];
                let Some(assembled) = self.kind.recipe(input.0) else {
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
                // write banks, the >0 gate the consume speaks. The
                // GETBURNDURATION WALK the kind's override answers
                // (the blast/smoker the `iconst_2; idiv` HALF keeps).
                if !now_lit {
                    let burn =
                        self.kind
                            .burn_ticks(mc_registry::generated::furnace_data::fuel_ticks(
                                slots[1].0,
                            ));
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
                        // the getTotalCookTime re-fetch the burn walk
                        // keeps (the same kind-aware fetch the reset
                        // speaks)
                        data[DATA_COOKING_TOTAL_TIME] =
                            self.kind.total_cook_time(slots[0].0);
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
    result.1 + assembled.1 <= crate::world_view::item_max_stack(result.0)
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

/// THE FURNACE'S OWN ANIMATE-TICK VERDICT (javap `FurnaceBlock.animateTick`
/// pcs 0-2xx over the 26.2 client.jar — the LIT state's own client walk
/// the crackle + the front-face ember pair speak): the sound gate's
/// verdict + the particle position's three offsets (the d5/d6/d7
/// dstore lanes the bytecode banks).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FurnaceAnimateTick {
    /// The `nextDouble() < 0.1` gate's verdict (draw 1) — the
    /// playLocalSound arm the BLOCKS source speaks at the block's own
    /// bottom-center anchor (x + 0.5, y, z + 0.5), volume 1.0, pitch
    /// 1.0 (the fconst_1 pair pc 73-74 keeps), distanceDelay false.
    pub crackle: bool,
    /// d5 — the lateral x offset: the axis-X facing the
    /// `getStepX() * 0.52` fold speaks, the lateral draw d4 the rest
    /// answers.
    pub dx: f64,
    /// d6 — the height offset (draw 3: `nextDouble() * 6.0 / 16.0`).
    pub dy: f64,
    /// d7 — the lateral z offset: the axis-Z facing the
    /// `getStepZ() * 0.52` fold speaks, the lateral draw d4 the rest
    /// answers.
    pub dz: f64,
}

/// THE FURNACE'S OWN ANIMATE TICK (javap FurnaceBlock.animateTick over
/// the 26.2 client.jar, the walk's own draw order kept — draw 1 the
/// sound gate pc 46-56, draw 2 the lateral d4 = r × 0.6 − 0.3
/// pc 103-118, draw 3 the height d6 = r × 6.0 / 16.0 pc 145-160; the
/// two axis arms the facing's own vocabulary decodes: pc 120-143
/// `axis == X ? getStepX() * 0.52 : d4` into d5, pc 162-185
/// `axis == Z ? getStepZ() * 0.52 : d4` into d7; the byte law the
/// no-opinion 0 the north default the mesher's front-tile law keeps):
/// * facing 2 east — axis X, stepX +1: dx = +0.52, dz = d4
/// * facing 4 west — axis X, stepX −1: dx = −0.52, dz = d4
/// * facing 3 south — axis Z, stepZ +1: dz = +0.52, dx = d4
/// * facing 1 north / 0 the no-opinion — axis Z, stepZ −1:
///   dz = −0.52, dx = d4
///
/// The caller spends the three draws ONLY for the LIT kind (the gate
/// pc 13-16 `ifne 17 / return` the unlit never passes — the sampled
/// air the walk never reaches), and the draws ride even when the
/// sound gate fails (the smoke/flame pair the tail always speaks —
/// the particles the level's addParticle walk spawns at
/// (x + 0.5 + d5, y + d6, z + 0.5 + d7), SMOKE first, FLAME second,
/// the zero velocity args the dconst_0 trio keeps).
pub fn animate_tick(facing: u8, draw1: f64, draw2: f64, draw3: f64) -> FurnaceAnimateTick {
    let lateral = draw2 * 0.6 - 0.3;
    let (dx, dz) = match facing {
        2 => (0.52, lateral),
        4 => (-0.52, lateral),
        3 => (lateral, 0.52),
        // 1 north + the 0 no-opinion: the mesher's own default arm
        _ => (lateral, -0.52),
    };
    FurnaceAnimateTick {
        crackle: draw1 < 0.1,
        dx,
        dy: draw3 * 6.0 / 16.0,
        dz,
    }
}

/// THE BLAST FURNACE'S OWN ANIMATE TICK (javap `BlastFurnaceBlock.
/// animateTick` over the 26.2 client.jar — the furnace's own walk the
/// SAME constants keep, the ONE TEXTUAL DIFFERENCE the height draw's
/// literal banks: pc 152-156 `ldc2_w 9.0d; dmul; ldc2_w 16.0d; ddiv`
/// the furnace's `6.0d` literal the 9.0 replaces — the smoke the
/// TALLER band the fast furnace's own flame column speaks). The ONE
/// PARTICLE DIFFERENCE the tail keeps: pc 187-209 the SINGLE
/// addParticle(SMOKE) call, the FLAME pair the furnace's pc 187-232
/// speaks ABSENT — the blast furnace the smoke alone spawns at
/// (x + 0.5 + d5, y + d6, z + 0.5 + d7), the zero velocity args the
/// same dconst_0 trio keeps. The sound the BLASTFURNACE_FIRE_CRACKLE
/// event the pc 67 field banks ("block.blastfurnace.fire_crackle" the
/// registration name carries — javap SoundEvents ldc pc 1746), the
/// same 0.1 gate the same BLOCKS source the same volume/pitch pair
/// keeps. The draw order the furnace's own law keeps: gate, lateral,
/// height — three draws per sample, the same lateral/axis folds.
pub fn blast_animate_tick(facing: u8, draw1: f64, draw2: f64, draw3: f64) -> FurnaceAnimateTick {
    let mut v = animate_tick(facing, draw1, draw2, draw3);
    v.dy = draw3 * 9.0 / 16.0;
    v
}

/// THE SMOKER'S OWN ANIMATE TICK (javap `SmokerBlock.animateTick` over
/// the 26.2 client.jar — the SIMPLEST tail the trio wears): the LIT
/// gate the same pair speaks, the (x+0.5, y, z+0.5) anchor reads the
/// same banks, the sound gate the SAME 0.1 verdict draws (the
/// SMOKER_SMOKE event the pc 67 field banks — "block.smoker.smoke"
/// the registration name carries, javap SoundEvents ldc pc 13521) —
/// and the tail the ONE addParticle(SMOKE) at (x + 0.5, y + 1.1,
/// z + 0.5) speaks: the `ldc2_w 1.1; dadd` the +1.1 height the smoke
/// ABOVE the block answers, NO facing reads, NO lateral/height draws
/// — the ONLY random the walk spends is the gate's own. The draw law
/// the caller's sample walk keeps: ONE draw per visit, the verdict's
/// crackle flag the gate's own answer, the smoke the caller spawns at
/// the fixed (x + 0.5, y + 1.1, z + 0.5) anchor (the verdict struct's
/// offsets the no-opinion zeros keep).
pub fn smoker_animate_tick(draw1: f64) -> FurnaceAnimateTick {
    FurnaceAnimateTick {
        crackle: draw1 < 0.1,
        dx: 0.0,
        dy: 0.0,
        dz: 0.0,
    }
}

/// THE LEVEL'S OWN ANIMATE-TICK SAMPLER (javap ClientLevel.animateTick
/// pcs 0-71 + doAnimateTick pcs 0-92 over the 26.2 client.jar): the
/// 667-iteration loop the TWO ranges wears — per iteration the first
/// doAnimateTick the `bipush 16` arm speaks, the second the
/// `bipush 32` — each the `pos + nextInt(range) − nextInt(range)`
/// fold per axis draws (the SIX draws per sample the level's own
/// stream spends — the `getfield random` pair pc 1-19/27-45/54-72 the
/// bytecode keeps), the sampled cell the visitor answers. The call
/// order the bytecode keeps: the range-16 arm FIRST, the range-32 arm
/// SECOND, per iteration; the sampled state the block's own
/// animateTick the caller's furnace arm answers (the fluid tail the
/// client's vocabulary never hears — no fluid animate-tick voice
/// speaks yet).
///
/// The generator law the replay policy keeps: the sampler's positions
/// ride the LEVEL random analog (vanilla `ClientLevel.random` — the
/// `level_ambient_rng` lane the client names), the block's own three
/// draws the `animate_rng` lane the thread-local random analog
/// carries (vanilla `RandomSource.createThreadLocalInstance()` the
/// animateTick prologue creates fresh per tick — non-replayable by
/// nature, the persistent seeded LCG the stand-in the house law
/// speaks).
pub fn animate_sample_walk(
    px: i32,
    py: i32,
    pz: i32,
    rng: &mut crate::drops::Lcg,
    mut visit: impl FnMut(i32, i32, i32),
) {
    for _ in 0..667 {
        for range in [16u64, 32] {
            let sx = px + rng.next_int(range) - rng.next_int(range);
            let sy = py + rng.next_int(range) - rng.next_int(range);
            let sz = pz + rng.next_int(range) - rng.next_int(range);
            visit(sx, sy, sz);
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

    /// THE FAST LANES' OWN LAWS (javap the dev-26 round — the
    /// BlastingRecipe/SmokingRecipe clinit `bipush 100` codec
    /// defaults, the getBurnDuration overrides' `iconst_2; idiv`
    /// halves, the three tables' own rows the extractor generated).
    #[test]
    fn the_fast_lanes_the_tables_the_halves_wear() {
        // THE THREE TABLES' OWN ROWS (the extractor's type filter —
        // the blasting rows the ORE-family answers, the smoking rows
        // the FOOD answers, the cross-table rows the other types
        // never speak)
        assert_eq!(
            FurnaceKind::BlastFurnace.recipe(wire("iron_ore")),
            Some((wire("iron_ingot"), 1)),
            "the blast furnace smelts the ore"
        );
        assert_eq!(
            FurnaceKind::BlastFurnace.recipe(wire("raw_iron")),
            Some((wire("iron_ingot"), 1)),
            "the raw iron the blast answers too"
        );
        assert_eq!(
            FurnaceKind::BlastFurnace.recipe(wire("potato")),
            None,
            "the food row the blast table never carries"
        );
        assert_eq!(
            FurnaceKind::BlastFurnace.recipe(wire("cobblestone")),
            None,
            "the smelting-only stone row the blast refuses"
        );
        assert_eq!(
            FurnaceKind::Smoker.recipe(wire("potato")),
            Some((wire("baked_potato"), 1)),
            "the smoker cooks the potato"
        );
        assert_eq!(
            FurnaceKind::Smoker.recipe(wire("iron_ore")),
            None,
            "the ore row the smoker table never carries"
        );
        assert_eq!(
            FurnaceKind::Furnace.recipe(wire("cobblestone")),
            Some((wire("stone"), 1)),
            "the furnace's own stone row keeps"
        );
        // THE GETTOTALCOOKTIME WALK (the recipe arm the per-type
        // defaults keep — 200 smelting, 100 blasting/smoking; the
        // no-recipe arm the sipush-200 fallback EVERY type keeps)
        assert_eq!(
            FurnaceKind::Furnace.total_cook_time(wire("raw_iron")),
            200,
            "the furnace's own 200 keeps"
        );
        assert_eq!(
            FurnaceKind::BlastFurnace.total_cook_time(wire("iron_ore")),
            100,
            "the blast's own 100 the codec speaks"
        );
        assert_eq!(
            FurnaceKind::Smoker.total_cook_time(wire("potato")),
            100,
            "the smoker's own 100 the codec speaks"
        );
        assert_eq!(
            FurnaceKind::BlastFurnace.total_cook_time(wire("dirt")),
            200,
            "the no-recipe fallback the 200 keeps (every type)"
        );
        assert_eq!(
            FurnaceKind::Smoker.total_cook_time(wire("dirt")),
            200,
            "the smoker's no-recipe fallback the 200 keeps too"
        );
        // THE GETBURNDURATION WALK (the override's `iconst_2; idiv` —
        // the coal 1600/2 = 800, the lava bucket 20000/2 = 10000, the
        // bamboo 50/2 = 25; the furnace the base itself wears; the
        // integer division the odd-base floor keeps)
        let coal = mc_registry::generated::furnace_data::fuel_ticks(wire("coal"));
        assert_eq!(coal, 1600, "the coal's own base row keeps");
        assert_eq!(
            FurnaceKind::Furnace.burn_ticks(coal),
            1600,
            "the furnace the full duration wears"
        );
        assert_eq!(
            FurnaceKind::BlastFurnace.burn_ticks(coal),
            800,
            "the blast the half speaks"
        );
        assert_eq!(
            FurnaceKind::Smoker.burn_ticks(coal),
            800,
            "the smoker the half speaks too"
        );
        let lava = mc_registry::generated::furnace_data::fuel_ticks(wire("lava_bucket"));
        assert_eq!(
            FurnaceKind::BlastFurnace.burn_ticks(lava),
            10000,
            "the lava bucket the half keeps (20000/2)"
        );
        let bamboo = mc_registry::generated::furnace_data::fuel_ticks(wire("bamboo"));
        assert_eq!(
            FurnaceKind::Smoker.burn_ticks(bamboo),
            25,
            "the bamboo the half keeps (50/2)"
        );
        // THE LIT PAIRS (the world walk's own kinds the flip wears)
        assert_eq!(FurnaceKind::Furnace.lit_kind(), crate::world::FURNACE_LIT);
        assert_eq!(FurnaceKind::Furnace.unlit_kind(), crate::world::FURNACE);
        assert_eq!(
            FurnaceKind::BlastFurnace.lit_kind(),
            crate::world::BLAST_FURNACE_LIT
        );
        assert_eq!(
            FurnaceKind::BlastFurnace.unlit_kind(),
            crate::world::BLAST_FURNACE
        );
        assert_eq!(FurnaceKind::Smoker.lit_kind(), crate::world::SMOKER_LIT);
        assert_eq!(FurnaceKind::Smoker.unlit_kind(), crate::world::SMOKER);
    }

    /// THE BLAST FURNACE'S OWN TICK ARC (javap the same serverTick
    /// walk the BLASTING recipe type + the halved burn duration
    /// answer — the 800-tick coal, the 100-tick cook, the
    /// total-time re-fetch the completion keeps; the potato the
    /// blast table refuses the frozen walk wears). The SMOKER's own
    /// analog the food lane speaks.
    #[test]
    fn the_blast_the_hundred_tick_arc_the_half_coal_wears() {
        // THE COLD IGNITE (the blast furnace's own kind — the coal
        // the HALF burn banks: 1600/2 = 800)
        let mut sim = FurnaceSim::default();
        sim.kind = FurnaceKind::BlastFurnace;
        let mut slots = [(wire("iron_ore"), 5), (wire("coal"), 3), (0, 0)];
        let mut data = [0; 4];
        assert!(sim.tick(&mut slots, &mut data), "the ignite flips");
        assert_eq!(data[DATA_LIT_TIME], 800, "the coal the half banks");
        assert_eq!(data[DATA_LIT_DURATION], 800, "the total banks too");
        assert_eq!(slots[1], (wire("coal"), 2), "the coal the consume loses");
        assert_eq!(data[DATA_COOKING_PROGRESS], 1, "the same tick's cook speaks");
        // THE COOK TOTAL (the kind's own 100 the reset banked)
        assert_eq!(
            data[DATA_COOKING_TOTAL_TIME],
            100,
            "the blast's own 100 the reset speaks"
        );
        // THE HUNDRED-TICK ARC (98 more ticks climb to 99, the 100th
        // the burn lands — the ingot the copy takes, the input the
        // shrink loses, the total the re-fetch keeps)
        for _ in 0..98 {
            sim.tick(&mut slots, &mut data);
        }
        assert_eq!(data[DATA_COOKING_PROGRESS], 99, "the climb speaks");
        assert!(!sim.tick(&mut slots, &mut data), "the warm tick no flip");
        assert_eq!(data[DATA_COOKING_PROGRESS], 0, "the completion zeroes");
        assert_eq!(slots[2], (wire("iron_ingot"), 1), "the ingot the burn lands");
        assert_eq!(slots[0], (wire("iron_ore"), 4), "the input the shrink loses");
        assert_eq!(
            data[DATA_COOKING_TOTAL_TIME],
            100,
            "the re-fetch the same 100 keeps"
        );
        // THE TABLE'S OWN REFUSAL (the potato the BLASTING rows never
        // carry — the no-recipe arm the frozen walk wears, the lit
        // state the burn keeps, the potato NEVER cooks)
        let potato_slots = [(wire("potato"), 3), (wire("coal"), 3), (0, 0)];
        let mut potato_slots = potato_slots;
        let mut potato_data = [0; 4];
        let mut potato_sim = FurnaceSim::default();
        potato_sim.kind = FurnaceKind::BlastFurnace;
        assert!(
            !potato_sim.tick(&mut potato_slots, &mut potato_data),
            "no ignite — the no-recipe arm freezes"
        );
        assert_eq!(potato_data[DATA_LIT_TIME], 0, "the fuel never burns");
        assert_eq!(potato_slots[0], (wire("potato"), 3), "the potato uncooked");
        // THE SMOKER'S OWN ARC (the food lane — the same 800-tick
        // coal, the same 100-tick cook, the baked potato the SMOKING
        // row answers)
        let mut smoker = FurnaceSim::default();
        smoker.kind = FurnaceKind::Smoker;
        let mut food = [(wire("potato"), 2), (wire("coal"), 1), (0, 0)];
        let mut food_data = [0; 4];
        assert!(smoker.tick(&mut food, &mut food_data), "the smoker ignites");
        assert_eq!(food_data[DATA_LIT_TIME], 800, "the coal the half banks");
        assert_eq!(food_data[DATA_COOKING_TOTAL_TIME], 100, "the 100 speaks");
        for _ in 0..99 {
            smoker.tick(&mut food, &mut food_data);
        }
        assert_eq!(
            food[2],
            (wire("baked_potato"), 1),
            "the baked potato the burn lands"
        );
        assert_eq!(food[0], (wire("potato"), 1), "the potato the shrink loses");
    }

    /// THE FAST FAMILY'S OWN ANIMATE ARMS (javap BlastFurnaceBlock /
    /// SmokerBlock.animateTick over the 26.2 client.jar — the blast
    /// furnace the furnace's own walk the 9.0/16.0 height literal
    /// keeps, the smoker the ONE-draw walk the fixed +1.1 anchor
    /// speaks).
    #[test]
    fn the_blast_the_smoke_only_the_smoker_the_one_draw_wear() {
        // THE BLAST FURNACE'S OWN ARMS (the furnace's SAME lateral
        // and axis folds, the ONE literal the 9.0 replaces — the
        // height 1.0*9/16 = 0.5625 the furnace's 0.375 the taller
        // band answers)
        let v = blast_animate_tick(2, 0.05, 0.5, 1.0);
        assert!(v.crackle, "the 0.05 the gate passes");
        assert_eq!(v.dx, 0.52, "the east front +X the same stepX fold");
        assert_eq!(v.dz, 0.0, "the lateral the same d4 fold keeps");
        assert_eq!(v.dy, 9.0 / 16.0, "the 9.0/16.0 literal the taller band");
        let base = animate_tick(2, 0.05, 0.5, 1.0);
        assert_eq!(
            (v.dx, v.dz, v.crackle),
            (base.dx, base.dz, base.crackle),
            "the folds the furnace's own law keeps"
        );
        assert_ne!(v.dy, base.dy, "the height the ONE difference speaks");
        // THE WEST ARM (the same -0.52 the furnace's stepX keeps)
        let v = blast_animate_tick(4, 0.05, 0.25, 0.5);
        assert_eq!(v.dx, -0.52, "the west front −X");
        assert_eq!(v.dy, 0.5 * 9.0 / 16.0, "the taller band the west keeps");
        // THE SOUND GATE (the same 0.1 the furnace's boundary keeps)
        assert!(
            !blast_animate_tick(2, 0.1, 0.5, 1.0).crackle,
            "the 0.1 itself refuses"
        );
        // THE SMOKER'S OWN ONE-DRAW VERDICT (the offsets the no-opinion
        // zeros keep — the caller's fixed (x+0.5, y+1.1, z+0.5) anchor
        // the smoke above the block speaks; the gate the same 0.1)
        let v = smoker_animate_tick(0.05);
        assert!(v.crackle, "the 0.05 the gate passes");
        assert_eq!(v.dx, 0.0, "no lateral draw the smoker ever spends");
        assert_eq!(v.dy, 0.0, "no height draw the smoker ever spends");
        assert_eq!(v.dz, 0.0, "no facing read the smoker ever speaks");
        assert!(!smoker_animate_tick(0.1).crackle, "the 0.1 itself refuses");
        assert!(
            !smoker_animate_tick(0.95).crackle,
            "the high draw refuses — the smoke STILL spawns (the tail the gate never gates)"
        );
    }

    /// THE ANIMATE TICK'S OWN LAWS (javap FurnaceBlock.animateTick over
    /// the 26.2 jar — the sound gate, the two axis arms, the exact
    /// folds, the draw order the caller's three-draw spend keeps).
    #[test]
    fn the_animate_tick_the_gate_the_axis_arms_wear() {
        // THE SOUND GATE (pc 46-56: dcmpg 0.1, ifge 79 — the draw
        // BELOW 0.1 the playLocalSound arm speaks, the 0.1 itself the
        // `>=` arm keeps)
        assert!(animate_tick(2, 0.05, 0.5, 1.0).crackle, "the 0.05 the gate passes");
        assert!(!animate_tick(2, 0.1, 0.5, 1.0).crackle, "the 0.1 itself refuses");
        assert!(!animate_tick(2, 0.95, 0.5, 1.0).crackle, "the high draw refuses");
        assert!(
            animate_tick(2, 0.09999999999999995, 0.5, 1.0).crackle,
            "the hair below the gate passes"
        );
        // THE EAST ARM (pc 120-143: axis X, stepX +1 — d5 = +0.52,
        // d7 = d4 the lateral fold keeps)
        let v = animate_tick(2, 0.05, 0.5, 1.0);
        assert_eq!(v.dx, 0.52, "the east front +X the stepX fold speaks");
        assert_eq!(v.dz, 0.0, "the lateral 0.5*0.6-0.3 the d4 fold keeps");
        assert_eq!(v.dy, 0.375, "the height 1.0*6/16 the d6 fold keeps");
        // THE WEST ARM (stepX −1)
        let v = animate_tick(4, 0.05, 0.25, 0.5);
        assert_eq!(v.dx, -0.52, "the west front −X");
        assert_eq!(v.dz, 0.25 * 0.6 - 0.3, "the lateral the d4 fold wears");
        assert_eq!(v.dy, 0.5 * 6.0 / 16.0, "the height the d6 fold wears");
        // THE SOUTH ARM (pc 162-185: axis Z, stepZ +1 — d7 = +0.52,
        // d5 = d4 the lateral rides x)
        let v = animate_tick(3, 0.05, 1.0, 0.0);
        assert_eq!(v.dz, 0.52, "the south front +Z");
        assert_eq!(v.dx, 0.3, "the lateral 1.0*0.6-0.3 rides x");
        assert_eq!(v.dy, 0.0, "the height 0.0 the d6 fold keeps");
        // THE NORTH ARM (stepZ −1) + THE 0 NO-OPINION (the mesher's
        // own default the same north answers)
        for facing in [1u8, 0] {
            let v = animate_tick(facing, 0.05, 0.75, 0.25);
            assert_eq!(v.dz, -0.52, "the north front −Z (facing {facing})");
            assert_eq!(v.dx, 0.75 * 0.6 - 0.3, "the lateral rides x (facing {facing})");
        }
        // THE DRAWS THE GATE FAIL NEVER SKIPS (the caller's three-draw
        // spend the LIT kind alone gates — the verdict shape the
        // failing gate's own offsets keep: the folds speak regardless)
        let passed = animate_tick(2, 0.05, 0.5, 1.0);
        let failed = animate_tick(2, 0.9, 0.5, 1.0);
        assert_eq!(passed.dx, failed.dx, "the offsets the gate never gates");
        assert_eq!(passed.dz, failed.dz, "the lateral the gate never gates");
        assert_eq!(passed.dy, failed.dy, "the height the gate never gates");
    }

    /// THE SAMPLER'S OWN LAWS (javap ClientLevel.animateTick
    /// pcs 0-71 — the 667 iterations, the two arms' order, the fold
    /// the level stream's own draws keep, the determinism the seed
    /// replays).
    #[test]
    fn the_sampler_the_counts_the_order_the_fold_wears() {
        // THE COUNT + THE ORDER (the 667×2 samples the walk visits;
        // the FIRST visit the range-16 arm speaks, the SECOND the
        // range-32 — the exact fold the test's own stream mirrors:
        // the same LCG walked by hand answers visit-for-visit)
        let mut walked: Vec<(i32, i32, i32)> = Vec::new();
        animate_sample_walk(10, 20, 30, &mut crate::drops::Lcg::new(7), |x, y, z| {
            walked.push((x, y, z));
        });
        assert_eq!(walked.len(), 1334, "the 667×2 the loop banks");
        let mut mirror = crate::drops::Lcg::new(7);
        for i in 0..667 {
            for range in [16u64, 32] {
                let ex = 10 + mirror.next_int(range) - mirror.next_int(range);
                let ey = 20 + mirror.next_int(range) - mirror.next_int(range);
                let ez = 30 + mirror.next_int(range) - mirror.next_int(range);
                assert_eq!(
                    walked[i * 2 + if range == 16 { 0 } else { 1 }],
                    (ex, ey, ez),
                    "the visit the stream's own fold computes (iteration {i}, range {range})"
                );
            }
        }
        // THE RANGES' OWN SPREAD (the 16 arm never walks past ±15 —
        // the |Δ| bound every odd-positioned visit keeps; the 32 arm
        // the mirror walk above already answers visit-for-visit)
        for (x, y, z) in walked.iter().step_by(2) {
            assert!((x - 10).abs() <= 15 && (y - 20).abs() <= 15 && (z - 30).abs() <= 15);
        }
        // THE DETERMINISM (the same seed the same 1334 walks)
        let mut again: Vec<(i32, i32, i32)> = Vec::new();
        animate_sample_walk(10, 20, 30, &mut crate::drops::Lcg::new(7), |x, y, z| {
            again.push((x, y, z));
        });
        assert_eq!(walked, again, "the seed the walk replays");
    }
}

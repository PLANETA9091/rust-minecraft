//! THE FOOD LANE — the vanilla `FoodData`'s own arithmetic (javap
//! net.minecraft.world.food.FoodData + FoodConstants, the cached jar's
//! Mojang-mapped classes), the survival loop's hunger half: the
//! exhaustion bank folds at 4.0 into the saturation first and the food
//! bar second, the DOUBLE-LANE regeneration heals off a full bar (the
//! saturated fast lane every 10 ticks, the classic slow lane every 80),
//! and the starving bar bites at the difficulty's own gates.
//!
//! The tick's consumers: the OFFLINE world's stand-in server (the
//! networked lane's FoodData is the server's own — the wire's Set
//! Health speaks its truth, the client folds nothing there). The
//! exhaustion sources speak the server-side `causeFoodExhaustion`'s
//! own `!level.isClientSide` gate: the jump (0.05 / the sprint's 0.2),
//! the sprint move (0.1 per block over the cm law), the mine (0.005
//! per block), the attack (0.1), the damage source's own field (the
//! fall/starve types' data verdict 0.0), the swim (0.01 per block).
//! The walk and the crouch speak the modern law's honest 0.0.

/// `FoodConstants.MAX_FOOD` — the bar's own width.
pub const MAX_FOOD: i32 = 20;
/// `FoodConstants.START_SATURATION` — the constructor's own 5.0 (the
/// FoodData() the spawn speaks).
pub const START_SATURATION: f32 = 5.0;
/// `FoodConstants.EXHAUSTION_DROP` — the fold threshold (javap tick:
/// `exhaustionLevel > 4.0f` banks 4.0 per fold).
pub const EXHAUSTION_DROP: f32 = 4.0;
/// `addExhaustion`'s own ceiling (javap: `min(exhaustionLevel + f,
/// 40.0f)`).
pub const MAX_EXHAUSTION: f32 = 40.0;
/// `FoodConstants.HEALTH_TICK_COUNT` — the slow regen lane's 80-tick
/// beat.
pub const HEALTH_TICK_COUNT: i32 = 80;
/// `FoodConstants.HEALTH_TICK_COUNT_SATURATED` — the fast lane's
/// 10-tick beat.
pub const HEALTH_TICK_COUNT_SATURATED: i32 = 10;
/// `FoodConstants.HEAL_LEVEL` — the slow lane's food floor (18).
pub const HEAL_LEVEL: i32 = 18;
/// `FoodConstants.SPRINT_LEVEL` — `hasEnoughFood`'s own bar (`food >
/// 6`, the exhaustive manoeuvres gate the sprint speaks).
pub const SPRINT_LEVEL: i32 = 6;
/// `FoodConstants.EXHAUSTION_HEAL` — the slow lane's heal cost (6.0
/// per heart).
pub const EXHAUSTION_HEAL: f32 = 6.0;
/// `FoodConstants.EXHAUSTION_JUMP` — the plain jump's cost.
pub const EXHAUSTION_JUMP: f32 = 0.05;
/// `FoodConstants.EXHAUSTION_SPRINT_JUMP` — the sprint jump's cost.
pub const EXHAUSTION_SPRINT_JUMP: f32 = 0.2;
/// `FoodConstants.EXHAUSTION_MINE` — Block.playerDestroy's own inlined
/// 0.005 (the awardStat(BLOCK_MINED) tail the bytecode speaks).
pub const EXHAUSTION_MINE: f32 = 0.005;
/// `FoodConstants.EXHAUSTION_ATTACK` — Player.attack's own inlined 0.1
/// (the damageStatsAndHearts tail, both attack variants).
#[expect(dead_code)] // the 26.2 FoodConstants rank — the exhaustion/saturation ledger keeps
pub const EXHAUSTION_ATTACK: f32 = 0.1;
/// `FoodConstants.EXHAUSTION_WALK` — the modern law's honest 0.0.
#[expect(dead_code)] // the 26.2 FoodConstants rank — the exhaustion/saturation ledger keeps
pub const EXHAUSTION_WALK: f32 = 0.0;
/// `FoodConstants.EXHAUSTION_CROUCH` — the modern law's honest 0.0.
#[expect(dead_code)] // the 26.2 FoodConstants rank — the exhaustion/saturation ledger keeps
pub const EXHAUSTION_CROUCH: f32 = 0.0;
/// `FoodConstants.EXHAUSTION_SPRINT` — the on-ground sprint lane's
/// rate per block (0.1 * cm * 0.01 over the stats' own cm walk).
pub const EXHAUSTION_SPRINT: f32 = 0.1;
/// `FoodConstants.EXHAUSTION_SWIM` — the in-water lanes' rate per
/// block (the swim, the walk-on-water, the walk-under-water — all
/// 0.01 * cm * 0.01).
pub const EXHAUSTION_SWIM: f32 = 0.01;
/// The `damage_type/fall.json`'s own `exhaustion` field — the data's
/// verdict the hurt's food cost speaks (the classic wiki's 0.1 is
/// gone; the modern data says 0.0).
pub const FALL_DAMAGE_EXHAUSTION: f32 = 0.0;
/// The `damage_type/starve.json`'s own `exhaustion` field (0.0).
#[expect(dead_code)] // the 26.2 FoodConstants rank — the exhaustion/saturation ledger keeps
pub const STARVE_DAMAGE_EXHAUSTION: f32 = 0.0;
/// The starve bite's own damage (javap tick: `hurtServer(level,
/// damageSources.starve(), 1.0f)`).
pub const STARVE_DAMAGE: f32 = 1.0;

/// The saturation modifier table (`FoodConstants`' own ranks — the
/// Foods' builder rows speak them per item).
#[expect(dead_code)] // the 26.2 FoodConstants rank — the exhaustion/saturation ledger keeps
pub const FOOD_SATURATION_POOR: f32 = 0.1;
#[expect(dead_code)] // the 26.2 FoodConstants rank — the exhaustion/saturation ledger keeps
pub const FOOD_SATURATION_LOW: f32 = 0.3;
#[cfg_attr(not(test), expect(dead_code))]
pub const FOOD_SATURATION_NORMAL: f32 = 0.6;
#[cfg_attr(not(test), expect(dead_code))]
pub const FOOD_SATURATION_GOOD: f32 = 0.8;
#[expect(dead_code)] // the 26.2 FoodConstants rank — the exhaustion/saturation ledger keeps
pub const FOOD_SATURATION_MAX: f32 = 1.0;
#[expect(dead_code)] // the 26.2 FoodConstants rank — the exhaustion/saturation ledger keeps
pub const FOOD_SATURATION_SUPERNATURAL: f32 = 1.2;

/// `FoodConstants.saturationByModifier` (javap: `food * modifier *
/// 2.0` — the eat lane's own saturation math; the cake bite's own
/// walk the autopush-524 round the (int, float) overload serves).
pub fn saturation_by_modifier(nutrition: i32, modifier: f32) -> f32 {
    nutrition as f32 * modifier * 2.0
}

/// `Level.getDifficulty`'s own vocabulary (the tick's difficulty gates:
/// the food drop refuses on PEACEFUL, the starve bites through HARD).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    /// THE DIAL'S OWN VOCABULARY MAP (the level's
    /// `crate::difficulty::Difficulty` the FoodData tick's ctx wears —
    /// the same four arms the ids keep, the WorldOptions dial the
    /// OFFLINE lane's own server speaks): the world's field the tick's
    /// gates read — the peaceful bar the drop refuses, the hard bite
    /// the last-heart gate ignores.
    pub fn of(difficulty: crate::difficulty::Difficulty) -> Difficulty {
        match difficulty {
            crate::difficulty::Difficulty::Peaceful => Difficulty::Peaceful,
            crate::difficulty::Difficulty::Easy => Difficulty::Easy,
            crate::difficulty::Difficulty::Normal => Difficulty::Normal,
            crate::difficulty::Difficulty::Hard => Difficulty::Hard,
        }
    }
}

/// One `FoodData.tick`'s outcome — the events the server's player
/// folds: the regen lanes' heal (the Entity.heal law: only a living
/// body heals, the clamp rides setHealth) and the starve lane's bite
/// (the hurtServer(starve, 1.0) full pipeline).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickEvents {
    pub heal: f32,
    pub starve: bool,
}

/// The tick's player-side context — the gates the bytecode reads:
/// `ServerPlayer.isHurt` (the Player's own override:
/// `health > 0 && health < maxHealth`), the health float the starve
/// gate compares, the level's difficulty, the
/// `GameRules.NATURAL_HEALTH_REGENERATION` flag.
pub struct TickContext {
    pub health: f32,
    pub max_health: f32,
    pub difficulty: Difficulty,
    pub natural_regen: bool,
}

impl TickContext {
    /// `Player.isHurt` (javap, the Player's own override LivingEntity
    /// never speaks): `getHealth() > 0.0F && getHealth() <
    /// getMaxHealth()`.
    pub fn is_hurt(&self) -> bool {
        self.health > 0.0 && self.health < self.max_health
    }
}

/// The vanilla `FoodData` (javap, the cached jar's own fields): the
/// food bar 0..=20, the saturation capped at the bar, the exhaustion
/// bank capped at 40, and the shared regen/starve tick timer.
#[derive(Debug, Clone, PartialEq)]
pub struct FoodData {
    food_level: i32,
    saturation: f32,
    exhaustion: f32,
    tick_timer: i32,
}

impl Default for FoodData {
    fn default() -> Self {
        Self::new()
    }
}

impl FoodData {
    /// The constructor's own spawn state (javap FoodData(): `foodLevel
    /// = 20; saturationLevel = 5.0f` — the exhaustion and the timer
    /// ride the JVM's zero init).
    pub fn new() -> Self {
        Self {
            food_level: MAX_FOOD,
            saturation: START_SATURATION,
            exhaustion: 0.0,
            tick_timer: 0,
        }
    }

    pub fn food_level(&self) -> i32 {
        self.food_level
    }

    pub fn saturation(&self) -> f32 {
        self.saturation
    }

    pub fn exhaustion(&self) -> f32 {
        self.exhaustion
    }

    /// The shared regen/starve beat's own read (the file's foodTickTimer
    /// row the reload re-wears — the lane switch's own count the restart
    /// keeps).
    pub fn tick_timer(&self) -> i32 {
        self.tick_timer
    }

    /// THE SAVED ROW'S OWN CONSTRUCTOR (the level.dat read the reload
    /// speaks — the four fields the player tag's food rows store
    /// verbatim, the save-time state's own clamps the file's row
    /// already wore, the constructor the honest restore answers).
    pub fn from_saved(food_level: i32, saturation: f32, exhaustion: f32, tick_timer: i32) -> Self {
        Self {
            food_level,
            saturation,
            exhaustion,
            tick_timer,
        }
    }

    /// `hasEnoughFood` (javap: `foodLevel > 6.0f` as a float compare —
    /// the exhaustive manoeuvres gate the sprint speaks).
    pub fn has_enough_food(&self) -> bool {
        (self.food_level as f32) > SPRINT_LEVEL as f32
    }

    /// `needsFood` (javap: `foodLevel < 20`).
    pub fn needs_food(&self) -> bool {
        self.food_level < MAX_FOOD
    }

    /// The private `add(int, float)` (javap): the bar clamps 0..=20,
    /// the saturation clamps to the BAR (not a constant — `foodLevel`
    /// i2f is the ceiling).
    fn add(&mut self, food: i32, saturation: f32) {
        self.food_level = (self.food_level + food).clamp(0, MAX_FOOD);
        self.saturation = (self.saturation + saturation).clamp(0.0, self.food_level as f32);
    }

    /// `eat(int, float)` (javap: `add(food, saturationByModifier(food,
    /// modifier))` — the eat arc's own entry; the cake bite's own
    /// walk the autopush-524 round serves).
    pub fn eat(&mut self, nutrition: i32, modifier: f32) {
        self.add(nutrition, saturation_by_modifier(nutrition, modifier));
    }

    /// `eat(FoodProperties)` (javap: `add(props.nutrition(),
    /// props.saturation())` — the COMPONENT lane's own fold: the 26.2
    /// record carries the ABSOLUTE saturation, the legacy
    /// (int, float) overload's modifier*2 law is the old API's own).
    pub fn eat_props(&mut self, nutrition: i32, saturation: f32) {
        self.add(nutrition, saturation);
    }

    /// `addExhaustion` (javap: `min(exhaustionLevel + f, 40.0f)`).
    pub fn add_exhaustion(&mut self, amount: f32) {
        self.exhaustion = (self.exhaustion + amount).min(MAX_EXHAUSTION);
    }

    /// `setFoodLevel` / `setSaturation` — the wire's own landing
    /// (handleSetHealth speaks the server's truth straight in; the
    /// Set Health packet carries no exhaustion, the bank keeps its
    /// own).
    pub fn set_food_level(&mut self, food: i32) {
        self.food_level = food;
    }

    pub fn set_saturation(&mut self, saturation: f32) {
        self.saturation = saturation;
    }

    /// THE TICK (javap `FoodData.tick(ServerPlayer)` verbatim, the
    /// ServerPlayer.tick's own head position — the movement's
    /// exhaustion sources land AFTER this fold, banking for the next):
    ///
    /// 1. the EXHAUSTION FOLD: over 4.0 banks 4.0; the saturation
    ///    drains 1.0 first, the bar drops 1 second (the peaceful level
    ///    refuses the bar's own drop);
    /// 2. the FAST REGEN lane: natural regen, saturation > 0, hurt,
    ///    and a FULL bar (>= 20 — the modern law's own floor, not the
    ///    wiki's 18) beat every 10 ticks: heal min(sat, 6) / 6, pay
    ///    min(sat, 6) exhaustion;
    /// 3. the SLOW REGEN lane: natural regen, hurt, bar >= 18 — the
    ///    every-80-ticks heart costs 6.0 exhaustion (the timer is
    ///    SHARED between the lanes — a lane switch keeps its count);
    /// 4. the STARVE lane: an empty bar's 80-tick beat bites 1.0
    ///    through the difficulty's own gates (over 10 health, HARD's
    ///    always, NORMAL down to the last heart) and resets;
    /// 5. the else tail resets the timer when no lane fires.
    pub fn tick(&mut self, ctx: &TickContext) -> TickEvents {
        let mut events = TickEvents {
            heal: 0.0,
            starve: false,
        };
        // 1. THE EXHAUSTION FOLD (`exhaustionLevel > 4.0f` — strictly
        //    over, the fcmpl's own shape)
        if self.exhaustion > EXHAUSTION_DROP {
            self.exhaustion -= EXHAUSTION_DROP;
            if self.saturation > 0.0 {
                self.saturation = (self.saturation - 1.0).max(0.0);
            } else if ctx.difficulty != Difficulty::Peaceful {
                self.food_level = (self.food_level - 1).max(0);
            }
        }
        // 2. THE FAST REGEN LANE (the saturated body's own beat)
        if ctx.natural_regen
            && self.saturation > 0.0
            && ctx.is_hurt()
            && self.food_level >= MAX_FOOD
        {
            self.tick_timer += 1;
            if self.tick_timer >= HEALTH_TICK_COUNT_SATURATED {
                let take = self.saturation.min(6.0);
                events.heal = take / 6.0;
                self.add_exhaustion(take);
                self.tick_timer = 0;
            }
        }
        // 3. THE SLOW REGEN LANE (the classic 18+ beat, the 6.0 cost)
        else if ctx.natural_regen && self.food_level >= HEAL_LEVEL && ctx.is_hurt() {
            self.tick_timer += 1;
            if self.tick_timer >= HEALTH_TICK_COUNT {
                events.heal = 1.0;
                self.add_exhaustion(EXHAUSTION_HEAL);
                self.tick_timer = 0;
            }
        }
        // 4. THE STARVE LANE (the empty bar's own bite)
        else if self.food_level <= 0 {
            self.tick_timer += 1;
            if self.tick_timer >= HEALTH_TICK_COUNT {
                events.starve = ctx.health > 10.0
                    || ctx.difficulty == Difficulty::Hard
                    || (ctx.health > 1.0 && ctx.difficulty == Difficulty::Normal);
                self.tick_timer = 0;
            }
        }
        // 5. THE ELSE TAIL (the fed body's own reset)
        else {
            self.tick_timer = 0;
        }
        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(health: f32) -> TickContext {
        TickContext {
            health,
            max_health: 20.0,
            difficulty: Difficulty::Normal,
            natural_regen: true,
        }
    }

    /// THE SPAWN STATE: the constructor's own 20 / 5.0 / 0 / 0.
    #[test]
    fn the_spawn_state_speaks_the_constructor() {
        let f = FoodData::new();
        assert_eq!(f.food_level(), 20);
        assert_eq!(f.saturation(), 5.0);
        assert_eq!(f.exhaustion(), 0.0);
        assert!(!f.needs_food(), "the full bar never needs food");
        assert!(f.has_enough_food(), "the spawn bar sprints");
    }

    /// THE ADD LAW: the bar clamps 0..=20, the saturation's ceiling is
    /// the BAR itself (not a constant — the i2f's own verdict), the
    /// floor is zero.
    #[test]
    fn the_add_clamps_the_bar_and_the_saturation_to_it() {
        let mut f = FoodData::new();
        f.set_food_level(20);
        f.set_saturation(10.0);
        f.add(5, 0.0);
        assert_eq!(f.food_level(), 20, "the bar's own ceiling");
        f.set_food_level(4);
        f.set_saturation(4.0);
        f.add(0, 10.0);
        assert_eq!(f.saturation(), 4.0, "the saturation caps at the bar");
        f.set_food_level(2);
        f.set_saturation(1.0);
        f.add(-5, -10.0);
        assert_eq!(f.food_level(), 0, "the bar's own floor");
        assert_eq!(f.saturation(), 0.0, "the saturation's own floor");
    }

    /// THE EAT LAW: `add(food, food * modifier * 2)` — the
    /// saturationByModifier's own fold (the bread's 5 @ 0.6 speaks 6.0
    /// saturation — the f32's own rounding the JVM and the IEEE mul
    /// share). The bar clamps at 20; the saturation rides to its
    /// ≈18.8 sum under the bar's own cap.
    #[test]
    fn the_eat_folds_the_saturation_modifier() {
        assert!((saturation_by_modifier(5, FOOD_SATURATION_NORMAL) - 6.0).abs() < 1e-4);
        let mut f = FoodData::new();
        f.set_food_level(10);
        f.set_saturation(0.0);
        f.eat(5, FOOD_SATURATION_NORMAL);
        assert_eq!(f.food_level(), 15);
        assert!((f.saturation() - 6.0).abs() < 1e-4);
        // the steak's 8 @ 0.8: the bar 15 + 8 clamps at 20, the
        // saturation 6.0 + 12.8 rides under the bar's own cap
        f.eat(8, FOOD_SATURATION_GOOD);
        assert_eq!(f.food_level(), 20, "the bar's own clamp");
        assert!((f.saturation() - 18.8).abs() < 1e-3, "the sum under the bar: {}", f.saturation());
    }

    /// THE COMPONENT EAT LAW (javap `eat(FoodProperties)`:
    /// `add(nutrition, saturation)` — the ABSOLUTE saturation the 26.2
    /// record carries; the apple's 4 @ 2.4 lands 2.4, NOT the legacy
    /// overload's 4 * 2.4 * 2 fold).
    #[test]
    fn the_component_eat_folds_the_absolute_saturation() {
        let mut f = FoodData::new();
        f.set_food_level(10);
        f.set_saturation(0.0);
        f.eat_props(4, 2.4);
        assert_eq!(f.food_level(), 14);
        assert!((f.saturation() - 2.4).abs() < 1e-4, "the absolute value lands");
        // the legacy overload would have spoken 19.2 clamped to the
        // BAR (14) — the split's own verdict the probe's rows pinned
        let mut g = FoodData::new();
        g.set_food_level(10);
        g.set_saturation(0.0);
        g.eat(4, 2.4);
        assert_eq!(g.food_level(), 14);
        assert!((g.saturation() - 14.0).abs() < 1e-4, "the legacy fold rides to the bar's cap");
    }

    /// THE EXHAUSTION BANK: `min(banked + f, 40.0)` — the ceiling the
    /// javap speaks.
    #[test]
    fn the_exhaustion_bank_caps_at_forty() {
        let mut f = FoodData::new();
        f.add_exhaustion(25.0);
        f.add_exhaustion(25.0);
        assert_eq!(f.exhaustion(), 40.0, "the bank's own ceiling");
    }

    /// THE FOLD: over 4.0 banks 4.0 and drains the saturation 1.0 —
    /// the saturation's own drain first.
    #[test]
    fn the_fold_drains_the_saturation_first() {
        let mut f = FoodData::new();
        f.add_exhaustion(4.5);
        let events = f.tick(&ctx(20.0));
        assert_eq!(f.exhaustion(), 0.5, "the 4.0 banked");
        assert_eq!(f.saturation(), 4.0, "the saturation's own 1.0 drop");
        assert_eq!(events, TickEvents { heal: 0.0, starve: false });
        // the exactly-4.0 bank never folds (the fcmpl's strictly-over
        // gate)
        let mut f = FoodData::new();
        f.add_exhaustion(4.0);
        f.tick(&ctx(20.0));
        assert_eq!(f.exhaustion(), 4.0, "the 4.0 rides until over");
    }

    /// THE FOLD'S SECOND HALF: the saturation empty, the BAR drops 1
    /// per fold — the peaceful level refuses.
    #[test]
    fn the_fold_drops_the_bar_when_the_saturation_is_gone() {
        let mut f = FoodData::new();
        f.set_saturation(0.0);
        f.add_exhaustion(8.5);
        f.tick(&ctx(20.0));
        assert_eq!(f.food_level(), 19, "the bar's own 1-per-fold drop");
        assert_eq!(f.exhaustion(), 4.5, "one fold's 4.0 banked");
        // the peaceful level's own refusal
        let mut f = FoodData::new();
        f.set_saturation(0.0);
        f.add_exhaustion(5.0);
        f.tick(&TickContext {
            health: 20.0,
            max_health: 20.0,
            difficulty: Difficulty::Peaceful,
            natural_regen: true,
        });
        assert_eq!(f.food_level(), 20, "the peaceful level never starves");
    }

    /// THE FAST REGEN LANE: the full bar + saturation + hurt beats
    /// every 10 ticks — heal min(sat, 6) / 6, pay min(sat, 6)
    /// exhaustion.
    #[test]
    fn the_fast_lane_heals_off_the_saturation_every_ten_ticks() {
        let mut f = FoodData::new();
        f.set_saturation(6.0);
        // nine silent beats
        for _ in 0..9 {
            let events = f.tick(&ctx(6.0));
            assert_eq!(events.heal, 0.0);
        }
        assert_eq!(f.tick_timer, 9, "the timer's own walk");
        // the tenth tick speaks
        let events = f.tick(&ctx(6.0));
        assert_eq!(events.heal, 1.0, "min(6, 6) / 6");
        assert_eq!(f.exhaustion(), 6.0, "the heal's own 6.0 cost");
        assert_eq!(f.tick_timer, 0, "the beat's own reset");
    }

    /// THE FAST LANE'S CEILING: the saturation over 6 heals at the 6.0
    /// cap's own rate and pays 6.0.
    #[test]
    fn the_fast_lane_caps_the_saturation_take_at_six() {
        let mut f = FoodData::new();
        f.set_saturation(10.0);
        for _ in 0..10 {
            f.tick(&ctx(6.0));
        }
        assert_eq!(f.exhaustion(), 6.0, "the capped take");
        // the saturation dropped only by the fold lane's absence — the
        // regen pays EXHAUSTION, not saturation
        assert_eq!(f.saturation(), 10.0);
    }

    /// THE SLOW LANE: the bar under 20 (no saturation's fast lane) but
    /// over 18 beats every 80 ticks — 1.0 heart, 6.0 exhaustion.
    #[test]
    fn the_slow_lane_heals_a_heart_every_eighty_ticks() {
        let mut f = FoodData::new();
        f.set_food_level(19);
        f.set_saturation(0.0);
        for _ in 0..79 {
            let events = f.tick(&ctx(6.0));
            assert_eq!(events.heal, 0.0);
        }
        let events = f.tick(&ctx(6.0));
        assert_eq!(events.heal, 1.0, "the slow lane's own heart");
        assert_eq!(f.exhaustion(), 6.0);
    }

    /// THE LANE SWITCH KEEPS THE TIMER: a body banking 5 ticks in the
    /// fast lane (the saturation drains mid-way) continues the count
    /// in the slow lane — the SHARED timer the bytecode speaks.
    #[test]
    fn the_two_lanes_share_one_timer() {
        let mut f = FoodData::new();
        f.set_food_level(20);
        f.set_saturation(1.0);
        // five fast-lane ticks (saturation > 0, full bar)
        for _ in 0..5 {
            f.tick(&ctx(6.0));
        }
        // the wire drops the bar to 19 — the slow lane takes over
        f.set_food_level(19);
        f.set_saturation(0.0);
        for _ in 0..74 {
            f.tick(&ctx(6.0));
        }
        // the 80th TOTAL beat speaks (5 + 75)
        let events = f.tick(&ctx(6.0));
        assert_eq!(events.heal, 1.0, "the shared timer's own 80");
    }

    /// THE FAST LANE'S OWN FLOOR: the bar under 20 never rides it —
    /// the modern law's own 18/20 split (the wiki's old 18-everything
    /// never speaks here).
    #[test]
    fn the_fast_lane_demands_the_full_bar() {
        let mut f = FoodData::new();
        f.set_food_level(19);
        f.set_saturation(5.0);
        let events = f.tick(&ctx(6.0));
        assert_eq!(events.heal, 0.0, "the 19 bar rides the slow lane");
        assert_eq!(f.tick_timer, 1, "the slow lane's own count");
    }

    /// THE REGEN GATES: the fed body (no hurt) and the dead body never
    /// heal — the isHurt's own shape (health > 0 && health < max).
    #[test]
    fn the_regen_needs_a_living_hurt_body() {
        let mut f = FoodData::new();
        f.set_saturation(6.0);
        // the healthy body: no lane, the timer resets
        let events = f.tick(&ctx(20.0));
        assert_eq!(events.heal, 0.0);
        assert_eq!(f.tick_timer, 0, "the else tail's own reset");
        // the dead body: the isHurt gate refuses
        let events = f.tick(&ctx(0.0));
        assert_eq!(events.heal, 0.0);
    }

    /// THE NATURAL REGEN RULE'S OWN GATE: the gamerule off, no lane
    /// fires (the timer walks only in the starve lane's shape — the
    /// fed body's else tail resets).
    #[test]
    fn the_natural_regen_gate_silences_the_lanes() {
        let mut f = FoodData::new();
        f.set_food_level(19);
        f.set_saturation(0.0);
        let events = f.tick(&TickContext {
            health: 6.0,
            max_health: 20.0,
            difficulty: Difficulty::Normal,
            natural_regen: false,
        });
        assert_eq!(events.heal, 0.0, "the gamerule's own refusal");
        assert_eq!(f.tick_timer, 0, "no lane fired");
    }

    /// THE STARVE LANE: the empty bar's 80-tick beat bites 1.0 through
    /// the difficulty's gates — NORMAL stops at the last heart.
    #[test]
    fn the_starve_lane_bites_on_its_own_beat() {
        let mut f = FoodData::new();
        f.set_food_level(0);
        f.set_saturation(0.0);
        for _ in 0..79 {
            let events = f.tick(&ctx(20.0));
            assert!(!events.starve);
        }
        let events = f.tick(&ctx(20.0));
        assert!(events.starve, "the 80th tick bites");
        // NORMAL's floor: health 1.0 never bites (the fcmpl's own
        // gate — health > 1.0 required; the probe rides a fresh beat)
        f.tick_timer = 79;
        let events = f.tick(&ctx(1.0));
        assert!(!events.starve, "the last heart is safe on NORMAL");
        // over 10 health always bites
        f.tick_timer = 79;
        let events = f.tick(&ctx(11.0));
        assert!(events.starve);
        // the bar refed, the starve lane leaves — the fed healthy body
        // rides the else tail's own silence
        f.set_food_level(20);
        let events = f.tick(&ctx(20.0));
        assert_eq!(
            events,
            TickEvents { heal: 0.0, starve: false },
            "the fed body's own silence"
        );
    }

    /// THE STARVE GATES PER DIFFICULTY: HARD always bites, EASY never
    /// under 10 health's own lane (the easy body only bites over 10 —
    /// the health > 10 lane), PEACEFUL rides the > 10 lane only.
    #[test]
    fn the_starve_gates_speak_the_difficulty() {
        let mut f = FoodData::new();
        f.set_food_level(0);
        f.set_saturation(0.0);
        for _ in 0..80 {
            f.tick(&ctx(5.0));
        }
        // HARD: health 0.5 still bites (the always lane)
        f.tick_timer = 79;
        let events = f.tick(&TickContext {
            health: 0.5,
            max_health: 20.0,
            difficulty: Difficulty::Hard,
            natural_regen: true,
        });
        assert!(events.starve, "HARD always bites");
        // EASY at health 5: only the > 10 lane could bite — refused
        f.tick_timer = 79;
        let events = f.tick(&TickContext {
            health: 5.0,
            max_health: 20.0,
            difficulty: Difficulty::Easy,
            natural_regen: true,
        });
        assert!(!events.starve, "EASY spares under 10");
        // EASY over 10: bites (the shared > 10 lane)
        f.tick_timer = 79;
        let events = f.tick(&TickContext {
            health: 11.0,
            max_health: 20.0,
            difficulty: Difficulty::Easy,
            natural_regen: true,
        });
        assert!(events.starve);
    }

    /// THE WIRE'S LANDING: set_food_level / set_saturation speak the
    /// handleSetHealth's own assignments (the bank's exhaustion keeps
    /// its own — the packet carries none).
    #[test]
    fn the_wire_lands_food_and_saturation_verbatim() {
        let mut f = FoodData::new();
        f.add_exhaustion(3.0);
        f.set_food_level(14);
        f.set_saturation(2.5);
        assert_eq!(f.food_level(), 14);
        assert_eq!(f.saturation(), 2.5);
        assert_eq!(f.exhaustion(), 3.0, "the bank keeps its own");
    }
}

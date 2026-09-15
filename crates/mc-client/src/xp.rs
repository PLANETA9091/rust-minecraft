//! THE EXPERIENCE STAT (javap `Player` over the 26.2 jar — the XP
//! quartet the HUD's contextual bar and the grant lane speak):
//! `experienceLevel` the int, `experienceProgress` the f32 fraction,
//! `totalExperience` the clamped sum. The stat is the OFFLINE lane's
//! stand-in server voice — the NETWORKED lane's truth rides the wire's
//! Set Experience (the future's own parse).

use crate::drops::Lcg;
use crate::world;

/// THE LEVEL CURVE (javap `Player.getXpNeededForNextLevel` — the
/// 26.2 jar's own three arms; the early arm is `7 + level*2`, the
/// drifted curve the current bytecode speaks):
/// `level >= 30` → `112 + (level-30)*9`,
/// `level >= 16` → `37 + (level-15)*5`,
/// else          → `7 + level*2`.
pub fn xp_needed_for_next_level(level: i32) -> i32 {
    if level >= 30 {
        112 + (level - 30) * 9
    } else if level >= 16 {
        37 + (level - 15) * 5
    } else {
        7 + level * 2
    }
}

/// THE ORE'S OWN XP ROLL (javap `Blocks`' static init — the
/// `DropExperienceBlock` constructions speak `UniformInt.of(min, max)`;
/// the sample rides the loot lane's LCG). The blocks the palette speaks
/// that grant NOTHING (copper, iron, gold — the smelting lane's own
/// reward) burn no roll, exactly like the vanilla's own gate: a
/// non-`DropExperienceBlock` never calls `tryDropExperience`.
pub fn ore_xp(kind: u16, rng: &mut Lcg) -> i32 {
    let (min, max) = match kind {
        world::COAL_ORE => (0, 2),
        world::REDSTONE_ORE => (1, 5),
        world::LAPIS_ORE => (2, 5),
        world::DIAMOND_ORE => (3, 7),
        world::EMERALD_ORE => (3, 7),
        _ => return 0,
    };
    rng.next_int_inclusive(min, max)
}

/// THE STAT (the Player's own three fields; the state machine the
/// giveExperiencePoints bytecode folds verbatim — the f32 math the
/// d2f/f2d round-trips speak, the saturated adds, the borrow loops).
#[derive(Debug, Clone, PartialEq)]
pub struct ExperienceStat {
    pub level: i32,
    pub progress: f32,
    pub total: i32,
}

impl Default for ExperienceStat {
    fn default() -> Self {
        Self::new()
    }
}

impl ExperienceStat {
    pub fn new() -> Self {
        Self {
            level: 0,
            progress: 0.0,
            total: 0,
        }
    }

    /// `giveExperienceLevels`' own fold (javap): the saturated add,
    /// then the negative fall — `level < 0` resets ALL THREE (the
    /// level, the progress, the total) to zero, the enchantment
    /// bankruptcy's own verdict.
    pub fn give_levels(&mut self, n: i32) {
        self.level = self.level.saturating_add(n);
        if self.level < 0 {
            self.level = 0;
            self.progress = 0.0;
            self.total = 0;
        }
    }

    /// `giveExperiencePoints`' own law (javap, the three loops):
    /// `progress += amount / needed` (the f32 divide the i2f/f2d
    /// speaks), the total rides the saturated clamp [0, i32::MAX],
    /// then the NEGATIVE loop borrows a level per iteration (`1.0 +
    /// f/needed` the refill speaks, the level-0 floor flat-zeroes),
    /// then the POSITIVE loop spends a level per pass (`(progress-1)
    /// * needed` on the OLD curve, `/ needed` on the NEW — the
    /// giveExperienceLevels call sits BETWEEN the two, the bytecode's
    /// own order).
    pub fn give_points(&mut self, amount: i32) -> &mut Self {
        self.progress += amount as f32 / xp_needed_for_next_level(self.level) as f32;
        self.total = self.total.saturating_add(amount);
        // THE NEGATIVE LOOP (the borrow the bytecode loops to 39)
        while self.progress < 0.0 {
            let f = self.progress * xp_needed_for_next_level(self.level) as f32;
            if self.level > 0 {
                self.give_levels(-1);
                self.progress = 1.0 + f / xp_needed_for_next_level(self.level) as f32;
            } else {
                self.give_levels(-1);
                self.progress = 0.0;
            }
        }
        // THE POSITIVE LOOP (the spend the bytecode loops to 100)
        while self.progress >= 1.0 {
            self.progress = (self.progress - 1.0) * xp_needed_for_next_level(self.level) as f32;
            self.give_levels(1);
            self.progress /= xp_needed_for_next_level(self.level) as f32;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_curve_folds_the_javap_arms() {
        // the early arm `7 + level*2` (the 26.2 drift the old
        // `2 + level*7` shape never speaks)
        assert_eq!(xp_needed_for_next_level(0), 7);
        assert_eq!(xp_needed_for_next_level(1), 9);
        assert_eq!(xp_needed_for_next_level(15), 37);
        // the mid arm `37 + (level-15)*5`
        assert_eq!(xp_needed_for_next_level(16), 42);
        assert_eq!(xp_needed_for_next_level(30), 112);
        // the late arm `112 + (level-30)*9`
        assert_eq!(xp_needed_for_next_level(31), 121);
        assert_eq!(xp_needed_for_next_level(40), 202);
    }

    #[test]
    fn the_exact_level_up_folds_clean() {
        // 7 points at level 0: progress 7/7 = 1.0 → the spend loop
        // folds (1-1)*7 = 0, the level speaks 1, 0/7 = 0
        let mut xp = ExperienceStat::new();
        xp.give_points(7);
        assert_eq!(xp.level, 1);
        assert_eq!(xp.progress, 0.0);
        assert_eq!(xp.total, 7);
    }

    #[test]
    fn the_partial_grand_rides_the_fraction() {
        // 4 points at level 0: progress 4/7, no fold
        let mut xp = ExperienceStat::new();
        xp.give_points(4);
        assert_eq!(xp.level, 0);
        assert!((xp.progress - 4.0 / 7.0).abs() < 1e-6);
        assert_eq!(xp.total, 4);
    }

    #[test]
    fn the_multi_level_spend_walks_both_curves() {
        // 7 + 9 = 16 points: level 2. The second grant spends on the
        // level-1 curve (9), the division rides the level-2 curve (11).
        // The f32 round-trips leave a hair of residue — the honest
        // float law the d2f/f2d pair speaks
        let mut xp = ExperienceStat::new();
        xp.give_points(16);
        assert_eq!(xp.level, 2);
        assert!(xp.progress < 1e-6, "the spend spends all but the float hair: {}", xp.progress);
        assert_eq!(xp.total, 16);
    }

    #[test]
    fn the_negative_grand_borrows_then_bankrupts() {
        // -10 from level 0: the borrow arm hits the level-0 floor —
        // the bankruptcy resets all three, the bytecode's own verdict
        let mut xp = ExperienceStat::new();
        xp.give_points(5);
        xp.give_points(-10);
        assert_eq!(xp.level, 0);
        assert_eq!(xp.progress, 0.0);
        assert_eq!(xp.total, 0, "the bankruptcy resets the total too");
    }

    #[test]
    fn the_ore_table_speaks_the_uniform_ranges() {
        // the DropExperienceBlock constructions the Blocks init speaks;
        // the roll rides [min, max] inclusive
        for kind in [world::COAL_ORE] {
            let mut hit = false;
            for seed in 0..64u64 {
                let v = ore_xp(kind, &mut Lcg::new(0x5EED + seed));
                assert!((0..=2).contains(&v), "coal speaks [0,2], got {v}");
                if v > 0 {
                    hit = true;
                }
            }
            assert!(hit, "the roll's window must open across seeds");
        }
        for seed in 0..64u64 {
            let v = ore_xp(world::REDSTONE_ORE, &mut Lcg::new(0x5EED + seed));
            assert!((1..=5).contains(&v), "redstone speaks [1,5], got {v}");
            let v = ore_xp(world::LAPIS_ORE, &mut Lcg::new(0x5EED + seed));
            assert!((2..=5).contains(&v), "lapis speaks [2,5], got {v}");
            let v = ore_xp(world::DIAMOND_ORE, &mut Lcg::new(0x5EED + seed));
            assert!((3..=7).contains(&v), "diamond speaks [3,7], got {v}");
            let v = ore_xp(world::EMERALD_ORE, &mut Lcg::new(0x5EED + seed));
            assert!((3..=7).contains(&v), "emerald speaks [3,7], got {v}");
        }
    }

    #[test]
    fn the_silent_ores_burn_no_roll() {
        // copper/iron/gold grant NOTHING (the smelting lane's own
        // reward) — and the non-XpBlock gate never samples, so the
        // rng's state rides untouched (the same first draw a fresh
        // same-seed Lcg speaks)
        let mut rng = Lcg::new(0xBEEF);
        for kind in [
            world::COPPER_ORE,
            world::IRON_ORE,
            world::GOLD_ORE,
            world::STONE,
        ] {
            assert_eq!(ore_xp(kind, &mut rng), 0, "{} never grants", kind);
        }
        let mut fresh = Lcg::new(0xBEEF);
        assert_eq!(rng.next(), fresh.next(), "the silent ores never roll");
    }

    #[test]
    fn the_bankruptcy_via_levels_resets_the_stat() {
        // giveExperienceLevels' own negative fall: the saturated add
        // rides, the < 0 verdict resets all three. 20 points = level 2;
        // the first -1 speaks level 1 (no bankruptcy), the walk under
        // zero flips the verdict
        let mut xp = ExperienceStat::new();
        xp.give_points(20);
        assert_eq!(xp.level, 2, "20 points climbed two levels");
        xp.give_levels(-1);
        assert_eq!(xp.level, 1);
        assert_eq!(xp.total, 20, "a plain -1 never bankrupts");
        xp.give_levels(-1);
        assert_eq!(xp.level, 0, "the add rides the floor exactly");
        assert_eq!(xp.total, 20);
        xp.give_levels(-1);
        assert_eq!(xp.level, 0);
        assert_eq!(xp.progress, 0.0);
        assert_eq!(xp.total, 0, "the fall under resets the stat");
    }
}

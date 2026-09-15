//! THE EXPERIENCE STAT (javap `Player` over the 26.2 jar — the XP
//! quartet the HUD's contextual bar and the grant lane speak):
//! `experienceLevel` the int, `experienceProgress` the f32 fraction,
//! `totalExperience` the clamped sum. The stat is the OFFLINE lane's
//! stand-in server voice — the NETWORKED lane's truth rides the wire's
//! Set Experience (the future's own parse).

use crate::drops::Lcg;
use crate::world;

/// THE ORB ENTITY (javap `ExperienceOrb` over the 26.2 jar — the
/// award split's own physical truth): the value one split step speaks,
/// the zero-velocity spawn the `award` path passes (`Vec3.ZERO`), the
/// 0.5 cube the bbox law holds, the 6000-tick LIFETIME the age discards.
#[derive(Debug, Clone)]
pub struct ExperienceOrb {
    /// the entity id (the client's own counter — the wire ids stay the
    /// server's)
    pub id: i32,
    /// the orb's own XP (ONE `getExperienceValue` step of the award)
    pub value: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    /// the age clock (the LIFETIME's own discard at 6000)
    pub age: i32,
    pub on_ground: bool,
}

/// `LIFETIME` (javap the age tail's `sipush 6000` — the discard).
pub const ORB_LIFETIME: i32 = 6000;
/// `getDefaultGravity` (javap: `ldc2_w 0.03d`).
pub const ORB_GRAVITY: f64 = 0.03;
/// The bbox half-width (javap the dimensions `0.5F` — the cube).
pub const ORB_HALF: f64 = 0.25;
/// The follow scan's own radius (javap `getNearestPlayer(this, 8.0D)`
/// and the keep gate's `64.0d` squared fold).
pub const ORB_FOLLOW_RADIUS: f64 = 8.0;
/// The follow accel's own fold (javap: `scale(e * e * 0.1D)` over
/// `e = 1.0 - sqrt(lenSqr) / 8.0`).
pub const ORB_FOLLOW_ACCEL: f64 = 0.1;

/// `getExperienceValue`'s own band walk (javap — the descending bands
/// the `if_icmplt` chain speaks; the award loop's one split step).
pub fn get_experience_value(amount: i32) -> i32 {
    for band in [2477, 1237, 617, 307, 149, 73, 37, 17, 7, 3] {
        if amount >= band {
            return band;
        }
    }
    1
}

/// `getIcon`'s own band walk (javap — the sprite variant index the
/// renderer's uv law speaks: `u = (icon % 4) * 16`, `v = (icon / 4) * 16`
/// over the 64x64 sheet's 4x4 grid).
pub fn orb_icon(value: i32) -> i32 {
    for (band, idx) in [
        (2477, 10),
        (1237, 9),
        (617, 8),
        (307, 7),
        (149, 6),
        (73, 5),
        (37, 4),
        (17, 3),
        (7, 2),
        (3, 1),
    ] {
        if value >= band {
            return idx;
        }
    }
    0
}

impl ExperienceOrb {
    /// The constructor's own shape (javap the `(Level, Vec3, Vec3, int)`
    /// overload the `awardWithDirection` path arms): the position rides,
    /// the delta starts ZERO (the award's own `Vec3.ZERO`), the age 0.
    pub fn spawn(id: i32, x: f64, y: f64, z: f64, value: i32) -> Self {
        Self {
            id,
            value,
            x,
            y,
            z,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            age: 0,
            on_ground: false,
        }
    }

    /// The bbox (javap the 0.5F dimensions — the cube the touch check
    /// intersects the player's own body against).
    pub fn box_(&self) -> (f64, f64, f64, f64, f64, f64) {
        (
            self.x - ORB_HALF,
            self.y,
            self.z - ORB_HALF,
            self.x + ORB_HALF,
            self.y + ORB_HALF * 2.0,
            self.z + ORB_HALF,
        )
    }

    /// The orb's own tick (javap the 26.2 tick's order): the gravity
    /// (`applyGravity`'s `getDefaultGravity` 0.03), the follow scan's
    /// accel (`followNearbyPlayer`'s `scale(e * e * 0.1)` toward the
    /// eye-mid point the `getY() + getEyeHeight() / 2.0` fold speaks),
    /// the move + collide, the drag (the ground friction fold the
    /// `getAirDrag() * block friction` shape speaks), the age clock's
    /// LIFETIME discard. The lava wander and the merge scan stay the
    /// server's own (the offline vocabulary has no lava pools and the
    /// orbs spawn one value each — the merge never speaks).
    pub fn tick(&mut self, world: &crate::world::World, player: Option<&crate::mob::PlayerView>) {
        // setUnderwaterMovement's shape (the buoyant lift the water
        // branch speaks — the same fold the drops' tick carries)
        let in_water = world.get(
            self.x.floor() as i32,
            self.y.floor() as i32,
            self.z.floor() as i32,
        ) == world::WATER;
        if in_water {
            self.vx *= 0.9;
            self.vz *= 0.9;
            self.vy = self.vy * 0.8 + 0.05;
        } else {
            // applyGravity (getDefaultGravity's own 0.03)
            self.vy -= ORB_GRAVITY;
        }
        // followNearbyPlayer (javap): the target rides the eye-mid fold
        // `player.getY() + getEyeHeight() / 2.0` — the PlayerView's own
        // (feet + eye) / 2 midpoint speaks the same law for any eye
        // height; the accel `e * e * 0.1` over `e = 1 - sqrt(lenSqr)/8`
        // rides only within the 8.0 scan (the dead player the scan
        // refuses, the spectator the client never speaks)
        if let Some(p) = player {
            if !p.dead {
                let dx = p.x - self.x;
                let dy = (p.y + p.eye_y) / 2.0 - self.y;
                let dz = p.z - self.z;
                let len_sqr = dx * dx + dy * dy + dz * dz;
                let e = 1.0 - len_sqr.sqrt() / ORB_FOLLOW_RADIUS;
                if len_sqr > 1.0e-12 && e > 0.0 {
                    let len = len_sqr.sqrt();
                    let s = e * e * ORB_FOLLOW_ACCEL / len;
                    self.vx += dx * s;
                    self.vy += dy * s;
                    self.vz += dz * s;
                }
            }
        }
        // the move + collide (the vanilla move's own restitution: the
        // collided axes zero)
        let c = crate::body::collide(world, self.box_(), (self.vx, self.vy, self.vz));
        self.x += c.dx;
        self.y += c.dy;
        self.z += c.dz;
        if c.collided_x {
            self.vx = 0.0;
        }
        if c.collided_z {
            self.vz = 0.0;
        }
        self.on_ground = c.vertical_collision_below;
        if c.collided_y {
            self.vy = 0.0;
        }
        // the drag (javap the tick tail: the ground fold rides the block
        // friction, the air rides 0.98 — the same shape the drops speak)
        let f3 = if self.on_ground {
            crate::drops::AIR_DRAG * crate::drops::BLOCK_FRICTION
        } else {
            crate::drops::AIR_DRAG
        };
        self.vx *= f3;
        self.vy *= crate::drops::AIR_DRAG;
        self.vz *= f3;
        self.age += 1;
    }

    /// The touch check (javap the `playerTouch` dispatch's own bbox
    /// intersect — the plain overlap, no inflate, the gate the caller's
    /// takeXpDelay fold arms).
    pub fn touches_body(&self, body: &crate::body::PlayerBody) -> bool {
        let height = if body.crouching {
            crate::body::CROUCH_HEIGHT
        } else {
            crate::body::STANDING_HEIGHT
        };
        let hw = crate::body::WIDTH / 2.0;
        let (px0, py0, pz0) = (body.x - hw, body.y, body.z - hw);
        let (px1, py1, pz1) = (body.x + hw, body.y + height, body.z + hw);
        let b = self.box_();
        b.0 < px1 && b.3 > px0 && b.1 < py1 && b.4 > py0 && b.2 < pz1 && b.5 > pz0
    }
}

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

    #[test]
    fn the_split_walks_the_value_bands() {
        // getExperienceValue's own band chain: the largest band the
        // amount reaches, the 1 the sub-3 fall speaks
        assert_eq!(get_experience_value(1), 1);
        assert_eq!(get_experience_value(2), 1);
        assert_eq!(get_experience_value(3), 3);
        assert_eq!(get_experience_value(6), 3);
        assert_eq!(get_experience_value(7), 7);
        assert_eq!(get_experience_value(16), 7);
        assert_eq!(get_experience_value(17), 17);
        assert_eq!(get_experience_value(36), 17);
        assert_eq!(get_experience_value(37), 37);
        assert_eq!(get_experience_value(72), 37);
        assert_eq!(get_experience_value(73), 73);
        assert_eq!(get_experience_value(148), 73);
        assert_eq!(get_experience_value(149), 149);
        assert_eq!(get_experience_value(306), 149);
        assert_eq!(get_experience_value(307), 307);
        assert_eq!(get_experience_value(616), 307);
        assert_eq!(get_experience_value(617), 617);
        assert_eq!(get_experience_value(1236), 617);
        assert_eq!(get_experience_value(1237), 1237);
        assert_eq!(get_experience_value(2476), 1237);
        assert_eq!(get_experience_value(2477), 2477);
        assert_eq!(get_experience_value(100000), 2477);
    }

    #[test]
    fn the_award_loop_splits_the_sum_exact() {
        // awardWithDirection's own while loop: the split steps must
        // consume the amount exactly (the sum the orbs speak)
        for amount in 1..=250 {
            let mut rest = amount;
            let mut orbs = Vec::new();
            while rest > 0 {
                let v = get_experience_value(rest);
                rest -= v;
                orbs.push(v);
            }
            assert_eq!(orbs.iter().sum::<i32>(), amount, "amount {amount}");
            // the greedy band walk's own shape: the fragment count
            // peaks at the sub-1 tails (the 139 the [73,37,17,7,3,1,1]
            // seven speak — the vanilla's own verdict), every orb rides
            // a band value
            assert!(orbs.len() <= 8, "amount {amount}: {orbs:?}");
            assert!(orbs.iter().all(|v| [1, 3, 7, 17, 37, 73, 149, 307, 617, 1237, 2477].contains(v)));
        }
        // the kill's own flat 5: the [3, 1, 1] walk the bands speak
        // (the greedy 3, then the sub-3 remainder's own 1s — the
        // vanilla has no 2 band)
        let mut rest = 5;
        let mut orbs = Vec::new();
        while rest > 0 {
            let v = get_experience_value(rest);
            rest -= v;
            orbs.push(v);
        }
        assert_eq!(orbs, vec![3, 1, 1]);
    }

    #[test]
    fn the_icon_walks_the_sprite_bands() {
        // getIcon's own band chain (the 4x4 sheet's variant index)
        for (value, icon) in [
            (1, 0),
            (2, 0),
            (3, 1),
            (6, 1),
            (7, 2),
            (16, 2),
            (17, 3),
            (36, 3),
            (37, 4),
            (72, 4),
            (73, 5),
            (148, 5),
            (149, 6),
            (306, 6),
            (307, 7),
            (616, 7),
            (617, 8),
            (1236, 8),
            (1237, 9),
            (2476, 9),
            (2477, 10),
            (100000, 10),
        ] {
            assert_eq!(orb_icon(value), icon, "value {value}");
        }
        // the icon's uv law: 4 columns, 4 rows over the 64x64 sheet
        for icon in 0..=10 {
            let u = (icon % 4) * 16;
            let v = (icon / 4) * 16;
            assert!(u < 64 && v < 64, "icon {icon} in the sheet");
        }
    }

    #[test]
    fn the_follow_pulls_toward_the_eye_mid() {
        // followNearbyPlayer's own fold: the accel e*e*0.1 toward the
        // (feet + eye) / 2 midpoint, the 8.0 scan the pull sleeps past
        let world = crate::world::World::new(2, 42);
        let mut orb = ExperienceOrb::spawn(1, 20.5, 40.0, 20.5, 3);
        let p = crate::mob::PlayerView {
            x: 21.0,
            y: 40.0,
            z: 20.5,
            eye_y: 41.62,
            dead: false,
            width: 0.6,
        };
        // far past the scan: the pull never speaks, the gravity does
        let mut far = orb.clone();
        far.x = 50.5;
        far.z = 50.5;
        let vy_before = far.vy;
        far.tick(&world, Some(&p));
        assert!((far.vx, far.vz) == (0.0, 0.0), "out of the 8.0 scan no pull");
        assert!(far.vy < vy_before, "the gravity still speaks");
        // in range: the pull rides toward the mid (40.0 + 41.62) / 2
        orb.tick(&world, Some(&p));
        assert!(orb.vx > 0.0 || orb.vz > 0.0, "the pull speaks in range");
        // the dead player the scan refuses
        let mut dead_orb = ExperienceOrb::spawn(2, 20.5, 40.0, 20.5, 3);
        let dead_p = crate::mob::PlayerView { dead: true, ..p };
        dead_orb.tick(&world, Some(&dead_p));
        assert!((dead_orb.vx, dead_orb.vz) == (0.0, 0.0), "the dead never pull");
    }

    #[test]
    fn the_orb_rests_on_the_ground_and_ages_out() {
        // the gravity settles the orb onto the floor, the age clock
        // walks to the LIFETIME's own discard
        let world = crate::world::World::new(2, 42);
        let sx = 8;
        let mut sy = 40;
        while sy > 0 && !crate::body::solid(&world, sx, sy - 1, 8) {
            sy -= 1;
        }
        let mut orb = ExperienceOrb::spawn(1, sx as f64 + 0.5, sy as f64 + 2.0, 8.5, 5);
        for _ in 0..80 {
            orb.tick(&world, None);
        }
        assert!(orb.on_ground, "the orb settles");
        assert_eq!(orb.vy, 0.0, "the rest zero");
        let y_rest = orb.y;
        for _ in 0..20 {
            orb.tick(&world, None);
        }
        assert_eq!(orb.y, y_rest, "the rest holds");
        // the lifetime: 6000 - the walked age speaks the discard's own
        // boundary (the caller's retain the age check answers)
        let mut old = ExperienceOrb::spawn(2, sx as f64 + 0.5, sy as f64 + 2.0, 8.5, 5);
        old.age = ORB_LIFETIME - 1;
        old.tick(&world, None);
        assert_eq!(old.age, ORB_LIFETIME, "the boundary tick rides");
    }

    #[test]
    fn the_touch_reads_the_plain_bbox() {
        // playerTouch's own intersect: the plain overlap, no inflate —
        // the orb inside the body touches, the orb one gap away does not
        let body = crate::body::PlayerBody::new(10.0, 1.0, 10.0, 0.0, 0.0);
        let on = ExperienceOrb::spawn(1, 10.2, 1.2, 10.0, 3);
        assert!(on.touches_body(&body), "the inside orb touches");
        let off = ExperienceOrb::spawn(2, 12.0, 1.2, 10.0, 3);
        assert!(!off.touches_body(&body), "the gap refuses");
        let under = ExperienceOrb::spawn(3, 10.0, 4.0, 10.0, 3);
        assert!(!under.touches_body(&body), "the above-body refuses");
    }
}

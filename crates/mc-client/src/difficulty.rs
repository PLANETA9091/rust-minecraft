//! THE LOCAL DIFFICULTY'S OWN FOLD (javap the 26.2 jar the
//! `DifficultyInstance` class speaks — the world's own danger dial the
//! F3's `Local Difficulty: %.2f // %.2f` row wears).

/// THE WORLD'S OWN DANGER DIAL (the `Difficulty` enum the jar's own
/// ids keep — `getId()` the byte the constructor's f2f cast wears:
/// Peaceful 0, Easy 1, Normal 2, Hard 3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    /// The vanilla id (`Difficulty.getId()I` — the int the fold's i2f
    /// cast wears).
    pub fn id(self) -> u8 {
        match self {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        }
    }
}

impl Difficulty {
    /// THE VALUES-ORDER TABLE THE CLINIT KEEPS (the enum's own
    /// declaration order — the `Difficulty.values()` array the
    /// WorldOptionsScreen's CycleButton `withValues` walk banks
    /// verbatim, the cycle the `next` step answers).
    pub const VALUES: [Difficulty; 4] = [
        Difficulty::Peaceful,
        Difficulty::Easy,
        Difficulty::Normal,
        Difficulty::Hard,
    ];

    /// THE CYCLE BUTTON'S OWN STEP (javap the 26.2 jar the
    /// `DifficultyButtons.create` walk speaks — the CycleButton the
    /// `withValues(Difficulty.values())` array rides, the click the
    /// NEXT value the enum's own declaration order answers, the tail
    /// the wrap keeps): Peaceful -> Easy -> Normal -> Hard -> Peaceful.
    pub fn next(self) -> Difficulty {
        Self::VALUES
            .iter()
            .position(|&d| d == self)
            .map(|i| Self::VALUES[(i + 1) % Self::VALUES.len()])
            .unwrap_or(Difficulty::Normal)
    }

    /// THE EN_US WORDS THE CYCLE BUTTON DISPLAYS (the
    /// `options.difficulty.*` keys the lang table keeps — the
    /// `getDisplayName` component the vanilla CycleButton's own
    /// `label: value` composition rides): Peaceful, Easy, Normal, Hard.
    pub fn display_name(self) -> &'static str {
        match self {
            Difficulty::Peaceful => "Peaceful",
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard",
        }
    }
}

/// THE MOON'S OWN BRIGHTNESS PER PHASE (javap the 26.2 jar the
/// `DimensionType.<clinit>` walk speaks — the 8-float array the
/// `MOON_BRIGHTNESS_PER_PHASE` putstatic banks: full 1.0 the phase-0
/// arm wears, the wane the half-steps keeps, the new moon the 0.0
/// phase-4 answers, the wax the same half-steps back; the index the
/// `MoonPhase.index()` walk speaks, the row the
/// `ServerLevel.getMoonBrightness` faload wears).
pub const MOON_BRIGHTNESS_PER_PHASE: [f32; 8] = [1.0, 0.75, 0.5, 0.25, 0.0, 0.25, 0.5, 0.75];

/// THE EFFECTIVE DIFFICULTY'S OWN LAW (javap the 26.2 jar the
/// `DifficultyInstance.calculateDifficulty` walk speaks — the private
/// static the constructor's one call banks). THE WALK: the PEACEFUL
/// arm the 0.0 keeps (`if_acmpne` the fconst_0/freturn pair); the
/// HARD flag the istore keeps; the day ramp the
/// `(dayTime - 72000) / 1440000` clamp-[0,1] × 0.25 fold the l2f/fadd
/// pair wears (the JVM's l2f the i64→f32 nearest cast answers, the
/// f32 width the whole ramp keeps); the base `0.75 + ramp`; the
/// inhabited ramp the `inhabitedTime / 3600000` clamp-[0,1] fold wears
/// — the HARD arm the ×1.0 multiplier keeps, every other difficulty
/// the ×0.75 the ldc banks; the moon term the
/// `clamp(moonBrightness * 0.25, 0.0, dayRamp)` fadd answers (the
/// day ramp the moon's own ceiling keeps); the EASY arm the ×0.5 fmul
/// wears; the sum the base + the walk the id multiply returns
/// (`getId()` the i2f the final fmul speaks).
pub fn effective_difficulty(
    difficulty: Difficulty,
    day_time: i64,
    inhabited: u64,
    moon_brightness: f32,
) -> f32 {
    if difficulty == Difficulty::Peaceful {
        return 0.0;
    }
    let hard = difficulty == Difficulty::Hard;
    let day_ramp = clamp01((day_time as f32 - 72000.0) / 1440000.0) * 0.25;
    let mut f = 0.75 + day_ramp;
    let inhabited_ramp =
        clamp01(inhabited as f32 / 3600000.0) * if hard { 1.0 } else { 0.75 };
    let mut special = inhabited_ramp;
    special += clamp(moon_brightness * 0.25, 0.0, day_ramp);
    if difficulty == Difficulty::Easy {
        special *= 0.5;
    }
    f += special;
    f32::from(difficulty.id()) * f
}

/// THE SPECIAL MULTIPLIER'S OWN LAW (javap the 26.2 jar the
/// `DifficultyInstance.getSpecialMultiplier` walk speaks — the
/// `fcmpg 2.0` the 0.0 arm opens (`eff < 2.0`), the `fcmpl 4.0` the
/// 1.0 arm wears (`eff > 4.0`), the middle the `(eff - 2.0) / 2.0`
/// half-step keeps).
pub fn special_multiplier(effective: f32) -> f32 {
    if effective < 2.0 {
        0.0
    } else if effective > 4.0 {
        1.0
    } else {
        (effective - 2.0) / 2.0
    }
}

/// `Mth.clamp(value, min, max)` — the f32 width the bytecode keeps.
fn clamp(v: f32, min: f32, max: f32) -> f32 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

fn clamp01(v: f32) -> f32 {
    clamp(v, 0.0, 1.0)
}

/// `String.format(Locale.ROOT, "%.2f", v)`'s own word (the
/// `DebugEntryLocalDifficulty` ldc the two args' row wears) — the
/// Formatter's HALF_UP law at the second decimal: the exact decimal
/// of the f32's f64 promotion, the third digit the tie answers
/// (2.125 the `2.13` up-round speaks). The row's own inputs the
/// non-negative small domain keeps (|v| < 2^16: the ×100 product the
/// exact 53-bit f64 mantissa banks, the tie the scaled + 0.5 floor
/// never misses — the `mood_percent` round's own precedent the
/// strict walk keeps).
pub fn java_two_decimals(v: f32) -> String {
    let r = (f64::from(v) * 100.0 + 0.5).floor() as i64;
    format!("{}.{:02}", r / 100, r % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_effective_difficulty_folds_the_jar_walk() {
        // THE PEACEFUL ARM (the acmp the 0.0 keeps — every other
        // input the dial ignores)
        assert_eq!(
            effective_difficulty(Difficulty::Peaceful, 999_999, 9_999_999, 1.0),
            0.0
        );
        // THE DAWN'S OWN FLOOR (NORMAL, day 0, no dwell, full moon):
        // the day ramp the negative clamp zeroes, the moon term the
        // zeroed ramp's own ceiling keeps — the 0.75 base the id-2
        // multiply answers: 1.5
        let dawn = effective_difficulty(Difficulty::Normal, 0, 0, 1.0);
        assert_eq!(dawn, 1.5);
        // THE DAY RAMP'S OWN FULL STEP (the 72000 offset the ramp
        // opens, the 1_440_000 span the 0.25 ceiling wears): the
        // dawn's ramp 0.0, the span's end 0.25 — the base the 1.0
        // answers, the id the 2.0 speaks
        assert_eq!(
            effective_difficulty(Difficulty::Normal, 72000, 0, 0.0),
            2.0 * 0.75
        );
        assert_eq!(
            effective_difficulty(Difficulty::Normal, 72000 + 1_440_000, 0, 0.0),
            2.0 * 1.0
        );
        // THE INHABITED RAMP'S OWN CEILING (the 3_600_000 span the
        // clamp-1 wears, the NORMAL arm the ×0.75 multiplier keeps)
        assert_eq!(
            effective_difficulty(Difficulty::Normal, 0, 3_600_000, 0.0),
            2.0 * (0.75 + 0.75)
        );
        // THE MOON TERM'S OWN CEILING (the full moon's 0.25 the
        // day-ramp's own max caps — the ramp the term's ceiling keeps:
        // the moon term can never outgrow the day ramp)
        assert_eq!(
            effective_difficulty(Difficulty::Normal, 72000 + 1_440_000, 0, 1.0),
            2.0 * (1.0 + 0.25)
        );
        // THE EASY ARM'S OWN HALF (the special walk the ×0.5 fmul
        // wears, the id-1 the same base multiplies)
        assert_eq!(
            effective_difficulty(Difficulty::Easy, 72000 + 1_440_000, 0, 1.0),
            1.0 * (1.0 + 0.125)
        );
        // THE HARD ARM'S OWN MULTPLIER (the inhabited ramp the ×1.0
        // keeps — the ×0.75 every other difficulty banks — the id-3
        // the same walk multiplies)
        assert_eq!(
            effective_difficulty(Difficulty::Hard, 0, 3_600_000, 0.0),
            3.0 * (0.75 + 1.0)
        );
    }

    #[test]
    fn the_special_multiplier_keeps_the_fcmp_arms() {
        // THE fcmpg 2.0 ARM (the 0.0 the below-keep wears)
        assert_eq!(special_multiplier(0.0), 0.0);
        assert_eq!(special_multiplier(1.999), 0.0);
        // THE fcmpl 4.0 ARM (the 1.0 the above-keep wears)
        assert_eq!(special_multiplier(4.001), 1.0);
        assert_eq!(special_multiplier(6.75), 1.0);
        // THE MIDDLE'S OWN HALF-STEP (the (eff - 2) / 2 the 2..4 span
        // wears — the 3.0 the exact 0.5 answers)
        assert_eq!(special_multiplier(2.0), 0.0);
        assert_eq!(special_multiplier(3.0), 0.5);
        assert_eq!(special_multiplier(4.0), 1.0);
    }

    #[test]
    fn the_two_decimals_wear_the_formatter_half_up() {
        // THE TIE THE HALF-UP ROUND ANSWERS (2.125 the exact binary
        // the f32 keeps — the `2.13` the Formatter speaks, the
        // banker's `2.12` the law never wears)
        assert_eq!(java_two_decimals(2.125), "2.13");
        assert_eq!(java_two_decimals(0.125), "0.13");
        // THE WHOLE'S OWN ZEROS (the %.2f the two-cent pad keeps)
        assert_eq!(java_two_decimals(1.0), "1.00");
        assert_eq!(java_two_decimals(0.0), "0.00");
        // THE DAWN'S OWN WORD (the 1.5009375 the inhabited bank's
        // own effective wears — the third digit 0 the no-round keeps)
        assert_eq!(java_two_decimals(1.5009375), "1.50");
        // THE SPAN'S OWN TAIL (6.75 the exact quarter the no-tie
        // keeps)
        assert_eq!(java_two_decimals(6.75), "6.75");
    }

    #[test]
    fn the_moon_table_wears_the_clinit_walk() {
        // THE 8-PHASE TABLE (DimensionType.<clinit>'s own array — the
        // full moon the 1.0 phase-0 keeps, the wane the half-steps,
        // the new moon the 0.0 phase-4 answers, the wax the same
        // half-steps back (the wrap the phase-0's own 1.0 completes:
        // the wax arm reaches 0.75 at phase 7, the full the next
        // period's phase 0 wears))
        assert_eq!(MOON_BRIGHTNESS_PER_PHASE[0], 1.0);
        assert_eq!(MOON_BRIGHTNESS_PER_PHASE[4], 0.0);
        assert_eq!(MOON_BRIGHTNESS_PER_PHASE[7], 0.75);
        // the V the wane/wax pairs keep (the new moon the fold's own
        // hinge answers): 1↔7, 2↔6, 3↔5 mirrored about the phase-4
        // zero — the 0↔0 pair the period wrap completes (the next
        // period's phase 0 the same full moon wears)
        for i in 1..4usize {
            assert_eq!(
                MOON_BRIGHTNESS_PER_PHASE[i],
                MOON_BRIGHTNESS_PER_PHASE[8 - i],
                "phase {i} the wax/wane pair keeps"
            );
        }
        assert_eq!(MOON_BRIGHTNESS_PER_PHASE[4], 0.0);
    }

    /// THE CYCLE'S OWN WALK (javap the 26.2 jar the
    /// `DifficultyButtons.create` + WorldOptionsScreen walks speak —
    /// the `withValues(Difficulty.values())` array the CycleButton
    /// rides, the click the next value the enum's own declaration
    /// order answers): the VALUES table the clinit order banks (the
    /// ids the getId bytes keep), the full cycle the tail's wrap
    /// keeps, the display words the `options.difficulty.*` keys the
    /// lang table speaks, the food vocabulary map the four arms the
    /// FoodData ctx wears.
    #[test]
    fn the_difficulty_values_cycle_the_clinit_order() {
        // THE VALUES TABLE (the declaration order — the ids the
        // getId walk keeps: Peaceful 0, Easy 1, Normal 2, Hard 3)
        assert_eq!(Difficulty::VALUES.len(), 4);
        for (i, d) in Difficulty::VALUES.iter().enumerate() {
            assert_eq!(d.id() as usize, i, "the id the ordinal banks");
        }
        // THE CYCLE (the click the +1 ordinal answers, the tail the
        // Peaceful wrap keeps)
        assert_eq!(Difficulty::Peaceful.next(), Difficulty::Easy);
        assert_eq!(Difficulty::Easy.next(), Difficulty::Normal);
        assert_eq!(Difficulty::Normal.next(), Difficulty::Hard);
        assert_eq!(Difficulty::Hard.next(), Difficulty::Peaceful);
        // the full walk the new-world default rides — four turns the
        // dial rings home
        let mut d = Difficulty::Normal;
        for _ in 0..4 {
            d = d.next();
        }
        assert_eq!(d, Difficulty::Normal, "the four-turn walk rings home");
        // THE DISPLAY WORDS (the `options.difficulty.*` keys the en_us
        // lang table keeps — the CycleButton's `Difficulty: <name>`
        // composition the value slot wears)
        assert_eq!(Difficulty::Peaceful.display_name(), "Peaceful");
        assert_eq!(Difficulty::Easy.display_name(), "Easy");
        assert_eq!(Difficulty::Normal.display_name(), "Normal");
        assert_eq!(Difficulty::Hard.display_name(), "Hard");
        // THE FOOD VOCABULARY MAP (the same four arms the FoodData
        // tick's ctx wears — the dial's turn the gates read)
        for (from, to) in [
            (Difficulty::Peaceful, crate::food::Difficulty::Peaceful),
            (Difficulty::Easy, crate::food::Difficulty::Easy),
            (Difficulty::Normal, crate::food::Difficulty::Normal),
            (Difficulty::Hard, crate::food::Difficulty::Hard),
        ] {
            assert_eq!(crate::food::Difficulty::of(from), to);
        }
    }
}

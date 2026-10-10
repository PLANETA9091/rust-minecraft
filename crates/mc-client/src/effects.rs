//! THE STATUS-EFFECT STORE (javap of the 26.2 `MobEffectInstance`,
//! `LivingEntity.addEffect`/`tickEffects`/`actuallyHurt`,
//! `AbsorptionMobEffect`/`RegenerationMobEffect` — the real laws, one
//! store for both lanes): the LOCAL player's active effects, the
//! add/update/compare laws the wire and the eat arc ride, the tick-down
//! the HUD's blink reads, the absorption value the white hearts row
//! speaks, and the effect-icons row's sort.
//!
//! - `add_effect` = `LivingEntity.addEffect`: the absent effect inserts
//!   and fires `MobEffectInstance.onEffectStarted`; the present one runs
//!   `MobEffectInstance.update` — a strictly greater amplifier replaces,
//!   an equal amplifier with a longer duration replaces (the refresh),
//!   everything else keeps the standing instance. `onEffectStarted`
//!   fires on BOTH branches (the bytecode runs it unconditionally at the
//!   tail), which is exactly the absorption's set law: the value lifts
//!   to `max(current, 4 * (1 + amplifier))` on every application.
//! - The vanilla `MobEffectInstance.update`'s hidden-effect downgrade
//!   chain (the shorter stronger instance stashing the longer weaker
//!   one for the expiry swap) stays the documented absence it already
//!   was in the model's wire store: our effects never mix amplifiers on
//!   one id long enough to show the swap, and the wire's next Update
//!   speaks the truth regardless.
//! - `set_absorption` clamps to `[0, MAX_ABSORPTION]` (the attribute's
//!   own 20.0 default the clamp reads).
//! - Removal never resets the absorption value (the 26.2
//!   `AbsorptionMobEffect` carries no removal hook — the hearts persist
//!   until damage drains them).

use std::collections::BTreeMap;

use crate::world_model::MobEffectState;

/// `minecraft:regeneration`'s mob-effect registry id — the 0-based wire
/// order the generated `MOB_EFFECT_ORDER` table carries (the test pins
/// the slot).
pub const MOB_EFFECT_REGENERATION: i32 = 9;
/// `minecraft:resistance`'s mob-effect registry id (the 0-based
/// order's own slot — the damage-fold lane's scale arm reads it).
pub const MOB_EFFECT_RESISTANCE: i32 = 10;
/// `minecraft:fire_resistance`'s mob-effect registry id (the 0-based
/// order's own slot — the lava fog's vision arm reads it).
pub const MOB_EFFECT_FIRE_RESISTANCE: i32 = 11;
/// `minecraft:absorption`'s mob-effect registry id (the test pins it).
pub const MOB_EFFECT_ABSORPTION: i32 = 21;

/// `minecraft:instant_health`'s mob-effect registry id — the 0-based
/// wire order's own slot (the generated `MOB_EFFECT_ORDER` table's 6th
/// row, the HealOrHarmMobEffect's beneficial row).
pub const MOB_EFFECT_INSTANT_HEALTH: i32 = 5;

/// `minecraft:instant_damage`'s mob-effect registry id (the wire
/// order's own 7th row, the HealOrHarmMobEffect's harmful row).
pub const MOB_EFFECT_INSTANT_DAMAGE: i32 = 6;

/// `minecraft:poison`'s mob-effect registry id (the wire order's own
/// 19th row — the undead-refusal lane's one arm reads it).
pub const MOB_EFFECT_POISON: i32 = 18;

/// `minecraft:wither`'s mob-effect registry id (the wire order's own
/// 20th row — the periodic-dose lane's ungated arm reads it).
pub const MOB_EFFECT_WITHER: i32 = 19;

/// `minecraft:hunger`'s mob-effect registry id (the wire order's own
/// 17th row — the exhaustion-drain lane's every-tick beat reads it).
pub const MOB_EFFECT_HUNGER: i32 = 16;

/// `minecraft:weakness`'s mob-effect registry id (the wire order's own
/// 18th row — the hunger-16/poison-18 pair brackets the slot the
/// MobEffects clinit's own registration order keeps: the `hunger` row
/// puts at pc 466, the `weakness` row at pc 503, the `poison` row
/// follows). The attack-damage attribute lane's harmful arm reads it.
pub const MOB_EFFECT_WEAKNESS: i32 = 17;

/// `minecraft:saturation`'s mob-effect registry id (the wire order's
/// own 23rd row, the InstantaneousMobEffect's food row).
pub const MOB_EFFECT_SATURATION: i32 = 22;

/// `minecraft:darkness`'s mob-effect registry id — the 0-based wire
/// order's own slot (the generated `MOB_EFFECT_ORDER` table's 33rd row,
/// the index the warden's lightmap lane consumes).
pub const MOB_EFFECT_DARKNESS: i32 = 32;

/// `minecraft:speed`'s mob-effect registry id — the 0-based wire
/// order's own FIRST row (the attribute lane's movement fold reads it).
pub const MOB_EFFECT_SPEED: i32 = 0;

/// `minecraft:haste`'s mob-effect registry id (the wire order's own
/// THIRD row — the dig-speed lane's helpful arm reads it, the
/// `+0.2 × (amplifier + 1)` multiplier the ladder's head wears).
pub const MOB_EFFECT_HASTE: i32 = 2;

/// `minecraft:mining_fatigue`'s mob-effect registry id (the wire
/// order's own FOURTH row — the haste-2 slot follows, the dig-speed
/// lane's harmful arm reads it).
pub const MOB_EFFECT_MINING_FATIGUE: i32 = 3;

/// `minecraft:strength`'s mob-effect registry id (the wire order's own
/// FIFTH row — the fatigue-3 slot follows, the attack-damage attribute
/// lane's helpful arm reads it).
pub const MOB_EFFECT_STRENGTH: i32 = 4;

/// `minecraft:slowness`'s mob-effect registry id (the wire order's own
/// SECOND row, the movement fold's harmful arm).
pub const MOB_EFFECT_SLOWNESS: i32 = 1;

/// `minecraft:jump_boost`'s mob-effect registry id (the wire order's
/// own 8th row — the jump-power and safe-fall lanes read it).
pub const MOB_EFFECT_JUMP_BOOST: i32 = 7;

/// DARKNESS's own blend duration (javap MobEffects pc 884-908 — the
/// `new MobEffect(HARMFUL, 2696993).setBlendDuration(22)` registration
/// walk: the single-arg overload delegates `setBlendDuration(22, 22,
/// 22)` (javap MobEffect setBlendDuration pc 0-7 — the same int the
/// THREE fields speak), so the blend-in, the blend-out and the
/// blend-out ADVANCE padding all wear 22 ticks. The advance is the
/// blend's own padding law (javap BlendState.hasEffect pc 0-27 — the
/// `!instance.endsWithin(effect.getBlendOutAdvanceTicks())` verdict:
/// the factor's target flips to 0 the 22 ticks BEFORE the expiry, the
/// fade-out completes exactly as the instance dies — the removal's
/// snap the padding makes invisible).
pub const DARKNESS_BLEND_TICKS: i32 = 22;

/// The per-effect blend durations `(blend_in, blend_out, advance)` —
/// javap MobEffects clinit: the effects the `setBlendDuration` call
/// rides carry their own row, every PLAIN `MobEffect` ctor the JVM's
/// zero-init keeps (the 0 blend ticks the factor SNAPS to the target —
/// the fade never speaks). The DARKNESS row the registration's own
/// `setBlendDuration(22)` banks.
fn blend_durations(effect_id: i32) -> (i32, i32, i32) {
    if effect_id == MOB_EFFECT_DARKNESS {
        (
            DARKNESS_BLEND_TICKS,
            DARKNESS_BLEND_TICKS,
            DARKNESS_BLEND_TICKS,
        )
    } else {
        (0, 0, 0)
    }
}

/// The flags' packing the Update Mob Effect packet speaks: bit 0 ambient,
/// bit 1 visible, bit 2 show icon, bit 3 blend.
pub const FLAG_AMBIENT: u8 = 1;
pub const FLAG_VISIBLE: u8 = 2;
pub const FLAG_SHOW_ICON: u8 = 4;

/// The flags the datapack effects' codec defaults speak: non-ambient,
/// visible, show icon (`ambient=false, visible=true, show_icon=true`).
pub const DEFAULT_EFFECT_FLAGS: u8 = FLAG_VISIBLE | FLAG_SHOW_ICON;

/// `MobEffectInstance.INFINITE_DURATION` — the -1 sentinel; infinite
/// effects never tick down and never sort as "shorter".
pub const INFINITE_DURATION: i32 = -1;

/// `Attributes.MAX_ABSORPTION`'s default — the clamp ceiling
/// `LivingEntity.setAbsorptionAmount` reads (the attribute's own
/// base 20.0; no vanilla modifier ever moves it here).
pub const MAX_ABSORPTION: f32 = 20.0;

/// The absorption's per-level lift: `4 * (1 + amplifier)` — the
/// `AbsorptionMobEffect.onEffectStarted` constant, half-hearts per
/// amplifier level times two.
pub fn absorption_amount_for(amplifier: i32) -> f32 {
    (4 * (1 + amplifier)) as f32
}

/// THE HUNGER'S OWN FEED (javap `HungerMobEffect.applyEffectTick` pcs
/// 13-22 — the `0.005f * (amplifier + 1)` product the
/// causeFoodExhaustion call wears): the per-beat exhaustion the
/// every-tick law repeats (the shouldApplyEffectTickThisTick pcs 0-4
/// iconst_1 the period gate's absence speaks).
pub const HUNGER_EXHAUSTION_PER_TICK: f32 = 0.005;

/// THE RESISTANCE'S OWN FOLD (javap `LivingEntity
/// .getDamageAfterMagicAbsorb` pcs 32-73 — the fold the RESISTANCE
/// holder's own bytecode keeps): `i = amplifier + 1` (the iadd the
/// pcs 42-43 speak), `j = 25 - i * 5` (the imul/isub pair), the
/// survivor `max(damage * j / 25.0, 0.0)` (the fmul/fdiv/fconst_0/
/// Math.max chain). Amp 0 the ×0.8 wears, amp 3 the ×0.2, amp 4+ the
/// j-at-or-below-zero the max(0) clamps — the full immunity the
/// vanilla's own arithmetic speaks (the negative product the fdiv
/// carries, the fconst_0 the max lifts). The BYPASSES lanes the
/// CALLER's own verdict keeps (the starve the BYPASSES_EFFECTS
/// member, the generic_kill the BYPASSES_RESISTANCE one — the
/// pcs 0-29 early returns the fold never reaches).
pub fn resistance_scale(amplifier: i32, damage: f32) -> f32 {
    let j = 25 - (amplifier + 1) * 5;
    ((damage * j as f32) / 25.0).max(0.0)
}

/// THE FOLD'S OWN STORE READ (the caller's lane — the RESISTANCE
/// instance the body wears the amplifier the fold reads, the
/// unworn body the damage the identity passes): the same free fn
/// the player's and the mob's hurt bodies share, the one site the
/// 26.2's own single-method law keeps.
pub fn folded_by_resistance(store: &EffectsStore, damage: f32) -> f32 {
    match store.get(MOB_EFFECT_RESISTANCE) {
        Some(inst) => resistance_scale(inst.amplifier, damage),
        None => damage,
    }
}

/// THE WEAKNESS MODIFIER'S OWN AMOUNT (javap `MobEffects` clinit pcs
/// 469-503 — the WEAKNESS row the PLAIN `MobEffect(HARMFUL, 4738376)`
/// ctor keeps, no subclass the jar speaks — the addAttributeModifier
/// call the row's own attribute lane arms:
/// `(Attributes.ATTACK_DAMAGE, Identifier.withDefaultNamespace(
/// "effect.weakness"), -4.0d, AttributeModifier.Operation.ADD_VALUE)`).
/// The ldc2_w the constant's own word speaks.
pub const WEAKNESS_ATTACK_ADDEND: f64 = -4.0;

/// THE ATTACK-DAMAGE MODIFIER THE WEAKNESS WEARS (javap
/// `MobEffect$AttributeTemplate.create` pcs 0-24 — the
/// `amount * (amplifier + 1)` product the iload/iconst_1/iadd/i2d/dmul
/// walk keeps, the ADD_VALUE operation the modifier's own ctor pass
/// speaks): the store read the two melee dose sites share — the
/// unworn body the 0.0 identity rides, the amp-N body the
/// −4.0×(N+1) addend the attribute sum wears.
pub fn weakness_addend(store: &EffectsStore) -> f64 {
    match store.get(MOB_EFFECT_WEAKNESS) {
        Some(inst) => WEAKNESS_ATTACK_ADDEND * (inst.amplifier + 1) as f64,
        None => 0.0,
    }
}

/// THE STRENGTH MODIFIER'S OWN AMOUNT (javap `MobEffects` clinit pcs
/// 148-182 — the STRENGTH row the PLAIN `MobEffect(BENEFICIAL,
/// 16762624)` ctor keeps, no subclass the jar speaks — the
/// addAttributeModifier call the row's own attribute lane arms:
/// `(Attributes.ATTACK_DAMAGE, Identifier.withDefaultNamespace(
/// "effect.strength"), 3.0d, AttributeModifier.Operation.ADD_VALUE)`).
/// The ldc2_w the constant's own word speaks.
pub const STRENGTH_ATTACK_ADDEND: f64 = 3.0;

/// THE ATTACK-DAMAGE MODIFIER THE STRENGTH WEARS (the SAME
/// `MobEffect$AttributeTemplate.create` product law the WEAKNESS
/// pair's doc pins — the `amount * (amplifier + 1)` walk the
/// ADD_VALUE operation arms): the store read the two melee dose
/// sites share — the unworn body the 0.0 identity rides, the amp-N
/// body the +3.0×(N+1) addend the attribute sum wears. The pair the
/// RangedAttribute's 0.0 floor folds together: the strengthened
/// bare fist the 5.0 attribute carries, the weakened the zero the
/// floor keeps.
pub fn strength_addend(store: &EffectsStore) -> f64 {
    match store.get(MOB_EFFECT_STRENGTH) {
        Some(inst) => STRENGTH_ATTACK_ADDEND * (inst.amplifier + 1) as f64,
        None => 0.0,
    }
}

/// THE INSTANT WALK'S OWN SET (javap `MobEffects` clinit — the effects
/// the `InstantaneousMobEffect` subclasses carry: the `instant_health`
/// and `instant_damage` rows the `HealOrHarmMobEffect` ctor arms and
/// the `saturation` row the `SaturationMobEffect` (extends
/// `InstantaneousMobEffect`) banks; every OTHER row the plain
/// `MobEffect`/`RegenerationMobEffect`/`AbsorptionMobEffect` family
/// the base's `isInstantaneous` 0-ireturn keeps). The /effect give
/// walk's duration law reads it (the seconds the TICKS the instant
/// body speaks, the *20 fold every other body wears).
pub fn is_instantaneous(effect_id: i32) -> bool {
    effect_id == MOB_EFFECT_INSTANT_HEALTH
        || effect_id == MOB_EFFECT_INSTANT_DAMAGE
        || effect_id == MOB_EFFECT_SATURATION
}

/// THE UNDEAD'S OWN REFUSAL (javap `LivingEntity.canBeAffected` pcs
/// 52-87 — the `EntityTypeTags.IGNORES_POISON_AND_REGEN` member the
/// `REGENERATION` OR `POISON` instance the false return answers,
/// everything else the 1-ireturn accept wears): the two effects the
/// undead roster the /effect give walk refuses. The OFFLINE roster's
/// own undead the `Zombie`/`Skeleton` kinds speak — the tag's wider
/// roster (the wither, the phantom, the drowned) the offline kinds
/// the single pair the honest absence keeps.
pub fn undead_ignores(effect_id: i32) -> bool {
    effect_id == MOB_EFFECT_REGENERATION || effect_id == MOB_EFFECT_POISON
}

/// The effect-icons row's blink alpha (javap `Hud.extractEffects`'s
/// `endsWithin(200)` lane, verbatim): `clamp(duration * 0.01, 0, 0.5)
/// + cos(duration * PI / 5) * clamp((10 - duration/20) * 0.025, 0, 0.25)`
/// then `clamp(.., 0, 1)`. An effect outside its last 200 ticks (or an
/// infinite one) draws at full alpha.
pub fn icon_alpha(duration: i32) -> f32 {
    if duration == INFINITE_DURATION || duration > 200 {
        return 1.0;
    }
    let base = (duration as f32 * 0.01).clamp(0.0, 0.5);
    let wave = (10 - duration / 20) as f32 * 0.025;
    let osc = (duration as f32 * std::f32::consts::PI / 5.0).cos();
    (base + osc * wave.clamp(0.0, 0.25)).clamp(0.0, 1.0)
}

/// The effect icon's atlas sprite name (javap `Hud.getMobEffectSprite`:
/// the holder key with the `mob_effect/` prefix — the GUI atlas's own
/// directory source). The 40 names leak once, process-lifetime, the same
/// semantics the static tables carry.
pub fn effect_icon_sprite(effect_id: i32) -> Option<&'static str> {
    use std::sync::OnceLock;
    static TABLE: OnceLock<Vec<&'static str>> = OnceLock::new();
    let table = TABLE.get_or_init(|| {
        mc_registry::generated::mob_effects::MOB_EFFECT_ORDER
            .iter()
            .map(|n| {
                let path = n.trim_start_matches("minecraft:");
                let leaked: &'static str =
                    Box::leak(format!("mob_effect/{path}").into_boxed_str());
                leaked
            })
            .collect()
    });
    table.get(effect_id as usize).copied()
}

/// The mob-effect NAME → wire id walk (the generated order table IS the
/// registry; the foods table's payload rows carry the full identifiers).
pub fn effect_id_by_name(name: &str) -> Option<i32> {
    mc_registry::generated::mob_effects::MOB_EFFECT_ORDER
        .iter()
        .position(|n| *n == name)
        .map(|i| i as i32)
}

/// ONE active effect — the facts the wire / the eat arc delivered,
/// retained verbatim (the amplifier, the duration, the flags packing).
#[derive(Debug, Clone, PartialEq)]
pub struct EffectInstance {
    pub amplifier: i32,
    pub duration: i32,
    /// Bit 0 ambient, bit 1 visible, bit 2 show icon, bit 3 blend — the
    /// packet's packing order, retained raw.
    pub flags: u8,
    /// THE BLEND STATE (javap `MobEffectInstance$BlendState` — the
    /// instance's own pair the 26.2 class carries: `factor` the current
    /// 0..1 blend, `factorPreviousFrame` the pre-tick value the render's
    /// partial-tick lerp walks FROM). The fresh instance the ctor's
    /// zero pair keeps — the fade-in walks from 0 (the BlendState ctor's
    /// own JVM zero-init; the `setImmediate` full-sync path the wire
    /// lane never carries). The DARKNESS lightmap fold consumes it.
    pub blend_factor: f32,
    pub blend_prev: f32,
}

impl EffectInstance {
    pub fn is_ambient(&self) -> bool {
        self.flags & FLAG_AMBIENT != 0
    }

    #[expect(dead_code)] // the effect-row's own visibility word — the honest-skip lane keeps
    pub fn is_visible(&self) -> bool {
        self.flags & FLAG_VISIBLE != 0
    }

    /// `MobEffectInstance.showIcon` — the flag raw (the HUD's gate).
    pub fn show_icon(&self) -> bool {
        self.flags & FLAG_SHOW_ICON != 0
    }

    /// `isInfiniteDuration` — the -1 sentinel.
    pub fn is_infinite(&self) -> bool {
        self.duration == INFINITE_DURATION
    }

    /// `hasRemainingDuration` — infinite or strictly positive.
    pub fn has_remaining(&self) -> bool {
        self.is_infinite() || self.duration > 0
    }

    /// `endsWithin(t)` — `!infinite && duration <= t`.
    pub fn ends_within(&self, t: i32) -> bool {
        !self.is_infinite() && self.duration <= t
    }

    /// THE BLEND STATE'S OWN TICK (javap `BlendState.tick` pc 0-109 —
    /// the walk the instance's `tickClient` tail serves AFTER the
    /// duration's decrement (javap tickClient pc 16-21 — the blend
    /// consults the POST-decrement duration)): the head the
    /// `factorPreviousFrame = factor` copy keeps, the target the
    /// `hasEffect` verdict answers (`!endsWithin(blendOutAdvanceTicks)`
    /// — the padding flips the target the ADVANCE ticks before the
    /// expiry), the equal-value early return the `fcmpl` pair keeps,
    /// the step the `1.0F / blendDurationTicks` fold speaks — the
    /// `Mth.clamp(target - factor, -step, step)` accumulate the same
    /// f32 arithmetic wears, the ZERO blend ticks the snap keeps.
    fn tick_blend(&mut self, effect_id: i32) {
        self.blend_prev = self.blend_factor;
        let (in_ticks, out_ticks, advance) = blend_durations(effect_id);
        let has = !self.ends_within(advance);
        let target = if has { 1.0_f32 } else { 0.0_f32 };
        if self.blend_factor == target {
            return;
        }
        let blend_ticks = if has { in_ticks } else { out_ticks };
        if blend_ticks == 0 {
            self.blend_factor = target;
        } else {
            let step = 1.0_f32 / blend_ticks as f32;
            self.blend_factor += (target - self.blend_factor).clamp(-step, step);
        }
    }

    /// `isShorterDurationThan(other)` — `!infinite && (duration <
    /// other.duration || other.infinite)`.
    fn is_shorter_duration_than(&self, other: &EffectInstance) -> bool {
        !self.is_infinite()
            && (self.duration < other.duration || other.is_infinite())
    }

    /// `MobEffectInstance.update(new)` — the upgrade law, the
    /// hidden-effect chain the documented absence (a `false` verdict
    /// keeps the standing instance; `true` replaces it with `new`):
    /// - a strictly greater amplifier replaces,
    /// - an equal amplifier replaces when the standing one is the
    ///   shorter (the longer duration refreshes),
    /// - everything else keeps.
    fn update_from(&mut self, new: &EffectInstance) -> bool {
        if new.amplifier > self.amplifier {
            if new.is_shorter_duration_than(self) {
                // the vanilla stashes the standing instance into
                // `hiddenEffect` for the expiry swap — the documented
                // absence (see the module head)
            }
            *self = new.clone();
            return true;
        }
        if new.amplifier == self.amplifier && self.is_shorter_duration_than(new) {
            // the longer duration wins at equal amplifier; the vanilla's
            // shorter-wins branch only feeds the hidden chain — absence
            *self = new.clone();
            return true;
        }
        false
    }
}

/// The store: the LOCAL player's active effects keyed by the mob-effect
/// registry id, plus the absorption value the white hearts row speaks.
/// The BTreeMap keeps the tick deterministic (the id order, the same
/// walk every time).
#[derive(Debug, Default, Clone)]
pub struct EffectsStore {
    effects: BTreeMap<i32, EffectInstance>,
    absorption: f32,
}

/// The offline tick's outcome: the heals the regeneration beat wanted
/// (1.0 each — the caller folds them through the `Entity.heal` clamp),
/// the magic doses the instant-harm beat wanted (the `hurtServer`
/// pipeline the caller runs), the saturation's food lifts (the
/// `FoodData.eat(amplifier + 1, 1.0)` doses — the caller folds them
/// through the food walk), the POISON/WITHER beats the periodic laws
/// landed (the 1.0 `hurtServer` doses the CALLER's own health the
/// poison's `> 1.0F` gate reads — the store the beat alone banks) and
/// the ids that expired this tick (the removal the server's
/// `tickServer` false return speaks).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct OfflineTick {
    pub heals: Vec<f32>,
    pub harms: Vec<f32>,
    pub eats: Vec<i32>,
    pub poisons: Vec<f32>,
    pub withers: Vec<f32>,
    /// The hunger's own per-beat exhaustion feeds — the values the
    /// caller's food_data bank spends (the Player walk the vanilla
    /// instance-check serves; the mobs the dose never reaches).
    pub exhaustions: Vec<f32>,
    pub expired: Vec<i32>,
}

impl EffectsStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    #[cfg_attr(not(test), expect(dead_code))]
    pub fn get(&self, effect_id: i32) -> Option<&EffectInstance> {
        self.effects.get(&effect_id)
    }

    pub fn has_effect(&self, effect_id: i32) -> bool {
        self.effects.contains_key(&effect_id)
    }

    /// The absorption value (the raw field — `LivingEntity
    /// .getAbsorptionAmount` is the plain read).
    pub fn absorption(&self) -> f32 {
        self.absorption
    }

    /// `LivingEntity.setAbsorptionAmount` — the clamp to
    /// `[0, MAX_ABSORPTION]` rides.
    pub fn set_absorption(&mut self, value: f32) {
        self.absorption = value.clamp(0.0, MAX_ABSORPTION);
    }

    /// `LivingEntity.addEffect` (the entity-arg overload's verdicts —
    /// `canBeAffected` is a player's unconditional true): insert-or-
    /// update per `MobEffectInstance.update`, and the absorption's
    /// `onEffectStarted` lift on EVERY application (the bytecode's
    /// unconditional tail). Returns whether the store changed.
    pub fn add_effect(&mut self, effect_id: i32, amplifier: i32, duration: i32, flags: u8) -> bool {
        // THE FRESH INSTANCE'S OWN ZERO PAIR (the BlendState ctor's JVM
        // zero-init — the fade-in walks from 0, the setImmediate
        // full-sync path the wire lane never carries)
        let new = EffectInstance {
            amplifier,
            duration,
            flags,
            blend_factor: 0.0,
            blend_prev: 0.0,
        };
        let changed = match self.effects.get_mut(&effect_id) {
            None => {
                self.effects.insert(effect_id, new);
                true
            }
            Some(current) => current.update_from(&new),
        };
        // the onEffectStarted dispatch rides BOTH branches (the
        // bytecode's tail call): the absorption's value lifts to
        // max(current, 4 * (1 + amp)) — the new instance's amplifier.
        if effect_id == MOB_EFFECT_ABSORPTION {
            let target = absorption_amount_for(amplifier);
            if target > self.absorption {
                self.set_absorption(target);
            }
        }
        changed
    }

    /// `LivingEntity.removeEffect` — the map's removal; the absorption
    /// value persists (no removal hook — the module head's law). The
    /// /effect clear walk the offline dispatch runs the non-test
    /// caller the lane keeps.
    pub fn remove_effect(&mut self, effect_id: i32) -> bool {
        self.effects.remove(&effect_id).is_some()
    }

    /// `LivingEntity.removeAllEffects` — the milk's
    /// `ClearAllStatusEffects` lane.
    pub fn remove_all(&mut self) -> bool {
        let changed = !self.effects.is_empty();
        self.effects.clear();
        changed
    }

    /// THE OFFLINE STAND-IN SERVER'S TICK (the
    /// `MobEffectInstance.tickServer` arc per instance, in id order):
    /// an expired effect removes; a live one fires the effect beat —
    /// the regeneration's `shouldApplyEffectTickThisTick` law
    /// (`50 >> amplifier` period against the pre-tick-down duration,
    /// heal 1.0 through the caller) — then the duration ticks down and
    /// a zero hands the effect to the removal (the same-tick expiry the
    /// `tickServer` false return speaks). The -1 sentinel never ticks
    /// and never expires.
    pub fn tick_offline(&mut self) -> OfflineTick {
        let mut out = OfflineTick::default();
        let ids: Vec<i32> = self.effects.keys().copied().collect();
        for id in ids {
            let expired = {
                let inst = self.effects.get_mut(&id).expect("key just read");
                if !inst.has_remaining() {
                    true
                } else {
                    if id == MOB_EFFECT_REGENERATION {
                        let period = 50 >> inst.amplifier;
                        let due = if period > 0 { inst.duration % period == 0 } else { true };
                        if due {
                            out.heals.push(1.0);
                        }
                    }
                    // THE POISON'S OWN BEAT (javap PoisonMobEffect
                    // shouldApplyEffectTickThisTick pcs 0-22 — the
                    // `25 >> amplifier` period the PRE-tick-down
                    // duration the remainder law speaks, the zero
                    // period the `k > 0` false arm answers true) —
                    // the beat ALONE the store banks, the
                    // `hurtServer(magic, 1.0F)` dose the health gate
                    // the caller's own body reads (javap
                    // applyEffectTick pcs 0-24 — the `getHealth()
                    // > 1.0F` fcmpl word the half-heart floor speaks,
                    // the kill the dose refuses).
                    if id == MOB_EFFECT_POISON {
                        let period = 25 >> inst.amplifier;
                        let due = if period > 0 { inst.duration % period == 0 } else { true };
                        if due {
                            out.poisons.push(1.0);
                        }
                    }
                    // THE WITHER'S OWN BEAT (javap WitherMobEffect
                    // shouldApplyEffectTickThisTick pcs 0-22 — the
                    // `40 >> amplifier` period the same remainder
                    // law speaks) — the `hurtServer(wither, 1.0F)`
                    // dose the health gate NEVER wears (javap
                    // applyEffectTick pcs 0-15 — the hurtServer call
                    // the getHealth read the bytecode never makes,
                    // the kill the dose lands).
                    if id == MOB_EFFECT_WITHER {
                        let period = 40 >> inst.amplifier;
                        let due = if period > 0 { inst.duration % period == 0 } else { true };
                        if due {
                            out.withers.push(1.0);
                        }
                    }
                    // THE HUNGER'S OWN BEAT (javap HungerMobEffect —
                    // the shouldApplyEffectTickThisTick pcs 0-4 the
                    // iconst_1 EVERY-TICK law speaks, the applyEffectTick
                    // pcs 1-24 the Player instance check wears: the
                    // `causeFoodExhaustion(0.005f * (amp + 1))` dose the
                    // exhaustion bank rides, the caller's own food_data
                    // the Player walk alone serves, the mobs the dose
                    // never reaches)
                    if id == MOB_EFFECT_HUNGER {
                        out.exhaustions
                            .push(HUNGER_EXHAUSTION_PER_TICK * (inst.amplifier + 1) as f32);
                    }
                    // THE INSTANT WALK (javap `InstantaneousMobEffect
                    // .shouldApplyEffectTickThisTick` pcs 0-11 — the
                    // `duration >= 1` word the every-remaining-tick
                    // law answers; the applyEffectTick doses the
                    // HealOrHarm/Saturation bodies carry: the heal
                    // `4 << amp` (the Java shift the mask-31
                    // wrapping_shl law speaks), the harm `6 << amp`
                    // the magic source wears, the eat
                    // `amplifier + 1` the FoodData dose speaks). The
                    // DEFAULT 1-tick duration the /effect give walk
                    // arms the ONE application the same law keeps.
                    if is_instantaneous(id) {
                        let amp = inst.amplifier;
                        if id == MOB_EFFECT_INSTANT_HEALTH {
                            out.heals.push((4i32).wrapping_shl(amp as u32).max(0) as f32);
                        }
                        if id == MOB_EFFECT_INSTANT_DAMAGE {
                            out.harms.push((6i32).wrapping_shl(amp as u32) as f32);
                        }
                        if id == MOB_EFFECT_SATURATION {
                            out.eats.push(1 + amp);
                        }
                    }
                    if !inst.is_infinite() {
                        inst.duration -= 1;
                    }
                    // THE BLEND'S OWN TICK (javap tickClient pc 16-21 —
                    // the blend consults the POST-decrement duration;
                    // the expired instance's last tick the padding's
                    // own zero factor keeps — the removal's snap the
                    // fade-out already made invisible)
                    inst.tick_blend(id);
                    !inst.has_remaining()
                }
            };
            if expired {
                self.effects.remove(&id);
                out.expired.push(id);
            }
        }
        out
    }

    /// THE CLIENT COPY'S TICK (the networked lane's `tickClient`): the
    /// durations tick down, nothing ever removes here (the wire's next
    /// Update/Remove is the truth the model's own store mirrors).
    pub fn tick_client(&mut self) {
        for (&id, inst) in self.effects.iter_mut() {
            if inst.has_remaining() && !inst.is_infinite() {
                inst.duration -= 1;
            }
            // THE BLEND'S OWN TICK (javap tickClient pc 16-21 — the
            // walk's own tail, the decrement's POST state the blend
            // consults; the expired-and-not-yet-removed instance the
            // fade-out's own tail keeps walking)
            inst.tick_blend(id);
        }
    }

    /// THE BLEND FACTOR'S OWN READ (javap `LivingEntity.getEffectBlend
    /// Factor` pc 0-17 — the `getEffect(holder)` instance the blend's
    /// own factor answers, the ABSENT effect the 0.0 the null arm
    /// keeps; the factor itself the `BlendState.getFactor` walk serves
    /// — javap pc 15-27: `Mth.lerp(partialTick, factorPreviousFrame,
    /// factor)` the render's interpolation folds, the tick-aligned
    /// full-tick partial the POST-tick factor wears).
    pub fn get_blend_factor(&self, effect_id: i32, partial: f32) -> f32 {
        match self.effects.get(&effect_id) {
            None => 0.0,
            Some(inst) => inst.blend_prev + (inst.blend_factor - inst.blend_prev) * partial,
        }
    }

    /// THE WIRE'S MIRROR (the networked lane's sync — the model's
    /// own-effect store is the packet-filtered truth): every carried
    /// state rides `add_effect` (the update law corrects the local
    /// copy), every locally-known id the snapshot dropped removes.
    pub fn sync_from_wire(&mut self, snapshot: &[(i32, MobEffectState)]) {
        for (id, state) in snapshot {
            self.add_effect(*id, state.amplifier, state.duration, state.flags);
        }
        let carried: Vec<i32> = snapshot.iter().map(|(id, _)| *id).collect();
        let stale: Vec<i32> = self
            .effects
            .keys()
            .copied()
            .filter(|id| !carried.contains(id))
            .collect();
        for id in stale {
            self.effects.remove(&id);
        }
    }

    /// THE HUD'S ICON ORDER (javap `Hud.extractEffects`:
    /// `Ordering.natural().reverse().sortedCopy(getActiveEffects())` —
    /// the reverse of `MobEffectInstance.compareTo`):
    /// - the SIMPLE branch (both durations beyond 32147, or both
    ///   ambient) compares the ambient booleans then the colors,
    /// - the FULL branch compares non-ambient first, finite first,
    ///   shorter duration first, color first;
    /// reversed for the row. The colors ride the generated
    /// `MOB_EFFECT_META` table (the dump's `getColor` column).
    pub fn sorted_for_hud(&self) -> Vec<(i32, EffectInstance)> {
        let color_of = |id: i32| -> i32 {
            mc_registry::generated::mob_effects::MOB_EFFECT_META
                .get(id as usize)
                .map(|m| m.1)
                .unwrap_or(0)
        };
        /// `MobEffectInstance.compareTo` — the natural order (the ids
        /// carry each instance's color; the branch structure the
        /// bytecode speaks).
        fn natural_cmp(
            a: &EffectInstance,
            b: &EffectInstance,
            color_a: i32,
            color_b: i32,
        ) -> std::cmp::Ordering {
            use std::cmp::Ordering;
            let simple = (a.duration > 32147 && b.duration > 32147)
                || (a.is_ambient() && b.is_ambient());
            if simple {
                return a
                    .is_ambient()
                    .cmp(&b.is_ambient())
                    .then_with(|| color_a.cmp(&color_b));
            }
            // the FULL chain: compareFalseFirst(ambient), then
            // compareFalseFirst(infinite), then duration, then color
            let amb = match (a.is_ambient(), b.is_ambient()) {
                (false, true) => Ordering::Less,
                (true, false) => Ordering::Greater,
                _ => Ordering::Equal,
            };
            let inf = match (a.is_infinite(), b.is_infinite()) {
                (false, true) => Ordering::Less,
                (true, false) => Ordering::Greater,
                _ => Ordering::Equal,
            };
            amb.then_with(|| inf)
                .then_with(|| a.duration.cmp(&b.duration))
                .then_with(|| color_a.cmp(&color_b))
        }
        let mut rows: Vec<(i32, EffectInstance)> = self
            .effects
            .iter()
            .map(|(id, inst)| (*id, inst.clone()))
            .collect();
        rows.sort_by(|a, b| natural_cmp(&a.1, &b.1, color_of(a.0), color_of(b.0)));
        // THE REVERSE (the extractEffects' own
        // `Ordering.natural().reverse()` — the row grows leftward from
        // the right edge in the reversed natural order)
        rows.reverse();
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The generated table's slots the constants pin (the DumpMobEffects
    /// boot's own verdict — the 26.2 registration order).
    #[test]
    fn the_wire_ids_match_the_registry_order() {
        assert_eq!(
            mc_registry::generated::mob_effects::MOB_EFFECT_ORDER
                [MOB_EFFECT_REGENERATION as usize],
            "minecraft:regeneration"
        );
        assert_eq!(
            mc_registry::generated::mob_effects::MOB_EFFECT_ORDER
                [MOB_EFFECT_ABSORPTION as usize],
            "minecraft:absorption"
        );
        // the meta's own columns: both beneficial (the HUD's top row)
        assert!(mc_registry::generated::mob_effects::MOB_EFFECT_META
            [MOB_EFFECT_REGENERATION as usize]
            .0);
        assert!(mc_registry::generated::mob_effects::MOB_EFFECT_META
            [MOB_EFFECT_ABSORPTION as usize]
            .0);
    }

    #[test]
    fn the_absorption_lifts_to_four_per_level_on_apply() {
        let mut s = EffectsStore::new();
        assert!(s.add_effect(MOB_EFFECT_ABSORPTION, 0, 2400, DEFAULT_EFFECT_FLAGS));
        assert_eq!(s.absorption(), 4.0);
        // the SAME amplifier again: max(4, 4) — no lift, the refresh
        assert!(!s.add_effect(MOB_EFFECT_ABSORPTION, 0, 2400, DEFAULT_EFFECT_FLAGS));
        assert_eq!(s.absorption(), 4.0);
        // the amplifier upgrades: the max rides to 8
        assert!(s.add_effect(MOB_EFFECT_ABSORPTION, 1, 2400, DEFAULT_EFFECT_FLAGS));
        assert_eq!(s.absorption(), 8.0);
        // a downgrade keeps the standing value (only a max lifts)
        assert!(!s.add_effect(MOB_EFFECT_ABSORPTION, 0, 100, DEFAULT_EFFECT_FLAGS));
        assert_eq!(s.absorption(), 8.0);
    }

    #[test]
    fn the_absorption_clamps_and_survives_removal() {
        let mut s = EffectsStore::new();
        s.set_absorption(25.0);
        assert_eq!(s.absorption(), MAX_ABSORPTION); // the 20.0 ceiling
        s.set_absorption(-3.0);
        assert_eq!(s.absorption(), 0.0);
        // the removal hook's absence: the value persists
        s.set_absorption(8.0);
        s.add_effect(MOB_EFFECT_ABSORPTION, 0, 2400, DEFAULT_EFFECT_FLAGS);
        assert!(s.remove_effect(MOB_EFFECT_ABSORPTION));
        assert_eq!(s.absorption(), 8.0);
        // the second removal: the effect's already gone
        assert!(!s.remove_effect(MOB_EFFECT_ABSORPTION));
    }

    #[test]
    fn the_update_law_speaks_the_comparator() {
        let mut s = EffectsStore::new();
        // the standing 100-tick amp-0
        s.add_effect(50, 0, 100, DEFAULT_EFFECT_FLAGS);
        // the same amp, longer: the refresh wins
        assert!(s.add_effect(50, 0, 300, DEFAULT_EFFECT_FLAGS));
        assert_eq!(s.get(50).unwrap().duration, 300);
        // the same amp, shorter: the standing instance keeps
        assert!(!s.add_effect(50, 0, 50, DEFAULT_EFFECT_FLAGS));
        assert_eq!(s.get(50).unwrap().duration, 300);
        // the greater amp: replaces even when shorter
        assert!(s.add_effect(50, 1, 40, DEFAULT_EFFECT_FLAGS));
        assert_eq!(s.get(50).unwrap().amplifier, 1);
        // the lesser amp: never replaces
        assert!(!s.add_effect(50, 0, 99999, DEFAULT_EFFECT_FLAGS));
        assert_eq!(s.get(50).unwrap().amplifier, 1);
        // the infinite beats any finite at equal amp
        s.add_effect(51, 0, 10, DEFAULT_EFFECT_FLAGS);
        assert!(s.add_effect(51, 0, INFINITE_DURATION, DEFAULT_EFFECT_FLAGS));
        assert!(s.get(51).unwrap().is_infinite());
        // the finite cannot replace an infinite at equal amp
        assert!(!s.add_effect(51, 0, 500, DEFAULT_EFFECT_FLAGS));
        assert!(s.get(51).unwrap().is_infinite());
    }

    #[test]
    fn the_offline_tick_walks_the_regen_beat() {
        let mut s = EffectsStore::new();
        // the golden apple's own payload: 100t Regeneration II —
        // the 25-tick period beats at 100, 75, 50, 25
        s.add_effect(MOB_EFFECT_REGENERATION, 1, 100, DEFAULT_EFFECT_FLAGS);
        let mut heals = 0;
        let mut expired_at = None;
        for t in 0..200 {
            let tick = s.tick_offline();
            heals += tick.heals.len();
            if !tick.expired.is_empty() {
                expired_at = Some(t);
                break;
            }
        }
        assert_eq!(heals, 4, "four beats over the 100-tick span");
        assert_eq!(expired_at, Some(99), "the zero removes the same tick");
        assert!(s.is_empty());
    }

    /// THE POISON'S OWN BEAT THE 25-PERIOD WEARS (javap
    /// PoisonMobEffect.shouldApplyEffectTickThisTick pcs 0-22 — the
    /// `25 >> amplifier` period the PRE-tick-down duration the
    /// remainder speaks): the amp-0 body the 100-tick span the
    /// 100/75/50/25 multiples beat, the beats the store banks, the
    /// caller's health gate the doses alone answer.
    #[test]
    fn the_poison_beats_the_25_period() {
        let mut s = EffectsStore::new();
        s.add_effect(MOB_EFFECT_POISON, 0, 100, DEFAULT_EFFECT_FLAGS);
        let mut beats = 0;
        let mut expired_at = None;
        for t in 0..200 {
            let tick = s.tick_offline();
            beats += tick.poisons.len();
            assert!(
                tick.withers.is_empty(),
                "the poison row the wither lane never fills"
            );
            if !tick.expired.is_empty() {
                expired_at = Some(t);
                break;
            }
        }
        assert_eq!(beats, 4, "the 25-multiples the four beats wear");
        assert_eq!(expired_at, Some(99), "the zero removes the same tick");
        assert!(s.is_empty());
    }

    /// THE WITHER'S OWN BEAT THE 40-PERIOD WEARS (javap
    /// WitherMobEffect.shouldApplyEffectTickThisTick pcs 0-22 — the
    /// `40 >> amplifier` period the same remainder law speaks): the
    /// amp-0 body the 100-tick span the 80/40 multiples beat, the
    /// 25-law the 40-period never rides.
    #[test]
    fn the_wither_beats_the_40_period() {
        let mut s = EffectsStore::new();
        s.add_effect(MOB_EFFECT_WITHER, 0, 100, DEFAULT_EFFECT_FLAGS);
        let mut beats = 0;
        let mut expired_at = None;
        for t in 0..200 {
            let tick = s.tick_offline();
            beats += tick.withers.len();
            assert!(
                tick.poisons.is_empty(),
                "the wither row the poison lane never fills"
            );
            if !tick.expired.is_empty() {
                expired_at = Some(t);
                break;
            }
        }
        assert_eq!(beats, 2, "the 40-multiples the two beats wear");
        assert_eq!(expired_at, Some(99), "the zero removes the same tick");
        assert!(s.is_empty());
    }

    /// THE PERIOD'S OWN SHIFT AND ZERO (the `>> amplifier` ishr the
    /// two laws speak): the amp-1 poison the 12-period wears, the
    /// amp-6 poison the `25 >> 6` ZERO the `k > 0` false arm the
    /// every-tick verdict answers, the silent rows the lanes the
    /// empty law keeps.
    #[test]
    fn the_period_the_amplifier_shifts_and_the_zero_speaks() {
        // the amp-1 poison: the 12-period the 24-tick span the
        // 24/12 multiples beat
        let mut s = EffectsStore::new();
        s.add_effect(MOB_EFFECT_POISON, 1, 24, DEFAULT_EFFECT_FLAGS);
        let mut beats = 0;
        for _ in 0..24 {
            beats += s.tick_offline().poisons.len();
        }
        assert_eq!(beats, 2, "the 12-period the two beats wear");
        // the amp-6 poison: the ZERO period the every-tick verdict
        // wears (25 >> 6 = 0 — the `k > 0` branch the false arm keeps)
        let mut s2 = EffectsStore::new();
        s2.add_effect(MOB_EFFECT_POISON, 6, 5, DEFAULT_EFFECT_FLAGS);
        let mut beats2 = 0;
        for _ in 0..5 {
            beats2 += s2.tick_offline().poisons.len();
        }
        assert_eq!(beats2, 5, "the zero period the every-tick beat wears");
        // the silent rows: the absorption the lanes never fills
        let mut s3 = EffectsStore::new();
        s3.add_effect(MOB_EFFECT_ABSORPTION, 0, 10, DEFAULT_EFFECT_FLAGS);
        for _ in 0..10 {
            let tick = s3.tick_offline();
            assert!(tick.poisons.is_empty() && tick.withers.is_empty());
        }
        assert!(s3.is_empty());
    }

    /// THE RESISTANCE'S OWN LADDER (javap getDamageAfterMagicAbsorb
    /// pcs 32-73 — the j = 25 − 5(amp+1) law the fold wears): the amp
    /// walk the ×0.8/×0.6/×0.4/×0.2 survivors speak, the amp-4 the
    /// j-zero zero wears, the amp-5 the negative product the max(0)
    /// clamp lifts — the 10.0 dose the exact f32 speakers keep (the
    /// 20/15/10/5 products the /25 the exact decimals wear).
    #[test]
    fn the_resistance_the_25_law_the_ladder_wears() {
        assert_eq!(resistance_scale(0, 10.0), 8.0, "the amp-0 the x0.8 wears");
        assert_eq!(resistance_scale(1, 10.0), 6.0, "the amp-1 the x0.6 wears");
        assert_eq!(resistance_scale(2, 10.0), 4.0, "the amp-2 the x0.4 wears");
        assert_eq!(resistance_scale(3, 10.0), 2.0, "the amp-3 the x0.2 wears");
        assert_eq!(
            resistance_scale(4, 10.0),
            0.0,
            "the amp-4 the j-zero zero wears"
        );
        assert_eq!(
            resistance_scale(5, 10.0),
            0.0,
            "the amp-5 the negative product the max(0) lifts"
        );
        // the store read: the worn body the fold wears, the bare body
        // the identity passes
        let mut s = EffectsStore::new();
        assert_eq!(folded_by_resistance(&s, 10.0), 10.0);
        s.add_effect(MOB_EFFECT_RESISTANCE, 2, 400, DEFAULT_EFFECT_FLAGS);
        assert_eq!(folded_by_resistance(&s, 10.0), 4.0);
        // the expiry the identity restores (the clock's own walk)
        for _ in 0..400 {
            s.tick_offline();
        }
        assert_eq!(
            folded_by_resistance(&s, 10.0),
            10.0,
            "the expired brand the fold never reads"
        );
    }

    /// THE WEAKNESS'S OWN ADDEND (javap MobEffects clinit pcs 469-503 —
    /// the addAttributeModifier(ATTACK_DAMAGE, "effect.weakness",
    /// -4.0d, ADD_VALUE) row the plain MobEffect ctor keeps + the
    /// MobEffect$AttributeTemplate.create(int) pcs 8-16 the
    /// amount × (amplifier + 1) product speaks): the amp ladder the
    /// exact f64s wear, the unworn body the 0.0 identity rides, the
    /// expiry the identity restores (the clock's own walk).
    #[test]
    fn the_weakness_the_addend_ladder_the_attribute_wears() {
        let mut s = EffectsStore::new();
        assert_eq!(weakness_addend(&s), 0.0, "the bare body the 0.0 rides");
        s.add_effect(MOB_EFFECT_WEAKNESS, 0, 400, DEFAULT_EFFECT_FLAGS);
        assert_eq!(
            weakness_addend(&s),
            -4.0,
            "the amp-0 the amount x1 the exact f64 wears"
        );
        let mut s2 = EffectsStore::new();
        s2.add_effect(MOB_EFFECT_WEAKNESS, 1, 400, DEFAULT_EFFECT_FLAGS);
        assert_eq!(
            weakness_addend(&s2),
            -8.0,
            "the amp-1 the amount x2 the exact f64 wears"
        );
        let mut s3 = EffectsStore::new();
        s3.add_effect(MOB_EFFECT_WEAKNESS, 3, 400, DEFAULT_EFFECT_FLAGS);
        assert_eq!(
            weakness_addend(&s3),
            -16.0,
            "the amp-3 the amount x4 the exact f64 wears"
        );
        // the expiry the identity restores (the clock's own walk)
        for _ in 0..400 {
            s3.tick_offline();
        }
        assert_eq!(
            weakness_addend(&s3),
            0.0,
            "the expired brand the 0.0 identity rides"
        );
    }

    /// THE STRENGTH'S OWN LADDER (javap `MobEffects` clinit pcs
    /// 148-182 — the addAttributeModifier(ATTACK_DAMAGE,
    /// "effect.strength", 3.0d, ADD_VALUE) row the plain
    /// MobEffect(BENEFICIAL, 16762624) ctor keeps + the
    /// MobEffect$AttributeTemplate.create(int) pcs 8-16 the
    /// amount × (amplifier + 1) product speaks): the amp ladder the
    /// exact f64s wear, the unworn body the 0.0 identity rides, the
    /// expiry the identity restores (the clock's own walk), the
    /// WEAKNESS pair the sum folds (the +3.0 the −4.0 the 2.0 base
    /// the 5.0/1.0 attribute the two stores' own brands wear).
    #[test]
    fn the_strength_the_addend_ladder_the_attribute_wears() {
        let mut s = EffectsStore::new();
        assert_eq!(strength_addend(&s), 0.0, "the bare body the 0.0 rides");
        s.add_effect(MOB_EFFECT_STRENGTH, 0, 400, DEFAULT_EFFECT_FLAGS);
        assert_eq!(
            strength_addend(&s),
            3.0,
            "the amp-0 the amount x1 the exact f64 wears"
        );
        let mut s2 = EffectsStore::new();
        s2.add_effect(MOB_EFFECT_STRENGTH, 1, 400, DEFAULT_EFFECT_FLAGS);
        assert_eq!(
            strength_addend(&s2),
            6.0,
            "the amp-1 the amount x2 the exact f64 wears"
        );
        let mut s3 = EffectsStore::new();
        s3.add_effect(MOB_EFFECT_STRENGTH, 3, 400, DEFAULT_EFFECT_FLAGS);
        assert_eq!(
            strength_addend(&s3),
            12.0,
            "the amp-3 the amount x4 the exact f64 wears"
        );
        // the expiry the identity restores (the clock's own walk)
        for _ in 0..400 {
            s3.tick_offline();
        }
        assert_eq!(
            strength_addend(&s3),
            0.0,
            "the expired brand the 0.0 identity rides"
        );
        // the PAIR the sum folds (the attribute law the two melee
        // sites the same sum wears): the strength-0 fist the 5.0
        // the base+addend wears, the strength-0 weakness-0 the 1.0
        // the cancelling pair keeps
        let mut s4 = EffectsStore::new();
        s4.add_effect(MOB_EFFECT_STRENGTH, 0, 400, DEFAULT_EFFECT_FLAGS);
        assert_eq!(
            2.0 + strength_addend(&s4),
            5.0,
            "the strengthened fist the 5.0 attribute carries"
        );
        s4.add_effect(MOB_EFFECT_WEAKNESS, 0, 400, DEFAULT_EFFECT_FLAGS);
        assert_eq!(
            2.0 + strength_addend(&s4) + weakness_addend(&s4),
            1.0,
            "the cancelling pair the 1.0 attribute keeps"
        );
    }

    /// THE HUNGER'S OWN BEAT (javap HungerMobEffect — the
    /// shouldApplyEffectTickThisTick pcs 0-4 the iconst_1 EVERY-TICK
    /// law speaks, the applyEffectTick the 0.005 × (amp+1) feed the
    /// exhaustion bank rides): the 25-tick window the 25 feeds speak
    /// (the poison's 25-period the contrast the one beat wears), the
    /// amp-2 the tripled feed wears, the expiry the silence keeps,
    /// the non-hunger rows the lanes never fill.
    #[test]
    fn the_hunger_the_every_tick_beat_the_bank_wears() {
        // THE EVERY-TICK LAW (the 25 ticks the 25 feeds speak)
        let mut s = EffectsStore::new();
        s.add_effect(MOB_EFFECT_HUNGER, 0, 25, DEFAULT_EFFECT_FLAGS);
        let mut feeds = 0;
        for _ in 0..25 {
            let tick = s.tick_offline();
            assert_eq!(
                tick.exhaustions,
                vec![HUNGER_EXHAUSTION_PER_TICK],
                "the every-tick beat the amp-0 feed wears"
            );
            feeds += 1;
        }
        assert_eq!(feeds, 25, "the 25-tick window the 25 beats speak");
        assert!(
            s.get(MOB_EFFECT_HUNGER).is_none(),
            "the 25th tick the expiry spends"
        );
        // the silence the expiry keeps
        for _ in 0..5 {
            assert!(s.tick_offline().exhaustions.is_empty());
        }
        // THE AMP-2 LIFT (the tripled feed the ×(amp+1) product wears)
        let mut s2 = EffectsStore::new();
        s2.add_effect(MOB_EFFECT_HUNGER, 2, 10, DEFAULT_EFFECT_FLAGS);
        for _ in 0..10 {
            let tick = s2.tick_offline();
            assert_eq!(
                tick.exhaustions,
                vec![HUNGER_EXHAUSTION_PER_TICK * 3.0],
                "the amp-2 the tripled feed wears"
            );
        }
        // THE SILENT ROWS' CONTRAST (the hunger the EVERY-tick feed
        // keeps, the poison the 25-multiple beat alone speaks — the
        // two lanes the same window's own rhythms wear)
        let mut s3 = EffectsStore::new();
        s3.add_effect(MOB_EFFECT_POISON, 0, 30, DEFAULT_EFFECT_FLAGS);
        s3.add_effect(MOB_EFFECT_HUNGER, 0, 30, DEFAULT_EFFECT_FLAGS);
        let mut poison_beats = 0;
        for _ in 0..30 {
            let tick = s3.tick_offline();
            assert_eq!(
                tick.exhaustions,
                vec![HUNGER_EXHAUSTION_PER_TICK],
                "the hunger beat the every-tick law keeps"
            );
            poison_beats += tick.poisons.len();
        }
        assert_eq!(
            poison_beats, 1,
            "the poison the 25-multiple beat alone speaks"
        );
    }

    #[test]
    fn the_offline_tick_holds_the_infinite_sentinel() {
        let mut s = EffectsStore::new();
        s.add_effect(MOB_EFFECT_ABSORPTION, 0, INFINITE_DURATION, DEFAULT_EFFECT_FLAGS);
        for _ in 0..5000 {
            let tick = s.tick_offline();
            assert!(tick.expired.is_empty());
            assert!(tick.heals.is_empty());
        }
        assert_eq!(s.get(MOB_EFFECT_ABSORPTION).unwrap().duration, INFINITE_DURATION);
    }

    #[test]
    fn the_non_regen_effects_never_heal() {
        let mut s = EffectsStore::new();
        s.add_effect(MOB_EFFECT_ABSORPTION, 0, 10, DEFAULT_EFFECT_FLAGS);
        for _ in 0..10 {
            let tick = s.tick_offline();
            assert!(tick.heals.is_empty());
        }
        assert!(s.is_empty());
    }

    #[test]
    fn the_client_tick_never_removes() {
        let mut s = EffectsStore::new();
        s.add_effect(MOB_EFFECT_REGENERATION, 0, 3, DEFAULT_EFFECT_FLAGS);
        for _ in 0..100 {
            s.tick_client();
        }
        // the wire's Remove is the truth — the copy just floors at zero
        assert_eq!(s.get(MOB_EFFECT_REGENERATION).unwrap().duration, 0);
    }

    #[test]
    fn the_wire_mirror_lands_and_drops() {
        let mut s = EffectsStore::new();
        s.add_effect(9, 0, 10, DEFAULT_EFFECT_FLAGS);
        let snapshot = vec![
            (
                9,
                MobEffectState { amplifier: 2, duration: 400, flags: DEFAULT_EFFECT_FLAGS },
            ),
            (
                21,
                MobEffectState { amplifier: 0, duration: 2400, flags: DEFAULT_EFFECT_FLAGS },
            ),
        ];
        s.sync_from_wire(&snapshot);
        assert_eq!(s.get(9).unwrap().amplifier, 2);
        assert_eq!(s.get(21).unwrap().duration, 2400);
        // the snapshot's silence removes the stale
        s.sync_from_wire(&[]);
        assert!(s.is_empty());
    }

    #[test]
    fn the_hud_sort_walks_the_reverse_natural_order() {
        let mut s = EffectsStore::new();
        // the golden apple's pair: the regen is SHORTER (100) than the
        // absorption (2400); both non-ambient finite — the natural order
        // puts the shorter first, the reverse puts the absorption first
        s.add_effect(MOB_EFFECT_REGENERATION, 0, 100, DEFAULT_EFFECT_FLAGS);
        s.add_effect(MOB_EFFECT_ABSORPTION, 0, 2400, DEFAULT_EFFECT_FLAGS);
        let rows = s.sorted_for_hud();
        assert_eq!(rows[0].0, MOB_EFFECT_ABSORPTION);
        assert_eq!(rows[1].0, MOB_EFFECT_REGENERATION);
        // the ambient flips the front: the natural chain puts non-ambient
        // first, the reversal puts the ambient one first
        let mut s2 = EffectsStore::new();
        s2.add_effect(9, 0, 100, FLAG_VISIBLE | FLAG_SHOW_ICON | FLAG_AMBIENT);
        s2.add_effect(21, 0, 100, DEFAULT_EFFECT_FLAGS);
        let rows2 = s2.sorted_for_hud();
        assert_eq!(rows2[0].0, MOB_EFFECT_REGENERATION, "the ambient rides the front");
        // the equal everything: the color tie-break decides (the meta's
        // own ints — the absorption's 2445989 < the regen's 13458603, so
        // the reversed row puts the regen's bigger color first)
        let mut s3 = EffectsStore::new();
        s3.add_effect(MOB_EFFECT_REGENERATION, 0, 100, DEFAULT_EFFECT_FLAGS);
        s3.add_effect(MOB_EFFECT_ABSORPTION, 0, 100, DEFAULT_EFFECT_FLAGS);
        let rows3 = s3.sorted_for_hud();
        assert_eq!(rows3[0].0, MOB_EFFECT_REGENERATION);
    }

    #[test]
    fn the_icon_alpha_walks_the_cos_law() {
        // the far-from-expiry: full alpha
        assert_eq!(icon_alpha(2400), 1.0);
        assert_eq!(icon_alpha(INFINITE_DURATION), 1.0);
        // the 200-tick boundary: the law's own walk (the cos fold)
        let a = icon_alpha(180);
        assert!((0.0..=1.0).contains(&a));
        // the zero: the base's 0.0 + the wave's 0.25*cos(0) = 0.25
        assert!((icon_alpha(0) - 0.25).abs() < 1e-6);
        // the fold's own arithmetic at a known tick: duration 100 →
        // base clamp(1.0, 0, 0.5) = 0.5, wave clamp((10-5)*0.025,0,.25)
        // = 0.125, cos(100*PI/5) = cos(20PI) = 1 → 0.625
        assert!((icon_alpha(100) - 0.625).abs() < 1e-6);
    }

    #[test]
    fn the_icon_sprites_and_the_name_walk() {
        assert_eq!(effect_icon_sprite(9), Some("mob_effect/regeneration"));
        assert_eq!(effect_icon_sprite(21), Some("mob_effect/absorption"));
        assert_eq!(effect_icon_sprite(9999), None);
        assert_eq!(effect_id_by_name("minecraft:regeneration"), Some(9));
        assert_eq!(effect_id_by_name("minecraft:absorption"), Some(21));
        assert_eq!(effect_id_by_name("minecraft:nope"), None);
    }

    /// THE DARKNESS BLEND THE PADDING THE STEP WEARS (javap the 26.2
    /// jar BlendState.tick pc 0-109 + the tickClient pc 16-21 seat +
    /// MobEffects pc 884-908's setBlendDuration(22) — the fade-in the
    /// 22 steps speak, the padding the target flips 22 ticks before
    /// the expiry, the fade-out completes exactly as the instance
    /// dies, the plain effect's zero-row the snap keeps, the
    /// partial-tick lerp the full-tick partial the POST-tick factor
    /// wears).
    #[test]
    fn the_darkness_blend_the_padding_the_step_wears() {
        let step = 1.0_f32 / DARKNESS_BLEND_TICKS as f32;
        // THE FADE-IN (the 22 steps the 1/22 fold speaks): the MIRROR
        // walk the same f32 accumulation speaks — the exact equality
        // the identical arithmetic keeps
        let mut s = EffectsStore::new();
        assert!(s.add_effect(MOB_EFFECT_DARKNESS, 0, 260, DEFAULT_EFFECT_FLAGS));
        assert_eq!(s.get_blend_factor(MOB_EFFECT_DARKNESS, 1.0), 0.0,
            "the fresh pair the ctor's zero-init keeps");
        let mut mirror = 0.0_f32;
        for k in 1..=DARKNESS_BLEND_TICKS {
            s.tick_client();
            mirror += (1.0_f32 - mirror).clamp(-step, step);
            assert_eq!(s.get_blend_factor(MOB_EFFECT_DARKNESS, 1.0), mirror,
                "fade-in tick {k} the same accumulation wears");
        }
        assert_eq!(mirror, 1.0, "the 22nd step the clamp's convergence lands");
        assert_eq!(s.get(MOB_EFFECT_DARKNESS).unwrap().duration, 260 - DARKNESS_BLEND_TICKS,
            "the decrement the tick's own head serves");
        // THE HOLD (the factor's 1.0 the equal-value early return
        // keeps): the drain to the duration 23 — the padding's own
        // boundary the NEXT tick's post-decrement 22 flips (the blend
        // consults the POST-decrement duration — the tick that LANDS
        // on 22 already wears the fade-out's first step)
        for _ in 0..(260 - 2 * DARKNESS_BLEND_TICKS - 1) {
            s.tick_client();
        }
        assert_eq!(s.get(MOB_EFFECT_DARKNESS).unwrap().duration, DARKNESS_BLEND_TICKS + 1);
        assert_eq!(s.get_blend_factor(MOB_EFFECT_DARKNESS, 1.0), 1.0,
            "the hold the 1.0 keeps");
        // THE FADE-OUT (the padding's own 22 steps — the target the
        // endsWithin(22) verdict flipped): the zero the expiry's tail
        // banks exactly
        let mut mirror = 1.0_f32;
        for k in 1..=DARKNESS_BLEND_TICKS {
            s.tick_client();
            mirror += (0.0_f32 - mirror).clamp(-step, step);
            assert_eq!(s.get_blend_factor(MOB_EFFECT_DARKNESS, 1.0), mirror,
                "fade-out tick {k} the padding's own step wears");
        }
        assert_eq!(mirror, 0.0, "the fade-out completes exactly as the instance dies");
        assert_eq!(s.get(MOB_EFFECT_DARKNESS).unwrap().duration, 1,
            "the fade-out's own 22 steps the duration-1 boundary wears");
        // THE EXPIRED INSTANCE STAYS (the client's copy never removes —
        // the wire's Remove the truth), the factor the 0.0 floor keeps
        s.tick_client();
        assert_eq!(s.get(MOB_EFFECT_DARKNESS).unwrap().duration, 0);
        assert_eq!(s.get_blend_factor(MOB_EFFECT_DARKNESS, 1.0), 0.0,
            "the expiry the fade-out already completed — the padding's own poetry");
        // THE PARTIAL-TICK LERP (the BlendState.getFactor pc 15-27 —
        // the prev/factor pair the partial interpolates): the fresh
        // instance's first tick the prev 0 the lerp walks FROM
        let mut s2 = EffectsStore::new();
        s2.add_effect(MOB_EFFECT_DARKNESS, 0, 260, DEFAULT_EFFECT_FLAGS);
        s2.tick_client();
        assert_eq!(s2.get_blend_factor(MOB_EFFECT_DARKNESS, 0.5), step * 0.5,
            "the half-tick partial the mid-fade wears");
        assert_eq!(s2.get_blend_factor(MOB_EFFECT_DARKNESS, 0.0), 0.0,
            "the zero partial the pre-tick value wears");
        // THE PLAIN EFFECT'S OWN SNAP (the zero blend row the JVM
        // zero-init keeps — the 0 blend ticks the factor snaps to the
        // target): the fire-resistance add the first tick's 1.0 wears
        let mut s3 = EffectsStore::new();
        s3.add_effect(MOB_EFFECT_FIRE_RESISTANCE, 0, 2400, DEFAULT_EFFECT_FLAGS);
        s3.tick_client();
        assert_eq!(s3.get_blend_factor(MOB_EFFECT_FIRE_RESISTANCE, 1.0), 1.0,
            "the zero row the snap keeps");
        // THE OFFLINE LANE'S OWN SEAT (the same blend tick the
        // tick_offline walk's tail wears — the decrement's POST state
        // the blend consults; the expired instance the removal's own
        // verdict keeps)
        let mut s4 = EffectsStore::new();
        s4.add_effect(MOB_EFFECT_DARKNESS, 0, 260, DEFAULT_EFFECT_FLAGS);
        s4.tick_offline();
        assert_eq!(s4.get_blend_factor(MOB_EFFECT_DARKNESS, 1.0), step,
            "the offline walk the blend's own tick serves");
        s4.effects.get_mut(&MOB_EFFECT_DARKNESS).unwrap().duration = 1;
        s4.tick_offline();
        assert!(s4.get(MOB_EFFECT_DARKNESS).is_none(),
            "the duration-1 walk the same-tick expiry REMOVES — the walk's own law");
        assert_eq!(s4.get_blend_factor(MOB_EFFECT_DARKNESS, 1.0), 0.0,
            "the removal the padding's zero factor keeps");
    }
}

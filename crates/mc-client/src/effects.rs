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
/// `minecraft:fire_resistance`'s mob-effect registry id (the 0-based
/// order's own slot — the lava fog's vision arm reads it).
pub const MOB_EFFECT_FIRE_RESISTANCE: i32 = 11;
/// `minecraft:absorption`'s mob-effect registry id (the test pins it).
pub const MOB_EFFECT_ABSORPTION: i32 = 21;

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
/// (1.0 each — the caller folds them through the `Entity.heal` clamp)
/// and the ids that expired this tick (the removal the server's
/// `tickServer` false return speaks).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct OfflineTick {
    pub heals: Vec<f32>,
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
        let new = EffectInstance { amplifier, duration, flags };
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
    /// value persists (no removal hook — the module head's law).
    #[cfg_attr(not(test), expect(dead_code))]
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
                    if !inst.is_infinite() {
                        inst.duration -= 1;
                    }
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
        for inst in self.effects.values_mut() {
            if inst.has_remaining() && !inst.is_infinite() {
                inst.duration -= 1;
            }
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
}

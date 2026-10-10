//! THE DIG LANE — the vanilla `MultiPlayerGameMode`'s destroy half, the
//! survival state machine the left mouse button speaks (javap of the real
//! 26.2 class: `startDestroyBlock` / `continueDestroyBlock` /
//! `stopDestroyBlock` / `destroyBlock` / `sameDestroyTarget`, plus the
//! progress law from `BlockBehaviour.getDestroyProgress` and
//! `Player.getDestroySpeed`). The client's own mirror: the same fields
//! (`isDestroying` / `destroyBlockPos` / `destroyingItem` / `destroyProgress`
//! / `destroyTicks` / `destroyDelay`), the same tick order, the same wire
//! sends (the Player Action packet's START/ABORT/STOP actions through the
//! use arc's own prediction counter).

use crate::world::World;

/// The ServerboundPlayerActionPacket$Action ordinals (javap of the enum's
/// static init: START iconst_0, ABORT iconst_1, STOP iconst_2; RELEASE
/// iconst_5 rides the use arc's own lane).
pub const ACTION_START: u8 = 0;
pub const ACTION_ABORT: u8 = 1;
pub const ACTION_STOP: u8 = 2;

/// One wire send the state machine produced: the action triple the
/// caller wraps into the command channel (the worker speaks the packet
/// with the same monotonic prediction sequence the use arc carries).
#[derive(Debug, Clone, PartialEq)]
pub struct DigSend {
    pub action: u8,
    pub pos: (i32, i32, i32),
    pub face: u8,
}

/// The destroy state machine — the survival lane's own fields, verbatim.
#[derive(Debug, Clone)]
pub struct DigState {
    pub is_destroying: bool,
    pub pos: (i32, i32, i32),
    /// The held item at the dig's start (the vanilla `destroyingItem` —
    /// `sameDestroyTarget` demands the SAME item id; a hotbar switch
    /// restarts the dig). The client's own wire id domain (i32).
    pub destroying_item: Option<i32>,
    pub progress: f32,
    pub ticks: f32,
    /// The 5-tick cooldown after a completion (the vanilla `destroyDelay`
    /// — the broken cell's neighbors breathe before the next dig arms).
    pub delay: i32,
    /// The local-prediction flag the whole client speaks: the OFFLINE
    /// world's voxels edit in place (the singleplayer behavior), the
    /// NETWORKED world stays the server's own authority (the completion
    /// waits for the server's chunk refresh — the placement lane's own
    /// verdict). The machine's reads stay honest either way.
    pub apply_locally: bool,
}

impl Default for DigState {
    fn default() -> Self {
        Self::new()
    }
}

impl DigState {
    pub fn new() -> Self {
        Self {
            is_destroying: false,
            pos: (0, 0, 0),
            destroying_item: None,
            progress: 0.0,
            ticks: 0.0,
            delay: 0,
            apply_locally: true,
        }
    }

    /// The overlay's destroy stage — the vanilla `getDestroyStage`'s own
    /// law: `progress > 0 ? (int)(progress * 10.0) : -1` (the index the
    /// destroy_stage_0..9 tiles speak; -1 renders nothing).
    pub fn destroy_stage(&self) -> i32 {
        if self.progress > 0.0 {
            (self.progress * 10.0) as i32
        } else {
            -1
        }
    }

    /// The vanilla `sameDestroyTarget`: the same cell AND the same held
    /// item (`ItemStack.isSameItemSameComponents` — the client's stack
    /// carries the item id alone, the id is the honest comparison).
    pub fn same_destroy_target(&self, pos: (i32, i32, i32), held: Option<i32>) -> bool {
        self.pos == pos && self.destroying_item == held
    }

    /// `Player.getDestroySpeed`'s own chain, the survival defaults the
    /// client speaks (javap verbatim): the item speed 1.0 (the hand —
    /// the tool components' speeds ride the item tables when they land),
    /// the `BLOCK_BREAK_SPEED` attribute's default 1.0 multiply, the
    /// `SUBGERGED_MINING_SPEED` 0.2 in a water eye, the off-ground /5.
    /// THE HASTE LANE (javap getDestroySpeed pcs 29-51 — the
    /// MobEffectUtil.hasDigSpeed branch the fatigue's own head keeps):
    /// the amplifier the simple PRODUCT reads (NOT the tableswitch the
    /// harmful arm speaks) — `f *= 1.0f + (amplifier + 1) as f32 *
    /// 0.2f` — the amp-0 the ×1.2, the amp-1 the ×1.4, the amp-2 the
    /// ×1.6, the amp-9 the ×3.0 (the ladder NEVER caps: the amplifier's
    /// own arithmetic the rung grows every +1 wears), the lane the
    /// chain's HEAD the haste's own seat before the fatigue speaks.
    /// The DOSE's own source (javap MobEffectUtil — the
    /// getDigSpeedAmplification the Math.max wears): the haste amp and
    /// the conduit amp the max folds, the conduit's own half the
    /// future CONDUIT_POWER round carries (the haste slice the lane
    /// here speaks).
    /// THE FATIGUE LANE (javap pcs 52-125 — the
    /// hasEffect(MINING_FATIGUE) branch the haste's own tail keeps): the
    /// amplifier the TABLESWITCH reads (NOT the per-amp product the
    /// attribute templates speak) — the amp-0 the ×0.3f, the amp-1 the
    /// ×0.09f (0.3²), the amp-2 the ×0.0027f, the amp-3 AND BEYOND the
    /// default arm the ×8.1E-4 CAP wears — the fold the haste's own
    /// product rides and the block-break-speed/water/air arms the
    /// vanilla order keeps (the haste the head, the fatigue the NEXT
    /// fold, the f32 order the exact walk preserves).
    pub fn player_dig_speed(
        on_ground: bool,
        eye_in_water: bool,
        haste: Option<i32>,
        mining_fatigue: Option<i32>,
    ) -> f32 {
        let mut f = 1.0f32;
        if let Some(amp) = haste {
            // the DIG_SPEED product: 1.0 + 0.2 × (amplifier + 1) — the
            // f32 multiply the vanilla's own `1.0F + f * 0.2F` speaks
            f *= 1.0 + (amp + 1) as f32 * 0.2;
        }
        if let Some(amp) = mining_fatigue {
            f *= match amp {
                0 => 0.3,
                1 => 0.09,
                2 => 0.0027,
                _ => 8.1E-4,
            };
        }
        if eye_in_water {
            f *= 0.2; // Attributes.SUBGERGED_MINING_SPEED's default
        }
        if !on_ground {
            f /= 5.0; // the airborne mining penalty's own law
        }
        f
    }

    /// `BlockBehaviour.getDestroyProgress` verbatim: the per-tick
    /// fraction = playerSpeed / destroySpeed / (correctTool ? 30 : 100).
    /// The -1.0 hardness (bedrock) pins the progress at zero; the 0.0
    /// hardness (leaf_litter) divides to +inf — the vanilla insta-break
    /// (the START press's own destroy, the completion branch fires).
    pub fn destroy_progress(player_speed: f32, destroy_speed: f32, correct_tool: bool) -> f32 {
        if destroy_speed == -1.0 {
            return 0.0;
        }
        let div = if correct_tool { 30.0 } else { 100.0 };
        player_speed / destroy_speed / div
    }

    /// The block's dig row (the generated registry's own numbers):
    /// (destroySpeed, requiresCorrectToolForDrops). Unknown names speak
    /// the honest hand defaults (1.0 hardness-like, no tool demand).
    pub fn block_dig_row(kind: u16) -> (f32, bool) {
        let name = crate::world::NAME_OF[kind as usize];
        match mc_registry::generated::block_dig::dig_row(name) {
            Some(row) => row,
            None => (1.0, false),
        }
    }

    /// `hasCorrectToolForDrops` with the client's honest tool belief: the
    /// block's demand answered by the hand (no tool data feeds the
    /// client's stacks yet — the demanding blocks (stone family) take
    /// the 100 divisor, the rest the 30, exactly the bare-hand rates).
    pub fn correct_tool(kind: u16) -> bool {
        let (_, requires_tool) = Self::block_dig_row(kind);
        !requires_tool
    }

    /// `MultiPlayerGameMode.destroyBlock` — the completion's local half:
    /// the cell must be solid (the vanilla's air/GameMaster checks; the
    /// client palette carries none of the masters) and becomes air (the
    /// fluid-legacy law is a no-op for the client palette — no
    /// waterlogged states exist; the broken cell takes the EMPTY fluid).
    /// THE REMOVAL'S OWN CHAIN (javap `Level.removeBlock` pc 12
    /// `iconst_3` — the flag-3 setBlock the UPDATE_NEIGHBORS broadcast
    /// keeps): the broken block's own neighborhood the chain re-reads —
    /// the fire cells the dug block held the [`crate::world::World::
    /// fire_shape_updates`] walk the SAME beat douses (the updateShape
    /// law the scheduled tick's 30..=39 lag never waits). The caller
    /// re-speaks the mesh (the placement lane's own rhythm; the chain's
    /// own cells the same re-speak folds).
    pub fn destroy_block(world: &mut World, pos: (i32, i32, i32)) -> bool {
        if world.get(pos.0, pos.1, pos.2) == crate::world::AIR {
            return false;
        }
        world.set(pos.0, pos.1, pos.2, crate::world::AIR);
        let _ = world.fire_shape_updates(pos);
        true
    }

    /// The completion's local half, gated by the prediction flag (the
    /// networked world's voxels are the server's own authority — the
    /// placement lane's verdict the dig lane speaks too).
    fn destroy_local(&self, world: &mut World, pos: (i32, i32, i32)) {
        if self.apply_locally {
            Self::destroy_block(world, pos);
        }
    }

    /// `MultiPlayerGameMode.startDestroyBlock`'s own dispatch (javap of
    /// the real 26.2 method, scripts/jar_work/mpgamemode_262.txt): the
    /// RESTRICTED head the press speaks first (pcs 0-26 —
    /// `Player.blockActionRestricted`'s verdict: the spectator's acmp
    /// lane refuses, the adventure's mayBuild/item walk refuses — the
    /// client's stacks carry no `can_break` vocabulary, the honest
    /// absence the item lane answers), the INSTABUILD press the
    /// `GameType.updatePlayerAbilities` grant answers (CREATIVE alone
    /// wears the flag — the break the lambda$0 prediction speaks, the
    /// 5-tick delay the continue's own head spends, isDestroying the
    /// lane never arms), and the survival machine the else lane keeps.
    /// Returns the wire sends; the caller speaks them through the
    /// prediction counter. The body state feeds the insta-break check
    /// (the vanilla lambda computes `getDestroyProgress` from the LIVE
    /// player).
    pub fn start_destroy_block(
        &mut self,
        pos: (i32, i32, i32),
        face: u8,
        held: Option<i32>,
        world: &mut World,
        on_ground: bool,
        eye_in_water: bool,
        instabuild: bool,
        restricted: bool,
        mining_fatigue: Option<i32>,
        haste: Option<i32>,
    ) -> Vec<DigSend> {
        // THE RESTRICTED HEAD (audited pcs 0-26): the press speaks
        // NOTHING — no break, no send, no arm. The survival machine's
        // retarget tail re-enters HERE (continueDestroyBlock's own tail
        // call), the flick-to-spectator hold the same verdict keeps.
        if restricted {
            return Vec::new();
        }
        // THE CREATIVE PRESS (audited pcs 46-136): the break rides
        // destroyBlock's own guards (the air cell the press never
        // moves), the START send the lambda$0 prediction speaks, the
        // 5-tick delay the continue's own head spends. The quick
        // re-press the delay never gates (the audited start has no
        // delay check — the fast click breaks faster than the hold).
        // isDestroying NEVER arms (the crack overlay the lane never
        // earns, the release the isDestroying check keeps silent), the
        // retarget abort the state has nothing to abort.
        if instabuild {
            self.destroy_local(world, pos);
            self.delay = 5;
            return vec![DigSend {
                action: ACTION_START,
                pos,
                face,
            }];
        }
        // the same-target re-press: a no-op (the vanilla returns true
        // without a send — the dig continues where it stands)
        if self.is_destroying && self.same_destroy_target(pos, held) {
            return Vec::new();
        }
        let mut sends = Vec::new();
        // the retarget: ABORT the old cell (the vanilla quirk — the
        // abort rides the NEW hit's direction, the caller's own face)
        if self.is_destroying {
            sends.push(DigSend {
                action: ACTION_ABORT,
                pos: self.pos,
                face,
            });
        }
        let kind = world.get(pos.0, pos.1, pos.2);
        let air = kind == crate::world::AIR;
        if !air {
            // the insta-break: the START press's own completion (the
            // vanilla lambda: getDestroyProgress >= 1.0 -> destroyBlock,
            // the dig state never arms)
            let speed = Self::player_dig_speed(on_ground, eye_in_water, haste, mining_fatigue);
            let (destroy_speed, _) = Self::block_dig_row(kind);
            let progress =
                Self::destroy_progress(speed, destroy_speed, Self::correct_tool(kind));
            if progress >= 1.0 {
                self.destroy_local(world, pos);
                sends.push(DigSend {
                    action: ACTION_START,
                    pos,
                    face,
                });
                return sends;
            }
        }
        // arm the dig (the vanilla's own else: the state resets even on
        // the defensive air arm — the next continue disarms it)
        self.is_destroying = true;
        self.pos = pos;
        self.destroying_item = held;
        self.progress = 0.0;
        self.ticks = 0.0;
        sends.push(DigSend {
            action: ACTION_START,
            pos,
            face,
        });
        sends
    }

    /// `MultiPlayerGameMode.continueDestroyBlock`'s own dispatch (the
    /// audited 26.2 walk): the delay head (pcs 0-22 — the decrement
    /// TRUE the swing's beat keeps), the INSTABUILD hold (pcs 39-131 —
    /// the delay-0 tick breaks the aimed cell, the START send rides,
    /// the delay re-arms, the same-target check the lane never asks —
    /// the carve walks where the aim walks), and the survival machine
    /// the else lane keeps. Returns (sends, dug): `dug` names whether
    /// the tick counts as a mining beat (the vanilla's `true` — the
    /// caller swings; the hit sound rides the same beat every 4th tick,
    /// the audio lane's own future hook).
    pub fn continue_destroy_block(
        &mut self,
        pos: (i32, i32, i32),
        face: u8,
        held: Option<i32>,
        world: &mut World,
        on_ground: bool,
        eye_in_water: bool,
        instabuild: bool,
        restricted: bool,
        mining_fatigue: Option<i32>,
        haste: Option<i32>,
    ) -> (Vec<DigSend>, bool) {
        // the completion cooldown ticks first (the vanilla's own order)
        if self.delay > 0 {
            self.delay -= 1;
            return (Vec::new(), true);
        }
        // THE CREATIVE HOLD (audited pcs 39-131): the break the
        // destroyBlock's own guards keep honest (the air cell the tick
        // never moves), the START send the lambda$0 prediction speaks,
        // the delay re-arms. The restricted flick (creative ->
        // spectator mid-hold) the instabuild flip the survival lane
        // resumes — the start tail's restricted head the flick's
        // verdict speaks.
        if instabuild {
            self.destroy_local(world, pos);
            self.delay = 5;
            return (
                vec![DigSend {
                    action: ACTION_START,
                    pos,
                    face,
                }],
                true,
            );
        }
        if !self.same_destroy_target(pos, held) {
            // the retarget restarts through the start lane (the vanilla's
            // own tail call — the restricted head the flick's verdict
            // keeps)
            let sends = self.start_destroy_block(
                pos, face, held, world, on_ground, eye_in_water, instabuild, restricted,
                mining_fatigue, haste,
            );
            let retargeted = !sends.is_empty();
            return (sends, retargeted);
        }
        let kind = world.get(pos.0, pos.1, pos.2);
        if kind == crate::world::AIR {
            // the target vanished (a neighboring dig, a server edit):
            // the state disarms, the beat does not count
            self.is_destroying = false;
            return (Vec::new(), false);
        }
        let (destroy_speed, _) = Self::block_dig_row(kind);
        let speed = Self::player_dig_speed(on_ground, eye_in_water, haste, mining_fatigue);
        self.progress +=
            Self::destroy_progress(speed, destroy_speed, Self::correct_tool(kind));
        // the hit-sound beat: every 4th tick (destroyTicks % 4 == 0) —
        // the audio lane plays the block's hit sound here when sound
        // data lands; the beat itself is the law
        self.ticks += 1.0;
        if self.progress >= 1.0 {
            // THE COMPLETION: the local break + the STOP send (the
            // vanilla's lambda: destroyBlock then STOP), the 5-tick
            // cooldown, the counters reset
            self.is_destroying = false;
            self.destroy_local(world, pos);
            self.progress = 0.0;
            self.ticks = 0.0;
            self.delay = 5;
            return (
                vec![DigSend {
                    action: ACTION_STOP,
                    pos,
                    face,
                }],
                true,
            );
        }
        (Vec::new(), true)
    }

    /// `MultiPlayerGameMode.stopDestroyBlock` — the release/re-target's
    /// own disarm: the ABORT rides the held cell with `Direction.DOWN`
    /// (the vanilla's own constant — the release's direction byte is 0),
    /// the progress resets (the crack overlay clears).
    pub fn stop_destroy_block(&mut self) -> Vec<DigSend> {
        if !self.is_destroying {
            return Vec::new();
        }
        self.is_destroying = false;
        self.progress = 0.0;
        vec![DigSend {
            action: ACTION_ABORT,
            pos: self.pos,
            face: 0, // Direction.DOWN's 3D data value
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::World;

    fn small_world() -> World {
        World::new(2, 42)
    }

    #[test]
    fn destroy_progress_law_matches_vanilla_rates() {
        // the hand rates everyone knows: stone (hardness 1.5, demands
        // the tool) digs at 1/1.5/100 per tick, dirt (0.5, no demand) at
        // 1/0.5/30 — 150 ticks / 15 ticks to break
        let stone = DigState::destroy_progress(1.0, 1.5, false);
        assert!((stone - 1.0 / 1.5 / 100.0).abs() < 1e-9);
        let dirt = DigState::destroy_progress(1.0, 0.5, true);
        assert!((dirt - 1.0 / 0.5 / 30.0).abs() < 1e-9);
        // the off-ground penalty: the speed's /5 rides BEFORE the law
        let air = DigState::player_dig_speed(false, false, None, None);
        assert!((air - 0.2).abs() < 1e-9);
        let water_eye = DigState::player_dig_speed(true, true, None, None);
        assert!((water_eye - 0.2).abs() < 1e-9);
        let both = DigState::player_dig_speed(false, true, None, None);
        assert!((both - 0.2 / 5.0).abs() < 1e-9);
    }

    #[test]
    fn bedrock_pins_zero_and_leaf_litter_is_instant() {
        // the -1 hardness: the progress NEVER moves (bedrock's own law)
        assert_eq!(DigState::destroy_progress(1.0, -1.0, false), 0.0);
        // the 0 hardness: +inf — the insta-break (the START's completion)
        let p = DigState::destroy_progress(1.0, 0.0, true);
        assert!(p.is_infinite() && p > 0.0);
        // the registry's own numbers: the palette rows speak the values
        let (speed, requires) = DigState::block_dig_row(crate::world::STONE);
        assert_eq!(speed, 1.5);
        assert!(requires);
        let (speed, requires) = DigState::block_dig_row(crate::world::DIRT);
        assert_eq!(speed, 0.5);
        assert!(!requires);
        let (speed, _) = DigState::block_dig_row(crate::world::LEAF_LITTER);
        assert_eq!(speed, 0.0);
        let (speed, _) = DigState::block_dig_row(crate::world::BEDROCK);
        assert_eq!(speed, -1.0);
    }

    #[test]
    fn destroy_stage_law() {
        let mut d = DigState::new();
        assert_eq!(d.destroy_stage(), -1, "no progress renders nothing");
        d.progress = 0.3;
        assert_eq!(d.destroy_stage(), 3);
        d.progress = 0.999;
        assert_eq!(d.destroy_stage(), 9, "the last stage caps at index 9");
    }

    #[test]
    fn start_arms_dig_and_sends_start() {
        let mut world = small_world();
        // seed a stone cell
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        let sends = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        assert_eq!(
            sends,
            vec![DigSend {
                action: ACTION_START,
                pos: (1, 1, 1),
                face: 1
            }]
        );
        assert!(d.is_destroying);
        assert_eq!(d.progress, 0.0);
        // the same-target re-press: NO second send (the vanilla's no-op)
        let sends = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        assert!(sends.is_empty());
    }

    #[test]
    fn leaf_litter_instabreak_on_start() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::LEAF_LITTER);
        let mut d = DigState::new();
        let sends = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        // the START still speaks (the vanilla lambda's own return) and
        // the cell is ALREADY air — no dig state armed
        assert_eq!(sends.len(), 1);
        assert_eq!(sends[0].action, ACTION_START);
        assert_eq!(world.get(1, 1, 1), crate::world::AIR);
        assert!(!d.is_destroying);
    }

    #[test]
    fn retarget_aborts_old_with_new_face() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        world.set(2, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        // dig a few ticks into the first cell
        for _ in 0..3 {
            let _ = d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        }
        assert!(d.progress > 0.0);
        // the switch to the neighbor: ABORT(old pos, the NEW face) + the
        // fresh START, the progress zeroed (the vanilla's own reset)
        let sends = d.start_destroy_block((2, 1, 1), 4, None, &mut world, true, false, false, false, None, None);
        assert_eq!(sends.len(), 2);
        assert_eq!(
            sends[0],
            DigSend {
                action: ACTION_ABORT,
                pos: (1, 1, 1),
                face: 4
            }
        );
        assert_eq!(sends[1].action, ACTION_START);
        assert_eq!(sends[1].pos, (2, 1, 1));
        assert!(d.is_destroying);
        assert_eq!(d.progress, 0.0);
    }

    #[test]
    fn hotbar_switch_restarts_the_dig() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        let _ = d.start_destroy_block((1, 1, 1), 1, Some(7), &mut world, true, false, false, false, None, None);
        // the same cell with a DIFFERENT held item: the vanilla's
        // sameDestroyTarget refuses — the abort + the fresh start
        let sends = d.start_destroy_block((1, 1, 1), 1, Some(9), &mut world, true, false, false, false, None, None);
        assert_eq!(sends.len(), 2, "the item switch retargets");
        assert_eq!(sends[0].action, ACTION_ABORT);
        assert_eq!(sends[1].action, ACTION_START);
    }

    #[test]
    fn stone_by_hand_takes_150_beats() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        let start = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        assert_eq!(start.len(), 1);
        let mut beats = 0;
        let mut stop_seen = false;
        while beats < 400 {
            let (sends, dug) =
                d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
            if sends.iter().any(|s| s.action == ACTION_STOP) {
                stop_seen = true;
                break;
            }
            assert!(dug, "the mining beat counts while the dig holds");
            beats += 1;
        }
        assert!(stop_seen, "the stone broke");
        // 150 nominal beats: the f32 accumulation drifts a hair below 1.0
        // on the 150th add (1/1.5/100 = 0.006666667f32 rounds low), the
        // 151st crosses — the vanilla's OWN arithmetic (Java float and
        // Rust f32 are the same IEEE 754 round-to-nearest), so 151 ticks
        // IS the original's hand-stone time (7.55s of the famous 7.5)
        assert_eq!(beats, 150, "the 151st beat completes (the f32 drift the vanilla speaks too)");
        // the cell is air, the dig disarmed, the cooldown armed
        assert_eq!(world.get(1, 1, 1), crate::world::AIR);
        assert!(!d.is_destroying);
        assert_eq!(d.delay, 5);
        // the cooldown: the next continue tick does nothing but tick it
        let (sends, _) = d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        assert!(sends.is_empty());
        assert_eq!(d.delay, 4);
    }

    #[test]
    fn the_fatigue_ladder_the_tableswitch_cap_wears() {
        // THE TABLESWITCH'S OWN RUNGS (javap getDestroySpeed pcs 69-124
        // — the amp-0 the 0.3f, the amp-1 the 0.09f (0.3²), the amp-2
        // the 0.0027f, the amp-3 the default arm the 8.1E-4f speaks)
        let amp0 = DigState::player_dig_speed(true, false, None, Some(0));
        assert!((amp0 - 0.3).abs() < 1e-9, "the amp-0 the x0.3 wears");
        let amp1 = DigState::player_dig_speed(true, false, None, Some(1));
        assert!((amp1 - 0.09).abs() < 1e-9, "the amp-1 the x0.09 wears");
        let amp2 = DigState::player_dig_speed(true, false, None, Some(2));
        assert!((amp2 - 0.0027).abs() < 1e-9, "the amp-2 the x0.0027 wears");
        let amp3 = DigState::player_dig_speed(true, false, None, Some(3));
        assert!(
            (amp3 - 8.1E-4).abs() < 1e-9,
            "the amp-3 the default arm wears: {}",
            amp3
        );
        // THE CAP (the amp-4+ the DEFAULT arm the same 8.1E-4 wears —
        // the ladder the 4th rung never grows past, the tableswitch's
        // own three-case walk keeps)
        let amp4 = DigState::player_dig_speed(true, false, None, Some(4));
        assert_eq!(amp4, amp3, "the amp-4 the default cap wears");
        let amp9 = DigState::player_dig_speed(true, false, None, Some(9));
        assert_eq!(amp9, amp3, "the amp-9 the same cap wears");
        // THE ORDER (the fatigue the HEAD of the chain — the water and
        // air arms the f32 walk rides AFTER the ladder, the vanilla
        // body's own sequence keeps)
        let tired_wet = DigState::player_dig_speed(true, true, None, Some(0));
        assert!((tired_wet - 0.3 * 0.2).abs() < 1e-9, "the wet eye the ladder first wears");
        let tired_air = DigState::player_dig_speed(false, false, None, Some(0));
        assert!((tired_air - 0.3 / 5.0).abs() < 1e-9, "the air the ladder first wears");
    }

    #[test]
    fn the_haste_ladder_the_product_never_caps_wears() {
        // THE PRODUCT'S OWN RUNGS (javap getDigSpeed — the
        // hasEffect(DIG_SPEED) arm the simple multiply speaks, NOT the
        // tableswitch the fatigue's own arm reads): the amp-0 the ×1.2,
        // the amp-1 the ×1.4, the amp-2 the ×1.6, the amp-3 the ×1.8 —
        // the rung the +0.2 every amplifier wears, the ladder the CAP
        // never knows (the fatigue's own 8.1E-4 floor the opposite
        // pole keeps)
        let amp0 = DigState::player_dig_speed(true, false, Some(0), None);
        assert!((amp0 - 1.2).abs() < 1e-9, "the amp-0 the x1.2 wears");
        let amp1 = DigState::player_dig_speed(true, false, Some(1), None);
        assert!((amp1 - 1.4).abs() < 1e-9, "the amp-1 the x1.4 wears");
        let amp2 = DigState::player_dig_speed(true, false, Some(2), None);
        assert!((amp2 - 1.6).abs() < 1e-9, "the amp-2 the x1.6 wears");
        let amp3 = DigState::player_dig_speed(true, false, Some(3), None);
        assert!((amp3 - 1.8).abs() < 1e-9, "the amp-3 the x1.8 wears");
        // THE NO-CAP (the amp-9 the ×3.0 the arithmetic wears — the
        // amplifier the rung keeps growing past the fatigue's own
        // tableswitch the 4th rung caps)
        let amp9 = DigState::player_dig_speed(true, false, Some(9), None);
        assert!((amp9 - 3.0).abs() < 1e-9, "the amp-9 the x3.0 wears");
        // THE SEAT (the haste the chain's HEAD — the water and air arms
        // the f32 walk rides AFTER the product, the vanilla body's own
        // sequence keeps)
        let hasty_wet = DigState::player_dig_speed(true, true, Some(0), None);
        assert!(
            (hasty_wet - 1.2 * 0.2).abs() < 1e-9,
            "the wet eye the product first wears"
        );
        let hasty_air = DigState::player_dig_speed(false, false, Some(0), None);
        assert!(
            (hasty_air - 1.2 / 5.0).abs() < 1e-9,
            "the air the product first wears"
        );
    }

    #[test]
    fn the_haste_and_the_fatigue_the_head_then_the_table_wears() {
        // THE COMPOSITION (the vanilla's own order: the DIG_SPEED
        // product the chain's head speaks, the DIG_SLOWDOWN tableswitch
        // the NEXT fold wears — 1.2 × 0.3 the amp-pair-0 the exact f32
        // keeps, 3.0 × 8.1E-4 the amp-pair-9 the cap tail keeps)
        let both0 = DigState::player_dig_speed(true, false, Some(0), Some(0));
        assert!(
            (both0 - 1.2 * 0.3).abs() < 1e-9,
            "the head the product wears: {}",
            both0
        );
        let both9 = DigState::player_dig_speed(true, false, Some(9), Some(3));
        assert!(
            (both9 - 3.0 * 8.1E-4).abs() < 1e-9,
            "the tail the cap wears: {}",
            both9
        );
        // THE HASTE THE DIG THE STONE'S BEATS THE STRETCH WEARS: the
        // ×1.2 speed the per-tick 1.2/1.5/100 the ladder speaks
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        let _ = d.start_destroy_block(
            (1, 1, 1), 1, None, &mut world, true, false, false, false, None, Some(0),
        );
        let _ = d.continue_destroy_block(
            (1, 1, 1), 1, None, &mut world, true, false, false, false, None, Some(0),
        );
        assert!(
            (d.progress - 1.2_f32 / 1.5 / 100.0).abs() < 1e-12,
            "the hasty per-tick the exact f32 wears: {}",
            d.progress
        );
    }

    #[test]
    fn the_fatigue_stretches_the_stone_beats() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        let _ = d.start_destroy_block(
            (1, 1, 1), 1, None, &mut world, true, false, false, false, Some(0), None,
        );
        // THE FIRST BEAT (the per-tick progress the ×0.3 speed wears —
        // 0.3/1.5/100 the exact f32 the mirror keeps)
        let (sends, _) =
            d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, Some(0), None);
        assert!(sends.is_empty(), "the hold beat speaks no send");
        assert!(
            (d.progress - 0.3_f32 / 1.5 / 100.0).abs() < 1e-12,
            "the fatigued per-tick the exact f32 wears: {}",
            d.progress
        );
        // THE STRETCH (the ×3.33 the 500-beat nominal the vanilla's own
        // f32 accumulation keeps — the same drift law the 150-beat test
        // speaks, the completion the STOP send posts)
        let mut beats = 1;
        let mut stop_seen = false;
        while beats < 2000 {
            let (sends, dug) =
                d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, Some(0), None);
            if sends.iter().any(|s| s.action == ACTION_STOP) {
                stop_seen = true;
                break;
            }
            assert!(dug, "the mining beat counts while the dig holds");
            beats += 1;
        }
        assert!(stop_seen, "the fatigued stone broke");
        assert_eq!(
            beats, 500,
            "the 500-beat nominal the ×0.3 speed wears (the f32 drift the vanilla speaks too)"
        );
        assert_eq!(world.get(1, 1, 1), crate::world::AIR);
        assert!(!d.is_destroying);
        assert_eq!(d.delay, 5);
    }

    #[test]
    fn dirt_by_hand_takes_15_beats() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::DIRT);
        let mut d = DigState::new();
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        let mut beats = 0;
        loop {
            let (sends, _) =
                d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
            beats += 1;
            if sends.iter().any(|s| s.action == ACTION_STOP) {
                break;
            }
            assert!(beats < 100);
        }
        assert_eq!(beats, 15, "1/0.5/30: the 15th beat completes");
    }

    #[test]
    fn airborne_mining_is_five_times_slower() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::DIRT);
        let mut d = DigState::new();
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        let mut beats = 0;
        loop {
            let (sends, _) =
                d.continue_destroy_block((1, 1, 1), 1, None, &mut world, false, false, false, false, None, None);
            beats += 1;
            if sends.iter().any(|s| s.action == ACTION_STOP) {
                break;
            }
            assert!(beats < 400);
        }
        // the same f32 drift: 75 nominal beats cross on the 76th (the
        // vanilla's own accumulation)
        assert_eq!(beats, 76, "the /5 airborne speed: the 76th beat completes");
    }

    #[test]
    fn release_aborts_with_down_face() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        let _ = d.start_destroy_block((1, 1, 1), 4, None, &mut world, true, false, false, false, None, None);
        for _ in 0..2 {
            let _ = d.continue_destroy_block((1, 1, 1), 4, None, &mut world, true, false, false, false, None, None);
        }
        let sends = d.stop_destroy_block();
        assert_eq!(
            sends,
            vec![DigSend {
                action: ACTION_ABORT,
                pos: (1, 1, 1),
                face: 0 // the vanilla's Direction.DOWN constant
            }]
        );
        assert!(!d.is_destroying);
        assert_eq!(d.progress, 0.0, "the crack overlay clears");
        // the disarm is idempotent (the vanilla's own guard)
        assert!(d.stop_destroy_block().is_empty());
        // the cell SURVIVES the abort (the dig never completed)
        assert_eq!(world.get(1, 1, 1), crate::world::STONE);
    }

    #[test]
    fn vanished_target_disarms_without_a_beat() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        // the cell vanishes under the dig (a neighbor's completion)
        world.set(1, 1, 1, crate::world::AIR);
        let (sends, dug) = d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        assert!(sends.is_empty());
        assert!(!dug, "the air beat counts nothing");
        assert!(!d.is_destroying);
    }

    #[test]
    fn the_creative_press_breaks_instant_and_arms_the_delay() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        world.set(2, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        // THE PRESS: the cell vanishes the SAME tick, the START rides,
        // the delay arms, isDestroying never speaks (the audited
        // instabuild branch's own shape)
        let sends = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, true, false, None, None);
        assert_eq!(world.get(1, 1, 1), crate::world::AIR, "the instant break");
        assert_eq!(sends.len(), 1, "the single prediction send");
        assert_eq!(sends[0].action, ACTION_START, "the lambda$0's own action");
        assert_eq!(sends[0].pos, (1, 1, 1));
        assert!(!d.is_destroying, "the crack lane never arms");
        assert_eq!(d.progress, 0.0);
        assert_eq!(d.delay, 5, "the audited destroyDelay arm");
        // THE RE-PRESS: the delay never gates the start (the audited
        // pcs 46-136 carry no delay check — the fast click breaks
        // faster than the hold)
        let sends = d.start_destroy_block((2, 1, 1), 1, None, &mut world, true, false, true, false, None, None);
        assert_eq!(world.get(2, 1, 1), crate::world::AIR, "the second instant break");
        assert_eq!(sends.len(), 1);
        assert_eq!(d.delay, 5, "the delay re-armed");
        // THE RELEASE: the isDestroying check keeps silent (no ABORT —
        // the audited stopDestroyBlock's own guard)
        assert!(
            d.stop_destroy_block().is_empty(),
            "the creative release speaks nothing"
        );
    }

    #[test]
    fn the_creative_hold_carves_the_six_tick_law() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        world.set(2, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        // the press breaks the first cell
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, true, false, None, None);
        // THE HOLD: five decrement ticks speak nothing (the delay
        // head), each still beating the swing (the TRUE return)
        for t in 0..5 {
            let (sends, dug) =
                d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, true, false, None, None);
            assert!(sends.is_empty(), "the decrement tick {t} sends nothing");
            assert!(dug, "the decrement tick still beats the swing");
        }
        // the sixth tick: the delay spent, the aimed cell breaks — the
        // carve walks where the aim walks (the same-target check the
        // hold lane never asks)
        let (sends, dug) =
            d.continue_destroy_block((2, 1, 1), 1, None, &mut world, true, false, true, false, None, None);
        assert!(dug);
        assert_eq!(sends.len(), 1);
        assert_eq!(sends[0].action, ACTION_START, "the hold's own START send");
        assert_eq!(world.get(2, 1, 1), crate::world::AIR, "the carve's second cell");
        assert_eq!(d.delay, 5, "the delay re-armed");
        assert!(!d.is_destroying, "the hold lane the state never arms");
    }

    #[test]
    fn the_restricted_press_speaks_nothing() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        // THE RESTRICTED HEAD: the spectator's acmp lane, the
        // adventure's mayBuild/item walk — no break, no send, no arm
        let sends = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, true, None, None);
        assert!(sends.is_empty(), "the press sends nothing");
        assert_eq!(world.get(1, 1, 1), crate::world::STONE, "the block stands");
        assert!(!d.is_destroying, "the dig never arms");
        assert_eq!(d.delay, 0, "the delay never arms");
        // THE HOLD after the refused press: the retarget tail re-enters
        // the refused start — the same silence, the beat never counts
        let (sends, dug) =
            d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, true, None, None);
        assert!(sends.is_empty());
        assert!(!dug, "the refused continue never beats");
    }

    #[test]
    fn the_creative_flick_the_survival_lane_resumes() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        // the creative press breaks and arms the delay
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, true, false, None, None);
        assert_eq!(d.delay, 5);
        // THE FLICK TO SURVIVAL (the F4 switcher's own revoke): the
        // survival machine resumes — the creative press's delay the
        // continue's own head spends first (five TRUE beats), then the
        // same-target state never armed (the creative press never
        // arms) and the survival start re-arms through the tail
        for i in 0..5 {
            let (sends, dug) =
                d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
            assert!(sends.is_empty(), "the spent delay tick {i} sends nothing");
            assert!(dug, "the delay tick {i} still beats");
        }
        let (sends, dug) =
            d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false, None, None);
        assert!(dug, "the survival re-arm beats");
        assert!(d.is_destroying, "the survival dig armed");
        assert!(!sends.is_empty(), "the survival START rode the re-arm");
        // THE FLICK TO SPECTATOR: the retarget tail the restricted head
        // refuses — the carve stops where the flick lands
        let (sends, dug) =
            d.continue_destroy_block((2, 1, 1), 1, None, &mut world, true, false, false, true, None, None);
        assert!(sends.is_empty());
        assert!(!dug, "the refused retarget never beats");
    }

    #[test]
    fn the_dig_the_support_the_fire_chain_douses() {
        // THE REMOVAL'S OWN CHAIN (javap `Level.removeBlock` pc 12
        // `iconst_3` — the flag-3 broadcast the dig's own
        // destroyBlock walk serves, `FireBlock.updateShape` the
        // pc 0-8 canSurvive verdict the pc 32-38 AIR answers): the
        // fire the dug block held dies the SAME beat the dig lands —
        // the scheduled tick's own survival arm the 30..=39 lag the
        // vanilla never waits
        let mut world = small_world();
        // THE STAGE: the log the fire stands on, the fire's age the
        // ladder banks, a FAR fire on its own log the chain's reach
        // never crosses
        world.set(1, 1, 1, crate::world::LOG);
        world.set(1, 2, 1, crate::world::FIRE);
        world.fire_age.insert((1, 2, 1), 2);
        world.set(6, 1, 6, crate::world::LOG);
        world.set(6, 2, 6, crate::world::FIRE);
        world.fire_age.insert((6, 2, 6), 5);
        let mut d = DigState::new();
        // THE CREATIVE PRESS: the log breaks, the chain fires, the
        // flame above the SAME beat douses (the stale schedule entry
        // the block-match gate later drops)
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, true, false, None, None);
        assert_eq!(world.get(1, 1, 1), crate::world::AIR, "the log the press breaks");
        assert_eq!(
            world.get(1, 2, 1),
            crate::world::AIR,
            "the fire the support's loss the chain douses — the same beat"
        );
        assert!(
            !world.fire_age.contains_key(&(1, 2, 1)),
            "the dead cell's age entry the chain drops"
        );
        assert_eq!(
            world.get(6, 2, 6),
            crate::world::FIRE,
            "the far fire the chain's reach never crosses"
        );
        assert!(
            world.fire_age.contains_key(&(6, 2, 6)),
            "the living fire's own age entry the chain keeps"
        );
        // THE CASCADE (the dying cell's own broadcast the chain
        // re-fires): the fire stack the log's loss kills bottom-up —
        // the top cell's own verdict reads the BOTTOM cell's air, the
        // walk the worklist keeps
        world.set(1, 1, 1, crate::world::LOG);
        world.set(1, 2, 1, crate::world::FIRE);
        world.fire_age.insert((1, 2, 1), 0);
        world.set(1, 3, 1, crate::world::FIRE);
        world.fire_age.insert((1, 3, 1), 0);
        let mut d = DigState::new();
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, true, false, None, None);
        assert_eq!(world.get(1, 2, 1), crate::world::AIR, "the bottom flame douses");
        assert_eq!(world.get(1, 3, 1), crate::world::AIR, "the top flame the cascade douses");
        // THE AIR PRESS (the chain's own silence): the fireless dig
        // the chain's verdict never speaks — the return the dig's own
        // law keeps
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, true, false, None, None);
        assert_eq!(world.get(1, 1, 1), crate::world::AIR, "the stone breaks as ever");
    }
}

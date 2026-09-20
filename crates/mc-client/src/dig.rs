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
    /// (The haste/fatigue effect branches: no effect lanes feed the
    /// client yet — their absence IS the vanilla's no-effect default.)
    pub fn player_dig_speed(on_ground: bool, eye_in_water: bool) -> f32 {
        let mut f = 1.0f32;
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
    /// The caller re-speaks the mesh (the placement lane's own rhythm).
    pub fn destroy_block(world: &mut World, pos: (i32, i32, i32)) -> bool {
        if world.get(pos.0, pos.1, pos.2) == crate::world::AIR {
            return false;
        }
        world.set(pos.0, pos.1, pos.2, crate::world::AIR);
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
            let speed = Self::player_dig_speed(on_ground, eye_in_water);
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
        let speed = Self::player_dig_speed(on_ground, eye_in_water);
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
        let air = DigState::player_dig_speed(false, false);
        assert!((air - 0.2).abs() < 1e-9);
        let water_eye = DigState::player_dig_speed(true, true);
        assert!((water_eye - 0.2).abs() < 1e-9);
        let both = DigState::player_dig_speed(false, true);
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
        let sends = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
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
        let sends = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
        assert!(sends.is_empty());
    }

    #[test]
    fn leaf_litter_instabreak_on_start() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::LEAF_LITTER);
        let mut d = DigState::new();
        let sends = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
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
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
        // dig a few ticks into the first cell
        for _ in 0..3 {
            let _ = d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
        }
        assert!(d.progress > 0.0);
        // the switch to the neighbor: ABORT(old pos, the NEW face) + the
        // fresh START, the progress zeroed (the vanilla's own reset)
        let sends = d.start_destroy_block((2, 1, 1), 4, None, &mut world, true, false, false, false);
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
        let _ = d.start_destroy_block((1, 1, 1), 1, Some(7), &mut world, true, false, false, false);
        // the same cell with a DIFFERENT held item: the vanilla's
        // sameDestroyTarget refuses — the abort + the fresh start
        let sends = d.start_destroy_block((1, 1, 1), 1, Some(9), &mut world, true, false, false, false);
        assert_eq!(sends.len(), 2, "the item switch retargets");
        assert_eq!(sends[0].action, ACTION_ABORT);
        assert_eq!(sends[1].action, ACTION_START);
    }

    #[test]
    fn stone_by_hand_takes_150_beats() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        let start = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
        assert_eq!(start.len(), 1);
        let mut beats = 0;
        let mut stop_seen = false;
        while beats < 400 {
            let (sends, dug) =
                d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
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
        let (sends, _) = d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
        assert!(sends.is_empty());
        assert_eq!(d.delay, 4);
    }

    #[test]
    fn dirt_by_hand_takes_15_beats() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::DIRT);
        let mut d = DigState::new();
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
        let mut beats = 0;
        loop {
            let (sends, _) =
                d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
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
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
        let mut beats = 0;
        loop {
            let (sends, _) =
                d.continue_destroy_block((1, 1, 1), 1, None, &mut world, false, false, false, false);
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
        let _ = d.start_destroy_block((1, 1, 1), 4, None, &mut world, true, false, false, false);
        for _ in 0..2 {
            let _ = d.continue_destroy_block((1, 1, 1), 4, None, &mut world, true, false, false, false);
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
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
        // the cell vanishes under the dig (a neighbor's completion)
        world.set(1, 1, 1, crate::world::AIR);
        let (sends, dug) = d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
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
        let sends = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, true, false);
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
        let sends = d.start_destroy_block((2, 1, 1), 1, None, &mut world, true, false, true, false);
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
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, true, false);
        // THE HOLD: five decrement ticks speak nothing (the delay
        // head), each still beating the swing (the TRUE return)
        for t in 0..5 {
            let (sends, dug) =
                d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, true, false);
            assert!(sends.is_empty(), "the decrement tick {t} sends nothing");
            assert!(dug, "the decrement tick still beats the swing");
        }
        // the sixth tick: the delay spent, the aimed cell breaks — the
        // carve walks where the aim walks (the same-target check the
        // hold lane never asks)
        let (sends, dug) =
            d.continue_destroy_block((2, 1, 1), 1, None, &mut world, true, false, true, false);
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
        let sends = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, true);
        assert!(sends.is_empty(), "the press sends nothing");
        assert_eq!(world.get(1, 1, 1), crate::world::STONE, "the block stands");
        assert!(!d.is_destroying, "the dig never arms");
        assert_eq!(d.delay, 0, "the delay never arms");
        // THE HOLD after the refused press: the retarget tail re-enters
        // the refused start — the same silence, the beat never counts
        let (sends, dug) =
            d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, true);
        assert!(sends.is_empty());
        assert!(!dug, "the refused continue never beats");
    }

    #[test]
    fn the_creative_flick_the_survival_lane_resumes() {
        let mut world = small_world();
        world.set(1, 1, 1, crate::world::STONE);
        let mut d = DigState::new();
        // the creative press breaks and arms the delay
        let _ = d.start_destroy_block((1, 1, 1), 1, None, &mut world, true, false, true, false);
        assert_eq!(d.delay, 5);
        // THE FLICK TO SURVIVAL (the F4 switcher's own revoke): the
        // survival machine resumes — the creative press's delay the
        // continue's own head spends first (five TRUE beats), then the
        // same-target state never armed (the creative press never
        // arms) and the survival start re-arms through the tail
        for i in 0..5 {
            let (sends, dug) =
                d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
            assert!(sends.is_empty(), "the spent delay tick {i} sends nothing");
            assert!(dug, "the delay tick {i} still beats");
        }
        let (sends, dug) =
            d.continue_destroy_block((1, 1, 1), 1, None, &mut world, true, false, false, false);
        assert!(dug, "the survival re-arm beats");
        assert!(d.is_destroying, "the survival dig armed");
        assert!(!sends.is_empty(), "the survival START rode the re-arm");
        // THE FLICK TO SPECTATOR: the retarget tail the restricted head
        // refuses — the carve stops where the flick lands
        let (sends, dug) =
            d.continue_destroy_block((2, 1, 1), 1, None, &mut world, true, false, false, true);
        assert!(sends.is_empty());
        assert!(!dug, "the refused retarget never beats");
    }
}

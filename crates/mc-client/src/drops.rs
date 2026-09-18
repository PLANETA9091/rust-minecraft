//! THE DROP LANE — the vanilla `Block.popResource` / `ItemEntity` /
//! `Player` pickup arc, the dig's reward half (javap of the real 26.2
//! classes). The survival break rolls the block's loot (the generated
//! hand-lane table, the TOOL GATE riding block_dig's own
//! `requiresCorrectToolForDrops` flag), spawns the item entity by the
//! popResource law, and the entity lives its vanilla tick: the gravity
//! 0.04, the collision sweep (body.rs's own Y-first law), the ground
//! drag, the landing bounce, the settle skip, the neighbour merge, the
//! 6000-tick despawn. The player's box scans `inflate(1.0, 0.5, 1.0)`
//! (Player.aiStep), the pickup rides `Inventory.add`'s own merge order
//! (the selected slot first, the scan, the free slot, the 64 cap), and
//! the pickup voice folds `volume 0.2`, `pitch (r - r) * 1.4 + 2.0`
//! (ClientPacketListener.handleTakeItemEntity — the modern law).
//!
//! The RENDER laws ride the constants here; the quad composition lives
//! in screens.rs (the registry's ground-context chain).

use crate::body::{collide, Collision};
use crate::world::World;

/// The ITEM entity's own dimensions — the runtime probe's verdict
/// (ProbeItemDims.java, the bootstrapped registry's own getters).
pub const ITEM_WIDTH: f64 = 0.25;
pub const ITEM_HEIGHT: f64 = 0.25;
const HALF: f64 = ITEM_WIDTH / 2.0;

/// The `popResource` spawn law's own spread (javap Block.popResource:
/// `Mth.nextDouble(random, -0.25, 0.25)` on each axis, the y lifted by
/// `EntityType.ITEM.getHeight() / 2.0` = 0.125).
pub const POP_SPREAD: f64 = 0.25;
/// The pickup delay the block drops arm (javap: `setPickUpDelay(10)`.
pub const POP_PICKUP_DELAY: i32 = 10;
/// The despawn age (javap ItemEntity.tick: `age >= 6000` discards).
pub const DESPAWN_AGE: i32 = 6000;
/// The settle skip's divisor (javap: `(tickCount + getId()) % 4 != 0`
/// skips the move while the item rests).
pub const SETTLE_PERIOD: u64 = 4;
/// The neighbour-merge cadence (javap: the block-crossing variant's %2,
/// the settled %40).
pub const MERGE_PERIOD: u64 = 40;
/// The merge scan's inflate (javap mergeWithNeighbours).
pub const MERGE_INFLATE: f64 = 0.5;
/// The aiStep pickup scan's inflate (javap Player.aiStep, the no-vehicle
/// branch — `(1.0, 0.5, 1.0)`).
pub const PICKUP_INFLATE: (f64, f64, f64) = (1.0, 0.5, 1.0);
/// The pickup voice's own fold (javap handleTakeItemEntity: the
/// ITEM_PICKUP branch plays volume 0.2, the pitch `(r - r) * 1.4 + 2.0`).
pub const PICKUP_VOLUME: f32 = 0.2;
pub const PICKUP_PITCH_SPREAD: f32 = 1.4;
pub const PICKUP_PITCH_CENTER: f32 = 2.0;
/// The render bob's own law (javap ItemEntityRenderer.submit:
/// `Mth.sin(ageInTicks / 10.0 + bobOffset) * 0.1 + 0.1`).
pub const BOB_AMPLITUDE: f32 = 0.1;
pub const BOB_BASE: f32 = 0.1;
pub const BOB_PERIOD: f32 = 10.0;
/// The spin law (javap ItemEntity.getSpin: `ageInTicks / 20.0 +
/// bobOffs`, the radians the rotY speaks).
pub const SPIN_PERIOD: f32 = 20.0;
/// The model's own base lift (javap: `-modelBoundingBox.minY + 0.0625`).
pub const BASE_LIFT_MIN: f32 = 0.0625;
/// The default block friction the drag folds (the palette's own — no
/// ice/slime in the vocabulary; body.rs's own constant).
pub const BLOCK_FRICTION: f64 = 0.6;
/// Entity.getAirDrag's own default (javap: 0.98).
pub const AIR_DRAG: f64 = 0.98;
/// The landing bounce (javap: on-ground && vy < 0 -> `multiply(1, -0.5, 1)`).
pub const LANDING_BOUNCE: f64 = -0.5;

/// The drop lane's own deterministic generator — the audio lane's LCG
/// (the same constants, the same seed scramble; the vanilla's
/// RandomSource stand-in the whole audio lane already speaks). Each
/// ItemDrop world carries ONE generator: the spawn spreads, the loot
/// rolls and the pickup pitches draw from it in tick order.
/// `Clone` the tests' own stream-position probes keep (the reference
/// replay the clone's state rides).
#[derive(Clone)]
pub struct Lcg {
    state: u64,
    /// The Marsaglia polar method's cached pair (javap
    /// `MarsagliaPolarGaussian`: the `haveNextNextGaussian` flag + the
    /// stored second draw — the death cloud's gaussian lane speaks
    /// them). `None` = no cached draw.
    next_gaussian_cache: Option<f64>,
}

impl Lcg {
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0x5EED_C0FF_EE12_3456,
            next_gaussian_cache: None,
        }
    }

    /// The cached-pair second draw (the flag's own truth).
    pub fn reset_gaussian(&mut self) {
        self.next_gaussian_cache = None;
    }

    /// The raw 48-bit draw (the mob lane's goal clocks draw the same
    /// stream the loot rolls speak).
    pub fn next(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(0x5851_F42D_4C95_7F2D)
            .wrapping_add(0x1405_7B7E_F767_814F);
        self.state >> 16
    }

    /// [0, 1) over 24 bits — the RandomSource.nextFloat shape.
    pub fn next_f32(&mut self) -> f32 {
        (self.next() % (1u64 << 24)) as f32 / (1u64 << 24) as f32
    }

    /// [0, 1) over 48 bits — the nextDouble shape.
    pub fn next_f64(&mut self) -> f64 {
        (self.next() % (1u64 << 48)) as f64 / (1u64 << 48) as f64
    }

    /// `MarsagliaPolarGaussian.nextGaussian` (javap verbatim): the
    /// cached pair returns first; otherwise the polar walk — two
    /// doubles mapped to [-1, 1), the `s >= 1 || s == 0` reject, the
    /// `sqrt(-2 * ln(s) / s)` fold — v1's product returns, v2's stores
    /// for the next call.
    pub fn next_gaussian(&mut self) -> f64 {
        if let Some(cached) = self.next_gaussian_cache.take() {
            return cached;
        }
        let (mut v1, mut v2, mut s);
        loop {
            v1 = 2.0 * self.next_f64() - 1.0;
            v2 = 2.0 * self.next_f64() - 1.0;
            s = v1 * v1 + v2 * v2;
            if s < 1.0 && s != 0.0 {
                break;
            }
        }
        let w = (-2.0 * s.ln() / s).sqrt();
        self.next_gaussian_cache = Some(v2 * w);
        v1 * w
    }

    /// `Mth.nextInt(random, min, max)` — the INCLUSIVE law (javap:
    /// `min + nextInt(max - min + 1)`; the degenerate range returns min).
    pub fn next_int_inclusive(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        min + (self.next() % (max - min + 1) as u64) as i32
    }

    /// `Mth.nextDouble(random, min, max)` (javap: `min + nextDouble() *
    /// (max - min)`; the degenerate range returns min).
    pub fn next_f64_between(&mut self, min: f64, max: f64) -> f64 {
        if min >= max {
            return min;
        }
        min + self.next_f64() * (max - min)
    }

    /// `RandomSource.nextInt(bound)` — the bound-draw law the ambient
    /// walk's gates speak (the 16/32/100/200 bounds the fluid's own
    /// ticks ride). The stand-in convention the audio lane's weighted
    /// pick already speaks: one raw draw mod the bound (the client's
    /// own LCG never spoke the Java rejection walk — the mod law IS
    /// the lane's stand-in). The zero bound stays silent (the vanilla
    /// would throw — the caller never asks).
    pub fn next_int(&mut self, bound: u64) -> i32 {
        if bound == 0 {
            return 0;
        }
        (self.next() % bound) as i32
    }
}

/// One item entity — the vanilla `ItemEntity`'s own fields (the pos is
/// the FEET-anchored center the entity protocol speaks, the velocity the
/// deltaMovement, the age/tickCount the two clocks the tick speaks).
#[derive(Debug, Clone)]
pub struct ItemDrop {
    /// The entity id (the settle law's phase partner — `(tickCount +
    /// id) % 4`; the offline lane's own counter, the wire ids stay the
    /// networked world's).
    pub id: i32,
    /// The stack: the wire item id and the count (the vanilla ItemStack).
    pub item: i32,
    pub count: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    /// The loot age (the despawn clock; -32768 = the never-despawn
    /// marker the vanilla's own sentinel speaks).
    pub age: i32,
    /// The tick count (the settle phase's own clock).
    pub ticks: u64,
    /// The pickup delay (the block drops arm 10; 32767 = the frozen).
    pub pickup_delay: i32,
    /// The render seed (`random.nextFloat() * PI * 2` at spawn).
    pub bob_offs: f32,
    pub on_ground: bool,
    /// The previous tick's position (javap `Entity.baseTick`'s own
    /// `xo = x` fold — the pre-move pose the render's lerp walks
    /// FROM). `None` = the just-spawned (no tick has walked it yet):
    /// the render shows the spawn point whole. The tick's own head
    /// saves it before the physics moves.
    pub prev: Option<(f64, f64, f64)>,
}

impl ItemDrop {
    /// The popResource spawn (javap Block.popResource): the block
    /// center, the ±0.25 spread per axis, the 0.125 height lift, the
    /// 10-tick delay, the render seed.
    pub fn spawn(id: i32, item: i32, count: i32, bx: i32, by: i32, bz: i32, rng: &mut Lcg) -> Self {
        Self {
            id,
            item,
            count,
            x: bx as f64 + 0.5 + rng.next_f64_between(-POP_SPREAD, POP_SPREAD),
            y: by as f64 + 0.5 + rng.next_f64_between(-POP_SPREAD, POP_SPREAD) - ITEM_HEIGHT / 2.0,
            z: bz as f64 + 0.5 + rng.next_f64_between(-POP_SPREAD, POP_SPREAD),
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            age: 0,
            ticks: 0,
            pickup_delay: POP_PICKUP_DELAY,
            bob_offs: rng.next_f32() * std::f32::consts::PI * 2.0,
            on_ground: false,
            prev: None,
        }
    }

    /// THE ENTITY LOOT's spawn (javap `Entity.spawnAtLocation` over the
    /// death-loot consumer: the ItemEntity rides the entity's OWN
    /// position (getX, getY, getZ) with a ZERO offset, the delta
    /// movement untouched (zero), `setDefaultPickUpDelay` (10), the
    /// bob seed the ItemEntity constructor's own
    /// `random.nextFloat() * PI * 2` draws).
    pub fn spawn_entity_loot(id: i32, item: i32, count: i32, x: f64, y: f64, z: f64, rng: &mut Lcg) -> Self {
        Self {
            id,
            item,
            count,
            x,
            y,
            z,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            age: 0,
            ticks: 0,
            pickup_delay: POP_PICKUP_DELAY,
            bob_offs: rng.next_f32() * std::f32::consts::PI * 2.0,
            on_ground: false,
            prev: None,
        }
    }

    /// The entity's bounding box (the 0.25 cube at the feet-anchored
    /// pos) — body.rs's own box shape.
    pub fn box_(&self) -> (f64, f64, f64, f64, f64, f64) {
        (self.x - HALF, self.y, self.z - HALF, self.x + HALF, self.y + ITEM_HEIGHT, self.z + HALF)
    }

    /// `ItemEntity.tick`'s own order, verbatim: the empty-stack
    /// discard, the delay decrement (the 32767 sentinel frozen), the
    /// water branch (the buoyancy lane the vanilla speaks — our
    /// vocabulary's full source cells pass the >0.1 gate), the gravity
    /// 0.04, the no-collision un-stuck, the settle skip, the Y-first
    /// sweep, the drag, the landing bounce, the clocks. Returns false
    /// when the entity discards (the empty stack, the despawn age).
    pub fn tick(&mut self, world: &World) -> bool {
        // the tick's own head (javap `Entity.baseTick`'s first fold —
        // `xo = x` rides BEFORE the physics): the current pose becomes
        // the previous, the render's lerp then walks the frame between
        // the two
        self.prev = Some((self.x, self.y, self.z));
        if self.count <= 0 {
            return false;
        }
        if self.pickup_delay > 0 && self.pickup_delay != 32767 {
            self.pickup_delay -= 1;
        }
        // the fluid branch: the item's own cell is the honest stand-in
        // for the vanilla's getFluidHeight > 0.1 gate (our source cells
        // speak the full column)
        let in_water = world.get(
            self.x.floor() as i32,
            self.y.floor() as i32,
            self.z.floor() as i32,
        ) == crate::world::WATER;
        if in_water {
            // setUnderwaterMovement's shape: the buoyant lift with the
            // water drag (the palette has no lava — that branch never
            // fires in our vocabulary)
            self.vx *= 0.9;
            self.vz *= 0.9;
            self.vy = self.vy * 0.8 + 0.05;
        } else {
            // applyGravity -> ItemEntity.getDefaultGravity (javap: 0.04)
            self.vy -= 0.04;
        }
        // the no-collision un-stuck: the box deflated by the vanilla's
        // 1e-7 must not overlap (moveTowardsClosestSpace's own trigger);
        // our vocabulary's honest push-out: the nearest non-solid
        // neighbour cell's center along x/z (the y keeps its lane)
        let b = self.box_();
        if box_overlaps_solid(world, b, 1.0e-7) {
            self.move_towards_closest_space(world);
        }
        // the settle skip: on ground, effectively still, and the phase
        // says so — the move does not run (the vanilla's own breathing;
        // the flip below still speaks — the resting item's own hop)
        let still = self.vx * self.vx + self.vz * self.vz < 1.0e-5;
        let skip_move = self.on_ground
            && still
            && (self.ticks.wrapping_add(self.id as u64)) % SETTLE_PERIOD != 0;
        if !skip_move {
            let c: Collision = collide(world, self.box_(), (self.vx, self.vy, self.vz));
            self.x += c.dx;
            self.y += c.dy;
            self.z += c.dz;
            // the vanilla move's restitution: the collided axes zero
            // (the non-bouncy entity's own verdict)
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
        }
        // the drag: x/z fold the ground friction on the floor lane,
        // y folds the air drag alone (javap: multiply(f3, f2, f3))
        let f3 = if self.on_ground { AIR_DRAG * BLOCK_FRICTION } else { AIR_DRAG };
        self.vx *= f3;
        self.vy *= AIR_DRAG;
        self.vz *= f3;
        // the landing bounce: the fresh floor contact flips the descent
        if self.on_ground && self.vy < 0.0 {
            self.vy *= LANDING_BOUNCE;
        }
        // the clocks
        if self.age != -32768 {
            self.age += 1;
        }
        self.ticks = self.ticks.wrapping_add(1);
        if self.age >= DESPAWN_AGE {
            return false;
        }
        true
    }

    /// THE RENDER FOLD (javap `EntityRenderDispatcher`'s own lerp —
    /// the `Mth.lerp(partialTicks, xo, x)` law every entity render
    /// speaks): the pose the frame rides — the previous tick's
    /// position walked `partial` of the way to the current. `None`
    /// prev (the just-spawned) renders the spawn point whole. The
    /// partial >= 1.0 arm speaks the CURRENT tick position bit for
    /// bit (the headless lanes' own default — the goldens'
    /// byte-identical contract).
    pub fn lerped_pos(&self, partial: f32) -> (f64, f64, f64) {
        let Some((px, py, pz)) = self.prev else {
            return (self.x, self.y, self.z);
        };
        if partial >= 1.0 {
            return (self.x, self.y, self.z);
        }
        let t = partial as f64;
        (
            px + (self.x - px) * t,
            py + (self.y - py) * t,
            pz + (self.z - pz) * t,
        )
    }

    /// The un-stuck push (moveTowardsClosestSpace's own shape): the
    /// nearest non-solid neighbour cell's center along x/z, the y
    /// lifted one cell when the own cell is buried.
    fn move_towards_closest_space(&mut self, world: &World) {
        let (cx, cy, cz) = (
            self.x.floor() as i32,
            self.y.floor() as i32,
            self.z.floor() as i32,
        );
        let solid_here = crate::body::solid(world, cx, cy, cz);
        let mut best: Option<(f64, f64)> = None;
        let mut best_d = f64::INFINITY;
        for dx in -1..=1 {
            for dz in -1..=1 {
                if dx == 0 && dz == 0 {
                    continue;
                }
                if crate::body::solid(world, cx + dx, cy, cz + dz) {
                    continue;
                }
                let (tx, tz) = ((cx + dx) as f64 + 0.5, (cz + dz) as f64 + 0.5);
                let d = (tx - self.x).powi(2) + (tz - self.z).powi(2);
                if d < best_d {
                    best_d = d;
                    best = Some((tx, tz));
                }
            }
        }
        if solid_here {
            // the buried lane: the cell above's floor is the honest lift
            self.y = (cy + 1) as f64;
        }
        if let Some((tx, tz)) = best {
            self.x = tx;
            self.z = tz;
        }
    }
}

/// The box-vs-world overlap the no-collision check speaks (the deflate
/// rides the caller's own epsilon).
fn box_overlaps_solid(world: &World, b: (f64, f64, f64, f64, f64, f64), deflate: f64) -> bool {
    let (x0, y0, z0, x1, y1, z1) = (b.0 + deflate, b.1 + deflate, b.2 + deflate, b.3 - deflate, b.4 - deflate, b.5 - deflate);
    for cx in x0.floor() as i32..=(x1 - 1.0e-9).floor() as i32 {
        for cy in y0.floor() as i32..=(y1 - 1.0e-9).floor() as i32 {
            for cz in z0.floor() as i32..=(z1 - 1.0e-9).floor() as i32 {
                if crate::body::solid(world, cx, cy, cz) {
                    return true;
                }
            }
        }
    }
    false
}

/// `isMergable`'s own law (javap): the stack fits one more of its kind
/// and the two stacks share the item id (the components' honest
/// stand-in — the client's stacks carry the id alone).
fn mergeable(a: &ItemDrop, b: &ItemDrop) -> bool {
    a.item == b.item && a.count > 0 && b.count > 0 && a.count + b.count <= 64
}

/// The neighbour merge (javap mergeWithNeighbours): the closest
/// same-item neighbour within `inflate(0.5, 0.5, 0.5)` absorbs — the
/// vanishing entity's own discard. The whole list passes (the App's own
/// Vec); the cadence gate rides each entity's own tickCount (the
/// vanilla's %40 inside the tick).
pub fn merge_pass(drops: &mut Vec<ItemDrop>) {
    let n = drops.len();
    for i in 0..n {
        // the entity's own cadence (the vanilla's merge check speaks
        // every MERGE_PERIOD-th tick of ITS OWN clock)
        if drops[i].ticks % MERGE_PERIOD != 0 {
            continue;
        }
        let Some(self_ref) = drops.get(i) else { continue };
        if self_ref.count <= 0 || self_ref.count >= 64 {
            continue;
        }
        let b = self_ref.box_();
        let (bx0, by0, bz0, bx1, by1, bz1) = (
            b.0 - MERGE_INFLATE,
            b.1 - MERGE_INFLATE,
            b.2 - MERGE_INFLATE,
            b.3 + MERGE_INFLATE,
            b.4 + MERGE_INFLATE,
            b.5 + MERGE_INFLATE,
        );
        // the closest eligible neighbour
        let mut best: Option<usize> = None;
        let mut best_d = f64::INFINITY;
        for j in 0..n {
            if j == i {
                continue;
            }
            let other = match drops.get(j) {
                Some(o) => o,
                None => continue,
            };
            if !mergeable(self_ref, other) {
                continue;
            }
            let (ox, oy, oz) = (other.x, other.y, other.z);
            if ox < bx0 || ox > bx1 || oy < by0 || oy > by1 || oz < bz0 || oz > bz1 {
                continue;
            }
            let d = (ox - self_ref.x).powi(2) + (oy - self_ref.y).powi(2) + (oz - self_ref.z).powi(2);
            if d < best_d {
                best_d = d;
                best = Some(j);
            }
        }
        if let Some(j) = best {
            let other = drops.get_mut(j).unwrap();
            let (oc, _oid) = (other.count, other.item);
            other.count = 0; // the absorbed entity discards
            let _ = _oid;
            let self_mut = drops.get_mut(i).unwrap();
            self_mut.count += oc;
        }
    }
    drops.retain(|d| d.count > 0);
}

/// `Inventory`'s own shape (javap Inventory): the THREE lists the
/// container's own indices speak — 36 `items` (0..8 the hotbar, 9..35
/// the main), 4 `armor` (the SLOT_IDS loop's own order HEAD, CHEST,
/// LEGS, FEET — the ctor walk's container indices 39-k), 1 `offhand`
/// (container 40); the stacks (id, count) with count <= 0 the empty.
/// The armor list carries NO components — the curse-of-binding gate
/// (`EnchantmentHelper.has(PREVENT_ARMOR_CHANGE)` the ArmorSlot.
/// mayPickup walk keeps) never speaks against a bare (id, count),
/// `mayPickup` answers true (the store's own honest shape).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inventory {
    pub items: [(i32, i32); 36],
    pub armor: [(i32, i32); 4],
    pub offhand: [(i32, i32); 1],
}

/// The offhand's container index (the `getItem(40)` the pickup walk
/// speaks — the three-list fold's own tail).
pub const OFFHAND_CONTAINER: usize = 40;

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: [(0, 0); 36],
            armor: [(0, 0); 4],
            offhand: [(0, 0); 1],
        }
    }
}

impl Inventory {
    /// `hasRemainingSpaceForItem` (javap): the same item, stackable,
    /// and the count below the max (the client's stacks carry the id
    /// alone — the components' comparison collapses to the id).
    fn has_remaining_space(item: (i32, i32), id: i32) -> bool {
        item.1 > 0 && item.0 == id && item.1 < 64
    }

    /// `getSlotWithRemainingSpace` (javap): the SELECTED hotbar slot
    /// first, then the OFFHAND container 40 (the `getItem(40)` arm the
    /// bytecode 21-37 speaks), then the full items scan — the armor
    /// list the walk never consults (the pickup's own lanes answer).
    fn get_slot_with_remaining_space(&self, selected: usize, id: i32) -> Option<usize> {
        if Self::has_remaining_space(self.items[selected], id) {
            return Some(selected);
        }
        if Self::has_remaining_space(self.offhand[0], id) {
            return Some(OFFHAND_CONTAINER);
        }
        (0..36).find(|&i| Self::has_remaining_space(self.items[i], id))
    }

    /// `getFreeSlot` (javap): the first empty in the items order.
    fn get_free_slot(&self) -> Option<usize> {
        (0..36).find(|&i| self.items[i].1 <= 0)
    }

    /// `addResource(slot)` (javap): the empty slot takes the id, the
    /// fill rides `min(incoming, max - count)`, the leftover returns.
    /// The offhand's container 40 routes to its own list (the
    /// remaining-space walk's slot-40 verdict lands here).
    fn add_resource(&mut self, slot: usize, id: i32, incoming: i32) -> i32 {
        let offhand = slot == OFFHAND_CONTAINER;
        let cur = if offhand {
            self.offhand[0]
        } else {
            self.items[slot]
        };
        if cur.1 <= 0 {
            if offhand {
                self.offhand[0] = (id, 0);
            } else {
                self.items[slot] = (id, 0);
            }
        }
        let count = if offhand {
            self.offhand[0].1
        } else {
            self.items[slot].1
        };
        let space = 64 - count;
        let take = incoming.min(space);
        if take == 0 {
            return incoming;
        }
        if offhand {
            self.offhand[0].1 += take;
        } else {
            self.items[slot].1 += take;
        }
        incoming - take
    }

    /// `Inventory.add` (javap): the loop re-locates per pass (the merge
    /// stack fills, the next pass finds the next home) and stops when a
    /// pass makes no progress. Returns the LEFTOVER count (the caller
    /// speaks the pickup when it shrinks).
    pub fn add(&mut self, selected: usize, id: i32, count: i32) -> i32 {
        if count <= 0 {
            return 0;
        }
        let mut remaining = count;
        loop {
            let before = remaining;
            let slot = self
                .get_slot_with_remaining_space(selected, id)
                .or_else(|| self.get_free_slot());
            let Some(slot) = slot else {
                return remaining;
            };
            remaining = self.add_resource(slot, id, remaining);
            if remaining <= 0 {
                return 0;
            }
            if remaining >= before {
                return remaining;
            }
        }
    }

    /// The hotbar row's projection (the HUD's own (id, count) pairs —
    /// count <= 0 the empty sentinel).
    pub fn hotbar(&self) -> [(i32, i32); 9] {
        let mut out = [(0, 0); 9];
        out.copy_from_slice(&self.items[0..9]);
        out
    }
}

/// The loot roll — the generated hand-lane table's own consumer. The
/// TOOL GATE answers first (the vanilla drops nothing when
/// `canHarvestBlock` fails — the dig lane's `correct_tool` verdict),
/// then each pool walks its case chain IN ORDER (the generated table's
/// own eval law): the definite pass wins the pool, the roll beats its
/// chance, the fail moves on. The win rolls the count through the
/// INCLUSIVE int law and names the item by its wire id. `None` ids (a
/// table row outside the wire vocabulary) drop nothing — the honest
/// absence.
pub fn roll_drops(kind: u16, rng: &mut Lcg) -> Vec<(i32, i32)> {
    let name = crate::world::NAME_OF[kind as usize];
    if !crate::dig::DigState::correct_tool(kind) {
        return Vec::new();
    }
    let Some(pools) = mc_registry::generated::block_drops::drops(name) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for pool in pools {
        for entry in pool.entries {
            if entry.chance < 1.0 {
                let roll = rng.next_f32();
                if roll >= entry.chance {
                    continue; // the case fails; the next row speaks
                }
            }
            // the win: the inclusive count roll
            let count = rng.next_int_inclusive(entry.count_min, entry.count_max);
            if let Some(id) = mc_registry::item_id_by_name(entry.item) {
                out.push((id, count));
            }
            break; // the pool spoke
        }
    }
    out
}

/// The pickup outcome (the caller's own lanes: the sound beat, the hotbar
/// refresh).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Touch {
    /// The delay still holds (or the stack vanished) — nothing spoke.
    Idle,
    /// The inventory took `added` of the stack (the pickup voice's own
    /// beat — the ORIGINAL count the take law names).
    Picked { added: i32 },
}

/// `ItemEntity.playerTouch`'s own law (javap): the delay gate, the
/// target gate (the block drops carry none), the inventory add, the
/// partial count's honest shrink, the empty stack's discard (the
/// caller's retain).
pub fn player_touch(
    drop: &mut ItemDrop,
    inventory: &mut Inventory,
    selected: usize,
) -> Touch {
    if drop.pickup_delay != 0 || drop.count <= 0 {
        return Touch::Idle;
    }
    let original = drop.count;
    let leftover = inventory.add(selected, drop.item, drop.count);
    if leftover >= original {
        return Touch::Idle; // no room — the vanilla's own verdict (no voice)
    }
    drop.count = leftover;
    Touch::Picked {
        added: original - leftover,
    }
}

/// The aiStep scan (javap Player.aiStep, the no-vehicle branch): the
/// player's own bounding box inflated `(1.0, 0.5, 1.0)` against the
/// item's box. The crouch pose's own height rides (the vanilla's
/// getBoundingBox speaks the CURRENT pose).
pub fn pickup_scan(body: &crate::body::PlayerBody, drop: &ItemDrop) -> bool {
    let height = if body.crouching {
        crate::body::CROUCH_HEIGHT
    } else {
        crate::body::STANDING_HEIGHT
    };
    let hw = crate::body::WIDTH / 2.0;
    let (inf_x, inf_y, inf_z) = PICKUP_INFLATE;
    let (px0, py0, pz0) = (body.x - hw - inf_x, body.y - inf_y, body.z - hw - inf_z);
    let (px1, py1, pz1) = (body.x + hw + inf_x, body.y + height + inf_y, body.z + hw + inf_z);
    let b = drop.box_();
    b.0 < px1 && b.3 > px0 && b.1 < py1 && b.4 > py0 && b.2 < pz1 && b.5 > pz0
}

/// The pickup voice's pitch law (javap handleTakeItemEntity's
/// ITEM_PICKUP branch: `(r - r) * 1.4 + 2.0`).
pub fn pickup_pitch(r1: f32, r2: f32) -> f32 {
    (r1 - r2) * PICKUP_PITCH_SPREAD + PICKUP_PITCH_CENTER
}

/// The render bob (javap ItemEntityRenderer.submit).
pub fn bob(age: f32, bob_offs: f32) -> f32 {
    (age / BOB_PERIOD + bob_offs).sin() * BOB_AMPLITUDE + BOB_BASE
}

/// The render spin, radians (javap ItemEntity.getSpin).
pub fn spin(age: f32, bob_offs: f32) -> f32 {
    age / SPIN_PERIOD + bob_offs
}

/// The model's own base lift (javap: `-minY + 0.0625` — the posed
/// quads' actual minimum feeds the call).
pub fn base_lift(model_min_y: f32) -> f32 {
    -model_min_y + BASE_LIFT_MIN
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat_world() -> World {
        // 2x2 chunks, seed 42 — the plain procedural terrain; the tests
        // carve their own cells where they need them
        World::new(2, 42)
    }

    // ---------- the LCG laws ----------

    #[test]
    fn lcg_range_laws() {
        let mut r = Lcg::new(7);
        for _ in 0..256 {
            let f = r.next_f32();
            assert!((0.0..1.0).contains(&f));
            let d = r.next_f64();
            assert!((0.0..1.0).contains(&d));
        }
        // the inclusive int law: every bound rides (javap Mth.nextInt)
        let mut r = Lcg::new(99);
        let mut seen_min = false;
        let mut seen_max = false;
        for _ in 0..2048 {
            let v = r.next_int_inclusive(2, 5);
            assert!((2..=5).contains(&v));
            seen_min |= v == 2;
            seen_max |= v == 5;
        }
        assert!(seen_min && seen_max, "the inclusive span speaks both ends");
        // the degenerate range returns min (the vanilla's own guard)
        assert_eq!(Lcg::new(1).next_int_inclusive(3, 3), 3);
        // the signed spread: the ±0.25 span (the popResource law's own)
        let mut r = Lcg::new(3);
        for _ in 0..512 {
            let v = r.next_f64_between(-0.25, 0.25);
            assert!((-0.25..0.25).contains(&v));
        }
    }

    // ---------- the spawn law ----------

    #[test]
    fn spawn_rides_pop_resource_law() {
        // javap Block.popResource: x = bx + 0.5 + nextDouble(-0.25, 0.25),
        // y = by + 0.5 + spread - 0.125, z = bz + 0.5 + spread, the delay
        // 10, the bob seed nextFloat() * PI * 2
        let mut rng = Lcg::new(1234);
        let d = ItemDrop::spawn(5, 42, 1, 10, 20, 30, &mut rng);
        assert!(d.x > 10.25 && d.x < 10.75);
        assert!(d.y > 20.125 && d.y < 20.625, "the 0.5 center + spread - the 0.125 lift");
        assert!(d.z > 30.25 && d.z < 30.75);
        assert_eq!(d.pickup_delay, POP_PICKUP_DELAY);
        assert!(d.bob_offs >= 0.0 && d.bob_offs < std::f32::consts::PI * 2.0);
        assert_eq!(d.id, 5);
        assert_eq!(d.item, 42);
        assert_eq!(d.count, 1);
        assert_eq!(d.age, 0);
    }

    // ---------- the physics laws ----------

    #[test]
    fn gravity_and_drag_laws() {
        // the free fall: the gravity 0.04 rides BEFORE the move, the air
        // drag 0.98 folds AFTER — vy1 = (0 - 0.04) * 0.98 = -0.0392
        let world = flat_world();
        let mut d = ItemDrop::spawn(1, 1, 1, 8, 100, 8, &mut Lcg::new(1));
        // lift the item into the sky (the procedural world's own height)
        d.y = 60.0;
        d.tick(&world);
        assert!((d.vy - (-0.04f64 * 0.98)).abs() < 1e-12, "gravity then drag: {}", d.vy);
        // the second tick: the carried velocity rides (vy2 = (vy1 - 0.04) * 0.98)
        d.on_ground = false;
        d.tick(&world);
        assert!((d.vy - ((-0.04 * 0.98 - 0.04) * 0.98)).abs() < 1e-12);
        // the fall moved the pos DOWN by the carried amount (the sweep's
        // own allowed = the full dy in open air)
        assert!(d.y < 59.9);
    }

    #[test]
    fn landing_bounce_and_ground_drag() {
        // the floor contact: the descent flips -0.5, the ground drag
        // folds 0.98 * 0.6 = 0.588 on the horizontals
        let world = flat_world();
        // the item lands on the terrain's own surface
        let sx = 8;
        let mut sy = 40;
        while sy > 0 && !crate::body::solid(&world, sx, sy - 1, 8) {
            sy -= 1;
        }
        let mut d = ItemDrop::spawn(1, 1, 1, sx, sy, 8, &mut Lcg::new(2));
        d.y = sy as f64; // the feet exactly on the surface
        d.pickup_delay = 0;
        // a descending velocity: the tick sweeps to the contact and the
        // flip fires on the fresh floor
        d.vy = -0.5;
        let mut bounced = false;
        for _ in 0..20 {
            d.tick(&world);
            if d.on_ground && d.vy > 0.0 {
                bounced = true;
                break;
            }
        }
        assert!(bounced, "the landing hop the vanilla items speak");
        // the settled item: the horizontals decay toward zero (the
        // 0.588 fold per tick)
        for _ in 0..40 {
            d.tick(&world);
        }
        assert!(d.vx.abs() < 1e-6 && d.vz.abs() < 1e-6);
    }

    #[test]
    fn settle_skip_breathes_every_fourth_tick() {
        // the settled item's move rides only when (ticks + id) % 4 == 0:
        // the y never drifts far (the flip's own hop is the only lift,
        // the skip holds the descent), and the phase tracks the ticks
        let world = flat_world();
        let sx = 8;
        let mut sy = 40;
        while sy > 0 && !crate::body::solid(&world, sx, sy - 1, 8) {
            sy -= 1;
        }
        let mut d = ItemDrop::spawn(3, 1, 1, sx, sy, 8, &mut Lcg::new(4));
        d.y = sy as f64;
        d.on_ground = true;
        let y0 = d.y;
        for _ in 0..12 {
            d.tick(&world);
        }
        // the settled rhythm: the y stays within the flip's own tiny hop
        assert!((d.y - y0).abs() < 0.01, "the settle holds the rest: {} vs {}", d.y, y0);
        // the phase: (ticks + id) % 4 — the skip's own arithmetic
        assert_eq!(d.ticks.wrapping_add(d.id as u64) % SETTLE_PERIOD, (12 + 3) % 4);
    }

    #[test]
    fn water_buoyancy_lifts() {
        // the water cell's own lane: the buoyant lift (vy -> *0.8 + 0.05)
        let mut world = flat_world();
        // carve a pool: a water cell at the item's spawn
        let (wx, wy, wz) = (8, 20, 8);
        world.set(wx, wy, wz, crate::world::WATER);
        let mut d = ItemDrop::spawn(1, 1, 1, wx, wy, wz, &mut Lcg::new(5));
        d.x = wx as f64 + 0.5;
        d.y = wy as f64 + 0.1;
        d.z = wz as f64 + 0.5;
        d.vy = -0.2;
        d.tick(&world);
        // the water lane: no gravity fold — the buoyancy spoke, and the
        // drag folded after (the vanilla's unconditional multiply)
        let expected = (-0.2f64 * 0.8 + 0.05) * AIR_DRAG;
        assert!((d.vy - expected).abs() < 1e-12, "vy = {}", d.vy);
    }

    #[test]
    fn despawn_after_six_thousand_ticks() {
        let world = flat_world();
        let mut d = ItemDrop::spawn(1, 1, 1, 8, 60, 8, &mut Lcg::new(6));
        d.age = DESPAWN_AGE - 2;
        assert!(d.tick(&world), "age 5999 keeps");
        assert_eq!(d.age, DESPAWN_AGE - 1);
        // the tick that CROSSES 6000 discards in the same tick (the
        // vanilla's own order: the age++ then the check)
        assert!(!d.tick(&world), "the despawn age discards");
        // the never-despawn sentinel
        let mut d = ItemDrop::spawn(2, 1, 1, 8, 60, 8, &mut Lcg::new(7));
        d.age = -32768;
        for _ in 0..7000 {
            assert!(d.tick(&world), "the sentinel keeps forever");
            d.age = -32768;
        }
    }

    #[test]
    fn empty_stack_discards_at_once() {
        let world = flat_world();
        let mut d = ItemDrop::spawn(1, 1, 0, 8, 60, 8, &mut Lcg::new(8));
        assert!(!d.tick(&world), "the empty stack discards");
    }

    // ---------- the merge law ----------

    #[test]
    fn merge_absorbs_closest_same_item() {
        let mut a = ItemDrop::spawn(1, 42, 30, 0, 0, 0, &mut Lcg::new(9));
        let mut b = ItemDrop::spawn(2, 42, 20, 0, 0, 0, &mut Lcg::new(10));
        let mut c = ItemDrop::spawn(3, 7, 5, 0, 0, 0, &mut Lcg::new(11));
        a.x = 100.0;
        a.y = 60.0;
        a.z = 100.0;
        b.x = 100.2; // within the 0.5 inflate
        b.y = 60.0;
        b.z = 100.0;
        c.x = 100.3;
        c.y = 60.0;
        c.z = 100.0;
        let mut drops = vec![a, b, c];
        merge_pass(&mut drops);
        assert_eq!(drops.len(), 2);
        // the same-item pair merged (30 + 20 fits one stack)
        let m = drops.iter().find(|d| d.item == 42).unwrap();
        assert_eq!(m.count, 50);
        // the different item stands alone
        assert!(drops.iter().any(|d| d.item == 7 && d.count == 5));
    }

    #[test]
    fn merge_refuses_overflow() {
        // 50 + 30 exceeds the 64 cap — the vanilla's own areMergable refuses
        let mut a = ItemDrop::spawn(1, 42, 50, 0, 0, 0, &mut Lcg::new(12));
        let mut b = ItemDrop::spawn(2, 42, 30, 0, 0, 0, &mut Lcg::new(13));
        a.x = 100.0;
        a.y = 60.0;
        a.z = 100.0;
        b.x = 100.1;
        b.y = 60.0;
        b.z = 100.0;
        let mut drops = vec![a, b];
        merge_pass(&mut drops);
        assert_eq!(drops.len(), 2, "the overflow refuses the merge");
    }

    // ---------- the inventory laws ----------

    #[test]
    fn add_prefers_selected_slot_then_scan_then_free() {
        let mut inv = Inventory::default();
        // the selected slot holds 30 dirt, slot 5 holds 60 dirt
        inv.items[0] = (7, 30);
        inv.items[5] = (7, 60);
        // 40 dirt: the selected merges to 64, the scan finds slot 5 for the rest
        let leftover = inv.add(0, 7, 40);
        // the honest walk: the selected takes min(40, 34) = 34 -> 64; the
        // next pass scans slot 5 (60): min(6, 4) = 4 -> 64; the third
        // pass finds no merge space and opens the first free slot (1)
        // for the last 2 — nothing left over
        assert_eq!(inv.items[0], (7, 64));
        assert_eq!(inv.items[5], (7, 64));
        assert_eq!(inv.items[1], (7, 2), "the overflow opens the first free slot");
        assert_eq!(leftover, 0);
        // the whole 64 stack in an empty inventory: getFreeSlot scans from
        // slot 0 (the vanilla's own order — NOT the selected slot)
        let mut inv = Inventory::default();
        let leftover = inv.add(3, 9, 64);
        assert_eq!(leftover, 0);
        assert_eq!(inv.items[0], (9, 64), "the free-slot scan starts at slot 0");
    }

    #[test]
    fn add_refuses_when_full() {
        let mut inv = Inventory::default();
        for slot in inv.items.iter_mut() {
            *slot = (5, 64);
        }
        let leftover = inv.add(0, 5, 3);
        assert_eq!(leftover, 3, "the full inventory returns the whole count");
        // a full inventory refuses a DIFFERENT item the same way
        let leftover = inv.add(0, 6, 1);
        assert_eq!(leftover, 1);
    }

    #[test]
    fn add_merge_never_exceeds_cap() {
        let mut inv = Inventory::default();
        inv.items[2] = (7, 62);
        let leftover = inv.add(0, 7, 5);
        // the selected (0) is empty — the merge scan finds slot 2: min(5, 2)
        // = 2 -> 64; the leftover 3 opens slot 0 (the selected, the first
        // free in the scan order)
        assert_eq!(inv.items[2], (7, 64));
        assert_eq!(inv.items[0], (7, 3));
        assert_eq!(leftover, 0);
    }

    #[test]
    fn hotbar_projection() {
        let mut inv = Inventory::default();
        inv.items[0] = (7, 3);
        inv.items[8] = (9, 1);
        let hb = inv.hotbar();
        assert_eq!(hb[0], (7, 3));
        assert_eq!(hb[8], (9, 1));
        assert_eq!(hb[4], (0, 0));
    }

    // ---------- the pickup laws ----------

    #[test]
    fn pickup_delay_gates_the_touch() {
        let mut inv = Inventory::default();
        let mut d = ItemDrop::spawn(1, 7, 1, 0, 0, 0, &mut Lcg::new(14));
        assert_eq!(player_touch(&mut d, &mut inv, 0), Touch::Idle, "the 10-tick delay holds");
        d.pickup_delay = 0;
        assert_eq!(
            player_touch(&mut d, &mut inv, 0),
            Touch::Picked { added: 1 },
            "the delay gone, the inventory speaks"
        );
        assert_eq!(d.count, 0, "the stack emptied");
        assert_eq!(inv.items[0], (7, 1));
    }

    #[test]
    fn pickup_partial_shrink_keeps_the_entity() {
        // the inventory near-full: 70 dirt -> 64 in, 6 stay on the entity
        let mut inv = Inventory::default();
        inv.items[1] = (7, 60);
        let mut d = ItemDrop::spawn(1, 7, 70, 0, 0, 0, &mut Lcg::new(15));
        d.pickup_delay = 0; // the delay's own gate is tested separately
        // fill the rest of the inventory with OTHER items so the merge
        // lands only on slot 1 and one free slot takes 4
        for i in (2..36).step_by(2) {
            inv.items[i] = (9, 1);
        }
        for i in (3..36).step_by(2) {
            inv.items[i] = (8, 1);
        }
        inv.items[0] = (5, 64);
        let t = player_touch(&mut d, &mut inv, 0);
        match t {
            Touch::Picked { added } => {
                assert!(added > 0);
                assert!(d.count > 0, "the entity keeps the leftover");
            }
            Touch::Idle => panic!("the pickup must speak"),
        }
    }

    #[test]
    fn pickup_scan_inflate_law() {
        // the aiStep box: the player's AABB inflated (1.0, 0.5, 1.0)
        let mut body = crate::body::PlayerBody::new(100.0, 60.0, 100.0, 0.0, 0.0);
        body.crouching = false;
        // the box: x ±(0.3 + 1.0), y 60 - 0.5 .. 61.8 + 0.5, z ±1.3
        let mut d = ItemDrop::spawn(1, 1, 1, 0, 0, 0, &mut Lcg::new(16));
        d.x = 102.29; // 100 + 1.3 = 101.3 is the edge — 102.29 is OUTSIDE? no:
        // the inflate reaches x = 101.3; 102.29 misses
        d.y = 60.0;
        d.z = 100.0;
        // the item's own 0.25 box: x 102.165..102.415 — beyond 101.3: MISS
        assert!(!pickup_scan(&body, &d), "beyond the inflate");
        d.x = 101.4; // the item's box x 101.275..101.525 — overlaps 101.3
        assert!(pickup_scan(&body, &d), "inside the inflate");
        // the vertical: the inflate reaches y 59.5..62.3
        d.y = 62.4;
        assert!(!pickup_scan(&body, &d), "above the inflate");
        d.y = 62.2;
        assert!(pickup_scan(&body, &d), "inside the vertical reach");
    }

    // ---------- the roll laws ----------

    #[test]
    fn tool_gate_refuses_the_stone_family() {
        // the bare hand cannot harvest: the vanilla drops NOTHING
        let mut rng = Lcg::new(17);
        assert!(roll_drops(crate::world::STONE, &mut rng).is_empty(), "stone demands the pickaxe");
        assert!(roll_drops(crate::world::GRANITE, &mut rng).is_empty());
        assert!(roll_drops(crate::world::DEEPSLATE, &mut rng).is_empty());
        assert!(roll_drops(crate::world::COAL_ORE, &mut rng).is_empty());
        assert!(roll_drops(crate::world::NETHERRACK, &mut rng).is_empty());
        assert!(roll_drops(crate::world::BEDROCK, &mut rng).is_empty(), "bedrock's -1 hardness never harvests");
    }

    #[test]
    fn simple_drops_speak_the_table() {
        let mut rng = Lcg::new(18);
        // dirt -> dirt; grass -> dirt; oak_log -> oak_log; sand -> sand
        let drops = roll_drops(crate::world::DIRT, &mut rng);
        assert_eq!(drops, vec![(mc_registry::item_id_by_name("dirt").unwrap(), 1)]);
        let drops = roll_drops(crate::world::GRASS, &mut rng);
        assert_eq!(drops, vec![(mc_registry::item_id_by_name("dirt").unwrap(), 1)], "the grass block drops dirt");
        let drops = roll_drops(crate::world::LOG, &mut rng);
        assert_eq!(drops, vec![(mc_registry::item_id_by_name("oak_log").unwrap(), 1)]);
        let drops = roll_drops(crate::world::SAND, &mut rng);
        assert_eq!(drops, vec![(mc_registry::item_id_by_name("sand").unwrap(), 1)]);
    }

    #[test]
    fn ore_counts_speak_the_inclusive_span() {
        // the TOOL GATE first: the bare hand cannot harvest the ores —
        // the vanilla drops NOTHING (the pickaxe tables ride a future
        // round; the generated table carries the counts verbatim)
        let mut rng = Lcg::new(19);
        assert!(roll_drops(crate::world::COPPER_ORE, &mut rng).is_empty());
        assert!(roll_drops(crate::world::REDSTONE_ORE, &mut rng).is_empty());
        assert!(roll_drops(crate::world::DIAMOND_ORE, &mut rng).is_empty());
        // the generated table's own rows: the INCLUSIVE count spans the
        // loot JSONs speak (javap Mth.nextInt)
        let pools = mc_registry::generated::block_drops::drops("copper_ore").unwrap();
        let e = &pools[0].entries[0];
        assert_eq!(e.item, "raw_copper");
        assert_eq!((e.count_min, e.count_max), (2, 5));
        let e = &mc_registry::generated::block_drops::drops("redstone_ore").unwrap()[0].entries[0];
        assert_eq!((e.count_min, e.count_max), (4, 5));
        let e = &mc_registry::generated::block_drops::drops("lapis_ore").unwrap()[0].entries[0];
        assert_eq!((e.count_min, e.count_max), (4, 9));
        let e = &mc_registry::generated::block_drops::drops("diamond_ore").unwrap()[0].entries[0];
        assert_eq!(e.item, "diamond");
        assert_eq!((e.count_min, e.count_max), (1, 1));
    }

    #[test]
    fn gravel_flint_chain_speaks_both_cases() {
        // the case chain: the flint's 10% roll, then the gravel fallback —
        // a seeded LCG drives BOTH branches deterministically
        let mut flint = 0;
        let mut gravel = 0;
        let mut rng = Lcg::new(23);
        for _ in 0..4096 {
            let drops = roll_drops(crate::world::GRAVEL, &mut rng);
            assert_eq!(drops.len(), 1, "one case wins the pool");
            if drops[0].0 == mc_registry::item_id_by_name("flint").unwrap() {
                flint += 1;
            } else {
                assert_eq!(drops[0].0, mc_registry::item_id_by_name("gravel").unwrap());
                gravel += 1;
            }
        }
        // the 10% gate: the flint lands near 409 of 4096 (the honest
        // statistical pin — the LCG's uniformity, never the exact count)
        assert!(flint > 300 && flint < 520, "the flint rate {flint}/4096");
        assert_eq!(flint + gravel, 4096);
    }

    #[test]
    fn leaves_pools_roll_independently() {
        // the three pools fire independently: over many breaks, the
        // sapling (5%), the sticks (2%, 1..2) and the apples (0.5%)
        // each speak, and SOME breaks speak more than one at once
        let mut rng = Lcg::new(24);
        let mut sapling = 0;
        let mut sticks = 0;
        let mut apples = 0;
        let mut multi = 0;
        let sap_id = mc_registry::item_id_by_name("oak_sapling").unwrap();
        let stick_id = mc_registry::item_id_by_name("stick").unwrap();
        let apple_id = mc_registry::item_id_by_name("apple").unwrap();
        for _ in 0..8192 {
            let drops = roll_drops(crate::world::LEAVES, &mut rng);
            let mut lanes = 0;
            if drops.iter().any(|d| d.0 == sap_id) {
                sapling += 1;
                lanes += 1;
            }
            if let Some(s) = drops.iter().find(|d| d.0 == stick_id) {
                assert!((1..=2).contains(&s.1), "the stick's uniform [1,2]");
                sticks += 1;
                lanes += 1;
            }
            if drops.iter().any(|d| d.0 == apple_id) {
                apples += 1;
                lanes += 1;
            }
            if lanes > 1 {
                multi += 1;
            }
        }
        assert!(sapling > 8192 * 3 / 100, "the 5% sapling pool: {sapling}");
        assert!(sapling < 8192 * 7 / 100);
        assert!(sticks > 8192 / 100, "the 2% stick pool: {sticks}");
        assert!(sticks < 8192 * 3 / 100);
        assert!(apples > 0, "the 0.5% apple pool speaks over 8192: {apples}");
        assert!(multi > 0, "the independent pools CAN co-fire: {multi}");
    }

    #[test]
    fn leaf_litter_drops_itself() {
        // the default state's segment_amount=1 gate (the probe's own
        // verdict) — the set_count 1 branch matches
        let drops = roll_drops(crate::world::LEAF_LITTER, &mut Lcg::new(25));
        assert_eq!(drops, vec![(mc_registry::item_id_by_name("leaf_litter").unwrap(), 1)]);
    }

    // ---------- the render laws ----------

    #[test]
    fn the_lerped_pos_walks_the_prev_to_the_current() {
        // the None prev (the just-spawned): the spawn point whole
        let mut d = ItemDrop::spawn(1, 1, 1, 0, 0, 0, &mut Lcg::new(7));
        assert_eq!(d.lerped_pos(0.5), (d.x, d.y, d.z));
        // the tick head saves, the physics walks, the lerp folds
        d.prev = Some((d.x, d.y, d.z));
        d.x += 4.0;
        d.y -= 2.0;
        d.z += 6.0;
        let (px, py, pz) = d.prev.expect("the head saved");
        let (lx, ly, lz) = d.lerped_pos(0.5);
        assert!((lx - (px + 2.0)).abs() < 1e-12);
        assert!((ly - (py - 1.0)).abs() < 1e-12);
        assert!((lz - (pz + 3.0)).abs() < 1e-12);
        // the exactness arms: >= 1.0 the CURRENT bit for bit,
        // 0.0 the prev whole
        assert_eq!(d.lerped_pos(1.0), (d.x, d.y, d.z));
        assert_eq!(d.lerped_pos(1.5), (d.x, d.y, d.z));
        assert_eq!(d.lerped_pos(0.0), (px, py, pz));
    }

    #[test]
    fn the_tick_head_saves_before_the_physics() {
        // the falling drop: one tick walks the gravity, the prev keeps
        // the pre-move pose the lerp walks FROM
        let world = World::new(2, 42);
        let mut d = ItemDrop::spawn(1, 1, 1, 0, 40, 0, &mut Lcg::new(7));
        let (x0, y0, z0) = (d.x, d.y, d.z);
        assert!(d.tick(&world));
        assert_eq!(d.prev, Some((x0, y0, z0)));
        assert!(d.y < y0, "the gravity walked down");
    }

    #[test]
    fn render_laws_match_the_javap() {
        // the bob: sin(age/10 + offs) * 0.1 + 0.1
        let b = bob(0.0, 0.0);
        assert!((b - BOB_BASE).abs() < 1e-6, "age 0: sin(0) = 0");
        let b = bob(10.0, 0.0);
        assert!((b - (10f32 / 10.0).sin() * 0.1 - 0.1).abs() < 1e-6);
        // the spin: age/20 + offs (the radians)
        assert!((spin(40.0, 0.5) - (2.0 + 0.5)).abs() < 1e-6);
        // the base lift: -minY + 0.0625 (a full cube model's minY 0 -> 0.0625)
        assert!((base_lift(0.0) - 0.0625).abs() < 1e-6);
        // the pickup voice's pitch law
        assert!((pickup_pitch(1.0, 1.0) - PICKUP_PITCH_CENTER).abs() < 1e-6);
        assert!((pickup_pitch(1.0, 0.0) - (1.4 + 2.0)).abs() < 1e-6);
        assert!((pickup_pitch(0.0, 1.0) - (-1.4 + 2.0)).abs() < 1e-6);
        assert!((PICKUP_VOLUME - 0.2).abs() < 1e-6);
    }
}

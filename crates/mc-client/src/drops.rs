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
//! (the selected slot first, the scan, the free slot, the min(64,
//! itemMax) cap the jar's own getMaxStackSize(ItemStack) fold reads), and
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

/// THE CONTAINER SPILL'S OWN SPREAD — javap `Containers.dropItemStack`
/// pc 3-84 the 26.2 jar banks: `d0 = EntityType.ITEM.getWidth()` (0.25),
/// `d1 = 1 - d0` (0.75), `d2 = d0 / 2` (0.125); the position ONE draw
/// triplet wears — x/z `floor(x) + nextDouble() * d1 + d2`, y
/// `floor(y) + nextDouble() * 0.8` — the stack's own spot the chunks
/// share.
pub const SPILL_XZ_SPAN: f64 = 0.75;
pub const SPILL_XZ_OFFSET: f64 = 0.125;
pub const SPILL_Y_SPAN: f64 = 0.8;
/// The spill throw's own triangle (javap pc 140-169: `setDeltaMovement(
/// triangle(0.0, 0.11485000171139836), triangle(0.2, 0.11485...),
/// triangle(0.0, 0.11485...))` — the 0.11485 float the widened double
/// carries, the `RandomSource.triangle(mode, delta)` default
/// `mode + delta * (nextDouble() - nextDouble())` the two-draw spread
/// speaks).
pub const SPILL_THROW_DELTA: f64 = 0.11485000171139836;
pub const SPILL_THROW_Y_CENTER: f64 = 0.2;
/// The chunk split's own roll (javap pc 112: `nextInt(21) + 10` — the
/// 10..30 span the `Math.min(roll, stack.getMaxStackSize())` fold caps;
/// the stack the while-loop drains until empty).
pub const SPILL_CHUNK_ROLL: u64 = 21;
pub const SPILL_CHUNK_MIN: i32 = 10;
/// The spill's own stack cap (the `stack.getMaxStackSize()` arm the min
/// fold reads — the registry's item_max_stack table the autopush-133
/// round banks: the sword the 1, the snowball the 16, the missing row
/// the 64 default wears; the unstackable spill one entity per count
/// speaks, the 16-stack the 10..30 rolls the 16 caps). The constant
/// the walk's own table read answers — the per-item cap no single
/// number keeps.

/// The drop lane's own deterministic generator — the audio lane's LCG
/// (the same constants, the same seed scramble; the vanilla's
/// RandomSource stand-in the whole audio lane already speaks). Each
/// ItemDrop world carries ONE generator: the spawn spreads, the loot
/// rolls and the pickup pitches draw from it in tick order.
/// `Clone` the tests' own stream-position probes keep (the reference
/// replay the clone's state rides). `PartialEq` the stream-state
/// compares the despawn lane's short-circuit pins keep (the fired
/// walk exactly-one-draw the reference fold answers).
#[derive(Clone, PartialEq, Debug)]
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
    #[expect(dead_code)] // the RNG's own reseed lane — the deterministic seed law keeps
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

    /// `RandomSource.triangle(mode, delta)` (javap the default method
    /// verbatim: `mode + delta * (nextDouble() - nextDouble())` — the
    /// two-draw triangular spread the arrow's inaccuracy scales).
    pub fn triangle(&mut self, delta: f64) -> f64 {
        delta * (self.next_f64() - self.next_f64())
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

    /// THE CONTAINER SPILL'S OWN SPAWN — javap `Containers.dropItemStack`
    /// the 26.2 walk verbatim (`BlockEntity.preRemoveSideEffects`'s own
    /// dropContents arm the broken container speaks): the position ONE
    /// draw triplet wears (`floor(x) + nextDouble() * 0.75 + 0.125` x/z,
    /// `floor(y) + nextDouble() * 0.8` y — the stack's own spot), the
    /// stack the while-loop DRAINS by `min(nextInt(21) + 10, stackMax)`
    /// chunks (`stack.split` takes the min(count, roll) the remainder
    /// keeps), each chunk one ItemEntity at the SAME spot: the ctor's
    /// throw pair the `setDeltaMovement` overwrite answers (the two
    /// `(rnd - 0.5) * 0.1` draws the stream keeps, the velocity the
    /// triangle arm re-speaks — `triangle(0.0, 0.11485)` x/z,
    /// `triangle(0.2, 0.11485)` y), the bob seed the render's own spin
    /// wears. One entity per chunk — the ids the base rides (the
    /// caller's counter the slot's own walk bumps by the chunk count
    /// the Vec speaks). Returns one entity per chunk; the empty stack
    /// the isEmpty guard silences.
    pub fn spawn_container_spill(
        id: i32,
        item: i32,
        count: i32,
        bx: i32,
        by: i32,
        bz: i32,
        rng: &mut Lcg,
    ) -> Vec<Self> {
        if count <= 0 {
            return Vec::new(); // the dropContents walk's isEmpty guard
        }
        // THE POSITION'S OWN TRIPLET (one draw set per STACK — the
        // chunks the same spot share, javap pc 24-84 before the loop)
        let x = bx as f64 + rng.next_f64_between(0.0, SPILL_XZ_SPAN) + SPILL_XZ_OFFSET;
        let y = by as f64 + rng.next_f64_between(0.0, SPILL_Y_SPAN);
        let z = bz as f64 + rng.next_f64_between(0.0, SPILL_XZ_SPAN) + SPILL_XZ_OFFSET;
        let mut out = Vec::new();
        let mut rest = count;
        while rest > 0 {
            // the chunk's own roll (javap pc 112: the 10..30 span the
            // stack max caps, the split's min-count the remainder drains)
            let roll = SPILL_CHUNK_MIN + rng.next_int(SPILL_CHUNK_ROLL);
            let stack_max = mc_registry::item_max_stack(item) as i32;
            let taken = roll.min(stack_max).min(rest);
            rest -= taken;
            // THE CTOR'S OWN THROW (the 5-arg ItemEntity ctor's
            // setDeltaMovement — `(nextDouble() - 0.5) * 0.1` x/z, 0.2 y
            // — the pair the spill's triangle overwrite answers: the
            // draws the stream keeps, the velocity the re-speak)
            let _ = rng.next_f64();
            let _ = rng.next_f64();
            // THE SPILL'S OWN THROW (javap pc 140-169 — the triangle
            // arm the setDeltaMovement call re-speaks)
            let vx = rng.triangle(SPILL_THROW_DELTA);
            let vy = SPILL_THROW_Y_CENTER + rng.triangle(SPILL_THROW_DELTA);
            let vz = rng.triangle(SPILL_THROW_DELTA);
            // the bob seed (the render's own spin draw)
            let bob_offs = rng.next_f32() * std::f32::consts::PI * 2.0;
            out.push(Self {
                id: id + out.len() as i32,
                item,
                count: taken,
                x,
                y,
                z,
                vx,
                vy,
                vz,
                age: 0,
                ticks: 0,
                pickup_delay: POP_PICKUP_DELAY,
                bob_offs,
                on_ground: false,
                prev: None,
            });
        }
        out
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
/// stand-in — the client's stacks carry the id alone). The cap the
/// item's own max stack wears — the registry's item_max_stack table
/// (the snowball pair the 16 the sum refuses, the azalea pair the 64
/// the sum accepts).
fn mergeable(a: &ItemDrop, b: &ItemDrop) -> bool {
    let cap = mc_registry::item_max_stack(a.item) as i32;
    a.item == b.item && a.count > 0 && b.count > 0 && a.count + b.count <= cap
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
        let cap = mc_registry::item_max_stack(self_ref.item) as i32;
        if self_ref.count <= 0 || self_ref.count >= cap {
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
    /// and the count below the max — the cap the Inventory's own
    /// `getMaxStackSize(ItemStack)` fold speaks (the min(64, itemMax)
    /// the addResource split's Math.min answers, the registry's
    /// item_max_stack table the itemMax side banks).
    fn has_remaining_space(item: (i32, i32), id: i32) -> bool {
        let cap = mc_registry::item_max_stack(id).min(64);
        item.1 > 0 && item.0 == id && item.1 < cap
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
        // the space the jar's own getMaxStackSize(ItemStack) fold speaks
        // (the min(64, itemMax) — the item's own cap the table banks)
        let cap = mc_registry::item_max_stack(id).min(64);
        let space = cap - count;
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

    /// `Inventory.add(int, ItemStack)` (javap 26.2, sha1 2dc72797 —
    /// pcs 0-101): the undamaged walk the loop head RE-FRESHES —
    /// `count_in = stack.getCount()` rides INSIDE the loop (pc 16-20),
    /// each pass re-locates (the remaining-space scan, the free-slot
    /// fallback), the pass's take shrinks the stack, the `count <
    /// count_in` compare (pc 55-62) re-runs ONLY while the LAST pass
    /// made progress. The EXIT verdict (pcs 63-101): the stalled pass's
    /// `count == count_in` speaks the VOID arm — `hasInfiniteMaterials`
    /// zero the stack and return TRUE (the creative sweep: the whole
    /// touch vanishes the drop, what fits landed, the tail voided) —
    /// the survival arm returns `count < count_in`: the LAST pass's
    /// own progress, NOT the walk's overall take (a walk that landed
    /// 10 then stalled speaks FALSE — tryPickup refuses, the entity
    /// keeps the leftover, the voice never plays). Returns the
    /// (leftover, tryPickup) pair — the leftover the caller's honest
    /// shrink, the bool the vanilla add's own return.
    pub fn add(&mut self, selected: usize, id: i32, count: i32, creative: bool) -> (i32, bool) {
        if count <= 0 {
            return (0, false); // the isEmpty gate (pc 0-8) — nothing walks
        }
        let mut remaining = count;
        loop {
            let before = remaining; // count_in — the loop head's own refresh
            let slot = self
                .get_slot_with_remaining_space(selected, id)
                .or_else(|| self.get_free_slot());
            let Some(slot) = slot else {
                // addResource's no-home arm (the (ItemStack) overload's
                // `if (slot == -1) return count`) — the pass stalled
                if creative {
                    return (0, true); // the void arm (pcs 71-87)
                }
                return (remaining, false); // pc 88-101: count < count_in FALSE
            };
            remaining = self.add_resource(slot, id, remaining);
            if remaining <= 0 {
                return (0, true); // the empty stack — pc 48-52's exit, TRUE
            }
            if remaining >= before {
                // the stalled pass (pc 60's compare refuses) — the exit
                if creative {
                    return (0, true); // the void arm
                }
                return (remaining, false);
            }
            // the pass spoke (pc 60's `count < count_in`) — re-locate
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
pub fn roll_drops(kind: u16, held: Option<i32>, rng: &mut Lcg) -> Vec<(i32, i32)> {
    // THE WALL HEADS' OWN TWIN FOLD (the dev-83 round — javap
    // Blocks.wallVariant the 26.2 jar the probe scripts/skull_probe_83e.py
    // banks: the wall skull's registration the Properties full-copy
    // walk speaks — the LOOT TABLE the floor twin's own row the copy
    // carries [the generated block_drops table the wall names never
    // carries — the jar's loot_table/blocks/ the wall files never
    // had], the dig drop the floor item wears). The non-head kinds
    // the plain NAME_OF row the walk keeps.
    let name = if let Some(twin) = crate::world::wall_head_floor_twin(kind) {
        crate::world::NAME_OF[twin as usize]
    } else {
        crate::world::NAME_OF[kind as usize]
    };
    if !crate::dig::DigState::correct_tool(kind, held) {
        return Vec::new();
    }
    // THE HARVEST'S FAMILY GATE — the wheat ladder's age law the
    // generated table's clean-room flatten never wore (the
    // alternatives' state condition the flat row keeps): the harvest
    // arm's own verdict speaks.
    if crate::world::is_wheat(kind) {
        return roll_wheat_drops(kind, rng);
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

/// THE HARVEST'S OWN VERDICT — `data/minecraft/loot_table/blocks/wheat.json`
/// (the official 26.2 jar, sha 2dc72797) the two pools speak:
///
/// POOL 1 the ALTERNATIVES walk — the first child whose condition
/// passes wins (javap AlternativesEntry): the age-7 arm the WHEAT
/// item wears, the ELSE arm the bare wheat_seeds speaks (no
/// set_count — the count 1 the entry's own default keeps; no roll —
/// the definite pass no draw consumes).
///
/// POOL 2 the age-7 CONDITION rides (`block_state_property` age=7 —
/// the immature break the pool never rolls, no draw) — the
/// apply_bonus binomial_with_bonus_count the hand's fortune-0 lane
/// keeps: the javap calculateNewCount the `nextFloat() < probability`
/// comparison walks, the rounds `extraRounds + enchantmentLevel`
/// speak (0 + the json's extra=3 — the three draws the parameters
/// wear), the probability 0.5714286 the 4/7 rounds, the initial
/// count 1 the no-set_count law keeps — the mature harvest the
/// 1..4 seeds wears.
///
/// THE GENERATED ROW THE FLATTEN EXPLAINS — the clean-room walk
/// (scripts/gen_block_drops.py) the alternatives' ELSE arm alone
/// carried (DROPS_WHEAT = seeds ×1): the age law the state condition
/// the flat table never expressed — the stage kinds the client's own
/// kind ladder carries, the family gate roll_drops routes here
/// BEFORE the flat row's verdict (the harvest the real json's rows
/// answer).
pub fn roll_wheat_drops(kind: u16, rng: &mut Lcg) -> Vec<(i32, i32)> {
    let Some(age) = crate::world::wheat_age(kind) else {
        return Vec::new();
    };
    if age < 7 {
        // pool 1's ELSE arm: the bare seed — the definite pass, no draw
        return mc_registry::item_id_by_name("wheat_seeds")
            .map(|id| vec![(id, 1)])
            .unwrap_or_default();
    }
    let mut out = Vec::new();
    // pool 1's age-7 arm: the grain — the definite item, no draw
    if let Some(id) = mc_registry::item_id_by_name("wheat") {
        out.push((id, 1));
    }
    // pool 2's bonus walk: the seed + the three-draw binomial
    // (nextFloat() < 4/7 the javap comparison keeps, extra=3 the
    // fortune-0 hand lane's own rounds)
    if let Some(sid) = mc_registry::item_id_by_name("wheat_seeds") {
        let mut count = 1i32;
        for _ in 0..3 {
            if rng.next_f32() < 0.5714286 {
                count += 1;
            }
        }
        out.push((sid, count));
    }
    out
}

/// The pickup outcome (the caller's own lanes: the sound beat, the
/// hotbar refresh).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Touch {
    /// The delay still holds (or the stack vanished) — nothing spoke.
    Idle,
    /// The vanilla add returned TRUE — the last pass took (or the
    /// creative void consumed): the pickup voice's own beat, the
    /// entity's discard.
    Picked { added: i32 },
    /// The walk LANDED items but the LAST pass stalled — the vanilla
    /// add's own FALSE (javap pcs 88-101: `count < count_in` reads the
    /// last pass, not the walk): tryPickup refuses, the voice never
    /// plays, the entity keeps the leftover — the landed tail the
    /// caller's row diff still wears.
    Took { added: i32 },
}

/// `ItemEntity.playerTouch`'s own law (javap 26.2 — the tryPickup walk
/// the inventory add answers): the delay gate, the target gate (the
/// block drops carry none), the inventory add's (leftover, verdict)
/// pair, the TRUE verdict's `drop()` + discard (the voice the caller
/// plays, the whole entity gone), the FALSE verdict's honest keep (the
/// entity holds the leftover, the landed tail stays taken). The
/// CREATIVE void (the add's hasInfiniteMaterials arm) returns TRUE
/// with the leftover zeroed — the touch consumes the whole drop.
pub fn player_touch(
    drop: &mut ItemDrop,
    inventory: &mut Inventory,
    selected: usize,
    creative: bool,
) -> Touch {
    if drop.pickup_delay != 0 || drop.count <= 0 {
        return Touch::Idle;
    }
    let original = drop.count;
    let (leftover, spoke) = inventory.add(selected, drop.item, drop.count, creative);
    if leftover < original {
        drop.count = leftover; // the honest shrink (the void's own zero rides)
    }
    if !spoke {
        if leftover < original {
            return Touch::Took {
                added: original - leftover,
            };
        }
        return Touch::Idle; // no room — the vanilla's own verdict (no voice)
    }
    drop.count = 0; // the TRUE verdict discards the whole entity
    Touch::Picked {
        added: original,
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

    // ---------- the container spill law ----------

    #[test]
    fn container_spill_rides_drop_item_stack_law() {
        // javap Containers.dropItemStack: ONE pos triplet per stack
        // (floor + nextDouble * 0.75 + 0.125 x/z, floor + nextDouble *
        // 0.8 y), the chunks `min(nextInt(21) + 10, stackMax)` at the
        // SAME spot, the triangle throw, the 10-tick delay
        let mut rng = Lcg::new(555);
        let chunks = ItemDrop::spawn_container_spill(7, 42, 5, 10, 20, 30, &mut rng);
        assert_eq!(chunks.len(), 1, "the small stack ONE chunk wears");
        let c = &chunks[0];
        assert_eq!(c.count, 5);
        assert_eq!(c.id, 7);
        assert_eq!(c.item, 42);
        assert!(c.x >= 10.125 && c.x < 10.875, "the 0.75 span + the 0.125 offset: {}", c.x);
        assert!(c.y >= 20.0 && c.y < 20.8, "the 0.8 y span: {}", c.y);
        assert!(c.z >= 30.125 && c.z < 30.875, "the 0.75 span + the 0.125 offset: {}", c.z);
        assert_eq!(c.pickup_delay, POP_PICKUP_DELAY);
        // the throw bands (the triangle ±0.11485, the y center 0.2)
        assert!(c.vx.abs() <= SPILL_THROW_DELTA, "vx band: {}", c.vx);
        assert!(c.vz.abs() <= SPILL_THROW_DELTA, "vz band: {}", c.vz);
        assert!(
            (SPILL_THROW_Y_CENTER - SPILL_THROW_DELTA..=SPILL_THROW_Y_CENTER + SPILL_THROW_DELTA)
                .contains(&c.vy),
            "vy band: {}",
            c.vy
        );
        assert!(c.bob_offs >= 0.0 && c.bob_offs < std::f32::consts::PI * 2.0);

        // THE BIG STACK'S OWN DRAIN (the stack max the min(roll, max)
        // fold caps — the azalea the 64 default wears, the chunks the
        // while-loop wears, the SUM the stack keeps, the spot the whole
        // stack shares)
        let mut rng = Lcg::new(777);
        let chunks = ItemDrop::spawn_container_spill(100, 42, 64, 0, 0, 0, &mut rng);
        let sum: i32 = chunks.iter().map(|c| c.count).sum();
        assert_eq!(sum, 64, "the chunks the whole stack drain");
        assert!(chunks.len() > 1, "the 64 the 10..30 rolls split");
        for (n, c) in chunks.iter().enumerate() {
            assert!((1..=30).contains(&c.count), "the roll band: {}", c.count);
            assert_eq!(c.id, 100 + n as i32, "the chunk ids the base rides");
            assert_eq!(c.x, chunks[0].x, "the pos triplet one draw set keeps");
            assert_eq!(c.y, chunks[0].y);
            assert_eq!(c.z, chunks[0].z);
        }
        // the empty count the isEmpty guard silences
        assert!(ItemDrop::spawn_container_spill(1, 42, 0, 0, 0, 0, &mut rng).is_empty());
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

    #[test]
    fn the_spill_chunks_the_item_max_stack_cap_wears() {
        // the 16-stack item: the 10..30 rolls the 16 caps (the 64 the
        // stand-in spoke), the chunks the while-loop drains, the SUM the
        // stack keeps
        let snowball =
            mc_registry::item_id_by_name("snowball").expect("the snowball rides the wire table");
        let mut rng = Lcg::new(909);
        let chunks = ItemDrop::spawn_container_spill(200, snowball, 100, 0, 0, 0, &mut rng);
        let sum: i32 = chunks.iter().map(|c| c.count).sum();
        assert_eq!(sum, 100, "the chunks the whole stack drain");
        assert!(chunks.len() > 1, "the 100 the 16-capped rolls split");
        for (n, c) in chunks.iter().enumerate() {
            assert!((1..=16).contains(&c.count), "the 16 cap: {}", c.count);
            assert_eq!(c.item, snowball);
            assert_eq!(c.id, 200 + n as i32, "the chunk ids the base rides");
        }
        // THE UNSTACKABLE SPILL (the 1-stack item: the roll the 1 caps —
        // one entity PER count, the sword pile the single entities wear)
        let sword =
            mc_registry::item_id_by_name("diamond_sword").expect("the sword rides the wire table");
        let mut rng = Lcg::new(910);
        let chunks = ItemDrop::spawn_container_spill(300, sword, 5, 0, 0, 0, &mut rng);
        assert_eq!(chunks.len(), 5, "the 5 swords the 5 single chunks wear");
        for (n, c) in chunks.iter().enumerate() {
            assert_eq!(c.count, 1, "the 1-stack the roll the 1 caps");
            assert_eq!(c.item, sword);
            assert_eq!(c.id, 300 + n as i32);
        }
    }

    #[test]
    fn the_merge_law_the_item_max_stack_cap_wears() {
        // THE 16-STACK ITEM THE 16 CAP WEARS: 10 + 10 exceeds the 16 —
        // the merge refuses (the 64 the stand-in never spoke)
        let snowball =
            mc_registry::item_id_by_name("snowball").expect("the snowball rides the wire table");
        let mut a = ItemDrop::spawn(1, snowball, 10, 0, 0, 0, &mut Lcg::new(21));
        let mut b = ItemDrop::spawn(2, snowball, 10, 0, 0, 0, &mut Lcg::new(22));
        a.x = 100.0;
        a.y = 60.0;
        a.z = 100.0;
        b.x = 100.1;
        b.y = 60.0;
        b.z = 100.0;
        let mut drops = vec![a, b];
        merge_pass(&mut drops);
        assert_eq!(drops.len(), 2, "the 20 sum the 16 cap refuses");
        // THE FITTING SUM MERGES (5 + 5 = 10 <= 16 — the merge the same
        // law accepts)
        let mut a = ItemDrop::spawn(1, snowball, 5, 0, 0, 0, &mut Lcg::new(23));
        let mut b = ItemDrop::spawn(2, snowball, 5, 0, 0, 0, &mut Lcg::new(24));
        a.x = 100.0;
        a.y = 60.0;
        a.z = 100.0;
        b.x = 100.1;
        b.y = 60.0;
        b.z = 100.0;
        let mut drops = vec![a, b];
        merge_pass(&mut drops);
        assert_eq!(drops.len(), 1, "the 10 sum the 16 cap accepts");
        assert_eq!(drops[0].count, 10);
        // THE FULL STACK NEVER MERGES (the 16-count entity the count >=
        // cap gate the merge pass skips)
        let mut a = ItemDrop::spawn(1, snowball, 16, 0, 0, 0, &mut Lcg::new(25));
        let mut b = ItemDrop::spawn(2, snowball, 4, 0, 0, 0, &mut Lcg::new(26));
        a.x = 100.0;
        a.y = 60.0;
        a.z = 100.0;
        b.x = 100.1;
        b.y = 60.0;
        b.z = 100.0;
        let mut drops = vec![a, b];
        merge_pass(&mut drops);
        assert_eq!(drops.len(), 2, "the full stack the gate refuses");
        assert_eq!(drops[0].count, 16);
        assert_eq!(drops[1].count, 4);
    }

    #[test]
    fn the_inventory_add_the_item_max_stack_cap_wears() {
        // THE EMPTY INVENTORY THE 16-CAP SPLIT WEARS: 20 snowballs —
        // the first slot takes min(20, 16) = 16, the leftover 4 the
        // loop re-locates (the second pass the free slot opens — the
        // 64 stand-in the single 20-slot wore)
        let snowball =
            mc_registry::item_id_by_name("snowball").expect("the snowball rides the wire table");
        let mut inv = Inventory::default();
        let (leftover, spoke) = inv.add(0, snowball, 20, false);
        assert_eq!(inv.items[0], (snowball, 16), "the first slot the 16 caps");
        assert_eq!(
            inv.items[1],
            (snowball, 4),
            "the overflow the next slot opens"
        );
        assert_eq!(leftover, 0);
        assert!(spoke, "the walk's last pass took — the add's TRUE");
        // THE FULL 16-SLOT THE REMAINING-SPACE GATE REFUSES (the scan
        // the 16 < 16 verdict skips — the add the free slot opens)
        let mut inv = Inventory::default();
        inv.items[0] = (snowball, 16);
        let (leftover, spoke) = inv.add(0, snowball, 5, false);
        assert_eq!(
            inv.items[0],
            (snowball, 16),
            "the full 16 the add never bends"
        );
        assert_eq!(
            inv.items[1],
            (snowball, 5),
            "the free slot the overflow opens"
        );
        assert_eq!(leftover, 0);
        assert!(spoke);
        // THE 1-STACK ITEM THE ONE-PER-SLOT LAW WEARS (the swords never
        // merge — the 5-add the 5 slots the scan order fills)
        let sword =
            mc_registry::item_id_by_name("diamond_sword").expect("the sword rides the wire table");
        let mut inv = Inventory::default();
        let (leftover, spoke) = inv.add(0, sword, 5, false);
        assert_eq!(
            inv.items[0],
            (sword, 1),
            "the first sword the selected takes"
        );
        assert_eq!(inv.items[1], (sword, 1));
        assert_eq!(
            inv.items[4],
            (sword, 1),
            "the fifth sword the fifth slot wears"
        );
        assert_eq!(leftover, 0);
        assert!(spoke);
    }

    // ---------- the inventory laws ----------

    #[test]
    fn add_prefers_selected_slot_then_scan_then_free() {
        let mut inv = Inventory::default();
        // the selected slot holds 30 dirt, slot 5 holds 60 dirt
        inv.items[0] = (7, 30);
        inv.items[5] = (7, 60);
        // 40 dirt: the selected merges to 64, the scan finds slot 5 for the rest
        let (leftover, spoke) = inv.add(0, 7, 40, false);
        // the honest walk: the selected takes min(40, 34) = 34 -> 64; the
        // next pass scans slot 5 (60): min(6, 4) = 4 -> 64; the third
        // pass finds no merge space and opens the first free slot (1)
        // for the last 2 — nothing left over
        assert_eq!(inv.items[0], (7, 64));
        assert_eq!(inv.items[5], (7, 64));
        assert_eq!(inv.items[1], (7, 2), "the overflow opens the first free slot");
        assert_eq!(leftover, 0);
        assert!(spoke, "the walk's last pass took — the add's TRUE");
        // the whole 64 stack in an empty inventory: getFreeSlot scans from
        // slot 0 (the vanilla's own order — NOT the selected slot)
        let mut inv = Inventory::default();
        let (leftover, spoke) = inv.add(3, 9, 64, false);
        assert_eq!(leftover, 0);
        assert_eq!(inv.items[0], (9, 64), "the free-slot scan starts at slot 0");
    }

    #[test]
    fn add_refuses_when_full() {
        let mut inv = Inventory::default();
        for slot in inv.items.iter_mut() {
            *slot = (5, 64);
        }
        let (leftover, spoke) = inv.add(0, 5, 3, false);
        assert_eq!(leftover, 3, "the full inventory returns the whole count");
        assert!(!spoke, "the stalled pass the add's FALSE speaks");
        // the CREATIVE void: the same full inventory, the infinite
        // materials — the add's hasInfiniteMaterials arm zeroes the
        // stack and returns TRUE (the touch consumes the whole drop)
        let (leftover, spoke) = inv.add(0, 5, 3, true);
        assert_eq!(leftover, 0, "the void arm zeroes the leftover");
        assert!(spoke, "the void arm's own TRUE");
        // a full inventory refuses a DIFFERENT item the same way
        let (leftover, spoke) = inv.add(0, 6, 1, false);
        assert_eq!(leftover, 1);
        assert!(!spoke);
    }

    #[test]
    fn add_merge_never_exceeds_cap() {
        let mut inv = Inventory::default();
        inv.items[2] = (7, 62);
        let (leftover, spoke) = inv.add(0, 7, 5, false);
        // the selected (0) is empty — the merge scan finds slot 2: min(5, 2)
        // = 2 -> 64; the leftover 3 opens slot 0 (the selected, the first
        // free in the scan order)
        assert_eq!(inv.items[2], (7, 64));
        assert_eq!(inv.items[0], (7, 3));
        assert_eq!(leftover, 0);
        assert!(spoke);
    }

    #[test]
    fn add_partial_take_the_last_pass_the_verdict_names() {
        // javap pcs 88-101: the add's return reads the LAST pass's own
        // `count < count_in`, NOT the walk's overall take — a walk that
        // landed 10 then stalled speaks FALSE even though items landed
        let mut inv = Inventory::default();
        inv.items[1] = (7, 60); // the merge takes 4, then the walk stalls
        for i in (2..36).step_by(2) {
            inv.items[i] = (9, 1);
        }
        for i in (3..36).step_by(2) {
            inv.items[i] = (8, 1);
        }
        inv.items[0] = (5, 64);
        let (leftover, spoke) = inv.add(0, 7, 70, false);
        assert_eq!(leftover, 66, "the merge took 4 (60 -> 64), the walk stalled");
        assert_eq!(inv.items[1], (7, 64), "the landed tail stays taken");
        assert!(!spoke, "the LAST pass stalled — the add's own FALSE");
    }

    #[test]
    fn add_creative_void_the_partial_tail_the_same_verdict() {
        // javap pcs 71-87: the stalled pass + hasInfiniteMaterials —
        // the stack ZEROED and TRUE, whatever the earlier passes took
        let mut inv = Inventory::default();
        inv.items[1] = (7, 60);
        for i in (2..36).step_by(2) {
            inv.items[i] = (9, 1);
        }
        for i in (3..36).step_by(2) {
            inv.items[i] = (8, 1);
        }
        inv.items[0] = (5, 64);
        let (leftover, spoke) = inv.add(0, 7, 70, true);
        assert_eq!(leftover, 0, "the void arm zeroes the tail");
        assert_eq!(inv.items[1], (7, 64), "the landed tail stays taken");
        assert!(spoke, "the void arm's own TRUE — the touch consumes");
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
        assert_eq!(player_touch(&mut d, &mut inv, 0, false), Touch::Idle, "the 10-tick delay holds");
        d.pickup_delay = 0;
        assert_eq!(
            player_touch(&mut d, &mut inv, 0, false),
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
        let t = player_touch(&mut d, &mut inv, 0, false);
        match t {
            Touch::Took { added } => {
                assert_eq!(added, 4, "the merge took 4 (60 -> 64)");
                assert_eq!(d.count, 66, "the entity keeps the leftover");
            }
            Touch::Picked { .. } => panic!(
                "the stalled last pass must NOT speak — the vanilla add's \
                 own FALSE the tryPickup refuses"
            ),
            Touch::Idle => panic!("the landed tail the Took verdict wears"),
        }
    }

    #[test]
    fn pickup_creative_void_consumes_the_whole_touch() {
        // the add's hasInfiniteMaterials arm: whatever the walk landed,
        // the stalled pass zeroes the stack — the whole drop vanishes
        // and the voice plays (the creative sweep's own feel)
        let mut inv = Inventory::default();
        inv.items[1] = (7, 60);
        for i in (2..36).step_by(2) {
            inv.items[i] = (9, 1);
        }
        for i in (3..36).step_by(2) {
            inv.items[i] = (8, 1);
        }
        inv.items[0] = (5, 64);
        let mut d = ItemDrop::spawn(1, 7, 70, 0, 0, 0, &mut Lcg::new(17));
        d.pickup_delay = 0;
        assert_eq!(
            player_touch(&mut d, &mut inv, 0, true),
            Touch::Picked { added: 70 },
            "the void arm's TRUE — the whole touch consumed"
        );
        assert_eq!(d.count, 0, "the entity discarded");
        assert_eq!(inv.items[1], (7, 64), "the fit landed before the void");
        // the never-took arm the same verdict wears (the full inventory
        // in creative still vanishes the drop — the sweep's own law)
        let mut full = Inventory::default();
        for slot in full.items.iter_mut() {
            *slot = (5, 64);
        }
        let mut d2 = ItemDrop::spawn(1, 7, 3, 0, 0, 0, &mut Lcg::new(18));
        d2.pickup_delay = 0;
        assert_eq!(
            player_touch(&mut d2, &mut full, 0, true),
            Touch::Picked { added: 3 },
            "the never-took touch the void's TRUE wears"
        );
        assert_eq!(d2.count, 0, "the untouched stack zeroed");
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
        assert!(roll_drops(crate::world::STONE, None, &mut rng).is_empty(), "stone demands the pickaxe");
        assert!(roll_drops(crate::world::GRANITE, None, &mut rng).is_empty());
        assert!(roll_drops(crate::world::DEEPSLATE, None, &mut rng).is_empty());
        assert!(roll_drops(crate::world::COAL_ORE, None, &mut rng).is_empty());
        assert!(roll_drops(crate::world::NETHERRACK, None, &mut rng).is_empty());
        assert!(roll_drops(crate::world::BEDROCK, None, &mut rng).is_empty(), "bedrock's -1 hardness never harvests");
    }

    #[test]
    fn simple_drops_speak_the_table() {
        let mut rng = Lcg::new(18);
        // dirt -> dirt; grass -> dirt; oak_log -> oak_log; sand -> sand
        let drops = roll_drops(crate::world::DIRT, None, &mut rng);
        assert_eq!(drops, vec![(mc_registry::item_id_by_name("dirt").unwrap(), 1)]);
        let drops = roll_drops(crate::world::GRASS, None, &mut rng);
        assert_eq!(drops, vec![(mc_registry::item_id_by_name("dirt").unwrap(), 1)], "the grass block drops dirt");
        let drops = roll_drops(crate::world::LOG, None, &mut rng);
        assert_eq!(drops, vec![(mc_registry::item_id_by_name("oak_log").unwrap(), 1)]);
        let drops = roll_drops(crate::world::SAND, None, &mut rng);
        assert_eq!(drops, vec![(mc_registry::item_id_by_name("sand").unwrap(), 1)]);
    }

    #[test]
    fn ore_counts_speak_the_inclusive_span() {
        // the TOOL GATE first: the bare hand cannot harvest the ores —
        // the vanilla drops NOTHING (the pickaxe tables ride a future
        // round; the generated table carries the counts verbatim)
        let mut rng = Lcg::new(19);
        assert!(roll_drops(crate::world::COPPER_ORE, None, &mut rng).is_empty());
        assert!(roll_drops(crate::world::REDSTONE_ORE, None, &mut rng).is_empty());
        assert!(roll_drops(crate::world::DIAMOND_ORE, None, &mut rng).is_empty());
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
            let drops = roll_drops(crate::world::GRAVEL, None, &mut rng);
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
            let drops = roll_drops(crate::world::LEAVES, None, &mut rng);
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
        let drops = roll_drops(crate::world::LEAF_LITTER, None, &mut Lcg::new(25));
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

    // ---------- the harvest's own verdict (wheat.json's two pools) ----------

    #[test]
    fn the_harvest_the_mature_the_grain_wears() {
        // pool 1's age-7 arm: the WHEAT item, the definite pass; pool 2's
        // binomial: 1 + Bin(3, 4/7) seeds — the 1..4 span the fortune-0
        // hand lane keeps
        let wheat = mc_registry::item_id_by_name("wheat").unwrap();
        let seeds = mc_registry::item_id_by_name("wheat_seeds").unwrap();
        let mut rng = Lcg::new(26_2);
        let drops = roll_wheat_drops(crate::world::WHEAT_STAGE7, &mut rng);
        assert_eq!(drops.len(), 2, "the two pools the two stacks speak");
        assert_eq!(drops[0], (wheat, 1), "the grain the definite arm wears");
        assert_eq!(drops[1].0, seeds);
        assert!((1..=4).contains(&drops[1].1), "the binomial span: {}", drops[1].1);
        // the same seed the same field: the replay's own determinism
        let mut rng = Lcg::new(26_2);
        assert_eq!(roll_wheat_drops(crate::world::WHEAT_STAGE7, &mut rng), drops);
        // the seed sweep: the span holds everywhere, BOTH extremes the
        // three draws speak (P(4) = (4/7)^3, P(1) = (3/7)^3 — the 200
        // fixed seeds the walk the extremes virtually guarantee)
        let mut saw_one = false;
        let mut saw_four = false;
        for seed in 1..=200u64 {
            let mut rng = Lcg::new(seed);
            let drops = roll_wheat_drops(crate::world::WHEAT_STAGE7, &mut rng);
            assert_eq!(drops[0], (wheat, 1));
            assert!((1..=4).contains(&drops[1].1), "seed {seed}: count {}", drops[1].1);
            saw_one |= drops[1].1 == 1;
            saw_four |= drops[1].1 == 4;
        }
        assert!(saw_one && saw_four, "the binomial's both ends the sweep speaks");
    }

    #[test]
    fn the_harvest_the_immature_the_seed_keeps() {
        // pool 1's ELSE arm the bare seed wears, pool 2's age-7 condition
        // the immature break never rolls — ZERO draws the mirror pins
        let seeds = mc_registry::item_id_by_name("wheat_seeds").unwrap();
        for stage in crate::world::WHEAT..=crate::world::WHEAT_STAGE6 {
            let mut rng = Lcg::new(stage as u64 * 7 + 1);
            let mut probe = rng.clone();
            // the FAMILY GATE the roll_drops' own early verdict rides —
            // the whole ladder the flat DROPS_WHEAT row's blanket the
            // arm replaces
            let drops = roll_drops(stage, None, &mut rng);
            assert_eq!(drops, vec![(seeds, 1)], "stage kind {stage}: the bare seed");
            // the zero-draw law: the probe's next draw the arm's own
            // untouched stream answers (a draw the mirror would split)
            assert_eq!(rng.next_f64(), probe.next_f64(), "stage kind {stage}: no draw");
        }
    }

    #[test]
    fn the_harvest_the_bonus_the_three_draws_keep() {
        // the mature break's own draw count: pool 1 the definite arm no
        // draw consumes, pool 2 the THREE nextFloat rounds the extra=3
        // keeps — the mirror replay the count pairs
        let mut rng = Lcg::new(505);
        let mut probe = rng.clone();
        let _ = roll_wheat_drops(crate::world::WHEAT_STAGE7, &mut rng);
        for _ in 0..3 {
            let _ = probe.next_f32();
        }
        assert_eq!(rng.next_f64(), probe.next_f64(), "the arm's own three draws");
        // the generic rows the gate never moved: the dirt row the flat
        // table keeps, the stone's tool refusal the gate's own verdict
        let mut rng = Lcg::new(9);
        assert_eq!(
            roll_drops(crate::world::DIRT, None, &mut rng),
            vec![(mc_registry::item_id_by_name("dirt").unwrap(), 1)],
            "the generic table the non-crop kinds keep"
        );
        assert!(roll_drops(crate::world::STONE, None, &mut Lcg::new(9)).is_empty());
    }
}

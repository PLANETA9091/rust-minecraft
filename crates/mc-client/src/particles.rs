//! THE PARTICLE LANE — the world's own dust (javap, the cached jar's
//! Mojang-mapped classes): the base `Particle` physics every particle
//! rides, the three emitters this client speaks — the block's own
//! burst (`TerrainParticle`, the block's particle tile), the death
//! cloud (`ExplodeParticle`, the poof grey), the crit star
//! (`CritParticle` + `TrackingEmitter`) — every number the bytecode
//! speaks.
//!
//! The wire's shape (javap verbatim):
//! - `Particle.tick` (the base physics all three ride): the age walk
//!   (`age++`, removed at `age >= lifetime`), the gravity lane
//!   (`yd -= 0.04 * gravity` — the field defaults 0.0F), the move
//!   (`move(xd, yd, zd)` — the collision lane sleeps here: the
//!   client's particles carry the pieces' own honest absence, the
//!   `hasPhysics` the base speaks never blocks), the friction lane
//!   (`xd *= friction` — the base ctor's default 0.98F), and the
//!   onGround's 0.7 lane (invisible here — no collision, no ground).
//! - The base ctor's own defaults: `lifetime = (int)(4.0F /
//!   (nextFloat() * 0.9F + 0.1F))` (4..40 ticks), `setSize(0.2, 0.2)`.
//! - `SingleQuadParticle` ctor: `rCol = gCol = bCol = alpha = 1.0F`,
//!   `quadSize = 0.1F * (nextFloat() * 0.5F + 0.5F) * 2.0F` (0.1..0.2).
//! - `TerrainParticle` ctor: the block's own PARTICLE sprite (the
//!   `getParticleMaterial(state).sprite()` the particle_table lane
//!   already speaks), `gravity = 1.0F`, `rCol = gCol = bCol = 0.6F`
//!   (the dark multiply over the tile's own colors), `quadSize /= 2`
//!   (0.05..0.1), `uo = nextFloat() * 3.0F`, `vo = nextFloat() *
//!   3.0F` — the render's uv law `getU0 = sprite.getU((uo + 1) / 4)`,
//!   `getU1 = sprite.getU(uo / 4)` (the U MIRRORS — the same walk the
//!   fire/block screens speak), `getV0 = sprite.getV(vo / 4)`,
//!   `getV1 = sprite.getV((vo + 1) / 4)`: a random QUARTER of the
//!   tile, the 4x4-texel slice a 16-px tile speaks.
//! - `ExplodeParticle` ctor (the poof): `gravity = -0.1F` (it RISES),
//!   `friction = 0.9F`, `xd = xa + (nextFloat() * 2 - 1) * 0.05F`
//!   (three axes), `col = nextFloat() * 0.3F + 0.7F` (the grey),
//!   `quadSize = 0.1F * (nextFloat() * nextFloat() * 6.0F + 1.0F)`
//!   (0.1..0.7), `lifetime = (int)(16.0 / (nextFloat() * 0.8 + 0.2)) + 2`
//!   (18..82 ticks), the sprite walk `setSpriteFromAge` over the
//!   `generic_0..7` frames — `MutableSpriteSet.get(age, lifetime)`
//!   verbatim: `sprites.get(age * (size - 1) / lifetime)`.
//! - `CritParticle` (the star): the base ctor's own defaults ride, the
//!   tick fades `gCol *= 0.96F`, `bCol *= 0.9F` (per tick), and the
//!   render grows `getQuadSize = quadSize * clamp((age + a) /
//!   lifetime * 32.0F, 0.0F, 1.0F)` — the star blooms from a point.
//! - `TrackingEmitter` (the crit's own emitter, `LocalPlayer.crit` →
//!   `ParticleEngine.createTrackingEmitter`): lifetime 3 ticks, per
//!   tick the 16-sample walk: three `nextFloat() * 2.0 - 1.0` draws,
//!   the unit-ball reject (`d0*d0 + d2*d2 + d4*d4 > 1.0` skips), the
//!   spawn at the bbox fractions `entity.getX(d0 / 4)`,
//!   `entity.getY(0.5 + d2 / 4)`, `entity.getZ(d4 / 4)` (the lerp the
//!   bb speaks), the velocity `(d0, d2 + 0.2, d4)`.
//! - `LivingEntity.makePoofParticles` (the death cloud, the broadcast
//!   60's own arm): 20 particles, `d = nextGaussian() * 0.02` (three
//!   draws per particle — the `MarsagliaPolarGaussian` polar method
//!   the RandomSource speaks), the spawn at `getRandomX(1.0) - d0 *
//!   10.0`, `getRandomY() - d1 * 10.0`, `getRandomZ(1.0) - d2 * 10.0`
//!   (the body's random point walked BACK along the velocity), the
//!   velocity `(d0, d1, d2)`.
//! - `ClientLevel.addDestroyBlockEffect` (the 2001 event's own burst,
//!   the lambda verbatim): per shape box `dx = min(1.0, maxX - minX)`
//!   (three axes), `sections = max(2, ceil(dx / 0.25))` (three axes —
//!   the full cube's 4x4x4 = 64), the particle at the cell's fraction
//!   `(i + 0.5) / sections * dx + min` with the velocity
//!   `(fx - 0.5, fy - 0.5, fz - 0.5)`.
//! - `ClientLevel.addBreakingBlockEffect` (the dig's own dust,
//!   `Minecraft.continueAttack`'s arm — every dig tick): the random
//!   point inset `pos + nextDouble() * (max - min - 0.2) + 0.1 + min`
//!   (three axes), the face arm moving the axis to the face (± 0.1
//!   beyond the bounds), the velocity (0,0,0) through
//!   `setPower(0.2F)` (the power multiplies the velocity — the zero
//!   fold rides), `.scale(0.6F)` (the quad shrinks).
//! - `BubbleParticle` (the drown bubble, the `bubble` particle type's
//!   own sprite — the jar's 8x8 `textures/particle/bubble.png`): the
//!   ctor chain's draws ride the base laws first (the dead lifetime
//!   float, the 0.1..0.2 quadSize — `setSize(0.02, 0.02)` only moves
//!   the BOUNDING BOX, the quad's own size the chain keeps), then the
//!   bubble's own folds: `quadSize *= nextFloat() * 0.6 + 0.2`,
//!   each velocity axis `xa * 0.2 + (nextFloat() * 2 - 1) * 0.02`
//!   (the caller's velocity thinned, the ±0.02 jitter added), the
//!   lifetime `(int)(8.0 / (nextFloat() * 0.8 + 0.2))` (8..40). The
//!   tick is the BUBBLE'S OWN (the base law never rides it): the
//!   countdown lifetime (`original > 0` gates the body, no motion on
//!   the removal tick), the buoyancy lane `yd += 0.002` BEFORE the
//!   move (the bubble RISES — no gravity lane), the friction
//!   `*= 0.85` AFTER the move, and the dry death — the fluid at the
//!   containing cell leaving WATER removes the bubble the same tick.
//! - `LivingEntity.makeDrownParticles` (the broadcast-67 arm — the
//!   handleEntityEvent's own case, the client lane's visual): 8
//!   bubbles, three `triangle(0.0, 1.0)` offsets per particle
//!   (`min + max * (nextDouble() - nextDouble())` — the [-1, 1]
//!   triangle the RandomSource default speaks), the spawn at
//!   `(getX() + d3, getY() + d5, getZ() + d7)`, the velocity the
//!   body's OWN delta movement (the vec3 the method opens with).

/// The base ctor's own lifetime law: `(int)(4.0F / (r * 0.9F + 0.1F))`
/// — 4..40 ticks.
pub fn base_lifetime(r: f32) -> u32 {
    (4.0f32 / (r * 0.9 + 0.1)) as u32
}

/// The `SingleQuadParticle` ctor's own size law: `0.1F * (r * 0.5F +
/// 0.5F) * 2.0F` — 0.1..0.2.
pub fn base_quad_size(r: f32) -> f32 {
    0.1 * (r * 0.5 + 0.5) * 2.0
}

/// The poof's frame walk (`MutableSpriteSet.get(age, lifetime)`
/// verbatim): `age * (size - 1) / lifetime` — the 8 generic frames.
pub fn poof_frame(age: u32, lifetime: u32) -> usize {
    (age as usize * (POOF_FRAME_COUNT - 1)) / lifetime.max(1) as usize
}

/// The poof's 8 frames (the `generic_0..7` textures the jar speaks).
pub const POOF_FRAME_COUNT: usize = 8;

/// The crit's render-growth law (javap `CritParticle.getQuadSize`):
/// `clamp((age + a) / lifetime * 32.0F, 0.0F, 1.0F)` — the star blooms
/// from a point to the full quad inside the first lifetime/32 ticks.
pub fn crit_grow(age: u32, lifetime: u32, partial: f32) -> f32 {
    let t = (age as f32 + partial) / lifetime.max(1) as f32 * 32.0;
    t.clamp(0.0, 1.0)
}

/// The three kinds this client speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleKind {
    /// `TerrainParticle` — the block's own particle tile (the
    /// `particle_tile` table's kind, the atlas tile id).
    Terrain { tile: u16 },
    /// `ExplodeParticle` — the death cloud (the generic frames).
    Poof,
    /// `CritParticle` — the crit star (the critical_hit texture).
    Crit,
    /// `BubbleParticle` — the drown bubble (the bubble texture, the
    /// full sprite the quad samples, the body's own rise).
    Bubble,
}

/// One particle — the vanilla `Particle` + `SingleQuadParticle` fields
/// the three kinds' laws speak (the fields the tick and the render
/// read; the removed lane rides the caller's retain).
pub struct Particle {
    pub kind: ParticleKind,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub age: u32,
    pub lifetime: u32,
    /// The gravity field (the base default 0.0; the terrain's 1.0,
    /// the poof's -0.1 — javap `Particle.tick`'s `yd -= 0.04 * g`).
    pub gravity: f64,
    /// The friction field (the base default 0.98; the poof's 0.9).
    pub friction: f64,
    /// The quad's own size (the corner spans ± size / 2 — the render
    /// fold the pieces lane speaks).
    pub quad_size: f32,
    /// The uv walk's offsets (the terrain's uo/vo law, `r * 3.0`).
    pub uo: f32,
    pub vo: f32,
    /// The vertex colors (the terrain's 0.6 grey, the poof's own
    /// grey, the crit's fading g/b).
    pub col: [f32; 3],
}

impl Particle {
    /// The base `Particle.tick` verbatim: the age walk (removed at the
    /// lifetime), the gravity lane (`yd -= 0.04 * gravity`), the move
    /// (the collision lane sleeps — the pieces' honest absence), the
    /// friction lane, the onGround 0.7 fold (invisible without
    /// collision). The per-kind tails follow: the poof's sprite walk
    /// (the frame rides `age`), the crit's color fade (`gCol *=
    /// 0.96F`, `bCol *= 0.9F`).
    pub fn tick(&mut self) -> bool {
        self.age += 1;
        if self.age >= self.lifetime {
            return false;
        }
        self.vy -= 0.04 * self.gravity;
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
        self.vx *= self.friction;
        self.vy *= self.friction;
        self.vz *= self.friction;
        match self.kind {
            ParticleKind::Crit => {
                // javap CritParticle.tick: super.tick() then the two
                // color folds — the star cools as it flies
                self.col[1] *= 0.96;
                self.col[2] *= 0.9;
            }
            _ => {}
        }
        true
    }

    /// The render's half-span (the corners ride ± half — the pieces'
    /// own fold): the crit's growth law multiplies first.
    pub fn half(&self) -> f32 {
        let base = self.quad_size * 0.5;
        match self.kind {
            ParticleKind::Crit => base * crit_grow(self.age, self.lifetime, 0.0),
            _ => base,
        }
    }

    /// The `BubbleParticle.tick` verbatim — the bubble rides NONE of
    /// the base law: the countdown lifetime (`original > 0` gates the
    /// body — no motion on the removal tick, exactly the spawn count
    /// of motions), the buoyancy `yd += 0.002` BEFORE the move (no
    /// gravity lane — the bubble RISES), the move, the friction
    /// `*= 0.85` AFTER the move, and the dry death (the fluid at the
    /// containing cell leaving WATER removes — the caller answers the
    /// gate with the active world's own read, the client's fluid
    /// query the body's eye scan speaks).
    pub fn tick_bubble(&mut self, in_water: bool) -> bool {
        // the countdown's own gate: `original = lifetime; lifetime -=
        // 1; if (original <= 0) remove` — the u32 walk stops at zero
        // (the vanilla's int never counts below the removal)
        if self.lifetime == 0 {
            return false;
        }
        self.lifetime -= 1;
        self.vy += 0.002;
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
        self.vx *= 0.85;
        self.vy *= 0.85;
        self.vz *= 0.85;
        in_water
    }
}

/// The terrain particle's ctor (javap `TerrainParticle` verbatim): the
/// base ctor's size then `/ 2`, the 0.6 grey, the gravity 1.0, the
/// `uo/vo = r * 3.0` pair, the base lifetime law. Draw order (the ctor
/// chain's own): the base lifetime's float FIRST (the Particle ctor
/// runs before the SingleQuadParticle's size and the terrain's own
/// uo/vo pair).
pub fn terrain(
    tile: u16,
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    r_life: f32,
    r_size: f32,
    r_uo: f32,
    r_vo: f32,
) -> Particle {
    let quad_size = base_quad_size(r_size) / 2.0;
    Particle {
        kind: ParticleKind::Terrain { tile },
        x,
        y,
        z,
        vx,
        vy,
        vz,
        age: 0,
        lifetime: base_lifetime(r_life),
        gravity: 1.0,
        friction: 0.98,
        quad_size,
        uo: r_uo * 3.0,
        vo: r_vo * 3.0,
        col: [0.6; 3],
    }
}

/// The poof particle's ctor (javap `ExplodeParticle` verbatim): the
/// gravity -0.1, the friction 0.9, the velocity's ±0.05 jitter over
/// the caller's base velocity, the grey `r * 0.3 + 0.7`, the size
/// `0.1 * (r * r * 6 + 1)`, the lifetime `16 / (r * 0.8 + 0.2) + 2`.
/// Draw order: the BASE ctor's lifetime float first (the particle ctor
/// chain runs before the ExplodeParticle's own — its draw rides the
/// base law then gets overridden by the 16/(r*0.8+0.2)+2 fold), then
/// the ctor's own vx jitter, vy jitter, vz jitter, grey, size's two
/// floats, lifetime's.
pub fn poof(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    _r_base_life: f32,
    r_jx: f32,
    r_jy: f32,
    r_jz: f32,
    r_grey: f32,
    r_s1: f32,
    r_s2: f32,
    r_life: f32,
) -> Particle {
    let grey = r_grey * 0.3 + 0.7;
    Particle {
        kind: ParticleKind::Poof,
        x,
        y,
        z,
        vx: vx + (r_jx * 2.0 - 1.0) as f64 * 0.05,
        vy: vy + (r_jy * 2.0 - 1.0) as f64 * 0.05,
        vz: vz + (r_jz * 2.0 - 1.0) as f64 * 0.05,
        age: 0,
        lifetime: (16.0f32 / (r_life * 0.8 + 0.2)) as u32 + 2,
        gravity: -0.1,
        friction: 0.9,
        quad_size: 0.1 * (r_s1 * r_s2 * 6.0 + 1.0),
        uo: 0.0,
        vo: 0.0,
        col: [grey, grey, grey],
    }
}

/// The crit particle's ctor (the base ctor's own defaults — the star's
/// growth and fade ride the tick and the render).
pub fn crit(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    r_life: f32,
    r_size: f32,
) -> Particle {
    Particle {
        kind: ParticleKind::Crit,
        x,
        y,
        z,
        vx,
        vy,
        vz,
        age: 0,
        lifetime: base_lifetime(r_life),
        gravity: 0.0,
        friction: 0.98,
        quad_size: base_quad_size(r_size),
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
}

/// The bubble particle's ctor (javap `BubbleParticle` verbatim). Draw
/// order (the ctor chain's own): the BASE ctor's dead lifetime float
/// first (the law gets overridden by the 8/(r*0.8+0.2) fold — the draw
/// still counts), the SingleQuadParticle's 0.1..0.2 quadSize draw
/// (`setSize(0.02, 0.02)` only moves the BOUNDING BOX — the quad's own
/// size the chain keeps and the bubble's fold scales), then the
/// bubble's own: the quadSize *= r*0.6+0.2 fold, the three velocity
/// folds (`xa * 0.2 + (r * 2 - 1) * 0.02` — the caller's velocity
/// thinned to a fifth, the ±0.02 jitter added), the lifetime
/// `(int)(8.0 / (r * 0.8 + 0.2))` (8..40). The white color rides the
/// SingleQuadParticle's defaults (the bubble sprite's own pixels).
#[allow(clippy::too_many_arguments)]
pub fn bubble(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    r_base_life: f32,
    r_size0: f32,
    r_scale: f32,
    r_jx: f32,
    r_jy: f32,
    r_jz: f32,
    r_life: f32,
) -> Particle {
    let _ = r_base_life; // the dead draw the chain's order keeps
    let quad_size = base_quad_size(r_size0) * (r_scale * 0.6 + 0.2);
    Particle {
        kind: ParticleKind::Bubble,
        x,
        y,
        z,
        vx: vx * 0.2 + (r_jx * 2.0 - 1.0) as f64 * 0.02,
        vy: vy * 0.2 + (r_jy * 2.0 - 1.0) as f64 * 0.02,
        vz: vz * 0.2 + (r_jz * 2.0 - 1.0) as f64 * 0.02,
        age: 0,
        lifetime: (8.0f32 / (r_life * 0.8 + 0.2)) as u32,
        gravity: 0.0,
        friction: 0.85,
        quad_size,
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
}

/// The drown cloud (`LivingEntity.makeDrownParticles` verbatim — the
/// broadcast-67 arm the handleEntityEvent speaks): 8 bubbles around
/// the body, three `triangle(0.0, 1.0)` offsets per particle (the
/// `min + max * (d1 - d2)` triangle — the [-1, 1] spread the
/// RandomSource default speaks), the spawn at `(x + d3, y + d5,
/// z + d7)` (the FEET the getX/getY/getZ law opens), the velocity the
/// body's OWN delta movement (the vec3 the method reads first). Draw
/// order per particle: the six triangle doubles, then the ctor
/// chain's seven draws (the dead lifetime float, the quadSize, the
/// scale fold, the three jitters, the lifetime's).
pub fn drown_bubbles(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::with_capacity(8);
    for _ in 0..8 {
        // three triangle(0, 1) offsets — the default method's
        // `min + max * (nextDouble() - nextDouble())`, one pair each
        let d3 = rng.next_f64() - rng.next_f64();
        let d5 = rng.next_f64() - rng.next_f64();
        let d7 = rng.next_f64() - rng.next_f64();
        // the ctor chain's seven draws (the base lifetime's dead float
        // the order speaks, the quadSize, the scale, the three
        // jitters, the lifetime's own)
        let r_base_life = rng.next_f32();
        let r_size0 = rng.next_f32();
        let r_scale = rng.next_f32();
        let r_jx = rng.next_f32();
        let r_jy = rng.next_f32();
        let r_jz = rng.next_f32();
        let r_life = rng.next_f32();
        out.push(bubble(
            x + d3,
            y + d5,
            z + d7,
            vx,
            vy,
            vz,
            r_base_life,
            r_size0,
            r_scale,
            r_jx,
            r_jy,
            r_jz,
            r_life,
        ));
    }
    out
}

/// The 2001 event's own burst (`ClientLevel.addDestroyBlockEffect`'s
/// lambda verbatim) over one full-cube box: the box's extents
/// `min(1.0, max - min)` (the cube's 1.0), the sections
/// `max(2, ceil(extent / 0.25))` (the cube's 4x4x4 = 64), the
/// particle at the cell's fraction with the outward velocity. Draws
/// per particle: none — the lambda's fractions ride no randoms (the
/// randoms ride the caller's terrain ctor: the size, the uo/vo, the
/// lifetime — three floats per particle).
pub fn terrain_burst(tile: u16, bx: i32, by: i32, bz: i32, rng: &mut crate::drops::Lcg) -> Vec<Particle> {
    let (min_x, min_y, min_z, max_x, max_y, max_z) = (0.0f64, 0.0, 0.0, 1.0, 1.0, 1.0);
    let dx = 1.0f64.min(max_x - min_x);
    let dy = 1.0f64.min(max_y - min_y);
    let dz = 1.0f64.min(max_z - min_z);
    let sx = 2usize.max((dx / 0.25).ceil() as usize);
    let sy = 2usize.max((dy / 0.25).ceil() as usize);
    let sz = 2usize.max((dz / 0.25).ceil() as usize);
    let mut out = Vec::with_capacity(sx * sy * sz);
    for ix in 0..sx {
        for iy in 0..sy {
            for iz in 0..sz {
                let fx = (ix as f64 + 0.5) / sx as f64;
                let fy = (iy as f64 + 0.5) / sy as f64;
                let fz = (iz as f64 + 0.5) / sz as f64;
                let px = fx * dx + min_x;
                let py = fy * dy + min_y;
                let pz = fz * dz + min_z;
                // the terrain ctor chain's own four draws (life, size,
                // uo, vo — the base ctor speaks first)
                let r_life = rng.next_f32();
                let r_size = rng.next_f32();
                let r_uo = rng.next_f32();
                let r_vo = rng.next_f32();
                out.push(terrain(
                    tile,
                    bx as f64 + px,
                    by as f64 + py,
                    bz as f64 + pz,
                    fx - 0.5,
                    fy - 0.5,
                    fz - 0.5,
                    r_life,
                    r_size,
                    r_uo,
                    r_vo,
                ));
            }
        }
    }
    out
}

/// The dig's own dust (`ClientLevel.addBreakingBlockEffect` verbatim)
/// over one full-cube cell: the random point inset (three
/// `nextDouble()` draws), then the face arm moving the axis 0.1
/// beyond the face's bound. Face encoding (the mesher's own): 0 = +X
/// (EAST), 1 = -X (WEST), 2 = +Y (UP), 3 = -Y (DOWN), 4 = +Z (SOUTH),
/// 5 = -Z (NORTH). The velocity (0,0,0); the `scale(0.6F)` shrinks
/// the quad (the base size times 0.6); the `setPower(0.2F)` folds
/// into the zero velocity (the honest note the javap speaks).
pub fn dig_dust(
    tile: u16,
    bx: i32,
    by: i32,
    bz: i32,
    face: u8,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the inset walk (the bb's own [0,1] cube): `r * (max - min - 0.2)
    // + 0.1 + min`
    let mut dx = bx as f64 + rng.next_f64() * 0.8 + 0.1;
    let mut dy = by as f64 + rng.next_f64() * 0.8 + 0.1;
    let mut dz = bz as f64 + rng.next_f64() * 0.8 + 0.1;
    // the face arm (the switch's own six arms)
    match face {
        3 => dy = by as f64 - 0.1,       // DOWN: minY - 0.1
        2 => dy = by as f64 + 1.0 + 0.1, // UP: maxY + 0.1
        5 => dz = bz as f64 - 0.1,       // NORTH: minZ - 0.1
        4 => dz = bz as f64 + 1.0 + 0.1, // SOUTH: maxZ + 0.1
        1 => dx = bx as f64 - 0.1,       // WEST: minX - 0.1
        0 => dx = bx as f64 + 1.0 + 0.1, // EAST: maxX + 0.1
        _ => {}
    }
    // the ctor chain's four draws (life, size, uo, vo); the scale fold
    let r_life = rng.next_f32();
    let r_size = rng.next_f32();
    let r_uo = rng.next_f32();
    let r_vo = rng.next_f32();
    let mut p = terrain(tile, dx, dy, dz, 0.0, 0.0, 0.0, r_life, r_size, r_uo, r_vo);
    p.quad_size *= 0.6; // javap Particle.scale(0.6F): quadSize *= 0.6
    p
}

/// The death cloud (`LivingEntity.makePoofParticles` verbatim): 20
/// poof particles around the body — the gaussian `d * 0.02` velocity,
/// the spawn walked BACK along the velocity (`getRandomX(1.0) - d0 *
/// 10.0` etc). Draw order per particle (the bytecode's own): three
/// gaussians, then the three position randoms (`nextDouble` pairs
/// inside getRandomX/Z, one for getRandomY).
///
/// The bbox rides the caller's own fold (the zombie's
/// `(x - w/2, y, z - w/2, x + w/2, y + height, z + w/2)` the mob
/// speaks): `getRandomX(1.0)` walks the width, `getRandomY()` the
/// height.
pub fn poof_cloud(
    bbox: (f64, f64, f64, f64, f64, f64),
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::with_capacity(20);
    for _ in 0..20 {
        // three gaussian draws (the Marsaglia polar method the Lcg
        // speaks through next_gaussian)
        let d0 = rng.next_gaussian() * 0.02;
        let d1 = rng.next_gaussian() * 0.02;
        let d2 = rng.next_gaussian() * 0.02;
        // the three position randoms (getRandomX(1.0), getRandomY(),
        // getRandomZ(1.0) — one nextDouble each, the ±1 walk the
        // width's own law speaks)
        let rx = rng.next_f64() * 2.0 - 1.0;
        let ry = rng.next_f64();
        let rz = rng.next_f64() * 2.0 - 1.0;
        let (bw, bh) = ((bbox.3 - bbox.0) * 0.5, bbox.5 - bbox.1);
        let px = bbox.0 + bw + bw * rx - d0 * 10.0;
        let py = bbox.1 + bh * ry - d1 * 10.0;
        let pz = bbox.2 + bw + bw * rz - d2 * 10.0;
        // the ctor chain's draws: the base lifetime's float (the base
        // ctor runs first — its law gets overridden), then the ctor's
        // own three jitters, the grey, the size's two, the lifetime's
        let r_base_life = rng.next_f32();
        let r_jx = rng.next_f32();
        let r_jy = rng.next_f32();
        let r_jz = rng.next_f32();
        let r_grey = rng.next_f32();
        let r_s1 = rng.next_f32();
        let r_s2 = rng.next_f32();
        let r_life = rng.next_f32();
        out.push(poof(
            px, py, pz, d0, d1, d2, r_base_life, r_jx, r_jy, r_jz, r_grey, r_s1,
            r_s2, r_life,
        ));
    }
    out
}

/// The crit's own emitter (javap `TrackingEmitter` verbatim): the
/// target's frozen position (the removed entity keeps its own), 3
/// ticks of life, the 16-sample walk per tick.
pub struct CritEmitter {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// The emitter's own life (3 ticks — the ctor's iconst_3).
    pub life: u32,
    /// The target's id (the emitter follows the LIVE mob; the removed
    /// one freezes the position the last read spoke).
    pub target_id: i32,
}

/// The zombie's own bbox halves (the mob lane's constants — the
/// emitter's fractions walk the same box).
pub const EMITTER_BB_WIDTH: f64 = crate::mob::ZOMBIE_WIDTH;
pub const EMITTER_BB_HEIGHT: f64 = crate::mob::ZOMBIE_HEIGHT;

impl CritEmitter {
    /// The ctor (the first read freezes the spawn position).
    pub fn new(target_id: i32, bbox: (f64, f64, f64, f64, f64, f64)) -> CritEmitter {
        CritEmitter {
            x: bbox.0 + (bbox.3 - bbox.0) * 0.5,
            y: bbox.1,
            z: bbox.2 + (bbox.5 - bbox.2) * 0.5,
            life: 0,
            target_id,
        }
    }

    /// The tick (javap `TrackingEmitter.tick` verbatim): the position
    /// refresh from the live target, the 16-sample walk (the unit-ball
    /// reject), the spawn at the bbox fractions, the life walk
    /// (removed at 3). Returns the spawned crits + the emitter's own
    /// survival.
    pub fn tick(
        &mut self,
        target: Option<(f64, f64, f64)>,
        rng: &mut crate::drops::Lcg,
    ) -> (Vec<Particle>, bool) {
        // the live target's position (the removed entity keeps the
        // last read — the frozen fold)
        if let Some((tx, ty, tz)) = target {
            self.x = tx;
            self.y = ty;
            self.z = tz;
        }
        let mut out = Vec::new();
        for _ in 0..16 {
            // three draws (nextFloat * 2 - 1), the ball reject
            let d0 = rng.next_f32() as f64 * 2.0 - 1.0;
            let d2 = rng.next_f32() as f64 * 2.0 - 1.0;
            let d4 = rng.next_f32() as f64 * 2.0 - 1.0;
            if d0 * d0 + d2 * d2 + d4 * d4 > 1.0 {
                continue;
            }
            // the bbox fractions (Entity.getX(d0/4) etc over the
            // zombie's own 0.6 x 1.95 box)
            let half = EMITTER_BB_WIDTH * 0.5;
            let px = self.x - half + (half * 2.0) * (d0 / 4.0 + 0.5);
            let py = self.y + EMITTER_BB_HEIGHT * (0.5 + d2 / 4.0);
            let pz = self.z - half + (half * 2.0) * (d4 / 4.0 + 0.5);
            // the ctor's two draws (life, size)
            let r_life = rng.next_f32();
            let r_size = rng.next_f32();
            out.push(crit(px, py, pz, d0, d2 + 0.2, d4, r_life, r_size));
        }
        self.life += 1;
        let alive = self.life < 3;
        (out, alive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The law's own fixture: the Lcg the caller lanes speak.
    fn rng() -> crate::drops::Lcg {
        crate::drops::Lcg::new(0x5EED_10CA)
    }

    #[test]
    fn the_base_lifetime_law_spans_the_vanilla_band() {
        // (int)(4.0 / (r * 0.9 + 0.1)): r=0 -> 40, r>=0.75 -> 4
        assert_eq!(base_lifetime(0.0), 40);
        assert_eq!(base_lifetime(1.0), 4);
        assert_eq!(base_lifetime(0.25), 12); // 4 / 0.325 = 12.3 -> 12
        assert_eq!(base_lifetime(0.5), 7); // 4 / 0.55 = 7.27 -> 7
    }

    #[test]
    fn the_base_quad_size_spans_the_vanilla_band() {
        assert!((base_quad_size(0.0) - 0.1).abs() < 1e-6);
        assert!((base_quad_size(1.0) - 0.2).abs() < 1e-6);
    }

    #[test]
    fn the_terrain_particle_ctor_speaks_the_bytecode() {
        // draw order (the ctor chain's own): life, size, uo, vo
        let p = terrain(7, 1.0, 2.0, 3.0, 0.1, 0.2, 0.3, 0.0, 0.5, 0.25, 0.0);
        // the gravity the ctor sets, the 0.6 grey, the size halved
        assert!((p.gravity - 1.0).abs() < 1e-6);
        assert!((p.col[0] - 0.6).abs() < 1e-6 && (p.col[2] - 0.6).abs() < 1e-6);
        assert!((p.quad_size - base_quad_size(0.5) / 2.0).abs() < 1e-6);
        assert!((p.uo - 0.75).abs() < 1e-6); // r * 3.0
        assert!((p.vo - 0.0).abs() < 1e-6);
        assert_eq!(p.lifetime, base_lifetime(0.0));
        // the friction rides the base default
        assert!((p.friction - 0.98).abs() < 1e-6);
    }

    #[test]
    fn the_poof_ctor_speaks_the_bytecode() {
        let p = poof(
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
        );
        // the gravity -0.1 (it rises), the friction 0.9, the grey band
        assert!((p.gravity - (-0.1)).abs() < 1e-6);
        assert!((p.friction - 0.9).abs() < 1e-6);
        // the jitter (0.5 * 2 - 1) * 0.05 = 0 — the mid draw is still
        assert!(p.vx.abs() < 1e-9);
        assert!((p.col[0] - (0.5 * 0.3 + 0.7)).abs() < 1e-6);
        // the size: 0.1 * (0.25 * 6 + 1) = 0.25
        assert!((p.quad_size - 0.1 * (0.5 * 0.5 * 6.0 + 1.0)).abs() < 1e-6);
        // the lifetime: 16 / 0.6 = 26.67 -> 26 + 2 = 28
        assert_eq!(p.lifetime, 28);
    }

    #[test]
    fn the_tick_rides_the_base_physics() {
        let mut p = terrain(1, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        p.lifetime = 3;
        // the gravity lane first (0.04 * 1.0), the move, THEN the
        // friction (the base tick's own order — the velocity the next
        // tick inherits rides the 0.98 fold)
        assert!(p.tick());
        assert!((p.vy - (-0.04 * 0.98)).abs() < 1e-9);
        assert!((p.x - 1.0).abs() < 1e-9);
        assert!((p.vx - 0.98).abs() < 1e-9);
        // the age walk removes at the lifetime
        assert!(p.tick());
        assert!(!p.tick());
        assert_eq!(p.age, 3);
    }

    #[test]
    fn the_crit_tick_fades_g_and_b() {
        let mut p = crit(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        p.lifetime = 40;
        assert!(p.tick());
        assert!((p.col[1] - 0.96).abs() < 1e-6);
        assert!((p.col[2] - 0.9).abs() < 1e-6);
        assert!((p.col[0] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn the_burst_walks_the_four_grid_and_speaks_the_outward_velocity() {
        let mut r = rng();
        let burst = terrain_burst(7, 10, 20, 30, &mut r);
        // the full cube's 4x4x4
        assert_eq!(burst.len(), 64);
        // the fractions: the first cell's center (0.125, 0.125, 0.125)
        let first = &burst[0];
        assert!((first.x - 10.125).abs() < 1e-9);
        assert!((first.y - 20.125).abs() < 1e-9);
        assert!((first.z - 30.125).abs() < 1e-9);
        assert!((first.vx - (0.125 - 0.5)).abs() < 1e-9);
        assert!((first.vy - (0.125 - 0.5)).abs() < 1e-9);
        assert!((first.vz - (0.125 - 0.5)).abs() < 1e-9);
        // the last cell's center (0.875)
        let last = burst.last().unwrap();
        assert!((last.x - 10.875).abs() < 1e-9);
        // the terrain kind rides the caller's tile
        assert_eq!(first.kind, ParticleKind::Terrain { tile: 7 });
    }

    #[test]
    fn the_dust_walks_the_face_arms() {
        let mut r = rng();
        // the EAST face (+X): x rides the bound + 0.1
        let p = dig_dust(7, 5, 5, 5, 0, &mut r);
        assert!((p.x - (5.0 + 1.0 + 0.1)).abs() < 1e-9);
        // the inset walk's own band: y, z stay inside
        assert!(p.y > 5.0 && p.y < 6.0 && p.z > 5.0 && p.z < 6.0);
        // the UP face: y rides the bound + 0.1
        let p = dig_dust(7, 5, 5, 5, 2, &mut r);
        assert!((p.y - (5.0 + 1.0 + 0.1)).abs() < 1e-9);
        // the DOWN face: y rides the bound - 0.1
        let p = dig_dust(7, 5, 5, 5, 3, &mut r);
        assert!((p.y - (5.0 - 0.1)).abs() < 1e-9);
        // the dust never moves (the zero velocity through the power)
        assert!(p.vx.abs() < 1e-9 && p.vy.abs() < 1e-9 && p.vz.abs() < 1e-9);
    }

    #[test]
    fn the_dust_scale_shrinks_the_quad() {
        // the seed replay: the same draws the ctor chain speaks — the
        // three f64 insets, then life, size, uo, vo — pin the size's
        // 0.6 fold against the plain terrain law
        let mut r = rng();
        let p = dig_dust(7, 0, 0, 0, 0, &mut r);
        let mut r2 = rng();
        let _ = r2.next_f64();
        let _ = r2.next_f64();
        let _ = r2.next_f64();
        let r_life = r2.next_f32();
        let r_size = r2.next_f32();
        let expected = base_quad_size(r_size) / 2.0 * 0.6;
        assert!((p.quad_size - expected).abs() < 1e-6);
        assert_eq!(p.lifetime, base_lifetime(r_life));
    }

    #[test]
    fn the_poof_cloud_walks_twenty_and_the_gaussian_sanity() {
        let mut r = rng();
        let bbox = (-0.3, 0.0, -0.3, 0.3, 1.95, 0.3);
        let cloud = poof_cloud(bbox, &mut r);
        assert_eq!(cloud.len(), 20);
        // every particle rides the poof kind and the gaussian drift
        for p in &cloud {
            assert_eq!(p.kind, ParticleKind::Poof);
            // the |d| band: gauss * 0.02 keeps the velocity tiny
            assert!(p.vx.abs() < 0.5 && p.vy.abs() < 0.5 && p.vz.abs() < 0.5);
        }
        // the gaussian sanity (the honest band): the polar method's
        // own law rides THIS Lcg's sequential pairs — the pair walk
        // biases the 200-draw sum (the sim pinned ±20); the cloud's
        // drift stays subtle at the 0.02 scale the law speaks
        let mut g = rng();
        let mut sum = 0.0f64;
        for _ in 0..200 {
            let v = g.next_gaussian();
            assert!(v.is_finite());
            sum += v;
        }
        assert!(sum.abs() < 25.0, "the gaussian stream stays in the band");
    }

    #[test]
    fn the_emitter_walks_the_ball_reject_and_dies_at_three() {
        let mut r = rng();
        let bbox = (-0.3, 0.0, -0.3, 0.3, 1.95, 0.3);
        let mut em = CritEmitter::new(1, bbox);
        // the frozen position: the box's center
        assert!((em.x - 0.0).abs() < 1e-9 && (em.z - 0.0).abs() < 1e-9);
        let (a, alive) = em.tick(Some((10.0, 0.0, 10.0)), &mut r);
        assert!(alive && !a.is_empty());
        // the live target's position rides the read
        for p in &a {
            assert!((p.x - 10.0).abs() < 0.35 && (p.z - 10.0).abs() < 0.35);
            assert!(p.y >= 0.0 && p.y <= 1.95 + 0.01);
            assert_eq!(p.kind, ParticleKind::Crit);
            // the velocity: (d0, d2 + 0.2, d4) — the y lane lifts
            assert!(p.vy > -1.0 && p.vy < 1.2);
        }
        let (_b, alive) = em.tick(None, &mut r);
        assert!(alive);
        let (_c, alive) = em.tick(None, &mut r);
        assert!(!alive);
    }

    #[test]
    fn the_poof_frame_walk_speaks_the_sprite_set_law() {
        // age * (size - 1) / lifetime — 8 frames
        assert_eq!(poof_frame(0, 28), 0);
        assert_eq!(poof_frame(27, 28), 6); // 27 * 7 / 28 = 6.75 -> 6
        assert_eq!(poof_frame(28, 28), 7);
    }

    #[test]
    fn the_crit_grow_speaks_the_clamp_law() {
        // the star blooms inside the first lifetime/32 ticks
        assert_eq!(crit_grow(0, 32, 0.0), 0.0);
        assert_eq!(crit_grow(1, 32, 0.0), 1.0);
        assert_eq!(crit_grow(0, 40, 0.5), (0.5f32 / 40.0 * 32.0).clamp(0.0, 1.0));
    }

    #[test]
    fn the_bubble_ctor_speaks_the_bytecode_chain() {
        // the draw order's own math: the dead lifetime float (kept for
        // the order), the quadSize 0.1..0.2 riding the 0.2..0.8 scale,
        // the velocity thinned to a fifth + the ±0.02 jitter, the
        // lifetime 8/(r*0.8+0.2) — 8..40
        let p = bubble(1.0, 2.0, 3.0, 0.5, -0.3, 0.1, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 0.0);
        assert_eq!(p.kind, ParticleKind::Bubble);
        assert_eq!(p.lifetime, 40); // r=0 -> 8/0.2
        assert!((p.quad_size - 0.1 * 0.2).abs() < 1e-6); // size0=0 -> 0.1, scale=0 -> *0.2
        assert!((p.vx - (0.5 * 0.2 + 0.0 * 0.02)).abs() < 1e-9); // jx=0.5 -> jitter 0
        assert!((p.vy - (-0.3 * 0.2)).abs() < 1e-9);
        assert!((p.vz - (0.1 * 0.2 + 0.0 * 0.02)).abs() < 1e-9);
        assert_eq!(p.col, [1.0, 1.0, 1.0]); // the white the defaults speak
        let big = bubble(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(big.lifetime, 8); // r=1 -> 8/1.0
        assert!((big.quad_size - 0.2 * 0.8).abs() < 1e-6);
        assert!((big.vx - (-0.02)).abs() < 1e-9); // jx=0 -> -1 * 0.02
    }

    #[test]
    fn the_bubble_tick_rides_its_own_law() {
        // the countdown gate: exactly `lifetime` motion ticks, then
        // the removal (no motion on the removal tick)
        let mut p = bubble(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0);
        let total = p.lifetime;
        let y0 = p.y;
        for _ in 0..total {
            assert!(p.tick_bubble(true), "alive inside the countdown");
        }
        // the countdown spent: the next tick removes — the y never
        // moves the dead tick speaks
        let y_last = p.y;
        assert!(!p.tick_bubble(true));
        assert_eq!(p.y, y_last);
        assert!(total >= 8 && total <= 40, "the vanilla band");
        // the buoyancy BEFORE the move, the friction AFTER: the first
        // tick's vy = 0*0.2 + jitter + 0.002, then *= 0.85
        let mut q = bubble(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0);
        let vy_seed = q.vy;
        q.tick_bubble(true);
        let expected = (vy_seed + 0.002) * 0.85;
        assert!((q.vy - expected).abs() < 1e-12, "vy {} vs {}", q.vy, expected);
        assert!(q.y > y0, "the bubble RISES");
        // no gravity lane: the vy never falls — the friction's own
        // walk rides the buoyancy's climb
        let vy1 = q.vy;
        q.tick_bubble(true);
        assert!((q.vy - (vy1 + 0.002) * 0.85).abs() < 1e-12);
    }

    #[test]
    fn the_bubble_dies_the_dry_cell_speaks() {
        // the water gate: the fluid read leaving WATER removes the
        // same tick (the caller's own answer)
        let mut p = bubble(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0);
        assert!(p.tick_bubble(true));
        assert!(!p.tick_bubble(false));
        // and the wet answer never kills the countdown's own death
        let mut q = bubble(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0);
        while q.lifetime > 0 {
            assert!(q.tick_bubble(true));
        }
        assert!(!q.tick_bubble(true));
    }

    #[test]
    fn the_drown_cloud_walks_eight_and_the_body_velocity() {
        let mut r = rng();
        let cloud = drown_bubbles(10.0, 20.0, 30.0, 0.4, 0.2, -0.1, &mut r);
        assert_eq!(cloud.len(), 8);
        for p in &cloud {
            assert_eq!(p.kind, ParticleKind::Bubble);
            // the triangle offsets spread ±1 around the feet
            assert!(p.x > 9.0 && p.x < 11.0);
            assert!(p.y > 19.0 && p.y < 21.0);
            assert!(p.z > 29.0 && p.z < 31.0);
            // the velocity thinned to a fifth + the ±0.02 jitter
            assert!(p.vx > 0.4 * 0.2 - 0.02 && p.vx < 0.4 * 0.2 + 0.02);
            assert!(p.vz > -0.1 * 0.2 - 0.02 && p.vz < -0.1 * 0.2 + 0.02);
            assert!(p.lifetime >= 8 && p.lifetime <= 40);
        }
        // the seed-replayed fold: the same seed walks the same cloud
        let mut a = rng();
        let mut b = rng();
        let ca = drown_bubbles(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, &mut a);
        let cb = drown_bubbles(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, &mut b);
        for (pa, pb) in ca.iter().zip(cb.iter()) {
            assert_eq!(pa.x, pb.x);
            assert_eq!(pa.lifetime, pb.lifetime);
            assert_eq!(pa.quad_size, pb.quad_size);
        }
    }
}

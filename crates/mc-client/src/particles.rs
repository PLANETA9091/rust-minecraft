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
/// The index rides the SPRITE LIST the json speaks (smoke.json /
/// poof.json both list `generic_7..generic_0` — descending), NOT the
/// file order: index 0 reads the list's FIRST entry, generic_7.
pub fn poof_frame(age: u32, lifetime: u32) -> usize {
    (age as usize * (POOF_FRAME_COUNT - 1)) / lifetime.max(1) as usize
}

/// The sprite list's own direction (the two jsons' descending order):
/// the list index the `get` law speaks maps to the generic_N FILE via
/// `count - 1 - index` — the pool's smoke pops in the DENSE puff
/// (generic_7, 29/64 opaque pixels the decode speaks) and dissolves
/// into the wisp (generic_0, 1/64). The poof and the smoke share the
/// one law (the two jsons' identical texture lists).
pub fn generic_file_index(list_index: usize) -> usize {
    (POOF_FRAME_COUNT - 1).saturating_sub(list_index)
}

/// The smoke's render-file index (the full chain the smoke's frame
/// ride speaks): the `get(age, lifetime)` list index, then the json's
/// descending order fold.
pub fn smoke_sprite_file_index(age: u32, lifetime: u32) -> usize {
    generic_file_index(poof_frame(age, lifetime))
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

/// The lava drip's own lifetime law (javap `LavaParticle` ctor:
/// `(int)(16.0D / (nextFloat() * 0.8D + 0.2D))`) — 16..80 ticks.
pub fn lava_lifetime(r: f32) -> u32 {
    (16.0f32 / (r * 0.8 + 0.2)) as u32
}

/// The smoke child's own lifetime law (javap
/// `BaseAshSmokeParticle` ctor: `(int)(lifetimeBase / (nextFloat() *
/// 0.8D + 0.2D) * scaleMul)` with the SMOKE provider's base 8 and
/// scale 1.0, then the `Math.max(lifetime, 1)` floor) — 8..40 ticks.
pub fn smoke_lifetime(r: f32) -> u32 {
    let life = (8.0f32 / (r * 0.8 + 0.2)) as u32;
    life.max(1)
}

/// The flame's own lifetime law (javap `RisingParticle` ctor verbatim:
/// the f64 fold the bytecode speaks — `ldc2_w 8.0; nextFloat; f2d;
/// ldc2_w 0.8; dmul; ldc2_w 0.2; dadd; ddiv; d2i; iconst_4; iadd`) —
/// `(int)(8.0 / (r * 0.8 + 0.2)) + 4` (the +4 the iconst tail adds).
/// The f64 constants' own edges speak: r=0 draws 39.99999940… → 39 + 4
/// = 43 (the 0.2f32 the f64 widens, the truncation the d2i speaks),
/// r->1 draws 7.99999988… → 7 + 4 = 11 — the grid the nextFloat
/// speaks never draws either edge draw (the [12, 43] band the stream
/// holds, the f64 domain the smoke helper's f32 stand-in never
/// leaves).
pub fn flame_lifetime(r: f32) -> u32 {
    let denom = r as f64 * f64::from(0.8f32) + f64::from(0.2f32);
    (8.0f64 / denom) as u32 + 4
}

/// The lava drip's render-shrink law (javap `LavaParticle.getQuadSize`:
/// `quadSize * (1 - t * t)` with `t = (age + partial) / lifetime`) —
/// the blob thins quadratically to nothing at its death.
pub fn lava_shrink(age: u32, lifetime: u32, partial: f32) -> f32 {
    let t = (age as f32 + partial) / lifetime.max(1) as f32;
    1.0 - t * t
}

/// The flame's render-shrink law (javap `FlameParticle.getQuadSize`
/// verbatim: `quadSize * (1.0F - t * t * 0.5F)` with `t = (age +
/// partial) / lifetime`) — the flame thins to HALF its birth quad at
/// the death (the 0.5 fold the lava's own (1 - t*t) never keeps).
pub fn flame_shrink(age: u32, lifetime: u32, partial: f32) -> f32 {
    let t = (age as f32 + partial) / lifetime.max(1) as f32;
    1.0 - t * t * 0.5
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
    /// `CritParticle` the DAMAGE_INDICATOR provider speaks — the same
    /// class the crit rides (the fade, the zero gravity), the own
    /// `damage` sprite the json's one-texture set holds, the lifetime
    /// the provider's setLifetime(20) pins.
    Damage,
    /// `BubbleParticle` — the drown bubble (the bubble texture, the
    /// full sprite the quad samples, the body's own rise).
    Bubble,
    /// `SplashParticle` — the entry splash's droplet (the splash
    /// sprite set's random frame the Provider's `SpriteSet.get`
    /// speaks, the drop's own countdown tick).
    Splash { frame: usize },
    /// `LavaParticle` — the lava pool's own drip (the single 8x8 lava
    /// sprite the `lava` particle type speaks, the pop the ctor's yd
    /// draw gives, the full block light the getLightCoords override
    /// speaks, the shrink the (1 - t²) fold rides).
    Lava,
    /// `SmokeParticle` — the drip's own child (the SMOKE type the
    /// LavaParticle.tick's draw speaks, the BaseAshSmokeParticle laws:
    /// the rise the −0.1 gravity gives, the grey tint the r*0.3 draw,
    /// the 0.75 size fold, the 8 generic frames the smoke.json's
    /// descending order lists).
    Smoke,
    /// `AttackSweepParticle` — the sword's own arc (the SWEEP_ATTACK
    /// type the `doSweepAttack` sendParticles speaks, the 8 32x32
    /// sweep frames the json's ascending order lists, the static pose
    /// the tick override keeps — no motion, no gravity, the 4-tick
    /// life, the grey tint the r*0.6+0.4 draw, the quadSize the
    /// `1.0 - xSpeed * 0.5` fold the ctor's velocity arg gives, the
    /// full-bright look the getLightCoords override speaks).
    Sweep,
    /// `HeartParticle` — the love burst's own red (the `heart` type
    /// the event-18 broadcast spawns, the aiStep's ambient drip the
    /// in-love walk speaks; javap verbatim: the friction the ctor's
    /// 0.86 fold keeps, the velocity the `*0.01` super-zeroed fields
    /// the `+=` args own, the quadSize the `*1.5` fold carries, the
    /// lifetime the 26.2's flat 16 speaks, the `hasPhysics = false`
    /// the no-collision client already keeps, the OPAQUE layer the
    /// getLayer override returns, the growth the getQuadSize's 32x
    /// clamp rides — the same law the crit star blooms by).
    Heart,
    /// `SuspendedTownParticle` the happy villager's own green (the
    /// `happy_villager` type the `AgeableMob.aiStep`'s forcedAgeTimer
    /// walk spawns — javap verbatim: the lifetime the `20.0 /
    /// (nextFloat() * 0.8 + 0.2)` fold answers [20, 100], the quadSize
    /// the `setSize(0.02, 0.02)` x the `* (nextFloat() * 0.6 + 0.5)`
    /// draw gives, the zero drift the aiStep's (0,0,0) velocity the
    /// `* 0.02` ctor fold keeps, the friction the tick's own `* 0.99`
    /// speaks (no gravity the law never arms), the white tint the
    /// provider's `setColor(1, 1, 1)` override — the glint texture's
    /// own green the quad samples whole, the OPAQUE layer the
    /// getLayer override returns).
    HappyVillager,
    /// `BubbleColumnUpParticle` — the upward column's own bubble (the
    /// `bubble_column_up` type the column's animateTick spawns two of,
    /// the same `minecraft:bubble` sprite the swim bubble rides, the
    /// −0.125 gravity the base tick's lane turns into the rise —
    /// `yd -= 0.04 * g` ADDS 0.005 a tick — the 0.85 friction the move
    /// keeps, the dry death the fluid gate speaks: the containing cell
    /// leaving WATER removes).
    BubbleColumnUp,
    /// `WaterCurrentDownParticle` — the whirlpool's own sinking droplet
    /// (the `current_down` type the column's animateTick spawns one of,
    /// the same `minecraft:bubble` sprite, the fixed −0.05 birth speed
    /// the ctor's own fields speak, the 0.002 gravity the
    /// WaterDropParticle tick's plain-field pull rides, the 0.98
    /// friction, no fluid gate — the droplet sinks its lifetime whole).
    CurrentDown,
    /// `FlameParticle` — the torch's own flame (the `flame` type the
    /// animateTick pair's second call speaks, the single
    /// `minecraft:flame` sprite the 26.2 json's one-texture list
    /// holds): the RisingParticle laws the ctor chain wears — the
    /// 0.96 friction, the *0.01 velocity fold the addParticle's zero
    /// args speak (the ctor spread the +0.1 yd bump the
    /// normalization adds keeps — the rise the fold gives), the
    /// ±0.05 position jitter, the 8/(r*0.8+0.2)+4 lifetime, the no
    /// gravity the RisingParticle never arms, the OPAQUE layer the
    /// getLayer override returns (the full-bright look the untinted
    /// full sample keeps), the white tint the ctor defaults speak,
    /// the (1 - t*t*0.5) half-thin the getQuadSize override wears.
    Flame,
    /// `SpellParticle` — the splash-potion ring's own colored puff (the
    /// `effect`/`instant_effect` types the `LevelEventHandler` case-2002
    /// arm's `SpellParticleOption.create(ParticleTypes.EFFECT|INSTANT_
    /// EFFECT, r*f, g*f, b*f, d0)` walk speaks, the `instant` flag the
    /// `if_icmpne 2007` branch answers): the frame the age's own walk
    /// rides (`setSpriteFromAge` — the type's own json's descending
    /// list the file fold speaks), the color tint the option's floats
    /// carry (the ctor's `setColor` the provider walks), the −0.1
    /// gravity the ctor passes (the rise `yd -= 0.04 * (-0.1)` gives),
    /// the 0.96 friction, the 0.75 size fold, the 8..40 lifetime the
    /// ctor's own write answers.
    Spell { instant: bool },
    /// `BreakingItemParticle` — the splash-potion debris burst's own
    /// shard (the `ItemParticleOption(ParticleTypes.ITEM, SPLASH_
    /// POTION)` walk the case-2002 arm's eight-fold loop speaks, the
    /// layer the `pickParticleMaterial`'s `nextInt(activeLayerCount)`
    /// fold picks — 0 the `potion_overlay` glass, 1 the `splash_potion`
    /// liquid the model's own two layers keep): the quarter-region uv
    /// walk the terrain dust shares (the uo/vo the ctor's own
    /// `nextFloat() * 3.0` draws give), the gravity 1.0 the ctor
    /// passes, the base 0.98 friction, the `/2` size fold, the base
    /// lifetime the ctor never overrides.
    PotionItem { layer: usize },
}

/// The splash droplet's 4 frames (the `splash_0..3` textures the jar
/// speaks — the sprite set the Provider's `SpriteSet.get(random)`
/// picks one of, the frame static for the particle's own life).
pub const SPLASH_FRAME_COUNT: usize = 4;

/// The sweep arc's 8 frames (the `sweep_0..7` textures the jar speaks
/// — the sprite set the json's ASCENDING list orders, the list index
/// the frame law reads directly).
pub const SWEEP_FRAME_COUNT: usize = 8;

/// The sweep arc's own frame law (javap `Particle.setSpriteFromAge`
/// -> `MutableSpriteSet.get(age, lifetime)` verbatim:
/// `sprites.get(age * (size - 1) / lifetime)` — 8 frames, the 4-tick
/// life walking frames 0, 1, 3, 5 over ages 0..3; the min clamp the
/// list.get's own bound speaks — the vanilla walk never fires it
/// (the age walk removes the particle at the lifetime first), the
/// Rust shape keeps it honest).
pub fn sweep_frame(age: u32, lifetime: u32) -> usize {
    ((age as usize * (SWEEP_FRAME_COUNT - 1)) / lifetime.max(1) as usize)
        .min(SWEEP_FRAME_COUNT - 1)
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
    /// The previous tick's position (javap `Particle.tick`'s own `xo,
    /// yo, zo` — the pre-move pose the render's lerp walks FROM).
    /// `None` = the just-spawned (no tick has walked it yet): the
    /// render shows the spawn point whole, the vanilla new-particle
    /// shape. The tick heads save it before the physics moves.
    pub prev: Option<(f64, f64, f64)>,
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
    /// The tick's own head (javap `Particle.tick`'s first fold — `xo
    /// = x` rides BEFORE the physics): the current pose becomes the
    /// previous, the render's lerp then walks the frame between the
    /// two. Every per-kind tick calls this first.
    pub fn tick_head(&mut self) {
        self.prev = Some((self.x, self.y, self.z));
    }

    /// THE RENDER FOLD (javap `Particle.render`'s own lerp — `x =
    /// Mth.lerp(partialTicks, xo, x)`): the pose the frame rides —
    /// the previous tick's position walked `partial` of the way to
    /// the current. `None` prev (the just-spawned) renders the spawn
    /// point whole. The partial >= 1.0 arm speaks the CURRENT tick
    /// position bit for bit (the headless lanes' own default — the
    /// goldens' byte-identical contract).
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

    pub fn tick(&mut self) -> bool {
        self.tick_head();
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
            ParticleKind::Crit | ParticleKind::Damage => {
                // javap CritParticle.tick: super.tick() then the two
                // color folds — the star cools as it flies (the
                // indicator IS the crit's own class, the provider's
                // ctor chain keeps the laws)
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
            // the heart's own bloom (javap HeartParticle.getQuadSize —
            // the SAME 32x clamp law the crit speaks: the sprite grows
            // from a point inside the first lifetime/32 ticks, the
            // full size the rest of the life keeps)
            ParticleKind::Heart => base * crit_grow(self.age, self.lifetime, 0.0),
            // the drip's own quadratic thin (the getQuadSize override's
            // (1 - t*t) fold — the partial rides 0.0 the same way the
            // crit's bloom does)
            ParticleKind::Lava => base * lava_shrink(self.age, self.lifetime, 0.0).max(0.0),
            // the smoke's own fast bloom (the BaseAshSmokeParticle's
            // getQuadSize override — the SAME 32x law the crit speaks)
            ParticleKind::Smoke => base * crit_grow(self.age, self.lifetime, 0.0),
            // the flame's own half-thin (the FlameParticle.getQuadSize
            // override — the (1 - t*t*0.5) fold the death walks to the
            // HALF quad, the lava's own to-none never keeps it)
            ParticleKind::Flame => base * flame_shrink(self.age, self.lifetime, 0.0),
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
        self.tick_head();
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

    /// The `BubbleColumnUpParticle.tick` verbatim — the base law rides
    /// FIRST (the age walk, the gravity lane the −0.125 field turns
    /// into the rise — `yd -= 0.04 * g` ADDS 0.005 a tick, the move,
    /// the 0.85 friction), then the fluid gate the override's own tail
    /// speaks: the containing cell leaving WATER removes (the caller
    /// answers with the world's own read — the same wet lane the swim
    /// bubble's gate walks). The already-removed walk never re-arms.
    pub fn tick_bubble_column_up(&mut self, in_water: bool) -> bool {
        let alive = self.tick();
        alive && in_water
    }

    /// The `AttackSweepParticle.tick` verbatim — the arc rides NONE of
    /// the base law (the override's own block: the age walk, the
    /// removal at the lifetime, the sprite walk — NO motion, NO
    /// gravity, NO friction; the arc HANGS in the air the whole 4-tick
    /// life, the frame the render's `sweep_frame` computation picks).
    pub fn tick_sweep(&mut self) -> bool {
        self.tick_head();
        self.age += 1;
        if self.age >= self.lifetime {
            return false;
        }
        true
    }

    /// The `WaterDropParticle.tick` verbatim — the countdown lifetime
    /// (`--lifetime; if (lifetime > 0) ... else remove` — the u32 walk
    /// stops at zero, the removal tick keeps no motion), the drop's own
    /// gravity fold (`yd -= gravity` — the plain field, no 0.04
    /// factor), the move, the 0.98 friction AFTER it. The onGround tail
    /// sleeps (the collision lane's honest absence — no particle
    /// ground exists for the pieces, none for the drop).
    pub fn tick_splash(&mut self) -> bool {
        self.tick_head();
        if self.lifetime == 0 {
            return false;
        }
        self.lifetime -= 1;
        if self.lifetime == 0 {
            return false;
        }
        self.vy -= self.gravity;
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
        self.vx *= 0.98;
        self.vy *= 0.98;
        self.vz *= 0.98;
        true
    }

    /// The `LavaParticle.tick` verbatim (the base law first — the
    /// gravity 0.75's pull, the move, the 0.999 friction — then the
    /// smoke child's own gate: `nextFloat() > age / lifetime` spawns
    /// the SMOKE child AT the blob's own position riding the blob's
    /// OWN velocity). The gate's draw rides the PARENT's own random —
    /// the Particle's `this.random = level.random` field the ctor
    /// speaks — so the caller's level stream takes it (the draw order
    /// the bytecode lists: the roll AFTER the motion, one per living
    /// tick). The child's six floats ride the caller's out-vec (the
    /// spawn lands AFTER the tick loop the vanilla queue order
    /// speaks). The bytecode's `if (!removed)` gate sits BEFORE the
    /// draw: a blob the base tick removed never speaks the child —
    /// and never draws the roll.
    pub fn tick_lava(
        &mut self,
        rng: &mut crate::drops::Lcg,
        smoke_out: &mut Vec<(f64, f64, f64, f64, f64, f64)>,
    ) -> bool {
        self.tick_head();
        // the base Particle.tick's own walk (the age, the gravity
        // lane, the move, the friction — the same law the match arm's
        // base tick speaks), inlined: the removal speaks before the
        // draw
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
        // the child's own gate: `nextFloat() <= t` stays silent, the
        // spawn fires only when the roll rides OVER the age's own
        // fraction (the fcmpl/ifle the javap speaks — the smoke comes
        // likelier as the blob ages)
        let t = self.age as f32 / self.lifetime.max(1) as f32;
        if rng.next_f32() > t {
            smoke_out.push((self.x, self.y, self.z, self.vx, self.vy, self.vz));
        }
        true
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
        prev: None,
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
        prev: None,
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
        prev: None,
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

/// The heart particle's ctor (javap `HeartParticle.<init>` verbatim).
/// The super call passes the zero velocity (`0.0, 0.0, 0.0` the dump's
/// own args speak), so the ctor's `xd *= 0.01F` walk zeroes the
/// super's fields and the `+=` folds carry the args VERBATIM — the
/// effective velocity IS the caller's gaussian trio. The overrides:
/// `friction = 0.86F` (the base 0.98 replaced), `quadSize *= 1.5F`
/// (the base 0.1..0.2 walks 0.15..0.3), `lifetime = 16` (the 26.2's
/// flat constant — the draw-order discipline keeps the BASE ctor's
/// lifetime float the chain consumed first, the override discards
/// it), `hasPhysics = false` (the client's no-collision lane already
/// keeps the shape), the gravity untouched (the base 0.0), the u/v
/// offsets the super's zeros keep. The layer the getLayer override
/// returns is OPAQUE — the render samples the one 8x8 sprite the
/// provider's json lists.
pub fn heart(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    r_base_life: f32,
    r_size: f32,
) -> Particle {
    let _ = r_base_life; // the base ctor's own draw — the flat 16 overrides it
    Particle {
        kind: ParticleKind::Heart,
        x,
        y,
        z,
        prev: None,
        vx,
        vy,
        vz,
        age: 0,
        lifetime: 16,
        gravity: 0.0,
        friction: 0.86,
        quad_size: base_quad_size(r_size) * 1.5,
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
}

/// The damage indicator's own particle (javap
/// `CritParticle$DamageIndicatorProvider.createParticle` verbatim):
/// the CRIT's own ctor chain (the same class, the same draws — the
/// base lifetime's float first, the base size's float second) then
/// the provider's `setLifetime(20)` pin — the indicator always lives
/// 20 ticks, whatever the base lifetime draw rolled.
pub fn damage_indicator(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    r_life: f32,
    r_size: f32,
) -> Particle {
    let mut p = crit(x, y, z, vx, vy, vz, r_life, r_size);
    p.kind = ParticleKind::Damage;
    p.lifetime = 20;
    p
}

/// The sweep arc's own particle (javap `AttackSweepParticle` ctor
/// verbatim): the velocity args are KEPT (the packet walk passes the
/// `(d8, 0.0, d10)` pair the sendParticles spoke — the xSpeed the
/// quadSize fold reads) but the tick override never moves the arc;
/// the lifetime 4 the ctor pins; the grey tint ONE draw
/// `nextFloat() * 0.6 + 0.4` to all three channels; the quadSize the
/// `1.0F - xSpeed * 0.5F` fold — the yaw's x-component the vanilla
/// law scales the arc with (the law's own quirk the bytecode keeps);
/// the white-adjacent defaults the gravity-free static pose keeps.
pub fn sweep(x: f64, y: f64, z: f64, x_speed: f64, z_speed: f64, r_grey: f32) -> Particle {
    let v = r_grey * 0.6 + 0.4;
    Particle {
        kind: ParticleKind::Sweep,
        x,
        y,
        z,
        prev: None,
        vx: x_speed,
        vy: 0.0,
        vz: z_speed,
        age: 0,
        lifetime: 4,
        gravity: 0.0,
        friction: 1.0,
        quad_size: 1.0 - x_speed as f32 * 0.5,
        uo: 0.0,
        vo: 0.0,
        col: [v, v, v],
    }
}

/// THE HIT'S OWN BURST (javap `Player.damageStatsAndHearts` ->
/// `ServerLevel.sendParticles` -> the client's
/// `ClientPacketListener.handleParticleEvent` count>0 walk verbatim):
/// the caller's dealt gates (`> 2.0F`) and counts
/// (`(int)(dealt * 0.5F)`) ride the attack site; the six gaussians
/// per particle speak the packet handler's own order — the position
/// offsets `gauss * 0.1 / gauss * 0.0 / gauss * 0.1` (the y draw
/// consumed, the offset zero) then the velocities
/// `gauss * 0.2` per axis — at the mob's mid height
/// (`getY(0.5)` = y + height * 0.5). The provider's
/// `ySpeed + 1.0` bumps the burst UP (the dconst_1/dadd the
/// stack walk speaks) before the ctor's draws.
pub fn damage_indicators(
    x: f64,
    y: f64,
    z: f64,
    count: usize,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let ox = rng.next_gaussian() * 0.1;
        let oy = rng.next_gaussian() * 0.0;
        let oz = rng.next_gaussian() * 0.1;
        let vx = rng.next_gaussian() * 0.2;
        let vy = rng.next_gaussian() * 0.2;
        let vz = rng.next_gaussian() * 0.2;
        out.push(damage_indicator(
            x + ox,
            y + oy,
            z + oz,
            vx,
            vy + 1.0,
            vz,
            rng.next_f32(),
            rng.next_f32(),
        ));
    }
    out
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
        prev: None,
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

/// The upward column's own bubble (javap `BubbleColumnUpParticle` ctor
/// verbatim — the `bubble_column_up` type the column's animateTick
/// spawns two of). The chain and the draw order the swim bubble's own
/// fold speaks (the SQ 5-arg -> the base 4-arg): the dead base
/// lifetime float, the SingleQuadParticle's 0.1..0.2 quadSize draw,
/// then the bubble's own: the quadSize *= r*0.6+0.2 fold, the three
/// velocity folds (`va * 0.2 + (r * 2 - 1) * 0.02` — the caller's push
/// thinned to a fifth, the ±0.02 jitter added), the lifetime
/// `(int)(40.0 / (r * 0.8 + 0.2))` — the 26.2's own 40.0 the swim
/// bubble's 8 shares the fold's shape with. The −0.125 gravity the
/// base tick's lane turns into the rise, the 0.85 friction the move
/// keeps, the `minecraft:bubble` sprite the json names.
#[allow(clippy::too_many_arguments)]
pub fn bubble_column_up(
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
        kind: ParticleKind::BubbleColumnUp,
        x,
        y,
        z,
        prev: None,
        vx: vx * 0.2 + (r_jx * 2.0 - 1.0) as f64 * 0.02,
        vy: vy * 0.2 + (r_jy * 2.0 - 1.0) as f64 * 0.02,
        vz: vz * 0.2 + (r_jz * 2.0 - 1.0) as f64 * 0.02,
        age: 0,
        lifetime: (40.0f32 / (r_life * 0.8 + 0.2)) as u32,
        gravity: -0.125,
        friction: 0.85,
        quad_size,
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
}

/// The whirlpool's own sinking droplet (javap `WaterCurrentDownParticle`
/// ctor verbatim — the `current_down` type the column's animateTick
/// spawns one of). The chain rides the SQ 5-arg -> the base 4-arg: the
/// dead base lifetime float, the SingleQuadParticle's quadSize draw,
/// then the particle's own — the lifetime `(int)(r * 60.0F) + 30` FIRST
/// (the bytecode's own order), the quadSize *= r*0.6+0.2 fold after.
/// The fixed motion the ctor's own fields speak: xd/zd 0.0, yd −0.05
/// (the current's pull), the 0.002 gravity the WaterDropParticle tick's
/// plain-field fold rides, the 0.98 friction the move keeps, the
/// `minecraft:bubble` sprite the json names.
pub fn current_down(
    x: f64,
    y: f64,
    z: f64,
    r_base_life: f32,
    r_size0: f32,
    r_life: f32,
    r_scale: f32,
) -> Particle {
    let _ = r_base_life; // the dead draw the chain's order keeps
    Particle {
        kind: ParticleKind::CurrentDown,
        x,
        y,
        z,
        prev: None,
        vx: 0.0,
        vy: -0.05,
        vz: 0.0,
        age: 0,
        lifetime: (r_life * 60.0) as u32 + 30,
        gravity: 0.002,
        friction: 0.98,
        quad_size: base_quad_size(r_size0) * (r_scale * 0.6 + 0.2),
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

/// The splash droplet's ctor (javap `SplashParticle` + its
/// `WaterDropParticle` super verbatim). The super ctor the chain calls
/// takes NO speeds — the base `Particle` got `(0, 0, 0)` — so the
/// caller's carried velocity never rides the fields directly: the
/// `xd *= 0.3` and `zd *= 0.3` folds ride zeros, the yd draw speaks
/// `nextFloat() * 0.2F + 0.1F`, the `setSize(0.01F, 0.01F)` folds the
/// quadSize 0.01, the super's gravity 0.06F the SplashParticle's own
/// 0.04F overrides, the lifetime `(int)(8.0 / (r * 0.8 + 0.2))`
/// (8..=40). The SplashParticle's own branch forwards the carried
/// speed ONLY in the drip lane's shape (`yd == 0 && xd == 0 && zd !=
/// 0`): the z speed rides, the yd pins 0.1, the x stays the zero. The
/// sprite frame rides the PROVIDER's draw (`SpriteSet.get(random)` —
/// `sprites.get(random.nextInt(4))`, the frame static for the
/// particle's own life); the level.random the ctor draws and the
/// engine random the frame pick draws are two streams the vanilla
/// keeps apart — the Lcg stand-in merges them, the draw order the
/// bytecode's own: the frame pick (the provider's createParticle
/// evaluates the sprite before the ctor runs), then the ctor's yd
/// float, then the lifetime float.
pub fn splash(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    r_yd: f32,
    r_life: f32,
    r_frame: f32,
) -> Particle {
    let mut vx_out = 0.0f64;
    let mut vy_out = (r_yd * 0.2 + 0.1) as f64;
    let mut vz_out = 0.0f64;
    if vy == 0.0 && vx == 0.0 && vz != 0.0 {
        vx_out = vx;
        vy_out = 0.1;
        vz_out = vz;
    }
    Particle {
        kind: ParticleKind::Splash {
            frame: (r_frame * SPLASH_FRAME_COUNT as f32) as usize % SPLASH_FRAME_COUNT,
        },
        x,
        y,
        z,
        prev: None,
        vx: vx_out,
        vy: vy_out,
        vz: vz_out,
        age: 0,
        lifetime: (8.0f32 / (r_life * 0.8 + 0.2)) as u32,
        gravity: 0.04,
        friction: 0.98,
        quad_size: 0.01,
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
}

/// THE ENTRY SPLASH'S OWN RINGS (javap `Entity.doWaterSplashEffect` —
/// the two loops the sound's own tail speaks, the site the fluid
/// update's entry gate fires): the surface Y `(float) Mth.floor(y) +
/// 1.0F`, the spread walk `j < 1.0F + width * 20.0F` (the 0.6 player:
/// 13 per ring, 26 drops), the offset `(r * 2.0 - 1.0) * width` per
/// axis. Ring one speaks the BUBBLE particles — the carried velocity
/// with the z thinned `- r * 0.2`, the ctor chain's seven floats (the
/// dead lifetime float first, the draw order the drown lane speaks).
/// Ring two the SPLASH ones — the carried velocity verbatim, the
/// frame + yd + lifetime draws the ctor law lists. The rings run to
/// completion one before the other (the bytecode's own first and
/// second walks).
pub fn water_entry_rings(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    width: f64,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let surface = (y.floor() as f32 + 1.0) as f64;
    let mut out = Vec::new();
    // RING ONE — the BUBBLE loop (the bytecode's own first walk)
    let mut j = 0.0f32;
    while j < 1.0f32 + width as f32 * 20.0f32 {
        let dx = (rng.next_f64() * 2.0 - 1.0) * width;
        let dz = (rng.next_f64() * 2.0 - 1.0) * width;
        let vz_thin = vz - rng.next_f64() * 0.2;
        // the ctor chain's seven floats (the base lifetime's dead
        // float first, the quadSize, the scale, the three jitters,
        // the lifetime's own)
        let r_base_life = rng.next_f32();
        let r_size0 = rng.next_f32();
        let r_scale = rng.next_f32();
        let r_jx = rng.next_f32();
        let r_jy = rng.next_f32();
        let r_jz = rng.next_f32();
        let r_life = rng.next_f32();
        out.push(bubble(
            x + dx,
            surface,
            z + dz,
            vx,
            vy,
            vz_thin,
            r_base_life,
            r_size0,
            r_scale,
            r_jx,
            r_jy,
            r_jz,
            r_life,
        ));
        j += 1.0;
    }
    // RING TWO — the SPLASH loop (the bytecode's own second walk)
    let mut j = 0.0f32;
    while j < 1.0f32 + width as f32 * 20.0f32 {
        let dx = (rng.next_f64() * 2.0 - 1.0) * width;
        let dz = (rng.next_f64() * 2.0 - 1.0) * width;
        // the engine's frame pick, then the ctor's yd + lifetime floats
        let r_frame = rng.next_f32();
        let r_yd = rng.next_f32();
        let r_life = rng.next_f32();
        out.push(splash(
            x + dx,
            surface,
            z + dz,
            vx,
            vy,
            vz,
            r_yd,
            r_life,
            r_frame,
        ));
        j += 1.0;
    }
    out
}

/// The lava pool's own drip (javap `LavaParticle` ctor verbatim): the
/// zero velocity base the super call speaks, the gravity 0.75, the
/// friction 0.999, the pop `yd = nextFloat() * 0.4F + 0.05F`, the
/// size `quadSize *= nextFloat() * 2.0F + 0.2F`, the lifetime
/// `(int)(16.0D / (nextFloat() * 0.8D + 0.2D))`. Draw order (the ctor
/// chain's own, all on the LEVEL random the Particle's field speaks):
/// the base ctor's size float FIRST, then the yd draw, the size-mul
/// draw, the lifetime draw. The position/velocity the addParticle
/// call speaks: `(x + nextDouble(), y + 1.0, z + nextDouble(),
/// 0.0, 0.0, 0.0)` — the caller's floats land verbatim.
pub fn lava_drip(x: f64, y: f64, z: f64, r_base: f32, r_yd: f32, r_size: f32, r_life: f32) -> Particle {
    Particle {
        kind: ParticleKind::Lava,
        x,
        y,
        z,
        prev: None,
        vx: 0.0,
        vy: (r_yd * 0.4 + 0.05) as f64,
        vz: 0.0,
        age: 0,
        lifetime: lava_lifetime(r_life),
        gravity: 0.75,
        friction: 0.999,
        quad_size: base_quad_size(r_base) * (r_size * 2.0 + 0.2),
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
}

/// The drip's own smoke child (javap `SmokeParticle` + `BaseAshSmokeParticle`
/// ctor verbatim): the friction 0.96, the gravity −0.1 (the smoke RISES —
/// the negative the base's gravity field rides), the `speedUpWhenYMotionIsBlocked`
/// the base speaks (the collision lane sleeps here — the fold's own
/// absence), the velocity `xd(0) * 0.1 + d = d` — the carried velocity
/// verbatim, the grey tint `r=g=b = nextFloat() * 0.3F`, the size
/// `quadSize *= 0.75F * 1.0F`, the lifetime `(int)(8 / (r * 0.8 + 0.2) * 1.0)`
/// with the `max(1)` floor. Draw order (the ctor chain's own, all on
/// the LEVEL random): the base ctor's size float FIRST, then the tint
/// draw, the lifetime draw. The frames ride `setSpriteFromAge` — the
/// render index the smoke_sprite_file_index law speaks.
pub fn smoke_puff(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64, r_base: f32, r_tint: f32, r_life: f32) -> Particle {
    let grey = r_tint * 0.3;
    Particle {
        kind: ParticleKind::Smoke,
        x,
        y,
        z,
        prev: None,
        vx,
        vy,
        vz,
        age: 0,
        lifetime: smoke_lifetime(r_life),
        gravity: -0.1,
        friction: 0.96,
        quad_size: base_quad_size(r_base) * 0.75,
        uo: 0.0,
        vo: 0.0,
        col: [grey, grey, grey],
    }
}

/// The SHOOT-SMOKE particle's own spawn (javap
/// `LevelEventHandler.shootParticles` case 2000's `addParticle` walk
/// over the 26.2 jar — the `ParticleTypes.SMOKE` word the
/// `SmokeParticle` provider answers with the `BaseAshSmokeParticle`
/// ctor the full chain wears): the position the walk's own fold
/// places (the caller's — the `dx*0.6 + 0.5 + dx*0.01` anchor plus
/// the crosswise `(nextDouble - 0.5) * perpendicular * 0.5` jitter
/// the facing decodes), the velocity `dir * d10 + gaussian * 0.01`
/// the walk folds (the caller's), then the FULL ctor chain the
/// provider's `1.0F` quadSizeMultiplier speaks — the dead base
/// lifetime draw, the three spread draws, the d0 pair, the size
/// draw, the tint draw, the own lifetime draw — the `xd = xd * 0.1 +
/// xSpeed` fold the three `0.1F` xDist args and the speed args the
/// BaseAshSmokeParticle body's own dmul/dadd pairs speak (the torch's
/// zero args the same fold rides — the walk's own speed the add
/// carries), the grey tint the `r * 0.3` draw, the `0.75 * 1.0` size
/// fold, the 8..40 lifetime the smoke_puff helper speaks, the −0.1
/// gravity the BaseAshSmokeParticle passes, the 0.96 friction.
pub fn shoot_smoke_puff(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the dead base lifetime (the draw the stream keeps, the value
    // the ash ctor's own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    let (sx, sy, sz) = ctor_spread_and_normalize(rng);
    let r_base = rng.next_f32();
    let r_tint = rng.next_f32();
    let r_life = rng.next_f32();
    let scale = f64::from(0.1f32);
    smoke_puff(
        x,
        y,
        z,
        sx * scale + vx,
        sy * scale + vy,
        sz * scale + vz,
        r_base,
        r_tint,
        r_life,
    )
}

/// The SHOOT-WHITE-SMOKE particle's own spawn (javap
/// `LevelEventHandler.shootParticles` case 2010's `addParticle` walk
/// over the 26.2 jar — the `ParticleTypes.WHITE_SMOKE` word the
/// `WhiteSmokeParticle` provider answers with the SAME
/// `BaseAshSmokeParticle` ctor chain the smoke kind wears: the 26.2
/// `SmokeParticle`/`WhiteSmokeParticle` ctor bytecodes the identical
/// arg trios pin — the spreads `0.1F`³, the tint-scale arg `0.3F`
/// dead, the lifetime base 8, the gravity `-0.1F`, the `iconst_1`
/// hasPhysics): the FULL draw chain the smoke lane's own helper
/// speaks (the dead base lifetime draw, the three spread draws, the
/// size draw, THE TINT DRAW the stream still consumes, the own
/// lifetime draw) — the ONE delta the ctor's tail speaks: the base's
/// `nextFloat() * 0.3` grey the r/g/b putfield trio OVERRIDES with
/// the constants the cp #12/#17/#21 decode (r 0.7294118, g
/// 0.69411766, b 0.7607843 — the lavender-white the crafter's eject
/// nights wear). The kind the smoke lane's own render arm carries —
/// the generic frames the same law folds, the tint the quad's own
/// `col` the render multiplies.
pub fn shoot_white_smoke_puff(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the dead base lifetime (the draw the stream keeps, the value
    // the ash ctor's own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    let (sx, sy, sz) = ctor_spread_and_normalize(rng);
    let r_base = rng.next_f32();
    // the base's own tint draw (the grey the smoke lane renders — the
    // WHITE kind's r/g/b putfield trio the value overrides, the draw
    // the stream still spends)
    let _dead_tint = rng.next_f32();
    let r_life = rng.next_f32();
    let scale = f64::from(0.1f32);
    Particle {
        kind: ParticleKind::Smoke,
        x,
        y,
        z,
        prev: None,
        vx: sx * scale + vx,
        vy: sy * scale + vy,
        vz: sz * scale + vz,
        age: 0,
        lifetime: smoke_lifetime(r_life),
        gravity: -0.1,
        friction: 0.96,
        quad_size: base_quad_size(r_base) * 0.75,
        uo: 0.0,
        vo: 0.0,
        col: [0.7294118, 0.69411766, 0.7607843],
    }
}

/// The spell ring's 8 frames per set (the `effect_0..7` and
/// `spell_0..7` textures the two jsons speak).
pub const SPELL_FRAME_COUNT: usize = 8;

/// The spell puff's own lifetime (javap `SpellParticle.<init>`'s own
/// write verbatim: `(int)(8.0 / (nextFloat() * 0.8F + 0.2F))` — the
/// [8, 40] band the draw gives, the f64 domain the d-arith keeps: the
/// edge r=0 answers 39.99999940 → 39, r→1 answers 7.99999988 → 7,
/// the grid never draws either edge's byte-exact neighbor).
pub fn spell_lifetime(r: f32) -> u32 {
    (f64::from(8.0f32) / f64::from(r * 0.8f32 + 0.2f32)) as u32
}

/// The spell puff's birth velocity chain (javap the ctor chain
/// verbatim): the super call's own args the STATIC random feeds —
/// `0.5 - RANDOM.nextDouble()` the x/z inputs (the two draws the
/// `SpellParticle.RANDOM` static stream spends, NOT the level's), the
/// yd arg the caller's own walk passes — then the base ctor's spread +
/// normalization the helper folds (the five draws the LEVEL stream
/// keeps). Returns the pre-`yd *= 0.2` velocity triple.
fn spell_birth_velocity(
    jitter_x: f64,
    jitter_y: f64,
    jitter_z: f64,
    rng: &mut crate::drops::Lcg,
) -> (f64, f64, f64) {
    ctor_spread_and_normalize_after(jitter_x, jitter_y, jitter_z, rng)
}

/// The base ctor's spread + normalization fold over EXPLICIT args (the
/// same five draws and the same f64 math the
/// `ctor_spread_and_normalize` helper speaks — the args the spell's
/// own super call carries instead of the zeros the raw helper feeds).
fn ctor_spread_and_normalize_after(
    xd_arg: f64,
    yd_arg: f64,
    zd_arg: f64,
    rng: &mut crate::drops::Lcg,
) -> (f64, f64, f64) {
    let sx = xd_arg + f64::from((rng.next_f32() * 2.0 - 1.0) * 0.4);
    let sy = yd_arg + f64::from((rng.next_f32() * 2.0 - 1.0) * 0.4);
    let sz = zd_arg + f64::from((rng.next_f32() * 2.0 - 1.0) * 0.4);
    let d0 = (rng.next_f32() + rng.next_f32() + 1.0) * 0.15;
    let d1 = (sx * sx + sy * sy + sz * sz).sqrt();
    let vx = sx / d1 * d0 as f64 * f64::from(0.4f32);
    let vy = sy / d1 * d0 as f64 * f64::from(0.4f32) + f64::from(0.1f32);
    let vz = sz / d1 * d0 as f64 * f64::from(0.4f32);
    (vx, vy, vz)
}

/// The BREAKING-POTION shard's own spawn (javap
/// `BreakingItemParticle` over the 26.2 jar — the
/// `ItemParticleOption(ParticleTypes.ITEM, Items.SPLASH_POTION)` the
/// case-2002 arm's `bipush 8` loop speaks, the provider's
/// `getSprite` walk the `pickParticleMaterial` fold answers): the
/// nine draws the ctor chain consumes on the LEVEL stream in the
/// vanilla order — the base lifetime (the LIVE one here, the shard
/// ctor never overrides), the three spreads, the d0 pair (the
/// `(0, 0, 0)` velocity args the 3-arg ctor passes the base), the
/// size draw, then the uo/vo pair the 3-arg ctor's own `nextFloat()
/// * 3.0F` adds — then the 7-arg ctor's `xd = xd * 0.1 + xSpeed`
/// fold the wire velocity rides. The gravity 1.0, the `/2` size
/// fold, the base 0.98 friction the shard keeps, the white tint the
/// item sprite's own pixels wear.
///
/// The `layer` arg the caller folds (the `nextInt(activeLayerCount)`
/// draw the `getSprite` walk spends BEFORE the ctor — the splash
/// potion model's two layers): 0 the overlay glass, 1 the liquid.
#[allow(clippy::too_many_arguments)]
pub fn breaking_potion_shard(
    x: f64,
    y: f64,
    z: f64,
    layer: usize,
    wire_vx: f64,
    wire_vy: f64,
    wire_vz: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    let life = base_lifetime(rng.next_f32());
    let (sx, sy, sz) = ctor_spread_and_normalize(rng);
    let r_size = rng.next_f32();
    let uo = rng.next_f32() * 3.0;
    let vo = rng.next_f32() * 3.0;
    let scale = f64::from(0.1f32);
    Particle {
        kind: ParticleKind::PotionItem { layer },
        x,
        y,
        z,
        prev: None,
        vx: sx * scale + wire_vx,
        vy: sy * scale + wire_vy,
        vz: sz * scale + wire_vz,
        age: 0,
        lifetime: life,
        gravity: 1.0,
        friction: 0.98,
        quad_size: base_quad_size(r_size) / 2.0,
        uo,
        vo,
        col: [1.0, 1.0, 1.0],
    }
}

/// The SPELL-RING puff's own spawn (javap `SpellParticle` +
/// `SpellParticle$InstantProvider.createParticle` over the 26.2 jar —
/// the `SpellParticleOption.create(type, r*f, g*f, b*f, (float)d0)`
/// the case-2002 arm's `bipush 100` loop speaks): the caller's own
/// four draws (the `d0` radius, the `d1` angle, the `d3` rise, the
/// `f` tint scale — the handler's loop body the bytecode orders)
/// precede the ctor chain the LEVEL stream keeps — the dead base
/// lifetime, the three spreads, the d0 pair, the size draw — then
/// the OWN lifetime the ctor's write answers (the second live draw).
/// The super call's x/z inputs ride the STATIC `SpellParticle.RANDOM`
/// stream's `0.5 - nextDouble()` pair (the `jitter` arg — the
/// separate stream the static field's own create() seeds, NOT the
/// level's), the yd input the wire's own `d3` carries.
///
/// The setPower fold the provider's tail speaks (`xd *= power; yd =
/// (yd - 0.1) * power + 0.1; zd *= power` — the power the option's
/// own `(float)d0` word carries) rides AFTER the ctor's `yd *= 0.2`:
/// the wire's x/z velocity words (the ring's own `d2`/`d4`) never
/// reach the motion — the position offset alone they place, the
/// zero-speed arm the bytecode keeps (both wire words 0.0 the
/// `xd *= 0.1`/`zd *= 0.1` fold arms).
#[allow(clippy::too_many_arguments)]
pub fn spell_ring_puff(
    x: f64,
    y: f64,
    z: f64,
    instant: bool,
    d0: f64,
    wire_x_speed: f64,
    wire_y_speed: f64,
    wire_z_speed: f64,
    col: [f32; 3],
    rng: &mut crate::drops::Lcg,
    jitter: &mut crate::drops::Lcg,
) -> Particle {
    // the super call's own x/z inputs — the STATIC stream's
    // `0.5 - nextDouble()` pair the ctor's args evaluation draws
    let jx = f64::from(0.5f32) - jitter.next_f64();
    let jz = f64::from(0.5f32) - jitter.next_f64();
    // the dead base lifetime (the draw the stream keeps, the value
    // the spell ctor's own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    let (mut vx, mut vy, mut vz) = spell_birth_velocity(jx, wire_y_speed, jz, rng);
    let r_size = rng.next_f32();
    // the ctor's own write (the second draw the override keeps)
    let lifetime = spell_lifetime(rng.next_f32());
    // the ctor's yd fold
    vy *= f64::from(0.2f32);
    // THE ZERO-SPEED ARM (the bytecode's own dcmp pair — the wire's
    // x/z words both 0.0 the fold arms, the FIELDS it scales)
    if wire_x_speed == 0.0 && wire_z_speed == 0.0 {
        vx *= 0.1;
        vz *= 0.1;
    }
    // THE SETPOWER FOLD (the provider's tail — the power the option's
    // `(float)d0` word carries)
    let power = d0 as f32;
    vx *= f64::from(power);
    vy = (vy - 0.1) * f64::from(power) + 0.1;
    vz *= f64::from(power);
    Particle {
        kind: ParticleKind::Spell { instant },
        x,
        y,
        z,
        prev: None,
        vx,
        vy,
        vz,
        age: 0,
        lifetime,
        gravity: -0.1,
        friction: 0.96,
        quad_size: base_quad_size(r_size) * 0.75,
        uo: 0.0,
        vo: 0.0,
        col,
    }
}

/// The velocity chain the `Particle` 6-arg ctor speaks (javap
/// verbatim over the 26.2 jar) — the fold BOTH the torch pair's
/// members ride before their own scales: the three spread draws
/// `(nextFloat * 2.0F - 1.0F) * 0.4F` (f32), the `d0 = (nextFloat +
/// nextFloat + 1.0F) * 0.15F` pair (f32), then the f64 normalization
/// `v = v / sqrt(xd² + yd² + zd²) * d0 * 0.4` with the `yd += 0.1`
/// bump (the rise the smoke and the flame share). Draw order: the
/// three spreads, then the d0 pair — five draws on the caller's
/// stream. Returns the normalized (vx, vy, vz) pre-scale.
fn ctor_spread_and_normalize(rng: &mut crate::drops::Lcg) -> (f64, f64, f64) {
    let sx = (rng.next_f32() * 2.0 - 1.0) * 0.4;
    let sy = (rng.next_f32() * 2.0 - 1.0) * 0.4;
    let sz = (rng.next_f32() * 2.0 - 1.0) * 0.4;
    let d0 = (rng.next_f32() + rng.next_f32() + 1.0) * 0.15;
    let d1 = (sx as f64 * sx as f64 + sy as f64 * sy as f64 + sz as f64 * sz as f64).sqrt();
    let vx = sx as f64 / d1 * d0 as f64 * f64::from(0.4f32);
    let vy = sy as f64 / d1 * d0 as f64 * f64::from(0.4f32) + f64::from(0.1f32);
    let vz = sz as f64 / d1 * d0 as f64 * f64::from(0.4f32);
    (vx, vy, vz)
}

/// The torch's ambient SMOKE (javap `TorchBlock.animateTick`'s first
/// addParticle verbatim: `ParticleTypes.SMOKE` at the (0.5, 0.7,
/// 0.5) constant with the (0, 0, 0) velocity args) — the FULL ctor
/// chain the call wears, the nine draws the stream consumes in the
/// vanilla order: the dead base lifetime (the 3-arg ctor's
/// `(int)(4.0 / (r * 0.9 + 0.1))` the BaseAshSmokeParticle ctor's own
/// lifetime write overrides), the three spreads, the d0 pair, the
/// size draw, the tint draw, the own lifetime draw. The velocity the
/// chain gives rides the *0.1 scale the SmokeParticle ctor's xdScale
/// args speak (f2d the f64 fold keeps) — the +0.1 yd bump the
/// normalization added scales to the +0.01 rise, the gravity −0.1
/// the BaseAshSmokeParticle passes pulls further up (`yd -= 0.04 *
/// (-0.1)`), the 0.96 friction the ctor sets. The grey tint the
/// r*0.3 draw carries, the 0.75 size fold, the 8..40 lifetime the
/// smoke_puff helper speaks.
pub fn torch_smoke(x: f64, y: f64, z: f64, rng: &mut crate::drops::Lcg) -> Particle {
    // the dead base lifetime (the draw the stream keeps, the value
    // the ash ctor's own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    let (vx, vy, vz) = ctor_spread_and_normalize(rng);
    let scale = f64::from(0.1f32);
    smoke_puff(
        x,
        y,
        z,
        vx * scale,
        vy * scale,
        vz * scale,
        rng.next_f32(),
        rng.next_f32(),
        rng.next_f32(),
    )
}

/// The torch's ambient FLAME (javap `TorchBlock.animateTick`'s second
/// addParticle verbatim: `this.flameParticle` — the `flame` type the
/// registration passes — at the same (0.5, 0.7, 0.5) constant, the
/// same zero velocity) — the FULL ctor chain the call wears, the
/// fifteen draws the stream consumes in the vanilla order: the dead
/// sprite pick (the `SpriteSet.get(random)` the provider's arg speaks
/// — nextInt(1) the one-texture list answers, always 0), the dead
/// base lifetime (the RisingParticle ctor's own lifetime write
/// overrides), the three spreads, the d0 pair, the size draw, the
/// six jitter draws (`x += (nextFloat - nextFloat) * 0.05F` per axis,
/// f32), the own lifetime draw (`(int)(8.0 / (r * 0.8 + 0.2)) + 4`).
/// The velocity the chain gives rides the *0.01 fold the
/// RisingParticle ctor speaks (`xd * 0.01D + xdArg`) — the flame's
/// own drift, the +0.01 yd rise the bump scales to, the 0.96
/// friction the ctor sets, the no gravity the RisingParticle never
/// arms (the rise the fold alone gives), the white tint the ctor
/// defaults speak, the OPAQUE layer the render lane keeps.
pub fn torch_flame(x: f64, y: f64, z: f64, rng: &mut crate::drops::Lcg) -> Particle {
    // the dead sprite pick (nextInt(1) the one-element list answers)
    let _dead_sprite = rng.next_int(1);
    // the dead base lifetime (the draw the stream keeps, the value
    // the RisingParticle ctor's own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    let (vx, vy, vz) = ctor_spread_and_normalize(rng);
    let quad = base_quad_size(rng.next_f32());
    let fold = f64::from(0.01f32);
    let jx = ((rng.next_f32() - rng.next_f32()) * 0.05f32) as f64;
    let jy = ((rng.next_f32() - rng.next_f32()) * 0.05f32) as f64;
    let jz = ((rng.next_f32() - rng.next_f32()) * 0.05f32) as f64;
    Particle {
        kind: ParticleKind::Flame,
        x: x + jx,
        y: y + jy,
        z: z + jz,
        prev: None,
        vx: vx * fold,
        vy: vy * fold,
        vz: vz * fold,
        age: 0,
        lifetime: flame_lifetime(rng.next_f32()),
        gravity: 0.0,
        friction: 0.96,
        quad_size: quad,
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
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
        // the box's own reads: the ±half-width the min/max x pair
        // speaks, the HEIGHT the max-y minus the min-y speaks (the
        // getRandomY walk the vanilla law keeps — the max-z the
        // bbox.5 slot held was the tuple's own misread the love walk
        // exposed)
        let (bw, bh) = ((bbox.3 - bbox.0) * 0.5, bbox.4 - bbox.1);
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

/// THE LOVE WALK the two heart emitters share (javap `Animal.aiStep`'s
/// own loop body and `Animal.handleEntityEvent`'s event-18 arm — the
/// identical walk both speak). The draw order the bytecode keeps:
/// THREE `nextGaussian() * 0.02` draws first (the velocity), then the
/// position's `getRandomX(1.0)`, `getRandomY()`, `getRandomZ(1.0)`
/// (one `nextDouble` each — the `*2.0 - 1.0` spread the ±half-width
/// law speaks, the height fraction the `getY() + nextDouble() *
/// getBbHeight()` walk speaks, the `+ 0.5` the dump's dadd carries),
/// then the ctor chain's own draws (the base lifetime float the flat
/// 16 overrides, the base size float the 1.5 fold carries).
fn heart_walk(bbox: (f64, f64, f64, f64, f64, f64), rng: &mut crate::drops::Lcg) -> Particle {
    let d0 = rng.next_gaussian() * 0.02;
    let d1 = rng.next_gaussian() * 0.02;
    let d2 = rng.next_gaussian() * 0.02;
    let rx = rng.next_f64() * 2.0 - 1.0;
    let ry = rng.next_f64();
    let rz = rng.next_f64() * 2.0 - 1.0;
    // the box's own reads: the ±half-width the min/max x pair speaks,
    // the HEIGHT the max-y minus the min-y speaks (the tuple the
    // (minx, miny, minz, maxx, maxy, maxz) order keeps — the same
    // read the CritEmitter's z-center fold speaks)
    let (bw, bh) = ((bbox.3 - bbox.0) * 0.5, bbox.4 - bbox.1);
    let px = bbox.0 + bw + bw * rx;
    let py = bbox.1 + bh * ry + 0.5;
    let pz = bbox.2 + bw + bw * rz;
    let r_base_life = rng.next_f32();
    let r_size = rng.next_f32();
    heart(px, py, pz, d0, d1, d2, r_base_life, r_size)
}

/// The event-18 burst (javap `Animal.handleEntityEvent` verbatim: the
/// `(byte)18` arm the `setInLove` broadcast speaks — the SEVEN hearts
/// the loop's own count keeps, each riding the shared love walk above,
/// the position the body's own registration box spreads).
pub fn heart_burst(
    bbox: (f64, f64, f64, f64, f64, f64),
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::with_capacity(7);
    for _ in 0..7 {
        out.push(heart_walk(bbox, rng));
    }
    out
}

/// THE TAMING FAILURE'S OWN SMOKE (javap `TamableAnimal.
/// spawnTamingParticles` the 26.2 bytecode keeps — the SMOKE arm the
/// failure broadcast (byte 6) rides, the SAME geometry the HEART arm
/// wears: SEVEN the loop's own count keeps, each at
/// `getRandomX(1.0), getRandomY() + 0.5, getRandomZ(1.0)` the box
/// reads, the `nextGaussian * 0.02` velocity pair the three draws
/// speak, the ParticleTypes.SMOKE the small-smoke ctor the
/// smoke_puff walk answers — the draw order the ctor chain's own
/// shape keeps).
pub fn taming_smoke_burst(
    bbox: (f64, f64, f64, f64, f64, f64),
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::with_capacity(7);
    for _ in 0..7 {
        let d0 = rng.next_gaussian() * 0.02;
        let d1 = rng.next_gaussian() * 0.02;
        let d2 = rng.next_gaussian() * 0.02;
        let rx = rng.next_f64() * 2.0 - 1.0;
        let ry = rng.next_f64();
        let rz = rng.next_f64() * 2.0 - 1.0;
        let (bw, bh) = ((bbox.3 - bbox.0) * 0.5, bbox.4 - bbox.1);
        let px = bbox.0 + bw + bw * rx;
        let py = bbox.1 + bh * ry + 0.5;
        let pz = bbox.2 + bw + bw * rz;
        let r_base_life = rng.next_f32();
        let r_size = rng.next_f32();
        let r_tint = rng.next_f32();
        out.push(smoke_puff(px, py, pz, d0, d1, d2, r_base_life, r_tint, r_size));
    }
    out
}

/// The aiStep ambient (javap `Animal.aiStep` verbatim: the `isInLove`
/// gate ONE heart per tick — the decrement the same walk arms, the
/// spawn the post-decrement gate `inLove > 0` keeps, so the last
/// in-love tick spawns none).
pub fn heart_ambient(
    bbox: (f64, f64, f64, f64, f64, f64),
    rng: &mut crate::drops::Lcg,
) -> Particle {
    heart_walk(bbox, rng)
}

/// THE FEED SPARKLE (javap `SuspendedTownParticle` + the
/// `$HappyVillagerProvider` verbatim: the position the aiStep's own
/// `getRandomX(1.0) / getRandomY() + 0.5 / getRandomZ(1.0)` walk
/// carried (the caller's word), the lifetime the `20.0 / (nf * 0.8 +
/// 0.2)` fold answers, the size the `0.02 * (nf * 0.6 + 0.5)` draw
/// gives, the zero velocity the ctor's `* 0.02` fold keeps whole,
/// the friction the tick's own `* 0.99` speaks, the white tint the
/// glint texture's own green passes).
pub fn happy_villager_sparkle(x: f64, y: f64, z: f64, rng: &mut crate::drops::Lcg) -> Particle {
    let lifetime = (20.0 / (rng.next_f32() as f64 * 0.8 + 0.2)) as u32;
    let quad_size = 0.02 * (rng.next_f32() * 0.6 + 0.5);
    Particle {
        kind: ParticleKind::HappyVillager,
        x,
        y,
        z,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        prev: None,
        age: 0,
        lifetime,
        gravity: 0.0,
        friction: 0.99,
        quad_size,
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
}

/// THE GROWTH BURST'S OWN WALK (javap `ParticleUtils.spawnParticles`
/// verbatim — the LevelEventHandler's 1505 case the bone-meal item's
/// own use speaks, the level event the client vocabulary now renders):
/// `count` attempts, each ONE particle the three-draw walk arms —
/// `xd = nextGaussian() * 0.02`, `yd = nextGaussian() * 0.02`,
/// `zd = nextGaussian() * 0.02` (the literal 0.02 the ldc2_w speaks),
/// then the anchor's own spread: `x = pos.x + (0.5 - dx) +
/// nextDouble() * dx * 2.0`, `y = pos.y + nextDouble() * dy`,
/// `z = pos.z + (0.5 - dx) + nextDouble() * dx * 2.0` (the `0.5 - dx`
/// half-fold the dsub keeps, the `* 2.0` the second dmul speaks). THE
/// GROUND GATE (the `force` flag false keeps): the containing cell's
/// BELOW block (`BlockPos.containing(x, y, z).below()`) the walk
/// reads — an AIR answer the attempt abandons (the `isAir` branch the
/// ifne jump skips, no particle the skipped draw ever draws). THE
/// VELOCITY'S OWN FOLD: the passed deltas ride the
/// SuspendedTownParticle ctor's `* 0.02` walk (`javap`: the three
/// putfield dmul pairs) — the effective drift the two folds keep
/// (`nextGaussian() * 0.02 * 0.02`). The lifetime/size draws ride the
/// [`happy_villager_sparkle`] law verbatim (the same
/// `$HappyVillagerProvider` sprite the feed sparkle shares).
///
/// The caller's word: `anchor` the `getParticlePos` answer (the grass
/// block's `pos.above()`, the crop's own cell), `count` the data word
/// the event carried (the spreader's `data * 3`, the grower's plain
/// `data`), `dx`/`dy` the spread pair (`3.0, 1.0` the spreader's wide
/// band, `0.5, shape-max-Y` the grower's in-block fold), `force` the
/// gate's flag (the grower's in-block lane `true`, the wide band
/// `false`).
pub fn growth_burst(
    anchor: (i32, i32, i32),
    count: usize,
    dx: f64,
    dy: f64,
    force: bool,
    world: &crate::world::World,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::new();
    for _ in 0..count {
        let vx = rng.next_gaussian() * 0.02;
        let vy = rng.next_gaussian() * 0.02;
        let vz = rng.next_gaussian() * 0.02;
        let half = 0.5 - dx;
        let px = anchor.0 as f64 + half + rng.next_f64() * dx * 2.0;
        let py = anchor.1 as f64 + rng.next_f64() * dy;
        let pz = anchor.2 as f64 + half + rng.next_f64() * dx * 2.0;
        if !force {
            // THE GROUND GATE (the force=false lane's own read): the
            // containing cell's below block — an air answer the
            // attempt abandons (the particle would float)
            let cx = px.floor() as i32;
            let cy = py.floor() as i32;
            let cz = pz.floor() as i32;
            if world.get(cx, cy - 1, cz) == crate::world::AIR {
                continue;
            }
        }
        let mut p = happy_villager_sparkle(px, py, pz, rng);
        // the ctor's own velocity fold (the passed delta × 0.02)
        p.vx = vx * 0.02;
        p.vy = vy * 0.02;
        p.vz = vz * 0.02;
        out.push(p);
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
            // THE HEIGHT BAND (the getRandomY law the fix keeps — the
            // body's own 0..1.95 band the max-y read speaks, walked
            // back the |d1 * 10| <= g*0.2 <= 2.4 the gaussian tail
            // gives): the max-z misread the love walk exposed would
            // have flattened this cloud to the 0..0.3 sliver the
            // bbox's z pair speaks
            assert!(
                p.y >= -2.5 && p.y <= 4.4,
                "the walked-back height band: {}",
                p.y
            );
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
    fn the_column_particles_fold_the_ctor_chains() {
        // THE UP BUBBLE'S OWN FOLD (the swim bubble's chain — the dead
        // base lifetime float, the quadSize 0.1..0.2 draw, the
        // r*0.6+0.2 scale fold, the three velocity folds, the
        // lifetime): the 26.2's own 40.0 the fold divides, the −0.125
        // gravity the base tick's rise rides, the 0.85 friction
        let p = bubble_column_up(
            5.5, 11.0, 7.5, 0.0, 0.04, 0.0, 0.25, 0.5, 0.7, 0.125, 0.25, 0.375, 0.5,
        );
        assert_eq!(p.kind, ParticleKind::BubbleColumnUp);
        assert_eq!(p.lifetime, (40.0f32 / (0.5 * 0.8 + 0.2)) as u32, "the 40.0 fold's word");
        assert_eq!(p.gravity, -0.125);
        assert_eq!(p.friction, 0.85);
        assert!((p.quad_size - base_quad_size(0.5) * (0.7 * 0.6 + 0.2)).abs() < 1e-6);
        assert!(
            (p.vy - (0.04 * 0.2 + (0.25 * 2.0 - 1.0) as f64 * 0.02)).abs() < 1e-12,
            "the push's fifth + the jitter: {}",
            p.vy
        );
        assert_eq!(p.col, [1.0, 1.0, 1.0], "the white the defaults speak");
        // the lifetime band: r=0 -> 40/0.2 = 200, r=1 -> 40/1.0 = 40
        let young = bubble_column_up(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(young.lifetime, 200);
        let old = bubble_column_up(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(old.lifetime, 40);
        // THE WHIRLPOOL DROPLET'S OWN FOLD: the (int)(r*60)+30 lifetime
        // FIRST (the bytecode's own order — the scale fold rides the
        // LAST draw), the fixed −0.05 birth speed the ctor's own
        // fields speak, the 0.002 gravity, the 0.98 friction
        let d = current_down(5.5, 11.8, 7.5, 0.3, 0.5, 0.25, 0.7);
        assert_eq!(d.kind, ParticleKind::CurrentDown);
        assert_eq!(d.lifetime, (0.25f32 * 60.0) as u32 + 30, "the (int)(r*60)+30 fold");
        assert_eq!(d.vx, 0.0);
        assert_eq!(d.vy, -0.05);
        assert_eq!(d.vz, 0.0);
        assert_eq!(d.gravity, 0.002);
        assert_eq!(d.friction, 0.98);
        assert!((d.quad_size - base_quad_size(0.5) * (0.7 * 0.6 + 0.2)).abs() < 1e-6);
        // the lifetime band: r=0 -> 30, r=1 -> 90
        let short = current_down(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(short.lifetime, 30);
        let long = current_down(0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(long.lifetime, 90);
    }

    #[test]
    fn the_column_particles_walk_their_tick_laws() {
        // THE UP BUBBLE'S RISE (the base law the −0.125 field turns):
        // `yd -= 0.04 * g` ADDS 0.005 a tick, the 0.85 friction AFTER
        // the move, then the dry death the wet gate speaks
        let mut p = bubble_column_up(
            5.5, 11.0, 7.5, 0.0, 0.04, 0.0, 0.25, 0.5, 0.7, 0.5, 0.5, 0.5, 0.5,
        );
        let vy0 = p.vy;
        let y0 = p.y;
        assert!(p.tick_bubble_column_up(true), "the wet cell keeps it");
        let expected = (vy0 + 0.005) * 0.85;
        assert!(
            (p.vy - expected).abs() < 1e-12,
            "the rise then the friction: {} vs {}",
            p.vy,
            expected
        );
        assert!(p.y > y0, "the column bubble RISES");
        assert!(!p.tick_bubble_column_up(false), "the dry cell removes");
        // the age walk rides the base law: the age check FIRST (the
        // removal tick speaks no motion — L-1 alive ticks), then the
        // gate never re-arms the dead
        let mut q = bubble_column_up(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0);
        let total = q.lifetime;
        for _ in 0..total - 1 {
            assert!(q.tick_bubble_column_up(true), "alive inside the age walk");
        }
        assert!(!q.tick_bubble_column_up(true), "the lifetime spent");
        assert!(!q.tick_bubble_column_up(true), "the dead never re-arms");
        // THE DROPLET'S OWN SINK (the WaterDropParticle law): the
        // countdown, the plain-field gravity pull (−0.002 a tick), the
        // 0.98 friction AFTER the move, no fluid gate
        let mut d = current_down(5.5, 11.8, 7.5, 0.3, 0.5, 0.5, 0.7);
        let dvy = d.vy;
        let dy0 = d.y;
        assert!(d.tick_splash());
        let dexpected = (dvy - 0.002) * 0.98;
        assert!(
            (d.vy - dexpected).abs() < 1e-12,
            "the sink then the friction: {} vs {}",
            d.vy,
            dexpected
        );
        assert!(d.y < dy0, "the droplet SINKS");
        assert!(d.tick_splash(), "no wet gate — the sink rides its whole life");
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

    #[test]
    fn the_splash_drop_walks_the_water_drop_law() {
        // the ctor law: the zero base speed the super's folds ride,
        // the yd draw band [0.1, 0.3), the lifetime band 8..=40, the
        // size 0.01 the setSize(0.01, 0.01) folds, the gravity the
        // SplashParticle's 0.04 overrides, the white tint
        let p = splash(1.0, 2.0, 3.0, 0.4, -0.2, 0.3, 0.5, 0.5, 0.5);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
        // the carried speeds never ride the fields directly (the
        // super ctor got the zeros — the folds ride them)
        assert_eq!(p.vx, 0.0);
        assert_eq!(p.vz, 0.0);
        assert!(p.vy >= 0.1 && p.vy < 0.3, "the yd draw band");
        assert_eq!(p.quad_size, 0.01);
        assert_eq!(p.gravity, 0.04);
        assert!(p.lifetime >= 8 && p.lifetime <= 40, "the vanilla band");
        assert_eq!(p.col, [1.0, 1.0, 1.0]);
        // the frame pick rides the provider's draw (0..4)
        assert!(
            matches!(p.kind, ParticleKind::Splash { frame } if frame < SPLASH_FRAME_COUNT),
            "the frame band"
        );
        // THE DRIP LANE'S BRANCH (the zero x/y speeds, the live z):
        // the z rides, the yd pins 0.1, the x stays the zero
        let drip = splash(0.0, 5.0, 0.0, 0.0, 0.0, 0.5, 0.9, 0.5, 0.0);
        assert_eq!(drip.vx, 0.0);
        assert_eq!(drip.vy, 0.1);
        assert_eq!(drip.vz, 0.5);
        // the branch never fires when the x speed speaks
        let plain = splash(0.0, 5.0, 0.0, 0.1, 0.0, 0.5, 0.9, 0.5, 0.0);
        assert_eq!(plain.vz, 0.0, "the carried z never rides");
        assert!(plain.vy >= 0.1 && plain.vy < 0.3, "the yd draw rides");
    }

    #[test]
    fn the_splash_tick_walks_the_drop_countdown() {
        let mut p = splash(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        // the countdown's own removal: the drop lives exactly the
        // ctor's lifetime ticks (the removal tick keeps no motion)
        let born = p.lifetime;
        let mut ticks = 0;
        while p.tick_splash() {
            ticks += 1;
        }
        assert_eq!(ticks, born - 1, "the countdown's own walk");
        // the gravity fold speaks the plain field (no 0.04 factor),
        // the friction rides AFTER the move
        let mut q = splash(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0);
        let vy0 = q.vy;
        q.tick_splash();
        let expected = (vy0 - q.gravity) * 0.98;
        assert!((q.vy - expected).abs() < 1e-12, "vy {} vs {}", q.vy, expected);
    }

    #[test]
    fn the_entry_rings_speak_two_walks() {
        let mut r = rng();
        let rings = water_entry_rings(10.0, 20.4, 30.0, 0.4, -0.3, -0.1, 0.6, &mut r);
        // the 0.6 width: 1.0 + 0.6 * 20.0 = 13.0 -> 13 per ring, 26
        assert_eq!(rings.len(), 26);
        // ring one the BUBBLE walk, ring two the SPLASH walk
        for p in &rings[..13] {
            assert_eq!(p.kind, ParticleKind::Bubble);
        }
        for p in &rings[13..] {
            assert!(
                matches!(p.kind, ParticleKind::Splash { .. }),
                "the second walk speaks the splash"
            );
        }
        // the surface Y the Mth.floor law folds: floor(20.4) + 1.0
        for p in &rings {
            assert_eq!(p.y, 21.0, "the surface the entry fires at");
            // the spread rides +-width around the body's own x/z
            assert!(p.x > 10.0 - 0.6 && p.x < 10.0 + 0.6);
            assert!(p.z > 30.0 - 0.6 && p.z < 30.0 + 0.6);
        }
        // the bubble ring's z thinned by the -r * 0.2 draw, then the
        // ctor's own fifth-fold + the +-0.02 jitter the bubble law
        // speaks; the splash ring's speeds never rode (the carried
        // fall speaks non-zero x and y — the drip branch asleep)
        for p in &rings[..13] {
            assert!(
                p.vz > (-0.1 - 0.2) * 0.2 - 0.02 && p.vz < (-0.1 - 0.0) * 0.2 + 0.02,
                "the thin {}",
                p.vz
            );
        }
        for p in &rings[13..] {
            assert_eq!(p.vx, 0.0);
            assert_eq!(p.vz, 0.0);
            assert!(p.vy >= 0.1 && p.vy < 0.3);
        }
        // the seed-replayed fold: the same seed walks the same rings
        let mut a = rng();
        let mut b = rng();
        let ra = water_entry_rings(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6, &mut a);
        let rb = water_entry_rings(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6, &mut b);
        for (pa, pb) in ra.iter().zip(rb.iter()) {
            assert_eq!(pa.x, pb.x);
            assert_eq!(pa.z, pb.z);
            assert_eq!(pa.lifetime, pb.lifetime);
        }
    }

    #[test]
    fn the_lava_drip_ctor_rides_the_javap_bands() {
        // the pop: `nextFloat() * 0.4F + 0.05F` — 0.05..0.45 (the f32
        // widen the ctor's f2d speaks — the tolerance rides it)
        let p = lava_drip(1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0);
        assert!((p.vy - 0.05).abs() < 1e-9, "the pop's floor {}", p.vy);
        assert_eq!(p.vx, 0.0);
        assert_eq!(p.vz, 0.0);
        let top = lava_drip(0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert!((top.vy - 0.45f64).abs() < 1e-6, "the pop's top {}", top.vy);
        // the lifetime: `(int)(16.0 / (r * 0.8 + 0.2))` — 16..80
        assert_eq!(lava_lifetime(0.0), 80);
        assert_eq!(lava_lifetime(1.0), 16);
        // the size: base `0.1 * (r*0.5+0.5) * 2` times the mul
        // `r * 2 + 0.2` — 0.2..2.2
        assert!((p.quad_size - 0.1 * 1.0 * 0.2).abs() < 1e-9);
        // the body fields: the gravity 0.75, the friction 0.999
        assert_eq!(p.gravity, 0.75);
        assert_eq!(p.friction, 0.999);
        assert_eq!(p.lifetime, lava_lifetime(0.0));
    }

    #[test]
    fn the_lava_shrink_law_folds_the_quadratic() {
        // `quadSize * (1 - t*t)` — the full quad at birth, a quarter
        // at the 87% mark, nothing at the death
        assert_eq!(lava_shrink(0, 100, 0.0), 1.0);
        assert!((lava_shrink(50, 100, 0.0) - 0.75).abs() < 1e-6);
        assert!((lava_shrink(100, 100, 0.0)).abs() < 1e-6);
        // the half-span rides it: the blob's own corners thin
        let mut p = lava_drip(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        p.age = p.lifetime / 2;
        let full_half = base_quad_size(0.0) * 0.5;
        assert!(p.half() < full_half, "the thin walks");
    }

    #[test]
    fn the_lava_tick_answers_the_smoke_gate_in_order() {
        // the roll `<= t` stays silent, the roll `> t` spawns the
        // child at the blob's own position riding its own velocity —
        // the draw consumed EVERY living tick, the removed tick
        // consumes none
        let mut p = lava_drip(10.0, 20.0, 30.0, 0.0, 0.0, 0.0, 0.0);
        p.age = p.lifetime - 1; // the next tick removes
        let mut out = Vec::new();
        let mut r = rng();
        assert!(!p.tick_lava(&mut r, &mut out), "the death walk");
        assert!(out.is_empty(), "the removed blob never speaks");
        // a young blob with a HIGH roll (next_f32 after seeding —
        // replay the seeded stream's own draw)
        let mut p = lava_drip(1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let mut r = crate::drops::Lcg::new(7);
        r.next_f32(); // burn one draw so the next is known-ish
        let mut out = Vec::new();
        let kept = p.tick_lava(&mut r, &mut out);
        assert!(kept);
        // the six floats ride verbatim when the gate fired
        if !out.is_empty() {
            assert_eq!(out[0].0, 1.0);
            assert_eq!(out[0].2, 1.0);
        }
    }

    #[test]
    fn the_smoke_puff_ctor_rides_the_base_ash_bands() {
        // the tint: `nextFloat() * 0.3F` — 0..0.3
        let p = smoke_puff(0.0, 0.0, 0.0, 0.1, 0.2, 0.3, 0.0, 0.0, 0.0);
        assert_eq!(p.col, [0.0, 0.0, 0.0]);
        let light = smoke_puff(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(light.col, [0.3, 0.3, 0.3]);
        // the velocity rides the CARRIED verbatim (the `xd(0) * 0.1 + d`
        // fold the base ctor speaks)
        assert_eq!(p.vx, 0.1);
        assert_eq!(p.vy, 0.2);
        assert_eq!(p.vz, 0.3);
        // the rise: the gravity -0.1, the friction 0.96
        assert_eq!(p.gravity, -0.1);
        assert_eq!(p.friction, 0.96);
        // the size: base times the 0.75 fold
        assert!((p.quad_size - base_quad_size(0.0) * 0.75).abs() < 1e-9);
        // the lifetime: `max(1, (int)(8 / (r * 0.8 + 0.2)))` — 8..40
        assert_eq!(smoke_lifetime(0.0), 40);
        assert_eq!(smoke_lifetime(1.0), 8);
        assert_eq!(smoke_lifetime(0.0), 40);
        assert_eq!(p.lifetime, 40);
    }

    #[test]
    fn the_smoke_bloom_rides_the_32x_law() {
        // the BaseAshSmokeParticle's getQuadSize: `clamp(32 * t, 0, 1)`
        // — the full quad inside the first lifetime/32 ticks
        let p = smoke_puff(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(p.half(), 0.0, "the point at birth");
        let mut mid = smoke_puff(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        mid.age = mid.lifetime / 16; // 32 * (1/16) = 2 -> clamped 1.0
        assert_eq!(mid.half(), base_quad_size(0.0) * 0.75 * 0.5);
    }

    #[test]
    fn the_damage_indicator_rides_the_crit_chain_and_the_pin() {
        // the provider's ctor chain: the same crit laws (the fade's
        // base, the zero gravity, the 0.98 friction, the base size)
        // then the setLifetime(20) pin
        let p = damage_indicator(1.0, 2.0, 3.0, 0.1, 1.1, 0.3, 0.0, 0.0);
        assert_eq!(p.kind, ParticleKind::Damage);
        assert_eq!(p.lifetime, 20, "the pin the provider speaks");
        assert_eq!(p.gravity, 0.0);
        assert_eq!(p.friction, 0.98);
        assert_eq!(p.vx, 0.1);
        assert_eq!(p.vy, 1.1, "the +1.0 burst the provider rides");
        assert_eq!(p.vz, 0.3);
        // the fade: the same CritParticle.tick the crit speaks
        let mut p = damage_indicator(0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let g0 = p.col[1];
        p.tick();
        assert!((p.col[1] - g0 * 0.96).abs() < 1e-6, "the fade walks");
    }

    #[test]
    fn the_damage_burst_rides_the_six_gaussian_law() {
        // the packet walk's own order: ox, oy, oz, vx, vy, vz — six
        // gaussians per particle, the offsets 0.1/0.0/0.1, the
        // velocities 0.2, the +1.0 up-burst, the ctor draws last
        let mut r = rng();
        let stars = damage_indicators(10.0, 20.0, 30.0, 3, &mut r);
        assert_eq!(stars.len(), 3);
        for s in &stars {
            assert!(
                (s.x - 10.0).abs() <= 0.6,
                "the x offset's gaussian band: {}",
                s.x - 10.0
            );
            assert_eq!(s.y, 20.0, "the y offset's zero the 0.0 dist speaks");
            assert!(
                (s.z - 30.0).abs() <= 0.6,
                "the z offset's gaussian band: {}",
                s.z - 30.0
            );
            assert!(
                s.vy > 0.5 && s.vy < 1.5,
                "the +1.0 burst over the 0.2 gaussian: {}",
                s.vy
            );
            assert!((s.vx).abs() < 1.0 && (s.vz).abs() < 1.0);
            assert_eq!(s.lifetime, 20);
            assert_eq!(s.kind, ParticleKind::Damage);
        }
        // the replay: the same seed walks the same stars
        let mut a = rng();
        let mut b = rng();
        let da = damage_indicators(0.0, 0.0, 0.0, 2, &mut a);
        let db = damage_indicators(0.0, 0.0, 0.0, 2, &mut b);
        for (pa, pb) in da.iter().zip(db.iter()) {
            assert_eq!(pa.x, pb.x);
            assert_eq!(pa.vy, pb.vy);
            assert_eq!(pa.lifetime, pb.lifetime);
        }
    }

    #[test]
    fn the_generic_file_index_folds_the_json_order() {
        // the jsons list generic_7..generic_0 — the list index 0 reads
        // the DENSE puff (the file 7), the index 7 the wisp (the file
        // 0): the pool's smoke pops dense and dissolves
        assert_eq!(generic_file_index(0), 7);
        assert_eq!(generic_file_index(7), 0);
        assert_eq!(generic_file_index(3), 4);
        // the full chain: age 0 -> list 0 -> file 7; the death tick's
        // own age rides the walk's clamp — the poof_frame law
        assert_eq!(smoke_sprite_file_index(0, 40), 7);
        assert_eq!(smoke_sprite_file_index(40, 40), 0);
    }

    #[test]
    fn the_sweep_frame_rides_the_get_law() {
        // the MutableSpriteSet.get law `age * (size - 1) / lifetime`:
        // 8 frames over the 4-tick life walk 0, 1, 3, 5 — the ages
        // 0..3 the arc's own life speaks, the json's ASCENDING order
        // the index reads directly (the poof's descending walk the
        // mirror image)
        assert_eq!(sweep_frame(0, 4), 0);
        assert_eq!(sweep_frame(1, 4), 1);
        assert_eq!(sweep_frame(2, 4), 3);
        assert_eq!(sweep_frame(3, 4), 5);
        // a zero lifetime never divides (the max(1) floor keeps — the
        // min clamp rides the list bound the vanilla walk never fires)
        assert_eq!(sweep_frame(2, 0), 7);
    }

    #[test]
    fn the_sweep_ctor_rides_the_attack_sweep_laws() {
        // the AttackSweepParticle ctor: the lifetime 4 the ctor pins,
        // the grey tint ONE draw `r * 0.6 + 0.4` to all three
        // channels, the quadSize the `1.0 - xSpeed * 0.5` fold (the
        // yaw's x-component the vanilla quirk scales the arc with),
        // the velocity args KEPT (the packet walk's (d8, 0.0, d10)
        // pair) — the zero gravity, the neutral friction (the tick
        // override never moves the arc anyway)
        let p = sweep(10.0, 20.0, 30.0, -0.5, 1.0, 0.5);
        assert_eq!(p.kind, ParticleKind::Sweep);
        assert_eq!(p.lifetime, 4, "the 4-tick life the ctor pins");
        assert_eq!(p.gravity, 0.0);
        assert_eq!(p.friction, 1.0);
        assert_eq!(p.vx, -0.5, "the d8 arg the packet walk passes");
        assert_eq!(p.vy, 0.0, "the ySpeed the 0.0 constant speaks");
        assert_eq!(p.vz, 1.0, "the d10 arg the packet walk passes");
        assert!(
            (p.quad_size - (1.0 - (-0.5) * 0.5)).abs() < 1e-6,
            "the 1.0 - xSpeed/2 fold: {}",
            p.quad_size
        );
        // the grey band: r in [0, 1) -> v in [0.4, 1.0)
        let lo = sweep(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert!((lo.col[0] - 0.4).abs() < 1e-6);
        assert_eq!(lo.col[0], lo.col[1], "the ONE draw all three speak");
        assert_eq!(lo.col[1], lo.col[2]);
        let hi = sweep(0.0, 0.0, 0.0, 0.0, 0.0, 1.0 - 1e-6);
        assert!(hi.col[0] < 1.0 && hi.col[0] > 0.99, "the top band: {}", hi.col[0]);
        // the yaw quirk: the west-facing swing's xSpeed -1.0 widens
        // the arc to 1.5, the south-facing 0.0 keeps 1.0, the
        // east-facing 1.0 thins to 0.5
        assert!((sweep(0.0, 0.0, 0.0, -1.0, 0.0, 0.0).quad_size - 1.5).abs() < 1e-6);
        assert!((sweep(0.0, 0.0, 0.0, 0.0, 1.0, 0.0).quad_size - 1.0).abs() < 1e-6);
        assert!((sweep(0.0, 0.0, 0.0, 1.0, 0.0, 0.0).quad_size - 0.5).abs() < 1e-6);
    }

    #[test]
    fn the_sweep_tick_hangs_the_arc_static() {
        // the AttackSweepParticle.tick override: the age walk + the
        // removal at the lifetime — NO motion, NO gravity, NO
        // friction; the arc hangs in the air the whole 4-tick life
        let mut p = sweep(5.0, 6.0, 7.0, 0.8, 0.6, 0.3);
        assert!(p.tick_sweep(), "tick 1 keeps the arc");
        assert_eq!(p.age, 1);
        assert!(p.tick_sweep(), "tick 2 keeps the arc");
        assert!(p.tick_sweep(), "tick 3 keeps the arc");
        assert!(!p.tick_sweep(), "tick 4 removes the arc");
        // the stillness: the position never moves whatever the
        // velocity args carry
        let mut q = sweep(5.0, 6.0, 7.0, 0.8, 0.6, 0.3);
        q.tick_sweep();
        assert_eq!(q.x, 5.0);
        assert_eq!(q.y, 6.0);
        assert_eq!(q.z, 7.0);
    }

    /// The tick-head save law (javap `Particle.tick`'s own `xo = x`
    /// fold): the pre-move pose lands in `prev` before the physics
    /// walks, every tick re-arms it — the lerp walks FROM the pose
    /// the last tick spoke.
    #[test]
    fn the_tick_head_saves_the_pre_move_pose() {
        let mut p = crit(10.0, 20.0, 30.0, 1.0, 0.0, 0.0, 0.5, 0.5);
        assert!(p.prev.is_none(), "the just-spawned carries no prev");
        assert!(p.tick(), "tick 1 lives");
        let (px, py, pz) = p.prev.expect("the tick saves the pose");
        assert_eq!(px, 10.0);
        assert_eq!(py, 20.0);
        assert_eq!(pz, 30.0);
        // the second tick re-arms: the pose the FIRST tick landed
        let x1 = p.x;
        assert!(p.tick(), "tick 2 lives");
        let (px2, _, _) = p.prev.expect("tick 2 re-arms");
        assert_eq!(px2, x1);
    }

    /// The render fold's own lerp (javap `Particle.render`'s
    /// `Mth.lerp(partialTicks, xo, x)`): the half-way frame rides the
    /// midpoint, the spawn-fresh particle (None prev) renders the
    /// spawn point whole, and the partial >= 1.0 arm speaks the
    /// CURRENT position bit for bit (the headless lanes' byte-identical
    /// contract).
    #[test]
    fn the_lerped_pos_walks_the_prev_to_the_current() {
        let mut p = crit(10.0, 20.0, 30.0, 2.0, 0.0, 0.0, 0.5, 0.5);
        // the fresh spawn: any partial, the spawn point whole
        assert_eq!(p.lerped_pos(0.25), (10.0, 20.0, 30.0));
        assert!(p.tick(), "tick 1 lives");
        // the walk moved x by +2 (friction 0.98 lands 1.96): the
        // midpoint rides half the delta
        let (lx, _, _) = p.lerped_pos(0.5);
        let half = 10.0 + (p.x - 10.0) * 0.5;
        assert!((lx - half).abs() < 1e-12, "midpoint: {lx} vs {half}");
        // the exactness arm: partial 1.0 = the current position, the
        // f64 round-trip bit for bit
        assert_eq!(p.lerped_pos(1.0), (p.x, p.y, p.z));
        assert_eq!(p.lerped_pos(2.0), (p.x, p.y, p.z));
        // partial 0.0 = the previous pose, the walk's own source
        assert_eq!(p.lerped_pos(0.0), (10.0, 20.0, 30.0));
    }

    #[test]
    fn the_heart_ctor_speaks_the_bytecode() {
        // draw order (the ctor chain's own): the base lifetime float
        // the flat 16 overrides, the base size float the 1.5 carries
        let p = heart(1.0, 2.0, 3.0, 0.02, -0.03, 0.04, 0.5, 0.0);
        assert_eq!(p.kind, ParticleKind::Heart);
        assert_eq!(p.lifetime, 16, "the 26.2's flat 16 the ctor speaks");
        assert!((p.friction - 0.86).abs() < 1e-6, "the ctor's own 0.86 fold");
        assert!(p.gravity.abs() < 1e-12, "the base 0.0 the ctor never touches");
        // the super's zeroed fields the += args own — the velocity
        // rides the caller's gaussians verbatim
        assert!((p.vx - 0.02).abs() < 1e-9 && (p.vy + 0.03).abs() < 1e-9 && (p.vz - 0.04).abs() < 1e-9);
        // the size: base_quad_size(0.0) = 0.1, the *1.5 fold walks 0.15
        assert!((p.quad_size - 0.15).abs() < 1e-6);
        // the OPAQUE layer's white tint, the zeros the super's u/v keep
        assert_eq!(p.col, [1.0, 1.0, 1.0]);
        assert!(p.uo.abs() < 1e-6 && p.vo.abs() < 1e-6);
    }

    #[test]
    fn the_heart_burst_walks_the_event18_law() {
        // the SEVEN the loop's own count keeps, the box spread the
        // getRandomX/getRandomY law speaks, the determinism the shared
        // walk's own draw order keeps
        let bbox = (1.0, 2.0, 3.0, 1.9, 3.3, 3.9); // the sheep's 0.9 x 1.3
        let burst = heart_burst(bbox, &mut rng());
        assert_eq!(burst.len(), 7, "the event-18 arm's own count");
        let (bw, _bh) = ((bbox.3 - bbox.0) * 0.5, bbox.4 - bbox.1);
        let (cx, cz) = (bbox.0 + bw, bbox.2 + bw);
        for p in &burst {
            assert_eq!(p.lifetime, 16);
            assert!((p.x - cx).abs() <= bw + 1e-9, "the ±half-width spread: {}", p.x);
            assert!((p.z - cz).abs() <= bw + 1e-9, "the ±half-width spread: {}", p.z);
            assert!(
                p.y >= bbox.1 + 0.5 - 1e-9 && p.y <= bbox.4 + 0.5 + 1e-9,
                "the height fraction + 0.5: {}",
                p.y
            );
            assert!(
                p.vx.abs() < 0.3 && p.vy.abs() < 0.3 && p.vz.abs() < 0.3,
                "the gaussian * 0.02 scale the dump speaks"
            );
        }
        // the same seed the same burst speaks — the draw order the
        // bytecode keeps is the walk's own determinism
        let again = heart_burst(bbox, &mut rng());
        for (a, b) in burst.iter().zip(again.iter()) {
            assert_eq!((a.x, a.y, a.z), (b.x, b.y, b.z));
            assert_eq!((a.vx, a.vy, a.vz), (b.vx, b.vy, b.vz));
        }
    }

    #[test]
    fn the_heart_ambient_and_bloom_ride_their_laws() {
        // the aiStep's own one-per-tick walk
        let bbox = (0.0, 0.0, 0.0, 0.9, 1.3, 0.9);
        let mut p = heart_ambient(bbox, &mut rng());
        assert_eq!(p.kind, ParticleKind::Heart);
        // the bloom: the 32x clamp law the crit shares — the point the
        // spawn starts, the full quad the first tick speaks
        assert!(p.half().abs() < 1e-6, "age 0 the point the bloom starts");
        let full = p.quad_size * 0.5;
        p.age = 1;
        assert!((p.half() - full).abs() < 1e-6, "age 1 the full quad speaks");
        p.age = 15;
        assert!((p.half() - full).abs() < 1e-6, "the rest of the life the size keeps");
        // the base tick's own walk under the 0.86 friction
        p.age = 14;
        assert!(p.tick(), "the 16-tick life walks");
        assert_eq!(p.age, 15);
        assert!(!p.tick(), "the lifetime's removal speaks");
    }

    /// The law's own arena: the floor plane at `y` the gate reads, the
    /// air above it the burst occupies (the generated terrain's own
    /// hills the explicit carve overrides).
    fn growth_arena(y: i32) -> crate::world::World {
        let mut w = crate::world::World::new(3, 7);
        for x in 2..22 {
            for z in 2..22 {
                w.set(x, y, z, crate::world::DIRT);
                w.set(x, y + 1, z, crate::world::AIR);
                w.set(x, y + 2, z, crate::world::AIR);
            }
        }
        w
    }

    #[test]
    fn the_growth_burst_wears_the_spawn_particles_law() {
        // THE IN-BLOCK LANE (the grower's own walk — force=true): the
        // count the caller's word speaks (the data verbatim), EVERY
        // attempt lands (the forced lane never skips), the positions
        // the block's own bounds keep (x/z the nextDouble lane, y the
        // shape max-Y's band), the velocity the two 0.02 folds scale.
        let w = growth_arena(20);
        let burst = growth_burst((12, 21, 12), 15, 0.5, 1.0, true, &w, &mut rng());
        assert_eq!(burst.len(), 15, "the forced lane never skips");
        for p in &burst {
            assert!((12.0..13.0).contains(&p.x), "x inside the block: {}", p.x);
            assert!((21.0..22.0).contains(&p.y), "y inside the block: {}", p.y);
            assert!((12.0..13.0).contains(&p.z), "z inside the block: {}", p.z);
            assert_eq!(p.kind, ParticleKind::HappyVillager, "the glint sprite");
            // the effective drift the two folds keep (the gaussian
            // * 0.02 the spawn walk speaks, the * 0.02 the ctor adds):
            // the 5-sigma tail the bound covers
            assert!(p.vx.abs() < 0.0004 * 5.0 && p.vy.abs() < 0.0004 * 5.0
                && p.vz.abs() < 0.0004 * 5.0, "the drift the folds keep");
            assert!(p.lifetime >= 4 && p.lifetime <= 100, "the 20/(r*0.8+0.2) band");
            assert!(p.friction == 0.99 && p.gravity == 0.0, "the sparkle's own tick law");
        }
        // the determinism the draw order keeps (the same seed the same
        // burst — the gaussians, the doubles, the ctor draws)
        let again = growth_burst((12, 21, 12), 15, 0.5, 1.0, true, &w, &mut rng());
        for (a, b) in burst.iter().zip(again.iter()) {
            assert_eq!((a.x, a.y, a.z), (b.x, b.y, b.z));
            assert_eq!((a.vx, a.vy, a.vz), (b.vx, b.vy, b.vz));
        }
    }

    #[test]
    fn the_growth_spread_band_the_ground_gate_the_skips_keep() {
        // THE WIDE LANE (the spreader's own walk — force=false): the
        // band the half-fold opens — x in [x-2.5, x+3.5), y in
        // [y, y+1) above the anchor — and the FULL floor the gate
        // keeps alive (every attempt lands: the below block the
        // DIRT answer speaks).
        let w = growth_arena(20);
        // the anchor rides the pos.above() arm the caller spoke: y=21
        let wide = growth_burst((12, 21, 12), 45, 3.0, 1.0, false, &w, &mut rng());
        assert_eq!(wide.len(), 45, "the solid floor the gate never skips");
        for p in &wide {
            assert!(
                (9.5..15.5).contains(&p.x),
                "x inside the [x-2.5, x+3.5) band: {}",
                p.x
            );
            assert!((21.0..22.0).contains(&p.y), "y above the anchor: {}", p.y);
            assert!((9.5..15.5).contains(&p.z), "z inside the band: {}", p.z);
        }
        // THE HOLE (the below cell the AIR answer keeps): the attempts
        // over the void the gate abandons — fewer particles than the
        // count, more than none (the rest of the floor stands), and
        // the SAME seed the SAME skips speak (the determinism).
        let mut holed = growth_arena(20);
        for z in 10..15 {
            holed.set(12, 20, z, crate::world::AIR); // the below column gone
        }
        let clipped = growth_burst((12, 21, 12), 45, 3.0, 1.0, false, &holed, &mut rng());
        assert!(clipped.len() < 45, "the hole the skips speak: {}", clipped.len());
        assert!(!clipped.is_empty(), "the standing floor still bursts");
        let clipped_again =
            growth_burst((12, 21, 12), 45, 3.0, 1.0, false, &holed, &mut rng());
        assert_eq!(
            clipped.len(),
            clipped_again.len(),
            "the same seed the same gate answers"
        );
        // the forced lane the hole ignores (the grower's own truth):
        // the count the caller's word speaks even over the void
        let forced = growth_burst((12, 21, 12), 15, 0.5, 1.0, true, &holed, &mut rng());
        assert_eq!(forced.len(), 15, "the forced lane never reads the ground");
    }

    #[test]
    fn the_flame_lifetime_wears_the_rising_law() {
        // (int)(8.0 / (r * 0.8 + 0.2)) + 4 — the f64 edges the fold
        // speaks: r=0 -> 39.99999940… -> 39 + 4, r=1 -> 7.99999988…
        // -> 7 + 4 (the pure fold's own edge), the midpoint 8/0.6 =
        // 13.33 -> 13 + 4
        assert_eq!(flame_lifetime(0.0), 43);
        assert_eq!(flame_lifetime(1.0), 11);
        assert_eq!(flame_lifetime(0.5), 17);
        // every draw the stream speaks lands the grid band (the
        // nextFloat's own [0, 1-2^-24] grid never draws the edges)
        for _ in 0..4096 {
            let life = flame_lifetime(rng().next_f32());
            assert!(
                (12..=43).contains(&life),
                "the rising lifetime's own band: {life}"
            );
        }
        // THE RENDER SHRINK (the getQuadSize override's (1 - t*t*0.5)
        // fold): the birth quad whole, the death quad the HALF keeps
        assert!((flame_shrink(0, 20, 0.0) - 1.0).abs() < 1e-6);
        assert!((flame_shrink(20, 20, 0.0) - 0.5).abs() < 1e-6);
        assert!((flame_shrink(10, 20, 0.0) - 0.875).abs() < 1e-6);
    }

    #[test]
    fn the_torch_smoke_wears_the_ash_laws() {
        // THE BIRTH (the addParticle pos the walk speaks — no jitter
        // the ash ctor never arms): the smoke sits at the constant
        let mut r = rng();
        let p = torch_smoke(10.5, 20.7, 30.5, &mut r);
        assert!(matches!(p.kind, ParticleKind::Smoke));
        assert_eq!(p.x, 10.5, "the smoke's own birth x");
        assert_eq!(p.y, 20.7, "the (y + 0.7) the constant speaks");
        assert_eq!(p.z, 30.5, "the smoke's own birth z");
        // THE LAWS (the BaseAshSmokeParticle chain): the grey tint
        // the r*0.3 draw carries, the 0.75 size fold, the 8..40
        // lifetime, the -0.1 gravity the rise rides, the 0.96
        // friction the ctor sets
        assert_eq!(p.gravity, -0.1);
        assert_eq!(p.friction, 0.96);
        assert!((8..=40).contains(&p.lifetime), "the ash lifetime: {}", p.lifetime);
        assert!(
            (0.075..0.15).contains(&p.quad_size),
            "the 0.75 fold's own band: {}",
            p.quad_size
        );
        assert!(
            p.col.iter().all(|c| (0.0..0.3).contains(c)),
            "the grey tint the r*0.3 keeps: {:?}",
            p.col
        );
        // THE VELOCITY (the normalization fold the *0.1 scale wears):
        // |unit| <= 1 -> |vx| <= 0.45*0.4*0.1 = 0.018, the vy band
        // the +0.1 bump scales to [-0.008, 0.028) — the rise the
        // fold and the gravity both give
        assert!(p.vx.abs() <= 0.018 + 1e-9, "the drift band: {}", p.vx);
        assert!(p.vz.abs() <= 0.018 + 1e-9, "the drift band: {}", p.vz);
        assert!(
            (-0.008..0.028).contains(&p.vy),
            "the smoke's own rise band: {}",
            p.vy
        );
        // THE DETERMINISM (the same seed the same chain answers)
        let p2 = torch_smoke(10.5, 20.7, 30.5, &mut rng());
        assert_eq!(
            (p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
            (p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size),
            "the seeded chain replays"
        );
        // THE STREAM (the nine draws the chain consumes — the dead
        // base lifetime, the three spreads, the d0 pair, the size,
        // the tint, the own lifetime; the tenth draw differs)
        let mut a = rng();
        let _ = torch_smoke(0.0, 0.0, 0.0, &mut a);
        let after_nine = a.next();
        let mut b = rng();
        for _ in 0..9 {
            b.next();
        }
        assert_eq!(after_nine, b.next(), "the smoke chain's own nine draws");
    }

    #[test]
    fn the_torch_flame_wears_the_rising_laws() {
        // THE BIRTH (the addParticle pos + the ±0.05 jitter the
        // RisingParticle ctor speaks — the f32 sub-mul the f2d wears)
        let mut r = rng();
        let p = torch_flame(10.5, 20.7, 30.5, &mut r);
        assert!(matches!(p.kind, ParticleKind::Flame));
        assert!(
            (10.45..10.55).contains(&p.x),
            "the x jitter's own band: {}",
            p.x
        );
        assert!(
            (20.65..20.75).contains(&p.y),
            "the y jitter's own band: {}",
            p.y
        );
        assert!(
            (30.45..30.55).contains(&p.z),
            "the z jitter's own band: {}",
            p.z
        );
        // THE LAWS (the RisingParticle chain): the white tint the
        // ctor defaults speak, the no gravity the rise never needs,
        // the 0.96 friction, the 12..44 lifetime the +4 tail pins,
        // the 0.1..0.2 quad the base ctor's own draw gives (no 0.75
        // fold the flame never wears)
        assert_eq!(p.col, [1.0, 1.0, 1.0]);
        assert_eq!(p.gravity, 0.0, "the flame's own no-gravity law");
        assert_eq!(p.friction, 0.96);
        assert!((12..=43).contains(&p.lifetime), "the rising lifetime: {}", p.lifetime);
        assert!(
            (0.1..0.2).contains(&p.quad_size),
            "the base size's own band: {}",
            p.quad_size
        );
        // THE VELOCITY (the *0.01 fold the RisingParticle speaks):
        // |vx| <= 0.45*0.4*0.01 = 0.0018, the vy band the +0.1 bump
        // scales to [-0.0008, 0.0028) — the slow climb the vanilla
        // flame rides
        assert!(p.vx.abs() <= 0.0018 + 1e-12, "the flame drift: {}", p.vx);
        assert!(p.vz.abs() <= 0.0018 + 1e-12, "the flame drift: {}", p.vz);
        assert!(
            (-0.0008..0.0028).contains(&p.vy),
            "the flame's own climb band: {}",
            p.vy
        );
        // THE DETERMINISM (the same seed the same chain answers)
        let p2 = torch_flame(10.5, 20.7, 30.5, &mut rng());
        assert_eq!(
            (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
            (p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size),
            "the seeded chain replays"
        );
        // THE STREAM (the fifteen draws the chain consumes — the dead
        // sprite pick, the dead base lifetime, the three spreads, the
        // d0 pair, the size, the six jitters, the own lifetime)
        let mut a = rng();
        let _ = torch_flame(0.0, 0.0, 0.0, &mut a);
        let after_fifteen = a.next();
        let mut b = rng();
        for _ in 0..15 {
            b.next();
        }
        assert_eq!(after_fifteen, b.next(), "the flame chain's own fifteen draws");
    }

    #[test]
    fn the_spread_chain_spans_the_normalization_law() {
        // THE NORMALIZATION (the d0*0.4 speed the ctor renormalizes
        // to — d0 in [0.15, 0.45) -> the speed in [0.06, 0.18), the
        // yd bump the +0.1 keeps): the pair's pre-scale magnitudes
        // the bands hold across the stream — vy = unit_y * speed +
        // 0.1 with |unit_y| <= 1 -> vy in (-0.08, 0.28)
        for _ in 0..2048 {
            let (vx, vy, vz) = ctor_spread_and_normalize(&mut rng());
            let speed = (vx * vx + vy * vy + vz * vz).sqrt();
            assert!(
                (0.06..0.18).contains(&speed),
                "the normalized speed's own band: {speed}"
            );
            assert!(
                (-0.08..0.28).contains(&vy),
                "the pre-scale yd's own band: {vy}"
            );
        }
    }
}

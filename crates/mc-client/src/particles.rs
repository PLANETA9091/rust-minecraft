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

/// The campfire smoke's 12 frames (the `big_smoke_0..11` textures the
/// two jsons' shared ASCENDING lists speak — the file index the
/// `SpriteSet.get` pick reads directly).
pub const CAMPFIRE_BIG_SMOKE_FRAME_COUNT: usize = 12;

/// The spark's 8 frames (the `spark_0..7` textures the firework.json
/// list speaks).
pub const SPARK_FRAME_COUNT: usize = 8;

/// The firework spark's own frame walk (the same `MutableSpriteSet.
/// get(age, lifetime)` chain the poof rides — the `setSpriteFromAge`
/// call the ctor AND the tick both speak): `age * (size - 1) /
/// lifetime` — the 8 spark frames the firework.json's own list holds
/// (`spark_7..spark_0` the DESCENDING order speaks).
pub fn spark_frame(age: u32, lifetime: u32) -> usize {
    (age as usize * (SPARK_FRAME_COUNT - 1)) / lifetime.max(1) as usize
}

/// The spark list's own direction (the firework.json's descending
/// order — index 0 reads spark_7, index 7 the spark_0): the file the
/// same `count - 1 - index` fold the generic pool speaks.
pub fn spark_file_index(list_index: usize) -> usize {
    (SPARK_FRAME_COUNT - 1).saturating_sub(list_index)
}

/// The spark's own alpha walk (javap `SimpleAnimatedParticle.tick`
/// pcs 12-47 verbatim): the provider's setAlpha(0.99) the spawn
/// keeps, the `age > lifetime / 2` int-division crossing the fade
/// arms — `1.0 - (age - lifetime / 2) / lifetime` the fsub/fdiv pair
/// speaks. The tint's own dim the raster's honest equivalent keeps
/// (the alpha lane the shriek's precedent folds).
pub fn spark_alpha(age: u32, lifetime: u32) -> f32 {
    let half = lifetime / 2;
    if age <= half {
        0.99
    } else {
        1.0 - (age as f32 - half as f32) / lifetime as f32
    }
}

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
#[derive(Debug, Clone, PartialEq)]
pub enum ParticleKind {
    /// `TerrainParticle` — the block's own particle tile (the
    /// `particle_tile` table's kind, the atlas tile id).
    Terrain { tile: u16 },
    /// `ExplodeParticle` — the death cloud (the generic frames).
    Poof,
    /// `FireworkParticles$SparkParticle` — the FIREWORK particle
    /// type's own spark (the rocket's trail the tick's client walk
    /// spawns every flight tick, the `SparkProvider` the
    /// `createParticle` walk answers): the SimpleAnimatedParticle
    /// chain's own laws — the base ctor's size draw the ×0.75 fold
    /// shrinks, the lifetime the 48 + nextInt(12) re-roll banks, the
    /// friction 0.91 the ctor's #13 carries, the gravity 0.1 the
    /// super's float arg keeps, the alpha 0.99 the provider's
    /// setAlpha speaks, the mid-life fade the age>lifetime/2 walk
    /// serves (the tint's own dim the raster's honest equivalent
    /// keeps), the fullbright look the getLightCoords 0xF000F0
    /// override banks (the particle lane's own no-lightmap fold the
    /// lava's precedent speaks), the spark_7..spark_0 frame walk the
    /// descending json list orders. THE STARTER'S OWN FIELDS (javap
    /// `SparkParticle` the 26.2 class the walk reads — the trail
    /// bit the `setTrail` bank stores, the twinkle bit the
    /// `setTwinkle` stores, the fade target the `setFadeColor`
    /// decode holds): the trail bit gates the tick's own CHILD walk
    /// (the half-life's own connecting spark the (age + lifetime) %
    /// 2 gate spawns), the twinkle bit the CHILDREN alone ride (the
    /// 26.2 class speaks no render override — the recorded note),
    /// the fade target the tick's own color lerp arms (`rCol +=
    /// (fadeR − rCol) × 0.2` — the #75 fold the ctor chain speaks).
    /// The trail lane the ctor's white (1,1,1) the base fields never
    /// tint — the Starter's colored sparks the component decode the
    /// explosion walk banks.
    Spark {
        /// The `setTrail` bit — the tick's own child walk's gate (the
        /// explosion's `hasTrail` flag the Starter's createParticle
        /// passes; the flight trail's ctor never sets it).
        trail: bool,
        /// The `setTwinkle` bit — the CHILDREN ride it (no render
        /// override the 26.2 class bytes speak — the recorded note).
        twinkle: bool,
        /// The fade target (the `setFadeColor` decode — `None` = the
        /// hasFade bit never set, the color lane the base keeps).
        fade: Option<[f32; 3]>,
    },
    /// `FireworkParticles$OverlayParticle` — the FLASH particle the
    /// Starter's cadence spawns per explosion (javap
    /// `FireworkParticles$FlashProvider.createParticle` +
    /// `OverlayParticle.<init>` the 26.2 walk): the lifetime the
    /// ctor's `iconst_4` pins, the quad the `7.1 × sin((age +
    /// partial − 1) × π/4)` bell the getQuadSize override speaks
    /// (the #33/#23/#36 constants the cp decode banks), the color
    /// + alpha the ColorParticleOption's own decode gives (`ARGB.
    /// red/alpha / 255`). THE ALPHA THE QUAD'S OWN VISIBILITY CARRIES
    /// (javap `ARGB.alpha(I)` = `i >>> 24` — the dye colors' 0xRRGGBB
    /// the ZERO byte answers, the setAlpha(0) quad the vanilla render
    /// itself paints nothing for; the custom-ARGB colors the alpha
    /// byte's own answer keep): the render arm the alpha-0 quad
    /// honestly skips, the alpha>0 one the tint × alpha dim draws.
    Flash {
        /// The `setAlpha` store — `(color >>> 24) / 255` the decode
        /// gives (the 0.0 the dye colors keep, the visible quad the
        /// custom-ARGB bytes arm).
        alpha: f32,
    },
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
    /// `DripParticle$DripHangParticle` — the dripstone drip's own hang
    /// drop (the LevelEvent-1504 walk's spawn — the `drip_hang` sprite
    /// the one-texture jsons keep, the 40-tick countdown the
    /// preMoveUpdate wears, the ×0.02 damping the postMoveUpdate
    /// speaks, the raw 0.0012 gravity the ×0.02 fold gives; the lava
    /// bit rides the CoolingDripHangParticle's own fade — the r/g/b
    /// the `40 − lifetime` fold speaks).
    DripstoneHang { lava: bool },
    /// `DripParticle$DripstoneFallAndLandParticle` — the fall the
    /// hang's expiry spawns (the `drip_fall` sprite, the
    /// `64.0 / (r × 0.8 + 0.2)` lifetime the FallAndLand ctor draws,
    /// the raw 0.06 gravity the base ctor keeps, the ×0.98 damping;
    /// the landing the world-read fold answers — the splash the water
    /// keeps, the land splat the lava spawns).
    DripstoneFall { lava: bool },
    /// `DripParticle$DripLandParticle` — the lava fall's own landing
    /// splat (the LANDING_LAVA option the fall's postMoveUpdate
    /// spawns, the `drip_land` sprite, the `16.0 / (r × 0.8 + 0.2)`
    /// lifetime the ctor draws, the cooling-end tint the provider
    /// keeps, the resting pose the collision fold pins).
    DripstoneLand,
    /// `SmokeParticle` — the drip's own child (the SMOKE type the
    /// LavaParticle.tick's draw speaks, the BaseAshSmokeParticle laws:
    /// the rise the −0.1 gravity gives, the grey tint the r*0.3 draw,
    /// the 0.75 size fold, the 8 generic frames the smoke.json's
    /// descending order lists).
    Smoke,
    /// `CampfireSmokeParticle` — the campfire's own smoke (the
    /// CAMPFIRE_COSY_SMOKE / CAMPFIRE_SIGNAL_SMOKE types the dowse's
    /// makeParticles walk speaks, the ctor chain the own laws wears:
    /// the `scale(3.0f)` triple the birth quad carries, the
    /// nextInt(50) + 80/280 lifetime the force flag's own ladder
    /// banks, the 3e-6 gravity the raw subtract rides, the ONE
    /// `big_smoke_N` frame the `SpriteSet.get` pick banks static the
    /// whole life (no setSpriteFromAge the tick override never calls),
    /// the 0.9/0.95 alpha the providers' setAlpha stores, the drift
    /// the tick's own ±nextFloat/5000 rolls speak, the fade the
    /// last-60-ticks −0.015 walk serves, the TRANSLUCENT layer the
    /// getLayer override returns). The frame rides the ASCENDING json
    /// list the file index reads directly.
    CampfireSmoke {
        /// The `SpriteSet.get(random)` pick — the big_smoke_N file
        /// index (0..11, the ascending list the quads sample).
        frame: usize,
        /// The provider's setAlpha store — the 0.9 the Cosy answers,
        /// the 0.95 the Signal's force arm keeps; the tick's fade walk
        /// drains it (the last-60 band the −0.015 fold rides).
        alpha: f32,
    },
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
    /// `SuspendedTownParticle` the egg crack's own white (the
    /// `egg_crack` type the sniffer egg's own placement broadcasts —
    /// javap verbatim: the lifetime the `20.0 / (nextFloat() * 0.8 +
    /// 0.2)` fold answers [20, 100], the quadSize the `setSize(0.02,
    /// 0.02)` x the `* (nextFloat() * 0.6 + 0.5)` draw gives, the
    /// drift the supplier's ±0.5 draws the `* 0.02` ctor fold keeps,
    /// the friction the tick's own `* 0.99` speaks (no gravity the
    /// law never arms), the white tint the provider's `setColor(1, 1,
    /// 1)` override — the SAME glint sprite the happy-villager bank
    /// shares the quad samples whole, the OPAQUE layer the getLayer
    /// override returns).
    EggCrack,
    /// `SnowflakeParticle` the trial spawner's own detection spark
    /// (the `trial_spawner_detect_player` type the
    /// `TrialSpawner.addDetectPlayerParticles` walk spawns — javap
    /// verbatim: the lifetime the `(int)(16.0 / (nextFloat() * 0.8 +
    /// 0.2)) + 2` fold answers [18, 81], the quadSize the
    /// `0.1F * (nextFloat() * nextFloat() * 1.0F + 1.0F)` two-draw
    /// product gives [0.1, 0.2] — the base law's own twin the
    /// DIFFERENT fold arms — the velocity spread the three
    /// `(nf * 2 - 1) * 0.05` draws the ctor speaks, the gravity the
    /// 0.225 field keeps, the friction the 1.0F no-op arms, the
    /// tick's own damping the tail speaks (xd *0.95, yd *0.9,
    /// zd *0.95), the near-white bluish tint the provider's
    /// `setColor(0.923, 0.964, 0.999)` override — the SAME eight
    /// generic frames the poof bank shares the snowflake.json list
    /// samples, the OPAQUE layer the getLayer override returns).
    Snowflake,
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
    /// `FlameParticle` the PLAIN provider arms — the SAME class the
    /// `flame` type wears (the bare RisingParticle ctor the
    /// `getLayer`'s OPAQUE answer speaks, the (1 - t*t*0.5) shrink
    /// the getQuadSize override keeps, the white tint the ctor
    /// defaults speak, the 0.96 friction, the no gravity, the
    /// *0.01 velocity fold) — the ONLY delta the registration
    /// carries: the `soul_fire_flame` sprite the one-texture json
    /// banks (the blue pixels the `minecraft:soul_fire_flame`
    /// texture holds). THE CORRECTIVE BYTE AUDIT the fresh
    /// `ParticleResources.registerProviders` dump banks: the
    /// `SOUL_FIRE_FLAME` type rides the IDENTICAL invokedynamic call
    /// site the plain `FLAME` type rides (indy cp#125 — the
    /// bootstrap arg cp#1085 the `FlameParticle$Provider.<init>`
    /// methodref answers), the FULL quad the plain provider keeps —
    /// NO scale the provider's own walk spends (the half-quad
    /// claim the round-454 docs spoke the raw operands correct; the
    /// `scale(0.5F)` tail belongs to `FlameParticle
    /// $SmallFlameProvider` — the `SMALL_FLAME` type's OWN provider,
    /// indy cp#447 — the SmallFlame kind the eject walk's flame
    /// answers). The `soul_fire_flame` type the 3011 walk's OMINOUS
    /// flame rides (the `TrialSpawner$FlameParticle.OMINOUS`
    /// constant the clinit's second banks — the decoded
    /// `particleType` the `addSpawnParticles` arg passes).
    SoulFlame,
    /// `FlameParticle` the SMALL provider arms — the SAME class the
    /// plain `flame` type wears (the bare RisingParticle ctor the
    /// `getLayer`'s OPAQUE answer speaks, the (1 - t*t*0.5) shrink
    /// the getQuadSize override keeps, the white tint the ctor
    /// defaults speak, the 0.96 friction, the no gravity, the
    /// *0.01 velocity fold the `xd = xd * 0.01D + xdArg` add the
    /// ctor's own pc 26-66 speaks — the ARG velocities the
    /// addParticle args carry enter FULL scale) — the ONE delta the
    /// `FlameParticle$SmallFlameProvider` tail carries: the
    /// `scale(0.5F)` call the ldc1 0.5 + invokevirtual pair pc
    /// 34-36 speaks (the quadSize the half-fold keeps, NO draw the
    /// scale ever spends); the sprite the `small_flame.json` list
    /// banks — `minecraft:flame` the SAME pixels the plain flame
    /// holds, NO new assets the kind demands. The `small_flame`
    /// type the 3014/3017 eject walk's flame rides (the
    /// `TrialSpawner.addEjectItemParticles` walk's first getstatic
    /// the `zd * 0.25` fold speaks) — the candle/vault flame the
    /// SAME registration answers (indy cp#447, the bootstrap arg
    /// cp#1349 the SmallFlameProvider methodref carries).
    SmallFlame,
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
    /// `DragonBreathParticle` — the dragon fireball's own splash puff
    /// (the `dragon_breath` type the `LevelEventHandler` case-2006
    /// arm's `PowerParticleOption.create(ParticleTypes.DRAGON_BREATH,
    /// f6)` walk speaks): the three-frame walk the age's own ride
    /// answers (`setSpriteFromAge` — the dragon_breath.json's
    /// ASCENDING `generic_5/6/7` list the file fold speaks), the
    /// purple tint the ctor's own ranges carry (the r/b the
    /// `Mth.nextFloat` bands give, the green the (0,0) range keeps —
    /// the draw the stream still spends), the no-gravity rise the
    /// `setPower` fold's own 0.1 y-floor speaks, the 0.96 friction the
    /// x/z lanes alone wear (the y-lane the tick's ground gate never
    /// arms — the hasPhysics-false move the plain translate answers),
    /// the 20..100 lifetime the f64-domain fold gives, the OPAQUE
    /// layer the getLayer override returns, the growth the getQuadSize's
    /// 32x clamp rides (the same law the crit star blooms by).
    DragonBreath,
    /// `PlayerCloudParticle` — the water-evaporating steam's own puff
    /// (the `cloud` type the `LevelEventHandler` case-2009 arm's
    /// `addParticle(ParticleTypes.CLOUD, ...)` walk speaks): the
    /// eight-frame walk the age's own ride answers
    /// (`setSpriteFromAge` — the cloud.json's DESCENDING
    /// `generic_7..0` list the file fold speaks — the same generic
    /// pool the poof bank loads), the tint the ctor's ONE
    /// `nextFloat * 0.3` draw carries (the r/g/b trio the shared
    /// f16 keeps — the near-white the steam nights wear), the
    /// no-gravity drift the zero velocity trio speaks, the 0.96
    /// friction the all-three-axes lanes wear (the y-lane the base
    /// tick's own body arms), the 17..65 lifetime the `8/(r*0.8+0.3)
    /// × 2.5` fold gives, the hasPhysics-false no-collision move the
    /// plain translate answers, the player-lift the tick override's
    /// own tail speaks (the 2.0-range read the feet distance folds,
    /// the 0.2 blends the pose and the vy ease toward the player's
    /// own level), the TRANSLUCENT layer the getLayer override
    /// returns, the growth the getQuadSize's 32x clamp rides (the
    /// same law the crit star blooms by).
    Cloud,
    /// `HugeExplosionParticle` — the dragon's own block-break burst
    /// (the `explosion` type the `LevelEventHandler` case-2008 arm's
    /// ONE `addParticle(EXPLOSION, x + 0.5, y + 0.5, z + 0.5, 0, 0,
    /// 0)` walk speaks): the sixteen-frame walk the age's own ride
    /// answers (`setSpriteFromAge` — the explosion.json's ASCENDING
    /// `explosion_0..15` list the index IS the file), the tint the
    /// ctor's ONE `nextFloat * 0.6 + 0.4` draw carries (the r/g/b
    /// trio the shared f11 keeps — the grey the blast nights wear),
    /// the stationary tick the override's own body speaks (the
    /// prev-trio + the age walk + the removal gate — no motion the
    /// zero velocity trio keeps, no friction lane, no gravity), the
    /// 6..9 lifetime the `6 + nextInt(4)` draw gives, the quadSize
    /// the ctor's own `2.0 * (1.0 - (float)xd * 0.5)` write answers
    /// (the case's zero speed the 2.0 keeps), the OPAQUE layer the
    /// getLayer override returns, the NO getQuadSize override the
    /// base law keeps (the plain quad the whole life wears).
    Explosion,
    /// `HugeExplosionSeedParticle` — the gateway spawn's own
    /// invisible engine (the `explosion_emitter` option the
    /// `LevelEventHandler` case-3000 arm's ONE
    /// `addAlwaysVisibleParticle(EXPLOSION_EMITTER, true, x + 0.5, y
    /// + 0.5, z + 0.5, 0, 0, 0)` walk speaks): the `NoRenderParticle`
    /// subclass the seed's own class bytes keep — NOTHING the quad
    /// lanes bank (the honest absence the render match speaks), the
    /// 8-tick countdown the ctor's `lifetime = 8` write carries, the
    /// per-tick SIX `ParticleTypes.EXPLOSION` children the tick's
    /// `i < 6` loop banks (each child the `(nextDouble -
    /// nextDouble) * 4.0` triple-offset pair the seed's own random
    /// draws carries, the child's size input the
    /// `(float)age / (float)lifetime` walk feeds — the
    /// `HugeExplosionParticle`'s `2.0F * (1.0F - xa * 0.5F)` quadSize
    /// law the children's shrink rides), the removal the `age ==
    /// lifetime` gate answers (the 48-child burst the eight ticks
    /// spend, the last six the death tick still spawns).
    ExplosionSeed,
    /// `PortalParticle` — the eye-of-ender landing's own ring (the
    /// `portal` type the `LevelEventHandler` case-2003 arm's ring
    /// walk speaks): the RANDOM STATIC frame the provider's
    /// `SpriteSet.get(random)` pick answers (the `nextInt(8)` draw
    /// the eight generic frames consume — the portal.json's
    /// ASCENDING `generic_0..7` list the file index IS the draw, NO
    /// setSpriteFromAge the class never speaks), the tint the ctor's
    /// own draw chain carries (the `f = nextFloat * 0.6 + 0.4` the
    /// r `* 0.9` / g `* 0.3` / b plain putfield trio keeps — the
    /// purple the ring nights wear), the 40..49 lifetime the
    /// `(int)(nextFloat * 10.0) + 40` draw gives, the swirl the tick
    /// override's own body speaks (the position Start + velocity ×
    /// the `1 − (2t² − t)` blend the f1 fold answers, the y-lift
    /// `(1 − t)` the loc2 copy rides — the friction field the
    /// recompute never reads), the OPAQUE layer the getLayer
    /// override returns, the growth the getQuadSize override's
    /// `1 − (1 − t)²` fold answers (the ring swells as it pulls
    /// in). The Start the variant carries (the ctor's own
    /// xStart/yStart/zStart write — the recompute the swirl's
    /// exact-from-Start law demands).
    Portal {
        /// The SpriteSet.get(random) pick (the nextInt(8) draw's
        /// answer — the ascending json the file index IS).
        frame: usize,
        /// The swirl's own Start (the ctor's xStart/yStart/zStart —
        /// the ring position the addParticle args place).
        sx: f64,
        sy: f64,
        sz: f64,
    },
    /// `BreakingItemParticle` the ENDER_EYE shard wears — the
    /// eye-of-ender landing's own debris (the `ItemParticleOption(
    /// ParticleTypes.ITEM, Items.ENDER_EYE)` walk the case-2003
    /// arm's `bipush 8` loop speaks, the SAME class the splash
    /// potion's shard rides — the quarter-region uv walk the terrain
    /// dust shares, the uo/vo the ctor's own `nextFloat() * 3.0`
    /// draws give, the gravity 1.0 the ctor passes, the base 0.98
    /// friction, the `/2` size fold, the base lifetime the ctor
    /// never overrides): the ONE sprite the eye's own model keeps
    /// (the `item/ender_eye.png` layer0 the generated stanza speaks
    /// — the single layer the `nextInt(1)` pick draws, the draw the
    /// stream still spends), the white tint the item pixels wear.
    EyeItem,
    /// `BreakingItemParticle` the EGG shard wears — the thrown egg's
    /// own debris (javap `ClientLevel.handleEntityEvent` case 3 — the
    /// `spawnItemCrackParticles` walk the ThrownEgg's own discard
    /// broadcasts: the `bipush 10` loop the ten shards speak, the
    /// `ItemParticleOption(ParticleTypes.ITEM, stack)` the egg's own
    /// sprite wears — the SAME class the eye's shard rides, the
    /// quarter-region uv walk the terrain dust shares, the uo/vo the
    /// ctor's own `nextFloat() * 3.0` draws give, the gravity 1.0 the
    /// ctor passes, the base 0.98 friction, the `/2` size fold, the
    /// base lifetime the ctor never overrides): the ONE sprite the
    /// egg's own model keeps (the `item/egg.png` layer0 the generated
    /// stanza speaks — the single layer the `nextInt(1)` pick draws,
    /// the draw the stream still spends), the white tint the item
    /// pixels wear.
    EggItem,
    /// `GlowParticle` — the wax trio's own puff (the single `glow`
    /// sprite the `wax_on` / `wax_off` / `scrape` jsons' one-texture
    /// sets keep, the provider tints the ctor's own `setColor` rows
    /// speak — the wax-on amber, the wax-off white, the scrape's
    /// nextBoolean teal pair — the 0.96 friction the ctor's own
    /// putfield keeps, the physics-free fly the `hasPhysics = false`
    /// write rides, the OPAQUE layer the `getLayer` override returns,
    /// the 10..39 lifetime the `nextInt(30) + 10` band carries).
    Glow,
    /// `SculkChargeParticle` — the LevelEvent-3006 face walk's own
    /// pulse (the 7-frame `sculk_charge_0..6` set the json's
    /// ASCENDING list orders, the `setSpriteFromAge` walk the age
    /// law rides): the static roll the options carry — π for the
    /// DOWN face the all-faces arm picks, π for the UP face the
    /// unpack arm keeps, 0 otherwise — the quad the renderer flips
    /// half a turn when `flipped` rides (`flipped: bool` the kind
    /// folds from the roll), the 1.5x scale the ctor's own write
    /// carries, the 0.96 friction, the physics-free fly, the
    /// fullbright look the `getLightCoords` withBlock(15) override
    /// speaks, the TRANSLUCENT layer the `getLayer` override
    /// returns, the 8..19 lifetime the provider's `nextInt(12) + 8`
    /// draw gives, the VERBATIM caller speed the provider's own
    /// `setParticleSpeed` keeps (no multiply chain — the tiny
    /// ±0.005 supplier draws the speed already carries).
    SculkCharge {
        /// The options roll's own fold (`roll != 0.0` — the π
        /// faces the quad's half-turn uv walk rides).
        flipped: bool,
    },
    /// `SculkChargePopParticle` — the LevelEvent-3006 pop arm's own
    /// cube burst (the 4-frame `sculk_charge_pop_0..3` set the
    /// json's ASCENDING list orders, the same `setSpriteFromAge`
    /// law): the 1.0 scale the ctor keeps, the 0.96 friction, the
    /// physics-free fly, the fullbright look, the TRANSLUCENT layer,
    /// the 6..9 lifetime the provider's `nextInt(4) + 6` draw
    /// gives, the ±0.07 cube speeds the caller's trio verbatim
    /// carries.
    SculkChargePop,
    /// `SonicBoomParticle` — the warden's ranged dose's own beam
    /// ring (javap the 26.2 class verbatim: the
    /// `HugeExplosionParticle` chain the `lifetime 16` +
    /// `quadSize 1.5f` overrides keep, the base ctor's grey draw
    /// `nextFloat() * 0.6f + 0.4f` the r/g/b ONE draw shares, the
    /// velocity the base's dconst_0 trio ZEROES — the static ring
    /// the sendParticles count-1 walk plants, the OPAQUE layer the
    /// `getLayer` override returns, the no-fade tick the age walk
    /// keeps, the 16-frame `sonic_boom_0..15` set the json's
    /// ASCENDING list orders — the `setSpriteFromAge` law the age
    /// walk rides, the `MutableSpriteSet.get(age, lifetime)` fold
    /// the frame index speaks).
    SonicBoom,
    /// `ShriekParticle` — the LevelEvent-3007 arm's own ring (the
    /// single 13x13 `shriek` sprite the one-texture json keeps, the
    /// `SpriteSet.get(random)` pick the Provider speaks — the
    /// `BitRandomSource.nextInt(1)` draw the power-of-2 arm still
    /// spends): the DELAY the option carries (0, 5, .., 45 ticks —
    /// the ten-ring's staggered launch the i*5 fold gives) the sleep
    /// walk rides (NO age, NO physics the delay guards keep — the
    /// render skips the asleep the same way), the 30-tick life the
    /// ctor's `bipush 30` write carries, the 0.1 rise the `yd`
    /// write speaks (the base 0.98 friction the decay rides), the
    /// 0.85 quadSize the ctor's DIRECT putfield keeps (no random —
    /// the growth the `getQuadSize`'s `* 0.75` clamp law gives),
    /// the alpha fade the `1 - t` extract speaks (the brightness
    /// dim the renderer's honest stand-in wears), the fullbright
    /// look the `getLightCoords` withBlock(15) override speaks, the
    /// TRANSLUCENT layer, the TWO tilted quads the crossed-rings
    /// look wears (the rotationX(-60°) / rotationYXZ(-180°, 60°, 0)
    /// pair the camera-space quaternion maths speaks — the
    /// (right, up, away) basis tilt the billboard raster's honest
    /// equivalent keeps).
    SculkShriek {
        /// The option's own delay (the ticks the sleep walk counts
        /// down — 0, 5, .., 45 the i*5 fold gives).
        delay: u32,
    },
    /// `SoulParticle` — the SCULK_SOUL bloom wisp (javap the
    /// `SculkCatalystBlockEntity$CatalystListener.bloom`'s
    /// `sendParticles(SCULK_SOUL, ...)` post the 26.2 jar speaks, the
    /// `sculk_soul` particle type's OWN 11-frame set the json banks —
    /// `minecraft:sculk_soul_0..10` ASCENDING — the `SoulParticle`
    /// ctor chain the walk serves: the base draws (the dead life + the
    /// size the scale(1.5F) fold rides), the `RisingParticle` jit +
    /// lifetime overrides, the sprite the `SpriteSet.first()` /
    /// `setSpriteFromAge` picks — NO draw the first() call speaks),
    /// the zero velocity the packet's gaussian*0.0 trio keeps (the
    /// handler's 6 nextGaussian draws the SPENDS the 0-speed
    /// multiplies discard — the wisps hover the jitter ±0.05 alone
    /// wears), the white tint the ctor never tints, the alpha 1.0 the
    /// provider's own setAlpha speaks, the TRANSLUCENT layer the
    /// getLayer override returns, the NOT-glowing light the default
    /// isGlowing=false keeps (the glow twin the $GlowProvider alone
    /// arms).
    #[cfg_attr(not(test), expect(dead_code))]
    SculkSoul,
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

/// The dragon breath's 3 frames (the `generic_5..generic_7` textures
/// the 26.2 dragon_breath.json speaks — the same generic pool the
/// smoke's 8-frame set banks, the json's ASCENDING list orders).
pub const DRAGON_BREATH_FRAME_COUNT: usize = 3;

/// The explosion burst's 16 frames (the `explosion_0..15` textures
/// the jar speaks — the sprite set the json's ASCENDING list orders,
/// the list index the frame law reads directly).
pub const EXPLOSION_FRAME_COUNT: usize = 16;

/// The explosion burst's own frame law (javap `Particle
/// .setSpriteFromAge` -> `MutableSpriteSet.get(age, lifetime)`
/// verbatim: `sprites.get(age * (size - 1) / lifetime)` — 16 frames,
/// the 6..9-tick life walking the ascending list the json speaks;
/// the min clamp the list.get's own bound speaks — the vanilla walk
/// never fires it (the age walk removes the particle at the lifetime
/// first), the Rust shape keeps it honest).
pub fn explosion_frame(age: u32, lifetime: u32) -> usize {
    ((age as usize * (EXPLOSION_FRAME_COUNT - 1)) / lifetime.max(1) as usize)
        .min(EXPLOSION_FRAME_COUNT - 1)
}

/// The sculk charge's 7 frames (the `sculk_charge_0..6` textures the
/// jar speaks — the sprite set the json's ASCENDING list orders, the
/// list index the frame law reads directly).
pub const SCULK_CHARGE_FRAME_COUNT: usize = 7;

/// The sculk charge pop's 4 frames (the `sculk_charge_pop_0..3`
/// textures the jar speaks — the sprite set the json's ASCENDING list
/// orders, the list index the frame law reads directly).
pub const SCULK_POP_FRAME_COUNT: usize = 4;

/// The SONIC BOOM's 16 frames (the `sonic_boom_0..15` textures the
/// jar speaks — the `sonic_boom` particle type's OWN set the json
/// lists ASCENDING, the list index the frame law reads directly).
pub const SONIC_BOOM_FRAME_COUNT: usize = 16;

/// The sonic boom's own frame law (javap `Particle
/// .setSpriteFromAge` → `MutableSpriteSet.get(age, lifetime)`
/// verbatim: `sprites.get(age * (size - 1) / lifetime)` — 16
/// frames, the 16-tick life walking the ascending list the json
/// speaks; the min clamp the list.get's own bound speaks — the
/// vanilla walk never fires it (the age walk removes the particle
/// at the lifetime first), the Rust shape keeps it honest).
pub fn sonic_boom_frame(age: u32, lifetime: u32) -> usize {
    ((age as usize * (SONIC_BOOM_FRAME_COUNT - 1)) / lifetime.max(1) as usize)
        .min(SONIC_BOOM_FRAME_COUNT - 1)
}

/// The sculk charge's own frame law (javap `Particle.setSpriteFromAge`
/// -> `MutableSpriteSet.get(age, lifetime)` verbatim:
/// `sprites.get(age * (size - 1) / lifetime)` — 7 frames, the 8..19
/// tick life walking the ascending list the json speaks; the min
/// clamp the list.get's own bound speaks — the vanilla walk never
/// fires it (the age walk removes the particle at the lifetime
/// first), the Rust shape keeps it honest).
pub fn sculk_charge_frame(age: u32, lifetime: u32) -> usize {
    ((age as usize * (SCULK_CHARGE_FRAME_COUNT - 1)) / lifetime.max(1) as usize)
        .min(SCULK_CHARGE_FRAME_COUNT - 1)
}

/// The sculk pop's own frame law (the same `MutableSpriteSet.get`
/// walk — 4 frames, the 6..9 tick life walking the ascending list).
pub fn sculk_pop_frame(age: u32, lifetime: u32) -> usize {
    ((age as usize * (SCULK_POP_FRAME_COUNT - 1)) / lifetime.max(1) as usize)
        .min(SCULK_POP_FRAME_COUNT - 1)
}

/// The SCULK_SOUL wisp's 11 frames (the `sculk_soul_0..10` textures
/// the jar speaks — the `sculk_soul` particle type's OWN set the json
/// lists ASCENDING, the list index the frame law reads directly; the
/// plain `soul` type's `soul_0..10` set the SEPARATE pool the same
/// walk speaks — the two jsons the two lists keep, this lane the
/// catalyst's own type wears).
pub const SCULK_SOUL_FRAME_COUNT: usize = 11;

/// The soul wisp's own frame law (the same `MutableSpriteSet.get`
/// walk — 11 frames, the 12..44 tick life walking the ascending list).
pub fn sculk_soul_frame(age: u32, lifetime: u32) -> usize {
    ((age as usize * (SCULK_SOUL_FRAME_COUNT - 1)) / lifetime.max(1) as usize)
        .min(SCULK_SOUL_FRAME_COUNT - 1)
}

/// The soul wisp's own lifetime law (javap `RisingParticle` ctor pc
/// 165-191 verbatim: `(int)(8.0 / (nextFloat * 0.8 + 0.2)) + 4` — the
/// divisor 0.2..1.0 the 8..40 quotient speaks, the +4 tail the
/// 12..44 band wears).
#[cfg_attr(not(test), expect(dead_code))]
pub fn sculk_soul_lifetime(r: f32) -> u32 {
    (8.0 / (r as f64 * 0.8 + 0.2)) as u32 + 4
}

/// The shriek's own lifetime (javap the ctor's `bipush 30` write —
/// FIXED, no draw the 30-tick band speaks).
pub const SCULK_SHRIEK_LIFETIME: u32 = 30;

/// The shriek's own growth law (javap `ShriekParticle.getQuadSize`
/// verbatim: `quadSize * clamp(((age + partial) / lifetime) * 0.75F,
/// 0, 1)` — the ring grows from NOTHING to the `0.85 * 0.75 =
/// 0.6375` cap the 30 ticks speak, the clamp never fires).
pub fn shriek_grow(age: u32, lifetime: u32, partial: f32) -> f32 {
    let t = (age as f32 + partial) / lifetime.max(1) as f32;
    (t * 0.75).clamp(0.0, 1.0)
}

/// The shriek's own fade law (javap the `extract` walk's alpha
/// verbatim: `1 - clamp((age + partial) / lifetime, 0, 1)` — the ring
/// dims the whole 30-tick life; the raster's alpha lane never
/// carries — the brightness dim the tint's own fold answers).
pub fn shriek_fade(age: u32, lifetime: u32, partial: f32) -> f32 {
    1.0 - ((age as f32 + partial) / lifetime.max(1) as f32).clamp(0.0, 1.0)
}

/// The dragon breath's own frame law (javap `Particle.setSpriteFromAge`
/// -> `MutableSpriteSet.get(age, lifetime)` verbatim:
/// `sprites.get(age * (size - 1) / lifetime)` — 3 frames, the list
/// index the walk gives).
pub fn dragon_breath_frame(age: u32, lifetime: u32) -> usize {
    ((age as usize * (DRAGON_BREATH_FRAME_COUNT - 1)) / lifetime.max(1) as usize)
        .min(DRAGON_BREATH_FRAME_COUNT - 1)
}

/// The dragon breath's render-file index (the json's ASCENDING list
/// the file fold speaks — the smoke/poof jsons' DESCENDING order the
/// `generic_file_index` fold mirrors; here the list index 0 reads
/// `generic_5` directly, the file number the +5 offset keeps).
pub fn dragon_breath_file(list_index: usize) -> usize {
    5 + list_index
}

/// The dragon breath's own lifetime law (javap `DragonBreathParticle`
/// ctor verbatim — the f64 DOMAIN the bytecode's own f2d order speaks:
/// `nextFloat; f2d; dmul 0.8; dadd 0.2; ddiv 20.0; d2i` — the draw
/// converts FIRST, the multiply-add the double domain keeps, the
/// spell/smoke folds' f32-first arithmetic the one order's own
/// difference wears): 20..100 ticks.
pub fn dragon_breath_lifetime(r: f32) -> u32 {
    (20.0f64 / (f64::from(r) * 0.8f64 + 0.2f64)) as u32
}

/// The cloud's own lifetime law (javap `PlayerCloudParticle` ctor
/// verbatim — the f64 DOMAIN the bytecode's own f2d order speaks:
/// `ldc2_w 8.0; nextFloat; f2d; ldc2_w 0.8; dmul; ldc2_w 0.3; dadd;
/// ddiv; d2i` the base draw answers 7..26, then the `iload 17; i2f;
/// ldc 2.5F; fmul; fconst_1; Math.max; f2i` tail the ×2.5 fold
/// rides — the f64 0.3 offset the smoke folds' 0.2 the one
/// constant's own difference wears): 17..65 ticks (the r=0 edge
/// draws 8/0.3 = 26.67 → 26 × 2.5 = 65, the r→1 arm 8/1.1 = 7.27 →
/// 7 × 2.5 = 17.5 → 17 — the max floor the band never fires).
pub fn cloud_lifetime(r: f32) -> u32 {
    let base = (8.0f64 / (f64::from(r) * 0.8f64 + 0.3f64)) as u32;
    ((base as f32 * 2.5f32).max(1.0f32)) as u32
}

/// One particle — the vanilla `Particle` + `SingleQuadParticle` fields
/// the three kinds' laws speak (the fields the tick and the render
/// read; the removed lane rides the caller's retain).
#[derive(Debug)]
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

    /// The SparkParticle's own tick tail (javap `SparkParticle.tick` +
    /// `SimpleAnimatedParticle.tick` the 26.2 walk — the base law the
    /// super call keeps, the fade lerp the hasFade bit gates, the
    /// trail CHILD the gate arm spawns): the base `tick()` runs FIRST
    /// (the physics the super chain speaks — the age the increment
    /// banks), then the color lerp (`rCol += (fadeR − rCol) × 0.2F`
    /// — the #75 fold pcs 57-96 speak, all THREE channels), then the
    /// child gate (`trail && age < lifetime / 2 && (age + lifetime) %
    /// 2 == 0` — pcs 4-35 the icmpge/irem pair keeps): the child
    /// rides the parent's own POST-fade color, the fade target, the
    /// twinkle bit — the spec the caller's post-loop push consumes
    /// (the vanilla queue order the other lanes keep). The twinkle
    /// bit the CHILDREN alone carry (no render override the 26.2
    /// class speaks — the recorded note).
    pub fn tick_spark(
        &mut self,
        children: &mut Vec<(f64, f64, f64, [f32; 3], Option<[f32; 3]>, bool)>,
    ) -> bool {
        let (trail, twinkle, fade) = match self.kind {
            ParticleKind::Spark {
                trail,
                twinkle,
                fade,
            } => (trail, twinkle, fade),
            _ => return self.tick(),
        };
        // the base law first (the super.tick() the override keeps)
        let alive = self.tick();
        // THE FADE LERP (javap SimpleAnimatedParticle.tick pcs 50-120:
        // the hasFade bit gates, the three channels the ×0.2 fold
        // walks — the fade target the setFadeColor decode banked)
        if let Some(f) = fade {
            self.col[0] += (f[0] - self.col[0]) * 0.2;
            self.col[1] += (f[1] - self.col[1]) * 0.2;
            self.col[2] += (f[2] - self.col[2]) * 0.2;
        }
        // THE TRAIL CHILD'S OWN GATE (javap SparkParticle.tick pcs
        // 4-35: the trail bit AND the half-life band AND the (age +
        // lifetime) % 2 pair — the child at the CURRENT pose, the
        // zero motion the ctor args keep, the parent's own color +
        // fade + twinkle the stores ride)
        if trail && self.age < self.lifetime / 2 && (self.age + self.lifetime) % 2 == 0 {
            children.push((self.x, self.y, self.z, self.col, fade, twinkle));
        }
        alive
    }

    /// The campfire smoke's own tick (javap `CampfireSmokeParticle.tick`
    /// the 26.2 jar verbatim — the FULL override the base law never
    /// reaches): the head saves the pre-move pose, the age walks, the
    /// remove the `age >= lifetime || alpha <= 0` OR-pair answers (the
    /// drained alpha the early exit wears), then the DRIFT the four
    /// rolls per tick speak — `xd += nextFloat()/5000.0f * (nextBoolean
    /// ? 1 : -1)` the x and z lanes keep (the f32 math the f2d
    /// widening wears), `yd -= gravity` the RAW subtract rides (the
    /// 3e-6 the NO 0.04 factor the base lane folds), the plain move
    /// (the collision lane sleeps — the pieces' honest absence), NO
    /// friction the override never applies, then the FADE the
    /// last-60-ticks band serves — `age >= lifetime - 60 && alpha >
    /// 0.01f` the gate keeps, `-0.015f` per tick the drain walks (the
    /// 0.9 spawn alpha the 60-tick band exactly drains — the remove
    /// the age walk the same tick answers). The frame the ONE pick
    /// static keeps (no setSpriteFromAge the override never calls).
    /// The tick's own draws ride the caller's rng (the vanilla
    /// per-particle `RandomSource.create()` the level's ambient stream
    /// the lava tick's own precedent stands in for).
    pub fn tick_campfire_smoke(&mut self, rng: &mut crate::drops::Lcg) -> bool {
        let (frame, alpha) = match self.kind {
            ParticleKind::CampfireSmoke { frame, alpha } => (frame, alpha),
            _ => return self.tick(),
        };
        self.tick_head();
        self.age += 1;
        if self.age >= self.lifetime || alpha <= 0.0 {
            return false;
        }
        // THE DRIFT'S OWN ROLLS (the f32 /5000.0 the ± coin folds —
        // the f2d widening the dadd pairs wear)
        self.vx += f64::from(rng.next_f32() / 5000.0) * if rng.next() & 1 == 0 { 1.0 } else { -1.0 };
        self.vz += f64::from(rng.next_f32() / 5000.0) * if rng.next() & 1 == 0 { 1.0 } else { -1.0 };
        // THE RAW GRAVITY SUBTRACT (the 3e-6 the no-0.04 law keeps)
        self.vy -= self.gravity;
        // THE PLAIN MOVE (the collision lane sleeps)
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
        // THE FADE'S OWN BAND (the last 60 ticks the −0.015 drain —
        // the >0.01 gate the final sub-quantum ticks wear)
        if let ParticleKind::CampfireSmoke { alpha, .. } = &mut self.kind {
            if self.age >= self.lifetime.saturating_sub(60) && *alpha > 0.01 {
                *alpha -= 0.015;
            }
        }
        let _ = frame;
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
            // the dragon breath's own fast bloom (the
            // DragonBreathParticle.getQuadSize override — the SAME 32x
            // clamp law the crit speaks: `(age + f) / lifetime * 32.0F`
            // the full size the first lifetime/32 ticks reach)
            ParticleKind::DragonBreath => base * crit_grow(self.age, self.lifetime, 0.0),
            // the cloud's own fast bloom (PlayerCloudParticle.getQuadSize
            // override — the SAME 32x clamp law the crit speaks: the
            // steam grows from a point inside the first lifetime/32
            // ticks, the full quad the rest of the life keeps)
            ParticleKind::Cloud => base * crit_grow(self.age, self.lifetime, 0.0),
            // the flame's own half-thin (the FlameParticle.getQuadSize
            // override — the (1 - t*t*0.5) fold the death walks to the
            // HALF quad, the lava's own to-none never keeps it)
            ParticleKind::Flame => base * flame_shrink(self.age, self.lifetime, 0.0),
            // the soul flame's own half-thin (the SAME
            // FlameParticle.getQuadSize override the plain flame wears
            // — the FULL quad the plain Provider registration keeps,
            // the shrink law the class share speaks)
            ParticleKind::SoulFlame => base * flame_shrink(self.age, self.lifetime, 0.0),
            // the small flame's own half-thin (the SAME
            // FlameParticle.getQuadSize override the plain flame wears
            // — the scale(0.5F) half the SmallFlameProvider's birth
            // quad keeps, the shrink law the class share speaks)
            ParticleKind::SmallFlame => base * flame_shrink(self.age, self.lifetime, 0.0),
            // the portal ring's own swell (the PortalParticle.getQuadSize
            // override — the 1 − (1 − t)² fold the partial rides 0.0 the
            // same way the crit's bloom does: the quad starts a point,
            // the full size the life's end reaches)
            ParticleKind::Portal { .. } => base * portal_grow(self.age as f32 / self.lifetime.max(1) as f32),
            // the shriek's own growth (javap ShriekParticle.getQuadSize
            // — the `* 0.75` clamp law the 30-tick life rides: the ring
            // grows from a point to the 0.6375 cap, the partial rides
            // 0.0 the same way the crit's bloom does)
            ParticleKind::SculkShriek { .. } => {
                base * shriek_grow(self.age, self.lifetime, 0.0)
            }
            // the flash's own bell (javap OverlayParticle.getQuadSize —
            // the SIZE the bell returns, the corner span the ±size/2
            // convention halves; the negative first-tick fold the
            // vanilla vertex fold's own ±f5 swap answers — the covered
            // area the same square keeps, the magnitude the raster's
            // CCW edge test the flip would otherwise trip preserves)
            ParticleKind::Flash { .. } => flash_bell(self.age, 0.0).abs() * 0.5,
            _ => base,
        }
    }

    /// The `ShriekParticle.tick` verbatim (javap over the 26.2 jar) —
    /// the `delay > 0` gate the sleep walk rides: the countdown the
    /// kind's own delay field carries (NO age walk, NO physics the
    /// asleep ticks never spend), the base law the awake arm keeps.
    pub fn tick_shriek(&mut self) -> bool {
        if let ParticleKind::SculkShriek { delay } = &mut self.kind {
            if *delay > 0 {
                *delay -= 1;
                return true;
            }
        }
        self.tick()
    }

    /// The `SnowflakeParticle.tick` verbatim (javap over the 26.2
    /// jar) — the base law the super call keeps (the gravity 0.225
    /// the ctor's field speaks, the friction 1.0F the base law's own
    /// no-op arms), then the damping tail the override speaks:
    /// `xd *= 0.95, yd *= 0.9, zd *= 0.95` — the flake slows each
    /// tick, the vertical axis's stronger 0.9 the fall damps.
    pub fn tick_snowflake(&mut self) -> bool {
        let alive = self.tick();
        if !alive {
            return false;
        }
        self.vx *= 0.95;
        self.vy *= 0.9;
        self.vz *= 0.95;
        true
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

    /// The `DragonBreathParticle.tick` verbatim — the override the
    /// base law never rides: the age walk + the removal (the same
    /// gate the base speaks), the sprite walk (the frame the render's
    /// own age read answers), then the ground gate the hasPhysics-
    /// false move NEVER arms (`onGround` the collision walk's own
    /// writer — the plain translate the field's writer never reaches:
    /// the `yd = 0` + `hasHitGround = true` pair, the `+0.002`
    /// up-drift branch, the y-friction tail — the branch trio the
    /// death keeps dead, the bytecode's own shape the faithful port
    /// documents), the move (the plain translate the hasPhysics=false
    /// field answers), the y-stall speed-up (`if (y == yo) { xd *=
    /// 1.1; zd *= 1.1; }` — the branch the `setPower` fold's own 0.1
    /// y-floor never arms: the vy the 0.1 base keeps alive, the y the
    /// every-tick move answers), the x/z friction lanes (the y-lane
    /// the ground gate never arms — the vy the friction-free rise the
    /// fold's floor speaks).
    pub fn tick_dragon_breath(&mut self) -> bool {
        self.tick_head();
        self.age += 1;
        if self.age >= self.lifetime {
            return false;
        }
        // the ground gate never arms (the hasPhysics-false move the
        // plain translate answers — onGround the writer never reaches,
        // the hasHitGround trio the gate's own death keeps dead)
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
        // the y == yo stall the vy's own 0.1 floor never fires (the
        // move always answers a y-step)
        self.vx *= self.friction;
        self.vz *= self.friction;
        // NO gravity lane, NO vy friction (the x/z lanes alone the
        // override's own body speaks)
        true
    }

    /// The `PlayerCloudParticle.tick` verbatim — the base law first
    /// (the same walk the super tick speaks: the age walk, the gravity
    /// lane the zero field keeps silent, the move the hasPhysics-false
    /// field answers with the plain translate, the 0.96 friction the
    /// all-three-axes body speaks), then the override's own tail: the
    /// sprite walk the render's own age read answers, the nearest
    /// player the 2.0-range read arms (the distance² the feet position
    /// folds — the strict `<` the `e < distance * distance` gate
    /// speaks), the puff ABOVE the player's feet the y > fold answers
    /// — the pose the 0.2 blend eases toward the player's own level,
    /// the vy the player's own rise the blend follows, the setPos the
    /// tail's own write answers (the lift's own branch the only
    /// writer — the fall-through the setPos never reaches). The
    /// removed walk never re-arms (the `if (!removed)` gate the
    /// override's own head keeps).
    pub fn tick_cloud(&mut self, player: Option<(f64, f64, f64, f64)>) -> bool {
        let alive = self.tick();
        if !alive {
            return false;
        }
        // THE LIFT (the override's own tail — the player read the
        // caller's own world answers, the local body the single
        // nearest-player read arms)
        if let Some((px, py, pz, pvy)) = player {
            let dx = px - self.x;
            let dy = py - self.y;
            let dz = pz - self.z;
            if dx * dx + dy * dy + dz * dz < 4.0 && self.y > py {
                self.y += (py - self.y) * 0.2;
                self.vy += (pvy - self.vy) * 0.2;
            }
        }
        true
    }

    /// The `HugeExplosionParticle.tick` verbatim — the override the
    /// base law never rides: the prev-trio the tick head saves, the
    /// age walk (`age++` the dup_x1 body speaks), the removal gate
    /// (`age >= lifetime` the remove() the branch arms) — and
    /// NOTHING else (no motion the zero velocity keeps, no gravity
    /// lane, no friction lane — the stationary burst the whole
    /// 6..9-tick life stays at the block's own center, the sprite
    /// walk the render's own age read answers).
    pub fn tick_explosion(&mut self) -> bool {
        self.tick_head();
        self.age += 1;
        if self.age >= self.lifetime {
            return false;
        }
        true
    }

    /// The `HugeExplosionSeedParticle.tick` verbatim (javap over the
    /// 26.2 jar): the SIX-CHILDREN loop the `i < 6` bound keeps —
    /// each tick the seed spawns six `ParticleTypes.EXPLOSION`
    /// children, the child's own position the
    /// `(nextDouble() - nextDouble()) * 4.0` triple-offset pair the
    /// seed's own random draws carries (the six draws the shared
    /// stream spends, IN THE caller's out-vec order), the child's
    /// own size input the `(float)age / (float)lifetime` walk feeds
    /// (the `HugeExplosionParticle`'s `2.0F * (1.0F - xa * 0.5F)`
    /// quadSize law the children's shrink rides — the full 2.0 quad
    /// the birth tick keeps, the 1.125 the seventh tick banks),
    /// THEN the age walk (the children the death tick still spawns
    /// — the removal the `age == lifetime` gate answers, the
    /// `if_icmpne` branch the bytecode pins). The children ride the
    /// caller's out-vec as `(x, y, z, size_speed)` rows — the
    /// landing walk the queue order keeps (the ctor draws the
    /// landing's own stream spends, AFTER the tick loop the smoke
    /// lane's own walk answers). Returns false the removal tick the
    /// vanilla remove() speaks. NO prev-trio the NoRender walk
    /// saves — the seed never renders, the lerp lane never reads
    /// it.
    pub fn tick_explosion_seed(
        &mut self,
        rng: &mut crate::drops::Lcg,
        out: &mut Vec<(f64, f64, f64, f32)>,
    ) -> bool {
        // the child's size input (the `(float)age / (float)lifetime`
        // walk — the dload/i2f/i2f/fdiv chain the bytecode pins, the
        // value BEFORE the age walk runs)
        let size_speed = self.age as f32 / self.lifetime.max(1) as f32;
        // the six-child loop (the `i < 6` bound the bytecode pins —
        // each child the three (r - r) * 4.0 pairs the seed's own
        // random draws carry)
        for _ in 0..6 {
            let cx = self.x + (rng.next_f64() - rng.next_f64()) * 4.0;
            let cy = self.y + (rng.next_f64() - rng.next_f64()) * 4.0;
            let cz = self.z + (rng.next_f64() - rng.next_f64()) * 4.0;
            out.push((cx, cy, cz, size_speed));
        }
        // the age walk rides AFTER the children (the death tick the
        // last six still spawns — the if_icmpne gate the removal
        // answers)
        self.age += 1;
        self.age != self.lifetime
    }

    /// The `PortalParticle.tick` verbatim — the swirl the override's
    /// own body speaks: the prev-trio the tick head saves, the age
    /// walk (`age++` the dup_x1 body speaks — the NEW age the t read
    /// answers), the removal gate (`age >= lifetime` the remove()
    /// branch arms) — then the RECOMPUTE the Start law keeps: the
    /// blend `1 − (2t² − t)` the f1 fold answers, the position
    /// `Start + velocity * blend` the x/z lanes carry, the y-lift
    /// `(1 − t)` the loc2 copy adds (the ring rides UP as it pulls
    /// in). The friction field the recompute never reads (the
    /// hasPhysics-false plain-translate law the base tick keeps is
    /// the override's own silence — no spread, no ground gate).
    pub fn tick_portal(&mut self) -> bool {
        self.tick_head();
        self.age += 1;
        if self.age >= self.lifetime {
            return false;
        }
        let (sx, sy, sz) = match self.kind {
            ParticleKind::Portal { sx, sy, sz, .. } => (sx, sy, sz),
            _ => (self.x, self.y, self.z),
        };
        // THE SWIRL (the tick's own folds — the NEW age the t read
        // speaks, the blend the f1 fold answers, the y-lift the
        // loc2 copy adds)
        let t = self.age as f32 / self.lifetime as f32;
        let blend = f64::from(portal_swirl_blend(t));
        self.x = sx + self.vx * blend;
        self.y = sy + self.vy * blend + f64::from(1.0f32 - t);
        self.z = sz + self.vz * blend;
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

    /// THE DRIPSTONE HANG'S OWN WALK (javap `DripHangParticle` +
    /// `CoolingDripHangParticle` verbatim): the lava's cooling fade
    /// the preMove head folds FIRST (the r/g/b the `40 − lifetime`
    /// speak — g = 16/(fade+16), b = 4/(fade+8), r the constant 1.0
    /// keeps; the water drop the static provider tint (0.2, 0.3,
    /// 1.0) wears), then the countdown the base preMoveUpdate keeps
    /// (`--lifetime <= 0` → remove + the FALLING option the carried
    /// motion rides into `fall_out`), the RAW gravity lane the
    /// DripParticle.tick's own `yd -= gravity` keeps (the 0.0012 the
    /// DripHang's ×0.02 fold gives — NO 0.04 scale the base Particle
    /// law wears), the move (the collision lane sleeps — the drift
    /// the damping kills, the hang's honest absence), the ×0.02
    /// damping the postMoveUpdate's trio keeps.
    pub fn tick_dripstone_hang(
        &mut self,
        fall_out: &mut Vec<(f64, f64, f64, f64, f64, f64, bool)>,
    ) -> bool {
        self.tick_head();
        // THE COOLING FADE (the CoolingDripHangParticle's preMove
        // head — the water drop the static tint keeps)
        if let ParticleKind::DripstoneHang { lava } = &self.kind {
            if *lava {
                let fade = 40 - self.lifetime;
                self.col = [
                    1.0,
                    16.0 / (fade as f32 + 16.0),
                    4.0 / (fade as f32 + 8.0),
                ];
            }
        }
        // THE COUNTDOWN (the preMoveUpdate's own `--lifetime <= 0`
        // walk — the expiry REMOVES and spawns the falling option)
        self.lifetime -= 1;
        if self.lifetime == 0 {
            let lava = matches!(&self.kind, ParticleKind::DripstoneHang { lava: true });
            fall_out.push((self.x, self.y, self.z, self.vx, self.vy, self.vz, lava));
            return false;
        }
        // THE RAW GRAVITY (the DripParticle.tick's own `yd -= gravity`
        // — no 0.04 scale)
        self.vy -= self.gravity;
        // THE MOVE (the collision lane sleeps — the damped drift the
        // hang's own absence keeps)
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
        // THE DAMPING (the postMoveUpdate's ×0.02 trio)
        self.vx *= self.friction;
        self.vy *= self.friction;
        self.vz *= self.friction;
        true
    }

    /// THE DRIPSTONE FALL'S OWN WALK (javap `DripParticle.tick` +
    /// `FallAndLandParticle.postMoveUpdate` verbatim): the countdown
    /// the base preMoveUpdate keeps (the fall CAN time out — the
    /// silence the honest expiry speaks), the raw 0.06 gravity, the
    /// move + the LANDING the client's world-read fold answers (the
    /// vanilla move()'s own clip — the cell the drop ENTERS below its
    /// birth cell the solid check speaks; the birth cell the check
    /// never re-fires — the drop starts BELOW the tip's own collision
    /// shape the 0.25 fold keeps), the landing → remove + the LAND
    /// walk into `land_out` (the splash the water keeps, the
    /// LANDING_LAVA splat the lava spawns), the ×0.98 damping.
    pub fn tick_dripstone_fall(
        &mut self,
        world: Option<&crate::world::World>,
        land_out: &mut Vec<(f64, f64, f64, bool)>,
    ) -> bool {
        self.tick_head();
        // THE COUNTDOWN (the base preMoveUpdate's own walk)
        self.lifetime -= 1;
        if self.lifetime == 0 {
            return false;
        }
        // THE RAW GRAVITY
        self.vy -= self.gravity;
        // THE MOVE + THE COLLISION FOLD (the cell-boundary walk the
        // vanilla clip keeps — the crossed cells the solid check
        // answers)
        let cur_cell = self.y.floor() as i32;
        let ny = self.y + self.vy;
        let entered = ny.floor() as i32;
        let cx = self.x.floor() as i32;
        let cz = self.z.floor() as i32;
        let is_solid = |wy: i32| -> bool {
            world
                .map(|w| w.get(cx, wy, cz) != crate::world::AIR)
                .unwrap_or(false)
        };
        self.x += self.vx;
        self.z += self.vz;
        if entered < cur_cell {
            // THE CROSSED CELLS (the boundary walk the clip keeps —
            // the first solid cell the landing answers at its top)
            let mut landed: Option<f64> = None;
            for wy in (entered..cur_cell).rev() {
                if is_solid(wy) {
                    landed = Some((wy + 1) as f64);
                    break;
                }
            }
            if let Some(boundary) = landed {
                let lava = matches!(&self.kind, ParticleKind::DripstoneFall { lava: true });
                land_out.push((self.x, boundary, self.z, lava));
                return false;
            }
        }
        self.y = ny;
        // THE DAMPING (the base tick tail's ×0.98 trio)
        self.vx *= self.friction;
        self.vy *= self.friction;
        self.vz *= self.friction;
        true
    }

    /// THE DRIPSTONE LAND'S OWN WALK (javap `DripParticle.tick` over
    /// the resting splat — the countdown, the collision fold the
    /// vanilla onGround clip keeps: the pose PINNED at the boundary
    /// the client's honest fold speaks, the ×0.98 damping the motion
    /// fields keep).
    pub fn tick_dripstone_land(&mut self) -> bool {
        self.tick_head();
        self.lifetime -= 1;
        if self.lifetime == 0 {
            return false;
        }
        // THE RESTING FOLD (the vanilla onGround clip the constant y
        // keeps — the splat the boundary holds, the damping the
        // residual motion fields carry)
        self.vx *= self.friction;
        self.vy *= self.friction;
        self.vz *= self.friction;
        true
    }
}

/// THE DRIPSTONE OFFSET FOLD (javap `BlockBehaviour$Properties
/// .lambda$offsetType$1` verbatim — the XZ offsetter the
/// speleothem's own offsetType answers): the 4-bit draw the seed's
/// own lanes keep (`seed & 15` the x, `seed >> 8 & 15` the z), the
/// `/ 15.0F − 0.5 × 0.5` fold the f32→f64 walk keeps, the clamp the
/// `±maxHorizontalOffset` answers (the speleothem's own 0.125 the
/// `SHAPE_BASE.min(X)` speaks — the tighter bound the plants'
/// default 0.25 never reaches). The seed the [`crate::world
/// ::mth_get_seed`] law hashes.
pub fn dripstone_offset(bit: i64, max_off: f32) -> f64 {
    let v = (bit as f32) / 15.0f32;
    let v = f64::from(v);
    let v = (v - 0.5) * 0.5;
    v.clamp(f64::from(-max_off), f64::from(max_off))
}

/// The dripstone drip's HANG drop ctor (javap
/// `DripParticle$DripstoneWaterHangProvider` + `DripstoneLavaHangProvider`
/// verbatim): the ZERO speed trio the addParticle args carry, the
/// `setSize(0.01F, 0.01F)` the quadSize 0.01 folds, the lifetime the
/// DripHang's own `bipush 40` keeps, the gravity the base 0.06 × the
/// DripHang's 0.02 fold gives (0.0012 — the RAW lane the
/// DripParticle.tick keeps), the water tint the setColor(0.2F, 0.3F,
/// 1.0F) override speaks, the lava tint the CoolingDripHang's own
/// fade answers (r = 1.0, g = 16/(fade+16), b = 4/(fade+8) — at the
/// birth the fade is 0: (1.0, 1.0, 0.5)). The SPRITE PICK the ambient
/// stream's own draw carries (`SpriteSet.get(random)` — the
/// nextInt(1) the one-element list answers); the level random the
/// ctor draws and the pick's draw the same merged stream keeps.
pub fn dripstone_drip_hang(
    x: f64,
    y: f64,
    z: f64,
    lava: bool,
    r_sprite: f32,
) -> Particle {
    let _ = r_sprite; // the one-element pick the roll never splits
    Particle {
        kind: ParticleKind::DripstoneHang { lava },
        x,
        y,
        z,
        prev: None,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        age: 0,
        lifetime: 40,
        gravity: 0.0012,
        friction: 0.02,
        quad_size: 0.01,
        uo: 0.0,
        vo: 0.0,
        col: if lava { [1.0, 1.0, 0.5] } else { [0.2, 0.3, 1.0] },
    }
}

/// The dripstone drip's FALL drop ctor (javap
/// `DripParticle$DripstoneWaterFallProvider` + `DripstoneLavaFallProvider`
/// verbatim): the `lifetime = (int)(64.0 / (r × 0.8 + 0.2))` the
/// FallAndLand ctor draws (80..=320), the gravity the base 0.06
/// keeps, the ×0.98 friction the base tick tail speaks, the water
/// tint the setColor(0.2F, 0.3F, 1.0F) keeps, the lava tint the
/// setColor(1.0F, 0.2857143F, 0.08333333F) — the cooling fade's END
/// state the constants speak. The draws: the sprite pick FIRST (the
/// provider's createParticle evaluates the sprite before the ctor
/// runs), then the lifetime float.
pub fn dripstone_drip_fall(
    x: f64,
    y: f64,
    z: f64,
    lava: bool,
    r_sprite: f32,
    r_life: f32,
) -> Particle {
    let _ = r_sprite; // the one-element pick the roll never splits
    Particle {
        kind: ParticleKind::DripstoneFall { lava },
        x,
        y,
        z,
        prev: None,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        age: 0,
        lifetime: (64.0f32 / (r_life * 0.8 + 0.2)) as u32,
        gravity: 0.06,
        friction: 0.98,
        quad_size: 0.01,
        uo: 0.0,
        vo: 0.0,
        col: if lava {
            [1.0, 0.2857143, 0.083333336]
        } else {
            [0.2, 0.3, 1.0]
        },
    }
}

/// The dripstone drip's LAND splat ctor (javap
/// `DripParticle$LavaLandProvider` → `DripLandParticle` verbatim —
/// the LANDING_LAVA option the lava fall's landing spawns): the
/// `lifetime = (int)(16.0 / (r × 0.8 + 0.2))` the DripLand ctor
/// draws (16..=80), the gravity the base 0.06 keeps, the tint the
/// setColor(1.0F, 0.2857143F, 0.08333333F) — the same cooling-end
/// pair the fall keeps. The draws: the sprite pick FIRST, then the
/// lifetime float.
pub fn dripstone_drip_land(x: f64, y: f64, z: f64, r_sprite: f32, r_life: f32) -> Particle {
    let _ = r_sprite; // the one-element pick the roll never splits
    Particle {
        kind: ParticleKind::DripstoneLand,
        x,
        y,
        z,
        prev: None,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        age: 0,
        lifetime: (16.0f32 / (r_life * 0.8 + 0.2)) as u32,
        gravity: 0.06,
        friction: 0.98,
        quad_size: 0.01,
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 0.2857143, 0.083333336],
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

/// The campfire smoke's own lifetime law (javap
/// `CampfireSmokeParticle` ctor pc 27-69 the 26.2 jar speaks): the
/// force flag's own ladder — the COSY provider's `false` arm the
/// `nextInt(50) + 80` bank keeps (80..129 ticks), the SIGNAL
/// provider's `true` arm the `nextInt(50) + 280` bank carries
/// (280..329 — the hay-bale signal fire the offline palette never
/// lights, the arm's own law the helper still wears).
pub fn campfire_smoke_lifetime(r: i32, force: bool) -> u32 {
    (if force { 280 } else { 80 } + r) as u32
}

/// The CAMPFIRE-SMOKE particle's own spawn (javap
/// `CampfireSmokeParticle` + the Cosy/Signal providers over the 26.2
/// jar — the CAMPFIRE_COSY_SMOKE type the dowse's makeParticles walk
/// banks, the full ctor chain the bytecodes pin): the draw order the
/// arg evaluation + the ctor chain speak — the `sprites.get(random)`
/// pick FIRST (the nextInt(12) frame draw the ascending list reads),
/// then the base `Particle(level, DDD)` ctor's DEAD lifetime draw
/// (the `(int)(4.0 / (nextFloat * 0.9 + 0.1))` fold the value the
/// own lifetime write overrides), then the `SingleQuadParticle` tail's
/// SIZE draw (the `0.1 * (r * 0.5 + 0.5) * 2.0` base the `scale(3.0f)`
/// triple multiplies), then the own `nextInt(50)` lifetime + the
/// `nextFloat()/500` yd bump. The body the bytecodes keep: the
/// `setSize(0.25, 0.25)` bb the collision lane sleeps (the quad the
/// render alone wears), the gravity `3.0E-6f` the RAW tick subtract
/// rides (no 0.04 factor the base lane folds), the xd/zd the speed
/// args verbatim (the cosy walk's zeros), the `yd = ySpeed +
/// nextFloat()/500.0` bump the f2d widening wears, the friction the
/// base ctor's 0.98 the tick override never applies (the campfire
/// smoke's own tick the friction lane never folds), the WHITE tint
/// the SingleQuadParticle defaults keep (no tint draw the chain
/// spends — the sprite's own greys the quad samples), the alpha the
/// provider's setAlpha banks (0.9 the Cosy, 0.95 the Signal), the
/// ONE frame static the whole life the no-sprite tick keeps.
pub fn campfire_smoke_puff(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    force: bool,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the frame pick (the SpriteSet.get(random) the ctor arg FIRST
    // evaluates — the nextInt(12) the ascending list reads directly)
    let frame = rng.next_int(CAMPFIRE_BIG_SMOKE_FRAME_COUNT as u64) as usize;
    // the dead base lifetime (the draw the stream keeps, the value
    // the own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    // the base quad size × the scale(3.0f) triple
    let quad = base_quad_size(rng.next_f32()) * 3.0;
    // the own lifetime ladder (the force flag the provider arms keep)
    let lifetime = campfire_smoke_lifetime(rng.next_int(50), force);
    // the yd bump (the nextFloat()/500 the f2d widening wears)
    let bump = f64::from(rng.next_f32() / 500.0);
    Particle {
        kind: ParticleKind::CampfireSmoke {
            frame,
            alpha: if force { 0.95 } else { 0.9 },
        },
        x,
        y,
        z,
        prev: None,
        vx,
        vy: vy + bump,
        vz,
        age: 0,
        lifetime,
        gravity: 3.0e-6,
        friction: 0.98,
        quad_size: quad,
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
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

/// The LARGE-SMOKE puff's own spawn (javap `LargeSmokeParticle` +
/// `$Provider.createParticle` over the 26.2 jar — the
/// `ParticleTypes.LARGE_SMOKE` word the `LevelEventHandler` case-1501
/// arm's `bipush 8` loop speaks): the SAME `SmokeParticle` ctor chain
/// the plain smoke kind wears — the `LargeSmokeParticle` class the
/// bytecodes pin is the `SmokeParticle` subclass whose ONLY delta the
/// ctor's own arg speaks (the `2.5F` quadSizeMultiplier the plain
/// provider's `1.0F` the one constant's own difference carries — the
/// 26.2 `<init>` dumps pin: `super(level, x, y, z, xs, ys, zs,
/// spreads 0.1F³, multiplier, sprites, 0.3F, 8, -0.1F, 1)` both
/// classes speak, the 2.5 the Large passes) — the FULL draw chain the
/// smoke lane's own helper speaks (the dead base lifetime draw, the
/// three spread draws + the d0 pair, the size draw, the tint draw,
/// the own lifetime draw — the NINE draws the torch lane's doc
/// pins), the size fold the multiplier DOUBLES (`quadSize *= 0.75F ×
/// 2.5F` the ×1.875 the plain smoke's ×0.75 the one constant's own
/// difference wears — the puffs the lava's fizz nights grow). The
/// grey tint the `r * 0.3` draw carries, the 8..40 lifetime the
/// smoke fold speaks, the −0.1 gravity the rise rides, the 0.96
/// friction, the 8-frame DESCENDING `generic_7..0` pool the
/// `large_smoke.json` banks VERBATIM — the SAME texture list the
/// smoke.json speaks (the two jsons' identical rows the decode pins,
/// NO new textures the lane demands), the render bank the smoke
/// kind's own arm carries.
pub fn large_smoke_puff(
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
    let grey = r_tint * 0.3;
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
        quad_size: base_quad_size(r_base) * 0.75 * 2.5,
        uo: 0.0,
        vo: 0.0,
        col: [grey, grey, grey],
    }
}

/// The DRAGON-BREATH splash puff's own spawn (javap
/// `DragonBreathParticle` + `$Provider.createParticle` over the 26.2
/// jar): the ctor chain the case's own draw walk feeds — the dead
/// base lifetime draw (the value the dragon ctor's own write
/// overrides), the SingleQuadParticle's size draw (the 0.1..0.2 the
/// ctor's `* 0.75` fold rides), THE TINT DRAWS the three
/// `Mth.nextFloat` ranges speak (the red `0.7176470756530762..
/// 0.8745098114013672` band, the green the `(0, 0)` range the draw
/// the stream still spends, the blue `0.8235294222831726..
/// 0.9764705896377563` band — the purple the fireball's splash
/// nights wear), the lifetime the f64-domain fold answers (20..100 —
/// the `dragon_breath_lifetime` helper the f2d order pins), the
/// friction the ctor's own 0.96 write, the NO-gravity field the
/// override's own tick keeps, the hasPhysics-false no-collision move
/// the plain translate answers. THE SET-POWER FOLD (javap
/// `Particle.setPower` verbatim — the provider's own tail
/// `setPower(option.getPower())` the case's `f6 = nextFloat * 4.0F`
/// arg rides): the x/z the plain `* p` multiply, the y the `(v -
/// 0.1) * p + 0.1` the base's own 0.1 floor keeps — the up-drift the
/// case's `0.01 + nextDouble * 0.5` arg rides. The case's own draws
/// the caller's stream spends (the `f6`/`f7`/`d10` trio the walk's
/// own order keeps) — the velocities arrive PRE-fold (the `d8/d10/
/// d12` the case computed), the power fold the helper applies.
pub fn dragon_breath_splash_puff(
    x: f64,
    y: f64,
    z: f64,
    power: f32,
    vx: f64,
    vy: f64,
    vz: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the dead base lifetime (the draw the stream keeps, the value
    // the dragon ctor's own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    // the SingleQuadParticle's own size draw (the 0.1..0.2 the ctor's
    // own ×0.75 fold rides)
    let r_quad = rng.next_f32();
    // THE TINT DRAWS (the three Mth.nextFloat ranges the ctor's own
    // body speaks — the green's (0, 0) range the draw CONSUMES, the
    // 0.0 the zero-width answer keeps)
    let r = 0.7176470756530762f32 + rng.next_f32() * (0.8745098114013672f32 - 0.7176470756530762f32);
    let _g_draw = rng.next_f32();
    let b = 0.8235294222831726f32 + rng.next_f32() * (0.9764705896377563f32 - 0.8235294222831726f32);
    let r_life = rng.next_f32();
    Particle {
        kind: ParticleKind::DragonBreath,
        x,
        y,
        z,
        prev: None,
        // THE SET-POWER FOLD (the provider's own tail)
        vx: vx * f64::from(power),
        vy: (vy - 0.1) * f64::from(power) + 0.1,
        vz: vz * f64::from(power),
        age: 0,
        lifetime: dragon_breath_lifetime(r_life),
        gravity: 0.0,
        friction: 0.96,
        quad_size: base_quad_size(r_quad) * 0.75,
        uo: 0.0,
        vo: 0.0,
        col: [r, 0.0, b],
    }
}

/// The WATER-EVAPORATING steam puff's own spawn (javap
/// `PlayerCloudParticle` + `$Provider.createParticle` over the 26.2
/// jar): the ctor chain the case's own draw walk feeds — the dead
/// base lifetime draw (the value the cloud ctor's own write
/// overrides), the SingleQuadParticle's size draw (the 0.1..0.2 the
/// ctor's `* 1.875` fold rides), THE TINT DRAW the ctor's own body
/// speaks (the ONE `nextFloat` the f16 body spends — the shared
/// `1.0F - r * 0.3F` the r/g/b putfield trio keeps, the near-white
/// the steam nights wear), the lifetime the f64-domain fold answers
/// (17..65 — the `cloud_lifetime` helper the ×2.5 tail pins), the
/// friction the ctor's own 0.96 write, the no-gravity field the
/// base tick's own lane keeps silent, the hasPhysics-false
/// no-collision move the plain translate answers, the zero velocity
/// the case's `dconst_0` trio passes (the ×0.1 fold the zero keeps
/// zero, the + input the same). The caller's own position draws the
/// case's stream spends (the x/z nextDouble pair the walk's own
/// order keeps) — the pose arrives PRE-folded (the +1.2 lift the
/// case's constant adds).
pub fn cloud_puff(x: f64, y: f64, z: f64, rng: &mut crate::drops::Lcg) -> Particle {
    // the dead base lifetime (the draw the stream keeps, the value
    // the cloud ctor's own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    // the SingleQuadParticle's own size draw (the 0.1..0.2 the ctor's
    // own ×1.875 fold rides)
    let r_quad = rng.next_f32();
    // THE TINT DRAW (the ONE nextFloat the ctor's f16 body spends —
    // the shared answer the r/g/b trio keeps)
    let r_tint = rng.next_f32();
    let r_life = rng.next_f32();
    let tint = 1.0f32 - r_tint * 0.30000001192092896f32;
    Particle {
        kind: ParticleKind::Cloud,
        x,
        y,
        z,
        prev: None,
        // the zero velocity the case's dconst_0 trio passes
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        age: 0,
        lifetime: cloud_lifetime(r_life),
        gravity: 0.0,
        friction: 0.96,
        quad_size: base_quad_size(r_quad) * 1.875,
        uo: 0.0,
        vo: 0.0,
        col: [tint, tint, tint],
    }
}

/// The DRAGON BLOCK-BREAK burst's own spawn (javap
/// `HugeExplosionParticle` + `$Provider.createParticle` over the
/// 26.2 jar): the ctor chain the case's own draw walk feeds — the
/// dead base lifetime draw (the value the explosion ctor's own
/// lifetime write overrides), the SingleQuadParticle's size draw
/// (the value the ctor's own `2.0F * (1.0F - (float)xd * 0.5F)`
/// write overrides — the case's zero speed the 2.0 keeps), the
/// `6 + nextInt(4)` lifetime draw the 6..9 band speaks, THE TINT
/// DRAW the ctor's own body speaks (the ONE `nextFloat` the f11
/// body spends — the `* 0.6F + 0.4F` fold the r/g/b putfield trio
/// keeps, the grey the blast nights wear), the zero velocity the
/// case's `dconst_0` trio passes (the provider drops the y/z speeds
/// the walk never reads), the stationary tick the override's own
/// body answers (the prev-trio + the age walk + the removal gate —
/// no motion, no friction lane, no gravity the override never
/// reads), the OPAQUE layer the getLayer override returns. The
/// caller's own position the case's cp `#197` Double fold speaks —
/// the 0.5 centers arrive PRE-folded. The `size_speed` word the
/// provider's own fourth double keeps (the `dload 8` the ctor's
/// d2f fold reads — the dragon case's zero the 2.0 full quad
/// answers, the gateway seed's `age / lifetime` walk the children's
/// shrink rides — the `2.0 * (1.0 - xa * 0.5)` law the write
/// speaks).
pub fn explosion_puff(
    x: f64,
    y: f64,
    z: f64,
    size_speed: f32,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the dead base lifetime (the draw the stream keeps, the value
    // the explosion ctor's own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    // the SingleQuadParticle's own size draw (the value the ctor's
    // own quadSize write overrides — the case's zero speed the
    // 2.0 fold keeps)
    let _dead_quad = base_quad_size(rng.next_f32());
    // THE LIFETIME DRAW (the `6 + nextInt(4)` the ctor's own write
    // speaks — the 6..9 band the burst's brief nights wear)
    let extra = rng.next_int(4);
    // THE TINT DRAW (the ONE nextFloat the ctor's f11 body spends —
    // the shared answer the r/g/b trio keeps)
    let r_tint = rng.next_f32();
    let tint = r_tint * 0.6f32 + 0.4f32;
    Particle {
        kind: ParticleKind::Explosion,
        x,
        y,
        z,
        prev: None,
        // the zero velocity the case's dconst_0 trio passes
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        age: 0,
        lifetime: 6 + extra.max(0) as u32,
        gravity: 0.0,
        // the vanilla field default the override never reads (the
        // stationary walk the tick body keeps)
        friction: 0.98,
        // the ctor's own write — the `size_speed` word the shrink
        // rides (the zero the 2.0 full quad answers)
        quad_size: 2.0f32 * (1.0f32 - size_speed * 0.5f32),
        uo: 0.0,
        vo: 0.0,
        col: [tint, tint, tint],
    }
}

/// One sonic boom ring — javap `SonicBoomParticle` 26.2 verbatim:
/// the `HugeExplosionParticle` chain the base ctor's walk serves (the
/// dconst_0 velocity trio the base's own args keep — the motion the
/// sendParticles count-1 walk's zero speeds speak), the `lifetime =
/// 16` the ctor's own putfield overrides (the base's `6 +
/// nextInt(4)` draw the override retires — NO draw the fixed 16
/// spends), the `quadSize = 1.5f` the ctor's own write keeps (the
/// base's 6.0 the override retires, the dead base draw the caller
/// never needs — the caller's ONE grey draw the base ctor's f11 walk
/// speaks), the grey tint `nextFloat() * 0.6f + 0.4f` the r/g/b
/// ONE draw shares, the OPAQUE layer the getLayer override returns,
/// the no-fade tick the age walk keeps. The caller's own position
/// the lambda$tick$2 walk plants (the beam walk's exact points, the
/// sendParticles count-1 zero-offset the spread never scatters).
pub fn sonic_boom_ring(x: f64, y: f64, z: f64, grey: f32) -> Particle {
    Particle {
        kind: ParticleKind::SonicBoom,
        x,
        y,
        z,
        prev: None,
        // the zero velocity the base ctor's dconst_0 trio passes
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        age: 0,
        // the ctor's own putfield — the fixed 16 the override keeps
        lifetime: 16,
        gravity: 0.0,
        // the vanilla field default the stationary tick never reads
        friction: 0.98,
        // the ctor's own write — the 1.5f the ldc keeps
        quad_size: 1.5f32,
        uo: 0.0,
        vo: 0.0,
        // the base ctor's grey draw — the ONE answer the trio keeps
        col: [grey, grey, grey],
    }
}

/// The gateway spawn's own invisible seed — the
/// `HugeExplosionSeedParticle.<init>` verbatim (javap over the 26.2
/// jar): the `NoRenderParticle` base the x/y/z pose rides (the zero
/// velocity trio the seed's own addAlwaysVisibleParticle call
/// carries, the NOTHING the quad lanes bank), the 8-tick countdown
/// the ctor's own `lifetime = 8` write keeps. The render match the
/// honest absence speaks — the seed renders NOTHING, the children
/// the tick walk spawns do the visible work.
pub fn gateway_explosion_seed(x: f64, y: f64, z: f64) -> Particle {
    Particle {
        kind: ParticleKind::ExplosionSeed,
        x,
        y,
        z,
        prev: None,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        age: 0,
        // the ctor's own write — the 48-child burst the eight
        // ticks spend
        lifetime: 8,
        gravity: 0.0,
        // the vanilla field default the NoRender walk never reads
        friction: 0.98,
        // the invisible engine — no quad the lanes bank
        quad_size: 0.0,
        uo: 0.0,
        vo: 0.0,
        col: [0.0, 0.0, 0.0],
    }
}

/// THE EXPLOSION'S OWN FLASH (javap `ClientPacketListener
/// .handleExplosion` pc 100-125 the 26.2 jar — the addParticle walk
/// the packet's own particle the center's exact coords serves: the
/// `explosionParticle()` read the `(1.0, 0.0, 0.0)` velocity trio
/// answers [the speed the flash's own ctor never reads — the
/// dconst_0 trio the base walk keeps], the option the
/// `ServerLevel.explode` pc 165 `isSmall()` fork packs). THE FORK'S
/// OWN VERDICT (javap `ServerExplosion.isSmall` pc 0-21 — the
/// `radius >= 2.0F && interactsWithBlocks()` pair the fcmpg/iflt
/// fold arms: the LARGE blast the EXPLOSION_EMITTER wears, the small
/// blast the SINGLE HugeExplosion flash answers). THE OFFLINE FORK
/// the blast's own block walk the verdict keeps (every offline boom
/// the to_blow walk serves — the interactsWithBlocks arm the true
/// answer keeps, the radius compare the fork's own law wears): the
/// large arm the [`gateway_explosion_seed`] emitter proxy banks
/// (the invisible engine the six children per tick the same walk
/// serves), the small arm the [`explosion_puff`] full quad the
/// age-0 size-speed wears (the `2.0 * (1.0 - 0.0 * 0.5)` fold the
/// 2.0 answers).
pub fn explosion_flash(
    x: f64,
    y: f64,
    z: f64,
    radius: f32,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    if radius >= 2.0 {
        gateway_explosion_seed(x, y, z)
    } else {
        explosion_puff(x, y, z, 0.0, rng)
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

/// The ENDER-EYE debris shard's own spawn (javap `BreakingItemParticle`
/// over the 26.2 jar — the `ItemParticleOption(ParticleTypes.ITEM,
/// Items.ENDER_EYE)` the case-2003 arm's `bipush 8` loop speaks): the
/// SAME class and the SAME draw chain the splash potion's shard rides
/// (the nine draws the ctor chain consumes on the LEVEL stream in the
/// vanilla order — the base lifetime, the three spreads, the d0 pair,
/// the size draw, the uo/vo pair — then the 7-arg ctor's
/// `xd = xd * 0.1 + xSpeed` fold the wire velocity rides, the gravity
/// 1.0, the `/2` size fold, the base 0.98 friction). The ONE sprite
/// the eye's own model keeps: the provider's `getSprite` walk the
/// `pickParticleMaterial`'s `nextInt(activeLayerCount)` fold answers —
/// activeLayerCount 1 the single-layer stanza banks — the draw the
/// caller's stream STILL spends (the zero answer the bound-1 pick
/// keeps, the stream the alignment law guards).
#[allow(clippy::too_many_arguments)]
pub fn breaking_eye_shard(
    x: f64,
    y: f64,
    z: f64,
    wire_vx: f64,
    wire_vy: f64,
    wire_vz: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // THE SPRITE PICK (the `nextInt(1)` the single-layer model's
    // activeLayerCount speaks — the draw the stream spends, the zero
    // answer the pick keeps)
    let _layer = rng.next_int(1);
    let life = base_lifetime(rng.next_f32());
    let (sx, sy, sz) = ctor_spread_and_normalize(rng);
    let r_size = rng.next_f32();
    let uo = rng.next_f32() * 3.0;
    let vo = rng.next_f32() * 3.0;
    let scale = f64::from(0.1f32);
    Particle {
        kind: ParticleKind::EyeItem,
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

/// The thrown egg's own shatter (javap `ClientLevel.handleEntityEvent`
/// case 3 — the `spawnItemCrackParticles` walk the ThrownEgg's own
/// discard broadcasts): the SAME `BreakingItemParticle` chain the
/// eye's shard wears — the `bipush 10` loop the caller speaks, the
/// `ItemParticleOption(ParticleTypes.ITEM, stack)` the egg's own
/// sprite wears, the ONE sprite the egg's model keeps (the
/// `nextInt(1)` draw the stream spends), the 9-draw ctor chain the
/// eye's own walk banks (the base lifetime, the three spreads, the d0
/// pair, the size draw, the uo/vo pair — then the 7-arg ctor's
/// `xd = xd * 0.1 + xSpeed` fold the wire velocity rides, the gravity
/// 1.0, the `/2` size fold, the base 0.98 friction). The wire
/// velocity the caller's own gaussian trio carries (the
/// `nextGaussian() * 0.02` shard spread the event walk speaks).
#[allow(clippy::too_many_arguments)]
pub fn breaking_egg_shard(
    x: f64,
    y: f64,
    z: f64,
    wire_vx: f64,
    wire_vy: f64,
    wire_vz: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // THE SPRITE PICK (the `nextInt(1)` the single-layer model's
    // activeLayerCount speaks — the draw the stream spends, the zero
    // answer the pick keeps)
    let _layer = rng.next_int(1);
    let life = base_lifetime(rng.next_f32());
    let (sx, sy, sz) = ctor_spread_and_normalize(rng);
    let r_size = rng.next_f32();
    let uo = rng.next_f32() * 3.0;
    let vo = rng.next_f32() * 3.0;
    let scale = f64::from(0.1f32);
    Particle {
        kind: ParticleKind::EggItem,
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

/// The portal ring puff's own spawn (javap `PortalParticle` +
/// `$Provider.createParticle` over the 26.2 jar — the
/// `addParticle(ParticleTypes.PORTAL, ...)` the case-2003 arm's ring
/// walk fires): the provider's own `SpriteSet.get(random)` pick the
/// `nextInt(8)` draw answers FIRST (the random STATIC frame the
/// ascending portal.json keeps — no setSpriteFromAge the class never
/// speaks), then the ctor's own three draws — the quadSize the
/// `0.1F * (nextFloat * 0.2F + 0.5F)` write answers (NOT the base's
/// own `* 0.5 * 2.0` chain the single particle class folds), the tint
/// draw the `f = nextFloat * 0.6F + 0.4F` body spends (the r `* 0.9`
/// / g `* 0.3` / b plain putfield trio the purple keeps), the
/// lifetime the `(int)(nextFloat * 10.0F) + 40` write gives. The
/// velocity the addParticle args carry VERBATIM (the ctor's plain
/// field stores — no spread, no scale the recompute-from-Start swirl
/// never folds), the Start the ctor's own xStart/yStart/zStart write
/// keeps (the exact positions the swirl recompute demands). The base
/// friction field the vanilla default keeps (the tick override the
/// field never reads).
#[allow(clippy::too_many_arguments)]
pub fn portal_ring_puff(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // THE FRAME PICK (the provider's SpriteSet.get(random) — the
    // nextInt(8) draw the eight-frame set consumes)
    let frame = rng.next_int(8) as usize;
    // THE SIZE DRAW (the ctor's own write — the 0.05..0.07 band the
    // 0.1 × (r·0.2 + 0.5) fold gives)
    let r_quad = rng.next_f32();
    // THE TINT DRAW (the ONE nextFloat the ctor's f15 body spends —
    // the f = r·0.6 + 0.4 the r/g/b trio re-reads)
    let r_tint = rng.next_f32();
    // THE LIFETIME DRAW (the (int)(r · 10.0) + 40 write — the
    // 40..49 band the ring's long nights wear)
    let r_life = rng.next_f32();
    let tint = r_tint * 0.6f32 + 0.4f32;
    Particle {
        kind: ParticleKind::Portal { frame, sx: x, sy: y, sz: z },
        x,
        y,
        z,
        prev: None,
        // the verbatim velocity args the ctor's plain stores keep
        vx,
        vy,
        vz,
        age: 0,
        lifetime: (r_life * 10.0f32) as u32 + 40,
        gravity: 0.0,
        // the vanilla field default the swirl's recompute never reads
        friction: 0.98,
        quad_size: 0.1f32 * (r_quad * 0.2f32 + 0.5f32),
        uo: 0.0,
        vo: 0.0,
        col: [tint * 0.9f32, tint * 0.3f32, tint],
    }
}

/// The `PortalParticle.tick`'s own blend (javap the tick body's f1
/// fold verbatim: `t = (float)age / (float)lifetime` the NEW age the
/// putfield-first order keeps, then `f1 = -t + (t * t) * 2.0F` the
/// dadd fold speaks, then `f1 = 1.0F - f1` — the 1 − 2t² + t curve
/// the ring's pull-in rides: the blend opens at 1.0 (the birth
/// position Start + full velocity), holds ~1.0 through the middle
/// swing, closes at 0.0 (the particle lands ON its Start)).
pub fn portal_swirl_blend(t: f32) -> f32 {
    1.0f32 - (-t + (t * t) * 2.0f32)
}

/// The `PortalParticle.getQuadSize` override's own fold (javap
/// verbatim: `t = ((float)age + partial) / (float)lifetime`, `f =
/// 1.0F - t`, `f *= f`, `f = 1.0F - f` — the 1 − (1 − t)² growth the
/// ring swells by: the quad starts a point, reaches the full size at
/// the life's end, the pull-in the swirl's own closing rides).
pub fn portal_grow(t: f32) -> f32 {
    let f = 1.0f32 - t;
    1.0f32 - f * f
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

/// The trial spawner's own FLAME (javap
/// `TrialSpawner.addSpawnParticles`'s second `Level.addParticle` walk
/// verbatim — the decoded flame type the `FlameParticle.decode(data)
/// .particleType` arg carries, at the walk's own position with the
/// `dconst_0` trio the zero velocity args speak): the FULL chain the
/// provider wears — the plain `FlameParticle$Provider` chain BOTH
/// words wear (the CORRECTIVE BYTE AUDIT the fresh
/// `registerProviders` dump banks: the OMINOUS word — data 1, the
/// `ParticleTypes.SOUL_FIRE_FLAME` the clinit's second banks — rides
/// the IDENTICAL invokedynamic call site the NORMAL word — data 0,
/// `ParticleTypes.FLAME` — rides, indy cp#125 the bootstrap arg
/// cp#1085 the `FlameParticle$Provider.<init>` methodref answers;
/// the half-quad claim the round-454 docs spoke the raw operands
/// correct — the `scale(0.5F)` tail belongs to
/// `FlameParticle$SmallFlameProvider`, the `SMALL_FLAME` type's OWN
/// provider the eject walk's flame answers). The chain the
/// torch_flame bank pins (the FIFTEEN draws in the vanilla order:
/// the dead sprite pick — `SpriteSet.get(random)` the one-texture
/// json answers, nextInt(1) the draw the pick always spends — the
/// dead base lifetime, the three spread draws, the d0 pair, the
/// size draw, the six jitter draws, the own lifetime), the *0.01
/// velocity fold the ctor's own walk keeps (the +0.01 yd rise the
/// bump scales to; the ZERO args the spawn walk's dconst_0 trio
/// passes the fold's arg half keeps), the 0.96 friction, the no
/// gravity, the white tint, the quadSize the FULL birth band the
/// plain provider keeps (NO scale the provider's own walk spends).
fn trial_flame(
    x: f64,
    y: f64,
    z: f64,
    soul: bool,
    rng: &mut crate::drops::Lcg,
) -> Particle {
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
        kind: if soul {
            ParticleKind::SoulFlame
        } else {
            ParticleKind::Flame
        },
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

/// The trial spawner's own SPAWN BURST (javap
/// `TrialSpawner.addSpawnParticles` verbatim over the 26.2 jar —
/// `TrialSpawner$FlameParticle.decode(data).particleType` the
/// LevelEventHandler case 3011's invokestatic arg carries): the
/// `bipush 20` loop the bytecode's own count speaks (pc 0-121,
/// len=124), the THREE `RandomSource.nextDouble()` draws per
/// iteration the position triple carries (`pos.getX() + 0.5 +
/// (nextDouble() - 0.5) * 2.0` per axis — the f64 domain the
/// dsub/dmul constants speak, the ±1.0 spread the block-center
/// anchor keeps), the TWO `Level.addParticle` calls share the SAME
/// position (the SMOKE companion the `getstatic ParticleTypes.SMOKE`
/// always fires FIRST — pc 88-101 — then the decoded flame type pc
/// 104-115 passes) with the ZERO velocity the `dconst_0` trio
/// speaks. The chains the providers' own walks bank: the SMOKE the
/// `torch_smoke` chain verbatim (the NINE draws — the dead base
/// lifetime, the five-draw spread+normalize pair, the size, the
/// tint, the own lifetime — the *0.1 velocity scale the ash ctor's
/// xdScale args speak), the flame the `trial_flame` chain (the
/// FIFTEEN draws the OMINOUS half-scale the soul flag arms). TWENTY
/// SEVEN draws per iteration the stream consumes, FORTY particles
/// the burst answers (twenty smoke + twenty flame), NO sound the
/// case speaks (the 3012/3021 twins the voice lanes bank — the
/// next rounds' words).
pub fn trial_spawner_spawn_burst(
    x: i32,
    y: i32,
    z: i32,
    data: i32,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    // the decode the jar's own bytecodes pin (`id > 2 || id < 0` →
    // NORMAL — the values[2] edge the vanilla crash wears, the
    // NORMAL intent the honest arm keeps; `1` the OMINOUS word the
    // soul flame answers)
    let soul = data == 1;
    let mut out = Vec::with_capacity(40);
    for _ in 0..20 {
        let px = f64::from(x) + 0.5 + (rng.next_f64() - 0.5) * 2.0;
        let py = f64::from(y) + 0.5 + (rng.next_f64() - 0.5) * 2.0;
        let pz = f64::from(z) + 0.5 + (rng.next_f64() - 0.5) * 2.0;
        // the SMOKE companion (the zero velocity args the walk speaks)
        out.push(torch_smoke(px, py, pz, rng));
        // the decoded flame (the same position, the zero velocity)
        out.push(trial_flame(px, py, pz, soul, rng));
    }
    out
}

/// The eject walk's own SMALL FLAME (javap
/// `TrialSpawner.addEjectItemParticles`'s first `Level.addParticle`
/// verbatim — the `ParticleTypes.SMALL_FLAME` getstatic the pc 111
/// banks, at the walk's own position with the walk's own velocity
/// args): the FULL `FlameParticle` ctor chain the plain provider
/// wears (the FIFTEEN draws the torch-flame bank pins in the
/// vanilla order: the dead sprite pick — `SpriteSet.get(random)` the
/// one-texture `small_flame.json` list answers, nextInt(1) the draw
/// the pick always spends — the dead base lifetime, the three
/// spread draws, the d0 pair, the size draw, the six jitter draws,
/// the own lifetime), the `FlameParticle$SmallFlameProvider`'s ONE
/// `scale(0.5F)` tail the birth quad halves (the cp#447
/// registration the fresh audit pins, NO draw the scale ever
/// spends), the `xd = xd * 0.01D + xdArg` fold the RisingParticle
/// ctor's own pc 26-66 speaks — the WALK'S OWN ARG velocities the
/// addParticle args carry enter FULL scale (`ax, ay, az` the
/// gaussian * 0.02 trio gives; the flame's arg z the `0.25 dmul`
/// pc 126-129 folds — the quarter-pull the fold speaks), the 0.96
/// friction, the no gravity, the white tint, the OPAQUE layer.
fn eject_flame(
    x: f64,
    y: f64,
    z: f64,
    ax: f64,
    ay: f64,
    az: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the dead sprite pick (nextInt(1) the one-element list answers)
    let _dead_sprite = rng.next_int(1);
    // the dead base lifetime (the draw the stream keeps, the value
    // the RisingParticle ctor's own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    let (vx, vy, vz) = ctor_spread_and_normalize(rng);
    let quad = base_quad_size(rng.next_f32()) * 0.5;
    let fold = f64::from(0.01f32);
    let jx = ((rng.next_f32() - rng.next_f32()) * 0.05f32) as f64;
    let jy = ((rng.next_f32() - rng.next_f32()) * 0.05f32) as f64;
    let jz = ((rng.next_f32() - rng.next_f32()) * 0.05f32) as f64;
    Particle {
        kind: ParticleKind::SmallFlame,
        x: x + jx,
        y: y + jy,
        z: z + jz,
        prev: None,
        vx: vx * fold + ax,
        vy: vy * fold + ay,
        vz: vz * fold + az * 0.25,
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

/// The eject walk's own SMOKE (javap
/// `TrialSpawner.addEjectItemParticles`'s second `Level.addParticle`
/// verbatim — the `ParticleTypes.SMOKE` getstatic the pc 134 banks,
/// at the walk's own position with the walk's own velocity args the
/// RAW scale the pair rides): the SAME `SmokeParticle` ctor chain
/// the torch_smoke bank pins (the NINE draws — the dead base
/// lifetime, the five-draw spread+normalize pair, the size, the
/// tint, the own lifetime — the *0.1 velocity scale the
/// BaseAshSmokeParticle's xdScale args speak), the WALK'S OWN ARG
/// velocities the addParticle args carry enter FULL scale (the
/// `arg + spread * 0.1` fold the ash ctor's own walk speaks — the
/// +0.01 yd bump the normalization added keeps its place beside the
/// arg), the grey tint the r*0.3 draw carries, the 0.75 size fold,
/// the 8..40 lifetime the smoke_puff helper speaks.
fn eject_smoke(
    x: f64,
    y: f64,
    z: f64,
    ax: f64,
    ay: f64,
    az: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the dead base lifetime (the draw the stream keeps, the value
    // the ash ctor's own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    let (vx, vy, vz) = ctor_spread_and_normalize(rng);
    let scale = f64::from(0.1f32);
    smoke_puff(
        x,
        y,
        z,
        vx * scale + ax,
        vy * scale + ay,
        vz * scale + az,
        rng.next_f32(),
        rng.next_f32(),
        rng.next_f32(),
    )
}

/// The trial spawner's own EJECT BURST (javap
/// `TrialSpawner.addEjectItemParticles` verbatim over the 26.2 jar —
/// the `bipush 20` loop the iconst_0/if_icmpge pair answers
/// (len=159), the per-iteration SIX draws the walk itself spends:
/// the THREE `nextDouble()` position spreads (`pos.getX() + 0.4 +
/// nextDouble() * 0.2` per axis — the +0.4 anchor the ldc2w 0.4
/// carries, the [0.4, 0.6] band the dmul 0.2 folds), the THREE
/// `nextGaussian()` velocity draws (`* 0.02` per axis the dmul
/// 0.02 trio speaks), then the TWO `Level.addParticle` calls share
/// the SAME position + velocity args — the SMALL_FLAME the first
/// getstatic pc 111 banks (the zd arg the `0.25 dmul` fold scales),
/// the SMOKE the pc 134 banks (the raw args). The provider chains
/// the fresh audit banks: the flame the `eject_flame` chain (the
/// FIFTEEN draws + the scale(0.5F) half-quad), the smoke the
/// `eject_smoke` chain (the NINE draws + the arg velocities). THIRTY
/// draws per iteration the stream consumes, FORTY particles the
/// burst answers (twenty small flames + twenty smoke), NO sound the
/// walk speaks (the 3014 voice word the case body's own playLocalSound
/// lane banks — the sound twin the 3017 silent word never arms).
pub fn trial_eject_item_burst(x: i32, y: i32, z: i32, rng: &mut crate::drops::Lcg) -> Vec<Particle> {
    let mut out = Vec::with_capacity(40);
    for _ in 0..20 {
        let px = f64::from(x) + 0.4 + rng.next_f64() * 0.2;
        let py = f64::from(y) + 0.4 + rng.next_f64() * 0.2;
        let pz = f64::from(z) + 0.4 + rng.next_f64() * 0.2;
        let ax = rng.next_gaussian() * 0.02;
        let ay = rng.next_gaussian() * 0.02;
        let az = rng.next_gaussian() * 0.02;
        // the SMALL_FLAME companion (the zd * 0.25 fold the walk speaks)
        out.push(eject_flame(px, py, pz, ax, ay, az, rng));
        // the SMOKE companion (the raw args the pair rides)
        out.push(eject_smoke(px, py, pz, ax, ay, az, rng));
    }
    out
}

/// The vault walk's own FLAME (javap the `VaultBlockEntity$Client`
/// walks' flame `Level.addParticle` verbatim — the gate arg the
/// 3015/3016 case bodies pass, `data != 0 ? SOUL_FIRE_FLAME :
/// SMALL_FLAME` the `iload_3` + `ifne` pair answers): the chain the
/// provider registry banks — the SMALL_FLAME the
/// `FlameParticle$SmallFlameProvider` chain wears (the FIFTEEN draws
/// + the `scale(0.5F)` half-quad, the `xd = xd * 0.01D + xdArg` fold
/// the arg velocities ride at FULL scale — NO `0.25` z-fold the
/// eject walk's own arg-scaling keeps, the vault walks' raw args the
/// bytecodes never scale), the SOUL_FIRE_FLAME the plain
/// `FlameParticle$Provider` chain wears (the FULL quad the identical
/// indy call site the corrective audit banks, the SAME fifteen
/// draws). The zero-arg activation the fold's arg half keeps (the
/// `dconst_0` trio the walk's own addParticle args speak), the
/// gaussian deactivation the `* 0.02` args the `new Vec3` ctor
/// carries ride the same fold.
fn vault_flame(
    x: f64,
    y: f64,
    z: f64,
    small: bool,
    ax: f64,
    ay: f64,
    az: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the dead sprite pick (nextInt(1) the one-element list answers)
    let _dead_sprite = rng.next_int(1);
    // the dead base lifetime (the draw the stream keeps, the value
    // the RisingParticle ctor's own lifetime write overrides)
    let _dead_base_life = base_lifetime(rng.next_f32());
    let (vx, vy, vz) = ctor_spread_and_normalize(rng);
    let quad = if small {
        base_quad_size(rng.next_f32()) * 0.5
    } else {
        base_quad_size(rng.next_f32())
    };
    let fold = f64::from(0.01f32);
    let jx = ((rng.next_f32() - rng.next_f32()) * 0.05f32) as f64;
    let jy = ((rng.next_f32() - rng.next_f32()) * 0.05f32) as f64;
    let jz = ((rng.next_f32() - rng.next_f32()) * 0.05f32) as f64;
    Particle {
        kind: if small {
            ParticleKind::SmallFlame
        } else {
            ParticleKind::SoulFlame
        },
        x: x + jx,
        y: y + jy,
        z: z + jz,
        prev: None,
        vx: vx * fold + ax,
        vy: vy * fold + ay,
        vz: vz * fold + az,
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

/// The vault walk's own SMOKE (javap `VaultBlockEntity$Client
/// .emitActivationParticles`'s first `Level.addParticle` verbatim —
/// the `ParticleTypes.SMOKE` getstatic pc 32 banks, at the cage
/// position the `randomPosInsideCage` walk gives with the ZERO
/// velocity args the `dconst_0` trio speaks): the SAME `SmokeParticle`
/// ctor chain the eject smoke bank pins (the NINE draws — the dead
/// base lifetime, the five-draw spread+normalize pair, the size, the
/// tint, the own lifetime — the `arg + spread * 0.1` fold the zero
/// args keep), the grey tint the r*0.3 draw carries, the 0.75 size
/// fold, the 8..40 lifetime the smoke_puff helper speaks.
fn vault_smoke(x: f64, y: f64, z: f64, rng: &mut crate::drops::Lcg) -> Particle {
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

/// The vault's own ACTIVATION BURST (javap
/// `VaultBlockEntity$Client.emitActivationParticles` verbatim over
/// the 26.2 jar — len=87, the 3015 case's own invokestatic): the
/// connection tail the client's own BE state answers (the
/// `getConnectedPlayers()` set the `isEmpty()` early-return speaks —
/// the client's BE-less registry the EMPTY set the no-op's own
/// return banks, NO draws the tail ever spends), the `bipush 20`
/// loop the per-iteration walk spends: the THREE `Mth.nextDouble`
/// position draws the `randomPosInsideCage(pos, random)` walk
/// carries (the x/z bands `0.1 + nd * 0.8`, the y band
/// `0.25 + nd * 0.5` — the [0.1, 0.9] + [0.25, 0.75] cages the
/// ldc2w quartet speaks), the TWO `Level.addParticle` calls share
/// the SAME position with the ZERO velocity args — the SMOKE the
/// first getstatic pc 32 banks, the flame ARG the second `aload [4]`
/// carries (the gate's own pick the caller passes). The chains the
/// providers' own walks bank: the smoke the `vault_smoke` chain
/// (the NINE draws), the flame the `vault_flame` chain (the FIFTEEN
/// draws + the gate's own half-or-full quad). TWENTY-SEVEN draws per
/// iteration the stream consumes, FORTY particles the burst answers
/// (twenty smoke + twenty flame, the smoke the pair's FIRST member
/// the walk's own call order keeps — the eject walk's flame-first
/// order the vault's own walk MIRRORS).
pub fn vault_activation_burst(
    x: i32,
    y: i32,
    z: i32,
    small: bool,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::with_capacity(40);
    for _ in 0..20 {
        let px = f64::from(x) + 0.1 + rng.next_f64() * 0.8;
        let py = f64::from(y) + 0.25 + rng.next_f64() * 0.5;
        let pz = f64::from(z) + 0.1 + rng.next_f64() * 0.8;
        // the SMOKE companion (the zero args the dconst_0 trio speaks)
        out.push(vault_smoke(px, py, pz, rng));
        // the flame ARG (the gate's own pick, the zero args the pair rides)
        out.push(vault_flame(px, py, pz, small, 0.0, 0.0, 0.0, rng));
    }
    out
}

/// The vault's own DEACTIVATION BURST (javap
/// `VaultBlockEntity$Client.emitDeactivationParticles` verbatim over
/// the 26.2 jar — len=103, the 3016 case's own invokestatic): the
/// `bipush 20` loop the per-iteration walk spends: the THREE
/// `Mth.nextDouble(random, 0.4, 0.6)` position draws the
/// `randomPosCenterOfCage(pos, random)` walk carries (the
/// `0.4 + nd * 0.2` band the cage CENTER the tight span keeps), the
/// THREE `nextGaussian() * 0.02` velocity draws the `new Vec3(...)`
/// ctor carries, the ONE `Level.addParticle` call the flame ARG
/// spends (NO smoke companion the activation's own pair keeps). The
/// flame the `vault_flame` chain wears (the FIFTEEN draws + the
/// gate's own half-or-full quad, the gaussian args the `xd * 0.01D +
/// xdArg` fold rides at FULL scale). TWENTY-ONE draws per iteration
/// the stream consumes, TWENTY particles the burst answers.
pub fn vault_deactivation_burst(
    x: i32,
    y: i32,
    z: i32,
    small: bool,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::with_capacity(20);
    for _ in 0..20 {
        let px = f64::from(x) + 0.4 + rng.next_f64() * 0.2;
        let py = f64::from(y) + 0.4 + rng.next_f64() * 0.2;
        let pz = f64::from(z) + 0.4 + rng.next_f64() * 0.2;
        let ax = rng.next_gaussian() * 0.02;
        let ay = rng.next_gaussian() * 0.02;
        let az = rng.next_gaussian() * 0.02;
        out.push(vault_flame(px, py, pz, small, ax, ay, az, rng));
    }
    out
}

/// The BECOME-OMINOUS puff's own spawn (javap `SpellParticle` + the
/// plain `SpellParticle$Provider` verbatim over the 26.2 jar — the
/// `TRIAL_OMEN` type the become walk's FIRST getstatic banks, the
/// bootstrap arg the indy cp#253 resolves): the pass-through
/// createParticle the provider speaks (NO setColor — the WHITE
/// [1, 1, 1] tint the SingleQuadParticle defaults keep, NO power
/// fold the potion ring's own tail spends), the ctor chain verbatim:
/// the super-arg pair the STATIC `SpellParticle.RANDOM` stream
/// spends (`0.5 - nextDouble()` twice — the x/z velocity args the
/// walk's own gaussians NEVER ride, the static stream the separate
/// Lcg answers), the base ctor's dead lifetime + five-draw
/// spread+normalize fold (the walk's gaussian yd the ONLY arg the
/// super call carries), the yd `* 0.2` field fold, the zero-args
/// arm the dcmp pair guards (the ARGS the condition reads, the
/// FIELDS the `* 0.1` fold scales), the size ×0.75 fold, the
/// `(int)(8.0 / (nextFloat() * 0.8F + 0.2F))` lifetime. The
/// isCloseToScopingPlayer alpha fade the render lane does not model
/// (the client IS the scoping player — the fade-in the vanilla
/// render speaks rides the alpha the quad sampler never banks; the
/// golden frames' static scenes the lane never arms this kind in).
fn become_ominous_puff(
    x: f64,
    y: f64,
    z: f64,
    vx_arg: f64,
    vy_arg: f64,
    vz_arg: f64,
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
    let (mut vx, mut vy, mut vz) = ctor_spread_and_normalize_after(jx, vy_arg, jz, rng);
    let r_size = rng.next_f32();
    // the ctor's own write (the second draw the override keeps)
    let lifetime = spell_lifetime(rng.next_f32());
    // the ctor's yd fold
    vy *= f64::from(0.2f32);
    // THE ZERO-SPEED ARM (the bytecode's own dcmp pair — the ARGS the
    // condition reads, the FIELDS the 0.1 fold scales)
    if vx_arg == 0.0 && vz_arg == 0.0 {
        vx *= 0.1;
        vz *= 0.1;
    }
    Particle {
        kind: ParticleKind::Spell { instant: false },
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
        col: [1.0, 1.0, 1.0],
    }
}

/// The trial-spawner's BECOME-OMINOUS BURST (javap
/// `TrialSpawner.addBecomeOminousParticles` verbatim over the 26.2
/// jar — len=167, the 3020 case's own tail invokestatic): the
/// `bipush 20` loop the per-iteration walk spends: the THREE
/// `nextDouble()` position draws (`pos.getX() + 0.5 +
/// (nextDouble() - 0.5) * 2.0` per axis — the [−0.5, +1.5] band the
/// 2.0F scale folds, the CENTER the +0.5 anchor places), the THREE
/// `nextGaussian() * 0.02` velocity draws the dmul chain carries,
/// and the TWO `Level.addParticle` calls the SAME position +
/// velocity args share (the dload quartet both calls re-read): the
/// TRIAL_OMEN puff FIRST (the `become_ominous_puff` chain — the
/// static jitter stream the super-arg pair spends), the
/// SOUL_FIRE_FLAME the second getstatic speaks (the `vault_flame`
/// FULL-quad chain — the gaussian args the `xd * 0.01D + xdArg` fold
/// rides). TWENTY-NINE level draws per iteration the stream
/// consumes (6 walk + 8 puff + 15 flame, the TWO static draws the
/// jitter stream answers each iteration), FORTY particles the burst
/// answers (twenty omen + twenty soul, the omen-first order the
/// case's own call pair keeps).
pub fn become_ominous_burst(
    x: i32,
    y: i32,
    z: i32,
    rng: &mut crate::drops::Lcg,
    jitter: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::with_capacity(40);
    for _ in 0..20 {
        let px = f64::from(x) + 0.5 + (rng.next_f64() - 0.5) * 2.0;
        let py = f64::from(y) + 0.5 + (rng.next_f64() - 0.5) * 2.0;
        let pz = f64::from(z) + 0.5 + (rng.next_f64() - 0.5) * 2.0;
        let vx = rng.next_gaussian() * 0.02;
        let vy = rng.next_gaussian() * 0.02;
        let vz = rng.next_gaussian() * 0.02;
        // the TRIAL_OMEN puff (the FIRST getstatic pc 123 banks)
        out.push(become_ominous_puff(px, py, pz, vx, vy, vz, rng, jitter));
        // the SOUL_FIRE_FLAME (the SECOND getstatic pc 142 speaks —
        // the same position + velocity args the dload quartet rides)
        out.push(vault_flame(px, py, pz, false, vx, vy, vz, rng));
    }
    out
}

/// THE COBWEB-SPAWN BURST'S OWN WALK (javap `LevelEventHandler` case
/// 3018 — offset 2367 — verbatim): the `bipush 10` loop the
/// ten-member answer wears — NOT the family's twenty the trial
/// loops carry. Per iteration the SIX level draws the case's own
/// order keeps: the THREE `nextGaussian() * 0.02` velocity draws
/// FIRST (the `dstore 6/8/10` trio the velocity args carry), THEN
/// the THREE `nextDouble()` position draws the `addParticle` arg
/// walk interleaves — `pos.getX() + nextDouble()` per axis, the
/// `[0, 1)` CORNER band the bare nextDouble keeps (NO +0.5 center
/// the family's other walks carry, NO spread scale the dmul never
/// arms — the raw corner-anchored cube the vanilla client answers).
/// The ONE `addParticle(POOF, ...)` call the same six args share;
/// the POOF provider chain's own EIGHT draws the ctor bank rides
/// (the base lifetime float, the THREE jitter floats the raw
/// gaussians widen, the grey float, the size's TWO, the lifetime
/// float — the death-cloud round's own law). The caller's word:
/// `anchor` the PLACED cobweb's own cell (the
/// `WeavingMobEffect.spawnCobwebsRandomlyAround` loop the per-block
/// event fires after the `setBlock(COBWEB, 3)` walk).
pub fn cobweb_spawn_burst(anchor: (i32, i32, i32), rng: &mut crate::drops::Lcg) -> Vec<Particle> {
    let mut out = Vec::with_capacity(10);
    for _ in 0..10 {
        // the THREE GAUSSIAN VELOCITY DRAWS FIRST (the dstore trio
        // the case body's own pc 2377-2414 order keeps)
        let vx = rng.next_gaussian() * 0.02;
        let vy = rng.next_gaussian() * 0.02;
        let vz = rng.next_gaussian() * 0.02;
        // the THREE POSITION DRAWS the arg walk interleaves (the
        // corner band the bare nextDouble keeps — pos + [0, 1))
        let px = f64::from(anchor.0) + rng.next_f64();
        let py = f64::from(anchor.1) + rng.next_f64();
        let pz = f64::from(anchor.2) + rng.next_f64();
        // the POOF provider chain's own eight draws (the death
        // cloud's ctor bank the same provider wears)
        let r_base_life = rng.next_f32();
        let r_jx = rng.next_f32();
        let r_jy = rng.next_f32();
        let r_jz = rng.next_f32();
        let r_grey = rng.next_f32();
        let r_s1 = rng.next_f32();
        let r_s2 = rng.next_f32();
        let r_life = rng.next_f32();
        out.push(poof(
            px, py, pz, vx, vy, vz, r_base_life, r_jx, r_jy, r_jz, r_grey, r_s1, r_s2, r_life,
        ));
    }
    out
}

/// THE FIREWORK'S OWN EXPLOSION POOF (javap `ClientLevel.createFireworks`
/// the 26.2 bytecode's EMPTY-explosions branch verbatim — the plain
/// rocket's own answer the `handleEntityEvent(17)` walk calls): the
/// loop bound `2 + level.random.nextInt(3)` the RE-ROLLED head keeps
/// (pcs 13-27 the condition rolls a FRESH `nextInt(3)` each iteration
/// — the Java for-loop's own re-evaluation the bytecode speaks, the
/// count the first crossing's own roll answers — the observed band
/// 2..=4 the shared shape wears), the POOF particle the `addParticle
/// (POOF, x, y, z, nextGaussian() * 0.05, 0.005, nextGaussian() *
/// 0.05)` base velocity rides (pcs 30-67 — the FIXED 0.005 y the
/// bytecode's own f64 keeps, the ±0.05 gaussian band the f32
/// promotion speaks, the motion args the
/// caller passes the walk never reads), the POOF provider chain's own
/// eight draws the ctor bank spends. NO sound the empty branch speaks
/// (the blast the Starter's non-empty branch alone banks — the plain
/// rocket's own silence the same walk keeps), no damage the power-0
/// no-op wears (the `dealExplosionDamage`'s `5.0 + size * 2` the
/// empty list never arms). The (x, y, z) the createFireworks anchor
/// carries — the rocket's own post-tick pose the caller passes.
pub fn firework_poof_burst(x: f64, y: f64, z: f64, rng: &mut crate::drops::Lcg) -> Vec<Particle> {
    let mut out = Vec::new();
    let mut i = 0u32;
    loop {
        // THE RE-ROLLED BOUND (the bytecode's own loop head — the
        // fresh `nextInt(3)` each iteration the condition speaks)
        if i >= 2u32.saturating_add(rng.next_int(3) as u32) {
            break;
        }
        // THE BASE VELOCITY (pc 38-67 the draw order keeps: the
        // gaussian x FIRST, the fixed y, the gaussian z)
        let vx = rng.next_gaussian() * 0.05;
        let vy = 0.005;
        let vz = rng.next_gaussian() * 0.05;
        // the POOF provider chain's own eight draws (the death
        // cloud's ctor bank the same provider wears)
        let r_base_life = rng.next_f32();
        let r_jx = rng.next_f32();
        let r_jy = rng.next_f32();
        let r_jz = rng.next_f32();
        let r_grey = rng.next_f32();
        let r_s1 = rng.next_f32();
        let r_s2 = rng.next_f32();
        let r_life = rng.next_f32();
        out.push(poof(
            x, y, z, vx, vy, vz, r_base_life, r_jx, r_jy, r_jz, r_grey, r_s1, r_s2, r_life,
        ));
        i += 1;
    }
    out
}

/// The firework's TRAIL spark (javap `FireworkParticles$SparkProvider.
/// createParticle` + `SparkParticle.<init>` the 26.2 bytecode speaks —
/// the FIREWORK particle type the rocket's tick spawns every flight
/// tick): the ctor chain's own THREE particle-stream draws (the base
/// `Particle` ctor's lifetime float FIRST — the 4/(r*0.9+0.1) fold
/// the 48+nextInt(12) re-roll overwrites, the draw the stream still
/// spends; the `SingleQuadParticle` ctor's size float SECOND — the
/// `0.1 * (r * 0.5 + 0.5) * 2.0` fold; the `SparkParticle` ctor's
/// `nextInt(12)` THIRD — the `48 + draw` lifetime banks), the
/// friction the SimpleAnimatedParticle's #13 0.91F carries, the
/// gravity the super's own float arg 0.1F keeps, the quad the ×0.75F
/// fold the ctor's #28 speaks, the alpha the provider's setAlpha
/// 0.99F banks, the white (1,1,1) the base color fields never tint.
/// The motion args the CALLER passes (the entity stream's gaussian
/// pair the tick's own walk spends — the -vy*0.5 fold the y keeps).
pub fn firework_spark(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the base ctor's own lifetime float (the draw the re-roll
    // overwrites — the stream the fold still spends)
    let r_base_life = rng.next_f32();
    let _ = base_lifetime(r_base_life);
    // the SingleQuadParticle ctor's own size float (the ×0.75 fold
    // the SparkParticle ctor speaks after the chain)
    let r_size = rng.next_f32();
    let quad = base_quad_size(r_size) * 0.75;
    // the SparkParticle ctor's own lifetime re-roll
    let life_draw = rng.next_int(12);
    Particle {
        kind: ParticleKind::Spark {
            trail: false,
            twinkle: false,
            fade: None,
        },
        x,
        y,
        z,
        prev: None,
        vx,
        vy,
        vz,
        age: 0,
        lifetime: 48 + life_draw.max(0) as u32,
        gravity: 0.1,
        friction: 0.91,
        quad_size: quad,
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
}

/// THE TRAIL CHILD'S OWN CTOR (javap `SparkParticle.tick` pcs 38-72
/// the `new SparkParticle(level, x, y, z, 0, 0, 0, engine, sprites)`
/// walk answers — the SAME ctor chain the flight spark keeps: the
/// base lifetime float, the size float ×0.75, the 48 + nextInt(12)
/// re-roll — then the stores: `setAlpha(0.99F)` the ldc #85 speaks,
/// `setColor(parent's r/g/b)`, the `age = lifetime / 2` pin pcs
/// 96-102 bank, the fade + twinkle the parent's own bits ride, the
/// trail bit the child ctor never sets). The (x, y, z) the parent's
/// own post-tick pose carries, the motion the (0, 0, 0) args give.
pub fn firework_spark_trail_child(
    x: f64,
    y: f64,
    z: f64,
    col: [f32; 3],
    fade: Option<[f32; 3]>,
    twinkle: bool,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    let r_base_life = rng.next_f32();
    let _ = base_lifetime(r_base_life);
    let r_size = rng.next_f32();
    let quad = base_quad_size(r_size) * 0.75;
    let life_draw = rng.next_int(12);
    let lifetime = 48 + life_draw.max(0) as u32;
    Particle {
        kind: ParticleKind::Spark {
            trail: false,
            twinkle,
            fade,
        },
        x,
        y,
        z,
        prev: None,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        age: lifetime / 2,
        lifetime,
        gravity: 0.1,
        friction: 0.91,
        quad_size: quad,
        uo: 0.0,
        vo: 0.0,
        col,
    }
}

/// The SimpleAnimatedParticle's own ARGB decode (javap `setColor(I)`
/// pcs 0-53 verbatim: `((c >> 16) & 0xFF) / 255F` the red lane, `((c
/// >> 8) & 0xFF) / 255F` the green, `(c & 0xFF) / 255F` the blue —
/// the ×1.0 alpha the fconst_1 multiplies, the standard decode both
/// `setColor` and `setFadeColor` speak).
pub fn spark_color_decode(c: i32) -> [f32; 3] {
    [
        ((c >> 16) & 0xFF) as f32 / 255.0,
        ((c >> 8) & 0xFF) as f32 / 255.0,
        (c & 0xFF) as f32 / 255.0,
    ]
}

/// THE STARTER'S OWN EXPLOSION SPARK (javap
/// `FireworkParticles$Starter.createParticle` the 26.2 walk verbatim
/// — the `engine.createParticle(ParticleTypes.FIREWORK, x, y, z, vx,
/// vy, vz)` the SparkParticle cast answers, then the four stores:
/// `setTrail(trail)`, `setTwinkle(twinkle)`, `setAlpha(0.99F)`,
/// `setColor(Util.getRandom(colors, random))` and — the fade list
/// non-empty alone — `setFadeColor(Util.getRandom(fadeColors,
/// random))`). The DRAW ORDER the walk speaks: the ctor chain's own
/// THREE particle-stream draws FIRST (the base lifetime float, the
/// size float, the 48 + nextInt(12) re-roll), the random PICKS ride
/// AFTER (the LEVEL's own stream — `this.random` the shared
/// RandomSource the Starter speaks; `Util.getRandom` =
/// `list.get(random.nextInt(size))` — one draw per non-empty list).
/// The (x, y, z, vx, vy, vz) the shape walk passes (the explosion's
/// own walk answers).
pub fn firework_explosion_spark(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    colors: &[i32],
    fade_colors: &[i32],
    trail: bool,
    twinkle: bool,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the base ctor's own lifetime float (the draw the re-roll
    // overwrites — the stream the fold still spends)
    let r_base_life = rng.next_f32();
    let _ = base_lifetime(r_base_life);
    // the SingleQuadParticle ctor's own size float (the ×0.75 fold
    // the SparkParticle ctor speaks after the chain)
    let r_size = rng.next_f32();
    let quad = base_quad_size(r_size) * 0.75;
    // the SparkParticle ctor's own lifetime re-roll
    let life_draw = rng.next_int(12);
    // THE COLOR PICK (the setColor store's Util.getRandom walk — the
    // one nextInt(len) draw the non-empty list spends; the BLACK
    // fallback the caller arms keeps the list non-empty)
    let color = colors[rng.next_int(colors.len() as u64) as usize % colors.len()];
    // THE FADE PICK (the setFadeColor store's own walk — the
    // non-empty fade list the store's isEmpty gate answers)
    let fade = if fade_colors.is_empty() {
        None
    } else {
        Some(spark_color_decode(
            fade_colors[rng.next_int(fade_colors.len() as u64) as usize % fade_colors.len()],
        ))
    };
    Particle {
        kind: ParticleKind::Spark {
            trail,
            twinkle,
            fade,
        },
        x,
        y,
        z,
        prev: None,
        vx,
        vy,
        vz,
        age: 0,
        lifetime: 48 + life_draw.max(0) as u32,
        gravity: 0.1,
        friction: 0.91,
        quad_size: quad,
        uo: 0.0,
        vo: 0.0,
        col: spark_color_decode(color),
    }
}

/// THE STARTER'S OWN FLASH (javap `Starter.tick` pcs 395-425 the
/// `engine.createParticle(ColorParticleOption.create(FLASH,
/// colors[0]), x, y, z, 0, 0, 0)` walk answers — the
/// `FlashProvider.createParticle` the OverlayParticle ctor chain
/// speaks: the sprite pick `sprites.get(random)` the ONE nextInt(1)
/// draw spends (the flash.json's single-texture set the bound-1 roll
/// answers), the lifetime the ctor's `iconst_4` pins, the color the
/// option's decode gives, the alpha the `ARGB.alpha(color) / 255`
/// law keeps (the dye colors' zero byte the verbatim law stores —
/// the vanilla render itself paints nothing for them)). No motion,
/// no gravity the OverlayParticle ctor never arms.
pub fn firework_flash(x: f64, y: f64, z: f64, color: i32, rng: &mut crate::drops::Lcg) -> Particle {
    // the sprite pick's own draw (the bound-1 roll the single-frame
    // set spends — `sprites.get(random)` the nextInt(1) answers)
    let _sprite_roll = rng.next_int(1);
    Particle {
        kind: ParticleKind::Flash {
            alpha: ((color as u32) >> 24) as f32 / 255.0,
        },
        x,
        y,
        z,
        prev: None,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        age: 0,
        lifetime: 4,
        gravity: 0.0,
        friction: 1.0,
        quad_size: 7.1,
        uo: 0.0,
        vo: 0.0,
        col: spark_color_decode(color),
    }
}

/// The FLASH's own quad bell (javap `OverlayParticle.getQuadSize`
/// verbatim: `7.1F × sin((age + partial − 1) × 0.25F × π)` — the
/// growth-and-shrink arc the four-tick life wears, the negative
/// first-tick fold the spawn frame's own quirk the verbatim law
/// keeps; the SIZE the render's half() fold halves — the corner span
/// the ±size/2 convention keeps).
pub fn flash_bell(age: u32, partial: f32) -> f32 {
    7.1 * (((age as f32 + partial - 1.0) * 0.25 * std::f32::consts::PI).sin())
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
/// carried (the caller's word), the FOUR-draw provider chain the
/// bytecodes pin — the one-frame glint pick FIRST (the
/// `SpriteSet.get(random)` the happy_villager.json's one-texture set
/// consumes — the nextInt(1) the draw the pick always lands, the
/// SAME glint bank the composter puff's kind renders), the dead tint
/// draw NEXT (the base ctor's ONE `nextFloat() * 0.1 + 0.2` — the
/// fstore [15] the r/g/b putfield trio all loads, the
/// `setColor(1.0F, 1.0F, 1.0F)` the VALUE overrides, the draw the
/// stream still spends; THE CORRECTION the autopush-451 re-audit
/// banks — the prior reading spent TWO draws (life + size), the
/// glint pick and the dead tint the jar's own chain never skips),
/// the size draw (`0.02 * (nf * 0.6 + 0.5)` the ctor's own write
/// gives), the lifetime LAST (`20.0 / (nf * 0.8 + 0.2)` the ctor's
/// own write answers — the provider NO setLifetime speaks, the fold
/// the particle's REAL life keeps, the 21..100 band the denom band
/// wears). The zero velocity the aiStep's own (0, 0, 0) the ctor's
/// `* 0.02` fold keeps whole, the friction the tick's own `* 0.99`
/// speaks, the white tint the glint texture's own green passes.
pub fn happy_villager_sparkle(x: f64, y: f64, z: f64, rng: &mut crate::drops::Lcg) -> Particle {
    // the one-frame glint pick (the SpriteSet.get(random) draw —
    // nextInt(1) the one-entry list consumes, the pick always lands 0)
    let _glint_pick = rng.next_int(1);
    // the dead tint draw (the base ctor's ONE nextFloat — the fstore
    // [15] the r/g/b putfield trio all loads; the setColor(1,1,1)
    // override the VALUE keeps, the draw the stream still spends)
    let _dead_tint = rng.next_f32();
    // the size draw (the 0.02 * (nf * 0.6 + 0.5) fold the ctor's
    // own write speaks)
    let quad_size = 0.02 * (rng.next_f32() * 0.6 + 0.5);
    // the lifetime (the ctor's own 20.0 / (nf * 0.8 + 0.2) write —
    // the provider NO setLifetime speaks, the fold the REAL life keeps)
    let lifetime = (20.0 / (rng.next_f32() as f64 * 0.8 + 0.2)) as u32;
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

/// THE COMPOSTER-FILL PUFF'S OWN SPAWN (javap
/// `SuspendedTownParticle$ComposterFillProvider.createParticle` +
/// the `SuspendedTownParticle` ctor over the 26.2 jar — the
/// `ParticleTypes.COMPOSTER` word the `ComposterBlock.handleFill`
/// case-1500 arm's `bipush 10` loop speaks): the FULL provider chain
/// the FIVE draws the stream spends in the vanilla order — the
/// one-frame glint pick FIRST (`SpriteSet.get(random)` the
/// composter.json's one-texture set consumes — the nextInt(1) the
/// draw the pick always lands, the SAME glint bank the feed
/// sparkle's kind renders), the dead tint draw NEXT (the base
/// ctor's `nextFloat() * 0.1 + 0.2` — the ONE invoke the bytecodes
/// spend, the `fstore [15]` the r/g/b putfield trio all loads —
/// the `setColor(1.0F, 1.0F, 1.0F)` putfield trio the VALUE
/// overrides, the draw the stream still keeps; THE CORRECTION the
/// autopush-450 re-audit banks — the prior reading counted THREE
/// dead tints, the jar's own shared-local shape speaks ONE, the
/// fload [15] the three putfields keep), the size draw
/// (`quadSize = 0.02 * (nf * 0.6 + 0.5)` the setSize + the
/// getQuadSize draw the chain gives), the dead base lifetime
/// (`20.0 / (nf * 0.8 + 0.2)` the ctor's own write — the
/// `setLifetime(3 + nextInt(5))` the provider's override replaces,
/// the 3..7 band the `iconst_3` + `iconst_5` the bytecode pins).
/// The velocity the `* 0.02` ctor fold speaks (the case's own
/// `nextGaussian() * 0.02` args the fold scales twice — the
/// effective drift the `* 0.0004` the two folds keep, the same law
/// the growth burst's ride wears). The white tint, the 0.99
/// friction the tick's own walk keeps, the zero gravity, the
/// OPAQUE layer the getLayer override returns — the kind the glint
/// bank's own render arm carries.
pub fn composter_fill_puff(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    rng: &mut crate::drops::Lcg,
) -> Particle {
    // the one-frame glint pick (the SpriteSet.get(random) draw —
    // nextInt(1) the one-entry list consumes, the pick always lands 0)
    let _glint_pick = rng.next_int(1);
    // the dead tint draw (the base ctor's ONE nextFloat — the fstore
    // [15] the r/g/b putfield trio all loads; the setColor(1,1,1)
    // override the VALUE keeps, the draw the stream still spends)
    let _dead_tint = rng.next_f32();
    // the size draw (the 0.02 * (nf * 0.6 + 0.5) fold the ctor's
    // own write speaks)
    let quad_size = 0.02 * (rng.next_f32() * 0.6 + 0.5);
    // the dead base lifetime (the 20.0 / (nf * 0.8 + 0.2) fold the
    // ctor's own write keeps — the setLifetime the value replaces)
    let _dead_base_life = (20.0 / (rng.next_f32() as f64 * 0.8 + 0.2)) as u32;
    // the own lifetime (the setLifetime(3 + nextInt(5)) the
    // provider's override speaks — the 3..7 band the iconst_3 +
    // iconst_5 pins)
    let lifetime = (3 + rng.next_int(5)) as u32;
    // the velocity the * 0.02 ctor fold speaks (the case's own
    // gaussian trio the fold scales — the effective drift the two
    // folds keep)
    let scale = 0.02;
    Particle {
        kind: ParticleKind::HappyVillager,
        x,
        y,
        z,
        vx: vx * scale,
        vy: vy * scale,
        vz: vz * scale,
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

/// THE EGG-CRACK FACES WALK (javap `LevelEventHandler` case 3009 —
/// offset 2576 — plus `ParticleUtils.spawnParticlesOnBlockFaces` +
/// `spawnParticleOnFace` + `getRandomSpeedRanges` + the
/// `SuspendedTownParticle$EggCrackProvider.createParticle` chain over
/// the 26.2 jar): the ONE walk the case body spends — the SIX
/// `Direction.values()` faces the enum's own order banks (DOWN, UP,
/// NORTH, SOUTH, WEST, EAST), each face the
/// `UniformInt.of(3, 6).sample` count arms (the `3 + nextInt(4)` band
/// the randomBetweenInclusive law speaks — ONE draw the power-of-2
/// fold spends). THE PER-PARTICLE WALK the chain's own order pins:
/// the supplier FIRST (the `getRandomSpeedRanges` verbatim — the
/// THREE `Mth.nextDouble(-0.5, 0.5)` draws the Vec3 ctor args carry,
/// drawn WHOLE the ternary's own zeroing never refunds), the
/// position pair NEXT (the face axis rides `center + axis * 0.55`
/// — the deterministic arm NO draw spends — the perpendicular pair
/// the `Mth.nextDouble(-0.5, 0.5)` draws, X→Y→Z the axis order
/// keeps), the provider chain LAST (the glint pick the egg_crack
/// json's one-texture set consumes — the `nextInt(1)` the draw the
/// pick always lands, the dead tint draw the base ctor's r/g/b trio
/// shares ONE — the fstore [15] the bytecode pins, the setColor(1,
/// 1, 1) putfield trio the values override, the size draw the
/// `0.02 * (nf * 0.6 + 0.5)` fold the ctor's own write speaks, the
/// lifetime draw the `20.0 / (nf * 0.8 + 0.2)` fold answers — the
/// provider NO setLifetime speaks, the 21..100 band the fold keeps).
/// The velocity the IN-PLANE pair alone keeps (the ternary trio the
/// jar's own ifne arms speak: the face-normal axis's component
/// ZEROES), the `* 0.02` ctor fold the drift damps, the white tint,
/// the 0.99 friction the tick's own walk keeps, the zero gravity,
/// the OPAQUE layer the glint bank's own render arm carries.
pub fn egg_crack_faces(anchor: (i32, i32, i32), rng: &mut crate::drops::Lcg) -> Vec<Particle> {
    let mut out = Vec::new();
    let cx = anchor.0 as f64 + 0.5;
    let cy = anchor.1 as f64 + 0.5;
    let cz = anchor.2 as f64 + 0.5;
    // Direction.values()'s own order (the enum's wire the switch's
    // own array keep — DOWN, UP, NORTH, SOUTH, WEST, EAST)
    let faces: [(i32, i32, i32); 6] = [
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
        (-1, 0, 0),
        (1, 0, 0),
    ];
    for &(sx, sy, sz) in &faces {
        // THE PER-FACE COUNT (UniformInt.of(3, 6).sample — the
        // `3 + nextInt(4)` band the randomBetweenInclusive speaks)
        let per_face = 3 + rng.next_int(4).max(0);
        for _ in 0..per_face {
            // THE VELOCITY SUPPLIER (the `getRandomSpeedRanges`
            // verbatim — the THREE `Mth.nextDouble(-0.5, 0.5)` draws
            // the Vec3 ctor args keep, drawn WHOLE the ternary's own
            // zeroing never refunds)
            let vx = rng.next_f64_between(-0.5, 0.5);
            let vy = rng.next_f64_between(-0.5, 0.5);
            let vz = rng.next_f64_between(-0.5, 0.5);
            // THE POSITION PAIR (spawnParticleOnFace verbatim: the
            // face axis rides `center + axis * 0.55` — the
            // deterministic arm NO draw spends — the perpendicular
            // pair the `Mth.nextDouble(-0.5, 0.5)` draws, X→Y→Z the
            // axis order keeps)
            let px = cx
                + if sx != 0 {
                    f64::from(sx) * 0.55
                } else {
                    rng.next_f64_between(-0.5, 0.5)
                };
            let py = cy
                + if sy != 0 {
                    f64::from(sy) * 0.55
                } else {
                    rng.next_f64_between(-0.5, 0.5)
                };
            let pz = cz
                + if sz != 0 {
                    f64::from(sz) * 0.55
                } else {
                    rng.next_f64_between(-0.5, 0.5)
                };
            // THE PROVIDER CHAIN (the glint pick, the dead tint draw
            // the trio shares one, the size draw, the lifetime draw —
            // the provider NO setLifetime speaks)
            let _glint_pick = rng.next_int(1);
            let _dead_tint = rng.next_f32();
            let quad_size = 0.02 * (rng.next_f32() * 0.6 + 0.5);
            let lifetime = (20.0 / (rng.next_f32() as f64 * 0.8 + 0.2)) as u32;
            // THE IN-PLANE VELOCITY (the ternary trio the jar's own
            // ifne arms speak: the face-normal axis's component
            // ZEROES, the in-plane pair keeps the supplier's draws —
            // the `* 0.02` ctor fold the drift damps)
            let scale = 0.02;
            out.push(Particle {
                kind: ParticleKind::EggCrack,
                x: px,
                y: py,
                z: pz,
                vx: if sx == 0 { vx * scale } else { 0.0 },
                vy: if sy == 0 { vy * scale } else { 0.0 },
                vz: if sz == 0 { vz * scale } else { 0.0 },
                prev: None,
                age: 0,
                lifetime,
                gravity: 0.0,
                friction: 0.99,
                quad_size,
                uo: 0.0,
                vo: 0.0,
                col: [1.0, 1.0, 1.0],
            });
        }
    }
    out
}

/// THE SNOWFLAKE'S OWN LIFETIME LAW (javap `SnowflakeParticle` ctor
/// pc 136-161 over the 26.2 jar): `(int)(16.0 / (nextFloat() * 0.8D +
/// 0.2D)) + 2` — the same f64 fold the lava drip speaks, the +2 the
/// iconst tail adds. The f64 domain's own edges speak: r=0 draws
/// 79.99999940… → 79 + 2 = 81, r->1 draws 15.99999988… → 15 + 2 = 17
/// — the grid the nextFloat speaks never draws the exact edge, the
/// [18, 81] band the stream holds.
pub fn snowflake_lifetime(r: f32) -> u32 {
    let denom = r as f64 * f64::from(0.8f32) + f64::from(0.2f32);
    (16.0f64 / denom) as u32 + 2
}

/// THE SNOWFLAKE'S OWN SIZE LAW (javap `SnowflakeParticle` ctor
/// pc 106-133 verbatim): `0.1F * (nextFloat() * nextFloat() * 1.0F +
/// 1.0F)` — the TWO draws' PRODUCT the fold speaks, the base law's
/// own twin the DIFFERENT shape arms (`base_quad_size` folds
/// `r * 0.5 + 0.5`, one draw; the flake's product never reaches the
/// same middles the mirror would — the [0.1, 0.2] band the pair
/// gives).
pub fn snowflake_quad_size(r1: f32, r2: f32) -> f32 {
    0.1 * (r1 * r2 * 1.0 + 1.0)
}

/// THE TRIAL DETECT-PLAYER BURST'S OWN WALK (javap
/// `TrialSpawner.addDetectPlayerParticles` — len=126 — over the 26.2
/// jar, the `LevelEventHandler` case 3013's tail the voice rides
/// after): the loop bound `30 + Math.min(amount, 10) * 5` the bipush
/// 30/bipush 10/imul trio speaks (the DATA word the amount arg
/// carries — the detected player count the caller's set size gives,
/// the 10-cap the min arm keeps), the per-iteration THREE
/// `nextFloat()` draws the position pair + the height carry —
/// `dx = (nf * 2.0F - 1.0F) * 0.65D` and `dz` the same shape (the
/// ±0.65 spread the dmul 0.65 folds), `py = pos.getY() + 0.1 + nf *
/// 0.8D` (the 0.1..0.9 band the height fold keeps), the ZERO
/// velocity the `dconst_0` trio speaks. THE SNOWFLAKE CHAIN the
/// provider's own walks bank: the THREE velocity spreads
/// `(nf * 2 - 1) * 0.05` the ctor pc 34-55 speaks (x→y→z the order
/// keeps), the size `0.1 * (nf * nf + 1.0)` the TWO draws' product
/// answers, the lifetime the `(16.0 / (nf * 0.8 + 0.2)) + 2` fold
/// arms — NINE draws per iteration the walk exactly spends (three
/// position + three velocity + two size + one life), the near-white
/// bluish tint the `setColor(0.923, 0.964, 0.999)` constants carry
/// (NO draw the trio spends), the gravity 0.225 the ctor's field
/// keeps, the friction 1.0F the base law's own no-op arms, the OPAQUE
/// layer the flake's own render bank carries.
pub fn trial_detect_player_burst(
    anchor: (i32, i32, i32),
    amount: i32,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::new();
    let iterations = 30 + amount.min(10) * 5;
    for _ in 0..iterations {
        // THE POSITION PAIR + THE HEIGHT (the three nextFloat draws
        // the walk's own order keeps — dx, dz, then the height)
        let dx = (rng.next_f32() * 2.0 - 1.0) as f64 * 0.65;
        let dz = (rng.next_f32() * 2.0 - 1.0) as f64 * 0.65;
        let px = f64::from(anchor.0) + 0.5 + dx;
        let py = f64::from(anchor.1) + 0.1 + f64::from(rng.next_f32()) * 0.8;
        let pz = f64::from(anchor.2) + 0.5 + dz;
        // THE PROVIDER CHAIN (the six draws the ctor's bytecodes pin
        // — the velocity spreads x→y→z, the size's two-draw product,
        // the lifetime's f64 fold)
        let vx = (rng.next_f32() * 2.0 - 1.0) as f64 * 0.05;
        let vy = (rng.next_f32() * 2.0 - 1.0) as f64 * 0.05;
        let vz = (rng.next_f32() * 2.0 - 1.0) as f64 * 0.05;
        let r1 = rng.next_f32();
        let r2 = rng.next_f32();
        let quad_size = snowflake_quad_size(r1, r2);
        let lifetime = snowflake_lifetime(rng.next_f32());
        out.push(Particle {
            kind: ParticleKind::Snowflake,
            x: px,
            y: py,
            z: pz,
            vx,
            vy,
            vz,
            prev: None,
            age: 0,
            lifetime,
            gravity: 0.225,
            friction: 1.0,
            quad_size,
            uo: 0.0,
            vo: 0.0,
            col: [0.923, 0.964, 0.999],
        });
    }
    out
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
/// (`nextGaussian() * 0.02 * 0.02`). The FOUR-draw provider chain
/// (the glint pick, the dead tint, the size, the lifetime — THE
/// CORRECTION the autopush-451 re-audit banks, the same
/// [`happy_villager_sparkle`] law the feed sparkle shares) the
/// `$HappyVillagerProvider` sprite's own walk rides.
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

/// THE IN-BLOCK GROWTH BURST'S OWN WALK (javap
/// `ParticleUtils.spawnParticleInBlock` over the real 26.2 jar — the
/// LevelEventHandler's 2011/2012 cases the bee's growth word and the
/// turtle egg's placement word speak): the anchor cell's own state
/// the shape read opens — the `isAir` gate the `dconst_1` answer
/// keeps (an air cell the 1.0 speaks), the block's own shape the
/// Y-max fold rides (the wheat family the age ladder
/// `(age * 2 + 2) / 16` the growth arm's own law wears — the same
/// fold the 1505 case's grower lane keeps; the full cubes the
/// vocabulary banks the 1.0 the shape's own Y-max speaks — the
/// unbanked sub-cubes the honest stand-in keeps), then the
/// [`growth_burst`] walk the IN-BLOCK FORCE LANE arms (`0.5` the
/// spread pair's x, the shape's own Y the dy, `true` the force flag —
/// the ground gate the force lane never reads: the burst lives INSIDE
/// the anchor cell, the below-block read the vanilla delegate never
/// speaks). The count the caller's own data word carries (the bee's
/// `bipush 15`, the egg-lay's own `bipush 15`).
pub fn in_block_growth_burst(
    anchor: (i32, i32, i32),
    count: usize,
    world: &crate::world::World,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let cell = world.get(anchor.0, anchor.1, anchor.2);
    let dy = match crate::world::wheat_age(cell) {
        Some(age) => (age as f64 * 2.0 + 2.0) / 16.0,
        None => 1.0,
    };
    growth_burst(anchor, count, 0.5, dy, true, world, rng)
}

/// THE SMASH-ATTACK BURST'S OWN WALK (javap `ParticleUtils
/// .spawnSmashAttackParticles` over the real 26.2 jar — the
/// LevelEventHandler's 2013 case the mace's own knockback broadcasts):
/// the anchor fold `Vec3.atCenterOf(pos).add(0.0, 0.5, 0.0)` the
/// block's own TOP FACE carries (the y the never-drawn band keeps —
/// every particle hovers the anchor cell's own +1.0 plane), then the
/// TWO loops the bytecode pins: the in-block loop `(float)i <
/// (float)count / 3.0F` (the `nextDouble() / 2.0` x/z offsets the
/// in-block jitter keeps, the `* 0.2` speed trio the provider's own
/// override discards) and the RING loop `(float)i < (float)count /
/// 1.5F` (`3.5 * Math.cos(i)` / `3.5 * Math.sin(i)` the radius walk —
/// the index the RADIAN arg keeps, the vanilla's own unnormalized
/// spiral — the `* 0.05` trio the same override discards). Per
/// particle the terrain ctor chain the house fold replays (the life /
/// size / uo / vo draws the per-particle stream the vanilla keeps —
/// the one stream the replay folds), then the provider's own
/// overrides: `setParticleSpeed(nextDouble() / 30.0, ySpeed +
/// nextDouble() / 2.0, nextDouble() / 30.0)` (the caller's x/z speeds
/// the override DISCARDS, the y the `+ next/2` upward bias keeps) and
/// `setLifetime(20 + nextInt(20))` (the 20..39 band the base ctor's
/// own draw overrides). The `tile: Option<u16>` the caller's state
/// read carries — the AIR / MOVING_PISTON verdicts the
/// `createTerrainParticle` gate keeps and the unbanked kinds the
/// break lane's TILE_NONE verdict keeps all ride `None`: the caller's
/// own loop draws alone spend (the vanilla truth the level stream
/// never skips), the burst the lane never speaks.
pub fn smash_attack_burst(
    anchor: (i32, i32, i32),
    count: usize,
    tile: Option<u16>,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::new();
    let cx = anchor.0 as f64 + 0.5;
    let cy = anchor.1 as f64 + 1.0;
    let cz = anchor.2 as f64 + 0.5;
    // the in-block loop (the count's own third)
    let mut i: i64 = 0;
    while (i as f32) < count as f32 / 3.0 {
        let ox = rng.next_f64() / 2.0;
        let oz = rng.next_f64() / 2.0;
        let d3 = rng.next_f64() * 0.2;
        let d4 = rng.next_f64() * 0.2;
        let d5 = rng.next_f64() * 0.2;
        if let Some(tile) = tile {
            let r_life = rng.next_f32();
            let r_size = rng.next_f32();
            let r_uo = rng.next_f32();
            let r_vo = rng.next_f32();
            let sx = rng.next_f64() / 30.0;
            let sy = rng.next_f64() / 2.0;
            let sz = rng.next_f64() / 30.0;
            let r_life2 = rng.next_int(20);
            let mut p = terrain(
                tile,
                cx + ox,
                cy,
                cz + oz,
                d3,
                d4,
                d5,
                r_life,
                r_size,
                r_uo,
                r_vo,
            );
            // the provider's own override (the caller's x/z speeds the
            // discard rides, the y the +next/2 bias keeps)
            p.vx = sx;
            p.vy = d4 + sy;
            p.vz = sz;
            p.lifetime = 20 + r_life2.max(0) as u32;
            out.push(p);
        }
        i += 1;
    }
    // the ring loop (the count's own two-thirds — the index the
    // radian arg keeps)
    let mut j: i64 = 0;
    while (j as f32) < count as f32 / 1.5 {
        let ox = rng.next_f64() / 2.0;
        let oz = rng.next_f64() / 2.0;
        let d3 = rng.next_f64() * 0.05;
        let d4 = rng.next_f64() * 0.05;
        let d5 = rng.next_f64() * 0.05;
        if let Some(tile) = tile {
            let r_life = rng.next_f32();
            let r_size = rng.next_f32();
            let r_uo = rng.next_f32();
            let r_vo = rng.next_f32();
            let sx = rng.next_f64() / 30.0;
            let sy = rng.next_f64() / 2.0;
            let sz = rng.next_f64() / 30.0;
            let r_life2 = rng.next_int(20);
            let mut p = terrain(
                tile,
                cx + 3.5 * (j as f64).cos() + ox,
                cy,
                cz + 3.5 * (j as f64).sin() + oz,
                d3,
                d4,
                d5,
                r_life,
                r_size,
                r_uo,
                r_vo,
            );
            p.vx = sx;
            p.vy = d4 + sy;
            p.vz = sz;
            p.lifetime = 20 + r_life2.max(0) as u32;
            out.push(p);
        }
        j += 1;
    }
    out
}

/// The wax trio's own GlowParticle ctor (javap `GlowParticle.<init>` +
/// `$WaxOnProvider` / `$WaxOffProvider` / `$ScrapeProvider`
/// `.createParticle` verbatim): the base ctor's lifetime draw runs
/// DEAD (the provider's `setLifetime(nextInt(30) + 10)` write
/// overrides), the SingleQuadParticle size draw the ×0.75 fold keeps,
/// the 0.96 friction the ctor's own putfield speaks, the physics-free
/// fly the `hasPhysics = false` write rides, the gravity the base
/// default 0.0 keeps. NO spread draws the zero-speed ctor args speak
/// (the providers pass `dconst_0` x3 — the setParticleSpeed walk the
/// provider's own multiply chain keeps). The caller folds the tint
/// (the `setColor` rows — the scrape's `nextBoolean()` gate the
/// caller's own draw spends) and the 10..39 lifetime — the draw
/// ORDER the caller's walk keeps.
pub fn glow_puff(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    col: [f32; 3],
    r_life: f32,
    r_base: f32,
    lifetime: u32,
) -> Particle {
    // the dead base lifetime (the draw the stream keeps, the value
    // the provider's own lifetime write overrides)
    let _dead_base_life = base_lifetime(r_life);
    Particle {
        kind: ParticleKind::Glow,
        x,
        y,
        z,
        prev: None,
        vx,
        vy,
        vz,
        age: 0,
        lifetime,
        gravity: 0.0,
        friction: 0.96,
        quad_size: base_quad_size(r_base) * 0.75,
        uo: 0.0,
        vo: 0.0,
        col,
    }
}

/// The wax trio's own faces walk (javap `ParticleUtils
/// .spawnParticlesOnBlockFaces` -> `spawnParticlesOnBlockFace` ->
/// `spawnParticleOnFace` + the `getRandomSpeedRanges` supplier
/// verbatim — the `LevelEventHandler` 3003/3004/3005 cases' ONE
/// delegate each speaks): the SIX faces the `Direction.values()`
/// order keeps (DOWN, UP, NORTH, SOUTH, WEST, EAST), the per-face
/// count `3 + nextInt(3)` the `UniformInt.of(3, 5)` sample law speaks
/// (`randomBetweenInclusive`: `nextInt(max - min + 1) + min`), the
/// per-particle velocity supplier's THREE `Mth.nextDouble(-0.5, 0.5)`
/// draws first, then the position pair — the face axis rides
/// `center + axis * 0.55`, the perpendicular pair draws (exactly two
/// per face), then the provider chain: the dead base lifetime, the
/// size draw, the scrape's tint gate (`nextBoolean` the bit law
/// reads), the `nextInt(30) + 10` lifetime. The velocity the
/// IN-PLANE pair keeps (the face-normal axis's component the `sx !=
/// 0 → dconst_0` ifne arm zeroes — the 445 round's re-audit banked
/// the flip the earlier walk inverted), the provider's speed
/// folds the caller's velocity carries (`xSpeed * 0.01F / 2.0F` the
/// wax pair's halves law speaks, the PLAIN `* 0.01F` the scrape
/// keeps — the one constant's own difference the bytecodes pin).
/// NO sound the walker speaks — the 3003 voice the render lane's own
/// serve banks beside the burst.
pub fn wax_faces_burst(
    anchor: (i32, i32, i32),
    word: i32,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::new();
    // THE MODEL'S OWN ARM the three words alone banks — the unknown
    // word the switch's own silence keeps (no draws the neither-case
    // ever spends)
    if word != mc_protocol::PARTICLES_AND_SOUND_WAX_ON_EFFECT
        && word != mc_protocol::PARTICLES_WAX_OFF_EFFECT
        && word != mc_protocol::PARTICLES_SCRAPE_EFFECT
    {
        return out;
    }
    let cx = anchor.0 as f64 + 0.5;
    let cy = anchor.1 as f64 + 0.5;
    let cz = anchor.2 as f64 + 0.5;
    // Direction.values()'s own order (the enum's wire the switch's
    // own array keep — DOWN, UP, NORTH, SOUTH, WEST, EAST)
    let faces: [(i32, i32, i32); 6] = [
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
        (-1, 0, 0),
        (1, 0, 0),
    ];
    for &(sx, sy, sz) in &faces {
        // the per-face count (the IntProvider.sample the block face
        // walk spends once per face — `randomBetweenInclusive(3, 5)`)
        let count = 3 + rng.next_int(3).max(0);
        for _ in 0..count {
            // THE VELOCITY SUPPLIER (getRandomSpeedRanges verbatim:
            // `new Vec3(nextDouble(-0.5, 0.5), ...)`) — the three
            // draws the supplier's own ctor args keep)
            let vx = rng.next_f64_between(-0.5, 0.5);
            let vy = rng.next_f64_between(-0.5, 0.5);
            let vz = rng.next_f64_between(-0.5, 0.5);
            // THE POSITION PAIR (spawnParticleOnFace verbatim: the
            // face axis rides `center + axis * 0.55`, the
            // perpendicular pair the `Mth.nextDouble(-0.5, 0.5)`
            // draws — exactly two per face)
            let px = cx
                + if sx != 0 {
                    f64::from(sx) * 0.55
                } else {
                    rng.next_f64_between(-0.5, 0.5)
                };
            let py = cy
                + if sy != 0 {
                    f64::from(sy) * 0.55
                } else {
                    rng.next_f64_between(-0.5, 0.5)
                };
            let pz = cz
                + if sz != 0 {
                    f64::from(sz) * 0.55
                } else {
                    rng.next_f64_between(-0.5, 0.5)
                };
            // THE IN-PLANE VELOCITY (the ternary trio the jar's own
            // ifne arms speak — the 445 round's re-audit banks: the
            // face-normal axis's component ZEROES (`131: iload 8;
            // ifne → 142: dconst_0`), the in-plane pair keeps the
            // supplier's draws — the pulse slides ALONG the face)
            let x_speed = if sx == 0 { vx } else { 0.0 };
            let y_speed = if sy == 0 { vy } else { 0.0 };
            let z_speed = if sz == 0 { vz } else { 0.0 };
            // THE PROVIDER CHAIN (the draws the createParticle walk
            // spends — the base ctor pair, the scrape's tint gate,
            // the lifetime draw)
            let r_life = rng.next_f32();
            let r_base = rng.next_f32();
            let (col, halves) = match word {
                mc_protocol::PARTICLES_AND_SOUND_WAX_ON_EFFECT => ([0.91, 0.55, 0.08], true),
                mc_protocol::PARTICLES_WAX_OFF_EFFECT => ([1.0, 0.9, 1.0], true),
                mc_protocol::PARTICLES_SCRAPE_EFFECT => {
                    // the tint gate (javap: `nextBoolean(); ifeq` —
                    // the TRUE arm the teal speaks, the FALSE the
                    // lighter twin keeps)
                    let teal = rng.next_int(2) == 1;
                    (
                        if teal {
                            [0.29, 0.58, 0.51]
                        } else {
                            [0.43, 0.77, 0.62]
                        },
                        false,
                    )
                }
                _ => unreachable!("the model's own arm the three words alone banks"),
            };
            let lifetime = 10 + rng.next_int(30).max(0) as u32;
            // the provider's own speed folds (the wax pair's halves
            // law, the scrape's plain trio)
            let (fx, fy, fz) = if halves {
                (
                    (x_speed * 0.01) / 2.0,
                    y_speed * 0.01,
                    (z_speed * 0.01) / 2.0,
                )
            } else {
                (x_speed * 0.01, y_speed * 0.01, z_speed * 0.01)
            };
            out.push(glow_puff(
                px, py, pz, fx, fy, fz, col, r_life, r_base, lifetime,
            ));
        }
    }
    out
}

/// The electric-spark walk (javap `LevelEventHandler` case 3002 —
/// offset 2596 — over the 26.2 jar + `ParticleUtils
/// .spawnParticlesAlongAxis` verbatim): the DATA word the case's own
/// `iload_3` read splits — the `0..3` arm the
/// `Direction.Axis.VALUES[data]` pick banks (the X/Y/Z ordinals the
/// enum's own array keeps), the `data < 0 || data >= 3` arm the
/// faces walk the wax trio's own six-face law wears. THE ALONG-AXIS
/// WALK (the 0..3 arm): the count `10 + nextInt(10)` the
/// `UniformInt.of(10, 19).sample` law speaks
/// (`randomBetweenInclusive`), the per-child position trio the
/// `Mth.nextDouble(-1.0, 1.0)` draws carry — the CHOSEN axis the
/// `* 0.5` half-span multiplier rides (the `if_acmpne`/`0.5` pair
/// the bytecodes pin), the perpendicular pair the `* 0.125` edge
/// distance keeps (the `ldc2_w 0.125` the case's own call site
/// banks) — then the velocity trio the CHOSEN axis alone draws (the
/// `Mth.nextDouble(-1.0, 1.0)` the gated arms spend, the others the
/// `dconst_0` silence keeps). THE FACES WALK (the negative/overflow
/// arm): the SAME geometry the wax burst wears (the six
/// `Direction.values()` faces, the `3 + nextInt(3)` per-face count,
/// the face-axis `* 0.55` position pair, the IN-PLANE velocity the
/// ifne arm zeroes on the face-normal axis — the 445 round's
/// re-audit banked the flip the earlier walk inverted, the
/// supplier's three `Mth.nextDouble(-0.5, 0.5)` draws). THE PROVIDER
/// CHAIN both arms
/// share (javap `GlowParticle$ElectricSparkProvider.createParticle`
/// verbatim): the dead base lifetime draw, the SingleQuadParticle
/// size draw (the ×0.75 fold the GlowParticle's own ctor keeps),
/// the tint the `setColor(1.0F, 0.9F, 1.0F)` trio banks (NO draw —
/// the wax-off twin's own constants the pair shares), the speed the
/// `setParticleSpeed(xa * 0.25, ya * 0.25, za * 0.25)` multiply
/// chain folds, the `nextInt(2) + 2` lifetime the 2..3 band speaks.
/// NO sound the case speaks — the sparks alone the burst renders.
pub fn electric_spark_burst(
    anchor: (i32, i32, i32),
    data: i32,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::new();
    let cx = anchor.0 as f64 + 0.5;
    let cy = anchor.1 as f64 + 0.5;
    let cz = anchor.2 as f64 + 0.5;
    // the provider chain's own draws + fold (the createParticle walk
    // both arms spend — the closure keeps the draw ORDER the
    // caller's walk banks: base life, size, lifetime)
    let spark = |px: f64,
                     py: f64,
                     pz: f64,
                     sx: f64,
                     sy: f64,
                     sz: f64,
                     rng: &mut crate::drops::Lcg|
     -> Particle {
        let r_life = rng.next_f32();
        let r_base = rng.next_f32();
        let lifetime = 2 + rng.next_int(2).max(0);
        glow_puff(
            px,
            py,
            pz,
            sx * 0.25,
            sy * 0.25,
            sz * 0.25,
            [1.0, 0.9, 1.0],
            r_life,
            r_base,
            lifetime as u32,
        )
    };
    if (0..3).contains(&data) {
        // THE ALONG-AXIS WALK (the `Direction.Axis.VALUES[data]`
        // arm — X=0, Y=1, Z=2 the enum's own array order keeps)
        let bx = data == 0;
        let by = data == 1;
        let bz = data == 2;
        // THE COUNT (UniformInt.of(10, 19).sample — the
        // `10 + nextInt(10)` law the randomBetweenInclusive speaks)
        let count = 10 + rng.next_int(10).max(0);
        for _ in 0..count {
            // THE POSITION TRIO (the `Mth.nextDouble(-1.0, 1.0)`
            // draws — the chosen axis the `* 0.5` half-span rides,
            // the others the `* 0.125` edge distance keeps)
            let px = cx + rng.next_f64_between(-1.0, 1.0) * if bx { 0.5 } else { 0.125 };
            let py = cy + rng.next_f64_between(-1.0, 1.0) * if by { 0.5 } else { 0.125 };
            let pz = cz + rng.next_f64_between(-1.0, 1.0) * if bz { 0.5 } else { 0.125 };
            // THE VELOCITY TRIO (the chosen axis alone the
            // `Mth.nextDouble(-1.0, 1.0)` draw rides — the others
            // the `dconst_0` silence keeps, NO draw)
            let vx = if bx { rng.next_f64_between(-1.0, 1.0) } else { 0.0 };
            let vy = if by { rng.next_f64_between(-1.0, 1.0) } else { 0.0 };
            let vz = if bz { rng.next_f64_between(-1.0, 1.0) } else { 0.0 };
            out.push(spark(px, py, pz, vx, vy, vz, rng));
        }
    } else {
        // THE FACES WALK (the SAME law the wax burst wears — the
        // `spawnParticlesOnBlockFaces` delegate the negative and
        // overflow words arm)
        // Direction.values()'s own order (DOWN, UP, NORTH, SOUTH,
        // WEST, EAST)
        let faces: [(i32, i32, i32); 6] = [
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
            (-1, 0, 0),
            (1, 0, 0),
        ];
        for &(sx, sy, sz) in &faces {
            // the per-face count (the UniformInt.of(3, 5).sample
            // law the block face walk spends once per face)
            let count = 3 + rng.next_int(3).max(0);
            for _ in 0..count {
                // THE VELOCITY SUPPLIER (getRandomSpeedRanges
                // verbatim: the three `Mth.nextDouble(-0.5, 0.5)`
                // draws the Vec3 ctor args keep)
                let vx = rng.next_f64_between(-0.5, 0.5);
                let vy = rng.next_f64_between(-0.5, 0.5);
                let vz = rng.next_f64_between(-0.5, 0.5);
                // THE POSITION PAIR (spawnParticleOnFace verbatim:
                // the face axis rides `center + axis * 0.55`, the
                // perpendicular pair draws — exactly two per face)
                let px = cx
                    + if sx != 0 {
                        f64::from(sx) * 0.55
                    } else {
                        rng.next_f64_between(-0.5, 0.5)
                    };
                let py = cy
                    + if sy != 0 {
                        f64::from(sy) * 0.55
                    } else {
                        rng.next_f64_between(-0.5, 0.5)
                    };
                let pz = cz
                    + if sz != 0 {
                        f64::from(sz) * 0.55
                    } else {
                        rng.next_f64_between(-0.5, 0.5)
                    };
                // THE IN-PLANE VELOCITY (the ternary trio the jar's
                // own ifne arms speak — the 445 round's re-audit
                // banks: the face-normal axis's component ZEROES
                // (`131: iload 8; ifne → 142: dconst_0`), the
                // in-plane pair keeps the supplier's draws — the
                // pulse slides ALONG the face)
                let x_speed = if sx == 0 { vx } else { 0.0 };
                let y_speed = if sy == 0 { vy } else { 0.0 };
                let z_speed = if sz == 0 { vz } else { 0.0 };
                out.push(spark(px, py, pz, x_speed, y_speed, z_speed, rng));
            }
        }
    }
    out
}

/// One sculk-charge pulse (javap `SculkChargeParticle` ctor +
/// `$Provider.createParticle` verbatim): the base ctor's own draws
/// the stream keeps (the dead base lifetime the provider's write
/// overrides, the quad size the `scale(1.5F)` fold rides), the
/// provider's VERBATIM speed (the `setParticleSpeed(xs, ys, zs)`
/// call carries the caller's velocity unmultiplied — the tiny
/// supplier draws the ±0.005 bounds already speak), the static roll
/// the options' π|0 the `flipped` fold banks, the `nextInt(12) + 8`
/// lifetime the 8..19 band speaks, the 0.96 friction the ctor's own
/// putfield keeps, the white tint the ctor never speaks (no
/// `setColor` row the class bytes carry — the fullbright look the
/// `getLightCoords` withBlock(15) override answers, the honest
/// untinted full sample the lane draws).
fn sculk_charge_puff(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    flipped: bool,
    r_life: f32,
    r_size: f32,
    lifetime: u32,
) -> Particle {
    // the dead base lifetime (the draw the stream keeps, the value
    // the provider's own lifetime write overrides)
    let _dead_base_life = base_lifetime(r_life);
    Particle {
        kind: ParticleKind::SculkCharge { flipped },
        x,
        y,
        z,
        prev: None,
        vx,
        vy,
        vz,
        age: 0,
        lifetime,
        gravity: 0.0,
        friction: 0.96,
        quad_size: base_quad_size(r_size) * 1.5,
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
}

/// One sculk-charge pop (javap `SculkChargePopParticle` ctor +
/// `$Provider.createParticle` verbatim): the same base-ctor draw
/// pair, the VERBATIM ±0.07 cube speed the caller's trio carries,
/// the `nextInt(4) + 6` lifetime the 6..9 band speaks, the 0.96
/// friction, the white tint, the fullbright look.
fn sculk_pop_puff(
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    r_life: f32,
    r_size: f32,
    lifetime: u32,
) -> Particle {
    let _dead_base_life = base_lifetime(r_life);
    Particle {
        kind: ParticleKind::SculkChargePop,
        x,
        y,
        z,
        prev: None,
        vx,
        vy,
        vz,
        age: 0,
        lifetime,
        gravity: 0.0,
        friction: 0.96,
        quad_size: base_quad_size(r_size),
        uo: 0.0,
        vo: 0.0,
        col: [1.0, 1.0, 1.0],
    }
}

/// The sculk-charge face walk (javap `LevelEventHandler` case 3006 —
/// offset 2668 — the count > 0 arm over the 26.2 jar): the DATA word
/// the case's own `data >> 6` count the caller decodes, the
/// `packed = data & 63` byte the face split rides — the ZERO arm the
/// SIX `Direction.values()` faces walks (the DOWN face the π roll
/// picks, the axis spans 0.65 Y / 0.57 X-Z the case's own doubles
/// keep), the NON-ZERO arm the `MultifaceBlock.unpack` ordinal-order
/// set walks (the UP face the π roll picks, the 0.35 span the flat
/// 0.35 double keeps). THE PER-FACE COUNT the
/// `UniformInt.of(0, count).sample` law speaks
/// (`randomBetweenInclusive`: `nextInt(count + 1)`, the draw the
/// count >= 1 arm always spends), the per-particle walk the
/// `spawnParticlesOnBlockFace` → `spawnParticleOnFace` chain pins:
/// the supplier's THREE `Mth.nextDouble(-0.005, 0.005)` draws first,
/// then the position pair — the face axis rides `center + axis *
/// span` (deterministic, NO draw), the perpendicular pair the
/// `Mth.nextDouble(-0.5, 0.5)` draws (exactly two, X→Y→Z axis
/// order) — then the provider chain (the base ctor's dead life +
/// size draws, the `nextInt(12)` lifetime). THE VELOCITY the
/// `spawnParticleOnFace` ternary trio keeps: the face-normal axis's
/// component ZEROES (the `sx != 0 → dconst_0` arm the bytecode
/// speaks), the in-plane pair keeps the supplier's draws — the
/// pulse slides ALONG the face the way the jar's own ifne arms.
/// NO sound the walker speaks — the gated voice the render lane's
/// own serve banks beside the burst.
pub fn sculk_charge_faces(
    anchor: (i32, i32, i32),
    count: i32,
    packed_faces: i32,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::new();
    if count <= 0 {
        // the caller's own arm split (the `ifle` the case opens the
        // face walk with — the count <= 0 words the pop arm answers,
        // the faces walk never spends)
        return out;
    }
    let cx = anchor.0 as f64 + 0.5;
    let cy = anchor.1 as f64 + 0.5;
    let cz = anchor.2 as f64 + 0.5;
    // Direction.values()'s own order (the enum's wire the switch's
    // own array keep — DOWN, UP, NORTH, SOUTH, WEST, EAST)
    let faces: [(i32, i32, i32); 6] = [
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
        (-1, 0, 0),
        (1, 0, 0),
    ];
    for &(sx, sy, sz) in &faces {
        // the unpack arm's own filter (the EnumSet the
        // `MultifaceBlock.unpack` builds — bit i = the ordinal i's
        // face, the iteration order the enum order keeps; the ZERO
        // byte the filter never arms)
        if packed_faces != 0 && (packed_faces & (1 << face_ordinal(sx, sy, sz))) == 0 {
            continue;
        }
        // THE ROLL (the options' own pick — the zero arm the DOWN
        // face's π keeps, the unpack arm the UP face's π; the span
        // the same split rides)
        let down_face = sx == 0 && sy == -1 && sz == 0;
        let up_face = sx == 0 && sy == 1 && sz == 0;
        let flipped = if packed_faces == 0 {
            down_face
        } else {
            up_face
        };
        let span: f64 = if packed_faces == 0 {
            if sy != 0 {
                0.65
            } else {
                0.57
            }
        } else {
            0.35
        };
        // THE PER-FACE COUNT (UniformInt.of(0, count).sample — the
        // `nextInt(count + 1)` law the randomBetweenInclusive speaks)
        let per_face = rng.next_int_inclusive(0, count);
        for _ in 0..per_face.max(0) {
            // THE VELOCITY SUPPLIER (the `lambda$levelEvent$0`
            // verbatim — the THREE `Mth.nextDouble(-0.005, 0.005)`
            // draws the Vec3 ctor args keep, drawn WHOLE the
            // ternary's own zeroing never refunds)
            let vx = rng.next_f64_between(-0.005, 0.005);
            let vy = rng.next_f64_between(-0.005, 0.005);
            let vz = rng.next_f64_between(-0.005, 0.005);
            // THE POSITION PAIR (spawnParticleOnFace verbatim: the
            // face axis rides `center + axis * span` — the
            // deterministic arm NO draw spends — the perpendicular
            // pair the `Mth.nextDouble(-0.5, 0.5)` draws, X→Y→Z the
            // axis order keeps)
            let px = cx
                + if sx != 0 {
                    f64::from(sx) * span
                } else {
                    rng.next_f64_between(-0.5, 0.5)
                };
            let py = cy
                + if sy != 0 {
                    f64::from(sy) * span
                } else {
                    rng.next_f64_between(-0.5, 0.5)
                };
            let pz = cz
                + if sz != 0 {
                    f64::from(sz) * span
                } else {
                    rng.next_f64_between(-0.5, 0.5)
                };
            // THE IN-PLANE VELOCITY (the ternary trio the jar's own
            // ifne arms speak: the face-normal axis's component
            // ZEROES, the in-plane pair keeps the supplier's draws)
            let x_speed = if sx == 0 { vx } else { 0.0 };
            let y_speed = if sy == 0 { vy } else { 0.0 };
            let z_speed = if sz == 0 { vz } else { 0.0 };
            // THE PROVIDER CHAIN (the base ctor's dead life + size
            // draws, the `nextInt(12) + 8` lifetime the 8..19 band
            // carries)
            let r_life = rng.next_f32();
            let r_size = rng.next_f32();
            let lifetime = 8 + rng.next_int(12).max(0);
            out.push(sculk_charge_puff(
                px,
                py,
                pz,
                x_speed,
                y_speed,
                z_speed,
                flipped,
                r_life,
                r_size,
                lifetime as u32,
            ));
        }
    }
    out
}

/// The sculk-charge pop walk (javap `LevelEventHandler` case 3006 —
/// offset 2668 — the count <= 0 arm, THE VANILLA-CALLER ARM the
/// `SculkSpreader.updateCursors` `charge <= 0` gate feeds with the
/// data 0 word): the ONE fixed voice the serve's own walk banks, the
/// cube burst the `isCollisionShapeFullBlock` split keeps — the FULL
/// block the 40-pop / ±0.45-spread arm answers, the non-full the
/// 20-pop / ±0.25 arm keeps. THE PER-POP WALK the case's own loop
/// pins: the `2.0F * nextFloat() - 1.0F` trio (dx, dy, dz — the
/// axis order the bytecodes keep), the position trio `pos + 0.5 +
/// d * spread`, the speed trio `d * 0.07F`, then the provider chain
/// (the base ctor's dead life + size draws, the `nextInt(4) + 6`
/// lifetime the 6..9 band carries). The full-block verdict the
/// model's own arm carries (the state read the `block()` walk
/// answers — the collision law the client's registry approximation
/// keeps, see `world_model::collision_full_block`).
pub fn sculk_charge_pops(
    anchor: (i32, i32, i32),
    full_block: bool,
    rng: &mut crate::drops::Lcg,
) -> Vec<Particle> {
    let mut out = Vec::new();
    let (count, spread): (i32, f64) = if full_block { (40, 0.45) } else { (20, 0.25) };
    let cx = anchor.0 as f64 + 0.5;
    let cy = anchor.1 as f64 + 0.5;
    let cz = anchor.2 as f64 + 0.5;
    for _ in 0..count {
        // THE CUBE TRIO (the `2.0F * nextFloat() - 1.0F` draws — the
        // position AND speed fold both read, the axis order the
        // bytecodes keep)
        let dx = 2.0 * f64::from(rng.next_f32()) - 1.0;
        let dy = 2.0 * f64::from(rng.next_f32()) - 1.0;
        let dz = 2.0 * f64::from(rng.next_f32()) - 1.0;
        // THE PROVIDER CHAIN (the base ctor's dead life + size
        // draws, the `nextInt(4) + 6` lifetime the 6..9 band
        // carries)
        let r_life = rng.next_f32();
        let r_size = rng.next_f32();
        let lifetime = 6 + rng.next_int(4).max(0);
        out.push(sculk_pop_puff(
            cx + dx * spread,
            cy + dy * spread,
            cz + dz * spread,
            dx * 0.07,
            dy * 0.07,
            dz * 0.07,
            r_life,
            r_size,
            lifetime as u32,
        ));
    }
    out
}

/// The sculk-shriek ring walk (javap `LevelEventHandler` case 3007 —
/// offset 3193 — over the 26.2 jar): the data word the case's own
/// body NEVER reads (no `iload_3` the transport honesty keeps), the
/// TEN `addParticle(new ShriekParticleOption(i * 5), pos.x + 0.5,
/// pos.y + TOP_Y, pos.z + 0.5, 0, 0, 0)` walks the `i < 10` loop
/// banks — the delay 0, 5, .., 45 ticks the staggered launch rides,
/// the shriek height `TOP_Y` = 0.5 the bottom-half collision shape's
/// max keeps. THE PROVIDER CHAIN each ring member spends (javap
/// `ShriekParticle$Provider.createParticle` + the base ctor over the
/// 26.2 jar): the `SpriteSet.get(random)` sprite pick FIRST (the
/// `BitRandomSource.nextInt(1)` power-of-2 arm the single-sprite set
/// keeps — the draw SPENDS, the value the one-sprite list ignores),
/// then the base ctor's dead life + dead size draws (the values the
/// ctor's own putfields override — 0.85 quadSize, 30 lifetime), NO
/// position/speed draws the zero trio keeps. THE RISE the ctor's
/// `yd = 0.1` write carries (the base 0.98 friction the decay
/// rides, the gravity 0.0F the plain float keeps), the 30-tick life
/// the fixed write speaks, the sleep the delay walk answers (NO age,
/// NO physics the asleep ticks never spend). The muted voice the
/// render lane's own serve banks beside the ring (the waterlogged
/// gate the arm's own state read folds).
pub fn sculk_shriek_ring(anchor: (i32, i32, i32), rng: &mut crate::drops::Lcg) -> Vec<Particle> {
    let mut out = Vec::new();
    let px = anchor.0 as f64 + 0.5;
    let py = anchor.1 as f64 + 0.5;
    let pz = anchor.2 as f64 + 0.5;
    for i in 0..10u32 {
        let delay = i * 5;
        // THE SPRITE PICK (the `SpriteSet.get(random)` law — the
        // `nextInt(1)` power-of-2 arm the draw spends, the value the
        // one-sprite list ignores)
        let _sprite_pick = rng.next_int(1);
        // THE BASE CTOR'S OWN DRAWS (the dead life + the dead size —
        // the values the ctor's putfields override)
        let r_life = rng.next_f32();
        let r_size = rng.next_f32();
        let _dead_base_life = base_lifetime(r_life);
        let _dead_size = base_quad_size(r_size);
        out.push(Particle {
            kind: ParticleKind::SculkShriek { delay },
            x: px,
            y: py,
            z: pz,
            prev: None,
            vx: 0.0,
            vy: 0.1,
            vz: 0.0,
            age: 0,
            lifetime: SCULK_SHRIEK_LIFETIME,
            gravity: 0.0,
            friction: 0.98,
            quad_size: 0.85,
            uo: 0.0,
            vo: 0.0,
            col: [1.0, 1.0, 1.0],
        });
    }
    out
}

/// The SCULK_SOUL bloom wisps (javap the walk the 26.2 jar serves —
/// the bloom's `sendParticles(SCULK_SOUL, x + 0.5, y + 1.15, z + 0.5,
/// 2, 0.2, 0.0, 0.2, 0)` post, the CLIENT handler the packet answers:
/// `ClientPacketListener.handleParticleEvent` count>0 branch pc
/// 113-266 — per particle SIX `nextGaussian` draws the walk spends
/// (3 offsets × the packet's dist trio + 3 velocities × the packet's
/// speed), the offsets the `x + g * dist` folds wear, the velocities
/// the `g * 0.0` speed multiplies DISCARD (the spends the 0-speed
/// packet keeps, the values the folds never wear); then the
/// `SoulParticle` ctor chain (the `Particle` base's dead life + size
/// draws, the `RisingParticle` pc 69-162 the ±0.05 jitter pairs and
/// the 8.0/(r*0.8+0.2)+4 lifetime write, the `scale(1.5F)` the quad
/// ×1.5 fold rides, the `SpriteSet.first()` NO draw the first() call
/// speaks) — the friction 0.96 the RisingParticle putfield keeps, the
/// gravity 0.0 the base float, the velocity ZERO the packet's
/// 0.0-speed multiplies wear (the wisps hover the jitter alone
/// moves). THE SAME STREAM the ambient walk serves — the level
/// ambient rng the client's own random bank stands in (the shriek
/// ring's own convention the wire lanes keep).
#[cfg_attr(not(test), expect(dead_code))]
pub fn sculk_soul_wisps(anchor: (i32, i32, i32), rng: &mut crate::drops::Lcg) -> Vec<Particle> {
    // the packet's own base pos (the bloom walk's +0.5 / +1.15 /
    // +0.5 folds the bytecode speaks)
    let px = anchor.0 as f64 + 0.5;
    let py = anchor.1 as f64 + 1.15;
    let pz = anchor.2 as f64 + 0.5;
    const X_DIST: f64 = 0.2;
    const Y_DIST: f64 = 0.0;
    const Z_DIST: f64 = 0.2;
    const SPEED: f64 = 0.0;
    let mut out = Vec::new();
    for _ in 0..2u32 {
        // THE HANDLER'S SIX GAUSSIANS (pc 123-222 — 3 offsets + 3
        // velocities, ALL the draws spend, the velocity trio the 0.0
        // multiplies discard)
        let ox = rng.next_gaussian() * X_DIST;
        let oy = rng.next_gaussian() * Y_DIST;
        let oz = rng.next_gaussian() * Z_DIST;
        let _vx = rng.next_gaussian() * SPEED;
        let _vy = rng.next_gaussian() * SPEED;
        let _vz = rng.next_gaussian() * SPEED;
        // THE BASE CTOR'S DEAD DRAWS (the life the RisingParticle's
        // own write overrides, the size the scale(1.5F) fold rides)
        let r_life = rng.next_f32();
        let r_size = rng.next_f32();
        let _dead_base_life = base_lifetime(r_life);
        // THE RISINGPARTICLE WALK (pc 69-191 — the ±0.05 jitter
        // pairs, the lifetime write the law speaks)
        let jx = (rng.next_f32() - rng.next_f32()) * 0.05;
        let jy = (rng.next_f32() - rng.next_f32()) * 0.05;
        let jz = (rng.next_f32() - rng.next_f32()) * 0.05;
        let r_life2 = rng.next_f32();
        out.push(Particle {
            kind: ParticleKind::SculkSoul,
            x: px + ox + jx as f64,
            y: py + oy + jy as f64,
            z: pz + oz + jz as f64,
            prev: None,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            age: 0,
            lifetime: sculk_soul_lifetime(r_life2),
            gravity: 0.0,
            friction: 0.96,
            quad_size: base_quad_size(r_size) * 1.5,
            uo: 0.0,
            vo: 0.0,
            col: [1.0, 1.0, 1.0],
        });
    }
    out
}

/// The Direction enum's own ordinal (the values() array index the
/// `MultifaceBlock.unpack` bit law reads — DOWN 0, UP 1, NORTH 2,
/// SOUTH 3, WEST 4, EAST 5).
fn face_ordinal(sx: i32, sy: i32, sz: i32) -> u32 {
    if sy < 0 {
        0
    } else if sy > 0 {
        1
    } else if sz < 0 {
        2
    } else if sz > 0 {
        3
    } else if sx < 0 {
        4
    } else {
        5
    }
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
    fn the_campfire_smoke_the_ctor_chain_wears() {
        // THE STREAM'S OWN PARITY (javap CampfireSmokeParticle + the
        // CosyProvider the 26.2 jar speaks — the draw order the arg
        // evaluation + the ctor chain pin): the `SpriteSet.get` pick
        // FIRST (nextInt(12) the ascending list reads), then the base
        // Particle ctor's DEAD lifetime draw, then the
        // SingleQuadParticle tail's SIZE draw the scale(3.0f) triple
        // multiplies, then the own nextInt(50) + 80 ladder and the
        // nextFloat()/500 yd bump. The replayed Lcg the same five
        // draws spends — the spawned puff bit for bit.
        let mut r = rng();
        let frame = r.next_int(12) as usize;
        let _dead = base_lifetime(r.next_f32());
        let quad = base_quad_size(r.next_f32()) * 3.0;
        let life = campfire_smoke_lifetime(r.next_int(50), false);
        let bump = f64::from(r.next_f32() / 500.0);
        let mut r2 = rng();
        let p = campfire_smoke_puff(1.0, 2.0, 3.0, 0.0, 0.07, 0.0, false, &mut r2);
        assert!(matches!(
            p.kind,
            ParticleKind::CampfireSmoke { .. }
        ));
        let ParticleKind::CampfireSmoke {
            frame: got_frame,
            alpha,
        } = p.kind
        else {
            panic!("the campfire kind the ctor banks");
        };
        assert_eq!(got_frame, frame, "the ascending list the pick reads");
        assert!((alpha - 0.9).abs() < 1e-6, "the Cosy setAlpha keeps");
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
        assert_eq!(p.vx, 0.0);
        assert!((p.vy - (0.07 + bump)).abs() < 1e-12, "the 0.07 arg + the bump");
        assert_eq!(p.vz, 0.0);
        assert_eq!(p.lifetime, life, "the nextInt(50)+80 the cosy arm keeps");
        assert!(
            (p.lifetime as f64) >= 80.0 && (p.lifetime as f64) <= 129.0,
            "the cosy band the ladder keeps: {}",
            p.lifetime
        );
        assert!((p.quad_size - quad).abs() < 1e-6);
        assert!(p.quad_size >= 0.3 && p.quad_size <= 0.6, "the scale(3.0f)");
        assert_eq!(p.gravity, 3.0e-6, "the RAW gravity the tick subtracts");
        assert_eq!(p.friction, 0.98, "the base ctor's own field");
        assert_eq!(p.col, [1.0, 1.0, 1.0], "the WHITE tint no draw tints");
        // THE SIGNAL ARM (the force flag the ladder's own row keeps —
        // the nextInt(50) + 280 band, the 0.95 the setAlpha keeps)
        let mut r3 = rng();
        let sig = campfire_smoke_puff(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true, &mut r3);
        let ParticleKind::CampfireSmoke {
            alpha: sig_alpha, ..
        } = sig.kind
        else {
            panic!("the campfire kind the ctor banks");
        };
        assert!(
            (sig_alpha - 0.95).abs() < 1e-6,
            "the Signal setAlpha the spawn keeps: {}",
            sig_alpha
        );
        assert!(
            (280..=329).contains(&sig.lifetime),
            "the +280 band the force arm keeps: {}",
            sig.lifetime
        );
    }

    #[test]
    fn the_campfire_smoke_the_tick_the_drift_fade_wear() {
        // THE TICK'S OWN LAW (javap CampfireSmokeParticle.tick the
        // 26.2 jar verbatim): the four rolls per tick, the RAW
        // gravity subtract (no 0.04 the base lane folds), the NO
        // friction the override never applies, the fade the last-60
        // band serves, the remove the alpha<=0 OR-arm answers.
        let mut r = rng();
        let mut p = campfire_smoke_puff(0.0, 0.0, 0.0, 0.25, 0.07, 0.0, false, &mut r);
        let life = p.lifetime;
        let fade_start = life - 60; // the age the fade band's head opens
        let spawn_vy = p.vy;
        // THE FADE'S OWN BOUNDARY (the tick n = fade_start the age
        // walks FIRST — the fade arm the SAME tick drains -0.015)
        for _ in 0..fade_start {
            assert!(p.tick_campfire_smoke(&mut r), "the alive tick");
        }
        assert_eq!(p.age, fade_start);
        let ParticleKind::CampfireSmoke { alpha, .. } = &p.kind else {
            panic!();
        };
        assert!(
            (*alpha - (0.9 - 0.015)).abs() < 1e-6,
            "the boundary tick's own drain the alpha wears: {}",
            alpha
        );
        // THE GRAVITY'S OWN RAW SUBTRACT (the vy -= 3e-6 per tick the
        // NO 0.04 fold wears — the exact row the spawn's bump keeps)
        assert!(
            (p.vy - (spawn_vy - fade_start as f64 * 3.0e-6)).abs() < 1e-12,
            "the raw subtract the vy row wears: {}",
            p.vy
        );
        // THE NO-FRICTION LAW (the vx the drift rolls alone move —
        // the 0.98 multiply the base lane never reaches, the
        // ±1/5000-per-tick the band keeps)
        assert!(
            p.vx.abs() <= 1.0 && p.vx != 0.0,
            "the drift the vx moves (no shrink): {}",
            p.vx
        );
        // THE DRAIN'S OWN LADDER (the -0.015 per tick while > 0.01 —
        // 0.885 / 0.015 = 59 the exact drains the band holds)
        for _ in 0..58 {
            assert!(p.tick_campfire_smoke(&mut r), "the alive drain");
            let ParticleKind::CampfireSmoke { alpha, .. } = &p.kind else {
                panic!();
            };
            assert!(*alpha > 0.0, "the positive the drain keeps: {}", alpha);
        }
        // THE DRAINED ALPHA'S OWN REMOVE (the last sub-quantum tick
        // the alpha 0.0 banks, the next tick the OR-arm answers)
        let mut alive = true;
        for _ in 0..5 {
            alive = p.tick_campfire_smoke(&mut r);
            if !alive {
                break;
            }
        }
        assert!(!alive, "the drained alpha the remove answers");
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
    fn the_smash_burst_walks_the_two_loops_the_overrides_wear() {
        // THE SMASH-ATTACK BURST'S OWN LAW (javap
        // `ParticleUtils.spawnSmashAttackParticles` + the
        // DustPillarProvider's own overrides): the two loops the
        // count's own thirds and two-thirds keep, the anchor fold the
        // +1.0 top-face plane speaks, the provider's speed/lifetime
        // overrides the walk pins, the dead draws ride, the seeded
        // chain replays, the gate-less caller (None) the draws alone
        // spend.
        let mut r = rng();
        let burst = smash_attack_burst((10, 20, 30), 750, Some(7), &mut r);
        // the loop bounds: 750/3 = 250 in-block + 750/1.5 = 500 ring
        assert_eq!(burst.len(), 750);
        // the anchor fold: the first particle's own spot (the in-block
        // jitter the +next/2 offset gives — the x/z bands [0.5, 1.0),
        // the y the +1.0 plane the never-drawn band keeps)
        let first = &burst[0];
        assert!((first.y - 21.0).abs() < 1e-9);
        assert!(first.x >= 10.5 && first.x < 11.0);
        assert!(first.z >= 30.5 && first.z < 31.0);
        // the terrain kind rides the caller's tile, the gravity the
        // ctor keeps
        assert_eq!(first.kind, ParticleKind::Terrain { tile: 7 });
        assert!((first.gravity - 1.0).abs() < 1e-6);
        // the provider's own overrides: the x/z speeds the /30 fold
        // gives (the tiny band [0, 1/30)), the y the caller's speed +
        // the /2 bias (the caller's speed the *0.2 draw carries)
        assert!(first.vx >= 0.0 && first.vx < 1.0 / 30.0);
        assert!(first.vz >= 0.0 && first.vz < 1.0 / 30.0);
        assert!(first.vy >= 0.0 && first.vy < 0.2 + 0.5);
        // the lifetime the 20 + nextInt(20) band keeps
        assert!(first.lifetime >= 20 && first.lifetime < 40);
        // the ring loop's own spots: the 3.5-radius circle around the
        // anchor center (the index the radian arg keeps — the first
        // ring particle rides cos(0) = 1.0, sin(0) = 0.0 plus its own
        // jitter)
        let ring = &burst[250];
        assert!(ring.x >= 10.5 + 3.5 && ring.x < 10.5 + 3.5 + 0.5 + 1e-9);
        assert!(ring.z >= 30.5 && ring.z < 31.0);
        assert!((ring.y - 21.0).abs() < 1e-9);
        // the ring speeds the *0.05 draws carry (the y the +next/2
        // bias keeps — the near-zero band)
        assert!(ring.vy >= 0.0 && ring.vy < 0.05 + 0.5);
        // the second in-block particle's spot replays the seeded chain
        // (the draws the one stream spends)
        let mut r2 = rng();
        let burst2 = smash_attack_burst((10, 20, 30), 750, Some(7), &mut r2);
        assert_eq!(burst.len(), burst2.len());
        for (a, b) in burst.iter().zip(burst2.iter()) {
            assert_eq!(
                (a.x, a.y, a.z, a.vx, a.vy, a.vz, a.lifetime),
                (b.x, b.y, b.z, b.vx, b.vy, b.vz, b.lifetime),
                "the seeded chain replays"
            );
            assert_eq!(&a.kind, &b.kind, "the kind the chain replays");
        }
        // the gate-less caller (None): no burst, the loop draws alone
        // spend (the vanilla truth the level stream never skips)
        let mut r3 = rng();
        let none = smash_attack_burst((10, 20, 30), 750, None, &mut r3);
        assert!(none.is_empty(), "the null verdict the burst never speaks");
        // the count the loop bounds wear: the zero word the silence
        // keeps, the small counts the floor band keeps (1 -> the
        // in-block loop alone)
        let mut r4 = rng();
        assert!(smash_attack_burst((0, 0, 0), 0, Some(7), &mut r4).is_empty());
        let mut r5 = rng();
        assert_eq!(
            smash_attack_burst((0, 0, 0), 3, Some(7), &mut r5).len(),
            1 + 2,
            "the third and the two-thirds the small counts keep"
        );
    }

    #[test]
    fn the_wax_faces_walk_the_six_faces_the_folds_wear() {
        // THE WAX TRIO'S OWN LAW (javap `ParticleUtils
        // .spawnParticlesOnBlockFaces` -> `spawnParticleOnFace` + the
        // `getRandomSpeedRanges` supplier + the GlowParticle
        // providers): the six faces the Direction.values() order
        // keeps, the per-face `3 + nextInt(3)` counts, the position
        // pair (the face axis's * 0.55 plane, the perpendicular
        // pair's draws), the in-plane velocity (the face-normal axis
        // zeroes), the provider
        // folds (the halves law the wax pair speaks, the plain trio
        // the scrape keeps), the tints the setColor rows bank, the
        // 10..39 lifetime, the seeded chain replays.
        let seed = 0x5EED_3003;
        let mut r = crate::drops::Lcg::new(seed);
        let burst = wax_faces_burst((10, 20, 30), mc_protocol::PARTICLES_AND_SOUND_WAX_ON_EFFECT, &mut r);
        // the count law: six faces x (3..=5) puffs
        assert!(burst.len() >= 18 && burst.len() <= 30);
        // the six face planes: each face's own group — the DOWN face
        // rides cy - 0.55, the UP cy + 0.55, the NORTH cz - 0.55, the
        // SOUTH cz + 0.55, the WEST cx - 0.55, the EAST cx + 0.55;
        // the perpendicular pair draws (the [c - 0.5, c + 0.5) bands),
        // the counts the 3..=5 band keeps, the groups tile the whole
        // burst in the values() order
        let cx = 10.5;
        let cy = 20.5;
        let cz = 30.5;
        for p in &burst {
            // per-particle provider facts (the GlowParticle ctor law)
            assert_eq!(p.kind, ParticleKind::Glow);
            assert!(p.lifetime >= 10 && p.lifetime < 40, "the nextInt(30) + 10 band");
            assert!((p.friction - 0.96).abs() < 1e-6);
            assert!(p.gravity.abs() < 1e-9, "the base default the zero keeps");
            assert!(p.quad_size >= 0.1 * 0.75 && p.quad_size <= 0.2 * 0.75 + 1e-9);
            // the wax-on tint the setColor row banks
            assert_eq!(p.col, [0.91, 0.55, 0.08]);
        }
        // the face groups: DOWN (y = cy - 0.55) FIRST, then UP, NORTH,
        // SOUTH, WEST, EAST — each group's perpendicular pair inside
        // the draw bands, each group's count 3..=5
        let mut i = 0usize;
        // DOWN group
        let down: Vec<_> = burst[i..]
            .iter()
            .take_while(|p| (p.y - (cy - 0.55)).abs() < 1e-9)
            .collect();
        assert!(down.len() >= 3 && down.len() <= 5);
        for p in &down {
            assert!(p.x > cx - 0.5 && p.x < cx + 0.5);
            assert!(p.z > cz - 0.5 && p.z < cz + 0.5);
            // the in-plane velocity: the DOWN face's own axis the
            // ternary zeroes, the in-plane pair keeps the supplier's
            // draws (the 445 round's re-audit banked the flip)
            assert_eq!(p.vy, 0.0, "the face-normal axis the ifne zeroes");
            // the wax pair's halves law the x/z folds speak
            assert!(p.vx.abs() <= 0.005 / 2.0 + 1e-15);
            assert!(p.vz.abs() <= 0.005 / 2.0 + 1e-15);
        }
        i += down.len();
        // UP group
        let up: Vec<_> = burst[i..]
            .iter()
            .take_while(|p| (p.y - (cy + 0.55)).abs() < 1e-9)
            .collect();
        assert!(up.len() >= 3 && up.len() <= 5);
        for p in &up {
            assert_eq!(p.vy, 0.0, "the face-normal axis the ifne zeroes");
            assert!(p.vx.abs() <= 0.005 / 2.0 + 1e-15);
            assert!(p.vz.abs() <= 0.005 / 2.0 + 1e-15);
        }
        i += up.len();
        // NORTH group (z = cz - 0.55)
        let north: Vec<_> = burst[i..]
            .iter()
            .take_while(|p| (p.z - (cz - 0.55)).abs() < 1e-9)
            .collect();
        assert!(north.len() >= 3 && north.len() <= 5);
        for p in &north {
            assert_eq!(p.vz, 0.0, "the face-normal axis the ifne zeroes");
            assert!(p.vx.abs() <= 0.005 / 2.0 + 1e-15);
            assert!(p.vy.abs() <= 0.005 + 1e-12);
            assert!(p.x > cx - 0.5 && p.x < cx + 0.5);
            assert!(p.y > cy - 0.5 && p.y < cy + 0.5);
        }
        i += north.len();
        // SOUTH group (z = cz + 0.55)
        let south: Vec<_> = burst[i..]
            .iter()
            .take_while(|p| (p.z - (cz + 0.55)).abs() < 1e-9)
            .collect();
        assert!(south.len() >= 3 && south.len() <= 5);
        i += south.len();
        // WEST group (x = cx - 0.55): the halves law the x fold speaks
        let west: Vec<_> = burst[i..]
            .iter()
            .take_while(|p| (p.x - (cx - 0.55)).abs() < 1e-9)
            .collect();
        assert!(west.len() >= 3 && west.len() <= 5);
        for p in &west {
            assert_eq!(p.vx, 0.0, "the face-normal axis the ifne zeroes");
            // the wax pair's halves law the z fold speaks
            assert!(p.vz.abs() <= 0.005 / 2.0 + 1e-15);
            assert!(p.vy.abs() <= 0.005 + 1e-12);
            assert!(p.z > cz - 0.5 && p.z < cz + 0.5);
            assert!(p.y > cy - 0.5 && p.y < cy + 0.5);
        }
        i += west.len();
        // EAST group (x = cx + 0.55) — the burst's tail
        let east: Vec<_> = burst[i..]
            .iter()
            .take_while(|p| (p.x - (cx + 0.55)).abs() < 1e-9)
            .collect();
        assert!(east.len() >= 3 && east.len() <= 5);
        i += east.len();
        assert_eq!(i, burst.len(), "the six groups tile the whole burst");
        // the seeded chain replays
        let mut r2 = crate::drops::Lcg::new(seed);
        let burst2 =
            wax_faces_burst((10, 20, 30), mc_protocol::PARTICLES_AND_SOUND_WAX_ON_EFFECT, &mut r2);
        assert_eq!(burst.len(), burst2.len());
        for (a, b) in burst.iter().zip(burst2.iter()) {
            assert_eq!(
                (a.x, a.y, a.z, a.vx, a.vy, a.vz, a.lifetime, a.quad_size),
                (b.x, b.y, b.z, b.vx, b.vy, b.vz, b.lifetime, b.quad_size),
                "the seeded chain replays"
            );
        }
        // THE WAX-OFF TINT (the same walk, the white twin's row)
        let mut r3 = crate::drops::Lcg::new(seed);
        let off =
            wax_faces_burst((10, 20, 30), mc_protocol::PARTICLES_WAX_OFF_EFFECT, &mut r3);
        assert_eq!(off.len(), burst.len(), "the count stream the word never shifts");
        for p in &off {
            assert_eq!(p.col, [1.0, 0.9, 1.0]);
        }
        // THE SCRAPE TINT GATE (the nextBoolean pair — every puff
        // rides one of the two rows, the speed fold the PLAIN trio
        // keeps: the west face's x band the no-halves speak)
        let mut r4 = crate::drops::Lcg::new(seed);
        let scrape =
            wax_faces_burst((10, 20, 30), mc_protocol::PARTICLES_SCRAPE_EFFECT, &mut r4);
        // the scrape stream the extra nextBoolean draw per particle
        // spends — the counts the stream's own shift re-draws (the
        // band the six faces keep)
        assert!(scrape.len() >= 18 && scrape.len() <= 30);
        for p in &scrape {
            assert!(
                p.col == [0.29, 0.58, 0.51] || p.col == [0.43, 0.77, 0.62],
                "the tint gate the two rows keep"
            );
        }
        // the scrape's west-face x band the no-halves fold widens
        let mut i2 = 0usize;
        let down2: Vec<_> = scrape
            .iter()
            .take_while(|p| (p.y - (cy - 0.55)).abs() < 1e-9)
            .collect();
        i2 += down2.len();
        let up2: Vec<_> = scrape[i2..]
            .iter()
            .take_while(|p| (p.y - (cy + 0.55)).abs() < 1e-9)
            .collect();
        i2 += up2.len();
        let north2: Vec<_> = scrape[i2..]
            .iter()
            .take_while(|p| (p.z - (cz - 0.55)).abs() < 1e-9)
            .collect();
        i2 += north2.len();
        let south2: Vec<_> = scrape[i2..]
            .iter()
            .take_while(|p| (p.z - (cz + 0.55)).abs() < 1e-9)
            .collect();
        i2 += south2.len();
        let west2: Vec<_> = scrape[i2..]
            .iter()
            .take_while(|p| (p.x - (cx - 0.55)).abs() < 1e-9)
            .collect();
        for p in &west2 {
            assert!(p.vx.abs() <= 0.005 + 1e-15, "the plain trio the no-halves keeps");
        }
        // THE UNKNOWN WORD the silence keeps (no burst, no draws)
        let mut r5 = crate::drops::Lcg::new(seed);
        assert!(wax_faces_burst((1, 2, 3), 3006, &mut r5).is_empty());
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
    fn the_flash_bell_speaks_the_quad_law() {
        // javap `OverlayParticle.getQuadSize`: `7.1F × sin((age +
        // partial − 1) × 0.25F × π)` — the growth-and-shrink arc the
        // four-tick life wears, the negative first-tick fold the
        // spawn frame's own quirk the verbatim law keeps
        let bell = flash_bell(0, 0.0);
        let want = 7.1f32 * (-1.0f32 * 0.25 * std::f32::consts::PI).sin();
        assert!((bell - want).abs() < 1e-6, "the spawn frame the negative fold keeps");
        assert_eq!(flash_bell(1, 0.0), 0.0, "the zero crossing the sin answers");
        assert_eq!(flash_bell(3, 0.0), 7.1, "the peak the sin(π/2) answer keeps");
        let mid = flash_bell(2, 0.0);
        assert!(
            (mid - 7.1f32 * (std::f32::consts::FRAC_PI_4).sin()).abs() < 1e-6,
            "the mid arc the sin(π/4) fold keeps"
        );
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
    fn the_in_block_burst_the_shape_fold_the_force_lane_wears() {
        // THE IN-BLOCK GROWTH BURST'S OWN WALK (javap
        // `ParticleUtils.spawnParticleInBlock` over the 26.2 jar — the
        // LevelEventHandler's 2011/2012 cases the bee's growth word and
        // the turtle egg's placement word speak): the anchor cell's own
        // state the shape read opens — the `isAir` gate the 1.0 answer
        // keeps, the wheat ladder the block's own shape folds (the age
        // ladder `(age * 2 + 2) / 16` the growth arm's own law wears) —
        // then the burst the in-block force lane arms (the count the
        // caller's data word carries, the x/z the [anchor, anchor+1)
        // band the 0.5 spread pair folds, the ground gate the force
        // lane never reads).
        // THE AIR CELL (the `isAir` gate the dconst_1 answer keeps):
        // the dy 1.0 the band [y, y+1) speaks
        let w = growth_arena(20);
        let burst = in_block_growth_burst((12, 21, 12), 15, &w, &mut rng());
        assert_eq!(burst.len(), 15, "the bipush 15 the callers' own count wears");
        for p in &burst {
            assert_eq!(p.kind, ParticleKind::HappyVillager, "the glint sprite");
            assert!(
                (12.0..13.0).contains(&p.x),
                "x inside the in-block band: {}",
                p.x
            );
            assert!(
                (21.0..22.0).contains(&p.y),
                "y inside the air cell's own 1.0 band: {}",
                p.y
            );
            assert!(
                (12.0..13.0).contains(&p.z),
                "z inside the in-block band: {}",
                p.z
            );
        }
        // THE WHEAT CELL (the age ladder the shape's own Y-max speaks):
        // stage 3 the (3*2+2)/16 = 0.5 band keeps — the burst lives
        // INSIDE the stalk's own half-height
        let mut cropped = growth_arena(20);
        cropped.set(12, 21, 12, crate::world::WHEAT + 3);
        let in_crop = in_block_growth_burst((12, 21, 12), 15, &cropped, &mut rng());
        assert_eq!(in_crop.len(), 15, "the in-block lane never skips");
        for p in &in_crop {
            assert!(
                p.y >= 21.0 && p.y < 21.0 + 0.5,
                "y inside the stage-3 stalk's own band: {}",
                p.y
            );
            assert!(
                (12.0..13.0).contains(&p.x) && (12.0..13.0).contains(&p.z),
                "the x/z band the 0.5 pair folds keeps"
            );
        }
        // THE FULL CUBE (the DIRT cell the shape's 1.0 Y-max keeps —
        // the same answer the air gate wears)
        let mut solid = growth_arena(20);
        solid.set(12, 21, 12, crate::world::DIRT);
        let in_solid = in_block_growth_burst((12, 21, 12), 15, &solid, &mut rng());
        for p in &in_solid {
            assert!(
                (21.0..22.0).contains(&p.y),
                "y inside the full cube's own 1.0 band: {}",
                p.y
            );
        }
        // THE FORCE LANE (the ground gate the delegate never reads):
        // the below column gone the whole burst still lands — the
        // in-block truth the vanilla walk keeps
        let mut holed = growth_arena(20);
        for z in 10..15 {
            holed.set(12, 20, z, crate::world::AIR);
        }
        let floating = in_block_growth_burst((12, 21, 12), 15, &holed, &mut rng());
        assert_eq!(floating.len(), 15, "the force lane never reads the ground");
        // the determinism the draw order keeps (the same seed the same
        // burst — the shape read draws nothing, the burst's own chain
        // the gaussians the doubles the ctor draws)
        let again = in_block_growth_burst((12, 21, 12), 15, &w, &mut rng());
        for (a, b) in burst.iter().zip(again.iter()) {
            assert_eq!((a.x, a.y, a.z), (b.x, b.y, b.z));
            assert_eq!((a.vx, a.vy, a.vz), (b.vx, b.vy, b.vz));
        }
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
    fn the_large_smoke_puff_the_draws_the_size_wears() {
        // THE BIRTH (the addParticle pos the case-1501 walk speaks —
        // the (x + nd, y + 1.2, z + nd) folds the caller places, the
        // zero velocity trio the `dconst_0` answers): the puff sits
        // at the caller's own corner
        let mut r = rng();
        let p = large_smoke_puff(11.3, 21.2, 32.7, 0.0, 0.0, 0.0, &mut r);
        assert!(matches!(p.kind, ParticleKind::Smoke));
        assert_eq!(p.x, 11.3, "the caller's own birth x");
        assert_eq!(p.y, 21.2, "the caller's own birth y — the +1.2 lift the walk keeps");
        assert_eq!(p.z, 32.7, "the caller's own birth z");
        // THE LAWS (the SAME BaseAshSmokeParticle chain the plain
        // smoke wears — the LargeSmokeParticle subclass the ONE
        // multiplier constant speaks): the grey tint the r*0.3 draw
        // carries, the 0.75 × 2.5 = 1.875 size fold the plain's
        // 0.75 the one constant's own difference wears, the 8..40
        // lifetime, the -0.1 gravity the rise rides, the 0.96
        // friction the ctor sets
        assert_eq!(p.gravity, -0.1);
        assert_eq!(p.friction, 0.96);
        assert!((8..=40).contains(&p.lifetime), "the ash lifetime: {}", p.lifetime);
        assert!(
            (0.1875..0.375).contains(&p.quad_size),
            "the 0.75 × 2.5 fold's own band: {}",
            p.quad_size
        );
        assert!(
            p.col.iter().all(|c| (0.0..0.3).contains(c)),
            "the grey tint the r*0.3 keeps: {:?}",
            p.col
        );
        // THE VELOCITY (the zero trio the case passes — the chain's
        // own spread the *0.1 scale carries): |vx| <= 0.018, the vy
        // band the +0.1 bump scales to [-0.008, 0.028) — the rise
        // the fold and the gravity both give
        assert!(p.vx.abs() <= 0.018 + 1e-9, "the drift band: {}", p.vx);
        assert!(p.vz.abs() <= 0.018 + 1e-9, "the drift band: {}", p.vz);
        assert!(
            (-0.008..0.028).contains(&p.vy),
            "the smoke's own rise band: {}",
            p.vy
        );
        // THE DETERMINISM (the same seed the same chain answers)
        let p2 = large_smoke_puff(11.3, 21.2, 32.7, 0.0, 0.0, 0.0, &mut rng());
        assert_eq!(
            (p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
            (p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size),
            "the seeded chain replays"
        );
        // THE STREAM (the nine draws the chain consumes — the dead
        // base lifetime, the three spreads, the d0 pair, the size,
        // the tint, the own lifetime; the tenth draw differs)
        let mut a = rng();
        let _ = large_smoke_puff(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, &mut a);
        let after_nine = a.next();
        let mut b = rng();
        for _ in 0..9 {
            b.next();
        }
        assert_eq!(after_nine, b.next(), "the large smoke chain's own nine draws");
    }

    #[test]
    fn the_composter_puff_the_five_draws_the_lifetime_wears() {
        // THE BIRTH (the addParticle pos the handleFill loop speaks —
        // the interior corner the case's own folds place, the
        // gaussian trio the velocity args carry): the puff sits at
        // the caller's own fold
        let mut r = rng();
        let p = composter_fill_puff(12.3, 41.2, -21.7, 1.0, 2.0, -1.0, &mut r);
        assert!(matches!(p.kind, ParticleKind::HappyVillager));
        assert_eq!(p.x, 12.3, "the caller's own birth x");
        assert_eq!(p.y, 41.2, "the caller's own birth y");
        assert_eq!(p.z, -21.7, "the caller's own birth z");
        // THE LAWS (the ComposterFillProvider chain): the white tint
        // the setColor(1,1,1) override keeps, the 0.02 × (nf * 0.6 +
        // 0.5) size band, the 3..7 lifetime the setLifetime(3 +
        // nextInt(5)) override gives, the zero gravity, the 0.99
        // friction the tick's own walk keeps
        assert_eq!(p.col, [1.0, 1.0, 1.0], "the white the override keeps");
        assert!(
            (0.01..0.022).contains(&p.quad_size),
            "the size band: {}",
            p.quad_size
        );
        assert!((3..=7).contains(&p.lifetime), "the 3..7 band: {}", p.lifetime);
        assert!(p.gravity.abs() < 1e-9, "the no-gravity law");
        assert!((p.friction - 0.99).abs() < 1e-9, "the 0.99 friction");
        // THE VELOCITY (the * 0.02 ctor fold the passed args the
        // gaussian's own * 0.02 the case carries — the effective
        // drift the two folds keep: 1.0 → 0.02, 2.0 → 0.04, -1.0 →
        // -0.02)
        assert!((p.vx - 0.02).abs() < 1e-12, "the x fold: {}", p.vx);
        assert!((p.vy - 0.04).abs() < 1e-12, "the y fold: {}", p.vy);
        assert!((p.vz + 0.02).abs() < 1e-12, "the z fold: {}", p.vz);
        // THE DETERMINISM (the same seed the same chain answers)
        let p2 = composter_fill_puff(12.3, 41.2, -21.7, 1.0, 2.0, -1.0, &mut rng());
        assert_eq!(
            (p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
            (p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size),
            "the seeded chain replays"
        );
        // THE STREAM (the five draws the chain consumes — the
        // one-frame glint pick, the ONE dead tint the ctor's fstore
        // [15] the r/g/b trio shares, the size, the dead base
        // lifetime, the own nextInt(5); the sixth draw differs; THE
        // CORRECTION the autopush-450 re-audit banks — the prior
        // seven-draw reading spent two phantom tints the jar's own
        // shared-local shape never draws)
        let mut a = rng();
        let _ = composter_fill_puff(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, &mut a);
        let after_five = a.next();
        let mut b = rng();
        for _ in 0..5 {
            b.next();
        }
        assert_eq!(after_five, b.next(), "the composter chain's own five draws");
    }

    #[test]
    fn the_sparkle_the_four_draws_the_glint_the_dead_tint_wear() {
        // THE PROVIDER CHAIN (javap `$HappyVillagerProvider
        // .createParticle` verbatim — THE CORRECTION the autopush-451
        // re-audit banks): the FOUR draws the chain spends in the
        // vanilla order — the one-frame glint pick (pc 22 the
        // invokeinterface the SpriteSet.get(random) walk speaks — the
        // nextInt(1) the draw the pick always lands), the ONE dead
        // tint the base ctor's fstore [15] the r/g/b putfield trio
        // loads (the setColor(1,1,1) the VALUE overrides, the draw
        // the stream still spends), the size, the lifetime (the
        // provider NO setLifetime speaks — the ctor's own write the
        // REAL life keeps). The prior reading spent TWO draws (life +
        // size) — the glint pick and the dead tint the jar's own
        // chain never skips.
        let mut r = rng();
        let p = happy_villager_sparkle(1.5, 5.6, 2.5, &mut r);
        assert!(matches!(p.kind, ParticleKind::HappyVillager));
        assert_eq!(p.col, [1.0, 1.0, 1.0], "the white the override keeps");
        assert!(
            (0.01..0.022).contains(&p.quad_size),
            "the size band: {}",
            p.quad_size
        );
        assert!(
            (20..=100).contains(&p.lifetime),
            "the lifetime the ctor's own fold keeps: {}",
            p.lifetime
        );
        assert_eq!(p.vx, 0.0, "the zero drift the aiStep's (0,0,0) keeps");
        // THE STREAM (the four draws the chain consumes — the glint
        // pick, the dead tint, the size, the lifetime; the fifth draw
        // differs)
        let mut a = rng();
        let _ = happy_villager_sparkle(0.0, 0.0, 0.0, &mut a);
        let after_four = a.next();
        let mut b = rng();
        for _ in 0..4 {
            b.next();
        }
        assert_eq!(after_four, b.next(), "the sparkle chain's own four draws");
        // THE SIBLING LAW (the growth burst's per-particle chain the
        // SAME provider shape rides — the caller's 3 gaussians + the
        // 3 position doubles, then the provider's four): the seeded
        // replay the burst's own stream answers.
        let w = growth_arena(20);
        let mut a = rng();
        let burst = growth_burst((12, 21, 12), 1, 0.5, 1.0, true, &w, &mut a);
        assert_eq!(burst.len(), 1, "the forced lane the single attempt lands");
        let after_burst = a.next();
        let mut b = rng();
        let vx = b.next_gaussian();
        let _vy = b.next_gaussian();
        let _vz = b.next_gaussian();
        let _px = b.next_f64();
        let _py = b.next_f64();
        let _pz = b.next_f64();
        let _glint = b.next_int(1);
        let _tint = b.next_f32();
        let _size = b.next_f32();
        let _life = b.next_f32();
        assert_eq!(after_burst, b.next(), "the burst's own provider four");
        assert_eq!(burst[0].vx, vx * 0.02 * 0.02, "the two-fold drift");
    }

    #[test]
    fn the_egg_crack_faces_the_six_faces_the_draws_wear() {
        // THE FACES WALK (javap the case 3009 chain verbatim): the six
        // faces the enum's own order banks, the per-face count the
        // `3 + nextInt(4)` band arms, the per-particle chain the
        // supplier-trio → position-pair → provider-chain order pins.
        let mut r = rng();
        let burst = egg_crack_faces((10, 20, 30), &mut r);
        // THE BANDS (the 6 faces x 3..6 the UniformInt.of(3, 6)
        // count speaks)
        assert!(
            (18..=36).contains(&burst.len()),
            "the 18..36 band: {}",
            burst.len()
        );
        for p in &burst {
            // THE LAWS (the EggCrackProvider chain): the white tint
            // the setColor(1,1,1) override keeps, the 0.02 x (nf * 0.6
            // + 0.5) size band, the lifetime the ctor's own
            // `20.0 / (nf * 0.8 + 0.2)` fold answers (the provider NO
            // setLifetime speaks — the 21..100 band the denom band
            // keeps), the zero gravity, the 0.99 friction the tick's
            // own walk keeps
            assert!(matches!(p.kind, ParticleKind::EggCrack));
            assert_eq!(p.col, [1.0, 1.0, 1.0], "the white the override keeps");
            assert!(
                (0.01..0.022).contains(&p.quad_size),
                "the size band: {}",
                p.quad_size
            );
            assert!(
                (21..=100).contains(&p.lifetime),
                "the 21..100 band: {}",
                p.lifetime
            );
            assert!(p.gravity.abs() < 1e-9, "the no-gravity law");
            assert!((p.friction - 0.99).abs() < 1e-9, "the 0.99 friction");
            // THE POSITION BANDS (the anchor center ± the face axis's
            // 0.55 arm or the perpendicular ±0.5 pair — the widest arm
            // the 0.55 band speaks)
            assert!(
                (9.95..=11.05).contains(&p.x),
                "the x band: {}",
                p.x
            );
            assert!(
                (19.95..=21.05).contains(&p.y),
                "the y band: {}",
                p.y
            );
            assert!(
                (29.95..=31.05).contains(&p.z),
                "the z band: {}",
                p.z
            );
            // THE VELOCITY BANDS (the in-plane pair the supplier's
            // ±0.5 draws the `* 0.02` ctor fold damps — the aligned
            // axis's component the ternary's own zeroing keeps)
            assert!(p.vx.abs() <= 0.01, "the x drift: {}", p.vx);
            assert!(p.vy.abs() <= 0.01, "the y drift: {}", p.vy);
            assert!(p.vz.abs() <= 0.01, "the z drift: {}", p.vz);
        }
        // THE SURFACE ARMS (the face-axis particles ride the exact
        // 0.55 arm — a DOWN-face member's y the cy - 0.55 fold gives
        // (cy = 20.5 the +0.5 center keeps), its vy the zeroing keeps)
        let down: Vec<&Particle> = burst
            .iter()
            .filter(|p| (p.y - (20.5 - 0.55)).abs() < 1e-9)
            .collect();
        let up: Vec<&Particle> = burst
            .iter()
            .filter(|p| (p.y - (20.5 + 0.55)).abs() < 1e-9)
            .collect();
        let north: Vec<&Particle> = burst
            .iter()
            .filter(|p| (p.z - (30.5 - 0.55)).abs() < 1e-9)
            .collect();
        assert!(
            !down.is_empty() && !up.is_empty() && !north.is_empty(),
            "the face-axis arms the walk answers: down {} up {} north {}",
            down.len(),
            up.len(),
            north.len()
        );
        for p in down.iter().chain(up.iter()) {
            assert!(p.vy.abs() < 1e-9, "the aligned y the zeroing keeps");
        }
        for p in &north {
            assert!(p.vz.abs() < 1e-9, "the aligned z the zeroing keeps");
        }
        // THE DETERMINISM (the same seed the same chain answers)
        let burst2 = egg_crack_faces((10, 20, 30), &mut rng());
        assert_eq!(burst.len(), burst2.len(), "the seeded chain replays");
        for (p, p2) in burst.iter().zip(burst2.iter()) {
            assert_eq!(
                (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
                (p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size),
                "the seeded members replay"
            );
        }
        // THE STREAM (the exact draw chain the walk consumes: the six
        // count draws + the per-particle nine — the supplier trio +
        // the position pair + the glint pick + the dead tint + the
        // size + the lifetime; the NEXT draw differs)
        let n = burst.len();
        let mut a = rng();
        let _ = egg_crack_faces((10, 20, 30), &mut a);
        let after_walk = a.next();
        let mut b = rng();
        for _ in 0..(6 + n * 9) {
            b.next();
        }
        assert_eq!(after_walk, b.next(), "the walk's own exact draw count");
    }

    #[test]
    fn the_snowflake_burst_the_amounts_the_draws_wear() {
        // THE DETECT-PLAYER WALK (javap `TrialSpawner
        // .addDetectPlayerParticles` verbatim — the `30 + min(amount,
        // 10) * 5` loop bound the min arm caps, the per-iteration
        // THREE nextFloat position draws + the six-draw provider
        // chain the order pins: dx, dz, height, then the velocity
        // spreads x→y→z, the size's two-draw product, the lifetime's
        // f64 fold).
        let mut r = rng();
        let burst = trial_detect_player_burst((10, 20, 30), 1, &mut r);
        // THE COUNT (the 30 + min(1, 10) * 5 = 35 the bound arms)
        assert_eq!(burst.len(), 35, "the amount-1 snowfall");
        // THE CAP (the min(10) arm the bound speaks — amount 77 rides
        // the same 80-member pool the amount 10 arms)
        assert_eq!(
            trial_detect_player_burst((0, 0, 0), 77, &mut rng()).len(),
            80,
            "the 10-cap the min arm keeps"
        );
        assert_eq!(
            trial_detect_player_burst((0, 0, 0), 0, &mut rng()).len(),
            30,
            "the amount-0 floor"
        );
        for p in &burst {
            // THE LAWS (the SnowflakeParticle chain): the near-white
            // bluish tint the setColor(0.923, 0.964, 0.999) constants
            // keep, the size the two-draw product fold answers, the
            // lifetime the `(16.0 / (nf * 0.8 + 0.2)) + 2` fold gives
            // (the 18..81 band the denom band keeps), the 0.225
            // gravity the ctor's field keeps, the 1.0 friction the
            // base law's own no-op arms
            assert!(matches!(p.kind, ParticleKind::Snowflake));
            assert_eq!(
                p.col,
                [0.923, 0.964, 0.999],
                "the bluish-white the override keeps"
            );
            assert!(
                (0.1..0.2).contains(&p.quad_size),
                "the product size band: {}",
                p.quad_size
            );
            assert!(
                (18..=81).contains(&p.lifetime),
                "the 18..81 band: {}",
                p.lifetime
            );
            assert!(
                (p.gravity - 0.225).abs() < 1e-9,
                "the 0.225 gravity"
            );
            assert!(
                (p.friction - 1.0).abs() < 1e-9,
                "the 1.0 friction"
            );
            // THE POSITION BANDS (the center + the ±0.65 spread the
            // dmul 0.65 folds — x and z the pair shares, the height
            // the 0.1..0.9 band the nf * 0.8 fold keeps)
            assert!(
                (9.85..11.15).contains(&p.x),
                "the x band: {}",
                p.x
            );
            assert!(
                (20.1..20.9).contains(&p.y),
                "the y band: {}",
                p.y
            );
            assert!(
                (29.85..31.15).contains(&p.z),
                "the z band: {}",
                p.z
            );
            // THE VELOCITY BANDS (the ±0.05 spreads the ctor's three
            // draws give — the ZERO wire velocity the dconst_0 trio
            // speaks, the spread the whole drift carries)
            assert!(p.vx.abs() <= 0.05, "the x spread: {}", p.vx);
            assert!(p.vy.abs() <= 0.05, "the y spread: {}", p.vy);
            assert!(p.vz.abs() <= 0.05, "the z spread: {}", p.vz);
        }
        // THE DETERMINISM (the same seed the same chain answers)
        let burst2 = trial_detect_player_burst((10, 20, 30), 1, &mut rng());
        assert_eq!(burst.len(), burst2.len(), "the seeded chain replays");
        for (p, p2) in burst.iter().zip(burst2.iter()) {
            assert_eq!(
                (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
                (p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size),
                "the seeded members replay"
            );
        }
        // THE STREAM (the exact draw chain the walk consumes: the
        // per-iteration NINE — three position + three velocity + two
        // size + one life; the NEXT draw differs)
        let mut a = rng();
        let _ = trial_detect_player_burst((10, 20, 30), 1, &mut a);
        let after_walk = a.next();
        let mut b = rng();
        for _ in 0..(35 * 9) {
            b.next();
        }
        assert_eq!(after_walk, b.next(), "the walk's own exact draw count");
    }

    #[test]
    fn the_trial_spawn_the_forty_particles_the_chains_wear() {
        // THE ADD-SPAWN WALK (javap `TrialSpawner.addSpawnParticles`
        // verbatim — the `bipush 20` loop, the THREE nextDouble
        // position draws the SMOKE + flame pair shares, the zero
        // velocity the `dconst_0` trio speaks, the SMOKE companion
        // first the getstatic order keeps): the NORMAL word (data 0)
        // the plain flame kind answers, the OMINOUS word (data 1) the
        // soul twin, the out-of-range words the NORMAL fallback the
        // jar's own guard keeps.
        let mut r = rng();
        let burst = trial_spawner_spawn_burst(10, 20, 30, 0, &mut r);
        assert_eq!(burst.len(), 40, "the twenty pairs the loop answers");
        // THE PAIRS (smoke first the addParticle order keeps — the
        // even members the SMOKE companion, the odd members the
        // decoded flame)
        for (i, p) in burst.iter().enumerate() {
            if i % 2 == 0 {
                assert!(matches!(p.kind, ParticleKind::Smoke), "pair {i} smoke first");
            } else {
                assert!(matches!(p.kind, ParticleKind::Flame), "pair {i} flame");
            }
            // THE POSITION BANDS (the block-center anchor + the ±1.0
            // spread the dsub/dmul constants fold — the jitter adds
            // ±0.05 the flame ctor speaks)
            assert!(
                (9.45..=11.55).contains(&p.x),
                "the x band: {}",
                p.x
            );
            assert!(
                (19.45..=21.55).contains(&p.y),
                "the y band: {}",
                p.y
            );
            assert!(
                (29.45..=31.55).contains(&p.z),
                "the z band: {}",
                p.z
            );
        }
        // THE PAIRED POSITIONS (the same triple feeds BOTH the
        // addParticle calls — the jitter the flame's own chain adds
        // rides WITHIN the ±0.05 the pair keeps)
        for i in 0..20 {
            let smoke = &burst[i * 2];
            let flame = &burst[i * 2 + 1];
            assert!(
                (smoke.x - flame.x).abs() <= 0.05
                    && (smoke.y - flame.y).abs() <= 0.05
                    && (smoke.z - flame.z).abs() <= 0.05,
                "pair {i} shares the position triple"
            );
            // THE CHAIN LAWS: the smoke the ash laws wear (the −0.1
            // gravity, the grey tint the r*0.3 draw gives, the 0.75
            // size fold, the 8..40 lifetime), the flame the rising
            // laws keep (the zero gravity, the white tint, the
            // 12..43 band the +4 tail answers, the *0.01 fold the
            // ±0.001 drift speaks — the d0*0.4*0.01 band)
            assert!(
                (smoke.friction - 0.96).abs() < 1e-9 && (flame.friction - 0.96).abs() < 1e-9,
                "the 0.96 friction the ctors set"
            );
            assert!((smoke.gravity - -0.1).abs() < 1e-9, "the ash pull");
            assert!(flame.gravity.abs() < 1e-9, "the no-gravity law");
            assert_eq!(flame.col, [1.0, 1.0, 1.0], "the white tint");
            assert!(
                (8..=40).contains(&smoke.lifetime),
                "the smoke band: {}",
                smoke.lifetime
            );
            assert!(
                (12..=43).contains(&flame.lifetime),
                "the flame band: {}",
                flame.lifetime
            );
            assert!(
                flame.vx.abs() <= 0.005 && flame.vy.abs() <= 0.011 && flame.vz.abs() <= 0.005,
                "the *0.01 fold the drift damps: {} {} {}",
                flame.vx,
                flame.vy,
                flame.vz
            );
            assert!(smoke.vy >= 0.0, "the rise the +0.1 bump keeps");
        }
        // THE DECODE (the jar's own guard verbatim: `1` the OMINOUS
        // word the SoulFlame kind + the FULL quad the plain Provider
        // registration keeps (the CORRECTIVE BYTE AUDIT the fresh
        // registerProviders dump banks — SOUL_FIRE_FLAME rides the
        // IDENTICAL indy the NORMAL word rides, NO scale the
        // provider's own walk spends), `0` and the out-of-range
        // `2`/`77`/`-1` the NORMAL flame answer — the values[2] edge
        // the vanilla crash wears, the honest arm the NORMAL intent
        // keeps)
        for (data, want_soul) in [(1, true), (0, false), (2, false), (77, false), (-1, false)] {
            let b = trial_spawner_spawn_burst(0, 0, 0, data, &mut rng());
            let soul_count = b
                .iter()
                .filter(|p| matches!(p.kind, ParticleKind::SoulFlame))
                .count();
            let flame_count = b
                .iter()
                .filter(|p| matches!(p.kind, ParticleKind::Flame))
                .count();
            assert_eq!(soul_count, if want_soul { 20 } else { 0 }, "data {data}");
            assert_eq!(flame_count, if want_soul { 0 } else { 20 }, "data {data}");
            if want_soul {
                // the FULL birth band the plain provider keeps — the
                // base_quad_size law's own [0.1, 0.22] band the twin
                // answers verbatim (the blue sprite the only delta)
                for p in b.iter().filter(|p| matches!(p.kind, ParticleKind::SoulFlame)) {
                    assert!(
                        (0.1..0.22).contains(&p.quad_size),
                        "the soul full-quad: {}",
                        p.quad_size
                    );
                }
            }
        }
        // THE DETERMINISM (the same seed the same chain answers)
        let burst2 = trial_spawner_spawn_burst(10, 20, 30, 0, &mut rng());
        assert_eq!(burst.len(), burst2.len(), "the seeded chain replays");
        for (p, p2) in burst.iter().zip(burst2.iter()) {
            assert_eq!(
                (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
                (p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size),
                "the seeded members replay"
            );
        }
        // THE STREAM (the exact draw chain the walk consumes: the
        // THREE nextDouble position draws + the smoke's NINE (the
        // dead base lifetime + the five-draw spread+normalize pair +
        // the size + the tint + the own lifetime) + the flame's
        // FIFTEEN (the dead sprite pick + the dead base lifetime +
        // the spread+normalize pair + the size + the six jitter
        // draws + the own lifetime) = twenty-seven per iteration ×
        // twenty; the NEXT draw differs)
        let mut a = rng();
        let _ = trial_spawner_spawn_burst(10, 20, 30, 0, &mut a);
        let after_walk = a.next();
        let mut b = rng();
        for _ in 0..(20 * 27) {
            b.next();
        }
        assert_eq!(after_walk, b.next(), "the walk's own exact draw count");
    }

    #[test]
    fn the_eject_burst_the_walk_the_args_wear() {
        // THE BURST (the `bipush 20` loop the forty-particle answer
        // wears: twenty SMALL_FLAME + twenty SMOKE pairs, the flame
        // FIRST the walk's own addParticle order keeps)
        let burst = trial_eject_item_burst(3, 4, 5, &mut rng());
        assert_eq!(burst.len(), 40, "the twenty pairs");
        for (i, p) in burst.iter().enumerate() {
            if i % 2 == 0 {
                assert!(
                    matches!(p.kind, ParticleKind::SmallFlame),
                    "the flame first at pair {}: {:?}",
                    i,
                    p.kind
                );
            } else {
                assert!(matches!(p.kind, ParticleKind::Smoke), "the smoke second");
            }
        }
        // THE BANDS (the walk's own folds): the positions the
        // `[0.4, 0.6]` band the +0.4 anchor + the 0.2 dmul answer
        // (the ±0.05 jitter the ctor adds), the flames the scale(0.5F)
        // half the birth band keeps (the 0.1..0.22 plain band x 0.5),
        // the smoke the plain band verbatim, the velocities the arg
        // halves the folds keep (the flame's z the * 0.25 quarter-pull)
        for (i, p) in burst.iter().enumerate() {
            assert!(
                (3.35..3.65).contains(&p.x),
                "the x band at {}: {}",
                i,
                p.x
            );
            assert!(
                (5.35..5.65).contains(&p.z),
                "the z band at {}: {}",
                i,
                p.z
            );
            if i % 2 == 0 {
                assert!(
                    (0.05..0.11).contains(&p.quad_size),
                    "the small half-quad: {}",
                    p.quad_size
                );
            } else {
                assert!(
                    (0.075..0.15).contains(&p.quad_size),
                    "the smoke 0.75-fold band: {}",
                    p.quad_size
                );
            }
        }
        // THE DETERMINISM (the same seed the same chain answers)
        let burst2 = trial_eject_item_burst(3, 4, 5, &mut rng());
        assert_eq!(burst.len(), burst2.len(), "the seeded chain replays");
        for (p, p2) in burst.iter().zip(burst2.iter()) {
            assert_eq!(
                (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
                (p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size),
                "the seeded members replay"
            );
        }
        // THE STREAM (the walk's own draw ORDER the manual replay
        // pins verbatim: per iteration the THREE nextDouble position
        // draws + the THREE nextGaussian velocity draws (the
        // polar-method rejection loop the variable next() count wears
        // — the stream the count itself rides) + the flame's FIFTEEN
        // + the smoke's NINE, the SAME order the burst's own chain
        // keeps — any reorder the values' mismatch speaks; the NEXT
        // draw the replay's tail answers)
        let mut a = rng();
        let mut manual: Vec<Particle> = Vec::with_capacity(40);
        for _ in 0..20 {
            let px = f64::from(3) + 0.4 + a.next_f64() * 0.2;
            let py = f64::from(4) + 0.4 + a.next_f64() * 0.2;
            let pz = f64::from(5) + 0.4 + a.next_f64() * 0.2;
            let ax = a.next_gaussian() * 0.02;
            let ay = a.next_gaussian() * 0.02;
            let az = a.next_gaussian() * 0.02;
            manual.push(eject_flame(px, py, pz, ax, ay, az, &mut a));
            manual.push(eject_smoke(px, py, pz, ax, ay, az, &mut a));
        }
        let after_walk = a.next();
        assert_eq!(manual.len(), burst.len(), "the same forty the replay answers");
        for (p, m) in burst.iter().zip(manual.iter()) {
            assert_eq!(
                (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
                (m.x, m.y, m.z, m.vx, m.vy, m.vz, m.lifetime, m.quad_size),
                "the walk's own exact draw order"
            );
        }
        let mut b = rng();
        for _ in 0..20 {
            let _px = f64::from(3) + 0.4 + b.next_f64() * 0.2;
            let _py = f64::from(4) + 0.4 + b.next_f64() * 0.2;
            let _pz = f64::from(5) + 0.4 + b.next_f64() * 0.2;
            let _ax = b.next_gaussian() * 0.02;
            let _ay = b.next_gaussian() * 0.02;
            let _az = b.next_gaussian() * 0.02;
            let _f = eject_flame(_px, _py, _pz, _ax, _ay, _az, &mut b);
            let _s = eject_smoke(_px, _py, _pz, _ax, _ay, _az, &mut b);
        }
        assert_eq!(after_walk, b.next(), "the replay's own tail draw");
    }

    #[test]
    fn the_vault_bursts_the_cage_walks_the_args_wear() {
        // THE ACTIVATION BURST (the `emitActivationParticles` walk
        // verbatim: the `bipush 20` loop, the cage bands the
        // `randomPosInsideCage` walk speaks, the SMOKE FIRST the
        // walk's own addParticle order keeps — the eject walk's
        // flame-first order the vault's own call pair MIRRORS)
        let burst = vault_activation_burst(3, 4, 5, true, &mut rng());
        assert_eq!(burst.len(), 40, "the twenty pairs");
        for (i, p) in burst.iter().enumerate() {
            if i % 2 == 0 {
                assert!(
                    matches!(p.kind, ParticleKind::Smoke),
                    "the smoke first at pair {}: {:?}",
                    i,
                    p.kind
                );
            } else {
                assert!(
                    matches!(p.kind, ParticleKind::SmallFlame),
                    "the small flame second"
                );
            }
        }
        // THE BANDS (the `randomPosInsideCage` folds: the x/z bands
        // the `0.1 + nd * 0.8` pair answers, the y band the
        // `0.25 + nd * 0.5` fold keeps — the ±0.05 jitter the ctor
        // adds; the flames the scale(0.5F) half the birth band keeps)
        for (i, p) in burst.iter().enumerate() {
            assert!(
                (3.05..3.95).contains(&p.x),
                "the x cage band at {}: {}",
                i,
                p.x
            );
            assert!(
                (4.20..4.80).contains(&p.y),
                "the y cage band at {}: {}",
                i,
                p.y
            );
            assert!(
                (5.05..5.95).contains(&p.z),
                "the z cage band at {}: {}",
                i,
                p.z
            );
            if i % 2 == 1 {
                assert!(
                    (0.05..0.11).contains(&p.quad_size),
                    "the small half-quad: {}",
                    p.quad_size
                );
            }
        }
        // THE ZERO ARGS (the `dconst_0` trio the walk's own
        // addParticle velocity slots speak — the flame the spread
        // * 0.01 fold alone keeps, the smoke the spread * 0.1 the
        // ash ctor speaks)
        for (i, p) in burst.iter().enumerate() {
            if i % 2 == 1 {
                assert!(
                    p.vx.abs() < 0.05 && p.vy.abs() < 0.05 && p.vz.abs() < 0.05,
                    "the zero-arg flame the spread fold alone keeps: {} {} {}",
                    p.vx,
                    p.vy,
                    p.vz
                );
            } else {
                assert!(
                    p.vx.abs() < 0.15 && p.vy.abs() < 0.15 && p.vz.abs() < 0.15,
                    "the zero-arg smoke the spread fold alone keeps"
                );
            }
        }
        // THE SOUL GATE (the `data != 0 ? SOUL_FIRE_FLAME :
        // SMALL_FLAME` ternary the case body reads — the ominous
        // vault the FULL-quad soul word the plain provider banks)
        let soul = vault_activation_burst(3, 4, 5, false, &mut rng());
        let souls = soul
            .iter()
            .filter(|p| matches!(p.kind, ParticleKind::SoulFlame))
            .count();
        assert_eq!(souls, 20, "the soul word the ominous gate arms");
        for p in soul.iter().filter(|p| matches!(p.kind, ParticleKind::SoulFlame)) {
            assert!(
                (0.1..0.22).contains(&p.quad_size),
                "the full quad the plain provider keeps: {}",
                p.quad_size
            );
        }
        // THE DEACTIVATION BURST (the `emitDeactivationParticles`
        // walk verbatim: the center bands the `randomPosCenterOfCage`
        // walk speaks, the ONE flame the smoke-free pair keeps, the
        // gaussian args the * 0.02 fold the Vec3 ctor carries)
        let decay = vault_deactivation_burst(15, 16, 17, true, &mut rng());
        assert_eq!(decay.len(), 20, "the twenty flames");
        for (i, p) in decay.iter().enumerate() {
            assert!(
                matches!(p.kind, ParticleKind::SmallFlame),
                "the flame alone at {}: {:?}",
                i,
                p.kind
            );
            assert!(
                (15.35..15.65).contains(&p.x),
                "the x center band at {}: {}",
                i,
                p.x
            );
            assert!(
                (16.35..16.65).contains(&p.y),
                "the y center band at {}: {}",
                i,
                p.y
            );
            assert!(
                (17.35..17.65).contains(&p.z),
                "the z center band at {}: {}",
                i,
                p.z
            );
            assert!(
                (0.05..0.11).contains(&p.quad_size),
                "the small half-quad the deactivation keeps: {}",
                p.quad_size
            );
            assert!(
                p.vx.abs() < 0.15 && p.vy.abs() < 0.15 && p.vz.abs() < 0.15,
                "the gaussian args the 0.02 scale keeps: {} {} {}",
                p.vx,
                p.vy,
                p.vz
            );
        }
        // THE DETERMINISM (the same seed the same chain answers)
        let burst2 = vault_activation_burst(3, 4, 5, true, &mut rng());
        for (p, p2) in burst.iter().zip(burst2.iter()) {
            assert_eq!(
                (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
                (p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size),
                "the seeded members replay"
            );
        }
        // THE STREAM (the walk's own draw ORDER the manual replay
        // pins verbatim: per iteration the THREE nextDouble position
        // draws + the smoke's NINE + the flame's FIFTEEN — the
        // smoke-first order the activation's own call pair keeps)
        let mut a = rng();
        let mut manual: Vec<Particle> = Vec::with_capacity(40);
        for _ in 0..20 {
            let px = f64::from(3) + 0.1 + a.next_f64() * 0.8;
            let py = f64::from(4) + 0.25 + a.next_f64() * 0.5;
            let pz = f64::from(5) + 0.1 + a.next_f64() * 0.8;
            manual.push(vault_smoke(px, py, pz, &mut a));
            manual.push(vault_flame(px, py, pz, true, 0.0, 0.0, 0.0, &mut a));
        }
        let after_walk = a.next();
        for (p, m) in burst.iter().zip(manual.iter()) {
            assert_eq!(
                (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
                (m.x, m.y, m.z, m.vx, m.vy, m.vz, m.lifetime, m.quad_size),
                "the walk's own exact draw order"
            );
        }
        let mut b = rng();
        for _ in 0..20 {
            let _px = f64::from(3) + 0.1 + b.next_f64() * 0.8;
            let _py = f64::from(4) + 0.25 + b.next_f64() * 0.5;
            let _pz = f64::from(5) + 0.1 + b.next_f64() * 0.8;
            let _s = vault_smoke(_px, _py, _pz, &mut b);
            let _f = vault_flame(_px, _py, _pz, true, 0.0, 0.0, 0.0, &mut b);
        }
        assert_eq!(after_walk, b.next(), "the replay's own tail draw");
        // THE DEACTIVATION'S OWN STREAM (the THREE nextDouble center
        // draws + the THREE nextGaussian velocity draws + the flame's
        // FIFTEEN — the one-particle order the walk keeps)
        let mut c = rng();
        let mut manual_decay: Vec<Particle> = Vec::with_capacity(20);
        for _ in 0..20 {
            let px = f64::from(15) + 0.4 + c.next_f64() * 0.2;
            let py = f64::from(16) + 0.4 + c.next_f64() * 0.2;
            let pz = f64::from(17) + 0.4 + c.next_f64() * 0.2;
            let ax = c.next_gaussian() * 0.02;
            let ay = c.next_gaussian() * 0.02;
            let az = c.next_gaussian() * 0.02;
            manual_decay.push(vault_flame(px, py, pz, true, ax, ay, az, &mut c));
        }
        let after_decay = c.next();
        let mut d = rng();
        for _ in 0..20 {
            let _px = f64::from(15) + 0.4 + d.next_f64() * 0.2;
            let _py = f64::from(16) + 0.4 + d.next_f64() * 0.2;
            let _pz = f64::from(17) + 0.4 + d.next_f64() * 0.2;
            let _ax = d.next_gaussian() * 0.02;
            let _ay = d.next_gaussian() * 0.02;
            let _az = d.next_gaussian() * 0.02;
            let _f = vault_flame(_px, _py, _pz, true, _ax, _ay, _az, &mut d);
        }
        assert_eq!(after_decay, d.next(), "the decay replay's own tail draw");
    }

    #[test]
    fn the_ominous_burst_the_walk_the_args_wear() {
        // THE BURST (javap `TrialSpawner.addBecomeOminousParticles`
        // verbatim — the `bipush 20` loop the forty-particle answer
        // wears: twenty TRIAL_OMEN puffs + twenty SOUL_FIRE_FLAME
        // pairs, the OMEN FIRST the walk's own addParticle order
        // keeps, the same position + velocity args both calls share)
        let mut a = rng();
        let mut j = crate::drops::Lcg::new(0xCAFE);
        let burst = become_ominous_burst(3, 4, 5, &mut a, &mut j);
        assert_eq!(burst.len(), 40, "the twenty pairs");
        for (i, p) in burst.iter().enumerate() {
            if i % 2 == 0 {
                assert!(
                    matches!(
                        p.kind,
                        ParticleKind::Spell { instant: false }
                    ),
                    "the omen first at pair {}: {:?}",
                    i,
                    p.kind
                );
            } else {
                assert!(
                    matches!(p.kind, ParticleKind::SoulFlame),
                    "the soul flame second"
                );
            }
        }
        // THE BANDS (the walk's own folds): the omen's positions the
        // `[x − 0.5, x + 1.5]` band the +0.5 anchor + the (nd − 0.5) ×
        // 2.0 spread answers (NO position jitter the spell chain
        // keeps — the args the ctor's own fields ride), the soul's
        // positions the same band the ±0.05 jitter widens, the omen
        // quad the ×0.75 fold the spell birth band keeps (the
        // 0.1..0.2 plain band x 0.75), the soul the FULL quad the
        // plain provider answers, the lifetimes the 8..40 spell band
        // + the flame band the rising chain keeps
        for (i, p) in burst.iter().enumerate() {
            if i % 2 == 0 {
                assert!(
                    (2.5..4.5).contains(&p.x),
                    "the omen x band at {}: {}",
                    i,
                    p.x
                );
                assert!(
                    (3.5..5.5).contains(&p.y),
                    "the omen y band at {}: {}",
                    i,
                    p.y
                );
                assert!(
                    (4.5..6.5).contains(&p.z),
                    "the omen z band at {}: {}",
                    i,
                    p.z
                );
                assert!(
                    (0.075..0.15).contains(&p.quad_size),
                    "the spell 0.75-fold band: {}",
                    p.quad_size
                );
                assert!(
                    (8..=40).contains(&p.lifetime),
                    "the spell lifetime band: {}",
                    p.lifetime
                );
                assert_eq!(
                    p.gravity, -0.1,
                    "the spell's own gravity the ctor pins"
                );
            } else {
                assert!(
                    (2.45..4.55).contains(&p.x),
                    "the soul x band at {}: {}",
                    i,
                    p.x
                );
                assert!(
                    (3.45..5.55).contains(&p.y),
                    "the soul y band at {}: {}",
                    i,
                    p.y
                );
                assert!(
                    (4.45..6.55).contains(&p.z),
                    "the soul z band at {}: {}",
                    i,
                    p.z
                );
                assert!(
                    (0.1..0.2).contains(&p.quad_size),
                    "the soul FULL quad the plain provider keeps: {}",
                    p.quad_size
                );
                assert_eq!(
                    p.gravity, 0.0,
                    "the flame's own no-gravity law the chain keeps"
                );
            }
            assert_eq!(p.col, [1.0, 1.0, 1.0], "the white tint at pair {i}");
            assert_eq!(p.friction, 0.96, "the 0.96 friction both chains keep");
        }
        // THE DETERMINISM (the same seeds the same chains answer)
        let burst2 = become_ominous_burst(
            3,
            4,
            5,
            &mut rng(),
            &mut crate::drops::Lcg::new(0xCAFE),
        );
        assert_eq!(burst.len(), burst2.len(), "the seeded chains replay");
        for (p, p2) in burst.iter().zip(burst2.iter()) {
            assert_eq!(
                (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
                (p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size),
                "the seeded members replay"
            );
        }
        // THE STREAM (the walk's own draw ORDER the manual replay
        // pins verbatim: per iteration the THREE nextDouble position
        // draws + the THREE nextGaussian velocity draws + the puff's
        // EIGHT level draws (the dead base lifetime + the five-draw
        // spread + the size + the lifetime — the TWO static draws the
        // jitter stream answers) + the flame's FIFTEEN, the SAME
        // order the burst's own chain keeps — any reorder the values'
        // mismatch speaks; the NEXT draw the replay's tail answers)
        let mut a2 = rng();
        let mut j2 = crate::drops::Lcg::new(0xCAFE);
        let mut manual: Vec<Particle> = Vec::with_capacity(40);
        for _ in 0..20 {
            let px = f64::from(3) + 0.5 + (a2.next_f64() - 0.5) * 2.0;
            let py = f64::from(4) + 0.5 + (a2.next_f64() - 0.5) * 2.0;
            let pz = f64::from(5) + 0.5 + (a2.next_f64() - 0.5) * 2.0;
            let ax = a2.next_gaussian() * 0.02;
            let ay = a2.next_gaussian() * 0.02;
            let az = a2.next_gaussian() * 0.02;
            manual.push(become_ominous_puff(px, py, pz, ax, ay, az, &mut a2, &mut j2));
            manual.push(vault_flame(px, py, pz, false, ax, ay, az, &mut a2));
        }
        let after_walk = a2.next();
        assert_eq!(
            manual.len(),
            burst.len(),
            "the same forty the replay answers"
        );
        for (p, m) in burst.iter().zip(manual.iter()) {
            assert_eq!(
                (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
                (m.x, m.y, m.z, m.vx, m.vy, m.vz, m.lifetime, m.quad_size),
                "the draw-order replay pins the chain verbatim"
            );
            assert_eq!(p.kind, m.kind, "the kind the order keeps");
        }
        assert_eq!(after_walk, a.next(), "the walk's own exact draw count");
    }

    #[test]
    fn the_cobweb_burst_the_walk_the_draws_wear() {
        // THE BURST (javap the LevelEventHandler case 3018 — offset
        // 2367 — verbatim: the `bipush 10` loop the ten-member answer
        // wears — NOT the family's twenty — the POOF kind the
        // getstatic arms, the corner-anchored cube the bare
        // nextDouble keeps)
        let mut a = rng();
        let burst = cobweb_spawn_burst((7, 8, 9), &mut a);
        assert_eq!(burst.len(), 10, "the ten-iteration bound");
        for p in &burst {
            assert!(
                matches!(p.kind, ParticleKind::Poof),
                "the POOF the case's own getstatic arms: {:?}",
                p.kind
            );
        }
        // THE BANDS (the walk's own folds): the positions the [x, x+1)
        // corner band the bare nextDouble keeps — NO +0.5 center the
        // family's other walks carry, NO spread scale the dmul never
        // arms; the velocities the gaussians' * 0.02 fold rides (the
        // ctor's ±0.05 jitter the f32 pairs widen — the [−0.08, 0.08]
        // band the jitter's own band the gaussians' tail carries);
        // the grey the `nf * 0.3 + 0.7` band speaks, the quad the
        // `0.1 * (nf * nf * 6 + 1)` band keeps, the lifetime the
        // `16.0 / (nf * 0.8 + 0.2) + 2` walk answers, the -0.1
        // gravity + the 0.9 friction the ctor law pins
        for (i, p) in burst.iter().enumerate() {
            assert!(
                (7.0..8.0).contains(&p.x),
                "the corner x band at {}: {}",
                i,
                p.x
            );
            assert!(
                (8.0..9.0).contains(&p.y),
                "the corner y band at {}: {}",
                i,
                p.y
            );
            assert!(
                (9.0..10.0).contains(&p.z),
                "the corner z band at {}: {}",
                i,
                p.z
            );
            assert!(
                p.vx.abs() < 0.5 && p.vy.abs() < 0.5 && p.vz.abs() < 0.5,
                "the gaussian*0.02 + ±0.05 jitter sanity band at {}: {} {} {}",
                i,
                p.vx,
                p.vy,
                p.vz
            );
            assert!(
                (0.7..1.0).contains(&p.col[0])
                    && (0.7..1.0).contains(&p.col[1])
                    && (0.7..1.0).contains(&p.col[2]),
                "the grey tint band at {}: {:?}",
                i,
                p.col
            );
            assert!(
                (0.1..0.7).contains(&p.quad_size),
                "the poof quad band at {}: {}",
                i,
                p.quad_size
            );
            assert!(
                (18..=82).contains(&p.lifetime),
                "the poof lifetime band at {}: {}",
                i,
                p.lifetime
            );
            assert_eq!(p.gravity, -0.1, "the poof's own gravity the ctor pins");
            assert_eq!(p.friction, 0.9, "the poof's own friction the ctor pins");
        }
        // THE DETERMINISM (the same seed the same chain answers)
        let burst2 = cobweb_spawn_burst((7, 8, 9), &mut rng());
        assert_eq!(burst.len(), burst2.len(), "the seeded chain replays");
        for (p, p2) in burst.iter().zip(burst2.iter()) {
            assert_eq!(
                (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
                (p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size),
                "the seeded members replay"
            );
        }
        // THE STREAM (the walk's own draw ORDER the manual replay
        // pins verbatim: per iteration the THREE nextGaussian
        // velocity draws FIRST (the dstore trio the case body's own
        // pc 2377-2414 order keeps) + the THREE nextDouble position
        // draws the arg walk interleaves + the POOF chain's own EIGHT
        // (the base lifetime + the THREE jitters + the grey + the
        // size's TWO + the lifetime) — any reorder the values'
        // mismatch speaks; the NEXT draw the replay's tail answers)
        let mut a2 = rng();
        let mut manual: Vec<Particle> = Vec::with_capacity(10);
        for _ in 0..10 {
            let vx = a2.next_gaussian() * 0.02;
            let vy = a2.next_gaussian() * 0.02;
            let vz = a2.next_gaussian() * 0.02;
            let px = f64::from(7) + a2.next_f64();
            let py = f64::from(8) + a2.next_f64();
            let pz = f64::from(9) + a2.next_f64();
            let r_base_life = a2.next_f32();
            let r_jx = a2.next_f32();
            let r_jy = a2.next_f32();
            let r_jz = a2.next_f32();
            let r_grey = a2.next_f32();
            let r_s1 = a2.next_f32();
            let r_s2 = a2.next_f32();
            let r_life = a2.next_f32();
            manual.push(poof(
                px, py, pz, vx, vy, vz, r_base_life, r_jx, r_jy, r_jz, r_grey, r_s1, r_s2,
                r_life,
            ));
        }
        let after_walk = a2.next();
        assert_eq!(
            manual.len(),
            burst.len(),
            "the same ten the replay answers"
        );
        for (p, m) in burst.iter().zip(manual.iter()) {
            assert_eq!(
                (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size),
                (m.x, m.y, m.z, m.vx, m.vy, m.vz, m.lifetime, m.quad_size),
                "the draw-order replay pins the chain verbatim"
            );
            assert_eq!(p.kind, m.kind, "the kind the order keeps");
        }
        assert_eq!(after_walk, a.next(), "the walk's own exact draw count");
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

    #[test]
    fn the_dragon_breath_puff_the_power_fold_the_frames_wear() {
        // THE DRAGON-BREATH SPLASH PUFF'S OWN SPAWN (javap
        // `DragonBreathParticle` + `$Provider.createParticle` over the
        // 26.2 jar — the case-2006 arm's own walk feeds): the draw
        // chain the case's trio opens (the f6/f7/d10 the walk's own
        // order keeps), the ctor chain the helper spends (the dead
        // base lifetime, the size draw, the r/g/b tint draws the
        // (0,0)-green consumes, the lifetime the f64-domain fold
        // answers), the setPower fold the provider's tail applies.
        let mut stream = crate::drops::Lcg::new(0xD2A6_10CA);
        // THE CASE'S OWN DRAWS (the f6/f7/d10 trio the bytecode's own
        // order keeps) + the cos/sin folds the narrow shape speaks
        let f6 = stream.next_f32() * 4.0f32;
        let f7 = stream.next_f32() * 6.2831855f32;
        let d10 = 0.01f64 + stream.next_f64() * 0.5f64;
        let d8 = ((f7 as f64).cos() as f32 * f6) as f64;
        let d12 = ((f7 as f64).sin() as f32 * f6) as f64;
        let p = dragon_breath_splash_puff(
            10.0 + d8 * 0.1,
            20.3,
            30.0 + d12 * 0.1,
            f6,
            d8,
            d10,
            d12,
            &mut stream,
        );
        assert!(matches!(p.kind, ParticleKind::DragonBreath));
        // THE SET-POWER FOLD (javap `Particle.setPower` verbatim — the
        // x/z the plain multiply, the y the `(v - 0.1) * p + 0.1` the
        // base's own 0.1 floor keeps)
        assert_eq!(p.vx, d8 * f64::from(f6), "the x the plain multiply rides");
        assert_eq!(p.vy, (d10 - 0.1) * f64::from(f6) + 0.1, "the y the floor keeps");
        assert_eq!(p.vz, d12 * f64::from(f6), "the z the plain multiply rides");
        // THE SPAWN FOLD (the raw corner reads the case's own args
        // place — the d8*0.1 the x speaks, the +0.3 the y keeps)
        assert_eq!(p.x, 10.0 + d8 * 0.1);
        assert_eq!(p.y, 20.3);
        assert_eq!(p.z, 30.0 + d12 * 0.1);
        // THE TINT LAW (the purple the ctor's own ranges carry — the
        // r/b inside the Mth.nextFloat bands, the green the (0,0)
        // range keeps, the draw the stream still spends)
        assert!(
            (0.7176470..=0.8745099).contains(&p.col[0]),
            "the red band: {}",
            p.col[0]
        );
        assert_eq!(p.col[1], 0.0, "the green the zero-width range keeps");
        assert!(
            (0.8235294..=0.9764706).contains(&p.col[2]),
            "the blue band: {}",
            p.col[2]
        );
        // THE LIFETIME LAW (the f64 domain the f2d order speaks — the
        // edge r=0 answers 100, the r→1 arm 20, the f32-first folds'
        // own difference the one order wears)
        assert_eq!(dragon_breath_lifetime(0.0), 100, "the r=0 edge");
        assert_eq!(dragon_breath_lifetime(0.9999999), 20, "the r→1 arm");
        assert!((20..=100).contains(&p.lifetime), "the band: {}", p.lifetime);
        // THE FIELDS (the no-gravity law the override's own tick
        // keeps, the 0.96 friction, the 0.75 fold's own band)
        assert_eq!(p.gravity, 0.0);
        assert_eq!(p.friction, 0.96);
        assert!(
            (0.075..=0.15).contains(&p.quad_size),
            "the 0.75 fold's own band: {}",
            p.quad_size
        );
        // THE GROWTH (the 32x clamp law the crit speaks — the age-0
        // bloom the clamp keeps at the point)
        assert_eq!(p.half(), 0.0, "the age-0 bloom the 32x clamp answers");
        // the pre-tick state the determinism + the tick walks read
        let pre = (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size, p.col);
        // THE TICK (the override the base law never rides): no
        // gravity lane, the y the friction-free rise keeps, the x/z
        // the 0.96 folds ride, the move the plain translate answers
        let (x0, y0, z0, vx0, vy0, vz0) = (p.x, p.y, p.z, p.vx, p.vy, p.vz);
        let mut t = p;
        assert!(t.tick_dragon_breath(), "the age walk the lifetime gates");
        assert_eq!(t.age, 1);
        assert_eq!(t.vx, vx0 * 0.96, "the x friction the tick's own lane speaks");
        assert_eq!(t.vy, vy0, "the y the friction-free rise keeps");
        assert_eq!(t.vz, vz0 * 0.96, "the z friction the tick's own lane speaks");
        assert_eq!(t.x, x0 + vx0, "the move the plain translate answers");
        assert_eq!(t.y, y0 + vy0, "the move the plain translate answers");
        assert_eq!(t.z, z0 + vz0, "the move the plain translate answers");
        // the lifetime the removal walk answers (the 20..100 band's
        // own edge — the age the removal's own tick reaches)
        let mut dying =
            dragon_breath_splash_puff(0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, &mut stream);
        dying.lifetime = 3;
        assert!(dying.tick_dragon_breath());
        assert!(dying.tick_dragon_breath());
        assert!(!dying.tick_dragon_breath(), "the age the removal walk answers");
        // THE FRAMES (the get law's list index, the json's ASCENDING
        // order the +5 file offset keeps — the generic_5/6/7 the
        // dragon_breath.json lists; the 3-frame walk the removal age
        // NEVER renders — the index 2 the age >= lifetime arm alone
        // reaches, the min clamp the honest shape keeps)
        assert_eq!(dragon_breath_frame(0, 100), 0, "the age-0 index");
        assert_eq!(dragon_breath_frame(50, 100), 1, "the mid-life index");
        assert_eq!(dragon_breath_frame(99, 100), 1, "the last rendered index");
        assert_eq!(dragon_breath_frame(100, 100), 2, "the clamp the removal never renders");
        assert_eq!(dragon_breath_file(0), 5, "the list's own first file");
        assert_eq!(dragon_breath_file(2), 7, "the list's own last file");
        // THE DETERMINISM (the same seed the same chain answers)
        let mut a = crate::drops::Lcg::new(0xD2A6_10CA);
        let f6b = a.next_f32() * 4.0f32;
        let f7b = a.next_f32() * 6.2831855f32;
        let d10b = 0.01f64 + a.next_f64() * 0.5f64;
        let d8b = ((f7b as f64).cos() as f32 * f6b) as f64;
        let d12b = ((f7b as f64).sin() as f32 * f6b) as f64;
        let p2 = dragon_breath_splash_puff(
            10.0 + d8b * 0.1,
            20.3,
            30.0 + d12b * 0.1,
            f6b,
            d8b,
            d10b,
            d12b,
            &mut a,
        );
        assert_eq!(
            pre, (p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size, p2.col),
            "the seeded chain replays"
        );
        // THE STREAM (the nine draws the chain consumes — the f6/f7/
        // d10 case trio, the dead base lifetime, the size, the three
        // tint draws, the own lifetime; the tenth draw differs)
        let mut a = crate::drops::Lcg::new(0xD2A6_10CA);
        let f6c = a.next_f32() * 4.0f32;
        let f7c = a.next_f32() * 6.2831855f32;
        let d10c = 0.01f64 + a.next_f64() * 0.5f64;
        let d8c = ((f7c as f64).cos() as f32 * f6c) as f64;
        let d12c = ((f7c as f64).sin() as f32 * f6c) as f64;
        let _ = dragon_breath_splash_puff(0.0, 0.0, 0.0, f6c, d8c, d10c, d12c, &mut a);
        let after_nine = a.next();
        let mut b = crate::drops::Lcg::new(0xD2A6_10CA);
        for _ in 0..9 {
            b.next();
        }
        assert_eq!(after_nine, b.next(), "the dragon chain's own nine draws");
    }

    #[test]
    fn the_cloud_puff_the_lift_the_frames_wear() {
        // THE WATER-EVAPORATING STEAM PUFF'S OWN SPAWN (javap
        // `PlayerCloudParticle` + `$Provider.createParticle` over the
        // 26.2 jar — the case-2009 arm's own walk feeds): the draw
        // chain the case's x/z pair opens (the ctor chain the four
        // draws — the dead base lifetime, the size, THE ONE tint
        // draw the shared f16 keeps, the own lifetime the ×2.5 fold
        // answers), the zero velocity the dconst_0 trio passes.
        // THE LIFETIME EDGES (the f64-domain fold the helper pins):
        assert_eq!(cloud_lifetime(0.0), 65, "the r=0 edge (8/0.3 → 26 × 2.5)");
        assert_eq!(cloud_lifetime(0.9999999), 17, "the r→1 arm (8/1.1 → 7 × 2.5 → 17.5)");
        // the seeded chain: the two case draws (x, z) then the ctor's
        // four
        let mut stream = crate::drops::Lcg::new(0xC10D_2009);
        let rx = stream.next_f64();
        let rz = stream.next_f64();
        let p = cloud_puff(10.0 + rx, 21.2, 30.0 + rz, &mut stream);
        let pre = (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size, p.col);
        assert!(matches!(p.kind, ParticleKind::Cloud), "the CLOUD type");
        // THE ZERO VELOCITY TRIO (the dconst_0 the case passes — the
        // ×0.1 fold the zero keeps zero, the + input the same)
        assert_eq!(p.vx, 0.0, "the zero vx");
        assert_eq!(p.vy, 0.0, "the zero vy");
        assert_eq!(p.vz, 0.0, "the zero vz");
        // THE CTOR LAWS (the 0.96 friction the all-three-axes body
        // speaks, the no-gravity field the base tick keeps silent,
        // the ×1.875 fold the size ride answers, the shared f16 the
        // r/g/b trio keeps)
        assert!((p.friction - 0.96).abs() < 1e-9, "the 0.96 friction");
        assert!((p.gravity).abs() < 1e-9, "the no-gravity law");
        assert!(p.lifetime >= 17 && p.lifetime <= 65, "the band: {}", p.lifetime);
        let expect_tint = p.col[0];
        assert!(
            (0.7..=1.0).contains(&expect_tint),
            "the tint band the 1.0 - r*0.3 draw gives: {expect_tint}"
        );
        assert_eq!(p.col[1], expect_tint, "the shared f16 the g keeps");
        assert_eq!(p.col[2], expect_tint, "the shared f16 the b keeps");
        assert!(
            p.quad_size > 0.1875 && p.quad_size < 0.375,
            "the ×1.875 band the size ride gives: {}",
            p.quad_size
        );
        // THE FRAMES (the cloud.json's DESCENDING list the file fold
        // speaks — the same law the smoke lane wears, the 8 generic
        // frames the poof bank loads)
        assert_eq!(poof_frame(0, p.lifetime), 0, "the age-0 frame");
        assert_eq!(generic_file_index(0), 7, "the list's own first file (generic_7)");
        assert_eq!(generic_file_index(7), 0, "the list's own last file (generic_0)");
        // THE TICK (the base law the zero velocity keeps put — the
        // friction no-ops the zero rides, the no-player arm the
        // Option keeps silent)
        let mut t = cloud_puff(0.0, 0.0, 0.0, &mut stream);
        t.lifetime = 100;
        assert!(t.tick_cloud(None), "the alive tick");
        assert_eq!(
            (t.x, t.y, t.z),
            (0.0, 0.0, 0.0),
            "the zero velocity the put keeps"
        );
        // THE REMOVAL (the age walk the lifetime gates)
        let mut dying = cloud_puff(0.0, 0.0, 0.0, &mut stream);
        dying.lifetime = 2;
        assert!(dying.tick_cloud(None), "the first tick the alive gate answers");
        assert!(!dying.tick_cloud(None), "the age the removal walk answers");
        // THE LIFT (the override's own tail): the player below the
        // puff within 2.0 — the pose the 0.2 blend eases toward the
        // feet, the vy the player's own rise the blend follows
        let mut lifted = cloud_puff(0.0, 1.2, 0.0, &mut stream);
        lifted.lifetime = 100;
        assert!(lifted.tick_cloud(Some((0.0, 0.0, 0.0, 0.5))));
        assert!(
            (lifted.y - (1.2 + (0.0 - 1.2) * 0.2)).abs() < 1e-12,
            "the 0.2 blend the pose eases: {}",
            lifted.y
        );
        assert!(
            (lifted.vy - (0.0 + (0.5 - 0.0) * 0.2)).abs() < 1e-12,
            "the vy the blend follows: {}",
            lifted.vy
        );
        // THE PLAYER ABOVE (the y > fold the lift never arms)
        let mut above = cloud_puff(0.0, 1.2, 0.0, &mut stream);
        above.lifetime = 100;
        assert!(above.tick_cloud(Some((0.0, 2.0, 0.0, 0.0))));
        assert!(
            (above.y - 1.2).abs() < 1e-12,
            "the player above the lift never arms: {}",
            above.y
        );
        // THE RANGE GATE (the distance² the strict < keeps — the
        // 2.0-beyond the read never arms)
        let mut far = cloud_puff(0.0, 1.2, 0.0, &mut stream);
        far.lifetime = 100;
        assert!(far.tick_cloud(Some((2.1, 0.0, 0.0, 0.0))));
        assert!(
            (far.y - 1.2).abs() < 1e-12,
            "the range gate the lift refuses"
        );
        // THE DETERMINISM (the same seed the same chain answers)
        let mut a = crate::drops::Lcg::new(0xC10D_2009);
        let rx2 = a.next_f64();
        let rz2 = a.next_f64();
        let p2 = cloud_puff(10.0 + rx2, 21.2, 30.0 + rz2, &mut a);
        assert_eq!(
            pre,
            (p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size, p2.col),
            "the seeded chain replays"
        );
        // THE STREAM (the six draws the per-puff chain consumes — the
        // x/z case pair, the dead base lifetime, the size, the tint,
        // the own lifetime; the seventh draw differs)
        let mut a = crate::drops::Lcg::new(0xC10D_2009);
        let _ = a.next_f64();
        let _ = a.next_f64();
        let _ = cloud_puff(0.0, 0.0, 0.0, &mut a);
        let after_six = a.next();
        let mut b = crate::drops::Lcg::new(0xC10D_2009);
        for _ in 0..6 {
            b.next();
        }
        assert_eq!(after_six, b.next(), "the cloud chain's own six draws");
    }

    #[test]
    fn the_portal_ring_puff_the_swirl_the_grow_wear() {
        // THE EYE-OF-ENDER LANDING'S OWN RING (javap `PortalParticle`
        // + `$Provider.createParticle` over the 26.2 jar — the
        // case-2003 arm's own walk feeds): the four draws the
        // per-puff chain consumes (the frame pick the SpriteSet.get
        // law answers FIRST, the size, THE ONE tint draw the f15
        // body spends, the lifetime the +40 write answers), the
        // verbatim velocity the ctor's plain stores keep, the swirl
        // the exact-from-Start recompute speaks, the growth the
        // getQuadSize fold gives. THE EYE SHARD (javap
        // `BreakingItemParticle` — the SAME chain the splash shard
        // rides, the bound-1 pick the stream still spends).
        // THE BLEND EDGES (the f1 fold the tick body pins):
        assert!((portal_swirl_blend(0.0) - 1.0).abs() < 1e-6, "the t=0 blend (the birth rides the full velocity)");
        assert!((portal_swirl_blend(0.5) - 1.0).abs() < 1e-6, "the mid swing the blend holds (1 - (0.5 - 0.5))");
        assert!((portal_swirl_blend(1.0)).abs() < 1e-6, "the t=1 blend (the particle lands ON its Start)");
        // THE GROW EDGES (the getQuadSize override's own fold):
        assert!((portal_grow(0.0)).abs() < 1e-6, "the quad starts a point");
        assert!((portal_grow(1.0) - 1.0).abs() < 1e-6, "the full size the life's end reaches");
        assert!((portal_grow(0.5) - 0.75).abs() < 1e-6, "the mid growth (1 - 0.25)");
        // THE RING PUFF (the seeded chain — the frame pick, the size,
        // the tint, the lifetime: the four draws the ctor chain
        // consumes)
        let mut stream = crate::drops::Lcg::new(0xC10D_2003);
        let p = portal_ring_puff(10.0, 20.0, 30.0, -5.0, 0.0, 0.0, &mut stream);
        assert!(
            matches!(p.kind, ParticleKind::Portal { sx: 10.0, sy: 20.0, sz: 30.0, .. }),
            "the Start the variant carries (the ctor's xStart/yStart/zStart)"
        );
        assert!((p.vx - -5.0).abs() < 1e-9, "the verbatim vx the ctor stores");
        assert!((p.vy).abs() < 1e-9, "the verbatim vy (the zero the ring carries)");
        assert!((p.vz).abs() < 1e-9, "the verbatim vz");
        // THE CTOR LAWS (the 40..49 lifetime the +40 write gives, the
        // 0.05..0.07 band the 0.1 × (r·0.2 + 0.5) fold gives, the
        // purple the r·0.9 / g·0.3 / b trio keeps — the b the tint
        // itself, the r/g the SAME draw's folds re-read)
        assert!((40..=49).contains(&p.lifetime), "the lifetime band: {}", p.lifetime);
        assert!(
            p.quad_size > 0.05 && p.quad_size < 0.07,
            "the ring's own size band: {}",
            p.quad_size
        );
        let b = p.col[2];
        assert!((0.4..=1.0).contains(&b), "the tint band the r*0.6+0.4 draw gives: {b}");
        assert!((p.col[0] - b * 0.9).abs() < 1e-6, "the r the ×0.9 fold keeps");
        assert!((p.col[1] - b * 0.3).abs() < 1e-6, "the g the ×0.3 fold keeps");
        // THE FRICTION FIELD (the vanilla default the swirl never
        // reads — the recompute the field's own silence keeps)
        assert!((p.friction - 0.98).abs() < 1e-9, "the dead field the base default keeps");
        // THE SWIRL (the tick's own recompute — the position
        // Start + velocity × blend, the y-lift (1 − t) the loc2 copy
        // adds: the first tick the blend 1−(2t²−t) at t=1/lifetime
        // folds, the pull-in the exact law keeps)
        let mut t = portal_ring_puff(10.0, 20.0, 30.0, -5.0, 0.0, 0.0, &mut stream);
        t.lifetime = 100;
        assert!(t.tick_portal(), "the alive tick");
        let t1 = 1.0f32 / 100.0f32;
        let blend1 = portal_swirl_blend(t1);
        assert!(
            (t.x - (10.0 + -5.0 * f64::from(blend1))).abs() < 1e-9,
            "the x the Start + vx × blend law keeps: {}",
            t.x
        );
        assert!(
            (t.y - (20.0 + 0.0 + f64::from(1.0f32 - t1))).abs() < 1e-9,
            "the y the lift (1 − t) the loc2 copy adds: {}",
            t.y
        );
        assert!((t.z - 30.0).abs() < 1e-9, "the z the zero vz the Start keeps");
        // the mid-life swing (the age 50 the blend ~1 the half point
        // answers, the position the Start's neighborhood keeps)
        let mut mid = portal_ring_puff(10.0, 20.0, 30.0, -5.0, 0.0, 0.0, &mut stream);
        mid.lifetime = 100;
        for _ in 0..50 {
            mid.tick_portal();
        }
        assert!(
            (mid.x - (10.0 + -5.0 * f64::from(portal_swirl_blend(0.5)))).abs() < 1e-9,
            "the mid swing the exact-from-Start law keeps: {}",
            mid.x
        );
        // THE REMOVAL (the age walk the lifetime gates — the removal
        // tick the swirl never walks)
        let mut dying = portal_ring_puff(0.0, 0.0, 0.0, 1.0, 0.0, 0.0, &mut stream);
        dying.lifetime = 2;
        assert!(dying.tick_portal(), "the first tick the alive gate answers");
        assert!(!dying.tick_portal(), "the age the removal walk answers");
        // THE EYE SHARD (the SAME chain the splash shard rides — the
        // bound-1 pick the stream spends FIRST, the nine draws the
        // ctor chain consumes, the gravity 1.0, the /2 size fold, the
        // white tint the item pixels wear)
        let mut shard_stream = crate::drops::Lcg::new(0xC10D_2004);
        let s = breaking_eye_shard(1.0, 2.0, 3.0, 0.05, 0.02, -0.05, &mut shard_stream);
        assert!(matches!(s.kind, ParticleKind::EyeItem), "the ITEM type the eye wears");
        assert!((s.gravity - 1.0).abs() < 1e-9, "the gravity 1.0 the ctor passes");
        assert!((s.friction - 0.98).abs() < 1e-9, "the base 0.98 friction");
        assert_eq!(s.col, [1.0, 1.0, 1.0], "the white tint the item pixels wear");
        assert!(
            s.quad_size > 0.05 && s.quad_size < 0.1,
            "the /2 fold the base band halves: {}",
            s.quad_size
        );
        assert!((s.vx - 0.05).abs() < 1.0, "the wire velocity the ctor's fold rides");
        // the shard's own draw count (the bound-1 pick + the nine
        // draws the ctor chain consumes — the next draw the tenth
        // after)
        let mut a = crate::drops::Lcg::new(0xC10D_2004);
        let _ = breaking_eye_shard(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, &mut a);
        let after_ten = a.next();
        let mut b = crate::drops::Lcg::new(0xC10D_2004);
        for _ in 0..10 {
            b.next();
        }
        assert_eq!(after_ten, b.next(), "the shard chain's own ten draws");
        // THE DETERMINISM (the same seed the same chain answers)
        let mut a = crate::drops::Lcg::new(0xC10D_2003);
        let p2 = portal_ring_puff(10.0, 20.0, 30.0, -5.0, 0.0, 0.0, &mut a);
        assert_eq!(
            (p2.x, p2.y, p2.z, p2.lifetime, p2.quad_size, p2.col),
            (p.x, p.y, p.z, p.lifetime, p.quad_size, p.col),
            "the seeded chain replays"
        );
    }

    #[test]
    fn the_explosion_puff_the_stationary_frames_wear() {
        // THE DRAGON BLOCK-BREAK BURST'S OWN SPAWN (javap
        // `HugeExplosionParticle` + `$Provider.createParticle` over
        // the 26.2 jar — the case-2008 arm's own walk feeds): the
        // draw chain the case's ZERO-arg call opens (the ctor chain
        // the four draws — the dead base lifetime, the dead base
        // quad size the ctor's own write overrides, the `6 +
        // nextInt(4)` lifetime, THE ONE tint draw the shared f11
        // keeps), the zero velocity the dconst_0 trio passes.
        // THE FRAME EDGES (the get law the helper pins):
        assert_eq!(explosion_frame(0, 6), 0, "the age-0 frame");
        assert_eq!(explosion_frame(0, 9), 0, "the age-0 frame the long life keeps");
        assert_eq!(explosion_frame(5, 6), 12, "the 5*15/6 the short life walks");
        assert_eq!(explosion_frame(8, 9), 13, "the 8*15/9 the long life walks");
        assert_eq!(explosion_frame(0, 0), 0, "the zero lifetime the clamp keeps");
        // the seeded chain: the ctor's four draws (the case's own
        // position folds the fixed constants keep — no draws ride)
        let mut stream = crate::drops::Lcg::new(0xD2A6_2008u64 ^ 0xC1A5);
        let p = explosion_puff(-120.5, 64.5, 77.5, 0.0, &mut stream);
        let pre = (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size, p.col);
        assert!(
            matches!(p.kind, ParticleKind::Explosion),
            "the EXPLOSION type the provider answers"
        );
        // THE ZERO VELOCITY TRIO (the dconst_0 the case passes — the
        // provider drops the y/z speeds the walk never reads)
        assert_eq!(p.vx, 0.0, "the zero vx");
        assert_eq!(p.vy, 0.0, "the zero vy");
        assert_eq!(p.vz, 0.0, "the zero vz");
        // THE CTOR LAWS (the vanilla friction default the override
        // never reads, the no-gravity field, the 6..9 band the
        // nextInt(4) draw gives, the 2.0 quad the zero-speed fold
        // keeps, the shared f11 the r/g/b trio keeps)
        assert!((p.friction - 0.98).abs() < 1e-9, "the 0.98 field default");
        assert!((p.gravity).abs() < 1e-9, "the no-gravity law");
        assert!(p.lifetime >= 6 && p.lifetime <= 9, "the band: {}", p.lifetime);
        assert!((p.quad_size - 2.0).abs() < 1e-9, "the 2.0 the zero speed keeps");
        let expect_tint = p.col[0];
        assert!(
            (0.4..=1.0).contains(&expect_tint),
            "the tint band the r*0.6+0.4 draw gives: {expect_tint}"
        );
        assert_eq!(p.col[1], expect_tint, "the shared f11 the g keeps");
        assert_eq!(p.col[2], expect_tint, "the shared f11 the b keeps");
        // THE STATIONARY TICK (the override's own body — the prev the
        // head saves, the age walk, no motion the zero velocity and
        // the override's own silence keep)
        let mut t = explosion_puff(0.0, 0.0, 0.0, 0.0, &mut stream);
        t.lifetime = 100;
        assert!(t.tick_explosion(), "the alive tick");
        assert_eq!(
            (t.x, t.y, t.z),
            (0.0, 0.0, 0.0),
            "the stationary walk the pose keeps"
        );
        assert_eq!(
            t.prev,
            Some((0.0, 0.0, 0.0)),
            "the prev-trio the tick head saves"
        );
        // THE REMOVAL (the age walk the lifetime gates)
        let mut dying = explosion_puff(0.0, 0.0, 0.0, 0.0, &mut stream);
        dying.lifetime = 2;
        assert!(
            dying.tick_explosion(),
            "the first tick the alive gate answers"
        );
        assert!(!dying.tick_explosion(), "the age the removal walk answers");
        // THE DETERMINISM (the same seed the same chain answers)
        let mut a = crate::drops::Lcg::new(0xD2A6_2008u64 ^ 0xC1A5);
        let p2 = explosion_puff(-120.5, 64.5, 77.5, 0.0, &mut a);
        assert_eq!(
            pre,
            (p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.lifetime, p2.quad_size, p2.col),
            "the seeded chain replays"
        );
        // THE STREAM (the four draws the per-puff chain consumes —
        // the dead base lifetime, the dead quad, the nextInt(4), the
        // tint; the fifth draw differs)
        let mut a = crate::drops::Lcg::new(0xD2A6_2008u64 ^ 0xC1A5);
        let _ = explosion_puff(0.0, 0.0, 0.0, 0.0, &mut a);
        let after_four = a.next();
        let mut b = crate::drops::Lcg::new(0xD2A6_2008u64 ^ 0xC1A5);
        for _ in 0..4 {
            b.next();
        }
        assert_eq!(after_four, b.next(), "the explosion chain's own four draws");
    }

    #[test]
    fn the_gateway_seed_the_six_children_the_shrink_wears() {
        // THE GATEWAY SEED'S OWN LAW (javap `HugeExplosionSeedParticle`
        // over the 26.2 jar): the invisible NoRender engine the
        // 8-tick countdown banks, the SIX children the per-tick loop
        // spawns (the (r - r) * 4.0 triple-offset pair each carries,
        // the size input the seed's own age/lifetime walk feeds), the
        // removal the age == lifetime gate answers — the 48-child
        // burst the eight ticks spend, the death tick the last six
        // still spawns.
        let seed = gateway_explosion_seed(-4.5, 64.5, 100.5);
        assert!(
            matches!(seed.kind, ParticleKind::ExplosionSeed),
            "the seed kind the invisible engine banks"
        );
        assert_eq!((seed.x, seed.y, seed.z), (-4.5, 64.5, 100.5));
        assert_eq!(seed.lifetime, 8, "the ctor's own countdown write");
        assert_eq!(seed.age, 0, "the birth tick the zero keeps");
        assert_eq!(seed.quad_size, 0.0, "the invisible engine no quad banks");
        // THE SIX-CHILD WALK (the per-tick loop the `i < 6` bound
        // keeps): the first tick rides the size input 0.0 (the full
        // 2.0 quad the children's ctor folds), the offsets the ±4.0
        // band the (r - r) * 4.0 pair gives
        let mut seed2 = gateway_explosion_seed(100.0, 100.0, 100.0);
        let mut stream = crate::drops::Lcg::new(0x3000_3000u64 ^ 0xC1A5);
        let mut out: Vec<(f64, f64, f64, f32)> = Vec::new();
        assert!(seed2.tick_explosion_seed(&mut stream, &mut out), "tick 1 alive");
        assert_eq!(out.len(), 6, "the six children the first tick banks");
        for (cx, cy, cz, ss) in &out {
            assert!(
                (96.0..=104.0).contains(cx)
                    && (96.0..=104.0).contains(cy)
                    && (96.0..=104.0).contains(cz),
                "the offset the ±4.0 band answers: {cx} {cy} {cz}"
            );
            assert_eq!(*ss, 0.0, "the birth tick's size input the zero keeps");
        }
        // THE WALK TO THE DEATH TICK (the age walk feeds the size
        // input — the `2.0 * (1.0 - xa * 0.5)` law the children's
        // ctor rides): seven alive ticks the first seven answers, the
        // eighth the false the removal walk speaks — the death tick
        // the last six children STILL spawn (the size input the 7/8
        // the age walk banks). The whole burst the eight ticks spend:
        // the 48 children the six-per-tick loop gives.
        let mut ticks_alive = 1;
        let mut total_children = out.len();
        let mut death_children: Option<Vec<(f64, f64, f64, f32)>> = None;
        for _ in 1..8 {
            let mut out2: Vec<(f64, f64, f64, f32)> = Vec::new();
            if seed2.tick_explosion_seed(&mut stream, &mut out2) {
                ticks_alive += 1;
                total_children += out2.len();
            } else {
                total_children += out2.len();
                death_children = Some(out2);
                break;
            }
        }
        assert_eq!(ticks_alive, 7, "the seven alive ticks the countdown walks");
        assert_eq!(total_children, 48, "the 48-child burst the eight ticks spend");
        let death = death_children.expect("the death tick the eighth walk answers");
        assert_eq!(death.len(), 6, "the death tick the last six still spawns");
        assert!((death[0].3 - 0.875).abs() < 1e-6, "the 7/8 size input");
        // THE SHRINK'S OWN FOLD (the `2.0 * (1.0 - xa * 0.5)` law the
        // child ctor rides): the 0.875 input the 1.125 quad answers,
        // the zero input the 2.0 full quad keeps
        let mut a = crate::drops::Lcg::new(0x3000_3000u64 ^ 0xC1A5);
        let child = explosion_puff(0.0, 0.0, 0.0, 0.875, &mut a);
        assert!((child.quad_size - 1.125).abs() < 1e-6, "the shrink the law folds");
        let mut b = crate::drops::Lcg::new(0x3000_3000u64 ^ 0xC1A5);
        let full = explosion_puff(0.0, 0.0, 0.0, 0.0, &mut b);
        assert!((full.quad_size - 2.0).abs() < 1e-6, "the full quad the zero keeps");
        // THE DETERMINISM (the same seed the same six-offset chain
        // replays)
        let mut a = crate::drops::Lcg::new(0x3000_3000u64 ^ 0xC1A5);
        let mut seed_a = gateway_explosion_seed(100.0, 100.0, 100.0);
        let mut out_a: Vec<(f64, f64, f64, f32)> = Vec::new();
        seed_a.tick_explosion_seed(&mut a, &mut out_a);
        let mut b = crate::drops::Lcg::new(0x3000_3000u64 ^ 0xC1A5);
        let mut seed_b = gateway_explosion_seed(100.0, 100.0, 100.0);
        let mut out_b: Vec<(f64, f64, f64, f32)> = Vec::new();
        seed_b.tick_explosion_seed(&mut b, &mut out_b);
        assert_eq!(out_a, out_b, "the seeded chain replays");
    }

    #[test]
    fn the_electric_spark_walk_the_two_branches_the_data_arms() {
        // THE ELECTRIC-SPARK'S OWN WALK (javap the case 3002 body +
        // `spawnParticlesAlongAxis` + the ElectricSparkProvider over
        // the 26.2 jar): the DATA word the branch split reads — the
        // 0..3 arm the along-axis walk banks, the negative/overflow
        // arm the faces walk keeps; the provider chain the glow
        // pair's own laws wear.
        // THE ALONG-AXIS ARM (data=0 → X): the count the 10..19 band
        // speaks, the X-sliver the ±0.5/±0.125 multipliers keep — the
        // Y/Z positions the edge band answers, the X the half-span
        // one; the X velocity the only nonzero the draw gives
        let mut r = crate::drops::Lcg::new(0x3002_3002u64 ^ 0xC1A5);
        let burst = electric_spark_burst((10, 20, 30), 0, &mut r);
        assert!(
            (10..=19).contains(&burst.len()),
            "the UniformInt.of(10,19) count the axis walk banks: {}",
            burst.len()
        );
        for p in &burst {
            assert!(matches!(p.kind, ParticleKind::Glow), "the spark the Glow pair banks");
            assert_eq!(p.col, [1.0, 0.9, 1.0], "the white-pink the provider tints");
            assert!((2..=3).contains(&p.lifetime), "the nextInt(2)+2 band");
            assert!(
                (10.5 - 0.5..=10.5 + 0.5).contains(&p.x),
                "the X half-span the chosen axis rides: {}",
                p.x
            );
            assert!(
                (20.5 - 0.125..=20.5 + 0.125).contains(&p.y),
                "the Y edge band the other axis keeps: {}",
                p.y
            );
            assert!(
                (30.5 - 0.125..=30.5 + 0.125).contains(&p.z),
                "the Z edge band the other axis keeps: {}",
                p.z
            );
            assert_eq!(p.vy, 0.0, "the Y velocity the dconst_0 keeps");
            assert_eq!(p.vz, 0.0, "the Z velocity the dconst_0 keeps");
            assert!(
                p.vx.abs() <= 0.25,
                "the X velocity the *0.25 fold answers: {}",
                p.vx
            );
            assert_eq!(p.friction, 0.96, "the GlowParticle's own friction");
        }
        // THE FACES ARM (data=-1, the lightning's own word): the
        // 18..30 band the six-face law speaks, the face planes the
        // 0.55 offsets keep
        let mut r = crate::drops::Lcg::new(0x3002_3001u64 ^ 0xC1A5);
        let faces = electric_spark_burst((10, 20, 30), -1, &mut r);
        assert!(
            (18..=30).contains(&faces.len()),
            "the six-face count band the faces walk banks: {}",
            faces.len()
        );
        let mut seen_face_plane = false;
        for p in &faces {
            assert!(matches!(p.kind, ParticleKind::Glow));
            assert_eq!(p.col, [1.0, 0.9, 1.0], "the white-pink the provider tints");
            // the face plane: one axis rides the ±0.55 offset over
            // the block CENTER (the 10.5/20.5/30.5 the anchor's own
            // +0.5 fold keeps)
            if (p.x - 10.5 - 0.55).abs() < 1e-9
                || (p.x - 10.5 + 0.55).abs() < 1e-9
                || (p.y - 20.5 - 0.55).abs() < 1e-9
                || (p.y - 20.5 + 0.55).abs() < 1e-9
                || (p.z - 30.5 - 0.55).abs() < 1e-9
                || (p.z - 30.5 + 0.55).abs() < 1e-9
            {
                seen_face_plane = true;
            }
        }
        assert!(seen_face_plane, "the face plane the 0.55 offsets keep");
        // THE DETERMINISM (the same seed the same branch replays)
        let mut a = crate::drops::Lcg::new(0x3002_3002u64 ^ 0xC1A5);
        let burst_a = electric_spark_burst((10, 20, 30), 0, &mut a);
        let mut b = crate::drops::Lcg::new(0x3002_3002u64 ^ 0xC1A5);
        let burst_b = electric_spark_burst((10, 20, 30), 0, &mut b);
        assert_eq!(burst_a.len(), burst_b.len(), "the same count the seed gives");
        for (pa, pb) in burst_a.iter().zip(burst_b.iter()) {
            assert_eq!(
                (pa.x, pa.y, pa.z, pa.vx, pa.vy, pa.vz, pa.lifetime),
                (pb.x, pb.y, pb.z, pb.vx, pb.vy, pb.vz, pb.lifetime),
                "the seeded chain replays"
            );
        }
    }

    #[test]
    fn the_sculk_charge_walk_the_faces_the_rolls_wear() {
        // THE SCULK CHARGE'S OWN FACE LAW (javap the case 3006 count>0
        // arm + `spawnParticlesOnBlockFace` -> `spawnParticleOnFace` +
        // the `lambda$levelEvent$0` supplier + the SculkChargeParticle
        // providers over the 26.2 jar): the six faces the
        // Direction.values() order keeps (the zero packed byte the
        // filter never arms), the per-face `nextInt(count + 1)`
        // counts, the supplier's three ±0.005 draws, the position
        // pair (the face axis's span plane — 0.65 Y / 0.57 X-Z — the
        // perpendicular pair's ±0.5 draws), the IN-PLANE velocity
        // (the face-normal axis zeroes), the π roll the DOWN face's
        // flip wears, the 8..19 lifetime, the 1.5x size, the seeded
        // chain replays.
        let seed = 0x5C01_3006u64 ^ 0xC1A5;
        let mut r = crate::drops::Lcg::new(seed);
        let burst = sculk_charge_faces((7, 70, 3), 4, 0, &mut r);
        assert!(
            !burst.is_empty(),
            "the zero-count faces the law always arms"
        );
        let (cx, cy, cz) = (7.5f64, 70.5f64, 3.5f64);
        let mut i = 0usize;
        let mut seen_down_flip = false;
        // DOWN group (y = cy - 0.65)
        let down: Vec<_> = burst[i..]
            .iter()
            .take_while(|p| (p.y - (cy - 0.65)).abs() < 1e-9)
            .collect();
        assert!(!down.is_empty());
        for p in &down {
            assert_eq!(p.vy, 0.0, "the face-normal axis the ifne zeroes");
            assert!(p.vx.abs() <= 0.005 + 1e-12, "the supplier's ±0.005 bound");
            assert!(p.vz.abs() <= 0.005 + 1e-12);
            assert!(p.x > cx - 0.5 && p.x < cx + 0.5, "the ±0.5 perpendicular draw");
            assert!(p.z > cz - 0.5 && p.z < cz + 0.5);
            assert!(
                matches!(p.kind, ParticleKind::SculkCharge { flipped: true }),
                "the DOWN face the π roll flips"
            );
            seen_down_flip = true;
            assert!(p.quad_size > 0.0);
        }
        assert!(seen_down_flip);
        i += down.len();
        // UP group (y = cy + 0.65, NO flip the zero byte's arm keeps)
        let up: Vec<_> = burst[i..]
            .iter()
            .take_while(|p| (p.y - (cy + 0.65)).abs() < 1e-9)
            .collect();
        assert!(!up.is_empty());
        for p in &up {
            assert_eq!(p.vy, 0.0);
            assert!(
                matches!(p.kind, ParticleKind::SculkCharge { flipped: false }),
                "the UP face the zero byte's arm never flips"
            );
        }
        i += up.len();
        // the X-Z spans the 0.57 faces keep (N/S/W/E planes)
        let mut seen_nswe = 0usize;
        for off in [-0.57, 0.57] {
            let g: Vec<_> = burst[i..]
                .iter()
                .take_while(|p| (p.z - (cz + off)).abs() < 1e-9)
                .collect();
            if !g.is_empty() {
                seen_nswe += 1;
                for p in &g {
                    assert_eq!(p.vz, 0.0, "the Z faces the normal zeroes");
                }
                i += g.len();
            }
        }
        for off in [-0.57, 0.57] {
            let g: Vec<_> = burst[i..]
                .iter()
                .take_while(|p| (p.x - (cx + off)).abs() < 1e-9)
                .collect();
            if !g.is_empty() {
                seen_nswe += 1;
                for p in &g {
                    assert_eq!(p.vx, 0.0, "the X faces the normal zeroes");
                }
                i += g.len();
            }
        }
        assert_eq!(seen_nswe, 4, "the four side faces the 0.57 spans keep");
        assert_eq!(i, burst.len(), "the six groups tile the whole burst");
        for p in &burst {
            assert!(
                (8..=19).contains(&p.lifetime),
                "the 8..19 band the nextInt(12) + 8 speaks: {}",
                p.lifetime
            );
        }
        // THE UNPACK ARM (packed = UP|NORTH = (1<<1)|(1<<2) = 6): the
        // UP face the π roll flips, the NORTH face the 0.35 span
        // keeps, the other four the filter refuses
        let mut r2 = crate::drops::Lcg::new(seed ^ 0x5A1);
        let unpacked = sculk_charge_faces((7, 70, 3), 2, 6, &mut r2);
        for p in &unpacked {
            let on_up = (p.y - (cy + 0.35)).abs() < 1e-9;
            let on_north = (p.z - (cz - 0.35)).abs() < 1e-9;
            assert!(
                on_up || on_north,
                "the unpack arm the ordinal bits alone keeps"
            );
            if on_up {
                assert!(
                    matches!(p.kind, ParticleKind::SculkCharge { flipped: true }),
                    "the UP face the unpack arm's π keeps"
                );
            } else {
                assert!(p.vz == 0.0);
                assert!((p.x - cx).abs() <= 0.5);
                assert!((p.y - cy).abs() <= 0.5);
            }
        }
        // THE COUNT <= 0 SPLIT (the `ifle` the face walk opens with —
        // the pop arm the caller answers, the faces walk never spends)
        let mut r3 = crate::drops::Lcg::new(0x5C01_0000);
        let _ = r3.next_f32();
        let before = r3.clone();
        assert!(sculk_charge_faces((0, 0, 0), 0, 0, &mut r3).is_empty());
        assert_eq!(r3, before, "the empty arm the draws never spends");
        // THE SEEDED CHAIN REPLAYS
        let mut a = crate::drops::Lcg::new(seed);
        let burst_a = sculk_charge_faces((7, 70, 3), 4, 0, &mut a);
        assert_eq!(burst.len(), burst_a.len());
        for (x, y) in burst.iter().zip(burst_a.iter()) {
            assert_eq!(
                (x.x, x.y, x.z, x.vx, x.vy, x.vz, x.lifetime, x.quad_size),
                (y.x, y.y, y.z, y.vx, y.vy, y.vz, y.lifetime, y.quad_size),
                "the seeded chain replays"
            );
        }
    }

    #[test]
    fn the_sculk_pops_the_cube_the_count_bands_wear() {
        // THE POP ARM'S OWN LAW (javap the case 3006 count<=0 arm over
        // the 26.2 jar — the vanilla-caller arm the spreader's decay
        // feeds): the FULL block the 40-pop / ±0.45 arm answers, the
        // non-full the 20-pop / ±0.25 keeps; the per-pop trio the
        // 2.0F*nextFloat()-1.0F draws speak, the position + speed
        // folds the spread and 0.07 constants ride, the 6..9
        // lifetime the nextInt(4)+6 band carries, the seeded chain
        // replays.
        let seed = 0x3006_7001u64 ^ 0xC1A5;
        // THE FULL ARM (40 pops, ±0.45)
        let mut r = crate::drops::Lcg::new(seed);
        let full = sculk_charge_pops((0, -3, 0), true, &mut r);
        assert_eq!(full.len(), 40, "the full block the 40-pop arm speaks");
        for p in &full {
            assert!(matches!(p.kind, ParticleKind::SculkChargePop));
            assert!((p.x - 0.5).abs() <= 0.45 + 1e-9);
            assert!((p.y + 2.5).abs() <= 0.45 + 1e-9);
            assert!((p.z - 0.5).abs() <= 0.45 + 1e-9);
            assert!(p.vx.abs() <= 0.07 + 1e-12);
            assert!(p.vy.abs() <= 0.07 + 1e-12);
            assert!(p.vz.abs() <= 0.07 + 1e-12);
            assert!((6..=9).contains(&p.lifetime), "the 6..9 band: {}", p.lifetime);
            assert!(p.quad_size > 0.0);
        }
        // THE NON-FULL ARM (20 pops, ±0.25)
        let mut r2 = crate::drops::Lcg::new(seed ^ 0x77);
        let small = sculk_charge_pops((0, -3, 0), false, &mut r2);
        assert_eq!(small.len(), 20, "the non-full block the 20-pop arm keeps");
        for p in &small {
            assert!((p.x - 0.5).abs() <= 0.25 + 1e-9);
            assert!((p.z - 0.5).abs() <= 0.25 + 1e-9);
        }
        // THE SEEDED CHAIN REPLAYS
        let mut a = crate::drops::Lcg::new(seed);
        let full_a = sculk_charge_pops((0, -3, 0), true, &mut a);
        assert_eq!(full.len(), full_a.len());
        for (x, y) in full.iter().zip(full_a.iter()) {
            assert_eq!(
                (x.x, x.y, x.z, x.vx, x.vy, x.vz, x.lifetime, x.quad_size),
                (y.x, y.y, y.z, y.vx, y.vy, y.vz, y.lifetime, y.quad_size),
                "the seeded chain replays"
            );
        }
        // THE FRAME LAWS (the ascending lists the setSpriteFromAge walk
        // rides — 7 charge frames, 4 pop frames)
        assert_eq!(sculk_charge_frame(0, 19), 0);
        assert_eq!(sculk_charge_frame(19, 19), 6);
        assert_eq!(sculk_pop_frame(0, 9), 0);
        assert_eq!(sculk_pop_frame(9, 9), 3);
        // THE SONIC RING'S OWN LAW (the 16-frame ascending set the
        // `age * 15 / lifetime` fold walks — the age 0 the first frame
        // banks, the last age the 14 the integer division keeps, the
        // 15th texture the vanilla walk never reaches — the same
        // buffer-frame quirk the law's own shape speaks)
        assert_eq!(sonic_boom_frame(0, 16), 0);
        assert_eq!(sonic_boom_frame(1, 16), 0);
        assert_eq!(sonic_boom_frame(15, 16), 14);
        assert_eq!(sonic_boom_frame(16, 16), 15);
        // THE RING'S OWN CTOR (the fixed 16 the lifetime banks, the
        // 1.5f the quad carries, the grey the ONE draw shares, the
        // zero velocity the base trio keeps)
        let ring = sonic_boom_ring(1.0, 2.0, 3.0, 0.85);
        assert!(matches!(ring.kind, ParticleKind::SonicBoom));
        assert_eq!(ring.lifetime, 16);
        assert_eq!(ring.quad_size, 1.5);
        assert_eq!(ring.age, 0);
        assert_eq!(ring.vx, 0.0);
        assert_eq!(ring.vy, 0.0);
        assert_eq!(ring.vz, 0.0);
        assert_eq!(ring.col, [0.85, 0.85, 0.85]);
    }
    #[test]
    fn the_shriek_ring_the_delays_the_draws_wear() {
        // THE SHRIEK RING'S OWN LAW (javap the case 3007 ring + the
        // ShriekParticle providers over the 26.2 jar): the ten
        // members the i*5 delays stagger, the block-top anchor the
        // +0.5 trio rides, the 0.1 rise the yd write carries, the 30
        // -tick life the fixed write speaks, the 0.85 quad the
        // direct putfield keeps, the THREE draws per member the
        // provider chain spends (the sprite pick the nextInt(1)
        // power-of-2 arm spends, the two dead base draws the
        // putfields override), the growth + fade laws the clamps
        // keep, the seeded chain replays.
        let seed = 0x5C01_3007u64 ^ 0xC1A5;
        let mut r = crate::drops::Lcg::new(seed);
        let ring = sculk_shriek_ring((7, 70, 3), &mut r);
        assert_eq!(ring.len(), 10, "the ten members the i<10 loop banks");
        for (i, p) in ring.iter().enumerate() {
            assert!(
                matches!(p.kind, ParticleKind::SculkShriek { delay } if delay == (i as u32) * 5),
                "the i*5 delay the staggered launch speaks: {}",
                i
            );
            assert_eq!((p.x, p.y, p.z), (7.5, 70.5, 3.5), "the block-top anchor rides");
            assert_eq!((p.vx, p.vy, p.vz), (0.0, 0.1, 0.0), "the rise the yd write carries");
            assert_eq!(p.lifetime, 30, "the 30-tick life the fixed write speaks");
            assert_eq!(p.quad_size, 0.85, "the direct putfield the quad keeps");
            assert_eq!(p.gravity, 0.0);
            assert_eq!(p.friction, 0.98);
            assert_eq!(p.col, [1.0, 1.0, 1.0]);
        }
        // THE DRAW COUNT (the 30 draws the provider chain spends — 3
        // per member)
        let mut replay = crate::drops::Lcg::new(seed);
        for _ in 0..10 {
            let _ = replay.next_int(1);
            let _ = replay.next_f32();
            let _ = replay.next_f32();
        }
        assert_eq!(r, replay, "the 30 draws the walk exactly spends");
        // THE GROWTH + FADE LAWS (the clamps the getQuadSize/extract
        // walks keep)
        assert_eq!(shriek_grow(0, 30, 0.0), 0.0, "the ring starts a point");
        assert_eq!(shriek_grow(30, 30, 0.0), 0.75, "the 0.75 cap the growth never exceeds");
        assert_eq!(shriek_fade(0, 30, 0.0), 1.0, "the fade starts whole");
        assert_eq!(shriek_fade(30, 30, 0.0), 0.0, "the fade ends dark");
        // THE SLEEP (the delay walk the countdown rides — the awake
        // arm the base law keeps; the second seeded walk the sleeper
        // banks — the first the replay's own ten keeps)
        let mut r_sleep = crate::drops::Lcg::new(seed);
        let mut sleeper_ring = sculk_shriek_ring((7, 70, 3), &mut r_sleep);
        let mut sleeper = sleeper_ring.pop().expect("the last member the 45-delay sleeper banks");
        let age_before = sleeper.age;
        let sleep_ticks = match sleeper.kind {
            ParticleKind::SculkShriek { delay } => delay,
            _ => unreachable!(),
        };
        assert_eq!(sleep_ticks, 45, "the last member the 45-delay banks");
        for _ in 0..sleep_ticks {
            assert!(sleeper.tick_shriek(), "the asleep member the removal never fires");
        }
        assert_eq!(sleeper.age, age_before, "the sleep the age walk never spends");
        assert!(
            matches!(sleeper.kind, ParticleKind::SculkShriek { delay: 0 }),
            "the countdown the delay reaches zero"
        );
        // the awake arm the base law walks (the age advances, the
        // rise decays the 0.98 friction speaks)
        assert!(sleeper.tick_shriek());
        assert_eq!(sleeper.age, age_before + 1);
        assert!(sleeper.y < 70.5 + 0.1 + 1e-12 && sleeper.y > 70.5, "the rise rode the tick");
        // THE SEEDED CHAIN REPLAYS
        let mut a = crate::drops::Lcg::new(seed);
        let ring_a = sculk_shriek_ring((7, 70, 3), &mut a);
        assert_eq!(ring.len(), ring_a.len());
        for (x, y) in ring.iter().zip(ring_a.iter()) {
            assert_eq!(
                (x.x, x.y, x.z, x.vx, x.vy, x.vz, x.lifetime, x.quad_size),
                (y.x, y.y, y.z, y.vx, y.vy, y.vz, y.lifetime, y.quad_size),
                "the seeded chain replays"
            );
        }
    }

    /// THE SOUL WISPS' OWN WALK (javap the bloom's sendParticles post +
    /// the client handler's count>0 branch + the SoulParticle ctor
    /// chain over the 26.2 jar): the TWO particles the count speaks,
    /// the +0.5/+1.15/+0.5 base the packet pos keeps, the SIX
    /// gaussians per particle the handler spends (the offsets the 0.2
    /// x/z dists scale, the y dist 0.0 the offset discards, the
    /// velocity trio the 0.0 speed discards), the EIGHT f32 draws the
    /// ctor chain spends (the dead life + the size the scale rides,
    /// the six jitter floats the ±0.05 pairs fold, the lifetime
    /// divisor the 12..44 band speaks), the ZERO velocity the packet
    /// keeps, the 0.96 friction the RisingParticle putfield keeps,
    /// the ×1.5 quad the scale fold rides, the white tint the ctor
    /// never tints, the frame law the ascending 11 keeps, the seeded
    /// chain replays.
    #[test]
    fn the_soul_wisps_the_gaussians_the_zero_velocity_keep() {
        let seed = 0x5C01_5EEDu64 ^ 0xB100;
        let mut r = crate::drops::Lcg::new(seed);
        let wisps = sculk_soul_wisps((4, 33, 2), &mut r);
        assert_eq!(wisps.len(), 2, "the two particles the count speaks");
        for p in &wisps {
            assert!(
                matches!(p.kind, ParticleKind::SculkSoul),
                "the soul kind the bloom walk banks"
            );
            // THE BASE + SPREAD (the packet pos ± the 0.05 jitter, the
            // x/z the 0.2-dist gaussians widen — the y the 0.0 dist
            // keeps the jitter band alone)
            let (bx, by, bz) = (4.5f64, 34.15f64, 2.5f64);
            assert!(
                (p.x - bx).abs() <= 0.05 + 1.0 && (p.z - bz).abs() <= 0.05 + 1.0,
                "the x/z spreads the 0.2-dist gaussians ride"
            );
            assert!(
                (p.y - by).abs() <= 0.05,
                "the y the 0.0 dist keeps — the jitter band alone"
            );
            assert_eq!(
                (p.vx, p.vy, p.vz),
                (0.0, 0.0, 0.0),
                "the zero velocity the 0.0-speed multiplies wear"
            );
            assert!(
                (12..=44).contains(&p.lifetime),
                "the 8.0/(r*0.8+0.2)+4 law the 12..44 band speaks: {}",
                p.lifetime
            );
            assert_eq!(p.friction, 0.96, "the RisingParticle putfield keeps");
            assert_eq!(p.gravity, 0.0);
            assert_eq!(p.col, [1.0, 1.0, 1.0], "the white tint the ctor keeps");
            assert!(
                (0.15..=0.3).contains(&p.quad_size),
                "the base*1.5 quad the scale fold rides: {}",
                p.quad_size
            );
        }
        // THE DRAW COUNT (28 draws the walk spends — per particle the
        // six gaussians + the eight f32s the ctor chain spends)
        let mut replay = crate::drops::Lcg::new(seed);
        for _ in 0..2 {
            let _ = replay.next_gaussian();
            let _ = replay.next_gaussian();
            let _ = replay.next_gaussian();
            let _ = replay.next_gaussian();
            let _ = replay.next_gaussian();
            let _ = replay.next_gaussian();
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            let _ = replay.next_f32();
        }
        assert_eq!(r, replay, "the 28 draws the walk exactly spends");
        // THE LIFETIME LAW (the divisor edges — the r=0 the 44 keeps,
        // the r→1 the 12 wears)
        assert_eq!(sculk_soul_lifetime(0.0), 44, "the 0.2 divisor the 40 quotient keeps");
        assert_eq!(
            sculk_soul_lifetime(0.999),
            12,
            "the ~1.0 divisor the 8 quotient keeps"
        );
        // THE FRAME LAW (the ascending 11 — the age walk the
        // MutableSpriteSet.get law folds)
        assert_eq!(SCULK_SOUL_FRAME_COUNT, 11, "the json's own count");
        assert_eq!(sculk_soul_frame(0, 40), 0, "the birth the first frame keeps");
        assert_eq!(sculk_soul_frame(40, 40), 10, "the death the last frame wears");
        assert_eq!(
            sculk_soul_frame(20, 40),
            5,
            "the midpoint the 5*10/40 law speaks"
        );
        assert_eq!(
            sculk_soul_frame(40, 1),
            10,
            "the degenerate life the min clamp keeps"
        );
    }
}
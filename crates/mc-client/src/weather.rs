//! THE WEATHER'S OWN RAIN (javap `WeatherEffectRenderer`, the 26.2 jar —
//! the classfile_dump + raw_disasm + cp walks the no-javap container's
//! tools keep). The rain the splash sounds already herald now FALLS: the
//! per-column quads the jar's `extractRenderState` walk banks, the
//! classic vanilla seed hash and all.
//!
//! THE AUDIT CHAIN, class by class:
//!
//! - `WeatherEffectRenderer.<init>`: the TWO direction tables
//!   (`columnSizeX`/`columnSizeZ`, 1024 entries each) the double loop
//!   banks — for i in 0..32, j in 0..32: `f = j - 16`, `g = i - 16`,
//!   `len = Mth.length(f, g)`; `columnSizeX[i*32+j] = -g / len`,
//!   `columnSizeZ[i*32+j] = f / len`. The normalized camera-facing
//!   diagonal the render's quad corners consume (halved). The center
//!   entry (i=j=16) divides 0/0 — the jar's own NaN, the degenerate
//!   camera-column quad the GPU drops; the port keeps the entry and
//!   the render skips non-finite halves (the same verdict).
//! - `extractRenderState(level, partialTick, cam, state)`: the
//!   intensity gate first (`intensity = level.getRainLevel(partialTick)`,
//!   the `<= 0` early return — the dry worlds never press), then the
//!   radius (the synced `weatherRadius` option, `IntRange(3, 10)` with
//!   the DEFAULT 10 — the optionless client's honest word), then the
//!   double loop `z in k-radius..=k+radius, x in i-radius..=i+radius`
//!   over the floored camera. Per column: `height =
//!   level.getHeight(MOTION_BLOCKING, x, z)`; `bottomY =
//!   max(j - radius, height)`, `topY = max(j + radius, height)` — the
//!   ± radius band around the camera Y the two Math.max pairs keep;
//!   `topY == bottomY` skips the zero-height column;
//!   `getPrecipitationAt(pos.set(x, j, z))` gates NONE; then THE SEED —
//!   `setSeed((long)(x*x*3121 + x*45238971 ^ z*z*418711 + z*13761))`
//!   the IXOR the raw disasm banks (the classic vanilla rain seed, the
//!   int hash sign-extended into the long scramble) — then the light
//!   probe `max(j, height)` high, and the RAIN/SNOW branch picks the
//!   column constructor (the SAME seeded stream flows into it).
//! - `ClientLevel.getPrecipitationAt` → `Biome.getPrecipitationAt`:
//!   the `hasChunk` gate first (an unloaded column is NONE before any
//!   biome read — the client's own grid gate), then
//!   `hasPrecipitation()` (the synced registry's boolean), then
//!   `coldEnoughToSnow` = `!warmEnoughToRain` =
//!   `getTemperature(pos, seaLevel) >= 0.15F`. The temperature law
//!   (`getHeightAdjustedTemperature`): the modifier applies, then for
//!   `y > seaLevel + 17` the height chill
//!   `temp - (noise*8 + y - (seaLevel+17)) * 0.05 / 40` — the
//!   PerlinSimplexNoise term is the REAL `TEMPERATURE_NOISE` (seed
//!   1234, octaves `{0}` — the `noise.rs` synth module the port owns;
//!   the ±0.01 expectation-0 marginal the earlier stand-in kept the
//!   port retires); the FROZEN modifier's noise-gated 0.2F patchwork
//!   rides through the synced `temperature_modifier` word (`frozen`
//!   the only non-identity string — unknown names the jar's own
//!   registry codec would have refused; the built-in jar tables the
//!   generator emits predate the modifier field, that fallback keeps
//!   NONE — the documented limitation, frozen biomes' base 0.0 the
//!   snow answer either way).
//! - `createRainColumnInstance`: `t = (int)(gameTime & 131071L)` (the
//!   17-bit time mask the cp #452 bank keeps), the & 255 hash
//!   `x*x*3121 + x*45238971 + z*z*418711 + z*13761` (the IADD chain —
//!   DISTINCT from the seed's IXOR), `f = 3.0 + nextFloat()`, the
//!   scroll `g = f * -((t + h) + partialTick) / 32.0`, the v offset
//!   `g % 32.0` (Java frem — the truncated remainder, the sign of the
//!   dividend, the Rust `%` twin); the record `(x, z, bottomY, topY,
//!   0.0, vOffset, light)` — the rain's u offset is the CONSTANT 0.0,
//!   the full texture width the quad spans.
//! - `createSnowColumnInstance`: the u offset
//!   `(float)(nextDouble() + (double)(f * 0.01F * (float)nextGaussian()))`
//!   (f = the masked time + partialTick), the extra wander
//!   `(float)(nextDouble() + (double)(f * (float)nextGaussian() * 0.001F))`,
//!   the v scroll `-((gameTime & 511L) + partialTick) / 512.0F` (the 9-bit
//!   mask the cp #474 bank keeps) PLUS the extra, and the BRIGHTENED
//!   light `pack((block*3+15)/4, (sky*3+15)/4)` — the snow's own
//!   full-bright lift the integer law keeps.
//! - `renderInstances(vc, list, cam, baseAlpha, radius, intensity)`:
//!   per column `dx = x + 0.5 - cam.x`, `dz = z + 0.5 - cam.z`;
//!   `alphaF = Mth.lerp(min(lenSq / radius², 1.0), baseAlpha, 0.5) *
//!   intensity` — near columns FULL alpha, the ring's edge at half —
//!   `ARGB.white(alphaF)` = `floor(alphaF * 255) << 24 | 0xFFFFFF` (the
//!   `as8BitChannel` Mth.floor law); the direction-table index
//!   `(z - floor(cam.z) + 16) * 32 + (x - floor(cam.x) + 16)`; the
//!   DIAGONAL quad `(dx-hx, top, dz-hz) (dx+hx, top, dz+hz)
//!   (dx+hx, bottom, dz+hz) (dx-hx, bottom, dz-hz)` in camera-relative
//!   space (the port keeps the world-space twin the view matrix
//!   subtracts); the uv walk `u ∈ [uOffset, uOffset+1]`,
//!   `v = worldY * 0.25 + vOffset` (the WORLD-y anchor the vertical
//!   scroll keeps, the GL_REPEAT wrap the streak texture tiles through).
//! - `render(...)`: the rain quads draw FIRST with `textures/environment/
//!   rain.png` (baseAlpha 1.0), then the snow quads with snow.png
//!   (baseAlpha 0.8 — the fconst 0.800000011920929 pair the bytecode
//!   keeps), one vertex buffer under the WEATHER_NO_DEPTH_WRITE pass —
//!   the depth TEST survives, nothing writes (the translucent lane's
//!   own convention).
//!
//! THE RANDOM: `RandomSource.createThreadLocalInstance()` =
//! `new SingleThreadedRandomSource(ThreadLocalRandom.nextLong())` whose
//! `setSeed` scrambles `(seed ^ 0x5DEECE66D) & ((1<<48)-1)` (the cp
//! #22/#24 constants the raw disasm banks) and whose draws follow the
//! java.util.Random laws — `next(bits) = seed >>> (48 - bits)` after the
//! `seed * 0x5DEECE66D + 0xB` step, `nextFloat = next(24) / 2^24`,
//! `nextDouble = (next(26) << 27) + next(27)) * 2^-53`, the gaussian
//! through `MarsagliaPolarGaussian` (the cached-pair polar walk the
//! drops lane already speaks). `setSeed` resets the gaussian cache.
//!
//! THE RECORD'S LIGHT: the jar's `lightCoords` int packs the two levels
//! through `LightCoordsUtil`; the port carries the block and sky levels
//! UNPACKED (the same data, the client's own shape) — the snow
//! brighten's components stay exact. Unknown light cells (the grid's
//! LIGHT_UNKNOWN) keep the level-15 full-bright — the mesher's legacy
//! unshaded look.
//!
//! THE GOLDEN GATE: every procedural/golden scene carries rain_level
//! 0.0 — the intensity gate returns before any column exists, the dry
//! framebuffers never change, the pixel goldens stay byte-identical.

use crate::math::Vec3;
use crate::raster::SampledQuad;
use crate::world::{World, AIR, CHUNK_Y};

/// The 17-bit game-time mask the rain scroll reads (`gameTime & 131071L`,
/// the cp #452 bank).
pub const RAIN_TIME_MASK: i64 = 131071;
/// The 9-bit mask the snow scroll reads (`gameTime & 511L`, cp #474).
pub const SNOW_TIME_MASK: i64 = 511;
/// The `weatherRadius` option's DEFAULT (the `IntRange(3, 10)` creation
/// the Options <init> bytecode keeps) — the optionless client's radius.
pub const WEATHER_RADIUS: i32 = 10;
/// The sea level the temperature law reads (`ClientLevel.seaLevel`, the
/// overworld's standard — the model carries no sea-level sync).
pub const SEA_LEVEL: i32 = 63;
/// The snow threshold (`warmEnoughToRain`'s `>= 0.15F`, cp #213).
const SNOW_THRESHOLD: f32 = 0.15;

/// The vanilla `SingleThreadedRandomSource` port — the java.util.Random
/// LCG the LegacyRandomSource laws keep, the MarsagliaPolarGaussian
/// cached-pair gaussian on top. NOW the synth module's own word (the
/// jar's `SimplexNoise` ctor argument chain) — the port moved to
/// [`crate::noise`] when the PerlinSimplexNoise trio landed; this
/// re-export keeps the weather lane's stream law one name away.
pub use crate::noise::VanillaRandom;

/// The `WeatherEffectRenderer$ColumnInstance` record — the field order
/// the ctor walk banks: `(x, z, bottomY, topY, uOffset, vOffset,
/// lightCoords)`. The light rides UNPACKED (block, sky) — the same data
/// the jar's packed int carries, the client's own shape (see the module
/// docs).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColumnInstance {
    pub x: i32,
    pub z: i32,
    pub bottom_y: i32,
    pub top_y: i32,
    pub u_offset: f32,
    pub v_offset: f32,
    /// The packed block-light level (low nibble of the jar's lightCoords).
    pub light_block: u8,
    /// The packed sky-light level (high nibble of the jar's lightCoords).
    pub light_sky: u8,
}

/// The direction table's cell law — `columnSizeX[i*32+j] = -g / len`
/// with `f = j - 16`, `g = i - 16`, `len = Mth.length(f, g)`. The center
/// cell (16, 16) divides zero by zero — the jar's own NaN (the
/// degenerate camera-column quad the render drops).
#[inline]
pub fn column_size_x(i: usize, j: usize) -> f32 {
    let f = j as f32 - 16.0;
    let g = i as f32 - 16.0;
    -g / mth_length(f, g)
}

/// The z twin — `columnSizeZ[i*32+j] = f / len`.
#[inline]
pub fn column_size_z(i: usize, j: usize) -> f32 {
    let f = j as f32 - 16.0;
    let g = i as f32 - 16.0;
    f / mth_length(f, g)
}

/// `Mth.length(FF)`: the hypotenuse `sqrt(f*f + g*g)`.
#[inline]
fn mth_length(f: f32, g: f32) -> f32 {
    (f * f + g * g).sqrt()
}

/// The render's direction-table index —
/// `(z - floor(cam.z) + 16) * 32 + (x - floor(cam.x) + 16)` (the raw
/// disasm's isub/iadd/imul chain). The camera-relative diagonal the
/// streaks face.
#[inline]
pub fn column_dir_index(x: i32, z: i32, cam_floor_x: i32, cam_floor_z: i32) -> usize {
    ((z - cam_floor_z + 16) * 32 + (x - cam_floor_x + 16)) as usize
}

/// The & 255 column hash `createRainColumnInstance` recomputes — the
/// IADD chain the raw disasm keeps (DISTINCT from the seed's IXOR):
/// `x*x*3121 + x*45238971 + z*z*418711 + z*13761`, all wrapping, then
/// the 255 mask.
#[inline]
pub fn rain_column_hash(x: i32, z: i32) -> i32 {
    let a = (x
        .wrapping_mul(x)
        .wrapping_mul(3121))
    .wrapping_add(x.wrapping_mul(45238971));
    let b = (z
        .wrapping_mul(z)
        .wrapping_mul(418711))
    .wrapping_add(z.wrapping_mul(13761));
    a.wrapping_add(b) & 255
}

/// THE SEED (the extractRenderState walk): `x*x*3121 + x*45238971`
/// IXOR `z*z*418711 + z*13761` — the classic vanilla rain seed, the
/// int the `setSeed((long)hash)` sign-extends into the scramble.
#[inline]
pub fn rain_seed_hash(x: i32, z: i32) -> i32 {
    let a = (x
        .wrapping_mul(x)
        .wrapping_mul(3121))
    .wrapping_add(x.wrapping_mul(45238971));
    let b = (z
        .wrapping_mul(z)
        .wrapping_mul(418711))
    .wrapping_add(z.wrapping_mul(13761));
    a ^ b
}

/// `createRainColumnInstance` — the whole walk, the seeded stream
/// consuming exactly ONE `nextFloat`:
/// `t = (int)(gameTime & 131071)`, `h = rain_column_hash & 255`,
/// `f = 3.0 + nextFloat()`,
/// `g = f * -((t + h) as f32 + partialTick) / 32.0`,
/// `v_offset = g % 32.0` (the Java frem — sign of the dividend), the
/// record with the CONSTANT u offset 0.0 and the light passed through
/// verbatim.
#[allow(clippy::too_many_arguments)]
pub fn create_rain_column(
    random: &mut VanillaRandom,
    game_time: i64,
    x: i32,
    bottom_y: i32,
    top_y: i32,
    z: i32,
    light_block: u8,
    light_sky: u8,
    partial_tick: f32,
) -> ColumnInstance {
    let t = (game_time & RAIN_TIME_MASK) as i32;
    let h = rain_column_hash(x, z);
    let f = 3.0 + random.next_f32();
    let g = f * (-(t.wrapping_add(h) as f32 + partial_tick) / 32.0);
    let v_offset = g % 32.0;
    ColumnInstance {
        x,
        z,
        bottom_y,
        top_y,
        u_offset: 0.0,
        v_offset,
        light_block,
        light_sky,
    }
}

/// The snow brighten's component law — `(level * 3 + 15) / 4`, the
/// integer division the bipush 15/iconst_4 chain keeps.
#[inline]
fn brighten(level: u8) -> u8 {
    ((level as i32 * 3 + 15) / 4) as u8
}

/// `createSnowColumnInstance` — the stream consumes
/// nextDouble, nextGaussian, nextDouble, nextGaussian IN ORDER:
/// the u wander `(float)(nextDouble() + (double)(f * 0.01F *
/// (float)nextGaussian()))`, the extra
/// `(float)(nextDouble() + (double)(f * (float)nextGaussian() * 0.001F))`
/// (f = the masked time + partialTick), the v scroll
/// `-(gameTime & 511) + partialTick) / 512.0` PLUS the extra, and the
/// BRIGHTENED light pair.
#[allow(clippy::too_many_arguments)]
pub fn create_snow_column(
    random: &mut VanillaRandom,
    game_time: i64,
    x: i32,
    bottom_y: i32,
    top_y: i32,
    z: i32,
    light_block: u8,
    light_sky: u8,
    partial_tick: f32,
) -> ColumnInstance {
    let t = (game_time & RAIN_TIME_MASK) as i32;
    let f = t as f32 + partial_tick;
    // the stream order the bytecode stack keeps: nextDouble FIRST, then
    // the gaussian the wander multiplies — twice over
    let d1 = random.next_f64();
    let gauss1 = random.next_gaussian() as f32;
    let u_wander = f * 0.01 * gauss1;
    let u_offset = (d1 + u_wander as f64) as f32;
    let d2 = random.next_f64();
    let gauss2 = random.next_gaussian() as f32;
    let extra_word = f * gauss2 * 0.001;
    let extra = (d2 + extra_word as f64) as f32;
    let v_base = -(((game_time & SNOW_TIME_MASK) as f32) + partial_tick) / 512.0;
    ColumnInstance {
        x,
        z,
        bottom_y,
        top_y,
        u_offset,
        v_offset: v_base + extra,
        light_block: brighten(light_block),
        light_sky: brighten(light_sky),
    }
}

/// The precipitation pick (`Biome.Precipitation`, the enum the jar's
/// walk returns).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precipitation {
    None,
    Rain,
    Snow,
}

/// `Biome$TemperatureModifier$2.modifyTemperature` — the FROZEN
/// patchwork the synced modifier word drives: the frozen-temperature
/// noise at `x*0.05, z*0.05` (the i2d·0.05 f64 lane) times 7.0 PLUS
/// the biome-info noise at `x*0.2, z*0.2`; the sum `d0` below 0.3 (the
/// dcmpg/ifge law — the `!(d0 >= 0.3)` form keeps even the NaN lane's
/// verdict) and the biome-info noise at `x*0.09, z*0.09` below 0.8
/// pins the temperature to 0.2F; otherwise the base rides. The NONE
/// modifier is the identity (`TemperatureModifier$1` returns f).
pub fn modify_temperature(climate: crate::world::BiomeClimate, x: i32, z: i32) -> f32 {
    let temperature = climate.temperature;
    if climate.temperature_modifier != crate::world::TemperatureModifier::Frozen {
        return temperature;
    }
    let d0 = crate::noise::frozen_temperature_noise().get_value(
        x as f64 * 0.05,
        z as f64 * 0.05,
        false,
    ) * 7.0
        + crate::noise::biome_info_noise().get_value(x as f64 * 0.2, z as f64 * 0.2, false);
    if !(d0 >= 0.3) {
        let d2 = crate::noise::biome_info_noise().get_value(x as f64 * 0.09, z as f64 * 0.09, false);
        if !(d2 >= 0.8) {
            return 0.2;
        }
    }
    temperature
}

/// `Biome.getHeightAdjustedTemperature` — the modifier's word first,
/// then the height chill: for `y > seaLevel + 17` (the if_icmple gate)
/// the temperature drops `(noiseTerm + y - (seaLevel + 17)) * 0.05F /
/// 40.0F`. THE LANES the bytecode keeps: the noise sample feeds
/// through the f32 divide — `(x / 8.0F) as f64` (the i2f/fdiv/f2d
/// walk), the f64 value times 8.0 narrows with d2f, and the whole band
/// `f1 + y - j` multiplied by f32 0.05 and divided by f32 40.0 stays
/// f32 arithmetic end to end. The noise term is the REAL
/// `TEMPERATURE_NOISE` (seed 1234, octaves `{0}`) — the ±0.01
/// expectation-0 marginal the earlier stand-in kept the port retires.
pub fn height_adjusted_temperature(
    climate: crate::world::BiomeClimate,
    x: i32,
    probe_y: i32,
    z: i32,
    sea_level: i32,
) -> f32 {
    let f = modify_temperature(climate, x, z);
    let j = sea_level + 17;
    if probe_y > j {
        let noise = crate::noise::temperature_noise().get_value(
            (x as f32 / 8.0f32) as f64,
            (z as f32 / 8.0f32) as f64,
            false,
        );
        let f1 = (noise * 8.0) as f32;
        f - (f1 + probe_y as f32 - j as f32) * 0.05f32 / 40.0f32
    } else {
        f
    }
}

/// `Biome.getPrecipitationAt` — the gate, the threshold and the height
/// chill: `hasPrecipitation()` or NONE; the temperature
/// (`height_adjusted_temperature`'s law) `>= 0.15` rains (the fcmpl
/// word the warmEnoughToRain walk keeps), below snows.
pub fn precipitation(
    climate: crate::world::BiomeClimate,
    x: i32,
    probe_y: i32,
    z: i32,
    sea_level: i32,
) -> Precipitation {
    if !climate.has_precipitation {
        return Precipitation::None;
    }
    let temp = height_adjusted_temperature(climate, x, probe_y, z, sea_level);
    if temp >= SNOW_THRESHOLD {
        Precipitation::Rain
    } else {
        Precipitation::Snow
    }
}

/// The extraction's product — the two column lists the render walks,
/// the intensity and radius the alpha law reads.
#[derive(Debug, Clone, PartialEq)]
pub struct WeatherColumns {
    pub rain: Vec<ColumnInstance>,
    pub snow: Vec<ColumnInstance>,
    pub intensity: f32,
    pub radius: i32,
}

/// The column's surface the height scan answers — the client's own
/// `getHeight(MOTION_BLOCKING)` word: the first non-air cell from the
/// grid top, +1 (the splash lane's convention); an all-air SENT column
/// keeps the grid floor 0.
fn column_surface(world: &World, x: i32, z: i32) -> i32 {
    (0..CHUNK_Y as i32)
        .rev()
        .find(|y| world.get(x, *y, z) != AIR)
        .map_or(0, |top| top + 1)
}

/// The light probe's levels — the grid's (sky, block) cell at the
/// column's light height `max(cam_floor_y, height)`, the unknown layers
/// keeping the level-15 full-bright (the mesher's legacy unshaded look).
fn column_light(world: &World, x: i32, light_y: i32, z: i32) -> (u8, u8) {
    let y = light_y.clamp(0, CHUNK_Y as i32 - 1);
    world
        .light
        .as_ref()
        .map(|g| g.cell(world.chunks_per_side, x, y, z))
        .map_or((15, 15), |(sky, block)| {
            (block.unwrap_or(15), sky.unwrap_or(15))
        })
}

/// `extractRenderState` — the whole walk. The dry gate first (the
/// intensity `<= 0` return — the goldens' early exit), the radius-10
/// double loop over the floored camera, the sent-column gate (the jar's
/// `hasChunk`), the surface scan, the ± radius Y band, the zero-height
/// skip, the climate's precipitation pick at the CAMERA's Y, the seed
/// IXOR, the light probe and the branch into the column constructors
/// (the SAME seeded stream the jar's thread-local keeps).
pub fn extract_columns(
    world: &World,
    cam: Vec3,
    game_time: i64,
    partial_tick: f32,
    rain_level: f32,
) -> WeatherColumns {
    let mut out = WeatherColumns {
        rain: Vec::new(),
        snow: Vec::new(),
        intensity: rain_level,
        radius: WEATHER_RADIUS,
    };
    if rain_level <= 0.0 {
        return out; // the dry worlds never press — the goldens' gate
    }
    let radius = WEATHER_RADIUS;
    let cam_floor = (
        cam.x.floor() as i32,
        cam.y.floor() as i32,
        cam.z.floor() as i32,
    );
    for z in (cam_floor.2 - radius)..=(cam_floor.2 + radius) {
        for x in (cam_floor.0 - radius)..=(cam_floor.0 + radius) {
            // the jar's hasChunk gate: an unloaded column's pick is NONE
            if !world.column_sent(x, z) {
                continue;
            }
            let height = column_surface(world, x, z);
            let bottom_y = (cam_floor.1 - radius).max(height);
            let top_y = (cam_floor.1 + radius).max(height);
            if top_y == bottom_y {
                continue; // the zero-height column the jar skips
            }
            // the probe rides the CAMERA's Y (the pos.set(x, j, z) walk)
            let Some(climate) = world.biome_climate_at(x, cam_floor.1, z) else {
                continue; // the no-opinion voxel — never a guessed climate
            };
            let precip = precipitation(climate, x, cam_floor.1, z, SEA_LEVEL);
            if precip == Precipitation::None {
                continue;
            }
            // THE SEED: the IXOR hash sign-extended into the scramble —
            // setSeed BEFORE the constructor draws (the stream discipline
            // the bytecode order keeps), and ONLY the precipitating
            // columns ever seed
            let mut random = VanillaRandom::new(rain_seed_hash(x, z) as i64);
            let light_y = cam_floor.1.max(height);
            let (light_block, light_sky) = column_light(world, x, light_y, z);
            match precip {
                Precipitation::Rain => out.rain.push(create_rain_column(
                    &mut random,
                    game_time,
                    x,
                    bottom_y,
                    top_y,
                    z,
                    light_block,
                    light_sky,
                    partial_tick,
                )),
                Precipitation::Snow => out.snow.push(create_snow_column(
                    &mut random,
                    game_time,
                    x,
                    bottom_y,
                    top_y,
                    z,
                    light_block,
                    light_sky,
                    partial_tick,
                )),
                Precipitation::None => unreachable!("the None pick returned above"),
            }
        }
    }
    out
}

/// `ARGB.white(F)` — the vertex color's alpha word:
/// `as8BitChannel(alpha) << 24 | 0xFFFFFF` with
/// `as8BitChannel = Mth.floor(alpha * 255.0)` (the floor law the raw
/// disasm keeps — never a round).
#[inline]
#[cfg_attr(not(test), expect(dead_code))]
pub fn argb_white(alpha: f32) -> u32 {
    ((alpha * 255.0).floor() as i64 as u32) << 24 | 0xFFFFFF
}

/// `renderInstances`' fade law — `Mth.lerp(min(distSq / radiusSq, 1.0),
/// baseAlpha, 0.5) * intensity`: near columns FULL alpha, the radius
/// ring's edge at half, the intensity the rain level scales.
#[inline]
pub fn column_alpha(dist_sq: f32, radius_sq: f32, base_alpha: f32, intensity: f32) -> f32 {
    let fade = (dist_sq / radius_sq).min(1.0);
    (base_alpha + fade * (0.5 - base_alpha)) * intensity
}

/// The weather TEXTURE (the jar's `textures/environment/rain.png` /
/// `snow.png` the RAIN_LOCATION/SNOW_LOCATION constants name — the
/// 64x256 strips the jar cache carries). `None` = the missing file's
/// honest invisibility (the sprite lane's own rule).
pub struct WeatherSprite {
    pub w: usize,
    pub h: usize,
    pub rgba: Vec<u8>,
}

impl WeatherSprite {
    /// The jar-cache load (the avatar lane's direct-path pattern:
    /// `assets-cache/jar/environment/rain.png`).
    pub fn load(root: &std::path::Path, file: &str) -> Option<WeatherSprite> {
        let bytes = std::fs::read(root.join(file)).ok()?;
        let (w, h, rgba) = decode_rgba(&bytes).ok()?;
        Some(WeatherSprite { w, h, rgba })
    }

    /// The texel at the jar's NORMALIZED uv — the GL_REPEAT wrap both
    /// axes keep (the streak sheet tiles: `v` scrolls far beyond [0, 1],
    /// the wrap law `v - floor(v)` answers), NEAREST filtering.
    pub fn pixel_wrapped(&self, u: f32, v: f32) -> [u8; 4] {
        let uu = (u - u.floor()) as f64;
        let vv = (v - v.floor()) as f64;
        let x = ((uu * self.w as f64) as usize).min(self.w - 1);
        let y = ((vv * self.h as f64) as usize).min(self.h - 1);
        let o = (y * self.w + x) * 4;
        [self.rgba[o], self.rgba[o + 1], self.rgba[o + 2], self.rgba[o + 3]]
    }
}

/// The PNG decode (the avatar lane's own pattern — the `png` crate the
/// module deps carry).
fn decode_rgba(bytes: &[u8]) -> anyhow::Result<(usize, usize, Vec<u8>)> {
    let decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    buf.truncate(info.buffer_size());
    Ok((info.width as usize, info.height as usize, buf))
}

/// The light fold the quads' tint carries — `max(block, sky)` through
/// the mesher's vanilla LightTexture ramp (the water overlay's own
/// `getMaxLocalRawBrightness` chain, the darken-0 clear law).
fn light_tint(block: u8, sky: u8) -> f32 {
    crate::mesher::light_brightness(block.max(sky) as f32)
}

/// `renderInstances` — the quads + per-column alphas the raster's
/// per-quad-alpha walk consumes. The DIAGONAL quad the jar's vertex
/// order keeps: `(x+0.5-hx, top, z+0.5-hz)`, `(x+0.5+hx, top, z+0.5+hz)`,
/// `(x+0.5+hx, bottom, z+0.5+hz)`, `(x+0.5-hx, bottom, z+0.5-hz)` — the
/// world-space twin of the jar's camera-relative corners (the view
/// matrix subtracts what the jar's translate already carried); the uv
/// walk `u ∈ [uOffset, uOffset+1]`, `v = worldY * 0.25 + vOffset` (the
/// WORLD-y anchor); the tint the light ramp folds; the alpha the
/// `ARGB.white` 8-bit channel law quantizes (the vertex color the GPU
/// reads). The NaN center cell (the jar's own 0/0 entry) SKIPS — the
/// degenerate quad the GPU drops. `tex` names the sprite the sampler
/// reads (`weather_rain` / `weather_snow`).
pub fn build_weather_quads(
    columns: &[ColumnInstance],
    cam: Vec3,
    base_alpha: f32,
    radius: i32,
    intensity: f32,
    tex: &'static str,
) -> (Vec<SampledQuad<'static>>, Vec<f32>) {
    let mut quads = Vec::with_capacity(columns.len());
    let mut alphas = Vec::with_capacity(columns.len());
    let radius_sq = (radius * radius) as f32;
    let cam_floor_x = cam.x.floor() as i32;
    let cam_floor_z = cam.z.floor() as i32;
    for inst in columns {
        let dx = inst.x as f32 + 0.5 - cam.x;
        let dz = inst.z as f32 + 0.5 - cam.z;
        let dist_sq = dx * dx + dz * dz;
        let alpha = column_alpha(dist_sq, radius_sq, base_alpha, intensity);
        // the ARGB.white round-trip: the vertex color's 8-bit channel
        // (floor) is ALL the GPU sees — the quantized word is the law
        let alpha = (alpha * 255.0).floor() / 255.0;
        if alpha <= 0.0 {
            continue; // the fully transparent column blends nothing
        }
        let idx = column_dir_index(inst.x, inst.z, cam_floor_x, cam_floor_z);
        let (ti, tj) = (idx / 32, idx % 32);
        let hx = column_size_x(ti, tj) / 2.0;
        let hz = column_size_z(ti, tj) / 2.0;
        // the jar's NaN center: the degenerate camera-column quad the
        // rasterizer itself drops
        if !hx.is_finite() || !hz.is_finite() {
            continue;
        }
        let cx = inst.x as f32 + 0.5;
        let cz = inst.z as f32 + 0.5;
        let top = inst.top_y as f32;
        let bottom = inst.bottom_y as f32;
        let tint = light_tint(inst.light_block, inst.light_sky);
        let u0 = inst.u_offset;
        let u1 = inst.u_offset + 1.0;
        let v_top = top * 0.25 + inst.v_offset;
        let v_bottom = bottom * 0.25 + inst.v_offset;
        quads.push(SampledQuad {
            pts: [
                [cx - hx, top, cz - hz],
                [cx + hx, top, cz + hz],
                [cx + hx, bottom, cz + hz],
                [cx - hx, bottom, cz - hz],
            ],
            uv: [[u0, v_bottom], [u1, v_bottom], [u1, v_top], [u0, v_top]],
            tex,
            tint: [tint, tint, tint],
        });
        alphas.push(alpha);
    }
    (quads, alphas)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The java.util.Random stream the reference vectors bank (the
    /// seeded 48-bit LCG the setSeed scramble keeps — the python twin's
    /// words, bit for bit).
    #[test]
    fn the_vanilla_random_the_java_law_speaks() {
        let mut r = VanillaRandom::new(0);
        let words = [r.next_f32(), r.next_f32(), r.next_f32()];
        assert_eq!(words[0], 0.7309677600860596);
        assert_eq!(words[1], 0.8314409852027893);
        assert_eq!(words[2], 0.2405363917350769);
        r.set_seed(1337);
        let a = r.next_f32();
        let b = r.next_f64();
        assert_eq!(a, 0.6599297523498535);
        assert_eq!(b, 0.17398951484624536);
        // the polar gaussian's cached pair: the two draws the reference
        // stream banks, first and second
        let g1 = r.next_gaussian();
        let g2 = r.next_gaussian();
        assert_eq!(g1, 1.1195996178884284);
        assert_eq!(g2, 0.5055384907968187);
        // the negative seed's sign extension the i2l keeps
        r.set_seed(-45238971);
        let words = [r.next_f32(), r.next_f32(), r.next_f32()];
        assert_eq!(words[0], 0.09426051378250122);
        assert_eq!(words[1], 0.16877442598342896);
        assert_eq!(words[2], 0.5321779251098633);
        // setSeed resets the gaussian cache (the jar's reset walk)
        r.set_seed(1337);
        let _ = r.next_f64();
        let first = r.next_gaussian();
        r.set_seed(1337);
        let _ = r.next_f64();
        let again = r.next_gaussian();
        assert_eq!(first, again, "the cache reset replays the stream");
    }

    /// The rain column's math the reference vectors bank: the & 255
    /// hash, the 3+nextFloat roll, the scroll and the frem word.
    #[test]
    fn the_rain_column_the_hash_and_the_scroll_speak() {
        // the two hashes are DISTINCT laws: the seed's IXOR vs the
        // & 255 IADD chain
        assert_eq!(rain_seed_hash(5, -3), 222591356);
        assert_eq!(rain_column_hash(5, -3), 124);
        assert_eq!(rain_seed_hash(0, 0), 0);
        assert_eq!(rain_seed_hash(-7, 9), -282776364);
        // the full walk at the reference vector's inputs
        let mut r = VanillaRandom::new(rain_seed_hash(5, -3) as i64);
        let col = create_rain_column(&mut r, 1000, 5, 60, 90, -3, 12, 15, 0.5);
        assert_eq!(col.u_offset, 0.0, "the rain's u is the CONSTANT 0.0");
        assert_eq!(col.v_offset, -2.042724609375);
        assert_eq!(col.light_block, 12, "the rain's light passes verbatim");
        assert_eq!(col.light_sky, 15);
        assert_eq!((col.x, col.z, col.bottom_y, col.top_y), (5, -3, 60, 90));
        // the zero tick's identity: no scroll, frem 0
        r.set_seed(rain_seed_hash(0, 0) as i64);
        let col = create_rain_column(&mut r, 0, 0, 0, 1, 0, 0, 0, 0.0);
        assert_eq!(col.v_offset, 0.0);
        // the frem's sign-of-dividend law: a big negative scroll stays
        // in (-32, 0]
        r.set_seed(rain_seed_hash(16, 16) as i64);
        let col = create_rain_column(&mut r, 131071, 16, 0, 1, 16, 0, 0, 0.999);
        assert_eq!(col.v_offset, -19.1435546875);
        assert!(col.v_offset > -32.0 && col.v_offset <= 0.0);
        // the stream discipline: a rain column consumes EXACTLY one
        // nextFloat — replaying the seed and drawing once matches
        let mut r1 = VanillaRandom::new(rain_seed_hash(5, -3) as i64);
        let col1 = create_rain_column(&mut r1, 1000, 5, 0, 1, -3, 0, 0, 0.5);
        let mut r2 = VanillaRandom::new(rain_seed_hash(5, -3) as i64);
        let rolled = r2.next_f32();
        let col2 = ColumnInstance {
            u_offset: 0.0,
            v_offset: (3.0 + rolled)
                * (-(1000i32.wrapping_add(rain_column_hash(5, -3)) as f32 + 0.5) / 32.0)
                % 32.0,
            ..col1
        };
        assert_eq!(col1, col2, "one nextFloat is the whole stream");
    }

    /// The snow column's wander the reference vectors bank: the
    /// gaussian u, the 0.001 extra, the 512-divided scroll and the
    /// brightened light pair.
    #[test]
    fn the_snow_column_the_gaussian_wander_speaks() {
        let mut r = VanillaRandom::new(rain_seed_hash(1, 2) as i64);
        let col = create_snow_column(&mut r, 709, 1, 60, 90, 2, 8, 12, 0.25);
        assert_eq!(col.u_offset, 1.1125444173812866);
        assert_eq!(col.v_offset, 0.6242645978927612);
        // the brighten law: (level*3+15)/4 — 8 -> 9, 12 -> 12
        assert_eq!(col.light_block, (8 * 3 + 15) / 4);
        assert_eq!(col.light_sky, (12 * 3 + 15) / 4);
        // the zero tick: the v base lands on NEGATIVE zero (the fneg
        // the fneg/fdiv chain keeps)
        let mut r = VanillaRandom::new(rain_seed_hash(0, 0) as i64);
        let col = create_snow_column(&mut r, 0, 0, 0, 1, 0, 0, 0, 0.0);
        assert_eq!(col.u_offset, 0.7309677600860596);
        assert_eq!(col.v_offset, -0.0 + 0.5504370331764221);
    }

    /// The direction tables the <init> double loop banks: the
    /// normalized camera-facing diagonal, the 16/0 axis word and the
    /// jar's own NaN center.
    #[test]
    fn the_direction_tables_the_camera_facing_speak() {
        assert_eq!(column_size_x(0, 0), 0.7071067690849304);
        assert_eq!(column_size_z(0, 0), -0.7071067690849304);
        assert_eq!(column_size_x(0, 31), 0.7295371890068054);
        assert_eq!(column_size_z(0, 31), 0.683941125869751);
        assert_eq!(column_size_x(16, 0), -0.0);
        assert_eq!(column_size_z(16, 0), -1.0);
        assert_eq!(column_size_x(31, 31), -0.7071067690849304);
        assert_eq!(column_size_z(31, 31), 0.7071067690849304);
        // the center's 0/0 — the jar's own NaN the render skips
        assert!(column_size_x(16, 16).is_nan());
        assert!(column_size_z(16, 16).is_nan());
        // the index law: (z - floor(camz) + 16) * 32 + (x - floor(camx) + 16)
        assert_eq!(column_dir_index(5, -3, 3, -2), 498);
    }

    /// The precipitation pick: the gate, the threshold boundary and the
    /// height chill above the seaLevel+17 band — at x=z=0 the origin's
    /// noise is EXACTLY 0.0 (the twin's word), so the band math reads
    /// bit-for-bit like the no-noise walk; then the REAL noise words at
    /// the twin's end-to-end vectors.
    #[test]
    fn the_precipitation_the_registry_word_speaks() {
        use crate::world::TemperatureModifier;
        let rain = crate::world::BiomeClimate {
            has_precipitation: true,
            temperature: 0.5,
            temperature_modifier: TemperatureModifier::None,
        };
        let cold = crate::world::BiomeClimate {
            has_precipitation: true,
            temperature: 0.0,
            temperature_modifier: TemperatureModifier::None,
        };
        let dry = crate::world::BiomeClimate {
            has_precipitation: false,
            temperature: 2.0,
            temperature_modifier: TemperatureModifier::None,
        };
        // the gate first: no precipitation is NONE whatever the temp
        assert_eq!(precipitation(dry, 0, 64, 0, SEA_LEVEL), Precipitation::None);
        // the threshold: 0.5 rains at the surface, 0.0 snows
        assert_eq!(precipitation(rain, 0, 64, 0, SEA_LEVEL), Precipitation::Rain);
        assert_eq!(precipitation(cold, 0, 64, 0, SEA_LEVEL), Precipitation::Snow);
        // the boundary word: EXACTLY 0.15 rains (the >= the fcmpl keeps)
        let boundary = crate::world::BiomeClimate {
            has_precipitation: true,
            temperature: 0.15,
            temperature_modifier: TemperatureModifier::None,
        };
        assert_eq!(
            precipitation(boundary, 0, 64, 0, SEA_LEVEL),
            Precipitation::Rain
        );
        // the height chill at the silent origin: 0.5 at y = 63+17+280
        // drops exactly 280 * 0.05/40 = 0.35 -> 0.15 — the
        // exact-boundary rain; one higher snows
        let y_chill = SEA_LEVEL + 17 + 280;
        assert_eq!(
            precipitation(rain, 0, y_chill, 0, SEA_LEVEL),
            Precipitation::Rain
        );
        assert_eq!(
            precipitation(rain, 0, y_chill + 1, 0, SEA_LEVEL),
            Precipitation::Snow
        );
        // below the y0 band the chill never applies
        assert_eq!(
            precipitation(rain, 0, SEA_LEVEL + 17, 0, SEA_LEVEL),
            Precipitation::Rain
        );
        // THE REAL NOISE the twin's end-to-end vectors pin (base 0.5,
        // NONE, sea 63): the f32/f64 lanes bit-exactly
        assert_eq!(
            height_adjusted_temperature(rain, 0, 100, 0, SEA_LEVEL),
            0.4749999940395355
        );
        assert_eq!(
            height_adjusted_temperature(rain, 17, 81, -9, SEA_LEVEL),
            0.5036682486534119
        );
        assert_eq!(
            height_adjusted_temperature(rain, -333, 264, 2077, SEA_LEVEL),
            0.2756045162677765
        );
        assert_eq!(
            height_adjusted_temperature(rain, 1024, 401, -4096, SEA_LEVEL),
            0.09546956419944763
        );
        // the chill's word the temperature answers: 401 high at
        // (1024, -4096) the noise band drags below the threshold
        assert_eq!(
            precipitation(rain, 1024, 401, -4096, SEA_LEVEL),
            Precipitation::Snow
        );
    }

    /// The FROZEN modifier's patchwork the twin's gate table pins: the
    /// d0/d2 sums, the 0.2F override where the gate opens, the base
    /// where it stays shut — and the end-to-end rain-in-frozen answer.
    #[test]
    fn the_frozen_patchwork_the_noise_gate_speaks() {
        use crate::world::TemperatureModifier;
        let frozen = crate::world::BiomeClimate {
            has_precipitation: true,
            temperature: 0.0,
            temperature_modifier: TemperatureModifier::Frozen,
        };
        // the identity word: the NONE modifier returns the base
        let plain = crate::world::BiomeClimate {
            has_precipitation: true,
            temperature: 0.42,
            temperature_modifier: TemperatureModifier::None,
        };
        assert_eq!(modify_temperature(plain, 0, 0), 0.42);
        // the gate table the twin pins (d0 = frozen*7 + info(0.2),
        // d2 = info(0.09))
        // (0, 0): d0 = 0.0 < 0.3, d2 = 0.0 < 0.8 -> the 0.2F override
        assert_eq!(modify_temperature(frozen, 0, 0), 0.2);
        // (11, -3): d0 = 2.7683931904963615 >= 0.3 -> the base rides
        assert_eq!(modify_temperature(frozen, 11, -3), 0.0);
        // (-47, 13): d0 = 0.9094131382952035 >= 0.3 -> the base
        assert_eq!(modify_temperature(frozen, -47, 13), 0.0);
        // (100, 100): d0 = 1.217791342213207 >= 0.3 -> the base
        assert_eq!(modify_temperature(frozen, 100, 100), 0.0);
        // (-2000, 700): d0 = -3.100928759135157 < 0.3,
        // d2 = 0.7482012446965214 < 0.8 -> the override
        assert_eq!(modify_temperature(frozen, -2000, 700), 0.2);
        // (12345, -54321): d0 = 2.3117244753223445 >= 0.3 -> the base
        assert_eq!(modify_temperature(frozen, 12345, -54321), 0.0);
        // end-to-end at y 64 (below the chill band, the sea 63): the
        // opened gate WARMENS the frozen biome to rain — the classic
        // vanilla "sometimes it rains in frozen biomes" word
        assert_eq!(
            precipitation(frozen, 0, 64, 0, SEA_LEVEL),
            Precipitation::Rain
        );
        assert_eq!(
            precipitation(frozen, -2000, 64, 700, SEA_LEVEL),
            Precipitation::Rain
        );
        // the shut gate keeps the 0.0 base — snow
        assert_eq!(
            precipitation(frozen, 11, 64, -3, SEA_LEVEL),
            Precipitation::Snow
        );
        assert_eq!(
            precipitation(frozen, 100, 64, 100, SEA_LEVEL),
            Precipitation::Snow
        );
    }

    /// The extraction walk on a synthetic world: the sent gate, the
    /// climate pick, the seed discipline and the columns' shapes.
    #[test]
    fn the_extraction_the_walk_and_the_stream_keep() {
        use crate::world::{BiomeClimate, BiomeLayer, CHUNK_X, CHUNK_Y, CHUNK_Z};
        let mut w = World::new(3, 0);
        // CLEAR the procedural terrain the seeded constructor generates
        // (the bare grid the weather walk reads), then a floor at y=9
        // everywhere (stone), the surface word = 10
        for x in 0..(3 * CHUNK_X as i32) {
            for z in 0..(3 * CHUNK_X as i32) {
                for y in 0..CHUNK_Y as i32 {
                    w.set(x, y, z, AIR);
                }
                w.set(x, 9, z, crate::world::STONE);
            }
        }
        // the plains climate on id 0, an unknown on nothing else — the
        // layer carries chunks_per_side² chunk entries (3×3)
        w.biomes = Some(BiomeLayer {
            ids: vec![vec![0; CHUNK_X * CHUNK_Y * CHUNK_Z]; 3 * 3],
            tints: Vec::new(),
            climates: vec![BiomeClimate {
                has_precipitation: true,
                temperature: 0.5,
                temperature_modifier: crate::world::TemperatureModifier::None,
            }],
        });
        // the camera over the middle chunk, at the floor's level
        let cam = Vec3::new(24.5, 10.0, 24.5);
        // THE DRY GATE: intensity 0 — no columns, nothing touched
        let dry = extract_columns(&w, cam, 1000, 0.0, 0.0);
        assert!(dry.rain.is_empty() && dry.snow.is_empty());
        // the rain level rides the intensity the alpha law reads
        let wet = extract_columns(&w, cam, 1000, 0.5, 0.75);
        assert_eq!(wet.intensity, 0.75);
        assert_eq!(wet.radius, WEATHER_RADIUS);
        // every sent column within the radius with a climate RAINS
        // (the camera at y 10 is BELOW the height band's floor — the
        // bottom = max(10-10, 10) = 10, top = max(10+10, 10) = 20)
        assert!(!wet.rain.is_empty());
        assert!(wet.snow.is_empty(), "a 0.5 climate rains everywhere");
        for col in &wet.rain {
            assert_eq!(col.bottom_y, 10);
            assert_eq!(col.top_y, 20);
            assert!(
                (col.x as f32 - 24.5).abs() <= 10.0 + 0.5
                    && (col.z as f32 - 24.5).abs() <= 10.0 + 0.5,
                "the radius band bounds every column"
            );
        }
        // the NO-CLIMATE world: no biome layer — no columns at all
        let mut bare = World::new(3, 0);
        for x in 0..(3 * CHUNK_X as i32) {
            for z in 0..(3 * CHUNK_X as i32) {
                for y in 0..CHUNK_Y as i32 {
                    bare.set(x, y, z, AIR);
                }
                bare.set(x, 9, z, crate::world::STONE);
            }
        }
        let none = extract_columns(&bare, cam, 1000, 0.0, 1.0);
        assert!(none.rain.is_empty() && none.snow.is_empty());
        // the UNSENT column gate: outside the grid's chunks — no columns
        let far = extract_columns(&bare, Vec3::new(-100.5, 10.0, 24.5), 1000, 0.0, 1.0);
        assert!(far.rain.is_empty());
    }

    /// The render math the reference vectors bank: the fade lerp, the
    /// ARGB.white floor law and the quad shape the diagonal keeps.
    #[test]
    fn the_render_math_the_fade_and_the_white_speak() {
        // the alpha law: dist 5 of radius 10 -> fade 0.25, rain base 1.0
        // -> (1.0 + 0.25 * (0.5 - 1.0)) * 0.75 = 0.65625
        assert_eq!(column_alpha(25.0, 100.0, 1.0, 0.75), 0.65625);
        // the ARGB.white floor law: the reference words
        assert_eq!(argb_white(0.65625), 0xA7FFFFFF);
        assert_eq!(argb_white(0.5), 0x7FFFFFFF);
        assert_eq!(argb_white(1.0), 0xFFFFFFFF);
        assert_eq!(argb_white(0.0), 0x00FFFFFF);
        assert_eq!(argb_white(0.335), 0x55FFFFFF);
        // the quad build: one rain column at the world origin, the
        // camera 5 blocks away on x, 3.5 on z (floor -4)
        let col = ColumnInstance {
            x: 5,
            z: -3,
            bottom_y: 60,
            top_y: 90,
            u_offset: 0.0,
            v_offset: -2.042724609375,
            light_block: 12,
            light_sky: 15,
        };
        let cam = Vec3::new(0.0, 65.0, -3.5);
        let (quads, alphas) =
            build_weather_quads(&[col], cam, 1.0, WEATHER_RADIUS, 1.0, "weather_rain");
        assert_eq!(quads.len(), 1);
        assert_eq!(alphas.len(), 1);
        // the diagonal corners: the direction cell at
        // idx = (-3 - (-4) + 16) * 32 + (5 - 0 + 16) = 17*32 + 21 = 565
        // -> ti 17, tj 21: f = 5, g = 1, len = sqrt(26)
        let len = (26.0f32).sqrt();
        let hx = -1.0 / len / 2.0;
        let hz = 5.0 / len / 2.0;
        let q = &quads[0];
        assert_eq!(q.tex, "weather_rain");
        assert_eq!(q.pts[0], [5.5 - hx, 90.0, -2.5 - hz]);
        assert_eq!(q.pts[1], [5.5 + hx, 90.0, -2.5 + hz]);
        assert_eq!(q.pts[2], [5.5 + hx, 60.0, -2.5 + hz]);
        assert_eq!(q.pts[3], [5.5 - hx, 60.0, -2.5 - hz]);
        // the uv walk: u spans the full width, v anchors to WORLD y
        assert_eq!(q.uv[0], [0.0, 60.0 * 0.25 + (-2.042724609375)]);
        assert_eq!(q.uv[1], [1.0, 60.0 * 0.25 + (-2.042724609375)]);
        assert_eq!(q.uv[2], [1.0, 90.0 * 0.25 + (-2.042724609375)]);
        assert_eq!(q.uv[3], [0.0, 90.0 * 0.25 + (-2.042724609375)]);
        // the alpha: dist to (5.5, -2.5) from (0, -3.5) — dx 5.5, dz 1.0
        // -> 31.25 / 100 -> fade 0.3125 -> (1 - 0.5*0.3125) = 0.84375
        // -> floor(0.84375*255)/255 = 215/255
        assert_eq!(alphas[0], 215.0 / 255.0);
        // the tint: max(12, 15) = 15 through the ramp = EXACTLY 1.0
        assert_eq!(q.tint, [1.0, 1.0, 1.0]);
        // the NaN center column (the camera's own) is SKIPPED
        let center = ColumnInstance {
            x: 0,
            z: -4,
            ..col
        };
        let (quads, _) =
            build_weather_quads(&[center], cam, 1.0, WEATHER_RADIUS, 1.0, "weather_rain");
        assert!(quads.is_empty(), "the jar's degenerate center quad drops");
    }

    /// The sprite's GL_REPEAT wrap law: negative and oversized v land
    /// on the tiled texel (the streak sheet the vertical scroll rides).
    #[test]
    fn the_sprite_the_repeat_wrap_speaks() {
        let s = WeatherSprite {
            w: 2,
            h: 4,
            rgba: vec![
                1, 2, 3, 255, 4, 5, 6, 255, // row 0
                7, 8, 9, 255, 10, 11, 12, 255, // row 1
                13, 14, 15, 255, 16, 17, 18, 255, // row 2
                19, 20, 21, 255, 22, 23, 24, 255, // row 3
            ],
        };
        assert_eq!(s.pixel_wrapped(0.0, 0.0), [1, 2, 3, 255]);
        assert_eq!(s.pixel_wrapped(0.75, 0.5), [16, 17, 18, 255]);
        // v = -0.25 wraps to 0.75 -> row 3
        assert_eq!(s.pixel_wrapped(0.0, -0.25), [19, 20, 21, 255]);
        // v = 4.25 wraps to 0.25 (one full turn) -> row 1
        assert_eq!(s.pixel_wrapped(0.0, 4.25), [7, 8, 9, 255]);
        // u = -0.5 wraps to 0.5
        assert_eq!(s.pixel_wrapped(-0.5, 0.0), [4, 5, 6, 255]);
    }
}

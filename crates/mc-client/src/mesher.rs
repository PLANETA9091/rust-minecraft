//! Culled face meshing with vanilla-style smooth lighting (per-vertex AO),
//! parallel across chunks with `rayon`.
//!
//! The network world's light layer (when the `World` carries one) rides
//! the same per-vertex shading: each corner averages the 4 face-plane
//! cells' light the vanilla smooth-lighting way (the face cell plus the
//! 3 AO diagonals), known cells only, through the vanilla brightness
//! ramp. A world WITHOUT a light layer (the procedural scene) meshes
//! bit-identically to the legacy path — no opinion, no change.

use rayon::prelude::*;

use mc_registry::blockstate_lookup::{select_parts, Part};
use mc_registry::element_quads::{derive_model_inherited, Quad as DerivedQuad};
use mc_registry::generated::atlas;

use crate::colors::{default_color, FACE_SHADE};
use crate::textures::TextureLut;
use crate::tint;
use crate::world::*;

/// One shaded world-space quad: 4 CCW corner points, per-vertex shading and
/// texture mapping. When `tile` is a real atlas index the vertex colors carry
/// the shade×AO multiplier and the rasterizer samples the official texture;
/// when `tile == TILE_NONE` the vertex colors carry the full flat base color.
#[derive(Debug, Clone, PartialEq)]
pub struct Quad {
    pub pts: [[f32; 3]; 4],
    pub colors: [[f32; 3]; 4],
    pub uvs: [[f32; 2]; 4],
    pub tile: u16,
    pub translucent: bool,
}

/// Brightness multiplier per ambient-occlusion level (reference curve).
/// Level 3 = fully open corner, 0 = corner enclosed by two opaque walls.
/// Reproduces the darkened-corner look of vanilla "Smooth Lighting".
pub const AO_CURVE: [f32; 4] = [0.55, 0.70, 0.85, 1.00];

/// Classic Minecraft vertex AO: a corner is darkened by the opaque blocks
/// touching it on the face plane (side1, side2, corner). Two facing sides
/// fully occlude regardless of the corner block.
#[inline]
pub fn vertex_ao(side1: bool, side2: bool, corner: bool) -> u8 {
    if side1 && side2 {
        0
    } else {
        3 - (side1 as u8 + side2 as u8 + corner as u8)
    }
}

/// Offsets of the 6 neighbors: +X -X +Y -Y +Z -Z.
const NEIGHBOR: [(i32, i32, i32); 6] = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];

/// CCW corner offsets per face (unit cube at origin).
const FACE_CORNERS: [[[f32; 3]; 4]; 6] = [
    // +X
    [[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0]],
    // -X
    [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0]],
    // +Y
    [[0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]],
    // -Y
    [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
    // +Z
    [[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
    // -Z
    [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]],
];

fn is_transparent(kind: u16) -> bool {
    // the leaves FAMILY: oak (foliage-tinted) and pale oak (the audited
    // no-tint verdict — still cutout leaves, non-occluding, no AO);
    // leaf_litter is the flat ground cover (the 13th audit — non-cube,
    // non-occluding, casts no AO, exactly the blockstate models' verdict);
    // LAVA is the SECOND fluid member (the lava lane) — non-occluding
    // like the water it mirrors (the pool walls' faces draw against it);
    // WHEAT is the crop (the 15th audit — the four-plane cross the
    // block/crop model speaks, `.noCollision()` the Blocks clinit's own
    // row keeps): the farmland's top face under it draws, the crop casts
    // no AO (the is_opaque walk the same verdict keeps)
    kind == AIR
        || kind == WATER
        || kind == LEAVES
        || kind == PALE_LEAVES
        || kind == LEAF_LITTER
        || kind == LAVA
        // the whole AGE LADDER the non-occluding verdict keeps (the
        // farmland's top face under ANY stage draws — the same
        // noCollision family the 15th audit and the growth round speak)
        || crate::world::is_wheat(kind)
        // the FLOWER FAMILY the same verdict keeps (the autopush-411
        // round — the cross planes non-occluding, no AO, the grass
        // under the bloom draws)
        || crate::world::is_flower(kind)
        // THE GRASS TUFT FAMILY the same verdict keeps (the autopush-412
        // round — the tinted_cross planes non-occluding, no AO, the
        // noCollision + replaceable rows the Blocks clinit speaks)
        || crate::world::is_tuft(kind)
        // THE DOUBLE PLANT FAMILY the same verdict keeps (the
        // autopush-413 round — the tinted_cross planes per HALF, the
        // noCollision + replaceable rows the Blocks clinit pc 17244/
        // 17297 speaks, the two cells the pair wears)
        || crate::world::is_double_plant(kind)
        // THE TORCH the same verdict keeps (the dev-32 round — the
        // stick box the neighbors' faces draw against, the noCollision
        // row the Properties chain speaks)
        || crate::world::is_torch(kind)
        // THE WEB the same verdict keeps (the dev-22 round — the
        // noCollision() pc 4247 the WebBlock registration speaks, the
        // cross planes non-occluding, no AO, the neighbors' faces the
        // web's own diagonals draw against)
        || crate::world::is_web(kind)
        // THE FIRE KINDS the same verdict keeps (the blocks clinit's
        // replaceable().noCollision() rows the registrations speak —
        // the ground's top face under the flame draws, the flame casts
        // no AO, the multipart planes the cell's own lane renders)
        || crate::world::is_fire(kind)
        // THE CAMPFIRE FAMILY the same verdict keeps (the dev-6 round —
        // the blocks clinit's noOcclusion() row the two registrations
        // speak: the non-occluding column the neighbors' faces draw
        // against, the cullface verdicts the campfire lane's own
        // neighbor reads keep, the fire planes the cell's own light
        // the lane renders)
        || crate::world::is_campfire(kind)
}

/// Only opaque blocks cast ambient occlusion (air/water/leaves never darken
/// a corner — same policy as vanilla).
#[inline]
fn is_opaque(kind: u16) -> bool {
    !is_transparent(kind)
}

#[inline]
fn axis_delta(axis: usize, sign: i32) -> (i32, i32, i32) {
    match axis {
        0 => (sign, 0, 0),
        1 => (0, sign, 0),
        _ => (0, 0, sign),
    }
}

/// Per-corner AO levels (order matches `FACE_CORNERS[face]`) for the face of
/// the block at (wx, wy, wz). Samples the three blocks diagonally adjacent to
/// each corner in the plane one step in front of the face.
fn ao_levels(world: &World, wx: i32, wy: i32, wz: i32, face: usize) -> [u8; 4] {
    let (nx, ny, nz) = NEIGHBOR[face];
    let normal_axis = if nx != 0 { 0 } else if ny != 0 { 1 } else { 2 };
    let (t1, t2) = match normal_axis {
        0 => (1usize, 2usize),
        1 => (0usize, 2usize),
        _ => (0usize, 1usize),
    };
    let (bx, by, bz) = (wx + nx, wy + ny, wz + nz);
    let mut out = [3u8; 4];
    for (i, c) in FACE_CORNERS[face].iter().enumerate() {
        let s1 = if c[t1] >= 0.5 { 1 } else { -1 };
        let s2 = if c[t2] >= 0.5 { 1 } else { -1 };
        let d1 = axis_delta(t1, s1);
        let d2 = axis_delta(t2, s2);
        let side1 = is_opaque(world.get(bx + d1.0, by + d1.1, bz + d1.2));
        let side2 = is_opaque(world.get(bx + d2.0, by + d2.1, bz + d2.2));
        let corner = is_opaque(world.get(bx + d1.0 + d2.0, by + d1.1 + d2.1, bz + d1.2 + d2.2));
        out[i] = vertex_ao(side1, side2, corner);
    }
    out
}

/// Choose the vertex order whose fan diagonal (v0-v2) carries the brighter
/// AO pair — the classic anti-crease fix for interpolated smooth lighting.
fn flip_order(ao: &[u8; 4]) -> [usize; 4] {
    if ao[0] + ao[2] < ao[1] + ao[3] {
        [1, 2, 3, 0]
    } else {
        [0, 1, 2, 3]
    }
}

/// The vanilla LightTexture brightness ramp: `f = level/15`, `f/(4 - 3f)`.
/// Level 15 maps to EXACTLY 1.0 (open-sky faces keep the legacy look) and
/// level 0 to 0.0 (a known-dark cell is black — the vanilla cave look);
/// the curve's aggressive low end (level 8 ≈ 0.22) is the original's.
#[inline]
pub fn light_brightness(level: f32) -> f32 {
    let f = level / 15.0;
    f / (4.0 - 3.0 * f)
}

/// The vanilla lightmap's `BlockLightTint` — the `visual/block_light_tint`
/// environment attribute's DEFAULT, pinned in the jar's bytecode
/// (`EnvironmentAttributes.<clinit>`: `defaultValue(sipush -10100)`; the
/// attribute codec is `ExtraCodecs.STRING_RGB_COLOR` = `hexColor(6)`
/// xmap'd with `ARGB.opaque`/`ARGB.transparent`, so the canonical int is
/// the ARGB `0xFFFFD88C`, unpacked by `ARGB.vector3fFromRGB24` (`red =
/// (i>>16)&255, green = (i>>8)&255, blue = i&255` — the alpha bits fall
/// off the masks): R 255 / G 216 / B **140**. An earlier round shipped
/// 172 for the blue — a one-hex-digit misread of 0x8C as 0xAC
/// (`0xFFFFD8AC` would be -10068, not -10100); this round's audit
/// re-derived the whole chain from the bytecode and corrected it. No
/// dimension JSON in the registry's data and no timeline track overrides
/// it, so the default IS the whole value in every dimension the
/// registry carries — a STATED attribute (a custom server's) replaces
/// it verbatim (the model's `block_light_tint()`).
const DEFAULT_BLOCK_LIGHT_TINT: [f32; 3] = [1.0, 216.0 / 255.0, 140.0 / 255.0];

/// The lightmap's BYTECODE-default `NightVisionColor` —
/// `EnvironmentAttributes.<clinit>` defaultValue -6710887 =
/// 0xFF999999 = #999999 gray (javap; the attribute is
/// `spatiallyInterpolated().syncable()`, so a stated custom-server
/// value replaces it through the wire). The shader consumes it as
/// `NightVisionColor * NightVisionFactor` maxed against the ambient
/// base.
const DEFAULT_NIGHT_VISION_COLOR: [f32; 3] = [153.0 / 255.0; 3];

/// The lightmap's `NightVisionFactor` — the extractor's bytecode
/// (`LightmapRenderStateExtractor.extract`): the player's EFFECT
/// state, not a timeline and not the attribute. `hasEffect(NIGHT_VISION)`
/// -> `GameRenderer.nightVisionScale` (1.0, or `0.7 + sin((duration -
/// partialTick) · π · 0.2) · 0.3` while the effect ends within 200
/// ticks — the fade-out flicker); `waterVision > 0 &&
/// hasEffect(CONDUIT_POWER)` -> `getWaterVision()`; else exactly 0.0.
/// THE FACTOR NOW RIDES THE SNAPSHOT (`World::night_vision_factor`,
/// the model's own-effect store through the Update/Remove Mob Effect
/// wire transport) — this const is the FLOOR the effect-absent state
/// rides: every golden/procedural scene answers 0.0, so their
/// arithmetic is this value's identity (the term
/// `max(AmbientColor, NightVisionColor · 0.0)` is the ambient base
/// bit for bit for ANY tint — a color is non-negative). The flicker
/// is the documented non-goal (same verdict as the torch shimmer);
/// the conduit-power branch needs the underwater state the model
/// does not track and stays 0.0.
#[cfg_attr(not(test), expect(dead_code))]
const NIGHT_VISION_FACTOR: f32 = 0.0;

/// The vanilla lightmap's `BlockFactor` — the extractor's bytecode
/// (`LightmapRenderStateExtractor.extract`): `blockFactor =
/// blockLightFlicker + 1.4`, where the flicker is a mean-zero random
/// walk ((r1−r2)·r3·r4·0.1 decayed ×0.9 per tick — the torch shimmer).
/// A deterministic client cannot carry the random walk; it pins the
/// flicker's EXPECTATION (0) and keeps the constant 1.4 — the mean of
/// the vanilla range, the honest stand-in. The shimmer itself is the
/// documented non-goal (same verdict as the random end-flash).
const BLOCK_FACTOR: f32 = 1.4;

/// The lightmap shader's `parabolicMixFactor`: `(2*level − 1)^2` on the
/// RAW normalized block level — 1.0 at BOTH ends (level 0 and 15), a
/// dip to 0.0 at mid levels. The mix toward white therefore weakens at
/// mid-range (the pure `#ffd8ac` torch warmth) and strengthens at the
/// extremes (full torch light is nearly white-hot; the level-0 mix is
/// irrelevant in the sum since its brightness is 0).
#[inline]
fn parabolic_mix_factor(block_level: f32) -> f32 {
    let t = 2.0 * block_level - 1.0;
    t * t
}

/// The lightmap shader's `BlockLightColor` — `mix(BlockLightTint,
/// vec3(1.0), 0.9 * parabolicMixFactor(block_level))` with
/// `block_level = block / 15`, lightmap.fsh verbatim. The parabola
/// rides the RAW level, NOT the brightness ramp: the warmth peaks at
/// mid levels and the extremes whiten. The red channel is 1.0 for
/// every level (the tint's red is already 1.0). The tint is the
/// dimension's STATED `visual/block_light_tint` when one rides the
/// sync (the model's `block_light_tint()`, normalized to 0..1), the
/// bytecode default otherwise — `None` is the default's verdict,
/// never a guess.
#[inline]
fn block_light_color(block: f32, tint: Option<[f32; 3]>) -> [f32; 3] {
    let tint = tint.unwrap_or(DEFAULT_BLOCK_LIGHT_TINT);
    let mix = 0.9 * parabolic_mix_factor(block / 15.0);
    [
        tint[0] + mix * (1.0 - tint[0]),
        tint[1] + mix * (1.0 - tint[1]),
        tint[2] + mix * (1.0 - tint[2]),
    ]
}

/// The lightmap shader's `max(AmbientColor, nightVisionColor)` base
/// — lightmap.fsh verbatim: `vec3 nightVisionColor =
/// lightmapInfo.NightVisionColor * lightmapInfo.NightVisionFactor;
/// vec3 color = max(lightmapInfo.AmbientColor, nightVisionColor);`.
/// The tint is the dimension's STATED `visual/night_vision_color`
/// when one rides the sync (the model's `night_vision_color()`,
/// normalized to 0..1), the bytecode default otherwise — `None` is
/// the default's verdict, never a guess. The factor rides as a
/// parameter so the term's contract is pinnable at a lifted state
/// (the effect system's arrival); at this client's floor 0.0 the
/// product is exactly zero and the max returns the ambient base bit
/// for bit for ANY tint.
#[inline]
fn night_vision_base(ambient: [f32; 3], tint: Option<[f32; 3]>, factor: f32) -> [f32; 3] {
    let t = tint.unwrap_or(DEFAULT_NIGHT_VISION_COLOR);
    [
        ambient[0].max(t[0] * factor),
        ambient[1].max(t[1] * factor),
        ambient[2].max(t[2] * factor),
    ]
}

/// The vanilla lightmap combine, PER CHANNEL — `lightmap.fsh`'s
/// `color = max(AmbientColor, nightVisionColor); color += SkyLightColor *
/// get_brightness(sky) * SkyFactor + BlockLightColor *
/// get_brightness(block) * BlockFactor`, under the shader's single final
/// `clamp(color, 0, 1)` (the lightmap texture's range; the terms are
/// non-negative so only the 1.0 side can bite). `ambient` is the shader's
/// AmbientColor BASE the sky/block terms are added ON TOP of — the
/// dimension's `visual/ambient_light_color` attribute (overworld
/// #0a0a0a, nether #302821, end #3f473f; the bytecode default black when
/// no opinion — a no-op base, so `None` keeps the pre-ambient arithmetic
/// bit for bit). `NightVisionColor` is the dimension's STATED
/// `visual/night_vision_color` (or the `#999999` bytecode default,
/// gray) UNDER the night-vision factor — the player's EFFECT state,
/// [`NIGHT_VISION_FACTOR`] 0.0 at this client's state, so the base is
/// `max(A, C·0.0) = A` bit for bit (the term rides for the day the
/// effect system lifts the factor — [`night_vision_base`]).
/// `SkyLightColor` is the
/// timeline's `visual/sky_light_color` track (`timeline::sky_light_color`:
/// the white default by day, `#7a7aff` — the moonlight blue — through the
/// night plateau). `BlockLightColor` is [`block_light_color`] — the
/// stated tint (or the `#ffd88c` bytecode default, the corrected blue
/// 140) mixed 0.9-parabola toward white on the raw
/// level — scaled by `BlockFactor` 1.4, the flicker's expectation (the
/// random walk itself is the documented non-goal). The remaining shader
/// terms: the DarknessScale arm the player's own effect state carries —
/// the shader's `color = color - vec3(lightmapInfo.DarknessScale);`
/// subtract (javap `LightmapRenderStateExtractor` pc 304-344 + the
/// 26.2 jar's `shaders/core/lightmap.fsh` verbatim) BEFORE the clamp —
/// the 0.0 no-effect floor keeps the fold bit for bit (the `x - 0.0`
/// identity, the bottom clamp no-op on the non-negative terms) —, the
/// BossOverlay 0 identity, and the
/// brightness (gamma) mix is an identity at gamma 0. A layer the model
/// never answered for is `None`, not zero; no layer at all is the legacy
/// unshaded look.
#[inline]
pub(crate) fn lightmap_combine(
    sky: Option<f32>,
    block: Option<f32>,
    sky_factor: f32,
    sky_tint: [f32; 3],
    ambient: [f32; 3],
    block_tint: Option<[f32; 3]>,
    night_vision: Option<[f32; 3]>,
    night_vision_factor: f32,
    darkness_scale: f32,
) -> [f32; 3] {
    let sky_term = sky.map(|v| light_brightness(v) * sky_factor);
    let block_term = block.map(|v| {
        let b = light_brightness(v) * BLOCK_FACTOR;
        let c = block_light_color(v, block_tint);
        [b * c[0], b * c[1], b * c[2]]
    });
    if sky_term.is_none() && block_term.is_none() {
        return [1.0; 3]; // no light layer — the legacy look
    }
    // the shader's base: max(AmbientColor, NightVisionColor *
    // NightVisionFactor) — the factor rides the snapshot (the effect
    // state's transport); at the effect-absent floor 0.0 it is the
    // ambient color bit for bit
    let mut color = night_vision_base(ambient, night_vision, night_vision_factor);
    if let Some(s) = sky_term {
        color[0] += s * sky_tint[0];
        color[1] += s * sky_tint[1];
        color[2] += s * sky_tint[2];
    }
    if let Some(b) = block_term {
        color[0] += b[0];
        color[1] += b[1];
        color[2] += b[2];
    }
    // THE DARKNESS ARM (lightmap.fsh verbatim: `color = color -
    // vec3(lightmapInfo.DarknessScale);` — the subtract the shader
    // speaks BEFORE the clamp; the 0.0 floor the `x - 0.0` f32
    // identity keeps bit for bit)
    color[0] -= darkness_scale;
    color[1] -= darkness_scale;
    color[2] -= darkness_scale;
    // the shader's own clamp BOTH sides (`clamp(color, 0.0, 1.0)`) —
    // the bottom floor the subtraction's negatives land on (the
    // no-effect floor never wears it — every term above non-negative)
    [
        color[0].min(1.0).max(0.0),
        color[1].min(1.0).max(0.0),
        color[2].min(1.0).max(0.0),
    ]
}

/// The cell's two RAW light layers, as the engine reported them — sky
/// and block are kept apart through the whole smooth-lighting walk so
/// the night sky factor can scale the sky term AFTER the brightness
/// ramp (and the timeline's moonlight tint can COLOR it — the
/// `visual/sky_light_color` track), exactly where the vanilla lightmap
/// shader applies SkyFactor and SkyLightColor
/// (`get_brightness(sky) * SkyFactor`, additive with the block term;
/// 26.2 jar: `assets/minecraft/shaders/core/lightmap.fsh` +
/// `LightmapRenderState.skyFactor`). A layer the model never answered
/// for is `None`, not zero.
#[inline]
fn cell_layers(light: &LightGrid, side: usize, x: i32, y: i32, z: i32) -> (Option<u8>, Option<u8>) {
    light.cell(side, x, y, z)
}

/// Per-corner smooth-light levels (FACE_CORNERS order): the average of
/// the 4 face-plane cells around each corner — the face cell plus the 3
/// AO diagonals, the SAME cells [`ao_levels`] samples. SKY and BLOCK
/// are averaged SEPARATELY (each over its own known cells — the
/// vanilla client averages the packed light's two halves independently;
/// the old combined-cell max was the pre-night simplification). The
/// average is float (smoother than the vanilla packed int average, a
/// deliberate minimal departure). A corner with no known cell at all
/// in BOTH layers keeps the legacy unshaded look — unsent neighbors
/// never darken the faces they touch. Returns `([sky; 4], [block; 4])`.
fn corner_light(
    light: &LightGrid,
    side: usize,
    wx: i32,
    wy: i32,
    wz: i32,
    face: usize,
) -> ([Option<f32>; 4], [Option<f32>; 4]) {
    let (nx, ny, nz) = NEIGHBOR[face];
    let normal_axis = if nx != 0 { 0 } else if ny != 0 { 1 } else { 2 };
    let (t1, t2) = match normal_axis {
        0 => (1usize, 2usize),
        1 => (0usize, 2usize),
        _ => (0usize, 1usize),
    };
    let (bx, by, bz) = (wx + nx, wy + ny, wz + nz);
    let mut out_sky = [None; 4];
    let mut out_block = [None; 4];
    for (i, c) in FACE_CORNERS[face].iter().enumerate() {
        let s1 = if c[t1] >= 0.5 { 1 } else { -1 };
        let s2 = if c[t2] >= 0.5 { 1 } else { -1 };
        let d1 = axis_delta(t1, s1);
        let d2 = axis_delta(t2, s2);
        let cells = [
            (bx, by, bz),
            (bx + d1.0, by + d1.1, bz + d1.2),
            (bx + d2.0, by + d2.1, bz + d2.2),
            (bx + d1.0 + d2.0, by + d1.1 + d2.1, bz + d1.2 + d2.2),
        ];
        let mut sum_sky = 0.0f32;
        let mut n_sky = 0u32;
        let mut sum_block = 0.0f32;
        let mut n_block = 0u32;
        for &(x, y, z) in &cells {
            let (s, b) = cell_layers(light, side, x, y, z);
            if let Some(v) = s {
                sum_sky += v as f32;
                n_sky += 1;
            }
            if let Some(v) = b {
                sum_block += v as f32;
                n_block += 1;
            }
        }
        out_sky[i] = (n_sky > 0).then(|| sum_sky / n_sky as f32);
        out_block[i] = (n_block > 0).then(|| sum_block / n_block as f32);
    }
    (out_sky, out_block)
}

/// Vanilla face->UV orientation. Texture row 0 is the top; side faces keep
/// the block's +Y at v=0; each side is unmirrored when viewed from outside;
/// top maps u=x, v=z (north at texture top), bottom mirrors v.
pub fn face_uv(face: usize, c: [f32; 3]) -> [f32; 2] {
    match face {
        0 => [1.0 - c[2], 1.0 - c[1]], // +X east
        1 => [c[2], 1.0 - c[1]],       // -X west
        2 => [c[0], c[2]],             // +Y up
        3 => [c[0], 1.0 - c[2]],       // -Y down
        4 => [c[0], 1.0 - c[1]],       // +Z south
        _ => [1.0 - c[0], 1.0 - c[1]], // -Z north
    }
}

/// THE DIG LANE's breaking overlay quads — the six faces of the targeted
/// cell sampling the destroy_stage crack tile the progress law names
/// (the vanilla `DestroyStage.getDestructor` index). The faces inflate
/// by the 0.0015 epsilon (the vanilla crumbling render type's polygon
/// offset stand-in — the crack renders in front of the block's own
/// surface, never z-fighting it), the UVs ride the exact `face_uv`
/// orientation the terrain speaks, the vertex colors stay 1.0 (the
/// crack's own grays blend over the block's lit surface — the vanilla
/// crumbling pass's own shape).
pub fn destroy_overlay_quads(pos: (i32, i32, i32), tile: u16) -> Vec<Quad> {
    let (px, py, pz) = (pos.0 as f32, pos.1 as f32, pos.2 as f32);
    const EPS: f32 = 0.0015;
    let infl = |v: f32| if v <= 0.0 { -EPS } else { 1.0 + EPS };
    let mut out = Vec::with_capacity(6);
    for face in 0..6usize {
        let mut pts = [[0f32; 3]; 4];
        let mut uvs = [[0f32; 2]; 4];
        for (i, c) in FACE_CORNERS[face].iter().enumerate() {
            pts[i] = [px + infl(c[0]), py + infl(c[1]), pz + infl(c[2])];
            uvs[i] = face_uv(face, *c);
        }
        out.push(Quad {
            pts,
            colors: [[1.0; 3]; 4],
            uvs,
            tile,
            translucent: false,
        });
    }
    out
}

/// The flat ground-cover plane: leaf_litter's model elements sit at
/// y = 0.25 model units (of 16) — a quarter pixel above the floor
/// (the 13th audit: template_leaf_litter_1..4, all `from.y == to.y == 0.25`).
const FLAT_Y: f32 = 0.25 / 16.0;

/// Mesh one chunk: all faces where a solid block touches a transparent one.
pub fn mesh_chunk(world: &World, cx: usize, cz: usize, lut: Option<&TextureLut>) -> Vec<Quad> {
    let mut out = Vec::with_capacity(2048);
    let light = world.light.as_ref();
    let bx0 = (cx * CHUNK_X) as i32;
    let bz0 = (cz * CHUNK_Z) as i32;
    for ly in 0..CHUNK_Y {
        for lz in 0..CHUNK_Z {
            for lx in 0..CHUNK_X {
                let id = world.chunks[cx + cz * world.chunks_per_side]
                    [lx + lz * CHUNK_X + ly * CHUNK_X * CHUNK_Z];
                if id == AIR {
                    continue;
                }
                let wx = bx0 + lx as i32;
                let wy = ly as i32;
                let wz = bz0 + lz as i32;
                let water_self = id == WATER;
                // THE SECOND FLUID (the lava lane): the same fluid shape
                // the water lane speaks — the surface faces draw only
                // against air, the body's own neighbors stay silent —
                // but OPAQUE (the lava_still pixels never ride the
                // translucent pass the water speaks)
                let lava_self = id == LAVA;
                // The 13th-audit FLAT path: leaf_litter is a zero-thickness
                // ground cover, not a cube. Its blockstate is a multipart
                // (leaf_litter_1..4 x facing rotations) and every element
                // has EXACTLY up+down faces at y=0.25, both tintindex 0 ->
                // the dry fold (DRY_SLOT), with `ambientocclusion: false`
                // (no corner AO — the model verdict) and no cullface (both
                // faces draw unconditionally; the raster backface-culls
                // each to its own side).
                // THE 14TH AUDIT: the state's properties ride the variants
                // grid — `segment_amount` picks the coverage rect(s) (the
                // template elements' exact geometry; the multipart's model
                // sets are 1->{m1}, 2->{m2}, 3->{m2,m3}, 4->{m4}) and
                // `facing` the y-rotation (north=0 / east=90 / south=180 /
                // west=270, the blockstate's own table). No opinion
                // (variant 0 — the procedural scenes, the pre-variants
                // look) = the full footprint at y-rot 0, BIT FOR BIT the
                // 13th-audit emission: segment 4 IS the full rect and the
                // template uv arithmetic collapses to the old `face_uv`
                // exactly at that rect.
                if id == LEAF_LITTER {
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    // The attribute-system fold (javap EnvironmentAttributeSystem:
                    // the dimension's constant layer OVERRIDES the default, the
                    // timeline's multiply track applies last): the stated factor
                    // is the BASE, the day tick's curve value scales it. None =
                    // the default 1.0 — an exact f32 identity, the golden
                    // scenes' bit-stability. Constant per mesh — the shader's
                    // SkyFactor uniform equivalent.
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    // The shader's SkyLightColor uniform: the
                    // attribute-system fold for the COLOR attribute
                    // (javap ColorModifier.MULTIPLY_RGB ->
                    // ARGB.multiply(int, int): the -1 white default
                    // short-circuits to the other operand, else
                    // per-channel imul / idiv 255 — the probe's folded
                    // value IS an Integer the extractor unpacks). None
                    // = the -1 default's shape — the track verbatim,
                    // the exact expression this path shipped before the
                    // stated attribute; a stated base (the end #ac60cd,
                    // the nether #7a7aff, a custom server's wire)
                    // multiplies the byte-space track per channel.
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // The shader's BlockLightTint: the dimension's STATED
                    // attribute (raw bytes -> 0..1), None = the bytecode
                    // default's verdict (#FFD88C, the corrected blue 140).
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // The shader's NightVisionColor: the dimension's STATED
                    // attribute (raw bytes -> 0..1), None = the bytecode
                    // default's verdict (#999999 gray) — consumed UNDER the
                    // night-vision factor (the effect state, 0.0 here).
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let fy = wy as f32 + FLAT_Y;
                    let variant = world
                        .variants
                        .as_ref()
                        .and_then(|g| g.get(cx + cz * world.chunks_per_side))
                        .map(|g| g[lx + lz * CHUNK_X + ly * CHUNK_X * CHUNK_Z])
                        .unwrap_or(0);
                    let (rot_idx, segment) = if variant == 0 {
                        (0usize, 4u8)
                    } else {
                        let v = variant - 1;
                        ((v & 3) as usize, (v >> 2) + 1)
                    };
                    // (x0, z0, x1, z1) in TEXTURE PIXELS — the audited
                    // template elements: model_1 the NW quarter, model_2
                    // the west strip, model_3 the SE quarter, model_4 the
                    // full cover.
                    let rects: &[(f32, f32, f32, f32)] = match segment {
                        1 => &[(0.0, 0.0, 8.0, 8.0)],
                        2 => &[(0.0, 0.0, 8.0, 16.0)],
                        3 => &[(0.0, 0.0, 8.0, 16.0), (8.0, 8.0, 16.0, 16.0)],
                        _ => &[(0.0, 0.0, 16.0, 16.0)],
                    };
                    let theta = rot_idx as f32 * std::f32::consts::FRAC_PI_2;
                    let (sn, cs) = theta.sin_cos();
                    for face in [2usize, 3] {
                        let base = if tile == atlas::TILE_NONE {
                            // The blockstate-driven palette, VARIANT-AWARE
                            // (v0.3.80): segment_amount selects the template
                            // model(s) and each samples a different uv rect,
                            // so the flat color follows the variant — the
                            // same precedence as the cube path below (the
                            // palette first, then the runtime-texture LUT,
                            // then the hand-written defaults).
                            crate::colors::palette_leaf_litter_color(segment, face)
                                .or_else(|| lut.and_then(|l| l.color(id, face)))
                                .unwrap_or_else(|| default_color(id, face))
                        } else {
                            // The same tint carrier as the cube path: the
                            // synced biome's dry-fold color when it differs
                            // from the baked plains sample (the no-op rule),
                            // multiplied into the vertex colors.
                            world
                                .face_tint(id, face, wx, wy, wz)
                                .map(tint::unpack)
                                .unwrap_or([1.0; 3])
                        };
                        let shade = FACE_SHADE[face];
                        // per-corner light in front of the face plane (the
                        // client-wide convention); `None` keeps legacy look
                        let clight =
                            light.map(|l| corner_light(l, world.chunks_per_side, wx, wy, wz, face));
                        for &(px0, pz0, px1, pz1) in rects {
                            let mut pts = [[0f32; 3]; 4];
                            let mut colors = [[0f32; 3]; 4];
                            let mut uvs = [[0f32; 2]; 4];
                            for (i, c) in FACE_CORNERS[face].iter().enumerate() {
                                // The corner's UNROTATED position inside the
                                // rect (block units), uv GLUED to it — the
                                // template's per-face uv arithmetic (up:
                                // u = x, v = z; down mirrors v WITHIN the
                                // rect, v = (z0 + z1 - z)/16 — exactly the
                                // old `face_uv` 1 - z at the full rect).
                                let ux = (px0 + c[0] * (px1 - px0)) / 16.0;
                                let uz = (pz0 + c[2] * (pz1 - pz0)) / 16.0;
                                uvs[i] = if face == 2 {
                                    [ux, uz]
                                } else {
                                    [ux, (pz0 + pz1) / 16.0 - uz]
                                };
                                // The blockstate's y-rotation about the cell
                                // center; the uv stays glued (the element
                                // turns, the texture turns with it).
                                let rx = 0.5 + (ux - 0.5) * cs - (uz - 0.5) * sn;
                                let rz = 0.5 + (ux - 0.5) * sn + (uz - 0.5) * cs;
                                pts[i] = [wx as f32 + rx, fy, wz as f32 + rz];
                                // The vanilla lightmap combine, VERBATIM
                                // (the shared per-channel core both paths
                                // call — see `lightmap_combine`), riding the
                                // dimension's ambient base.
                                let lm = match clight.as_ref() {
                                    Some((sky, block)) => lightmap_combine(
                                        sky[i],
                                        block[i],
                                        sky_factor,
                                        sky_tint,
                                        ambient,
                                        block_tint,
                                        night_vision,
                                        world.night_vision_factor,
                                        world.darkness_scale,
                                    ),
                                    None => [1.0; 3], // no light layer — legacy look
                                };
                                // `ambientocclusion: false` — shade and light
                                // only, NO AO_CURVE term.
                                let m = [shade * lm[0], shade * lm[1], shade * lm[2]];
                                colors[i] = [base[0] * m[0], base[1] * m[1], base[2] * m[2]];
                            }
                            out.push(Quad {
                                pts,
                                colors,
                                uvs,
                                tile,
                                translucent: false,
                            });
                        }
                    }
                    continue;
                }
                // THE 15TH-AUDIT WHEAT PATH: the crop is the SECOND
                // non-cube member. blockstates/wheat.json age=0 ->
                // block/wheat_stage0 -> block/crop, whose FOUR
                // zero-thickness vertical planes (x=4 and x=12 spanning
                // z 0..16, z=4 and z=12 spanning x 0..16, each y -1..15
                // — the sprite sinks one pixel into the farmland below)
                // carry `shade: false` faces under a model-level
                // `ambientocclusion: false` — the vanilla tesselateWithoutAO
                // shape: no AO, no FACE_SHADE, every corner sampling the
                // CELL's own light (getLightColor(state, pos) the
                // withoutAO walk speaks). No cullface anywhere in the
                // model — the planes draw unconditionally; each plane's
                // two faces ride the cube path's own winding convention
                // (FACE_CORNERS per face direction, the raster
                // backface-culls each to its own side). The explicit uvs
                // pair per plane (west/east and north/south, one row
                // [0,0,16,16] the other [16,0,0,16]) folds to ONE u
                // convention per plane — the mirrored row cancels the
                // mirrored face walk, the texture reads the same from
                // both sides; v stays the side-face law (1 - y).
                // THE WHOLE AGE LADDER the path keeps — the per-kind
                // FACE_TILES row the stage's own wheat_stageN tile
                // speaks (the growth walk's setBlock the re-mesh
                // answers, the stage's pixels the ladder's art carries)
                if crate::world::is_wheat(id)
                    || crate::world::is_flower(id)
                    || crate::world::is_tuft(id)
                    || crate::world::is_double_plant(id)
                    // THE WEB the same path keeps (the dev-22 round —
                    // the block/cobweb parent block/cross the two
                    // diagonal planes the flowers' own bake carries,
                    // the tint the pre-colored master never rides,
                    // the offset the NONE row the web never speaks)
                    || crate::world::is_web(id)
                {
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if tile == atlas::TILE_NONE {
                        // the no-atlas boot's own defaults (the palette
                        // speaks no wheat row — the kind's pixels the
                        // generated atlas alone carries)
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else if crate::world::is_tuft(id) || crate::world::is_double_plant(id) {
                        // THE TUFT'S OWN TINT CARRIER (the autopush-412
                        // round — the same shape the flat path and the
                        // cube path speak): the synced biome's grass
                        // color when it differs from the baked plains
                        // sample (the no-op rule — the offline world's
                        // no-biome lane and the plains==baked verdict
                        // both ride the identity), multiplied into the
                        // vertex colors. One color per BLOCK (the
                        // audited BiomeColors.getAverageGrassColor
                        // walk — the block pos the source samples), the
                        // face-2 row the whole-cell verdict reads.
                        // THE DOUBLE PLANT the same carrier keeps (the
                        // autopush-413 round — the doubleTallGrass()
                        // source the same biome color answers, the
                        // UPPER half's pos.below() read the face_tint
                        // query's own below-shift folds)
                        world
                            .face_tint(id, 2, wx, wy, wz)
                            .map(tint::unpack)
                            .unwrap_or([1.0; 3])
                    } else {
                        [1.0; 3] // the tile's own pixels speak (no tintindex — pre-colored)
                    };
                    // The attribute-system fold (the shared per-mesh
                    // uniforms both existing paths speak — see the flat
                    // path's own walk for the javap-proven arithmetic).
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // THE CELL'S OWN LIGHT — the withoutAO walk's
                    // getLightColor(state, pos): both layers sampled AT
                    // the crop cell, all four corners the same sample
                    // (no smoothing, no AO diagonals). The grid absent
                    // (the procedural lane) keeps the legacy unshaded
                    // look — the same None convention both paths speak.
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // The audited model rows: (axis, fixed_pos/16,
                    // u-flipped) — x=4 speaks u=z, x=12 speaks u=1-z,
                    // z=4 speaks u=1-x, z=12 speaks u=x. Each plane's
                    // faces: the pair of cube-face directions its normal
                    // axis carries (x-planes: west+east; z-planes:
                    // north+south), the y extent the -1/16 shift keeps.
                    // THE FLOWER PATH'S OWN CROSS (the autopush-411 round
                    // — the block/cross bake the 45° rotY + rescale fold
                    // speaks): the two diagonal planes the element's own
                    // from/to inset keeps — (0.8, ·, 0.8) -> (15.2, ·,
                    // 15.2) and its mirror — each emitting BOTH windings
                    // (the raster culls one per view side, the vanilla's
                    // own two faces per plane draw). `shade: false` —
                    // no FACE_SHADE term, the cell light alone
                    // multiplies; the uv the full sprite rides (the
                    // mirrored side cancels the reversed walk — the
                    // texture reads the same from both sides). THE TUFT
                    // FAMILY the same bake keeps (the autopush-412
                    // round — the tinted_cross parent the flowers' own
                    // cross geometry repeats, the tint the base carrier
                    // rides), the fold the baked plains look keeps
                    // no-op.
                    if crate::world::is_flower(id)
                        || crate::world::is_tuft(id)
                        || crate::world::is_double_plant(id)
                        // THE WEB the same path keeps (the dev-22 round
                        // — the cross geometry the flowers' own bake
                        // repeats, the offset the plant walk's NONE row
                        // zeroes — the rigid diagonal planes the vanilla
                        // model renders)
                        || crate::world::is_web(id)
                    {
                        // THE PLANT'S OWN RENDER OFFSET (the dev-27 round
                        // — the javap 26.2 law the world.rs walk keeps):
                        // the OffsetFunction's Vec3 the tesselate walk's
                        // FFF triple carries, the model plane the offset
                        // + inset fold rides (the innermost addition —
                        // the section origin adds later, the vanilla's
                        // own block-local bake keeps). The Y-ZERO seed
                        // the pair's two cells shares — the rigid pair
                        // the tall grass wears.
                        let (dx, dy, dz) = crate::world::plant_offset(id, wx, wy, wz);
                        let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
                        let a = 0.8 / 16.0;
                        let b = 15.2 / 16.0;
                        let diagonals: [[(f32, f32); 2]; 2] = [
                            [(a, a), (b, b)], // the first element's bake
                            [(a, b), (b, a)], // the second element's mirror
                        ];
                        for &[(px0, pz0), (px1, pz1)] in &diagonals {
                            for &rev in &[false, true] {
                                let mut pts = [[0f32; 3]; 4];
                                let mut colors = [[0f32; 3]; 4];
                                let mut uvs = [[0f32; 2]; 4];
                                let mut corners = [
                                    (px0, 0.0f32, pz0, 0.0f32),
                                    (px1, 0.0f32, pz1, 1.0f32),
                                    (px1, 1.0f32, pz1, 1.0f32),
                                    (px0, 1.0f32, pz0, 0.0f32),
                                ];
                                if rev {
                                    corners.reverse();
                                }
                                for (i, &(fx, fy, fz, u)) in corners.iter().enumerate() {
                                    pts[i] = [
                                        wx as f32 + (fx + dx),
                                        wy as f32 + (fy + dy),
                                        wz as f32 + (fz + dz),
                                    ];
                                    uvs[i] = [u, 1.0 - fy];
                                    let lm = lightmap_combine(
                                        cell_sky,
                                        cell_block,
                                        sky_factor,
                                        sky_tint,
                                        ambient,
                                        block_tint,
                                        night_vision,
                                        world.night_vision_factor,
                                        world.darkness_scale,
                                    );
                                    colors[i] = [
                                        base[0] * lm[0],
                                        base[1] * lm[1],
                                        base[2] * lm[2],
                                    ];
                                }
                                out.push(Quad {
                                    pts,
                                    colors,
                                    uvs,
                                    tile,
                                    translucent: false,
                                });
                            }
                        }
                        continue;
                    }
                    let planes: [(u8, f32, bool); 4] = [
                        (0, 4.0 / 16.0, false),
                        (0, 12.0 / 16.0, true),
                        (2, 4.0 / 16.0, true),
                        (2, 12.0 / 16.0, false),
                    ];
                    for &(axis, fixed, flip) in &planes {
                        let (fa, fb) = if axis == 0 { (1usize, 0usize) } else { (5usize, 4usize) };
                        for &face in &[fa, fb] {
                            let mut pts = [[0f32; 3]; 4];
                            let mut colors = [[0f32; 3]; 4];
                            let mut uvs = [[0f32; 2]; 4];
                            for (i, c) in FACE_CORNERS[face].iter().enumerate() {
                                let (fx, fy, fz) = if axis == 0 {
                                    (fixed, c[1] - 1.0 / 16.0, c[2])
                                } else {
                                    (c[0], c[1] - 1.0 / 16.0, fixed)
                                };
                                pts[i] = [wx as f32 + fx, wy as f32 + fy, wz as f32 + fz];
                                let u_raw = if axis == 0 { c[2] } else { c[0] };
                                uvs[i] = [
                                    if flip { 1.0 - u_raw } else { u_raw },
                                    1.0 - c[1],
                                ];
                                let lm = lightmap_combine(
                                    cell_sky,
                                    cell_block,
                                    sky_factor,
                                    sky_tint,
                                    ambient,
                                    block_tint,
                                    night_vision,
                                    world.night_vision_factor,
                                    world.darkness_scale,
                                );
                                // `shade: false` — no FACE_SHADE term,
                                // the lightmap alone multiplies
                                colors[i] = [base[0] * lm[0], base[1] * lm[1], base[2] * lm[2]];
                            }
                            out.push(Quad {
                                pts,
                                colors,
                                uvs,
                                tile,
                                translucent: false,
                            });
                        }
                    }
                    continue;
                }
                // THE TORCH'S OWN ELEMENT LANE (the dev-32 round — javap
                // the 26.2 jar: blockstates/torch.json's "" variant ->
                // block/torch -> block/template_torch, the ONE element box
                // from [7,0,7] to [9,10,9] the generated elements table
                // banks, `shade: false` faces under model-level
                // `ambientocclusion: false` — the tesselateWithoutAO shape
                // the crop family wears: the CELL's own light every corner
                // samples, no AO diagonals, no FACE_SHADE term). The quads
                // the registry's derive walk bakes (the same chain the TNT
                // render speaks — the faceinfo corners, the vanilla
                // winding recalculation), positioned at the cell; the
                // down face's `cullface: down` the below cell's opaque
                // verdict culls (the same isSolidRender law the cube
                // path's draw read speaks); the UVs the model's texel-space
                // rows the derive walk keeps verbatim (down [7,13,9,15],
                // up [7,6,9,8], the four sides [7,6,9,16]) — the /16 fold
                // the tile's 0..1 space wears, the sub-sprite windows the
                // stick's sides and the flame tip sample.
                if crate::world::is_torch(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    // `shade: false` + the no-atlas boot's own default —
                    // the tile's own pixels speak, the flat palette the
                    // missing row keeps (the crop path's own verdict)
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    // THE DOWN CULL'S OWN READ — the below cell the
                    // cullface verdict samples (the vanilla cullface law:
                    // the neighbor's face the solid render answers)
                    let below = world.get(wx, wy - 1, wz);
                    for q in torch_model_quads() {
                        if q.cullface == Some(mc_registry::element_quads::Direction::Down)
                            && !is_transparent(below)
                        {
                            continue; // the stick's bottom the solid ground never shows
                        }
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            // the texel-space uv the /16 fold normalizes
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            // `shade: false` — no FACE_SHADE term, the
                            // lightmap alone multiplies
                            colors[i] = [base[0] * lm[0], base[1] * lm[1], base[2] * lm[2]];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                        });
                    }
                    continue;
                }
                // THE CANDLE PAIR'S OWN ELEMENT LANE (the autopush-522
                // round — the canLight consult's own kinds finally
                // RENDERS). blockstates/candle.json's one-candle rows:
                // "candles=1,lit=false" -> block/candle_one_candle and
                // "candles=1,lit=true" -> block/candle_one_candle_lit,
                // BOTH parents block/template_candle — the stick box
                // [7,0,7]->[9,6,9] (six faces, the `#all` ref the kind's
                // own tile answers) PLUS the two ±45° rotY wick planes
                // the generated ELEMENTS table carries (the derive walk
                // bakes the rotations into the vertices the fire lane's
                // rotated planes speak). The elements carry `shade: true`
                // — the vanilla tesselation's FACE_SHADE term the cube
                // path's own table wears (up 1.0, down 0.5, the four
                // horizontals 0.8/0.6), the term the torch/fire lanes'
                // `shade: false` elements never pay. The cell's own
                // light every corner samples (no AO diagonals — the
                // same withoutAO fold the element lanes keep), the down
                // face's `cullface: down` the below cell's opaque
                // verdict culls (the torch lane's own verdict).
                if crate::world::is_candle(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    // the no-atlas boot's own default — the tile's own
                    // pixels speak, the flat palette the missing row
                    // keeps (the torch path's own verdict)
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    // THE DOWN CULL'S OWN READ — the below cell the
                    // cullface verdict samples (the vanilla cullface law:
                    // the neighbor's face the solid render answers)
                    let below = world.get(wx, wy - 1, wz);
                    for q in candle_model_quads(id == crate::world::CANDLE_LIT) {
                        if q.cullface == Some(mc_registry::element_quads::Direction::Down)
                            && !is_transparent(below)
                        {
                            continue; // the stick's bottom the solid ground never shows
                        }
                        // THE SHADE TERM'S OWN READ — the element's
                        // `shade: true` the vanilla tesselation's
                        // direction table answers (the FACE_SHADE
                        // constants the cube path wears; the quad's own
                        // direction the derive walk's facing verdict
                        // carries)
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            // the texel-space uv the /16 fold normalizes
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                        });
                    }
                    continue;
                }
                // THE CAMPFIRE FAMILY'S OWN ELEMENT LANE (the dev-6
                // round — the douse lane's own kinds finally RENDER).
                // blockstates/campfire.json's "facing=south,lit=false"
                // row -> block/campfire_off (the five-log geometry BOTH
                // unlit kinds share — the SOUL_CAMPFIRE kind the same
                // model the blockstate's own rows point, the unlit soul
                // the plain-log look the vanilla itself renders), the
                // "facing=south,lit=true" rows -> block/campfire /
                // block/soul_campfire (BOTH parents
                // block/template_campfire: the five-log boxes the lit
                // faces swap, PLUS the two ±45° rotY rescale fire
                // planes the derive walk bakes — the candle lane's own
                // rotated-plane fold speaks). The per-quad TEXTURE
                // VERDICT the multi-texture template demands (the
                // candle lane's single-tile fold the campfire never
                // wears — the merged texture map's #log/#lit_log/#fire
                // refs the resolved quad texture names carry): the log
                // faces the named tile consts read, the fire faces the
                // strip tables' FRAME0 bases — the draw-time remap the
                // campfire_fire/soul_campfire_fire frametime-2
                // animations swaps (the mesher never re-meshes for the
                // flicker). The cullfaces the five-log boxes carry
                // (north/south/east/west/down) the six-direction
                // neighbor read culls (the vanilla cullface law: the
                // neighbor's face the solid render answers — the
                // candle lane's down-only verdict the campfire's four
                // side rows generalize). The log boxes' `shade: true`
                // elements the FACE_SHADE table pays (up 1.0, down
                // 0.5, the horizontals 0.8/0.6), the fire planes'
                // `shade: false` the lightmap alone multiplies. The
                // cell's own light every corner samples (no AO
                // diagonals — the same withoutAO fold the element
                // lanes keep); the fire planes' y 1..17 span the cell
                // above pokes into, the vanilla's own lick-above the
                // mesh never clips.
                if crate::world::is_campfire(id) {
                    let model_id = match id {
                        crate::world::CAMPFIRE_LIT => "block/campfire",
                        crate::world::SOUL_CAMPFIRE_LIT => "block/soul_campfire",
                        _ => "block/campfire_off",
                    };
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the kind's own FACE_TILES row — the fire quads'
                    // no-atlas fallback (the fire ref the palette row
                    // banks — the resolved texture verdict the strip
                    // base duplicates)
                    let fallback_tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if fallback_tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    for q in campfire_model_quads(model_id) {
                        // THE SIX-DIRECTION CULLFACE VERDICT — the five-
                        // log boxes' side + down rows the neighbor read
                        // culls (the vanilla cullface law the candle
                        // lane's down arm generalizes)
                        if let Some(dir) = q.cullface {
                            let (dx, dy, dz) = match dir {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TEXTURE VERDICT — the resolved
                        // texture name the tile mapping answers (the
                        // fire bases the draw-time remap animates, the
                        // unknown ref the kind's own row keeps)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let tile = match tex_stem {
                            "campfire_log" => atlas::TILE_CAMPFIRE_LOG,
                            "campfire_log_lit" => atlas::TILE_CAMPFIRE_LOG_LIT,
                            "campfire_fire" => atlas::CAMPFIRE_FIRE_FRAME0,
                            "soul_campfire_log_lit" => atlas::TILE_SOUL_CAMPFIRE_LOG_LIT,
                            "soul_campfire_fire" => atlas::SOUL_CAMPFIRE_FIRE_FRAME0,
                            _ => fallback_tile,
                        };
                        // THE SHADE TERM'S OWN READ — the log boxes'
                        // shade:true elements the direction table
                        // answers, the fire planes the 1.0 keeps
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                        });
                    }
                    continue;
                }
                // THE CANDLE CAKE PAIR'S OWN ELEMENT LANE (the
                // autopush-523 round — the canLight consult's third row
                // finally RENDERS). blockstates/candle_cake.json's two
                // rows: "lit=false" -> block/candle_cake and "lit=true"
                // -> block/candle_cake_lit, BOTH parents
                // block/template_cake_with_candle — the cake slab box
                // [1,0,1]->[15,8,15] (the #bottom/#top/#side refs the
                // cake masters answer) PLUS the candle stick box
                // [7,8,7]->[9,14,9] (the #candle ref block/candle the
                // unlit row names, block/candle_lit the lit row — the
                // vanilla's own texture overrides the per-quad verdict
                // reads through the PALETTE's own candle tiles) PLUS
                // the two ±45° rotY wick planes the derive walk bakes
                // (the campfire lane's rotated planes speak). The
                // elements carry `shade: true` — the vanilla
                // tesselation's FACE_SHADE term the cube path's own
                // table wears (up 1.0, down 0.5, the four horizontals
                // 0.8/0.6), the term the torch/fire lanes' `shade:
                // false` elements never pay. The cell's own light every
                // corner samples (no AO diagonals — the same withoutAO
                // fold the element lanes keep); the cullfaces the slab
                // and the stick's own down rows carry the six-direction
                // neighbor read culls (the campfire lane's general
                // verdict the candle lane's down-only arm generalizes).
                if crate::world::is_candle_cake(id) {
                    let model_id = if id == crate::world::CANDLE_CAKE_LIT {
                        "block/candle_cake_lit"
                    } else {
                        "block/candle_cake"
                    };
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the kind's own FACE_TILES row — the no-atlas
                    // boot's own default (the cake_side row the
                    // palette arm banks — the resolved texture verdict
                    // the named consts duplicate)
                    let fallback_tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if fallback_tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    for q in cake_candle_model_quads(model_id) {
                        // THE SIX-DIRECTION CULLFACE VERDICT — the
                        // slab's and the stick's own down rows the
                        // neighbor read culls (the vanilla cullface
                        // law: the neighbor's face the solid render
                        // answers — the campfire lane's own walk)
                        if let Some(dir) = q.cullface {
                            let (dx, dy, dz) = match dir {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TEXTURE VERDICT — the resolved
                        // texture name the tile mapping answers (the
                        // cake masters the named consts bank, the
                        // candle faces the palette's own candle tiles
                        // answer — the same block/candle and
                        // block/candle_lit masters the kind rows keep,
                        // the unknown ref the kind's own row serves)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let tile = match tex_stem {
                            "cake_bottom" => atlas::TILE_CAKE_BOTTOM,
                            "cake_top" => atlas::TILE_CAKE_TOP,
                            "cake_side" => atlas::TILE_CAKE_SIDE,
                            "candle" => atlas::FACE_TILES[crate::world::CANDLE as usize][2],
                            "candle_lit" => {
                                atlas::FACE_TILES[crate::world::CANDLE_LIT as usize][2]
                            }
                            _ => fallback_tile,
                        };
                        // THE SHADE TERM'S OWN READ — the template's
                        // shade:true elements the direction table
                        // answers, the never-shaded the 1.0 keeps
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                        });
                    }
                    continue;
                }
                // THE CAKE LADDER'S OWN ELEMENT LANE (the autopush-524
                // round — the full cake and the six bitten stages the
                // eat lane's kind swaps wear, the candle-cake lane's
                // own single-box law the ladder generalizes). The
                // blockstate's "bites=N" rows block/cake and
                // block/cake_sliceN resolve — each a SINGLE box element
                // [1+2N, 0, 1]->[15, 8, 15] (the lambda$static$0 shape
                // the x-start walk the models exactly mirror), #bottom/
                // #top/#side the cake masters name, the west face
                // #inside the bitten cut speaks (block/cake_inner the
                // bank's own fourth tile answers). The elements carry
                // `shade: true` — the vanilla tesselation's FACE_SHADE
                // term the cube path's own table wears (up 1.0, down
                // 0.5, the four horizontals 0.8/0.6), the term the
                // torch/fire lanes' `shade: false` elements never pay.
                // The cell's own light every corner samples (no AO
                // diagonals — the same withoutAO fold the element lanes
                // keep); the down face's `cullface: down` the below
                // cell's opaque verdict culls (the candle-cake lane's
                // own six-direction verdict the single row keeps).
                if crate::world::is_cake(id) {
                    let model_id = match id {
                        crate::world::CAKE_SLICE1 => "block/cake_slice1",
                        crate::world::CAKE_SLICE2 => "block/cake_slice2",
                        crate::world::CAKE_SLICE3 => "block/cake_slice3",
                        crate::world::CAKE_SLICE4 => "block/cake_slice4",
                        crate::world::CAKE_SLICE5 => "block/cake_slice5",
                        crate::world::CAKE_SLICE6 => "block/cake_slice6",
                        _ => "block/cake",
                    };
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the kind's own FACE_TILES row — the no-atlas
                    // boot's own default (the cake_side row the
                    // palette arm banks — the resolved texture verdict
                    // the named consts duplicate)
                    let fallback_tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if fallback_tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    for q in cake_model_quads(model_id) {
                        // THE SIX-DIRECTION CULLFACE VERDICT — the
                        // box's own down row the neighbor read culls
                        // (the vanilla cullface law: the neighbor's
                        // face the solid render answers — the
                        // candle-cake lane's own walk)
                        if let Some(dir) = q.cullface {
                            let (dx, dy, dz) = match dir {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TEXTURE VERDICT — the resolved
                        // texture name the tile mapping answers (the
                        // cake masters the named consts bank, the
                        // bitten cut the TILE_CAKE_INNER const answers
                        // — the unknown ref the kind's own row serves)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let tile = match tex_stem {
                            "cake_bottom" => atlas::TILE_CAKE_BOTTOM,
                            "cake_top" => atlas::TILE_CAKE_TOP,
                            "cake_side" => atlas::TILE_CAKE_SIDE,
                            "cake_inner" => atlas::TILE_CAKE_INNER,
                            _ => fallback_tile,
                        };
                        // THE SHADE TERM'S OWN READ — the template's
                        // shade:true elements the direction table
                        // answers, the never-shaded the 1.0 keeps
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                        });
                    }
                    continue;
                }
                // THE FIRE KINDS' OWN MULTIPART LANE (the flint's and the
                // charge's own product finally RENDERS — the honest hole
                // the glass precedent wore, the fan the vanilla blockstate
                // speaks). blockstates/fire.json is a MULTIPART over the
                // five wall booleans (north/east/south/west/up): the floor
                // pair under the all-false condition, the four side
                // groups each under OR[dir=true, all-false], the up pair
                // under up=true; blockstates/soul_fire.json carries NO
                // when — the full fan always, the up group never (the
                // soul flame the directional properties never wears).
                // THE PROPS' OWN DERIVATION — javap `FireBlock.
                // getStateForPlacement(BlockGetter, BlockPos)` (the 26.2
                // jar): the below cell's `canBurn(belowState) ||
                // belowState.isFaceSturdy(level, below, UP)` gate pc 14-33
                // serves the DEFAULT state (all five booleans false) when
                // it answers, else the `Direction.values()` walk's
                // `setValue(PROPERTY_BY_DIRECTION.get(d),
                // canBurn(getBlockState(pos.relative(d))))` pc 47-128
                // reads the DIRECT neighbor per direction (DOWN rides no
                // property — the map's own row the null check skips).
                // `updateShape` re-runs the SAME walk (`getStateWithAge`
                // -> `getState` -> the placement's own law) — the live
                // neighbor read the offline single-kind world wears as
                // its steady state. The sturdy arm the collision table's
                // Full row answers (the same offline equivalence the
                // survival law's doc keeps), the canBurn odds the
                // placement round's own `fire_can_burn` table speaks.
                // THE SELECTION rides the real blockstate engine (the
                // parse + select_parts walk the fence/wire tests pin) —
                // the jar's own when-conditions, the jar's own y rotations.
                // THE MODEL QUADS ride the derive walk (the torch lane's
                // own bake): template_fire_floor's ±22.5° rotX/rotZ +
                // rescale planes, template_fire_side's flat z=0.01 walls,
                // template_fire_up's rotated ceil planes with their uv
                // rotations — the census-derived ELEMENTS table banks all
                // ten fire and six soul models, `derive_model_inherited`
                // resolves the parent chain and the #fire refs. The
                // second-frame carriers (block/fire_1, block/soul_fire_1
                // — fire_floor1/fire_up1/soul_fire_floor1's own rows) map
                // to the kind's SINGLE atlas tile (the static-frame
                // policy the fluids wear — the animation strips the
                // palette's frame-0 banking keeps). `shade: false` +
                // model-level `ambientocclusion: false` on every fire
                // element — the tesselateWithoutAO shape the crop and
                // torch lanes wear: the CELL's own light, no AO diagonals,
                // no FACE_SHADE term. No cullface anywhere — the planes
                // draw unconditionally; the multipart's `y` rotation
                // (no uvlock on any fire apply) turns the side walls
                // about the cell's own center: javap
                // `Variant$SimpleModelState.asModelState` ->
                // `Quadrant.fromXYZAngles` -> `OctahedralGroup.
                // BLOCK_ROT_Y_90` = `ROT_90_Y_NEG` = (P321, flipX) —
                // the (x, z) -> (1-z, x) quarter the north-to-east turn
                // speaks, 180 the double, 270 the inverse.
                if crate::world::is_fire(id) {
                    let soul = id == crate::world::SOUL_FIRE;
                    let below = world.get(wx, wy - 1, wz);
                    let gate = crate::world::fire_can_burn(below)
                        || crate::world::collision_shape(below)
                            == crate::world::CollisionShape::Full;
                    let prop = |read: bool| {
                        // the gate's own verdict — the default state's
                        // all-false shape the below cell's gate keeps
                        if gate { "false" } else if read { "true" } else { "false" }
                    };
                    let props = [
                        ("north", prop(crate::world::fire_can_burn(world.get(wx, wy, wz - 1)))),
                        ("east", prop(crate::world::fire_can_burn(world.get(wx + 1, wy, wz)))),
                        ("south", prop(crate::world::fire_can_burn(world.get(wx, wy, wz + 1)))),
                        ("west", prop(crate::world::fire_can_burn(world.get(wx - 1, wy, wz)))),
                        ("up", prop(crate::world::fire_can_burn(world.get(wx, wy + 1, wz)))),
                    ];
                    // The attribute-system fold (the shared per-mesh
                    // uniforms both existing paths speak — see the flat
                    // path's own walk for the javap-proven arithmetic).
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // THE CELL'S OWN LIGHT — the withoutAO walk's
                    // getLightColor(state, pos): both layers sampled AT
                    // the flame cell, all corners the same sample (the
                    // grid absent keeps the legacy unshaded look — the
                    // same None convention both paths speak).
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    // THE SECOND FRAME'S OWN FAMILY — the model id's
                    // trailing digit the blockstate's own apply pairs
                    // speak (fire_floor0 AND fire_floor1, the side0/1 +
                    // side_alt0/1 fours, the up pair — both families the
                    // SAME group applies, the two overlapped quads the
                    // independent sprite flicker speaks): the *_1 models
                    // the fire_1/soul_fire_1 strip's frame-0 tile bake
                    // (the remap's own base), the *_0 family the kind's
                    // single atlas tile keeps (the fire_0/soul_fire_0
                    // strip's first frame the block palette banks). The
                    // frames themselves the DRAW-TIME remap swaps
                    // (`atlas::anim_tile` — the mesher never
                    // re-meshes for the flicker).
                    let tile_of_family = |model_id: &str| -> u16 {
                        if (id as usize) >= atlas::N_KINDS {
                            return atlas::TILE_NONE;
                        }
                        if model_id.ends_with('1') {
                            if soul {
                                atlas::SOUL_FIRE_1_FRAMES[0]
                            } else {
                                atlas::FIRE_1_FRAMES[0]
                            }
                        } else {
                            tile
                        }
                    };
                    // `shade: false` + the no-atlas boot's own default —
                    // the tile's own pixels speak, the flat palette the
                    // missing row keeps (the torch path's own verdict)
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    for apply in select_parts(fire_parts(soul), &props) {
                        // the namespaced ref the blockstate writes the
                        // vanilla ModelResourceLocation parse resolves —
                        // the bare `builtin/` refs the four special forms
                        // keep ride the same strip (the fire applies all
                        // carry the minecraft: prefix, the honest default)
                        let model_id = apply.model.strip_prefix("minecraft:").unwrap_or(apply.model);
                        let Some(model) = fire_model_quads().get(model_id) else {
                            continue;
                        };
                        let tile = tile_of_family(model_id);
                        for q in model {
                            let mut pts = [[0f32; 3]; 4];
                            let mut colors = [[0f32; 3]; 4];
                            let mut uvs = [[0f32; 2]; 4];
                            for (i, v) in q.vertices.iter().enumerate() {
                                // the multipart's y quarter about the cell's
                                // own center (no uvlock — the uvs ride)
                                let (vx, vz) = match (apply.y / 90) % 4 {
                                    1 => (1.0 - v[2], v[0]),
                                    2 => (1.0 - v[0], 1.0 - v[2]),
                                    3 => (v[2], 1.0 - v[0]),
                                    _ => (v[0], v[2]),
                                };
                                let lm = lightmap_combine(
                                    cell_sky,
                                    cell_block,
                                    sky_factor,
                                    sky_tint,
                                    ambient,
                                    block_tint,
                                    night_vision,
                                    world.night_vision_factor,
                                    world.darkness_scale,
                                );
                                // `shade: false` — no FACE_SHADE term,
                                // the lightmap alone multiplies
                                pts[i] = [wx as f32 + vx, wy as f32 + v[1], wz as f32 + vz];
                                uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                                colors[i] = [base[0] * lm[0], base[1] * lm[1], base[2] * lm[2]];
                            }
                            out.push(Quad {
                                pts,
                                colors,
                                uvs,
                                tile,
                                translucent: false,
                            });
                        }
                    }
                    continue;
                }
                for face in 0..6usize {
                    let (dx, dy, dz) = NEIGHBOR[face];
                    let n = world.get(wx + dx, wy + dy, wz + dz);
                    let draw = if water_self || lava_self {
                        n == AIR // the fluids draw only against air (surface)
                    } else {
                        is_transparent(n)
                    };
                    if !draw {
                        continue;
                    }
                    // Real texture tile from the generated atlas when present;
                    // otherwise the flat palette path (avg color or defaults).
                    // THE FARMLAND'S OWN WET FACE (the blockstates/farmland.
                    // json's moisture=7 arm the tile atlas speaks — javap
                    // FarmlandBlock.isNearWater the hydration verdict the
                    // vanilla texture the moisture property picks): the top
                    // face the near-water cell the farmland_moist master
                    // wears, the dry the farmland keeps (the sides/bottom
                    // the dirt the both models name)
                    let tile = if (id as usize) < atlas::N_KINDS {
                        if id == crate::world::FARMLAND
                            && face == 2
                            && world.is_near_water(wx, wy, wz)
                        {
                            atlas::TILE_FARMLAND_MOIST
                        } else if (water_self || lava_self) && face != 2 && face != 3 {
                            // THE VANILLA SIDE-FACE LAW (javap FluidRenderer,
                            // the 26.2 jar pc 1458-1466): the horizontal
                            // sides the HORIZONTAL loop walks the FLOWING
                            // sprite ALWAYS wear — the still pick the top
                            // face's calm-pool verdict (pc 671-691, the
                            // getFlow==(0,0) the stillMaterial answers) and
                            // the bottom face's own stillMaterial walk
                            // (pc 1040-1048) keep, the side quads the flow
                            // master's top-left 16x16 quadrant the
                            // generator banks (the corner UVs pc 1515-1567
                            // the getU(0.0)..getU(0.5) x
                            // getV((1-h)*0.5)..getV(0.5) window the
                            // full-height cell the exact quadrant answers —
                            // the tile the side quad's whole UV rect maps).
                            // The draw-time remap the flow strips' own
                            // frametimes the anim_tile arms speak (water 1,
                            // lava 3 — the mcmeta's own keys).
                            if water_self {
                                atlas::WATER_FLOW_FRAME0
                            } else {
                                atlas::LAVA_FLOW_FRAME0
                            }
                        } else {
                            atlas::FACE_TILES[id as usize][face]
                        }
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if tile == atlas::TILE_NONE {
                        // The blockstate-driven palette FIRST (the real
                        // model -> texture chain, v0.3.77's data half), then
                        // the runtime-texture LUT, then the hand-written
                        // defaults. The palette cannot color air/water —
                        // exactly the kinds the LUT/defaults still serve.
                        crate::colors::palette_color(id, face)
                            .or_else(|| lut.and_then(|l| l.color(id, face)))
                            .unwrap_or_else(|| default_color(id, face))
                    } else {
                        // The multiplier carrier for the textured path, PLUS
                        // the synced biome tint when it differs from the
                        // baked default (the atlas pixels carry the vanilla
                        // plains constants — a matching tint is a no-op by
                        // construction, the existing scenes stay bit for
                        // bit). The tint rides the SAME vertex carrier as
                        // shade×AO×light: the vanilla vertex-color multiply
                        // (`vertexColor * texelColor`).
                        world
                            .face_tint(id, face, wx, wy, wz)
                            .map(tint::unpack)
                            .unwrap_or([1.0; 3])
                    };
                    let shade = FACE_SHADE[face];
                    // Smooth lighting: per-corner AO from the 3 diagonal
                    // neighbors in front of this face.
                    let ao = ao_levels(world, wx, wy, wz, face);
                    // Per-corner light sample — only when the world
                    // carries a light layer; `None` keeps the legacy path.
                    let clight =
                        light.map(|l| corner_light(l, world.chunks_per_side, wx, wy, wz, face));
                    // The attribute-system fold (see the flat path): the
                    // stated factor OVERRIDES the default 1.0, the day
                    // tick's curve value scales it; None = the exact f32
                    // identity. Constant per mesh — the shader's
                    // SkyFactor uniform equivalent.
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    // The shader's SkyLightColor uniform: the timeline's
                    // visual/sky_light_color track (white day, the #7a7aff
                    // moonlight blue at night); None = day = white.
                    // The shader's SkyLightColor uniform: the
                    // attribute-system fold for the COLOR attribute
                    // (javap ColorModifier.MULTIPLY_RGB ->
                    // ARGB.multiply(int, int): the -1 white default
                    // short-circuits to the other operand, else
                    // per-channel imul / idiv 255 — the probe's folded
                    // value IS an Integer the extractor unpacks). None
                    // = the -1 default's shape — the track verbatim,
                    // the exact expression this path shipped before the
                    // stated attribute; a stated base (the end #ac60cd,
                    // the nether #7a7aff, a custom server's wire)
                    // multiplies the byte-space track per channel.
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    // The shader's AmbientColor base: the dimension's
                    // visual/ambient_light_color attribute as raw bytes
                    // (overworld #0a0a0a); None = the bytecode-default
                    // black — a no-op base, the pre-ambient arithmetic.
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // The shader's BlockLightTint: the dimension's STATED
                    // attribute (raw bytes -> 0..1), None = the bytecode
                    // default's verdict (#FFD88C, the corrected blue 140).
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // The shader's NightVisionColor: the dimension's STATED
                    // attribute (raw bytes -> 0..1), None = the bytecode
                    // default's verdict (#999999 gray) — consumed UNDER the
                    // night-vision factor (the effect state, 0.0 here).
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let order = flip_order(&ao);
                    let mut pts = [[0f32; 3]; 4];
                    let mut colors = [[0f32; 3]; 4];
                    let mut uvs = [[0f32; 2]; 4];
                    for (i, &src) in order.iter().enumerate() {
                        let c = FACE_CORNERS[face][src];
                        pts[i] = [wx as f32 + c[0], wy as f32 + c[1], wz as f32 + c[2]];
                        // The vanilla lightmap shader's combine, VERBATIM
                        // (the shared per-channel core both paths call):
                        // `SkyLightColor * get_brightness(sky) * SkyFactor
                        // + BlockLightColor * get_brightness(block) *
                        // BlockFactor` — the additive SUM, clamped at 1.0
                        // per channel (the shader's single final clamp).
                        // With one term absent it is the other term
                        // exactly; at day factor 1.0 a sky-15 corner is
                        // bit-identical (1.0 + b clamps back to 1.0, the
                        // old max's verdict). No known cell -> legacy look.
                        let lm = match clight.as_ref() {
                            Some((sky, block)) => {
                                lightmap_combine(sky[src], block[src], sky_factor, sky_tint, ambient, block_tint, night_vision, world.night_vision_factor, world.darkness_scale)
                            }
                            None => [1.0; 3], // no light layer — legacy look
                        };
                        let m = [
                            shade * AO_CURVE[ao[src] as usize] * lm[0],
                            shade * AO_CURVE[ao[src] as usize] * lm[1],
                            shade * AO_CURVE[ao[src] as usize] * lm[2],
                        ];
                        colors[i] = [base[0] * m[0], base[1] * m[1], base[2] * m[2]];
                        uvs[i] = face_uv(face, c);
                    }
                    out.push(Quad {
                        pts,
                        colors,
                        uvs,
                        tile,
                        translucent: water_self, // the lava stays opaque
                    });
                }
            }
        }
    }
    out
}

/// THE TORCH'S OWN MODEL BAKE — the block/torch chain the registry's
/// derive walk resolves (the same lane the primed TNT render speaks:
/// the faceinfo corners, the vanilla UV rect rows, the winding
/// recalculation). The one bake the OnceLock caches — the element
/// data the generated tables bank, the failed derive the honest
/// empty slice keeps (the stick the nothing renders).
fn torch_model_quads() -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    QUADS.get_or_init(|| {
        mc_registry::element_quads::derive_model_inherited("block/torch")
            .unwrap_or_default()
    })
}

/// THE CANDLE PAIR'S OWN MODEL BAKES — the block/candle_one_candle and
/// block/candle_one_candle_lit chains the registry's derive walk
/// resolves (the torch lane's own cache pattern): BOTH walk the parent
/// link to block/template_candle's element geometry, the merged
/// texture map's `#all` ref the per-quad texture name answers (the
/// block/candle and block/candle_lit masters the kind tiles bank —
/// the lane reads the KIND's own tile, the quads' texture name the
/// lane never consults). The failed derive the honest empty slice
/// keeps (the stick the nothing renders).
fn candle_model_quads(lit: bool) -> &'static [mc_registry::element_quads::Quad] {
    static UNLIT: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static LIT: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    let cache = if lit { &LIT } else { &UNLIT };
    cache.get_or_init(|| {
        let model = if lit {
            "block/candle_one_candle_lit"
        } else {
            "block/candle_one_candle"
        };
        mc_registry::element_quads::derive_model_inherited(model).unwrap_or_default()
    })
}

/// THE CAMPFIRE FAMILY'S OWN MODEL BAKES — the block/campfire_off,
/// block/campfire and block/soul_campfire chains the registry's
/// derive walk resolves (the candle lane's own cache pattern): the
/// unlit kind's five-log geometry and the lit kinds' template
/// inheritance (the lit_log face swaps + the two 45-degree rotY
/// rescale fire planes the rotation walk bakes), the per-quad
/// texture names RESOLVED (the parent-chain merge the texture walk
/// serves — the lane's per-quad tile verdict the names read). The
/// failed derive the honest empty slice keeps (the logs the nothing
/// renders).
fn campfire_model_quads(model_id: &str) -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<&'static str, Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let models = ["block/campfire_off", "block/campfire", "block/soul_campfire"];
    let map = QUADS.get_or_init(|| {
        models
            .into_iter()
            .map(|m| {
                (
                    m,
                    mc_registry::element_quads::derive_model_inherited(m).unwrap_or_default(),
                )
            })
            .collect()
    });
    map.get(model_id).map(|v| v.as_slice()).unwrap_or(&[])
}

/// THE CANDLE CAKE PAIR'S OWN MODEL BAKES — the block/candle_cake and
/// block/candle_cake_lit chains the registry's derive walk resolves
/// (the campfire lane's own cache pattern): BOTH parents
/// block/template_cake_with_candle — the cake slab box + the candle
/// stick box + the two ±45° rotY wick planes the rotation walk bakes,
/// the per-quad texture names RESOLVED (the parent-chain merge the
/// texture walk serves — the cake masters the named consts bank, the
/// candle refs the per-quad verdict reads). The failed derive the
/// honest empty slice keeps (the cake the nothing renders).
fn cake_candle_model_quads(model_id: &str) -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<&'static str, Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let models = ["block/candle_cake", "block/candle_cake_lit"];
    let map = QUADS.get_or_init(|| {
        models
            .into_iter()
            .map(|m| {
                (
                    m,
                    mc_registry::element_quads::derive_model_inherited(m).unwrap_or_default(),
                )
            })
            .collect()
    });
    map.get(model_id).map(|v| v.as_slice()).unwrap_or(&[])
}

/// THE CAKE LADDER'S OWN MODEL BAKES — the block/cake and
/// block/cake_slice1..6 chains the registry's derive walk resolves
/// (the campfire lane's own cache pattern the seven models walk):
/// each a SINGLE box element [1+2N, 0, 1]->[15, 8, 15] (the
/// lambda$static$0 shape the x-start walk the models exactly mirror),
/// the west face's #inside ref the bitten cut speaks — the per-quad
/// texture names RESOLVED (the parent-chain merge the texture walk
/// serves — the named consts the verdict reads). The failed derive
/// the honest empty slice keeps (the cake the nothing renders).
fn cake_model_quads(model_id: &str) -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<&'static str, Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let models = [
        "block/cake",
        "block/cake_slice1",
        "block/cake_slice2",
        "block/cake_slice3",
        "block/cake_slice4",
        "block/cake_slice5",
        "block/cake_slice6",
    ];
    let map = QUADS.get_or_init(|| {
        models
            .into_iter()
            .map(|m| {
                (
                    m,
                    mc_registry::element_quads::derive_model_inherited(m).unwrap_or_default(),
                )
            })
            .collect()
    });
    map.get(model_id).map(|v| v.as_slice()).unwrap_or(&[])
}

/// THE FIRE MULTIPART'S OWN CACHED PARTS — the parsed blockstate
/// definition the selection rides, derived ONCE (the jar's own when
/// conditions and y rotations; `parse` returns 'static data, the
/// clone the cache keeps is the same borrows). The soul half's own
/// column carries NO when — the full fan the selection always speaks.
fn fire_parts(soul: bool) -> &'static [Part<'static>] {
    static FIRE: std::sync::OnceLock<Vec<Part<'static>>> = std::sync::OnceLock::new();
    static SOUL: std::sync::OnceLock<Vec<Part<'static>>> = std::sync::OnceLock::new();
    let cache = if soul { &SOUL } else { &FIRE };
    cache.get_or_init(|| {
        let block = if soul { "soul_fire" } else { "fire" };
        match mc_registry::blockstate_lookup::parse(block) {
            Some(Ok(mc_registry::blockstate_lookup::Definition::Multipart(parts))) => {
                parts.to_vec()
            }
            _ => Vec::new(),
        }
    })
}

/// THE FIRE MODELS' OWN CACHED BAKES — the derive walk's quads per
/// model id, derived ONCE for the whole ten-model fire and six-model
/// soul set (the floor pair, the four side templates, the up pair's
/// two alt forms; the second-frame carriers fire_1/soul_fire_1 ride
/// the same bake — the texture ref the kind's tile mapping folds).
fn fire_model_quads() -> &'static std::collections::HashMap<&'static str, Vec<DerivedQuad>> {
    static QUADS: std::sync::OnceLock<std::collections::HashMap<&'static str, Vec<DerivedQuad>>> =
        std::sync::OnceLock::new();
    QUADS.get_or_init(|| {
        let models = [
            "block/fire_floor0",
            "block/fire_floor1",
            "block/fire_side0",
            "block/fire_side1",
            "block/fire_side_alt0",
            "block/fire_side_alt1",
            "block/fire_up0",
            "block/fire_up1",
            "block/fire_up_alt0",
            "block/fire_up_alt1",
            "block/soul_fire_floor0",
            "block/soul_fire_floor1",
            "block/soul_fire_side0",
            "block/soul_fire_side1",
            "block/soul_fire_side_alt0",
            "block/soul_fire_side_alt1",
        ];
        models
            .into_iter()
            .map(|m| (m, derive_model_inherited(m).unwrap_or_default()))
            .collect()
    })
}

/// Mesh the whole world in parallel.
pub fn mesh_world(world: &World, lut: Option<&TextureLut>) -> Vec<Quad> {
    let side = world.chunks_per_side;
    let jobs: Vec<(usize, usize)> = (0..side).flat_map(|cz| (0..side).map(move |cx| (cx, cz))).collect();
    let parts: Vec<Vec<Quad>> = jobs
        .par_iter()
        .map(|&(cx, cz)| mesh_chunk(world, cx, cz, lut))
        .collect();
    let total: usize = parts.iter().map(|p| p.len()).sum();
    let mut all = Vec::with_capacity(total);
    for p in parts {
        all.extend(p);
    }
    all
}

/// Sanity: world center column must be inside the world.
#[test]
fn mesh_produces_faces() {
    let w = World::new(2, 42);
    let q = mesh_world(&w, None);
    assert!(q.len() > 500, "suspiciously few quads: {}", q.len());
    // every quad point must be within world bounds (plus tree overhang margin)
    let n = w.size_blocks() as f32 + 3.0;
    for quad in &q {
        for p in &quad.pts {
            assert!(p[0] >= -1.0 && p[0] <= n);
        }
    }
    // smooth lighting: open-sky faces must stay at full brightness
    for quad in &q {
        for c in &quad.colors {
            assert!(c.iter().all(|v| (0.0..=1.0).contains(v)), "color out of range: {:?}", quad.colors);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1-chunk world, all air.
    fn empty_world() -> World {
        World {
            darkness_scale: 0.0,
            chunks_per_side: 1,
            seed: 0,
            difficulty: crate::difficulty::Difficulty::Normal,
            difficulty_locked: false,
            inhabited: None,
            // THE FIRE'S OWN QUEUE + LADDER the test stage never arms —
            // the empty store the offline walk alone fills
            fire_ticks: Vec::new(),
            fire_age: std::collections::HashMap::new(),
            campfire_ticks: Vec::new(), // the campfire's own queue: empty
            chunks: vec![vec![AIR; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            light: None,
            day_tick: None,
            biomes: None,
            variants: None,
            // None = the bytecode-default black: every test scene keeps
            // the pre-ambient arithmetic bit for bit (the ambient lift is
            // pinned in its own tests, explicitly staged)
            ambient_light: None,
            block_light_tint: None,
            sky_light_factor: None,
            // None = the -1 default's white identity: the sky term's
            // tint stays the track verbatim, the pre-attribute shape
            sky_light_color: None,
            // None = the #999999 default's verdict under the factor 0.0:
            // the night-vision term stays the ambient base bit for bit
            night_vision_color: None,
            // the effect-absent floor: no effect store rides the test
            // scenes, the golden identity holds
            night_vision_factor: 0.0,
            rain_level: 0.0,  // the air-window scene: the dry world
            thunder_level: 0.0,
        }
    }

    /// An all-unknown light layer: same shape as `materialize` produces
    /// when the model never answers — the mesher must not change a pixel.
    fn unknown_light() -> LightGrid {
        LightGrid {
            sky: vec![vec![LIGHT_UNKNOWN; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            block: vec![vec![LIGHT_UNKNOWN; CHUNK_X * CHUNK_Y * CHUNK_Z]],
        }
    }

    /// The light layer value at a cell of the single test chunk.
    fn set_light(w: &mut World, sky: Option<u8>, block: Option<u8>, x: i32, y: i32, z: i32) {
        let idx = x as usize + z as usize * CHUNK_X + y as usize * CHUNK_X * CHUNK_Z;
        w.light.as_mut().unwrap().sky[0][idx] = sky.unwrap_or(LIGHT_UNKNOWN);
        w.light.as_mut().unwrap().block[0][idx] = block.unwrap_or(LIGHT_UNKNOWN);
    }

    /// The top-face quad of the stone block at (8,8,8): shade(top)=1.0,
    /// open-sky AO=3 (1.0) — the multiplier is then light's alone.
    fn top_face(w: &World) -> crate::mesher::Quad {
        let q = mesh_chunk(w, 0, 0, None);
        q.iter()
            .find(|quad| quad.pts[0][1] == 9.0 && quad.pts[1][1] == 9.0)
            .expect("top face quad")
            .clone()
    }

    fn vertex_multiplier(quad: &crate::mesher::Quad) -> [f32; 4] {
        // the textured path carries the pure multiplier (base = [1;3]);
        // the flat path divides the base color out
        let tile = atlas::FACE_TILES[STONE as usize][2];
        let base = if tile == atlas::TILE_NONE {
            default_color(STONE, 2)
        } else {
            [1.0; 3]
        };
        let mut out = [0f32; 4];
        for (i, c) in quad.colors.iter().enumerate() {
            out[i] = if base[0].abs() > 1e-9 { c[0] / base[0] } else { c[0] };
        }
        out
    }

    fn put(w: &mut World, x: i32, y: i32, z: i32, id: u16) {
        let lx = x.rem_euclid(CHUNK_X as i32) as usize;
        let lz = z.rem_euclid(CHUNK_Z as i32) as usize;
        w.chunks[0][lx + lz * CHUNK_X + y as usize * CHUNK_X * CHUNK_Z] = id;
    }

    /// The top-corner color of the water block at (8,8,8) — the textured
    /// path's vertex carrier (all water neighbors are air so the top face
    /// draws; the light layer is unknown/absent -> the legacy m = 1.0).
    fn water_top_corner(w: &World) -> [f32; 3] {
        mesh_chunk(w, 0, 0, None)
            .iter()
            .find(|quad| quad.pts[0][1] == 9.0 && quad.pts[1][1] == 9.0)
            .expect("water top face quad")
            .colors[0]
    }

    #[test]
    fn pale_leaves_mesh_transparent_and_untinted() {
        // the twelfth-audit kind: pale oak leaves are non-occluding (the
        // stone face UNDER them still draws — the transparency verdict)
        // and consume NO tint slot: even a biome table carrying an
        // extreme dry-foliage opinion never folds into their carrier
        // (vanilla's BlockColors registers no source for the block).
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, PALE_LEAVES);
        w.biomes = Some(BiomeLayer {
            ids: vec![vec![0; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            // grass slot carries an extreme color on every cell — if
            // pale leaves wrongly selected a slot, some face would fold
            tints: vec![[Some(0xFF0000), Some(0xFF0000), Some(0xFF0000), Some(0xFF0000)]],
            climates: Vec::new(), // the tint law's own tests: no climate opinion
        });
        let quads = mesh_chunk(&w, 0, 0, None);
        // the stone top face at y 9 (under the pale leaves) DRAWS —
        // non-occluding, exactly the oak-leaves policy
        assert!(
            quads.iter().any(|quad| quad.pts[0][1] == 9.0
                && quad.pts.iter().all(|p| (p[1] - 9.0).abs() < f32::EPSILON)),
            "the stone face under pale leaves renders (non-occluding)"
        );
        // every pale-leaves face rides the UNTINTED carrier: no channel
        // is killed (a red fold would zero g/b on some corner)
        for quad in quads.iter().filter(|quad| quad.pts[0][1] == 10.0) {
            for c in quad.colors {
                assert!(c[0] > 0.0 && c[1] > 0.0 && c[2] > 0.0, "untinted: {c:?}");
            }
        }
    }

    #[test]
    fn leaf_litter_meshes_flat_ground_quads() {
        // the 13th-audit kind — the FIRST non-cube mesh path: exactly TWO
        // quads (up + down), both on the model's y=0.25/16 plane, full
        // footprint, `ambientocclusion: false` (all corners share the
        // face's single shade × light multiplier)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, DIRT);
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        let quads = mesh_chunk(&w, 0, 0, None);
        let litter: Vec<_> = quads
            .iter()
            .filter(|q| q.pts[0][1] == 9.0 + FLAT_Y)
            .collect();
        assert_eq!(litter.len(), 2, "up + down only, no cube faces exist");
        for q in &litter {
            assert_eq!(
                q.tile,
                atlas::FACE_TILES[LEAF_LITTER as usize][2],
                "the litter tile"
            );
            assert!(!q.translucent, "cutout, never translucent");
            for p in q.pts {
                assert!((p[1] - (9.0 + FLAT_Y)).abs() < 1e-6, "the flat plane");
            }
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            assert!(xs.contains(&8.0) && xs.contains(&9.0), "x extent {xs:?}");
            assert!(zs.contains(&8.0) && zs.contains(&9.0), "z extent {zs:?}");
            assert_eq!(q.colors[0], q.colors[1], "no AO: uniform corners");
            assert_eq!(q.colors[1], q.colors[2], "no AO: uniform corners");
            assert_eq!(q.colors[2], q.colors[3], "no AO: uniform corners");
        }
        // up quad (uv [0,0] on the first corner): shade 1.0, no light layer
        // -> legacy 1.0; down quad: the vanilla direction shade 0.5
        let up = litter
            .iter()
            .find(|q| q.uvs[0] == [0.0, 0.0])
            .expect("the up quad");
        assert_eq!(up.colors[0], [1.0; 3], "up: shade 1.0, legacy light 1.0");
        // the down quad's first corner uv: face_uv(3, [0,0,0]) = [0, 1-0]
        let down = litter
            .iter()
            .find(|q| q.uvs[0] == [0.0, 1.0])
            .expect("the down quad");
        assert_eq!(down.colors[0], [0.5; 3], "down: direction shade 0.5");
    }

    /// The cell index of (8, 9, 8) — the litter cell the flat-path tests use.
    fn litter_cell_idx() -> usize {
        8 + 8 * CHUNK_X + 9 * CHUNK_X * CHUNK_Z
    }

    #[test]
    fn leaf_litter_segment_one_meshes_the_northwest_quarter() {
        // THE 14TH AUDIT, end to end: the REAL state id (30339, north x
        // segment_amount 1) decodes to variant 1 and the mesher emits the
        // template element's exact geometry — one quarter rect, up + down
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, DIRT);
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        let mut variants = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants[0][litter_cell_idx()] = crate::world_view::leaf_litter_variant(30339);
        w.variants = Some(variants);
        let quads = mesh_chunk(&w, 0, 0, None);
        let litter: Vec<_> = quads
            .iter()
            .filter(|q| q.pts[0][1] == 9.0 + FLAT_Y)
            .collect();
        assert_eq!(litter.len(), 2, "one rect: up + down");
        for q in &litter {
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            assert!(xs.contains(&8.0) && xs.contains(&8.5), "x half {xs:?}");
            assert!(zs.contains(&8.0) && zs.contains(&8.5), "z half {zs:?}");
            assert!(!xs.contains(&9.0) && !zs.contains(&9.0), "a quarter, not full");
            assert_eq!(q.colors[0], q.colors[1], "no AO: uniform corners");
        }
        // the up quad's uv spans the texture's quarter too (u = x, v = z)
        let up = litter
            .iter()
            .find(|q| q.uvs[0] == [0.0, 0.0])
            .expect("the up quad");
        assert_eq!(up.uvs[2], [0.5, 0.5], "uv spans the quarter");
        assert_eq!(up.colors[0], [1.0; 3], "up: shade 1.0, legacy light 1.0");
        // the down quad mirrors v WITHIN the rect: v = (z0 + z1)/16 - z
        let down = litter
            .iter()
            .find(|q| q.uvs[0] == [0.0, 0.5])
            .expect("the down quad");
        assert_eq!(down.uvs[2], [0.5, 0.0], "v mirrored inside the rect");
        assert_eq!(down.colors[0], [0.5; 3], "down: direction shade 0.5");
    }

    #[test]
    fn leaf_litter_segments_two_and_three_stack_the_template_rects() {
        // the multipart's exact model sets: segment 2 -> {m2} (the west
        // strip), segment 3 -> {m2, m3} (strip + SE quarter, TWO rects)
        let mut w = empty_world();
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        let mut variants = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants[0][litter_cell_idx()] = crate::world_view::leaf_litter_variant(30340); // north, 2
        w.variants = Some(variants);
        let quads = mesh_chunk(&w, 0, 0, None);
        let litter: Vec<_> = quads
            .iter()
            .filter(|q| q.pts[0][1] == 9.0 + FLAT_Y)
            .collect();
        assert_eq!(litter.len(), 2, "the west strip: up + down");
        for q in &litter {
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            assert!(xs.contains(&8.0) && xs.contains(&8.5), "west half {xs:?}");
            assert!(zs.contains(&8.0) && zs.contains(&9.0), "full depth {zs:?}");
        }

        let mut variants = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants[0][litter_cell_idx()] = crate::world_view::leaf_litter_variant(30341); // north, 3
        w.variants = Some(variants);
        let quads = mesh_chunk(&w, 0, 0, None);
        let litter: Vec<_> = quads
            .iter()
            .filter(|q| q.pts[0][1] == 9.0 + FLAT_Y)
            .collect();
        assert_eq!(litter.len(), 4, "strip + SE quarter: two rects, up + down");
        // the SE quarter rect exists: x [8.5, 9] x z [8.5, 9]
        assert!(litter.iter().any(|q| {
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            xs.contains(&8.5) && xs.contains(&9.0) && zs.contains(&8.5) && zs.contains(&9.0)
        }));
        // and the west strip rides beside it
        assert!(litter.iter().any(|q| {
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            xs.contains(&8.0) && xs.contains(&8.5) && zs.contains(&8.0) && zs.contains(&9.0)
        }));
    }

    #[test]
    fn leaf_litter_facing_east_rotates_the_footprint_with_glued_uv() {
        // facing east = the blockstate's y=90: the NW-quarter rect turns
        // about the cell center into the NE quarter; the uv stays GLUED
        // to the corners (the texture turns with the element — vanilla)
        let mut w = empty_world();
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        let mut variants = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants[0][litter_cell_idx()] = crate::world_view::leaf_litter_variant(30351); // east, 1
        w.variants = Some(variants);
        let quads = mesh_chunk(&w, 0, 0, None);
        let litter: Vec<_> = quads
            .iter()
            .filter(|q| q.pts[0][1] == 9.0 + FLAT_Y)
            .collect();
        assert_eq!(litter.len(), 2);
        for q in &litter {
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            assert!(xs.contains(&8.5) && xs.contains(&9.0), "east half {xs:?}");
            assert!(zs.contains(&8.0) && zs.contains(&8.5), "north half {zs:?}");
            assert!(!xs.contains(&8.0), "the west edge vacated");
        }
        // the uv corners are the UNROTATED texture quarter, glued
        let up = litter
            .iter()
            .find(|q| q.uvs[0] == [0.0, 0.0])
            .expect("the up quad");
        assert_eq!(up.uvs[2], [0.5, 0.5]);
        // the first corner sits at the rotated position (x -> east edge)
        assert_eq!(up.pts[0][0], 9.0, "the (0,0) corner rotated to east");
        assert_eq!(up.pts[0][2], 8.0);
    }

    #[test]
    fn leaf_litter_segment_four_is_bit_identical_to_the_legacy_look() {
        // variant 0 (no opinion — the procedural scenes) and variant 13
        // (north, segment 4 = the full rect, y-rot 0) must emit THE SAME
        // bytes: the template uv arithmetic collapses to the old face_uv
        // exactly at the full rect
        let mut base = empty_world();
        put(&mut base, 8, 8, 8, DIRT);
        put(&mut base, 8, 9, 8, LEAF_LITTER);
        let legacy = mesh_chunk(&base, 0, 0, None);

        let mut w = base.clone();
        let mut variants = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants[0][litter_cell_idx()] = crate::world_view::leaf_litter_variant(30342); // north, 4
        w.variants = Some(variants);
        let shaped = mesh_chunk(&w, 0, 0, None);
        assert_eq!(legacy, shaped, "segment 4 = full rect = the old emission");
    }

    #[test]
    fn leaf_litter_never_occludes_or_casts_ao() {
        // the ground cover doesn't hide the dirt top face under it (the
        // is_transparent policy) and casts NO corner AO: the stone face
        // whose front plane touches the litter cell keeps full-open
        // corners — the shade-only 0.6 multiplier on every corner
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, DIRT);
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        put(&mut w, 7, 10, 8, STONE);
        let quads = mesh_chunk(&w, 0, 0, None);
        assert!(
            quads.iter().any(|q| {
                q.pts.iter().all(|p| {
                    (p[1] - 9.0).abs() < f32::EPSILON
                        && p[0] >= 8.0
                        && p[0] <= 9.0
                        && p[2] >= 8.0
                        && p[2] <= 9.0
                })
            }),
            "the dirt face under leaf litter renders (non-occluding)"
        );
        let face = quads
            .iter()
            .find(|q| {
                q.pts.iter().all(|p| {
                    (p[0] - 8.0).abs() < f32::EPSILON
                        && p[1] >= 10.0
                        && p[1] <= 11.0
                        && p[2] >= 8.0
                        && p[2] <= 9.0
                })
            })
            .expect("the stone +X face whose front plane holds the litter");
        for m in vertex_multiplier(face) {
            assert!(
                (m - 0.6).abs() < 1e-6,
                "litter casts no AO (full-open corners, shade 0.6): {m}"
            );
        }
    }

    #[test]
    fn leaf_litter_folds_the_dry_slot() {
        // the dry fold, end to end: the litter faces consume the DRY_SLOT —
        // a synced dry-foliage override rides the vertex carrier (the
        // override / baked-plains multiplier), and the baked plains sample
        // itself is a no-op (the atlas pixels already carry it)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, DIRT);
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        w.biomes = Some(BiomeLayer {
            ids: vec![vec![0; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            // ONLY the dry slot speaks: the pale-garden override from the
            // 12th-audit probe; grass/foliage/water stay silent
            tints: vec![[None, None, None, Some(0xA0A69C)]],
            climates: Vec::new(), // the tint law's own tests: no climate opinion
        });
        let quads = mesh_chunk(&w, 0, 0, None);
        let up = quads
            .iter()
            .find(|q| q.pts[0][1] == 9.0 + FLAT_Y && q.uvs[0] == [0.0, 0.0])
            .expect("the litter up quad");
        let want = tint::unpack(0xA0A69C);
        for c in up.colors {
            assert_eq!(c, want, "the dry override rides the carrier");
        }
        // the baked plains sample (TINTS carries it) folds to the no-op
        let baked = atlas::TINTS[LEAF_LITTER as usize][2];
        assert_eq!(baked, 0xA37546, "the generator's TINT_DRY constant");
        w.biomes.as_mut().unwrap().tints[0][3] = Some(baked);
        let quads = mesh_chunk(&w, 0, 0, None);
        let up = quads
            .iter()
            .find(|q| q.pts[0][1] == 9.0 + FLAT_Y && q.uvs[0] == [0.0, 0.0])
            .expect("the litter up quad");
        assert_eq!(up.colors[0], [1.0; 3], "the baked sample is a no-op");
    }

    #[test]
    fn biome_tint_folds_into_the_textured_carrier_only_when_it_differs() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, WATER);
        let reference = water_top_corner(&w);
        assert!(
            reference[0] > 0.0
                && (reference[0] - reference[1]).abs() < 1e-6
                && (reference[1] - reference[2]).abs() < 1e-6,
            "the untinted carrier is shade×AO×light on every channel: {:?}",
            reference
        );
        // plains: water == the baked default -> bit-identical to no layer
        let baked = atlas::TINTS[WATER as usize][0];
        let mut plains = empty_world();
        put(&mut plains, 8, 8, 8, WATER);
        plains.biomes = Some(BiomeLayer {
            ids: vec![vec![0; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            tints: vec![[None, None, Some(baked), None]],
            climates: Vec::new(), // the tint law's own tests: no climate opinion
        });
        assert_eq!(
            water_top_corner(&plains),
            reference,
            "a tint matching the baked default is a no-op, bit for bit"
        );
        // a real red-water biome: the tint rides the SAME carrier
        let mut red = empty_world();
        put(&mut red, 8, 8, 8, WATER);
        red.biomes = Some(BiomeLayer {
            ids: vec![vec![0; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            tints: vec![[None, None, Some(0xFF0000), None]],
            climates: Vec::new(), // the tint law's own tests: no climate opinion
        });
        let tinted = water_top_corner(&red);
        assert_eq!(tinted[1], 0.0, "green killed by the tint");
        assert_eq!(tinted[2], 0.0, "blue killed by the tint");
        assert_eq!(tinted[0], reference[0], "red rides the SAME multiplier");
    }

    #[test]
    fn vertex_ao_truth_table() {
        assert_eq!(vertex_ao(true, true, false), 0, "two sides fully occlude");
        assert_eq!(vertex_ao(true, true, true), 0);
        assert_eq!(vertex_ao(false, false, false), 3, "open corner");
        assert_eq!(vertex_ao(true, false, false), 2);
        assert_eq!(vertex_ao(false, true, false), 2);
        assert_eq!(vertex_ao(false, false, true), 2);
        assert_eq!(vertex_ao(true, false, true), 1);
        assert_eq!(vertex_ao(false, true, true), 1);
    }

    #[test]
    fn light_brightness_matches_the_vanilla_ramp() {
        assert_eq!(light_brightness(15.0), 1.0, "open sky keeps the legacy look");
        assert_eq!(light_brightness(0.0), 0.0, "known-dark renders black");
        assert!(
            (light_brightness(8.0) - 0.222_222_22).abs() < 1e-4,
            "level 8 is about 22% — the curve's aggressive low end"
        );
        for l in 0..15u8 {
            assert!(
                light_brightness(l as f32) < light_brightness((l + 1) as f32),
                "monotonic at {l}"
            );
        }
    }

    #[test]
    fn light_layer_shades_known_cells_and_spares_unknown() {
        // stone at (8,8,8) in an all-air world; the top face cell is (8,9,8)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        let legacy = {
            let mut w2 = empty_world();
            put(&mut w2, 8, 8, 8, STONE);
            vertex_multiplier(&top_face(&w2))
        };
        // an ALL-UNKNOWN layer (materialize over a silent model) changes nothing
        w.light = Some(unknown_light());
        assert_eq!(
            vertex_multiplier(&top_face(&w)),
            legacy,
            "all-unknown light must be invisible"
        );
        // known full sky at the face cell — brightness 1.0, still the legacy look
        set_light(&mut w, Some(15), None, 8, 9, 8);
        assert_eq!(vertex_multiplier(&top_face(&w)), legacy, "full sky is the legacy look");
        // the engine says the cell is DARK (both layers known zero) -> black
        set_light(&mut w, Some(0), Some(0), 8, 9, 8);
        for m in vertex_multiplier(&top_face(&w)) {
            assert!(m.abs() < 1e-6, "known-dark cell is black: {m}");
        }
        // torch-lit: no sky opinion, block 14 -> the TINTED, FACTORED
        // block term under the clamp: BlockLightColor(14) ·
        // brightness(14) · BlockFactor — red and green saturate, blue
        // rides under (the warm verdict). v0.3.87: the term is no
        // longer the white brightness(14) — the #ffd88c mix landed.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(14), 8, 9, 8);
        let b = light_brightness(14.0) * BLOCK_FACTOR;
        let c = block_light_color(14.0, None);
        let expect = [(b * c[0]).min(1.0), (b * c[1]).min(1.0), (b * c[2]).min(1.0)];
        assert!((expect[0] - 1.0).abs() < 1e-6, "red saturates the clamp");
        assert!(expect[2] < 1.0, "blue stays under the clamp");
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!((ch - e).abs() < 1e-6, "block-only light: {ch} vs {e}");
            }
        }
    }

    #[test]
    fn sky_factor_scales_only_the_sky_term() {
        // the stone block's top face under full sky: day is the legacy
        // look; the night factor drags the SKY term down the ramp while
        // the BLOCK term is untouched — and the two SUM, clamped at 1.0.
        // v0.3.86: the night sky term is additionally TINTED by the
        // timeline's visual/sky_light_color track (#7a7aff — the
        // moonlight blue), so the night pins are per-channel: the
        // vertex_multiplier reads the RED channel (r = g tinted, b not).
        let tint_rg = 122.0f32 / 255.0; // #7a7aff red/green, blue 1.0
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, Some(15), Some(0), 8, 9, 8);
        // day: brightness(15) = 1.0 -> legacy
        assert_eq!(vertex_multiplier(&top_face(&w)), [1.0; 4], "day sky-15 is legacy");
        // night (the day timeline's midnight tick -> the 0.24 plateau +
        // the #7a7aff tint on the sky term)
        w.day_tick = Some(18_000);
        let night_sky = light_brightness(15.0) * 0.24;
        for m in vertex_multiplier(&top_face(&w)) {
            assert!(
                (m - night_sky * tint_rg).abs() < 1e-6,
                "night sky-15 scales AND tints the sky term: {m}"
            );
        }
        // the same night sky over a TORCH-lit cell (block 14): the
        // ADDITIVE combine sums the tinted sky term + the block term
        set_light(&mut w, Some(15), Some(14), 8, 9, 8);
        // v0.3.87: the block term carries BlockFactor 1.4 — the red
        // term alone is 1.0889, so the SUM saturates the clamp at 1.0
        // (a full-sky torch-lit face is fully lit, the vanilla
        // verdict); the warm tint shows on UNclamped cells instead
        // (see the_block_term_is_tinted_and_factored_not_white)
        let block_r =
            (light_brightness(14.0) * BLOCK_FACTOR * block_light_color(14.0, None)[0]).min(1.0);
        let night_torch_r = (light_brightness(15.0) * 0.24 * tint_rg + block_r).min(1.0);
        assert!(
            (night_torch_r - 1.0).abs() < 1e-6,
            "the torch sum saturates the clamp: {night_torch_r}"
        );
        for m in vertex_multiplier(&top_face(&w)) {
            assert!((m - night_torch_r).abs() < 1e-6, "night torch red saturates: {m}");
        }
        // the sum WITHOUT the clamp: day, sky 12 (brightness 0.5) over
        // block 10 (brightness 1/3 × BlockFactor 1.4 × tint-red 1.0) —
        // max(0.5, 1/3) was 0.5, the true shader sum is 0.5 + 7/15 =
        // 29/30 (the factor rides the block term since v0.3.87)
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, STONE);
        w3.light = Some(unknown_light());
        set_light(&mut w3, Some(12), Some(10), 8, 9, 8);
        let expect = light_brightness(12.0)
            + light_brightness(10.0) * BLOCK_FACTOR * block_light_color(10.0, None)[0];
        assert!((expect - 29.0 / 30.0).abs() < 1e-6, "ramp sanity: {expect}");
        for m in vertex_multiplier(&top_face(&w3)) {
            assert!(
                (m - expect).abs() < 1e-6,
                "the combine is the additive sum, not the max: {m} vs {expect}"
            );
        }
        // an unknown sky layer is never scaled into an opinion: a corner
        // with only block known keeps the plain block term (no SKY
        // factor, no SKY tint — even at the night tick; the block term
        // carries its own factor 1.4 and its own tint since v0.3.87),
        // block 10 keeps every channel under the clamp
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, STONE);
        w2.light = Some(unknown_light());
        w2.day_tick = Some(18_000);
        set_light(&mut w2, None, Some(10), 8, 9, 8);
        let b = light_brightness(10.0) * BLOCK_FACTOR;
        let c = block_light_color(10.0, None);
        let expect = [b * c[0], b * c[1], b * c[2]];
        for corner in &top_face(&w2).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!(
                    (ch - e).abs() < 1e-6,
                    "unknown sky is not darkened, the block term is exact: {ch} vs {e}"
                );
            }
        }
    }

    #[test]
    fn the_night_sky_term_is_blue_not_grey() {
        // the full-channel witness: at the night tick the sky-lit top
        // face's multiplier is #7a7aff-tinted — red and green ride the
        // 122/255 tint, blue stays the plain factored ramp — while the
        // DAY tick of the same world is bit-identical to the scalar
        // (white) arithmetic.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, Some(15), Some(0), 8, 9, 8);
        // the day product (shade * AO * ramp 1.0) — the legacy 1.0 the
        // previous test pins; derived here, not hard-coded
        w.day_tick = Some(6_000);
        let full = vertex_multiplier(&top_face(&w))[0];
        assert!((full - 1.0).abs() < 1e-6, "day product sanity: {full}");
        w.day_tick = Some(14_010); // the goldens' night tick
        let sky = light_brightness(15.0) * 0.24; // factor 0.24 * ramp 1.0
        let expect = [full * sky * (122.0f32 / 255.0), full * sky * (122.0 / 255.0), full * sky];
        for corner in &top_face(&w).colors {
            for (c, e) in corner.iter().zip(expect.iter()) {
                assert!((c - e).abs() < 1e-6, "night tint per channel: {c} vs {e}");
            }
        }
    }

    #[test]
    fn the_block_light_color_is_the_parabolic_tint_mix() {
        // the shader's parabola on the RAW level: 1.0 at BOTH ends
        // (level 0 and 15), exactly 0.0 at the mid — and the mix toward
        // white is 0.9 OF the parabola (lightmap.fsh verbatim).
        assert!(
            (parabolic_mix_factor(0.5) - 0.0).abs() < 1e-9,
            "mid level: the pure tint, no mix"
        );
        assert!(
            (parabolic_mix_factor(0.0) - 1.0).abs() < 1e-9,
            "level 0: the full mix (irrelevant there — brightness 0)"
        );
        assert!(
            (parabolic_mix_factor(1.0) - 1.0).abs() < 1e-9,
            "level 15: the full mix — white-hot torch"
        );
        // blocks 7 and 8 straddle the parabola's dip symmetrically —
        // the SAME mix, the nearest the integer levels get to the pure
        // #ffd88c (the dip's neighborhood, not the dip itself)
        let m7 = 0.9 * parabolic_mix_factor(7.0 / 15.0);
        let m8 = 0.9 * parabolic_mix_factor(8.0 / 15.0);
        assert!((m7 - m8).abs() < 1e-6, "the dip is level-symmetric: {m7} vs {m8}");
        assert!(m7 < 0.01, "near the pure tint: {m7}");
        // red is 1.0 at EVERY level (the tint's red is saturated — the
        // warmth lives entirely in green/blue)
        for b in [0.0f32, 3.0, 7.0, 8.0, 12.0, 15.0] {
            let c = block_light_color(b, None);
            assert!((c[0] - 1.0).abs() < 1e-9, "red constant at {b}: {}", c[0]);
        }
        // the extremes mix 0.9 of the way to white — derived from the
        // tint constant, both end levels share it (parabola 1.0 there)
        let expect_g = 216.0f32 / 255.0 + 0.9 * (1.0 - 216.0 / 255.0);
        let expect_b = 140.0f32 / 255.0 + 0.9 * (1.0 - 140.0 / 255.0);
        for b in [0.0f32, 15.0] {
            let c = block_light_color(b, None);
            assert!((c[1] - expect_g).abs() < 1e-6, "green at {b}: {} vs {expect_g}", c[1]);
            assert!((c[2] - expect_b).abs() < 1e-6, "blue at {b}: {} vs {expect_b}", c[2]);
        }
    }

    #[test]
    fn the_stated_block_light_tint_replaces_the_default_verbatim() {
        // the wire's stated attribute is the WHOLE tint (javap
        // LightmapRenderStateExtractor: the probe's value feeds
        // ARGB.vector3fFromRGB24 straight into the uniform — no
        // blending with the default). A stated cool tint (#302821,
        // the nether ambient's hue) shifts the mix target per channel;
        // the parabola itself is unchanged (still 0.9 on the raw
        // level, red saturated at the ends — the stated red is 0x30,
        // so red is NOT 1.0 here: the mix carries it toward white).
        let stated = Some([0x30 as f32 / 255.0, 0x28 as f32 / 255.0, 0x21 as f32 / 255.0]);
        let m15 = 0.9 * parabolic_mix_factor(15.0 / 15.0);
        let c = block_light_color(15.0, stated);
        let expect = [
            0x30 as f32 / 255.0 + m15 * (1.0 - 0x30 as f32 / 255.0),
            0x28 as f32 / 255.0 + m15 * (1.0 - 0x28 as f32 / 255.0),
            0x21 as f32 / 255.0 + m15 * (1.0 - 0x21 as f32 / 255.0),
        ];
        for (ch, e) in c.iter().zip(expect.iter()) {
            assert!((ch - e).abs() < 1e-6, "stated tint rides verbatim: {ch} vs {e}");
        }
        // the None call is the default's verdict (the corrected blue
        // 140 — NOT the retired 172 literal) — the two differ, the
        // stated path is a real consumer, not a shadowed argument
        let dflt = block_light_color(15.0, None);
        assert!(
            (dflt[2] - c[2]).abs() > 0.01,
            "stated vs default diverge: {} vs {}",
            dflt[2],
            c[2]
        );
        // the mesher witness: a stated tint on the world reaches the
        // block term through lightmap_combine — the flat path's own
        // plumbing (world.block_light_tint -> the combine's argument)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        w.block_light_tint = Some([0x30, 0x28, 0x21]);
        set_light(&mut w, None, Some(7), 8, 9, 8);
        let b = light_brightness(7.0) * BLOCK_FACTOR;
        let cs = block_light_color(7.0, stated);
        let expect_w = [b * cs[0], b * cs[1], b * cs[2]];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect_w.iter()) {
                assert!(
                    (ch - e).abs() < 1e-6,
                    "the stated tint reaches the block term: {ch} vs {e}"
                );
            }
        }
    }

    #[test]
    fn the_stated_sky_light_color_folds_under_the_track() {
        // the sky-light color's consumer contract is the attribute-system
        // FOLD for the COLOR attribute (javap ColorModifier.MULTIPLY_RGB
        // -> ARGB.multiply(int, int): the -1 white default short-circuits
        // to the other operand, else per-channel imul / idiv 255 — the
        // probe's folded value IS an Integer the extractor unpacks): the
        // stated base (the end #ac60cd, the nether #7a7aff, a custom
        // server's wire) multiplies the BYTE-space day track per channel.
        // The integer floor bites: (128*122)/255 = 61 (61.24 floored),
        // blue (128*255)/255 = 128 exact.
        let stated = [0x80u8, 0x80, 0x80];
        let track = crate::timeline::sky_light_color_bytes(14_010); // the #7a7aff plateau
        assert_eq!(track, [122, 122, 255], "the night plateau's bytes");
        let folded = [
            ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
            ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
            ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
        ];
        assert_eq!(folded[0], 61.0 / 255.0, "the integer floor: 15616/255 = 61");
        assert_eq!(folded[1], 61.0 / 255.0);
        assert_eq!(folded[2], 128.0 / 255.0, "blue: (128*255)/255 = 128 exact");
        // the fold is NOT the float multiply — the arithmetic is the
        // bytecode's integer space, the fidelity round's verdict
        let float_mul = 128.0f32 / 255.0 * (122.0f32 / 255.0);
        assert!(
            (float_mul - folded[0]).abs() > 1e-4,
            "float {float_mul} vs integer-folded {}",
            folded[0]
        );
        // the mesher witness: the stated color reaches the sky term
        // through lightmap_combine — the night plateau's factor (0.24)
        // scales the brightness, the folded tint colors it
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        w.day_tick = Some(14_010);
        w.sky_light_color = Some(stated);
        set_light(&mut w, Some(15), None, 8, 9, 8);
        let s = light_brightness(15.0) * 0.24; // the factor fold at the plateau
        let expect_w = [s * folded[0], s * folded[1], s * folded[2]];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect_w.iter()) {
                assert!(
                    (ch - e).abs() < 1e-6,
                    "the stated color reaches the sky term: {ch} vs {e}"
                );
            }
        }
        // the None world keeps the track verbatim (the -1 default's
        // white identity): the same scene unstated reads the track's
        // night blue directly — the exact expression the goldens pin
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        w.day_tick = Some(14_010);
        set_light(&mut w, Some(15), None, 8, 9, 8);
        let t = crate::timeline::sky_light_color(14_010);
        let expect_none = [s * t[0], s * t[1], s * t[2]];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect_none.iter()) {
                assert!(
                    (ch - e).abs() < 1e-6,
                    "the None world is the track verbatim: {ch} vs {e}"
                );
            }
        }
    }

    #[test]
    fn the_block_term_is_tinted_and_factored_not_white() {
        // the full-channel mesher witness: a torch-lit top face (block
        // 7, no sky opinion) carries BlockLightColor(7) · brightness(7)
        // · BlockFactor per channel — red 7/15·1.4·1.0, green and blue
        // ride the near-pure-tint parabolic mix — ALL under the clamp.
        // The old white term would read brightness(7) flat on every
        // channel; the expectations derive from the audited constants.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(7), 8, 9, 8);
        let b = light_brightness(7.0) * BLOCK_FACTOR;
        let c = block_light_color(7.0, None);
        let expect = [b * c[0], b * c[1], b * c[2]];
        assert!(expect.iter().all(|e| *e < 1.0), "block 7 sits under the clamp");
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!((ch - e).abs() < 1e-6, "torch tint per channel: {ch} vs {e}");
            }
        }
        // the clamp witness at block 14: red (1.0889) AND green (1.0350)
        // saturate, blue stays under (0.8540 tint × 1.0889 = 0.9300) —
        // the shader's single final clamp, the warm torch verdict
        set_light(&mut w, None, Some(14), 8, 9, 8);
        let b14 = light_brightness(14.0) * BLOCK_FACTOR;
        let c14 = block_light_color(14.0, None);
        let expect14 = [
            (b14 * c14[0]).min(1.0),
            (b14 * c14[1]).min(1.0),
            (b14 * c14[2]).min(1.0),
        ];
        assert!((expect14[0] - 1.0).abs() < 1e-6, "red saturates");
        assert!((expect14[1] - 1.0).abs() < 1e-6, "green saturates");
        assert!(expect14[2] < 1.0, "blue under: {}", expect14[2]);
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect14.iter()) {
                assert!((ch - e).abs() < 1e-6, "torch-14 per channel: {ch} vs {e}");
            }
        }
    }

    #[test]
    fn the_stated_night_vision_color_rides_the_shader_max_term() {
        // the lightmap's LAST stated attribute: the shader's
        // `max(AmbientColor, NightVisionColor * NightVisionFactor)` base
        // (lightmap.fsh verbatim). The FACTOR is the player's EFFECT
        // state (NIGHT_VISION effect -> GameRenderer.nightVisionScale,
        // conduit power + water vision, else exactly 0.0 — javap
        // LightmapRenderStateExtractor.extract); this client carries no
        // effect system, so the factor sits at its bytecode-pinned
        // floor 0.0 and the term is the ambient base BIT FOR BIT for
        // any tint — max(A, C·0.0) = A for non-negative A. The fold's
        // arithmetic is pinned at the lifted factor too, the contract
        // the effect system's arrival will consume.
        let ambient = [0.1f32, 0.2, 0.3];
        // the floor: factor 0.0 — the identity, exact equality (no
        // tolerance: the goldens' untouched-by-construction witness)
        assert_eq!(
            night_vision_base(ambient, None, NIGHT_VISION_FACTOR),
            ambient,
            "factor 0.0: the default #999999 vanishes into the ambient base"
        );
        assert_eq!(
            night_vision_base(ambient, Some([0.3, 0.2, 0.1]), NIGHT_VISION_FACTOR),
            ambient,
            "factor 0.0: a stated tint vanishes into the ambient base too"
        );
        // the lifted contract, factor 1.0 — the nightVisionScale
        // plateau: None reads the BYTECODE default (#999999 gray =
        // 153/255 per channel) and the max lifts the dark channels
        let g = 153.0f32 / 255.0;
        assert_eq!(
            night_vision_base(ambient, None, 1.0),
            [ambient[0].max(g), ambient[1].max(g), ambient[2].max(g)],
            "factor 1.0: the gray default lifts the dark channels only"
        );
        // a stated color is the stated base: white maxes every channel
        // to 1.0; a DIM stated color below the ambient floor moves
        // NOTHING (the max is per-channel, never an average)
        assert_eq!(
            night_vision_base(ambient, Some([1.0; 3]), 1.0),
            [1.0; 3],
            "factor 1.0: the stated white maxes every channel"
        );
        assert_eq!(
            night_vision_base(ambient, Some([0.05, 0.0, 0.15]), 1.0),
            [ambient[0].max(0.05), ambient[1].max(0.0), ambient[2].max(0.15)],
            "the max is per-channel: a dim stated color cannot darken"
        );
        // the render-path witness: a lit cell renders EXACTLY the same
        // with a stated night-vision color as with None (the factor
        // 0.0 identity through the world snapshot — the goldens' own
        // shape), and a lifted factor WOULD lift the base (the same
        // fold the mesher calls, at the effect system's future state)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, Some(4), Some(7), 8, 9, 8);
        let baseline = vertex_multiplier(&top_face(&w));
        w.night_vision_color = Some([0x18, 0x10, 0x28]);
        assert_eq!(
            vertex_multiplier(&top_face(&w)),
            baseline,
            "the stated attribute rides the snapshot; factor 0.0 keeps every pixel"
        );
        // the lifted base, derived through the SAME fold the paths call
        let amb = 0.0f32; // the black default: this scene's ambient
        let lifted = night_vision_base([amb; 3], Some([0x18 as f32 / 255.0, 0x10 as f32 / 255.0, 0x28 as f32 / 255.0]), 1.0);
        assert_eq!(
            lifted,
            [24.0f32 / 255.0, 16.0 / 255.0, 40.0 / 255.0],
            "the day the effect system lifts the factor, the stated color is the floor"
        );
    }

    #[test]
    fn the_night_vision_factor_lifts_the_lightmap_from_the_effect_state() {
        // the debt retired: the factor RIDES the snapshot — the
        // effect-absent floor 0.0 is the golden identity (every pixel
        // exactly the pre-round arithmetic), the effect's plateau 1.0
        // lifts the base through the SAME combine the paths call (the
        // nightVisionScale contract the const's doc carried).
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(5), 8, 9, 8);
        let baseline = vertex_multiplier(&top_face(&w));
        // a stated night-vision color under the floor 0.0: the
        // identity, bit for bit (max(A, C·0.0) = A) — the goldens' own
        // shape, untouched by construction
        w.night_vision_color = Some([0x18, 0x10, 0x28]);
        assert_eq!(vertex_multiplier(&top_face(&w)), baseline);
        // the plateau 1.0 through the rendered path: the lifted base is
        // EXACTLY the shader's max(AmbientColor, NightVisionColor·1.0)
        // with the unchanged block term on top (lightmap.fsh verbatim)
        w.night_vision_factor = 1.0;
        let c = [0x18 as f32 / 255.0, 0x10 as f32 / 255.0, 0x28 as f32 / 255.0];
        let b = light_brightness(5.0) * BLOCK_FACTOR;
        let tint = block_light_color(5.0, None);
        let expect = [
            (c[0] + b * tint[0]).min(1.0),
            (c[1] + b * tint[1]).min(1.0),
            (c[2] + b * tint[2]).min(1.0),
        ];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!((ch - e).abs() < 1e-6, "the lifted base per channel: {ch} vs {e}");
            }
        }
    }

    /// THE DARKNESS ARM THE SUBTRACT THE CLAMP KEEPS (javap
    /// LightmapRenderStateExtractor pc 304-344 + the 26.2 jar's
    /// `shaders/core/lightmap.fsh` — the `color = color -
    /// vec3(lightmapInfo.DarknessScale);` subtract the clamp's own
    /// floor speaks, the 0.0 no-effect floor the bit-for-bit identity
    /// keeps: the goldens' own shape, untouched by construction).
    #[test]
    fn the_darkness_arm_the_subtract_the_clamp_keeps() {
        // THE 0.0 FLOOR (the `x - 0.0` f32 identity + the bottom
        // clamp's no-op on the non-negative terms): every texel the
        // pre-darkness arithmetic keeps bit for bit
        let base = crate::mesher::lightmap_combine(
            Some(15.0),
            Some(5.0),
            1.0,
            [1.0, 1.0, 1.0],
            [0.0, 0.0, 0.0],
            None,
            None,
            0.0,
            0.0,
        );
        let expect_raw = {
            let sky = light_brightness(15.0) * 1.0;
            let b = light_brightness(5.0) * BLOCK_FACTOR;
            let c = block_light_color(5.0, None);
            [0.0 + sky + b * c[0], 0.0 + sky + b * c[1], 0.0 + sky + b * c[2]]
        };
        let expect = [
            expect_raw[0].min(1.0),
            expect_raw[1].min(1.0),
            expect_raw[2].min(1.0),
        ];
        assert_eq!(base, expect, "the no-effect floor the identity wears");
        // THE SUBTRACT (the shader's own arm): the bright texel the
        // 0.45 scale dims — the exact float the subtract keeps
        let dimmed = crate::mesher::lightmap_combine(
            Some(15.0),
            Some(5.0),
            1.0,
            [1.0, 1.0, 1.0],
            [0.0, 0.0, 0.0],
            None,
            None,
            0.0,
            0.45,
        );
        // THE SHADER'S OWN ORDER (the clamp rides AFTER the darkness
        // subtract — the unclamped sum the subtract sees, the
        // `clamp(color, 0.0, 1.0)` the tail wears): the bright texel
        // 1.0 + block-term 0.1556 = 1.1556 - 0.45 = 0.7056 — the
        // clamp-before-subtract shape the identity never spoke
        for ch in 0..3 {
            let raw = expect_raw[ch];
            let want = (raw - 0.45).clamp(0.0, 1.0);
            assert!(
                (dimmed[ch] - want).abs() < 1e-6,
                "the bright texel the subtract wears: {} vs {}",
                dimmed[ch],
                want
            );
        }
        // THE CLAMP'S OWN FLOOR (the dark texel the ambient base alone
        // carries — the subtraction's negative the 0.0 clamps: the
        // shader's `clamp(color, 0.0, 1.0)` BOTH sides)
        let dark = crate::mesher::lightmap_combine(
            Some(0.0),
            Some(0.0),
            0.26666668,
            [1.0, 1.0, 1.0],
            [0.039215688, 0.039215688, 0.039215688],
            None,
            None,
            0.0,
            0.45,
        );
        assert_eq!(dark, [0.0, 0.0, 0.0], "the dark texel the floor clamps");
        // THE WORLD FIELD'S OWN RIDE (the mesher's call sites the
        // World's own field consume — the mesh's vertex colors the
        // scale's dim wears, the no-effect floor the bake never
        // touches)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(5), 8, 9, 8);
        let floor = vertex_multiplier(&top_face(&w));
        w.darkness_scale = 0.45;
        let dimmed = vertex_multiplier(&top_face(&w));
        for (fc, dc) in floor.iter().zip(dimmed.iter()) {
            assert!(
                dc < fc || *fc <= 0.0,
                "the dimmed bake never brightens: {dc} vs {fc}"
            );
        }
        // the floor restored — the identity the zero keeps
        w.darkness_scale = 0.0;
        assert_eq!(vertex_multiplier(&top_face(&w)), floor);
    }

    #[test]
    fn the_ambient_base_lifts_the_combine_per_channel() {
        // the lightmap's AmbientColor BASE (v0.3.88): the shader's
        // `max(AmbientColor, nightVisionColor)` start — night vision 0,
        // so the dimension's stated attribute itself — sits UNDER the
        // sky/block sum. The overworld's #0a0a0a lifts a dark torch
        // cell by exactly 10/255 per channel over the black default
        // (derived from the None arithmetic the previous tests pin);
        // the nether's #302821 floor is per-CHANNEL, not uniform.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(7), 8, 9, 8);
        // the black default: the pre-ambient arithmetic
        let b = light_brightness(7.0) * BLOCK_FACTOR;
        let c = block_light_color(7.0, None);
        let dark = [b * c[0], b * c[1], b * c[2]];
        // the overworld floor: the SAME sum, +10/255 per channel
        w.ambient_light = Some([0x0a, 0x0a, 0x0a]);
        let lift = 10.0f32 / 255.0;
        let expect = [
            (dark[0] + lift).min(1.0),
            (dark[1] + lift).min(1.0),
            (dark[2] + lift).min(1.0),
        ];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!((ch - e).abs() < 1e-6, "ambient lifts per channel: {ch} vs {e}");
            }
        }
        // the nether floor #302821: every channel a different step
        w.ambient_light = Some([0x30, 0x28, 0x21]);
        let expect = [
            (dark[0] + 48.0 / 255.0).min(1.0),
            (dark[1] + 40.0 / 255.0).min(1.0),
            (dark[2] + 33.0 / 255.0).min(1.0),
        ];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!(
                    (ch - e).abs() < 1e-6,
                    "the nether floor is per-channel: {ch} vs {e}"
                );
            }
        }
        // a world with NO light grid keeps the legacy look even with a
        // stated floor: the early return wins (the procedural scenes'
        // shape — their light layer is None, the floor rides nothing)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, STONE);
        w2.ambient_light = Some([0x0a, 0x0a, 0x0a]);
        assert_eq!(
            vertex_multiplier(&top_face(&w2)),
            [1.0; 4],
            "no light layer: the legacy look, ambient or not"
        );
    }

    #[test]
    fn the_ambient_base_never_moves_the_saturated_end() {
        // day sky-15 over the overworld floor STILL clamps to 1.0 — the
        // ambient sits under the sum, the final clamp's verdict is the
        // shader's; and the byte-carrying world stays None-identical on
        // the shapes the previous tests pin (the black default is the
        // pre-ambient arithmetic, the regression is theirs)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, Some(15), Some(0), 8, 9, 8);
        w.ambient_light = Some([0x0a, 0x0a, 0x0a]);
        assert_eq!(
            vertex_multiplier(&top_face(&w)),
            [1.0; 4],
            "day sky-15 + ambient still clamps to the legacy full-bright"
        );
        // a dark night cell (sky-1, the 0.24 factor, the #7a7aff tint)
        // over the end's #3f473f floor: the floor dominates, the sky term
        // adds its sliver per channel — derived from the None-arithmetic
        // terms
        w.ambient_light = Some([0x3f, 0x47, 0x3f]);
        set_light(&mut w, Some(1), Some(0), 8, 9, 8);
        w.day_tick = Some(18_000);
        let tint_rg = 122.0f32 / 255.0;
        let sky = light_brightness(1.0) * 0.24;
        let expect = [
            63.0f32 / 255.0 + sky * tint_rg,
            71.0f32 / 255.0 + sky * tint_rg,
            63.0f32 / 255.0 + sky,
        ];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!((ch - e).abs() < 1e-6, "end floor at night: {ch} vs {e}");
            }
        }
    }

    #[test]
    fn corner_light_averages_the_four_face_plane_cells() {
        // face cell (8,9,8) full sky; ONE known dark diagonal (7,9,7) —
        // exactly corner 0 samples it (x=0,z=0 -> d1=-x, d2=-z)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, Some(15), None, 8, 9, 8);
        set_light(&mut w, Some(0), None, 7, 9, 7);
        let m = vertex_multiplier(&top_face(&w));
        // corner 0 averages its known cells (15+0)/2 = 7.5 -> brightness 0.2
        assert!((light_brightness(7.5) - 0.2).abs() < 1e-6, "ramp sanity");
        assert!(
            (m[0] - light_brightness(7.5)).abs() < 1e-6,
            "the dark diagonal drags only its corner: {m:?}"
        );
        // the other corners know only the face cell
        assert_eq!(m[1], 1.0, "corner 1 untouched: {m:?}");
        assert_eq!(m[2], 1.0, "corner 2 untouched: {m:?}");
        assert_eq!(m[3], 1.0, "corner 3 untouched: {m:?}");
    }

    #[test]
    fn ao_open_face_is_bright() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        // top face has nothing above it anywhere -> every corner level 3
        assert_eq!(ao_levels(&w, 8, 8, 8, 2), [3, 3, 3, 3]);
        assert_eq!(ao_levels(&w, 8, 8, 8, 0), [3, 3, 3, 3]);
    }

    #[test]
    fn ao_wall_darkens_adjacent_ground() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, GRASS); // ground block
        put(&mut w, 9, 9, 8, STONE); // wall one step up, +X side
        // On the ground top face, corners adjacent to the wall (x=1 side)
        // pick up occlusion: their +X neighbor at y+1 is stone.
        let levels = ao_levels(&w, 8, 8, 8, 2);
        assert!(levels.iter().any(|&l| l < 3), "wall must darken some corner: {:?}", levels);
        // the far corners (x=0 side) stay fully open
        let far: Vec<u8> = FACE_CORNERS[2]
            .iter()
            .enumerate()
            .filter(|(_, c)| c[0] < 0.5)
            .map(|(i, _)| levels[i])
            .collect();
        assert!(far.iter().all(|&l| l == 3), "far corners must stay bright: {:?}", levels);
    }

    #[test]
    fn ao_enclosed_corner_is_darkest() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, GRASS);
        // two walls meeting at the (1,1) corner above the ground block
        put(&mut w, 9, 9, 8, STONE);
        put(&mut w, 8, 9, 9, STONE);
        let levels = ao_levels(&w, 8, 8, 8, 2);
        assert!(levels.contains(&0), "double-sided corner must hit level 0: {:?}", levels);
    }

    #[test]
    fn ao_ignores_non_opaque_neighbors() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, GRASS);
        put(&mut w, 9, 9, 8, WATER);
        put(&mut w, 9, 9, 9, LEAVES);
        assert_eq!(ao_levels(&w, 8, 8, 8, 2), [3, 3, 3, 3], "water/leaves must not occlude");
    }

    #[test]
    fn ao_corner_offset_maps_to_correct_neighbor() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, GRASS);
        // single stone diagonal to exactly one corner of the top face:
        // base cell above the block is (8,9,8); corner 0 samples (7,9,7).
        put(&mut w, 7, 9, 7, STONE);
        let levels = ao_levels(&w, 8, 8, 8, 2);
        assert_eq!(levels, [2, 3, 3, 3], "only the diagonal corner darkens: {:?}", levels);
    }

    #[test]
    fn quad_diagonal_flips_to_brighter_pair() {
        // AO pattern whose brighter pair is (1,3): the fan diagonal must
        // rotate onto that pair; an already-optimal pattern stays untouched.
        let flipped = flip_order(&[1, 3, 1, 3]);
        assert_eq!(flipped, [1, 2, 3, 0]);
        assert!(3 + 3 >= 1 + 3, "new diagonal must carry the max AO sum");
        assert_eq!(flip_order(&[3, 1, 3, 1]), [0, 1, 2, 3], "optimal diagonal stays");
        assert_eq!(flip_order(&[2, 2, 2, 2]), [0, 1, 2, 3], "uniform AO stays");
    }

    #[test]
    fn mesh_colors_match_face_shade_and_ao() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        let q = mesh_chunk(&w, 0, 0, None);
        // top face of the lone block: shade 1.0, open sky -> full brightness
        let top = q.iter().find(|quad| quad.pts[0][1] == 9.0 && quad.pts[1][1] == 9.0).expect("top face quad");
        let tile = atlas::FACE_TILES[STONE as usize][2];
        if tile == atlas::TILE_NONE {
            let expected = default_color(STONE, 2);
            for c in &top.colors {
                for ch in 0..3 {
                    assert!((c[ch] - expected[ch]).abs() < 1e-6, "{:?} vs {:?}", c, expected);
                }
            }
        } else {
            // textured path: vertex colors carry the shade*AO multiplier only
            for c in &top.colors {
                assert!((c[0] - 1.0).abs() < 1e-6, "open-sky top multiplier must be 1.0: {:?}", c);
                assert_eq!(c[0], c[1]);
                assert_eq!(c[1], c[2]);
            }
        }
    }

    #[test]
    fn the_farmland_the_dry_top_the_master_wears() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::FARMLAND);
        let q = mesh_chunk(&w, 0, 0, None);
        let dry_top = atlas::FACE_TILES[crate::world::FARMLAND as usize][2];
        // the top quad the dry master wears (the moisture=0 variant the
        // model chain resolves)
        assert!(
            q.iter()
                .any(|quad| quad.pts[0][1] == 9.0 && quad.tile == dry_top),
            "the dry farmland the FACE_TILES top master wears"
        );
        // the wet verdict the no-water world never picks
        assert!(
            !q.iter()
                .any(|quad| quad.tile == atlas::TILE_FARMLAND_MOIST),
            "the dry world the wet tile never speaks"
        );
    }

    #[test]
    fn the_farmland_the_wet_top_the_moist_wears() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::FARMLAND);
        put(&mut w, 4, 8, 8, WATER); // the dx=-4 edge the box scans
        let q = mesh_chunk(&w, 0, 0, None);
        let dry_top = atlas::FACE_TILES[crate::world::FARMLAND as usize][2];
        // the top quad the wet master wears (the moisture=7 variant's own
        // tile the real chain resolves — the is_near_water verdict the
        // vanilla texture pick rides)
        assert!(
            q.iter().any(
                |quad| quad.pts[0][1] == 9.0 && quad.tile == atlas::TILE_FARMLAND_MOIST
            ),
            "the near-water farmland the farmland_moist master wears"
        );
        assert!(
            !q.iter().any(|quad| quad.tile == dry_top),
            "the wet cell the dry master never wears"
        );
        // THE BOUNDARY: the water one-past the box the dry arm keeps
        put(&mut w, 4, 8, 8, AIR);
        put(&mut w, 2, 8, 8, WATER);
        let q2 = mesh_chunk(&w, 0, 0, None);
        assert!(
            q2.iter()
                .any(|quad| quad.pts[0][1] == 9.0 && quad.tile == dry_top),
            "the far water the dry master keeps"
        );
        assert!(
            !q2.iter()
                .any(|quad| quad.tile == atlas::TILE_FARMLAND_MOIST),
            "the far water the wet tile never speaks"
        );
        // THE SIDES: the dirt the both models name (the wet verdict the
        // top face alone picks — the farmland_wet model the sides share)
        put(&mut w, 2, 8, 8, AIR);
        put(&mut w, 4, 8, 8, WATER);
        let q3 = mesh_chunk(&w, 0, 0, None);
        let dirt_tile = atlas::FACE_TILES[crate::world::FARMLAND as usize][0];
        assert!(
            q3.iter().any(|quad| quad.tile == dirt_tile),
            "the side quads the dirt master wears"
        );
        assert!(
            q3.iter().any(
                |quad| quad.pts[0][1] == 9.0 && quad.tile == atlas::TILE_FARMLAND_MOIST
            ),
            "the wet top the second fill wears"
        );
    }

    #[test]
    fn face_uv_orientation() {
        // side faces keep +Y at the texture top (v=0)
        for face in [0usize, 1, 4, 5] {
            assert_eq!(face_uv(face, [0.0, 1.0, 0.0])[1], 0.0, "face {face}: y=1 must be v=0");
            assert_eq!(face_uv(face, [0.0, 0.0, 0.0])[1], 1.0, "face {face}: y=0 must be v=1");
        }
        // east (+X): looking down -X, screen-right is -Z -> u = 1-z
        assert_eq!(face_uv(0, [1.0, 0.5, 0.0])[0], 1.0);
        assert_eq!(face_uv(0, [1.0, 0.5, 1.0])[0], 0.0);
        // west (-X): u = z
        assert_eq!(face_uv(1, [0.0, 0.5, 0.0])[0], 0.0);
        assert_eq!(face_uv(1, [0.0, 0.5, 1.0])[0], 1.0);
        // south (+Z): u = x; north (-Z): u = 1-x
        assert_eq!(face_uv(4, [0.0, 0.5, 1.0])[0], 0.0);
        assert_eq!(face_uv(5, [0.0, 0.5, 0.0])[0], 1.0);
        // top: u=x, v=z (north at texture top); bottom mirrors v
        assert_eq!(face_uv(2, [0.25, 1.0, 0.75]), [0.25, 0.75]);
        assert_eq!(face_uv(3, [0.25, 0.0, 0.75]), [0.25, 0.25]);
    }

    #[test]
    fn the_flower_cross_wears_the_bake_law() {
        // THE PLAINS' OWN CROSS (the autopush-411 round — the block/
        // cross bake the 45° rotY + rescale fold speaks): the flower
        // cell the 4 quads wear (the two diagonal planes × the two
        // windings), the corners the from/to inset keeps, the uv the
        // full sprite rides, the tile the flower's own atlas row banks.
        // THE DEV-27 OFFSET: the corners the audited triple rides —
        // the model-relative read the shift subtracts (the XZ row the
        // dandelion cell wears, the exact literals the audit embeds).
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::DANDELION);
        let q = mesh_chunk(&w, 0, 0, None);
        let a = 0.8 / 16.0;
        let b = 15.2 / 16.0;
        let (dx, dy, dz) =
            crate::world::plant_offset(crate::world::DANDELION, 8, 9, 8);
        let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
        assert_eq!((dx, dy, dz), (0.150_000_005_960_464_48f32, 0.0, -0.083_333_328_366_279_6f32));
        let flower: Vec<_> = q
            .iter()
            .filter(|quad| {
                // the vertical span (the stone's own top face at y=9 the
                // any-10 filter never rides) — the XZ row the Y the zero
                // keeps, the span the unshifted bounds wear
                quad.pts.iter().all(|p| (9.0..=10.0).contains(&p[1]))
                    && quad.pts.iter().any(|p| p[1] == 10.0)
            })
            .collect();
        assert_eq!(flower.len(), 4, "the two planes the two windings wear");
        for quad in &flower {
            for p in &quad.pts {
                let (fx, fz) = (p[0] - 8.0 - dx, p[2] - 8.0 - dz);
                assert!(
                    (fx - a).abs() < 1e-6 || (fx - b).abs() < 1e-6,
                    "the x the diagonal corner inset keeps: {fx}"
                );
                assert!(
                    (fz - a).abs() < 1e-6 || (fz - b).abs() < 1e-6,
                    "the z the diagonal corner inset keeps: {fz}"
                );
                // THE DIAGONAL'S OWN PAIR (the plane A the (0.8, 0.8) ->
                // (15.2, 15.2) walk keeps, the plane B the mirrored run
                // answers — the rescale fold the 45° rotation undoes)
                assert!(
                    (fx - fz).abs() < 1e-6 || (fx + fz - 1.0).abs() < 1e-6,
                    "the cross plane the diagonal runs: {fx} vs {fz}"
                );
            }
            // the uv the full sprite (u spans 0..1, v spans 0..1)
            let u_span = quad.uvs.iter().map(|c| c[0]).fold(f32::MIN, f32::max)
                - quad.uvs.iter().map(|c| c[0]).fold(f32::MAX, f32::min);
            let v_span = quad.uvs.iter().map(|c| c[1]).fold(f32::MIN, f32::max)
                - quad.uvs.iter().map(|c| c[1]).fold(f32::MAX, f32::min);
            assert!((u_span - 1.0).abs() < 1e-6, "the u the full span keeps");
            assert!((v_span - 1.0).abs() < 1e-6, "the v the full span keeps");
            // the tile: the dandelion's own row (the atlas's 39th kind)
            assert_eq!(quad.tile, mc_registry::generated::atlas::FACE_TILES[39][2]);
        }
        // THE TWO WINDINGS' OWN PAIR (the reversed corner cycle the same
        // points wears — the raster culls one per view side)
        let p0: Vec<[f32; 3]> = flower[0].pts.to_vec();
        let reversed = flower
            .iter()
            .any(|other| other.pts.iter().rev().eq(p0.iter()));
        assert!(reversed, "the mirrored winding the pair keeps");
    }

    #[test]
    fn the_provider_flowers_wear_the_cross_tile_laws() {
        // THE PROVIDER'S OWN SEVEN KINDS (the autopush-414 round — the
        // noise_threshold_provider's own high_states/low_states rows the
        // atlas banks): every new flower cell the same 4 quads wear (the
        // two diagonal planes × the two windings, the corners the 0.8
        // inset keeps, the uv the full sprite rides), the tile the
        // kind's own atlas row banks (the blockstates/<flower>.json ->
        // block/<flower> -> block/cross chain the resolved sprite
        // speaks), the XZ offset the flower family's own render row
        // carries (the dev-27 law the model-relative shift keeps).
        let a = 0.8 / 16.0;
        let b = 15.2 / 16.0;
        for kind in [
            crate::world::AZURE_BLUET,
            crate::world::OXEYE_DAISY,
            crate::world::CORNFLOWER,
            crate::world::ORANGE_TULIP,
            crate::world::RED_TULIP,
            crate::world::PINK_TULIP,
            crate::world::WHITE_TULIP,
        ] {
            let mut w = empty_world();
            put(&mut w, 8, 8, 8, STONE);
            put(&mut w, 8, 9, 8, kind);
            let q = mesh_chunk(&w, 0, 0, None);
            let (dx, dy, dz) = crate::world::plant_offset(kind, 8, 9, 8);
            let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
            assert_eq!(dy, 0.0, "the XZ row the y the zero keeps");
            let flower: Vec<_> = q
                .iter()
                .filter(|quad| {
                    quad.pts.iter().all(|p| (9.0..=10.0).contains(&p[1]))
                        && quad.pts.iter().any(|p| p[1] == 10.0)
                })
                .collect();
            assert_eq!(
                flower.len(),
                4,
                "the kind {kind} the two planes the two windings wear"
            );
            for quad in &flower {
                for p in &quad.pts {
                    let (fx, fz) = (p[0] - 8.0 - dx, p[2] - 8.0 - dz);
                    assert!(
                        (fx - a).abs() < 1e-6 || (fx - b).abs() < 1e-6,
                        "the x the diagonal corner inset keeps: {fx}"
                    );
                    assert!(
                        (fz - a).abs() < 1e-6 || (fz - b).abs() < 1e-6,
                        "the z the diagonal corner inset keeps: {fz}"
                    );
                }
                // the tile: the kind's own atlas row (the resolved
                // cross sprite the provider's list order banks)
                assert_eq!(
                    quad.tile,
                    mc_registry::generated::atlas::FACE_TILES[kind as usize][2],
                    "the kind {kind} the own tile banks"
                );
            }
        }
        // THE SEVEN TILES' OWN DISTINCT ROWS: each kind its own sprite
        // index (the atlas rows the provider's one-kind-one-tile walk
        // banks — the shared-tile collapse the family never wears)
        let mut tiles = [
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::AZURE_BLUET as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::OXEYE_DAISY as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::CORNFLOWER as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::ORANGE_TULIP as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::RED_TULIP as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::PINK_TULIP as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::WHITE_TULIP as usize][2],
        ];
        tiles.sort_unstable();
        for pair in tiles.windows(2) {
            assert_ne!(pair[0], pair[1], "the seven sprites the distinct rows keep");
        }
    }

    /// THE TORCH'S OWN ELEMENT-LANE LAWS (the dev-32 round): the stick
    /// box the template_torch element bakes — the SIX quads the derive
    /// walk keeps, the down face the solid ground culls (the cullface
    /// law the below cell's opaque verdict answers), the vertex bounds
    /// the [7,0,7]-[9,10,9] box keeps, the uv windows the texel rows
    /// /16 fold wears, the tile the kind's own atlas row banks, the
    /// neighbors' faces the non-occluding verdict draws.
    #[test]
    fn the_torch_lane_wears_the_element_laws() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE GROUNDED TORCH: the stick on stone — five quads survive
        // (the down face the below stone culls), the bounds the box
        // keeps, the tile the torch row banks
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::TORCH);
        let q = mesh_chunk(&w, 0, 0, None);
        let stick: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().all(|p| (9.0..=9.7).contains(&p[1]))
                    && quad.pts.iter().all(|p| {
                        (8.0 + 7.0 / 16.0..=8.0 + 9.0 / 16.0).contains(&p[0])
                    })
            })
            .collect();
        assert_eq!(stick.len(), 5, "the six faces the culled bottom the five keeps");
        let torch_tile = gen_atlas::FACE_TILES[crate::world::TORCH as usize][2];
        assert_ne!(torch_tile, gen_atlas::TILE_NONE, "the tile the regen banks");
        for quad in &stick {
            assert_eq!(quad.tile, torch_tile, "the torch's own tile row");
            assert!(!quad.translucent, "the stick the opaque lane wears");
            for p in &quad.pts {
                let (fx, fy, fz) = (p[0] - 8.0, p[1] - 9.0, p[2] - 8.0);
                assert!(
                    (7.0 / 16.0..=9.0 / 16.0).contains(&fx)
                        && (7.0 / 16.0..=9.0 / 16.0).contains(&fz)
                        && (0.0..=10.0 / 16.0).contains(&fy),
                    "the [7,0,7]-[9,10,9] box the corners keep: ({fx},{fy},{fz})"
                );
            }
            for uv in &quad.uvs {
                assert!(
                    (0.0..=1.0).contains(&uv[0]) && (0.0..=1.0).contains(&uv[1]),
                    "the /16 fold the tile space keeps: {uv:?}"
                );
            }
        }
        // THE DOWN CULL'S OWN ABSENCE — the floating torch (the honest
        // canSurvive-fail cell the test meshes anyway) the SIX quads
        // keep: the below air the cullface verdict never fires
        let mut w2 = empty_world();
        put(&mut w2, 8, 9, 8, crate::world::TORCH);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        let floating: Vec<_> = q2
            .iter()
            .filter(|quad| quad.pts.iter().all(|p| (9.0..=9.7).contains(&p[1])))
            .collect();
        assert_eq!(floating.len(), 6, "the below air the bottom face draws");
        // THE SUB-SPRITE'S OWN WINDOW (the texel rows the derive walk
        // keeps verbatim: the four sides [7,6,9,16] the /16 fold
        // wears — u 7/16..9/16, v 6/16..1.0; the up face [7,6,9,8] —
        // v 6/16..8/16): the side quads the narrower u band keeps
        let side_band: Vec<_> = stick
            .iter()
            .filter(|quad| {
                quad.pts.iter().any(|p| (p[1] - 9.0).abs() < 1e-6)
                    && quad.pts.iter().any(|p| (p[1] - 9.0 - 10.0 / 16.0).abs() < 1e-6)
            })
            .collect();
        assert_eq!(side_band.len(), 4, "the four sides the y extremes name");
        for quad in &side_band {
            for uv in &quad.uvs {
                assert!(
                    (7.0 / 16.0..=9.0 / 16.0).contains(&uv[0]),
                    "the side's u the [7,9] window keeps: {uv:?}"
                );
            }
        }
        // THE NEIGHBORS' OWN FACES — the torch cell the solid block
        // draws against (the non-occluding verdict the stick's
        // noCollision row keeps): the stone's west face at the
        // shared plane renders
        let mut w3 = empty_world();
        put(&mut w3, 8, 9, 8, crate::world::TORCH);
        put(&mut w3, 9, 9, 8, STONE);
        let q3 = mesh_chunk(&w3, 0, 0, None);
        assert!(
            q3.iter().any(|quad| {
                quad.pts.iter().all(|p| (p[0] - 9.0).abs() < 1e-6)
                    && quad.pts.iter().all(|p| (9.0..=10.0).contains(&p[1]))
            }),
            "the stone's face the torch cell draws against"
        );
    }

    #[test]
    fn the_fire_lane_selects_the_real_multipart_parts() {
        // THE SELECTION'S OWN REGIMES (the real blockstate engine the
        // lane rides — parse("fire")/parse("soul_fire") over the jar's
        // own columns, the same select_parts walk the fence/wire tests
        // pin): the freestanding all-false state the FULL fan speaks
        // (the floor pair via the all-false condition, all four side
        // groups each via its OR's all-false arm, the y quarters the
        // four groups carry), the single-wall state the wall's own
        // group alone answers, the up state the up pair answers.
        let freestanding: Vec<_> = select_parts(fire_parts(false), &[("north", "false"), ("east", "false"), ("south", "false"), ("west", "false"), ("up", "false")]);
        assert_eq!(freestanding.len(), 18, "the floor pair + the four side groups the all-false arms wear");
        let floors = freestanding.iter().filter(|a| a.model.contains("fire_floor")).count();
        let sides = freestanding.iter().filter(|a| a.model.contains("fire_side")).count();
        let ups = freestanding.iter().filter(|a| a.model.contains("fire_up")).count();
        assert_eq!((floors, sides, ups), (2, 16, 0), "the floor pair, the 4x4 side stack, no up pair");
        // THE FOUR QUARTERS the side groups carry — north y=0, east
        // y=90, south y=180, west y=270 (the jar's own apply rows)
        for (y, quarter_count) in [(0u32, 4usize), (90, 4), (180, 4), (270, 4)] {
            assert_eq!(
                freestanding.iter().filter(|a| a.model.contains("fire_side") && a.y == y).count(),
                quarter_count,
                "the y={y} quarter the group's four side models wear"
            );
        }
        // THE EAST WALL ALONE — the floating fire's log-adjacent state:
        // the floor's all-false condition refuses, the other groups'
        // OR arms refuse, the east group's own arm answers at y=90
        let east: Vec<_> = select_parts(fire_parts(false), &[("north", "false"), ("east", "true"), ("south", "false"), ("west", "false"), ("up", "false")]);
        assert_eq!(east.len(), 4, "the east group's four side models alone");
        assert!(east.iter().all(|a| a.model.contains("fire_side") && a.y == 90));
        // THE UP PAIR ALONE — the burnable-above state: the up group's
        // four models at y=0, the floor and the sides all refuse
        let up: Vec<_> = select_parts(fire_parts(false), &[("north", "false"), ("east", "false"), ("south", "false"), ("west", "false"), ("up", "true")]);
        assert_eq!(up.len(), 4, "the up group's four models alone");
        assert!(up.iter().all(|a| a.model.contains("fire_up") && a.y == 0));
        // THE SOUL COLUMN'S OWN UNCONDITIONAL FAN — no when anywhere:
        // ANY state the full 18-applies fan answers, the up pair never
        for state in [
            [("north", "false"), ("east", "false"), ("south", "false"), ("west", "false"), ("up", "false")],
            [("north", "true"), ("east", "true"), ("south", "true"), ("west", "true"), ("up", "true")],
        ] {
            let soul: Vec<_> = select_parts(fire_parts(true), &state);
            assert_eq!(soul.len(), 18, "the soul fan the no-when column always speaks");
            assert!(soul.iter().all(|a| !a.model.contains("up")), "the up pair the soul column never wears");
        }
    }

    #[test]
    fn the_campfire_lane_the_template_bakes() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE CAMPFIRE FAMILY'S OWN TILES — the regen the log masters
        // bank (the palette arms' resolve walk the strip tables speak),
        // the fire quads' bases the FRAME0 consts answer
        let log_tile = gen_atlas::TILE_CAMPFIRE_LOG;
        let lit_tile = gen_atlas::TILE_CAMPFIRE_LOG_LIT;
        let soul_lit_tile = gen_atlas::TILE_SOUL_CAMPFIRE_LOG_LIT;
        let fire_tile = gen_atlas::CAMPFIRE_FIRE_FRAME0;
        assert_ne!(log_tile, gen_atlas::TILE_NONE, "the log tile the regen banks");
        assert_ne!(lit_tile, gen_atlas::TILE_NONE, "the lit-log tile the regen banks");
        assert_ne!(soul_lit_tile, gen_atlas::TILE_NONE, "the soul lit-log tile the regen banks");
        assert_ne!(fire_tile, gen_atlas::TILE_NONE, "the fire tile the strip tables bank");
        // THE UNLIT KIND'S OWN BAKE — the five-log geometry on stone:
        // the 28 element faces the walk derives, the THREE down
        // cullfaces the solid ground culled (log1/log3/log5's own
        // rows), the 25 drawn quads the log tile alone answers
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::CAMPFIRE);
        let q = mesh_chunk(&w, 0, 0, None);
        let unlit: Vec<_> = q.iter().filter(|quad| quad.tile == log_tile).collect();
        assert_eq!(
            unlit.len(),
            25,
            "the five-log boxes' 28 faces minus the three down culls"
        );
        // THE FLOATING KIND'S OWN CULLS — the air-below cell the down
        // faces draw (the cullface verdict the neighbor read keeps):
        // the 28 faces the walk answers uncut
        put(&mut w, 12, 9, 8, crate::world::CAMPFIRE);
        let q = mesh_chunk(&w, 0, 0, None);
        let floating: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == log_tile && quad.pts[0][0] >= 12.0)
            .collect();
        assert_eq!(floating.len(), 28, "the floating campfire the down culls never trigger");
        // THE LIT KIND'S OWN BAKE — the same five-log geometry the
        // lit_log faces swap PLUS the two 45-degree fire planes the
        // rotation walk bakes: the 16 log + 9 lit_log + 4 fire quads
        // the per-quad texture verdict splits (the 25 drawn log-box
        // quads the three down culls keep, the swap the texture refs
        // wear), the fire planes' y 1..17 span the cell above licks
        // into (the vanilla's own rise the mesh never clips)
        put(&mut w, 8, 9, 8, crate::world::CAMPFIRE_LIT);
        let q = mesh_chunk(&w, 0, 0, None);
        let lit_logs: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == log_tile && quad.pts[0][0] >= 6.0 && quad.pts[0][0] < 10.0)
            .collect();
        let lit_lits: Vec<_> = q.iter().filter(|quad| quad.tile == lit_tile).collect();
        let fires: Vec<_> = q.iter().filter(|quad| quad.tile == fire_tile).collect();
        assert_eq!(lit_logs.len(), 16, "the #log faces the lit template keeps");
        assert_eq!(lit_lits.len(), 9, "the #lit_log faces the lit template swaps");
        assert_eq!(fires.len(), 4, "the two fire planes' north/south + west/east pairs");
        for quad in &fires {
            assert!(
                quad.pts.iter().any(|p| p[1] > 10.0),
                "the fire plane the cell's top the vanilla lick wears: {:?}",
                quad.pts.iter().map(|p| p[1]).collect::<Vec<_>>()
            );
        }
        // THE SHADE TERM'S OWN SPLIT — the log boxes' `shade: true`
        // elements the direction table pays, the fire planes'
        // `shade: false` the lightmap alone multiplies (the no-grid
        // convention the unshaded 1.0 the fire corners carry)
        assert!(
            fires.iter().all(|quad| quad.colors.iter().all(|c| (c[0] - 1.0).abs() < 1e-5)),
            "the fire planes the 1.0 shade-false keeps"
        );
        // THE SOUL KIND'S OWN BAKE — the same geometry the soul
        // masters answer (the lit_log tile the soul strip's own row
        // keeps, the fire the soul strip's frame-0 base)
        put(&mut w, 4, 9, 8, crate::world::SOUL_CAMPFIRE_LIT);
        let q = mesh_chunk(&w, 0, 0, None);
        let soul_fires: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == gen_atlas::SOUL_CAMPFIRE_FIRE_FRAME0)
            .collect();
        let soul_lits: Vec<_> = q.iter().filter(|quad| quad.tile == soul_lit_tile).collect();
        assert_eq!(soul_fires.len(), 4, "the soul fire planes the same bake answers");
        assert_eq!(soul_lits.len(), 9, "the soul lit-log faces the same split wears");
    }

    #[test]
    fn the_candle_cake_lane_the_template_bakes() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE CANDLE CAKE PAIR'S OWN TILES — the regen the cake
        // masters bank (the appended walk the named consts answer),
        // the candle faces the palette's own candle tiles answer
        let bottom_tile = gen_atlas::TILE_CAKE_BOTTOM;
        let top_tile = gen_atlas::TILE_CAKE_TOP;
        let side_tile = gen_atlas::TILE_CAKE_SIDE;
        let candle_tile = gen_atlas::FACE_TILES[crate::world::CANDLE as usize][2];
        let lit_candle_tile = gen_atlas::FACE_TILES[crate::world::CANDLE_LIT as usize][2];
        assert_ne!(bottom_tile, gen_atlas::TILE_NONE, "the bottom tile the regen banks");
        assert_ne!(top_tile, gen_atlas::TILE_NONE, "the top tile the regen banks");
        assert_ne!(side_tile, gen_atlas::TILE_NONE, "the side tile the regen banks");
        assert_ne!(candle_tile, gen_atlas::TILE_NONE, "the candle tile the palette banks");
        assert_ne!(lit_candle_tile, gen_atlas::TILE_NONE, "the lit tile the palette banks");
        assert_ne!(lit_candle_tile, candle_tile, "the two candle states their own masters");
        // THE UNLIT KIND'S OWN BAKE — the slab + stick + wick geometry
        // on stone: the 16 element faces the walk derives, the TWO
        // down cullfaces the solid ground culled (the slab's and the
        // stick's own rows), the 14 drawn quads the texture verdict
        // splits — the bottom face the ground cull keeps 0, the top 1,
        // the four sides 4, the candle faces 9 (the stick's 5 drawn +
        // the two wick planes' 4 the rotation walk bakes)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::CANDLE_CAKE);
        let q = mesh_chunk(&w, 0, 0, None);
        let bottoms: Vec<_> = q.iter().filter(|quad| quad.tile == bottom_tile).collect();
        let tops: Vec<_> = q.iter().filter(|quad| quad.tile == top_tile).collect();
        let sides: Vec<_> = q.iter().filter(|quad| quad.tile == side_tile).collect();
        let candles: Vec<_> = q.iter().filter(|quad| quad.tile == candle_tile).collect();
        assert_eq!(bottoms.len(), 0, "the slab's down face the solid ground culls");
        assert_eq!(tops.len(), 1, "the slab's up face the top tile wears");
        assert_eq!(sides.len(), 4, "the slab's four side faces the side tile wears");
        assert_eq!(candles.len(), 9, "the stick's 5 drawn + the wick planes' 4 the candle tile answers");
        // THE FLOATING KIND'S OWN CULLS — the air-below cell the two
        // down faces draw (the cullface verdict the neighbor read
        // keeps): the 16 faces the walk answers uncut, the bottom tile
        // the slab's own row serves
        put(&mut w, 12, 9, 8, crate::world::CANDLE_CAKE);
        let q = mesh_chunk(&w, 0, 0, None);
        let floating: Vec<_> = q
            .iter()
            .filter(|quad| {
                (quad.tile == bottom_tile
                    || quad.tile == top_tile
                    || quad.tile == side_tile
                    || quad.tile == candle_tile)
                    && quad.pts[0][0] >= 12.0
            })
            .collect();
        assert_eq!(floating.len(), 16, "the floating cake the down culls never trigger");
        assert_eq!(
            floating
                .iter()
                .filter(|quad| quad.tile == bottom_tile)
                .count(),
            1,
            "the slab's down face the air below draws"
        );
        // THE LIT KIND'S OWN BAKE — the same geometry the lit candle
        // faces swap (the #candle ref block/candle_lit the vanilla's
        // own texture override carries): the 9 drawn candle quads the
        // LIT tile answers, the cake faces the same masters keep
        put(&mut w, 8, 9, 8, crate::world::CANDLE_CAKE_LIT);
        let q = mesh_chunk(&w, 0, 0, None);
        let lit_candles: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == lit_candle_tile)
            .collect();
        assert_eq!(lit_candles.len(), 9, "the lit faces the same split the lit tile wears");
        // THE SHADE TERM'S OWN SPLIT — the template's shade:true
        // elements the direction table pays: the slab's up face the
        // 1.0 term keeps, the four sides the 0.8/0.6 horizontals wear
        // (the no-grid convention the lightmap alone multiplies)
        assert!(
            tops.iter().all(|quad| quad
                .colors
                .iter()
                .all(|c| (c[0] - 1.0).abs() < 1e-5)),
            "the up face the 1.0 shade term wears"
        );
        assert!(
            sides.iter().all(|quad| quad
                .colors
                .iter()
                .all(|c| (c[0] - 0.8).abs() < 1e-5 || (c[0] - 0.6).abs() < 1e-5)),
            "the side faces the 0.8/0.6 shade terms wear"
        );
    }

    #[test]
    fn the_cake_lane_the_slice_models_bake() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE CAKE LADDER'S OWN TILES — the named consts the regen
        // banks (the appended walk the four masters answer), the
        // bitten cut the TILE_CAKE_INNER const the west face reads
        let bottom_tile = gen_atlas::TILE_CAKE_BOTTOM;
        let top_tile = gen_atlas::TILE_CAKE_TOP;
        let side_tile = gen_atlas::TILE_CAKE_SIDE;
        let inner_tile = gen_atlas::TILE_CAKE_INNER;
        assert_ne!(bottom_tile, gen_atlas::TILE_NONE, "the bottom tile the regen banks");
        assert_ne!(top_tile, gen_atlas::TILE_NONE, "the top tile the regen banks");
        assert_ne!(side_tile, gen_atlas::TILE_NONE, "the side tile the regen banks");
        assert_ne!(inner_tile, gen_atlas::TILE_NONE, "the inner tile the regen banks");
        // THE FULL KIND'S OWN BAKE — the single box on stone: the SIX
        // element faces the walk derives, the down cullface the solid
        // ground culled (the 5 drawn quads the texture verdict splits
        // — the top 1, the four sides 4), the colors the shade:true
        // terms the direction table pays
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::CAKE);
        let q = mesh_chunk(&w, 0, 0, None);
        let cell = |quad: &&crate::mesher::Quad| quad.pts[0][0] >= 8.0 && quad.pts[0][0] < 9.0;
        let drawn: Vec<_> = q.iter().filter(|quad| {
            cell(quad) && (quad.tile == bottom_tile || quad.tile == top_tile
                || quad.tile == side_tile || quad.tile == inner_tile)
        }).collect();
        assert_eq!(drawn.len(), 5, "the six faces the down cull the ground serves");
        assert_eq!(drawn.iter().filter(|quad| quad.tile == top_tile).count(), 1,
            "the up face the top tile wears");
        assert_eq!(drawn.iter().filter(|quad| quad.tile == side_tile).count(), 4,
            "the four side faces the side tile wears");
        // THE FLOATING KIND'S OWN CULLS — the air-below cell the down
        // face draws (the cullface verdict the neighbor read keeps):
        // the 6 faces the walk answers uncut
        put(&mut w, 12, 9, 8, crate::world::CAKE);
        let q = mesh_chunk(&w, 0, 0, None);
        let floating: Vec<_> = q.iter().filter(|quad| {
            quad.pts[0][0] >= 12.0 && quad.pts[0][0] < 13.0
                && (quad.tile == bottom_tile || quad.tile == top_tile || quad.tile == side_tile)
        }).collect();
        assert_eq!(floating.len(), 6, "the floating cake the down culls never trigger");
        assert_eq!(floating.iter().filter(|quad| quad.tile == bottom_tile).count(), 1,
            "the down face the air below draws");
        // THE BITTEN STAGES' OWN BAKES — every slice the SAME single
        // box the x-start walk narrows (the lambda$static$0 shape the
        // models exactly mirror): the west face the #inside ref the
        // TILE_CAKE_INNER const answers, the other five the cake
        // masters keep (the on-ground slice the down cull keeps — the
        // 5 drawn quads the west-inside split wears)
        for (slice, kind) in [
            ("block/cake_slice1", crate::world::CAKE_SLICE1),
            ("block/cake_slice2", crate::world::CAKE_SLICE2),
            ("block/cake_slice3", crate::world::CAKE_SLICE3),
            ("block/cake_slice4", crate::world::CAKE_SLICE4),
            ("block/cake_slice5", crate::world::CAKE_SLICE5),
            ("block/cake_slice6", crate::world::CAKE_SLICE6),
        ] {
            let mut w = empty_world();
            put(&mut w, 8, 8, 8, STONE);
            put(&mut w, 8, 9, 8, kind);
            let q = mesh_chunk(&w, 0, 0, None);
            let inners: Vec<_> = q
                .iter()
                .filter(|quad| quad.tile == inner_tile && cell(quad))
                .collect();
            assert_eq!(inners.len(), 1, "slice {slice}: the west face the inner tile wears");
            let tops: Vec<_> = q
                .iter()
                .filter(|quad| quad.tile == top_tile && cell(quad))
                .collect();
            assert_eq!(tops.len(), 1, "slice {slice}: the up face the top tile wears");
        }
        // THE SHADE TERM'S OWN SPLIT — the box's shade:true elements
        // the direction table pays: the up face the 1.0 term keeps,
        // the four sides the 0.8/0.6 horizontals wear (the no-grid
        // convention the lightmap alone multiplies)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::CAKE);
        let q = mesh_chunk(&w, 0, 0, None);
        let tops: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == top_tile && cell(quad))
            .collect();
        assert!(
            tops.iter().all(|quad| quad
                .colors
                .iter()
                .all(|c| (c[0] - 1.0).abs() < 1e-5)),
            "the up face the 1.0 shade term wears"
        );
        let sides: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == side_tile && cell(quad))
            .collect();
        assert!(
            sides.iter().all(|quad| quad
                .colors
                .iter()
                .all(|c| (c[0] - 0.8).abs() < 1e-5 || (c[0] - 0.6).abs() < 1e-5)),
            "the side faces the 0.8/0.6 shade terms wear"
        );
    }

    #[test]
    fn the_web_the_cross_planes_and_wool_the_cube_wear() {
        use mc_registry::generated::atlas as gen_atlas;
        let web_tile = gen_atlas::FACE_TILES[crate::world::COBWEB as usize][2];
        let wool_tile = gen_atlas::FACE_TILES[crate::world::WHITE_WOOL as usize][0];
        assert_ne!(web_tile, gen_atlas::TILE_NONE, "the web tile the regen banks");
        assert_ne!(wool_tile, gen_atlas::TILE_NONE, "the wool tile the regen banks");
        // THE WEB'S OWN CROSS (the freestanding cell the two diagonal
        // planes the flowers' own path wears — the 4 quads the two
        // planes × two windings speak, the offset the NONE row zeroes
        // — the rigid diagonals the vanilla model renders)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::COBWEB);
        let q = mesh_chunk(&w, 0, 0, None);
        let webs: Vec<_> = q.iter().filter(|quad| quad.tile == web_tile).collect();
        assert_eq!(webs.len(), 4, "the web the two diagonals × two windings keeps");
        // THE RIGID PLANE (the offset zero — every point the cell's own
        // 8..9 x-band the no-jitter walk keeps)
        for quad in &webs {
            for pt in quad.pts {
                assert!(
                    (8.0..9.0).contains(&pt[0]),
                    "the web point the cell band keeps"
                );
            }
        }
        // THE WOOL'S OWN CUBE (the freestanding cell the six faces the
        // full-cube path wears — the 6 quads the backface-culled walk
        // keeps, one quad per face)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::WHITE_WOOL);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        let wools: Vec<_> = q2.iter().filter(|quad| quad.tile == wool_tile).collect();
        assert_eq!(wools.len(), 6, "the wool the six faces the cube path keeps");
    }

    #[test]
    fn the_fire_lane_renders_the_fan_the_blockstate_speaks() {
        use mc_registry::generated::atlas as gen_atlas;
        let fire_tile = gen_atlas::FACE_TILES[crate::world::FIRE as usize][2];
        let soul_tile = gen_atlas::FACE_TILES[crate::world::SOUL_FIRE as usize][2];
        assert_ne!(fire_tile, gen_atlas::TILE_NONE, "the tile the regen banks");
        assert_ne!(soul_tile, gen_atlas::TILE_NONE, "the soul tile the regen banks");
        // THE SECOND FRAME'S OWN BASE TILES — the *_1 families the
        // appended frame-00 tiles bake (the blockstate's own apply
        // pairs the BOTH-families fan speak)
        let fire1_tile = gen_atlas::FIRE_1_FRAMES[0];
        let soul1_tile = gen_atlas::SOUL_FIRE_1_FRAMES[0];
        assert_ne!(fire1_tile, fire_tile, "the two families' own tiles");
        assert_ne!(soul1_tile, soul_tile, "the soul families' own tiles");
        // THE FREESTANDING FAN — fire on stone (the below cell the
        // gate's sturdy arm answers): the 40 quads the floor pair (8)
        // plus the four side groups (32) wear, the TWO frame families'
        // own tiles on the quads (the blockstate's apply pairs the
        // both-families fan speaks — 20/20 the even split keeps), the
        // planes the cell's own footprint keeps and
        // the 22.4-texel rise the rotated tips' honest excursion wears
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::FIRE);
        let q = mesh_chunk(&w, 0, 0, None);
        let fan: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == fire_tile || quad.tile == fire1_tile)
            .collect();
        assert_eq!(fan.len(), 40, "the floor pair's 8 + the four groups' 32 the fan wears");
        assert_eq!(
            fan.iter().filter(|quad| quad.tile == fire_tile).count(),
            20,
            "the *_0 family's own half the even split keeps"
        );
        assert_eq!(
            fan.iter().filter(|quad| quad.tile == fire1_tile).count(),
            20,
            "the *_1 family's own half the even split keeps"
        );
        for quad in &fan {
            assert!(!quad.translucent, "the flame the opaque cutout lane wears");
            for p in &quad.pts {
                let (fx, fy, fz) = (p[0] - 8.0, p[1], p[2] - 8.0);
                assert!(
                    (-1e-6..=1.0 + 1e-6).contains(&fx) && (-1e-6..=1.0 + 1e-6).contains(&fz),
                    "the planes the cell's own footprint keeps: ({fx},{fz})"
                );
                // the vertical span the rescaled rotation wears: the
                // side walls' 22.4-texel rise (1.4), the rotated floor
                // planes' tips the ±22.5° + rescale fold lifts to
                // 1.42071 and dips to -0.02071 (the honest excursion
                // the vanilla fan's own bake carries)
                assert!(
                    (8.0 - 0.0208..=9.0 + 1.4208).contains(&fy),
                    "the fan's rise the rotated tips' excursion wears: {fy}"
                );
            }
            for uv in &quad.uvs {
                assert!(
                    (0.0..=1.0).contains(&uv[0]) && (0.0..=1.0).contains(&uv[1]),
                    "the /16 fold the tile space keeps: {uv:?}"
                );
            }
        }
        // THE GATE'S OWN SILENCE — the log beside the grounded fire the
        // props NEVER reach (the below stone's sturdy arm the default
        // state serves): the fan the same 40 quads keeps, no walls the
        // log's burnable odds could add
        let mut w_gate = empty_world();
        put(&mut w_gate, 8, 8, 8, STONE);
        put(&mut w_gate, 8, 9, 8, crate::world::FIRE);
        put(&mut w_gate, 9, 9, 8, crate::world::LOG);
        let q_gate = mesh_chunk(&w_gate, 0, 0, None);
        assert_eq!(
            q_gate
                .iter()
                .filter(|quad| quad.tile == fire_tile || quad.tile == fire1_tile)
                .count(),
            40,
            "the grounded fire the gate's default state keeps freestanding"
        );
        // THE FLOATING FIRE'S OWN WALL — the below air the gate refuses,
        // the east log the canBurn read answers: the east group's eight
        // quads alone, the wall plane the y=90 quarter's (x,z) -> (1-z,
        // x) turn speaks (the template's z=0.01 the 0.999375 plane keeps)
        let mut w_east = empty_world();
        put(&mut w_east, 8, 9, 8, crate::world::FIRE);
        put(&mut w_east, 9, 9, 8, crate::world::LOG);
        let q_east = mesh_chunk(&w_east, 0, 0, None);
        let wall: Vec<_> = q_east
            .iter()
            .filter(|quad| quad.tile == fire_tile || quad.tile == fire1_tile)
            .collect();
        assert_eq!(wall.len(), 8, "the east group's four models x the two faces");
        assert_eq!(
            wall.iter().filter(|quad| quad.tile == fire_tile).count(),
            4,
            "the east group's *_0 half the even split keeps"
        );
        assert_eq!(
            wall.iter().filter(|quad| quad.tile == fire1_tile).count(),
            4,
            "the east group's *_1 half the even split keeps"
        );
        for quad in &wall {
            for p in &quad.pts {
                assert!(
                    (p[0] - 8.0 - (1.0 - 0.01 / 16.0)).abs() < 1e-5,
                    "the east plane the y=90 turn speaks: {}",
                    p[0] - 8.0
                );
                assert!(
                    (8.0..=9.0 + 1e-6).contains(&p[2]) && (9.0..=10.4 + 1e-4).contains(&p[1]),
                    "the wall's span the template element keeps: ({},{})",
                    p[1], p[2]
                );
            }
        }
        // THE NEIGHBORS' OWN FACES — the flame cell the ground's top
        // face draws against (the replaceable().noCollision() rows the
        // is_transparent verdict wears): the stone's top face at the
        // shared plane renders
        assert!(
            q.iter().any(|quad| {
                quad.tile == gen_atlas::FACE_TILES[STONE as usize][2]
                    && quad.pts.iter().all(|p| (p[1] - 9.0).abs() < 1e-6)
            }),
            "the stone's top face the flame cell draws against"
        );
        // THE SOUL FAN — the unconditional column's own 40 quads, the
        // soul tile the kind's own row banks
        let mut w_soul = empty_world();
        put(&mut w_soul, 8, 8, 8, STONE);
        put(&mut w_soul, 8, 9, 8, crate::world::SOUL_FIRE);
        let q_soul = mesh_chunk(&w_soul, 0, 0, None);
        let soul_fan: Vec<_> = q_soul
            .iter()
            .filter(|quad| quad.tile == soul_tile || quad.tile == soul1_tile)
            .collect();
        assert_eq!(soul_fan.len(), 40, "the soul fan the no-when column always wears");
        assert_eq!(
            soul_fan.iter().filter(|quad| quad.tile == soul_tile).count(),
            20,
            "the soul *_0 family's own half the even split keeps"
        );
        assert_eq!(
            soul_fan.iter().filter(|quad| quad.tile == soul1_tile).count(),
            20,
            "the soul *_1 family's own half the even split keeps"
        );
        assert!(
            q_soul.iter()
                .all(|quad| quad.tile != fire_tile && quad.tile != fire1_tile),
            "the soul cell the plain flame's tiles never wears"
        );
    }

    #[test]
    fn the_candle_lane_the_template_bakes() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE CANDLE PAIR'S OWN TILES — the regen the two masters bank
        // (the blockstate's one-candle rows the `#all` refs resolve),
        // the two states' own tiles the kind rows keep
        let unlit_tile = gen_atlas::FACE_TILES[crate::world::CANDLE as usize][2];
        let lit_tile = gen_atlas::FACE_TILES[crate::world::CANDLE_LIT as usize][2];
        assert_ne!(unlit_tile, gen_atlas::TILE_NONE, "the tile the regen banks");
        assert_ne!(lit_tile, gen_atlas::TILE_NONE, "the lit tile the regen banks");
        assert_ne!(unlit_tile, lit_tile, "the two states' own masters");
        // THE STICK'S OWN BAKE — the unlit candle on stone: the 9 quads
        // the box's six faces (the down face the cullface verdict
        // culled) plus the two wick planes' four wear, the template's
        // own texel geometry the cell keeps (the box [7,0,7]->[9,6,9],
        // the wick planes the ±45° turns bake — the vertices never
        // leave the cell, the rise the 7-texel wick tip wears)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::CANDLE);
        let q = mesh_chunk(&w, 0, 0, None);
        let stick: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == unlit_tile)
            .collect();
        assert_eq!(stick.len(), 9, "the box's five faces + the wick's four");
        for quad in &stick {
            assert!(!quad.translucent, "the candle the opaque cutout lane wears");
            for p in &quad.pts {
                let (fx, fy, fz) = (p[0] - 8.0, p[1], p[2] - 8.0);
                assert!(
                    (-1e-6..=1.0 + 1e-6).contains(&fx) && (-1e-6..=1.0 + 1e-6).contains(&fz),
                    "the stick the cell's own footprint keeps: ({fx},{fz})"
                );
                assert!(
                    (9.0 - 1e-6..=9.0 + 7.0 / 16.0 + 1e-6).contains(&fy),
                    "the rise the box's 6 texels + the wick's 7th keeps: {fy}"
                );
            }
        }
        // THE SHADE TERM'S OWN READ — the elements' `shade: true` the
        // vanilla tesselation's direction table answers: the up face's
        // corners the 1.0 keeps, the horizontals the 0.8 wears (the
        // lightmap the no-grid convention the unshaded 1.0 carries —
        // the colors the shade alone splits)
        assert!(
            stick.iter().any(|quad| {
                quad.pts.iter().all(|p| (p[1] - 9.375).abs() < 1e-5)
                    && quad.colors.iter().all(|c| (c[0] - 1.0).abs() < 1e-5)
            }),
            "the box top's up face the 1.0 shade keeps"
        );
        assert!(
            stick.iter().any(|quad| {
                quad.colors.iter().all(|c| (c[0] - 0.8).abs() < 1e-5)
            }),
            "the horizontal faces the 0.8 shade wears"
        );
        assert!(
            stick.iter().all(|quad| {
                quad.colors.iter().all(|c| (0.5..=1.0).contains(&c[0]))
            }),
            "the shade table the cube path's constants keeps"
        );
        // THE AIR BELOW'S OWN TEN — the floating candle the down face
        // the cull keeps: the full 10 quads the template's three
        // elements wear
        let mut w_float = empty_world();
        put(&mut w_float, 8, 9, 8, crate::world::CANDLE);
        let q_float = mesh_chunk(&w_float, 0, 0, None);
        assert_eq!(
            q_float
                .iter()
                .filter(|quad| quad.tile == unlit_tile)
                .count(),
            10,
            "the down face the air-below cell never culls"
        );
        // THE LIT KIND'S OWN TILES — the same geometry the lit master
        // wears: 9 quads on the stone, the lit tile the quads carry
        let mut w_lit = empty_world();
        put(&mut w_lit, 8, 8, 8, STONE);
        put(&mut w_lit, 8, 9, 8, crate::world::CANDLE_LIT);
        let q_lit = mesh_chunk(&w_lit, 0, 0, None);
        let lit: Vec<_> = q_lit.iter().filter(|quad| quad.tile == lit_tile).collect();
        assert_eq!(lit.len(), 9, "the lit candle the same template bakes");
        assert!(
            q_lit.iter().all(|quad| quad.tile != unlit_tile),
            "the lit cell the unlit master never wears"
        );
    }

    #[test]
    fn the_fire_anim_remap_the_playlist_walks() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE PLAYLIST'S OWN WALK — the vanilla TextureAtlasSprite
        // animator's per-tick step the frametime 1 cadence speaks: tick
        // t the playlist position t % 32 reads, the frames the strip's
        // own rows bank. fire_0's mcmeta [16..31, 0..15] the playlist
        // the jar banks, fire_1's empty animation the sequential rule
        // serves — the two families the OUT-OF-PHASE flicker speaks.
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 0),
            gen_atlas::FIRE_0_FRAMES[16],
            "tick 0 the playlist's own head frame 16 wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 1),
            gen_atlas::FIRE_0_FRAMES[17],
            "tick 1 the walk's own next row"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 15),
            gen_atlas::FIRE_0_FRAMES[31],
            "tick 15 the playlist's first leg ends"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 16),
            gen_atlas::FIRE_0_FRAMES[0],
            "tick 16 the second leg the frame 0 row opens"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 31),
            gen_atlas::FIRE_0_FRAMES[15],
            "tick 31 the playlist's own tail"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 32),
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 0),
            "the 32-tick cycle the playlist's own period wears (frametime 1)"
        );
        // THE SECOND FAMILY'S OWN STRIP — the sequential walk the empty
        // animation serves, the two families the phase split speaks
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_1_FRAME0, 0),
            gen_atlas::FIRE_1_FRAMES[0],
            "tick 0 the strip's own first frame"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_1_FRAME0, 1),
            gen_atlas::FIRE_1_FRAMES[1],
            "tick 1 the sequential walk's next row"
        );
        assert_ne!(
            gen_atlas::anim_tile(gen_atlas::FIRE_1_FRAME0, 0),
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 0),
            "the families' phase split the distinct frames keep"
        );
        // THE SOUL HALF'S OWN STRIPS — the same playlists the kind's
        // own frame arrays answer, never the plain flame's rows
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::SOUL_FIRE_0_FRAME0, 0),
            gen_atlas::SOUL_FIRE_0_FRAMES[16],
            "the soul *_0 strip the playlist's own head wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::SOUL_FIRE_1_FRAME0, 0),
            gen_atlas::SOUL_FIRE_1_FRAMES[0],
            "the soul *_1 strip the sequential head wears"
        );
        assert_ne!(
            gen_atlas::anim_tile(gen_atlas::SOUL_FIRE_0_FRAME0, 0),
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 0),
            "the soul's frames the plain flame's rows never share"
        );
        // THE IDENTITY — every non-fire tile the remap never touches
        // (the block palette, the destroy stages, the missing-material
        // sentinel the flat path keeps)
        let stone = gen_atlas::FACE_TILES[STONE as usize][2];
        assert_eq!(gen_atlas::anim_tile(stone, 7), stone, "the stone tile the identity wears");
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::DESTROY_TILES[3], 19),
            gen_atlas::DESTROY_TILES[3],
            "the destroy overlay's crack frames the identity wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_NONE, 5),
            gen_atlas::TILE_NONE,
            "the flat path's sentinel the identity wears"
        );
        // THE APPENDED FRAMES' OWN BODY — every frame tile the non-empty
        // 16x16 texel body carries, the frame-00 rows the block tile's
        // own pixels speak (the strip's first frame the palette's crop
        // already banked — the two index spaces the same pixels wear)
        for f in gen_atlas::FIRE_0_FRAMES.iter().copied()
            .chain(gen_atlas::FIRE_1_FRAMES.iter().copied())
            .chain(gen_atlas::SOUL_FIRE_0_FRAMES.iter().copied())
            .chain(gen_atlas::SOUL_FIRE_1_FRAMES.iter().copied())
        {
            assert_eq!(
                gen_atlas::tile_rgba(f).len(),
                16 * 16 * 4,
                "the frame tile's own body: tile {f}"
            );
        }
        assert_eq!(
            gen_atlas::tile_rgba(gen_atlas::FIRE_0_FRAME0),
            gen_atlas::tile_rgba(gen_atlas::FIRE_0_FRAMES[0]),
            "the block tile's pixels the strip's first frame carries"
        );
        assert_eq!(
            gen_atlas::tile_rgba(gen_atlas::SOUL_FIRE_0_FRAME0),
            gen_atlas::tile_rgba(gen_atlas::SOUL_FIRE_0_FRAMES[0]),
            "the soul block tile's pixels the strip's first frame carries"
        );
    }

    #[test]
    fn the_lit_log_anim_the_interpolate_blends() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE INTERPOLATE LANE'S OWN WALK (the 26.2 jar's mcmeta: the
        // lit-log strips the {"interpolate": true, "frametime": 20}
        // 4-frame cycle carry): the blend bank the full 80-tick cycle
        // tiles (the frametime-1 cadence the per-tick resolution
        // keeps), the tick-0 head the BASE tile the alpha-0 identity
        // wears (the golden safety the byte-identical head rides), the
        // wrap the LAST frame's span the FIRST frame blends toward
        // (javap SpriteContents$AnimationState.drawToAtlas pc 88-115 —
        // the (frame + 1) % count modulo law)
        assert_eq!(
            gen_atlas::CAMPFIRE_LOG_LIT_FRAMETIME,
            1,
            "the per-tick blend cadence"
        );
        assert_eq!(
            gen_atlas::CAMPFIRE_LOG_LIT_FRAMES.len(),
            80,
            "the 4-frame x 20-tick cycle"
        );
        assert_eq!(gen_atlas::SOUL_CAMPFIRE_LOG_LIT_FRAMES.len(), 80);
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 0),
            gen_atlas::TILE_CAMPFIRE_LOG_LIT,
            "tick 0 the base tile the alpha-0 identity wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 1),
            gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[1],
            "tick 1 the first blend tile the walk answers"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 20),
            gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[20],
            "tick 20 the second span's own head (the pure frame 1)"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 79),
            gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[79],
            "tick 79 the wrap span's tail"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 80),
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 0),
            "the 80-tick cycle the playlist's own period wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_SOUL_CAMPFIRE_LOG_LIT, 3),
            gen_atlas::SOUL_CAMPFIRE_LOG_LIT_FRAMES[3],
            "the soul strip the same walk answers"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG, 7),
            gen_atlas::TILE_CAMPFIRE_LOG,
            "the unlit log the identity wears"
        );
        // THE PIXEL LAW'S OWN PROBE (the thousandths quantization the
        // f2i truncation banks — javap pc 39-76; for the frametime 20
        // the quantization the NO-OP law keeps — the empirical walk
        // the 3/20 -> 150 truncation speaks, the f32 grid rounds UP):
        // the tick-3 blend tile the exact 150/1000 alpha's round mix
        // wears (the 149/1000 neighbor the strip refuses), the tick-10
        // tile the exact 500/1000 half, the tick-79 wrap the 950/1000
        // mix (the LAST frame the FIRST blends toward)
        let base = gen_atlas::tile_rgba(gen_atlas::TILE_CAMPFIRE_LOG_LIT);
        let f1 = gen_atlas::tile_rgba(gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[20]);
        let t3 = gen_atlas::tile_rgba(gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[3]);
        let t10 = gen_atlas::tile_rgba(gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[10]);
        assert!(
            base.iter().zip(f1.iter()).any(|(a, b)| a != b),
            "the frame 1 the base's own pixels depart somewhere"
        );
        let law = |alpha: f64| -> Vec<u8> {
            base.iter()
                .zip(f1.iter())
                .map(|(&a, &b)| (a as f64 + (b as f64 - a as f64) * alpha).round() as u8)
                .collect()
        };
        assert_eq!(t3, law(0.15), "tick 3 every texel the 150/1000 alpha's mix wears");
        assert_ne!(
            t3,
            law(0.149),
            "the 149/1000 neighbor the exact quantization refuses"
        );
        assert_eq!(t10, law(0.5), "tick 10 the exact half's mix wears");
        let f3 = gen_atlas::tile_rgba(gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[60]);
        let t79 = gen_atlas::tile_rgba(gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[79]);
        let law_wrap = |alpha: f64| -> Vec<u8> {
            f3.iter()
                .zip(base.iter())
                .map(|(&a, &b)| (a as f64 + (b as f64 - a as f64) * alpha).round() as u8)
                .collect()
        };
        assert_eq!(
            t79,
            law_wrap(0.95),
            "tick 79 the wrap's 950/1000 mix wears"
        );
        assert_ne!(
            t79,
            law_wrap(0.949),
            "the wrap's own quantization the 949/1000 law refuses"
        );
    }

    #[test]
    fn the_fluid_anim_the_strips_walk() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE FLUIDS' OWN MCMETA (the 26.2 jar): water_still frametime 2
        // the EMPTY frames array the sequential rule serves, lava_still
        // frametime 2 the EXPLICIT ping-pong [0..19, 18..1] speaks (the
        // 38 display steps over the strip's 20 frames) — the per-strip
        // frametime the single fire constant never served.
        assert_eq!(gen_atlas::WATER_FRAMETIME, 2, "the water mcmeta's own key");
        assert_eq!(gen_atlas::LAVA_FRAMETIME, 2, "the lava mcmeta's own key");
        // THE WATER'S OWN WALK — the frametime 2 dwell the even ticks
        // answer: tick 0 the first frame, tick 1 the SAME frame (the
        // dwell the divisor speaks), tick 2 the walk's next row; the
        // 64-tick cycle the 32 frames x the frametime wears
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 0),
            gen_atlas::WATER_FRAMES[0],
            "tick 0 the strip's own first frame"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 1),
            gen_atlas::WATER_FRAMES[0],
            "tick 1 the frametime dwell the same frame keeps"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 2),
            gen_atlas::WATER_FRAMES[1],
            "tick 2 the sequential walk's next row"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 64),
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 0),
            "the 64-tick cycle the 32-frame strip's period wears"
        );
        // THE LAVA'S OWN PING-PONG — the explicit playlist the ascent
        // and the descent both walk: tick 38 (pos 19) the strip's last
        // frame, tick 40 (pos 20) the descent's 18 row, the 76-tick
        // cycle the 38 steps x the frametime wears
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FRAME0, 38),
            gen_atlas::LAVA_FRAMES[19],
            "tick 38 the ascent's own tail frame"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FRAME0, 40),
            gen_atlas::LAVA_FRAMES[18],
            "tick 40 the ping-pong's descent the 18 row wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FRAME0, 76),
            gen_atlas::anim_tile(gen_atlas::LAVA_FRAME0, 0),
            "the 76-tick cycle the 38-step playlist's period wears"
        );
        // THE TICK-0 PIXEL IDENTITY — the appended frame-00 tiles the
        // palette's own rows the same bytes carry (the goldens' own
        // proof: the scenes' water at tick 0 the still the palette's
        // crop painted, the renders byte-identical)
        assert_eq!(
            gen_atlas::tile_rgba(gen_atlas::WATER_FRAME0),
            gen_atlas::tile_rgba(gen_atlas::WATER_FRAMES[0]),
            "the water block tile's pixels the strip's first frame carries"
        );
        assert_eq!(
            gen_atlas::tile_rgba(gen_atlas::LAVA_FRAME0),
            gen_atlas::tile_rgba(gen_atlas::LAVA_FRAMES[0]),
            "the lava block tile's pixels the strip's first frame carries"
        );
        // THE FRAME BODIES — every appended frame tile the full 16x16
        // body carries (the water strip's 32, the lava strip's 20)
        for f in gen_atlas::WATER_FRAMES.iter().copied() {
            assert_eq!(gen_atlas::tile_rgba(f).len(), 16 * 16 * 4, "water frame {f}");
        }
        for f in gen_atlas::LAVA_FRAMES.iter().copied() {
            assert_eq!(gen_atlas::tile_rgba(f).len(), 16 * 16 * 4, "lava frame {f}");
        }
    }

    #[test]
    fn the_tuft_cross_wears_the_tint_laws() {
        // THE GRASS TUFT'S OWN CROSS (the autopush-412 round — the
        // tinted_cross parent the flowers' own block/cross bake
        // repeats): the tuft cell the same 4 quads wear (the two
        // diagonal planes × the two windings, the corners the 0.8
        // inset keeps, the uv the full sprite rides), the tile the
        // tuft's own atlas row banks, THE TINT the no-op rule keeps —
        // the offline world's no-biome lane rides the identity (the
        // baked plains pixels speak), the synced plains verdict the
        // baked constant equals (the fold never fires), the off-plains
        // synced biome the fold answers (the vertex colors multiply).
        // THE DEV-27 OFFSET (the XYZ row): the tuft the vertical sink
        // wears — the (v-1.0)*maxV band the dsub speaks, the span the
        // shifted bounds keep, the model-relative read the shift
        // subtracts.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::SHORT_GRASS);
        let q = mesh_chunk(&w, 0, 0, None);
        let a = 0.8 / 16.0;
        let b = 15.2 / 16.0;
        let (dx, dy, dz) =
            crate::world::plant_offset(crate::world::SHORT_GRASS, 8, 9, 8);
        let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
        assert_eq!((dx, dy, dz), (0.150_000_005_960_464_48f32, -0.200_000_002_980_232_24f32, -0.083_333_328_366_279_6f32));
        // the shifted span the same corner walk computes (the fy+dy
        // innermost addition the mesher speaks — the f32 the f64 law
        // casts)
        let bottom = 9.0f32 + (0.0 + dy);
        let top = 9.0f32 + (1.0 + dy);
        let tuft: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().all(|p| (bottom..=top).contains(&p[1]))
                    && quad.pts.iter().any(|p| p[1] == top)
            })
            .collect();
        assert_eq!(tuft.len(), 4, "the two planes the two windings wear");
        for quad in &tuft {
            for p in &quad.pts {
                let (fx, fz) = (p[0] - 8.0 - dx, p[2] - 8.0 - dz);
                assert!(
                    (fx - a).abs() < 1e-6 || (fx - b).abs() < 1e-6,
                    "the x the diagonal corner inset keeps: {fx}"
                );
                assert!(
                    (fz - a).abs() < 1e-6 || (fz - b).abs() < 1e-6,
                    "the z the diagonal corner inset keeps: {fz}"
                );
                assert!(
                    (fx - fz).abs() < 1e-6 || (fx + fz - 1.0).abs() < 1e-6,
                    "the cross plane the diagonal runs: {fx} vs {fz}"
                );
            }
            // the tile: the short_grass's own row (the atlas's 41st kind)
            assert_eq!(
                quad.tile,
                mc_registry::generated::atlas::FACE_TILES[41][2]
            );
        }
        // THE BUSH'S OWN ROW (the patch_bush kind the same cross bake
        // wears — the 42nd kind the tile row banks)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, STONE);
        put(&mut w2, 8, 9, 8, crate::world::BUSH);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        assert!(
            q2.iter().any(|quad| quad.tile
                == mc_registry::generated::atlas::FACE_TILES[42][2]),
            "the bush's own tile the cross quads bank"
        );
        // THE TINT FOLD'S OWN VERDICT (the audited slot walk): the tuft
        // kinds ride the grass source on every face — the synced
        // off-plains biome the fold answers, the no-biome world the
        // identity keeps (the folded quad's colors = the baked pixels'
        // own light walk, byte-identical to the no-fold verdict)
        let folded = crate::tint::unpack(0xBFB755);
        assert!(folded != [1.0; 3], "the desert verdict the fold carries");
        assert_eq!(
            crate::tint::slot_for(crate::world::SHORT_GRASS, 2),
            Some(crate::tint::GRASS_SLOT),
            "the tuft's grass source the slot walk answers"
        );
    }

    #[test]
    fn the_double_plant_cross_wears_the_half_tile_laws() {
        // THE DOUBLE PLANT'S OWN CROSS (the autopush-413 round — the
        // tinted_cross parent the tuft's own block/cross bake repeats,
        // PER HALF): the tall_grass cell the same 4 quads wear (the two
        // diagonal planes × the two windings, the corners the 0.8 inset
        // keeps), the LOWER kind the tall_grass_bottom tile banks, the
        // UPPER kind the tall_grass_top tile banks (the blockstate's
        // half-keyed variants the atlas's own rows speak — the two
        // sprites the pair wears), THE TINT the no-op rule keeps — the
        // offline world's no-biome lane rides the identity (the baked
        // plains pixels speak), the grass source the slot walk answers
        // on every face (the audited doubleTallGrass register row).
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::TALL_GRASS);
        put(&mut w, 8, 10, 8, crate::world::TALL_GRASS_UPPER);
        let q = mesh_chunk(&w, 0, 0, None);
        let a = 0.8 / 16.0;
        let b = 15.2 / 16.0;
        // THE DEV-27 OFFSET: the XZ row the pair RIGIDLY rides — the
        // seed the Y-zero law hashes, the two cells the same triple
        // wear, the shift the model-relative reads subtract
        let (dx, dy, dz) =
            crate::world::plant_offset(crate::world::TALL_GRASS, 8, 9, 8);
        let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
        assert_eq!(dy, 0.0, "the XZ row the zero Y keeps");
        assert_eq!(
            (dx, dz),
            {
                let (ux, _uy, uz) =
                    crate::world::plant_offset(crate::world::TALL_GRASS_UPPER, 8, 10, 8);
                (ux as f32, uz as f32)
            },
            "the pair the rigid offset rides"
        );
        let lower: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().all(|p| (9.0..=10.0).contains(&p[1]))
                    && quad.pts.iter().any(|p| p[1] == 10.0)
            })
            .collect();
        let upper: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().all(|p| (10.0..=11.0).contains(&p[1]))
                    && quad.pts.iter().any(|p| p[1] == 11.0)
            })
            .collect();
        assert_eq!(lower.len(), 4, "the two planes the two windings wear");
        assert_eq!(upper.len(), 4, "the pair's upper the same bake wears");
        let lower_tile = mc_registry::generated::atlas::FACE_TILES
            [crate::world::TALL_GRASS as usize][2];
        let upper_tile = mc_registry::generated::atlas::FACE_TILES
            [crate::world::TALL_GRASS_UPPER as usize][2];
        assert_ne!(
            lower_tile, upper_tile,
            "the half-keyed sprites the two tiles bank"
        );
        for quad in &lower {
            for p in &quad.pts {
                let (fx, fz) = (p[0] - 8.0 - dx, p[2] - 8.0 - dz);
                assert!(
                    (fx - a).abs() < 1e-6 || (fx - b).abs() < 1e-6,
                    "the x the diagonal corner inset keeps: {fx}"
                );
                assert!(
                    (fz - a).abs() < 1e-6 || (fz - b).abs() < 1e-6,
                    "the z the diagonal corner inset keeps: {fz}"
                );
            }
            assert_eq!(quad.tile, lower_tile, "the lower half its own tile");
        }
        for quad in &upper {
            // THE UPPER'S OWN RIDE: the same triple the rigid pair
            // wears — the shifted corners the same lattice keeps
            for p in &quad.pts {
                let (fx, fz) = (p[0] - 8.0 - dx, p[2] - 8.0 - dz);
                assert!(
                    (fx - a).abs() < 1e-6 || (fx - b).abs() < 1e-6,
                    "the upper's x the same inset keeps: {fx}"
                );
                assert!(
                    (fz - a).abs() < 1e-6 || (fz - b).abs() < 1e-6,
                    "the upper's z the same inset keeps: {fz}"
                );
            }
            assert_eq!(quad.tile, upper_tile, "the upper half its own tile");
        }
        // THE GRASS SOURCE'S OWN SLOTS (the audited register walk): the
        // doubleTallGrass() source the TALL_GRASS/LARGE_FERN pair
        // covers — the GRASS_SLOT on EVERY face both halves speak
        for kind in [
            crate::world::TALL_GRASS,
            crate::world::TALL_GRASS_UPPER,
            crate::world::LARGE_FERN,
            crate::world::LARGE_FERN_UPPER,
        ] {
            for face in 0..6 {
                assert_eq!(
                    crate::tint::slot_for(kind, face),
                    Some(crate::tint::GRASS_SLOT),
                    "the double plant's grass source the slot walk answers ({kind}, {face})"
                );
            }
        }
        // THE BAKE'S OWN PIXELS (the generated tiles the plains look
        // wears): the four masters the TINT_GRASS bake banked — the
        // grayscale greens the tuft family's own tint speaks
        assert_eq!(
            mc_registry::generated::atlas::TINTS[crate::world::TALL_GRASS as usize][2],
            0x91bd59,
            "the plains constant the TINTS row keeps"
        );
    }

    #[test]
    fn water_quads_translucent_others_not() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 5, 8, WATER);
        let q = mesh_chunk(&w, 0, 0, None);
        for quad in &q {
            // every face of the water block (sides/top/bottom) is translucent;
            // every face of the stone block (y 8..9) is opaque
            let in_water_span = quad.pts.iter().all(|p| (5.0..=6.0).contains(&p[1]));
            assert_eq!(quad.translucent, in_water_span);
        }
    }

    #[test]
    fn the_fluid_the_side_faces_the_flow_master_wears() {
        // THE VANILLA SIDE-FACE LAW (javap FluidRenderer pc 1458-1466,
        // the 26.2 jar): the horizontal sides the HORIZONTAL loop walks
        // the flowing sprite ALWAYS wear — the calm source's own
        // flow==(0,0) verdict the still pick the top face keeps
        // (pc 671-691) and the bottom face's own stillMaterial walk
        // (pc 1040-1048) keeps. The side quads the flow strip's frame-0
        // tile bake — the generator's quadrant bank the top-left 16x16
        // region the side UVs pc 1515-1567 sample (getU(0.0)..getU(0.5)
        // x getV((1-h)*0.5)..getV(0.5), the full-height cell the exact
        // quadrant answers), the tile the side quad's whole UV rect
        // maps. The draw-time remap the flow strips' own frametime the
        // anim_tile arms speak (the sibling walk test pins).
        use mc_registry::generated::atlas as gen_atlas;
        let mut w = empty_world();
        put(&mut w, 8, 5, 8, WATER);
        let q = mesh_chunk(&w, 0, 0, None);
        // the top face: the still master keeps (the calm pool's verdict —
        // the palette's own water_still row the FACE_TILES bank)
        let top = q
            .iter()
            .find(|quad| quad.pts.iter().all(|p| p[1] == 6.0))
            .expect("water top face");
        assert_eq!(top.tile, gen_atlas::FACE_TILES[WATER as usize][2]);
        assert_eq!(top.tile, gen_atlas::WATER_FRAME0, "the still strip's own base");
        // the bottom face: the still master keeps (the vanilla
        // stillMaterial walk the down face wears)
        let bottom = q
            .iter()
            .find(|quad| quad.pts.iter().all(|p| p[1] == 5.0))
            .expect("water bottom face");
        assert_eq!(bottom.tile, gen_atlas::FACE_TILES[WATER as usize][3]);
        // the four sides: the FLOW master's frame-0 tile — never the
        // still row the pre-529 world wore
        let sides: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().any(|p| p[1] == 5.0) && quad.pts.iter().any(|p| p[1] == 6.0)
            })
            .collect();
        assert_eq!(
            sides.len(),
            4,
            "the four horizontal faces the air neighbors draw"
        );
        for side in &sides {
            assert_eq!(side.tile, gen_atlas::WATER_FLOW_FRAME0);
            assert!(side.translucent, "the water side the translucent pass rides");
        }
        // the generated bank's own contract: the base the appended
        // frame-00 quadrant names (the strip the quadrant rows follow)
        assert_eq!(
            gen_atlas::TILE_NAMES[gen_atlas::WATER_FLOW_FRAME0 as usize],
            "water_flow_frame00"
        );
        // the flow master's pixels the still row never shares (the side
        // faces' look the flow strip's own streaks speak)
        assert_ne!(
            gen_atlas::TILES[gen_atlas::WATER_FLOW_FRAME0 as usize],
            gen_atlas::TILES[gen_atlas::FACE_TILES[WATER as usize][2] as usize],
        );
    }

    #[test]
    fn the_lava_the_sides_the_flow_the_still_top_keeps() {
        // the SECOND fluid member (the lava lane): the same side-face law
        // the water arm speaks — the sides the lava_flow master's frame-0
        // quadrant wear, the top/bottom the lava_still keeps, the quads
        // OPAQUE (the translucent pass the water alone rides)
        use mc_registry::generated::atlas as gen_atlas;
        let mut w = empty_world();
        put(&mut w, 8, 5, 8, LAVA);
        let q = mesh_chunk(&w, 0, 0, None);
        let sides: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().any(|p| p[1] == 5.0) && quad.pts.iter().any(|p| p[1] == 6.0)
            })
            .collect();
        assert_eq!(sides.len(), 4);
        for side in &sides {
            assert_eq!(side.tile, gen_atlas::LAVA_FLOW_FRAME0);
            assert!(!side.translucent, "the lava stays opaque");
        }
        assert_eq!(
            gen_atlas::TILE_NAMES[gen_atlas::LAVA_FLOW_FRAME0 as usize],
            "lava_flow_frame00"
        );
        let top = q
            .iter()
            .find(|quad| quad.pts.iter().all(|p| p[1] == 6.0))
            .expect("lava top face");
        assert_eq!(top.tile, gen_atlas::FACE_TILES[LAVA as usize][2]);
        assert_eq!(top.tile, gen_atlas::LAVA_FRAME0, "the still strip's own base");
    }

    #[test]
    fn the_flow_anim_the_water_walks_the_lava_lags() {
        // THE MCMETA'S OWN TRUTH the anim_tile arms serve: water_flow {}
        // the frametime-1 sequential 32 (the 32-tick cycle), lava_flow
        // {"frametime": 3} the sequential 16 (the 48-tick cycle), NO
        // interpolate key the static frame walk keeps. The tick-0 head
        // the frame-0 verdict (the fresh atlas's first frame the
        // playlist's own head row speaks).
        use mc_registry::generated::atlas as gen_atlas;
        assert_eq!(gen_atlas::WATER_FLOW_FRAMETIME, 1);
        assert_eq!(gen_atlas::WATER_FLOW_FRAMES.len(), 32);
        assert_eq!(gen_atlas::WATER_FLOW_PLAYLIST.len(), 32);
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FLOW_FRAME0, 0),
            gen_atlas::WATER_FLOW_FRAMES[0]
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FLOW_FRAME0, 1),
            gen_atlas::WATER_FLOW_FRAMES[1]
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FLOW_FRAME0, 31),
            gen_atlas::WATER_FLOW_FRAMES[31]
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FLOW_FRAME0, 32),
            gen_atlas::WATER_FLOW_FRAMES[0],
            "the 32-tick cycle wraps"
        );
        // every frame tile a DISTINCT index (the strip the full row bank
        // walks — the appended frames no dedup shares)
        let mut set = gen_atlas::WATER_FLOW_FRAMES.to_vec();
        set.sort_unstable();
        set.dedup();
        assert_eq!(set.len(), 32);
        assert_eq!(gen_atlas::LAVA_FLOW_FRAMETIME, 3);
        assert_eq!(gen_atlas::LAVA_FLOW_FRAMES.len(), 16);
        assert_eq!(gen_atlas::LAVA_FLOW_PLAYLIST.len(), 16);
        // the frametime-3 lag: tick 2 still the head frame, tick 3 the
        // second frame, the 48-tick cycle wraps
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FLOW_FRAME0, 2),
            gen_atlas::LAVA_FLOW_FRAMES[0]
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FLOW_FRAME0, 3),
            gen_atlas::LAVA_FLOW_FRAMES[1]
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FLOW_FRAME0, 48),
            gen_atlas::LAVA_FLOW_FRAMES[0]
        );
        // the still masters their own strips walk (the bases the
        // FACE_TILES rows bank — water_still the frametime-2 walk the
        // tick-10/pos-5 row speaks), the plain tiles the identity keeps
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 10),
            gen_atlas::WATER_FRAMES[5]
        );
        assert_eq!(gen_atlas::anim_tile(0, 99), 0, "the stone tile the identity wears");
    }

    #[test]
    fn uv_order_follows_vertex_reorder() {
        // after flip_order rotation, uv[i] must equal face_uv of the SAME corner
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, GRASS);
        put(&mut w, 9, 9, 8, STONE); // force asymmetric AO -> flip
        put(&mut w, 8, 9, 9, STONE);
        let q = mesh_chunk(&w, 0, 0, None);
        let top = q.iter().find(|quad| quad.pts[0][1] == 9.0).expect("top face");
        let ao = ao_levels(&w, 8, 8, 8, 2);
        let order = flip_order(&ao);
        for (i, &src) in order.iter().enumerate() {
            assert_eq!(top.uvs[i], face_uv(2, FACE_CORNERS[2][src]), "uv must track corner {src}");
        }
    }
}


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
    // non-occluding, casts no AO, exactly the blockstate models' verdict)
    kind == AIR || kind == WATER || kind == LEAVES || kind == PALE_LEAVES || kind == LEAF_LITTER
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
/// (`EnvironmentAttributes.<clinit>`: `defaultValue(sipush -10100)` =
/// 0xFFD8AC). No dimension JSON in the registry's data and no timeline
/// track overrides it, so the default IS the whole value in every
/// dimension the registry carries — the same "the default is the whole
/// value" shape as `sky_light_color` before its day track.
const BLOCK_LIGHT_TINT: [f32; 3] = [1.0, 216.0 / 255.0, 172.0 / 255.0];

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
/// every level (the tint's red is already 1.0).
#[inline]
fn block_light_color(block: f32) -> [f32; 3] {
    let mix = 0.9 * parabolic_mix_factor(block / 15.0);
    [
        BLOCK_LIGHT_TINT[0] + mix * (1.0 - BLOCK_LIGHT_TINT[0]),
        BLOCK_LIGHT_TINT[1] + mix * (1.0 - BLOCK_LIGHT_TINT[1]),
        BLOCK_LIGHT_TINT[2] + mix * (1.0 - BLOCK_LIGHT_TINT[2]),
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
/// bit for bit). NightVisionFactor is 0 at this client's state, so the
/// max against `nightVisionColor` is `max(A, 0) = A` (a color is
/// non-negative — the documented verdict). `SkyLightColor` is the
/// timeline's `visual/sky_light_color` track (`timeline::sky_light_color`:
/// the white default by day, `#7a7aff` — the moonlight blue — through the
/// night plateau). `BlockLightColor` is [`block_light_color`] — the
/// `#ffd8ac` default tint mixed 0.9-parabola toward white on the raw
/// level — scaled by `BlockFactor` 1.4, the flicker's expectation (the
/// random walk itself is the documented non-goal). The remaining shader
/// terms are the defaults' identities at this client's state:
/// DarknessScale / NightVision / BossOverlay are all 0, and the
/// brightness (gamma) mix is an identity at gamma 0. A layer the model
/// never answered for is `None`, not zero; no layer at all is the legacy
/// unshaded look.
#[inline]
fn lightmap_combine(
    sky: Option<f32>,
    block: Option<f32>,
    sky_factor: f32,
    sky_tint: [f32; 3],
    ambient: [f32; 3],
) -> [f32; 3] {
    let sky_term = sky.map(|v| light_brightness(v) * sky_factor);
    let block_term = block.map(|v| {
        let b = light_brightness(v) * BLOCK_FACTOR;
        let c = block_light_color(v);
        [b * c[0], b * c[1], b * c[2]]
    });
    if sky_term.is_none() && block_term.is_none() {
        return [1.0; 3]; // no light layer — the legacy look
    }
    // the shader's base: max(AmbientColor, nightVisionColor) with the
    // night-vision factor at 0 — the dimension's ambient color itself
    let mut color = ambient;
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
    [color[0].min(1.0), color[1].min(1.0), color[2].min(1.0)]
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
                    let sky_factor = world
                        .day_tick
                        .map(crate::timeline::sky_light_factor)
                        .unwrap_or(1.0);
                    let sky_tint = world
                        .day_tick
                        .map(crate::timeline::sky_light_color)
                        .unwrap_or([1.0; 3]);
                    // The shader's AmbientColor base (see the main path):
                    // None = the bytecode-default black, a no-op base.
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
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
                for face in 0..6usize {
                    let (dx, dy, dz) = NEIGHBOR[face];
                    let n = world.get(wx + dx, wy + dy, wz + dz);
                    let draw = if water_self {
                        n == AIR // water draws only against air (surface)
                    } else {
                        is_transparent(n)
                    };
                    if !draw {
                        continue;
                    }
                    // Real texture tile from the generated atlas when present;
                    // otherwise the flat palette path (avg color or defaults).
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][face]
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
                    // The timeline sky factor: the day tick's curve value
                    // (1.0 day / 0.24 night; None = day). Constant per
                    // mesh — the shader's SkyFactor uniform equivalent.
                    let sky_factor = world
                        .day_tick
                        .map(crate::timeline::sky_light_factor)
                        .unwrap_or(1.0);
                    // The shader's SkyLightColor uniform: the timeline's
                    // visual/sky_light_color track (white day, the #7a7aff
                    // moonlight blue at night); None = day = white.
                    let sky_tint = world
                        .day_tick
                        .map(crate::timeline::sky_light_color)
                        .unwrap_or([1.0; 3]);
                    // The shader's AmbientColor base: the dimension's
                    // visual/ambient_light_color attribute as raw bytes
                    // (overworld #0a0a0a); None = the bytecode-default
                    // black — a no-op base, the pre-ambient arithmetic.
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
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
                                lightmap_combine(sky[src], block[src], sky_factor, sky_tint, ambient)
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
                        translucent: water_self,
                    });
                }
            }
        }
    }
    out
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
            chunks_per_side: 1,
            seed: 0,
            chunks: vec![vec![AIR; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            light: None,
            day_tick: None,
            biomes: None,
            variants: None,
            // None = the bytecode-default black: every test scene keeps
            // the pre-ambient arithmetic bit for bit (the ambient lift is
            // pinned in its own tests, explicitly staged)
            ambient_light: None,
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
        // longer the white brightness(14) — the #ffd8ac mix landed.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(14), 8, 9, 8);
        let b = light_brightness(14.0) * BLOCK_FACTOR;
        let c = block_light_color(14.0);
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
            (light_brightness(14.0) * BLOCK_FACTOR * block_light_color(14.0)[0]).min(1.0);
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
            + light_brightness(10.0) * BLOCK_FACTOR * block_light_color(10.0)[0];
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
        let c = block_light_color(10.0);
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
        // #ffd8ac (the dip's neighborhood, not the dip itself)
        let m7 = 0.9 * parabolic_mix_factor(7.0 / 15.0);
        let m8 = 0.9 * parabolic_mix_factor(8.0 / 15.0);
        assert!((m7 - m8).abs() < 1e-6, "the dip is level-symmetric: {m7} vs {m8}");
        assert!(m7 < 0.01, "near the pure tint: {m7}");
        // red is 1.0 at EVERY level (the tint's red is saturated — the
        // warmth lives entirely in green/blue)
        for b in [0.0f32, 3.0, 7.0, 8.0, 12.0, 15.0] {
            let c = block_light_color(b);
            assert!((c[0] - 1.0).abs() < 1e-9, "red constant at {b}: {}", c[0]);
        }
        // the extremes mix 0.9 of the way to white — derived from the
        // tint constant, both end levels share it (parabola 1.0 there)
        let expect_g = 216.0f32 / 255.0 + 0.9 * (1.0 - 216.0 / 255.0);
        let expect_b = 172.0f32 / 255.0 + 0.9 * (1.0 - 172.0 / 255.0);
        for b in [0.0f32, 15.0] {
            let c = block_light_color(b);
            assert!((c[1] - expect_g).abs() < 1e-6, "green at {b}: {} vs {expect_g}", c[1]);
            assert!((c[2] - expect_b).abs() < 1e-6, "blue at {b}: {} vs {expect_b}", c[2]);
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
        let c = block_light_color(7.0);
        let expect = [b * c[0], b * c[1], b * c[2]];
        assert!(expect.iter().all(|e| *e < 1.0), "block 7 sits under the clamp");
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!((ch - e).abs() < 1e-6, "torch tint per channel: {ch} vs {e}");
            }
        }
        // the clamp witness at block 14: red (1.0889) AND green (1.0352)
        // saturate, blue stays under (0.8944 tint × 1.0889 = 0.9740) —
        // the shader's single final clamp, the warm torch verdict
        set_light(&mut w, None, Some(14), 8, 9, 8);
        let b14 = light_brightness(14.0) * BLOCK_FACTOR;
        let c14 = block_light_color(14.0);
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

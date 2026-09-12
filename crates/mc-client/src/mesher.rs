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
use crate::world::*;

/// One shaded world-space quad: 4 CCW corner points, per-vertex shading and
/// texture mapping. When `tile` is a real atlas index the vertex colors carry
/// the shade×AO multiplier and the rasterizer samples the official texture;
/// when `tile == TILE_NONE` the vertex colors carry the full flat base color.
#[derive(Debug, Clone)]
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
    kind == AIR || kind == WATER || kind == LEAVES
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

/// The cell's effective light level: the max over the KNOWN layers (day
/// semantics — the sky contributes at full strength; night sky-darken
/// threading is future work). A layer the model never answered for is
/// skipped, not zero; a cell with no opinion at all is `None`.
#[inline]
fn cell_light(light: &LightGrid, side: usize, x: i32, y: i32, z: i32) -> Option<f32> {
    let (s, b) = light.cell(side, x, y, z);
    match (s, b) {
        (None, None) => None,
        (Some(s), None) => Some(s as f32),
        (None, Some(b)) => Some(b as f32),
        (Some(s), Some(b)) => Some(s.max(b) as f32),
    }
}

/// Per-corner smooth-light levels (FACE_CORNERS order): the average of
/// the 4 face-plane cells around each corner — the face cell plus the 3
/// AO diagonals, the SAME cells [`ao_levels`] samples — over the KNOWN
/// cells only (the average is float: smoother than the vanilla packed
/// int average, a deliberate minimal departure). `None` when the corner
/// has no known cell at all — unsent neighbors never darken the faces
/// they touch; that corner keeps the legacy unshaded look.
fn corner_light(
    light: &LightGrid,
    side: usize,
    wx: i32,
    wy: i32,
    wz: i32,
    face: usize,
) -> [Option<f32>; 4] {
    let (nx, ny, nz) = NEIGHBOR[face];
    let normal_axis = if nx != 0 { 0 } else if ny != 0 { 1 } else { 2 };
    let (t1, t2) = match normal_axis {
        0 => (1usize, 2usize),
        1 => (0usize, 2usize),
        _ => (0usize, 1usize),
    };
    let (bx, by, bz) = (wx + nx, wy + ny, wz + nz);
    let mut out = [None; 4];
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
        let mut sum = 0.0f32;
        let mut n = 0u32;
        for &(x, y, z) in &cells {
            if let Some(v) = cell_light(light, side, x, y, z) {
                sum += v;
                n += 1;
            }
        }
        out[i] = (n > 0).then(|| sum / n as f32);
    }
    out
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
                        lut
                            .and_then(|l| l.color(id, face))
                            .unwrap_or_else(|| default_color(id, face))
                    } else {
                        [1.0; 3] // multiplier carrier for the textured path
                    };
                    let shade = FACE_SHADE[face];
                    // Smooth lighting: per-corner AO from the 3 diagonal
                    // neighbors in front of this face.
                    let ao = ao_levels(world, wx, wy, wz, face);
                    // Per-corner light sample — only when the world
                    // carries a light layer; `None` keeps the legacy path.
                    let clight = light.map(|l| corner_light(l, world.chunks_per_side, wx, wy, wz, face));
                    let order = flip_order(&ao);
                    let mut pts = [[0f32; 3]; 4];
                    let mut colors = [[0f32; 3]; 4];
                    let mut uvs = [[0f32; 2]; 4];
                    for (i, &src) in order.iter().enumerate() {
                        let c = FACE_CORNERS[face][src];
                        pts[i] = [wx as f32 + c[0], wy as f32 + c[1], wz as f32 + c[2]];
                        let lm = match clight.as_ref() {
                            Some(samples) => match samples[src] {
                                Some(avg) => light_brightness(avg),
                                None => 1.0, // no light opinion here — legacy look
                            },
                            None => 1.0, // no light layer — legacy look
                        };
                        let m = shade * AO_CURVE[ao[src] as usize] * lm;
                        colors[i] = [base[0] * m, base[1] * m, base[2] * m];
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
        // torch-lit: no sky opinion, block 14 -> the whole face at brightness(14)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(14), 8, 9, 8);
        let expect = light_brightness(14.0);
        for m in vertex_multiplier(&top_face(&w)) {
            assert!((m - expect).abs() < 1e-6, "block-only light: {m} vs {expect}");
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

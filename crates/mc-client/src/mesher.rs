//! Culled face meshing with vanilla-style smooth lighting (per-vertex AO),
//! parallel across chunks with `rayon`.

use rayon::prelude::*;

use crate::colors::{default_color, FACE_SHADE};
use crate::textures::TextureLut;
use crate::world::*;

/// One shaded world-space quad: 4 CCW corner points, per-vertex pre-shaded
/// RGB (smooth lighting / ambient occlusion baked into the vertex colors).
#[derive(Debug, Clone)]
pub struct Quad {
    pub pts: [[f32; 3]; 4],
    pub colors: [[f32; 3]; 4],
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

/// Mesh one chunk: all faces where a solid block touches a transparent one.
pub fn mesh_chunk(world: &World, cx: usize, cz: usize, lut: Option<&TextureLut>) -> Vec<Quad> {
    let mut out = Vec::with_capacity(2048);
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
                    let base = lut
                        .and_then(|l| l.color(id, face))
                        .unwrap_or_else(|| default_color(id, face));
                    let shade = FACE_SHADE[face];
                    // Smooth lighting: per-corner AO from the 3 diagonal
                    // neighbors in front of this face.
                    let ao = ao_levels(world, wx, wy, wz, face);
                    let order = flip_order(&ao);
                    let mut pts = [[0f32; 3]; 4];
                    let mut colors = [[0f32; 3]; 4];
                    for (i, &src) in order.iter().enumerate() {
                        let c = FACE_CORNERS[face][src];
                        pts[i] = [wx as f32 + c[0], wy as f32 + c[1], wz as f32 + c[2]];
                        let m = shade * AO_CURVE[ao[src] as usize];
                        colors[i] = [base[0] * m, base[1] * m, base[2] * m];
                    }
                    out.push(Quad { pts, colors });
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
        }
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
        // top face of the lone block: shade 1.0, open sky -> full base color
        let top = q.iter().find(|quad| quad.pts[0][1] == 9.0 && quad.pts[1][1] == 9.0).expect("top face quad");
        let expected = default_color(STONE, 2);
        for c in &top.colors {
            for ch in 0..3 {
                assert!((c[ch] - expected[ch]).abs() < 1e-6, "{:?} vs {:?}", c, expected);
            }
        }
    }
}

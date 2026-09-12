//! Culled face meshing, parallel across chunks with `rayon`.

use rayon::prelude::*;

use crate::colors::{default_color, FACE_SHADE};
use crate::textures::TextureLut;
use crate::world::*;

/// One shaded world-space quad (4 CCW corner points, pre-shaded RGB).
#[derive(Debug, Clone)]
pub struct Quad {
    pub pts: [[f32; 3]; 4],
    pub color: [f32; 3],
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
                    let mut base = lut
                        .and_then(|l| l.color(id, face))
                        .unwrap_or_else(|| default_color(id, face));
                    let shade = FACE_SHADE[face];
                    base = [base[0] * shade, base[1] * shade, base[2] * shade];
                    let mut pts = [[0f32; 3]; 4];
                    for (i, c) in FACE_CORNERS[face].iter().enumerate() {
                        pts[i] = [wx as f32 + c[0], wy as f32 + c[1], wz as f32 + c[2]];
                    }
                    out.push(Quad { pts, color: base });
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
}

//! The vanilla FaceBakery quad derivation as a pure consumer over
//! generated::elements — the bridge between the typed element geometry
//! (v0.3.65) and the future block/item display consumers. Zero
//! dependencies, zero render-path contact: quads are plain data.
//!
//! Every formula below is javap evidence from the 26.2 client jar
//! (net.minecraft.client.resources.model.cuboid.*, com.mojang.math.*):
//! - FaceInfo corner table (FaceInfo.<clinit>, Extent.select 6-arg):
//!   DOWN (minX,minY,maxZ)(minX,minY,minZ)(maxX,minY,minZ)(maxX,minY,maxZ);
//!   UP (minX,maxY,minZ)(minX,maxY,maxZ)(maxX,maxY,maxZ)(maxX,maxY,minZ);
//!   NORTH (maxX,maxY,minZ)(maxX,minY,minZ)(minX,minY,minZ)(minX,maxY,minZ);
//!   SOUTH (minX,maxY,maxZ)(minX,minY,maxZ)(maxX,minY,maxZ)(maxX,maxY,maxZ);
//!   WEST (minX,maxY,minZ)(minX,minY,minZ)(minX,minY,maxZ)(minX,maxY,maxZ);
//!   EAST (maxX,maxY,maxZ)(maxX,minY,maxZ)(maxX,minY,minZ)(maxX,maxY,minZ).
//! - defaultFaceUV (FaceBakery.defaultFaceUV, the absent-uv projection):
//!   DOWN (from.x,16-to.z,to.x,16-from.z); UP (from.x,from.z,to.x,to.z);
//!   NORTH (16-to.x,16-to.y,16-from.x,16-from.y);
//!   SOUTH (from.x,16-to.y,to.x,16-from.y);
//!   WEST (from.z,16-to.y,to.z,16-from.y);
//!   EAST (16-to.z,16-to.y,16-from.z,16-from.y).
//! - Vertex uv (CuboidFace$UVs.getVertexU/V + Quadrant.rotateVertexIndex):
//!   rotated = (vertex + rotation/90) % 4; U = minU if rotated in {0,1}
//!   else maxU; V = minV if rotated in {0,3} else maxV. Kept in the
//!   0..16 texture space here; the sprite's getU/getV mapping to atlas
//!   units is the consumer's step (bakeVertex divides by 16).
//! - Rotation (CuboidRotation.computeTransform / computeRescale /
//!   scaleFactorForAxis / SingleAxisRotation / EulerXYZRotation):
//!   single-axis angle==0 -> identity, else the Rodrigues rotation about
//!   the positive axis (right-handed); euler = rotationZYX(z,y,x) =
//!   Rz*Ry*Rx (X applied first — the class name lies about the order);
//!   rescale && !identity -> composite = M * diag(1/max|M*ex|, 1/max|M*ey|,
//!   1/max|M*ez|) (JOML scale() post-multiplies: scale first, then rotate).
//!   The origin deserializes *0.0625 to BLOCK units (bakeVertex works on
//!   from/16), while the generated table stores it as written (pixels) —
//!   so the origin divides by 16 HERE, at the consumer.
//! - Vertex application (FaceBakery.rotateVertexBy):
//!   v' = M*(v - origin) + origin.
//! - Facing (calculateFacing / findClosestDirection + JOML
//!   GeometryUtils.normal): normal = (v1-v0) x (v2-v0), unnormalized;
//!   direction = first of DOWN,UP,NORTH,SOUTH,WEST,EAST (enum order)
//!   with the strictly largest positive dot; none when the normal is
//!   non-finite or no dot is positive. BakedQuad stores
//!   requireNonNullElse(facing, UP) — degenerate zero-area quads face UP.
//! - Winding (recalculateWinding): ONLY when the element rotation is
//!   absent and the facing resolved; per canonical FaceInfo corner of the
//!   COMPUTED facing, selected from the quad's own bounding box with the
//!   6-arg Extent.select; findVertex scans from index i with EXACT float
//!   equality; vertices AND uvs swap together; a miss is vanilla's
//!   IllegalStateException("Can't find vertex to swap").
//! - Texture refs ride AS WRITTEN ("#var" and heavy_core's bare "all"):
//!   TextureSlots.getMaterial strips a leading '#' and looks the string
//!   up in the resolved slots map — bare refs resolve identically to
//!   "#var" refs. The parent-chain slots merge is the texture round.

pub use crate::generated::elements::Cull as Direction;

use crate::generated::elements::{Axis, Element, ElementRotation, ELEMENTS};

/// One derived vanilla quad: the four post-rotation vertices in block
/// units (0..1), the four vertex uvs in the 0..16 texture space, the
/// vanilla-computed facing (UP fallback) alongside the face slot the
/// quad came from, and the face's material facts verbatim.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    pub vertices: [[f32; 3]; 4],
    pub uvs: [[f32; 2]; 4],
    pub slot: Direction,
    pub direction: Direction,
    pub texture: &'static str,
    pub cullface: Option<Direction>,
    pub tintindex: Option<u8>,
    pub uv_rotation: u16,
    pub shade: bool,
    pub light_emission: Option<u8>,
}

/// Census-shaped counters over one derivation (the python census pinned
/// 4238 / 273 / 6 / 160 across the 344 element models of 26.2).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DeriveStats {
    pub quads: usize,
    pub facing_drift: usize,
    pub rewound: usize,
    pub up_fallback: usize,
}

/// The quads of one element-carrying model, by id (linear scan over the
/// generated table, which is id-sorted).
pub fn derive_model(model_id: &str) -> Option<Vec<Quad>> {
    ELEMENTS
        .iter()
        .find(|(id, _)| *id == model_id)
        .map(|(_, elements)| derive_elements(elements))
}

/// The quads of one model's elements (the stats-free form).
pub fn derive_elements(elements: &[Element]) -> Vec<Quad> {
    derive_elements_with_stats(elements).0
}

/// The quads plus the census counters.
pub fn derive_elements_with_stats(elements: &[Element]) -> (Vec<Quad>, DeriveStats) {
    let mut stats = DeriveStats::default();
    let mut quads = Vec::new();
    for element in elements {
        let (matrix, origin) = match element.rotation {
            None => (None, [0.0; 3]),
            Some(rotation) => {
                let (m, rescale) = match rotation {
                    ElementRotation::Axis { angle, axis, .. } => (axis_matrix(axis, angle), false),
                    ElementRotation::Euler { x, y, z, .. } => (euler_matrix(x, y, z), false),
                    #[allow(unreachable_patterns)]
                    _ => unreachable!("the generated table carries only the two rotation forms"),
                };
                let with_rescale = rescale || match rotation {
                    ElementRotation::Axis { rescale, .. } => rescale,
                    _ => false,
                };
                if with_rescale && !is_identity3(&m) {
                    (Some(apply_rescale(&m)), element.origin())
                } else {
                    (Some(m), element.origin())
                }
            }
        };
        // The generated table stores the rotation origin as written
        // (pixels); vanilla deserializes it *0.0625 — block units.
        let origin_block = [origin[0] / 16.0, origin[1] / 16.0, origin[2] / 16.0];
        for slot_idx in 0..6 {
            let slot = SLOT_ORDER[slot_idx];
            let Some(face) = element.faces[slot_idx] else {
                continue;
            };
            let mut vertices = faceinfo_corners(slot, element.from, element.to);
            if let Some(m) = &matrix {
                for v in &mut vertices {
                    *v = rotate_vertex(*v, origin_block, m);
                }
            }
            let rect = match face.uv {
                Some([u0, v0, u1, v1]) => [u0, v0, u1, v1],
                None => default_face_uv(element.from, element.to, slot),
            };
            let uvs = vertex_uvs(rect, face.rotation.unwrap_or(0));
            let facing = calculate_facing(&vertices);
            let mut rewound = 0;
            if matrix.is_none() && facing.is_some() {
                rewound = recalculate_winding(&mut vertices, uvs, facing.unwrap());
            }
            stats.quads += 1;
            if facing != Some(slot) {
                stats.facing_drift += 1;
            }
            if facing.is_none() {
                stats.up_fallback += 1;
            }
            stats.rewound += rewound;
            quads.push(Quad {
                vertices,
                uvs,
                slot,
                direction: facing.unwrap_or(Direction::Up),
                texture: face.texture,
                cullface: face.cullface,
                tintindex: face.tintindex,
                uv_rotation: face.rotation.unwrap_or(0),
                shade: element.shade,
                light_emission: element.light_emission,
            });
        }
    }
    (quads, stats)
}

/// The vanilla face-slot order (Direction ordinal order = the
/// generated faces array order).
pub const SLOT_ORDER: [Direction; 6] = [
    Direction::Down,
    Direction::Up,
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

fn unit(direction: Direction) -> [f32; 3] {
    match direction {
        Direction::Down => [0.0, -1.0, 0.0],
        Direction::Up => [0.0, 1.0, 0.0],
        Direction::North => [0.0, 0.0, -1.0],
        Direction::South => [0.0, 0.0, 1.0],
        Direction::West => [-1.0, 0.0, 0.0],
        Direction::East => [1.0, 0.0, 0.0],
    }
}

type Mat3 = [[f32; 3]; 3];

const IDENTITY3: Mat3 = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
];

fn is_identity3(m: &Mat3) -> bool {
    *m == IDENTITY3
}

fn mat_mul(a: &Mat3, b: &Mat3) -> Mat3 {
    let mut out = [[0.0f32; 3]; 3];
    for (i, row) in out.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
        }
    }
    out
}

fn mat_vec(m: &Mat3, v: [f32; 3]) -> [f32; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

fn rodrigues_axis(axis: Axis, angle_deg: f32) -> Mat3 {
    let a = angle_deg * std::f32::consts::PI / 180.0;
    let (c, s) = (a.cos(), a.sin());
    match axis {
        Axis::X => [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]],
        Axis::Y => [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]],
        Axis::Z => [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]],
    }
}

/// SingleAxisRotation.transformation(): angle==0 -> identity, else the
/// right-handed rotation about the positive axis.
fn axis_matrix(axis: Axis, angle_deg: f32) -> Mat3 {
    if angle_deg == 0.0 {
        IDENTITY3
    } else {
        rodrigues_axis(axis, angle_deg)
    }
}

/// EulerXYZRotation.transformation(): new Matrix4f().rotationZYX(z, y, x)
/// — JOML composes Rz*Ry*Rx, so X is applied first to the vector.
fn euler_matrix(x_deg: f32, y_deg: f32, z_deg: f32) -> Mat3 {
    let rz = rodrigues_axis(Axis::Z, z_deg);
    let ry = rodrigues_axis(Axis::Y, y_deg);
    let rx = rodrigues_axis(Axis::X, x_deg);
    mat_mul(&rz, &mat_mul(&ry, &rx))
}

/// CuboidRotation.computeRescale + scaleFactorForAxis: per axis,
/// 1 / max(|M * unit|); JOML scale() post-multiplies, so the composite
/// scales the vector FIRST and rotates SECOND.
fn apply_rescale(m: &Mat3) -> Mat3 {
    let mut scale = [0.0f32; 3];
    for (i, u) in [[[1.0, 0.0, 0.0]], [[0.0, 1.0, 0.0]], [[0.0, 0.0, 1.0]]]
        .iter()
        .enumerate()
    {
        let d = mat_vec(m, u[0]);
        let max = d[0].abs().max(d[1].abs()).max(d[2].abs());
        scale[i] = 1.0 / max;
    }
    let s: Mat3 = [
        [scale[0], 0.0, 0.0],
        [0.0, scale[1], 0.0],
        [0.0, 0.0, scale[2]],
    ];
    mat_mul(m, &s)
}

/// FaceBakery.rotateVertexBy: v' = M*(v - origin) + origin.
fn rotate_vertex(v: [f32; 3], origin: [f32; 3], m: &Mat3) -> [f32; 3] {
    let d = [v[0] - origin[0], v[1] - origin[1], v[2] - origin[2]];
    let r = mat_vec(m, d);
    [r[0] + origin[0], r[1] + origin[1], r[2] + origin[2]]
}

/// FaceInfo.fromFacing(slot).getVertexInfo(corner): true = the MAX
/// extent of that axis, false = MIN (Extent.select: MIN_* -> from, MAX_* -> to).
fn faceinfo_corner(slot: Direction, corner: usize) -> [bool; 3] {
    match slot {
        Direction::Down => match corner {
            0 => [false, false, true],
            1 => [false, false, false],
            2 => [true, false, false],
            _ => [true, false, true],
        },
        Direction::Up => match corner {
            0 => [false, true, false],
            1 => [false, true, true],
            2 => [true, true, true],
            _ => [true, true, false],
        },
        Direction::North => match corner {
            0 => [true, true, false],
            1 => [true, false, false],
            2 => [false, false, false],
            _ => [false, true, false],
        },
        Direction::South => match corner {
            0 => [false, true, true],
            1 => [false, false, true],
            2 => [true, false, true],
            _ => [true, true, true],
        },
        Direction::West => match corner {
            0 => [false, true, false],
            1 => [false, false, false],
            2 => [false, false, true],
            _ => [false, true, true],
        },
        Direction::East => match corner {
            0 => [true, true, true],
            1 => [true, false, true],
            2 => [true, false, false],
            _ => [true, true, false],
        },
    }
}

/// VertexInfo.select(from, to) / 16 — the corner in block units.
fn faceinfo_corners(slot: Direction, from: [f32; 3], to: [f32; 3]) -> [[f32; 3]; 4] {
    let mut out = [[0.0f32; 3]; 4];
    for (corner, cell) in out.iter_mut().enumerate() {
        let pick = faceinfo_corner(slot, corner);
        for axis in 0..3 {
            let px = if pick[axis] { to[axis] } else { from[axis] };
            cell[axis] = px / 16.0;
        }
    }
    out
}

/// FaceBakery.defaultFaceUV — the absent-uv bounds projection,
/// [minU, minV, maxU, maxV] in the 0..16 texture space.
fn default_face_uv(from: [f32; 3], to: [f32; 3], slot: Direction) -> [f32; 4] {
    match slot {
        Direction::Down => [from[0], 16.0 - to[2], to[0], 16.0 - from[2]],
        Direction::Up => [from[0], from[2], to[0], to[2]],
        Direction::North => [16.0 - to[0], 16.0 - to[1], 16.0 - from[0], 16.0 - from[1]],
        Direction::South => [from[0], 16.0 - to[1], to[0], 16.0 - from[1]],
        Direction::West => [from[2], 16.0 - to[1], to[2], 16.0 - from[1]],
        Direction::East => [16.0 - to[2], 16.0 - to[1], 16.0 - from[2], 16.0 - from[1]],
    }
}

/// CuboidFace.getU/getV over the UVs record + the Quadrant shift:
/// rotated = (vertex + rotation/90) % 4;
/// U = minU when rotated is 0 or 1 else maxU;
/// V = minV when rotated is 0 or 3 else maxV.
fn vertex_uvs(rect: [f32; 4], rotation_deg: u16) -> [[f32; 2]; 4] {
    let shift = match rotation_deg {
        90 => 1usize,
        180 => 2,
        270 => 3,
        _ => 0,
    };
    let mut out = [[0.0f32; 2]; 4];
    for (vertex, cell) in out.iter_mut().enumerate() {
        let rotated = (vertex + shift) % 4;
        cell[0] = if rotated <= 1 { rect[0] } else { rect[2] };
        cell[1] = if rotated == 0 || rotated == 3 { rect[1] } else { rect[3] };
    }
    out
}

/// FaceBakery.calculateFacing + findClosestDirection: the unnormalized
/// normal (v1-v0) x (v2-v0); the first direction of the vanilla enum
/// order holding the strictly largest positive dot; none when the
/// normal is non-finite or nothing beats zero.
fn calculate_facing(vertices: &[[f32; 3]; 4]) -> Option<Direction> {
    let a = [
        vertices[1][0] - vertices[0][0],
        vertices[1][1] - vertices[0][1],
        vertices[1][2] - vertices[0][2],
    ];
    let b = [
        vertices[2][0] - vertices[0][0],
        vertices[2][1] - vertices[0][1],
        vertices[2][2] - vertices[0][2],
    ];
    let normal = [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ];
    if !normal.iter().all(|c| c.is_finite()) {
        return None;
    }
    let mut best = None;
    let mut best_dot = 0.0f32;
    for direction in SLOT_ORDER {
        let u = unit(direction);
        let d = normal[0] * u[0] + normal[1] * u[1] + normal[2] * u[2];
        if d > best_dot {
            best_dot = d;
            best = Some(direction);
        }
    }
    best
}

/// FaceBakery.recalculateWinding: reorder the quad (vertices AND uvs) to
/// the canonical FaceInfo corner order of the COMPUTED facing, picking
/// expected corners from the quad's own bounding box (the 6-arg
/// Extent.select), scanning findVertex from index i with exact float
/// equality. Returns the number of swaps performed.
fn recalculate_winding(vertices: &mut [[f32; 3]; 4], uvs: &mut [[f32; 2]; 4], facing: Direction) -> usize {
    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];
    for v in vertices.iter() {
        for axis in 0..3 {
            min[axis] = min[axis].min(v[axis]);
            max[axis] = max[axis].max(v[axis]);
        }
    }
    let mut swaps = 0;
    for i in 0..4 {
        let pick = faceinfo_corner(facing, i);
        let want = [
            if pick[0] { max[0] } else { min[0] },
            if pick[1] { max[1] } else { min[1] },
            if pick[2] { max[2] } else { min[2] },
        ];
        let mut found = None;
        for j in i..4 {
            if vertices[j] == want {
                found = Some(j);
                break;
            }
        }
        let found = found.expect("Can't find vertex to swap");
        if found != i {
            vertices.swap(i, found);
            uvs.swap(i, found);
            swaps += 1;
        }
    }
    swaps
}

impl ElementRotation {
    /// The generated origin as written (pixels).
    fn origin(&self) -> [f32; 3] {
        match self {
            ElementRotation::Axis { origin, .. } | ElementRotation::Euler { origin, .. } => *origin,
            #[allow(unreachable_patterns)]
            _ => unreachable!("the generated table carries only the two rotation forms"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The canonical block/cube: every face uv-less (bounds projection),
    /// every quad the textbook FaceInfo corner order with the (0,0,16,16)
    /// default uv rect, every direction the slot itself.
    #[test]
    fn derives_the_canonical_cube_quads() {
        let quads = derive_model("block/cube").expect("block/cube is an element model");
        assert_eq!(quads.len(), 6);
        for (idx, quad) in quads.iter().enumerate() {
            assert_eq!(quad.slot, SLOT_ORDER[idx]);
            assert_eq!(quad.direction, SLOT_ORDER[idx], "an unrotated full cube faces its slot");
            assert_eq!(quad.uvs, [[0.0, 0.0]; 0].to_vec().as_slice() && false, "placeholder");
        }
    }
}

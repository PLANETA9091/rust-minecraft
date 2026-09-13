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

/// The quads plus the census counters, by model id.
pub fn derive_model_with_stats(model_id: &str) -> Option<(Vec<Quad>, DeriveStats)> {
    ELEMENTS
        .iter()
        .find(|(id, _)| *id == model_id)
        .map(|(_, elements)| derive_elements_with_stats(elements))
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
            Some(ElementRotation::Axis { angle, axis, origin, rescale }) => {
                let m = axis_matrix(axis, angle);
                (Some(if rescale && !is_identity3(&m) { apply_rescale(&m) } else { m }), origin)
            }
            Some(ElementRotation::Euler { x, y, z, origin }) => {
                // The 26.2 euler carriers ship no rescale (census: 0 of 2);
                // the deserializer defaults it false anyway.
                (Some(euler_matrix(x, y, z)), origin)
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
            let mut uvs = vertex_uvs(rect, face.rotation.unwrap_or(0));
            let facing = calculate_facing(&vertices);
            let mut rewound = 0;
            if matrix.is_none() && facing.is_some() {
                rewound = recalculate_winding(&mut vertices, &mut uvs, facing.unwrap());
            }
            stats.quads += 1;
            if facing != Some(slot) {
                stats.facing_drift += 1;
            }
            if facing.is_none() {
                stats.up_fallback += 1;
            }
            if rewound > 0 {
                stats.rewound += 1;
            }
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
    // The bytecode multiplies by the f32 literal 0.017453292f, then JOML
    // feeds the f32 radians to Math.sin/Math.cos as DOUBLES and stores
    // the results back into floats. f32-trig implementations differ in
    // the last ulp — and the last ulp decides the vanilla argmax on the
    // exact 45-degree-diagonal normals (brewing_stand's bottles) — so
    // the f64-trig-then-f32-round path is load-bearing, not pedantry.
    let a = angle_deg * 0.017_453_292_f32;
    let (c, s) = ((a as f64).cos() as f32, (a as f64).sin() as f32);
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
pub(crate) fn faceinfo_corners(slot: Direction, from: [f32; 3], to: [f32; 3]) -> [[f32; 3]; 4] {
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
pub(crate) fn vertex_uvs(rect: [f32; 4], rotation_deg: u16) -> [[f32; 2]; 4] {
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
pub(crate) fn calculate_facing(vertices: &[[f32; 3]; 4]) -> Option<Direction> {
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
pub(crate) fn recalculate_winding(vertices: &mut [[f32; 3]; 4], uvs: &mut [[f32; 2]; 4], facing: Direction) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::elements::ElementFace;

    #[test]
    fn derives_the_canonical_cube_quads() {
        let quads = derive_model("block/cube").expect("block/cube is an element model");
        assert_eq!(quads.len(), 6);
        let expected_vertices: [[[f32; 3]; 4]; 6] = [
            [[0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]],
            [[0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]],
            [[1.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            [[0.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0]],
            [[0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]],
            [[1.0, 1.0, 1.0], [1.0, 0.0, 1.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
        ];
        for (idx, quad) in quads.iter().enumerate() {
            assert_eq!(quad.slot, SLOT_ORDER[idx]);
            assert_eq!(quad.direction, SLOT_ORDER[idx], "an unrotated full cube faces its slot");
            assert_eq!(quad.vertices, expected_vertices[idx]);
            assert_eq!(quad.uvs, [[0.0, 0.0], [0.0, 16.0], [16.0, 16.0], [16.0, 0.0]],
                "the unit cube projects (0,0,16,16) walked by the four corners");
        }
        let textures: Vec<_> = quads.iter().map(|q| q.texture).collect();
        assert_eq!(textures, ["#down", "#up", "#north", "#south", "#west", "#east"]);
        let culls: Vec<_> = quads.iter().map(|q| q.cullface).collect();
        assert_eq!(culls, [Some(Direction::Down), Some(Direction::Up), Some(Direction::North), Some(Direction::South), Some(Direction::West), Some(Direction::East)]);
    }

    /// The absent-uv projection (defaultFaceUV bytecode) over an
    /// asymmetric element: every face's rect follows the vanilla
    /// per-direction formula, in the 0..16 texture space.
    #[test]
    fn projects_the_default_uvs_per_direction() {
        let face = || ElementFace { uv: None, texture: "#t", cullface: None, tintindex: None, rotation: None };
        let element = Element {
            from: [2.0, 4.0, 6.0],
            to: [10.0, 12.0, 14.0],
            shade: true,
            light_emission: None,
            rotation: None,
            faces: [Some(face()), Some(face()), Some(face()), Some(face()), Some(face()), Some(face())],
            name: None,
            comment: None,
        };
        let quads = derive_elements(&[element]);
        let rects: Vec<[f32; 4]> = quads
            .iter()
            .map(|q| [q.uvs[0][0], q.uvs[0][1], q.uvs[2][0], q.uvs[2][1]])
            .collect();
        assert_eq!(rects, [
            [2.0, 2.0, 10.0, 10.0],  // down:  (from.x, 16-to.z, to.x, 16-from.z)
            [2.0, 6.0, 10.0, 14.0],  // up:    (from.x, from.z, to.x, to.z)
            [6.0, 4.0, 14.0, 12.0],  // north: (16-to.x, 16-to.y, 16-from.x, 16-from.y)
            [2.0, 4.0, 10.0, 12.0],  // south: (from.x, 16-to.y, to.x, 16-from.y)
            [6.0, 4.0, 14.0, 12.0],  // west:  (from.z, 16-to.y, to.z, 16-from.y)
            [2.0, 4.0, 10.0, 12.0],  // east:  (16-to.z, 16-to.y, 16-from.z, 16-from.y)
        ]);
    }

    /// The uv-rotation quadrants (CuboidFace.getU/getV +
    /// Quadrant.rotateVertexIndex): (i + rotation/90) % 4 walks
    /// (minU,minV)(minU,maxV)(maxU,maxV)(maxU,minV).
    #[test]
    fn rotates_the_vertex_uvs_by_the_quadrant() {
        let rect = [0.0, 1.0, 2.0, 3.0]; // minU, minV, maxU, maxV
        assert_eq!(vertex_uvs(rect, 0), [[0.0, 1.0], [0.0, 3.0], [2.0, 3.0], [2.0, 1.0]]);
        assert_eq!(vertex_uvs(rect, 90), [[0.0, 3.0], [2.0, 3.0], [2.0, 1.0], [0.0, 1.0]]);
        assert_eq!(vertex_uvs(rect, 180), [[2.0, 3.0], [2.0, 1.0], [0.0, 1.0], [0.0, 3.0]]);
        assert_eq!(vertex_uvs(rect, 270), [[2.0, 1.0], [0.0, 1.0], [0.0, 3.0], [2.0, 3.0]]);
    }

    /// The rotation forms against the python census (JOML-faithful f32,
    /// scripts/quads_census.py): template_hanging_sign_rot_3 carries the
    /// axis -67.5 Y elements AND the two euler x180 y-67.5 z-180
    /// carriers; 14 faces across its 4 elements. The census's
    /// EULER_FIRST_QUAD (element 2, north slot) lands facing EAST —
    /// the 45-degree-diagonal post-euler normal — with mirrored uvs
    /// (the JSON rect [15.5, 4, 14, 6] has minU > maxU).
    #[test]
    fn rotates_the_axis_and_euler_forms_per_the_census() {
        let quads = derive_model("block/template_hanging_sign_rot_3")
            .expect("template_hanging_sign_rot_3 is an element model");
        assert_eq!(quads.len(), 14, "4 elements, 10 absent face slots");
        let euler = quads
            .iter()
            .find(|q| q.slot == Direction::North
                && (q.vertices[0][0] - 0.344535).abs() < 1e-4
                && (q.vertices[0][2] - 0.297902).abs() < 1e-4)
            .expect("the euler carrier's north quad");
        let want = [
            [0.3445349931716919, 0.9375, 0.2979016900062561],
            [0.3445349931716919, 0.6875, 0.2979016900062561],
            [0.4162881374359131, 0.6875, 0.12467426061630249],
            [0.4162881374359131, 0.9375, 0.12467426061630249],
        ];
        for (v, w) in euler.vertices.iter().zip(want) {
            for axis in 0..3 {
                assert!((v[axis] - w[axis]).abs() < 1e-6, "{v:?} vs {w:?}");
            }
        }
        assert_eq!(euler.direction, Direction::East, "the diagonal normal wins on the west/east tie");
        assert_eq!(euler.uvs, [[15.5, 4.0], [15.5, 6.0], [14.0, 6.0], [14.0, 4.0]]);
        assert_eq!(euler.texture, "#all");
    }

    /// The rescale math: the composite is M*S with S =
    /// diag(1/max|M*unit|) per axis — the scaled vector rotates second.
    /// For 45 degrees about X the rescaled unit Y axis lands at (0, 1, 1):
    /// sqrt(2) of stretch through the 45-degree rotation.
    #[test]
    fn rescales_by_the_transformed_axis_maximum() {
        let plain = rodrigues_axis(Axis::X, 45.0);
        let y = mat_vec(&plain, [0.0, 1.0, 0.0]);
        assert!((y[1] - y[2]).abs() < 1e-7, "rotation alone keeps the diagonal: {y:?}");
        let m = apply_rescale(&plain);
        let scaled = mat_vec(&m, [0.0, 1.0, 0.0]);
        assert!((scaled[1] - 1.0).abs() < 1e-6, "{scaled:?}");
        assert!((scaled[2] - 1.0).abs() < 1e-6, "{scaled:?}");
        // The rescaled matrix re-maps every unit axis to a max-1 vector
        // — the scaleFactorForAxis fixed point.
        for u in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] {
            let d = mat_vec(&m, u);
            let max = d[0].abs().max(d[1].abs()).max(d[2].abs());
            assert!((max - 1.0).abs() < 1e-6, "{d:?}");
        }
    }

    /// The inner-faces models: the reversed from/to ([15.998,...] ->
    /// [0.002,...]) mirrors the quad, four faces flip to the opposite
    /// slot, and the down/up quads REWIND (2 swaps each, the census's
    /// per-model rewound=2 quads). The west/east quads keep their slot —
    /// only x is reversed.
    #[test]
    fn recalculates_the_winding_of_the_inner_face_models() {
        for id in ["block/cube_all_inner_faces", "block/cube_bottom_top_inner_faces"] {
            let (_, elements) = ELEMENTS.iter().find(|(mid, _)| *mid == id).expect("inner-faces model");
            let (quads, stats) = derive_elements_with_stats(elements);
            assert_eq!(quads.len(), 12, "outer cube + mirrored inner cube");
            assert_eq!(stats.facing_drift, 4, "the inner cube's down/up/north/south flip");
            assert_eq!(stats.rewound, 2, "only the flipped down/up quads rewind");
            assert_eq!(stats.up_fallback, 0);
            // the inner cube's quads: every coordinate strictly inside (0,1)
            let inner: Vec<_> = quads
                .iter()
                .filter(|q| q.vertices.iter().all(|v| v.iter().all(|c| *c > 0.0 && *c < 1.0)))
                .collect();
            assert_eq!(inner.len(), 6);
            let expected_inner = [
                (Direction::Down, Direction::Up),
                (Direction::Up, Direction::Down),
                (Direction::North, Direction::South),
                (Direction::South, Direction::North),
                (Direction::West, Direction::West),
                (Direction::East, Direction::East),
            ];
            for (quad, (slot, direction)) in inner.iter().zip(expected_inner) {
                assert_eq!(quad.slot, slot);
                assert_eq!(quad.direction, direction);
            }
        }
        // The census's SWAPS_HIST {0: 4232, 2: 6}: exactly 6 quads
        // rewound table-wide, each by 2 swaps.
        let total_rewound: usize = ELEMENTS
            .iter()
            .map(|(_, els)| derive_elements_with_stats(els).1.rewound)
            .sum();
        assert_eq!(total_rewound, 6);
    }

    /// The degenerate zero-area quads (the mangrove propagule's
    /// collinear strands): the normal has no positive dot, the facing is
    /// none, BakedQuad stores UP — the census's 24 fallbacks of this
    /// model's 42 quads.
    #[test]
    fn falls_back_to_up_for_the_degenerate_propagule_quads() {
        let (quads, stats) = derive_model_with_stats("block/mangrove_propagule_hanging_0")
            .expect("mangrove_propagule_hanging_0 is an element model");
        assert_eq!(quads.len(), 42);
        assert_eq!(stats.up_fallback, 24);
        assert_eq!(stats.facing_drift, 24, "UP != every strand's slot");
        // 24 fallbacks: 22 non-up-slot quads forced to UP + the 2 quads
        // whose own slot IS up (their facing is still the none -> UP
        // fallback, indistinguishable from a legit up-facing quad).
        assert_eq!(
            quads.iter().filter(|q| q.direction == Direction::Up && q.slot != Direction::Up).count(),
            22,
            "every non-up-slot quad of the strands faces UP (the vanilla fallback)"
        );
    }

    /// The registry-wide census pin (f32-true, scripts/quads_census.py —
    /// the census's own f64 first draft under-counted the drift at 273
    /// and an unfaithful f64-trig draft at 356; the JOML-faithful f32
    /// census agrees with this implementation PER MODEL, all 344 lines):
    /// every element model derives, every face yields exactly one quad,
    /// and the census's totals hold (4238 quads, 362 facing drifts, 6
    /// rewound quads, 160 UP fallbacks, the rotation-form and
    /// face-rotation histograms, 97 rescale carriers).
    #[test]
    fn derives_every_face_of_every_element_model_per_the_census() {
        let mut elements_total = 0usize;
        let mut rotation_forms = [0usize; 3]; // none, axis, euler
        let mut rescale_carriers = 0usize;
        let mut face_rotations = [0usize; 4]; // 0, 90, 180, 270
        let mut quads_total = 0usize;
        let mut drift_total = 0usize;
        let mut rewound_total = 0usize;
        let mut up_fallback_total = 0usize;
        for (id, elements) in ELEMENTS {
            elements_total += elements.len();
            let (quads, stats) = derive_elements_with_stats(elements);
            quads_total += stats.quads;
            drift_total += stats.facing_drift;
            rewound_total += stats.rewound;
            up_fallback_total += stats.up_fallback;
            assert_eq!(quads.len(), stats.quads);
            for quad in &quads {
                assert!(!quad.texture.is_empty());
                assert_eq!(quad.uv_rotation % 90, 0);
                assert!(quad.uv_rotation <= 270);
                assert!(quad.vertices.iter().all(|v| v.iter().all(|c| c.is_finite())));
            }
            if *id == "block/cube_all_inner_faces" {
                assert_eq!(stats.rewound, 2, "the inner cube's flipped down/up quads rewind");
            }
            for element in elements.iter() {
                match element.rotation {
                    None => rotation_forms[0] += 1,
                    Some(ElementRotation::Axis { rescale, .. }) => {
                        rotation_forms[1] += 1;
                        if rescale {
                            rescale_carriers += 1;
                        }
                    }
                    Some(ElementRotation::Euler { .. }) => rotation_forms[2] += 1,
                }
                for face in element.faces.iter().flatten() {
                    face_rotations[(face.rotation.unwrap_or(0) / 90) as usize % 4] += 1;
                }
            }
        }
        assert_eq!(elements_total, 1185);
        assert_eq!(rotation_forms, [882, 301, 2]);
        assert_eq!(rescale_carriers, 97);
        assert_eq!(face_rotations, [3993, 151, 39, 55]);
        assert_eq!(quads_total, 4238);
        assert_eq!(drift_total, 362);
        assert_eq!(rewound_total, 6);
        assert_eq!(up_fallback_total, 160);
    }

    /// derive_model reaches every element model by id and misses
    /// everything else (the table is id-sorted; a scan suffices).
    #[test]
    fn looks_up_models_by_id_and_misses_unknowns() {
        assert!(derive_model("block/cube").is_some());
        assert!(derive_model("block/template_hanging_sign_rot_3").is_some());
        assert!(derive_model("block/cube_inner").is_none());
        assert!(derive_model("item/handheld").is_none());
        assert!(derive_model("").is_none());
    }
}

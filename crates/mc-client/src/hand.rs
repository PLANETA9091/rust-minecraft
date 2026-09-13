//! The first-person hand: the empty main-hand arm, rendered from the
//! official player skin (wide Steve). Every constant below is 26.2
//! bytecode evidence — no invented numbers:
//!
//! - `ItemInHandRenderer.renderPlayerArm` (javap): the rest pose for an
//!   empty hand is `translate(side*0.64000005, -0.6, -0.71999997)`,
//!   `rotY(side*45°)`, then (after the zero swing terms)
//!   `translate(side*-1, 3.6, 3.5)`, `rotZ(side*120°)`, `rotX(200°)`,
//!   `rotY(side*-135°)`, `translate(side*5.6, 0, 0)`.
//! - `GameRenderer.renderItemInHand` (javap): the hand pose stack is
//!   multiplied by the camera rotation's INVERSE — hand space IS view
//!   space (+x right, +y up, -z forward), view matrix = identity.
//! - `AvatarRenderer.renderHand` (javap): the arm part gets
//!   `resetPose()` + `rightArm.zRot = 0.1` (radians) and submits with
//!   `RenderTypes.entityTranslucent` (two-sided — no backface cull).
//! - `HumanoidModel.createMesh` (javap): `right_arm` = texOffs(40, 16),
//!   `addBox(-3, -2, -2, 4, 12, 4)`, pivot `(-5, 2, 0)` (model units,
//!   /16 at render — `ModelPart$Vertex.worldX/Y/Z`).
//! - `ModelPart$Cube` + `$Polygon` (javap): the six faces' corner
//!   orders and their UV-rect corners (below, verbatim).
//!
//! The skin pixels stream from the runtime asset root and are
//! sha1-verified against the generated `skin` table before a single
//! texel is trusted; an absent or mismatched file renders NOTHING
//! (the reference renderer must not lie about its data).

use std::path::Path;

use anyhow::Context;
use mc_registry::generated::skin;

use crate::math::Mat4;

/// One view-space textured quad of the arm box (the model corner order
/// and uv assignment are `ModelPart$Cube`'s, verbatim).
pub struct HandQuad {
    pub pts: [[f32; 3]; 4],
    /// texel coords into the 64x64 skin (u right, v down)
    pub uv: [[f32; 2]; 4],
}

pub struct ArmModel {
    /// decoded RGBA8 (64*64*4), the runtime file's verbatim pixels
    rgba: Vec<u8>,
    quads: Vec<HandQuad>,
}

/// Load + sha1-verify the skin from the runtime asset root, then build
/// the vanilla arm geometry. `None` = not drawn.
pub fn load(root: &Path) -> Option<ArmModel> {
    let bytes = std::fs::read(root.join(skin::RUNTIME_PATH)).ok()?;
    let digest = sha1::Sha1::digest(&bytes);
    if hex::encode(digest) != skin::STEVE_WIDE_PNG_SHA1 {
        return None;
    }
    let (w, h, rgba) = decode_rgba(&bytes).ok()?;
    if w != skin::SKIN_SIDE
        || h != skin::SKIN_SIDE
        || rgba.len() != (skin::SKIN_SIDE as usize) * (skin::SKIN_SIDE as usize) * 4
    {
        return None;
    }
    Some(ArmModel {
        rgba,
        quads: build_quads(),
    })
}

impl ArmModel {
    /// The hand pass: clear depth (vanilla clears depth between the level
    /// and the hand) and draw the arm in view space under the world's
    /// projection (same fov — vanilla's hand fov follows the world's).
    pub fn draw(&self, fb: &mut crate::raster::Framebuf, proj: Mat4) {
        fb.clear_depth();
        fb.draw_hand_quads(proj, &self.quads, &self.rgba, skin::SKIN_SIDE as usize);
    }
}

fn decode_rgba(bytes: &[u8]) -> anyhow::Result<(u32, u32, Vec<u8>)> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().context("skin png read_info")?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).context("skin png decode")?;
    let (w, h) = (info.width, info.height);
    let samples = info.color_type.samples();
    let mut rgba = vec![255u8; (w * h * 4) as usize];
    for p in 0..(w * h) as usize {
        let s = &buf[p * samples..p * samples + samples];
        let o = p * 4;
        match samples {
            4 => rgba[o..o + 4].copy_from_slice(s),
            3 => {
                rgba[o] = s[0];
                rgba[o + 1] = s[1];
                rgba[o + 2] = s[2];
            }
            _ => {
                rgba[o] = s[0];
                rgba[o + 1] = s[0];
                rgba[o + 2] = s[0];
            }
        }
    }
    Ok((w, h, rgba))
}

/// The rest-pose transform (see the module docs for the bytecode source).
/// `side` is +1 for the RIGHT main hand (the default offline player's).
fn arm_transform() -> Mat4 {
    let side = 1.0f32;
    let t = |x: f32, y: f32, z: f32| Mat4::translation(x, y, z);
    let deg = |d: f32| d.to_radians();
    Mat4::IDENT
        .mul(&t(side * 0.640_000_05, -0.6, -0.719_999_97))
        .mul(&Mat4::rot_y(deg(side * 45.0)))
        .mul(&t(-side, 3.6, 3.5))
        .mul(&Mat4::rot_z(deg(side * 120.0)))
        .mul(&Mat4::rot_x(deg(200.0)))
        .mul(&Mat4::rot_y(deg(side * -135.0)))
        .mul(&t(side * 5.6, 0.0, 0.0))
        // the PlayerModel right_arm part: pivot (-5, 2, 0)/16, zRot 0.1 rad
        .mul(&t(-5.0 / 16.0, 2.0 / 16.0, 0.0))
        .mul(&Mat4::rot_z(0.1))
}

/// Build the six hand quads. The corner/uv pairs are the javap decode of
/// `ModelPart$Cube`: DOWN = (x1,y0,z1),(x0,y0,z1),(x0,y0,z0),(x1,y0,z0)
/// over rect (u+d, v); UP = (x1,y1,z0),(x0,y1,z0),(x0,y1,z1),(x1,y1,z1)
/// over rect (u+d+w, v); WEST = (x0,y0,z0),(x0,y0,z1),(x0,y1,z1),
/// (x0,y1,z0) over (u, v+d); NORTH = (x1,y0,z0),(x0,y0,z0),(x0,y1,z0),
/// (x1,y1,z0) over (u+d, v+d); EAST = (x1,y0,z1),(x1,y0,z0),(x1,y1,z0),
/// (x1,y1,z1) over (u+d+w, v+d); SOUTH = (x0,y0,z1),(x1,y0,z1),(x1,y1,z1),
/// (x0,y1,z1) over (u+d+w+d, v+d). Each face's corner k takes the
/// Polygon constructor's fixed rect corner: k=0 -> (u1,v0), 1 -> (u0,v0),
/// 2 -> (u0,v1), 3 -> (u1,v1).
fn build_quads() -> Vec<HandQuad> {
    let (u0, v0) = skin::ARM_TEX_OFFS;
    let [w, h, d] = skin::ARM_BOX_PX;
    let (u0, v0, w, h, d) = (u0 as f32, v0 as f32, w as f32, h as f32, d as f32);
    // model-space box bounds (addBox), /16 at transform time
    let (x0, x1) = (-3.0f32, -3.0 + w);
    let (y0, y1) = (-2.0f32, -2.0 + h);
    let (z0, z1) = (-2.0f32, -2.0 + d);
    let m = arm_transform();
    let px = |p: [f32; 3]| {
        let q = m.transform_point(crate::math::Vec3::new(p[0] / 16.0, p[1] / 16.0, p[2] / 16.0));
        [q[0], q[1], q[2]]
    };
    let faces: [(&[[f32; 3]; 4], (f32, f32), (f32, f32)); 6] = [
        (
            &[[x1, y0, z1], [x0, y0, z1], [x0, y0, z0], [x1, y0, z0]],
            (u0 + d, v0),
            (w, d),
        ), // DOWN  (the arm's shoulder end) — rect (u+d, v)
        (
            &[[x1, y1, z0], [x0, y1, z0], [x0, y1, z1], [x1, y1, z1]],
            (u0 + d + w, v0 + d),
            (w, d),
        ), // UP    (the arm's wrist end) — rect (u+d+w, v) V-FLIPPED
           // (the Polygon call passes v0 = the strip's LOWER edge)
        (
            &[[x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0]],
            (u0, v0 + d),
            (d, h),
        ), // WEST  (outer) — rect (u, v+d)
        (
            &[[x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0]],
            (u0 + d, v0 + d),
            (w, h),
        ), // NORTH — rect (u+d, v+d)
        (
            &[[x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1]],
            (u0 + d + w, v0 + d),
            (d, h),
        ), // EAST  (inner) — rect (u+d+w, v+d)
        (
            &[[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]],
            (u0 + d + w + d, v0 + d),
            (w, h),
        ), // SOUTH — rect (u+d+w+d, v+d)
    ];
    let mut quads = Vec::with_capacity(6);
    for (corners, (ru, rv), (rw, rh)) in faces {
        let (ru1, rv1) = (ru + rw, rv + rh);
        let uv = [[ru1, rv], [ru, rv], [ru, rv1], [ru1, rv1]];
        quads.push(HandQuad {
            pts: [
                px(corners[0]),
                px(corners[1]),
                px(corners[2]),
                px(corners[3]),
            ],
            uv,
        });
    }
    quads
}

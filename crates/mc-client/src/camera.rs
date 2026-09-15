//! Fly/orbit camera producing the view matrix, and the F5 perspective
//! cycle (the vanilla `CameraType` + `Camera.setup`'s detached branch).

use crate::math::{Mat4, Vec3};

/// The vanilla `CameraType` (javap): the three constants ride
/// `(firstPerson, mirrored)` flags — FIRST_PERSON(true, false),
/// THIRD_PERSON_BACK(false, false), THIRD_PERSON_FRONT(false, true) —
/// and `cycle()` walks `VALUES[(ordinal + 1) % VALUES.length]`, the F5
/// key's own loop (`Minecraft.handleKeybinds`:
/// `keyTogglePerspective.consumeClick()` → `options.setCameraType(cam.cycle())`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraType {
    #[default]
    FirstPerson,
    ThirdPersonBack,
    ThirdPersonFront,
}

impl CameraType {
    /// `CameraType.isFirstPerson()`.
    pub fn is_first_person(&self) -> bool {
        matches!(self, CameraType::FirstPerson)
    }

    /// `CameraType.isMirrored()`.
    pub fn is_mirrored(&self) -> bool {
        matches!(self, CameraType::ThirdPersonFront)
    }

    /// `CameraType.cycle()` — the VALUES declaration order is the
    /// `$values()` bytecode's own: FIRST_PERSON, THIRD_PERSON_BACK,
    /// THIRD_PERSON_FRONT.
    pub fn cycle(&self) -> CameraType {
        match self {
            CameraType::FirstPerson => CameraType::ThirdPersonBack,
            CameraType::ThirdPersonBack => CameraType::ThirdPersonFront,
            CameraType::ThirdPersonFront => CameraType::FirstPerson,
        }
    }
}

/// `Camera.DEFAULT_CAMERA_DISTANCE` (javap: the private constant the
/// detached branch feeds `getMaxZoom`; the LivingEntity branch takes
/// `max(scale * CAMERA_DISTANCE_attribute, vehicle_terms)` — the
/// player's attribute default is 4.0 and this client has no passenger
/// lane, so the default IS the law).
pub const DEFAULT_CAMERA_DISTANCE: f32 = 4.0;

/// The `Camera.getMaxZoom(desired)` bytecode, verbatim: 8 sequential
/// raycasts (i = 0..8) whose starts sit at the cube corners of
/// `position ± 0.1` (the `(i&1)*2-1` fold per axis, x then y then z),
/// each from that offset start toward `start + forwards * (-zoom)` —
/// the zoom SHRINKS as the loop goes (the later corners raycast to the
/// already-shrunk distance). A hit closer than the zoom narrows it to
/// the hit's distance FROM THE EYE (`hit.getLocation().distanceToSqr(position)`,
/// not from the offset start). The clip callback speaks `Level.clip`'s
/// verdict for one ray: `Some(hit_point)` = the first solid boundary.
pub fn max_zoom(
    eye: Vec3,
    forwards: Vec3,
    mut desired: f32,
    clip: &mut dyn FnMut(Vec3, Vec3) -> Option<Vec3>,
) -> f32 {
    for i in 0..8i32 {
        let corner = |bits: i32| ((bits & 1) * 2 - 1) as f32 * 0.1;
        let off = Vec3::new(corner(i), corner(i >> 1), corner(i >> 2));
        let start = eye.add(off);
        let end = start.add(forwards.scale(-desired));
        if let Some(hit) = clip(start, end) {
            let dist_sq = (hit.x - eye.x) * (hit.x - eye.x)
                + (hit.y - eye.y) * (hit.y - eye.y)
                + (hit.z - eye.z) * (hit.z - eye.z);
            if dist_sq < desired * desired {
                desired = dist_sq.sqrt();
            }
        }
    }
    desired
}

/// `Camera.setup`'s detached branch (the F5 third-person law), verbatim:
/// the mirrored type first FLIPS the rotation — `setRotation(yRot + 180,
/// -xRot)` — then the camera `move(-getMaxZoom(4.0), 0, 0)` backs off
/// along its own view. The `move(dLeft, dUp, dForward)` bytecode rotates
/// the local `(dForward, dUp, -dLeft)` vector by the camera rotation, so
/// `move(-z, 0, 0)` walks the camera BACKWARD along its forwards by
/// exactly the zoom: the back view sits behind the eye, the front view
/// (whose rotation already flipped) sits ahead, looking at the face.
pub fn detached(
    base: Camera,
    mirrored: bool,
    clip: &mut dyn FnMut(Vec3, Vec3) -> Option<Vec3>,
) -> Camera {
    let rot = if mirrored {
        Camera {
            pos: base.pos,
            yaw: base.yaw + std::f32::consts::PI,
            pitch: -base.pitch,
        }
    } else {
        base
    };
    let forwards = rot.forward();
    let zoom = max_zoom(rot.pos, forwards, DEFAULT_CAMERA_DISTANCE, clip);
    Camera {
        pos: rot.pos.add(forwards.scale(-zoom)),
        yaw: rot.yaw,
        pitch: rot.pitch,
    }
}

/// A plain pose value — the join result ships one to the frame thread
/// (the teleport's exact look direction), so derives follow the data.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub pos: Vec3,
    /// Radians, 0 = looking toward -Z.
    pub yaw: f32,
    /// Radians, positive = looking up.
    pub pitch: f32,
}

impl Camera {
    pub fn forward(&self) -> Vec3 {
        let cp = self.pitch.cos();
        Vec3::new(self.yaw.sin() * cp, self.pitch.sin(), -self.yaw.cos() * cp)
    }

    pub fn view(&self) -> Mat4 {
        let eye = self.pos;
        let center = eye.add(self.forward());
        Mat4::look_at(eye, center, Vec3::new(0.0, 1.0, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The F5 cycle: first person -> back -> front -> first (the
    /// CameraType.cycle bytecode's own VALUES walk).
    #[test]
    fn the_perspective_cycle_walks_the_three_views() {
        assert_eq!(CameraType::FirstPerson.cycle(), CameraType::ThirdPersonBack);
        assert_eq!(
            CameraType::ThirdPersonBack.cycle(),
            CameraType::ThirdPersonFront
        );
        assert_eq!(
            CameraType::ThirdPersonFront.cycle(),
            CameraType::FirstPerson
        );
        assert!(CameraType::FirstPerson.is_first_person());
        assert!(!CameraType::ThirdPersonBack.is_first_person());
        assert!(CameraType::ThirdPersonFront.is_mirrored());
        assert!(!CameraType::ThirdPersonBack.is_mirrored());
    }

    /// The open-field detach: the back view sits exactly 4.0 blocks
    /// behind the eye along the view direction (the getMaxZoom's
    /// MISS lane — no clip shrinks the zoom).
    #[test]
    fn the_back_view_sits_the_full_zoom_behind() {
        let base = Camera {
            pos: Vec3::new(8.5, 1.62, 8.5),
            yaw: std::f32::consts::FRAC_PI_2, // facing +X (east)
            pitch: 0.0,
        };
        let mut clip_called = 0;
        let cam = detached(base, false, &mut |_f, _t| {
            clip_called += 1;
            None
        });
        assert_eq!(clip_called, 8, "the eight corner raycasts");
        let fwd = base.forward();
        let want = base.pos.add(fwd.scale(-4.0));
        assert!((cam.pos.x - want.x).abs() < 1e-4);
        assert!((cam.pos.y - want.y).abs() < 1e-4);
        assert!((cam.pos.z - want.z).abs() < 1e-4);
        assert!((cam.yaw - base.yaw).abs() < 1e-6, "the back view keeps the yaw");
    }

    /// The mirrored detach: the rotation flips (yaw + π, pitch negated)
    /// BEFORE the move — the camera ends up AHEAD of the eye looking
    /// back at the face.
    #[test]
    fn the_front_view_flips_then_sits_ahead() {
        let base = Camera {
            pos: Vec3::new(8.5, 1.62, 8.5),
            yaw: 0.0,
            pitch: 0.3,
        };
        let cam = detached(base, true, &mut |_f, _t| None);
        let fwd = base.forward();
        let want = base.pos.add(fwd.scale(4.0)); // -(-zoom) along the OLD forward
        assert!((cam.pos.x - want.x).abs() < 1e-4);
        assert!((cam.pos.y - want.y).abs() < 1e-4);
        assert!((cam.pos.z - want.z).abs() < 1e-4);
        assert!((cam.pitch - (-0.3)).abs() < 1e-6, "the pitch negates");
        assert!((cam.yaw - (base.yaw + std::f32::consts::PI)).abs() < 1e-6);
    }

    /// The clip shrink: a wall two-and-a-half blocks behind the eye
    /// narrows the zoom to the hit's distance from the EYE (the
    /// getMaxZoom fold over the real voxel clip — the 0.1 corner
    /// offsets keep every hit |Δy| = |Δz| = 0.1 from the eye, the sqrt
    /// fold's own inflation).
    #[test]
    fn the_wall_shrinks_the_zoom_to_the_hit() {
        let mut w = crate::world::World::new(1, 42);
        // a flat open world: every cell AIR, then the wall column at
        // x=11 (cells [11,12)) — BEHIND the eye, which faces -X
        for x in 0..16i32 {
            for z in 0..16i32 {
                for y in 0..crate::world::CHUNK_Y {
                    w.set(x, y as i32, z, crate::world::AIR);
                }
            }
        }
        for z in 0..16i32 {
            for y in 0..8 {
                w.set(11, y, z, crate::world::STONE);
            }
        }
        let base = Camera {
            pos: Vec3::new(8.5, 1.62, 8.5),
            yaw: -std::f32::consts::FRAC_PI_2, // facing -X (west)
            pitch: 0.0,
        };
        let cam = detached(base, false, &mut |f, t| crate::body::clip_ray(&w, f, t));
        // the wall's -X face at x=11.0: every corner ray crosses it
        // with |Δx| = 2.5, |Δy| = |Δz| = 0.1 (the 0.1 offsets ride the
        // START, the axis-parallel ray keeps them in the hit)
        let want_zoom = ((11.0f32 - 8.5) * (11.0 - 8.5) + 0.1 * 0.1 + 0.1 * 0.1).sqrt();
        let fwd = base.forward();
        let want = base.pos.add(fwd.scale(-want_zoom));
        assert!(
            (cam.pos.x - want.x).abs() < 1e-3,
            "zoom shrunk to the wall: {} vs {}",
            cam.pos.x,
            want.x
        );
        assert!((cam.pos.y - base.pos.y).abs() < 1e-6);
        assert!((cam.pos.z - base.pos.z).abs() < 1e-6);
    }

    /// The max_zoom fold alone: a MISS keeps the desired, a hit closer
    /// than the desired shrinks it to the hit's EYE distance, a hit
    /// farther leaves the zoom alone (the fixed hit point speaks the
    /// clip verdict's own shape).
    #[test]
    fn max_zoom_speaks_the_bytecode_arithmetic() {
        let eye = Vec3::new(0.0, 0.0, 0.0);
        let fwd = Vec3::new(1.0, 0.0, 0.0);
        let z = max_zoom(eye, fwd, 4.0, &mut |_f, _t| None);
        assert!((z - 4.0).abs() < 1e-6);
        // the fixed hit on the eye's axis at 1.5: every corner ray
        // returns the SAME point, the eye distance is 1.5
        let z = max_zoom(eye, fwd, 4.0, &mut |_f, _t| Some(Vec3::new(1.5, 0.0, 0.0)));
        assert!((z - 1.5).abs() < 1e-4);
        // a hit FARTHER than the desired leaves the zoom alone
        let z = max_zoom(eye, fwd, 4.0, &mut |_f, _t| Some(Vec3::new(9.0, 0.0, 0.0)));
        assert!((z - 4.0).abs() < 1e-6);
    }
}

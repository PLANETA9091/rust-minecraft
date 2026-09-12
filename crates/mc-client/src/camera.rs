//! Fly/orbit camera producing the view matrix.

use crate::math::{Mat4, Vec3};

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

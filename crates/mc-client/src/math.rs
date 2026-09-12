//! Deterministic 2D/3D math for the CPU reference renderer.
//! Hand-rolled on purpose: bit-stable across platforms for pixel tests.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn add(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }

    pub fn sub(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }

    pub fn scale(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(self, o: Vec3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        let l = self.length();
        if l > 1e-8 {
            self.scale(1.0 / l)
        } else {
            Vec3::ZERO
        }
    }
}

/// Column-major 4x4 matrix (OpenGL-style), applied as m * v.
#[derive(Debug, Clone, Copy)]
pub struct Mat4(pub [f32; 16]);

impl Mat4 {
    pub const IDENT: Mat4 = Mat4([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]);

    pub fn mul(&self, o: &Mat4) -> Mat4 {
        let a = &self.0;
        let b = &o.0;
        let mut r = [0f32; 16];
        for col in 0..4 {
            for row in 0..4 {
                let mut s = 0.0;
                for k in 0..4 {
                    s += a[k * 4 + row] * b[col * 4 + k];
                }
                r[col * 4 + row] = s;
            }
        }
        Mat4(r)
    }

    /// Right-handed look-at (like gluLookAt).
    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
        let f = center.sub(eye).normalized();
        let s = f.cross(up).normalized();
        let u = s.cross(f);
        Mat4([
            s.x, u.x, -f.x, 0.0, s.y, u.y, -f.y, 0.0, s.z, u.z, -f.z, 0.0, -s.dot(eye), -u.dot(eye),
            f.dot(eye), 1.0,
        ])
    }

    /// Right-handed perspective projection.
    pub fn perspective(fovy_rad: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        let f = 1.0 / (fovy_rad / 2.0).tan();
        let nf = 1.0 / (near - far);
        Mat4([
            f / aspect, 0.0, 0.0, 0.0, 0.0, f, 0.0, 0.0, 0.0, 0.0, (far + near) * nf, -1.0, 0.0,
            0.0, 2.0 * far * near * nf, 0.0,
        ])
    }

    /// Rotation about the X axis (radians), column-major.
    pub fn rot_x(rad: f32) -> Mat4 {
        let (s, c) = rad.sin_cos();
        Mat4([
            1.0, 0.0, 0.0, 0.0, 0.0, c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    /// Rotation about the Y axis (radians), column-major.
    pub fn rot_y(rad: f32) -> Mat4 {
        let (s, c) = rad.sin_cos();
        Mat4([
            c, 0.0, -s, 0.0, 0.0, 1.0, 0.0, 0.0, s, 0.0, c, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn transform_point(&self, p: Vec3) -> [f32; 4] {
        let m = &self.0;
        [
            m[0] * p.x + m[4] * p.y + m[8] * p.z + m[12],
            m[1] * p.x + m[5] * p.y + m[9] * p.z + m[13],
            m[2] * p.x + m[6] * p.y + m[10] * p.z + m[14],
            m[3] * p.x + m[7] * p.y + m[11] * p.z + m[15],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_at_identity_eye() {
        let m = Mat4::look_at(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0));
        let p = m.transform_point(Vec3::new(0.0, 0.0, 0.0));
        // point at origin is 5 units in front of the eye -> view z = -5, w = 1
        assert!((p[2] - (-5.0)).abs() < 1e-5, "z = {}", p[2]);
        assert!((p[3] - 1.0).abs() < 1e-5, "w = {}", p[3]);
    }

    #[test]
    fn vec_basics() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        assert!((a.cross(b).z - 1.0).abs() < 1e-6);
        assert!((a.dot(b)).abs() < 1e-6);
    }

    #[test]
    fn rotations_match_right_handed_convention() {
        // rot_x(+90°) maps +Y -> +Z.
        let p = Mat4::rot_x(std::f32::consts::FRAC_PI_2).transform_point(Vec3::new(0.0, 1.0, 0.0));
        assert!(p[1].abs() < 1e-6 && (p[2] - 1.0).abs() < 1e-6, "{p:?}");
        // rot_y(+90°) maps +Z -> +X.
        let p = Mat4::rot_y(std::f32::consts::FRAC_PI_2).transform_point(Vec3::new(0.0, 0.0, 1.0));
        assert!((p[0] - 1.0).abs() < 1e-6 && p[2].abs() < 1e-6, "{p:?}");
        // rot_y(-90°) maps -Z -> +X (the celestial frame: sun rises at +X).
        let p = Mat4::rot_y(-std::f32::consts::FRAC_PI_2)
            .transform_point(Vec3::new(0.0, 0.0, -1.0));
        assert!((p[0] - 1.0).abs() < 1e-6, "{p:?}");
    }
}

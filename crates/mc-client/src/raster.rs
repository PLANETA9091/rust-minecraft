//! Deterministic software rasterizer with z-buffer, near-plane clipping,
//! perspective-correct depth and distance fog. Bit-stable by construction —
//! this is the pixel-perfect reference every optimization must match.

use crate::math::{Mat4, Vec3};

pub struct Framebuf {
    pub w: usize,
    pub h: usize,
    /// RGB, 3 bytes per pixel.
    pub color: Vec<u8>,
    /// depth = view-space distance (positive); f32::MAX = clear.
    pub depth: Vec<f32>,
}

pub struct RenderCtx {
    pub view: Mat4,
    pub proj: Mat4,
    pub fog_color: [f32; 3],
    pub fog_near: f32,
    pub fog_far: f32,
}

impl Framebuf {
    pub fn new(w: usize, h: usize) -> Self {
        Self { w, h, color: vec![0u8; w * h * 3], depth: vec![f32::MAX; w * h] }
    }

    pub fn clear(&mut self, sky_top: [f32; 3], sky_bottom: [f32; 3]) {
        for y in 0..self.h {
            let t = y as f32 / (self.h.max(2) - 1) as f32;
            let c = [
                (sky_top[0] + (sky_bottom[0] - sky_top[0]) * t) * 255.0,
                (sky_top[1] + (sky_bottom[1] - sky_top[1]) * t) * 255.0,
                (sky_top[2] + (sky_bottom[2] - sky_top[2]) * t) * 255.0,
            ];
            let row = y * self.w * 3;
            for x in 0..self.w {
                let o = row + x * 3;
                self.color[o] = c[0] as u8;
                self.color[o + 1] = c[1] as u8;
                self.color[o + 2] = c[2] as u8;
            }
        }
        self.depth.iter_mut().for_each(|d| *d = f32::MAX);
    }

    /// Rasterize a batch of world-space quads.
    pub fn draw_quads(&mut self, ctx: &RenderCtx, quads: &[super::mesher::Quad]) {
        let vp = ctx.proj.mul(&ctx.view);
        let w_px = self.w as f32;
        let h_px = self.h as f32;
        for quad in quads {
            // ---- transform + near-plane clip (Sutherland–Hodgman in clip space) ----
            let clip: Vec<[f32; 4]> = quad.pts.iter().map(|p| vp.transform_point(Vec3::new(p[0], p[1], p[2]))).collect();
            let clipped = clip_near(&clip);
            if clipped.len() < 3 {
                continue;
            }
            // ---- project to screen ----
            let mut sx = vec![0f32; clipped.len()];
            let mut sy = vec![0f32; clipped.len()];
            let mut ow = vec![0f32; clipped.len()];
            let mut vz = vec![0f32; clipped.len()];
            let mut off_x = [0f32; 2] ;
            let _ = &mut off_x;
            let mut minx = f32::MAX;
            let mut miny = f32::MAX;
            let mut maxx = f32::MIN;
            let mut maxy = f32::MIN;
            for i in 0..clipped.len() {
                let c = clipped[i];
                let iw = 1.0 / c[3];
                let ndc_x = c[0] * iw;
                let ndc_y = c[1] * iw;
                sx[i] = (ndc_x * 0.5 + 0.5) * w_px;
                sy[i] = (1.0 - (ndc_y * 0.5 + 0.5)) * h_px;
                ow[i] = iw;
                vz[i] = c[3]; // view-space distance proxy = clip w
                minx = minx.min(sx[i]);
                miny = miny.min(sy[i]);
                maxx = maxx.max(sx[i]);
                maxy = maxy.max(sy[i]);
            }
            // ---- backface cull via signed area of first triangle ----
            let area = edge(sx[0], sy[0], sx[1], sy[1], sx[2], sy[2]);
            if area <= 0.0 {
                continue;
            }
            // ---- bbox ----
            let x0 = (minx.floor().max(0.0)) as isize;
            let y0 = (miny.floor().max(0.0)) as isize;
            let x1 = (maxx.ceil().min(w_px - 1.0)) as isize;
            let y1 = (maxy.ceil().min(h_px - 1.0)) as isize;
            if x1 < x0 || y1 < y0 {
                continue;
            }
            // ---- fan rasterization with perspective-correct depth ----
            for tri in 1..clipped.len() - 1 {
                let (ax, ay, a_ow, a_vz) = (sx[0], sy[0], ow[0], vz[0]);
                let (bx, by, b_ow, b_vz) = (sx[tri], sy[tri], ow[tri], vz[tri]);
                let (cx, cy, c_ow, c_vz) = (sx[tri + 1], sy[tri + 1], ow[tri + 1], vz[tri + 1]);
                let area_ab = edge(ax, ay, bx, by, cx, cy);
                if area_ab.abs() < 1e-9 {
                    continue;
                }
                let inv_area = 1.0 / area_ab;
                for py in y0..=y1 {
                    for px in x0..=x1 {
                        let fx = px as f32 + 0.5;
                        let fy = py as f32 + 0.5;
                        let w0 = edge(bx, by, cx, cy, fx, fy);
                        let w1 = edge(cx, cy, ax, ay, fx, fy);
                        let w2 = edge(ax, ay, bx, by, fx, fy);
                        if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                            continue;
                        }
                        let b0 = w0 * inv_area;
                        let b1 = w1 * inv_area;
                        let b2 = w2 * inv_area;
                        // perspective-correct interpolation of view distance
                        let dist = 1.0 / (b0 * a_ow + b1 * b_ow + b2 * c_ow);
                        let z = (b0 * a_vz * a_ow + b1 * b_vz * b_ow + b2 * c_vz * c_ow) * dist;
                        let idx = (py as usize) * self.w + px as usize;
                        if z < self.depth[idx] {
                            self.depth[idx] = z;
                            let fog = ((z - ctx.fog_near) / (ctx.fog_far - ctx.fog_near))
                                .clamp(0.0, 1.0);
                            let o = idx * 3;
                            for ch in 0..3 {
                                let c0 = quad.color[ch];
                                let c = c0 + (ctx.fog_color[ch] - c0) * fog;
                                self.color[o + ch] = (c.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Convert to 0xAARRGGBB buffer (minifb layout).
    pub fn to_argb(&self) -> Vec<u32> {
        self.color
            .chunks_exact(3)
            .map(|c| 0xFF00_0000 | ((c[0] as u32) << 16) | ((c[1] as u32) << 8) | c[2] as u32)
            .collect()
    }
}

fn edge(ax: f32, ay: f32, bx: f32, by: f32, px: f32, py: f32) -> f32 {
    (px - ax) * (by - ay) - (py - ay) * (bx - ax)
}

/// Clip a clip-space polygon against w >= eps.
fn clip_near(poly: &[[f32; 4]]) -> Vec<[f32; 4]> {
    const EPS: f32 = 1e-3;
    let mut out = Vec::with_capacity(poly.len() + 2);
    let n = poly.len();
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        let a_in = a[3] >= EPS;
        let b_in = b[3] >= EPS;
        if a_in {
            out.push(a);
        }
        if a_in != b_in {
            let t = (EPS - a[3]) / (b[3] - a[3]);
            out.push([
                a[0] + (b[0] - a[0]) * t,
                a[1] + (b[1] - a[1]) * t,
                a[2] + (b[2] - a[2]) * t,
                EPS,
            ]);
        }
    }
    out
}

#[test]
fn near_clip_square() {
    let square = [
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
    ];
    assert_eq!(clip_near(&square).len(), 4);
    let crossing = [
        [-2.0, 0.0, 0.0, 2.0],
        [2.0, 0.0, 0.0, 2.0],
        [2.0, 1.0, 0.0, -2.0],
        [-2.0, 1.0, 0.0, -2.0],
    ];
    let out = clip_near(&crossing);
    assert!(out.len() >= 3);
    out.iter().for_each(|p| assert!(p[3] >= 1e-3 - 1e-6));
}

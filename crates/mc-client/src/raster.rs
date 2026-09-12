//! Deterministic software rasterizer with z-buffer, near-plane clipping,
//! perspective-correct depth and distance fog. Bit-stable by construction —
//! this is the pixel-perfect reference every optimization must match.

use crate::math::{Mat4, Vec3};
use crate::textures;
use mc_registry::generated::atlas;

/// Vanilla-style water translucency (surface alpha over the background).
pub const WATER_OPACITY: f32 = 0.80;

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

/// Blend function for color-only FlatQuads (official pipeline semantics).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlatBlend {
    /// BlendFunction.OVERLAY (SRC_ALPHA, ONE): out = rgb·a + dst — STARS,
    /// and textured sun/moon.
    Additive,
    /// BlendFunction.TRANSLUCENT (SRC_ALPHA, ONE_MINUS_SRC_ALPHA):
    /// out = rgb·a + dst·(1-a) — the sunrise/sunset glow disc.
    Translucent,
}

/// One celestial quad (sun / moon / star / sunrise disc) in world space.
/// Rendered with the official sky-pipeline semantics: no depth test or
/// write, no fog, no backface cull — drawn right after the sky-color clear,
/// before terrain, so terrain always overdraws it.
pub struct FlatQuad {
    pub pts: [[f32; 3]; 4],
    /// Per-vertex RGBA (the game interpolates vertex color in screen space;
    /// the sunrise fan uses alpha 1 at the center falling to 0 on the ring).
    pub colors: [[f32; 4]; 4],
    /// (rgba pixels, side length) for textured celestial sprites (sun/moon);
    /// None = solid vertex-color quad.
    pub rgba: Option<(&'static [u8], u32)>,
    pub blend: FlatBlend,
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

    /// Rasterize a batch of world-space quads. Opaque quads first, then
    /// translucent (water) — the translucent pass blends over the framebuffer.
    pub fn draw_quads(&mut self, ctx: &RenderCtx, quads: &[super::mesher::Quad]) {
        self.draw_pass(ctx, quads, false);
        self.draw_pass(ctx, quads, true);
    }

    fn draw_pass(&mut self, ctx: &RenderCtx, quads: &[super::mesher::Quad], translucent: bool) {
        let vp = ctx.proj.mul(&ctx.view);
        let w_px = self.w as f32;
        let h_px = self.h as f32;
        for quad in quads {
            if quad.translucent != translucent {
                continue;
            }
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
            // ---- fan rasterization with perspective-correct depth + UV and
            // per-vertex color (smooth lighting) ----
            let textured = quad.tile != atlas::TILE_NONE && atlas::has_tiles();
            for tri in 1..clipped.len() - 1 {
                let (ax, ay, a_ow, a_vz) = (sx[0], sy[0], ow[0], vz[0]);
                let (bx, by, b_ow, b_vz) = (sx[tri], sy[tri], ow[tri], vz[tri]);
                let (cx, cy, c_ow, c_vz) = (sx[tri + 1], sy[tri + 1], ow[tri + 1], vz[tri + 1]);
                // near-clipping can emit 5-vertex polygons; vertex attributes
                // stay per-corner so the extra fan step reuses the last corner
                // (the case never triggers for terrain from the orbit/spawn
                // cameras, but must not panic)
                let (ca, cb, cc) = (
                    quad.colors[0],
                    quad.colors[tri.min(3)],
                    quad.colors[(tri + 1).min(3)],
                );
                let (ua, va) = (quad.uvs[0][0], quad.uvs[0][1]);
                let (ub, vb) = (quad.uvs[tri.min(3)][0], quad.uvs[tri.min(3)][1]);
                let (uc, vc) = (
                    quad.uvs[(tri + 1).min(3)][0],
                    quad.uvs[(tri + 1).min(3)][1],
                );
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
                            let fog = ((z - ctx.fog_near) / (ctx.fog_far - ctx.fog_near))
                                .clamp(0.0, 1.0);
                            let o = idx * 3;
                            // affine screen-space vertex-color interpolation
                            // (same as GPU linear interpolation across the quad)
                            let mut c0 = [0f32; 3];
                            for ch in 0..3 {
                                c0[ch] = b0 * ca[ch] + b1 * cb[ch] + b2 * cc[ch];
                            }
                            if textured {
                                // perspective-correct texture coordinates, nearest texel
                                let u = (b0 * ua * a_ow + b1 * ub * b_ow + b2 * uc * c_ow) * dist;
                                let v = (b0 * va * a_ow + b1 * vb * b_ow + b2 * vc * c_ow) * dist;
                                let texel = textures::sample_tile(quad.tile, u, v);
                                if texel[3] < 0.5 {
                                    continue; // cutout hole (leaves): no color, no depth
                                }
                                for ch in 0..3 {
                                    c0[ch] *= texel[ch];
                                }
                            }
                            if translucent {
                                // vanilla-style water translucency over background
                                let a = WATER_OPACITY;
                                for ch in 0..3 {
                                    let bg = self.color[o + ch] as f32 / 255.0;
                                    c0[ch] = bg * (1.0 - a) + c0[ch] * a;
                                }
                            }
                            self.depth[idx] = z;
                            for ch in 0..3 {
                                let c = c0[ch] + (ctx.fog_color[ch] - c0[ch]) * fog;
                                self.color[o + ch] = (c.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Rasterize celestial quads (sun/moon/stars): depthless, fogless,
    /// double-sided, drawn before terrain which overdraws them. Textured
    /// quads sample nearest with alpha cutout at 0.5 (official atlas
    /// behavior); color quads alpha-blend with alpha = color.r (official
    /// STARS setShaderColor + SRC_ALPHA blend).
    pub fn draw_flat(&mut self, ctx: &RenderCtx, quads: &[FlatQuad]) {
        let vp = ctx.proj.mul(&ctx.view);
        let w_px = self.w as f32;
        let h_px = self.h as f32;
        for quad in quads {
            let clip: Vec<[f32; 4]> =
                quad.pts.iter().map(|p| vp.transform_point(Vec3::new(p[0], p[1], p[2]))).collect();
            let clipped = clip_near(&clip);
            if clipped.len() < 3 {
                continue;
            }
            let mut sx = vec![0f32; clipped.len()];
            let mut sy = vec![0f32; clipped.len()];
            let mut ow = vec![0f32; clipped.len()];
            let mut minx = f32::MAX;
            let mut miny = f32::MAX;
            let mut maxx = f32::MIN;
            let mut maxy = f32::MIN;
            for i in 0..clipped.len() {
                let c = clipped[i];
                let iw = 1.0 / c[3];
                sx[i] = (c[0] * iw * 0.5 + 0.5) * w_px;
                sy[i] = (1.0 - (c[1] * iw * 0.5 + 0.5)) * h_px;
                ow[i] = iw;
                minx = minx.min(sx[i]);
                miny = miny.min(sy[i]);
                maxx = maxx.max(sx[i]);
                maxy = maxy.max(sy[i]);
            }
            // No backface cull: the sun/moon are seen from both sides of
            // their orbit plane (vanilla renders them double-sided).
            let x0 = (minx.floor().max(0.0)) as isize;
            let y0 = (miny.floor().max(0.0)) as isize;
            let x1 = (maxx.ceil().min(w_px - 1.0)) as isize;
            let y1 = (maxy.ceil().min(h_px - 1.0)) as isize;
            if x1 < x0 || y1 < y0 {
                continue;
            }
            for tri in 1..clipped.len() - 1 {
                let (ax, ay, a_ow) = (sx[0], sy[0], ow[0]);
                let (bx, by, b_ow) = (sx[tri], sy[tri], ow[tri]);
                let (cx, cy, c_ow) = (sx[tri + 1], sy[tri + 1], ow[tri + 1]);
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
                        let dist = 1.0 / (b0 * a_ow + b1 * b_ow + b2 * c_ow);
                        let o = ((py as usize) * self.w + px as usize) * 3;
                        let mut c0 = [0f32; 4];
                        let (ca, cb, cc) = (
                            quad.colors[0],
                            quad.colors[tri.min(3)],
                            quad.colors[(tri + 1).min(3)],
                        );
                        for ch in 0..4 {
                            c0[ch] = b0 * ca[ch] + b1 * cb[ch] + b2 * cc[ch];
                        }
                        if let Some((rgba, side)) = quad.rgba {
                            // perspective-correct nearest texel; corner UVs
                            // (0,0) (1,0) (1,1) (0,1) cover the full sprite
                            // (official buildCelestialQuad layout).
                            let uv = [[0.0f32, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
                            let (ub, vb) = (uv[tri][0], uv[tri][1]);
                            let (uc, vc) = (uv[tri + 1][0], uv[tri + 1][1]);
                            let u = (b0 * 0.0 * a_ow
                                + b1 * ub * b_ow
                                + b2 * uc * c_ow)
                                * dist;
                            let v = (b0 * 0.0 * a_ow
                                + b1 * vb * b_ow
                                + b2 * vc * c_ow)
                                * dist;
                            let s = side as f32;
                            let xi = ((u * s).floor() as isize).clamp(0, side as isize - 1) as usize;
                            let yi = ((v * s).floor() as isize).clamp(0, side as isize - 1) as usize;
                            let off = (yi * side as usize + xi) * 4;
                            let a = rgba[off + 3] as f32 / 255.0;
                            // Official CELESTIAL pipeline = BlendFunction.OVERLAY:
                            // additive color (src.rgb * src.a + dst) — the black
                            // surroundings of the sun/moon textures add nothing.
                            for ch in 0..3 {
                                let src = c0[ch] * (rgba[off + ch] as f32 / 255.0) * a;
                                let bg = self.color[o + ch] as f32 / 255.0;
                                c0[ch] = src + bg;
                            }
                        } else {
                            let bg = [
                                self.color[o] as f32 / 255.0,
                                self.color[o + 1] as f32 / 255.0,
                                self.color[o + 2] as f32 / 255.0,
                            ];
                            match quad.blend {
                                FlatBlend::Additive => {
                                    for ch in 0..3 {
                                        c0[ch] = c0[ch] * c0[3] + bg[ch];
                                    }
                                }
                                FlatBlend::Translucent => {
                                    for ch in 0..3 {
                                        c0[ch] = c0[ch] * c0[3] + bg[ch] * (1.0 - c0[3]);
                                    }
                                }
                            }
                        }
                        for ch in 0..3 {
                            self.color[o + ch] = (c0[ch].clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
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

fn edge(ax: f32, ay: f32, bx: f32, by: f32, px: f32, py: f32) -> f32 {
    (px - ax) * (by - ay) - (py - ay) * (bx - ax)
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

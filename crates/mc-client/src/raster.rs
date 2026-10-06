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
    /// THE FIRE ANIMATION'S OWN CLOCK (the vanilla TextureAtlasSprite
    /// animator's per-tick walk the 20-tps cadence speaks): the fire
    /// multipart's two frame families the `atlas::fire_anim_tile` remap
    /// answers at DRAW time — the strip's playlist position
    /// `(tick / frametime) % 32`, the mesher's baked base tile the
    /// remap never re-bakes. The 0 the golden/preview renders keep
    /// (the honest still the fresh atlas's first playlist row wears).
    pub fire_anim_tick: u64,
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

/// ONE DEBUG GIZMO LINE (the audited 26.2 Gizmos.line fold's own
/// tuple — the world-space endpoints, the rgb the ARGB constants
/// speak, the alpha the colorFromFloat keep, the always-on-top arm
/// the setAlwaysOnTop style flags). The `chunk_border_lines` builder
/// owns the geometry law, the `draw_chunk_borders` pass only
/// projects.
pub struct DebugLine {
    pub a: Vec3,
    pub b: Vec3,
    pub rgb: [f32; 3],
    pub alpha: f32,
    pub always_on_top: bool,
}

/// One debug POINT gizmo (javap 26.2 the `PointGizmo.emit` walk —
/// `GizmoPrimitives.addPoint(pos, color*alpha, size)` the F3+B
/// entity hitboxes' own white dot speaks: one at every entity's
/// lerped position, the `point(lerped, -1, 2.0F)` submit rides). The
/// draw the same pipeline the [`DebugLine`] wears — the projection,
/// the near-plane guard, the 2x2 plot the GL point's own ~2px look
/// proxies, the depth test the stroke's non-always-on-top style
/// keeps (`GizmoStyle.stroke`'s flag the `iconst_0` init banks).
pub struct DebugPoint {
    pub pos: Vec3,
    pub rgb: [f32; 3],
    pub alpha: f32,
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

/// One uniform-color cloud quad (bottom or side face of a cloud cell box).
/// Rendered with the official CLOUDS pipeline semantics (rendertype_clouds):
/// TRANSLUCENT blend, depth-tested + depth-written, per-pixel alpha fog
/// `a *= 1 - linear_fog_value(d, 0, FogCloudsEnd)`.
pub struct CloudQuad {
    pub pts: [[f32; 3]; 4],
    /// Face color * CloudColor (the uniform `vertexColor` of the shader).
    pub color: [f32; 4],
}

/// One view-space textured quad of the first-person hand pass (the
/// vanilla entity_translucent semantics: two-sided, depth-tested, no fog).
#[allow(dead_code)] // the hand pass's carrier — rides the held-item render round
pub struct HandQuad {
    pub pts: [[f32; 3]; 4],
    /// texel coords into the skin texture (u right, v down)
    pub uv: [[f32; 2]; 4],
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

    /// Depth-only clear — the vanilla hand pass runs after the level with a
    /// cleared depth buffer (the hand never clips into world geometry).
    pub fn clear_depth(&mut self) {
        self.depth.iter_mut().for_each(|d| *d = f32::MAX);
    }

    /// Rasterize a batch of world-space quads. Opaque quads first, then
    /// translucent (water) — the translucent pass blends over the framebuffer.
    pub fn draw_quads(&mut self, ctx: &RenderCtx, quads: &[super::mesher::Quad]) {
        self.draw_pass(ctx, quads, false, false);
        self.draw_pass(ctx, quads, true, false);
    }

    /// THE DIG LANE's breaking overlay — the destroy_stage crack quads
    /// alpha-blend over the terrain (the vanilla crumbling render type's
    /// own pass: depth-TESTED against the block it covers, never depth-
    /// written, the crack's transparent texels let the block through).
    pub fn draw_destroy_overlay(&mut self, ctx: &RenderCtx, quads: &[super::mesher::Quad]) {
        self.draw_pass(ctx, quads, false, true);
    }

    /// THE DEBUG LINE'S OWN CORE (the projected Bresenham walk the
    /// hit outline and the F3+G chunk borders share — the near-plane
    /// guard `pa[3] < 0.1`, the perspective divide, the 2x2 plot the
    /// vanilla's ~2.5px GL line width proxies, the alpha blend
    /// `bg*(1-a) + rgb*a` the hit outline's black fold rides verbatim
    /// (rgb = the zero vector speaks its old math exactly), the depth
    /// test the `<` pass keeps — the ALWAYS-ON-TOP arm the audited
    /// setAlwaysOnTop gizmo style (ChunkBorderRenderer's section
    /// cuboid, pcs 776-791) skips).
    fn draw_debug_line(
        &mut self,
        vp: &Mat4,
        a: Vec3,
        b: Vec3,
        rgb: [f32; 3],
        alpha: f32,
        always_on_top: bool,
    ) {
        let w_px = self.w as f32;
        let h_px = self.h as f32;
        let pa = vp.transform_point(a);
        let pb = vp.transform_point(b);
        // the near-plane guard: both endpoints must sit in front
        if pa[3] < 0.1 || pb[3] < 0.1 {
            return;
        }
        let ax = (pa[0] / pa[3] * 0.5 + 0.5) * w_px;
        let ay = (1.0 - (pa[1] / pa[3] * 0.5 + 0.5)) * h_px;
        let bx = (pb[0] / pb[3] * 0.5 + 0.5) * w_px;
        let by = (1.0 - (pb[1] / pb[3] * 0.5 + 0.5)) * h_px;
        let az = pa[3]; // the view-distance proxy the depth test speaks
        let bz = pb[3];
        // Bresenham over the projected edge, the depth interpolated
        // linearly (the span is a few pixels — the proxy is honest)
        let (x0, y0) = (ax.round() as i64, ay.round() as i64);
        let (x1, y1) = (bx.round() as i64, by.round() as i64);
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let steps = dx.max(dy).max(1) as f32;
        let mut step: f32 = 0.0;
        let (mut cx, mut cy) = (x0, y0);
        loop {
            let t = step / steps;
            let z = az + (bz - az) * t;
            // the 2x2 plot: the vanilla's ~2.5px GL line width
            for (ox, oy) in [(0, 0i64), (1, 0), (0, 1)] {
                let px = cx + ox;
                let py = cy + oy;
                if px < 0 || py < 0 || px >= self.w as i64 || py >= self.h as i64 {
                    continue;
                }
                let idx = py as usize * self.w + px as usize;
                if always_on_top || z < self.depth[idx] {
                    let o = idx * 3;
                    for ch in 0..3 {
                        let bg = self.color[o + ch] as f32 / 255.0;
                        self.color[o + ch] =
                            (bg * (1.0 - alpha) * 255.0 + rgb[ch] * alpha * 255.0 + 0.5) as u8;
                    }
                }
            }
            step += 1.0;
            if cx == x1 && cy == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                cx += sx;
            }
            if e2 <= dx {
                err += dx;
                cy += sy;
            }
        }
    }

    /// THE DIG LANE's hit outline — the vanilla
    /// `LevelRenderer.renderHitOutline`'s black wireframe around the
    /// crosshair's block: the 12 edges of the cell's box inflated 0.002
    /// (the vanilla's own epsilon), alpha 0.4 black, depth-tested against
    /// the terrain (the inflation keeps the lines in front of the
    /// surface), ~2px wide (the vanilla GL line's own look at the
    /// reference window). Edges straddling the near plane skip — the
    /// vanilla clips them properly; the 4.5-block reach keeps the
    /// targeted box in front of the camera for every honest aim.
    pub fn draw_hit_outline(&mut self, ctx: &RenderCtx, pos: (i32, i32, i32)) {
        let vp = ctx.proj.mul(&ctx.view);
        const EPS: f32 = 0.002;
        const ALPHA: f32 = 0.4;
        let (x, y, z) = (pos.0 as f32 - EPS, pos.1 as f32 - EPS, pos.2 as f32 - EPS);
        let s = 1.0 + 2.0 * EPS;
        let corners = [
            [x, y, z],
            [x + s, y, z],
            [x + s, y, z + s],
            [x, y, z + s],
            [x, y + s, z],
            [x + s, y + s, z],
            [x + s, y + s, z + s],
            [x, y + s, z + s],
        ];
        const EDGES: [(usize, usize); 12] = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0), // the bottom ring
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4), // the top ring
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7), // the verticals
        ];
        for (a, b) in EDGES {
            self.draw_debug_line(
                &vp,
                Vec3::new(corners[a][0], corners[a][1], corners[a][2]),
                Vec3::new(corners[b][0], corners[b][1], corners[b][2]),
                [0.0, 0.0, 0.0],
                ALPHA,
                false,
            );
        }
    }

    /// THE F3+G CHUNK BORDERS' OWN DRAW (the audited 26.2
    /// ChunkBorderRenderer.emitGizmos walk — scripts/jar_work/
    /// chunkborder_262.txt — the seven layers the pure
    /// `chunk_border_lines` builder speaks, the line list the geometry
    /// law owns and this draw only projects): the debug lines ride the
    /// shared Bresenham core, the gizmos' own colors the bytecode's
    /// ARGB folds keep.
    pub fn draw_chunk_borders(&mut self, ctx: &RenderCtx, lines: &[DebugLine]) {
        self.draw_debug_lines(ctx, lines);
    }

    /// THE DEBUG GIZMO LINES' OWN PASS (the shared walk the chunk
    /// borders and the F3+B hitboxes ride — the projection, the
    /// near-plane guard, the Bresenham core, the depth law each
    /// [`DebugLine`]'s own always-on-top flag speaks).
    pub fn draw_debug_lines(&mut self, ctx: &RenderCtx, lines: &[DebugLine]) {
        let vp = ctx.proj.mul(&ctx.view);
        for l in lines {
            self.draw_debug_line(&vp, l.a, l.b, l.rgb, l.alpha, l.always_on_top);
        }
    }

    /// THE DEBUG GIZMO POINTS' OWN PASS (javap 26.2 the point
    /// primitive's GL lane the F3+B hitboxes' white dots speak): the
    /// projection the lines' own walk wears, the near-plane guard the
    /// `pa[3] < 0.1` law keeps, the 2x2 plot the GL point's ~2px look
    /// proxies, the depth test the `<` pass keeps (the stroke style's
    /// non-always-on-top flag the init banks).
    pub fn draw_debug_points(&mut self, ctx: &RenderCtx, points: &[DebugPoint]) {
        let vp = ctx.proj.mul(&ctx.view);
        let w_px = self.w as f32;
        let h_px = self.h as f32;
        for p in points {
            let pa = vp.transform_point(p.pos);
            if pa[3] < 0.1 {
                continue;
            }
            let ax = (pa[0] / pa[3] * 0.5 + 0.5) * w_px;
            let ay = (1.0 - (pa[1] / pa[3] * 0.5 + 0.5)) * h_px;
            let az = pa[3];
            let (cx, cy) = (ax.round() as i64, ay.round() as i64);
            for (ox, oy) in [(0, 0i64), (1, 0), (0, 1)] {
                let px = cx + ox;
                let py = cy + oy;
                if px < 0 || py < 0 || px >= self.w as i64 || py >= self.h as i64 {
                    continue;
                }
                let idx = py as usize * self.w + px as usize;
                if az < self.depth[idx] {
                    let o = idx * 3;
                    for ch in 0..3 {
                        let bg = self.color[o + ch] as f32 / 255.0;
                        self.color[o + ch] = (bg * (1.0 - p.alpha) * 255.0
                            + p.rgb[ch] * p.alpha * 255.0
                            + 0.5)
                            as u8;
                    }
                }
            }
        }
    }

    fn draw_pass(
        &mut self,
        ctx: &RenderCtx,
        quads: &[super::mesher::Quad],
        translucent: bool,
        overlay: bool,
    ) {
        let vp = ctx.proj.mul(&ctx.view);
        let w_px = self.w as f32;
        let h_px = self.h as f32;
        for quad in quads {
            if quad.translucent != translucent {
                continue;
            }
            // THE FIRE ANIMATION'S DRAW-TIME REMAP (the vanilla
            // TextureAtlasSprite animator's own seat — the sprite's frame
            // the atlas's tick walk swaps WITHOUT a re-mesh, the mesher's
            // baked base tile the quads keep): the fire multipart's two
            // frame families the fire_anim_tile walk answers, every other
            // tile the identity wears.
            let tile = atlas::fire_anim_tile(quad.tile, ctx.fire_anim_tick);
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
            let textured = overlay || (tile != atlas::TILE_NONE && atlas::has_tiles());
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
                            if overlay {
                                // THE BREAKING OVERLAY's own blend: the
                                // crack texel's REAL alpha composites over
                                // the block the pass covers (the vanilla
                                // crumbling shader's own translucent body —
                                // no cutout threshold, the depth test keeps
                                // the pass honest), the depth never writes
                                // (the overlay occludes nothing)
                                let u =
                                    (b0 * ua * a_ow + b1 * ub * b_ow + b2 * uc * c_ow) * dist;
                                let v =
                                    (b0 * va * a_ow + b1 * vb * b_ow + b2 * vc * c_ow) * dist;
                                let texel = textures::sample_tile(tile, u, v);
                                let a = texel[3];
                                if a <= 0.0 {
                                    continue;
                                }
                                for ch in 0..3 {
                                    let fg = c0[ch] * texel[ch];
                                    let bg = self.color[o + ch] as f32 / 255.0;
                                    c0[ch] = bg * (1.0 - a) + fg * a;
                                }
                            } else {
                                if textured {
                                    // perspective-correct texture coordinates, nearest texel
                                    let u = (b0 * ua * a_ow + b1 * ub * b_ow + b2 * uc * c_ow)
                                        * dist;
                                    let v = (b0 * va * a_ow + b1 * vb * b_ow + b2 * vc * c_ow)
                                        * dist;
                                    let texel = textures::sample_tile(tile, u, v);
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
                            }
                            if !overlay {
                                self.depth[idx] = z;
                            }
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
                            // Official CELESTIAL pipeline = BlendFunction.OVERLAY:
                            // additive color (src.rgb * src.a + dst) — the black
                            // surroundings of the sun/moon textures add nothing.
                            // src.a = texture.a x ColorModulator.a — the vertex
                            // colors' alpha the DynamicTransforms color carries
                            // (the rain fade the 26.2 renderSun/renderMoon walks
                            // keep; the dry identity the 1.0 alpha never bends).
                            let a = rgba[off + 3] as f32 / 255.0 * c0[3];
                            // src.a = texture.a x ColorModulator.a
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

    /// Rasterize the cloud layer with the official CLOUDS pipeline semantics
    /// (rendertype_clouds.vsh/.fsh + RenderPipelines.CLOUDS):
    /// - TRANSLUCENT blend: out = src·a + dst·(1-a)
    /// - depth-tested AND depth-written against terrain (DepthStencilState
    ///   DEFAULT), so terrain correctly occludes clouds
    /// - per-pixel alpha fade `a *= 1 - clamp(d / fog_alpha_end)` where d is
    ///   the SPHERICAL (Euclidean) view distance, perspective-correctly
    ///   interpolated like the shader's vertexDistance varying
    /// - no fog-color mixing (the fragment shader only fades alpha)
    /// - no backface cull (faces are built per visibility rule upstream)
    pub fn draw_clouds(
        &mut self,
        ctx: &RenderCtx,
        cam_pos: [f32; 3],
        fog_alpha_end: f32,
        quads: &[CloudQuad],
    ) {
        let vp = ctx.proj.mul(&ctx.view);
        let w_px = self.w as f32;
        let h_px = self.h as f32;
        for quad in quads {
            // Per-vertex Euclidean distance to the camera (the shader's
            // fog_spherical_distance = length(pos), pos camera-relative).
            let vd: Vec<f32> = quad
                .pts
                .iter()
                .map(|p| {
                    let dx = p[0] - cam_pos[0];
                    let dy = p[1] - cam_pos[1];
                    let dz = p[2] - cam_pos[2];
                    (dx * dx + dy * dy + dz * dz).sqrt()
                })
                .collect();
            let clip: Vec<[f32; 4]> =
                quad.pts.iter().map(|p| vp.transform_point(Vec3::new(p[0], p[1], p[2]))).collect();
            let clipped = clip_near(&clip);
            if clipped.len() < 3 {
                continue;
            }
            let mut sx = vec![0f32; clipped.len()];
            let mut sy = vec![0f32; clipped.len()];
            let mut ow = vec![0f32; clipped.len()];
            let mut ov = vec![0f32; clipped.len()];
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
                // clip_near reprojects corners on the near plane; keep the
                // attribute indices aligned with the original corners.
                let vi = i.min(3);
                ov[i] = vd[vi];
                minx = minx.min(sx[i]);
                miny = miny.min(sy[i]);
                maxx = maxx.max(sx[i]);
                maxy = maxy.max(sy[i]);
            }
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
                let (a_vd, b_vd, c_vd) = (ov[0], ov[tri.min(3)], ov[(tri + 1).min(3)]);
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
                        // perspective-correct view distance (shader varying)
                        let d = (b0 * a_vd * a_ow + b1 * b_vd * b_ow + b2 * c_vd * c_ow) * dist;
                        // perspective-correct depth, same convention as terrain
                        let z = dist;
                        let idx = (py as usize) * self.w + px as usize;
                        if z >= self.depth[idx] {
                            continue;
                        }
                        // alpha *= 1 - linear_fog_value(d, 0, FogCloudsEnd)
                        let fade =
                            (1.0 - ((d / fog_alpha_end).clamp(0.0, 1.0))).clamp(0.0, 1.0);
                        if fade <= 0.0 {
                            continue;
                        }
                        let a = quad.color[3] * fade;
                        let o = idx * 3;
                        self.depth[idx] = z;
                        for ch in 0..3 {
                            let src = quad.color[ch] * a;
                            let bg = self.color[o + ch] as f32 / 255.0;
                            self.color[o + ch] =
                                ((src + bg * (1.0 - a)).clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
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

    /// The first-person hand pass: VIEW-SPACE quads (view matrix already
    /// folded into the vertices) under the world's projection, sampled from
    /// an explicit RGBA texture (the player skin, not the block atlas).
    /// Official `entity_translucent` semantics, adapted to the reference
    /// raster: two-sided (NO backface cull — entity render types don't
    /// cull), depth test + write against a freshly cleared buffer, NO fog
    /// (the hand sits at the player's fog distance, ~0), nearest texel
    /// with the alpha cutout at 0.5, vertex color = white (the hand's
    /// packed light is the player position's full skylight in the scenes
    /// the reference renders).
    pub fn draw_hand_quads(
        &mut self,
        proj: Mat4,
        quads: &[super::hand::HandQuad],
        rgba: &[u8],
        side: usize,
    ) {
        let w_px = self.w as f32;
        let h_px = self.h as f32;
        for quad in quads {
            let clip: Vec<[f32; 4]> = quad
                .pts
                .iter()
                .map(|p| proj.transform_point(Vec3::new(p[0], p[1], p[2])))
                .collect();
            let clipped = clip_near(&clip);
            if clipped.len() < 3 {
                continue;
            }
            let mut sx = vec![0f32; clipped.len()];
            let mut sy = vec![0f32; clipped.len()];
            let mut ow = vec![0f32; clipped.len()];
            let mut vz = vec![0f32; clipped.len()];
            for i in 0..clipped.len() {
                let c = clipped[i];
                let iw = 1.0 / c[3];
                sx[i] = (c[0] * iw * 0.5 + 0.5) * w_px;
                sy[i] = (1.0 - (c[1] * iw * 0.5 + 0.5)) * h_px;
                ow[i] = iw;
                vz[i] = c[3]; // view distance proxy = clip w (draw_pass convention)
            }
            for tri in 1..clipped.len() - 1 {
                let (ax, ay, a_ow, a_vz) = (sx[0], sy[0], ow[0], vz[0]);
                let (bx, by, b_ow, b_vz) = (sx[tri], sy[tri], ow[tri], vz[tri]);
                let (cx, cy, c_ow, c_vz) = (sx[tri + 1], sy[tri + 1], ow[tri + 1], vz[tri + 1]);
                let (ua, va) = (quad.uv[0][0], quad.uv[0][1]);
                let (ub, vb) = (quad.uv[tri.min(3)][0], quad.uv[tri.min(3)][1]);
                let (uc, vc) = (
                    quad.uv[(tri + 1).min(3)][0],
                    quad.uv[(tri + 1).min(3)][1],
                );
                let area_ab = edge(ax, ay, bx, by, cx, cy);
                if area_ab.abs() < 1e-9 {
                    continue;
                }
                let inv_area = 1.0 / area_ab;
                let minx = ax.min(bx).min(cx).floor().max(0.0) as isize;
                let miny = ay.min(by).min(cy).floor().max(0.0) as isize;
                let maxx = ax.max(bx).max(cx).ceil().min(w_px - 1.0) as isize;
                let maxy = ay.max(by).max(cy).ceil().min(h_px - 1.0) as isize;
                for py in miny..=maxy {
                    for px_ in minx..=maxx {
                        let fx = px_ as f32 + 0.5;
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
                        let z = (b0 * a_vz * a_ow + b1 * b_vz * b_ow + b2 * c_vz * c_ow) * dist;
                        let u = (b0 * ua * a_ow + b1 * ub * b_ow + b2 * uc * c_ow) * dist;
                        let v = (b0 * va * a_ow + b1 * vb * b_ow + b2 * vc * c_ow) * dist;
                        let idx = (py as usize) * self.w + px_ as usize;
                        if z >= self.depth[idx] {
                            continue;
                        }
                        let tx = ((u.floor() as isize).clamp(0, side as isize - 1)) as usize;
                        let ty = ((v.floor() as isize).clamp(0, side as isize - 1)) as usize;
                        let t = ((ty * side + tx) * 4) as usize;
                        if t + 3 >= rgba.len() || rgba[t + 3] < 128 {
                            continue; // cutout hole: no color, no depth
                        }
                        self.depth[idx] = z;
                        let o = idx * 3;
                        for (ch, texel) in (0..3).zip(&rgba[t..t + 3]) {
                            self.color[o + ch] = *texel;
                        }
                    }
                }
            }
        }
    }
}

/// One view-space textured quad whose texels come from the SAMPLED
/// texture namespace (the held-item pass): every texel fetch goes through
/// the caller's sampler with the quad's texture id and the
/// perspective-correct 0..16 uv (the generated table is embedded and
/// sha-pinned, so there is no runtime file to trust). `tex` is the bare
/// texture id the sampler resolves (`sample_uv`'s namespace join).
pub struct SampledQuad<'a> {
    pub pts: [[f32; 3]; 4],
    pub uv: [[f32; 2]; 4],
    pub tex: &'a str,
    /// The per-quad color multiplier (the GUI icons' ItemColors tint x
    /// the directional shade law) — [1, 1, 1] for the untinted lanes.
    pub tint: [f32; 3],
}

impl Framebuf {
    /// The textured-quad raster over a texel SAMPLER (the held-item
    /// path). Same clip / perspective-correct interpolation / depth
    /// conventions as `draw_hand_quads`; the alpha-cutout rule is also
    /// the same (alpha < 128: no color, no depth write), applied to the
    /// sampler's RGBA. A sampler `None` (an unknown texture id) leaves
    /// the pixel untouched.
    pub fn draw_sampled_quads(
        &mut self,
        proj: Mat4,
        quads: &[SampledQuad],
        sample: &mut dyn FnMut(&str, f32, f32) -> Option<[u8; 4]>,
    ) {
        self.draw_sampled_quads_impl(proj, quads, false, sample);
    }

    /// The textured-quad raster over a texel SAMPLER with the LEQUAL
    /// depth func (the UNDERCOAT's own entry — the vanilla entity
    /// layer's depth law: the coplanar body-mesh overlay the
    /// SheepWoolUndercoatLayer speaks renders AFTER the body, the depth
    /// TIE the LEQUAL func answers — the undercoat's OPAQUE texels win
    /// the tie the body's own face set (the fringe over the brow the
    /// vanilla face keeps), the transparent ones keep it; any CLOSER
    /// geometry (the fur hood's inflated mesh the unsheared sheep
    /// wears) still wins on its own smaller z, the strict-LESS walk the
    /// other entries keep untouched).
    pub fn draw_sampled_quads_lequal(
        &mut self,
        proj: Mat4,
        quads: &[SampledQuad],
        sample: &mut dyn FnMut(&str, f32, f32) -> Option<[u8; 4]>,
    ) {
        self.draw_sampled_quads_impl(proj, quads, true, sample);
    }

    /// The shared raster core — `depth_lequal` picks the depth func:
    /// `false` = the strict-LESS convention every existing lane keeps
    /// (`z >= depth` rejects, ties never overwrite), `true` = the
    /// entity-layer LEQUAL (`z > depth` rejects, ties overwrite — the
    /// later layer's own draw order wins the coplanar tie the vanilla
    /// layer stack speaks).
    fn draw_sampled_quads_impl(
        &mut self,
        proj: Mat4,
        quads: &[SampledQuad],
        depth_lequal: bool,
        sample: &mut dyn FnMut(&str, f32, f32) -> Option<[u8; 4]>,
    ) {
        let w_px = self.w as f32;
        let h_px = self.h as f32;
        for quad in quads {
            let clip: Vec<[f32; 4]> = quad
                .pts
                .iter()
                .map(|p| proj.transform_point(Vec3::new(p[0], p[1], p[2])))
                .collect();
            let clipped = clip_near(&clip);
            if clipped.len() < 3 {
                continue;
            }
            let mut sx = vec![0f32; clipped.len()];
            let mut sy = vec![0f32; clipped.len()];
            let mut ow = vec![0f32; clipped.len()];
            let mut vz = vec![0f32; clipped.len()];
            for i in 0..clipped.len() {
                let c = clipped[i];
                let iw = 1.0 / c[3];
                sx[i] = (c[0] * iw * 0.5 + 0.5) * w_px;
                sy[i] = (1.0 - (c[1] * iw * 0.5 + 0.5)) * h_px;
                ow[i] = iw;
                vz[i] = c[3]; // view distance proxy = clip w (draw_pass convention)
            }
            for tri in 1..clipped.len() - 1 {
                let (ax, ay, a_ow, a_vz) = (sx[0], sy[0], ow[0], vz[0]);
                let (bx, by, b_ow, b_vz) = (sx[tri], sy[tri], ow[tri], vz[tri]);
                let (cx, cy, c_ow, c_vz) = (sx[tri + 1], sy[tri + 1], ow[tri + 1], vz[tri + 1]);
                let (ua, va) = (quad.uv[0][0], quad.uv[0][1]);
                let (ub, vb) = (quad.uv[tri.min(3)][0], quad.uv[tri.min(3)][1]);
                let (uc, vc) = (
                    quad.uv[(tri + 1).min(3)][0],
                    quad.uv[(tri + 1).min(3)][1],
                );
                let area_ab = edge(ax, ay, bx, by, cx, cy);
                if area_ab.abs() < 1e-9 {
                    continue;
                }
                let inv_area = 1.0 / area_ab;
                let minx = ax.min(bx).min(cx).floor().max(0.0) as isize;
                let miny = ay.min(by).min(cy).floor().max(0.0) as isize;
                let maxx = ax.max(bx).max(cx).ceil().min(w_px - 1.0) as isize;
                let maxy = ay.max(by).max(cy).ceil().min(h_px - 1.0) as isize;
                for py in miny..=maxy {
                    for px_ in minx..=maxx {
                        let fx = px_ as f32 + 0.5;
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
                        let z = (b0 * a_vz * a_ow + b1 * b_vz * b_ow + b2 * c_vz * c_ow) * dist;
                        let u = (b0 * ua * a_ow + b1 * ub * b_ow + b2 * uc * c_ow) * dist;
                        let v = (b0 * va * a_ow + b1 * vb * b_ow + b2 * vc * c_ow) * dist;
                        let idx = (py as usize) * self.w + px_ as usize;
                        if if depth_lequal {
                            z > self.depth[idx]
                        } else {
                            z >= self.depth[idx]
                        } {
                            continue;
                        }
                        let Some(texel) = sample(quad.tex, u, v) else {
                            continue;
                        };
                        if texel[3] < 128 {
                            continue; // cutout hole: no color, no depth
                        }
                        let texel = [
                            (texel[0] as f32 / 255.0 * quad.tint[0] * 255.0).round() as u8,
                            (texel[1] as f32 / 255.0 * quad.tint[1] * 255.0).round() as u8,
                            (texel[2] as f32 / 255.0 * quad.tint[2] * 255.0).round() as u8,
                            texel[3],
                        ];
                        self.depth[idx] = z;
                        let o = idx * 3;
                        for (ch, texel) in (0..3).zip(&texel[..3]) {
                            self.color[o + ch] = *texel;
                        }
                    }
                }
            }
        }
    }

    /// THE SCREEN-EFFECT BLEND (the sampled walk's translucent sibling
    /// — the fire screen overlay's own draw): the same clip /
    /// perspective-correct interpolation / depth TEST as
    /// `draw_sampled_quads`, but NO depth write (the translucent pass's
    /// own convention — the two overlapping camera quads both blend)
    /// and the texel's own alpha scales the pass's vertex alpha
    /// (`a = texel_a * alpha_scale`), the result blended over the
    /// framebuffer: `out = src * a + dst * (1 - a)` — the vanilla
    /// translucent blend the fire's RenderType speaks. The vertex
    /// alpha is the 0xE5FFFFFF vertex color the buildQuad bytecode
    /// speaks (229/255), the per-quad tint rides before the blend.
    pub fn draw_translucent_sampled_quads(
        &mut self,
        proj: Mat4,
        quads: &[SampledQuad],
        alpha_scale: f32,
        sample: &mut dyn FnMut(&str, f32, f32) -> Option<[u8; 4]>,
    ) {
        self.draw_translucent_sampled_quads_impl(proj, quads, &|_| alpha_scale, sample);
    }

    /// THE PER-QUAD ALPHA WALK (the weather renderer's own entry — the
    /// jar's `renderInstances` folds ONE vertex color per column: the
    /// `ARGB.white(alphaF)` the distance-fade lerp speaks, so every
    /// quad carries its own alpha and the translucent blend scales the
    /// texel's alpha by THAT quad's word — the per-call scalar's
    /// per-quad twin, everything else identical). The alphas slice
    /// indexes with the quad; a missing entry keeps the 1.0 identity.
    pub fn draw_translucent_sampled_quads_alphas(
        &mut self,
        proj: Mat4,
        quads: &[SampledQuad],
        alphas: &[f32],
        sample: &mut dyn FnMut(&str, f32, f32) -> Option<[u8; 4]>,
    ) {
        self.draw_translucent_sampled_quads_impl(
            proj,
            quads,
            &|i| alphas.get(i).copied().unwrap_or(1.0),
            sample,
        );
    }

    /// The shared translucent core — `alpha_of` picks the quad's vertex
    /// alpha (the constant scalar the screen overlays speak, the
    /// per-column word the weather lane folds).
    fn draw_translucent_sampled_quads_impl(
        &mut self,
        proj: Mat4,
        quads: &[SampledQuad],
        alpha_of: &dyn Fn(usize) -> f32,
        sample: &mut dyn FnMut(&str, f32, f32) -> Option<[u8; 4]>,
    ) {
        let w_px = self.w as f32;
        let h_px = self.h as f32;
        for (quad_index, quad) in quads.iter().enumerate() {
            let alpha_scale = alpha_of(quad_index);
            let clip: Vec<[f32; 4]> = quad
                .pts
                .iter()
                .map(|p| proj.transform_point(Vec3::new(p[0], p[1], p[2])))
                .collect();
            let clipped = clip_near(&clip);
            if clipped.len() < 3 {
                continue;
            }
            let mut sx = vec![0f32; clipped.len()];
            let mut sy = vec![0f32; clipped.len()];
            let mut ow = vec![0f32; clipped.len()];
            let mut vz = vec![0f32; clipped.len()];
            for i in 0..clipped.len() {
                let c = clipped[i];
                let iw = 1.0 / c[3];
                sx[i] = (c[0] * iw * 0.5 + 0.5) * w_px;
                sy[i] = (1.0 - (c[1] * iw * 0.5 + 0.5)) * h_px;
                ow[i] = iw;
                vz[i] = c[3];
            }
            for tri in 1..clipped.len() - 1 {
                let (ax, ay, a_ow, a_vz) = (sx[0], sy[0], ow[0], vz[0]);
                let (bx, by, b_ow, b_vz) = (sx[tri], sy[tri], ow[tri], vz[tri]);
                let (cx, cy, c_ow, c_vz) = (sx[tri + 1], sy[tri + 1], ow[tri + 1], vz[tri + 1]);
                let (ua, va) = (quad.uv[0][0], quad.uv[0][1]);
                let (ub, vb) = (quad.uv[tri.min(3)][0], quad.uv[tri.min(3)][1]);
                let (uc, vc) = (
                    quad.uv[(tri + 1).min(3)][0],
                    quad.uv[(tri + 1).min(3)][1],
                );
                let area_ab = edge(ax, ay, bx, by, cx, cy);
                if area_ab.abs() < 1e-9 {
                    continue;
                }
                let inv_area = 1.0 / area_ab;
                let minx = ax.min(bx).min(cx).floor().max(0.0) as isize;
                let miny = ay.min(by).min(cy).floor().max(0.0) as isize;
                let maxx = ax.max(bx).max(cx).ceil().min(w_px - 1.0) as isize;
                let maxy = ay.max(by).max(cy).ceil().min(h_px - 1.0) as isize;
                for py in miny..=maxy {
                    for px_ in minx..=maxx {
                        let fx = px_ as f32 + 0.5;
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
                        let z = (b0 * a_vz * a_ow + b1 * b_vz * b_ow + b2 * c_vz * c_ow) * dist;
                        let u = (b0 * ua * a_ow + b1 * ub * b_ow + b2 * uc * c_ow) * dist;
                        let v = (b0 * va * a_ow + b1 * vb * b_ow + b2 * vc * c_ow) * dist;
                        let idx = (py as usize) * self.w + px_ as usize;
                        if z >= self.depth[idx] {
                            continue; // the depth TEST survives; nothing writes
                        }
                        let Some(texel) = sample(quad.tex, u, v) else {
                            continue;
                        };
                        let a = (texel[3] as f32 / 255.0) * alpha_scale;
                        if a <= 0.0 {
                            continue; // the fully transparent texel blends nothing
                        }
                        let o = idx * 3;
                        for ch in 0..3 {
                            let src = texel[ch] as f32 / 255.0 * quad.tint[ch];
                            let dst = self.color[o + ch] as f32 / 255.0;
                            let out = src * a + dst * (1.0 - a);
                            self.color[o + ch] = (out.clamp(0.0, 1.0) * 255.0).round() as u8;
                        }
                    }
                }
            }
        }
    }

    /// THE PIP'S OWN DEPTH WALK (the inventory mannequin's draw — the
    /// affine ortho chain the PiP speaks): identical to
    /// `draw_sampled_quads` in clip / interpolation / cutout, but the
    /// depth proxy rides NEGATED clip z instead of clip w. The affine
    /// chain leaves w = 1 for every vertex, so the w proxy ties every
    /// quad and the first-drawn layer (the skin) wins every pixel the
    /// armor's inflated shell actually covers — the sparse margin-only
    /// paint the v0.3.244 dig named. The ortho's z row already encodes
    /// the view depth (z_clip = 2/2000 * z_view), and the chain's -z
    /// mirror makes the camera-facing side land LARGER z_clip, so the
    /// negation restores the raster's smaller-is-nearer law and each
    /// inflate step wins the pixels its geometry covers — the vanilla
    /// depth-tested layer stack.
    pub fn draw_pip_sampled_quads(
        &mut self,
        proj: Mat4,
        quads: &[SampledQuad],
        sample: &mut dyn FnMut(&str, f32, f32) -> Option<[u8; 4]>,
    ) {
        let w_px = self.w as f32;
        let h_px = self.h as f32;
        for quad in quads {
            let clip: Vec<[f32; 4]> = quad
                .pts
                .iter()
                .map(|p| proj.transform_point(Vec3::new(p[0], p[1], p[2])))
                .collect();
            let clipped = clip_near(&clip);
            if clipped.len() < 3 {
                continue;
            }
            let mut sx = vec![0f32; clipped.len()];
            let mut sy = vec![0f32; clipped.len()];
            let mut ow = vec![0f32; clipped.len()];
            let mut vz = vec![0f32; clipped.len()];
            for i in 0..clipped.len() {
                let c = clipped[i];
                let iw = 1.0 / c[3];
                sx[i] = (c[0] * iw * 0.5 + 0.5) * w_px;
                sy[i] = (1.0 - (c[1] * iw * 0.5 + 0.5)) * h_px;
                ow[i] = iw;
                vz[i] = -c[2]; // the pip's depth: nearer (larger z_clip) rides smaller
            }
            for tri in 1..clipped.len() - 1 {
                let (ax, ay, a_ow, a_vz) = (sx[0], sy[0], ow[0], vz[0]);
                let (bx, by, b_ow, b_vz) = (sx[tri], sy[tri], ow[tri], vz[tri]);
                let (cx, cy, c_ow, c_vz) = (sx[tri + 1], sy[tri + 1], ow[tri + 1], vz[tri + 1]);
                let (ua, va) = (quad.uv[0][0], quad.uv[0][1]);
                let (ub, vb) = (quad.uv[tri.min(3)][0], quad.uv[tri.min(3)][1]);
                let (uc, vc) = (
                    quad.uv[(tri + 1).min(3)][0],
                    quad.uv[(tri + 1).min(3)][1],
                );
                let area_ab = edge(ax, ay, bx, by, cx, cy);
                if area_ab.abs() < 1e-9 {
                    continue;
                }
                let inv_area = 1.0 / area_ab;
                let minx = ax.min(bx).min(cx).floor().max(0.0) as isize;
                let miny = ay.min(by).min(cy).floor().max(0.0) as isize;
                let maxx = ax.max(bx).max(cx).ceil().min(w_px - 1.0) as isize;
                let maxy = ay.max(by).max(cy).ceil().min(h_px - 1.0) as isize;
                for py in miny..=maxy {
                    for px_ in minx..=maxx {
                        let fx = px_ as f32 + 0.5;
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
                        let z = (b0 * a_vz * a_ow + b1 * b_vz * b_ow + b2 * c_vz * c_ow) * dist;
                        let u = (b0 * ua * a_ow + b1 * ub * b_ow + b2 * uc * c_ow) * dist;
                        let v = (b0 * va * a_ow + b1 * vb * b_ow + b2 * vc * c_ow) * dist;
                        let idx = (py as usize) * self.w + px_ as usize;
                        if z >= self.depth[idx] {
                            continue;
                        }
                        let Some(texel) = sample(quad.tex, u, v) else {
                            continue;
                        };
                        if texel[3] < 128 {
                            continue; // cutout hole: no color, no depth
                        }
                        let texel = [
                            (texel[0] as f32 / 255.0 * quad.tint[0] * 255.0).round() as u8,
                            (texel[1] as f32 / 255.0 * quad.tint[1] * 255.0).round() as u8,
                            (texel[2] as f32 / 255.0 * quad.tint[2] * 255.0).round() as u8,
                            texel[3],
                        ];
                        self.depth[idx] = z;
                        let o = idx * 3;
                        for (ch, texel) in (0..3).zip(&texel[..3]) {
                            self.color[o + ch] = *texel;
                        }
                    }
                }
            }
        }
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

#[test]
fn sampled_quads_raster_with_the_cutout_rule() {
    use crate::math::Mat4;
    // one quad covering the viewport at z=-1; the sampler paints RED on
    // the left half of the 0..16 uv space and a fully transparent texel
    // on the right half — the cutout rule must write color+depth on the
    // left only
    let quads = [SampledQuad {
        pts: [
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
        ],
        uv: [[0.0, 0.0], [16.0, 0.0], [16.0, 16.0], [0.0, 16.0]],
        tex: "test",
        tint: [1.0, 1.0, 1.0],
    }];
    let mut fb = Framebuf::new(32, 32);
    fb.clear([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    let proj = Mat4::perspective(70.0f32.to_radians(), 1.0, 0.1, 512.0);
    fb.draw_sampled_quads(proj, &quads, &mut |_tex, u, _v| {
        if u < 8.0 {
            Some([255, 0, 0, 255])
        } else {
            Some([0, 255, 0, 0]) // alpha 0 -> cutout hole
        }
    });
    let mut red = 0;
    let mut green = 0;
    for px in fb.color.chunks_exact(3) {
        if px == &[255, 0, 0] {
            red += 1;
        }
        if px == &[0, 255, 0] {
            green += 1;
        }
    }
    assert!(red > 0, "the opaque half rasterized");
    assert_eq!(green, 0, "the transparent half must not write color");
    // depth: written on the red half only
    assert!(fb.depth.iter().any(|&d| d < 1e30));
}

#[test]
fn translucent_sampled_quads_blend_and_skip_depth() {
    use crate::math::Mat4;
    // THE FIRE OVERLAY'S OWN DRAW LAW: one full-viewport quad at z=-1,
    // the sampler paints half-opaque RED everywhere (alpha 128) — at
    // the alpha scale 0.5 the blend folds src*0.5*a + dst*(1 - src_a):
    // the framebuffer's black dst rides the red at half strength, the
    // DEPTH buffer stays untouched (the translucent pass writes
    // nothing), and a fully transparent texel blends nothing.
    let quads = [SampledQuad {
        pts: [
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
        ],
        uv: [[0.0, 0.0], [16.0, 0.0], [16.0, 16.0], [0.0, 16.0]],
        tex: "test",
        tint: [1.0, 1.0, 1.0],
    }];
    let mut fb = Framebuf::new(32, 32);
    fb.clear([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    let depth_before = fb.depth.clone();
    let proj = Mat4::perspective(70.0f32.to_radians(), 1.0, 0.1, 512.0);
    fb.draw_translucent_sampled_quads(proj, &quads, 0.5, &mut |_tex, _u, _v| {
        Some([255, 0, 0, 128]) // half-covered texel, everywhere
    });
    // THE BLEND: 255 * (128/255 * 0.5) = ~63.5 -> the red rides at
    // ~63, the green/blue stay zero (the black dst's own share)
    let mut red_mid = 0;
    let mut untouched = 0;
    for px in fb.color.chunks_exact(3) {
        if px[0] > 40 && px[0] < 90 && px[1] == 0 && px[2] == 0 {
            red_mid += 1;
        }
        if px[0] == 0 && px[1] == 0 && px[2] == 0 {
            untouched += 1;
        }
    }
    assert!(red_mid > 0, "the half-alpha red blended over the black");
    assert_eq!(
        untouched, 0,
        "the full-quad sampler must leave no pixel unblended"
    );
    // THE NO-DEPTH-WRITE LAW (the translucent pass's own convention):
    // the depth buffer never moved
    assert_eq!(fb.depth, depth_before, "the blend must not write depth");
    // THE ZERO-ALPHA TEXEL: a second draw with alpha 0 changes nothing
    let fb2_color = fb.color.clone();
    fb.draw_translucent_sampled_quads(proj, &quads, 0.5, &mut |_tex, _u, _v| {
        Some([0, 255, 0, 0])
    });
    assert_eq!(fb.color, fb2_color, "the transparent texel blends nothing");
}

#[test]
fn lequal_entry_wins_the_coplanar_tie_the_strict_entry_keeps() {
    use crate::math::Mat4;
    // THE UNDERCOAT'S OWN DEPTH LAW: two COPLANAR full-viewport quads
    // at z=-1 — the body's face the first draw fills RED, the
    // undercoat's overlay the second draws BLUE. The strict-LESS entry
    // (`z >= depth` rejects) keeps the tie the body set — the blue
    // never lands. The LEQUAL entry (`z > depth` rejects) answers the
    // tie the vanilla entity layer speaks — the later layer's opaque
    // texels overwrite, and any CLOSER geometry (z=-0.5 white) still
    // wins on its own smaller z, the FARTHER one (z=-2 green) never
    // comes back.
    let quad_at = |z: f32| SampledQuad {
        pts: [
            [-1.0, -1.0, z],
            [1.0, -1.0, z],
            [1.0, 1.0, z],
            [-1.0, 1.0, z],
        ],
        uv: [[0.0, 0.0], [16.0, 0.0], [16.0, 16.0], [0.0, 16.0]],
        tex: "test",
        tint: [1.0, 1.0, 1.0],
    };
    let proj = Mat4::perspective(70.0f32.to_radians(), 1.0, 0.1, 512.0);
    let solid = |r: u8, g: u8, b: u8| {
        move |_tex: &str, _u: f32, _v: f32| Some([r, g, b, 255])
    };
    let count = |fb: &Framebuf, px: [u8; 3]| -> usize {
        fb.color.chunks_exact(3).filter(|c| c[..3] == px).count()
    };
    // THE STRICT TIE: the coplanar second draw the strict entry rejects
    let mut fb = Framebuf::new(32, 32);
    fb.clear([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    fb.draw_sampled_quads(proj, &[quad_at(-1.0)], &mut solid(255, 0, 0));
    fb.draw_sampled_quads(proj, &[quad_at(-1.0)], &mut solid(0, 0, 255));
    assert_eq!(count(&fb, [0, 0, 255]), 0, "the strict tie never overwrites");
    assert!(count(&fb, [255, 0, 0]) > 0, "the body's own red stays");
    // THE LEQUAL TIE: the same coplanar draw the LEQUAL entry WINS
    let mut fb = Framebuf::new(32, 32);
    fb.clear([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    fb.draw_sampled_quads(proj, &[quad_at(-1.0)], &mut solid(255, 0, 0));
    fb.draw_sampled_quads_lequal(proj, &[quad_at(-1.0)], &mut solid(0, 0, 255));
    assert!(count(&fb, [0, 0, 255]) > 0, "the layer's own tie wins");
    assert_eq!(count(&fb, [255, 0, 0]), 0, "the body's own red replaced");
    // THE CLOSER GEOMETRY STILL WINS + THE CUTOUT HOLE KEEPS: a nearer
    // white quad through the strict entry covers the lequal blue, the
    // farther green never comes back
    fb.draw_sampled_quads(proj, &[quad_at(-0.5)], &mut solid(255, 255, 255));
    assert!(count(&fb, [255, 255, 255]) > 0, "the nearer quad covers");
    assert_eq!(count(&fb, [0, 0, 255]), 0);
    fb.draw_sampled_quads(proj, &[quad_at(-2.0)], &mut solid(0, 255, 0));
    assert_eq!(count(&fb, [0, 255, 0]), 0, "the farther quad never returns");
    // THE TRANSPARENT TEXEL: the lequal entry's cutout hole keeps the
    // body's color AND depth (no write, no overwrite)
    let mut fb = Framebuf::new(32, 32);
    fb.clear([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    fb.draw_sampled_quads(proj, &[quad_at(-1.0)], &mut solid(255, 0, 0));
    let depth_after_body = fb.depth.clone();
    fb.draw_sampled_quads_lequal(proj, &[quad_at(-1.0)], &mut |_tex, _u, _v| {
        Some([0, 0, 255, 0]) // alpha 0 -> the cutout hole
    });
    assert!(count(&fb, [255, 0, 0]) > 0, "the hole keeps the body's red");
    assert_eq!(fb.depth, depth_after_body, "the hole writes no depth");
}

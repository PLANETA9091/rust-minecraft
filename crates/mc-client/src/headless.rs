//! Headless deterministic rendering (CI / pixel tests / no-display hosts).

use std::path::Path;
use std::time::Instant;

use anyhow::Result;

use crate::camera::Camera;
use crate::math::{Mat4, Vec3};
use crate::mesher::mesh_world;
use crate::png_io::write_rgb_png;
use crate::raster::{Framebuf, RenderCtx};
use crate::textures::TextureLut;
use crate::world::World;

pub const SKY_TOP: [f32; 3] = [0.45, 0.66, 0.94];
pub const SKY_BOTTOM: [f32; 3] = [0.74, 0.85, 0.95];
pub const FOG_COLOR: [f32; 3] = [0.74, 0.85, 0.95];

/// Deterministic orbit camera for frame `frame_index` of `frames`.
pub fn orbit_camera(world: &World, frame_index: u32, frames: u32) -> Camera {
    let n = world.size_blocks() as f32;
    let center = Vec3::new(n / 2.0, 12.0, n / 2.0);
    let radius = n * 0.62;
    let t = if frames <= 1 { 0.62 } else { frame_index as f32 / frames as f32 };
    let angle = 0.62 + t * 0.8;
    let eye = Vec3::new(
        center.x + angle.sin() * radius,
        center.y + 6.5,
        center.z + angle.cos() * radius,
    );
    Camera { pos: eye, yaw: 0.0, pitch: -0.33 }.with_look_at(center.add(Vec3::new(0.0, 2.0, 0.0)))
}

impl Camera {
    fn with_look_at(mut self, target: Vec3) -> Camera {
        let dir = target.sub(self.pos).normalized();
        self.pitch = dir.y.asin();
        self.yaw = dir.z.atan2(dir.x) + std::f32::consts::FRAC_PI_2;
        self
    }
}

/// Render `frames` deterministic frames; save the last one as PNG.
/// Returns per-frame render milliseconds.
pub fn render_frames(
    world: &World,
    lut: Option<&TextureLut>,
    width: usize,
    height: usize,
    frames: u32,
    out_path: &Path,
) -> Result<Vec<f64>> {
    let quads = mesh_world(world, lut);
    let mut fb = Framebuf::new(width, height);
    let n = world.size_blocks() as f32;
    let mut timings = Vec::with_capacity(frames as usize);

    for f in 0..frames {
        let cam = orbit_camera(world, f, frames);
        let t0 = Instant::now();
        fb.clear(SKY_TOP, SKY_BOTTOM);
        let ctx = RenderCtx {
            view: cam.view(),
            proj: Mat4::perspective(70.0f32.to_radians(), width as f32 / height as f32, 0.1, 512.0),
            fog_color: FOG_COLOR,
            fog_near: n * 0.55,
            fog_far: n * 1.7,
        };
        fb.draw_quads(&ctx, &quads);
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        timings.push(ms);
        println!(
            "frame {}/{}: {:6.1} ms ({:6.1} fps) — {}x{}",
            f + 1,
            frames,
            ms,
            if ms > 0.0 { 1000.0 / ms } else { 0.0 },
            width,
            height
        );
    }

    write_rgb_png(out_path, width, height, &fb.color)?;
    println!("saved {}", out_path.display());
    Ok(timings)
}
